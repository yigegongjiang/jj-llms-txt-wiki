> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Erweiterte Einrichtung

> Systemanforderungen, plattformspezifische Installation, Versionsverwaltung und Deinstallation für Claude Code.

Diese Seite behandelt Systemanforderungen, plattformspezifische Installationsdetails, Updates und Deinstallation. Eine geführte Anleitung für Ihre erste Sitzung finden Sie im [Schnellstart](/docs/de/quickstart). Wenn Sie noch nie ein Terminal verwendet haben, siehe [Terminalanleitung](/docs/de/terminal-guide).

<h2 id="system-requirements">
  Systemanforderungen
</h2>

Claude Code läuft auf den folgenden Plattformen und Konfigurationen:

* **Betriebssystem**:
  * macOS 13.0+
  * Windows 10 1809+ oder Windows Server 2019+
  * Ubuntu 20.04+
  * Debian 10+
  * Alpine Linux 3.19+
* **Hardware**: 4 GB+ RAM, x64 oder ARM64 Prozessor
* **Netzwerk**: Internetverbindung erforderlich. Siehe [Netzwerkkonfiguration](/docs/de/network-config#network-access-requirements).
* **Shell**: Bash, Zsh, PowerShell oder CMD.
* **Standort**: [Von Anthropic unterstützte Länder](https://www.anthropic.com/supported-countries)

<h3 id="additional-dependencies">
  Zusätzliche Abhängigkeiten
</h3>

* **ripgrep**: normalerweise in Claude Code enthalten. Falls die Suche fehlschlägt, siehe [Suche-Fehlerbehebung](/docs/de/troubleshooting#search-and-discovery-issues).

<h2 id="install-claude-code">
  Claude Code installieren
</h2>

<Tip>
  Bevorzugen Sie eine grafische Benutzeroberfläche? Die [Desktop-App](/docs/de/desktop-quickstart) ermöglicht es Ihnen, Claude Code ohne das Terminal zu verwenden. Laden Sie sie für [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs), [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) oder [Linux](/docs/de/desktop-linux) herunter.

  Neu im Terminal? Siehe die [Terminalanleitung](/docs/de/terminal-guide) für Schritt-für-Schritt-Anweisungen.
</Tip>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

Nach Abschluss der Installation öffnen Sie ein Terminal in dem Projekt, an dem Sie arbeiten möchten, und starten Sie Claude Code:

```bash theme={null}
claude
```

Wenn während der Installation Probleme auftreten, siehe [Fehlerbehebung bei Installation und Anmeldung](/docs/de/troubleshoot-install).

<h3 id="set-up-on-windows">
  Einrichtung unter Windows
</h3>

Sie können Claude Code nativ unter Windows oder in WSL ausführen. Wählen Sie basierend darauf, wo sich Ihre Projekte befinden und welche Funktionen Sie benötigen:

| Option          | Erfordert                                                                | [Sandboxing](/docs/de/sandboxing) | Wann zu verwenden                               |
| --------------- | ------------------------------------------------------------------------ | ---------------------------- | ----------------------------------------------- |
| Natives Windows | Keine; [Git für Windows](https://git-scm.com/downloads/win) ist optional | Nicht unterstützt            | Windows-native Projekte und Tools               |
| WSL 2           | WSL 2 aktiviert                                                          | Unterstützt                  | Linux-Toolchains oder Sandbox-Befehlsausführung |
| WSL 1           | WSL 1 aktiviert                                                          | Nicht unterstützt            | Wenn WSL 2 nicht verfügbar ist                  |

**Option 1: Natives Windows**

Führen Sie den Installationsbefehl von PowerShell oder CMD aus. Sie müssen nicht als Administrator ausführen. Die Installation von [Git für Windows](https://git-scm.com/downloads/win) ist optional. Dies ermöglicht das [Bash-Tool](/docs/de/tools-reference#bash-tool-behavior) durch Bereitstellung von Git Bash.

Ob Sie von PowerShell oder CMD installieren, wirkt sich nur auf den Installationsbefehl aus, den Sie ausführen. Ihre Eingabeaufforderung zeigt `PS C:\Users\YourName>` in PowerShell und `C:\Users\YourName>` ohne das `PS` in CMD. Wenn Sie neu im Terminal sind, führt die [Terminalanleitung](/docs/de/terminal-guide#windows) Sie durch jeden Schritt.

Nach der Installation starten Sie `claude` von jedem Terminal aus.

* **Ohne Git für Windows** führt Claude Code Shell-Befehle über das [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) aus.
* **Mit Git für Windows** verwendet Claude Code Git Bash für das [Bash-Tool](/docs/de/tools-reference#bash-tool-behavior). Wenn Claude Code Git Bash nicht finden kann, legen Sie den Pfad in Ihrer [settings.json-Datei](/docs/de/settings) fest:

  ```json theme={null}
  {
    "env": {
      "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
    }
  }
  ```

Wenn Git für Windows installiert ist, wird das PowerShell-Tool schrittweise als zusätzliche Option neben Bash eingeführt. Setzen Sie `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`, um sich anzumelden, oder `0`, um sich abzumelden. Siehe [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) für Einrichtung und Einschränkungen.

**Option 2: WSL**

Öffnen Sie Ihre WSL-Distribution und führen Sie das Linux-Installationsprogramm aus den [Installationsanweisungen](#install-claude-code) oben aus. Sie installieren und starten `claude` im WSL-Terminal, nicht von PowerShell oder CMD.

<h3 id="alpine-linux-and-musl-based-distributions">
  Alpine Linux und musl-basierte Distributionen
</h3>

Das native Installationsprogramm auf Alpine und anderen musl/uClibc-basierten Distributionen erfordert `libgcc`, `libstdc++` und `ripgrep`. Installieren Sie diese mit dem Paketmanager Ihrer Distribution, und setzen Sie dann `USE_BUILTIN_RIPGREP=0`.

Dieses Beispiel installiert die erforderlichen Pakete auf Alpine:

```bash theme={null}
apk add libgcc libstdc++ ripgrep
```

Setzen Sie dann `USE_BUILTIN_RIPGREP` auf `0` in Ihrer [`settings.json`](/docs/de/settings#available-settings)-Datei:

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

<h2 id="verify-your-installation">
  Installation überprüfen
</h2>

Nach der Installation bestätigen Sie, dass Claude Code funktioniert:

```bash theme={null}
claude --version
```

Wenn dies mit `command not found` oder einem anderen Fehler fehlschlägt, siehe [Installation und Anmeldung beheben](/docs/de/troubleshoot-install).

Für eine detailliertere Überprüfung Ihrer Installation und Konfiguration führen Sie [`claude doctor`](/docs/de/troubleshooting#get-more-help) aus:

```bash theme={null}
claude doctor
```

<h2 id="authenticate">
  Authentifizierung
</h2>

Claude Code erfordert ein Pro-, Max-, Team-, Enterprise- oder Console-Konto. Der kostenlose Claude.ai-Plan beinhaltet keinen Claude Code-Zugriff. Sie können Claude Code auch mit einem Drittanbieter-API-Provider wie [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry) verwenden.

Nach der Installation melden Sie sich an, indem Sie `claude` ausführen und den Browser-Aufforderungen folgen. Siehe [Authentifizierung](/docs/de/authentication) für alle Kontotypen und Team-Setup-Optionen.

<h2 id="update-claude-code">
  Claude Code aktualisieren
</h2>

Native Installationen werden automatisch im Hintergrund aktualisiert. Sie können [den Release-Kanal konfigurieren](#configure-release-channel), um zu steuern, ob Sie Updates sofort oder nach einem verzögerten stabilen Zeitplan erhalten, oder [Auto-Updates vollständig deaktivieren](#disable-auto-updates). Homebrew-, WinGet- und [Linux-Paketmanager](#install-with-linux-package-managers)-Installationen erfordern standardmäßig manuelle Updates.

<h3 id="auto-updates">
  Auto-Updates
</h3>

Claude Code prüft beim Start und regelmäßig während der Ausführung auf Updates. Updates werden im Hintergrund heruntergeladen und installiert und treten beim nächsten Start von Claude Code in Kraft.

Führen Sie `claude doctor` aus, um das Ergebnis des letzten Aktualisierungsversuchs anzuzeigen.

Unter macOS und Linux verwaltet das native Installationsprogramm das Startprogramm unter `~/.local/bin/claude` als Symlink in `~/.local/share/claude/versions/`. Wenn Sie dieses Startprogramm durch Ihr eigenes Skript oder Symlink ersetzen, lassen Auto-Update und `claude update` es an Ort und Stelle: neue Versionen werden weiterhin im Verzeichnis `versions/` installiert, und Ihr Startprogramm entscheidet, welche Version ausgeführt wird. Vor v2.1.207 ersetzte der Auto-Updater ein benutzerdefiniertes Startprogramm bei diesem Pfad bei jedem Update durch seinen eigenen Symlink.

Mit einem benutzerdefinierten Startprogramm behält Claude Code auch jede installierte Version auf der Festplatte, da es nicht erkennen kann, welche Version das Startprogramm benötigt. `claude doctor` meldet ein Startprogramm, das das native Installationsprogramm nicht erstellt hat.

Um Claude Code das Startprogramm wieder verwalten zu lassen, entfernen Sie `~/.local/bin/claude` und führen Sie `claude update` aus.

Wenn eine npm-Globalinstallation nicht automatisch aktualisiert werden kann, weil das npm-Globalverzeichnis nicht beschreibbar ist, zeigt Claude Code beim Start eine einmalige Benachrichtigung an, und `claude doctor` listet die verfügbaren Fixes auf. Weitere Informationen finden Sie unter [Berechtigungsfehler während der Installation](/docs/de/troubleshoot-install#permission-errors-during-installation).

<Note>
  Homebrew-, WinGet-, apt-, dnf- und apk-Installationen werden standardmäßig nicht automatisch aktualisiert. Weitere Informationen finden Sie unten, um sich für Homebrew und WinGet anzumelden. Um Homebrew manuell zu aktualisieren, führen Sie `brew upgrade claude-code` oder `brew upgrade claude-code@latest` aus, je nachdem, welches Cask Sie installiert haben. Für WinGet führen Sie `winget upgrade Anthropic.ClaudeCode` aus. Für Linux-Paketmanager siehe die Upgrade-Befehle in [Mit Linux-Paketmanagern installieren](#install-with-linux-package-managers).

  Um Claude Code den Upgrade-Befehl für Sie auf Homebrew oder WinGet ausführen zu lassen, setzen Sie [`CLAUDE_CODE_PACKAGE_MANAGER_AUTO_UPDATE`](/docs/de/env-vars) auf `1`. Claude Code führt dann das Upgrade im Hintergrund aus, wenn eine neue Version verfügbar ist, und zeigt bei Erfolg eine Neustartaufforderung an. Das Upgrade zielt nur auf das Claude Code-Paket ab und beeinträchtigt keine andere Software, die Sie installiert haben.

  Unter WinGet kann das Upgrade fehlschlagen, während Claude Code ausgeführt wird, da Windows die ausführbare Datei sperrt. In diesem Fall zeigt Claude Code stattdessen den manuellen Befehl an. apt, dnf und apk erfordern weiterhin ein manuelles Upgrade, da diese Befehle erhöhte Berechtigungen benötigen.

  **Bekanntes Problem:** Claude Code kann Sie über Updates benachrichtigen, bevor die neue Version in diesen Paketmanagern verfügbar ist. Wenn ein Upgrade fehlschlägt, warten Sie und versuchen Sie es später erneut.

  Homebrew behält alte Versionen nach Upgrades auf der Festplatte. Führen Sie regelmäßig `brew cleanup` aus, um Speicherplatz freizugeben.
</Note>

<h3 id="configure-release-channel">
  Release-Kanal konfigurieren
</h3>

Steuern Sie, welchem Release-Kanal Claude Code für Auto-Updates und `claude update` folgt, mit der Einstellung `autoUpdatesChannel`:

* `"latest"`, die Standardeinstellung: Erhalten Sie neue Funktionen, sobald sie veröffentlicht werden
* `"stable"`: Verwenden Sie eine Version, die normalerweise etwa eine Woche alt ist und überspringen Sie Releases mit großen Regressionen

Konfigurieren Sie dies über `/config` → **Auto-update channel**, oder fügen Sie es zu Ihrer [settings.json-Datei](/docs/de/settings) hinzu:

```json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Für Enterprise-Bereitstellungen können Sie einen konsistenten Release-Kanal in Ihrer Organisation mit [verwalteten Einstellungen](/docs/de/permissions#managed-settings) erzwingen.

Homebrew-Installationen wählen einen Kanal nach Cask-Name statt dieser Einstellung: `claude-code` verfolgt stabil und `claude-code@latest` verfolgt neueste.

<h3 id="pin-a-minimum-version">
  Mindestversion festlegen
</h3>

Die Einstellung `minimumVersion` etabliert eine Untergrenze. Hintergrund-Auto-Updates und `claude update` weigern sich, eine Version unter diesem Wert zu installieren, daher führt ein Wechsel zum Kanal `"stable"` nicht zu einem Downgrade, wenn Sie bereits auf einem neueren `"latest"`-Build sind.

Ein Wechsel von `"latest"` zu `"stable"` über `/config` fordert Sie auf, entweder bei der aktuellen Version zu bleiben oder das Downgrade zuzulassen. Wenn Sie sich entscheiden zu bleiben, wird `minimumVersion` auf diese Version gesetzt. Ein Wechsel zurück zu `"latest"` löscht es.

Fügen Sie es zu Ihrer [settings.json-Datei](/docs/de/settings) hinzu, um eine Untergrenze explizit festzulegen:

```json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

In [verwalteten Einstellungen](/docs/de/permissions#managed-settings) erzwingt dies ein organisationsweites Minimum, das Benutzer- und Projekteinstellungen nicht überschreiben können.

Die `minimumVersion`-Festlegung beschränkt nur Updates. Um Claude Code zu zwingen, außerhalb eines Versionsbereichs nicht zu starten, verwenden Sie stattdessen die verwalteten Einstellungen `requiredMinimumVersion` und `requiredMaximumVersion`. Updates respektieren auch die `requiredMaximumVersion`-Obergrenze. Siehe [verfügbare Einstellungen](/docs/de/settings#available-settings).

<h3 id="disable-auto-updates">
  Auto-Updates deaktivieren
</h3>

Setzen Sie `DISABLE_AUTOUPDATER` auf `"1"` im `env`-Schlüssel Ihrer [`settings.json`](/docs/de/settings#available-settings)-Datei:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

`DISABLE_AUTOUPDATER` stoppt nur die Hintergrundprüfung; `claude update` und `claude install` funktionieren weiterhin. Um alle Update-Pfade, einschließlich manueller Updates, zu blockieren, setzen Sie stattdessen [`DISABLE_UPDATES`](/docs/de/env-vars). Verwenden Sie dies, wenn Sie Claude Code über Ihre eigenen Kanäle verteilen und Benutzer auf der Version bleiben müssen, die Sie bereitstellen.

<h3 id="update-manually">
  Manuell aktualisieren
</h3>

Um ein Update sofort anzuwenden, ohne auf die nächste Hintergrundprüfung zu warten, führen Sie aus:

```bash theme={null}
claude update
```

<h2 id="advanced-installation-options">
  Erweiterte Installationsoptionen
</h2>

Diese Optionen sind für Versions-Pinning, Linux-Paketmanager, npm und Überprüfung der Binärintegrität.

<h3 id="install-a-specific-version">
  Eine bestimmte Version installieren
</h3>

Das native Installationsprogramm akzeptiert entweder eine bestimmte Versionsnummer oder einen Release-Kanal (`latest` oder `stable`). Der Kanal, den Sie bei der Installation wählen, wird zu Ihrem Standard für Auto-Updates. Siehe [Release-Kanal konfigurieren](#configure-release-channel) für weitere Informationen.

So installieren Sie die neueste Version (Standard):

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```
  </Tab>
</Tabs>

So installieren Sie die stabile Version:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s stable
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) stable
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd stable && del install.cmd
    ```
  </Tab>
</Tabs>

So installieren Sie eine bestimmte Versionsnummer:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) 2.1.89
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd 2.1.89 && del install.cmd
    ```
  </Tab>
</Tabs>

<h3 id="install-with-linux-package-managers">
  Mit Linux-Paketmanagern installieren
</h3>

Claude Code veröffentlicht signierte apt-, dnf- und apk-Repositories. Jedes Repository bietet zwei Kanäle: `stable` stellt eine Version bereit, die typischerweise etwa eine Woche alt ist und Releases mit großen Regressionen überspringt, und `latest` stellt jeden Release bereit, sobald er veröffentlicht wird. Die folgenden Befehle konfigurieren den `stable`-Kanal, der für die meisten Benutzer geeignet ist; jeder Tab zeigt auch die `latest`-Repository-URL. Paketmanager-Installationen werden nicht automatisch durch Claude Code aktualisiert; Updates erfolgen durch Ihren normalen System-Upgrade-Workflow.

Alle Repositories sind mit dem [Claude Code Release-Signaturschlüssel](#binary-integrity-and-code-signing) signiert. Bevor Sie dem Schlüssel vertrauen, überprüfen Sie ihn wie in jedem Tab beschrieben.

<Tabs>
  <Tab title="apt">
    Für Debian und Ubuntu. Die Installationsbefehle unten laden den Signaturschlüssel mit `curl` herunter, den frische Debian- und Ubuntu-Installationen möglicherweise nicht enthalten. Wenn der Download mit `sudo: curl: command not found` fehlschlägt, installieren Sie zuerst curl:

    ```bash theme={null}
    sudo apt install curl
    ```

    Die folgenden Befehle konfigurieren den `stable`-Kanal:

    ```bash theme={null}
    sudo install -d -m 0755 /etc/apt/keyrings
    sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
      -o /etc/apt/keyrings/claude-code.asc
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    sudo apt update
    sudo apt install claude-code
    ```

    Um stattdessen den `latest`-Kanal zu verwenden, ändern sich sowohl der URL-Pfad als auch der Suite-Name. Verwenden Sie diese `deb`-Zeile:

    ```bash theme={null}
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/latest latest main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    ```

    Überprüfen Sie den GPG-Schlüssel-Fingerabdruck, bevor Sie ihm vertrauen: `gpg --show-keys /etc/apt/keyrings/claude-code.asc` sollte `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` melden.

    Um später zu aktualisieren, führen Sie `sudo apt update && sudo apt upgrade claude-code` aus.
  </Tab>

  <Tab title="dnf">
    Für Fedora und RHEL. Die folgenden Befehle konfigurieren den `stable`-Kanal:

    ```bash theme={null}
    sudo tee /etc/yum.repos.d/claude-code.repo <<'EOF'
    [claude-code]
    name=Claude Code
    baseurl=https://downloads.claude.ai/claude-code/rpm/stable
    enabled=1
    gpgcheck=1
    gpgkey=https://downloads.claude.ai/keys/claude-code.asc
    EOF
    sudo dnf install claude-code
    ```

    Um stattdessen den `latest`-Kanal zu verwenden, setzen Sie `baseurl` auf das `latest`-Repository:

    ```ini theme={null}
    baseurl=https://downloads.claude.ai/claude-code/rpm/latest
    ```

    dnf lädt den Schlüssel bei der ersten Installation herunter und fordert Sie auf, den Fingerabdruck zu bestätigen. Überprüfen Sie, ob er `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` entspricht, bevor Sie akzeptieren.

    Um später zu aktualisieren, führen Sie `sudo dnf upgrade claude-code` aus.
  </Tab>

  <Tab title="apk">
    Für Alpine Linux. Die folgenden Befehle konfigurieren den `stable`-Kanal:

    ```sh theme={null}
    wget -O /etc/apk/keys/claude-code.rsa.pub \
      https://downloads.claude.ai/keys/claude-code.rsa.pub
    echo "https://downloads.claude.ai/claude-code/apk/stable" >> /etc/apk/repositories
    apk add claude-code
    ```

    Um zum `latest`-Kanal zu wechseln, entfernen Sie die `stable`-Repository-Zeile und fügen Sie das `latest`-Repository hinzu:

    ```sh theme={null}
    sed -i '\|downloads.claude.ai/claude-code/apk/stable|d' /etc/apk/repositories
    echo "https://downloads.claude.ai/claude-code/apk/latest" >> /etc/apk/repositories
    ```

    Überprüfen Sie den heruntergeladenen Schlüssel mit `sha256sum /etc/apk/keys/claude-code.rsa.pub`, was `395759c1f7449ef4cdef305a42e820f3c766d6090d142634ebdb049f113168b6` melden sollte.

    Um später zu aktualisieren, führen Sie `apk update && apk upgrade claude-code` aus.
  </Tab>
</Tabs>

<h3 id="install-with-npm">
  Mit npm installieren
</h3>

Sie können Claude Code auch als globales npm-Paket installieren. Ab v2.1.198 erfordert das npm-Paket [Node.js 22 oder später](https://nodejs.org/en/download). Bei einer älteren Node.js-Version gibt npm während der Installation eine `EBADENGINE`-Warnung aus, anstatt fehlzuschlagen; die Installation wird abgeschlossen und `claude` läuft weiterhin, da das Paket eine native Binärdatei herunterlädt, die Ihr Node.js zur Laufzeit nicht verwendet.

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

Das npm-Paket installiert die gleiche native Binärdatei wie das eigenständige Installationsprogramm. npm zieht die Binärdatei durch eine plattformspezifische optionale Abhängigkeit wie `@anthropic-ai/claude-code-darwin-arm64` ein, und ein Postinstall-Schritt verknüpft sie. Die installierte `claude`-Binärdatei ruft selbst nicht Node auf.

Unterstützte npm-Installationsplattformen sind `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` und `win32-arm64`. Ihr Paketmanager muss optionale Abhängigkeiten zulassen. Siehe [Fehlerbehebung](/docs/de/troubleshoot-install#native-binary-not-found-after-npm-install), wenn die Binärdatei nach der Installation fehlt.

Um eine npm-Installation zu aktualisieren, führen Sie `npm install -g @anthropic-ai/claude-code@latest` aus. Vermeiden Sie `npm update -g`, das den Semver-Bereich aus der ursprünglichen Installation respektiert und Sie möglicherweise nicht zur neuesten Version führt.

<Warning>
  Verwenden Sie NICHT `sudo npm install -g`, da dies zu Berechtigungsproblemen und Sicherheitsrisiken führen kann. Wenn Sie auf Berechtigungsfehler stoßen, siehe [Fehlerbehebung bei Berechtigungsfehlern](/docs/de/troubleshoot-install#permission-errors-during-installation).
</Warning>

<h3 id="binary-integrity-and-code-signing">
  Binärintegrität und Code-Signierung
</h3>

Jede Veröffentlichung veröffentlicht eine `manifest.json`, die SHA256-Checksummen für jede Plattform-Binärdatei enthält. Das Manifest ist mit einem Anthropic GPG-Schlüssel signiert, daher überprüft die Überprüfung der Signatur auf dem Manifest transitiv jede Binärdatei, die es auflistet.

<h4 id="verify-the-manifest-signature">
  Manifest-Signatur überprüfen
</h4>

Die Schritte 1-3 erfordern eine POSIX-Shell mit `gpg` und `curl`. Unter Windows führen Sie sie in Git Bash oder WSL aus. Schritt 4 enthält eine PowerShell-Option.

<Steps>
  <Step title="Laden Sie den öffentlichen Schlüssel herunter und importieren Sie ihn">
    Der Release-Signaturschlüssel wird unter einer festen URL veröffentlicht.

    ```bash theme={null}
    curl -fsSL https://downloads.claude.ai/keys/claude-code.asc | gpg --import
    ```

    Zeigen Sie den Fingerabdruck des importierten Schlüssels an.

    ```bash theme={null}
    gpg --fingerprint security@anthropic.com
    ```

    Bestätigen Sie, dass die Ausgabe diesen Fingerabdruck enthält:

    ```text theme={null}
    31DD DE24 DDFA B679 F42D  7BD2 BAA9 29FF 1A7E CACE
    ```
  </Step>

  <Step title="Laden Sie das Manifest und die Signatur herunter">
    Setzen Sie `VERSION` auf die Veröffentlichung, die Sie überprüfen möchten.

    ```bash theme={null}
    REPO=https://downloads.claude.ai/claude-code-releases
    VERSION=2.1.89
    curl -fsSLO "$REPO/$VERSION/manifest.json"
    curl -fsSLO "$REPO/$VERSION/manifest.json.sig"
    ```
  </Step>

  <Step title="Überprüfen Sie die Signatur">
    Überprüfen Sie die abgelöste Signatur gegen das Manifest.

    ```bash theme={null}
    gpg --verify manifest.json.sig manifest.json
    ```

    Ein gültiges Ergebnis meldet `Good signature from "Anthropic Claude Code Release Signing <security@anthropic.com>"`.

    `gpg` druckt auch `WARNING: This key is not certified with a trusted signature!` für jeden neu importierten Schlüssel. Dies ist zu erwarten. Die Zeile `Good signature` bestätigt, dass die kryptografische Überprüfung bestanden wurde. Der Fingerabdruckvergleich in Schritt 1 bestätigt, dass der Schlüssel selbst authentisch ist.
  </Step>

  <Step title="Überprüfen Sie die Binärdatei gegen das Manifest">
    Vergleichen Sie die SHA256-Prüfsumme der Binärdatei mit dem Wert, der unter `platforms.<platform>.checksum` in `manifest.json` aufgelistet ist. Die folgenden Befehle gehen davon aus, dass sich eine `claude`-Binärdatei im aktuellen Verzeichnis befindet. Um stattdessen eine installierte native Binärdatei zu überprüfen, führen Sie den Befehl gegen `~/.local/share/claude/versions/VERSION` aus und ersetzen Sie VERSION durch die Veröffentlichung, die Sie in Schritt 2 festgelegt haben.

    <Tabs>
      <Tab title="Linux">
        ```bash theme={null}
        sha256sum claude
        ```
      </Tab>

      <Tab title="macOS">
        ```bash theme={null}
        shasum -a 256 claude
        ```
      </Tab>

      <Tab title="Windows PowerShell">
        ```powershell theme={null}
        (Get-FileHash claude.exe -Algorithm SHA256).Hash.ToLower()
        ```
      </Tab>
    </Tabs>
  </Step>
</Steps>

<Note>
  Manifest-Signaturen sind für Veröffentlichungen ab `2.1.89` verfügbar. Frühere Veröffentlichungen veröffentlichen Checksummen in `manifest.json` ohne abgelöste Signatur.
</Note>

<h4 id="platform-code-signatures">
  Plattform-Code-Signaturen
</h4>

Zusätzlich zum signierten Manifest tragen einzelne Binärdateien plattformspezifische Code-Signaturen, wo unterstützt.

* **macOS**: signiert von „Anthropic PBC" und beglaubigt von Apple. Überprüfen Sie mit `codesign --verify --verbose ./claude`.
* **Windows**: signiert von „Anthropic, PBC". Überprüfen Sie mit `Get-AuthenticodeSignature .\claude.exe`.
* **Linux**: Binärdateien sind nicht einzeln code-signiert. Wenn Sie direkt aus dem `claude-code-releases`-Bucket herunterladen oder das native Installationsprogramm verwenden, überprüfen Sie die Integrität mit der Manifest-Signatur oben. Wenn Sie mit [apt, dnf oder apk](#install-with-linux-package-managers) installieren, überprüft Ihr Paketmanager Signaturen automatisch mit dem Repository-Signaturschlüssel.

<h2 id="uninstall-claude-code">
  Claude Code deinstallieren
</h2>

Um Claude Code zu entfernen, folgen Sie den Anweisungen für Ihre Installationsmethode. Wenn `claude` danach immer noch ausgeführt wird, haben Sie wahrscheinlich eine zweite Installation oder einen verbleibenden Shell-Alias von einem älteren Installer. Siehe [Nach konfliktierenden Installationen prüfen](/docs/de/troubleshoot-install#check-for-conflicting-installations), um diese zu finden und zu entfernen.

<h3 id="native-installation">
  Native Installation
</h3>

Entfernen Sie die Claude Code-Binärdatei und Versionsdateien:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    rm -f ~/.local/bin/claude
    rm -rf ~/.local/share/claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force
    Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force
    ```
  </Tab>
</Tabs>

<h3 id="homebrew-installation">
  Homebrew-Installation
</h3>

Entfernen Sie das Homebrew-Cask, das Sie installiert haben. Wenn Sie das stabile Cask installiert haben:

```bash theme={null}
brew uninstall --cask claude-code
```

Wenn Sie das neueste Cask installiert haben:

```bash theme={null}
brew uninstall --cask claude-code@latest
```

<h3 id="winget-installation">
  WinGet-Installation
</h3>

Entfernen Sie das WinGet-Paket:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="apt-/-dnf-/-apk">
  apt / dnf / apk
</h3>

Entfernen Sie das Paket und die Repository-Konfiguration:

<Tabs>
  <Tab title="apt">
    ```bash theme={null}
    sudo apt remove claude-code
    sudo rm /etc/apt/sources.list.d/claude-code.list /etc/apt/keyrings/claude-code.asc
    ```
  </Tab>

  <Tab title="dnf">
    ```bash theme={null}
    sudo dnf remove claude-code
    sudo rm /etc/yum.repos.d/claude-code.repo
    ```
  </Tab>

  <Tab title="apk">
    ```sh theme={null}
    apk del claude-code
    sed -i '\|downloads.claude.ai/claude-code/apk|d' /etc/apk/repositories
    rm /etc/apk/keys/claude-code.rsa.pub
    ```
  </Tab>
</Tabs>

<h3 id="npm">
  npm
</h3>

Entfernen Sie das globale npm-Paket:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

<h3 id="remove-configuration-files">
  Konfigurationsdateien entfernen
</h3>

<Warning>
  Das Entfernen von Konfigurationsdateien löscht alle Ihre Einstellungen, zulässigen Tools, MCP-Serverkonfigurationen und Sitzungsverlauf.
</Warning>

Die VS Code-Erweiterung, das JetBrains-Plugin und die Desktop-App schreiben auch in `~/.claude/`. Wenn eines davon noch installiert ist, wird das Verzeichnis beim nächsten Ausführen neu erstellt. Um Claude Code vollständig zu entfernen, deinstallieren Sie die [VS Code-Erweiterung](/docs/de/vs-code#uninstall-the-extension), das JetBrains-Plugin und die Desktop-App, bevor Sie diese Dateien löschen.

So entfernen Sie Claude Code-Einstellungen und zwischengespeicherte Daten:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    # Entfernen Sie Benutzereinstellungen und Status
    rm -rf ~/.claude
    rm ~/.claude.json

    # Entfernen Sie projektspezifische Einstellungen (führen Sie dies aus Ihrem Projektverzeichnis aus)
    rm -rf .claude
    rm -f .mcp.json
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    # Entfernen Sie Benutzereinstellungen und Status
    Remove-Item -Path "$env:USERPROFILE\.claude" -Recurse -Force
    Remove-Item -Path "$env:USERPROFILE\.claude.json" -Force

    # Entfernen Sie projektspezifische Einstellungen (führen Sie dies aus Ihrem Projektverzeichnis aus)
    Remove-Item -Path ".claude" -Recurse -Force
    Remove-Item -Path ".mcp.json" -Force
    ```
  </Tab>
</Tabs>
