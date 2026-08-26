> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ausgabestile

> Passen Sie Claude Code für Anwendungsfälle über Softwareentwicklung hinaus an

Ausgabestile ändern, wie Claude antwortet, nicht was Claude weiß. Sie ändern die Systemaufforderung, um Rolle, Ton und Ausgabeformat festzulegen. Verwenden Sie einen, wenn Sie sich in jedem Durchgang immer wieder nach derselben Stimme oder demselben Format erkundigen, oder wenn Sie möchten, dass Claude als etwas anderes als ein Softwareentwickler fungiert.

Ein benutzerdefinierter Ausgabestil fügt Ihre Anweisungen zur Systemaufforderung hinzu und lässt Sie wählen, ob Sie die integrierten Softwareentwicklungsanweisungen von Claude Code beibehalten möchten. Behalten Sie sie, wenn Sie ändern, wie Claude kommuniziert, aber immer noch codiert, z. B. immer mit einem Diagramm antwortet. Lassen Sie sie weg, wenn Claude überhaupt keine Softwareentwicklung durchführt, z. B. als Schreib-Assistent oder Datenanalyst.

Für Anweisungen zu Ihrem Projekt, Konventionen oder Ihrer Codebasis verwenden Sie stattdessen [CLAUDE.md](/docs/de/memory).

<h2 id="built-in-output-styles">
  Integrierte Ausgabestile
</h2>

Der **Standard**-Ausgabestil von Claude Code ist die vorhandene Systemaufforderung, die Ihnen helfen soll, Softwareentwicklungsaufgaben effizient zu bewältigen.

Es gibt drei zusätzliche integrierte Ausgabestile:

* **Proaktiv**: Claude führt sofort aus, trifft vernünftige Annahmen statt bei Routineentscheidungen zu pausieren, und bevorzugt Handeln gegenüber Planung. Dies ist eine stärkere Anleitung zur autonomen Ausführung als [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) bietet, und es funktioniert ohne Änderung Ihres Berechtigungsmodus, sodass Sie vor der Ausführung von Tools weiterhin Berechtigungsaufforderungen sehen.

* **Explanatory**: Bietet pädagogische „Insights" zwischen der Unterstützung bei Softwareentwicklungsaufgaben. Hilft Ihnen, Implementierungsentscheidungen und Codebase-Muster zu verstehen.

* **Learning**: Kollaborativer, Lern-durch-Tun-Modus, in dem Claude nicht nur „Insights" beim Codieren teilt, sondern Sie auch auffordert, kleine, strategische Codestücke selbst beizutragen. Claude Code fügt `TODO(human)`-Marker in Ihren Code ein, damit Sie diese implementieren können.

<h2 id="change-your-output-style">
  Ändern Sie Ihren Ausgabestil
</h2>

Führen Sie `/config` aus und wählen Sie **Output style**, um einen Stil aus einem Menü auszuwählen. Ihre Auswahl wird in `.claude/settings.local.json` auf der [lokalen Projektebene](/docs/de/settings) gespeichert.

<Note>Der eigenständige Befehl `/output-style` wurde in v2.1.73 veraltet und in v2.1.91 entfernt. Verwenden Sie `/config` oder bearbeiten Sie die Einstellung `outputStyle` direkt.</Note>

Um einen Stil ohne Menü festzulegen, bearbeiten Sie das Feld `outputStyle` direkt in einer Einstellungsdatei:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

