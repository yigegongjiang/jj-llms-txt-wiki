> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento dei canali

> Crea un server MCP che invia webhook, avvisi e messaggi di chat in una sessione di Claude Code. Riferimento per il contratto del canale: dichiarazione di capacità, eventi di notifica, strumenti di risposta, gating del mittente e inoltro delle autorizzazioni.

<Note>
  I canali sono in [anteprima di ricerca](/docs/it/channels#research-preview). Le organizzazioni Team ed Enterprise devono [abilitarli esplicitamente](/docs/it/channels#enterprise-controls).
</Note>

Un canale è un server MCP che invia eventi in una sessione di Claude Code in modo che Claude possa reagire a cose che accadono al di fuori del terminale.

Puoi creare un canale unidirezionale o bidirezionale. I canali unidirezionali inoltrano avvisi, webhook o eventi di monitoraggio per cui Claude può agire. I canali bidirezionali come i bridge di chat [espongono anche uno strumento di risposta](#expose-a-reply-tool) in modo che Claude possa inviare messaggi indietro. Un canale con un percorso mittente affidabile può anche optare per [inoltro delle richieste di autorizzazione](#relay-permission-prompts) in modo da poter approvare o negare l'uso dello strumento da remoto.

Questa pagina copre:

* [Panoramica](#overview): come funzionano i canali
* [Cosa ti serve](#what-you-need): requisiti e passaggi generali
* [Esempio: crea un ricevitore webhook](#example-build-a-webhook-receiver): una procedura dettagliata minima unidirezionale
* [Opzioni del server](#server-options): i campi del costruttore
* [Formato di notifica](#notification-format): il payload dell'evento e il comportamento di consegna
* [Esponi uno strumento di risposta](#expose-a-reply-tool): consenti a Claude di inviare messaggi indietro
* [Gating dei messaggi in entrata](#gate-inbound-messages): controlli del mittente per prevenire l'iniezione di prompt
* [Inoltro delle richieste di autorizzazione](#relay-permission-prompts): inoltra i prompt di approvazione dello strumento ai canali remoti

Per utilizzare un canale esistente invece di crearne uno, vedi [Canali](/docs/it/channels). Telegram, Discord, iMessage e fakechat sono inclusi nell'anteprima di ricerca.

<h2 id="overview">
  Panoramica
</h2>

Un canale è un server [MCP](https://modelcontextprotocol.io) che viene eseguito sulla stessa macchina di Claude Code. Claude Code lo genera come un sottoprocesso e comunica tramite stdio. Il tuo server di canale è il ponte tra i sistemi esterni e la sessione di Claude Code:

* **Piattaforme di chat** (Telegram, Discord): il tuo plugin viene eseguito localmente e interroga l'API della piattaforma per i nuovi messaggi. Quando qualcuno invia un DM al tuo bot, il plugin riceve il messaggio e lo inoltra a Claude. Nessun URL da esporre.
* **Webhook** (CI, monitoraggio): il tuo server ascolta su una porta HTTP locale. I sistemi esterni POST a quella porta e il tuo server invia il payload a Claude.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-architecture.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=9a037b7da80184ae49015c0256b21a1f" alt="Diagramma dell'architettura che mostra i sistemi esterni che si connettono al tuo server di canale locale, che comunica con Claude Code tramite stdio" width="600" height="220" data-path="images/channel-architecture.svg" />

<h2 id="what-you-need">
  Cosa ti serve
</h2>

L'unico requisito difficile è il pacchetto [`@modelcontextprotocol/sdk`](https://www.npmjs.com/package/@modelcontextprotocol/sdk) e un runtime compatibile con Node.js. [Bun](https://bun.sh), [Node](https://nodejs.org) e [Deno](https://deno.com) funzionano tutti. I plugin precostruiti nell'anteprima di ricerca utilizzano Bun, ma il tuo canale non deve necessariamente.

Il tuo server deve:

1. Dichiarare la capacità `claude/channel` in modo che Claude Code registri un listener di notifica
2. Emettere eventi `notifications/claude/channel` quando accade qualcosa
3. Connettersi tramite [trasporto stdio](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) (Claude Code genera il tuo server come un sottoprocesso)

Le sezioni [Opzioni del server](#server-options) e [Formato di notifica](#notification-format) coprono ciascuna di queste in dettaglio. Vedi [Esempio: crea un ricevitore webhook](#example-build-a-webhook-receiver) per una procedura dettagliata completa.

Durante l'anteprima di ricerca, i canali personalizzati non sono nella [lista di approvazione approvata](/docs/it/channels#supported-channels). Usa `--dangerously-load-development-channels` per testare localmente. Vedi [Test durante l'anteprima di ricerca](#test-during-the-research-preview) per i dettagli.

<h2 id="example-build-a-webhook-receiver">
  Esempio: crea un ricevitore webhook
</h2>

Questa procedura dettagliata crea un server a file singolo che ascolta le richieste HTTP e le inoltra nella tua sessione di Claude Code. Alla fine, qualsiasi cosa possa inviare un HTTP POST, come una pipeline CI, un avviso di monitoraggio o un comando `curl`, può inviare eventi a Claude.

Questo esempio utilizza [Bun](https://bun.sh) come runtime per il suo server HTTP integrato e il supporto di TypeScript. Puoi utilizzare [Node](https://nodejs.org) o [Deno](https://deno.com) invece; l'unico requisito è l'[MCP SDK](https://www.npmjs.com/package/@modelcontextprotocol/sdk).

<Steps>
  <Step title="Crea il progetto">
    Crea una nuova directory e installa l'MCP SDK:

    ```bash theme={null}
    mkdir webhook-channel && cd webhook-channel
    bun add @modelcontextprotocol/sdk
    ```
  </Step>

  <Step title="Scrivi il server di canale">
    Crea un file chiamato `webhook.ts`. Questo è l'intero server di canale: si connette a Claude Code tramite stdio e ascolta i POST HTTP sulla porta 8788. Quando arriva una richiesta, invia il corpo a Claude come evento di canale.

    ```ts title="webhook.ts" theme={null}
    #!/usr/bin/env bun
    import { Server } from '@modelcontextprotocol/sdk/server/index.js'
    import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'

    // Crea il server MCP e dichiaralo come canale
    const mcp = new Server(
      { name: 'webhook', version: '0.0.1' },
      {
        // questa chiave è ciò che lo rende un canale — Claude Code registra un listener per essa
        capabilities: { experimental: { 'claude/channel': {} } },
        // aggiunto al prompt di sistema di Claude in modo che sappia come gestire questi eventi
        instructions: 'Gli eventi dal canale webhook arrivano come <channel source="webhook" ...>. Sono unidirezionali: leggili e agisci, nessuna risposta prevista.',
      },
    )

    // Connettiti a Claude Code tramite stdio (Claude Code genera questo processo)
    await mcp.connect(new StdioServerTransport())

    // Avvia un server HTTP che inoltra ogni POST a Claude
    Bun.serve({
      port: 8788,  // qualsiasi porta aperta funziona
      // solo localhost: nulla al di fuori di questa macchina può POST
      hostname: '127.0.0.1',
      async fetch(req) {
        const body = await req.text()
        await mcp.notification({
          method: 'notifications/claude/channel',
          params: {
            content: body,  // diventa il corpo del tag <channel>
            // ogni chiave diventa un attributo del tag, ad es. <channel path="/" method="POST">
            meta: { path: new URL(req.url).pathname, method: req.method },
          },
        })
        return new Response('ok')
      },
    })
    ```

    Il file fa tre cose in ordine:

    * **Configurazione del server**: crea il server MCP con `claude/channel` nelle sue capacità, che è ciò che dice a Claude Code che questo è un canale. La stringa [`instructions`](#server-options) va nel prompt di sistema di Claude: dì a Claude quali eventi aspettarsi, se rispondere e come instradare le risposte se dovrebbe.
    * **Connessione stdio**: si connette a Claude Code tramite stdin/stdout. Questo è standard per qualsiasi [server MCP](https://modelcontextprotocol.io/docs/concepts/transports#standard-io): Claude Code lo genera come un sottoprocesso.
    * **Listener HTTP**: avvia un server web locale sulla porta 8788. Ogni corpo POST viene inoltrato a Claude come evento di canale tramite `mcp.notification()`. Il `content` diventa il corpo dell'evento e ogni voce `meta` diventa un attributo sul tag `<channel>`. Il listener ha bisogno dell'accesso all'istanza `mcp`, quindi viene eseguito nello stesso processo. Potresti dividerlo in moduli separati per un progetto più grande.
  </Step>

  <Step title="Registra il tuo server con Claude Code">
    Aggiungi il server alla tua configurazione MCP in modo che Claude Code sappia come avviarlo. Per un `.mcp.json` a livello di progetto nella stessa directory, usa un percorso relativo. Per la configurazione a livello di utente in `~/.claude.json`, usa il percorso assoluto completo in modo che il server possa essere trovato da qualsiasi progetto:

    ```json title=".mcp.json" theme={null}
    {
      "mcpServers": {
        "webhook": { "command": "bun", "args": ["./webhook.ts"] }
      }
    }
    ```

    Claude Code legge la tua configurazione MCP all'avvio e genera ogni server come un sottoprocesso.
  </Step>

  <Step title="Testalo">
    Durante l'anteprima di ricerca, i canali personalizzati non sono nella lista di approvazione, quindi avvia Claude Code con il flag di sviluppo:

    ```bash theme={null}
    claude --dangerously-load-development-channels server:webhook
    ```

    La prima volta che avvii una sessione in questo progetto, Claude Code chiede il consenso prima di utilizzare il nuovo server da `.mcp.json`. La finestra di dialogo segnala "Nuovo server MCP trovato in questo progetto: webhook". Seleziona **Usa questo server MCP** per continuare.

    Quando Claude Code si avvia, legge la tua configurazione MCP, genera il tuo `webhook.ts` come un sottoprocesso e il listener HTTP si avvia automaticamente sulla porta che hai configurato (8788 in questo esempio). Non è necessario eseguire il server da solo.

    Un avviso attenuato sotto il banner di avvio conferma che il canale è registrato: `Channels (experimental) messages from server:webhook inject directly in this session · restart without --dangerously-load-development-channels to stop`.

    Se vedi "bloccato dalla politica dell'organizzazione", il tuo amministratore dell'organizzazione deve prima [abilitare i canali](/docs/it/channels#enterprise-controls).

    In un terminale separato, simula un webhook inviando un HTTP POST con un messaggio al tuo server. Questo esempio invia un avviso di errore CI alla porta 8788 (o qualsiasi porta tu abbia configurato):

    ```bash theme={null}
    curl -X POST localhost:8788 -d "build failed on main: https://ci.example.com/run/1234"
    ```

    Il payload arriva nella tua sessione di Claude Code come un tag `<channel>`:

    ```text theme={null}
    <channel source="webhook" path="/" method="POST">build failed on main: https://ci.example.com/run/1234</channel>
    ```

    Nel tuo terminale di Claude Code, vedrai Claude ricevere il messaggio e iniziare a rispondere: leggendo file, eseguendo comandi o qualsiasi cosa il messaggio richieda. Questo è un canale unidirezionale, quindi Claude agisce nella tua sessione ma non invia nulla indietro tramite il webhook. Per aggiungere risposte, vedi [Esponi uno strumento di risposta](#expose-a-reply-tool).

    Se l'evento non arriva, la diagnosi dipende da ciò che `curl` ha restituito:

    * **`curl` ha successo ma nulla raggiunge Claude**: esegui `/mcp` nella tua sessione per controllare lo stato del server. "Impossibile connettersi" di solito significa un errore di dipendenza o importazione nel tuo file server; controlla il log di debug in `~/.claude/debug/<session-id>.txt` per la traccia stderr.
    * **`curl` fallisce con "connessione rifiutata"**: la porta non è ancora associata o un processo stantio da un'esecuzione precedente la sta mantenendo. `lsof -i :<port>` mostra cosa sta ascoltando; `kill` il processo stantio prima di riavviare la tua sessione.
  </Step>
</Steps>

Il [server fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) estende questo modello con un'interfaccia web, allegati di file e uno strumento di risposta per chat bidirezionale.

<h2 id="test-during-the-research-preview">
  Test durante l'anteprima di ricerca
</h2>

Durante l'anteprima di ricerca, ogni canale deve essere nella [lista di approvazione approvata](/docs/it/channels#research-preview) per registrarsi. Il flag di sviluppo bypassa la lista di approvazione per voci specifiche dopo un prompt di conferma. Questo esempio mostra entrambi i tipi di voce:

```bash theme={null}
# Test di un plugin che stai sviluppando
claude --dangerously-load-development-channels plugin:yourplugin@yourmarketplace

# Test di un server .mcp.json nudo (nessun wrapper di plugin ancora)
claude --dangerously-load-development-channels server:webhook
```

Il bypass è per voce. Combinare questo flag con `--channels` non estende il bypass alle voci `--channels`. Durante l'anteprima di ricerca, la lista di approvazione approvata è curata da Anthropic, quindi il tuo canale rimane sul flag di sviluppo mentre lo costruisci e lo testi.

<Note>
  Questo flag salta solo la lista di approvazione. La politica organizzativa `channelsEnabled` si applica comunque. Non usarla per eseguire canali da fonti non attendibili.
</Note>

<h2 id="server-options">
  Opzioni del server
</h2>

Un canale imposta queste opzioni nel costruttore [`Server`](https://modelcontextprotocol.io/docs/concepts/servers). I campi `instructions` e `capabilities.tools` sono [MCP standard](https://modelcontextprotocol.io/docs/concepts/servers); `capabilities.experimental['claude/channel']` e `capabilities.experimental['claude/channel/permission']` sono le aggiunte specifiche del canale:

| Campo                                                    | Tipo     | Descrizione                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------------------------- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capabilities.experimental['claude/channel']`            | `object` | Obbligatorio. Sempre `{}`. La presenza registra il listener di notifica.                                                                                                                                                                                                                                                               |
| `capabilities.experimental['claude/channel/permission']` | `object` | Facoltativo. Sempre `{}`. Dichiara che questo canale può ricevere richieste di inoltro delle autorizzazioni. Quando dichiarato, Claude Code inoltra i prompt di approvazione dello strumento al tuo canale in modo da poter approvare o negare da remoto. Vedi [Inoltro delle richieste di autorizzazione](#relay-permission-prompts). |
| `capabilities.tools`                                     | `object` | Solo bidirezionale. Sempre `{}`. Capacità dello strumento MCP standard. Vedi [Esponi uno strumento di risposta](#expose-a-reply-tool).                                                                                                                                                                                                 |
| `instructions`                                           | `string` | Consigliato. Aggiunto al prompt di sistema di Claude. Dì a Claude quali eventi aspettarsi, cosa significano gli attributi del tag `<channel>`, se rispondere e, se sì, quale strumento usare e quale attributo passare indietro (come `chat_id`).                                                                                      |

Per creare un canale unidirezionale, ometti `capabilities.tools`. Questo esempio mostra una configurazione bidirezionale con la capacità del canale, gli strumenti e le istruzioni impostate:

```ts theme={null}
import { Server } from '@modelcontextprotocol/sdk/server/index.js'

const mcp = new Server(
  { name: 'your-channel', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },  // registra il listener del canale
      tools: {},  // ometti per canali unidirezionali
    },
    // aggiunto al prompt di sistema di Claude in modo che sappia come gestire i tuoi eventi
    instructions: 'I messaggi arrivano come <channel source="your-channel" ...>. Rispondi con lo strumento di risposta.',
  },
)
```

Per inviare un evento, chiama `mcp.notification()` con il metodo `notifications/claude/channel`. I parametri sono nella sezione successiva.

<h2 id="notification-format">
  Formato di notifica
</h2>

Il tuo server emette `notifications/claude/channel` con due parametri:

| Campo     | Tipo                     | Descrizione                                                                                                                                                                                                                                                                                                              |
| :-------- | :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content` | `string`                 | Il corpo dell'evento. Consegnato come il corpo del tag `<channel>`.                                                                                                                                                                                                                                                      |
| `meta`    | `Record<string, string>` | Facoltativo. Ogni voce diventa un attributo sul tag `<channel>` per il contesto di instradamento come ID chat, nome del mittente o gravità dell'avviso. Le chiavi devono essere identificatori: solo lettere, cifre e sottolineature. Le chiavi contenenti trattini o altri caratteri vengono silenziosamente eliminate. |

Il tuo server invia gli eventi chiamando `mcp.notification()` sull'istanza `Server`. Questo esempio invia un avviso di errore CI con due chiavi meta:

```ts theme={null}
await mcp.notification({
  method: 'notifications/claude/channel',
  params: {
    content: 'build failed on main: https://ci.example.com/run/1234',
    meta: { severity: 'high', run_id: '1234' },
  },
})
```

L'evento arriva nel contesto di Claude avvolto in un tag `<channel>`. L'attributo `source` viene impostato automaticamente dal nome configurato del tuo server:

```text theme={null}
<channel source="your-channel" severity="high" run_id="1234">
build failed on main: https://ci.example.com/run/1234
</channel>
```

Le notifiche non vengono riconosciute. L'`await` su `mcp.notification()` si risolve quando il messaggio viene scritto nel trasporto, non quando Claude lo ha elaborato. Se la sessione non ha caricato il tuo server come canale, o la politica organizzativa lo blocca, gli eventi vengono eliminati silenziosamente senza errore restituito al tuo server.

Se hai bisogno di conferma di consegna, traccia lo stato dell'evento nel tuo server ed esponi uno [strumento di risposta](#expose-a-reply-tool) che Claude può chiamare per segnalare lo stato indietro.

Gli eventi si accodano nella sessione e vengono elaborati in ordine. Se più notifiche arrivano mentre Claude è occupato, vengono consegnate insieme al turno successivo e Claude le gestisce come un gruppo. Per elaborare flussi di eventi indipendenti contemporaneamente, esegui sessioni separate.

<h2 id="expose-a-reply-tool">
  Esponi uno strumento di risposta
</h2>

Se il tuo canale è bidirezionale, come un bridge di chat piuttosto che un inoltro di avvisi, esponi uno [strumento MCP](https://modelcontextprotocol.io/docs/concepts/tools) standard che Claude può chiamare per inviare messaggi indietro. Nulla della registrazione dello strumento è specifico del canale. Uno strumento di risposta ha tre componenti:

1. Una voce `tools: {}` nelle capacità del tuo costruttore `Server` in modo che Claude Code scopra lo strumento
2. Handler dello strumento che definiscono lo schema dello strumento e implementano la logica di invio
3. Una stringa `instructions` nel tuo costruttore `Server` che dice a Claude quando e come chiamare lo strumento

Per aggiungere questi al [ricevitore webhook sopra](#example-build-a-webhook-receiver):

<Steps>
  <Step title="Abilita la scoperta dello strumento">
    Nel tuo costruttore `Server` in `webhook.ts`, aggiungi `tools: {}` alle capacità in modo che Claude Code sappia che il tuo server offre strumenti:

    ```ts theme={null}
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},  // abilita la scoperta dello strumento
    },
    ```
  </Step>

  <Step title="Registra lo strumento di risposta">
    Aggiungi quanto segue a `webhook.ts`. L'`import` va in cima al file con i tuoi altri import; i due handler vanno tra il costruttore `Server` e `mcp.connect()`. Questo registra uno strumento `reply` che Claude può chiamare con un `chat_id` e `text`:

    ```ts theme={null}
    // Aggiungi questo import in cima a webhook.ts
    import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

    // Claude interroga questo all'avvio per scoprire quali strumenti offre il tuo server
    mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [{
        name: 'reply',
        description: 'Invia un messaggio indietro su questo canale',
        // inputSchema dice a Claude quali argomenti passare
        inputSchema: {
          type: 'object',
          properties: {
            chat_id: { type: 'string', description: 'La conversazione in cui rispondere' },
            text: { type: 'string', description: 'Il messaggio da inviare' },
          },
          required: ['chat_id', 'text'],
        },
      }],
    }))

    // Claude chiama questo quando vuole invocare uno strumento
    mcp.setRequestHandler(CallToolRequestSchema, async req => {
      if (req.params.name === 'reply') {
        const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
        // send() è il tuo outbound: POST alla tua piattaforma di chat, o per il test locale
        // il broadcast SSE mostrato nell'esempio completo di seguito.
        send(`Reply to ${chat_id}: ${text}`)
        return { content: [{ type: 'text', text: 'sent' }] }
      }
      throw new Error(`unknown tool: ${req.params.name}`)
    })
    ```
  </Step>

  <Step title="Aggiorna le istruzioni">
    Aggiorna la stringa `instructions` nel tuo costruttore `Server` in modo che Claude sappia di instradare le risposte indietro tramite lo strumento. Questo esempio dice a Claude di passare `chat_id` dal tag in entrata:

    ```ts theme={null}
    instructions: 'I messaggi arrivano come <channel source="webhook" chat_id="...">. Rispondi con lo strumento di risposta, passando il chat_id dal tag.'
    ```
  </Step>
</Steps>

Ecco il `webhook.ts` completo con supporto bidirezionale. Le risposte in uscita vengono trasmesse su `GET /events` utilizzando [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE), quindi `curl -N localhost:8788/events` può guardarle dal vivo; la chat in entrata arriva su `POST /`:

```ts title="Full webhook.ts with reply tool' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

// --- Outbound: scrivi su qualsiasi listener curl -N su /events ---
// Un vero bridge farebbe POST alla tua piattaforma di chat invece.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},
    },
    instructions: 'I messaggi arrivano come <channel source="webhook" chat_id="...">. Rispondi con lo strumento di risposta, passando il chat_id dal tag.',
  },
)

mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Invia un messaggio indietro su questo canale',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'La conversazione in cui rispondere' },
        text: { type: 'string', description: 'Il messaggio da inviare' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

await mcp.connect(new StdioServerTransport())

let nextId = 1
Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // non chiudere i flussi SSE inattivi
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: flusso SSE in modo che curl -N possa guardare le risposte di Claude dal vivo
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // in modo che curl mostri qualcosa immediatamente
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // POST: inoltra a Claude come evento di canale
    const body = await req.text()
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: {
        content: body,
        meta: { chat_id, path: url.pathname, method: req.method },
      },
    })
    return new Response('ok')
  },
})
```

Il [server fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) mostra un esempio più completo con allegati di file e modifica dei messaggi.

<h2 id="gate-inbound-messages">
  Gating dei messaggi in entrata
</h2>

Un canale senza gating è un vettore di iniezione di prompt. Chiunque possa raggiungere il tuo endpoint può mettere testo davanti a Claude. Un canale che ascolta una piattaforma di chat o un endpoint pubblico ha bisogno di un vero controllo del mittente prima di emettere qualsiasi cosa.

Controlla il mittente rispetto a una lista di approvazione prima di chiamare `mcp.notification()`. Questo esempio elimina qualsiasi messaggio da un mittente non nel set:

```ts theme={null}
const allowed = new Set(loadAllowlist())  // dal tuo access.json o equivalente

// dentro il tuo gestore di messaggi, prima di emettere:
if (!allowed.has(message.from.id)) {  // mittente, non stanza
  return  // elimina silenziosamente
}
await mcp.notification({ ... })
```

Gating sull'identità del mittente, non sull'identità della chat o della stanza: `message.from.id` nell'esempio, non `message.chat.id`. Nelle chat di gruppo, questi differiscono e il gating sulla stanza permetterebbe a chiunque in un gruppo approvato di iniettare messaggi nella sessione.

I canali [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) e [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) fanno il gating su una lista di approvazione del mittente allo stesso modo. Avviano la lista tramite accoppiamento: l'utente invia un DM al bot, il bot risponde con un codice di accoppiamento, l'utente lo approva nella sua sessione di Claude Code e il suo ID della piattaforma viene aggiunto. Vedi entrambe le implementazioni per il flusso di accoppiamento completo. Il canale [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) adotta un approccio diverso: rileva gli indirizzi dell'utente dal database dei Messaggi all'avvio e li lascia passare automaticamente, con altri mittenti aggiunti per handle.

<h2 id="relay-permission-prompts">
  Inoltro delle richieste di autorizzazione
</h2>

Quando Claude chiama uno strumento che ha bisogno di approvazione, la finestra di dialogo del terminale locale si apre e la sessione attende. Un canale bidirezionale può optare per ricevere lo stesso prompt in parallelo e inoltrarlo a te su un altro dispositivo. Entrambi rimangono attivi: puoi rispondere nel terminale o sul tuo telefono e Claude Code applica qualsiasi risposta arrivi per prima e chiude l'altra.

L'inoltro copre le approvazioni di uso dello strumento come `Bash`, `Write` e `Edit`. La fiducia del progetto e i dialoghi di consenso del server MCP non vengono inoltrati; quelli appaiono solo nel terminale locale.

<h3 id="how-relay-works">
  Come funziona l'inoltro
</h3>

Quando si apre un prompt di autorizzazione, il ciclo di inoltro ha quattro passaggi:

1. Claude Code genera un breve ID di richiesta e notifica il tuo server
2. Il tuo server inoltra il prompt e l'ID alla tua app di chat
3. L'utente remoto risponde con un sì o no e quell'ID
4. Il tuo gestore in entrata analizza la risposta in un verdetto e Claude Code lo applica solo se l'ID corrisponde a una richiesta aperta

La finestra di dialogo del terminale locale rimane aperta durante tutto questo. Se qualcuno al terminale risponde prima che il verdetto remoto arrivi, quella risposta viene applicata invece e la richiesta remota in sospeso viene eliminata.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-permission-relay.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=97d57f128f0da55f105ab1e3a7e10240" alt="Diagramma di sequenza: Claude Code invia una notifica permission_request al server del canale, il server formatta e invia il prompt all'app di chat, l'umano risponde con un verdetto e il server analizza quella risposta in una notifica di autorizzazione indietro a Claude Code" width="600" height="230" data-path="images/channel-permission-relay.svg" />

<h3 id="permission-request-fields">
  Campi della richiesta di autorizzazione
</h3>

La notifica in uscita da Claude Code è `notifications/claude/channel/permission_request`. Come la [notifica del canale](#notification-format), il trasporto è MCP standard ma il metodo e lo schema sono estensioni di Claude Code. L'oggetto `params` ha quattro campi stringa che il tuo server formatta nel prompt in uscita:

| Campo           | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                              |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `request_id`    | Cinque lettere minuscole tratte da `a`-`z` senza `l`, quindi non legge mai come `1` o `I` quando digitato su un telefono. Includilo nel tuo prompt in uscita in modo che possa essere ripetuto nella risposta. Claude Code accetta solo un verdetto che porta un ID che ha emesso. La finestra di dialogo del terminale locale non visualizza questo ID, quindi il tuo gestore in uscita è l'unico modo per apprenderlo. |
| `tool_name`     | Nome dello strumento che Claude vuole usare, ad esempio `Bash` o `Write`.                                                                                                                                                                                                                                                                                                                                                |
| `description`   | Riepilogo leggibile di cosa fa questa specifica chiamata dello strumento, lo stesso testo che la finestra di dialogo del terminale locale mostra. Per una chiamata Bash questo è la descrizione di Claude del comando, o il comando stesso se nessuno è stato fornito.                                                                                                                                                   |
| `input_preview` | Gli argomenti dello strumento come stringa JSON, troncati a 200 caratteri. Per Bash questo è il comando; per Write è il percorso del file e un prefisso del contenuto. Omettilo dal tuo prompt se hai solo spazio per un messaggio di una riga. Il tuo server decide cosa mostrare.                                                                                                                                      |

Il verdetto che il tuo server rimanda è `notifications/claude/channel/permission` con due campi: `request_id` che ripete l'ID sopra e `behavior` impostato su `'allow'` o `'deny'`. Allow consente alla chiamata dello strumento di procedere; deny la rifiuta, lo stesso che rispondere No nella finestra di dialogo locale. Nessun verdetto influisce sulle chiamate future.

<h3 id="add-relay-to-a-chat-bridge">
  Aggiungi inoltro a un bridge di chat
</h3>

L'aggiunta dell'inoltro delle autorizzazioni a un canale bidirezionale richiede tre componenti:

1. Una voce `claude/channel/permission: {}` sotto le capacità `experimental` nel tuo costruttore `Server` in modo che Claude Code sappia di inoltrare i prompt
2. Un gestore di notifica per `notifications/claude/channel/permission_request` che formatta il prompt e lo invia tramite l'API della tua piattaforma
3. Un controllo nel tuo gestore di messaggi in entrata che riconosce `yes <id>` o `no <id>` e emette una notifica di verdetto `notifications/claude/channel/permission` invece di inoltrare il testo a Claude

Dichiara la capacità solo se il tuo canale [autentica il mittente](#gate-inbound-messages), perché chiunque possa rispondere tramite il tuo canale può approvare o negare l'uso dello strumento nella tua sessione.

Per aggiungere questi a un bridge di chat bidirezionale come quello assemblato in [Esponi uno strumento di risposta](#expose-a-reply-tool):

<Steps>
  <Step title="Dichiara la capacità di autorizzazione">
    Nel tuo costruttore `Server`, aggiungi `claude/channel/permission: {}` accanto a `claude/channel` sotto `experimental`:

    ```ts theme={null}
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opta per l'inoltro delle autorizzazioni
      },
      tools: {},
    },
    ```
  </Step>

  <Step title="Gestisci la richiesta in entrata">
    Registra un gestore di notifica tra il tuo costruttore `Server` e `mcp.connect()`. Claude Code lo chiama con i [quattro campi di richiesta](#permission-request-fields) quando si apre una finestra di dialogo di autorizzazione. Il tuo gestore formatta il prompt per la tua piattaforma e include istruzioni per rispondere con l'ID:

    ```ts theme={null}
    import { z } from 'zod'

    // setNotificationHandler instrada per z.literal sul campo method,
    // quindi questo schema è sia il validatore che la chiave di dispatch
    const PermissionRequestSchema = z.object({
      method: z.literal('notifications/claude/channel/permission_request'),
      params: z.object({
        request_id: z.string(),     // cinque lettere minuscole, includi verbatim nel tuo prompt
        tool_name: z.string(),      // ad es. "Bash", "Write"
        description: z.string(),    // riepilogo leggibile di questa chiamata
        input_preview: z.string(),  // argomenti dello strumento come JSON, troncati a ~200 caratteri
      }),
    })

    mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
      // send() è il tuo outbound: POST alla tua piattaforma di chat, o per il test locale
      // il broadcast SSE mostrato nell'esempio completo di seguito.
      send(
        `Claude vuole eseguire ${params.tool_name}: ${params.description}\n\n` +
        // l'ID nell'istruzione è ciò che il tuo gestore in entrata analizza nel Passaggio 3
        `Rispondi "yes ${params.request_id}" o "no ${params.request_id}"`,
      )
    })
    ```
  </Step>

  <Step title="Intercetta il verdetto nel tuo gestore in entrata">
    Il tuo gestore in entrata è il ciclo o il callback che riceve messaggi dalla tua piattaforma: lo stesso posto dove [fai il gating sul mittente](#gate-inbound-messages) e emetti `notifications/claude/channel` per inoltrare la chat a Claude. Aggiungi un controllo prima della chiamata di inoltro della chat che riconosce il formato del verdetto e emette la notifica di autorizzazione invece.

    L'espressione regolare corrisponde al formato dell'ID che Claude Code genera: cinque lettere, mai `l`. Il flag `/i` tollera l'autocorrezione del telefono che capitalizza la risposta; minuscola l'ID catturato prima di inviarlo indietro.

    ```ts theme={null}
    // corrisponde a "y abcde", "yes abcde", "n abcde", "no abcde"
    // [a-km-z] è l'alfabeto dell'ID che Claude Code usa (minuscolo, salta 'l')
    // /i tollera l'autocorrezione del telefono; minuscola la cattura prima di inviare
    const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i

    async function onInbound(message: PlatformMessage) {
      if (!allowed.has(message.from.id)) return  // fai il gating sul mittente per primo

      const m = PERMISSION_REPLY_RE.exec(message.text)
      if (m) {
        // m[1] è la parola del verdetto, m[2] è l'ID della richiesta
        // emetti la notifica del verdetto indietro a Claude Code invece della chat
        await mcp.notification({
          method: 'notifications/claude/channel/permission',
          params: {
            request_id: m[2].toLowerCase(),  // normalizza nel caso di autocorrezione maiuscola
            behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
          },
        })
        return  // gestito come verdetto, non inoltrare anche come chat
      }

      // non corrisponde al formato del verdetto: passa al percorso della chat normale
      await mcp.notification({
        method: 'notifications/claude/channel',
        params: { content: message.text, meta: { chat_id: String(message.chat.id) } },
      })
    }
    ```
  </Step>
</Steps>

Claude Code mantiene anche la finestra di dialogo del terminale locale aperta, quindi puoi rispondere in entrambi i posti e la prima risposta ad arrivare viene applicata. Una risposta remota che non corrisponde esattamente al formato previsto fallisce in uno di due modi e in entrambi i casi la finestra di dialogo rimane aperta:

* **Formato diverso**: l'espressione regolare del tuo gestore in entrata non corrisponde, quindi testo come `approve it` o `yes` senza un ID passa come messaggio normale a Claude.
* **Formato corretto, ID sbagliato**: il tuo server emette un verdetto, ma Claude Code non trova nessuna richiesta aperta con quell'ID e lo elimina silenziosamente.

<h3 id="full-example">
  Esempio completo
</h3>

Il `webhook.ts` assemblato di seguito combina tutte e tre le estensioni da questa pagina: lo strumento di risposta, il gating del mittente e l'inoltro delle autorizzazioni. Se stai iniziando qui, avrai anche bisogno della [configurazione del progetto e della voce `.mcp.json`](#example-build-a-webhook-receiver) dalla procedura dettagliata iniziale.

Per rendere entrambe le direzioni testabili da curl, il listener HTTP serve due percorsi:

* **`GET /events`**: mantiene aperto un flusso SSE e invia ogni messaggio in uscita come una riga `data:`, quindi `curl -N` può guardare le risposte di Claude e i prompt di autorizzazione arrivare dal vivo.
* **`POST /`**: il lato in entrata, lo stesso gestore di prima, ora con il controllo del formato del verdetto inserito prima del ramo di inoltro della chat.

```ts title="Full webhook.ts with permission relay' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'
import { z } from 'zod'

// --- Outbound: scrivi su qualsiasi listener curl -N su /events ---
// Un vero bridge farebbe POST alla tua piattaforma di chat invece.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

// Lista di approvazione del mittente. Per la procedura dettagliata locale confidiamo nel singolo valore dell'intestazione X-Sender
// "dev"; un vero bridge controllerebbe l'ID utente della piattaforma.
const allowed = new Set(['dev'])

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opta per l'inoltro delle autorizzazioni
      },
      tools: {},
    },
    instructions:
      'I messaggi arrivano come <channel source="webhook" chat_id="...">. ' +
      'Rispondi con lo strumento di risposta, passando il chat_id dal tag.',
  },
)

// --- strumento di risposta: Claude chiama questo per inviare un messaggio indietro ---
mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Invia un messaggio indietro su questo canale',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'La conversazione in cui rispondere' },
        text: { type: 'string', description: 'Il messaggio da inviare' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

// --- inoltro delle autorizzazioni: Claude Code (non Claude) chiama questo quando si apre una finestra di dialogo
const PermissionRequestSchema = z.object({
  method: z.literal('notifications/claude/channel/permission_request'),
  params: z.object({
    request_id: z.string(),
    tool_name: z.string(),
    description: z.string(),
    input_preview: z.string(),
  }),
})

mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
  send(
    `Claude vuole eseguire ${params.tool_name}: ${params.description}\n\n` +
    `Rispondi "yes ${params.request_id}" o "no ${params.request_id}"`,
  )
})

await mcp.connect(new StdioServerTransport())

// --- HTTP su :8788: GET /events trasmette in uscita, POST instrada in entrata ---
const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i
let nextId = 1

Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // non chiudere i flussi SSE inattivi
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: flusso SSE in modo che curl -N possa guardare risposte e prompt dal vivo
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // in modo che curl mostri qualcosa immediatamente
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // tutto il resto è in entrata: fai il gating sul mittente per primo
    const body = await req.text()
    const sender = req.headers.get('X-Sender') ?? ''
    if (!allowed.has(sender)) return new Response('forbidden', { status: 403 })

    // controlla il formato del verdetto prima di trattare come chat
    const m = PERMISSION_REPLY_RE.exec(body)
    if (m) {
      await mcp.notification({
        method: 'notifications/claude/channel/permission',
        params: {
          request_id: m[2].toLowerCase(),
          behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
        },
      })
      return new Response('verdict recorded')
    }

    // chat normale: inoltra a Claude come evento di canale
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: { content: body, meta: { chat_id, path: url.pathname } },
    })
    return new Response('ok')
  },
})
```

Testa il percorso del verdetto in tre terminali. Il primo è la tua sessione di Claude Code, avviata con il [flag di sviluppo](#test-during-the-research-preview) in modo che generi `webhook.ts`:

```bash theme={null}
claude --dangerously-load-development-channels server:webhook
```

Nel secondo, trasmetti il lato in uscita in modo da poter vedere le risposte di Claude e i prompt di autorizzazione mentre arrivano:

```bash theme={null}
curl -N localhost:8788/events
```

Nel terzo, invia un messaggio che farà sì che Claude tenti di eseguire un comando:

```bash theme={null}
curl -d "list the files in this directory" -H "X-Sender: dev" localhost:8788
```

L'elenco dei file è di sola lettura, quindi Claude lo esegue senza approvazione. La finestra di dialogo di autorizzazione si apre quando Claude chiama lo strumento `reply` per inviare la sua risposta indietro. La finestra di dialogo locale si apre nel tuo terminale di Claude Code e un momento dopo il prompt per `mcp__webhook__reply` appare nel flusso `/events`, incluso l'ID di cinque lettere. Approvalo dal lato remoto:

```bash theme={null}
curl -d "yes <id>" -H "X-Sender: dev" localhost:8788
```

La finestra di dialogo locale si chiude, lo strumento `reply` viene eseguito e la risposta di Claude atterra nel flusso.

I tre pezzi specifici del canale in questo file:

* **Capacità** nel costruttore `Server`: `claude/channel` registra il listener di notifica, `claude/channel/permission` opta per l'inoltro delle autorizzazioni, `tools` consente a Claude di scoprire lo strumento di risposta.
* **Percorsi in uscita**: il gestore dello strumento `reply` è ciò che Claude chiama per le risposte conversazionali; il gestore di notifica `PermissionRequestSchema` è ciò che Claude Code chiama quando si apre una finestra di dialogo di autorizzazione. Entrambi chiamano `send()` per trasmettere su `/events`, ma vengono attivati da parti diverse del sistema.
* **Gestore HTTP**: `GET /events` mantiene aperto un flusso SSE in modo che curl possa guardare l'uscita dal vivo; `POST` è in entrata, gated sull'intestazione `X-Sender`. Un corpo `yes <id>` o `no <id>` va a Claude Code come notifica di verdetto e non raggiunge mai Claude; qualsiasi altra cosa viene inoltrata a Claude come evento di canale.

<h2 id="package-as-a-plugin">
  Pacchetto come plugin
</h2>

Per rendere il tuo canale installabile e condivisibile, avvolgilo in un [plugin](/docs/it/plugins) e pubblicalo in un [marketplace](/docs/it/plugin-marketplaces). Gli utenti lo installano con `/plugin install`, quindi lo abilitano per sessione con `--channels plugin:<name>@<marketplace>`.

Un canale pubblicato nel tuo marketplace ha ancora bisogno di `--dangerously-load-development-channels` per essere eseguito, poiché non è nella [lista di approvazione](/docs/it/channels#supported-channels). La lista di approvazione predefinita è i plugin di canale in `claude-plugins-official`, che Anthropic cura a sua discrezione. I [moduli di invio in-app](/docs/it/plugins#submit-your-plugin-to-the-community-marketplace) aggiungono plugin al marketplace della comunità, che non è nella lista di approvazione dei canali.

Se stai lavorando con un contatto partner di Anthropic, contattalo per coordinare un'inserzione nel marketplace ufficiale. Nei piani Team ed Enterprise, un amministratore può invece includere il tuo plugin nella lista [`allowedChannelPlugins`](/docs/it/channels#restrict-which-channel-plugins-can-run) dell'organizzazione, che sostituisce la lista di approvazione predefinita di Anthropic.

<h2 id="see-also">
  Vedi anche
</h2>

* [Canali](/docs/it/channels) per installare e utilizzare Telegram, Discord, iMessage o la demo fakechat, e per abilitare i canali per un'organizzazione Team o Enterprise
* [Implementazioni di canali funzionanti](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) per il codice del server completo con flussi di accoppiamento, strumenti di risposta e allegati di file
* [MCP](/docs/it/mcp) per il protocollo sottostante che i server di canale implementano
* [Plugin](/docs/it/plugins) per pacchettizzare il tuo canale in modo che gli utenti possano installarlo con `/plugin install`
