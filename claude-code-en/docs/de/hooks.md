> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hooks-Referenz

> Referenz für Claude Code Hook-Ereignisse, Konfigurationsschema, JSON-Ein-/Ausgabeformate, Exit-Codes, asynchrone Hooks, HTTP-Hooks, Prompt-Hooks und MCP-Tool-Hooks.

<Tip>
  Eine Schnellstartanleitung mit Beispielen finden Sie unter [Hooks automatisieren](/docs/de/hooks-guide).
</Tip>

Hooks sind benutzerdefinierte Shell-Befehle, HTTP-Endpunkte, MCP-Tool-Aufrufe, LLM-Prompts oder Subagenten, die automatisch an bestimmten Punkten im Lebenszyklus von Claude Code ausgeführt werden. Claude Code löst die gleichen Hook-Ereignisse überall aus, wo es läuft: Sitzungen im Terminal, IDE-Erweiterungen, die [Desktop-App](/docs/de/desktop-quickstart) und [Cloud-Sitzungen](/docs/de/claude-code-on-the-web). Verwenden Sie diese Referenz, um Ereignisschemas, Konfigurationsoptionen, JSON-Ein-/Ausgabeformate und erweiterte Funktionen wie asynchrone Hooks, HTTP-Hooks und MCP-Tool-Hooks nachzuschlagen.

<h2 id="hook-lifecycle">
  Hook-Lebenszyklus
</h2>

Claude Code führt Hooks an bestimmten Punkten während einer Sitzung aus. Wenn ein Ereignis ausgelöst wird und ein Matcher passt, übergibt Claude Code JSON-Kontext über das Ereignis an Ihren Hook-Handler. Für Command-Hooks kommt die Eingabe über stdin an. Für HTTP-Hooks kommt sie als POST-Request-Body an. Ihr Handler kann dann die Eingabe überprüfen, Maßnahmen ergreifen und optional eine Entscheidung zurückgeben.

Ereignisse fallen in drei Rhythmen:

* pro Sitzung: `SessionStart` und `SessionEnd`
* pro Runde: `UserPromptSubmit`, `Stop` und `StopFailure`
* bei jedem Tool-Aufruf innerhalb der agentengesteuerten Schleife: `PreToolUse` und `PostToolUse`, außer [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior)-Aufrufen, die beide überspringen

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=81b9256c1bbe8832553485f5d9e9c746" className="dark:hidden" alt="Hook-Lebenszyklus-Diagramm, das optionales Setup zeigt, das in SessionStart führt, dann eine Pro-Runde-Schleife mit UserPromptSubmit, UserPromptExpansion für Slash-Befehle, die verschachtelte agentengesteuerte Schleife (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted) und Stop oder StopFailure, gefolgt von TeammateIdle, PreCompact, PostCompact und SessionEnd, mit Elicitation und ElicitationResult verschachtelt in MCP-Tool-Ausführung, PermissionDenied als Seitenzweig von PermissionRequest für Auto-Mode-Ablehnungen, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged und DirectoryAdded als eigenständige asynchrone Ereignisse, PreModelSwitch als eigenständiges sequenzielles Ereignis, das vor einem angeforderten Modellwechsel ausgeführt wird, PostModelSwitch als eigenständiges asynchrones Ereignis, das nach dem Modellwechsel der Sitzung ausgeführt wird, und MessageDisplay als reines Anzeigereignis, das während des Streamings von Assistenten-Nachrichtentexten ausgeführt wird" width="520" height="1336" data-path="images/hooks-lifecycle.svg" />

    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle-dark.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=c9b3d88487335f58cce0b52e2f9e7531" className="hidden dark:block" alt="Hook-Lebenszyklus-Diagramm, das optionales Setup zeigt, das in SessionStart führt, dann eine Pro-Runde-Schleife mit UserPromptSubmit, UserPromptExpansion für Slash-Befehle, die verschachtelte agentengesteuerte Schleife (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted) und Stop oder StopFailure, gefolgt von TeammateIdle, PreCompact, PostCompact und SessionEnd, mit Elicitation und ElicitationResult verschachtelt in MCP-Tool-Ausführung, PermissionDenied als Seitenzweig von PermissionRequest für Auto-Mode-Ablehnungen, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged und DirectoryAdded als eigenständige asynchrone Ereignisse, PreModelSwitch als eigenständiges sequenzielles Ereignis, das vor einem angeforderten Modellwechsel ausgeführt wird, PostModelSwitch als eigenständiges asynchrones Ereignis, das nach dem Modellwechsel der Sitzung ausgeführt wird, und MessageDisplay als reines Anzeigereignis, das während des Streamings von Assistenten-Nachrichtentexten ausgeführt wird" width="520" height="1336" data-path="images/hooks-lifecycle-dark.svg" />
  </Frame>
</div>

