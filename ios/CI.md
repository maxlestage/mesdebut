# CI iOS — build & TestFlight

Deux workflows GitHub Actions (dans `.github/workflows/`) :

| Workflow | Rôle | Déclenchement | Secrets |
|---|---|---|---|
| `ios-build.yml` | **Compile** l'app sur simulateur (sans signature) | push/PR touchant `ios/**` | aucun |
| `ios-testflight.yml` | **Build signé + envoi sur TestFlight** (fastlane) | manuel, ou tag `ios-v*` | oui (voir plus bas) |

Le premier ne demande aucune configuration : il sert à détecter tôt les erreurs de
compilation. Le second nécessite les réglages ci-dessous.

## Préalables côté Apple (une seule fois)

1. **Compte Apple Developer** actif (99 €/an).
2. Dans **App Store Connect**, créer l'app avec le bundle id `com.mesdebuts.MesDebuts`
   (ou change-le partout : `project.pbxproj`, `fastlane/Appfile`).
3. Créer une **clé API App Store Connect** (Utilisateurs et accès → Intégrations →
   Clés App Store Connect), rôle **App Manager**. Télécharger le fichier `.p8`
   (téléchargeable une seule fois) et noter le **Key ID** et l'**Issuer ID**.
4. Récupérer le **Team ID** (Apple Developer → Membership).

## Secrets GitHub à ajouter

Dépôt → *Settings* → *Secrets and variables* → *Actions* → *New repository secret* :

| Secret | Contenu |
|---|---|
| `APP_STORE_CONNECT_API_KEY_ID` | le Key ID de la clé API (ex. `A1B2C3D4E5`) |
| `APP_STORE_CONNECT_API_ISSUER_ID` | l'Issuer ID (un UUID) |
| `APP_STORE_CONNECT_API_KEY_BASE64` | le contenu du `.p8`, encodé en base64 |
| `APPLE_TEAM_ID` | le Team ID (ex. `1A2B3C4D5E`) |

Pour encoder la clé `.p8` en base64 :

```bash
base64 -i AuthKey_XXXXXXXXXX.p8 | pbcopy   # macOS : le résultat est dans le presse-papier
```

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

- La signature utilise la **signature automatique « cloud »** (`-allowProvisioningUpdates`
  + clé API) : pas besoin de gérer manuellement certificats et profils.
- `skip_waiting_for_build_processing: true` : le workflow n'attend pas la fin du
  traitement Apple. Le build apparaît dans TestFlight quelques minutes plus tard ;
  pense à remplir les informations de conformité export si demandé.
