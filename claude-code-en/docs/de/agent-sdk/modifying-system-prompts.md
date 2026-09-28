> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ändern von Systemaufforderungen

> Wählen Sie zwischen der `claude_code`-Voreinstellung und einer benutzerdefinierten Systemaufforderung, und passen Sie das Verhalten mit CLAUDE.md, Ausgabestilen, Append oder einer vollständig benutzerdefinierten Aufforderung an.

Systemaufforderungen definieren Claudes Verhalten, Fähigkeiten und Antwortstil. Beginnen Sie mit der `claude_code`-Voreinstellung für CLI- oder IDE-ähnliche Codierungswerkzeuge, bei denen ein Mensch die Arbeit beobachtet und steuert. Schreiben Sie Ihre eigene Aufforderung für Agenten mit einer anderen Oberfläche, Identität oder einem anderen Berechtigungsmodell.

<h2 id="how-system-prompts-work">
  Wie Systemaufforderungen funktionieren
</h2>

Eine Systemaufforderung ist der anfängliche Anweisungssatz, der definiert, wie sich Claude während eines Gesprächs verhält. Das Agent SDK hat drei Ausgangspunkte dafür:

* **Minimale Standardeinstellung**: Wenn Sie `systemPrompt` in TypeScript oder `system_prompt` in Python nicht festlegen, verwendet das SDK eine minimale Aufforderung, die Werkzeugaufrufe abdeckt, aber den Rest des `claude_code`-Presets auslässt, einschließlich seiner Sicherheits- und Sicherheitsanweisungen sowie seines Kontexts zum Arbeitsverzeichnis und zur Umgebung. Dies unterscheidet sich von `claude -p`, das standardmäßig die Claude Code-Systemaufforderung verwendet. Wenn Sie von der CLI migrieren und ein übereinstimmendes Verhalten wünschen, legen Sie das `claude_code`-Preset fest.
* **`claude_code`-Preset**: die Systemaufforderung, die die Claude Code CLI verwendet, mit Werkzeugnutzungsanweisungen, Sicherheits- und Sicherheitsanweisungen sowie Kontext zum Arbeitsverzeichnis und zur Umgebung. Legen Sie `systemPrompt: { type: "preset", preset: "claude_code" }` in TypeScript oder `system_prompt={"type": "preset", "preset": "claude_code"}` in Python fest, optional mit `append`, um Ihre eigenen Anweisungen am Ende hinzuzufügen.
* **Benutzerdefinierte Zeichenkette**: eine Aufforderung, die Sie selbst schreiben. Das SDK sendet nur das, was Sie bereitstellen.

<h3 id="decide-on-a-starting-point">
  Entscheiden Sie sich für einen Ausgangspunkt
</h3>

Der entscheidende Faktor ist, wie sehr Ihr Agent Claude Code ähnelt: ein Codierungs-Agent, der in einem Repository arbeitet, mit einem Menschen, der die Streaming-Ausgabe beobachtet und die Arbeit lenkt. Je weiter Ihr Produkt davon entfernt ist, desto mehr werden Sie Ihre eigene Aufforderung schreiben wollen.

| Sie bauen                                                                                                                                                   | Verwenden Sie                                | Was Sie erhalten                                                                                                                                        |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Ein CLI- oder IDE-ähnliches Codierungswerkzeug, bei dem ein Mensch beobachtet und lenkt, und Claude Code's Standardeinstellungen sind das, was Sie wünschen | `claude_code`-Preset                         | Die Claude Code-Aufforderung, einschließlich Werkzeuganleitung, Sicherheitsregeln und Umgebungskontext                                                  |
| Das gleiche Werkzeug plus produktspezifische Regeln wie Codierungsstandards, Ausgabeformat oder Domänenkontext                                              | `claude_code`-Preset mit `append`            | Alles oben Genannte, mit Ihren Anweisungen nach dem Preset hinzugefügt. Nichts wird entfernt, daher ist dies die Anpassung mit dem niedrigsten Risiko   |
| Ein Agent mit einer anderen Oberfläche, Identität oder Berechtigungsmodell, oder ein Nicht-Codierungs-Agent                                                 | Benutzerdefinierte Aufforderungszeichenkette | Nur das, was Sie schreiben. Sie tragen die Verantwortung für das Ersetzen der Werkzeuganleitung und Sicherheitsanweisungen, die Ihr Agent noch benötigt |
| Eine dünne Werkzeugaufrufs-Schleife ohne Agent-Persona, bei der Sie alles Verhalten in der Benutzeraufforderung bereitstellen                               | Keine `systemPrompt`-Option                  | Die minimale Standardeinstellung: Werkzeugaufrufs-Unterstützung und nichts anderes                                                                      |

