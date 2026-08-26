> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop unter Linux (Beta)

> Installieren und aktualisieren Sie die Claude-Desktop-App unter Ubuntu und Debian

<Note>
  Die Linux-Unterstützung für die Claude-Desktop-App befindet sich in der Beta-Phase. Die Registerkarten Chat, Cowork und Code sind alle verfügbar.
</Note>

Die Desktop-App unter Linux bietet Ihnen die gleiche Chat-, Cowork- und Claude Code-Erfahrung wie macOS und Windows: parallele Sitzungen, visuelle Diff-Überprüfung, ein integriertes Terminal und Editor sowie Live-App-Vorschau. Siehe [Claude Code Desktop verwenden](/docs/de/desktop) für die vollständige Funktionsreferenz.

<h2 id="requirements">
  Anforderungen
</h2>

* Ubuntu 22.04 oder später oder Debian 12 oder später
* x86\_64 oder arm64

Andere Debian-basierte Distributionen, die diese Anforderungen erfüllen, funktionieren möglicherweise, werden aber nicht offiziell getestet.

<h2 id="install">
  Installation
</h2>

Installieren Sie aus dem apt-Repository von Anthropic, damit Updates über die regulären Paketaktualisierungen Ihres Systems ankommen. Öffnen Sie ein Terminal und führen Sie die Befehle in jedem Schritt aus.

