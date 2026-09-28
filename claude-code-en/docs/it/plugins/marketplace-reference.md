> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Marketplace reference

> Riferimento completo per i campi marketplace.json, le voci dei plugin e gli oggetti sorgente del plugin e del marketplace, con indicazione di dove ciascuno è valido.

`marketplace.json` è il file che definisce un marketplace di plugin. Contiene il nome del marketplace, il suo proprietario e una voce per ogni plugin. La sorgente del plugin di ogni voce indica da dove Claude Code recupera quel plugin.

Una sorgente marketplace è un oggetto separato che indica da dove Claude Code recupera il file marketplace stesso. Si scrive nelle impostazioni, oppure Claude Code ne crea una quando si esegue `claude plugin marketplace add`.

Questo riferimento è per i manutentori di marketplace che hanno bisogno di un nome di campo o di un valore esatto, e per gli amministratori che hanno bisogno di sapere quali valori `source` sono validi in [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces), [`strictKnownMarketplaces`](/docs/it/settings-reference#strictknownmarketplaces) e [`blockedMarketplaces`](/docs/it/plugins/org#restrict-what-users-can-install).

<Note>
  Questi casi sono trattati in altre pagine:

  * **Creazione o hosting di un marketplace**: vedere [Create a marketplace](/docs/it/plugins/create-marketplace) e [Host and maintain a marketplace](/docs/it/plugins/host-marketplace)
  * **Ricette per allowlist e blocklist**: vedere [Manage plugins for your organization](/docs/it/plugins/org)
</Note>

Trovare la sezione per quello che si sta scrivendo o leggendo:

* **Il file marketplace**: [Top-level fields](#top-level-fields) e [Plugin entries](#plugin-entries)
* **La `source` di una voce**: [Plugin sources](#plugin-sources)
* **Un oggetto `source` nelle impostazioni**: [Marketplace sources](#marketplace-sources)
* **Output da [`claude plugin validate <path>`](/docs/it/plugins/cli-reference)**: [Validation messages](#validation-messages), che mappa ogni messaggio al campo che nomina

<h2 id="marketplace-file">
  Marketplace file
</h2>

Salvare il file marketplace in `.claude-plugin/marketplace.json` nella directory del marketplace. Se si mantiene il file altrove nel repository, gli utenti devono dichiarare il marketplace in [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces) con `path` impostato sulla sua sorgente, perché `claude plugin marketplace add` non ha un'opzione per questo.

La directory che contiene `.claude-plugin/` è chiamata marketplace root, e ogni sorgente di plugin relativa si risolve da essa, non da `.claude-plugin/`.

Ogni utente registra un marketplace per `name`, quindi un utente non può avere due marketplace con lo stesso nome registrati contemporaneamente.

Claude Code ignora una chiave di primo livello sconosciuta o una chiave di voce di plugin piuttosto che rifiutarla, quindi un errore di battitura si carica silenziosamente. `claude plugin validate` segnala ogni chiave sconosciuta come un avviso.

<h3 id="reserved-names">
  Reserved names
</h3>

Non è possibile assegnare al marketplace nessuno dei seguenti nomi:

* **Nomi ufficiali del marketplace**: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins` e `claude-tag-plugins`. Riservati a meno che il marketplace non provenga da una [sorgente marketplace](#marketplace-sources) `github` o `git` sotto `github.com/anthropics/`.
* **Nomi del marketplace della comunità**: `claude-community`, `claude-plugins-community` e `healthcare`. Riservati secondo la stessa regola dei nomi ufficiali.
* **Nomi della directory dei plugin**: `anthropic-plugin-directory` e `claude-plugin-directory`. Riservati secondo la stessa regola dei nomi ufficiali.
* **Nomi che impersonano un marketplace ufficiale**: nomi come `official-claude-plugins` o `claude-plugins-v2`, e qualsiasi nome contenente un carattere non ASCII. L'errore è `Marketplace name impersonates an official Anthropic/Claude marketplace`. Un carattere di controllo o di formattazione bidirezionale in un nome segnala anche `Marketplace name cannot contain control or bidirectional-formatting characters`.
* <span id="reserved-name-spellings" />**Un'altra ortografia di un nome riservato**: un nome che differisce da un nome riservato solo per un punto finale, o per un simbolo diverso da un trattino al posto di un trattino, quindi `claude.code.plugins` conta come `claude-code-plugins`. `claude plugin validate` accetta tale nome; l'aggiunta del marketplace non riesce con [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/it/errors#marketplace-name-is-another-spelling-of-a-reserved-name), e un marketplace già registrato sotto uno smette di caricarsi. Questo controllo richiede Claude Code v2.1.280 o successivo.
* **Nomi che Claude Code utilizza per i plugin che non provengono da un marketplace**: `inline` per i plugin caricati con [`--plugin-dir`](/docs/it/cli-reference), `builtin` per i plugin integrati, `skills-dir` per i plugin caricati automaticamente da [`.claude/skills/`](/docs/it/skills) e `synced` per i plugin sincronizzati dal proprio account claude.ai. Anche `claude-plugin-test` è riservato. `skills-dir` appare anche come `{"source": "skills-dir"}` in `strictKnownMarketplaces` e `blockedMarketplaces`, descritto in [Source values valid only in policy lists](#source-values-valid-only-in-policy-lists).
* **`npm`, `pip`, `uv`, `cargo`, `github` e `gh`**: riservati in qualsiasi maiuscola. Questo controllo richiede Claude Code v2.1.275 o successivo.
* **Nomi che iniziano con `claudeai-`**: riservati per i marketplace ospitati su claude.ai. `claude plugin marketplace add` rifiuta qualsiasi altro marketplace che ne utilizzi uno con `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai`.

<h2 id="top-level-fields">
  Top-level fields
</h2>

La tabella elenca ogni chiave che Claude Code legge da `marketplace.json`. `name`, `owner` e `plugins` sono obbligatori.

| Field                                      | Type             | Description                                                                                                                                                                                                                                                                                      |
| :----------------------------------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                     | string           | Identificatore del marketplace. Nessuno spazio, caratteri di controllo o caratteri di formattazione bidirezionale, nessun `/` o `\`, nessun `..` e non `.`. Vedere [Reserved names](#reserved-names). Gli utenti lo digitano dopo `@` quando installano un plugin                                |
| `owner`                                    | object           | Informazioni del manutentore. `name` è obbligatorio; `email` e `url` sono facoltativi                                                                                                                                                                                                            |
| `plugins`                                  | array            | [Plugin entries](#plugin-entries). Ogni voce è convalidata da sola, quindi una voce non valida non fa fallire il marketplace                                                                                                                                                                     |
| `$schema`                                  | string           | URL dello schema JSON per l'autocompletamento dell'editor. Ignorato al momento del caricamento                                                                                                                                                                                                   |
| `description`                              | string           | Descrizione del marketplace mostrata agli utenti. `claude plugin validate` avverte quando manca                                                                                                                                                                                                  |
| `version`                                  | string           | Versione del manifesto del marketplace                                                                                                                                                                                                                                                           |
| `metadata.description`, `metadata.version` | string           | Posizione alternativa per `description` e `version`                                                                                                                                                                                                                                              |
| `metadata.pluginRoot`                      | string           | Directory in cui i nomi di sorgente di plugin bare si risolvono. Vedere [Relative path plugin source](#relative-path-plugin-source). Richiede Claude Code v2.1.239 o successivo                                                                                                                  |
| `forceRemoveDeletedPlugins`                | boolean          | Quando `true`, un plugin che si rimuove da `plugins` viene disinstallato sulle macchine degli utenti. Vedere [Host and maintain a marketplace](/docs/it/plugins/host-marketplace)                                                                                                                     |
| `allowCrossMarketplaceDependenciesOn`      | array of strings | Nomi di marketplace i cui plugin possono essere installati come dipendenze dei plugin di questo marketplace. Quando si installa un plugin, si applica solo l'elenco nel marketplace del plugin stesso, per l'intera catena di dipendenze. Vedere [Plugin dependencies](/docs/it/plugins/dependencies) |
| `renames`                                  | object           | Mappa da un precedente `name` del plugin al suo nome attuale, o a `null` per un plugin rimosso. Richiede Claude Code v2.1.193 o successivo. Vedere [Host and maintain a marketplace](/docs/it/plugins/host-marketplace)                                                                               |

<h2 id="plugin-entries">
  Plugin entries
</h2>

Ogni oggetto nell'array `plugins` di primo livello di `marketplace.json` nomina un plugin e dice dove recuperarlo. `name` e `source` sono obbligatori.

Una voce accetta anche ogni campo [`plugin.json`](/docs/it/plugins/manifest-reference), come `description`, `version`, `author`, `commands` e `hooks`. Per quando questi campi si applicano, vedere [How an entry combines with plugin.json](#entry-and-plugin-json).

La tabella elenca i campi propri della voce e i campi del manifesto il cui significato cambia in una voce.

| Field            | Type             | Description                                                                                                                                                                                                                                                                                                                                       |
| :--------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`           | string           | Identificatore del plugin, senza spazi, caratteri di controllo o caratteri di formattazione bidirezionale. Gli utenti lo digitano prima di `@` quando installano, anche quando il `plugin.json` del plugin stesso imposta un `name` diverso                                                                                                       |
| `source`         | string or object | Dove recuperare il plugin. Vedere [Plugin sources](#plugin-sources)                                                                                                                                                                                                                                                                               |
| `description`    | string           | Mostrato negli elenchi e nei dettagli di [`/plugin`](/docs/it/plugins/install)                                                                                                                                                                                                                                                                         |
| `version`        | string           | Stringa di versione per il plugin. Quando `plugin.json` imposta anche `version`, `plugin.json` ha la precedenza e `claude plugin validate` avverte. Vedere [Plugin loading reference](/docs/it/plugins/loading)                                                                                                                                        |
| `category`       | string           | Categoria libera per organizzare il catalogo                                                                                                                                                                                                                                                                                                      |
| `tags`           | array of strings | Tag liberi per la ricerca                                                                                                                                                                                                                                                                                                                         |
| `strict`         | boolean          | Predefinito `true`. Se `plugin.json` è la fonte definitiva per i componenti del plugin. Vedere [Strict mode](#strict-mode)                                                                                                                                                                                                                        |
| `relevance`      | object           | Segnali che dicono a Claude Code quando suggerire il plugin. Vedere [Recommend plugins for your org](/docs/it/plugins/relevance)                                                                                                                                                                                                                       |
| `dependencies`   | array            | Plugin che devono essere abilitati affinché questo funzioni. Ogni elemento è `"name"`, `"name@marketplace"` o un oggetto. Vedere [Plugin dependencies](/docs/it/plugins/dependencies)                                                                                                                                                                  |
| `defaultEnabled` | boolean          | Predefinito `true`. Se il plugin inizia abilitato quando l'utente non lo ha impostato in [`enabledPlugins`](/docs/it/settings-reference#enabledplugins). Il valore della voce ha la precedenza su `plugin.json`                                                                                                                                        |
| `displayName`    | string           | Nome leggibile mostrato nell'interfaccia utente. Quando né la voce né il `plugin.json` del plugin ne impostano uno, gli utenti vedono il `name` del plugin                                                                                                                                                                                        |
| `metadata`       | object           | Oggetto libero per i propri campi. Claude Code non lo legge. Richiede Claude Code v2.1.222 o successivo                                                                                                                                                                                                                                           |
| `headers`        | object           | Intestazioni HTTP che Claude Code invia quando scarica l'[archivio](#archive-plugin-source) di questa voce. Un'intestazione impostata qui sostituisce un'intestazione con lo stesso nome dalla [`headers`](#fields-by-type) della sorgente del marketplace. Richiede Claude Code v2.1.238 o successivo                                            |
| `headersHelper`  | string           | Comando che stampa le intestazioni di download dell'archivio di questa voce come un oggetto JSON, per una credenziale che scade. La voce deve anche impostare [`"strict": false`](#strict-mode). Richiede Claude Code v2.1.238 o successivo. Vedere [Authenticate archive downloads](/docs/it/plugins/host-marketplace#authenticate-archive-downloads) |

<h3 id="entry-and-plugin-json">
  How an entry combines with plugin.json
</h3>

I campi della voce si applicano diversamente a un plugin recuperato che ha il suo `.claude-plugin/plugin.json` e a uno che non lo ha:

* **No `plugin.json`**: la voce è il manifesto indipendentemente da `strict`. Ogni campo del manifesto nella voce si applica, inclusi [`mcpServers`, `lspServers`, `userConfig` e `channels`](/docs/it/plugins/manifest-reference).
* **`plugin.json` presente**: `plugin.json` è il manifesto. [Strict mode](#strict-mode) decide se i sei campi componenti della voce, `commands`, `agents`, `skills`, `hooks`, `outputStyles` e `themes`, sono combinati con esso o rifiutati come conflitto. La voce `mcpServers`, `lspServers`, `userConfig` e `channels` non si applicano. Dichiararli in `plugin.json`.

<h4 id="hooks-in-an-entry">
  Hooks in an entry
</h4>

Scrivere `hooks` della voce come un oggetto inline che mappa i nomi degli eventi hook agli array di matcher. Se si scrive un percorso di file o un array, `claude plugin validate` lo passa. Questi hook non vengono mai eseguiti e Claude Code segnala un errore `not yet supported in a marketplace entry` per il plugin. Mettere gli hook basati su file nel [`hooks/hooks.json`](/docs/it/plugins/components) del plugin stesso o in `plugin.json`.

<h4 id="display-fields">
  Display fields
</h4>

Sia la voce che il `plugin.json` del plugin stesso possono impostare i campi di visualizzazione `displayName`, `description`, `author`, `homepage`, `repository`, `license` e `keywords`. Gli utenti vedono questi valori negli elenchi e nei dettagli dei plugin, prima e dopo l'installazione:

* Per un campo impostato sulla voce, gli utenti vedono il valore della voce, anche quando `plugin.json` ne imposta uno diverso.
* Per un campo che la voce lascia non impostato, gli utenti vedono il valore di `plugin.json`.

Prima dell'installazione, Claude Code può leggere `plugin.json` solo per le voci con una [sorgente relativa](#relative-path-plugin-source), i cui file di plugin si trovano all'interno del marketplace stesso. Per una voce con qualsiasi altro tipo di sorgente, gli utenti vedono solo i campi propri della voce fino a quando non installano il plugin.

<h3 id="strict-mode">
  Strict mode
</h3>

`strict` decide cosa succede quando il plugin recuperato ha il suo `plugin.json` e la voce dichiara anche uno dei [campi componenti](#entry-and-plugin-json): `commands`, `agents`, `skills`, `hooks`, `outputStyles` o `themes`. Con `strict: true`, il predefinito, Claude Code aggiunge i campi componenti della voce a `plugin.json`, tranne `hooks`, i cui matcher sostituiscono quelli del manifesto per evento. Con `strict: false`, una voce che dichiara un campo componente è un conflitto e il plugin non riesce a caricarsi. La tabella mostra ogni combinazione di `strict`, `plugin.json` e i campi componenti della voce.

| `strict`               | `plugin.json` | Entry component fields | Result                                                                                                                                                                                                                                         |
| :--------------------- | :------------ | :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| any                    | absent        | any                    | La voce è il manifesto                                                                                                                                                                                                                         |
| `true`, il predefinito | present       | any                    | `plugin.json` è l'autorità. Claude Code aggiunge i campi componenti della voce a esso, tranne `hooks`, i cui matcher [sostituiscono quelli del manifesto per evento](/docs/it/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`                | present       | none                   | `plugin.json` è il manifesto, come con `true`                                                                                                                                                                                                  |
| `false`                | present       | one or more            | Conflitto. Il plugin non riesce a caricarsi con `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components`                                                                                           |

<h2 id="plugin-sources">
  Plugin sources
</h2>

La `source` di una voce di plugin dice da dove Claude Code recupera quel plugin. È una stringa di percorso relativo o un oggetto la cui chiave `source` nomina il tipo, quindi una voce assomiglia a `"source": { "source": "github", "repo": "your-org/formatter" }`.

La tabella elenca ogni tipo di sorgente di plugin e i suoi campi.

| Type          | Fields                           | Notes                                                                                                                                                                                                                                                |
| :------------ | :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Relative path | the string itself                | Una directory all'interno del marketplace, risolta dalla radice del marketplace. Deve iniziare con `./`, a meno che non si scriva un [nome bare sotto `metadata.pluginRoot`](#relative-path-plugin-source). `"."` da solo significa la radice stessa |
| `github`      | `repo`, `ref`, `sha`             | Repository GitHub in forma `owner/repo`                                                                                                                                                                                                              |
| `url`         | `url`, `ref`, `sha`              | Qualsiasi repository git per URL                                                                                                                                                                                                                     |
| `git-subdir`  | `url`, `path`, `ref`, `sha`      | Una sottodirectory di un repository git, recuperata con un clone parziale sparse                                                                                                                                                                     |
| `npm`         | `package`, `version`, `registry` | Pacchetto npm, recuperato con il client npm e decompresso senza eseguire script di installazione                                                                                                                                                     |
| `archive`     | `url`, `sha256`                  | Archivio Zip su HTTPS. Richiede Claude Code v2.1.224 o successivo                                                                                                                                                                                    |
| `command`     | `command`, `timeout`, `mode`     | Directory stampata da un comando che Claude Code esegue sulla macchina dell'utente. Richiede Claude Code v2.1.229 o successivo                                                                                                                       |

I nomi `url` e `github` sono anche tipi di [sorgente marketplace](#marketplace-sources), dove `url` significa un collegamento diretto a un file `marketplace.json` piuttosto che a un repository git. `git` esiste solo come sorgente marketplace e `npm` esiste come entrambi. `git-subdir`, `archive` e `command` esistono solo come sorgenti di plugin.

Utilizzare un percorso relativo per un plugin in una sottodirectory del repository del marketplace stesso. Utilizzare `git-subdir` per una sottodirectory di un altro repository.

Le sorgenti `github`, `url` e `git-subdir` condividono i campi `ref` e `sha`:

* **`ref`**: un ramo o un tag. Predefinito al ramo predefinito del repository.
* **`sha`**: un SHA di commit completo di 40 caratteri in minuscolo. Quando si impostano sia `ref` che `sha`, Claude Code estrae `sha`. Sulla maggior parte degli host git, inclusi GitHub, GitLab e Bitbucket, ciò significa che l'installazione ha successo anche se il ramo o il tag denominato da `ref` è stato eliminato a monte, purché il commit sia ancora raggiungibile dal repository. Alcuni server, come AWS CodeCommit, non supportano il recupero di commit per SHA. Su questi server il `ref` deve ancora esistere e il commit bloccato deve essere raggiungibile da esso.

Per come ogni tipo viene recuperato, memorizzato nella cache e versionato, vedere [Plugin loading reference](/docs/it/plugins/loading).

<h3 id="relative-path-plugin-source">
  Relative path plugin source
</h3>

Il percorso si risolve dalla radice del marketplace. `./plugins/formatter` è `<root>/plugins/formatter` anche se il file marketplace è in `<root>/.claude-plugin/`.

Un percorso contenente `..` non supera la convalida. Su macOS e Linux, Claude Code rifiuta un percorso di voce che contiene una barra rovesciata in qualsiasi punto dopo il `./` iniziale, quindi scrivere il percorso con barre in avanti.

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

Un percorso relativo si risolve solo quando Claude Code ha i file del marketplace, quindi controllare il tipo di [sorgente marketplace](#marketplace-sources):

* **`github`, `git`, `file` e `directory`**: Claude Code ha i file del marketplace.
* **`url`**: Claude Code recupera solo `marketplace.json`, quindi i percorsi relativi non possono risolversi. Dare a ogni plugin una sorgente di oggetto, come `github` o `git-subdir`.
* **`settings`**: i percorsi relativi sono rifiutati completamente.

<h4 id="bare-names-under-pluginroot">
  Bare names under pluginRoot
</h4>

Un nome bare è un singolo nome di directory senza `/`, come `"formatter"`. Per scrivere nomi bare invece di percorsi `./`, impostare [`metadata.pluginRoot`](#top-level-fields) sulla directory in cui si risolvono. Con `"pluginRoot": "./plugins"`, `"source": "formatter"` si risolve in `./plugins/formatter`. Richiede Claude Code v2.1.239 o successivo.

`metadata.pluginRoot` ha questi limiti:

* Deve essere esso stesso un percorso relativo all'interno del marketplace.
* Non ha effetto su una sorgente che inizia già con `./`.
* Una sorgente che contiene un `/`, come `team-a/formatter`, non è un nome bare e ha ancora bisogno del prefisso `./`, anche quando `metadata.pluginRoot` è impostato.

<h3 id="github-plugin-source">
  github plugin source
</h3>

`repo` accetta `owner/repo`. `ref` e `sha` sono facoltativi.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  url plugin source
</h3>

`url` è un URL git completo: `https://`, `http://`, `file://` o `git@`. Un suffisso `.git` non è obbligatorio, quindi gli URL di Azure DevOps e AWS CodeCommit funzionano come scritti. Questo tipo non accetta la scorciatoia `owner/repo`.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  git-subdir plugin source
</h3>

`url` accetta un URL git completo o la scorciatoia GitHub `owner/repo`. `path` è la sottodirectory che contiene il plugin e Claude Code scarica solo quella sottodirectory.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  npm plugin source
</h3>

Una sorgente `npm` accetta questi campi:

* `package`: un nome di pacchetto, o un nome con scope come `@your-org/formatter`
* `version`: una versione o un intervallo
* `registry`: un URL di registro per un pacchetto che non è nel registro predefinito

Claude Code recupera il pacchetto con il client npm. Gli script di installazione del pacchetto, come `preinstall` o `postinstall`, non vengono mai eseguiti e le sue dipendenze non vengono installate durante il recupero. Se il pacchetto ha un lockfile supportato accanto al suo `package.json`, Claude Code installa quelle [dipendenze del pacchetto Node.js](/docs/it/plugins/loading#node-js-package-dependencies) in un passaggio separato, anche con script disabilitati.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  archive plugin source
</h3>

`url` deve utilizzare `https://` e non può puntare a un host loopback, link-local o cloud-metadata.

La radice del plugin può essere in cima al zip o una directory in basso.

`sha256` è il digest dell'archivio come 64 caratteri esadecimali, maiuscoli o minuscoli. Quando lo si imposta, Claude Code rifiuta un download che non corrisponde.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  command plugin source
</h3>

Utilizzare una sorgente `command` quando uno strumento installato sulla macchina dell'utente produce la directory del plugin, come un IDE che renderizza il suo plugin per la toolchain che l'utente ha selezionato. Claude Code esegue il comando quando l'utente installa o aggiorna il plugin e [di nuovo una volta per sessione](/docs/it/plugins/loading#when-a-command-source-re-runs), quindi gli utenti ottengono l'output modificato dello strumento senza reinstallare.

Una sorgente `command` accetta questi campi:

* `command`: un comando shell che stampa il percorso assoluto della directory del plugin come una riga e esce 0. Claude Code mostra agli utenti l'intera stringa per la revisione prima di eseguirla. Scriverla come ASCII stampabile, al massimo 500 caratteri, senza una sequenza di quattro o più spazi.
* `timeout`: un numero intero di secondi da 1 a 600. Predefinito a 60.
* `mode`: `copy`, il predefinito, o `link`. Vedere [Copy mode and link mode](#copy-mode-and-link-mode).

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

Per come gli utenti accettano il comando, vedere [Install from your shell](/docs/it/plugins/install#install-from-your-shell). Per cosa vedono gli utenti dopo averlo modificato, vedere [Change the command of a command source](/docs/it/plugins/host-marketplace#change-the-command-of-a-command-source). Gli amministratori disattivano le sorgenti di comando con [`disableCommandPluginSources`](/docs/it/settings-reference#disablecommandpluginsources).

<h4 id="what-the-command-must-do">
  What the command must do
</h4>

Scrivere il comando per soddisfare questi requisiti:

* **Shell e directory di lavoro**: Claude Code esegue il comando attraverso `sh`, o attraverso `cmd.exe` su Windows, dalla directory home dell'utente. Fornire un percorso assoluto o un comando su `PATH`.
* **Output**: stampare esattamente una riga su stdout, il percorso assoluto della directory del plugin, e uscire 0 entro `timeout` secondi.
* **Contenuti della directory**: la directory contiene il plugin completo al momento dell'uscita del comando. Il percorso può differire da un'esecuzione all'altra.

<h4 id="output-that-fails-the-install-or-update">
  Output that fails the install or update
</h4>

L'installazione o l'aggiornamento non riesce quando il comando esce non-zero, viene eseguito più a lungo di `timeout`, o stampa qualcosa di diverso da un percorso assoluto. Fallisce anche quando la directory stampata è una di queste:

* **Nessun contenuto di plugin**: la directory stampata non ha contenuto di plugin al suo livello superiore, come una directory `.claude-plugin/` o una directory `skills/`, `commands/`, `agents/` o `hooks/`.
* **La directory della sessione stessa**: la directory stampata è quella in cui Claude Code è stato avviato, o una dei suoi genitori.
* **Un percorso di rete**: su Windows, il percorso stampato è un percorso UNC.
* **Troppo grande da copiare**: in modalità copia, la directory è più grande di 256 MiB o ha più di 20.000 voci.

<h4 id="copy-mode-and-link-mode">
  Copy mode and link mode
</h4>

`mode` decide se Claude Code copia la directory stampata o la utilizza in posizione:

* **`copy`**: Claude Code copia la directory nella cache del plugin e deriva la [versione del plugin](/docs/it/plugins/loading#how-claude-code-computes-the-version) da un hash dei file copiati. Lo strumento può eliminare o riscrivere la directory dopo l'uscita del comando. Una ri-esecuzione che produce file identici conta come aggiornato.
* **`link`**: Claude Code riempie la voce della cache del plugin con un collegamento a ogni voce di primo livello della directory stampata e carica i file in posizione. Nulla viene copiato, i contenuti dei file non vengono sottoposti a hash e i limiti di dimensione non si applicano. Utilizzarlo per una directory troppo grande da copiare, come un'esportazione SDK renderizzata.

Un plugin in modalità link ha questi requisiti:

* **Mantenere la directory in posizione**: Claude Code carica il plugin attraverso i link ad ogni avvio, quindi la directory stampata deve rimanere dove si trova finché il plugin rimane installato.
* **Stampare un percorso diverso per segnalare nuovo contenuto**: la versione proviene dal percorso reale della directory stampata e dalle sue voci di primo livello, non dai file al loro interno.
* **Mantenere i symlink di primo livello all'interno della directory**: l'installazione non riesce se una voce di primo livello è un symlink che punta al di fuori della directory stampata.
* **Includere `node_modules`**: Claude Code salta l'[installazione della dipendenza del pacchetto Node.js](/docs/it/plugins/loading#node-js-package-dependencies) per un plugin in modalità link, quindi stampare una directory che contiene già i pacchetti di cui il plugin ha bisogno.
* **Sessioni avviate all'interno della directory**: una sessione avviata nella directory stampata o in qualsiasi punto al di sotto di essa non carica il plugin.
* **Non su Windows**: Claude Code rifiuta di installare un plugin in modalità link su Windows. Dichiarare `"mode": "copy"` lì.

<h2 id="marketplace-sources">
  Marketplace sources
</h2>

Una sorgente marketplace dice da dove Claude Code recupera un `marketplace.json`. La CLI ne crea una per voi quando aggiungete un marketplace, e ne scrivete una voi stessi nelle impostazioni:

* **[`claude plugin marketplace add`](/docs/it/plugins/cli-reference)**: Claude Code crea la sorgente dalla stringa che passate.
* **[`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces)**: scrivete la sorgente voi stessi come oggetto `source`.
* **[`strictKnownMarketplaces`](/docs/it/settings-reference#strictknownmarketplaces) e [`blockedMarketplaces`](/docs/it/plugins/org#restrict-what-users-can-install)**: gli amministratori scrivono le sorgenti in questi due elenchi di policy. `strictKnownMarketplaces` è l'allowlist e `blockedMarketplaces` è la blocklist.

I nomi di tipo `url`, `git` e `github` significano qualcosa di diverso in una sorgente marketplace rispetto a una [sorgente di plugin](#plugin-sources):

| Type name | As a marketplace source                                                                            | As a plugin source                                                     |
| :-------- | :------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `url`     | Un collegamento diretto a un file `marketplace.json`, con campi `url`, `headers` e `headersHelper` | Un repository git da clonare, con campi `url`, `ref` e `sha`           |
| `git`     | Un repository git da clonare, con campi `url`, `ref`, `path` e `sparsePaths`                       | Non esiste                                                             |
| `github`  | Un repository GitHub, con campi `repo`, `ref`, `path` e `sparsePaths`                              | Un repository GitHub, con campi `repo`, `ref` e `sha`, e nessun `path` |

La tabella elenca ogni tipo di sorgente marketplace con i suoi campi, l'input `claude plugin marketplace add` che lo produce e cosa fa in ciascuna delle tre chiavi di impostazioni.

| Type          | Fields                               | `marketplace add` input                                                                                                                                   | `extraKnownMarketplaces`                                              | `strictKnownMarketplaces`                                                                                                                                                                                                                                  | `blockedMarketplaces`                                                  |
| :------------ | :----------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `url`         | `url`, `headers`, `headersHelper`    | Un URL `http://` o `https://` che non corrisponde a una forma git                                                                                         | Carica                                                                | Consente lo stesso URL                                                                                                                                                                                                                                     | Blocca lo stesso URL                                                   |
| `github`      | `repo`, `ref`, `path`, `sparsePaths` | `owner/repo`, `owner/repo@ref` o `owner/repo#ref`                                                                                                         | Carica                                                                | Consente lo stesso `repo`, `ref` e `path`. `repo` può essere `owner/*`                                                                                                                                                                                     | Blocca lo stesso e un URL `git` allo stesso repository                 |
| `git`         | `url`, `ref`, `path`, `sparsePaths`  | Un URL `user@host:path`, o un URL `https://` che termina in `.git`, contiene `/_git/` o nomina un repository github.com o gitlab.com. `#ref` fissa un ref | Carica                                                                | Consente lo stesso URL, `ref` e `path`                                                                                                                                                                                                                     | Blocca lo stesso e altre ortografie dello stesso repository github.com |
| `npm`         | `package`                            | Non prodotto                                                                                                                                              | Non riesce a caricarsi: `NPM marketplace sources not yet implemented` | Analizza ma non corrisponde a nulla, perché nulla registra un marketplace `npm`                                                                                                                                                                            | Analizza ma non corrisponde a nulla                                    |
| `file`        | `path`                               | Un percorso a un file `.json`                                                                                                                             | Carica                                                                | Consente lo stesso percorso                                                                                                                                                                                                                                | Blocca lo stesso percorso                                              |
| `directory`   | `path`                               | Un percorso a una directory                                                                                                                               | Carica                                                                | Consente lo stesso percorso                                                                                                                                                                                                                                | Blocca lo stesso percorso                                              |
| `settings`    | `name`, `plugins`, `owner`           | Non prodotto                                                                                                                                              | Carica                                                                | Consente una voce con lo stesso `name` e `plugins` identici                                                                                                                                                                                                | Blocca lo stesso `name`                                                |
| `skills-dir`  | none                                 | Non prodotto                                                                                                                                              | Non riesce a caricarsi: `Unsupported marketplace source type`         | Mantiene il caricamento dei [plugin della directory delle competenze](/docs/it/plugins/org#keep-skills-directory-plugins-loading) mentre è impostata un'allowlist. Vedere [Source values valid only in policy lists](#source-values-valid-only-in-policy-lists) | Interrompe il caricamento dei plugin della directory delle competenze  |
| `hostPattern` | `hostPattern`                        | Non prodotto                                                                                                                                              | Non riesce a caricarsi: `Unsupported marketplace source type`         | Consente sorgenti `github`, `git` e `url` il cui host corrisponde                                                                                                                                                                                          | Blocca quelle sorgenti                                                 |
| `pathPattern` | `pathPattern`                        | Non prodotto                                                                                                                                              | Non riesce a caricarsi: `Unsupported marketplace source type`         | Consente sorgenti `file` e `directory` il cui `path` corrisponde                                                                                                                                                                                           | Blocca quelle sorgenti                                                 |

<h3 id="fields-by-type">
  Fields by type
</h3>

La tabella elenca ogni campo della sorgente marketplace che ha un predefinito, un vincolo o un significato specifico del tipo.

| Field           | Types           | Description                                                                                                                                                                                                                                                                   |
| :-------------- | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`           | `url`           | Collegamento al file `marketplace.json`. Claude Code scarica solo quel file, quindi i plugin del marketplace non possono utilizzare [sorgenti di percorso relativo](#relative-path-plugin-source)                                                                             |
| `url`           | `git`           | Il repository git da clonare                                                                                                                                                                                                                                                  |
| `headers`       | `url`           | Mappa di intestazioni HTTP che Claude Code invia con il recupero, per host autenticati                                                                                                                                                                                        |
| `headersHelper` | `url`           | Comando che stampa intestazioni i cui valori sono troppo brevi per essere elencati in `headers`. Richiede Claude Code v2.1.238 o successivo. Vedere [Authenticate archive downloads](/docs/it/plugins/host-marketplace#authenticate-archive-downloads)                             |
| `repo`          | `github`        | In `marketplace add` e `extraKnownMarketplaces`, `repo` deve nominare un repository. `marketplace add` rifiuta `owner/*` come scorciatoia `owner/repo` non valida; in `extraKnownMarketplaces` Claude Code la prende letteralmente e il clone non riesce                      |
| `ref`           | `github`, `git` | Ramo o tag. Predefinito al ramo predefinito del repository                                                                                                                                                                                                                    |
| `path`          | `github`, `git` | Il percorso del file marketplace all'interno del repository. Predefinito a `.claude-plugin/marketplace.json`                                                                                                                                                                  |
| `path`          | `file`          | Il file marketplace stesso. Claude Code lo legge in posizione e prende la directory due livelli su come radice del marketplace, quindi mantenere il file in `<root>/.claude-plugin/marketplace.json`                                                                          |
| `path`          | `directory`     | La radice del marketplace, la directory che contiene `.claude-plugin/marketplace.json`                                                                                                                                                                                        |
| `sparsePaths`   | `github`, `git` | Array di directory per un checkout sparse, come `[".claude-plugin", "plugins"]`. `claude plugin marketplace add --sparse` lo imposta                                                                                                                                          |
| `skipLfs`       | `github`, `git` | Accettato e non ha effetto. Vedere [Keep plugin files out of Git LFS](/docs/it/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs)                                                                                                                                          |
| `name`          | `settings`      | Deve essere uguale alla chiave `extraKnownMarketplaces` e non può essere un [nome riservato](#reserved-names)                                                                                                                                                                 |
| `plugins`       | `settings`      | Il catalogo inline, senza file ospitato. Ogni elemento accetta `name`, `source`, `description`, `version`, `strict`, `headers` e `headersHelper`. Scrivere la `source` di ogni elemento come tipo di oggetto, perché un percorso relativo non ha repository da cui risolversi |

<h3 id="source-values-valid-only-in-policy-lists">
  Source values valid only in policy lists
</h3>

`hostPattern`, `pathPattern`, `skills-dir` e la forma `owner/*` di `repo` sono validi solo nei due elenchi di policy, `strictKnownMarketplaces` e `blockedMarketplaces`:

* **`hostPattern` e `pathPattern`**: espressioni regolari che Claude Code testa rispetto a una sorgente prima di recuperare da essa.
* **`skills-dir`**: non una sorgente. Se si imposta `strictKnownMarketplaces` affatto, i [plugin della directory delle competenze](/docs/it/plugins/org#keep-skills-directory-plugins-loading) smettono di caricarsi fino a quando non si aggiunge `{"source": "skills-dir"}` a quell'elenco.
* **`owner/*`**: come valore `repo` di `github`, corrisponde a ogni repository esattamente sotto quel proprietario GitHub. Richiede Claude Code v2.1.223 o successivo.

Per l'ordine di corrispondenza, la semantica esatta di `ref` e le ricette, vedere [Manage plugins for your organization](/docs/it/plugins/org).

<h3 id="source-objects-in-settings">
  Source objects in settings
</h3>

Un valore `extraKnownMarketplaces` è una mappa dal nome del marketplace a un oggetto con `source`. Questa voce registra un marketplace da un repository git al suo ramo `main`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` e `blockedMarketplaces` sono array di oggetti sorgente. Questa allowlist ammette un proprietario GitHub e un host interno:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  Validation messages
</h2>

`claude plugin validate <path>` accetta la radice del marketplace o il file marketplace stesso. Stampa errori e avvisi. Per i codici di uscita e `--strict`, vedere [plugin validate](/docs/it/plugins/cli-reference#plugin-validate).

Un messaggio nomina una voce di plugin per il suo indice, scritto come `plugins.1.source` o `plugins[1].source`.

Un messaggio con prefisso di un indice di voce e `plugin.json →`, come `plugins[2] plugin.json →`, riguarda i file propri di quel plugin. [`claude plugin validate` segnala errori](/docs/it/plugins/troubleshooting#claude-plugin-validate-reports-errors) elenca quei messaggi con le loro correzioni.

Gli avvisi che menzionano i nomi dei flag di Claude Desktop segnalano i nomi che Claude Code accetta ma Claude Desktop rifiuta, perché le regole dei nomi di Claude Desktop sono più rigorose.

La tabella mappa i messaggi a livello di marketplace al campo di cui ciascuno parla.

| Message                                                                                                                                                                                       | Level   | Field                                                                                                                             |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------- |
| `Marketplace must have a name`                                                                                                                                                                | Error   | `name` è vuoto                                                                                                                    |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                             | Error   | `name`                                                                                                                            |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                         | Error   | `name`                                                                                                                            |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                      | Error   | `name`. Vedere [Reserved names](#reserved-names)                                                                                  |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                              | Error   | `name` contiene un carattere di controllo, come un escape o una nuova riga, o un carattere di formattazione bidirezionale Unicode |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, e le varianti `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github` e `gh` | Error   | `name`                                                                                                                            |
| `Author name cannot be empty`                                                                                                                                                                 | Error   | `owner.name`                                                                                                                      |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                       | Error   | `plugins[i].name`                                                                                                                 |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                   | Error   | `plugins[i].name`                                                                                                                 |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                              | Error   | Due voci condividono un `name`                                                                                                    |
| `plugins.i.source: Invalid input`                                                                                                                                                             | Error   | La `source` della voce non corrisponde a nessun tipo. Vedere [Invalid input on a source](#invalid-input-on-a-source)              |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                               | Error   | Una `source` relativa che sfugge dalla radice del marketplace                                                                     |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                             | Error   | `plugins[i].source`                                                                                                               |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                    | Error   | `plugins[i].headersHelper`, su una voce `archive`                                                                                 |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                           | Error   | `renames.<old>`                                                                                                                   |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                      | Error   | `renames.<old>`                                                                                                                   |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                     | Warning | La chiave denominata al livello superiore, sotto `metadata`, in una voce, o sotto la `relevance` di una voce                      |
| `Marketplace has no plugins defined`                                                                                                                                                          | Warning | `plugins` è vuoto                                                                                                                 |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                            | Warning | `plugins[i].headers` o `plugins[i].headersHelper`, su una voce la cui `source` non è `archive`                                    |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                                  | Warning | `plugins[i].source.sha256`                                                                                                        |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                    | Warning | `plugins[i].headers.<name>`                                                                                                       |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                          | Warning | `plugins[i].source`                                                                                                               |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                               | Warning | `description`                                                                                                                     |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                               | Warning | `plugins[i].version`, su una voce di percorso relativo                                                                            |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                    | Warning | `plugins[i].relevance`                                                                                                            |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                         | Warning | `plugins[i].metadata`                                                                                                             |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                            | Warning | `plugins[i].experimental`                                                                                                         |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                          | Warning | `name` è `org`, `org-provisioned` o `unknown`. Claude Desktop rifiuta il marketplace                                              |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                             | Warning | `name`. Claude Desktop rifiuta il marketplace                                                                                     |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                  | Warning | `plugins[i].name`. Claude Desktop scarta la voce                                                                                  |

<h3 id="invalid-input-on-a-source">
  Invalid input on a source
</h3>

`Invalid input` su una `source` significa che l'oggetto non ha corrisposto a nessun tipo di sorgente. Controllare queste cause:

* Un percorso relativo che non inizia con `./`, diverso da `"."` o un [nome bare sotto `metadata.pluginRoot`](#relative-path-plugin-source)
* Un `package` `npm` contenente `..`
* Un tipo di `source` che non è uno delle [sorgenti di plugin](#plugin-sources)
* Un tipo noto con un campo obbligatorio mancante o di tipo errato, come `github` senza `repo`

<h3 id="failures-that-validation-doesn’t-catch">
  Failures that validation doesn't catch
</h3>

`claude plugin validate` non segnala ogni fallimento. Una `hooks` di voce scritta come percorso di file o array passa la convalida e l'errore appare solo quando il plugin si carica, come [Hooks in an entry](#hooks-in-an-entry) descrive. Gli errori di recupero di una `source` appaiono anche solo dopo l'installazione, non nella convalida.

[`claude plugin list`](/docs/it/plugins/cli-reference) mostra un plugin che non è riuscito a caricarsi con il suo errore e [Troubleshoot plugins](/docs/it/plugins/troubleshooting) copre le stringhe di caricamento.

<h2 id="next-steps">
  Next steps
</h2>

* [Create a marketplace](/docs/it/plugins/create-marketplace): costruire un marketplace da questi campi e installare da esso localmente
* [Host and maintain a marketplace](/docs/it/plugins/host-marketplace): dove mettere il file e come gli utenti ricevono i cambiamenti
* [Plugin manifest reference](/docs/it/plugins/manifest-reference): i campi `plugin.json` che una voce può sovrascrivere
* [Manage plugins for your organization](/docs/it/plugins/org): ricette di allowlist e blocklist che utilizzano questi valori di sorgente
