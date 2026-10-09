# Realer Durchlauf – 9. Oktober 2026

Crate `ix-trusted-publishing-demo-crates`, Repository https://github.com/fdeitelhoff/ix-trusted-publishing-demo-crates (öffentlich, Environment `crates-io` mit Pflicht-Reviewer und Deployment nur von `main`).

| Punkt | Ergebnis |
|---|---|
| Lokale Prüfung | Windows, rustup 1.29.1, Toolchain `stable-x86_64-pc-windows-gnu` (rustc 1.99.0, cargo 1.99.0; GNU-Toolchain, da keine MSVC Build Tools vorhanden): `cargo generate-lockfile`, `cargo test --locked` (2/2), `cargo package --locked --list`, `cargo publish --locked --dry-run` erfolgreich |
| Action-Pins | SHAs per GitHub-API gegen die Tags geprüft: checkout v6.1.0, crates-io-auth-action v1.0.5 (zugleich aktuellste Version), attest v4.2.2 |
| Bootstrap 0.1.0 | lokal mit Token (Scope `publish-new`, Crate-Scope auf den Namen begrenzt) per `cargo login` / `cargo publish --locked` / `cargo logout`, Token anschließend widerrufen; 17:48:58 UTC; API: `published_by` = `fdeitelhoff`, `trustpub_data` = `null` |
| Trusted Publisher | crates.io, GitHub, `fdeitelhoff` / `ix-trusted-publishing-demo-crates`, `release.yml`, Environment `crates-io` |
| Publish-Lauf 0.1.1 | https://github.com/fdeitelhoff/ix-trusted-publishing-demo-crates/actions/runs/37970148131 – erfolgreich beim ersten Versuch; `check` 18 s, Freigabe nach ca. 33 s, `publish` 15 s; Commit `2001da7419e2b38203c1f81d7125cc76fd780596` |
| Runner | `ubuntu-24.04`, Image 20261004.327.1, rustc 1.99.0, cargo 1.99.0 |
| Tokenaustausch (Log) | „Retrieving GitHub Actions JWT token with audience: crates.io“ → „Requesting token from: https://crates.io/api/v1/trusted_publishing/tokens“ → „Retrieved token successfully“; Post-Schritt: „Token revoked successfully“ |
| Registry-Metadaten 0.1.1 | `published_by` = `null`; `trustpub_data` = `{"provider": "github", "repository": "fdeitelhoff/ix-trusted-publishing-demo-crates", "run_id": "37970148131", "sha": "2001da7…"}` |
| Archiv 0.1.0 | `ix-trusted-publishing-demo-crates-0.1.0.crate`, SHA-256 `d5dc20ac986e242a480e334eb71b4704e3ff5a81260de1aafdcdbde6e0145074` |
| Archiv 0.1.1 | `ix-trusted-publishing-demo-crates-0.1.1.crate`, SHA-256 `c0d9e2777ce2c37870b4f58d9bc8aeff972f215a76b90d0524eeef8a255c8f28` (identisch mit `checksum` im Index) |
| Archivinhalt | 7 Dateien: `.cargo_vcs_info.json`, `Cargo.lock`, `Cargo.toml`, `Cargo.toml.orig`, `LICENSE`, `README.md`, `src/lib.rs`; durchgehend LF |

Beobachtungen, die das README nicht nennt:

- Ein Versionseintrag aus Trusted Publishing hat bei crates.io keinen `published_by`-Nutzer mehr, sondern ein Feld `trustpub_data` mit Provider, Repository, Run-ID und Commit-SHA. Für die per Token veröffentlichte 0.1.0 ist es `null`. Daran lässt sich der Veröffentlichungsweg einer Version über die öffentliche API ablesen.
- Cargo legt `.cargo_vcs_info.json` mit dem Commit-SHA ins Archiv, sobald das Paket in einem Git-Repository liegt (der lokale Dry-run vor `git init` zeigte nur 6 Dateien). Diese Angabe stammt vom Packenden selbst und ist bei 0.1.0 ebenso vorhanden; bestätigt hat crates.io den Commit nur über `trustpub_data` aus dem OIDC-Token.
- Die Action meldet trotz Pin auf v1.0.5 den User-Agent `crates-io-auth-action/1.0.1`; die `package.json` im gepinnten Commit trägt noch Version 1.0.1. Die Versionsangabe im Log taugt daher nicht als Nachweis der verwendeten Action-Version – maßgeblich ist der SHA im Log („Download action repository …“).
- Cargo wartet nach dem Upload selbst, bis die Version im Index verfügbar ist („waiting for … to be available at registry `crates-io`“). Anders als bei npm war Version 0.1.1 unmittelbar danach über API und `static.crates.io` abrufbar.
- Die Trusted-Publishing-Konfiguration eines Crates ist über `GET /api/v1/trusted_publishing/github_configs` nur authentifiziert lesbar (HTTP 403 ohne Anmeldung); öffentlich überprüfbar ist erst das Ergebnis in `trustpub_data`.
- GitHub-Hinweis in beiden Jobs: `ubuntu-latest` wechselt ab 19. Oktober 2026 auf Ubuntu 26. Für Messwiederholungen deshalb Runner-Image protokollieren oder festlegen (siehe README).
- Windows-Stolperstein beim Versionssprung: `Set-Content -Encoding utf8` in Windows PowerShell 5.1 schreibt eine UTF-8-BOM in `Cargo.toml`. Vor dem Commit entfernt.

Nicht ausgeführt: optionale GitHub Artifact Attestation samt `gh attestation verify` und Negativprobe, OIDC-Negativprobe mit abweichendem Environmentnamen, Option `Require trusted publishing for all new versions`.
