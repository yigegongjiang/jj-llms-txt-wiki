> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Creare e distribuire un marketplace di plugin

> Crea e ospita marketplace di plugin per distribuire estensioni Claude Code tra team e comunità.

Un **plugin marketplace** è un catalogo che ti consente di distribuire plugin ad altri. I marketplace forniscono scoperta centralizzata, tracciamento delle versioni, aggiornamenti automatici e supporto per più tipi di fonte, inclusi repository git e percorsi locali. Questa guida ti mostra come creare il tuo marketplace per condividere plugin con il tuo team o comunità.

Stai cercando di installare plugin da un marketplace esistente? Vedi [Scopri e installa plugin precostruiti](/docs/it/discover-plugins).

<h2 id="overview">
  Panoramica
</h2>

La creazione e la distribuzione di un marketplace comporta:

1. **Creazione di plugin**: crea uno o più plugin con skills, agenti, hooks, server MCP o server LSP. Questa guida presuppone che tu abbia già plugin da distribuire; vedi [Crea plugin](/docs/it/plugins) per i dettagli su come crearli.
2. **Creazione di un file marketplace**: definisci un `marketplace.json` che elenca i tuoi plugin e dove trovarli. Vedi [Crea il file marketplace](#create-the-marketplace-file).
3. **Ospita il marketplace**: esegui il push su GitHub, GitLab o un altro host git. Vedi [Ospita e distribuisci marketplace](#host-and-distribute-marketplaces).
4. **Condividi con gli utenti**: gli utenti aggiungono il tuo marketplace con `/plugin marketplace add` e installano singoli plugin. Vedi [Scopri e installa plugin](/docs/it/discover-plugins).

Una volta che il tuo marketplace è attivo, puoi aggiornarlo eseguendo il push delle modifiche al tuo repository. Gli utenti aggiornano la loro copia locale con `/plugin marketplace update`.

<h2 id="walkthrough-create-a-local-marketplace">
  Procedura dettagliata: creare un marketplace locale
</h2>

Questo esempio crea un marketplace con un plugin: una skill `quality-review` per le revisioni del codice. Creerai la struttura delle directory, aggiungerai una skill, creerai il manifest del plugin e il catalogo del marketplace, quindi lo installerai e lo testerai.

<Steps>
  <Step title="Crea la struttura delle directory">
    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/skills/quality-review
    ```
  </Step>

  <Step title="Crea la skill">
    Crea un file `SKILL.md` che definisce cosa fa la skill `quality-review`.

    ```markdown my-marketplace/plugins/quality-review-plugin/skills/quality-review/SKILL.md theme={null}
    ---
    description: Rivedi il codice per bug, sicurezza e prestazioni
    ---

    Rivedi il codice che ho selezionato o i cambiamenti recenti per:
    - Potenziali bug o casi limite
    - Problemi di sicurezza
    - Problemi di prestazioni
    - Miglioramenti di leggibilità

    Sii conciso e pratico.
    ```
  </Step>

  <Step title="Crea il manifest del plugin">
    Crea un file `plugin.json` che descrive il plugin. Il manifest va nella directory `.claude-plugin/`.

    ```json my-marketplace/plugins/quality-review-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "quality-review-plugin",
      "description": "Aggiunge una skill quality-review per revisioni rapide del codice",
      "version": "1.0.0"
    }
    ```

    <Note>
      Impostare `version` significa che gli utenti ricevono aggiornamenti solo quando modifichi questo campo, quindi incrementalo ad ogni rilascio. Se ometti `version` e ospiti questo marketplace in git, ogni commit conta automaticamente come una nuova versione. Vedi [Risoluzione della versione](#version-resolution-and-release-channels) per scegliere l'approccio giusto.
    </Note>
  </Step>

  <Step title="Crea il file marketplace">
    Crea il catalogo marketplace che elenca il tuo plugin.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-plugins",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "quality-review-plugin",
          "source": "./plugins/quality-review-plugin",
          "description": "Aggiunge una skill quality-review per revisioni rapide del codice"
        }
      ]
    }
    ```
  </Step>

  <Step title="Aggiungi e installa">
    Aggiungi il marketplace e installa il plugin.

    ```shell theme={null}
    /plugin marketplace add ./my-marketplace
    /plugin install quality-review-plugin@my-plugins
    ```
  </Step>

  <Step title="Provalo">
    Seleziona del codice nel tuo editor ed esegui la tua nuova skill. Le skill dei plugin sono associate allo spazio dei nomi del nome del plugin.

    ```shell theme={null}
    /quality-review-plugin:quality-review
    ```
  </Step>
</Steps>

Per saperne di più su cosa possono fare i plugin, inclusi hooks, agenti, server MCP e server LSP, vedi [Plugins](/docs/it/plugins).

<Note>
  **Come vengono installati i plugin**: Quando gli utenti installano un plugin, Claude Code copia la directory del plugin in una posizione cache. Ciò significa che i plugin non possono fare riferimento a file al di fuori della loro directory utilizzando percorsi come `../shared-utils`, perché quei file non verranno copiati.

  Se hai bisogno di condividere file tra plugin, usa symlink. Vedi [Plugin caching and file resolution](/docs/it/plugins-reference#plugin-caching-and-file-resolution) per i dettagli.
</Note>

<h2 id="create-the-marketplace-file">
  Crea il file marketplace
</h2>

Crea `.claude-plugin/marketplace.json` nella radice del tuo repository. Questo file definisce il nome del tuo marketplace, le informazioni del proprietario e un elenco di plugin con le loro fonti.

Ogni voce di plugin ha bisogno almeno di un `name` e di una `source` che dice a Claude Code da dove recuperarla. Vedi lo [schema completo](#marketplace-schema) di seguito per tutti i campi disponibili.

```json theme={null}
{
  "name": "company-tools",
  "owner": {
    "name": "DevTools Team",
    "email": "devtools@example.com"
  },
  "plugins": [
    {
      "name": "code-formatter",
      "source": "./plugins/formatter",
      "description": "Automatic code formatting on save",
      "version": "2.1.0",
      "author": {
        "name": "DevTools Team"
      }
    },
    {
      "name": "deployment-tools",
      "source": {
        "source": "github",
        "repo": "company/deploy-plugin"
      },
      "description": "Deployment automation tools"
    }
  ]
}
```

<h2 id="marketplace-schema">
  Schema del marketplace
</h2>

<h3 id="required-fields">
  Campi obbligatori
</h3>

| Campo     | Tipo   | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Esempio         |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------- |
| `name`    | string | Identificatore del marketplace (kebab-case, senza spazi). Questo è pubblico: gli utenti lo vedono quando installano plugin (ad esempio, `/plugin install my-tool@your-marketplace`). Ogni utente può registrare un solo marketplace per nome: aggiungere un secondo marketplace con lo stesso nome sostituisce il primo. Per pubblicare più plugin sotto un nome di marketplace, elencarli tutti in un singolo [`marketplace.json`](#create-the-marketplace-file). | `"acme-tools"`  |
| `owner`   | object | Informazioni sul manutentore del marketplace ([vedi i campi di seguito](#owner-fields))                                                                                                                                                                                                                                                                                                                                                                            |                 |
| `plugins` | array  | Elenco dei plugin disponibili                                                                                                                                                                                                                                                                                                                                                                                                                                      | Vedi di seguito |

<Note>
  **Nomi riservati**: i seguenti nomi di marketplace sono riservati per uso ufficiale di Anthropic e non possono essere utilizzati da marketplace di terze parti: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `claude-plugins-community`, `claude-community`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `knowledge-work-plugins`, `life-sciences`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `healthcare`. Anche i nomi che impersonano marketplace ufficiali, come `official-claude-plugins` o `anthropic-plugins-v2`, sono bloccati. La riserva di questi nomi impedisce a un marketplace di terze parti di presentarsi come fonte pubblicata da Anthropic.

  Claude Code ricontrolla i nomi riservati ogni volta che carica un marketplace, non solo quando ne aggiungi uno. Un marketplace registrato con uno di questi nomi prima che il nome diventasse riservato smette di caricarsi e segnala che è [registrato da una fonte non attendibile](/docs/it/errors#marketplace-is-registered-from-an-untrusted-source). Rimuovi quel marketplace e aggiungilo di nuovo dalla fonte ufficiale di Anthropic. Un marketplace di terze parti interessato da un nome appena riservato si carica di nuovo non appena lo aggiungi di nuovo con un nome diverso. Prima della v2.1.205, `first-party-plugins` e `healthcare` non erano riservati, e un marketplace già registrato con un nome riservato continuava a caricarsi.
</Note>

<h3 id="owner-fields">
  Campi del proprietario
</h3>

| Campo   | Tipo   | Obbligatorio | Descrizione                          |
| :------ | :----- | :----------- | :----------------------------------- |
| `name`  | string | Sì           | Nome del manutentore o del team      |
| `email` | string | No           | Email di contatto per il manutentore |

<h3 id="optional-fields">
  Campi opzionali
</h3>

| Campo                                 | Tipo   | Descrizione                                                                                                                                                                                                                                                                                                                        |
| :------------------------------------ | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                             | string | URL dello schema JSON per l'autocompletamento dell'editor e la convalida. Claude Code ignora questo campo al momento del caricamento.                                                                                                                                                                                              |
| `description`                         | string | Breve descrizione del marketplace                                                                                                                                                                                                                                                                                                  |
| `version`                             | string | Versione del manifest del marketplace                                                                                                                                                                                                                                                                                              |
| `metadata.pluginRoot`                 | string | Directory di base anteposta ai percorsi di fonte del plugin relativo (ad esempio, `"./plugins"` ti consente di scrivere `"source": "formatter"` invece di `"source": "./plugins/formatter"`)                                                                                                                                       |
| `allowCrossMarketplaceDependenciesOn` | array  | Altri marketplace su cui i plugin in questo marketplace possono dipendere. Le dipendenze da un marketplace non elencato qui sono bloccate all'installazione. Vedi [Dipendi da un plugin di un altro marketplace](/docs/it/plugin-dependencies#depend-on-a-plugin-from-another-marketplace).                                             |
| `renames`                             | object | Mappa da un precedente `name` del plugin al suo nome attuale, o a `null` se il plugin è stato rimosso. Consente agli utenti esistenti di migrare automaticamente quando rinomini o rimuovi una voce in `plugins`. Vedi [Rinominare o rimuovere un plugin](#rename-or-remove-a-plugin). Richiede Claude Code v2.1.193 o successivo. |

`description` e `version` sono accettati anche sotto `metadata` per compatibilità con le versioni precedenti.

<h2 id="plugin-entries">
  Voci di plugin
</h2>

Ogni voce di plugin nell'array `plugins` descrive un plugin e dove trovarlo. Puoi includere qualsiasi campo dallo [schema del manifest del plugin](/docs/it/plugins-reference#plugin-manifest-schema), come `description`, `version`, `author`, `commands` e `hooks`, più questi campi specifici del marketplace: `source`, `category`, `tags`, `strict` e `relevance`.

<h3 id="required-fields-2">
  Campi obbligatori
</h3>

| Campo    | Tipo           | Descrizione                                                                                                                                                           |
| :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`   | string         | Identificatore del plugin (kebab-case, senza spazi). Questo è pubblico: gli utenti lo vedono quando installano (ad esempio, `/plugin install my-plugin@marketplace`). |
| `source` | string\|object | Da dove recuperare il plugin (vedi [Plugin sources](#plugin-sources) di seguito)                                                                                      |

<h3 id="optional-plugin-fields">
  Campi di plugin opzionali
</h3>

**Campi di metadati standard:**

| Campo            | Tipo    | Descrizione                                                                                                                                                                                                                                                                                                                                             |
| :--------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `displayName`    | string  | Nome leggibile mostrato nelle superfici dell'interfaccia utente. Ritorna a `name` quando omesso. Può contenere spazi e qualsiasi maiuscola/minuscola. Non utilizzato per il namespacing o la ricerca. Richiede Claude Code v2.1.143 o successivo.                                                                                                       |
| `description`    | string  | Breve descrizione del plugin                                                                                                                                                                                                                                                                                                                            |
| `version`        | string  | Versione del plugin. Se impostato (qui o in `plugin.json`), il plugin è bloccato a questa stringa e gli utenti ricevono aggiornamenti solo quando cambia. Ometti per tornare allo SHA del commit git. Vedi [Version resolution](#version-resolution-and-release-channels).                                                                              |
| `author`         | object  | Informazioni sull'autore del plugin (`name` obbligatorio, `email` opzionale)                                                                                                                                                                                                                                                                            |
| `homepage`       | string  | URL della homepage o della documentazione del plugin                                                                                                                                                                                                                                                                                                    |
| `repository`     | string  | URL del repository del codice sorgente                                                                                                                                                                                                                                                                                                                  |
| `license`        | string  | Identificatore di licenza SPDX (ad esempio, MIT, Apache-2.0)                                                                                                                                                                                                                                                                                            |
| `keywords`       | array   | Tag per la scoperta e la categorizzazione dei plugin                                                                                                                                                                                                                                                                                                    |
| `category`       | string  | Categoria del plugin per l'organizzazione                                                                                                                                                                                                                                                                                                               |
| `tags`           | array   | Tag per la ricercabilità                                                                                                                                                                                                                                                                                                                                |
| `strict`         | boolean | Controlla se `plugin.json` è l'autorità per le definizioni dei componenti (predefinito: true). Vedi [Strict mode](#strict-mode) di seguito.                                                                                                                                                                                                             |
| `relevance`      | object  | Segnali che indicano a Claude Code quando suggerire questo plugin agli utenti. Ha effetto solo per i marketplace che un amministratore consente nelle impostazioni gestite. Vedi [Recommend plugins for your org](/docs/it/plugin-relevance). Richiede Claude Code v2.1.152 o successivo.                                                                    |
| `defaultEnabled` | boolean | Se il plugin è abilitato dopo l'installazione (predefinito: true). Impostare su `false` per installare il plugin disabilitato fino a quando l'utente non acconsente. Ha la precedenza sullo stesso campo nel `plugin.json` del plugin. Vedi [Default enablement](/docs/it/plugins-reference#default-enablement). Richiede Claude Code v2.1.154 o successivo. |

**Campi di configurazione dei componenti:**

| Campo        | Tipo           | Descrizione                                                                     |
| :----------- | :------------- | :------------------------------------------------------------------------------ |
| `skills`     | string\|array  | Percorsi personalizzati alle directory delle skill contenenti `<name>/SKILL.md` |
| `commands`   | string\|array  | Percorsi personalizzati ai file di skill flat `.md` o alle directory            |
| `agents`     | string\|array  | Percorsi personalizzati ai file degli agenti                                    |
| `hooks`      | string\|object | Configurazione degli hook personalizzati o percorso al file degli hook          |
| `mcpServers` | string\|object | Configurazioni del server MCP o percorso alla configurazione MCP                |
| `lspServers` | string\|object | Configurazioni del server LSP o percorso alla configurazione LSP                |

<h2 id="plugin-sources">
  Plugin sources
</h2>

Le plugin sources indicano a Claude Code dove recuperare ogni singolo plugin elencato nel tuo marketplace. Questi sono impostati nel campo `source` di ogni voce di plugin in `marketplace.json`.

Dopo che Claude Code clona o scarica un plugin sulla macchina locale, lo copia nella cache del plugin locale con versione in `~/.claude/plugins/cache`.

| Source            | Tipo                              | Campi                              | Note                                                                                                                                                                         |
| ----------------- | --------------------------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Percorso relativo | `string` (ad es. `"./my-plugin"`) | nessuno                            | Directory locale all'interno del repository del marketplace. Deve iniziare con `./`. Risolto relativamente alla radice del marketplace, non alla directory `.claude-plugin/` |
| `github`          | object                            | `repo`, `ref?`, `sha?`             |                                                                                                                                                                              |
| `url`             | object                            | `url`, `ref?`, `sha?`              | Fonte URL Git                                                                                                                                                                |
| `git-subdir`      | object                            | `url`, `path`, `ref?`, `sha?`      | Sottodirectory all'interno di un repository git. Clona in modo sparso per ridurre al minimo la larghezza di banda per i monorepo                                             |
| `npm`             | object                            | `package`, `version?`, `registry?` | Installato tramite `npm install`                                                                                                                                             |

<Note>
  **Marketplace sources vs plugin sources**: Questi sono concetti diversi che controllano cose diverse.

  * **Marketplace source**: dove recuperare il catalogo `marketplace.json` stesso. Impostato quando gli utenti eseguono `/plugin marketplace add` o nelle impostazioni `extraKnownMarketplaces`. Supporta `ref` (branch/tag) ma non `sha`.
  * **Plugin source**: dove recuperare un singolo plugin elencato nel marketplace. Impostato nel campo `source` di ogni voce di plugin all'interno di `marketplace.json`. Supporta sia `ref` (branch/tag) che `sha` (commit esatto).

  Ad esempio, un marketplace ospitato in `acme-corp/plugin-catalog` (marketplace source) può elencare un plugin recuperato da `acme-corp/code-formatter` (plugin source). La marketplace source e la plugin source puntano a repository diversi e sono fissate indipendentemente.
</Note>

I tipi di source basati su git di seguito sono `github`, `url` e `git-subdir`. Quando sia `ref` che `sha` sono impostati su uno qualsiasi di essi, `sha` è il pin effettivo. Claude Code recupera e controlla il commit fissato direttamente.

Su la maggior parte degli host git, inclusi GitHub, GitLab e Bitbucket, questo significa che l'installazione ha successo anche se il branch o il tag denominato da `ref` è stato eliminato a monte, purché il commit sia ancora raggiungibile dal repository. Alcuni server, come AWS CodeCommit, non supportano il recupero dei commit per SHA. Su quei server il `ref` deve ancora esistere e il commit fissato deve essere raggiungibile da esso.

<h3 id="relative-paths">
  Percorsi relativi
</h3>

Per i plugin nello stesso repository, usa un percorso che inizia con `./`:

```json theme={null}
{
  "name": "my-plugin",
  "source": "./plugins/my-plugin"
}
```

I percorsi si risolvono relativamente alla radice del marketplace, che è la directory contenente `.claude-plugin/`. Nell'esempio sopra, `./plugins/my-plugin` punta a `<repo>/plugins/my-plugin`, anche se `marketplace.json` si trova in `<repo>/.claude-plugin/marketplace.json`. Non usare `../` per fare riferimento a percorsi al di fuori della radice del marketplace.

<Note>
  I percorsi relativi si risolvono rispetto a una copia locale del marketplace, quindi funzionano quando gli utenti aggiungono il tuo marketplace da una fonte git o da una directory locale. Se gli utenti aggiungono il tuo marketplace tramite un URL diretto al file `marketplace.json`, i percorsi relativi non si risolveranno, perché viene scaricato solo quel file. Per la distribuzione basata su URL, usa invece GitHub, npm o fonti URL git. Vedi [Troubleshooting](#plugins-with-relative-paths-fail-in-url-based-marketplaces) per i dettagli.
</Note>

<h3 id="github-repositories">
  Repository GitHub
</h3>

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo"
  }
}
```

Puoi fissare a un branch, tag o commit specifico:

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Campo  | Tipo   | Descrizione                                                                             |
| :----- | :----- | :-------------------------------------------------------------------------------------- |
| `repo` | string | Obbligatorio. Repository GitHub nel formato `owner/repo`                                |
| `ref`  | string | Opzionale. Branch o tag Git (predefinito al branch predefinito del repository)          |
| `sha`  | string | Opzionale. SHA del commit git completo a 40 caratteri per fissare a una versione esatta |

<h3 id="git-repositories">
  Repository Git
</h3>

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git"
  }
}
```

Puoi fissare a un branch, tag o commit specifico:

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git",
    "ref": "main",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Campo | Tipo   | Descrizione                                                                                                                                                                       |
| :---- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url` | string | Obbligatorio. URL completo del repository git (`https://` o `git@`). Il suffisso `.git` è opzionale, quindi gli URL di Azure DevOps e AWS CodeCommit senza il suffisso funzionano |
| `ref` | string | Opzionale. Branch o tag Git (predefinito al branch predefinito del repository)                                                                                                    |
| `sha` | string | Opzionale. SHA del commit git completo a 40 caratteri per fissare a una versione esatta                                                                                           |

<h3 id="git-subdirectories">
  Sottodirectory Git
</h3>

Usa `git-subdir` per puntare a un plugin che si trova all'interno di una sottodirectory di un repository git. Claude Code utilizza un clone parziale e sparso per recuperare solo la sottodirectory, riducendo al minimo la larghezza di banda per i grandi monorepo.

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin"
  }
}
```

Puoi fissare a un branch, tag o commit specifico:

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

Il campo `url` accetta anche una scorciatoia GitHub (`owner/repo`) o URL SSH (`git@github.com:owner/repo.git`).

| Campo  | Tipo   | Descrizione                                                                                                                 |
| :----- | :----- | :-------------------------------------------------------------------------------------------------------------------------- |
| `url`  | string | Obbligatorio. URL del repository Git, scorciatoia GitHub `owner/repo` o URL SSH                                             |
| `path` | string | Obbligatorio. Percorso della sottodirectory all'interno del repo contenente il plugin (ad esempio, `"tools/claude-plugin"`) |
| `ref`  | string | Opzionale. Branch o tag Git (predefinito al branch predefinito del repository)                                              |
| `sha`  | string | Opzionale. SHA del commit git completo a 40 caratteri per fissare a una versione esatta                                     |

<h3 id="npm-packages">
  Pacchetti npm
</h3>

I plugin distribuiti come pacchetti npm vengono installati utilizzando `npm install`. Questo funziona con qualsiasi pacchetto nel registro npm pubblico o in un registro privato ospitato dal tuo team.

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin"
  }
}
```

Per fissare a una versione specifica, aggiungi il campo `version`:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "2.1.0"
  }
}
```

Per installare da un registro privato o interno, aggiungi il campo `registry`:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

| Campo      | Tipo   | Descrizione                                                                                                     |
| :--------- | :----- | :-------------------------------------------------------------------------------------------------------------- |
| `package`  | string | Obbligatorio. Nome del pacchetto o pacchetto con scope (ad esempio, `@org/plugin`)                              |
| `version`  | string | Opzionale. Versione o intervallo di versione (ad esempio, `2.1.0`, `^2.0.0`, `~1.5.0`)                          |
| `registry` | string | Opzionale. URL del registro npm personalizzato. Predefinito al registro npm del sistema (tipicamente npmjs.org) |

<h3 id="advanced-plugin-entries">
  Voci di plugin avanzate
</h3>

Questo esempio mostra una voce di plugin che utilizza molti dei campi opzionali, inclusi percorsi personalizzati per comandi, agenti, hooks e server MCP:

```json theme={null}
{
  "name": "enterprise-tools",
  "source": {
    "source": "github",
    "repo": "company/enterprise-plugin"
  },
  "description": "Strumenti di automazione del flusso di lavoro aziendale",
  "version": "2.1.0",
  "author": {
    "name": "Enterprise Team",
    "email": "enterprise@example.com"
  },
  "homepage": "https://docs.example.com/plugins/enterprise-tools",
  "repository": "https://github.com/company/enterprise-plugin",
  "license": "MIT",
  "keywords": ["enterprise", "workflow", "automation"],
  "category": "productivity",
  "commands": [
    "./commands/core/",
    "./commands/enterprise/",
    "./commands/experimental/preview.md"
  ],
  "agents": ["./agents/security-reviewer.md", "./agents/compliance-checker.md"],
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh"
          }
        ]
      }
    ]
  },
  "mcpServers": {
    "enterprise-db": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"]
    }
  },
  "strict": false
}
```

Cose importanti da notare:

* **`commands` e `agents`**: puoi specificare più directory o singoli file. I percorsi sono relativi alla radice del plugin.
* **`${CLAUDE_PLUGIN_ROOT}`**: Usa questa variabile nelle configurazioni degli hook e del server MCP per fare riferimento ai file all'interno della directory di installazione del plugin. Questo è necessario perché i plugin vengono copiati in una posizione cache quando installati.
  * Vedi la [tabella di sostituzione](/docs/it/plugins-reference#environment-variables) per quali campi di configurazione la sostituiscono per tipo di server
  * Per le dipendenze o lo stato che dovrebbe sopravvivere agli aggiornamenti dei plugin, usa [`${CLAUDE_PLUGIN_DATA}`](/docs/it/plugins-reference#persistent-data-directory) invece
* **`strict: false`**: poiché è impostato su false, il plugin non ha bisogno del suo `plugin.json`. La voce del marketplace definisce tutto. Vedi [Strict mode](#strict-mode) di seguito.

Per impostazione predefinita, le skills di un plugin vengono caricate dalla directory `skills/` sotto la sua `source`. I percorsi elencati nel campo `skills` si aggiungono a quella scansione:

```json theme={null}
"skills": ["./skills/", "./extra-skills/"]
```

Quando più voci di plugin condividono una cartella `skills/` alla radice del marketplace (`source: "./"`), elenca invece sottodirectory specifiche in modo che ogni voce carichi solo le sue skills:

```json theme={null}
"source": "./",
"skills": ["./skills/code-review", "./skills/docs"]
```

Con una source della radice del marketplace, i percorsi elencati sono il set completo per quella voce, e altre directory nella cartella `skills/` condivisa non vengono caricate. L'elenco di `./skills/` stesso, o della radice del plugin, mantiene la scansione completa. Se nessuno dei percorsi elencati esiste, viene eseguita la scansione predefinita.

<h3 id="strict-mode">
  Strict mode
</h3>

Il campo `strict` controlla se `plugin.json` è l'autorità per le definizioni dei componenti (skills, agenti, hooks, server MCP, stili di output).

| Valore               | Comportamento                                                                                                                                                         |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `true` (predefinito) | `plugin.json` è l'autorità. La voce del marketplace può integrarla con componenti aggiuntivi e entrambe le fonti vengono unite.                                       |
| `false`              | La voce del marketplace è la definizione completa. Se il plugin ha anche un `plugin.json` che dichiara componenti, è un conflitto e il plugin non riesce a caricarsi. |

**Quando usare ogni modalità:**

* **`strict: true`**: il plugin ha il suo `plugin.json` e gestisce i suoi componenti. La voce del marketplace può aggiungere skills o hooks extra in cima. Questo è il predefinito e funziona per la maggior parte dei plugin.
* **`strict: false`**: l'operatore del marketplace vuole il controllo completo. Il repository del plugin fornisce file grezzi e la voce del marketplace definisce quali di questi file sono esposti come skills, agenti, hooks, ecc. Utile quando il marketplace ristruttura o cura i componenti di un plugin diversamente da quanto previsto dall'autore del plugin.

<h2 id="host-and-distribute-marketplaces">
  Ospita e distribuisci marketplace
</h2>

<h3 id="host-on-github-recommended">
  Ospita su GitHub (consigliato)
</h3>

GitHub è il metodo consigliato per ospitare e distribuire un marketplace:

1. **Crea un repository**: configura un nuovo repository per il tuo marketplace
2. **Aggiungi il file marketplace**: crea `.claude-plugin/marketplace.json` con le tue definizioni di plugin
3. **Condividi con i team**: gli utenti aggiungono il tuo marketplace con `/plugin marketplace add owner/repo`

**Vantaggi**: controllo della versione integrato, tracciamento dei problemi e funzionalità di collaborazione del team.

<h3 id="host-on-other-git-services">
  Ospita su altri servizi git
</h3>

Qualsiasi servizio di hosting git funziona, come GitLab, Bitbucket e server self-hosted. Gli utenti aggiungono con l'URL completo del repository:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

<h3 id="private-repositories">
  Repository privati
</h3>

Claude Code supporta l'installazione di plugin da repository privati. Per l'installazione manuale e gli aggiornamenti, Claude Code utilizza i tuoi helper di credenziali git esistenti, quindi l'accesso HTTPS tramite `gh auth login`, Keychain di macOS o `git-credential-store` funziona come nel tuo terminale. L'accesso SSH funziona finché l'host è già nel tuo file `known_hosts` e la chiave è caricata in `ssh-agent`, poiché Claude Code sopprime i prompt SSH interattivi per l'impronta digitale dell'host e la passphrase della chiave. Gli shorthand `owner/repo` di GitHub clonano per impostazione predefinita su SSH; imposta [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/it/env-vars#variables) per clonare su HTTPS invece.

Gli aggiornamenti automatici in background funzionano diversamente. Per impostazione predefinita, l'aggiornamento in background disabilita gli helper di credenziali git per il suo `git pull`, quindi il pull non può autenticarsi ai repository privati su HTTPS anche quando un helper è configurato. I remote SSH non sono interessati: una chiave caricata in `ssh-agent` autentica i pull in background allo stesso modo delle operazioni manuali. Quando il pull in background non riesce, Claude Code ricade nel ri-clonare il marketplace da zero. Il ri-clone utilizza le tue credenziali git archiviate, ma può [scadere su repository di grandi dimensioni](#git-operations-time-out), quindi gli aggiornamenti automatici del marketplace privato possono non riuscire intermittentemente.

Due impostazioni rendono i marketplace privati comportarsi in modo prevedibile:

* Imposta `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` per mantenere il clone esistente quando il pull in background non riesce, invece di eliminare e ri-clonare. I tuoi plugin continuano a funzionare dallo stato sincronizzato più recente, e gli aggiornamenti manuali con `/plugin marketplace update` continuano a eseguire il pull con le tue credenziali.
* Configura un helper di credenziali git, ad esempio con `gh auth setup-git` per GitHub, in modo che il fallback di ri-clone possa autenticarsi senza richiedere.

Impostare un token del provider come `GITHUB_TOKEN` nel tuo ambiente non abilita di per sé l'autenticazione in background. I token hanno effetto solo attraverso un helper di credenziali configurato, ad esempio l'helper della CLI `gh`, che legge `GH_TOKEN` e `GITHUB_TOKEN`.

Per fare in modo che il pull in background stesso si autentichi su HTTPS, configura una riscrittura URL git globale. La riscrittura incorpora un token nell'URL remoto, quindi ha effetto anche se il pull in background disabilita gli helper di credenziali, e un pull riuscito salta il fallback di ri-clone. L'esempio seguente riscrive l'URL del repository del marketplace per includere un token di accesso:

```bash theme={null}
git config --global url."https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins".insteadOf "https://github.com/acme-corp/plugins"
```

Limita la riscrittura al repository del marketplace o al percorso dell'organizzazione. Una riscrittura la cui base è solo l'host si applica a ogni fetch e push a quell'host sulla macchina e sostituisce le tue credenziali normali, inclusi i push ai tuoi repository.

Ogni provider si aspetta un nome utente diverso nell'URL riscritto, e lo stesso scoping del percorso si applica a ogni provider. Per server self-hosted, sostituisci il nome host con il nome host del tuo server:

| Provider  | Forma URL riscritta                                               |
| :-------- | :---------------------------------------------------------------- |
| GitHub    | `https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins`  |
| GitLab    | `https://oauth2:YOUR_TOKEN@gitlab.com/acme-corp/plugins`          |
| Bitbucket | `https://x-token-auth:YOUR_TOKEN@bitbucket.org/acme-corp/plugins` |

