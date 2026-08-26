> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatisieren Sie Aktionen mit Hooks

> Führen Sie Shell-Befehle automatisch aus, wenn Claude Code Dateien bearbeitet, Aufgaben abschließt oder Eingaben benötigt. Formatieren Sie Code, senden Sie Benachrichtigungen, validieren Sie Befehle und erzwingen Sie Projektregeln.

Hooks sind benutzerdefinierte Shell-Befehle, die an bestimmten Punkten im Lebenszyklus von Claude Code ausgeführt werden. Sie bieten deterministische Kontrolle über das Verhalten von Claude Code und stellen sicher, dass bestimmte Aktionen immer stattfinden, anstatt sich darauf zu verlassen, dass das LLM sich dafür entscheidet, sie auszuführen. Verwenden Sie Hooks, um Projektregeln durchzusetzen, sich wiederholende Aufgaben zu automatisieren und Claude Code mit Ihren vorhandenen Tools zu integrieren.

Für Entscheidungen, die Urteilsvermögen erfordern, anstatt deterministischer Regeln, können Sie auch [Prompt-basierte Hooks](#prompt-based-hooks) oder [Agent-basierte Hooks](#agent-based-hooks) verwenden, die ein Claude-Modell zur Bewertung von Bedingungen nutzen.

Für andere Möglichkeiten, Claude Code zu erweitern, siehe [skills](/docs/de/skills) zum Bereitstellen zusätzlicher Anweisungen und ausführbarer Befehle, [subagents](/docs/de/sub-agents) zum Ausführen von Aufgaben in isolierten Kontexten und [plugins](/docs/de/plugins) zum Verpacken von Erweiterungen, die über Projekte hinweg freigegeben werden können.

<Tip>
  Dieser Leitfaden behandelt häufige Anwendungsfälle und wie Sie anfangen. Für vollständige Event-Schemas, JSON-Ein-/Ausgabeformate und erweiterte Funktionen wie asynchrone Hooks und MCP-Tool-Hooks siehe die [Hooks-Referenz](/docs/de/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Richten Sie Ihren ersten Hook ein
</h2>

Um einen Hook zu erstellen, fügen Sie einen `hooks`-Block zu einer [Einstellungsdatei](#configure-hook-location) hinzu. Diese Anleitung erstellt einen Desktop-Benachrichtigungs-Hook, damit Sie benachrichtigt werden, wenn Claude auf Ihre Eingabe wartet, anstatt das Terminal zu beobachten.

<Steps>
  <Step title="Fügen Sie den Hook zu Ihren Einstellungen hinzu">
    Öffnen Sie `~/.claude/settings.json` und fügen Sie einen `Notification`-Hook hinzu. Das Beispiel unten verwendet `osascript` für macOS; siehe [Benachrichtigung erhalten, wenn Claude Eingaben benötigt](#get-notified-when-claude-needs-input) für Linux- und Windows-Befehle.

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    Wenn Ihre Einstellungsdatei bereits einen `hooks`-Schlüssel hat, fügen Sie `Notification` als Geschwister der vorhandenen Event-Schlüssel hinzu, anstatt das ganze Objekt zu ersetzen. Jeder Event-Name ist ein Schlüssel innerhalb des einzelnen `hooks`-Objekts:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    Sie können auch Claude bitten, den Hook für Sie zu schreiben, indem Sie beschreiben, was Sie in der CLI möchten.
  </Step>

  <Step title="Überprüfen Sie die Konfiguration">
    Geben Sie `/hooks` ein, um den Hooks-Browser zu öffnen. Sie sehen eine Liste aller verfügbaren Hook-Events mit einer Anzahl neben jedem Event, das Hooks konfiguriert hat. Wählen Sie `Notification` aus, um zu bestätigen, dass Ihr neuer Hook in der Liste angezeigt wird. Wenn Sie den Hook auswählen, werden seine Details angezeigt: das Event, der Matcher, der Typ, die Quelldatei und der Befehl.
  </Step>

  <Step title="Testen Sie den Hook">
    Drücken Sie `Esc`, um zur CLI zurückzukehren. Bitten Sie Claude, etwas zu tun, das eine Berechtigung erfordert, und wechseln Sie dann weg vom Terminal. Sie sollten eine Desktop-Benachrichtigung erhalten.
  </Step>
</Steps>

<Tip>
  Das Menü `/hooks` ist schreibgeschützt. Um Hooks hinzuzufügen, zu ändern oder zu entfernen, bearbeiten Sie Ihre Einstellungs-JSON direkt oder bitten Sie Claude, die Änderung vorzunehmen.
</Tip>

<h2 id="what-you-can-automate">
  Was Sie automatisieren können
</h2>

Hooks ermöglichen es Ihnen, Code an Schlüsselpunkten im Lebenszyklus von Claude Code auszuführen: Dateien nach Bearbeitungen formatieren, Befehle vor der Ausführung blockieren, Benachrichtigungen senden, wenn Claude Eingaben benötigt, Kontext beim Sitzungsstart injizieren und vieles mehr. Für die vollständige Liste der Hook-Events siehe die [Hooks-Referenz](/docs/de/hooks#hook-lifecycle).

Jedes Beispiel enthält einen einsatzbereiten Konfigurationsblock, den Sie einer [Einstellungsdatei](#configure-hook-location) hinzufügen.

Ein Produktionsbeispiel von Hooks, die eine separate Modellüberprüfung ausführen und Erkenntnisse zurück in die Sitzung einspeisen, finden Sie unter [wie das `security-guidance`-Plugin mit Claude Code integriert wird](/docs/de/security-guidance#how-the-plugin-integrates-with-claude-code).

<h3 id="get-notified-when-claude-needs-input">
  Benachrichtigung erhalten, wenn Claude Eingaben benötigt
</h3>

Erhalten Sie eine Desktop-Benachrichtigung, wenn Claude die Arbeit beendet und Ihre Eingabe benötigt, damit Sie zu anderen Aufgaben wechseln können, ohne das Terminal zu überprüfen.

Dieser Hook verwendet das `Notification`-Event, das ausgelöst wird, wenn Claude auf Eingaben oder Berechtigungen wartet. Jede Registerkarte unten verwendet den nativen Benachrichtigungsbefehl der Plattform. Fügen Sie dies zu `~/.claude/settings.json` hinzu:

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="Wenn keine Benachrichtigung angezeigt wird">
      `osascript` leitet Benachrichtigungen über die integrierte Script Editor-App weiter. Wenn Script Editor keine Benachrichtigungsberechtigung hat, schlägt der Befehl stillschweigend fehl, und macOS fordert Sie nicht auf, sie zu gewähren. Führen Sie dies einmal im Terminal aus, um Script Editor in Ihren Benachrichtigungseinstellungen angezeigt zu bekommen:

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      Es wird noch nichts angezeigt. Öffnen Sie **Systemeinstellungen > Benachrichtigungen**, suchen Sie **Script Editor** in der Liste und aktivieren Sie **Benachrichtigungen zulassen**. Führen Sie den Befehl erneut aus, um zu bestätigen, dass die Test-Benachrichtigung angezeigt wird.
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Der leere `matcher` wird bei allen Benachrichtigungstypen ausgelöst. Um nur bei bestimmten Events ausgelöst zu werden, setzen Sie ihn auf einen dieser Werte:

| Matcher                | Wird ausgelöst, wenn                                                                                                           |
| :--------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`    | Claude benötigt Ihre Genehmigung für einen Tool-Einsatz                                                                        |
| `idle_prompt`          | Claude ist fertig und wartet auf Ihre nächste Eingabe                                                                          |
| `auth_success`         | Authentifizierung ist abgeschlossen                                                                                            |
| `elicitation_dialog`   | Ein MCP-Server öffnet ein Elicitation-Formular                                                                                 |
| `elicitation_complete` | Ein MCP-Elicitation-Formular wird eingereicht oder verworfen                                                                   |
| `elicitation_response` | Eine MCP-Elicitation-Antwort wird an den Server zurückgesendet                                                                 |
| `agent_needs_input`    | Eine Hintergrund-Sitzung wartet auf Ihre Eingabe. Wird nur ausgelöst, während [Agent-Ansicht](/docs/de/agent-view) offen ist        |
| `agent_completed`      | Eine Hintergrund-Sitzung wird beendet oder schlägt fehl. Wird nur ausgelöst, während [Agent-Ansicht](/docs/de/agent-view) offen ist |

Die Matcher `agent_needs_input` und `agent_completed` erfordern Claude Code v2.1.198 oder später.

Geben Sie `/hooks` ein und wählen Sie `Notification` aus, um zu bestätigen, dass der Hook registriert ist. Für das vollständige Event-Schema siehe die [Notification-Referenz](/docs/de/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Code nach Bearbeitungen automatisch formatieren
</h3>

Führen Sie [Prettier](https://prettier.io/) automatisch auf jeder Datei aus, die Claude bearbeitet, damit die Formatierung konsistent bleibt, ohne manuelle Eingriffe.

Dieser Hook verwendet das `PostToolUse`-Event mit einem `Edit|Write`-Matcher, sodass er nur nach Datei-Bearbeitungs-Tools ausgeführt wird. Der Befehl extrahiert den bearbeiteten Dateipfad mit [`jq`](https://jqlang.github.io/jq/) und übergibt ihn an Prettier. Fügen Sie dies zu `.claude/settings.json` in Ihrem Projektverzeichnis hinzu:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

Auf Claude Code v2.1.191 oder später können Sie den Matcher auch als `Edit,Write` schreiben, da `|` und `,` auf diesen Versionen austauschbare Listentrennzeichen für Tool-Name-Matcher sind.

<Note>
  Die Bash-Beispiele auf dieser Seite verwenden `jq` zum Parsen von JSON. Installieren Sie es mit `brew install jq` auf macOS, `apt-get install jq` auf Debian und Ubuntu, oder siehe [`jq`-Downloads](https://jqlang.github.io/jq/download/).
</Note>

<h3 id="block-edits-to-protected-files">
  Bearbeitungen geschützter Dateien blockieren
</h3>

Verhindern Sie, dass Claude sensible Dateien wie `.env`, `package-lock.json` oder alles in `.git/` ändert. Claude erhält Feedback, das erklärt, warum die Bearbeitung blockiert wurde, sodass es seinen Ansatz anpassen kann.

Dieses Beispiel verwendet eine separate Skriptdatei, die der Hook aufruft. Das Skript überprüft den Zieldateipfad gegen eine Liste geschützter Muster und beendet sich mit Code 2, um die Bearbeitung zu blockieren.

<Steps>
  <Step title="Erstellen Sie das Hook-Skript">
    Speichern Sie dies unter `.claude/hooks/protect-files.sh`:

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="Machen Sie das Skript ausführbar auf macOS und Linux">
    Hook-Skripte müssen ausführbar sein, damit Claude Code sie ausführen kann:

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="Registrieren Sie den Hook">
    Fügen Sie einen `PreToolUse`-Hook zu `.claude/settings.json` hinzu, der das Skript vor jedem `Edit`- oder `Write`-Tool-Aufruf ausführt:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Kontext nach Komprimierung erneut injizieren
</h3>

Wenn Claudes Kontextfenster voll wird, fasst die Komprimierung das Gespräch zusammen, um Platz freizugeben. Dies kann wichtige Details verlieren. Verwenden Sie einen `SessionStart`-Hook mit einem `compact`-Matcher, um nach jeder Komprimierung kritischen Kontext erneut zu injizieren.

Jeder Text, den Ihr Befehl auf stdout schreibt, wird zu Claudes Kontext hinzugefügt. Dieses Beispiel erinnert Claude an Projektkonventionen und aktuelle Arbeiten. Fügen Sie dies zu `.claude/settings.json` in Ihrem Projektverzeichnis hinzu:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

Sie können das `echo` durch jeden Befehl ersetzen, der dynamische Ausgabe erzeugt, wie `git log --oneline -5`, um aktuelle Commits anzuzeigen. Zum Injizieren von Kontext bei jedem Sitzungsstart sollten Sie stattdessen [CLAUDE.md](/docs/de/memory) verwenden. Für Umgebungsvariablen siehe [`CLAUDE_ENV_FILE`](/docs/de/hooks#persist-environment-variables) in der Referenz.

<h3 id="audit-configuration-changes">
  Konfigurationsänderungen prüfen
</h3>

Verfolgen Sie, wenn sich Einstellungs- oder Skills-Dateien während einer Sitzung ändern. Das `ConfigChange`-Event wird ausgelöst, wenn ein externer Prozess oder Editor eine Konfigurationsdatei ändert, sodass Sie Änderungen für Compliance protokollieren oder nicht autorisierte Änderungen blockieren können.

Dieses Beispiel hängt jede Änderung an ein Audit-Protokoll an. Fügen Sie dies zu `~/.claude/settings.json` hinzu:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

Der Matcher filtert nach Konfigurationstyp: `user_settings`, `project_settings`, `local_settings`, `policy_settings` oder `skills`. Um eine Änderung zu blockieren, beenden Sie mit Code 2 oder geben Sie `{"decision": "block"}` zurück. Siehe die [ConfigChange-Referenz](/docs/de/hooks#configchange) für das vollständige Eingabe-Schema.

<h3 id="reload-environment-when-directory-or-files-change">
  Umgebung neu laden, wenn sich Verzeichnis oder Dateien ändern
</h3>

Einige Projekte setzen unterschiedliche Umgebungsvariablen je nachdem, in welchem Verzeichnis Sie sich befinden. Tools wie [direnv](https://direnv.net/) tun dies automatisch in Ihrer Shell, aber Claudes Bash-Tool übernimmt diese Änderungen nicht automatisch.

Das Pairing eines `SessionStart`-Hooks mit einem `CwdChanged`-Hook behebt dies. `SessionStart` lädt die Variablen für das Verzeichnis, in dem Sie starten, und `CwdChanged` lädt sie jedes Mal neu, wenn Claude das Verzeichnis wechselt. Beide schreiben in `CLAUDE_ENV_FILE`, die Claude Code vor jedem Bash-Befehl als Skript-Präambel ausführt. Fügen Sie dies zu `~/.claude/settings.json` hinzu:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Führen Sie `direnv allow` einmal in jedem Verzeichnis aus, das eine `.envrc` hat, damit direnv berechtigt ist, sie zu laden. Wenn Sie devbox oder nix statt direnv verwenden, funktioniert das gleiche Muster mit `devbox shellenv` oder `devbox global shellenv` anstelle von `direnv export bash`.

Um auf bestimmte Dateien statt auf jeden Verzeichniswechsel zu reagieren, verwenden Sie `FileChanged` mit einem `matcher`, der die zu überwachenden Dateinamen auflistet, getrennt durch `|`. Beim Erstellen der Überwachungsliste wird dieser Wert in Dateinamen aufgeteilt, anstatt als Regex ausgewertet zu werden. Siehe [FileChanged](/docs/de/hooks#filechanged) für die Funktionsweise desselben Werts, der auch filtert, welche Hook-Gruppen ausgeführt werden, wenn sich eine Datei ändert. Dieses Beispiel überwacht `.envrc` und `.env` im Arbeitsverzeichnis:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Siehe die [CwdChanged](/docs/de/hooks#cwdchanged)- und [FileChanged](/docs/de/hooks#filechanged)-Referenzeinträge für Eingabe-Schemas, `watchPaths`-Ausgabe und `CLAUDE_ENV_FILE`-Details.

<h3 id="auto-approve-specific-permission-prompts">
  Bestimmte Berechtigungsaufforderungen automatisch genehmigen
</h3>

Überspringen Sie den Genehmigungsdialog für Tool-Aufrufe, die Sie immer zulassen. Dieses Beispiel genehmigt automatisch `ExitPlanMode`, das Tool, das Claude aufruft, wenn es fertig ist, einen Plan zu präsentieren und fragt, ob es fortfahren soll, sodass Sie nicht jedes Mal aufgefordert werden, wenn ein Plan bereit ist.

Im Gegensatz zu den Exit-Code-Beispielen oben erfordert die automatische Genehmigung, dass Ihr Hook eine JSON-Entscheidung auf stdout schreibt. Ein `PermissionRequest`-Hook wird ausgelöst, wenn Claude Code einen Berechtigungsdialog anzeigen wird, und die Rückgabe von `"behavior": "allow"` beantwortet ihn in Ihrem Namen.

Der Matcher beschränkt den Hook nur auf `ExitPlanMode`, sodass keine anderen Aufforderungen betroffen sind. Fügen Sie dies zu `~/.claude/settings.json` hinzu:

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

Wenn der Hook genehmigt, beendet Claude Code den Plan-Modus und stellt den Berechtigungsmodus wieder her, der vor dem Eintritt in den Plan-Modus aktiv war. Das Transkript zeigt „Allowed by PermissionRequest hook" an der Stelle, an der der Dialog angezeigt worden wäre. Der Hook-Pfad behält immer das aktuelle Gespräch: Er kann den Kontext nicht löschen und eine neue Implementierungssitzung auf die Weise starten, wie der Dialog es kann.

Um stattdessen einen bestimmten Berechtigungsmodus festzulegen, kann die Ausgabe Ihres Hooks ein Array `updatedPermissions` mit einem `setMode`-Eintrag enthalten. Der Wert `mode` ist ein beliebiger Berechtigungsmodus wie `default`, `acceptEdits` oder `bypassPermissions`, und `destination: "session"` wendet ihn nur für die aktuelle Sitzung an.

<Note>
  `bypassPermissions` gilt nur, wenn die Sitzung mit bereits verfügbarem Bypass-Modus gestartet wurde: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` oder `permissions.defaultMode: "bypassPermissions"` in Einstellungen, und nicht deaktiviert durch [`permissions.disableBypassPermissionsMode`](/docs/de/permissions#managed-settings). Es wird niemals als `defaultMode` beibehalten.
</Note>

Um die Sitzung zu `acceptEdits` zu wechseln, schreibt Ihr Hook dieses JSON auf stdout:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

Halten Sie den Matcher so eng wie möglich. Das Abgleichen von `.*` oder das Lassen des Matchers leer würde jede Berechtigungsaufforderung automatisch genehmigen, einschließlich Dateischreibvorgänge und Shell-Befehle. Siehe die [PermissionRequest-Referenz](/docs/de/hooks#permissionrequest-decision-control) für den vollständigen Satz von Entscheidungsfeldern.

<h2 id="how-hooks-work">
  Wie Hooks funktionieren
</h2>

Hook-Events werden an bestimmten Lebenszykluspunkten in Claude Code ausgelöst. Wenn ein Event ausgelöst wird, werden alle übereinstimmenden Hooks parallel ausgeführt, und identische Hook-Befehle werden automatisch dedupliziert. Die folgende Tabelle zeigt jedes Event und wann es ausgelöst wird:

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

Jeder Hook hat einen `type`, der bestimmt, wie er ausgeführt wird. Die meisten Hooks verwenden `"type": "command"`, was einen Shell-Befehl ausführt. Vier weitere Typen sind verfügbar:

* `"type": "http"`: Event-Daten an eine URL POSTen. Siehe [HTTP-Hooks](#http-hooks).
* `"type": "mcp_tool"`: ein Tool auf einem bereits verbundenen MCP-Server aufrufen. Siehe [MCP-Tool-Hooks](/docs/de/hooks#mcp-tool-hook-fields).
* `"type": "prompt"`: Single-Turn-LLM-Bewertung. Siehe [Prompt-basierte Hooks](#prompt-based-hooks).
* `"type": "agent"`: Multi-Turn-Verifizierung mit Tool-Zugriff. Agent-Hooks sind experimentell und können sich ändern. Siehe [Agent-basierte Hooks](#agent-based-hooks).

<h3 id="combine-results-from-multiple-hooks">
  Ergebnisse aus mehreren Hooks kombinieren
</h3>

Wenn mehrere Hooks das gleiche Event abgleichen, wird jeder Hook-Befehl bis zur Fertigstellung ausgeführt, bevor Claude Code die Ergebnisse zusammenführt. Ein Hook, der `deny` zurückgibt, stoppt nicht die Ausführung von Sibling-Hooks. Verlassen Sie sich nicht darauf, dass ein Hook's `deny` Nebenwirkungen in einem anderen Hook unterdrückt.

Nachdem alle übereinstimmenden Hooks fertig sind, kombiniert Claude Code ihre Ausgaben. Für `PreToolUse`-Berechtigungsentscheidungen gewinnt die restriktivste Antwort in der Reihenfolge `deny`, `defer`, `ask`, `allow`. Text aus `additionalContext` wird von jedem Hook beibehalten und zusammen an Claude übergeben.

Das folgende Beispiel registriert zwei `PreToolUse`-Hooks auf `Bash`. Der erste hängt jeden Befehl an eine Protokolldatei an und beendet sich mit 0. Der zweite führt ein Skript aus, das mit 2 beendet wird, um zu verweigern, wenn der Befehl `rm -rf` enthält:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

Wenn Claude versucht, `rm -rf /tmp/build` auszuführen, werden beide Hooks parallel ausgeführt. Der Logging-Hook schreibt den Befehl in `~/.claude/bash.log` und beendet sich mit 0, was keine Entscheidung meldet. Der Guardrail-Hook beendet sich mit 2, was den Tool-Aufruf verweigert. Die Verweigerung gewinnt, sodass Claude Code den Befehl blockiert und Claude das Guardrail's stderr zeigt. Der Log-Eintrag wird trotzdem geschrieben, weil der Logging-Hook bereits ausgeführt wurde.

<h3 id="read-input-and-return-output">
  Eingabe lesen und Ausgabe zurückgeben
</h3>

Hooks kommunizieren mit Claude Code über stdin, stdout, stderr und Exit-Codes. Wenn ein Event ausgelöst wird, übergibt Claude Code Event-spezifische Daten als JSON an stdin Ihres Skripts. Ihr Skript liest diese Daten, führt seine Arbeit aus und teilt Claude Code mit, was als nächstes zu tun ist, über den Exit-Code.

<h4 id="hook-input">
  Hook-Eingabe
</h4>

Jedes Event enthält gemeinsame Felder wie `session_id` und `cwd`, aber jeder Event-Typ fügt unterschiedliche Daten hinzu. Wenn Claude beispielsweise einen Bash-Befehl ausführt, erhält ein `PreToolUse`-Hook etwa folgendes auf stdin:

```json theme={null}
{
  "session_id": "abc123",          // eindeutige ID für diese Sitzung
  "cwd": "/Users/sarah/myproject", // Arbeitsverzeichnis, wenn das Event ausgelöst wurde
  "hook_event_name": "PreToolUse", // welches Event diesen Hook ausgelöst hat
  "tool_name": "Bash",             // das Tool, das Claude verwenden wird
  "tool_input": {                  // die Argumente, die Claude an das Tool übergeben hat
    "command": "npm test"          // für Bash ist dies der Shell-Befehl
  }
}
```

Ihr Skript kann dieses JSON parsen und auf alle diese Felder reagieren. `UserPromptSubmit`-Hooks erhalten stattdessen den `prompt`-Text, `SessionStart`-Hooks erhalten die `source` (startup, resume, clear, compact) und so weiter. Siehe [Gemeinsame Eingabefelder](/docs/de/hooks#common-input-fields) in der Referenz für gemeinsame Felder und jeden Event-Abschnitt für Event-spezifische Schemas.

<h4 id="hook-output">
  Hook-Ausgabe
</h4>

Ihr Skript teilt Claude Code mit, was als nächstes zu tun ist, indem es auf stdout oder stderr schreibt und mit einem bestimmten Code beendet wird. Beispielsweise ein `PreToolUse`-Hook, der einen Befehl blockieren möchte:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr wird zu Claudes Feedback
  exit 2 # exit 2 = Aktion blockieren
fi

exit 0  # exit 0 = keine Entscheidung; der normale Berechtigungsfluss gilt
```

Der Exit-Code bestimmt, was als nächstes passiert:

* **Exit 0**: der Hook meldet keinen Einwand und die Aktion wird normal fortgesetzt. Für einen `PreToolUse`-Hook genehmigt dies nicht den Tool-Aufruf: der normale [Berechtigungsfluss](/docs/de/permissions) gilt weiterhin. Für `UserPromptSubmit`-, `UserPromptExpansion`- und `SessionStart`-Hooks wird alles, was Sie auf stdout schreiben, zu Claudes Kontext hinzugefügt.
* **Exit 2**: die Aktion wird blockiert. Schreiben Sie einen Grund auf stderr, und Claude erhält ihn als Feedback, sodass er sich anpassen kann. Einige Events können nicht blockiert werden: Für `SessionStart`, `Setup`, `Notification` und andere zeigt exit 2 stderr dem Benutzer an und die Ausführung wird fortgesetzt. Siehe [Exit-Code-2-Verhalten pro Event](/docs/de/hooks#exit-code-2-behavior-per-event) für die vollständige Liste.
* **Jeder andere Exit-Code**: die Aktion wird fortgesetzt. Das Transkript zeigt eine `<hook name> hook error`-Meldung gefolgt von der ersten Zeile von stderr; das vollständige stderr geht in das [Debug-Protokoll](/docs/de/hooks#debug-hooks).

<h4 id="structured-json-output">
  Strukturierte JSON-Ausgabe
</h4>

Exit-Codes geben Ihnen nur die Möglichkeit, zu blockieren oder zu schweigen. Für mehr Kontrolle beenden Sie mit 0 und geben stattdessen ein JSON-Objekt auf stdout aus.

<Note>
  Verwenden Sie exit 2, um mit einer stderr-Meldung zu blockieren, oder exit 0 mit JSON für strukturierte Kontrolle. Mischen Sie sie nicht: Claude Code ignoriert JSON, wenn Sie mit 2 beenden.
</Note>

Beispielsweise kann ein `PreToolUse`-Hook einen Tool-Aufruf ablehnen und Claude mitteilen, warum, oder ihn dem Benutzer zur Genehmigung eskalieren:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

Mit `"deny"` bricht Claude Code den Tool-Aufruf ab und gibt `permissionDecisionReason` an Claude als Feedback zurück. Diese `permissionDecision`-Werte sind spezifisch für `PreToolUse`:

* `"allow"`: die interaktive Berechtigungsaufforderung überspringen. Deny- und Ask-Regeln, einschließlich verwalteter Deny-Listen, gelten weiterhin, ebenso wie Aufforderungen für Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind
* `"deny"`: Tool-Aufruf abbrechen und den Grund an Claude senden
* `"ask"`: Berechtigungsaufforderung dem Benutzer wie gewohnt anzeigen

Ein vierter Wert, `"defer"`, ist im [nicht-interaktiven Modus](/docs/de/headless) mit dem Flag `-p` verfügbar. Er beendet den Prozess mit dem beibehaltenen Tool-Aufruf, sodass ein Agent SDK-Wrapper Eingaben sammeln und fortfahren kann. Siehe [Einen Tool-Aufruf für später aufschieben](/docs/de/hooks#defer-a-tool-call-for-later) in der Referenz.

Die Rückgabe von `"allow"` überspringt die interaktive Aufforderung, überschreibt aber nicht [Berechtigungsregeln](/docs/de/permissions#manage-permissions). Wenn eine Deny-Regel dem Tool-Aufruf entspricht, wird der Aufruf blockiert, auch wenn Ihr Hook `"allow"` zurückgibt. Wenn eine Ask-Regel entspricht, wird der Benutzer immer noch aufgefordert, ebenso wie Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind. Dies bedeutet, dass Deny-Regeln aus jedem Einstellungsbereich, einschließlich [verwalteter Einstellungen](/docs/de/settings#settings-files), immer Vorrang vor Hook-Genehmigungen haben.

Andere Events verwenden unterschiedliche Entscheidungsmuster. Beispielsweise verwenden `PostToolUse`- und `Stop`-Hooks ein Top-Level-Feld `decision: "block"`, während `PermissionRequest` `hookSpecificOutput.decision.behavior` verwendet. Siehe die [Zusammenfassungstabelle](/docs/de/hooks#decision-control) in der Referenz für eine vollständige Aufschlüsselung nach Event.

Für `UserPromptSubmit`-Hooks verwenden Sie `hookSpecificOutput.additionalContext` stattdessen, um Text in Claudes Kontext zu injizieren. Verschachteln Sie `additionalContext` in `hookSpecificOutput`; wenn Sie es auf der obersten Ebene des JSON platzieren, ignoriert Claude Code es stillschweigend. Beispielsweise fügt diese Ausgabe den aktuellen Branch-Status zu jedem Prompt hinzu:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

Siehe [UserPromptSubmit-Entscheidungskontrolle](/docs/de/hooks#userpromptsubmit-decision-control) für die vollständige Ausgabeform, einschließlich Blockierung von Prompts und Festlegung des Sitzungstitels.

Hooks mit `type: "prompt"` handhaben die Ausgabe anders: siehe [Prompt-basierte Hooks](#prompt-based-hooks).

<h3 id="filter-hooks-with-matchers">
  Hooks mit Matchern filtern
</h3>

Ohne einen Matcher wird ein Hook bei jedem Auftreten seines Events ausgelöst. Matcher ermöglichen es Ihnen, das einzugrenzen. Wenn Sie beispielsweise einen Formatter nur nach Datei-Bearbeitungen ausführen möchten (nicht nach jedem Tool-Aufruf), fügen Sie einen Matcher zu Ihrem `PostToolUse`-Hook hinzu:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

Der `"Edit|Write"`-Matcher wird ausgelöst, wenn Claude das `Edit`- oder `Write`-Tool verwendet, nicht wenn es `Bash`, `Read` oder ein anderes Tool verwendet. In Claude Code v2.1.191 oder später trennt ein Komma Alternativen auf die gleiche Weise, sodass `"Edit, Write"` gleichwertig ist. Siehe [Matcher-Muster](/docs/de/hooks#matcher-patterns) für die Auswertung von einfachen Namen und regulären Ausdrücken.

<Note>
  Claude kann auch Dateien erstellen oder ändern, indem er Shell-Befehle über das `Bash`-Tool ausführt. Wenn Ihr Hook jede Dateiänderung sehen muss, z. B. für Compliance-Scanning oder Audit-Protokollierung, fügen Sie einen [`Stop`](/docs/de/hooks#stop)-Hook hinzu, der den Arbeitsbaum einmal pro Runde scannt. Für Pro-Aufruf-Abdeckung stattdessen auch `Bash` abgleichen und Ihr Skript geänderte und nicht verfolgte Dateien mit `git status --porcelain` auflisten.
</Note>

Jeder Event-Typ gleicht ein bestimmtes Feld ab:

| Event                                                                                                                                                           | Worauf der Matcher filtert                                             | Beispiel-Matcher-Werte                                                                                                                                                              |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | Tool-Name                                                              | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                                  | wie die Sitzung gestartet wurde                                        | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                                         | welches CLI-Flag das Setup ausgelöst hat                               | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                                    | warum die Sitzung endete                                               | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                                  | Benachrichtigungstyp                                                   | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                                 | Agent-Typ                                                              | `general-purpose`, `Explore`, `Plan` oder benutzerdefinierte Agent-Namen                                                                                                            |
| `PreCompact`, `PostCompact`                                                                                                                                     | was die Komprimierung ausgelöst hat                                    | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                                  | Agent-Typ                                                              | gleiche Werte wie `SubagentStart`                                                                                                                                                   |
| `ConfigChange`                                                                                                                                                  | Konfigurationsquelle                                                   | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `StopFailure`                                                                                                                                                   | Fehlertyp                                                              | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                                            | Ladegrund                                                              | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `Elicitation`                                                                                                                                                   | MCP-Servername                                                         | Ihre konfigurierten MCP-Servernamen                                                                                                                                                 |
| `ElicitationResult`                                                                                                                                             | MCP-Servername                                                         | gleiche Werte wie `Elicitation`                                                                                                                                                     |
| `FileChanged`                                                                                                                                                   | Dateinamen zum Überwachen (siehe [FileChanged](/docs/de/hooks#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `UserPromptExpansion`                                                                                                                                           | Befehlsname                                                            | Ihre Skill- oder Befehlsnamen                                                                                                                                                       |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | keine Matcher-Unterstützung                                            | wird immer bei jedem Auftreten ausgelöst                                                                                                                                            |

Die Registerkarten unten zeigen ein paar weitere Matcher auf verschiedene Event-Typen.

<Tabs>
  <Tab title="Jeden Bash-Befehl protokollieren">
    Gleichen Sie nur `Bash`-Tool-Aufrufe ab und protokollieren Sie jeden Befehl in einer Datei. Das `PostToolUse`-Event wird ausgelöst, nachdem der Befehl abgeschlossen ist, sodass `tool_input.command` enthält, was ausgeführt wurde. Der Hook erhält die Event-Daten als JSON auf stdin, und `jq -r '.tool_input.command'` extrahiert nur die Befehlszeichenfolge, die `>>` an die Protokolldatei anhängt:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="MCP-Tools abgleichen">
    MCP-Tools verwenden eine andere Namenskonvention als integrierte Tools: `mcp__<server>__<tool>`, wobei `<server>` der MCP-Servername und `<tool>` das Tool ist, das er bereitstellt. Beispielsweise `mcp__github__search_repositories` oder `mcp__filesystem__read_file`. Tools von einem [Plugin-gebündelten Server](/docs/de/mcp#plugin-provided-mcp-servers) verwenden stattdessen ein Scoped-Server-Segment, z. B. `mcp__plugin_my-plugin_db__query`. Verwenden Sie einen Regex-Matcher, um alle Tools von einem bestimmten Server zu erfassen, oder gleichen Sie Server übergreifend mit einem Muster wie `mcp__.*__write.*` ab. Siehe [MCP-Tools abgleichen](/docs/de/hooks#match-mcp-tools) in der Referenz für die vollständige Liste der Beispiele.

    Der folgende Befehl extrahiert den Tool-Namen aus der Hook-JSON-Eingabe mit `jq` und schreibt ihn auf stderr. Das Schreiben auf stderr hält stdout sauber für JSON-Ausgabe und sendet die Meldung in das [Debug-Protokoll](/docs/de/hooks#debug-hooks):

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Beim Sitzungsende aufräumen">
    Das `SessionEnd`-Event unterstützt Matcher auf den Grund, warum die Sitzung endete. Dieser Hook wird nur bei `clear` ausgelöst (wenn Sie `/clear` ausführen), nicht bei normalen Exits:

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Für die vollständige Matcher-Syntax siehe die [Hooks-Referenz](/docs/de/hooks#configuration).

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  Hooks mit dem Feld `if` nach Tool-Name und Argumenten filtern
</h4>

Das Feld `if` verwendet [Berechtigungsregel-Syntax](/docs/de/permissions) zum Filtern von Hooks nach Tool-Name und Argumenten zusammen, sodass der Hook-Prozess nur spawnt, wenn der Tool-Aufruf übereinstimmt. Dies geht über `matcher` hinaus, das auf der Gruppenebene nur nach Tool-Name filtert.

Beispielsweise, um einen Hook nur auszuführen, wenn Claude `git`-Befehle verwendet, anstatt alle Bash-Befehle:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

Ob Ihr Hook-Befehl ausgeführt wird, hängt von der Form Ihres `if`-Musters und dem Bash-Befehl ab, den Claude aufruft:

| `if`-Muster        | Bash-Befehl            | Hook wird ausgeführt? | Warum                                                                                                        |
| :----------------- | :--------------------- | :-------------------- | :----------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `git push`             | ja                    | Befehlsname stimmt überein                                                                                   |
| `Bash(git *)`      | `npm test && git push` | ja                    | jeder Subbefehl wird überprüft; `git push` stimmt überein                                                    |
| `Bash(git *)`      | `echo $(git log)`      | ja                    | Befehle in `$()` und Backticks werden überprüft; `git log` stimmt überein                                    |
| `Bash(git *)`      | `echo $(date)`         | nein                  | kein Subbefehl stimmt mit `git *` überein                                                                    |
| `Bash(git push *)` | `echo $(date)`         | ja                    | Muster, die mehr als den Befehlsnamen angeben, führen den Hook trotzdem bei `$()`, Backticks oder `$VAR` aus |

Der Filter schlägt auch offen fehl und führt Ihren Hook unabhängig vom Muster aus, wenn der Bash-Befehl nicht geparst werden kann. Da der Filter Best-Effort ist, verwenden Sie das [Berechtigungssystem](/docs/de/permissions) anstelle eines Hooks, um ein hartes Allow oder Deny durchzusetzen.

Das Feld `if` akzeptiert die gleichen Muster wie Berechtigungsregeln: `"Bash(git *)"`, `"Edit(*.ts)"` und so weiter. Um mehrere Tool-Namen abzugleichen, verwenden Sie separate Handler, jeder mit seinem eigenen `if`-Wert, oder gleichen Sie auf der `matcher`-Ebene ab, wo Pipe-Alternation unterstützt wird.

`if` funktioniert nur bei Tool-Events: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` und `PermissionDenied`. Das Hinzufügen zu einem anderen Event verhindert, dass der Hook ausgeführt wird.

<h3 id="configure-hook-location">
  Hook-Speicherort konfigurieren
</h3>

Wo Sie einen Hook hinzufügen, bestimmt seinen Bereich:

| Speicherort                                                  | Bereich                                | Freigegeben                                     |
| :----------------------------------------------------------- | :------------------------------------- | :---------------------------------------------- |
| `~/.claude/settings.json`                                    | Alle Ihre Projekte                     | Nein, lokal auf Ihrem Computer                  |
| `.claude/settings.json`                                      | Einzelnes Projekt                      | Ja, kann im Repo committed werden               |
| `.claude/settings.local.json`                                | Einzelnes Projekt                      | Nein, gitignoriert wenn Claude Code es erstellt |
| Verwaltete Richtlinieneinstellungen                          | Organisationsweit                      | Ja, von Admin kontrolliert                      |
| [Plugin](/docs/de/plugins) `hooks/hooks.json`                     | Wenn Plugin aktiviert ist              | Ja, mit dem Plugin gebündelt                    |
| [Skill](/docs/de/skills) oder [Agent](/docs/de/sub-agents) Frontmatter | Während der Skill oder Agent aktiv ist | Ja, in der Komponentendatei definiert           |

Führen Sie [`/hooks`](/docs/de/hooks#the-%2Fhooks-menu) in Claude Code aus, um alle konfigurierten Hooks nach Event gruppiert zu durchsuchen.

Um Hooks zu deaktivieren, setzen Sie `"disableAllHooks": true` in Ihrer Einstellungsdatei. Hooks, die in verwalteten Einstellungen konfiguriert sind, werden weiterhin ausgeführt, es sei denn, `disableAllHooks` ist auch dort gesetzt.

Wenn Sie Einstellungsdateien direkt bearbeiten, während Claude Code läuft, werden Hook-Änderungen normalerweise automatisch vom Datei-Watcher aufgegriffen.

<h2 id="prompt-based-hooks">
  Prompt-basierte Hooks
</h2>

Für Entscheidungen, die Urteilsvermögen erfordern, anstatt deterministischer Regeln, verwenden Sie `type: "prompt"`-Hooks. Anstatt einen Shell-Befehl auszuführen, sendet Claude Code Ihren Prompt und die Hook-Eingabedaten an ein Claude-Modell (standardmäßig Haiku), um die Entscheidung zu treffen. Sie können ein anderes Modell mit dem Feld `model` angeben, wenn Sie mehr Leistung benötigen.

Die einzige Aufgabe des Modells ist, eine Ja/Nein-Entscheidung als JSON zurückzugeben:

* `"ok": true`: die Aktion wird fortgesetzt
* `"ok": false`: was passiert, hängt vom Ereignis ab:
  * `Stop` und `SubagentStop`: der `reason` wird an Claude zurückgegeben, sodass es weiterarbeitet
  * `PreToolUse`: der Tool-Aufruf wird verweigert und der `reason` wird an Claude als Tool-Fehler zurückgegeben, sodass es sich anpassen und fortfahren kann
  * `PostToolUse`, `PostToolBatch`, `UserPromptSubmit` und `UserPromptExpansion`: der Zug endet und der `reason` erscheint im Chat als Warnzeile

Dieses Beispiel verwendet einen `Stop`-Hook, um das Modell zu fragen, ob alle angeforderten Aufgaben abgeschlossen sind. Wenn das Modell `"ok": false` zurückgibt, arbeitet Claude weiter und verwendet den `reason` als nächste Anweisung:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

Für vollständige Konfigurationsoptionen siehe [Prompt-basierte Hooks](/docs/de/hooks#prompt-based-hooks) in der Referenz.

<h2 id="agent-based-hooks">
  Agent-basierte Hooks
</h2>

<Warning>
  Agent-Hooks sind experimentell. Verhalten und Konfiguration können sich in zukünftigen Versionen ändern. Für Produktions-Workflows bevorzugen Sie [Command-Hooks](/docs/de/hooks#command-hook-fields).
</Warning>

Wenn die Verifizierung das Inspizieren von Dateien oder das Ausführen von Befehlen erfordert, verwenden Sie `type: "agent"`-Hooks. Im Gegensatz zu Prompt-Hooks, die einen einzelnen LLM-Aufruf tätigen, spawnen Agent-Hooks einen Subagent, der Dateien lesen, Code durchsuchen und andere Tools verwenden kann, um Bedingungen zu überprüfen, bevor eine Entscheidung zurückgegeben wird.

Agent-Hooks verwenden das gleiche `"ok"` / `"reason"`-Antwortformat wie Prompt-Hooks, aber mit einem längeren Standard-Timeout von 60 Sekunden und bis zu 50 Tool-Use-Turns.

Dieses Beispiel überprüft, dass Tests bestanden werden, bevor Claude beendet werden darf:

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

Verwenden Sie Prompt-Hooks, wenn die Hook-Eingabedaten allein ausreichen, um eine Entscheidung zu treffen. Verwenden Sie Agent-Hooks, wenn Sie etwas gegen den tatsächlichen Zustand der Codebasis überprüfen müssen.

Für vollständige Konfigurationsoptionen siehe [Agent-basierte Hooks](/docs/de/hooks#agent-based-hooks) in der Referenz.

<h2 id="http-hooks">
  HTTP-Hooks
</h2>

Verwenden Sie `type: "http"`-Hooks, um Event-Daten an einen HTTP-Endpunkt zu POSTen, anstatt einen Shell-Befehl auszuführen. Der Endpunkt erhält die gleichen JSON-Daten, die ein Command-Hook auf stdin erhalten würde, und gibt Ergebnisse über den HTTP-Antwortkörper mit dem gleichen JSON-Format zurück.

HTTP-Hooks sind nützlich, wenn Sie möchten, dass ein Webserver, eine Cloud-Funktion oder ein externer Service Hook-Logik handhabt: beispielsweise ein gemeinsamer Audit-Service, der Tool-Use-Events über ein Team hinweg protokolliert.

Dieses Beispiel POSTet jeden Tool-Use an einen lokalen Logging-Service:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
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

Der Endpunkt sollte einen JSON-Antwortkörper mit dem gleichen [Ausgabeformat](/docs/de/hooks#json-output) wie Command-Hooks zurückgeben. Um einen Tool-Aufruf zu blockieren, geben Sie eine 2xx-Antwort mit den entsprechenden `hookSpecificOutput`-Feldern zurück. HTTP-Statuscodes allein können Aktionen nicht blockieren.

Header-Werte unterstützen Umgebungsvariablen-Interpolation mit `$VAR_NAME` oder `${VAR_NAME}`-Syntax. Nur Variablen, die im Array `allowedEnvVars` aufgelistet sind, werden aufgelöst; alle anderen `$VAR`-Referenzen bleiben leer.

Für vollständige Konfigurationsoptionen und Response-Handling siehe [HTTP-Hooks](/docs/de/hooks#http-hook-fields) in der Referenz.

<h2 id="limitations-and-troubleshooting">
  Einschränkungen und Fehlerbehebung
</h2>

<h3 id="limitations">
  Einschränkungen
</h3>

Beachten Sie diese Einschränkungen beim Entwerfen von Hooks:

* Command-Hooks kommunizieren nur über stdout, stderr und Exit-Codes. Sie können `/`-Befehle oder Tool-Aufrufe nicht auslösen. Text, der über `additionalContext` zurückgegeben wird, wird als Systemerinnerung injiziert, die Claude als Klartext liest. HTTP-Hooks kommunizieren stattdessen über den Response-Body.
* Hook-Timeouts variieren je nach Typ. Überschreiben Sie pro Hook mit dem Feld `timeout` in Sekunden.
  * `command`, `http`, `mcp_tool`: 10 Minuten. `UserPromptSubmit` reduziert diese auf 30 Sekunden, und `MessageDisplay` reduziert sie auf 10 Sekunden.
  * `prompt`: 30 Sekunden.
  * `agent`: 60 Sekunden.
* `PostToolUse`-Hooks können Aktionen nicht rückgängig machen, da das Tool bereits ausgeführt wurde.
* `PermissionRequest`-Hooks werden nicht im [nicht-interaktiven Modus](/docs/de/headless) mit dem Flag `-p` ausgelöst. Verwenden Sie `PreToolUse`-Hooks für automatisierte Berechtigungsentscheidungen.
* `Stop`-Hooks werden ausgelöst, wenn Claude antwortet, nicht nur bei Aufgabenabschluss. Sie werden nicht bei Benutzerunterbrechungen ausgelöst. API-Fehler lösen stattdessen [StopFailure](/docs/de/hooks#stopfailure) aus.
* Wenn mehrere `PreToolUse`-Hooks [`updatedInput`](/docs/de/hooks#pretooluse) zurückgeben, um die Argumente eines Tools umzuschreiben, gewinnt der letzte, der fertig wird. Da Hooks parallel ausgeführt werden, ist die Reihenfolge nicht deterministisch. Vermeiden Sie, dass mehr als ein Hook die Eingabe desselben Tools ändert.

<h3 id="hooks-and-permission-modes">
  Hooks und Berechtigungsmodi
</h3>

`PreToolUse`-Hooks werden vor jeder Berechtigungsmodus-Überprüfung ausgelöst. Ein Hook, der `permissionDecision: "deny"` zurückgibt, blockiert das Tool auch im `bypassPermissions`-Modus oder mit `--dangerously-skip-permissions`. Dies ermöglicht es Ihnen, Richtlinien durchzusetzen, die Benutzer nicht umgehen können, indem sie ihren Berechtigungsmodus ändern.

Das Gegenteil ist nicht wahr: Ein Hook, der `"allow"` zurückgibt, umgeht keine Deny-Regeln aus Einstellungen, und er kann die Aufforderung für Connector-Tools nicht unterdrücken, [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), oder MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind. Hooks können Einschränkungen verschärfen, aber nicht über das hinaus lockern, was Berechtigungsregeln zulassen.

<h3 id="hook-not-firing">
  Hook wird nicht ausgelöst
</h3>

Der Hook ist konfiguriert, wird aber nie ausgeführt.

* Führen Sie `/hooks` aus und bestätigen Sie, dass der Hook unter dem richtigen Event angezeigt wird
* Überprüfen Sie, dass das Matcher-Muster den Tool-Namen genau abgleicht. Matcher sind Groß-/Kleinschreibung-empfindlich
* Überprüfen Sie, dass Sie den richtigen Event-Typ auslösen: `PreToolUse` wird vor der Tool-Ausführung ausgelöst, `PostToolUse` wird danach ausgelöst
* Wenn Sie `PermissionRequest`-Hooks im nicht-interaktiven Modus mit dem Flag `-p` verwenden, wechseln Sie stattdessen zu `PreToolUse`

<h3 id="hook-error-in-output">
  Hook-Fehler in der Ausgabe
</h3>

Sie sehen eine Meldung wie "PreToolUse hook error: ..." im Transkript.

* Ihr Skript wurde unerwartet mit einem Nicht-Null-Code beendet. Testen Sie es manuell, indem Sie Beispiel-JSON pipen:
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # Überprüfen Sie den Exit-Code
  ```
* Wenn Sie "command not found" sehen, verwenden Sie absolute Pfade oder `${CLAUDE_PROJECT_DIR}`, um Skripte zu referenzieren. Um Shell-Quoting vollständig zu vermeiden, fügen Sie `"args": []` hinzu, um zur [exec-Form](/docs/de/hooks#exec-form-and-shell-form) zu wechseln, die das Skript direkt ohne eine Shell spawnt
* Wenn Sie "jq: command not found" sehen, installieren Sie `jq` oder verwenden Sie Python/Node.js zum Parsen von JSON
* Wenn das Skript überhaupt nicht ausgeführt wird, machen Sie es ausführbar: `chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks` zeigt keine konfigurierten Hooks
</h3>

Sie haben eine Einstellungsdatei bearbeitet, aber die Hooks werden nicht im Menü angezeigt.

* Datei-Bearbeitungen werden normalerweise automatisch aufgegriffen. Wenn sie nach ein paar Sekunden nicht angezeigt wurden, hat der Datei-Watcher die Änderung möglicherweise verpasst: Starten Sie Ihre Sitzung neu, um ein Neuladen zu erzwingen.
* Überprüfen Sie, dass Ihr JSON gültig ist: Nachfolgende Kommas und Kommentare sind nicht zulässig
* Bestätigen Sie, dass die Einstellungsdatei am richtigen Speicherort ist: `.claude/settings.json` für Projekt-Hooks, `~/.claude/settings.json` für globale Hooks

<h3 id="stop-hook-hits-the-block-cap">
  Stop-Hook trifft die Blockierungsgrenze
</h3>

Claude arbeitet weiter, anstatt zu stoppen, und beendet dann den Zug mit einer Warnung, dass der Stop-Hook zu viele Male hintereinander blockiert hat.

Claude Code setzt einen Stop-Hook außer Kraft, nachdem er acht Mal hintereinander blockiert hat, ohne Fortschritt zu erzielen. Ihr Hook-Skript muss überprüfen, ob es bereits eine Fortsetzung ausgelöst hat. Parsen Sie das Feld `stop_hook_active` aus der JSON-Eingabe und beenden Sie früh, wenn es `true` ist:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Erlauben Sie Claude zu stoppen
fi
# ... Rest Ihrer Hook-Logik
```

Wenn Ihr Hook legitim mehr als acht Iterationen benötigt, um zu konvergieren, erhöhen Sie die Grenze mit [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/de/env-vars).

<h3 id="json-validation-failed">
  JSON-Validierung fehlgeschlagen
</h3>

Claude Code zeigt einen JSON-Parsing-Fehler an, obwohl Ihr Hook-Skript gültiges JSON ausgibt.

Wenn Claude Code einen Shell-Form-Command-Hook ausführt (einen ohne `args`), spawnt es `sh -c` auf macOS und Linux oder Git Bash auf Windows standardmäßig. Diese Shell ist nicht-interaktiv, aber Git Bash und einige Konfigurationen, wie `BASH_ENV`, das auf `~/.bashrc` zeigt, sourcen trotzdem Ihr Profil. Wenn dieses Profil bedingungslose `echo`-Anweisungen enthält, wird die Ausgabe Ihrem Hook-JSON vorangestellt:

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

Claude Code versucht, dies als JSON zu parsen, und schlägt fehl. Um dies zu beheben, wrappen Sie Echo-Anweisungen in Ihrem Shell-Profil, sodass sie nur in interaktiven Shells ausgeführt werden:

```bash theme={null}
# In ~/.zshrc oder ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

Die Variable `$-` enthält Shell-Flags, und `i` bedeutet interaktiv. Hooks werden in nicht-interaktiven Shells ausgeführt, sodass das Echo übersprungen wird.

<h3 id="debug-techniques">
  Debug-Techniken
</h3>

Die Transkript-Ansicht, umgeschaltet mit `Ctrl+O`, zeigt eine einzeilige Zusammenfassung für jeden Hook, der ausgelöst wurde: Erfolg ist stumm, Blockierungsfehler zeigen stderr, und Nicht-Blockierungsfehler zeigen eine `<hook name> hook error`-Meldung gefolgt von der ersten Zeile von stderr.

Für vollständige Ausführungsdetails einschließlich welche Hooks abgeglichen wurden, ihrer Exit-Codes, stdout und stderr, lesen Sie das Debug-Protokoll. Starten Sie Claude Code mit `claude --debug-file /tmp/claude.log`, um in einen bekannten Pfad zu schreiben, dann `tail -f /tmp/claude.log` in einem anderen Terminal. Wenn Sie ohne dieses Flag gestartet haben, führen Sie `/debug` während der Sitzung aus, um Logging zu aktivieren und den Protokollpfad zu finden.

<h2 id="learn-more">
  Weitere Informationen
</h2>

* [Hooks-Referenz](/docs/de/hooks): vollständige Event-Schemas, JSON-Ausgabeformat, asynchrone Hooks und MCP-Tool-Hooks
* [Sicherheitsüberlegungen](/docs/de/hooks#security-considerations): überprüfen Sie vor der Bereitstellung von Hooks in gemeinsamen oder Produktionsumgebungen
* [Bash-Befehlsvalidator-Beispiel](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py): vollständige Referenzimplementierung
