> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Fornisci a Claude strumenti personalizzati

> Definisci strumenti personalizzati con il server MCP in-process dell'Agent SDK di Claude in modo che Claude possa chiamare le tue funzioni, accedere alle tue API ed eseguire operazioni specifiche del dominio.

Gli strumenti personalizzati estendono l'Agent SDK permettendoti di definire le tue funzioni che Claude può chiamare durante una conversazione. Utilizzando il server MCP in-process dell'SDK, puoi dare a Claude accesso a database, API esterne, logica specifica del dominio o qualsiasi altra capacità di cui la tua applicazione ha bisogno.

<h2 id="quick-reference">
  Riferimento rapido
</h2>

| Se vuoi...                                               | Fai questo                                                                                                                                                                                                                           |
| :------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Definire uno strumento                                   | Usa [`@tool`](/docs/it/agent-sdk/python#tool) (Python) o [`tool()`](/docs/it/agent-sdk/typescript#tool) (TypeScript) con un nome, una descrizione, uno schema e un handler. Vedi [Creare uno strumento personalizzato](#create-a-custom-tool). |
| Registrare uno strumento con Claude                      | Avvolgi in `create_sdk_mcp_server` / `createSdkMcpServer` e passa a `mcpServers` in `query()`. Vedi [Chiamare uno strumento personalizzato](#call-a-custom-tool).                                                                    |
| Pre-approvare uno strumento                              | Aggiungi ai tuoi strumenti consentiti. Vedi [Configurare gli strumenti consentiti](#configure-allowed-tools).                                                                                                                        |
| Rimuovere uno strumento integrato dal contesto di Claude | Passa un array `tools` elencando solo gli strumenti integrati che desideri. Vedi [Configurare gli strumenti consentiti](#configure-allowed-tools).                                                                                   |
| Permettere a Claude di chiamare strumenti in parallelo   | Imposta `readOnlyHint: true` su strumenti senza effetti collaterali. Vedi [Aggiungere annotazioni degli strumenti](#add-tool-annotations).                                                                                           |
| Controllare il messaggio di errore che Claude legge      | Restituisci `isError: true` per comporre il messaggio invece di esporre l'eccezione grezza. Vedi [Gestire gli errori](#handle-errors).                                                                                               |
| Restituire immagini o file                               | Usa blocchi `image` o `resource` nell'array di contenuti. Vedi [Restituire immagini e risorse](#return-images-and-resources).                                                                                                        |
| Restituire un risultato JSON leggibile da macchina       | Imposta `structuredContent` sul risultato. Vedi [Restituire dati strutturati](#return-structured-data).                                                                                                                              |
| Scalare a molti strumenti                                | Usa [tool search](/docs/it/agent-sdk/tool-search) per caricare gli strumenti su richiesta.                                                                                                                                                |

<h2 id="create-a-custom-tool">
  Creare uno strumento personalizzato
</h2>

Uno strumento è definito da quattro parti, passate come argomenti al helper [`tool()`](/docs/it/agent-sdk/typescript#tool) in TypeScript o al decoratore [`@tool`](/docs/it/agent-sdk/python#tool) in Python:

* **Nome:** un identificatore univoco che Claude utilizza per chiamare lo strumento.
* **Descrizione:** cosa fa lo strumento. Claude legge questo per decidere quando chiamarlo.
* **Schema di input:** gli argomenti che Claude deve fornire. In TypeScript questo è sempre uno [schema Zod](https://zod.dev/), e gli `args` del handler sono tipizzati automaticamente da esso. In Python questo è un dict che mappa nomi a tipi, come `{"latitude": float}`, che l'SDK converte in JSON Schema per voi. Il decoratore Python accetta anche un dict completo di [JSON Schema](https://json-schema.org/understanding-json-schema/about) direttamente quando avete bisogno di enum, intervalli, campi opzionali o oggetti annidati.
* **Handler:** la funzione asincrona che viene eseguita quando Claude chiama lo strumento. Riceve gli argomenti convalidati e deve restituire un oggetto con:
  * `content` (obbligatorio): un array di blocchi di risultato, ciascuno con un `type` di `"text"`, `"image"`, `"audio"`, `"resource"` o `"resource_link"`. Vedere [Return images and resources](#return-images-and-resources) per i blocchi non testuali.
  * `structuredContent` (opzionale): un oggetto JSON che contiene il risultato come dati leggibili da macchina, restituito insieme a `content`. Vedere [Return structured data](#return-structured-data).
  * `isError` (opzionale): impostare su `true` per segnalare un errore dello strumento in modo che Claude possa reagire. Vedere [Handle errors](#handle-errors).

Dopo aver definito uno strumento, avvolgetelo in un server con [`createSdkMcpServer`](/docs/it/agent-sdk/typescript#createsdkmcpserver) (TypeScript) o [`create_sdk_mcp_server`](/docs/it/agent-sdk/python#create_sdk_mcp_server) (Python). Il server viene eseguito in-process all'interno della vostra applicazione, non come processo separato.

<h3 id="weather-tool-example">
  Esempio di strumento meteo
</h3>

Questo esempio definisce uno strumento `get_temperature` e lo avvolge in un server MCP. Configura solo lo strumento; per passarlo a `query` e eseguirlo, vedere [Call a custom tool](#call-a-custom-tool) di seguito.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  import httpx
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # Define a tool: name, description, input schema, handler
  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
  )
  async def get_temperature(args: dict[str, Any]) -> dict[str, Any]:
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "current": "temperature_2m",
                  "temperature_unit": "fahrenheit",
              },
          )
          data = response.json()

      # Return a content array - Claude sees this as the tool result
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Temperature: {data['current']['temperature_2m']}°F",
              }
          ]
      }


  # Wrap the tool in an in-process MCP server
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  // Define a tool: name, description, input schema, handler
  const getTemperature = tool(
    "get_temperature",
    "Get the current temperature at a location",
    {
      latitude: z.number().describe("Latitude coordinate"), // .describe() adds a field description Claude sees
      longitude: z.number().describe("Longitude coordinate")
    },
    async (args) => {
      // args is typed from the schema: { latitude: number; longitude: number }
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`
      );
      const data: any = await response.json();

      // Return a content array - Claude sees this as the tool result
      return {
        content: [{ type: "text", text: `Temperature: ${data.current.temperature_2m}°F` }]
      };
    }
  );

  // Wrap the tool in an in-process MCP server
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature]
  });
  ```
</CodeGroup>

Vedere il riferimento TypeScript [`tool()`](/docs/it/agent-sdk/typescript#tool) o il riferimento Python [`@tool`](/docs/it/agent-sdk/python#tool) per i dettagli completi dei parametri, inclusi i formati di input JSON Schema e la struttura del valore di ritorno.

<Tip>
  Per rendere un parametro opzionale: in TypeScript, aggiungete `.default()` al campo Zod. In Python, lo schema dict tratta ogni chiave come obbligatoria, quindi omettete il parametro dallo schema, menzionatelo nella stringa di descrizione e leggetelo con `args.get()` nel handler. Lo strumento [`get_precipitation_chance` di seguito](#add-more-tools) mostra entrambi i modelli.
</Tip>

<h3 id="call-a-custom-tool">
  Chiamare uno strumento personalizzato
</h3>

Passate il server MCP che avete creato a `query` tramite l'opzione `mcpServers`. La chiave in `mcpServers` diventa il segmento `{server_name}` nel nome completamente qualificato di ogni strumento: `mcp__{server_name}__{tool_name}`. Elencate quel nome in `allowedTools` in modo che lo strumento venga eseguito senza un prompt di autorizzazione.

Questi snippet riutilizzano il `weatherServer` dall'[esempio di strumento meteo](#weather-tool-example) per chiedere a Claude qual è il meteo in una posizione specifica.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"weather": weather_server},
          allowed_tools=["mcp__weather__get_temperature"],
      )

      async for message in query(
          prompt="What's the temperature in San Francisco?",
          options=options,
      ):
          # ResultMessage is the final message after all tool calls complete
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "What's the temperature in San Francisco?",
    options: {
      mcpServers: { weather: weatherServer },
      allowedTools: ["mcp__weather__get_temperature"]
    }
  })) {
    // "result" is the final message after all tool calls complete
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Combinate questo snippet con le definizioni di strumento e server dall'[esempio di strumento meteo](#weather-tool-example) in un unico file, quindi eseguitelo con `python weather.py` per Python o `npx tsx weather.ts` per TypeScript. Claude chiama `get_temperature` e lo script stampa una risposta su una riga con la temperatura attuale a San Francisco.

<h3 id="add-more-tools">
  Aggiungere più strumenti
</h3>

Un server contiene tutti gli strumenti che elencate nel suo array `tools`. Con più di uno strumento su un server, potete elencare ciascuno in `allowedTools` individualmente o utilizzare il wildcard `mcp__weather__*` per coprire ogni strumento che il server espone.

L'esempio di seguito definisce un secondo strumento, `get_precipitation_chance`, e sostituisce la definizione di `weatherServer` dall'[esempio di strumento meteo](#weather-tool-example) con una che elenca entrambi gli strumenti nell'array.

<CodeGroup>
  ```python Python theme={null}
  # Define a second tool for the same server
  @tool(
      "get_precipitation_chance",
      "Get the hourly precipitation probability for a location. "
      "Optionally pass 'hours' (1-24) to control how many hours to return.",
      {"latitude": float, "longitude": float},
  )
  async def get_precipitation_chance(args: dict[str, Any]) -> dict[str, Any]:
      # 'hours' isn't in the schema - read it with .get() to make it optional
      hours = args.get("hours", 12)
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "hourly": "precipitation_probability",
                  "forecast_days": 1,
              },
          )
          data = response.json()
      chances = data["hourly"]["precipitation_probability"][:hours]

      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Next {hours} hours: {'%, '.join(map(str, chances))}%",
              }
          ]
      }


  # Rebuild the server with both tools in the array
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature, get_precipitation_chance],
  )
  ```

  ```typescript TypeScript theme={null}
  // Define a second tool for the same server
  const getPrecipitationChance = tool(
    "get_precipitation_chance",
    "Get the hourly precipitation probability for a location",
    {
      latitude: z.number(),
      longitude: z.number(),
      hours: z
        .number()
        .int()
        .min(1)
        .max(24)
        .default(12) // .default() makes the parameter optional
        .describe("How many hours of forecast to return")
    },
    async (args) => {
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&hourly=precipitation_probability&forecast_days=1`
      );
      const data: any = await response.json();
      const chances = data.hourly.precipitation_probability.slice(0, args.hours);

      return {
        content: [{ type: "text", text: `Next ${args.hours} hours: ${chances.join("%, ")}%` }]
      };
    }
  );

  // Rebuild the server with both tools in the array
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature, getPrecipitationChance]
  });
  ```
</CodeGroup>

[Tool search](/docs/it/agent-sdk/tool-search) è abilitato per impostazione predefinita e rinvia gli strumenti SDK MCP: Claude vede il nome di ogni strumento in un elenco compatto e carica il suo schema completo su richiesta. Con la ricerca degli strumenti disabilitata, ogni strumento in questo array consuma spazio della finestra di contesto ad ogni turno. In TypeScript, passate `alwaysLoad: true` nell'argomento `extras` di [`tool()`](/docs/it/agent-sdk/typescript#tool) o nelle opzioni di [`createSdkMcpServer()`](/docs/it/agent-sdk/typescript#createsdkmcpserver) per mantenere lo schema completo di uno strumento nel prompt iniziale.

<h3 id="add-tool-annotations">
  Aggiungere annotazioni di strumento
</h3>

Le [annotazioni di strumento](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations) sono metadati opzionali che descrivono il comportamento di uno strumento. Passateli come quinto argomento al helper `tool()` in TypeScript o tramite l'argomento della parola chiave `annotations` per il decoratore `@tool` in Python. Tutti i campi di suggerimento sono booleani.

| Campo             | Predefinito | Significato                                                                                                                                |
| :---------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `readOnlyHint`    | `false`     | Lo strumento non modifica il suo ambiente. Controlla se lo strumento può essere chiamato in parallelo con altri strumenti di sola lettura. |
| `destructiveHint` | `true`      | Lo strumento può eseguire aggiornamenti distruttivi. Solo informativo.                                                                     |
| `idempotentHint`  | `false`     | Le chiamate ripetute con gli stessi argomenti non hanno effetti aggiuntivi. Solo informativo.                                              |
| `openWorldHint`   | `true`      | Lo strumento raggiunge sistemi al di fuori del vostro processo. Solo informativo.                                                          |

Le annotazioni sono metadati, non applicazione. Uno strumento contrassegnato con `readOnlyHint: true` può comunque scrivere su disco se è quello che fa il handler. Mantenete l'annotazione accurata rispetto al handler.

Questo esempio aggiunge `readOnlyHint` allo strumento `get_temperature` dall'[esempio di strumento meteo](#weather-tool-example).

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import tool, ToolAnnotations


  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
      annotations=ToolAnnotations(
          readOnlyHint=True
      ),  # Lets Claude batch this with other read-only calls
  )
  async def get_temperature(args):
      return {"content": [{"type": "text", "text": "..."}]}
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

Vedere `ToolAnnotations` nel riferimento [TypeScript](/docs/it/agent-sdk/typescript#toolannotations) o [Python](/docs/it/agent-sdk/python#toolannotations).

<h2 id="control-tool-access">
  Controllare l'accesso ai tool
</h2>

L'[esempio di tool meteo](#weather-tool-example) ha registrato un server e ha elencato i tool in `allowedTools`. Questa sezione illustra come limitare l'accesso quando si hanno più tool o si desidera limitare i tool integrati. Per informazioni su come vengono costruiti i nomi dei tool, vedere [Chiamare un tool personalizzato](#call-a-custom-tool).

<h3 id="configure-allowed-tools">
  Configurare i tool consentiti
</h3>

L'opzione `tools` e gli elenchi consentiti/non consentiti influiscono su due livelli: la disponibilità, che controlla se un tool appare nel contesto di Claude, e il permesso, che controlla se una chiamata viene approvata una volta che Claude la tenta. `tools` e le voci `disallowedTools` con nome semplice modificano la disponibilità. `allowedTools` e le regole `disallowedTools` con ambito modificano il permesso. Se si nomina uno dei [tool di tracciamento delle attività](/docs/it/agent-sdk/todo-tracking#model-availability) in `allowedTools`, Claude Code opta anche la sessione.

| Opzione                   | Livello       | Effetto                                                                                                                                                                                                                                                                        |
| :------------------------ | :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools: ["Read", "Grep"]` | Disponibilità | Solo i tool integrati elencati sono nel contesto di Claude. I tool integrati non elencati vengono rimossi. I tool MCP non sono interessati.                                                                                                                                    |
| `tools: []`               | Disponibilità | Tutti i tool integrati vengono rimossi. Claude può utilizzare solo i vostri tool MCP.                                                                                                                                                                                          |
| tool consentiti           | Permesso      | I tool elencati vengono eseguiti senza un prompt di permesso. Gli altri tool non elencati rimangono disponibili; le chiamate passano attraverso il [flusso di permesso](/docs/it/agent-sdk/permissions).                                                                            |
| tool non consentiti       | Entrambi      | Un nome di tool semplice come `"Bash"` rimuove il tool dal contesto di Claude, come se lo ometteste da `tools`. Una regola con ambito come `"Bash(rm *)"` lascia il tool nel contesto e nega solo le chiamate corrispondenti [come scritto](/docs/it/permissions#bash-rule-limits). |

Per rimuovere completamente un tool integrato, omettetelo da `tools` o elencate il suo nome semplice in `disallowedTools` (Python: `disallowed_tools`); entrambi mantengono il tool fuori dal contesto in modo che Claude non lo tenti mai. Una regola `disallowedTools` con ambito blocca le chiamate corrispondenti ma lascia il tool visibile, quindi Claude potrebbe sprecare un turno tentandolo. Vedere [Configurare i permessi](/docs/it/agent-sdk/permissions) per l'ordine di valutazione completo.

<h2 id="handle-errors">
  Gestire gli errori
</h2>

Un errore del handler non interrompe il ciclo dell'agente. Il server MCP in-process dell'SDK cattura le eccezioni non gestite e le restituisce come risultati di errore, quindi il modo in cui segnalate un errore determina ciò che Claude legge, non se la query fallisce:

| Cosa accade                                                                               | Risultato                                                                                                                                                             |
| :---------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Handler genera un'eccezione non gestita                                                   | Il server MCP la converte in un risultato di errore contenente il messaggio di eccezione grezzo. Claude vede quel messaggio e il ciclo dell'agente continua.          |
| Handler cattura l'errore e restituisce `isError: true` (TS) / `"is_error": True` (Python) | Claude vede il messaggio che componete. Potete aggiungere contesto che il messaggio di eccezione grezzo non ha, come quale richiesta è fallita o cosa provare invece. |

In entrambi i casi Claude può riprovare, provare uno strumento diverso o spiegare il fallimento. Catturate gli errori voi stessi quando il messaggio di eccezione grezzo non è sufficiente per Claude per agire.

L'esempio seguente cattura due tipi di fallimenti all'interno del handler e compone il messaggio di errore che Claude legge. Uno stato HTTP non-200 viene catturato dalla risposta e restituito come risultato di errore. Un errore di rete o JSON non valido viene catturato dal `try/except` (Python) o `try/catch` (TypeScript) circostante e anch'esso restituito come risultato di errore. In entrambi i casi Claude riceve un messaggio che descrive il fallimento invece di una stringa di eccezione nuda.

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any
  from claude_agent_sdk import tool


  @tool(
      "fetch_data",
      "Fetch data from an API",
      {"endpoint": str},  # Simple schema
  )
  async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
      try:
          async with httpx.AsyncClient() as client:
              response = await client.get(args["endpoint"])
              if response.status_code != 200:
                  # Return the failure as a tool result so Claude can react to it.
                  # is_error marks this as a failed call rather than odd-looking data.
                  return {
                      "content": [
                          {
                              "type": "text",
                              "text": f"API error: {response.status_code} {response.reason_phrase}",
                          }
                      ],
                      "is_error": True,
                  }

              data = response.json()
              return {"content": [{"type": "text", "text": json.dumps(data, indent=2)}]}
      except Exception as e:
          # Composes the message Claude reads. An uncaught exception would
          # reach Claude as the raw str(e) with no context.
          return {
              "content": [{"type": "text", "text": f"Failed to fetch data: {str(e)}"}],
              "is_error": True,
          }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_data",
    "Fetch data from an API",
    {
      endpoint: z.string().url().describe("API endpoint URL")
    },
    async (args) => {
      try {
        const response = await fetch(args.endpoint);

        if (!response.ok) {
          // Return the failure as a tool result so Claude can react to it.
          // isError marks this as a failed call rather than odd-looking data.
          return {
            content: [
              {
                type: "text",
                text: `API error: ${response.status} ${response.statusText}`
              }
            ],
            isError: true
          };
        }

        const data = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(data, null, 2)
            }
          ]
        };
      } catch (error) {
        // Composes the message Claude reads. An uncaught throw would
        // reach Claude as the raw error message with no context.
        return {
          content: [
            {
              type: "text",
              text: `Failed to fetch data: ${error instanceof Error ? error.message : String(error)}`
            }
          ],
          isError: true
        };
      }
    }
  );
  ```
</CodeGroup>

<h2 id="return-images-and-resources">
  Restituire immagini e risorse
</h2>

L'array `content` in un risultato di tool accetta blocchi `text`, `image`, `audio`, `resource` e `resource_link`. È possibile mescolarli nella stessa risposta. In TypeScript, l'SDK salva i blocchi audio su disco e Claude riceve un blocco di testo con il percorso del file salvato; in Python, l'SDK elimina i blocchi audio dal risultato del tool e registra un avviso.

Claude riceve ogni blocco di collegamento a risorse come un blocco di testo contenente il nome, l'URI e la descrizione del collegamento. In TypeScript, l'applicazione riceve anche i collegamenti stessi come [`resourceLinks`](/docs/it/agent-sdk/typescript#sdkmcpresourcelink) sul `tool_use_result` del messaggio dell'utente; in Python, l'SDK li appiattisce a testo prima che la CLI veda il risultato, quindi la chiave Python [`resourceLinks`](/docs/it/agent-sdk/python#usermessage) non viene mai prodotta per i tool in-process.

<h3 id="images">
  Immagini
</h3>

Un blocco immagine contiene i byte dell'immagine inline, codificati in base64. Non esiste un campo URL. Per restituire un'immagine che si trova in un URL, recuperarla nel gestore, leggere i byte della risposta e codificarli in base64 prima di restituirli. Il risultato viene elaborato come input visivo.

| Campo      | Tipo      | Note                                                                                   |
| :--------- | :-------- | :------------------------------------------------------------------------------------- |
| `type`     | `"image"` |                                                                                        |
| `data`     | `string`  | Byte codificati in base64. Solo base64 grezzo, senza prefisso `data:image/...;base64,` |
| `mimeType` | `string`  | Obbligatorio. Ad esempio `image/png`, `image/jpeg`, `image/webp`, `image/gif`          |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx
  from claude_agent_sdk import tool


  # Define a tool that fetches an image from a URL and returns it to Claude
  @tool("fetch_image", "Fetch an image from a URL and return it to Claude", {"url": str})
  async def fetch_image(args):
      async with httpx.AsyncClient() as client:  # Fetch the image bytes
          response = await client.get(args["url"])

      return {
          "content": [
              {
                  "type": "image",
                  "data": base64.b64encode(response.content).decode(
                      "ascii"
                  ),  # Base64-encode the raw bytes
                  "mimeType": response.headers.get(
                      "content-type", "image/png"
                  ),  # Read MIME type from the response
              }
          ]
      }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_image",
    "Fetch an image from a URL and return it to Claude",
    {
      url: z.string().url()
    },
    async (args) => {
      const response = await fetch(args.url); // Fetch the image bytes
      const buffer = Buffer.from(await response.arrayBuffer()); // Read into a Buffer for base64 encoding
      const mimeType = response.headers.get("content-type") ?? "image/png";

      return {
        content: [
          {
            type: "image",
            data: buffer.toString("base64"), // Base64-encode the raw bytes
            mimeType
          }
        ]
      };
    }
  );
  ```
