# Trusted Publishing mit crates.io ausprobieren

Recherche: 7. Oktober 2026. Dieses eigenständige Demopaket enthält keine Fremdabhängigkeiten. Es zeigt eine geprüfte Addition und konzentriert sich auf den Veröffentlichungsweg. Für diesen Entwurf hat kein Registry-Upload stattgefunden.

## Vorbereitung

1. Diesen Ordner samt `.github` in ein eigenes GitHub-Repository kopieren. Alle Befehle beziehen sich auf dessen Wurzel. Standardbranch: `main`.
2. In `Cargo.toml` den Namen `trusted-publishing-demo-change-me` durch einen eigenen, auf crates.io freien Namen ersetzen und `repository` auf die echte Repository-URL setzen. Der Name hat einen globalen Namensraum, keinen npm-artigen Scope.
3. Eine aktuelle stabile Rust-Toolchain installieren. Edition 2024 setzt mindestens Rust 1.85 voraus. Docker ist nicht nötig.
4. Das mitgelieferte minimale `Cargo.lock` nach der Namensänderung mit `cargo generate-lockfile` aktualisieren. `Cargo.lock` mit allen Quelldateien committen. Die Workflow-Prüfungen verwenden `--locked` und erwarten diese Datei. Bei jeder Namensänderung das Lockfile neu erzeugen.

```sh
cargo generate-lockfile
cargo test --locked
cargo package --locked --list
cargo publish --locked --dry-run
```

Erwartung: zwei Tests erfolgreich; die Paketliste enthält nur die beabsichtigten Paketdateien. `--dry-run` prüft das Paket ohne Upload. Er beweist weder die OIDC-Konfiguration noch die Annahme durch crates.io. Cargo kann auch dabei auf den Registry-Index zugreifen.

## Einmaliger Bootstrap

crates.io erwartet ein bereits vorhandenes Crate. Für die erste Version ist daher ein API-Token nötig. In diesem Beispiel bleibt er auf dem eigenen Rechner.

1. Auf crates.io anmelden, E-Mail verifizieren und einen möglichst eng begrenzten API-Token für die Erstveröffentlichung erstellen.
2. Mit `cargo login` den Token interaktiv eingeben. Cargo speichert ihn lokal in seinem Credential Store. Kein Token gehört ins Repository oder in diese Dokumentation.
3. Erst nach Prüfung von Name, Inhalt und Lizenz: `cargo publish --locked`. Das veröffentlicht Version 0.1.0 öffentlich; eine Versionsnummer lässt sich anschließend nicht erneut verwenden.
4. `cargo logout` entfernt den lokalen Eintrag. Den Bootstrap-Token zusätzlich in den crates.io-Kontoeinstellungen widerrufen: Logout allein widerruft ihn nicht.

Für reine Trockenübungen nach dem Dry-run stoppen. crates.io hat ein Staging-System, aber dieses Beispiel setzt keinen dauerhaft verfügbaren TestPyPI-ähnlichen Service voraus.

## Vertrauensbeziehung einrichten

Im GitHub-Repository unter Settings → Environments das Environment `crates-io` anlegen. Deployment branches auf `main` begrenzen; verfügbare Review-Freigaben bei Bedarf aktivieren. Schutzfunktionen hängen von Repository-Sichtbarkeit und GitHub-Tarif ab. Ein Environment-Name allein schafft keine Freigaberegel.

Auf crates.io beim eigenen Crate unter Settings → Trusted Publishing einen GitHub-Publisher eintragen:

| Feld | Wert |
|---|---|
| Repository owner | eigener GitHub-Nutzer oder Organisation |
| Repository name | Name des eigenen Repositorys |
| Workflow filename | `release.yml` |
| Environment | `crates-io` |

`release.yml` bezeichnet die Datei, nicht den frei wählbaren Anzeigenamen des Workflows. Die Datei liegt unter `.github/workflows/release.yml`. Owner, Repository, Workflow und Environment müssen mit dem tatsächlichen Lauf zusammenpassen.

## Tokenlose Folgeversion veröffentlichen

1. `Cargo.toml` auf Version 0.1.1 ändern, mit `cargo generate-lockfile` das Lockfile aktualisieren, prüfen, committen und nach `main` pushen.
2. Im Actions-Tab `Publish crate` wählen und `Run workflow` auf `main` starten.
3. Falls konfiguriert, die Environment-Freigabe erteilen.
4. Tests und Dry-run laufen in einem eigenen Job ohne `id-token: write`. Der Publish-Job startet erst nach dessen Erfolg. Erst danach tauscht die Auth-Action ein GitHub-OIDC-Token gegen einen auf die passenden Crates begrenzten Registry-Token aus. Er gilt maximal 30 Minuten; der Post-Schritt der Action widerruft ihn bei Jobende.
5. Version und Metadaten auf crates.io prüfen. Nach dem erfolgreichen Umstieg alte CI-Tokens widerrufen, entsprechende Secrets entfernen und im Crate die Option `Require trusted publishing for all new versions` aktivieren.

Das Workflow-Secret `CARGO_REGISTRY_TOKEN` existiert nicht: Gleichnamig ist hier lediglich eine Umgebungsvariable des Publish-Schritts, gespeist aus dem kurzlebigen Action-Output. „Tokenlos“ meint den Verzicht auf selbst verwaltete langlebige Tokens.

