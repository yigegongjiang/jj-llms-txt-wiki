> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Connetti Claude Code ai tuoi strumenti tramite MCP

> Scopri come connettere Claude Code ai tuoi strumenti con il Model Context Protocol.

Claude Code può connettersi a centinaia di strumenti e fonti di dati esterni attraverso il [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction), uno standard open source per le integrazioni AI-tool. I server MCP danno a Claude Code accesso ai tuoi strumenti, database e API.

Connetti un server quando ti trovi a copiare dati in chat da un altro strumento, come un issue tracker o un dashboard di monitoraggio. Una volta connesso, Claude può leggere e agire su quel sistema direttamente invece di lavorare da quello che incolla.

Se stai connettendo il tuo primo server, inizia con la [guida rapida MCP](/docs/it/mcp-quickstart) per una procedura dettagliata. Questa pagina è il riferimento completo.

<h2 id="what-you-can-do-with-mcp">
  Cosa puoi fare con MCP
</h2>

Con i server MCP connessi, puoi chiedere a Claude Code di:

* **Implementare funzionalità da issue tracker**: "Aggiungi la funzionalità descritta nel ticket JIRA ENG-4521 e crea una PR su GitHub."
* **Analizzare dati di monitoraggio**: "Controlla Sentry e Statsig per verificare l'utilizzo della funzionalità descritta in ENG-4521."
* **Interrogare database**: "Trova gli indirizzi email di 10 utenti casuali che hanno utilizzato la funzionalità ENG-4521, in base al nostro database PostgreSQL."
* **Integrare design**: "Aggiorna il nostro modello di email standard in base ai nuovi design Figma che sono stati pubblicati su Slack"
* **Automatizzare flussi di lavoro**: "Crea bozze Gmail invitando questi 10 utenti a una sessione di feedback sulla nuova funzionalità."
* **Reagire a eventi esterni**: Un server MCP può anche agire come un [canale](/docs/it/channels) che invia messaggi nella tua sessione, in modo che Claude reagisca ai messaggi Telegram, chat Discord o eventi webhook mentre sei assente.

<h2 id="find-and-build-mcp-servers">
  Trovare e costruire server MCP
</h2>

