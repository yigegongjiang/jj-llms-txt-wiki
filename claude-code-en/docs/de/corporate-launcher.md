> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code hinter einem Corporate Launcher ausführen

> Leiten Sie die Prozesse, die Claude Code von seiner eigenen Binärdatei aus startet, einschließlich des Hintergrunddienstes und jeder Agent-View-Sitzung, durch einen erforderlichen Launcher mit CLAUDE_CODE_PROCESS_WRAPPER oder der processWrapper-Einstellung.

Einige Organisationen erfordern, dass jeder Prozess auf einer Workstation durch einen obligatorischen Launcher gestartet wird. Der Launcher wendet die Sandbox, Netzwerkkontrollen oder Credential-Injection an, auf die die Sicherheitsposition des Unternehmens angewiesen ist, und eine Binärdatei, die ohne diese startet, ist ein Verstoß gegen die Richtlinie.

`CLAUDE_CODE_PROCESS_WRAPPER` startet jeden Prozess, den Claude Code von seiner eigenen Binärdatei aus startet, durch Ihren Launcher: den Hintergrund­dienst, jede Sitzung, die er in [Agent View](/docs/de/agent-view) hostet, und Claude Codes Neustarts nach einem Update. Setzen Sie es auf den absoluten Pfad Ihres Launchers, und Claude Code führt den Launcher mit dem Claude-Code-Befehl als seine Argumente aus.

Ein Launcher, der den `claude`-Befehl auf Ihrem `PATH` umhüllt, kann diese Prozesse nicht erreichen, da sie vom direkten Pfad der Binärdatei aus starten, ohne `claude` nachzuschlagen.

