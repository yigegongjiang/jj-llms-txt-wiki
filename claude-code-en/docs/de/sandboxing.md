> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurieren Sie das Sandboxed-Bash-Tool

> Erfahren Sie, wie das Sandboxed-Bash-Tool von Claude Code Dateisystem- und Netzwerkisolation für sicherere und autonomere Agent-Ausführung bietet.

Die Bash-Sandbox ermöglicht es Claude, die meisten Shell-Befehle auszuführen, ohne um Genehmigung zu fragen. Anstatt jeden Befehl zu genehmigen, definieren Sie, welche Dateien und Netzwerk-Domains Befehle berühren können, und das Betriebssystem erzwingt diese Grenze für jeden Bash-, PowerShell- oder Monitor-Befehl und seine Kindprozesse.

<Note>
  Um andere Isolationsansätze wie Dev-Container, benutzerdefinierte Container und virtuelle Maschinen zu vergleichen, siehe [Sandbox-Umgebungen](/docs/de/sandbox-environments). Um Genehmigungseingaben für Tools außer Bash zu reduzieren, siehe [Genehmigungsmodi](/docs/de/permission-modes).
</Note>

<h2 id="get-started">
  Erste Schritte
</h2>

Die Sandbox ist in Claude Code integriert und läuft auf macOS, Linux und WSL2. Native Windows wird nicht unterstützt. Führen Sie Claude Code unter Windows in einer WSL2-Distribution aus.

