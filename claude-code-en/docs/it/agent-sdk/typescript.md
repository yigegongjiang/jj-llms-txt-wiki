> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento Agent SDK - TypeScript

> Riferimento API completo per l'Agent SDK TypeScript, incluse tutte le funzioni, i tipi e le interfacce.

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  Installazione
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  L'SDK raggruppa un binario nativo Claude Code per la tua piattaforma come dipendenza opzionale come `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Non è necessario installare Claude Code separatamente. Se il tuo gestore di pacchetti salta le dipendenze opzionali, l'SDK genera `Native CLI binary for <platform> not found`; imposta [`pathToClaudeCodeExecutable`](#options) su un binario `claude` installato separatamente.
</Note>

<h3 id="compile-to-a-single-executable">
  Compilare in un singolo eseguibile
</h3>

Quando compili la tua applicazione in un eseguibile a file singolo con `bun build --compile`, l'SDK non può risolvere il binario CLI raggruppato in fase di esecuzione. `require.resolve` non funziona all'interno del filesystem virtuale `$bunfs` dell'eseguibile compilato, quindi l'SDK genera `Native CLI binary for <platform> not found`.

Per aggirare questo problema, incorpora il binario della piattaforma come risorsa file, estrailo in un percorso reale all'avvio con `extractFromBunfs()` e passa quel percorso a [`pathToClaudeCodeExecutable`](#options).

L'helper `extractFromBunfs()` richiede `@anthropic-ai/claude-agent-sdk` v0.3.144 o successivo. L'esempio seguente compila per macOS su Apple Silicon:

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()` copia il binario incorporato dal filesystem virtuale dell'eseguibile compilato in una directory temporanea per utente e restituisce il percorso reale. Al di fuori di un eseguibile compilato restituisce il percorso di input invariato, quindi lo stesso codice viene eseguito in sviluppo senza modifiche.

Ogni eseguibile compilato incorpora il binario di una singola piattaforma. Fai corrispondere il pacchetto della piattaforma nell'importazione al tuo `--target`:

* Per la compilazione incrociata, installa il pacchetto della piattaforma non corrispondente, ad esempio `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`.
* Su Windows, il sottopercorso binario è `claude.exe`, ad esempio `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`.

<h2 id="functions">
  Funzioni
</h2>

<h3 id="query">
  `query()`
</h3>

La funzione principale per interagire con Claude Code. Crea un generatore asincrono che trasmette i messaggi man mano che arrivano.

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

<h4 id="parameters">
  Parametri
</h4>

| Parametro | Tipo                                                             | Descrizione                                                                     |
| :-------- | :--------------------------------------------------------------- | :------------------------------------------------------------------------------ |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | Il prompt di input come stringa o iterabile asincrono per la modalità streaming |
| `options` | [`Options`](#options)                                            | Oggetto di configurazione opzionale (vedi il tipo Options di seguito)           |

<h4 id="returns">
  Restituisce
</h4>

Restituisce un oggetto [`Query`](#query-object) che estende `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` con metodi aggiuntivi.

<h3 id="startup">
  `startup()`
</h3>

Pre-riscalda il subprocess CLI generandolo e completando l'handshake di inizializzazione prima che un prompt sia disponibile. L'handle [`WarmQuery`](#warmquery) restituito accetta un prompt in seguito e lo scrive in un processo già pronto, quindi la prima chiamata `query()` si risolve senza pagare il costo di generazione e inizializzazione del subprocess inline.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  Parametri
</h4>

| Parametro             | Tipo                  | Descrizione                                                                                                                                                                                                |
| :-------------------- | :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | Oggetto di configurazione opzionale. Uguale al parametro `options` di `query()`                                                                                                                            |
| `initializeTimeoutMs` | `number`              | Tempo massimo in millisecondi per attendere l'inizializzazione del subprocess. Predefinito a `60000`. Se l'inizializzazione non si completa in tempo, la promessa viene rifiutata con un errore di timeout |

<h4 id="returns-2">
  Restituisce
</h4>

Restituisce una `Promise<`[`WarmQuery`](#warmquery)`>` che si risolve una volta che il subprocess è stato generato e ha completato il suo handshake di inizializzazione.

<h4 id="example">
  Esempio
</h4>

Chiama `startup()` presto, ad esempio all'avvio dell'applicazione, quindi chiama `.query()` sull'handle restituito una volta che un prompt è pronto. Questo sposta la generazione del subprocess e l'inizializzazione fuori dal percorso critico.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Paga il costo di avvio in anticipo
const warm = await startup({ options: { maxTurns: 3 } });

// Più tardi, quando un prompt è pronto, questo è immediato
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

Crea una definizione di tool MCP type-safe per l'uso con i server MCP dell'SDK.

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Parametri
</h4>

| Parametro     | Tipo                                                              | Descrizione                                                                           |
| :------------ | :---------------------------------------------------------------- | :------------------------------------------------------------------------------------ |
| `name`        | `string`                                                          | Il nome del tool                                                                      |
| `description` | `string`                                                          | Una descrizione di cosa fa il tool                                                    |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | Schema Zod che definisce i parametri di input del tool (supporta sia Zod 3 che Zod 4) |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Funzione asincrona che esegue la logica del tool                                      |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | Annotazioni MCP tool opzionali che forniscono suggerimenti comportamentali ai client  |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Re-esportato da `@modelcontextprotocol/sdk/types.js`. Tutti i campi sono suggerimenti opzionali; i client non dovrebbero fare affidamento su di essi per decisioni di sicurezza.

| Campo             | Tipo      | Predefinito | Descrizione                                                                                                                                            |
| :---------------- | :-------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | Titolo leggibile per il tool                                                                                                                           |
| `readOnlyHint`    | `boolean` | `false`     | Se `true`, il tool non modifica il suo ambiente                                                                                                        |
| `destructiveHint` | `boolean` | `true`      | Se `true`, il tool può eseguire aggiornamenti distruttivi (significativo solo quando `readOnlyHint` è `false`)                                         |
| `idempotentHint`  | `boolean` | `false`     | Se `true`, le chiamate ripetute con gli stessi argomenti non hanno effetto aggiuntivo (significativo solo quando `readOnlyHint` è `false`)             |
| `openWorldHint`   | `boolean` | `true`      | Se `true`, il tool interagisce con entità esterne (ad esempio, ricerca web). Se `false`, il dominio del tool è chiuso (ad esempio, un tool di memoria) |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

<h3 id="createsdkmcpserver">
  `createSdkMcpServer()`
</h3>

Crea un'istanza di server MCP che viene eseguita nello stesso processo della tua applicazione.

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Parametri
</h4>

| Parametro         | Tipo                          | Descrizione                                               |
| :---------------- | :---------------------------- | :-------------------------------------------------------- |
| `options.name`    | `string`                      | Il nome del server MCP                                    |
| `options.version` | `string`                      | Stringa di versione opzionale                             |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | Array di definizioni di tool create con [`tool()`](#tool) |

<h3 id="listsessions">
  `listSessions()`
</h3>

Scopre ed elenca le sessioni passate con metadati leggeri. Filtra per directory di progetto o elenca le sessioni in tutti i progetti.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  Parametri
</h4>

| Parametro                  | Tipo      | Predefinito | Descrizione                                                                                              |
| :------------------------- | :-------- | :---------- | :------------------------------------------------------------------------------------------------------- |
| `options.dir`              | `string`  | `undefined` | Directory per cui elencare le sessioni. Se omesso, restituisce le sessioni in tutti i progetti           |
| `options.limit`            | `number`  | `undefined` | Numero massimo di sessioni da restituire                                                                 |
| `options.includeWorktrees` | `boolean` | `true`      | Quando `dir` si trova all'interno di un repository git, includi le sessioni da tutti i percorsi worktree |

<h4 id="return-type-sdksessioninfo">
  Tipo di ritorno: `SDKSessionInfo`
</h4>

| Proprietà      | Tipo                  | Descrizione                                                                                         |
| :------------- | :-------------------- | :-------------------------------------------------------------------------------------------------- |
| `sessionId`    | `string`              | Identificatore di sessione univoco (UUID)                                                           |
| `summary`      | `string`              | Titolo di visualizzazione: titolo personalizzato, riepilogo generato automaticamente o primo prompt |
| `lastModified` | `number`              | Ora dell'ultima modifica in millisecondi dall'epoca                                                 |
| `fileSize`     | `number \| undefined` | Dimensione del file di sessione in byte. Popolato solo per l'archiviazione JSONL locale             |
| `customTitle`  | `string \| undefined` | Titolo della sessione impostato dall'utente (tramite `/rename`)                                     |
| `firstPrompt`  | `string \| undefined` | Primo prompt utente significativo nella sessione                                                    |
| `gitBranch`    | `string \| undefined` | Ramo Git alla fine della sessione                                                                   |
| `cwd`          | `string \| undefined` | Directory di lavoro per la sessione                                                                 |
| `tag`          | `string \| undefined` | Tag della sessione impostato dall'utente (vedi [`tagSession()`](#tagsession))                       |
| `createdAt`    | `number \| undefined` | Ora di creazione in millisecondi dall'epoca, dal timestamp della prima voce                         |

<h4 id="example-2">
  Esempio
</h4>

Stampa le 10 sessioni più recenti per un progetto. I risultati sono ordinati per `lastModified` decrescente, quindi il primo elemento è il più recente. Ometti `dir` per cercare in tutti i progetti.

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

<h3 id="getsessionmessages">
  `getSessionMessages()`
</h3>

Legge i messaggi dell'utente e dell'assistente da una trascrizione di sessione passata.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  Parametri
</h4>

| Parametro        | Tipo     | Predefinito  | Descrizione                                                                             |
| :--------------- | :------- | :----------- | :-------------------------------------------------------------------------------------- |
| `sessionId`      | `string` | obbligatorio | UUID della sessione da leggere (vedi `listSessions()`)                                  |
| `options.dir`    | `string` | `undefined`  | Directory del progetto in cui trovare la sessione. Se omesso, cerca in tutti i progetti |
| `options.limit`  | `number` | `undefined`  | Numero massimo di messaggi da restituire                                                |
| `options.offset` | `number` | `undefined`  | Numero di messaggi da saltare dall'inizio                                               |

<h4 id="return-type-sessionmessage">
  Tipo di ritorno: `SessionMessage`
</h4>

| Proprietà            | Tipo                    | Descrizione                                                                                                                                                                                                                                                                                  |
| :------------------- | :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Ruolo del messaggio                                                                                                                                                                                                                                                                          |
| `uuid`               | `string`                | Identificatore di messaggio univoco                                                                                                                                                                                                                                                          |
| `session_id`         | `string`                | Sessione a cui appartiene questo messaggio                                                                                                                                                                                                                                                   |
| `message`            | `unknown`               | Payload del messaggio grezzo dalla trascrizione                                                                                                                                                                                                                                              |
| `parent_tool_use_id` | `string \| null`        | Per i messaggi dei subagent, l'`tool_use_id` della chiamata del tool `Agent` che lo ha generato. `null` per i messaggi della sessione principale e le sessioni precedenti                                                                                                                    |
| `parent_agent_id`    | `string \| null`        | Per i messaggi da un [subagent annidato](/docs/it/sub-agents#spawn-nested-subagents), l'`agentId` del subagent che lo ha generato. `null` per i messaggi della sessione principale, i messaggi dai subagent di primo livello e le sessioni precedenti. Richiede Claude Code v2.1.202 o successivo |

<h4 id="example-3">
  Esempio
</h4>

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

<h3 id="getsessioninfo">
  `getSessionInfo()`
</h3>

Legge i metadati per una singola sessione per ID senza scansionare la directory del progetto completa.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  Parametri
</h4>

| Parametro     | Tipo     | Predefinito  | Descrizione                                                                                |
| :------------ | :------- | :----------- | :----------------------------------------------------------------------------------------- |
| `sessionId`   | `string` | obbligatorio | UUID della sessione da cercare                                                             |
| `options.dir` | `string` | `undefined`  | Percorso della directory del progetto. Se omesso, cerca in tutte le directory del progetto |

Restituisce [`SDKSessionInfo`](#return-type-sdksessioninfo), o `undefined` se la sessione non viene trovata.

<h3 id="renamesession">
  `renameSession()`
</h3>

Rinomina una sessione aggiungendo una voce di titolo personalizzato. Le chiamate ripetute sono sicure; il titolo più recente vince.

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  Parametri
</h4>

| Parametro     | Tipo     | Predefinito  | Descrizione                                                                                |
| :------------ | :------- | :----------- | :----------------------------------------------------------------------------------------- |
| `sessionId`   | `string` | obbligatorio | UUID della sessione da rinominare                                                          |
| `title`       | `string` | obbligatorio | Nuovo titolo. Deve essere non vuoto dopo il trimming dello spazio bianco                   |
| `options.dir` | `string` | `undefined`  | Percorso della directory del progetto. Se omesso, cerca in tutte le directory del progetto |

<h3 id="tagsession">
  `tagSession()`
</h3>

Etichetta una sessione. Passa `null` per cancellare l'etichetta. Le chiamate ripetute sono sicure; l'etichetta più recente vince.

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  Parametri
</h4>

| Parametro     | Tipo             | Predefinito  | Descrizione                                                                                |
| :------------ | :--------------- | :----------- | :----------------------------------------------------------------------------------------- |
| `sessionId`   | `string`         | obbligatorio | UUID della sessione da etichettare                                                         |
| `tag`         | `string \| null` | obbligatorio | Stringa di etichetta, o `null` per cancellare                                              |
| `options.dir` | `string`         | `undefined`  | Percorso della directory del progetto. Se omesso, cerca in tutte le directory del progetto |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

Risolve le impostazioni effettive di Claude Code per una determinata directory utilizzando lo stesso motore di merge della CLI, senza generare la CLI Claude. Utilizzalo per ispezionare quale configurazione una chiamata `query()` vedrebbe prima di invocarne una.

<Note>
  Questa funzione è in fase alpha e la sua API potrebbe cambiare prima della stabilizzazione. Legge le fonti MDM, inclusi plist macOS e Windows HKLM/HKCU, per la parità con l'avvio della CLI, ma non esegue il subprocess `policyHelper` configurato dall'amministratore. Il campo `permissions.defaultMode` viene restituito così com'è da tutti i livelli incluse le impostazioni del progetto. Il filtro di fiducia che la CLI applica prima di onorare i modi di autorizzazione crescenti non viene applicato.
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parametri
</h4>

`resolveSettings()` accetta un singolo oggetto di opzioni. Tutti i campi sono opzionali.

| Parametro                       | Tipo                                  | Predefinito     | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------------ | :------------------------------------ | :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | Directory per risolvere le impostazioni di progetto e locali relative a                                                                                                                                                                                                                                                                                                                                                           |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Tutte le fonti  | Quali fonti del filesystem caricare. Passa `[]` per saltare le impostazioni utente, progetto e locali. Le impostazioni della politica gestita si caricano in tutti i casi. Le impostazioni gestite dal server vengono prese da `serverManagedSettings` quando l'host le passa, o lette dalla cache su disco della CLI altrimenti; lo snapshot non le recupera dalla rete                                                          |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Impostazioni della politica restrittiva fornite dall'host di incorporamento. Eliminate quando è presente un livello gestito distribuito dall'amministratore; unite sotto quel livello quando [`parentSettingsBehavior`](/docs/it/settings#available-settings) è `"merge"`. Le chiavi non restrittive come `model` vengono silenziosamente eliminate in modo che questa opzione possa restringere la politica gestita ma non allentarla |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Payload delle impostazioni gestite dal server da `/api/claude_code/settings`. Le chiavi non restrittive passano attraverso senza filtri                                                                                                                                                                                                                                                                                           |

<h4 id="return-type-resolvedsettings">
  Tipo di ritorno: `ResolvedSettings`
</h4>

`resolveSettings()` restituisce un oggetto che descrive le impostazioni unite e la fonte che ha contribuito a ogni chiave.

| Proprietà    | Tipo                                                | Descrizione                                                                                |
| :----------- | :-------------------------------------------------- | :----------------------------------------------------------------------------------------- |
| `effective`  | `Settings`                                          | Impostazioni unite dopo l'applicazione di tutte le fonti abilitate in ordine di precedenza |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Per ogni chiave di primo livello in `effective`, quale fonte ha fornito il valore          |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Impostazioni grezze per fonte, ordinate dalla precedenza più bassa a quella più alta       |

<h4 id="example-4">
  Esempio
</h4>

L'esempio seguente risolve le impostazioni per una directory di progetto e stampa la fonte che controlla il periodo di pulizia.

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

<h2 id="types">
  Tipi
</h2>

<h3 id="options">
  `Options`
</h3>

Oggetto di configurazione per la funzione `query()`.

| Proprietà                         | Tipo                                                                                                     | Predefinito                                     | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :---------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`                         | Controller per annullare le operazioni                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                                            | Directory aggiuntive a cui Claude può accedere                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agent`                           | `string`                                                                                                 | `undefined`                                     | Nome dell'agente per il thread principale. L'agente deve essere definito nell'opzione `agents` o nelle impostazioni                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                                     | Definisci programmaticamente i subagenti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                                         | Quando `true`, genera riassunti di progresso a una riga per i subagenti e inoltrarli su eventi [`task_progress`](#sdktaskprogressmessage) tramite il campo `summary`. Si applica ai subagenti in primo piano e in background                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                                         | Abilita il bypass dei permessi. Obbligatorio quando si usa `permissionMode: 'bypassPermissions'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                                            | Tool da approvare automaticamente senza richiedere. Questo non limita Claude a solo questi tool; i tool non elencati ricadono in `permissionMode` e `canUseTool`. Usa `disallowedTools` per bloccare i tool. Vedi [Permessi](/docs/it/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                       |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                                            | Abilita le funzioni beta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                                     | Funzione di permesso personalizzata, invocata solo quando il [flusso di permesso](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) ricade in un prompt. Non invocata per le chiamate pre-approvate da `allowedTools`, regole di autorizzazione, o `permissionMode`. `AskUserQuestion`, tool connettore [impostati dalla tua organizzazione su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e tool MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) la raggiungono anche se li hai consentiti; in modalità `dontAsk` questi vengono negati invece. Vedi [`CanUseTool`](#canusetool) per i dettagli |
| `continue`                        | `boolean`                                                                                                | `false`                                         | Continua la conversazione più recente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`                                 | Directory di lavoro corrente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `debug`                           | `boolean`                                                                                                | `false`                                         | Abilita la modalità debug per il processo Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `debugFile`                       | `string`                                                                                                 | `undefined`                                     | Scrivi i log di debug in un percorso di file specifico. Abilita implicitamente la modalità debug                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                                            | Tool da negare. Un nome semplice come `"Bash"` rimuove il tool dal contesto di Claude. Una regola con ambito come `"Bash(rm *)"` lascia il tool disponibile e nega le chiamate corrispondenti in ogni modalità di permesso, incluso `bypassPermissions`. Vedi [Permessi](/docs/it/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                           |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | Predefinito del modello                         | Controlla quanto sforzo Claude mette nella sua risposta. Funziona con il pensiero adattivo per guidare la profondità del pensiero. Vedi [regola il livello di sforzo](/docs/it/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                                         | Abilita il tracciamento dei cambiamenti di file per il rewind. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                                   | Variabili di ambiente. Quando impostato, questo sostituisce l'ambiente del subprocess invece di unirsi a `process.env`, quindi passa `{ ...process.env, YOUR_VAR: 'value' }` per mantenere le variabili ereditate come `PATH`. Vedi [Gestisci risposte API lente o bloccate](#handle-slow-or-stalled-api-responses) per un esempio di questo modello, e [Variabili di ambiente](/docs/it/env-vars) per le variabili che la CLI sottostante legge. Imposta `CLAUDE_AGENT_SDK_CLIENT_APP` per identificare la tua app nell'intestazione User-Agent                                                                                                                        |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | Auto-rilevato                                   | Runtime JavaScript da usare                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                                            | Argomenti da passare all'eseguibile                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                                            | Argomenti aggiuntivi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                                     | Modello da usare se il primario fallisce                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `forkSession`                     | `boolean`                                                                                                | `false`                                         | Quando si riprende con `resume`, esegui il fork a un nuovo ID di sessione invece di continuare la sessione originale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                                         | Inoltra i blocchi di testo e pensiero dei subagenti come messaggi dell'assistente e dell'utente con `parent_tool_use_id` impostato, in modo che i consumer possano renderizzare una trascrizione nidificata. Per impostazione predefinita, solo i blocchi `tool_use` e `tool_result` dai subagenti vengono emessi                                                                                                                                                                                                                                                                                                                                                  |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                                            | Callback hook per gli eventi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                                         | Includi gli eventi del ciclo di vita hook per ogni evento hook nel flusso di messaggi come [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), e [`SDKHookResponseMessage`](#sdkhookresponsemessage). Gli eventi del ciclo di vita per gli hook `SessionStart` e `Setup` sono sempre inclusi e non hanno bisogno di questa opzione                                                                                                                                                                                                                                                                             |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                                         | Includi gli eventi di messaggi parziali                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                                         | *Alpha.* Timeout in millisecondi per ogni chiamata `sessionStore.load()` e `sessionStore.listSubkeys()` durante la materializzazione del resume. Se l'adapter non si stabilizza entro questa finestra, la query fallisce invece di bloccarsi. Ignorato quando `sessionStore` non è impostato                                                                                                                                                                                                                                                                                                                                                                       |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                                     | Impostazioni a livello di politica fornite dal processo genitore che genera. Eliminate quando un livello di impostazioni gestite controllato da IT esiste già sulla macchina, a meno che l'amministratore non acconsenta con `parentSettingsBehavior: 'merge'`. Filtrate solo alle chiavi restrittive indipendentemente                                                                                                                                                                                                                                                                                                                                            |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                                     | Interrompi la query quando la stima del costo lato client raggiunge questo valore in USD. Confrontato con la stessa stima di `total_cost_usd`; vedi [Traccia costo e utilizzo](/docs/it/agent-sdk/cost-tracking) per le avvertenze di accuratezza                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                                     | *Deprecato:* Usa `thinking` invece. Token massimi per il processo di pensiero                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                                     | Turni agentici massimi (round trip di uso dei tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                                            | Configurazioni del server MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `model`                           | `string`                                                                                                 | Predefinito da CLI                              | Alias del modello Claude o nome completo del modello. Vedi [valori accettati e ID specifici del provider](/docs/it/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                                     | Callback per gestire le richieste di elicitazione MCP. Chiamato quando un server MCP richiede input dell'utente e nessun hook lo gestisce per primo. Se non fornito, le richieste di elicitazione non gestite vengono rifiutate automaticamente                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                                     | Definisci il formato di output per i risultati dell'agente. Vedi [Output strutturati](/docs/it/agent-sdk/structured-outputs) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                                     | Non è un campo `Options`. Imposta `outputStyle` nell'oggetto [`settings`](/docs/it/settings) inline o in un file di impostazioni. Vedi [Attiva uno stile di output](/docs/it/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | Auto-risolto dal binario nativo raggruppato     | Percorso all'eseguibile Claude Code. Necessario solo se le dipendenze opzionali sono state saltate durante l'installazione o la tua piattaforma non è nel set supportato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                                     | Modalità di permesso per la sessione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                                     | Nome del tool MCP per i prompt di permesso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `persistSession`                  | `boolean`                                                                                                | `true`                                          | Quando `false`, disabilita la persistenza della sessione su disco. Le sessioni non possono essere riprese in seguito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                                     | Istruzioni di flusso di lavoro personalizzate per la modalità plan. Quando `permissionMode` è `'plan'`, questa stringa sostituisce il corpo del flusso di lavoro della modalità plan predefinito. La CLI lo avvolge comunque con il preambolo di applicazione di sola lettura e il footer del protocollo ExitPlanMode                                                                                                                                                                                                                                                                                                                                              |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                                            | Carica plugin personalizzati da percorsi locali. Vedi [Plugins](/docs/it/agent-sdk/plugins) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                                         | Abilita i suggerimenti di prompt. Emette un messaggio `prompt_suggestion` dopo ogni turno con un prompt utente successivo previsto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `resume`                          | `string`                                                                                                 | `undefined`                                     | ID della sessione da riprendere                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                                     | Riprendi la sessione a un UUID di messaggio specifico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                                     | Configura il comportamento della sandbox a livello di programmazione. Vedi [Impostazioni sandbox](#sandboxsettings) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `sessionId`                       | `string`                                                                                                 | Auto-generato                                   | Usa un UUID specifico per la sessione invece di generarne uno automaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `sessionStore`                    | [`SessionStore`](/docs/it/agent-sdk/session-storage#the-sessionstore-interface)                               | `undefined`                                     | Specchia i trascritti della sessione in un backend esterno in modo che qualsiasi host possa riprenderli. Vedi [Persisti le sessioni nell'archiviazione esterna](/docs/it/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                                     | *Alpha.* Modalità di flush per `sessionStore`. Ignorato quando `sessionStore` non è impostato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                                     | Oggetto [impostazioni](/docs/it/settings) inline o percorso a un file di impostazioni. Popola il livello flag-settings nell'[ordine di precedenza](/docs/it/settings#settings-precedence). Cambia a runtime con [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | Impostazioni predefinite CLI (tutte le fonti)   | Controlla quali impostazioni del filesystem caricare. Passa `[]` per disabilitare le impostazioni utente, progetto e locali. Le impostazioni della politica gestita vengono caricate indipendentemente; le impostazioni gestite dal server vengono recuperate quando la sessione si autentica con una credenziale organizzativa su una [configurazione idonea](/docs/it/server-managed-settings#platform-availability). Vedi [Usa le funzioni Claude Code](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                     |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                                     | Skills disponibili per la sessione. Passa `'all'` per abilitare ogni skill scoperta, o un elenco di nomi di skill. Quando impostato, l'SDK aggiunge automaticamente lo strumento Skill a `allowedTools`. Se passi anche `tools`, includi `'Skill'` in quell'elenco. Vedi [Skills](/docs/it/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                            |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                                     | Funzione personalizzata per generare il processo Claude Code. Usa per eseguire Claude Code in VM, container o ambienti remoti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                                     | Callback per l'output stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                                         | Usa solo i server passati in `mcpServers` e ignora il progetto `.mcp.json`, le impostazioni utente, i server MCP forniti dai plugin e i [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined` (prompt minimo)                     | Configurazione del prompt di sistema. Passa una stringa per un prompt personalizzato, o `{ type: 'preset', preset: 'claude_code' }` per usare il prompt di sistema di Claude Code. Quando si usa la forma dell'oggetto preset, aggiungi `append` per estenderlo con istruzioni aggiuntive, e imposta `excludeDynamicSections: true` per spostare il contesto per sessione nel primo messaggio utente per un [migliore riutilizzo della cache dei prompt tra le macchine](/docs/it/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                                                  |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                                     | *Alpha.* Budget di attività lato API in token. Quando impostato, il modello viene informato del suo budget di token rimanente in modo che possa regolare l'uso dei tool e concludere prima del limite                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | `{ type: 'adaptive' }` per i modelli supportati | Controlla il comportamento di pensiero/ragionamento di Claude. Vedi [`ThinkingConfig`](#thinkingconfig) per le opzioni                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `title`                           | `string`                                                                                                 | `undefined`                                     | Titolo di visualizzazione per la sessione. Quando si riprende tramite `resume` o `continue`, il titolo persistente della sessione ripresa ha la precedenza; usa [`renameSession()`](#renamesession) per rinominare una sessione esistente                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                                     | Mappa i nomi dei tool incorporati ai nomi dei tool MCP in modo che Claude chiami la tua implementazione MCP al posto di quella incorporata. Ad esempio, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                                     | Configurazione per il comportamento dei tool incorporati. Vedi [`ToolConfig`](#toolconfig) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                                     | Configurazione dei tool. Passa un array di nomi di tool o usa il preset per ottenere i tool predefiniti di Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

<h4 id="handle-slow-or-stalled-api-responses">
  Gestisci risposte API lente o bloccate
</h4>

Il subprocess CLI legge diverse variabili di ambiente che controllano i timeout dell'API e il rilevamento dei blocchi. Passale attraverso l'opzione `env`:

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* `API_TIMEOUT_MS`: timeout per richiesta sul client Anthropic, in millisecondi. Predefinito `600000`. Si applica al loop principale e a tutti i subagenti.
* `CLAUDE_CODE_MAX_RETRIES`: numero massimo di tentativi API. Predefinito `10`, limitato a `15`. Ogni tentativo ottiene la propria finestra `API_TIMEOUT_MS`, quindi il tempo wall case peggiore è approssimativamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` più backoff. Per esecuzioni incustodite che devono attendere attraverso interruzioni più lunghe, imposta `CLAUDE_CODE_RETRY_WATCHDOG=1`: ritenta gli errori di capacità indefinitamente, e a partire da Claude Code v2.1.199 aumenta il predefinito per altri errori transitori a `300` e rimuove il limite su questa variabile.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog di blocco per i subagenti lanciati con `run_in_background`. Predefinito `600000`. Si ripristina su ogni evento di stream; al blocco interrompe il subagente, contrassegna l'attività come fallita e presenta l'errore al genitore con qualsiasi risultato parziale. Non si applica ai subagenti sincroni.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` con `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: interrompe la richiesta quando le intestazioni sono arrivate ma il corpo della risposta smette di trasmettere. Il watchdog è attivo per impostazione predefinita per tutti i provider; imposta `CLAUDE_ENABLE_STREAM_WATCHDOG=0` per disabilitarlo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` predefinito a `300000` ed è limitato a quel minimo. La richiesta interrotta passa attraverso il percorso di tentativo normale.

<h3 id="query-object">
  Oggetto `Query`
</h3>

Interfaccia restituita dalla funzione `query()`.

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: { [K in keyof Settings]?: Settings[K] | null }): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Metodi
</h4>

| Metodo                                 | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Interrompe la query. Disponibile solo in modalità input streaming. Quando la CLI pubblicizza la capacità `interrupt_receipt_v1` in [`SDKSystemMessage.capabilities`](#sdksystemmessage), si risolve con un [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) che elenca i messaggi in coda che sopravvivono all'interruzione. Si risolve `undefined` su CLI prima della v2.1.205                                                                                                                                                          |
| `rewindFiles(userMessageId, options?)` | Ripristina i file al loro stato al messaggio utente specificato. Passa `{ dryRun: true }` per visualizzare in anteprima i cambiamenti. Richiede `enableFileCheckpointing: true`. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                          |
| `setPermissionMode()`                  | Cambia la modalità di permesso (disponibile solo in modalità input streaming)                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `setModel()`                           | Cambia il modello (disponibile solo in modalità input streaming)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `setMaxThinkingTokens()`               | *Deprecato:* Usa l'opzione `thinking` invece. Cambia i token di pensiero massimi. Passare `null` ripristina il pensiero al predefinito della sessione: un override a metà sessione viene cancellato, e il pensiero rimane disattivato per le sessioni che lo hanno disabilitato                                                                                                                                                                                                                                                                       |
| `applyFlagSettings(settings)`          | Unisce le impostazioni nel livello flag settings della sessione a runtime (disponibile solo in modalità input streaming). Vedi [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                            |
| `initializationResult()`               | Restituisce il risultato di inizializzazione completo inclusi i comandi supportati, i modelli, le informazioni dell'account e la configurazione dello stile di output                                                                                                                                                                                                                                                                                                                                                                                 |
| `reinitialize()`                       | Rinvia la richiesta di controllo `initialize` al CLI in esecuzione e restituisce un risultato fresco invece del risultato della prima connessione memorizzato nella cache. Usalo dopo un gap di trasporto, come il ricollegamento a una sessione dopo una disconnessione, in modo che le richieste di permesso in sospeso raggiungano di nuovo il tuo callback `canUseTool`. Rendi il callback idempotente per ID di richiesta, perché una richiesta la cui risposta è stata persa viene inviata di nuovo. Richiede Claude Code v2.1.195 o successivo |
| `supportedCommands()`                  | Restituisce i comandi slash disponibili                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `supportedModels()`                    | Restituisce i modelli disponibili con le informazioni di visualizzazione                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `supportedAgents()`                    | Restituisce i subagenti disponibili come [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `mcpServerStatus()`                    | Restituisce lo stato dei server MCP connessi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `accountInfo()`                        | Restituisce le informazioni dell'account                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `reconnectMcpServer(serverName)`       | Ricollega un server MCP per nome                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `toggleMcpServer(serverName, enabled)` | Abilita o disabilita un server MCP per nome                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `setMcpServers(servers)`               | Sostituisci dinamicamente l'insieme dei server MCP per questa sessione. Restituisce informazioni su quali server sono stati aggiunti, rimossi e eventuali errori                                                                                                                                                                                                                                                                                                                                                                                      |
| `streamInput(stream)`                  | Trasmetti i messaggi di input alla query per le conversazioni multi-turno                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `stopTask(taskId)`                     | Interrompi un'attività di background in esecuzione per ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `close()`                              | Chiudi la query e termina il processo sottostante. Termina forzatamente la query e pulisce tutte le risorse                                                                                                                                                                                                                                                                                                                                                                                                                                           |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Cambia qualsiasi [impostazione](/docs/it/settings) su una sessione in esecuzione senza riavviare la query. Usalo quando un'impostazione che non ha un setter dedicato deve cambiare a metà sessione, come irrigidire `permissions` dopo che l'agente legge input non attendibile. `setModel()` e `setPermissionMode()` sono setter dedicati per quelle due chiavi; `applyFlagSettings()` è la forma generale che accetta qualsiasi sottoinsieme delle chiavi di impostazioni, e passare `model` qui si comporta come `setModel()`.

Solo alcune chiavi hanno effetto a metà sessione:

* **Applicate al turno successivo**: `model`, `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Cambiare `agent` applica anche l'override del modello di quell'agente, gli hook e il prompt di sistema al turno successivo.
* **Nessun effetto a metà sessione**: le opzioni del prompt di sistema. Questi vengono risolti una volta all'avvio, quindi la sessione in esecuzione mantiene il valore originale anche se la chiamata ha successo. Per cambiarli, avvia una nuova sessione.

`effortLevel` accetta un nome di [livello di sforzo](/docs/it/model-config#adjust-effort-level). Accetta anche `"ultracode"`, che esegue la sessione a sforzo `xhigh` e attiva [ultracode](/docs/it/workflows#let-claude-decide-with-ultracode). Il tipo `Settings` dichiara `effortLevel` senza quel valore, quindi passa l'equivalente `{ ultracode: true }` in TypeScript. Il valore `ultracode` richiede Claude Code v2.1.203 o successivo ed è accettato solo da `applyFlagSettings()`, non dalla chiave `effortLevel` in un file di impostazioni.

I valori vengono scritti nel livello flag-settings, lo stesso livello che l'opzione `settings` inline di `query()` popola all'avvio. Le impostazioni flag si trovano vicino alla parte superiore dell'[ordine di precedenza delle impostazioni](/docs/it/settings#settings-precedence): sovrascrivono le impostazioni utente, progetto e locali, e solo le impostazioni della politica gestita possono sovrascriverle. Questo è lo stesso livello che la [sezione di precedenza in pagina](#settings-precedence) chiama opzioni programmatiche.

Le chiamate successive eseguono un shallow-merge delle chiavi di livello superiore. Una seconda chiamata con `{ permissions: {...} }` sostituisce l'intero oggetto `permissions` dalla chiamata precedente piuttosto che eseguire un deep-merge in esso. Per cancellare una chiave dal livello flag e ricadere in fonti di precedenza inferiore, passa `null` per quella chiave. Passare `undefined` non ha effetto perché la serializzazione JSON lo elimina.

Disponibile solo in modalità input streaming, lo stesso vincolo di `setModel()` e `setPermissionMode()`.

L'esempio seguente cambia il modello attivo a metà sessione, quindi cancella l'override in modo che il modello ricada in qualsiasi cosa specifichino le impostazioni utente o progetto.

```typescript theme={null}
const q = query({ prompt: messageStream });

// Sovrascrivi il modello per il resto della sessione
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Più tardi: cancella l'override e ricadi alle impostazioni di precedenza inferiore
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` è solo TypeScript. L'SDK Python non espone un metodo equivalente.
</Note>

<h3 id="warmquery">
  `WarmQuery`
</h3>

Handle restituito da [`startup()`](#startup). Il subprocess è già generato e inizializzato, quindi chiamare `query()` su questo handle scrive il prompt direttamente in un processo pronto senza latenza di avvio.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Metodi
</h4>

| Metodo          | Descrizione                                                                                                                                 |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `query(prompt)` | Invia un prompt al subprocess pre-riscaldato e restituisci una [`Query`](#query-object). Può essere chiamato solo una volta per `WarmQuery` |
| `close()`       | Chiudi il subprocess senza inviare un prompt. Usa questo per scartare una query calda che non è più necessaria                              |

`WarmQuery` implementa `AsyncDisposable`, quindi può essere usato con `await using` per la pulizia automatica.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Tipo di ritorno di `initializationResult()`. Contiene i dati di inizializzazione della sessione.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
};
```

Quando un client invia `initialize` a una sessione che è già in esecuzione, il wrapper di risposta di controllo porta anche un array `pending_permission_requests` opzionale. Il campo si trova sul wrapper di risposta stesso, non nel payload `SDKControlInitializeResponse` sopra. Ogni voce è un messaggio `control_request` completo con la stessa forma `{ type: "control_request", request_id, request }` che la sessione trasmette per le richieste di permesso durante l'esecuzione.

Queste sono richieste che sono state emesse prima che il client si connettesse e sono ancora in attesa di una risposta. L'SDK legge l'array per te e invia ogni voce al tuo callback [`canUseTool`](#canusetool), lo stesso reinvio che [`reinitialize()`](#query-object) attiva dopo un gap di trasporto. Gestisci gli ID di richiesta ripetuti in modo idempotente, perché una voce può ripetere una richiesta che il callback ha già ricevuto prima che la connessione si interrompesse.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

La ricevuta di interruzione: il valore che [`interrupt()`](#query-object) si risolve con su una CLI che pubblicizza la capacità `interrupt_receipt_v1` in [`SDKSystemMessage.capabilities`](#sdksystemmessage). Richiede Claude Code v2.1.205 o successivo. Le CLI precedenti rispondono all'interruzione con un payload di successo vuoto, quindi `interrupt()` si risolve a `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued` elenca gli UUID dei messaggi utente che sopravvivono all'interruzione: messaggi ancora nella coda, più qualsiasi batch già rimosso dalla coda per il turno successivo ma non ancora raggiungibile dall'interruzione. Ognuno viene eseguito come il suo turno dopo l'interruzione a meno che non lo annulli per primo. Usa la ricevuta per decidere se rinviare qualcosa; rinviare un messaggio che è già elencato produce un turno duplicato.

Interpreta l'elenco con questi avvertimenti:

* Solo i messaggi che sono stati accodati con un UUID appaiono. Un array vuoto non significa che nient'altro verrà eseguito.
* Solo i messaggi del thread principale sono elencati. I messaggi indirizzati a un subagente sono fuori portata.
* L'elenco può includere UUID che il tuo client non ha mai inviato, come i trigger di [attività pianificate](/docs/it/scheduled-tasks). Ignora gli UUID che non riconosci invece di trattarli come un errore.

La ricevuta è uno snapshot scattato nel momento in cui l'interruzione viene elaborata, e su un'interruzione pulita arriva prima del [`SDKResultMessage`](#sdkresultmessage) del turno interrotto. Leggi la ricevuta piuttosto che ispezionare la coda dopo quel risultato: il loop avvia il turno in coda successivo immediatamente, quindi la coda che ispezioni dopo il risultato è già cambiata.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configurazione per un subagente definito programmaticamente.

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  background?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Campo                                 | Obbligatorio | Descrizione                                                                                                                                                                                                                                             |
| :------------------------------------ | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`                         | Sì           | Descrizione in linguaggio naturale di quando usare questo agente                                                                                                                                                                                        |
| `tools`                               | No           | Array di nomi di tool consentiti. Se omesso, eredita tutti i tool dal genitore. Per precaricare Skills nel contesto dell'agente, usa il campo `skills` piuttosto che elencando `'Skill'` qui                                                            |
| `disallowedTools`                     | No           | Array di nomi di tool da esplicitamente disabilitare per questo agente. Sono accettati anche i modelli a livello di server MCP: `mcp__server` o `mcp__server__*` rimuove ogni tool da quel server, e `mcp__*` rimuove ogni tool MCP da qualsiasi server |
| `prompt`                              | Sì           | Il prompt di sistema dell'agente                                                                                                                                                                                                                        |
| `model`                               | No           | Override del modello per questo agente. Accetta un alias come `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, o un ID modello completo. Se omesso o `'inherit'`, usa il modello principale                                                     |
| `mcpServers`                          | No           | Specifiche del server MCP per questo agente                                                                                                                                                                                                             |
| `skills`                              | No           | Array di nomi di skill da precaricare nel contesto dell'agente                                                                                                                                                                                          |
| `initialPrompt`                       | No           | Auto-inviato come il primo turno utente quando questo agente viene eseguito come agente del thread principale                                                                                                                                           |
| `maxTurns`                            | No           | Numero massimo di turni agentici (round trip API) prima di fermarsi                                                                                                                                                                                     |
| `background`                          | No           | Esegui questo agente come un'attività di background non bloccante quando invocato                                                                                                                                                                       |
| `memory`                              | No           | Fonte di memoria per questo agente: `'user'`, `'project'`, o `'local'`                                                                                                                                                                                  |
| `effort`                              | No           | Livello di sforzo di ragionamento per questo agente. Accetta un livello denominato o un numero intero                                                                                                                                                   |
| `permissionMode`                      | No           | Modalità di permesso per l'esecuzione dei tool all'interno di questo agente. Vedi [`PermissionMode`](#permissionmode)                                                                                                                                   |
| `criticalSystemReminder_EXPERIMENTAL` | No           | Sperimentale: Promemoria critico aggiunto al prompt di sistema                                                                                                                                                                                          |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Specifica i server MCP disponibili per un subagente. Può essere un nome di server (stringa che fa riferimento a un server dalla configurazione `mcpServers` del genitore) o una configurazione di server inline che mappa i nomi dei server alle configurazioni.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Dove `McpServerConfigForProcessTransport` è `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`.

<h3 id="settingsource">
  `SettingSource`
</h3>

Controlla quali fonti di configurazione basate su filesystem l'SDK carica le impostazioni da.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Valore      | Descrizione                                                       | Posizione                     |
| :---------- | :---------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Impostazioni globali dell'utente                                  | `~/.claude/settings.json`     |
| `'project'` | Impostazioni del progetto condivise (controllate dalla versione)  | `.claude/settings.json`       |
| `'local'`   | Impostazioni del progetto locale (non controllate dalla versione) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamento predefinito
</h4>

Quando `settingSources` è omesso o `undefined`, `query()` carica le stesse impostazioni del filesystem del CLI Claude Code: utente, progetto e locale. Le impostazioni della politica gestita vengono caricate in tutti i casi; le impostazioni gestite dal server vengono recuperate quando la sessione si autentica con una credenziale organizzativa su una [configurazione idonea](/docs/it/server-managed-settings#platform-availability). Vedi [Cosa settingSources non controlla](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control) per gli input che vengono letti indipendentemente da questa opzione, e come disabilitarli.

<h4 id="why-use-settingsources">
  Perché usare settingSources
</h4>

**Disabilita le impostazioni del filesystem:**

```typescript theme={null}
// Non caricare le impostazioni utente, progetto o locali dal disco
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Carica tutte le impostazioni del filesystem esplicitamente:**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // Carica tutte le impostazioni
  }
});
```

**Carica solo fonti di impostazioni specifiche:**

```typescript theme={null}
// Carica solo le impostazioni del progetto, ignora utente e locale
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Solo .claude/settings.json
  }
});
```

**Ambienti di test e CI:**

```typescript theme={null}
// Assicura un comportamento coerente in CI escludendo le impostazioni locali
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // Solo impostazioni condivise dal team
    permissionMode: "bypassPermissions"
  }
});
```

**Applicazioni solo SDK:**

```typescript theme={null}
// Definisci tutto a livello di programmazione.
// Passa [] per rinunciare alle fonti di impostazioni del filesystem.
const result = query({
  prompt: "Review this PR",
  options: {
    settingSources: [],
    agents: {
      /* ... */
    },
    mcpServers: {
      /* ... */
    },
    allowedTools: ["Read", "Grep", "Glob"]
  }
});
```

**Caricamento delle istruzioni del progetto CLAUDE.md:**

```typescript theme={null}
// Carica le impostazioni del progetto per includere i file CLAUDE.md
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // Usa il prompt di sistema di Claude Code
    },
    settingSources: ["project"], // Carica CLAUDE.md dalla directory del progetto
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

<h4 id="settings-precedence">
  Precedenza delle impostazioni
</h4>

Quando più fonti vengono caricate, le impostazioni vengono unite con questa precedenza (più alta a più bassa):

1. Impostazioni locali (`.claude/settings.local.json`)
2. Impostazioni del progetto (`.claude/settings.json`)
3. Impostazioni dell'utente (`~/.claude/settings.json`)

Le opzioni programmatiche come `agents`, `allowedTools` e `settings` sovrascrivono le impostazioni del filesystem utente, progetto e locale. Le impostazioni della politica gestita hanno precedenza sulle opzioni programmatiche.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Comportamento di permesso standard
  | "acceptEdits" // Auto-accetta le modifiche ai file
  | "bypassPermissions" // Bypass di tutti i controlli di permesso; le regole di richiesta esplicita richiedono comunque
  | "plan" // Modalità di pianificazione - esplora senza modificare
  | "dontAsk" // Non richiedere i permessi, nega se non pre-approvato
  | "auto"; // Usa un classificatore di modello per approvare o negare ogni chiamata di tool
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Tipo di funzione di permesso personalizzato per controllare l'uso dei tool.

La funzione è la sostituzione SDK per il prompt di permesso interattivo: viene invocata solo quando il [flusso di valutazione del permesso](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) si risolve in un prompt. Le chiamate di tool già approvate da una voce `allowedTools`, una regola di autorizzazione nelle impostazioni, o la modalità di permesso, come `acceptEdits` o `bypassPermissions`, non la invocano mai. Per controllare ogni chiamata di tool, usa un [hook `PreToolUse`](/docs/it/agent-sdk/hooks) invece.

`AskUserQuestion`, tool MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool), e tool connettore [impostati dalla tua organizzazione su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) la raggiungono anche quando una regola di autorizzazione corrisponde. In modalità `dontAsk` queste chiamate vengono negate invece, senza invocarla.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Opzione          | Tipo                                        | Descrizione                                                                                                                                                                                                                                                                                                                                  |
| :--------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Segnalato se l'operazione deve essere interrotta                                                                                                                                                                                                                                                                                             |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Aggiornamenti di permesso suggeriti in modo che l'utente non venga richiesto di nuovo per questo tool. I prompt Bash includono un suggerimento con la destinazione [`localSettings`](#permissionupdatedestination), quindi restituirlo in `updatedPermissions` scrive la regola in `.claude/settings.local.json` e persiste tra le sessioni. |
| `blockedPath`    | `string`                                    | Il percorso del file che ha attivato la richiesta di permesso, se applicabile                                                                                                                                                                                                                                                                |
| `decisionReason` | `string`                                    | Spiega perché questa richiesta di permesso è stata attivata                                                                                                                                                                                                                                                                                  |
| `toolUseID`      | `string`                                    | Identificatore univoco per questa specifica chiamata di tool all'interno del messaggio dell'assistente                                                                                                                                                                                                                                       |
| `agentID`        | `string`                                    | Se in esecuzione all'interno di un sub-agente, l'ID del sub-agente                                                                                                                                                                                                                                                                           |
| `requestId`      | `string`                                    | L'`request_id` dell'envelope `control_request`. Una `control_response` che la tua applicazione invia al di fuori dell'SDK, come un POST HTTP firmato, deve ripetere questo valore in modo che il processo Claude Code possa abbinare la risposta alla richiesta                                                                              |

Il callback normalmente risolve la richiesta restituendo un [`PermissionResult`](#permissionresult), che l'SDK scrive di nuovo sul suo trasporto come `control_response`. Restituisci `null` solo quando la tua applicazione ha già inviato la `control_response` per questa richiesta sul suo canale, ripetendo `requestId`; l'SDK quindi salta la scrittura della risposta al suo trasporto. Restituire `null` in qualsiasi altro caso lascia la chiamata di tool bloccata indefinitamente, perché nessuna `control_response` viene mai inviata e i prompt di permesso non scadono.

L'opzione `requestId` e il valore di ritorno `null` richiedono Claude Code v2.1.199 o successivo.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Risultato di un controllo di permesso.

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

<h3 id="toolconfig">
  `ToolConfig`
</h3>

Configurazione per il comportamento dei tool incorporati.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Campo                           | Tipo                   | Descrizione                                                                                                                                                                                 |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Acconsente al campo `preview` su [`AskUserQuestion`](/docs/it/agent-sdk/user-input#question-format) opzioni e imposta il suo formato di contenuto. Se non impostato, Claude non emette anteprime |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Configurazione per i server MCP.

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcpsdkserverconfigwithinstance">
  `McpSdkServerConfigWithInstance`
</h4>

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  instance: McpServer;
};
```

<h4 id="mcpclaudeaiproxyserverconfig">
  `McpClaudeAIProxyServerConfig`
</h4>

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Configurazione per il caricamento dei plugin nell'SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Campo              | Tipo      | Descrizione                                                                                                                                                                                                                |
| :----------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Deve essere `'local'` (attualmente supportati solo plugin locali)                                                                                                                                                          |
| `path`             | `string`  | Percorso assoluto o relativo alla directory del plugin                                                                                                                                                                     |
| `skipMcpDiscovery` | `boolean` | Quando `true`, l'SDK carica skills, hooks, agenti e comandi da questo plugin ma non legge il suo `.mcp.json` o il manifest `mcpServers`. Imposta questo quando la tua applicazione possiede le connessioni MCP del plugin. |

**Esempio:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Per informazioni complete sulla creazione e l'uso dei plugin, vedi [Plugins](/docs/it/agent-sdk/plugins).

<h2 id="message-types">
  Tipi di messaggio
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

Tipo di unione di tutti i possibili messaggi restituiti dalla query.

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

<h3 id="sdkassistantmessage">
  `SDKAssistantMessage`
</h3>

Messaggio di risposta dell'assistente.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Dall'SDK Anthropic
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
};
```

Il campo `message` è un [`BetaMessage`](https://platform.claude.com/docs/it/api/messages/create) dall'SDK Anthropic. Include campi come `id`, `content`, `model`, `stop_reason` e `usage`.

`SDKAssistantMessageError` è uno di: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, o `'unknown'`. `'model_not_found'` significa che il modello selezionato non esiste o non è disponibile per il tuo account o deployment. `'overloaded'` significa che l'API ha restituito un 529 perché il server è al massimo della capacità, a differenza di `'rate_limit'`, che è un 429 rispetto alla tua quota.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Messaggio di input dell'utente.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Dall'SDK Anthropic
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

Imposta `shouldQuery` a `false` per aggiungere il messaggio alla trascrizione senza attivare un turno dell'assistente. Il messaggio viene mantenuto e unito al prossimo messaggio utente che attiva un turno. Usa questo per iniettare contesto, come l'output di un comando che hai eseguito fuori banda, senza spendere una chiamata di modello su di esso.

Su un messaggio che contiene un blocco `tool_result`, `tool_use_result` è l'oggetto di output strutturato dello strumento piuttosto che il testo inviato al modello. La sua forma dipende dallo strumento denominato dal blocco `tool_use` corrispondente, quindi il campo è tipizzato `unknown`; le forme integrate sono elencate in [Tipi di output dello strumento](#tool-output-types).

Per lo strumento `Agent`, `tool_use_result` è [`AgentOutput`](#agent-2). Su un risultato `completed`, `content` contiene il rapporto del subagente senza l'ID agente e il trailer di utilizzo che Claude Code aggiunge al testo `tool_result`, quindi esegui il rendering da `tool_use_result` invece di analizzare quel testo.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

Messaggio utente riprodotto con UUID obbligatorio.

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

Un turno utente iniettato dall'esterno della sessione, uno il cui [`origin`](#sdkmessageorigin) è di tipo `peer` o `channel`, raggiunge il flusso come una riproduzione indipendentemente dal fatto che sia stato consegnato durante un turno attivo o abbia avviato un nuovo turno mentre la sessione era inattiva. Prima della v2.1.207, un turno iniettato consegnato mentre la sessione era inattiva non produceva alcun messaggio sul flusso e appariva solo quando rileggi la trascrizione.

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

Messaggio di risultato finale.

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      errors: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    };
```

Diversi campi sul risultato contengono dettagli diagnostici oltre a `subtype`:

* `api_error_status`: il codice di stato HTTP dell'errore API che ha terminato la conversazione. Assente o `null` quando il turno è terminato senza un errore API.
* `ttft_ms`: tempo al primo token in millisecondi, misurato quando arriva il primo messaggio dell'assistente completo. Presente solo sul ramo di successo.
* `ttft_stream_ms`: tempo in millisecondi fino al primo evento di flusso `message_start`, quando il flusso di risposta si apre. Inferiore a `ttft_ms`; il divario tra i due è il tempo impiegato per lo streaming del primo messaggio. Presente solo sul ramo di successo.
* `terminal_reason`: il motivo per cui il ciclo è terminato. Uno di `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, o `"turn_setup_failed"`.
* `fast_mode_state`: uno di `"on"`, `"off"`, o `"cooldown"`.

Il campo `origin` inoltro l'[`SDKMessageOrigin`](#sdkmessageorigin) del messaggio utente che ha attivato questo risultato. Quando un'attività in background finisce e l'SDK inietta un turno di follow-up sintetico, il `SDKResultMessage` risultante contiene `origin: { kind: "task-notification" }`. Controlla questo campo per distinguere i risultati che rispondono al tuo prompt dai risultati emessi per i follow-up di attività in background, in modo da poter instradare o sopprimere questi ultimi. Il campo è assente per i risultati emessi prima di qualsiasi turno utente, come gli errori di avvio.

Quando un hook `PreToolUse` restituisce `permissionDecision: "defer"`, il risultato ha `stop_reason: "tool_deferred"` e `deferred_tool_use` contiene l'`id`, il `name` e l'`input` del tool in sospeso. Leggi questo campo per visualizzare la richiesta nella tua interfaccia utente, quindi riprendi con lo stesso `session_id` per continuare. Vedi [Rinvia una chiamata di tool per dopo](/docs/it/hooks#defer-a-tool-call-for-later) per il percorso completo.

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

Messaggio di inizializzazione del sistema.

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

L'array `capabilities` nomina i comportamenti del protocollo che questa CLI implementa, in modo da poter rilevare le funzionalità invece di confrontare le stringhe `claude_code_version`. È un insieme aperto: ignora i valori che non riconosci e controlla la capacità specifica su cui fai affidamento. Il campo richiede Claude Code v2.1.205 o successivo ed è assente su CLI precedenti.

| Capacità               | Significato                                                                                                                                                                              |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) si risolve con una ricevuta [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) che nomina i messaggi in coda che sopravvivono all'interruzione |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Messaggio parziale di streaming (solo quando `includePartialMessages` è true). Il campo `parent_tool_use_id` è sempre `null`: gli eventi di flusso vengono emessi solo per la sessione principale. Per l'attribuzione del subagente, utilizza messaggi completi, che contengono `parent_tool_use_id`, o abilita [`forwardSubagentText`](#options) per ricevere il testo e il pensiero del subagente come messaggi completi.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Dall'SDK Anthropic
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Tempo al primo token in ms, presente solo negli eventi message_start
};
```

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

Messaggio che indica un limite di compattazione della conversazione.

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

<h3 id="sdkinformationalmessage">
  `SDKInformationalMessage`
</h3>

Banner di testo generico emesso dal ciclo. Contiene righe di stato non di errore, feedback di hook come il motivo del blocco di un hook `UserPromptSubmit`, e output di comando. Renderizza `content` come testo semplice al livello specificato.

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkworkershuttingdownmessage">
  `SDKWorkerShuttingDownMessage`
</h3>

Emesso durante lo spegnimento elegante del worker in modo che i client remoti possano mostrare il motivo per cui il worker se n'è andato invece di aspettare il timeout del battito cardiaco. Il `reason` è una stringa breve in snake\_case impostata dalla CLI host, come `"host_exit"` o `"remote_control_disabled"`. Agisci su questo solo quando stai eseguendo lo streaming in diretta. Una sessione ripresa riproduce le istanze passate di questo messaggio, quindi ignorale in quel caso.

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkplugininstallmessage">
  `SDKPluginInstallMessage`
</h3>

Evento di progresso dell'installazione del plugin. Emesso quando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/it/env-vars) è impostato, in modo che la tua applicazione Agent SDK possa tracciare l'installazione del plugin del marketplace prima del primo turno. Gli stati `started` e `completed` racchiudono l'installazione complessiva. Gli stati `installed` e `failed` segnalano i singoli marketplace e includono `name`.

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpermissiondeniedmessage">
  `SDKPermissionDeniedMessage`
</h3>

Evento di flusso emesso quando il sistema di autorizzazione nega automaticamente una chiamata di tool senza un prompt interattivo. Usalo per rendere il rifiuto nella tua interfaccia utente mentre accade, piuttosto che osservare solo il risultato del tool `is_error` che segue. Il percorso della richiesta interattiva raggiunge la tua applicazione separatamente tramite il callback [`canUseTool`](#canusetool). I rifiuti emessi da un hook `PreToolUse` non vengono segnalati tramite questo evento.

Questo evento richiede Claude Code v2.1.136 o successivo.

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| Campo                  | Tipo     | Descrizione                                                                                                                                                  |
| ---------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `tool_name`            | `string` | Nome del tool che è stato negato                                                                                                                             |
| `tool_use_id`          | `string` | ID del blocco `tool_use` a cui questo rifiuto risponde                                                                                                       |
| `agent_id`             | `string` | ID del subagente quando la chiamata negata ha avuto origine all'interno di un subagente. Rispecchia il campo su `can_use_tool` per l'instradamento lato host |
| `decision_reason_type` | `string` | Discriminatore per il componente che ha deciso, come `"rule"`, `"mode"`, `"classifier"`, o `"asyncAgent"`                                                    |
| `decision_reason`      | `string` | Motivo leggibile dall'uomo dal componente che ha deciso, quando disponibile                                                                                  |
| `message`              | `string` | Messaggio di rifiuto restituito al modello nel `tool_result`                                                                                                 |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Informazioni su un uso di tool negato.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Provenienza di un messaggio con ruolo utente. Questo appare come `origin` su [`SDKUserMessage`](#sdkusermessage) e viene inoltrato al corrispondente [`SDKResultMessage`](#sdkresultmessage) in modo da poter dire cosa ha attivato un determinato turno.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      name?: string;
      senderTaskId?: string;
      body?: string;
    }
  | { kind: "task-notification" }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" };
```

| `kind`              | Significato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Input diretto dall'utente finale. Sui messaggi utente, un `origin` assente significa anche input umano.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `channel`           | Messaggio in arrivo su un [canale](/docs/it/channels). `server` è il nome del server MCP di origine.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `peer`              | Messaggio da un altro agente. Per un [collega](/docs/it/agent-teams) in-process che invia a `main` tramite `SendMessage`, `from` è il nome del collega e `senderTaskId` è il suo ID attività. Per un peer tra sessioni come un altro processo Claude Code locale, `from` è l'indirizzo del mittente e `senderTaskId` è assente. }`name` e `body` richiedono Claude Code v2.1.205 o successivo. `name` è il nome visualizzato del mittente, normalizzato da Claude Code: rimuove i punti di codice di controllo, formato, surrogato e separatore di riga o paragrafo Unicode, quindi taglia il risultato e lo limita a 64 punti di codice con un'ellissi. `body` è il corpo del messaggio decodificato con l'involucro peer rimosso, byte-esatto con quello che il modello vede. Per un messaggio di collega `body` è sempre presente; per un peer tra sessioni è presente solo quando il turno è esattamente un involucro peer formato da Claude Code. Renderizza `name` e `body` invece di ri-analizzare il testo del messaggio. |
| `task-notification` | Turno sintetico iniettato dopo il completamento di un'attività in background. Vedi [`SDKTaskNotificationMessage`](#sdktasknotificationmessage).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `coordinator`       | Messaggio da un coordinatore di team in un [team di agenti](/docs/it/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `auto-continuation` | Turno sintetico iniettato quando la sessione continua senza input utente fresco, come un risultato di comando che attiva un prompt di follow-up.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

<h2 id="hook-types">
  Tipi di hook
</h2>

Per una guida completa sull'uso degli hook con esempi e pattern comuni, vedi la [guida Hooks](/docs/it/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Eventi hook disponibili.

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PermissionRequest"
  | "Setup"
  | "TeammateIdle"
  | "TaskCompleted"
  | "ConfigChange"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

Tipo di funzione callback hook.

```typescript theme={null}
type HookCallback = (
  input: HookInput, // Unione di tutti i tipi di input hook
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

Configurazione hook con matcher opzionale.

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // Timeout in secondi per tutti gli hook in questo matcher
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

Tipo di unione di tutti i tipi di input hook.

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCompletedHookInput
  | ConfigChangeHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Interfaccia base che tutti i tipi di input hook estendono.

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

Il campo `prompt_id` è un UUID che identifica il prompt dell'utente attualmente in elaborazione. Corrisponde all'[attributo `prompt.id` sugli eventi OpenTelemetry](/docs/it/monitoring-usage#event-correlation-attributes) ed è assente fino al primo input dell'utente. Richiede Claude Code v2.1.196 o successivo.

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
};
```

<h4 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h4>

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
};
```

<h4 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h4>

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

Si attiva una volta dopo che ogni chiamata di strumento in un batch è stata risolta, prima della prossima richiesta del modello. `tool_response` contiene il contenuto serializzato di `tool_result` che il modello vede; la forma differisce dall'oggetto strutturato `Output` di `PostToolUseHookInput`.

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

<h4 id="notificationhookinput">
  `NotificationHookInput`
</h4>

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

<h4 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h4>

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact";
  agent_type?: string;
  model?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // Stringa dall'array EXIT_REASONS
};
```

<h4 id="stophookinput">
  `StopHookInput`
</h4>

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

<h4 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h4>

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

<h4 id="subagentstophookinput">
  `SubagentStopHookInput`
</h4>

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

<h4 id="precompacthookinput">
  `PreCompactHookInput`
</h4>

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
};
```

<h4 id="setuphookinput">
  `SetupHookInput`
</h4>

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

<h4 id="teammateidlehookinput">
  `TeammateIdleHookInput`
</h4>

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated da v2.1.178. Contiene il nome del team derivato dalla sessione; verrà rimosso. */
  team_name: string;
};
```

<h4 id="taskcompletedhookinput">
  `TaskCompletedHookInput`
</h4>

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated da v2.1.178. Contiene il nome del team derivato dalla sessione; verrà rimosso. */
  team_name?: string;
};
```

<h4 id="configchangehookinput">
  `ConfigChangeHookInput`
</h4>

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

<h4 id="worktreecreatehookinput">
  `WorktreeCreateHookInput`
</h4>

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

<h4 id="worktreeremovehookinput">
  `WorktreeRemoveHookInput`
</h4>

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

<h4 id="messagedisplayhookinput">
  `MessageDisplayHookInput`
</h4>

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Valore di ritorno hook.

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated Usa `updatedToolOutput`, che funziona per tutti gli strumenti. */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      };
};
```

<h2 id="tool-input-types">
  Tipi di input dei tool
</h2>

Documentazione degli schemi di input per tutti i tool Claude Code incorporati. Questi tipi vengono esportati da `@anthropic-ai/claude-agent-sdk` e possono essere usati per le interazioni dei tool type-safe.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

Unione di tutti i tipi di input dei tool, esportati da `@anthropic-ai/claude-agent-sdk`.

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | TaskOutputInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ReadMcpResourceInput
  | SubscribeMcpResourceInput
  | SubscribePollingInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | UnsubscribeMcpResourceInput
  | UnsubscribePollingInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nome del tool:** `Agent` (precedentemente `Task`, che è ancora accettato come alias)

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan";
  isolation?: "worktree";
};
```

Avvia un nuovo agente per gestire compiti complessi e multi-step in modo autonomo.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nome del tool:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
};
```

Pone domande di chiarimento all'utente durante l'esecuzione. Vedi [Gestisci approvazioni e input dell'utente](/docs/it/agent-sdk/user-input#handle-clarifying-questions) per i dettagli di utilizzo.

<h3 id="bash">
  Bash
</h3>

**Nome del tool:** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

Esegue comandi bash in una sessione shell persistente con timeout opzionale ed esecuzione in background.

<h3 id="monitor">
  Monitor
</h3>

**Nome del tool:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
  description: string;
  timeout_ms?: number;
  persistent?: boolean;
};
```

Esegue una fonte di background e consegna ogni evento a Claude in modo che possa reagire senza polling: `command` esegue uno script e emette un evento per riga stdout, e `ws` apre un WebSocket e emette un evento per frame di testo. Fornisci esattamente uno tra `command` o `ws`. La fonte `ws` richiede Claude Code v2.1.195 o successivo.

Imposta `persistent: true` per i watch di lunghezza della sessione come code tail. Quando Monitor esegue un comando, segue le stesse regole di permesso di Bash; un watch WebSocket richiede l'approvazione separatamente. Vedi il [riferimento del tool Monitor](/docs/it/tools-reference#monitor-tool) per il comportamento e la disponibilità del provider.

<h3 id="taskoutput">
  TaskOutput
</h3>

**Nome del tool:** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

Recupera l'output da un'attività di background in esecuzione o completata.

<h3 id="edit">
  Edit
</h3>

**Nome del tool:** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

Esegue sostituzioni di stringhe esatte nei file.

<h3 id="read">
  Read
</h3>

**Nome del tool:** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

Legge i file dal filesystem locale, inclusi testo, immagini, PDF e notebook Jupyter. Usa `pages` per gli intervalli di pagine PDF (ad esempio, `"1-5"`).

<h3 id="write">
  Write
</h3>

**Nome del tool:** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

Scrive un file nel filesystem locale, sovrascrivendo se esiste.

<h3 id="glob">
  Glob
</h3>

**Nome del tool:** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

Corrispondenza di pattern di file veloce che funziona con qualsiasi dimensione di codebase.

<h3 id="grep">
  Grep
</h3>

**Nome del tool:** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

Potente tool di ricerca costruito su ripgrep con supporto regex.

<h3 id="taskstop">
  TaskStop
</h3>

**Nome del tool:** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Deprecato: usa task_id
};
```

Interrompe un'attività di background o shell in esecuzione per ID. A partire da v2.1.198, `task_id` accetta anche un compagno di squadra agent-team o un agente di background denominato per ID agente o nome.

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nome del tool:** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Modifica le celle nei file dei notebook Jupyter.

<h3 id="webfetch">
  WebFetch
</h3>

**Nome del tool:** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

Recupera il contenuto da un URL e lo elabora con un modello AI.

<h3 id="websearch">
  WebSearch
</h3>

**Nome del tool:** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

Cerca il web e restituisce risultati formattati.

<h3 id="workflow">
  Workflow
</h3>

**Nome del tool:** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown;
  resumeFromRunId?: string;
};
```

Esegue un [workflow dinamico](/docs/it/workflows): uno script che orchestra molti subagenti in background e restituisce un risultato consolidato. Il tool `Workflow` è disponibile in Agent SDK v0.3.149 e versioni successive. Almeno uno tra `script`, `name` o `scriptPath` è obbligatorio.

| Campo             | Tipo      | Descrizione                                                                                                                                                                                                                                                                                                   |
| ----------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Script di workflow inline. Deve iniziare con `export const meta = { name, description }` come letterale, seguito dal corpo dello script usando `agent()`, `parallel()`, `pipeline()` e `phase()`. Un array `phases` facoltativo in `meta` raggruppa gli agenti sotto fasi denominate nella vista di progresso |
| `name`            | `string`  | Nome di un workflow incorporato o uno salvato in `.claude/workflows/`. Risolto in uno script                                                                                                                                                                                                                  |
| `scriptPath`      | `string`  | Percorso a un file di script di workflow su disco. Ha la precedenza su `script` e `name`. Ogni invocazione persiste il suo script e restituisce il percorso nel risultato, quindi puoi modificare quel file e reinvocare con lo stesso `scriptPath` per iterare                                               |
| `args`            | `unknown` | Valore di input esposto allo script come `args` globale, per workflow denominati parametrizzati come una domanda di ricerca o un elenco di percorsi di file. Passa array e oggetti come valori JSON effettivi, non come stringa codificata in JSON                                                            |
| `resumeFromRunId` | `string`  | ID di esecuzione di una precedente invocazione di `Workflow` da riprendere. Le chiamate `agent()` completate con input invariati restituiscono risultati memorizzati nella cache; solo le chiamate modificate o nuove vengono eseguite live. Solo la stessa sessione                                          |

<h3 id="todowrite">
  TodoWrite
</h3>

**Nome del tool:** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Crea e gestisce un elenco di attività strutturato per il tracciamento del progresso.

<Note>
  A partire da TypeScript Agent SDK 0.3.142, `TodoWrite` è disabilitato per impostazione predefinita. Usa `TaskCreate`, `TaskGet`, `TaskUpdate` e `TaskList` invece. Vedi [Migra ai tool Task](/docs/it/agent-sdk/todo-tracking#migrate-to-task-tools) per aggiornare il tuo codice di monitoraggio, oppure imposta `CLAUDE_CODE_ENABLE_TASKS=0` per ripristinare `TodoWrite`.
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nome del tool:** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

Crea un singolo compito e restituisce il suo ID assegnato.

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nome del tool:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

Applica patch a un compito per ID. Imposta `status` a `"deleted"` per rimuoverlo.

<h3 id="taskget">
  TaskGet
</h3>

**Nome del tool:** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

Restituisce i dettagli completi per un compito, o `null` quando l'ID non viene trovato.

<h3 id="tasklist">
  TaskList
</h3>

**Nome del tool:** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

Restituisce uno snapshot di tutti i compiti nell'elenco corrente.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nome del tool:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Deprecato: non più utilizzato. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
};
```

Esce dalla modalità di pianificazione. Il campo `allowedPrompts` è deprecato e ignorato; Claude Code lo accetta comunque in modo che i chiamanti e i transcript esistenti siano validi. Prima di v2.1.205, richiedeva permessi Bash basati su prompt per implementare il piano.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nome del tool:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

Elenca le risorse MCP disponibili dai server connessi.

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Nome del tool:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

Legge una risorsa MCP specifica da un server.

<h3 id="enterworktree">
  EnterWorktree
</h3>

**Nome del tool:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

Crea e entra in un worktree git temporaneo per il lavoro isolato. Passa `path` per passare a un worktree esistente invece di crearne uno nuovo. Su primo ingresso il target deve essere un worktree registrato del repository corrente o, in uno spazio di lavoro multi-repo, di un repository annidato al suo interno; da una sessione worktree deve essere sotto `.claude/worktrees/` del repository della sessione. `name` e `path` si escludono a vicenda.

<h2 id="tool-output-types">
  Tipi di output dei tool
</h2>

Documentazione degli schemi di output per tutti i tool Claude Code incorporati. Questi tipi vengono esportati da `@anthropic-ai/claude-agent-sdk` e rappresentano i dati di risposta effettivi restituiti da ogni tool.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Unione di tutti i tipi di output dei tool.

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | AskUserQuestionOutput
  | BashOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | MonitorOutput
  | NotebookEditOutput
  | ReadMcpResourceOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

<h3 id="agent-2">
  Agent
</h3>

**Nome del tool:** `Agent` (precedentemente `Task`, che è ancora accettato come alias)

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

Restituisce il risultato dal subagente. Discriminato sul campo `status`: `"completed"` per le attività finite, `"async_launched"` per le attività di background, e `"remote_launched"` per le attività che Claude Code ha inviato a una sessione cloud remota, dove `sessionUrl` si collega a quella sessione e `taskId` l'identifica.

Il campo `resolvedModel` sulle varianti `completed` e `async_launched` nomina il modello su cui il subagente ha effettivamente eseguito, che può differire dal `model` input richiesto quando [`availableModels`](/docs/it/model-config#restrict-model-selection) o un altro override si applica. Questo campo richiede Claude Code v2.1.174 o successivo.

Sulla variante `completed`, `worktreePath` viene impostato quando il subagente è stato eseguito in un worktree git isolato, e `worktreeBranch` nomina il ramo di quel worktree quando Claude Code l'ha creato. `usage.service_tier` contiene la stringa del livello di servizio che l'API ha segnalato per le richieste del subagente.

Prima della v2.1.207, il tipo pubblicato era più ristretto. Ometteva `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount`, e i campi di utilizzo `inference_geo`, `speed` e `iterations`, e tipizzava `service_tier` come `"standard" | "priority" | "batch"`. I campi che il tipo contrassegna come opzionali possono essere assenti nei risultati registrati da versioni precedenti.

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**Nome del tool:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
};
```

Restituisce le domande poste e le risposte dell'utente. `response` viene impostato quando l'utente ha digitato una risposta in forma libera invece di rispondere alle domande strutturate; quando presente, Claude riceve "L'utente ha risposto: …" invece dell'elenco di risposte per domanda.

<h3 id="bash-2">
  Bash
</h3>

**Nome del tool:** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

Restituisce l'output del comando con stdout/stderr divisi. I comandi di background includono un `backgroundTaskId`.

<h3 id="monitor-2">
  Monitor
</h3>

**Nome del tool:** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

Restituisce l'ID dell'attività di background per il monitor in esecuzione. Usa questo ID con `TaskStop` per annullare il watch in anticipo.

<h3 id="edit-2">
  Edit
</h3>

**Nome del tool:** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Restituisce il diff strutturato dell'operazione di modifica.

<h3 id="read-2">
  Read
</h3>

**Nome del tool:** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
    };
```

Restituisce il contenuto del file in un formato appropriato al tipo di file. Discriminato sul campo `type`.

<h3 id="write-2">
  Write
</h3>

**Nome del tool:** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Restituisce il risultato della scrittura con informazioni sul diff strutturato.

<h3 id="glob-2">
  Glob
</h3>

**Nome del tool:** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
};
```

Restituisce i percorsi dei file che corrispondono al pattern glob, ordinati per tempo di modifica.

<h3 id="grep-2">
  Grep
</h3>

**Nome del tool:** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Restituisce i risultati della ricerca. La forma varia in base a `mode`: elenco di file, contenuto con corrispondenze o conteggi di corrispondenze.

<h3 id="taskstop-2">
  TaskStop
</h3>

**Nome del tool:** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

Restituisce la conferma dopo l'interruzione dell'attività di background.

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**Nome del tool:** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

Restituisce il risultato della modifica del notebook con i contenuti del file originale e aggiornato.

<h3 id="webfetch-2">
  WebFetch
</h3>

**Nome del tool:** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
};
```

Restituisce il contenuto recuperato con lo stato HTTP e i metadati.

<h3 id="websearch-2">
  WebSearch
</h3>

**Nome del tool:** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
};
```

Restituisce i risultati della ricerca dal web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nome del tool:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched";
  taskId: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  error?: string;
};
```

Restituisce immediatamente dopo che il tool accetta l'invocazione. Il risultato finale arriva successivamente come completamento di un'attività. Controlla `error` prima di trattare l'esecuzione come avviata: uno script che non supera il controllo della sintassi restituisce `status: "async_launched"` con `error` impostato e non viene mai eseguito.

| Campo           | Tipo               | Descrizione                                                                                                                                                    |
| --------------- | ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched"` | Il tool ha accettato l'invocazione. Questo è l'unico valore che il campo assume                                                                                |
| `taskId`        | `string`           | Identificatore dell'attività di background per l'esecuzione                                                                                                    |
| `runId`         | `string`           | Identificatore dell'esecuzione del workflow da passare come `resumeFromRunId` in una successiva invocazione                                                    |
| `summary`       | `string`           | Descrizione in una riga di ciò che fa il workflow                                                                                                              |
| `transcriptDir` | `string`           | Directory dove i transcript dei subagenti vengono scritti durante l'esecuzione                                                                                 |
| `scriptPath`    | `string`           | Percorso dello script del workflow persistente per questa esecuzione. Modificalo e passalo come `scriptPath` per rieseguire senza inviare nuovamente lo script |
| `error`         | `string`           | Impostato quando lo script non supera il controllo della sintassi. Quando presente, l'esecuzione non è stata avviata nonostante lo stato `async_launched`      |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**Nome del tool:** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Restituisce gli elenchi di attività precedenti e aggiornati.

<Note>
  A partire da TypeScript Agent SDK 0.3.142, `TodoWrite` è disabilitato per impostazione predefinita. Usa invece `TaskCreate`, `TaskGet`, `TaskUpdate` e `TaskList`. Vedi [Migrazione ai tool Task](/docs/it/agent-sdk/todo-tracking#migrate-to-task-tools) per aggiornare il tuo codice di monitoraggio, oppure imposta `CLAUDE_CODE_ENABLE_TASKS=0` per ripristinare `TodoWrite`.
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**Nome del tool:** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

Restituisce l'attività creata con il suo ID assegnato.

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**Nome del tool:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

Restituisce il risultato dell'aggiornamento, inclusi i campi che sono stati modificati.

<h3 id="taskget-2">
  TaskGet
</h3>

**Nome del tool:** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

Restituisce il record completo dell'attività, o `null` quando l'ID non viene trovato.

<h3 id="tasklist-2">
  TaskList
</h3>

**Nome del tool:** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

Restituisce uno snapshot di tutte le attività nell'elenco corrente.

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**Nome del tool:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Restituisce lo stato del piano dopo l'uscita dalla modalità di pianificazione.

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**Nome del tool:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

Restituisce un array di risorse MCP disponibili.

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**Nome del tool:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
  }>;
};
```

Restituisce i contenuti della risorsa MCP richiesta.

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**Nome del tool:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

Restituisce le informazioni sul worktree git.

<h2 id="permission-types">
  Tipi di permesso
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Operazioni per l'aggiornamento dei permessi.

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

<h3 id="permissionbehavior">
  `PermissionBehavior`
</h3>

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

<h3 id="permissionupdatedestination">
  `PermissionUpdateDestination`
</h3>

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // Impostazioni globali dell'utente
  | "projectSettings" // Impostazioni del progetto per directory
  | "localSettings" // Impostazioni locali del progetto
  | "session" // Solo sessione corrente
  | "cliArg"; // Argomento CLI
```

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

<h2 id="other-types">
  Altri tipi
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Funzioni beta disponibili che possono essere abilitate tramite l'opzione `betas`. Vedi [Intestazioni beta](https://platform.claude.com/docs/it/api/beta-headers) per ulteriori informazioni.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  La beta `context-1m-2025-08-07` è ritirata a partire dal 30 aprile 2026. Passare questo valore con Claude Sonnet 4.5 o Sonnet 4 non ha effetto, e le richieste che superano la finestra di contesto standard di 200k token restituiscono un errore. Per usare una finestra di contesto di 1M token, esegui la migrazione a [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, o Claude Opus 4.8](https://platform.claude.com/docs/it/about-claude/models/overview), che includono 1M di contesto ai prezzi standard senza intestazione beta richiesta.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informazioni su un comando slash disponibile.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
};
```

<h3 id="modelinfo">
  `ModelInfo`
</h3>

Informazioni su un modello disponibile.

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| Campo                      | Tipo                                                               | Descrizione                                                                                                                                                                                                                                                                                                                       |
| :------------------------- | :----------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | Identificatore del modello da passare nelle chiamate API                                                                                                                                                                                                                                                                          |
| `resolvedModel`            | `string \| undefined`                                              | ID del modello wire canonico a cui il `value` di questa voce si risolve. Una voce di alias come `sonnet` si risolve a un ID di modello esplicito come `claude-sonnet-5`, quindi un host può abbinare un ID di modello esplicito memorizzato rispetto alla voce di alias che lo copre. Richiede Claude Code v2.1.197 o successivo. |
| `displayName`              | `string`                                                           | Nome di visualizzazione leggibile dall'uomo                                                                                                                                                                                                                                                                                       |
| `description`              | `string`                                                           | Descrizione delle capacità del modello                                                                                                                                                                                                                                                                                            |
| `supportsEffort`           | `boolean \| undefined`                                             | Se questo modello supporta i livelli di sforzo                                                                                                                                                                                                                                                                                    |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | Livelli di sforzo che questo modello accetta                                                                                                                                                                                                                                                                                      |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | Se questo modello supporta il pensiero adattivo, dove Claude decide quando e quanto pensare                                                                                                                                                                                                                                       |
| `supportsFastMode`         | `boolean \| undefined`                                             | Se questo modello supporta la modalità veloce                                                                                                                                                                                                                                                                                     |
| `supportsAutoMode`         | `boolean \| undefined`                                             | Se questo modello supporta la modalità auto                                                                                                                                                                                                                                                                                       |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Informazioni su un subagente disponibile che può essere invocato tramite il tool Agent.

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| Campo         | Tipo                  | Descrizione                                                                         |
| :------------ | :-------------------- | :---------------------------------------------------------------------------------- |
| `name`        | `string`              | Identificatore del tipo di agente (ad esempio, `"Explore"`, `"general-purpose"`)    |
| `description` | `string`              | Descrizione di quando usare questo agente                                           |
| `model`       | `string \| undefined` | Alias del modello che questo agente usa. Se omesso, eredita il modello del genitore |

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Stato di un server MCP connesso.

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
  }[];
};
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

La configurazione di un server MCP come segnalato da `mcpServerStatus()`. Questa è l'unione di tutti i tipi di trasporto del server MCP.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

Vedi [`McpServerConfig`](#mcpserverconfig) per i dettagli su ogni tipo di trasporto.

<h3 id="accountinfo">
  `AccountInfo`
</h3>

Informazioni sull'account per l'utente autenticato.

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

<h3 id="modelusage">
  `ModelUsage`
</h3>

Statistiche di utilizzo per modello restituite nei messaggi di risultato. Il valore `costUSD` è una stima lato client. Vedi [Traccia costo e utilizzo](/docs/it/agent-sdk/cost-tracking) per le avvertenze di fatturazione.

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
};
```

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

Una versione di [`Usage`](#usage) con tutti i campi nullable resi non-nullable.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

Statistiche di utilizzo dei token. Questo è il tipo `BetaUsage` da `@anthropic-ai/sdk`.

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
};
```

`BetaServerToolUsage` e `BetaIterationsUsage` sono definiti in `@anthropic-ai/sdk`.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

Tipo di risultato del tool MCP (da `@modelcontextprotocol/sdk/types.js`). `structuredContent` è un oggetto JSON che può essere restituito insieme a `content`, inclusi blocchi di immagini. Vedi [Restituisci dati strutturati](/docs/it/agent-sdk/custom-tools#return-structured-data).

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // I campi aggiuntivi variano in base al tipo
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Controlla il comportamento di pensiero/ragionamento di Claude. Ha precedenza sul deprecato `maxThinkingTokens`.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // Il modello determina quando e quanto ragionare (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Budget di token di pensiero fisso
  | { type: "disabled" }; // Nessun pensiero esteso
```

Il campo opzionale `display` controlla se il testo di pensiero viene restituito `"summarized"` o `"omitted"`. Su Claude Opus 4.7 e versioni successive, l'impostazione predefinita dell'API è `"omitted"`, quindi imposta `"summarized"` per ricevere il contenuto di pensiero nei blocchi `thinking`.

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

Interfaccia per la generazione di processi personalizzati (usata con l'opzione `spawnClaudeCodeProcess`). `ChildProcess` soddisfa già questa interfaccia.

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

<h3 id="spawnoptions">
  `SpawnOptions`
</h3>

Opzioni passate alla funzione di generazione personalizzata.

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

<Note>
  Il campo `signal` comunica alla tua funzione di generazione quando smontare il processo. Passalo come opzione `signal` al `spawn()` di Node, oppure passalo al tuo gestore di smontaggio della VM o del contenitore.

  Questo segnale non si attiva nell'istante in cui [`Options.abortController`](#options) si interrompe. L'SDK prima chiude lo stdin del processo e attende circa due secondi affinché la CLI si arresti correttamente, quindi interrompe questo segnale. Per reagire nel momento in cui il chiamante si interrompe, ascolta il tuo `Options.abortController.signal`, che la tua funzione di generazione può referenziare dal suo ambito di chiusura.
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

Risultato di un'operazione `setMcpServers()`.

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

Risultato di un'operazione `rewindFiles()`.

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
};
```

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

Messaggio di aggiornamento dello stato (ad esempio, compattazione).

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktasknotificationmessage">
  `SDKTaskNotificationMessage`
</h3>

Notifica quando un'attività di background si completa, fallisce o viene interrotta. Le attività di background includono i comandi Bash `run_in_background`, i watch [Monitor](#monitor) e i subagenti di background.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

Riepilogo dell'uso dei tool in una conversazione.

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookstartedmessage">
  `SDKHookStartedMessage`
</h3>

Emesso quando un hook inizia l'esecuzione.

Claude Code fornisce questo messaggio, [`SDKHookProgressMessage`](#sdkhookprogressmessage), e [`SDKHookResponseMessage`](#sdkhookresponsemessage) al flusso di messaggi immediatamente, incluso mentre un hook `SessionStart` o `Setup` è ancora in esecuzione durante l'avvio della sessione. Claude Code v2.1.169 attraverso v2.1.203 ha fornito questi messaggi in un batch dopo che un hook `SessionStart` o `Setup` era completato; v2.1.204 ha ripristinato la consegna dal vivo.

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookprogressmessage">
  `SDKHookProgressMessage`
</h3>

Emesso mentre un hook è in esecuzione, con output stdout/stderr.

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookresponsemessage">
  `SDKHookResponseMessage`
</h3>

Emesso quando un hook finisce l'esecuzione.

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolprogressmessage">
  `SDKToolProgressMessage`
</h3>

Emesso periodicamente mentre un tool è in esecuzione per indicare il progresso.

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

Emesso durante i flussi di autenticazione.

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskstartedmessage">
  `SDKTaskStartedMessage`
</h3>

Emesso quando un'attività di background inizia. Il campo `task_type` è `"local_bash"` per i comandi Bash di background e i watch [Monitor](#monitor), `"local_agent"` per i subagenti, o `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

Emesso periodicamente mentre un subagente o un'attività di background è in esecuzione. Il campo `summary` è popolato solo quando [`agentProgressSummaries`](#options) è abilitato.

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskupdatedmessage">
  `SDKTaskUpdatedMessage`
</h3>

Emesso quando lo stato di un'attività di background cambia, ad esempio quando passa da `running` a `completed`. Unisci `patch` nella tua mappa attività locale con chiave `task_id`. Il campo `end_time` è un timestamp Unix epoch in millisecondi, confrontabile con `Date.now()`.

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkbackgroundtaskschangedmessage">
  `SDKBackgroundTasksChangedMessage`
</h3>

Emesso ogni volta che l'insieme delle attività di background attive cambia: un'attività inizia, si completa, viene terminata, o un agente in primo piano viene messo in background. L'array `tasks` è l'insieme completo attivo. Sostituisci qualsiasi insieme memorizzato nella cache con ogni payload invece di abbinare gli eventi `task_started` e `task_notification`, in modo che il prossimo cambio di appartenenza corregga qualsiasi evento che hai perso.

L'ordine relativo a quegli eventi per attività è non specificato, quindi non correlare i due flussi.

Nulla viene emesso all'avvio. Reimposta a un insieme vuoto ogni volta che il processo CLI della sessione inizia o si riavvia e lascia che il prossimo cambio di appartenenza lo ripopoli.

Richiede Claude Code v2.1.203 o successivo.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Emesso mentre Claude sta producendo un blocco di pensiero, incluso uno redatto, con una stima in esecuzione dei token di pensiero generati finora. `estimated_tokens` è il totale in esecuzione per il blocco di pensiero corrente e `estimated_tokens_delta` è l'incremento portato da questo frame. Usalo per la visualizzazione del progresso. Il conteggio finale per il ciclo dell'agente di primo livello è il `usage.output_tokens` del messaggio di risultato, che [non include i token dei subagenti](/docs/it/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); usa [`modelUsage`](#modelusage) per la contabilità dell'intero albero.

Richiede Claude Code v2.1.153 o successivo.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

Emesso quando i checkpoint dei file vengono persistiti su disco.

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkratelimitevent">
  `SDKRateLimitEvent`
</h3>

Emesso quando la sessione incontra un limite di velocità.

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

Quando `errorCode` è `"credits_required"`, il rifiuto proviene da un abbonamento claude.ai il cui utilizzo incluso è esaurito, e la sessione non può continuare fino a quando l'utente non acquista crediti di utilizzo. `canUserPurchaseCredits` indica se l'utente autenticato può acquistare crediti per l'account, e `hasChargeableSavedPaymentMethod` indica se un metodo di pagamento salvato è registrato. Tutti e tre i campi sono assenti negli eventi di limite di velocità che non sono rifiuti con crediti richiesti. Richiede Claude Code v2.1.181 o successivo.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Output da un comando slash locale (ad esempio, `/voice` o `/usage`). Visualizzato come testo in stile assistente nella trascrizione.

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkcommandschangedmessage">
  `SDKCommandsChangedMessage`
</h3>

Emesso quando l'insieme dei comandi disponibili cambia durante la sessione, ad esempio quando le skill vengono scoperte mentre l'agente entra in una sottodirectory. L'array `commands` è l'elenco completo aggiornato, quindi sostituisci qualsiasi elenco di comandi memorizzato nella cache con questo payload. Chiamare di nuovo `supportedCommands()` non è equivalente: quel metodo restituisce lo snapshot acquisito all'inizializzazione e non riflette i cambiamenti durante la sessione.

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpromptsuggestionmessage">
  `SDKPromptSuggestionMessage`
</h3>

Emesso dopo ogni turno quando `promptSuggestions` è abilitato. Contiene un prompt utente successivo previsto.

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkconversationresetmessage">
  `SDKConversationResetMessage`
</h3>

Emesso quando la conversazione della sessione viene sostituita senza terminare la sessione, ad esempio dopo `/clear`, all'uscita dalla modalità piano, o quando inizia una conversazione nuova. Monta una trascrizione vuota sotto `new_conversation_id` e scarta qualsiasi titolo di sessione memorizzato nella cache.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

I tipi pubblicati dall'SDK dichiarano `SDKConversationResetMessage` in Claude Code v2.1.203 e successivo. Prima di v2.1.203, `SDKMessage` faceva riferimento al tipo senza dichiararlo, quindi il restringimento su `type === "conversation_reset"` non riusciva a typecheck quando `skipLibCheck` era disabilitato.

<h3 id="aborterror">
  `AbortError`
</h3>

Classe di errore personalizzata per le operazioni di interruzione.

```typescript theme={null}
class AbortError extends Error {}
```

<h2 id="sandbox-configuration">
  Configurazione della sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configurazione per il comportamento della sandbox. Usa questo per abilitare il sandboxing dei comandi e configurare le restrizioni di rete a livello di programmazione.

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| Proprietà                   | Tipo                                                  | Predefinito | Descrizione                                                                                                                                                                                                                                                        |
| :-------------------------- | :---------------------------------------------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | Abilita la modalità sandbox per l'esecuzione dei comandi                                                                                                                                                                                                           |
| `failIfUnavailable`         | `boolean`                                             | `true`      | Arresta all'avvio se `enabled` è `true` ma la sandbox non può avviarsi. Imposta `false` per ricadere nell'esecuzione senza sandbox con un avviso su stderr                                                                                                         |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Auto-approva i comandi bash quando la sandbox è abilitata                                                                                                                                                                                                          |
| `excludedCommands`          | `string[]`                                            | `[]`        | Comandi che sempre bypassano le restrizioni della sandbox (ad esempio, `['docker']`). Questi vengono eseguiti senza sandbox automaticamente senza coinvolgimento del modello                                                                                       |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Consenti al modello di richiedere l'esecuzione di comandi al di fuori della sandbox. Quando `true`, il modello può impostare `dangerouslyDisableSandbox` nell'input del tool, che ricade nel [sistema di permessi](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Configurazione della sandbox specifica della rete                                                                                                                                                                                                                  |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Configurazione della sandbox specifica del filesystem per le restrizioni di lettura/scrittura                                                                                                                                                                      |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Mappa delle categorie di violazione ai pattern da ignorare (ad esempio, `{ file: ['/tmp/*'], network: ['localhost'] }`)                                                                                                                                            |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Abilita una sandbox nidificata più debole per la compatibilità                                                                                                                                                                                                     |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Configurazione del binario ripgrep personalizzato per gli ambienti sandbox                                                                                                                                                                                         |

<Note>
  La sandbox dipende dal supporto della piattaforma e, su Linux, da strumenti come `bubblewrap` e `socat`. Quando `enabled` è `true` e la sandbox non può avviarsi, `query()` segnala un messaggio `result` con `subtype: "error_during_execution"` e il motivo in `errors`. Per una singola chiamata `query()`, l'SDK genera un'eccezione dopo aver ceduto quel risultato di errore, quindi racchiudi il ciclo in un blocco try per continuare oltre. Vedi [Gestire il risultato](/docs/it/agent-sdk/agent-loop#handle-the-result) per il contratto di errore.

  Per eseguire senza sandbox, imposta `failIfUnavailable: false`.
</Note>

<h4 id="example-usage">
  Esempio di utilizzo
</h4>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // such as when the sandbox can't start (failIfUnavailable defaults to true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Sicurezza del socket Unix:** L'opzione `allowUnixSockets` può concedere l'accesso a potenti servizi di sistema. Ad esempio, consentire `/var/run/docker.sock` concede effettivamente l'accesso completo al sistema host tramite l'API Docker, bypassando l'isolamento della sandbox. Consenti solo i socket Unix strettamente necessari e comprendi le implicazioni di sicurezza di ciascuno.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configurazione specifica della rete per la modalità sandbox. Queste impostazioni si applicano ai comandi Bash in sandbox quando `enabled` è `true` nella [`SandboxSettings`](#sandboxsettings) padre. Non limitano lo strumento WebFetch, che utilizza invece [regole di permesso](/docs/it/permissions#webfetch).

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Proprietà                 | Tipo       | Predefinito | Descrizione                                                                                                                                                                                                                                                                                                             |
| :------------------------ | :--------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Nomi di dominio a cui i processi in sandbox possono accedere                                                                                                                                                                                                                                                            |
| `deniedDomains`           | `string[]` | `[]`        | Nomi di dominio a cui i processi in sandbox non possono accedere. Ha la precedenza su `allowedDomains`                                                                                                                                                                                                                  |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Solo impostazioni gestite. Quando impostato nelle [impostazioni gestite](/docs/it/permissions#managed-settings), solo le voci `allowedDomains` dalle impostazioni gestite vengono rispettate e le voci dalle impostazioni utente, progetto o locali vengono ignorate. Non ha effetto quando impostato tramite le opzioni SDK |
| `allowLocalBinding`       | `boolean`  | `false`     | Consenti ai processi di associarsi alle porte locali (ad esempio, per i server di sviluppo)                                                                                                                                                                                                                             |
| `allowUnixSockets`        | `string[]` | `[]`        | Percorsi dei socket Unix a cui i processi possono accedere (ad esempio, socket Docker)                                                                                                                                                                                                                                  |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Consenti l'accesso a tutti i socket Unix                                                                                                                                                                                                                                                                                |
| `httpProxyPort`           | `number`   | `undefined` | Porta del proxy HTTP per le richieste di rete                                                                                                                                                                                                                                                                           |
| `socksProxyPort`          | `number`   | `undefined` | Porta del proxy SOCKS per le richieste di rete                                                                                                                                                                                                                                                                          |

<Note>
  Il proxy sandbox integrato applica `allowedDomains` in base al nome host richiesto e non termina o ispeziona il traffico TLS, quindi tecniche come il [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) possono potenzialmente bypassarlo. Vedi [Limitazioni di sicurezza del sandboxing](/docs/it/sandboxing#security-limitations) per i dettagli e [Distribuzione sicura](/docs/it/agent-sdk/secure-deployment#traffic-forwarding) per configurare un proxy che termina TLS.
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

Configurazione specifica del filesystem per la modalità sandbox.

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| Proprietà    | Tipo       | Predefinito | Descrizione                                                      |
| :----------- | :--------- | :---------- | :--------------------------------------------------------------- |
| `allowWrite` | `string[]` | `[]`        | Pattern di percorso file per consentire l'accesso in scrittura a |
| `denyWrite`  | `string[]` | `[]`        | Pattern di percorso file per negare l'accesso in scrittura a     |
| `denyRead`   | `string[]` | `[]`        | Pattern di percorso file per negare l'accesso in lettura a       |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback dei permessi per i comandi senza sandbox
</h3>

Quando `allowUnsandboxedCommands` è abilitato, il modello può richiedere di eseguire comandi al di fuori della sandbox impostando `dangerouslyDisableSandbox: true` nell'input del tool. Queste richieste ricadono nel sistema di permessi esistente, il che significa che il tuo handler `canUseTool` viene invocato, permettendoti di implementare la logica di autorizzazione personalizzata. Nell'esempio seguente, `isCommandAuthorized` rappresenta un controllo di autorizzazione che definisci.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Un elenco statico di comandi che sempre bypassano la sandbox automaticamente (ad esempio, `['docker']`). Il modello non ha controllo su questo.
  * `allowUnsandboxedCommands`: Consenti al modello di decidere in fase di esecuzione se richiedere l'esecuzione senza sandbox impostando `dangerouslyDisableSandbox: true` nell'input del tool.
</Note>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Il modello può richiedere l'esecuzione senza sandbox
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Controlla se il modello sta richiedendo di bypassare la sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // Il modello sta richiedendo di eseguire questo comando al di fuori della sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

Questo pattern ti consente di:

* **Controllare le richieste del modello:** Registra quando il modello richiede l'esecuzione senza sandbox
* **Implementare allowlist:** Consenti solo comandi specifici di essere eseguiti senza sandbox
* **Aggiungere flussi di lavoro di approvazione:** Richiedi l'autorizzazione esplicita per le operazioni privilegiate

<Warning>
  I comandi in esecuzione con `dangerouslyDisableSandbox: true` hanno accesso completo al sistema. Assicurati che il tuo handler `canUseTool` convalidi queste richieste attentamente.

  Se `permissionMode` è impostato su `bypassPermissions` e `allowUnsandboxedCommands` è abilitato, il modello può autonomamente eseguire comandi al di fuori della sandbox senza alcun prompt di approvazione (una [regola `ask`](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) esplicita ne forza comunque una). Questa combinazione consente effettivamente al modello di sfuggire all'isolamento della sandbox silenziosamente.
</Warning>

<h2 id="see-also">
  Vedi anche
</h2>

* [Panoramica dell'SDK](/docs/it/agent-sdk/overview) - Concetti generali dell'SDK
* [Riferimento Python SDK](/docs/it/agent-sdk/python) - Documentazione dell'SDK Python
* [Riferimento CLI](/docs/it/cli-reference) - Interfaccia della riga di comando
* [Flussi di lavoro comuni](/docs/it/common-workflows) - Guide passo dopo passo
