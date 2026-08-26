> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Scopri e installa plugin precostruiti tramite marketplace

> Trova e installa plugin dai marketplace per estendere Claude Code con nuove skills, agenti e funzionalità.

I plugin estendono Claude Code con skills, agenti, hooks e MCP servers. I marketplace dei plugin sono cataloghi che vi aiutano a scoprire e installare queste estensioni senza doverle costruire da soli.

Cercate di creare e distribuire il vostro marketplace? Consultate [Creare e distribuire un marketplace di plugin](/docs/it/plugin-marketplaces).

<h2 id="how-marketplaces-work">
  Come funzionano i marketplace
</h2>

Un marketplace è un catalogo di plugin che qualcun altro ha creato e condiviso. Utilizzare un marketplace è un processo in due fasi:

<Steps>
  <Step title="Aggiungere il marketplace">
    Questo registra il catalogo con Claude Code in modo che possiate sfogliare ciò che è disponibile. Nessun plugin viene installato ancora.
  </Step>

  <Step title="Installare singoli plugin">
    Sfogliate il catalogo e installate i plugin che desiderate.
  </Step>
</Steps>

Pensatelo come aggiungere un app store: aggiungere lo store vi dà accesso per sfogliare la sua collezione, ma voi scegliete comunque quali app scaricare individualmente.

<h2 id="official-anthropic-marketplace">
  Marketplace ufficiale Anthropic
</h2>