Die folgende Tabelle fasst zusammen, wann jedes Ereignis ausgelöst wird. Der Abschnitt [Hook-Ereignisse](#hook-events) dokumentiert das vollständige Eingabeschema und die Optionen zur Entscheidungskontrolle für jedes Ereignis.

| Ereignis              | Wann es ausgelöst wird                                                                                                                                                                                                                                                                                                                                       |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | Wenn eine Sitzung beginnt oder fortgesetzt wird                                                                                                                                                                                                                                                                                                              |
| `Setup`               | Wenn Sie Claude Code mit `--init-only` starten oder mit `--init` oder `--maintenance` im `-p`-Modus. Für einmalige Vorbereitung in CI oder Skripten                                                                                                                                                                                                          |
| `UserPromptSubmit`    | Wenn Sie eine Eingabeaufforderung absenden, bevor Claude sie verarbeitet                                                                                                                                                                                                                                                                                     |
| `UserPromptExpansion` | Wenn ein von Ihnen eingegebener Befehl in eine Eingabeaufforderung erweitert wird, bevor sie Claude erreicht. Kann die Erweiterung blockieren                                                                                                                                                                                                                |
| `PreToolUse`          | Bevor ein Werkzeugaufruf ausgeführt wird. Kann ihn blockieren                                                                                                                                                                                                                                                                                                |
| `PermissionRequest`   | Wenn ein Werkzeugaufruf eine Genehmigungsentscheidung benötigt                                                                                                                                                                                                                                                                                               |
| `PermissionDenied`    | Wenn der automatische Modus einen Werkzeugaufruf ablehnt, einschließlich Ablehnungen ohne Klassifizierer-Urteil. Verwenden Sie JSON `hookSpecificOutput.retry: true`, um dem Modell mitzuteilen, dass es den abgelehnten Werkzeugaufruf möglicherweise erneut versuchen kann. Claude Code ignoriert `retry`, wenn der Klassifizierer kein Urteil gefällt hat |
| `PostToolUse`         | Nach erfolgreichem Werkzeugaufruf                                                                                                                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | Nach fehlgeschlagenem Werkzeugaufruf                                                                                                                                                                                                                                                                                                                         |
| `PostToolBatch`       | Nach Auflösung eines vollständigen Satzes paralleler Werkzeugaufrufe, bevor der nächste Modellaufruf erfolgt                                                                                                                                                                                                                                                 |
| `Notification`        | Wenn Claude Code eine Benachrichtigung sendet                                                                                                                                                                                                                                                                                                                |
| `MessageDisplay`      | Während der Text der Assistentnachricht angezeigt wird                                                                                                                                                                                                                                                                                                       |
| `SubagentStart`       | Wenn ein Subagent erzeugt wird                                                                                                                                                                                                                                                                                                                               |
| `SubagentStop`        | Wenn ein Subagent beendet wird                                                                                                                                                                                                                                                                                                                               |
| `TaskCreated`         | Wenn eine Aufgabe über `TaskCreate` erstellt wird                                                                                                                                                                                                                                                                                                            |
| `TaskCompleted`       | Wenn eine Aufgabe als abgeschlossen markiert wird                                                                                                                                                                                                                                                                                                            |
| `Stop`                | Wenn Claude die Antwort beendet                                                                                                                                                                                                                                                                                                                              |
| `StopFailure`         | Wenn die Runde aufgrund eines API-Fehlers endet                                                                                                                                                                                                                                                                                                              |
| `TeammateIdle`        | Wenn ein [Agent-Team](/docs/de/agent-teams)-Teamkollege im Begriff ist, untätig zu werden                                                                                                                                                                                                                                                                         |
| `InstructionsLoaded`  | Wenn eine CLAUDE.md- oder `.claude/rules/*.md`-Datei in den Kontext geladen wird. Wird beim Sitzungsstart und beim verzögerten Laden von Dateien während einer Sitzung ausgelöst                                                                                                                                                                             |
| `ConfigChange`        | Wenn sich eine Konfigurationsdatei während einer Sitzung ändert                                                                                                                                                                                                                                                                                              |
| `CwdChanged`          | Wenn sich das Arbeitsverzeichnis ändert, z. B. wenn Claude einen `cd`-Befehl ausführt. Nützlich für reaktive Umgebungsverwaltung mit Tools wie direnv                                                                                                                                                                                                        |
| `DirectoryAdded`      | Wenn ein Arbeitsverzeichnis während einer Sitzung über `/add-dir` oder die SDK-Steueranforderung `register_repo_root` hinzugefügt wird                                                                                                                                                                                                                       |
| `FileChanged`         | Wenn sich eine überwachte Datei auf der Festplatte ändert. Das Feld `matcher` gibt an, welche Dateinamen überwacht werden sollen                                                                                                                                                                                                                             |
| `WorktreeCreate`      | Wenn ein Worktree über `--worktree`, `isolation: "worktree"` oder für eine Hintergrundsitzung erstellt wird. Ersetzt das Standard-Git-Verhalten                                                                                                                                                                                                              |
| `WorktreeRemove`      | Wenn ein Worktree beim Sitzungsende, beim Beenden eines Subagenten oder beim Löschen einer Hintergrundsitzung entfernt wird                                                                                                                                                                                                                                  |
| `PreCompact`          | Vor Kontextkomprimierung                                                                                                                                                                                                                                                                                                                                     |
| `PostCompact`         | Nach Abschluss der Kontextkomprimierung                                                                                                                                                                                                                                                                                                                      |
| `PreModelSwitch`      | Bevor Claude Code einen Modellwechsel anwendet, den Sie oder ein Client angefordert haben. Kann den Wechsel blockieren                                                                                                                                                                                                                                       |
| `PostModelSwitch`     | Nach Änderung des Modells der Sitzung, einschließlich Änderungen, die Claude Code selbst vornimmt, z. B. Wiederherstellung des Modells beim Fortsetzen einer Sitzung                                                                                                                                                                                         |
| `Elicitation`         | Wenn ein MCP-Server während eines Werkzeugaufrufs Benutzereingaben anfordert                                                                                                                                                                                                                                                                                 |
| `ElicitationResult`   | Nachdem ein Benutzer auf eine MCP-Abfrage antwortet, bevor die Antwort an den Server zurückgesendet wird                                                                                                                                                                                                                                                     |
| `SessionEnd`          | Wenn eine Sitzung beendet wird                                                                                                                                                                                                                                                                                                                               |

<h3 id="how-a-hook-resolves">
  Wie ein Hook aufgelöst wird
</h3>

Um zu sehen, wie das Ereignis, der Matcher und der Handler zusammenpassen, betrachten Sie diesen `PreToolUse`-Hook, der destruktive Shell-Befehle blockiert.

<Tabs>
  <Tab title="macOS/Linux">
    Der `matcher` grenzt auf Bash-Tool-Aufrufe ein und die `if`-Bedingung grenzt weiter auf Bash-Unterbefehle ein, die mit `rm *` übereinstimmen, daher wird `block-rm.sh` nur ausgeführt, wenn beide Filter passen:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Das Skript liest die JSON-Eingabe von stdin, extrahiert den Befehl und gibt eine `permissionDecision` von `"deny"` zurück, wenn es `rm -rf` enthält. Speichern Sie es unter `.claude/hooks/block-rm.sh` in Ihrem Projekt und machen Sie es mit `chmod +x .claude/hooks/block-rm.sh` ausführbar, damit Claude Code es ausführen kann:

    ```bash theme={null}
    #!/bin/bash
    # .claude/hooks/block-rm.sh
    COMMAND=$(jq -r '.tool_input.command')

    if echo "$COMMAND" | grep -q 'rm -rf'; then
      jq -n '{
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked by hook"
        }
      }'
    else
      exit 0  # no decision; normal permission flow applies
    fi
    ```

    Dieses Skript verwendet wie die anderen Bash-Beispiele auf dieser Seite, die JSON-Eingabe analysieren, `jq`, daher installieren Sie `jq` und stellen Sie sicher, dass es sich in Ihrem `PATH` befindet, bevor Sie sie versuchen.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Der Matcher `Bash|PowerShell` deckt das [PowerShell-Tool](#powershell) sowie Bash ab. Eine einzelne `if`-Regel passt nur zu den Aufrufen eines Tools, daher erhält jedes Tool seinen eigenen Handler: der erste grenzt auf Bash-Unterbefehle ein, die mit `rm *` übereinstimmen, der zweite auf PowerShell-Befehle, die mit `Remove-Item *` übereinstimmen. Beide führen das gleiche Skript über `powershell.exe` aus:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash|PowerShell",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              },
              {
                "type": "command",
                "if": "PowerShell(Remove-Item *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Das Flag `-NoProfile` überspringt das Laden Ihres PowerShell-Profils, damit der Hook schnell startet, und `-ExecutionPolicy Bypass` ermöglicht PowerShell, die lokale Skriptdatei auszuführen.

    Das Skript liest die JSON-Eingabe von stdin, extrahiert den Befehl und gibt eine `permissionDecision` von `"deny"` zurück, wenn es `rm -rf` oder `Remove-Item` gefolgt von `-Recurse` enthält. Speichern Sie es unter `.claude/hooks/block-rm.ps1` in Ihrem Projekt:

    ```powershell theme={null}
    # .claude/hooks/block-rm.ps1
    $callInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $command = $callInput.tool_input.command

    if ($command -match 'rm -rf|Remove-Item.*-Recurse') {
      @{
        hookSpecificOutput = @{
          hookEventName = "PreToolUse"
          permissionDecision = "deny"
          permissionDecisionReason = "Destructive command blocked by hook"
        }
      } | ConvertTo-Json
    } else {
      exit 0  # no decision; normal permission flow applies
    }
    ```
  </Tab>
</Tabs>

Angenommen, Claude Code entscheidet sich, `Bash "rm -rf /tmp/build"` gegen die macOS/Linux-Konfiguration auszuführen. Hier ist, was passiert:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" className="dark:hidden" alt="Diagramm der Hook-Auflösung: PreToolUse wird ausgelöst, der Matcher prüft auf eine Bash-Übereinstimmung, dann prüft die if-Bedingung auf eine Bash(rm *)-Übereinstimmung. Wenn beide passen, wird der Hook-Befehl ausgeführt und gibt permissionDecision deny zurück, daher wird der Tool-Aufruf blockiert und Claude Code wird fortgesetzt. Wenn eine der Prüfungen nicht passt, wird der Hook übersprungen und der Tool-Aufruf darf fortgesetzt werden." width="930" height="270" data-path="images/hook-resolution.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/hook-resolution-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e80af91f8507cee6bd51ac3c2dd92f63" className="hidden dark:block" alt="Diagramm der Hook-Auflösung: PreToolUse wird ausgelöst, der Matcher prüft auf eine Bash-Übereinstimmung, dann prüft die if-Bedingung auf eine Bash(rm *)-Übereinstimmung. Wenn beide passen, wird der Hook-Befehl ausgeführt und gibt permissionDecision deny zurück, daher wird der Tool-Aufruf blockiert und Claude Code wird fortgesetzt. Wenn eine der Prüfungen nicht passt, wird der Hook übersprungen und der Tool-Aufruf darf fortgesetzt werden." width="930" height="270" data-path="images/hook-resolution-dark.svg" />
</Frame>

<Steps>
  <Step title="Ereignis wird ausgelöst">
    Das `PreToolUse`-Ereignis wird ausgelöst. Claude Code sendet die Tool-Eingabe als JSON über stdin an den Hook:

    ```json theme={null}
    { "tool_name": "Bash", "tool_input": { "command": "rm -rf /tmp/build" }, ... }
    ```
  </Step>

  <Step title="Matcher prüft">
    Der Matcher `"Bash"` passt zum Tool-Namen, daher wird diese Hook-Gruppe aktiviert. Wenn Sie den Matcher weglassen oder `"*"` verwenden, wird die Gruppe bei jedem Auftreten des Ereignisses aktiviert.
  </Step>

  <Step title="If-Bedingung prüft">
    Die `if`-Bedingung `"Bash(rm *)"` passt, weil `rm -rf /tmp/build` ein Unterbefehl ist, der mit `rm *` übereinstimmt, daher wird dieser Handler ausgeführt. Wenn der Befehl `npm test` gewesen wäre, würde die `if`-Prüfung fehlschlagen und `block-rm.sh` würde nie ausgeführt, wodurch der Prozess-Spawn-Overhead vermieden wird. Das Feld `if` ist optional; ohne es wird jeder Handler in der passenden Gruppe ausgeführt.
  </Step>

  <Step title="Hook-Handler wird ausgeführt">
    Das Skript überprüft den vollständigen Befehl und findet `rm -rf`, daher gibt es eine Entscheidung auf stdout aus:

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by hook"
      }
    }
    ```

    Wenn der Befehl eine sicherere `rm`-Variante gewesen wäre, wie `rm file.txt`, würde das Skript stattdessen `exit 0` treffen. Exit-Code 0 ohne Ausgabe bedeutet, dass der Hook keine Entscheidung zu melden hat, daher wird der Tool-Aufruf durch den normalen [Berechtigungsfluss](/docs/de/permissions) fortgesetzt. Der Hook kann den Aufruf ablehnen, aber Stille bedeutet nicht, dass er ihn genehmigt.
  </Step>

  <Step title="Claude Code handelt nach dem Ergebnis">
    Claude Code liest die JSON-Entscheidung, blockiert den Tool-Aufruf und zeigt Claude den Grund an.
  </Step>
</Steps>

Der Abschnitt [Konfiguration](#configuration) unten dokumentiert das vollständige Schema, und jeder Abschnitt [Hook-Ereignis](#hook-events) dokumentiert, welche Eingabe Ihr Befehl erhält und welche Ausgabe er zurückgeben kann.

<h2 id="configuration">
  Konfiguration
</h2>

Hooks werden in JSON-Einstellungsdateien definiert. Die Konfiguration hat drei Verschachtelungsebenen:

1. Wählen Sie ein [Hook-Ereignis](#hook-events) aus, auf das reagiert werden soll, wie `PreToolUse` oder `Stop`
2. Fügen Sie eine [Matcher-Gruppe](#matcher-patterns) hinzu, um zu filtern, wann es ausgelöst wird, z. B. „nur für das Bash-Tool"
3. Definieren Sie einen oder mehrere [Hook-Handler](#hook-handler-fields), die ausgeführt werden, wenn eine Übereinstimmung gefunden wird

Siehe [Wie ein Hook aufgelöst wird](#how-a-hook-resolves) oben für eine vollständige Anleitung mit einem kommentierten Beispiel.

<Note>
  Diese Seite verwendet spezifische Begriffe für jede Ebene: **Hook-Ereignis** für den Lebenszykluspunkt, **Matcher-Gruppe** für den Filter und **Hook-Handler** für den Shell-Befehl, HTTP-Endpunkt, MCP-Tool, Prompt oder Agent, der ausgeführt wird. „Hook" bezieht sich allein auf die allgemeine Funktion.
</Note>

<h3 id="hook-locations">
  Hook-Speicherorte
</h3>

Der Ort, an dem Sie einen Hook definieren, bestimmt seinen Umfang:

| Speicherort                                       | Umfang                                                                                                                  | Freigegeben                                                         |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------ |
| `~/.claude/settings.json`                         | Alle Ihre Projekte                                                                                                      | Nein, lokal auf Ihrem Computer                                      |
| `.claude/settings.json`                           | Einzelnes Projekt                                                                                                       | Ja, kann im Repository committed werden                             |
| `.claude/settings.local.json`                     | Einzelnes Projekt                                                                                                       | Nein, gitignored, wenn Claude Code eine Einstellung darin speichert |
| Verwaltete Richtlinieneinstellungen               | Organisationsweit                                                                                                       | Ja, von Administrator kontrolliert                                  |
| [Plugin](/docs/de/plugins/overview) `hooks/hooks.json` | Wenn Plugin aktiviert ist                                                                                               | Ja, mit dem Plugin gebündelt                                        |
| [Skill](/docs/de/skills) Frontmatter                   | Der Rest der Sitzung, sobald der Skill aufgerufen wird. Siehe [Hooks in Skills und Agents](#hooks-in-skills-and-agents) | Ja, in der Skill-Datei definiert                                    |
| [Subagent](/docs/de/sub-agents) Frontmatter            | Während dieser Subagent ausgeführt wird                                                                                 | Ja, in der Subagent-Datei definiert                                 |

[Cloud-Sitzungen](/docs/de/claude-code-on-the-web) lesen Ihre lokale `~/.claude/settings.json` nicht. In einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments-configuration#permissions-and-tool-approval) führt Claude Code auch die Hooks aus, die der Operator vom Host `~/.claude/` des Runners seeded hat, und führt die Hooks in der verwalteten Einstellungsdatei des Runner-Images aus, wenn diese Datei unter den [verwalteten Quellen liegt, die Claude Code anwendet](/docs/de/managed-settings#how-claude-code-combines-managed-sources), was standardmäßig bedeutet, dass nur dann, wenn weder servergesteuerte Einstellungen noch eine von MDM bereitgestellte Claude Code-Richtlinie die verwaltete Ebene bereitstellt. Siehe [was von Ihrem Setup übertragen wird](/docs/de/cloud-environments#what-carries-over-from-your-setup) für welche Einstellungsdateien und Plugins und somit welche Hooks eine Cloud-Sitzung erreichen.

Weitere Informationen zur Auflösung von Einstellungsdateien finden Sie unter [Einstellungen](/docs/de/settings).

Hooks aus Einstellungsdateien, verwalteten Richtlinieneinstellungen und Plugins werden auch in [Subagents](/docs/de/sub-agents) ausgeführt. Wenn ein Subagent ein Tool aufruft, werden Tool-Ereignisse wie `PreToolUse` und `PostToolUse` die gleichen konfigurierten Hooks wie im Hauptgespräch ausgelöst, und die Eingabe enthält die [gemeinsamen Eingabefelder](#common-input-fields) `agent_id` und `agent_type`, die den Subagent identifizieren.

Unternehmensadministratoren können `allowManagedHooksOnly` verwenden, um einzuschränken, welche Hooks ausgeführt werden:

* Ihre Benutzer-, Projekt-, lokalen und Plugin-Hooks werden blockiert. Hooks aus Plugins, die in verwalteten Einstellungen `enabledPlugins` erzwungen aktiviert sind, sind ausgenommen
* Claude Code schränkt auch Ihre [`statusLine`](/docs/de/statusline), [`fileSuggestion`](/docs/de/settings-reference#filesuggestion) und [`subagentStatusLine`](/docs/de/statusline#subagent-status-lines) Einstellungen auf verwaltete Einstellungen ein
* Claude Code deaktiviert auch Plugins mit einer [`command`-Quelle](/docs/de/plugins/marketplace-reference#command-plugin-source), einschließlich Plugins, die in verwalteten Einstellungen `enabledPlugins` erzwungen aktiviert sind, es sei denn, [`disableCommandPluginSources`](/docs/de/settings-reference#disablecommandpluginsources) ist explizit auf `false` gesetzt. `command`-Quellen erfordern Claude Code v2.1.229 oder später
* Claude Code blockiert auch Marketplace-[`headersHelper`-Befehle](/docs/de/plugins/host-marketplace#authenticate-archive-downloads), es sei denn, [`disableCommandPluginSources`](/docs/de/settings-reference#disablecommandpluginsources) ist explizit auf `false` gesetzt, außer für einen Marketplace, den verwaltete Einstellungen selbst deklarieren

Siehe [was unter `allowManagedHooksOnly` ausgeführt wird](/docs/de/settings-reference#what-runs-under-allowmanagedhooksonly).

Hook-Einträge werden über Einstellungsebenen hinweg zusammengeführt, anstatt sich gegenseitig zu ersetzen: Benutzer-, Projekt- und lokale Einstellungen fügen ihre eigenen Hooks hinzu, ohne verwaltete zu entfernen, und die Einstellung [`disableAllHooks`](#disable-or-remove-hooks) kann verwaltete Hooks von außerhalb verwalteter Einstellungen nicht deaktivieren.

Die [HTTP-Hook-Allowlists](/docs/de/settings-reference#hook-and-skill-settings) gelten für Hooks aus jeder Quelle, einschließlich verwalteter Richtlinieneinstellungen:

* `allowedHttpHookUrls`: Wenn auf einer beliebigen Einstellungsebene definiert, führt Claude Code einen HTTP-Hook-Handler nur aus, wenn seine URL mit der zusammengeführten Allowlist übereinstimmt
* `httpHookAllowedEnvVars`: Wenn definiert, interpoliert Claude Code nur die Umgebungsvariablen auf dieser Liste in Hook-Header

<h3 id="matcher-patterns">
  Matcher-Muster
</h3>

Das Feld `matcher` filtert, wann Hooks ausgelöst werden. Wie ein Matcher ausgewertet wird, hängt von den Zeichen ab, die er enthält:

| Matcher-Wert                                                 | Ausgewertet als                                                                                                          | Beispiel                                                                                                                                                            |
| :----------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `"*"`, `""` oder weggelassen                                 | Alle abgleichen                                                                                                          | wird bei jedem Auftreten des Ereignisses ausgelöst                                                                                                                  |
| Nur Buchstaben, Ziffern, `_`, `-`, Leerzeichen, `,` und `\|` | Exakte Zeichenkette oder Liste exakter Zeichenketten, getrennt durch `\|` oder `,` mit optionalem umgebendem Leerzeichen | `Bash` passt nur auf das Bash-Tool; `Edit\|Write` und `Edit, Write` passen jeweils auf eines der beiden Tools genau; `code-reviewer` passt nur auf diesen Agent-Typ |
| Enthält ein anderes Zeichen                                  | JavaScript-Regulärer Ausdruck, nicht verankert                                                                           | `^Notebook` passt auf jedes Tool, dessen Name mit `Notebook` beginnt; `mcp__memory__.*` passt auf jedes Tool vom `memory`-Server                                    |

Ein Matcher auf dem Pfad des regulären Ausdrucks wird mit `RegExp.prototype.test` von JavaScript getestet, was bei einer Übereinstimmung an einer beliebigen Stelle im Wert erfolgreich ist. `Edit.*` passt sowohl auf `Edit` als auch auf `NotebookEdit`; umgeben Sie das Muster mit `^` und `$`, wie in `^Edit$`, wenn Sie eine Übereinstimmung mit der gesamten Zeichenkette benötigen.

Bindestriche in der exakten Übereinstimmungsmenge erfordern Claude Code v2.1.195 oder später. In früheren Versionen wird ein Name mit Bindestrich wie `code-reviewer` als nicht verankerter regulärer Ausdruck ausgewertet, sodass er auch für `senior-code-reviewer` ausgelöst wird; verankern Sie ihn als `^code-reviewer$` in diesen Versionen, um nur diesen Namen abzugleichen.

`FileChanged` und `StopFailure` verwenden einen engeren exakten Übereinstimmungssatz von nur Buchstaben, Ziffern, `_` und `|`. Ein Bindestrich, Leerzeichen oder Komma in einem Matcher für diese beiden Ereignisse hält ihn auf dem Pfad des regulären Ausdrucks, und nur `|` trennt Alternativen. Jedes andere Ereignis mit Matcher-Unterstützung in der folgenden Tabelle akzeptiert `|` oder `,`.

Das Ereignis `FileChanged` folgt diesen Regeln nicht, wenn es seine Beobachtungsliste erstellt. Siehe [FileChanged](#filechanged).

Jeder Ereignistyp passt auf ein anderes Feld:

| Ereignis                                                                                                                                          | Worauf der Matcher filtert                                                                                         | Beispiel-Matcher-Werte                                                                                                                                                                                                                                                         |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | Tool-Name                                                                                                          | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                    | wie die Sitzung gestartet wurde                                                                                    | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                           | welches CLI-Flag Setup ausgelöst hat                                                                               | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                      | warum die Sitzung endete                                                                                           | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                    | Benachrichtigungstyp                                                                                               | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                   | Agent-Typ                                                                                                          | `general-purpose`, `Explore`, `Plan`, benutzerdefinierte Agent-Namen oder Plugin-bezogene Namen wie `^my-plugin:reviewer$`                                                                                                                                                     |
| `PreCompact`, `PostCompact`                                                                                                                       | was Komprimierung ausgelöst hat                                                                                    | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                               | kanonischer Name des Modells, zu dem die Sitzung wechselt, wie unter [PreModelSwitch](#premodelswitch) beschrieben | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                    | Agent-Typ                                                                                                          | gleiche Werte wie `SubagentStart`                                                                                                                                                                                                                                              |
| `ConfigChange`                                                                                                                                    | Konfigurationsquelle                                                                                               | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `CwdChanged`                                                                                                                                      | keine Matcher-Unterstützung                                                                                        | wird immer bei jedem Auftreten ausgelöst                                                                                                                                                                                                                                       |
| `DirectoryAdded`                                                                                                                                  | wie das Verzeichnis hinzugefügt wurde                                                                              | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `FileChanged`                                                                                                                                     | wörtliche Dateinamen zum Beobachten (siehe [FileChanged](#filechanged))                                            | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `StopFailure`                                                                                                                                     | Fehlertyp                                                                                                          | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                              | Ladegrund                                                                                                          | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `UserPromptExpansion`                                                                                                                             | Befehlsname                                                                                                        | Ihre Skill- oder Befehlsnamen                                                                                                                                                                                                                                                  |
| `Elicitation`                                                                                                                                     | MCP-Servername                                                                                                     | Ihre konfigurierten MCP-Servernamen                                                                                                                                                                                                                                            |
| `ElicitationResult`                                                                                                                               | MCP-Servername                                                                                                     | gleiche Werte wie `Elicitation`                                                                                                                                                                                                                                                |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | keine Matcher-Unterstützung                                                                                        | wird immer bei jedem Auftreten ausgelöst                                                                                                                                                                                                                                       |

Das Abgleichen von `StopFailure` auf `cloud_credential_error` erfordert Claude Code v2.1.267 oder später, die erste Version, die Fehler beim Laden von Anmeldedaten unter diesem Wert statt unter `server_error` oder `unknown` meldet.

Für die meisten Ereignisse wertet Claude Code den Matcher gegen ein Feld aus der [JSON-Eingabe](#hook-input-and-output) aus, die es Ihrem Hook auf stdin sendet. Für Tool-Ereignisse ist dieses Feld `tool_name`. Für `PreModelSwitch` und `PostModelSwitch` wertet Claude Code den Matcher gegen den kanonischen Namen aus, den es aus `to_model` ableitet, wie unter [PreModelSwitch](#premodelswitch) beschrieben. Jeder [Hook-Ereignis](#hook-events)-Abschnitt listet den vollständigen Satz von Matcher-Werten und das Eingabeschema für dieses Ereignis auf.

Dieses Beispiel führt ein Linting-Skript nur aus, wenn Claude eine Datei schreibt oder bearbeitet:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/lint-check.sh"
          }
        ]
      }
    ]
  }
}
```

Wenn Sie ein `matcher`-Feld zu einem Ereignis ohne Matcher-Unterstützung hinzufügen, wird es stillschweigend ignoriert.

Für Tool-Ereignisse können Sie enger filtern, indem Sie das Feld [`if`](#common-fields) auf einzelnen Hook-Handlern setzen. `if` verwendet [Berechtigungsregelsyntax](/docs/de/permissions), um gegen den Tool-Namen und die Argumente zusammen abzugleichen, sodass `"Bash(git *)"` ausgeführt wird, wenn ein Bash-Eingabe-Subbefehl `git *` passt und `"Edit(*.ts)"` nur für TypeScript-Dateien ausgeführt wird.

<h4 id="match-mcp-tools">
  MCP-Tools abgleichen
</h4>

[MCP](/docs/de/mcp)-Server-Tools erscheinen als reguläre Tools in Tool-Ereignissen (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), sodass Sie sie auf die gleiche Weise abgleichen können wie jeden anderen Tool-Namen.

MCP-Tools folgen dem Benennungsmuster `mcp__<server>__<tool>`, zum Beispiel:

* `mcp__memory__create_entities`: Entitäten-Tool des Memory-Servers erstellen
* `mcp__filesystem__read_file`: Datei-Lese-Tool des Filesystem-Servers
* `mcp__github__search_repositories`: Such-Tool des GitHub-Servers

Um jedes Tool von einem Server abzugleichen, hängen Sie `.*` an das Server-Präfix an. Das `.*` ist erforderlich: ein Matcher wie `mcp__memory` oder `mcp__brave-search` enthält nur exakte Übereinstimmungszeichen, sodass er als exakte Zeichenkette verglichen wird und kein Tool passt.

* `mcp__memory__.*` passt auf alle Tools vom `memory`-Server
* `mcp__brave-search__.*` passt auf alle Tools von einem Server, dessen Name einen Bindestrich enthält
* `mcp__.*__write.*` passt auf jedes Tool, dessen Name mit `write` beginnt, von jedem Server

Bindestriche in der exakten Übereinstimmungsmenge erfordern Claude Code v2.1.195 oder später. In früheren Versionen wird ein bloßes Präfix mit Bindestrich wie `mcp__brave-search` als nicht verankerter regulärer Ausdruck ausgewertet und passt auf jedes Tool von diesem Server. Die Form `mcp__brave-search__.*` funktioniert auf jeder Version.

Tools von einem [Plugin-gebündelten MCP-Server](/docs/de/mcp#plugin-provided-mcp-servers) verwenden ein bereichsbezogenes Server-Segment, das den Plugin-Namen enthält: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Ein Matcher, der gegen den bloßen Server-Schlüssel geschrieben wird, wird nie für diese Tools ausgelöst. Für ein Plugin namens `my-plugin`, das einen Server unter dem Schlüssel `db` bündelt, erscheint ein `query`-Tool als `mcp__plugin_my-plugin_db__query`, sodass der Matcher für jedes Tool von diesem Server `mcp__plugin_my-plugin_db__.*` ist. Verwenden Sie denselben bereichsbezogenen Tool-Namen im Feld [`if`](#common-fields) eines Handlers. Siehe [Plugin-bereitgestellte MCP-Server](/docs/de/mcp#plugin-provided-mcp-servers) für die Erstellung des bereichsbezogenen Namens.

Dieses Beispiel protokolliert alle Memory-Server-Operationen und validiert Schreibvorgänge von jedem MCP-Server:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

<h3 id="hook-handler-fields">
  Hook-Handler-Felder
</h3>

Jedes Objekt im inneren `hooks`-Array ist ein Hook-Handler: der Shell-Befehl, HTTP-Endpunkt, MCP-Tool, LLM-Prompt oder Agent, der ausgeführt wird, wenn der Matcher passt. Es gibt fünf Typen:

* **[Command-Hooks](#command-hook-fields)** (`type: "command"`): Führen einen Shell-Befehl aus. Ihr Skript empfängt die [JSON-Eingabe](#hook-input-and-output) des Ereignisses auf stdin und kommuniziert Ergebnisse über Exit-Codes und stdout zurück.
* **[HTTP-Hooks](#http-hook-fields)** (`type: "http"`): Senden Sie die [JSON-Eingabe](#hook-input-and-output) des Ereignisses als HTTP-POST-Anfrage an eine URL. Der Endpunkt kommuniziert Ergebnisse über den Antwortkörper mit dem gleichen [JSON-Ausgabeformat](#json-output) wie Command-Hooks zurück.
* **[MCP-Tool-Hooks](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): Rufen Sie ein Tool auf einem bereits verbundenen [MCP-Server](/docs/de/mcp) auf. Die Textausgabe des Tools wird wie Command-Hook-stdout behandelt.
* **[Prompt-Hooks](#prompt-and-agent-hook-fields)** (`type: "prompt"`): Senden Sie einen Prompt an ein Claude-Modell zur Einzelturn-Bewertung. Das Modell gibt seine Entscheidung als JSON zurück. Siehe [Prompt-basierte Hooks](#prompt-based-hooks).
* **[Agent-Hooks](#prompt-and-agent-hook-fields)** (`type: "agent"`): Spawnen Sie einen Subagent, der Tools wie Read, Grep und Glob verwenden kann, um Bedingungen zu überprüfen, bevor er eine Entscheidung zurückgibt. Agent-Hooks sind experimentell und können sich ändern. Siehe [Agent-basierte Hooks](#agent-based-hooks).

Alle passenden Hooks werden parallel ausgeführt. Wenn Sie denselben Handler in mehr als einer Einstellungsdatei definieren, wird er einmal ausgeführt. Eine Kopie desselben Handlers eines Plugins oder Skills bleibt separat.

Handler werden im aktuellen Verzeichnis mit der Umgebung von Claude Code ausgeführt. Wenn das aktuelle Verzeichnis nicht mehr existiert, z. B. ein Worktree oder temporäres Verzeichnis, das eine andere Shell während der Sitzung gelöscht hat, führt Claude Code Command-Hooks aus dem ersten dieser Verzeichnisse aus, das noch existiert: das Verzeichnis, in dem die Sitzung gestartet wurde, das Projekt-Root, Ihr Home-Verzeichnis oder das System-Temp-Verzeichnis. Claude Code zeichnet eine Warnung auf, die das Fallback-Verzeichnis im [Debug-Log](#debug-hooks) benennt.

Die Umgebungsvariable `$CLAUDE_CODE_REMOTE` ist `"true"` in Remote-Web-Umgebungen und nicht gesetzt in der lokalen CLI. Claude Code v2.1.199 und später setzt [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/de/env-vars) auf die [Remote Control](/docs/de/remote-control)-Sitzungs-ID, während die lokale Sitzung eine aktive Remote Control-Verbindung hat.

<h4 id="common-fields">
  Gemeinsame Felder
</h4>

Diese Felder gelten für alle Hook-Typen:

| Feld            | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :-------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | ja           | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` oder `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `if`            | nein         | Berechtigungsregelsyntax zum Filtern, wann dieser Hook ausgeführt wird, z. B. `"Bash(git *)"` oder `"Edit(*.ts)"`. Der Hook-Befehl wird nur ausgeführt, wenn der Tool-Aufruf dem Muster entspricht. Siehe die [Bash-Matching-Tabelle](#bash-if-matching) unten, wie Bash-Muster gegen Subcommands, `$()` und Backticks ausgewertet werden. Nur auf Tool-Ereignissen ausgewertet: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` und `PermissionDenied`. Bei anderen Ereignissen wird ein Hook mit `if` gesetzt nie ausgeführt. Verwendet die gleiche Syntax wie [Berechtigungsregeln](/docs/de/permissions)                                                                                             |
| `timeout`       | nein         | Sekunden vor dem Abbruch. Claude Code erzwingt es nicht auf einem Command-Hook, den Sie mit [`async: true`](#run-hooks-in-the-background) ausführen. Standardwerte: 600 für `command`, `http` und `mcp_tool`; 30 für `prompt`; 60 für `agent`. Claude Code senkt den Standard für `command`, `http` und `mcp_tool` auf 30 bei [`UserPromptSubmit`](#userpromptsubmit), [`PreModelSwitch`](#premodelswitch) und [`PostModelSwitch`](#postmodelswitch) und auf 10 bei [`MessageDisplay`](#messagedisplay). [`SessionEnd`](#sessionend)-Hooks teilen sich ein Budget von 1,5 Sekunden; wenn Ihre Einstellungen einen längeren Pro-Hook-`timeout` setzen, erhöht Claude Code das Budget, um zu entsprechen, bis zu 60 Sekunden |
| `statusMessage` | nein         | Benutzerdefinierte Spinner-Nachricht, die angezeigt wird, während der Hook ausgeführt wird                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `once`          | nein         | Wenn `true`, entfernt Claude Code den Hook nach seiner ersten erfolgreichen Ausführung. Eine Ausführung, die fehlschlägt, mit Exit-Code 2 blockiert oder das Timeout überschreitet, hinterlässt den Hook an Ort und Stelle, sodass er beim nächsten passenden Ereignis erneut ausgeführt wird. Wird nur für Hooks beachtet, die in [Skill-Frontmatter](#hooks-in-skills-and-agents) deklariert sind; wird in Einstellungsdateien und Agent-Frontmatter ignoriert                                                                                                                                                                                                                                                           |

Das Feld `if` enthält genau eine Berechtigungsregel. Es gibt keine `&&`-, `||`- oder Listsyntax zum Kombinieren von Regeln; um mehrere Bedingungen anzuwenden, definieren Sie einen separaten Hook-Handler für jede.

In einer `if`-Bedingung für ein Datei-Tool passt ein Verzeichnismuster mit einem Segment wie `"Edit(src/**)"` nur auf das `src`-Verzeichnis im Arbeitsverzeichnis und die Dateien darunter. Um ein Verzeichnis namens `src` in beliebiger Tiefe abzugleichen, schreiben Sie `"Edit(**/src/**)"`. Vor v2.1.214 passte `"Edit(src/**)"` auf ein Verzeichnis namens `src` in beliebiger Tiefe unter dem Arbeitsverzeichnis.

<span id="bash-if-matching" />Für Bash-Muster hängt davon ab, ob Ihr Hook-Befehl ausgeführt wird, von der Form des Musters und dem Bash-Befehl ab, den Claude aufruft. Führende `VAR=value`-Zuweisungen werden vor dem Abgleich entfernt.

| `if`-Muster        | Bash-Befehl                 | Hook wird ausgeführt? | Warum                                                                                                                                               |
| :----------------- | :-------------------------- | :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`          | ja                    | führende Zuweisungen werden entfernt; `git push` passt                                                                                              |
| `Bash(git *)`      | `npm test && git push`      | ja                    | jeder Subbefehl wird überprüft; `git push` passt                                                                                                    |
| `Bash(rm *)`       | `echo $(rm -rf /)`          | ja                    | Befehle in `$()` und Backticks werden überprüft; `rm -rf /` passt                                                                                   |
| `Bash(rm *)`       | `echo $(date)`              | nein                  | kein Subbefehl passt auf `rm *`                                                                                                                     |
| `Bash(cat *)`      | `echo before $(date) after` | nein                  | eine Substitution kann an jeder Argumentposition sitzen, sodass der vollständige Befehl und `date` beide überprüft werden; keiner passt auf `cat *` |
| `Bash(git *)`      | `$TOOL git push`            | ja                    | Claude Code kann nicht sagen, worauf sich der Befehlsname erweitert, sodass es den Hook ausführt                                                    |
| `Bash(git push *)` | `echo $(date)`              | ja                    | Muster, die mehr als den Befehlsnamen angeben, führen den Hook trotzdem bei `$()`, Backticks oder `$VAR` aus                                        |

Wenn Claude Code nicht bestimmen kann, welche Befehle die Bash-Eingabe ausführt, führt es Ihren Hook unabhängig vom Muster aus. Da der `if`-Filter Best-Effort ist, verwenden Sie das [Berechtigungssystem](/docs/de/permissions) statt eines Hooks, um ein hartes Zulassen oder Verweigern durchzusetzen.

<h4 id="command-hook-fields">
  Command-Hook-Felder
</h4>

Zusätzlich zu den [gemeinsamen Feldern](#common-fields) akzeptieren Command-Hooks diese Felder:

| Feld          | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------ | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | ja           | Shell-Befehl zum Ausführen. Mit `args` die Ausführungsdatei zum direkten Spawnen. Siehe [Exec-Form und Shell-Form](#exec-form-and-shell-form)                                                                                                                                                                                                                                                              |
| `args`        | nein         | Argumentliste. Wenn vorhanden, wird `command` als Ausführungsdatei aufgelöst und direkt mit `args` als Argumentvektor gespawnt, ohne Shell. Siehe [Exec-Form und Shell-Form](#exec-form-and-shell-form)                                                                                                                                                                                                    |
| `async`       | nein         | Wenn `true`, wird im Hintergrund ohne Blockierung ausgeführt. Siehe [Hooks im Hintergrund ausführen](#run-hooks-in-the-background)                                                                                                                                                                                                                                                                         |
| `asyncRewake` | nein         | Wenn `true`, wird im Hintergrund ausgeführt und weckt Claude bei Exit-Code 2 auf. Die stderr des Hooks oder stdout, wenn stderr leer ist, wird Claude als Systemerinnerung angezeigt, damit es auf einen langfristigen Hintergrund-Fehler reagieren kann                                                                                                                                                   |
| `shell`       | nein         | Shell, die für diesen Hook verwendet werden soll. Akzeptiert `"bash"` oder `"powershell"`. Standardmäßig `"bash"` oder `"powershell"` unter Windows, wenn Git Bash nicht installiert ist. Das Setzen von `"powershell"` führt den Befehl über PowerShell unter Windows aus. Erfordert nicht `CLAUDE_CODE_USE_POWERSHELL_TOOL`, da Hooks PowerShell direkt spawnen. Wird ignoriert, wenn `args` gesetzt ist |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Exec-Form und Shell-Form
</h5>

Ein Command-Hook wird als Exec-Form ausgeführt, wenn `args` gesetzt ist, und als Shell-Form, wenn `args` weggelassen ist. Setzen Sie `args`, wenn der Hook auf einen [Pfad-Platzhalter](#reference-scripts-by-path) verweist, da jedes Element als ein Argument ohne Anführungszeichen übergeben wird. Lassen Sie `args` weg, wenn Sie Shell-Funktionen wie Pipes oder `&&` benötigen, oder wenn keine der beiden Bedenken zutrifft.

**Exec-Form** wird ausgeführt, wenn `args` vorhanden ist. Claude Code löst `command` als Ausführungsdatei auf `PATH` auf und spawnt es direkt mit `args` als Argumentvektor. Es gibt keine Shell, sodass jedes `args`-Element genau ein Argument ist, wie geschrieben, und Pfad-Platzhalter wie `${CLAUDE_PLUGIN_ROOT}` werden in `command` und in jedes `args`-Element als einfache Zeichenketten ersetzt. Sonderzeichen wie Apostrophe, `$` und Backticks werden wörtlich durchgeleitet, da es keine Shell gibt, um sie zu interpretieren. Auf keiner Plattform findet Shell-Tokenisierung statt.

**Shell-Form** wird ausgeführt, wenn `args` fehlt. Die `command`-Zeichenkette wird an eine Shell übergeben: `sh -c` auf macOS und Linux, Git Bash unter Windows oder PowerShell, wenn Git Bash nicht installiert ist. Setzen Sie das Feld `shell`, um explizit zu wählen. Die Shell tokenisiert die Zeichenkette, erweitert Variablen und interpretiert Pipes, `&&`, Umleitungen und Globs.

<Note>
  Unter Windows erfordert die Exec-Form, dass `command` sich zu einer echten Ausführungsdatei wie `.exe` auflöst. Die `.cmd`- und `.bat`-Shims, die npm, npx, eslint und andere Tools in `node_modules/.bin` installieren, sind keine Ausführungsdateien und können ohne Shell nicht gespawnt werden. Um sie in Exec-Form auszuführen, rufen Sie das zugrunde liegende Skript direkt mit `node` auf, z. B. `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. Das Muster `node` plus Skriptpfad funktioniert auf jeder Plattform, da `node.exe` eine echte Binärdatei ist. Um einen `.cmd`- oder `.bat`-Shim nach Name auszuführen, verwenden Sie Shell-Form.
</Note>

Dieses Beispiel führt ein Node-Skript aus, das mit einem Plugin gebündelt ist. Exec-Form übergibt den aufgelösten Skriptpfad als ein Argument ohne Anführungszeichen:

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

Die äquivalente Shell-Form benötigt Anführungszeichen, um Pfade mit Leerzeichen oder Sonderzeichen zu handhaben:

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

Beide Formen unterstützen die gleichen [Pfad-Platzhalter](#reference-scripts-by-path) und exportieren sie beide als Umgebungsvariablen `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT` und `CLAUDE_PLUGIN_DATA` auf dem gespawnten Prozess, sodass ein Skript `process.env.CLAUDE_PLUGIN_ROOT` unabhängig davon lesen kann, wie es gestartet wurde.

Plugin-Hooks ersetzen zusätzlich [`${user_config.*}`](/docs/de/plugins/manifest-reference#user-configuration)-Werte, nur in Exec-Form: Der Wert wird in `command` und in jedes `args`-Element als einfache Zeichenkette ersetzt, sodass die Shell ihn nicht erneut analysiert.

Ein Shell-Form-Plugin-Hook, dessen `command` auf `${user_config.*}` verweist, schlägt mit einem [Fehler](/docs/de/errors#plugin-command-references-user-config) fehl, anstatt ausgeführt zu werden. Um einen Optionswert aus einem Shell-Form-Hook zu verwenden, lesen Sie die Umgebungsvariable `$CLAUDE_PLUGIN_OPTION_<KEY>`, z. B. `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` für eine `webhook_url`-Option, oder setzen Sie `args`, um den Hook auf Exec-Form umzuschalten. Vor v2.1.207 ersetzten Shell-Form-Plugin-Hook-Befehle auch `${user_config.*}`.

<Note>
  In Exec-Form ist `command` nur der Ausführungsdateiname oder -pfad. Wenn `command` ein bloßer Name ohne Pfad-Trennzeichen ist und Leerzeichen neben `args` enthält, protokolliert Claude Code eine Warnung, da das Spawn fehlschlägt: Es gibt keine Ausführungsdatei namens `node script.js`. Verschieben Sie die zusätzlichen Token in `args`. Absolute Pfade mit Leerzeichen, z. B. `C:\Program Files\nodejs\node.exe`, sind eine einzelne gültige Ausführungsdatei und lösen die Warnung nicht aus.
</Note>

<h4 id="http-hook-fields">
  HTTP-Hook-Felder
</h4>

Zusätzlich zu den [gemeinsamen Feldern](#common-fields) akzeptieren HTTP-Hooks diese Felder:

| Feld             | Erforderlich | Beschreibung                                                                                                                                                                                                                  |
| :--------------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`            | ja           | URL zum Senden der POST-Anfrage an                                                                                                                                                                                            |
| `headers`        | nein         | Zusätzliche HTTP-Header als Schlüssel-Wert-Paare. Werte unterstützen Umgebungsvariablen-Interpolation mit `$VAR_NAME` oder `${VAR_NAME}`-Syntax. Nur Variablen in `allowedEnvVars` werden aufgelöst                           |
| `allowedEnvVars` | nein         | Liste von Umgebungsvariablennamen, die in Header-Werte interpoliert werden dürfen. Verweise auf nicht aufgelistete Variablen werden durch leere Zeichenketten ersetzt. Erforderlich für jede Umgebungsvariablen-Interpolation |

Claude Code sendet die [JSON-Eingabe](#hook-input-and-output) des Hooks als POST-Anfragekörper mit `Content-Type: application/json`. Der Antwortkörper verwendet das gleiche [JSON-Ausgabeformat](#json-output) wie Command-Hooks.

Die Fehlerbehandlung unterscheidet sich von Command-Hooks; siehe [HTTP-Antwortbehandlung](#http-response-handling).

Dieses Beispiel sendet `PreToolUse`-Ereignisse an einen lokalen Validierungsdienst und authentifiziert sich mit einem Token aus der Umgebungsvariable `MY_TOKEN`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/pre-tool-use",
            "timeout": 30,
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

<h4 id="mcp-tool-hook-fields">
  MCP-Tool-Hook-Felder
</h4>

Zusätzlich zu den [gemeinsamen Feldern](#common-fields) akzeptieren MCP-Tool-Hooks diese Felder:

| Feld     | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                               |
| :------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | ja           | Name eines konfigurierten MCP-Servers. Für einen [Plugin-gebündelten Server](/docs/de/mcp#plugin-provided-mcp-servers) ist dies der bereichsbezogene Name `plugin:<plugin-name>:<server-name>`, z. B. `plugin:my-plugin:db`, nicht der bloße Server-Schlüssel. Der Server muss bereits verbunden sein; der Hook löst nie einen OAuth- oder Verbindungsfluss aus |
| `tool`   | ja           | Name des Tools, das auf diesem Server aufgerufen werden soll                                                                                                                                                                                                                                                                                               |
| `input`  | nein         | Argumente, die an das Tool übergeben werden. Zeichenkettenwerte unterstützen `${path}`-Ersetzung aus der [JSON-Eingabe](#hook-input-and-output) des Hooks, z. B. `"${tool_input.file_path}"`                                                                                                                                                               |

Claude Code liest den Textinhalt des Tools auf die gleiche Weise wie Command-Hook-stdout und folgt der [Parsing-Regel unter Exit-Code 0](#exit-code-0). Wenn der benannte Server nicht verbunden ist oder das Tool `isError: true` zurückgibt, erzeugt der Hook einen nicht blockierenden Fehler und die Ausführung wird fortgesetzt.

Dieses Beispiel ruft das Tool `security_scan` auf dem MCP-Server `my_server` nach jedem `Write` oder `Edit` auf und übergibt den Pfad der bearbeiteten Datei:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "security_scan",
            "input": { "file_path": "${tool_input.file_path}" }
          }
        ]
      }
    ]
  }
}
```

Ein `mcp_tool`-Hook kann nur ausgeführt werden, nachdem Claude Code die MCP-Server der Sitzung für Hooks verfügbar gemacht hat. `SessionStart` und `Setup` können vor diesem Punkt ausgelöst werden:

* **Beim Start**: `SessionStart` wird ausgelöst, bevor die Server verfügbar sind, auch wenn Sie mit `--continue` oder `--resume` starten. Claude Code überspringt die `mcp_tool`-Hooks des Ereignisses, ohne ihre Tools aufzurufen, und das [Debug-Log](#debug-hooks) zeichnet `mcp_tool hooks are not available for the 'SessionStart' hook event (no MCP client context)` auf.
* **Später in einer laufenden Sitzung**: Nach `/clear` oder einer Komprimierung wird `SessionStart` erneut ausgelöst, wobei die Server bereits verfügbar sind, und seine `mcp_tool`-Hooks werden ausgeführt.
* **Bei `Setup`**: `Setup` wird immer ausgelöst, bevor die Server verfügbar sind, sodass Claude Code seine `mcp_tool`-Hooks jedes Mal überspringt und die gleiche Nachricht aufzeichnet, die `Setup` benennt.

Zum Beispiel ruft diese Konfiguration das Tool `load_context` auf dem MCP-Server `my_server` aus einem `SessionStart`-Hook ohne Matcher auf, sodass es auf jede `SessionStart`-Quelle angewendet wird:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "load_context"
          }
        ]
      }
    ]
  }
}
```

Wenn Sie `claude` ausführen, überspringt Claude Code diesen Hook, ruft `load_context` nie auf und schreibt die Nachricht `no MCP client context` in das Debug-Log. Führen Sie `/clear` in dieser gleichen Sitzung aus und der Hook wird ausgeführt und ruft `load_context` auf. Ein `type: "command"`-Hook auf `SessionStart` wird beim Start ausgeführt, verwenden Sie also einen für alles, das die Sitzung von ihrem ersten Turn benötigt.

<h4 id="prompt-and-agent-hook-fields">
  Prompt- und Agent-Hook-Felder
</h4>

Zusätzlich zu den [gemeinsamen Feldern](#common-fields) akzeptieren Prompt- und Agent-Hooks diese Felder:

| Feld     | Erforderlich | Beschreibung                                                                                                                                                                                                    |
| :------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt` | ja           | Prompt-Text zum Senden an das Modell. Verwenden Sie `$ARGUMENTS` als Platzhalter für die Hook-Eingabe-JSON. Mit einem Backslash escapen, um wörtlichen Text einzuschließen: `\$1.00` wird als `$1.00` gerendert |
| `model`  | nein         | Modell, das für die Bewertung verwendet werden soll. Standardmäßig ein schnelles Modell                                                                                                                         |

<h3 id="reference-scripts-by-path">
  Skripte nach Pfad referenzieren
</h3>

Verwenden Sie diese Platzhalter, um Hook-Skripte relativ zum Projekt- oder Plugin-Root zu referenzieren, unabhängig vom Arbeitsverzeichnis, wenn der Hook ausgeführt wird:

* `${CLAUDE_PROJECT_DIR}`: das Projekt-Root, wo die Sitzung gestartet wurde. Claude Code setzt diese Variable auch in der Umgebung von [stdio MCP-Servern](/docs/de/mcp#option-3-add-a-local-stdio-server) und Plugin-LSP-Servern.
* `${CLAUDE_PLUGIN_ROOT}`: das Plugin-Installationsverzeichnis für Skripte, die mit einem [Plugin](/docs/de/plugins/overview) gebündelt sind. Siehe [Plugin-Umgebungsvariablen](/docs/de/plugins/manifest-reference#environment-variables) für das Verhalten des Pfads über Updates hinweg.
* `${CLAUDE_PLUGIN_DATA}`: das [persistente Datenverzeichnis](/docs/de/plugins/components#path-variables-and-persistent-data) des Plugins für Abhängigkeiten und Status, die Plugin-Updates überstehen sollten.

<Note>
  **Worktrees sind anders.** Wenn Claude während der Sitzung einen [Worktree](/docs/de/worktrees) betritt, behält Claude Code `${CLAUDE_PROJECT_DIR}` bei, wo es war, und übergibt den Worktree-Pfad Ihren Hooks auf andere Weise:

  * **`${CLAUDE_PROJECT_DIR}` bleibt stehen**: Es zeigt immer noch auf das Projekt-Root, wo die Sitzung gestartet wurde, sodass ein Befehl wie `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` das Skript immer noch im Haupt-Checkout ausführt.
  * **`cwd` folgt Claude**: Das Feld `cwd` in der Hook-[Eingabe-JSON](#common-input-fields) ist das Worktree-Root, nachdem Claude einen Worktree betritt, und das neue Verzeichnis, nachdem Claude `cd` ausführt. Lesen Sie es, wenn ein Hook wissen muss, welches Verzeichnis Claude bearbeitet.
</Note>

Bevorzugen Sie [Exec-Form](#exec-form-and-shell-form) für jeden Hook, der auf einen Pfad-Platzhalter verweist. In Shell-Form umgeben Sie jeden Platzhalter mit doppelten Anführungszeichen.

<Tabs>
  <Tab title="Projekt-Skripte">
    Dieses Beispiel verwendet `${CLAUDE_PROJECT_DIR}`, um einen Style-Checker aus dem Verzeichnis `.claude/hooks/` des Projekts nach jedem `Write`- oder `Edit`-Tool-Aufruf auszuführen:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Plugin-Skripte">
    Definieren Sie Plugin-Hooks in `hooks/hooks.json` mit einem optionalen Top-Level-Feld `description`. Wenn ein Plugin aktiviert ist, werden seine Hooks mit Ihren Benutzer- und Projekt-Hooks zusammengeführt.

    Dieses Beispiel führt ein Formatierungsskript aus, das mit dem Plugin gebündelt ist:

    ```json theme={null}
    {
      "description": "Automatic code formatting",
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
                "args": [],
                "timeout": 30
              }
            ]
          }
        ]
      }
    }
    ```

    Siehe die [Plugin-Komponenten-Referenz](/docs/de/plugins/components#hooks) für Details zum Erstellen von Plugin-Hooks.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks in Skills und Agents
</h3>

Zusätzlich zu Einstellungsdateien und Plugins können Hooks direkt in [Skills](/docs/de/skills) und [Subagents](/docs/de/sub-agents) mit Frontmatter im gleichen Konfigurationsformat wie einstellungsbasierte Hooks definiert werden. Wie lange Claude Code sie registriert hält, hängt von der Komponente ab:

* **Subagent-Hooks**: Claude Code führt sie nur aus, während dieser Subagent ausgeführt wird, und entfernt sie, wenn er fertig ist. Claude Code konvertiert einen `Stop`-Hook hier zu `SubagentStop`, dem Ereignis, das ausgelöst wird, wenn ein Subagent abgeschlossen ist.
* **Skill-Hooks**: Claude Code registriert sie, wenn Sie oder Claude den Skill aufrufen, und führt sie für den Rest der Sitzung aus, auf Turns nach dem eigenen Turn des Skills auch. Um Claude Code stattdessen einen Hook nach seiner ersten erfolgreichen Ausführung zu entfernen, setzen Sie [`once: true`](#common-fields) darauf.

Dieser Skill definiert einen `PreToolUse`-Hook, der ein Sicherheitsvalidierungsskript vor jedem `Bash`-Befehl ausführt:

```yaml theme={null}
---
name: secure-operations
description: Perform operations with security checks
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/security-check.sh"
---
```

Subagents verwenden das gleiche Format in ihrem YAML-Frontmatter.

Frontmatter-Hooks in einem Projekt-Skill folgen der gleichen [Workspace-Trust-Regel wie Hooks in Einstellungsdateien](#workspace-trust). Claude Code registriert sie, wenn Sie oder Claude den Skill aufrufen, auch in einem `-p`-Run in einem Ordner, dem Sie nicht vertraut haben.

Frontmatter-Hooks in einem Projekt-Subagent werden nur ausgeführt, nachdem Sie den [Workspace-Trust-Dialog](/docs/de/permissions#project-allow-rules-and-workspace-trust) für den Ordner akzeptieren, aus dem die Agent-Datei stammt. Eine `-p`-Sitzung zählt nicht als Akzeptanz. [Was vor dem Vertrauen in einen Ordner ausgeführt wird](/docs/de/permissions#what-runs-before-you-trust-a-folder) vergleicht dies mit der Einstellungsdatei-Regel, und die Subagents-Seite listet auf, [welche Bereiche ausgenommen sind](/docs/de/sub-agents#hooks-in-subagent-frontmatter). Vor v2.1.218 konnten diese Hooks aus Ordnern ausgeführt werden, denen Sie nicht vertraut haben.

<h3 id="the-/hooks-menu">
  Das `/hooks`-Menü
</h3>

Geben Sie `/hooks` in Claude Code ein, um einen schreibgeschützten Browser für Ihre konfigurierten Hooks zu öffnen. Das Menü zeigt jedes Hook-Ereignis mit einer Anzahl konfigurierter Hooks, lässt Sie in Matcher bohren und zeigt die vollständigen Details jedes Hook-Handlers. Verwenden Sie es, um die Konfiguration zu überprüfen, zu überprüfen, aus welcher Einstellungsdatei ein Hook stammt, oder um einen Hook-Befehl, Prompt oder URL zu überprüfen.

Das Menü zeigt alle fünf Hook-Typen: `command`, `prompt`, `agent`, `http` und `mcp_tool`. Jeder Hook ist mit einem `[type]`-Präfix und einer Quelle gekennzeichnet, die angibt, wo er definiert wurde:

* `User Settings`: aus `~/.claude/settings.json`
* `Project Settings`: aus `.claude/settings.json`
* `Local Settings`: aus `.claude/settings.local.json`
* `Plugin Hooks`: aus der `hooks/hooks.json` eines Plugins
* `Session Hooks`: in der aktuellen Sitzung im Speicher registriert

Das Auswählen eines Hooks öffnet eine Detailansicht, die sein Ereignis, Matcher, Typ, Quellendatei und den vollständigen Befehl, Prompt oder URL anzeigt. Das Menü ist schreibgeschützt: Um Hooks hinzuzufügen, zu ändern oder zu entfernen, bearbeiten Sie die Einstellungs-JSON direkt oder bitten Sie Claude, die Änderung vorzunehmen.

<h3 id="disable-or-remove-hooks">
  Hooks deaktivieren oder entfernen
</h3>

Um einen Hook zu entfernen, löschen Sie seinen Eintrag aus der Einstellungs-JSON-Datei.

Um alle Hooks vorübergehend zu deaktivieren, ohne sie zu entfernen, setzen Sie `"disableAllHooks": true` in Ihrer Einstellungsdatei. Claude Code liest den Wert, der nach [Einstellungspriorität](/docs/de/settings#settings-precedence) bleibt, sodass ein `"disableAllHooks": false` in der `.claude/settings.json` eines Projekts ein `true` in Ihren Benutzereinstellungen überschreibt. Um Hooks für einen Run auszuschalten, unabhängig davon, was die Einstellungen des Projekts sagen, übergeben Sie `--settings '{"disableAllHooks": true}'`, was Vorrang vor Projekt- und lokalen Einstellungen hat. Es gibt keine Möglichkeit, einen einzelnen Hook zu deaktivieren, während er in der Konfiguration bleibt.

Die Einstellung `disableAllHooks` respektiert die Hierarchie der verwalteten Einstellungen. Wenn ein Administrator Hooks durch verwaltete Richtlinieneinstellungen konfiguriert hat, kann `disableAllHooks`, das in Benutzer-, Projekt- oder lokalen Einstellungen gesetzt ist, diese verwalteten Hooks nicht deaktivieren. Nur `disableAllHooks`, das auf der Ebene der verwalteten Einstellungen gesetzt ist, kann verwaltete Hooks deaktivieren. Für die vollständige Reichweite jeder Ebene siehe [`disableAllHooks`](/docs/de/settings-reference#disableallhooks).

Direkte Änderungen an Hooks in Einstellungsdateien werden normalerweise automatisch vom Datei-Watcher aufgegriffen.

<h2 id="hook-input-and-output">
  Hook-Eingabe und -Ausgabe
</h2>

Command Hooks empfangen JSON-Daten über stdin und teilen Ergebnisse über Exit-Codes, stdout und stderr mit. HTTP Hooks empfangen das gleiche JSON wie der POST-Request-Body und teilen Ergebnisse über den HTTP-Response-Body mit. Dieser Abschnitt behandelt Felder und Verhalten, die für alle Events gemeinsam sind. Jeder Event-Abschnitt unter [Hook Events](#hook-events) enthält sein spezifisches Input-Schema und Optionen zur Entscheidungskontrolle.

Auf macOS und Linux werden Command Hooks in ihrer eigenen Session ohne steuerndes Terminal ausgeführt. Der Hook-Prozess und alle untergeordneten Prozesse können `/dev/tty` nicht öffnen oder Escape-Sequenzen direkt an die Claude Code-Schnittstelle senden. Windows hat kein `/dev/tty`.

Um eine Nachricht für den Benutzer auf jeder Plattform anzuzeigen, geben Sie [`systemMessage`](#json-output) in der JSON-Ausgabe zurück. Einige Events verwerfen sie oder liefern sie an anderer Stelle, und jeder [Event-Abschnitt](#hook-events) gibt an, wo. Um eine Desktop-Benachrichtigung auszulösen, einen Fenstertitel zu setzen oder die Glocke zu läuten, geben Sie stattdessen [`terminalSequence`](#emit-terminal-notifications) zurück.

<h3 id="common-input-fields">
  Allgemeine Eingabefelder
</h3>

Hook Events empfangen diese Felder als JSON, zusätzlich zu Event-spezifischen Feldern, die in jedem [Hook-Event](#hook-events)-Abschnitt dokumentiert sind. Für Command Hooks kommt dieses JSON über stdin an. Für HTTP Hooks kommt es als POST-Request-Body an.

| Feld              | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | Aktuelle Session-Kennung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `prompt_id`       | UUID, die den aktuell verarbeiteten Benutzer-Prompt identifiziert. Stimmt mit dem [`prompt.id`-Attribut auf OpenTelemetry-Events](/docs/de/monitoring-usage#event-correlation-attributes) überein, sodass Sie Hook-Ausgabe mit Telemetrie für einen einzelnen Prompt korrelieren können. Nicht vorhanden bis zur ersten Benutzereingabe. Erfordert Claude Code v2.1.196 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `transcript_path` | Pfad zur Konversations-JSON. Die Transkriptdatei wird asynchron geschrieben und kann der In-Memory-Konversation hinterherhinken, daher kann sie möglicherweise noch nicht die neuesten Nachrichten des aktuellen Turns enthalten, wenn ein Hook ausgelöst wird. Hooks, die den endgültigen Assistant-Text des aktuellen Turns benötigen, sollten `last_assistant_message` auf [Stop](#stop) und [SubagentStop](#subagentstop) verwenden, anstatt das Transkript zu lesen                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `cwd`             | Aktuelles Arbeitsverzeichnis, wenn der Hook aufgerufen wird                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `scratchpad_dir`  | Pfad zum Scratchpad-Verzeichnis der Session, in dem Claude temporäre Arbeitsdateien speichert. Nicht vorhanden, wenn die Session kein Scratchpad hat oder das Temp-Verzeichnis nicht verfügbar ist. Erfordert Claude Code v2.1.257 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `permission_mode` | Aktueller [Berechtigungsmodus](/docs/de/permissions#permission-modes): `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` oder `"bypassPermissions"`. Der als **Manuell** bezeichnete Modus kommt als `"default"` an, nie als `"manual"`, sodass Skripte, die `"default"` abgleichen, weiterhin funktionieren. Nicht alle Events erhalten dieses Feld. Überprüfen Sie das JSON-Beispiel in jedem [Hook-Event](#hook-events)-Abschnitt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `effort`          | Objekt mit einem `level`-Feld, das die [Effort-Stufe](/docs/de/model-config#adjust-effort-level) enthält, die wirksam ist, wenn der Hook ausgeführt wird: `"low"`, `"medium"`, `"high"`, `"xhigh"` oder `"max"`. Wenn Sie eine Stufe festlegen, die das aktive Modell nicht unterstützt, meldet `level` die Stufe, die Claude Code stattdessen ausgeführt hat; [Effort-Stufe anpassen](/docs/de/model-config#adjust-effort-level) sagt, wie es diese Stufe auswählt. Ultracode ist keine separate Stufe und wird als `"xhigh"` gemeldet. Das Objekt stimmt mit dem [Status-Feld](/docs/de/statusline#available-data) `effort`-Feld überein. Vorhanden für Events, die innerhalb eines Tool-Use-Kontexts ausgelöst werden, wie `PreToolUse`, `PostToolUse`, `Stop` und `SubagentStop`, wenn das aktuelle Modell den Effort-Parameter unterstützt. Die Stufe ist auch für Hook-Befehle und das Bash-Tool als die `$CLAUDE_EFFORT`-Umgebungsvariable verfügbar. |
| `hook_event_name` | Name des ausgelösten Events                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

Bei Ausführung mit `--agent` oder innerhalb eines Subagenten sind zwei zusätzliche Felder enthalten:

| Feld         | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Eindeutige Kennung für den Subagenten. Nur vorhanden, wenn der Hook innerhalb eines Subagenten-Aufrufs ausgelöst wird. Verwenden Sie dies, um Subagenten-Hook-Aufrufe von Main-Thread-Aufrufen zu unterscheiden.                                                                                                                                                                                                                        |
| `agent_type` | Agent-Name (z. B. `"Explore"` oder `"security-reviewer"`). Vorhanden, wenn die Session `--agent` verwendet oder der Hook innerhalb eines Subagenten ausgelöst wird. Für Subagenten hat der Typ des Subagenten Vorrang vor dem `--agent`-Wert der Session. Siehe [SubagentStart](#subagentstart) für die Werte, die benutzerdefinierte und Plugin-Subagenten melden, und wie man einen Matcher gegen einen Plugin-Scoped-Namen schreibt. |

Nur [`SessionStart`](#sessionstart)-Hooks können ein `model`-Feld empfangen, und Claude Code fügt es nicht immer ein. [`PreModelSwitch`](#premodelswitch)- und [`PostModelSwitch`](#postmodelswitch)-Hooks empfangen stattdessen `from_model` und `to_model`, verwenden Sie also einen PostModelSwitch-Hook, um das Modell zu verfolgen, während es sich während einer Session ändert.

Es gibt keine `$CLAUDE_MODEL`-Umgebungsvariable. Der Hook kann `$ANTHROPIC_MODEL` lesen, wenn Sie es in Ihrer Shell festlegen, aber dieser Wert ändert sich nicht, wenn Sie während einer Session mit `/model` Modelle wechseln.

Ein Hook-Prozess erbt die übergeordnete Umgebung, mit Ausnahme der `OTEL_*`-Exporter-Variablen, die Claude Code [aus jedem Subprocess entfernt, den es spawnt](/docs/de/monitoring-usage#administrator-configuration), und, wenn [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/de/env-vars#variables) auf `1` gesetzt ist, die Variablen, die es entfernt.

Beispielsweise empfängt ein `PreToolUse`-Hook für einen Bash-Befehl dies auf stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "scratchpad_dir": "/tmp/claude-1000/-home-user-my-project/abc123/scratchpad",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite",
    "timeout": 120000,
    "run_in_background": false
  },
  "tool_use_id": "toolu_01ABC123..."
}
```

Die Felder `tool_name`, `tool_input` und `tool_use_id` sind Event-spezifisch. Jeder [Hook-Event](#hook-events)-Abschnitt dokumentiert die zusätzlichen Felder für diesen Event.

<h3 id="exit-code-output">
  Exit-Code-Ausgabe
</h3>

Der Exit-Code aus Ihrem Hook-Befehl teilt Claude Code mit, ob die Aktion fortgesetzt, blockiert oder ignoriert werden soll. Der Exit-Code wirkt nicht allein. Claude Code liest [JSON-Ausgabefelder](#json-output) von stdout bei jedem Exit-Code, nicht nur 0, und für Events, die das Standard-Entscheidungsmodell verwenden, wirkt ein gepartes Objekt, das die Schema-Validierung besteht, neben dem Code. Exit 2's Block ist das einzige Ergebnis, das JSON nicht überschreiben kann.

Zwei Tabellen besitzen die Event-spezifischen Ausnahmen: [Exit-Code-2-Verhalten pro Event](#exit-code-2-behavior-per-event) sagt, was Exit-Codes für jeden Event tun, und [Entscheidungskontrolle](#decision-control) sagt, welche Entscheidungsfelder jeder Event berücksichtigt. Universelle Felder wie `systemMessage` funktionieren über die meisten Events hinweg und sind in der [JSON-Ausgabe](#json-output)-Tabelle aufgelistet.

<h4 id="exit-code-0">
  Exit-Code 0
</h4>

Exit 0 bedeutet Erfolg und ist der beabsichtigte Exit-Code, wenn Sie JSON für strukturierte Kontrolle drucken.

Für die meisten Events schreibt Claude Code stdout in das Debug-Protokoll und zeigt es nicht im Transkript an. Die Ausnahmen sind `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` und `PostModelSwitch`, wo Claude Code einfachen Text-stdout als Kontext hinzufügt, den Claude sehen und bearbeiten kann.

Ob Claude Code Ihren stdout als [JSON-Ausgabe](#json-output) oder als einfachen Text liest, hängt davon ab, wie er beginnt und endet, wobei umgebender Whitespace ignoriert wird:

* **Beginnt mit `{` und endet mit `}`**: Claude Code parst es als JSON. Wenn die Ausgabe zwei oder mehr Zeilen sind, die jeweils selbst als JSON geparst werden, und keine Zeile ein [JSON-Ausgabe](#json-output)-Objekt ist, das ein Feld setzt, behandelt Claude Code die gesamte Ausgabe als einfachen Text. Wenn eine dieser Zeilen ein Feld setzt, ist die gesamte Ausgabe ein Parse-Fehler, der unten beschrieben wird.
* **Beginnt mit `{` aber endet nicht mit `}`**: Claude Code behandelt es als einfachen Text.
* **Beginnt mit etwas anderem**: Claude Code behandelt es als einfachen Text, ein JSON-Array oder einen zitierten JSON-String enthalten.

Für Events, die das Standard-Entscheidungsmodell verwenden, ist Exit 0 mit einem geparsten Objekt, das die Schema-Validierung nicht besteht, ein nicht blockierender Fehler: die Aktion wird fortgesetzt, und das Transkript zeigt einen `<Hook-Name> hook error`-Hinweis mit der Validierungsmeldung. Das gleiche passiert bei jedem Exit-Code außer 2, während [Exit 2 immer noch blockiert](#exit-code-2).

Für Events, die das Standard-Entscheidungsmodell verwenden, wenn Claude Code versucht, Ihren stdout als JSON zu parsen und kann nicht, meldet es einen nicht blockierenden Fehler bei jedem Exit-Code außer 2. Das Transkript zeigt einen `<Hook-Name> hook error`-Hinweis mit der Parse-Meldung. Bei den Events, die einfachen Text-stdout als Kontext hinzufügen, fügt Claude Code den Text nicht hinzu. Vor v2.1.248 behandelte Claude Code diesen stdout als einfachen Text.

Stderr von einem Hook, der mit 0 beendet wird, geht nur in das Debug-Protokoll, nie in das Transkript, und Claude sieht es nie. Um es selbst zu lesen, aktivieren Sie [Debug-Protokollierung](#debug-hooks). Um eine Warnung an Claude von einem `PostToolUse`- oder `PostToolUseFailure`-Hook zu übermitteln, beenden Sie stattdessen mit 2, damit [Claude den stderr sieht](#exit-code-2-behavior-per-event), obwohl das Tool bereits ausgeführt wurde.

<h4 id="exit-code-2">
  Exit-Code 2
</h4>

Exit 2 bedeutet einen blockierenden Fehler. Bei [Events, die blockieren können](#exit-code-2-behavior-per-event), blockiert Exit 2, unabhängig davon, ob Sie JSON drucken oder nicht: selbst ein JSON `permissionDecision` von `"allow"` kann es nicht überschreiben. Claude Code liest immer noch alle gültigen [JSON-Ausgabe](#json-output) auf stdout. Bei `Elicitation` und `ElicitationResult` wird die `hookSpecificOutput` eines Exit-2-Hooks ignoriert.

Die Blockierungsmeldung ist der Grund aus der Blockierungsentscheidung Ihres JSON, wenn es eine gibt, und Ihr stderr-Text andernfalls. Was der Block tut, variiert je nach Event: `PreToolUse` blockiert den Tool-Aufruf, `UserPromptSubmit` lehnt den Prompt ab, und so weiter. [Exit-Code-2-Verhalten pro Event](#exit-code-2-behavior-per-event) listet die Auswirkung für jeden Event auf, und jeder Event-Abschnitt sagt, wohin die Meldung geht.

Ein Hook, der mit 2 beendet wird, während JSON gedruckt wird, das die [JSON-Ausgabe](#json-output)-Schema-Validierung nicht besteht, blockiert immer noch: Claude Code verwendet stderr als Blockierungsgrund und zeichnet den Validierungsfehler im Debug-Protokoll auf. Vor v2.1.214 behandelte Claude Code diese Kombination als nicht blockierenden Fehler und die Aktion wurde fortgesetzt.

Dieses Skript blockiert `rm`-Befehle durch Beendigung mit 2 und lässt jeden anderen Befehl zum normalen Berechtigungsfluss:

```bash theme={null}
#!/bin/bash
# Reads JSON input from stdin, checks the command
input=$(cat)
command=$(jq -r '.tool_input.command' <<<"$input")

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Blocking error: tool call is prevented
fi

exit 0  # No decision: the normal permission flow applies
```

<h4 id="other-exit-codes">
  Andere Exit-Codes
</h4>

Jeder andere Exit-Code blockiert nicht allein für die meisten Hook-Events. Was passiert, hängt von Ihrem stdout ab:

* Mit einem geparsten Objekt, das die Schema-Validierung besteht, für Events, die das Standard-Entscheidungsmodell verwenden, ignoriert Claude Code den Exit-Code und das JSON allein entscheidet das Ergebnis:
  * Jedes Feld, das der Event unterstützt, wird berücksichtigt, einschließlich `permissionDecision`, `additionalContext`, `updatedInput` und `systemMessage`, und der Hook wird nicht als Fehler gemeldet.
  * [Entscheidungskontrolle](#decision-control) listet die Entscheidungsfelder pro Event auf; universelle Felder wie `systemMessage` folgen der [JSON-Ausgabe](#json-output)-Tabelle.
* Mit einem geparsten Objekt, das die Schema-Validierung nicht besteht, für Events, die das Standard-Entscheidungsmodell verwenden, ist es der gleiche nicht blockierende Fehler wie [bei Exit 0](#exit-code-0): die Aktion wird fortgesetzt, und der `<Hook-Name> hook error`-Hinweis trägt die Validierungsmeldung.
* Mit stdout, das Claude Code [versucht als JSON zu parsen](#exit-code-0) und kann nicht, meldet Claude Code den gleichen nicht blockierenden Fehler wie bei Exit 0 für Events, die das Standard-Entscheidungsmodell verwenden. Die Aktion wird fortgesetzt, und der Hinweis trägt die Parse-Meldung.
* Mit stdout, das Claude Code [als einfachen Text behandelt](#exit-code-0), oder mit leerem stdout, ist es ein nicht blockierender Fehler für die meisten Hook-Events: die Aktion wird fortgesetzt, und das Transkript zeigt einen `<Hook-Name> hook error`-Hinweis gefolgt von der ersten Zeile von stderr, mit dem Präfix `Failed with non-blocking status code:`. Um den vollständigen stderr zu erfassen, aktivieren Sie [Debug-Protokollierung](#debug-hooks).

Events außerhalb des Standard-Entscheidungsmodells behalten ihre eigenen Zeilen in der [Pro-Event-Tabelle](#exit-code-2-behavior-per-event): `WorktreeCreate` schlägt die Erstellung bei jedem Nonzero-Exit fehl, unabhängig davon, was Ihr JSON sagt, und Events, die Hook-Ausgabe vollständig verwerfen, wie `StopFailure`, ignorieren Ihr JSON bei jedem Exit-Code, abgesehen von Nebeneffekt-Feldern wie `terminalSequence`, die immer noch ausgelöst werden.

Ein Hook, der nicht starten kann, landet im gleichen nicht blockierenden Bucket. Wenn der Skriptpfad nicht existiert oder nicht ausführbar ist, beendet die Shell mit einem Code wie 127 und Sie sehen den gleichen Hinweis mit der Interpreter-Meldung, zum Beispiel `Failed with non-blocking status code: /bin/sh: /path/to/hook.sh: No such file or directory`. Für die meisten Hook-Events wird die Aktion fortgesetzt. Wenn Sie einen Policy-Hook einrichten, achten Sie auf diesen Hinweis bei seiner ersten Ausführung: ein Tippfehler im Pfad in `settings.json` lässt das Gate stillschweigend deaktiviert.

<Warning>
  Für die meisten Hook-Events ist Exit-Code 2 der einzige Exit-Code, der allein durch den Code blockiert. Ohne gültiges JSON auf stdout behandelt Claude Code Exit-Code 1 als nicht blockierenden Fehler und setzt die Aktion fort, obwohl 1 der konventionelle Unix-Fehlercode ist. Wenn Ihr Hook eine Richtlinie durchsetzen soll, verwenden Sie `exit 2`. Die Worktree-Events unterscheiden sich: jeder Nonzero-Exit-Code von `WorktreeCreate` bricht die Worktree-Erstellung ab, und jeder Nonzero-Exit-Code von `WorktreeRemove` lässt die Worktree-Entfernung fehlschlagen, wenn das Verzeichnis danach noch existiert.
</Warning>

<h4 id="timeouts">
  Timeouts
</h4>

Abgesehen von einem Command Hook, den Sie mit [`async: true`](#run-hooks-in-the-background) ausführen, bricht Claude Code einen `command`-, `http`- oder `mcp_tool`-Hook ab, der sein [`timeout`](#common-fields) erreicht, verwirft die Hook-Ausgabe, sodass bei den meisten Events ein abgelaufener Hook keine Entscheidung rendert.

Bei [`PreModelSwitch`](#premodelswitch) blockiert ein Hook, der bei seinem Timeout abgebrochen wird, den Modellwechsel. Bei `PreToolUse` unterscheiden sich die beiden Hook-Familien:

* Ein abgelaufener `command`-, `http`- oder `mcp_tool`-Hook blockiert den Tool-Aufruf nicht. Der Aufruf wird durch den normalen [Berechtigungsfluss](/docs/de/permissions) fortgesetzt, verlassen Sie sich also nicht auf einen steckengebliebenen Hook, um als Gate zu fungieren.
* Ein [Agent SDK Callback Hook](/docs/de/agent-sdk/hooks), der sein Timeout überschreitet, [blockiert den Tool-Aufruf](#pretooluse).

<h4 id="exit-code-2-behavior-per-event">
  Exit-Code-2-Verhalten pro Event
</h4>

Exit-Code 2 ist die Art, wie ein Hook signalisiert „Stopp, mach das nicht." Die Auswirkung hängt vom Event ab, da einige Events Aktionen darstellen, die blockiert werden können (wie ein Tool-Aufruf, der noch nicht stattgefunden hat), und andere Dinge darstellen, die bereits passiert sind oder nicht verhindert werden können.

| Hook-Event            | Kann blockieren? | Was passiert bei Exit 2                                                                                                                                                                                                                                                                                                |
| :-------------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Ja               | Blockiert den Tool-Aufruf                                                                                                                                                                                                                                                                                              |
| `PermissionRequest`   | Nein             | Exit-Code 2 wird für diesen Event nicht berücksichtigt und der Berechtigungsfluss wird unverändert fortgesetzt. Verweigern Sie stattdessen durch das [`decision`-Objekt](#permissionrequest-decision-control)                                                                                                          |
| `UserPromptSubmit`    | Ja               | Blockiert die Prompt-Verarbeitung und löscht den Prompt                                                                                                                                                                                                                                                                |
| `UserPromptExpansion` | Ja               | Blockiert die Erweiterung                                                                                                                                                                                                                                                                                              |
| `Stop`                | Ja               | Verhindert, dass Claude stoppt, setzt die Konversation fort                                                                                                                                                                                                                                                            |
| `SubagentStop`        | Ja               | Verhindert, dass der Subagent stoppt                                                                                                                                                                                                                                                                                   |
| `TeammateIdle`        | Ja               | Verhindert, dass der Teammate untätig wird, sodass er weiterarbeitet                                                                                                                                                                                                                                                   |
| `TaskCreated`         | Ja               | Rollback der Task-Erstellung                                                                                                                                                                                                                                                                                           |
| `TaskCompleted`       | Ja               | Verhindert, dass die Task als abgeschlossen markiert wird                                                                                                                                                                                                                                                              |
| `ConfigChange`        | Ja               | Blockiert die Konfigurationsänderung von der Wirksamkeit (außer `policy_settings`)                                                                                                                                                                                                                                     |
| `StopFailure`         | Nein             | Ausgabe und Exit-Code werden ignoriert, außer `terminalSequence`                                                                                                                                                                                                                                                       |
| `PostToolUse`         | Nein             | Zeigt stderr Claude; das Tool ist bereits ausgeführt                                                                                                                                                                                                                                                                   |
| `PostToolUseFailure`  | Nein             | Zeigt stderr Claude; das Tool ist bereits fehlgeschlagen                                                                                                                                                                                                                                                               |
| `PostToolBatch`       | Ja               | Stoppt die agentic Loop vor dem nächsten Modellaufruf                                                                                                                                                                                                                                                                  |
| `PermissionDenied`    | Nein             | Exit-Code und stderr werden ignoriert, da die Verweigerung bereits aufgetreten ist. Verwenden Sie JSON `hookSpecificOutput.retry: true`, um dem Modell zu sagen, dass es möglicherweise erneut versuchen kann; Claude Code ignoriert `retry: true` für [No-Verdict-Verweigerungen](#permissiondenied-decision-control) |
| `Notification`        | Nein             | Exit-Code und stderr werden ignoriert                                                                                                                                                                                                                                                                                  |
| `SubagentStart`       | Nein             | Zeigt stderr nur dem Benutzer                                                                                                                                                                                                                                                                                          |
| `SessionStart`        | Nein             | Zeigt stderr nur dem Benutzer                                                                                                                                                                                                                                                                                          |
| `Setup`               | Nein             | Exit-Code und stderr werden ignoriert                                                                                                                                                                                                                                                                                  |
| `SessionEnd`          | Nein             | Zeigt stderr nur dem Benutzer                                                                                                                                                                                                                                                                                          |
| `CwdChanged`          | Nein             | Zeigt stderr nur dem Benutzer                                                                                                                                                                                                                                                                                          |
| `DirectoryAdded`      | Nein             | Stderr geht in das Debug-Protokoll; das Verzeichnis ist bereits hinzugefügt                                                                                                                                                                                                                                            |
| `FileChanged`         | Nein             | Zeigt stderr nur dem Benutzer                                                                                                                                                                                                                                                                                          |
| `PreCompact`          | Ja               | Blockiert die Komprimierung                                                                                                                                                                                                                                                                                            |
| `PostCompact`         | Nein             | Zeigt stderr nur dem Benutzer                                                                                                                                                                                                                                                                                          |
| `PreModelSwitch`      | Ja               | Blockiert den Modellwechsel und zeigt stderr dem Benutzer                                                                                                                                                                                                                                                              |
| `PostModelSwitch`     | Nein             | Zeigt stderr nur dem Benutzer; das Modell ist bereits gewechselt                                                                                                                                                                                                                                                       |
| `Elicitation`         | Ja               | Verweigert die Elicitation                                                                                                                                                                                                                                                                                             |
| `ElicitationResult`   | Ja               | Blockiert die Antwort (Aktion wird Ablehnung)                                                                                                                                                                                                                                                                          |
| `WorktreeCreate`      | Ja               | Jeder Nonzero-Exit-Code führt dazu, dass die Worktree-Erstellung fehlschlägt                                                                                                                                                                                                                                           |
| `WorktreeRemove`      | Ja               | Jeder Nonzero-Exit-Code führt dazu, dass die Worktree-Entfernung fehlschlägt, wenn das Verzeichnis danach noch existiert. Siehe [WorktreeRemove](#worktreeremove) für das, was mit dem Verzeichnis passiert                                                                                                            |
| `InstructionsLoaded`  | Nein             | Exit-Code wird ignoriert                                                                                                                                                                                                                                                                                               |
| `MessageDisplay`      | Nein             | Der ursprüngliche Text wird angezeigt                                                                                                                                                                                                                                                                                  |

Für `SessionStart`, `SubagentStart` und `PostModelSwitch` rendert Claude Code den Exit-Code-2-stderr im Transkript als `<Hook-Name> hook error`-Hinweis, auf die gleiche Weise wie es einen [nicht blockierenden Fehler](#exit-code-output) rendert. Claude sieht es nicht, und die Session oder der Subagent wird fortgesetzt. Für `SubagentStart` erscheint der Hinweis im eigenen Transkript des Subagenten, nicht in der übergeordneten Konversation.

<h3 id="http-response-handling">
  HTTP-Response-Handling
</h3>

HTTP Hooks verwenden HTTP-Statuscodes und Response-Bodies anstelle von Exit-Codes und stdout. Die folgenden Ergebnisse gelten für die meisten Events; ein Event mit seinem eigenen Fehlervertrag in der [Pro-Event-Tabelle](#exit-code-2-behavior-per-event), wie `WorktreeCreate`, wendet diesen Vertrag auch auf einen fehlgeschlagenen HTTP-Hook an:

* **2xx mit leerem Body**: Erfolg, äquivalent zu Exit-Code 0 ohne Ausgabe
* **2xx mit JSON-Objekt-Body**: geparst mit dem gleichen [JSON-Ausgabe](#json-output)-Schema wie Command Hooks. Ein Body, der die Schema-Validierung nicht besteht, ist ein nicht blockierender Fehler
* **2xx mit jedem anderen Body, wie einfacher Text**: nicht blockierender Fehler, behandelt wie ein Nicht-2xx-Status. Claude Code fügt den Text nicht zu Claudes Kontext hinzu
* **Nicht-2xx-Status**: nicht blockierender Fehler, Ausführung wird fortgesetzt
* **Verbindungsfehler**: nicht blockierender Fehler, Ausführung wird fortgesetzt
* **Timeout**: der Hook wird abgebrochen, wie unter [Timeouts](#timeouts) beschrieben

Im Gegensatz zu Command Hooks können HTTP Hooks einen blockierenden Fehler nicht allein durch Statuscodes signalisieren. Um einen Tool-Aufruf zu blockieren oder eine Berechtigung zu verweigern, geben Sie eine 2xx-Response mit einem JSON-Body zurück, der die entsprechenden Entscheidungsfelder enthält.

<h3 id="json-output">
  JSON-Ausgabe
</h3>

Exit-Codes lassen Sie nur blockieren oder schweigen, aber JSON-Ausgabe gibt Ihnen feinere Kontrolle. Anstatt mit Code 2 zu beenden, um zu blockieren, beenden Sie mit 0 und drucken Sie ein JSON-Objekt auf stdout. Claude Code liest spezifische Felder aus diesem JSON, um Verhalten zu steuern, einschließlich [Entscheidungskontrolle](#decision-control) zum Blockieren, Zulassen oder Eskalieren an den Benutzer.

<Note>
  Wählen Sie einen Ansatz pro Hook: Verwenden Sie entweder Exit-Codes allein zum Signalisieren oder beenden Sie mit 0 und drucken Sie JSON für strukturierte Kontrolle. Wenn Sie sie mischen, behält Exit 2 seine [blockierende Auswirkung](#exit-code-2-behavior-per-event), und Claude Code liest immer noch die JSON-Felder, mit der einen Elicitation-Ausnahme, die unter [Exit-Code 2](#exit-code-2) notiert ist.
</Note>

Der stdout Ihres Hooks muss nur das JSON-Objekt enthalten. Wenn Ihr Shell-Profil beim Start Text druckt, kann es die JSON-Analyse beeinträchtigen. Siehe [Hook JSON hat keine Auswirkung](/docs/de/hooks-guide#hook-json-has-no-effect) im Troubleshooting-Leitfaden.

Die Strings `additionalContext`, `systemMessage` und `initialUserMessage` eines Hooks sowie sein einfacher stdout sind auf 10.000 Zeichen begrenzt:

* **Umfang**: Claude Code misst jeden String für sich, auch wenn mehrere Hooks für den gleichen Event ausgeführt werden. Für JSON-Ausgabe wird jedes Feld separat gemessen; einfacher stdout wird als Ganzes gemessen.
* **Über dem Limit**: Claude Code speichert die Ausgabe in einer Datei im Session-Verzeichnis und ersetzt sie durch den Dateipfad und eine Vorschau von bis zu den ersten 2.000 Zeichen. Ein großes gültiges Bash-Ergebnis wird auf die gleiche Weise behandelt, beschrieben unter [Ausgabelimits](/docs/de/tools-reference#output-limits). Im Gegensatz zu dieser Bash-Obergrenze hat diese Obergrenze keine Einstellung oder Umgebungsvariable, um sie zu erhöhen.
* **Datei lesen**: Claude Code bittet Claude nicht, die Datei zu lesen, daher halten Sie alles, das Claude immer sehen muss, innerhalb der Obergrenze.

Das JSON-Objekt unterstützt drei Arten von Feldern:

* **Universelle Felder** wie `continue` sind in der folgenden Tabelle aufgelistet. Jeder Event akzeptiert sie, aber einige Events verwerfen sie oder liefern `systemMessage` an anderer Stelle als dem Transkript. Jeder Event-Abschnitt sagt so. `terminalSequence` funktioniert auch auf diesen Events, mit den Ausnahmen, die unter [Terminal-Benachrichtigungen ausgeben](#emit-terminal-notifications) aufgelistet sind.
* **Top-Level `decision` und `reason`** werden von einigen Events verwendet, um zu blockieren oder Feedback zu geben.
* **`hookSpecificOutput`** ist ein verschachteltes Objekt für Events, die reichere Kontrolle benötigen. Es erfordert ein `hookEventName`-Feld, das auf den Event-Namen gesetzt ist.

| Feld               | Standard | Beschreibung                                                                                                                                                                                                                                                                                                                                                                               |
| :----------------- | :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`   | Wenn `false`, stoppt Claude die Verarbeitung vollständig, nachdem der Hook ausgeführt wird. Hat Vorrang vor allen Event-spezifischen Entscheidungsfeldern                                                                                                                                                                                                                                  |
| `stopReason`       | keine    | Meldung, die dem Benutzer angezeigt wird, wenn `continue` `false` ist. Sie bleibt in der Konversation, sodass Claude sie sieht, wenn die Konversation fortgesetzt wird                                                                                                                                                                                                                     |
| `suppressOutput`   | `false`  | Hat keine Auswirkung: Claude Code akzeptiert das Feld, aber handelt nicht danach. Der stdout eines erfolgreichen Hooks wird nie im Transkript angezeigt und wird im Debug-Protokoll aufgezeichnet                                                                                                                                                                                          |
| `systemMessage`    | keine    | Warnmeldung, die dem Benutzer angezeigt wird. In [Agent SDK](/docs/de/agent-sdk/overview) und [`--output-format stream-json`](/docs/de/headless)-Ausgabe kann es als [`SDKInformationalMessage`](/docs/de/agent-sdk/typescript#sdkinformationalmessage) ankommen                                                                                                                                          |
| `terminalSequence` | keine    | Eine Terminal-Escape-Sequenz, die Claude Code in Ihrem Namen ausgeben soll, wie eine Desktop-Benachrichtigung, einen Fenstertitel oder eine Glocke. Beschränkt auf OSC `0`/`1`/`2`/`9`/`99`/`777` und BEL. Wenn der Wert etwas außerhalb der Zulassungsliste enthält, wird das Feld ignoriert. Verwenden Sie dies anstelle des Schreibens zu `/dev/tty`, das für Hooks nicht verfügbar ist |

Um Claude vollständig zu stoppen:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

Für `PreToolUse`- und `PostToolUse`-Hooks gilt der Stop auch, wenn der Tool-Aufruf fehlschlägt oder abgeschlossen wird, während Claude immer noch eine Antwort streamt.

<h4 id="emit-terminal-notifications">
  Terminal-Benachrichtigungen ausgeben
</h4>

Hooks werden ohne steuerndes Terminal ausgeführt, daher schlägt das direkte Schreiben von Escape-Sequenzen zu `/dev/tty` fehl. Geben Sie stattdessen die Escape-Sequenz im `terminalSequence`-Feld zurück und Claude Code gibt sie für Sie über seinen eigenen Terminal-Schreibpfad aus. Dies ist race-frei, funktioniert innerhalb von tmux und GNU screen und funktioniert unter Windows, wo es kein `/dev/tty` gibt.

Das Feld akzeptiert einen String von einer oder mehreren zugelassenen Escape-Sequenzen:

* OSC `0`, `1`, `2`: Fenster- und Icon-Titel
* OSC `9`: iTerm2, ConEmu, Windows Terminal und WezTerm-Benachrichtigungen, einschließlich `9;4` Taskleisten-Fortschritt
* OSC `99`: Kitty-Benachrichtigungen
* OSC `777`: urxvt, Ghostty und Warp-Benachrichtigungen
* Bare BEL

Sequenzen können mit BEL oder mit ST beendet werden. Alles außerhalb der Zulassungsliste, einschließlich CSI-Cursor- und Farbsequenzen, OSC-Palettensequenzen, OSC-8-Hyperlinks, OSC-52-Clipboard-Schreibvorgänge und OSC 1337, wird abgelehnt und das Feld wird ignoriert.

Claude Code schreibt die Sequenz selbst, wenn es Ihre Hook-Ausgabe verarbeitet, sodass das Feld bei Events funktioniert, die `systemMessage` und `continue` verwerfen, wie `Notification` und `StopFailure`. Es hat zwei Limits:

* Claude Code schreibt die Sequenz nur in einer interaktiven Session und nur, während seine Schnittstelle auf dem Bildschirm ist. Im nicht-interaktiven Modus mit dem `-p`-Flag und im Agent SDK ignoriert es das Feld.
* Ein `WorktreeCreate`-Command-Hook kann kein JSON zurückgeben, da Claude Code seinen stdout als Worktree-Pfad liest. Ein HTTP-`WorktreeCreate`-Hook gibt JSON zurück und kann das Feld enthalten.

Das folgende Beispiel löst eine Desktop-Benachrichtigung von einem `Notification`-Hook aus. Die Escape-Sequenz wird mit `printf`-Oktal-Escapes erstellt, sodass die Steuerbytes nie auf der Shell-Befehlszeile erscheinen, und `jq -n --arg` erstellt die JSON-Ausgabe, sodass Anführungszeichen, Backslashes und Zeilenumbrüche in der Benachrichtigungsmeldung korrekt escaped werden:

```bash theme={null}
#!/bin/bash
# Notification hook: ping the desktop when Claude Code needs attention.
input=$(cat)
title="Claude Code"
body=$(jq -r '.message // "Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

Die `{ "terminalSequence": "..." }`-Form ist die gleiche aus jeder Shell oder Sprache.

<h4 id="add-context-for-claude">
  Kontext für Claude hinzufügen
</h4>

Das `additionalContext`-Feld übergibt einen String von Ihrem Hook in Claudes Kontextfenster. Claude Code umhüllt den String in eine Systemerinnerung und fügt ihn in die Konversation an dem Punkt ein, an dem der Hook ausgelöst wurde. Claude liest die Erinnerung bei der nächsten Modellanfrage, aber sie erscheint nicht als Chat-Nachricht in der Schnittstelle.

Geben Sie `additionalContext` innerhalb von `hookSpecificOutput` neben dem Event-Namen zurück:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "This file is generated. Edit src/schema.ts and run `bun generate` instead."
  }
}
```

Wo die Erinnerung erscheint, hängt vom Event ab:

* [SessionStart](#sessionstart) und [SubagentStart](#subagentstart): am Anfang der Konversation, vor dem ersten Prompt
* [UserPromptSubmit](#userpromptsubmit) und [UserPromptExpansion](#userpromptexpansion): neben dem eingereichten Prompt
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) und [PostToolBatch](#posttoolbatch): neben dem Tool-Ergebnis
* [Stop](#stop) und [SubagentStop](#subagentstop): am Ende des Turns. Die Konversation wird fortgesetzt, sodass Claude auf das Feedback reagieren kann. Siehe [Stop-Entscheidungskontrolle](#stop-decision-control)
* [PostModelSwitch](#postmodelswitch): mit der nächsten Anfrage nach dem Wechsel. Siehe [PostModelSwitch-Entscheidungskontrolle](#postmodelswitch-decision-control) für Timing

Wenn mehrere Hooks `additionalContext` für den gleichen Event zurückgeben, empfängt Claude alle Werte.

Wenn ein Wert 10.000 Zeichen überschreitet, schreibt Claude Code den Text in eine Datei im Session-Verzeichnis und übergibt Claude stattdessen den Dateipfad mit einer Vorschau von bis zu den ersten 2.000 Zeichen. Claude kann die Datei lesen, aber Claude Code bittet nicht darum.

Verwenden Sie `additionalContext` für Informationen, die Claude über den aktuellen Zustand Ihrer Umgebung oder die gerade ausgeführte Operation wissen sollte:

* **Umgebungszustand**: der aktuelle Branch, Bereitstellungsziel oder aktive Feature-Flags
* **Bedingte Projektregeln**: welcher Test-Befehl für die gerade bearbeitete Datei gilt, welche Verzeichnisse in diesem Worktree schreibgeschützt sind
* **Externe Daten**: offene Issues, die Ihnen zugewiesen sind, aktuelle CI-Ergebnisse, Inhalte, die von einem internen Service abgerufen wurden

Für Anweisungen, die sich nie ändern, bevorzugen Sie [CLAUDE.md](/docs/de/memory). Es wird ohne Ausführung eines Skripts geladen und ist der Standard-Ort für statische Projektkonventionen.

Schreiben Sie den Text als sachliche Aussagen statt imperativer Systembefehle. Formulierungen wie „Das Bereitstellungsziel ist Produktion" oder „Dieses Repo verwendet `bun test`" werden als Projektinformationen gelesen. Text, der als Out-of-Band-Systembefehle formuliert ist, kann Claudes Prompt-Injection-Abwehr auslösen, was dazu führt, dass Claude den Text an Sie übermittelt, anstatt ihn als Kontext zu behandeln.

Claude Code speichert den eingefügten Text im Session-Transkript. Für Mid-Session-Events wie `PostToolUse` oder `UserPromptSubmit`, wenn Sie mit `--continue` oder `--resume` fortfahren, spielt Claude Code den gespeicherten Text erneut ab, anstatt den Hook für vergangene Turns erneut auszuführen, sodass Werte wie Zeitstempel oder Commit-SHAs veraltet werden. `SessionStart`-Hooks werden bei Wiederaufnahme mit `source` auf `"resume"` oder `"fork"` gesetzt, wenn Sie `--fork-session` hinzugefügt haben, erneut ausgeführt, sodass sie ihren Kontext aktualisieren können.

<h4 id="decision-control">
  Entscheidungskontrolle
</h4>

Nicht jeder Event unterstützt Blockierung oder Verhaltenskontrolle durch JSON. Die Events, die dies tun, verwenden jeweils einen anderen Satz von Feldern, um diese Entscheidung auszudrücken. Verwenden Sie diese Tabelle als schnelle Referenz, bevor Sie einen Hook schreiben:

| Events                                                                                                                              | Entscheidungsmuster                            | Schlüsselfelder                                                                                                                                                                                                                                                                                            |
| :---------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | Top-Level `decision`                           | `decision: "block"`, `reason`. Stop und SubagentStop akzeptieren auch `hookSpecificOutput.additionalContext` für [nicht-fehlerhafte Rückmeldung, die die Konversation fortsetzt](#stop-decision-control)                                                                                                   |
| TeammateIdle, TaskCompleted                                                                                                         | Exit-Code oder `continue: false`               | Exit-Code 2 blockiert die Aktion mit stderr-Rückmeldung. JSON `{"continue": false, "stopReason": "..."}` stoppt auch den Teammate vollständig, was dem `Stop`-Hook-Verhalten entspricht; [TaskCompleted ignoriert es, wenn das `TaskUpdate`-Tool den Event ausgelöst hat](#taskcompleted-decision-control) |
| TaskCreated                                                                                                                         | Exit-Code oder Top-Level `decision`            | Exit-Code 2 oder `decision: "block"` [bricht die Task ab](#taskcreated-decision-control) und gibt die Meldung an Claude zurück. `continue: false` wird ignoriert                                                                                                                                           |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                           | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                                                                                    |
| PreModelSwitch                                                                                                                      | `hookSpecificOutput` oder Top-Level `decision` | `permissionDecision` (allow/deny/ask), `permissionDecisionReason`. `decision: "block"` [bricht auch den Wechsel ab](#premodelswitch-decision-control)                                                                                                                                                      |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                           | `decision.behavior` (allow/deny)                                                                                                                                                                                                                                                                           |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                           | `retry: true` teilt dem Modell mit, dass es den verweigerten Tool-Aufruf möglicherweise erneut versuchen kann; Claude Code ignoriert es für [No-Verdict-Verweigerungen](#permissiondenied-decision-control)                                                                                                |
| WorktreeCreate                                                                                                                      | Pfad-Rückgabe                                  | Command Hook druckt Pfad auf stdout; HTTP Hook gibt `hookSpecificOutput.worktreePath` zurück. Hook-Fehler oder fehlender Pfad schlägt die Erstellung fehl                                                                                                                                                  |
| WorktreeRemove                                                                                                                      | Exit-Code                                      | Jeder Nonzero-Exit-Code lässt die Entfernung fehlschlagen, wenn das Verzeichnis danach noch existiert. JSON-Ausgabe wird verworfen                                                                                                                                                                         |
| Elicitation                                                                                                                         | `hookSpecificOutput`                           | `action` (accept/decline/cancel), `content` (Formularfeldwerte für accept)                                                                                                                                                                                                                                 |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                           | `action` (accept/decline/cancel), `content` (Formularfeldwerte überschreiben)                                                                                                                                                                                                                              |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                           | `displayContent` ersetzt den angezeigten Text auf dem Bildschirm. Nur Anzeige: das Transkript und das, was Claude sieht, behalten das Original                                                                                                                                                             |
| SessionStart, SubagentStart, PostModelSwitch                                                                                        | Nur Kontext                                    | `hookSpecificOutput.additionalContext` fügt Kontext für Claude hinzu. SessionStart akzeptiert auch [`initialUserMessage`, `watchPaths`, `sessionTitle` und `reloadSkills`](#sessionstart-decision-control). Keine Blockierung oder Entscheidungskontrolle                                                  |
| Setup, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, DirectoryAdded, FileChanged              | Keine                                          | Keine Entscheidungskontrolle. Wird für Nebeneffekte wie Protokollierung oder Bereinigung verwendet                                                                                                                                                                                                         |

Einige Events können auch Inhalte umschreiben, anstatt nur zu erlauben oder zu blockieren:

* `PreToolUse`: `updatedInput` direkt unter `hookSpecificOutput` ersetzt die Argumente eines Tools, bevor es ausgeführt wird. Siehe [PreToolUse-Entscheidungskontrolle](#pretooluse-decision-control)
* `PermissionRequest`: `updatedInput` innerhalb des `decision`-Objekts. Siehe [PermissionRequest-Entscheidungskontrolle](#permissionrequest-decision-control)
* `PostToolUse`: `updatedToolOutput` ersetzt das Tool-Ergebnis. Siehe [PostToolUse-Entscheidungskontrolle](#posttooluse-decision-control)
* `UserPromptSubmit`: kann den Prompt nicht ersetzen; es injiziert nur `additionalContext` daneben

Für Redaktions- oder Transformationsfälle, fangen Sie bei `PreToolUse` für ausgehende Tool-Eingaben und `PostToolUse` für eingehende Tool-Ergebnisse ab.

Hier sind Beispiele für jedes Muster in Aktion:

<Tabs>
  <Tab title="Top-level decision">
    Der einzige Wert für `decision` ist `"block"`. Um die Aktion fortzusetzen, lassen Sie `decision` aus Ihrem JSON weg oder beenden Sie mit 0 ohne JSON:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Verwendet `hookSpecificOutput` für reichere Kontrolle: erlauben, verweigern oder eskalieren an den Benutzer. Sie können auch Tool-Eingabe vor der Ausführung ändern oder zusätzlichen Kontext für Claude injizieren. Siehe [PreToolUse-Entscheidungskontrolle](#pretooluse-decision-control) für den vollständigen Satz von Optionen.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Database writes are not allowed"
      }
    }
    ```
  </Tab>

  <Tab title="PermissionRequest">
    Verwendet `hookSpecificOutput`, um eine Berechtigungsanfrage im Namen des Benutzers zu erlauben oder zu verweigern. Beim Erlauben können Sie auch die Tool-Eingabe ändern oder Berechtigungsregeln anwenden, sodass der Benutzer nicht erneut aufgefordert wird. Siehe [PermissionRequest-Entscheidungskontrolle](#permissionrequest-decision-control) für den vollständigen Satz von Optionen.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "allow",
          "updatedInput": {
            "command": "npm run lint"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Für erweiterte Beispiele einschließlich Bash-Befehlsvalidierung, Prompt-Filterung und Auto-Approval-Skripte, siehe [Was Sie automatisieren können](/docs/de/hooks-guide#what-you-can-automate) im Leitfaden und die [Bash-Befehlsvalidierungs-Referenzimplementierung](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py).

<h2 id="hook-events">
  Hook-Ereignisse
</h2>

Jedes Ereignis entspricht einem Punkt im Lebenszyklus von Claude Code, an dem Hooks ausgeführt werden können. Die folgenden Abschnitte sind in der Reihenfolge des Lebenszyklus angeordnet: von der Sitzungseinrichtung über die agentengesteuerte Schleife bis zum Sitzungsende. Jeder Abschnitt beschreibt, wann das Ereignis ausgelöst wird, welche Matcher es unterstützt, welche JSON-Eingabe es empfängt, und wie das Verhalten durch die Ausgabe gesteuert wird.

<h3 id="sessionstart">
  SessionStart
</h3>

Wird ausgeführt, wenn Claude Code eine neue Sitzung startet oder eine vorhandene Sitzung fortsetzt. Nützlich zum Laden von Entwicklungskontext wie vorhandenen Problemen oder kürzlichen Änderungen an Ihrer Codebasis oder zum Einrichten von Umgebungsvariablen. Für statischen Kontext, der kein Skript erfordert, verwenden Sie stattdessen [CLAUDE.md](/docs/de/memory).

SessionStart wird bei jeder Sitzung ausgeführt, daher halten Sie diese Hooks schnell. Nur `type: "command"` und `type: "mcp_tool"` Hooks werden unterstützt. Siehe [MCP-Tool-Hook-Felder](#mcp-tool-hook-fields) für den Zeitpunkt der Ausführung von `mcp_tool` Hooks.

Der Matcher-Wert entspricht der Art, wie die Sitzung eingeleitet wurde:

| Matcher   | Wann wird es ausgelöst                                                                                                                                        |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `startup` | Neue Sitzung                                                                                                                                                  |
| `resume`  | `--resume`, `--continue` oder `/resume`                                                                                                                       |
| `clear`   | `/clear`                                                                                                                                                      |
| `compact` | Automatische oder manuelle Komprimierung                                                                                                                      |
| `fork`    | Eine neue Sitzung, die aus einer vorhandenen abgezweigt wurde: `--fork-session` mit `--resume` oder `--continue`, die `/fork` Hintergrundkopie oder `/branch` |

Vor v2.1.214 meldeten abgezweigte Sitzungen die Quelle `"resume"`.

Wenn Sie eine interaktive Sitzung starten, ein Gespräch beim Start mit `--continue` oder `--resume` fortsetzen oder `/clear` ausführen, werden SessionStart-Hooks im Hintergrund ausgeführt. Sie können sofort tippen, und ein fortgesetztes Gespräch wird angezeigt, ohne auf die Hooks zu warten. Claudes erste Antwort wartet immer noch darauf, dass die Hooks fertig sind, damit ihr Kontext Claude erreicht.

Wenn Sie mit `/resume` innerhalb einer Sitzung zu Gesprächen wechseln, wartet der Wechsel darauf, dass die Hooks fertig sind. Wenn Sie `/clear` ausführen oder zu einem anderen Gespräch wechseln, während Hintergrund-Hooks noch laufen, gilt nichts, was sie zurückgeben, für die Sitzung.

Die gleiche Wartezeit gilt beim Start, einschließlich einer fortgesetzten Sitzung: eine Eingabeaufforderung, die Sie senden, während SessionStart-Hooks noch laufen, erreicht Claude nicht, bis sie fertig sind.

Während dieser Wartezeit drücken Sie `Esc`, um die Eingabeaufforderung zurück in die Eingabe zu nehmen, ohne sie zu senden. Die Hooks laufen weiter.

<h4 id="sessionstart-input">
  SessionStart-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten SessionStart-Hooks `source` und optional `model`, `agent_type` und `session_title`:

| Feld            | Beschreibung                                                                                                                                                                                                                                                   |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Wie die Sitzung gestartet wurde: `"startup"` für neue Sitzungen, `"resume"` für fortgesetzte Sitzungen, `"clear"` nach `/clear`, `"compact"` nach Komprimierung oder `"fork"` für eine neue Sitzung, die aus einer vorhandenen abgezweigt wurde                |
| `model`         | Die aktive Modell-ID. Sie kann weggelassen werden, z. B. nach `/clear` oder wenn eine Sitzung durch Gesprächswiederherstellung wiederhergestellt wird, daher überprüfen Sie das Feld, bevor Sie es lesen                                                       |
| `agent_type`    | Der Agent-Name, vorhanden, wenn Sie Claude Code mit `claude --agent <name>` starten                                                                                                                                                                            |
| `session_title` | Der aktuelle Sitzungstitel, falls bereits gesetzt, z. B. über `--name` oder `/rename`. Ein Hook, der `sessionTitle` ausgibt, kann `session_title` zuerst überprüfen, um zu vermeiden, dass ein Titel überschrieben wird, den der Benutzer explizit gesetzt hat |

Wenn `source` `"resume"` oder `"fork"` ist und das Transkript mindestens eine Antwort von Claude enthält, erhalten SessionStart-Hooks auch die vier folgenden Felder. Ihr Hook kann sie verwenden, um zu melden, was das Fortsetzen eines veralteten Gesprächs kostet, bevor die erste Anfrage erfolgt, z. B. in einer [`systemMessage`](#json-output). Diese Felder erfordern Claude Code v2.1.251 oder später.

| Feld                          | Beschreibung                                                                                                                                                                                              |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `seconds_since_last_response` | Wanduhr-Sekunden seit der letzten Antwort im fortgesetzten Transkript                                                                                                                                     |
| `context_tokens`              | Tokens, die die erste Anfrage der fortgesetzten Sitzung als ihre Eingabeaufforderung erneut sendet                                                                                                        |
| `prompt_cache_likely_expired` | `true`, wenn die letzte Antwort älter ist als die [Prompt-Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime) der Sitzung oder eine spätere Komprimierung das zwischengespeicherte Gespräch ersetzt hat |
| `estimated_cache_write_usd`   | Geschätzte Kosten in US-Dollar für das Schreiben von `context_tokens` in den Prompt-Cache auf dem Modell der Sitzung, ohne die Antwort                                                                    |

Dieses Beispiel zeigt die Eingabe für eine Sitzung, die 90 Minuten nach ihrer letzten Antwort fortgesetzt wurde:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "resume",
  "model": "claude-opus-5",
  "seconds_since_last_response": 5400,
  "context_tokens": 182340,
  "prompt_cache_likely_expired": true,
  "estimated_cache_write_usd": 1.1396
}
```

<h4 id="sessionstart-decision-control">
  SessionStart-Entscheidungskontrolle
</h4>

Claude Code fügt stdout, das es [als Klartext behandelt](#exit-code-0), zu Claudes Kontext hinzu. Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, können Sie diese ereignisspezifischen Felder zurückgeben:

| Feld                 | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | String, der zu Claudes Kontext am Anfang des Gesprächs hinzugefügt wird, vor der ersten Eingabeaufforderung. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude), um zu erfahren, wie der Text bereitgestellt wird und was Sie darin einfügen sollten                                                                                                                                                   |
| `initialUserMessage` | String, der als erste Benutzernachricht der Sitzung verwendet wird. Gilt im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p` Flag, wo es zum ersten Zug wird, auch wenn keine Eingabeaufforderung bereitgestellt wird. Wenn eine Eingabeaufforderung bereitgestellt wird, folgt sie als nächster Zug. Im Gegensatz zu `additionalContext`, das an einen vorhandenen Zug angehängt wird, erstellt dies den Zug |
| `sessionTitle`       | Legt den Sitzungstitel fest, mit der gleichen Wirkung wie `/rename`. Verwenden Sie dies, um Sitzungen automatisch aus dem Start-Ordner, Git-Branch oder Worktree-Namen zu benennen. Gilt, wenn `source` `"startup"`, `"resume"` oder `"fork"` ist; wird bei `"clear"` und `"compact"` ignoriert                                                                                                                    |
| `watchPaths`         | Array von absoluten Pfaden zum Überwachen von [FileChanged](#filechanged) Ereignissen während dieser Sitzung                                                                                                                                                                                                                                                                                                       |
| `reloadSkills`       | Boolean. Wenn `true`, scannt Claude Code die [Skill](/docs/de/skills)- und Befehlsverzeichnisse erneut, nachdem die SessionStart-Hooks abgeschlossen sind, sodass Skills, die der Hook installiert hat, in der gleichen Sitzung verfügbar sind, beginnend mit der ersten Eingabeaufforderung                                                                                                                            |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Da Klartext-stdout bereits dieses Ereignis für Claude erreicht, kann ein Hook, der nur Kontext lädt, direkt zu stdout drucken, ohne JSON zu erstellen. Verwenden Sie die JSON-Form, wenn Sie Kontext mit anderen Feldern wie `sessionTitle` kombinieren müssen.

Verwenden Sie `reloadSkills`, wenn ein SessionStart-Hook Skills installiert oder aktualisiert. Die Skill-Erkennung wird normalerweise ausgeführt, bevor SessionStart-Hooks fertig sind, daher würden Dateien, die der Hook in `~/.claude/skills/` oder `.claude/skills/` schreibt, sonst erst in der nächsten Sitzung angezeigt. Dieses Beispiel synchronisiert ein gemeinsames Skills-Repository und fordert die erneute Überprüfung an:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

Die Repository-URL ist ein Platzhalter; ersetzen Sie sie durch Ihr eigenes Skills-Repository. Mit dem Platzhalter schlägt der Klon fehl und druckt eine `fatal:` Nachricht zu stderr. Stderr von einem SessionStart-Hook, der mit 0 beendet wird, ist nur informativ, daher gilt die `reloadSkills` Anfrage immer noch.

<h4 id="persist-environment-variables">
  Umgebungsvariablen beibehalten
</h4>

SessionStart-Hooks haben Zugriff auf die `CLAUDE_ENV_FILE` Umgebungsvariable, die einen Dateipfad bereitstellt, in dem Sie Umgebungsvariablen für nachfolgende Bash-Befehle beibehalten können.

Um einzelne Umgebungsvariablen zu setzen, schreiben Sie `export` Anweisungen in `CLAUDE_ENV_FILE`. Verwenden Sie Anhängen (`>>`), um von anderen Hooks gesetzte Variablen zu bewahren:

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Um alle Umgebungsänderungen von Setup-Befehlen zu erfassen, vergleichen Sie die exportierten Variablen vorher und nachher:

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# Run your setup commands that modify the environment
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

<Note>
  `CLAUDE_ENV_FILE` ist für SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged) und [FileChanged](#filechanged) Hooks verfügbar. Andere Hook-Typen haben keinen Zugriff auf diese Variable.
</Note>

<h3 id="setup">
  Setup
</h3>

Wird nur ausgeführt, wenn Sie Claude Code mit `--init-only` starten oder mit `--init` oder `--maintenance` im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p` Flag. Es wird beim normalen Start nicht ausgeführt. Verwenden Sie es für einmalige Abhängigkeitsinstallation oder geplante Bereinigung, die Sie explizit von CI oder Skripten aus auslösen, getrennt vom normalen Sitzungsstart. Für die Initialisierung pro Sitzung verwenden Sie stattdessen [SessionStart](#sessionstart).

Der Matcher-Wert entspricht dem CLI-Flag, das den Hook ausgelöst hat:

| Matcher       | Wann wird es ausgelöst                       |
| :------------ | :------------------------------------------- |
| `init`        | `claude --init-only` oder `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                    |

Wenn Sie `claude --init-only` ausführen, führt Claude Code Setup-Hooks und `SessionStart` Hooks mit dem `startup` Matcher aus und beendet sich dann, ohne ein Gespräch zu starten.

Wenn Sie ein Gespräch mit `-p` starten oder fortsetzen, müssen Sie auch eine Eingabeaufforderung bereitstellen, als Argument oder über stdin weitergeleitet. Sie können die Eingabeaufforderung überspringen, wenn ein `SessionStart` Hook [`initialUserMessage`](#sessionstart-decision-control) bereitstellt oder wenn Sie eine Sitzung mit einem [aufgeschobenen Tool-Aufruf](#defer-a-tool-call-for-later) fortsetzen.

Bei Erfolg druckt `--init-only` nichts auf das Terminal. Um zu bestätigen, dass die Hooks ausgeführt wurden, starten Sie mit `claude --debug-file <path> --init-only`, ersetzen Sie `<path>` durch einen Protokolldateispeicherort, und überprüfen Sie das Protokoll auf die Setup- und SessionStart-Hook-Einträge.

Da Setup nicht bei jedem Start ausgeführt wird, kann sich ein Plugin, das eine Abhängigkeit installiert benötigt, nicht nur auf Setup verlassen. Das praktische Muster ist, die Abhängigkeit bei der ersten Verwendung zu überprüfen und bei Fehlen zu installieren, z. B. ein Hook oder Skill, der auf `${CLAUDE_PLUGIN_DATA}/node_modules` testet und `npm install` ausführt, wenn nicht vorhanden. Siehe das [persistente Datenverzeichnis](/docs/de/plugins/components#path-variables-and-persistent-data), um zu erfahren, wo installierte Abhängigkeiten gespeichert werden. Wenn Sie Ihr Plugin über einen Marketplace verteilen, benötigen Sie möglicherweise dieses Muster nicht: Claude Code [installiert automatisch berechtigte Node.js-Paketabhängigkeiten](/docs/de/plugins/loading#node-js-package-dependencies), wenn es das Plugin zwischenspeichert.

<h4 id="setup-input">
  Setup-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten Setup-Hooks ein `trigger` Feld, das auf `"init"` oder `"maintenance"` gesetzt ist:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Setup",
  "trigger": "init"
}
```

<h4 id="setup-decision-control">
  Setup-Entscheidungskontrolle
</h4>

Setup-Hooks können nicht blockieren; die Ausführung wird bei jedem Exit-Code fortgesetzt. Bei jedem Exit-Code verwirft Claude Code die [JSON-Ausgabefelder](#json-output) eines Setup-Hooks, wie `systemMessage`, `continue` und `hookSpecificOutput.additionalContext`. Mit `-p` werden stdout, stderr und Exit-Code eines Setup-Hooks in der Ausgabe des Laufs nur als [`hook_response` Ereignisse](/docs/de/headless#read-session-metadata) angezeigt, wenn Sie mit `--output-format stream-json --verbose` starten.

Setup-Hooks haben Zugriff auf `CLAUDE_ENV_FILE`. Variablen, die in diese Datei geschrieben werden, bleiben in nachfolgenden Bash-Befehlen für die Sitzung bestehen, genau wie in [SessionStart-Hooks](#persist-environment-variables). Nur `type: "command"` Hooks werden auf `Setup` ausgeführt. Ein `type: "mcp_tool"` Hook auf `Setup` wird immer übersprungen, wie unter [MCP-Tool-Hook-Felder](#mcp-tool-hook-fields) beschrieben.

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Wird ausgeführt, wenn eine `CLAUDE.md` oder `.claude/rules/*.md` Datei in den Kontext geladen wird. Dieses Ereignis wird beim Sitzungsstart für eifrig geladene Dateien ausgeführt und später erneut, wenn Dateien träge geladen werden, z. B. wenn Claude auf ein Unterverzeichnis zugreift, das eine verschachtelte `CLAUDE.md` enthält, oder wenn bedingte Regeln mit `paths:` Frontmatter übereinstimmen. Der Hook unterstützt keine Blockierung oder Entscheidungskontrolle. Er wird asynchron zu Beobachtungszwecken ausgeführt.

Dieses Ereignis wird nicht ausgeführt, wenn Claude [direkt `AGENTS.md` liest](/docs/de/memory#agents-md) über die Einstellung **Projektanweisungen**. Es wird ausgeführt, wenn eine `CLAUDE.md` Ihre `AGENTS.md` importiert, mit `load_reason` auf `include` gesetzt wie für jede andere importierte Datei, und wenn `CLAUDE.md` ein Symlink zu ihr ist, als normales `CLAUDE.md` Laden.

Der Matcher wird gegen `load_reason` ausgeführt. Verwenden Sie beispielsweise `"matcher": "session_start"`, um nur für Dateien zu aktivieren, die beim Sitzungsstart geladen werden, oder `"matcher": "path_glob_match|nested_traversal"`, um nur für träge Ladevorgänge zu aktivieren.

<h4 id="instructionsloaded-input">
  InstructionsLoaded-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten InstructionsLoaded-Hooks diese Felder:

| Feld                | Beschreibung                                                                                                                                                                                                                                    |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Absoluter Pfad zur Anweisungsdatei, die geladen wurde                                                                                                                                                                                           |
| `memory_type`       | Umfang der Datei: `"User"`, `"Project"`, `"Local"` oder `"Managed"`                                                                                                                                                                             |
| `load_reason`       | Warum die Datei geladen wurde: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"` oder `"compact"`. Der `"compact"` Wert wird ausgeführt, wenn Anweisungsdateien nach einem Komprimierungsereignis erneut geladen werden |
| `globs`             | Pfad-Glob-Muster aus dem `paths:` Frontmatter der Datei, falls vorhanden. Nur für `path_glob_match` Ladevorgänge vorhanden                                                                                                                      |
| `trigger_file_path` | Pfad zur Datei, deren Zugriff diesen Ladevorgang ausgelöst hat, für träge Ladevorgänge                                                                                                                                                          |
| `parent_file_path`  | Pfad zur übergeordneten Anweisungsdatei, die diese eingebunden hat, für `include` Ladevorgänge                                                                                                                                                  |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "InstructionsLoaded",
  "file_path": "/Users/my-project/CLAUDE.md",
  "memory_type": "Project",
  "load_reason": "session_start"
}
```

<h4 id="instructionsloaded-decision-control">
  InstructionsLoaded-Entscheidungskontrolle
</h4>

InstructionsLoaded-Hooks haben keine Entscheidungskontrolle. Sie können das Laden von Anweisungen nicht blockieren oder ändern. Claude Code verwirft ihre [JSON-Ausgabefelder](#json-output), wie `systemMessage` und `continue`. Verwenden Sie dieses Ereignis für Audit-Protokollierung, Compliance-Verfolgung oder Beobachtbarkeit.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Wird ausgeführt, wenn der Benutzer eine Eingabeaufforderung einreicht, bevor Claude sie verarbeitet. Dies ermöglicht es Ihnen, zusätzlichen Kontext basierend auf der Eingabeaufforderung/dem Gespräch hinzuzufügen, Eingabeaufforderungen zu validieren oder bestimmte Arten von Eingabeaufforderungen zu blockieren.

`UserPromptSubmit` Hooks haben ein Standard-Timeout von 30 Sekunden für `command`, `http` und `mcp_tool` Typen, kürzer als das 600-Sekunden-Standard für diese Typen bei den meisten anderen Ereignissen. Da dieser Hook vor jeder Eingabeaufforderung ausgeführt wird und die Modellverarbeitung blockiert, bis er abgeschlossen ist, stellt ein feststeckender Hook die Sitzung still. Wenn Ihr Hook mehr Zeit benötigt, setzen Sie das `timeout` Feld im Hook-Eintrag.

Abgesehen von einem Command-Hook, den Sie mit [`async: true`](#run-hooks-in-the-background) ausführen, wird ein `UserPromptSubmit` Command-, HTTP- oder MCP-Tool-Hook, der sein Timeout erreicht, abgebrochen und seine Ausgabe, einschließlich `additionalContext`, wird verworfen. Die Eingabeaufforderung erreicht Claude immer noch ohne diesen Kontext. Das Transkript zeigt einen Hinweis, der den Hook, das ausgelöste Timeout und dass die Ausgabe verworfen wurde, benennt.

Ein [Agent SDK Callback-Hook](/docs/de/agent-sdk/hooks) auf `UserPromptSubmit`, der sein Timeout erreicht, blockiert die Eingabeaufforderung mit einer Nachricht, die den Hook und das Timeout benennt, da ein Callback dort als Richtlinientor fungieren kann, das nicht offen fehlschlagen darf. Die Sitzung wird fortgesetzt. Vor v2.1.208 endete ein Callback-Timeout bei diesem Ereignis mit einem Ausführungsfehler.

<h4 id="userpromptsubmit-input">
  UserPromptSubmit-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten UserPromptSubmit-Hooks das `prompt` Feld mit dem Text, den der Benutzer eingereicht hat. Eingefügter Inhalt, der zu einem `[Pasted text #N]` Platzhalter zusammengefallen ist, kommt erweitert an seiner Stelle an. In Sitzungen, in denen Claude Code [eingefügten Text für Claude markiert](/docs/de/terminal-config#how-claude-treats-pasted-text), sitzt dieser erweiterte Inhalt zwischen einer `<pasted_content id="…">` Zeile und einer `</pasted_content id="…">` Zeile, daher berücksichtigen Sie diese Zeilen, wenn Ihr Hook die Eingabeaufforderung analysiert.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

<h4 id="userpromptsubmit-decision-control">
  UserPromptSubmit-Entscheidungskontrolle
</h4>

`UserPromptSubmit` Hooks können steuern, ob eine Benutzereingabeaufforderung verarbeitet wird und Kontext hinzufügen. Alle [JSON-Ausgabefelder](#json-output) sind verfügbar.

Es gibt zwei Möglichkeiten, Kontext zum Gespräch bei Exit-Code 0 hinzuzufügen:

* **Klartext-stdout**: Claude Code fügt stdout, das es [als Klartext behandelt](#exit-code-0), zu Claudes Kontext hinzu
* **JSON mit `additionalContext`**: Verwenden Sie das JSON-Format unten für mehr Kontrolle. Das `additionalContext` Feld wird als Kontext hinzugefügt

Keiner der Kanäle erzeugt einen sichtbaren Transkript-Eintrag. Klartext-stdout und der `additionalContext` Wert werden jeweils als Systemerinnerung eingefügt, die mit dem Namen des Hooks beginnt; Claude liest beide. Um die Bereitstellung zu bestätigen, überprüfen Sie das [Debug-Protokoll](#debug-hooks).

Um eine Eingabeaufforderung zu blockieren, geben Sie ein JSON-Objekt mit `decision` auf `"block"` zurück:

| Feld                     | Beschreibung                                                                                                                                                |
| :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`               | `"block"` verhindert, dass die Eingabeaufforderung verarbeitet wird und löscht sie aus dem Kontext. Weglassen, um die Eingabeaufforderung fortzufahren      |
| `reason`                 | Wird dem Benutzer angezeigt, wenn `decision` `"block"` ist. Nicht zum Kontext hinzugefügt                                                                   |
| `additionalContext`      | String, der zu Claudes Kontext neben der eingereichten Eingabeaufforderung hinzugefügt wird. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude) |
| `sessionTitle`           | Legt den Sitzungstitel fest. Verwenden Sie dies, um Sitzungen automatisch basierend auf dem Inhalt der Eingabeaufforderung zu benennen                      |
| `suppressOriginalPrompt` | Wenn `true`, wenn `decision` `"block"` ist, wird der ursprüngliche Eingabeaufforderungstext aus der angezeigten Blockierungsmeldung weggelassen             |

Ein Hook, der durch Beendigung mit 2 blockiert, wird auf die gleiche Weise wie `reason` weitergeleitet: Die Blockierungsmeldung zeigt dem Benutzer den stderr-Text an, und er wird nicht zum Kontext hinzugefügt.

```json theme={null}
{
  "decision": "block",
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here",
    "sessionTitle": "My session title"
  }
}
```

<h3 id="userpromptexpansion">
  UserPromptExpansion
</h3>

Wird ausgeführt, wenn ein vom Benutzer eingegebener Befehl vor dem Erreichen von Claude in eine Eingabeaufforderung erweitert wird. Verwenden Sie dies, um bestimmte Befehle von direkter Aufrufen zu blockieren, Kontext für einen bestimmten Skill einzufügen oder zu protokollieren, welche Befehle Benutzer aufrufen. Beispielsweise kann ein Hook, der `deploy` abgleicht, `/deploy` blockieren, es sei denn, eine Genehmigungsdatei ist vorhanden, oder ein Hook, der einen Review-Skill abgleicht, kann die Review-Checkliste des Teams als `additionalContext` anhängen.

Dieses Ereignis deckt den Pfad ab, den `PreToolUse` nicht abdeckt: Ein `PreToolUse` Hook, der das `Skill` Tool abgleicht, wird nur ausgeführt, wenn Claude das Tool aufruft, aber das direkte Eingeben von `/skillname` umgeht `PreToolUse`. `UserPromptExpansion` wird auf diesem direkten Pfad ausgeführt.

Gleicht `command_name` ab. Lassen Sie den Matcher leer, um bei jedem Eingabeaufforderungs-Typ-Befehl zu aktivieren.

<h4 id="userpromptexpansion-input">
  UserPromptExpansion-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten UserPromptExpansion-Hooks `expansion_type`, `command_name`, `command_args`, `command_source` und die ursprüngliche `prompt` Zeichenkette. Das `expansion_type` Feld ist `slash_command` für Skill- und benutzerdefinierte Befehle oder `mcp_prompt` für MCP-Server-Eingabeaufforderungen.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../00893aaf.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptExpansion",
  "expansion_type": "slash_command",
  "command_name": "example-skill",
  "command_args": "arg1 arg2",
  "command_source": "plugin",
  "prompt": "/example-skill arg1 arg2"
}
```

<h4 id="userpromptexpansion-decision-control">
  UserPromptExpansion-Entscheidungskontrolle
</h4>

`UserPromptExpansion` Hooks können die Erweiterung blockieren oder Kontext hinzufügen. Alle [JSON-Ausgabefelder](#json-output) sind verfügbar.

| Feld                | Beschreibung                                                                                                                                              |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` verhindert, dass der Befehl erweitert wird. Weglassen, um fortzufahren                                                                          |
| `reason`            | Wird dem Benutzer angezeigt, wenn `decision` `"block"` ist                                                                                                |
| `additionalContext` | String, der zu Claudes Kontext neben der erweiterten Eingabeaufforderung hinzugefügt wird. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude) |

Ein Hook, der durch Beendigung mit 2 blockiert, wird auf die gleiche Weise wie `reason` weitergeleitet: Die Blockierungsmeldung zeigt dem Benutzer den stderr-Text an.

```json theme={null}
{
  "decision": "block",
  "reason": "This slash command is not available",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptExpansion",
    "additionalContext": "Additional context for this expansion"
  }
}
```

<h3 id="messagedisplay">
  MessageDisplay
</h3>

Wird ausgeführt, während eine Assistenten-Nachricht auf den Bildschirm gestreamt wird. Claude Code zeigt die Nachricht in Inkrementen an: Jedes Mal, wenn ein Batch neu fertiggestellter Zeilen zum Rendern bereit ist, wird der Hook einmal mit diesen Zeilen ausgeführt und Claude Code rendert den Ersatztext des Hooks an ihrer Stelle. Eine lange Nachricht erzeugt mehrere Aufrufe; eine kurze Nachricht kann nur einen erzeugen.

Verwenden Sie MessageDisplay für:

* Markdown für eine minimale Anzeige entfernen
* Den Text transformieren, den eine Agent SDK Anwendung ihren Benutzern zeigt
* API-Schlüssel oder interne Hostnamen aus Claudes Antworten redigieren

Claude Code hält jeden Batch, bis Ihr Hook zurückkommt, daher halten Sie den Hook schnell. Wenn der Hook fehlschlägt oder das Timeout überschreitet, zeigt Claude Code den ursprünglichen Text an. Das Standard-Timeout für dieses Ereignis beträgt 10 Sekunden; wenn Ihr Hook mehr Zeit benötigt, setzen Sie das `timeout` Feld im Hook-Eintrag.

MessageDisplay ist nur für die Anzeige: Der Ersatztext ändert nur das, was auf dem Bildschirm gerendert wird. Das Transkript und das, was Claude sieht, behalten den ursprünglichen Text, daher sieht Claude den Ersatz nie, und der ausführliche Modus zeigt das Original. Der Hook empfängt nur Assistenten-Nachrichtentext, daher werden Tool-Ergebnisse und der Text, den Sie eingeben, unverändert gerendert.

MessageDisplay unterstützt keine Matcher und wird für jede Assistenten-Nachricht ausgeführt, die Text streamt; Nachrichten ohne Text, wie nur Tool-Aufrufe, lösen es nicht aus.

In nicht-interaktiven Läufen, einschließlich Agent SDK Abfragen und `claude -p`, wird MessageDisplay einmal pro Assistenten-Nachricht statt einmal pro Batch von Zeilen ausgeführt. Der einzelne Aufruf kommt nach Abschluss der Nachricht an und trägt den vollständigen Nachrichtentext: `index` ist `0`, `final` ist `true` und `delta` enthält die gesamte Nachricht. Ein Hook, der den `delta` Text für jede Nachricht sammelt, empfängt den gleichen Gesamttext in beiden Modi.

<h4 id="messagedisplay-input">
  MessageDisplay-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten MessageDisplay-Hooks Identifikatoren für den Zug und die Nachricht, die Position dieses Aufrufs innerhalb der Nachricht und den neuen Text in `delta`. Batch-Grenzen hängen davon ab, wie der Text streamt, daher verwenden Sie `index` und `final`, um den Fortschritt durch eine Nachricht zu verfolgen, anstatt zu erwarten, dass Zeilen auf eine bestimmte Weise gruppiert werden.

| Feld         | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID des aktuellen Zugs                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `message_id` | UUID der angezeigten Assistenten-Nachricht. Stabil über jeden Batch der gleichen Nachricht. Dies ist nicht die API `msg_…` ID, daher kann sie nicht mit Transkript-Nachrichten-IDs korreliert werden                                                                                                                                                                                                                                                                                          |
| `index`      | Null-basierter Index dieses Batches innerhalb der Nachricht                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `final`      | `true` beim letzten Batch der Nachricht. Jede Nachricht hat genau einen finalen Batch                                                                                                                                                                                                                                                                                                                                                                                                         |
| `delta`      | Die neu fertiggestellten Zeilen seit dem vorherigen Batch, einschließlich abschließender Zeilenumbrüche. Immer ganze Zeilen, außer dem finalen Batch, der in der Mitte einer Zeile enden kann. In interaktiven Läufen ist das Delta des finalen Batches leer, wenn die Nachricht mit einem Zeilenumbruch endet, daher behandeln Sie `final`, nicht ein nicht-leeres Delta, als das End-of-Message-Signal. In Agent SDK und `claude -p` Läufen trägt der einzelne Aufruf die gesamte Nachricht |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "MessageDisplay",
  "turn_id": "0c9e6a2f-7d41-4f4e-9a15-3f4f7c2b8d10",
  "message_id": "5b2a9c8e-1f63-4d8a-b7c4-9e0d2a6f1c3b",
  "index": 0,
  "final": false,
  "delta": "Here is the plan:\n"
}
```

<h4 id="messagedisplay-output">
  MessageDisplay-Ausgabe
</h4>

Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, können MessageDisplay-Hooks `displayContent` zurückgeben, um das Delta auf dem Bildschirm zu ersetzen:

| Feld             | Beschreibung                                                                       |
| :--------------- | :--------------------------------------------------------------------------------- |
| `displayContent` | Text, der anstelle des Delta angezeigt wird. Weglassen, um das Original anzuzeigen |

MessageDisplay-Hooks haben keine Entscheidungskontrolle. Sie können die Nachricht nicht blockieren oder ändern, was im Transkript gespeichert oder an Claude gesendet wird. Claude Code handelt `displayContent` aus ihrer JSON-Ausgabe und verwirft `systemMessage` und `continue`.

Dieses Beispiel entfernt Markdown-Formatierung aus Claudes Antworten für eine Klartext-Anzeige. Das Skript liest jeden Batch von stdin, entfernt fette Marker und Inline-Code-Backticks aus `delta` und gibt das Ergebnis als `displayContent` zurück.

<Tabs>
  <Tab title="macOS/Linux">
    Registrieren Sie einen Command-Hook für das Ereignis in Ihrer Einstellungsdatei:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Speichern Sie dieses Skript unter `.claude/hooks/plain-display.sh` in Ihrem Projekt und machen Sie es mit `chmod +x` ausführbar:

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registrieren Sie einen Command-Hook, der das Skript durch PowerShell ausführt:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Das `-NoProfile` Flag überspringt das Laden Ihres PowerShell-Profils, damit der Hook schnell startet, und `-ExecutionPolicy Bypass` ermöglicht PowerShell, die lokale Skriptdatei auszuführen.

    Speichern Sie dieses Skript unter `.claude/hooks/plain-display.ps1` in Ihrem Projekt:

    ```powershell theme={null}
    $batch = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $text = $batch.delta -replace '\*\*', '' -replace '`', ''
    @{
      hookSpecificOutput = @{
        hookEventName = "MessageDisplay"
        displayContent = $text
      }
    } | ConvertTo-Json
    ```
  </Tab>
</Tabs>

Batches ohne Markdown werden unverändert durchgelassen. Wenn das Skript fehlschlägt, z. B. weil `jq` fehlt, zeigt Claude Code den ursprünglichen Text an und notiert den Fehler nur in der [Debug-Ausgabe](#debug-hooks), nicht in der Sitzung.

<h3 id="pretooluse">
  PreToolUse
</h3>

Wird ausgeführt, nachdem Claude Tool-Parameter erstellt hat und bevor der Tool-Aufruf verarbeitet wird. Gleicht jeden Tool-Namen außer `EndConversation` ab: integrierte Tools wie `Bash`, `PowerShell`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `Workflow`, `WebFetch`, `WebSearch`, `AskUserQuestion` und `ExitPlanMode` sowie alle [MCP-Tool-Namen](#match-mcp-tools).

Um einen Hook auszuführen, wenn sich eine bestimmte Datei auf der Festplatte ändert, unabhängig davon, wer sie schreibt, verwenden Sie stattdessen [FileChanged](#filechanged). Im Gegensatz zu PreToolUse führt Claude Code FileChanged-Hooks nach der Änderung aus, und sie haben keine Entscheidungskontrolle, daher können sie den Schreibvorgang nicht blockieren.

<Warning>
  PreToolUse wird nur ausgeführt, wenn Claude ein Tool aufruft. Dateien, die Sie [mit `@` in Ihrer Eingabeaufforderung referenzieren](/docs/de/common-workflows#reference-files-and-directories), werden hinzugefügt, ohne dass ein Tool-Aufruf erfolgt: Claude Code fügt ihren Inhalt beim Erstellen der Eingabeaufforderung ein, daher wird kein PreToolUse-Hook für sie ausgeführt, einschließlich Hooks, die `Read` abgleichen. Um bestimmte Pfade von `@` Referenzen zu blockieren, verwenden Sie stattdessen eine [`Read` Ablehnungsregel](/docs/de/permissions#read-and-edit).

  PreToolUse wird auch nicht für [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior) ausgeführt.
</Warning>

Verwenden Sie [PreToolUse-Entscheidungskontrolle](#pretooluse-decision-control), um den Tool-Aufruf zu erlauben, zu verweigern, zu fragen oder aufzuschieben.

Ein [Agent SDK Callback-Hook](/docs/de/agent-sdk/hooks) auf `PreToolUse`, der sein Timeout überschreitet, blockiert den Tool-Aufruf, und Claude erhält ein Fehlerergebnis, das das Timeout benennt. Eine explizite Ablehnung, die von einem anderen Hook zurückgegeben wird, hat immer noch Vorrang.

<h4 id="pretooluse-input">
  PreToolUse-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten PreToolUse-Hooks `tool_name`, `tool_input` und `tool_use_id`.

Für ein [MCP-Tool](#match-mcp-tools) trägt die Eingabe auch `mcp_server`, ein Objekt mit dem Namen des Servers und einer `source`, die angibt, woher die Definition des Servers stammt. Die `source` Werte umfassen `plugin`, `sdk` und Konfigurationsbereiche wie `user` und `project`. [`McpServerProvenance`](/docs/de/agent-sdk/typescript#mcpserverprovenance) in der Agent SDK Referenz listet sie alle auf und sagt, wie man eine behandelt, die man nicht erkennt. Treffen Sie Vertrauensentscheidungen basierend auf `source` statt auf `name` oder dem `mcp__<server>__` Tool-Namen-Präfix. Das `mcp_server` Feld erfordert Claude Code v2.1.274 oder später.

Für die Datei-Tools `Write`, `Edit` und `Read` ist `tool_input.file_path` immer absolut:

* Claude Code erweitert `~` und relative Pfade, bevor Hooks ausgeführt werden, daher kann ein Hook, der auf Pfaden abgleicht, nicht durch `~` oder eine relative Schreibweise des gleichen Pfads umgangen werden
* Unter Windows kommt der Pfad mit Backslash-Trennzeichen an, auch wenn Ihr Hook unter Git Bash läuft, wo `$PWD` wie `/c/project` aussieht
* Ein Vergleich mit Schrägstrichen, wie eine `/src/` Überprüfung, gleicht nie einen Backslash-Pfad ab, und der Tool-Aufruf wird fortgesetzt, als hätte der Hook nichts zu blockieren
* Normalisieren Sie Trennzeichen vor dem Vergleich: `FILE_PATH="${FILE_PATH//\\//}"` in Bash oder `file_path.replace("\\", "/")` in Python, dann gleichen Sie ein Pfad-Segment wie `/src/` ab, anstatt mit `^` zu verankern, da der Pfad absolut ist

Ein `Write` Aufruf unter Windows liefert:

```json theme={null}
{
  "hook_event_name": "PreToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "C:\\project\\src\\index.ts",
    "content": "..."
  },
  ...
}
```

Die `tool_input` Felder hängen vom Tool ab:

<a id="bash" />

<h5 id="bash">
  Bash
</h5>

Führt Shell-Befehle aus.

| Feld                | Typ     | Beispiel           | Beschreibung                                                                                                                                                        |
| :------------------ | :------ | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `command`           | string  | `"npm test"`       | Der auszuführende Shell-Befehl                                                                                                                                      |
| `description`       | string  | `"Run test suite"` | Optionale Beschreibung, was der Befehl tut                                                                                                                          |
| `timeout`           | number  | `120000`           | Optionales Timeout in Millisekunden. Werte über dem [Maximum](/docs/de/tools-reference#bash-tool-behavior) werden auf das Maximum reduziert, anstatt abgelehnt zu werden |
| `run_in_background` | boolean | `false`            | Ob der Befehl im Hintergrund ausgeführt werden soll                                                                                                                 |

Wenn ein Bash-Befehl Dateien in einem Git-Repository ändert, kann Claude Code aufzeichnen, was sich geändert hat. Es zeichnet die Änderungen in jedem Berechtigungsmodus auf, wenn die [`bashEditDiffEnabled`](/docs/de/settings-reference#basheditdiffenabled) Einstellung die Aufzeichnung aktiviert; der Eintrag dieser Einstellung sagt, welche Dateien sie setzen können. Andernfalls zeichnet es sie nur im Auto-Modus und `bypassPermissions` Modus auf, und nur wenn Claude Code Claude anweist, Dateien durch Bash zu bearbeiten. Setzen Sie `bashEditDiffEnabled` auf `false`, um die Aufzeichnung auszuschalten. Hintergrund-Befehle und schreibgeschützte Befehle tragen keinen Diff.

Ihr [PostToolUse-Hook](#posttooluse) empfängt dann die geänderten Dateien in `tool_response.bashEditDiff`. Die Liste deckt ab, was sich im Repository geändert hat, während der Befehl lief. Dateien, die Git ignoriert, und Dateien in Submodulen werden nicht aufgelistet. Erfordert Claude Code v2.1.269 oder später.

<Note>
  Die Liste ist Best-Effort und in öffentlicher Beta. Claude Code kann eine Änderung verpassen, eine Datei einschließen, die ein anderer Prozess gleichzeitig geändert hat, oder bei seinen Größenlimits stoppen. Die Feldform kann sich ändern. Verwenden Sie die Liste, um zu finden, was zu überprüfen ist, nicht um eine Richtlinie durchzusetzen.
</Note>

`changedFiles` und `files` listen auf, was der Befehl geändert hat; die verbleibenden Felder sagen, wie vollständig und wie zuverlässig diese Liste ist.

| Feld           | Typ     | Beispiel                                                | Beschreibung                                                                                                                                                                            |
| :------------- | :------ | :------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `changedFiles` | array   | `["/path/to/src/app.ts"]`                               | Absolute Pfade der Dateien, die der Befehl geändert hat, höchstens 200. Vorhanden, wenn `files` einen Diff enthält oder `moreFiles` über Null liegt                                     |
| `files`        | array   | `[{"filePath": "/path/to/src/app.ts", "hunks": [...]}]` | Diffs von bis zu 5 geänderten Dateien zur Anzeige. `created` oder `deleted` ist `true` für eine Datei, die der Befehl hinzugefügt oder entfernt hat                                     |
| `moreFiles`    | number  | `2`                                                     | Anzahl der geänderten Dateien ohne Diff in `files`                                                                                                                                      |
| `unavailable`  | boolean | `true`                                                  | Gesetzt, wenn der Diff unvollständig ist oder nicht genommen werden konnte                                                                                                              |
| `skipped`      | boolean | `true`                                                  | Gesetzt für einen Git-Befehl, der den Arbeitsbaum bewegt, wie `git checkout` oder `git stash`, daher nimmt Claude Code keinen Diff                                                      |
| `shared`       | boolean | `true`                                                  | Gesetzt, wenn ein anderer Bash-Tool-Aufruf, wie der eines Subagenten, im gleichen Repository zur gleichen Zeit lief, daher können einige aufgelistete Änderungen von diesem Befehl sein |

<a id="powershell" />

<h5 id="powershell">
  PowerShell
</h5>

Führt PowerShell-Befehle aus. Siehe das [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) für Verfügbarkeit nach Plattform.

Die Felder entsprechen dem Bash-Tool, mit der Befehlszeichenkette in `command`:

| Feld                | Typ     | Beispiel                   | Beschreibung                                        |
| :------------------ | :------ | :------------------------- | :-------------------------------------------------- |
| `command`           | string  | `"Get-ChildItem -Recurse"` | Der auszuführende PowerShell-Befehl                 |
| `description`       | string  | `"List files recursively"` | Optionale Beschreibung, was der Befehl tut          |
| `timeout`           | number  | `120000`                   | Optionales Timeout in Millisekunden                 |
| `run_in_background` | boolean | `false`                    | Ob der Befehl im Hintergrund ausgeführt werden soll |

Gleichen Sie `Bash|PowerShell` in Hooks ab, die Shell-Befehle überprüfen, damit sie beide Tools abdecken:

* Unter Windows, überall wo das PowerShell-Tool aktiviert ist, behandelt Claude PowerShell als die primäre Shell und leitet Shell-Befehle durch sie.
* Unter Windows ohne Git Bash ist das Tool automatisch aktiviert und Claude Code registriert das Bash-Tool überhaupt nicht.
* Ein Hook, der nur `Bash` abgleicht, wird dort nie ausgeführt.

<h5 id="write">
  Write
</h5>

Erstellt oder überschreibt eine Datei.

| Feld        | Typ    | Beispiel              | Beschreibung                             |
| :---------- | :----- | :-------------------- | :--------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Absoluter Pfad zur zu schreibenden Datei |
| `content`   | string | `"file content"`      | Inhalt zum Schreiben in die Datei        |

<h5 id="edit">
  Edit
</h5>

Ersetzt eine Zeichenkette in einer vorhandenen Datei.

| Feld          | Typ     | Beispiel              | Beschreibung                              |
| :------------ | :------ | :-------------------- | :---------------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Absoluter Pfad zur zu bearbeitenden Datei |
| `old_string`  | string  | `"original text"`     | Text zum Suchen und Ersetzen              |
| `new_string`  | string  | `"replacement text"`  | Ersatztext                                |
| `replace_all` | boolean | `false`               | Ob alle Vorkommen ersetzt werden sollen   |

<h5 id="read">
  Read
</h5>

Liest Dateiinhalte.

| Feld        | Typ    | Beispiel              | Beschreibung                                  |
| :---------- | :----- | :-------------------- | :-------------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Absoluter Pfad zur zu lesenden Datei          |
| `offset`    | number | `10`                  | Optionale Zeilennummer zum Starten des Lesens |
| `limit`     | number | `50`                  | Optionale Anzahl der zu lesenden Zeilen       |

<h5 id="glob">
  Glob
</h5>

Findet Dateien, die einem Glob-Muster entsprechen.

| Feld      | Typ    | Beispiel         | Beschreibung                                                                       |
| :-------- | :----- | :--------------- | :--------------------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Glob-Muster zum Abgleichen von Dateien                                             |
| `path`    | string | `"/path/to/dir"` | Optionales Verzeichnis zum Durchsuchen. Standardmäßig aktuelles Arbeitsverzeichnis |

<h5 id="grep">
  Grep
</h5>

Durchsucht Dateiinhalte mit regulären Ausdrücken.

| Feld          | Typ     | Beispiel         | Beschreibung                                                                             |
| :------------ | :------ | :--------------- | :--------------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Muster für reguläre Ausdrücke zum Suchen                                                 |
| `path`        | string  | `"/path/to/dir"` | Optionale Datei oder Verzeichnis zum Durchsuchen                                         |
| `glob`        | string  | `"*.ts"`         | Optionales Glob-Muster zum Filtern von Dateien                                           |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"` oder `"count"`. Standardmäßig `"files_with_matches"` |
| `-i`          | boolean | `true`           | Groß-/Kleinschreibung ignorieren                                                         |
| `multiline`   | boolean | `false`          | Mehrzeilige Übereinstimmung aktivieren                                                   |

<h5 id="webfetch">
  WebFetch
</h5>

Ruft Web-Inhalte ab und verarbeitet sie.

| Feld     | Typ    | Beispiel                      | Beschreibung                                                 |
| :------- | :----- | :---------------------------- | :----------------------------------------------------------- |
| `url`    | string | `"https://example.com/api"`   | URL zum Abrufen von Inhalten                                 |
| `prompt` | string | `"Extract the API endpoints"` | Eingabeaufforderung zum Ausführen auf dem abgerufenen Inhalt |

<h5 id="websearch">
  WebSearch
</h5>

Durchsucht das Web.

| Feld              | Typ    | Beispiel                       | Beschreibung                                             |
| :---------------- | :----- | :----------------------------- | :------------------------------------------------------- |
| `query`           | string | `"react hooks best practices"` | Suchanfrage                                              |
| `allowed_domains` | array  | `["docs.example.com"]`         | Optional: Nur Ergebnisse von diesen Domains einschließen |
| `blocked_domains` | array  | `["spam.example.com"]`         | Optional: Ergebnisse von diesen Domains ausschließen     |

<h5 id="agent">
  Agent
</h5>

Spawnt einen [Subagenten](/docs/de/sub-agents).

| Feld            | Typ    | Beispiel                   | Beschreibung                                            |
| :-------------- | :----- | :------------------------- | :------------------------------------------------------ |
| `prompt`        | string | `"Find all API endpoints"` | Die Aufgabe für den Agenten                             |
| `description`   | string | `"Find API endpoints"`     | Kurze Beschreibung der Aufgabe                          |
| `subagent_type` | string | `"Explore"`                | Typ des zu verwendenden spezialisierten Agenten         |
| `model`         | string | `"sonnet"`                 | Optionaler Modell-Alias zum Überschreiben des Standards |

Wenn ein Vordergrund-Agent-Aufruf abgeschlossen ist, empfängt Ihr [PostToolUse-Hook](#posttooluse) das Ergebnis des Subagenten und die Telemetrie des Laufs in `tool_response`. Lesen Sie diese Felder, um den Lauf zu überprüfen; für Token- und Kosten-Rollups über Subagenten verwenden Sie die [Token- und Kosten-Zähler](/docs/de/monitoring-usage#token-counter), gefiltert nach `query_source` `"subagent"`, da `totalTokens` und `usage` nur die letzte Anfrage abdecken:

| Feld                | Typ    | Beispiel                                              | Beschreibung                                                                                                                                                                                                                     |
| :------------------ | :----- | :---------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` für Vordergrund-Subagenten, `"async_launched"` für Hintergrund-Subagenten. Ab v2.1.198 laufen Subagenten standardmäßig im Hintergrund, daher erzeugt ein weggelassenes `run_in_background` auch `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Identifikator für den Subagenten-Lauf                                                                                                                                                                                            |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Die finalen Textblöcke des Subagenten oder, für einen Subagenten, dessen Bericht durch `SubagentHandback` geht, eine kurze Notiz über diesen Handback an ihrer Stelle                                                            |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Modell, auf dem der Subagent gestartet wurde, das sich vom angeforderten Modell unterscheiden kann                                                                                                                               |
| `modelsUsed`        | array  | `["claude-sonnet-4-5", "claude-haiku-4-5"]`           | Verwendete Modelle in Reihenfolge, mit aufeinanderfolgenden Wiederholungen zusammengefasst; nur gesetzt, wenn das Modell während des Laufs gewechselt wurde. Erfordert Claude Code v2.1.212 oder später                          |
| `totalTokens`       | number | `12450`                                               | Token-Anzahl aus der letzten API-Anfrage des Subagenten: Eingabe-, Ausgabe- und Cache-Tokens kombiniert. Dies ist keine Gesamtsumme über den ganzen Lauf                                                                         |
| `totalDurationMs`   | number | `48211`                                               | Wanduhr-Dauer des Subagenten-Laufs                                                                                                                                                                                               |
| `totalToolUseCount` | number | `7`                                                   | Anzahl der Tool-Aufrufe, die der Subagent gemacht hat                                                                                                                                                                            |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Pro-Typ Token-Aufschlüsselung der letzten API-Anfrage: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                 |

Auf Claude Code v2.1.271 oder später liefert ein Subagent, der mit dem [`SubagentHandback`](/docs/de/tools-reference) Tool läuft, das Claude Code im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) bereitstellt, seinen Bericht durch dieses Tool statt ihn als Text zurückzugeben. Das `content` Feld seines `completed` Ergebnisses trägt dann eine kurze Notiz über diesen Handback statt des Berichts selbst. Um den Bericht zu lesen, gleichen Sie einen `PreToolUse` oder `PostToolUse` Hook auf `SubagentHandback` ab und lesen Sie `tool_input.message`.

Für Hintergrund-Subagenten gibt das Tool zurück, wenn die Aufgabe in den Hintergrund geht, daher trägt `tool_response` keine Nutzungsfelder: Ein Hintergrund-Start gibt sofort zurück, und eine Vordergrund-Aufgabe, die Claude Code während des Laufs in den Hintergrund verschiebt, gibt bei diesem Übergang zurück. Es hat `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile` und `resolvedModel`.

Bei einem `completed` Ergebnis benennt `resolvedModel` das Modell, auf dem der Subagent gestartet wurde, das sich vom `model` Wert in `tool_input` unterscheiden kann, wie wenn `availableModels` oder ein anderer Override gilt. Bei einem `async_launched` Ergebnis benennt `resolvedModel` das Modell in Gebrauch, wenn der Agent in den Hintergrund ging, daher wird ein Wechsel, der vor dem Hintergrund-Gehen stattfand, dort widergespiegelt. `modelsUsed` und das Hintergrund-Zeit-`resolvedModel` Verhalten erfordern Claude Code v2.1.212 oder später.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Stellt dem Benutzer eine bis vier Multiple-Choice-Fragen.

| Feld        | Typ    | Beispiel                                                                                                           | Beschreibung                                                                                                                                                                                                                        |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Zu stellende Fragen, jeweils mit einer `question` Zeichenkette, kurzem `header`, `options` Array und optionalem `multiSelect` Flag                                                                                                  |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Optional. Ordnet Fragentext der ausgewählten Optionsbeschriftung zu. Multi-Select-Antworten verbinden Beschriftungen mit Kommas. Claude setzt dieses Feld nicht; liefern Sie es über `updatedInput`, um programmatisch zu antworten |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Präsentiert einen Plan und fragt den Benutzer, ihn zu genehmigen, bevor Claude den [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) verlässt. Claude schreibt den Plan vor dem Aufrufen des Tools in eine Datei auf der Festplatte, daher ist die wörtliche `tool_input` vom Modell typischerweise leer. Claude Code injiziert den Plan-Inhalt und Dateipfad, bevor die Eingabe an Hooks übergeben wird.

| Feld             | Typ    | Beispiel                                    | Beschreibung                                                                                                                                                                         |
| :--------------- | :----- | :------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Plan-Inhalt in Markdown. Injiziert aus der Plan-Datei auf der Festplatte                                                                                                             |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Pfad zur Plan-Datei. Injiziert                                                                                                                                                       |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Veraltet. Claude Code akzeptiert das Feld, ignoriert es aber. Vor v2.1.205 trug es eingabeaufforderungsbasierte Berechtigungen, die Claude anforderte, um den Plan zu implementieren |

In `PostToolUse` ist `tool_response` ein Objekt mit `plan` und `filePath` Feldern, die den genehmigten Plan enthalten, plus interne Status-Flags. Lesen Sie `tool_response.plan` für den Plan-Inhalt, anstatt die Datei von der Festplatte erneut zu lesen.

<h4 id="pretooluse-decision-control">
  PreToolUse-Entscheidungskontrolle
</h4>

`PreToolUse` Hooks können steuern, ob ein Tool-Aufruf fortgesetzt wird. Im Gegensatz zu anderen Hooks, die ein Top-Level-`decision` Feld verwenden, gibt PreToolUse seine Entscheidung in einem `hookSpecificOutput` Objekt zurück. Dies gibt ihm reichere Kontrolle: vier Ergebnisse (erlauben, verweigern, fragen oder aufschieben) plus die Möglichkeit, Tool-Eingabe vor der Ausführung zu ändern.

| Feld                       | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `permissionDecision`       | `"allow"` überspringt die Berechtigungsaufforderung, außer für die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves) und für `AskUserQuestion` und `ExitPlanMode`, die [`updatedInput` gepaart damit benötigen](#allow-with-updatedinput). `"deny"` verhindert den Tool-Aufruf. `"ask"` fordert den Benutzer zur Bestätigung auf. `"defer"` beendet sich elegant, damit das Tool später fortgesetzt werden kann. [Ablehnungs- und Frageregel](/docs/de/permissions#manage-permissions) werden immer noch ausgewertet, unabhängig davon, was der Hook zurückgibt |
| `permissionDecisionReason` | Für `"allow"` und `"ask"`, dem Benutzer angezeigt, aber nicht Claude. Für `"deny"`, Claude angezeigt. Für `"defer"`, ignoriert                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `updatedInput`             | Ändert die Tool-Eingabeparameter vor der Ausführung. Ersetzt das gesamte Eingabeobjekt, daher schließen Sie unveränderte Felder neben geänderten ein. Claude Code wertet Berechtigungsregeln und die [Auto-Background-Berechtigung](/docs/de/tools-reference#background-commands) eines Bash-Befehls gegen die Eingabe aus, die Ihr Hook zurückgibt, nicht die Eingabe, die Claude gesendet hat. Kombinieren Sie mit `"allow"`, um automatisch zu genehmigen, oder mit `"ask"`, um die geänderte Eingabe dem Benutzer zu zeigen. Für `"defer"`, ignoriert                                                          |
| `additionalContext`        | String, der zu Claudes Kontext neben dem Tool-Ergebnis hinzugefügt wird. Ignoriert, wenn `permissionDecision` `"defer"` ist. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                                                                   |

Wenn mehrere PreToolUse-Hooks unterschiedliche Entscheidungen zurückgeben, ist die Priorität `deny` > `defer` > `ask` > `allow`.

Ein Hook, der durch Beendigung mit 2 blockiert, wird auf die gleiche Weise wie `"deny"` weitergeleitet: Claude sieht die stderr-Nachricht als Ablehnungsgrund.

Wenn ein Hook `"ask"` zurückgibt, enthält die dem Benutzer angezeigte Berechtigungsaufforderung ein Label, das angibt, woher der Hook stammt: `[settings]` für einen Hook aus einer beliebigen Einstellungsdatei oder aus Agent-Frontmatter, `[plugin:<name>]` für einen Hook eines Plugins oder `[skill]` für einen Hook aus Skill-Frontmatter. Dies hilft Benutzern zu verstehen, welche Konfigurationsquelle eine Bestätigung anfordert.

Ein Hook-`"ask"` erzwingt auch eine Berechtigungsaufforderung im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode): Der Klassifizierer kann den Tool-Aufruf immer noch verweigern, aber er kann den Aufruf nicht stillschweigend genehmigen. Vor v2.1.211 konnte der Klassifizierer einen Bash-Befehl, der außerhalb der [Sandbox](/docs/de/sandboxing) läuft, ohne die Aufforderung zu zeigen, die der Hook anforderte, genehmigen; der Klassifizierer wendete immer noch seine eigenen Sicherheitsregeln auf diesen Befehl an, und ein Hook `"deny"` wurde immer berücksichtigt.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "My reason here",
    "updatedInput": {
      "field_to_modify": "new value"
    },
    "additionalContext": "Current environment: production. Proceed with caution."
  }
}
```

<span id="allow-with-updatedinput" />

Im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p` Flag bietet Claude Code `AskUserQuestion` und `ExitPlanMode` nur an, wenn der Lauf einen [Berechtigungshost](/docs/de/headless#turn-off-permission-prompts-in-unattended-runs) hat, um die Aufforderung zu empfangen, wie einen Agent SDK `canUseTool` Callback. Diese Tools erfordern Benutzerinteraktion. Das Zurückgeben von `permissionDecision: "allow"` zusammen mit `updatedInput` erfüllt diese Anforderung: Der Hook liest die Tool-Eingabe von stdin, sammelt die Antwort über Ihre eigene Benutzeroberfläche und gibt sie in `updatedInput` zurück, damit das Tool ohne Aufforderung ausgeführt wird. Das Zurückgeben von `"allow"` allein ist nicht ausreichend für diese Tools. Für `AskUserQuestion` geben Sie das ursprüngliche `questions` Array zurück und fügen ein [`answers`](#askuserquestion) Objekt hinzu, das jede Frage des Textes der gewählten Antwort zuordnet.

Ab v2.1.199 ist ein MCP-Tool, dessen Server es mit [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) markiert, strenger: Ein Hook kann seine Genehmigungsaufforderung nicht mit `"allow"` überspringen, mit oder ohne `updatedInput`, da Claude Code nicht bestätigen kann, dass der Hook die Interaktion sammelte, die das Tool benötigt.

<Note>
  PreToolUse verwendete zuvor Top-Level-`decision` und `reason` Felder, aber diese sind für dieses Ereignis veraltet. Verwenden Sie stattdessen `hookSpecificOutput.permissionDecision` und `hookSpecificOutput.permissionDecisionReason`. Die veralteten Werte `"approve"` und `"block"` ordnen sich `"allow"` und `"deny"` zu. Andere Ereignisse wie PostToolUse und Stop verwenden weiterhin Top-Level-`decision` und `reason` als ihr aktuelles Format.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Einen Tool-Aufruf für später aufschieben
</h4>

`"defer"` ist für Integrationen, die `claude -p` als Unterprozess ausführen und seine JSON-Ausgabe lesen, wie eine Agent SDK App oder eine benutzerdefinierte Benutzeroberfläche, die auf Claude Code aufgebaut ist. Es ermöglicht diesem aufrufenden Prozess, Claude bei einem Tool-Aufruf zu pausieren, Eingabe über seine eigene Schnittstelle zu sammeln und dort fortzufahren, wo er aufgehört hat. Claude Code berücksichtigt diesen Wert nur im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p` Flag. In interaktiven Sitzungen protokolliert es eine Warnung und ignoriert das Hook-Ergebnis.

Das `AskUserQuestion` Tool ist der typische Fall: Claude möchte den Benutzer etwas fragen, aber es gibt kein Terminal zum Antworten. Ein `-p` Lauf bietet `AskUserQuestion` nur an, wenn er einen [Berechtigungshost](/docs/de/headless#turn-off-permission-prompts-in-unattended-runs) hat, wie ein MCP-Tool, das Sie mit `--permission-prompt-tool` übergeben, daher starten Sie den Lauf mit einem. Der Roundtrip funktioniert so:

1. Claude ruft `AskUserQuestion` auf. Der `PreToolUse` Hook wird ausgeführt.
2. Der Hook gibt `permissionDecision: "defer"` zurück. Das Tool wird nicht ausgeführt. Der Prozess beendet sich mit `stop_reason: "tool_deferred"` und dem ausstehenden Tool-Aufruf, der im Transkript erhalten bleibt.
3. Der aufrufende Prozess liest `deferred_tool_use` aus dem SDK-Ergebnis, zeigt die Frage in seiner eigenen Benutzeroberfläche an und wartet auf eine Antwort.
4. Der aufrufende Prozess führt `claude -p --resume <session-id>` mit dem gleichen Berechtigungshost aus. Der gleiche Tool-Aufruf wird `PreToolUse` erneut ausgeführt.
5. Der Hook gibt `permissionDecision: "allow"` mit der Antwort in `updatedInput` zurück. Das Tool wird ausgeführt und Claude setzt fort.

Das `deferred_tool_use` Feld trägt die `id`, `name` und `input` des Tools. Die `input` sind die Parameter, die Claude für den Tool-Aufruf generiert hat, erfasst vor der Ausführung:

```json theme={null}
{
  "type": "result",
  "subtype": "success",
  "stop_reason": "tool_deferred",
  "session_id": "abc123",
  "deferred_tool_use": {
    "id": "toolu_01abc",
    "name": "AskUserQuestion",
    "input": { "questions": [{ "question": "Which framework?", "header": "Framework", "options": [{"label": "React"}, {"label": "Vue"}], "multiSelect": false }] }
  }
}
```

Es gibt kein Timeout oder Wiederholungslimit. Die Sitzung bleibt auf der Festplatte, bis Sie sie fortsetzen, unterliegt aber der [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays) Aufbewahrungssweep, die Sitzungsdateien nach 30 Tagen standardmäßig löscht, nach den [Aufbewahrungssweep-Regeln](/docs/de/claude-directory#cleaned-up-automatically). Wenn die Antwort nicht bereit ist, wenn Sie fortsetzen, kann der Hook `"defer"` erneut zurückgeben und der Prozess beendet sich auf die gleiche Weise. Der aufrufende Prozess steuert, wann die Schleife unterbrochen wird, indem er schließlich `"allow"` oder `"deny"` vom Hook zurückgibt.

`"defer"` funktioniert nur, wenn Claude einen einzelnen Tool-Aufruf im Zug macht. Wenn Claude mehrere Tool-Aufrufe gleichzeitig macht, wird `"defer"` mit einer Warnung ignoriert und das Tool wird durch den normalen Berechtigungsfluss fortgesetzt. Die Einschränkung existiert, weil Resume nur ein Tool erneut ausführen kann: Es gibt keine Möglichkeit, einen Aufruf aus einem Batch aufzuschieben, ohne die anderen ungelöst zu lassen.

Wenn das aufgeschobene Tool nicht mehr verfügbar ist, wenn Sie fortsetzen, beendet sich der Prozess mit `stop_reason: "tool_deferred_unavailable"` und `is_error: true` bevor der Hook ausgeführt wird. Dies geschieht, wenn ein MCP-Server, der das Tool bereitgestellt hat, für die fortgesetzte Sitzung nicht verbunden ist. Die `deferred_tool_use` Nutzlast ist immer noch enthalten, damit Sie identifizieren können, welches Tool fehlte.

<Note>
  Um eine aufgeschobene Sitzung im Plan-Modus fortzusetzen, übergeben Sie [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) zusammen mit `--resume`, damit Claude Code den Plan zur Genehmigung präsentieren kann. Ohne es stellt Claude Code den Plan-Modus nicht wieder her. Erfordert Claude Code v2.1.246 oder später.

  Wenn Sie mit `-p` fortsetzen, stellt Claude Code keinen anderen gespeicherten Berechtigungsmodus wieder her. Es startet den Lauf im Berechtigungsmodus, den ein neuer `claude -p` Lauf starten würde, daher übergeben Sie `--permission-mode` oder `--dangerously-skip-permissions` erneut, wenn die aufgeschobene Sitzung einen verwendet hat. Wenn Sie mit `claude --resume <session-id>` ohne `-p` fortsetzen, stellt Claude Code den gespeicherten Berechtigungsmodus wieder her, mit den Ausnahmen, die in [Berechtigungsmodus bei Fortsetzen](/docs/de/sessions#permission-mode-on-resume) aufgelistet sind.
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Wird ausgeführt, wenn Claude Code Sie um Erlaubnis bitten möchte, ein Tool zu verwenden. In Sitzungen, die keine Aufforderung anzeigen können, wie Hintergrund-Subagenten im [nicht-interaktiven Modus](/docs/de/headless), führt Claude Code diese Hooks immer noch aus, und wenn kein Hook eine Entscheidung zurückgibt, verweigert es den Tool-Aufruf.
Verwenden Sie [PermissionRequest-Entscheidungskontrolle](#permissionrequest-decision-control), um im Namen des Benutzers zu erlauben oder zu verweigern.

Verwenden Sie dieses Ereignis, wenn Sie ein Signal benötigen, in dem Moment, in dem Claude um Erlaubnis bittet, ein Tool zu verwenden. Claude Code führt einen [Notification](#notification) Hook mit dem `permission_prompt` Typ nur aus, nachdem die Aufforderung etwa sechs Sekunden gewartet hat.

Claude Code führt PermissionRequest-Hooks nicht für eine Sandbox-Anfrage eines Befehls aus [Netzwerkanfrage](/docs/de/sandboxing#network-isolation). Um ein Signal für diese Aufforderung zu erhalten, verwenden Sie den `permission_prompt` Benachrichtigungstyp.

Gleicht Tool-Namen ab, gleiche Werte wie PreToolUse.

<h4 id="permissionrequest-input">
  PermissionRequest-Eingabe
</h4>

PermissionRequest-Hooks erhalten `tool_name` und `tool_input` Felder wie PreToolUse-Hooks, aber ohne `tool_use_id`. Für ein MCP-Tool erhalten sie auch das [`mcp_server`](#pretooluse-input) Objekt. Ein optionales `permission_suggestions` Array enthält die [Berechtigungsaktualisierungen](#permission-update-entries), die Claude Code für diese Anfrage vorschlägt, wie das Hinzufügen einer Erlaubnisregel oder das Ändern des Berechtigungsmodus.

Das `permission_suggestions` Array ist keine genaue Liste der Optionen, die Sie sehen, da jeder Berechtigungsdialog seine eigenen Optionen erstellt. Einige Dialoge, wie der für Dateibearbeitungen, lesen das Array überhaupt nicht und leiten ihre Optionen aus der Anfrage selbst ab. Ein Dialog, der es liest, kann immer noch eine Option zurückhalten, deren Vorschlag im Array bleibt, z. B. wenn [`allowManagedPermissionRulesOnly`](/docs/de/settings-reference#allowmanagedpermissionrulesonly) Regel-Speicheroptionen verbirgt. Es kann auch Optionen anbieten, die keinen Vorschlagseintrag haben, wie [**Ja, und zum Auto-Modus wechseln**](/docs/de/permission-modes#switch-permission-modes), das den Berechtigungsmodus direkt ändert, anstatt durch eine Berechtigungsaktualisierung.

PreToolUse-Hooks werden vor jedem Tool-Aufruf ausgeführt, unabhängig davon, ob er Berechtigung benötigt. PermissionRequest-Hooks werden nur ausgeführt, wenn Claude Code Sie um Erlaubnis bitten möchte, oder wenn es sonst einen Aufruf automatisch verweigern würde, der nicht auffordern kann. Keines der Ereignisse wird für [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior) ausgeführt.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PermissionRequest",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf node_modules",
    "description": "Remove node_modules directory"
  },
  "permission_suggestions": [
    {
      "type": "addRules",
      "rules": [{ "toolName": "Bash", "ruleContent": "rm -rf node_modules" }],
      "behavior": "allow",
      "destination": "localSettings"
    }
  ]
}
```

<h4 id="permissionrequest-decision-control">
  PermissionRequest-Entscheidungskontrolle
</h4>

`PermissionRequest` Hooks können Berechtigungsanfragen erlauben oder verweigern. Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, kann Ihr Hook-Skript ein `decision` Objekt mit diesen ereignisspezifischen Feldern zurückgeben:

| Feld                 | Beschreibung                                                                                                                                                                                                                                               |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` gewährt die Berechtigung, `"deny"` verweigert sie. [Ablehnungs- und Frageregel](/docs/de/permissions#manage-permissions) werden immer noch ausgewertet, daher überschreibt ein Hook, der `"allow"` zurückgibt, keine übereinstimmende Ablehnungsregel |
| `updatedInput`       | Nur für `"allow"`: ändert die Tool-Eingabeparameter vor der Ausführung. Ersetzt das gesamte Eingabeobjekt, daher schließen Sie unveränderte Felder neben geänderten ein. Die geänderte Eingabe wird erneut gegen Ablehnungs- und Frageregel ausgewertet    |
| `updatedPermissions` | Nur für `"allow"`: Array von [Berechtigungsaktualisierungseinträgen](#permission-update-entries) zum Anwenden, wie das Hinzufügen einer Erlaubnisregel oder das Ändern des Sitzungsberechtigungsmodus                                                      |
| `message`            | Nur für `"deny"`: sagt Claude, warum die Berechtigung verweigert wurde                                                                                                                                                                                     |
| `interrupt`          | Nur für `"deny"`: wenn `true`, stoppt Claude                                                                                                                                                                                                               |

Ein Hook, der mit 2 beendet wird, ohne ein `decision` Objekt zu hinterlassen, lässt den Berechtigungsfluss unverändert, und sein stderr wird verworfen. Nur das `decision` Objekt kann die Anfrage gewähren oder verweigern.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```

<h4 id="permission-update-entries">
  Berechtigungsaktualisierungseinträge
</h4>

Das `updatedPermissions` Ausgabefeld und das [`permission_suggestions` Eingabefeld](#permissionrequest-input) verwenden beide das gleiche Array von Einträgen. Jeder Eintrag hat einen `type`, der seine anderen Felder bestimmt, und ein `destination`, das steuert, wo die Änderung geschrieben wird.

| `type`              | Felder                             | Effekt                                                                                                                                                                                                                        |
| :------------------ | :--------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `addRules`          | `rules`, `behavior`, `destination` | Fügt Berechtigungsregeln hinzu. `rules` ist ein Array von `{toolName, ruleContent?}` Objekten. Weglassen `ruleContent`, um das ganze Tool abzugleichen. `behavior` ist `"allow"`, `"deny"` oder `"ask"`                       |
| `replaceRules`      | `rules`, `behavior`, `destination` | Ersetzt alle Regeln des gegebenen `behavior` am `destination` mit den bereitgestellten `rules`                                                                                                                                |
| `removeRules`       | `rules`, `behavior`, `destination` | Entfernt übereinstimmende Regeln des gegebenen `behavior`                                                                                                                                                                     |
| `setMode`           | `mode`, `destination`              | Ändert den Berechtigungsmodus. Gültige Modi sind `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan` und `manual` als Alias für `default`. Der `manual` Alias erfordert Claude Code v2.1.200 oder später |
| `addDirectories`    | `directories`, `destination`       | Fügt Arbeitsverzeichnisse hinzu. `directories` ist ein Array von Pfad-Zeichenketten                                                                                                                                           |
| `removeDirectories` | `directories`, `destination`       | Entfernt Arbeitsverzeichnisse                                                                                                                                                                                                 |

<Note>
  `setMode` mit `bypassPermissions` wird nur wirksam, wenn Sie die Sitzung mit Bypass-Modus bereits verfügbar gestartet haben: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` oder `permissions.defaultMode: "bypassPermissions"` in [Benutzer-, `--settings`- oder verwalteten Einstellungen](/docs/de/settings-reference#permissions-defaultmode). Andernfalls ist die Aktualisierung ein No-Op. Die Aktualisierung ist auch ein No-Op, wenn [`permissions.disableBypassPermissionsMode`](/docs/de/permissions#managed-settings) den Modus deaktiviert oder die Sitzung im [eingeschränkten Modus](/docs/de/cli-reference#cli-flags) startet.

  `bypassPermissions` wird niemals als `defaultMode` beibehalten, unabhängig von `destination`.
</Note>

Das `destination` Feld auf jedem Eintrag bestimmt, ob die Änderung im Speicher bleibt oder in einer Einstellungsdatei beibehalten wird.

| `destination`     | Schreibt zu                                        |
| :---------------- | :------------------------------------------------- |
| `session`         | Nur im Speicher, verworfen, wenn die Sitzung endet |
| `localSettings`   | `.claude/settings.local.json`                      |
| `projectSettings` | `.claude/settings.json`                            |
| `userSettings`    | `~/.claude/settings.json`                          |

Ein Hook kann eines der `permission_suggestions` widerspiegeln, die er als seine eigene `updatedPermissions` Ausgabe erhalten hat.

<h3 id="posttooluse">
  PostToolUse
</h3>

Wird sofort nach erfolgreichem Abschluss eines Tools ausgeführt.

Gleicht Tool-Namen ab, gleiche Werte wie PreToolUse.

Gleichen Sie breiter ab, wenn der Tool-Name nicht der richtige Filter ist:

* Um einen Hook nach jedem erfolgreichen Tool-Abschluss auszuführen, weglassen Sie den `matcher` oder setzen Sie ihn auf `"*"`. Ihr Hook kann dann selbst entdecken, was sich geändert hat, z. B. durch Ausführung von `git status --porcelain`, das auch nicht verfolgte Dateien auflistet, die `git diff` verpasst. Für Tool-Aufrufe, die fehlschlagen, fügen Sie den gleichen Hook unter [PostToolUseFailure](#posttoolusefailure) hinzu.
* Um einen Hook auszuführen, wenn sich eine bestimmte Datei auf der Festplatte ändert, unabhängig davon, wer sie schreibt, verwenden Sie [FileChanged](#filechanged). Claude Code führt keinen `PostToolUse` Hook aus, der `Edit|Write` abgleicht, wenn ein `Bash` Befehl oder ein Prozess außerhalb von Claude Code die gleiche Datei umschreibt.

<h4 id="posttooluse-input">
  PostToolUse-Eingabe
</h4>

`PostToolUse` Hooks werden ausgeführt, nachdem ein Tool bereits erfolgreich ausgeführt wurde. Die Eingabe umfasst sowohl `tool_input`, die an das Tool gesendeten Argumente, als auch `tool_response`, das Ergebnis, das es zurückgegeben hat. Das genaue Schema für beide hängt vom Tool ab. Datei-Tool-`tool_input` Pfade kommen im gleichen Format wie für [PreToolUse](#pretooluse-input) an: immer absolut, mit den nativen Trennzeichen der Plattform, daher Backslashes unter Windows. Für ein MCP-Tool trägt die Eingabe auch das [`mcp_server`](#pretooluse-input) Objekt.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "type": "create"
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Feld          | Beschreibung                                                                                                                               |
| :------------ | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `duration_ms` | Optional. Tool-Ausführungszeit in Millisekunden. Schließt Zeit aus, die in Berechtigungsaufforderungen und PreToolUse-Hooks verbracht wird |

<h4 id="posttooluse-decision-control">
  PostToolUse-Entscheidungskontrolle
</h4>

`PostToolUse` Hooks können Claude nach der Tool-Ausführung Feedback geben. Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, kann Ihr Hook-Skript diese ereignisspezifischen Felder zurückgeben:

| Feld                   | Beschreibung                                                                                                                                                                                                                                                                                                         |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`             | `"block"` fügt den `reason` neben dem Tool-Ergebnis hinzu. Claude sieht immer noch die ursprüngliche Ausgabe; um sie zu ersetzen, verwenden Sie `updatedToolOutput`                                                                                                                                                  |
| `reason`               | Erklärung, die Claude angezeigt wird, wenn `decision` `"block"` ist                                                                                                                                                                                                                                                  |
| `additionalContext`    | String, der zu Claudes Kontext neben dem Tool-Ergebnis hinzugefügt wird. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude)                                                                                                                                                                              |
| `classifierContext`    | Kurze Notiz über das Ergebnis dieses Aufrufs für den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) Klassifizierer statt für Claude. Siehe [Ergebnis für den Auto-Modus Klassifizierer annotieren](#annotate-a-result-for-the-auto-mode-classifier). Erfordert Claude Code v2.1.236 oder später |
| `updatedToolOutput`    | Ersetzt die Ausgabe des Tools mit dem bereitgestellten Wert, bevor er an Claude gesendet wird. Der Wert muss der Ausgabeform des Tools entsprechen                                                                                                                                                                   |
| `updatedMCPToolOutput` | Ersetzt die Ausgabe nur für [MCP-Tools](#match-mcp-tools). Bevorzugen Sie `updatedToolOutput`, das für alle Tools funktioniert                                                                                                                                                                                       |

Das Beispiel unten ersetzt die Ausgabe eines `Bash` Aufrufs. Der Ersatzwert entspricht der Ausgabeform des `Bash` Tools:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude",
    "updatedToolOutput": {
      "stdout": "[redacted]",
      "stderr": "",
      "interrupted": false,
      "isImage": false
    }
  }
}
```

<Warning>
  `updatedToolOutput` ändert nur das, was Claude sieht. Das Tool hat bereits ausgeführt, wenn der Hook ausgeführt wird, daher haben alle geschriebenen Dateien, ausgeführten Befehle oder gesendeten Netzwerkanfragen bereits Auswirkungen. Telemetrie wie OpenTelemetry-Tool-Spans und Analyseereignisse erfassen auch die ursprüngliche Ausgabe, bevor der Hook ausgeführt wird. Um einen Tool-Aufruf zu verhindern oder zu ändern, bevor er ausgeführt wird, verwenden Sie stattdessen einen [PreToolUse](#pretooluse) Hook.

  Der Ersatzwert muss der Ausgabeform des Tools entsprechen. Integrierte Tools geben strukturierte Objekte statt einfacher Zeichenketten zurück. Beispielsweise gibt `Bash` ein Objekt mit `stdout`, `stderr`, `interrupted` und `isImage` Feldern zurück. Für integrierte Tools wird ein Wert, der nicht dem Ausgabeschema des Tools entspricht, ignoriert und die ursprüngliche Ausgabe wird verwendet. MCP-Tool-Ausgabe wird ohne Schema-Validierung durchgelassen. Das Entfernen von Fehlerdetails, die Claude benötigt, kann dazu führen, dass er bei einer falschen Annahme fortfährt.
</Warning>

<h4 id="annotate-a-result-for-the-auto-mode-classifier">
  Ergebnis für den Auto-Modus Klassifizierer annotieren
</h4>

Geben Sie `classifierContext` zurück, um eine kurze Notiz über das Ergebnis des Tool-Aufrufs an den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) Klassifizierer statt an Claude zu senden. Der Klassifizierer [empfängt niemals Tool-Ergebnisse selbst](/docs/de/permission-modes#how-the-classifier-evaluates-actions), daher ist dieses Feld die unterstützte Methode, um ihm etwas über das, was ein Aufruf zurückgegeben hat, zu sagen, bevor er spätere Aktionen überprüft. Das Feld erfordert Claude Code v2.1.236 oder später.

Das Beispiel unten sagt dem Klassifizierer, woher die Ausgabe einer Abfrage stammt:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "classifierContext": "This query ran against the staging database, not production."
  }
}
```

Wie viel Gewicht der Klassifizierer der Notiz gibt, hängt davon ab, wo Sie den Hook konfiguriert haben:

* **Hooks, die in Claude Code konfiguriert sind**: Für Hooks aus Einstellungsdateien, Plugins, Skills und Agent-Frontmatter behandelt der Klassifizierer die Notiz als nicht verifizierten, von der Anwendung bereitgestellten Kontext. Die Notiz stellt niemals Benutzerabsicht fest, und wenn sie behauptet, Sie hätten etwas genehmigt oder angefordert, überprüft der Klassifizierer diese Behauptung gegen Ihre eigenen Nachrichten im Gespräch
* **In-Process Agent SDK Callbacks**: Wenn eine Anwendung, die Claude Code einbettet, den Hook als [TypeScript SDK Callback](/docs/de/agent-sdk/hooks) registriert und die Notiz während der Live-Sitzung zurückgibt, kann der Klassifizierer eine Benutzeraussage, die in der Notiz weitergeleitet wird, als Benutzerabsicht gewichten. Eine solche Aussage kann eine Zustimmungsanforderung erfüllen, die der Klassifizierer von einer Nachricht akzeptieren würde, die Sie senden, aber sie hebt niemals einen Block auf, den Ihre eigene Nachricht auch nicht heben könnte. Nach einer Sitzungsfortsetzung behandelt Claude Code wiederhergestellte Notizen als nicht verifizierten Kontext. Wenn Hooks aus beiden Gruppen den gleichen Aufruf annotieren, behandelt der Klassifizierer die kombinierte Notiz als nicht verifizierten Kontext

Claude Code wendet diese Grenzen an, wenn die Notiz bereitgestellt wird:

* **Länge**: Claude Code begrenzt die Notizen für einen Tool-Aufruf auf 2.000 Zeichen und schneidet den Rest ab. Die Grenze wird über jeden Hook geteilt, der auf diesen Aufruf antwortet
* **Nur synchrone Antworten**: Claude Code ignoriert das Feld in der Antwort eines Hooks, der [im Hintergrund ausgeführt wird](#run-hooks-in-the-background), da diese Antwort nach der Aufzeichnung des Tool-Ergebnisses ankommt
* **Aufrufe, die der Klassifizierer nicht aufzeichnet**: Das Transkript des Klassifizierers lässt schreibgeschützte Lookups wie Dateilesevorgänge und Suchen aus. Claude Code verwirft eine Notiz, die an einen dieser Aufrufe angehängt ist
* **Interaktion mit Umschreibungen**: Wenn die Notiz Ausgabe beschreibt, die Sie mit `updatedToolOutput` ersetzen, geben Sie beide Felder in der gleichen Hook-Antwort zurück. Claude Code löscht die Notiz, wenn diese Umschreibung abgelehnt wird oder eine andere Hook-Umschreibung sie ersetzt. Claude Code liefert eine Notiz, die Sie ohne Umschreibung zurückgeben, auch wenn ein anderer Hook die Ausgabe umschreibt

<Warning>
  Der Klassifizierer liest Inhalte, die Sie in `classifierContext` einfügen, als Informationen vom Anwendungshost der Sitzung, daher kopieren Sie nicht vertrauenswürdige Tool-Ausgabe oder Text von Drittanbietern hinein. Halten Sie die Notiz auf eine kurze Aussage über diesen einen Aufruf, wie eine Tatsache über seinen Ursprung oder eine Benutzeraussage darüber; verwenden Sie das Feld nicht, um nicht verwandte Nachrichten oder einen Strom von Ereignissen zu liefern.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Wird ausgeführt, wenn ein Tool, das mit der Ausführung begonnen hat, fehlschlägt: Das Tool warf einen Fehler oder ein MCP-Tool gab ein Fehlerergebnis zurück. Verwenden Sie dies, um Fehler zu protokollieren, Warnungen zu senden oder korrektes Feedback an Claude zu geben.

Gleicht Tool-Namen ab, gleiche Werte wie PreToolUse.

<Note>
  Dieses Ereignis wird nicht für Tool-Aufrufe ausgeführt, die vor der Ausführung abgelehnt werden: Ein unbekannter Tool-Name, Eingabe, die Schema- oder Tool-spezifische Validierung fehlschlägt, oder eine Berechtigungsverweigerung. Validierungsablehnungen werden als `tool_use_error` Ergebnisse zurückgegeben und treten auf, bevor Hooks ausgeführt werden, daher werden weder `PreToolUse` noch dieses Ereignis ausgeführt. Berechtigungsverweigerungen führen `PreToolUse` aus, aber nicht dieses Ereignis; siehe [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  PostToolUseFailure-Eingabe
</h4>

PostToolUseFailure-Hooks erhalten die gleichen `tool_name` und `tool_input` Felder wie PostToolUse, zusammen mit Fehlerinformationen als Top-Level-Felder. Für ein MCP-Tool erhalten sie auch das [`mcp_server`](#pretooluse-input) Objekt. Beispielsweise könnte ein fehlgeschlagener `npm test` Befehl liefern:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUseFailure",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite"
  },
  "tool_use_id": "toolu_01ABC123...",
  "error": "Exit code 1\nError: Cannot find module 'express'",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Feld           | Beschreibung                                                                                                                                                                                                                                  |
| :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | Zeichenkette, die beschreibt, was schief gelaufen ist. Das Format hängt vom Tool ab, das fehlgeschlagen ist                                                                                                                                   |
| `is_interrupt` | Optionaler Boolean. True, wenn der Fehler Claude Code als Abbruch erreichte, anstatt als Fehler, den das Tool meldete. Das Abbrechen eines laufenden Tools führt nicht zu diesem Hook; das Tool-Ergebnis trägt stattdessen die Abbruchmeldung |
| `duration_ms`  | Optional. Tool-Ausführungszeit in Millisekunden. Schließt Zeit aus, die in Berechtigungsaufforderungen und PreToolUse-Hooks verbracht wird                                                                                                    |

Die `error` Zeichenkette ist im Allgemeinen der gleiche Text, den Claude als Ergebnis des fehlgeschlagenen Tools empfängt. Sein Format variiert je nach Tool und Fehler. Schlüsseln Sie Ihren Hook auf `tool_name`, `is_interrupt` und die erste Zeile `Exit code N`; behandeln Sie den Rest der Zeichenkette als Anzeigetext, nicht als stabiles Format.

* Für Bash und PowerShell erzeugt ein Befehl, der lief und beendet wurde, eine erste Zeile `Exit code N`, dann jede Ausgabe, die der Befehl als einen Block mit stdout und stderr vermischt erzeugte
* Eine Nutzlast kann auch eine bloße Fehlermeldung ohne Exit-Code-Zeile tragen, wenn Claude Code den Shell-Prozess selbst nicht starten konnte
* Claude Code schneidet lange Zeichenketten in der Mitte um einen `... [N characters truncated] ...` Marker ab und kann Zeilen von sich selbst einfügen, wie `Command timed out after 2m 0s`

<h4 id="posttoolusefailure-decision-control">
  PostToolUseFailure-Entscheidungskontrolle
</h4>

`PostToolUseFailure` Hooks können Claude nach einem Tool-Fehler Kontext geben. Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, kann Ihr Hook-Skript diese ereignisspezifischen Felder zurückgeben:

| Feld                | Beschreibung                                                                                                                     |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String, der zu Claudes Kontext neben dem Fehler hinzugefügt wird. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Additional information about the failure for Claude"
  }
}
```

<h3 id="posttoolbatch">
  PostToolBatch
</h3>

Wird einmal ausgeführt, nachdem jeder Tool-Aufruf in einem Batch aufgelöst wurde, bevor Claude Code die nächste Anfrage an das Modell sendet. `PostToolUse` wird einmal pro Tool ausgeführt, was bedeutet, dass es gleichzeitig ausgeführt wird, wenn Claude parallele Tool-Aufrufe macht. `PostToolBatch` wird genau einmal mit dem vollständigen Batch ausgeführt, daher ist es der richtige Ort, um Kontext einzufügen, der vom Satz von Tools abhängt, die liefen, statt von einem einzelnen Tool. Es gibt keinen Matcher für dieses Ereignis.

<h4 id="posttoolbatch-input">
  PostToolBatch-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten PostToolBatch-Hooks `tool_calls`, ein Array, das jeden Tool-Aufruf im Batch beschreibt:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolBatch",
  "tool_calls": [
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/accounts.py"},
      "tool_use_id": "toolu_01...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    },
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/transactions.py"},
      "tool_use_id": "toolu_02...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    }
  ]
}
```

`tool_response` enthält den gleichen Inhalt, den das Modell im entsprechenden `tool_result` Block empfängt. Der Wert ist eine serialisierte Zeichenkette oder ein Content-Block-Array, genau wie das Tool es ausgegeben hat. Für `Read` bedeutet das Zeilennummern-Präfix-Text statt rohe Dateiinhalte. Antworten können groß sein, daher analysieren Sie nur die Felder, die Sie benötigen.

<Note>
  Die `tool_response` Form unterscheidet sich von der von `PostToolUse`. `PostToolUse` übergibt das strukturierte `Output` Objekt des Tools, wie `{filePath: "...", type: "create"}` für `Write`; `PostToolBatch` übergibt den serialisierten `tool_result` Inhalt, den das Modell sieht.
</Note>

<h4 id="posttoolbatch-decision-control">
  PostToolBatch-Entscheidungskontrolle
</h4>

`PostToolBatch` Hooks können Kontext für Claude einfügen. Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, kann Ihr Hook-Skript diese ereignisspezifischen Felder zurückgeben:

| Feld                | Beschreibung                                                                                                                                                                                                                                                            |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Kontext-Zeichenkette, die einmal vor dem nächsten Modell-Aufruf eingefügt wird. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude) für Bereitstellungsdetails, was Sie darin einfügen sollten und wie fortgesetzte Sitzungen mit vergangenen Werten umgehen |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Das Zurückgeben von `decision: "block"` oder `continue: false` stoppt die agentengesteuerte Schleife vor dem nächsten Modell-Aufruf. Die Blockierungsmeldung kommt aus dem JSON `reason` oder `stopReason` oder aus stderr bei Beendigung mit 2. Sie sehen sie als Warnung im Transkript, und sie bleibt im Gespräch, daher sieht Claude sie, wenn das Gespräch fortgesetzt wird.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Wird ausgeführt, wenn [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) einen Tool-Aufruf verweigert, einschließlich wenn er verweigert, ohne ein Klassifizierer-Urteil zu haben, weil [eine Sicherheitsprüfung, die vom Auto-Modus getrennt ist, die Anfrage des Klassifizierers selbst verweigerte](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action) oder seine Antwort nicht analysiert wurde. Dieser Hook wird nur im Auto-Modus ausgeführt: Er wird nicht ausgeführt, wenn Sie einen Berechtigungsdialog manuell verweigern, wenn ein `PreToolUse` Hook einen Aufruf blockiert oder wenn eine `deny` Regel übereinstimmt. Verwenden Sie ihn, um Verweigerungen zu protokollieren, die Konfiguration anzupassen oder dem Modell zu sagen, dass es den Tool-Aufruf möglicherweise erneut versuchen kann.

Gleicht Tool-Namen ab, gleiche Werte wie PreToolUse.

<h4 id="permissiondenied-input">
  PermissionDenied-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten PermissionDenied-Hooks `tool_name`, `tool_input`, `tool_use_id` und `reason`. Für ein MCP-Tool erhalten sie auch das [`mcp_server`](#pretooluse-input) Objekt.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "auto",
  "hook_event_name": "PermissionDenied",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf /tmp/build",
    "description": "Clean build directory"
  },
  "tool_use_id": "toolu_01ABC123...",
  "reason": "[Irreversible Local Destruction]"
}
```

| Feld     | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `reason` | Der Verweigerungsgrund. Für ein Klassifizierer-Urteil benennt es in den meisten Sitzungen die übereinstimmende Regel in eckigen Klammern, wie `[Data Exfiltration]`; siehe [Verweigerungen überprüfen](/docs/de/auto-mode-config#review-denials) für die anderen Formen. Für eine [Verweigerung ohne Urteil](#permissiondenied-decision-control) beginnt sie mit `Auto mode could not evaluate this action and is blocking it for safety`. Für eine Verweigerung, weil das Klassifizierer-Modell nicht verfügbar war, ist es der feste Text `Classifier unavailable` |

<h4 id="permissiondenied-decision-control">
  PermissionDenied-Entscheidungskontrolle
</h4>

PermissionDenied-Hooks können dem Modell sagen, dass es den verweigerten Tool-Aufruf möglicherweise erneut versuchen kann. Geben Sie ein JSON-Objekt mit `hookSpecificOutput.retry` auf `true` zurück:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Wenn `retry` `true` ist, fügt Claude Code eine Nachricht zum Gespräch hinzu, die dem Modell sagt, dass es den Tool-Aufruf möglicherweise erneut versuchen kann. Claude Code kehrt die Verweigerung selbst nicht um. Wenn Ihr Hook kein JSON zurückgibt oder `retry: false` zurückgibt, bleibt die Verweigerung bestehen und das Modell empfängt die ursprüngliche Ablehnungsmeldung.

Claude Code ignoriert `retry: true`, wenn der Klassifizierer [kein Urteil über die Aktion erzeugt hat](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action): Seine Antwort wurde nicht analysiert oder eine Sicherheitsprüfung, die vom Auto-Modus getrennt ist, verweigerte die Anfrage des Klassifizierers. Für diese Verweigerungen sagt Claude Code dem Modell in der Ablehnungsmeldung bereits, ob es später erneut versuchen oder weitermachen soll.

<h3 id="notification">
  Notification
</h3>

Wird ausgeführt, wenn Claude Code Benachrichtigungen sendet. Gleicht Benachrichtigungstyp ab. Weglassen Sie den Matcher, um Hooks für alle Benachrichtigungstypen auszuführen.

Sie erhalten diese Hook-Ereignisse auch mit ausgeschalteten Desktop-Benachrichtigungen: Die `preferredNotifChannel` Einstellung, einschließlich `notifications_disabled`, ändert nur, wie Sie benachrichtigt werden, nicht ob Ihr Hook ausgeführt wird.

| Matcher                      | Wann wird es ausgelöst                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `permission_prompt`          | Claude benötigt Ihre Genehmigung für einen Tool-Aufruf oder eine Netzwerkanfrage eines Sandbox-Befehls, und die Aufforderung hat etwa sechs Sekunden gewartet                                                                                                                                                                                                                                                                                                                                                                                                 |
| `idle_prompt`                | Claude hat vor etwa 60 Sekunden geantwortet und Sie haben seitdem nicht eingegeben                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `auth_success`               | Authentifizierung ist abgeschlossen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `elicitation_dialog`         | Ein MCP-Server öffnet ein Elicitierungsformular und Sie haben etwa sechs Sekunden nicht eingegeben                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `elicitation_url_dialog`     | Ein MCP-Server fordert Sie auf, eine Browser-URL zu öffnen und Sie haben etwa sechs Sekunden nicht eingegeben                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `elicitation_complete`       | Ein MCP-Server meldet, dass eine [URL-Modus-Elicitierung](#elicitation-input) abgeschlossen ist                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `elicitation_response`       | Eine MCP-Elicitierungs-Antwort wird an den Server zurückgesendet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `agent_needs_input`          | Eine Hintergrund-Sitzung beginnt, auf Ihre Eingabe zu warten, während [Agent-Ansicht](/docs/de/agent-view) in einem Terminal offen ist, oder die aktuelle Sitzung stellt Ihnen eine [Agent-Team-Teamkollegen-Terminal-Setup-Frage](/docs/de/agent-teams#choose-a-display-mode) und Sie haben etwa sechs Sekunden nicht eingegeben                                                                                                                                                                                                                                       |
| `agent_completed`            | Eine Hintergrund-Sitzung wird beendet oder schlägt fehl. Wird nur ausgeführt, während [Agent-Ansicht](/docs/de/agent-view) in einem Terminal offen ist                                                                                                                                                                                                                                                                                                                                                                                                             |
| `quota_auto_resume_fired`    | Claude Code setzt Ihre Aufgabe nach einer claude.ai Nutzungslimit-Pause fort: beim Reset oder früher, wenn Sie etwas in Claude Code tun, während Sie warten, wie das Hinzufügen von Nutzungsguthaben, das Upgrade Ihres Plans oder das Wechseln von Modellen, macht Nutzung wieder verfügbar, mit der [Modell-Einstellung Ausnahme](/docs/de/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                                                                                     |
| `quota_auto_resume_stale`    | Ein claude.ai Nutzungslimit wurde zurückgesetzt, während Ihr Computer für mehr als etwa 30 Minuten schlief. Claude Code wartet darauf, dass Sie `Enter` drücken, anstatt fortzufahren. Nach einem kürzeren Schlaf wird es fortgesetzt und wird stattdessen `quota_auto_resume_fired` ausgeführt                                                                                                                                                                                                                                                               |
| `quota_auto_resume_disabled` | Claude Code beendet sein Warten auf ein claude.ai Nutzungslimit, ohne Ihre Aufgabe fortzusetzen: [`autoContinueAtUsageLimit`](/docs/de/settings-reference#autocontinueatusagelimit) wurde ausgeschaltet oder der Reset rückte während eines Wartens, das Claude Code selbst gestartet hat, mehr als 24 Stunden in die Zukunft, die fortgesetzte Aufgabe traf immer wieder das Limit oder die Fortsetzung wurde blockiert, bevor sie das Modell erreichte. Wird nicht ausgeführt, wenn Sie `Esc` oder `Ctrl+C` drücken oder **Nicht automatisch fortsetzen** wählen |

Die `agent_needs_input` und `agent_completed` Typen erfordern Claude Code v2.1.198 oder später.

Die `quota_auto_resume_fired`, `quota_auto_resume_stale` und `quota_auto_resume_disabled` Typen erfordern Claude Code v2.1.234 oder später.

In Terminal-Sitzungen erfordert `permission_prompt` für eine Netzwerkanfrage eines Sandbox-Befehls Claude Code v2.1.246 oder später.

`agent_needs_input` für eine Teamkollegen-Terminal-Setup-Frage erfordert Claude Code v2.1.248 oder später.

<Note>
  Die `permission_prompt`, `idle_prompt`, `elicitation_dialog` und `elicitation_url_dialog` Typen teilen ihr Timing mit Desktop-Benachrichtigungen, daher sehen Sie sie in Terminal-Sitzungen nur, wenn Sie vom Terminal entfernt zu sein scheinen:

  * Erwarten Sie `permission_prompt`, sobald Sie etwa sechs Sekunden nicht eingegeben haben. Der Timer startet, wenn die Berechtigungsaufforderung erscheint, und jeder Tastendruck verschiebt ihn. Um einen Hook sofort auszuführen, wenn Claude um Erlaubnis bittet, ein Tool zu verwenden, verwenden Sie stattdessen [PermissionRequest](#permissionrequest).
  * Erwarten Sie `idle_prompt` etwa 60 Sekunden, nachdem Claude geantwortet hat, und nur wenn Sie seitdem nicht eingegeben haben. Claude Code sendet `idle_prompt` nicht, während es auf einen claude.ai Nutzungslimit-Reset wartet. Wenn das Warten von selbst endet, wird stattdessen einer der `quota_auto_resume_*` Typen ausgeführt.
  * Erwarten Sie `elicitation_dialog` für ein Elicitierungsformular oder `elicitation_url_dialog` für eine Browser-URL-Anfrage, sobald Sie etwa sechs Sekunden nicht eingegeben haben. Beide teilen das gleiche Sechs-Sekunden-Gate wie `permission_prompt`: Der Timer startet, wenn der Dialog erscheint, und jeder Tastendruck verschiebt ihn.

  Eine Berechtigungsanfrage oder Elicitierung, die ankommt, während ein anderer Dialog auf dem Bildschirm ist, behält das gleiche Sechs-Sekunden-Gate, zeitlich von wenn die Anfrage ankommt. Seine Benachrichtigung kann Sie erreichen, während die Anfrage immer noch hinter dem offenen Dialog wartet.
</Note>

Claude Code zeitlich `permission_prompt` unterschiedlich in Sitzungen, in denen es Berechtigungsanfragen an den Agent SDK [`canUseTool` Callback](/docs/de/agent-sdk/user-input) sendet, was ist, wie Claude Desktop und die VS Code Erweiterung Claude Code hosten:

* Erwarten Sie `permission_prompt` etwa sechs Sekunden, nachdem Claude um Erlaubnis bittet. Claude Code verschiebt es nicht, während Sie eingeben.
* Wenn Sie oder ein [PermissionRequest](#permissionrequest) Hook früher antworten, führt Claude Code `permission_prompt` nicht aus.
* Setzen Sie [`CLAUDE_CODE_DISABLE_PERMISSION_PROMPT_NOTIFY_HOOKS`](/docs/de/env-vars) auf `1`, um `permission_prompt` in diesen Sitzungen auszuschalten.

Vor v2.1.233 wurde `permission_prompt` in diesen Sitzungen nicht ausgeführt.

Verwenden Sie separate Matcher, um verschiedene Handler je nach Benachrichtigungstyp auszuführen. Diese Konfiguration löst ein Berechtigungs-spezifisches Warnungsskript aus, wenn Claude Berechtigungsgenehmigung benötigt, und eine andere Benachrichtigung, wenn Claude untätig war:

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/permission-alert.sh"
          }
        ]
      },
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/idle-notification.sh"
          }
        ]
      }
    ]
  }
}
```

<h4 id="notification-input">
  Notification-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten Notification-Hooks `message` mit dem Benachrichtigungstext, ein optionaler `title` und `notification_type`, das angibt, welcher Typ ausgeführt wurde.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission",
  "title": "Permission needed",
  "notification_type": "permission_prompt"
}
```

Notification-Hooks können Benachrichtigungen nicht blockieren oder ändern. Claude Code verwirft ihre `systemMessage` und `continue` Felder, gibt aber immer noch [`terminalSequence`](#emit-terminal-notifications) aus, auf das sich das Desktop-Benachrichtigungsbeispiel verlässt. Notification-Hooks sind für Nebenwirkungen wie das Weiterleiten der Benachrichtigung an einen externen Service gedacht.

<h3 id="subagentstart">
  SubagentStart
</h3>

Wird ausgeführt, wenn Claude einen Subagenten mit dem Agent-Tool spawnt, wenn Claude [einen Subagenten fortsetzt](/docs/de/sub-agents#resume-subagents) und jedes Mal, wenn ein In-Process [Agent-Team](/docs/de/agent-teams) Teamkollege eine neue Nachricht verarbeitet. Unterstützt Matcher zum Filtern nach Agent-Typ-Name. Für integrierte Agenten ist dies der Agent-Name wie `general-purpose`, `Explore` oder `Plan`. Für [benutzerdefinierte Subagenten](/docs/de/sub-agents) ist dies das `name` Feld aus dem Agent-Frontmatter, nicht der Dateiname.

Für Subagenten, die von einem [Plugin](/docs/de/plugins/overview) versendet werden, ist der Agent-Typ der Plugin-scoped Identifikator wie `my-plugin:reviewer`, nicht der bloße Frontmatter-Name. Der Doppelpunkt platziert einen Plugin-scoped Namen auf dem regulären Ausdruckspfad, daher verankern Sie den Matcher mit `^` und `$` für einen genauen Treffer: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  SubagentStart-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten SubagentStart-Hooks `agent_id` mit dem eindeutigen Identifikator für den Subagenten und `agent_type` mit dem Agent-Namen, den der Matcher filtert.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SubagentStart",
  "agent_id": "agent-abc123",
  "agent_type": "Explore"
}
```

SubagentStart-Hooks können die Erstellung von Subagenten nicht blockieren, aber sie können Kontext in den Subagenten einfügen. Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, können Sie zurückgeben:

| Feld                | Beschreibung                                                                                                                                                                                  |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String, der zu Claudes Kontext am Anfang des Gesprächs des Subagenten hinzugefügt wird, vor seiner ersten Eingabeaufforderung. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

Wenn der Hook erneut für den gleichen Subagenten ausgeführt wird, injiziert Claude Code den zurückgegebenen Kontext nur, wenn der Kontext des Subagenten nicht bereits die Kopie aus einem früheren Lauf enthält. Die beim Start eingefügte Kopie bleibt bestehen, wobei der [Prompt-Cache](/docs/de/prompt-caching#subagents-and-the-cache) des Subagenten intakt bleibt. Nach [Auto-Komprimierung](/docs/de/sub-agents#auto-compaction) verwirft diese Kopie, injiziert Claude Code den Kontext des nächsten Laufs erneut.

<h3 id="subagentstop">
  SubagentStop
</h3>

Wird ausgeführt, wenn ein Claude Code Subagent mit dem Antworten fertig ist. Gleicht Agent-Typ ab, gleiche Werte wie SubagentStart.

<h4 id="subagentstop-input">
  SubagentStop-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten SubagentStop-Hooks `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path` und `last_assistant_message`. Das `agent_type` Feld ist der Wert, der zum Filtern des Matchers verwendet wird. Der `transcript_path` ist das Transkript der Hauptsitzung, während `agent_transcript_path` das eigene Transkript des Subagenten ist, das in einem verschachtelten `subagents/` Ordner gespeichert ist. Das `last_assistant_message` Feld enthält den Textinhalt der letzten Antwort des Subagenten, daher können Hooks darauf zugreifen, ohne die Transkript-Datei zu analysieren.

Nicht jedes SubagentStop-Ereignis kommt von einem Subagenten, den Claude spawnt. Claude Code führt auch interne Agenten für einige seiner eigenen Funktionen aus, wie [Eingabeaufforderungsvorschläge](/docs/de/interactive-mode#prompt-suggestions) und [`/btw` Seitenfragen](/docs/de/interactive-mode#side-questions-with-%2Fbtw), und SubagentStop wird ausgeführt, wenn einer davon fertig ist. Für diese Ereignisse ist `agent_type` der Agent-Name, den die Sitzung selbst ausführt, wie einer, der mit [`--agent`](/docs/de/cli-reference#cli-flags) oder der [`agent` Einstellung](/docs/de/settings-reference#agent) gesetzt ist, und eine leere Zeichenkette, wenn die Sitzung ohne einen läuft.

Ein `matcher`, der Agent-Typen benennt, gleicht keine leere `agent_type` ab. Ein Hook, dessen Matcher weggelassen, `""` oder `"*"` ist oder ein regulärer Ausdruck, der eine leere Zeichenkette abgleicht, wird auch für Ereignisse mit einer leeren `agent_type` ausgeführt.

Auf Claude Code v2.1.271 oder später liefert ein Subagent, der mit dem [`SubagentHandback`](/docs/de/tools-reference) Tool läuft, seinen Bericht durch dieses Tool, bevor er stoppt. Das `last_assistant_message` Feld enthält dann den Schließungstext des Subagenten, falls vorhanden, das ist nicht der gelieferte Bericht. Der Bericht ist die `message` Eingabe dieses Aufrufs, die ein `PreToolUse` oder `PostToolUse` Hook, der auf `SubagentHandback` abgleicht, als `tool_input.message` empfängt.

SubagentStop-Hooks erhalten auch die `background_tasks` und `session_crons` Arrays, die unter [Stop-Eingabe](#stop-input) beschrieben sind. Beide Arrays sind auf die Eltern-Sitzung scoped, nicht den Subagenten.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "SubagentStop",
  "stop_hook_active": false,
  "agent_id": "def456",
  "agent_type": "Explore",
  "agent_transcript_path": "~/.claude/projects/.../abc123/subagents/agent-def456.jsonl",
  "last_assistant_message": "Analysis complete. Found 3 potential issues...",
  "background_tasks": [],
  "session_crons": []
}
```

SubagentStop-Hooks verwenden das gleiche Entscheidungskontrollformat wie [Stop-Hooks](#stop-decision-control), einschließlich `hookSpecificOutput.additionalContext` mit `hookEventName` auf `"SubagentStop"` gesetzt, für Fehler-freies Feedback, das den Subagenten laufen lässt. Das Zurückgeben von `decision: "block"` mit einem `reason` lässt den Subagenten laufen und liefert `reason` an den Subagenten als seine nächste Anweisung. Ein Hook, der durch Beendigung mit 2 blockiert, liefert seine stderr-Nachricht auf die gleiche Weise. Um Kontext in die Eltern-Sitzung einzufügen, nachdem ein Subagent zurückkommt, verwenden Sie stattdessen einen [`PostToolUse`](#posttooluse) Hook auf dem `Agent` Tool.

<h3 id="taskcreated">
  TaskCreated
</h3>

Wird ausgeführt, wenn eine Aufgabe über das `TaskCreate` Tool erstellt wird. Verwenden Sie dies, um Benennungskonventionen durchzusetzen, Aufgabenbeschreibungen zu erfordern oder zu verhindern, dass bestimmte Aufgaben erstellt werden. In einer [Sitzung ohne die Task-Tools](/docs/de/tools-reference#task-tool-availability) wird dieses Ereignis nicht ausgeführt.

TaskCreated-Hooks unterstützen keine Matcher und werden bei jedem Vorkommen ausgeführt.

<h4 id="taskcreated-input">
  TaskCreated-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten TaskCreated-Hooks `task_id`, `task_subject` und optional `task_description`, `teammate_name` und `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Feld               | Beschreibung                                                                          |
| :----------------- | :------------------------------------------------------------------------------------ |
| `task_id`          | Identifikator der zu erstellenden Aufgabe                                             |
| `task_subject`     | Titel der Aufgabe                                                                     |
| `task_description` | Detaillierte Beschreibung der Aufgabe. Kann fehlen                                    |
| `teammate_name`    | Name des Teamkollegen, der die Aufgabe erstellt. Kann fehlen                          |
| `team_name`        | Veraltet. Sitzungs-abgeleiteter Team-Name; wird in einer zukünftigen Version entfernt |

<h4 id="taskcreated-decision-control">
  TaskCreated-Entscheidungskontrolle
</h4>

Ein TaskCreated-Hook kann die Erstellung auf zwei Wegen blockieren. In beiden Fällen löscht Claude Code die Aufgabe und gibt Ihre Nachricht an Claude als Fehler des Tools zurück. Claude Code ignoriert `continue: false` aus diesem Ereignis und Claude arbeitet weiter.

* **Exit-Code 2**: Claude Code gibt den stderr-Text als die Nachricht zurück.
* **JSON `{"decision": "block", "reason": "..."}`**: Claude Code gibt `reason` als die Nachricht zurück.

Dieses Beispiel blockiert Aufgaben, deren Betreff nicht dem erforderlichen Format folgt:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

if [[ ! "$TASK_SUBJECT" =~ ^\[TICKET-[0-9]+\] ]]; then
  echo "Task subject must start with a ticket number, e.g. '[TICKET-123] Add feature'" >&2
  exit 2
fi

exit 0
```

<h3 id="taskcompleted">
  TaskCompleted
</h3>

Wird ausgeführt, wenn eine Aufgabe als abgeschlossen markiert wird. Dies wird in zwei Situationen ausgeführt: wenn ein Agent eine Aufgabe explizit durch das TaskUpdate-Tool als abgeschlossen markiert oder wenn ein [Agent-Team](/docs/de/agent-teams) Teamkollege seinen Zug mit laufenden Aufgaben beendet. Verwenden Sie dies, um Abschluss-Kriterien wie bestandene Tests oder Lint-Überprüfungen durchzusetzen, bevor eine Aufgabe geschlossen werden kann.

TaskCompleted-Hooks unterstützen keine Matcher und werden bei jedem Vorkommen ausgeführt.

<h4 id="taskcompleted-input">
  TaskCompleted-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten TaskCompleted-Hooks `task_id`, `task_subject` und optional `task_description`, `teammate_name` und `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCompleted",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Feld               | Beschreibung                                                                          |
| :----------------- | :------------------------------------------------------------------------------------ |
| `task_id`          | Identifikator der zu abschließenden Aufgabe                                           |
| `task_subject`     | Titel der Aufgabe                                                                     |
| `task_description` | Detaillierte Beschreibung der Aufgabe. Kann fehlen                                    |
| `teammate_name`    | Name des Teamkollegen, der die Aufgabe abschließt. Kann fehlen                        |
| `team_name`        | Veraltet. Sitzungs-abgeleiteter Team-Name; wird in einer zukünftigen Version entfernt |

<h4 id="taskcompleted-decision-control">
  TaskCompleted-Entscheidungskontrolle
</h4>

TaskCompleted-Hooks unterstützen zwei Wege zur Steuerung des Aufgaben-Abschlusses:

* **Exit-Code 2**: Die Aufgabe wird nicht als abgeschlossen markiert und die stderr-Nachricht wird an das Modell als Feedback zurückgesendet.
* **JSON `{"continue": false, "stopReason": "..."}`**: Wenn ein Teamkollege, der seinen Zug beendet, das Ereignis ausgelöst hat, stoppt den Teamkollegen vollständig, was dem `Stop` Hook-Verhalten entspricht. Der `stopReason` wird dem Benutzer angezeigt. Wenn das `TaskUpdate` Tool das Ereignis ausgelöst hat, ignoriert Claude Code `continue: false`; Exit-Code 2 blockiert immer noch den Abschluss.

Dieses Beispiel führt Tests aus und blockiert den Aufgaben-Abschluss, wenn sie fehlschlagen:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Run the test suite
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

Wird ausgeführt, wenn der Haupt-Claude Code Agent mit dem Antworten fertig ist. Wird nicht ausgeführt, wenn der Stopp aufgrund einer Benutzerunterbrechung auftrat. API-Fehler führen stattdessen [StopFailure](#stopfailure) aus.

<Tip>
  Der [`/goal`](/docs/de/goal) Befehl ist eine integrierte Abkürzung für einen Sitzungs-scoped Eingabeaufforderungs-basierten Stop-Hook. Verwenden Sie ihn, wenn Sie möchten, dass Claude auf eine Bedingung hinarbeitet, ohne Hook-Konfiguration zu schreiben.
</Tip>

<h4 id="stop-input">
  Stop-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten Stop-Hooks `stop_hook_active`, `last_assistant_message`, `background_tasks` und `session_crons`. Das `stop_hook_active` Feld ist `true`, wenn Claude Code bereits als Ergebnis eines Stop-Hooks fortgesetzt wird. Überprüfen Sie diesen Wert oder verarbeiten Sie das Transkript, um zu vermeiden, auf einer Bedingung zu blockieren, die sich nie auflösen wird. Claude Code überschreibt den Hook und beendet den Zug nach 8 aufeinanderfolgenden Blockierungen.

Das `last_assistant_message` Feld enthält den Textinhalt von Claudes letzter Antwort, daher können Hooks darauf zugreifen, ohne die Transkript-Datei zu analysieren. Für Hooks, die auf dem gerade abgeschlossenen Zug handeln, wie Vorlesen oder Benachrichtigungs-Hooks, verwenden Sie dieses Feld statt `transcript_path` zu lesen: Die Transkript-Datei ist nicht garantiert, die letzte Nachricht bei Stop-Zeit auf allen Versionen einzuschließen.

Die `background_tasks` und `session_crons` Arrays ermöglichen es Hooks, "Sitzung ist fertig" von "Sitzung ist pausiert und wartet auf Hintergrund-Arbeit, um sie aufzuwecken" zu unterscheiden. Beide Arrays sind vorhanden, wenn die Task-Registry erreichbar ist und sind leer, wenn nichts in Flug oder geplant ist.

Jeder Eintrag in `background_tasks` beschreibt eine laufende Aufgabe und verwendet diese Felder:

| Feld          | Beschreibung                                                                                                                                                                                                                                                                             |
| :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Aufgaben-Identifikator                                                                                                                                                                                                                                                                   |
| `type`        | Freundliche Aufgaben-Typ-Beschriftung wie `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session` oder `MCP task`. Jede Beschriftung identifiziert, welche Claude Code Funktion die Aufgabe erstellt hat. Fällt auf den rohen Diskriminanten für unbekannte Typen zurück |
| `status`      | Aktueller Aufgaben-Status                                                                                                                                                                                                                                                                |
| `description` | Freitext-Beschreibung, begrenzt auf 1000 Zeichen mit einem In-String `… [+N chars]` Marker, wenn gekürzt                                                                                                                                                                                 |
| `command`     | Shell-Befehlszeile, begrenzt auf 1000 Zeichen. Nur für `shell` Aufgaben vorhanden                                                                                                                                                                                                        |
| `agent_type`  | Subagenten-Typ-Name. Nur für `subagent` Aufgaben vorhanden                                                                                                                                                                                                                               |
| `server`      | MCP-Server-Name. Nur für `monitor` und `MCP task` Aufgaben vorhanden                                                                                                                                                                                                                     |
| `tool`        | MCP-Tool-Name. Nur für `monitor` und `MCP task` Aufgaben vorhanden                                                                                                                                                                                                                       |
| `name`        | Workflow-Name. Nur für `workflow` Aufgaben vorhanden                                                                                                                                                                                                                                     |

Jeder Eintrag in `session_crons` beschreibt einen Sitzungs-scoped geplanten Aufweck, stammt von `CronCreate`, `ScheduleWakeup` und `/loop`:

| Feld        | Beschreibung                                                                                                                             |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Cron-Aufgaben-Identifikator                                                                                                              |
| `schedule`  | Cron-Ausdruck, z. B. `0 9 * * 1-5`                                                                                                       |
| `recurring` | `false` für einmalige Aufwecke, deren Zeitplan eine einzelne Feuerzeit kodiert, `true` für Aufgaben, die bei jedem Treffer erneut feuern |
| `prompt`    | Eingabeaufforderung, die eingereicht wird, wenn der Cron feuert, begrenzt auf 1000 Zeichen mit dem gleichen `… [+N chars]` Marker        |

Dieses Beispiel zeigt eine Stop-Eingabe mit einer laufenden Shell-Aufgabe und einem wiederkehrenden Cron:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": true,
  "last_assistant_message": "I've completed the refactoring. Here's a summary...",
  "background_tasks": [
    {
      "id": "task-001",
      "type": "shell",
      "status": "running",
      "description": "tail logs",
      "command": "tail -f /var/log/syslog"
    }
  ],
  "session_crons": [
    {
      "id": "cron-001",
      "schedule": "0 9 * * 1-5",
      "recurring": true,
      "prompt": "check the build"
    }
  ]
}
```

<h4 id="stop-decision-control">
  Stop-Entscheidungskontrolle
</h4>

`Stop` und `SubagentStop` Hooks können steuern, ob Claude fortgesetzt wird. Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, kann Ihr Hook-Skript diese ereignisspezifischen Felder zurückgeben:

| Feld                                   | Beschreibung                                                                                                                                                                                                         |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` verhindert, dass Claude stoppt. Weglassen, um Claude zu stoppen                                                                                                                                            |
| `reason`                               | Erforderlich, wenn `decision` `"block"` ist. Sagt Claude, warum es fortgesetzt werden sollte                                                                                                                         |
| `hookSpecificOutput.additionalContext` | Fehler-freies Feedback für Claude. Das Gespräch wird fortgesetzt, damit Claude darauf handeln kann, aber im Gegensatz zu `decision: "block"` wird es im Transkript als Hook-Feedback statt als Hook-Fehler angezeigt |

Ein Hook, der durch Beendigung mit 2 blockiert, wird auf die gleiche Weise wie `reason` weitergeleitet: Claude empfängt die stderr-Nachricht als Erklärung, warum es fortgesetzt werden sollte.

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Verwenden Sie `additionalContext`, wenn der Hook wie beabsichtigt funktioniert und Claude Anleitung gibt, wie "führe die Test-Suite vor dem Beenden aus". Es hält das Gespräch durch die gleichen Loop-Schutzmaßnahmen wie `decision: "block"` am Laufen, nämlich die `stop_hook_active` Eingabe und die 8-aufeinanderfolgende-Fortsetzungs-Kappe, aber das Transkript kennzeichnet es als `Stop hook feedback` und es wird keine Hook-Fehler-Benachrichtigung angezeigt:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "additionalContext": "Please run the test suite before finishing"
  }
}
```

<h3 id="stopfailure">
  StopFailure
</h3>

Wird statt [Stop](#stop) ausgeführt, wenn der Zug aufgrund eines API-Fehlers endet. Claude Code ignoriert die Ausgabe und den Exit-Code des Hooks, abgesehen von [`terminalSequence`](#emit-terminal-notifications). Verwenden Sie dies, um Fehler zu protokollieren, Warnungen zu senden oder Wiederherstellungsmaßnahmen zu ergreifen, wenn Claude aufgrund von Ratenlimits, Authentifizierungsproblemen oder anderen API-Fehlern keine Antwort abschließen kann.

<h4 id="stopfailure-input">
  StopFailure-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten StopFailure-Hooks `error`, optional `error_details` und optional `last_assistant_message`. Das `error` Feld identifiziert den Fehlertyp und wird zum Filtern des Matchers verwendet.

| Feld                     | Beschreibung                                                                                                                                                                                                                                                  |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `error`                  | Fehlertyp: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error` oder `unknown`               |
| `error_details`          | Zusätzliche Details zum Fehler, wenn verfügbar                                                                                                                                                                                                                |
| `last_assistant_message` | Der gerenderte Fehlertext, der im Gespräch angezeigt wird. Im Gegensatz zu `Stop` und `SubagentStop`, wo dieses Feld Claudes Gesprächsausgabe enthält, enthält es für `StopFailure` die API-Fehler-Zeichenkette selbst, wie `"API Error: Rate limit reached"` |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "StopFailure",
  "error": "rate_limit",
  "error_details": "429 Too Many Requests",
  "last_assistant_message": "API Error: Rate limit reached"
}
```

StopFailure-Hooks haben keine Entscheidungskontrolle. Sie werden nur zu Benachrichtigungs- und Protokollierungszwecken ausgeführt.

<h3 id="teammateidle">
  TeammateIdle
</h3>

Wird ausgeführt, wenn ein [Agent-Team](/docs/de/agent-teams) Teamkollege nach Abschluss seines Zugs untätig wird. Verwenden Sie dies, um Qualitäts-Gates durchzusetzen, bevor ein Teamkollege die Arbeit einstellt, wie das Erfordern bestandener Lint-Überprüfungen oder das Überprüfen, dass Ausgabe-Dateien existieren.

TeammateIdle-Hooks unterstützen keine Matcher und werden bei jedem Vorkommen ausgeführt.

<h4 id="teammateidle-input">
  TeammateIdle-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten TeammateIdle-Hooks `teammate_name` und `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TeammateIdle",
  "teammate_name": "researcher",
  "team_name": "session-a1b2c3d4"
}
```

| Feld            | Beschreibung                                                                          |
| :-------------- | :------------------------------------------------------------------------------------ |
| `teammate_name` | Name des Teamkollegen, der untätig wird                                               |
| `team_name`     | Veraltet. Sitzungs-abgeleiteter Team-Name; wird in einer zukünftigen Version entfernt |

<h4 id="teammateidle-decision-control">
  TeammateIdle-Entscheidungskontrolle
</h4>

TeammateIdle-Hooks unterstützen zwei Wege zur Steuerung des Teamkollegen-Verhaltens:

* **Exit-Code 2**: Der Teamkollege empfängt die stderr-Nachricht als Feedback und arbeitet weiter, anstatt untätig zu werden.
* **JSON `{"continue": false, "stopReason": "..."}`**: Stoppt den Teamkollegen vollständig, was dem `Stop` Hook-Verhalten entspricht. Der `stopReason` wird dem Benutzer angezeigt.

Dieses Beispiel überprüft, dass ein Build-Artefakt existiert, bevor ein Teamkollege untätig wird:

```bash theme={null}
#!/bin/bash

if [ ! -f "./dist/output.js" ]; then
  echo "Build artifact missing. Run the build before stopping." >&2
  exit 2
fi

exit 0
```

<h3 id="configchange">
  ConfigChange
</h3>

Wird ausgeführt, wenn sich eine Konfigurationsdatei während einer Sitzung ändert. Verwenden Sie dies, um Einstellungs-Änderungen zu überprüfen, Sicherheitsrichtlinien durchzusetzen oder nicht autorisierte Änderungen an Konfigurationsdateien zu blockieren.

Claude Code führt ConfigChange-Hooks aus, wenn sich eine Einstellungsdatei, eine verwaltete Richtlinien-Datei oder eine Skill-Datei ändert. Für verwaltete Richtlinien wird es nur ausgeführt, wenn `managed-settings.json` oder eine Datei in `managed-settings.d/` sich ändert. Es wendet [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) und Änderungen an macOS verwalteten Einstellungen oder Windows Registry-Richtlinien ohne Ausführung an. Auf WSL mit [`wslInheritsWindowsSettings`](/docs/de/settings-reference#wslinheritswindowssettings) wendet es auch eine geänderte Windows-seitige verwaltete Einstellungsdatei bei seiner Richtlinien-Abfrage ohne Ausführung an.

Der Matcher filtert auf die Konfigurationsquelle:

| Matcher            | Wann wird es ausgelöst                                                       |
| :----------------- | :--------------------------------------------------------------------------- |
| `user_settings`    | `~/.claude/settings.json` ändert sich                                        |
| `project_settings` | `.claude/settings.json` ändert sich                                          |
| `local_settings`   | `.claude/settings.local.json` ändert sich                                    |
| `policy_settings`  | `managed-settings.json` oder eine Datei in `managed-settings.d/` ändert sich |
| `skills`           | Eine Skill-Datei in `.claude/skills/` ändert sich                            |

Dieses Beispiel protokolliert alle Konfigurationsänderungen für Sicherheits-Audits:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/audit-config-change.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

<h4 id="configchange-input">
  ConfigChange-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten ConfigChange-Hooks `source` und optional `file_path`. Das `source` Feld gibt an, welcher Konfigurationstyp sich geändert hat, und `file_path` stellt den Pfad zur spezifischen Datei bereit, die geändert wurde.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ConfigChange",
  "source": "project_settings",
  "file_path": "/Users/.../my-project/.claude/settings.json"
}
```

<h4 id="configchange-decision-control">
  ConfigChange-Entscheidungskontrolle
</h4>

ConfigChange-Hooks können Konfigurationsänderungen blockieren, damit sie nicht wirksam werden. Verwenden Sie Exit-Code 2 oder ein JSON `decision`, um die Änderung zu verhindern. Wenn blockiert, werden die neuen Einstellungen nicht auf die laufende Sitzung angewendet.

| Feld       | Beschreibung                                                                                                  |
| :--------- | :------------------------------------------------------------------------------------------------------------ |
| `decision` | `"block"` verhindert, dass die Konfigurationsänderung angewendet wird. Weglassen, um die Änderung zu erlauben |
| `reason`   | Akzeptiert, aber nie angezeigt                                                                                |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

`policy_settings` Änderungen können nicht blockiert werden. Hooks werden immer noch für `policy_settings` Quellen ausgeführt, wenn sich eine verwaltete Einstellungsdatei auf der Maschine ändert, daher können Sie sie verwenden, um diese Bearbeitungen zu protokollieren, aber jede Blockierungs-Entscheidung wird ignoriert. Dies stellt sicher, dass unternehmens-verwaltete Einstellungen immer wirksam werden. Claude Code führt `ConfigChange` Hooks nicht aus, wenn [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) ankommen oder aktualisiert werden.

Claude Code handelt die Blockierungs-Entscheidung aus der JSON-Ausgabe eines ConfigChange-Hooks und verwirft `systemMessage` und `continue`. Eine blockierte Änderung zeigt keine Nachricht für Sie oder Claude, ob Sie mit `reason` oder mit stderr bei Beendigung mit 2 blockieren. Claude Code schreibt nur eine Zeile in das Debug-Protokoll.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Wird ausgeführt, wenn ein Shell-Befehl in der Hauptkonversation das Arbeitsverzeichnis ändert, z. B. wenn Claude einen `cd` Befehl ausführt. Verwenden Sie dies, um auf Verzeichniswechsel zu reagieren: Umgebungsvariablen neu laden, projektspezifische Toolchains aktivieren oder Setup-Skripte automatisch ausführen. Paart mit [FileChanged](#filechanged) für Tools wie [direnv](https://direnv.net/), die Pro-Verzeichnis-Umgebung verwalten.

CwdChanged-Hooks haben Zugriff auf [`CLAUDE_ENV_FILE`](#persist-environment-variables). Variablen, die in diese Datei geschrieben werden, bleiben in nachfolgenden Bash-Befehlen bestehen, bis zum nächsten CwdChanged-Ereignis, wenn Claude Code sie löscht.

CwdChanged unterstützt keine Matcher und wird bei jedem Vorkommen ausgeführt.

<h4 id="cwdchanged-input">
  CwdChanged-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten CwdChanged-Hooks `old_cwd` und `new_cwd`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project/src",
  "hook_event_name": "CwdChanged",
  "old_cwd": "/Users/my-project",
  "new_cwd": "/Users/my-project/src"
}
```

<h4 id="cwdchanged-output">
  CwdChanged-Ausgabe
</h4>

Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, können CwdChanged-Hooks `watchPaths` zurückgeben, um dynamisch zu setzen, welche Dateipfade [FileChanged](#filechanged) überwacht:

| Feld         | Beschreibung                                                                                                                                                                                                                                                              |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `watchPaths` | Array von absoluten Pfaden. Ersetzt die aktuelle dynamische Überwachungsliste. Pfade aus Ihrer `matcher` Konfiguration werden immer überwacht. Das Zurückgeben eines leeren Arrays löscht die dynamische Liste, was typisch ist, wenn ein neues Verzeichnis betreten wird |

CwdChanged-Hooks haben keine Entscheidungskontrolle. Sie können den Verzeichniswechsel nicht blockieren.

Claude Code liest `watchPaths` und `systemMessage` aus ihrer JSON-Ausgabe und verwirft `continue`. In interaktiven Sitzungen zeigt es die `systemMessage` als kurze Terminal-Benachrichtigung. Die Nachricht erreicht nicht den SDK-Nachrichtenstrom.

<h3 id="directoryadded">
  DirectoryAdded
</h3>

Wird ausgeführt, nachdem Sie ein Arbeitsverzeichnis während einer Sitzung mit dem `/add-dir` Befehl hinzufügen oder nachdem ein SDK-Client eines mit der `register_repo_root` Kontroll-Anfrage hinzufügt. Verwenden Sie dies, um ein neu hinzugefügtes Repository vorzubereiten, z. B. durch Installation seiner Abhängigkeiten.

Claude Code führt dieses Ereignis nicht aus, wenn:

* Sie ein Verzeichnis mit dem `--add-dir` Start-Flag übergeben; [SessionStart](#sessionstart) deckt diese Verzeichnisse ab
* Sie ein Verzeichnis auf der `/permissions` Workspace-Registerkarte hinzufügen
* Sie ein Verzeichnis hinzufügen, das bereits ein Arbeitsverzeichnis ist oder in einem liegt

Claude Code führt DirectoryAdded aus, nachdem Sandbox- und Berechtigungsstatus aktualisiert wurden, daher sehen Sandbox-Tools das neue Verzeichnis bereits, wenn Ihr Hook ausgeführt wird. Hook-Befehle selbst werden unsandboxed ausgeführt.

Claude Code wartet nicht auf den Hook: Das Hinzufügen wird sofort abgeschlossen und der Hook wird im Hintergrund mit dem 600-Sekunden-Standard-Timeout ausgeführt.

Der Matcher filtert, wie das Verzeichnis hinzugefügt wurde:

| Matcher              | Wann wird es ausgelöst                                                                  |
| :------------------- | :-------------------------------------------------------------------------------------- |
| `slash_command`      | Sie fügen ein Verzeichnis mit `/add-dir` hinzu                                          |
| `register_repo_root` | Ein SDK-Client fügt ein Verzeichnis mit der `register_repo_root` Kontroll-Anfrage hinzu |

<h4 id="directoryadded-input">
  DirectoryAdded-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten DirectoryAdded-Hooks `directory` und `source`.

| Feld        | Beschreibung                                                                                                                     |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `directory` | Absoluter Pfad des hinzugefügten Verzeichnisses                                                                                  |
| `source`    | Wie das Verzeichnis hinzugefügt wurde, `"slash_command"` für `/add-dir` oder `"register_repo_root"` für die SDK-Kontroll-Anfrage |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "DirectoryAdded",
  "directory": "/Users/my-other-repo",
  "source": "slash_command"
}
```

DirectoryAdded-Hooks haben keine Entscheidungskontrolle. Sie können das Hinzufügen nicht blockieren, das bereits abgeschlossen ist, wenn der Hook ausgeführt wird. Claude Code verwirft das `continue` Feld aus ihrer JSON-Ausgabe und zeigt den Rest unterschiedlich pro Quelle:

* `slash_command`: Claude Code liefert den `systemMessage` des Hooks an Claude als Kontext beim nächsten Gesprächszug, anstatt ihn Ihnen zu zeigen. Eine Anzahl fehlgeschlagener Hooks erscheint im Transkript. Vollständige Fehlerausgabe geht zum Debug-Protokoll
* `register_repo_root`: Claude Code schreibt `systemMessage` Ausgabe und Fehlerausgabe nur zum Debug-Protokoll

<h3 id="filechanged">
  FileChanged
</h3>

Wird ausgeführt, wenn sich eine überwachte Datei auf der Festplatte ändert. Claude Code erkennt Änderungen mit einem Dateisystem-Watcher, nicht durch Überprüfung von Tool-Aufrufen, daher wird der Hook unabhängig davon ausgeführt, was die Datei geändert hat: ein `Edit` oder `Write` Tool-Aufruf, ein Skript, das Claude mit `Bash` ausführt, oder ein Prozess außerhalb von Claude Code. Ein häufiger Anwendungsfall ist das Neuladen von Umgebungsvariablen, wenn sich Projekt-Konfigurationsdateien ändern.

Der `matcher` für dieses Ereignis dient zwei Rollen:

* **Erstelle die Überwachungsliste**: Der Wert wird auf `|` aufgeteilt und jedes Segment wird als wörtlicher Dateiname im Arbeitsverzeichnis registriert, daher überwacht `".envrc|.env"` genau diese zwei Dateien. Regex-Muster sind hier nicht nützlich: Ein Wert wie `^\.env` würde eine Datei wörtlich namens `^\.env` überwachen.
* **Filtere, welche Hooks ausgeführt werden**: Wenn sich eine überwachte Datei ändert, wird der gleiche Wert verwendet, um zu filtern, welche Hook-Gruppen mit den Standard-[Matcher-Regeln](#matcher-patterns) gegen den Dateinamen der geänderten Datei ausgeführt werden.

Dieses Beispiel normalisiert Zeilenumbrüche in `data.csv` nach jeder Änderung, einschließlich eines `Bash` Befehls oder eines externen Skripts, das die Datei umschreibt:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": "data.csv",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/normalize-line-endings.sh"
          }
        ]
      }
    ]
  }
}
```

Der Hook liest den geänderten Dateipfad aus dem `file_path` Feld der [JSON-Eingabe](#filechanged-input) von stdin. Seine `grep` Wache testet auf das gleiche, das `perl` entfernt, einen CR am Ende einer Zeile, daher wird der Lauf nach einer Normalisierung beendet, ohne die Datei zu berühren. Eine lockere Wache schleift für immer, weil `perl -i` die Datei umschreibt, auch wenn es nichts ersetzt und Claude Code den Hook nach jedem Umschreiben erneut ausführt. Speichern Sie dieses Skript unter `/path/to/normalize-line-endings.sh` und machen Sie es ausführbar:

```bash theme={null}
#!/bin/bash
FILE=$(jq -r .file_path)
if grep -q $'\r$' "$FILE"; then
  perl -pi -e 's/\r$//' "$FILE"
fi
```

Um zu bestätigen, dass der Hook funktioniert, bitten Sie Claude, eine CRLF-Zeile mit einem `Bash` Befehl zu `data.csv` anzuhängen. Claude Code führt den Hook aus und die Datei endet mit LF-Umbrüchen.

Um Dateien zu überwachen, die Sie nicht im Voraus benennen können, geben Sie [`watchPaths`](#filechanged-output) von einem Hook zurück, um die Überwachungsliste dynamisch zu aktualisieren. Claude Code startet den Watcher nur, wenn etwas eine Datei zum Überwachen benennt, daher seeden Sie die Liste mit einer FileChanged-Gruppe, deren Matcher mindestens eine Datei benennt, oder mit einem [SessionStart](#sessionstart-decision-control) oder [CwdChanged](#cwdchanged) Hook, der `watchPaths` zurückgibt. Der Matcher filtert immer noch, welche Hook-Gruppen ausgeführt werden, wenn sich eine überwachte Datei ändert, daher geben Sie der Gruppe, die dynamische Pfade verarbeitet, einen weggelassenen Matcher, der jede überwachte Datei abgleicht und nichts zur Überwachungsliste hinzufügt. Ein `"*"` Matcher gleicht auch jede Datei ab, aber Claude Code registriert ihn in der Überwachungsliste wie jeden anderen Wert, als wörtliche Datei namens `*`.

FileChanged-Hooks haben Zugriff auf [`CLAUDE_ENV_FILE`](#persist-environment-variables). Variablen, die in diese Datei geschrieben werden, bleiben in nachfolgenden Bash-Befehlen bestehen, bis zum nächsten [CwdChanged](#cwdchanged) Ereignis, wenn Claude Code sie löscht.

<h4 id="filechanged-input">
  FileChanged-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten FileChanged-Hooks `file_path` und `event`.

| Feld        | Beschreibung                                                                                                                     |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `file_path` | Absoluter Pfad zur Datei, die sich geändert hat                                                                                  |
| `event`     | Was passiert ist: `"change"` für eine geänderte Datei, `"add"` für eine erstellte Datei oder `"unlink"` für eine gelöschte Datei |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "FileChanged",
  "file_path": "/Users/my-project/.envrc",
  "event": "change"
}
```

<h4 id="filechanged-output">
  FileChanged-Ausgabe
</h4>

Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, können FileChanged-Hooks `watchPaths` zurückgeben, um dynamisch zu aktualisieren, welche Dateipfade überwacht werden:

| Feld         | Beschreibung                                                                                                                                                                                                                                                           |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array von absoluten Pfaden. Ersetzt die aktuelle dynamische Überwachungsliste. Pfade aus Ihrer `matcher` Konfiguration werden immer überwacht. Verwenden Sie dies, wenn Ihr Hook-Skript basierend auf der geänderten Datei zusätzliche Dateien zum Überwachen entdeckt |

FileChanged-Hooks haben keine Entscheidungskontrolle. Sie können die Dateiänderung nicht blockieren.

Claude Code liest `watchPaths` und `systemMessage` aus ihrer JSON-Ausgabe und verwirft `continue`. In interaktiven Sitzungen zeigt es die `systemMessage` als kurze Terminal-Benachrichtigung. Die Nachricht erreicht nicht den SDK-Nachrichtenstrom.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Wird ausgeführt, wenn ein Worktree erstellt wird, ob von `claude --worktree`, von einem [Subagenten mit `isolation: "worktree"`](/docs/de/sub-agents#choose-the-subagent-scope) oder für eine [Hintergrund-Sitzung](/docs/de/agent-view#how-file-edits-are-isolated), die Claude Code in ihrem eigenen Worktree isoliert. Standardmäßig erstellt Claude Code die isolierte Arbeitskopie mit `git worktree`. Das Konfigurieren eines WorktreeCreate-Hooks ersetzt dieses Standard-Git-Verhalten, sodass Sie ein anderes Versionskontrollsystem wie SVN, Perforce oder Mercurial verwenden können.

Da der Hook das Standard-Verhalten vollständig ersetzt, wird [`.worktreeinclude`](/docs/de/worktrees#copy-gitignored-files-into-worktrees) nicht verarbeitet. Wenn Sie lokale Konfigurationsdateien wie `.env` in den neuen Worktree kopieren müssen, tun Sie dies in Ihrem Hook-Skript.

Der Hook muss den Pfad zum erstellten Worktree-Verzeichnis zurückgeben. Claude Code verwendet diesen Pfad als Arbeitsverzeichnis für die isolierte Sitzung. Siehe [WorktreeCreate-Ausgabe](#worktreecreate-output), wie jeder Hook-Typ den Pfad zurückgibt.

Claude Code handelt den Erfolg des Hooks und den zurückgegebenen Pfad und verwirft `systemMessage` und `continue`.

Dieses Beispiel erstellt eine SVN-Arbeitskopie und druckt den Pfad für Claude Code zur Verwendung. Ersetzen Sie die Repository-URL durch Ihre eigene:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Der Hook liest den Worktree `name` aus der JSON-Eingabe von stdin, checkt eine frische Kopie in ein neues Verzeichnis aus und druckt den Verzeichnispath. Das `echo` auf der letzten Zeile ist das, was Claude Code als Worktree-Pfad liest. Leiten Sie jede andere Ausgabe zu stderr um, damit sie nicht mit dem Pfad interferiert.

<h4 id="worktreecreate-input">
  WorktreeCreate-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten WorktreeCreate-Hooks das `name` Feld. Dies ist ein Slug-Identifikator für den neuen Worktree, entweder vom Benutzer angegeben oder automatisch generiert, z. B. `bold-oak-a3f2`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeCreate",
  "name": "feature-auth"
}
```

<h4 id="worktreecreate-output">
  WorktreeCreate-Ausgabe
</h4>

WorktreeCreate-Hooks verwenden nicht das Standard-Erlauben/Blockieren-Entscheidungsmodell. Stattdessen bestimmt der Erfolg oder Fehler des Hooks das Ergebnis. Der Hook muss den Pfad zum erstellten Worktree-Verzeichnis zurückgeben:

* **Command-Hooks** (`type: "command"`): Drucken Sie den Pfad als letzte nicht-leere Zeile von stdout. Claude Code entfernt ANSI-Escape-Codes, bevor diese Zeile gelesen wird, daher werden Shell-Start-Banner, die vor Ihrem `echo` gedruckt werden, ignoriert. Leiten Sie jede andere Hook-Ausgabe zu stderr um.
* **HTTP-Hooks** (`type: "http"`): Geben Sie `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` im Antwort-Body zurück.

Wenn der Hook fehlschlägt oder keinen Pfad erzeugt, schlägt die Worktree-Erstellung mit einem Fehler fehl.

Claude Code löst einen relativen Pfad gegen das Verzeichnis auf, in dem der Hook lief, und bricht alle `.` oder `..` Segmente darin zusammen. Wenn der resultierende Pfad kein Verzeichnis ist, das Claude Code betreten kann, druckt die Sitzung einen Fehler, der den Pfad benennt, und beendet sich mit Code 1.

Claude Code lehnt einen absoluten Pfad ab, der `.` oder `..` Segmente enthält, und jeden Pfad, der durch einen Symlink unterhalb der Repository-Root geht, da ein Symlink, der zum Repository committed ist, den Worktree außerhalb davon umleiten könnte. Der Fehler benennt die abgelehnte Komponente. Geben Sie einen normalisierten Pfad zurück, der nicht durch einen Symlink im Repository geht. Vor v2.1.216 folgte die Worktree-Erstellung dem Pfad des Hooks ohne diese Überprüfung.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Wird ausgeführt, wenn ein Worktree entfernt wird. Dies ist das Bereinigungsgegenüber zu [WorktreeCreate](#worktreecreate). Das Ereignis wird ausgeführt, wenn:

* Sie eine `--worktree` Sitzung beenden und wählen, sie zu entfernen
* Ein Subagent mit `isolation: "worktree"` beendet
* Sie eine [Hintergrund-Sitzung](/docs/de/agent-view#what-deleting-a-session-removes) löschen, deren Worktree der Hook erstellt hat

Für Git-basierte Worktrees verarbeitet Claude Code die Bereinigung automatisch mit `git worktree remove`. Wenn Sie einen WorktreeCreate-Hook für ein nicht-Git-Versionskontrollsystem konfiguriert haben, paaren Sie ihn mit einem WorktreeRemove-Hook, um die Bereinigung zu verarbeiten. Ohne einen wird das Worktree-Verzeichnis auf der Festplatte gelassen.

Claude Code verwirft die [JSON-Ausgabefelder](#json-output) eines WorktreeRemove-Hooks, wie `systemMessage` und `continue`.

Für einen Hintergrund-Sitzungs-Löschung überprüft Claude Code den gespeicherten Worktree-Pfad, bevor der Hook ausgeführt wird, und lehnt einen Pfad ab, der ein Symlink ist oder durch einen unterhalb der Repository-Root geht. Der Hook wird für einen Worktree, der immer noch Dateien enthält, nur ausgeführt, wenn Sie den Löschvorgang in [Agent-Ansicht](/docs/de/agent-view#what-deleting-a-session-removes) bestätigen; für einen solchen Worktree behält [`claude rm`](/docs/de/agent-view#manage-sessions-from-the-shell) die Sitzung und den Worktree statt. Vor v2.1.216 wurde der Hook auf dem gespeicherten Pfad ohne diese Überprüfungen ausgeführt.

Claude Code übergibt den von WorktreeCreate zurückgegebenen Pfad als `worktree_path` in der Hook-Eingabe. Dieses Beispiel liest diesen Pfad und entfernt das Verzeichnis:

```json theme={null}
{
  "hooks": {
    "WorktreeRemove": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'jq -r .worktree_path | xargs rm -rf'"
          }
        ]
      }
    ]
  }
}
```

<h4 id="worktreeremove-input">
  WorktreeRemove-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten WorktreeRemove-Hooks das `worktree_path` Feld, das der absolute Pfad zum entfernten Worktree ist.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

Der Exit-Code eines WorktreeRemove-Hooks entscheidet das Ergebnis. Wenn ein Hook mit nicht-Null beendet wird und das Verzeichnis bei `worktree_path` immer noch danach existiert, schlägt die Entfernung fehl:

* Der Worktree bleibt auf der Festplatte, und der Hook-Befehl und stderr gehen zum [Debug-Protokoll](#debug-hooks).
* Wenn Sie eine Hintergrund-Sitzung löschten, bleibt die Sitzung auch. Die Verweigerungsmeldung in [Agent-Ansicht](/docs/de/agent-view#what-deleting-a-session-removes) meldet, wie der Hook endete, wie `exited 1`, zitiert den Anfang seines stderr und sagt, ob das Löschen der Sitzung erneut das Verzeichnis trotzdem entfernt.

<h3 id="precompact">
  PreCompact
</h3>

Wird ausgeführt, bevor Claude Code einen Komprimierungsvorgang ausführen möchte.

Der Matcher-Wert gibt an, ob die Komprimierung manuell oder automatisch ausgelöst wurde:

| Matcher  | Wann wird es ausgelöst                                                                                                         |
| :------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `manual` | `/compact`                                                                                                                     |
| `auto`   | Auto-Komprimierung, wenn das Gespräch das [Auto-Komprimierungs-Fenster](/docs/de/model-config#set-the-auto-compact-window) erreicht |

Beenden Sie mit Code 2, um die Komprimierung zu blockieren. Für ein manuelles `/compact` wird die stderr-Nachricht dem Benutzer angezeigt. Sie können auch blockieren, indem Sie JSON mit `"decision": "block"` zurückgeben.

Das Blockieren der automatischen Komprimierung hat unterschiedliche Auswirkungen, je nachdem, wann es ausgeführt wird. Wenn die Komprimierung proaktiv ausgelöst wurde, bevor das Kontext-Limit erreicht wurde, überspringt Claude Code sie und das Gespräch wird unkomprimiert fortgesetzt. Wenn die Komprimierung ausgelöst wurde, um sich von einem Kontext-Limit-Fehler zu erholen, der bereits von der API zurückgegeben wurde, zeigt sich der zugrunde liegende Fehler und die aktuelle Anfrage schlägt fehl.

Claude Code verwirft die `systemMessage` und `continue` Felder eines PreCompact-Hooks.

<h4 id="precompact-input">
  PreCompact-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten PreCompact-Hooks `trigger` und `custom_instructions`. Für `manual` enthält `custom_instructions` das, was der Benutzer in `/compact` übergibt und ist `null`, wenn sie nichts übergeben. Für `auto` ist `custom_instructions` `null`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": null
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Wird ausgeführt, nachdem Claude Code einen Komprimierungsvorgang abgeschlossen hat. Verwenden Sie dieses Ereignis, um auf den neuen komprimierten Zustand zu reagieren, z. B. um die generierte Zusammenfassung zu protokollieren oder externen Zustand zu aktualisieren. Claude Code verwirft die `systemMessage` und `continue` Felder eines PostCompact-Hooks.

Die gleichen Matcher-Werte gelten wie für `PreCompact`:

| Matcher  | Wann wird es ausgelöst                                                                                                              |
| :------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `manual` | Nach `/compact`                                                                                                                     |
| `auto`   | Nach Auto-Komprimierung, wenn das Gespräch das [Auto-Komprimierungs-Fenster](/docs/de/model-config#set-the-auto-compact-window) erreicht |

<h4 id="postcompact-input">
  PostCompact-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten PostCompact-Hooks `trigger` und `compact_summary`. Das `compact_summary` Feld enthält die Gesprächs-Zusammenfassung, die vom Komprimierungsvorgang generiert wurde.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostCompact",
  "trigger": "manual",
  "compact_summary": "Summary of the compacted conversation..."
}
```

PostCompact-Hooks haben keine Entscheidungskontrolle. Sie können das Komprimierungs-Ergebnis nicht beeinflussen, können aber Folge-Aufgaben ausführen.

<h3 id="premodelswitch">
  PreModelSwitch
</h3>

Wird ausgeführt, bevor Claude Code einen Modell-Wechsel anwendet, den Sie oder ein Client angefordert haben. Verwenden Sie ihn, um einen Wechsel zu blockieren, eine Bestätigung zu erfordern oder zu zeigen, was der Wechsel kostet, bevor er passiert.

PreModelSwitch erfordert Claude Code v2.1.251 oder später. Claude Code führt ihn für diese Anfragen aus:

* `/model <name>` und der `/model` Picker
* Der `Option+P` oder `Alt+P` Modell-Picker
* Die Modell-Einstellung in `/config`
* Das Aktivieren des [Fast-Modus](/docs/de/fast-mode), wenn das das Modell der Sitzung ändert
* Eine `set_model` Anfrage oder eine Modell-Änderung in einer `apply_flag_settings` Anfrage von einem [Agent SDK](/docs/de/agent-sdk/typescript#query-object) Host oder [Remote Control](/docs/de/remote-control)

Claude Code führt PreModelSwitch-Hooks nicht für Wechsel aus, die es selbst macht, wie ein [automatisches Modell-Fallback](/docs/de/model-config#automatic-model-fallback) oder das Wiederherstellen des Modells, wenn Sie eine Sitzung fortsetzen. Diese Änderungen erreichen [PostModelSwitch](#postmodelswitch) nur.

Claude Code vergleicht den Matcher gegen den kanonischen Namen des Modells, zu dem die Sitzung wechselt, und ignoriert jeden `[1m]` Suffix. Ein Alias wie `opus`, eine datierte Modell-ID und eine Provider-spezifische ID wie eine Amazon Bedrock Modell-ID gleichen alle den einen kanonischen Namen ab, den sie auflösen, daher deckt `claude-opus-5` jede Schreibweise von Opus 5 ab.

Wenn Claude Code einen kanonischen Namen für das Ziel nicht bestimmen kann, z. B. eine benutzerdefinierte Modell-ID, die nur Ihr [LLM-Gateway](/docs/de/llm-gateway) kennt, führt es jeden PreModelSwitch-Hook unabhängig vom Matcher aus. Ein Hook, der blockiert, sollte daher `to_model` aus seiner Eingabe überprüfen, anstatt sich nur auf den Matcher zu verlassen.

Schreiben Sie den Matcher als genauen Namen, eine `|`-getrennte Liste wie `claude-opus-4-6|claude-opus-5` oder einen regulären Ausdruck wie `.*opus.*`. Dieses Beispiel verwendet einen genauen Namen-Matcher und überprüft auch `to_model` aus der Hook-Eingabe, daher verweigert es einen Wechsel zu Opus 4.6 durch Beendigung mit Code 2 und lässt jedes andere Ziel durch:

<Tabs>
  <Tab title="macOS/Linux">
    Der Befehl überprüft `to_model` mit `jq`:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "jq -e '.to_model | test(\"opus-4-6\")' > /dev/null && { echo 'Opus 4.6 is retired for this project. Use a newer model.' >&2; exit 2; }; exit 0"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registrieren Sie einen Command-Hook, der ein Skript durch PowerShell ausführt:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-opus-46.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Speichern Sie dieses Skript unter `.claude/hooks/block-opus-46.ps1` in Ihrem Projekt:

    ```powershell theme={null}
    $hookInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    if ($hookInput.to_model -match 'opus-4-6') {
      [Console]::Error.WriteLine('Opus 4.6 is retired for this project. Use a newer model.')
      exit 2
    }
    exit 0
    ```
  </Tab>
</Tabs>

Um zu bestätigen, dass der Hook funktioniert, führen Sie `/model claude-opus-4-6` aus einer Sitzung aus, die ein anderes Modell ausführt. Claude Code behält das aktuelle Modell und meldet, dass ein PreModelSwitch-Hook den Wechsel blockiert hat, mit Ihrer Nachricht als Grund.

<h4 id="premodelswitch-input">
  PreModelSwitch-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten PreModelSwitch-Hooks die Felder in dieser Tabelle. Die letzten fünf beschreiben, was das Erneut-Senden des Gesprächs zum neuen Modell kostet, daher kann ein Hook diese Zahl zeigen, bevor der Wechsel passiert.

| Feld                        | Typ                | Beschreibung                                                                                                                                                                                                                                                                                                           |
| :-------------------------- | :----------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `from_model`                | string             | Modell-ID, zu der der Wechsel wechselt                                                                                                                                                                                                                                                                                 |
| `to_model`                  | string             | Modell-ID, zu der der Wechsel wechselt. Der Matcher vergleicht gegen den kanonischen Namen dieses Modells                                                                                                                                                                                                              |
| `requested_model`           | string oder `null` | Das Modell, das die Anfrage benannt hat: ein Alias wie `opus`, eine vollständige Modell-ID oder `null`, wenn die Anfrage für das Standard-Modell war                                                                                                                                                                   |
| `source`                    | string             | Woher die Anfrage kam: `"command"` für `/model <name>`, die Modell-Einstellung in `/config` oder das Aktivieren des Fast-Modus; `"picker"` für einen Modell-Picker; `"sdk"` für eine `set_model` Anfrage oder eine Modell-Änderung in einer `apply_flag_settings` Anfrage von einem Agent SDK Host oder Remote Control |
| `context_tokens`            | number             | Tokens, die die nächste Anfrage als ihre Eingabeaufforderung erneut sendet: die Eingabe-, Cache-Lese-, Cache-Erstellungs- und Ausgabe-Tokens der letzten Antwort im Hauptgespräch, kombiniert. `0` vor der ersten Antwort                                                                                              |
| `prompt_cache_warm`         | boolean            | Ob der Prompt-Cache des aktuellen Modells wahrscheinlich noch warm ist, was bedeutet, dass der Wechsel ihn aufgibt                                                                                                                                                                                                     |
| `cache_ttl`                 | string             | [Prompt-Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime), die Claude Code für diese Sitzung anfordert: `"5m"` oder `"1h"`                                                                                                                                                                                         |
| `estimated_cache_write_usd` | number             | Geschätzte Kosten in US-Dollar für das Schreiben von `context_tokens` in den Prompt-Cache auf `to_model` bei der `cache_ttl` Rate, ohne die nächste Antwort. Der Server muss möglicherweise nicht den ganzen Kontext erneut zwischenspeichern, daher behandeln Sie es als Schätzung                                    |
| `pricing`                   | string             | Wie Claude Code `estimated_cache_write_usd` bepreist: `"configured"` bei den Raten Ihrer Organisation, wenn sie konfiguriert hat, `"catalog"` bei Listenpreis oder `"default"`, wenn `to_model` keinen bekannten Preis hat und Claude Code einen Standard-Satz annahm                                                  |

Dieses Beispiel zeigt die Eingabe für `/model opus` in einer Sitzung, die Sonnet 5 ausführt:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreModelSwitch",
  "from_model": "claude-sonnet-5",
  "to_model": "claude-opus-5",
  "requested_model": "opus",
  "source": "command",
  "context_tokens": 182340,
  "prompt_cache_warm": true,
  "cache_ttl": "5m",
  "estimated_cache_write_usd": 1.1396,
  "pricing": "catalog"
}
```

<h4 id="premodelswitch-decision-control">
  PreModelSwitch-Entscheidungskontrolle
</h4>

`PreModelSwitch` Hooks können den Wechsel abbrechen, den Benutzer zur Bestätigung auffordern oder ihn fortgesetzt lassen. Exit-Code 2 oder ein Top-Level-`decision: "block"` bricht den Wechsel ab.

Für feinere Kontrolle geben Sie `permissionDecision` und `permissionDecisionReason` in einem `hookSpecificOutput` Objekt zurück, wie auf [PreToolUse](#pretooluse-decision-control). `PreModelSwitch` akzeptiert `"allow"`, `"deny"` und `"ask"`. Es akzeptiert nicht `"defer"`, `updatedInput` oder `additionalContext`. Die Tabelle unten beschreibt beide Felder:

| Feld                       | Beschreibung                                                                                                                                                                                                                            |
| :------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` fährt fort und überspringt die [Bestätigung, die Claude Code zeigt, während der Prompt-Cache warm ist](/docs/de/prompt-caching#switching-models). `"deny"` bricht den Wechsel ab. `"ask"` fordert den Benutzer zur Bestätigung auf |
| `permissionDecisionReason` | Für `"deny"`, dem Benutzer als Grund angezeigt, warum der Wechsel blockiert wurde, oder als Fehler für eine `set_model` Anfrage zurückgegeben. Für `"ask"`, in der Bestätigungs-Aufforderung angezeigt. Ignoriert für `"allow"`         |

Nur `/model` in einer interaktiven Sitzung kann die `"ask"` Aufforderung anzeigen. Auf jeder anderen Oberfläche, einschließlich nicht-interaktivem Modus mit dem `-p` Flag, `/config` und `set_model` Anfragen, behandelt Claude Code `"ask"` als Verweigerung.

Dieses Beispiel fordert den Benutzer zur Bestätigung auf und zitiert die Token-Anzahl aus `context_tokens`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreModelSwitch",
    "permissionDecision": "ask",
    "permissionDecisionReason": "Switching now re-sends about 180k tokens to the new model. Continue?"
  }
}
```

Wenn mehrere PreModelSwitch-Hooks unterschiedliche Entscheidungen zurückgeben, ist die Priorität `deny` > `ask` > `allow`.

Claude Code zeigt dem Benutzer jede `systemMessage`, die Ihr Hook zurückgibt, unabhängig von der Entscheidung, daher kann ein Kosten-Bericht-Hook `{"systemMessage": "..."}` zurückgeben und mit 0 beenden.

Ein PreModelSwitch-Hook, der nicht vor seinem Timeout antwortet, blockiert den Wechsel. Bei [PreToolUse](#timeouts) lässt ein Timeout-Command-Hook den Tool-Aufruf dagegen fortgesetzt. Das Standard-Timeout für dieses Ereignis beträgt 30 Sekunden. `PreModelSwitch` führt nur `command`, `http` und `mcp_tool` Hooks aus, daher gelten die `prompt` und `agent` Standards nicht.

Ein Hook, der mit einem Code anderen als 0 oder 2 beendet und keine JSON-Entscheidung druckt, blockiert nicht: Claude Code zeigt sein stderr und wendet den Wechsel an, wie unter [Andere Exit-Codes](#other-exit-codes) beschrieben.

<h3 id="postmodelswitch">
  PostModelSwitch
</h3>

Wird ausgeführt, nachdem sich das Modell der Sitzung geändert hat. Verwenden Sie es, um Claude modell-spezifische Anleitung zu geben, ohne jede CLAUDE.md zu bearbeiten, z. B. eine organisations-weite Anweisung, die auf bestimmten Modellen gilt.

PostModelSwitch erfordert Claude Code v2.1.251 oder später. Es kann nicht blockieren, da sich das Modell bereits geändert hat. Claude Code führt PostModelSwitch-Hooks nach jeder dieser Änderungen aus:

* Ein Wechsel, den Sie oder ein Client angefordert haben
* Ein [automatisches Modell-Fallback](/docs/de/model-config#automatic-model-fallback), das das Modell der Sitzung ändert
* Eine Einstellung wie [`opusplan`](/docs/de/model-config#opusplan-model-setting), die den Plan-Modus betritt oder verlässt
* Claude Code stellt das Modell wieder her, wenn Sie eine Sitzung fortsetzen

Claude Code führt PostModelSwitch-Hooks nicht aus, wenn ein Modell aus einer [Fallback-Modell-Kette](/docs/de/model-config#fallback-model-chains) einen Zug bedient, da diese Substitution einen Zug dauert und das Modell der Sitzung unverändert lässt.

Der Matcher folgt den gleichen Regeln wie [PreModelSwitch](#premodelswitch): Claude Code vergleicht ihn gegen den kanonischen Namen des Modells, zu dem die Sitzung wechselt.

Dieses Beispiel fügt Anleitung hinzu, wenn sich das Modell der Sitzung zu einem Opus-Modell ändert:

```json theme={null}
{
  "hooks": {
    "PostModelSwitch": [
      {
        "matcher": ".*opus.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'On Opus, delegate implementation work to subagents and keep this conversation for planning and review.'"
          }
        ]
      }
    ]
  }
}
```

Um zu bestätigen, dass der Hook funktioniert, wechseln Sie zu einem Opus-Modell aus einer Sitzung, die ein anderes Modell ausführt, z. B. führen Sie `/model opus` aus einer Sonnet-Sitzung aus, und fragen Sie Claude dann, welche Anleitung es zum aktuellen Modell hat.

<h4 id="postmodelswitch-input">
  PostModelSwitch-Eingabe
</h4>

PostModelSwitch-Hooks erhalten die gleichen Felder wie [PreModelSwitch](#premodelswitch-input), mit `hook_event_name` auf `"PostModelSwitch"` gesetzt und zwei weitere `source` Werte: `"auto"` für ein automatisches Fallback oder eine andere Änderung, die Claude Code selbst gemacht hat, und `"resume"` für das Modell, das wiederhergestellt wird, wenn Sie eine Sitzung fortsetzen.

`requested_model` ist `null`, wenn `source` `"auto"` ist. Wenn `source` `"resume"` ist, ist es die gespeicherte Modell-Einstellung, die Claude Code wiederhergestellt hat.

<h4 id="postmodelswitch-decision-control">
  PostModelSwitch-Entscheidungskontrolle
</h4>

Claude Code nimmt Ihren Hook-[Klartext-stdout](#exit-code-0) bei Beendigung mit 0 oder `additionalContext` aus JSON-Ausgabe und liefert es an Claude mit der nächsten Anfrage nach dem Wechsel. Zusätzlich zu den [JSON-Ausgabefeldern](#json-output), die für alle Hooks verfügbar sind, können Sie zurückgeben:

| Feld                | Beschreibung                                                                                                                             |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String, der zu Claudes Kontext mit der nächsten Anfrage hinzugefügt wird. Siehe [Kontext für Claude hinzufügen](#add-context-for-claude) |

Wenn der Hook nicht innerhalb von fünf Sekunden nach dem Senden der nächsten Eingabeaufforderung fertig ist, sendet Claude Code diese Anfrage ohne die Ausgabe und hängt sie stattdessen an die folgende Anfrage an. Wenn sich das Modell mehrmals ändert, bevor die nächste Anfrage erfolgt, liefert Claude Code nur die Ausgabe für den letzten Wechsel zum Ziel-Modell.

<h3 id="sessionend">
  SessionEnd
</h3>

Wird ausgeführt, wenn eine Claude Code Sitzung endet. Nützlich für Bereinigungsaufgaben, Protokollierung von Sitzungs-Statistiken oder Speicherung des Sitzungs-Zustands. Unterstützt Matcher zum Filtern nach Exit-Grund.

Das `reason` Feld in der Hook-Eingabe gibt an, warum die Sitzung endete:

| Grund                         | Beschreibung                                                                                      |
| :---------------------------- | :------------------------------------------------------------------------------------------------ |
| `clear`                       | Sitzung mit `/clear` Befehl gelöscht                                                              |
| `resume`                      | Sitzung mit interaktivem `/resume` gewechselt                                                     |
| `logout`                      | Benutzer abgemeldet                                                                               |
| `prompt_input_exit`           | Benutzer beendet, während Eingabeaufforderungs-Eingabe sichtbar war                               |
| `other`                       | Andere Exit-Gründe                                                                                |
| `bypass_permissions_disabled` | Entfernt in v2.1.234; Claude Code sendet es nicht. Löschen Sie es aus Ihren `SessionEnd` Matchern |

<h4 id="sessionend-input">
  SessionEnd-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten SessionEnd-Hooks ein `reason` Feld, das angibt, warum die Sitzung endete. Siehe die [Grund-Tabelle](#sessionend) oben für alle Werte.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

SessionEnd-Hooks haben keine Entscheidungskontrolle. Sie können die Sitzungs-Beendigung nicht blockieren, können aber Bereinigungsaufgaben ausführen. Claude Code verwirft ihre [JSON-Ausgabefelder](#json-output), wie `systemMessage`.

SessionEnd-Hooks haben ein Standard-Timeout von 1,5 Sekunden. Es gilt, wenn Sie beenden, `/clear` ausführen oder mit interaktivem `/resume` zu Sitzungen wechseln. Sie können einem Hook auf zwei Wegen mehr Zeit geben:

* **Pro-Hook `timeout`**: Setzen Sie `timeout` in der Konfiguration dieses Hooks. Das Gesamt-Budget steigt automatisch, um das höchste Pro-Hook-`timeout` in Ihren Einstellungsdateien zu entsprechen, bis zu 60 Sekunden. Wenn Sie das Budget auf diese Weise erhöhen, behält ein Hook ohne sein eigenes `timeout` immer noch den Standard. Timeouts, die auf Plugin-bereitgestellten Hooks gesetzt sind, erhöhen das Budget nicht.
* **`CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS`**: Setzen Sie diese Umgebungsvariable in Millisekunden, um das Budget explizit zu überschreiben. Der Wert, den Sie setzen, wird auch zum Timeout für jeden Hook ohne sein eigenes `timeout`.

Dieses Beispiel setzt das Budget auf 5 Sekunden:

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

Vor v2.1.268 erhöhte `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` nur das Gesamt-Budget, und ein Hook ohne sein eigenes `timeout` wurde immer noch nach 1,5 Sekunden abgebrochen.

<h3 id="elicitation">
  Elicitation
</h3>

Wird ausgeführt, wenn ein MCP-Server Benutzereingabe während einer Aufgabe anfordert. Standardmäßig zeigt Claude Code einen interaktiven Dialog für den Benutzer zum Antworten. Hooks können diese Anfrage abfangen und programmatisch antworten, wobei der Dialog vollständig übersprungen wird.

Das Matcher-Feld gleicht gegen den MCP-Server-Namen ab.

<h4 id="elicitation-input">
  Elicitation-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten Elicitation-Hooks `mcp_server_name`, `message` und optionale `mode`, `url`, `elicitation_id` und `requested_schema` Felder.

Für Form-Modus-Elicitierung, der häufigste Fall:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please provide your credentials",
  "mode": "form",
  "requested_schema": {
    "type": "object",
    "properties": {
      "username": { "type": "string", "title": "Username" }
    }
  }
}
```

Für URL-Modus-Elicitierung, verwendet für Browser-basierte Authentifizierung:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Elicitation-Ausgabe
</h4>

Um programmatisch ohne Anzeige des Dialogs zu antworten, geben Sie ein JSON-Objekt mit `hookSpecificOutput` zurück:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Elicitation",
    "action": "accept",
    "content": {
      "username": "alice"
    }
  }
}
```

| Feld      | Werte                         | Beschreibung                                                                      |
| :-------- | :---------------------------- | :-------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Ob die Anfrage akzeptiert, abgelehnt oder abgebrochen werden soll                 |
| `content` | object                        | Formular-Feldwerte zum Einreichen. Wird nur verwendet, wenn `action` `accept` ist |

Exit-Code 2 verweigert die Elicitierung. Claude Code zeigt Ihre stderr-Nachricht nirgendwo an.

Claude Code handelt `hookSpecificOutput` aus der JSON-Ausgabe eines Elicitation-Hooks und verwirft `systemMessage` und `continue`.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Wird ausgeführt, nachdem ein Benutzer auf eine MCP-Elicitierung antwortet. Hooks können die Antwort beobachten, ändern oder blockieren, bevor sie an den MCP-Server zurückgesendet wird.

Das Matcher-Feld gleicht gegen den MCP-Server-Namen ab.

<h4 id="elicitationresult-input">
  ElicitationResult-Eingabe
</h4>

Zusätzlich zu den [allgemeinen Eingabefeldern](#common-input-fields) erhalten ElicitationResult-Hooks `mcp_server_name`, `action` und optionale `mode`, `elicitation_id` und `content` Felder.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  ElicitationResult-Ausgabe
</h4>

Um die Antwort des Benutzers zu überschreiben, geben Sie ein JSON-Objekt mit `hookSpecificOutput` zurück:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Feld      | Werte                         | Beschreibung                                                              |
| :-------- | :---------------------------- | :------------------------------------------------------------------------ |
| `action`  | `accept`, `decline`, `cancel` | Überschreibt die Aktion des Benutzers                                     |
| `content` | object                        | Überschreibt Formular-Feldwerte. Nur sinnvoll, wenn `action` `accept` ist |

Exit-Code 2 blockiert die Antwort und ändert die effektive Aktion zu `decline`. Claude Code zeigt Ihre stderr-Nachricht nirgendwo an.

Claude Code handelt `hookSpecificOutput` aus der JSON-Ausgabe eines ElicitationResult-Hooks und verwirft `systemMessage` und `continue`.

<h2 id="prompt-based-hooks">
  Prompt-basierte Hooks
</h2>

Zusätzlich zu Command-, HTTP- und MCP-Tool-Hooks unterstützt Claude Code Prompt-basierte Hooks (`type: "prompt"`), die ein LLM verwenden, um zu evaluieren, ob eine Aktion zuzulassen oder zu blockieren ist, und Agent-Hooks (`type: "agent"`), die einen agentengesteuerten Verifizierer mit Tool-Zugriff spawnen. Nicht alle Ereignisse unterstützen jeden Hook-Typ.

Ereignisse, die alle fünf Hook-Typen unterstützen (`command`, `http`, `mcp_tool`, `prompt` und `agent`):

* `PermissionDenied`
* `PostToolBatch`
* `PostToolUse`
* `PostToolUseFailure`
* `PreToolUse`
* `Stop`
* `SubagentStop`
* `TaskCompleted`
* `TaskCreated`
* `TeammateIdle`
* `UserPromptExpansion`
* `UserPromptSubmit`

`PermissionRequest` unterstützt `command`, `http`, `mcp_tool` und `prompt` Hooks, aber keine `agent` Hooks. Wenn Sie einen Agent-Hook bei diesem Ereignis konfigurieren, überspringt Claude Code ihn und der Genehmigungsfluss wird unverändert fortgesetzt. Um von einem Hook aus zuzulassen oder zu verweigern, geben Sie das [Entscheidungsobjekt](#permissionrequest-decision-control) von einem Command- oder HTTP-Hook zurück.

Ereignisse, die `command`, `http` und `mcp_tool` Hooks unterstützen, aber nicht `prompt` oder `agent`:

* `ConfigChange`
* `CwdChanged`
* `DirectoryAdded`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `MessageDisplay`
* `Notification`
* `PostCompact`
* `PostModelSwitch`
* `PreCompact`
* `PreModelSwitch`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` und `Setup` unterstützen `command` und `mcp_tool` Hooks, und [MCP-Tool-Hook-Felder](#mcp-tool-hook-fields) beschreibt, wann ihre `mcp_tool` Hooks ausgeführt werden. Sie unterstützen keine `http`, `prompt` oder `agent` Hooks.

<h3 id="how-prompt-based-hooks-work">
  Wie Prompt-basierte Hooks funktionieren
</h3>

Anstatt einen Bash-Befehl auszuführen, Prompt-basierte Hooks:

1. Senden die Hook-Eingabe und Ihren Prompt an ein Claude-Modell, standardmäßig Haiku
2. Das LLM antwortet mit strukturiertem JSON, das eine Entscheidung enthält
3. Claude Code verarbeitet die Entscheidung automatisch

<h3 id="prompt-hook-configuration">
  Prompt-Hook-Konfiguration
</h3>

Setzen Sie `type` auf `"prompt"` und geben Sie eine `prompt`-Zeichenkette anstelle eines `command` an. Verwenden Sie den Platzhalter `$ARGUMENTS`, um die Hook-Eingabedaten in Ihren Prompt-Text einzufügen.

Dieser `Stop`-Hook fragt das LLM, ob Claude stoppen sollte, bevor Claude beendet wird:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Evaluate if Claude should stop: $ARGUMENTS. Check if all tasks are complete."
          }
        ]
      }
    ]
  }
}
```

| Feld              | Erforderlich | Beschreibung                                                                                                                                                                                                                                      |
| :---------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`            | ja           | Muss `"prompt"` sein                                                                                                                                                                                                                              |
| `prompt`          | ja           | Der Prompt-Text zum Senden an das LLM. Verwenden Sie `$ARGUMENTS` als Platzhalter für die Hook-Eingabe JSON. Wenn `$ARGUMENTS` nicht vorhanden ist, wird die Eingabe JSON an den Prompt angehängt                                                 |
| `model`           | nein         | Modell zur Verwendung für die Evaluierung. Standardwert ist ein schnelles Modell                                                                                                                                                                  |
| `timeout`         | nein         | Timeout in Sekunden. Standard: 30                                                                                                                                                                                                                 |
| `continueOnBlock` | nein         | Bei den Ereignissen, auf die es zutrifft, speist `true` einen `ok: false` Grund an Claude zurück und setzt den Turn fort, anstatt ihn zu beenden. Standard: `false`. Siehe [Response-Schema](#response-schema) für ereignisspezifisches Verhalten |

<h3 id="response-schema">
  Response-Schema
</h3>

Das LLM muss mit JSON antworten, das Folgendes enthält:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision",
  "impossible": true | false
}
```

| Feld         | Beschreibung                                                                                                                                                                                                                                                                  |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ok`         | `true` erlaubt die Aktion. Bei `false` siehe das ereignisspezifische Verhalten unten                                                                                                                                                                                          |
| `reason`     | Erforderlich, wenn `ok` `false` ist                                                                                                                                                                                                                                           |
| `impossible` | Optional. Das Modell gibt es mit `ok: false` zurück, wenn es beurteilt, dass die Bedingung niemals erfüllt werden kann. Bei `Stop` und `SubagentStop` lässt Claude Code dann den Turn enden, anstatt den Grund zurückzugeben. Agent-Hooks und andere Ereignisse ignorieren es |

Was bei `ok: false` passiert, hängt vom Ereignis ab:

* `Stop` und `SubagentStop`: der Grund wird an Claude als nächste Anweisung zurückgegeben und der Turn wird fortgesetzt, es sei denn, die Antwort setzt auch `impossible: true`, in welchem Fall Claude Code den Stop zulässt und der Turn endet
* `PreToolUse`: der Tool-Aufruf wird verweigert; standardmäßig endet der Turn und der Verweigerungsgrund wird im Chat als Warnzeile angezeigt. Setzen Sie `continueOnBlock: true`, um den Grund stattdessen an Claude als Tool-Fehler zurückzugeben, damit es anpassen und fortfahren kann, äquivalent zu einem Command-Hook mit `permissionDecision: "deny"`. Vor v2.1.210 wurde der Verweigerungsgrund an Claude als Tool-Fehler zurückgegeben und der Turn wurde fortgesetzt
* `PostToolUse`: standardmäßig endet der Turn und der Grund wird im Chat als Warnzeile angezeigt. Setzen Sie `continueOnBlock: true`, um den Grund an Claude zurückzugeben und den Turn stattdessen fortzusetzen
* `PostToolBatch`, `UserPromptSubmit` und `UserPromptExpansion`: der Turn endet und der Grund wird als Warnzeile angezeigt. Diese Ereignisse beenden den Turn bei `decision: "block"` unabhängig von `continue`
* `PostToolUseFailure` und `TaskCreated`: der Grund wird an Claude als Tool-Fehler zurückgegeben und der Turn wird fortgesetzt, unabhängig von `continueOnBlock`
* `TaskCompleted`: wenn es ausgelöst wird, weil eine Aufgabe während eines Turns als abgeschlossen markiert wird, wird der Grund an Claude als Tool-Fehler zurückgegeben und der Turn wird fortgesetzt, unabhängig von `continueOnBlock`. Wenn es ausgelöst wird, weil ein Teammate stoppt, verhält es sich wie `TeammateIdle` und stoppt den Teammate standardmäßig
* `TeammateIdle`: standardmäßig stoppt der Teammate und der Grund wird als Warnzeile angezeigt. Setzen Sie `continueOnBlock: true`, um den Grund an den Teammate zurückzugeben und ihn stattdessen weiterarbeiten zu lassen
* `PermissionRequest`: `ok: false` hat keine Auswirkung. Um eine Genehmigung von einem Hook zu verweigern, verwenden Sie einen [Command-Hook](#command-hook-fields) mit `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` hat keine Auswirkung, da die Verweigerung bereits erfolgt ist. Die einzige Ausgabe, die dieses Ereignis liest, ist `hookSpecificOutput.retry`, die Prompt- und Agent-Hooks nicht setzen können. Sie werden bei diesem Ereignis ausgeführt, aber ihre Ausgabe wird verworfen. Verwenden Sie einen [Command-Hook](#command-hook-fields), um `retry` zurückzugeben

Wenn Sie eine feinere Kontrolle bei einem Ereignis benötigen, verwenden Sie einen [Command-Hook](#command-hook-fields) mit den ereignisspezifischen Feldern, die in [Entscheidungskontrolle](#decision-control) beschrieben sind.

<h3 id="check-multiple-conditions-before-stopping">
  Mehrere Bedingungen vor dem Stoppen überprüfen
</h3>

Dieser `Stop`-Hook verwendet einen detaillierten Prompt, um drei Bedingungen zu überprüfen, bevor Claude stoppen darf. `SubagentStop`-Hooks verwenden das gleiche Format, um zu evaluieren, ob ein [Subagent](/docs/de/sub-agents) stoppen sollte. Wenn das Modell `"ok": false` zurückgibt, weil die Bedingung noch nicht erfüllt ist, setzt Claude die Arbeit mit dem bereitgestellten Grund als nächste Anweisung fort:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "You are evaluating whether Claude should stop working. Context: $ARGUMENTS\n\nAnalyze the conversation and determine if:\n1. All user-requested tasks are complete\n2. Any errors need to be addressed\n3. Follow-up work is needed\n\nRespond with JSON: {\"ok\": true} to allow stopping, or {\"ok\": false, \"reason\": \"your explanation\"} to continue working.",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<h2 id="agent-based-hooks">
  Agent-basierte Hooks
</h2>

<Warning>
  Agent-Hooks sind experimentell. Das Verhalten und die Konfiguration können sich in zukünftigen Versionen ändern. Für Produktions-Workflows bevorzugen Sie [Command Hooks](#command-hook-fields).
</Warning>

Agent-basierte Hooks (`type: "agent"`) sind wie Prompt-basierte Hooks, aber mit Multi-Turn-Tool-Zugriff. Anstelle eines einzelnen LLM-Aufrufs spawnt ein Agent-Hook einen Subagenten, der Dateien lesen, Code durchsuchen und die Codebasis überprüfen kann, um Bedingungen zu überprüfen. Agent-Hooks unterstützen die gleichen Ereignisse wie [Prompt-basierte Hooks](#prompt-based-hooks), mit Ausnahme von `PermissionRequest`.

<h3 id="how-agent-hooks-work">
  Wie Agent-Hooks funktionieren
</h3>

Wenn ein Agent-Hook ausgelöst wird:

1. Claude Code spawnt einen Subagenten mit Ihrem Prompt und der Hook-Eingabe JSON
2. Der Subagent kann Tools wie Read, Grep und Glob verwenden, um zu untersuchen
3. Nach bis zu 50 Turns gibt der Subagent eine strukturierte `{ "ok": true/false }`-Entscheidung zurück
4. Claude Code erlaubt die Aktion, wenn `ok` `true` ist. Wenn `ok` `false` ist, verarbeitet Claude Code die Blockierung auf die gleiche Weise wie ein Prompt-Hook mit `continueOnBlock: true` bei diesem Ereignis, wie unter [Response-Schema](#response-schema) aufgelistet

Agent-Hooks sind nützlich, wenn die Überprüfung das Überprüfen tatsächlicher Dateien oder Test-Ausgabe erfordert, nicht nur die Evaluierung der Hook-Eingabedaten allein.

<h3 id="agent-hook-configuration">
  Agent-Hook-Konfiguration
</h3>

Setzen Sie `type` auf `"agent"` und geben Sie eine `prompt`-Zeichenkette an, wobei Sie `$ARGUMENTS` als Platzhalter für die Hook-Eingabe JSON verwenden. Die Konfigurationsfelder sind die gleichen wie [Prompt-Hooks](#prompt-hook-configuration), mit der Ausnahme, dass Agent-Hooks ein längeres Standard-Timeout von 60 Sekunden haben und kein `continueOnBlock`-Feld haben.

Das Response-Schema ist `{ "ok": true }` zum Zulassen oder `{ "ok": false, "reason": "..." }` zum Blockieren. Bei `ok: false` verarbeitet Claude Code einen Agent-Hook auf die gleiche Weise wie einen [Prompt-Hook mit `continueOnBlock: true`](#response-schema) bei demselben Ereignis; Agent-Hooks haben kein `continueOnBlock`-Feld und unterstützen nicht das Prompt-Hook-Feld `impossible`.

Dieser `Stop`-Hook überprüft, dass alle Unit-Tests bestanden sind, bevor Claude fertig ist:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

<h2 id="run-hooks-in-the-background">
  Hooks im Hintergrund ausführen
</h2>

Standardmäßig blockieren Hooks die Ausführung von Claude, bis sie abgeschlossen sind. Für lang laufende Aufgaben wie Bereitstellungen, Test-Suites oder externe API-Aufrufe setzen Sie `"async": true`, um den Hook im Hintergrund auszuführen, während Claude weiterarbeitet. Asynchrone Hooks können nicht blockieren oder das Verhalten von Claude steuern: Response-Felder wie `decision`, `permissionDecision` und `continue` haben keine Auswirkung, da die Aktion, die sie steuern würden, bereits abgeschlossen ist.

<h3 id="configure-an-async-hook">
  Konfigurieren Sie einen asynchronen Hook
</h3>

Fügen Sie `"async": true` zur Konfiguration eines Command-Hooks hinzu, um ihn im Hintergrund auszuführen, ohne Claude zu blockieren. Dieses Feld ist nur auf `type: "command"`-Hooks verfügbar.

Dieser Hook führt ein Test-Skript nach jedem `Write`-Tool-Aufruf aus. Claude arbeitet sofort weiter, während `run-tests.sh` ausgeführt wird. Wenn das Skript fertig ist, wird seine Ausgabe beim nächsten Gesprächsturn geliefert:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/run-tests.sh",
            "async": true
          }
        ]
      }
    ]
  }
}
```

Sobald ein asynchroner Hook im Hintergrund ausgeführt wird, erzwingt Claude Code kein `timeout` darauf. Claude Code erzwingt immer noch `timeout` auf einem Hook, den Sie mit `asyncRewake` ausführen.

Claude Code liefert die Ergebnisse eines asynchronen Hooks nur während der Sitzung:

* Im [nicht-interaktiven Modus](/docs/de/headless) mit dem Flag `-p` beendet Claude Code jeden noch laufenden asynchronen Hook beim Herunterfahren und finalisiert ihn mit dem Ergebnis `cancelled`
* Wenn die Arbeit Ihres Hooks eine `claude -p`-Sitzung überdauern muss, starten Sie einen vollständig abgelösten Prozess davon

<h3 id="how-async-hooks-execute">
  Wie asynchrone Hooks ausgeführt werden
</h3>

Wenn ein asynchroner Hook ausgelöst wird, startet Claude Code den Hook-Prozess und setzt sofort fort, ohne auf den Abschluss zu warten. Der Hook erhält die gleiche JSON-Eingabe über stdin wie ein synchroner Hook.

Nachdem der Hintergrund-Prozess beendet ist, liefert Claude Code die Felder `additionalContext` und `systemMessage` aus der JSON-Response des Hooks Claude beim nächsten Gesprächsturn. Im Gegensatz zu einem synchronen Hook's `systemMessage` wird keines dieser Felder Ihnen angezeigt.

Claude Code validiert diese JSON-Response gegen das gleiche [Ausgabeschema](#json-output) wie synchrone Hooks und verwirft jedes Feld, dessen Wert den falschen Typ hat, wie z. B. eine `systemMessage`, die keine Zeichenkette ist, anstatt es zu liefern. Führen Sie mit `--debug` aus, um eine Warnung zu sehen, die jedes verworfene Feld benennt. Vor v2.1.202 konnte fehlerhafte JSON-Ausgabe von einem asynchronen Hook die Sitzung zum Absturz bringen, und der Absturz trat jedes Mal auf, wenn die Sitzung fortgesetzt wurde.

Benachrichtigungen über den Abschluss asynchroner Hooks werden standardmäßig unterdrückt. Um sie zu sehen, aktivieren Sie den ausführlichen Modus mit `Ctrl+O` oder starten Sie Claude Code mit `--verbose`.

<h3 id="run-tests-after-file-changes">
  Tests nach Dateiänderungen ausführen
</h3>

Dieser Hook startet eine Test-Suite im Hintergrund, wenn Claude eine Datei schreibt, und meldet die Ergebnisse Claude, wenn die Tests fertig sind. Speichern Sie dieses Skript unter `.claude/hooks/run-tests-async.sh` in Ihrem Projekt und machen Sie es mit `chmod +x` ausführbar:

```bash theme={null}
#!/bin/bash
# run-tests-async.sh

# Hook-Eingabe von stdin lesen
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Tests nur für Quelldateien ausführen
if [[ "$FILE_PATH" != *.ts && "$FILE_PATH" != *.js ]]; then
  exit 0
fi

# Tests ausführen und Ergebnisse über additionalContext an Claude melden
RESULT=$(npm test 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  MSG="Tests passed after editing $FILE_PATH"
else
  MSG="Tests failed after editing $FILE_PATH: $RESULT"
fi
jq -nc --arg msg "$MSG" '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $msg}}'
```

Fügen Sie dann diese Konfiguration zu `.claude/settings.json` im Projekt-Root hinzu. Das Flag `async: true` ermöglicht es Claude, weiterarbeiten zu können, während Tests ausgeführt werden:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/run-tests-async.sh",
            "args": [],
            "async": true
          }
        ]
      }
    ]
  }
}
```

<h3 id="limitations">
  Einschränkungen
</h3>

Asynchrone Hooks haben zusätzliche Einschränkungen im Vergleich zu synchronen Hooks:

* Hook-Ausgabe wird beim nächsten Gesprächsturn geliefert. Wenn die Sitzung untätig ist, wartet die Response, bis die nächste Benutzerinteraktion erfolgt. Ausnahme: Ein `asyncRewake`-Hook, der mit Code 2 beendet wird, weckt Claude sofort auf, auch wenn die Sitzung untätig ist.
* Jede Ausführung erstellt einen separaten Hintergrund-Prozess. Es gibt keine Deduplizierung über mehrere Auslösungen des gleichen asynchronen Hooks.

<h2 id="security-considerations">
  Sicherheitsüberlegungen
</h2>

<h3 id="disclaimer">
  Haftungsausschluss
</h3>

<Warning>
  Command-Hooks führen Shell-Befehle mit Ihren vollständigen Benutzerberechtigungen aus. Sie können alle Dateien ändern, löschen oder zugreifen, auf die Ihr Benutzerkonto zugreifen kann. Überprüfen und testen Sie alle Hook-Befehle, bevor Sie sie zu Ihrer Konfiguration hinzufügen.
</Warning>

<h3 id="workspace-trust">
  Workspace-Vertrauen
</h3>

Claude Code überprüft das Workspace-Vertrauen, bevor es einen Hook aus einer Einstellungsdatei ausführt. Was als vertrauenswürdig gilt, hängt vom Sitzungstyp ab:

* **Interaktive Sitzung**: Claude Code hält Hooks aus jeder Einstellungsdatei zurück, einschließlich Ihrer eigenen `~/.claude/settings.json`, bis Sie den [Workspace-Vertrauensdialog](/docs/de/permissions#project-allow-rules-and-workspace-trust) für den Ordner oder für ein übergeordnetes Verzeichnis, dessen Vertrauen sich darauf erstreckt, akzeptieren
* **`-p` oder SDK-Sitzung**: Claude Code zeigt den Dialog nie an und behandelt den Ordner als vertrauenswürdig, sodass Hooks, die in der `.claude/settings.json` eines Repositorys committed sind, in einem Ordner ausgeführt werden, dem Sie nie vertraut haben

Bevor Sie `claude -p` über ein Repository ausführen, das Sie nicht geschrieben haben, überprüfen Sie seine `.claude/`-Einstellungsdateien, starten Sie mit [`--bare`](/docs/de/headless#start-faster-with-bare-mode), oder [deaktivieren Sie Hooks für diesen Durchlauf](#disable-or-remove-hooks) mit `--settings '{"disableAllHooks": true}'`. Frontmatter-Hooks in einem Projekt-Subagent folgen einer strengeren Regel als Einstellungsdatei-Hooks. [Was vor dem Vertrauen eines Ordners ausgeführt wird](/docs/de/permissions#what-runs-before-you-trust-a-folder) listet jede Art von Repository-Inhalt nach Sitzungstyp auf.

<h3 id="security-best-practices">
  Best Practices für Sicherheit
</h3>

Beachten Sie diese Praktiken beim Schreiben von Hooks:

* **Validieren und bereinigen Sie Eingaben**: Vertrauen Sie niemals blind auf Eingabedaten
* **Zitieren Sie immer Shell-Variablen**: Verwenden Sie `"$VAR"` nicht `$VAR`
* **Blockieren Sie Pfad-Traversal**: Prüfen Sie auf `..` in Dateipfaden
* **Verwenden Sie absolute Pfade**: Geben Sie vollständige Pfade für Skripte an. In der Exec-Form verwenden Sie `${CLAUDE_PROJECT_DIR}` und der Pfad benötigt keine Anführungszeichen. In der Shell-Form wickeln Sie ihn in doppelte Anführungszeichen ein
* **Überspringen Sie sensible Dateien**: Vermeiden Sie `.env`, `.git/`, Schlüssel, etc.

<h2 id="windows-powershell-tool">
  Windows PowerShell-Tool
</h2>

Unter Windows können Sie einzelne Hooks in PowerShell ausführen, indem Sie `"shell": "powershell"` auf einem Command-Hook setzen. Claude Code erkennt automatisch `pwsh.exe`, die PowerShell 7 und später ausführbare Datei, und fällt auf `powershell.exe` für Windows PowerShell 5.1 zurück.

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "shell": "powershell",
            "command": "Write-Host 'File written'"
          }
        ]
      }
    ]
  }
}
```

Um auf das Projektverzeichnis aus einem PowerShell-Shell-Form-Befehl zu verweisen, schreiben Sie `${CLAUDE_PROJECT_DIR}` oder `$env:CLAUDE_PROJECT_DIR`. Ab v2.1.198 schreibt Claude Code die Platzhalter `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}` und `${CLAUDE_PLUGIN_DATA}` in einem PowerShell-Shell-Form-Befehl in die PowerShell-Form `${env:NAME}` um, unabhängig davon, ob der Hook in `settings.json`, einem Plugin oder einer Skill definiert ist. PowerShell löst dann den Wert aus der exportierten Umgebung nach dem Parsing auf, daher funktioniert der Platzhalter in doppelt angeführten Zeichenketten, aber nicht in einfach angeführten Zeichenketten, wo PowerShell niemals Variablen erweitert.

Vor v2.1.198 galt dieses Umschreiben nur für Plugin-Hooks. In früheren Versionen benötigt ein `settings.json`-Hook die Form `$env:` oder [Exec-Form](#exec-form-and-shell-form), wobei `${CLAUDE_PROJECT_DIR}` in jedem `args`-Element ersetzt wird, unabhängig davon, wo der Hook definiert ist.

Schreiben Sie nicht die bloße Schreibweise `$CLAUDE_PROJECT_DIR` in einem PowerShell-Hook. PowerShell analysiert sie als undefinierte lokale Variable und löst sie zu `$null` auf, was den Skriptpfad ohne sein Projektverzeichnis-Präfix hinterlässt. Claude Code schreibt diese Form nicht um; stattdessen protokolliert es eine Warnung im [Debug-Log](#debug-hooks).

Das folgende Beispiel zeigt einen `settings.json`-Hook, der ein Projektskript mit der Form `$env:` ausführt, die auf jeder Version funktioniert:

```json theme={null}
{
  "type": "command",
  "shell": "powershell",
  "command": "& \"$env:CLAUDE_PROJECT_DIR\\.claude\\hooks\\check.ps1\""
}
```

<h2 id="debug-hooks">
  Debug-Hooks
</h2>

Hook-Ausführungsdetails werden in die Debug-Log-Datei geschrieben. Starten Sie Claude Code mit `claude --debug-file <path>`, um das Log in einen bekannten Speicherort zu schreiben, oder führen Sie `claude --debug` aus und lesen Sie das Log unter `~/.claude/debug/<session-id>.txt`. Das Flag `--debug` gibt nicht auf dem Terminal aus.

Beispielsweise erzeugt ein `PostToolUse`-Hook auf `Write`, dessen Befehl `hook-ran` ausgibt, Einträge wie:

```text theme={null}
2026-07-19T02:03:24.382Z [DEBUG] Hook output does not start with {, treating as plain text
2026-07-19T02:03:24.382Z [DEBUG] "Hook PostToolUse:Write (PostToolUse) success:\nhook-ran"
```

Für granularere Hook-Matching-Details setzen Sie `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose`, um zusätzliche Log-Zeilen wie Hook-Matcher-Zählungen und Query-Matching zu sehen.

Zur Fehlerbehebung häufiger Probleme wie Hooks, die nicht ausgelöst werden, Stop-Hooks, die weiterhin blockieren, oder Konfigurationsfehler, siehe [Einschränkungen und Fehlerbehebung](/docs/de/hooks-guide#limitations-and-troubleshooting) in der Anleitung. Für eine umfassendere diagnostische Anleitung, die `/context`, `/doctor` und Einstellungspriorität abdeckt, siehe [Debug your config](/docs/de/debug-your-config).