Auf macOS gibt es nichts zu installieren: Sandboxing verwendet das integrierte Seatbelt-Framework. Auf Linux und WSL2 basiert die Sandbox auf zwei Paketen, die in [Linux und WSL2 einrichten](#set-up-linux-and-wsl2) behandelt werden. Selbst wenn Sie diese noch nicht installiert haben, können Sie mit `/sandbox` beginnen, da sein Panel anzeigt, ob etwas fehlt.

<Steps>
  <Step title="Führen Sie /sandbox aus">
    Starten Sie eine Claude Code-Sitzung und führen Sie den Befehl `/sandbox` aus:

    ```text theme={null}
    /sandbox
    ```

    Dies öffnet das Sandbox-Panel mit drei Registerkarten sowie einer Registerkarte „Dependencies" unter Linux, wenn der optionale Seccomp-Filter fehlt:

    * **Mode**: Wählen Sie, wie Sandbox-Befehle genehmigt werden, behandelt im nächsten Schritt
    * **Overrides**: Wählen Sie, ob Befehle, die unter der Sandbox fehlschlagen, auf unsandboxed ausweichen können. Dies ist die Einstellung [`allowUnsandboxedCommands`](/docs/de/settings-reference#sandbox-allowunsandboxedcommands)
    * **Config**: Zeigen Sie die aufgelösten Sandbox-Einstellungen an

    Wenn das Panel nur eine Registerkarte „Dependencies" anzeigt, fehlt ein erforderliches Paket. Installieren Sie es wie in [Linux und WSL2 einrichten](#set-up-linux-and-wsl2) beschrieben, starten Sie Claude Code neu und führen Sie `/sandbox` erneut aus.
  </Step>

  <Step title="Wählen Sie einen Modus">
    Wählen Sie auf der Registerkarte „Mode" Auto-Allow oder reguläre Genehmigungen. Auto-Allow führt Sandbox-Befehle ohne Eingabeaufforderung aus, und reguläre Genehmigungen behalten die regulären Genehmigungseingaben bei, auch wenn Befehle in der Sandbox ausgeführt werden. Siehe [Sandbox-Modi](#sandbox-modes) für die Befehle, die im Auto-Allow-Modus immer noch eingeben.
  </Step>

  <Step title="Führen Sie einen Bash-Befehl aus">
    Bitten Sie Claude, einen Befehl auszuführen, z. B. einen Build oder eine Test-Suite. Standardmäßig können Befehle in der Sandbox in das Arbeitsverzeichnis, das Sitzungs-Temp-Verzeichnis und alle [Verzeichnisse schreiben, die Sie hinzugefügt haben](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) mit `--add-dir`, `/add-dir` oder `permissions.additionalDirectories`.

    Wenn ein Befehl zum ersten Mal eine neue Netzwerk-Domain benötigt, fordert Claude Code zur Genehmigung auf; im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) benennt Claude stattdessen die Hosts, die ein Befehl benötigt, [im Befehl selbst](#per-command-allowed-domains-in-auto-mode) für den Klassifizierer zur Überprüfung damit.

    Befehle, die nicht in der Sandbox ausgeführt werden können, fallen auf den regulären Genehmigungsfluss zurück. Claude Code betitelt ihre Genehmigungseingabe mit „Bash-Befehl (unsandboxed)" statt „Bash-Befehl", damit Sie erkennen können, welche Befehle außerhalb der Sandbox ausgeführt wurden. Um diese Grenzen zu erweitern oder zu verengen, siehe [Sandboxing konfigurieren](#configure-sandboxing).

    Wenn Sandbox-Befehle in einem Container mit `Operation not permitted` fehlschlagen, siehe den Bubblewrap-Eintrag unter [Fehlerbehebung](#troubleshooting).
  </Step>
</Steps>

Wenn Sie einen Modus im Panel auswählen, speichert Claude Code ihn in den lokalen Einstellungen Ihres Projekts unter `.claude/settings.local.json`, die für das aktuelle Projekt gelten. Claude Code fügt diese Datei zu Ihrer globalen Gitignore hinzu, wenn es dort eine Einstellung speichert. Um die Sandbox in allen Ihren Projekten zu aktivieren, setzen Sie [`sandbox.enabled`](/docs/de/settings-reference#sandbox-enabled) auf `true` in Ihren Benutzereinstellungen unter `~/.claude/settings.json`. Um Sandboxing für jeden Entwickler in einer Organisation zu erzwingen, verwenden Sie [verwaltete Einstellungen](#enforce-sandboxing-with-managed-settings).

Um die Sandbox für eine Sitzung zu ändern, ohne in eine Einstellungsdatei zu schreiben, starten Sie Claude Code mit [`--settings`](/docs/de/settings#change-a-setting-for-one-session). Beispielsweise startet dieser Befehl eine Sandbox-Sitzung, in der Claude einen blockierten Befehl nicht außerhalb der Sandbox erneut versuchen kann:

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

<Warning>
  Standardmäßig zeigt Claude Code eine Warnung an und führt Befehle ohne Sandboxing aus, wenn die Sandbox nicht gestartet werden kann, da Abhängigkeiten fehlen oder die Plattform nicht unterstützt wird. Um dies stattdessen zu einem Hard Failure zu machen, setzen Sie [`sandbox.failIfUnavailable`](/docs/de/settings-reference#sandbox-failifunavailable) auf `true`. Dies ist für verwaltete Bereitstellungen vorgesehen, die Sandboxing als Sicherheits-Gate erfordern.
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Linux und WSL2 einrichten
</h3>

Auf Linux und WSL2 basiert die Sandbox auf zwei Paketen:

* [`bubblewrap`](https://github.com/containers/bubblewrap): das unprivilegierte Sandboxing-Tool, das Dateisystem-Isolation erzwingt
* [`socat`](http://www.dest-unreach.org/socat/): das Relay, das verwendet wird, um Netzwerk-Datenverkehr durch den Sandbox-Proxy zu leiten

Installieren Sie diese mit dem Paketmanager Ihrer Distribution:

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

Wenn eine Abhängigkeit fehlt, listet die Registerkarte „Dependencies" in `/sandbox` auf, welche von `ripgrep`, `bubblewrap`, `socat` und dem Seccomp-Filter Ihre Plattform nicht hat. Wenn Sie die Registerkarte nach der Installation und dem Neustart von Claude Code nicht sehen, sind alle Abhängigkeiten vorhanden.

Ripgrep ist mit der nativen Claude Code-Binärdatei gebündelt. Der Seccomp-Filter ist optional und fügt Unix-Domain-Socket-Blockierung hinzu. Installieren Sie ihn mit `npm install -g @anthropic-ai/sandbox-runtime`, wenn er fehlt.

Wenn eine erforderliche Abhängigkeit fehlt, ist die Registerkarte „Dependencies" die einzige angezeigte Registerkarte, bis Sie sie installieren. Wenn nur der optionale Seccomp-Filter fehlt, wird die Registerkarte „Dependencies" neben den anderen Registerkarten angezeigt. Die Abhängigkeitsprüfung läuft beim Start, daher starten Sie Claude Code nach der Installation von Paketen neu, damit `/sandbox` diese erkennt.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 und später: Erlauben Sie bubblewrap, Benutzer-Namespaces zu erstellen">
    Auf Ubuntu 24.04 und später verhindert die Standard-AppArmor-Richtlinie, dass bubblewrap die Benutzer-Namespaces erstellt, die es für die Isolation benötigt.

    Um zu überprüfen, ob Ihre Umgebung diese Einschränkung erzwingt, auch in WSL2, führen Sie `sysctl kernel.apparmor_restrict_unprivileged_userns` aus. Wenn der Befehl `0` zurückgibt, überspringen Sie diesen Schritt. Wenn er einen `No such file or directory`-Fehler ausgibt, existiert der Schlüssel nicht und Sie können diesen Schritt überspringen. Wenn er `1` zurückgibt, fügen Sie ein AppArmor-Profil hinzu, das `bwrap` diese Fähigkeit gewährt:

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    Das Profil gilt nur für `bwrap` selbst, nicht für die Befehle, die es in der Sandbox ausführt. Laden Sie AppArmor neu, um es anzuwenden:

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="WSL2-Hinweise">
    Überprüfen Sie Ihre WSL-Version mit `wsl -l -v` aus PowerShell. Wenn Sie `Sandboxing requires WSL2` sehen, läuft Ihre Distribution auf WSL1. Aktualisieren Sie sie auf WSL2 oder führen Sie Claude Code ohne Sandboxing aus.

    Auf WSL2 übergibt WSL den Start einer Windows-Binärdatei wie `cmd.exe`, `powershell.exe` oder etwas unter `/mnt/c/` über einen Unix-Socket an den Windows-Host, daher folgt, ob ein Sandbox-Befehl einen starten kann, den [Unix-Socket-Einstellungen](/docs/de/settings-reference#sandbox-network-allowunixsockets) der Sandbox: Der optionale Seccomp-Filter muss installiert sein, um den Socket überhaupt zu blockieren. Um diese Starts zu erlauben, setzen Sie `allowAllUnixSockets`; um sie vollständig aus der Sandbox zu halten, fügen Sie den Befehl zu [`excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) hinzu.
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Sandbox-Modi
</h3>

Claude Code bietet zwei Sandbox-Modi. In beiden erzwingt die Sandbox die gleichen Dateisystem- und Netzwerk-Einschränkungen; der Unterschied liegt nur darin, ob Sandbox-Befehle automatisch genehmigt oder explizit genehmigt werden müssen.

<h4 id="auto-allow-mode">
  Auto-Allow-Modus
</h4>

Wenn ein Befehl in der Sandbox ausgeführt werden kann, führt Claude Code ihn in der Sandbox aus und genehmigt ihn automatisch, ohne Ihre Genehmigung zu fragen. Befehle, die nicht in der Sandbox ausgeführt werden können, z. B. solche, die Netzwerkzugriff auf nicht zulässige Hosts benötigen, fallen auf den regulären Genehmigungsfluss zurück, bei dem Claude Code Ihre [Genehmigungsregeln](/docs/de/permissions) überprüft und jeden Befehl, den diese Regeln nicht bereits zulassen, mit einer Eingabeaufforderung im Manual-Modus blockiert.

Selbst im Auto-Allow-Modus gelten folgende Punkte:

* Explizite [Deny-Regeln](/docs/de/permissions) werden immer respektiert
* `rm`- oder `rmdir`-Befehle, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielen, durchlaufen immer noch den regulären Genehmigungsfluss
* Inhaltsgebundene [Ask-Regeln](/docs/de/permissions) wie `Bash(git push *)` erzwingen immer noch eine Eingabeaufforderung, auch für Sandbox-Befehle
* Eine einfache `Bash` Ask-Regel oder die entsprechende `Bash(*)` Form wird für Befehle übersprungen, die in der Sandbox ausgeführt werden; sie gilt immer noch für Befehle, die auf den regulären Genehmigungsfluss zurückfallen. Im [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) wird die Regel nicht übersprungen: Sie fordert auch für Sandbox-Befehle auf, einschließlich schreibgeschützter. Vor v2.1.212 galt das Überspringen auch im Plan-Modus

<Info>
  Der Auto-Allow-Modus funktioniert unabhängig von Ihrer Genehmigungsmodus-Einstellung, mit drei Ausnahmen: [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode), ein Auto-Modus-Befehl, der [pro-Befehl zulässige Domains](#per-command-allowed-domains-in-auto-mode) trägt, und [serverseitige Klassifizierer-Überprüfung](/docs/de/permission-modes#how-the-classifier-evaluates-actions) von Sandbox-Befehlen im Auto-Modus. Selbst wenn Sie sich nicht im „Bearbeitungen akzeptieren"-Modus befinden, werden Sandbox-Bash-Befehle automatisch ausgeführt, wenn Auto-Allow aktiviert ist. Dies bedeutet, dass Bash-Befehle, die Dateien innerhalb der Sandbox-Grenzen ändern, ohne Eingabeaufforderung ausgeführt werden, auch im Manual-Modus, wo die Datei-Bearbeitungs-Tools eine Eingabeaufforderung erfordern würden.

  Im Plan-Modus erweitert Auto-Allow die Genehmigungen nicht; siehe [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) für die Verwaltung von Befehlen durch Claude Code während der Planung. Vor v2.1.212 führte Auto-Allow Sandbox-Befehle auch im Plan-Modus ohne Eingabeaufforderung aus.
</Info>

<h4 id="regular-permissions-mode">
  Regulärer Genehmigungsmodus
</h4>

Alle Bash-Befehle durchlaufen den regulären Genehmigungsfluss, auch wenn sie in der Sandbox ausgeführt werden. Dies bietet mehr Kontrolle, erfordert aber mehr Genehmigungen.

<h4 id="the-unsandboxed-retry-escape-hatch">
  Die Fluchtluke für unsandboxed-Wiederversuche
</h4>

Einige Befehle können überhaupt nicht in der Sandbox ausgeführt werden, z. B. Tools, die nicht kompatibel sind oder einen Host benötigen, den Sie nicht zulässig gemacht haben. Claude Code meldet Sandbox-Verstöße im Ergebnis des blockierten Befehls und nennt den Pfad oder Host, den die Sandbox verweigert hat, damit Claude sieht, was die Sandbox blockiert hat. Anstatt die Aufgabe fehlschlagen zu lassen oder Sie zu zwingen, Sandboxing auszuschalten, enthält Claude Code eine Fluchtluke: Claude analysiert den Verstoß und kann den Befehl mit dem Parameter `dangerouslyDisableSandbox` erneut versuchen.

Der erneut versuchte Befehl läuft außerhalb der Sandbox, daher durchläuft er den regulären Genehmigungsfluss. Im Manual-Modus erhalten Sie eine Bestätigungseingabe. Im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) bewertet der Klassifizierer den zugrunde liegenden Befehl. Während [`permissions.blockReadsOutsideWorkingDirectories`](/docs/de/settings-reference#permissions-blockreadsoutsideworkingdirectories) aktiviert ist, fordert ein Wiederversuch, der Genehmigung benötigt, um außerhalb der Sandbox ausgeführt zu werden, Sie stattdessen auf. Um bei jedem unsandboxed-Wiederversuch auch im Auto-Modus aufgefordert zu werden, fügen Sie eine [Ask-Regel](/docs/de/permissions#match-by-input-parameter) für `Bash(dangerouslyDisableSandbox:true)` hinzu.

Sie können diese Fluchtluke deaktivieren, indem Sie `"allowUnsandboxedCommands": false` in Ihren [Sandbox-Einstellungen](/docs/de/settings-reference#sandbox-settings) setzen. Wenn die Fluchtluke deaktiviert ist, ignoriert Claude Code den Parameter `dangerouslyDisableSandbox`, und jeder Befehl, den Claude ausführt, muss in der Sandbox ausgeführt werden, es sei denn, Sie haben ihn in `excludedCommands` aufgelistet. Die Registerkarte `/sandbox` **Overrides** zeigt diese Einstellung als **Strict sandbox mode** an.

Der Strict-Sandbox-Modus gilt für die Befehle, die Claude ausführt. Befehle, die Sie selbst an der [`!` Shell-Modus-Eingabeaufforderung](/docs/de/interactive-mode#shell-mode-with-prefix) eingeben, laufen außerhalb der Sandbox, es sei denn, die Sitzung ist eine dieser:

* **Eine [Hintergrund-Sitzung](/docs/de/agent-view)**: Der Strict-Sandbox-Modus deckt auch Shell-Modus-Befehle ab
* **Eine Linux-Sitzung mit [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/de/env-vars#variables) gesetzt**: Jeder Befehl wird in der Sandbox ausgeführt, einschließlich Shell-Modus-Befehle

Vor v2.1.260 sandboxte der Strict-Sandbox-Modus Shell-Modus-Befehle in jeder Sitzung.

<h4 id="temporary-directories">
  Temporäre Verzeichnisse
</h4>

Das Sitzungs-Temp-Verzeichnis ist standardmäßig in der Sandbox beschreibbar, zusammen mit dem Arbeitsverzeichnis. Sofern Sie nicht [Dateisystem-Isolation deaktivieren](#disable-filesystem-isolation), setzt Claude Code `$TMPDIR` auf dieses Verzeichnis für Sandbox-Befehle, sodass Tools, die temporäre Dateien schreiben, ohne zusätzliche Konfiguration funktionieren.

Unsandboxed-Befehle erben Ihr Shell-`$TMPDIR`, wenn es gesetzt ist, daher lösen Sandbox- und Unsandboxed-Befehle `$TMPDIR` in verschiedene Verzeichnisse auf, während Dateisystem-Isolation aktiviert ist. Wenn Ihre Shell `$TMPDIR` nicht gesetzt oder leer lässt, erhält ein Unsandboxed-Befehl, der auf `$TMPDIR` verweist, Ihre [`CLAUDE_CODE_TMPDIR`](/docs/de/env-vars)-Überschreibung oder das Temp-Verzeichnis des Betriebssystems, wenn Sie keine gesetzt haben oder die Überschreibung ein langer Pfad ist, damit die Variable nicht zu einer leeren Zeichenkette expandiert. Um temporäre Dateien zwischen den beiden zu übergeben, schreiben Sie sie stattdessen unter das Arbeitsverzeichnis.

<h2 id="configure-sandboxing">
  Sandboxing konfigurieren
</h2>

Passen Sie das Sandbox-Verhalten durch Ihre `settings.json`-Datei an. Siehe [Einstellungen](/docs/de/settings-reference#sandbox-settings) für die vollständige Konfigurationsreferenz.

Standardmäßig können Sandbox-Befehle in das aktuelle Arbeitsverzeichnis, das Sitzungs-Temp-Verzeichnis und alle [Verzeichnisse schreiben, die Sie hinzugefügt haben](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) mit `--add-dir`, `/add-dir` oder `permissions.additionalDirectories`. Wenn Subprozess-Befehle wie `kubectl`, `terraform` oder `npm` außerhalb dieser Verzeichnisse schreiben müssen, verwenden Sie `sandbox.filesystem.allowWrite`, um Zugriff auf spezifische Pfade zu gewähren:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

Diese Pfade werden auf OS-Ebene durchgesetzt, daher respektieren alle Befehle, die in der Sandbox ausgeführt werden, einschließlich ihrer Kindprozesse, diese. Dies ist der empfohlene Ansatz, wenn ein Tool Schreibzugriff auf einen bestimmten Ort benötigt, anstatt das Tool mit `excludedCommands` vollständig aus der Sandbox auszuschließen.

Wenn Sie das gleiche Dateisystem-Array in mehreren [Einstellungs-Scopes](/docs/de/settings#settings-precedence) definieren, führt Claude Code diese zusammen und kombiniert Pfade aus jedem Scope, anstatt das Array eines Scopes durch das eines anderen zu ersetzen.

Wenn Sie eine Quelle mit [`--setting-sources`](/docs/de/cli-reference) in der CLI oder [`settingSources`](/docs/de/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) im Agent SDK ausschließen, ignoriert Claude Code ihre `sandbox.filesystem`-Einträge, ihre `Edit`-Genehmigungsregeln und ihre `Read`-Ablehnungsregeln beim Aufbau der Sandbox-Konfiguration. Erfordert Claude Code v2.1.246 oder später.

Wenn Sie diese Dateisystem-Listen während einer Sitzung bearbeiten, [wendet Claude Code die Änderung auf die laufende Sitzung an](/docs/de/settings#when-edits-take-effect), daher wird der nächste Sandbox-Befehl unter den neuen Pfaden ausgeführt.

Pfad-Präfixe steuern, wie Pfade aufgelöst werden:

| Präfix                | Bedeutung                                                                                         | Beispiel                                                              |
| :-------------------- | :------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------- |
| `/`                   | Absoluter Pfad vom Dateisystem-Root                                                               | `/tmp/build` bleibt `/tmp/build`                                      |
| `~/`                  | Relativ zum Home-Verzeichnis                                                                      | `~/.kube` wird zu `$HOME/.kube`                                       |
| `./` oder kein Präfix | Relativ zum Projekt-Root für Projekt-Einstellungen oder zu `~/.claude` für Benutzer-Einstellungen | `./output` in `.claude/settings.json` wird zu `<project-root>/output` |

Diese Syntax unterscheidet sich von [Read- und Edit-Genehmigungsregeln](/docs/de/permissions#read-and-edit), die `//path` für absolut und `/path` für projekt-relativ verwenden. Sandbox-Dateisystem-Pfade verwenden Standard-Konventionen: `/tmp/build` ist absolut. Wie Claude Code einen nachgestellten Schrägstrich oder einen Platzhalter in diesen Pfaden behandelt, siehe [Sandbox-Pfad-Präfixe](/docs/de/settings-reference#sandbox-path-prefixes).

Sie können auch Schreib- oder Lesezugriff mit `sandbox.filesystem.denyWrite` und `sandbox.filesystem.denyRead` blockieren und spezifische Pfade innerhalb einer blockierten Region mit `sandbox.filesystem.allowRead` erneut zulassen. Wenn sich Lesevorgänge überlappen, gewinnt der spezifischere Pfad:

| Beispielregeln                                         | Ergebnis                                                                                                                                                                                                                            |
| :----------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` mit `"allowRead": ["~/projects"]` | `~/projects` ist lesbar und der Rest des Home-Verzeichnisses bleibt blockiert. Die engere Zulassung öffnet diesen Teil der blockierten Region erneut                                                                                |
| `"allowRead": ["~/"]` mit `"denyRead": ["~/.env"]`     | `~/.env` bleibt blockiert und der Rest des Home-Verzeichnisses ist lesbar. Eine genaue Blockierung gilt innerhalb einer breiteren Zulassung, daher kann eine breite Zulassung ein Geheimnis nicht stillschweigend erneut offenlegen |
| `"allowRead": ["~/"]` mit `"denyRead": ["~/**/.env"]`  | Jede `.env` unter dem Home-Verzeichnis bleibt blockiert und der Rest ist lesbar. Ein [Platzhalter-Deny](/docs/de/settings-reference#sandbox-path-prefixes) gilt innerhalb einer breiteren Zulassung genauso wie ein genauer Pfad         |

Das folgende Beispiel blockiert das Lesen aus dem gesamten Home-Verzeichnis und erlaubt gleichzeitig Lesevorgänge aus dem aktuellen Projekt. Platzieren Sie es in der Datei `.claude/settings.json` Ihres Projekts, da der relative Pfad `.` nur zum Projekt-Root aufgelöst wird, wenn die Konfiguration in Projekt-Einstellungen lebt:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Wenn Sie die gleiche Konfiguration in `~/.claude/settings.json` platzieren würden, würde `.` stattdessen zu `~/.claude` aufgelöst, und Projektdateien würden durch die `denyRead`-Regel blockiert bleiben.

Um Sandbox-Befehlen Lesezugriff auf Home-Verzeichnisse und eingebundene Volumes zu verweigern und gleichzeitig die Arbeitsverzeichnisse lesbar zu halten, setzen Sie stattdessen [`permissions.blockReadsOutsideWorkingDirectories`](/docs/de/settings-reference#permissions-blockreadsoutsideworkingdirectories).

<h3 id="disable-filesystem-isolation">
  Dateisystem-Isolation deaktivieren
</h3>

Setzen Sie `sandbox.filesystem.disabled` auf `true`, um die Dateisystem-Isolation zu überspringen und gleichzeitig die Netzwerk-Isolation beizubehalten. Das folgende Beispiel deaktiviert die Dateisystem-Isolation und behält eine Zulassungsliste von Netzwerk-Domains:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Die Sandbox hat zwei unabhängige Schichten: [Dateisystem-Isolation](#filesystem-isolation) steuert, welche Pfade Sandbox-Befehle lesen und schreiben können, und [Netzwerk-Isolation](#network-isolation) steuert, welche Domains sie erreichen können. Mit der Dateisystem-Schicht aus erhalten Sandbox-Befehle unbegrenzten Lese- und Schreibzugriff auf das Host-Dateisystem, während ihr Netzwerk-Ausgang auf Ihre zulässigen Domains beschränkt bleibt. Schalten Sie die Schicht aus, wenn Sie Sandboxing verwenden, um zu steuern, wo Befehle sich verbinden, anstatt was sie schreiben.

Die Einstellung ist standardmäßig deaktiviert und gilt auf den Plattformen, auf denen die Sandbox ausgeführt wird: macOS, Linux und WSL2. Erfordert Claude Code v2.1.216 oder später.

<Warning>
  Mit deaktivierter Dateisystem-Isolation und automatisch zulässigen Befehlen kann ein Sandbox-Befehl Dateien schreiben, die später Befehle ausführen oder lesen, wie Shell-Startdateien, ausführbare Dateien auf `$PATH` oder `~/.claude/settings.json`, und diese verwenden, um seinen eigenen Zugriff beim nächsten Durchlauf zu erweitern. Setzen Sie `filesystem.disabled` auf `true` nur für Workloads, denen Sie vertrauen, dass sie ihren eigenen Zugriff nicht eskalieren. Das Sperren von Netzwerk-Domains mit [`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy) verengt das Risiko, entfernt es aber nicht, da diese Sperre nur auf Befehle gilt, die in der Sandbox ausgeführt werden.
</Warning>

<h4 id="which-settings-can-disable-it">
  Welche Einstellungen können es deaktivieren
</h4>

Da das Ausschalten der Dateisystem-Isolation erweitert, was Sandbox-Befehle tun können, berücksichtigt Claude Code `filesystem.disabled` nur aus diesen Einstellungsquellen:

* Benutzer-Einstellungen, verwaltete Einstellungen und das `--settings` CLI-Flag können es setzen. Projekt-Einstellungen in `.claude/settings.json` und `.claude/settings.local.json` können nicht, daher kann ein ausgechecktes Projekt die Dateisystem-Isolation nicht ausschalten.
* Wenn verwaltete Einstellungen `sandbox.filesystem` überhaupt konfigurieren oder einen beliebigen `sandbox.credentials.files`-Eintrag mit `"mode": "deny"` auflisten, können nur verwaltete Einstellungen den Schlüssel setzen. Dies hält vom Administrator bereitgestellte Dateisystem-Einschränkungen in Kraft; um eine solche Bereitstellung zu lockern, setzen Sie `"disabled": true` in verwalteten Einstellungen.
* Wenn [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/de/env-vars) gesetzt ist, ignoriert Claude Code `filesystem.disabled` aus jeder Quelle, einschließlich verwalteter Einstellungen, und lässt die Dateisystem-Isolation eingeschaltet.

Ob ein verwalteter `credentials.files`-Eintrag `filesystem.disabled` anheftet und den Schlüssel auf verwaltete Einstellungen sperrt, damit Entwickler die Dateisystem-Isolation nicht ausschalten können, hängt vom `mode` des Eintrags und davon ab, was mit dem Eintrag beim Start der Sandbox geschieht:

| Verwalteter Eintrag                                                                                                    | Heftet `filesystem.disabled` an | Was schützt die Datei, wenn Isolation aus ist                                                                                                                     |
| ---------------------------------------------------------------------------------------------------------------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"mode": "deny"`                                                                                                       | Ja                              | Nichts: der Lesblock ist Teil der Dateisystem-Schicht                                                                                                             |
| `"mode": "mask"`, angewendet als Maske                                                                                 | Nein                            | Maskierung selbst: die [Sentinel-Kopie und der Proxy](#mask-credential-files) auf Linux und WSL2, die Sandbox-eigenen Lesevorgänge auf macOS                      |
| `"mode": "mask"`, [auf `deny` zurückgefallen](#mask-credential-files) beim Setup                                       | Nein                            | Nichts, wie `deny`. Führen Sie einen Pfad auf, der nicht maskiert werden kann, wie ein Verzeichnis, als expliziten `deny`-Eintrag auf, der den Schlüssel anheftet |
| `"mode": "mask"`, [durch Validierung zu `deny` herabgestuft](/docs/de/managed-settings#invalid-entries-in-managed-settings) | Ja, wie ein expliziter `deny`   | Nichts, wie `deny`                                                                                                                                                |

Ein Fallback tritt auf, wenn die Sandbox startet, nachdem Claude Code die Einstellungen bereits gelesen hat, auf denen die Anheftungsprüfung läuft, daher heftet ein zurückgefallener Eintrag niemals an. Validierung schreibt einen ungültigen Eintrag zu `deny` um, während Einstellungen geladen werden, daher heftet ein herabgestufter Eintrag wie einer an, den Sie als `deny` geschrieben haben.

<h4 id="what-changes-when-filesystem-isolation-is-off">
  Was ändert sich, wenn Dateisystem-Isolation aus ist
</h4>

Setzen von `filesystem.disabled` hebt die Schutzmaßnahmen auf, die die Dateisystem-Schicht selbst durchsetzt. Schutzmaßnahmen, die andere Schichten durchsetzen, bleiben angewendet:

| Schutz                                                                                               | Mit deaktivierter Dateisystem-Isolation                                                                                                                                                          |
| ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `filesystem.denyRead` und [`credentials.files`](#protect-credentials) `deny` Lesevorgänge blockieren | Nicht durchgesetzt. Die Dateisystem-Schicht wendet beide an                                                                                                                                      |
| `credentials.envVars` `deny` und `mask` Einträge                                                     | Durchgesetzt. Umgebungsvariablen-Scrubbing ist unabhängig von der Dateisystem-Schicht                                                                                                            |
| [`credentials.files` `mask` Einträge](#mask-credential-files) angewendet als Masken                  | Durchgesetzt: Maskierung ist unabhängig von der Dateisystem-Schicht. Ein Eintrag, der [auf `deny` zurückgefallen ist](#mask-credential-files), wird nicht durchgesetzt, wie jeder `deny`-Eintrag |

Zwei weitere Dinge ändern sich:

* Sandbox-Befehle erben `$TMPDIR` Ihrer Shell anstelle des Sitzungs-Temp-Verzeichnisses, da jedes Temp-Verzeichnis beschreibbar ist und Claude Code Befehle nicht mehr zum Sitzungs-Temp-Verzeichnis umleitet.

  Auf Linux ist die Variable oft in der übergeordneten Shell nicht gesetzt. Das Bash-Tool-Leitfaden teilt Claude mit, Scratch-Verzeichnisse mit `mktemp -d` zu erstellen, anstatt sich auf `$TMPDIR` zu verlassen.
* [`autoAllowBashIfSandboxed`](/docs/de/settings-reference#sandbox-autoallowbashifsandboxed) hat immer noch den Standard `true`, daher laufen Sandbox-Befehle ohne Eingabeaufforderungen. Setzen Sie es auf `false`, um Eingabeaufforderungen für Sandbox-Befehle zu erhalten.

<h3 id="protect-credentials">
  Anmeldedaten schützen
</h3>

Die Einstellung `sandbox.credentials` deklariert Anmeldedatendateien und Umgebungsvariablen, um sie vor Sandbox-Befehlen zu schützen. Jeder Eintrag benennt einen Dateipfad oder eine Umgebungsvariable und einen `mode`. Der dedizierte `credentials`-Block hält Anmeldedatenregeln zusammen und getrennt von allgemeinen Dateisystem-Regeln.

Für Einträge mit `"mode": "deny"` werden Dateipfade für Lesevorgänge in der Sandbox blockiert, die gleiche Einschränkung, die `filesystem.denyRead` anwendet, und Umgebungsvariablen werden vor jedem Sandbox-Befehl deaktiviert. Der Dateischutz ist Teil der Dateisystem-Schicht, daher gilt er nicht, wenn Sie [Dateisystem-Isolation deaktivieren](#disable-filesystem-isolation); der Umgebungsvariablen-Schutz gilt immer noch.

Das folgende Beispiel blockiert Lesevorgänge der AWS-Anmeldedatendatei und des SSH-Verzeichnisses und entfernt `GITHUB_TOKEN` und `NPM_TOKEN` aus der Umgebung von Sandbox-Befehlen:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

Umgebungsvariablen-Einträge und Datei-Einträge akzeptieren auch `"mode": "mask"`, das unter [Anmeldedaten maskieren](#mask-credentials) beschrieben wird.

Dateipfade folgen den gleichen [Präfix-Regeln](/docs/de/settings-reference#sandbox-path-prefixes) wie `sandbox.filesystem.*`-Einstellungen.

Claude Code führt die `deny`-Einträge aus jedem [Einstellungs-Scope](/docs/de/settings#settings-precedence) zusammen, den die Sitzung lädt. Ein `deny`-Eintrag engt den Zugriff nur ein, daher kann jeder Scope einen hinzufügen, aber keiner kann einen entfernen, den ein anderer Scope hinzugefügt hat.

Wenn Sie [eine Einstellungsquelle ausschließen](#configure-sandboxing):

* **Projekt- oder lokale Einstellungen**: Claude Code wendet keine ihrer `credentials`-Einträge an. Erfordert Claude Code v2.1.246 oder später.
* **Benutzer-Einstellungen**: Claude Code wendet immer noch die `deny`-Einträge in `~/.claude/settings.json` an und behält ihre [Datei-`mask`-Einträge](#mask-credential-files) als Einschränkungen, aber verwirft ihre [Umgebungsvariablen-`mask`-Einträge](#mask-environment-variables).

Es gibt keine integrierte Anmeldedaten-Ablehnungsliste, daher werden nur die Dateien und Variablen, die Sie auflisten, eingeschränkt.

`sandbox.credentials` betrifft nur Sandbox-Bash-Befehle. Um Anmeldedaten aus allen Subprozessen unabhängig von Sandboxing zu entfernen, setzen Sie [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/de/env-vars).

<h3 id="mask-credentials">
  Anmeldedaten maskieren
</h3>

Maskierung geht weiter als ein `deny`-Eintrag unter [Anmeldedaten schützen](#protect-credentials). Anstatt eine Anmeldedaten zu blockieren, zeigt Claude Code Sandbox-Befehlen einen Platzhalter, den Sentinel, und der [Sandbox-Proxy](#network-isolation) tauscht den echten Wert bei ausgehenden Anfragen an Hosts aus, die Sie zulassen. Für Dateien ist die Substitution Linux- und WSL2-Verhalten; [macOS blockiert die Datei stattdessen](#mask-credential-files).

<h4 id="mask-environment-variables">
  Umgebungsvariablen maskieren
</h4>

`"mode": "mask"` schützt eine Anmeldedaten, während die Tools, die sich damit authentifizieren, funktionieren. `deny` entfernt die Variable vollständig, was auch Tools bricht, die sie benötigen, wie `gh` oder `npm`. Erfordert Claude Code v2.1.199 oder später.

Mit `mask` sieht der Sandbox-Befehl einen pro-Sitzungs-Sentinel-Wert anstelle des echten. Jeder `mask`-Eintrag kann `injectHosts` auflisten, die Hosts, an die der echte Wert erreichen darf. Wenn eine Anfrage die Sandbox für einen von ihnen verlässt, ersetzt der [Sandbox-Proxy](#network-isolation) den Sentinel durch den echten Wert. Der Befehl und alles, das er protokolliert, hält niemals die echte Anmeldedaten, aber seine Anfragen authentifizieren sich immer noch.

Der Proxy ersetzt die Anmeldedaten in Anfrageinhalten, daher muss er diese sehen. Setzen Sie [`network.tlsTerminate`](/docs/de/settings-reference#sandbox-network-tlsterminate), damit der Proxy TLS selbst beendet.

Ohne ihn schlägt die Maskierung geschlossen fehl: Der Befehl sieht immer noch nur den Sentinel, aber der Sentinel erreicht den Server unverändert und die Authentifizierung schlägt fehl. Claude Code meldet diese Fehlkonfiguration beim Start.

Die Substitution deckt Header und Anfragekörper ab. Anfragen, die sich mit einer Signatur authentifizieren, die von der Anmeldedaten abgeleitet ist, anstatt der Anmeldedaten selbst, benötigen eine Neusignierung beim Proxy; [AWS-Anfragen erneut signieren](#re-sign-aws-requests) behandelt, wie das für AWS funktioniert.

Der Proxy injiziert nur auf Verbindungen, die die [Domain-Zulassungsliste](#network-isolation) zulässt, daher muss jedes `injectHosts`-Ziel auch über `network.allowedDomains` erreichbar sein.

Das folgende Beispiel maskiert zwei Token. `GH_TOKEN` wird nur bei Anfragen an `api.github.com` ersetzt, während `NPM_TOKEN` keine `injectHosts` hat und bei Anfragen an jeden Host in `network.allowedDomains` ersetzt wird.

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

<span id="ipv6-destinations-in-injecthosts" />Schreiben Sie ein IPv6-Ziel in den beiden Listen unterschiedlich, da jede Liste ihren eigenen Matcher hat:

* **`network.allowedDomains`**: die [geklammerte Form, die Domain-Listen verwenden](#ipv6-addresses-in-domain-lists), wie `"[::1]"`. Der Proxy prüft diese Liste, um die Verbindung zuzulassen.
* **`injectHosts`**: die bloße Adresse in ihrer kanonischen komprimierten Form, wie `"::1"` oder `"2001:db8::1"`. Der Proxy gleicht jeden Eintrag gegen die bloße Zieladresse der Verbindung ab und ignoriert Ports, daher passt eine geklammerte, Zone-ID- oder anders komprimierte Schreibweise niemals und der Proxy injiziert die Anmeldedaten dort niemals.

`claude doctor` kennzeichnet `injectHosts`-Einträge, die niemals passen können, mit der Warnung `Sandbox credential injectHosts entries can never match their destination`. Diese Prüfung erfordert Claude Code v2.1.229 oder später.

Im Gegensatz zu `deny` autorisiert die Maskierung den Proxy, Ihre echte Anmeldedaten an die aufgelisteten Hosts zu senden, daher wird sie nur von Einstellungen berücksichtigt, die Sie oder Ihr Administrator kontrollieren: Benutzer-Einstellungen, verwaltete Einstellungen und das `--settings` CLI-Flag. Claude Code ignoriert `mask`-Einträge in der `.claude/settings.json` oder `.claude/settings.local.json` eines Repositorys. In diesen Dateien ignoriert es auch `network.tlsTerminate` und [`credentials.allowPlaintextInject`](/docs/de/settings-reference#sandbox-credentials-allowplaintextinject), die Einstellung, die dem Proxy erlaubt, Anmeldedaten in unverschlüsselte Anfragen zu injizieren. Wenn Sie [Benutzer-Einstellungen ausschließen](#configure-sandboxing), verwirft Claude Code die Umgebungsvariablen-`mask`-Einträge in `~/.claude/settings.json` auch.

Wenn Ihr Administrator `mask`-Einträge, `network.tlsTerminate` oder `credentials.allowPlaintextInject` über server-verwaltete Einstellungen bereitstellt, zählen sie als [Einstellungen, die Genehmigung benötigen](/docs/de/server-managed-settings#security-approval-dialogs).

Wenn die gleiche Variable mit `deny` in einem beliebigen Scope aufgelistet ist, hat `deny` Vorrang.

Die Maskierung ersetzt standardmäßig den gesamten Wert der Variablen, was sich für einen bloßen Token eignet. Optionale Eintragsfelder, die Claude Code v2.1.224 oder später erfordern, behandeln Werte mit Struktur:

* `extract`: ein regulärer Ausdruck, den Claude Code über den Wert anwendet und nur den Text ersetzt, der von Gruppe 1 jedes Treffers erfasst wird, daher funktioniert ein Tool, das den Wert analysiert, wie eine `DATABASE_URL`-Verbindungszeichenfolge, immer noch in der Sandbox. Das Muster muss mindestens eine erfassende Gruppe enthalten.
* `onExtractNoMatch` steuert, was geschieht, wenn das Muster nichts trifft:
  * `warn`, der Standard, warnt und übergibt die Variable unmasked
  * `deny` deaktiviert die Variable in der Sandbox
  * `error` stoppt das Sandbox-Setup, bis Sie die Konfiguration beheben
* `decode: "jwt"`: für eine Variable, die ein JSON Web Token (JWT) hält. Claude Code überprüft, dass der Wert ein JWT ist, und ersetzt ihn durch ein strukturell gültiges gefälschtes Token, daher funktioniert Code in der Sandbox, der das Token dekodiert, weiterhin. Fügen Sie `maskClaims` hinzu, um nur die aufgelisteten Top-Level-Payload-Ansprüche einzeln zu maskieren, anstatt das gesamte Token zu ersetzen; die anderen Ansprüche bleiben lesbar. Wenn der Wert nicht als JWT überprüft wird oder kein aufgelisteter Anspruch passt, übergibt Claude Code die Variable unmasked mit einer Warnung. `decode` kann nicht mit `extract` kombiniert werden.

Siehe die [`credentials.envVars[]`-Zeilen in der Einstellungsreferenz](/docs/de/settings-reference#sandbox-settings) für die vollständige Feldliste.

<h4 id="re-sign-aws-requests">
  AWS-Anfragen erneut signieren
</h4>

AWS-Anfragen tragen SigV4-Signaturen über die Anfrageinhalte, daher maskieren Sie `AWS_ACCESS_KEY_ID` und `AWS_SECRET_ACCESS_KEY` zusammen. Der Proxy erkennt eine SigV4-Anfrage durch den Sentinel des Zugangsschlüssels und signiert sie erneut, nachdem er die echten Werte ersetzt hat. Das Maskieren des Geheimnisses allein hinterlässt Anfragen, die mit dem Platzhalter signiert sind, den der Proxy nicht erkennen kann, daher schlagen sie bei AWS fehl; Claude Code warnt in diesem Fall beim Start, aber nicht, wenn nur die Zugangsschlüssel-ID maskiert ist. Eine erkannte Anfrage, die der Proxy nicht erneut signieren kann, wie eine, der der `x-amz-date`-Header fehlt, schlägt mit einem Proxy-Fehler fehl, anstatt den Server mit einer fehlerhaften Signatur zu erreichen.

Claude Code verknüpft die konventionellen `AWS_ACCESS_KEY_ID`-, `AWS_SECRET_ACCESS_KEY`- und `AWS_SESSION_TOKEN`-Variablen automatisch zu einer Anmeldedaten, wenn Sie ihre gesamten Werte maskieren. Wenn Ihre AWS-Anmeldedaten in Variablen mit anderen Namen lebt, gruppieren Sie sie selbst mit [`credentials.awsPairs`](/docs/de/settings-reference#sandbox-credentials-awspairs), das Claude Code v2.1.224 oder später erfordert. Dieses Beispiel fügt die Paarung zu einer Konfiguration hinzu, die bereits `MY_KEY_ID`, `MY_SECRET_KEY` und `MY_SESSION_TOKEN` ganz-Wert maskiert, wie in der [Maskierungskonfiguration oben](#mask-environment-variables):

```json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Jeder Eintrag folgt diesen Regeln:

* `accessKeyIdVar` und `secretAccessKeyVar` benennen die maskierten `envVars`-Einträge, die die Zugangsschlüssel-ID und den geheimen Schlüssel halten. Das optionale `sessionTokenVar` benennt den Eintrag, der das Sitzungs-Token für temporäre Anmeldedaten hält; wenn gesetzt, sendet der Proxy das echte Token als `x-amz-security-token` bei erneut signierten Anfragen.
* Jede benannte Variable muss ein `mask`-Eintrag sein, der ihren gesamten Wert maskiert, ohne `extract` oder `decode`.
* Der Proxy signiert Anfragen auf den Hosts erneut, die in der Zugangsschlüssel-ID-Eintrags-`injectHosts` aufgelistet sind.
* Das Benennen einer der konventionellen Variablen in einem Paar ersetzt die automatische Paarung.

Wie `mask`-Einträge wird `awsPairs` nur von Benutzer-Einstellungen, verwalteten Einstellungen und dem `--settings` CLI-Flag berücksichtigt.

Drei AWS-Anfrage-Formen tragen Signaturen, die der Proxy nicht neu berechnen kann. Wenn eine solche Anfrage mit einem Platzhalter eines maskierten Paares signiert ist, schlägt der Proxy sie fehl, anstatt eine fehlerhafte Signatur weiterzuleiten; Anfragen, die mit unmaskierten Anmeldedaten signiert sind, sind niemals betroffen. Die Einstellung [`credentials.sigv4`](/docs/de/settings-reference#sandbox-credentials-sigv4), die Claude Code v2.1.224 oder später erfordert, lockert dies pro Form: Das Setzen des Schlüssels einer Form auf `passthrough` leitet die Anfrage mit ihrer Platzhalter-abgeleiteten Signatur weiter, daher erhält das aufrufende Tool die eigene Ablehnungsantwort von AWS anstelle eines Proxy-Fehlers. Wie `awsPairs` wird `sigv4` nur von Benutzer-Einstellungen, verwalteten Einstellungen und dem `--settings` CLI-Flag berücksichtigt.

| Anfrage-Form                    | `sigv4` Schlüssel | Warum der Proxy sie nicht erneut signieren kann                                                                               |
| :------------------------------ | :---------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| aws-chunked Streaming-Uploads   | `streaming`       | Pro-Chunk-Signaturen verketten sich von der Seed-Signatur, daher würde eine Neusignierung das Rewriting des Körpers erfordern |
| Vorsignierte URLs               | `presigned`       | Die Signatur lebt in der URL selbst, ohne `Authorization`-Header                                                              |
| SigV4A asymmetrische Signaturen | `sigv4a`          | Es gibt keinen gemeinsamen Schlüssel-HMAC zum Neuberechnen                                                                    |

<h4 id="mask-credential-files">
  Anmeldedatendateien maskieren
</h4>

Datei-Einträge akzeptieren auch `"mode": "mask"`, das Claude Code v2.1.221 oder später erfordert. Was ein Sandbox-Befehl sieht, hängt von der Plattform ab:

* **Linux und WSL2**: Sandbox-Befehle lesen eine Sentinel-Kopie der Datei, einen Platzhalter, dessen Geheimnis durch einen Platzhalter-Wert ersetzt ist, und der [Sandbox-Proxy](#network-isolation) ersetzt den echten Wert bei Ausgang.
* **macOS**: Sandbox-Befehle können die aufgelistete Datei überhaupt nicht lesen. Claude Code erstellt keine Sentinel-Kopie und ersetzt nichts bei Ausgang, daher funktionieren Tools, die sich mit der Datei authentifizieren, nicht in der Sandbox, der gleiche Effekt wie `deny`. Im Gegensatz zu einem `deny`-Eintrag gilt der Lesblock auch, wenn Sie [Dateisystem-Isolation deaktivieren](#disable-filesystem-isolation).

Auf jeder Plattform wendet Claude Code die [`network.tlsTerminate`](/docs/de/settings-reference#sandbox-network-tlsterminate)-Anforderung und `injectHosts` genauso an wie für [maskierte Umgebungsvariablen](#mask-environment-variables), und ignoriert Repository-Einstellungen genauso. Wenn Sie [Benutzer-Einstellungen ausschließen](#configure-sandboxing), behält Claude Code die Datei-`mask`-Einträge in `~/.claude/settings.json` als Einschränkungen, aber die Einträge autorisieren den Proxy nicht mehr, den echten Wert zu ersetzen.

Das folgende Beispiel maskiert ein GitHub-Token, das in `~/.config/gh/hosts.yml` gespeichert ist; das `extract`-Muster, das unten behandelt wird, teilt Claude Code mit, welcher Teil der Datei das Geheimnis ist. Auf Linux und WSL2 erhalten Sandbox-Befehle, die die Datei lesen, einen Sentinel anstelle des Tokens, und der Proxy ersetzt das echte Token bei Anfragen an `api.github.com`:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

Um zu bestätigen, dass die Maske aktiv ist, bitten Sie Claude, `cat ~/.config/gh/hosts.yml` in einem Sandbox-Befehl auszuführen: Auf Linux und WSL2 zeigt die Ausgabe einen Sentinel-Wert anstelle des Tokens, und auf macOS schlägt der Lesevorgang stattdessen fehl.

Auf Linux und WSL2 ist das `extract`-Muster das, was den Rest von `hosts.yml` lesbar hält. Claude Code wendet den regulären Ausdruck über die gesamte Datei an und ersetzt nur den Text, der von Gruppe 1 jedes Treffers erfasst wird, daher analysiert `gh` immer noch seine Konfiguration und nur das Token ist ein Platzhalter. Verwenden Sie `extract` für jede strukturierte Datei, die Tools analysieren, wie `.netrc`, JSON oder YAML; das Muster muss mindestens eine erfassende Gruppe enthalten. Ohne `extract` ersetzt Claude Code den gesamten Dateiinhalt durch einen Sentinel-Wert, was sich für eine Datei eignet, die ein einzelnes bloßes Geheimnis und nichts anderes hält.

Für eine Datei, die ein JSON Web Token (JWT) hält, setzen Sie `decode: "jwt"` anstelle von oder zusammen mit `extract`. `decode` erfordert Claude Code v2.1.224 oder später. Claude Code findet JWT-Kandidaten mit einem eingebauten Muster oder mit Ihrem `extract`-Muster, wenn gesetzt, überprüft jeden Kandidaten, dass er ein JWT ist, und ersetzt ihn durch ein strukturell gültiges gefälschtes Token, daher funktioniert Code, der das Token in der Sandbox dekodiert, weiterhin. Fügen Sie `maskClaims` hinzu, um nur die benannten Top-Level-Payload-Ansprüche in jedem überprüften Token zu maskieren und die anderen Ansprüche lesbar zu lassen. Wenn kein Kandidat überprüft wird oder kein benannter Anspruch passt, regelt das Feld `onExtractNoMatch` unten das Ergebnis, wie es auch für ein Muster tut, das nichts trifft.

Zwei optionale Felder verfeinern, wie Matching sich verhält. Beide gelten nur, wenn `mode` `mask` ist und `extract` oder `decode` gesetzt ist. Auf macOS wendet Claude Code `mask`-Einträge als `deny` an, bevor das Muster läuft, wann immer Dateisystem-Isolation an ist, daher gelten diese Felder und die No-Match-Ergebnisse unten dort nur, wenn [Dateisystem-Isolation aus ist](#disable-filesystem-isolation):

* `onExtractNoMatch` steuert, was geschieht, wenn Matching nichts zu maskieren in der Datei findet:

  * `warn`, der Standard, warnt und überspringt den Eintrag, daher können Sandbox-Befehle die echte Datei unmasked lesen. Der Standard eignet sich für Anmeldedaten, die legitim abwesend sein können; wenn das Geheimnis möglicherweise vorhanden ist, aber das Muster es möglicherweise verpasst, verwenden Sie `deny`
  * `deny` macht die Datei stattdessen unlesbar
  * `error` stoppt das Sandbox-Setup, bis Sie die Konfiguration beheben

  Claude Code behandelt `deny` als `error`, wann immer der Lesblock nicht durchgesetzt würde: wenn Sie [Dateisystem-Isolation deaktivieren](#disable-filesystem-isolation), und wenn ein `filesystem.allowRead`-Eintrag aus einer beliebigen Einstellungsquelle den Dateipfad erneut öffnet.
* `maskDuplicates` ersetzt auch wörtliche Kopien jedes maskierten Anmeldedaten-Wertes, eine `extract`-Erfassung oder ein `decode`-überprüftes Token, das außerhalb der abgestimmten Spannweiten gefunden wird, für ein Geheimnis, das wiederholt wird, wo Matching nicht reicht. Es gleicht rohe Teilzeichenfolgen ab, daher würde ein kurzer oder häufiger Wert überall ersetzt, wo er erscheint; reservieren Sie es für lange, hochentropische Geheimnisse. Standard: false.

`mask` gilt für eine einzelne Datei, daher führen Sie jede Anmeldedatendatei einzeln auf. Claude Code fällt auf `deny` zurück für einen `mask`-Eintrag, den es nicht sicher maskieren kann: einen Verzeichnispfad, ein Glob-Muster, eine Datei größer als 8 MiB oder eine Datei, die kein UTF-8-Text ist. Schreiben Sie Verzeichnisse stattdessen als explizite `deny`-Einträge; die Tabelle unter [Welche Einstellungen können es deaktivieren](#which-settings-can-disable-it) behandelt, ob jede Form `filesystem.disabled` anheftet und wie sie sich mit Dateisystem-Isolation aus verhält.

<h2 id="how-sandboxing-works">
  Wie Sandboxing funktioniert
</h2>

<h3 id="filesystem-isolation">
  Dateisystem-Isolation
</h3>

Das Sandboxed-Bash-Tool beschränkt den Dateisystem-Zugriff auf spezifische Verzeichnisse:

* **Standard-Schreibverhalten**: Lese- und Schreibzugriff auf das aktuelle Arbeitsverzeichnis und seine Unterverzeichnisse, alle Verzeichnisse, die Sie mit `--add-dir`, `/add-dir` oder [`permissions.additionalDirectories`](/docs/de/settings-reference#permissions-additionaldirectories) hinzugefügt haben, plus das Session-Temp-Verzeichnis, auf das `$TMPDIR` verweist
* **Standard-Leseverhalten**: Lesezugriff auf den gesamten Computer, außer bestimmten blockierten Verzeichnissen. Beachten Sie, dass diese Standard-Einstellung immer noch das Lesen von Anmeldedatendateien wie `~/.aws/credentials` und `~/.ssh/` ermöglicht. Verwenden Sie [`sandbox.credentials`](#protect-credentials), um Lesezugriffe auf diese Dateien zu blockieren und geheime Umgebungsvariablen zu deaktivieren, oder fügen Sie die Pfade zu `denyRead` hinzu.
* **Blockierter Zugriff**: Kann Dateien außerhalb des Arbeitsverzeichnisses, hinzugefügter Verzeichnisse und des Session-Temp-Verzeichnisses nicht ohne explizite Genehmigung ändern, einschließlich Shell-Konfigurationsdateien wie `~/.bashrc` und System-Binärdateien in `/bin/`
* **Git Worktrees**: Wenn das Arbeitsverzeichnis ein [verknüpftes Git Worktree](/docs/de/worktrees) ist, erlaubt die Sandbox auch Schreibzugriff auf das gemeinsame `.git`-Verzeichnis des Haupt-Repositorys, damit Befehle wie `git commit` Refs und den Index aktualisieren können. Schreibzugriffe auf `hooks/` und `config` in diesem Verzeichnis bleiben blockiert.
* **Konfigurierbar**: Definieren Sie benutzerdefinierte zulässige und blockierte Pfade durch Einstellungen

Um die Dateisystem-Isolation vollständig zu überspringen und dabei die Netzwerk-Isolation beizubehalten, setzen Sie [`sandbox.filesystem.disabled`](#disable-filesystem-isolation).

<h3 id="protected-paths">
  Geschützte Pfade
</h3>

Innerhalb der Verzeichnisse, in die Sandbox-Befehle schreiben können, blockiert die Sandbox immer noch Schreibzugriffe auf die Dateien, aus denen Claude Code Konfiguration und Code lädt. Ein Befehl, der diese Dateien bearbeiten könnte, könnte sich selbst Berechtigungen gewähren oder einen Hook oder MCP-Server hinzufügen, den Claude Code außerhalb der Sandbox ausführt. Das Berechtigungssystem hat seine eigenen [geschützten Pfade](/docs/de/permission-modes#protected-paths), die steuern, was Claude Code genehmigt, bevor ein Tool ausgeführt wird; die Liste der Sandbox gilt für einen Befehl, der bereits ausgeführt wird. Sie umfasst vier Gruppen von Pfaden:

* **In Ihrem Arbeitsverzeichnis und den Verzeichnissen darüber**: die `.claude`-Einstellungsdateien, die Verzeichnisse `.claude/skills`, `.claude/agents`, `.claude/commands` und `.claude/hooks`, `.mcp.json` und die Dateien, die Claude Code selbst ausführt, wie `.claude/workflows` und `.claude/scheduled_tasks.json`
* **Nur in Ihrem Arbeitsverzeichnis**: Shell-Startdateien wie `.bashrc` und `.zshrc`, `.gitconfig`, die Verzeichnisse `.vscode` und `.idea` sowie `hooks` und `config` innerhalb von `.git`
* **Dateien, die Ihr Arbeitsverzeichnis in ein Bare-Git-Repository umwandeln würden**: `HEAD`, `objects` und `refs` auf der obersten Ebene, plus `config` und `hooks` dort, wenn ein `HEAD` neben ihnen sitzt. Eine Datei namens `config` wird auch ohne `HEAD` blockiert. Unter Linux und WSL2 löscht die Sandbox eine `HEAD`-Datei oder ein `objects`- oder `refs`-Verzeichnis auf der obersten Ebene, das während der Ausführung eines Sandbox-Befehls erscheint
* **In `~/.claude` oder dem Verzeichnis, auf das `CLAUDE_CONFIG_DIR` verweist**: die meisten seiner Inhalte, plus `~/.claude.json` und der Anmeldedaten-Speicher `.credentials.json`

Wenn ein Symlink während der Sitzung auf dem Pfad einer geschützten Einstellungsdatei erscheint, blockiert die Sandbox auch Schreibzugriffe auf die Datei, auf die er verweist, ab dem nächsten Befehl.

Es gibt keine Möglichkeit, einen dieser Pfade auszunehmen: Ein `allowWrite`-Eintrag oder eine `Edit`-Erlaubnisregel, die den Pfad abdeckt, hebt den Schutz nicht auf. Die einzige Möglichkeit, den Schutz auszuschalten, ist [`filesystem.disabled`](#disable-filesystem-isolation), was die Dateisystem-Isolation für jeden Pfad ausschaltet. Um die meisten dieser Pfade für Ihren Computer aufgelöst zu sehen, führen Sie `/sandbox` aus und öffnen Sie die Registerkarte **Config**, die sie unter **Denied within allowed** auflistet, gemischt mit Ihren eigenen `denyWrite`-Einträgen.

Wenn `git merge` oder `git checkout` mit `unable to unlink old` auf einem dieser Pfade fehlschlägt, siehe [Troubleshooting](#troubleshooting).

<h3 id="network-isolation">
  Netzwerk-Isolation
</h3>

Der Netzwerkzugriff wird durch einen Proxy-Server gesteuert, der außerhalb der Sandbox läuft:

* **Domain-Einschränkungen**: Claude Code erlaubt standardmäßig keine Domains vorab. Wenn ein Befehl zum ersten Mal eine neue Domain benötigt, fordert Claude Code zur Genehmigung auf; im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) nennt Claude stattdessen die Hosts, die ein Befehl benötigt, auf dem Befehl selbst, pro [Per-command allowed domains](#per-command-allowed-domains-in-auto-mode).
* **Genehmigungsoptionen**: Wenn Sie bei der Aufforderung „Ja" wählen, erlaubt Claude Code den Host für den Rest der aktuellen Sitzung und fordert nicht erneut auf für spätere Verbindungen zum gleichen Host. Wenn Sie „Ja, und nicht mehr fragen" wählen, speichert Claude Code eine `WebFetch(domain:...)`-Erlaubnisregel in Ihren [lokalen Einstellungen](/docs/de/permissions#permission-system), sodass der Host in zukünftigen Sitzungen zulässig bleibt.
* **Vorab zulässige Domains**: Lassen Sie Domains vorab mit [`allowedDomains`](/docs/de/settings-reference#sandbox-network-alloweddomains) zu, um die Aufforderung vollständig zu vermeiden. Claude Code erlaubt auch Domains vorab aus `WebFetch(domain:...)`-Erlaubnisregeln, wie in [Permission rules](#permission-rules) beschrieben.
* **Strikte Zulassungsliste**: Wenn Sie [`strictAllowlist`](/docs/de/settings-reference#sandbox-network-strictallowlist) in Benutzer-, verwalteten oder CLI-`--settings`-Einstellungen auf `true` setzen, blockiert Claude Code den Zugriff von Sandbox-Befehlen auf jeden Host außerhalb der Zulassungsliste, anstatt zu fragen. Die Zulassungsliste ist die gleiche, gegen die die Sandbox sonst fragt: `allowedDomains` plus Domains aus `WebFetch(domain:...)`-Erlaubnisregeln, oder nur die Einträge der verwalteten Einstellungen, wenn `allowManagedDomainsOnly` gesetzt ist. Claude Code erzwingt dies nur für Sandbox-Befehle; In-Process-Tools wie `WebFetch` folgen immer noch ihren [Permission rules](#permission-rules). Das Setzen in der `.claude/settings.json` oder `.claude/settings.local.json` eines Repositorys hat keine Auswirkung. Erfordert Claude Code v2.1.219 oder später.
* **Verwaltete Sperrung**: Wenn [`allowManagedDomainsOnly`](/docs/de/settings-reference#sandbox-network-allowmanageddomainsonly) in verwalteten Einstellungen gesetzt ist, werden nicht zulässige Domains automatisch blockiert, anstatt zu fragen, und nur `allowedDomains` und `WebFetch(domain:...)`-Erlaubnisregeln aus verwalteten Einstellungen werden berücksichtigt.
* **Unternehmens-Proxy**: Wenn Ihr Netzwerk erfordert, dass ausgehender Datenverkehr durch einen Unternehmens-Proxy geleitet wird, setzen Sie `HTTPS_PROXY`, `HTTP_PROXY` und `NO_PROXY` wie [Proxy-Konfiguration](/docs/de/network-config#proxy-configuration) beschreibt, im `env`-Block Ihrer Einstellungen, damit [Background Agents](/docs/de/network-config#set-network-variables-in-settings-not-the-shell) sie auch erhalten, oder in der Umgebung, aus der Sie Claude Code starten. Claude Code erzwingt die Domain-Zulassungsliste und tunnelt dann zulässige Verbindungen durch diesen Upstream-Proxy.
* **Benutzerdefinierte Proxy-Unterstützung**: Fortgeschrittene Benutzer können benutzerdefinierte Regeln für ausgehenden Datenverkehr implementieren
* **Umfassende Abdeckung**: Einschränkungen gelten für alle Skripte, Programme und Subprozesse, die durch Befehle erzeugt werden

In einer `WebFetch(domain:...)`-Regel ehrt die Sandbox zwei Wildcard-Formen: ein führendes `*.`, wie `*.example.com`, und ein einfaches `*`. Die einfache `*`-Form erfordert Claude Code v2.1.186 oder später. Ein Wildcard an einer anderen Position, wie `WebFetch(domain:example.*)`, stimmt immer noch mit Abrufen überein, hat aber keine Auswirkung auf Sandbox-Befehle.

<Note>
  Der integrierte Proxy erzwingt die Zulassungsliste basierend auf dem angeforderten Hostnamen und beendet oder inspiziert standardmäßig keinen TLS-Datenverkehr. Die experimentelle Einstellung [`network.tlsTerminate`](/docs/de/settings-reference#sandbox-network-tlsterminate), verfügbar in Claude Code v2.1.199 und später, lässt den integrierten Proxy TLS selbst beenden, was [`mask`-Anmeldedateneinträge](#mask-credentials) erfordern. Siehe [Security limitations](#security-limitations) für die Auswirkungen des Standards und [Custom proxy configuration](#custom-proxy-configuration), wenn Ihr Bedrohungsmodell TLS-Inspektion erfordert.
</Note>

<h4 id="per-command-allowed-domains-in-auto-mode">
  Per-command allowed domains im Auto-Modus
</h4>

Im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) mit aktiviertem Sandboxing nennt Claude die Hosts, die ein Befehl benötigt, auf dem Befehl selbst, anstatt für jede Verbindung eine Netzwerk-Genehmigung auszulösen. Jeder Bash-, PowerShell- oder [Monitor](/docs/de/tools-reference#monitor-tool)-Befehl, der in der Sandbox ausgeführt wird, kann eine Liste von Hosts über die Zulassungsliste der Sandbox hinaus tragen: eine Domain wie `registry.npmjs.org`, ein Wildcard wie `*.pythonhosted.org` oder eine IP-Adresse, jeweils mit einem optionalen `:port`. Der Klassifizierer überprüft die Hosts zusammen mit dem Befehl. Erfordert Claude Code v2.1.271 oder später.

Eine genehmigte Liste öffnet diese Hosts nur für diesen einen Befehl, solange er ausgeführt wird. Nichts wird zu Ihren Session-zulässigen Hosts oder Ihren Einstellungen hinzugefügt; der nächste Befehl nennt seine eigenen Hosts.

Ein Befehl, der Hosts trägt, geht an den Klassifizierer, anstatt von einer Erlaubnisregel oder dem [Auto-Allow-Modus](#sandbox-modes) der Sandbox genehmigt zu werden. Wenn eine [ask-Regel](/docs/de/permissions#manage-permissions) eine Aufforderung für den Befehl erzwingt, listet der Berechtigungsdialog in Ihrem Terminal die Hosts daneben auf, und das Genehmigen dort deckt beides ab.

Eine Per-Command-Liste verbreitert nur das, was die Sandbox standardmäßig blockiert. [`deniedDomains`](/docs/de/settings-reference#sandbox-network-denieddomains)-Einträge blockieren immer noch. Wenn [`strictAllowlist`](/docs/de/settings-reference#sandbox-network-strictallowlist) oder [`allowManagedDomainsOnly`](/docs/de/settings-reference#sandbox-network-allowmanageddomainsonly) die Zulassungsliste sperrt, lehnt Claude Code Per-Command-Listen ab.

Während Per-Command-Listen gelten, lehnt Claude Code eine Verbindung zu einem Host ab, den kein genehmigter Befehl aufgelistet hat, ohne eine Aufforderung oder eine Klassifizierer-Überprüfung. Die Ablehnung nennt den Host im Ergebnis des Befehls, und Claude führt den Befehl mit dem hinzugefügten Host erneut aus.

<h4 id="ipv6-addresses-in-domain-lists">
  IPv6-Adressen in Domain-Listen
</h4>

Die Domain-Listen der Sandbox sind `allowedDomains`, `deniedDomains` und die `WebFetch(domain:...)`-Regeln, die sie speisen. Um eine IPv6-Adresse in einer von ihnen zu treffen, schreiben Sie das Literal in Klammern: `"[::1]"` stimmt mit dieser Adresse auf jedem Port überein, und `"[::1]:443"` stimmt mit ihr nur auf Port 443 überein. Schreiben Sie den Port als Zahl von 1 bis 65535 ohne führende Nullen. Die geklammerte Form erfordert Claude Code v2.1.229 oder später. Vor v2.1.229, wenn der Text nach dem letzten Doppelpunkt eines ungeklammerten Eintrags eine Portnummer war, las Claude Code ihn als eine, also benannte `::1:443` die Adresse `::1` auf Port 443.

Wenn Sie bei der Netzwerk-Genehmigungsaufforderung für eine IPv6-Adresse „Ja, und nicht mehr fragen" wählen, speichert Claude Code die `WebFetch(domain:...)`-Regel mit der Adresse geklammert, sodass die Regel die Adresse in zukünftigen Sitzungen weiterhin trifft.

Ein ungeklammerter Eintrag mit zwei oder mehr Doppelpunkten ist mehrdeutig: `::1:443` ist sowohl eine vollständige IPv6-Adresse als auch eine Adresse gefolgt von einem Port. Claude Code erzwingt mehrdeutige Schreibweisen konservativ, anstatt zu erraten, welche Lesart Sie gemeint haben:

* **Deny-Listen**: Claude Code blockiert jede Lesart, die der Eintrag analysiert, also wird die Lesart, die Sie gemeint haben, blockiert. Für einen Eintrag ohne analysierbare Lesart blockiert Claude Code nichts.
* **Allow-Listen**: Claude Code erlaubt nie mehr als Sie geschrieben haben. Es schreibt einen mehrdeutigen Eintrag in seine Host-und-Port-Lesart um, wenn diese Lesart sauber analysiert, und kann den Eintrag vollständig fallen lassen, anstatt die Zulassungsliste zu verbreitern.

Führen Sie `claude doctor` in Ihrem Terminal aus, um die betroffenen Einträge zu finden: Die Warnung `Sandbox network domain entries have unreliable spellings` nennt bis zu drei von ihnen und zählt den Rest. Schreiben Sie jeden in der geklammerten Form um, um die Warnung zu löschen. Die Warnung nennt auch Einträge, deren Schreibweise aus anderen Gründen unzuverlässig ist, wie `@`, Pfad- oder Abfragezeichen oder Wildcards in Klammern.

<h3 id="os-level-enforcement">
  OS-Level-Durchsetzung
</h3>

Das Sandboxed-Bash-Tool nutzt Betriebssystem-Sicherheits-Primitive:

* **macOS**: Verwendet Seatbelt für Sandbox-Durchsetzung
* **Linux**: Verwendet [bubblewrap](https://github.com/containers/bubblewrap) für Isolation
* **WSL2**: Verwendet bubblewrap, wie Linux

WSL1 wird nicht unterstützt, da bubblewrap Kernel-Features erfordert, die nur in WSL2 verfügbar sind.

Diese gleichen Primitive sind als das eigenständige Paket [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) verfügbar, das die Seite [Sandbox-Umgebungen](/docs/de/sandbox-environments#sandbox-runtime) als separaten Ansatz zum Wrapping des gesamten Claude Code-Prozesses behandelt.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Wie Sandboxing sich auf Genehmigungen und Genehmigungsmodi bezieht
</h2>

Sandboxing, [Genehmigungsregeln](/docs/de/permissions) und [Genehmigungsmodi](/docs/de/permission-modes) sind komplementäre Schichten. Die folgenden Abschnitte behandeln, wie die Sandbox mit jedem interagiert.

<h3 id="permission-rules">
  Genehmigungsregeln
</h3>

Genehmigungsregeln und Sandboxing steuern verschiedene Dinge:

* **Genehmigungsregeln** steuern, welche Tools Claude Code verwenden kann, und werden evaluiert, bevor ein Tool ausgeführt wird. Sie gelten für alle Tools: Bash, Read, Edit, WebFetch, MCP und andere, außer dass eine Deny- oder Ask-Regel [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior) nicht blockieren kann, während ein anderes Tool verbleibt.
* **Sandboxing** bietet OS-Level-Durchsetzung, die einschränkt, worauf Shell-Befehle auf Dateisystem- und Netzwerk-Ebene zugreifen können. Es gilt nur für Bash, PowerShell und [Monitor](/docs/de/tools-reference#monitor-tool)-Befehle und ihre Kindprozesse.

Die beiden Schichten unterscheiden sich auch in ihrer Durchsetzung. Claude Code evaluiert Genehmigungsentscheidungen, bevor ein Befehl ausgeführt wird, basierend auf der Befehlszeichenfolge und, im Auto-Modus, dem Urteil eines separaten Klassifizierers darüber, ob der Befehl sicher ist. Das Betriebssystem erzwingt die Sandbox-Grenze auf dem laufenden Prozess, daher gilt sie unabhängig davon, was das Modell ausführen wollte, und selbst wenn ein zulässiger Befehl mehr tut als sein Name vermuten lässt.

Dateisystem- und Netzwerk-Einschränkungen werden sowohl durch Sandbox-Einstellungen als auch durch Genehmigungsregeln konfiguriert:

| Einstellung oder Regel                                           | Was es tut                                                                                                |
| :--------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                  | Gewährt Subprozess-Schreibzugriff auf Pfade außerhalb des Arbeitsverzeichnisses                           |
| `sandbox.filesystem.denyWrite` und `sandbox.filesystem.denyRead` | Blockiert Subprozess-Zugriff auf spezifische Pfade                                                        |
| `sandbox.filesystem.allowRead`                                   | Erlaubt das Lesen spezifischer Pfade innerhalb einer `denyRead`-Region erneut                             |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation)   | Deaktiviert die Dateisystem-Schicht vollständig, während die Netzwerk-Isolation beibehalten wird          |
| `Edit` Zulassungsregeln                                          | Gewähren Schreibzugriff auf spezifische Pfade, auf die gleiche Weise wie `sandbox.filesystem.allowWrite`  |
| `Read` und `Edit` Deny-Regeln                                    | Blockiert Zugriff auf spezifische Dateien oder Verzeichnisse                                              |
| `WebFetch(domain:...)` Zulassungs- und Deny-Regeln               | Steuern Domain-Zugriff                                                                                    |
| Sandbox `allowedDomains`                                         | Steuert, auf welche Domains Shell-Befehle zugreifen können                                                |
| Sandbox `deniedDomains`                                          | Blockiert spezifische Domains, auch wenn ein breiteres `allowedDomains`-Wildcard sie sonst zulassen würde |

Pfade und Domains aus beiden Sandbox-Einstellungen und Genehmigungsregeln werden zusammengeführt in die endgültige Sandbox-Konfiguration.

Das [Repository der claude-code mit Beispielen](https://github.com/anthropics/claude-code/tree/main/examples/settings) enthält Starter-Einstellungskonfigurationen für häufige Bereitstellungsszenarien, einschließlich Sandbox-spezifischer Beispiele. Verwenden Sie diese als Ausgangspunkte und passen Sie sie an Ihre Anforderungen an.

<h3 id="permission-modes">
  Genehmigungsmodi
</h3>

`/sandbox` ist kein [Genehmigungsmodus](/docs/de/permission-modes). Genehmigungsmodi entscheiden, ob ein Tool-Aufruf ausgeführt wird und ob Sie zuerst aufgefordert werden, während die Sandbox einschränkt, worauf ein Bash-Befehl zugreifen kann, sobald er ausgeführt wird. Sie unterscheiden sich darin, was sie steuern und was die Pro-Aktion-Eingabeaufforderung ersetzt:

|                                                                     | Was es steuert                                                   | Was die Eingabeaufforderung ersetzt                                                                                                                                                                                             |
| :------------------------------------------------------------------ | :--------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/sandbox`                                                          | Worauf ein Bash-Befehl zugreifen kann, sobald er ausgeführt wird | Die Sandbox-Grenze selbst, im [Auto-Allow-Modus](#sandbox-modes)                                                                                                                                                                |
| [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) | Ob jeder Tool-Aufruf ausgeführt wird                             | Ein Klassifizierer, der Aktionen überprüft                                                                                                                                                                                      |
| `--dangerously-skip-permissions`                                    | Ob jeder Tool-Aufruf ausgeführt wird                             | Nichts. [Geschützte Pfad](/docs/de/permission-modes#protected-paths)-Prüfungen werden auch übersprungen; die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves), gelten immer noch |

Der [Auto-Allow-Modus](#sandbox-modes) der Sandbox ist separat vom [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode): Auto-Allow genehmigt Bash-Befehle, weil die Sandbox-Grenze sie enthält, während der Auto-Modus einen Klassifizierer verwendet, um Aktionen zu überprüfen. Die beiden funktionieren unabhängig und können kombiniert werden, mit den Ausnahmen, die unter [Sandbox-Modi](#sandbox-modes) aufgelistet sind. Um eine Isolationsgrenze für unbeaufsichtigte Läufe zu wählen, siehe [Sandbox-Umgebungen](/docs/de/sandbox-environments#how-isolation-relates-to-permission-modes). Eine Tabelle mit häufigen Genehmigungsmodus- und Sandbox-Paarungen mit den Flags, die jeweils starten, finden Sie unter [Häufige Setups](/docs/de/permission-modes#common-setups).

<h2 id="configure-the-sandbox-for-your-organization">
  Konfigurieren Sie die Sandbox für Ihre Organisation
</h2>

Administratoren können Sandboxing für jeden Benutzer erfordern, Entwickler daran hindern, die Richtlinie zu erweitern, und Sandbox-Datenverkehr durch einen Unternehmens-Proxy leiten.

<h3 id="enforce-sandboxing-with-managed-settings">
  Erzwingen Sie Sandboxing mit verwalteten Einstellungen
</h3>

Um die Sandbox für jeden Entwickler zu erfordern, liefern Sie die `sandbox`-Schlüssel über [verwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms), entweder als Datei, die von Ihrem MDM verwaltet wird, oder über [server-verwaltete Einstellungen](/docs/de/server-managed-settings) auf claude.ai.

Die folgende Konfiguration verwalteter Einstellungen aktiviert die Sandbox, weigert sich, Claude Code zu starten, wenn die Sandbox nicht initialisiert werden kann, und verhindert, dass das Modell Befehle außerhalb der Sandbox erneut versucht:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

Die beiden Schlüssel über `enabled` hinaus steuern, was passiert, wenn die Sandbox einen Befehl nicht ausführen kann:

* **`failIfUnavailable`**: Eine fehlende Abhängigkeit wie bubblewrap auf Linux blockiert Claude Code vom Start, anstatt eine Warnung anzuzeigen und auf unsandboxed-Ausführung zurückzufallen
* **`allowUnsandboxedCommands: false`**: Claude Code ignoriert die `dangerouslyDisableSandbox`-Fluchtluke, daher können Befehle, die unter der Sandbox fehlschlagen, nicht außerhalb davon erneut versucht werden

Zwei Ergänzungen sind erwägenswert. Fügen Sie `excludedCommands` für alle von der Organisation genehmigten Tools hinzu, die ohne Isolation ausgeführt werden müssen. Fügen Sie [`sandbox.credentials`](#protect-credentials)-Einträge für Anmeldedaten-Verzeichnisse wie `~/.aws` und `~/.ssh` und für geheime Umgebungsvariablen hinzu, da die Standard-Lesrichtlinie diese immer noch zulässt.

Diese Konfiguration sandboxed die Befehle, die Claude ausführt. Ein Entwickler kann immer noch einen Befehl an der [`!`-Shell-Modus-Eingabeaufforderung](/docs/de/interactive-mode#shell-mode-with-prefix) eingeben und ihn außerhalb der Sandbox ausführen, mit dem gleichen Zugriff, den er bereits in jedem Terminal außerhalb von Claude Code hat. Siehe [Die unsandboxed-Wiederholungs-Fluchtluke](#the-unsandboxed-retry-escape-hatch) für die Sitzungen, in denen eingegebene Befehle sandboxed ausgeführt werden.

Die Sandbox läuft nicht auf nativem Windows, daher müssen Sie diese Konfiguration auf macOS und Linux beschränken oder diese Benutzer Claude Code in WSL2 oder einem Container ausführen lassen, wenn Ihre Flotte Windows-Hosts enthält.

<h3 id="keep-developers-from-widening-the-policy">
  Verhindern Sie, dass Entwickler die Richtlinie erweitern
</h3>

Für boolesche Schlüssel wie `enabled` und `failIfUnavailable` verwendet Claude Code den verwalteten Wert und ignoriert alles, das ein Entwickler lokal setzt. Für Array-Schlüssel wie `excludedCommands` und `allowRead` führt Claude Code Einträge aus jedem Scope zusammen, daher kann ein Entwickler Einträge anhängen, die die Richtlinie erweitern.

Setzen Sie `allowManagedReadPathsOnly` auf `true` in verwalteten Einstellungen, damit nur `allowRead`-Einträge aus verwalteten Einstellungen berücksichtigt werden. Dies verhindert, dass Entwickler den Lesezugriff über die von der Organisation genehmigten Pfade hinaus erweitern. Um Netzwerk-Domains auf die gleiche Weise auf die verwalteten Werte zu sperren, setzen Sie [`allowManagedDomainsOnly`](/docs/de/settings-reference#sandbox-network-allowmanageddomainsonly).

Wenn verwaltete Einstellungen `sandbox.filesystem` konfigurieren oder einen beliebigen `sandbox.credentials.files`-Eintrag mit `"mode": "deny"` auflisten, können nur verwaltete Einstellungen [`filesystem.disabled`](#disable-filesystem-isolation) setzen, daher können Entwickler von Administratoren bereitgestellte Filesystem-Einschränkungen nicht ausschalten. Ob ein `mask`-Eintrag den Schlüssel fixiert, hängt davon ab, wie er sich auflöst; die Tabelle unter [Welche Einstellungen können es deaktivieren](#which-settings-can-disable-it) behandelt die vier Fälle.

`excludedCommands` hat keine äquivalente verwaltete Sperrung, daher kann ein Entwickler immer Einträge anhängen, die zusätzliche Befehle außerhalb der Sandbox ausführen. Halten Sie die verwaltete Liste eng.

<h3 id="custom-proxy-configuration">
  Benutzerdefinierte Proxy-Konfiguration
</h3>

Für Organisationen, die erweiterte Netzwerk-Sicherheit erfordern, können Sie einen benutzerdefinierten Proxy implementieren, um:

* HTTPS-Datenverkehr zu entschlüsseln und zu inspizieren
* Benutzerdefinierte Filterregeln anzuwenden
* Alle Netzwerk-Anfragen zu protokollieren
* Mit bestehender Sicherheitsinfrastruktur zu integrieren

Um Claude Code auf Ihren Proxy zu verweisen, setzen Sie die Proxy-Ports in [Sandbox-Einstellungen](/docs/de/settings-reference#sandbox-settings):

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Einige Befehle schlagen in der Sandbox fehl, obwohl sie außerhalb funktionieren. Die folgenden Lösungen decken die häufigsten Fälle ab.

* **Befehle schlagen mit einem host-not-allowed-Fehler fehl**: Viele CLI-Tools müssen bestimmte Hosts erreichen. Wenn Sie die Berechtigung bei der Aufforderung erteilen, wird der Host zu Ihrer Zulassungsliste hinzugefügt, damit das Tool in Zukunft in der Sandbox ausgeführt wird.
* **`jest` hängt oder schlägt fehl**: `watchman` ist nicht kompatibel mit der Sandbox. Führen Sie stattdessen `jest --no-watchman` aus.
* **Go-basierte CLIs schlagen bei der TLS-Verifizierung auf macOS fehl**: Tools wie `gh`, `gcloud` und `terraform` können unter Seatbelt bei der TLS-Verifizierung fehlschlagen. Listen Sie diese Tools in [`excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) auf. Wenn Sie `httpProxyPort` mit einem MITM-Proxy und einer benutzerdefinierten CA verwenden, setzen Sie stattdessen [`enableWeakerNetworkIsolation`](/docs/de/settings-reference#sandbox-enableweakernetworkisolation) auf `true`.
* **`open`, `osascript` oder browserbasierte Authentifizierungsabläufe schlagen mit Fehler `-600` auf macOS fehl**: Die Sandbox blockiert Apple Events standardmäßig. Setzen Sie [`allowAppleEvents`](/docs/de/settings-reference#sandbox-allowappleevents) in Ihren Benutzer-, verwalteten oder CLI-Einstellungen auf `true`, um diese zuzulassen. Projekteinstellungen werden für diesen Schlüssel ignoriert. Das Aktivieren entfernt die Code-Ausführungsisolation, da sandboxed-Befehle dann andere Anwendungen ohne Sandbox ohne Benutzereingabeaufforderung starten können und AppleScript-Befehle an laufende Anwendungen senden können, unterliegen jedoch der macOS-Automatisierungszustimmungsaufforderung (TCC). Alternativ können Sie den Befehl zu [`excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) hinzufügen.
* **`docker`-Befehle schlagen fehl**: `docker` ist nicht kompatibel mit der Sandbox. Fügen Sie `docker *` zu [`excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) hinzu.
* **`pbcopy`, `xclip` oder `wl-copy` aktualisiert die Zwischenablage nicht**: Diese Zwischenablage-Dienstprogramme können aus der Sandbox heraus die Systemzwischenablage nicht erreichen, in welchem Fall der Text, der an sie weitergeleitet wird, nicht ankommt.

  Um Claudes Ausgabe in Ihre Zwischenablage zu kopieren, bitten Sie Claude, sie in seiner Antwort auszudrucken, und führen Sie dann [`/copy`](/docs/de/commands) aus. `/copy` schreibt aus dem Claude Code-Prozess in die Zwischenablage, nicht aus einem sandboxed-Befehl.

  Wenn Claude Text an eines dieser Tools weiterleitet, führt das Hinzufügen des Tools zu [`excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) diesen Aufruf nicht automatisch aus der Sandbox heraus.
* **Ein git-Befehl schlägt mit `unable to unlink old` fehl**: `git merge`, `git checkout` und ähnliche Befehle schlagen auf diese Weise fehl, wenn sie eine Datei ersetzen müssen, in die die Sandbox Schreibvorgänge verweigert, unabhängig davon, ob sich diese Datei unter einem [geschützten Pfad](#protected-paths) wie `.claude/skills` befindet, unter einem Ihrer `denyWrite`-Einträge oder außerhalb der Verzeichnisse, in die die Sandbox Befehle überhaupt schreiben lässt. Unter Linux und WSL2 endet der Fehler mit `Read-only file system`.

  Nach dem Fehler kann Claude [anbieten, den Befehl außerhalb der Sandbox erneut auszuführen](#the-unsandboxed-retry-escape-hatch); genehmigen Sie diesen erneuten Versuch, oder führen Sie den git-Befehl selbst in einem anderen Terminal aus. Wenn Sie `allowUnsandboxedCommands` auf `false` gesetzt haben, kann Claude den erneuten Versuch nicht anbieten, also führen Sie den Befehl selbst aus. Wenn derselbe git-Befehl häufig fehlschlägt, fügen Sie ihn zu [`excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) hinzu.
* **Bubblewrap schlägt fehl, um in einem Container zu starten**: In einem unprivilegierten Container kann bubblewrap kein neues `/proc`-Dateisystem bereitstellen, daher schlagen sandboxed-Befehle mit einem `bwrap`-Fehler wie `Can't mount proc on /newroot/proc: Operation not permitted` fehl. Setzen Sie [`enableWeakerNestedSandbox`](/docs/de/settings-reference#sandbox-enableweakernestedsandbox) auf `true`, damit die innere Sandbox das vorhandene `/proc` des Containers stattdessen bind-mounted. Verwenden Sie diese Einstellung nur, wenn der äußere Container bereits die Isolationsgrenze bietet, die Sie benötigen, da sie Prozessinformationen für sandboxed-Befehle verfügbar macht, die eine neue `/proc`-Bereitstellung verbergen würde.
* **0-Byte-Dateien mit Schreibschutz erscheinen in `.claude`-Einstellungspfaden, und „Ja, und nicht mehr fragen" wird nicht gespeichert**: Unter Linux und WSL2 hält die Sandbox einen Schreibverweigerung auf einer Datei, die noch nicht existiert, indem sie dort einen 0-Byte-Platzhalter mit Schreibschutz erstellt, während ein sandboxed-Befehl ausgeführt wird. Die Sandbox entfernt den Platzhalter danach. Wenn eine Sitzung vor dieser Bereinigung beendet wird, beispielsweise durch SIGKILL, bleiben die Platzhalter bestehen. Spätere Sitzungen binden sie bei jedem Start erneut schreibgeschützt, daher schlägt ein Einstellungsschreibvorgang wie das Speichern einer Berechtigungswahl fehl, wenn einer vorhanden ist.

  Führen Sie `claude doctor` aus, um die verbleibenden Platzhalter-Dateien aufzulisten. Die Warnung [`Stale sandbox mask files left by a killed session`](/docs/de/errors#stale-sandbox-mask-files-left-by-a-killed-session) nennt bis zu drei davon und zählt den Rest. Löschen Sie jede Datei mit `rm`, während keine andere Claude Code-Sitzung in diesem Projekt ausgeführt wird. Vor v2.1.257 ließ Claude Code dieselben Platzhalter ohne Kennzeichnung zurück.
* **`--dangerously-skip-permissions` schlägt als root fehl**: Dieses Flag wird blockiert, wenn es als root oder über sudo unter Linux und macOS ausgeführt wird, da root-Zugriff kombiniert mit keinen Berechtigungsaufforderungen jede Datei oder jeden Dienst auf dem System ändern kann. Die Überprüfung wird automatisch in einer erkannten Sandbox übersprungen. Um autonom in einem Container zu laufen, verwenden Sie die [dev container](/docs/de/devcontainer)-Konfiguration, die Claude Code als Nicht-Root-Benutzer ausführt.

<h2 id="limitations">
  Einschränkungen
</h2>

Sandboxing reduziert das Risiko, ist aber keine vollständige Isolationsgrenze. Überprüfen Sie die folgenden Einschränkungen, bevor Sie sich darauf als Hard-Sicherheitskontrolle verlassen.

<h3 id="security-limitations">
  Sicherheitsbeschränkungen
</h3>

* **Netzwerk-Filterung**: Die Sandbox schränkt ein, mit welchen Domains Prozesse sich verbinden können. Standardmäßig beendet oder inspiziert der integrierte Proxy TLS auf ausgehenden Datenverkehr nicht, daher werden die Inhalte verschlüsselter Verbindungen nicht untersucht. Die experimentelle Einstellung [`network.tlsTerminate`](/docs/de/settings-reference#sandbox-network-tlsterminate) beendet TLS am Proxy für [`mask`-Anmeldedaten-Substitution](#mask-credentials), fügt aber keine Inhaltsfilterung hinzu. Sie sind verantwortlich dafür, dass nur vertrauenswürdige Domains in Ihrer Richtlinie zulässig sind.

<Warning>
  Das Zulassen breiter Domains wie `github.com` kann Pfade für Datenexfiltration schaffen. Da der Proxy seine Zulassungsentscheidung vom Client-bereitgestellten Hostnamen trifft, ohne TLS zu inspizieren, kann Code, der in der Sandbox ausgeführt wird, möglicherweise [Domain Fronting](https://en.wikipedia.org/wiki/Domain_fronting) oder ähnliche Techniken verwenden, um Hosts außerhalb der Zulassungsliste zu erreichen. Wenn Ihr Bedrohungsmodell stärkere Garantien erfordert, konfigurieren Sie einen [benutzerdefinierten Proxy](#custom-proxy-configuration), der TLS beendet und Datenverkehr inspiziert, und installieren Sie sein CA-Zertifikat in der Sandbox. Stärkere TLS-bewusste Netzwerk-Isolation ist ein aktives Entwicklungsgebiet.
</Warning>

* **Privilege Escalation über Unix-Sockets**: Die Konfiguration `allowUnixSockets` kann versehentlich Zugriff auf System-Services gewähren, die zu Sandbox-Umgehungen führen könnten. Wenn Sie beispielsweise Zugriff auf `/var/run/docker.sock` zulassen, würde dies effektiv Zugriff auf das Host-System durch den Docker-Socket gewähren. Überdenken Sie sorgfältig alle Unix-Sockets, die Sie durch die Sandbox zulassen.
* **Dateisystem-Genehmigungseskalation**: Übermäßig breite Dateisystem-Schreibgenehmigungen können Privilege-Escalation-Angriffe ermöglichen. Das Zulassen von Schreibvorgängen zu Verzeichnissen, die ausführbare Dateien in `$PATH`, System-Konfigurationsverzeichnisse oder Benutzer-Shell-Konfigurationsdateien wie `.bashrc` oder `.zshrc` enthalten, kann zu Code-Ausführung in verschiedenen Sicherheitskontexten führen, wenn andere Benutzer oder System-Prozesse auf diese Dateien zugreifen.
* **Linux-Sandbox-Stärke**: Die Linux-Implementierung bietet starke Dateisystem- und Netzwerk-Isolation, enthält aber einen `enableWeakerNestedSandbox`-Modus, der es ermöglicht, in Docker-Umgebungen ohne privilegierte Namespaces zu funktionieren, oder auf Linux-Hosts, wo unprivilegierte Benutzer-Namespaces durch sysctl deaktiviert sind. Diese Option schwächt die Sicherheit erheblich ab und sollte nur verwendet werden, wenn zusätzliche Isolation anderweitig durchgesetzt wird.
* **Apple Events auf macOS**: Die macOS-Sandbox blockiert Apple Events standardmäßig. Die Einstellung `allowAppleEvents` hebt diese Einschränkung auf, damit Tools wie `open` und `osascript` funktionieren, aber es entfernt Code-Ausführungs-Isolation: Sandbox-Befehle können andere Anwendungen ohne Sandbox ohne Benutzer-Eingabeaufforderung starten und können AppleScript-Befehle an laufende Anwendungen senden, vorbehaltlich der Pro-App-macOS-Automatisierungs-Zustimmungsaufforderung (TCC). Es wird nur von Benutzer-, verwalteten oder CLI-Einstellungen berücksichtigt. Projekteinstellungen können es nicht aktivieren.

<h3 id="platform-and-tool-compatibility">
  Plattform- und Tool-Kompatibilität
</h3>

* **Plattform-Unterstützung**: Unterstützt macOS, Linux und WSL2. WSL1 und native Windows werden nicht unterstützt.
* **Performance-Overhead**: Minimal, aber einige Dateisystem-Operationen können leicht langsamer sein.
* **Tool-Kompatibilität**: Einige Tools, die spezifische System-Zugriffsmuster erfordern, benötigen möglicherweise Konfigurationsanpassungen oder müssen möglicherweise außerhalb der Sandbox ausgeführt werden.

<h3 id="scope">
  Umfang
</h3>

Die Sandbox isoliert Bash-Subprozesse. Andere Tools funktionieren unter verschiedenen Grenzen:

* **Integrierte Datei-Tools**: Read, Edit und Write verwenden das Genehmigungssystem direkt, anstatt durch die Sandbox zu laufen. Siehe [Genehmigungen](/docs/de/permissions).
* **Computer-Nutzung**: Wenn Claude Apps öffnet und Ihren Bildschirm steuert, läuft es auf Ihrem tatsächlichen Desktop, anstatt in einer isolierten Umgebung. Pro-App-Genehmigungseingaben kontrollieren jede Anwendung. Siehe [Computer-Nutzung in der CLI](/docs/de/computer-use) oder [Computer-Nutzung auf Desktop](/docs/de/desktop#let-claude-use-your-computer).
* **Umgebungsvariablen**: Sandbox-Bash-Befehle erben die Umgebung des übergeordneten Prozesses standardmäßig, einschließlich aller dort gesetzten Anmeldedaten. Verwenden Sie [`sandbox.credentials`](#protect-credentials), um spezifische Variablen für Sandbox-Befehle zu deaktivieren oder zu maskieren, oder setzen Sie [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/de/env-vars), um Anmeldedaten aus allen Subprozessen zu entfernen.
* **Subagenten**: [Subagenten](/docs/de/sub-agents) laufen im gleichen Prozess wie die übergeordnete Sitzung und verwenden die gleiche Sandbox-Konfiguration. Bash-Befehle in einem Subagenten werden in der Sandbox ausgeführt, wenn Sandboxing in der übergeordneten Sitzung aktiviert ist.

<Warning>
  Effektives Sandboxing erfordert sowohl Dateisystem- als auch Netzwerk-Isolation. Ohne Netzwerk-Isolation könnte ein kompromittierter Agent sensible Dateien wie SSH-Schlüssel exfiltrieren. Ohne Dateisystem-Isolation, ob durch eine permissive Richtlinie oder durch [Deaktivierung der Dateisystem-Schicht](#disable-filesystem-isolation), könnte ein kompromittierter Agent System-Ressourcen manipulieren, um Netzwerkzugriff zu erlangen. Wenn Sie die Standardwerte erweitern, überprüfen Sie, dass ein `allowWrite`-Pfad, ein breiter `allowedDomains`-Eintrag oder eine `excludedCommands`-Ausnahme keine Einschränkung auf der anderen Seite rückgängig macht.
</Warning>

<h2 id="see-also">
  Siehe auch
</h2>

* [Sandbox-Umgebungen](/docs/de/sandbox-environments): Vergleichen Sie die integrierte Sandbox mit Dev-Containern, Containern und VMs
* [Sicherheit](/docs/de/security): Umfassende Sicherheitsfeatures und Best Practices
* [Genehmigungen](/docs/de/permissions): Genehmigungskonfiguration und Zugriffskontrolle
* [Alle Einstellungen](/docs/de/settings-reference): Jeder Einstellungsschlüssel
* [CLI-Referenz](/docs/de/cli-reference): Befehlszeilenoptionen
