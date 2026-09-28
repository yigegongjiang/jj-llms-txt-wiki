> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ospitare e mantenere un marketplace

> Pubblica un marketplace di plugin dove gli utenti possono raggiungerlo, concedi accesso a uno privato e rilascia aggiornamenti e ridenominazioni senza interrompere le installazioni.

Ospitare un marketplace significa mettere il tuo catalogo `marketplace.json` dove altre persone possono aggiungerlo con `/plugin marketplace add`, installare i suoi plugin e continuare a ricevere i tuoi cambiamenti dopo che li hai pubblicati.

Questa pagina è per la persona che gestisce un marketplace.

<Note>
  Questi casi sono trattati su altre pagine:

  * **Non hai ancora scritto il file di catalogo**: inizia con [Crea un marketplace](/docs/it/plugins/create-marketplace)
  * **Sei un amministratore che richiede, limita o pre-installa marketplace su tutte le macchine della tua organizzazione**: leggi [Gestisci i plugin per la tua organizzazione](/docs/it/plugins/org)
</Note>

Inizia con [Ospita il tuo marketplace](#host-your-marketplace) per scegliere un host e il comando che i tuoi utenti eseguono. Leggi [Mantieni gli utenti aggiornati](#keep-users-up-to-date) prima della tua prima versione. Leggi [Rinomina o rimuovi un plugin](#rename-or-remove-a-plugin) prima di modificare il `name` di un plugin.

<h2 id="host-your-marketplace">
  Host your marketplace
</h2>

Puoi ospitare il marketplace su GitHub, su un altro host git, come URL `marketplace.json` ospitato, o in una directory su un filesystem condiviso. Invia ai tuoi utenti il comando add per il tuo host e comunica loro cosa serve sulla loro macchina:

| Host                                                             | Gli utenti eseguono, in una sessione Claude Code                       | Cosa serve agli utenti                                                                                                                    |
| :--------------------------------------------------------------- | :--------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| GitHub                                                           | `/plugin marketplace add your-org/your-marketplace`                    | `git`, e per un repository privato l'accesso descritto in [Grant access to a private marketplace](#grant-access-to-a-private-marketplace) |
| GitLab, Bitbucket, GitHub Enterprise Server, o un altro host git | `/plugin marketplace add https://gitlab.example.com/team/plugins.git`  | `git`, e accesso all'host dalla loro macchina. Invia l'URL completo, perché la scorciatoia `owner/repo` significa sempre github.com       |
| Un URL `marketplace.json` ospitato                               | `/plugin marketplace add https://plugins.example.com/marketplace.json` | Accesso HTTPS all'URL. Gli utenti non hanno bisogno di `git` per il catalogo stesso                                                       |
| Una directory su un filesystem condiviso                         | `/plugin marketplace add /Volumes/shared/claude-plugins`               | Accesso in lettura al percorso                                                                                                            |

Per fissare un branch o un tag di un marketplace GitHub o git-URL, comunica agli utenti di aggiungere `#<ref>`, come in `your-org/your-marketplace#stable`. Il [plugin commands reference](/docs/it/plugins/cli-reference#plugin-marketplace-add) elenca ogni forma che il comando accetta.

Un add riuscito stampa `Successfully added marketplace: your-marketplace`. Claude Code prende quel nome dal campo `name` nel tuo `marketplace.json`, non dal nome del repository.

Gli utenti quindi installano un plugin per il `name` della voce e il `name` del marketplace, come in `/plugin install code-formatter@your-marketplace`.

<h3 id="register-the-marketplace-for-everyone-in-a-repository">
  Register the marketplace for everyone in a repository
</h3>

Per condividere il marketplace con tutti coloro che lavorano in un repository, esegui `claude plugin marketplace add your-org/your-marketplace --scope project` lì una volta dalla tua shell e committa il `.claude/settings.json` che scrive. Claude Code quindi registra il marketplace per ogni collega che [trusts the folder](/docs/it/plugins/org#require-plugins-per-repository).

<h3 id="avoid-relative-path-entries-in-a-url-hosted-marketplace">
  Avoid relative-path entries in a URL-hosted marketplace
</h3>

Quando gli utenti aggiungono il tuo marketplace come URL `marketplace.json` nudo, Claude Code scarica solo quel file. Una voce nel tuo array `plugins` il cui `source` è un percorso relativo come `./plugins/formatter` fallisce quindi all'installazione con [`its marketplace entry path does not stay inside the marketplace directory`](/docs/it/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces). Dai a ogni voce un source che può essere recuperato da solo, come un repository `github` o un URL `archive`, oppure ospita il marketplace in un repository git in modo che Claude Code cloni l'intero albero.

<h3 id="edit-plugins-in-place-on-a-shared-directory">
  Edit plugins in place on a shared directory
</h3>

Quando gli utenti aggiungono il tuo marketplace da una directory condivisa, Claude Code legge i plugin con source a percorso relativo direttamente da quella directory invece di copiarli. Gli utenti vedono i tuoi modifiche quando avviano la prossima sessione o eseguono `/reload-plugins`, senza un passaggio di aggiornamento o un bump di versione.

<h3 id="keep-plugin-files-out-of-git-lfs">
  Keep plugin files out of Git LFS
</h3>

Mantieni i file di cui i tuoi plugin hanno bisogno fuori da [Git LFS](https://git-lfs.com). Quando gli utenti aggiungono un marketplace ospitato in un repository git, o installano un plugin basato su git che elenca, Claude Code clona quel marketplace o repository di plugin sulla loro macchina. Il clone non scarica mai il contenuto LFS, quindi i file tracciati da LFS arrivano come file puntatore.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Share files within a marketplace with symlinks
</h3>

Per condividere file tra il tuo plugin e altre parti dello stesso marketplace, crea link simbolici all'interno della directory del tuo plugin. Quando Claude Code copia il plugin nella sua cache, gestisce ogni symlink per dove il target si risolve:

* **All'interno della directory del plugin stesso**: il symlink viene preservato come symlink relativo nella cache, quindi continua a risolvere il target copiato al runtime.
* **Altrove all'interno dello stesso marketplace**: il symlink viene dereferenziato. Il contenuto del target viene copiato nella cache al suo posto. Questo consente alla directory `skills/` di un meta-plugin di collegarsi a skill definite da altri plugin nel marketplace.
* **Fuori dal marketplace**: il symlink viene saltato per motivi di sicurezza.

Per i plugin installati da un percorso locale, o da una [`command` source](/docs/it/plugins/marketplace-reference#command-plugin-source) il cui `mode` è il default `copy`, Claude Code preserva solo i symlink che si risolvono all'interno della directory del plugin stesso e salta tutti gli altri.

Il seguente comando crea un link dall'interno di un plugin marketplace a una skill condivisa definita da un plugin fratello. Su Windows, usa `mklink /D` da un Command Prompt elevato o abilita Developer Mode:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

<h2 id="distribute-through-organization-settings">
  Distribute through organization settings
</h2>

Su un piano Team o Enterprise, puoi anche distribuire il marketplace attraverso [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) su claude.ai invece di ospitarlo dove gli utenti lo aggiungono da soli. Organization sync legge il repository attraverso la connessione GitHub o GitLab della tua organizzazione su claude.ai, quindi le credenziali git dei tuoi utenti non sono coinvolte.

Organization sync è più rigoroso riguardo al repository rispetto a `/plugin marketplace add`:

* **Repository marketplace**: su github.com e gitlab.com, deve essere privato o interno
* **Plugin sources**: ogni plugin source deve essere di tipo `github`, `url`, o `git-subdir`, o un [relative path](/docs/it/plugins/marketplace-reference#relative-path-plugin-source) che inizia con `./`
* **Directory `bin/` di primo livello**: claude.ai rifiuta un plugin che ne ha una e sincronizza il resto del marketplace. Il messaggio di errore inizia con `Plugin contains a top-level bin/ directory`. Mantieni gli eseguibili in un'altra directory, come `scripts/`, e fai riferimento ad essi come `${CLAUDE_PLUGIN_ROOT}/scripts/<name>` dai tuoi hooks o dalle configurazioni del server MCP

Vedi [Manage plugins for your organization](https://support.claude.com/en/articles/13837433) per il flusso di lavoro dell'amministratore.

<h2 id="grant-access-to-a-private-marketplace">
  Grant access to a private marketplace
</h2>

Quando un utente aggiunge, installa da, o aggiorna il tuo marketplace, Claude Code esegue `git` sulla loro macchina con i prompt interattivi disattivati e si affida a qualsiasi credenziale quella macchina già possiede. Claude Code non ha un token git proprio, e `marketplace.json` non ha un campo per uno.

Scegli se il clone viene eseguito su SSH o HTTPS dalla forma del comando add che invii agli utenti:

* **GitHub `owner/repo`**: Claude Code verifica `ssh -T git@github.com` e clona su SSH quando la verifica ha successo. Se la verifica fallisce, o il clone SSH stesso fallisce, clona su HTTPS. Gli utenti su macchine senza una chiave SSH GitHub possono impostare `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` per saltare la verifica e clonare su HTTPS.
* **`git@host:path.git`**: SSH.
* **`https://example.com/repo.git`**: HTTPS.

Comunica agli utenti cosa serve per ogni protocollo sulla loro macchina:

* **SSH**: la chiave deve funzionare senza un prompt di passphrase, ad esempio perché è caricata in `ssh-agent`. L'host deve già essere in `known_hosts`.
* **HTTPS**: Claude Code lascia abilitato l'helper di credenziali git dell'utente ma gli vieta di chiedere. Una credenziale che l'helper già memorizza funziona; una che dovrebbe chiedere fallisce. Su GitHub, `gh auth login` seguito da `gh auth setup-git` ne memorizza una.

Per un host GitHub Enterprise Server, gli utenti hanno bisogno dell'accesso git a quell'host dalla loro macchina. Vedi [Plugin marketplaces on GHES](/docs/it/github-enterprise-server#plugin-marketplaces-on-ghes) per cosa serve a ogni superficie Claude Code per raggiungere un marketplace ospitato su GHES.

Se distribuisci attraverso **Organization settings > Plugins & skills** su claude.ai invece, le credenziali git dei tuoi utenti non sono coinvolte. Vedi [Distribute through organization settings](#distribute-through-organization-settings) per quali plugin sources possono essere privati lì.

<h3 id="serve-users-who-have-no-git-host-account">
  Serve users who have no git-host account
</h3>

Gli utenti senza un account su un host git possono aggiungere un marketplace che servi come URL `marketplace.json` o da una directory condivisa, ma possono installare solo i plugin le cui voci source possono anche raggiungere. Una voce che punta a un repository `github` privato fallisce comunque all'installazione per loro, perché Claude Code lo recupera con lo stesso `git` non interattivo che usa per un marketplace ospitato su git.

Queste entry sources non hanno bisogno di un account git:

* **`archive`**: uno zip scaricato su HTTPS. Gli utenti non hanno bisogno né di `git` né di un account, solo accesso di rete all'URL. Richiede Claude Code v2.1.224 o successivo. Fissa ogni archive con `sha256` in modo che Claude Code rifiuti un download modificato. Per inviare credenziali con il download, vedi [Authenticate archive downloads](#authenticate-archive-downloads).
* **Un repository git pubblico**: Claude Code clona una source `url` o `git-subdir` pubblica su HTTPS senza credenziali quando la voce fornisce un URL `https://`. Per una source `github`, o una source `git-subdir` scritta come `owner/repo`, gli utenti senza una chiave SSH GitHub impostano `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`.

Per un team su una rete, un marketplace `directory` su un filesystem condiviso funziona anche senza account git. Gli utenti hanno bisogno solo dell'accesso in lettura al percorso.

<h3 id="what-background-auto-update-does-with-credentials">
  What background auto-update does with credentials
</h3>

Background auto-update è l'aggiornamento automatico e incustodito di Claude Code dei marketplace e dei plugin installati dopo l'avvio di una sessione. È disattivato per il tuo marketplace finché un utente o un amministratore non lo attiva, come coperto in [Keep users up to date](#keep-users-up-to-date).

Quando è attivato per un marketplace privato, il controllo di background per i nuovi commit utilizza gli helper di credenziali git configurati dall'utente e non chiede mai. Ogni tipo di remote e helper dà un risultato diverso:

* **SSH remotes**: una chiave caricata in `ssh-agent` autentica il controllo.
* **HTTPS remotes con una credenziale memorizzata**: un helper che può fornire una credenziale memorizzata senza chiedere autentica il controllo. Git Credential Manager, l'helper Keychain di macOS, e `git-credential-store` funzionano in questo modo una volta che detengono una credenziale per l'host.
* **HTTPS remotes con un helper che ha bisogno di chiedere**: l'helper non può rispondere in background. L'aggiornamento fallisce silenziosamente e il checkout esistente rimane in posizione, quindi i plugin dell'utente continuano a funzionare dallo stato sincronizzato per ultimo.

Dopo il controllo, Claude Code fa uno dei seguenti:

* **Il checkout è aggiornato**: Claude Code lo lascia come è.
* **Il controllo trova nuovi commit, o fallisce perché non può raggiungere o autenticarsi al remote**: Claude Code clona il marketplace di nuovo e sostituisce il checkout esistente con il nuovo clone. Se quel clone fallisce, il checkout esistente rimane in posizione. Il re-clone può [time out on large repositories](/docs/it/plugins/troubleshooting#git-clone-timed-out-after-120s).

Per mantenere un marketplace privato aggiornato, un utente può fare uno dei seguenti:

* **Memorizzare una credenziale**: accedi all'helper di credenziali per primo in modo che memorizzi una credenziale per l'host. Per GitHub, esegui `gh auth login`, quindi `gh auth setup-git`.
* **Mantenere il checkout al fallimento**: se l'utente imposta `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`, Claude Code mantiene il checkout esistente senza tentare il re-clone quando il controllo di background non può raggiungere o autenticarsi al remote. I plugin continuano a funzionare dallo stato sincronizzato per ultimo.

Se un utente imposta `GITHUB_TOKEN` o un altro token del provider nell'ambiente, questo da solo non autentica il controllo di background. Un token ha effetto attraverso un helper di credenziali, come l'helper della CLI `gh`, che legge `GH_TOKEN` e `GITHUB_TOKEN`.

<h2 id="roll-out-to-a-whole-company">
  Roll out to a whole company
</h2>

Distribuire un plugin a un'intera azienda coinvolge te come proprietario del marketplace, un amministratore che controlla le impostazioni gestite, e ogni persona che usa Claude Code. Puoi eseguire il rollout senza l'amministratore, nel qual caso ogni persona aggiunge il marketplace e installa il plugin da sola.

| Chi                                 | Cosa fa                                                                                                                                                                              | Dove è coperto                                                                                                                                          |
| :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Tu, il proprietario del marketplace | Mantieni il catalogo in un repository che solo l'azienda può leggere, invia il comando add per il tuo host, e comunica cosa serve a ogni persona sulla loro macchina                 | [Host your marketplace](#host-your-marketplace) e [Grant access to a private marketplace](#grant-access-to-a-private-marketplace)                       |
| Un amministratore                   | Registra il marketplace e attiva i suoi plugin per tutti con `extraKnownMarketplaces` e `enabledPlugins` nelle impostazioni gestite, e imposta `autoUpdate` lì                       | [Require a marketplace and its plugins](/docs/it/plugins/org#require-a-marketplace-and-its-plugins) e [Set update policy](/docs/it/plugins/org#set-update-policy) |
| Ogni persona                        | Ha bisogno dell'accesso in lettura a un repository git privato, con credenziali già memorizzate sulla loro macchina. Senza un amministratore, eseguono anche i comandi add e install | [Add a private marketplace](/docs/it/plugins/install#add-a-private-marketplace)                                                                              |

Per le persone che non hanno un account su un host git, queste sezioni coprono ciascuna un modo per raggiungerle:

* **Entry sources che non hanno bisogno di un account git**: [Serve users who have no git-host account](#serve-users-who-have-no-git-host-account)
* **Una directory di plugin pre-popolata**: [Seed containers and CI](/docs/it/plugins/org#seed-containers-and-ci), che serve anche gli utenti che non hanno un account su un host git
* **Impostazioni dell'organizzazione claude.ai**: [Distribute through organization settings](#distribute-through-organization-settings), dove le credenziali git dei tuoi utenti non sono coinvolte

<h2 id="keep-users-up-to-date">
  Keep users up to date
</h2>

I tuoi cambiamenti raggiungono gli utenti attraverso background auto-update, una volta che è attivato per il tuo marketplace, o quando gli utenti aggiornano il plugin da soli. In entrambi i casi un utente ottiene una nuova copia di un plugin solo quando la sua versione calcolata cambia, come descritto in [Release a new version](#release-a-new-version).

<h3 id="turn-on-auto-update">
  Turn on auto-update
</h3>

Background auto-update è disattivato per il tuo marketplace per impostazione predefinita, e `marketplace.json` non ha un campo per attivarlo. Un utente o un amministratore lo attiva:

* **Comunica agli utenti di attivarlo**: ogni utente va a **Marketplaces** in `/plugin`, seleziona il tuo marketplace, e seleziona **Enable auto-update**.
* **Chiedi a un amministratore di impostarlo**: se un amministratore imposta `"autoUpdate": true` sulla voce `extraKnownMarketplaces` del tuo marketplace nelle impostazioni gestite, è attivato per tutti coloro che ricevono quelle impostazioni. Vedi [Set update policy](/docs/it/plugins/org#set-update-policy).

Senza auto-update, gli utenti ricevono i tuoi cambiamenti quando eseguono `/plugin marketplace update <name>` in una sessione o `claude plugin update <plugin>@<name>` nella shell.

Per cosa vedono gli utenti quando un aggiornamento li raggiunge, vedi [When auto-update runs](/docs/it/plugins/loading#when-auto-update-runs).

<h3 id="release-a-new-version">
  Release a new version
</h3>

Per rilasciare una nuova versione agli utenti, cambia il `version` del plugin. Gli utenti ottengono una nuova copia solo quando la versione calcolata del plugin differisce da quella che hanno. Quella versione viene da `plugin.json` per primo, poi dalla voce del marketplace, per [Versions and updates](/docs/it/plugins/loading#versions-and-updates).

Un plugin che gli utenti [load in place](/docs/it/plugins/loading#find-plugins-on-disk) da un marketplace che hanno aggiunto come directory locale non è controllato da `version`. Carica i tuoi file attuali a ogni avvio di sessione, qualunque sia la sua stringa di versione.

Per ogni installazione diversa da un caricamento in place o uno da una source `command`, aumenta `version` a ogni rilascio o omettilo:

* **Bump `version` a ogni rilascio**: gli utenti rimangono sulla loro copia in cache finché la stringa non cambia. Se imposti `"version": "1.0.0"` e spingi nuovi commit senza cambiarlo, gli utenti non li ricevono.
* **Ometti `version`**: gli utenti tracceranno i tuoi commit invece. Lascia `version` fuori sia da `plugin.json` che dalla voce del marketplace.

Non impostare `version` sia in `plugin.json` che nella voce del marketplace. Se lo fai, Claude Code usa il valore `plugin.json` senza avvertimento, e `claude plugin validate` segnala la mancata corrispondenza come `Entry declares version "<a>" but <path>/plugin.json says "<b>"`.

<h3 id="hold-users-on-one-version">
  Hold users on one version
</h3>

Un marketplace serve una versione di ogni plugin alla volta, quindi tieni gli utenti su una versione scegliendo cosa ogni voce punta:

* **`ref` e `sha` sulla voce del plugin**: `ref` nomina un branch o un tag e `sha` nomina un commit per una source `github`, `url`, o `git-subdir`. Vedi [Plugin sources](/docs/it/plugins/marketplace-reference#plugin-sources).
* **`#<ref>` sul comando add**: gli utenti che aggiungono `your-org/your-marketplace#stable` ottengono quel branch o tag del catalogo. Per due linee di rilascio contemporaneamente, vedi [Run release channels](#run-release-channels).
* **Tag `<plugin>--v<version>`**: un intervallo di versione di una dipendenza si risolve rispetto a questi tag. Vedi [Release a plugin that others depend on](/docs/it/plugins/dependencies#tag-plugin-releases-for-version-resolution).

[Release a new version](#release-a-new-version) dice quando una voce modificata raggiunge gli utenti.

<h3 id="change-the-command-of-a-command-source">
  Change the command of a command source
</h3>

Se cambi il `command` di una [`command` source](/docs/it/plugins/marketplace-reference#command-plugin-source), o cambi il suo `mode`, ogni utente deve accettare il nuovo comando prima che Claude Code lo esegua. Claude Code esegue solo il comando esatto che un utente ha accettato quando ha installato o aggiornato per ultimo il plugin.

Dopo che la copia del marketplace di un utente raccoglie il cambiamento, quell'utente vede uno dei seguenti:

* **Nessuna più esecuzione di background**: l'[esecuzione una volta per sessione](/docs/it/plugins/loading#when-a-command-source-re-runs) del comando si ferma per quell'utente, quindi l'output nuovo dello strumento non li raggiunge.
* **Una voce nella scheda `/plugin` Errors**: la voce mostra il nuovo comando e il comando `claude plugin update` da eseguire.

Comunica agli utenti di eseguire il comando `claude plugin update` che quella voce mostra, in un terminale. Claude Code mostra loro il nuovo comando e chiede loro di accettarlo.

<h2 id="run-release-channels">
  Run release channels
</h2>

Per offrire tracce stabili e ad accesso anticipato, ospita due marketplace le cui voci puntano a ref diversi dello stesso plugin, e lascia che ogni utente aggiunga quello che vuole. Claude Code non ha un concetto di canale di rilascio, e un marketplace serve una versione di ogni plugin alla volta.

Dai ai due file `marketplace.json` valori `name` diversi. Claude Code identifica un marketplace per il suo `name`, quindi un utente non può avere due marketplace con lo stesso nome registrati contemporaneamente.

Con questi due cataloghi, gli utenti che aggiungono `stable-tools` installano `code-formatter` dal branch `stable`, e gli utenti che aggiungono `latest-tools` lo installano da `latest`:

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

Dai ai due ref versioni `plugin.json` diverse, o ometti `version` in modo che lo SHA del commit li distingua. Gli aggiornamenti vengono rilevati confrontando le versioni, quindi un ref che si muove senza un cambio di versione lascia gli utenti sulla copia in cache.

Per assegnare i canali ai gruppi di utenti invece di lasciare che gli utenti scelgano, un amministratore dà a ogni gruppo la voce `extraKnownMarketplaces` corrispondente, come descritto in [Set update policy](/docs/it/plugins/org#set-update-policy).

<h2 id="rename-or-remove-a-plugin">
  Rename or remove a plugin
</h2>

Il `name` di un plugin è il suo identificatore. Gli utenti lo referenziano nelle chiavi di impostazione `enabledPlugins` e `pluginConfigs` e in `/plugin install`, quindi cambiarlo interrompe ogni installazione esistente.

Per cambiare l'etichetta che gli utenti vedono in `/plugin` senza interrompere nulla, imposta `displayName` in `plugin.json` e mantieni `name` invariato.

<h3 id="migrate-users-with-a-renames-map">
  Migrate users with a renames map
</h3>

Quando devi cambiare un `name`, aggiungi una mappa `renames` di primo livello a `marketplace.json` in modo che Claude Code migri gli utenti esistenti invece di segnalare [`Plugin "<name>" not found in marketplace`](/docs/it/plugins/troubleshooting#plugin-not-found-in-marketplace). Fai lo stesso quando rimuovi una voce da `plugins`. La migrazione automatica richiede Claude Code v2.1.193 o successivo.

Mappa ogni nome precedente al suo nome attuale, o a `null` quando il plugin è scomparso. Questo marketplace rinomina `formatter` a `code-formatter` e registra che `legacy-linter` è stato rimosso:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Dopo che spingi, un utente che ha ancora il vecchio nome abilitato vede uno di questi risultati:

* **Voce rinominata**: il plugin carica con il suo nuovo nome. `claude plugin list` e i dettagli del plugin in `/plugin` mostrano `Renamed to "code-formatter" in the "your-marketplace" marketplace` una volta, e Claude Code riscrive la vecchia chiave a quella nuova in `enabledPlugins` e `pluginConfigs` negli ambiti di impostazioni utente, progetto e locale.
* **Voce `null`**: la vecchia chiave viene eliminata da quegli ambiti e l'utente vede `Removed from the "your-marketplace" marketplace`.
* **Abilitato nelle impostazioni gestite**: il plugin continua a caricare con il suo nuovo nome, ma Claude Code non può riscrivere le impostazioni gestite, quindi l'avviso ricorre finché un amministratore non aggiorna `enabledPlugins` lì.

Per un marketplace che gli utenti hanno aggiunto da un repository git o URL, un plugin rinominato segnala [`Plugin "<name>" not cached at <path>`](/docs/it/plugins/troubleshooting#plugin-not-cached-at) finché l'utente non esegue `/plugin install code-formatter@your-marketplace` una volta in una sessione.

Tratta `renames` come storia di sola aggiunta. Mantieni le voci vecchie dopo che tutti hanno migrato. Quando rinomini di nuovo, aggiungi una seconda voce piuttosto che modificare la prima, perché Claude Code segue la catena dal nome più vecchio.

Nella tua shell, esegui `claude plugin validate .` dopo aver modificato la mappa. Rifiuta una catena che cicla o che termina da qualsiasi parte diversa da `null` o un nome in `plugins`, con `renames.<name>: chain does not resolve`.

<h3 id="uninstall-removed-plugins-from-users’-machines">
  Uninstall removed plugins from users' machines
</h3>

Per disinstallare un plugin rimosso dalle macchine degli utenti piuttosto che lasciare una copia dietro, imposta `"forceRemoveDeletedPlugins": true` al primo livello di `marketplace.json`. Senza il campo, un plugin rimosso rimane installato e segnala `Plugin "<name>" not found in marketplace` quando una sessione lo carica. Con esso, Claude Code fa quanto segue a ogni avvio di sessione:

1. Confronta cosa gli utenti hanno installato dal tuo marketplace rispetto alle voci e alla mappa `renames`, e tratta qualsiasi plugin che non è né elencato né rinominato come rimosso.
2. Disinstalla ogni plugin rimosso dagli ambiti utente, progetto e locale. I plugin che solo le impostazioni gestite hanno installato rimangono in posizione.
3. Elenca ogni plugin rimosso sotto un'intestazione **Flagged** in `/plugin` con lo stato `Removed from marketplace`.

<h2 id="authenticate-archive-downloads">
  Authenticate archive downloads
</h2>

Per autenticare un download [`archive`](/docs/it/plugins/marketplace-reference#archive-plugin-source), come un download da un registro privato, imposta le intestazioni HTTP che Claude Code invia con esso. Puoi impostare `headers` in uno di questi posti:

* **La source `url` del marketplace**: la source `url` da cui hai registrato il marketplace, come una voce [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces).
* **La voce del plugin**: su Claude Code v2.1.238 o successivo, puoi impostarla sulla voce `marketplace.json` del plugin invece, accanto a `source`.

In uno di questi posti, imposta un comando `headersHelper` invece di `headers` quando il valore è di breve durata, come un token che il tuo registro genera su richiesta. Claude Code esegue il comando e invia l'oggetto JSON che stampa come intestazioni di quel posto. Richiede Claude Code v2.1.238 o successivo.

Il [marketplace reference](/docs/it/plugins/marketplace-reference#plugin-entries) elenca i campi di voce `headers` e `headersHelper`.

Il posto che scegli decide quali download ottengono le intestazioni e quando Claude Code esegue il comando:

| Posto                        | Download che ottengono le intestazioni                                                                 | Quando Claude Code esegue un `headersHelper` impostato lì                                                                                                                          |
| :--------------------------- | :----------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Source `url` del marketplace | Download di archive sull'origine dell'URL del marketplace, significando lo stesso schema, host e porta | Prima di ogni fetch del `marketplace.json` del marketplace e prima di ogni download di archive su quell'origine. Claude Code riusa l'output di un'esecuzione per fino a 60 secondi |
| Voce del plugin              | Solo il download di quella voce                                                                        | Solo quando un utente installa o aggiorna quel singolo plugin da solo e [accetta il comando](#how-users-accept-a-headershelper-command)                                            |

Dove entrambi i posti impostano un'intestazione dello stesso nome, Claude Code invia il valore della voce. All'interno di un posto, un'intestazione che il comando stampa sostituisce un'intestazione dello stesso nome elencata in `headers`.

<h3 id="add-a-headershelper-to-a-plugin-entry">
  Add a headersHelper to a plugin entry
</h3>

Questa voce imposta `headersHelper` accanto a `source`. Imposta anche [`"strict": false`](/docs/it/plugins/marketplace-reference#strict-mode), che Claude Code richiede di una voce `marketplace.json` che imposta `headersHelper`:

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

Per controllare la voce, esegui `claude plugin install my-plugin@your-marketplace` nella tua shell. Claude Code ti mostra il comando e l'URL dell'archive, e scarica lo zip dopo che accetti.

<h3 id="write-the-headershelper-command">
  Write the headersHelper command
</h3>

Che tu imposti `headersHelper` su una source `url` del marketplace o su una voce del plugin, scrivi il comando per soddisfare questi requisiti:

* **Testo del comando**: al massimo 500 caratteri di ASCII stampabile, senza una sequenza di quattro o più spazi.
* **Output**: stampa un oggetto JSON di nomi di intestazione e valori di stringa su stdout, quindi esci 0 entro 10 secondi.
* **Shell e directory di lavoro**: Claude Code esegue il comando attraverso `sh`, o attraverso `cmd.exe` su Windows. La directory di lavoro è la directory di configurazione, che è `~/.claude` o [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars#variables). Dai un percorso assoluto o un comando su `PATH`, perché un percorso relativo si risolve rispetto a quella directory, non al progetto dell'utente.
* **Variabili che Claude Code rimuove**: quando il comando è impostato in una voce `marketplace.json`, o nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto, Claude Code rimuove dall'ambiente ogni variabile il cui nome sembra una credenziale, per la [stessa regola che applica a un MCP `headersHelper`](/docs/it/mcp#which-variables-a-helper-can-read). `ANTHROPIC_API_KEY` e `MY_REGISTRY_TOKEN` sono entrambi rimossi, quindi fai leggere la credenziale al comando da un file o da un archivio di credenziali. Questa rimozione non si applica a un comando impostato nelle impostazioni utente, un file `--settings`, o impostazioni gestite.
* **Variabili che Claude Code imposta**: `CLAUDE_CODE_MARKETPLACE_URL` e `CLAUDE_CODE_MARKETPLACE_NAME` per il comando di una source `url`, e `CLAUDE_CODE_PLUGIN_NAME` e `CLAUDE_CODE_PLUGIN_ARCHIVE_URL` per il comando di una voce. `CLAUDE_CODE_MARKETPLACE_NAME` non è impostato al primo fetch dopo che un utente aggiunge un marketplace per URL, perché quel fetch è quello che fornisce il nome.

Un comando che conia un token bearer stampa un oggetto come questo:

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

<h3 id="when-claude-code-skips-a-headershelper-command-or-drops-its-output">
  When Claude Code skips a headersHelper command or drops its output
</h3>

Un comando `headersHelper` non viene eseguito, o le intestazioni da `headers` o dall'output del comando vengono eliminate, quando uno dei seguenti si applica:

* **Il comando fallisce**: se il comando esce non-zero, viene eseguito oltre 10 secondi, o stampa qualcosa di diverso da un oggetto JSON di valori di stringa, il fetch o il download per il quale il comando è stato eseguito non accade.
* **L'URL del marketplace non inizia con `https://`**: il comando di quella source `url` non viene eseguito, e le richieste portano solo le intestazioni elencate nel suo campo `headers`.
* **Il reindirizzamento lascia l'origine**: quando un download viene reindirizzato fuori dall'origine dell'URL dell'archive, la richiesta reindirizzata non porta valori `headers` o output del comando da nessuno della source `url` del marketplace o della voce del plugin.
* **La voce imposta un'intestazione di routing o identità**: Claude Code elimina i nomi di routing di richiesta e identità del client come `Host`, `Cookie`, e `X-Forwarded-*` da `headers` e dall'output del comando di una voce, e mantiene i nomi di autenticazione come `Authorization`. Ogni voce `marketplace.json` viene filtrata in questo modo. Per una voce di plugin inline nelle impostazioni, vedi [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces).
* **Il comando è impostato nelle impostazioni di una directory `--add-dir`**: il comando viene ignorato, su una source `url` e su una [voce di plugin inline](/docs/it/settings-reference#extraknownmarketplaces) allo stesso modo, e solo le `headers` di quel file vengono inviate.
* **Le impostazioni gestite bloccano il comando**: impostare [`disableCommandPluginSources`](/docs/it/settings-reference#disablecommandpluginsources) a `true` blocca i comandi `headersHelper`, e [`allowManagedHooksOnly`](/docs/it/settings-reference#allowmanagedhooksonly) li blocca anche a meno che `disableCommandPluginSources` non sia esplicitamente `false`. Sotto uno di questi blocchi, Claude Code continua a eseguire il comando per un marketplace che le impostazioni gestite stesse dichiarano.

<h3 id="how-users-accept-a-headershelper-command">
  How users accept a headersHelper command
</h3>

Un utente accetta il comando di una voce del plugin ogni volta che installa o aggiorna quel singolo plugin da solo. Lo fanno dalla vista propria del plugin in `/plugin`, o con `claude plugin install` o `claude plugin update`. Claude Code mostra il comando e l'URL dell'archive, ed esegue il comando solo dopo che l'utente accetta.

In una shell non interattiva, passa [`--yes`](/docs/it/plugins/cli-reference#plugin-install) per accettare il comando. Per accettare solo il comando che un'esecuzione `--json` precedente ha visualizzato, passa [`--accept-command`](/docs/it/plugins/cli-reference#plugin-install) con lo `sha256` che l'esecuzione ha segnalato.

Claude Code esegue solo il comando che ha mostrato, per l'URL dell'archive che ha mostrato. Se il comando della voce o l'URL dell'archive sono cambiati nel frattempo, Claude Code rifiuta l'installazione o l'aggiornamento. Un cambiamento nella stringa di query da solo non conta.

<h3 id="installs-and-updates-that-refuse-the-command-instead-of-asking">
  Installs and updates that refuse a command instead of asking
</h3>

Su qualsiasi operazione diversa da un'installazione o aggiornamento di un singolo plugin, Claude Code non esegue il comando di una voce né scarica il suo archive. Il plugin rimane alla sua versione installata o rimane disinstallato, e l'utente vede uno di questi risultati:

* **Installazione di diversi plugin contemporaneamente, da un suggerimento di plugin, o come dipendenza di un altro plugin**: Claude Code rifiuta il plugin che ha il comando e indirizza l'utente alla vista propria di quel plugin in `/plugin`. Gli altri plugin in un'installazione in blocco continuano a installare. Un plugin che dipende dal plugin rifiutato non riesce a installare finché l'utente non installa il plugin rifiutato da solo.
* **Background auto-update, o avvio della sessione per un plugin il cui archive non è mai stato scaricato**: Claude Code elenca il plugin nella scheda `/plugin` Errors in modo che l'utente sappia di installarlo o aggiornarlo da solo.

<h3 id="when-a-marketplace-url-sources-command-runs">
  When a marketplace `url` source's command runs
</h3>

Dichiari il `headersHelper` di una source `url` del marketplace in un file di impostazioni, come una voce [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces), piuttosto che nel catalogo che il marketplace pubblica. Claude Code quindi non chiede all'utente di accettarlo a ogni installazione o aggiornamento. Invece, il file di impostazioni che lo dichiara decide quando Claude Code lo esegue:

| File di impostazioni                                                                        | Quando Claude Code esegue il comando                                                                                                                                                                                                                 |
| :------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Impostazioni utente, un file `--settings`, o un file di impostazioni gestite sulla macchina | Senza chiedere, incluso durante un aggiornamento di marketplace di background                                                                                                                                                                        |
| Il `.claude/settings.json` o `.claude/settings.local.json` di un progetto                   | Solo dopo che l'utente accetta il [workspace trust dialog](/docs/it/permissions#what-runs-before-you-trust-a-folder) per quella cartella stessa. Una sessione `-p` o SDK non conta come accettarla, e nemmeno la fiducia concessa a una cartella genitore |
| Impostazioni gestite dal server                                                             | In una sessione interattiva, solo dopo che l'utente approva le impostazioni consegnate nel [security approval dialog](/docs/it/server-managed-settings#security-approval-dialogs)                                                                         |

Per una [voce di plugin inline](/docs/it/settings-reference#extraknownmarketplaces) in uno di questi file, Claude Code richiede la stessa fiducia di cartella o approvazione di impostazioni come per un comando a livello di marketplace in quel file, e l'utente accetta anche il comando della voce a ogni installazione o aggiornamento.

<h2 id="depend-on-and-recommend-other-plugins">
  Depend on and recommend other plugins
</h2>

Una voce può dichiarare dipendenze da altri plugin.

* **Intervalli di versione**: una dipendenza può portare un intervallo semver.
* **Dipendenze cross-marketplace**: una dipendenza da un altro marketplace installa solo quando il tuo marketplace elenca quel marketplace in `allowCrossMarketplaceDependenciesOn`.

Per gli intervalli di versione, la convenzione di tag git `<plugin>--v<version>` su cui si risolvono, e la fiducia cross-marketplace, vedi [Plugin dependencies](/docs/it/plugins/dependencies).

Per fare in modo che Claude Code suggerisca un plugin quando un progetto lo corrisponde, aggiungi un blocco `relevance` alla voce con i segnali che identificano il progetto. Gli utenti vedono suggerimenti dal tuo marketplace solo quando un amministratore lo elenca in `pluginSuggestionMarketplaces`. Per i segnali e il passaggio di abilitazione, vedi [Plugin relevance](/docs/it/plugins/relevance).

<h2 id="work-around-what-a-marketplace-can’t-do">
  Work around what a marketplace can't do
</h2>

Alcune cose che i proprietari chiedono non hanno un campo in `marketplace.json`. Ecco l'opzione più vicina per ciascuna:

* **Limitare cosa altro gli utenti installano**: l'allowlist del marketplace è un'impostazione gestita, `strictKnownMarketplaces`. Vedi [Restrict what users can install](/docs/it/plugins/org#restrict-what-users-can-install).
* **Installare o abilitare un plugin senza che l'utente chieda**: nessun campo di voce installa un plugin. `enabledPlugins` gestito lo fa per una flotta; vedi [Pre-install and require plugins](/docs/it/plugins/org#pre-install-and-require-plugins).
* **Mostrare voci diverse a utenti diversi**: le voci non portano un campo di audience, e ogni utente che aggiunge il marketplace vede l'intero catalogo. Ospita marketplace separati per audience separate.
* **Contrassegnare un plugin come deprecato**: non c'è uno stato di deprecazione. L'opzione è rimuovere la voce, mappare il suo nome a `null` in `renames`, e opzionalmente impostare `forceRemoveDeletedPlugins`.
* **Attivare auto-update per i tuoi utenti**: ogni utente lo attiva in **Marketplaces** in `/plugin`, o un amministratore imposta `autoUpdate` nelle impostazioni gestite. Vedi [Turn on auto-update](#turn-on-auto-update).
* **Portare credenziali git**: nessun campo di marketplace contiene un token git. L'accesso a un marketplace o plugin ospitato su git segue la configurazione git dell'utente, per [Grant access to a private marketplace](#grant-access-to-a-private-marketplace). Per le source `archive`, una voce può impostare [`headers` o `headersHelper`](#authenticate-archive-downloads) invece.

<h2 id="next-steps">
  Next steps
</h2>

* [Marketplace reference](/docs/it/plugins/marketplace-reference): campi `marketplace.json`, tipi di source, e messaggi di convalida
* [Manage plugins for your organization](/docs/it/plugins/org): richiedi, limita, o semina il tuo marketplace su tutte le macchine della tua organizzazione
* [Plugin dependencies](/docs/it/plugins/dependencies): etichetta i rilasci in modo che i plugin che dipendono dal tuo possano risolvere le versioni
* [Troubleshoot plugins](/docs/it/plugins/troubleshooting): gli errori che i tuoi utenti vedono quando aggiungono o aggiornano dal tuo marketplace
