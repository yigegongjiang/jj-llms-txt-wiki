> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Installationsfehler und Anmeldungsprobleme beheben

> Beheben Sie Fehler wie „Befehl nicht gefunden", PATH, Berechtigungen, Netzwerk und Authentifizierungsfehler bei der Installation oder Anmeldung bei Claude Code.

Wenn die Installation fehlschlägt oder Sie sich nicht anmelden können, finden Sie Ihren Fehler unten. Für Laufzeitprobleme nach der Installation von Claude Code siehe [Fehlerbehebung](/docs/de/troubleshooting). Für Konfigurationsprobleme wie nicht angewendete Einstellungen oder nicht ausgelöste Hooks siehe [Konfiguration debuggen](/docs/de/debug-your-config).

<h2 id="find-your-error">
  Finden Sie Ihren Fehler
</h2>

Ordnen Sie die Fehlermeldung oder das Symptom, das Sie sehen, einer Lösung zu:

| Was Sie sehen                                                                                                       | Lösung                                                                                                                                          |
| :------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `command not found: claude` oder `'claude' is not recognized`                                                       | [Beheben Sie Ihren PATH](#command-not-found-claude-after-installation)                                                                          |
| `syntax error near unexpected token '<'`                                                                            | [Installationsskript gibt HTML zurück](#install-script-returns-html-instead-of-a-shell-script)                                                  |
| `curl: (22) The requested URL returned error: 403`                                                                  | [Installationsskript hat 403 zurückgegeben](#install-script-returns-html-instead-of-a-shell-script)                                             |
| `curl: (23)` oder `curl: (56) Failure writing output to destination`                                                | [Überprüfen Sie die Konnektivität oder verwenden Sie ein alternatives Installationsprogramm](#curl-56-failure-writing-output-to-destination)    |
| `Killed` während der Installation unter Linux oder `Installation was killed before it could finish (exit code 137)` | [Geben Sie Speicher frei oder fügen Sie Swap-Speicher hinzu](#install-killed-on-low-memory-linux-servers)                                       |
| `TLS connect error` oder `SSL/TLS secure channel`                                                                   | [Aktualisieren Sie CA-Zertifikate](#tls-or-ssl-connection-errors)                                                                               |
| `Failed to fetch version` oder kann den Download-Server nicht erreichen                                             | [Überprüfen Sie Netzwerk- und Proxy-Einstellungen](#check-network-connectivity)                                                                 |
| `irm is not recognized` oder `&& is not valid`                                                                      | [Verwenden Sie den richtigen Befehl für Ihre Shell](#wrong-install-command-on-windows)                                                          |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                                                  | [Aktualisieren Sie Homebrew](#homebrew-cask-unavailable-or-outdated)                                                                            |
| `'bash' is not recognized as the name of a cmdlet`                                                                  | [Verwenden Sie den Windows-Installationsbefehl](#wrong-install-command-on-windows)                                                              |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                                    | [Verwenden Sie den Windows-Installationsbefehl](#wrong-install-command-on-windows)                                                              |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`                                   | [Installieren Sie eine Shell](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                   |
| `Claude Code does not support 32-bit Windows`                                                                       | [Öffnen Sie Windows PowerShell, nicht den x86-Eintrag](#claude-code-does-not-support-32-bit-windows)                                            |
| `The process cannot access the file ... because it is being used by another process`                                | [Leeren Sie den Downloads-Ordner und versuchen Sie es erneut](#the-process-cannot-access-the-file-during-windows-install)                       |
| `Error loading shared library`                                                                                      | [Falsche Binärvariante für Ihr System](#linux-musl-or-glibc-binary-mismatch)                                                                    |
| `Illegal instruction`                                                                                               | [Architektur- oder CPU-Befehlssatz-Nichtübereinstimmung](#illegal-instruction)                                                                  |
| `cannot execute binary file: Exec format error` in WSL                                                              | [WSL1 native-binary Regression](#exec-format-error-on-wsl1)                                                                                     |
| PowerShell-Installationsprogramm wird abgeschlossen, aber `claude` wird nicht gefunden oder zeigt eine alte Version | [Fügen Sie das Installationsverzeichnis zu Ihrem PATH hinzu](#verify-your-path), dann öffnen Sie ein neues Terminal                             |
| `dyld: cannot load`, `dyld: Symbol not found` oder `Abort trap` unter macOS                                         | [Binärinkompatibilität](#dyld-cannot-load-on-macos)                                                                                             |
| `Invoke-Expression: Missing argument in parameter list`                                                             | [Installationsskript gibt HTML zurück](#install-script-returns-html-instead-of-a-shell-script)                                                  |
| `App unavailable in region`                                                                                         | Claude Code ist in Ihrem Land nicht verfügbar. Siehe [unterstützte Länder](https://www.anthropic.com/supported-countries).                      |
| `unable to get local issuer certificate`                                                                            | [Konfigurieren Sie Unternehmens-CA-Zertifikate](#tls-or-ssl-connection-errors)                                                                  |
| `OAuth error` oder `403 Forbidden`                                                                                  | [Beheben Sie die Authentifizierung](#login-and-authentication)                                                                                  |
| `Could not load the default credentials` oder `Could not load credentials from any providers`                       | [Amazon Bedrock-, Google Cloud Agent Platform- oder Microsoft Foundry-Anmeldedaten](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` oder `CredentialUnavailableError`                                    | [Amazon Bedrock-, Google Cloud Agent Platform- oder Microsoft Foundry-Anmeldedaten](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`, `529 Overloaded`, `429` oder andere 4xx und 5xx Fehler, die oben nicht aufgeführt sind            | Siehe die [Fehlerreferenz](/docs/de/errors)                                                                                                          |

Wenn Ihr Problem nicht aufgeführt ist, führen Sie die Diagnoseprüfungen unten durch, um die Ursache einzugrenzen.

<Tip>
  Wenn Sie das Terminal lieber ganz vermeiden möchten, können Sie mit der [Claude Code Desktop-App](/docs/de/desktop-quickstart) Claude Code über eine grafische Benutzeroberfläche installieren und verwenden. Laden Sie sie für [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) oder [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) herunter und beginnen Sie zu programmieren, ohne dass Sie eine Befehlszeileneinrichtung benötigen. Unter Linux installieren Sie die App mit apt, indem Sie die [Linux-Installationsanweisungen](/docs/de/desktop-linux) befolgen.
</Tip>

<h2 id="run-diagnostic-checks">
  Führen Sie Diagnoseprüfungen durch
</h2>

<h3 id="check-network-connectivity">
  Überprüfen Sie die Netzwerkkonnektivität
</h3>

Das Installationsprogramm lädt von `downloads.claude.ai` herunter. Überprüfen Sie, ob Sie es erreichen können:

```bash theme={null}
curl -sI https://downloads.claude.ai/claude-code-releases/latest
```

In PowerShell führen Sie stattdessen `curl.exe -sI` aus. PowerShell leitet `curl` zu `Invoke-WebRequest` weiter, das die Flags `-sI` ablehnt.

Eine `HTTP/2 200` Zeile bedeutet, dass Sie den Server erreicht haben. Wenn Sie keine Ausgabe, `Could not resolve host` oder ein Verbindungs-Timeout sehen, blockiert Ihr Netzwerk die Verbindung. Häufige Ursachen:

* Unternehmens-Firewalls oder Proxys, die `downloads.claude.ai` blockieren
* Regionale Netzwerkbeschränkungen: Versuchen Sie ein VPN oder ein alternatives Netzwerk
* TLS/SSL-Probleme: Aktualisieren Sie die CA-Zertifikate Ihres Systems, oder überprüfen Sie, ob `HTTPS_PROXY` konfiguriert ist

Wenn Sie sich hinter einem Unternehmens-Proxy befinden, setzen Sie `HTTPS_PROXY` und `HTTP_PROXY` auf die Adresse Ihres Proxys, bevor Sie installieren. Fragen Sie Ihr IT-Team nach der Proxy-URL, wenn Sie diese nicht kennen, oder überprüfen Sie die Proxy-Einstellungen Ihres Browsers.

Dieses Beispiel setzt beide Proxy-Variablen und führt dann das Installationsprogramm über Ihren Proxy aus:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  Überprüfen Sie Ihren PATH
</h3>

Wenn die Installation erfolgreich war, aber Sie einen `command not found` oder `not recognized` Fehler beim Ausführen von `claude` erhalten, befindet sich das Installationsverzeichnis nicht in Ihrem PATH. Ihre Shell sucht nach Programmen in Verzeichnissen, die in PATH aufgeführt sind, und das Installationsprogramm platziert `claude` unter `~/.local/bin/claude` auf macOS/Linux oder `%USERPROFILE%\.local\bin\claude.exe` unter Windows.

<Note>
  Die [VS Code-Erweiterung](/docs/de/vs-code) platziert `claude` nicht an diesem Ort. Sie bündelt eine private Kopie der CLI im Erweiterungsverzeichnis für ihr eigenes Chat-Panel und fügt sie nicht zu PATH hinzu. Wenn Sie nur die Erweiterung installiert haben, existiert `~/.local/bin/claude` nicht. Führen Sie die [eigenständige Installation](/docs/de/setup) aus, um `claude` von einem Terminal aus zu verwenden, und fahren Sie dann unten fort.
</Note>

Überprüfen Sie, ob sich das Installationsverzeichnis in Ihrem PATH befindet, indem Sie Ihre PATH-Einträge auflisten und nach `local/bin` filtern:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    Wenn dies `/Users/you/.local/bin` oder `/home/you/.local/bin` ausgibt, befindet sich das Verzeichnis in Ihrem PATH und Sie können zu [Überprüfen Sie auf konfliktfreie Installationen](#check-for-conflicting-installations) springen. Wenn es keine Ausgabe gibt, fügen Sie es zu Ihrer Shell-Konfiguration hinzu.

    Für Zsh, das Standard auf macOS:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    Für Bash, das Standard auf den meisten Linux-Distributionen:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    Alternativ können Sie Ihr Terminal schließen und erneut öffnen.

    Für andere Shells wie fish oder Nushell fügen Sie `~/.local/bin` zu Ihrem PATH mit der eigenen Konfigurationssyntax Ihrer Shell hinzu und starten Sie dann Ihr Terminal neu.

    Überprüfen Sie, ob die Behebung funktioniert hat:

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    Wenn es keine Ausgabe gibt, fügen Sie das Installationsverzeichnis zu Ihrem Benutzer-PATH hinzu:

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    Starten Sie Ihr Terminal neu, damit die Änderung wirksam wird.

    Überprüfen Sie, ob die Behebung funktioniert hat:

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    Wenn es keine Ausgabe gibt, öffnen Sie Systemeinstellungen, gehen Sie zu Umgebungsvariablen und fügen Sie `%USERPROFILE%\.local\bin` zu Ihrer Benutzer-PATH-Variable hinzu. Starten Sie Ihr Terminal neu.

    Überprüfen Sie, ob die Behebung funktioniert hat:

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  Überprüfen Sie auf konfliktfreie Installationen
</h3>

Mehrere Claude Code-Installationen können zu Versionskonflikten oder unerwartetem Verhalten führen. Überprüfen Sie, was installiert ist:

<Tabs>
  <Tab title="macOS/Linux">
    Listet alle `claude` Binärdateien auf, die in Ihrem PATH gefunden werden:

    ```bash theme={null}
    which -a claude
    ```

    Wenn dies nichts ausgibt, befindet sich noch kein `claude` in Ihrem PATH. Gehen Sie zurück zu [Überprüfen Sie Ihren PATH](#verify-your-path).

    Überprüfen Sie die drei Orte, von denen eine `claude` Binärdatei stammen kann. `~/.local/bin/claude` ist das native Installationsprogramm, `~/.claude/local/` ist eine ältere lokale npm-Installation, die von älteren Versionen von Claude Code erstellt wurde, und die npm-globale Liste zeigt eine `-g` Installation:

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    Eine native Installation zeigt einen Symlink in `~/.local/share/claude/versions/`. Ein Skript oder ein Symlink, den Sie selbst an diesem Pfad erstellt haben, ist ein benutzerdefinierter Launcher, den [Auto-Update an Ort und Stelle lässt](/docs/de/setup#auto-updates).

    Wenn einer der `ls` Befehle `No such file or directory` ausgibt, ist das kein Fehler. Das bedeutet, dass an diesem Ort nichts installiert ist, also fahren Sie mit der nächsten Prüfung fort.

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    Listet alle `claude` Binärdateien auf, die in Ihrem PATH gefunden werden:

    ```powershell theme={null}
    where.exe claude
    ```

    Überprüfen Sie, ob das native Installationsprogramm eine Binärdatei platziert hat:

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

Wenn Sie mehrere Installationen finden, behalten Sie nur eine. Die native Installation unter `~/.local/bin/claude` auf macOS/Linux oder `%USERPROFILE%\.local\bin\claude.exe` unter Windows wird empfohlen. Entfernen Sie die zusätzlichen:

Deinstallieren Sie eine globale npm-Installation:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

Entfernen Sie die ältere lokale npm-Installation:

```bash theme={null}
rm -rf ~/.claude/local
```

Unter Windows verwenden Sie PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
```

Entfernen Sie eine Homebrew-Installation auf macOS. Wenn Sie das `claude-code@latest` Cask installiert haben, ersetzen Sie diesen Namen:

```bash theme={null}
brew uninstall --cask claude-code
```

Entfernen Sie eine WinGet-Installation unter Windows:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  Überprüfen Sie Verzeichnisberechtigungen
</h3>

Das Installationsprogramm benötigt Schreibzugriff auf `~/.local/bin/` und `~/.claude/` auf macOS und Linux. Unter Windows befindet sich der Installationsort unter `%USERPROFILE%`, das standardmäßig von Ihrem Benutzer beschreibbar ist, daher gilt dieser Abschnitt dort selten.

Überprüfen Sie, ob die Verzeichnisse beschreibbar sind:

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

Wenn eines der Verzeichnisse nicht beschreibbar ist, erstellen Sie das Installationsverzeichnis und setzen Sie Ihren Benutzer als Eigentümer:

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  Überprüfen Sie, ob die Binärdatei funktioniert
</h3>

Wenn `claude --version` eine Version ausgibt, aber `claude` beim Start abstürzt oder hängt, führen Sie diese Prüfungen durch, um die Ursache einzugrenzen. Wenn `claude --version` sagt, dass der Befehl nicht gefunden wurde, gehen Sie zuerst zu [Überprüfen Sie Ihren PATH](#verify-your-path); die folgenden Befehle gehen davon aus, dass `claude` in Ihrem PATH ist.

Bestätigen Sie, dass die Binärdatei existiert und ausführbar ist:

```bash theme={null}
ls -la "$(command -v claude)"
```

Unter Windows verwenden Sie PowerShell:

```powershell theme={null}
Get-Command claude | Select-Object Source
```

Überprüfen Sie unter Linux auf fehlende gemeinsame Bibliotheken. Wenn `ldd` fehlende Bibliotheken anzeigt, müssen Sie möglicherweise Systempakete installieren. Auf Alpine Linux und anderen musl-basierten Distributionen siehe [Alpine Linux-Setup](/docs/de/setup#alpine-linux-and-musl-based-distributions).

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

Bestätigen Sie, dass die Binärdatei ausgeführt werden kann:

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  Häufige Installationsprobleme
</h2>

Dies sind die am häufigsten auftretenden Installationsprobleme und deren Lösungen.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  Installationsskript gibt HTML statt eines Shell-Skripts zurück
</h3>

Beim Ausführen des Installationsbefehls können Sie einen dieser Fehler sehen:

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

In PowerShell erscheint das gleiche Problem als:

```text theme={null}
Invoke-Expression: Missing argument in parameter list.
```

Je nachdem, wie die Anfrage weitergeleitet wurde, können Sie stattdessen auch einen 403-Fehler ohne HTML-Text sehen:

```text theme={null}
curl: (22) The requested URL returned error: 403
```

Dies alles bedeutet, dass die Installations-URL eine HTML-Seite oder einen Fehlerstatus statt des Installationsskripts zurückgegeben hat. Wenn die HTML-Seite „App unavailable in region" sagt, ist Claude Code in Ihrem Land nicht verfügbar. Siehe [unterstützte Länder](https://www.anthropic.com/supported-countries).

Ein bloßer 403 ohne Text hat oft die gleiche Ursache, kann aber auch von einem Unternehmens-Proxy oder einer Firewall stammen, die den Download blockiert. Wenn Sie sich in einem unterstützten Land befinden und immer noch den 403-Fehler sehen, arbeiten Sie sich durch [Überprüfen Sie die Netzwerkverbindung](#check-network-connectivity) durch, bevor Sie die alternativen Installationsprogramme unten versuchen, da diese die gleichen Hosts erreichen.

Andernfalls kann dies aufgrund von Netzwerkproblemen, regionalen Routing-Problemen oder einer vorübergehenden Dienstunterbrechung geschehen.

**Lösungen:**

1. **Verwenden Sie eine alternative Installationsmethode**:

   Auf macOS installieren Sie über Homebrew:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Unter Windows installieren Sie über WinGet:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

2. **Versuchen Sie es nach einigen Minuten erneut**: Das Problem ist oft vorübergehend. Warten Sie und versuchen Sie den ursprünglichen Befehl erneut.

<h3 id="command-not-found-claude-after-installation">
  `command not found: claude` nach der Installation
</h3>

Die Installation ist abgeschlossen, aber `claude` funktioniert nicht. Die genaue Fehlermeldung variiert je nach Plattform:

| Plattform   | Fehlermeldung                                                          |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

Dies bedeutet, dass sich das Installationsverzeichnis nicht im Suchpfad Ihrer Shell befindet. Siehe [Überprüfen Sie Ihren PATH](#verify-your-path) für die Behebung auf jeder Plattform.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

Der Befehl `curl ... | bash` lädt das Skript herunter und leitet es an Bash zur Ausführung weiter. Dieser Fehler und der verwandte `curl: (23) Failure writing output to destination` bedeuten, dass Bash das vollständige Skript nicht erhalten hat. Exit-Code 56 zeigt an, dass der Download selbst unterbrochen wurde, und Exit-Code 23 zeigt an, dass curl nicht schreiben konnte, was es erhielt, in die Pipe, normalerweise weil Bash vorzeitig beendet wurde.

**Lösungen:**

1. **Überprüfen Sie die Netzwerkstabilität**: Claude Code-Binärdateien werden unter `downloads.claude.ai` gehostet. Testen Sie, ob Sie es erreichen können:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```
   Eine `HTTP/2 200` Zeile bedeutet, dass Sie den Server erreicht haben und der ursprüngliche Fehler wahrscheinlich vorübergehend war; versuchen Sie den Installationsbefehl erneut. Wenn Sie `Could not resolve host` oder ein Verbindungs-Timeout sehen, blockiert Ihr Netzwerk den Download.

2. **Versuchen Sie eine alternative Installationsmethode**:

   Auf macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Unter Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="homebrew-cask-unavailable-or-outdated">
  Homebrew Cask nicht verfügbar oder veraltet
</h3>

Homebrew meldet `Error: Cask 'claude-code' is unavailable: No Cask with this name exists`, wenn Ihre lokale Kopie des Homebrew Cask-Index älter ist als die Veröffentlichung des Cask. Aktualisieren Sie den Index und versuchen Sie es erneut:

```bash theme={null}
brew update
brew install --cask claude-code
```

Wenn Homebrew eine ältere Claude Code-Version installiert als erwartet, ist normalerweise der gleiche veraltete Index die Ursache. Der `claude-code` Cask verfolgt den stabilen Kanal und liegt normalerweise etwa eine Woche hinter der neuesten Version; für die neueste Version führen Sie stattdessen `brew install --cask claude-code@latest` aus. Siehe [Konfigurieren Sie den Release-Kanal](/docs/de/setup#configure-release-channel) für den Unterschied zwischen den beiden Casks.

<h3 id="tls-or-ssl-connection-errors">
  TLS- oder SSL-Verbindungsfehler
</h3>

Fehler wie `curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed` oder PowerShells `Could not establish trust relationship for the SSL/TLS secure channel` deuten auf TLS-Handshake-Fehler hin.

**Lösungen:**

1. **Aktualisieren Sie Ihre System-CA-Zertifikate**:

   Auf Ubuntu/Debian:

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   Auf macOS verwendet das System-curl den Keychain-Vertrauensspeicher; das Aktualisieren von macOS selbst aktualisiert die Root-Zertifikate.

2. **Aktivieren Sie unter Windows TLS 1.2** in PowerShell, bevor Sie das Installationsprogramm ausführen:
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **Überprüfen Sie auf Proxy- oder Firewall-Interferenz**: Unternehmens-Proxys, die TLS-Inspektion durchführen, können diese Fehler verursachen, einschließlich `unable to get local issuer certificate` und `SELF_SIGNED_CERT_IN_CHAIN`. Für den Installationsschritt zeigen Sie curl auf Ihr Unternehmens-CA-Bundle mit `--cacert`:
   ```bash theme={null}
   curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
   ```
   Für Claude Code selbst nach der Installation setzen Sie `NODE_EXTRA_CA_CERTS`, damit API-Anfragen dem gleichen Bundle vertrauen:
   ```bash theme={null}
   export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
   ```
   Fragen Sie Ihr IT-Team nach der Zertifikatsdatei, wenn Sie diese nicht haben. Sie können auch auf einer direkten Verbindung versuchen, um zu bestätigen, dass der Proxy die Ursache ist.

4. **Unter Windows, wenn Ihr Netzwerk Sperrprüfungen blockiert**. Die Fehler `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` und `CRYPT_E_REVOCATION_OFFLINE (0x80092013)` bedeuten, dass curl den Server erreicht hat, aber Ihr Netzwerk die Zertifikatssperrprüfung blockiert, was hinter Unternehmens-Firewalls häufig vorkommt. Das Hinzufügen von curls `--ssl-revoke-best-effort` Flag behebt dies nicht: Das Flag gilt nur für das Herunterladen von `install.cmd` selbst, und die eigenen Downloads des Skripts werden ohne es ausgeführt, daher schlägt die Installation mit dem gleichen Fehler fehl. Verwenden Sie stattdessen eine Installationsmethode, die die blockierte Suche toleriert. Öffnen Sie PowerShell und führen Sie das PowerShell-Installationsprogramm aus, das über .NET herunterlädt und nicht fehlschlägt, wenn der Sperrserver nicht erreichbar ist:
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   Sie können auch mit `winget install Anthropic.ClaudeCode` installieren, was curl ganz vermeidet.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

Das Installationsprogramm konnte den Download-Server nicht erreichen. Dies bedeutet normalerweise, dass `downloads.claude.ai` in Ihrem Netzwerk blockiert ist.

**Lösungen:**

1. **Testen Sie die Konnektivität direkt**:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```

2. **Wenn Sie sich hinter einem Proxy befinden**, setzen Sie `HTTPS_PROXY`, damit das Installationsprogramm es durchleiten kann. Siehe [Proxy-Konfiguration](/docs/de/network-config#proxy-configuration) für Details.
   ```bash theme={null}
   export HTTPS_PROXY=http://proxy.example.com:8080
   curl -fsSL https://claude.ai/install.sh | bash
   ```

3. **Wenn Sie sich in einem eingeschränkten Netzwerk befinden**, versuchen Sie ein anderes Netzwerk oder VPN, oder verwenden Sie eine alternative Installationsmethode:

   Auf macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Unter Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="wrong-install-command-on-windows">
  Falscher Installationsbefehl unter Windows
</h3>

Wenn Sie `'irm' is not recognized`, `The token '&&' is not valid`, `A parameter cannot be found that matches parameter name 'fsSL'` oder `'bash' is not recognized as the name of a cmdlet` sehen, haben Sie den Installationsbefehl für eine andere Shell oder ein anderes Betriebssystem kopiert.

* **`irm` nicht erkannt**: Sie befinden sich in CMD, nicht in PowerShell. Sie haben zwei Optionen:

  Öffnen Sie PowerShell, indem Sie im Startmenü nach „PowerShell" suchen, und führen Sie dann den ursprünglichen Installationsbefehl aus:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  Oder bleiben Sie in CMD und verwenden Sie stattdessen das CMD-Installationsprogramm:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` nicht gültig**: Sie befinden sich in PowerShell, haben aber den CMD-Installationsbefehl ausgeführt. Verwenden Sie das PowerShell-Installationsprogramm:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**: Sie haben das macOS/Linux `curl -fsSL ... | bash` Installationsprogramm in Windows PowerShell ausgeführt, wo `curl` ein Alias für `Invoke-WebRequest` ist und die `-fsSL` Flags ablehnt. Verwenden Sie stattdessen das PowerShell-Installationsprogramm:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` nicht erkannt**: Sie haben das macOS/Linux-Installationsprogramm unter Windows ausgeführt. Verwenden Sie stattdessen das PowerShell-Installationsprogramm:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  `The process cannot access the file` während der Windows-Installation
</h3>

Wenn das PowerShell-Installationsprogramm mit `Failed to download binary: The process cannot access the file ... because it is being used by another process` fehlschlägt, konnte das Installationsprogramm nicht in `%USERPROFILE%\.claude\downloads` schreiben. Dies bedeutet normalerweise, dass ein vorheriger Installationsversuch noch läuft, oder Antivirus-Software scannt eine teilweise heruntergeladene Binärdatei in diesem Ordner.

Schließen Sie alle anderen PowerShell-Fenster, die das Installationsprogramm ausführen, und warten Sie, bis Antivirus-Scans die Datei freigeben. Löschen Sie dann den Downloads-Ordner und führen Sie das Installationsprogramm erneut aus:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  Installation auf Linux-Servern mit wenig Speicher beendet
</h3>

Eine `Killed` Meldung während der Installation bedeutet normalerweise, dass der Linux Out-of-Memory (OOM) Killer den `claude install` Schritt beendet hat, weil dem System der Speicher ausgegangen ist. Dies ist häufig auf kleinen VPS und Cloud-Instanzen der Fall. Das Installationsskript meldet die Ursache und beendet sich mit Exit-Code 137:

```text theme={null}
Setting up Claude Code...
bash: line 142: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Vor v2.1.200 beendete sich das Skript nur mit der bloßen `Killed` Zeile der Shell und ohne Erklärung.

Die Installation benötigt ungefähr 512 MB freien Speicher, und das Ausführen von Claude Code benötigt mehr. Siehe die [Systemanforderungen](/docs/de/setup#system-requirements).

**Lösungen:**

1. **Fügen Sie Swap-Speicher hinzu**, wenn Ihr Server über begrenzte RAM verfügt. Swap verwendet Festplattenspeicher als Überlauf-Speicher, sodass die Installation auch bei wenig physischem RAM abgeschlossen werden kann.

   Erstellen Sie eine 2-GB-Swap-Datei und aktivieren Sie sie:

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   Versuchen Sie dann die Installation erneut:

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Schließen Sie andere Prozesse**, um Speicher vor der Installation freizugeben.

3. **Verwenden Sie eine größere Instanz**, wenn möglich. Claude Code benötigt mindestens 4 GB RAM.

<h3 id="install-hangs-in-docker">
  Installation hängt in Docker
</h3>

Beim Installieren von Claude Code in einem Docker-Container kann die Installation als Root in `/` zu Hängern führen.

**Lösungen:**

1. **Setzen Sie ein Arbeitsverzeichnis**, bevor Sie das Installationsprogramm ausführen. Wenn es von `/` aus ausgeführt wird, scannt das Installationsprogramm das gesamte Dateisystem, was zu übermäßiger Speichernutzung führt. Das Setzen von `WORKDIR` begrenzt den Scan auf ein kleines Verzeichnis:
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Erhöhen Sie die Docker-Speicherlimits**, wenn Sie Docker Desktop verwenden:
   ```bash theme={null}
   docker build --memory=4g .
   ```

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop überschreibt den `claude` Befehl unter Windows
</h3>

Wenn Sie eine ältere Version von Claude Desktop installiert haben, kann sie eine `Claude.exe` im `WindowsApps` Verzeichnis registrieren, die PATH-Priorität über Claude Code CLI hat. Das Ausführen von `claude` öffnet die Desktop-App statt der CLI.

Aktualisieren Sie Claude Desktop auf die neueste Version, um dieses Problem zu beheben.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Claude Code unter Windows benötigt entweder Git für Windows (für Bash) oder PowerShell
</h3>

Git für Windows ist optional. Claude Code verwendet das [PowerShell-Tool](/docs/de/tools-reference#powershell-tool), wenn Git Bash nicht vorhanden ist, daher bedeutet dieser Fehler, dass keine Shell gefunden wurde.

**Wenn PowerShell in Ihrem PATH fehlt**, ist sein Standardort `C:\Windows\System32\WindowsPowerShell\v1.0\`. Fügen Sie dieses Verzeichnis zu Ihrem `PATH` hinzu, oder installieren Sie [PowerShell 7](https://aka.ms/powershell), das `pwsh` bereitstellt.

**Um Git für Windows stattdessen zu installieren**, laden Sie es von [git-scm.com/downloads/win](https://git-scm.com/downloads/win) herunter. Wählen Sie während der Einrichtung „Add to PATH" aus. Starten Sie Ihr Terminal nach der Installation neu. Die Installation ermöglicht das Bash-Tool, das beim Arbeiten mit Bash-basierten Skripten und Tools nützlich ist.

**Wenn Git bereits installiert ist**, aber Claude Code kann es nicht finden, setzen Sie den Pfad in Ihrer [settings.json Datei](/docs/de/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

Wenn Ihr Git an einem anderen Ort installiert ist, finden Sie den Pfad, indem Sie `where.exe git` in PowerShell ausführen, und verwenden Sie den `bin\bash.exe` Pfad aus diesem Verzeichnis.

**Wenn der Pfad korrekt ist und die Datei existiert**, aber Claude Code meldet immer noch, dass sie nicht gefunden wird, kann Endpoint-Security-Software wie AppLocker, Group Policy-Softwarebeschränkungsrichtlinien oder EDR-Agenten interferieren. In Versionen vor v2.1.116 hat Claude Code einen untergeordneten Prozess (`cmd.exe`) erzeugt, um den Pfad zu überprüfen, was diese Richtlinien blockieren können — ein häufiges Zeichen ist, dass `cmd.exe /c dir "C:\Program Files\Git\bin\bash.exe"` funktioniert, wenn Sie es direkt in PowerShell ausführen, aber stillschweigend fehlschlägt, wenn es von `claude.exe` gestartet wird.

Claude Code v2.1.116 und später überprüfen das Dateisystem direkt, daher aktualisieren Sie zuerst. Wenn der Fehler auf einer aktuellen Version weiterhin besteht, bitten Sie Ihr IT-Team, `claude.exe` und die Prozesse, die es erzeugt, einschließlich `cmd.exe` und `bash.exe`, in Ihrer Endpoint-Protection-Richtlinie auf die Whitelist zu setzen.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code unterstützt 32-Bit Windows nicht
</h3>

Windows enthält zwei PowerShell-Einträge im Startmenü: `Windows PowerShell` und `Windows PowerShell (x86)`. Der x86-Eintrag wird als 32-Bit-Prozess ausgeführt und löst diesen Fehler auch auf einer 64-Bit-Maschine aus. Um zu überprüfen, welcher Fall vorliegt, führen Sie dies im gleichen Fenster aus, das den Fehler verursacht hat:

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

Wenn dies `True` ausgibt, ist Ihr Betriebssystem in Ordnung. Schließen Sie das Fenster, öffnen Sie `Windows PowerShell` ohne das x86-Suffix und führen Sie den Installationsbefehl erneut aus.

Wenn dies `False` ausgibt, befinden Sie sich auf einer 32-Bit-Edition von Windows. Claude Code benötigt ein 64-Bit-Betriebssystem. Siehe die [Systemanforderungen](/docs/de/setup#system-requirements).

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Linux musl oder glibc Binärvarianten-Nichtübereinstimmung
</h3>

Wenn Sie nach der Installation Fehler über fehlende gemeinsame Bibliotheken wie `libstdc++.so.6` oder `libgcc_s.so.1` sehen, hat das Installationsprogramm möglicherweise die falsche Binärvariante für Ihr System heruntergeladen.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

Dies kann auf glibc-basierten Systemen geschehen, auf denen musl-Cross-Compilation-Pakete installiert sind, was das Installationsprogramm dazu veranlasst, das System fälschlicherweise als musl zu erkennen.

**Lösungen:**

1. **Überprüfen Sie, welche libc Ihr System verwendet**:
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   Die Ausgabe, die `GNU libc` oder `GLIBC` erwähnt, bedeutet glibc. Die Ausgabe, die `musl` erwähnt, bedeutet musl.

2. **Wenn Sie auf glibc sind, aber die musl-Binärdatei erhalten haben**, entfernen Sie die Installation und installieren Sie erneut. Sie können die richtige Binärdatei auch manuell mit dem Manifest unter `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json` herunterladen. Melden Sie ein [GitHub-Problem](https://github.com/anthropics/claude-code/issues) mit der Ausgabe von `ldd --version` und `ls /lib/libc.musl*`.

3. **Wenn Sie sich tatsächlich auf musl befinden**, wie Alpine Linux, installieren Sie die erforderlichen Pakete:
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

Wenn das Ausführen von `claude` oder dem Installationsprogramm `Illegal instruction` ausgibt, verwendet die native Binärdatei CPU-Befehle, die Ihr Prozessor nicht unterstützt. Es gibt zwei unterschiedliche Ursachen.

**Architektur-Nichtübereinstimmung.** Das Installationsprogramm hat die falsche Binärdatei heruntergeladen, zum Beispiel x86 auf einem ARM-Server. Überprüfen Sie mit `uname -m` auf macOS oder Linux oder `$env:PROCESSOR_ARCHITECTURE` in PowerShell. Wenn das Ergebnis nicht mit der Binärdatei übereinstimmt, die Sie erhalten haben, [melden Sie ein GitHub-Problem](https://github.com/anthropics/claude-code/issues) mit der Ausgabe.

**Fehlender AVX-Befehlssatz.** Wenn Ihre Architektur korrekt ist, aber Sie immer noch `Illegal instruction` sehen, fehlt Ihrer CPU wahrscheinlich AVX oder ein anderer Befehl, den die Binärdatei benötigt. Dies betrifft ungefähr Intel- und AMD-Prozessoren vor 2013 und virtuelle Maschinen, bei denen der Hypervisor AVX nicht an den Gast durchleitet.

Auf einem VPS oder einer VM führen Sie `grep -m1 -ow avx /proc/cpuinfo` aus; ein leeres Ergebnis bedeutet, dass AVX für den Gast nicht verfügbar ist.

Es gibt keine native-binary Umgehung; verfolgen Sie [Problem #50384](https://github.com/anthropics/claude-code/issues/50384) für den Status und geben Sie Ihr CPU-Modell von `grep -m1 "model name" /proc/cpuinfo` unter Linux oder `sysctl -n machdep.cpu.brand_string` auf macOS an, wenn Sie es melden.

Alternative Installationsmethoden laden die gleiche native Binärdatei herunter und werden keine der beiden Ursachen beheben.

<h3 id="dyld-cannot-load-on-macos">
  `dyld: cannot load` auf macOS
</h3>

Wenn Sie während der Installation `dyld: cannot load`, `dyld: Symbol not found` oder `Abort trap: 6` sehen, ist die Binärdatei mit Ihrer macOS-Version oder Hardware nicht kompatibel.

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

Ein `Symbol not found` Fehler, der auf `libicucore` verweist, zeigt auch an, dass Ihre macOS-Version älter ist als die Binärdatei unterstützt:

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

**Lösungen:**

1. **Überprüfen Sie Ihre macOS-Version**: Claude Code benötigt macOS 13.0 oder später. Öffnen Sie das Apple-Menü und wählen Sie „Über diesen Mac", um Ihre Version zu überprüfen.

2. **Aktualisieren Sie macOS**, wenn Sie eine ältere Version verwenden. Die Binärdatei verwendet Befehle und Systembibliotheken, die ältere macOS-Versionen nicht unterstützen. Alternative Installationsmethoden wie Homebrew laden die gleiche Binärdatei herunter und werden diesen Fehler nicht beheben.

<h3 id="exec-format-error-on-wsl1">
  `Exec format error` auf WSL1
</h3>

Wenn das Ausführen von `claude` in WSL `cannot execute binary file: Exec format error` ausgibt, befinden Sie sich auf WSL1 und treffen auf eine bekannte native-binary Regression, die in [Problem #38788](https://github.com/anthropics/claude-code/issues/38788) verfolgt wird. Die Programm-Header der Binärdatei haben sich auf eine Weise geändert, die der WSL1-Loader nicht verarbeiten kann.

Die sauberste Behebung ist die Konvertierung Ihrer Distribution zu WSL2 von PowerShell:

```powershell theme={null}
wsl --set-version <DistroName> 2
```

Wenn Sie auf WSL1 bleiben müssen, rufen Sie die Binärdatei über den dynamischen Linker auf. Fügen Sie diese Funktion zu `~/.bashrc` in WSL hinzu, ersetzen Sie den Pfad, wenn sich Ihr Home-Verzeichnis unterscheidet:

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

Führen Sie dann `source ~/.bashrc` aus und versuchen Sie `claude` erneut.

<h3 id="npm-install-errors-in-wsl">
  npm-Installationsfehler in WSL
</h3>

Diese Probleme gelten, wenn Sie Claude Code mit `npm install -g` in WSL installiert haben. Wenn Sie das [native Installationsprogramm](/docs/de/setup) verwendet haben, überspringen Sie diesen Abschnitt.

**Betriebssystem- oder Plattformerkennung Probleme.** Wenn npm während der Installation einen Plattform-Nichtübereinstimmung meldet, verwendet WSL wahrscheinlich das Windows `npm`. Führen Sie zuerst `npm config set os linux` aus, dann installieren Sie mit `npm install -g @anthropic-ai/claude-code --force`. Verwenden Sie nicht `sudo`.

**`exec: node: not found` beim Ausführen von `claude`.** Ihre WSL-Umgebung verwendet wahrscheinlich die Windows-Installation von Node.js. Bestätigen Sie mit `which npm` und `which node`: Pfade, die mit `/mnt/c/` beginnen, sind Windows-Binärdateien, während Linux-Pfade mit `/usr/` beginnen. Um dies zu beheben, installieren Sie Node über den Paketmanager Ihrer Linux-Distribution oder über [`nvm`](https://github.com/nvm-sh/nvm).

**nvm Versionskonflikte.** Wenn Sie nvm sowohl in WSL als auch in Windows installiert haben, kann das Wechseln von Node-Versionen in WSL fehlschlagen, da WSL standardmäßig den Windows-PATH importiert und das Windows-nvm Priorität hat. Die häufigste Ursache ist, dass nvm nicht in Ihrer Shell geladen wird. Fügen Sie den nvm-Loader zu `~/.bashrc` oder `~/.zshrc` hinzu:

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

Oder laden Sie es in Ihrer aktuellen Sitzung:

```bash theme={null}
source ~/.nvm/nvm.sh
```

Wenn nvm geladen ist, aber Windows-Pfade immer noch Priorität haben, stellen Sie Ihren Linux-Node-Pfad explizit voran:

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  Vermeiden Sie das Deaktivieren des Windows-PATH-Imports über `appendWindowsPath = false`, da dies die Möglichkeit bricht, Windows-Ausführbare aus WSL aufzurufen. Vermeiden Sie auch das Deinstallieren von Node.js von Windows, wenn Sie es für Windows-Entwicklung verwenden.
</Warning>

<h3 id="permission-errors-during-installation">
  Berechtigungsfehler während der Installation
</h3>

Wenn das native Installationsprogramm mit Berechtigungsfehlern fehlschlägt, ist das Zielverzeichnis möglicherweise nicht beschreibbar. Siehe [Überprüfen Sie Verzeichnisberechtigungen](#check-directory-permissions).

Wenn Sie zuvor mit npm installiert haben und npm-spezifische Berechtigungsfehler erhalten, wechseln Sie zum nativen Installationsprogramm:

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  Native Binärdatei nicht gefunden nach npm-Installation
</h3>

Das `@anthropic-ai/claude-code` npm-Paket zieht die native Binärdatei durch eine pro-Plattform optionale Abhängigkeit wie `@anthropic-ai/claude-code-darwin-arm64` ein. Wenn das Ausführen von `claude` nach der Installation `Could not find native binary package "@anthropic-ai/claude-code-<platform>"` ausgibt, überprüfen Sie die folgenden Ursachen:

* **Optionale Abhängigkeiten sind deaktiviert.** Entfernen Sie `--omit=optional` aus Ihrem npm-Installationsbefehl, `--no-optional` von pnpm oder `--ignore-optional` von yarn, und überprüfen Sie, dass `.npmrc` nicht `optional=false` setzt. Dann installieren Sie erneut. Die native Binärdatei wird nur als optionale Abhängigkeit bereitgestellt, daher gibt es keinen JavaScript-Fallback, wenn sie übersprungen wird.
* **Nicht unterstützte Plattform.** Vorkompilierte Binärdateien werden für `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` und `win32-arm64` veröffentlicht. Claude Code liefert keine Binärdatei für andere Plattformen; siehe die [Systemanforderungen](/docs/de/setup#system-requirements). Auf FreeBSD meldet das Installationsprogramm die Plattform als nicht unterstützt. Vor v2.1.205 behandelte es FreeBSD als Linux und lud eine Binärdatei herunter, die nicht ausgeführt werden konnte.
* **Unternehmens-npm-Spiegel fehlen die Plattform-Pakete.** Stellen Sie sicher, dass Ihr Registry alle acht `@anthropic-ai/claude-code-*` Plattform-Pakete zusätzlich zum Meta-Paket spiegelt.

Die Installation mit `--ignore-scripts` löst diesen Fehler nicht aus. Der Postinstall-Schritt, der die Binärdatei verknüpft, wird übersprungen, daher fällt Claude Code auf einen Wrapper zurück, der die Plattform-Binärdatei bei jedem Start findet und startet. Dies funktioniert, aber startet langsamer; installieren Sie mit aktivierten Skripten für direkte Ausführung erneut.

<h2 id="login-and-authentication">
  Anmeldung und Authentifizierung
</h2>

Diese Abschnitte behandeln Anmeldungsfehler, OAuth-Fehler und Token-Probleme.

<h3 id="reset-your-login">
  Setzen Sie Ihre Anmeldung zurück
</h3>

Wenn die Anmeldung fehlschlägt und die Ursache nicht offensichtlich ist, löst eine saubere Neuer-Authentifizierung die meisten Fälle:

1. Führen Sie `/logout` aus, um sich vollständig abzumelden
2. Schließen Sie Claude Code
3. Starten Sie mit `claude` neu und schließen Sie den Authentifizierungsprozess ab

Wenn der Browser während der Anmeldung nicht automatisch geöffnet wird, drücken Sie `c`, um die OAuth-URL in Ihre Zwischenablage zu kopieren, und fügen Sie sie dann manuell in einen Browser ein. Dies funktioniert auch, wenn die URL in einem schmalen oder SSH-Terminal über mehrere Zeilen verläuft und nicht direkt angeklickt werden kann.

<h3 id="oauth-error-invalid-code">
  OAuth-Fehler: Ungültiger Code
</h3>

Wenn Sie `OAuth error: Invalid code. Please make sure the full code was copied` sehen, ist der Anmeldecode abgelaufen oder wurde beim Kopieren und Einfügen gekürzt.

**Lösungen:**

* Drücken Sie Enter, um zu wiederholen und die Anmeldung schnell nach dem Öffnen des Browsers abzuschließen
* Geben Sie `c` ein, um die vollständige URL zu kopieren, wenn der Browser nicht automatisch geöffnet wird
* Wenn Sie eine Remote-/SSH-Sitzung verwenden, kann der Browser auf der falschen Maschine geöffnet werden. Kopieren Sie die im Terminal angezeigte URL und öffnen Sie sie stattdessen in Ihrem lokalen Browser.

<h3 id="403-forbidden-after-login">
  403 Forbidden nach der Anmeldung
</h3>

Wenn Sie `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}` nach der Anmeldung sehen:

* **Claude Pro/Max-Benutzer**: Überprüfen Sie, dass Ihr Abonnement unter [claude.ai/settings](https://claude.ai/settings) aktiv ist
* **Anthropic Console-Benutzer**: Bestätigen Sie, dass Ihr Konto die Rolle „Claude Code" oder „Developer" hat. Admins weisen dies in der Anthropic Console unter Einstellungen → Mitglieder zu.
* **Hinter einem Proxy**: Unternehmens-Proxys können API-Anfragen beeinträchtigen. Siehe [Netzwerkkonfiguration](/docs/de/network-config) für Proxy-Einrichtung.

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  Diese Organisation wurde mit einem aktiven Abonnement deaktiviert
</h3>

Wenn Sie `API Error: 400 ... "This organization has been disabled"` sehen, obwohl Sie ein aktives Claude-Abonnement haben, überschreibt eine `ANTHROPIC_API_KEY` Umgebungsvariable Ihr Abonnement. Dies geschieht häufig, wenn ein alter API-Schlüssel von einem früheren Arbeitgeber oder Projekt noch in Ihrem Shell-Profil gesetzt ist.

Wenn `ANTHROPIC_API_KEY` vorhanden ist und Sie es genehmigt haben, verwendet Claude Code diesen Schlüssel statt der OAuth-Anmeldedaten Ihres Abonnements. Im nicht-interaktiven Modus mit dem `-p` Flag wird der Schlüssel immer verwendet, wenn er vorhanden ist. Siehe [Authentifizierungs-Priorität](/docs/de/authentication#authentication-precedence) für die vollständige Auflösungsreihenfolge.

Um stattdessen Ihr Abonnement zu verwenden, heben Sie die Umgebungsvariable auf und entfernen Sie sie aus Ihrem Shell-Profil:

```bash theme={null}
unset ANTHROPIC_API_KEY
claude
```

Überprüfen Sie `~/.zshrc`, `~/.bashrc` oder `~/.profile` auf `export ANTHROPIC_API_KEY=...` Zeilen und entfernen Sie sie, um die Änderung dauerhaft zu machen. Unter Windows überprüfen Sie Ihr PowerShell-Profil unter `$PROFILE` und Ihre Benutzer-Umgebungsvariablen auf `ANTHROPIC_API_KEY`. Führen Sie `/status` in Claude Code aus, um zu bestätigen, welche Authentifizierungsmethode aktiv ist.

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  OAuth-Anmeldung schlägt in WSL2, SSH oder Containern fehl
</h3>

Wenn Claude Code in WSL2, auf einem Remote-Rechner über SSH oder in einem Container ausgeführt wird, öffnet sich der Browser normalerweise auf einem anderen Host und seine Umleitung kann Claude Code's lokalen Callback-Server nicht erreichen. Nachdem Sie sich anmelden, zeigt der Browser einen Anmeldecode statt einer automatischen Umleitung an. Fügen Sie diesen Code in das Terminal bei der Aufforderung `Paste code here if prompted` ein, um die Anmeldung abzuschließen.

Wenn der Browser überhaupt nicht aus WSL2 geöffnet wird, setzen Sie die `BROWSER` Umgebungsvariable auf Ihren Windows-Browser-Pfad:

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

Alternativ drücken Sie `c` bei der interaktiven Anmeldungsaufforderung, um die OAuth-URL zu kopieren, oder kopieren Sie die URL, die `claude auth login` ausgibt, und öffnen Sie sie in einem Browser auf Ihrem lokalen Rechner.

Wenn das Einfügen des Codes in die interaktive Aufforderung nichts bewirkt, erreicht die Paste-Bindung Ihres Terminals wahrscheinlich nicht das Eingabefeld. Versuchen Sie die alternative Paste-Verknüpfung Ihres Terminals, oft Rechtsklick oder Shift+Insert in Windows Terminal, oder verwenden Sie stattdessen `claude auth login`, das den eingefügten Code aus der Standardeingabe liest:

```bash theme={null}
claude auth login
```

Dieser Fallback gilt auch auf nativem Windows oder jedem Terminal, bei dem das Einfügen in die interaktive Aufforderung fehlschlägt.

<h3 id="not-logged-in-or-token-expired">
  Nicht angemeldet oder Token abgelaufen
</h3>

Wenn Claude Code Sie nach einer Sitzung erneut zur Anmeldung auffordert, ist Ihr OAuth-Token möglicherweise abgelaufen.

Führen Sie `/login` aus, um sich erneut zu authentifizieren. Wenn dies häufig geschieht, überprüfen Sie, dass Ihre Systemuhr genau ist, da die Token-Validierung von korrekten Zeitstempeln abhängt.

Auf macOS kann die Anmeldung auch fehlschlagen, wenn der Keychain gesperrt ist oder sein Passwort nicht mit Ihrem Kontopasswort synchronisiert ist, was Claude Code daran hindert, Anmeldedaten zu speichern. Führen Sie `claude doctor` aus, um den Keychain-Zugriff zu überprüfen. Um den Keychain manuell zu entsperren, führen Sie `security unlock-keychain ~/Library/Keychains/login.keychain-db` aus. Wenn das Entsperren nicht hilft, öffnen Sie Keychain Access, wählen Sie den `login` Keychain und wählen Sie Bearbeiten > Passwort für Keychain „login" ändern, um es mit Ihrem Kontopasswort zu resynchronisieren.

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Bedrock-, Agent Platform- oder Foundry-Anmeldedaten werden nicht geladen
</h3>

Wenn Sie Claude Code für die Verwendung eines Cloud-Anbieters konfiguriert haben und `Could not load credentials from any providers` auf Amazon Bedrock, `Could not load the default credentials` auf Google Cloud's Agent Platform oder `ChainedTokenCredential authentication failed` auf Microsoft Foundry sehen, ist Ihre Cloud-Anbieter-CLI wahrscheinlich nicht in der aktuellen Shell authentifiziert.

Für Amazon Bedrock bestätigen Sie, dass Ihre AWS-Anmeldedaten gültig sind:

```bash theme={null}
aws sts get-caller-identity
```

Für Google Cloud's Agent Platform bestätigen Sie, dass `ANTHROPIC_VERTEX_PROJECT_ID` und `CLOUD_ML_REGION` in Ihrer Shell gesetzt sind, dann setzen Sie Anwendungs-Standard-Anmeldedaten:

```bash theme={null}
gcloud auth application-default login
```

Für Microsoft Foundry bestätigen Sie, dass `ANTHROPIC_FOUNDRY_API_KEY` gesetzt ist, oder melden Sie sich mit der Azure CLI an, damit die Standard-Anmeldedaten-Kette Ihr Konto finden kann:

```bash theme={null}
az login
```

Wenn Anmeldedaten in Ihrem Terminal funktionieren, aber nicht in der VS Code oder JetBrains-Erweiterung, hat der IDE-Prozess wahrscheinlich Ihre Shell-Umgebung nicht geerbt. Setzen Sie die Anbieter-Umgebungsvariablen in den IDE-eigenen Einstellungen, oder starten Sie die IDE von einem Terminal aus, in dem sie bereits exportiert sind.

Siehe [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry) für die vollständige Anbieter-Einrichtung.

<h2 id="still-stuck">
  Immer noch festgefahren
</h2>

Wenn keine der obigen Lösungen Ihr Problem behebt:

1. Überprüfen Sie das [GitHub-Repository](https://github.com/anthropics/claude-code/issues) auf bekannte Probleme, oder öffnen Sie ein neues mit Ihrem Betriebssystem, dem Installationsbefehl, den Sie ausgeführt haben, und der vollständigen Fehlerausgabe
2. Wenn `claude --version` funktioniert, aber etwas anderes ist falsch, führen Sie `claude doctor` aus, um einen automatisierten Diagnosebericht zu erhalten
3. Wenn Sie eine Sitzung starten können, verwenden Sie `/feedback` in Claude Code, um das Problem zu melden
