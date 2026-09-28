> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop unter Linux (Beta)

> Installieren und aktualisieren Sie die Claude-Desktop-App unter Ubuntu und Debian

<Note>
  Die Linux-Unterstützung für die Claude-Desktop-App befindet sich in der Beta-Phase.
</Note>

Die Desktop-App unter Linux bietet Ihnen die gleiche Chat-, Cowork- und Claude Code-Erfahrung wie auf macOS und Windows: parallele Sitzungen, visuelle Diff-Überprüfung, ein integriertes Terminal und Editor sowie Live-App-Vorschau. Siehe [Claude Code Desktop verwenden](/docs/de/desktop) für die Funktionsreferenz.

<h2 id="requirements">
  Anforderungen
</h2>

* Eine Debian-basierte Distribution: Ubuntu 22.04 oder später oder Debian 12 oder später
* x86\_64 oder arm64

Andere Debian-basierte Distributionen, die diese Anforderungen erfüllen, funktionieren möglicherweise, werden aber nicht offiziell getestet. Auf Distributionen, die nicht Debian-basiert sind, wie Fedora oder Arch, führen Sie stattdessen die [CLI](/docs/de/setup#system-requirements) aus. Wenn Sie unter Windows mit WSL 2 arbeiten, installieren Sie die Windows-Desktop-App und führen Sie Sitzungen in Ihrer Distribution aus; siehe [Claude Code Desktop in WSL](/docs/de/desktop-wsl).

<h3 id="cowork-requirements">
  Cowork-Anforderungen
</h3>

Cowork ist die Desktop-Registerkarte für [Dispatch und längere agentengestützte Arbeiten](https://claude.com/docs/cowork/overview). Unter Linux führt Cowork diese Aufgaben in einer virtuellen Maschine aus, die die Desktop-App mit QEMU und KVM hostet. Um Cowork zu verwenden, benötigt Ihr Computer:

* **Hardware-Virtualisierung**: aktiviert in Ihren Firmware-Einstellungen. Ohne diese meldet die Cowork-Registerkarte „Cowork requires hardware virtualization (KVM)".
* **QEMU und UEFI-Firmware**: `qemu-system-x86`, `ovmf` und `virtiofsd` auf x86\_64 oder `qemu-system-arm`, `qemu-efi-aarch64` und `virtiofsd` auf arm64. `apt install claude-desktop` installiert diese standardmäßig als empfohlene Pakete. Wenn Sie mit `--no-install-recommends` installiert haben oder Ihr System ein minimales Image ist, das empfohlene Pakete überspringt, meldet die Cowork-Registerkarte „Cowork requires QEMU" und zeigt den auszuführenden `apt install`-Befehl an. Ubuntu 22.04 hat kein `virtiofsd`-Paket; die App verwendet dort eine gebündelte Kopie.
* **Zugriff auf `/dev/kvm`**: Fügen Sie Ihren Benutzer mit `sudo usermod -aG kvm $USER` zur `kvm`-Gruppe hinzu, melden Sie sich dann ab und wieder an. Einige Desktop-Umgebungen gewähren dem angemeldeten Benutzer Zugriff auf `/dev/kvm` ohne die Gruppe, aber Cowork benötigt auch `/dev/vhost-vsock`, auf das nur Mitglieder der `kvm`-Gruppe zugreifen können. Treten Sie der Gruppe bei, auch wenn `/dev/kvm` bereits für Sie funktioniert.

Die App überprüft diese Anforderungen einmal beim Start: Starten Sie sie neu, nachdem Sie Pakete installiert haben, und melden Sie sich ab und wieder an, nachdem Sie der Gruppe beigetreten sind. Wenn `/dev/vhost-vsock` fehlt und Ihr laufender Kernel kein Modulverzeichnis unter `/lib/modules` hat, meldet die Cowork-Registerkarte, dass der Kernel die Virtualisierungsunterstützung, die Cowork benötigt, nicht enthält und dass diese nicht manuell hinzugefügt werden kann. Diese Kombination ist häufig auf ChromeOS und in containergestützten Linux-Umgebungen anzutreffen.

<h2 id="install">
  Installation
</h2>

Installieren Sie aus dem apt-Repository von Anthropic, damit Updates über die regulären Paketaktualisierungen Ihres Systems ankommen. Öffnen Sie ein Terminal und führen Sie die Befehle in jedem Schritt aus.

<Steps>
  <Step title="Anthropics apt-Repository hinzufügen">
    Dieser Schritt lädt den Signaturschlüssel mit `curl` herunter und überprüft ihn mit `gpg`, die frische Debian- und Ubuntu-Installationen möglicherweise nicht enthalten. Wenn einer der Befehle `command not found` meldet, installieren Sie zunächst beide:

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Laden Sie den Signaturschlüssel von Anthropic herunter:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Der Befehl gibt nichts aus, wenn er erfolgreich ist, und einen `curl:`-Fehler, wenn er nicht erfolgreich ist. Ein fehlender oder falscher Schlüssel führt dazu, dass `apt update` später mit `NO_PUBKEY BAA929FF1A7ECACE` fehlschlägt. Bestätigen Sie daher, dass der Schlüssel heruntergeladen wurde und zu Anthropic gehört, bevor Sie fortfahren:

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    Der Fingerabdruck, den gpg ausgibt, sollte `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE` sein. Wenn gpg meldet, dass die Datei nicht geöffnet werden kann oder keine gültigen OpenPGP-Daten enthält, ist der Download fehlgeschlagen oder hat den falschen Inhalt zurückgegeben: Bestätigen Sie, dass Ihr Netzwerk `downloads.claude.ai` erreichen kann, und führen Sie dann den Download-Befehl erneut aus.

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

<h3 id="install-from-a-downloaded-file">
  Installation aus einer heruntergeladenen Datei
</h3>

Wenn Sie das apt-Repository nicht verwenden können, laden Sie das `.deb`-Paket direkt aus dem Repository-Paketpool herunter. Dieser Befehl sucht das neueste Paket für Ihre Architektur im Repository-Index auf und lädt es dann in das aktuelle Verzeichnis herunter:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Wenn der Befehl mit `Remote file name has no length` fehlschlägt, hat die Suche keinen Paketpfad zurückgegeben. Dies kann bedeuten, dass der Repository-Index nicht abgerufen werden konnte, beispielsweise wenn Ihr Netzwerk `downloads.claude.ai` blockiert, oder dass kein Paket für Ihre Architektur vorhanden ist. Bestätigen Sie, dass Ihr Netzwerk `downloads.claude.ai` erreichen kann und dass `dpkg --print-architecture` `amd64` oder `arm64` ausgibt; das Repository veröffentlicht keine Pakete für andere Architekturen.

Um ohne Registrierung von Anthropics apt-Repository zu installieren, erstellen Sie zunächst `/etc/default/claude-desktop` mit der Zeile `CLAUDE_DESKTOP_ADD_REPO="false"`. Ohne das Repository liefert apt keine neuen Versionen; zum Aktualisieren führen Sie den Download-Befehl erneut aus und installieren neu, oder [registrieren Sie das Repository](#install) später.

Öffnen Sie dann die heruntergeladene Datei mit Ihrem Software-Installer, z. B. GNOME Software, oder installieren Sie sie mit apt aus dem Verzeichnis, das die heruntergeladene Datei enthält:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Wenn apt `E: Unsupported file ./claude-desktop_*.deb given on commandline` meldet, hat das Muster keine `.deb`-Datei im aktuellen Verzeichnis gefunden. Bestätigen Sie, dass der Download abgeschlossen ist, und führen Sie den Befehl erneut aus dem Verzeichnis aus, das die Datei enthält.

Die Installation des `.deb` registriert auch Anthropics apt-Repository unter `/etc/apt/sources.list.d/claude-desktop.list`, sodass zukünftige Updates mit den [regulären Paketaktualisierungen](#update) Ihres Systems ankommen.

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

Das Deinstallieren des Pakets entfernt auch den Repository-Eintrag und den Signaturschlüssel, den es registriert hat. Wenn Sie den Repository-Eintrag selbst mit dem Schritt [Anthropics apt-Repository hinzufügen](#install) hinzugefügt haben, entfernen Sie ihn auch:

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

* Führen Sie `sudo apt update` nach dem Hinzufügen des Repositorys aus. `apt install` allein erkennt ein Repository nicht, das Sie nach der letzten Ausführung von `apt update` hinzugefügt haben.
* Bestätigen Sie, dass der Repository-Eintrag geschrieben wurde. `cat /etc/apt/sources.list.d/claude-desktop.list` sollte die `deb`-Zeile aus dem Schritt [Anthropics apt-Repository hinzufügen](#install) anzeigen. Wenn die Datei leer oder fehlend ist, führen Sie diesen Schritt erneut aus.
* Bestätigen Sie, dass Ihre Architektur unterstützt wird. `dpkg --print-architecture` sollte `amd64` oder `arm64` ausgeben. Das Repository veröffentlicht keine Pakete für andere Architekturen.
* Führen Sie `sudo apt update` erneut aus und überprüfen Sie die Ausgabe auf Fehler im Zusammenhang mit `downloads.claude.ai`. Ein Netzwerk- oder Schlüsselfehler dort bedeutet, dass das Repository hinzugefügt wurde, aber nicht erreichbar oder nicht verifizierbar war.

Wenn das Repository vorhanden und erreichbar ist und das Paket immer noch nicht gefunden wird, [installieren Sie stattdessen aus einer heruntergeladenen Datei](#install-from-a-downloaded-file).

<h3 id="unmet-dependencies">
  Unerfüllte Abhängigkeiten
</h3>

Wenn `apt` mit `The following packages have unmet dependencies` oder `Unsatisfied dependencies` stoppt, lesen Sie, welche Abhängigkeit es benennt:

* `libc6 (>= 2.34)`: Ihre Distribution ist älter als das Paket unterstützt. Ubuntu 20.04 wird mit `libc6` 2.31 ausgeliefert. Führen Sie ein Upgrade auf Ubuntu 22.04 oder später oder Debian 12 oder später durch.
* Alle fehlenden Abhängigkeiten zeigen `not installable` mit einem `:amd64`- oder `:arm64`-Suffix: Sie haben die `.deb` für eine andere Architektur heruntergeladen als die Ihres Computers. Führen Sie `dpkg --print-architecture` aus und laden Sie die entsprechende `.deb` herunter, oder [installieren Sie aus dem apt-Repository](#install), das das Paket für Ihre Architektur auswählt.

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  Ausführung als root ohne --no-sandbox wird nicht unterstützt
</h3>

Wenn `claude-desktop` mit dieser Meldung beendet wird, haben Sie es als root gestartet. Melden Sie sich als normaler Benutzer an und starten Sie es von dort aus.

<h3 id="cowork-isn’t-available">
  Cowork ist nicht verfügbar
</h3>

Wenn die Cowork-Registerkarte eine dieser Meldungen anzeigt, beheben Sie die Anforderung, die sie benennt, und starten Sie die App neu:

* **Cowork erfordert QEMU**: Installieren Sie die [QEMU- und UEFI-Firmware-Pakete](#cowork-requirements), die die Meldung auflistet.
* **Cowork erfordert Hardware-Virtualisierung (KVM)**: Aktivieren Sie die [Hardware-Virtualisierung](#cowork-requirements) in Ihren Firmware-Einstellungen.
* **Claude hat keine Berechtigung zur Verwendung von Virtualisierung (/dev/kvm)**: Fügen Sie Ihren Benutzer zur [`kvm`-Gruppe](#cowork-requirements) hinzu, melden Sie sich dann ab und wieder an.
* **Cowork erfordert das `vhost_vsock`-Kernel-Modul**: Führen Sie `sudo modprobe vhost_vsock` aus und starten Sie die App neu. Dies lädt das Modul nur für den aktuellen Boot. Um es bei jedem Boot zu laden, führen Sie `echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf` aus.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Was noch nicht in der Linux-Beta enthalten ist
</h2>

* **Computer Use**: [App- und Bildschirmsteuerung](/docs/de/desktop#let-claude-use-your-computer) ist unter Linux nicht verfügbar.
* **Diktat**: Spracheingabe ist in der Linux-Desktop-App nicht verfügbar. Verwenden Sie stattdessen [Sprachdiktat](/docs/de/voice-dictation) in der CLI.
* **Quick Entry Global Hotkey**: funktioniert auf X11. Auf nativem Wayland erfordert es das GlobalShortcuts-Portal Ihrer Desktop-Umgebung.
* **Fedora und RHEL**: Nur Debian-basierte Distributionen werden heute unterstützt. Unterstützung für zusätzliche Distributionen kommt in Zukunft.

Für alles, das in der Desktop-App noch nicht verfügbar ist, führt die [CLI](/docs/de/quickstart) die gleiche Claude Code-Engine aus und unterstützt eine breitere Palette von Linux-Distributionen. Siehe die [Systemanforderungen](/docs/de/setup#system-requirements).