„Unterschiedlich von Claude Code" bedeutet normalerweise eines der folgenden:

* **Andere Oberfläche**: Die Ausgabe wird nicht in einem Terminal von der Person gelesen, die sie ausgelöst hat. Chat-UIs, Strukturierte-Ausgabe-Consumer und Nicht-Codierungs-Automatisierung benötigen jeweils eine Aufforderung, die damit übereinstimmt, wie ihre Ausgabe gerendert und überprüft wird. Unbeaufsichtigte Codierungs-Automatisierung, wie ein CI-Job, der Lint-Fehler behebt oder Diffs überprüft, passt immer noch zum Preset, da die Arbeit selbst das ist, wofür das Preset geschrieben wurde.
* **Andere Identität**: Der Agent sollte sich nicht als Claude Code präsentieren. Ein Support-Bot, ein Datenanalyse-Assistent oder ein domänenspezifischer Agent benötigt seinen eigenen Namen, Umfang und eine eigene Persona.
* **Anderes Berechtigungsmodell**: Der Agent läuft autonom ohne menschliche Genehmigung bei jedem Schritt, oder arbeitet mit einem engen Satz von Ressourcen. Claude Code's Aufforderung geht davon aus, dass ein Mensch in der Schleife ist und Zugriff auf einen vollständigen Werkzeugsatz hat.
* **Nicht-Codierungs-Aufgaben**: Der Großteil von Claude Code's Aufforderung ist Codierungs-Anleitung. Für Forschungs-, Inhalts- oder Operations-Agenten konkurriert diese Anleitung mit den Anweisungen, die Sie tatsächlich benötigen.

Die [Vergleichstabelle](#compare-the-four-approaches) zeigt, was jede Anpassungsmethode bewahrt.

<h2 id="customize-agent-behavior">
  Verhalten des Agenten anpassen
</h2>

`append` und eine benutzerdefinierte Eingabeaufforderungszeichenkette ändern jeweils die Systemaufforderung direkt, und ein Ausgabestil ändert die Anweisungen, die Claude Code Claude für jede Antwort gibt. CLAUDE.md geht einen anderen Weg: Das SDK liest sie und injiziert ihren Inhalt als Projektkontext in die Konversation, sodass sie das Verhalten neben jeder Systemaufforderung, die Sie wählen, prägt. [Skills](/docs/de/agent-sdk/skills), [hooks](/docs/de/agent-sdk/hooks) und [permissions](/docs/de/agent-sdk/permissions) prägen das Verhalten auch außerhalb der Systemaufforderung und werden auf eigenen Seiten behandelt.

<h3 id="claude-md-files-for-project-level-instructions">
  CLAUDE.md-Dateien für projektspezifische Anweisungen
</h3>

CLAUDE.md-Dateien geben Claude persistenten Projektkontext und Anweisungen. Das SDK injiziert ihren Inhalt in die Konversation und lässt die Systemaufforderung unverändert, sodass sie mit jeder Systemaufforderungskonfiguration funktionieren. Informationen darüber, was in CLAUDE.md eingefügt werden soll, wo es platziert werden soll und wie effektive Anweisungen geschrieben werden, finden Sie unter [When to add to CLAUDE.md](/docs/de/memory#when-to-add-to-claude-md) und im Rest von [How Claude remembers your project](/docs/de/memory). Dieser Abschnitt behandelt, was für das SDK spezifisch ist: wie CLAUDE.md geladen wird.

Das SDK liest CLAUDE.md, wenn die entsprechende Einstellungsquelle aktiviert ist: `'project'` lädt `CLAUDE.md` oder `.claude/CLAUDE.md` aus dem Arbeitsverzeichnis, und `'user'` lädt `~/.claude/CLAUDE.md`. Standard-`query()`-Optionen aktivieren beide Quellen, sodass CLAUDE.md automatisch geladen wird. Wenn Sie `settingSources` in TypeScript oder `setting_sources` in Python explizit festlegen, beziehen Sie die benötigten Quellen ein. Das Laden von CLAUDE.md wird durch Einstellungsquellen gesteuert, nicht durch die `claude_code`-Voreinstellung.

<h4 id="load-claude-md-with-the-sdk">
  CLAUDE.md mit dem SDK laden
</h4>

Um CLAUDE.md zu laden, setzen Sie `settingSources` so, dass es die Ebene einschließt, auf der sich Ihre CLAUDE.md befindet. Das folgende Beispiel lädt eine projektspezifische CLAUDE.md zusammen mit der `claude_code`-Voreinstellung, sodass Claude sowohl die Coding-Agent-Eingabeaufforderung als auch die Konventionen Ihres Projekts hat:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []


  async def main():
      async for message in query(
          prompt="Add a new React component for user profiles",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",  # Use Claude Code's system prompt
              },
              setting_sources=["project"],  # Loads CLAUDE.md from project
          ),
      ):
          messages.append(message)


  asyncio.run(main())

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