La riscrittura archivia il token in testo semplice nel tuo gitconfig, quindi utilizza un token con accesso di sola lettura al repository del marketplace.

<Note>
  Negli ambienti CI/CD, configura un helper di credenziali git prima di installare plugin da repository privati. Su GitHub Actions, esporta un token con accesso in lettura al repository del marketplace come `GH_TOKEN`, quindi esegui `gh auth setup-git`. Il token del flusso di lavoro predefinito può accedere solo al repository del flusso di lavoro stesso, quindi un marketplace privato in un altro repository ha bisogno di un token di accesso personale o di un token dell'app. Una riscrittura URL globale configurata nella pipeline autentica anche il pull in background direttamente.
</Note>

<h3 id="test-locally-before-distribution">
  Testa localmente prima della distribuzione
</h3>

Testa il tuo marketplace localmente prima di condividerlo:

```shell theme={null}
/plugin marketplace add ./my-marketplace
/plugin install quality-review-plugin@my-plugins
```

Per l'intera gamma di comandi add (GitHub, URL Git, percorsi locali, URL remoti), vedi [Aggiungi marketplace](/docs/it/discover-plugins#add-marketplaces).

<h3 id="require-marketplaces-for-your-team">
  Richiedi marketplace per il tuo team
</h3>

Puoi configurare il tuo repository in modo che i membri del team vengano automaticamente invitati a installare il tuo marketplace quando fidano della cartella del progetto. Aggiungi il tuo marketplace a `.claude/settings.json`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "company-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Puoi anche specificare quali plugin devono essere abilitati per impostazione predefinita:

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@company-tools": true,
    "deployment-tools@company-tools": true
  }
}
```

Per le opzioni di configurazione complete, vedi [Plugin settings](/docs/it/settings#plugin-settings).

<Note>
  Se usi una fonte locale `directory` o `file` con un percorso relativo, il percorso si risolve rispetto al checkout principale del tuo repository. Quando esegui Claude Code da un git worktree, il percorso punta ancora al checkout principale, quindi tutti i worktree condividono la stessa posizione del marketplace. Lo stato del marketplace viene archiviato una volta per utente in `~/.claude/plugins/known_marketplaces.json`, non per progetto.
</Note>

<h3 id="pre-populate-plugins-for-containers">
  Pre-popola plugin per i container
</h3>

Per le immagini container e gli ambienti CI, puoi pre-popolare una directory di plugin al momento della compilazione in modo che Claude Code si avvii con marketplace e plugin già disponibili, senza clonare nulla al runtime. Imposta la variabile di ambiente `CLAUDE_CODE_PLUGIN_SEED_DIR` per puntare a questa directory.

Per stratificare più directory seed, separa i percorsi con `:` su Unix o `;` su Windows. Claude Code cerca ogni directory in ordine e utilizza il primo seed che contiene un determinato marketplace o cache di plugin.

La directory seed rispecchia la struttura di `~/.claude/plugins`:

```
$CLAUDE_CODE_PLUGIN_SEED_DIR/
  known_marketplaces.json
  marketplaces/<name>/...
  cache/<marketplace>/<plugin>/<version>/...
