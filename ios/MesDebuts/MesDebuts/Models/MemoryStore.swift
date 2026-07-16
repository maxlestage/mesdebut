import Foundation
import SQLite3

// Mémoire de l'apprenant persistée en SQLite.
// Chaque notion (identifiée par sa clé) suit un système de boîtes de Leitner :
// une bonne réponse la fait monter d'une boîte (révision plus espacée),
// une erreur la fait redescendre. C'est de la répétition espacée qui survit
// d'une session à l'autre — et même d'un jour à l'autre.
final class MemoryStore {
    static let shared = MemoryStore()

    private var db: OpaquePointer?
    // sqlite demande à recopier les chaînes liées (elles ne vivent que le temps de l'appel).
    private let transient = unsafeBitCast(-1, to: sqlite3_destructor_type.self)
    // intervalles de révision en jours, par boîte de Leitner (1 à 5)
    private let intervals: [Double] = [1, 2, 4, 7, 15]

    private init() {
        open()
        migrate()
    }

    // MARK: - Ouverture / schéma

    private func dbURL() -> URL {
        let fm = FileManager.default
        let base = (try? fm.url(for: .applicationSupportDirectory, in: .userDomainMask,
                                appropriateFor: nil, create: true))
            ?? fm.urls(for: .documentDirectory, in: .userDomainMask)[0]
        return base.appendingPathComponent("mesdebuts.sqlite")
    }

    private func open() {
        if sqlite3_open(dbURL().path, &db) != SQLITE_OK {
            print("SQLite: échec d'ouverture de la base")
        }
    }

    private func exec(_ sql: String) {
        var err: UnsafeMutablePointer<CChar>?
        if sqlite3_exec(db, sql, nil, nil, &err) != SQLITE_OK, let err {
            print("SQLite: \(String(cString: err))")
            sqlite3_free(err)
        }
    }

    private func migrate() {
        exec("""
        CREATE TABLE IF NOT EXISTS item_memory (
            key TEXT PRIMARY KEY,
            category TEXT,
            seen INTEGER NOT NULL DEFAULT 0,
            correct INTEGER NOT NULL DEFAULT 0,
            wrong INTEGER NOT NULL DEFAULT 0,
            streak INTEGER NOT NULL DEFAULT 0,
            box INTEGER NOT NULL DEFAULT 1,
            due REAL NOT NULL DEFAULT 0,
            last REAL NOT NULL DEFAULT 0
        );
        """)
        exec("""
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT,
            score INTEGER,
            total INTEGER,
            date REAL
        );
        """)
    }

    // MARK: - Enregistrement

    /// Enregistre une réponse et met à jour la boîte de Leitner de la notion.
    func recordAnswer(key: String, category: String, correct: Bool) {
        let now = Date().timeIntervalSince1970

        // état actuel (boîte, série)
        var box = 1
        var streak = 0
        var stmt: OpaquePointer?
        if sqlite3_prepare_v2(db, "SELECT box, streak FROM item_memory WHERE key = ?;", -1, &stmt, nil) == SQLITE_OK {
            sqlite3_bind_text(stmt, 1, key, -1, transient)
            if sqlite3_step(stmt) == SQLITE_ROW {
                box = Int(sqlite3_column_int(stmt, 0))
                streak = Int(sqlite3_column_int(stmt, 1))
            }
        }
        sqlite3_finalize(stmt)

        if correct { box = min(box + 1, 5); streak += 1 }
        else { box = max(box - 1, 1); streak = 0 }
        let due = now + intervals[box - 1] * 86_400
        let c = Int32(correct ? 1 : 0)
        let w = Int32(correct ? 0 : 1)

        let sql = """
        INSERT INTO item_memory (key, category, seen, correct, wrong, streak, box, due, last)
        VALUES (?, ?, 1, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(key) DO UPDATE SET
            seen = seen + 1,
            correct = correct + ?,
            wrong = wrong + ?,
            streak = ?,
            box = ?,
            due = ?,
            last = ?;
        """
        var s: OpaquePointer?
        if sqlite3_prepare_v2(db, sql, -1, &s, nil) == SQLITE_OK {
            sqlite3_bind_text(s, 1, key, -1, transient)
            sqlite3_bind_text(s, 2, category, -1, transient)
            sqlite3_bind_int(s, 3, c)
            sqlite3_bind_int(s, 4, w)
            sqlite3_bind_int(s, 5, Int32(streak))
            sqlite3_bind_int(s, 6, Int32(box))
            sqlite3_bind_double(s, 7, due)
            sqlite3_bind_double(s, 8, now)
            sqlite3_bind_int(s, 9, c)
            sqlite3_bind_int(s, 10, w)
            sqlite3_bind_int(s, 11, Int32(streak))
            sqlite3_bind_int(s, 12, Int32(box))
            sqlite3_bind_double(s, 13, due)
            sqlite3_bind_double(s, 14, now)
            sqlite3_step(s)
        }
        sqlite3_finalize(s)
    }

    /// Enregistre le résultat d'un quiz terminé.
    func recordSession(category: String, score: Int, total: Int) {
        var s: OpaquePointer?
        if sqlite3_prepare_v2(db, "INSERT INTO sessions (category, score, total, date) VALUES (?, ?, ?, ?);", -1, &s, nil) == SQLITE_OK {
            sqlite3_bind_text(s, 1, category, -1, transient)
            sqlite3_bind_int(s, 2, Int32(score))
            sqlite3_bind_int(s, 3, Int32(total))
            sqlite3_bind_double(s, 4, Date().timeIntervalSince1970)
            sqlite3_step(s)
        }
        sqlite3_finalize(s)
    }

    // MARK: - Lecture

    struct Summary {
        var answered: Int = 0   // nombre total de réponses données
        var correct: Int = 0    // nombre de bonnes réponses
        var mastered: Int = 0   // notions bien ancrées (boîte ≥ 4)
        var due: Int = 0        // notions à réviser maintenant
    }

    func summary() -> Summary {
        var out = Summary()
        let now = Date().timeIntervalSince1970
        let sql = """
        SELECT COALESCE(SUM(seen), 0),
               COALESCE(SUM(correct), 0),
               COALESCE(SUM(CASE WHEN box >= 4 THEN 1 ELSE 0 END), 0),
               COALESCE(SUM(CASE WHEN due > 0 AND due <= ? THEN 1 ELSE 0 END), 0)
        FROM item_memory;
        """
        var s: OpaquePointer?
        if sqlite3_prepare_v2(db, sql, -1, &s, nil) == SQLITE_OK {
            sqlite3_bind_double(s, 1, now)
            if sqlite3_step(s) == SQLITE_ROW {
                out.answered = Int(sqlite3_column_int(s, 0))
                out.correct = Int(sqlite3_column_int(s, 1))
                out.mastered = Int(sqlite3_column_int(s, 2))
                out.due = Int(sqlite3_column_int(s, 3))
            }
        }
        sqlite3_finalize(s)
        return out
    }

    /// Réinitialise toute la mémoire (utile pour repartir de zéro).
    func reset() {
        exec("DELETE FROM item_memory;")
        exec("DELETE FROM sessions;")
    }
}
