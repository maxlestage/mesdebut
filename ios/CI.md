# CI iOS — build & TestFlight

Deux workflows GitHub Actions (dans `.github/workflows/`) :

| Workflow | Rôle | Déclenchement | Secrets |
|---|---|---|---|
| `ios-build.yml` | **Compile** l'app sur simulateur (sans signature) | push/PR touchant `ios/**` | aucun |
| `ios-testflight.yml` | **Build signé + envoi sur TestFlight** (xcodebuild + altool) | manuel, ou tag `ios-v*` | oui (voir plus bas) |

Le premier ne demande aucune configuration : il sert à détecter tôt les erreurs de
compilation. Le second nécessite les réglages ci-dessous.

## Préalables côté Apple (une seule fois)

1. **Compte Apple Developer** actif (99 €/an).
2. Dans **App Store Connect**, créer l'app avec le bundle id `com.reneuro.ReNeuro`.
   L'app embarque deux autres cibles, dont les identifiants doivent exister eux
   aussi (la signature automatique les crée toute seule à la première archive) :
   - `com.reneuro.ReNeuro.Widget` — extension widget (activité en direct)
   - `com.reneuro.ReNeuro.watchkitapp` — application Apple Watch
   (ou change-le dans le projet Xcode : réglage `PRODUCT_BUNDLE_IDENTIFIER`).
3. Créer une **clé API App Store Connect** (Utilisateurs et accès → Intégrations →
   Clés App Store Connect), rôle **App Manager**. Télécharger le fichier `.p8`
   (téléchargeable une seule fois) et noter le **Key ID** et l'**Issuer ID**.
4. Récupérer le **Team ID** (Apple Developer → Membership).

## Secrets GitHub à ajouter

Dépôt → *Settings* → *Secrets and variables* → *Actions* → *New repository secret* :

| Secret | Obligatoire | Contenu |
|---|---|---|
| `ASC_KEY_ID` | oui | le Key ID de la clé API (ex. `A1B2C3D4E5`) |
| `ASC_ISSUER_ID` | oui | l'Issuer ID (un UUID) |
| `ASC_KEY_P8` | oui | le contenu du fichier `.p8` |
| `APPLE_TEAM_ID` | oui | le Team ID (ex. `1A2B3C4D5E`) |

`ASC_KEY_P8` s'accepte **sous les deux formes** : le contenu du `.p8` collé
tel quel, en-têtes `BEGIN`/`END PRIVATE KEY` compris, ou le même contenu
encodé en base64. Le workflow reconnaît laquelle des deux il a reçue, et
refuse tout de suite un secret qui n'est pas une clé privée exploitable —
plutôt que d'échouer trois étapes plus loin sur une erreur de signature.

`APPLE_TEAM_ID` est **obligatoire**, y compris avec une clé API : celle-ci
authentifie, mais elle ne désigne pas l'équipe. Sans lui, l'archive échoue sur
« Signing for "ReNeuro" requires a development team », sur les trois cibles.

Il se lit sur developer.apple.com, rubrique *Membership* — ou en haut à droite
d'App Store Connect, sous le nom du compte.

> Les anciens noms — `APP_STORE_CONNECT_API_KEY_ID`,
> `APP_STORE_CONNECT_API_ISSUER_ID`, `APP_STORE_CONNECT_API_KEY_BASE64` —
> restent acceptés. Le nom court l'emporte quand les deux sont définis.

## Lancer un envoi TestFlight

- **Manuellement** : onglet *Actions* → *iOS TestFlight* → *Run workflow*.
- **Par tag** :

  ```bash
  git tag ios-v1.0.0 && git push origin ios-v1.0.0
  ```

Le numéro de build (`CURRENT_PROJECT_VERSION`) est réglé automatiquement sur le
numéro d'exécution du workflow, pour que chaque envoi soit unique. La version
marketing (`MARKETING_VERSION`, ex. 1.0) se change dans le projet Xcode.

## Notes

- Aucune dépendance externe : le workflow utilise seulement les outils Xcode
  (`xcodebuild archive`, `xcodebuild -exportArchive`, puis `xcrun altool --upload-app`).
- La signature utilise la **signature automatique « cloud »** (`-allowProvisioningUpdates`
  + clé API) : pas besoin de gérer manuellement certificats et profils.
- Le workflow n'attend pas la fin du traitement Apple. Le build apparaît dans
  TestFlight quelques minutes plus tard ; pense à remplir les informations de
  conformité export si demandé.
