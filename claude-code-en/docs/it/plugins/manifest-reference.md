> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento del manifest del plugin

> Riferimento completo per plugin.json: ogni campo con il suo tipo e valore predefinito, forme di percorso accettate, e gli schemi userConfig e variabili di ambiente.

Un manifest del plugin è il file `plugin.json` nella directory `.claude-plugin/` di un plugin. Contiene i metadati del plugin e i valori [`userConfig`](#user-configuration) che Claude Code richiede all'utente. Dichiara inoltre qualsiasi componente che definite inline o mantenete al di fuori della sua [posizione predefinita](#standard-layout).

Questo riferimento è per i creatori di plugin e per i proprietari di marketplace che inseriscono campi di componenti in una voce di marketplace.

<Note>
  Questi casi sono trattati in altre pagine:

  * **Imparare a costruire un plugin**: iniziate con [Create a plugin](/docs/it/plugins/create)
  * **Cosa fa ogni componente al runtime**: vedere [Plugin components](/docs/it/plugins/components)
</Note>

Iniziate dalla sezione che corrisponde a quello che state cercando:

* Un campo: la tabella [Fields](#fields) fornisce il tipo di ogni campo, se è obbligatorio, il valore predefinito e cosa accetta. [Path rules](#path-rules) copre il prefisso `./` e il contenimento per ogni percorso di componente
* Un'opzione `userConfig` o una voce `channels`: gli schemi [User configuration](#user-configuration) e [Channels](#channels)
* `${CLAUDE_PLUGIN_ROOT}` o un'altra variabile a cui un plugin può fare riferimento: [Environment variables](#environment-variables)
* Dove vanno i file di ogni componente: [Standard layout](#standard-layout)
* Un messaggio da `claude plugin validate`: la [pagina di troubleshooting](/docs/it/plugins/troubleshooting) elenca ogni messaggio con la sua correzione e i link alle sezioni rilevanti di questa pagina

<h2 id="manifest-file">
  Manifest file
</h2>

Il manifest è facoltativo. Senza di esso, Claude Code carica i componenti che trova nel [standard layout](#standard-layout). Il nome del plugin proviene quindi dalla voce di marketplace, o dal nome della directory quando caricate il plugin con `--plugin-dir`.

Scrivete un manifest quando volete metadati, un componente al di fuori della sua directory predefinita, `userConfig`, o una definizione di componente inline.

Salvate il manifest in `.claude-plugin/plugin.json` sotto la radice del plugin. Mettete ogni altro file del plugin alla radice del plugin, non dentro `.claude-plugin/`. Questo include `skills/`, `commands/` e `hooks/`.

L'esempio seguente imposta la maggior parte delle chiavi nella tabella [Fields](#fields). Passa la validazione in una directory di plugin che contiene ogni percorso referenziato.

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

<h3 id="unrecognized-fields">
  Unrecognized fields
</h3>

Una chiave di primo livello non riconosciuta viene rimossa, e una chiave non riconosciuta dentro un'opzione `userConfig`, una voce `channels`, una configurazione `lspServers`, o una voce `monitors` viene rifiutata:

* **Top-level fields**: il campo viene rimosso e il plugin si carica. `claude plugin validate` segnala ogni campo di primo livello non riconosciuto come un avviso
* **Strict objects**: le opzioni `userConfig`, le voci `channels`, le configurazioni `lspServers` e le voci `monitors` sono rigorose. Una chiave sconosciuta dentro una di esse è un errore, e il plugin non si carica

<h3 id="validate-the-manifest">
  Validate the manifest
</h3>

`claude plugin validate` è il controllo autorevole per un manifest. Eseguitelo dalla vostra shell contro la directory del plugin:

```bash theme={null}
claude plugin validate ./my-plugin
```

Il comando segnala uno di questi risultati:

* **`Validation passed`**: il manifest si carica
* **`Validation passed with warnings`**: il manifest si carica, ma il validatore ha trovato qualcosa da correggere, come un campo di primo livello sconosciuto che Claude Code rimuove, un `name` che non è in kebab-case, o un `version`, `description`, o `author` mancante. Passate `--strict` per trasformare gli avvisi in errori in CI
* **`Validation failed`**: il manifest ha una mancata corrispondenza di tipo, un percorso mancante o che esce dalla radice del plugin, o una chiave sconosciuta dentro un'opzione `userConfig`, una voce `channels`, una configurazione `lspServers`, o una voce `monitors`. Claude Code segnala lo stesso problema quando carica il plugin

<h2 id="fields">
  Fields
</h2>

La tabella elenca le chiavi di primo livello in `plugin.json`. `name` è l'unica chiave obbligatoria. Dove un nome di campo è un link, la sezione collegata ha le sue regole complete.

Per le chiavi di componente come `commands` e `hooks`, [Component path forms](#component-path-forms) mostra ogni forma accettata con un esempio, e ogni percorso segue le [path rules](#path-rules) per il prefisso `./`, le estensioni e il contenimento.

| Field                                | Type                             | Description                                                                                                                                                                                                                                                                                                                                    |
| :----------------------------------- | :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                            | String                           | URL dello schema JSON per l'autocompletamento dell'editor. Claude Code lo ignora al momento del caricamento                                                                                                                                                                                                                                    |
| [`name`](#name)                      | String                           | Identificatore del plugin, obbligatorio. Usate kebab-case. Ogni componente è namespacato sotto di esso                                                                                                                                                                                                                                         |
| [`displayName`](#displayname)        | String                           | Nome mostrato nell'interfaccia utente al posto di `name`                                                                                                                                                                                                                                                                                       |
| [`version`](#version)                | String                           | Stringa di versione. Impostarla mantiene gli utenti su quella versione finché non la cambiate                                                                                                                                                                                                                                                  |
| `description`                        | String                           | Breve spiegazione di cosa fornisce il plugin                                                                                                                                                                                                                                                                                                   |
| `author`                             | Object                           | `name`, che è obbligatorio, più `email` e `url` opzionali                                                                                                                                                                                                                                                                                      |
| `homepage`                           | String                           | URL della documentazione. Deve essere analizzabile come URL, altrimenti il plugin non si carica                                                                                                                                                                                                                                                |
| `repository`                         | String                           | URL del repository di origine. Non convalidato                                                                                                                                                                                                                                                                                                 |
| `license`                            | String                           | Identificatore SPDX come `MIT` o `Apache-2.0`                                                                                                                                                                                                                                                                                                  |
| `keywords`                           | Array of strings                 | Tag di scoperta                                                                                                                                                                                                                                                                                                                                |
| [`metadata`](#metadata)              | Object                           | Oggetto in forma libera per i vostri dati. Claude Code non lo legge                                                                                                                                                                                                                                                                            |
| [`defaultEnabled`](#defaultenabled)  | Boolean                          | Se il plugin inizia abilitato quando l'utente non lo ha impostato. Predefinito a `true`                                                                                                                                                                                                                                                        |
| [`dependencies`](#dependencies)      | Array of strings or objects      | Plugin che devono essere abilitati affinché questo funzioni                                                                                                                                                                                                                                                                                    |
| [`settings`](#settings)              | Object                           | Impostazioni che Claude Code applica mentre il plugin è abilitato. Solo `agent` e `subagentStatusLine` hanno effetto                                                                                                                                                                                                                           |
| [`userConfig`](#user-configuration)  | Object                           | Valori che Claude Code richiede all'utente quando il plugin è abilitato                                                                                                                                                                                                                                                                        |
| [`channels`](#channels)              | Array of objects                 | Canali di messaggi che il plugin fornisce, ciascuno associato a uno dei suoi server MCP                                                                                                                                                                                                                                                        |
| `skills`                             | Path, or array of paths          | Directory da scansionare per skills, ciascuna una directory di cartelle `<name>/SKILL.md` o una cartella che contiene `SKILL.md` direttamente. `"."` nomina la radice del plugin. Aggiunge alla scansione predefinita `skills/`                                                                                                                |
| [`commands`](#commands)              | Path, array of paths, or object  | File di comando `.md` piatti, directory di essi, o una mappa di oggetti del nome del comando a `source` o `content`. Sostituisce la scansione predefinita `commands/`                                                                                                                                                                          |
| `agents`                             | Path, or array of paths          | File di agente `.md`. Le directory non sono accettate. Sostituisce la scansione predefinita `agents/`                                                                                                                                                                                                                                          |
| [`hooks`](#hooks)                    | Path, object, or array of either | File hook `.json` o configurazione hook inline. Caricati insieme a `hooks/hooks.json`                                                                                                                                                                                                                                                          |
| [`mcpServers`](#mcpservers)          | Path, object, or array of either | File di configurazione MCP `.json`, bundle `.mcpb` o `.dxt`, o configurazioni di server inline con chiave per nome. Caricati insieme a `.mcp.json`; un nome di server dichiarato successivamente sostituisce uno precedente                                                                                                                    |
| [`lspServers`](#lspservers)          | Path, object, or array of either | File di configurazione LSP `.json` o configurazioni di server inline con chiave per nome. Caricati insieme a `.lsp.json`                                                                                                                                                                                                                       |
| `outputStyles`                       | Path, or array of paths          | File di stile di output o directory. Sostituisce la scansione predefinita `output-styles/`                                                                                                                                                                                                                                                     |
| `workflows`                          | Path, or array of paths          | File [Workflow](/docs/it/workflows#distribute-a-workflow-in-a-plugin) `.js` o directory. Sostituisce la scansione predefinita `workflows/`                                                                                                                                                                                                          |
| `experimental`                       | Object                           | Contenitore per `themes`, `monitors` e `evals`, la cui forma di manifest potrebbe ancora cambiare                                                                                                                                                                                                                                              |
| `experimental.themes`                | Path, or array of paths          | File di tema o directory. Sostituisce la scansione predefinita `themes/`. Una chiave `themes` di primo livello si carica ancora, con un avviso `claude plugin validate`                                                                                                                                                                        |
| [`experimental.monitors`](#monitors) | Path, or inline array            | Un file `.json` che contiene l'array monitors, o l'array stesso. Predefinito a `monitors/monitors.json`. Una chiave `monitors` di primo livello si carica ancora, con un avviso `claude plugin validate`. I monitor vengono eseguiti solo in sessioni interattive, e non su Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry |
| `experimental.evals`                 | Path, or array of paths          | Directory che contiene i [casi di eval](/docs/it/plugin-evals#use-a-different-eval-directory) del plugin quando non è la directory predefinita `evals/`. `claude plugin eval --eval-dir` la sostituisce                                                                                                                                             |

Nella colonna Type, un percorso è una stringa relativa alla radice del plugin, come `"./custom/commands"`.

<h3 id="name">
  `name`
</h3>

L'identificatore del plugin. Deve essere non vuoto, senza spazi, `@`, `:`, separatori di percorso, caratteri di controllo, o caratteri di formattazione bidirezionale; usate kebab-case.

Claude Code namespaccia ogni componente sotto di esso, quindi un agente `reviewer` nel plugin `deploy-tools` appare come `deploy-tools:reviewer`.

<h3 id="displayname">
  `displayName`
</h3>

Il nome mostrato nell'interfaccia utente al posto di `name`. Può contenere spazi e qualsiasi maiuscola/minuscola, e non viene utilizzato per il namespacing o la ricerca.

Per un plugin installato da marketplace, un `displayName` sulla [voce di marketplace](/docs/it/plugins/marketplace-reference#plugin-entries) ha la precedenza su questo valore.

<h3 id="version">
  `version`
</h3>

Una stringa di versione, non controllata rispetto a semver. Impostarla fissa il plugin a quella versione finché non la cambiate; vedere [Versions and updates](/docs/it/plugins/loading#versions-and-updates). Un plugin con una [`command` source](/docs/it/plugins/marketplace-reference), un plugin da un [marketplace ospitato su claude.ai](/docs/it/plugins/install#add-from-claude-ai), e un plugin [caricato in place](/docs/it/plugins/loading#find-plugins-on-disk) da un marketplace aggiunto come directory locale non sono fissati da questo campo.

<h3 id="metadata">
  `metadata`
</h3>

Un oggetto in forma libera per i vostri dati, come campi di catalogo o di diritto. Claude Code non lo legge. Richiede Claude Code v2.1.222 o successivo.

<h3 id="defaultenabled">
  `defaultEnabled`
</h3>

Se il plugin inizia abilitato quando l'utente non lo ha impostato in [`enabledPlugins`](/docs/it/settings-reference#enabledplugins). Predefinito a `true`. Un plugin da cui dipende un plugin abilitato inizia abilitato indipendentemente. Lo stesso campo nella voce di marketplace sostituisce questo.

Una volta che la voce `enabledPlugins` di un utente è scritta, persiste attraverso gli aggiornamenti del plugin, quindi cambiare `defaultEnabled` in una versione successiva non cambia l'impostazione per un utente esistente.

<h3 id="dependencies">
  `dependencies`
</h3>

Plugin che devono essere abilitati affinché questo funzioni. Ogni voce è `"name"`, `"name@marketplace"`, o `{ "name": "...", "marketplace": "...", "version": "..." }`. I nomi nudi si risolvono rispetto al proprio marketplace del plugin. Vedere [dependency constraints](/docs/it/plugins/dependencies).

<h3 id="settings">
  `settings`
</h3>

Impostazioni che Claude Code applica mentre il plugin è abilitato. Solo `agent` e `subagentStatusLine` hanno effetto; altre chiavi vengono eliminate al caricamento. Un `settings.json` alla radice del plugin ha la precedenza su questa chiave. Vedere [Default settings](/docs/it/plugins/components#default-settings).

<h2 id="component-path-forms">
  Component path forms
</h2>

Ogni chiave di componente accetta un percorso relativo alla radice del plugin. `hooks`, `mcpServers`, `lspServers` e `experimental.monitors` accettano anche configurazione inline, `commands` accetta anche una mappa di oggetti, e `mcpServers` accetta anche percorsi di bundle MCP e URL. Gli esempi che seguono mostrano ogni forma accettata una volta. Per cosa fa ogni componente al runtime, vedere [Plugin components](/docs/it/plugins/components).

<h3 id="path-only-fields">
  Path-only fields
</h3>

`agents`, `skills`, `outputStyles`, `workflows` e `experimental.themes` accettano un percorso o un array di percorsi. Le voci `agents` devono essere file `.md`, e le voci `skills` devono essere directory. Gli altri tre accettano una directory o un file.

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

<h3 id="commands">
  `commands`
</h3>

`commands` accetta un percorso, un array di percorsi, o una mappa di oggetti. Un percorso nomina un file di comando `.md` piatto o una directory. Nella mappa di oggetti, ogni chiave diventa il nome del comando dopo il prefisso del plugin. Ad esempio, `"about"` nel plugin `deploy-tools` viene eseguito come `/deploy-tools:about`.

Ogni valore imposta esattamente uno di `source` o `content`, e una voce che imposta entrambi o nessuno dei due non passa la validazione. Gli altri campi in questa tabella sono opzionali:

| Field          | Type             | Description                                                                |
| :------------- | :--------------- | :------------------------------------------------------------------------- |
| `source`       | string           | Percorso al file Markdown del comando, relativo alla radice del plugin     |
| `content`      | string           | Markdown inline per il corpo del comando, invece di `source`               |
| `description`  | string           | Descrizione mostrata per il comando                                        |
| `argumentHint` | string           | Suggerimento di argomento mostrato dopo il nome del comando, come `[file]` |
| `model`        | string           | Modello predefinito per il comando                                         |
| `allowedTools` | array of strings | Strumenti che il comando può utilizzare senza chiedere                     |

Questa mappa dichiara un comando da un file e uno da contenuto inline:

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

<h3 id="hooks">
  `hooks`
</h3>

`hooks` accetta un percorso di file `.json`, un oggetto hooks inline nella stessa forma di [`hooks` in `settings.json`](/docs/it/hooks#configuration), o un array che mescola entrambi. Per gli eventi hook e i campi del gestore, vedere il [riferimento hooks](/docs/it/hooks#hook-events).

Claude Code unisce tutto ciò che dichiarate con `hooks/hooks.json` quando quel file esiste.

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

<h3 id="mcpservers">
  `mcpServers`
</h3>

`mcpServers` accetta un percorso di file `.json`, un percorso di bundle MCP o URL, una mappa inline, o un array che mescola loro. Per i campi di configurazione del server, vedere [plugin-provided MCP servers](/docs/it/mcp#plugin-provided-mcp-servers).

Claude Code carica `.mcp.json` alla radice del plugin per primo, poi ogni forma dichiarata in ordine. Un nome di server dichiarato successivamente sostituisce uno precedente.

Un valore `mcpServers` assume una di queste forme:

| Shape             | Example value                                                                          | What Claude Code does                                                                                                   |
| :---------------- | :------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `.json` file path | `"./mcp/servers.json"`                                                                 | Legge il file come una mappa `mcpServers`                                                                               |
| MCP bundle path   | `"./bundle.mcpb"`                                                                      | Estrae il bundle `.mcpb` o `.dxt` in `.mcpb-cache/` sotto la radice del plugin e legge la sua configurazione del server |
| MCP bundle URL    | `"https://example.com/server.mcpb"`                                                    | Scarica il bundle in `.mcpb-cache/`, poi lo legge                                                                       |
| Inline map        | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | Usa la mappa come configurazioni del server con chiave per nome                                                         |

Un percorso di bundle o URL deve terminare in `.mcpb` o `.dxt`. Qualsiasi altra estensione non passa la validazione.

<h3 id="lspservers">
  `lspServers`
</h3>

`lspServers` accetta un percorso di file `.json`, una mappa inline di nome del server a configurazione, o un array di uno qualsiasi.

Claude Code carica `.lsp.json` alla radice del plugin per primo, poi ogni configurazione dichiarata in ordine. Un nome di server dichiarato successivamente sostituisce uno precedente.

Ogni configurazione del server è un oggetto rigoroso con questi campi. Una chiave sconosciuta non passa la validazione.

| Field                   | Required | Description                                                                                                                                                                                     |
| :---------------------- | :------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`               | Yes      | Binario del language server. Nessuno spazio a meno che il valore non inizi con `/`; mettete gli argomenti in `args`                                                                             |
| `extensionToLanguage`   | Yes      | Mappa dell'estensione di file all'ID di linguaggio LSP, almeno una voce. Le chiavi iniziano con un punto, come `".go"`                                                                          |
| `args`                  | No       | Argomenti passati al server                                                                                                                                                                     |
| `transport`             | No       | Trasporto di comunicazione: `stdio` (predefinito) o `socket`. Claude Code accetta `socket` ma esegue ogni server su stdio, quindi le regole del protocollo stdout si applicano a tutti i server |
| `env`                   | No       | Variabili di ambiente per il processo del server                                                                                                                                                |
| `initializationOptions` | No       | Opzioni inviate nella richiesta di inizializzazione                                                                                                                                             |
| `settings`              | No       | Impostazioni inviate da `workspace/didChangeConfiguration`                                                                                                                                      |
| `workspaceFolder`       | No       | Percorso della cartella di lavoro per il server                                                                                                                                                 |
| `startupTimeout`        | No       | Millisecondi da attendere per l'avvio, un intero positivo                                                                                                                                       |
| `shutdownTimeout`       | No       | Millisecondi da attendere per un arresto elegante, un intero positivo. Quando il timeout scade, Claude Code termina il processo del server. Quando non impostato, non si applica alcun timeout  |
| `restartOnCrash`        | No       | Se riavviare il server dopo che si arresta in modo anomalo. Predefinito a `true`. Impostate a `false` per lasciare un server arrestato in modo anomalo fermo invece di riavviarlo               |
| `maxRestarts`           | No       | Tentativi di riavvio prima di rinunciare, zero o più                                                                                                                                            |
| `diagnostics`           | No       | Se spingere la diagnostica nel contesto dopo le modifiche. Predefinito a `true`                                                                                                                 |

Questa configurazione inline esegue `gopls` per i file `.go`:

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

Per i language server che Anthropic pubblica come plugin e come i server si comportano al runtime, vedere [Code intelligence](/docs/it/plugins/code-intelligence).

<h3 id="monitors">
  `monitors`
</h3>

`experimental.monitors` accetta un percorso di file `.json` o l'array inline. Quando omettete la chiave, Claude Code carica `monitors/monitors.json` se esiste.

Ogni voce è un oggetto rigoroso con questi campi.

| Field         | Required | Description                                                                                                                                                                                      |
| :------------ | :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Yes      | Identificatore unico all'interno del plugin                                                                                                                                                      |
| `command`     | Yes      | Comando shell che Claude Code esegue come processo di background persistente nella directory di lavoro della sessione                                                                            |
| `description` | Yes      | Breve riepilogo mostrato nel pannello dei task e nei riepiloghi delle notifiche                                                                                                                  |
| `when`        | No       | Con `"always"`, il predefinito, il monitor inizia all'avvio della sessione e al ricaricamento del plugin. Con `"on-skill-invoke:<skill>"`, inizia la prima volta che quella skill viene eseguita |

Questo array inline dichiara un monitor che inizia la prima volta che la skill `deploy` viene eseguita:

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

Un `command` di monitor non può fare riferimento a `${user_config.*}`. Vedere [Fields that run through a shell](#fields-that-run-through-a-shell).

<h2 id="path-rules">
  Regole dei percorsi
</h2>

Ogni percorso di componente in un manifest è relativo alla radice del plugin e deve iniziare con `./`. Un percorso come `commands/foo.md` non supera la convalida. `skills` e `mcpServers` accettano ciascuno una forma al di fuori di questa regola:

* **`skills`**: accetta anche `"."`. Sia `"."` che `"./"` indicano la radice del plugin. Prima della v2.1.221, `"."` non superava la convalida del manifest, quindi utilizzare `"./"` quando il plugin deve caricarsi su versioni precedenti
* **`mcpServers`**: accetta anche un URL di bundle `https://`

<h3 id="containment-and-existence">
  Contenimento ed esistenza
</h3>

Ogni percorso di componente deve risolversi all'interno della radice del plugin e deve esistere. `claude plugin validate` non controlla i percorsi `outputStyles`, `lspServers`, `monitors` o `themes`, quindi un percorso errato in questi campi non riesce solo quando il plugin si carica:

* **Contenimento**: un percorso che si risolve al di fuori della radice del plugin non si carica e la scheda **Errors** di `/plugin` mostra `<component> path escapes plugin directory: <path>`. Un percorso contenente `..` è il caso più comune e `claude plugin validate` lo segnala come `Path contains ".." which could be a path traversal attempt`
* **Esistenza**: un percorso che non esiste non si carica e la scheda **Errors** di `/plugin` mostra `<component> path not found: <path>`. `claude plugin validate` lo segnala come `Path not found`

<h3 id="how-each-key-combines-with-its-default-location">
  Come ogni chiave si combina con la sua posizione predefinita
</h3>

Ogni chiave di componente sostituisce la sua posizione predefinita, si aggiunge ad essa o si unisce ad essa:

* **Sostituisce il valore predefinito**: `commands`, `agents`, `outputStyles`, `workflows`, `experimental.themes`, `experimental.monitors`. Quando si imposta `commands`, la directory predefinita `commands/` non viene scansionata. Per mantenere il valore predefinito e aggiungerne altri, elencarli esplicitamente: `"commands": ["./commands/", "./extras/"]`
* **Si aggiunge al valore predefinito**: `skills`. La directory `skills/` viene ancora scansionata e le directory elencate si caricano insieme ad essa
* **Si unisce**: `hooks`, `mcpServers`, `lspServers`. Il file predefinito si carica per primo e ciò che il manifest dichiara si unisce ad esso, come descritto in [Component path forms](#component-path-forms)

Se un plugin ha una cartella predefinita come `commands/` e imposta anche la chiave del manifest che la sostituisce, Claude Code carica i percorsi del manifest e non la cartella. `claude plugin list` e l'interfaccia `/plugin` mostrano quindi l'avviso `Default <folder>/ folder is ignored because the manifest sets "<key>"`.

Per evitare l'avviso, impostare la chiave su un percorso all'interno di quella cartella: `"commands": ["./commands/deploy.md"]` nomina un file nella cartella predefinita e non produce alcun avviso.

<h2 id="user-configuration">
  Configurazione utente
</h2>

`userConfig` dichiara i valori che Claude Code richiede all'utente quando il plugin è abilitato, in modo che gli utenti non debbano modificare `settings.json` direttamente.

Le chiavi sono identificatori composti da lettere, cifre e caratteri di sottolineatura, e non possono iniziare con una cifra.

Ogni valore è un oggetto rigoroso con questi campi. Una chiave sconosciuta non supera la convalida.

| Campo         | Obbligatorio | Descrizione                                                                                                                                                                                                 |
| :------------ | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`        | Sì           | Uno di `string`, `number`, `boolean`, `directory`, o `file`                                                                                                                                                 |
| `title`       | Sì           | Etichetta mostrata nella finestra di dialogo di configurazione                                                                                                                                              |
| `description` | Sì           | Testo di aiuto mostrato sotto il campo                                                                                                                                                                      |
| `required`    | No           | Se `true`, la finestra di dialogo di configurazione non accetta un valore vuoto                                                                                                                             |
| `default`     | No           | Valore utilizzato quando l'utente non fornisce nulla: una stringa, un numero, un booleano, o un array di stringhe                                                                                           |
| `options`     | No           | Per `string`, i valori che il campo accetta, mostrati come un selettore in `/config`. Vedi [Limitare un campo a opzioni fisse](#limit-a-field-to-fixed-options). Richiede Claude Code v2.1.271 o successivo |
| `multiple`    | No           | Per `string`, consente un array di stringhe                                                                                                                                                                 |
| `sensitive`   | No           | Se `true`, maschera l'input e memorizza il valore nell'archiviazione sicura invece di `settings.json`                                                                                                       |
| `min` / `max` | No           | Limiti per `number`                                                                                                                                                                                         |

Ogni opzione di ogni plugin abilitato appare anche come una riga nel pannello `/config`, ad eccezione delle opzioni `sensitive` e degli elenchi `multiple`. Le righe `/config` richiedono Claude Code v2.1.269 o successivo.

Questo `userConfig` dichiara un endpoint e un token mascherato:

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

<h3 id="limit-a-field-to-fixed-options">
  Limitare un campo a opzioni fisse
</h3>

Impostare `options` su un campo `userConfig` per fare in modo che gli utenti scelgano il suo valore da un elenco fisso.

Per limitare un campo `tone` a tre opzioni, elencarle in `options` e impostare `default` su una di esse:

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

Se si dichiara `options` su qualsiasi campo, gli utenti su versioni di Claude Code precedenti a v2.1.271 non possono caricare il plugin.

`options` si applica a un campo `string` che non è `multiple` o `sensitive`. Impostare `default` su uno dei valori elencati, oppure impostare `required: true` in modo che l'utente debba sceglierne uno. Ogni opzione è un'etichetta semplice di 1 a 64 caratteri, e `claude plugin validate`, che si esegue nella shell, segnala tutto il resto che rifiuta. Un plugin le cui `options` violano queste regole non riesce a caricarsi.

<h3 id="where-values-are-stored">
  Dove vengono memorizzati i valori
</h3>

I valori non sensibili vengono salvati sotto [`pluginConfigs`](/docs/it/settings-reference#pluginconfigs) nel `settings.json` dell'utente. I valori sensibili vanno invece nell'archivio di credenziali sicure della piattaforma. La [pagina delle impostazioni](/docs/it/settings-reference#pluginconfigs) elenca da quali file di impostazioni viene letto `pluginConfigs`.

<h3 id="reference-a-saved-value">
  Fare riferimento a un valore salvato
</h3>

Fare riferimento a un valore salvato dove il plugin ne ha bisogno, in una di due forme:

* **`${user_config.KEY}`**: sostituito nella configurazione del server MCP, nella configurazione del server LSP, negli `args` dell'hook in [forma exec](/docs/it/hooks#exec-form-and-shell-form), e nel contenuto di skill e agent. Nel contenuto di skill e agent, solo i valori non sensibili vengono sostituiti, e un valore sensibile lì diventa un segnaposto
* **`CLAUDE_PLUGIN_OPTION_<KEY>`**: esportato ai processi hook per ogni opzione, con `<KEY>` in maiuscolo. Un hook in forma shell legge `$CLAUDE_PLUGIN_OPTION_API_TOKEN` per `api_token`

<h3 id="fields-that-run-through-a-shell">
  Campi che passano attraverso una shell
</h3>

I comandi hook in forma shell, i comandi di monitoraggio, e l'MCP [`headersHelper`](/docs/it/mcp#use-dynamic-headers-for-custom-authentication) rifiutano `${user_config.*}`. Un componente che lo riferisce in uno di questi campi non riesce con un [errore](/docs/it/errors#plugin-command-references-user-config) invece di eseguirsi, perché il valore del campo viene passato a una shell che riparsificherebbe il valore sostituito.

La tabella mostra come il valore può raggiungerlo per ciascuno di questi campi.

| Campo                       | Come il valore può raggiungerlo                                                                                                                                                                                                            |
| :-------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Comandi hook in forma shell | Utilizzare la [forma exec](/docs/it/hooks#exec-form-and-shell-form) con `args`, oppure leggere `CLAUDE_PLUGIN_OPTION_<KEY>` dall'ambiente dell'hook                                                                                             |
| Comandi di monitoraggio     | Non attraverso Claude Code. I processi di monitoraggio non ricevono `CLAUDE_PLUGIN_OPTION_<KEY>`, quindi lo script di monitoraggio deve ottenere il valore autonomamente                                                                   |
| MCP `headersHelper`         | Non attraverso Claude Code. L'ambiente dell'helper contiene `CLAUDE_PLUGIN_ROOT`, `CLAUDE_CODE_MCP_SERVER_NAME`, e `CLAUDE_CODE_MCP_SERVER_URL` ma nessun valore di opzione, quindi lo script helper deve ottenere il valore autonomamente |

<h2 id="channels">
  Channels
</h2>

`channels` dichiara i canali di messaggi che un plugin fornisce, come un ponte a un'app di chat. Quando ne dichiarate uno, Claude Code può chiedere la configurazione del canale quando il plugin è abilitato. Per come il server inietta i messaggi, vedere il [riferimento channels](/docs/it/channels-reference#package-as-a-plugin).

Ogni voce è un oggetto rigoroso associato a uno dei server MCP del plugin, con questi campi:

| Field         | Required | Description                                                                                                                                                                                   |
| :------------ | :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server`      | Yes      | Chiave del server MCP nel `mcpServers` di questo plugin a cui il canale si associa                                                                                                            |
| `displayName` | No       | Nome mostrato nel titolo della finestra di dialogo di configurazione. Predefinito al nome del server                                                                                          |
| `userConfig`  | No       | Opzioni da chiedere, nella stessa forma di [`userConfig` di primo livello](#user-configuration). I valori salvati si sostituiscono nei riferimenti `${user_config.KEY}` nell'`env` del server |

Questo manifest associa un canale al server MCP `telegram` del plugin e chiede un token bot che si sostituisce nell'`env` del server:

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

<h2 id="environment-variables">
  Environment variables
</h2>

Claude Code fornisce tre variabili di percorso ai componenti del plugin. Fate loro riferimento come `${NAME}` nei campi elencati sotto [Where each variable resolves](#where-each-variable-resolves), e leggetele come variabili di ambiente nei processi che le ricevono.

| Variable                | Resolves to                                                                                                                                                                                                                          | Use it for                                                         |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Percorso assoluto della versione installata del plugin                                                                                                                                                                               | Script, binari e file di configurazione forniti con il plugin      |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`, creato al primo riferimento e mantenuto attraverso gli aggiornamenti del plugin. `<id>` è l'identificatore del plugin con ogni carattere diverso da una lettera, cifra, `_`, o `-` sostituito da `-` | Dipendenze installate come `node_modules`, codice generato e cache |
| `${CLAUDE_PROJECT_DIR}` | La radice del progetto                                                                                                                                                                                                               | Script e file di configurazione locali del progetto                |

`${CLAUDE_PLUGIN_ROOT}` cambia quando il plugin si aggiorna, quindi non scrivete lo stato lì. Per dove si sposta la radice e quando la directory vecchia viene pulita, vedere la [pagina di caricamento](/docs/it/plugins/loading).

Quando disinstallate il plugin dall'ultimo posto in cui è installato, la directory `${CLAUDE_PLUGIN_DATA}` viene eliminata a meno che non passiate [`--keep-data`](/docs/it/plugins/cli-reference).

<h3 id="where-each-variable-resolves">
  Where each variable resolves
</h3>

In ogni componente del plugin, i riferimenti `${...}` si risolvono inline in campi specifici, e alcuni componenti ricevono anche le variabili nel loro ambiente di processo:

| Plugin component                  | Fields where `${...}` resolves              | Exported to the process                                                                         |
| :-------------------------------- | :------------------------------------------ | :---------------------------------------------------------------------------------------------- |
| Hook commands                     | Ovunque in `command` e `args`               | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR` e `CLAUDE_PLUGIN_OPTION_<KEY>` |
| Monitor commands                  | Ovunque in `command`                        | Non esportato                                                                                   |
| MCP `stdio` servers               | `command`, `args`, `env`                    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`                                                      |
| MCP `http`, `sse`, `ws` servers   | `url`, `headers`, `headersHelper`           | Non applicabile                                                                                 |
| LSP servers                       | `command`, `args`, `env`, `workspaceFolder` | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`                                |
| Skill, command, and agent content | Ovunque nel corpo Markdown                  | Non applicabile                                                                                 |

Le variabili non sono presenti nell'ambiente dei comandi che Claude esegue attraverso lo strumento Bash, nella sessione principale o in un subagent. Nel contenuto di skill, comando e agente, scrivete il riferimento `${...}` nel corpo Markdown invece, e Claude Code sostituisce il percorso inline quando carica il contenuto.

<h3 id="quoting-and-path-separators">
  Quoting and path separators
</h3>

Mantenete ogni percorso sostituito un singolo argomento:

* **Hook commands**: usate [exec form](/docs/it/hooks#exec-form-and-shell-form) con `args` in modo che ogni percorso sia un argomento senza virgolette
* **Shell-form hooks and monitor commands**: avvolgete la variabile tra virgolette doppie in modo che un percorso con spazi rimanga una parola

Questo hook in forma shell esegue uno script fornito con il plugin:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

Su Windows, i percorsi sostituiti usano barre in avanti in modo che una shell non legga le barre rovesciate come escape.

<h2 id="standard-layout">
  Standard layout
</h2>

Ogni tipo di componente ha una posizione predefinita sotto la radice del plugin, utilizzata quando il manifest non punta altrove.

| Component     | Default location             | Contents                                                                                                                                                                                                                                                                                                                                                    |
| :------------ | :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Manifest      | `.claude-plugin/plugin.json` | Metadati e configurazione del plugin. Facoltativo                                                                                                                                                                                                                                                                                                           |
| Skills        | `skills/`                    | Un `<name>/SKILL.md` per skill. Un plugin con `SKILL.md` alla sua radice, nessun `skills/`, e nessuna chiave `skills` si carica come una singola skill                                                                                                                                                                                                      |
| Commands      | `commands/`                  | File di comando Markdown piatti. Preferite `skills/` per i nuovi plugin                                                                                                                                                                                                                                                                                     |
| Agents        | `agents/`                    | File Markdown di agente. Le sottocartelle fanno parte del [nome dell'agente](/docs/it/plugins/components#agents)                                                                                                                                                                                                                                                 |
| Hooks         | `hooks/hooks.json`           | Configurazione hook                                                                                                                                                                                                                                                                                                                                         |
| MCP servers   | `.mcp.json`                  | Definizioni del server MCP                                                                                                                                                                                                                                                                                                                                  |
| LSP servers   | `.lsp.json`                  | Configurazioni del server LSP                                                                                                                                                                                                                                                                                                                               |
| Output styles | `output-styles/`             | File di stile di output Markdown                                                                                                                                                                                                                                                                                                                            |
| Workflows     | `workflows/`                 | File Workflow `.js`                                                                                                                                                                                                                                                                                                                                         |
| Themes        | `themes/`                    | File tema JSON                                                                                                                                                                                                                                                                                                                                              |
| Monitors      | `monitors/monitors.json`     | L'array monitors                                                                                                                                                                                                                                                                                                                                            |
| Executables   | `bin/`                       | I file qui sono sul `PATH` dello strumento Bash mentre il plugin è abilitato, quindi Claude li esegue come comandi nudi. claude.ai e Cowork non installano un plugin che ha questa directory, incluso uno che [distribuite attraverso le impostazioni dell'organizzazione claude.ai](/docs/it/plugins/host-marketplace#distribute-through-organization-settings) |
| Settings      | `settings.json`              | Impostazioni predefinite `agent` e `subagentStatusLine` applicate mentre il plugin è abilitato                                                                                                                                                                                                                                                              |

Un plugin che usa ogni posizione predefinita, più una cartella `scripts/` che i suoi hook chiamano, è disposto così:

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

Per fare clic attraverso questo layout e leggere cosa fa ogni file, aprite l'[esplora plugin](/docs/it/plugins/components#explore-the-plugin-directory).

Un `CLAUDE.md` alla radice del plugin non viene caricato come contesto, e `claude plugin validate` avvisa quando ne trova uno. Per includere istruzioni che si caricano nel contesto di Claude, mettetele in una skill.

<h2 id="marketplace-entries-and-the-manifest">
  Marketplace entries and the manifest
</h2>

Una [voce di marketplace](/docs/it/plugins/marketplace-reference) accetta ogni campo su questa pagina insieme ai [suoi propri campi](/docs/it/plugins/marketplace-reference#plugin-entries), incluso `strict`.

Il campo `strict` decide se la voce può aggiungere componenti a un plugin che ha il suo `plugin.json`. Predefinito a `true`.

<h3 id="how-entry-fields-combine-with-plugin-json">
  How entry fields combine with `plugin.json`
</h3>

La voce serve come manifest, aggiunge componenti ad esso, o entra in conflitto con esso:

* **No `plugin.json`**: la voce è il manifest, indipendentemente da `strict`. L'`hooks` della voce si carica solo nella forma di oggetto inline. Per un percorso di file o array lì, la scheda `/plugin` **Errors** mostra un errore `not yet supported in a marketplace entry`
* **`plugin.json` presente, `strict` non impostato o `true`**: Claude Code carica il manifest e aggiunge i `commands`, `agents`, `skills`, `outputStyles` e `themes` della voce ad esso. Per `hooks`, i matcher della voce per un evento sostituiscono i matcher del manifest per lo stesso evento, e gli eventi che solo il manifest dichiara mantengono i loro
* **`plugin.json` presente, `strict: false`**: una voce che dichiara qualsiasi di `commands`, `agents`, `skills`, `hooks`, `outputStyles` o `themes` è un conflitto, e il plugin non si carica con `Plugin <name> has conflicting manifests`

Quando una [voce di marketplace la cui `source` è la radice del marketplace](/docs/it/plugins/marketplace-reference) elenca sottodirectory `skills` specifiche, solo quelle sottodirectory si caricano, e la directory predefinita `skills/` del plugin non viene scansionata. Una chiave `skills` nel manifest invece [aggiunge al predefinito](#how-each-key-combines-with-its-default-location).

<h3 id="metadata-precedence">
  Metadata precedence
</h3>

Alcuni campi di metadati hanno una precedenza fissa indipendentemente da `strict`:

* **`defaultEnabled` e campi di visualizzazione**: il `defaultEnabled` della voce e i suoi [campi di visualizzazione](/docs/it/plugins/marketplace-reference#entry-and-plugin-json) come `displayName` sostituiscono quelli del manifest
* **`version`**: il `version` del manifest sostituisce quello della voce
* **`name`**: quando la voce elenca il plugin sotto un `name` diverso da quello del manifest, `enabledPlugins` usa il nome della voce, e i componenti sono namespacciati sotto il nome del manifest

Per la tabella di precedenza completa, vedere [Strict mode](/docs/it/plugins/marketplace-reference).

<h2 id="next-steps">
  Next steps
</h2>

* [Add components to a plugin](/docs/it/plugins/components): cosa fa ogni componente al runtime, con un esempio che passa la validazione
* [Marketplace reference](/docs/it/plugins/marketplace-reference): i campi della voce che un marketplace può impostare per il vostro plugin
* [Plugin commands reference](/docs/it/plugins/cli-reference#plugin-validate): flag e output di `claude plugin validate`
* [Troubleshoot plugins](/docs/it/plugins/troubleshooting#claude-plugin-validate-reports-errors): ogni messaggio di validazione con la sua correzione
