> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Scalare a molti strumenti con la ricerca di strumenti

> Scalare il vostro agente a migliaia di strumenti scoprendo e caricando solo ciò che è necessario, su richiesta.

La ricerca di strumenti consente al vostro agente di lavorare con centinaia o migliaia di strumenti scoprendo e caricando dinamicamente solo quelli di cui ha bisogno. Invece di caricare tutte le definizioni degli strumenti nella finestra di contesto in anticipo, l'agente cerca nel vostro catalogo di strumenti e carica solo gli strumenti di cui ha bisogno.

Questo approccio risolve due sfide man mano che le librerie di strumenti si scalano:

* **Efficienza del contesto:** Le definizioni degli strumenti possono consumare grandi porzioni della finestra di contesto (50 strumenti possono utilizzare 10-20K token), lasciando meno spazio per il lavoro effettivo.
* **Accuratezza della selezione degli strumenti:** L'accuratezza della selezione degli strumenti si degrada con più di 30-50 strumenti caricati contemporaneamente.

La ricerca di strumenti è abilitata per impostazione predefinita.

<h2 id="how-tool-search-works">
  Come funziona la ricerca di strumenti
</h2>

Quando la ricerca di strumenti è attiva, le definizioni degli strumenti vengono trattenute dalla finestra di contesto. L'agente riceve un riepilogo degli strumenti disponibili e cerca quelli rilevanti quando l'attività richiede una capacità non già caricata. Fino a cinque dei più rilevanti strumenti vengono caricati nel contesto per impostazione predefinita, dove rimangono disponibili per i turni successivi. Se la conversazione è abbastanza lunga da far sì che l'SDK compatti i messaggi precedenti per liberare spazio, gli strumenti precedentemente scoperti possono essere rimossi e l'agente ricerca di nuovo secondo le necessità.

La ricerca di strumenti aggiunge un extra round-trip la prima volta che Claude scopre uno strumento (il passaggio di ricerca), ma per grandi set di strumenti questo è compensato da un contesto più piccolo ad ogni turno. Con meno di \~10 strumenti, il caricamento di tutto in anticipo è generalmente più veloce.

