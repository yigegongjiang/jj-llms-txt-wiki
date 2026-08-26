> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ottenere output strutturati dagli agenti

> Restituire JSON convalidato dai flussi di lavoro degli agenti utilizzando JSON Schema, Zod o Pydantic. Ottenere dati strutturati e type-safe dopo l'uso di strumenti multi-turno.

Gli output strutturati consentono di definire la forma esatta dei dati che si desidera ottenere da un agente. L'agente può utilizzare qualsiasi strumento necessario per completare l'attività e si ottiene comunque JSON convalidato corrispondente allo schema alla fine. Definire uno [JSON Schema](https://json-schema.org/understanding-json-schema/about) per la struttura necessaria e l'SDK convalida l'output rispetto ad esso, ripetendo la richiesta in caso di mancata corrispondenza. Se la convalida non ha successo entro il limite di tentativi, il risultato è un errore anziché dati strutturati; vedere [Gestione degli errori](#error-handling).

Per la sicurezza dei tipi completa, utilizzare [Zod](#type-safe-schemas-with-zod-and-pydantic) (TypeScript) o [Pydantic](#type-safe-schemas-with-zod-and-pydantic) (Python) per definire lo schema e ottenere oggetti fortemente tipizzati.

<h2 id="why-structured-outputs">
  Perché gli output strutturati?
</h2>

Gli agenti restituiscono testo libero per impostazione predefinita, il che funziona per la chat ma non quando è necessario utilizzare l'output a livello di programmazione. Gli output strutturati forniscono dati tipizzati che è possibile passare direttamente alla logica dell'applicazione, al database o ai componenti dell'interfaccia utente.

Consideriamo un'app di ricette in cui un agente cerca sul web e recupera ricette. Senza output strutturati, si ottiene testo libero che sarebbe necessario analizzare manualmente. Con output strutturati, si definisce la forma desiderata e si ottengono dati tipizzati che è possibile utilizzare direttamente nell'app.

<AccordionGroup>
  <Accordion title="Senza output strutturati">
    ```text theme={null}
    Ecco una ricetta classica di biscotti al cioccolato!

    **Biscotti al cioccolato**
    Tempo di preparazione: 15 minuti | Tempo di cottura: 10 minuti

    Ingredienti:
    - 2 1/4 tazze di farina per tutti gli usi
    - 1 tazza di burro, ammorbidito
    ...
    ```

    Per utilizzare questo nell'app, sarebbe necessario analizzare il titolo, convertire "15 minuti" in un numero, separare gli ingredienti dalle istruzioni e gestire la formattazione incoerente tra le risposte.
  </Accordion>

  <Accordion title="Con output strutturati">
    ```json theme={null}
    {
      "name": "Biscotti al cioccolato",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "farina per tutti gli usi", "amount": 2.25, "unit": "tazze" },
        { "item": "burro, ammorbidito", "amount": 1, "unit": "tazza" }
        // ...
      ],
      "steps": ["Preriscaldare il forno a 375°F", "Mescolare burro e zucchero" /* ... */]
    }
    ```

    Dati tipizzati che è possibile utilizzare direttamente nell'interfaccia utente.
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  Avvio rapido
</h2>

Per utilizzare gli output strutturati, definire uno [JSON Schema](https://json-schema.org/understanding-json-schema/about) che descriva la forma dei dati desiderati, quindi passarlo a `query()` tramite l'opzione `outputFormat` (TypeScript) o `output_format` (Python). Quando l'agente termina, il messaggio di risultato include un campo `structured_output` con dati convalidati corrispondenti allo schema.

L'esempio seguente chiede all'agente di ricercare Anthropic e restituire il nome dell'azienda, l'anno di fondazione e la sede come output strutturato.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Definire la forma dei dati desiderati
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
    prompt: "Ricerca Anthropic e fornisci informazioni chiave sull'azienda",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    // Il messaggio di risultato contiene structured_output con dati convalidati
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Definire la forma dei dati desiderati
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
          prompt="Ricerca Anthropic e fornisci informazioni chiave sull'azienda",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": schema}
          ),
      ):
          # Il messaggio di risultato contiene structured_output con dati convalidati
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  Schema type-safe con Zod e Pydantic
</h2>

Invece di scrivere JSON Schema manualmente, è possibile utilizzare [Zod](https://zod.dev/) (TypeScript) o [Pydantic](https://docs.pydantic.dev/latest/) (Python) per definire lo schema. Queste librerie generano JSON Schema per voi e consentono di analizzare la risposta in un oggetto completamente tipizzato che è possibile utilizzare in tutto il codebase con autocomplete e type checking.

L'esempio seguente definisce uno schema per un piano di implementazione delle funzionalità con un riepilogo, un elenco di passaggi (ciascuno con livello di complessità) e rischi potenziali. L'agente pianifica la funzionalità e restituisce un oggetto `FeaturePlan` tipizzato. È quindi possibile accedere a proprietà come `plan.summary` e iterare su `plan.steps` con sicurezza dei tipi completa.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Definire lo schema con Zod
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

  // Convertire in JSON Schema
  const schema = z.toJSONSchema(FeaturePlan);

  // Utilizzare nella query
  for await (const message of query({
    prompt:
      "Pianifica come aggiungere il supporto della modalità scura a un'app React. Suddividilo in passaggi di implementazione.",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      // Convalidare e ottenere il risultato completamente tipizzato
      const parsed = FeaturePlan.safeParse(message.structured_output);
      if (parsed.success) {
        const plan: FeaturePlan = parsed.data;
        console.log(`Funzionalità: ${plan.feature_name}`);
        console.log(`Riepilogo: ${plan.summary}`);
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
          prompt="Pianifica come aggiungere il supporto della modalità scura a un'app React. Suddividilo in passaggi di implementazione.",
          options=ClaudeAgentOptions(
              output_format={
                  "type": "json_schema",
                  "schema": FeaturePlan.model_json_schema(),
              }
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              # Convalidare e ottenere il risultato completamente tipizzato
              plan = FeaturePlan.model_validate(message.structured_output)
              print(f"Funzionalità: {plan.feature_name}")
              print(f"Riepilogo: {plan.summary}")
              for step in plan.steps:
                  print(
                      f"{step.step_number}. [{step.estimated_complexity}] {step.description}"
                  )


  asyncio.run(main())
  ```
</CodeGroup>

**Vantaggi:**

* Inferenza dei tipi completa (TypeScript) e suggerimenti di tipo (Python)
* Convalida in fase di esecuzione con `safeParse()` o `model_validate()`
* Messaggi di errore migliori
* Schema componibili e riutilizzabili

<h2 id="output-format-configuration">
  Configurazione del formato di output
</h2>

L'opzione `outputFormat` (TypeScript) o `output_format` (Python) accetta un oggetto con:

* `type`: Impostare su `"json_schema"` per gli output strutturati
* `schema`: Un oggetto [JSON Schema](https://json-schema.org/understanding-json-schema/about) che definisce la struttura di output. È possibile generarlo da uno schema Zod con `z.toJSONSchema()` o da un modello Pydantic con `.model_json_schema()`

L'SDK supporta le funzionalità standard di JSON Schema inclusi tutti i tipi di base (object, array, string, number, boolean, null), `enum`, `const`, `required`, oggetti annidati e definizioni `$ref`. Per l'elenco completo delle funzionalità supportate e delle limitazioni, vedere [Limitazioni di JSON Schema](https://platform.claude.com/docs/it/build-with-claude/structured-outputs#json-schema-limitations).

Uno schema che non è un JSON Schema valido non riesce all'avvio con un errore che nomina il problema. Prima della v2.1.205, uno schema non valido veniva silenziosamente ignorato e l'agente restituiva testo non strutturato.

La parola chiave `format`, come `"format": "email"`, viene accettata come annotazione e non viene applicata dal validatore dell'SDK. Prima della v2.1.205, qualsiasi schema contenente `format` veniva trattato come non valido.

<h2 id="example-todo-tracking-agent">
  Esempio: agente di tracciamento TODO
</h2>

Questo esempio dimostra come funzionano gli output strutturati con l'uso di strumenti multi-step. L'agente deve trovare commenti TODO nel codebase, quindi cercare le informazioni di git blame per ciascuno. Decide autonomamente quali strumenti utilizzare (Grep per la ricerca, Bash per eseguire comandi git) e combina i risultati in una singola risposta strutturata.

Lo schema include campi facoltativi (`author` e `date`) poiché le informazioni di git blame potrebbero non essere disponibili per tutti i file. L'agente compila ciò che riesce a trovare e omette il resto.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Definire la struttura per l'estrazione TODO
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

  // L'agente utilizza Grep per trovare TODO, Bash per ottenere informazioni di git blame
  for await (const message of query({
    prompt: "Trova tutti i commenti TODO in questo codebase e identifica chi li ha aggiunti",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: todoSchema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      const data = message.structured_output as { total_count: number; todos: Array<{ file: string; line: number; text: string; author?: string; date?: string }> };
      console.log(`Trovati ${data.total_count} TODO`);
      data.todos.forEach((todo) => {
        console.log(`${todo.file}:${todo.line} - ${todo.text}`);
        if (todo.author) {
          console.log(`  Aggiunto da ${todo.author} il ${todo.date}`);
        }
      });
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Definire la struttura per l'estrazione TODO
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
      # L'agente utilizza Grep per trovare TODO, Bash per ottenere informazioni di git blame
      async for message in query(
          prompt="Trova tutti i commenti TODO in questo codebase e identifica chi li ha aggiunti",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": todo_schema}
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              data = message.structured_output
              print(f"Trovati {data['total_count']} TODO")
              for todo in data["todos"]:
                  print(f"{todo['file']}:{todo['line']} - {todo['text']}")
                  if "author" in todo:
                      print(f"  Aggiunto da {todo['author']} il {todo['date']}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Gestione degli errori
</h2>

La generazione di output strutturati può non riuscire quando l'agente non può produrre JSON valido corrispondente allo schema. Questo accade in genere quando lo schema è troppo complesso per l'attività, l'attività stessa è ambigua o l'agente raggiunge il limite di tentativi cercando di correggere gli errori di convalida. Può anche accadere senza alcun errore di convalida: un [fallback del modello](/docs/it/model-config#automatic-model-fallback) può ritirare un output già completato a metà flusso, e se nessun nuovo tentativo lo sostituisce, l'esecuzione termina con lo stesso errore. Controllare il campo `errors` sul messaggio di risultato per distinguere le due cause prima di eseguire il debug dello schema.

Quando si verifica un errore, il messaggio di risultato ha un `subtype` che indica cosa è andato storto:

| Subtype                               | Significato                                                                                                                                          |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `success`                             | L'output è stato generato e convalidato con successo                                                                                                 |
| `error_max_structured_output_retries` | Nessun output valido è rimasto dopo più tentativi (errori di convalida, o un ritiro dovuto a fallback del modello senza un nuovo tentativo riuscito) |

L'esempio seguente controlla il campo `subtype` per determinare se l'output è stato generato con successo o se è necessario gestire un errore:

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

**Suggerimenti per evitare errori:**

* **Mantenere gli schema focalizzati.** Gli schema profondamente annidati con molti campi obbligatori sono più difficili da soddisfare. Iniziare in modo semplice e aggiungere complessità secondo le necessità.
* **Abbinare lo schema all'attività.** Se l'attività potrebbe non avere tutte le informazioni richieste dallo schema, rendere questi campi facoltativi.
* **Utilizzare prompt chiari.** I prompt ambigui rendono più difficile per l'agente sapere quale output produrre.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Documentazione di JSON Schema](https://json-schema.org/): imparare la sintassi di JSON Schema per definire schema complessi con oggetti annidati, array, enum e vincoli di convalida
* [API Structured Outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs): utilizzare gli output strutturati direttamente con l'API Claude per richieste single-turn senza uso di strumenti
* [Custom tools](/docs/it/agent-sdk/custom-tools): fornire all'agente strumenti personalizzati da chiamare durante l'esecuzione prima di restituire l'output strutturato