</CodeGroup>

<h3 id="resources">
  Risorse
</h3>

Un blocco risorsa incorpora un pezzo di contenuto identificato da un URI. L'URI è un'etichetta per Claude a cui fare riferimento; il contenuto effettivo si trova nel campo `text` o `blob` del blocco. Utilizzare questa opzione quando il tool produce qualcosa che ha senso affrontare per nome in seguito, come un file generato o un record da un sistema esterno.

| Campo               | Tipo         | Note                                                                                                                                                  |
| :------------------ | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                                                                                       |
| `resource.uri`      | `string`     | Identificatore per il contenuto. Qualsiasi schema URI                                                                                                 |
| `resource.text`     | `string`     | Il contenuto, se è testo. Fornire questo o `blob`, non entrambi                                                                                       |
| `resource.blob`     | `string`     | Il contenuto codificato in base64, se è binario. Solo TypeScript: l'SDK Python elimina le risorse binarie dal risultato del tool e registra un avviso |
| `resource.mimeType` | `string`     | Facoltativo                                                                                                                                           |

Questo esempio mostra un blocco risorsa restituito dall'interno di un gestore di tool. L'URI `file:///tmp/report.md` è un'etichetta a cui Claude può fare riferimento in seguito; l'SDK non legge da quel percorso.