Der Ausgabestil ist Teil der Systemaufforderung, die Claude Code einmal beim Sitzungsstart liest. Änderungen werden nach `/clear` oder einer neuen Sitzung wirksam. Siehe [Wie Claude Code Prompt Caching nutzt](/docs/de/prompt-caching#changing-output-style), um zu erfahren, was eine Änderung des Ausgabestils für den Cache bewirkt.

<h2 id="create-a-custom-output-style">
  Erstellen Sie einen benutzerdefinierten Ausgabestil
</h2>

Ein benutzerdefinierter Ausgabestil ist eine Markdown-Datei: Frontmatter für Metadaten, dann die Anweisungen, die zur Systemaufforderung hinzugefügt werden.

<Steps>
  <Step title="Erstellen Sie eine Markdown-Datei">
    Speichern Sie sie auf einer von drei Ebenen. Der Dateiname wird zum Stilnamen, es sei denn, Sie legen `name` im Frontmatter fest.

    * Benutzer: `~/.claude/output-styles`
    * Projekt: `.claude/output-styles`
    * Verwaltete Richtlinie: `.claude/output-styles` im [Verzeichnis für verwaltete Einstellungen](/docs/de/settings#settings-files)

    Projekt-Ausgabestile werden aus jedem `.claude/output-styles/` zwischen dem Arbeitsverzeichnis und dem Repository-Root geladen. Ab v2.1.178 verwendet Claude Code, wenn mehr als eines dieser verschachtelten Verzeichnisse einen Stil mit demselben Namen definiert, denjenigen, der dem Arbeitsverzeichnis am nächsten ist.
  </Step>

  <Step title="Fügen Sie Frontmatter und Anweisungen hinzu">
    Entscheiden Sie, ob Sie die Softwareentwicklungsanweisungen von Claude Code beibehalten möchten. Setzen Sie `keep-coding-instructions: true`, wenn Sie ändern, wie Claude kommuniziert, aber möchten, dass es auf die gleiche Weise codiert. Lassen Sie es weg, wenn Claude keine Softwareentwicklung durchführt.

    Dieses Beispiel leitet jede Erklärung mit einem Diagramm ein und behält Claudes Codierungsverhalten bei:

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="Wechseln Sie zu Ihrem Stil">
    Führen Sie `/config` aus und wählen Sie Ihren Stil unter **Output style**. Es wird nach `/clear` oder beim nächsten Start einer Sitzung wirksam.
  </Step>
</Steps>

[Plugins](/docs/de/plugins-reference) können auch Ausgabestile in einem `output-styles/`-Verzeichnis bereitstellen.

<h3 id="frontmatter">
  Frontmatter
</h3>

Ausgabestil-Dateien unterstützen diese Frontmatter-Felder:

| Frontmatter                | Zweck                                                                                                                                                                                                                                                                                        | Standard                   |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------- |
| `name`                     | Name des Ausgabestils, falls nicht der Dateiname                                                                                                                                                                                                                                             | Wird vom Dateinamen geerbt |
| `description`              | Beschreibung des Ausgabestils, angezeigt in der `/config`-Auswahl                                                                                                                                                                                                                            | Keine                      |
| `keep-coding-instructions` | Behalten Sie die integrierten Softwareentwicklungsanweisungen von Claude Code                                                                                                                                                                                                                | `false`                    |
| `force-for-plugin`         | Nur Plugin-Ausgabestile: Wenden Sie diesen Stil automatisch an, wenn das Plugin aktiviert ist, ohne dass Benutzer ihn auswählen müssen. Überschreibt die `outputStyle`-Einstellung des Benutzers. Wenn mehrere aktivierte Plugins dies festlegen, verwendet Claude Code das zuerst geladene. | `false`                    |

<h2 id="how-output-styles-work">
  Wie Ausgabestile funktionieren
</h2>

Ausgabestile ändern direkt die Systemaufforderung von Claude Code.

* Alle Ausgabestile haben ihre eigenen benutzerdefinierten Anweisungen am Ende der Systemaufforderung hinzugefügt.
* Alle Ausgabestile lösen Erinnerungen für Claude aus, um die Ausgabestil-Anweisungen während des Gesprächs einzuhalten.
* Benutzerdefinierte Ausgabestile lassen die integrierten Softwareentwicklungsanweisungen von Claude Code weg, z. B. wie man Änderungen begrenzt, Kommentare schreibt und Arbeiten überprüft, es sei denn, `keep-coding-instructions` ist auf `true` gesetzt.

Die Tokennutzung hängt vom Stil ab. Das Hinzufügen von Anweisungen zur Systemaufforderung erhöht die Eingabe-Token, obwohl Prompt Caching diese Kosten nach der ersten Anfrage in einer Sitzung reduziert. Die integrierten Explanatory- und Learning-Stile erzeugen absichtlich längere Antworten als Standard, was die Ausgabe-Token erhöht. Bei benutzerdefinierten Stilen hängt die Tokennutzung für die Ausgabe davon ab, was Ihre Anweisungen Claude zu produzieren sagen.

<h2 id="comparisons-to-related-features">
  Vergleiche mit verwandten Funktionen
</h2>

Mehrere Funktionen passen an, wie sich Claude Code verhält. Ausgabestile ändern die Systemaufforderung direkt und gelten für jede Antwort. Die anderen fügen Anweisungen hinzu, ohne die Standard-Systemaufforderung zu ändern, oder begrenzen sie auf eine bestimmte Aufgabe.

| Funktion                 | Wie es funktioniert                                                                            | Verwenden Sie es, wenn                                                                                      |
| :----------------------- | :--------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| Ausgabestile             | Ändert die Systemaufforderung                                                                  | Sie möchten in jedem Durchgang eine andere Rolle, einen anderen Ton oder ein anderes Standard-Antwortformat |
| [CLAUDE.md](/docs/de/memory)  | Fügt eine Benutzernachricht nach der Systemaufforderung hinzu                                  | Claude sollte immer Ihre Projektkonventionen und Codebase-Kontext kennen                                    |
| `--append-system-prompt` | Hängt an die Systemaufforderung an, ohne etwas zu entfernen                                    | Sie möchten eine einmalige Ergänzung für einen einzelnen Aufruf                                             |
| [Agents](/docs/de/sub-agents) | Führt einen Subagent mit seiner eigenen Systemaufforderung, seinem Modell und seinen Tools aus | Sie möchten einen separat definierten Helper für eine fokussierte Aufgabe                                   |
| [Skills](/docs/de/skills)     | Lädt aufgabenspezifische Anweisungen, wenn aufgerufen oder relevant                            | Sie haben einen wiederverwendbaren Workflow                                                                 |

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Settings](/docs/de/settings): wo das Feld `outputStyle` lebt und wie die Einstellungspriorität funktioniert
* [Permission modes](/docs/de/permission-modes): wie der Proactive-Stil den Auto-Modus vergleicht
* [Plugins](/docs/de/plugins): Verpacken und verteilen Sie Ausgabestile zusammen mit Skills, Hooks und Agents
* [Debug your configuration](/docs/de/debug-your-config): Diagnostizieren Sie, warum ein Ausgabestil nicht wirksam wird