Sfoglia i connettori verificati nella [Anthropic Directory](https://claude.ai/directory). I connettori della Directory utilizzano la stessa infrastruttura MCP di Claude Code, quindi puoi aggiungere qualsiasi server remoto elencato lì con `claude mcp add`.

<Warning>
  Verifica di fidarti di ogni server prima di collegarlo. I server che recuperano contenuti esterni possono esporti al rischio di [prompt injection](/docs/it/security#protect-against-prompt-injection).
</Warning>

Per costruire il tuo server, consulta la [guida al server MCP](https://modelcontextprotocol.io/docs/develop/build-server) per i fondamenti del protocollo e la [documentazione sulla creazione di connettori Claude](https://claude.com/docs/connectors/building) per l'autenticazione, i test e l'invio alla Directory.

Puoi anche far scaffoldare un server da Claude con il plugin ufficiale [`mcp-server-dev`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev).

<Steps>
  <Step title="Installa il plugin">
    In una sessione Claude Code, esegui:

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    Se Claude Code segnala che il marketplace non è stato trovato, esegui prima `/plugin marketplace add anthropics/claude-plugins-official`, quindi riprova l'installazione. Una volta installato, esegui `/reload-plugins` per attivarlo nella sessione corrente.
  </Step>

  <Step title="Esegui lo skill di compilazione">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude ti chiede informazioni sul tuo caso d'uso e scaffolda un server HTTP remoto o un server stdio locale.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  Installazione dei server MCP
</h2>

I server MCP possono essere configurati in diversi modi a seconda delle vostre esigenze:

<h3 id="option-1-add-a-remote-http-server">
  Opzione 1: Aggiungi un server HTTP remoto
</h3>

I server HTTP sono l'opzione consigliata per connettersi ai server MCP remoti. Questo è il trasporto più ampiamente supportato per i servizi basati su cloud.

```bash theme={null}
# Sintassi di base
claude mcp add --transport http <name> <url>

# Esempio reale: Connessione a Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Esempio con token Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Quando si configurano i server MCP tramite JSON in `.mcp.json`, `~/.claude.json`, o `claude mcp add-json`, il campo `type` accetta `streamable-http` come alias per `http`. La specifica MCP utilizza il nome `streamable-http` per questo trasporto, quindi le configurazioni copiate dalla documentazione del server funzionano senza modifiche.

Una voce JSON che ha un `url` ma nessun `type` è un errore di configurazione, perché Claude Code legge una voce senza `type` come un server stdio. Claude Code salta quel server e segnala `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Prima della v2.1.202, Claude Code segnalava questa configurazione errata come `command: expected string, received undefined`.

<h3 id="option-2-add-a-remote-sse-server">
  Opzione 2: Aggiungi un server SSE remoto
</h3>

<Warning>
  Il trasporto SSE (Server-Sent Events) è deprecato. Utilizza server HTTP invece, dove disponibili.
</Warning>

```bash theme={null}
# Sintassi di base
claude mcp add --transport sse <name> <url>

# Esempio reale: Connessione ad Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Esempio con header di autenticazione
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Opzione 3: Aggiungi un server stdio locale
</h3>

I server stdio vengono eseguiti come processi locali sulla vostra macchina. Sono ideali per strumenti che necessitano di accesso diretto al sistema o script personalizzati.

Claude Code imposta `CLAUDE_PROJECT_DIR` nell'ambiente del server generato alla radice del progetto, in modo che il vostro server possa risolvere i percorsi relativi al progetto senza dipendere dalla directory di lavoro. Questa è la stessa directory che gli hook ricevono nella loro variabile `CLAUDE_PROJECT_DIR`. Leggetela dall'interno del vostro processo server, ad esempio `process.env.CLAUDE_PROJECT_DIR` in Node o `os.environ["CLAUDE_PROJECT_DIR"]` in Python.

`CLAUDE_PROJECT_DIR` è la radice stabile del progetto e non cambia quando aggiungete o rimuovete directory di lavoro durante la sessione. Un server che limita il proprio accesso al filesystem a un insieme di directory consentite dovrebbe implementare la richiesta MCP `roots/list`. Claude Code risponde a `roots/list` con la directory di avvio della sessione più ogni [directory di lavoro aggiuntiva](/docs/it/permissions#working-directories) che avete concesso con `--add-dir`, `/add-dir`, o l'impostazione `additionalDirectories`. Claude Code invia `notifications/roots/list_changed` quando tale insieme cambia. Prima della v2.1.203, `roots/list` restituiva solo la directory di avvio e Claude Code non inviava `notifications/roots/list_changed`.

Questa variabile è impostata nell'ambiente del server, non nell'ambiente di Claude Code stesso, quindi farvi riferimento tramite l'espansione `${VAR}` in un `.mcp.json` con ambito progetto o utente `command` o `args` richiede un valore predefinito come `${CLAUDE_PROJECT_DIR:-.}`. Le configurazioni MCP fornite da plugin sostituiscono `${CLAUDE_PROJECT_DIR}` direttamente e non hanno bisogno del valore predefinito.

```bash theme={null}
# Sintassi di base
claude mcp add [options] <name> -- <command> [args...]

# Esempio reale: Aggiungi server Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Importante: Separare gli argomenti del server con `--`**

  Per i server stdio, il `--` (doppio trattino) separa le opzioni di Claude, come `--transport`, `--env`, e `--scope`, dal comando e dagli argomenti che eseguono il server. Tutto ciò che viene dopo `--` viene passato al server senza modifiche.

  Per esempio:

  * `claude mcp add --transport stdio myserver -- npx server` → esegue `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → esegue `python server.py --port 8080` con `KEY=value` nell'ambiente

  Senza `--`, Claude Code cercherebbe di analizzare i flag del server, come `--port` sopra, come proprie opzioni.

  `--env` accetta più coppie `KEY=value`. Se il nome del server viene direttamente dopo `--env`, la CLI legge il nome come un'altra coppia e lo rifiuta, quindi posizionate almeno un'altra opzione tra `--env` e il nome del server, come negli esempi sopra.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Opzione 4: Aggiungi un server WebSocket remoto
</h3>

I server WebSocket mantengono una connessione bidirezionale persistente, che si adatta ai server MCP remoti che inviano eventi a Claude senza sollecitazione. Utilizza HTTP invece quando il vostro server risponde solo alle richieste, poiché HTTP supporta OAuth e il flag `claude mcp add --transport`, mentre WebSocket non supporta nessuno dei due.

Configura i server WebSocket in `.mcp.json` o con `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

La voce `type: "ws"` accetta gli stessi campi `url`, `headers`, `headersHelper`, `timeout`, e `alwaysLoad` di `http`. L'autenticazione è solo tramite header, quindi passa un token statico in `headers` o generane uno al momento della connessione con [`headersHelper`](#use-dynamic-headers-for-custom-authentication). Il flag `claude mcp add --transport` non accetta `ws`.

<h3 id="managing-your-servers">
  Gestione dei vostri server
</h3>

Una volta configurati, potete gestire i vostri server MCP con questi comandi:

```bash theme={null}
# Elenca tutti i server configurati
claude mcp list

# Ottieni dettagli per un server specifico
claude mcp get github

# Rimuovi un server
claude mcp remove github

# (all'interno di Claude Code) Controlla lo stato del server
/mcp
```

I server con ambito progetto da `.mcp.json` che sono in attesa della vostra approvazione appaiono in `claude mcp list` come `⏸ Pending approval`. Esegui `claude` in modo interattivo per esaminarli e approvarli. `claude mcp get <name>` mostra i server in sospeso come `⏸ Pending approval` e i server rifiutati come `✗ Rejected`.

A partire dalla v2.1.196, `claude mcp list` e `claude mcp get` leggono le approvazioni `.mcp.json` solo dai file di impostazioni che non sono archiviati nel repository finché non fidate dell'area di lavoro eseguendo `claude` in essa e accettando la finestra di dialogo di trust dell'area di lavoro. Un repository clonato non può approvare i propri server: [`enableAllProjectMcpServers` o `enabledMcpjsonServers`](/docs/it/settings#available-settings) committati nel `.claude/settings.json` del progetto vengono ignorati in una cartella non attendibile, e il server rimane a `⏸ Pending approval` invece di essere connesso e sottoposto a health-check.

Le approvazioni da queste fonti si applicano ancora in una cartella non attendibile:

* il vostro `~/.claude/settings.json` utente
* impostazioni gestite
* impostazioni passate con `--settings`

Le approvazioni in un `.claude/settings.local.json` non tracciato si applicano anche, ma solo dopo che accettate una finestra di dialogo di trust per quella cartella o una delle sue directory padre: Claude Code esegue git per verificare se il file è tracciato, ed esegue quel controllo solo in una cartella attendibile. In una cartella che non avete mai fidata, le approvazioni del file attendono la finestra di dialogo di trust a meno che la cartella non sia la vostra home di configurazione: la vostra home directory, o una directory il cui `.claude` avete impostato come [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars). Prima della v2.1.207, un `.claude/settings.local.json` non tracciato approvava i server in una cartella che non avete mai fidata.

Una voce `disabledMcpjsonServers` in qualsiasi file di impostazioni rifiuta comunque il server.

Il pannello `/mcp` mostra il conteggio degli strumenti accanto a ogni server connesso e contrassegna i server che pubblicizzano la capacità degli strumenti ma non espongono alcuno strumento.

Un server remoto la cui configurazione ha un `url` vuoto appare come `not configured` in `/mcp`, in `claude mcp list`, e nel gestore [`/plugin`](/docs/it/plugins), e Claude Code non tenta di connettersi ad esso. Un plugin può includere una voce segnaposto come questa per un connettore che configurate in seguito, in modo che Claude Code non lo segnali come un errore o un problema di configurazione. La vista dei dettagli del server in `/mcp` legge `No URL configured for this server`; impostare l'`url` della voce per connettersi. Prima della v2.1.208, Claude Code segnalava un `url` vuoto come un problema di configurazione con un prompt per riconnettersi.

Se la vostra richiesta ha bisogno di strumenti da un server che è ancora in fase di connessione in background, Claude attende quel server prima di continuare. Con la [ricerca degli strumenti](#scale-with-mcp-tool-search) abilitata, che è l'impostazione predefinita, l'attesa avviene all'interno della chiamata `ToolSearch`. Nelle configurazioni senza ricerca degli strumenti, come Google Cloud's Agent Platform, un `ANTHROPIC_BASE_URL` personalizzato, o `ENABLE_TOOL_SEARCH=false`, Claude utilizza lo strumento `WaitForMcpServers` invece.

Alcuni nomi di server sono riservati per i server integrati di Claude Code: `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, e `Claude Browser`. Se la vostra configurazione definisce un server con un nome riservato, Claude Code lo salta al momento del caricamento e mostra un avviso chiedendovi di rinominarlo. `claude mcp add` rifiuta un nome riservato con un errore.

`Claude Preview` e `Claude Browser` denominano entrambi il server integrato che il [riquadro di anteprima dell'app desktop Claude Code](/docs/it/desktop#preview-your-app) utilizza. Prima della v2.1.205, `Claude Browser` non era riservato, quindi un server configurato dall'utente poteva registrarsi con quel nome.

<h3 id="dynamic-tool-updates">
  Aggiornamenti dinamici degli strumenti
</h3>

Claude Code supporta le notifiche `list_changed` di MCP, consentendo ai server MCP di aggiornare dinamicamente i loro strumenti, prompt e risorse disponibili senza richiedere di disconnettersi e riconnettersi. Quando un server MCP invia una notifica `list_changed`, Claude Code aggiorna automaticamente le capacità disponibili da quel server.

<h3 id="automatic-reconnection">
  Riconnessione automatica
</h3>

Se un server HTTP o SSE si disconnette durante una sessione, Claude Code si riconnette automaticamente con backoff esponenziale: fino a cinque tentativi, a partire da un ritardo di un secondo e raddoppiando ogni volta. Il server appare come in sospeso in `/mcp` mentre la riconnessione è in corso. Dopo cinque tentativi falliti il server è contrassegnato come non riuscito e potete riprovare manualmente da `/mcp`. I server stdio sono processi locali e non vengono riconnessi automaticamente.

Lo stesso backoff si applica quando un server HTTP o SSE non riesce la sua connessione iniziale all'avvio. A partire dalla v2.1.121, Claude Code ritenta la connessione iniziale fino a tre volte su errori transitori come una risposta 5xx, una connessione rifiutata o un timeout, quindi contrassegna il server come non riuscito se ancora non riesce a connettersi. Gli errori di autenticazione e di non trovato non vengono ritentati perché richiedono una modifica della configurazione per essere risolti.

Quando un server configurato non riesce a connettersi, Claude Code dice a Claude quale server non ha avuto successo e il suo errore di connessione, incluso nei risultati `ToolSearch` che non trovano alcuno strumento corrispondente, in modo che Claude segnali l'errore di connessione nella sua risposta. Richiede la [ricerca degli strumenti](#scale-with-mcp-tool-search), che è abilitata per impostazione predefinita. Nelle configurazioni senza ricerca degli strumenti, come un `ANTHROPIC_BASE_URL` personalizzato, `ENABLE_TOOL_SEARCH=false`, o un modello che non supporta la ricerca degli strumenti, e su Amazon Bedrock, Google Cloud's Agent Platform, e Microsoft Foundry, Claude Code non segnala gli errori di connessione del server fallito a Claude. Prima della v2.1.205, Claude Code non passava gli errori di connessione a Claude, e Claude poteva rispondere come se gli strumenti del server fallito non fossero mai stati configurati.

A partire dalla v2.1.191, le richieste di scoperta delle capacità che vengono eseguite dopo una connessione riuscita, come `tools/list`, `prompts/list`, e `resources/list`, ritentano anche gli errori di rete transitori e del server fino a tre volte con backoff breve. Gli errori di autenticazione, le risposte 4xx e i timeout delle richieste non vengono ritentati.

<h3 id="push-messages-with-channels">
  Invia messaggi con canali
</h3>

Un server MCP può anche inviare messaggi direttamente nella vostra sessione in modo che Claude possa reagire a eventi esterni come risultati CI, avvisi di monitoraggio o messaggi di chat. Per abilitare questa funzionalità, il vostro server dichiara la capacità `claude/channel` e voi la abilitate con il flag `--channels` all'avvio. Vedi [Channels](/docs/it/channels) per utilizzare un canale ufficialmente supportato, oppure [Channels reference](/docs/it/channels-reference) per costruire il vostro.

<Tip>
  Suggerimenti:

  * Utilizza il flag `-s` o `--scope` per specificare dove viene archiviata la configurazione:
    * `local` (predefinito): disponibile solo per voi nel progetto corrente. Le versioni precedenti chiamavano questo ambito `project`
    * `project`: condiviso con tutti nel progetto tramite il file `.mcp.json`
    * `user`: disponibile per voi in tutti i progetti. Le versioni precedenti chiamavano questo ambito `global`
  * Imposta le variabili di ambiente con i flag `-e` o `--env` (per esempio, `-e KEY=value`)
  * I flag `--transport` e `--header` accettano anche le forme brevi `-t` e `-H`
  * Configura il timeout di avvio del server MCP utilizzando la variabile di ambiente `MCP_TIMEOUT` (per esempio, `MCP_TIMEOUT=10000 claude` imposta un timeout di 10 secondi)
  * Imposta un timeout di esecuzione dello strumento per server aggiungendo un campo `timeout` in millisecondi alla voce `.mcp.json` di quel server, per esempio `"timeout": 600000` per dieci minuti. Questo sostituisce la variabile di ambiente `MCP_TOOL_TIMEOUT` solo per quel server
  * Claude Code visualizza un avviso quando l'output dello strumento MCP supera 10.000 token e limita l'output a 25.000 token per impostazione predefinita. Per aumentare il limite, imposta la variabile di ambiente `MAX_MCP_OUTPUT_TOKENS` (per esempio, `MAX_MCP_OUTPUT_TOKENS=50000`); la soglia di avviso è fissa. Vedi [Limiti di output MCP e avvisi](#mcp-output-limits-and-warnings)
  * Utilizza `/mcp` per autenticarti con server remoti che richiedono l'autenticazione OAuth 2.0
</Tip>

Il timeout per server è un limite rigido di wall-clock per chiamata dello strumento, e le notifiche di progresso dal server non lo estendono. I valori inferiori a 1000 sono ignorati e ricadono in `MCP_TOOL_TIMEOUT`, o nel suo valore predefinito di circa 28 ore quando quella variabile non è impostata. Per un server HTTP, SSE, o [connettore claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) c'è anche un secondo timer per richiesta che copre ogni richiesta fino al primo byte di risposta del server. Quel timer è di 60 secondi a meno che non impostiate il timeout per server o `MCP_TOOL_TIMEOUT`; impostare uno di essi a 60 secondi o superiore aumenta il timer per richiesta a quel valore, un valore inferiore non lo accorcia, e il valore predefinito di 28 ore di un `MCP_TOOL_TIMEOUT` non impostato non lo alimenta mai. I server stdio e WebSocket non hanno un timer per richiesta. Prima della v2.1.162, i valori inferiori a 1000 erano arrotondati a un secondo.

Un timeout per server di almeno 1000 agisce anche come limite inferiore sul timeout di inattività descritto di seguito: Claude Code non interrompe mai le chiamate dello strumento di quel server per inattività prima del timeout per server. Richiede Claude Code v2.1.203 o successivo.

Una chiamata dello strumento a un server MCP che non invia alcuna risposta e nessuna notifica di progresso per la finestra di inattività si interrompe con un errore invece di attendere il limite di wall-clock. Il timeout di inattività richiede Claude Code v2.1.187 o successivo. Si applica a ogni tipo di server tranne i server IDE e i server in-process SDK. La finestra di inattività è predefinita a cinque minuti per i server HTTP, SSE, WebSocket, e [connettore claude.ai](#use-mcp-servers-from-claude-ai), e a 30 minuti per i server stdio. Prima della v2.1.203, i server stdio erano esenti dal timeout di inattività.

Imposta la variabile di ambiente [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/it/env-vars) in millisecondi per modificare la finestra di inattività, o impostala su `0` per disabilitare il controllo.

<h3 id="plugin-provided-mcp-servers">
  Server MCP forniti da plugin
</h3>

I [plugin](/docs/it/plugins) possono raggruppare server MCP, fornendo automaticamente strumenti e integrazioni quando il plugin è abilitato. I server MCP dei plugin funzionano in modo identico ai server configurati dall'utente.

**Come funzionano i server MCP dei plugin**:

* I plugin definiscono i server MCP in `.mcp.json` nella radice del plugin o inline in `plugin.json`
* Quando un plugin è abilitato, i suoi server MCP si avviano automaticamente
* Gli strumenti MCP del plugin appaiono insieme agli strumenti MCP configurati manualmente
* I server dei plugin vengono gestiti tramite l'installazione del plugin, non tramite comandi `/mcp`

**Esempio di configurazione MCP del plugin**:

In `.mcp.json` nella radice del plugin:

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

O inline in `plugin.json`:

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**Funzionalità MCP del plugin**:

* **Ciclo di vita automatico**: all'avvio della sessione, i server per i plugin abilitati si connettono automaticamente. Se abilitate o disabilitate un plugin durante una sessione, eseguite `/reload-plugins` per connettere o disconnettere i suoi server MCP
* **Variabili di percorso**: `${CLAUDE_PLUGIN_ROOT}` si risolve nella directory di installazione del plugin, `${CLAUDE_PLUGIN_DATA}` nella sua directory di [stato persistente](/docs/it/plugins-reference#persistent-data-directory), e `${CLAUDE_PROJECT_DIR}` nella radice stabile del progetto. La sostituzione si applica a:
  * server `stdio`: `command`, `args`, `env`
  * server `http`, `sse`, e `ws`: `url`, `headers`, e `headersHelper`. Prima della v2.1.195, `headersHelper` passava il segnaposto come una stringa letterale
* **Accesso alle variabili di ambiente dell'utente**: accesso alle stesse variabili di ambiente dei server configurati manualmente
* **Tipi di trasporto multipli**: supporto per trasporti stdio, SSE, HTTP e WebSocket, anche se il supporto del trasporto può variare a seconda del server

**Visualizzazione dei server MCP del plugin**:

```bash theme={null}
# All'interno di Claude Code, vedi tutti i server MCP inclusi quelli dei plugin
/mcp
```

I server dei plugin appaiono nell'elenco con indicatori che mostrano che provengono dai plugin.

**Nomi degli strumenti MCP del plugin**:

Gli strumenti da un server MCP raggruppato in un plugin includono sia il nome del plugin che la chiave del server nel loro nome richiamabile. La forma completa è `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, dove qualsiasi carattere al di fuori di `A-Z`, `a-z`, `0-9`, `_`, e `-` viene sostituito con `_`. Per il server `database-tools` raggruppato in un plugin denominato `my-plugin`, uno strumento `query` è richiamabile come:

```
mcp__plugin_my-plugin_database-tools__query
```

Utilizza questo nome completo quando fai riferimento allo strumento nelle [regole di autorizzazione](/docs/it/permissions), nell'elenco `allowed-tools` di una skill, nel [campo `tools` di un subagent](/docs/it/sub-agents#available-tools), o in un [matcher di hook](/docs/it/hooks#match-mcp-tools). Un matcher di hook scritto rispetto alla chiave del server nuda, come `mcp__database-tools__.*`, non si attiva mai per un server raggruppato in un plugin.

Il server stesso si registra sotto il nome con ambito `plugin:<plugin-name>:<server-name>`, come `plugin:my-plugin:database-tools`. Utilizza quel nome dove è previsto un nome di server configurato, come il [campo `server` di un hook `mcp_tool`](/docs/it/hooks#mcp-tool-hook-fields).

**Vantaggi dei server MCP dei plugin**:

* **Distribuzione raggruppata**: strumenti e server confezionati insieme
* **Configurazione automatica**: nessuna configurazione MCP manuale necessaria
* **Coerenza del team**: tutti ottengono gli stessi strumenti quando il plugin è installato

Vedi il [riferimento dei componenti del plugin](/docs/it/plugins-reference#mcp-servers) per i dettagli su come raggruppare i server MCP con i plugin.

<h2 id="mcp-installation-scopes">
  Ambiti di installazione MCP
</h2>

I server MCP possono essere configurati a tre ambiti diversi. L'ambito che scegli controlla in quali progetti il server viene caricato e se la configurazione è condivisa con il tuo team. Gli amministratori possono anche distribuire server a livello aziendale tramite [configurazione gestita](#managed-mcp-configuration).

| Ambito                     | Carica in                 | Condiviso con il team                | Archiviato in                         |
| -------------------------- | ------------------------- | ------------------------------------ | ------------------------------------- |
| [Locale](#local-scope)     | Solo il progetto corrente | No                                   | `~/.claude.json`                      |
| [Progetto](#project-scope) | Solo il progetto corrente | Sì, tramite controllo della versione | `.mcp.json` nella radice del progetto |
| [Utente](#user-scope)      | Tutti i tuoi progetti     | No                                   | `~/.claude.json`                      |

<h3 id="local-scope">
  Ambito locale
</h3>

L'ambito locale è il predefinito. Un server con ambito locale viene caricato solo nel progetto in cui lo hai aggiunto e rimane privato per te. Claude Code lo archivia in `~/.claude.json` nel percorso di quel progetto, quindi lo stesso server non apparirà nei tuoi altri progetti. Utilizza l'ambito locale per server di sviluppo personali, configurazioni sperimentali o server con credenziali che non desideri nel controllo della versione.

<Note>
  Il termine "ambito locale" per i server MCP differisce dalle impostazioni locali generali. I server MCP con ambito locale vengono archiviati in `~/.claude.json` (la tua directory home), mentre le impostazioni locali generali utilizzano `.claude/settings.local.json` (nella directory del progetto). Vedi [Impostazioni](/docs/it/settings#settings-files) per i dettagli sui percorsi dei file di impostazioni.
</Note>

```bash theme={null}
# Aggiungi un server con ambito locale (predefinito)
claude mcp add --transport http stripe https://mcp.stripe.com

# Specifica esplicitamente l'ambito locale
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

Il comando scrive il server nella voce per il tuo progetto corrente all'interno di `~/.claude.json`. L'esempio seguente mostra il risultato quando lo esegui da `/path/to/your/project`:

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  Ambito del progetto
</h3>

I server con ambito del progetto abilitano la collaborazione del team archiviando le configurazioni in un file `.mcp.json` nella directory radice del tuo progetto. Questo file è progettato per essere archiviato nel controllo della versione, assicurando che tutti i membri del team abbiano accesso agli stessi strumenti e servizi MCP. Quando aggiungi un server con ambito del progetto, Claude Code crea o aggiorna automaticamente questo file con la struttura di configurazione appropriata.

```bash theme={null}
# Aggiungi un server con ambito del progetto
claude mcp add --transport http paypal --scope project https://mcp.paypal.com/mcp
```

Il file `.mcp.json` risultante segue un formato standardizzato:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "command": "/path/to/server",
      "args": [],
      "env": {}
    }
  }
}
```

Per motivi di sicurezza, Claude Code richiede l'approvazione prima di utilizzare server con ambito del progetto dai file `.mcp.json`. Se devi ripristinare queste scelte di approvazione, utilizza il comando `claude mcp reset-project-choices`.

<h3 id="user-scope">
  Ambito utente
</h3>

I server con ambito utente vengono archiviati in `~/.claude.json` e forniscono accessibilità tra progetti, rendendoli disponibili in tutti i progetti sulla tua macchina mentre rimangono privati al tuo account utente. Questo ambito funziona bene per server di utilità personali, strumenti di sviluppo o servizi che utilizzi frequentemente in diversi progetti.

```bash theme={null}
# Aggiungi un server utente
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  Gerarchia e precedenza dell'ambito
</h3>

Quando lo stesso server è definito in più di un posto, Claude Code si connette ad esso una volta, utilizzando la definizione dalla fonte con la precedenza più alta. L'intera voce del server da quella fonte viene utilizzata; i campi non vengono uniti tra gli ambiti.

1. Ambito locale
2. Ambito del progetto
3. Ambito utente
4. [Server forniti da plugin](/docs/it/plugins)
5. [Connettori claude.ai](#use-mcp-servers-from-claude-ai)

I tre ambiti corrispondono ai duplicati per nome. I plugin e i connettori corrispondono per endpoint, quindi uno che punta allo stesso URL o comando di un server sopra è trattato come un duplicato.

<h3 id="environment-variable-expansion-in-mcp-json">
  Espansione delle variabili di ambiente in `.mcp.json`
</h3>

Claude Code supporta l'espansione delle variabili di ambiente nei file `.mcp.json`, consentendo ai team di condividere configurazioni mantenendo flessibilità per i percorsi specifici della macchina e i valori sensibili come le chiavi API.

**Sintassi supportata:**

* `${VAR}`: si espande al valore della variabile di ambiente `VAR`
* `${VAR:-default}`: si espande a `VAR` se impostato, altrimenti utilizza `default`

**Posizioni di espansione:**
Le variabili di ambiente possono essere espanse in:

* `command`: il percorso dell'eseguibile del server
* `args`: argomenti della riga di comando
* `env`: variabili di ambiente passate al server
* `url`: per i tipi di server HTTP
* `headers`: per l'autenticazione del server HTTP

**Esempio con espansione di variabili:**

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

Se una variabile di ambiente richiesta non è impostata e non ha un valore predefinito, Claude Code lascia il testo letterale `${VAR}` nel valore e segnala un avviso di variabile mancante per quel server. La configurazione viene comunque caricata, quindi imposta la variabile o aggiungi un fallback `:-default` in modo che il server si avvii con il valore che intendi.

<h2 id="practical-examples">
  Esempi pratici
</h2>

<h3 id="example-monitor-errors-with-sentry">
  Esempio: Monitora gli errori con Sentry
</h3>

```bash theme={null}
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

Autenticati con il tuo account Sentry:

```text theme={null}
/mcp
```

Quindi esegui il debug dei problemi di produzione:

```text theme={null}
Quali sono gli errori più comuni nelle ultime 24 ore?
```

```text theme={null}
Mostrami la stack trace per l'errore ID abc123
```

```text theme={null}
Quale deployment ha introdotto questi nuovi errori?
```

<h3 id="example-connect-to-github-for-code-reviews">
  Esempio: Connettiti a GitHub per le revisioni del codice
</h3>

Il server MCP remoto di GitHub si autentica con un token di accesso personale GitHub passato come header. Per ottenerne uno, apri le [impostazioni del token GitHub](https://github.com/settings/personal-access-tokens), genera un nuovo token con granularità fine con accesso ai repository con cui desideri che Claude lavori, quindi aggiungi il server:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Quindi lavora con GitHub:

```text theme={null}
Rivedi la PR #456 e suggerisci miglioramenti
```

```text theme={null}
Crea un nuovo issue per il bug che abbiamo appena trovato
```

```text theme={null}
Mostrami tutte le PR aperte assegnate a me
```

<h3 id="example-query-your-postgresql-database">
  Esempio: Interroga il tuo database PostgreSQL
</h3>

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Quindi interroga il tuo database naturalmente:

```text theme={null}
Qual è il nostro ricavo totale questo mese?
```

```text theme={null}
Mostrami lo schema per la tabella orders
```

```text theme={null}
Trova i clienti che non hanno effettuato un acquisto negli ultimi 90 giorni
```

<h2 id="authenticate-with-remote-mcp-servers">
  Autenticazione con server MCP remoti
</h2>

Molti server MCP basati su cloud richiedono l'autenticazione. Claude Code supporta OAuth 2.0 per connessioni sicure.

Claude Code contrassegna un server remoto come richiedente autenticazione quando il server risponde con `401 Unauthorized` o `403 Forbidden`. Per un server a cui non hai effettuato l'accesso, uno di questi codici di stato lo contrassegna in `/mcp` in modo che tu possa completare il flusso OAuth.

Quando una richiesta a un server OAuth a cui hai già effettuato l'accesso restituisce `401 Unauthorized`, Claude Code aggiorna il token archiviato, si riconnette e ritenta la richiesta una volta. Contrassegna il server in `/mcp` solo se anche quel tentativo non riesce. Prima della v2.1.206, un aggiornamento del token che non riusciva per un motivo transitorio, come un errore di rete, contrassegnava un server OAuth come richiedente autenticazione per il resto della sessione anche se il suo token di aggiornamento era ancora valido.

A partire da v2.1.195, quando un aggiornamento del token non riesce perché il server rifiuta il token di aggiornamento archiviato, Claude Code mostra immediatamente un avviso che punta a `/mcp`. Il menu del server connesso lì offre Re-authenticate, in modo che tu possa accedere di nuovo prima che la prossima chiamata dello strumento non riesca.

Un server personalizzato che restituisce un'intestazione `WWW-Authenticate` che punta al suo server di autorizzazione ottiene la stessa scoperta automatica di qualsiasi altro server remoto.

A partire da v2.1.193, Claude Code mostra anche un avviso di avvio quando uno o più server configurati richiedono l'autenticazione, quindi non è necessario aprire `/mcp` per scoprire quali server richiedono l'accesso.

In modalità non interattiva non c'è un pannello `/mcp`, quindi Claude Code non può eseguire il flusso OAuth per te. A partire da v2.1.196, quando un server configurato richiede l'autenticazione durante un'esecuzione `claude -p` o Agent SDK con [ricerca degli strumenti](#scale-with-mcp-tool-search) abilitata, che è l'impostazione predefinita, Claude Code comunica a Claude che gli strumenti del server non sono disponibili fino a quando non lo autorizzi. Claude può quindi nominare il server che richiede l'accesso invece di rispondere come se il server non fosse configurato. Completa l'accesso da una sessione interattiva con `/mcp` o `claude mcp login <name>`.

Se hai configurato `headers.Authorization` per il server e il server rifiuta quell'intestazione, Claude Code segnala la connessione come non riuscita invece di ricorrere a OAuth. Verifica che il token sia valido per l'endpoint MCP, oppure rimuovi l'intestazione per utilizzare il flusso OAuth.

<Steps>
  <Step title="Aggiungi il server che richiede l'autenticazione">
    Per esempio:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Utilizza il comando /mcp all'interno di Claude Code">
    In Claude Code, utilizza il comando:

    ```text theme={null}
    /mcp
    ```

    Quindi segui i passaggi nel tuo browser per accedere.
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * I token di autenticazione vengono archiviati in modo sicuro e aggiornati automaticamente
  * Utilizza "Clear authentication" nel menu `/mcp` per revocare l'accesso
  * Se il tuo browser non si apre automaticamente, copia l'URL fornito e aprilo manualmente
  * Se il reindirizzamento del browser non riesce con un errore di connessione dopo l'autenticazione, incolla l'URL di callback completo dalla barra degli indirizzi del tuo browser nel prompt dell'URL che appare in Claude Code
  * L'autenticazione OAuth funziona con i server HTTP
</Tip>

<h3 id="authenticate-from-the-command-line">
  Autenticazione dalla riga di comando
</h3>

Da v2.1.186, `claude mcp login <name>` esegue il flusso OAuth di un server configurato direttamente dalla tua shell, quindi non è necessario aprire il pannello `/mcp` all'interno di una sessione.

```bash theme={null}
claude mcp login sentry
```

Per cancellare le credenziali archiviate in seguito, esegui `claude mcp logout <name>`.

A partire da v2.1.191, il comando rileva quando nessun browser locale è disponibile, ad esempio durante una sessione SSH o su Linux senza un server di visualizzazione, e stampa l'URL di autorizzazione invece di provare ad aprire un browser. Apri l'URL sulla tua macchina locale, quindi incolla l'URL di reindirizzamento completo dalla barra degli indirizzi del tuo browser al prompt. Il comando ha bisogno di un terminale interattivo per il passaggio di incollamento, quindi connettiti con `ssh -t`. Passa `--no-browser` per forzare il prompt dell'URL anche quando viene rilevato un browser locale.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Utilizza una porta di callback OAuth fissa
</h3>

Alcuni server MCP richiedono un URI di reindirizzamento specifico registrato in anticipo. Per impostazione predefinita, Claude Code sceglie una porta disponibile casuale per il callback OAuth. Utilizza `--callback-port` per fissare la porta in modo che corrisponda a un URI di reindirizzamento pre-registrato della forma `http://localhost:PORT/callback`.

Puoi utilizzare `--callback-port` da solo (con registrazione dinamica del client) o insieme a `--client-id` (con credenziali pre-configurate).

```bash theme={null}
# Porta di callback fissa con registrazione dinamica del client
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  Utilizza credenziali OAuth pre-configurate
</h3>

Alcuni server MCP non supportano la configurazione OAuth automatica tramite Dynamic Client Registration. Se vedi un errore come "Incompatible auth server: does not support dynamic client registration," il server richiede credenziali pre-configurate. Claude Code supporta anche server che utilizzano un Client ID Metadata Document (CIMD) invece di Dynamic Client Registration e li scopre automaticamente. Se la scoperta automatica non riesce, registra prima un'app OAuth tramite il portale degli sviluppatori del server, quindi fornisci le credenziali quando aggiungi il server.

<Steps>
  <Step title="Registra un'app OAuth con il server">
    Crea un'app tramite il portale degli sviluppatori del server e annota il tuo ID client e il segreto client.

    Molti server richiedono anche un URI di reindirizzamento. Se è così, scegli una porta e registra un URI di reindirizzamento nel formato `http://localhost:PORT/callback`. Utilizza quella stessa porta con `--callback-port` nel passaggio successivo.
  </Step>

  <Step title="Aggiungi il server con le tue credenziali">
    Scegli uno dei seguenti metodi. La porta utilizzata per `--callback-port` può essere qualsiasi porta disponibile. Deve solo corrispondere all'URI di reindirizzamento che hai registrato nel passaggio precedente.

    <Tabs>
      <Tab title="claude mcp add">
        Utilizza `--client-id` per passare l'ID client della tua app. Il flag `--client-secret` richiede il segreto con input mascherato:

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        Includi l'oggetto `oauth` nella configurazione JSON e passa `--client-secret` come flag separato:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json (solo porta di callback)">
        Utilizza `--callback-port` senza un ID client per fissare la porta mentre utilizzi la registrazione dinamica del client:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / env var">
        Imposta il segreto tramite variabile di ambiente per saltare il prompt interattivo:

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Autenticati in Claude Code">
    Esegui `/mcp` in Claude Code e segui il flusso di accesso del browser.
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Il segreto client viene archiviato in modo sicuro nel tuo portachiavi di sistema (macOS) o in un file di credenziali, non nella tua configurazione
  * Se il server utilizza un client OAuth pubblico senza segreto, utilizza solo `--client-id` senza `--client-secret`
  * `--callback-port` può essere utilizzato con o senza `--client-id`
  * Questi flag si applicano solo ai trasporti HTTP e SSE. Non hanno effetto sui server stdio
  * Utilizza `claude mcp get <name>` per verificare che le credenziali OAuth siano configurate per un server
</Tip>

<h3 id="override-oauth-metadata-discovery">
  Sovrascrivi la scoperta dei metadati OAuth
</h3>

Indirizza Claude Code a un URL di metadati OAuth specifico per bypassare la catena di scoperta predefinita. Imposta `authServerMetadataUrl` quando gli endpoint standard del server MCP generano errori, o quando desideri instradare la scoperta attraverso un proxy interno. Per impostazione predefinita, Claude Code controlla prima i metadati della risorsa protetta RFC 9728 su `/.well-known/oauth-protected-resource`, quindi ricade sui metadati del server di autorizzazione RFC 8414 su `/.well-known/oauth-authorization-server`.

Imposta `authServerMetadataUrl` nell'oggetto `oauth` della configurazione del tuo server in `.mcp.json`:

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

L'URL deve utilizzare `https://`. L'`scopes_supported` dell'URL dei metadati sovrascrive gli ambiti che il server upstream pubblicizza.

<h3 id="restrict-oauth-scopes">
  Limita gli ambiti OAuth
</h3>

Imposta `oauth.scopes` per fissare gli ambiti che Claude Code richiede durante il flusso di autorizzazione. Questo è il modo supportato per limitare un server MCP a un sottoinsieme approvato dal team di sicurezza quando il server di autorizzazione upstream pubblicizza più ambiti di quelli che desideri concedere. Il valore è una singola stringa separata da spazi, corrispondente al formato del parametro `scope` in RFC 6749 §3.3.

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` ha la precedenza sia su `authServerMetadataUrl` che sugli ambiti che il server scopre su `/.well-known`. Lascialo non impostato per consentire al server MCP di determinare l'insieme di ambiti richiesti.

A partire da v2.1.196, quando `oauth.scopes` non è impostato, Claude Code richiede l'ambito fornito dall'intestazione `WWW-Authenticate` del server o dai suoi metadati della risorsa protetta, e non invia alcun parametro `scope` quando nessuno dei due lo fornisce. Non richiede più il catalogo completo di `scopes_supported` dai metadati del server di autorizzazione scoperto automaticamente. La richiesta di quel catalogo ha fatto sì che i provider di identità che pubblicizzano ambiti solo amministratore o modello rifiutassero la richiesta di autorizzazione con un errore `invalid_scope`. I metadati recuperati da un `authServerMetadataUrl` configurato forniscono comunque il loro `scopes_supported` come ambiti richiesti.

Se il server di autorizzazione pubblicizza `offline_access` in `scopes_supported`, Claude Code lo aggiunge agli ambiti fissati in modo che il token di accesso possa essere aggiornato senza un nuovo accesso al browser.

Se il server successivamente restituisce un 403 `insufficient_scope` per una chiamata di strumento, Claude Code si autentica di nuovo con gli stessi ambiti fissati. Amplia `oauth.scopes` quando uno strumento di cui hai bisogno richiede un ambito al di fuori del pin.

<h3 id="use-dynamic-headers-for-custom-authentication">
  Utilizza intestazioni dinamiche per l'autenticazione personalizzata
</h3>

Se il tuo server MCP utilizza uno schema di autenticazione diverso da OAuth, come Kerberos, token di breve durata o un SSO interno, utilizza `headersHelper` per generare intestazioni di richiesta al momento della connessione. Claude Code esegue il comando e unisce il suo output alle intestazioni di connessione.

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

Il comando può anche essere inline:

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**Requisiti:**

* Il comando deve scrivere un oggetto JSON di coppie chiave-valore stringa su stdout
* Il comando viene eseguito in una shell con un timeout di 10 secondi, dalla directory di lavoro corrente della sessione. Utilizza un percorso assoluto o un comando su `PATH` per lo script
* Le intestazioni dinamiche sovrascrivono qualsiasi `headers` statico con lo stesso nome

L'helper viene eseguito di nuovo ad ogni connessione, all'avvio della sessione e alla riconnessione. Non c'è caching, quindi il tuo script è responsabile di qualsiasi riutilizzo di token.

A partire da v2.1.193, se una chiamata di strumento restituisce `401 Unauthorized` o `403 Forbidden`, Claude Code esegue automaticamente di nuovo l'helper, si riconnette con le intestazioni aggiornate e ritenta la chiamata una volta. Claude Code contrassegna il server come richiedente autenticazione in `/mcp` solo se anche quel tentativo non riesce.

Claude Code imposta queste variabili di ambiente quando esegue l'helper:

| Variabile                     | Valore                                                                                                                  |
| :---------------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | il nome del server MCP                                                                                                  |
| `CLAUDE_CODE_MCP_SERVER_URL`  | l'URL del server MCP                                                                                                    |
| `CLAUDE_PLUGIN_ROOT`          | la directory radice del plugin. Impostato solo quando un [plugin](/docs/it/plugins-reference#mcp-servers) fornisce il server |

Utilizza questi per scrivere un singolo script helper che serve più server MCP.

Per un server fornito da un plugin, l'helper viene eseguito anche con la sua directory di lavoro impostata sulla radice del plugin, quindi un percorso `headersHelper` relativo si risolve all'interno della directory del plugin piuttosto che rispetto alla directory di lavoro della sessione. Richiede Claude Code v2.1.195 o successiva.

Un `headersHelper` fornito da un plugin non può fare riferimento ai valori [`${user_config.*}`](/docs/it/plugins-reference#user-configuration) del plugin, perché il comando viene eseguito attraverso una shell. Claude Code segnala il server come non configurato correttamente con un [errore](/docs/it/errors#plugin-command-references-user-config) e non sostituisce il valore. Metti `${user_config.KEY}` nel campo `headers` del server, che non viene analizzato dalla shell, oppure fai in modo che lo script helper legga il valore dal suo ambiente o da un file di configurazione. Prima della v2.1.207, `headersHelper` sostituiva i valori `${user_config.*}`.

<Note>
  `headersHelper` esegue comandi shell arbitrari. Quando definito a livello di progetto o locale, viene eseguito solo dopo che accetti la finestra di dialogo di fiducia dell'area di lavoro.
</Note>

<h2 id="add-mcp-servers-from-json-configuration">
  Aggiungi server MCP dalla configurazione JSON
</h2>

Se hai una configurazione JSON per un server MCP, puoi aggiungerla direttamente:

<Steps>
  <Step title="Aggiungi un server MCP da JSON">
    ```bash theme={null}
    # Sintassi di base
    claude mcp add-json <name> '<json>'

    # Esempio: Aggiunta di un server HTTP con configurazione JSON
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # Esempio: Aggiunta di un server stdio con configurazione JSON
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # Esempio: Aggiunta di un server HTTP con credenziali OAuth pre-configurate
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="Verifica che il server sia stato aggiunto">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Assicurati che il JSON sia correttamente sfuggito nella tua shell
  * Il JSON deve conformarsi allo schema di configurazione del server MCP
  * Puoi utilizzare `--scope user` per aggiungere il server alla tua configurazione utente invece di quella specifica del progetto
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  Importa server MCP da Claude Desktop
</h2>

Se hai già configurato server MCP in Claude Desktop, puoi importarli:

<Steps>
  <Step title="Importa server da Claude Desktop">
    ```bash theme={null}
    # Sintassi di base 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="Seleziona quali server importare">
    Dopo aver eseguito il comando, vedrai una finestra di dialogo interattiva che ti consente di selezionare quali server desideri importare.
  </Step>

  <Step title="Verifica che i server siano stati importati">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

I nomi dei server aggiunti tramite i comandi `claude mcp` possono contenere solo lettere, numeri, trattini e caratteri di sottolineatura. Claude Desktop non applica questa restrizione, quindi un server Claude Desktop il cui nome contiene qualsiasi altro carattere, come uno spazio, non può essere importato. L'importazione segnala ogni nome che rifiuta e continua comunque a importare gli altri server che hai selezionato. Prima della versione 2.1.205, il primo nome non valido interrompeva l'importazione e nessuno dei server selezionati veniva aggiunto.

<Tip>
  Suggerimenti:

  * Questa funzionalità funziona solo su macOS e Windows Subsystem for Linux (WSL)
  * Legge il file di configurazione di Claude Desktop dalla sua posizione standard su quelle piattaforme
  * Utilizza il flag `--scope user` per aggiungere server alla tua configurazione utente
  * I server importati mantengono gli stessi nomi di Claude Desktop quando il nome contiene solo lettere, numeri, trattini e caratteri di sottolineatura. Claude Code segnala un server il cui nome contiene qualsiasi altro carattere e lo salta
  * Se server con gli stessi nomi esistono già, riceveranno un suffisso numerico (per esempio, `server_1`)
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  Utilizza server MCP da claude.ai
</h2>

Se hai effettuato l'accesso a Claude Code con un account [claude.ai](https://claude.ai), i server MCP che hai aggiunto in claude.ai, noti come [connectors](https://claude.com/docs/connectors), sono automaticamente disponibili in Claude Code:

<Steps>
  <Step title="Configura server MCP in claude.ai">
    Aggiungi server su [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Nei piani Team ed Enterprise, solo gli amministratori possono aggiungere server.
  </Step>

  <Step title="Autentica il server MCP">
    Completa eventuali passaggi di autenticazione richiesti in claude.ai.
  </Step>

  <Step title="Visualizza e gestisci i server in Claude Code">
    In Claude Code, utilizza il comando:

    ```text theme={null}
    /mcp
    ```

    I server da claude.ai appaiono nell'elenco con indicatori che mostrano che provengono da claude.ai.
  </Step>
</Steps>

A partire dalla v2.1.161, i connettori a cui non hai mai effettuato l'accesso sono compressi dietro una riga `Show unused connectors` alla fine della sezione claude.ai, in modo che un elenco fornito dall'organizzazione non riempia il pannello. Seleziona la riga per espanderli. Un connettore a cui hai effettuato l'accesso in precedenza rimane visibile anche quando attualmente necessita di una nuova autenticazione.

I connettori da claude.ai vengono recuperati solo quando il tuo [metodo di autenticazione](/docs/it/authentication#authentication-precedence) attivo è il tuo abbonamento claude.ai. Non vengono caricati quando `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `apiKeyHelper`, o un provider di terze parti come Amazon Bedrock o Google Cloud's Agent Platform è attivo, anche se in precedenza hai eseguito `/login`.

Se `/mcp` non elenca un connettore che hai aggiunto, esegui `/status` per confermare quale metodo di autenticazione è attivo, annulla l'impostazione di quella variabile di ambiente o rimuovi l'impostazione `apiKeyHelper`, quindi esegui `/login` per selezionare il tuo account claude.ai.

Un server che hai aggiunto in Claude Code ha [precedenza](#scope-hierarchy-and-precedence) rispetto a un connettore claude.ai che punta allo stesso URL. Quando ciò accade, `/mcp` elenca il connettore come nascosto e mostra come rimuovere il duplicato se preferisci utilizzare il connettore.

Alcuni connettori ospitati da Anthropic, come Microsoft 365, Gmail e Google Calendar, non supportano OAuth locale da Claude Code perché il provider di identità upstream accetta solo l'URL di reindirizzamento registrato da claude.ai. A partire dalla v2.1.162, l'autenticazione di uno di questi host in `/mcp` mostra un messaggio che ti indirizza a collegarlo in Impostazioni → Connettori su claude.ai. Una volta collegato lì, il connettore appare in Claude Code automaticamente.

<h3 id="organization-controls-on-connector-tools">
  Controlli organizzativi sugli strumenti dei connettori
</h3>

La tua organizzazione può impostare controlli per singolo strumento sui [connettori claude.ai](https://claude.com/docs/connectors). Claude Code legge queste impostazioni all'avvio e le applica localmente. Esegui `/mcp` per vedere quale impostazione si applica a ogni strumento su un connettore.

* **Strumento impostato su `ask`**: Claude Code richiede un'approvazione ad ogni chiamata con il motivo `Your organization requires approval for this tool`. La richiesta appare anche nelle [modalità di autorizzazione](/docs/it/permissions#permission-modes) `acceptEdits`, `auto` e `bypassPermissions`, e non offre mai un'opzione per ricordare la tua scelta. Le [regole di autorizzazione](/docs/it/permissions) che corrispondono allo strumento non saltano nemmeno la richiesta. In modalità `dontAsk`, che non richiede mai, Claude Code nega la chiamata.
* **Strumento impostato su `blocked`**: Claude Code filtra lo strumento prima che Claude lo veda, quindi non appare mai nell'elenco degli strumenti.

L'applicazione di questi controlli richiede Claude Code v2.1.129 o successivo. Le versioni precedenti ignorano le impostazioni e applicano il flusso di autorizzazione standard.

<h3 id="disable-claude-ai-connectors">
  Disabilita connettori claude.ai
</h3>

Per disabilitare i server MCP di claude.ai in Claude Code, imposta [`disableClaudeAiConnectors`](/docs/it/settings#available-settings) su `true` in qualsiasi ambito di impostazioni:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Questa impostazione utilizza la semantica any-source-true: `true` in qualsiasi fonte di impostazioni ha la precedenza. Un file `.claude/settings.json` del progetto archiviato può escludere un repository dai connettori cloud, ma un `false` a livello di progetto non può riabilitare i connettori che un `true` a livello di utente o di policy ha disabilitato. I server passati esplicitamente tramite `--mcp-config` non sono interessati.

Puoi anche impostare la variabile di ambiente `ENABLE_CLAUDEAI_MCP_SERVERS` su `false`, che ha lo stesso effetto per la sessione shell corrente:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Per bloccare singoli connettori claude.ai invece di tutti, aggiungili a [`deniedMcpServers`](/docs/it/managed-mcp) per nome o per modello di URL. Ad esempio, una voce `serverName` di `"claude.ai Slack"` blocca il connettore Slack. Per attivare o disattivare un connettore solo per il progetto corrente, utilizza il pannello `/mcp`.

<Note>
  Queste impostazioni lato client governano le sessioni locali di Claude Code. Nelle sessioni di [Claude Code sul web](/docs/it/claude-code-on-the-web), i connettori claude.ai sono forniti dall'host remoto e arrivano come voci esplicite `--mcp-config`, quindi `disableClaudeAiConnectors` non si applica lì. Gli URL dei connettori vengono anche riscritti attraverso il proxy della sessione, quindi un modello `serverUrl` di `deniedMcpServers` che punta all'URL del fornitore non corrisponderà. Gestisci quali connettori una sessione cloud può utilizzare dalle impostazioni dell'organizzazione claude.ai.
</Note>

<h2 id="use-claude-code-as-an-mcp-server">
  Utilizza Claude Code come server MCP
</h2>

Puoi utilizzare Claude Code stesso come server MCP a cui altre applicazioni possono connettersi:

```bash theme={null}
# Avvia Claude come server MCP stdio
claude mcp serve
```

Puoi utilizzarlo in Claude Desktop aggiungendo questa configurazione a claude\_desktop\_config.json:

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **Configurazione del percorso dell'eseguibile**: Il campo `command` deve fare riferimento all'eseguibile di Claude Code. Se il comando `claude` non è nel PATH del tuo sistema, dovrai specificare il percorso completo dell'eseguibile.

  Per trovare il percorso completo:

  ```bash theme={null}
  which claude
  ```

  Quindi utilizza il percorso completo nella tua configurazione:

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  Senza il percorso dell'eseguibile corretto, incontrerai errori come `spawn claude ENOENT`.
</Warning>

<Tip>
  Suggerimenti:

  * Il server fornisce accesso agli strumenti di Claude come View, Edit, LS, ecc.
  * In Claude Desktop, prova a chiedere a Claude di leggere file in una directory, fare modifiche e altro ancora.
  * Questo server MCP espone solo gli strumenti di Claude Code al tuo client MCP, quindi il tuo client è responsabile dell'implementazione della conferma dell'utente per le singole chiamate di strumenti.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Limiti di output MCP e avvisi
</h2>

Quando gli strumenti MCP producono output di grandi dimensioni, Claude Code aiuta a gestire l'utilizzo dei token per evitare di sovraccaricare il contesto della vostra conversazione:

* **Soglia di avviso di output**: Claude Code visualizza un avviso quando l'output di qualsiasi strumento MCP supera 10.000 token
* **Limite configurabile**: potete regolare il massimo di token di output MCP consentiti utilizzando la variabile di ambiente `MAX_MCP_OUTPUT_TOKENS`
* **Limite predefinito**: il massimo predefinito è 25.000 token
* **Ambito**: la variabile di ambiente si applica agli strumenti che non dichiarano il loro limite. Gli strumenti che impostano [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) utilizzano quel valore invece per il contenuto di testo, indipendentemente da ciò che `MAX_MCP_OUTPUT_TOKENS` è impostato. Gli strumenti che restituiscono dati di immagine sono ancora soggetti a `MAX_MCP_OUTPUT_TOKENS`

Per aumentare il limite per gli strumenti che producono output di grandi dimensioni:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

Questo è particolarmente utile quando si lavora con server MCP che:

* interrogano grandi set di dati o database
* generano report o documentazione dettagliati
* elaborano file di log estesi o informazioni di debug

<h3 id="raise-the-limit-for-a-specific-tool">
  Aumenta il limite per uno strumento specifico
</h3>

Se state costruendo un server MCP, potete consentire ai singoli strumenti di restituire risultati più grandi della soglia predefinita impostando `_meta["anthropic/maxResultSizeChars"]` nella voce dello strumento nella risposta `tools/list`. Claude Code aumenta la soglia di quello strumento al valore annotato, fino a un limite massimo di 500.000 caratteri.

Questo è utile per strumenti che restituiscono output intrinsecamente grandi ma necessari, come schemi di database o alberi di file completi. Senza l'annotazione, i risultati che superano la soglia predefinita vengono persistiti su disco e sostituiti con un riferimento a file nella conversazione.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

L'annotazione si applica indipendentemente da `MAX_MCP_OUTPUT_TOKENS` per il contenuto di testo, quindi gli utenti non hanno bisogno di aumentare la variabile di ambiente per gli strumenti che la dichiarano. Gli strumenti che restituiscono dati di immagine sono ancora soggetti al limite di token.

<Warning>
  Se incontrate frequentemente avvisi di output con server MCP specifici che non controllate, considerate di aumentare il limite `MAX_MCP_OUTPUT_TOKENS`. Potete anche chiedere all'autore del server di aggiungere l'annotazione `anthropic/maxResultSizeChars` o di impaginare le loro risposte. L'annotazione non ha effetto sugli strumenti che restituiscono contenuto di immagine; per quelli, aumentare `MAX_MCP_OUTPUT_TOKENS` è l'unica opzione.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Schemi di input degli strumenti con un combinatore a livello di radice
</h2>

Alcuni server MCP dichiarano lo schema di input di uno strumento come un'unione JSON Schema, con `anyOf`, `oneOf`, o `allOf` al livello superiore dello schema. L'API Claude non accetta queste parole chiave alla radice dello schema. Accetta combinatori annidati all'interno di `properties`, che Claude Code invia senza modifiche.

A partire da Claude Code v2.1.195, gli strumenti con un combinatore a livello di radice rimangono disponibili. Prima di inviare lo strumento all'API, Claude Code appiattisce lo schema in un singolo oggetto e antepone una frase alla descrizione dello strumento che dice a Claude quali gruppi di parametri appartengono insieme:

* `allOf`: le proprietà di ogni ramo vengono unite, e l'elenco `required` di ogni ramo si applica ancora
* `anyOf` e `oneOf`: le proprietà di ogni ramo vengono unite, e l'elenco `required` di ogni ramo viene descritto nella descrizione dello strumento invece di essere applicato dallo schema

Il tuo server riceve gli argomenti che Claude ha scelto, quindi continua a convalidare la combinazione lato server.

Quando Claude Code non può produrre uno schema che l'API accetta, o su una distribuzione che non riceve la configurazione remota che abilita la riscrittura, come una macchina offline, salta quello strumento, registra il motivo nel log del server, e lascia gli altri strumenti del server disponibili. Le versioni precedenti a v2.1.195 saltano ogni strumento il cui schema di input ha un `anyOf`, `oneOf`, o `allOf` a livello di radice.

<h2 id="require-approval-for-a-specific-tool">
  Richiedi approvazione per uno strumento specifico
</h2>

Se stai costruendo un server MCP, puoi contrassegnare uno strumento come richiedente approvazione esplicita ad ogni chiamata impostando `_meta["anthropic/requiresUserInteraction"]` su `true` nella voce dello strumento nella risposta `tools/list`. Il valore deve essere il booleano JSON `true`; qualsiasi altro valore viene ignorato.

Claude Code mostra il prompt di autorizzazione di quello strumento ad ogni chiamata, anche in modalità `acceptEdits`, `auto`, e `bypassPermissions` [permission modes](/docs/it/permissions#permission-modes), e non offre un'opzione "non chiedere di nuovo" per esso. Le [regole di autorizzazione](/docs/it/permissions#permission-rule-syntax) che corrispondono allo strumento non saltano il prompt neanche. In modalità `dontAsk`, che non chiede mai, Claude Code nega la chiamata invece.

Il prompt deve raggiungere una persona. In modalità non interattiva con [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags), un risultato `allow` dal prompt tool per uno strumento contrassegnato viene convertito in un deny con il messaggio `MCP tool requires user interaction; not supported via --permission-prompt-tool`. Il callback [`canUseTool`](/docs/it/agent-sdk/permissions) dell'Agent SDK riceve queste chiamate e può approvarle, perché l'host SDK dovrebbe mostrarle a un utente.

Utilizza questo per strumenti il cui prompt di autorizzazione è il punto stesso, come un passaggio di consenso o concessione di accesso dove l'approvazione automatica significherebbe che nessun umano ha mai accettato. Gli altri strumenti dello stesso server mantengono il loro comportamento di autorizzazione normale.

La seguente voce `tools/list` contrassegna uno strumento come sempre richiedente approvazione.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

L'annotazione `anthropic/requiresUserInteraction` richiede Claude Code v2.1.199 o successiva. Le versioni precedenti la ignorano e applicano il flusso di autorizzazione standard.

Quando una sessione è connessa a [Remote Control](/docs/it/remote-control) o a un host SDK, Claude Code contrassegna il prompt di autorizzazione come richiedente interazione dell'utente, in modo che il client mostri il prompt di autorizzazione dello strumento per te di rispondere invece di un'azione di approvazione con un tocco.

<h2 id="respond-to-mcp-elicitation-requests">
  Rispondi alle richieste di elicitazione MCP
</h2>

I server MCP possono richiedere input strutturato da te durante un'attività utilizzando l'elicitazione. Quando un server ha bisogno di informazioni che non può ottenere da solo, Claude Code visualizza una finestra di dialogo interattiva e passa la tua risposta al server. Non è richiesta alcuna configurazione da parte tua: le finestre di dialogo di elicitazione appaiono automaticamente quando un server le richiede.

I server possono richiedere input in due modi:

* **Modalità modulo**: Claude Code mostra una finestra di dialogo con campi modulo definiti dal server (per esempio, un prompt di nome utente e password). Compila i campi e invia.
* **Modalità URL**: Claude Code apre un URL del browser per l'autenticazione o l'approvazione. Completa il flusso nel browser, quindi conferma nella CLI.

Per rispondere automaticamente alle richieste di elicitazione senza mostrare una finestra di dialogo, utilizza l'[hook `Elicitation`](/docs/it/hooks#elicitation).

Se stai costruendo un server MCP che utilizza l'elicitazione, vedi la [specifica di elicitazione MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) per i dettagli del protocollo e gli esempi di schema.

<h2 id="use-mcp-resources">
  Utilizza risorse MCP
</h2>

I server MCP possono esporre risorse che puoi referenziare utilizzando menzioni @, simile a come referenzi i file.

<h3 id="reference-mcp-resources">
  Referenzia risorse MCP
</h3>

<Steps>
  <Step title="Elenca le risorse disponibili">
    Digita `@` nel tuo prompt per vedere le risorse disponibili da tutti i server MCP connessi. Le risorse appaiono insieme ai file nel menu di completamento automatico.
  </Step>

  <Step title="Referenzia una risorsa specifica">
    Utilizza il formato `@server:protocol://resource/path` per referenziare una risorsa:

    ```text theme={null}
    Puoi analizzare @github:issue://123 e suggerire una correzione?
    ```

    ```text theme={null}
    Per favore rivedi la documentazione API su @docs:file://api/authentication
    ```
  </Step>

  <Step title="Referenze di risorse multiple">
    Puoi referenziare più risorse in un singolo prompt:

    ```text theme={null}
    Confronta @postgres:schema://users con @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Le risorse vengono recuperate automaticamente e incluse come allegati quando referenziate
  * I percorsi delle risorse sono ricercabili con fuzzy search nel completamento automatico della menzione @
  * Claude Code fornisce automaticamente strumenti per elencare e leggere risorse MCP quando i server le supportano
  * Le risorse possono contenere qualsiasi tipo di contenuto fornito dal server MCP (testo, JSON, dati strutturati, ecc.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Scala con MCP Tool Search
</h2>

Tool search mantiene l'utilizzo del contesto MCP basso rimandando le definizioni degli strumenti fino a quando Claude ne ha bisogno. Solo i nomi degli strumenti e le istruzioni del server vengono caricati all'avvio della sessione, quindi aggiungere più server MCP ha un impatto minimo sulla tua finestra di contesto. Claude Code non impone un limite fisso di strumenti per server; il limite pratico è il tuo budget della finestra di contesto.

<h3 id="how-it-works">
  Come funziona
</h3>

Tool search è abilitato per impostazione predefinita. Gli strumenti MCP vengono rimandati piuttosto che caricati nel contesto in anticipo, e Claude utilizza uno strumento di ricerca per scoprire quelli rilevanti quando un'attività ne ha bisogno. Solo gli strumenti che Claude effettivamente utilizza entrano nel contesto. Dal tuo punto di vista, gli strumenti MCP funzionano esattamente come prima.

Se preferisci il caricamento basato su soglia, imposta `ENABLE_TOOL_SEARCH=auto` per caricare gli schemi in anticipo quando si adattano entro il 10% della finestra di contesto e rimanda solo l'overflow. Vedi [Configura tool search](#configure-tool-search) per tutte le opzioni.

<h3 id="for-mcp-server-authors">
  Per gli autori di server MCP
</h3>

Se stai costruendo un server MCP, il campo delle istruzioni del server diventa più utile con tool search abilitato. Le istruzioni del server aiutano Claude a capire quando cercare i tuoi strumenti, simile a come funzionano le [skills](/docs/it/skills).

Aggiungi istruzioni del server chiare e descrittive che spieghino:

* Quale categoria di attività gestiscono i tuoi strumenti
* Quando Claude dovrebbe cercare i tuoi strumenti
* Capacità chiave che il tuo server fornisce

Claude Code tronca le descrizioni degli strumenti e le istruzioni del server a 2KB ciascuna. Mantienile concise per evitare il troncamento e metti i dettagli critici all'inizio.

<h3 id="configure-tool-search">
  Configura tool search
</h3>

Tool search è abilitato per impostazione predefinita: gli strumenti MCP vengono rimandati e scoperti su richiesta. Claude Code lo disabilita per impostazione predefinita su Google Cloud's Agent Platform. È anche disabilitato quando `ANTHROPIC_BASE_URL` punta a un host non di prima parte, poiché la maggior parte dei proxy non inoltrano blocchi `tool_reference`. Imposta `ENABLE_TOOL_SEARCH` esplicitamente per ignorare uno qualsiasi dei fallback.

Setting [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/it/env-vars) mantiene tool search disabilitato, e `ENABLE_TOOL_SEARCH` non può ignorarlo. La variabile rimuove l'intestazione beta che le definizioni degli strumenti `defer_loading` e i blocchi di contenuto `tool_reference` richiedono.

Tool search richiede un modello che supporta blocchi `tool_reference`: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 e modelli successivi. Vedi [compatibilità dei modelli nella documentazione API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) per l'elenco attuale. Su Google Cloud's Agent Platform, tool search è supportato per Claude Sonnet 4.5 e successivi e Claude Opus 4.5 e successivi.

Controlla il comportamento di tool search con la variabile di ambiente `ENABLE_TOOL_SEARCH`:

| Valore          | Comportamento                                                                                                                                                                                                                                                                                       |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (non impostato) | Tutti gli strumenti MCP rimandati e caricati su richiesta. Ricade al caricamento in anticipo su Google Cloud's Agent Platform o quando `ANTHROPIC_BASE_URL` è un host non di prima parte                                                                                                            |
| `true`          | Tutti gli strumenti MCP rimandati. Claude Code invia l'intestazione beta anche su Google Cloud's Agent Platform e attraverso i proxy. Le richieste non riescono su modelli Google Cloud's Agent Platform precedenti a Sonnet 4.5 o Opus 4.5, o su proxy che non supportano blocchi `tool_reference` |
| `auto`          | Modalità soglia: gli strumenti vengono caricati in anticipo se si adattano entro il 10% della finestra di contesto, rimandati altrimenti                                                                                                                                                            |
| `auto:N`        | Modalità soglia con una percentuale personalizzata, dove `N` è 0-100. Ad esempio, `auto:5` per il 5%                                                                                                                                                                                                |
| `false`         | Tutti gli strumenti MCP caricati in anticipo, nessun rinvio                                                                                                                                                                                                                                         |

```bash theme={null}
# Utilizza una soglia personalizzata del 5%
ENABLE_TOOL_SEARCH=auto:5 claude

# Disabilita completamente tool search
ENABLE_TOOL_SEARCH=false claude
```

Oppure imposta il valore nel campo `env` del tuo [settings.json](/docs/it/settings#available-settings).

Puoi anche disabilitare lo strumento `ToolSearch` specificamente:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Esentare un server dal rinvio
</h3>

Se gli strumenti di un server dovrebbero essere sempre visibili a Claude senza un passaggio di ricerca, imposta `alwaysLoad` su `true` nella configurazione di quel server. Ogni strumento da quel server viene quindi caricato nel contesto all'avvio della sessione indipendentemente dall'impostazione `ENABLE_TOOL_SEARCH`. Utilizza questa opzione per un piccolo numero di strumenti che Claude necessita ad ogni turno, poiché ogni strumento in anticipo consuma contesto che altrimenti sarebbe disponibile per la tua conversazione.

La seguente voce `.mcp.json` esentare un server HTTP mentre lascia gli altri server rimandati:

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

Il campo `alwaysLoad` è disponibile su tutti i tipi di server e richiede Claude Code v2.1.121 o successivo. Un server MCP può anche contrassegnare singoli strumenti come sempre caricati includendo `"anthropic/alwaysLoad": true` nell'oggetto `_meta` dello strumento, che ha lo stesso effetto solo per quello strumento.

L'impostazione di `alwaysLoad: true` blocca anche l'avvio fino a quando il server non si connette, limitato al timeout di connessione standard di 5 secondi. Questo si applica anche se MCP startup è altrimenti [non-blocking per impostazione predefinita](/docs/it/env-vars), poiché gli strumenti devono essere presenti quando viene costruito il primo prompt. Gli altri server continuano a connettersi in background.

<h2 id="use-mcp-prompts-as-commands">
  Utilizza i prompt MCP come comandi
</h2>

I server MCP possono esporre prompt che diventano disponibili come comandi in Claude Code.

<h3 id="execute-mcp-prompts">
  Esegui i prompt MCP
</h3>

<Steps>
  <Step title="Scopri i prompt disponibili">
    Digita `/` per vedere tutti i comandi disponibili, inclusi quelli dai server MCP. I prompt MCP appaiono con il formato `/mcp__servername__promptname`.
  </Step>

  <Step title="Esegui un prompt senza argomenti">
    ```text theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Esegui un prompt con argomenti">
    Molti prompt accettano argomenti. Passali separati da spazi dopo il comando:

    ```text theme={null}
    /mcp__github__pr_review 456
    ```

    ```text theme={null}
    /mcp__jira__create_issue "Bug nel flusso di accesso" high
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * I prompt MCP vengono scoperti dinamicamente dai server connessi
  * Gli argomenti vengono analizzati in base ai parametri definiti del prompt
  * I risultati del prompt vengono iniettati direttamente nella conversazione
  * I nomi del server e del prompt vengono normalizzati, con gli spazi convertiti in trattini bassi
</Tip>

<h2 id="managed-mcp-configuration">
  Configurazione MCP gestita
</h2>

Per le organizzazioni che necessitano di un controllo centralizzato su quali server MCP gli utenti possono connettere, consulta [Configurazione MCP gestita](/docs/it/managed-mcp). Copre la distribuzione di un set fisso di server con `managed-mcp.json`, la restrizione dei server con `allowedMcpServers` e `deniedMcpServers`, e cosa vedono gli utenti quando un server è bloccato.
