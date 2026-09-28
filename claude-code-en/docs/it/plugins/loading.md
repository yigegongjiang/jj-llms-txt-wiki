> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento al caricamento dei plugin

> Traccia da dove Claude Code carica ogni plugin, quale file di impostazioni decide se caricarlo e perché un aggiornamento non ha cambiato nulla.

Usa questa pagina quando un plugin non si è caricato, ha caricato una copia diversa da quella prevista, o non ha raccolto un aggiornamento, e vuoi vedere quale fonte, ambito di impostazioni o file su disco ha deciso questo. Fornisce le regole che Claude Code applica quando una sessione inizia e ogni volta che esegui `/reload-plugins`. Puoi anche chiedere a Claude di leggere questa pagina e diagnosticare la tua configurazione.

<Note>
  Questi casi sono coperti su altre pagine:

  * **Passaggi di installazione, abilitazione, disabilitazione e aggiornamento**: vedi [Installa e gestisci i plugin](/docs/it/plugins/install)
  * **Hai un messaggio di errore specifico**: vedi [Risolvi i problemi dei plugin](/docs/it/plugins/troubleshooting)
</Note>

Inizia con [Controlla quale fase ha raggiunto un plugin](#check-which-stage-a-plugin-reached) per le tre fasi che un plugin installato attraversa, oppure vai alla sezione che corrisponde a quello che stai vedendo:

* Un plugin che hai disattivato continua a caricarsi: [Trova dove un plugin è abilitato](#find-where-a-plugin-is-enabled)
* Un aggiornamento non ha cambiato nulla: [Versioni e aggiornamenti](#versions-and-updates)
* Stai guardando i file sotto `~/.claude/plugins/`: [Trova i plugin su disco](#find-plugins-on-disk)
* Un plugin `--plugin-dir` non si è caricato, o un plugin con lo stesso nome si è caricato invece: [Conflitti di nomi](#name-conflicts)

<h2 id="check-which-stage-a-plugin-reached">
  Controlla quale fase ha raggiunto un plugin
</h2>

Una voce `enabledPlugins` diventa un plugin che puoi usare in fasi: le tue impostazioni lo dichiarano, Claude Code lo recupera su disco, e la sessione in esecuzione lo carica. Quando un plugin non si comporta come un file di impostazioni suggerisce, controlla quale fase ha raggiunto:

* **Dichiarato, nelle impostazioni**: `enabledPlugins` dice quali plugin dovrebbero essere attivi, e `extraKnownMarketplaces` dice quali marketplace dovrebbero esistere. Quando esegui `claude plugin marketplace add`, Claude Code scrive il marketplace in `extraKnownMarketplaces` nelle tue impostazioni utente così come su disco
* **Recuperato, su disco sotto `~/.claude/plugins/`**: i record di quello che Claude Code ha recuperato, e i file recuperati stessi:
  * `known_marketplaces.json` registra ogni marketplace che Claude Code ha recuperato, con la sua `source`, `installLocation`, `lastUpdated` e `autoUpdate`. C'è un `known_marketplaces.json` per utente, quindi un marketplace che aggiungi in un progetto è disponibile in ogni progetto
  * `installed_plugins.json` registra ogni installazione con il suo `scope`, `installPath` e `version`
  * `cache/` contiene i file del plugin
* **Caricato, nella sessione in esecuzione**: l'insieme di plugin che Claude Code ha caricato all'avvio o all'ultimo `/reload-plugins`. Le modifiche alle impostazioni o al disco non raggiungono questo livello fino a quando non esegui `/reload-plugins` o non avvii una nuova sessione. Questo è il motivo per cui `claude plugin update` termina con `Restart to apply changes.` e gli aggiornamenti in background ti richiedono con `Run /reload-plugins to apply`

<h3 id="plugins-and-marketplaces-that-aren’t-on-disk-at-session-start">
  Plugin e marketplace che non sono su disco all'avvio della sessione
</h3>

I plugin si caricano all'avvio della sessione da `installed_plugins.json` e dalla cache senza usare la rete. Dopo l'avvio della sessione, Claude Code controlla i marketplace dichiarati in background:

* **Un marketplace che le impostazioni dichiarano ma che `known_marketplaces.json` non ha**: Claude Code lo clona, quindi ricarica i plugin e scarica i plugin abilitati che non sono ancora in cache
* **Un marketplace dichiarato la cui fonte è cambiata nelle impostazioni**: Claude Code lo recupera di nuovo dalla nuova fonte e mostra `Plugins changed. Run /reload-plugins to activate.`

Un plugin abilitato che nessuno dei due percorsi ha recuperato e che non ha una directory cache utilizzabile mostra `Plugin "<name>" not cached at <path>` nella scheda **Errors** di `/plugin`, e `claude plugin list` aggiunge `— run /plugin to refresh` alla stessa riga. Per la correzione, vedi [`Plugin "<name>" not cached at <path>`](/docs/it/plugins/troubleshooting#plugin-not-cached-at).

<h2 id="find-where-a-plugin-came-from">
  Trova da dove è venuto un plugin
</h2>

Ogni plugin ha un id della forma `<name>@<origin>`, che è quello che vedi nei file di impostazioni e in `claude plugin list --json`. La parte dopo `@` ti dice dove Claude Code ha trovato il plugin:

| L'ID termina con | Come il plugin è arrivato lì                                                                                                                                                                                      | Come lo attivi o disattivi                                                                                                                                                                                           |
| :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@<marketplace>` | L'hai installato da un marketplace che hai aggiunto                                                                                                                                                               | `"<name>@<marketplace>": true` o `false` sotto `enabledPlugins` in un file di impostazioni                                                                                                                           |
| `@inline`        | Hai avviato Claude Code con `--plugin-dir` o `--plugin-url`, impostato [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/it/env-vars#variables), o un'app Agent SDK ha passato l'opzione `plugins`. Si carica solo per quella sessione | Attivo per la sessione a meno che il manifest non imposti `defaultEnabled: false` o un file di impostazioni non imposti `"<name>@inline": false`                                                                     |
| `@skills-dir`    | Hai salvato una directory di plugin che ha un `.claude-plugin/plugin.json` sotto `~/.claude/skills/` o il `.claude/skills/` del progetto                                                                          | Il `defaultEnabled` del manifest, a meno che un file di impostazioni non imposti `"<name>@skills-dir"` su `true` o `false`                                                                                           |
| `@synced`        | Tu o la tua organizzazione l'hai attivato per il tuo account claude.ai, e Claude Code l'ha [scaricato](#synced-plugins)                                                                                           | Attivo a meno che il manifest non imposti `defaultEnabled: false` o un file di impostazioni non imposti `"<name>@synced": false`. Un plugin che la tua organizzazione contrassegna come richiesto si carica comunque |

Per un plugin marketplace, `<name>` è il nome della voce in `marketplace.json`; per `@inline` e `@skills-dir` è il `name` nel manifest del plugin.

I nomi di origine in questa tabella sono riservati, quindi nessun marketplace può essere denominato `inline`, `skills-dir` o `synced`.

<h3 id="entry-name-and-manifest-name">
  Nome della voce e nome del manifest
</h3>

Un plugin marketplace ha due nomi, e possono differire:

* **Il nome della voce in `marketplace.json`**: la chiave di installazione e abilitazione. È quello che scrivi in `enabledPlugins`, quello da cui è denominata la directory della cache, e quello che `claude plugin list` mostra
* **Il `name` nel manifest**: quello sotto cui i componenti del plugin sono namespacati, e quello che i [conflitti di nomi](#name-conflicts) confrontano

<h3 id="plugins-shared-through-a-repository">
  Plugin condivisi attraverso un repository
</h3>

Per condividere un plugin attraverso un repository, elencalo sotto `enabledPlugins` in `.claude/settings.json` o posizionalo sotto `.claude/skills/`. Claude Code non scansiona la directory `.claude/plugins/` di un progetto.

Una sessione cloud non aggiunge i marketplace che un repository elenca sotto [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces), perché ciò richiede la finestra di dialogo di fiducia dell'area di lavoro, che una sessione cloud non mostra mai.

Un plugin della directory di competenze con ambito di progetto si carica solo da `.claude/skills/` della [directory di lavoro primaria](/docs/it/permissions#working-directories) della sessione, e solo dopo che accetti la [finestra di dialogo di fiducia dell'area di lavoro](/docs/it/permissions#what-runs-before-you-trust-a-folder) per quella cartella. Non [cerca le directory padre fino alla radice del repository](/docs/it/skills#discovery-from-parent-and-nested-directories) come fanno le competenze e i comandi semplici. Se avvii da una sottodirectory, un plugin alla radice del repository non si carica. Avvia dalla radice del repository invece, o [sposta la sessione lì con `/cd`](/docs/it/permissions#move-the-session-to-another-directory) su v2.1.246 o successivo.

Un plugin con ambito di progetto è archiviato nel repository e raggiunge ogni collaboratore che lo clona. Poiché quel contenuto proviene dal repository piuttosto che da te, si carica solo dopo lo stesso controllo di fiducia che si applica alle regole di autorizzazione del progetto in `.claude/settings.json`. Fidarsi di una cartella padre o eseguire con `-p` non è sufficiente. I componenti che eseguono codice sono ulteriormente limitati:

* I server MCP che dichiara passano attraverso la [stessa approvazione per server](/docs/it/mcp) di un `.mcp.json` del progetto
* I server MCP che dichiara come un [bundle MCP](/docs/it/plugins/manifest-reference#mcpservers), un file `.mcpb` o `.dxt`, o da un file al di fuori della directory del plugin vengono saltati. Dichiarali inline o in un `.mcp.json` dentro la directory del plugin
* I [monitor in background](/docs/it/plugins/components#monitors) non si caricano

I plugin con ambito personale non hanno nessuna di queste restrizioni.

Per come scrivere plugin `--plugin-dir` e della directory di competenze, vedi [Crea plugin](/docs/it/plugins/create).

<h3 id="synced-plugins">
  Plugin sincronizzati da claude.ai
</h3>

Un plugin che attivi per il tuo account claude.ai si carica anche in Claude Code, insieme ai plugin che installi dai marketplace. Questo include i plugin che la tua organizzazione attiva per i suoi membri. Ognuno di questi plugin si carica come `<name>@synced`, senza marketplace e senza [record di installazione](#check-which-stage-a-plugin-reached).

Nelle sessioni di terminale, le competenze, gli agenti, gli hook, i server MCP e i server LSP di un plugin sincronizzato si caricano tutti, con la stessa fiducia di un plugin marketplace che hai installato.

Per i componenti che Cowork carica, vedi [Plugin su claude.ai e in Cowork](https://claude.com/docs/plugins/overview) su claude.com.

I plugin sincronizzati si caricano nelle sessioni Cowork e nelle sessioni di terminale dove accedi con il tuo account claude.ai:

* **[Cowork](https://claude.com/product/cowork)**: Claude Code li scarica nell'ambiente della sessione quando la sessione inizia
* **Sessioni di terminale**: ogni volta che avvii Claude Code, si sincronizza una volta in background, scaricando i plugin nuovi e aggiornati e rimuovendo quelli che tu o la tua organizzazione avete disattivato. La sincronizzazione nelle sessioni di terminale richiede Claude Code v2.1.273 o successivo

<h4 id="sync-timing-in-terminal-sessions">
  Tempistica di sincronizzazione nelle sessioni di terminale
</h4>

Poiché la sincronizzazione del terminale viene eseguita in background, può terminare dopo l'avvio della sessione. Quando aggiunge, aggiorna o rimuove un plugin sincronizzato in una sessione interattiva, vedi `Plugins changed. Run /reload-plugins to activate.` Esegui `/reload-plugins` per caricare la modifica in quella sessione, o lasciala per la prossima volta che avvii Claude Code.

Se abiliti un plugin su claude.ai mentre una sessione è in esecuzione, il plugin scarica la prossima volta che avvii Claude Code.

<h4 id="sign-in-requirements-for-terminal-sync">
  Requisiti di accesso per la sincronizzazione del terminale
</h4>

Nel tuo terminale, i plugin si sincronizzano solo nelle sessioni dove accedi con il tuo account claude.ai.

Se hai effettuato l'accesso su una versione precedente di Claude Code, quell'accesso non copre i plugin fino a quando Claude Code non lo rinnova in background. Per ottenere l'accesso prima, esegui di nuovo `/login`. La sincronizzazione dei plugin inizia quindi la prossima volta che avvii Claude Code.

<h4 id="control-which-synced-plugins-load">
  Controlla quali plugin sincronizzati si caricano
</h4>

Puoi disattivare i plugin sincronizzati uno alla volta, tranne un plugin che la tua organizzazione richiede, o disattivare ogni plugin sincronizzato sulla macchina:

* **Un plugin**: `claude plugin disable <name>@synced` nella tua shell e la scheda **Installed** di `/plugin` in una sessione salvano entrambi `"<name>@synced": false` nel tuo [`enabledPlugins`](/docs/it/settings-reference#enabledplugins) a livello di utente. Per mantenere il plugin fuori da un progetto in ogni ambiente, imposta la stessa chiave nel `.claude/settings.json` del progetto impegnato
* **Ogni plugin sincronizzato su una macchina**: imposta [`syncClaudeAiPlugins`](/docs/it/settings-reference#syncclaudeaiplugins) su `false` nelle tue impostazioni utente, o la tua organizzazione lo imposta nelle [impostazioni gestite](/docs/it/managed-settings). Claude Code smette di scaricare, e la prossima volta che lo avvii, sposta i plugin che ha già sincronizzato in `~/.claude/plugins/.trash/` e non li carica più. Se la tua organizzazione disattiva le competenze su claude.ai, i plugin smettono di sincronizzarsi anche
* **Un plugin che la tua organizzazione richiede**: un plugin che la tua organizzazione contrassegna come richiesto su claude.ai si carica anche se l'hai disabilitato in precedenza. `claude plugin disable` lo rifiuta con `Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.`, e `claude plugin list` lo contrassegna `required by your org`

Per rimuovere un plugin su claude.ai, vedi [Gestisci i plugin installati](/docs/it/plugins/install#manage-installed-plugins).

<h2 id="find-where-a-plugin-is-enabled">
  Trova dove un plugin è abilitato
</h2>

Puoi impostare una voce `enabledPlugins` in una qualsiasi di sei fonti. La tabella le elenca dalla precedenza più bassa a quella più alta, e chi ognuna si applica. Per i file di impostazioni stessi, vedi [File di impostazioni e chi influenzano](/docs/it/settings#where-settings-live).

| Fonte       | Dove lo imposti                                                                                    | Raggiunge                                                                                                            |
| :---------- | :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- |
| `--add-dir` | `.claude/settings.json` o `.claude/settings.local.json` in una directory che passi con `--add-dir` | Solo questa sessione. Solo un valore `true` ha effetto, e ogni altra fonte lo sostituisce                            |
| `user`      | `~/.claude/settings.json`                                                                          | Te, in ogni progetto                                                                                                 |
| `project`   | `.claude/settings.json`                                                                            | Chiunque cloni il repository                                                                                         |
| `local`     | `.claude/settings.local.json`                                                                      | Te, solo in questo repository                                                                                        |
| `flag`      | Il valore `--settings` che passi all'avvio                                                         | Solo questa sessione                                                                                                 |
| `managed`   | [Impostazioni gestite](/docs/it/managed-settings)                                                       | Ogni utente che la politica copre. `true` forza l'abilitazione e `false` blocca, e nessun'altra fonte le sostituisce |

Queste fonti si uniscono chiave per chiave. Per ogni id di plugin, il valore che si applica è quello dalla fonte con precedenza più alta che menziona l'id. Una fonte che non menziona l'id lascia il valore dalla fonte con precedenza più bassa in vigore.

<h3 id="disabled-in-user-settings-but-still-loads">
  Disabilitato nelle impostazioni utente ma continua a caricarsi
</h3>

Se imposti un plugin su `false` in `~/.claude/settings.json` e continua a caricarsi, un `true` in una fonte con precedenza più alta lo sta sostituendo. La riga del plugin in `claude plugin list` e in `/plugin` mostra `Disabled in ~/.claude/settings.json but still loads — project settings enable it, which overrides your user setting`. Il messaggio nomina la fonte che ti ha sostituito: `project`, `project, gitignored` per `.claude/settings.local.json`, `cli flag` o `managed`.

Per rinunciare a un plugin abilitato dal progetto sulla tua macchina, imposta l'id su `false` in `.claude/settings.local.json`, che ha precedenza più alta del file del progetto.

<h3 id="enabled-in-project-settings-but-not-installed">
  Abilitato nelle impostazioni del progetto ma non installato
</h3>

Quando l'unico `true` di un plugin è nel `.claude/settings.json` del progetto, Claude Code non lo recupera su una macchina dove non è installato, a meno che la sua voce marketplace non abbia una [fonte con percorso relativo](/docs/it/plugins/marketplace-reference#plugin-sources) o una [directory seed](/docs/it/plugins/org#seed-containers-and-ci) non lo contenga già. Invece, la scheda **Errors** di `/plugin` mostra `Plugin "<name>" is enabled in project settings but isn't installed here`.

Un plugin con percorso relativo non ha bisogno di un record di installazione perché si carica dal marketplace stesso.

Claude Code recupera un plugin con una fonte esterna solo quando una di queste fonti lo imposta su `true`:

* Le tue impostazioni utente
* Un `.claude/settings.local.json` che git non traccia
* Il flag `--settings`
* Impostazioni gestite

<h2 id="find-plugins-on-disk">
  Trova i plugin su disco
</h2>

Claude Code mantiene i file del plugin e i record di stato sotto una radice di plugin, che è `~/.claude/plugins` a meno che tu non imposti [`CLAUDE_CODE_PLUGIN_CACHE_DIR`](/docs/it/env-vars). Ogni percorso nella tabella è relativo a quella radice.

| Percorso                                             | Cosa contiene                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :--------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `cache/<marketplace>/<plugin>/<version>/`            | Una directory per ogni versione installata di un plugin marketplace. `<plugin>` è il nome della voce marketplace e `<version>` è la [versione risolta](#versions-and-updates). `${CLAUDE_PLUGIN_ROOT}` punta a questa directory                                                                                                                                                                                                                                     |
| `data/<plugin-id>/`                                  | La directory persistente del plugin, esposta come `${CLAUDE_PLUGIN_DATA}`. Per come `<plugin-id>` è formato, vedi [Variabili di percorso e dati persistenti](/docs/it/plugins/components#path-variables-and-persistent-data). Claude Code la crea quando un componente del plugin la usa per la prima volta e la mantiene attraverso gli aggiornamenti. Claude Code la elimina quando disinstalli il plugin dal suo ultimo ambito, a meno che tu non passi `--keep-data` |
| `marketplaces/<name>/`                               | Il clone o il download di un marketplace aggiunto da GitHub, un altro host Git, o un URL. Un marketplace aggiunto da una fonte `file` o `directory` locale non ha una copia qui, e il suo `installLocation` in `known_marketplaces.json` è il percorso che hai fornito                                                                                                                                                                                              |
| `synced/`                                            | I plugin che Claude Code ha [sincronizzato dal tuo account claude.ai](#synced-plugins)                                                                                                                                                                                                                                                                                                                                                                              |
| `.trash/`                                            | I plugin che la sincronizzazione di claude.ai ha rimosso, come dopo che ne hai disattivato uno su claude.ai o hai smesso di sincronizzare                                                                                                                                                                                                                                                                                                                           |
| `installed_plugins.json` e `known_marketplaces.json` | I record di quello che Claude Code ha installato e quali marketplace ha recuperato, descritti sotto [Controlla quale fase ha raggiunto un plugin](#check-which-stage-a-plugin-reached). Un [marketplace ospitato su claude.ai](/docs/it/plugins/install#add-from-claude-ai) è registrato in `known_marketplaces_claudeai.json` invece                                                                                                                                    |
| `flagged-plugins.json`                               | I plugin che Claude Code ha disinstallato perché il loro marketplace li ha rimossi dall'elenco. Appaiono nella sezione **Flagged** di `/plugin`; vedi [Ospita un marketplace](/docs/it/plugins/host-marketplace)                                                                                                                                                                                                                                                         |

Poiché `${CLAUDE_PLUGIN_ROOT}` punta a una directory di versione, il percorso radice di un plugin cambia con ogni versione. Mantieni i file durevoli di un plugin in `${CLAUDE_PLUGIN_DATA}` invece.

<h3 id="in-place-and-copied-plugins">
  Plugin in-place e copiati
</h3>

Claude Code carica alcuni plugin in-place da dove li mantieni e copia il resto nella cache, secondo la loro origine:

* **Plugin `--plugin-dir` e della directory di competenze**: la directory si carica in-place e non viene mai copiata. Un archivio `--plugin-url` o un `.zip` di `--plugin-dir` viene estratto in una directory temporanea della sessione prima
* **Plugin con percorso relativo in un marketplace che hai aggiunto da una directory locale**: il plugin si carica in-place dal suo percorso dentro la cartella del marketplace. Le tue modifiche alla directory di origine hanno effetto al prossimo avvio della sessione o `/reload-plugins`, e non hai bisogno di aumentare la versione. I processi hook del plugin e i server MCP e LSP ricevono un `CLAUDE_PLUGIN_ROOT` che punta alla directory di origine. Per le sue dipendenze del pacchetto Node.js, vedi [Quando viene eseguita l'installazione della dipendenza](#when-the-dependency-install-runs)
* **Plugin con origine `command` in [modalità link](/docs/it/plugins/marketplace-reference#command-plugin-source)**: la directory che il comando ha stampato si carica in-place, attraverso i link nella voce della cache
* **Ogni altro plugin marketplace**: Claude Code copia il plugin in `cache/<marketplace>/<plugin>/<version>/` all'installazione e carica quella copia. I file al di fuori della directory del plugin non vengono copiati, quindi quando uno script dentro un plugin copiato legge un percorso sopra la radice del plugin, come `../shared`, non li trova

<h3 id="paths-that-escape-the-plugin-directory">
  Percorsi che sfuggono alla directory del plugin
</h3>

Che un plugin si carichi in-place o da una copia in cache, Claude Code non gli permette di dichiarare componenti al di fuori della sua stessa directory. Rifiuta un percorso di componente che si risolve al di fuori della radice del plugin, che il percorso sia dichiarato in `plugin.json` o in una voce marketplace:

* **Un percorso che punta al di fuori del plugin come scritto**, come `../shared-utils`
* **Un symlink che porta al di fuori del plugin**, diverso dai [link tra plugin all'interno di un marketplace](/docs/it/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)
* **Su macOS e Linux, un percorso che contiene una barra rovesciata da qualche parte**, anche quando il percorso rimane dentro il plugin. I componenti dichiarati con percorsi con barra rovesciata si caricano quindi solo su Windows, quindi scrivi i percorsi dei componenti con barre in avanti, come `./commands/deploy.md`

Un percorso rifiutato appare come un errore [`path escapes plugin directory`](/docs/it/errors#path-escapes-plugin-directory), e il plugin si carica senza quel componente.

<h3 id="cleanup-of-previous-versions">
  Pulizia delle versioni precedenti
</h3>

Quando aggiorni o disinstalli un plugin, Claude Code scrive un marcatore `.orphaned_at` nella directory della versione precedente. La rimuove in una pulizia in background 14 giorni dopo, quindi una sessione che ha già caricato la vecchia versione continua a funzionare.

La scansione viene eseguita solo mentre `installed_plugins.json` registra almeno un'installazione. Dopo aver disinstallato l'ultimo plugin, le directory orfane rimangono fino a quando non installi un altro.

<h3 id="node-js-package-dependencies">
  Dipendenze del pacchetto Node.js
</h3>

Quando Claude Code copia un plugin nella cache, installa anche le dipendenze del pacchetto Node.js del plugin lì, quindi gli hook e i server MCP del plugin possono caricarli.

Questa sezione copre i pacchetti npm e Bun che un plugin dichiara nel suo `package.json`. Per i plugin che dipendono da altri plugin, vedi [versioni delle dipendenze del plugin](/docs/it/plugins/dependencies).

<h4 id="when-the-dependency-install-runs">
  Quando viene eseguita l'installazione della dipendenza
</h4>

Claude Code esegue l'installazione dentro la directory della versione copiata ogni volta che ne crea una:

* Quando installi un plugin
* Quando Claude Code aggiorna un plugin a una nuova versione
* All'avvio della sessione quando un plugin abilitato non è ancora in cache, come su una nuova macchina

Per un plugin con percorso relativo [caricato in-place](#in-place-and-copied-plugins) da un marketplace di directory locale, Claude Code non installa le dipendenze nella directory di origine. Installale lì tu stesso, o da un hook in [`${CLAUDE_PLUGIN_DATA}`](/docs/it/plugins/components#path-variables-and-persistent-data).

L'installazione viene eseguita solo quando la directory radice del plugin contiene sia un `package.json` che un lockfile supportato. Il lockfile decide quale comando Claude Code esegue:

| Lockfile                                    | Comando                                          |
| :------------------------------------------ | :----------------------------------------------- |
| `bun.lock` o `bun.lockb`                    | `bun install --frozen-lockfile --ignore-scripts` |
| `npm-shrinkwrap.json` o `package-lock.json` | `npm ci --ignore-scripts`                        |

Se un plugin contiene più di uno di questi lockfile, Claude Code usa la prima corrispondenza, controllando in ordine: `bun.lock`, `bun.lockb`, `npm-shrinkwrap.json`, `package-lock.json`.

Claude Code salta l'installazione per i lockfile Yarn e pnpm e per un `bunfig.toml` accanto al lockfile Bun:

* Se il tuo plugin ha solo un `yarn.lock` o `pnpm-lock.yaml`, sostituiscilo con un lockfile npm
* Se un `bunfig.toml` è nella stessa directory del lockfile Bun, rimuovi il `bunfig.toml`, o sostituisci il lockfile Bun con un lockfile npm

Includi un lockfile npm per raggiungere la maggior parte degli utenti. Claude Code esegue il gestore di pacchetti del lockfile corrispondente dal PATH dell'utente e non prova l'altro lockfile se quel gestore di pacchetti manca.

Per un plugin distribuito attraverso una fonte npm, usa `npm-shrinkwrap.json`, perché npm esclude `package-lock.json` dai pacchetti pubblicati.

<h4 id="limits-on-the-dependency-install">
  Limiti all'installazione della dipendenza
</h4>

Claude Code vincola questa installazione della dipendenza in modo che nessun codice dal plugin o dai suoi pacchetti venga eseguito durante essa, e limita quanto tempo può durare:

* **Risoluzione congelata**: Bun e npm installano esattamente quello che il lockfile fissa, e falliscono piuttosto che ri-risolvere le versioni quando `package.json` e il lockfile non concordano
* **Nessuno script del ciclo di vita**: `--ignore-scripts` impedisce ai script `preinstall`, `install` e `postinstall` di funzionare, quindi le dipendenze che costruiscono moduli nativi in quegli script scaricano ma non compilano durante questa installazione
* **Timeout di 60 secondi**: Claude Code interrompe un'installazione che dura più a lungo e la tratta come fallita

Claude Code recupera un plugin con origine npm prima di questa installazione della dipendenza, e nessuno degli script di installazione del pacchetto viene eseguito durante il recupero. Vedi [origine del plugin npm](/docs/it/plugins/marketplace-reference#npm-plugin-source).

Non puoi disattivare l'installazione automatica. Nessuna impostazione o variabile di ambiente la disabilita.

Nelle reti ristrette, vedi i [requisiti di accesso alla rete](/docs/it/network-config#network-access-requirements) per gli host da consentire.

<h4 id="when-the-dependency-install-fails-or-is-skipped">
  Quando l'installazione della dipendenza fallisce o viene saltata
</h4>

Un'installazione fallita o saltata non blocca mai il plugin, e ogni caso lascia un segno diverso:

* Un'installazione fallita, o una saltata a causa di un lockfile Yarn o pnpm o un `bunfig.toml`, appare come un avviso nell'output di `claude --debug`
* Un plugin con un `package.json` e nessun lockfile viene saltato senza una voce di log
* Un'installazione scaduta può lasciare un albero `node_modules` parziale nella copia in cache

Quando l'installazione automatica non può fornire una dipendenza, installala da un hook nella [directory dei dati persistenti](/docs/it/plugins/components#path-variables-and-persistent-data). Questo include i pacchetti che hanno bisogno dei loro script del ciclo di vita per costruire, le dipendenze Python, e i plugin bloccati con Yarn o pnpm.

<h2 id="versions-and-updates">
  Versioni e aggiornamenti
</h2>

Se l'autore di un plugin ha spinto nuovi commit e `claude plugin update` stampa `<name> is already at the latest version (<version>).`, la versione che Claude Code calcola per il plugin è invariata, quindi nulla cambia su disco.

Claude Code calcola una versione per ogni plugin che installa, e quella versione è come rileva un aggiornamento. `claude plugin update` e l'auto-aggiornamento in background calcolano di nuovo la versione e saltano il plugin quando corrisponde a quello che `installed_plugins.json` registra.

La versione nomina anche la directory della cache del plugin.

Un manifest che fissa `"version"` è un modo in cui la versione calcolata rimane la stessa attraverso i commit. Vedi [Come Claude Code calcola la versione](#how-claude-code-computes-the-version) per l'ordine di risoluzione.

Un plugin [caricato in-place](#in-place-and-copied-plugins) da un marketplace di directory locale carica i suoi file di origine attuali ad ogni avvio della sessione, qualunque cosa dica la sua stringa di versione. Per un plugin da un [marketplace ospitato su claude.ai](/docs/it/plugins/install#add-from-claude-ai), la versione che claude.ai registra per il plugin è la sua versione, e il `version` del manifest non viene letto.

<h3 id="how-claude-code-computes-the-version">
  Come Claude Code calcola la versione
</h3>

Per un marketplace che hai aggiunto per fonte, Claude Code sceglie la regola dal tipo `source` della voce marketplace del plugin. Il [riferimento marketplace](/docs/it/plugins/marketplace-reference#plugin-sources) elenca i tipi di fonte. Per ogni tipo di fonte in quell'elenco tranne `command`:

1. Il campo `version` nel manifest del plugin viene per primo
2. Poi il campo `version` nella voce marketplace del plugin
3. Quando nessuno è impostato, la versione viene dal tipo di fonte:

| Tipo di fonte                                                                                 | Versione quando nessun campo `version` è impostato                                                                                         |
| :-------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `github`, `url` o `git-subdir`                                                                | Lo SHA del commit della fonte, abbreviato a 12 caratteri. Una versione `git-subdir` porta anche un hash del percorso della sottodirectory  |
| `archive`                                                                                     | Il digest SHA-256, abbreviato a 12 caratteri: il pin `sha256` nella voce marketplace, o il digest del file scaricato quando non c'è un pin |
| Percorso relativo dentro un marketplace ospitato su Git                                       | Lo SHA del commit della directory installata                                                                                               |
| Directory locale, quando né la directory del plugin né il suo marketplace è un repository git | `unknown`                                                                                                                                  |
| `npm`                                                                                         | `unknown`                                                                                                                                  |

Claude Code non prende la versione da un repository che racchiude il percorso di installazione, come un `~/.claude` gestito da git.

Per una fonte `command`, Claude Code deriva sempre la versione da quello che il comando ha prodotto: un hash di 12 caratteri da solo, o `<manifest version>-<hash>` quando il manifest ne imposta uno. La `version` della voce marketplace viene ignorata per le fonti command. Per cosa copre l'hash, vedi [Modalità copia e modalità link](/docs/it/plugins/marketplace-reference#copy-mode-and-link-mode).

Poiché il manifest viene per primo, un manifest che fissa `"version": "1.0.0"` mantiene ogni utente sulla copia in cache fino a quando il suo autore non cambia la stringa, per quanti commit spingano. Per lasciare che gli utenti tracciino i commit invece, lascia `version` fuori sia dal manifest che dalla voce. [Ospita un marketplace](/docs/it/plugins/host-marketplace) copre quale scelta si adatta a quale configurazione di rilascio.

<h3 id="when-claude-code-refreshes-a-marketplace-before-an-install">
  Quando Claude Code aggiorna un marketplace prima di un'installazione
</h3>

Quando installi un plugin, Claude Code lo cerca nella sua copia locale del catalogo marketplace. Puoi eseguire `/plugin install` in una sessione o `claude plugin install` nella tua shell, e nominare il plugin con o senza il suo marketplace. La tabella mostra quale di quelle combinazioni aggiorna la copia locale.

| Nome del plugin    | Comando                                     | Cosa Claude Code aggiorna                                                                     |
| :----------------- | :------------------------------------------ | :-------------------------------------------------------------------------------------------- |
| `name@marketplace` | `/plugin install` o `claude plugin install` | Il marketplace denominato, prima della ricerca                                                |
| `name` solo        | `/plugin install`                           | Solo i marketplace che hanno l'auto-aggiornamento attivo, e solo dopo che la ricerca fallisce |
| `name` solo        | `claude plugin install`                     | Nulla. Legge i cataloghi in cache senza aggiornare                                            |

L'aggiornamento prima di un'installazione `name@marketplace` non dipende dall'impostazione di auto-aggiornamento del marketplace o da `DISABLE_AUTOUPDATER`.

Quando l'aggiornamento fallisce, l'installazione procede dal catalogo in cache e `claude plugin install` segnala `marketplace not refreshed`.

Claude Code salta l'aggiornamento prima di un'installazione `name@marketplace` quando:

* Il marketplace è stato aggiunto da una fonte `file` o `directory` locale, o è definito inline nelle impostazioni con una [fonte `settings`](/docs/it/settings-reference#extraknownmarketplaces)
* Una [directory seed](/docs/it/env-vars) fornisce il marketplace
* Claude Code ha aggiornato il marketplace negli ultimi 30 secondi
* Hai impostato `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* [Impostazioni gestite](/docs/it/plugins/org#restrict-what-users-can-install) bloccano il marketplace, nel qual caso Claude Code rifiuta anche l'installazione

<h3 id="when-auto-update-runs">
  Quando viene eseguito l'auto-aggiornamento
</h3>

In una sessione interattiva, dopo che invii il tuo primo messaggio, Claude Code attende un ritardo casuale fino a dieci minuti. Quindi aggiorna ogni marketplace con auto-aggiornamento attivo e aggiorna i plugin installati da loro su disco.

La sessione in esecuzione mantiene le versioni che ha caricato, e vedi `Plugin updated: <name> · Run /reload-plugins to apply`. Che tu ricarichi o no, le nuove versioni si caricano al tuo prossimo avvio.

<h4 id="which-marketplaces-and-plugins-auto-update">
  Quali marketplace e plugin si auto-aggiornano
</h4>

Se un marketplace si auto-aggiorna segue il primo di questi che è impostato:

1. **`autoUpdate` sulla sua voce `extraKnownMarketplaces`** in un file di impostazioni
2. **`autoUpdate` sulla sua voce `known_marketplaces.json`**, che l'interruttore **Enable auto-update** sotto `/plugin` **Marketplaces** scrive. Quando un file di impostazioni dichiara anche il marketplace sotto `extraKnownMarketplaces`, l'interruttore scrive `autoUpdate` anche a quella voce di impostazioni
3. **L'impostazione predefinita**: attivo per i marketplace ufficiali di Anthropic come `claude-plugins-official`, disattivo per `knowledge-work-plugins` e `first-party-plugins`, attivo per i [marketplace aggiunti da claude.ai](/docs/it/plugins/install#add-from-claude-ai), e disattivo per ogni altro marketplace

Se imposti `DISABLE_UPDATES=1`, `DISABLE_AUTOUPDATER=1` o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, l'intero passaggio è disattivo e l'interruttore **Enable auto-update** è nascosto, a meno che tu non imposti anche `FORCE_AUTOUPDATE_PLUGINS=1`. Il [riferimento delle variabili di ambiente](/docs/it/env-vars) copre l'effetto più ampio di ogni variabile.

L'auto-aggiornamento salta anche un plugin la cui voce marketplace dichiara un `headersHelper`. [Installazioni e aggiornamenti che rifiutano un comando invece di chiedere](/docs/it/plugins/host-marketplace#installs-and-updates-that-refuse-the-command-instead-of-asking) spiega quando tale plugin appare nella scheda **Errors** di `/plugin` e come lo aggiorni da lì.

Quando un plugin copiato si aggiorna a metà sessione, i comandi hook, i monitor, i server MCP e i server LSP continuano a usare il percorso della versione precedente. Esegui `/reload-plugins` per passare gli hook, i server MCP e i server LSP al nuovo percorso. I monitor richiedono un riavvio della sessione.

<h3 id="when-a-command-source-re-runs">
  Quando una fonte command viene rieseguita
</h3>

I plugin con una fonte `command` non aspettano il [passaggio di auto-aggiornamento](#when-auto-update-runs). La directory stampata riflette lo stato dello strumento al momento dell'esecuzione del comando, quindi Claude Code esegue di nuovo il [comando che hai accettato](/docs/it/plugins/host-marketplace#change-the-command-of-a-command-source) in questi momenti:

* Ogni volta che installi o aggiorni il plugin
* Una volta per sessione per ogni plugin con origine command abilitato, in background, poco dopo l'avvio della sessione. Questa esecuzione non dipende dall'impostazione di auto-aggiornamento del marketplace o da `DISABLE_AUTOUPDATER`
* All'avvio o su `/reload-plugins`, quando la versione installata di un plugin abilitato manca dalla cache del plugin

Claude Code salta le due esecuzioni in background quando imposti [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars). Le installazioni e gli aggiornamenti espliciti eseguono comunque il comando con quella variabile impostata.

Quando l'output con hash del comando è cambiato, Claude Code installa il risultato come una nuova versione e lo ricarica nella sessione interattiva in esecuzione, passando [gli stessi componenti che `/reload-plugins` passa](/docs/it/plugins/cli-reference#reload-plugins). Vedi una notifica che il plugin è stato ricaricato.

Se ricaricare in-place invaliderebbe la cache del prompt della sessione, Claude Code invece ti richiede di eseguire `/reload-plugins`, che [avverte del costo della cache e si applica quando rieseguito con `--force`](/docs/it/prompt-caching#enabling-or-disabling-a-plugin).

<h2 id="name-conflicts">
  Conflitti di nomi
</h2>

Quando i plugin abilitati da origini diverse condividono un nome di manifest, questo ordine decide quale si carica, dalla precedenza più alta a quella più bassa:

1. Un plugin il cui id appare nelle impostazioni gestite `enabledPlugins`, come `true` o `false`. Una copia `--plugin-dir` il cui nome di manifest corrisponde alla parte del nome dell'id non viene caricata, e vedi `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`
2. Un plugin `--plugin-dir`, `--plugin-url` o `CLAUDE_CODE_PLUGIN_DIRS` abilitato. Sostituisce un plugin marketplace installato o della directory di competenze con lo stesso nome:
   * **Un plugin marketplace installato**: sostituito silenziosamente. `claude plugin list` mostra ancora la riga del marketplace come abilitata, perché quella riga riflette le tue impostazioni. Solo il log che Claude Code scrive sotto `~/.claude/debug/` quando avvii con `--debug` registra `Plugin "<name>" from --plugin-dir overrides installed version`
   * **Un plugin della directory di competenze**: sostituito con una riga della scheda **Errors** di `/plugin` che legge `Not loaded — the name "<name>" is already taken by a session-only plugin (--plugin-dir / --plugin-url), which takes precedence`
3. Un plugin marketplace installato. Un plugin della directory di competenze con lo stesso nome ottiene la stessa riga `Not loaded`, nominando il plugin installato
4. Un plugin della directory di competenze. Tra due di questi, la copia sotto `~/.claude/skills/` si carica e la copia di `.claude/skills/` del progetto viene eliminata, con una riga che dice quale percorso l'ha oscurata
5. Un plugin [sincronizzato da claude.ai](#synced-plugins). Quando un plugin abilitato da qualsiasi altra origine corrisponde al suo nome, Claude Code carica quel plugin e segnala la copia di claude.ai come non caricata. Per usare la copia di claude.ai invece, disabilita la tua copia

Poiché l'ordine confronta i nomi di manifest, un plugin `--plugin-dir` denominato `hello-plugin` sostituisce `hello@example-marketplace` quando il manifest di quel plugin dice anche `"name": "hello-plugin"`.

<h3 id="keep-a-session-only-plugin-from-loading">
  Impedisci a un plugin solo per sessione di caricarsi
</h3>

Per impedire a un plugin `--plugin-dir` di oscurare qualcosa, o per disattivarne uno quando un processo padre passa il flag per te, imposta il suo id su `false` in un file di impostazioni. Per un plugin il cui nome di manifest è `hello-plugin`, la voce è `"enabledPlugins": {"hello-plugin@inline": false}`. Un plugin solo per sessione disabilitato non oscura, quindi la copia marketplace o della directory di competenze si carica invece.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Installa e gestisci i plugin](/docs/it/plugins/install): i passaggi di installazione, abilitazione, disabilitazione e aggiornamento stessi
* [Risolvi i problemi dei plugin](/docs/it/plugins/troubleshooting): messaggi di errore per fase che li produce
* [Riferimento ai comandi dei plugin](/docs/it/plugins/cli-reference): i flag e i comandi nominati su questa pagina
* [Gestisci i plugin per la tua organizzazione](/docs/it/plugins/org): le impostazioni gestite che forzano l'abilitazione o bloccano i plugin
