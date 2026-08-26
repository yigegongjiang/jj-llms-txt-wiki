> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guida rapida

> Inizia con l'Agent SDK per Python o TypeScript per creare agenti AI che funzionano autonomamente

Utilizza l'Agent SDK per creare un agente AI che legge il tuo codice, trova i bug e li corregge, il tutto senza intervento manuale.

**Quello che farai:**

1. Configurare un progetto con l'Agent SDK
2. Creare un file con del codice buggy
3. Eseguire un agente che trova e corregge automaticamente i bug

<h2 id="prerequisites">
  Prerequisiti
</h2>

* **Node.js 18+** o **Python 3.10+**
* Un **account Anthropic** ([iscriviti qui](https://platform.claude.com/))

<h2 id="setup">
  Configurazione
</h2>

<Steps>
  <Step title="Crea una cartella di progetto">
    Crea una nuova directory per questa guida rapida:

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    Per i tuoi progetti, puoi eseguire l'SDK da qualsiasi cartella; avrà accesso ai file in quella directory e nelle sue sottodirectory per impostazione predefinita.
  </Step>

  <Step title="Installa l'SDK">
    Installa il pacchetto Agent SDK per il tuo linguaggio:

    <Tabs>
      <Tab title="TypeScript (nuovo progetto)">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        Impostare `"type": "module"` in `package.json` consente al tuo script agente di utilizzare `await` di livello superiore, e [tsx](https://tsx.is) esegue i file TypeScript direttamente.
      </Tab>

      <Tab title="TypeScript (progetto esistente)">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.is) esegue i file TypeScript direttamente. Se il tuo progetto utilizza CommonJS, nomina il tuo script agente `agent.mts` invece di `agent.ts`. L'estensione `.mts` fa sì che tsx tratti il file come un modulo ES, quindi `await` di livello superiore funziona senza convertire l'intero progetto a moduli ES. Utilizza `agent.mts` al posto di `agent.ts` nei passaggi di creazione ed esecuzione successivi in questa guida rapida.
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) è un gestore di pacchetti Python veloce che gestisce automaticamente gli ambienti virtuali:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Crea e attiva un ambiente virtuale, quindi installa il pacchetto.

        Su macOS o Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Su Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Se PowerShell blocca `Activate.ps1` con un errore di criterio di esecuzione, esegui prima `Set-ExecutionPolicy -Scope Process RemoteSigned`.
      </Tab>
    </Tabs>

    <Note>
      L'SDK TypeScript raggruppa un binario Claude Code nativo per la tua piattaforma come dipendenza opzionale, quindi non è necessario installare Claude Code separatamente.
    </Note>
  </Step>

  <Step title="Imposta la tua chiave API">
    Ottieni una chiave API dalla [Claude Console](https://platform.claude.com/), quindi impostala come variabile di ambiente nella shell in cui eseguirai il tuo agente:

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows (PowerShell)">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    L'SDK legge la chiave dall'ambiente del processo che esegue il tuo agente; non carica automaticamente i file `.env`. Se mantieni la chiave in un file `.env`, caricala tu stesso, ad esempio con il pacchetto `dotenv`, prima di chiamare l'SDK.

    L'SDK supporta anche l'autenticazione tramite provider API di terze parti:

    * **Amazon Bedrock**: imposta la variabile di ambiente `CLAUDE_CODE_USE_BEDROCK=1` e configura le credenziali AWS
    * **Claude Platform on AWS**: imposta `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` e `ANTHROPIC_AWS_WORKSPACE_ID`, quindi configura le credenziali AWS
    * **Google Cloud's Agent Platform**: imposta la variabile di ambiente `CLAUDE_CODE_USE_VERTEX=1` e configura le credenziali Google Cloud
    * **Microsoft Azure**: imposta la variabile di ambiente `CLAUDE_CODE_USE_FOUNDRY=1` e configura le credenziali Azure

    Consulta le guide di configurazione per [Amazon Bedrock](/docs/it/amazon-bedrock), [Claude Platform on AWS](/docs/it/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o [Microsoft Foundry](/docs/it/microsoft-foundry) per i dettagli.

    <Note>
      Se non precedentemente approvato, Anthropic non consente agli sviluppatori di terze parti di offrire il login claude.ai o limiti di velocità per i loro prodotti, inclusi gli agenti costruiti su Agent SDK di Claude. Utilizza invece i metodi di autenticazione con chiave API descritti in questo documento.
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  Crea un file buggy
</h2>

Questa guida rapida ti guida attraverso la creazione di un agente che può trovare e correggere i bug nel codice. Per prima cosa, hai bisogno di un file con alcuni bug intenzionali che l'agente possa correggere. Crea `utils.py` nella directory `my-agent` e incolla il seguente codice:

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

Questo codice ha due bug:

1. `calculate_average([])` si arresta in modo anomalo con una divisione per zero
2. `get_user_name(None)` si arresta in modo anomalo con un TypeError

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  Costruisci un agente che trova e corregge i bug
</h2>

Crea `agent.py` se stai utilizzando l'SDK Python, o `agent.ts` per TypeScript. Utilizza `agent.mts` invece se il tuo progetto esistente utilizza CommonJS:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

Questo codice ha tre parti principali:

1. **`query`**: il punto di ingresso principale che crea il loop agentico. Restituisce un iteratore asincrono, quindi usi `async for` per trasmettere i messaggi mentre Claude lavora. Vedi l'API completa nel riferimento SDK [Python](/docs/it/agent-sdk/python#query) o [TypeScript](/docs/it/agent-sdk/typescript#query).

2. **`prompt`**: quello che vuoi che Claude faccia. Claude capisce quali strumenti usare in base al compito.

3. **`options`**: configurazione per l'agente. Questo esempio utilizza `allowedTools` per pre-approvare `Read`, `Edit` e `Glob`, e `permissionMode: "acceptEdits"` per auto-approvare i cambiamenti ai file. Altre opzioni includono `systemPrompt`, `mcpServers` e altro. Vedi tutte le opzioni per [Python](/docs/it/agent-sdk/python#claudeagentoptions) o [TypeScript](/docs/it/agent-sdk/typescript#options).

Il loop `async for` continua a funzionare mentre Claude pensa, chiama strumenti, osserva i risultati e decide cosa fare dopo. Ogni iterazione produce un messaggio: il ragionamento di Claude, una chiamata a uno strumento, un risultato dello strumento, o il risultato finale. L'SDK gestisce l'orchestrazione (esecuzione dello strumento, gestione del contesto, tentativi) quindi consumi semplicemente il flusso. Il loop termina quando Claude completa il compito o incontra un errore.

La gestione dei messaggi all'interno del loop filtra l'output leggibile dall'uomo. Senza filtraggio, vedresti oggetti messaggio grezzi inclusa l'inizializzazione del sistema e lo stato interno, il che è utile per il debug ma rumoroso altrimenti.

<Note>
  Questo esempio utilizza lo streaming per mostrare i progressi in tempo reale. Se non hai bisogno di output dal vivo (ad esempio per lavori in background o pipeline CI), puoi raccogliere tutti i messaggi contemporaneamente. Vedi [Streaming vs. modalità single-turn](/docs/it/agent-sdk/streaming-vs-single-mode) per i dettagli.
</Note>

<h3 id="run-your-agent">
  Esegui il tuo agente
</h3>

Il tuo agente è pronto. Eseguilo con il seguente comando:

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    Se hai nominato il tuo script `agent.mts`, esegui `npx tsx agent.mts` invece.
  </Tab>

  <Tab title="Python (uv)">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python (pip)">
    Con il tuo ambiente virtuale ancora attivato:

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

Mentre lavora, l'agente stampa il suo ragionamento e ogni strumento che chiama, terminando con `Done: success`. Dopo l'esecuzione, controlla `utils.py`. Vedrai codice difensivo che gestisce elenchi vuoti e utenti nulli. Il tuo agente autonomamente:

1. **Ha letto** `utils.py` per comprendere il codice
2. **Ha analizzato** la logica e identificato i casi limite che causerebbero arresti anomali
3. **Ha modificato** il file per aggiungere la gestione corretta degli errori

Questo è ciò che rende diverso l'Agent SDK: Claude esegue gli strumenti direttamente invece di chiederti di implementarli.

<Note>
  Se vedi "API key not found", assicurati di aver impostato la variabile di ambiente `ANTHROPIC_API_KEY` nella shell in cui esegui il tuo agente. L'SDK non carica automaticamente i file `.env`. Vedi la [guida completa alla risoluzione dei problemi](/docs/it/troubleshooting) per ulteriore aiuto.
</Note>

<h3 id="try-other-prompts">
  Prova altri prompt
</h3>

Ora che il tuo agente è configurato, prova alcuni prompt diversi:

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  Personalizza il tuo agente
</h3>

Puoi modificare il comportamento del tuo agente cambiando le opzioni. Ecco alcuni esempi:

**Aggiungi capacità di ricerca web:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Dai a Claude un prompt di sistema personalizzato:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**Esegui comandi nel terminale:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

Con `Bash` abilitato, prova: `"Write unit tests for utils.py, run them, and fix any failures"`

<h2 id="key-concepts">
  Concetti chiave
</h2>

**Tools** controllano cosa può fare il tuo agente:

| Tools                                  | Cosa può fare l'agente            |
| -------------------------------------- | --------------------------------- |
| `Read`, `Glob`, `Grep`                 | Analisi di sola lettura           |
| `Read`, `Edit`, `Glob`                 | Analizzare e modificare il codice |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | Automazione completa              |

**Permission modes** controllano quanto controllo umano desideri:

| Mode                | Comportamento                                                                                                                                                                                                                                                                                                                             | Caso d'uso                                                            |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `acceptEdits`       | Auto-approva le modifiche ai file e i comandi comuni del file system, chiede per altre azioni                                                                                                                                                                                                                                             | Flussi di lavoro di sviluppo affidabili                               |
| `plan`              | Esegue strumenti di sola lettura; le modifiche ai file non vengono mai auto-approvate e raggiungono il tuo callback `canUseTool`                                                                                                                                                                                                          | Definizione dell'ambito di un compito prima di approvare l'esecuzione |
| `dontAsk`           | Nega tutto ciò che non è in `allowedTools`; gli strumenti del connettore [la tua organizzazione impostata su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti che richiedono l'interazione dell'utente vengono negati anche se li hai elencati                                                                    | Agenti headless bloccati                                              |
| `auto`              | Un classificatore di modelli approva o nega ogni chiamata di strumento                                                                                                                                                                                                                                                                    | Agenti autonomi con protezioni di sicurezza                           |
| `bypassPermissions` | Esegue ogni strumento senza prompt, a meno che una regola [`ask`](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) esplicita non corrisponda, gli strumenti del connettore [la tua organizzazione impostata su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e gli strumenti che richiedono l'interazione dell'utente | CI sandbox, ambienti completamente affidabili                         |
| `default`           | Richiede un callback `canUseTool` per gestire l'approvazione                                                                                                                                                                                                                                                                              | Flussi di approvazione personalizzati                                 |

L'esempio sopra utilizza la modalità `acceptEdits`, che auto-approva le operazioni sui file in modo che l'agente possa funzionare senza prompt interattivi. Se desideri richiedere agli utenti l'approvazione, utilizza la modalità `default` e fornisci un callback [`canUseTool`](/docs/it/agent-sdk/user-input) che raccoglie l'input dell'utente. Per un maggiore controllo, vedi [Permissions](/docs/it/agent-sdk/permissions).

<h2 id="next-steps">
  Passaggi successivi
</h2>

Ora che hai creato il tuo primo agente, scopri come estendere le sue capacità e adattarlo al tuo caso d'uso:

* **[Permissions](/docs/it/agent-sdk/permissions)**: controlla cosa può fare il tuo agente e quando ha bisogno di approvazione
* **[Hooks](/docs/it/agent-sdk/hooks)**: esegui codice personalizzato prima o dopo le chiamate agli strumenti
* **[Sessions](/docs/it/agent-sdk/sessions)**: costruisci agenti multi-turn che mantengono il contesto
* **[MCP servers](/docs/it/agent-sdk/mcp)**: connettiti a database, browser, API e altri sistemi esterni
* **[Hosting](/docs/it/agent-sdk/hosting)**: distribuisci agenti a Docker, cloud e CI/CD
* **[Example agents](https://github.com/anthropics/claude-agent-sdk-demos)**: vedi esempi completi: assistente email, agente di ricerca e altro