```

Per costruire una directory seed, esegui Claude Code una volta durante la compilazione dell'immagine, installa i plugin di cui hai bisogno, quindi copia la directory `~/.claude/plugins` risultante nella tua immagine e punta `CLAUDE_CODE_PLUGIN_SEED_DIR` ad essa.

Per saltare il passaggio di copia, imposta `CLAUDE_CODE_PLUGIN_CACHE_DIR` sul tuo percorso seed di destinazione durante la compilazione in modo che i plugin si installino direttamente lì:

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/plugins
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install my-tool@your-plugins
```

Quindi imposta `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` nell'ambiente di runtime del tuo container in modo che Claude Code legga dal seed all'avvio.

All'avvio, Claude Code registra i marketplace trovati nel `known_marketplaces.json` del seed nella configurazione primaria e utilizza le cache dei plugin trovate sotto `cache/` al posto senza ri-clonare. Questo funziona sia in modalità interattiva che in modalità non interattiva con il flag `-p`.

Dettagli del comportamento:

* **Sola lettura**: la directory seed non viene mai scritta. Gli aggiornamenti automatici sono disabilitati per i marketplace seed poiché git pull fallirebbe su un filesystem di sola lettura.
* **Le voci seed hanno precedenza**: i marketplace dichiarati nel seed sovrascrivono qualsiasi voce corrispondente nella configurazione dell'utente ad ogni avvio. Per rinunciare a un plugin seed, usa `/plugin disable` piuttosto che rimuovere il marketplace.
* **Risoluzione del percorso**: Claude Code individua il contenuto del marketplace sondando `$CLAUDE_CODE_PLUGIN_SEED_DIR/marketplaces/<name>/` al runtime, non fidandosi dei percorsi archiviati all'interno del JSON del seed. Ciò significa che il seed funziona correttamente anche quando montato in un percorso diverso da dove è stato compilato.
* **La mutazione è bloccata**: l'esecuzione di `/plugin marketplace remove` o `/plugin marketplace update` su un marketplace gestito da seed non riesce con una guida per chiedere al tuo amministratore di aggiornare l'immagine seed.
* **Compone con le impostazioni**: se `extraKnownMarketplaces` o `enabledPlugins` dichiarano un marketplace che esiste già nel seed, Claude Code utilizza la copia del seed invece di clonare.