Wenn Sie eines der Beispiele ausführen, streamt das SDK Nachrichten, während Claude arbeitet: eine Systeminitalisierungsnachricht, Assistentennachrichten, Benutzernachrichten mit Werkzeugergebnissen und eine abschließende Ergebnisnachricht mit dem Sitzungsergebnis.

CLAUDE.md ist persistent über alle Sitzungen in einem Projekt, wird mit Ihrem Team über Git geteilt und wird automatisch erkannt, ohne dass Codeänderungen erforderlich sind. Sie wird nicht geladen, wenn Sie ein leeres `settingSources`-Array übergeben.

<h3 id="output-styles-for-persistent-configurations">
  Ausgabestile für persistente Konfigurationen
</h3>

Ausgabestile sind gespeicherte Konfigurationen von Anweisungen, die Claudes Rolle, Ton und Ausgabeformat ändern. Sie werden als Markdown-Dateien gespeichert und können über Sitzungen und Projekte hinweg wiederverwendet werden.

<h4 id="create-an-output-style">
  Einen Ausgabestil erstellen
</h4>

Ein Ausgabestil ist eine Markdown-Datei mit [Frontmatter](/docs/de/output-styles#frontmatter) für Metadaten, gefolgt vom Eingabeaufforderungsinhalt. Speichern Sie ihn unter `~/.claude/output-styles/` für einen Stil auf Benutzerebene, der in jedem Projekt verfügbar ist, oder `.claude/output-styles/` in Ihrem Repository für einen Stil auf Projektebene, den Sie committen und mit Ihrem Team teilen können.

Ein benutzerdefinierter Ausgabestil lässt die Softwareentwicklungsanweisungen der `claude_code`-Voreinstellung weg und verwendet Ihre eigenen. Um sie zu behalten und Ihre Anweisungen darauf zu schichten, setzen Sie `keep-coding-instructions: true` im Frontmatter. Diese Anweisungen sind nur in Claudes vollständiger Systemaufforderung von Claude Code vorhanden, daher hat die Einstellung keine Auswirkung in einer Sitzung auf der kürzeren Systemaufforderung, die Sie mit [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/de/env-vars#variables) ein- oder ausschalten. Behalten Sie sie, wenn Ihr Agent immer noch Softwareentwicklungsarbeit leistet. Lassen Sie sie weg, wenn Sie die Rolle vollständig ersetzen.

Das folgende Beispiel definiert eine Code-Review-Persona, die die Codierungsanweisungen beibehält, da die Überprüfung von Code immer noch von Claudes Sicherheits- und Code-Qualitätsleitlinien profitiert. Speichern Sie es als `~/.claude/output-styles/code-reviewer.md`, um es über Projekte hinweg verfügbar zu machen:

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  Einen Ausgabestil aktivieren
</h4>

Nach der Erstellung aktivieren Sie Ausgabestile über:

* **CLI**: Führen Sie `/output-style <style>` aus, zum Beispiel `/output-style concise`, oder führen Sie `/config` aus und wählen Sie einen. Der Befehl `/output-style` erfordert Claude Code v2.1.269 oder später.
* **Einstellungen**: Setzen Sie `outputStyle` in `.claude/settings.local.json`
* **TypeScript SDK**: Setzen Sie `outputStyle` innerhalb des Inline-`settings`-Objekts, das an `query()` übergeben wird, oder verweisen Sie `settings` auf eine Einstellungsdatei, die es setzt. `outputStyle` ist kein Feld auf oberster Ebene von `Options`:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Im Python SDK setzen Sie `outputStyle` über die `settings`-Option, die eine JSON-Zeichenkette wie `'{"outputStyle": "Explanatory"}'` oder einen Pfad zu einer Einstellungsdatei, die es setzt, akzeptiert.

**Hinweis für SDK-Benutzer:** Ausgabestile werden geladen, wenn Sie `settingSources: ['user']` oder `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` oder `setting_sources=["project"]` (Python) in Ihren Optionen einbeziehen.

<h3 id="append-to-the-claude_code-preset">
  An die `claude_code`-Voreinstellung anhängen
</h3>

Sie können die Claude Code-Voreinstellung mit einer `append`-Eigenschaft verwenden, um Ihre benutzerdefinierten Anweisungen hinzuzufügen und gleichzeitig alle integrierten Funktionen zu bewahren.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []


  async def main():
      async for message in query(
          prompt="Help me write a Python function to calculate fibonacci numbers",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": "Always include detailed docstrings and type hints in Python code.",
              }
          ),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Prompt-Caching über Benutzer und Maschinen verbessern
</h4>

Standardmäßig können zwei Sitzungen, die die gleiche `claude_code`-Voreinstellung und den gleichen `append`-Text verwenden, keinen Prompt-Cache-Eintrag teilen, wenn sie von verschiedenen Arbeitsverzeichnissen aus ausgeführt werden. Dies liegt daran, dass die Voreinstellung sitzungsspezifischen Kontext in die Systemaufforderung vor Ihrem `append`-Text einbettet: das Arbeitsverzeichnis, ob es sich um ein Git-Repository handelt, die Plattform, die aktive Shell, die Betriebssystemversion und Auto-Memory-Pfade. Jeder Unterschied in diesem Kontext erzeugt eine andere Systemaufforderung und einen Cache-Miss. CLAUDE.md-Inhalt beeinflusst den Systemaufforderungs-Cache nicht, da das SDK ihn in die Konversation injiziert, nicht in die Systemaufforderung.

Um die Systemaufforderung über Sitzungen hinweg identisch zu machen, setzen Sie `excludeDynamicSections: true` in TypeScript oder `"exclude_dynamic_sections": True` in Python. Der sitzungsspezifische Kontext wird in die erste Benutzernachricht verschoben, sodass nur die statische Voreinstellung und Ihr `append`-Text in der Systemaufforderung verbleiben, damit identische Konfigurationen einen Cache-Eintrag über Benutzer und Maschinen hinweg teilen können.

<Note>
  `excludeDynamicSections` erfordert `@anthropic-ai/claude-agent-sdk` v0.2.98 oder später oder `claude-agent-sdk` v0.1.58 oder später für Python. Setzen Sie es nur auf der Voreinstellungsobjektform. Das SDK ignoriert es, wenn Sie eine benutzerdefinierte Eingabeaufforderung statt der Voreinstellung übergeben; um die statischen Anweisungen einer benutzerdefinierten Eingabeaufforderung im TypeScript SDK zwischengespeichert zu halten, siehe [Cache the static part of a custom prompt](#cache-the-static-part-of-a-custom-prompt).
</Note>

Das folgende Beispiel kombiniert einen gemeinsamen `append`-Block mit `excludeDynamicSections`, sodass eine Flotte von Agenten, die von verschiedenen Verzeichnissen aus ausgeführt werden, die gleiche zwischengespeicherte Systemaufforderung wiederverwenden können:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Triage the open issues in this repo",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
                  "exclude_dynamic_sections": True,
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

**Kompromisse:** Das Arbeitsverzeichnis, das Git-Repo-Flag, die Plattform, die aktive Shell, die Betriebssystemversion und Auto-Memory-Pfade erreichen Claude immer noch, aber als Teil der ersten Benutzernachricht statt der Systemaufforderung. Anweisungen in der Benutzernachricht haben etwas weniger Gewicht als der gleiche Text in der Systemaufforderung, daher kann Claude sich bei der Überlegung zum aktuellen Verzeichnis oder zu Auto-Memory-Pfaden weniger stark auf sie verlassen. Aktivieren Sie diese Option, wenn die Wiederverwendung des Cross-Session-Cache wichtiger ist als maximal autoritative Umgebungskontexte.

Für das entsprechende Flag im nicht-interaktiven CLI-Modus siehe [`--exclude-dynamic-system-prompt-sections`](/docs/de/cli-reference).

<h3 id="custom-system-prompts">
  Benutzerdefinierte Systemaufforderungen
</h3>

Sie können eine benutzerdefinierte Zeichenkette als `systemPrompt` bereitstellen, um die Standardeinstellung vollständig durch Ihre eigenen Anweisungen zu ersetzen.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []


  async def main():
      async for message in query(
          prompt="Create a data processing pipeline",
          options=ClaudeAgentOptions(system_prompt=custom_prompt),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

In Python können Sie eine große benutzerdefinierte Eingabeaufforderung aus einer Datei mit `system_prompt={"type": "file", "path": "..."}` laden, anstatt sie als Zeichenkette zu übergeben. Das Python SDK übergibt eine Zeichenketten-Eingabeaufforderung als ein Befehlszeilenargument an den CLI-Unterprozess, daher schlägt eine Eingabeaufforderung, die das Betriebssystem-Argumentlängenlimit überschreitet, beim Prozessstart fehl, bevor eine API-Anfrage gesendet wird. Unter Linux ist der Fehler `Argument list too long`. Siehe [`SystemPromptFile`](/docs/de/agent-sdk/python#systempromptfile) für die Plattformschwellwerte und das Windows-Verhalten.

<h4 id="cache-the-static-part-of-a-custom-prompt">
  Den statischen Teil einer benutzerdefinierten Eingabeaufforderung zwischenspeichern
</h4>

Im TypeScript SDK können Sie eine benutzerdefinierte Eingabeaufforderung als Array von Zeichenketten statt als eine Zeichenkette übergeben, mit dem `SYSTEM_PROMPT_DYNAMIC_BOUNDARY`-Marker zwischen dem statischen Teil und dem Rest. Verwenden Sie dies, wenn Ihre Eingabeaufforderung Anweisungen kombiniert, die bei jeder Anfrage gleich sind, mit Kontext, der sich pro Anfrage ändert, wie z. B. der Kunde oder das Ticket, das der Agent bearbeitet. Wenn Sie beide Teile als eine Zeichenkette übergeben, ändert eine Änderung des Pro-Anfrage-Teils die gesamte Systemaufforderung, sodass die statischen Anweisungen auch den Cache verfehlen. Diese Form ist im Python SDK nicht verfügbar; [`ClaudeAgentOptions`](/docs/de/agent-sdk/python#claudeagentoptions) listet die Formen auf, die `system_prompt` akzeptiert.

<Note>
  Das SDK teilt die Eingabeaufforderung nur, wenn es die Claude API direkt aufruft oder auf [Claude Platform on AWS](/docs/de/claude-platform-on-aws) ausgeführt wird. In jeder anderen Konfiguration, wie Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder ein [LLM-Gateway](/docs/de/llm-gateway-connect), und wann immer Sie [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/de/llm-gateway-protocol#disable-pre-release-capabilities) setzen, sendet das SDK die gesamte Eingabeaufforderung als einen Block, genauso wie das Übergeben einer Zeichenkette.
</Note>

Um die Eingabeaufforderung zu teilen, importieren Sie `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` aus `@anthropic-ai/claude-agent-sdk` und übergeben Sie es als sein eigenes Array-Element zwischen den beiden Teilen. Das SDK sendet die Zeichenketten vor dem Marker als einen Textblock und die Zeichenketten nach ihm als einen zweiten Block, jeweils mit seinem eigenen Cache-Breakpoint. Im folgenden Beispiel lädt ein Support-Agent seine Triage-Anweisungen aus einer Datei und erhält Details zu einem Ticket bei jeder Anfrage, sodass die Anweisungen zwischengespeichert bleiben, während sich die Ticket-Details ändern:

```typescript TypeScript theme={null}
import { readFile } from "node:fs/promises";
import { query, SYSTEM_PROMPT_DYNAMIC_BOUNDARY } from "@anthropic-ai/claude-agent-sdk";

// Identical on every request
const instructions = await readFile("triage-instructions.md", "utf8");
// Different on every request
const ticketContext = "Customer plan: Enterprise. Other open tickets from this customer: 3.";

for await (const message of query({
  prompt: "Triage ticket 4821",
  options: {
    systemPrompt: [instructions, SYSTEM_PROMPT_DYNAMIC_BOUNDARY, ticketContext]
  }
})) {
  // ...
}
```

[Track cache tokens](/docs/de/agent-sdk/cost-tracking#track-cache-tokens) beschreibt die Felder `cache_creation_input_tokens` und `cache_read_input_tokens` auf jeder Ergebnisnachricht.

Das SDK setzt die Blöcke aus dem Array wie folgt zusammen:

* Das SDK verbindet die Zeichenketten auf jeder Seite des Markers mit einer Leerzeile dazwischen und entfernt den Marker selbst, sodass der Marker-Text Claude nicht erreicht.
* Wenn Sie den Marker mehr als einmal einbeziehen, ist der erste die Aufteilung und das SDK entfernt die anderen.
* Wenn Sie den Marker weglassen, verbindet das SDK alle Zeichenketten in einen Block, genauso wie das Übergeben einer Zeichenkette.

Mit den CLI-Flags [`--system-prompt` oder `--system-prompt-file`](/docs/de/cli-reference#system-prompt-flags) ist die Eingabeaufforderung eine Zeichenkette, daher gibt es kein Array, das den Marker tragen kann. Beziehen Sie stattdessen eine Zeile ein, die nur `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` enthält, zwischen den statischen und Pro-Anfrage-Teilen. Claude Code teilt die Eingabeaufforderung an der ersten solchen Zeile in die gleichen zwei Blöcke auf und entfernt diese Zeile. Erfordert Claude Code v2.1.275 oder später.

Im SDK bevorzugen Sie die Array-Form, die die Grenze ohne eine Marker-Zeile trägt.

<h3 id="change-the-prompt-of-an-existing-session">
  Ändern Sie die Eingabeaufforderung einer bestehenden Sitzung
</h3>

Standardmäßig erstellt Claude Code die Systemaufforderung einmal, bei der ersten Anfrage einer Sitzung, mit Ihrem `append`-Text oder benutzerdefinierten Eingabeaufforderung eingeschlossen, und zeichnet sie in der Sitzung auf. Bis die Sitzung komprimiert wird, verwendet jede spätere Anfrage diese aufgezeichnete Eingabeaufforderung, auch nachdem Sie zur Sitzung mit `resume` oder `continue` zurückkehren. Wenn Sie eine andere `append` oder benutzerdefinierte Eingabeaufforderung bei diesem späteren Aufruf übergeben, wird sie wirksam, sobald die Sitzung komprimiert wird oder in einer neuen Sitzung.

<h4 id="update-claude’s-instructions-mid-session">
  Aktualisieren Sie Claudes Anweisungen während der Sitzung
</h4>

Wenn sich die Anweisungen, die Sie in die Systemaufforderung eingefügt haben, ändern müssen, während eine Sitzung läuft, beispielsweise weil Ihr Benutzer den Agenten in einen schreibgeschützten Modus umgeschaltet hat oder seine Konfiguration in Ihrer App bearbeitet hat, senden Sie die neuen Anweisungen in der Konversation statt `systemPrompt` zu ändern:

* **In Ihrer nächsten Nachricht**: Beziehen Sie die neuen Anweisungen in die nächste Benutzernachricht ein, die Sie senden.
* **Aus einem Hook**: Geben Sie [`additionalContext`](/docs/de/hooks#add-context-for-claude) aus einem `UserPromptSubmit` oder `PostToolUse` [Hook-Callback](/docs/de/agent-sdk/hooks#outputs) zurück, geschrieben als eine sachliche Aussage wie „Der Arbeitsbereich ist jetzt schreibgeschützt". Das SDK fügt den Text an der Stelle in die Konversation ein, an der der Hook ausgelöst wurde, sodass die aufgezeichnete Eingabeaufforderung unverändert bleibt.

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  Schalten Sie die Aufzeichnung aus, während Sie an der Formulierung arbeiten
</h4>

Während Sie an der Formulierung der Eingabeaufforderung arbeiten und möchten, dass jede Bearbeitung eine Sitzung erreicht, die Sie fortsetzen, setzen Sie `snapshot` auf false auf der Objektform der Systemaufforderung. Claude Code erstellt dann die Eingabeaufforderung bei jeder Anfrage neu. Das Feld ist auf der Voreinstellungs- und benutzerdefinierten Form von [`systemPrompt`](/docs/de/agent-sdk/typescript#options) in TypeScript und von [`system_prompt`](/docs/de/agent-sdk/python#systempromptpreset) in Python verfügbar und erfordert `@anthropic-ai/claude-agent-sdk` v0.3.257 oder später oder `claude-agent-sdk` v0.2.153 oder später.

Halten Sie die Aufzeichnung in der Produktion ein. Mit ausgeschalteter Aufzeichnung erreicht eine andere `append` oder benutzerdefinierte Eingabeaufforderung auf einer fortgesetzten Sitzung Claude beim nächsten Durchgang, und diese Anfrage kann den [Prompt-Cache](/docs/de/prompt-caching#how-the-cache-is-organized) der Sitzung nicht wiederverwenden. Wo die API [bewahrtes Denken](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking) erzwingt, verliert Claude auch sein Denken aus früheren Durchgängen.

Außerhalb von [Cloud-Sitzungen](/docs/de/cloud-environments), wenn Sie Claude Code im [Bare-Modus](/docs/de/headless#start-faster-with-bare-mode) starten, indem Sie `--bare` durch `extraArgs` übergeben oder `CLAUDE_CODE_SIMPLE=1` setzen, bleibt die Aufzeichnung aus, es sei denn, Sie setzen `snapshot: true`.

Das Aufzeichnen einer `append` oder benutzerdefinierten Eingabeaufforderung standardmäßig erfordert Claude Code v2.1.265 oder später, das das TypeScript Agent SDK ab v0.3.265 und das Python Agent SDK ab v0.2.153 bündelt. Vor Claude Code v2.1.268 erstellten Sitzungen, die keine [Funktionsflags abrufen](/docs/de/env-vars#features-that-need-feature-flag-fetching), einschließlich Sitzungen auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry, die Eingabeaufforderung bei jeder Anfrage neu und `snapshot` hatte keine Auswirkung.

<h2 id="compare-the-four-approaches">
  Vergleich der vier Ansätze
</h2>

Die vier Anpassungsmethoden unterscheiden sich darin, wo sie sich befinden, wie sie gemeinsam genutzt werden und was sie aus der `claude_code`-Voreinstellung beibehalten.

| Funktion                   | CLAUDE.md         | Ausgabestile                     | `systemPrompt` mit Append | Benutzerdefinierte `systemPrompt` |
| -------------------------- | ----------------- | -------------------------------- | ------------------------- | --------------------------------- |
| **Persistenz**             | Pro-Projekt-Datei | Als Dateien gespeichert          | Nur Sitzung               | Nur Sitzung                       |
| **Wiederverwendbarkeit**   | Pro-Projekt       | Über Projekte hinweg             | Code-Duplizierung         | Code-Duplizierung                 |
| **Verwaltung**             | Im Dateisystem    | CLI + Dateien                    | Im Code                   | Im Code                           |
| **Standard-Werkzeuge**     | Bewahrt           | Bewahrt                          | Bewahrt                   | Verloren (sofern nicht enthalten) |
| **Integrierte Sicherheit** | Beibehalten       | Beibehalten                      | Beibehalten               | Muss hinzugefügt werden           |
| **Umgebungskontext**       | Automatisch       | Automatisch                      | Automatisch               | Muss bereitgestellt werden        |
| **Anpassungsebene**        | Nur Ergänzungen   | Standard ersetzen oder erweitern | Nur Ergänzungen           | Vollständige Kontrolle            |
| **Versionskontrolle**      | Mit Projekt       | Ja                               | Mit Code                  | Mit Code                          |
| **Umfang**                 | Projektspezifisch | Benutzer oder Projekt            | Code-Sitzung              | Code-Sitzung                      |

„Mit Append" bedeutet die Verwendung von `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` in TypeScript oder `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` in Python. CLAUDE.md ändert den System-Prompt selbst nicht: Das SDK injiziert seinen Inhalt als Projektkontext in die Konversation.

<h2 id="combine-approaches">
  Ansätze kombinieren
</h2>

Die Ansätze lassen sich kombinieren. Ein persistenter Ausgabestil oder CLAUDE.md legt das langfristige Verhalten fest, und `append` lagert sitzungsspezifische Anweisungen darauf, ohne die gespeicherte Konfiguration zu ändern.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Einen Ausgabestil mit sitzungsspezifischen Ergänzungen kombinieren
</h3>

Das folgende Beispiel setzt voraus, dass ein Ausgabestil „Code Reviewer" bereits aktiv ist. Der `append`-Block lagert sitzungsspezifische Schwerpunkte auf die Persona, sodass eine einzelne Review-Sitzung OAuth und Token-Speicherung priorisieren kann, ohne den gespeicherten Ausgabestil zu ändern:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Assuming "Code Reviewer" output style is active (via /config or settings)
  // Add session-specific focus areas
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []


  async def main():
      async for message in query(
          prompt="Review this authentication module",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": """
                  For this review, prioritize:
                  - OAuth 2.0 compliance
                  - Token storage security
                  - Session management
                  """,
              }
          ),
      ):
          messages.append(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="see-also">
  Siehe auch
</h2>

* [Ausgabestile](/docs/de/output-styles): Erstellen, verwalten und teilen Sie Ausgabestile für die CLI, einschließlich des Dateiformats und der Speicherorte
* [Wie Claude sich Ihr Projekt merkt](/docs/de/memory): Was in CLAUDE.md gehört, wo Sie es platzieren, und wie Sie effektive Projektanweisungen schreiben
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript): Der vollständige `Options`-Typ, einschließlich `systemPrompt`, `settingSources` und `settings`
* [Python SDK-Referenz](/docs/de/agent-sdk/python): Der vollständige `ClaudeAgentOptions`-Typ, einschließlich `system_prompt` und `setting_sources`
* [Einstellungen](/docs/de/settings): Die `settings.json`-Referenz, einschließlich des Speicherorts von Ausgabestilen und anderen Konfigurationen
