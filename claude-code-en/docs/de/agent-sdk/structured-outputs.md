> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Strukturierte Ausgaben von Agenten abrufen

> Validiertes JSON aus Agent-Workflows mit JSON Schema, Zod oder Pydantic zurückgeben. Erhalten Sie typsichere, strukturierte Daten nach Multi-Turn-Tool-Nutzung.

Strukturierte Ausgaben ermöglichen es Ihnen, die genaue Form der Daten zu definieren, die Sie von einem Agenten zurückerhalten möchten. Der Agent kann alle erforderlichen Tools verwenden, um die Aufgabe zu erfüllen, und Sie erhalten am Ende immer noch validiertes JSON, das Ihrem Schema entspricht. Definieren Sie ein [JSON Schema](https://json-schema.org/understanding-json-schema/about) für die benötigte Struktur, und das SDK validiert die Ausgabe dagegen und fordert bei Nichtübereinstimmung erneut auf. Wenn die Validierung nicht innerhalb des Wiederholungslimits erfolgreich ist, ist das Ergebnis ein Fehler statt strukturierter Daten; siehe [Fehlerbehandlung](#error-handling).

Für vollständige Typsicherheit verwenden Sie [Zod](#type-safe-schemas-with-zod-and-pydantic) (TypeScript) oder [Pydantic](#type-safe-schemas-with-zod-and-pydantic) (Python), um Ihr Schema zu definieren und stark typisierte Objekte zurückzuerhalten.

<h2 id="why-structured-outputs">
  Warum strukturierte Ausgaben?
</h2>

Agenten geben standardmäßig freien Text zurück, was für Chat funktioniert, aber nicht, wenn Sie die Ausgabe programmgesteuert verwenden müssen. Strukturierte Ausgaben geben Ihnen typisierte Daten, die Sie direkt an Ihre Anwendungslogik, Datenbank oder UI-Komponenten übergeben können.

Stellen Sie sich eine Rezept-App vor, bei der ein Agent das Web durchsucht und Rezepte zurückbringt. Ohne strukturierte Ausgaben erhalten Sie freien Text, den Sie selbst analysieren müssten. Mit strukturierten Ausgaben definieren Sie die gewünschte Form und erhalten typisierte Daten, die Sie direkt in Ihrer App verwenden können.

<AccordionGroup>
  <Accordion title="Ohne strukturierte Ausgaben">
    ```text theme={null}
    Hier ist ein klassisches Rezept für Schokoladenchip-Kekse!

    **Schokoladenchip-Kekse**
    Vorbereitungszeit: 15 Minuten | Backzeit: 10 Minuten

    Zutaten:
    - 2 1/4 Tassen Allzweckmehl
    - 1 Tasse Butter, erweicht
    ...
    ```

    Um dies in Ihrer App zu verwenden, müssten Sie den Titel extrahieren, '15 Minuten" in eine Zahl umwandeln, Zutaten von Anweisungen trennen und mit inkonsistenter Formatierung über Antworten hinweg umgehen.
  </Accordion>

  <Accordion title="Mit strukturierten Ausgaben">
    ```json theme={null}
    {
      "name": "Schokoladenchip-Kekse",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "Allzweckmehl", "amount": 2.25, "unit": "Tassen" },
        { "item": "Butter, erweicht", "amount": 1, "unit": "Tasse" }
        // ...
      ],
      "steps": ["Ofen auf 375°F vorheizen", "Butter und Zucker cremig rühren" /* ... */]
    }
    ```

    Typisierte Daten, die Sie direkt in Ihrer UI verwenden können.
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  Schnellstart
</h2>

Um strukturierte Ausgaben zu verwenden, definieren Sie ein [JSON Schema](https://json-schema.org/understanding-json-schema/about), das die Form der gewünschten Daten beschreibt, und übergeben Sie es dann an `query()` über die Option `outputFormat` (TypeScript) oder `output_format` (Python). Wenn der Agent fertig ist, enthält die Ergebnismeldung ein Feld `structured_output` mit validierten Daten, die Ihrem Schema entsprechen.

Das folgende Beispiel fordert den Agenten auf, Anthropic zu recherchieren und den Firmennamen, das Gründungsjahr und den Hauptsitz als strukturierte Ausgabe zurückzugeben.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Definieren Sie die Form der Daten, die Sie zurückerhalten möchten
  const schema = {
    type: "object",
    properties: {
      company_name: { type: "string" },
      founded_year: { type: "number" },
      headquarters: { type: "string" }
    },
    required: ["company_name"]
  };

  for await (const message of query({
    prompt: "Research Anthropic and provide key company information",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    // Die Ergebnismeldung enthält structured_output mit validierten Daten
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Definieren Sie die Form der Daten, die Sie zurückerhalten möchten
  schema = {
      "type": "object",
      "properties": {
          "company_name": {"type": "string"},
          "founded_year": {"type": "number"},
          "headquarters": {"type": "string"},
      },
      "required": ["company_name"],
  }


  async def main():
      async for message in query(
          prompt="Research Anthropic and provide key company information",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": schema}
          ),
      ):
          # Die Ergebnismeldung enthält structured_output mit validierten Daten
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  Typsichere Schemas mit Zod und Pydantic
</h2>

Anstatt JSON Schema von Hand zu schreiben, können Sie [Zod](https://zod.dev/) (TypeScript) oder [Pydantic](https://docs.pydantic.dev/latest/) (Python) verwenden, um Ihr Schema zu definieren. Diese Bibliotheken generieren das JSON Schema für Sie und ermöglichen es Ihnen, die Antwort in ein vollständig typisiertes Objekt zu analysieren, das Sie in Ihrer gesamten Codebasis mit Autovervollständigung und Typprüfung verwenden können.

Das folgende Beispiel definiert ein Schema für einen Implementierungsplan für Features mit einer Zusammenfassung, einer Liste von Schritten (jeweils mit Komplexitätsstufe) und potenziellen Risiken. Der Agent plant das Feature und gibt ein typisiertes `FeaturePlan`-Objekt zurück. Sie können dann auf Eigenschaften wie `plan.summary` zugreifen und über `plan.steps` mit vollständiger Typsicherheit iterieren.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Definieren Sie das Schema mit Zod
  const FeaturePlan = z.object({
    feature_name: z.string(),
    summary: z.string(),
    steps: z.array(
      z.object({
        step_number: z.number(),
        description: z.string(),
        estimated_complexity: z.enum(["low", "medium", "high"])
      })
    ),
    risks: z.array(z.string())
  });

  type FeaturePlan = z.infer<typeof FeaturePlan>;

  // Konvertieren Sie zu JSON Schema
  const schema = z.toJSONSchema(FeaturePlan);

  // Verwenden Sie in der Abfrage
  for await (const message of query({
    prompt:
      "Plan how to add dark mode support to a React app. Break it into implementation steps.",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      // Validieren Sie und erhalten Sie ein vollständig typisiertes Ergebnis
      const parsed = FeaturePlan.safeParse(message.structured_output);
      if (parsed.success) {
        const plan: FeaturePlan = parsed.data;
        console.log(`Feature: ${plan.feature_name}`);
        console.log(`Summary: ${plan.summary}`);
        plan.steps.forEach((step) => {
          console.log(`${step.step_number}. [${step.estimated_complexity}] ${step.description}`);
        });
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from pydantic import BaseModel
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  class Step(BaseModel):
      step_number: int
      description: str
      estimated_complexity: str  # 'low', 'medium', 'high'


  class FeaturePlan(BaseModel):
      feature_name: str
      summary: str
      steps: list[Step]
      risks: list[str]


  async def main():
      async for message in query(
          prompt="Plan how to add dark mode support to a React app. Break it into implementation steps.",
          options=ClaudeAgentOptions(
              output_format={
                  "type": "json_schema",
                  "schema": FeaturePlan.model_json_schema(),
              }
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              # Validieren Sie und erhalten Sie ein vollständig typisiertes Ergebnis
              plan = FeaturePlan.model_validate(message.structured_output)
              print(f"Feature: {plan.feature_name}")
              print(f"Summary: {plan.summary}")
              for step in plan.steps:
                  print(
                      f"{step.step_number}. [{step.estimated_complexity}] {step.description}"
                  )


  asyncio.run(main())
  ```
</CodeGroup>

**Vorteile:**

* Vollständige Typinferenz (TypeScript) und Typhinweise (Python)
* Laufzeitvalidierung mit `safeParse()` oder `model_validate()`
* Bessere Fehlermeldungen
* Zusammensetzbare, wiederverwendbare Schemas

<h2 id="output-format-configuration">
  Konfiguration des Ausgabeformats
</h2>

Die Option `outputFormat` (TypeScript) oder `output_format` (Python) akzeptiert ein Objekt mit:

* `type`: Setzen Sie auf `"json_schema"` für strukturierte Ausgaben
* `schema`: Ein [JSON Schema](https://json-schema.org/understanding-json-schema/about)-Objekt, das Ihre Ausgabestruktur definiert. Sie können dies aus einem Zod-Schema mit `z.toJSONSchema()` oder einem Pydantic-Modell mit `.model_json_schema()` generieren

Das SDK unterstützt Standard-JSON-Schema-Funktionen, einschließlich aller grundlegenden Typen (object, array, string, number, boolean, null), `enum`, `const`, `required`, verschachtelte Objekte und `$ref`-Definitionen. Die vollständige Liste der unterstützten Funktionen und Einschränkungen finden Sie unter [JSON Schema-Einschränkungen](https://platform.claude.com/docs/de/build-with-claude/structured-outputs#json-schema-limitations).

Ein Schema, das kein gültiges JSON Schema ist, führt beim Start zu einem Fehler, der das Problem benennt. Vor v2.1.205 wurde ein ungültiges Schema stillschweigend ignoriert und der Agent gab unstrukturierten Text zurück.

Das Schlüsselwort `format`, wie z. B. `"format": "email"`, wird als Anmerkung akzeptiert und wird vom Validator des SDK nicht erzwungen. Vor v2.1.205 wurde jedes Schema, das `format` enthielt, als ungültig behandelt.

<h2 id="example-todo-tracking-agent">
  Beispiel: TODO-Tracking-Agent
</h2>

Dieses Beispiel zeigt, wie strukturierte Ausgaben mit Multi-Step-Tool-Nutzung funktionieren. Der Agent muss TODO-Kommentare in der Codebasis finden und dann für jeden Git-Blame-Informationen nachschlagen. Er entscheidet autonom, welche Tools er verwenden soll (Grep zum Suchen, Bash zum Ausführen von Git-Befehlen) und kombiniert die Ergebnisse in eine einzelne strukturierte Antwort.

Das Schema enthält optionale Felder (`author` und `date`), da Git-Blame-Informationen möglicherweise nicht für alle Dateien verfügbar sind. Der Agent füllt aus, was er finden kann, und lässt den Rest weg.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Definieren Sie die Struktur für TODO-Extraktion
  const todoSchema = {
    type: "object",
    properties: {
      todos: {
        type: "array",
        items: {
          type: "object",
          properties: {
            text: { type: "string" },
            file: { type: "string" },
            line: { type: "number" },
            author: { type: "string" },
            date: { type: "string" }
          },
          required: ["text", "file", "line"]
        }
      },
      total_count: { type: "number" }
    },
    required: ["todos", "total_count"]
  };

  // Agent verwendet Grep zum Finden von TODOs, Bash zum Abrufen von Git-Blame-Informationen
  for await (const message of query({
    prompt: "Find all TODO comments in this codebase and identify who added them",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: todoSchema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      const data = message.structured_output as { total_count: number; todos: Array<{ file: string; line: number; text: string; author?: string; date?: string }> };
      console.log(`Found ${data.total_count} TODOs`);
      data.todos.forEach((todo) => {
        console.log(`${todo.file}:${todo.line} - ${todo.text}`);
        if (todo.author) {
          console.log(`  Added by ${todo.author} on ${todo.date}`);
        }
      });
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Definieren Sie die Struktur für TODO-Extraktion
  todo_schema = {
      "type": "object",
      "properties": {
          "todos": {
              "type": "array",
              "items": {
                  "type": "object",
                  "properties": {
                      "text": {"type": "string"},
                      "file": {"type": "string"},
                      "line": {"type": "number"},
                      "author": {"type": "string"},
                      "date": {"type": "string"},
                  },
                  "required": ["text", "file", "line"],
              },
          },
          "total_count": {"type": "number"},
      },
      "required": ["todos", "total_count"],
  }


  async def main():
      # Agent verwendet Grep zum Finden von TODOs, Bash zum Abrufen von Git-Blame-Informationen
      async for message in query(
          prompt="Find all TODO comments in this codebase and identify who added them",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": todo_schema}
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              data = message.structured_output
              print(f"Found {data['total_count']} TODOs")
              for todo in data["todos"]:
                  print(f"{todo['file']}:{todo['line']} - {todo['text']}")
                  if "author" in todo:
                      print(f"  Added by {todo['author']} on {todo['date']}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Fehlerbehandlung
</h2>

Die Generierung strukturierter Ausgaben kann fehlschlagen, wenn der Agent kein gültiges JSON erzeugen kann, das Ihrem Schema entspricht. Dies geschieht normalerweise, wenn das Schema für die Aufgabe zu komplex ist, die Aufgabe selbst mehrdeutig ist, oder der Agent sein Wiederholungslimit beim Versuch, Validierungsfehler zu beheben, erreicht. Es kann auch ohne Validierungsfehler geschehen: Ein [Modell-Fallback](/docs/de/model-config#automatic-model-fallback) kann eine bereits abgeschlossene Ausgabe mitten im Stream zurückziehen, und wenn kein erfolgreicher Wiederholungsversuch sie ersetzt, endet der Durchlauf mit demselben Fehler. Überprüfen Sie das Feld `errors` der Ergebnismeldung, um die beiden Ursachen auseinanderzuhalten, bevor Sie Ihr Schema debuggen.

Wenn ein Fehler auftritt, hat die Ergebnismeldung einen `subtype`, der angibt, was schief gelaufen ist:

| Subtype                               | Bedeutung                                                                                                                                      |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `success`                             | Ausgabe wurde erfolgreich generiert und validiert                                                                                              |
| `error_max_structured_output_retries` | Keine gültige Ausgabe überstand mehrere Versuche (Validierungsfehler oder ein Modell-Fallback-Rückzug ohne erfolgreichen Wiederholungsversuch) |

Das folgende Beispiel prüft das Feld `subtype`, um zu bestimmen, ob die Ausgabe erfolgreich generiert wurde oder ob Sie einen Fehler behandeln müssen:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const msg of query({
    prompt: "Extract contact info from the document",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: contactSchema
      }
    }
  })) {
    if (msg.type === "result") {
      if (msg.subtype === "success" && msg.structured_output) {
        // Use the validated output
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        // Handle the failure - retry with simpler prompt, fall back to unstructured, etc.
        console.error("Could not produce valid output");
      }
    }
  }
  ```

  ```python Python theme={null}
  async for message in query(
      prompt="Extract contact info from the document",
      options=ClaudeAgentOptions(
          output_format={"type": "json_schema", "schema": contact_schema}
      ),
  ):
      if isinstance(message, ResultMessage):
          if message.subtype == "success" and message.structured_output:
              # Use the validated output
              print(message.structured_output)
          elif message.subtype == "error_max_structured_output_retries":
              # Handle the failure
              print("Could not produce valid output")
  ```
</CodeGroup>

**Tipps zur Vermeidung von Fehlern:**

* **Halten Sie Schemas fokussiert.** Tief verschachtelte Schemas mit vielen erforderlichen Feldern sind schwerer zu erfüllen. Beginnen Sie einfach und fügen Sie Komplexität nach Bedarf hinzu.
* **Passen Sie das Schema an die Aufgabe an.** Wenn die Aufgabe möglicherweise nicht alle Informationen hat, die Ihr Schema erfordert, machen Sie diese Felder optional.
* **Verwenden Sie klare Prompts.** Mehrdeutige Prompts machen es für den Agenten schwieriger zu wissen, welche Ausgabe er erzeugen soll.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [JSON Schema-Dokumentation](https://json-schema.org/): Erfahren Sie mehr über JSON Schema-Syntax zum Definieren komplexer Schemas mit verschachtelten Objekten, Arrays, Enums und Validierungsbeschränkungen
* [API Structured Outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs): Verwenden Sie strukturierte Ausgaben direkt mit der Claude API für Single-Turn-Anfragen ohne Tool-Nutzung
* [Custom tools](/docs/de/agent-sdk/custom-tools): Geben Sie Ihrem Agenten benutzerdefinierte Tools, die während der Ausführung aufgerufen werden können, bevor strukturierte Ausgaben zurückgegeben werden