<h3 id="managed-marketplace-restrictions">
  Restrizioni del marketplace gestito
</h3>

Per le organizzazioni che richiedono un controllo rigoroso sulle fonti dei plugin, gli amministratori possono limitare quali marketplace di plugin gli utenti possono aggiungere utilizzando l'impostazione [`strictKnownMarketplaces`](/docs/it/settings#strictknownmarketplaces) nelle impostazioni gestite. Per anche rifiutare i flag CLI che caricano plugin, agenti e server MCP per una singola esecuzione, abbinalo a [`disableSideloadFlags`](/docs/it/settings#available-settings). Per consentire quali plugin dei marketplace possono apparire come suggerimenti di installazione contestuali, imposta [`pluginSuggestionMarketplaces`](/docs/it/settings#available-settings).

Quando `strictKnownMarketplaces` è configurato nelle impostazioni gestite, il comportamento della restrizione dipende dal valore:

| Valore                     | Comportamento                                                                                             |
| -------------------------- | --------------------------------------------------------------------------------------------------------- |
| Non definito (predefinito) | Nessuna restrizione. Gli utenti possono aggiungere qualsiasi marketplace                                  |
| Array vuoto `[]`           | Blocco completo. Gli utenti non possono aggiungere nuovi marketplace                                      |
| Elenco di fonti            | Gli utenti possono aggiungere solo marketplace che corrispondono esattamente all'elenco di autorizzazione |