<CodeGroup>
  ```typescript TypeScript theme={null}
  return {
    content: [
      {
        type: "resource",
        resource: {
          uri: "file:///tmp/report.md", // Label for Claude to reference, not a path the SDK reads
          mimeType: "text/markdown",
          text: "# Report\n..." // The actual content, inline
        }
      }
    ]
  };
  ```

  ```python Python theme={null}
  return {
      "content": [
          {
              "type": "resource",
              "resource": {
                  "uri": "file:///tmp/report.md",  # Label for Claude to reference, not a path the SDK reads
                  "mimeType": "text/markdown",
                  "text": "# Report\n...",  # The actual content, inline
              },
          }
      ]
  }
  ```
</CodeGroup>

Queste forme di blocco provengono dal tipo MCP `CallToolResult`. Consultare la [specifica MCP](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result) per la definizione completa.

<h2 id="return-structured-data">
  Restituire dati strutturati
</h2>

`structuredContent` è un oggetto JSON opzionale sul risultato, separato dall'array `content`. Utilizzarlo per restituire valori grezzi che Claude può leggere come campi esatti invece di analizzarli da una stringa di testo o da un'immagine.

Quando `structuredContent` è impostato, Claude riceve il JSON più eventuali blocchi di immagine o risorsa da `content`. I blocchi di testo in `content` non vengono inoltrati, poiché si presume che duplichino i dati strutturati. L'esempio seguente renderizza un grafico come blocco di immagine e restituisce i punti dati dietro di esso in `structuredContent` dallo stesso handler. Nel frammento, `chartPngBuffer` è un `Buffer` che contiene i byte PNG renderizzati.

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "image",
      data: chartPngBuffer.toString("base64"),
      mimeType: "image/png"
    }
  ],
  structuredContent: {
    series: "temperature_2m",
    unit: "fahrenheit",
    points: [62.1, 63.4, 65.0, 64.2]
  }
};
```

<Note>
  Il decoratore Python `@tool` inoltra solo `content` e `is_error` dal dizionario di ritorno dell'handler. Per restituire `structuredContent` da Python, eseguire un [server MCP standalone](/docs/it/agent-sdk/mcp) invece di un server SDK in-process.
</Note>

<h2 id="example-unit-converter">
  Esempio: convertitore di unità
</h2>

Questo strumento converte valori tra unità di lunghezza, temperatura e peso. Un utente può chiedere "converti 100 chilometri in miglia" o "quanto sono 72°F in Celsius", e Claude sceglie il tipo di unità e le unità corrette dalla richiesta.

Dimostra due pattern:

* **Schemi enum:** `unit_type` è vincolato a un insieme fisso di valori. In TypeScript, usa `z.enum()`. In Python, lo schema dict non supporta gli enum, quindi è richiesto lo schema JSON completo.
* **Gestione dell'input non supportato:** quando una coppia di conversione non viene trovata, il gestore restituisce `isError: true` in modo che Claude possa dire all'utente cosa è andato storto piuttosto che trattare un fallimento come un risultato normale.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # z.enum() in TypeScript becomes an "enum" constraint in JSON Schema.
  # The dict schema has no equivalent, so full JSON Schema is required.
  @tool(
      "convert_units",
      "Convert a value from one unit to another",
      {
          "type": "object",
          "properties": {
              "unit_type": {
                  "type": "string",
                  "enum": ["length", "temperature", "weight"],
                  "description": "Category of unit",
              },
              "from_unit": {
                  "type": "string",
                  "description": "Unit to convert from, e.g. kilometers, fahrenheit, pounds",
              },
              "to_unit": {"type": "string", "description": "Unit to convert to"},
              "value": {"type": "number", "description": "Value to convert"},
          },
          "required": ["unit_type", "from_unit", "to_unit", "value"],
      },
  )
  async def convert_units(args: dict[str, Any]) -> dict[str, Any]:
      conversions = {
          "length": {
              "kilometers_to_miles": lambda v: v * 0.621371,
              "miles_to_kilometers": lambda v: v * 1.60934,
              "meters_to_feet": lambda v: v * 3.28084,
              "feet_to_meters": lambda v: v * 0.3048,
          },
          "temperature": {
              "celsius_to_fahrenheit": lambda v: (v * 9) / 5 + 32,
              "fahrenheit_to_celsius": lambda v: (v - 32) * 5 / 9,
              "celsius_to_kelvin": lambda v: v + 273.15,
              "kelvin_to_celsius": lambda v: v - 273.15,
          },
          "weight": {
              "kilograms_to_pounds": lambda v: v * 2.20462,
              "pounds_to_kilograms": lambda v: v * 0.453592,
              "grams_to_ounces": lambda v: v * 0.035274,
              "ounces_to_grams": lambda v: v * 28.3495,
          },
      }

      key = f"{args['from_unit']}_to_{args['to_unit']}"
      fn = conversions.get(args["unit_type"], {}).get(key)

      if not fn:
          return {
              "content": [
                  {
                      "type": "text",
                      "text": f"Unsupported conversion: {args['from_unit']} to {args['to_unit']}",
                  }
              ],
              "is_error": True,
          }

      result = fn(args["value"])
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"{args['value']} {args['from_unit']} = {result:.4f} {args['to_unit']}",
              }
          ]
      }


  converter_server = create_sdk_mcp_server(
      name="converter",
      version="1.0.0",
      tools=[convert_units],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  const convert = tool(
    "convert_units",
    "Convert a value from one unit to another",
    {
      unit_type: z.enum(["length", "temperature", "weight"]).describe("Category of unit"),
      from_unit: z
        .string()
        .describe("Unit to convert from, e.g. kilometers, fahrenheit, pounds"),
      to_unit: z.string().describe("Unit to convert to"),
      value: z.number().describe("Value to convert")
    },
    async (args) => {
      type Conversions = Record<string, Record<string, (v: number) => number>>;

      const conversions: Conversions = {
        length: {
          kilometers_to_miles: (v) => v * 0.621371,
          miles_to_kilometers: (v) => v * 1.60934,
          meters_to_feet: (v) => v * 3.28084,
          feet_to_meters: (v) => v * 0.3048
        },
        temperature: {
          celsius_to_fahrenheit: (v) => (v * 9) / 5 + 32,
          fahrenheit_to_celsius: (v) => ((v - 32) * 5) / 9,
          celsius_to_kelvin: (v) => v + 273.15,
          kelvin_to_celsius: (v) => v - 273.15
        },
        weight: {
          kilograms_to_pounds: (v) => v * 2.20462,
          pounds_to_kilograms: (v) => v * 0.453592,
          grams_to_ounces: (v) => v * 0.035274,
          ounces_to_grams: (v) => v * 28.3495
        }
      };

      const key = `${args.from_unit}_to_${args.to_unit}`;
      const fn = conversions[args.unit_type]?.[key];

      if (!fn) {
        return {
          content: [
            {
              type: "text",
              text: `Unsupported conversion: ${args.from_unit} to ${args.to_unit}`
            }
          ],
          isError: true
        };
      }

      const result = fn(args.value);
      return {
        content: [
          {
            type: "text",
            text: `${args.value} ${args.from_unit} = ${result.toFixed(4)} ${args.to_unit}`
          }
        ]
      };
    }
  );

  const converterServer = createSdkMcpServer({
    name: "converter",
    version: "1.0.0",
    tools: [convert]
  });
  ```
