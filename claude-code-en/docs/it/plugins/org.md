> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestire i plugin Claude Code per la tua organizzazione

> Controlla quali plugin Claude Code installa e consente su ogni macchina della tua organizzazione attraverso impostazioni gestite.

Le impostazioni gestite ti permettono di decidere quali plugin Claude Code installa e consente su ogni macchina della tua organizzazione. Gli utenti non possono ignorarle. Le consegni come [impostazioni gestite dal server](/docs/it/server-managed-settings) dalla console di amministrazione di claude.ai o come impostazioni gestite dall'endpoint tramite MDM o un file `managed-settings.json`. La maggior parte dei controlli su questa pagina hanno effetto solo dalle impostazioni gestite.

Questa pagina è per gli amministratori e le impostazioni qui governano Claude Code.

<Note>
  Questi casi sono coperti su altre pagine:

  * **Installare plugin per te stesso**: inizia da [Install plugins](/docs/it/plugins/install)
  * **Controllare quali plugin i membri possono utilizzare in claude.ai e Cowork**: vedi [Manage plugins for your organization](https://support.claude.com/en/articles/13837433) nel centro assistenza
  * **La pagina dei plugin nelle impostazioni di amministrazione di claude.ai**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) attiva i plugin per gli account claude.ai dei membri, e questi raggiungono Claude Code come [synced plugins](/docs/it/plugins/loading#synced-plugins). Non imposta nessuna delle chiavi su questa pagina
</Note>

Le sezioni seguono l'ordine che la maggior parte dei rollout seguono: [richiedere plugin](#pre-install-and-require-plugins) per tutti o per repository, [seed containers e CI](#seed-containers-and-ci), [limitare](#restrict-what-users-can-install) ciò che gli utenti possono aggiungere da soli, [impostare la politica di aggiornamento](#set-update-policy), quindi [controllare](#audit-and-review) ciò che è installato. Per rivedere ogni chiave di politica in un unico posto, vedi la [matrice di controllo](#control-matrix).

<h2 id="pre-install-and-require-plugins">
  Pre-install and require plugins
</h2>

Un marketplace è un catalogo di plugin che Claude Code recupera da un repository git, un URL o un percorso locale. Una volta registrato un marketplace su una macchina, Claude Code può installare plugin da esso.

Per installare plugin per una flotta, imposta due chiavi insieme nelle [impostazioni gestite](/docs/it/managed-settings), il file di politica o la politica consegnata dal server che ogni macchina della tua organizzazione legge: `extraKnownMarketplaces` registra un marketplace su ogni macchina, e `enabledPlugins` nomina i plugin da installare e abilitare da esso. [Choose a delivery mechanism](#choose-a-delivery-mechanism) copre come le impostazioni gestite raggiungono ogni macchina.

<h3 id="choose-a-delivery-mechanism">
  Choose a delivery mechanism
</h3>

Le impostazioni gestite raggiungono una macchina attraverso uno di tre meccanismi di consegna:

* **Server-managed settings**: imposta le chiavi del plugin come JSON in [**Organization settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code). Richiede un [Owner role](/docs/it/server-managed-settings#access-control) nella tua organizzazione Claude. Una sessione cloud recupera queste impostazioni prima di installare i plugin.
* **MDM policies**: su macOS, consegna un plist le cui chiavi di livello superiore sono le chiavi delle impostazioni. Su Windows, archivia l'intero documento JSON come stringa in un valore di registro. Il dominio plist e la chiave di registro sono in [Where each mechanism stores the policy](/docs/it/managed-settings#where-each-mechanism-stores-the-policy).
* **Managed settings file**: posiziona un `managed-settings.json` nel percorso di sistema della piattaforma. Puoi anche aggiungere file alla directory drop-in `managed-settings.d/` accanto ad esso. I percorsi dei file per piattaforma sono in [Where each mechanism stores the policy](/docs/it/managed-settings#where-each-mechanism-stores-the-policy), e le regole di merge drop-in sono in [Split a file-based policy across teams](/docs/it/managed-settings#split-a-file-based-policy-across-teams).

Usa le impostazioni gestite dal server se hai un'organizzazione Claude for Teams o Enterprise su claude.ai e i tuoi dispositivi non sono tutti sotto MDM. Altrimenti usa una politica MDM o il file delle impostazioni gestite. Per il compromesso, vedi [Choose between server-managed and endpoint-managed settings](/docs/it/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings).

<h4 id="which-managed-source-applies-on-a-machine">
  Which managed source applies on a machine
</h4>

Per impostazione predefinita, solo una di queste tre fonti si applica su una macchina. Claude Code utilizza la prima che consegna una chiave di politica, controllando prima le impostazioni gestite dal server, poi le politiche MDM, quindi il file delle impostazioni gestite. Se le impostazioni gestite dal server consegnano anche una sola chiave non correlata, Claude Code ignora le chiavi del plugin in una politica MDM o nel file delle impostazioni gestite su quella macchina, a parte le [chiavi che legge da ogni fonte](/docs/it/managed-settings#keys-read-from-every-admin-source).

Per applicare ogni fonte invece, imposta [`managedSourcesBehavior`](/docs/it/managed-settings#compose-every-managed-source) a `"merge"`.

[How Claude Code combines managed sources](/docs/it/managed-settings#how-claude-code-combines-managed-sources) elenca anche le chiavi che Claude Code legge da ogni fonte in entrambi i modi.

<h3 id="require-a-marketplace-and-its-plugins">
  Require a marketplace and its plugins
</h3>

Aggiungi il marketplace sotto `extraKnownMarketplaces`, con chiave il `name` del marketplace dal suo `marketplace.json`. Quindi aggiungi ogni plugin sotto `enabledPlugins` come `plugin-name@marketplace-name`. Ogni voce di marketplace porta un oggetto `source` con un campo `source` che nomina il tipo, come `github`. Questo esempio di impostazioni gestite registra un marketplace dell'organizzazione e forza l'abilitazione di due plugin da esso:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

Dopo che le impostazioni raggiungono una macchina, Claude Code registra il marketplace e installa i due plugin all'inizio della sessione successiva dell'utente. Gli utenti li vedono in `/plugin`, e disabilitarne uno al loro ambito non lo impedisce di caricarsi, perché le impostazioni gestite hanno precedenza su ogni altro ambito.

Per bloccare un plugin a ogni ambito e nasconderlo dall'elenco del marketplace, impostalo a `false` nel `enabledPlugins` gestito invece.

Regola i campi `autoUpdate` e `source` per il tuo marketplace:

* **`autoUpdate`**: `true` mantiene il marketplace e i suoi plugin aggiornati in background, e `false` lo disattiva. Vedi [Set update policy](#set-update-policy).
* **`source`**: `github` è uno di diversi tipi di fonte. Una fonte `git` accetta un `url` per GitLab o un host interno, e una fonte `url` accetta l'indirizzo di un `marketplace.json` ospitato. Ogni forma di fonte è nel [marketplace reference](/docs/it/plugins/marketplace-reference).

Se il marketplace è un repository git privato, ogni utente ha bisogno dell'accesso in lettura ad esso. Il clone di un marketplace basato su git viene eseguito con git sulla macchina dell'utente, utilizzando credenziali archiviate e nessun prompt. Per gli utenti senza account su host git, usa un [seed](#seed-containers-and-ci) invece.

Una voce gestita sostituisce anche una voce di marketplace con lo stesso nome o una copia `--plugin-dir` da un'altra fonte:

* **Marketplaces**: una voce di marketplace gestita sostituisce una voce di precedenza inferiore con lo stesso nome, e i campi delle due voci non si uniscono.
* **Copie `--plugin-dir`**: `--plugin-dir` carica un plugin da una directory locale per una sessione. Per ciò che accade quando il nome di quella copia corrisponde a un plugin che il tuo `enabledPlugins` gestito nomina, vedi [Name conflicts](/docs/it/plugins/loading#name-conflicts).

Il marketplace ufficiale di Anthropic `claude-plugins-official` non ha bisogno di una voce `extraKnownMarketplaces` quando `enabledPlugins` imposta uno dei suoi plugin a `true`. Quella voce `name@claude-plugins-official` dichiara il marketplace da sola, ovunque si applichino queste chiavi. Se non abiliti nessuno dei suoi plugin e vuoi comunque che sia registrato su ogni macchina, dagli una voce esplicita, come [Allow the official marketplace and your own](#allow-the-official-marketplace-and-your-own) fa.

<h3 id="require-plugins-per-repository">
  Require plugins per repository
</h3>

Per coprire i contributori di un repository invece di tutta la tua flotta, imposta `extraKnownMarketplaces` e `enabledPlugins` nel `.claude/settings.json` di quel repository. Le voci `extraKnownMarketplaces` si applicano solo in una cartella che il contributore ha fiducia, e in una cartella non fidata Claude Code le ignora senza un messaggio:

* **Sessioni interattive**: Claude Code registra il marketplace solo dopo che il contributore accetta il [workspace trust dialog](/docs/it/permissions#what-runs-before-you-trust-a-folder) per quella cartella.
* **[Esecuzioni non interattive `-p`](/docs/it/headless)**: le voci si applicano solo in una cartella la cui fiducia l'utente ha già accettato interattivamente, o il cui flag `hasTrustDialogAccepted` hai impostato in `~/.claude.json`.

Un plugin che il marketplace elenca per un percorso relativo si carica dalla copia del marketplace una volta che le voci `extraKnownMarketplaces` del repository si applicano. Un plugin il cui ingresso del marketplace punta a una fonte esterna invece, come il repository GitHub del plugin stesso, non si installa dalle impostazioni del repository da solo. Ogni contributore vede `Plugin "<name>" is enabled in project settings but isn't installed` finché non esegue `claude plugin install <name>@<marketplace> --scope project`, come [Install plugins](/docs/it/plugins/install) descrive.

Se usi una fonte `directory` o `file` locale con un percorso relativo, il percorso si risolve rispetto al checkout principale del tuo repository. Quando esegui Claude Code da un git worktree, il percorso punta ancora al checkout principale, quindi tutti i worktree condividono la stessa posizione del marketplace.

Per distribuire un bundle di plugin con dipendenze, metti il plugin bundle in `enabledPlugins`, come [Plugin dependencies](/docs/it/plugins/dependencies) descrive.

<h3 id="when-each-surface-applies-the-plugin-keys">
  When each surface applies the plugin keys
</h3>

La tabella mostra quando ogni tipo di sessione Claude Code applica `extraKnownMarketplaces` e `enabledPlugins`, dalle impostazioni gestite e dal `.claude/settings.json` di un repository. Per l'app Desktop e le estensioni IDE, vedi [Install a plugin](/docs/it/plugins/install#install-a-plugin).

| Surface               | Managed `extraKnownMarketplaces` and `enabledPlugins`                                                                                                                                                                                                                                                                               | Repository `.claude/settings.json`                                                           |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
| Terminal, interactive | Applied at session start on every machine that receives the settings                                                                                                                                                                                                                                                                | `extraKnownMarketplaces` applied after trust; `enabledPlugins` applied at session start      |
| `-p` and CI           | Applied at session start, with installs running in the background                                                                                                                                                                                                                                                                   | `extraKnownMarketplaces` in trusted folders only; `enabledPlugins` applied                   |
| Cloud sessions        | In an Anthropic-hosted environment, only server-managed settings reach the session, which waits for them before it installs plugins. MDM policies and managed settings files stay on the user's machine. For a self-hosted environment, see [Where and when a policy applies](/docs/it/managed-settings#where-and-when-a-policy-applies) | See the **Cloud session** tab under [Install a plugin](/docs/it/plugins/install#install-a-plugin) |

In a `-p` or CI run, marketplaces and plugins install in the background, so a plugin can be missing from the first turn. Set `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1` to make the run wait for the install before its first query.

<h3 id="confirm-the-rollout">
  Confirm the rollout
</h3>

Check that the marketplace and plugins arrived on a machine or in a CI run:

* **On one machine**: start Claude Code and run `/plugin`. The marketplace and the plugins are listed.
* **In CI**: run `claude -p` with `--output-format stream-json --verbose`. The `init` event lists the loaded plugins under `plugins`.

<h2 id="seed-containers-and-ci">
  Contenitori seed e CI
</h2>

Per le immagini di container e i runner CI che non possono clonare in fase di esecuzione, pre-popola una directory di plugin al momento della compilazione e punta `CLAUDE_CODE_PLUGIN_SEED_DIR` ad essa. Claude Code registra i marketplace del seed all'avvio e carica le cache dei plugin dal seed in posizione, senza clonare.

Un seed serve anche gli utenti che non hanno un account su un git-host.

<Note>
  Negli ambienti CI/CD, configura un helper di credenziali git prima di installare plugin da repository privati. Su GitHub Actions, esporta un token con accesso in lettura al repository del marketplace come `GH_TOKEN`, quindi esegui `gh auth setup-git`. Il token di workflow predefinito può accedere solo al repository del workflow stesso, quindi un marketplace privato in un altro repository necessita di un token di accesso personale o un token dell'app.
</Note>

<Steps>
  <Step title="Installa nel seed al momento della compilazione">
    Imposta `CLAUDE_CODE_PLUGIN_CACHE_DIR` al percorso del seed in modo che il marketplace e i plugin si installino lì invece di `~/.claude/plugins`:

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    Il seed ha lo stesso layout di `~/.claude/plugins`: `known_marketplaces.json`, `marketplaces/<name>/`, e `cache/<marketplace>/<plugin>/<version>/`. Puoi montare il seed in un percorso diverso da quello in cui lo hai compilato.
  </Step>

  <Step title="Punta il runtime al seed">
    Imposta `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` nell'ambiente del container. Per utilizzare più seed, separa i loro percorsi con `:` su Unix o `;` su Windows. Claude Code utilizza il primo seed che contiene un determinato marketplace o cache di plugin.
  </Step>

  <Step title="Abilita i plugin">
    I plugin in un seed non sono abilitati automaticamente. Imposta `enabledPlugins` per ogni plugin del seed che desideri caricare, nelle impostazioni gestite o nel file `.claude/settings.json` del repository.
  </Step>
</Steps>

Per verificare un seed, esegui `claude -p` con `--output-format stream-json --verbose` nell'immagine. Nell'elenco `plugins` dell'evento `init`, il `path` di ogni plugin caricato si trova sotto il seed, ad esempio `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`.

I marketplace seed seguono queste regole:

* **Sola lettura**: Claude Code non scrive mai nel seed e forza `autoUpdate` disattivato per i marketplace seed.
* **Le voci seed hanno la precedenza**: ad ogni avvio, un marketplace dichiarato nel seed sovrascrive la voce dell'utente con lo stesso nome. Gli utenti escludono un plugin seed con `claude plugin disable`, non rimuovendo il marketplace.
* **L'aggiornamento e la rimozione non riescono**: `claude plugin marketplace update <name>` e `remove` senza `--scope` su un marketplace seed non riescono con un messaggio che nomina la directory del seed.
* **La policy si applica comunque**: il [allowlist e blocklist](#restrict-what-users-can-install) controllano anche la fonte registrata di un marketplace seed. Consenti la fonte da cui hai compilato il seed.

Per flotte senza accesso git in uscita, combina un seed con fonti di marketplace `directory` o `file` su un mount condiviso. Imposta anche `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, che disattiva anche l'[auto-aggiornamento dei plugin](/docs/it/plugins/loading#when-auto-update-runs). Se è disponibile un proxy, consulta [Configurazione proxy](/docs/it/network-config#proxy-configuration) per le variabili da impostare.

<h2 id="restrict-what-users-can-install">
  Limitare ciò che gli utenti possono installare
</h2>

L'elenco consentito gestito `strictKnownMarketplaces` e l'elenco bloccato `blockedMarketplaces` decidono da quali fonti di marketplace i plugin possono provenire. La fonte di un marketplace è il repository git, l'URL o il percorso locale da cui Claude Code lo recupera. Entrambi gli elenchi corrispondono alla fonte del marketplace da cui proviene un plugin, non alla voce del plugin stesso all'interno di quel marketplace.

Per il blocco comune, che consente il marketplace ufficiale e il vostro, vedere [Consentire il marketplace ufficiale e il vostro](#allow-the-official-marketplace-and-your-own). Abbinarlo a [`disableSideloadFlags`](#control-matrix) in modo che gli utenti non possano caricare plugin da una directory locale o URL.

Entrambi gli elenchi si applicano prima di qualsiasi download e di nuovo all'avvio della sessione:

* **Prima di un download**: gli elenchi si applicano quando un utente aggiunge un marketplace e ad ogni installazione, aggiornamento, refresh e auto-aggiornamento.
* **All'avvio della sessione**: gli elenchi si applicano di nuovo ai plugin già installati, quindi un plugin installato la cui fonte di marketplace non corrisponde più non si carica. `/plugin` lo elenca con `Marketplace "<name>" is not in the allowed marketplace list` o `Marketplace "<name>" is blocked by enterprise policy`.

Il punto in cui i due elenchi vengono applicati dipende da dove li impostate:

* **La console di amministrazione claude.ai**: Claude Code applica entrambi gli elenchi nelle sessioni che [leggono le impostazioni gestite dal server](/docs/it/managed-settings#where-and-when-a-policy-applies). claude.ai li controlla anche quando chiunque nella vostra organizzazione aggiunge un nuovo marketplace da un repository git su claude.ai, o da **Customize** nell'app Claude Desktop al di fuori della scheda Code. Questo copre un marketplace che un membro aggiunge per il proprio account e uno aggiunto per l'intera organizzazione in [**Organization settings > Plugins**](https://claude.ai/admin-settings/plugins). claude.ai rifiuta un repository che l'elenco consentito non ammette o che l'elenco bloccato nomina. Non ri-controlla un marketplace che è stato aggiunto in uno dei due posti prima di impostare gli elenchi, e non controlla i plugin caricati.
* **Un file di impostazioni gestite, una policy a livello di sistema operativo o un'altra fonte gestita**: Claude Code applica entrambi gli elenchi dove legge quella fonte. claude.ai non la legge.

Mentre è impostato un elenco consentito, o un elenco bloccato nomina una fonte diversa da [`skills-dir`](#blocklist-with-blockedmarketplaces), un plugin il cui marketplace Claude Code non riesce a trovare non si carica. `/plugin` mostra l'errore di policy per esso piuttosto che un errore di non trovato. Il caso comune è una voce `enabledPlugins` obsoleta per un marketplace che nessuno ha registrato.

<h3 id="control-matrix">
  Control matrix
</h3>

La tabella elenca ogni chiave di policy dei plugin, cosa applica e cosa non può fare.

| Chiave                                                                   | Cosa applica                                                                                                                                                                                                                                                                            | Cosa non può fare                                                                                                                                                                                                                                                            |
| :----------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `strictKnownMarketplaces`                                                | Elenco consentito delle fonti di marketplace. `[]` blocca ogni fonte, incluso il marketplace ufficiale. Alias: `allowedMarketplaces`                                                                                                                                                    | Non registra un marketplace, non limita le voci all'interno di un marketplace consentito, o non blocca `--plugin-dir`                                                                                                                                                        |
| `blockedMarketplaces`                                                    | Elenco bloccato delle fonti di marketplace, controllato prima dell'elenco consentito                                                                                                                                                                                                    | Non blocca un marketplace già registrato da una fonte che non corrisponde                                                                                                                                                                                                    |
| `syncClaudeAiPlugins`                                                    | Impostare `false` per interrompere il download e il caricamento da parte di Claude Code dei plugin [sincronizzati da claude.ai](/docs/it/plugins/loading#synced-plugins) per l'account di ogni utente. Richiede Claude Code v2.1.273 o successivo                                            | Non disattiva un plugin sincronizzato. Per questo, impostare `"<name>@synced": false` in [`enabledPlugins`](/docs/it/settings-reference#enabledplugins)                                                                                                                           |
| `enabledPlugins`                                                         | `true` forza l'abilitazione, `false` blocca in ogni ambito e nasconde il plugin                                                                                                                                                                                                         | Non installa un plugin il cui marketplace non è registrato o consentito                                                                                                                                                                                                      |
| `disableSideloadFlags`                                                   | Rifiuta `--plugin-dir`, `--plugin-url`, `--agents`, l'opzione `plugins` dell'Agent SDK, e non-SDK `--mcp-config` all'avvio, e rifiuta le cartelle nominate nella variabile [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/it/env-vars#variables) allo stesso modo                                         | Non limita `.mcp.json`, `claude mcp add`, o server forniti da SDK. Abbinarlo a [`allowedMcpServers`](/docs/it/managed-mcp)                                                                                                                                                        |
| `disableCommandPluginSources`                                            | Blocca i plugin con una fonte `command` dall'installazione, aggiornamento o caricamento. Una fonte `command` è quella il cui directory di plugin è prodotto eseguendo un comando sulla macchina. Quando non impostato, assume il valore di `allowManagedHooksOnly`                      | Non influisce su altri tipi di fonte                                                                                                                                                                                                                                         |
| `allowManagedHooksOnly`                                                  | Limita quali hook vengono eseguiti. Vedere [`allowManagedHooksOnly`](/docs/it/settings-reference#allowmanagedhooksonly)                                                                                                                                                                      | Non si fida degli hook dai plugin che gli utenti abilitano da soli                                                                                                                                                                                                           |
| `strictPluginOnlyCustomization`                                          | Blocca skills, agents, hooks e server MCP che non provengono da un plugin, impostazioni gestite o built-in di Claude Code. Impostare `true` per coprire tutti e quattro i tipi, o un array di valori `skills`, `agents`, `hooks` e `mcp` come `["skills", "hooks"]` per coprirne alcuni | Non limita quali plugin gli utenti installano. Abbinarlo a `strictKnownMarketplaces`                                                                                                                                                                                         |
| `pluginSuggestionMarketplaces`                                           | Marketplace i cui plugin possono apparire come suggerimenti di installazione. Vedere [Consigliare plugin](#recommend-plugins)                                                                                                                                                           | Non influisce sui suggerimenti incorporati                                                                                                                                                                                                                                   |
| `pluginTrustMessage`                                                     | Aggiunge il vostro testo all'avviso di fiducia che `/plugin` mostra prima che un plugin si installi                                                                                                                                                                                     | Non cambia il testo dell'avviso stesso                                                                                                                                                                                                                                       |
| `allowedChannelPlugins`                                                  | Sostituisce l'elenco predefinito di plugin autorizzati a inviare messaggi di canale. Richiede `channelsEnabled: true`                                                                                                                                                                   | Vedere [Limitare quali plugin di canale possono essere eseguiti](/docs/it/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                        |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/it/env-vars) | Interrompe le sessioni di terminale interattive dal registrare automaticamente il marketplace ufficiale                                                                                                                                                                                 | Non rimuove un marketplace già registrato. L'elenco consentito e l'elenco bloccato controllano la stessa registrazione automatica senza di esso. Una macchina che ha iniziato una volta con esso impostato non riprende la registrazione automatica dopo averlo disimpostato |

Ogni chiave nella tabella è un'impostazione gestita, a parte `enabledPlugins`, `syncClaudeAiPlugins` e `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`:

* **`enabledPlugins`**: potete impostarla in qualsiasi ambito, e le impostazioni gestite la bloccano.
* **`syncClaudeAiPlugins`**: ogni utente può anche impostarla nelle proprie impostazioni utente o locali. Vedere il suo [ambito nel riferimento delle impostazioni](/docs/it/settings-reference#syncclaudeaiplugins).
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**: questa è una variabile di ambiente che consegnate attraverso il blocco `env` gestito mostrato in [Disattivare gli aggiornamenti per l'intera flotta](#turn-updates-off-for-the-whole-fleet).

Ogni chiave di impostazioni qui ha una voce nel [riferimento delle impostazioni](/docs/it/settings-reference).

<h4 id="aliases-for-the-marketplace-keys">
  Alias per le chiavi di marketplace
</h4>

`strictKnownMarketplaces` può anche essere scritto `allowedMarketplaces`, e `extraKnownMarketplaces` può anche essere scritto `additionalMarketplaces`.

* **Versione**: gli alias richiedono Claude Code v2.1.232 o successivo, e i client più vecchi li ignorano. In un file che una flotta mista legge, mantenete i nomi canonici.
* **Entrambe le ortografie impostate**: quando un file imposta entrambe le ortografie, il valore della chiave canonica si applica.

<h3 id="allowlist-with-strictknownmarketplaces">
  Allowlist con `strictKnownMarketplaces`
</h3>

Impostate l'elenco consentito a un elenco di questi oggetti fonte. La maggior parte delle voci corrisponde esattamente, le voci `hostPattern` e `pathPattern` corrispondono come espressioni regolari, e i wildcard del proprietario `github` corrispondono per proprietario:

* **`github`**: `{ "source": "github", "repo": "your-org/approved-plugins" }`, con `ref` e `path` opzionali.
* **Wildcard del proprietario `github`**: `{ "source": "github", "repo": "your-org/*" }` corrisponde a ogni repository sotto quel proprietario. L'`*` deve rappresentare l'intero nome del repository. Claude Code ignora voci come `*/plugins` e `your-org/tools-*` come non valide, quindi non corrispondono a nulla. Richiede Claude Code v2.1.223 o successivo.
* **`git`**: `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`, con `ref` e `path` opzionali.
* **`url`**: `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`, con `headers` opzionali.
* **`file` e `directory`**: `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` o `{ "source": "directory", "path": "/opt/marketplace/plugins" }`, con percorsi assoluti.
* **`hostPattern`**: `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`, confrontato con l'host delle fonti `github`, `git` e `url`. Il pattern corrisponde ovunque nel nome host, quindi ancoratelo con `^` e `$` come mostrato per corrispondere all'intero host. Una fonte `github` conta sempre come `github.com`. Usate una voce `hostPattern` per un host GitHub Enterprise Server o GitLab dove gli sviluppatori creano i propri marketplace. La [pagina GHES](/docs/it/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings) ha l'esempio elaborato.
* **`pathPattern`**: `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`, confrontato con il `path` delle fonti `file` e `directory`. Il pattern corrisponde ovunque nel percorso, quindi iniziatelo con `^` per fissare un prefisso di directory. `".*"` consente ogni percorso locale.
* **`skills-dir`**: `{ "source": "skills-dir" }` mantiene il caricamento dei [plugin skills-directory](#keep-skills-directory-plugins-loading) mentre è impostato un elenco consentito, e non corrisponde a nessun marketplace.

<h4 id="how-entries-match">
  Come le voci corrispondono
</h4>

Una voce `url` corrisponde sul suo valore `url`; `headers` non vengono confrontati. Per le voci `github` e `git`, il `repo` o `url`, il `ref` e il `path` devono tutti corrispondere, o essere assenti su entrambi i lati:

* Una voce senza `ref` non copre una fonte con `ref: "main"`.
* Una voce per `your-org/your-marketplace` non copre un URL `git` che clona lo stesso repository.
* Una barra finale, un suffisso `.git` o `ssh://` al posto di `https://` è un valore diverso. Quando un marketplace può essere clonato da più di un URL, preferite una voce `hostPattern`.

Le voci con wildcard del proprietario seguono le regole esatte per `ref` e corrispondono a qualsiasi `path` all'interno del repository a meno che la voce non ne fissi uno. La corrispondenza con wildcard è sensibile alle maiuscole nell'elenco consentito.

<h4 id="keep-skills-directory-plugins-loading">
  Mantenere il caricamento dei plugin skills-directory
</h4>

I plugin skills-directory sono i plugin che gli utenti mantengono in `~/.claude/skills/` o in `.claude/skills/` di un progetto in cartelle che portano un `.claude-plugin/plugin.json`. Se impostate un elenco consentito senza una voce `{ "source": "skills-dir" }`, smettono di caricarsi. Le [skills](/docs/it/skills) semplici, cioè un `SKILL.md` senza quel manifest, continuano a caricarsi.

<h4 id="marketplaces-hosted-on-claude-ai">
  Marketplace ospitati su claude.ai
</h4>

L'elenco consentito e l'elenco bloccato corrispondono a un [marketplace ospitato su claude.ai](/docs/it/plugins/install#add-from-claude-ai) dal suo host. Per consentirne o bloccarne uno, aggiungete una voce `hostPattern` che corrisponde a `claude.ai` a `strictKnownMarketplaces` o `blockedMarketplaces`. Nell'elenco consentito, tale voce ammette i marketplace claude.ai della vostra organizzazione e i marketplace predefiniti di claude.ai, ma non un marketplace composto dai caricamenti claude.ai di un membro o uno il cui ambito claude.ai non ha dichiarato. Richiede Claude Code v2.1.273 o successivo.

<h4 id="lock-every-source-out">
  Bloccare ogni fonte
</h4>

Un elenco consentito vuoto, `[]`, blocca ogni fonte di marketplace, incluso il marketplace ufficiale.

Questo blocco non copre i plugin [sincronizzati da claude.ai](/docs/it/plugins/loading#synced-plugins), che Claude Code scarica dall'account di ogni utente piuttosto che da un marketplace. Per interromperli anche, impostate [`syncClaudeAiPlugins`](/docs/it/settings-reference#syncclaudeaiplugins) a `false` nelle impostazioni gestite, o disattivate Skills per la vostra organizzazione su claude.ai.

<h3 id="blocklist-with-blockedmarketplaces">
  Blocklist con `blockedMarketplaces`
</h3>

`blockedMarketplaces` accetta gli stessi oggetti fonte di [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces) ed è controllato per primo, quindi una fonte in entrambi gli elenchi è bloccata. La corrispondenza della blocklist è più ampia della corrispondenza dell'elenco consentito:

* Gli URL Git sono canonicizzati, quindi i moduli `git@` e `https://`, i suffissi `.git` e le barre finali di un repository `github.com` corrispondono tutti alla stessa voce.
* Una voce `github` blocca anche l'URL `git` equivalente, e viceversa.
* Per una voce `owner/*`, il confronto del proprietario è insensibile alle maiuscole.
* Una voce senza `ref` o `path` blocca ogni ref e path dei repository che corrisponde.

Questa voce blocca ogni repository sotto un proprietario GitHub:

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

Le voci `url` in `blockedMarketplaces` si applicano anche quando un utente aggiunge un URL di repository `https://` che Claude Code [clona piuttosto che recupera](/docs/it/plugins/cli-reference#plugin-marketplace-add), come un URL di repository `github.com` o `gitlab.com` semplice. L'utente non può aggiungere quell'URL se una voce lo nomina. La corrispondenza ignora il suffisso `.git` e qualsiasi ref che l'utente aggiunge dopo `#`. Richiede Claude Code v2.1.232 o successivo.

Una voce `{ "source": "skills-dir" }` qui interrompe il caricamento dei [plugin skills-directory](#keep-skills-directory-plugins-loading), da entrambi `~/.claude/skills/` e da `.claude/skills/` di un progetto.

Una blocklist che nomina solo quella voce non conta come una restrizione attiva, quindi non [interrompe i plugin il cui marketplace Claude Code non riesce a trovare](#restrict-what-users-can-install) dal caricamento.

<h3 id="allow-the-official-marketplace-and-your-own">
  Consentire il marketplace ufficiale e il vostro
</h3>

La maggior parte delle organizzazioni consente il marketplace ufficiale e il loro, e registra entrambi in modo che ogni macchina li abbia. Questa policy di impostazioni gestite consente entrambi i marketplace, registra entrambi, forza l'abilitazione di due plugin e rifiuta `--plugin-dir`:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

Su una macchina con questa policy, aggiungere qualsiasi fonte al di fuori dell'elenco, ad esempio `/plugin marketplace add https://example.com/other-marketplace.git`, fallisce con un messaggio contenente `is blocked by enterprise policy` seguito dalle fonti consentite. `claude --plugin-dir ./x` esce con un messaggio che nomina `disableSideloadFlags`.

La voce `{ "source": "skills-dir" }` mantiene il caricamento dei [plugin skills-directory](#keep-skills-directory-plugins-loading) sotto questo elenco consentito. Rimuovete quella voce e smettono di caricarsi.

Registrate entrambi i marketplace con voci esplicite `extraKnownMarketplaces`, come fa questa policy, piuttosto che affidarvi all'elenco consentito o al marketplace ufficiale che si registra da solo:

* **L'elenco consentito non registra nulla**: una voce `extraKnownMarketplaces` lo fa, e deve essa stessa passare l'elenco consentito. Claude Code rifiuta di registrare un marketplace gestito la cui fonte l'elenco consentito non corrisponde.
* **Il marketplace ufficiale si registra solo in una sessione di terminale interattiva**: anche lì, si registra solo quando l'elenco consentito lo consente. Un'esecuzione `-p` o un terminale collegato a una sessione cloud non lo registra mai.
* **Un tentativo bloccato viene ricordato**: se una macchina ha mai eseguito una policy che bloccava il marketplace ufficiale, Claude Code registra il tentativo bloccato e non ritenta dopo il cambio di policy. Un blocco `[]` è una tale policy. Quella macchina lo registra di nuovo solo attraverso una voce `extraKnownMarketplaces` come quella in questa policy, una voce `enabledPlugins` per uno dei suoi plugin, o un `/plugin marketplace add` manuale.

<h2 id="set-update-policy">
  Impostare la politica di aggiornamento
</h2>

È possibile impostare la politica di aggiornamento per ogni marketplace, per l'intera flotta o per ogni gruppo di utenti attraverso i canali di rilascio.

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  Attivare o disattivare l'aggiornamento automatico per marketplace
</h3>

L'aggiornamento automatico dei plugin viene eseguito in background dopo l'avvio per i marketplace che lo hanno attivato. Per sapere quali marketplace lo hanno attivato per impostazione predefinita, consultare [Quando viene eseguito l'aggiornamento automatico](/docs/it/plugins/loading#when-auto-update-runs). Per decidere per la flotta, impostare `"autoUpdate": true` o `false` su una voce `extraKnownMarketplaces` gestita:

* Se la voce gestita imposta il campo, Claude Code rifiuta l'interruttore `/plugin` dell'utente con un errore che inizia con `Auto-update for '<name>' is set by`.
* Se la voce gestita lascia il campo non impostato, l'interruttore dell'utente persiste.

<h3 id="turn-updates-off-for-the-whole-fleet">
  Disattivare gli aggiornamenti per l'intera flotta
</h3>

Per disattivare l'aggiornamento automatico dei plugin per ogni marketplace, impostare `DISABLE_AUTOUPDATER` nel blocco `env` gestito, come fa questo esempio. La stessa variabile interrompe anche gli aggiornamenti di Claude Code:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Per interrompere gli aggiornamenti di Claude Code ma mantenere l'aggiornamento automatico dei plugin, aggiungere `"FORCE_AUTOUPDATE_PLUGINS": "1"` allo stesso blocco. Le altre [variabili di ambiente che interrompono l'aggiornamento automatico dei plugin](/docs/it/plugins/loading#when-auto-update-runs) funzionano allo stesso modo.

`DISABLE_AUTOUPDATER` non copre i plugin con un'[origine `command`](/docs/it/plugins/marketplace-reference#command-plugin-source). Claude Code riesegue il comando di ogni plugin abilitato ad ogni sessione e installa l'output quando è cambiato. Per sapere cosa interrompe queste esecuzioni, consultare [Quando un'origine command viene rieseguita](/docs/it/plugins/loading#when-a-command-source-re-runs).

<h3 id="assign-release-channels-to-user-groups">
  Assegnare canali di rilascio ai gruppi di utenti
</h3>

Per eseguire canali stabili e ad accesso anticipato, ospitare due marketplace che puntano a ref diversi degli stessi plugin. Quindi assegnare a ogni gruppo di utenti il proprio marketplace attraverso impostazioni gestite da endpoint separate o una politica gateway. Le impostazioni gestite dal server dalla console di amministrazione [si applicano a ogni utente della tua organizzazione](/docs/it/server-managed-settings#current-limitations), quindi non possono assegnare impostazioni diverse a gruppi diversi.

* Distribuire [impostazioni gestite da endpoint](/docs/it/managed-settings#delivery-mechanisms) separate, come un file di impostazioni gestite o un profilo MDM, ai dispositivi di ogni gruppo. Per verificare se il file o il profilo per gruppo si applica su un dispositivo che ha anche un'origine a livello di organizzazione, consultare [Come Claude Code combina le origini gestite](/docs/it/managed-settings#precedence-within-the-managed-tier).
* Definire una [politica gateway delle app Claude](/docs/it/claude-apps-gateway-config#managed) per ogni gruppo. Il gateway applica la prima politica la cui regola di corrispondenza si adatta a un utente, quindi ordinare le politiche in modo che ogni utente raggiunga la politica del suo gruppo. La mappa `extraKnownMarketplaces` di quella politica non si unisce con quella di nessun'altra politica, quindi elencare ogni marketplace di cui il gruppo ha bisogno in essa, non solo il suo marketplace del canale.

Con uno dei due meccanismi, il gruppo stabile riceve questa configurazione:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

Il gruppo ad accesso anticipato riceve invece `latest-tools`. Per configurare i due marketplace, consultare [Eseguire canali di rilascio](/docs/it/plugins/host-marketplace#run-release-channels).

<h2 id="recommend-plugins">
  Consigliare plugin
</h2>

I proprietari del marketplace possono allegare segnali di `relevance` alle voci in modo che Claude Code suggerisca il plugin quando un progetto corrisponde.

I suggerimenti da un marketplace vengono visualizzati solo quando è registrato sulla macchina dell'utente, si elenca il suo nome in `pluginSuggestionMarketplaces` nelle impostazioni gestite e si dichiara la sua origine nella stessa policy. Dichiarare l'origine come voce `extraKnownMarketplaces` del marketplace o come voce dell'elenco consentiti. Il marketplace ufficiale richiede solo il nome. Vedere [Abilitare i suggerimenti nelle impostazioni gestite](/docs/it/plugins/relevance#enable-suggestions-in-managed-settings).

<h2 id="audit-and-review">
  Audit and review
</h2>

OpenTelemetry events and the Analytics API tell you what your fleet installs and runs.

For what a plugin can run on a machine and what each trust tier permits, read [Plugin security](/docs/it/plugins/security) before you approve a marketplace.

<h3 id="opentelemetry-events">
  OpenTelemetry events
</h3>

`claude_code.plugin_installed` records each install, and `claude_code.plugin_loaded` records each enabled plugin at session start. Both events redact or omit third-party plugin and marketplace names unless you set `OTEL_LOG_TOOL_DETAILS=1`, as [Redacted plugin names in your backend](/docs/it/plugins/measure#redacted-plugin-names-in-your-backend) shows. Field lists are under [Plugin installed event](/docs/it/monitoring-usage#plugin-installed-event) and [Plugin loaded event](/docs/it/monitoring-usage#plugin-loaded-event).

<h3 id="analytics-api">
  Analytics API
</h3>

On the Enterprise plan, `GET /v1/organizations/analytics/plugins` returns per-plugin, per-day install and invocation counts across Claude Code and Cowork. You can group the counts by user or RBAC group. Plugin activity that reaches Anthropic without a plugin name appears in one aggregate `third-party` row. See the [endpoint reference](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) and [Access data programmatically](/docs/it/analytics#access-data-programmatically) for the key it needs.

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  Piano per ciò che le impostazioni gestite non possono applicare
</h2>

Queste richieste dalle revisioni di sicurezza non hanno una chiave dedicata nello schema delle impostazioni corrente. I controlli esistenti più vicini sono:

* **Targeting per utente o per gruppo**: ogni chiave plugin si applica a ogni utente che riceve le impostazioni. Le impostazioni gestite dal server forniscono una configurazione per organizzazione. Per la politica per gruppo, utilizzare impostazioni gestite da endpoint separate o politiche gateway, come in [Assegnare canali di rilascio ai gruppi di utenti](#assign-release-channels-to-user-groups).
* **Limitazione delle voci all'interno di un marketplace consentito**: l'elenco consentito corrisponde alle fonti del marketplace. Per bloccare un plugin da un marketplace consentito, impostarlo su `false` in `enabledPlugins` gestito.
* **Nascondere `/plugin`**: nessuna chiave disabilita il comando. L'equivalente più vicino combina un elenco consentito che nomina solo il vostro marketplace, voci `enabledPlugins` gestite per i plugin che fornite, e `disableSideloadFlags`.
* **Controllare `--plugin-dir` attraverso l'elenco consentito**: l'elenco consentito non copre `--plugin-dir`. `disableSideloadFlags` lo fa.
* **Applicare gli interruttori plugin claude.ai attraverso queste chiavi**: [**Impostazioni organizzazione > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) non imposta le chiavi su questa pagina. Ciò che i membri e la vostra organizzazione attivano lì raggiunge la CLI come [plugin sincronizzati](/docs/it/plugins/loading#synced-plugins), che hanno i loro propri controlli.

<h2 id="troubleshoot-policy">
  Risoluzione dei problemi della policy
</h2>

Se la policy del plugin non si comporta come previsto su una macchina, controllare prima questi sintomi:

* **Il file gestito non è stato analizzato**: quando un `managed-settings.json` non è un JSON valido, Claude Code rifiuta di avviarsi e stampa [un errore che nomina il file](/docs/it/errors#managed-settings-document-could-not-be-parsed). Un file che viene analizzato ma ha una voce non valida mantiene il resto della sua policy. Vedere [Voci non valide nelle impostazioni gestite](/docs/it/managed-settings#invalid-entries-in-managed-settings).
* **La fonte gestita non è stata caricata**: eseguire `/status` e cercare `Enterprise managed settings` nella riga `Setting sources`. Se manca, la fonte non è stata caricata.
* **Un utente segnala `blocked by enterprise policy`**: il messaggio nomina il marketplace o la sua fonte. Per un allowlist, elenca anche le fonti consentite. Le voci rivolte all'utente si trovano su [Risoluzione dei problemi dei plugin](/docs/it/plugins/troubleshooting).
* **Un plugin che l'utente ha disabilitato in `~/.claude/settings.json` continua a caricarsi**: un'altra fonte di impostazioni lo ha riabilitato, ad esempio una voce `enabledPlugins` gestita che lo forza ad abilitarsi. `/plugin` e `claude plugin list` mostrano `Disabled in ~/.claude/settings.json but still loads` con quella fonte di impostazioni.

<h2 id="next-steps">
  Next steps
</h2>

* [Marketplace reference](/docs/it/plugins/marketplace-reference#marketplace-sources): the `source` values `extraKnownMarketplaces`, `strictKnownMarketplaces`, and `blockedMarketplaces` accept
* [Host and maintain a marketplace](/docs/it/plugins/host-marketplace): run the marketplace your policy points at
* [Plugin security and trust](/docs/it/plugins/security): what a plugin can do on a machine and how to review one before installing
* [Server-managed settings](/docs/it/server-managed-settings): deliver these keys from the claude.ai admin console
* [Troubleshoot plugins](/docs/it/plugins/troubleshooting#blocked-by-your-organization): the messages users see when policy blocks them
