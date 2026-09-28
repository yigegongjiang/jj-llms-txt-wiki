> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Creare un plugin Claude Code

> Crea il tuo primo plugin Claude Code da una directory vuota, testalo senza un marketplace e converti una configurazione .claude/ esistente.

Un plugin è una directory di skills, agents, hooks e server MCP, più un file `plugin.json`, chiamato manifest, che nomina il plugin. Claude Code carica la directory come un'unità, quindi puoi condividerla con i colleghi, installarla in diversi progetti o pubblicarla in un marketplace.

Questa pagina è per le persone che scrivono i propri plugin.

<Note>
  Questi casi sono coperti su altre pagine:

  * **Installare il plugin di qualcun altro**: vedi [Installare plugin](/docs/it/plugins/install)
  * **Non sei sicuro di aver bisogno di un plugin**: vedi [Decidere se hai bisogno di un plugin](/docs/it/plugins/overview#decide-whether-you-need-a-plugin) nella panoramica
  * **Gli utenti del tuo plugin sono su claude.ai o in Cowork**: la stessa cartella si installa lì con un sottoinsieme diverso di componenti. Vedi [Plugin su claude.ai e in Cowork](https://claude.com/docs/plugins/overview)
</Note>

Inizia dalla sezione che corrisponde a quello che hai già:

* **Niente ancora**: segui [Creare il tuo primo plugin](#create-your-first-plugin), quindi [Sviluppare senza un marketplace](#develop-without-a-marketplace) e [Testare e debuggare](#test-and-debug).
* **File già sotto `.claude/`**: fai la procedura del primo plugin una volta per imparare il layout, quindi segui [Convertire una configurazione `.claude/` esistente](#convert-an-existing-claude-setup).

<h2 id="decide-when-to-use-a-plugin">
  Decidere quando usare un plugin
</h2>

Skills, agents, hooks e server MCP funzionano tutti in modo autonomo nel tuo progetto o nella directory home. Mantieni quella configurazione autonoma mentre serve un solo progetto o solo te. Crea un plugin quando vuoi condividere la configurazione con i colleghi, installarla in diversi progetti o pubblicare versioni rilasciate.

Quando sposti skills, agents, hooks e configurazione MCP autonomi in un plugin, la loro posizione e i loro nomi cambiano:

* **Dove vanno i file**: sotto la directory del plugin, chiamata radice del plugin, come `skills/`, `agents/`, `hooks/hooks.json` e `.mcp.json`.
* **Come sono nominati**: le skills e gli agents del plugin ottengono il nome del plugin come prefisso, come `/my-plugin:hello`, quindi due plugin possono ciascuno fornire una skill `hello` senza collisioni.

Per spostare una configurazione esistente in un plugin, vedi [Convertire una configurazione `.claude/` esistente](#convert-an-existing-claude-setup).

<h2 id="create-your-first-plugin">
  Creare il tuo primo plugin
</h2>

In questa procedura, crei un plugin il cui unico componente è una skill, un saluto, e lo esegui con `--plugin-dir`, che carica un plugin per una sessione senza installarlo. Un plugin può contenere qualsiasi mix di [componenti](/docs/it/plugins/components), come skills, agents, hooks e server MCP, e nessuno è obbligatorio; una skill è l'esempio più piccolo che mostra il layout.

Hai bisogno di Claude Code [installato e connesso](/docs/it/quickstart#step-1-install-claude-code).

Apri un terminale nella directory in cui vuoi mantenere il plugin, come `~/projects`, ed esegui i comandi in questi passaggi da lì. Puoi mantenere un plugin ovunque, perché passi il suo percorso a Claude Code quando avvii una sessione.

<Steps>
  <Step title="Creare la directory del plugin">
    Crea la directory del plugin, con una cartella `.claude-plugin/` dentro per contenere il manifest:

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="Scrivere il manifest">
    Il [manifest](/docs/it/plugins/manifest-reference) è un file JSON denominato `plugin.json` che dice a Claude Code il nome del plugin e lo descrive. Salva questo come `my-first-plugin/.claude-plugin/plugin.json`:

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

    I quattro campi fanno questo:

    * **`name`**: obbligatorio. Identifica il plugin e diventa il prefisso su ogni skill e agent che il plugin fornisce. Non mettere spazi in esso.
    * **`description`**: il testo che gli utenti vedono per il plugin in `/plugin`.
    * **`version`**: opzionale. Impostarlo mantiene gli utenti su quella versione finché non la cambi; [Rilasciare una nuova versione](/docs/it/plugins/host-marketplace#release-a-new-version) dice quando impostarla o ometterla.
    * **`author`**: a chi attribuire il merito. `name` è obbligatorio al suo interno; `email` e `url` sono opzionali.

    Ogni altro campo è nel [riferimento del manifest](/docs/it/plugins/manifest-reference#fields).

    Solo `plugin.json` va dentro `.claude-plugin/`. La skill che aggiungi dopo va direttamente sotto `my-first-plugin/`, accanto a quella cartella.
  </Step>

  <Step title="Aggiungere una skill">
    L'unico componente di questo plugin è una skill. Ogni skill è una directory sotto `skills/` che contiene un file `SKILL.md`. Crea la directory della skill:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Quindi crea `my-first-plugin/skills/hello/SKILL.md` con questo contenuto:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    La riga `disable-model-invocation: true` significa che Claude non esegue la skill da solo, quindi solo tu la attivi. Rimuovi quella riga da una skill che vuoi che Claude esegua da solo. Il comando della skill combina il nome del plugin e il nome della skill, quindi esegui questo come `/my-first-plugin:hello`. Per gli altri campi del frontmatter, vedi il [riferimento del frontmatter della skill](/docs/it/skills#frontmatter-reference).
  </Step>

  <Step title="Convalidare il plugin">
    Controlla il manifest e il frontmatter della skill prima di eseguire qualsiasi cosa:

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    Il comando stampa il percorso del manifest che ha controllato e `✔ Validation passed`. Se stampa `✘ Validation failed` invece, ogni riga sopra quella riga di risultato nomina il campo da correggere. Cerca ogni messaggio sotto [`claude plugin validate` segnala errori](/docs/it/plugins/troubleshooting#claude-plugin-validate-reports-errors).
  </Step>

  <Step title="Eseguire Claude Code con il plugin">
    Avvia una sessione con il plugin caricato:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Una volta che Claude Code si avvia, esegui la skill:

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude risponde con un saluto.
  </Step>
</Steps>

Il plugin carica solo nelle sessioni che avvii con `--plugin-dir`. Per continuare a lavorarci senza il flag, o per testare una build `.zip`, vedi [Sviluppare senza un marketplace](#develop-without-a-marketplace).

<h3 id="share-the-plugin">
  Condividere il tuo plugin
</h3>

Un plugin che hai costruito con [Creare il tuo primo plugin](#create-your-first-plugin) esiste solo sulla tua macchina. Quando è pronto per altre persone, ci sono tre modi per farlo arrivare a loro:

* **Inviarlo a poche persone direttamente**: dai loro la directory del plugin o un `.zip` di esso, e niente deve essere pubblicato. Vedi [Condividere un plugin senza un marketplace](/docs/it/plugins/publish#share-a-plugin-without-a-marketplace).
* **Elencarlo nel tuo marketplace**: i colleghi aggiungono il tuo marketplace una volta e installano il plugin per nome, e ricevono i tuoi aggiornamenti. Vedi [Pubblicare attraverso il tuo marketplace](/docs/it/plugins/publish#publish-through-your-own-marketplace).
* **Inviarlo al marketplace della comunità di Anthropic**: una volta elencato, chiunque aggiunga quel marketplace può installarlo. Vedi [Inviare al marketplace della comunità](/docs/it/plugins/publish#submit-to-the-community-marketplace).

<h3 id="plugin-layout">
  Layout del plugin
</h3>

Ogni tipo di [componente](/docs/it/plugins/components), come skills, agents, hooks e server MCP, va in una directory fissa sotto la radice del plugin, che è la directory che passi a `--plugin-dir`. Aggiungi solo le directory che usi. Per fare clic attraverso una directory di plugin completa e leggere cosa fa ogni file, apri l'[esploratore di plugin](/docs/it/plugins/components#explore-the-plugin-directory).

La tabella elenca le directory con cui la maggior parte dei plugin inizia, e il [layout completo](/docs/it/plugins/manifest-reference#standard-layout) elenca il resto.

| Posizione                    | Contenuti                                                                                                                             |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `.claude-plugin/plugin.json` | Il manifest. Quando carichi un plugin con `--plugin-dir` e non ha un manifest, Claude Code nomina il plugin dopo la sua directory     |
| `skills/`                    | Una directory `<name>/SKILL.md` per skill                                                                                             |
| `commands/`                  | File Markdown piatti, la forma più vecchia di skills. Usa `skills/` per i nuovi plugin                                                |
| `agents/`                    | Un file Markdown per subagent                                                                                                         |
| `hooks/hooks.json`           | Configurazione hook: una chiave `"hooks"` di livello superiore il cui valore ha la stessa forma di `hooks` in un file di impostazioni |
| `.mcp.json`                  | Definizioni del server MCP                                                                                                            |

<Warning>
  Solo `plugin.json` va dentro `.claude-plugin/`. I componenti salvati lì non caricano.

  La radice del plugin è la directory del plugin stesso, non `~/.claude/` stesso. Un `.mcp.json` salvato in `~/.claude/.mcp.json` non carica.
</Warning>

<h2 id="develop-without-a-marketplace">
  Sviluppare senza un marketplace
</h2>

Non hai bisogno di un [marketplace](/docs/it/plugins/overview#get-plugins-from-a-marketplace) per eseguire un plugin che stai scrivendo. Caricalo direttamente dal disco o da un URL invece:

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session): carica una directory o un archivio `.zip` per una sessione.
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session): recupera un archivio `.zip` da un URL per una sessione.
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session): scaffolda un plugin sotto `~/.claude/skills/` che carica ogni sessione.

Se due plugin caricati in modi diversi condividono un nome, vedi [Conflitti di nomi](/docs/it/plugins/loading#name-conflicts) per quale Claude Code mantiene.

<h3 id="load-a-directory-or-archive-for-one-session">
  Caricare un plugin per una sessione
</h3>

Puoi caricare un plugin per una singola sessione in tre modi: da una directory o un archivio `.zip` sul disco con `--plugin-dir`, da un URL con `--plugin-url`, o da una variabile di ambiente quando non puoi aggiungere un flag. Ogni plugin carica solo per quella sessione, e niente viene scritto nelle tue impostazioni per esso. Quando modifichi i file del plugin durante la sessione, esegui `/reload-plugins` per caricare le modifiche.

<h4 id="from-a-directory-or-zip">
  Da una directory o `.zip`
</h4>

Quando avvii `claude` dalla tua shell, passa `--plugin-dir` con la directory radice del plugin o un archivio `.zip` di esso. Ripeti il flag per caricare diversi plugin:

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  Da una cartella di plugin
</h4>

Per caricare diversi plugin da un posto, passa una cartella che li contiene, come `--plugin-dir ./plugins`. Caricare una cartella di plugin richiede Claude Code v2.1.265 o successivo.

Se la cartella non ha una directory `.claude-plugin/` e nessun componente di plugin al suo livello superiore, Claude Code la tratta come una cartella di plugin. Ogni sottocartella immediata che ha un manifest `.claude-plugin/plugin.json` quindi carica come un plugin separato. Tutto il resto nella cartella viene saltato senza un errore, inclusa una sottocartella che non ha un manifest. Se un plugin nella cartella non carica, controlla che la sua sottocartella abbia un `.claude-plugin/plugin.json`.

In una sessione interattiva, puoi anche aggiungere e rimuovere plugin nella cartella dopo l'avvio:

* Una sottocartella che aggiungi carica come un nuovo plugin una volta che il suo manifest esiste.
* Quando rimuovi una sottocartella, il suo plugin scarica.

Un messaggio appare nella sessione per ciascuno di questi cambiamenti. Se caricare o scaricare un plugin a metà conversazione [invaliderebbe la cache del prompt](/docs/it/prompt-caching#enabling-or-disabling-a-plugin), il cambiamento viene mantenuto invece, e il messaggio ti dice di eseguire `/reload-plugins` per applicarlo.

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  Da un URL
</h4>

Quando avvii `claude` dalla tua shell, passa `--plugin-url` con l'indirizzo di un archivio `.zip`, come un artefatto di build che il tuo CI pubblica:

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code scarica l'archivio all'avvio. Per caricare diversi, ripeti il flag o passa gli URL separati da spazi in un argomento tra virgolette.

Punta il flag solo agli archivi che controlli o di cui ti fidi.

Se Claude Code non riesce a recuperare l'archivio, o l'archivio non è valido, si avvia senza il plugin e registra un errore di caricamento del plugin che puoi rivedere nella scheda **Errori** del gestore `/plugin`.

<h4 id="from-an-environment-variable">
  Da una variabile di ambiente
</h4>

Per caricare plugin in una sessione in cui non puoi aggiungere il flag `--plugin-dir`, elenca i loro percorsi assoluti nella variabile di ambiente [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/it/env-vars#variables) invece. Claude Code carica ogni percorso come carica un percorso `--plugin-dir`. Questi plugin caricano in aggiunta a quelli che passi con `--plugin-dir`. [Le impostazioni di progetto e locali non possono impostare questa variabile](/docs/it/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_PLUGIN_DIRS` richiede Claude Code v2.1.280 o successivo.

Le impostazioni gestite possono disattivare `--plugin-dir` e `CLAUDE_CODE_PLUGIN_DIRS`. Vedi [Flag che caricano un plugin per una sessione](/docs/it/plugins/cli-reference#flags-that-load-a-plugin-for-one-session). Per testare un plugin insieme a un plugin da cui dipende, vedi [Testare un plugin e la sua dipendenza localmente](/docs/it/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="scaffold-a-plugin-that-loads-every-session">
  Fare in modo che un plugin carichi in ogni sessione
</h3>

La tua directory di skills personale è `~/.claude/skills/`. Claude Code carica qualsiasi cartella lì che contiene un `.claude-plugin/plugin.json` come un plugin in ogni sessione, senza flag e senza passaggio di installazione. `claude plugin init` scaffolda uno di questi plugin per te.

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  Scaffoldare il plugin con `claude plugin init`
</h4>

`claude plugin init` scrive un plugin iniziale sotto `~/.claude/skills/`. Richiede Claude Code v2.1.157 o successivo. Scaffolda uno dalla tua shell:

```bash theme={null}
claude plugin init my-tool
```

Il comando crea `~/.claude/skills/my-tool/` con un `.claude-plugin/plugin.json` e un `SKILL.md` radice. Stampa `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool` seguito da `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.`

Passa `--with skills` per avere `claude plugin init` scaffolda una skill sotto `skills/` per te. Gli altri valori `--with` sono nel [riferimento dei comandi del plugin](/docs/it/plugins/cli-reference#plugin-init).

<h4 id="skill-names-in-a-scaffolded-plugin">
  Nominare le skills del plugin
</h4>

La skill radice in `~/.claude/skills/my-tool/SKILL.md` è anche una skill personale, quindi la invochi come `/my-tool`, non `/my-tool:my-tool`. Le skills che aggiungi sotto `skills/` dentro il plugin ottengono il prefisso del nome del plugin, come `/my-tool:example`.

<h4 id="stop-loading-the-plugin">
  Smettere di caricare il plugin
</h4>

Per smettere di caricare un plugin scaffoldato, elimina la sua directory, o esegui `claude plugin disable my-tool@skills-dir` nella tua shell con il nome `my-tool@skills-dir` che `claude plugin init` ha stampato. Nell'ID `my-tool@skills-dir`, `skills-dir` sta al posto di un nome di marketplace, perché il plugin carica dalla tua directory di skills piuttosto che da un marketplace.

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  Condividere il plugin attraverso un repository
</h4>

`claude plugin init` scrive il plugin nella tua directory di skills personale in `~/.claude/skills/`, quindi carica per te in ogni progetto. Per fare in modo che un plugin carichi per tutti in un repository, crea lo stesso layout tu stesso in `<project>/.claude/skills/<name>/`, incluso il suo `.claude-plugin/plugin.json`. Vedi [Plugin condivisi attraverso un repository](/docs/it/plugins/loading#plugins-shared-through-a-repository) per le condizioni in cui Claude Code lo carica.

<h2 id="test-and-debug">
  Testare e debuggare
</h2>

Quando una modifica al tuo plugin non viene visualizzata, lavora attraverso questi controlli in ordine. Ognuno ti dice cosa Claude Code ha fatto con il plugin:

1. Nella tua shell, esegui `claude plugin validate <path>`. Controlla il manifest e il frontmatter di ogni file di skill, agent e command, ed esce con `0` su `Validation passed`. Aggiungi `--strict` per fallire anche su avvisi. I codici di uscita e la gestione delle directory sono nel [riferimento dei comandi del plugin](/docs/it/plugins/cli-reference#plugin-validate).
2. Nella sessione in esecuzione, esegui `/reload-plugins` per applicare le modifiche che hai fatto sul disco. Stampa una riga `Reloaded:` con i conteggi. Quindi conferma che una skill ha caricato digitando il suo comando `/plugin-name:skill`, o trovando il plugin nella scheda **Installati** di `/plugin`.
3. Nella stessa sessione, esegui `/plugin`. La scheda **Installati** elenca il tuo plugin e, nei dettagli del plugin, i componenti che Claude Code ha trovato. La scheda **Errori** elenca cosa non ha caricato e perché, come un percorso nel tuo manifest che non esiste.
4. Di nuovo nella tua shell, esegui `claude plugin list`. Stampa plugin solo per sessione e della directory di skills nelle loro sezioni proprie con `Status: ✔ loaded` o l'errore di caricamento. Per includere il plugin che stai sviluppando, passa `--plugin-dir` con il suo percorso prima di `plugin list`.

Per controllare un server MCP, esegui `/mcp` nella sessione per vedere lo stato del server. Quando il server è sano, `/mcp` lo elenca come connesso. Se non lo è, vedi [Server MCP che non si avviano](/docs/it/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Per controllare un hook, attiva l'evento che corrisponde. Ad esempio, chiedi a Claude di modificare un file per attivare un hook `PostToolUse`. Quindi leggi il [log di debug](/docs/it/hooks#debug-hooks), che mostra quali hook hanno corrisposto, i loro codici di uscita e il loro output.

Le sezioni successive coprono i fallimenti che è più probabile che tu colpisca durante lo sviluppo, e la [pagina di troubleshooting](/docs/it/plugins/troubleshooting#build-a-plugin) ha la voce completa per ognuno.

<h3 id="a-component-path-isn’t-found">
  Un percorso di componente non viene trovato
</h3>

La scheda **Errori** di `/plugin` mostra `<component> path not found: <path>`, ad esempio `commands path not found`. Un percorso di componente nel tuo manifest, come `commands`, `skills`, `agents` o `hooks`, punta a niente. Correggi il percorso o crea la directory, quindi esegui `/reload-plugins` nella sessione. Vedi [`commands path not found`](/docs/it/plugins/troubleshooting#commands-path-not-found).

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir` alla radice di un marketplace non carica i plugin sotto `plugins/`
</h3>

`--plugin-dir` prende la directory radice del plugin, quella che contiene `.claude-plugin/plugin.json` e le directory dei componenti come `skills/`. Se lo punti alla radice di un marketplace invece, Claude Code non legge `marketplace.json`, quindi un plugin sotto `plugins/` non carica, e non vedi alcun errore. Punta il flag alla cartella di un plugin, o aggiungi il marketplace. Vedi [la voce di troubleshooting](/docs/it/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components).

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  Il plugin carica ma le sue skills sono mancanti
</h3>

La directory `skills/` è dentro `.claude-plugin/`, o una voce `skills` nel manifest punta a un file. Sposta `skills/` alla radice del plugin, punta ogni voce `skills` a una directory che contiene `SKILL.md`, e esegui `/reload-plugins` nella sessione. Vedi [Plugin carica ma le sue skills sono mancanti](/docs/it/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing).

<h3 id="the-userconfig-dialog-never-appears">
  La finestra di dialogo `userConfig` non appare mai
</h3>

La finestra di dialogo per le opzioni [`userConfig`](/docs/it/plugins/components#user-configuration) del tuo plugin fa parte dell'installazione attraverso `/plugin` in una sessione. Caricare con `--plugin-dir` non la mostra, e nemmeno `claude plugin install` nella shell. Con il plugin caricato, esegui `/plugin configure <plugin-name>` nella sessione per aprirla. Vedi [La finestra di dialogo `userConfig` non appare mai](/docs/it/plugins/troubleshooting#the-userconfig-dialog-never-appears).

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  Controllare che il plugin cambi il comportamento di Claude
</h3>

Un plugin che carica senza errori può comunque non riuscire a guidare Claude nel modo che intendi. `claude plugin eval`, che esegui nella tua shell, esegue i tuoi casi di test con e senza il plugin e valuta la differenza. Vedi [Testare plugin con evals](/docs/it/plugin-evals), iniziando con [Creare il tuo primo eval suite](/docs/it/plugin-evals#create-your-first-eval-suite).

<h2 id="convert-an-existing-claude-setup">
  Convertire una configurazione `.claude/` esistente
</h2>

Se hai già skills, agents o hooks sotto una directory `.claude/` di un progetto, puoi spostarli in un plugin senza riscriverli.

Esegui i comandi in questi passaggi dalla radice del progetto, che è la directory che contiene `.claude/`, perché i percorsi `cp` sono relativi ad esso.

<Steps>
  <Step title="Creare la struttura del plugin">
    Crea la directory del plugin e la sua cartella `.claude-plugin/` accanto a `.claude/`. Puoi spostare il plugin ovunque in seguito.

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Crea `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Copiare i tuoi file esistenti">
    Copia ogni directory di configurazione che hai alla radice del plugin, e salta il comando per qualsiasi directory che non hai.

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    Esegui `ls -a my-plugin` per confermare che ogni directory che hai copiato appare accanto a `.claude-plugin`.
  </Step>

  <Step title="Spostare i tuoi hook">
    Se hai hook in `.claude/settings.json` o `.claude/settings.local.json`, crea una directory di hook:

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    Crea `my-plugin/hooks/hooks.json` e copia l'oggetto `hooks` dal tuo file di impostazioni in esso. Il formato è lo stesso.

    Questo esempio mostra la forma con un hook che esegue un linter su ogni file che Claude scrive o modifica. Sostituisci l'esempio con il tuo oggetto `hooks`.

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

  <Step title="Testare il plugin migrato">
    Carica il plugin per una sessione:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Controlla ogni componente sotto il suo nuovo nome:

    * **Skills**: esegui `/my-plugin:deploy` per una skill che era `/deploy`.
    * **Subagent**: chiedi a Claude di usare l'agent `my-plugin:reviewer` per un agent che era `reviewer`.
    * **Hook**: attiva l'evento che ogni hook corrisponde.

    Se qualcosa è mancante, lavora attraverso [Testare e debuggare](#test-and-debug).
  </Step>
</Steps>

Mentre gli originali sono ancora sotto `.claude/`, rimangono caricati insieme alle copie del plugin:

* **Skills e agents**: i due set non si scontrano, perché le skills e gli agents del plugin portano il prefisso `my-plugin:`. `/deploy` e `/my-plugin:deploy` funzionano entrambi, e Claude vede `reviewer` e `my-plugin:reviewer` come due subagent.
* **Hook**: gli hook non hanno prefisso, quindi un hook che è sia nel tuo file di impostazioni che in `hooks/hooks.json` viene eseguito due volte ogni volta che il suo evento si attiva.

Dopo aver confermato che il plugin funziona, elimina gli originali da `.claude/` e rimuovi l'oggetto `hooks` dal tuo file di impostazioni.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Componenti del plugin](/docs/it/plugins/components): aggiungi agents, hook, server MCP, server LSP e configurazione utente al tuo plugin
* [Testare plugin con evals](/docs/it/plugin-evals): scrivi casi di eval ed eseguili con `claude plugin eval` per controllare quanto affidabilmente il plugin guida il comportamento di Claude
* [Pubblicare un plugin](/docs/it/plugins/publish): versiona, mettilo in un marketplace e invialo al marketplace della comunità
* [Plugin su claude.ai e in Cowork](https://claude.com/docs/plugins/overview): la stessa cartella di plugin si installa su claude.ai e in Cowork. Alcuni componenti sono solo Claude Code
* [Riferimento del manifest del plugin](/docs/it/plugins/manifest-reference): ogni campo `plugin.json`, regola di percorso e directory
* [Skills](/docs/it/skills): scrivi le skills che il tuo plugin fornisce
* [Plugin di Anthropic nel repository claude-code](https://github.com/anthropics/claude-code/tree/main/plugins): esempi completi e funzionanti del layout su questa pagina, come `feature-dev` e `code-review`
