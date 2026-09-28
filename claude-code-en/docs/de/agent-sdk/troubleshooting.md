> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Fehlerbehebung im Agent SDK

> Beheben Sie Agent SDK-Fehler, wenn die Claude Code CLI nicht startet, der CLI-Prozess beendet wird oder ein erfolgreiches Ergebnis ohne strukturierte Ausgabe ankommt.

Diese Seite behandelt Agent SDK-Fehler beim CLI-Start, beim CLI-Prozessausstieg und bei strukturierten Ausgaben. Einträge auf dieser Seite sind nach dem Fehler, den Sie sehen, sortiert. Jeder Eintrag nennt die Ursache und was zu tun ist.

Symptome, die an ein Feature gebunden sind, wie z. B. ein Hook, der nicht ausgelöst wird, oder ein Skill, der nicht verwendet wird, haben einen Fehlerbehebungsabschnitt auf der Seite dieses Features. Die Tabelle nennt den Abschnitt oder die Seite, die jedes Symptom behandelt:

| Symptom                                                                                                                                                                                                                                                                                                                                      | Gehen Sie zu                                                                                                                           |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| Skills nicht gefunden, ein Skill wird nicht verwendet, Fehler `Invalid skill name`                                                                                                                                                                                                                                                           | [Skills-Fehlerbehebung](/docs/de/agent-sdk/skills#troubleshooting)                                                                          |
| MCP-Server zeigt Status `failed`, Tools werden nicht aufgerufen, Verbindungs-Timeouts, Tool-Ausgabe überschreitet die maximal zulässigen Token                                                                                                                                                                                               | [MCP-Fehlerbehebung](/docs/de/agent-sdk/mcp#troubleshooting)                                                                                |
| Plugin wird nicht geladen, Plugin-Skills werden nicht angezeigt                                                                                                                                                                                                                                                                              | [Plugins-Fehlerbehebung](/docs/de/agent-sdk/plugins#troubleshooting)                                                                        |
| Claude delegiert nicht an Subagenten, dateisystembasierte Agenten werden nicht geladen                                                                                                                                                                                                                                                       | [Subagenten-Fehlerbehebung](/docs/de/agent-sdk/subagents#troubleshooting)                                                                   |
| Checkpointing-Optionen nicht erkannt, Benutzernachrichten ohne UUIDs, `No file checkpoint found`, `File rewinding is not enabled`, `ProcessTransport is not ready for writing`                                                                                                                                                               | [Fehlerbehebung für Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing#troubleshooting)                                             |
| Hook wird nicht ausgelöst, Matcher filtert nicht wie erwartet, Hook-Timeout, Tool unerwartet blockiert, geänderte Eingabe nicht angewendet, Session-Hooks nicht verfügbar in Python, Subagenten-Berechtigungsaufforderungen vervielfachen sich, rekursive Hook-Schleifen mit Subagenten, `systemMessage` wird nicht in der Ausgabe angezeigt | [Beheben Sie häufige Probleme](/docs/de/agent-sdk/hooks#fix-common-issues) auf der Hooks-Seite                                              |
| Ein Agent, der auf Ihrem Computer funktioniert, schlägt in einem bereitgestellten Service oder Container fehl                                                                                                                                                                                                                                | [Fehlerbehebung bei Bereitstellungsfehlern](/docs/de/agent-sdk/hosting#troubleshoot-deployment-failures)                                    |
| `Not logged in`, `Invalid API key`, `API Error`, `429`, `There's an issue with the selected model`                                                                                                                                                                                                                                           | [Fehlerreferenz](/docs/de/errors#find-your-error)                                                                                           |
| `CLINotFoundError`, `CLIConnectionError`, `ProcessError`, `Claude Code process exited with code N`, `Claude Code returned an error result`, `structured_output` ist `None`                                                                                                                                                                   | [CLI-Start](#cli-startup), [CLI-Prozessausstieg](#cli-process-exit) und [Strukturierte Ausgaben](#structured-outputs) auf dieser Seite |

<h2 id="cli-startup">
  CLI-Start
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError: Claude Code nicht gefunden
</h3>

Das Python SDK startet die Claude Code CLI als Unterprozess. Wenn es keine `claude`-Ausführungsdatei finden kann, schlägt die Verbindung mit einem `CLINotFoundError` fehl:

```
Claude Code not found at: /your/configured/path
```

Die Meldung enthält den konfigurierten Pfad, wenn Sie `ClaudeAgentOptions(cli_path=...)` setzen und dieser auf eine fehlende Datei verweist. Ohne `cli_path` durchsucht das SDK Ihren `PATH` und häufige Installationsorte, und die Meldung enthält Installationsanweisungen für Ihre Plattform.

So beheben Sie das Problem:

* Installieren Sie Claude Code, falls es nicht installiert ist. Siehe [Claude Code installieren](/docs/de/setup#install-claude-code) für den Befehl auf Ihrer Plattform.
* Wenn Sie `cli_path` setzen, bestätigen Sie, dass die Datei existiert und die `claude`-Ausführungsdatei ist.
* Wenn Sie sich auf `PATH`-Auflösung verlassen, bestätigen Sie, dass `claude --version` in der gleichen Umgebung funktioniert, in der Ihre Anwendung läuft. Prozesse, die Sie außerhalb Ihrer Shell starten, z. B. von einer IDE oder einem Service Manager, laufen oft mit einem anderen `PATH`.

Das TypeScript SDK sucht die CLI in seinem gebündelten Plattformpaket und dem Pfad, den Sie in `pathToClaudeCodeExecutable` setzen. Passen Sie die Meldung an, die Sie sehen:

* `Native CLI binary for <platform>-<arch> not found`: Das gebündelte Plattformpaket fehlt, meistens weil die Installation optionale Abhängigkeiten übersprungen hat. Installieren Sie `@anthropic-ai/claude-agent-sdk` neu, ohne optionale Abhängigkeiten zu überspringen, oder verweisen Sie `pathToClaudeCodeExecutable` auf eine [native Installation](/docs/de/setup#install-claude-code). In einer einzelnen ausführbaren Datei, die mit `bun build --compile` erstellt wurde, hat die gleiche Meldung eine andere Ursache und Lösung. Siehe [In eine einzelne ausführbare Datei kompilieren](/docs/de/agent-sdk/typescript#compile-to-a-single-executable).
* `Claude Code native binary not found at <path>` oder `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`: Die Datei im aufgelösten Pfad fehlt, oder der Prozess kann nicht darauf zugreifen. Bestätigen Sie, dass die Datei in diesem Pfad existiert und dass der Prozess darauf zugreifen kann.

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError: Refusing to execute batch script
</h3>

Unter Windows schlägt die Verbindung mit einem `CLIConnectionError` fehl, wenn der CLI-Pfad, den das Python SDK verwendet, ein `.bat`- oder `.cmd`-Batch-Skript ist, einschließlich des `claude.cmd`-Shims, das eine npm-Installation erstellt:

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

Die Weigerung ist absichtliche Sicherheitshärtung, nicht eine fehlerhafte Installation. Windows führt Batch-Skripte aus, indem es den Spawn in einen `cmd.exe /c`-Aufruf umschreibt, und `cmd.exe` analysiert die gesamte Befehlszeile zur Ausführungszeit neu, sodass ein Argumentwert injizierte Befehle ausführen kann.

Die meisten Windows-Installationen erreichen diesen Fehler nie. Das Windows x64-Wheel von `claude-agent-sdk` enthält eine `claude.exe`, und das SDK bevorzugt die gebündelte CLI, dann jede native `claude.exe`, die es entdecken kann, bevor es auf einen Batch-Shim zurückfällt. Sie sehen die Weigerung in zwei Fällen:

* Sie setzen `ClaudeAgentOptions(cli_path=...)` auf eine `.bat`- oder `.cmd`-Datei, z. B. das `claude.cmd`-Shim von npm.
* Ihre Installation hat keine gebündelte oder native `claude.exe`, z. B. eine Quellinstallation auf ARM64 Windows, wo die einzige `claude` auf Ihrem `PATH` das npm-Shim ist.

Um das Problem zu beheben, geben Sie dem SDK eine native ausführbare Datei statt eines Batch-Skripts:

* Wenn Sie `ClaudeAgentOptions(cli_path=...)` setzen, verweisen Sie auf eine `claude.exe` oder entfernen Sie die Option. Das SDK überspringt die Erkennung, während `cli_path` gesetzt ist, sodass eine native Installation allein nicht wirksam werden kann.
* Installieren Sie Claude Code nativ in PowerShell: `irm https://claude.ai/install.ps1 | iex`
* Auf x64 Windows installieren Sie das `claude-agent-sdk`-Wheel, das `claude.exe` enthält.

Vor `claude-agent-sdk` 0.2.124 spawnten das Python SDK Batch-Skripte über `cmd.exe` ohne diese Überprüfung.

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError: Failed to start Claude Code
</h3>

Das SDK hat eine Datei im aufgelösten Pfad gefunden, konnte sie aber nicht starten. Python löst diese Fehler als `CLIConnectionError` aus. TypeScript lehnt die Nachrichteniteration mit einem Fehler ab, der keine SDK-Klasse trägt. Die folgende Tabelle ordnet jede Meldung dem zu, was sie Ihnen sagt. Passen Sie die Meldung an, die Sie sehen:

| Meldung                                                           | SDK        | Was es Ihnen sagt                                                                         |
| ----------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------- |
| `Failed to start Claude Code: <detail>`                           | Python     | Der Rest der Meldung ist der eigene Fehler des Betriebssystems                            |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | Das Skript im konfigurierten Pfad kann nicht ausgeführt werden                            |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | Die Binärdatei kann nicht ausgeführt werden, mit einem libc-Vorschlag am Ende der Meldung |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | Jeder andere Startfehler                                                                  |

In beiden SDKs ist die übliche Ursache ein aufgelöster Pfad, der auf etwas verweist, das nicht ausgeführt werden kann, z. B. eine Textdatei, ein Verzeichnis oder eine Datei ohne Ausführungsberechtigung. Lesen Sie den libc-Vorschlag der Meldung der nativen Binärdatei als eine mögliche Ursache.

Um das Problem in beiden SDKs zu beheben:

* Bestätigen Sie, dass der konfigurierte Pfad auf die `claude`-Ausführungsdatei selbst verweist und dass die Datei Ausführungsberechtigung hat.
* Wenn Sie keinen benutzerdefinierten Pfad benötigen, entfernen Sie `cli_path` in Python oder `pathToClaudeCodeExecutable` in TypeScript, damit das SDK eine CLI selbst findet und seine gebündelte Kopie bevorzugt.
* Wenn die fehlerhafte Binärdatei die gebündelte Kopie des SDK in einem Container-Image ist, installieren Sie das SDK während des Image-Builds neu, damit die gebündelte Binärdatei der Plattform des Containers entspricht, oder erstellen Sie das Image für die Architektur, auf der es läuft, neu. Die übliche Ursache ist eine Binärdatei, die nicht der Architektur oder libc des Containers entspricht, oder eine, die während des Image-Builds ihre Ausführungsberechtigung verloren hat.

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError: Not connected
</h3>

Das Aufrufen einer `ClaudeSDKClient`-Methode in Python, bevor der Client verbunden ist, oder nachdem er getrennt wurde, löst einen `CLIConnectionError` mit dieser Meldung aus:

```
Not connected. Call connect() first.
```

Tun Sie, was die Meldung sagt. Rufen Sie entweder `await client.connect()` vor jeder anderen Client-Methode auf, oder öffnen Sie den Client mit `async with ClaudeSDKClient() as client:`, was beim Eintritt verbindet.

<h2 id="cli-process-exit">
  CLI-Prozessbeendigung
</h2>

Die Einträge in diesem Abschnitt bedeuten, dass der Claude Code-Prozess beendet wurde, während Ihre Anwendung ihn verwendete. Welcher Fehler Sie sehen, hängt von der SDK-Sprache und davon ab, ob die CLI ein Fehlerergebnis gemeldet hat, bevor sie beendet wurde.

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError: Command failed with exit code
</h3>

Das Python SDK löst einen `ProcessError` aus, wenn der Claude Code-Prozess mit einem Nicht-Null-Code beendet wird:

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

Die Meldung gibt den Exit-Code zweimal an, und die `Error output`-Zeile ist fester Text statt der Fehlerausgabe Ihres Prozesses. Der gleiche feste Text füllt das `stderr`-Attribut der Ausnahme. Das `exit_code`-Attribut der Ausnahme trägt den Code. Um zu erfassen, was die CLI tatsächlich in stderr geschrieben hat, übergeben Sie einen `stderr`-Callback in `ClaudeAgentOptions` und protokollieren Sie, was er empfängt.

Ein bloßer `ProcessError` bedeutet, dass die CLI beendet wurde, ohne ein Fehlerergebnis zu melden. Wenn die CLI eines gemeldet hat, löst das SDK stattdessen [`ResultError`](/docs/de/agent-sdk/python#resulterror) aus, das in [Claude Code hat ein Fehlerergebnis zurückgegeben](#claude-code-returned-an-error-result) behandelt wird. `ResultError` ist eine Unterklasse von `ProcessError`, sodass `except ProcessError` beide erfasst. Um sie unterschiedlich zu behandeln, setzen Sie die `except ResultError`-Klausel zuerst.

Vor `claude-agent-sdk` 0.2.140 löste das Python SDK Fehler-Ergebnis-Exits als einfache `Exception` statt als `ResultError` aus.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

IDE-Wrapper drucken diese Meldung auch, und die [Fehlerreferenz](/docs/de/errors#claude-code-process-exited-with-code-n) behandelt sie für VS Code und andere Launcher. Dieser Eintrag behandelt, was Ihr TypeScript SDK-Code empfängt. Das SDK zeigt einen Nicht-Null-CLI-Exit als einfachen `Error` an, der die `for await`-Schleife über die Nachrichten von `query()` ablehnt. Es gibt keine SDK-Fehlerklasse zum Erfassen, daher wickeln Sie die Schleife in `try`/`catch` ein und passen Sie die Meldung an:

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

Wenn die CLI in stderr geschrieben hat, endet die Meldung mit dem Ende davon. Um den vollständigen Stream zu erfassen, übergeben Sie einen `stderr`-Callback in den Abfrageoptionen. Ein Prozess, der durch ein Signal beendet wurde, meldet `Claude Code process terminated by signal <name>` in der gleichen Form.

<h3 id="claude-code-returned-an-error-result">
  Claude Code returned an error result
</h3>

Beide SDKs ersetzen den Prozessbeendigungsfehler durch diese Meldung, wenn die CLI ein Fehlerergebnis gemeldet hat, bevor sie beendet wurde:

```
Claude Code returned an error result: <the CLI's own error report>
```

Der Text nach dem Doppelpunkt ist der Bericht der CLI über das, was schief gelaufen ist, daher beginnen Sie dort statt mit dem Exit selbst. Python löst dies als [`ResultError`](/docs/de/agent-sdk/python#resulterror) aus, dessen `data`-Attribut das vollständige Fehlerergebnis trägt. TypeScript lehnt die Nachrichtenschleife mit einem einfachen `Error` ab, der die gleiche Nachrichtenform trägt.

<h2 id="structured-outputs">
  Strukturierte Ausgaben
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output ist None, aber das Ergebnis sagt Erfolg
</h3>

Eine Ergebnismeldung kann mit `subtype: "success"` enden, während `structured_output` in Python `None` oder in TypeScript `undefined` ist. Der Lauf wird abgeschlossen, aber es existiert keine validierte Ausgabe. Eine Möglichkeit, dies zu erreichen, ist ein Schema, das keine Ausgabe erfüllen kann, z. B. widersprüchliche Längenbeschränkungen. Der Lauf endet ohne Validierungsfehler, und das einzige Signal ist die fehlende `structured_output`.

Behandeln Sie dieses Ergebnis als Fehler im Anwendungscode. Überprüfen Sie sowohl, dass `subtype` `success` ist, als auch dass `structured_output` vorhanden ist, bevor Sie es verwenden. Der Abschnitt [Fehlerbehandlung](/docs/de/agent-sdk/structured-outputs#error-handling) zeigt dieses Muster für beide SDKs.

Wenn es wiederholt mit einem Schema auftritt, das Sie für korrekt halten, überprüfen Sie, dass das Schema erfüllbar ist, vereinfachen Sie es dann, bis Ausgaben validieren, und führen Sie Beschränkungen nacheinander wieder ein.

<h2 id="report-a-new-issue">
  Ein neues Problem melden
</h2>

Wenn Ihr Fehler hier nicht behandelt wird, überprüfen Sie die offenen Probleme oder melden Sie ein neues in den SDK-Repositories: [claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) oder [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues). Fügen Sie den vollständigen Fehlertext und Ihre SDK-Version ein.