<Steps>
  <Step title="Anthropics apt-Repository hinzufügen">
    Dieser Schritt lädt den Signaturschlüssel mit `curl` herunter, den frische Debian- und Ubuntu-Installationen möglicherweise nicht enthalten. Wenn der Download-Befehl mit `sudo: curl: command not found` fehlschlägt, installieren Sie zunächst curl:

    ```bash theme={null}
    sudo apt install curl
    ```

    Laden Sie den Signaturschlüssel von Anthropic herunter:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Registrieren Sie das Repository:

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="Installieren Sie das Paket">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="Starten und anmelden">
    Starten Sie **Claude** über Ihren Anwendungsstarter oder führen Sie `claude-desktop` von einem Terminal aus aus und melden Sie sich mit Ihrem Anthropic-Konto an.

    Die Linux-App meldet sich auf die gleiche Weise an wie auf macOS und Windows: mit einem claude.ai-Abonnement oder über das SSO Ihrer Organisation. Desktop akzeptiert keinen Claude Console API-Schlüssel direkt; verwenden Sie die [CLI](/docs/de/quickstart) für die API-Schlüssel-Authentifizierung. Für Enterprise-Bereitstellungen, die Desktop zu Googles Agent Platform oder einem LLM-Gateway weiterleiten, siehe [Claude Desktop auf 3P](https://claude.com/docs/third-party/claude-desktop/overview) und [Netzwerkkonfiguration](/docs/de/network-config).
  </Step>
</Steps>

<Accordion title="Signaturschlüssel überprüfen">
  Sie können bestätigen, dass der heruntergeladene Signaturschlüssel zu Anthropic gehört:

  ```bash theme={null}
  gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
  ```

  Der Fingerabdruck sollte `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` sein.
</Accordion>

<h3 id="install-from-a-downloaded-file">
  Installation aus einer heruntergeladenen Datei
</h3>

Wenn Sie das apt-Repository nicht verwenden können, laden Sie das `.deb`-Paket direkt aus dem Repository-Paketpool herunter. Dieser Befehl sucht das neueste Paket für Ihre Architektur im Repository-Index auf und lädt es dann in das aktuelle Verzeichnis herunter:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Wenn der Befehl mit `Remote file name has no length` fehlschlägt, hat die Suche keinen Paketpfad zurückgegeben. Dies kann bedeuten, dass der Repository-Index nicht abgerufen werden konnte, beispielsweise wenn Ihr Netzwerk `downloads.claude.ai` blockiert, oder dass kein Paket für Ihre Architektur vorhanden ist. Bestätigen Sie, dass Ihr Netzwerk `downloads.claude.ai` erreichen kann und dass `dpkg --print-architecture` `amd64` oder `arm64` ausgibt; das Repository veröffentlicht keine Pakete für andere Architekturen.

Öffnen Sie dann die heruntergeladene Datei mit Ihrem Software-Installer, z. B. GNOME Software, oder installieren Sie sie mit apt aus dem Verzeichnis, das die heruntergeladene Datei enthält:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Wenn apt `E: Unsupported file ./claude-desktop_*.deb given on commandline` meldet, hat das Muster keine `.deb`-Datei im aktuellen Verzeichnis gefunden. Bestätigen Sie, dass der Download abgeschlossen ist, und führen Sie den Befehl erneut aus dem Verzeichnis aus, das die Datei enthält.

Ein auf diese Weise installiertes `.deb`-Paket erhält keine Updates. Um Updates über apt zu erhalten, registrieren Sie das Repository aus dem Schritt [Anthropics apt-Repository hinzufügen](#install). Das Paket schreibt auch einen auskommentierten Repository-Eintrag in `/etc/apt/sources.list.d/claude-desktop.list`; das Auskommentieren seiner `deb`-Zeile ist gleichwertig.

<h2 id="update">
  Aktualisierung
</h2>

Die Desktop-App aktualisiert sich unter Linux nicht selbst. Updates kommen mit den regulären Paketaktualisierungen Ihres Systems:

```bash theme={null}
sudo apt update && sudo apt upgrade
```

Der grafische Software-Updater Ihrer Distribution wird auch neue Versionen erkennen.

<h2 id="uninstall">
  Deinstallation
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

Dies entfernt den Signaturschlüssel zusammen mit der App. Wenn Sie den Repository-Eintrag während der Installation hinzugefügt haben, entfernen Sie ihn auch:

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  Fehlerbehebung
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  Paket claude-desktop kann nicht gefunden werden
</h3>

Wenn `sudo apt install claude-desktop` mit `E: Unable to locate package claude-desktop` fehlschlägt, hat apt das hinzugefügte Repository nicht gefunden. Überprüfen Sie Folgendes:

* Bestätigen Sie, dass der Repository-Eintrag geschrieben wurde. `cat /etc/apt/sources.list.d/claude-desktop.list` sollte die `deb`-Zeile aus dem Schritt [Anthropics apt-Repository hinzufügen](#install) anzeigen. Wenn die Datei leer oder fehlend ist, führen Sie diesen Schritt erneut aus.
* Bestätigen Sie, dass Ihre Architektur unterstützt wird. `dpkg --print-architecture` sollte `amd64` oder `arm64` ausgeben. Das Repository veröffentlicht keine Pakete für andere Architekturen.
* Führen Sie `sudo apt update` erneut aus und überprüfen Sie die Ausgabe auf Fehler im Zusammenhang mit `downloads.claude.ai`. Ein Netzwerk- oder Schlüsselfehler dort bedeutet, dass das Repository hinzugefügt wurde, aber nicht erreichbar oder nicht verifizierbar war.

Wenn das Repository vorhanden und erreichbar ist und das Paket immer noch nicht gefunden wird, [installieren Sie stattdessen aus einer heruntergeladenen Datei](#install-from-a-downloaded-file).

<h2 id="what’s-not-in-the-linux-beta-yet">
  Was noch nicht in der Linux-Beta enthalten ist
</h2>

* **Computer Use**: [App- und Bildschirmsteuerung](/docs/de/desktop#let-claude-use-your-computer) ist unter Linux nicht verfügbar.
* **Diktat**: Spracheingabe ist in der Linux-Desktop-App nicht verfügbar. Verwenden Sie stattdessen [Sprachdiktat](/docs/de/voice-dictation) in der CLI.
* **Quick Entry Global Hotkey**: funktioniert auf X11. Auf nativem Wayland erfordert es das GlobalShortcuts-Portal Ihrer Desktop-Umgebung.
* **Fedora und RHEL**: Nur Debian-basierte Distributionen werden heute unterstützt. Unterstützung für zusätzliche Distributionen kommt in Zukunft.

Für alles, das in der Desktop-App noch nicht verfügbar ist, führt die [CLI](/docs/de/quickstart) die gleiche Claude Code-Engine aus und unterstützt eine breitere Palette von Linux-Distributionen. Siehe die [Systemanforderungen](/docs/de/setup#system-requirements).
