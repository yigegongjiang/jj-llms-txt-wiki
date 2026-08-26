> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Connettere i server MCP

> Aggiungere un server MCP a Claude Code, verificare la connessione e trovare la configurazione su disco.

Il [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction) consente a Claude Code di utilizzare strumenti oltre il suo set integrato, come la ricerca in un tracker di problemi, l'interrogazione di un database o il controllo di un browser web. Questi strumenti provengono dai server MCP, che vengono eseguiti sulla vostra macchina o come servizi ospitati.

Questa guida vi guida attraverso la connessione di un server MCP end-to-end con la CLI di Claude Code. Alla fine, avrete un server connesso e rispondente, saprete dove vive la sua configurazione su disco e saprete come risolvere gli errori di connessione più comuni.

<Note>
  Potete anche aggiungere server MCP da altre superfici, inclusa l'app desktop, VS Code e il web. Vedere [Connettere da altre superfici](#connect-from-other-surfaces).
</Note>

Per ogni modo di connettere e configurare i server MCP in Claude Code, consultare il [riferimento MCP](/docs/it/mcp).

<h2 id="before-you-begin">
  Prima di iniziare
</h2>

Assicuratevi di avere:

* [Claude Code installato](/docs/it/quickstart) e autenticato
* Un terminale aperto in una directory di progetto. Qualsiasi directory funziona, inclusa una vuota.

<h2 id="add-and-verify-a-server">
  Aggiungere e verificare un server
</h2>

L'esempio seguente si connette al [server MCP della documentazione di Claude Code](https://code.claude.com/docs/mcp), un server ospitato con ricerca full-text sulla documentazione di Claude Code. Non richiede autenticazione o alcuna configurazione speciale, quindi funziona bene come primo server per testare il flusso di configurazione.

