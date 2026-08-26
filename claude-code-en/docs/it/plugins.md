> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Creare plugin

> Crea plugin personalizzati per estendere Claude Code con skills, agents, hooks e MCP servers.

I plugin ti permettono di estendere Claude Code con funzionalità personalizzate che possono essere condivise tra progetti e team. Questa guida copre la creazione dei tuoi plugin con skills, agents, hooks e MCP servers.

Stai cercando di installare plugin esistenti? Vedi [Scopri e installa plugin](/docs/it/discover-plugins). Per le specifiche tecniche complete, vedi [Riferimento plugin](/docs/it/plugins-reference).

<h2 id="when-to-use-plugins-vs-standalone-configuration">
  Quando usare plugin rispetto alla configurazione standalone
</h2>

Claude Code supporta due modi per aggiungere skills, agents e hooks personalizzati:

| Approccio                                                                                   | Nomi skill           | Migliore per                                                                                              |
| :------------------------------------------------------------------------------------------ | :------------------- | :-------------------------------------------------------------------------------------------------------- |
| **Standalone** (directory `.claude/`)                                                       | `/hello`             | Flussi di lavoro personali, personalizzazioni specifiche del progetto, esperimenti rapidi                 |
| **Plugin** (directory con skills, agents, hooks o un manifest `.claude-plugin/plugin.json`) | `/plugin-name:hello` | Condivisione con i colleghi, distribuzione alla comunità, rilasci versionati, riutilizzabili tra progetti |

**Usa la configurazione standalone quando**:

* Stai personalizzando Claude Code per un singolo progetto
* La configurazione è personale e non ha bisogno di essere condivisa
* Stai sperimentando con skills o hooks prima di pacchettizzarli
* Vuoi nomi skill brevi come `/hello` o `/deploy`

**Usa i plugin quando**:

* Vuoi condividere funzionalità con il tuo team o comunità
* Hai bisogno degli stessi skills/agents in più progetti
* Vuoi il controllo della versione e aggiornamenti facili per le tue estensioni
* Stai distribuendo tramite un marketplace
* Sei d'accordo con skills con namespace come `/my-plugin:hello` (il namespace previene conflitti tra plugin)

<Tip>
  Inizia con la configurazione standalone in `.claude/` per un'iterazione rapida, poi [converti in un plugin](#convert-existing-configurations-to-plugins) quando sei pronto a condividere.
</Tip>

<h2 id="quickstart">
  Quickstart
</h2>

Questo quickstart ti guida attraverso la creazione di un plugin con uno skill personalizzato. Creerai un manifest (il file di configurazione che definisce il tuo plugin), aggiungerai uno skill e lo testerai localmente usando il flag `--plugin-dir`.

<h3 id="prerequisites">
  Prerequisiti
</h3>

