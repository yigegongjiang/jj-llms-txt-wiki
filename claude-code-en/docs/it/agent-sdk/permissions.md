> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare i permessi

> Controlla come il tuo agente utilizza gli strumenti con modalità di permesso, hook e regole dichiarative di consentimento/negazione.

Claude Agent SDK fornisce controlli di permesso per gestire come Claude utilizza gli strumenti. Utilizza modalità di permesso e regole per definire ciò che è consentito automaticamente, e il callback [`canUseTool`](/docs/it/agent-sdk/user-input) per gestire tutto il resto in fase di esecuzione.

<h2 id="how-permissions-are-evaluated">
  Come vengono valutate le autorizzazioni
</h2>

Quando Claude richiede uno strumento, l'SDK controlla le autorizzazioni in questo ordine:

<Steps>
  <Step title="Hooks">
    Eseguire prima gli [hooks](/docs/it/agent-sdk/hooks). Un hook può negare la chiamata completamente o lasciarla passare. Un hook che restituisce `allow` non salta le regole di negazione e richiesta di seguito; quelle vengono valutate indipendentemente dal risultato dell'hook. Un hook `PreToolUse` allow inoltre non può approvare una rimozione `rm` o `rmdir` che prende di mira un [percorso critico](/docs/it/permission-modes#critical-paths).
  </Step>

  <Step title="Regole di negazione">
    Controllare le regole `deny` (da `disallowed_tools` e [settings.json](/docs/it/settings-reference#permission-settings)). Se una regola di negazione corrisponde, lo strumento viene bloccato, anche in modalità `bypassPermissions`. Le regole di negazione con nome semplice come `Bash` rimuovono lo strumento dal contesto di Claude prima che questa valutazione inizi, quindi solo le regole con ambito come `Bash(rm *)` vengono controllate in questo passaggio.
  </Step>

  <Step title="Regole di richiesta">
    Controllare le regole `ask` da [settings.json](/docs/it/settings-reference#permission-settings). Se una regola di richiesta corrisponde, la chiamata passa al vostro callback [`canUseTool`](/docs/it/agent-sdk/user-input) per la conferma, anche in modalità `bypassPermissions`.

    Gli strumenti che richiedono l'interazione dell'utente si comportano allo stesso modo: `AskUserQuestion` e gli strumenti MCP il cui server imposta [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) passano sempre al callback, anche quando una regola di autorizzazione corrisponde. In modalità `dontAsk` entrambi i casi vengono negati, perché quella modalità non richiede mai. L'annotazione MCP richiede Claude Code v2.1.199 o successivo.

    Gli strumenti del connettore [claude.ai](/docs/it/mcp#organization-controls-on-connector-tools) che la vostra organizzazione ha impostato su `ask` lasciano anche il flusso in questo passaggio. Ogni chiamata passa al callback, anche in modalità `bypassPermissions` e anche quando una regola di autorizzazione corrisponde. Il callback riceve il motivo `Your organization requires approval for this tool`. In modalità `dontAsk` la chiamata viene negata, perché quella modalità non richiede mai.
  </Step>

  <Step title="Modalità di autorizzazione">
    Applicare la [modalità di autorizzazione](#permission-modes) attiva:

    * In modalità `bypassPermissions`, Claude Code approva tutto ciò che raggiunge questo passaggio tranne le rimozioni `rm` e `rmdir` che prendono di mira un [percorso critico](/docs/it/permission-modes#critical-paths), che passano invece.
    * In modalità `acceptEdits`, Claude Code approva le operazioni su file elencate in [Modalità accetta modifiche](#accept-edits-mode-acceptedits).
    * In modalità `plan`, Claude Code invia gli strumenti di modifica file e scrittura shell al vostro callback `canUseTool` indipendentemente dalle regole di autorizzazione, in modo che le operazioni di scrittura non possano essere approvate automaticamente durante la pianificazione.
    * In altre modalità, la richiesta passa.
  </Step>

  <Step title="Regole di autorizzazione">
    Controllare le regole `allow` (da `allowed_tools` e settings.json). Se una regola corrisponde, lo strumento viene approvato. Una chiamata che lo strumento approva da solo viene risolta in questo passaggio, senza alcuna regola necessaria: ad esempio una lettura di file all'interno delle vostre directory di lavoro o un [comando Bash di sola lettura](/docs/it/permissions#read-only-commands). Le rimozioni `rm` e `rmdir` che prendono di mira un [percorso critico](/docs/it/permission-modes#critical-paths) non vengono mai approvate da una regola di autorizzazione: raggiungono il vostro callback nelle modalità che richiedono, vanno al [classificatore](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) in modalità `auto` su Claude Code v2.1.218 o successivo, e vengono negate in modalità `dontAsk`.
  </Step>

  <Step title="Callback canUseTool">
    Se non risolto da nessuno dei precedenti, chiamare il vostro callback [`canUseTool`](/docs/it/agent-sdk/user-input) per una decisione. In modalità `dontAsk`, questo passaggio viene saltato e lo strumento viene negato.

    Nell'SDK TypeScript, se impostate [`permissionPrompts: 'none'`](/docs/it/agent-sdk/typescript#options), il vostro callback non viene chiamato in questo passaggio. Un hook [`PermissionRequest`](/docs/it/hooks#permissionrequest) ha ancora la possibilità di decidere, e se non lo fa, Claude Code nega la chiamata. L'opzione richiede Claude Code v2.1.259 o successivo.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="Diagramma del flusso di valutazione delle autorizzazioni in sei passaggi che corrisponde ai passaggi precedenti: una richiesta di strumento passa attraverso hook, regole di negazione, regole di richiesta, modalità di autorizzazione, regole di autorizzazione e canUseTool. Hook, regole di negazione e canUseTool possono instradare verso il basso a Bloccato; bypass della modalità di autorizzazione, regole di autorizzazione e canUseTool possono instradare verso l'alto a Esegui; le regole di richiesta instradano a canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="Diagramma del flusso di valutazione delle autorizzazioni in sei passaggi che corrisponde ai passaggi precedenti: una richiesta di strumento passa attraverso hook, regole di negazione, regole di richiesta, modalità di autorizzazione, regole di autorizzazione e canUseTool. Hook, regole di negazione e canUseTool possono instradare verso il basso a Bloccato; bypass della modalità di autorizzazione, regole di autorizzazione e canUseTool possono instradare verso l'alto a Esegui; le regole di richiesta instradano a canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

Se passate un callback `canUseTool` in una configurazione in cui l'SDK TypeScript si aspetta che l'ordine di valutazione approvi automaticamente le chiamate prima che il callback sia consultato, l'SDK emette un avviso di processo Node.js una volta quando la query viene costruita. Il codice dell'avviso è `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Due configurazioni lo attivano:

* `permissionMode: 'bypassPermissions'`, che approva automaticamente ogni chiamata che raggiunge il passaggio della modalità di autorizzazione a parte le [azioni che nessuna modalità approva automaticamente](/docs/it/permission-modes#actions-no-mode-auto-approves)
* Ogni voce `allowedTools` semplice come `"Read"`, che approva automaticamente quello strumento intero prima che il callback sia consultato, a parte le [azioni che nessuna modalità approva automaticamente](/docs/it/permission-modes#actions-no-mode-auto-approves)

Le voci con uno specificatore come `Bash(ls *)` e la modalità `acceptEdits` non lo attivano, e le regole di autorizzazione provenienti da file di impostazioni non sono visibili al controllo.

Ascoltate con `process.on('warning', ...)` e abbinate il codice per registrarlo o sopprimerlo. Per controllare ogni chiamata di strumento indipendentemente dalla modalità e dalle regole, utilizzate invece un [hook `PreToolUse`](/docs/it/agent-sdk/hooks).

Questa pagina si concentra su **regole di autorizzazione e negazione** e **modalità di autorizzazione**. Per gli altri passaggi:

* **Hooks:** eseguire codice personalizzato per consentire, negare o modificare le richieste di strumenti. Vedere [Controllare l'esecuzione con gli hook](/docs/it/agent-sdk/hooks).
* **Callback canUseTool:** richiedere l'approvazione degli utenti in fase di esecuzione, quando nessun passaggio precedente risolve la chiamata. Vedere [Gestire le approvazioni e l'input dell'utente](/docs/it/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Regole di consentimento e negazione
</h2>

`allowed_tools` e `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) aggiungono voci agli elenchi di regole di consentimento e negazione nel flusso di valutazione sopra descritto. Se nominate uno dei [strumenti di tracciamento delle attività](/docs/it/agent-sdk/todo-tracking#model-availability) in `allowed_tools`, Claude Code opta anche per la sessione. Qualsiasi altro strumento non elencato in `allowed_tools` è ancora disponibile per Claude, e una chiamata ad esso che necessita di approvazione passa attraverso la modalità di autorizzazione. Le regole di negazione si comportano diversamente a seconda che nominino uno strumento o limitino un modello all'interno di uno.

| Opzione                           | Effetto                                                                                                                                                                                                                                                                                           |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowed_tools=["Read", "Grep"]`  | `Read` e `Grep` sono approvati automaticamente. Gli altri strumenti non elencati qui esistono ancora, e le chiamate ad essi che necessitano di approvazione passano attraverso la modalità di autorizzazione e `canUseTool`.                                                                      |
| `disallowed_tools=["Bash"]`       | La definizione dello strumento `Bash` viene rimossa dalla richiesta. Claude non vede lo strumento e non può tentarlo.                                                                                                                                                                             |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` rimane disponibile. Le chiamate che corrispondono a `rm *` [come scritto](/docs/it/permissions#bash-rule-limits) vengono negate in ogni modalità di autorizzazione, inclusa `bypassPermissions`. Le altre chiamate `Bash`, incluso `/bin/rm`, passano attraverso la modalità di autorizzazione. |
| `disallowed_tools=["*"]`          | Ogni definizione di strumento viene rimossa dalla richiesta. I glob dei nomi degli strumenti sono supportati nelle regole di negazione: `"*"` corrisponde a ogni strumento e `"mcp__*"` corrisponde a ogni strumento MCP su tutti i server.                                                       |

Le regole di consentimento accettano glob dei nomi degli strumenti solo dopo un prefisso letterale `mcp__<server>__`. Il segmento del server deve essere privo di glob in modo che la regola nomini un server specifico che avete configurato: `mcp__puppeteer__*` corrisponde a ogni strumento dal server `puppeteer`, e `mcp__github__get_*` corrisponde ai suoi strumenti `get_`. Una voce non ancorata come `allowed_tools=["*"]` o `allowed_tools=["mcp__*"]` viene ignorata con un avviso di avvio e non approva automaticamente nulla.

Le regole limitate per `Read` e `Edit` accettano un modello di percorso. Le regole `Edit(path)` governano tutti gli strumenti integrati che scrivono file, inclusi `Write` e `NotebookEdit`; una regola `Write(path)` non viene mai abbinata dai controlli di autorizzazione dei file.

Utilizzate `//path` per un percorso del filesystem assoluto: una regola di negazione di `Edit(//secrets/**)` blocca le scritture ovunque sotto `/secrets` su disco. Con una singola barra iniziale, `Edit(/secrets/**)` si ancora alla fonte della regola. Per le regole passate attraverso `allowed_tools` o `disallowed_tools`, ciò significa la directory di lavoro della sessione, quindi la regola non blocca `/secrets` su disco. Consultate [Regole Read e Edit](/docs/it/permissions#read-and-edit) per i quattro moduli di ancoraggio e come le regole dai file di impostazioni si risolvono.

<Warning>
  **Gli strumenti approvati automaticamente non raggiungono mai `canUseTool`.** Una chiamata a uno strumento approvata in qualsiasi fase precedente, da `acceptEdits` o `bypassPermissions`, o da una regola di consentimento, salta il callback `canUseTool`, quindi i controlli di autorizzazione che inserite lì vengono silenziosamente ignorati per quello strumento. `AskUserQuestion`, gli strumenti MCP contrassegnati [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool), gli strumenti del connettore [che la vostra organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e le rimozioni `rm` e `rmdir` che puntano a un [percorso critico](/docs/it/permission-modes#critical-paths) raggiungono ancora il callback, anche quando una regola di consentimento corrisponde. In modalità `auto`, le rimozioni di percorsi critici vanno al [classificatore](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) invece del callback, mentre le altre chiamate elencate qui lo raggiungono ancora; il routing del classificatore richiede Claude Code v2.1.218 o successivo. In modalità `dontAsk` queste chiamate vengono invece negate, senza invocare il callback.

  La copertura dipende dalla forma della voce: un nome semplice come `Read` o `mcp__github__get_issue` approva automaticamente ogni chiamata a quello strumento a parte le eccezioni sopra, mentre una regola limitata come `Bash(npm test *)` approva automaticamente solo le chiamate corrispondenti, e le altre chiamate `Bash` che necessitano di approvazione passano ancora attraverso il callback. Per i controlli che devono essere eseguiti su ogni chiamata a uno strumento, utilizzate un [hook `PreToolUse`](/docs/it/agent-sdk/hooks): gli hook vengono eseguiti prima di ogni altro passaggio, e un hook di negazione si applica anche in modalità `bypassPermissions`.
</Warning>

Per un agente bloccato, abbinate `allowedTools` con `permissionMode: "dontAsk"`:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

Gli strumenti elencati sono approvati, a parte le [azioni che nessuna modalità approva automaticamente](/docs/it/permission-modes#actions-no-mode-auto-approves), e ogni altra chiamata che comporterebbe una richiesta viene invece negata. Le chiamate che non necessitano di approvazione in modalità `default` vengono eseguite indipendentemente dal fatto che le elenchiate, come i [comandi Bash di sola lettura](/docs/it/permissions#read-only-commands), strumenti come `Agent` che non chiedono prima di eseguire, e letture di file all'interno delle vostre directory di lavoro. Per mettere uno strumento completamente fuori dalla portata di Claude, aggiungete il suo nome semplice a `disallowedTools`.

<Warning>
  **`allowed_tools` non vincola `bypassPermissions`.** `allowed_tools` pre-approva gli strumenti che elencate. Gli altri strumenti non elencati non vengono abbinati da alcuna regola di consentimento e passano attraverso la modalità di autorizzazione, dove `bypassPermissions` li approva. L'impostazione di `allowed_tools=["Read"]` insieme a `permission_mode="bypassPermissions"` approva comunque ogni strumento, inclusi `Bash`, `Write` e `Edit`. Se avete bisogno di `bypassPermissions` ma volete che strumenti specifici siano bloccati, utilizzate `disallowed_tools`.
</Warning>

Potete anche configurare le regole di consentimento, negazione e richiesta in modo dichiarativo in `.claude/settings.json`. Queste regole vengono lette quando la fonte di impostazione `project` è abilitata, il che avviene per le opzioni predefinite di `query()`. Se impostate esplicitamente `setting_sources` (TypeScript: `settingSources`), includete `"project"` affinché si applichino. Consultate [Impostazioni di autorizzazione](/docs/it/settings-reference#permission-settings) per la sintassi delle regole.

<h2 id="permission-modes">
  Modalità di autorizzazione
</h2>

Le modalità di autorizzazione forniscono un controllo globale su come Claude utilizza gli strumenti. È possibile impostare la modalità di autorizzazione quando si chiama `query()` o modificarla dinamicamente durante le sessioni di streaming.

<h3 id="available-modes">
  Modalità disponibili
</h3>

L'SDK supporta queste modalità di autorizzazione:

| Modalità            | Descrizione                                  | Comportamento dello strumento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------ | :------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Comportamento di autorizzazione standard     | Nessuna approvazione automatica basata sulla modalità; le chiamate che richiedono approvazione e non corrispondono a nessuna regola di autorizzazione attivano il callback `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `dontAsk`           | Nega invece di chiedere                      | Qualsiasi chiamata che altrimenti richiederebbe una richiesta viene negata. Le chiamate approvate da `allowed_tools` o regole vengono eseguite, così come le chiamate che non richiedono approvazione in modalità `default`, come le letture di file all'interno delle directory di lavoro; gli strumenti connector [impostati dalla vostra organizzazione su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti che richiedono interazione dell'utente vengono negati anche se li avete pre-approvati, così come le rimozioni `rm` e `rmdir` che interessano un [percorso critico](/docs/it/permission-modes#critical-paths). `canUseTool` non viene mai chiamato |
| `acceptEdits`       | Accetta automaticamente le modifiche ai file | Le modifiche ai file e le [operazioni del filesystem](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, ecc.) vengono approvate automaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `bypassPermissions` | Ignora i controlli di autorizzazione         | Gli strumenti vengono eseguiti senza richieste di autorizzazione, ad eccezione delle [azioni che nessuna modalità approva automaticamente](/docs/it/permission-modes#actions-no-mode-auto-approves). Utilizzare con cautela                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `plan`              | Modalità di pianificazione                   | Claude esplora e pianifica senza modificare i file sorgente; le modifiche ai file non vengono mai approvate automaticamente e richiedono il callback `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `auto`              | Approvazioni classificate dal modello        | Un classificatore del modello approva o nega le richieste di autorizzazione. Vedere [Modalità Auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) per la disponibilità                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<Warning>
  **Ereditarietà dei subagent:** Un subagent viene eseguito nella modalità di autorizzazione della sessione padre a meno che non si imposti `permissionMode` sulla sua [`AgentDefinition`](/docs/it/agent-sdk/typescript#agentdefinition) e la sessione padre sia in modalità `default`, `dontAsk` o `plan`. Anche in questo caso, Claude Code non applica mai un valore `"bypassPermissions"`. Un subagent viene eseguito in modalità `bypassPermissions` solo quando la sessione padre stessa lo fa. L'eccezione `bypassPermissions` richiede Claude Code v2.1.267 o successivo.

  I subagent possono avere prompt di sistema diversi e comportamenti meno vincolati rispetto all'agente principale, quindi ereditare `bypassPermissions` concede loro accesso completo e autonomo al sistema. Le [azioni che nessuna modalità approva automaticamente](/docs/it/permission-modes#actions-no-mode-auto-approves) si applicano comunque.
</Warning>

<h3 id="set-permission-mode">
  Impostare la modalità di autorizzazione
</h3>

È possibile impostare la modalità di autorizzazione una volta all'avvio di una query, oppure modificarla dinamicamente mentre la sessione è attiva.

<Tabs>
  <Tab title="Al momento della query">
    Passare `permission_mode` (Python) o `permissionMode` (TypeScript) quando si crea una query. Questa modalità si applica per l'intera sessione a meno che non venga modificata dinamicamente.

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
    Chiamare `set_permission_mode()` (Python) o `setPermissionMode()` (TypeScript) per modificare la modalità durante la sessione. La nuova modalità ha effetto immediatamente per tutte le successive richieste di strumenti. Questo consente di iniziare in modo restrittivo e allentare le autorizzazioni man mano che la fiducia aumenta, ad esempio passando a `acceptEdits` dopo aver esaminato l'approccio iniziale di Claude.

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

Approva automaticamente le operazioni sui file in modo che Claude possa modificare il codice senza richiedere conferma. Gli altri strumenti (come i comandi Bash che non sono operazioni del filesystem) richiedono comunque autorizzazioni normali.

**Operazioni approvate automaticamente:**

* Modifiche ai file (strumenti Edit, Write)
* Comandi del filesystem: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Entrambi si applicano solo ai percorsi all'interno della directory di lavoro o `additionalDirectories`. In modalità `acceptEdits`, Claude Code non approva automaticamente la richiesta quando Claude:

* Lavora su un percorso al di fuori di tale ambito
* Scrive in un percorso protetto
* Rimuove un [percorso critico](/docs/it/permission-modes#critical-paths) con `rm` o `rmdir`

**Utilizzare quando:** si confida nelle modifiche di Claude e si desidera un'iterazione più veloce, ad esempio durante la prototipazione o quando si lavora in una directory isolata.

<h4 id="don’t-ask-mode-dontask">
  Modalità non chiedere (`dontAsk`)
</h4>

Converte qualsiasi richiesta di autorizzazione in una negazione, senza chiamare `canUseTool`. Gli strumenti pre-approvati da `allowed_tools`, regole di autorizzazione in `settings.json` o un hook vengono eseguiti normalmente, così come le chiamate che non richiedono approvazione in modalità `default`, come le letture di file all'interno delle directory di lavoro e le chiamate a `Agent`. Gli strumenti connector [impostati dalla vostra organizzazione su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), gli strumenti che richiedono interazione dell'utente e le rimozioni `rm` e `rmdir` che interessano un [percorso critico](/docs/it/permission-modes#critical-paths) vengono negati anche quando una regola di autorizzazione corrisponde. Un'autorizzazione del hook `PreToolUse` non cancella nemmeno una rimozione di percorso critico.

**Utilizzare quando:** si desidera una superficie di strumenti fissa ed esplicita per un agente headless e si preferisce un rifiuto netto rispetto all'affidamento silenzioso all'assenza di `canUseTool`.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Modalità ignora autorizzazioni (`bypassPermissions`)
</h4>

Approva automaticamente gli usi degli strumenti senza richiedere conferma, ad eccezione dei casi elencati nell'avviso di seguito. I hook vengono comunque eseguiti e possono bloccare le operazioni se necessario. Su Linux e macOS, Claude Code rifiuta di avviarsi in questa modalità come root o sotto `sudo` al di fuori di una [sandbox riconosciuta](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode), e la query non riesce prima del primo turno.

<Warning>
  Utilizzare con estrema cautela. Claude ha accesso completo al sistema in questa modalità. Utilizzare solo in ambienti controllati in cui si fidano di tutte le possibili operazioni.

  `allowed_tools` non vincola questa modalità. Ogni strumento è approvato, non solo quelli che avete elencato. Questi controlli si applicano comunque:

  * Le regole di negazione, le regole esplicite `ask` e i hook vengono valutati prima del controllo della modalità e possono comunque bloccare uno strumento.
  * Gli strumenti connector [impostati dalla vostra organizzazione su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), gli strumenti che richiedono interazione dell'utente e le rimozioni `rm` e `rmdir` che interessano un [percorso critico](/docs/it/permission-modes#critical-paths) continuano a passare al callback `canUseTool`.
  * Le [protezioni della messaggistica tra sessioni](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode) si applicano comunque.
</Warning>

<h4 id="plan-mode-plan">
  Modalità pianificazione (`plan`)
</h4>

Claude esplora la base di codice e produce un piano senza modificare i file sorgente. Gli strumenti di sola lettura vengono eseguiti come nella modalità di autorizzazione `default`.

Le modifiche ai file non vengono mai approvate automaticamente in modalità plan, anche quando una regola di autorizzazione corrisponde. Invece, richiedono il callback `canUseTool`. Su Claude Code v2.1.212 o successivo, i comandi shell che modificano i file, come `touch` e `rm`, raggiungono il callback `canUseTool` allo stesso modo.

Se si imposta `allowDangerouslySkipPermissions: true` insieme a `permissionMode: 'plan'`, le modifiche ai file e i comandi shell che modificano i file raggiungono comunque il callback `canUseTool`. L'opzione consente di passare a `bypassPermissions` in seguito con `setPermissionMode()`.

Claude può utilizzare `AskUserQuestion` per chiarire i requisiti prima di finalizzare il piano. Vedere [Gestire approvazioni e input dell'utente](/docs/it/agent-sdk/user-input#handle-clarifying-questions) per la gestione di queste richieste.

**Utilizzare quando:** si desidera che Claude proponga modifiche senza eseguirle, ad esempio durante la revisione del codice o quando è necessario approvare le modifiche prima che vengano apportate.

<h2 id="related-resources">
  Risorse correlate
</h2>

Per gli altri passaggi nel flusso di valutazione delle autorizzazioni:

* [Gestire approvazioni e input dell'utente](/docs/it/agent-sdk/user-input): prompt di approvazione interattivi e domande di chiarimento
* [Guida hooks](/docs/it/agent-sdk/hooks): eseguire codice personalizzato nei punti chiave del ciclo di vita dell'agente
* [Regole di autorizzazione](/docs/it/settings-reference#permission-settings): regole dichiarative di consentimento/negazione in `settings.json`