<Note>
  `CLAUDE_CODE_PROCESS_WRAPPER` erfordert Claude Code v2.1.208 oder später. Frühere Versionen ignorieren die Variable und starten jeden Prozess ohne Umhüllung. Die entsprechende [`processWrapper`-Einstellung](/docs/de/settings-reference#processwrapper) erfordert v2.1.210 oder später. Frühere Versionen ignorieren sie als unbekannten Schlüssel, wenden keinen Launcher an und melden keinen Fehler.

  Verwenden Sie nach der Bereitstellung einer der beiden Formen den [Verify-Schritt](#set-up-the-launcher), um zu bestätigen, dass die ausgeführte Version sie anwendet.
</Note>

<h2 id="what-the-launcher-covers">
  Was der Launcher abdeckt
</h2>

Mit `CLAUDE_CODE_PROCESS_WRAPPER` gesetzt startet Claude Code jeden der folgenden Prozesse durch Ihren Launcher:

* Der Hintergrund­dienst, den `claude agents` und Hintergrund­sitzungen bei Bedarf starten.
* Der Terminal-Host und die Claude-Code-Sitzung in jeder Agent-View-Zeile, einschließlich der Warm-Standby-Sitzungen, die der Dienst bereit hält.
* Sitzungen, die der Dienst nach einem Update oder Absturz neu startet.
* Der Neustart, den Claude Code durchführt, um die Installation eines Updates abzuschließen, einschließlich der Restart-for-Update-Aktion von Agent View.
* Die Sitzungsprozesse, die [Remote Control](/docs/de/remote-control) startet. Erfordert Claude Code v2.1.210 oder später.
* Die Split-Pane-Teammate-Sitzungen, die [Agent Teams](/docs/de/agent-teams) in tmux oder iTerm2 starten. Teammate-Panes sind interaktiv statt Hintergrund­prozesse, aber Claude Code startet sie aus seiner eigenen Binärdatei, daher deckt der Launcher sie ab. Erfordert Claude Code v2.1.210 oder später.

Unter Windows wird die Variable ignoriert: Der Launcher-Vertrag hängt von `exec` ab, das Windows nicht unterstützt. Ein Windows-Computer mit der gesetzten Variable führt jeden Prozess unwrapped aus und funktioniert weiterhin, und das einzige Signal ist eine Warnung im [Debug-Protokoll](/docs/de/troubleshooting). Wenn Ihre Launcher-Richtlinie Windows abdeckt, erfüllt die Variable sie dort nicht: Zählen Sie Windows-Computer als unwrapped, wenn Sie den Rollout planen.

<h3 id="processes-that-start-outside-the-launcher">
  Prozesse, die außerhalb des Launchers starten
</h3>

Die folgenden Prozesse starten nicht durch den Launcher:

* Ein [installierter Hintergrund­dienst](/docs/de/agent-view#the-supervisor-process), dessen Unit vor der Launcher-Konfiguration geschrieben wurde: `launchd` oder `systemd` startet diesen Prozess aus seiner Unit-Datei. `/status` und `claude daemon status` warnen, wenn der laufende Dienst und der konfigurierte Launcher nicht übereinstimmen, und die Sitzungen, die der Dienst startet, starten immer noch durch den Launcher, sobald der Dienst mit der Variable in seinen Einstellungen neu startet.
* Eine Sitzung, die Sie selbst in einem Terminal starten, die so ausgeführt wird, wie Sie sie aufgerufen haben. Um diese Sitzungen abzudecken, legen Sie ein Skript namens `claude` in ein Verzeichnis früher auf `PATH`, das Ihren Launcher mit der echten Binärdatei ausführt; ersetzen Sie nicht den verwalteten Symlink. Der Hintergrund­dienst und seine Sitzungen starten ohne `PATH`-Lookup, daher stapeln sich die beiden Launcher dort nicht.
* Der erste Prozess eines `claude-cli://` Deep Links, den der Protokoll-Handler des Betriebssystems direkt startet. Alles, das diese Sitzung danach im Hintergrund startet, läuft durch den Launcher. Um diesen Pfad vollständig zu schließen, [verhindern Sie die Handler-Registrierung](/docs/de/deep-links#registration-and-supported-platforms) mit der Einstellung `disableDeepLinkRegistration`.
* Der Neustart, den `--worktree` kombiniert mit `--tmux` durchführt: Der Terminal-Multiplexer startet diesen Pane, nicht die Binärdatei von Claude Code.
* Der Native-Messaging-Host, den [Claude in Chrome](/docs/de/chrome) registriert: Der Browser startet ihn, nicht die Binärdatei von Claude Code.

<h3 id="helper-process-names-in-process-monitors">
  Namen von Hilfsprozessen in Prozessmonitoren
</h3>

Mit einem konfigurierten Launcher zeigen `ps` und Activity Monitor keine Claude-Code-Labels `claude bg-pty-host` und `claude bg-spare` mehr für die Hintergrund-Hilfsprozesse an, da der `exec` des Launchers die Argumentliste neu erstellt. Das Verlieren der Labels ist ein Nebeneffekt, keine Verschleierung: Die Prozesse sind ansonsten unverändert, und Claude Code identifiziert seine eigenen Prozesse nach Binärpfad, niemals nach Anzeigename.

<h2 id="set-up-the-launcher">
  Richten Sie den Launcher ein
</h2>

<Steps>
  <Step title="Schreiben Sie das Launcher-Skript">
    Erstellen Sie ein ausführbares Skript unter einem absoluten Pfad, z. B. `/opt/corp/launcher`. Claude Code führt es mit dem vollständigen Claude-Code-Befehl als seine Argumente aus, und das Skript muss mit `exec "$@"` enden, damit es sich selbst durch Claude Code ersetzt:

    ```bash theme={null}
    #!/bin/sh
    # Einrichtung Ihrer Organisation: Geben Sie die Sandbox ein, wenden Sie
    # Netzwerkkontrollen an oder injizieren Sie Anmeldedaten.
    exec "$@"
    ```

    Machen Sie es mit `chmod +x` ausführbar. Der Einrichtungsteil ist das, was Ihr Launcher tun muss, bevor Claude Code ausgeführt wird; [der Launcher-Vertrag](#the-launcher-contract) unten listet die Regeln auf, die das Skript befolgen muss.

    <Note>
      Wenn Sie zuvor den Symlink `~/.local/bin/claude` durch Ihren Launcher ersetzt haben, stellen Sie den ursprünglichen Symlink in derselben Änderung wieder her. Ein ersetzter Symlink führt dazu, dass die erste umhüllte Sitzung den Hintergrund­dienst gleichzeitig durch beide Launcher startet, und es versetzt die Installation in einen extern verwalteten Zustand: `/doctor` meldet dies, Auto-Update lässt die Datei an Ort und Stelle, und die Bereinigung alter Versionen bleibt deaktiviert, bis das Installationsprogramm diesen Pfad erneut verwaltet.
    </Note>
  </Step>

  <Step title="Setzen Sie CLAUDE_CODE_PROCESS_WRAPPER in den Einstellungen">
    Setzen Sie die Variable im `env`-Block einer Einstellungsdatei, damit der abgelöste Hintergrund­dienst sie erbt. Ein Shell-`export` reicht nicht aus: Der Hintergrund­dienst startet bei Bedarf, überlebt Ihre Shell und liest Shell-Profile nie erneut.

    Für einen Computer fügen Sie es zu `~/.claude/settings.json` hinzu. Um es auf jedem Computer in Ihrer Organisation bereitzustellen, legen Sie denselben Block in [verwaltete Einstellungen](/docs/de/managed-settings):

    ```json theme={null}
    {
      "env": {
        "CLAUDE_CODE_PROCESS_WRAPPER": "/opt/corp/launcher"
      }
    }
    ```

    Wenn mehr als eine Quelle die Variable setzt, überschreibt der Wert der verwalteten Einstellungen sowohl `~/.claude/settings.json` als auch einen in der Shell exportierten Wert, sodass Benutzer Self-Spawns nicht auf einen anderen Launcher verweisen können.

    Die [`processWrapper`-Einstellung](/docs/de/settings-reference#processwrapper) trägt denselben Wert wie ein benannter Einstellungsschlüssel auf oberster Ebene. Setzen Sie ihn, wenn Ihre Organisation Einstellungen als einzelne Schlüssel statt als `env`-Block bereitstellt. Die `processWrapper`-Einstellung erfordert Claude Code v2.1.210 oder später. Die folgende Einstellungsdatei setzt denselben Launcher über den Schlüssel:

    ```json theme={null}
    {
      "processWrapper": "/opt/corp/launcher"
    }
    ```

    `CLAUDE_CODE_PROCESS_WRAPPER` hat Vorrang, wenn beide gesetzt sind.

    Da `processWrapper` eine benannte Einstellung ist, sieht eine Organisation, die sie über [remote verwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms) bereitstellt, sie auf dem [Sicherheitsgenehmigungsdialog](/docs/de/server-managed-settings#security-approval-dialogs) neben den anderen Einstellungen aufgelistet, die von Administratoren bereitgestellte ausführbare Dateien ausführen.

    Projekt- und lokale Einstellungen können den Launcher nicht konfigurieren. Eine Datei, die in ein Repository committed wird, darf nicht in der Lage sein, eine Binärdatei vor jedem Claude-Code-Prozess auf dem Computer zu platzieren, daher ignoriert Claude Code `CLAUDE_CODE_PROCESS_WRAPPER` in `.claude/settings.json` oder `.claude/settings.local.json` mit einer Warnung im [Debug-Protokoll](/docs/de/troubleshooting), und liest den `processWrapper`-Schlüssel nie aus diesen Dateien.
  </Step>

  <Step title="Starten Sie den Hintergrund­dienst und Ihre Sitzungen neu">
    Ein laufender Hintergrund­dienst und alle offenen `claude`-Sitzungen lesen die Variable einmal beim Start, daher starten sie weiterhin Prozesse ohne Umhüllung, bis sie neu gestartet werden. Führen Sie `claude daemon stop --any` aus, um den On-Demand-Dienst zu stoppen; der nächste Befehl, der ihn benötigt, z. B. `claude agents`, startet einen umhüllten. Ein [installierter Dienst](/docs/de/agent-view#the-supervisor-process) nimmt `claude daemon stop` ohne `--any`. Starten Sie dann Ihre offenen `claude`-Sitzungen neu.

    Auf Computern, die Sie nicht von Hand neu starten können, zieht die erste Sitzung, die nach dem Einstellungs-Push gestartet wird, automatisch einen verbleibenden On-Demand-Dienst ohne Umhüllung zurück. Ein Computer, auf dem keine neue Sitzung startet, behält seinen Dienst ohne Umhüllung, bis eine startet, und ein installierter Dienst benötigt immer den Neustart in diesem Schritt.
  </Step>

  <Step title="Überprüfen">
    Führen Sie `/status` in einer Sitzung aus: Der Self-exec-Eintrag zeigt den aufgelösten Startbefehl und warnt, wenn der laufende Hintergrund­dienst nicht damit übereinstimmt. `claude daemon status` gibt dieselben Informationen aus der Shell aus, auch nachdem Sie die Variable aufgehoben haben, wenn `/status` den Eintrag nicht mehr anzeigt.
  </Step>
</Steps>

<h2 id="the-launcher-contract">
  Der Launcher-Vertrag
</h2>

Wenn der Launcher nicht ausgeführt werden kann, weigert sich Claude Code, den Prozess zu starten, anstatt ihn ohne Umhüllung zu starten. Unter Windows wird [die Variable ignoriert](#what-the-launcher-covers) und Prozesse starten ohne Umhüllung. Claude Code hält das Skript an diese Regeln:

* **Enden Sie mit `exec "$@"`**. Ein Launcher, der ein Kind forkt und beendet, hinterlässt einen verwaisten Claude-Code-Prozess, den der Hintergrund­dienst nicht verfolgen kann. Agent View markiert eine solche Sitzung als fehlgeschlagen mit einer Nachricht, die den Launcher benennt, und der Dienst räumt auf, was der Launcher hinterlassen hat.
* **Ordnen Sie Argumente nicht neu an, absorbieren oder stellen Sie ihnen voran**. Das erste Argument ist die Claude-Code-Binärdatei und alles danach ist sein argv.
* **Geben Sie jede geerbte Umgebungsvariable an `exec` weiter**. Das Hinzufügen von Variablen, z. B. injizierte Anmeldedaten, ist in Ordnung; das Löschen geerbter ist nicht.
  * Die Pro-Sitzungs-Authentifizierungstoken, die Modell- und Anbieterauswahl und `CLAUDE_CODE_PROCESS_WRAPPER` selbst reisen alle in der geerbten Umgebung, daher bricht ein Launcher, der sie aus einer Zulassungsliste neu erstellt, die Sitzungen, die er startet, und `/status` meldet eine Launcher-Nichtübereinstimmung.
  * Wenn der Launcher einen Namespace oder eine Sandbox betreten muss, die die Umgebung zurückgesetzt, exportieren Sie die geerbte Umgebung darin wörtlich erneut.
* **Erreichen Sie `exec` innerhalb von etwa drei Sekunden jedes Mal, wenn der Launcher ausgeführt wird**. Ein kalter Hintergrund-Dispatch führt den Launcher zweimal hintereinander aus, bevor das erste Byte der Ausgabe, daher führen Sie langsame Arbeiten wie einen Single-Sign-On-Austausch träge oder aus einem Cache durch.
* **Tolerieren Sie, dass Sie von innen aufgerufen werden**. Claude Code wendet den Launcher auf jeden verschachtelten Self-Spawn an, daher muss ein Launcher, der eine exklusive Ressource erwirbt, erkennen, dass er sie bereits hält.
* **Schreiben Sie nicht auf das Terminal, bevor Claude Code startet**. Alles, das vor dem `exec` gedruckt wird, wird als Absturzursache gemeldet, wenn die Sitzung vor der Initialisierung stirbt.

<h3 id="format-of-the-launcher-value">
  Format des Launcher-Wertes
</h3>

`CLAUDE_CODE_PROCESS_WRAPPER` und die `processWrapper`-Einstellung verwenden das gleiche Format. Für die meisten Launcher ist der Wert der absolute Pfad des Skripts, z. B. `/opt/corp/launcher`.

Um Ihrem Launcher eigene Argumente zu übergeben, schreiben Sie sie nach dem Pfad. Claude Code analysiert den Wert als Argumentliste, nicht als Shell-Befehl:

* Whitespace trennt Token, und doppelte Anführungszeichen gruppieren ein Token, das Leerzeichen enthält.
* Ein Wert, der mit `[` beginnt, wird als JSON-String-Array gelesen, z. B. `["/opt/corp/launcher", "--profile", "cc"]`.
* Shell-Syntax funktioniert nicht: Es gibt keine Variablenerweiterung oder Globbing, und ein unquotierter Operator wie `;`, `|`, `&` oder `$(` wird als Konfigurationsfehler abgelehnt, anstatt neu interpretiert zu werden.

Wenn der Wert nicht verwendet werden kann, weigert sich Claude Code, den betroffenen Prozess zu starten, und [meldet den Grund](/docs/de/errors#claude_code_process_wrapper-launcher-errors).

<h2 id="relationship-to-claude_code_shell_prefix">
  Beziehung zu `CLAUDE_CODE_SHELL_PREFIX`
</h2>

`CLAUDE_CODE_PROCESS_WRAPPER` umhüllt Claude Codes eigene Prozesse und übergibt den Befehl als separate argv-Token an den Launcher zum `exec`. [`CLAUDE_CODE_SHELL_PREFIX`](/docs/de/env-vars) umhüllt die Shell-Befehle, die Claude in Ihrem Namen ausführt, z. B. Bash-Tool-Aufrufe, Hooks und die Befehle, die stdio-MCP-Server starten, und übergibt jeden als einzelne Shell-zitierte Zeichenkette in `$1` an den Wrapper zur Neubewertung. Ein für einen geschriebener Launcher funktioniert nicht als der andere.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Agent View](/docs/de/agent-view): die Hintergrund­sitzungen und der Supervisor-Prozess, den der Launcher abdeckt
* [Umgebungsvariablen](/docs/de/env-vars): der `CLAUDE_CODE_PROCESS_WRAPPER`-Referenzeintrag
* [Verwaltete Einstellungen](/docs/de/permissions#managed-settings): Liefern Sie den `env`-Block über eine Flotte
* [Launcher-Fehlerreferenz](/docs/de/errors#claude_code_process_wrapper-launcher-errors): die Verweigerungsmeldungen und wie man sich erholt