Cargo verpackt und prüft bei `cargo publish` erneut. Diese Demo trennt den vorgelagerten Testjob ab, erreicht aber keine vollständige Trennung aller Paketverarbeitung vom privilegierten Publish-Job. Abhängigkeiten mit Build-Skripten erfordern eine gesonderte Betrachtung; dieses Paket enthält keine.

Die vollständigen Commit-SHAs fixieren die Action-Versionen. Prüfpunkte: checkout v6.1.0 und crates-io-auth-action v1.0.5. Rust `stable` und `ubuntu-latest` bleiben beweglich; für spätere Messwiederholungen Toolchain und Runner-Image protokollieren oder gezielt festlegen.

## Option: Herkunft des Archivs nachweisen

GitHub Artifact Attestations ergänzen den Upload unabhängig von crates.io. Die Datei `snippets/attestation.yml.txt` zeigt den zusätzlichen Schritt. Im Publish-Job außerdem `attestations: write` ergänzen; `id-token: write` und `contents: read` sind schon gesetzt. Der Schritt muss nach `cargo publish` laufen, damit er das dort entstandene `.crate`-Archiv erfasst.

Die anschließende Prüfung des tatsächlichen Registry-Downloads muss die Übereinstimmung des Digests bestätigen. Dieser End-to-End-Test steht noch aus. Ein erfolgreicher Upload allein belegt die Archivübereinstimmung für diese Demo nicht.

Für öffentliche Repositorys ist die Funktion in aktuellen GitHub-Tarifen verfügbar. Private/interne Repositorys benötigen GitHub Enterprise Cloud. Es handelt sich um eine von GitHub gespeicherte Attestation; dieses Beispiel behauptet keine native crates.io-Attestationsfunktion.

Nach einem erfolgreichen Workflow mit Attestierung lokal die tatsächlich ausgelieferten Bytes prüfen. Die folgenden Shell-Variablen durch die eigenen Werte ersetzen:

```sh
CRATE=mein-eindeutiger-crate-name
VERSION=0.1.1
REPO=OWNER/REPOSITORY
curl --fail --location --output "$CRATE-$VERSION.crate" \
  "https://static.crates.io/crates/$CRATE/$CRATE-$VERSION.crate"
gh attestation verify "$CRATE-$VERSION.crate" \
  --repo "$REPO" \
  --signer-workflow "$REPO/.github/workflows/release.yml" \
  --deny-self-hosted-runners
```

Voraussetzung ist eine aktuelle GitHub CLI, gegebenenfalls mit `gh auth login` angemeldet. Mit `gh attestation verify --help` die verfügbaren Policy-Optionen prüfen. Die erwartete Repository- und Workflow-Identität vorgeben; eine beliebige gültige Signatur reicht als Vertrauensentscheidung nicht aus. Für eine konkrete Revision lässt sich zusätzlich `--source-digest` mit dem erwarteten Commit-SHA setzen.

Negativprobe ohne erneuten Upload: Archiv kopieren, ein Byte an die Kopie anhängen und den Prüfaufruf mit ihr wiederholen. Die Digest-Prüfung muss scheitern. Eine erfolgreiche Prüfung des Originals belegt die Zuordnung von Bytes zu einer Workflow-Identität. Sie bescheinigt weder Fehlerfreiheit noch fehlende Schadfunktionen.

## Fehler gezielt nachvollziehen

- Für eine OIDC-Negativprobe vorübergehend nur in der lokalen Workflow-Kopie `environment` falsch schreiben. Ein realer Lauf mit einer anderen Identität scheitert bereits beim Tokenaustausch und veröffentlicht nichts. Danach auf den registrierten Namen zurückstellen.
- Fehlendes `id-token: write`: GitHub kann dem Job das erforderliche OIDC-Token nicht liefern.
- Falscher Workflow-Dateiname oder falsches Repository: crates.io findet keine passende Vertrauensbeziehung.
- 0.1.0 noch einmal publizieren: Versionskonflikt, selbst bei korrekter Authentifizierung.
- `--locked` meldet veraltetes Lockfile: lokal `cargo generate-lockfile` ausführen und committen.
- `pull_request_target` und `workflow_run` eignen sich hier nicht: crates.io blockiert beide Trigger.

## Prüfstatus des Entwurfs

Manifest und YAML haben eine lokale Strukturprüfung erhalten. Die Umgebung enthielt weder `cargo` noch `rustc`; Kompilierung, Rust-Tests und Cargo-Dry-run konnten hier deshalb nicht laufen. Kein GitHub-Actions-Lauf, OIDC-Austausch, Registry-Upload oder echter Attestationsnachweis fand statt. Die obigen Schritte benennen die noch nötigen praktischen Prüfungen.

## Primärquellen

- https://crates.io/docs/trusted-publishing
- https://github.com/rust-lang/crates.io/blob/main/svelte/src/routes/docs/trusted-publishing/%2Bpage.svelte
- https://blog.rust-lang.org/2026/01/21/crates-io-development-update/
- https://github.com/rust-lang/crates-io-auth-action
- https://doc.rust-lang.org/cargo/reference/publishing.html
- https://doc.rust-lang.org/cargo/commands/cargo-publish.html
- https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations
- https://cli.github.com/manual/gh_attestation_verify