Il marketplace ufficiale Anthropic (`claude-plugins-official`) è automaticamente disponibile quando avviate Claude Code. Eseguite `/plugin` e andate alla scheda **Discover** per sfogliare ciò che è disponibile, oppure visualizzate il catalogo su [claude.com/plugins](https://claude.com/plugins).

Per installare un plugin dal marketplace ufficiale, utilizzate `/plugin install <name>@claude-plugins-official`. Ad esempio, per installare l'integrazione GitHub:

```shell theme={null}
/plugin install github@claude-plugins-official
```

Se Claude Code segnala che il plugin non è trovato in alcun marketplace, il vostro marketplace è mancante o obsoleto. Eseguite `/plugin marketplace update claude-plugins-official` per aggiornarlo, oppure `/plugin marketplace add anthropics/claude-plugins-official` se non lo avete ancora aggiunto. Quindi riprovate l'installazione.

<Note>
  Il marketplace ufficiale è curato da Anthropic e l'inclusione è a discrezione di Anthropic. I moduli di invio in-app aggiungono plugin al [marketplace della comunità](#community-marketplace), non a quello ufficiale. Per distribuire plugin in modo indipendente, [create il vostro marketplace](/docs/it/plugin-marketplaces) e condividetelo con gli utenti.
</Note>

Il marketplace ufficiale include diverse categorie di plugin:

<h3 id="code-intelligence">
  Code intelligence
</h3>

I plugin di code intelligence abilitano lo strumento LSP integrato di Claude Code, dando a Claude la capacità di saltare alle definizioni, trovare riferimenti e vedere errori di tipo immediatamente dopo le modifiche. Questi plugin configurano connessioni [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), la stessa tecnologia che alimenta la code intelligence di VS Code.

Questi plugin richiedono che il binario del language server sia installato sul vostro sistema. Se avete già un language server installato, Claude potrebbe chiedervi di installare il plugin corrispondente quando aprite un progetto.

| Linguaggio | Plugin              | Binario richiesto            |
| :--------- | :------------------ | :--------------------------- |
| C/C++      | `clangd-lsp`        | `clangd`                     |
| C#         | `csharp-lsp`        | `csharp-ls`                  |
| Go         | `gopls-lsp`         | `gopls`                      |
| Java       | `jdtls-lsp`         | `jdtls`                      |
| Kotlin     | `kotlin-lsp`        | `kotlin-language-server`     |
| Lua        | `lua-lsp`           | `lua-language-server`        |
| PHP        | `php-lsp`           | `intelephense`               |
| Python     | `pyright-lsp`       | `pyright-langserver`         |
| Rust       | `rust-analyzer-lsp` | `rust-analyzer`              |
| Swift      | `swift-lsp`         | `sourcekit-lsp`              |
| TypeScript | `typescript-lsp`    | `typescript-language-server` |

Potete anche [creare il vostro plugin LSP](/docs/it/plugins-reference#lsp-servers) per altri linguaggi.

<Note>
  Se vedete `Executable not found in $PATH` nella scheda `/plugin` Errors dopo aver installato un plugin, installate il binario richiesto dalla tabella sopra.
</Note>

<h4 id="what-claude-gains-from-code-intelligence-plugins">
  Cosa Claude guadagna dai plugin di code intelligence
</h4>

Una volta che un plugin di code intelligence è installato e il suo binario del language server è disponibile, Claude guadagna due capacità:

* **Diagnostica automatica**: dopo ogni modifica di file che Claude fa, il language server analizza i cambiamenti e segnala errori e avvisi automaticamente. Claude vede errori di tipo, import mancanti e problemi di sintassi senza dover eseguire un compilatore o linter. Se Claude introduce un errore, lo nota e corregge il problema nello stesso turno. Questo non richiede alcuna configurazione oltre all'installazione del plugin. Potete vedere la diagnostica inline premendo **Ctrl+O** quando appare l'indicatore "diagnostics found".
* **Navigazione del codice**: Claude può utilizzare il language server per saltare alle definizioni, trovare riferimenti, ottenere informazioni sul tipo al passaggio del mouse, elencare simboli, trovare implementazioni e tracciare gerarchie di chiamate. Queste operazioni danno a Claude una navigazione più precisa rispetto alla ricerca basata su grep, anche se la disponibilità può variare a seconda del linguaggio e dell'ambiente.

Se riscontrate problemi, consultate [Risoluzione dei problemi di code intelligence](#code-intelligence-issues).

<h3 id="external-integrations">
  Integrazioni esterne
</h3>

Questi plugin raggruppano [MCP servers](/docs/it/mcp) preconfigurati in modo che possiate connettere Claude a servizi esterni senza configurazione manuale:

* **Controllo del codice sorgente**: `github`, `gitlab`
* **Gestione dei progetti**: `atlassian` (Jira/Confluence), `asana`, `linear`, `notion`
* **Design**: `figma`
* **Infrastruttura**: `vercel`, `firebase`, `supabase`
* **Comunicazione**: `slack`
* **Monitoraggio**: `sentry`

<h3 id="automatic-security-review">
  Revisione automatica della sicurezza
</h3>

Il plugin `security-guidance` esamina ogni modifica che Claude fa per individuare vulnerabilità comuni e istruisce Claude a correggere ciò che trova nella stessa sessione. Consultate [Rilevare problemi di sicurezza mentre Claude scrive il codice](/docs/it/security-guidance) per vedere cosa controlla e come aggiungere regole specifiche del progetto.

<h3 id="development-workflows">
  Flussi di lavoro di sviluppo
</h3>

Plugin che aggiungono skills e agenti per attività di sviluppo comuni:

* **commit-commands**: Flussi di lavoro di commit Git inclusi commit, push e creazione di PR
* **pr-review-toolkit**: Agenti specializzati per la revisione delle pull request
* **agent-sdk-dev**: Strumenti per la costruzione con Claude Agent SDK
* **plugin-dev**: Toolkit per la creazione dei vostri plugin

<h3 id="output-styles">
  Stili di output
</h3>

Personalizzate come Claude risponde:

* **explanatory-output-style**: Approfondimenti educativi sulle scelte di implementazione
* **learning-output-style**: Modalità di apprendimento interattivo per la costruzione di competenze

<h2 id="community-marketplace">
  Marketplace della comunità
</h2>

Il marketplace della comunità su [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) ospita plugin di terze parti che hanno superato la convalida automatica e lo screening di sicurezza di Anthropic. Ogni plugin è fissato a uno specifico commit SHA nel catalogo. A differenza del marketplace ufficiale, lo aggiungete manualmente:

```shell theme={null}
/plugin marketplace add anthropics/claude-plugins-community
```

Quindi installate i plugin da esso utilizzando il nome del marketplace `claude-community`:

```shell theme={null}
/plugin install <plugin-name>@claude-community
```

Per inviare il vostro plugin al marketplace della comunità, consultate [Inviare il vostro plugin al marketplace della comunità](/docs/it/plugins#submit-your-plugin-to-the-community-marketplace) nella guida di creazione dei plugin.

<h2 id="try-it-add-the-demo-marketplace">
  Provate: aggiungere il marketplace demo
</h2>

Anthropic mantiene anche un [marketplace di plugin demo](https://github.com/anthropics/claude-code/tree/main/plugins) (`claude-code-plugins`) con plugin di esempio che mostrano cosa è possibile con il sistema di plugin. A differenza del marketplace ufficiale, dovete aggiungere questo manualmente.

<Steps>
  <Step title="Aggiungere il marketplace">
    Da Claude Code, eseguite il comando `plugin marketplace add` per il marketplace `anthropics/claude-code`:

    ```shell theme={null}
    /plugin marketplace add anthropics/claude-code
    ```

    Questo scarica il catalogo del marketplace e rende i suoi plugin disponibili per voi.
  </Step>

  <Step title="Sfogliare i plugin disponibili">
    Eseguite `/plugin` per aprire il gestore dei plugin. Questo apre un'interfaccia a schede con quattro schede che potete scorrere utilizzando **Tab**, o **Shift+Tab** per andare indietro:

    * **Discover**: sfogliate i plugin disponibili da tutti i vostri marketplace
    * **Installed**: visualizzate e gestite i vostri plugin installati
    * **Marketplaces**: aggiungete, rimuovete o aggiornate i vostri marketplace aggiunti
    * **Errors**: visualizzate eventuali errori di caricamento dei plugin

    Andate alla scheda **Discover** per vedere i plugin dal marketplace che avete appena aggiunto. Quando l'amministratore ha inserito il marketplace nella lista consentita tramite l'impostazione gestita [`pluginSuggestionMarketplaces`](/docs/it/settings#available-settings), i plugin contrassegnati come rilevanti per la vostra directory di lavoro corrente sono fissati in alto con un'etichetta **suggested for this directory**.
  </Step>

  <Step title="Installare un plugin">
    Selezionate un plugin per visualizzare i suoi dettagli. Il riquadro dei dettagli mostra cosa contiene il plugin e quanto costa:

    * Una stima di **Context cost** in modo da poter vedere quanti token il plugin aggiungerà alla vostra [finestra di contesto](/docs/it/features-overview#understand-context-costs) ad ogni turno (Claude Code v2.1.143 e versioni successive)
    * La data di **Last updated** del plugin (v2.1.144 e versioni successive)
    * Una sezione **Will install** che elenca i comandi, gli agenti, le skills, gli hook e i server MCP e LSP del plugin, in modo che possiate rivedere esattamente cosa aggiunge prima di installare (v2.1.145 e versioni successive)

    Scegliete un ambito di installazione:

    * **User scope**: installate per voi stessi in tutti i progetti
    * **Project scope**: installate per tutti i collaboratori su questo repository
    * **Local scope**: installate per voi stessi solo in questo repository

    Ad esempio, selezionate **commit-commands**, un plugin che aggiunge skills di flusso di lavoro git, e installatelo nel vostro ambito utente.

    Potete anche installare direttamente dalla riga di comando:

    ```shell theme={null}
    /plugin install commit-commands@claude-code-plugins
    ```

    Consultate [Ambiti di configurazione](/docs/it/settings#configuration-scopes) per saperne di più sugli ambiti.
  </Step>

  <Step title="Utilizzare il vostro nuovo plugin">
    Dopo l'installazione, eseguite `/reload-plugins` per attivare il plugin. Le skills dei plugin sono nello spazio dei nomi del nome del plugin, quindi **commit-commands** fornisce skills come `/commit-commands:commit`.

    Provate eseguendo una modifica a un file e eseguendo:

    ```shell theme={null}
    /commit-commands:commit
    ```

    Questo mette in stage le vostre modifiche, genera un messaggio di commit e crea il commit.

    Ogni plugin funziona diversamente. Controllate i dettagli del plugin nella scheda **Discover** per vedere i comandi e le skills che fornisce, oppure visitate la sua homepage per indicazioni sull'utilizzo.
  </Step>
</Steps>

Il resto di questa guida copre tutti i modi in cui potete aggiungere marketplace, installare plugin e gestire la vostra configurazione.

<h2 id="add-marketplaces">
  Aggiungere marketplace
</h2>

Utilizzate il comando `/plugin marketplace add` per aggiungere marketplace da diverse fonti.

<Tip>
  **Scorciatoie**: Potete utilizzare `/plugin market` invece di `/plugin marketplace` e `rm` invece di `remove`.
</Tip>

* **Repository GitHub**: formato `owner/repo`, ad esempio `anthropics/claude-code`
* **URL Git**: qualsiasi URL di repository git, inclusi GitLab, Bitbucket e server self-hosted
* **Percorsi locali**: directory o percorsi diretti ai file `marketplace.json`
* **URL remoti**: URL diretti ai file `marketplace.json` ospitati

<h3 id="add-from-github">
  Aggiungere da GitHub
</h3>

Aggiungete un repository GitHub che contiene un file `.claude-plugin/marketplace.json` utilizzando il formato `owner/repo`, dove `owner` è il nome utente GitHub o l'organizzazione e `repo` è il nome del repository.

Ad esempio, `anthropics/claude-code` si riferisce al repository `claude-code` di proprietà di `anthropics`:

```shell theme={null}
/plugin marketplace add anthropics/claude-code
```

<h3 id="add-from-other-git-hosts">
  Aggiungere da altri host Git
</h3>

Aggiungete qualsiasi repository git fornendo l'URL completo. Questo funziona con qualsiasi host Git, inclusi GitLab, Bitbucket e server self-hosted. Includete il suffisso `.git` in modo che Claude Code cloni il repository piuttosto che trattare l'URL come un collegamento diretto a un file `marketplace.json` ospitato.

Includete il prefisso `https://` anche. Claude Code v2.1.196 e versioni successive rifiutano un host digitato senza di esso, come `gitlab.com/company/plugins.git`, come una scorciatoia GitHub `owner/repo` non valida, e l'errore vi dice di aggiungere il prefisso. Le versioni precedenti lo leggono male come un percorso di repository GitHub e falliscono al momento del clone.

Utilizzando HTTPS:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

Utilizzando SSH:

```shell theme={null}
/plugin marketplace add git@gitlab.com:company/plugins.git
```

Per aggiungere un branch o tag specifico, aggiungete `#` seguito dal ref:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git#v1.0.0
```

<h3 id="add-from-local-paths">
  Aggiungere da percorsi locali
</h3>

Aggiungete una directory locale che contiene un file `.claude-plugin/marketplace.json`:

```shell theme={null}
/plugin marketplace add ./my-marketplace
```

Potete anche aggiungere un percorso diretto a un file `marketplace.json`:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace.json
```

<h3 id="add-from-remote-urls">
  Aggiungere da URL remoti
</h3>

Aggiungete un file `marketplace.json` remoto tramite URL:

```shell theme={null}
/plugin marketplace add https://example.com/marketplace.json
```

<Note>
  I marketplace basati su URL hanno alcune limitazioni rispetto ai marketplace basati su Git. Se riscontrate errori "path not found" durante l'installazione di plugin, consultate [Risoluzione dei problemi](/docs/it/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces).
</Note>

<h2 id="install-plugins">
  Installare plugin
</h2>

Una volta aggiunti i marketplace, potete installare plugin direttamente:

```shell theme={null}
/plugin install plugin-name@marketplace-name
```

Il comando apre i dettagli di quel plugin, dove scegliete un [ambito di installazione](/docs/it/settings#configuration-scopes). Vedrete le stesse scelte quando eseguite `/plugin`, andate alla scheda **Discover** e premete **Enter** su un plugin:

* **User scope** (predefinito): installate per voi stessi in tutti i progetti
* **Project scope**: installate per tutti i collaboratori su questo repository, che aggiunge il plugin a `.claude/settings.json`
* **Local scope**: installate per voi stessi solo in questo repository, non condiviso con i collaboratori

Per installare senza un passaggio interattivo, utilizzate il comando shell [`claude plugin install`](/docs/it/plugins-reference#plugin-install), che installa nell'ambito utente a meno che non passiate `--scope`.

Potete anche vedere plugin con ambito **managed**. Questi sono installati dagli amministratori tramite [impostazioni gestite](/docs/it/settings#settings-files) e non possono essere modificati.

<Warning>
  Assicuratevi di fidarvi di un plugin prima di installarlo. Anthropic non controlla quali MCP server, file o altro software sono inclusi nei plugin e non può verificare che funzionino come previsto. Controllate la homepage di ogni plugin per ulteriori informazioni.
</Warning>

<h2 id="manage-installed-plugins">
  Gestire i plugin installati
</h2>

Eseguite `/plugin` e andate alla scheda **Installed** per visualizzare, abilitare, disabilitare o disinstallare i vostri plugin. L'elenco è raggruppato per ambito e ordinato in modo che vediate prima i problemi: i plugin con errori di caricamento o dipendenze non risolte appaiono in alto, seguiti dai vostri preferiti, con i plugin disabilitati ripiegati dietro un'intestazione compressa in fondo.

Dall'elenco potete:

* premere `f` per contrassegnare come preferito o rimuovere il contrassegno dal plugin selezionato
* digitare per filtrare per nome o descrizione del plugin
* premere Enter per aprire la vista dettagli di un plugin e abilitare, disabilitare o disinstallarlo

La disinstallazione di un plugin che il `.claude/settings.json` di un progetto abilita chiede quale ambito intendete: disabilitarlo solo per voi, il che scrive un override nel vostro `.claude/settings.local.json` e lascia il plugin installato per il progetto, oppure disinstallarlo per tutti, il che lo rimuove dal `.claude/settings.json` condiviso. Richiede Claude Code v2.1.203 o versioni successive. Prima di v2.1.203, la finestra di dialogo offriva solo la disabilitazione locale.

La vista dettagli mostra i componenti che il plugin contribuisce: comandi, skills, agenti, hooks, server MCP e server LSP. Lo stesso inventario è disponibile dalla riga di comando con `claude plugin details`.

La scheda **Installed** raccoglie anche i plugin del marketplace che avete installato voi stessi ma non avete utilizzato negli ultimi due settimane, in un arco di almeno 10 sessioni, sotto un'intestazione **Not used recently**. La vista dettagli mostra una riga **Last used** per ogni plugin. Utilizzate questi elementi per trovare i plugin che aggiungono ancora costi di avvio e contesto anche se non li utilizzate più, quindi disabilitateli o disinstallateli. Richiede Claude Code v2.1.187 o versioni successive.

Due tipi di plugin non vengono mai elencati come inutilizzati:

* plugin che la vostra organizzazione gestisce o che caricate con `--plugin-dir`
* plugin che contribuiscono un tema, uno stile di output, un monitor o un workflow, poiché forniscono valore senza un'invocazione da tracciare

L'intestazione **Not used recently** e la riga **Last used** sono entrambe nascoste quando la vostra organizzazione limita i marketplace con [`strictKnownMarketplaces`](/docs/it/settings#strictknownmarketplaces).

Un [language server](/docs/it/plugins#add-lsp-servers-to-your-plugin) di un plugin conta come utilizzato quando fornisce diagnostica o risponde a una richiesta di navigazione del codice, quindi un plugin LSP il cui server è attivo nelle vostre sessioni non viene elencato come inutilizzato. Prima di v2.1.203, l'attività del language server non poteva essere conteggiata come utilizzo, quindi i plugin che contribuiscono un server LSP erano esenti dal gruppo interamente, allo stesso modo in cui i plugin di tema e stile di output lo sono ancora.

La prima sessione su una versione che conta l'attività del language server reimposta anche il record di utilizzo di ogni plugin LSP che non aveva ancora registrato alcun utilizzo, quindi Claude Code non giudica un plugin che avete installato in precedenza come inutilizzato in base ai dati registrati prima che l'attività del suo server fosse tracciata. Prima di v2.1.206, quella prima sessione poteva elencare un plugin LSP attivamente utilizzato sotto **Not used recently** e suggerire di rivederlo.

Quando installate un plugin che dichiara dipendenze, l'output dell'installazione elenca quali dipendenze sono state installate automaticamente insieme ad esso.

Potete anche gestire i plugin con comandi diretti.

Elencate i plugin installati senza aprire il menu:

```shell theme={null}
/plugin list
```

Passate `--enabled` o `--disabled` per mostrare solo i plugin in quello stato.

Disabilitate un plugin senza disinstallarlo:

```shell theme={null}
/plugin disable plugin-name@marketplace-name
```

Riabilitate un plugin disabilitato:

```shell theme={null}
/plugin enable plugin-name@marketplace-name
```

In questi identificatori, `plugin-name` è il `name` del plugin nella [voce del marketplace](/docs/it/plugin-marketplaces#plugin-entries), che può differire dal `name` nel `plugin.json` del plugin stesso.

A partire da Claude Code v2.1.195, **Enable** e **Disable** nell'interfaccia `/plugin` funzionano per i plugin i cui due nomi differiscono, e `/plugin enable` e `/plugin disable` accettano entrambi i nomi. Quando disabilitate un tale plugin in una versione precedente, Claude Code segnala `already disabled` e lo lascia abilitato.

Rimuovete completamente un plugin:

```shell theme={null}
/plugin uninstall plugin-name@marketplace-name
```

L'opzione `--scope` vi consente di indirizzare un ambito specifico con comandi CLI:

```shell theme={null}
claude plugin install formatter@your-org --scope project
claude plugin uninstall formatter@your-org --scope project
```

<h3 id="apply-plugin-changes-without-restarting">
  Applicare le modifiche dei plugin senza riavviare
</h3>

Quando installate, abilitate o disabilitate plugin durante una sessione, eseguite `/reload-plugins` per raccogliere tutte le modifiche senza riavviare:

```shell theme={null}
/reload-plugins
```

Claude Code ricarica tutti i plugin attivi e mostra i conteggi per i plugin, le skills, gli agenti, gli hooks, i server MCP dei plugin e i server LSP dei plugin.

Il ricaricamento ha un costo in token sulla richiesta successiva: i componenti appena caricati si annunciano nel contenuto aggiunto alla conversazione, mentre la cronologia esistente continua a leggere dalla cache del prompt. Un plugin che fornisce server MCP costa di più quando i suoi strumenti non sono differiti da [ricerca degli strumenti MCP](/docs/it/mcp#scale-with-mcp-tool-search): il cambiamento invalida la cache e la richiesta successiva rilegge l'intera conversazione. In quel caso `/reload-plugins` mostra un avviso e non applica il ricaricamento; passate `--force` per applicarlo comunque. Consultate [abilitazione o disabilitazione di un plugin](/docs/it/prompt-caching#enabling-or-disabling-a-plugin) per i dettagli.

<h2 id="manage-marketplaces">
  Gestire i marketplace
</h2>

Potete gestire i marketplace tramite l'interfaccia interattiva `/plugin` o con comandi CLI.

<h3 id="use-the-interactive-interface">
  Utilizzare l'interfaccia interattiva
</h3>

Eseguite `/plugin` e andate alla scheda **Marketplaces** per:

* Visualizzare tutti i vostri marketplace aggiunti con le loro fonti e stato
* Aggiungere nuovi marketplace
* Aggiornare gli elenchi dei marketplace per recuperare i plugin più recenti
* Rimuovere i marketplace di cui non avete più bisogno

<h3 id="use-cli-commands">
  Utilizzare comandi CLI
</h3>

Potete anche gestire i marketplace con comandi diretti.

Elencate tutti i marketplace configurati:

```shell theme={null}
/plugin marketplace list
```

Aggiornate gli elenchi dei plugin da un marketplace:

```shell theme={null}
/plugin marketplace update marketplace-name
```

Rimuovete un marketplace:

```shell theme={null}
/plugin marketplace remove marketplace-name
```

<Warning>
  La rimozione di un marketplace disinstallerà tutti i plugin che avete installato da esso.
</Warning>

<h3 id="configure-auto-updates">
  Configurare gli aggiornamenti automatici
</h3>

Claude Code può aggiornare automaticamente i marketplace e i loro plugin installati in background dopo l'avvio. Quando l'aggiornamento automatico è abilitato per un marketplace, Claude Code aggiorna i dati del marketplace e aggiorna i plugin installati alle loro versioni più recenti su disco.

Claude Code verifica gli aggiornamenti del marketplace e dei plugin dopo l'avvio della sessione, con un ritardo casuale fino a dieci minuti, quindi la sessione in esecuzione continua a utilizzare le versioni caricate all'avvio. Se sono stati aggiornati plugin, vedrete una notifica che vi chiede di eseguire `/reload-plugins`, oppure le nuove versioni si caricano al prossimo avvio.

Attivate/disattivate l'aggiornamento automatico per singoli marketplace tramite l'interfaccia utente:

1. Eseguite `/plugin` per aprire il gestore dei plugin
2. Selezionate **Marketplaces**
3. Scegliete un marketplace dall'elenco
4. Selezionate **Enable auto-update** o **Disable auto-update**

I marketplace ufficiali Anthropic hanno l'aggiornamento automatico abilitato per impostazione predefinita. I marketplace di terze parti e di sviluppo locale hanno l'aggiornamento automatico disabilitato per impostazione predefinita.

Gli amministratori possono anche impostare `"autoUpdate": true` su ogni voce [`extraKnownMarketplaces`](/docs/it/settings#extraknownmarketplaces) nelle impostazioni gestite per abilitare l'aggiornamento automatico per un marketplace dell'organizzazione senza richiedere a ogni utente di attivarlo.

Per disabilitare completamente tutti gli aggiornamenti automatici sia per Claude Code che per tutti i plugin, impostate la variabile di ambiente `DISABLE_AUTOUPDATER`. Consultate [Aggiornamenti automatici](/docs/it/setup#auto-updates) per i dettagli.

Per mantenere gli aggiornamenti automatici dei plugin abilitati mentre disabilitate gli aggiornamenti di Claude Code, impostate `FORCE_AUTOUPDATE_PLUGINS=1` insieme a `DISABLE_AUTOUPDATER`:

```bash theme={null}
export DISABLE_AUTOUPDATER=1
export FORCE_AUTOUPDATE_PLUGINS=1
```

Questo è utile quando volete gestire gli aggiornamenti di Claude Code manualmente ma ricevere comunque aggiornamenti automatici dei plugin.

<h2 id="configure-team-marketplaces">
  Configurare i marketplace del team
</h2>

Gli amministratori del team possono configurare l'installazione automatica del marketplace per i progetti aggiungendo la configurazione del marketplace a `.claude/settings.json`. Quando i membri del team si fidano della cartella del repository, Claude Code li invita a installare questi marketplace e plugin.

A partire da Claude Code v2.1.195, questo passaggio di installazione si applica su ogni percorso che carica i plugin. Un plugin che solo il file `.claude/settings.json` del progetto abilita, e che proviene da una fonte esterna come un repository GitHub o un pacchetto npm, non si carica fino a quando il membro del team non lo installa. Fino ad allora, Claude Code segnala il plugin come non installato e mostra il comando `claude plugin install` da eseguire.

Aggiungete `extraKnownMarketplaces` al file `.claude/settings.json` del vostro progetto:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "my-team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Per le opzioni di configurazione complete incluse `extraKnownMarketplaces` e `enabledPlugins`, consultate [Impostazioni dei plugin](/docs/it/settings#plugin-settings).

<h2 id="security">
  Sicurezza
</h2>

I plugin e i marketplace sono componenti altamente affidabili che possono eseguire codice arbitrario sulla vostra macchina con i vostri privilegi utente. Installate solo plugin e aggiungete marketplace da fonti di cui vi fidate. Le organizzazioni possono limitare quali marketplace gli utenti sono autorizzati ad aggiungere utilizzando [restrizioni gestite dei marketplace](/docs/it/plugin-marketplaces#managed-marketplace-restrictions).

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="/plugin-command-not-recognized">
  Comando /plugin non riconosciuto
</h3>

Se vedete "unknown command" o il comando `/plugin` non appare:

1. **Controllate la vostra versione**: Eseguite `claude --version` per vedere cosa è installato.
2. **Aggiornate Claude Code**:
   * **Homebrew**: `brew upgrade claude-code`, oppure `brew upgrade claude-code@latest` se avete installato quel cask
   * **npm**: `npm install -g @anthropic-ai/claude-code@latest`
   * **Programma di installazione nativo**: Rieseguite il comando di installazione da [Setup](/docs/it/setup)
3. **Riavviate Claude Code**: dopo l'aggiornamento, riavviate il vostro terminale ed eseguite `claude` di nuovo.

<h3 id="common-issues">
  Problemi comuni
</h3>

* **Marketplace non caricato**: verificate che l'URL sia accessibile e che `.claude-plugin/marketplace.json` esista nel percorso
* **Errori di installazione dei plugin**: controllate che gli URL di origine dei plugin siano accessibili e che i repository siano pubblici, oppure che abbiate accesso a essi
* **File non trovati dopo l'installazione**: i plugin vengono copiati in una cache, quindi i percorsi che fanno riferimento a file al di fuori della directory del plugin non funzioneranno
* **Le skill dei plugin non appaiono**: cancellate la cache con `rm -rf ~/.claude/plugins/cache`, riavviate Claude Code e reinstallate il plugin.

Per la risoluzione dettagliata dei problemi con soluzioni, consultate [Risoluzione dei problemi](/docs/it/plugin-marketplaces#troubleshooting) nella guida del marketplace. Per gli strumenti di debug, consultate [Strumenti di debug e sviluppo](/docs/it/plugins-reference#debugging-and-development-tools).

<h3 id="code-intelligence-issues">
  Problemi di code intelligence
</h3>

* **Language server non avviato**: verificate che il binario sia installato e disponibile nel vostro `$PATH`. Controllate la scheda `/plugin` Errors per i dettagli.
* **Utilizzo elevato della memoria**: i language server come `rust-analyzer` e `pyright` possono consumare memoria significativa su progetti di grandi dimensioni. Se riscontrate problemi di memoria, disabilitate il plugin con `/plugin disable <plugin-name>` e affidatevi invece agli strumenti di ricerca integrati di Claude.
* **Diagnostica falsa positiva nei monorepo**: i language server possono segnalare errori di import non risolti per i pacchetti interni se l'area di lavoro non è configurata correttamente. Questi non influiscono sulla capacità di Claude di modificare il codice.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* **Costruite i vostri plugin**: Consultate [Plugin](/docs/it/plugins) per creare skills, agenti e hook
* **Create un marketplace**: Consultate [Creare un marketplace di plugin](/docs/it/plugin-marketplaces) per distribuire plugin al vostro team o comunità
* **Riferimento tecnico**: Consultate [Riferimento dei plugin](/docs/it/plugins-reference) per le specifiche complete