</CodeGroup>

Una volta definito il server, passalo a `query` nello stesso modo dell'esempio meteo. Questo esempio invia tre prompt diversi in un ciclo per mostrare lo stesso strumento che gestisce diversi tipi di unità. Per ogni risposta, ispeziona gli oggetti `AssistantMessage` (che contengono le chiamate di strumento che Claude ha effettuato durante quel turno) e stampa ogni `ToolUseBlock` prima di stampare il testo finale di `ResultMessage`. Questo ti permette di vedere quando Claude sta usando lo strumento rispetto a quando sta rispondendo dalle sue conoscenze.

Poiché [tool search](/docs/it/agent-sdk/tool-search) è attivato per impostazione predefinita, l'output può includere anche una chiamata `ToolSearch` mentre Claude carica lo schema dello strumento differito.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      AssistantMessage,
      ToolUseBlock,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"converter": converter_server},
          allowed_tools=["mcp__converter__convert_units"],
      )

      prompts = [
          "Convert 100 kilometers to miles.",
          "What is 72°F in Celsius?",
          "How many pounds is 5 kilograms?",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt, options=options):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              print(f"[tool call] {block.name}({block.input})")
                  elif isinstance(message, ResultMessage) and message.subtype == "success":
                      print(f"Q: {prompt}\nA: {message.result}\n")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. Only success
              # results are printed above, so handle the failure here and continue with
              # the next prompt.
              print(f"Call failed: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompts = [
    "Convert 100 kilometers to miles.",
    "What is 72°F in Celsius?",
    "How many pounds is 5 kilograms?"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({
        prompt,
        options: {
          mcpServers: { converter: converterServer },
          allowedTools: ["mcp__converter__convert_units"]
        }
      })) {
        if (message.type === "assistant") {
          for (const block of message.message.content) {
            if (block.type === "tool_use") {
              console.log(`[tool call] ${block.name}`, block.input);
            }
          }
        } else if (message.type === "result" && message.subtype === "success") {
          console.log(`Q: ${prompt}\nA: ${message.result}\n`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. Only success
      // results are logged above, so handle the failure here and continue with
      // the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Passaggi successivi
</h2>

È possibile combinare i modelli su questa pagina nello stesso server: un singolo server può contenere uno strumento di database, uno strumento gateway API e un renderer di immagini uno accanto all'altro.

Da qui:

* Se il vostro server cresce fino a dozzine di strumenti, consultate [tool search](/docs/it/agent-sdk/tool-search) per rinviare il caricamento fino a quando Claude non ne avrà bisogno.
* Per connettersi a server MCP esterni (filesystem, GitHub, Slack) invece di costruire i vostri, consultate [Connect MCP servers](/docs/it/agent-sdk/mcp).
* Per controllare quali strumenti vengono eseguiti automaticamente rispetto a quelli che richiedono approvazione, consultate [Configure permissions](/docs/it/agent-sdk/permissions).
