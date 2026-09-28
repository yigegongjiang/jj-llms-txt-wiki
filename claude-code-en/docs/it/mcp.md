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

    Se l'installazione non riesce, fai corrispondere il messaggio che Claude Code segnala:

    * `Marketplace "claude-plugins-official" non trovato`: aggiungi il marketplace con `/plugin marketplace add anthropics/claude-plugins-official`, quindi riprova l'installazione.
    * Il plugin [non è trovato nel marketplace](/docs/it/plugins/install#install-a-plugin): controlla il nome del plugin.

    Se il riepilogo dell'installazione segnala `Run /reload-plugins to activate.`, Claude Code esegue quindi quel ricaricamento per te. Se il ricaricamento avverte che il tuo prossimo messaggio rileggerebbe la conversazione, esegui `/reload-plugins --force`.
  </Step>

  <Step title="Esegui lo skill di compilazione">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude ti chiede informazioni sul tuo caso d'uso e scaffolda un server HTTP remoto o un server stdio locale.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  Installazione di server MCP
</h2>

I server MCP possono essere configurati in diversi modi a seconda delle vostre esigenze:

<h3 id="option-1-add-a-remote-http-server">
  Opzione 1: Aggiungere un server HTTP remoto
</h3>

I server HTTP sono l'opzione consigliata per connettersi a server MCP remoti. Questo è il trasporto più ampiamente supportato per i servizi basati su cloud.

```bash theme={null}
# Sintassi di base
claude mcp add --transport http <name> <url>

# Esempio reale: Connessione a Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Esempio con token Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Quando si configurano server MCP tramite JSON in `.mcp.json`, `~/.claude.json`, o `claude mcp add-json`, il campo `type` accetta `streamable-http` come alias per `http`. La specifica MCP utilizza il nome `streamable-http` per questo trasporto, quindi le configurazioni copiate dalla documentazione del server funzionano senza modifiche.

Una voce JSON che ha un `url` ma nessun `type` è un errore di configurazione, perché Claude Code legge una voce senza `type` come server stdio. Claude Code salta quel server e segnala `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Prima della v2.1.202, Claude Code segnalava questa configurazione errata come `command: expected string, received undefined`.

Nelle esecuzioni `--output-format stream-json`, Claude Code segnala anche una voce `--mcp-config` saltata nell'evento `system/init` nel campo [`mcp_server_errors`](/docs/it/headless#stream-responses), in modo che gli script possano rilevare che il server non è mai stato caricato. Questo richiede Claude Code v2.1.219 o successivo.

<h3 id="option-2-add-a-remote-sse-server">
  Opzione 2: Aggiungere un server SSE remoto
</h3>

<Warning>
  Il trasporto SSE (Server-Sent Events) è deprecato. Utilizza server HTTP invece, dove disponibili.
</Warning>

Alcuni servizi espongono ancora solo un endpoint SSE. Aggiungili con lo stesso comando `claude mcp add --transport http <name> <url>` di [un server HTTP](#option-1-add-a-remote-http-server). Claude Code prova prima il trasporto HTTP e passa a SSE quando il server non lo accetta. Il passaggio automatico richiede Claude Code v2.1.265 o successivo.

Su una versione precedente, o per connettersi direttamente su SSE, passa `--transport sse` invece:

```bash theme={null}
# Sintassi di base
claude mcp add --transport sse <name> <url>

# Esempio reale: Connessione ad Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Esempio con intestazione di autenticazione
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Opzione 3: Aggiungere un server stdio locale
</h3>

I server stdio vengono eseguiti come processi locali sulla vostra macchina. Sono ideali per strumenti che necessitano di accesso diretto al sistema o script personalizzati.

Claude Code imposta `CLAUDE_PROJECT_DIR` nell'ambiente del server generato alla radice del progetto, in modo che il vostro server possa risolvere i percorsi relativi al progetto senza dipendere dalla directory di lavoro. Questa è la stessa directory che gli hook ricevono nella loro variabile `CLAUDE_PROJECT_DIR`. Leggetela dall'interno del processo del vostro server, ad esempio `process.env.CLAUDE_PROJECT_DIR` in Node o `os.environ["CLAUDE_PROJECT_DIR"]` in Python.

`CLAUDE_PROJECT_DIR` è la radice del progetto stabile e non cambia quando aggiungete o rimuovete directory di lavoro a metà sessione. Un server che limita il proprio accesso al file system a un insieme di directory consentite dovrebbe implementare la richiesta MCP `roots/list`. Claude Code risponde a `roots/list` con la directory di avvio della sessione più ogni [directory di lavoro aggiuntiva](/docs/it/permissions#working-directories) che avete concesso con `--add-dir`, `/add-dir`, o l'impostazione `additionalDirectories`. Claude Code invia `notifications/roots/list_changed` quando quel set cambia. Prima della v2.1.203, `roots/list` restituiva solo la directory di avvio e Claude Code non inviava `notifications/roots/list_changed`.

Questa variabile è impostata nell'ambiente del server, non nell'ambiente di Claude Code stesso, quindi farvi riferimento tramite l'espansione `${VAR}` nel `command` o `args` di una voce `.mcp.json` con ambito di progetto o una voce server con ambito locale o utente in `~/.claude.json` richiede un valore predefinito come `${CLAUDE_PROJECT_DIR:-.}`. Le configurazioni MCP fornite da plugin sostituiscono `${CLAUDE_PROJECT_DIR}` direttamente e non hanno bisogno del valore predefinito.

```bash theme={null}
# Sintassi di base
claude mcp add [options] <name> -- <command> [args...]

# Esempio reale: Aggiungere server Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Importante: Separare gli argomenti del server con `--`**

  Per i server stdio, il `--` (doppio trattino) separa le opzioni di Claude, come `--transport`, `--env`, e `--scope`, dal comando e dagli argomenti che eseguono il server. Tutto ciò che viene dopo `--` viene passato al server senza modifiche.

  Ad esempio:

  * `claude mcp add --transport stdio myserver -- npx server` → esegue `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → esegue `python server.py --port 8080` con `KEY=value` nell'ambiente

  Senza `--`, Claude Code cercherebbe di analizzare i flag del server, come `--port` sopra, come le sue stesse opzioni.

  `--env` accetta più coppie `KEY=value`. Se il nome del server viene direttamente dopo `--env`, la CLI legge il nome come un'altra coppia e lo rifiuta, quindi posizionate almeno un'altra opzione, come `--transport stdio`, tra `--env` e il nome del server.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Opzione 4: Aggiungere un server WebSocket remoto
</h3>

I server WebSocket mantengono una connessione bidirezionale persistente, che si adatta ai server MCP remoti che inviano eventi a Claude senza sollecitazione. Utilizzate HTTP quando il vostro server risponde solo alle richieste, poiché HTTP supporta OAuth e il flag `claude mcp add --transport`, mentre WebSocket non supporta nessuno dei due.

Configurate i server WebSocket in `.mcp.json` o con `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

La voce `type: "ws"` accetta gli stessi campi `url`, `headers`, `headersHelper`, `timeout`, e `alwaysLoad` di `http`. L'autenticazione è solo tramite intestazione, quindi passate un token statico in `headers` o generatene uno al momento della connessione con [`headersHelper`](#use-dynamic-headers-for-custom-authentication). Il flag `claude mcp add --transport` non accetta `ws`.

<h3 id="add-a-server-from-setup-instructions-written-for-another-client">
  Aggiungere un server da istruzioni di configurazione scritte per un altro client
</h3>

I server MCP non sono specifici di Claude Code, quindi le istruzioni di configurazione di un server potrebbero essere scritte per Claude Desktop, Cursor, o un altro client MCP e non fornire alcun comando `claude mcp add`. Per aggiungere comunque il server, cercate in quelle istruzioni un URL, un comando di avvio, o un blocco JSON:

* **Un URL** come `https://mcp.example.com/mcp`: il server è remoto.
* **Un comando di avvio** come `npx -y @example/mcp-server`: il server viene eseguito sulla vostra macchina.
* **Un blocco JSON `mcpServers`**: configurazione scritta per il file di impostazioni di un altro client.

Ognuno di questi è uno degli input che le quattro opzioni in [Installazione di server MCP](#installing-mcp-servers) accettano. Trovate la forma che avete di seguito per trasformarla nel comando che Claude Code accetta. Ogni comando scrive nell'[ambito locale](#local-scope) a meno che non aggiungiate `--scope project` o `--scope user`.

<h4 id="from-a-url">
  Da un URL
</h4>

Un URL significa che il server è remoto. Per un endpoint `https://`, aggiungetelo con `--transport http`, o seguite [Opzione 2](#option-2-add-a-remote-sse-server) quando le istruzioni dicono che l'endpoint utilizza SSE. Per un endpoint `wss://`, utilizzate [Opzione 4](#option-4-add-a-remote-websocket-server) invece, poiché `--transport` non accetta `ws`:

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

Se le istruzioni forniscono anche una chiave API o un'intestazione di token, passatela con `--header` come mostrato in [Opzione 1](#option-1-add-a-remote-http-server).

<h4 id="from-an-npx-uvx-or-binary-command">
  Da un comando `npx`, `uvx`, o binario
</h4>

Un comando di avvio significa che il server viene eseguito come processo stdio locale. Mettete l'intero comando dopo `--`, in modo che Claude Code passi flag come `-y` al comando che avvia il server invece di leggerli come le sue stesse opzioni. Passate tutte le variabili di ambiente che le istruzioni richiedono con `--env`, dopo il nome del server e prima di `--`:

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

[Opzione 3](#option-3-add-a-local-stdio-server) copre il separatore `--` completamente.

<h4 id="from-an-mcpservers-json-block">
  Da un blocco JSON `mcpServers`
</h4>

Un blocco `mcpServers` scritto per un altro client MCP, come Claude Desktop, utilizza la chiave wrapper e la forma di voce che Claude Code legge. Passate a `claude mcp add-json` l'oggetto all'interno di `mcpServers`, non il wrapper. Due voci hanno bisogno di una riparazione prima:

* **Un `url` senza `type`**: aggiungete `"type": "http"`, `"type": "sse"`, o `"type": "ws"` per corrispondere all'endpoint. Claude Code legge una voce senza `type` come server stdio, quindi una voce `url` senza `type` fallisce.
* **Una chiave con caratteri diversi da lettere, numeri, trattini e sottolineature**: scegliete un nome di server che utilizza solo quei caratteri. Altrimenti la chiave è il nome del server.

Ad esempio, questo blocco:

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

diventa questo comando:

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[Aggiungere server MCP da configurazione JSON](#add-mcp-servers-from-json-configuration) copre l'escape della shell e il flag `--scope` per `add-json`. Per condividere il server con il vostro team, aggiungete `--scope project`, o aggiungete la voce sotto `mcpServers` in `.mcp.json` alla radice del vostro progetto e committala. [Ambito di progetto](#project-scope) copre come Claude Code carica e approva quel file.

Ogni comando `claude mcp add` e `claude mcp add-json` stampa una riga `Added ...`. Per verificare che Claude Code si sia connesso, eseguite `claude mcp get <name>`; [Stato del server](#server-status) copre gli stati che mostra e il passaggio di approvazione per i server `.mcp.json`.

<h3 id="managing-your-servers">
  Gestione dei vostri server
</h3>

Una volta configurati, potete gestire i vostri server MCP con questi comandi:

```bash theme={null}
# Elencare tutti i server configurati
claude mcp list

# Ottenere dettagli per un server specifico
claude mcp get notion

# Rimuovere un server
claude mcp remove notion

# (all'interno di Claude Code) Controllare lo stato del server
/mcp
```

Quando rimuovete un server remoto, Claude Code elimina anche i token OAuth e la registrazione del client che ha memorizzato per quel server.

<h4 id="server-status">
  Stato del server
</h4>

`claude mcp add` conferma un'aggiunta riuscita stampando una riga `Added ...`, il che significa che la configurazione è stata scritta. `claude mcp list` mostra quindi uno stato di salute accanto a ogni server che elenca, come `✔ Connected`, `! Needs authentication`, o `✘ Failed to connect`. Uno stato di fallimento significa che Claude Code non poteva connettersi a quel server, non che il comando list sia fallito.

Gli stati in questo elenco segnalano una decisione di configurazione piuttosto che un tentativo di connessione, quindi Claude Code li stampa senza connettersi al server:

* ``⏸ Pending approval (run `claude` to approve)``: un server con ambito di progetto da `.mcp.json` che non avete ancora approvato. Claude Code lo mostra sia in `claude mcp list` che in `claude mcp get <name>`. Eseguite `claude` in modo interattivo per rivederlo e approvarlo.
* `✘ Rejected (see disabledMcpjsonServers in settings)`: un server `.mcp.json` che una voce [`disabledMcpjsonServers`](/docs/it/settings-reference#disabledmcpjsonservers) rifiuta. Claude Code lo mostra solo in `claude mcp get <name>`.
* `⊘ Disabled for this project (re-enable via /mcp)`: un server che l'elenco [`disabledMcpServers`](#disable-a-server-without-removing-it) del progetto nomina. Claude Code lo mostra sia in `claude mcp list` che in `claude mcp get <name>`. Riattivate il server dal pannello `/mcp`. Prima della v2.1.238, entrambi i comandi si connettevano a un server disabilitato per verificarne lo stato di salute e segnalano il risultato della connessione.

I server WebSocket non appaiono nell'output di `claude mcp list`. Utilizzate `claude mcp get <name>` o il pannello `/mcp` per controllarli.

<h4 id="project-server-approvals-and-workspace-trust">
  Approvazioni del server di progetto e fiducia dell'area di lavoro
</h4>

A partire dalla v2.1.196, `claude mcp list` e `claude mcp get` leggono le approvazioni `.mcp.json` solo dai file di impostazioni che non sono sottoposti a commit nel repository finché non fidate dell'area di lavoro eseguendo `claude` in essa e accettando la finestra di dialogo di fiducia dell'area di lavoro. Un repository clonato non può approvare i suoi stessi server: [`enableAllProjectMcpServers`](/docs/it/settings-reference#enableallprojectmcpservers) o [`enabledMcpjsonServers`](/docs/it/settings-reference#enabledmcpjsonservers) sottoposti a commit nel `.claude/settings.json` del progetto vengono ignorati in una cartella non attendibile, e il server rimane a `⏸ Pending approval` invece di essere connesso e verificato.

Le approvazioni da queste fonti si applicano ancora in una cartella non attendibile:

* il vostro `~/.claude/settings.json` utente
* impostazioni gestite
* impostazioni passate con `--settings`

Claude Code applica anche le approvazioni da un `.claude/settings.local.json` non tracciato, ma esegue git per verificare se il file è tracciato, ed esegue quel controllo solo in una [cartella attendibile](/docs/it/permissions#project-allow-rules-and-workspace-trust). In una cartella che non avete mai fidato, Claude Code attende la finestra di dialogo di fiducia prima di applicare le approvazioni del file, a meno che la cartella non sia la vostra home di configurazione: la vostra home directory, o una directory il cui `.claude` avete impostato come [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars). Prima della v2.1.207, Claude Code applicava le approvazioni da un `.claude/settings.local.json` non tracciato anche in una cartella che non avevate mai fidato.

Una voce `disabledMcpjsonServers` in qualsiasi file di impostazioni rifiuta comunque il server.

<h4 id="server-status-detail">
  Dettaglio dello stato del server
</h4>

In `/mcp`, incluso il menu di un server lì, e nel gestore [`/plugin`](/docs/it/plugins/install), un server HTTP o SSE remoto che avete usato prima può mostrare uno stato `cached` come `cached 2h ago · connects on first use · 5 tools`. Claude Code ha caricato l'elenco degli strumenti del server dalla sua cache di scoperta, salvata in una sessione precedente, invece di connettersi all'avvio, e Claude Code connette il server la prima volta che Claude chiama uno degli strumenti del server. Gli strumenti sono disponibili dal vostro primo messaggio, quindi non dovete fare nulla. La cache di scoperta e il suo stato `cached` richiedono Claude Code v2.1.221 o successivo.

La cache di scoperta è disattivata per impostazione predefinita a meno che un rollout graduale non l'abbia abilitata per il vostro account. Impostate [`MCP_DISCOVERY_CACHE=1`](/docs/it/env-vars) per attivarla, o `0` per mantenerla disattivata anche quando il rollout l'ha abilitata. Prima della v2.1.238, la cache era attivata per impostazione predefinita.

Due azioni nel menu di un server in `/mcp` influenzano anche la voce della cache di quel server:

* **Riconnetti**: su un server `cached`, Claude Code lo connette ora piuttosto che alla sua prima chiamata di strumento e mantiene la voce. Su un server connesso o fallito, Claude Code lo riconnette e scarta anche la voce.
* **Cancella autenticazione**: Claude Code revoca l'autenticazione del server e scarta anche la voce.

Dopo aver scartato la voce, Claude Code recupera l'elenco degli strumenti del server dal server invece che dalla cache.

Quando lo stato di un server è `✘ Failed to connect`, `claude mcp list` aggiunge il dettaglio del fallimento a quella riga di stato, e `claude mcp get <name>` lo mostra su una riga `Issue:`: il codice di stato HTTP o il codice di errore, più qualsiasi testo di errore che il server ha restituito. La vista dei dettagli del server in `/mcp` include lo stesso testo segnalato dal server nella sua riga `Issue:`. Claude Code redige il testo simile a credenziali da questo dettaglio e non include mai l'URL del server espanso, che può contenere segreti. Claude Code non aggiunge dettagli a uno stato `✘ Connection error`, perché il testo dell'eccezione che stamperebbe lì può incorporare quell'URL. Prima della v2.1.219, entrambi i comandi mostravano solo lo stato di fallimento nudo, senza il codice di stato o il testo di errore del server.

Quando completate l'autenticazione da `/mcp` e la connessione fallisce ancora con uno stato HTTP o un codice di errore di trasporto, Claude Code aggiunge quel codice e l'origine dell'URL del server al messaggio che stampa dopo il tentativo. L'origine è lo schema e l'host, più la porta quando l'URL ne nomina una, come `https://mcp.example.com`.

* Il percorso e la query non appaiono mai in quel messaggio.
* Per un server nell'ambito locale, di progetto, o utente [scope](#mcp-installation-scopes) o nella configurazione MCP gestita, l'origine mostra l'host come scritto in quella configurazione, quindi un riferimento `${VAR}` nell'host non viene espanso nel messaggio.
* Per un fallimento senza codice di stato o di errore, Claude Code mostra il testo di errore senza l'origine.

Un server remoto la cui configurazione ha un `url` vuoto viene mostrato come `not configured` in `/mcp`, in `claude mcp list`, e nel gestore [`/plugin`](/docs/it/plugins/install), e Claude Code non tenta di connettersi ad esso. Un plugin può includere una voce segnaposto come questa per un connettore che configurate in seguito, quindi Claude Code non lo segnala come un errore o un problema di configurazione. La vista dei dettagli del server in `/mcp` legge `No URL configured for this server`; impostate l'`url` della voce per connetterla. Prima della v2.1.208, Claude Code segnalava un `url` vuoto come un problema di configurazione con un prompt per riconnettersi.

<h4 id="configuration-warnings">
  Avvisi di configurazione
</h4>

Claude Code avverte sui problemi di configurazione di seguito. Ogni voce dice cosa Claude Code controlla e come cancellare l'avviso:

* **Spazi bianchi nascosti**: Claude Code avverte quando un valore di configurazione MCP contiene spazi bianchi nascosti iniziali o finali, che spesso provengono dall'incollamento di un token con una nuova riga finale. Claude Code controlla `command`, `url`, ogni voce `args`, e i valori e i nomi delle chiavi sotto `env` e `headers`. Claude Code mostra l'avviso nell'output di `claude mcp list` e in `/mcp`, nominando i campi interessati senza echeggiare i loro valori, ad esempio `Leading or trailing whitespace in: headers.Authorization`. Claude Code non taglia gli spazi bianchi e utilizza i valori esattamente come scritti, quindi modificate la configurazione per rimuoverli.
* **Stesso nome in più di un ambito**: se definite lo stesso nome di server in più di un [scope](#mcp-installation-scopes) con endpoint diversi, Claude Code avverte del conflitto nell'output di `claude mcp list` e in `/mcp`. Claude Code memorizza gli accessi OAuth per endpoint, quindi quando autenticate la definizione che si carica in un progetto, dovete comunque accedere separatamente in un progetto dove si carica una definizione diversa. Mantenete l'endpoint che desiderate e rimuovete gli altri con `claude mcp remove <name> --scope <scope>`. Nell'avviso, Claude Code cita l'endpoint di ogni ambito come scritto nella vostra configurazione, con i riferimenti [`${VAR}`](#environment-variable-expansion-in-mcp-json) non espansi, quindi non mostra mai un valore risolto come una chiave API.
* **Nomi riservati**: Claude Code riserva i nomi dei suoi server integrati, inclusi `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, e `Claude Browser`. Se la vostra configurazione definisce un server con un nome riservato, Claude Code lo salta al momento del caricamento e mostra un avviso chiedendovi di rinominarlo. `claude mcp add` rifiuta un nome riservato con un errore. `Claude Preview` e `Claude Browser` entrambi nominano il server integrato che il [pannello di anteprima dell'app desktop Claude Code](/docs/it/desktop#preview-your-app) utilizza. Prima della v2.1.205, `Claude Browser` non era riservato, quindi un server configurato dall'utente poteva registrarsi con quel nome.
* **Variabile di ambiente mancante**: se un riferimento [`${VAR}`](#environment-variable-expansion-in-mcp-json) nella configurazione di un server nomina una variabile che non è impostata e non ha `:-default`, Claude Code avverte nell'output di `claude mcp list` e in `/mcp`, nominando la variabile, e carica comunque il server con il testo `${VAR}` non espanso. Impostate la variabile o aggiungete un fallback `${VAR:-default}`. In un `url` e `headers` di un server remoto, alcune variabili di credenziali [leggono come vuote](#credential-variables-that-read-as-empty) invece, senza avviso.

<h4 id="tool-availability">
  Disponibilità degli strumenti
</h4>

Il pannello `/mcp` mostra il conteggio degli strumenti accanto a ogni server connesso e contrassegna i server che pubblicizzano la capacità degli strumenti ma non espongono strumenti.

Se la vostra richiesta ha bisogno di strumenti da un server che si sta ancora connettendo in background, Claude attende quel server prima di continuare. Come avviene l'attesa dipende dalla vostra configurazione:

* **Con [ricerca degli strumenti](#scale-with-mcp-tool-search), l'impostazione predefinita**: l'attesa avviene all'interno della chiamata `ToolSearch`.
* **Senza ricerca degli strumenti**: Claude utilizza lo strumento `WaitForMcpServers` invece. Le configurazioni senza ricerca degli strumenti includono un `ANTHROPIC_BASE_URL` personalizzato, `ENABLE_TOOL_SEARCH=false`, e un modello precedente alla generazione Claude 4.5 su Google Cloud's Agent Platform.
* **Su una distribuzione Microsoft Foundry [ospitata su Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)**: Claude inizia sul percorso di ricerca degli strumenti piuttosto che con `WaitForMcpServers`, poiché Claude Code scopre il rifiuto lato server della distribuzione solo dall'API. Dopo che Claude Code passa quella distribuzione al [caricamento anticipato](#scale-with-mcp-tool-search), gli strumenti da un server che finisce di connettersi diventano disponibili sulla richiesta successiva di Claude.

Con la ricerca degli strumenti abilitata, quando un server finisce di connettersi mentre Claude sta lavorando, Claude Code elenca i nomi degli strumenti del server a Claude sulla sua richiesta successiva nello stesso turno. Claude può quindi cercare e chiamare quegli strumenti senza attendere il vostro prossimo messaggio.

<h3 id="disable-a-server-without-removing-it">
  Disabilitare un server senza rimuoverlo
</h3>

Attivate/disattivate un server nel pannello `/mcp` per impedire a Claude Code di connettersi ad esso senza perdere la sua configurazione. Claude Code elenca comunque il server in `/mcp`, contrassegnato come disabilitato.

Quando attivate/disattivate un server, Claude Code registra la vostra scelta per progetto in `~/.claude.json`, in uno di due elenchi che coprono insiemi disgiunti di server:

* `disabledMcpServers`: un elenco di esclusione per server configurati dall'utente, server di plugin, server che la vostra organizzazione [fornisce tramite impostazioni gestite](/docs/it/managed-mcp#provide-servers-through-managed-settings), i connettori claude.ai che Claude Code [recupera da solo](#how-connectors-reach-claude-code), e server integrati che sono attivati per impostazione predefinita. Claude Code non si connette a un server che elencate qui. Quando disabilitate un connettore claude.ai con l'attivazione/disattivazione `/mcp` per progetto descritta in [Disabilitare connettori claude.ai](#disable-claude-ai-connectors), Claude Code lo scrive in questo elenco con il suo nome di visualizzazione, ad esempio `claude.ai Slack`.
* `enabledMcpServers`: un elenco di consenso per server integrati che sono disabilitati per impostazione predefinita, come `computer-use`. Claude Code si connette a un server disabilitato per impostazione predefinita solo quando lo elencate qui.

Claude Code consulta esattamente uno dei due elenchi per ogni server, quindi nessun elenco sostituisce l'altro. Se aggiungete un server regolare a `enabledMcpServers`, o un server integrato disabilitato per impostazione predefinita a `disabledMcpServers`, Claude Code ignora la voce.

`disabledMcpServers` e `enabledMcpServers` non sono correlati a [`enabledMcpjsonServers`](/docs/it/settings-reference#enabledmcpjsonservers) e [`disabledMcpjsonServers`](/docs/it/settings-reference#disabledmcpjsonservers), che controllano l'approvazione dei server definiti nel file `.mcp.json` di un progetto.

<h3 id="mcp-client-runtimes">
  Runtime client MCP
</h3>

Claude Code si connette ai server MCP attraverso uno di due runtime client. Il runtime v1 è costruito su MCP TypeScript SDK 1.x. Il runtime v2 è lo stesso codice su [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/), che aggiunge la revisione del protocollo MCP 2026-07-28. Il resto di questa pagina si applica a entrambi i runtime, tranne dove una sezione nomina il runtime v2.

Claude Code sceglie un runtime ogni volta che lo avviate e lo mantiene fino a quando non uscite. Nelle sessioni in cui [recupera flag di funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching), utilizza il runtime v2 su Claude Code v2.1.232 o successivo.

Nelle sessioni in cui non recupera flag di funzionalità, Claude Code utilizza il runtime v2 per impostazione predefinita su Claude Code v2.1.274 o successivo:

* Sessioni su Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform, o Microsoft Foundry, a meno che una piattaforma host che incorpora Claude Code non imposti [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/it/env-vars)
* Sessioni accedute tramite un [gateway di app Claude](/docs/it/claude-apps-gateway)
* Sessioni in cui disattivate la telemetria o il recupero dei flag di funzionalità, ad esempio con `DISABLE_TELEMETRY`

Su v2, Claude Code inoltre:

* Chiede ai server HTTP se supportano la revisione più recente, e la utilizza con quelli che lo fanno. Chiede anche ai server connettori claude.ai nelle sessioni in cui recupera flag di funzionalità. Per fargli chiedere ai server stdio, o ai server connettori in ogni sessione, impostate [`MCP_PROTOCOL_NEGOTIATION`](/docs/it/env-vars) su `auto`. Si connette a ogni altro server come v1 fa.
* Riceve notifiche `list_changed` dai server sulla revisione più recente su un [flusso che mantiene aperto](#notification-streams-on-the-v2-runtime).
* Non registra un server [channel](#push-messages-with-channels) che si connette sulla revisione più recente, perché quella revisione non può portare messaggi di canale.
* Fallisce un [accesso OAuth MCP](#authenticate-with-remote-mcp-servers) la cui risposta di autorizzazione nomina un emittente inaspettato.

Anthropic può mantenere un server specifico sul protocollo precedente, o fuori da quel flusso, con un flag di funzionalità che Claude Code recupera.

Per scegliere il runtime voi stessi, impostate [`MCP_SDK_GENERATION`](/docs/it/env-vars) su `v1` o `v2`. Per decidere se Claude Code chiede, impostate [`MCP_PROTOCOL_NEGOTIATION`](/docs/it/env-vars) su `auto` o `legacy`.

<h3 id="dynamic-tool-updates">
  Aggiornamenti dinamici degli strumenti
</h3>

Claude Code supporta le notifiche MCP `list_changed`, consentendo ai server MCP di aggiornare dinamicamente i loro strumenti, prompt, e risorse disponibili senza richiedere di disconnettersi e riconnettersi. Quando un server MCP invia una notifica `list_changed`, Claude Code aggiorna automaticamente le capacità disponibili da quel server.

Se una richiesta di aggiornamento fallisce, Claude Code mantiene gli strumenti, i prompt, e le risorse precedentemente scoperti del server fino a quando un aggiornamento successivo non riesce. Prima della v2.1.214, un errore transitorio durante l'aggiornamento sostituiva gli strumenti, i prompt, e le risorse del server con un elenco vuoto.

<h4 id="notification-streams-on-the-v2-runtime">
  Flussi di notifica sul runtime v2
</h4>

Sul [runtime v2](#mcp-client-runtimes), Claude Code riceve notifiche `list_changed` da un server sulla revisione del protocollo più recente su un flusso che mantiene aperto. Quando il flusso si chiude, Claude Code lo riapre, con due limiti:

* **Il flusso si chiude di nuovo entro 10 secondi**: Claude Code lo riapre fino a tre volte, quindi si ferma per quella connessione.
* **Il flusso rimane aperto più a lungo di 10 secondi, poi si chiude**, come i flussi agli host serverless comunemente fanno: dopo cinque riaperture in un'ora, Claude Code attende circa sei ore prima della prossima.

Fino a quando il flusso non si riapre, mantenete gli ultimi strumenti, prompt, e risorse recuperati del server. Per raccogliere i suoi cambiamenti più presto, riconnettete il server da `/mcp`.

<h3 id="automatic-reconnection">
  Riconnessione automatica
</h3>

Claude Code riconnette un server remoto che cade a metà sessione e ritenta la prima connessione di un server HTTP o SSE dopo un errore transitorio. I server stdio sono processi locali, e Claude Code non li riconnette automaticamente.

<h4 id="mid-session-drops-of-a-remote-server">
  Cadute a metà sessione di un server remoto
</h4>

Claude Code riconnette un server remoto caduto con backoff esponenziale: fino a cinque tentativi, iniziando con un ritardo di un secondo e raddoppiandolo ogni volta. Quello che vedete dipende da come state eseguendo Claude Code:

* **In una sessione interattiva**: `/mcp` mostra il server come in sospeso mentre Claude Code si riconnette. Dopo cinque tentativi falliti, Claude Code contrassegna il server come fallito, o come necessitante di autenticazione quando il server ha bisogno di autorizzazione di nuovo. Quando lo contrassegna come fallito, vedete una notifica `MCP server "<name>" disconnected · open /mcp to reconnect`. Potete ritentare manualmente da `/mcp`.
* **In esecuzioni [`claude -p`](/docs/it/headless) e sessioni [Agent SDK](/docs/it/agent-sdk/overview)**: Claude Code si riconnette sulla stessa pianificazione, senza pannello `/mcp` per mostrare i tentativi.

<h4 id="failed-first-connections">
  Connessioni iniziali fallite
</h4>

Quando la prima connessione di un server HTTP o SSE fallisce con un errore transitorio, come una risposta 5xx, una connessione rifiutata, o un timeout, Claude Code ritenta fino a tre volte. Se la connessione fallisce ancora, Claude Code contrassegna il server come fallito. Claude Code ritenta in questo modo all'avvio e quando un server viene aggiunto a metà sessione. Questo include un server che Claude Code aggiunge a una [sessione cloud](/docs/it/claude-code-on-the-web) dalla sua configurazione e un server che aggiungete con il metodo [`setMcpServers()`](/docs/it/agent-sdk/typescript) dell'Agent SDK.

Claude Code non ritenta in questi casi:

* La prima connessione di un server WebSocket
* Un errore di autenticazione o non trovato, perché richiede una modifica della configurazione per risolvere. Quando un [`headersHelper`](#use-dynamic-headers-for-custom-authentication) è l'unica fonte dell'intestazione `Authorization` del server, Claude Code ritenta comunque un errore di autenticazione, perché riesegue l'helper ad ogni tentativo e può raccogliere una credenziale fresca

<h4 id="failed-discovery-requests">
  Richieste di scoperta fallite
</h4>

Dopo che un server si connette, Claude Code gli invia richieste di scoperta delle capacità come `tools/list`, `prompts/list`, e `resources/list`. Claude Code ritenta quelle richieste fino a tre volte con backoff breve dopo un errore di rete o server transitorio. Non ritenta errori di autenticazione, risposte 4xx, o timeout delle richieste.

<h4 id="how-claude-learns-that-a-server-failed">
  Come Claude apprende che un server è fallito
</h4>

Se Claude Code dice a Claude di un server configurato che non si è connesso dipende dalla [ricerca degli strumenti](#scale-with-mcp-tool-search), che è attivata per impostazione predefinita:

* Con ricerca degli strumenti, Claude Code dice a Claude quale server è fallito e il suo errore di connessione, quindi Claude segnala il fallimento della connessione nella sua risposta. Claude Code include le stesse informazioni nei risultati di `ToolSearch` che non trovano strumenti corrispondenti.
* In qualsiasi [configurazione senza ricerca degli strumenti](#configure-tool-search), Claude Code non segnala i fallimenti di connessione del server remoto a Claude.

<h3 id="push-messages-with-channels">
  Inviare messaggi con canali
</h3>

Un server MCP può anche inviare messaggi direttamente nella vostra sessione in modo che Claude possa reagire a eventi esterni come risultati CI, avvisi di monitoraggio, o messaggi di chat. Per abilitare questo, il vostro server dichiara la capacità `claude/channel` e voi lo attivate con il flag `--channels` all'avvio. Vedete [Canali](/docs/it/channels) per utilizzare un canale ufficialmente supportato, o [Riferimento canali](/docs/it/channels-reference) per costruire il vostro.

Sul [runtime v2](#mcp-client-runtimes), se impostate [`MCP_PROTOCOL_NEGOTIATION`](/docs/it/env-vars) su `auto` e un server di canale negozia la revisione del protocollo MCP 2026-07-28, non può consegnare messaggi di canale, quindi Claude Code non lo registra come canale. Lasciate la variabile non impostata, o impostatela su `legacy`, mantiene i server stdio sul handshake precedente.

<Tip>
  Suggerimenti:

  * Utilizzate il flag `-s` o `--scope` per specificare dove viene memorizzata la configurazione:
    * `local` (predefinito): disponibile solo per voi nel progetto corrente
    * `project`: condiviso con tutti nel progetto tramite il file `.mcp.json`
    * `user`: disponibile per voi in tutti i progetti
  * Impostate le variabili di ambiente con i flag `-e` o `--env` (ad esempio, `-e KEY=value`)
  * I flag `--transport` e `--header` accettano anche le forme brevi `-t` e `-H`
  * Configurate il timeout di avvio del server MCP utilizzando la variabile di ambiente `MCP_TIMEOUT` (ad esempio, `MCP_TIMEOUT=10000 claude` imposta un timeout di 10 secondi)
  * Impostate un timeout di esecuzione dello strumento per server aggiungendo un campo `timeout` in millisecondi alla voce `.mcp.json` di quel server, ad esempio `"timeout": 600000` per dieci minuti. Questo sostituisce la variabile di ambiente `MCP_TOOL_TIMEOUT` solo per quel server
  * Claude Code visualizza un avviso quando l'output dello strumento MCP supera 10.000 token e limita l'output a 25.000 token per impostazione predefinita. Per aumentare il limite, impostate la variabile di ambiente `MAX_MCP_OUTPUT_TOKENS` (ad esempio, `MAX_MCP_OUTPUT_TOKENS=50000`); la soglia di avviso è fissa. Vedete [Limiti di output MCP e avvisi](#mcp-output-limits-and-warnings)
  * Utilizzate `/mcp` per autenticarvi con server remoti che richiedono l'autenticazione OAuth 2.0
</Tip>

Il `timeout` per server è un limite di wall-clock duro per chiamata di strumento, e le notifiche di progresso dal server non lo estendono. I valori inferiori a 1000 vengono ignorati e ricadono in `MCP_TOOL_TIMEOUT`, o nel suo valore predefinito di circa 28 ore quando quella variabile non è impostata. Per un server HTTP, SSE, o [connettore claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) c'è anche un secondo timer per richiesta che copre ogni richiesta fino al primo byte di risposta del server. Claude Code imposta quel timer al massimo di tre valori: 60 secondi, il timeout dello strumento che si applica al server, e `MCP_TIMEOUT`. Il valore predefinito di 28 ore di un `MCP_TOOL_TIMEOUT` non impostato non entra in quel confronto, e un valore inferiore a 60 secondi non accorcia il timer. I server stdio e WebSocket non hanno un timer per richiesta.

Un `timeout` per server di almeno 1000 agisce anche come limite inferiore sul timeout di inattività descritto di seguito: Claude Code non interrompe mai le chiamate di strumento di quel server per inattività prima del `timeout` per server. Richiede Claude Code v2.1.203 o successivo.

Una chiamata di strumento a un server MCP che non invia risposta e nessuna notifica di progresso per la finestra di inattività interrompe con un errore invece di attendere il limite di wall-clock. Si applica a ogni tipo di server tranne i server IDE e i server in-process SDK. La finestra di inattività è predefinita a cinque minuti per server HTTP, SSE, WebSocket, e [connettore claude.ai](#use-mcp-servers-from-claude-ai), e a 30 minuti per server stdio. Prima della v2.1.203, i server stdio erano esenti dal timeout di inattività.

Impostate la variabile di ambiente [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/it/env-vars) in millisecondi per cambiare la finestra di inattività, o impostatela su `0` per disabilitare il controllo.

Questi timeout limitano quanto a lungo una chiamata può essere eseguita, non sempre quanto a lungo blocca la sessione: una chiamata di conversazione principale che viene eseguita oltre due minuti si sposta prima a un'attività in background. Vedete [Backgrounding automatico di lunghe chiamate di strumento](#automatic-backgrounding-of-long-tool-calls).

<h3 id="automatic-backgrounding-of-long-tool-calls">
  Backgrounding automatico di lunghe chiamate di strumento
</h3>

Una chiamata di strumento MCP nella conversazione principale che è ancora in esecuzione dopo due minuti si sposta a un'attività in background invece di bloccare la sessione. Claude riceve l'ID dell'attività immediatamente e continua a lavorare, e il risultato arriva come notifica di attività quando la chiamata si risolve. Il backgrounding automatico richiede Claude Code v2.1.212 o successivo.

L'attività appare in [`/tasks`](/docs/it/commands#all-commands), dove potete anche fermarla, e non sopravvive all'uscita dalla sessione. I limiti per chiamata si applicano ancora mentre la chiamata viene eseguita in background: il limite di wall-clock impostato dal `timeout` per server o [`MCP_TOOL_TIMEOUT`](/docs/it/env-vars), e il timeout di inattività impostato da [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/it/env-vars).

Impostate la variabile di ambiente [`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](/docs/it/env-vars) in millisecondi per cambiare la soglia, o impostatela su `0` per disattivare il backgrounding automatico. Impostare `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` su `1` lo disattiva anche, insieme a tutte le altre funzionalità di attività in background.

Alcune chiamate non si spostano mai in background:

* Chiamate da [subagenti](/docs/it/sub-agents); Claude Code mette in background solo le chiamate della conversazione principale
* Chiamate ai server IDE
* Chiamate in [modalità non interattiva](/docs/it/headless), a meno che `CLAUDE_AUTO_BACKGROUND_TASKS` non sia impostato su `1`, poiché un'esecuzione una tantum può terminare prima che il risultato arrivi

Una chiamata in attesa di una [finestra di dialogo di elicitazione](#respond-to-mcp-elicitation-requests) aperta non viene messa in background mentre la finestra di dialogo è aperta; il server è bloccato sul vostro input, non lento, quindi Claude Code rinvia lo spostamento fino a quando la finestra di dialogo non si chiude.

<h3 id="plugin-provided-mcp-servers">
  Server MCP forniti da plugin
</h3>

[I plugin](/docs/it/plugins/overview) possono raggruppare server MCP che forniscono strumenti e integrazioni quando abilitate il plugin. I server MCP del plugin funzionano in modo identico ai server configurati dall'utente.

**Come funzionano i server MCP del plugin**:

* I plugin definiscono i server MCP in `.mcp.json` alla radice del plugin o inline in `plugin.json`
* Quando abilitate un plugin, Claude Code avvia automaticamente i suoi server MCP
* Claude Code offre gli strumenti MCP del plugin insieme agli strumenti MCP configurati manualmente
* Aggiungete e rimuovete i server del plugin installando o disinstallando il plugin, non con i comandi `/mcp`. Potete comunque [attivare/disattivare un server del plugin installato](#disable-a-server-without-removing-it) in `/mcp`, che impedisce a Claude Code di connettersi ad esso senza rimuovere il plugin

**Esempio di configurazione MCP del plugin**:

In `.mcp.json` alla radice del plugin:

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

* **Ciclo di vita automatico**: i server si connettono e disconnettono in questi punti:
  * All'avvio della sessione, Claude Code connette automaticamente i server per i plugin abilitati. In `/mcp`, un server plugin remoto (HTTP o SSE) che avete usato prima può mostrare lo stato [`cached`](#server-status-detail) invece; Claude Code lo connette quando Claude chiama per la prima volta uno dei suoi strumenti
  * Se abilitate o disabilitate un plugin durante una sessione, Claude Code connette o disconnette i suoi server MCP quando il cambiamento si applica. [Applicare i cambiamenti del plugin senza riavviare](/docs/it/plugins/cli-reference#reload-plugins) descrive quando è. In una sessione senza un terminale interattivo, `/reload-plugins` non connette o disconnette i server MCP del plugin; quei cambiamenti hanno effetto nella vostra prossima sessione
  * Quando ricaricate, Claude Code mantiene le connessioni live dei server del plugin la cui configurazione è invariata, e fa lo stesso quando [sostituite l'elenco dei server MCP della sessione](/docs/it/agent-sdk/typescript#mcpsetserversresult) dall'Agent SDK senza nominarli
  * Quando [spostate la sessione con `/cd`](/docs/it/permissions#move-the-session-to-another-directory) su v2.1.246 o successivo, Claude Code connette i server dei plugin che le impostazioni della nuova directory abilitano e disconnette i server dei plugin che non sono più abilitati, quindi non dovete eseguire `/reload-plugins` dopo lo spostamento
  * Nelle [sessioni cloud](/docs/it/claude-code-on-the-web), una chiamata MCP a un server del plugin che non è ancora connesso, come subito dopo il risveglio di una sessione inattiva, avvia il server su richiesta e attende che si connetta
* **Segnaposti di percorso**: `${CLAUDE_PLUGIN_ROOT}` si risolve nella directory di installazione del plugin, `${CLAUDE_PLUGIN_DATA}` nella sua directory di [stato persistente](/docs/it/plugins/components#path-variables-and-persistent-data), e `${CLAUDE_PROJECT_DIR}` nella radice del progetto stabile. La sostituzione si applica a:
  * server `stdio`: `command`, `args`, `env`
  * server `http`, `sse`, e `ws`: `url`, `headers`, e `headersHelper`. Prima della v2.1.195, `headersHelper` passava il segnaposto come stringa letterale
* **Accesso all'ambiente utente**: accesso alle stesse variabili di ambiente dei server configurati manualmente
* **Tipi di trasporto multipli**: supporto per trasporti stdio, SSE, HTTP, e WebSocket, anche se il supporto del trasporto può variare per server

I server del plugin appaiono in `/mcp` con indicatori che mostrano che provengono dai plugin.

**Nomi degli strumenti MCP del plugin**:

Gli strumenti da un server MCP raggruppato nel plugin includono sia il nome del plugin che la chiave del server nel loro nome richiamabile. La forma completa è `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, dove qualsiasi carattere al di fuori di `A-Z`, `a-z`, `0-9`, `_`, e `-` viene sostituito con `_`. Per il server `database-tools` raggruppato in un plugin denominato `my-plugin`, uno strumento `query` è richiamabile come:

```
mcp__plugin_my-plugin_database-tools__query
```

Utilizzate questo nome completo quando fate riferimento allo strumento nelle [regole di autorizzazione](/docs/it/permissions), nell'elenco `allowed-tools` di una skill, nel [campo `tools` di un subagente](/docs/it/sub-agents#available-tools), o in un [matcher di hook](/docs/it/hooks#match-mcp-tools). Un matcher di hook scritto contro la chiave del server nudo, come `mcp__database-tools__.*`, non si attiva mai per un server raggruppato nel plugin.

Il server stesso si registra con il nome con ambito `plugin:<plugin-name>:<server-name>`, come `plugin:my-plugin:database-tools`. Utilizzate quel nome dove è previsto un nome di server configurato, come il [campo `server` di un hook `mcp_tool`](/docs/it/hooks#mcp-tool-hook-fields).

Vedete il [riferimento dei componenti del plugin](/docs/it/plugins/components#mcp-servers) per i dettagli sul raggruppamento dei server MCP con i plugin.

<h2 id="mcp-installation-scopes">
  Ambiti di installazione MCP
</h2>

I server MCP possono essere configurati a tre ambiti diversi. L'ambito che scegli controlla in quali progetti il server viene caricato e se la configurazione è condivisa con il tuo team. Gli amministratori possono anche distribuire o fornire server a ogni utente tramite [configurazione gestita](#managed-mcp-configuration).

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
  Il termine "ambito locale" per i server MCP differisce dalle impostazioni locali generali. I server MCP con ambito locale vengono archiviati in `~/.claude.json` (la tua directory home), mentre le impostazioni locali generali utilizzano `.claude/settings.local.json` (nella directory del progetto). Vedi [Impostazioni](/docs/it/settings#where-settings-live) per i dettagli sui percorsi dei file di impostazioni.
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

I server con ambito del progetto abilitano la collaborazione del team archiviando le configurazioni in un file `.mcp.json` nella directory radice del tuo progetto. Quando aggiungi un server con ambito del progetto, Claude Code crea o aggiorna automaticamente questo file con la struttura di configurazione appropriata. Archivia `.mcp.json` nel controllo della versione in modo che tutti nel tuo team ottengano gli stessi strumenti e servizi MCP.

```bash theme={null}
# Aggiungi un server con ambito del progetto
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

Il file `.mcp.json` risultante segue un formato standardizzato:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

Per motivi di sicurezza, Claude Code richiede l'approvazione in sessioni interattive prima di utilizzare server con ambito del progetto dai file `.mcp.json`. Per ripristinare queste scelte di approvazione, esegui `claude mcp reset-project-choices`.

Nelle esecuzioni `claude -p`, nelle sessioni [Agent SDK](/docs/it/headless) e nelle [sessioni cloud](/docs/it/claude-code-on-the-web), Claude Code non può mostrare quel prompt: carica i server con ambito del progetto senza chiedere. Claude Code salta anche il prompt in una sessione che avvii in modalità `bypassPermissions` con [`skipDangerousModePermissionPrompt`](/docs/it/settings-reference#skipdangerousmodepermissionprompt) impostato nelle tue impostazioni utente o nelle impostazioni gestite. Per mantenere un server fuori comunque:

* Aggiungilo a [`disabledMcpjsonServers`](/docs/it/settings-reference#disabledmcpjsonservers), che lo blocca in ogni modalità di autorizzazione.
* Escludi completamente le impostazioni del progetto con [`--setting-sources`](/docs/it/cli-reference#cli-flags) o l'opzione `settingSources` dell'SDK.
* Avvia la sessione con [`--strict-mcp-config`](/docs/it/cli-reference#cli-flags). Claude Code utilizza quindi solo i server MCP che passi con `--mcp-config`. Saltare il prompt di approvazione per i server con ambito del progetto che Claude Code non sta caricando richiede Claude Code v2.1.246 o successivo; prima di v2.1.246, una sessione ristretta attendeva comunque l'approvazione per loro, il che lasciava le sessioni in background in attesa all'avvio. Vedi [Controllo esclusivo con managed-mcp.json](/docs/it/managed-mcp#exclusive-control-with-managed-mcp-json) per quello che il flag fa sotto un file MCP gestito.

[Approvazioni del server del progetto e fiducia dell'area di lavoro](#project-server-approvals-and-workspace-trust) copre come le approvazioni impegnate nel repository interagiscono con la fiducia dell'area di lavoro.

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
4. [Server forniti da plugin](/docs/it/plugins/components#mcp-servers)
5. [Connettori claude.ai](#use-mcp-servers-from-claude-ai)

I tre ambiti corrispondono ai duplicati per nome. I plugin e i connettori corrispondono per endpoint, quindi uno che punta allo stesso URL o comando di un server sopra è trattato come un duplicato.

Un server che la tua organizzazione fornisce tramite l'impostazione gestita [`managedMcpServers`](/docs/it/managed-mcp#provide-servers-through-managed-settings) si classifica al di sopra di tutti questi, quindi quando uno di loro lo duplica, Claude Code si connette alla definizione dell'organizzazione. Richiede Claude Code v2.1.259 o successivo.

Se apri una sessione locale nella [scheda Code dell'app Desktop](/docs/it/desktop#mcp-servers-from-the-claude-desktop-chat-app) con lo stesso nome di server stdio al livello superiore di `~/.claude.json` (ambito utente) e in `.mcp.json`, la scheda Code utilizza la definizione `~/.claude.json`.

<h3 id="environment-variable-expansion-in-mcp-json">
  Espansione delle variabili di ambiente in `.mcp.json`
</h3>

Claude Code supporta l'espansione delle variabili di ambiente nei file `.mcp.json`, consentendo ai team di condividere configurazioni mantenendo flessibilità per i percorsi specifici della macchina e i valori sensibili come le chiavi API.

<h4 id="supported-syntax">
  Sintassi supportata
</h4>

* `${VAR}`: si espande al valore della variabile di ambiente `VAR`
* `${VAR:-default}`: si espande a `VAR` se impostato, altrimenti utilizza `default`

<h4 id="expansion-locations">
  Posizioni di espansione
</h4>

Le variabili di ambiente possono essere espanse in:

* `command`: il percorso dell'eseguibile del server
* `args`: argomenti della riga di comando
* `env`: variabili di ambiente passate al server
* `url`: per i tipi di server HTTP
* `headers`: per l'autenticazione del server HTTP

<h4 id="example-with-variable-expansion">
  Esempio con espansione di variabili
</h4>

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

<h4 id="unset-variables-without-a-default">
  Variabili non impostate senza un valore predefinito
</h4>

Se una variabile di ambiente richiesta non è impostata e non ha un valore predefinito, la configurazione viene comunque caricata: Claude Code segnala un avviso di variabile mancante per quel server nell'output di `claude mcp list` e utilizza il testo `${VAR}` non espanso così com'è. Imposta la variabile o aggiungi un fallback `:-default` in modo che il server si avvii con il valore che intendi. In un URL remoto del server e negli header, alcune variabili di credenziali [leggono come vuote](#credential-variables-that-read-as-empty) invece, senza avviso.

<h4 id="credential-variables-that-read-as-empty">
  Variabili di credenziali che leggono come vuote
</h4>

In un URL remoto del server e negli header, Claude Code legge le variabili di credenziali dal tuo ambiente come vuote piuttosto che espanderle. Questo impedisce che il `.mcp.json` di un progetto o un plugin invii le tue credenziali di Claude Code o del provider cloud a un server che nomina. Se scrivi `Bearer ${ANTHROPIC_AUTH_TOKEN}`, il server riceve `Bearer ` senza credenziale e rifiuta la richiesta, di solito con un `401`. Claude Code lo segnala come una connessione non riuscita.

I nomi coperti sono:

* Le credenziali di Claude Code stesso, come `ANTHROPIC_API_KEY` e `ANTHROPIC_AUTH_TOKEN`
* Le credenziali del tuo provider cloud, come `AWS_BEARER_TOKEN_BEDROCK`
* Altre credenziali che il tuo ambiente contiene, come `HTTPS_PROXY` e `NPM_TOKEN`

Un nome coperto legge come vuoto indipendentemente dal fatto che tu abbia impostato la variabile, e un fallback `:-default` su di esso viene ignorato. Un URL di base del provider come `ANTHROPIC_BASE_URL` si espande comunque, quindi `"url": "${ANTHROPIC_BASE_URL}/mcp"` funziona, a meno che il valore dell'URL stesso non incorpori una credenziale come un nome utente e una password.

Un nome al di fuori di questo set, come `API_KEY`, si espande come scritto. Per dare al server una delle credenziali coperte, copiala in una variabile con un nome di tua scelta e fai riferimento a quel nome invece.

Quando l'URL o gli header di un server remoto fanno riferimento a una variabile coperta che hai impostato, Claude Code la nomina in una riga di log di debug. Per leggere la riga, esegui `claude --debug-file /tmp/claude-debug.log` e cerca in quel file `never expanded toward a remote server`.

<h4 id="how-references-appear-in-/mcp-and-cli-output">
  Come i riferimenti appaiono in `/mcp` e nell'output CLI
</h4>

Per un server negli ambiti [locale, progetto o utente](#mcp-installation-scopes), le seguenti superfici mostrano un riferimento `${VAR}` per nome piuttosto che come valore risolto:

* L'URL o la riga di comando nella vista dettagli `/mcp` di un server
* Output di `claude mcp list` e `claude mcp get`

La vista dettagli `/mcp` mostra i riferimenti in questo modo in Claude Code v2.1.268 o successivo.

Per un server che la tua organizzazione fornisce tramite l'impostazione `managedMcpServers`, queste superfici mostrano [solo l'host dell'URL](/docs/it/managed-mcp#what-users-can-see-and-change).

Per verificare cosa mostrano `claude mcp list`, `claude mcp get` e `/mcp` quando una connessione non riesce, vedi [Dettagli dello stato del server](#server-status-detail).

<h2 id="practical-examples">
  Esempi pratici
</h2>

<h3 id="example-connect-to-github-for-code-reviews">
  Esempio: Connettiti a GitHub per le revisioni del codice
</h3>

Il server MCP remoto di GitHub si autentica con un token di accesso personale GitHub passato come header. Per ottenerne uno, apri le [impostazioni del token GitHub](https://github.com/settings/personal-access-tokens), genera un nuovo token con granularità fine con accesso ai repository con cui desideri che Claude lavori, quindi aggiungi il server:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Sostituisci `YOUR_GITHUB_PAT` con il tuo token di accesso personale. Il comando `claude mcp add` salva la configurazione senza convalidare le credenziali, quindi un valore segnaposto è accettato qui ma il server non riesce a connettersi in seguito. Per verificare la connessione, esegui `/mcp` e controlla che il server mostri `connected`. Un server con credenziali errate mostra `failed`, e il dettaglio dell'errore include lo stato HTTP che il server ha restituito, come un 401.

Quindi lavora con GitHub:

```text wrap theme={null}
Rivedi la PR #456 e suggerisci miglioramenti
```

```text wrap theme={null}
Crea un nuovo issue per il bug che abbiamo appena trovato
```

```text wrap theme={null}
Mostrami tutte le PR aperte assegnate a me
```

<h3 id="example-query-your-postgresql-database">
  Esempio: Interroga il tuo database PostgreSQL
</h3>

[DBHub](https://github.com/bytebase/dbhub), il pacchetto `@bytebase/dbhub`, è un server MCP che connette Claude a un database relazionale attraverso la stringa di connessione che passi in `--dsn`. Utilizza un utente di database di sola lettura nella stringa di connessione in modo che le query che Claude esegue non possano modificare i dati:

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Per confermare che il server si avvia, esegui `/mcp` e controlla che `db` mostri `connected`.

Quindi interroga il tuo database naturalmente:

```text wrap theme={null}
Qual è il nostro ricavo totale questo mese?
```

```text wrap theme={null}
Mostrami lo schema per la tabella orders
```

```text wrap theme={null}
Trova i clienti che non hanno effettuato un acquisto negli ultimi 90 giorni
```

<h2 id="authenticate-with-remote-mcp-servers">
  Autenticazione con server MCP remoti
</h2>

Molti server MCP basati su cloud richiedono l'autenticazione. Claude Code supporta OAuth 2.0 per connessioni sicure.

Claude Code contrassegna un server remoto come richiedente autenticazione quando il server risponde con `401 Unauthorized` o `403 Forbidden`. Ciò che Claude Code mostra dipende dal server:

* Per un server a cui non hai effettuato l'accesso, uno di questi codici di stato lo contrassegna in `/mcp` in modo che tu possa completare il flusso OAuth.
* Per un [connettore claude.ai](#use-mcp-servers-from-claude-ai), un `401` causato dal rifiuto di claude.ai del tuo token di sessione non contrassegna il connettore, perché la riautorizzazione del connettore non può risolvere il tuo accesso. Claude Code mostra invece lo [stato di rifiuto del token di sessione](/docs/it/errors#claude-ai-rejected-the-session-token).
* Per un server il cui header `Authorization` hai configurato, in `headers` o tramite un [`headersHelper`](#use-dynamic-headers-for-custom-authentication), un `401` o `403` durante la connessione non contrassegna il server, perché la credenziale da correggere è quella che hai configurato. Claude Code segnala invece la connessione come non riuscita. Se hai impostato quell'header da un riferimento `${VAR}`, verifica se quella variabile è una che Claude Code [legge come vuota](#credential-variables-that-read-as-empty).
* Per un connettore [consegnato a una sessione cloud](#how-connectors-reach-claude-code), Claude Code non esegue un flusso di accesso, perché il proxy della sessione si autentica al connettore con l'autorizzazione che hai concesso in claude.ai. Quando un connettore lì ha bisogno di autorizzazione di nuovo, riconnettilo su [claude.ai/customize/connectors](https://claude.ai/customize/connectors) piuttosto che dalla sessione.

Quando una richiesta a un server OAuth a cui hai già effettuato l'accesso restituisce `401 Unauthorized`, Claude Code aggiorna il token archiviato, si riconnette e ritenta la richiesta una volta. Contrassegna il server in `/mcp` solo se anche quel tentativo non riesce. Prima della v2.1.206, un aggiornamento del token che non riusciva per un motivo transitorio, come un errore di rete, contrassegnava un server OAuth come richiedente autenticazione per il resto della sessione anche se il suo token di aggiornamento era ancora valido.

Quando il server rifiuta il token di aggiornamento archiviato, Claude Code mostra immediatamente un avviso che punta a `/mcp`. Apri `/mcp` e seleziona **Re-authenticate** sul server per accedere di nuovo prima che la prossima chiamata dello strumento non riesca.

Un server personalizzato che restituisce un header `WWW-Authenticate` che punta al suo server di autorizzazione ottiene la stessa scoperta automatica di qualsiasi altro server remoto.

Claude Code mostra anche un avviso di avvio quando uno o più server configurati richiedono l'autenticazione, quindi non è necessario aprire `/mcp` per scoprire quali server richiedono l'accesso. L'avviso richiede Claude Code v2.1.193 o successiva. Conta solo i server a cui puoi accedere da Claude Code. Prima della v2.1.218, contava anche i [connettori claude.ai](#use-mcp-servers-from-claude-ai) che non erano connessi in claude.ai, che puoi connettere solo dalle impostazioni di claude.ai.

L'avviso annuncia ogni server una volta e lo esclude dal conteggio ai successivi avvii fino a quando quel server non si è connesso e ha bisogno di accesso di nuovo. `/mcp` elenca comunque ogni server che ha bisogno di accesso.

In modalità non interattiva non c'è un pannello `/mcp`, quindi Claude Code non può eseguire il flusso OAuth per te. A partire da v2.1.196, quando un server configurato richiede l'autenticazione durante un'esecuzione `claude -p` o Agent SDK con [ricerca degli strumenti](#scale-with-mcp-tool-search) abilitata, che è l'impostazione predefinita, Claude Code comunica a Claude che gli strumenti del server non sono disponibili fino a quando non lo autorizzi. Claude può quindi nominare il server che richiede l'accesso invece di rispondere come se il server non fosse configurato. Completa l'accesso da una sessione interattiva con `/mcp` o `claude mcp login <name>`.

Se hai configurato `headers.Authorization` per il server e il server rifiuta quell'header, Claude Code segnala la connessione come non riuscita invece di ricorrere a OAuth. Verifica che il token sia valido per l'endpoint MCP, oppure rimuovi l'header per utilizzare il flusso OAuth.

<Steps>
  <Step title="Aggiungi il server che richiede l'autenticazione">
    Se hai già aggiunto il server `sentry` nella [guida rapida MCP](/docs/it/mcp-quickstart#connect-a-server-that-requires-sign-in), salta questo passaggio: eseguire di nuovo `claude mcp add` con lo stesso nome di server nello stesso ambito non riesce con `MCP server sentry already exists in local config`. Altrimenti, esegui:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Utilizza il comando /mcp all'interno di Claude Code">
    In Claude Code, utilizza il comando:

    ```text wrap theme={null}
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

Il comando `claude mcp login <name>` esegue il flusso OAuth di un server configurato direttamente dalla tua shell, quindi non è necessario aprire il pannello `/mcp` all'interno di una sessione.

```bash theme={null}
claude mcp login sentry
```

Per cancellare le credenziali archiviate in seguito, esegui `claude mcp logout <name>`.

`claude mcp login` rileva quando nessun browser locale è disponibile, ad esempio durante una sessione SSH o su Linux senza un server di visualizzazione, e stampa l'URL di autorizzazione invece di provare ad aprire un browser. Apri l'URL sulla tua macchina locale, quindi incolla l'URL di reindirizzamento completo dalla barra degli indirizzi del tuo browser al prompt. Il comando ha bisogno di un terminale interattivo per il passaggio di incollamento, quindi connettiti con `ssh -t`. Passa `--no-browser` per forzare il prompt dell'URL anche quando viene rilevato un browser locale.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Utilizza una porta di callback OAuth fissa
</h3>

Alcuni server MCP richiedono un URI di reindirizzamento specifico registrato in anticipo. Per impostazione predefinita, Claude Code sceglie una porta disponibile casuale per il callback OAuth. Utilizza `--callback-port` per fissare la porta in modo che corrisponda a un URI di reindirizzamento pre-registrato della forma `http://localhost:PORT/callback`. Se l'accesso su Claude Code v2.1.229 non riesce con una mancata corrispondenza dell'URI di reindirizzamento, vedi la nota sulla versione in [Utilizza credenziali OAuth pre-configurate](#use-pre-configured-oauth-credentials).

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

    In v2.1.229, Claude Code ha inviato `http://127.0.0.1:PORT/callback` invece, e i server che corrispondevano esattamente all'URI di reindirizzamento registrato hanno rifiutato l'accesso con una mancata corrispondenza dell'URI di reindirizzamento. Claude Code v2.1.231 ha ripristinato il modulo `localhost`. Per recuperare su v2.1.229, aggiorna Claude Code, oppure aggiungi temporaneamente il modulo `http://127.0.0.1:PORT/callback` agli URI di reindirizzamento registrati del server.
  </Step>

  <Step title="Aggiungi il server con le tue credenziali">
    Scegli uno dei seguenti metodi. La porta utilizzata per `--callback-port` può essere qualsiasi porta disponibile. Deve corrispondere all'URI di reindirizzamento che hai registrato nel passaggio precedente.

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
  * Puoi impostare il segreto client solo quando aggiungi il server. Quando ti autentichi con `claude mcp login` o da `/mcp`, Claude Code utilizza il segreto archiviato e non richiede uno o legge `MCP_CLIENT_SECRET`
  * Per aggiungere o modificare il segreto in seguito, rimuovi il server con `claude mcp remove <name>`, quindi aggiungilo di nuovo con `--client-secret` e lo stesso `--scope`
  * Se il server utilizza un client OAuth pubblico senza segreto, utilizza solo `--client-id` senza `--client-secret`
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

A partire da v2.1.196, quando `oauth.scopes` non è impostato, Claude Code richiede l'ambito fornito dall'header `WWW-Authenticate` del server o dai suoi metadati della risorsa protetta, e non invia alcun parametro `scope` quando nessuno dei due lo fornisce. Non richiede più il catalogo completo di `scopes_supported` dai metadati del server di autorizzazione scoperto automaticamente. La richiesta di quel catalogo ha fatto sì che i provider di identità che pubblicizzano ambiti solo amministratore o modello rifiutassero la richiesta di autorizzazione con un errore `invalid_scope`. I metadati recuperati da un `authServerMetadataUrl` configurato forniscono comunque il loro `scopes_supported` come ambiti richiesti.

Se il server di autorizzazione pubblicizza `offline_access` in `scopes_supported`, Claude Code lo aggiunge agli ambiti fissati in modo che il token di accesso possa essere aggiornato senza un nuovo accesso al browser.

Se il server successivamente restituisce un 403 `insufficient_scope` per una chiamata di strumento, la chiamata non riesce con un messaggio [`needs additional permissions`](/docs/it/errors#mcp-server-needs-you-to-sign-in-again) che nomina l'ambito che il server richiede. Il server viene mostrato come richiedente autenticazione in `/mcp`.

Se quell'ambito non è nei tuoi `oauth.scopes` fissati, aggiungilo, quindi esegui `/mcp` e autentica di nuovo il server. Claude Code richiede gli ambiti fissati piuttosto che l'ambito che il server ha nominato, quindi se ti autentichi di nuovo senza aggiungerlo, il token che ottieni ancora non lo ha.

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
* Claude Code esegue il comando in una shell e rinuncia dopo 10 secondi
* Claude Code sceglie la directory di lavoro del comando in base a [dove hai configurato il server](#where-the-helper-runs), quindi fornisci lo script come percorso assoluto o mettilo su `PATH`
* Le intestazioni dinamiche sovrascrivono qualsiasi `headers` statico con lo stesso nome

Claude Code esegue l'helper di nuovo ad ogni connessione, all'avvio della sessione e alla riconnessione, una volta che la [regola di fiducia per i server di ambito progetto e locale](#trust-a-folder-before-its-headershelper-runs) lo consente. Non memorizza il risultato nella cache, quindi il tuo script è responsabile di qualsiasi riutilizzo di token.

Se una chiamata di strumento restituisce `401 Unauthorized` o `403 Forbidden`, Claude Code esegue automaticamente di nuovo l'helper secondo la stessa regola, si riconnette con le intestazioni aggiornate e ritenta la chiamata una volta. Claude Code contrassegna il server come richiedente autenticazione in `/mcp` solo se anche quel tentativo non riesce.

Quando l'output dell'helper include un header `Authorization`, Claude Code utilizza quella credenziale come autenticazione del server e non ricorre a OAuth per il server.

Se il server rifiuta la credenziale dell'helper durante la connessione, Claude Code segnala la connessione come non riuscita piuttosto che contrassegnare il server come richiedente autenticazione. Correggi la credenziale che il tuo helper restituisce, quindi riconnettiti da `/mcp` per eseguire di nuovo l'helper.

Claude Code imposta queste variabili di ambiente quando esegue l'helper:

| Variabile                     | Valore                                                                                                                   |
| :---------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | il nome del server MCP                                                                                                   |
| `CLAUDE_CODE_MCP_SERVER_URL`  | l'URL del server MCP                                                                                                     |
| `CLAUDE_PLUGIN_ROOT`          | la directory radice del plugin. Impostato solo quando un [plugin](/docs/it/plugins/components#mcp-servers) fornisce il server |

Utilizza questi per scrivere un singolo script helper che serve più server MCP.

Un `headersHelper` fornito da un plugin non può fare riferimento ai valori [`${user_config.*}`](/docs/it/plugins/manifest-reference#user-configuration) del plugin, perché il comando viene eseguito attraverso una shell. Claude Code segnala il server come non configurato correttamente con un [errore](/docs/it/errors#plugin-command-references-user-config) e non sostituisce il valore. Metti `${user_config.KEY}` nel campo `headers` del server, che non viene analizzato dalla shell, oppure fai in modo che lo script helper legga il valore da un file di configurazione. Prima della v2.1.207, `headersHelper` sostituiva i valori `${user_config.*}`.

<h4 id="where-the-helper-runs">
  Dove l'helper viene eseguito
</h4>

Claude Code sceglie la directory di lavoro del comando `headersHelper` dalla configurazione che dichiara il server. Un `cd` che Claude esegue in Bash non lo sposta, e [`/cd`](/docs/it/permissions#move-the-session-to-another-directory) lo sposta solo per i server che vengono eseguiti dalla directory di lavoro primaria della sessione. Ogni riga di seguito fornisce la directory rispetto alla quale un percorso relativo nel tuo comando `headersHelper` si risolve.

| Dove hai configurato il server                                                                                                                                                                                | Directory di lavoro                                                                                                   |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------- |
| Un [plugin](/docs/it/plugins/components#mcp-servers)                                                                                                                                                               | La directory radice del plugin. Richiede Claude Code v2.1.195 o successiva                                            |
| Un `.mcp.json` di progetto o un server di [ambito locale](#local-scope)                                                                                                                                       | La directory del progetto in cui il server è dichiarato                                                               |
| Un file agent nel tuo progetto, un server dall'opzione `mcpServers` dell'SDK o dal metodo `setMcpServers()`, o [`--mcp-config`](/docs/it/cli-reference)                                                            | La [directory di lavoro primaria](/docs/it/permissions#working-directories) della sessione                                 |
| [Ambito utente](#user-scope), [MCP gestito](/docs/it/managed-mcp), un [connettore claude.ai](#use-mcp-servers-from-claude-ai), o un file agent da fuori dal tuo progetto, incluso uno da una directory `--add-dir` | La tua directory di configurazione, `~/.claude` a meno che tu non abbia impostato [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars) |

Prima della v2.1.238, Claude Code eseguiva anche gli helper dei server di ambito utente, gestiti e connettore claude.ai, e dei file agent da fuori dal tuo progetto, dalla directory da cui lo hai avviato.

<h4 id="which-variables-a-helper-can-read">
  Quali variabili un helper può leggere
</h4>

Un `headersHelper` che un repository o un plugin fornisce è un comando che non hai scritto, quindi Claude Code lo esegue senza le variabili di credenziale dal tuo ambiente, come `ANTHROPIC_API_KEY`. Dove hai configurato il server decide se questo si applica:

* **Rimosso**: un server in un `.mcp.json` di progetto o in un plugin, e un server inline in un file agent dal tuo progetto o da una directory `--add-dir`
* **Non rimosso**: un server di [ambito utente](#user-scope) o [ambito locale](#local-scope), in [MCP gestito](/docs/it/managed-mcp), da un [connettore claude.ai](#use-mcp-servers-from-claude-ai), o fornito dall'SDK o da [`--mcp-config`](/docs/it/cli-reference), e un server inline in un file agent da `~/.claude/agents/`, dalle impostazioni gestite, o passato con `--agents`

A parte le variabili `GIT_CONFIG_KEY_<n>` di Git, Claude Code rimuove ogni variabile dal tuo ambiente il cui nome sembra una credenziale, come un nome con `TOKEN`, `SECRET`, `PASSWORD`, `KEY`, o `AUTH` in esso in entrambi i casi di lettera, quindi sia `ANTHROPIC_API_KEY` che `MY_REGISTRY_TOKEN` vengono rimossi. Claude Code rimuove anche un elenco fisso di variabili di credenziale i cui nomi non seguono quel modello, come `ANTHROPIC_CUSTOM_HEADERS`.

Quando questo si applica al tuo helper, fai in modo che lo script legga la sua credenziale da un file o da un archivio di credenziali. Se l'`url` del server [espande una di queste variabili](#environment-variable-expansion-in-mcp-json), il valore `CLAUDE_CODE_MCP_SERVER_URL` che l'helper riceve ha quella parte sostituita con `REDACTED` anche.

<h4 id="trust-a-folder-before-its-headershelper-runs">
  Affida una cartella prima che il suo headersHelper venga eseguito
</h4>

Claude Code esegue un `headersHelper` come comando shell arbitrario. Per un server in un `.mcp.json` di progetto o di [ambito locale](#local-scope), lo esegue solo dopo che accetti la [finestra di dialogo di fiducia](/docs/it/permissions#project-allow-rules-and-workspace-trust) per la directory del progetto in cui il server è dichiarato. Prima della v2.1.238, una sessione `claude -p` o SDK eseguiva questi helper senza controllare la fiducia, e una sessione interattiva li eseguiva una volta che avevi affidato una cartella genitore.

* **Fiducia che non conta**: la fiducia di una cartella genitore, e la fiducia automatica che una sessione `claude -p` o SDK ottiene per gli [hook nei file di impostazioni](/docs/it/permissions#what-runs-before-you-trust-a-folder)
* **Fino a quando non affidi la cartella**: Claude Code connette il server con i suoi soli `headers` statici. In una sessione `claude -p` o SDK stampa anche una riga [`headersHelper not run`](/docs/it/errors#headershelper-not-run) per server su stderr, dicendoti come concedere la fiducia.
* **Fiducia senza una finestra di dialogo**: imposta `projects["<path>"].hasTrustDialogAccepted` su `true` in `~/.claude.json`. `<path>` è la cartella su cui [Project allow rules and workspace trust](/docs/it/permissions#project-allow-rules-and-workspace-trust) dice che Claude Code basa la fiducia.

Claude Code applica la stessa regola a un server dichiarato inline in un [file agent](/docs/it/sub-agents#scope-mcp-servers-to-a-subagent), controllando da dove proveniva quel file agent: il tuo progetto, per un file nella sua directory `.claude/agents/`, o una directory `--add-dir`. Fino a quando non [affidi quel progetto o quella directory stessa](/docs/it/permissions#what-runs-before-you-trust-a-folder), Claude Code non carica il server affatto, quindi il suo helper non viene mai eseguito nemmeno.

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
  Utilizzare i server MCP da claude.ai
</h2>

Se hai effettuato l'accesso a Claude Code con un account [claude.ai](https://claude.ai), i server MCP che hai aggiunto in claude.ai, noti come [connettori](https://claude.com/docs/connectors), sono automaticamente disponibili in Claude Code:

<Steps>
  <Step title="Configurare i server MCP in claude.ai">
    Aggiungi i server su [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Nei piani Team ed Enterprise, solo gli amministratori possono aggiungere server.
  </Step>

  <Step title="Autenticare il server MCP">
    Completa eventuali passaggi di autenticazione richiesti in claude.ai.
  </Step>

  <Step title="Visualizzare e gestire i server in Claude Code">
    In Claude Code, utilizza il comando:

    ```text wrap theme={null}
    /mcp
    ```

    I server da claude.ai vengono visualizzati nell'elenco con indicatori che mostrano che provengono da claude.ai.
  </Step>
</Steps>

Claude Code contrassegna un connettore come `managed` in `/mcp` e nel gestore [`/plugin`](/docs/it/plugins/install) quando la tua organizzazione gestisce la sua autenticazione in claude.ai. Lo stato managed non cambia il modo in cui Claude Code si connette al connettore o applica i [controlli degli strumenti](#organization-controls-on-connector-tools) della tua organizzazione.

I connettori a cui non hai mai effettuato l'accesso sono compressi dietro una riga `Show unused connectors` alla fine della sezione claude.ai, in modo che un elenco fornito dall'organizzazione non riempia il pannello. Seleziona la riga per espanderli. Un connettore a cui hai effettuato l'accesso in precedenza rimane visibile anche quando attualmente necessita di una nuova autenticazione.

I connettori da claude.ai vengono recuperati solo quando il tuo [metodo di autenticazione](/docs/it/authentication#authentication-precedence) attivo è un accesso con abbonamento a claude.ai. Non vengono caricati, anche se hai precedentemente eseguito `/login`, quando:

* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper` è attivo
* Un provider di terze parti come Amazon Bedrock o Agent Platform di Google Cloud è attivo
* `ANTHROPIC_PROFILE`, le variabili di federazione, o un [profilo Anthropic](/docs/it/authentication#anthropic-profiles-and-federation-credentials) attivo fornisce le credenziali
* `CLAUDE_CODE_OAUTH_TOKEN` contiene un token da [`claude setup-token`](/docs/it/authentication#generate-a-long-lived-token), che può solo effettuare richieste di modello

Se `/mcp` non elenca un connettore che hai aggiunto, esegui `/status` per confermare quale metodo di autenticazione è attivo. Annulla l'impostazione di quella variabile di ambiente, rimuovi l'impostazione `apiKeyHelper`, o [disattiva il profilo](/docs/it/authentication#anthropic-profiles-and-federation-credentials), quindi esegui `/login` per selezionare il tuo account claude.ai.

Se un problema di rete temporaneo impedisce il caricamento dell'elenco dei connettori all'avvio della sessione, Claude Code ritenta il recupero fino a tre volte in background, e i connettori vengono visualizzati una volta che un tentativo ha successo. Se ancora non sono apparsi, riavvia Claude Code per recuperare di nuovo l'elenco.

Se `/mcp` mostra un connettore come `connected · session token rejected`, o la sua vista dettagliata mostra [`claude.ai rejected the session token`](/docs/it/errors#claude-ai-rejected-the-session-token), claude.ai ha rifiutato il token dalla tua accesso a Claude Code, di solito perché l'accesso è scaduto e non poteva essere aggiornato. Autorizzare di nuovo il connettore non cancella questo stato, perché l'autorizzazione propria del connettore in claude.ai non è quello che è stato rifiutato. Per cancellarlo:

1. Esegui `/login` per accedere di nuovo.
2. Ricollega il connettore da `/mcp`.

Prima della v2.1.222, Claude Code contrassegnava i connettori come necessitanti di autenticazione, e autorizzarli non lo risolveva.

Un server che hai aggiunto in Claude Code ha [precedenza](#scope-hierarchy-and-precedence) su un connettore claude.ai che punta allo stesso URL. Quando ciò accade, `/mcp` elenca il connettore come nascosto e mostra come rimuovere il duplicato se preferisci utilizzare il connettore.

Alcuni connettori ospitati da Anthropic, come Microsoft 365, Gmail e Google Calendar, non supportano OAuth locale da Claude Code perché il provider di identità upstream accetta solo l'URL di reindirizzamento che claude.ai ha registrato. Quando un server che hai aggiunto con `claude mcp add` o in `.mcp.json` punta a uno di questi host e accedi da `/mcp` o con `claude mcp login`, Claude Code mostra [`is Anthropic-hosted and doesn't support local OAuth`](/docs/it/errors#anthropic-hosted-and-doesnt-support-local-oauth), indirizzandoti a connettere il servizio su [claude.ai/customize/connectors](https://claude.ai/customize/connectors).

Dopo aver rimosso la tua voce con `claude mcp remove <name>` e aver connesso il servizio su claude.ai, il connettore viene visualizzato in Claude Code automaticamente.

<h3 id="how-connectors-reach-claude-code">
  Come i connettori raggiungono Claude Code
</h3>

Quali impostazioni governano un connettore claude.ai dipende da dove viene eseguita la tua sessione, perché solo alcune sessioni recuperano i connettori da claude.ai stesse. Ogni riga sottostante nomina come i connettori arrivano in un tipo di sessione e cosa li controlla lì. Le [sessioni WSL](/docs/it/desktop-wsl#what-works-in-a-wsl-session) dell'app desktop non hanno una riga perché i connettori non sono ancora disponibili in esse.

| Dove viene eseguita la sessione                                                                                          | Come arrivano i connettori           | Cosa li governa                                                                                                                                                                                                                                  |
| :----------------------------------------------------------------------------------------------------------------------- | :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sessioni Terminal, [VS Code](/docs/it/vs-code), [JetBrains](/docs/it/jetbrains), e [Agent SDK](/docs/it/agent-sdk/claude-code-features) | Claude Code li recupera da claude.ai | Le impostazioni in questa sezione e [configurazione MCP gestita](/docs/it/managed-mcp)                                                                                                                                                                |
| [Sessioni cloud](/docs/it/claude-code-on-the-web)                                                                             | L'host remoto li passa               | Le impostazioni dell'organizzazione claude.ai, più le impostazioni [allowlist e denylist](/docs/it/managed-mcp#policy-based-control-with-allowlists-and-denylists) che raggiungono la sessione e qualsiasi `managed-mcp.json` sull'host che la esegue |
| Le sessioni locali e SSH dell'[app desktop](/docs/it/desktop)                                                                 | L'app desktop li fornisce in-process | Voci `blocked` nei [controlli degli strumenti del connettore](#organization-controls-on-connector-tools) della tua organizzazione                                                                                                                |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors), `ENABLE_CLAUDEAI_MCP_SERVERS`, e [`allowAllClaudeAiMcps`](/docs/it/settings-reference#allowallclaudeaimcps) agiscono solo sulla prima riga, i connettori che Claude Code recupera da solo. Le altre due righe differiscono da essa in questi modi:

* **Sessioni cloud**: le voci `allowedMcpServers` e `deniedMcpServers` che raggiungono la sessione, ad esempio attraverso [impostazioni gestite dal server](/docs/it/server-managed-settings), filtrano anche i connettori forniti. Il proxy della sessione riscrive l'URL di ogni connettore, quindi un pattern `serverUrl` scritto per l'URL del connettore stesso non lo corrisponde. Per ammettere i connettori forniti insieme a un allowlist di URL in un ambiente auto-ospitato, aggiungi le voci `serverUrl` elencate sotto [Connector traffic leaves your network](/docs/it/self-hosted-environments-deploy#connector-traffic-leaves-your-network). Claude Code elimina i connettori forniti quando un `managed-mcp.json` è presente sull'host che esegue la sessione, come un [host di runner auto-ospitato](/docs/it/self-hosted-environments-configuration#mcp-servers), indipendentemente dal fatto che tu imposti `allowAllClaudeAiMcps`.
* **Sessioni locali e SSH dell'app desktop**: l'app desktop registra i connettori come server `type: "sdk"` in-process, e nessuna impostazione MCP o `managed-mcp.json` li raggiunge. Un utente mantiene un connettore fuori dalle proprie sessioni disconnettendolo su [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Un'organizzazione blocca i [strumenti](#organization-controls-on-connector-tools) di un connettore o disattiva completamente [Claude Code nell'app desktop](/docs/it/desktop#admin-console-controls).

<h3 id="organization-controls-on-connector-tools">
  Controlli dell'organizzazione sugli strumenti del connettore
</h3>

La tua organizzazione può impostare controlli per strumento sui [connettori claude.ai](https://claude.com/docs/connectors). Claude Code legge queste impostazioni all'avvio e le applica localmente, tranne nelle [sessioni locali e SSH](#how-connectors-reach-claude-code) dell'app desktop. Lì, l'app desktop trattiene gli strumenti `blocked` prima di fornire un connettore, e l'impostazione `ask` non raggiunge Claude Code, quindi applica le [regole di autorizzazione](/docs/it/permissions) ordinarie della sessione a quegli strumenti invece di richiedere su ogni chiamata. Nelle sessioni in cui Claude Code recupera i connettori da solo, esegui `/mcp` per vedere quale impostazione si applica a ogni strumento su un connettore.

* **Strumento impostato su `ask`**: Claude Code richiede su ogni chiamata con il motivo `Your organization requires approval for this tool`. Il prompt appare anche in modalità [permission](/docs/it/permissions#permission-modes) `acceptEdits`, `auto`, e `bypassPermissions`, e non offre mai un'opzione per ricordare la tua scelta. Le [regole Allow](/docs/it/permissions) che corrispondono allo strumento non saltano il prompt neanche. In modalità `dontAsk`, che non richiede mai, Claude Code nega la chiamata.
* **Strumento impostato su `blocked`**: Claude Code filtra lo strumento prima che Claude lo veda, quindi non appare mai nell'elenco degli strumenti. L'app desktop e la chat claude.ai applicano la stessa impostazione `blocked`, quindi Claude non può usare lo strumento neanche lì, e non puoi trattenere uno strumento dalle sessioni dell'app desktop mantenendolo disponibile in chat. L'app desktop salta un connettore i cui strumenti sono tutti bloccati.

<h3 id="disable-claude-ai-connectors">
  Disabilitare i connettori claude.ai
</h3>

Claude Code applica [`disableClaudeAiConnectors`](/docs/it/settings-reference#disableclaudeaiconnectors) solo ai connettori che [recupera da solo](#how-connectors-reach-claude-code), non ai connettori che un host cloud o l'app desktop fornisce. Per disattivare i connettori che recupera, imposta l'impostazione su `true` in qualsiasi ambito di impostazioni:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Questa impostazione utilizza la semantica any-source-true: `true` in qualsiasi fonte di impostazioni ha la precedenza. Un `.claude/settings.json` di progetto archiviato può escludere un repository dai connettori che Claude Code recupera da solo, ma un `false` a livello di progetto non può riabilitare i connettori che un `true` a livello di utente o politica ha disabilitato. I server passati esplicitamente tramite `--mcp-config` non sono interessati.

Puoi anche impostare la variabile di ambiente `ENABLE_CLAUDEAI_MCP_SERVERS` su `false`, che ha lo stesso effetto per la sessione shell corrente:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Per bloccare i singoli connettori claude.ai invece di tutti loro, aggiungili a [`deniedMcpServers`](/docs/it/managed-mcp) per nome o per pattern di URL. Ad esempio, una voce `serverName` di `"claude.ai Slack"` blocca il connettore Slack. Puoi anche eseguire `/mcp` per attivare o disattivare qualsiasi connettore che Claude Code recupera solo per il progetto corrente.

<h2 id="use-claude-code-as-an-mcp-server">
  Usa Claude Code come server MCP
</h2>

Puoi usare Claude Code stesso come server MCP a cui altre applicazioni possono connettersi:

```bash theme={null}
# Avvia Claude come server MCP stdio
claude mcp serve
```

Il comando non stampa nulla all'avvio. Un server MCP stdio comunica tramite stdin e stdout, quindi un terminale silenzioso e bloccato significa che il server è in esecuzione e in attesa che un client si connetta.

Puoi usarlo in Claude Desktop aggiungendo questa configurazione a claude\_desktop\_config.json:

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
  **Configurazione del percorso dell'eseguibile**: il campo `command` deve fare riferimento all'eseguibile di Claude Code. Se il comando `claude` non è nel PATH del tuo sistema, dovrai specificare il percorso completo dell'eseguibile.

  Per trovare il percorso completo:

  ```bash theme={null}
  which claude
  ```

  Quindi usa il percorso completo nella tua configurazione:

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

  * In Claude Desktop, prova a chiedere a Claude di leggere file in una directory, fare modifiche e altro ancora.
  * Questo server MCP espone solo gli strumenti di Claude Code al tuo client MCP, quindi il tuo client è responsabile dell'implementazione della conferma dell'utente per le singole chiamate di strumenti.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Limiti di output MCP e avvisi
</h2>

Quando gli strumenti MCP producono output di grandi dimensioni, Claude Code aiuta a gestire l'utilizzo dei token per evitare di sovraccaricare il contesto della conversazione:

* **Soglia di avviso di output**: Claude Code visualizza un avviso quando l'output di qualsiasi strumento MCP supera 10.000 token
* **Limite configurabile**: è possibile regolare il massimo numero di token di output MCP consentiti utilizzando la variabile di ambiente `MAX_MCP_OUTPUT_TOKENS`
* **Limite predefinito**: il massimo predefinito è 25.000 token
* **Ambito**: la variabile di ambiente si applica agli strumenti che non dichiarano il proprio limite. Gli strumenti che impostano [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) utilizzano invece quel valore per il contenuto di testo, indipendentemente da ciò che `MAX_MCP_OUTPUT_TOKENS` è impostato. Gli strumenti che restituiscono dati di immagine sono comunque soggetti a `MAX_MCP_OUTPUT_TOKENS`
* **Oltre il limite**: quando un risultato senza contenuto di immagine supera il limite, Claude Code lo salva in un file e lo sostituisce nella conversazione con un messaggio che nomina il percorso del file, in modo che Claude legga il file quando ha bisogno del contenuto. Il file si trova nella directory `tool-results` della sessione sotto [`~/.claude/projects/`](/docs/it/claude-directory#cleaned-up-automatically).

Per aumentare il limite per gli strumenti che producono output di grandi dimensioni:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

<h3 id="raise-the-limit-for-a-specific-tool">
  Aumentare il limite per uno strumento specifico
</h3>

Se state creando un server MCP, potete consentire ai singoli strumenti di restituire risultati più grandi della soglia predefinita di persistenza su disco impostando `_meta["anthropic/maxResultSizeChars"]` nella voce di risposta `tools/list` dello strumento. Claude Code aumenta la soglia di quello strumento al valore annotato, fino a un limite massimo di 500.000 caratteri.

Questo è utile per gli strumenti che restituiscono output intrinsecamente grandi ma necessari, come schemi di database o alberi di file completi. Senza l'annotazione, i risultati che superano la soglia predefinita vengono persistiti su disco e sostituiti con un riferimento a file nella conversazione.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

L'annotazione si applica indipendentemente da `MAX_MCP_OUTPUT_TOKENS` per il contenuto di testo, quindi gli utenti non hanno bisogno di aumentare la variabile di ambiente per gli strumenti che la dichiarano. Gli strumenti che restituiscono dati di immagine sono comunque soggetti al limite di token.

<Warning>
  Se incontrate frequentemente avvisi di output con server MCP specifici che non controllate, considerate di aumentare il limite `MAX_MCP_OUTPUT_TOKENS`. Potete anche chiedere all'autore del server di aggiungere l'annotazione `anthropic/maxResultSizeChars` o di impaginare le loro risposte. L'annotazione non ha effetto sugli strumenti che restituiscono contenuto di immagine; per quelli, aumentare `MAX_MCP_OUTPUT_TOKENS` è l'unica opzione.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Tool input schemas con un combinatore a livello radice
</h2>

Alcuni server MCP dichiarano lo schema di input di uno strumento come un'unione JSON Schema, con `anyOf`, `oneOf`, o `allOf` al livello superiore dello schema. L'API Claude non accetta queste parole chiave alla radice dello schema. Accetta combinatori annidati all'interno di `properties`, che Claude Code invia invariati.

Gli strumenti con un combinatore a livello radice rimangono disponibili. Prima di inviare lo strumento all'API, Claude Code appiattisce lo schema in un singolo oggetto e antepone una frase alla descrizione dello strumento che dice a Claude quali gruppi di parametri appartengono insieme:

* `allOf`: le proprietà di ogni ramo vengono unite, e l'elenco `required` di ogni ramo si applica ancora
* `anyOf` e `oneOf`: le proprietà di ogni ramo vengono unite, e l'elenco `required` di ogni ramo viene descritto nella descrizione dello strumento invece di essere applicato dallo schema

Il server riceve gli argomenti che Claude ha scelto, quindi continuate a convalidare la combinazione lato server.

Quando Claude Code non riesce a produrre uno schema che l'API accetta, o su una distribuzione che non riceve la configurazione remota che abilita la riscrittura, salta quello strumento, registra il motivo nel log del server e lascia disponibili gli altri strumenti del server. Le versioni precedenti a v2.1.195 saltano ogni strumento il cui schema di input ha un `anyOf`, `oneOf`, o `allOf` a livello radice.

<h2 id="tools-with-invalid-input-schemas">
  Strumenti con schemi di input non validi
</h2>

L'API Claude controlla lo schema di input di ogni strumento in una richiesta e rifiuta l'intera richiesta quando uno schema non supera il controllo, quindi un singolo strumento MCP con uno schema malformato farebbe fallire ogni richiesta che lo include con un errore 400. Claude Code esegue due dei controlli dell'API da solo quando carica gli strumenti di un server ed esclude ogni strumento che non li supererebbe, in modo che gli altri strumenti del server continuino a funzionare:

* I nomi delle proprietà di primo livello devono essere lunghi da 1 a 64 caratteri e utilizzare solo lettere ASCII e cifre, `_`, `.` e `-`
* Lo schema deve essere valido rispetto al meta-schema JSON Schema draft 2020-12. Claude Code applica questo controllo agli schemi che non dichiarano alcun `$schema` e agli schemi che dichiarano draft 2020-12. Uno schema che dichiara qualsiasi altro dialetto salta questo controllo, anche se il controllo del nome della proprietà sopra indicato si applica comunque

Claude Code esegue i controlli dopo la [riscrittura del combinatore a livello di root](#tool-input-schemas-with-a-root-level-combinator), sullo schema che invierebbe effettivamente.

Quando Claude Code esclude uno strumento, registra il motivo nel log del server e comunica a Claude quali strumenti ha escluso e perché, in modo che tu possa chiedere a Claude perché uno strumento manca. Se correggi lo schema sul server, lo strumento riappare la prossima volta che Claude Code carica gli strumenti del server.

Claude Code attiva l'esclusione tramite un feature flag che recupera da Anthropic. Su una [distribuzione in cui il recupero dei flag è disattivato](/docs/it/env-vars#features-that-need-feature-flag-fetching), o su una macchina i cui flag non sono mai arrivati, come una macchina air-gapped, Claude Code esegue comunque i controlli e registra nel log del server quale strumento verrebbe rifiutato, ma invia comunque lo schema dello strumento all'API. L'API rifiuta una richiesta che include quello schema con [un errore 400 che nomina lo strumento in base alla sua posizione](/docs/it/errors#tool-input-schema-is-invalid). Prima della v2.1.216, nessuna distribuzione eseguiva questi controlli.

La [gestione del combinatore a livello di root](#tool-input-schemas-with-a-root-level-combinator) è separata e mantiene il suo comportamento quando il recupero dei flag è disattivato o i flag non sono mai arrivati.

<h2 id="require-approval-for-a-specific-tool">
  Richiedere approvazione per uno strumento specifico
</h2>

Se state creando un server MCP, potete contrassegnare uno strumento come richiedente approvazione esplicita ad ogni chiamata impostando `_meta["anthropic/requiresUserInteraction"]` a `true` nella voce di risposta `tools/list` dello strumento. Il valore deve essere il booleano JSON `true`; qualsiasi altro valore viene ignorato.

Claude Code mostra il prompt di autorizzazione di quello strumento ad ogni chiamata, anche in modalità di autorizzazione `acceptEdits`, `auto` e `bypassPermissions` [permission modes](/docs/it/permissions#permission-modes), e non offre un'opzione "non chiedere di nuovo" per esso. Le [Allow rules](/docs/it/permissions#permission-rule-syntax) che corrispondono allo strumento non saltano il prompt neanche. In modalità `dontAsk`, che non chiede mai, Claude Code nega la chiamata.

Il prompt deve raggiungere una persona. In modalità non interattiva con [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags), un risultato `allow` dal tool di prompt per uno strumento contrassegnato viene convertito in un rifiuto con il messaggio `MCP tool requires user interaction; not supported via --permission-prompt-tool`. Il callback [`canUseTool`](/docs/it/agent-sdk/permissions) dell'Agent SDK riceve effettivamente queste chiamate e può approvarle, perché la vostra applicazione SDK è prevista che le mostri a un utente.

Utilizzate questa funzione per strumenti il cui prompt di autorizzazione è esso stesso il punto, come un passaggio di consenso o concessione di accesso dove l'approvazione automatica significherebbe che nessun essere umano ha mai acconsentito. Gli altri strumenti dello stesso server mantengono il loro comportamento di autorizzazione normale.

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

L'annotazione `anthropic/requiresUserInteraction` richiede Claude Code v2.1.199 o successivo. Le versioni precedenti la ignorano e applicano il flusso di autorizzazione standard.

Alcune superfici, come [Remote Control](/docs/it/remote-control) e applicazioni costruite su [Agent SDK](/docs/it/agent-sdk/overview), normalmente vi permettono di approvare le chiamate di strumenti con un tocco. Per uno strumento contrassegnato con questa annotazione, Claude Code trattiene l'azione a un tocco e mostra il prompt di autorizzazione completo dello strumento, così l'approvazione viene ancora da una persona che risponde al prompt piuttosto che da un tocco.

Claude Code trattiene l'approvazione a un tocco allo stesso modo per qualsiasi richiesta di autorizzazione che solo il dialogo del terminale può rendere completamente, come una che porta un avviso di sicurezza o un'opzione di sempre-consenti che la superficie remota non può mostrare. Rispondete a quella richiesta nel dialogo del terminale piuttosto che da Remote Control. Richiede Claude Code v2.1.214 o successivo.

<h2 id="respond-to-mcp-elicitation-requests">
  Rispondere alle richieste di elicitazione MCP
</h2>

I server MCP possono richiedere input strutturato da voi durante un'attività utilizzando l'elicitazione. Quando un server ha bisogno di informazioni che non può ottenere da solo, Claude Code visualizza una finestra di dialogo interattiva e trasmette la vostra risposta al server. Non è richiesta alcuna configurazione da parte vostra: le finestre di dialogo di elicitazione vengono visualizzate automaticamente quando un server le richiede.

I server possono richiedere input in due modi:

* **Modalità modulo**: Claude Code mostra una finestra di dialogo con campi modulo definiti dal server (ad esempio, un prompt di nome utente e password). Compilate i campi e inviate.
* **Modalità URL**: Claude Code apre un URL del browser per l'autenticazione o l'approvazione. Completate il flusso nel browser, quindi confermate nella CLI.

In modalità URL, Claude Code trasmette l'URL come argomento della riga di comando al gestore URL del vostro sistema e limita la lunghezza di tale argomento. Quando l'URL, una volta sottoposto a escape per la riga di comando, supera tale limite, potete solo rifiutare la richiesta. Ogni carattere che necessita di escape, come `%` o `&`, conta quattro volte verso il limite: il suo carattere più tre caratteri di escape. Un URL senza nessuno di essi raggiunge il limite a circa 8.000 caratteri. Un URL costruito principalmente con escape percentuali, dove ogni terzo carattere è un `%`, lo raggiunge a circa 4.000.

Per rispondere automaticamente alle richieste di elicitazione senza mostrare una finestra di dialogo, utilizzate l'hook [`Elicitation`](/docs/it/hooks#elicitation).

Se state creando un server MCP che utilizza l'elicitazione, consultate la [specifica di elicitazione MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) per i dettagli del protocollo e gli esempi di schema.

<h2 id="use-mcp-resources">
  Utilizzare le risorse MCP
</h2>

I server MCP possono esporre risorse che è possibile referenziare utilizzando menzioni @, in modo simile a come si referenziano i file.

<h3 id="reference-mcp-resources">
  Referenziare le risorse MCP
</h3>

<Steps>
  <Step title="Elencare le risorse disponibili">
    Digiti `@` nel suo prompt per visualizzare le risorse disponibili da tutti i server MCP connessi. Le risorse vengono visualizzate insieme ai file nel menu di completamento automatico.
  </Step>

  <Step title="Referenziare una risorsa specifica">
    Utilizzi il formato `@server:protocol://resource/path` per referenziare una risorsa:

    ```text wrap theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text wrap theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Riferimenti a risorse multiple">
    È possibile referenziare più risorse in un singolo prompt:

    ```text wrap theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Le risorse vengono recuperate automaticamente e incluse come allegati quando referenziate
  * I percorsi delle risorse sono ricercabili in modo fuzzy nel completamento automatico della menzione @
  * Claude Code fornisce automaticamente strumenti per elencare e leggere le risorse MCP quando i server le supportano
  * Le risorse possono contenere qualsiasi tipo di contenuto fornito dal server MCP (testo, JSON, dati strutturati, ecc.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Scalare con la ricerca di strumenti MCP
</h2>

La ricerca di strumenti mantiene basso l'utilizzo del contesto MCP rimandando le definizioni degli strumenti fino a quando Claude non ne ha bisogno. Solo i nomi degli strumenti e le istruzioni del server si caricano all'inizio della sessione, quindi aggiungere più server MCP ha un impatto minimo sulla finestra di contesto. Claude Code non impone un limite fisso di strumenti per server; il limite pratico è il budget della finestra di contesto.

<Note>
  La ricerca di strumenti non è supportata su Microsoft Foundry [distribuzioni ospitate su Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), che la rifiutano lato server: Claude Code rileva il rifiuto e carica gli strumenti MCP in anticipo per quella distribuzione. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) non può ignorare questo, poiché il rifiuto proviene dalla distribuzione stessa.
</Note>

<h3 id="for-mcp-server-authors">
  Per gli autori di server MCP
</h3>

Se state creando un server MCP, il campo delle istruzioni del server diventa più utile con la ricerca di strumenti abilitata. Le istruzioni del server aiutano Claude a capire quando cercare i vostri strumenti, in modo simile a come funzionano le [skills](/docs/it/skills).

Aggiungete istruzioni del server chiare e descrittive che spieghino:

* Quale categoria di attività gestiscono i vostri strumenti
* Quando Claude dovrebbe cercare i vostri strumenti
* Capacità chiave che il vostro server fornisce

Claude Code tronca le descrizioni degli strumenti e le istruzioni del server a 2.048 caratteri per impostazione predefinita. Mantenetele concise e mettete i dettagli critici all'inizio.

Per modificare il limite per ogni server MCP nella vostra sessione, impostate [`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](/docs/it/env-vars#variables) a un numero di caratteri. Questa variabile richiede Claude Code v2.1.280 o successivo.

<h3 id="configure-tool-search">
  Configurare la ricerca di strumenti
</h3>

La ricerca di strumenti è abilitata per impostazione predefinita: gli strumenti MCP vengono rimandati e scoperti su richiesta. Claude Code la disabilita quando `ANTHROPIC_BASE_URL` punta a un host non di prima parte, poiché la maggior parte dei proxy non inoltrano i blocchi `tool_reference`. Impostate `ENABLE_TOOL_SEARCH` esplicitamente per ignorare quel fallback.

L'impostazione [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/it/env-vars) mantiene la ricerca di strumenti disattivata. Non potete ignorarla impostando `ENABLE_TOOL_SEARCH` voi stessi. La vostra organizzazione può mantenere la ricerca di strumenti attiva tramite [impostazioni gestite](/docs/it/managed-settings), su Claude Code v2.1.227 o successivo. [Disabilitare le capacità pre-release](/docs/it/llm-gateway-protocol#disable-pre-release-capabilities) copre dove si applica l'override e cosa fa la variabile.

La ricerca di strumenti richiede un modello che supporti i blocchi `tool_reference`: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 e modelli successivi. Consultate [la compatibilità dei modelli nella documentazione API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) per l'elenco attuale.

Su Google Cloud's Agent Platform, Claude Code decide in base alla generazione del modello:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 e successivi**: la ricerca di strumenti è attiva per impostazione predefinita, come su Anthropic API.
* **Modelli precedenti di Agent Platform**: Claude Code carica tutti gli strumenti MCP in anticipo, perché i loro stack di servizio rifiutano l'intestazione beta richiesta. `ENABLE_TOOL_SEARCH=true` non ignora questo.

Prima della v2.1.221, Claude Code disabilitava la ricerca di strumenti per tutti i modelli su Google Cloud's Agent Platform a meno che non impostaste `ENABLE_TOOL_SEARCH=true`.

Controllate il comportamento della ricerca di strumenti con la variabile di ambiente `ENABLE_TOOL_SEARCH`:

| Valore          | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| (non impostato) | Tutti gli strumenti MCP rimandati e caricati su richiesta. Ritorna al caricamento anticipato su modelli di Google Cloud's Agent Platform precedenti alla generazione Claude 4.5, quando `ANTHROPIC_BASE_URL` è un host non di prima parte, o su una distribuzione Microsoft Foundry ospitata su Azure                                                                                                                                                                     |
| `true`          | Tutti gli strumenti MCP rimandati, tranne su una distribuzione Microsoft Foundry ospitata su Azure, dove il rifiuto lato server forza comunque il caricamento anticipato, e su modelli di Google Cloud's Agent Platform precedenti alla generazione Claude 4.5, dove Claude Code continua a caricare gli strumenti in anticipo. Claude Code invia l'intestazione beta attraverso i proxy e le richieste falliscono su proxy che non supportano i blocchi `tool_reference` |
| `auto`          | Modalità soglia: Claude Code carica gli strumenti che altrimenti rimanda in anticipo mentre le loro definizioni totalizzano meno del 10% della finestra di contesto, e rimanda tutti una volta che le definizioni raggiungono il 10%                                                                                                                                                                                                                                      |
| `auto:N`        | Modalità soglia con una percentuale personalizzata, dove `N` è 0-100. Ad esempio, `auto:5` per il 5%                                                                                                                                                                                                                                                                                                                                                                      |
| `false`         | Tutti gli strumenti MCP caricati in anticipo, nessun rinvio                                                                                                                                                                                                                                                                                                                                                                                                               |

```bash theme={null}
# Usa una soglia personalizzata del 5%
ENABLE_TOOL_SEARCH=auto:5 claude

# Disabilita completamente la ricerca di strumenti
ENABLE_TOOL_SEARCH=false claude
```

Oppure impostate il valore nel campo `env` di [settings.json](/docs/it/settings-reference#env).

Potete anche disabilitare lo strumento `ToolSearch` specificamente:

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

Se gli strumenti di un server dovrebbero essere sempre visibili a Claude senza un passaggio di ricerca, impostate `alwaysLoad` a `true` nella configurazione di quel server. Ogni strumento da quel server viene quindi caricato nel contesto all'inizio della sessione indipendentemente dall'impostazione `ENABLE_TOOL_SEARCH`. Usate questo per un piccolo numero di strumenti che Claude necessita ad ogni turno, poiché ogni strumento anticipato consuma contesto che altrimenti sarebbe disponibile per la vostra conversazione.

La seguente voce `.mcp.json` esentera un server HTTP mentre lascia gli altri server rimandati:

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

Il campo `alwaysLoad` è disponibile su tutti i tipi di server. Un server MCP può anche contrassegnare singoli strumenti come sempre caricati includendo `"anthropic/alwaysLoad": true` nell'oggetto `_meta` dello strumento, che ha lo stesso effetto solo per quello strumento.

L'impostazione `alwaysLoad: true` fa anche sì che l'avvio attenda gli strumenti del server, limitato al timeout di connessione standard di 5 secondi, poiché devono essere presenti quando viene costruito il primo prompt. Un server remoto con una voce [`cached`](#server-status-detail) valida fornisce i suoi strumenti dalla cache senza connettersi, quindi non ritarda l'avvio. Gli altri server si connettono in background per impostazione predefinita; impostate [`MCP_CONNECTION_NONBLOCKING=0`](/docs/it/env-vars) per far sì che l'avvio li attenda anche.

<h2 id="use-mcp-prompts-as-commands">
  Usa i prompt MCP come comandi
</h2>

I server MCP possono esporre prompt che diventano disponibili come comandi in Claude Code.

<h3 id="execute-mcp-prompts">
  Esegui i prompt MCP
</h3>

<Steps>
  <Step title="Scopri i prompt disponibili">
    Digita `/` per vedere i comandi disponibili, inclusi quelli dai server MCP. Claude Code elenca ogni prompt MCP come `/servername:promptname (MCP)`. Digitando `/mcp__servername__promptname` lo esegui anche.
  </Step>

  <Step title="Esegui un prompt senza argomenti">
    ```text wrap theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Esegui un prompt con argomenti">
    Molti prompt accettano argomenti. Passali separati da spazi dopo il comando. Claude Code divide gli argomenti su spazi bianchi, quindi ogni argomento è un singolo token:

    ```text wrap theme={null}
    /mcp__github__pr_review 456
    ```

    ```text wrap theme={null}
    /mcp__jira__create_issue login-bug high
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * I prompt MCP vengono scoperti dinamicamente dai server connessi
  * Gli argomenti vengono analizzati in base ai parametri definiti del prompt
  * I risultati del prompt vengono iniettati direttamente nella conversazione
  * Nel modulo `/mcp__servername__promptname`, Claude Code sostituisce qualsiasi carattere nel nome del server al di fuori di `A-Z`, `a-z`, `0-9`, `_` e `-` con `_`, e utilizza il nome del prompt come dichiarato dal server
</Tip>

<h2 id="managed-mcp-configuration">
  Configurazione MCP gestita
</h2>

Per le organizzazioni che necessitano di un controllo centralizzato su quali server MCP gli utenti possono connettere, vedere [Configurazione MCP gestita](/docs/it/managed-mcp). Copre la distribuzione di un set di server fisso con `managed-mcp.json`, la fornitura di server a ogni utente con `managedMcpServers`, la restrizione dei server con `allowedMcpServers` e `deniedMcpServers`, e ciò che gli utenti vedono quando un server è bloccato.
