> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Passen Sie Ihre Statuszeile an

> Konfigurieren Sie eine benutzerdefinierte Statusleiste zur Überwachung der Kontextfensternutzung, Kosten und Git-Status in Claude Code

Die Statuszeile ist eine anpassbare Leiste am unteren Rand von Claude Code, die jedes Shell-Skript ausführt, das Sie konfigurieren. Sie empfängt JSON-Sitzungsdaten auf stdin und zeigt alles an, was Ihr Skript ausgibt, und bietet Ihnen eine persistente, auf einen Blick sichtbare Ansicht der Kontextnutzung, Kosten, Git-Status oder alles andere, das Sie verfolgen möchten.

Statuszeilen sind nützlich, wenn Sie:

* Die Kontextfensternutzung während der Arbeit überwachen möchten
* Sitzungskosten verfolgen müssen
* Über mehrere Sitzungen hinweg arbeiten und diese unterscheiden müssen
* Git-Branch und Status immer sichtbar haben möchten

Die Statuszeile wird in ihrer eigenen Zeile über den integrierten Fußzeilenabzeichen gerendert und ersetzt diese nicht. Um anklickbare Link-Abzeichen in der Fußzeile hinzuzufügen, wenn eine ID im Gespräch angezeigt wird, ohne ein Skript zu schreiben, konfigurieren Sie stattdessen [`footerLinksRegexes`](/docs/de/settings#footer-link-badges).

Hier ist ein Beispiel einer [mehrzeiligen Statuszeile](#display-multiple-lines), die Git-Informationen in der ersten Zeile und einen farbcodierten Kontextbalken in der zweiten Zeile anzeigt.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Eine mehrzeilige Statuszeile, die Modellname, Verzeichnis, Git-Branch in der ersten Zeile und einen Kontextnutzungs-Fortschrittsbalken mit Kosten und Dauer in der zweiten Zeile anzeigt" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Diese Seite führt Sie durch [das Einrichten einer grundlegenden Statuszeile](#set-up-a-status-line), erklärt [wie die Daten fließen](#how-status-lines-work) von Claude Code zu Ihrem Skript, listet [alle Felder auf, die Sie anzeigen können](#available-data), und bietet [einsatzbereite Beispiele](#examples) für häufige Muster wie Git-Status, Kostenverfolgung und Fortschrittsbalken.

<h2 id="set-up-a-status-line">
  Richten Sie eine Statuszeile ein
</h2>

Verwenden Sie den [`/statusline` Befehl](#use-the-%2Fstatusline-command), um Claude Code ein Skript für Sie generieren zu lassen, oder [erstellen Sie manuell ein Skript](#manually-configure-a-status-line) und fügen Sie es zu Ihren Einstellungen hinzu.

<h3 id="use-the-/statusline-command">
  Verwenden Sie den /statusline Befehl
</h3>

Der `/statusline` Befehl akzeptiert Anweisungen in natürlicher Sprache, die beschreiben, was Sie angezeigt haben möchten. Claude Code generiert eine Skriptdatei in `~/.claude/` und aktualisiert Ihre Einstellungen automatisch:

```text theme={null}
/statusline show model name and context percentage with a progress bar
```

<h3 id="manually-configure-a-status-line">
  Statuszeile manuell konfigurieren
</h3>

Fügen Sie ein `statusLine` Feld zu Ihren Benutzereinstellungen (`~/.claude/settings.json`, wobei `~` Ihr Heimatverzeichnis ist) oder [Projekteinstellungen](/docs/de/settings#settings-files) hinzu. Setzen Sie `type` auf `"command"` und verweisen Sie `command` auf einen Skriptpfad oder einen Inline-Shell-Befehl. Eine vollständige Anleitung zum Erstellen eines Skripts finden Sie unter [Erstellen Sie eine Statuszeile Schritt für Schritt](#build-a-status-line-step-by-step).

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/statusline.sh",
    "padding": 2
  }
}
```

Das `command` Feld wird in einer Shell ausgeführt, sodass Sie auch Inline-Befehle anstelle einer Skriptdatei verwenden können. Dieses Beispiel verwendet `jq`, um die JSON-Eingabe zu analysieren und den Modellnamen und den Kontextprozentsatz anzuzeigen:

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'"
  }
}
```

Das optionale `padding` Feld fügt zusätzlichen horizontalen Abstand (in Zeichen) zum Inhalt der Statuszeile hinzu. Standardmäßig `0`. Dieser Abstand wird zusätzlich zum integrierten Abstand der Benutzeroberfläche hinzugefügt, sodass er die relative Einrückung steuert, anstatt den absoluten Abstand vom Terminalrand.

Das optionale `refreshInterval` Feld führt Ihren Befehl zusätzlich zu den [ereignisgesteuerten Aktualisierungen](#how-status-lines-work) alle N Sekunden erneut aus. Das Minimum ist `1`. Setzen Sie dies, wenn Ihre Statuszeile zeitbasierte Daten wie eine Uhr anzeigt, oder wenn Hintergrund-Subagenten den Git-Status ändern, während die Hauptsitzung untätig ist. Lassen Sie es ungesetzt, um nur bei Ereignissen auszuführen.

Das optionale `hideVimModeIndicator` Feld unterdrückt den integrierten `-- INSERT --` Text unter der Eingabeaufforderung. Setzen Sie dies auf `true`, wenn Ihr Skript [`vim.mode`](#available-data) selbst rendert, sodass der Modus nicht zweimal angezeigt wird.

<h3 id="disable-the-status-line">
  Deaktivieren Sie die Statuszeile
</h3>

Führen Sie `/statusline` aus und bitten Sie es, Ihre Statuszeile zu entfernen oder zu löschen (z. B. `/statusline delete`, `/statusline clear`, `/statusline remove it`). Sie können auch das `statusLine` Feld manuell aus Ihrer settings.json löschen.

<h2 id="build-a-status-line-step-by-step">
  Erstellen Sie eine Statuszeile Schritt für Schritt
</h2>

Diese Anleitung zeigt, was unter der Haube passiert, indem Sie manuell eine Statuszeile erstellen, die das aktuelle Modell, das Arbeitsverzeichnis und den Prozentsatz der Kontextfensternutzung anzeigt.

<Note>Das Ausführen von [`/statusline`](#use-the-%2Fstatusline-command) mit einer Beschreibung dessen, was Sie möchten, konfiguriert all dies automatisch für Sie.</Note>

Diese Beispiele verwenden Bash-Skripte, die auf macOS und Linux funktionieren. Unter Windows finden Sie unter [Windows-Konfiguration](#windows-configuration) PowerShell- und Git Bash-Beispiele.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-quickstart.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=696445e59ca0059213250651ad23db6b" alt="Eine Statuszeile, die Modellname, Verzeichnis und Kontextprozentsatz anzeigt" width="726" height="164" data-path="images/statusline-quickstart.png" />
</Frame>

<Steps>
  <Step title="Erstellen Sie ein Skript, das JSON liest und Ausgabe druckt">
    Claude Code sendet JSON-Daten über stdin an Ihr Skript. Dieses Skript verwendet [`jq`](https://jqlang.github.io/jq/), einen Befehlszeilen-JSON-Parser, den Sie möglicherweise installieren müssen, um den Modellnamen, das Verzeichnis und den Kontextprozentsatz zu extrahieren, und druckt dann eine formatierte Zeile.

    Speichern Sie dies unter `~/.claude/statusline.sh` (wobei `~` Ihr Heimatverzeichnis ist, z. B. `/Users/username` auf macOS oder `/home/username` auf Linux):

    ```bash theme={null}
    #!/bin/bash
    # Read JSON data that Claude Code sends to stdin
    input=$(cat)

    # Extract fields using jq
    MODEL=$(echo "$input" | jq -r '.model.display_name')
    DIR=$(echo "$input" | jq -r '.workspace.current_dir')
    # The "// 0" provides a fallback if the field is null
    PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

    # Output the status line - ${DIR##*/} extracts just the folder name
    echo "[$MODEL] 📁 ${DIR##*/} | ${PCT}% context"
    ```
  </Step>

  <Step title="Machen Sie es ausführbar">
    Markieren Sie das Skript als ausführbar, damit Ihre Shell es ausführen kann:

    ```bash theme={null}
    chmod +x ~/.claude/statusline.sh
    ```
  </Step>

  <Step title="Zu Einstellungen hinzufügen">
    Teilen Sie Claude Code mit, dass es Ihr Skript als Statuszeile ausführen soll. Fügen Sie diese Konfiguration zu `~/.claude/settings.json` hinzu, die `type` auf `"command"` setzt (was bedeutet „diesen Shell-Befehl ausführen") und `command` auf Ihr Skript verweist:

    ```json theme={null}
    {
      "statusLine": {
        "type": "command",
        "command": "~/.claude/statusline.sh"
      }
    }
    ```

    Ihre Statuszeile wird am unteren Rand der Benutzeroberfläche angezeigt. Einstellungen werden automatisch neu geladen, aber Änderungen werden erst bei Ihrer nächsten Interaktion mit Claude Code angezeigt.
  </Step>
</Steps>

<h2 id="how-status-lines-work">
  Wie Statuszeilen funktionieren
</h2>

Claude Code führt Ihr Skript aus und leitet [JSON-Sitzungsdaten](#available-data) über stdin an es weiter. Ihr Skript liest die JSON, extrahiert das, was es benötigt, und druckt Text auf stdout. Claude Code zeigt alles an, was Ihr Skript druckt.

**Wann es aktualisiert wird**

Ihr Skript wird nach jeder neuen Assistentnachricht ausgeführt, nachdem `/compact` abgeschlossen ist, wenn sich der Berechtigungsmodus ändert oder wenn der Vim-Modus umgeschaltet wird. Aktualisierungen werden mit 300 ms entprellt, was bedeutet, dass schnelle Änderungen zusammengefasst werden und Ihr Skript einmal ausgeführt wird, wenn sich die Dinge beruhigen. Wenn eine neue Aktualisierung ausgelöst wird, während Ihr Skript noch läuft, wird die laufende Ausführung abgebrochen. Wenn Sie Ihr Skript bearbeiten, werden die Änderungen erst bei Ihrer nächsten Interaktion mit Claude Code angezeigt, die eine Aktualisierung auslöst.

Diese Trigger können ruhig werden, wenn die Hauptsitzung untätig ist, z. B. während ein Koordinator auf Hintergrund-Subagenten wartet. Um zeitbasierte oder extern bezogene Segmente während untätiger Perioden aktuell zu halten, setzen Sie [`refreshInterval`](#manually-configure-a-status-line), um den Befehl auch auf einem festen Timer erneut auszuführen.

**Was Ihr Skript ausgeben kann**

* **Mehrere Zeilen**: Jede `echo` oder `print` Anweisung wird als separate Zeile angezeigt. Siehe das [mehrzeilige Beispiel](#display-multiple-lines).
* **Farben**: Verwenden Sie [ANSI-Escape-Codes](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) wie `\033[32m` für Grün (Terminal muss diese unterstützen). Siehe das [Git-Status-Beispiel](#git-status-with-colors).
* **Links**: Verwenden Sie [OSC 8 Escape-Sequenzen](https://en.wikipedia.org/wiki/ANSI_escape_code#OSC), um Text anklickbar zu machen (Cmd+Klick auf macOS, Strg+Klick auf Windows/Linux). Erfordert ein Terminal, das Hyperlinks wie iTerm2, Kitty oder WezTerm unterstützt. Siehe das [anklickbare Links-Beispiel](#clickable-links).

**Ausgabe an die Terminalgröße anpassen**

Claude Code erfasst die Ausgabe Ihres Skripts, anstatt sie direkt mit dem Terminal zu verbinden, daher können `tput cols` und sprachgestützte Breitenerkennung die Terminalgröße nicht von innerhalb des Skripts lesen. Lesen Sie stattdessen die Umgebungsvariablen `COLUMNS` und `LINES`. Claude Code setzt diese auf die aktuellen Terminaldimensionen, bevor Ihr Skript ausgeführt wird. Erfordert Claude Code v2.1.153 oder später.

<Note>Die Statuszeile wird lokal ausgeführt und verbraucht keine API-Token. Sie wird vorübergehend während bestimmter UI-Interaktionen ausgeblendet, einschließlich Autovervollständigungsvorschlägen, dem Hilfemenü und Berechtigungsaufforderungen.</Note>

<h2 id="available-data">
  Verfügbare Daten
</h2>

Claude Code sendet die folgenden JSON-Felder über stdin an Ihr Skript:

| Feld                                                                             | Beschreibung                                                                                                                                                                                                                                                                                                           |
| -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model.id`, `model.display_name`                                                 | Aktuelle Modellkennung und Anzeigename                                                                                                                                                                                                                                                                                 |
| `cwd`, `workspace.current_dir`                                                   | Aktuelles Arbeitsverzeichnis. Beide Felder enthalten denselben Wert; `workspace.current_dir` wird für Konsistenz mit `workspace.project_dir` bevorzugt.                                                                                                                                                                |
| `workspace.project_dir`                                                          | Verzeichnis, in dem Claude Code gestartet wurde, das sich von `cwd` unterscheiden kann, wenn sich das Arbeitsverzeichnis während einer Sitzung ändert                                                                                                                                                                  |
| `workspace.added_dirs`                                                           | Zusätzliche Verzeichnisse, die über `/add-dir` oder `--add-dir` hinzugefügt wurden. Leeres Array, wenn keine hinzugefügt wurden                                                                                                                                                                                        |
| `workspace.git_worktree`                                                         | Git-Worktree-Name, wenn sich das aktuelle Verzeichnis in einem verknüpften Worktree befindet, der mit `git worktree add` erstellt wurde. Fehlt im Haupt-Worktree. Wird für jeden Git-Worktree gefüllt, im Gegensatz zu `worktree.*`, das nur auf `--worktree` Sitzungen zutrifft                                       |
| `workspace.repo.host`, `workspace.repo.owner`, `workspace.repo.name`             | Repository-Identität, die aus dem `origin` Remote geparst wird, zum Beispiel `"github.com"`, `"anthropics"`, `"claude-code"`. Fehlt außerhalb eines Git-Repositorys oder wenn kein `origin` Remote konfiguriert ist                                                                                                    |
| `cost.total_cost_usd`                                                            | Geschätzte Sitzungskosten in USD, berechnet auf der Client-Seite. Kann sich von Ihrer tatsächlichen Rechnung unterscheiden                                                                                                                                                                                             |
| `cost.total_duration_ms`                                                         | Gesamtverstrichene Zeit seit Sitzungsbeginn in Millisekunden                                                                                                                                                                                                                                                           |
| `cost.total_api_duration_ms`                                                     | Gesamtzeit, die auf API-Antworten wartet, in Millisekunden                                                                                                                                                                                                                                                             |
| `cost.total_lines_added`, `cost.total_lines_removed`                             | Geänderte Codezeilen                                                                                                                                                                                                                                                                                                   |
| `context_window.total_input_tokens`, `context_window.total_output_tokens`        | Token-Zählungen, die sich derzeit im Kontextfenster befinden, aus der letzten API-Antwort. Die Eingabe umfasst Cache-Lesevorgänge und -Schreibvorgänge. Vor v2.1.132 waren diese kumulative Sitzungssummen                                                                                                             |
| `context_window.context_window_size`                                             | Maximale Kontextfenstergröße in Token. Standardmäßig 200.000 oder 1.000.000 für Modelle mit erweitertem Kontext.                                                                                                                                                                                                       |
| `context_window.used_percentage`                                                 | Vorberechneter Prozentsatz der Kontextfensternutzung                                                                                                                                                                                                                                                                   |
| `context_window.remaining_percentage`                                            | Vorberechneter Prozentsatz des verbleibenden Kontextfensters                                                                                                                                                                                                                                                           |
| `context_window.current_usage`                                                   | Token-Zählungen aus dem letzten API-Aufruf, beschrieben in [Kontextfenster-Felder](#context-window-fields)                                                                                                                                                                                                             |
| `exceeds_200k_tokens`                                                            | Ob die Gesamttoken-Zählung (Eingabe-, Cache- und Ausgabe-Token kombiniert) aus der letzten API-Antwort 200k überschreitet. Dies ist ein fester Schwellenwert unabhängig von der tatsächlichen Kontextfenstergröße.                                                                                                     |
| `effort.level`                                                                   | Aktueller Reasoning-Aufwand (`low`, `medium`, `high`, `xhigh` oder `max`). Spiegelt den Live-Sitzungswert wider, einschließlich Änderungen von `/effort` während der Sitzung. Ultracode ist keine separate Stufe und wird als `xhigh` gemeldet. Fehlt, wenn das aktuelle Modell den Effort-Parameter nicht unterstützt |
| `thinking.enabled`                                                               | Ob erweitertes Denken für die Sitzung aktiviert ist                                                                                                                                                                                                                                                                    |
| `rate_limits.five_hour.used_percentage`, `rate_limits.seven_day.used_percentage` | Prozentsatz des 5-Stunden- oder 7-Tage-Ratenlimits verbraucht, von 0 bis 100                                                                                                                                                                                                                                           |
| `rate_limits.five_hour.resets_at`, `rate_limits.seven_day.resets_at`             | Unix-Epochensekunden, wenn das 5-Stunden- oder 7-Tage-Ratenlimit-Fenster zurückgesetzt wird                                                                                                                                                                                                                            |
| `session_id`                                                                     | Eindeutige Sitzungskennung                                                                                                                                                                                                                                                                                             |
| `session_name`                                                                   | Benutzerdefinierter Sitzungsname, der mit dem `--name` Flag oder `/rename` gesetzt wurde. Fehlt, wenn kein benutzerdefinierter Name gesetzt wurde                                                                                                                                                                      |
| `prompt_id`                                                                      | UUID, die die Benutzereingabe identifiziert, die gerade verarbeitet wird. Entspricht dem [`prompt.id` Attribut auf OpenTelemetry-Ereignissen](/docs/de/monitoring-usage#event-correlation-attributes). Fehlt bis zur ersten Benutzereingabe. Erfordert Claude Code v2.1.196 oder später                                     |
| `transcript_path`                                                                | Pfad zur Gesprächstranskriptdatei                                                                                                                                                                                                                                                                                      |
| `version`                                                                        | Claude Code-Version                                                                                                                                                                                                                                                                                                    |
| `output_style.name`                                                              | Name des aktuellen Ausgabestils                                                                                                                                                                                                                                                                                        |
| `vim.mode`                                                                       | Aktueller Vim-Modus (`NORMAL`, `INSERT`, `VISUAL` oder `VISUAL LINE`), wenn [Vim-Modus](/docs/de/interactive-mode#vim-editor-mode) aktiviert ist                                                                                                                                                                            |
| `agent.name`                                                                     | Agent-Name bei Ausführung mit dem `--agent` Flag oder konfigurierter Agent-Einstellung                                                                                                                                                                                                                                 |
| `pr.number`, `pr.url`                                                            | Offener Pull Request für den aktuellen Branch. Spiegelt das PR-Badge in der unteren Statusleiste wider. Fehlt, bis ein PR gefunden wird, wenn nicht in einem Git-Repository, oder sobald der PR zusammengeführt oder geschlossen wird                                                                                  |
| `pr.review_state`                                                                | Review-Status des offenen PR: `approved`, `pending`, `changes_requested` oder `draft`. Kann unabhängig fehlen, auch wenn `pr` vorhanden ist                                                                                                                                                                            |
| `worktree.name`                                                                  | Name des aktiven Worktree. Nur während `--worktree` Sitzungen vorhanden                                                                                                                                                                                                                                                |
| `worktree.path`                                                                  | Absoluter Pfad zum Worktree-Verzeichnis                                                                                                                                                                                                                                                                                |
| `worktree.branch`                                                                | Git-Branch-Name für den Worktree (zum Beispiel `"worktree-my-feature"`). Fehlt bei Hook-basierten Worktrees                                                                                                                                                                                                            |
| `worktree.original_cwd`                                                          | Das Verzeichnis, in dem Claude sich befand, bevor es den Worktree betrat                                                                                                                                                                                                                                               |
| `worktree.original_branch`                                                       | Git-Branch, der vor dem Betreten des Worktree ausgecheckt wurde. Fehlt bei Hook-basierten Worktrees                                                                                                                                                                                                                    |

<Accordion title="Vollständiges JSON-Schema">
  Ihr Statuszeilen-Befehl empfängt diese JSON-Struktur über stdin:

  ```json theme={null}
  {
    "cwd": "/current/working/directory",
    "session_id": "abc123...",
    "session_name": "my-session",
    "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
    "transcript_path": "/path/to/transcript.jsonl",
    "model": {
      "id": "claude-opus-4-8",
      "display_name": "Opus"
    },
    "workspace": {
      "current_dir": "/current/working/directory",
      "project_dir": "/original/project/directory",
      "added_dirs": [],
      "git_worktree": "feature-xyz",
      "repo": {
        "host": "github.com",
        "owner": "anthropics",
        "name": "claude-code"
      }
    },
    "version": "2.1.90",
    "output_style": {
      "name": "default"
    },
    "cost": {
      "total_cost_usd": 0.01234,
      "total_duration_ms": 45000,
      "total_api_duration_ms": 2300,
      "total_lines_added": 156,
      "total_lines_removed": 23
    },
    "context_window": {
      "total_input_tokens": 15500,
      "total_output_tokens": 1200,
      "context_window_size": 200000,
      "used_percentage": 8,
      "remaining_percentage": 92,
      "current_usage": {
        "input_tokens": 8500,
        "output_tokens": 1200,
        "cache_creation_input_tokens": 5000,
        "cache_read_input_tokens": 2000
      }
    },
    "exceeds_200k_tokens": false,
    "effort": {
      "level": "high"
    },
    "thinking": {
      "enabled": true
    },
    "rate_limits": {
      "five_hour": {
        "used_percentage": 23.5,
        "resets_at": 1738425600
      },
      "seven_day": {
        "used_percentage": 41.2,
        "resets_at": 1738857600
      }
    },
    "vim": {
      "mode": "NORMAL"
    },
    "agent": {
      "name": "security-reviewer"
    },
    "pr": {
      "number": 1234,
      "url": "https://github.com/anthropics/claude-code/pull/1234",
      "review_state": "pending"
    },
    "worktree": {
      "name": "my-feature",
      "path": "/path/to/.claude/worktrees/my-feature",
      "branch": "worktree-my-feature",
      "original_cwd": "/path/to/project",
      "original_branch": "main"
    }
  }
  ```

  **Felder, die möglicherweise fehlen** (nicht in JSON vorhanden):

  * `session_name`: erscheint nur, wenn ein benutzerdefinierter Name mit `--name` oder `/rename` gesetzt wurde
  * `prompt_id`: erscheint nur nach der ersten Benutzereingabe
  * `workspace.git_worktree`: erscheint nur, wenn sich das aktuelle Verzeichnis in einem verknüpften Git-Worktree befindet
  * `workspace.repo`: erscheint nur in einem Git-Repository mit einem konfigurierten `origin` Remote
  * `effort`: erscheint nur, wenn das aktuelle Modell den Reasoning-Effort-Parameter unterstützt
  * `vim`: erscheint nur, wenn Vim-Modus aktiviert ist
  * `agent`: erscheint nur bei Ausführung mit dem `--agent` Flag oder konfigurierter Agent-Einstellung
  * `pr`: erscheint nur, während ein offener PR für den aktuellen Branch gefunden wird, und wird entfernt, sobald der PR zusammengeführt oder geschlossen wird. `pr.review_state` kann unabhängig fehlen
  * `worktree`: erscheint nur während `--worktree` Sitzungen. Wenn vorhanden, können `branch` und `original_branch` auch bei Hook-basierten Worktrees fehlen
  * `rate_limits`: erscheint nur für Claude.ai-Abonnenten (Pro/Max) nach der ersten API-Antwort in der Sitzung. Jedes Fenster (`five_hour`, `seven_day`) kann unabhängig fehlen. Verwenden Sie `jq -r '.rate_limits.five_hour.used_percentage // empty'`, um Abwesenheit elegant zu handhaben.

  **Felder, die `null` sein können**:

  * `context_window.current_usage`: `null` vor dem ersten API-Aufruf in einer Sitzung und erneut nach `/compact`, bis der nächste API-Aufruf es erneut füllt
  * `context_window.used_percentage`, `context_window.remaining_percentage`: können früh in der Sitzung `null` sein

  Behandeln Sie fehlende Felder mit bedingtem Zugriff und Null-Werte mit Fallback-Standardwerten in Ihren Skripten.
</Accordion>

<h3 id="context-window-fields">
  Kontextfenster-Felder
</h3>

Das `context_window` Objekt beschreibt das Live-Kontextfenster aus der letzten API-Antwort. Ab v2.1.132 spiegeln `total_input_tokens` und `total_output_tokens` die aktuelle Kontextnutzung wider, nicht kumulative Sitzungssummen.

* **Kombinierte Summen** (`total_input_tokens`, `total_output_tokens`): Token, die sich derzeit im Kontextfenster befinden. `total_input_tokens` ist die Summe von `input_tokens`, `cache_creation_input_tokens` und `cache_read_input_tokens`; `total_output_tokens` sind die Ausgabe-Token aus der letzten Antwort. Beide sind `0` vor der ersten API-Antwort.
* **Pro-Komponenten-Nutzung** (`current_usage`): die gleichen Token-Zählungen nach Kategorie aufgeschlüsselt. Verwenden Sie dies, wenn Sie Cache-Treffer separat von frischer Eingabe benötigen.

Das `current_usage` Objekt enthält:

* `input_tokens`: Eingabe-Token im aktuellen Kontext
* `output_tokens`: generierte Ausgabe-Token
* `cache_creation_input_tokens`: Token, die in den Cache geschrieben wurden
* `cache_read_input_tokens`: Token, die aus dem Cache gelesen wurden

Für die Bedeutung der Cache-Felder und deren Abrechnung siehe [Cache-Leistung überprüfen](/docs/de/prompt-caching#check-cache-performance).

Das `used_percentage` Feld wird nur aus Eingabe-Token berechnet: `input_tokens + cache_creation_input_tokens + cache_read_input_tokens`. Es enthält keine `output_tokens`.

Wenn Sie den Kontextprozentsatz manuell aus `current_usage` berechnen, verwenden Sie die gleiche Eingabe-only-Formel, um `used_percentage` zu entsprechen.

Das `current_usage` Objekt ist `null` vor dem ersten API-Aufruf in einer Sitzung und erneut unmittelbar nach `/compact`, bis der nächste API-Aufruf es erneut füllt.

<h2 id="examples">
  Beispiele
</h2>

Diese Beispiele zeigen häufige Statuszeilen-Muster. Um ein Beispiel zu verwenden:

1. Speichern Sie das Skript in einer Datei wie `~/.claude/statusline.sh` (oder `.py`/`.js`)
2. Machen Sie es ausführbar: `chmod +x ~/.claude/statusline.sh`
3. Fügen Sie den Pfad zu Ihren [Einstellungen](#manually-configure-a-status-line) hinzu

Die Bash-Beispiele verwenden [`jq`](https://jqlang.github.io/jq/) zum Analysieren von JSON. Python und Node.js haben integriertes JSON-Parsing.

<h3 id="context-window-usage">
  Kontextfensternutzung
</h3>

Zeigen Sie das aktuelle Modell und die Kontextfensternutzung mit einem visuellen Fortschrittsbalken an. Jedes Skript liest JSON von stdin, extrahiert das `used_percentage` Feld und erstellt einen 10-Zeichen-Balken, wobei gefüllte Blöcke (▓) die Nutzung darstellen:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-context-window-usage.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=15b58ab3602f036939145dde3165c6f7" alt="Eine Statuszeile, die Modellname und einen Fortschrittsbalken mit Prozentsatz anzeigt" width="448" height="152" data-path="images/statusline-context-window-usage.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  # Read all of stdin into a variable
  input=$(cat)

  # Extract fields with jq, "// 0" provides fallback for null
  MODEL=$(echo "$input" | jq -r '.model.display_name')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

  # Build progress bar: printf -v creates a run of spaces, then
  # ${var// /▓} replaces each space with a block character
  BAR_WIDTH=10
  FILLED=$((PCT * BAR_WIDTH / 100))
  EMPTY=$((BAR_WIDTH - FILLED))
  BAR=""
  [ "$FILLED" -gt 0 ] && printf -v FILL "%${FILLED}s" && BAR="${FILL// /▓}"
  [ "$EMPTY" -gt 0 ] && printf -v PAD "%${EMPTY}s" && BAR="${BAR}${PAD// /░}"

  echo "[$MODEL] $BAR $PCT%"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  # json.load reads and parses stdin in one step
  data = json.load(sys.stdin)
  model = data['model']['display_name']
  # "or 0" handles null values
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)

  # String multiplication builds the bar
  filled = pct * 10 // 100
  bar = '▓' * filled + '░' * (10 - filled)

  print(f"[{model}] {bar} {pct}%")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  // Node.js reads stdin asynchronously with events
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      // Optional chaining (?.) safely handles null fields
      const pct = Math.floor(data.context_window?.used_percentage || 0);

      // String.repeat() builds the bar
      const filled = Math.floor(pct * 10 / 100);
      const bar = '▓'.repeat(filled) + '░'.repeat(10 - filled);

      console.log(`[${model}] ${bar} ${pct}%`);
  });
  ```
</CodeGroup>

<h3 id="git-status-with-colors">
  Git-Status mit Farben
</h3>

Zeigen Sie Git-Branch mit farbcodierten Indikatoren für bereitgestellte und geänderte Dateien an. Dieses Skript verwendet [ANSI-Escape-Codes](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) für Terminalfarben: `\033[32m` ist Grün, `\033[33m` ist Gelb und `\033[0m` setzt auf Standard zurück.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-git-context.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e656f34f90d1d9a1d0e220988914345f" alt="Eine Statuszeile, die Modell, Verzeichnis, Git-Branch und farbcodierte Indikatoren für bereitgestellte und geänderte Dateien anzeigt" width="742" height="178" data-path="images/statusline-git-context.png" />
</Frame>

Jedes Skript prüft, ob das aktuelle Verzeichnis ein Git-Repository ist, zählt bereitgestellte und geänderte Dateien und zeigt farbcodierte Indikatoren an:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')

  GREEN='\033[32m'
  YELLOW='\033[33m'
  RESET='\033[0m'

  if git rev-parse --git-dir > /dev/null 2>&1; then
      BRANCH=$(git branch --show-current 2>/dev/null)
      STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
      MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')

      GIT_STATUS=""
      [ "$STAGED" -gt 0 ] && GIT_STATUS="${GREEN}+${STAGED}${RESET}"
      [ "$MODIFIED" -gt 0 ] && GIT_STATUS="${GIT_STATUS}${YELLOW}~${MODIFIED}${RESET}"

      echo -e "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH $GIT_STATUS"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])

  GREEN, YELLOW, RESET = '\033[32m', '\033[33m', '\033[0m'

  try:
      subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
      staged_output = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
      modified_output = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
      staged = len(staged_output.split('\n')) if staged_output else 0
      modified = len(modified_output.split('\n')) if modified_output else 0

      git_status = f"{GREEN}+{staged}{RESET}" if staged else ""
      git_status += f"{YELLOW}~{modified}{RESET}" if modified else ""

      print(f"[{model}] 📁 {directory} | 🌿 {branch} {git_status}")
  except:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);

      const GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RESET = '\x1b[0m';

      try {
          execSync('git rev-parse --git-dir', { stdio: 'ignore' });
          const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
          const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
          const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;

          let gitStatus = staged ? `${GREEN}+${staged}${RESET}` : '';
          gitStatus += modified ? `${YELLOW}~${modified}${RESET}` : '';

          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} ${gitStatus}`);
      } catch {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="cost-and-duration-tracking">
  Kosten- und Dauer-Verfolgung
</h3>

Verfolgen Sie die API-Kosten und verstrichene Zeit Ihrer Sitzung. Das `cost.total_cost_usd` Feld sammelt die geschätzten Kosten aller API-Aufrufe in der aktuellen Sitzung. Das `cost.total_duration_ms` Feld misst die Gesamtverstrichene Zeit seit Sitzungsbeginn, während `cost.total_api_duration_ms` nur die Zeit verfolgt, die auf API-Antworten wartet.

Jedes Skript formatiert Kosten als Währung und konvertiert Millisekunden in Minuten und Sekunden:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-cost-tracking.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e3444a51fe6f3440c134bd5f1f08ad29" alt="Eine Statuszeile, die Modellname, Sitzungskosten und Dauer anzeigt" width="588" height="180" data-path="images/statusline-cost-tracking.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  COST_FMT=$(printf '$%.2f' "$COST")
  DURATION_SEC=$((DURATION_MS / 1000))
  MINS=$((DURATION_SEC / 60))
  SECS=$((DURATION_SEC % 60))

  echo "[$MODEL] 💰 $COST_FMT | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  duration_sec = duration_ms // 1000
  mins, secs = duration_sec // 60, duration_sec % 60

  print(f"[{model}] 💰 ${cost:.2f} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const cost = data.cost?.total_cost_usd || 0;
      const durationMs = data.cost?.total_duration_ms || 0;

      const durationSec = Math.floor(durationMs / 1000);
      const mins = Math.floor(durationSec / 60);
      const secs = durationSec % 60;

      console.log(`[${model}] 💰 $${cost.toFixed(2)} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="display-multiple-lines">
  Mehrere Zeilen anzeigen
</h3>

Ihr Skript kann mehrere Zeilen ausgeben, um eine reichhaltigere Anzeige zu erstellen. Jede `echo` Anweisung erzeugt eine separate Zeile im Statusbereich.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Eine mehrzeilige Statuszeile, die Modellname, Verzeichnis, Git-Branch in der ersten Zeile und einen Kontextnutzungs-Fortschrittsbalken mit Kosten und Dauer in der zweiten Zeile anzeigt" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Dieses Beispiel kombiniert mehrere Techniken: schwellenwertbasierte Farben (Grün unter 70%, Gelb 70-89%, Rot 90%+), einen Fortschrittsbalken und Git-Branch-Informationen. Jede `print` oder `echo` Anweisung erstellt eine separate Zeile:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  CYAN='\033[36m'; GREEN='\033[32m'; YELLOW='\033[33m'; RED='\033[31m'; RESET='\033[0m'

  # Pick bar color based on context usage
  if [ "$PCT" -ge 90 ]; then BAR_COLOR="$RED"
  elif [ "$PCT" -ge 70 ]; then BAR_COLOR="$YELLOW"
  else BAR_COLOR="$GREEN"; fi

  FILLED=$((PCT / 10)); EMPTY=$((10 - FILLED))
  printf -v FILL "%${FILLED}s"; printf -v PAD "%${EMPTY}s"
  BAR="${FILL// /█}${PAD// /░}"

  MINS=$((DURATION_MS / 60000)); SECS=$(((DURATION_MS % 60000) / 1000))

  BRANCH=""
  git rev-parse --git-dir > /dev/null 2>&1 && BRANCH=" | 🌿 $(git branch --show-current 2>/dev/null)"

  echo -e "${CYAN}[$MODEL]${RESET} 📁 ${DIR##*/}$BRANCH"
  COST_FMT=$(printf '$%.2f' "$COST")
  echo -e "${BAR_COLOR}${BAR}${RESET} ${PCT}% | ${YELLOW}${COST_FMT}${RESET} | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  CYAN, GREEN, YELLOW, RED, RESET = '\033[36m', '\033[32m', '\033[33m', '\033[31m', '\033[0m'

  bar_color = RED if pct >= 90 else YELLOW if pct >= 70 else GREEN
  filled = pct // 10
  bar = '█' * filled + '░' * (10 - filled)

  mins, secs = duration_ms // 60000, (duration_ms % 60000) // 1000

  try:
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True, stderr=subprocess.DEVNULL).strip()
      branch = f" | 🌿 {branch}" if branch else ""
  except:
      branch = ""

  print(f"{CYAN}[{model}]{RESET} 📁 {directory}{branch}")
  print(f"{bar_color}{bar}{RESET} {pct}% | {YELLOW}${cost:.2f}{RESET} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const cost = data.cost?.total_cost_usd || 0;
      const pct = Math.floor(data.context_window?.used_percentage || 0);
      const durationMs = data.cost?.total_duration_ms || 0;

      const CYAN = '\x1b[36m', GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RED = '\x1b[31m', RESET = '\x1b[0m';

      const barColor = pct >= 90 ? RED : pct >= 70 ? YELLOW : GREEN;
      const filled = Math.floor(pct / 10);
      const bar = '█'.repeat(filled) + '░'.repeat(10 - filled);

      const mins = Math.floor(durationMs / 60000);
      const secs = Math.floor((durationMs % 60000) / 1000);

      let branch = '';
      try {
          branch = execSync('git branch --show-current', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          branch = branch ? ` | 🌿 ${branch}` : '';
      } catch {}

      console.log(`${CYAN}[${model}]${RESET} 📁 ${dir}${branch}`);
      console.log(`${barColor}${bar}${RESET} ${pct}% | ${YELLOW}$${cost.toFixed(2)}${RESET} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="clickable-links">
  Anklickbare Links
</h3>

Dieses Beispiel erstellt einen anklickbaren Link zu Ihrem GitHub-Repository. Es liest die Git-Remote-URL, konvertiert das SSH-Format mit `sed` in HTTPS und umhüllt den Repository-Namen mit OSC 8 Escape-Codes. Halten Sie Cmd (macOS) oder Strg (Windows/Linux) gedrückt und klicken Sie, um den Link in Ihrem Browser zu öffnen.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-links.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=4bcc6e7deb7cf52f41ab85a219b52661" alt="Eine Statuszeile, die einen anklickbaren Link zu einem GitHub-Repository anzeigt" width="726" height="198" data-path="images/statusline-links.png" />
</Frame>

Jedes Skript ruft die Git-Remote-URL ab, konvertiert das SSH-Format in HTTPS und umhüllt den Repository-Namen mit OSC 8 Escape-Codes. Die Bash-Version verwendet `printf '%b'`, das Backslash-Escapes zuverlässiger interpretiert als `echo -e` über verschiedene Shells hinweg:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')

  # Convert git SSH URL to HTTPS
  REMOTE=$(git remote get-url origin 2>/dev/null | sed 's/git@github.com:/https:\/\/github.com\//' | sed 's/\.git$//')

  if [ -n "$REMOTE" ]; then
      REPO_NAME=$(basename "$REMOTE")
      # OSC 8 format: \e]8;;URL\a then TEXT then \e]8;;\a
      # printf %b interprets escape sequences reliably across shells
      printf '%b' "[$MODEL] 🔗 \e]8;;${REMOTE}\a${REPO_NAME}\e]8;;\a\n"
  else
      echo "[$MODEL]"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, re, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  # Get git remote URL
  try:
      remote = subprocess.check_output(
          ['git', 'remote', 'get-url', 'origin'],
          stderr=subprocess.DEVNULL, text=True
      ).strip()
      # Convert SSH to HTTPS format
      remote = re.sub(r'^git@github\.com:', 'https://github.com/', remote)
      remote = re.sub(r'\.git$', '', remote)
      repo_name = os.path.basename(remote)
      # OSC 8 escape sequences
      link = f"\033]8;;{remote}\a{repo_name}\033]8;;\a"
      print(f"[{model}] 🔗 {link}")
  except:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      try {
          let remote = execSync('git remote get-url origin', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          // Convert SSH to HTTPS format
          remote = remote.replace(/^git@github\.com:/, 'https://github.com/').replace(/\.git$/, '');
          const repoName = path.basename(remote);
          // OSC 8 escape sequences
          const link = `\x1b]8;;${remote}\x07${repoName}\x1b]8;;\x07`;
          console.log(`[${model}] 🔗 ${link}`);
      } catch {
          console.log(`[${model}]`);
      }
  });
  ```
</CodeGroup>

<h3 id="rate-limit-usage">
  Ratenlimit-Nutzung
</h3>

Zeigen Sie die Ratenlimit-Nutzung des Claude.ai-Abonnements in der Statuszeile an. Das `rate_limits` Objekt enthält `five_hour` (5-Stunden-Rollenfenster) und `seven_day` (wöchliche) Fenster. Jedes Fenster bietet `used_percentage` (0-100) und `resets_at` (Unix-Epochensekunden, wenn das Fenster zurückgesetzt wird).

Dieses Feld ist nur für Claude.ai-Abonnenten (Pro/Max) nach der ersten API-Antwort vorhanden. Jedes Skript behandelt das fehlende Feld elegant:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  # "// empty" produces no output when rate_limits is absent
  FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
  WEEK=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')

  LIMITS=""
  [ -n "$FIVE_H" ] && LIMITS="5h: $(printf '%.0f' "$FIVE_H")%"
  [ -n "$WEEK" ] && LIMITS="${LIMITS:+$LIMITS }7d: $(printf '%.0f' "$WEEK")%"

  [ -n "$LIMITS" ] && echo "[$MODEL] | $LIMITS" || echo "[$MODEL]"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  parts = []
  rate = data.get('rate_limits', {})
  five_h = rate.get('five_hour', {}).get('used_percentage')
  week = rate.get('seven_day', {}).get('used_percentage')

  if five_h is not None:
      parts.append(f"5h: {five_h:.0f}%")
  if week is not None:
      parts.append(f"7d: {week:.0f}%")

  if parts:
      print(f"[{model}] | {' '.join(parts)}")
  else:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      const parts = [];
      const fiveH = data.rate_limits?.five_hour?.used_percentage;
      const week = data.rate_limits?.seven_day?.used_percentage;

      if (fiveH != null) parts.push(`5h: ${Math.round(fiveH)}%`);
      if (week != null) parts.push(`7d: ${Math.round(week)}%`);

      console.log(parts.length ? `[${model}] | ${parts.join(' ')}` : `[${model}]`);
  });
  ```
</CodeGroup>

<h3 id="cache-expensive-operations">
  Teure Operationen zwischenspeichern
</h3>

Ihr Statuszeilen-Skript wird während aktiver Sitzungen häufig ausgeführt. Befehle wie `git status` oder `git diff` können langsam sein, besonders in großen Repositories. Dieses Beispiel speichert Git-Informationen in einer temporären Datei und aktualisiert sie nur alle 5 Sekunden.

Der Cache-Dateiname muss über Statuszeilen-Invokationen innerhalb einer Sitzung stabil sein, aber eindeutig über Sitzungen hinweg, damit gleichzeitige Sitzungen in verschiedenen Repositories nicht den Cache-Git-Status des anderen lesen. Prozessbasierte Identifikatoren wie `$$`, `os.getpid()` oder `process.pid` ändern sich bei jeder Invokation und besiegen den Cache. Verwenden Sie stattdessen die `session_id` aus der JSON-Eingabe: Sie ist stabil für die Lebensdauer einer Sitzung und eindeutig pro Sitzung.

Jedes Skript prüft, ob die Cache-Datei fehlt oder älter als 5 Sekunden ist, bevor Git-Befehle ausgeführt werden:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  SESSION_ID=$(echo "$input" | jq -r '.session_id')

  CACHE_FILE="/tmp/statusline-git-cache-$SESSION_ID"
  CACHE_MAX_AGE=5  # seconds

  cache_is_stale() {
      [ ! -f "$CACHE_FILE" ] || \
      # stat -f %m is macOS, stat -c %Y is Linux
      [ $(($(date +%s) - $(stat -f %m "$CACHE_FILE" 2>/dev/null || stat -c %Y "$CACHE_FILE" 2>/dev/null || echo 0))) -gt $CACHE_MAX_AGE ]
  }

  if cache_is_stale; then
      if git rev-parse --git-dir > /dev/null 2>&1; then
          BRANCH=$(git branch --show-current 2>/dev/null)
          STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
          MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')
          echo "$BRANCH|$STAGED|$MODIFIED" > "$CACHE_FILE"
      else
          echo "||" > "$CACHE_FILE"
      fi
  fi

  IFS='|' read -r BRANCH STAGED MODIFIED < "$CACHE_FILE"

  if [ -n "$BRANCH" ]; then
      echo "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH +$STAGED ~$MODIFIED"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os, time

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  session_id = data['session_id']

  CACHE_FILE = f"/tmp/statusline-git-cache-{session_id}"
  CACHE_MAX_AGE = 5  # seconds

  def cache_is_stale():
      if not os.path.exists(CACHE_FILE):
          return True
      return time.time() - os.path.getmtime(CACHE_FILE) > CACHE_MAX_AGE

  if cache_is_stale():
      try:
          subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
          branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
          staged = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
          modified = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
          staged_count = len(staged.split('\n')) if staged else 0
          modified_count = len(modified.split('\n')) if modified else 0
          with open(CACHE_FILE, 'w') as f:
              f.write(f"{branch}|{staged_count}|{modified_count}")
      except:
          with open(CACHE_FILE, 'w') as f:
              f.write("||")

  with open(CACHE_FILE) as f:
      branch, staged, modified = f.read().strip().split('|')

  if branch:
      print(f"[{model}] 📁 {directory} | 🌿 {branch} +{staged} ~{modified}")
  else:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const fs = require('fs');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const sessionId = data.session_id;

      const CACHE_FILE = `/tmp/statusline-git-cache-${sessionId}`;
      const CACHE_MAX_AGE = 5; // seconds

      const cacheIsStale = () => {
          if (!fs.existsSync(CACHE_FILE)) return true;
          return (Date.now() / 1000) - fs.statSync(CACHE_FILE).mtimeMs / 1000 > CACHE_MAX_AGE;
      };

      if (cacheIsStale()) {
          try {
              execSync('git rev-parse --git-dir', { stdio: 'ignore' });
              const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
              const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              fs.writeFileSync(CACHE_FILE, `${branch}|${staged}|${modified}`);
          } catch {
              fs.writeFileSync(CACHE_FILE, '||');
          }
      }

      const [branch, staged, modified] = fs.readFileSync(CACHE_FILE, 'utf8').trim().split('|');

      if (branch) {
          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} +${staged} ~${modified}`);
      } else {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="windows-configuration">
  Windows-Konfiguration
</h3>

Unter Windows führt Claude Code Statuszeilen-Befehle über Git Bash aus, wenn Git Bash installiert ist, oder über PowerShell, wenn Git Bash nicht vorhanden ist.

Git Bash behandelt unquotierte Backslashes als Escape-Zeichen, daher erreicht ein Windows-Pfad wie `C:\Users\username\script.mjs` den Skript-Runner mit entfernten Trennzeichen und der Befehl schlägt ohne sichtbaren Fehler fehl. Schreiben Sie Dateipfade in der `command` Zeichenkette mit Schrägstrichen, wie in den folgenden Beispielen gezeigt. Die `~` Abkürzung funktioniert auch und wird zu Ihrem Windows-Basisverzeichnis erweitert.

Um ein PowerShell-Skript als Statuszeile auszuführen, rufen Sie es über `powershell` auf. Dies funktioniert, ob Claude Code den Befehl über Git Bash oder PowerShell leitet:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "powershell -NoProfile -File C:/Users/username/.claude/statusline.ps1"
    }
  }
  ```

  ```powershell statusline.ps1 theme={null}
  $input_json = $input | Out-String | ConvertFrom-Json
  $cwd = $input_json.cwd
  $model = $input_json.model.display_name
  $used = $input_json.context_window.used_percentage
  $dirname = Split-Path $cwd -Leaf

  if ($used) {
      Write-Host "$dirname [$model] ctx: $used%"
  } else {
      Write-Host "$dirname [$model]"
  }
  ```
</CodeGroup>

Oder führen Sie ein Bash-Skript direkt aus:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "~/.claude/statusline.sh"
    }
  }
  ```

  ```bash statusline.sh theme={null}
  #!/usr/bin/env bash
  input=$(cat)
  cwd=$(echo "$input" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
  model=$(echo "$input" | grep -o '"display_name":"[^"]*"' | cut -d'"' -f4)
  dirname="${cwd##*[/\\]}"
  echo "$dirname [$model]"
  ```
</CodeGroup>

<h2 id="subagent-status-lines">
  Subagent-Statuszeilen
</h2>

Die `subagentStatusLine` Einstellung rendert einen benutzerdefinierten Zeilenkörper für jeden [Subagenten](/docs/de/sub-agents), der im Agent-Panel unterhalb der Eingabeaufforderung angezeigt wird. Verwenden Sie sie, um die Standard-`name · description · token count` Zeile durch Ihre eigene Formatierung zu ersetzen.

```json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "~/.claude/subagent-statusline.sh"
  }
}
```

Der Befehl wird einmal pro Aktualisierungstick mit allen sichtbaren Subagenten-Zeilen ausgeführt, die als einzelnes JSON-Objekt auf stdin übergeben werden. Die Eingabe enthält die [Basis-Hook-Felder](/docs/de/hooks#common-input-fields), ein `columns` Feld mit der nutzbaren Zeilenbreite und ein `tasks` Array. Jede Aufgabe hat `id`, `name`, `type`, `status`, `description`, `label`, `startTime`, `model`, `contextWindowSize`, `tokenCount`, `tokenSamples` und `cwd`.

Das `model` Feld pro Aufgabe ist die aufgelöste Modell-ID, auf der die Aufgabe ausgeführt wird. `contextWindowSize` ist das Kontextfenster dieses Modells in Token, berechnet auf die gleiche Weise wie das `context_window.context_window_size` der Hauptstatuszeile, sodass Sie einen Prozentsatz pro Zeile aus `tokenCount` rendern können. Beide Felder erfordern Claude Code v2.1.205 oder später und werden für eine Aufgabe weggelassen, deren Modell noch nicht aufgelöst ist.

Schreiben Sie eine JSON-Zeile auf stdout pro Zeile, die Sie überschreiben möchten, in der Form `{"id": "<task id>", "content": "<row body>"}`. Der `content` String wird wie vorhanden gerendert, einschließlich ANSI-Farben und OSC 8 Hyperlinks. Lassen Sie die `id` einer Aufgabe weg, um die Standard-Rendering für diese Zeile zu behalten; geben Sie einen leeren `content` String aus, um sie auszublenden.

Die gleichen Vertrauens- und `disableAllHooks` Gates, die für `statusLine` gelten, gelten auch hier. Plugins können eine Standard-`subagentStatusLine` in ihrer [`settings.json`](/docs/de/plugins-reference#standard-plugin-layout) versenden.

<h2 id="tips">
  Tipps
</h2>

* **Mit Mock-Eingabe testen**: `echo '{"model":{"display_name":"Opus"},"workspace":{"current_dir":"/home/user/project"},"context_window":{"used_percentage":25},"session_id":"test-session-abc"}' | ./statusline.sh`
* **Ausgabe kurz halten**: Die Statusleiste hat begrenzte Breite, daher kann lange Ausgabe abgeschnitten oder unangenehm umgebrochen werden
* **Langsame Operationen zwischenspeichern**: Ihr Skript wird während aktiver Sitzungen häufig ausgeführt, daher können Befehle wie `git status` zu Verzögerungen führen. Siehe das [Caching-Beispiel](#cache-expensive-operations) für die Handhabung.

Community-Projekte wie [ccstatusline](https://github.com/sirmalloc/ccstatusline) und [starship-claude](https://github.com/martinemde/starship-claude) bieten vorkonfigurierte Konfigurationen mit Designs und zusätzlichen Funktionen.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

**Statuszeile wird nicht angezeigt**

* Überprüfen Sie, dass Ihr Skript ausführbar ist: `chmod +x ~/.claude/statusline.sh`
* Überprüfen Sie, dass Ihr Skript auf stdout ausgibt, nicht auf stderr
* Führen Sie Ihr Skript manuell aus, um zu überprüfen, dass es Ausgabe erzeugt
* Unter Windows mit installiertem Git Bash werden Backslashes im `command`-Pfad wahrscheinlich als Escape-Zeichen verarbeitet, bevor das Skript ausgeführt wird. Verwenden Sie Schrägstriche im Pfad. Siehe [Windows-Konfiguration](#windows-configuration).
* Wenn `disableAllHooks` in Ihren Einstellungen auf `true` gesetzt ist, ist die Statuszeile auch deaktiviert. Entfernen Sie diese Einstellung oder setzen Sie sie auf `false`, um sie erneut zu aktivieren.
* Führen Sie `claude --debug` aus, um den Exit-Code und stderr aus der ersten Statuszeilen-Invokation in einer Sitzung zu protokollieren
* Bitten Sie Claude, Ihre Einstellungsdatei zu lesen und den `statusLine`-Befehl direkt auszuführen, um Fehler zu finden

**Statuszeile zeigt `--` oder leere Werte**

* Felder können `null` sein, bevor die erste API-Antwort abgeschlossen ist
* Behandeln Sie Null-Werte in Ihrem Skript mit Fallbacks wie `// 0` in jq
* Starten Sie Claude Code neu, wenn Werte nach mehreren Nachrichten leer bleiben

**Kontextprozentsatz zeigt unerwartete Werte**

* Verwenden Sie `used_percentage` für den einfachsten genauen Kontextzustand
* Der Kontextprozentsatz kann sich von der `/context`-Ausgabe unterscheiden, je nachdem, wann jeder berechnet wird

**OSC 8-Links nicht anklickbar**

* Überprüfen Sie, dass Ihr Terminal OSC 8-Hyperlinks unterstützt (iTerm2, Kitty, WezTerm)

* Terminal.app unterstützt keine anklickbaren Links

* Wenn Link-Text angezeigt wird, aber nicht anklickbar ist, hat Claude Code möglicherweise keine Hyperlink-Unterstützung in Ihrem Terminal erkannt. Dies betrifft häufig Windows Terminal und andere Emulatoren, die nicht in der Auto-Erkennungsliste enthalten sind. Setzen Sie die `FORCE_HYPERLINK`-Umgebungsvariable, um die Erkennung zu überschreiben, bevor Sie Claude Code starten:

  ```bash theme={null}
  FORCE_HYPERLINK=1 claude
  ```

  In PowerShell setzen Sie die Variable zuerst in der aktuellen Sitzung:

  ```powershell theme={null}
  $env:FORCE_HYPERLINK = "1"; claude
  ```

* SSH- und tmux-Sitzungen können OSC-Sequenzen je nach Konfiguration entfernen

* Wenn Escape-Sequenzen als Literaltext wie `\e]8;;` angezeigt werden, verwenden Sie `printf '%b'` anstelle von `echo -e` für zuverlässigere Escape-Behandlung

**Anzeigeglitches mit Escape-Sequenzen**

* Komplexe Escape-Sequenzen (ANSI-Farben, OSC 8-Links) können gelegentlich zu beschädigter Ausgabe führen, wenn sie mit anderen UI-Aktualisierungen überlappen
* Wenn Sie beschädigten Text sehen, versuchen Sie, Ihr Skript auf einfache Textausgabe zu vereinfachen
* Mehrzeilige Statuszeilen mit Escape-Codes sind anfälliger für Rendering-Probleme als einzeilige einfache Texte

**Workspace-Vertrauen erforderlich**

* Der Statuszeilen-Befehl wird nur ausgeführt, wenn Sie den Workspace-Vertrauensdialog für das aktuelle Verzeichnis akzeptiert haben. Da `statusLine` einen Shell-Befehl ausführt, erfordert es die gleiche Vertrauensannahme wie Hooks und andere Shell-ausführende Einstellungen.
* Wenn Vertrauen nicht akzeptiert wird, sehen Sie stattdessen die Benachrichtigung `statusline skipped · restart to fix`. Starten Sie Claude Code neu und akzeptieren Sie die Vertrauensaufforderung, um es zu aktivieren.

**Skriptfehler oder Hängen**

* Skripte, die mit Nicht-Null-Codes beendet werden oder keine Ausgabe erzeugen, führen dazu, dass die Statuszeile leer wird
* Langsame Skripte blockieren die Statuszeilen-Aktualisierung, bis sie abgeschlossen sind. Halten Sie Skripte schnell, um veraltete Ausgabe zu vermeiden.
* Wenn eine neue Aktualisierung ausgelöst wird, während ein langsames Skript läuft, wird das laufende Skript abgebrochen
* Testen Sie Ihr Skript unabhängig mit Mock-Eingabe, bevor Sie es konfigurieren

**Benachrichtigungen teilen sich die Statuszeilen-Zeile**

* Systembenachrichtigungen wie MCP-Serverfehler und automatische Updates werden auf der rechten Seite derselben Zeile wie Ihre Statuszeile angezeigt. Vorübergehende Benachrichtigungen wie die Kontext-niedrig-Warnung zirkulieren auch durch diesen Bereich.
* Das Aktivieren des ausführlichen Modus fügt einen Token-Zähler zu diesem Bereich hinzu
* Auf schmalen Terminals können diese Benachrichtigungen Ihre Statuszeilen-Ausgabe abschneiden