Per i dettagli sul meccanismo API sottostante, vedere [Ricerca di strumenti nell'API](https://platform.claude.com/docs/it/agents-and-tools/tool-use/tool-search-tool).

<Note>
  La ricerca di strumenti è supportata su Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 e modelli successivi; vedere [compatibilità dei modelli nella documentazione API](https://platform.claude.com/docs/it/agents-and-tools/tool-use/tool-search-tool#model-compatibility) per l'elenco attuale. Su Google Cloud's Agent Platform, i modelli supportati minimi sono Claude Sonnet 4.5 e Claude Opus 4.5.
</Note>

<h2 id="configure-tool-search">
  Configurare la ricerca di strumenti
</h2>

La ricerca di strumenti è attiva per impostazione predefinita. È disabilitata per impostazione predefinita su Google Cloud's Agent Platform, dove è supportata per Claude Sonnet 4.5 e successivo e Claude Opus 4.5 e successivo. È anche disabilitata quando `ANTHROPIC_BASE_URL` punta a un host non di prima parte, poiché la maggior parte dei proxy non inoltrano i blocchi `tool_reference`. Potete ignorare uno qualsiasi dei valori predefiniti con la variabile di ambiente `ENABLE_TOOL_SEARCH`:

| Valore          | Comportamento                                                                                                                                                                                                                                                                                          |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (non impostato) | La ricerca di strumenti è attiva. Le definizioni degli strumenti vengono differite e scoperte su richiesta. Ritorna al caricamento in anticipo su Google Cloud's Agent Platform o su un `ANTHROPIC_BASE_URL` non di prima parte.                                                                       |
| `true`          | La ricerca di strumenti è sempre attiva. L'SDK invia l'intestazione beta anche su Google Cloud's Agent Platform e attraverso i proxy. Le richieste falliscono sui modelli Google Cloud's Agent Platform precedenti a Sonnet 4.5 o Opus 4.5, o sui proxy che non supportano i blocchi `tool_reference`. |
| `auto`          | Controlla il conteggio dei token combinato di tutte le definizioni degli strumenti rispetto alla finestra di contesto del modello. Se superano il 10%, la ricerca di strumenti si attiva. Se sono sotto il 10%, tutti gli strumenti vengono caricati nel contesto normalmente.                         |
| `auto:N`        | Come `auto` con una percentuale personalizzata. `auto:5` si attiva quando le definizioni degli strumenti superano il 5% della finestra di contesto. Valori più bassi si attivano prima.                                                                                                                |
| `false`         | La ricerca di strumenti è disattivata. Tutte le definizioni degli strumenti vengono caricate nel contesto ad ogni turno.                                                                                                                                                                               |

L'impostazione [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/it/env-vars) mantiene la ricerca di strumenti disattivata, e `ENABLE_TOOL_SEARCH` non può ignorarla. La variabile rimuove l'intestazione beta che le definizioni degli strumenti `defer_loading` e i blocchi di contenuto `tool_reference` richiedono.

La ricerca di strumenti si applica a tutti gli strumenti registrati, che provengano da server MCP remoti o da [server MCP SDK personalizzati](/docs/it/agent-sdk/custom-tools). Quando si utilizza `auto`, la soglia si basa sulla dimensione combinata di tutte le definizioni degli strumenti su tutti i server.

Impostare il valore nell'opzione `env` su `query()`. In TypeScript, `env` sostituisce l'ambiente del sottoprocesso, quindi diffondere `...process.env` per mantenere le variabili ereditate. In Python, `env` viene unito sopra l'ambiente ereditato. Questo esempio si connette a un server MCP remoto che espone molti strumenti, pre-approva tutti loro con un carattere jolly e utilizza `auto:5` in modo che la ricerca di strumenti si attivi quando le loro definizioni superano il 5% della finestra di contesto:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Find and run the appropriate database query",
      options: {
        mcpServers: {
          "enterprise-tools": {
            // Connect to a remote MCP server
            type: "http",
            url: "https://tools.example.com/mcp"
          }
        },
        allowedTools: ["mcp__enterprise-tools__*"], // Wildcard pre-approves all tools from this server
        env: {
          ...process.env, // env replaces the subprocess environment, so keep inherited variables
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when tools exceed 5% of context
        }
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "enterprise-tools": {
                  "type": "http",
                  "url": "https://tools.example.com/mcp",
              }
          },
          allowed_tools=[
              "mcp__enterprise-tools__*"
          ],  # Wildcard pre-approves all tools from this server
          env={
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when tools exceed 5% of context
          },
      )

      try:
          async for message in query(
              prompt="Find and run the appropriate database query",
              options=options,
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Per eseguire questo esempio, sostituire `https://tools.example.com/mcp` con l'URL del vostro server MCP. In caso di successo, il testo del risultato viene stampato sulla console.

Poiché si tratta di una chiamata `query()` a singolo scatto, l'SDK genera un'eccezione dopo aver restituito un risultato di errore, quindi l'esempio racchiude il ciclo in un blocco try. Per vedere perché un'esecuzione non è riuscita, controllare il `subtype` del messaggio di risultato, come `error_during_execution`, all'interno del ciclo. Per ulteriori informazioni sui messaggi di risultato, vedere [Gestire il risultato](/docs/it/agent-sdk/agent-loop#handle-the-result).

Impostare `ENABLE_TOOL_SEARCH` su `"false"` disabilita la ricerca di strumenti e carica tutte le definizioni degli strumenti nel contesto ad ogni turno. Questo rimuove il round-trip di ricerca, che può essere più veloce quando il set di strumenti è piccolo (meno di \~10 strumenti) e le definizioni si adattano comodamente nella finestra di contesto.

<h2 id="optimize-tool-discovery">
  Ottimizzare la scoperta degli strumenti
</h2>

Il meccanismo di ricerca abbina le query ai nomi e alle descrizioni degli strumenti. Nomi come `search_slack_messages` emergono per una gamma più ampia di richieste rispetto a `query_slack`. Le descrizioni con parole chiave specifiche ("Cerca messaggi Slack per parola chiave, canale o intervallo di date") corrispondono a più query rispetto a quelle generiche ("Query Slack").

Potete anche aggiungere una sezione di prompt di sistema che elenca le categorie di strumenti disponibili. Questo dà all'agente il contesto su quali tipi di strumenti sono disponibili per la ricerca. Passate il testo attraverso l'opzione `systemPrompt` in TypeScript o `system_prompt` in Python, utilizzando il preset `claude_code` con `append`, che aggiunge il vostro testo al prompt del preset invece di sostituirlo:

<CodeGroup>
  ```typescript TypeScript theme={null}
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: "You can search for tools to interact with Slack, GitHub, and Jira."
    }
  }
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      system_prompt={
          "type": "preset",
          "preset": "claude_code",
          "append": "You can search for tools to interact with Slack, GitHub, and Jira.",
      }
  )
  ```
</CodeGroup>

Per l'insieme completo delle opzioni di prompt di sistema, consultate [Modifying system prompts](/docs/it/agent-sdk/modifying-system-prompts).

<h2 id="limits">
  Limiti
</h2>

* **Strumenti massimi:** 10.000 strumenti nel vostro catalogo
* **Risultati di ricerca:** restituisce fino a cinque strumenti più rilevanti per ricerca per impostazione predefinita
* **Supporto del modello:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 e modelli successivi; consultare la [compatibilità del modello nella documentazione API](https://platform.claude.com/docs/it/agents-and-tools/tool-use/tool-search-tool#model-compatibility) per l'elenco attuale. Su Google Cloud's Agent Platform, Claude Sonnet 4.5 e successivi e Claude Opus 4.5 e successivi.

<h2 id="related-documentation">
  Documentazione correlata
</h2>

* [Ricerca di strumenti nell'API](https://platform.claude.com/docs/it/agents-and-tools/tool-use/tool-search-tool): Documentazione API completa per la ricerca di strumenti, incluse implementazioni personalizzate
* [Connettere server MCP](/docs/it/agent-sdk/mcp): Connettere a strumenti esterni tramite server MCP
* [Strumenti personalizzati](/docs/it/agent-sdk/custom-tools): Creare i vostri strumenti con server MCP SDK
* [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript): Riferimento API completo
* [Riferimento SDK Python](/docs/it/agent-sdk/python): Riferimento API completo