<h4 id="common-configurations">
  Configurazioni comuni
</h4>

Disabilita tutti gli aggiunte di marketplace:

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

Consenti solo marketplace specifici:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    }
  ]
}
```

Consenti tutti i marketplace da un server git interno utilizzando la corrispondenza del modello regex sull'host. Questo è l'approccio consigliato per [GitHub Enterprise Server](/docs/it/github-enterprise-server#plugin-marketplaces-on-ghes) o istanze GitLab self-hosted:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Consenti marketplace basati su filesystem da una directory specifica utilizzando la corrispondenza del modello regex sul percorso:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "pathPattern",
      "pathPattern": "^/opt/approved/"
    }
  ]
}
```

Usa `".*"` come `pathPattern` per consentire qualsiasi percorso del filesystem controllando comunque le fonti di rete con `hostPattern`.

<Note>
  `strictKnownMarketplaces` limita ciò che gli utenti possono aggiungere, ma non registra i marketplace di per sé. Per rendere i marketplace consentiti disponibili automaticamente senza che gli utenti eseguano `/plugin marketplace add`, abbinalo a [`extraKnownMarketplaces`](/docs/it/settings#extraknownmarketplaces) nello stesso `managed-settings.json`. Vedi [Usare entrambi insieme](/docs/it/settings#strictknownmarketplaces).
</Note>

<h4 id="how-restrictions-work">
  Come funzionano le restrizioni
</h4>

Le restrizioni vengono convalidate prima di qualsiasi operazione di rete o del filesystem. Il controllo viene eseguito all'aggiunta del marketplace e all'installazione, aggiornamento, aggiornamento e auto-aggiornamento del plugin. Se un marketplace è stato aggiunto prima della configurazione della policy e la sua fonte non corrisponde più all'elenco di autorizzazione, Claude Code rifiuta di installare o aggiornare i plugin da esso. Lo stesso controllo si applica a `blockedMarketplaces`.

L'elenco di autorizzazione utilizza la corrispondenza esatta per la maggior parte dei tipi di fonte. Affinché un marketplace sia consentito, tutti i campi specificati devono corrispondere esattamente:

* Per le fonti GitHub: `repo` è obbligatorio e `ref` o `path` devono corrispondere anche se specificati nell'elenco di autorizzazione
* Per le fonti URL: l'URL completo deve corrispondere esattamente
* Per le fonti `hostPattern`: l'host del marketplace viene confrontato con il modello regex
* Per le fonti `pathPattern`: il percorso del filesystem del marketplace viene confrontato con il modello regex

La corrispondenza esatta non normalizza gli URL: una barra finale, il suffisso `.git` o il modulo `ssh://` rispetto a `https://` vengono trattati come valori diversi. Se il marketplace della tua organizzazione può essere clonato da più di una forma di URL, preferisci una voce `hostPattern` rispetto a un URL letterale in modo che tutte le forme corrispondano.

Poiché `strictKnownMarketplaces` è impostato nelle [impostazioni gestite](/docs/it/settings#settings-files), le configurazioni individuali degli utenti e dei progetti non possono ignorare queste restrizioni.

Per i dettagli di configurazione completi inclusi tutti i tipi di fonte supportati e il confronto con `extraKnownMarketplaces`, vedi il [riferimento strictKnownMarketplaces](/docs/it/settings#strictknownmarketplaces).

<h3 id="version-resolution-and-release-channels">
  Risoluzione della versione e canali di rilascio
</h3>

Le versioni dei plugin determinano i percorsi della cache e il rilevamento degli aggiornamenti: se la versione risolta corrisponde a quella che un utente ha già, `/plugin update` e l'auto-aggiornamento saltano il plugin.

Claude Code risolve la versione di un plugin dal primo di questi che è impostato:

1. `version` nel `plugin.json` del plugin
2. `version` nella voce del marketplace del plugin
3. Lo SHA del commit git della fonte del plugin

Per i tipi di fonte basati su git `github`, `url`, `git-subdir` e i percorsi relativi all'interno di un marketplace ospitato su git, puoi omettere completamente `version` e ogni nuovo commit viene trattato come una nuova versione. Questa è la configurazione più semplice per i plugin interni o in fase di sviluppo attivo.

<Warning>
  Impostare `version` fissa il plugin. Se `plugin.json` dichiara `"version": "1.0.0"`, spingere nuovi commit senza cambiare quella stringa non fa nulla per gli utenti esistenti, perché Claude Code vede la stessa versione e mantiene la copia in cache. Aumenta il campo ad ogni rilascio, o omettilo per usare lo SHA del commit.

  Evita di impostare `version` sia in `plugin.json` che nella voce del marketplace. Il valore `plugin.json` vince sempre silenziosamente, quindi una versione del manifest obsoleta può mascherare una versione che hai impostato in `marketplace.json`.
</Warning>

<h4 id="set-up-release-channels">
  Configura i canali di rilascio
</h4>

Per supportare i canali di rilascio "stable" e "latest" per i tuoi plugin, puoi configurare due marketplace che puntano a diversi ref o SHA dello stesso repo. Puoi quindi assegnare i due marketplace a diversi gruppi di utenti tramite [impostazioni gestite](/docs/it/settings#settings-files).

<Warning>
  Ogni canale deve risolversi a una versione diversa. Se usi versioni esplicite, `plugin.json` deve dichiarare una `version` diversa in ogni ref fissato. Se ometti `version`, gli SHA dei commit distinti già distinguono i canali. Se due ref si risolvono alla stessa stringa di versione, Claude Code li tratta come identici e salta l'aggiornamento.
</Warning>

<h5 id="example">
  Esempio
</h5>

```json theme={null}
{
  "name": "stable-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "stable"
      }
    }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "latest"
      }
    }
  ]
}
```

<h5 id="assign-channels-to-user-groups">
  Assegna i canali ai gruppi di utenti
</h5>

Assegna ogni marketplace al gruppo di utenti appropriato tramite impostazioni gestite. Ad esempio, il gruppo stabile riceve:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/stable-tools"
      }
    }
  }
}
```

Il gruppo early-access riceve invece `latest-tools`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "latest-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/latest-tools"
      }
    }
  }
}
```

