/*
 * Le noyau de ReNeuro, vu de Swift : deux fonctions.
 *
 * Écrit en Rust (noyau/src/ffi.rs), compilé en bibliothèque statique
 * libreneuro_noyau.a par noyau/xcode.sh pendant la construction Xcode.
 */
#ifndef RENEURO_H
#define RENEURO_H

/*
 * Appelle la fonction `fonction` du noyau ("quiz", "categories"…) avec ses
 * `arguments` : du texte, champs séparés par des tabulations, enregistrements
 * par des sauts de ligne. Renvoie du JSON en UTF-8, jamais NULL — en cas
 * d'erreur, {"erreur":"…"}. La réponse se rend avec reneuro_liberer.
 *
 * La liste des fonctions est dans la documentation de noyau/src/ffi.rs.
 */
char *reneuro_appel(const char *fonction, const char *arguments);

/* Rend la mémoire d'une réponse de reneuro_appel. */
void reneuro_liberer(char *reponse);

#endif
