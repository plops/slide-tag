# Release Process

Dieses Projekt nutzt GitHub Actions, um automatisch Release-Binaries für Linux (x86_64) zu bauen. Dies ist ideal für das Deployment auf Hetzner Ubuntu-Servern.

## Automatischer Release

Releases werden getriggert, sobald ein Git-Tag gepusht wird, das mit `v` beginnt (z.B. `v1.0.0`).

### Schritt-für-Schritt Anleitung

1.  **Version anpassen**:
    Aktualisiere die Version in der `Cargo.toml`.
    ```toml
    [package]
    name = "rs-scrape"
    version = "1.0.0"
    ```

2.  **Commit und Push**:
    ```bash
    git add .
    git commit -m "Prepare release v1.0.0"
    git push origin main
    ```

3.  **Tag erstellen und pushen**:
    ```bash
    git tag v1.0.0
    git push origin v1.0.0
    ```

4.  **Download Asset**:
    Unter dem Reiter **Releases** auf GitHub erscheint nach wenigen Minuten die Datei `rs-scrape-linux-x86_64.tar.gz`. Diese kann direkt auf den Hetzner Server heruntergeladen und entpackt werden.