<h4 id="pin-dependency-versions">
  Fissa le versioni delle dipendenze
</h4>

Un plugin può vincolare le sue dipendenze a un intervallo semver in modo che gli aggiornamenti a una dipendenza non interrompano il plugin dipendente. Vedi [Vincola le versioni delle dipendenze dei plugin](/docs/it/plugin-dependencies) per la convenzione del tag git `{plugin-name}--v{version}`, la sintassi dell'intervallo e come più vincoli sulla stessa dipendenza vengono combinati.

<h3 id="rename-or-remove-a-plugin">
  Rinomina o rimuovi un plugin
</h3>

Il `name` di un plugin è il suo identificatore stabile. Gli utenti lo referenziano in `enabledPlugins`, `pluginConfigs` e comandi `/plugin install`, quindi cambiarlo interrompe ogni installazione esistente. Per cambiare l'etichetta mostrata nell'interfaccia utente senza interrompere le installazioni, imposta [`displayName`](#optional-plugin-fields) e mantieni `name` invariato.

Se devi cambiare il `name` di un plugin, o rimuovi un plugin dall'array `plugins`, aggiungi una voce `renames` di livello superiore in modo che gli utenti esistenti migrino invece di vedere un errore `plugin-not-found`. La migrazione automatica richiede Claude Code v2.1.193 o successivo. Mappa ogni nome precedente al suo nome attuale, o a `null` se il plugin non esiste più. L'esempio seguente rinomina `formatter` a `code-formatter` e registra che `legacy-linter` è stato rimosso:

```json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Quando un utente avvia Claude Code con il vecchio nome ancora nelle sue impostazioni, Claude Code segue la mappa `renames`:

* Se la voce punta a un nuovo nome, Claude Code carica il plugin con il suo nuovo nome e mostra un avviso di una riga come `Renamed to "code-formatter" in the "acme-tools" marketplace`. Quindi riscrive la vecchia chiave alla nuova chiave negli ambiti di impostazioni utente, progetto e locale sia per `enabledPlugins` che per `pluginConfigs`, in modo che l'avviso appaia una volta.
* Per una voce `null`, Claude Code elimina la vecchia chiave e l'avviso segnala che il plugin è stato rimosso dal marketplace.
* Se il plugin rinominato utilizza una fonte remota come `github` o `npm`, Claude Code segnala `plugin-cache-miss` dopo il rinomina e l'utente deve eseguire `/plugin install` una volta per recuperarlo con il nuovo nome.

Tratta `renames` come una cronologia di sola aggiunta: mantieni le vecchie voci al loro posto anche dopo che ti aspetti che ogni utente abbia migrato. Claude Code segue le catene, quindi se in seguito rinomini `code-formatter` a `formatter-pro`, aggiungi una seconda voce piuttosto che modificare la prima. Un utente che ha ancora l'originale `formatter` abilitato si risolve quindi attraverso entrambe le voci a `formatter-pro`.

Esegui `claude plugin validate .` dopo aver modificato la mappa; rifiuta qualsiasi voce la cui catena forma un ciclo o non termina a `null` o a un nome elencato in `plugins`.

<Note>
  Le impostazioni gestite e di policy sono di sola lettura per Claude Code, quindi i plugin abilitati lì non possono essere riscritti automaticamente. Il plugin rinominato si carica ancora ogni sessione, ma l'avviso di rinomina ricorre fino a quando un amministratore non aggiorna `enabledPlugins` nel file di impostazioni gestite per usare il nuovo nome. Lo stesso vale per i plugin abilitati attraverso altre fonti di sola lettura come `--add-dir`.
</Note>

Le versioni precedenti di Claude Code ignorano il campo `renames` e segnalano `plugin-not-found` per il vecchio nome.

<h2 id="validation-and-testing">
  Validazione e test
</h2>

Testa il tuo marketplace prima di condividerlo.

Valida la sintassi JSON del tuo marketplace:

```bash theme={null}
claude plugin validate .
```

O da Claude Code:

```shell theme={null}
/plugin validate .
```

Aggiungi il marketplace per il test:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace
```

Installa un plugin di test per verificare che tutto funzioni:

```shell theme={null}
/plugin install test-plugin@marketplace-name
```

Per i flussi di lavoro di test completi dei plugin, vedi [Testa i tuoi plugin localmente](/docs/it/plugins#test-your-plugins-locally). Per la risoluzione dei problemi tecnici, vedi [Plugins reference](/docs/it/plugins-reference).

<h2 id="manage-marketplaces-from-the-cli">
  Gestisci marketplace dalla CLI
</h2>

Claude Code fornisce sottocomandi `claude plugin marketplace` non interattivi per lo scripting e l'automazione. Questi sono equivalenti ai comandi `/plugin marketplace` disponibili in una sessione interattiva.

<h3 id="plugin-marketplace-add">
  Plugin marketplace add
</h3>

Aggiungi un marketplace da un repository GitHub, URL git, URL remoto o percorso locale.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

**Argomenti:**

* `<source>`: Scorciatoia GitHub `owner/repo`, URL git, URL remoto a un file `marketplace.json` o percorso di directory locale. Per fissare a un branch o tag, aggiungi `@ref` alla scorciatoia GitHub o `#ref` a un URL git

Un URL deve includere il suo schema. A partire da Claude Code v2.1.196, un host digitato senza uno, come `gitlab.example.com/team/plugins`, viene rifiutato come una scorciatoia `owner/repo` non valida e l'errore ti dice di aggiungere `https://` o usare `./` per un percorso locale. Le versioni precedenti lo leggevano male come un percorso di repository GitHub e falliscono al momento del clone con un errore di GitHub non trovato.

**Opzioni:**

| Opzione               | Descrizione                                                                                                                                      | Predefinito |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- | :---------- |
| `--scope <scope>`     | Dove dichiarare il marketplace: `user`, `project` o `local`. Vedi [Plugin installation scopes](/docs/it/plugins-reference#plugin-installation-scopes) | `user`      |
| `--sparse <paths...>` | Limita il checkout a directory specifiche tramite git sparse-checkout. Utile per i monorepo                                                      |             |

Aggiungi un marketplace da GitHub utilizzando la scorciatoia `owner/repo`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins
```

Fissa a un branch o tag specifico con `@ref`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins@v2.0
```

Aggiungi da un URL git su un host non-GitHub:

```bash theme={null}
claude plugin marketplace add https://gitlab.example.com/team/plugins.git
```

Aggiungi da un URL remoto che serve il file `marketplace.json` direttamente:

```bash theme={null}
claude plugin marketplace add https://example.com/marketplace.json
```

Aggiungi da una directory locale per il test:

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

Dichiara il marketplace a livello di progetto in modo che sia condiviso con il tuo team tramite `.claude/settings.json`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins --scope project
```

Per un monorepo, limita il checkout alle directory che contengono il contenuto del plugin:

```bash theme={null}
claude plugin marketplace add acme-corp/monorepo --sparse .claude-plugin plugins
```

<h3 id="plugin-marketplace-list">
  Plugin marketplace list
</h3>

Elenca tutti i marketplace configurati.

```bash theme={null}
claude plugin marketplace list [options]
```

**Opzioni:**

| Opzione  | Descrizione      |
| :------- | :--------------- |
| `--json` | Output come JSON |

Con `--json`, ogni voce include `name`, `source` e campi specifici della fonte: `repo` per le fonti GitHub, `url` per le fonti git e URL, e `path` per le fonti locali. Le fonti GitHub e git includono anche un campo `ref` quando il marketplace è stato aggiunto con un branch o tag fissato.

<h3 id="plugin-marketplace-remove">
  Plugin marketplace remove
</h3>

Rimuovi un marketplace configurato. L'alias `rm` è accettato anche.

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

**Argomenti:**

* `<name>`: nome del marketplace da rimuovere, come mostrato da `claude plugin marketplace list`. Questo è il `name` da `marketplace.json`, non la fonte che hai passato a `add`

**Opzioni:**

| Opzione           | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Predefinito        |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------- |
| `--scope <scope>` | Limita la rimozione a un singolo ambito di impostazioni: `user`, `project` o `local`. Vedi [Plugin installation scopes](/docs/it/plugins-reference#plugin-installation-scopes). Se omesso, la dichiarazione viene rimossa da ogni ambito modificabile. Se fornito, solo la dichiarazione di quell'ambito viene rimossa; lo stato condiviso, la cache e i dati dei plugin installati vengono preservati quando il marketplace è ancora dichiarato in un altro ambito | (tutti gli ambiti) |

<Warning>
  La rimozione di un marketplace dal suo ultimo ambito rimanente disinstalla anche tutti i plugin che hai installato da esso. Per aggiornare un marketplace senza perdere i plugin installati, usa `claude plugin marketplace update` invece.
</Warning>

<h3 id="plugin-marketplace-update">
  Plugin marketplace update
</h3>

Aggiorna i marketplace dalle loro fonti per recuperare nuovi plugin e cambiamenti di versione. Un marketplace aggiunto con un branch o tag `ref` si aggiorna al commit più recente di quel ref, non al branch predefinito del repository.

```bash theme={null}
claude plugin marketplace update [name]
```

**Argomenti:**

* `[name]`: nome del marketplace da aggiornare, come mostrato da `claude plugin marketplace list`. Aggiorna tutti i marketplace se omesso

Sia `remove` che `update` non riescono quando eseguiti su un marketplace gestito da seed, che è di sola lettura. Quando si aggiornano tutti i marketplace, le voci gestite da seed vengono saltate e gli altri marketplace si aggiornano comunque. Per modificare i plugin forniti da seed, chiedi al tuo amministratore di aggiornare l'immagine seed. Vedi [Pre-popola plugin per i container](#pre-populate-plugins-for-containers).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="marketplace-not-loading">
  Marketplace non carica
</h3>

**Sintomi**: Non riesci ad aggiungere il marketplace o a vedere i plugin da esso

**Soluzioni**:

* Verifica che l'URL del marketplace sia accessibile
* Controlla che `.claude-plugin/marketplace.json` esista nel percorso specificato
* Assicurati che la sintassi JSON sia valida utilizzando `claude plugin validate` o `/plugin validate`. Per verificare il frontmatter di skill, agente e comando, esegui il comando su ogni directory di plugin
* Per i repository privati, conferma di avere i permessi di accesso

<h3 id="marketplace-validation-errors">
  Errori di validazione del marketplace
</h3>

Esegui `claude plugin validate .` o `/plugin validate .` dalla directory del tuo marketplace per verificare i problemi. Quando puntato a una directory del marketplace, il validatore controlla `marketplace.json` per errori di schema, nomi di plugin duplicati e traversal del percorso di origine. Per ogni voce il cui `source` è un percorso locale, valida anche il `plugin.json` di quel plugin e avvisa quando la `version` della voce non corrisponde a quella in `plugin.json`. I problemi trovati nel `plugin.json` di un plugin sono preceduti dall'indice della voce, nella forma `plugins[2] plugin.json →`.

A partire da Claude Code v2.1.196, il pass per voce include anche:

* plugin il cui `source` è `.`
* esecuzione quando `marketplace.json` è al di fuori di una directory `.claude-plugin`, risolvendo le origini rispetto alla directory del file stesso
* segnalazione dei problemi di ogni voce anche quando un'altra parte del file ha errori di schema

Le versioni precedenti saltano i plugin alla radice del marketplace e scendono solo da un `.claude-plugin/marketplace.json`.

Per validare il `plugin.json` di un singolo plugin e i suoi file di skill, agente, comando e hook, esegui il comando sulla directory del plugin stesso, ad esempio `claude plugin validate ./plugins/my-plugin`. Errori comuni:

| Errore                                            | Causa                                                 | Soluzione                                                                                                                                                    |
| :------------------------------------------------ | :---------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: .claude-plugin/marketplace.json` | Manifest mancante                                     | Crea `.claude-plugin/marketplace.json` con i campi obbligatori                                                                                               |
| `Invalid JSON syntax: Unexpected token...`        | Errore di sintassi JSON in marketplace.json           | Controlla le virgole mancanti, le virgole extra o le stringhe non quotate                                                                                    |
| `Duplicate plugin name "x" found in marketplace`  | Due plugin condividono lo stesso nome                 | Dai a ogni plugin un valore `name` univoco                                                                                                                   |
| `plugins[0].source: Path contains ".."`           | Il percorso di origine contiene `..`                  | Usa percorsi relativi alla radice del marketplace senza `..`. Vedi [Percorsi relativi](#relative-paths)                                                      |
| `YAML frontmatter failed to parse: ...`           | YAML non valido in un file di skill, agente o comando | Correggi la sintassi YAML nel blocco frontmatter. Al runtime questo file si carica senza metadati. Segnalato solo quando si valida una directory di plugin   |
| `Invalid JSON syntax: ...` (hooks.json)           | `hooks/hooks.json` malformato                         | Correggi la sintassi JSON. Un `hooks/hooks.json` malformato impedisce al plugin intero di caricarsi. Segnalato solo quando si valida una directory di plugin |

**Avvisi** (non bloccanti):

* `Marketplace has no plugins defined`: aggiungi almeno un plugin all'array `plugins`
* `No marketplace description provided`: aggiungi una `description` di livello superiore per aiutare gli utenti a comprendere il tuo marketplace
* `Plugin name "x" is not kebab-case`: il nome del plugin contiene lettere maiuscole, spazi o caratteri speciali. Rinomina in lettere minuscole, cifre e trattini solo (ad esempio, `my-plugin`). Claude Code accetta altre forme, ma la sincronizzazione del marketplace Claude.ai le rifiuta.

<h3 id="plugin-installation-failures">
  Errori di installazione del plugin
</h3>

**Sintomi**: Il marketplace appare ma l'installazione del plugin non riesce

**Soluzioni**:

* Verifica che gli URL di origine del plugin siano accessibili
* Controlla che le directory dei plugin contengano i file richiesti
* Per le origini GitHub, assicurati che i repository siano pubblici o che tu abbia accesso
* Testa manualmente le origini dei plugin clonando/scaricando
* Se l'origine fissa sia `ref` che `sha`, un ramo o un tag upstream eliminato non blocca l'installazione sulla maggior parte degli host git, inclusi GitHub, GitLab e Bitbucket. Su server che non supportano il recupero dei commit per SHA, come AWS CodeCommit, il `ref` deve ancora esistere e il commit fissato deve essere raggiungibile da esso. Se l'installazione continua a non riuscire, conferma che il commit fissato esiste ancora nel repository

<h3 id="private-repository-authentication-fails">
  L'autenticazione del repository privato non riesce
</h3>

**Sintomi**: Errori di autenticazione durante l'installazione di plugin da repository privati

**Soluzioni**:

Per l'installazione manuale e gli aggiornamenti:

* Verifica di essere autenticato con il tuo provider git (ad esempio, esegui `gh auth status` per GitHub)
* Controlla che il tuo helper di credenziali sia configurato correttamente: `git config --global credential.helper`
* Prova a clonare il repository manualmente per verificare che le tue credenziali funzionino

Per gli aggiornamenti automatici in background:

* Per impostazione predefinita, gli aggiornamenti in background disabilitano gli helper di credenziali git per il pull, quindi il pull non può autenticarsi su HTTPS. I remote SSH con una chiave caricata in `ssh-agent` si autenticano ancora. Un pull non riuscito attiva una ri-clonazione da zero, che utilizza le tue credenziali archiviate ma potrebbe scadere su repository di grandi dimensioni
* Imposta `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` per mantenere il clone esistente quando il pull in background non riesce
* Configura un helper di credenziali git, ad esempio `gh auth setup-git`, in modo che il fallback di ri-clonazione possa autenticarsi
* Se la ri-clonazione scade su un repository di grandi dimensioni, aumenta il limite con [`CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`](#git-operations-time-out)
* Configura una [riscrittura dell'URL git](#private-repositories) limitata al repository del marketplace in modo che il pull in background si autentichi direttamente
* Oppure aggiorna i marketplace privati manualmente con `/plugin marketplace update <name>`, che utilizza le tue credenziali

<h3 id="marketplace-updates-fail-in-offline-environments">
  Gli aggiornamenti del marketplace non riescono in ambienti offline
</h3>

**Sintomi**: Il `git pull` del marketplace non riesce in background e Claude Code tenta ripetutamente una ri-clonazione che non può avere successo.

**Causa**: Per impostazione predefinita, quando un `git pull` non riesce, Claude Code tenta una ri-clonazione da zero. In ambienti offline o airgapped, la ri-clonazione non riesce allo stesso modo, e il ripristino della cache precedente successivamente è best-effort. L'aggiornamento viene eseguito in background dopo l'avvio, quindi non ritarda l'avvio, ma ogni sessione ripete i tentativi non riusciti e ogni operazione git può attendere il [timeout di 120 secondi](#git-operations-time-out).

**Soluzione**: Imposta `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` per saltare il tentativo di ri-clonazione e continuare a utilizzare la cache esistente quando il pull non riesce:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
```

Con questa variabile impostata, Claude Code mantiene il clone obsoleto del marketplace in caso di fallimento di `git pull` e continua a utilizzare lo stato ultimo noto come buono. Per distribuzioni completamente offline in cui il repository non sarà mai raggiungibile, utilizza [`CLAUDE_CODE_PLUGIN_SEED_DIR`](#pre-populate-plugins-for-containers) per pre-popolare la directory dei plugin al momento della compilazione.

<h3 id="git-operations-time-out">
  Le operazioni Git scadono
</h3>

**Sintomi**: L'installazione del plugin o gli aggiornamenti del marketplace non riescono con un errore di timeout come "Git clone timed out after 120s" o "Git pull timed out after 120s".

**Causa**: Claude Code utilizza un timeout di 120 secondi per tutte le operazioni git, inclusa la clonazione dei repository dei plugin e il pull degli aggiornamenti del marketplace. I repository di grandi dimensioni o le connessioni di rete lente possono superare questo limite.

**Soluzione**: Aumenta il timeout utilizzando la variabile di ambiente `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`. Il valore è in millisecondi:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000  # 5 minuti
```

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  I plugin con percorsi relativi non riescono nei marketplace basati su URL
</h3>

**Sintomi**: Hai aggiunto un marketplace tramite URL (come `https://example.com/marketplace.json`), ma i plugin con origini di percorso relativo come `"./plugins/my-plugin"` non riescono a installare con errori "path not found".

**Causa**: I marketplace basati su URL scaricano solo il file `marketplace.json` stesso. Non scaricano i file dei plugin dal server. I percorsi relativi nella voce del marketplace fanno riferimento a file sul server remoto che non sono stati scaricati.

**Soluzioni**:

* **Usa origini esterne**: Cambia le voci dei plugin per usare GitHub, npm o origini URL git invece di percorsi relativi:
  ```json theme={null}
  { "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
  ```
* **Usa un marketplace basato su Git**: Ospita il tuo marketplace in un repository Git e aggiungilo con l'URL git. I marketplace basati su Git clonano l'intero repository, rendendo i percorsi relativi funzionanti correttamente.

<h3 id="files-not-found-after-installation">
  File non trovati dopo l'installazione
</h3>

**Sintomi**: Il plugin si installa ma i riferimenti ai file non riescono, specialmente i file al di fuori della directory del plugin

**Causa**: I plugin vengono copiati in una directory cache piuttosto che utilizzati in-place. I percorsi che fanno riferimento a file al di fuori della directory del plugin (come `../shared-utils`) non funzioneranno perché quei file non vengono copiati.

**Soluzioni**: Vedi [Plugin caching and file resolution](/docs/it/plugins-reference#plugin-caching-and-file-resolution) per le soluzioni alternative inclusi symlink e ristrutturazione delle directory.

Per ulteriori strumenti di debug e problemi comuni, vedi [Debugging and development tools](/docs/it/plugins-reference#debugging-and-development-tools).

<h2 id="see-also">
  Vedi anche
</h2>

* [Scopri e installa plugin precostruiti](/docs/it/discover-plugins) - Installazione di plugin da marketplace esistenti
* [Plugins](/docs/it/plugins) - Creazione dei tuoi plugin
* [Plugins reference](/docs/it/plugins-reference) - Specifiche tecniche complete e schemi
* [Plugin settings](/docs/it/settings#plugin-settings) - Opzioni di configurazione dei plugin
* [strictKnownMarketplaces reference](/docs/it/settings#strictknownmarketplaces) - Restrizioni del marketplace gestito