I passaggi sono gli stessi per qualsiasi server: aggiungerlo, controllare lo stato della connessione, quindi utilizzarlo in una sessione, con un passaggio di pulizia facoltativo alla fine. Alcuni server aggiungono un passaggio, come un accesso al browser, mostrato in [Esempi di server MCP aggiuntivi](#additional-mcp-server-examples). Per altri server a cui connettersi, consultare la [Directory Anthropic](/docs/it/mcp#find-and-build-mcp-servers).

<Steps>
  <Step title="Aggiungere il server MCP">
    Registrare il server con Claude Code. Eseguire questo nel vostro terminale, non all'interno di una sessione `claude`: state configurando il server prima di avviare una conversazione.

    ```bash theme={null}
    claude mcp add --transport http claude-code-docs https://code.claude.com/docs/mcp
    ```

    Le parti del comando:

    * `claude mcp add`: registra un server con Claude Code.
    * `--transport http`: il server è ospitato a un URL piuttosto che eseguito come processo locale.
    * `claude-code-docs`: un nome che voi scegliete. Chiamare lo stesso server `docs` funzionerebbe in modo identico. Claude Code utilizza qualsiasi nome voi scegliate per etichettare gli strumenti del server nell'output di Claude e per fare riferimento al server in comandi come `claude mcp remove`.
    * `https://code.claude.com/docs/mcp`: l'URL dove il server è ospitato.

    Il comando stampa una conferma come `Added HTTP MCP server claude-code-docs with URL: https://code.claude.com/docs/mcp to local config`. La parte `local config` significa che il server è registrato per voi, in questo progetto: se avviate Claude Code in un progetto diverso, questo server non è attivo lì. Per registrare un server una volta per tutti i vostri progetti, aggiungetelo a livello di utente, coperto in [Cambiare l'ambito del server](#change-server-scope).
  </Step>

  <Step title="Controllare lo stato della connessione">
    Confermare che il server appaia nell'elenco dei server e controllare il suo stato:

    ```bash theme={null}
    claude mcp list
    ```

    Il server appare con un indicatore di stato:

    | Stato                              | Significato                                                                                                                                                                                    |
    | :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `✓ Connected`                      | Pronto all'uso. Questo è quello che dovreste vedere per `claude-code-docs`                                                                                                                     |
    | `! Connected · tools fetch failed` | Il server si è connesso ma non ha potuto elencare i suoi strumenti. Eseguire `claude mcp get <name>` per il dettaglio dell'errore                                                              |
    | `! Needs authentication`           | Il server è raggiungibile ma necessita un accesso al browser, o un token passato con `--header`. Vedere [Connettere un server che richiede l'accesso](#connect-a-server-that-requires-sign-in) |
    | `✗ Failed to connect`              | Il server non ha risposto. Vedere [Risoluzione dei problemi](#troubleshooting)                                                                                                                 |
    | `✗ Connection error`               | Il tentativo di connessione ha generato un errore. Vedere [Risoluzione dei problemi](#troubleshooting)                                                                                         |
    | `⏸ Pending approval`               | Un server con ambito di progetto che non avete ancora approvato. Vedere [Modificare .mcp.json direttamente](#edit-mcp-json-directly)                                                           |
  </Step>

  <Step title="Utilizzare il server">
    Avviare una sessione e chiedere a Claude di utilizzare il nuovo server per nome:

    ```bash theme={null}
    claude
    ```

    ```text theme={null}
    Use the claude-code-docs server to look up what MCP_TIMEOUT does
    ```

    <Info>
      Normalmente non è necessario nominare un server nel vostro prompt, poiché Claude sceglie gli strumenti rilevanti da solo. Nominarlo qui garantisce che la dimostrazione passi attraverso il nuovo server piuttosto che un altro strumento, come web fetch, che potrebbe rispondere alla stessa domanda.
    </Info>

    La prima volta che Claude chiama il server, chiede il permesso di utilizzare il nuovo strumento. Approvate per continuare. La chiamata dello strumento nell'output di Claude è etichettata con il nome del server, che è come confermate che la risposta proviene dal server MCP piuttosto che dalla conoscenza integrata di Claude.
  </Step>

  <Step title="Rimuovere il server">
    Questo passaggio è facoltativo. Quando avete finito di sperimentare, potete rimuovere il server:

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    <Note>
      Ogni server connesso occupa dello spazio nella [finestra di contesto di Claude](/docs/it/how-claude-code-works#the-context-window) perché i nomi degli strumenti e le istruzioni del server si caricano in ogni sessione. Rimuovere i server che non utilizzate più mantiene quello spazio libero.
    </Note>
  </Step>
</Steps>

<h2 id="where-servers-are-saved">
  Dove vengono salvati i server
</h2>

Il comando `claude mcp add` scrive i dettagli del server in un file di configurazione. Per impostazione predefinita registra il server a livello di ambito `local`: privato per voi, attivo solo nel progetto corrente. Passare `--scope user` per registrarlo una volta per tutti i vostri progetti, o `--scope project` per condividerlo con i vostri compagni di squadra. [Cambiare l'ambito del server](#change-server-scope) illustra entrambi.

<Note>
  `claude mcp add` funziona allo stesso modo in ogni shell, inclusi PowerShell e Command Prompt. All'interno di una sessione `claude`, utilizzare il comando `/mcp` per controllare e gestire i server che avete già aggiunto.
</Note>

Ci sono altri modi per aggiungere un server, ognuno coperto più avanti in questa pagina:

* [Aggiungere un server locale](#add-a-local-server): eseguire un programma sulla vostra macchina invece di connettersi a un URL.
* [Modificare `.mcp.json` direttamente](#edit-mcp-json-directly): scrivere l'entry JSON voi stessi invece di utilizzare il comando.
* [Connettere un server che richiede l'accesso](#connect-a-server-that-requires-sign-in): aggiungere un server ospitato che necessita un accesso al browser prima che i suoi strumenti funzionino.

<h3 id="find-your-configuration-on-disk">
  Trovare la vostra configurazione su disco
</h3>

Il comando `claude mcp add` scrive il server in uno di tre ambiti, archiviati in due file, a seconda del flag `--scope`. Non è necessario modificare questi file direttamente, ma sapere dove si trovano aiuta con il debug e il controllo della versione.

| Ambito    | File                                                                | Disponibile per                                            |
| :-------- | :------------------------------------------------------------------ | :--------------------------------------------------------- |
| `local`   | `~/.claude.json`, sotto l'entry per questo progetto                 | Solo voi, solo questo progetto. L'impostazione predefinita |
| `project` | `.mcp.json` nella radice del vostro progetto                        | Chiunque cloni il progetto                                 |
| `user`    | `~/.claude.json`, sotto la chiave `mcpServers` di livello superiore | Solo voi, tutti i progetti                                 |

Su Windows, `~/.claude.json` si risolve in `%USERPROFILE%\.claude.json`, tipicamente `C:\Users\YourName\.claude.json`. Se avete impostato [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars), Claude Code legge `.claude.json` da dentro quella directory invece.

Eseguire `claude mcp get claude-code-docs` per vedere quale ambito contiene la definizione di un server. Per come gli ambiti interagiscono quando lo stesso server è definito in più di uno, vedere [Ambiti di installazione MCP](/docs/it/mcp#mcp-installation-scopes).

<h2 id="change-server-scope">
  Cambiare l'ambito del server
</h2>

L'ambito di un server è fisso quando lo aggiungete, quindi cambiare l'ambito significa rimuovere l'entry e riaggiungerla a quello nuovo. Entrambi i casi seguenti iniziano rimuovendo l'entry locale dalla prima procedura dettagliata, in modo che il server abbia una sola definizione. Se l'avete già rimosso alla fine di quella procedura dettagliata, saltate questo comando:

```bash theme={null}
claude mcp remove claude-code-docs --scope local
```

<h3 id="use-a-server-in-all-your-projects">
  Utilizzare un server in tutti i vostri progetti
</h3>

Riaggiunger il server a livello di ambito `user` per renderlo attivo in ogni progetto che aprite, ancora privato per voi:

```bash theme={null}
claude mcp add --scope user --transport http claude-code-docs https://code.claude.com/docs/mcp
```

<h3 id="share-a-server-with-your-team">
  Condividere un server con il vostro team
</h3>

Riaggiunger il server a livello di ambito `project`, che scrive in `.mcp.json` nella radice del progetto:

```bash theme={null}
claude mcp add --scope project --transport http claude-code-docs https://code.claude.com/docs/mcp
```

Eseguire il commit di `.mcp.json` al controllo della versione. I compagni di squadra che clonano il repository e avviano Claude Code vedono un prompt per approvare il server, quindi si connette anche per loro.

<h2 id="additional-mcp-server-examples">
  Esempi di server MCP aggiuntivi
</h2>

La prima procedura dettagliata ha utilizzato un server ospitato che si connette senza alcun accesso. Gli esempi seguenti coprono le altre due forme comuni, con lo stesso flusso di aggiunta, controllo, utilizzo.

<h3 id="add-a-local-server">
  Aggiungere un server locale
</h3>

Un server stdio locale è un programma che Claude Code avvia come sottoprocesso sulla vostra macchina, piuttosto che un servizio che raggiunge tramite un URL. Utilizzatene uno per strumenti che necessitano accesso a risorse locali come un browser, il vostro filesystem o un socket di database.

Il [server MCP Playwright](https://github.com/microsoft/playwright-mcp) è un buono da provare: dà a Claude un browser che può navigare, fare clic e leggere, e non necessita alcun account. Viene eseguito tramite `npx`, quindi richiede [Node.js](https://nodejs.org/en/download) 18 o successivo.

<Steps>
  <Step title="Aggiungere il server Playwright">
    Registrare il server con il comando che Claude Code dovrebbe eseguire per avviarlo:

    ```bash theme={null}
    claude mcp add playwright -- npx -y @playwright/mcp@latest
    ```

    Questo comando differisce dall'esempio ospitato in tre modi:

    * Non c'è il flag `--transport`, perché i server locali utilizzano il trasporto predefinito `stdio`.
    * Tutto dopo il separatore `--` è il comando che Claude Code esegue per avviare il server.
    * `-y` dice a `npx` di installare il pacchetto senza chiedere.

    Playwright guida qualsiasi Chrome già installato sulla vostra macchina. Per utilizzare un browser diverso, aggiungere `--browser` con il nome del browser, ad esempio `--browser firefox`, dopo `@playwright/mcp@latest`.
  </Step>

  <Step title="Controllare la connessione">
    La conferma `Added` significa che l'entry è stata salvata, non che il comando viene eseguito. Controllare la connessione:

    ```bash theme={null}
    claude mcp list
    ```

    Il primo controllo può mostrare `✗ Failed to connect` mentre `npx` scarica il pacchetto, quindi attendere un momento ed eseguirlo di nuovo.
  </Step>

  <Step title="Utilizzare il browser">
    Dare a Claude un compito che necessita il browser:

    ```text theme={null}
    Use playwright to open https://example.com and tell me the page title
    ```

    Una finestra del browser si apre in modo che possiate vederla funzionare, e le chiamate dello strumento nell'output di Claude sono etichettate con il nome del server `playwright` e l'azione, come `browser_navigate`.

    Provate a puntarlo al vostro server di sviluppo locale per controllare che una pagina ancora si renderizzi dopo una modifica, o fatelo camminare attraverso un rapporto di bug passo dopo passo.
  </Step>
</Steps>

<h3 id="connect-a-server-that-requires-sign-in">
  Connettere un server che richiede l'accesso
</h3>

Servizi ospitati come Sentry, Linear e Notion eseguono i loro server MCP dietro OAuth: aggiungete l'URL del server, quindi accedete tramite il vostro browser.

I passaggi seguenti utilizzano Sentry come esempio. Per connettere un servizio diverso, sostituire il suo URL, che potete trovare nella [Directory Anthropic](/docs/it/mcp#find-and-build-mcp-servers) o nella documentazione del servizio.

<Steps>
  <Step title="Aggiungere il server">
    Il comando `add` è lo stesso che per il server docs, con l'URL di Sentry:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```

    Dopo l'aggiunta, `claude mcp list` mostra il server con `! Needs authentication`. È previsto: il passaggio successivo completa l'accesso.
  </Step>

  <Step title="Autenticarsi nel vostro browser">
    Avviare una sessione di Claude Code e aprire il pannello MCP:

    ```text theme={null}
    /mcp
    ```

    Selezionare `sentry` dall'elenco, premere Invio e scegliere `Authenticate`. Il vostro browser si apre alla pagina di accesso di Sentry. Approvare la connessione lì.

    Di nuovo in Claude Code, lo stato del server cambia a connesso. Se l'accesso fallisce o il browser non si apre, vedere [Risoluzione dei problemi](#troubleshooting).
  </Step>

  <Step title="Utilizzare il server">
    Chiedere a Claude qualcosa che necessita il servizio, come `What Sentry projects do I have access to?`, e cercare le chiamate dello strumento etichettate con il nome del server `sentry` nel suo output.
  </Step>
</Steps>

I server che si autenticano con un token statico invece di OAuth prendono il token al momento dell'aggiunta con `--header "Authorization: Bearer <token>"`. Vedere l'[esempio GitHub](/docs/it/mcp#example-connect-to-github-for-code-reviews) per una versione elaborata.

<h2 id="edit-mcp-json-directly">
  Modificare .mcp.json direttamente
</h2>

Ogni file nella [tabella degli ambiti](#find-your-configuration-on-disk) utilizza lo stesso formato JSON per le entry dei server. Questa sezione modifica `.mcp.json`, il file con ambito di progetto. È quello che vale più la pena scrivere a mano perché è controllato nel repository, dove funge anche da configurazione come codice per il vostro team.

Creare `.mcp.json` nella radice del vostro progetto. L'esempio seguente definisce entrambi i server da questa guida, il server docs ospitato raggiunto tramite HTTP e il server Playwright come processo `stdio` locale:

```json theme={null}
{
  "mcpServers": {
    "claude-code-docs": {
      "type": "http",
      "url": "https://code.claude.com/docs/mcp"
    },
    "playwright": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@playwright/mcp@latest"]
    }
  }
}
```

I campi differiscono per tipo di server:

* Per i server HTTP, `url` è l'endpoint a cui Claude Code si connette.
* Per i server stdio, `command` e `args` sono il programma che esegue.

Dopo aver salvato il file, avviare una nuova sessione di Claude Code nel progetto. Claude Code legge `.mcp.json` all'avvio.

La prima volta che Claude Code vede un server con ambito di progetto, vi chiede di approvarlo. Il prompt esiste in modo che un repository che clonate non possa lanciare processi sulla vostra macchina senza il vostro consenso. Approvate il prompt, o eseguite `/mcp` per approvare più tardi se l'avete perso.

Una volta approvato, eseguire `/mcp` e controllare che i server mostrino come connessi. Se uno mostra un errore invece, vedere [Risoluzione dei problemi](#troubleshooting).

<h2 id="connect-from-other-surfaces">
  Connettere da altre superfici
</h2>

Questa guida utilizza i comandi CLI `claude mcp`, ma ogni superficie di Claude Code può connettersi ai server MCP:

* **App desktop Claude Code**: aggiungere server tramite l'[UI Connectors](/docs/it/desktop#connect-external-tools).
* **App Claude Desktop chat**: un'app separata da Claude Code. Per copiare i server dal suo `claude_desktop_config.json` nella CLI, eseguire `claude mcp add-from-claude-desktop` su macOS o WSL.
* **VS Code**: vedere [Connettere a strumenti esterni con MCP](/docs/it/vs-code#connect-to-external-tools-with-mcp).
* **Claude Code sul web**: legge `.mcp.json` dal vostro repository. Vedere [Modificare .mcp.json direttamente](#edit-mcp-json-directly).
* **Claude.ai**: i connettori che aggiungete a [claude.ai/customize/connectors](https://claude.ai/customize/connectors) si caricano automaticamente nella CLI quando accedete con quell'account. Vedere [Utilizzare i server MCP da Claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai).

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Se un server non si connette, controllare il suo stato con `/mcp` all'interno di una sessione o `claude mcp list` dal vostro shell, quindi abbinare il sintomo seguente. Il pannello `/mcp` vi consente anche di riconnettervi o autenticarvi senza lasciare la sessione.

<AccordionGroup>
  <Accordion title="/mcp shows No MCP servers configured">
    Claude Code non ha trovato alcun server per la directory corrente. Le cause più comuni:

    * Avete eseguito `claude mcp add` da un progetto diverso. I server con ambito locale sono legati al progetto dove li avete aggiunti: la radice del repository, o la directory esatta se non eravate in un repository git. Riaggiunger il server dal progetto in cui siete ora, o aggiungetelo con `--scope user` in modo che non sia legato a un progetto.
    * Avete modificato un file di configurazione al percorso sbagliato. I file corretti sono `~/.claude.json` e `<project>/.mcp.json`. Claude Code non legge percorsi come `~/.claude/.mcp.json`, `~/.claude/config/mcp.json`, `~/.claude/mcp.json`, o `%APPDATA%\Claude\mcp.json`. Per i server con ambito utente, eseguire `claude mcp add --scope user`, che scrive nella chiave `mcpServers` in `~/.claude.json`; per i server con ambito progetto, modificare `.mcp.json` nella radice del progetto.
  </Accordion>

  <Accordion title="Status shows Failed to connect or Connection error">
    Entrambi gli stati significano che il server non è stato avviato o l'URL non ha risposto. Possono anche apparire per i server HTTP che si aspettano un token piuttosto che l'accesso al browser coperto in [Connettere un server che richiede l'accesso](#connect-a-server-that-requires-sign-in).

    A partire dalla v2.1.191, un server HTTP che restituisce `404 Not Found` mostra `MCP endpoint not found at <url>. Check the URL in your MCP config.` quando selezionate il server in `/mcp`, con l'URL che Claude Code ha provato. Le versioni precedenti mostrano un messaggio generico `Error POSTing to endpoint` senza l'URL. Confrontare l'URL con il percorso dell'endpoint MCP documentato del server, quindi eseguire `claude mcp remove <name>` e riaggiunger con l'URL corretto.

    Per i server HTTP, confermare che l'URL sia raggiungibile dalla vostra macchina:

    ```bash theme={null}
    curl -I https://mcp.sentry.dev/mcp
    ```

    In PowerShell, utilizzare `curl.exe` invece di `curl` in modo che la richiesta vada al binario curl reale piuttosto che all'alias `Invoke-WebRequest`.

    La risposta vi dice quale tipo di problema avete:

    * Un `404` o `405`: il server è attivo. Molti endpoint MCP rispondono solo alle richieste POST, quindi questo comunque conferma che l'URL è raggiungibile dalla vostra macchina.
    * Un `401` o `403`: il server è attivo e dovete autenticarvi. Utilizzare l'accesso al browser in [Connettere un server che richiede l'accesso](#connect-a-server-that-requires-sign-in), o per i server che prendono un token invece, come quello di GitHub, passarlo con `--header "Authorization: Bearer <token>"` sul comando `claude mcp add`.
    * Nessuna risposta: controllare l'URL e la vostra rete.

    Per i server stdio, eseguire il comando configurato direttamente nel vostro terminale per vedere l'errore sottostante. Per il server Playwright da questa guida, eseguire:

    ```bash theme={null}
    npx -y @playwright/mcp@latest
    ```

    Quello che succede dopo vi dice dove è il problema:

    * Il comando si avvia e attende l'input: il server stesso funziona. Eseguire `claude mcp get <name>` e confermare che il comando mostrato lì corrisponda a quello che avete appena eseguito. Se il comando mostrato differisce da quello che avete digitato, probabilmente avete omesso il separatore `--` prima del comando del server. Rimuovere il server e riaggiungerlo con `--` al suo posto. Se avete scritto `.mcp.json` a mano, controllare la sua sintassi e posizione.
    * Il comando genera un errore: il messaggio nomina cosa manca, come Node.js o un browser.
  </Accordion>

  <Accordion title="Connection timed out at startup">
    Il server ha impiegato più del timeout di avvio predefinito di 30 secondi. La prima esecuzione di un server stdio può essere lenta mentre `npx` scarica il pacchetto. Aumentare il limite con la variabile di ambiente [`MCP_TIMEOUT`](/docs/it/env-vars), in millisecondi:

    ```bash theme={null}
    MCP_TIMEOUT=60000 claude
    ```

    In PowerShell, impostare la variabile prima del comando sulla stessa riga:

    ```powershell theme={null}
    $env:MCP_TIMEOUT = "60000"; claude
    ```
  </Accordion>

  <Accordion title="Server already exists">
    Avete già aggiunto un server con quel nome allo stesso ambito. O rimuovete l'entry esistente per primo o scegliete un nome diverso:

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    Se il nome esiste in più di un ambito, `remove` segnala `exists in multiple scopes`. Passare `--scope` per scegliere quale copia eliminare, ad esempio `claude mcp remove claude-code-docs --scope local`.
  </Accordion>

  <Accordion title="Server connects but no tools appear">
    Eseguire `/mcp` all'interno di una sessione e selezionare il server per vedere il suo elenco di strumenti. Se l'elenco è vuoto, il server è stato avviato ma non ha registrato alcuno strumento, il che di solito significa che manca una variabile di ambiente richiesta come una chiave API.

    Passare la variabile con `--env KEY=value` su `claude mcp add`, o nel campo `env` dell'entry `.mcp.json` del server. La documentazione del server elenca le variabili di cui ha bisogno.
  </Accordion>

  <Accordion title="Changes to .mcp.json don't take effect">
    Claude Code legge `.mcp.json` all'avvio della sessione. Uscire e riavviare la sessione dopo aver modificato il file.

    Se i vostri server ancora non appaiono, eseguire `/mcp` e cercare un avviso di analisi. Claude Code salta le entry malformate e mostra il campo offensivo lì.

    Se in precedenza avete rifiutato il server quando richiesto, reimpostare le approvazioni del progetto:

    ```bash theme={null}
    claude mcp reset-project-choices
    ```
  </Accordion>

  <Accordion title="OAuth sign-in fails or browser doesn't open">
    Eseguire `/mcp`, selezionare il server e scegliere `Authenticate` di nuovo. Se il browser non si apre automaticamente, copiare l'URL mostrato nel terminale e aprirlo manualmente. Vedere [Autenticarsi con server MCP remoti](/docs/it/mcp#authenticate-with-remote-mcp-servers) per porte di callback fisse e credenziali preconfigurate.
  </Accordion>
</AccordionGroup>

<h2 id="next-steps">
  Passaggi successivi
</h2>

Con un server connesso, esplorare il resto di ciò che MCP abilita:

* [Trovare altri server MCP](/docs/it/mcp#find-and-build-mcp-servers) nella Directory Anthropic
* [Condividere server con il vostro team](/docs/it/mcp#mcp-installation-scopes) utilizzando gli ambiti di installazione
* [Gestire l'accesso MCP per un'organizzazione](/docs/it/managed-mcp) con impostazioni gestite e controlli delle politiche
* [Fare riferimento alle risorse MCP](/docs/it/mcp#use-mcp-resources) nei prompt con menzioni @
* [Eseguire i prompt MCP come comandi](/docs/it/mcp#use-mcp-prompts-as-commands) dal menu `/`
* [Costruire il vostro server](https://modelcontextprotocol.io/quickstart/server) con l'SDK MCP
