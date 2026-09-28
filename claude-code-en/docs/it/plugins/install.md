> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Installare e gestire i plugin

> Installare i plugin di Claude Code da un marketplace su qualsiasi superficie che utilizzi, scegliere un ambito di installazione e aggiornarli o rimuoverli in seguito.

L'installazione di un plugin aggiunge le sue skills, agents, hooks e server MCP a Claude Code sulla tua macchina.

Questa pagina è per chiunque utilizzi i plugin sulla propria macchina o account, sia nel terminale, nell'app desktop, in un IDE o in una sessione cloud: copre l'installazione, la scelta di un ambito, l'aggiunta di marketplace e il mantenimento dei plugin aggiornati.

<Note>
  Questi casi sono coperti su altre pagine:

  * **Utilizzi claude.ai chat o Cowork, non Claude Code**: vedi [Plugin su claude.ai e in Cowork](https://claude.com/docs/plugins/overview)
  * **Claude Code ha stampato un errore**: trovalo in [Troubleshoot plugins](/docs/it/plugins/troubleshooting)
</Note>

Inizia con [Installare un plugin](#install-a-plugin). Se qualcuno ti ha inviato un comando di installazione il cui nome `@` non è `claude-plugins-official`, [aggiungi prima quel marketplace](#add-a-marketplace).

<h2 id="install-a-plugin">
  Installare un plugin
</h2>

Come esempio, questa sezione installa [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands) dal [marketplace ufficiale di Anthropic](/docs/it/plugins/anthropic-marketplaces), che aggiunge comandi per il commit, il push e l'apertura di pull request.

Gli stessi passaggi installano qualsiasi altro plugin: sostituisci il suo nome e il nome del suo marketplace ovunque compaiano `commit-commands` e `claude-plugins-official`. Se quel plugin proviene da un marketplace diverso, [aggiungi prima il marketplace](#add-a-marketplace).

Scegli la scheda per il luogo in cui esegui Claude Code.

<Tabs>
  <Tab title="Terminal">
    Avvia Claude Code con `claude` nel tuo progetto, quindi:

    <Steps>
      <Step title="Apri i dettagli del plugin con il comando di installazione">
        Esegui `/plugin install` con il nome del plugin e il marketplace. In una sessione, questo comando non installa subito: apre il pannello `/plugin` sui dettagli di quel plugin in modo che tu possa rivederlo e scegliere prima un ambito.

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        Per sfogliare invece, esegui `/plugin` senza un nome di plugin: il pannello si apre sulla scheda **Discover**, che elenca i plugin da ogni marketplace che hai aggiunto, e puoi digitare per cercare, quindi premere **Enter** su un plugin per aprire i suoi dettagli.
      </Step>

      <Step title="Rivedi cosa aggiunge il plugin">
        Il riquadro dei dettagli mostra la descrizione del plugin. Può anche mostrare:

        * **Will install**: i comandi, gli agents, le skills, gli hooks e i server MCP e LSP che il plugin aggiunge.
        * **Last updated**: mostrato per un plugin nel marketplace ufficiale di Anthropic.
        * **Context cost**: per un plugin nel marketplace ufficiale di Anthropic, due stime di token. **Every turn** è quello che il plugin aggiunge a ogni messaggio che invii, e **When invoked** è quello che le sue skills e agents aggiungono una volta che Claude le carica. Le stime appaiono quando apri il plugin nominando il suo marketplace, come fa il comando del passaggio 1, o dalla scheda **Marketplaces**. Il riquadro dei dettagli che raggiungi dall'elenco **Discover** non le mostra.

        I plugin da un marketplace locale o personalizzato possono mostrare `Components will be discovered at installation` invece.

        Un plugin può eseguire hooks e server MCP, quindi leggi il riquadro prima di installare. Vedi [Plugin security and trust](/docs/it/plugins/security).
      </Step>

      <Step title="Scegli un ambito">
        Seleziona una delle tre opzioni di installazione:

        * **Install for you (user scope)**: ottieni il plugin in ogni progetto su questa macchina
        * **Install for all collaborators on this repository (project scope)**: è abilitato per tutti coloro che lavorano in questo repository
        * **Install for you, in this repo only (local scope)**: lo ottieni solo in questo repository

        [Scegli un ambito di installazione](#choose-an-install-scope) dice quale file di impostazioni ogni uno scrive e quale si applica quando lo stesso plugin è impostato in più di uno.

        Dopo aver selezionato un ambito, Claude Code installa il plugin insieme a tutte le dipendenze che dichiara, quindi stampa un riepilogo dell'installazione.
      </Step>

      <Step title="Leggi il riepilogo dell'installazione">
        L'ultima frase del riepilogo ti dice se il plugin è utilizzabile in questa sessione:

        * **Active now**: `Plugin is now active.` Non è necessario ricaricare.
        * **Reload needed**: `Run /reload-plugins to activate.` Il pannello si chiude e Claude Code esegue quel ricaricamento per te. Se il ricaricamento [invaliderebbe la prompt cache](/docs/it/prompt-caching#enabling-or-disabling-a-plugin), avverte e lascia il plugin in sospeso invece. Esegui `/reload-plugins --force` per attivarlo comunque, il che costa una richiesta non memorizzata nella cache.
        * **Load failed**: `The plugin couldn't be loaded`. Apri la scheda **Errors** in `/plugin` per il motivo, quindi vedi [After install: plugin not working](/docs/it/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>

      <Step title="Conferma che il plugin funziona">
        Digita `/` e cerca le skills del plugin sotto il suo nome, nella forma `/<plugin>:<skill>`. Per `commit-commands`, appare `/commit-commands:commit`. Due altri posti elencano il plugin:

        * Apri la scheda **Installed** in `/plugin`, che elenca il plugin con il suo ambito.
        * Nel tuo shell, esegui `claude plugin list`, che stampa lo stesso elenco con le righe `Version`, `Scope` e `Status`.

        Se `/commit-commands:commit` non appare, vedi [After install: plugin not working](/docs/it/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>
    </Steps>

    L'installazione da qualsiasi altro marketplace richiede un passaggio extra prima: [aggiungi il marketplace](#add-a-marketplace). Claude Code aggiunge il marketplace ufficiale di Anthropic per te la prima volta che avvii una sessione di terminale interattiva, motivo per cui l'esempio salta quel passaggio. Se hai trovato un plugin su [claude.com/marketplace](https://claude.com/marketplace), il suo pulsante **Claude Code** copia il comando di installazione nella sua [forma shell](#install-from-your-shell), `claude plugin install <name>@claude-plugins-official`.
  </Tab>

  <Tab title="Desktop app">
    In una sessione locale o SSH nella scheda **Code** dell'app desktop:

    <Steps>
      <Step title="Apri il browser dei plugin">
        Fai clic sul pulsante **+** accanto alla casella del prompt e seleziona **Plugins**, quindi **Add plugin**. Il browser dei plugin si apre con i plugin dai tuoi marketplace.
      </Step>

      <Step title="Seleziona il plugin">
        Trova `commit-commands` e selezionalo.
      </Step>

      <Step title="Scegli un ambito">
        Scegli un [ambito](#choose-an-install-scope): il tuo account utente, questo progetto o solo locale.
      </Step>
    </Steps>

    Per abilitare, disabilitare o disinstallare in seguito, usa **+ > Plugins > Manage plugins**. Il browser dei plugin non è disponibile nelle sessioni cloud dell'app desktop. Vedi [Install plugins in the desktop app](/docs/it/desktop#install-plugins).
  </Tab>

  <Tab title="VS Code">
    Nel pannello Claude Code in VS Code:

    <Steps>
      <Step title="Apri Manage plugins">
        Digita `/plugins` nella casella del prompt per aprire **Manage plugins**.
      </Step>

      <Step title="Installa il plugin">
        Sulla scheda **Plugins**, cerca `commit-commands` e fai clic su **Install**. Se la scheda non elenca alcun plugin, aggiungi prima `anthropics/claude-plugins-official` sulla scheda **Marketplaces**.
      </Step>

      <Step title="Scegli un ambito">
        Scegli un [ambito](#choose-an-install-scope): **Install for you**, **Install for this project** o **Install locally**.
      </Step>
    </Steps>

    Le tue modifiche si applicano alle sessioni aperte senza un riavvio. Vedi [Manage plugins in VS Code](/docs/it/vs-code#manage-plugins).
  </Tab>

  <Tab title="Cloud session">
    Una [sessione cloud](/docs/it/cloud-environments), incluso [il browser su claude.ai/code](/docs/it/claude-code-on-the-web), non ha un browser dei plugin e non carica i plugin che hai installato sulla tua macchina o quelli che il `.claude/settings.json` del tuo repository attiva. Per i plugin che la tua organizzazione distribuisce attraverso le impostazioni gestite, vedi [Manage plugins for your organization](/docs/it/plugins/org).

    Vedi [quali parti della tua configurazione sono disponibili anche in una sessione cloud](/docs/it/cloud-environments#what-carries-over-from-your-setup) per il resto della tua configurazione.
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  Scegli un ambito di installazione
</h3>

L'ambito di installazione di un plugin decide chi ottiene il plugin e quale file di impostazioni lo registra come abilitato:

* **User scope**: il plugin è abilitato per te in ogni progetto su questa macchina. La voce va in `enabledPlugins` in `~/.claude/settings.json`.
* **Project scope**: il plugin è abilitato per tutti coloro che lavorano in questo repository. La voce va in `.claude/settings.json`, che esegui il commit.
* **Local scope**: il plugin è abilitato per te solo in questo repository. La voce va in `.claude/settings.local.json`.

Alcuni plugin sono impostati dal loro autore per iniziare disattivati, attraverso il campo [`defaultEnabled`](/docs/it/plugins/manifest-reference#defaultenabled). Tale plugin è installato ma rimane disattivato finché non lo attivi con `claude plugin enable <name>` nel tuo shell, o dalla scheda **Installed** di `/plugin` in una sessione.

Quando lo stesso plugin è impostato in più ambiti, l'impostazione locale sostituisce l'impostazione del progetto e l'impostazione del progetto sostituisce l'impostazione dell'utente. Vedi [Find where a plugin is enabled](/docs/it/plugins/loading#find-where-a-plugin-is-enabled) per la regola completa.

Il terminale, le sessioni locali dell'app desktop e l'estensione VS Code su un computer leggono gli stessi file di impostazioni, quindi un plugin che installi a livello di utente in uno di essi è disponibile negli altri due.

<h3 id="other-places-you-run-claude-code">
  JetBrains, esecuzioni non interattive e Agent SDK
</h3>

Alcuni posti in cui esegui Claude Code non hanno un browser dei plugin proprio:

* **JetBrains IDEs**: il plugin JetBrains esegue Claude Code nel terminale dell'IDE, quindi usa i passaggi della scheda **Terminal**.
* **`claude -p` e altre esecuzioni non interattive**: `/plugin` non viene eseguito e Claude risponde `/plugin isn't available in this environment.` I plugin che hai già installato si caricano. Installali e gestiscili dal tuo shell con i [comandi `claude plugin`](#install-from-your-shell).
* **Agent SDK**: carica i plugin attraverso l'opzione plugin dell'SDK. Vedi [Load plugins in the Agent SDK](/docs/it/agent-sdk/plugins).

Se Claude Code segnala che un plugin abilitato nel `.claude/settings.json` del repository non è installato, vedi [Enabled in project settings but not installed](/docs/it/plugins/loading#enabled-in-project-settings-but-not-installed).

<Tip>
  Se sei un autore di plugin che testa una copia del tuo plugin su disco, avvia Claude Code dal tuo shell con `--plugin-dir` per caricarlo per una sessione invece di installarlo. Vedi [Flags that load a plugin for one session](/docs/it/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  Plugin dal tuo account claude.ai
</h3>

Il tuo account claude.ai è una fonte separata di plugin, insieme ai marketplace da cui installi:

* **Cosa arriva**: ogni plugin che attivi per il tuo account claude.ai e ogni plugin che la tua organizzazione attiva per i suoi membri. In una sessione di terminale si sincronizzano in background ogni volta che avvii Claude Code mentre sei connesso con quell'account; nelle sessioni Cowork si scaricano quando la sessione inizia.
* **Dove li vedi**: in `/plugin` e `claude plugin list` sotto l'ID `<name>@synced`. Puoi disattivarne uno al tuo ambito a meno che la tua organizzazione non lo richieda.
* **Cosa non va dall'altra parte**: i plugin che installi con `/plugin` o `claude plugin install` rimangono su questa macchina e non vengono aggiunti al tuo account claude.ai.

Per i tempi di sincronizzazione, i requisiti di accesso e la disattivazione della sincronizzazione, vedi [Plugins synced from claude.ai](/docs/it/plugins/loading#synced-plugins).

<h3 id="install-from-your-shell">
  Installa dal tuo shell
</h3>

Esegui `claude plugin install` nel tuo shell per installare un plugin senza avviare una sessione Claude Code, ad esempio da uno script di configurazione.

* **Scope**: ambito utente per impostazione predefinita. Passa `--scope project` o `--scope local` per cambiarlo.
* **Quando i plugin si caricano**: i plugin che installa si caricano la prossima volta che avvii Claude Code, o quando esegui `/reload-plugins` in una sessione già aperta.
* **Il marketplace deve essere aggiunto prima**: su una macchina in cui nessuno ha ancora aperto una sessione Claude Code interattiva, il marketplace ufficiale non è registrato, quindi uno script che installa da esso esegue `claude plugin marketplace add anthropics/claude-plugins-official` prima dell'installazione.

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

Il comando stampa `Successfully installed plugin: formatter@your-org (scope: project)` quando finisce.

Alcuni plugin si installano eseguendo un comando che il loro marketplace nomina, chiamato una [`command` source](/docs/it/plugins/marketplace-reference#command-plugin-source). Claude Code ti mostra quel comando e ti chiede di accettarlo prima che venga eseguito. Uno script non ha nessuno per rispondere a quel prompt, quindi passa `--yes` lì per accettarlo.

Per ogni flag `claude plugin install`, vedi [plugin install](/docs/it/plugins/cli-reference#plugin-install).

<h2 id="add-a-marketplace">
  Aggiungi un marketplace
</h2>

Hai bisogno di questa sezione solo quando il plugin che desideri non è nel marketplace ufficiale di Anthropic, ad esempio uno che un collega ha pubblicato o uno dal marketplace della comunità di Anthropic.

Un marketplace è un catalogo di plugin e Claude Code deve conoscere un marketplace prima di poter installare da esso. Aggiungi un marketplace una volta. Dopo di che, i suoi plugin appaiono sulla scheda **Discover** e si installano con `/plugin install <plugin>@<marketplace>` in una sessione o `claude plugin install <plugin>@<marketplace>` nel tuo shell, dove `<marketplace>` è il nome con cui il marketplace si è registrato. Per fare entrambi in un passaggio, vedi [Aggiungi un marketplace e installa in un comando](#add-a-marketplace-and-install-in-one-command).

In una sessione Claude Code, esegui `/plugin marketplace add` seguito dalla fonte del marketplace: un repository GitHub, un repository git su qualsiasi host, una directory o file locale, o un `marketplace.json` ospitato.

| Fonte                            | Cosa digiti                                                                                                                                                                                                                                  | Esempio                                                                                                                           |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| Repository GitHub                | `owner/repo`. Aggiungi `#ref` per fissare un ramo o un tag.                                                                                                                                                                                  | `/plugin marketplace add anthropics/claude-code`, o `/plugin marketplace add your-org/plugins#v1.2.0` per fissare il tag `v1.2.0` |
| Repository Git su qualsiasi host | L'URL di clonazione completo. Aggiungi `#ref` per fissare un ramo o un tag.                                                                                                                                                                  | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                                       |
| Directory o file locale          | Un percorso relativo o assoluto a una directory che contiene `.claude-plugin/marketplace.json`, o al file JSON stesso. Inizia un percorso relativo con `./` o `../`, perché Claude Code legge un `name/name` nudo come un repository GitHub. | `/plugin marketplace add ./my-marketplace`                                                                                        |
| `marketplace.json` ospitato      | Il suo URL `https://`                                                                                                                                                                                                                        | `/plugin marketplace add https://example.com/marketplace.json`                                                                    |

Dal tuo shell, `claude plugin marketplace add` accetta le stesse fonti.

<Tip>
  `/plugin market` funziona anche come forma più breve di `/plugin marketplace`.
</Tip>

Includi il prefisso `https://` su ogni URL, o usa la forma `git@host:path` per SSH. Se digiti un `gitlab.example.com/your-group/your-marketplace.git` nudo, Claude Code lo legge come scorciatoia GitHub `owner/repo` e lo rifiuta.

Quando il comando ha successo, stampa `Successfully added marketplace: <name>`, e i plugin del marketplace appaiono sulla scheda **Discover** la prossima volta che apri `/plugin`, senza necessità di ricaricamento. Se fallisce, abbina il messaggio di errore in [Troubleshoot plugins](/docs/it/plugins/troubleshooting#add-a-marketplace).

<h3 id="add-a-marketplace-and-install-in-one-command">
  Aggiungi un marketplace e installa in un comando
</h3>

Per installare un plugin da un marketplace che non hai ancora aggiunto, esegui `/plugin install` in una sessione Claude Code e nomina la fonte del marketplace con `--marketplace`. Richiede Claude Code v2.1.275 o successivo.

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

La fonte accetta [le stesse forme di `/plugin marketplace add`](#add-a-marketplace), come GitHub `owner/repo`, un URL git o un percorso locale, tranne che non può contenere spazi. Dai il nome del plugin da solo, senza un suffisso `@marketplace`.

Se non hai ancora aggiunto quel marketplace, Claude Code mostra la fonte che ha risolto e ti chiede di confermare prima di aggiungerla. Una volta aggiunto il marketplace, i dettagli del plugin si aprono e scegli un [ambito di installazione](#install-a-plugin). Se la fonte corrisponde a un marketplace che hai già aggiunto, Claude Code salta la conferma e apre i dettagli del plugin in quel marketplace.

<h3 id="add-a-private-marketplace">
  Aggiungi un marketplace privato
</h3>

Un marketplace privato è uno in un repository a cui hai bisogno di credenziali per clonare, su GitHub o su qualsiasi altro host git. Lo aggiungi con lo stesso comando `/plugin marketplace add` o `claude plugin marketplace add` di uno pubblico. Claude Code lo clona con le credenziali git già sulla tua macchina e non chiede mai, quindi ogni modo di connettersi ha un requisito:

* **HTTPS**: i tuoi helper di credenziali git si applicano, quindi l'accesso che hai configurato con `gh auth login`, il Portachiavi macOS o `git-credential-store` funziona. I prompt interattivi sono soppressi, quindi un host a cui non ti sei mai autenticato fallisce invece di chiedere una password.
* **SSH**: l'host deve già essere nel tuo file `known_hosts` e la chiave deve funzionare senza un prompt di passphrase, perché i prompt di impronta digitale dell'host e di passphrase sono soppressi anche loro.
* **Scorciatoia GitHub `owner/repo`**: Claude Code controlla se la tua chiave SSH si autentica a `github.com`, quindi clona su SSH se lo fa e su HTTPS se non lo fa. Imposta [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/it/env-vars#variables) per saltare quel controllo e clonare sempre su HTTPS.

Le stesse credenziali si applicano quando esegui `/plugin install`, `/plugin marketplace update` e `claude plugin update`.

Su un host GitHub Enterprise Server, vedi [Plugin marketplaces on GHES](/docs/it/github-enterprise-server#plugin-marketplaces-on-ghes) per le credenziali che ogni operazione richiede.

Se la tua organizzazione registra il marketplace per te attraverso le impostazioni gestite, non lo aggiungi tu stesso. Vedi [Pre-install and require plugins](/docs/it/plugins/org#pre-install-and-require-plugins).

<h3 id="add-from-claude-ai">
  Aggiungi un marketplace da claude.ai
</h3>

Nelle sessioni di terminale in cui [i plugin si sincronizzano dal tuo account claude.ai](/docs/it/plugins/loading#synced-plugins), claude.ai può anche elencare i marketplace dei plugin per te, come la libreria dei plugin della tua organizzazione e i tuoi caricamenti su claude.ai. Aggiungi uno di questi per nome piuttosto che per fonte. L'aggiunta di un marketplace da claude.ai richiede Claude Code v2.1.273 o successivo.

Aggiungi un marketplace claude.ai dal pannello `/plugin` o dal tuo shell:

* **All'interno di una sessione**: esegui `/plugin` e vai alla scheda **Marketplaces**, che elenca i marketplace da claude.ai. Selezionane uno lì per aggiungerlo.
* **Dal tuo shell**: esegui `claude plugin marketplace list`, che li stampa in una sezione `From claude.ai:`. Quindi esegui `claude plugin marketplace add` con il flag `--claudeai` e il nome mostrato nell'elenco.

Ad esempio, questo comando aggiunge un marketplace denominato `claudeai-organization-library`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code registra il marketplace con un nome locale che inizia con `claudeai-`, derivato dal nome che claude.ai lo elenca. Ad esempio, un marketplace elencato come "Organization library" diventa `claudeai-organization-library`. Installa i suoi plugin per quel nome, ad esempio con `claude plugin install <plugin>@claudeai-organization-library`.

Se esci, o accedi a un'organizzazione claude.ai diversa, il marketplace rimane configurato ma non mostra alcun plugin, e i plugin che hai già installato da esso continuano a caricarsi.

La sezione `From claude.ai:` può anche elencare marketplace basati su git condivisi attraverso claude.ai, e stampa una fonte per ognuno di quelli. Aggiungili per quella fonte come in [Aggiungi un marketplace](#add-a-marketplace), non con `--claudeai`.

<h2 id="manage-installed-plugins">
  Gestisci i plugin installati
</h2>

La scheda **Installed** in `/plugin` elenca i tuoi plugin con azioni per abilitare, disabilitare, aggiornare o disinstallare ognuno. In una sessione Claude Code, esegui `/plugin` e premi **Tab** per raggiungerla, o esegui `/plugin enable`, `/plugin disable` o `/plugin uninstall` per aprire il pannello e fare quel cambiamento lì. I plugin disabilitati sono raggruppati sotto un'intestazione compressa in fondo all'elenco. Usa questi tasti sull'elenco:

* Digita per filtrare per nome o descrizione.
* Premi **Space** per abilitare o disabilitare il plugin selezionato, e **f** per contrassegnarlo come preferito.
* Premi **Enter** per aprire i dettagli di un plugin. Il menu lì offre **Disable plugin** o **Enable plugin**, **Update now** e **Uninstall**. I plugin che accettano impostazioni offrono anche **Configure options**.

La scheda può anche mostrare plugin a livello **Managed**. La tua organizzazione li ha installati attraverso [impostazioni gestite](/docs/it/settings#settings-files), e non puoi abilitarli, disabilitarli o disinstallarli qui.

Per un plugin sincronizzato che la tua organizzazione richiede su claude.ai, vedi [Gestisci i plugin sincronizzati da claude.ai](#manage-plugins-synced-from-claude-ai).

Quando chiudi il pannello `/plugin` con modifiche in sospeso che hai fatto in esso, Claude Code esegue `/reload-plugins` per te per applicarle. Se il ricaricamento [invaliderebbe la prompt cache](/docs/it/prompt-caching#enabling-or-disabling-a-plugin), avverte e lascia le modifiche in sospeso invece. Esegui `/reload-plugins --force` per applicarle comunque.

<h3 id="manage-plugins-synced-from-claude-ai">
  Gestisci i plugin sincronizzati da claude.ai
</h3>

La scheda **Installed** in `/plugin` elenca anche i [plugin sincronizzati dal tuo account claude.ai](/docs/it/plugins/loading#synced-plugins), con `synced` come loro fonte. I plugin sincronizzati appaiono nelle sessioni di terminale su Claude Code v2.1.273 o successivo.

* **Abilita o disabilita**: usa la scheda **Installed**, a meno che la tua organizzazione non abbia contrassegnato il plugin come richiesto.
* **Rimuovi**: disattiva il plugin su claude.ai.

Quando Claude Code sincronizza un plugin aggiunto, aggiornato o rimosso in una sessione interattiva, vedi `Plugins changed. Run /reload-plugins to activate.` Esegui `/reload-plugins` per caricare il cambiamento in quella sessione, o lascialo per la prossima volta che avvii Claude Code.

<h3 id="uninstall-a-plugin-the-project-enables">
  Disinstalla un plugin che il progetto abilita
</h3>

Quando scegli **Uninstall** per un plugin che il `.claude/settings.json` di questo repository abilita, sia dalla scheda **Installed** che con `/plugin uninstall`, Claude Code ti chiede se disabilitarlo per te o disinstallarlo per tutti:

* **Disable for me**: premi **y**. Claude Code scrive `false` per il plugin nel tuo `.claude/settings.local.json` e lo lascia installato per il progetto.
* **Uninstall for everyone**: premi **u**. Claude Code rimuove il plugin dal `.claude/settings.json` condiviso.

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  Vedi cosa aggiunge un plugin installato alle tue sessioni
</h3>

Nel tuo shell, esegui `claude plugin details <name>` per un plugin installato. La riga `Always-on` è il numero di token che il plugin aggiunge a ogni sessione in cui è abilitato, e le righe per componente mostrano quale skill o agent contribuisce di più. Per l'output completo e cosa significa ogni figura, vedi [Measure what a plugin costs](/docs/it/plugins/measure#measure-what-a-plugin-costs).

<h3 id="find-plugins-you-no-longer-use">
  Trova i plugin che non usi più
</h3>

Sulla scheda **Installed** in `/plugin`, i plugin che hai installato tu stesso e che non hai usato di recente appaiono sotto un'intestazione **Not used recently**, e i dettagli di ogni plugin mostrano una riga **Last used**. Usa quell'intestazione e quella riga per trovare i plugin che ancora aggiungono costo di avvio e contesto, quindi disabilitali o disinstallali.

<h3 id="plugins-with-dependencies">
  Plugin con dipendenze
</h3>

Un plugin può dichiarare altri plugin da cui dipende. Quando installi, disabiliti o disinstalli tale plugin da un marketplace, Claude Code agisce anche su quelle dipendenze:

* **Install**: Claude Code installa anche e abilita le dipendenze dichiarate del plugin allo stesso ambito. Il messaggio di successo le elenca.
* **Enable**: Claude Code abilita anche le dipendenze del plugin che sono installate ma disabilitate. Se una dipendenza dichiarata non è installata, l'abilitazione fallisce e il messaggio ti dice di installarla prima.
* **Disable**: quando un altro plugin abilitato ha ancora bisogno di quello che hai nominato, Claude Code rifiuta e stampa un comando concatenato che disabilita entrambi nell'ordine giusto.
* **Uninstall**: le dipendenze auto-installate rimangono fino a quando non esegui `claude plugin prune` nel tuo shell; vedi [plugin prune](/docs/it/plugins/cli-reference#plugin-prune).

Se hai caricato il plugin con `--plugin-dir` invece, vedi [Test a plugin and its dependency locally](/docs/it/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="manage-plugins-from-your-shell">
  Gestisci i plugin dal tuo shell
</h3>

Puoi anche gestire i plugin senza avviare una sessione Claude Code. Nel tuo shell, esegui `claude plugin install`, `enable`, `disable` o `uninstall` come comandi di terminale ordinari; cambiano le stesse impostazioni che il pannello `/plugin` fa. Ognuno accetta `--scope` per mirare a un ambito, e usa un ambito predefinito quando lo ometti:

* `enable` e `disable` agiscono sull'ambito più specifico le cui impostazioni già elencano il plugin.
* `install` e `uninstall` agiscono sull'ambito utente.

Ad esempio, questi comandi disabilitano e riabilitano un plugin, quindi lo disinstallano a livello di progetto:

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  Mantieni i plugin aggiornati
</h2>

I plugin si aggiornano automaticamente quando il marketplace da cui provengono ha l'auto-update attivato. Dopo l'avvio di una sessione, Claude Code aggiorna quei marketplace e aggiorna le copie su disco dei plugin che hai installato da essi.

La sessione in esecuzione mantiene le versioni che ha già caricato. Dopo un aggiornamento, vedi `Plugin updated: <name> · Run /reload-plugins to apply`, e la sessione successiva carica automaticamente le nuove versioni.

Questi sono i valori predefiniti di auto-update per ogni tipo di marketplace:

* **On by default**: `claude-plugins-official` e gli altri [nomi di marketplace ufficiali](/docs/it/plugins/security#official-marketplace-names) tranne `knowledge-work-plugins` e `first-party-plugins`, più [marketplace aggiunti da claude.ai](#add-from-claude-ai).
* **Off by default**: ogni altro marketplace, incluso il marketplace della comunità, i marketplace di terze parti e i marketplace di sviluppo locale.

Per quando viene eseguito l'auto-update, quali plugin salta e le variabili di ambiente che lo disattivano, vedi [When auto-update runs](/docs/it/plugins/loading#when-auto-update-runs).

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  Attiva o disattiva l'auto-update per un marketplace
</h3>

In una sessione Claude Code, esegui `/plugin` e vai alla scheda **Marketplaces**. Seleziona il marketplace, quindi seleziona **Enable auto-update** o **Disable auto-update**.

<h3 id="update-one-plugin-now">
  Aggiorna un plugin ora
</h3>

In una sessione, apri il plugin sulla scheda **Installed** in `/plugin` e seleziona **Update now**, o nel tuo shell esegui `claude plugin update <plugin>@<marketplace>`.

<h3 id="auto-update-from-a-private-marketplace">
  Auto-update da un marketplace privato
</h3>

Per un marketplace privato, vedi [What background auto-update does with credentials](/docs/it/plugins/host-marketplace#what-background-auto-update-does-with-credentials) per come gli auto-update in background si autenticano su SSH e HTTPS, e [Troubleshoot plugins](/docs/it/plugins/troubleshooting#add-a-marketplace) per i messaggi che vedi quando falliscono.

<h2 id="manage-marketplaces">
  Gestisci i marketplace
</h2>

La scheda **Marketplaces** in `/plugin` elenca ogni marketplace che hai registrato, insieme alla sua fonte. Selezionane uno per sfogliare i suoi plugin, aggiornare il suo elenco, attivare o disattivare l'auto-update, o rimuoverlo.

Puoi anche elencare, aggiornare e rimuovere i marketplace con comandi, dal tuo shell o all'interno di una sessione:

| Azione                              | Nel tuo shell                             | All'interno di una sessione         |
| :---------------------------------- | :---------------------------------------- | :---------------------------------- |
| Elenca i marketplace                | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| Aggiorna l'elenco di un marketplace | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| Rimuovi un marketplace              | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

Quando rimuovi un marketplace, Claude Code disinstalla ogni plugin che hai installato da esso e rimuove le loro voci `enabledPlugins` dai tuoi file di impostazioni. La scheda **Marketplaces** nomina quei plugin prima di chiederti di confermare.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Marketplace di Anthropic](/docs/it/plugins/anthropic-marketplaces): come differiscono i marketplace ufficiale, della comunità e demo e dove sfogliare ognuno
* [Plugin loading reference](/docs/it/plugins/loading): perché un plugin si è caricato, non si è caricato o non è cambiato dopo un aggiornamento
* [Plugin security and trust](/docs/it/plugins/security): cosa rivedere prima di installare un plugin da un marketplace che non conosci
* [Troubleshoot plugins](/docs/it/plugins/troubleshooting): messaggi di errore di installazione e marketplace con le loro correzioni
* [Create a plugin](/docs/it/plugins/create): costruisci il tuo
