> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare i permessi

> Controlla come il tuo agente utilizza gli strumenti con modalità di permesso, hook e regole dichiarative di consentimento/negazione.

Claude Agent SDK fornisce controlli di permesso per gestire come Claude utilizza gli strumenti. Utilizza modalità di permesso e regole per definire cosa è consentito automaticamente, e il callback [`canUseTool`](/docs/it/agent-sdk/user-input) per gestire tutto il resto in fase di esecuzione.

<Note>
  Questa pagina copre le modalità di permesso e le regole. Per creare flussi di approvazione interattivi in cui gli utenti approvano o negano le richieste di strumenti in fase di esecuzione, vedi [Gestire approvazioni e input dell'utente](/docs/it/agent-sdk/user-input).
</Note>

<h2 id="how-permissions-are-evaluated">
  Come vengono valutati i permessi
</h2>

Quando Claude richiede uno strumento, l'SDK controlla i permessi in questo ordine:

<Steps>
  <Step title="Hooks">
    Esegui [hooks](/docs/it/agent-sdk/hooks) per primo. Un hook può negare la chiamata completamente o trasmetterla. Un hook che restituisce `allow` non salta le regole di negazione e richiesta di seguito; quelle vengono valutate indipendentemente dal risultato dell'hook.
  </Step>

  <Step title="Regole di negazione">
    Controlla le regole `deny` (da `disallowed_tools` e [settings.json](/docs/it/settings#permission-settings)). Se una regola di negazione corrisponde, lo strumento viene bloccato, anche in modalità `bypassPermissions`. Le regole di negazione con nome semplice come `Bash` rimuovono lo strumento dal contesto di Claude prima che questa valutazione inizi, quindi solo le regole con ambito come `Bash(rm *)` vengono controllate in questo passaggio.
  </Step>

  <Step title="Regole di richiesta">
    Controlla le regole `ask` da [settings.json](/docs/it/settings#permission-settings). Se una regola di richiesta corrisponde, la chiamata passa al tuo callback [`canUseTool`](/docs/it/agent-sdk/user-input) per la conferma, anche in modalità `bypassPermissions`.

    Gli strumenti che richiedono l'interazione dell'utente si comportano allo stesso modo: `AskUserQuestion` e gli strumenti MCP il cui server imposta [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) passano sempre al callback, anche quando una regola di consentimento corrisponde. In modalità `dontAsk` entrambi i casi vengono negati invece, perché quella modalità non richiede mai conferma. L'annotazione MCP richiede Claude Code v2.1.199 o successivo.

    Gli strumenti del connettore [claude.ai](/docs/it/mcp#organization-controls-on-connector-tools) che la tua organizzazione ha impostato su `ask` lasciano anche il flusso in questo passaggio. Ogni chiamata passa al callback, anche in modalità `bypassPermissions` e anche quando una regola di consentimento corrisponde. Il callback riceve il motivo `La tua organizzazione richiede l'approvazione per questo strumento`. In modalità `dontAsk` la chiamata viene negata invece, perché quella modalità non richiede mai conferma.
  </Step>

  <Step title="Modalità di permesso">
    Applica la [modalità di permesso](#permission-modes) attiva. `bypassPermissions` approva tutto ciò che raggiunge questo passaggio. `acceptEdits` approva le operazioni su file. `plan` instrada gli strumenti di modifica file e scrittura shell al tuo callback `canUseTool` indipendentemente dalle regole di consentimento, quindi le operazioni di scrittura non possono essere approvate automaticamente durante la pianificazione. Le altre modalità passano oltre.
  </Step>

  <Step title="Regole di consentimento">
    Controlla le regole `allow` (da `allowed_tools` e settings.json). Se una regola corrisponde, lo strumento viene approvato.
  </Step>

  <Step title="Callback canUseTool">
    Se non risolto da nessuno dei precedenti, chiama il tuo callback [`canUseTool`](/docs/it/agent-sdk/user-input) per una decisione. In modalità `dontAsk`, questo passaggio viene saltato e lo strumento viene negato.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" alt="Diagramma del flusso di valutazione dei permessi in sei passaggi che corrisponde ai passaggi precedenti: una richiesta di strumento passa attraverso hook, regole di negazione, regole di richiesta, modalità di permesso, regole di consentimento e canUseTool. Hook, regole di negazione e canUseTool possono instradare verso Bloccato; bypass della modalità di permesso, regole di consentimento e canUseTool possono instradare verso Esegui; regole di richiesta instradano verso canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

A partire da v2.1.198, se passi un callback `canUseTool` che questo ordine di valutazione non può mai raggiungere, l'SDK TypeScript emette un avviso del processo Node.js una volta quando la query viene costruita. Il codice dell'avviso è `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Due configurazioni lo attivano:

* `permissionMode: 'bypassPermissions'`, che approva automaticamente ogni chiamata che raggiunge il passaggio della modalità di permesso
* Ogni voce `allowedTools` semplice come `"Read"`, che approva automaticamente quello strumento intero prima che il callback sia consultato

Le voci con uno specificatore come `Bash(ls *)` e la modalità `acceptEdits` non lo attivano, e le regole di consentimento provenienti da file di impostazioni non sono visibili al controllo.

Ascolta con `process.on('warning', ...)` e abbina il codice per registrarlo o sopprimerlo. Per controllare ogni chiamata di strumento indipendentemente dalla modalità e dalle regole, utilizza invece un hook [`PreToolUse`](/docs/it/agent-sdk/hooks).

Questa pagina si concentra su **regole di consentimento e negazione** e **modalità di permesso**. Per gli altri passaggi:

* **Hooks:** esegui codice personalizzato per consentire, negare o modificare le richieste di strumenti. Vedi [Controllare l'esecuzione con gli hook](/docs/it/agent-sdk/hooks).
* **Callback canUseTool:** richiedi agli utenti l'approvazione in fase di esecuzione, quando nessun passaggio precedente risolve la chiamata. Vedi [Gestire approvazioni e input dell'utente](/docs/it/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Regole di consentimento e negazione
</h2>

`allowed_tools` e `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) aggiungono voci agli elenchi di regole di consentimento e negazione nel flusso di valutazione sopra. Le regole di consentimento influiscono solo sull'approvazione: uno strumento non elencato in `allowed_tools` è ancora disponibile per Claude e passa alla modalità di permesso. Le regole di negazione si comportano diversamente a seconda che denominino uno strumento o limitino un modello all'interno di uno.

| Opzione                           | Effetto                                                                                                                                                                                                                                     |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowed_tools=["Read", "Grep"]`  | `Read` e `Grep` vengono approvati automaticamente. Gli strumenti non elencati qui esistono ancora e passano alla modalità di permesso e `canUseTool`.                                                                                       |
| `disallowed_tools=["Bash"]`       | La definizione dello strumento `Bash` viene rimossa dalla richiesta. Claude non vede lo strumento e non può tentarlo.                                                                                                                       |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` rimane disponibile. Le chiamate corrispondenti a `rm *` vengono negate in ogni modalità di permesso, inclusa `bypassPermissions`. Altre chiamate `Bash` passano alla modalità di permesso.                                           |
| `disallowed_tools=["*"]`          | Ogni definizione di strumento viene rimossa dalla richiesta. I glob dei nomi degli strumenti sono supportati nelle regole di negazione: `"*"` corrisponde a ogni strumento e `"mcp__*"` corrisponde a ogni strumento MCP su tutti i server. |

Le regole di consentimento accettano glob dei nomi degli strumenti solo dopo un prefisso letterale `mcp__<server>__`. Il segmento del server deve essere privo di glob in modo che la regola nomini un server specifico che hai configurato: `mcp__puppeteer__*` corrisponde a ogni strumento dal server `puppeteer` e `mcp__github__get_*` corrisponde ai suoi strumenti `get_`. Una voce non ancorata come `allowed_tools=["*"]` o `allowed_tools=["mcp__*"]` viene ignorata con un avviso di avvio e non approva automaticamente nulla.

Le regole limitate per `Read` e `Edit` accettano un modello di percorso. Le regole `Edit(path)` governano tutti gli strumenti integrati che scrivono file, inclusi `Write` e `NotebookEdit`; una regola `Write(path)` non viene mai abbinata dai controlli di permesso dei file.

Utilizza `//path` per un percorso del file system assoluto: una regola di negazione di `Edit(//secrets/**)` blocca le scritture ovunque sotto `/secrets` su disco. Con una singola barra iniziale, `Edit(/secrets/**)` si ancora alla fonte della regola. Per le regole passate tramite `allowed_tools` o `disallowed_tools`, ciò significa la directory di lavoro della sessione, quindi la regola non blocca `/secrets` su disco. Vedi [Regole Read e Edit](/docs/it/permissions#read-and-edit) per i quattro moduli di ancoraggio e come le regole dai file di impostazioni si risolvono.

<Warning>
  **Gli strumenti approvati automaticamente non raggiungono mai `canUseTool`.** Una chiamata a uno strumento approvata in qualsiasi fase precedente, da `acceptEdits` o `bypassPermissions`, o da una regola di consentimento, salta il tuo callback `canUseTool`, quindi i controlli di permesso che inserisci lì vengono silenziosamente ignorati per quello strumento. `AskUserQuestion`, gli strumenti MCP contrassegnati [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) e gli strumenti connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) raggiungono comunque il callback, anche quando una regola di consentimento corrisponde.

  La copertura dipende dalla forma della voce: un nome semplice come `Read` o `mcp__github__get_issue` approva automaticamente ogni chiamata a quello strumento, mentre una regola limitata come `Bash(ls *)` approva automaticamente solo le chiamate corrispondenti e altre chiamate `Bash` passano comunque al callback. Per i controlli che devono essere eseguiti su ogni chiamata a uno strumento, utilizza un hook [`PreToolUse`](/docs/it/agent-sdk/hooks): gli hook vengono eseguiti prima di ogni altro passaggio e un hook di negazione si applica anche in modalità `bypassPermissions`.
</Warning>

Per un agente bloccato, abbina `allowedTools` con `permissionMode: "dontAsk"`. Gli strumenti elencati vengono approvati, a parte gli strumenti sempre-prompt nella Avvertenza sopra; tutto il resto viene negato completamente invece di richiedere:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

<Warning>
  **`allowed_tools` non vincola `bypassPermissions`.** `allowed_tools` pre-approva solo gli strumenti che elenchi. Gli strumenti non elencati non vengono abbinati da alcuna regola di consentimento e passano alla modalità di permesso, dove `bypassPermissions` li approva. Impostare `allowed_tools=["Read"]` insieme a `permission_mode="bypassPermissions"` approva comunque ogni strumento, inclusi `Bash`, `Write` e `Edit`. Se hai bisogno di `bypassPermissions` ma vuoi bloccare strumenti specifici, usa `disallowed_tools`.
</Warning>

Puoi anche configurare regole di consentimento, negazione e richiesta in modo dichiarativo in `.claude/settings.json`. Queste regole vengono lette quando la fonte di impostazione `project` è abilitata, il che è il caso per le opzioni predefinite di `query()`. Se imposti `setting_sources` (TypeScript: `settingSources`) esplicitamente, includi `"project"` affinché si applichino. Vedi [Impostazioni di permesso](/docs/it/settings#permission-settings) per la sintassi delle regole.

<h2 id="permission-modes">
  Modalità di permesso
</h2>

Le modalità di permesso forniscono un controllo globale su come Claude utilizza gli strumenti. Puoi impostare la modalità di permesso quando chiami `query()` o cambiarla dinamicamente durante le sessioni di streaming.

<h3 id="available-modes">
  Modalità disponibili
</h3>

L'SDK supporta queste modalità di permesso:

| Modalità            | Descrizione                                  | Comportamento dello strumento                                                                                                                                                                                                                                                                                                                                                         |
| :------------------ | :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Comportamento di permesso standard           | Nessuna approvazione automatica; gli strumenti non abbinati attivano il tuo callback `canUseTool`                                                                                                                                                                                                                                                                                     |
| `dontAsk`           | Nega invece di richiedere                    | Qualsiasi cosa non pre-approvata da `allowed_tools` o regole viene negata; gli strumenti del connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti che richiedono l'interazione dell'utente vengono negati anche se li hai pre-approvati. `canUseTool` non viene mai chiamato                                |
| `acceptEdits`       | Accetta automaticamente le modifiche ai file | Le modifiche ai file e le [operazioni del filesystem](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, ecc.) vengono approvate automaticamente                                                                                                                                                                                                                                   |
| `bypassPermissions` | Ignora i controlli di permesso               | Gli strumenti vengono eseguiti senza richieste di permesso, ad eccezione degli strumenti abbinati da una regola [`ask`](#how-permissions-are-evaluated) esplicita, degli strumenti del connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e degli strumenti che richiedono l'interazione dell'utente (usare con cautela) |
| `plan`              | Modalità di pianificazione                   | Claude esplora e pianifica senza modificare i tuoi file sorgente; le modifiche ai file non vengono mai approvate automaticamente e richiedono il tuo callback `canUseTool`                                                                                                                                                                                                            |
| `auto`              | Approvazioni classificate dal modello        | Un classificatore di modello approva o nega ogni chiamata di strumento. Vedi [Auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) per la disponibilità                                                                                                                                                                                                                  |

<Warning>
  **Eredità del subagente:** Quando il genitore utilizza `bypassPermissions`, `acceptEdits` o `auto`, tutti i subagenti ereditano quella modalità e non può essere sovrascritta per subagente. I subagenti possono avere prompt di sistema diversi e comportamento meno vincolato rispetto al tuo agente principale, quindi ereditare `bypassPermissions` concede loro accesso completo e autonomo al sistema. Una regola [`ask`](#how-permissions-are-evaluated) esplicita, gli strumenti del connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e gli strumenti che richiedono l'interazione dell'utente forzano comunque una richiesta.
</Warning>

<h3 id="set-permission-mode">
  Impostare la modalità di permesso
</h3>

Puoi impostare la modalità di permesso una volta all'inizio di una query, o cambiarla dinamicamente mentre la sessione è attiva.

<Tabs>
  <Tab title="Al momento della query">
    Passa `permission_mode` (Python) o `permissionMode` (TypeScript) quando crei una query. Questa modalità si applica per l'intera sessione a meno che non venga modificata dinamicamente.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Help me refactor this code",
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Set the mode here
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        for await (const message of query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Set the mode here
          }
        })) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Durante lo streaming">
    Chiama `set_permission_mode()` (Python) o `setPermissionMode()` (TypeScript) per cambiare la modalità a metà sessione. La nuova modalità ha effetto immediatamente per tutte le richieste di strumenti successive. Questo ti consente di iniziare in modo restrittivo e allentare i permessi man mano che la fiducia aumenta, ad esempio passando a `acceptEdits` dopo aver esaminato l'approccio iniziale di Claude.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions


      async def main():
          async with ClaudeSDKClient(
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Start in default mode
              )
          ) as client:
              await client.query("Help me refactor this code")

              # Change mode dynamically mid-session
              await client.set_permission_mode("acceptEdits")

              # Process messages with the new permission mode
              async for message in client.receive_response():
                  if hasattr(message, "result"):
                      print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        const q = query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Start in default mode
          }
        });

        // Change mode dynamically mid-session
        await q.setPermissionMode("acceptEdits");

        // Process messages with the new permission mode
        for await (const message of q) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>
</Tabs>

<h3 id="mode-details">
  Dettagli della modalità
</h3>

<h4 id="accept-edits-mode-acceptedits">
  Modalità accetta modifiche (`acceptEdits`)
</h4>

Approva automaticamente le operazioni su file in modo che Claude possa modificare il codice senza richiedere. Altri strumenti (come i comandi Bash che non sono operazioni del filesystem) richiedono comunque i permessi normali.

**Operazioni approvate automaticamente:**

* Modifiche ai file (strumenti Edit, Write)
* Comandi del filesystem: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Entrambi si applicano solo ai percorsi all'interno della directory di lavoro o `additionalDirectories`. I percorsi al di fuori di tale ambito e le scritture su percorsi protetti richiedono comunque una richiesta.

**Usare quando:** ti fidi delle modifiche di Claude e desideri un'iterazione più veloce, ad esempio durante la prototipazione o quando lavori in una directory isolata.

<h4 id="don’t-ask-mode-dontask">
  Modalità non chiedere (`dontAsk`)
</h4>

Converte qualsiasi richiesta di permesso in una negazione. Gli strumenti pre-approvati da `allowed_tools`, regole di consentimento di `settings.json` o un hook vengono eseguiti normalmente. Gli strumenti del connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti che richiedono l'interazione dell'utente vengono negati anche quando una regola di consentimento corrisponde. Tutto il resto viene negato senza chiamare `canUseTool`.

**Usare quando:** desideri una superficie di strumenti fissa ed esplicita per un agente headless e preferisci una negazione definitiva rispetto a un affidamento silenzioso su `canUseTool` assente.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Modalità ignora permessi (`bypassPermissions`)
</h4>

Approva automaticamente tutti gli usi degli strumenti senza richieste. Gli hook vengono comunque eseguiti e possono bloccare le operazioni se necessario.

<Warning>
  Usare con estrema cautela. Claude ha accesso completo al sistema in questa modalità. Usare solo in ambienti controllati in cui ti fidi di tutte le operazioni possibili.

  `allowed_tools` non vincola questa modalità. Ogni strumento viene approvato, non solo quelli che hai elencato. Le regole di negazione (`disallowed_tools`), le regole esplicite `ask` e gli hook vengono valutati prima del controllo della modalità e possono comunque bloccare uno strumento. Gli strumenti del connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti che richiedono l'interazione dell'utente continuano a passare al tuo callback `canUseTool`.
</Warning>

<h4 id="plan-mode-plan">
  Modalità piano (`plan`)
</h4>

Claude esplora la base di codice e produce un piano senza modificare i tuoi file sorgente. Gli strumenti di sola lettura vengono eseguiti come in modalità predefinita. Le modifiche ai file non vengono mai approvate automaticamente in modalità piano, anche quando una regola di consentimento corrisponde. Richiedono il tuo callback `canUseTool` invece. Claude può utilizzare `AskUserQuestion` per chiarire i requisiti prima di finalizzare il piano. Vedi [Gestire approvazioni e input dell'utente](/docs/it/agent-sdk/user-input#handle-clarifying-questions) per gestire queste richieste.

**Usare quando:** desideri che Claude proponga modifiche senza eseguirle, ad esempio durante la revisione del codice o quando hai bisogno di approvare le modifiche prima che vengano apportate.

<h2 id="related-resources">
  Risorse correlate
</h2>

Per gli altri passaggi nel flusso di valutazione dei permessi:

* [Gestire approvazioni e input dell'utente](/docs/it/agent-sdk/user-input): richieste di approvazione interattive e domande di chiarimento
* [Guida agli hook](/docs/it/agent-sdk/hooks): esegui codice personalizzato nei punti chiave del ciclo di vita dell'agente
* [Regole di permesso](/docs/it/settings#permission-settings): regole dichiarative di consentimento/negazione in `settings.json`
