> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Creare un marketplace

> Crea un marketplace di plugin da un file marketplace.json e testalo localmente prima di ospitarlo.

Un marketplace di plugin è una directory o un repository con un file `.claude-plugin/marketplace.json` che elenca i tuoi plugin e da dove recuperare ognuno di essi. Spingi la directory su un host git, e chiunque abbia accesso la registra in Claude Code con un comando e installa i tuoi plugin da essa.

Crea il tuo marketplace quando desideri che un gruppo che scegli, come il tuo team o la tua organizzazione, installi i tuoi plugin e continui a ricevere i tuoi aggiornamenti da un catalogo che controlli. Il repository può essere privato, può elencare quanti plugin desideri, e un amministratore può [richiederlo su ogni macchina](/docs/it/plugins/org).

<Note>
  Questi casi sono coperti su altre pagine:

  * **Condividere un plugin con poche persone**: invia loro la directory del plugin o un `.zip` di essa. Vedi [Condividere un plugin senza un marketplace](/docs/it/plugins/publish#share-a-plugin-without-a-marketplace).
  * **Offrire un plugin a tutti**: invialo al marketplace della comunità di Anthropic. Vedi [Inviare al marketplace della comunità](/docs/it/plugins/publish#submit-to-the-community-marketplace).
  * **Utilizzare un plugin tu stesso**: caricalo con `--plugin-dir` o salvalo nella tua directory skills. Vedi [Sviluppare senza un marketplace](/docs/it/plugins/create#develop-without-a-marketplace).
</Note>

Inizia con [Creare un marketplace](#create-a-marketplace) per costruirne uno sulla tua macchina e installare un plugin da esso, quindi [aggiungi più voci di plugin](#add-plugin-entries).

<h2 id="create-a-marketplace">
  Creare un marketplace
</h2>

I seguenti passaggi creano un marketplace sulla tua macchina, aggiungono un plugin ad esso, lo registrano in Claude Code e installano il plugin da esso. Questo è l'intero ciclo, ed è lo stesso ciclo che i tuoi utenti attraverseranno una volta che ospiterai il marketplace da qualche parte che possono raggiungere. Esegui ogni comando nella tua shell, dalla directory in cui desideri che `my-marketplace/` sia creato.

Hai bisogno di un plugin da elencare. L'esempio utilizza `my-first-plugin` da [Creare il tuo primo plugin](/docs/it/plugins/create#create-your-first-plugin), un plugin con una skill che esegui come `/my-first-plugin:hello`; costruiscilo prima se non hai ancora un plugin. Per utilizzare un plugin tuo, sostituisci la sua directory e il suo `name` ovunque i passaggi dicano `my-first-plugin`. Per ciò che una directory di plugin può contenere, vedi l'[esploratore della directory del plugin](/docs/it/plugins/components#explore-the-plugin-directory).

<Steps>
  <Step title="Configurare la directory del marketplace">
    Un marketplace è una directory con un file `.claude-plugin/marketplace.json`, più i plugin che elenca. Crea la directory del marketplace e la sua cartella `.claude-plugin/`, quindi copia il tuo plugin sotto `plugins/`:

    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
    cp -r my-first-plugin my-marketplace/plugins/
    ```

    Verifica che il plugin sia valido dove ora si trova, in modo che qualsiasi errore successivo riguardi il marketplace e non il plugin:

    ```bash theme={null}
    claude plugin validate ./my-marketplace/plugins/my-first-plugin
    ```

    L'ultima riga dell'output legge `✔ Validation passed`.
  </Step>

  <Step title="Creare il file del marketplace">
    Salva `marketplace.json` in `my-marketplace/.claude-plugin/marketplace.json`. Il file richiede un `name`, un `owner` e un array `plugins`.

    Ogni oggetto in `plugins` è una voce di plugin e ha bisogno di un `name` e di una `source`. Scrivi la `source` della voce come un percorso dalla radice del marketplace. La radice è `my-marketplace/`, la directory che contiene `.claude-plugin/`.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-marketplace",
      "description": "Plugins for my team",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "my-first-plugin",
          "source": "./plugins/my-first-plugin",
          "description": "A greeting plugin to learn the basics"
        }
      ]
    }
    ```
  </Step>

  <Step title="Convalidare il marketplace">
    Esegui `claude plugin validate` sulla directory del marketplace per controllare la sintassi JSON, i campi obbligatori e ogni voce di plugin nel suo `.claude-plugin/marketplace.json`.

    ```bash theme={null}
    claude plugin validate ./my-marketplace
    ```

    Per il file come scritto nel passaggio 2, l'ultima riga dell'output legge `✔ Validation passed`.
  </Step>

  <Step title="Aggiungere il marketplace e installare il plugin">
    Registra la directory come marketplace.

    ```bash theme={null}
    claude plugin marketplace add ./my-marketplace
    ```

    Il comando stampa `✔ Successfully added marketplace: my-marketplace (declared in user settings)`, il che significa che il marketplace è registrato nel tuo file di impostazioni utente.

    Installa il plugin. L'id di installazione è il `name` della voce, un `@` e il `name` del marketplace.

    ```bash theme={null}
    claude plugin install my-first-plugin@my-marketplace
    ```

    Il comando stampa `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)`.

    All'interno di una sessione, `/plugin marketplace add ./my-marketplace` registra il marketplace allo stesso modo. `/plugin install my-first-plugin@my-marketplace` apre i dettagli del plugin nel pannello `/plugin`, dove lo installi. Per quel flusso, vedi [Installare e gestire i plugin](/docs/it/plugins/install).
  </Step>

  <Step title="Confermare che il plugin è stato caricato">
    Elenca i plugin installati.

    ```bash theme={null}
    claude plugin list
    ```

    L'output elenca `my-first-plugin@my-marketplace` con `Status: ✔ enabled`.

    Per vedere cosa ha caricato il plugin, mostra i suoi dettagli.

    ```bash theme={null}
    claude plugin details my-first-plugin
    ```

    La sezione `Component inventory` legge `Skills (1)  hello`.

    Per eseguire la skill, avvia una sessione e inserisci `/my-first-plugin:hello`. Claude ti saluta. Il comando ha il nome del plugin come prefisso, come fa il nome di ogni skill del plugin.
  </Step>
</Steps>

<h2 id="add-plugin-entries">
  Aggiungere voci di plugin
</h2>

Ogni plugin che distribuisci è un oggetto nell'array `plugins` di `marketplace.json`. Per aggiungere un secondo plugin, aggiungi un secondo oggetto. Questi campi coprono la maggior parte delle voci:

* `name`: l'identificatore che le persone digitano prima di `@` quando installano. Non può contenere spazi.
* `source`: da dove Claude Code recupera il plugin. Scrivi una stringa di percorso relativo per un plugin all'interno della directory del marketplace, come nella [procedura dettagliata](#create-a-marketplace), o un oggetto source per un plugin al di fuori di essa. Vedi [Scegliere una fonte di plugin](#choose-a-plugin-source).
* `description`: la riga che le persone vedono accanto al plugin quando sfogliano il tuo marketplace in `/plugin`.

Per l'elenco completo dei campi, vedi [Voci di plugin](/docs/it/plugins/marketplace-reference#plugin-entries).

Una voce può anche impostare qualsiasi campo [`plugin.json`](/docs/it/plugins/manifest-reference). Per quando i campi `plugin.json` di una voce si applicano a un plugin che ha il suo `plugin.json`, vedi [Voce e plugin.json](/docs/it/plugins/marketplace-reference#entry-and-plugin-json).

<h2 id="rules-for-plugin-entries">
  Regole per le voci di plugin
</h2>

La maggior parte degli errori di installazione da un nuovo marketplace provengono da un percorso relativo scritto dalla directory sbagliata, o da un nome di voce che differisce dal `name` nel `plugin.json` del plugin.

<h3 id="write-relative-paths-from-the-marketplace-root">
  Scrivi percorsi relativi dalla radice del marketplace
</h3>

La radice del marketplace è la directory che contiene `.claude-plugin/`. Nella [procedura dettagliata](#create-a-marketplace), è `my-marketplace/`, quindi la `source` della voce è `"./plugins/my-first-plugin"`. Il percorso non inizia all'interno di `.claude-plugin/`, quindi non usare `..` per lasciarlo.

Un percorso con `..` e un percorso a una directory mancante falliscono in comandi diversi:

* **Un percorso con `..`**: `claude plugin validate` segnala la voce come non valida. Il messaggio inizia con `Path contains "..": ./../plugins/my-first-plugin`.
* **Un percorso a una directory che non esiste**: `claude plugin validate` passa. `claude plugin install` fallisce con `Source path does not exist: <path>`, e `<path>` è la posizione assoluta che Claude Code ha controllato.

<h3 id="keep-the-entry-name-and-the-manifest-name-the-same">
  Mantieni il nome della voce e il nome del manifest uguali
</h3>

Un plugin del marketplace ha un `name` della voce in `marketplace.json` e un `name` nel suo `plugin.json`, chiamato nome del manifest. Ogni nome appare in posti diversi:

* **Nome della voce**: l'id di installazione, `<entry-name>@<marketplace>`. È quello che le persone digitano per installare, quello che `claude plugin list` mostra, e la chiave che Claude Code scrive sotto [`enabledPlugins`](/docs/it/settings-reference#enabledplugins) nel loro file di impostazioni.
* **Nome del manifest**: il prefisso sulle skill del plugin, e il nome che `claude plugin details` accetta.

Quando i due nomi differiscono e qualcuno installa per nome del manifest, Claude Code segnala `Plugin "<manifest-name>" not found in marketplace "<marketplace>"`. Mantieni i due nomi uguali. Per ulteriori informazioni su come Claude Code utilizza i due nomi, vedi [Riferimento al caricamento dei plugin](/docs/it/plugins/loading#find-where-a-plugin-came-from).

<h2 id="choose-a-plugin-source">
  Scegliere una fonte di plugin
</h2>

Ogni voce di plugin in `marketplace.json` ha una `source` che dice a Claude Code da dove recuperare quel plugin. Scegli la fonte in base a dove sono archiviati i file del plugin. La tabella elenca le fonti che la maggior parte dei proprietari di marketplace utilizza.

| Source            | Usalo quando                                                                    | Valore minimo di `source`                                                                 |
| :---------------- | :------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------- |
| Percorso relativo | I file del plugin si trovano all'interno della directory del marketplace stessa | `"./plugins/my-first-plugin"`                                                             |
| `github`          | Il plugin è un repository GitHub a sé stante                                    | `{ "source": "github", "repo": "your-org/my-first-plugin" }`                              |
| `git-subdir`      | Il plugin è una sottodirectory di un altro repository, come un monorepo         | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

In una fonte `git-subdir`, `url` accetta un URL git o una scorciatoia GitHub `owner/repo`.

Un plugin può anche provenire da uno di questi tipi di fonte:

* `url`: un repository git per URL, su qualsiasi host
* `archive`: un file zip scaricato su HTTPS
* `npm`: un pacchetto npm
* `command`: una directory prodotta eseguendo un comando sulla macchina in cui il plugin è installato

Per i campi di ogni tipo di fonte, e per fissare una fonte basata su git a un `ref` o `sha`, vedi [Fonti di plugin](/docs/it/plugins/marketplace-reference#plugin-sources).

<h2 id="validate-and-test">
  Convalidare e testare
</h2>

Mentre aggiungi plugin, esegui `claude plugin validate ./my-marketplace` nella tua shell dopo ogni modifica, e installa dal marketplace sulla tua macchina prima di condividerlo. La convalidazione e l'installazione catturano problemi diversi.

<h3 id="problems-that-validation-reports">
  Problemi che la convalidazione segnala
</h3>

`claude plugin validate` legge solo i file all'interno della directory del marketplace. Segnala:

* Errori di sintassi JSON, come `json: Invalid JSON syntax: <reason>`
* Campi obbligatori mancanti, come `owner: Invalid input`
* Un nome di marketplace con spazi, caratteri non ASCII, o una forma che imita un marketplace ufficiale di Anthropic, come `claude-official`
* Una `source` relativa che contiene `..`
* Campi sconosciuti al livello superiore o in una voce di plugin, come avvisi
* Problemi nel `plugin.json` di ogni plugin con percorso relativo, come `plugins[N] plugin.json → <field>: <message>`

Per ogni messaggio che `validate` può stampare, vedi [Messaggi di convalidazione](/docs/it/plugins/marketplace-reference#validation-messages). Per i suoi flag e codici di uscita, vedi [`plugin validate`](/docs/it/plugins/cli-reference#plugin-validate).

<h3 id="problems-that-surface-when-you-add-or-install">
  Problemi che emergono quando aggiungi o installi
</h3>

I problemi che `claude plugin validate` non segnala appaiono quando aggiungi il marketplace o installi da esso:

* **Quando aggiungi il marketplace**: i [nomi ufficiali del marketplace](/docs/it/plugins/marketplace-reference#reserved-names) esatti, come `claude-plugins-official`, passano la convalidazione. Quando aggiungi un marketplace con uno di questi nomi, Claude Code lo rifiuta con un messaggio che inizia con `The name '<name>' is reserved for official Anthropic marketplaces`.
* **Quando installi un plugin**:
  * Claude Code recupera prima una fonte `github`, `git-subdir` o un'altra fonte remota quando installi il plugin, quindi un `repo` o `path` sbagliato appare allora.
  * Una `source` relativa la cui directory non esiste fallisce anche all'installazione, con `Source path does not exist: <path>`.

<h3 id="test-an-edit-to-a-plugin">
  Testare una modifica a un plugin
</h3>

Nella [procedura dettagliata](#create-a-marketplace), hai aggiunto `my-marketplace` da una directory locale con una `source` con percorso relativo. Con quella configurazione, Claude Code legge i file del plugin direttamente da `my-marketplace/plugins/`. Le tue modifiche hanno effetto all'avvio della sessione successiva o quando esegui `/reload-plugins` in una sessione, senza alcun cambiamento alla `version` del plugin.

Le persone che installano dal tuo marketplace ospitato ottengono una copia nella cache del plugin. Per come ricevono una nuova versione, vedi [Mantenere gli utenti aggiornati](/docs/it/plugins/host-marketplace#keep-users-up-to-date).

<h3 id="remove-the-marketplace-to-start-over">
  Rimuovere il marketplace per ricominciare
</h3>

Per rimuovere tutto e ricominciare, esegui `claude plugin marketplace remove my-marketplace` nella tua shell. Il comando rimuove il marketplace e disinstalla i suoi plugin.

<h2 id="host-your-marketplace">
  Ospitare il tuo marketplace
</h2>

Una volta che puoi installare un plugin dal marketplace sulla tua macchina, come in [Creare un marketplace](#create-a-marketplace), spingi la directory del marketplace su un host git.

I tuoi colleghi quindi eseguono `claude plugin marketplace add <owner>/<repo>` nella loro shell per un repository GitHub, o lo stesso comando con l'URL del repository. Quindi installano un plugin per nome come nella [procedura dettagliata](#create-a-marketplace).

Per l'accesso ai repository privati, gli aggiornamenti, il versionamento e la ridenominazione o la rimozione di voci, vedi [Ospitare e mantenere un marketplace](/docs/it/plugins/host-marketplace).

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Ospitare e mantenere un marketplace](/docs/it/plugins/host-marketplace): scegli un host, mantieni gli utenti aggiornati e rinomina o rimuovi i plugin in modo sicuro
* [Riferimento del marketplace](/docs/it/plugins/marketplace-reference): campi `marketplace.json` e tipi di fonte
* [Gestire i plugin per la tua organizzazione](/docs/it/plugins/org): richiedi il tuo marketplace e i suoi plugin su ogni macchina
* [Suggerire i plugin per rilevanza](/docs/it/plugins/relevance): fai in modo che Claude Code suggerisca un plugin dal tuo marketplace quando una sessione corrisponde