* Claude Code [installato e autenticato](/docs/it/quickstart#step-1-install-claude-code)

<Note>
  Se non vedi il comando `/plugin`, aggiorna Claude Code all'ultima versione. Vedi [Troubleshooting](/docs/it/troubleshooting) per le istruzioni di aggiornamento.
</Note>

<h3 id="create-your-first-plugin">
  Crea il tuo primo plugin
</h3>

<Steps>
  <Step title="Crea la directory del plugin">
    Ogni plugin vive nella sua directory contenente i tuoi skill, agent o hook, facoltativamente insieme a un manifest `.claude-plugin/plugin.json`. La posizione non importa per questo quickstart perché punterai Claude Code alla directory con `--plugin-dir` nel passaggio di test. Crealo ovunque sia conveniente, ad esempio in una cartella scratch o in una directory di progetti:

    ```bash theme={null}
    mkdir my-first-plugin
    ```

    I passaggi rimanenti vengono eseguiti dalla directory padre e fanno riferimento a percorsi come `my-first-plugin/...` relativi ad essa.
  </Step>

  <Step title="Crea il manifest del plugin">
    Il file manifest in `.claude-plugin/plugin.json` definisce l'identità del tuo plugin: il suo nome, descrizione e versione. Claude Code usa questi metadati per visualizzare il tuo plugin nel plugin manager.

    Crea la directory `.claude-plugin` dentro la cartella del tuo plugin:

    ```bash theme={null}
    mkdir my-first-plugin/.claude-plugin
    ```

    Poi crea `my-first-plugin/.claude-plugin/plugin.json` con questo contenuto:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    | Campo         | Scopo                                                                                                                                                                                                                                                                                                   |
    | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    | `name`        | Identificatore univoco e namespace dello skill. Gli skill sono prefissati con questo (ad es., `/my-first-plugin:hello`).                                                                                                                                                                                |
    | `description` | Mostrato nel plugin manager quando si sfogliano o si installano plugin.                                                                                                                                                                                                                                 |
    | `version`     | Opzionale. Se impostato, gli utenti ricevono aggiornamenti solo quando aumenti questo campo. Se omesso e il tuo plugin è distribuito tramite git, viene utilizzato il commit SHA e ogni commit conta come una nuova versione. Vedi [gestione della versione](/docs/it/plugins-reference#version-management). |
    | `author`      | Opzionale. Utile per l'attribuzione.                                                                                                                                                                                                                                                                    |

    Per campi aggiuntivi come `homepage`, `repository` e `license`, vedi lo [schema manifest completo](/docs/it/plugins-reference#plugin-manifest-schema).
  </Step>

  <Step title="Aggiungi uno skill">
    Gli skill vivono nella directory `skills/`. Ogni skill è una cartella contenente un file `SKILL.md`. Il nome della cartella diventa il nome dello skill, prefissato con il namespace del plugin (`hello/` in un plugin denominato `my-first-plugin` crea `/my-first-plugin:hello`).

    Crea una directory skill nella cartella del tuo plugin:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Poi crea `my-first-plugin/skills/hello/SKILL.md` con questo contenuto:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```
  </Step>

  <Step title="Testa il tuo plugin">
    Esegui Claude Code con il flag `--plugin-dir` per caricare il tuo plugin:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Una volta che Claude Code si avvia, prova il tuo nuovo skill:

    ```shell theme={null}
    /my-first-plugin:hello
    ```

    Vedrai Claude rispondere con un saluto. Esegui `/help` per vedere il tuo skill elencato sotto il namespace del plugin.

    <Note>
      **Perché il namespace?** Gli skill del plugin hanno sempre il namespace (come `/my-first-plugin:hello`) per prevenire conflitti quando più plugin hanno skill con lo stesso nome.

      Per cambiare il prefisso del namespace, aggiorna il campo `name` in `plugin.json`.
    </Note>
  </Step>

  <Step title="Aggiungi argomenti dello skill">
    Rendi il tuo skill dinamico accettando input dell'utente. Il placeholder `$ARGUMENTS` cattura qualsiasi testo che l'utente fornisce dopo il nome dello skill.

    Aggiorna il tuo file `SKILL.md`:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a personalized message
    ---

    # Hello Skill

    Greet the user named "$ARGUMENTS" warmly and ask how you can help them today. Make the greeting personal and encouraging.
    ```

    Esegui `/reload-plugins` per raccogliere i cambiamenti, poi prova lo skill con il tuo nome:

    ```shell theme={null}
    /my-first-plugin:hello Alex
    ```

    Claude ti saluterà per nome. Per ulteriori informazioni sul passaggio di argomenti agli skill, vedi [Skills](/docs/it/skills#pass-arguments-to-skills).
  </Step>
</Steps>

Hai creato e testato con successo un plugin con questi componenti chiave:

* **Plugin manifest** (`.claude-plugin/plugin.json`): descrive i metadati del tuo plugin
* **Directory skills** (`skills/`): contiene i tuoi skill personalizzati
* **Argomenti dello skill** (`$ARGUMENTS`): cattura l'input dell'utente per il comportamento dinamico

<Tip>
  Il flag `--plugin-dir` è utile per lo sviluppo e il test. Quando sei pronto a condividere il tuo plugin con altri, vedi [Crea e distribuisci un marketplace di plugin](/docs/it/plugin-marketplaces).
</Tip>

<h2 id="develop-a-plugin-in-your-skills-directory">
  Sviluppa un plugin nella tua directory skills
</h2>

Invece di passare `--plugin-dir` ad ogni avvio, puoi mantenere un plugin nella tua directory skills e fare in modo che Claude Code lo carichi automaticamente. `claude plugin init` ne crea uno:

```bash theme={null}
claude plugin init my-tool
```

Questo crea `~/.claude/skills/my-tool/` con un manifest `.claude-plugin/plugin.json` e uno starter `SKILL.md`. Nella sessione successiva si carica come `my-tool@skills-dir` senza alcun passaggio di marketplace o installazione.

Per le regole di caricamento automatico, l'ambito personale rispetto a quello del progetto, il requisito di fiducia dell'area di lavoro e come aggiornare o rimuoverne uno, vedi [Plugin della directory skills](/docs/it/plugins-reference#skills-directory-plugins).

<h2 id="plugin-structure-overview">
  Panoramica della struttura del plugin
</h2>

Hai creato un plugin con uno skill, ma i plugin possono includere molto di più: agents personalizzati, hooks, MCP servers, LSP servers e monitor in background.

<Warning>
  **Errore comune**: Non mettere `commands/`, `agents/`, `skills/` o `hooks/` dentro la directory `.claude-plugin/`. Solo `plugin.json` va dentro `.claude-plugin/`. Tutte le altre directory devono essere al livello radice del plugin.

  La radice del plugin è la directory del singolo plugin: quella che contiene `.claude-plugin/plugin.json`. Non è mai `~/.claude/`. Ad esempio, Claude Code non legge un `.mcp.json` posizionato in `~/.claude/.mcp.json`.
</Warning>

| Directory         | Posizione         | Scopo                                                                                      |
| :---------------- | :---------------- | :----------------------------------------------------------------------------------------- |
| `.claude-plugin/` | Radice del plugin | Contiene il manifest `plugin.json` (opzionale se i componenti usano posizioni predefinite) |
| `skills/`         | Radice del plugin | Skills come directory `<name>/SKILL.md`                                                    |
| `commands/`       | Radice del plugin | Skills come file Markdown flat. Usa `skills/` per i nuovi plugin                           |
| `agents/`         | Radice del plugin | Definizioni di agent personalizzati                                                        |
| `hooks/`          | Radice del plugin | Gestori di eventi in `hooks.json`                                                          |
| `.mcp.json`       | Radice del plugin | Configurazioni del server MCP                                                              |
| `.lsp.json`       | Radice del plugin | Configurazioni del server LSP per l'intelligenza del codice                                |
| `monitors/`       | Radice del plugin | Configurazioni del monitor in background in `monitors.json`                                |
| `bin/`            | Radice del plugin | Eseguibili aggiunti al `PATH` dello strumento Bash mentre il plugin è abilitato            |
| `settings.json`   | Radice del plugin | [Impostazioni](/docs/it/settings) predefinite applicate quando il plugin è abilitato            |

Un plugin che fornisce esattamente uno skill può posizionare `SKILL.md` direttamente alla radice del plugin invece di creare una directory `skills/`. Claude Code lo carica come uno skill singolo e utilizza il campo `name` del frontmatter per il nome di invocazione. Usa il layout `skills/` per i plugin che potrebbero crescere fino a più di uno skill.

<Note>
  **Prossimi passi**: Pronto ad aggiungere più funzionalità? Vai a [Sviluppa plugin più complessi](#develop-more-complex-plugins) per aggiungere agents, hooks, MCP servers e LSP servers. Per le specifiche tecniche complete di tutti i componenti del plugin, vedi [Riferimento plugin](/docs/it/plugins-reference).
</Note>

<h2 id="develop-more-complex-plugins">
  Sviluppa plugin più complessi
</h2>

Una volta che hai familiarità con i plugin di base, puoi creare estensioni più sofisticate.

<h3 id="add-skills-to-your-plugin">
  Aggiungi Skills al tuo plugin
</h3>

I plugin possono includere [Agent Skills](/docs/it/skills) per estendere le capacità di Claude. Gli skill sono invocati dal modello: Claude li usa automaticamente in base al contesto dell'attività.

Aggiungi una directory `skills/` alla radice del tuo plugin con cartelle Skill contenenti file `SKILL.md`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── code-review/
        └── SKILL.md
```

Ogni `SKILL.md` contiene frontmatter YAML e istruzioni. Includi una `description` in modo che Claude sappia quando usare lo skill:

```yaml theme={null}
---
description: Reviews code for best practices and potential issues. Use when reviewing code, checking PRs, or analyzing code quality.
---

When reviewing code, check for:
1. Code organization and structure
2. Error handling
3. Security concerns
4. Test coverage
```

Dopo aver installato il plugin, esegui `/reload-plugins` per caricare gli Skills. Per una guida completa sulla creazione di Skill inclusa la divulgazione progressiva e le restrizioni degli strumenti, vedi [Agent Skills](/docs/it/skills).

<h3 id="add-lsp-servers-to-your-plugin">
  Aggiungi server LSP al tuo plugin
</h3>

<Tip>
  Per linguaggi comuni come TypeScript, Python e Rust, installa i plugin LSP pre-costruiti dal marketplace ufficiale. Crea plugin LSP personalizzati solo quando hai bisogno di supporto per linguaggi non ancora coperti.
</Tip>

I plugin LSP (Language Server Protocol) danno a Claude l'intelligenza del codice in tempo reale. Se hai bisogno di supportare un linguaggio che non ha un plugin LSP ufficiale, puoi crearne uno tuo aggiungendo un file `.lsp.json` al tuo plugin:

```json .lsp.json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

Gli utenti che installano il tuo plugin devono avere il binario del language server installato sulla loro macchina.

Per le opzioni di configurazione LSP complete, vedi [LSP servers](/docs/it/plugins-reference#lsp-servers).

<h3 id="add-background-monitors-to-your-plugin">
  Aggiungi monitor in background al tuo plugin
</h3>

I monitor in background permettono al tuo plugin di osservare log, file o stato esterno in background e notificare Claude quando gli eventi arrivano. Claude Code avvia automaticamente ogni monitor quando il plugin è attivo, quindi non hai bisogno di istruire Claude ad avviare la sorveglianza.

Aggiungi un file `monitors/monitors.json` alla radice del plugin con un array di voci di monitor:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Ogni riga stdout da `command` viene consegnata a Claude come notifica durante la sessione. Per lo schema completo, incluso il trigger `when` e la sostituzione delle variabili, vedi [Monitors](/docs/it/plugins-reference#monitors).

<h3 id="ship-default-settings-with-your-plugin">
  Spedisci impostazioni predefinite con il tuo plugin
</h3>

I plugin possono includere un file `settings.json` alla radice del plugin per applicare la configurazione predefinita quando il plugin è abilitato. Attualmente, sono supportate solo le chiavi `agent` e `subagentStatusLine`.

Impostare `agent` attiva uno dei [custom agents](/docs/it/sub-agents) del plugin come thread principale, applicando il suo system prompt, le restrizioni degli strumenti e il modello. Questo consente a un plugin di cambiare il comportamento predefinito di Claude Code quando abilitato.

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Questo esempio attiva l'agent `security-reviewer` definito nella directory `agents/` del plugin. Le impostazioni da `settings.json` hanno priorità rispetto alle `settings` dichiarate in `plugin.json`. Le chiavi sconosciute vengono silenziosamente ignorate.

<h3 id="organize-complex-plugins">
  Organizza plugin complessi
</h3>

Per i plugin con molti componenti, organizza la tua struttura di directory per funzionalità. Per i layout di directory completi e i modelli di organizzazione, vedi [Struttura della directory del plugin](/docs/it/plugins-reference#plugin-directory-structure).

<h3 id="test-your-plugins-locally">
  Testa i tuoi plugin localmente
</h3>

Usa il flag `--plugin-dir` per testare i plugin durante lo sviluppo. Questo carica il tuo plugin direttamente senza richiedere l'installazione.

```bash theme={null}
claude --plugin-dir ./my-plugin
```

Il flag accetta anche un archivio `.zip` della directory del plugin, che richiede Claude Code v2.1.128 o successivo.

```bash theme={null}
claude --plugin-dir ./my-plugin.zip
```

Quando un plugin `--plugin-dir` ha lo stesso nome di un plugin marketplace installato, la copia locale ha la precedenza per quella sessione. Questo ti consente di testare le modifiche a un plugin che hai già installato senza disinstallarlo prima. L'eccezione è rappresentata dai plugin le cui impostazioni gestite forzano l'abilitazione o la disabilitazione: `--plugin-dir` non può sovrascrivere quelli.

Man mano che apporti modifiche al tuo plugin, esegui `/reload-plugins` per raccogliere gli aggiornamenti senza riavviare. Questo ricarica plugin, skills, agents, hooks, MCP servers del plugin e LSP servers del plugin. Testa i componenti del tuo plugin:

* Prova i tuoi skill con `/plugin-name:skill-name`
* Verifica che gli agents appaiano in `/context` sotto Custom Agents, o @-menziona uno per il suo nome con scope
* Verifica che gli hooks funzionino come previsto

<Tip>
  Puoi caricare più plugin contemporaneamente specificando il flag più volte:

  ```bash theme={null}
  claude --plugin-dir ./plugin-one --plugin-dir ./plugin-two
  ```
</Tip>

Per testare un plugin che è già stato confezionato come archivio `.zip` e ospitato su un URL, come un artefatto di build CI, usa `--plugin-url` invece. Claude Code recupera l'archivio all'avvio e lo carica solo per quella sessione. Se il recupero fallisce o l'archivio non è valido, Claude Code segnala un errore di caricamento del plugin e si avvia senza di esso. Le stesse [considerazioni sulla fiducia](/docs/it/discover-plugins#security) si applicano come per qualsiasi fonte di plugin: punta questo flag solo ad archivi che controlli o di cui ti fidi.

Per caricare più plugin, ripeti il flag per ogni URL:

```bash theme={null}
claude --plugin-url https://example.com/my-plugin.zip --plugin-url https://example.com/other.zip
```

Oppure passa URL separati da spazi come un singolo argomento tra virgolette:

```bash theme={null}
claude --plugin-url "https://example.com/my-plugin.zip https://example.com/other.zip"
```

<h3 id="debug-plugin-issues">
  Esegui il debug dei problemi del plugin
</h3>

Se il tuo plugin non funziona come previsto:

1. **Controlla la struttura**: Assicurati che le tue directory siano alla radice del plugin, non dentro `.claude-plugin/`
2. **Testa i componenti individualmente**: Controlla ogni skill, agent e hook separatamente
3. **Usa strumenti di validazione e debug**: Vedi [Strumenti di debug e sviluppo](/docs/it/plugins-reference#debugging-and-development-tools) per i comandi CLI e le tecniche di troubleshooting

<h3 id="share-your-plugins">
  Condividi i tuoi plugin
</h3>

Quando il tuo plugin è pronto per essere condiviso:

1. **Aggiungi documentazione**: Includi un `README.md` con istruzioni di installazione e utilizzo
2. **Scegli una strategia di versionamento**: Decidi se impostare una `version` esplicita o affidarti al commit SHA di git. Vedi [gestione della versione](/docs/it/plugins-reference#version-management)
3. **Crea o usa un marketplace**: Distribuisci tramite [marketplace di plugin](/docs/it/plugin-marketplaces) per l'installazione
4. **Testa con altri**: Fai testare il plugin ai colleghi del team prima di una distribuzione più ampia

Una volta che il tuo plugin è in un marketplace, altri possono installarlo usando le istruzioni in [Scopri e installa plugin](/docs/it/discover-plugins). Per mantenere un plugin interno al tuo team, ospita il marketplace in un [repository privato](/docs/it/plugin-marketplaces#private-repositories).

<h3 id="submit-your-plugin-to-the-community-marketplace">
  Invia il tuo plugin al marketplace della comunità
</h3>

Anthropic mantiene due marketplace pubblici per i plugin di Claude Code:

* **`claude-plugins-official`**: un insieme curato di plugin mantenuti da Anthropic. Registrato automaticamente la prima volta che avvii Claude Code in modo interattivo. Uno script non interattivo che viene eseguito prima di quel primo avvio deve aggiungerlo esplicitamente con `claude plugin marketplace add anthropics/claude-plugins-official`.
* **`claude-community`**: il marketplace pubblico della comunità dove gli invii di terze parti arrivano dopo la revisione. Gli utenti lo aggiungono con `/plugin marketplace add anthropics/claude-plugins-community` e lo installano come `@claude-community`.

Per inviare il tuo plugin per la revisione del marketplace della comunità, usa uno dei moduli in-app:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Il modulo claude.ai richiede un'organizzazione Team o Enterprise e accesso alla gestione della directory; i proprietari dell'organizzazione hanno questo accesso per impostazione predefinita. Gli autori individuali che non fanno parte di un'organizzazione Team o Enterprise possono utilizzare il modulo Console.

Esegui `claude plugin validate` localmente prima di inviare. La pipeline di revisione esegue lo stesso controllo su ogni invio, insieme a uno screening di sicurezza automatizzato.

I plugin approvati sono fissati a uno specifico commit SHA nel catalogo [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), e CI aumenta automaticamente il pin mentre esegui il push di nuovi commit nel tuo repository. Il catalogo pubblico si sincronizza ogni notte dalla pipeline di revisione, quindi può esserci un ritardo tra l'approvazione e la comparsa del tuo plugin in `marketplace.json`. Per verificare se il tuo plugin è già installabile, cerca il suo nome nel [catalogo della comunità](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

Il marketplace ufficiale, `claude-plugins-official`, è curato separatamente. Anthropic decide quali plugin includere a sua discrezione. Non c'è un processo di candidatura e il modulo di invio non aggiunge plugin al marketplace ufficiale.

Se Anthropic elenca il tuo plugin nel marketplace ufficiale, il tuo CLI può invitare gli utenti di Claude Code a installarlo. Vedi [Consiglia il tuo plugin dal tuo CLI](/docs/it/plugin-hints).

<Note>
  Per le specifiche tecniche complete, le tecniche di debug e le strategie di distribuzione, vedi [Riferimento plugin](/docs/it/plugins-reference).
</Note>

<h2 id="convert-existing-configurations-to-plugins">
  Converti configurazioni esistenti in plugin
</h2>

Se hai già skill o hooks nella tua directory `.claude/`, puoi convertirli in un plugin per una condivisione e distribuzione più facile.

<h3 id="migration-steps">
  Passaggi di migrazione
</h3>

<Steps>
  <Step title="Crea la struttura del plugin">
    Crea una nuova directory plugin nella radice del tuo progetto, accanto alla cartella `.claude/` esistente, in modo che i percorsi relativi `cp` nel passaggio successivo si risolvano:

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Crea il file manifest in `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Copia i tuoi file esistenti">
    Copia le tue configurazioni esistenti nella directory del plugin:

    ```bash theme={null}
    # Copy commands
    cp -r .claude/commands my-plugin/

    # Copy agents (if any)
    cp -r .claude/agents my-plugin/

    # Copy skills (if any)
    cp -r .claude/skills my-plugin/
    ```
  </Step>

  <Step title="Migra gli hook">
    Se hai hook nelle tue impostazioni, crea una directory hooks:

    ```bash theme={null}
    mkdir my-plugin/hooks
    ```

    Crea `my-plugin/hooks/hooks.json` con la tua configurazione degli hook. Copia l'oggetto `hooks` dal tuo `.claude/settings.json` o `settings.local.json`, poiché il formato è lo stesso. Il comando riceve l'input dell'hook come JSON su stdin, quindi usa `jq` per estrarre il percorso del file:

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Testa il tuo plugin migrato">
    Carica il tuo plugin per verificare che tutto funzioni:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Testa ogni componente: esegui i tuoi comandi, verifica che gli agents appaiano in `/context`, e verifica che gli hook si attivino correttamente.
  </Step>
</Steps>

<h3 id="what-changes-when-migrating">
  Cosa cambia durante la migrazione
</h3>

| Standalone (`.claude/`)                         | Plugin                                   |
| :---------------------------------------------- | :--------------------------------------- |
| Disponibile solo in un progetto                 | Può essere condiviso tramite marketplace |
| File in `.claude/commands/`                     | File in `plugin-name/commands/`          |
| Hook in `settings.json`                         | Hook in `hooks/hooks.json`               |
| Deve essere copiato manualmente per condividere | Installa con `/plugin install`           |

<Note>
  Dopo la migrazione, rimuovi i file originali da `.claude/` per evitare duplicati. Le definizioni di `.claude/agents/` a livello di progetto e utente sovrascrivono gli agents del plugin con lo stesso nome, quindi la versione del plugin ha effetto solo una volta rimossi gli originali. Le skill del plugin sono spaziate dei nomi come `/plugin-name:skill-name`, quindi sia l'originale `/skill-name` che la copia del plugin rimangono disponibili piuttosto che uno che sovrascrive l'altro.
</Note>

<h2 id="next-steps">
  Prossimi passi
</h2>

Ora che comprendi il sistema di plugin di Claude Code, ecco i percorsi suggeriti per diversi obiettivi:

<h3 id="for-plugin-users">
  Per gli utenti di plugin
</h3>

* [Scopri e installa plugin](/docs/it/discover-plugins): sfoglia i marketplace e installa i plugin
* [Configura marketplace del team](/docs/it/discover-plugins#configure-team-marketplaces): configura plugin a livello di repository per il tuo team

<h3 id="for-plugin-developers">
  Per gli sviluppatori di plugin
</h3>

* [Crea e distribuisci un marketplace](/docs/it/plugin-marketplaces): pacchetto e condividi i tuoi plugin
* [Riferimento plugin](/docs/it/plugins-reference): specifiche tecniche complete
* Approfondisci componenti specifici del plugin:
  * [Skills](/docs/it/skills): dettagli dello sviluppo dello skill
  * [Subagents](/docs/it/sub-agents): configurazione e capacità dell'agent
  * [Hooks](/docs/it/hooks): gestione degli eventi e automazione
  * [MCP](/docs/it/mcp): integrazione di strumenti esterni
