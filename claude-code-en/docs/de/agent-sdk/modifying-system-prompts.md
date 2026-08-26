> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ändern von Systemaufforderungen

> Wählen Sie zwischen der `claude_code`-Voreinstellung und einer benutzerdefinierten Systemaufforderung, und passen Sie das Verhalten mit CLAUDE.md, Ausgabestilen, Append oder einer vollständig benutzerdefinierten Aufforderung an.

Systemaufforderungen definieren Claudes Verhalten, Fähigkeiten und Antwortstil. Beginnen Sie mit der `claude_code`-Voreinstellung für CLI- oder IDE-ähnliche Codierungswerkzeuge, bei denen ein Mensch die Arbeit beobachtet und steuert. Schreiben Sie Ihre eigene Aufforderung für Agenten mit einer anderen Oberfläche, Identität oder einem anderen Berechtigungsmodell.

Diese Seite behandelt:

* [Wie Systemaufforderungen funktionieren](#how-system-prompts-work), mit einer Entscheidungstabelle zur Wahl zwischen der Voreinstellung, der Voreinstellung mit `append` und einer benutzerdefinierten Aufforderung
* [Passen Sie das Verhalten des Agenten an](#customize-agent-behavior) mit CLAUDE.md-Dateien, Ausgabestilen, `append` oder einer benutzerdefinierten Zeichenkette
* [Vergleichen Sie die vier Ansätze](#compare-the-four-approaches) nach Persistenz, Umfang und was sie bewahren
* [Kombinieren Sie Ansätze](#combine-approaches), um Anpassungsmethoden übereinander zu schichten

<h2 id="how-system-prompts-work">
  Wie Systemaufforderungen funktionieren
</h2>

Eine Systemaufforderung ist der anfängliche Anweisungssatz, der definiert, wie sich Claude während eines Gesprächs verhält. Das Agent SDK hat drei Ausgangspunkte dafür:

* **Minimale Standardeinstellung**: Wenn Sie `systemPrompt` in TypeScript oder `system_prompt` in Python nicht festlegen, verwendet das SDK eine minimale Aufforderung, die Werkzeugaufrufe abdeckt, aber Claude Code's Codierungsrichtlinien, Antwortstil und Projektkontext auslässt. Dies unterscheidet sich von `claude -p`, das standardmäßig die vollständige Claude Code-Aufforderung verwendet. Wenn Sie von der CLI migrieren und ein übereinstimmendes Verhalten wünschen, legen Sie die `claude_code`-Voreinstellung fest.
* **`claude_code`-Voreinstellung**: die vollständige Systemaufforderung, die die Claude Code CLI verwendet, mit Werkzeugnutzungsanweisungen, Code-Stil- und Formatierungsrichtlinien, Antworttone und Ausführlichkeitsregeln, Sicherheits- und Sicherheitsanweisungen sowie Kontext zum Arbeitsverzeichnis und zur Umgebung. Legen Sie `systemPrompt: { type: "preset", preset: "claude_code" }` in TypeScript oder `system_prompt={"type": "preset", "preset": "claude_code"}` in Python fest, optional mit `append`, um Ihre eigenen Anweisungen am Ende hinzuzufügen.
* **Benutzerdefinierte Zeichenkette**: eine Aufforderung, die Sie selbst schreiben. Das SDK sendet nur das, was Sie bereitstellen.

<h3 id="decide-on-a-starting-point">
  Entscheiden Sie sich für einen Ausgangspunkt
</h3>

Der entscheidende Faktor ist, wie sehr Ihr Agent Claude Code ähnelt: ein Codierungs-Agent, der in einem Repository arbeitet, mit einem Menschen, der die Streaming-Ausgabe beobachtet und die Arbeit lenkt. Je weiter Ihr Produkt davon entfernt ist, desto mehr werden Sie Ihre eigene Aufforderung schreiben wollen.

| Sie bauen                                                                                                                                                   | Verwenden Sie                                | Was Sie erhalten                                                                                                                                              |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Ein CLI- oder IDE-ähnliches Codierungswerkzeug, bei dem ein Mensch beobachtet und lenkt, und Claude Code's Standardeinstellungen sind das, was Sie wünschen | `claude_code`-Voreinstellung                 | Die vollständige Claude Code-Aufforderung: Werkzeuganleitung, Sicherheitsregeln, terminalfreundliche Antworten, Bewusstsein für Repository-Konventionen       |
| Das gleiche Werkzeug plus produktspezifische Regeln wie Codierungsstandards, Ausgabeformat oder Domänenkontext                                              | `claude_code`-Voreinstellung mit `append`    | Alles oben Genannte, mit Ihren Anweisungen nach der Voreinstellung hinzugefügt. Nichts wird entfernt, daher ist dies die Anpassung mit dem niedrigsten Risiko |
| Ein Agent mit einer anderen Oberfläche, Identität oder Berechtigungsmodell, oder ein Nicht-Codierungs-Agent                                                 | Benutzerdefinierte Aufforderungszeichenkette | Nur das, was Sie schreiben. Sie tragen die Verantwortung für das Ersetzen der Werkzeuganleitung und Sicherheitsanweisungen, die Ihr Agent noch benötigt       |
| Eine dünne Werkzeugaufrufs-Schleife ohne Agent-Persona, bei der Sie alles Verhalten in der Benutzeraufforderung bereitstellen                               | Keine `systemPrompt`-Option                  | Die minimale Standardeinstellung: Werkzeugaufrufs-Unterstützung und nichts anderes                                                                            |

„Unterschiedlich von Claude Code" bedeutet normalerweise eines der folgenden:

* **Andere Oberfläche**: Die Ausgabe wird nicht in einem Terminal von der Person gelesen, die sie ausgelöst hat. Chat-UIs, Strukturierte-Ausgabe-Consumer und Nicht-Codierungs-Automatisierung benötigen jeweils eine Aufforderung, die damit übereinstimmt, wie ihre Ausgabe gerendert und überprüft wird. Unbeaufsichtigte Codierungs-Automatisierung, wie ein CI-Job, der Lint-Fehler behebt oder Diffs überprüft, passt immer noch zur Voreinstellung, da die Arbeit selbst das ist, wofür die Voreinstellung geschrieben wurde.
* **Andere Identität**: Der Agent sollte sich nicht als Claude Code präsentieren. Ein Support-Bot, ein Datenanalyse-Assistent oder ein domänenspezifischer Agent benötigt seinen eigenen Namen, Umfang und eine eigene Persona.
* **Anderes Berechtigungsmodell**: Der Agent läuft autonom ohne menschliche Genehmigung bei jedem Schritt, oder arbeitet mit einem engen Satz von Ressourcen. Claude Code's Aufforderung geht davon aus, dass ein Mensch in der Schleife ist und Zugriff auf einen vollständigen Werkzeugsatz hat.
* **Nicht-Codierungs-Aufgaben**: Der Großteil von Claude Code's Aufforderung ist Codierungs-Anleitung. Für Forschungs-, Inhalts- oder Operations-Agenten konkurriert diese Anleitung mit den Anweisungen, die Sie tatsächlich benötigen.

Die [Vergleichstabelle](#compare-the-four-approaches) zeigt, was jede Anpassungsmethode bewahrt.

<h2 id="customize-agent-behavior">
  Verhalten des Agenten anpassen
</h2>

Ausgabestile, `append` und eine benutzerdefinierte Eingabeaufforderung ändern jeweils die Systemaufforderung direkt. CLAUDE.md geht einen anderen Weg: Das SDK liest sie und injiziert ihren Inhalt als Projektkontext in die Konversation, nicht in die Systemaufforderung, sodass sie das Verhalten neben jeder Systemaufforderung, die Sie wählen, prägt. [Skills](/docs/de/agent-sdk/skills), [hooks](/docs/de/agent-sdk/hooks) und [permissions](/docs/de/agent-sdk/permissions) prägen das Verhalten auch außerhalb der Systemaufforderung und werden auf eigenen Seiten behandelt.

<h3 id="claude-md-files-for-project-level-instructions">
  CLAUDE.md-Dateien für projektspezifische Anweisungen
</h3>

CLAUDE.md-Dateien geben Claude persistenten Projektkontext und Anweisungen. Das SDK injiziert ihren Inhalt in die Konversation, nicht in die Systemaufforderung, sodass sie mit jeder Systemaufforderungskonfiguration funktionieren. Informationen darüber, was in CLAUDE.md eingefügt werden soll, wo es platziert werden soll und wie effektive Anweisungen geschrieben werden, finden Sie unter [How Claude remembers your project](/docs/de/memory). Dieser Abschnitt behandelt, was für das SDK spezifisch ist: wie CLAUDE.md geladen wird.

Das SDK liest CLAUDE.md, wenn die entsprechende Einstellungsquelle aktiviert ist: `'project'` lädt `CLAUDE.md` oder `.claude/CLAUDE.md` aus dem Arbeitsverzeichnis, und `'user'` lädt `~/.claude/CLAUDE.md`. Standard-`query()`-Optionen aktivieren beide Quellen, sodass CLAUDE.md automatisch geladen wird. Wenn Sie `settingSources` in TypeScript oder `setting_sources` in Python explizit festlegen, beziehen Sie die benötigten Quellen ein. Das Laden von CLAUDE.md wird durch Einstellungsquellen gesteuert, nicht durch die `claude_code`-Voreinstellung.

<h4 id="load-claude-md-with-the-sdk">
  CLAUDE.md mit dem SDK laden
</h4>

Um CLAUDE.md zu laden, setzen Sie `settingSources` so, dass es die Ebene einschließt, auf der sich Ihre CLAUDE.md befindet. Das folgende Beispiel lädt eine projektspezifische CLAUDE.md zusammen mit der `claude_code`-Voreinstellung, sodass Claude sowohl die vollständige Coding-Agent-Eingabeaufforderung als auch die Konventionen Ihres Projekts hat:

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
  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []

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

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

CLAUDE.md ist persistent über alle Sitzungen in einem Projekt, wird mit Ihrem Team über Git geteilt und wird automatisch erkannt, ohne dass Codeänderungen erforderlich sind. Sie wird nicht geladen, wenn Sie ein leeres `settingSources`-Array übergeben.

<h3 id="output-styles-for-persistent-configurations">
  Ausgabestile für persistente Konfigurationen
</h3>

Ausgabestile sind gespeicherte Konfigurationen, die Claudes Systemaufforderung ändern. Sie werden als Markdown-Dateien gespeichert und können über Sitzungen und Projekte hinweg wiederverwendet werden.

<h4 id="create-an-output-style">
  Einen Ausgabestil erstellen
</h4>

Ein Ausgabestil ist eine Markdown-Datei mit [Frontmatter](/docs/de/output-styles#frontmatter) für Metadaten, gefolgt vom Eingabeaufforderungsinhalt. Speichern Sie ihn unter `~/.claude/output-styles/` für einen Stil auf Benutzerebene, der in jedem Projekt verfügbar ist, oder `.claude/output-styles/` in Ihrem Repository für einen Stil auf Projektebene, den Sie committen und mit Ihrem Team teilen können.

Standardmäßig ersetzt ein benutzerdefinierter Ausgabestil die Softwareentwicklungsanweisungen der `claude_code`-Voreinstellung durch Ihre eigenen. Um sie zu behalten und Ihre Anweisungen darauf zu schichten, setzen Sie `keep-coding-instructions: true` im Frontmatter. Behalten Sie sie, wenn Ihr Agent immer noch Softwareentwicklungsarbeit leistet. Lassen Sie sie weg, wenn Sie die Rolle vollständig ersetzen.

Das folgende Beispiel definiert eine Code-Review-Persona, die die Codierungsanweisungen beibehält, da die Überprüfung von Code immer noch von Claudes Code-Sicherheits- und Code-Qualitätsleitlinien profitiert. Speichern Sie es als `~/.claude/output-styles/code-reviewer.md`, um es über Projekte hinweg verfügbar zu machen:

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

* **CLI**: Führen Sie `/config` aus und wählen Sie einen Ausgabestil
* **Einstellungen**: Setzen Sie `outputStyle` in `.claude/settings.local.json`
* **TypeScript SDK**: Setzen Sie `outputStyle` innerhalb des Inline-`settings`-Objekts, das an `query()` übergeben wird, oder verweisen Sie `settings` auf eine Einstellungsdatei, die es setzt. `outputStyle` ist kein Feld auf oberster Ebene von `Options`:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Das Python SDK hat keine Option, einen Ausgabestil programmgesteuert auszuwählen. Verwenden Sie für Code-only-Bereitstellungen, bei denen Sie nicht in `.claude/settings.local.json` schreiben können, stattdessen `append` oder eine benutzerdefinierte Eingabeaufforderungszeichenkette.

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
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []

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
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Prompt-Caching über Benutzer und Maschinen verbessern
</h4>

Standardmäßig können zwei Sitzungen, die die gleiche `claude_code`-Voreinstellung und den gleichen `append`-Text verwenden, keinen Prompt-Cache-Eintrag teilen, wenn sie von verschiedenen Arbeitsverzeichnissen aus ausgeführt werden. Dies liegt daran, dass die Voreinstellung sitzungsspezifischen Kontext in die Systemaufforderung vor Ihrem `append`-Text einbettet: das Arbeitsverzeichnis, ob es sich um ein Git-Repository handelt, die Plattform, die aktive Shell, die Betriebssystemversion und Auto-Memory-Pfade. Jeder Unterschied in diesem Kontext erzeugt eine andere Systemaufforderung und einen Cache-Miss. CLAUDE.md-Inhalt beeinflusst den Systemaufforderungs-Cache nicht, da das SDK ihn in die Konversation injiziert, nicht in die Systemaufforderung.

Um die Systemaufforderung über Sitzungen hinweg identisch zu machen, setzen Sie `excludeDynamicSections: true` in TypeScript oder `"exclude_dynamic_sections": True` in Python. Der sitzungsspezifische Kontext wird in die erste Benutzernachricht verschoben, sodass nur die statische Voreinstellung und Ihr `append`-Text in der Systemaufforderung verbleiben, damit identische Konfigurationen einen Cache-Eintrag über Benutzer und Maschinen hinweg teilen können.

<Note>
  `excludeDynamicSections` erfordert `@anthropic-ai/claude-agent-sdk` v0.2.98 oder später oder `claude-agent-sdk` v0.1.58 oder später für Python. Es gilt nur für die Voreinstellungsobjektform und hat keine Auswirkung, wenn `systemPrompt` eine Zeichenkette ist.
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
  from claude_agent_sdk import query, ClaudeAgentOptions

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
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []

  async for message in query(
      prompt="Create a data processing pipeline",
      options=ClaudeAgentOptions(system_prompt=custom_prompt),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

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

<h2 id="use-cases-and-best-practices">
  Anwendungsfälle und Best Practices
</h2>

<h3 id="when-to-use-claude-md">
  Wann CLAUDE.md verwenden
</h3>

Verwenden Sie CLAUDE.md für Anweisungen, die für jede Sitzung in einem Projekt gelten sollten, unabhängig davon, welche Systemaufforderung die Sitzung verwendet: Codierungsstandards, häufige Befehle, Architekturkontext und Team-Konventionen. CLAUDE.md wird in Ihr Repository übernommen, sodass es mit dem Code, den es beschreibt, synchron bleibt. Siehe [Wann zu CLAUDE.md hinzufügen](/docs/de/memory#when-to-add-to-claude-md) für vollständige Anleitung.

CLAUDE.md-Dateien werden geladen, wenn die `project`-Einstellungsquelle aktiviert ist, was sie für Standard-`query()`-Optionen ist. Wenn Sie `settingSources` in TypeScript oder `setting_sources` in Python explizit festlegen, beziehen Sie `'project'` ein, um das Laden von projektspezifischen CLAUDE.md-Dateien beizubehalten.

<h3 id="when-to-use-output-styles">
  Wann Ausgabestile verwenden
</h3>

Ausgabestile sind für Personas, die Sie über die CLI und das SDK hinweg wiederverwenden möchten, ohne den Anwendungscode zu ändern. Da sie als Dateien in `.claude/output-styles` vorhanden sind, ist dieselbe Persona über `/config` in der CLI und aus jeder SDK-Sitzung verfügbar, die die entsprechende Einstellungsquelle lädt.

**Am besten für:**

* Persistente Verhaltensänderungen über Sitzungen hinweg
* Team-gemeinsame Konfigurationen
* Spezialisierte Assistenten wie ein Code-Reviewer, Datenwissenschaftler oder DevOps-Assistent
* Komplexe Aufforderungsänderungen, die Versionierung benötigen

**Beispiele:**

* Erstellen eines dedizierten SQL-Optimierungsassistenten
* Aufbau eines sicherheitsorientierten Code-Reviewers
* Entwicklung eines Lehrers mit spezifischer Pädagogik

<h3 id="when-to-use-systemprompt-with-append">
  Wann `systemPrompt` mit Append verwenden
</h3>

Verwenden Sie `append`, wenn die `claude_code`-Voreinstellung bereits zu Ihrem Produkt passt und Sie nur zusätzliche Anweisungen hinzufügen müssen. Sie behalten die Werkzeuganleitung, Sicherheitsregeln und Codierungskonventionen der Voreinstellung, ohne sie neu zu implementieren.

**Am besten für:**

* Hinzufügen spezifischer Codierungsstandards oder Vorlieben
* Anpassung der Ausgabeformatierung
* Hinzufügen domänenspezifischen Wissens
* Änderung der Antwortausführlichkeit
* Verbesserung des Standardverhaltens von Claude Code ohne Verlust von Werkzeuganweisungen

<h3 id="when-to-use-custom-systemprompt">
  Wann benutzerdefinierte `systemPrompt` verwenden
</h3>

Verwenden Sie eine benutzerdefinierte Aufforderung, wenn sich die Oberfläche, Identität oder das Berechtigungsmodell Ihres Agenten von Claude Code unterscheidet, wie in [Entscheiden Sie sich für einen Startpunkt](#decide-on-a-starting-point) beschrieben. Sie definieren den vollständigen Anweisungssatz, einschließlich aller Werkzeuganleitung und Sicherheitsregeln, die Ihr Agent benötigt.

**Am besten für:**

* Vollständige Kontrolle über Claudes Verhalten
* Spezialisierte Aufgaben mit einer Sitzung
* Testen neuer Aufforderungsstrategien
* Situationen, in denen Standard-Werkzeuge nicht benötigt werden
* Aufbau spezialisierter Agenten mit einzigartigem Verhalten

<h2 id="combine-approaches">
  Ansätze kombinieren
</h2>

Diese Methoden lassen sich kombinieren. Ein persistenter Ausgabestil oder CLAUDE.md legt das langfristige Verhalten fest, und `append` lagert sitzungsspezifische Anweisungen darauf, ohne die gespeicherte Konfiguration zu ändern.

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
  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []

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
