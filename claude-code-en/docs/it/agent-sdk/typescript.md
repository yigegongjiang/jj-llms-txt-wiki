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
  L'SDK raggruppa un binario nativo Claude Code per la tua piattaforma come dipendenza opzionale come `@anthropic-ai/claude-agent-sdk-darwin-arm64`. La maggior parte delle installazioni non necessita di un'installazione separata di Claude Code. La versione dell'SDK traccia la versione del Claude Code raggruppato. SDK v0.3.191 raggruppa Claude Code v2.1.191, quindi una funzione su questa pagina che richiede una versione di Claude Code necessita della versione SDK con lo stesso numero di patch o successivo. Se il tuo gestore di pacchetti salta le dipendenze opzionali, l'SDK genera `Native CLI binary for <platform>-<arch> not found`; imposta [`pathToClaudeCodeExecutable`](#options) su un binario `claude` installato separatamente.

  Se il tuo gestore di pacchetti non applica il campo `libc` di npm, come non fa Yarn 1.x, ottieni sia i pacchetti della piattaforma glibc che musl su Linux, raddoppiando approssimativamente la dimensione dell'installazione. Su Agent SDK v0.2.141 o successivo, l'SDK avvia comunque la variante corretta. Per recuperare lo spazio in un'immagine contenitore, elimina il pacchetto della piattaforma che non corrisponde al libc dove viene eseguita la tua app; per un runtime glibc su x64, è `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`. Su una macchina di sviluppo l'eliminazione è temporanea, poiché Yarn reinstalla il pacchetto al prossimo cambio di dipendenza.
</Note>

<h3 id="compile-to-a-single-executable">
  Compilare in un singolo eseguibile
</h3>

Quando compili la tua applicazione in un eseguibile a file singolo con `bun build --compile`, l'SDK non può risolvere il binario CLI raggruppato in fase di esecuzione. `require.resolve` non funziona all'interno del filesystem virtuale `$bunfs` dell'eseguibile compilato, quindi l'SDK genera `Native CLI binary for <platform>-<arch> not found`.

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
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Parametri
</h4>

| Parametro     | Tipo                                                                                                   | Descrizione                                                                                                                                                                                                                                                                                                                                    |
| :------------ | :----------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                                                               | Il nome del tool                                                                                                                                                                                                                                                                                                                               |
| `description` | `string`                                                                                               | Una descrizione di cosa fa il tool                                                                                                                                                                                                                                                                                                             |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | Schema Zod che definisce i parametri di input del tool (supporta sia Zod 3 che Zod 4)                                                                                                                                                                                                                                                          |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | Funzione asincrona che esegue la logica del tool                                                                                                                                                                                                                                                                                               |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | Extras opzionali. `annotations` fornisce suggerimenti comportamentali MCP ai client. `searchHint` è una frase di capacità su una riga mostrata nell'elenco dei tool differiti quando [tool search](/docs/it/agent-sdk/tool-search) è attivo. `alwaysLoad: true` mantiene lo schema completo di questo tool nel prompt iniziale invece di differirlo |

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
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Parametri
</h4>

| Parametro              | Tipo                          | Descrizione                                                                                                                                                                                                                                                                          |
| :--------------------- | :---------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.name`         | `string`                      | Il nome del server MCP                                                                                                                                                                                                                                                               |
| `options.version`      | `string`                      | Stringa di versione opzionale                                                                                                                                                                                                                                                        |
| `options.instructions` | `string`                      | Istruzioni del server opzionali, restituite da `initialize` e presentate al modello come un blocco di istruzioni MCP                                                                                                                                                                 |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | Array di definizioni di tool create con [`tool()`](#tool)                                                                                                                                                                                                                            |
| `options.alwaysLoad`   | `boolean`                     | Quando `true`, ogni tool da questo server rimane nel prompt iniziale e non viene mai differito dietro [tool search](/docs/it/agent-sdk/tool-search). Si combina con `alwaysLoad` per tool in [`tool()`](#tool)                                                                            |
| `options.timeout`      | `number`                      | Timeout in millisecondi per le chiamate ai tool di questo server. Claude Code lo applica a questo server al posto di [`MCP_TOOL_TIMEOUT`](/docs/it/env-vars). Passa un numero intero di almeno 1000. Claude Code ignora altri valori. Richiede TypeScript Agent SDK v0.3.248 o successivo |

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

| Proprietà            | Tipo                    | Descrizione                                                                                                                                                                                                                                                                                                   |
| :------------------- | :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`               | `"user" \| "assistant"` | Ruolo del messaggio                                                                                                                                                                                                                                                                                           |
| `uuid`               | `string`                | Identificatore di messaggio univoco                                                                                                                                                                                                                                                                           |
| `session_id`         | `string`                | Sessione a cui appartiene questo messaggio                                                                                                                                                                                                                                                                    |
| `message`            | `unknown`               | Payload del messaggio grezzo dalla trascrizione                                                                                                                                                                                                                                                               |
| `parent_tool_use_id` | `string \| null`        | Per i messaggi dei subagent, l'`tool_use_id` della chiamata del tool `Agent` o `Skill` che lo ha generato. `null` per i messaggi della sessione principale e le sessioni precedenti                                                                                                                           |
| `parent_agent_id`    | `string \| null`        | Per i messaggi da un [subagent annidato](/docs/it/sub-agents#let-subagents-spawn-their-own-subagents), l'`agentId` del subagent che lo ha generato. `null` per i messaggi della sessione principale, i messaggi dai subagent di primo livello e le sessioni precedenti. Richiede Claude Code v2.1.202 o successivo |

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
  Questa funzione è in fase alpha e la sua API potrebbe cambiare prima della stabilizzazione.
</Note>

Lo snapshot differisce da quello che una sessione `query()` live applica:

* **`policyHelper`**: `resolveSettings()` legge le fonti MDM, inclusi plist macOS e Windows HKLM/HKCU, ma non esegue il subprocess `policyHelper` configurato dall'amministratore.
* **Impostazioni gestite dal server**: `resolveSettings()` non recupera [impostazioni gestite dal server](/docs/it/server-managed-settings#fetch-and-caching-behavior). Passale come `options.serverManagedSettings` per includerle.
* **`defaultMode`**: lo snapshot restituisce `permissions.defaultMode` così com'è da ogni livello, quindi può includere i valori `'auto'` e `'bypassPermissions'` dalle impostazioni di progetto e locali, che [una sessione live ignora](/docs/it/permission-modes#which-mode-a-session-starts-in).

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parametri
</h4>

`resolveSettings()` accetta un singolo oggetto di opzioni. Tutti i campi sono opzionali.

| Parametro                       | Tipo                                  | Predefinito     | Descrizione                                                                                                                                                                                                                                                                                                                       |
| :------------------------------ | :------------------------------------ | :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | Directory per risolvere le impostazioni di progetto e locali relative a                                                                                                                                                                                                                                                           |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Tutte le fonti  | Quali fonti del filesystem caricare. Passa `[]` per saltare le impostazioni utente, progetto e locali. La [politica gestita dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) si carica in tutti i casi. `resolveSettings()` include le impostazioni gestite dal server solo quando passi `options.serverManagedSettings`  |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Impostazioni della politica fornite dall'host di incorporamento. Segue le stesse regole di [`managedSettings` in `Options`](#options), tranne che `resolveSettings()` non esegue un [`policyHelper`](/docs/it/settings-reference#policyhelper) configurato, quindi lo snapshot può includere impostazioni che una sessione live scarta |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Payload delle impostazioni gestite dal server da `/api/claude_code/settings`. Le chiavi non restrittive passano attraverso senza filtri                                                                                                                                                                                           |

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

L'esempio seguente risolve le impostazioni per una directory di progetto e stampa la fonte che controlla il periodo di pulizia. Su una macchina dove nessun file di impostazioni imposta `cleanupPeriodDays`, entrambe le righe stampate mostrano `undefined` per il valore, che è l'output previsto piuttosto che un errore.

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

| Proprietà                         | Tipo                                                                                                                                                                                                           | Predefinito                                     | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`                         | Controller per annullare le operazioni                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                                            | Directory aggiuntive a cui Claude può accedere. L'SDK passa ogni voce a Claude Code come `--add-dir`, quindi con l'impostazione `project` source Claude Code [carica anche le skill, i comandi e i subagent della directory](/docs/it/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                                     | Nome dell'agente per il thread principale. L'agente deve essere definito nell'opzione `agents` o nelle impostazioni                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                                     | Definire i subagent a livello di programmazione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                                         | Quando `true`, genera riepiloghi di progresso su una riga per i subagent e li inoltra su eventi [`task_progress`](#sdktaskprogressmessage) tramite il campo `summary`. Si applica ai subagent in primo piano e in background                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                                         | Abilita il bypass dei permessi. Richiesto quando si utilizza `permissionMode: 'bypassPermissions'`, all'avvio o successivamente tramite `setPermissionMode()`. Vedere [plan mode](/docs/it/agent-sdk/permissions#plan-mode-plan) per come interagisce con `permissionMode: 'plan'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                                            | Strumenti da approvare automaticamente senza richiedere conferma. Questo non limita Claude solo a questi strumenti. Se nomini uno dei [task-tracking tools](/docs/it/agent-sdk/todo-tracking#model-availability) qui, Claude Code opta anche la sessione. Gli altri strumenti non elencati ricadono in `permissionMode` e `canUseTool`. Usa `disallowedTools` per bloccare gli strumenti. Vedi [Permissions](/docs/it/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                                            | Abilita le funzioni beta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                                     | Funzione di permesso personalizzata, invocata solo quando il [flusso di permesso](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) ricade in un prompt. Non invocata per le chiamate pre-approvate da `allowedTools`, regole di autorizzazione o `permissionMode`. Una regola di autorizzazione non pre-approva le [azioni che nessuna modalità auto-approva](/docs/it/permission-modes#actions-no-mode-auto-approves). Vedi [`CanUseTool`](#canusetool) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                                         | Continua la conversazione più recente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                                 | Directory di lavoro corrente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                                         | Abilita la modalità debug per il processo Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                                     | Scrivi i log di debug in un percorso file specifico. Abilita implicitamente la modalità debug                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                                            | Strumenti da negare. Un nome semplice come `"Bash"` rimuove lo strumento dal contesto di Claude. Una regola con ambito come `"Bash(rm *)"` lascia lo strumento disponibile e nega le chiamate corrispondenti in ogni modalità di permesso, incluso `bypassPermissions`, per il comando [come scritto](/docs/it/permissions#bash-rule-limits). Vedi [Permissions](/docs/it/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                                     | Controlla quanto sforzo Claude dedica alla sua risposta. Funziona con il pensiero adattivo per guidare la profondità del pensiero. Vedi [regola il livello di sforzo](/docs/it/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                                         | Abilita il tracciamento dei cambiamenti di file per il rewind. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                                   | Variabili di ambiente. Quando impostato, questo sostituisce l'ambiente del subprocess invece di unirsi a `process.env`, quindi passa `{ ...process.env, YOUR_VAR: 'value' }` per mantenere le variabili ereditate come `PATH`. Vedi [Gestire risposte API lente o bloccate](#handle-slow-or-stalled-api-responses) per un esempio di questo modello, e [Variabili di ambiente](/docs/it/env-vars) per le variabili che la CLI sottostante legge. Imposta `CLAUDE_AGENT_SDK_CLIENT_APP` per identificare la tua app nell'intestazione User-Agent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | Rilevato automaticamente                        | Runtime JavaScript da utilizzare                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                                            | Argomenti da passare all'eseguibile                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                                            | Argomenti aggiuntivi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                                     | Modello da utilizzare se il modello primario fallisce. Accetta un elenco separato da virgole. Per l'ordine e il limite, vedi [Fallback model chains](/docs/it/model-config#fallback-model-chains). Per indicazioni, vedi [Scegli un modello](/docs/it/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                                         | Quando si riprende con `resume`, esegui il fork a un nuovo ID di sessione invece di continuare la sessione originale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                                         | Inoltra i blocchi di testo e pensiero del subagent come messaggi di assistente e utente con `parent_tool_use_id` impostato, in modo che i consumer possano rendere una trascrizione nidificata. Senza questa opzione, Claude Code emette blocchi `tool_use` e `tool_result` del subagent ma non testo o pensiero. I messaggi dai subagent a ogni profondità di nidificazione vengono inoltrati su Claude Code v2.1.219 e successivi; prima di v2.1.219, solo i messaggi dai subagent di profondità-1 apparivano. I messaggi dei subagent che una skill con fork genera, e delle skill con fork nidificate, richiedono v2.1.275 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                                            | Callback hook per gli eventi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                                         | Includi gli eventi del ciclo di vita hook nel flusso di messaggi come [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), e [`SDKHookResponseMessage`](#sdkhookresponsemessage). Gli eventi del ciclo di vita per gli hook `SessionStart` e `Setup` sono sempre inclusi e non necessitano di questa opzione. Alcuni eventi hook, come `Notification`, `SessionEnd`, `PreCompact`, e `PostCompact`, non producono mai un `SDKHookStartedMessage`, anche con questa opzione. Per questi eventi, Claude Code emette comunque un `SDKHookProgressMessage` mentre un hook di comando che viene eseguito per più di un secondo produce output, ed emette un `SDKHookResponseMessage` solo quando un hook [che viene eseguito in background](/docs/it/hooks#run-hooks-in-the-background) termina                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                                         | Includi gli eventi di messaggi parziali                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                                         | *Alpha.* Timeout in millisecondi per ogni chiamata `sessionStore.load()` e `sessionStore.listSubkeys()` durante la materializzazione del resume. Se l'adapter non si stabilizza entro questa finestra, la query fallisce invece di bloccarsi. Ignorato quando `sessionStore` non è impostato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                                     | Impostazioni a livello di policy che il tuo processo host fornisce alla sessione generata. Su macchine con impostazioni gestite distribuite dall'amministratore, Claude Code ignora queste a meno che la fonte gestita di priorità più alta dell'amministratore non imposti `parentSettingsBehavior: 'merge'`, e non le unisce mai mentre un [`policyHelper`](/docs/it/settings-reference#policyhelper) fornisce impostazioni gestite. I valori uniti passano attraverso un filtro solo restrittivo; [Limita le impostazioni padre](/docs/it/claude-apps-gateway#restrict-parent-settings) copre ciò che il filtro ammette e i blocchi `allowManaged*Only`. Un host che imposta [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/it/env-vars) ha tre chiavi lette direttamente da questo payload: la sua [configurazione del modello](/docs/it/model-config#restrict-model-selection) su Claude Code v2.1.222 o successivo, [`modelPricing`](/docs/it/settings-reference#modelpricing) quando nessuna fonte gestita lo imposta su v2.1.246 o successivo, e la sua voce `ENABLE_TOOL_SEARCH` env su v2.1.247 o successivo                                                                                                                                                                                                                            |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                                     | Interrompi la query quando la stima del costo lato client raggiunge questo valore in USD. Conta solo la spesa della chiamata stessa; i totali ripristinati da una sessione ripresa non contano. Per le avvertenze di accuratezza e il comportamento di reset, vedi [Traccia costo e utilizzo](/docs/it/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                                     | *Deprecato:* Usa `thinking` invece. Token massimi per il processo di pensiero                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                                     | Turni agentici massimi (round trip di utilizzo di strumenti)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                                            | Configurazioni del server MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `model`                           | `string`                                                                                                                                                                                                       | Predefinito da CLI                              | Alias del modello Claude o nome completo del modello. Vedi [valori accettati e ID specifici del provider](/docs/it/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                                     | Callback per gestire le richieste di elicitazione MCP. Chiamato quando un server MCP richiede input dell'utente e nessun hook lo gestisce per primo. Quando non fornito, le richieste di elicitazione non gestite vengono rifiutate automaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                                     | Definisci il formato di output per i risultati dell'agente. Vedi [Structured outputs](/docs/it/agent-sdk/structured-outputs) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                                     | Non è un campo `Options`. Imposta `outputStyle` nell'oggetto [`settings`](/docs/it/settings) inline o in un file di impostazioni. Vedi [Attiva uno stile di output](/docs/it/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | Auto-risolto dal binario nativo in bundle       | Percorso all'eseguibile Claude Code. Necessario solo se le dipendenze opzionali sono state saltate durante l'installazione o la tua piattaforma non è nel set supportato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                                     | Modalità di permesso per la sessione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                                     | Nome dello strumento MCP per i prompt di permesso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                                        | Chi risponde ai prompt di permesso: `'host'` li instrada al tuo callback [`canUseTool`](#canusetool) o allo strumento `permissionPromptToolName`, e `'none'` [nega le chiamate che avrebbero richiesto](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated). Richiede Claude Code v2.1.259 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                                          | Quando `false`, disabilita la persistenza della sessione su disco. Le sessioni non possono essere riprese in seguito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                                     | Istruzioni di flusso di lavoro personalizzate per la modalità plan. Quando `permissionMode` è `'plan'`, questa stringa sostituisce il corpo del flusso di lavoro della modalità plan predefinito. La CLI lo avvolge comunque con il preambolo di applicazione di sola lettura e il footer del protocollo ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                                            | Carica plugin personalizzati da percorsi locali. Vedi [Plugins](/docs/it/agent-sdk/plugins) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                                     | Percorso assoluto del checkout attendibile di cui `cwd` è un worktree. Claude Code legge le impostazioni del progetto, `.mcp.json`, e i comandi, agenti, skill, flussi di lavoro, routine e stili di output del progetto `.claude/` da questa directory invece che da `cwd`, e imposta `CLAUDE_PROJECT_DIR` su di essa. Gli hook, gli script helper come `apiKeyHelper`, e i server MCP stdio iniziano con questa directory come loro directory di lavoro. I file `CLAUDE.md` e `.claude/rules/` caricano comunque da `cwd`. Richiede Claude Code v2.1.275 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                                         | Abilita i suggerimenti di prompt. Dopo un turno, Claude Code emette un messaggio `prompt_suggestion` che trasporta un prompt utente previsto successivo. Claude Code non genera suggerimenti per alcuni turni, come quando il tuo account è vicino o al limite di utilizzo. Vedi [Quando Claude Code salta i suggerimenti](/docs/it/interactive-mode#when-claude-code-skips-suggestions)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                                     | ID della sessione da riprendere                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                                     | Con `resumeSessionAt`: l'UUID del prompt del turno che il resume di troncamento intende scartare. Claude Code rifiuta il resume quando l'intervallo scartato contiene qualcosa non attribuibile a quel turno, come messaggi in coda assorbiti o notifiche di attività, e nomina il flag `--resume-drops-turn` nel messaggio di rifiuto. Solo l'Agent SDK e i resume in modalità print leggono la coppia. Richiede Claude Code v2.1.223 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                                     | Riprendi la sessione a un UUID di messaggio specifico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                                     | Configura il comportamento della sandbox a livello di programmazione. Vedi [Sandbox settings](#sandboxsettings) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `sessionId`                       | `string`                                                                                                                                                                                                       | Auto-generato                                   | Usa un UUID specifico per la sessione invece di generarne uno automaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sessionStore`                    | [`SessionStore`](/docs/it/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                                     | Specchia le trascrizioni della sessione in un backend esterno in modo che un altro host possa riprenderle. Vedi [Persist sessions to external storage](/docs/it/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                                     | *Alpha.* Modalità di flush per `sessionStore`. Ignorato quando `sessionStore` non è impostato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                                     | Oggetto [settings](/docs/it/settings) inline, percorso file di impostazioni, o stringa JSON inline. Popola il livello flag-settings nell'[ordine di precedenza](/docs/it/settings#settings-precedence). Cambia a runtime con [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | Impostazioni predefinite CLI (tutte le fonti)   | Controlla quali impostazioni del filesystem caricare. Passa `[]` per disabilitare le impostazioni utente, progetto e locali. [Endpoint-managed policy](/docs/it/managed-settings#delivery-mechanisms) carica comunque; le impostazioni gestite dal server vengono recuperate quando la sessione si autentica con una credenziale organizzativa su una [configurazione idonea](/docs/it/server-managed-settings#platform-availability). Vedi [Usa le funzioni Claude Code](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                                     | Skill disponibili per la sessione. Passa `'all'` per abilitare ogni skill scoperta, o un elenco di nomi di skill. Passa solo nomi esatti. Su Agent SDK v0.3.221 o successivo, l'SDK rifiuta i nomi malformati e in forma wildcard con un errore prima di avviare il processo Claude Code. Quando impostato, l'SDK aggiunge automaticamente lo strumento Skill a `allowedTools`. Se passi anche `tools`, includi `'Skill'` in quell'elenco. Vedi [Skills](/docs/it/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                                     | Funzione personalizzata per generare il processo Claude Code. Usa per eseguire Claude Code in VM, container o ambienti remoti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                                     | Callback per l'output stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                                         | Usa solo i server passati in `mcpServers` e ignora il progetto `.mcp.json`, le impostazioni utente, i server MCP forniti dal plugin, e i [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined` (prompt minimo)                     | Configurazione del prompt di sistema. Passa una stringa per un prompt personalizzato, o `{ type: 'preset', preset: 'claude_code' }` per usare il prompt di sistema di Claude Code. Passa un array di stringhe con la costante esportata `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` tra le parti statiche e per-richiesta per [memorizzare nella cache la parte statica di un prompt personalizzato](/docs/it/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt). Quando usi la forma dell'oggetto preset, aggiungi `append` per estenderlo con istruzioni aggiuntive, e imposta `excludeDynamicSections: true` per spostare il contesto per-sessione nel primo messaggio utente per [un migliore riutilizzo della cache dei prompt tra le macchine](/docs/it/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines). Imposta `snapshot: false` per ricostruire il prompt su ogni richiesta invece di [riutilizzare il prompt che la sessione ha registrato sulla sua prima richiesta](/docs/it/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Per impostare `snapshot` su un prompt personalizzato, passa la forma `{ type: 'custom', prompt }`. La forma `{ type: 'custom' }` e il campo `snapshot` richiedono TypeScript Agent SDK v0.3.257 o successivo |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                                     | *Alpha.* Budget di attività lato API in token. Quando impostato, al modello viene detto il suo budget di token rimanente in modo che possa regolare l'utilizzo dello strumento e concludere prima del limite                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | `{ type: 'adaptive' }` per i modelli supportati | Controlla il comportamento di pensiero/ragionamento di Claude. Vedi [`ThinkingConfig`](#thinkingconfig) per le opzioni                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                                     | Titolo di visualizzazione per la sessione. Quando si riprende tramite `resume` o `continue`, il titolo persistente della sessione ripresa ha la precedenza; usa [`renameSession()`](#renamesession) per rinominare una sessione esistente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                                     | Mappa i nomi degli strumenti incorporati ai nomi degli strumenti MCP in modo che Claude chiami la tua implementazione MCP al posto di quella incorporata. Ad esempio, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                                     | Configurazione per il comportamento dello strumento incorporato. Vedi [`ToolConfig`](#toolconfig) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                                     | Configurazione dello strumento. Passa un array di nomi di strumenti o usa il preset per ottenere gli strumenti predefiniti di Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="handle-slow-or-stalled-api-responses">
  Gestire risposte API lente o bloccate
</h4>

Il subprocess CLI legge diverse variabili di ambiente che controllano i timeout dell'API e il rilevamento dei blocchi. Passale tramite l'opzione `env`:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

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

* `API_TIMEOUT_MS`: timeout per-richiesta sul client Anthropic, in millisecondi. Predefinito `600000`. Si applica al loop principale e a tutti i subagent.
* `CLAUDE_CODE_MAX_RETRIES`: tentativi API massimi. Predefinito `10`, limitato a `15`. Ogni tentativo ottiene la sua finestra `API_TIMEOUT_MS`, quindi il tempo di parete nel caso peggiore è approssimativamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` più backoff. Per le esecuzioni incustodite che devono aspettare attraverso interruzioni più lunghe, imposta [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/it/errors#tune-retry-behavior): ritenta gli errori di capacità transienti indefinitamente e, su Claude Code v2.1.199 o successivo, aumenta il predefinito per altri errori transienti a `300` e rimuove il limite su questa variabile.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog di blocco per i subagent. Mentre il watchdog del flusso è attivo, il predefinito è `CLAUDE_STREAM_IDLE_TIMEOUT_MS` più 5 minuti, che arriva a `600000` a meno che non aumenti quella variabile. Con il watchdog del flusso spento, il predefinito è `600000`. Prima di v2.1.257, il predefinito era sempre `600000`.

  Il timer si ripristina su ogni evento del flusso. Su un blocco, Claude Code interrompe il subagent e segnala il blocco al genitore. Per un subagent in background, contrassegna anche l'attività come non riuscita e allega qualsiasi risultato parziale.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` con `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: watchdog del flusso che interrompe la richiesta quando le intestazioni sono arrivate ma il corpo della risposta smette di trasmettere. Il watchdog è attivo per impostazione predefinita per tutti i provider; imposta `CLAUDE_ENABLE_STREAM_WATCHDOG=0` per disabilitarlo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` predefinito a `300000` e viene bloccato a quel minimo. Dopo l'interruzione, [Automatic retries](/docs/it/errors#automatic-retries) copre cosa fa Claude Code, in base a quanto la risposta aveva progredito.

  Mentre il watchdog aspetta una risposta che un gateway dietro `ANTHROPIC_BASE_URL` tiene aperta con ping keep-alive, un host che imposta `includePartialMessages` continua a ricevere eventi di `ping` [stream](#sdkpartialassistantmessage), quindi leggi quei frame come vivacità piuttosto che cronometrare la sessione su silenzio. Prima di v2.1.257, i frame si fermavano 5 minuti dopo l'ultimo evento di flusso reale.

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
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  readMcpResource(serverName: string, uri: string): Promise<SDKControlMcpReadResourceResponse>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Metodi
</h4>

| Metodo                                 | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Interrompe la query. Disponibile solo in modalità di input in streaming. Quando la CLI pubblicizza la capacità `interrupt_receipt_v1` in [`SDKSystemMessage.capabilities`](#sdksystemmessage), si risolve con un [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) che elenca i messaggi che erano in sospeso quando l'interruzione è arrivata. Si risolve `undefined` su CLI prima di v2.1.205                                                                                                                                                     |
| `rewindFiles(userMessageId, options?)` | Ripristina i file al loro stato al messaggio utente specificato. Passa `{ dryRun: true }` per visualizzare in anteprima le modifiche. Richiede `enableFileCheckpointing: true`. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                     |
| `setPermissionMode()`                  | Cambia la modalità di permesso (disponibile solo in modalità di input in streaming)                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `setModel()`                           | Cambia il modello (disponibile solo in modalità di input in streaming). Passare `undefined` o la stringa `"default"` ripristina il [modello predefinito di Claude Code](/docs/it/model-config)                                                                                                                                                                                                                                                                                                                                                                       |
| `setMaxThinkingTokens()`               | *Deprecato:* Usa l'opzione `thinking` invece. Cambia i token di pensiero massimi. Passare `null` ripristina il pensiero al predefinito della sessione: un override mid-session viene cancellato, e il pensiero rimane disattivato per le sessioni che lo hanno disabilitato                                                                                                                                                                                                                                                                                     |
| `applyFlagSettings(settings)`          | Unisce le impostazioni nel livello flag settings della sessione a runtime (disponibile solo in modalità di input in streaming). Vedi [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                |
| `updateSettings(source, settings)`     | Scrive una chiave consentita nel file di impostazioni locali del progetto o nel file di impostazioni utente, in modo che il valore persista per le sessioni successive. Vedi [`updateSettings()`](#updatesettings). Richiede TypeScript SDK v0.3.257 o successivo, che raggruppa Claude Code v2.1.257                                                                                                                                                                                                                                                           |
| `initializationResult()`               | Restituisce il risultato di inizializzazione completo inclusi i comandi supportati, i modelli, le informazioni dell'account e la configurazione dello stile di output                                                                                                                                                                                                                                                                                                                                                                                           |
| `reinitialize()`                       | Invia di nuovo la richiesta di controllo `initialize` alla CLI in esecuzione e restituisce un risultato fresco invece del risultato memorizzato nella cache della prima connessione. Usalo dopo un gap di trasporto, come il ricollegamento a una sessione dopo una disconnessione, in modo che le richieste di permesso in sospeso raggiungano di nuovo il tuo callback `canUseTool`. Rendi il callback idempotente per ID di richiesta, perché una richiesta la cui risposta è stata persa viene inviata di nuovo. Richiede Claude Code v2.1.195 o successivo |
| `supportedCommands()`                  | Restituisce i comandi disponibili. Da Agent SDK v0.3.216 l'elenco riflette i cambiamenti di comando mid-session; vedi [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)                                                                                                                                                                                                                                                                                                                                                                                 |
| `supportedModels()`                    | Restituisce i modelli disponibili con informazioni di visualizzazione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `supportedAgents()`                    | Restituisce i subagent disponibili come [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `mcpServerStatus()`                    | Restituisce lo stato dei server MCP connessi come [`McpServerStatus`](#mcpserverstatus)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `getContextUsage(opts?)`               | Restituisce un [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) che suddivide l'utilizzo della finestra di contesto della sessione per categoria, skill e strumento. Con il predefinito `detail`, sono gli stessi dati che `/context` mostra in una sessione interattiva. L'[opzione `detail`](#sdkcontrolgetcontextusageresponse) richiede Agent SDK v0.3.257 o successivo                                                                                                                                                            |
| `readFile(path, options?)`             | Legge un file dal filesystem della sessione. Claude Code risolve il percorso rispetto a `cwd`; [Cosa `readFile()` può leggere](#what-readfile-can-read) elenca i file che serve. Passa `{ maxBytes }` per cambiare il limite di lettura (predefinito 1 MB, massimo 10 MB) e `{ encoding: 'base64' }` per file binari come immagini. Si risolve con un [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse), o `null` su rifiuto di permesso, file mancante, o errore di trasporto. Richiede TypeScript SDK v0.2.121 o successivo                         |
| `reloadSkills()`                       | Ricarica le skill dal disco, in modo che le skill che aggiungi o modifichi mid-session diventino disponibili per la sessione in esecuzione. Si risolve con un [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) che elenca le skill disponibili dopo il ricaricamento. Richiede Agent SDK v0.3.163 o successivo                                                                                                                                                                                                                               |
| `accountInfo()`                        | Restituisce le informazioni dell'account                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `reconnectMcpServer(serverName)`       | Ricollega un server MCP per nome. Se il nome corrisponde anche a una voce in un file di impostazioni come `.mcp.json` o `~/.claude.json`, Claude Code ricollega il server che hai configurato tramite [`mcpServers`](#options) o `setMcpServers()`, non la voce del file di impostazioni. Quell'ordine di risoluzione richiede Claude Code v2.1.257 o successivo                                                                                                                                                                                                |
| `toggleMcpServer(serverName, enabled)` | Abilita o disabilita un server MCP per nome, con la stessa risoluzione del nome di `reconnectMcpServer()`. La disabilitazione disconnette il server                                                                                                                                                                                                                                                                                                                                                                                                             |
| `setMcpServers(servers)`               | Sostituisci dinamicamente l'insieme dei server MCP per questa sessione. Si risolve con un [`McpSetServersResult`](#mcpsetserversresult) che nomina quali server sono stati aggiunti e rimossi, e eventuali errori                                                                                                                                                                                                                                                                                                                                               |
| `readMcpResource(serverName, uri)`     | *Alpha.* Legge una risorsa MCP Apps `ui://` da un server MCP connesso in modo che la tua applicazione possa rendere il widget di uno strumento. Si risolve con un [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse). Richiede TypeScript Agent SDK v0.3.280 o successivo                                                                                                                                                                                                                                                                |
| `streamInput(stream)`                  | Trasmetti i messaggi di input alla query per conversazioni multi-turno                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `stopTask(taskId)`                     | Interrompi un'attività in background in esecuzione per ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `close()`                              | Chiudi la query e termina il processo sottostante. Termina forzatamente la query e pulisce tutte le risorse                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Cambia le [impostazioni](/docs/it/settings) su una sessione in esecuzione senza riavviare la query. Usalo quando un'impostazione che non ha un setter dedicato deve cambiare mid-session, come l'irrigidimento di `permissions` dopo che l'agente legge input non attendibile. `setModel()` e `setPermissionMode()` sono setter dedicati per quelle due chiavi; `applyFlagSettings()` è la forma generale che accetta qualsiasi sottoinsieme delle chiavi di impostazioni, e passare `model` qui si comporta come `setModel()`.

Solo alcune chiavi hanno effetto mid-session:

* **Applicate al turno successivo**: `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Passare a `agent` applica anche l'override del modello e gli hook di quell'agente al turno successivo. Il suo prompt di sistema si applica al turno successivo, o, in una sessione che [riutilizza un prompt di sistema registrato](/docs/it/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session), una volta che la sessione è compattata.
* **Applicate durante il turno corrente**: `model`. Se cambi `model` mentre Claude sta lavorando su un turno, la risposta che Claude sta già generando termina sul modello vecchio, e il resto del turno, a partire dalla prossima chiamata che Claude Code fa al modello, usa quello nuovo. I subagent mantengono il loro modello. Prima di v2.1.212, un cambio mid-turn aspettava il turno successivo.
* **Nessun effetto mid-session**: le opzioni del prompt di sistema. Questi vengono risolti una volta all'avvio, quindi la sessione in esecuzione mantiene il valore originale anche se la chiamata ha successo. Per cambiarli, avvia una nuova sessione.

`effortLevel` accetta un nome di [livello di sforzo](/docs/it/model-config#adjust-effort-level). Accetta anche `"ultracode"`, che richiede uno sforzo `xhigh` con [ultracode](/docs/it/workflows#let-claude-decide-with-ultracode) attivo. `applyFlagSettings()` dichiara `effortLevel` senza quel valore, quindi passa l'equivalente `{ ultracode: true }` in TypeScript. Il valore `ultracode` richiede Claude Code v2.1.203 o successivo ed è accettato solo da `applyFlagSettings()`, non dalla chiave `effortLevel` in un file di impostazioni.

I valori vengono scritti nel livello flag-settings, lo stesso livello che l'opzione `settings` inline di `query()` popola all'avvio. Questo è lo stesso livello che la [sezione di precedenza sulla pagina](#settings-precedence) chiama opzioni programmatiche.

Le chiamate successive uniscono superficialmente le chiavi di livello superiore. Una seconda chiamata con `{ permissions: {...} }` sostituisce l'intero oggetto `permissions` dalla chiamata precedente piuttosto che unirsi profondamente in esso.

Per cancellare una chiave che hai impostato con `applyFlagSettings()`, passa `null` per quella chiave. La maggior parte delle chiavi ricade quindi su un valore che l'opzione `settings` di `query()` ha impostato all'avvio, quindi su fonti di precedenza inferiore. Un `model` cancellato si ripristina al [modello predefinito di Claude Code](/docs/it/model-config), anche quando un file di impostazioni imposta `model`. Passare `undefined` non ha effetto perché la serializzazione JSON lo elimina.

Tre chiavi oltre a `model` ripristinano lo stato della sessione invece di ricadere:

* `effortLevel: null` restituisce la sessione al livello di sforzo predefinito del modello, non all'opzione `effort` di `query()` o a un `effortLevel` da un file di impostazioni.
* `agent: null` esegue il thread principale senza agente, a partire dal turno successivo, piuttosto che ripristinare l'opzione `agent` di `query()` o un `agent` da un file di impostazioni. Se l'agente cancellato aveva applicato il suo modello, la sessione ritorna al modello che ha risolto all'avvio.
* `ultracode: null` disattiva ultracode, come `false` fa, piuttosto che ripristinare un valore `ultracode` da un file di impostazioni. La sessione mantiene il suo livello di sforzo corrente, quindi passa `effortLevel` nella stessa chiamata per cambiarlo.

Disponibile solo in modalità di input in streaming, lo stesso vincolo di `setModel()` e `setPermissionMode()`.

L'esempio seguente cambia il modello attivo mid-session, quindi cancella l'override in modo che il modello si ripristini al [modello predefinito di Claude Code](/docs/it/model-config).

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Override the model for the rest of the session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Later: clear the override; the model resets to Claude Code's default
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` è solo TypeScript. L'SDK Python non espone un metodo equivalente.
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

Scrive una chiave consentita in un file di impostazioni su disco, in modo che il valore persista per le sessioni successive che caricano quella fonte. Ogni fonte accetta una chiave, con un valore stringa:

* **`"localSettings"`**: accetta `outputStyle` e lo unisce nel file di impostazioni locali del progetto, `.claude/settings.local.json`. Il nuovo stile ha effetto sulla richiesta successiva della sessione.
* **`"userSettings"`**: accetta `effortLevel` e lo salva come [livello di sforzo](/docs/it/model-config#adjust-effort-level) predefinito per il modello corrente della sessione, sotto [`modelSettings`](/docs/it/settings-reference#modelsettings) nel file di impostazioni utente. Passare `max` non scrive nulla, perché `max` è solo per sessione. La sessione in esecuzione mantiene il suo livello di sforzo corrente comunque, quindi chiama [`applyFlagSettings()`](#applyflagsettings) quando vuoi anche cambiare quello. Questa fonte richiede TypeScript SDK v0.3.277 o successivo, che raggruppa Claude Code v2.1.277.

La chiamata rifiuta quando la richiesta trasporta qualsiasi altra chiave, quando la sessione viene eseguita su un trasporto remoto, e quando il [`settingSources`](#options) della sessione esclude la fonte che nomini. L'eliminazione di una chiave non è supportata.

<h3 id="warmquery">
  `WarmQuery`
</h3>

Handle restituito da [`startup()`](#startup). Il subprocess è già generato e inizializzato, quindi chiamare `query()` su questo handle scrive il prompt direttamente a un processo pronto senza latenza di avvio.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Metodi
</h4>

| Metodo          | Descrizione                                                                                                                                |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Invia un prompt al subprocess pre-riscaldato e restituisci un [`Query`](#query-object). Può essere chiamato solo una volta per `WarmQuery` |
| `close()`       | Chiudi il subprocess senza inviare un prompt. Usalo per scartare una warm query che non è più necessaria                                   |

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
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
};
```

`hooks_applied` segnala se Claude Code ha registrato gli `hooks` che la richiesta `initialize` ha trasportato. L'SDK invia quella richiesta una volta quando la sessione inizia e di nuovo su ogni chiamata [`reinitialize()`](#query-object). Il campo richiede Agent SDK v0.3.238 o successivo.

Claude Code omette il campo quando la richiesta non ha trasportato hook. Quando la richiesta ha trasportato hook, il valore dipende dal fatto che la richiesta sia la prima inizializzazione della sessione e, per una ripetuta, da come ha raggiunto la sessione:

* `true`: Claude Code ha registrato gli hook. Un'inizializzazione della prima sessione restituisce questo valore. Un'inizializzazione ripetuta inviata su stdin della CLI restituisce anche `true`. In quel caso gli hook nella nuova richiesta sostituiscono gli hook registrati in precedenza.
* `false`: Claude Code ha ignorato gli hook. Un'inizializzazione ripetuta inviata a una sessione remota restituisce questo valore, quindi un secondo client che si unisce a una sessione non può sostituire gli hook che il primo client ha registrato.

Prima di Agent SDK v0.3.238, la risposta non ha mai trasportato il campo, e Claude Code ha ignorato `hooks` su ogni inizializzazione ripetuta.

La risposta segnala sempre `fast_mode_state`, e quando qualcosa blocca [fast mode](/docs/it/fast-mode), `fast_mode_disabled_reason` trasporta il codice della ragione insieme ad esso, in modo che tu possa spiegare lo stato bloccato invece di ri-derivare la disponibilità. Entrambi i comportamenti richiedono Claude Code v2.1.219 o successivo. Prima di v2.1.219, la risposta ometteva `fast_mode_state` quando fast mode non era disponibile e non trasportava mai una ragione. Per i codici della ragione e i loro significati, vedi [`fast_mode_disabled_reason`](#sdkresultmessage) sul messaggio di risultato.

Il wrapper di risposta di controllo per un `initialize` riuscito trasporta anche un array `pending_permission_requests`. Il campo è sul wrapper di risposta stesso, non nel payload `SDKControlInitializeResponse` sopra. Ogni voce è un messaggio `control_request` completo con la stessa forma `{ type: "control_request", request_id, request }` che la sessione trasmette per le richieste di permesso durante l'esecuzione.

L'array elenca le richieste di permesso che questo processo Claude Code ha emesso e non ancora risolto. L'SDK legge l'array per te e invia ogni voce al tuo callback [`canUseTool`](#canusetool), la stessa reinoltro che [`reinitialize()`](#query-object) attiva dopo un gap di trasporto. Gestisci gli ID di richiesta ripetuti in modo idempotente, perché una voce può ripetere una richiesta che il callback ha già ricevuto prima che la connessione si interrompesse.

L'array è sempre presente su una risposta `initialize` riuscita ed è vuoto quando questo processo non ha alcuna richiesta di permesso non risolta. Richiede Claude Code v2.1.268 o successivo. Le versioni precedenti potrebbero omettere il campo, quindi se analizzi il protocollo wire tu stesso, tratta un campo mancante come una CLI più vecchia piuttosto che come prova che nulla è in sospeso.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

La ricevuta di interruzione: il valore che [`interrupt()`](#query-object) si risolve con su una CLI che pubblicizza la capacità `interrupt_receipt_v1` in [`SDKSystemMessage.capabilities`](#sdksystemmessage). Richiede Claude Code v2.1.205 o successivo. Le CLI precedenti rispondono all'interruzione con un payload di successo vuoto, quindi `interrupt()` si risolve a `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` elenca gli UUID dei messaggi utente che erano in sospeso quando l'interruzione è arrivata: messaggi ancora nella coda, più eventuali messaggi che Claude Code aveva già tolto dalla coda per il turno successivo. Una volta che il primo turno della sessione ha iniziato, Claude Code elabora i messaggi elencati dopo l'interruzione a meno che non li annulli per primo, e può unire diversi in un turno. Se interrompi prima che il primo turno inizi, Claude Code interrompe quel turno non appena inizia, e i messaggi elencati in quel turno non ricevono risposta.

Usa la ricevuta per decidere se inviare di nuovo qualcosa. Un messaggio elencato che non annulli entra nella conversazione indipendentemente dal fatto che riceva una risposta, quindi rinviarlo lo consegna a Claude due volte.

Interpreta l'elenco con questi avvertimenti:

* Solo i messaggi che sono stati accodati con un UUID appaiono. Un array vuoto non significa che nient'altro verrà eseguito.
* Solo i messaggi del thread principale sono elencati. I messaggi indirizzati a un subagent sono fuori portata.
* L'elenco può includere UUID che il tuo client non ha mai inviato, come i trigger di [attività pianificate](/docs/it/scheduled-tasks). Ignora gli UUID che non riconosci invece di trattarli come un errore.

Un client che guida il protocollo di controllo della CLI direttamente, piuttosto che tramite `interrupt()`, può impostare `cancel_queued: true` sulla richiesta di controllo `interrupt`. Claude Code v2.1.219 e successivo pubblicizza il supporto con la capacità `interrupt_cancel_queued_v1` in [`SDKSystemMessage.capabilities`](#sdksystemmessage); le CLI più vecchie ignorano il campo e lasciano i messaggi in coda per l'esecuzione come al solito. Tale interruzione annulla anche ogni messaggio che altrimenti sarebbe elencato sotto `still_queued`: la ricevuta li elenca sotto `cancelled` invece, `still_queued` è vuoto, e nessuno di loro viene eseguito.

L'elenco `cancelled` trasporta gli stessi avvertimenti di `still_queued`. Il metodo `interrupt()` non invia mai `cancel_queued`, quindi le ricevute che si risolve non trasportano `cancelled`.

La ricevuta è uno snapshot scattato nel momento in cui l'interruzione viene elaborata, e su un'interruzione pulita arriva prima del [`SDKResultMessage`](#sdkresultmessage) del turno interrotto. Leggi la ricevuta piuttosto che ispezionare la coda dopo quel risultato: il loop avvia il turno in coda successivo immediatamente, quindi la coda che ispezioni dopo il risultato ha già cambiato.

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

Tipo di ritorno di [`getContextUsage()`](#query-object). Con il predefinito `detail`, questo è lo stesso payload che Claude Code rende per il comando `/context` in una sessione interattiva, quindi insieme ai conteggi di token trasporta campi di visualizzazione come `color` e `gridRows` che Claude Code usa per disegnare la griglia di utilizzo `/context`.

L'argomento `detail` facoltativo del metodo sceglie come Claude Code conta ogni categoria. Con il predefinito, `'full'`, Claude Code conta ogni categoria con richieste API di conteggio dei token. Passa `{ detail: 'summary' }` per ottenere una risposta dall'utilizzo dell'ultima risposta e dalle stime locali. Nessuna richiesta di conteggio dei token esce, e i numeri per categoria sono approssimativi. L'argomento `detail` richiede Agent SDK v0.3.257 o successivo.

Quando invii `/context` come prompt invece di chiamare il metodo, Claude Code allega un payload [`SDKContextUsage`](#sdkcontextusage) al campo `context_usage` del messaggio dell'assistente che consegna il risultato. Quel campo richiede Agent SDK v0.3.232 o successivo.

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

Leggi l'attribuzione dei token dalle raccolte di campi:

* `categories` contiene i totali per categoria.
* `mcpTools` e `agents` attribuiscono i token ai singoli strumenti MCP e subagent.
* `memoryFiles` elenca ogni file di memoria caricato con il suo costo.
* `skills.skillFrontmatter` attribuisce i token della voce di skill a ogni skill inclusa. I conteggi per skill misurano ogni voce di skill come Claude Code effettivamente la invia, che può essere più breve del frontmatter completo della skill. Confronta `skills.totalSkills` con `skills.includedSkills` per vedere se ogni skill scoperta ha fatto parte dell'elenco.

`totalTokens` è l'utilizzo di contesto corrente della sessione, e `maxTokens` è la finestra rispetto alla quale l'utilizzo viene misurato. Quella finestra è la finestra di contesto del modello, o la finestra di auto-compattazione inferiore quando una si applica. `rawMaxTokens` trasporta lo stesso valore di `maxTokens`, e `percentage` è `totalTokens` come percentuale arrotondata di quella finestra.

Claude Code lascia i diagnostici facoltativi `deferredBuiltinTools`, `systemTools`, e `systemPromptSections` non impostati, quindi aspettati che siano assenti anche se il tipo li dichiara.

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

Tipo di ritorno di [`readFile()`](#query-object).

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` contiene il testo del file, o dati base64 quando hai richiesto `encoding: 'base64'`; il campo `encoding` della risposta è impostato a `'base64'` in quel caso. `absPath` è il percorso assoluto risolto. `truncated` è impostato quando il file era più lungo del limite `maxBytes` e i contenuti sono stati tagliati a quel limite.

<h4 id="what-readfile-can-read">
  Cosa `readFile()` può leggere
</h4>

`readFile()` serve un insieme più ristretto di file rispetto allo strumento Read:

* Un file regolare all'interno di una delle directory di lavoro della sessione, come `cwd` e `additionalDirectories`
* Alcuni dei file di Claude Code stesso per la sessione, come i risultati degli strumenti

Le regole di negazione e richiesta di Read bloccano comunque un percorso corrispondente, e una regola di autorizzazione Read ampia non apre il resto del filesystem a `readFile()`. Per qualsiasi altra cosa la chiamata si risolve con `null`.

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

Tipo di ritorno di [`reloadSkills()`](#query-object).

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` elenca le skill disponibili dopo il ricaricamento, nella stessa forma [`SlashCommand`](#slashcommand) che `supportedCommands()` restituisce.

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

Tipo di ritorno di [`readMcpResource()`](#query-object), che trasporta il risultato `resources/read` del server MCP. Richiede TypeScript Agent SDK v0.3.280 o successivo.

```typescript theme={null}
type SDKControlMcpReadResourceResponse = {
  contents: {
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
    _meta?: Record<string, unknown>;
  }[];
};
```

Passa `readMcpResource()` il nome del server come `mcpServerStatus()` lo segnala e un URI `ui://`, come l'`ui.resourceUri` che uno strumento dichiara nel suo [`_meta`](#mcpserverstatus). La chiamata rifiuta per qualsiasi altro schema URI, per un [SDK MCP server](#createsdkmcpserver) che la tua applicazione ospita stessa, e per un server che non è connesso. È disponibile quando il messaggio init's [`capabilities`](#sdksystemmessage) include `mcp_read_resource_v1`.

Ogni voce `contents` è un elemento di contenuto come il server lo ha inviato. `blob` contiene dati base64 per un elemento binario, e `_meta` è il `_meta` dell'elemento stesso, dove un server MCP Apps mette il `ui.csp` e `ui.permissions` della risorsa. I contenuti sono HTML di terze parti non attendibile, quindi rendili in una sandbox.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configurazione per un subagent definito a livello di programmazione.

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
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Campo                                 | Richiesto | Descrizione                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------ | :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Sì        | Descrizione in linguaggio naturale di quando usare questo agente                                                                                                                                                                                                                                                                                                                              |
| `tools`                               | No        | Array di nomi di strumenti consentiti. Se omesso, eredita ogni [strumento disponibile per i subagent](/docs/it/sub-agents#available-tools). Per precaricare le Skill nel contesto dell'agente, usa il campo `skills` piuttosto che elencare `'Skill'` qui                                                                                                                                          |
| `disallowedTools`                     | No        | Array di nomi di strumenti da esplicitamente disalloware per questo agente. Sono accettati anche modelli a livello di server MCP: `mcp__server` o `mcp__server__*` rimuove ogni strumento da quel server, e `mcp__*` rimuove ogni strumento MCP da qualsiasi server                                                                                                                           |
| `prompt`                              | Sì        | Il prompt di sistema dell'agente                                                                                                                                                                                                                                                                                                                                                              |
| `model`                               | No        | Override del modello per questo agente. Accetta un alias come `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, o un ID modello completo. `'inherit'` usa il modello principale. Quando lo ometti, Claude Code sceglie il modello nell'[ordine del modello del subagent](/docs/it/sub-agents#choose-a-model)                                                                                |
| `mcpServers`                          | No        | Specifiche del server MCP per questo agente                                                                                                                                                                                                                                                                                                                                                   |
| `skills`                              | No        | Array di nomi di skill da precaricare nel contesto dell'agente                                                                                                                                                                                                                                                                                                                                |
| `initialPrompt`                       | No        | Auto-inviato come il primo turno utente quando questo agente viene eseguito come agente del thread principale                                                                                                                                                                                                                                                                                 |
| `maxTurns`                            | No        | Numero massimo di turni agentici (round-trip API) prima di fermarsi                                                                                                                                                                                                                                                                                                                           |
| `background`                          | No        | Esegui questo agente come un'attività in background non bloccante quando invocato                                                                                                                                                                                                                                                                                                             |
| `omitClaudeMd`                        | No        | Esegui questo agente senza i file CLAUDE.md utente, progetto e locali quando viene eseguito come subagent; i file di policy gestiti caricano comunque. Usalo per gli agenti che prendono tutto ciò di cui hanno bisogno dal prompt dello strumento Agent. Ignorato quando questo agente viene eseguito come agente del thread principale. Richiede TypeScript Agent SDK v0.3.271 o successivo |
| `memory`                              | No        | Fonte di memoria per questo agente: `'user'`, `'project'`, o `'local'`                                                                                                                                                                                                                                                                                                                        |
| `effort`                              | No        | Livello di sforzo di ragionamento per questo agente. Accetta un livello denominato o un numero intero                                                                                                                                                                                                                                                                                         |
| `permissionMode`                      | No        | Modalità di permesso per l'esecuzione dello strumento all'interno di questo agente. Le [regole di eredità del subagent](/docs/it/agent-sdk/permissions#available-modes) decidono quando si applica. Vedi [`PermissionMode`](#permissionmode)                                                                                                                                                       |
| `criticalSystemReminder_EXPERIMENTAL` | No        | Sperimentale: Promemoria critico aggiunto al prompt di sistema                                                                                                                                                                                                                                                                                                                                |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Specifica i server MCP disponibili per un subagent. Può essere un nome di server (stringa che fa riferimento a un server dalla configurazione `mcpServers` del genitore) o un record di configurazione del server inline che mappa i nomi dei server alle configurazioni.

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

| Valore      | Descrizione                                                                            | Posizione                     |
| :---------- | :------------------------------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Impostazioni globali dell'utente                                                       | `~/.claude/settings.json`     |
| `'project'` | Impostazioni del progetto condivise (controllate dalla versione)                       | `.claude/settings.json`       |
| `'local'`   | Impostazioni del progetto locale, gitignorate quando Claude Code salva un'impostazione | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamento predefinito
</h4>

Quando `settingSources` è omesso o `undefined`, `query()` carica le stesse impostazioni del filesystem della CLI Claude Code: utente, progetto e locale. Vedi [Cosa settingSources non controlla](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control) per gli input che vengono letti indipendentemente da questa opzione, e come disabilitarli.

<h4 id="why-use-settingsources">
  Perché usare settingSources
</h4>

**Disabilita le impostazioni del filesystem:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Do not load user, project, or local settings from disk
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Carica solo fonti di impostazioni specifiche:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Load only project settings, ignore user and local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Only .claude/settings.json
  }
});
```

Per caricare le istruzioni del progetto CLAUDE.md, includi `"project"` in `settingSources`. Vedi [Modifica i prompt di sistema](/docs/it/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) per come il caricamento di CLAUDE.md interagisce con le opzioni del prompt di sistema.

<h4 id="settings-precedence">
  Precedenza delle impostazioni
</h4>

Quando più fonti vengono caricate, le impostazioni vengono unite con questa precedenza (da più alta a più bassa):

1. Impostazioni locali (`.claude/settings.local.json`)
2. Impostazioni del progetto (`.claude/settings.json`)
3. Impostazioni dell'utente (`~/.claude/settings.json`)

Le opzioni programmatiche come `agents`, `allowedTools`, e `settings` sovrascrivono le impostazioni del filesystem utente, progetto e locale. Le impostazioni di policy gestite hanno precedenza sulle opzioni programmatiche.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Standard permission behavior
  | "acceptEdits" // Auto-accept file edits
  | "bypassPermissions" // Bypass permission checks; explicit ask rules still prompt
  | "plan" // Planning mode - explore without editing
  | "dontAsk" // Don't prompt for permissions, deny if not pre-approved
  | "auto"; // Model classifier approves or denies permission prompts
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Tipo di funzione di permesso personalizzato per controllare l'utilizzo dello strumento.

La funzione è il sostituto SDK per il prompt di permesso interattivo: viene invocata solo quando il [flusso di valutazione del permesso](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) si risolve in un prompt. Le chiamate dello strumento già approvate da una voce `allowedTools`, una regola di autorizzazione delle impostazioni, o la modalità di permesso, come `acceptEdits` o `bypassPermissions`, non la invocano mai. Per controllare ogni chiamata dello strumento, usa un hook [`PreToolUse`](/docs/it/agent-sdk/hooks) invece.

Una regola di autorizzazione non pre-approva le [azioni che nessuna modalità auto-approva](/docs/it/permission-modes#actions-no-mode-auto-approves); vedi [Come vengono valutati i permessi](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) per quale di loro raggiunge il callback e cosa succede in modalità `dontAsk` e `auto`.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Opzione          | Tipo                                        | Descrizione                                                                                                                                                                                                                                                                                                                                      |
| :--------------- | :------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Segnalato se l'operazione deve essere interrotta                                                                                                                                                                                                                                                                                                 |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Aggiornamenti di permesso suggeriti in modo che l'utente non venga richiesto di nuovo per questo strumento. I prompt Bash includono un suggerimento con la [destinazione](#permissionupdatedestination) `localSettings`, quindi restituirlo in `updatedPermissions` scrive la regola a `.claude/settings.local.json` e persiste tra le sessioni. |
| `blockedPath`    | `string`                                    | Il percorso del file che ha attivato la richiesta di permesso, se applicabile                                                                                                                                                                                                                                                                    |
| `mcpServer`      | `{ name: string; source: string }`          | Per uno strumento `mcp__*`, il server MCP che lo serve e da dove proviene la definizione di quel server, con i campi di [`McpServerProvenance`](#mcpserverprovenance). Assente per altri strumenti. Richiede Agent SDK v0.3.274 o successivo                                                                                                     |
| `decisionReason` | `string`                                    | Spiega perché questa richiesta di permesso è stata attivata                                                                                                                                                                                                                                                                                      |
| `toolUseID`      | `string`                                    | Identificatore univoco per questa specifica chiamata dello strumento all'interno del messaggio dell'assistente                                                                                                                                                                                                                                   |
| `agentID`        | `string`                                    | Se in esecuzione all'interno di un sub-agente, l'ID del sub-agente                                                                                                                                                                                                                                                                               |
| `requestId`      | `string`                                    | L'`request_id` dell'envelope `control_request`. Un `control_response` che la tua applicazione invia al di fuori dell'SDK, come un POST HTTP firmato, deve echeggiare questo valore in modo che il processo Claude Code possa abbinare la risposta alla richiesta                                                                                 |

Il callback normalmente risolve la richiesta restituendo un [`PermissionResult`](#permissionresult), che l'SDK scrive di nuovo sul suo trasporto come `control_response`. Restituisci `null` solo quando la tua applicazione ha già inviato il `control_response` per questa richiesta sul suo canale, echeggiando `requestId`; l'SDK quindi salta la scrittura della risposta al suo trasporto. Restituire `null` in qualsiasi altro caso lascia la chiamata dello strumento bloccata indefinitamente, perché nessun `control_response` viene mai inviato e i prompt di permesso non scadono.

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

Configurazione per il comportamento dello strumento incorporato.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Campo                           | Tipo                   | Descrizione                                                                                                                                                                                |
| :------------------------------ | :--------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Opta nel campo `preview` su opzioni [`AskUserQuestion`](/docs/it/agent-sdk/user-input#question-format) e imposta il suo formato di contenuto. Quando non impostato, Claude non emette anteprime |

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
  timeout?: number;
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

| Campo              | Tipo      | Descrizione                                                                                                                                                                                                              |
| :----------------- | :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Deve essere `'local'` (attualmente sono supportati solo plugin locali)                                                                                                                                                   |
| `path`             | `string`  | Percorso assoluto o relativo alla directory del plugin                                                                                                                                                                   |
| `skipMcpDiscovery` | `boolean` | Quando `true`, l'SDK carica skill, hook, agenti e comandi da questo plugin ma non legge il suo `.mcp.json` o il manifest `mcpServers`. Imposta questo quando la tua applicazione possiede le connessioni MCP del plugin. |

**Esempio:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Per informazioni complete sulla creazione e l'utilizzo dei plugin, vedi [Plugins](/docs/it/agent-sdk/plugins).

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
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Il campo `message` è un [`BetaMessage`](https://platform.claude.com/docs/it/api/messages/create) dall'SDK Anthropic. Include campi come `id`, `content`, `model`, `stop_reason` e `usage`.

`SDKAssistantMessageError` è uno di: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'account_on_hold'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, `'cloud_credential_error'`, o `'unknown'`. Quattro di questi valori significano più di quanto i loro nomi dicono:

* `'model_not_found'`: il modello selezionato non esiste o non è disponibile per il tuo account o deployment
* `'overloaded'`: l'API ha restituito un 529 perché il server è al massimo della capacità, a differenza di `'rate_limit'`, che è un 429 rispetto alla tua quota
* `'account_on_hold'`: [il tuo account è in sospeso](/docs/it/errors#your-account-is-on-hold)
* `'cloud_credential_error'`: Claude Code non ha potuto ottenere credenziali AWS o Google Cloud utilizzabili sulla macchina su cui viene eseguito, quindi nessuna richiesta ha raggiunto il provider cloud. La causa solita è un accesso al cloud scaduto o mai completato su quella macchina, anche se un servizio di credenziali brevemente irraggiungibile segnala lo stesso valore. Vedi [Impossibile caricare le credenziali AWS o Google Cloud](/docs/it/errors#could-not-load-aws-or-google-cloud-credentials). Richiede TypeScript Agent SDK v0.3.267 o successivo, che raggruppa Claude Code v2.1.267

`aborted` è `true` quando un'interruzione o un'interruzione ha troncato il messaggio dell'assistente prima del completamento del flusso: il messaggio non ha `stop_reason` e il contenuto può terminare a metà parola. Il campo è assente sui messaggi completati normalmente. Richiede Agent SDK v0.3.214 o successivo.

Claude Code imposta `user_message_uuid` e `user_message_uuids` sul primo messaggio dell'assistente del turno, secondo le condizioni in [`user_message_uuid`](#user_message_uuid).

`timestamp` è l'ora ISO 8601 quando il contenuto del messaggio ha finito di generarsi sul processo che lo ha prodotto. Il valore proviene dall'orologio di quella macchina, quindi usalo solo per la visualizzazione e non ordinare i messaggi per esso. Un turno API può produrre diversi messaggi dell'assistente che condividono un `message.id`, ciascuno con il proprio `timestamp`. Quando il campo è assente, ricadi al momento in cui hai ricevuto il messaggio.

`context_usage` è una copia strutturata del rapporto `/context`, tipizzata come [`SDKContextUsage`](#sdkcontextusage), e richiede Agent SDK v0.3.232 o successivo. Quando invii `/context` come prompt, Claude Code consegna il rapporto come messaggio dell'assistente il cui `message.content` contiene la tabella markdown, e allega `context_usage` a quello stesso messaggio. Claude Code non imposta il campo su nessun altro messaggio dell'assistente, e le versioni precedenti consegnano la tabella `/context` senza di esso, quindi leggi la suddivisione dal campo quando è presente e ricadi al testo markdown quando non lo è.

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
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

Imposta `pasted_content` per inviare contenuto che l'utente ha incollato nella tua interfaccia utente del prompt piuttosto che digitato, una voce per incolla, ciascuna una stringa o un array di blocchi di contenuto. Claude Code aggiunge il testo di ogni voce dopo il testo digitato, in ordine, e può avvolgere ogni incolla in tag `<pasted_content>`. I blocchi diversi dal testo vengono ignorati, quindi invia immagini e documenti in `message.content`. Richiede Agent SDK v0.3.277 o successivo.

Imposta `shouldQuery` a `false` per aggiungere il messaggio alla trascrizione senza attivare un turno dell'assistente. Il messaggio viene mantenuto e unito al prossimo messaggio utente che attiva un turno. Usa questo per iniettare contesto, come l'output di un comando che hai eseguito fuori banda, senza spendere una chiamata di modello su di esso.

Su un messaggio che contiene un blocco `tool_result`, `tool_use_result` è l'oggetto di output strutturato dello strumento piuttosto che il testo inviato al modello. La sua forma dipende dallo strumento denominato dal blocco `tool_use` corrispondente, quindi il campo è tipizzato `unknown`; le forme integrate sono elencate in [Tipi di output dello strumento](#tool-output-types).

Per lo strumento `Agent`, `tool_use_result` è [`AgentOutput`](#agent-2). Su un risultato `completed`, `content` contiene il rapporto del subagente senza l'ID agente e il trailer di utilizzo che Claude Code aggiunge al testo `tool_result`, quindi esegui il rendering da `tool_use_result` invece di analizzare quel testo.

Per uno strumento MCP il cui risultato contiene blocchi `resource_link`, `tool_use_result` è un oggetto con un array `resourceLinks` di voci [`SDKMcpResourceLink`](#sdkmcpresourcelink). Claude riceve ogni link come una riga di testo nel blocco `tool_result`, quindi leggi `resourceLinks` per rendere i file che il server ha restituito invece di analizzare quel testo. Claude Code omette `resourceLinks` quando il risultato non ha link e sui risultati dei subagenti, mantiene al massimo 50 link per risultato, e smette di aggiungere link una volta che l'array raggiunge 64 KiB di JSON serializzato. `resourceLinks` richiede Agent SDK v0.3.257 o successivo.

Imposta `inline_pastes` per dire a Claude Code quali parti di `message.content` l'utente ha incollato piuttosto che digitato, una stringa per incolla. Il testo del prompt rimane dove l'utente lo ha messo. Claude Code può avvolgere ogni incolla elencata in tag `<pasted_content>` dove si trova, in modo che Claude possa distinguere il materiale incollato dalle parole dell'utente. Solo gli incolla nell'ultimo blocco di testo del prompt vengono avvolti. Richiede TypeScript Agent SDK v0.3.280 o successivo.

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
      user_message_uuid?: string;
      user_message_uuids?: string[];
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
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
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

Diversi campi sul risultato contengono dettagli diagnostici oltre a `subtype`:

* `api_error_status`: il codice di stato HTTP dell'errore API che ha terminato la conversazione. Assente o `null` quando il turno è terminato senza un errore API.
* `ttft_ms`: tempo al primo token in millisecondi, misurato quando arriva il primo messaggio dell'assistente completo. Presente solo sul ramo di successo.
* `ttft_stream_ms`: tempo in millisecondi fino al primo evento di flusso `message_start`, quando il flusso di risposta si apre. Inferiore a `ttft_ms`; il divario tra i due è il tempo impiegato per lo streaming del primo messaggio. Presente solo sul ramo di successo.
* `user_message_uuid`: l'`uuid` del messaggio che hai inviato a cui questo turno ha risposto. Vedi [`user_message_uuid`](#user_message_uuid) per quali risultati lo contengono.
* `user_message_uuids`: gli `uuid` di ogni messaggio che hai inviato a cui Claude Code ha risposto in questo turno. Vedi [`user_message_uuids`](#user_message_uuids).
* `request_sent_wall_ms`: millisecondi di epoca in cui Claude Code ha inviato la richiesta API, per join rispetto ai timestamp lato server. Presente solo insieme a [`user_message_uuid`](#user_message_uuid), su un risultato di successo con `is_error` false il cui turno ha inviato una richiesta API.
* `first_content_frame_ms`: tempo in millisecondi fino al primo evento di flusso `content_block_start` o `content_block_delta`, contando i blocchi di pensiero come contenuto. Presente sul ramo di successo solo, quando `is_error` è false. Richiede Agent SDK v0.3.260 o successivo.
* `first_stream_post_ms`, `first_stream_post_ack_ms`, `first_stream_post_wall_ms`: tempi per il caricamento del primo evento di flusso del turno. Claude Code li registra solo nelle sessioni che trasmette a claude.ai, come [sessioni cloud](/docs/it/claude-code-on-the-web), e i risultati che `query()` produce non li contengono. Richiede Agent SDK v0.3.260 o successivo.
* `usage`: solo ciclo agente principale. Esclude le chiamate di subagente e modello ausiliario, ed è per turno nelle sessioni di input in streaming. Preferisci `modelUsage` per la contabilità di token/costo.
* `modelUsage`: totali per modello per ogni chiamata di modello effettuata attraverso la pipeline di query durante questa chiamata `query()`, incluso il ciclo principale, i subagenti e le chiamate interne come la compattazione e gli agenti Workflow. Le chiamate helper al di fuori di quella pipeline, come il classificatore di autorizzazione e le richieste di conteggio dei token, sono escluse. Una chiamata che riprende una sessione conta anche i [totali per modello ripristinati dalle chiamate precedenti della sessione](/docs/it/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Nelle sessioni di input in streaming i totali sono cumulativi tra i turni, quindi leggi il risultato più recente piuttosto che sommare tra i risultati. Vedi [Traccia i costi in modalità input in streaming](/docs/it/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) per i reset e [Recupera i totali dopo un arresto anomalo della sessione](/docs/it/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) per i risultati azzerati.
* `total_cost_usd`: costo stimato cumulativo in USD, coprendo le stesse chiamate di `modelUsage` e reset negli stessi punti. Una chiamata che riprende una sessione conta anche i [totali ripristinati dalle chiamate precedenti della sessione](/docs/it/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). È una stima, non un estratto conto di fatturazione. Vedi [Traccia costo e utilizzo](/docs/it/agent-sdk/cost-tracking) per le avvertenze di accuratezza.
* `queued_turn_count`: il numero di messaggi che hai inviato con `origin: { kind: "human" }` che sono ancora in attesa quando Claude Code ha prodotto il risultato. Vedi [`queued_turn_count`](#queued_turn_count) per cosa significano `0` e un campo assente.
* `startup_failure_reason`: il motivo per cui Claude Code ha rifiutato di avviarsi, sul risultato `error_during_execution` che scrive prima di uscire su un errore di avvio noto. Vedi [`startup_failure_reason`](#startup_failure_reason) per i valori e quali errori lo contengono. Richiede Agent SDK v0.3.274 o successivo.
* `terminal_reason`: il motivo per cui il ciclo è terminato. Uno di `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, o `"turn_setup_failed"`.
* `fast_mode_state`: uno di `"on"`, `"off"`, o `"cooldown"`.
* `fast_mode_disabled_reason`: il motivo per cui [fast mode](/docs/it/fast-mode) non è disponibile in questo momento. Assente quando nulla blocca fast mode, anche se una richiesta potrebbe comunque essere eseguita a velocità standard. Durante il cooldown dopo un limite di velocità fast mode, Claude Code segnala `fast_mode_state: "cooldown"` senza codice di motivo e riabilita fast mode quando il cooldown scade. Richiede Claude Code v2.1.219 o successivo.

Usa il codice di motivo per spiegare perché fast mode è disattivato nella tua interfaccia utente invece di derivare nuovamente la disponibilità. Ogni codice nomina il controllo che ha bloccato fast mode:

| Codice di motivo       | Significato                                                                                                                                                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | L'account non ha l'abbonamento a pagamento o i crediti di utilizzo che fast mode richiede                                                                     |
| `preference`           | L'organizzazione ha disabilitato fast mode                                                                                                                    |
| `extra_usage_disabled` | I crediti di utilizzo sono disattivati per l'account                                                                                                          |
| `network_error`        | Il [controllo di disponibilità](/docs/it/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) non ha potuto raggiungere `api.anthropic.com`                    |
| `unknown`              | Claude Code non ha potuto determinare la disponibilità                                                                                                        |
| `not_first_party`      | La sessione utilizza un provider diverso dall'API Anthropic                                                                                                   |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/it/env-vars) è impostato                                                                                                   |
| `model_not_allowed`    | Il modello Opus fast mode non è nella lista di autorizzazione [`availableModels`](/docs/it/model-config#restrict-model-selection) dell'organizzazione              |
| `sdk_opt_in_required`  | La sessione non ha acconsentito a fast mode: passa `fastMode: true` nell'opzione [`settings`](#options) o tramite [`applyFlagSettings()`](#applyflagsettings) |
| `pending`              | Il controllo di disponibilità non è ancora completato                                                                                                         |

La stessa coppia di campi appare su [`SDKSystemMessage`](#sdksystemmessage) e su [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse), quindi puoi leggere lo stato fast mode prima del primo turno.

Il campo `origin` inoltro l'[`SDKMessageOrigin`](#sdkmessageorigin) del messaggio utente che ha attivato questo risultato. Quando l'SDK inietta un turno di follow-up sintetico, come per un'attività finita in background, il `SDKResultMessage` risultante contiene `origin: { kind: "task-notification" }`. Le routine il cui trigger si è attivato e i messaggi verificati dal server dalle tue altre sessioni arrivano con questo tipo anche, ciascuno con il `subkind` descritto in [Subkind di notifica attività](#task-notification-subkinds). Controlla `kind` per distinguere i risultati che rispondono al tuo prompt dai follow-up iniettati prima di instradare o sopprimere questi ultimi. Se la tua applicazione [dichiara esecuzioni programmate](#declare-a-scheduled-run), i loro risultati contengono `kind: "task-notification"` anche, quindi non sopprimere solo su `kind`.

Quando più completamenti di attività in background sono in coda insieme, Claude Code può rispondere a loro in un turno piuttosto che uno turno ciascuno. Ogni completamento produce comunque il suo risultato con questa origine. Tutti tranne l'ultimo dei completamenti a cui Claude Code risponde insieme producono risultati vuoti con `num_turns: 0`, in ordine, e il risultato dell'ultimo contiene il turno che risponde a tutti loro.

Il campo è assente per i risultati emessi prima di qualsiasi turno utente, come gli errori di avvio.

Quando un hook `PreToolUse` restituisce `permissionDecision: "defer"`, il risultato ha `stop_reason: "tool_deferred"` e `deferred_tool_use` contiene l'`id`, il `name` e l'`input` del tool in sospeso. Leggi questo campo per visualizzare la richiesta nella tua interfaccia utente, quindi riprendi con lo stesso `session_id` per continuare. Vedi [Rinvia una chiamata di tool per dopo](/docs/it/hooks#defer-a-tool-call-for-later) per il percorso completo.

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

L'`uuid` del [`SDKUserMessage`](#sdkusermessage) a cui il turno sta rispondendo, ripetuto in modo da poter abbinare la risposta di Claude Code al messaggio che hai inviato. Claude Code ripete un `uuid` solo se ne hai impostato uno sul messaggio. Il campo è facoltativo su `SDKUserMessage`, e un prompt di stringa passato a `query()` non ne contiene nessuno.

Quale dei tuoi messaggi un turno risponde dipende da come il turno è iniziato:

* **Un messaggio regolare che hai inviato**, cioè uno senza `isSynthetic: true`: il turno risponde a quel messaggio per tutta la sua esecuzione. Quando invii diversi messaggi uno dopo l'altro, Claude Code può unirli in un turno, e il campo contiene solo l'`uuid` dell'ultimo messaggio. Per abbinare la risposta a uno qualsiasi dei messaggi uniti, usa [`user_message_uuids`](#user_message_uuids).
* **Un messaggio che hai inviato con `isSynthetic: true`**: il turno risponde a quel messaggio all'inizio. Se Claude Code raccoglie un messaggio regolare tuo tra le chiamate di tool, il turno risponde al messaggio raccolto da allora in poi. L'eco di un `uuid` di messaggio sintetico richiede Agent SDK v0.3.265 o successivo; le versioni precedenti non ecolano nulla sui turni sintetici.
* **Un prompt che Claude Code ha generato da solo**, come il turno che continua il lavoro interrotto dopo il riavvio di una sessione: il turno non risponde a nessun messaggio tuo all'inizio e i suoi frame non contengono alcun eco. Se Claude Code raccoglie un messaggio regolare tuo tra le chiamate di tool, il turno risponde a quel messaggio da allora in poi. L'eco di raccolta richiede Agent SDK v0.3.265 o successivo; le versioni precedenti non ecolano nulla su questi turni.

Claude Code ripete l'`uuid` del messaggio a cui ha risposto su tre tipi di frame:

* **Il risultato**: ogni risultato di un turno che ha risposto a un messaggio che hai inviato. Ogni tale risultato lo contiene su Agent SDK v0.3.265 o successivo. Prima della v0.3.265, il risultato di successo di un turno che un messaggio regolare ha avviato non lo conteneva quando il turno non ha inviato alcuna richiesta API o è terminato con una chiamata di tool differita. Prima della v0.3.246, anche i risultati di errore non lo contenevano, e prima della v0.3.216 ogni risultato non lo conteneva.
* **La prima risposta del turno**: il primo [messaggio dell'assistente](#sdkassistantmessage), o con `includePartialMessages` il primo [evento di flusso](#sdkpartialassistantmessage) il cui `event.type` non è `ping`, in modo da poter associare la risposta prima che il risultato arrivi. Quando un turno non trasmette nulla, Claude Code lo imposta sul primo messaggio dell'assistente. L'eco della prima risposta richiede Agent SDK v0.3.246 o successivo. Quando il messaggio a cui il turno sta rispondendo cambia a metà turno, la prima risposta dopo il cambio contiene il campo anche, su Agent SDK v0.3.265 o successivo; le versioni precedenti lo impostano su un frame di risposta per turno.
* **Ogni frame [`thinking_tokens`](#sdkthinkingtokensmessage) del turno**: in modo da poter attribuire il progresso del pensiero al messaggio che hai inviato senza aspettare la prima risposta del turno. Richiede Agent SDK v0.3.260 o successivo.

Claude Code omette il campo in questi casi:

* Frame di risposta diversi da quelle prime risposte
* Frame di subagente
* Turni che non rispondono a nessun messaggio con un `uuid`: il turno ha risposto a un messaggio che hai inviato senza uno, o Claude Code ha avviato il turno stesso e non ha raccolto nessun messaggio regolare che ne ha uno
* Risultati che non rispondono a nessun messaggio che hai inviato, come il risultato azzerato dopo un arresto anomalo del processo worker

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

Gli `uuid` di ogni messaggio che hai inviato a cui Claude Code ha risposto in questo turno. Quando invii diversi messaggi uno dopo l'altro, Claude Code può unirli in un turno, e `user_message_uuid` nomina solo l'ultimo di essi. Per abbinare la risposta a uno qualsiasi dei messaggi uniti, cerca l'`uuid` di quel messaggio ovunque in questo elenco. Richiede Agent SDK v0.3.259 o successivo.

Claude Code imposta l'elenco insieme a `user_message_uuid` su ogni frame di risposta che contiene quel campo e sul risultato. Per l'insieme completo di frame che contengono `user_message_uuid`, e la versione che ciascuno richiede, vedi [`user_message_uuid`](#user_message_uuid). L'elenco contiene sempre `user_message_uuid` e contiene al massimo 64 voci.

Quando Claude Code raccoglie un messaggio regolare che hai inviato mentre un turno era in esecuzione, aggiunge l'`uuid` di quel messaggio all'elenco del risultato.

Quando una prima risposta o un risultato contiene `user_message_uuid` senza l'elenco, proviene da una versione precedente di Claude Code, quindi ricadi al campo singolo.

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

Il numero di messaggi che hai inviato con [`origin: { kind: "human" }`](#sdkmessageorigin) che sono ancora in attesa nella coda di comando quando Claude Code ha prodotto il risultato. Richiede Agent SDK v0.3.242 o successivo.

Cosa significano `0` e un campo assente:

* **`0`**: Claude Code non conta i messaggi che hai inviato senza quel `origin`, e non conta le notifiche di attività, quindi un turno può comunque seguire.
* **Assente**: il risultato finale che Claude Code emette dopo un arresto anomalo o un errore di avvio fatale omette il campo, e [può contenere totali azzerati](/docs/it/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Il motivo per cui Claude Code ha rifiutato di avviarsi, in modo che la tua applicazione possa offrire la correzione invece di un nuovo tentativo. Claude Code lo imposta sul risultato `error_during_execution` che scrive prima di uscire su un errore di avvio noto. Quel risultato contiene totali azzerati, e il suo array `errors` contiene lo stesso testo di stderr. Il campo è assente su ogni altro risultato. Richiede Agent SDK v0.3.274 o successivo.

Imposta `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` a `1` in [`env`](#options) per ricevere questo risultato per ogni valore `SDKStartupFailureReason`. Senza quella variabile, Claude Code scrive il risultato solo per questi errori, e il resto termina con output stderr, un'uscita non zero, e nessun messaggio di risultato:

* Un resume che Claude Code interrompe perché non può [restituire la sessione al suo worktree](/docs/it/worktrees#the-session-resumes-outside-its-worktree), con `worktree_unverified` o `worktree_resume_refused`. Quella sezione dice quale errore contiene quale valore.
* Un [`continue`](#options) rifiutato di una conversazione che una sessione in background contiene, con `session_held_by_background`. Per un [`resume`](#options) rifiutato di tale conversazione, Claude Code scrive il risultato solo quando la variabile è impostata.

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root";
```

Ogni valore nomina un rifiuto:

| Valore                                 | Cosa ha interrotto la sessione                                                                                                                                                                                                  |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `org_pin_api_key_conflict`             | Le impostazioni gestite [richiedono un accesso gateway cloud o first-party](/docs/it/authentication#restrict-login-to-your-organization), e una chiave API Anthropic, token di autenticazione, o `apiKeyHelper` è configurato invece |
| `org_verify_failed`                    | L'organizzazione dell'accesso non ha potuto essere verificata rispetto al pin, ad esempio a causa di un errore di rete o un token revocato                                                                                      |
| `org_pin_mismatch`                     | L'accesso appartiene a un'organizzazione che il pin non consente                                                                                                                                                                |
| `managed_settings_invalid`             | Le impostazioni della politica gestita non hanno potuto essere lette, o il pin non nomina alcuna organizzazione                                                                                                                 |
| `remote_settings_required_unavailable` | Le impostazioni gestite che l'organizzazione richiede non hanno potuto essere caricate                                                                                                                                          |
| `gateway_signin_required`              | Il [gateway cloud](/docs/it/claude-apps-gateway) ha terminato questo accesso                                                                                                                                                         |
| `gateway_access_denied`                | La richiesta di impostazioni gestite al gateway cloud è tornata con un 403, che la [tabella di risoluzione dei problemi](/docs/it/claude-apps-gateway-deploy#troubleshooting) del gateway copre                                      |
| `proxy_invalid`                        | Un'impostazione proxy non è un URL completo                                                                                                                                                                                     |
| `temp_dir_unusable`                    | La directory temporanea per utente non è sicura o non ha potuto essere creata                                                                                                                                                   |
| `cwd_unavailable`                      | La directory di lavoro è stata eliminata, spostata, o non può essere letta                                                                                                                                                      |
| `shell_tool_missing`                   | Su Windows, nessuno strumento shell è disponibile: Git Bash manca, e PowerShell manca o è disattivato con `CLAUDE_CODE_USE_POWERSHELL_TOOL`                                                                                     |
| `session_held_by_background`           | La conversazione da riprendere o continuare è in esecuzione come una [sessione in background](/docs/it/agent-view)                                                                                                                   |
| `worktree_resume_refused`              | Il worktree della sessione ha fallito i suoi controlli di sicurezza, o il resume è stato lanciato dall'interno di esso. `errors` dice se l'esecuzione dello stesso resume di nuovo continua senza il worktree                   |
| `worktree_unverified`                  | Il worktree della sessione non ha potuto essere verificato in questo momento, e il nuovo tentativo potrebbe avere successo                                                                                                      |
| `cli_version_too_old`                  | Questa versione di Claude Code è al di sotto del minimo che Anthropic richiede                                                                                                                                                  |
| `bypass_root`                          | La modalità di autorizzazione bypass è stata richiesta mentre si esegue come root                                                                                                                                               |

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
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state` segnala lo stato [fast mode](/docs/it/fast-mode) della sessione. Quando qualcosa blocca fast mode, `fast_mode_disabled_reason` nomina il controllo che lo ha bloccato; il campo richiede Claude Code v2.1.219 o successivo. Per i codici di motivo e i loro significati, vedi [`fast_mode_disabled_reason`](#sdkresultmessage) sul messaggio di risultato.

`terminal_slash_commands` nomina le voci in `slash_commands` la cui interfaccia è associata al terminale locale, come `exit`. Puoi inviarle come qualsiasi altra voce in `slash_commands`; il campo esiste in modo che un client remoto o mobile possa nasconderle dai suoi menu di comando. Il campo è presente solo quando non vuoto, e richiede Agent SDK v0.3.229 o successivo.

*

`source` su ogni voce `mcp_servers`: da dove proviene la definizione del server, con gli stessi valori di [`McpServerStatus`](#mcpserverstatus)'s `source`. Richiede Agent SDK v0.3.274 o successivo.

*

`effort`: il [livello di sforzo](/docs/it/model-config#adjust-effort-level) che Claude Code invia sulla prossima richiesta della sessione, o `null` quando non ne invia nessuno. Claude Code imposta il campo solo sul messaggio di init che invia ai client [Remote Control](/docs/it/remote-control), e lo omette dal messaggio di init che la tua applicazione legge. Richiede Agent SDK v0.3.234 o successivo.

L'array `capabilities` nomina i comportamenti del protocollo che questa CLI implementa, in modo da poter rilevare le funzionalità invece di confrontare le stringhe `claude_code_version`. È un insieme aperto: ignora i valori che non riconosci, e controlla la capacità specifica su cui fai affidamento. Il campo richiede Claude Code v2.1.205 o successivo ed è assente su CLI precedenti.

| Capacità                     | Significato                                                                                                                                                                                                                                                                                                  |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `interrupt_receipt_v1`       | [`interrupt()`](#query-object) si risolve con una ricevuta [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) che nomina i messaggi che erano in sospeso quando l'interruzione è arrivata                                                                                                         |
| `interrupt_cancel_queued_v1` | La richiesta di controllo `interrupt` onora `cancel_queued: true`, annullando i messaggi che la ricevuta altrimenti elencherebbe sotto `still_queued` e elencandoli sotto `cancelled` invece. Vedi [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse). Richiede Claude Code v2.1.219 o successivo |

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
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code imposta `user_message_uuid` e `user_message_uuids` sul primo evento di flusso non-ping del turno, e di nuovo quando il messaggio a cui il turno sta rispondendo cambia, secondo le condizioni in [`user_message_uuid`](#user_message_uuid).

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

Banner di testo generico emesso dal ciclo. Contiene righe di stato non di errore, feedback di hook come il motivo del blocco di un hook `UserPromptSubmit`, e output di comando. Su Claude Code v2.1.227 o successivo, il [`systemMessage`](/docs/it/hooks#json-output) di un hook può arrivare come questo messaggio, con ogni riga prefissata dal nome dell'hook, come `PostToolUse:Bash says:`. Se il `systemMessage` di un hook arriva come questo messaggio dipende dall'evento. Ogni [sezione dell'evento](/docs/it/hooks#hook-events) sulla pagina dei hook dice come l'output viene visualizzato. Renderizza `content` come testo semplice al livello specificato.

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

Evento di flusso emesso quando il sistema di autorizzazione nega una chiamata di tool senza un prompt interattivo. Usalo per rendere il rifiuto nella tua interfaccia utente mentre accade, piuttosto che osservare solo il risultato del tool `is_error` che segue. Quali rifiuti segnala dipende da come l'esecuzione gestisce i prompt di autorizzazione:

* **Con un callback [`canUseTool`](#canusetool) e il [`permissionPrompts: 'host'`](#options) predefinito**: i prompt di autorizzazione vanno al tuo callback, e questo evento segnala i rifiuti che Claude Code decide da solo senza chiamarlo.
* **Con nessuno dei due**: un'esecuzione `-p` nuda, o `query()` che non imposta né `canUseTool` né `permissionPromptToolName`, nega qualsiasi chiamata di tool che avrebbe richiesto un prompt, e questo evento segnala anche quei rifiuti oltre a quelli che Claude Code decide da solo. Prima della v2.1.223, Claude Code non emetteva questo evento nelle esecuzioni senza un callback.
* **Con uno strumento di prompt MCP**, impostato con `permissionPromptToolName` o il flag [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags), e il `permissionPrompts: 'host'` predefinito: Claude Code non emette questo evento affatto, nemmeno per i rifiuti di regola che decide da solo.
* **Con [`permissionPrompts: 'none'`](#options)**: Claude Code nega le chiamate che avrebbero richiesto un prompt, anche quando `canUseTool` o uno strumento di prompt MCP è anche impostato, e questo evento segnala anche quei rifiuti oltre a quelli che Claude Code decide da solo. Richiede Claude Code v2.1.259 o successivo.

In ogni configurazione, questo evento salta qualsiasi rifiuto deciso sul percorso dell'hook `PreToolUse`, indipendentemente dal fatto che l'hook abbia negato la chiamata stessa o una regola di negazione abbia sovrascritto la decisione di consentire o chiedere dell'hook. L'evento è anche best-effort: occasionalmente Claude Code registra un rifiuto senza emettere questo evento, quindi `permission_denials` sul [messaggio di risultato](#sdkresultmessage) è il record autorevole.

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

<h3 id="sdkcontextusage">
  `SDKContextUsage`
</h3>

Forma strutturata del rapporto `/context`, portata come `context_usage` sul [`SDKAssistantMessage`](#sdkassistantmessage) che consegna un risultato `/context`. Agent SDK v0.3.232 e successivo esportano il tipo. A differenza di [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse), contiene solo i dati necessari per rendere la suddivisione dell'utilizzo, senza campi di visualizzazione come `color` e `gridRows`.

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

La tabella elenca cosa Claude Code mette in ogni campo. I campi da `model` a `over_limit` descrivono la sessione nel suo insieme, e i campi di raccolta attribuiscono i token a elementi individuali.

| Campo            | Tipo                                                      | Descrizione                                                                                                                                                                                                                                                                                                                                          |
| ---------------- | --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | Il modello del ciclo principale per il quale Claude Code ha calcolato l'utilizzo, non quello di un subagente                                                                                                                                                                                                                                         |
| `total_tokens`   | `number`                                                  | La stima di Claude Code dei token in uso. Non limitato alla finestra, quindi può superare `raw_max_tokens` quando la sessione è oltre il limite                                                                                                                                                                                                      |
| `raw_max_tokens` | `number`                                                  | La finestra di contesto del modello, o la [finestra di auto-compattazione](/docs/it/model-config#context-window-and-auto-compaction) inferiore quando una si applica, come una che hai impostato o il limite di 200K che Claude Code applica ad alcuni modelli con una finestra di 1M token. Claude Code misura `total_tokens` rispetto a questa finestra |
| `percentage`     | `number`                                                  | `total_tokens` come percentuale arrotondata di `raw_max_tokens`, quindi può superare 100 quando la sessione è oltre il limite                                                                                                                                                                                                                        |
| `over_limit`     | `object`                                                  | Presente solo quando `total_tokens` supera `raw_max_tokens`. `tokens_over` è l'importo in eccesso, e `kind` dice come Claude Code ha risolto la finestra                                                                                                                                                                                             |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | Una voce per riga della suddivisione dell'utilizzo per categoria                                                                                                                                                                                                                                                                                     |
| `mcp_tools`      | `object[]`                                                | Token attribuiti a ogni tool MCP, con il suo nome di trasmissione, come `mcp__linear__create_issue`, e il suo `server_name`                                                                                                                                                                                                                          |
| `memory_files`   | `object[]`                                                | Token attribuiti a ogni file di memoria caricato, con il suo `path` e un'etichetta di origine come `Project` o `User` in `type`                                                                                                                                                                                                                      |
| `agents`         | `object[]`                                                | Token attribuiti a ogni definizione di subagente personalizzato, con un identificatore di origine come `projectSettings`, `userSettings`, o `plugin`. I subagenti integrati non sono elencati                                                                                                                                                        |
| `skills`         | `object[]`                                                | Token attribuiti a ogni skill nell'elenco di skill, con un identificatore di origine e, per le skill di plugin, il nome del plugin in `plugin_name`. Assente quando nessuna skill contribuisce token                                                                                                                                                 |

`over_limit.kind` registra come Claude Code ha risolto la finestra, non se l'API accetta la prossima richiesta:

* `hard_limit`: la finestra è quella che Claude Code crede sia il limite proprio del modello, oltre il quale l'API rifiuta le richieste
* `compaction_window`: la finestra è una finestra di politica di compattazione, che può o non può coincidere con il limite del modello

Claude Code evolve il tipo in modo additivo, aggiungendo nuovi dati come campi facoltativi piuttosto che rimodellando quelli esistenti. Leggi i campi che conosci e ignora quelli che non riconosci.

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

Una riga della suddivisione dell'utilizzo `/context` per categoria.

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

La tabella elenca cosa Claude Code mette in ogni campo di una riga.

| Campo    | Tipo     | Descrizione                                                                                                                    |
| -------- | -------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `name`   | `string` | Il nome di visualizzazione della riga come `/context` lo stampa, come `Messages`. Classifica le righe per `kind`, non per nome |
| `tokens` | `number` | Il conteggio dei token della riga. Le righe possono contenere zero token                                                       |
| `kind`   | `string` | Cosa rappresenta la riga: `used`, `free`, `buffer`, o `deferred`                                                               |

Ogni valore `kind` dice cosa sono i token della riga:

* `used`: contenuto che occupa la finestra di contesto
* `free`: la finestra rimanente
* `buffer`: la riserva di compattazione
* `deferred`: schemi di tool che Claude Code tiene fuori dalla finestra ed esclude dal calcolo dell'utilizzo, elencati per consapevolezza

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
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
      fireReason?: string;
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind`              | Significato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Input diretto dall'utente finale. Se la tua applicazione inoltro quello che l'utente ha digitato come messaggio utente, imposta il suo `origin` a `{ kind: "human" }` esplicitamente: Claude Code tratta un messaggio utente senza `origin` come non attribuito, e controlla che richiedono un prompt digitato dall'utente, come la [parola chiave del workflow `ultracode`](/docs/it/workflows#ask-for-a-workflow-in-your-prompt), non lo accettano. Prima della v2.1.210, Claude Code trattava un `origin` assente su un messaggio utente come input umano. |
| `channel`           | Messaggio in arrivo su un [canale](/docs/it/channels). `server` è il nome del server MCP di origine.                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `peer`              | Messaggio da un altro agente: un [collega](/docs/it/agent-teams) in-process o un [peer tra sessioni](/docs/it/cross-session-messaging), un'altra delle tue sessioni Claude Code. Vedi [Campi di origine peer](#peer-origin-fields) per la semantica per campo e il modello di fiducia.                                                                                                                                                                                                                                                                             |
| `task-notification` | Turno sintetico iniettato per una consegna che arriva senza un prompt utente fresco, come un'attività finita in background; vedi [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) per quel ramo. Un prompt che la tua applicazione [dichiara come esecuzione programmata](#declare-a-scheduled-run) contiene questo tipo anche. Il `subkind` facoltativo marca cosa ha sollevato la notifica. Vedi [Subkind di notifica attività](#task-notification-subkinds).                                                                               |
| `coordinator`       | Messaggio da un coordinatore di team in un [team di agenti](/docs/it/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `auto-continuation` | Turno sintetico iniettato quando la sessione continua senza input utente fresco, come un risultato di comando che attiva un prompt di follow-up.                                                                                                                                                                                                                                                                                                                                                                                                         |
| `unclassified`      | Turno iniettato il cui origin non ha potuto essere determinato. Richiede Claude Code v2.1.223 o successivo. Quando Claude Code riceve un [`SDKUserMessage`](#sdkusermessage) con `isSynthetic: true` e non può classificarlo come nessun altro `kind`, imposta questo kind mentre il messaggio arriva e inquadra il turno al modello come una fonte non utente piuttosto che trattarlo come input umano. La tua applicazione non dovrebbe impostare questo valore.                                                                                       |

<h3 id="task-notification-subkinds">
  Task-notification subkinds
</h3>

Quando Claude Code consegna una notifica di attività in una sessione, imposta `subkind` sull'`origin` della notifica se i server Anthropic hanno verificato da dove proveniva quella notifica. Imposta anche `subkind` quando la tua applicazione [dichiara il messaggio come esecuzione programmata](#declare-a-scheduled-run) stessa, che richiede TypeScript Agent SDK v0.3.280 o successivo. `subkind` richiede Claude Code v2.1.213 o successivo, e assume uno di due valori:

* `scheduled-trigger`: la notifica è il prompt memorizzato di una [routine](/docs/it/routines), consegnato perché uno dei trigger della routine si è attivato: il suo programma, il suo [trigger API](/docs/it/routines#add-an-api-trigger), il suo [trigger GitHub](/docs/it/routines#add-a-github-trigger), o **Esegui ora**. Un prompt che la tua applicazione [dichiara come esecuzione programmata](#declare-a-scheduled-run) contiene questo valore anche. Claude Code inquadra questi al modello come l'attività assegnata della sessione, con un avviso diverso dall'[avviso che altre notifiche di attività contengono](#sdktasknotificationmessage).
*

`peer-send-message`: la notifica è un messaggio che un'altra delle tue sessioni ha inviato con lo strumento `send_message` lato server che le sessioni [Claude Code sul web](/docs/it/claude-code-on-the-web) usano per messaggiarsi l'una con l'altra, non lo [strumento `SendMessage` tra sessioni](/docs/it/cross-session-messaging), e i server Anthropic hanno verificato che entrambe le sessioni appartengono allo stesso gruppo privato di sessioni. Richiede Claude Code v2.1.224 o successivo. Una consegna `send_message` che i server non hanno verificato in quel modo non ha `subkind`.

Ogni altra notifica di attività non ha `subkind`. Questo include [attività PR](/docs/it/claude-code-on-the-web#how-claude-responds-to-pr-activity) consegnate in una sessione e eventi in background come un'attività finita. I messaggi dallo strumento [`SendMessage`](/docs/it/cross-session-messaging) tra sessioni non sono notifiche di attività affatto: che provengano da una sessione sulla stessa macchina o attraverso i server Anthropic da un'altra macchina, Claude Code dà loro `kind: "peer"` e i [campi di origine peer](#peer-origin-fields).

`fireReason` dice perché una notifica `scheduled-trigger` si è attivata, come un token minuscolo breve come `scheduled`, `manual`, `retry`, `catch_up`, o `api`. I server Anthropic lo impostano sulle consegne di una [routine](/docs/it/routines), e la tua applicazione lo imposta quando dichiara un'esecuzione programmata. È assente quando nessuno dei due ne ha inviato uno. Richiede TypeScript Agent SDK v0.3.280 o successivo.

<h4 id="declare-a-scheduled-run">
  Declare a scheduled run
</h4>

Se la tua applicazione esegue prompt secondo il suo programma, dichiara ogni esecuzione in modo che Claude Code inquadri il turno al modello come un'attività programmata piuttosto che come input dal vivo dell'utente. Avvia la sessione con `CLAUDE_CODE_HOST_SCHEDULED_RUN` impostato a `1` in [`env`](#options), quindi invia il [`SDKUserMessage`](#sdkusermessage) dell'esecuzione con `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` e senza `isSynthetic`. Claude Code ignora la dichiarazione in un processo avviato senza quella variabile. Ignora anche in un processo il cui ambiente contiene [`CLAUDECODE`](/docs/it/env-vars) o `CLAUDE_CODE_CHILD_SESSION`. Claude Code mantiene `fireReason` solo quando il valore è 1 a 32 lettere minuscole o trattini bassi. Richiede TypeScript Agent SDK v0.3.280 o successivo.

<h3 id="peer-origin-fields">
  Peer origin fields
</h3>

Un'origine `peer` identifica quale agente ha inviato il messaggio: un [collega](/docs/it/agent-teams) in-process che invia a `main` con `SendMessage`, o un [peer tra sessioni](/docs/it/cross-session-messaging), un'altra delle tue sessioni Claude Code. I peer tra sessioni richiedono Claude Code v2.1.224 o successivo su macOS e Linux; vedi [disponibilità di messaggistica tra sessioni](/docs/it/cross-session-messaging#availability) per il requisito di Windows nativo. Un peer tra sessioni può essere eseguito sulla stessa macchina, o su [un'altra delle tue macchine](/docs/it/cross-session-messaging#message-sessions-on-other-machines) o [Claude Code sul web](/docs/it/claude-code-on-the-web) quando il suo messaggio arriva attraverso Remote Control. I due tipi di mittente riempiono i campi diversamente:

* `from`: il nome del collega, o l'indirizzo del mittente per un peer tra sessioni. Per un [messaggio tra macchine unidirezionale](/docs/it/cross-session-messaging#message-sessions-on-other-machines), il mittente non ha indirizzo di risposta e `from` è `"unknown"`. Il valore è creato dal mittente; `verifiedPeerPid` è l'identità verificata.
*

`fromMode`: la classe di autorizzazione della sessione di invio, `bypass` o `prompting`, dichiarata da un host che inoltro un messaggio peer tra le tue sessioni, come l'[app desktop](/docs/it/desktop#work-across-sessions). Claude Code lo legge nella sessione ricevente quando applica i [controlli in entrata](/docs/it/cross-session-messaging#control-inbound-messages). Richiede Agent SDK v0.3.234 o successivo.

* `senderTaskId`: l'ID attività del collega. Assente per un peer tra sessioni.
*

`name`: il nome di visualizzazione del mittente, normalizzato da Claude Code: rimuove i punti di codice di controllo, formato, surrogato e separatore di riga o paragrafo Unicode, quindi taglia il risultato e lo limita a 64 punti di codice con un'ellissi. Richiede Claude Code v2.1.205 o successivo.

*

`body`: il corpo del messaggio decodificato con l'involucro peer rimosso, byte-esatto con quello che il modello vede. Sempre presente per un messaggio di collega; per un peer tra sessioni, presente solo quando il turno è esattamente un involucro peer formato da Claude Code. Renderizza `name` e `body` invece di ri-analizzare il testo del messaggio. Richiede Claude Code v2.1.205 o successivo.

*

`fromSession`: l'ID sessione del mittente apribile dall'host, impostato dall'host del mittente in modo che la tua interfaccia utente possa collegarsi di nuovo alla sessione di invio. Come `from`, è asserito dal mittente: usalo solo come destinazione di navigazione, e non trattarlo come prova dell'identità del mittente. Richiede Claude Code v2.1.216 o successivo.

*

`verifiedPeerPid`: l'ID del processo del processo che si è connesso al socket di messaggistica tra sessioni di questa sessione, verificato dal kernel e letto dalla connessione stessa, mai dal payload. Usalo, non `from`, per identificare il mittente: `from` è falsificabile da qualsiasi processo dello stesso utente. Il campo è assente quando Claude Code non può verificarlo, come su Windows o ingresso non-socket, quindi un valore assente significa che il mittente non è verificato. Per il traffico inoltrato identifica l'inoltro piuttosto che l'autore del messaggio, e gli ID di processo sono riciclabili, quindi trattalo come provenienza piuttosto che come token di autenticazione. Richiede Claude Code v2.1.216 o successivo.

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
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
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
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
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
  mcp_server?: McpServerProvenance;
};
```

`mcp_server` è presente quando lo strumento proviene da un server MCP; vedi [`McpServerProvenance`](#mcpserverprovenance). Gli input `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` e `PermissionDenied` portano lo stesso campo. Il campo richiede Agent SDK v0.3.274 o successivo.

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
  mcp_server?: McpServerProvenance;
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
  mcp_server?: McpServerProvenance;
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

<h4 id="permissiondeniedhookinput">
  `PermissionDeniedHookInput`
</h4>

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
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
  session_title?: string;
};
```

<h4 id="userpromptexpansionhookinput">
  `UserPromptExpansionHookInput`
</h4>

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
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

<h4 id="stopfailurehookinput">
  `StopFailureHookInput`
</h4>

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
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

<h4 id="postcompacthookinput">
  `PostCompactHookInput`
</h4>

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

<h4 id="premodelswitchhookinput">
  `PreModelSwitchHookInput`
</h4>

Si attiva prima che un cambio di modello richiesto abbia effetto. `context_tokens` e i campi dopo di esso stimano il costo di reinviare la conversazione al nuovo modello. Per le descrizioni complete dei campi e la semantica di blocco, vedi [PreModelSwitch](/docs/it/hooks#premodelswitch).

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="postmodelswitchhookinput">
  `PostModelSwitchHookInput`
</h4>

Si attiva dopo che il modello della sessione cambia. Contiene gli stessi campi di `PreModelSwitchHookInput`, con due valori `source` aggiuntivi. Vedi [PostModelSwitch](/docs/it/hooks#postmodelswitch).

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
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
  mcp_server?: McpServerProvenance;
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

<h4 id="taskcreatedhookinput">
  `TaskCreatedHookInput`
</h4>

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated da v2.1.178. Contiene il nome del team derivato dalla sessione; verrà rimosso. */
  team_name?: string;
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

<h4 id="elicitationhookinput">
  `ElicitationHookInput`
</h4>

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

<h4 id="elicitationresulthookinput">
  `ElicitationResultHookInput`
</h4>

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
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

<h4 id="instructionsloadedhookinput">
  `InstructionsLoadedHookInput`
</h4>

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

<h4 id="directoryaddedhookinput">
  `DirectoryAddedHookInput`
</h4>

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory` è il percorso assoluto della directory che è stata aggiunta. `source` è `"slash_command"` quando `/add-dir` l'ha aggiunta e `"register_repo_root"` quando la richiesta di controllo SDK l'ha fatto.

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

<h4 id="cwdchangedhookinput">
  `CwdChangedHookInput`
</h4>

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

<h4 id="filechangedhookinput">
  `FileChangedHookInput`
</h4>

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
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
  /**
   * Una sequenza di escape del terminale (ad es. OSC 9 / OSC 777 desktop-notification)
   * per Claude Code da emettere per tuo conto. Solo notification/title OSCs
   * (0, 1, 2, 9, 99, 777) e BEL sono consentiti; un valore contenente
   * qualcos'altro viene ignorato nel complesso. Solo la CLI interattiva lo emette;
   * l'SDK ignora il campo.
   */
  terminalSequence?: string;
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
        sessionTitle?: string;
        /** Quando decision è "block", ometti il prompt originale dal messaggio di blocco. */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * Riscannerizza le directory di skill e comandi dopo che gli hook SessionStart
         * sono completati, in modo che le skill installate dall'hook siano disponibili nella
         * stessa sessione.
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * Stesso contratto di PreToolUse: "allow" procede, "deny" annulla
         * il cambio, "ask" chiede all'utente di confermare. Solo /model in una
         * sessione interattiva mostra quel prompt; ogni altra superficie,
         * incluse le richieste set_model, tratta "ask" come un rifiuto.
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** Raggiunge il modello con la prossima richiesta che il nuovo modello serve. */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * Breve nota sul risultato di questa chiamata di strumento per il classificatore
         * di autorizzazione in modalità auto. Limitato a 2000 caratteri, condiviso tra
         * tutti gli hook che rispondono alla stessa chiamata; rispettato solo su
         * risposte di hook sincrone. Non copiare output di strumento non attendibile in esso.
         */
        classifierContext?: string;
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
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
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
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** Testo visualizzato al posto del delta. Ometti (o restituisci il delta invariato) per visualizzare l'originale. */
        displayContent?: string;
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

Unione di tipi di input dei tool esportati da `@anthropic-ai/claude-agent-sdk`; i membri includono:

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nome del tool:** `Agent`. Il nome precedente `Task` è ancora accettato come alias, e l'array `tools` nel messaggio di inizializzazione [`SDKSystemMessage`](#sdksystemmessage) attualmente elenca questo tool come `Task` per compatibilità all'indietro.

<Note>
  Il campo `mode` è deprecato e ignorato su Claude Code v2.1.212 o successivo. Un subagente viene eseguito in modalità di permesso della sessione padre o della sua definizione [`permissionMode`](#agentdefinition), e le [regole di ereditarietà del subagente](/docs/it/agent-sdk/permissions#available-modes) decidono quale.
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // Deprecato; ignorato
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // Deprecato; ignorato. Le regole di ereditarietà del subagente decidono la modalità di permesso di un subagente
  isolation?: "worktree" | "remote";
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
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
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

Esegue comandi Bash con timeout opzionale ed esecuzione in background. La directory di lavoro persiste tra i comandi, inclusi i comandi eseguiti in turni successivi di una sessione multi-turno; lo stato della shell come le variabili di ambiente esportate non persiste. Per i limiti su quali cambiamenti di directory si mantengono, vedi [Cosa persiste tra i comandi](/docs/it/tools-reference#what-persists-between-commands).

<h3 id="monitor">
  Monitor
</h3>

**Nome del tool:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

Esegue una fonte di background e consegna ogni evento a Claude in modo che possa reagire senza polling: `command` esegue uno script e emette un evento per riga stdout, e `ws` apre un WebSocket e emette un evento per frame di testo. Fornisci esattamente uno tra `command` o `ws`. La fonte `ws` richiede Claude Code v2.1.195 o successivo.

`timeout_ms` è la scadenza del watch in millisecondi. Predefinito a 300000 e accetta valori fino a 3600000. La scadenza effettiva è al massimo 1800000, che è 30 minuti, quindi un valore accettato più grande viene accorciato a quello. Alla scadenza il watch termina e Claude riceve un avviso in modo che possa avviare un nuovo watch se ne ha ancora bisogno.

Il tipo esportato contrassegna `timeout_ms` come obbligatorio perché lo schema riempie il valore predefinito; una chiamata che lo omette convalida.

Quando Monitor esegue un comando, segue le stesse regole di permesso di Bash; un watch WebSocket richiede l'approvazione separatamente. Vedi il [riferimento del tool Monitor](/docs/it/tools-reference#monitor-tool) per il comportamento e la disponibilità del provider.

<h3 id="taskoutput">
  TaskOutput
</h3>

Rimosso in Claude Code v2.1.277, insieme al suo tipo `TaskOutputInput`. In precedenza recuperava l'output da un'attività di background in esecuzione o completata; Claude legge il file di output di un'attività di background con `Read` invece.

Una voce `disallowedTools` o una regola di negazione che ancora nomina `TaskOutput` viene ignorata senza un avviso.

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

Per un PDF, Claude riceve il contenuto del file all'interno del `tool_result` della chiamata Read. Una lettura che restituisce l'output `pdf` [output](#tool-output-types) contiene un blocco `text` di riepilogo seguito da un blocco `document`. Una che restituisce l'output `parts` contiene il blocco `text` di riepilogo seguito da un blocco per ogni pagina estratta: un blocco `image`, o un blocco `text` che nomina la pagina quando Claude Code non poteva renderla come immagine. Prima di Agent SDK v0.3.242, Claude Code consegnava il contenuto del file come messaggio `user` separato dopo il risultato del tool.

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
  "-o"?: boolean; // print only the matched parts of each line; requires output_mode: "content"
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
  args?: unknown; // any JSON value; the published typings render this as an object map
  resumeFromRunId?: string;
  title?: string; // ignored; the script's meta block sets the title
  description?: string; // ignored; the script's meta block sets the description
};
```

Esegue un [workflow dinamico](/docs/it/workflows): uno script che orchestra molti subagenti in background e restituisce un risultato consolidato. Il tool `Workflow` è disponibile in Agent SDK v0.3.149 e versioni successive. Almeno uno tra `script`, `name` o `scriptPath` è obbligatorio.

| Campo             | Tipo      | Descrizione                                                                                                                                                                                                                                                                                                                                                           |
| ----------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Script di workflow inline. Deve iniziare con `export const meta = { name, description }` come letterale, seguito dal corpo dello script usando `agent()`, `parallel()`, `pipeline()` e `phase()`. Un array `phases` facoltativo in `meta` raggruppa gli agenti sotto fasi denominate nella vista di progresso                                                         |
| `name`            | `string`  | Nome di un workflow incorporato o uno salvato in `.claude/workflows/`. Risolto in uno script                                                                                                                                                                                                                                                                          |
| `scriptPath`      | `string`  | Percorso a un file di script di workflow su disco. Ha la precedenza su `script` e `name`. Claude Code persiste ogni invocazione dello script e restituisce il percorso nel risultato, quindi puoi modificare quel file e reinvocare con lo stesso `scriptPath` per iterare                                                                                            |
| `args`            | `unknown` | Valore di input esposto allo script come `args` globale, per workflow denominati parametrizzati come una domanda di ricerca o un elenco di percorsi di file. Passa array e oggetti come valori JSON effettivi, non come stringa codificata in JSON                                                                                                                    |
| `resumeFromRunId` | `string`  | ID di esecuzione di una precedente invocazione di `Workflow` da riprendere. Le chiamate `agent()` completate con input invariati restituiscono solitamente risultati memorizzati nella cache; il resto viene eseguito live. [Riprendi dopo una pausa](/docs/it/workflows#resume-after-a-pause) copre quali chiamate completate vengono rieseguite. Solo la stessa sessione |
| `title`           | `string`  | Ignorato; il blocco `meta` dello script imposta il titolo                                                                                                                                                                                                                                                                                                             |
| `description`     | `string`  | Ignorato; il blocco `meta` dello script imposta la descrizione                                                                                                                                                                                                                                                                                                        |

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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Vedi [Disponibilità del modello](/docs/it/agent-sdk/todo-tracking#model-availability) per aderire.
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
  [k: string]: unknown;
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

<h3 id="exitworktree">
  ExitWorktree
</h3>

**Nome del tool:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

Esce dal worktree git corrente e ritorna alla directory di lavoro originale. L'azione `keep` lascia il worktree e il ramo su disco, mentre `remove` elimina entrambi. `discard_changes` deve essere `true` quando si rimuove un worktree che ha file non committati o commit non uniti.

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**Nome del tool:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

Entra in modalità di pianificazione, dove Claude ricerca e presenta un piano prima di apportare modifiche.

<h3 id="croncreate">
  CronCreate
</h3>

**Nome del tool:** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

Pianifica un prompt da eseguire su una pianificazione cron a 5 campi nell'ora locale. Imposta `recurring` a `false` per attivare una volta al prossimo match. I job sono scoped alla sessione per impostazione predefinita: avviare una conversazione fresca li cancella, e riprendere con `--resume` o `--continue` ripristina i job che non sono scaduti. Vedi [Attività pianificate](/docs/it/scheduled-tasks).

Impostare `durable` a `true` richiede la persistenza a `.claude/scheduled_tasks.json` in modo che il job sopravviva ai riavvii. La pianificazione duratura non è disponibile in ogni sessione: quando non lo è, Claude Code accetta `durable: true` ma crea il job solo per la sessione. Leggi il campo `durable` dell'output per vedere se il job è persistito.

<h3 id="crondelete">
  CronDelete
</h3>

**Nome del tool:** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

Elimina un job cron pianificato per l'ID restituito da `CronCreate`.

<h3 id="cronlist">
  CronList
</h3>

**Nome del tool:** `CronList`

```typescript theme={null}
type CronListInput = {};
```

Elenca i job cron pianificati: job durevoli da `.claude/scheduled_tasks.json` e job solo per la sessione dalla sessione corrente.

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**Nome del tool:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

Pianifica un wake-up una tantum che attiva il prompt dato dopo un ritardo. Questo tool supporta il comando `/loop` auto-paced. Il runtime limita `delaySeconds` tra 60 e 3600 secondi. I campi `delaySeconds`, `reason`, `prompt` e `noop` sono obbligatori a meno che `stop` non sia true. `noop: true` segnala un wake-up dove nulla è cambiato. Impostare `stop: true` annulla il wakeup in sospeso e termina il `/loop` auto-paced. Il campo `stop` richiede Claude Code v2.1.202 o successivo. Vedi la [riga ScheduleWakeup nel riferimento dei tool](/docs/it/tools-reference).

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**Nome del tool:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

Gestisce [Routine](/docs/it/routines), le esecuzioni Claude Code pianificate e attivate ospitate nel cloud. Questo tool supporta il comando `/schedule`. `trigger_id` è obbligatorio per le azioni `get`, `update`, `run` e `list_runs`. `body` è obbligatorio per `create`, `update` e `create_webhook_trigger`, e facoltativo per `run`.

`create_webhook_trigger` allega una fonte di evento a una routine esistente, come un [evento GitHub](/docs/it/routines#add-a-github-trigger) che lo attiva. Il `body` nomina la fonte, gli eventi e la routine da attivare. Richiede Claude Code v2.1.225 o successivo.

`list_runs` elenca le esecuzioni recenti di una routine, e `get_run_log` legge il log di un'esecuzione. `session_id` nomina l'esecuzione da leggere, da un risultato `list_runs`, e `cursor` pagina attraverso i risultati di entrambe le azioni. Entrambe le azioni richiedono Claude Code v2.1.227 o successivo.

Questo tool è disponibile solo quando la sessione è autenticata con un account claude.ai su un piano con Routine abilitate, ed è assente quando la politica della tua organizzazione disabilita [Claude Code sul web](/docs/it/claude-code-on-the-web). Su Claude Code v2.1.227 o successivo, il tool è anche assente quando un Proprietario ha [disattivato le routine per l'organizzazione](/docs/it/routines#routines-are-disabled-by-your-organizations-policy). Prima di v2.1.227, una sessione con solo il toggle delle routine disattivato mostrava comunque il tool, e il server negava le sue chiamate.

<h3 id="pushnotification">
  PushNotification
</h3>

**Nome del tool:** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

Invia una notifica push proattiva all'utente. Mantieni `message` sotto 200 caratteri perché i sistemi operativi mobili troncano il testo più lungo. Vedi la [riga PushNotification nel riferimento dei tool](/docs/it/tools-reference) per la disponibilità del provider; la consegna push viene eseguita attraverso l'infrastruttura ospitata da Anthropic che non è accessibile da Amazon Bedrock, Claude Platform su AWS, Agent Platform di Google Cloud, o Microsoft Foundry.

<h3 id="repl">
  REPL
</h3>

Rimosso in v2.1.275. Fino a v2.1.274, uno strumento sperimentale `REPL` poteva essere attivato con `CLAUDE_CODE_REPL=1` nell'[opzione `env`](#options).

<h3 id="reportfindings">
  ReportFindings
</h3>

**Nome del tool:** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Segnala i risultati della revisione del codice come un elenco strutturato in modo che Claude Code possa renderli invece di stamparli come testo. `level` è il livello di sforzo con cui è stata eseguita la revisione. I risultati sono ordinati dal più grave al meno grave, con al massimo 32 per chiamata, e l'array è vuoto quando nessuno è sopravvissuto. Richiede Claude Code v2.1.196 o successivo.

Ogni risultato contiene questi campi:

* `file`: percorso relativo al repository in cui si trova il risultato. L'opzionale `line` è la riga 1-indicizzata a cui si ancora.
* `summary`: dichiarazione di una frase del difetto. `failure_scenario` descrive gli input concreti e lo stato che portano all'output errato o al crash.
* `short_summary`: etichetta compressa opzionale di al massimo 60 caratteri per la visualizzazione compatta. Richiede Claude Code v2.1.212 o successivo.
* `category`: slug opzionale in kebab-case breve del tipo di risultato, come `correctness` o `test-coverage`. Richiede Claude Code v2.1.199 o successivo.
* `verdict`: impostato quando è stata eseguita una pass di verifica; assente nelle revisioni solo inline.
* `outcome`: impostato solo quando si segnala di nuovo dopo aver applicato le correzioni.

<h3 id="artifact">
  Artifact
</h3>

**Nome del tool:** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

Pubblica un file `.html` o `.md` locale come pagina di artifact ospitata, o elenca gli artifact pubblicati dell'utente. Ometti `action` o passa `"publish"` per pubblicare `file_path`, che è obbligatorio per l'azione di pubblicazione. Ogni campo di seguito si applica a una pubblicazione:

* `icon`: una parola generica breve per l'icona della scheda del browser dell'artifact, come `chart` o `map`. Claude la include su una prima pubblicazione e la omette su un aggiornamento, il che mantiene l'icona memorizzata dell'artifact.
* `favicon`: deprecato, e Claude lo omette.
* `title`: nomina la pagina pubblicata nella scheda del browser e nella galleria quando il file HTML non ha un tag `<title>`.
* `url`: indirizza un artifact esistente da aggiornare sul posto invece di crearne uno nuovo.

`force` è un'ultima risorsa di sovrascrittura che scarta una versione più recente che un'altra sessione ha pubblicato. In caso di conflitto, la pubblicazione non riuscita restituisce il contenuto più recente; Claude unisce le sue modifiche a quel contenuto, o rilegge l'artifact, e pubblica di nuovo. Passa `force` solo quando l'utente chiede esplicitamente di scartare quella versione.

Passa `"list"` per enumerare gli artifact pubblicati dell'utente; solo `limit` e `scope` possono accompagnarlo. `scope` predefinito a `"mine"`, che elenca gli artifact che l'utente possiede; `"shared"` elenca gli artifact che altre persone hanno condiviso con l'utente, e `"all"` elenca entrambi.

* `capabilities`: le capacità di runtime che la pagina pubblicata utilizza, codificate per nome di capacità, come i [connettori che la pagina può chiamare](/docs/it/artifacts#pull-live-data-with-mcp-connectors). Il servizio di artifact convalida la dichiarazione e rifiuta una pubblicazione che nomina una capacità che l'account non può utilizzare o ne fornisce una con una configurazione non valida. Passa `{}` per cancellare una dichiarazione memorizzata, e ometti il campo su una ridistribuzione per mantenerla. Richiede Agent SDK v0.3.235 o successivo.
* `contract`: la versione di runtime su cui viene eseguita la pagina pubblicata. Omettilo per mantenere la versione corrente dell'artifact, passa `"latest"` per aggiornare, o passa una versione specifica per fissare o eseguire il rollback. Richiede Agent SDK v0.3.235 o successivo.

I tipi vengono esportati, ma il tool è disattivato per impostazione predefinita nelle sessioni Agent SDK. La pubblicazione richiede anche ogni condizione nella [tabella di disponibilità degli artifact](/docs/it/artifacts#availability), che le sessioni autenticate con una chiave API non soddisfano.

<h3 id="projects">
  Projects
</h3>

**Nome del tool:** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

Legge e scrive il Progetto claude.ai allegato alla sessione. Invia a `method`:

* `project_info`: restituisce i metadati del progetto e l'elenco dei documenti.
* `project_read`: legge un documento per `path`.
* `project_search`: interroga la base di conoscenza del progetto con `query`. `n` limita i risultati e predefinito a 5.
* `project_write`: crea o sostituisce un documento in `path` da esattamente uno tra `content`, che contiene testo inline, o `local_path`, che nomina un file all'interno della directory di lavoro. `present_to_user: true` contrassegna il documento scritto come il deliverable che l'utente deve vedere.
* `project_delete`: elimina un documento per `path`.

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**Nome del tool:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

Elenca i figli diretti di una risorsa di directory su un server MCP. Utilizzabile solo su un server che ha dichiarato il supporto per l'elenco delle directory; l'elenco non è ricorsivo. L'elenco delle directory non è abilitato in ogni sessione: quando è disattivato, la chiamata restituisce un elenco `resources` vuoto e il campo `error` segnala che l'elenco delle directory non è abilitato.

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**Nome del tool:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

Riesamina l'elenco dei tool dei server MCP connessi e applica eventuali modifiche. I tipi vengono esportati, ma Claude Code registra il tool solo quando imposti `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` nell'[opzione `env`](#options), e solo nelle sessioni con almeno un server MCP. Richiede Claude Code v2.1.211 o successivo.

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**Nome del tool:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Renderizza una riga di chip selettore di ruolo cliccabile durante l'onboarding di Cowork in modo che l'utente possa scegliere il suo ruolo e ottenere un plugin corrispondente installato. Non accetta argomenti; l'elenco dei ruoli è definito dal client. La chiamata si blocca fino a quando l'utente non risponde.

<h3 id="mcpinput">
  McpInput
</h3>

**Nome del tool:** nomi di tool MCP dinamici della forma `mcp__<server>__<tool>`

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

Gli argomenti dei tool MCP sono un oggetto aperto: ogni server definisce i suoi propri parametri, quindi il tipo non pone vincoli sui nomi dei campi o sui valori. Consulta lo schema dei tool del server per i campi che uno strumento specifico accetta.

<h2 id="tool-output-types">
  Tipi di output dei tool
</h2>

Documentazione degli schemi di output per tutti i tool Claude Code incorporati. Questi tipi vengono esportati da `@anthropic-ai/claude-agent-sdk` e rappresentano i dati di risposta effettivi restituiti da ogni tool.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Unione di tipi di output dei tool esportati da `@anthropic-ai/claude-agent-sdk`; i membri includono:

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
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

**Nome del tool:** `Agent`. Il nome precedente `Task` è ancora accettato come alias, e l'array `tools` nel messaggio di inizializzazione [`SDKSystemMessage`](#sdksystemmessage) attualmente elenca questo tool come `Task` per compatibilità con le versioni precedenti.

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
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
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
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
      modelsUsed?: string[];
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

Sulla variante `completed`, `resolvedModel` nomina il modello su cui il subagente ha iniziato, che può differire dal `model` input richiesto quando [`availableModels`](/docs/it/model-config#restrict-model-selection) o un altro override si applica. Questo campo richiede Claude Code v2.1.174 o successivo. Su `async_launched`, nomina il modello in uso quando l'attività è passata allo sfondo.

`modelsUsed` elenca i modelli utilizzati dal subagente, in ordine. Il campo è presente solo quando si è verificato uno scambio a metà esecuzione, e un modello appare di nuovo quando l'esecuzione è tornata a esso. Su `async_launched`, l'elenco copre i modelli utilizzati prima di passare allo sfondo. Sia `modelsUsed` che il comportamento di passaggio allo sfondo di `resolvedModel` richiedono Claude Code v2.1.212 o successivo.

Se Claude Code [ha mantenuto il worktree isolato del subagente](/docs/it/worktrees#isolate-subagents-with-worktrees), `worktreePath` sul risultato `completed` è dove trovarlo. `worktreeBranch` è il suo ramo, presente quando Claude Code ha creato il worktree con git.

Claude Code riempie `usage` e `totalTokens` dalla richiesta API finale del subagente, non dall'intera esecuzione, quindi `usage.service_tier` è la stringa del livello di servizio che l'API ha segnalato su quella richiesta. Quando presente, `usage.output_tokens_details.thinking_tokens` è il numero di token di output di quella richiesta che erano token di thinking. Il campo `output_tokens_details` richiede TypeScript SDK v0.3.228 o successivo, che raggruppa Claude Code v2.1.228.

`usage.output_tokens_details` corrisponde a [`Usage.output_tokens_details`](#usage) nel significato, limitato a quella richiesta finale, ma ogni livello di esso è opzionale qui. Proteggi sia l'oggetto che il campo, ad esempio `usage.output_tokens_details?.thinking_tokens ?? 0`, piuttosto che leggerlo direttamente.

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
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
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
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

I campi `stdout`, `stderr` e `backgroundTaskId` contengono:

| Campo              | Cosa contiene                                                                                                                |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| `stdout`           | Lo stdout e stderr del comando, uniti in un unico flusso intercalato                                                         |
| `stderr`           | Avvisi che lo strumento stesso aggiunge, come un ripristino della directory di lavoro della shell, non lo stderr del comando |
| `backgroundTaskId` | Presente per i comandi di background                                                                                         |

`timedOutAfterMs` è il timeout in millisecondi, impostato quando il comando ha raggiunto il suo timeout e si è spostato allo sfondo piuttosto che iniziare lì esplicitamente. `backgroundCwdHint` viene impostato quando il comando in background conteneva un builtin di cambio directory come `cd`, `pushd`, `popd` o `chdir`, e nota che la directory di lavoro della sessione non è cambiata. Entrambi i campi richiedono Claude Code v2.1.210 o successivo.

Quando un subagente in esecuzione in primo piano possiede un comando in background, Claude Code termina il comando quando quel subagente fornisce la sua risposta finale. Claude Code imposta `backgroundEndsWithFinalResponse` su `true` su tali comandi, e omette il campo quando il comando sopravvive al turno, come i comandi avviati dalla conversazione principale o dai subagenti di background. Il campo richiede Claude Code v2.1.227 o successivo.

Claude Code imposta `gitOperation.commit.branch` al ramo denominato nella riga di riepilogo del commit di git, e lo omette per un commit effettuato su un HEAD staccato. Il campo richiede Agent SDK v0.3.227 o successivo. Claude Code segnala un comando `gh pr reopen` come l'azione PR `reopened`, che richiede Agent SDK v0.3.234 o successivo.

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
  originalFile: string | null;
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
    repository?: string | null;
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
        /** True quando una lettura di file intero è stata impaginata automaticamente perché ha superato il limite di token (il contenuto è una prima pagina parziale). */
        truncatedByTokenCap?: boolean;
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
      /** Numero di pagina del documento della prima pagina estratta; etichetta le immagini della pagina nel contenuto tool_result. */
      firstPage?: number;
      /** Solo in-process: i byte dell'immagine della pagina vengono consegnati come blocchi di immagine nel contenuto tool_result e non vengono conservati nel tool_use_result emesso, quindi questa chiave è assente lì. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Impostato quando la dedup ha corrisposto a una voce seminata all'avvio (CLAUDE.md / memoria nidificata) piuttosto che a un precedente risultato tool_result di Read. */
      source?: "seeded";
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
    repository?: string | null;
  };
  userModified?: boolean;
};
```

Restituisce il risultato della scrittura con informazioni sul diff strutturato. Ciò che `originalFile` e `structuredPatch` contengono dipende dalla scrittura:

* Per un file appena creato, `originalFile` è null e `structuredPatch` è vuoto
* Su una sovrascrittura, `originalFile` contiene il contenuto precedente, tranne quando quel contenuto è più grande di circa 10 MB: Claude Code quindi salta il diff e restituisce `originalFile` null e `structuredPatch` vuoto
* `structuredPatch` è anche vuoto quando la scrittura non ha cambiato nulla o il diff è scaduto

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
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

Restituisce i percorsi dei file che corrispondono al pattern glob, ordinati per tempo di modifica.

`totalMatches` e `countIsComplete` richiedono Claude Code v2.1.191 o successivo. `totalMatches` segnala il numero di file corrispondenti prima del troncamento. Quando `countIsComplete` è false, `totalMatches` è un limite inferiore perché la ricerca sottostante ha troncato il suo stesso output.

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
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Restituisce i risultati della ricerca. La forma varia in base a `mode`: elenco di file, contenuto con corrispondenze o conteggi di corrispondenze. In modalità `count`, `numFiles` e `numMatches` sono totali sull'intero set di risultati, non sulla sezione impaginata. Prima della v2.1.208, un `head_limit` o `offset` che troncava le voci elencate troncava anche questi totali.

`totalFiles` richiede Claude Code v2.1.208 o successivo e segnala il numero totale di risultati prima della paginazione `head_limit` e `offset` in modalità `files_with_matches`. `totalLines` richiede Claude Code v2.1.210 o successivo e segnala il numero totale di righe prima della paginazione in modalità `content`.

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
  old_source?: string;
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
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

Restituisce il contenuto recuperato con lo stato HTTP e i metadati.

`artifactRead` è il record proprio di Claude Code di una lettura di artifact, presente solo quando Claude ha recuperato un artifact che la sessione può pubblicare. Claude Code lo legge di nuovo quando una sessione riprende in modo che una successiva pubblicazione si basi sulla versione giusta; il tuo codice non ha bisogno di agire su di esso. `slug` nomina l'artifact, `ver` è la versione che la lettura ha messo in record ed è assente quando non ne ha registrata nessuna, e `seeded: false` contrassegna una lettura il cui codice sorgente completo non ha raggiunto Claude. Il campo `seeded` richiede Agent SDK v0.3.239 o successivo.

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
  searchCount?: number;
};
```

Restituisce i risultati della ricerca dal web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nome del tool:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // impostato quando il workflow è stato avviato come sessione remota
  warning?: string;
  error?: string;
};
```

Restituisce immediatamente dopo che il tool accetta l'invocazione. Il risultato finale arriva successivamente come completamento di un'attività. Controlla `error` prima di trattare l'esecuzione come avviata: uno script che non supera il controllo della sintassi restituisce `status: "async_launched"` con `error` impostato e non viene mai eseguito.

| Campo           | Tipo                                    | Descrizione                                                                                                                                                                                                     |
| --------------- | --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | Il tool ha accettato l'invocazione. `"async_launched"` per le esecuzioni in-process, `"remote_launched"` per le esecuzioni inviate a una sessione remota invece di essere eseguite in-process                   |
| `taskId`        | `string`                                | Identificatore dell'attività di background per l'esecuzione                                                                                                                                                     |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | Tipo di attività dell'attività di background registrata, corrispondente al ramo `status`                                                                                                                        |
| `workflowName`  | `string`                                | Il `meta.name` dallo script del workflow                                                                                                                                                                        |
| `runId`         | `string`                                | Identificatore dell'esecuzione del workflow da passare come `resumeFromRunId` in una successiva invocazione. Assente per le esecuzioni `remote_launched`, dove l'URL della sessione cloud è l'handle di ripresa |
| `summary`       | `string`                                | Descrizione in una riga di ciò che fa il workflow                                                                                                                                                               |
| `transcriptDir` | `string`                                | Directory dove i transcript dei subagenti vengono scritti durante l'esecuzione                                                                                                                                  |
| `scriptPath`    | `string`                                | Percorso dello script del workflow persistente per questa esecuzione. Modificalo e passalo come `scriptPath` per rieseguire senza inviare nuovamente lo script                                                  |
| `sessionUrl`    | `string`                                | URL della sessione cloud, impostato quando `status` è `"remote_launched"`                                                                                                                                       |
| `warning`       | `string`                                | Avviso non bloccante, come lo stato git locale che diverge dal ramo spinto che una sessione cloud clonerà                                                                                                       |
| `error`         | `string`                                | Impostato quando lo script non supera il controllo della sintassi. Quando presente, l'esecuzione non è stata avviata nonostante lo stato di avvio                                                               |

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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Vedi [Disponibilità del modello](/docs/it/agent-sdk/todo-tracking#model-availability) per aderire.
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
  planWasEdited?: boolean;
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
    blobSavedTo?: string;
  }>;
  error?: string;
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

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**Nome del tool:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

Restituisce l'azione intrapresa e i dettagli sul worktree che è stato abbandonato.

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**Nome del tool:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

Restituisce una conferma che la modalità di pianificazione è stata attivata.

<h3 id="croncreate-2">
  CronCreate
</h3>

**Nome del tool:** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true quando persistito in .claude/scheduled_tasks.json; false quando solo sessione
};
```

Restituisce l'ID del lavoro e una descrizione leggibile della pianificazione.

<h3 id="crondelete-2">
  CronDelete
</h3>

**Nome del tool:** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

Restituisce l'ID del lavoro eliminato.

<h3 id="cronlist-2">
  CronList
</h3>

**Nome del tool:** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
  }[];
};
```

Restituisce i lavori cron pianificati: lavori durevoli da `.claude/scheduled_tasks.json` e lavori solo sessione dalla sessione corrente. Un lavoro solo sessione porta `durable: false`; i lavori letti dal disco omettono il campo.

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**Nome del tool:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

Restituisce quando il risveglio si attiverà come timestamp di epoca in millisecondi, il ritardo effettivamente utilizzato, e se il ritardo richiesto è stato limitato. Il campo `stopped` è `true` quando la chiamata ha terminato il loop con `stop: true`. Richiede Claude Code v2.1.202 o successivo. Il campo `cancelledWakeups` conta quanti risvegli in sospeso una chiamata `stop: true` ha annullato. Un valore di 0 significa che nulla era in sospeso, e un cron `/loop` ricorrente non viene annullato da `stop: true`. Richiede Claude Code v2.1.206 o successivo.

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**Nome del tool:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

Restituisce lo stato della risposta API e il corpo per l'operazione di attivazione.

<h3 id="pushnotification-2">
  PushNotification
</h3>

**Nome del tool:** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

Restituisce i dettagli di consegna, incluso se una notifica push o locale è stata inviata e perché la consegna è stata saltata.

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**Nome del tool:** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Restituisce il numero di risultati segnalati, il livello di sforzo con cui la revisione è stata eseguita, e i risultati ripetuti per il corpo del risultato. Richiede Claude Code v2.1.196 o successivo. Il campo `short_summary` ripetuto richiede Claude Code v2.1.212 o successivo.

<h3 id="artifact-2">
  Artifact
</h3>

**Nome del tool:** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      scope?: "shared" | "all";
    };
```

Restituisce l'`url` della pagina pubblicata e il `path` locale che è stato pubblicato per l'azione di pubblicazione, con `updated` impostato su true quando la pubblicazione ha ridistribuito un artifact esistente, e `warnings` che contiene eventuali avvisi al momento della pubblicazione. L'azione di elenco restituisce invece le righe `artifacts`, con `truncated` impostato quando esistono più artifact del limite richiesto. Negli elenchi il cui ambito non è `"mine"`, ogni riga contiene `rel` che contrassegna se l'utente possiede l'artifact o se è stato condiviso con loro, e l'`scope` dell'output registra quale ambito non predefinito ha prodotto l'elenco; entrambi sono assenti negli elenchi predefiniti.

<h3 id="projects-2">
  Projects
</h3>

**Nome del tool:** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

Discriminato sul campo `method`, rispecchiando l'input. `project_read` restituisce piccoli documenti di testo inline in `content` e scrive documenti più grandi in un percorso `local_file` invece; `project_search` restituisce `hits` RAG con `rag: true` quando l'indice del progetto è disponibile e ricade su un elenco di percorsi `docs` altrimenti.

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**Nome del tool:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

Restituisce i figli diretti della risorsa directory. Le sottodirectory appaiono con mimeType `"inode/directory"`; `error` contiene un messaggio leggibile quando il server non poteva elencare la directory.

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**Nome del tool:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // strumenti ora disponibili da questo server
  added?: string[]; // nomi degli strumenti che questo aggiornamento ha aggiunto
  removed?: string[]; // nomi degli strumenti che questo aggiornamento ha rimosso
  error?: string; // perché l'aggiornamento non è riuscito o il server non era disponibile
}>;
```

Restituisce una voce per server: `refreshed` significa che l'elenco degli strumenti ri-interrogato è stato applicato, `error` significa che la ri-interrogazione non è riuscita e il set di strumenti precedente è stato mantenuto, e `not_connected` significa che il server non ha una connessione live per interrogare.

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**Nome del tool:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

Restituisce la selezione dell'utente: `role` quando ha scelto un chip di ruolo o ne ha digitato uno, e `dismissed: true` quando ha chiuso il selettore. Un oggetto vuoto significa che l'utente ha approvato la chiamata senza scegliere un ruolo.

<h3 id="mcpoutput">
  McpOutput
</h3>

**Nome del tool:** nomi di tool MCP dinamici della forma `mcp__<server>__<tool>`

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

I risultati degli strumenti MCP vengono restituiti come stringa o come array di blocchi di contenuto, a seconda del server. Il ramo di oggetto semplice finale nel tipo esportato è un artefatto della generazione dello schema: l'SDK non restituisce un oggetto nudo, perché l'output strutturato di un server viene serializzato in una stringa JSON prima di essere restituito. Al runtime il valore può anche essere `undefined`, sebbene il tipo esportato non modelli questo.

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

Da dove proviene la chiave API per le richieste della sessione, segnalata come `apiKeySource` nel messaggio di inizializzazione [`SDKSystemMessage`](#sdksystemmessage).

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code segnala uno di quattro valori:

| Valore               | Chiave in uso                                                                                                                                              |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`  | La chiave nella variabile di ambiente `ANTHROPIC_API_KEY`                                                                                                  |
| `apiKeyHelper`       | La chiave restituita dal tuo comando [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper)                                                                 |
| `/login managed key` | La chiave che Claude Code ha memorizzato quando hai effettuato l'accesso con un [account Claude Console](/docs/it/authentication#claude-console-authentication) |
| `none`               | Nessuna chiave API. La sessione si autentica in un altro modo, ad esempio un accesso a claude.ai, un token bearer, o un provider cloud                     |

Agent SDK v0.3.234 e versioni successive elencano questi quattro valori nel tipo. Il tipo mantiene anche `user`, `project`, `org`, `temporary` e `oauth` affinché il codice più vecchio continui a compilarsi, e Claude Code non li segnala.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Funzioni beta disponibili che possono essere abilitate tramite l'opzione `betas`. Vedi [Intestazioni beta](https://platform.claude.com/docs/en/api/beta-headers) per ulteriori informazioni.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  La beta `context-1m-2025-08-07` è ritirata a partire dal 30 aprile 2026. Passare questo valore con Claude Sonnet 4.5 o Sonnet 4 non ha effetto, e le richieste che superano la finestra di contesto standard di 200k token restituiscono un errore. Per usare una finestra di contesto di 1M token, esegui la migrazione a [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, o Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), che includono 1M di contesto ai prezzi standard senza intestazione beta richiesta.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informazioni su un comando disponibile.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` è `true` su una riga quando il comando è proprio di Claude Code e digitare `/name` lo esegue. È assente per un comando definito da un utente, progetto, plugin, o server MCP, e per un comando in bundle che uno di quelli [sostituisce per nome](/docs/it/skills#resolve-skills-that-share-a-name). Richiede Agent SDK v0.3.277 o successivo.

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

| Campo         | Tipo                  | Descrizione                                                                                                                                                                                                                        |
| :------------ | :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`              | Identificatore del tipo di agente (ad esempio, `"Explore"`, `"general-purpose"`)                                                                                                                                                   |
| `description` | `string`              | Descrizione di quando usare questo agente                                                                                                                                                                                          |
| `model`       | `string \| undefined` | Modello che questo agente usa: un alias o un ID di modello, o `'inherit'` per il modello del genitore. Quando è `undefined`, Claude Code sceglie il modello nell'[ordine del modello del subagente](/docs/it/sub-agents#choose-a-model) |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

Il server MCP che serve un tool `mcp__*`, e da dove proviene la definizione di quel server. Gli input dei hook [`PreToolUse`](#pretoolusehookinput), `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` e `PermissionDenied` lo portano come `mcp_server`, e le opzioni [`CanUseTool`](#canusetool) lo portano come `mcpServer`. Entrambi lo omettono per i tool che non provengono da un server MCP.

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| Campo    | Tipo     | Descrizione                                                                                                        |
| :------- | :------- | :----------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | Il nome con cui il server è registrato, lo stesso valore che [`mcpServerStatus()`](#query-object) segnala per esso |
| `source` | `string` | Da dove proviene la definizione del server: `sdk`, `plugin`, o un ambito di configurazione                         |

`source` assume uno dei seguenti valori. L'insieme è aperto, quindi tratta un valore che non riconosci come una fonte configurata, mai come `sdk`:

* **`sdk`**: un server in-process che la tua applicazione ha registrato. Solo l'applicazione host dell'SDK può registrarne uno, quindi un server configurato non segnala mai `sdk`, qualunque sia il suo nome.
* **`plugin`**: un server che un [plugin](/docs/it/agent-sdk/plugins) fornisce. Il suo `name` è la forma con ambito `plugin:<plugin-name>:<server-name>` descritta sotto [server MCP forniti da plugin](/docs/it/mcp#plugin-provided-mcp-servers).
* **Un ambito di configurazione**: `user`, `project`, `local`, `dynamic`, `managed`, `enterprise`, `claudeai`, o `agent`. Un server `.mcp.json` segnala `project`, e [ambiti di installazione MCP](/docs/it/mcp#mcp-installation-scopes) definisce `local`, `project` e `user`. I server che la tua applicazione passa nell'opzione [`mcpServers`](#options), diversi dai server SDK in-process, segnalano `dynamic`.

Basa le decisioni di fiducia su `source`, non su `name` o il prefisso del nome del tool `mcp__<server>__`. Per qualsiasi fonte diversa da `sdk`, `name` è testo non attendibile: sfuggilo prima della visualizzazione.

`McpServerProvenance` e i campi che lo portano richiedono Agent SDK v0.3.274 o successivo.

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
  source?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
    _meta?: Record<string, unknown>;
  }[];
};
```

`source` dice da dove proviene la definizione del server, con gli stessi valori e regola di fiducia di [`McpServerProvenance`](#mcpserverprovenance) `source`. Il campo richiede Agent SDK v0.3.274 o successivo ed è assente nelle versioni precedenti.

`_meta` su una voce `tools` contiene i membri MCP Apps di `_meta` di quel tool, quindi la tua applicazione può trovare la risorsa `ui://` da rendere con [`readMcpResource()`](#query-object). Claude Code passa attraverso l'oggetto `ui` e la stringa deprecata `ui/resourceUri`, e trattiene ogni altra chiave. All'interno di `ui`, `resourceUri` è una stringa `ui://` e `visibility` è un array di `"model"` e `"app"` quando il server li imposta, e qualsiasi altro membro passa attraverso invariato. Claude Code scarta entrambe le chiavi quando il valore è malformato, e omette `_meta` da un tool che non dichiara nessuno. Il campo è presente solo quando le [`capabilities`](#sdksystemmessage) del messaggio di inizializzazione includono `mcp_tool_ui_meta_v1`, e richiede TypeScript Agent SDK v0.3.280 o successivo.

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
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens` conta i token di pensiero che questo modello ha generato. `outputTokens` li include già, quindi non sommare i due insieme. Il campo è assente fino a quando un turno non viene eseguito su una versione di Claude Code che lo registra, quindi una sessione ripresa che è iniziata su una versione precedente segnala un conteggio parziale. `thinkingTokens` richiede Agent SDK v0.3.257 o successivo.

I campi `canonicalModel` e `provider` richiedono Claude Code v2.1.218 o successivo. `canonicalModel` è l'ID del modello canonico che la ricerca dei prezzi utilizza; può differire dalla stringa del modello grezzo che chiave la voce, ad esempio quando quella stringa è un ID specifico del provider o un alias.

`provider` nomina il backend API che ha servito il modello, come `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle`, o `gateway`.

`costBasis` nomina la tabella dei prezzi che ha prezzato la richiesta più recente del modello: `list` per il prezzo di listino, `managed` per una tabella [`modelPricing`](/docs/it/settings-reference#modelpricing), o `unknown` quando nessuno dei due ha corrisposto all'ID del modello. Il campo richiede Claude Code v2.1.246 o successivo.

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
  output_tokens_details: BetaOutputTokensDetails | null;
};
```

`BetaServerToolUsage`, `BetaIterationsUsage` e `BetaOutputTokensDetails` sono definiti in `@anthropic-ai/sdk`.

`output_tokens_details` suddivide l'output fatturato per categoria. Attualmente contiene un campo, `thinking_tokens: number`, che conta i token di output che il modello ha generato come ragionamento interno, inclusi i delimitatori del blocco di pensiero. Il campo `output_tokens_details` richiede TypeScript SDK v0.3.228 o successivo, che raggruppa Claude Code v2.1.228.

* **Fatturazione**: leggi la suddivisione per l'osservabilità, non per la fatturazione. `output_tokens` rimane il totale autorevole, e `output_tokens - thinking_tokens` approssima l'output non di ragionamento.
* **Cosa conta il conteggio**: il ragionamento grezzo che il modello ha prodotto, che può essere più lungo del testo di pensiero restituito nel corpo della risposta. L'API lo calcola ritokenizzando quel testo grezzo, quindi può differire dal conteggio esatto della generazione del modello di alcuni token.
* **Streaming**: sui messaggi dell'assistente trasmessi questa suddivisione, come `output_tokens`, è un placeholder `message_start` e non contiene un conteggio reale, quindi leggilo dal messaggio di risultato `usage` come [Leggi i token di output dal messaggio di risultato](/docs/it/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) descrive. Nel messaggio di risultato, `thinking_tokens` legge `0` quando il modello o il provider non segnala alcuna suddivisione.
* **Casi `null`**: `output_tokens_details` stesso è `null` sui messaggi dell'assistente che Claude Code sintetizza, come i messaggi di errore API.

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

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

Un file che un tool MCP ha restituito per riferimento. Claude Code costruisce ogni voce da un blocco `resource_link` nel risultato del tool e fornisce l'elenco come `resourceLinks` su [`SDKUserMessage.tool_use_result`](#sdkusermessage), o come `resource_links` su [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) quando la chiamata è terminata in background. Richiede Agent SDK v0.3.257 o successivo.

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code scarta un blocco il cui `uri` o `name` non è una stringa, e omette un campo opzionale il cui valore non è del tipo elencato.

| Campo         | Tipo                                   | Descrizione                                                                |
| :------------ | :------------------------------------- | :------------------------------------------------------------------------- |
| `uri`         | `string`                               | URI della risorsa, come il server l'ha restituito                          |
| `name`        | `string`                               | Nome che il server ha dato alla risorsa                                    |
| `title`       | `string \| undefined`                  | Titolo di visualizzazione, quando il server ne ha impostato uno            |
| `description` | `string \| undefined`                  | Descrizione, quando il server ne ha impostato una                          |
| `mimeType`    | `string \| undefined`                  | Tipo MIME, quando il server ne ha impostato uno                            |
| `size`        | `number \| undefined`                  | Dimensione in byte, quando il server ne ha impostato una                   |
| `annotations` | `Record<string, unknown> \| undefined` | L'oggetto annotazioni MCP del blocco, quando il server ne ha impostato uno |

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

Il campo opzionale `display` controlla se il testo di pensiero viene restituito `"summarized"` o `"omitted"`. Su Claude Opus 4.7 e versioni successive, l'impostazione predefinita dell'API è `"omitted"`, quindi imposta `"summarized"` per ricevere il contenuto di pensiero nei blocchi `thinking`. Claude Code non invia `display` ad Amazon Bedrock o alla piattaforma Agent di Google Cloud, quindi su quei provider Opus 4.7 e versioni successive restituiscono blocchi `thinking` vuoti anche quando imposti `display` su `"summarized"`.

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

Quando chiami `setMcpServers()`, Claude Code applica queste regole:

* **Server che la chiamata non nomina**: Claude Code mantiene i server forniti dai plugin in esecuzione. Richiede Agent SDK v0.3.210 o successivo.
* **Server che la chiamata nomina**: ad eccezione dei server integrati che la CLI ha avviato all'avvio, Claude Code sostituisce un server in esecuzione solo quando la sua configurazione differisce da quella che hai passato.
* **Server integrati che la CLI ha avviato all'avvio**: se la chiamata ne nomina uno, Claude Code scarta quella voce e la segnala in `errors`.

La promessa si risolve dopo che i server stdio, HTTP e SSE appena aggiunti si connettono o falliscono, quindi i tool dai server che si sono connessi sono disponibili al turno successivo.

`added` elenca i server che Claude Code ha aggiunto o sostituito, indipendentemente dal fatto che si siano connessi. Un server che non si è connesso appare sia in `added` che in `errors`, con il testo di errore sotto `errors` e una riga `failed` in [`mcpServerStatus()`](#methods). Prima di Claude Code v2.1.257, un server il cui tentativo di connessione ha lanciato un'eccezione era segnalato solo sotto `errors`.

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
  skippedLinks?: number;
};
```

`skippedLinks` conta i percorsi tracciati che il rewind ha rifiutato di ripristinare o eliminare per la sicurezza dei link: un symlink, hard link, o altro file non regolare nel percorso tracciato, una directory padre che non si risolve più a dove puntava quando il checkpoint è stato preso, o un backup che non poteva essere letto in sicurezza. Il campo richiede Claude Code v2.1.216 o successivo. Una chiamata di anteprima con `rewindFiles(userMessageId, { dryRun: true })` non lo imposta mai.

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

Notifica quando un'attività di background si completa, fallisce o viene interrotta. Le attività di background includono i comandi Bash `run_in_background`, i watch [Monitor](#monitor) e i subagenti di background. Per il campo `ambient`, vedi [`SDKTaskStartedMessage`](#sdktaskstartedmessage), che lo definisce e il suo requisito di versione.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

Quando Claude Code [sposta una lunga chiamata al tool MCP in background](/docs/it/mcp#automatic-backgrounding-of-long-tool-calls), il blocco `tool_result` per quella chiamata contiene solo un placeholder e il risultato reale della chiamata arriva in questa notifica. Abbina la notifica alla chiamata con `tool_use_id`. Su una notifica `completed`, `resource_links` elenca i file che il tool ha restituito per riferimento come voci [`SDKMcpResourceLink`](#sdkmcpresourcelink), con gli stessi limiti di 50 link e 64 KiB di [`tool_use_result.resourceLinks`](#sdkusermessage). Claude Code omette `resource_links` quando il risultato non aveva link e sulle notifiche per attività che non sono chiamate al tool MCP. `resource_links` richiede Agent SDK v0.3.257 o successivo.

Claude Code antepone un avviso a ogni notifica di attività che invia al modello, ad eccezione delle consegne contrassegnate con il sottotipo [`scheduled-trigger`](#task-notification-subkinds), che portano invece un inquadramento di attività assegnata. L'avviso afferma che non si è verificato alcun input umano, quindi il modello non tratta la notifica come un'istruzione o un'approvazione dell'utente.

Per rilevare un turno di notifica di attività, controlla `origin.kind === "task-notification"` su [`SDKUserMessage`](#sdkusermessage) o [`SDKResultMessage`](#sdkresultmessage) piuttosto che abbinare il testo dell'avviso. Leggi `subkind` dallo stesso campo se hai bisogno di sapere cosa l'ha sollevato. Prima di v2.1.205, Claude Code ometteva l'avviso dalle notifiche che arrivavano mentre la sessione era inattiva.

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
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

Mentre una chiamata al tool viene eseguita nella conversazione principale, Claude Code emette un messaggio `tool_progress` ogni 30 secondi con `heartbeat: true`. Ogni heartbeat contiene il nome del tool e i secondi trascorsi, quindi puoi distinguere una chiamata di lunga durata da una sessione bloccata. Claude Code non emette heartbeat per le chiamate al tool all'interno di un subagente. Il campo `heartbeat` richiede Agent SDK v0.3.214 o successivo. Prima di v2.1.257, Claude Code non emetteva heartbeat nemmeno per una chiamata al tool Agent in primo piano.

Sui messaggi `tool_progress` per il tool Agent diversi dagli heartbeat, `subagent_type` nomina il tipo di subagente in esecuzione, come `general-purpose`. `subagent_retry` è presente mentre quel subagente attende un backoff di errore API, come un limite di velocità o un sovraccarico, con un messaggio per tentativo di ripetizione. Entrambi i campi richiedono Agent SDK v0.3.214 o successivo.

Per rendere un indicatore di ripetizione da `subagent_retry`:

* Traccia l'indicatore per `parent_tool_use_id`, che è univoco per subagente. `tool_use_id` è condiviso da subagenti paralleli da un turno dell'assistente, quindi tracciare per esso lascerebbe che l'aggiornamento di un subagente cancelli l'indicatore di un altro.
* Cancella l'indicatore quando un successivo `tool_progress` per lo stesso `parent_tool_use_id` arriva senza `subagent_retry` né `heartbeat: true`, o quando arriva il messaggio di risultato del tool. I frame con `heartbeat: true` segnalano solo vivacità, quindi mantieni l'indicatore quando uno arriva. `attempt` può superare `max_retries` sotto ripetizione persistente, quindi non derivare la cancellazione dai contatori.
* Tratta `error_category` come un token per scegliere il tuo testo di messaggio, non come testo di visualizzazione. I valori sono `rate_limit`, `overloaded`, `authentication_failed`, `server_error`, `cloud_credential_error` e `unknown`. Gestisci un valore che non riconosci come gestisci `unknown`, perché le versioni successive possono aggiungere valori.

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

Emesso quando un'attività inizia. Il campo `task_type` è `"local_bash"` per i comandi Bash e i watch [Monitor](#monitor), `"local_agent"` per i subagenti, o `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`ambient` è `true` per le attività che non fanno parte del lavoro della sessione, come le attività che Claude Code esegue per la sua stessa operazione. I watcher di aggiornamento dal vivo sono anche ambient, inclusi i watcher che l'utente ha chiesto. Escludi le attività ambient dagli indicatori di attività. Il campo richiede Agent SDK v0.3.247 o successivo.

`ambient` appare anche su [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) e su voci [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage).

`is_backgrounded` e `spawn_depth` descrivono come Claude Code ha avviato l'attività. Entrambi i campi richiedono Agent SDK v0.3.238 o successivo.

* `is_backgrounded`: Claude Code lo imposta su attività `"local_agent"` e `"local_bash"`. `true` significa che l'attività viene eseguita in background. `false` significa che l'attività viene eseguita in primo piano, e la chiamata al tool che l'ha avviata rimane bloccata fino a quando l'attività non finisce o si sposta in background.
* `spawn_depth`: Claude Code lo imposta solo su attività `"local_agent"`. Un subagente che il thread principale ha generato ha profondità `1`. Un subagente che un subagente di profondità `1` ha generato ha profondità `2`, e così via.

Un [subagente ripreso](/docs/it/agent-sdk/subagents#resume-subagents) segnala sempre `is_backgrounded: true`, perché Claude Code esegue ogni subagente ripreso in background. Quando un'attività in primo piano si sposta in background in seguito, Claude Code segnala il nuovo valore `is_backgrounded` in un messaggio [`task_updated`](#sdktaskupdatedmessage) piuttosto che inviare un secondo `task_started`.

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

Emesso ogni volta che l'insieme delle attività di background attive cambia: un'attività inizia, si completa, viene terminata, un agente in primo piano viene messo in background, o il campo `description` o `ambient` di un'attività cambia.

L'array `tasks` è l'insieme completo attivo. Sostituisci qualsiasi insieme memorizzato nella cache con ogni payload invece di abbinare gli eventi `task_started` e `task_notification`, in modo che il prossimo cambio di appartenenza corregga qualsiasi evento che hai perso.

L'ordine relativo a quegli eventi per attività è non specificato, quindi non correlare i due flussi.

Nulla viene emesso all'avvio. Reimposta a un insieme vuoto ogni volta che il processo CLI della sessione inizia o si riavvia e lascia che il prossimo cambio di appartenenza lo ripopoli.

Quando invii una richiesta di controllo `initialize` ripetuta a una sessione in esecuzione, ad esempio con [`reinitialize()`](#query-object) dopo un gap di trasporto, Claude Code segue la risposta con uno snapshot dell'insieme attivo corrente, anche quando è vuoto. Un host che si ricollega quindi apprende cosa è in esecuzione senza aspettare il prossimo cambio di appartenenza. Prima di Agent SDK v0.3.239, Claude Code non inviava snapshot dopo un `initialize` ripetuto.

Richiede Claude Code v2.1.203 o successivo.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Emesso mentre Claude sta producendo un blocco di pensiero, incluso uno redatto. `estimated_tokens` è una stima in esecuzione dei token di pensiero generati finora nel blocco corrente, e `estimated_tokens_delta` è l'incremento portato da questo frame. Usa queste stime per la visualizzazione del progresso.

Quando il modello o il provider segnala una suddivisione, il conteggio finale per il ciclo dell'agente di primo livello è il [`usage.output_tokens_details.thinking_tokens`](#usage) del messaggio di risultato, che [non include i token dei subagenti](/docs/it/agent-sdk/cost-tracking#get-the-total-cost-of-a-query).

Richiede Claude Code v2.1.153 o successivo.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
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

Claude Code non emette questo tipo di messaggio. Quando invii un comando come `/context` o `/usage` come prompt, il suo output arriva come [`SDKAssistantMessage`](#sdkassistantmessage).

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

Emesso quando l'insieme dei comandi disponibili cambia durante la sessione, ad esempio quando Claude Code scopre skill mentre l'agente entra in una sottodirectory. L'array `commands` è l'elenco completo aggiornato, quindi sostituisci qualsiasi elenco di comandi memorizzato nella cache con questo payload. Chiamare [`supportedCommands()`](#query-object) dopo questo messaggio restituisce lo stesso elenco aggiornato, perché il metodo traccia l'ultimo push; questo richiede Agent SDK v0.3.216 o successivo. Nelle versioni SDK precedenti, `supportedCommands()` restituisce lo snapshot acquisito all'inizializzazione e non riflette mai i cambiamenti durante la sessione.

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

Emesso dopo un turno quando [`promptSuggestions`](#options) è abilitato e Claude Code ha generato un suggerimento per quel turno. Contiene il prompt utente successivo previsto. Per i turni che non ne ricevono, vedi [Quando Claude Code salta i suggerimenti](/docs/it/interactive-mode#when-claude-code-skips-suggestions).

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

Emesso quando la conversazione della sessione viene sostituita senza terminare la sessione. In una chiamata `query()`, solo `/clear` e i suoi alias producono questo messaggio. Monta una trascrizione vuota sotto `new_conversation_id` e scarta qualsiasi titolo di sessione memorizzato nella cache.

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

`AbortError` è l'unica classe di errore nell'API tipizzata dell'SDK. Altri errori, come il processo Claude Code che esce o non riesce ad avviarsi, rifiutano l'iterazione del messaggio con errori che non portano alcuna classe SDK su cui abbinare. [Troubleshooting](/docs/it/agent-sdk/troubleshooting) chiave quegli errori per messaggio, con la causa e la correzione per ciascuno.

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

| Proprietà                   | Tipo                                                  | Predefinito | Descrizione                                                                                                                                                                                                                                                                     |
| :-------------------------- | :---------------------------------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `enabled`                   | `boolean`                                             | `false`     | Abilita la modalità sandbox per l'esecuzione dei comandi                                                                                                                                                                                                                        |
| `failIfUnavailable`         | `boolean`                                             | `true`      | Arresta all'avvio se `enabled` è `true` ma la sandbox non può avviarsi. Imposta `false` per ricadere nell'esecuzione senza sandbox con un avviso su stderr                                                                                                                      |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Auto-approva i comandi Bash quando la sandbox è abilitata                                                                                                                                                                                                                       |
| `excludedCommands`          | `string[]`                                            | `[]`        | Comandi che bypassano le restrizioni della sandbox, come `['docker *']`. Questi vengono eseguiti senza sandbox automaticamente senza coinvolgimento del modello; [`sandbox.excludedCommands`](/docs/it/settings-reference#sandbox-excludedcommands) copre quando una voce si applica |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Consenti al modello di richiedere l'esecuzione di comandi al di fuori della sandbox. Quando `true`, il modello può impostare `dangerouslyDisableSandbox` nell'input del tool, che ricade nel [sistema di permessi](#permissions-fallback-for-unsandboxed-commands)              |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Configurazione della sandbox specifica della rete                                                                                                                                                                                                                               |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Configurazione della sandbox specifica del filesystem per le restrizioni di lettura/scrittura                                                                                                                                                                                   |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Mappa delle sottostringhe di comando, o `*` per ogni comando, alle sottostringhe del testo di violazione da ignorare, come `{ "*": ['/etc/hosts'] }`; vedi [`sandbox.ignoreViolations`](/docs/it/settings-reference#sandbox-ignoreviolations)                                        |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Abilita una sandbox nidificata più debole per la compatibilità                                                                                                                                                                                                                  |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Configurazione del binario ripgrep personalizzato per gli ambienti sandbox                                                                                                                                                                                                      |

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
  **Sicurezza del socket Unix:** L'opzione `allowUnixSockets` può concedere l'accesso a servizi di sistema che raggiungono al di fuori della sandbox. Ad esempio, consentire `/var/run/docker.sock` concede effettivamente l'accesso completo al sistema host tramite l'API Docker, bypassando l'isolamento della sandbox. Consenti solo i socket Unix strettamente necessari e comprendi le implicazioni di sicurezza di ciascuno.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configurazione specifica della rete per la modalità sandbox. Queste impostazioni si applicano ai comandi Bash in sandbox quando `enabled` è `true` nella [`SandboxSettings`](#sandboxsettings) padre. Non limitano lo strumento WebFetch, che utilizza invece [regole di permesso](/docs/it/permissions#webfetch).

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Proprietà                 | Tipo       | Predefinito | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------ | :--------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowedDomains`          | `string[]` | `[]`        | Nomi di dominio a cui i processi in sandbox possono accedere                                                                                                                                                                                                                                                                                                                                                                                |
| `deniedDomains`           | `string[]` | `[]`        | Nomi di dominio a cui i processi in sandbox non possono accedere. Ha la precedenza su `allowedDomains`                                                                                                                                                                                                                                                                                                                                      |
| `strictAllowlist`         | `boolean`  | `false`     | Nega ai comandi in sandbox l'accesso agli host al di fuori della [lista di autorizzazione della rete](/docs/it/sandboxing#network-isolation) invece di richiedere conferma. Applicato solo ai comandi in sandbox; i tool in-process come WebFetch non sono controllati da esso. Rispettato solo dalle impostazioni utente, gestite o CLI `--settings`; le impostazioni del progetto vengono ignorate. Richiede Claude Code v2.1.219 o successivo |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Solo impostazioni gestite. Quando impostato nelle [impostazioni gestite](/docs/it/managed-settings), solo le voci `allowedDomains` e le regole di autorizzazione `WebFetch(domain:...)` dalle impostazioni gestite vengono rispettate, e le voci di autorizzazione dalle impostazioni utente, progetto o locali vengono ignorate. Non ha effetto quando impostato tramite le opzioni SDK                                                         |
| `allowLocalBinding`       | `boolean`  | `false`     | Consenti ai processi di associarsi alle porte locali (ad esempio, per i server di sviluppo)                                                                                                                                                                                                                                                                                                                                                 |
| `allowUnixSockets`        | `string[]` | `[]`        | Percorsi dei socket Unix a cui i processi possono accedere (ad esempio, socket Docker)                                                                                                                                                                                                                                                                                                                                                      |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Consenti l'accesso a tutti i socket Unix                                                                                                                                                                                                                                                                                                                                                                                                    |
| `httpProxyPort`           | `number`   | `undefined` | Porta del proxy HTTP per le richieste di rete                                                                                                                                                                                                                                                                                                                                                                                               |
| `socksProxyPort`          | `number`   | `undefined` | Porta del proxy SOCKS per le richieste di rete                                                                                                                                                                                                                                                                                                                                                                                              |

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

Quando `allowUnsandboxedCommands` è abilitato, il modello può richiedere di eseguire comandi al di fuori della sandbox impostando `dangerouslyDisableSandbox: true` nell'input del tool. Queste richieste ricadono nel sistema di permessi esistente, il che significa che il tuo handler `canUseTool` viene invocato, permettendoti di implementare la logica di autorizzazione personalizzata.

Le tue voci `excludedCommands` invece escono dalla sandbox senza coinvolgimento del modello; [`sandbox.excludedCommands`](/docs/it/settings-reference#sandbox-excludedcommands) copre quando una voce si applica.

Nell'esempio seguente, `isCommandAuthorized` rappresenta un controllo di autorizzazione che definisci.

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

<Warning>
  I comandi in esecuzione con `dangerouslyDisableSandbox: true` hanno accesso completo al sistema. Assicurati che il tuo handler `canUseTool` convalidi queste richieste attentamente.

  Se `permissionMode` è impostato su `bypassPermissions` e `allowUnsandboxedCommands` è abilitato, il modello può autonomamente eseguire comandi al di fuori della sandbox senza prompt di approvazione, a parte le [azioni che nessuna modalità auto-approva](/docs/it/permission-modes#actions-no-mode-auto-approves). Questa combinazione consente effettivamente al modello di sfuggire all'isolamento della sandbox silenziosamente.
</Warning>

<h2 id="see-also">
  Vedi anche
</h2>

* [Panoramica dell'SDK](/docs/it/agent-sdk/overview) - Concetti generali dell'SDK
* [Riferimento Python SDK](/docs/it/agent-sdk/python) - Documentazione dell'SDK Python
* [Riferimento CLI](/docs/it/cli-reference) - Interfaccia della riga di comando
* [Flussi di lavoro comuni](/docs/it/common-workflows) - Guide passo dopo passo
