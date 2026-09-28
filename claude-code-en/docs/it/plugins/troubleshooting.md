> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Risolvere i problemi dei plugin

> Correggi gli errori dei plugin in Claude Code. Trova il messaggio esatto che hai visto, raggruppato per fase da dove /plugin viene eseguito fino all'installazione e alla politica organizzativa.

Questa pagina elenca i messaggi di errore e i sintomi per i plugin di Claude Code e per i marketplace, i cataloghi da cui Claude Code installa i plugin. Ogni voce fornisce la causa, una soluzione e cosa vedi una volta che la soluzione funziona.

Dove un messaggio nomina un plugin o un marketplace, la voce mostra un segnaposto come `<name>` al suo posto.

Utilizza questa pagina che tu stia installando plugin, costruendoli, ospitando un marketplace o amministrando plugin per un'organizzazione.

<Note>
  Questi casi sono trattati su altre pagine:

  * **Perché gli ambiti, la cache e la precedenza si comportano nel modo in cui lo fanno**: leggi [Plugin loading reference](/docs/it/plugins/loading)
  * **Ricerca di un flag, un campo o un comando**: utilizza il [plugin commands reference](/docs/it/plugins/cli-reference), il [manifest reference](/docs/it/plugins/manifest-reference) o il [marketplace reference](/docs/it/plugins/marketplace-reference)
</Note>

Cerca il messaggio esatto che hai visto. Ogni messaggio è elencato sotto la fase che lo produce, che non è sempre il comando che hai eseguito. Ad esempio, un'installazione può fallire perché manca un marketplace, quindi quel messaggio è sotto [Add a marketplace](#add-a-marketplace).

<h2 id="find-where-/plugin-runs">
  Find where `/plugin` runs
</h2>

`/plugin` è un comando che si digita all'interno di una sessione di terminale Claude Code in esecuzione e apre un pannello interattivo. Le voci in questa sezione coprono i luoghi in cui è possibile digitarlo ma non può essere eseguito e i comandi che non esistono.

<h3 id="plugin-isnt-available-in-this-environment">
  `/plugin isn't available in this environment`
</h3>

È stato digitato `/plugin` da qualche parte diversa da una sessione di terminale Claude Code e Claude ha risposto con questa riga invece di aprire qualcosa.

Si riceve questa risposta in una sessione che non ha un terminale per disegnare il pannello `/plugin` in: [non-interactive mode](/docs/it/headless) con `claude -p`, l'Agent SDK, la scheda Code dell'app desktop Claude, il pannello dell'estensione VS Code e il browser su claude.ai/code.

Nel pannello dell'estensione VS Code, solo una riga `/plugin` con qualcosa dopo di essa, come `/plugin install <plugin>@<marketplace>`, riceve questa risposta. `/plugin` o `/plugins` digitati da soli aprono la finestra di dialogo **Manage plugins**.

Installare il plugin dalla superficie su cui ci si trova:

* **App desktop Claude, sessione locale o SSH**: fare clic sul pulsante **+** accanto al prompt, quindi **Plugins**, quindi **Add plugin** per aprire il [plugin browser](/docs/it/desktop#install-plugins)
* **Estensione VS Code**: utilizzare la scheda **VS Code** sotto [Install a plugin](/docs/it/plugins/install#install-a-plugin)
* **Claude Code sul web o una sessione cloud desktop**: una sessione cloud non ha un browser di plugin. Vedere la scheda **Cloud session** sotto [Install a plugin](/docs/it/plugins/install#install-a-plugin) per ciò che una sessione cloud carica
* **Un terminale a cui si ha accesso**: eseguire `claude` e digitare `/plugin` lì, oppure eseguire `claude plugin install <plugin>@<marketplace>` nella shell senza avviare una sessione

Quando un'installazione da terminale funziona, `/plugin` stampa un riepilogo di installazione che inizia con `✓ Installed <plugin>.` e `claude plugin install` stampa `Successfully installed plugin: <plugin>@<marketplace>`.

<h3 id="zsh-no-such-file-or-directory-plugin">
  `zsh: no such file or directory: /plugin`
</h3>

È stato digitato `/plugin ...` al prompt della shell e la shell ha segnalato che non esiste alcun file denominato `/plugin`. Bash segnala `bash: /plugin: No such file or directory`.

`/plugin` è un comando che si digita all'interno di una sessione Claude Code, non al prompt della shell. Avviare una sessione e digitare lo stesso comando lì:

```shell theme={null}
claude
```

Quindi, al prompt di Claude Code:

```text theme={null}
/plugin install <plugin>@<marketplace>
```

Un'installazione riuscita stampa un riepilogo che inizia con `✓ Installed <plugin>.` Se l'installazione stessa fallisce, il suo messaggio è sotto [Add a marketplace](#add-a-marketplace) o [Install a plugin](#install-a-plugin).

Per installare dalla shell senza avviare una sessione, eseguire `claude plugin install <plugin>@<marketplace>` al suo posto.

<h3 id="the-term-plugin-is-not-recognized-as-the-name-of-a-cmdlet">
  `The term '/plugin' is not recognized as the name of a cmdlet`
</h3>

È stato digitato `/plugin ...` al prompt di PowerShell e `/plugin` è un comando Claude Code, non un programma. Bash e Zsh segnalano [la loro forma di questo errore](#zsh-no-such-file-or-directory-plugin).

Utilizzare uno di questi al suo posto:

* Eseguire `claude`, quindi digitare `/plugin` al prompt di Claude Code
* Eseguire `claude plugin install <plugin>@<marketplace>` in PowerShell senza avviare una sessione

<h3 id="claude-command-not-found-after-claude-plugin">
  `claude: command not found` after `claude plugin ...`
</h3>

È stato eseguito `claude plugin install ...` nella shell e la shell non ha potuto trovare `claude` affatto. Su Windows il messaggio è `'claude' is not recognized as the name of a cmdlet` o `'claude' is not recognized as an internal or external command`.

La causa non è il comando del plugin. O Claude Code non è installato, oppure la sua directory di installazione non è su `PATH` in questa shell. Seguire [`command not found: claude` after installation](/docs/it/troubleshoot-install#command-not-found-claude-after-installation), quindi riprovare il comando del plugin.

<h3 id="unknown-command-and-command-spellings-that-dont-exist">
  `Unknown command` and command spellings that don't exist
</h3>

È stato digitato un comando di plugin visto da qualche parte e si è ricevuto `Unknown command: /<name>` in una sessione, o `error: unknown command '<name>'` o `error: unknown option '<flag>'` dal binario `claude` nella shell.

Diversi comandi sono in uso che Claude Code non ha. La tabella seguente mappa ciascuno al comando reale. Il [plugin commands reference](/docs/it/plugins/cli-reference) elenca ogni sottocomando e flag.

| You typed                                  | What Claude Code says                                                        | Use instead                                                                                                                                       |
| :----------------------------------------- | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `claude plugin add <source>`               | `error: unknown command 'add'`                                               | `claude plugin marketplace add <source>` per aggiungere un marketplace, o `claude plugin install <plugin>@<marketplace>` per installare un plugin |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'`                                          | `claude plugin install <plugin>@<marketplace> --scope project`                                                                                    |
| `/install <plugin>`                        | `Unknown command: /install`                                                  | `/plugin install <plugin>@<marketplace>`                                                                                                          |
| `/plugin add <source>`                     | Il pannello `/plugin` si apre sulla scheda **Discover**                      | `/plugin marketplace add <source>`                                                                                                                |
| `marketplace.anthropic.com` come fonte     | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | `anthropics/claude-plugins-official` per il marketplace ufficiale                                                                                 |

Questi comandi sembrano sbagliati ma funzionano:

* `claude plugins` è un alias di `claude plugin`
* `claude plugin remove` è un alias di `claude plugin uninstall`
* `/plugins` e `/marketplace` in una sessione aprono lo stesso pannello di `/plugin`

<h2 id="add-a-marketplace">
  Add a marketplace
</h2>

Un marketplace è un catalogo che si aggiunge a Claude Code da un repository git, un URL o un percorso locale. Queste voci coprono i messaggi che si ricevono quando l'aggiunta fallisce o un successivo aggiornamento fallisce.

<h3 id="marketplace-claude-plugins-official-not-found">
  `Marketplace "claude-plugins-official" not found`
</h3>

È stato eseguito `/plugin install <plugin>@claude-plugins-official` in una sessione e Claude Code ha segnalato che non ha alcun marketplace con quel nome.

Il marketplace ufficiale non è ancora registrato su questa macchina. Claude Code normalmente lo registra da solo la prima volta che si avvia una sessione di terminale interattiva. Non è stato eseguito se si è utilizzato Claude Code solo attraverso l'estensione VS Code e salta o rinvia quel passaggio:

* Quando una politica blocca la fonte
* Quando `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` è impostato
* Dopo un tentativo fallito che sta aspettando di riprovare

I comandi della shell `claude plugin` non lo registrano mai per voi.

Aggiungerlo, quindi riprovare l'installazione:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code stampa `Successfully added marketplace: claude-plugins-official` e `/plugin marketplace list` mostra il marketplace con la sua fonte.

Per qualsiasi altro nome di marketplace in questo messaggio, vedere [`Marketplace "<name>" not found`](#marketplace-not-found).

La stessa stringa appare anche nella scheda **Errors** di `/plugin`, l'elenco dei fallimenti di caricamento del pannello, quando un plugin elencato nelle impostazioni nomina un marketplace che non è stato aggiunto.

<h3 id="marketplace-not-found">
  `Marketplace "<name>" not found`
</h3>

È stato eseguito `/plugin install <plugin>@<name>` in una sessione, spesso da una riga di installazione che qualcuno ha inviato, e Claude Code ha segnalato che non ha alcun marketplace con quel nome.

Se il nome inizia con `claudeai-`, il marketplace è ospitato su claude.ai e lo si aggiunge per nome dalla shell con `claude plugin marketplace add --claudeai <name>`. Vedere [Add a marketplace from claude.ai](/docs/it/plugins/install#add-from-claude-ai).

Per qualsiasi altro nome, una riga di installazione nomina un marketplace ma non dice dove è ospitato il marketplace e Claude Code non ha un indice per cercare un nome di marketplace. Chiedere a chi ha inviato la riga la fonte del marketplace, che è un GitHub `owner/repo`, un URL git o un percorso. Quindi [aggiungere il marketplace](/docs/it/plugins/install#add-a-marketplace) ed eseguire di nuovo la riga di installazione.

Un marketplace che qualcuno invia è di terze parti, quindi [rivedere il plugin prima di installarlo](/docs/it/plugins/security#review-a-plugin-before-you-install).

Se il marketplace è già stato aggiunto, controllare l'ortografia rispetto a `/plugin marketplace list`.

<h3 id="invalid-marketplace-source-format">
  `Invalid marketplace source format`
</h3>

È stato eseguito `/plugin marketplace add <source>` o `claude plugin marketplace add <source>` e Claude Code ha risposto `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`.

Claude Code accetta una fonte in una di queste forme:

* Un shorthand GitHub `owner/repo`
* Un URL `https://` o `http://`
* Un URL SSH `user@host:path`
* Un percorso locale che inizia con `./`, `../`, `/` o `~`

Un nome semplice come `claude-plugins-official` non corrisponde a nessuno di essi. Nemmeno un nome host semplice come `marketplace.anthropic.com`.

Riscrivere la fonte in una delle forme accettate:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code stampa `Successfully added marketplace: <name>` quando l'aggiunta funziona.

<h3 id="is-not-a-valid-github-owner-repo-shorthand">
  `'<source>' is not a valid GitHub owner/repo shorthand`
</h3>

È stata passata una fonte con una barra che non è `owner/repo`, come `github.com/owner/repo` o un percorso `gitlab.example.com/group/project`. Claude Code l'ha rifiutata con questo messaggio e un elenco di forme accettate.

Lo shorthand `owner/repo` è solo per GitHub e deve seguire le regole di denominazione di GitHub, quindi un nome host o un segmento di percorso aggiuntivo fallisce. Passare la fonte nella forma che corrisponde a dove è ospitato il marketplace:

* **Un repository su qualsiasi host**: l'URL di clonazione completo
* **Un `marketplace.json` ospitato**: il suo URL `https://`
* **Un checkout locale**: `./path` o un percorso assoluto

Ad esempio, per aggiungere il marketplace ufficiale tramite il suo URL di clonazione, in una sessione:

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

Un'aggiunta riuscita stampa `Successfully added marketplace: <name>`.

<h3 id="path-does-not-exist">
  `Path does not exist: <path>`
</h3>

È stato passato un percorso locale a `marketplace add` e nulla esiste in quel percorso. Un percorso relativo si risolve rispetto alla directory corrente.

Controllare il percorso risolto nel messaggio. Quindi eseguire il comando dalla directory da cui inizia il percorso relativo, o passare un percorso assoluto alla directory del marketplace. Un'aggiunta riuscita stampa `Successfully added marketplace: <name>`.

Claude Code accetta una directory che contiene `.claude-plugin/marketplace.json` o un percorso a un file `.json`. Un percorso a qualsiasi altro file fallisce con `File path must point to a .json file (marketplace.json)`.

<h3 id="marketplace-file-not-found-at-claude-plugin-marketplace-json">
  `Marketplace file not found at <path>/.claude-plugin/marketplace.json`
</h3>

Claude Code ha clonato o scaricato il marketplace ma non ha trovato `marketplace.json` nel percorso previsto al suo interno. Il comando di aggiunta lo segnala come `Failed to add marketplace: Marketplace file not found at ...`.

La posizione predefinita è `.claude-plugin/marketplace.json` alla radice del repository e il [marketplace reference](/docs/it/plugins/marketplace-reference) elenca le posizioni accettate.

La soluzione differisce per il proprietario e per tutti gli altri:

* **Si possiede il marketplace**: mettere il file in quella posizione e re-aggiungere il marketplace
* **Qualcun altro lo ospita**: chiedere al proprietario la fonte esatta che pubblica

<h3 id="ssh-authentication-failed-or-https-authentication-failed">
  `SSH authentication failed` or `HTTPS authentication failed`
</h3>

È stato aggiunto o aggiornato un marketplace da un repository git e il clone è fallito con `Failed to clone marketplace repository:` seguito da una di queste righe.

Per prima cosa controllare il repository stesso: un `owner/repo` scritto male, un repository che non esiste o un repository privato che non si può vedere termina anche con questo messaggio. Aprire l'URL del repository nel browser o eseguire `git ls-remote <url>` nel terminale per confermare che esiste e che si ha accesso.

Se il repository è corretto, la causa è le credenziali. Claude Code esegue git con i prompt interattivi disabilitati, quindi non può chiedere una password, una passphrase della chiave o una credenziale come farebbe il terminale. Se git ha bisogno di un prompt, si vede `fatal: Cannot prompt because user interactivity has been disabled` o `terminal prompts disabled` nell'errore originale. Solo le credenziali che già funzionano in modo non interattivo hanno successo:

* **SSH**: `ssh -T git@<host>` deve avere successo senza chiedere una passphrase e l'host deve già essere in `known_hosts`
* **HTTPS**: l'helper delle credenziali deve contenere un token per l'host. Per GitHub, eseguire `gh auth login` e `gh auth setup-git`. Per un altro host, memorizzare un token di accesso personale nell'helper delle credenziali git. Testare con `git ls-remote <url>`

Una volta che `git ls-remote` ha successo nel terminale senza un prompt, eseguire di nuovo l'aggiunta o l'aggiornamento. Un'aggiunta riuscita stampa `Successfully added marketplace: <name>`. Un aggiornamento riuscito stampa `Successfully updated marketplace: <name>` dalla shell o `✔ Updated 1 marketplace` in una sessione.

Per fare in modo che Claude Code salti SSH per le fonti GitHub `owner/repo`, impostare `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`. Senza di esso, Claude Code clona quelle fonti su SSH quando una chiave SSH per `github.com` sembra configurata e torna a HTTPS quando il clone SSH fallisce.

Per ciò che gli aggiornamenti automatici in background possono e non possono fare con le credenziali, vedere [What background auto-update does with credentials](/docs/it/plugins/host-marketplace#what-background-auto-update-does-with-credentials).

<h3 id="ssh-host-key-is-not-in-your-known-hosts-file">
  `SSH host key is not in your known_hosts file`
</h3>

È stato aggiunto un marketplace su SSH da un host a cui non ci si è mai connessi e il clone è fallito con questa riga e un suggerimento `ssh -T git@<host>`. Per un host la cui chiave è cambiata, il messaggio è `SSH host key has changed` con un suggerimento `ssh-keygen -R <host>` al suo posto.

Claude Code clona con `StrictHostKeyChecking=yes`, quindi rifiuta un host la cui chiave non è stata ancora accettata piuttosto che accettare la chiave automaticamente. Connettersi una volta dal terminale per accettare l'impronta digitale, quindi riprovare:

```shell theme={null}
ssh -T git@github.com
```

Per un repository pubblico, aggiungere il marketplace tramite il suo URL `https://` al suo posto per evitare completamente SSH.

<h3 id="command-git-not-found-or-is-in-an-unsafe-location">
  `Command 'git' not found or is in an unsafe location`
</h3>

Su Windows, è stato aggiunto un marketplace e Claude Code ha segnalato `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)`.

Claude Code cerca `git` su `PATH` e rifiuta di eseguirne uno trovato solo nella directory corrente. Per correggerlo, installare Git e riprovare:

<Steps>
  <Step title="Install Git for Windows">
    Installare Git for Windows in modo che `git` sia su `PATH`.
  </Step>

  <Step title="Open a new terminal">
    Aprire un nuovo terminale in modo che il `PATH` aggiornato si applichi.
  </Step>

  <Step title="Confirm git runs">
    Confermare che `git --version` stampa una versione.
  </Step>

  <Step title="Retry the add">
    Eseguire di nuovo il comando `marketplace add`.
  </Step>
</Steps>

<h3 id="git-clone-timed-out-after-120s">
  `Git clone timed out after 120s`
</h3>

È stato aggiunto o aggiornato un marketplace e ha fallito con `Git clone timed out after 120s`, seguito da un suggerimento per impostare `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`.

La clonazione di un marketplace e la re-clonazione per aggiornarlo ottiene 120 secondi per impostazione predefinita. Per un repository di grandi dimensioni o una connessione lenta, aumentare il limite. Il valore è in millisecondi:

<Tabs>
  <Tab title="Bash or Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
    ```
  </Tab>
</Tabs>

Quindi riprovare nella stessa shell.

Se il repository è un monorepo, limitare il checkout alle directory che si nominano con `claude plugin marketplace add <source> --sparse <paths>`.

<h3 id="marketplace-updates-keep-failing-offline">
  Marketplace updates keep failing offline
</h3>

Si lavora in un ambiente in cui l'host git del marketplace non è raggiungibile e ogni sessione ripete un aggiornamento fallito in background. Il checkout esistente del marketplace rimane in posizione e l'avvio non è ritardato.

Ogni sessione, per un marketplace con [auto-update on](/docs/it/plugins/loading#which-marketplaces-and-plugins-auto-update), Claude Code controlla l'host git del marketplace per nuovi commit in background. Quando quel controllo non può raggiungere l'host, tenta di clonare di nuovo il marketplace e offline anche quel clone fallisce.

Impostare questa variabile per saltare il tentativo di re-clone e continuare a utilizzare il checkout esistente quando il controllo non può raggiungere l'host:

<Tabs>
  <Tab title="Bash or Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
    ```
  </Tab>
</Tabs>

Con la variabile impostata, Claude Code salta il re-clone solo per un checkout che contiene già `.claude-plugin/marketplace.json`. Un marketplace che non è mai stato clonato o il cui clone si è fermato a metà ottiene comunque il tentativo di clone, quindi aggiungerlo una volta mentre online.

Per una distribuzione completamente offline, pre-popolare la directory dei plugin al momento della compilazione dell'immagine con `CLAUDE_CODE_PLUGIN_SEED_DIR` al suo posto, seguendo [Seed containers and CI](/docs/it/plugins/org#seed-containers-and-ci).

<h3 id="marketplace-add-fails-on-a-github-enterprise-server-host">
  Marketplace add fails on a GitHub Enterprise Server host
</h3>

È stato aggiunto un marketplace da un URL GitHub Enterprise Server (GHES) e si è ricevuto un errore di politica, oppure è stato aggiunto da claude.ai e si è ricevuto un errore di accesso a GitHub.

Entrambi i casi sono sulla pagina GHES:

* [Un errore di politica](/docs/it/github-enterprise-server#marketplace-add-fails-with-a-policy-error) significa che l'organizzazione ha limitato le fonti del marketplace e un amministratore deve aggiungere un `hostPattern` per l'host
* [Un errore di accesso a GitHub su claude.ai](/docs/it/github-enterprise-server#marketplace-add-on-claude-ai-fails-with-a-github-access-error) significa che il proprio account GitHub Enterprise non è ancora connesso

<h2 id="install-a-plugin">
  Install a plugin
</h2>

È stato aggiunto un marketplace ed è stata eseguita un'installazione e l'installazione si è fermata con un messaggio invece di installare qualcosa. Queste voci coprono quei messaggi. Coprono anche i messaggi correlati che appaiono in seguito nella scheda **Errors** di `/plugin` o come una scheda **Discover** vuota, quando un plugin o il suo marketplace non può essere trovato, letto o considerato attendibile.

<h3 id="plugin-not-found-in-marketplace">
  `Plugin "<name>" not found in marketplace "<marketplace>"`
</h3>

È stato eseguito `/plugin install <name>@<marketplace>` o `claude plugin install <name>@<marketplace>` e il nome del plugin non è nella copia del catalogo di quel marketplace sulla macchina.

`claude plugin install` nella shell stampa lo stesso messaggio quando non è stato aggiunto il marketplace affatto. Se `claude plugin marketplace update <marketplace>` risponde quindi con `Marketplace '<marketplace>' not found`, [aggiungere il marketplace](#add-a-marketplace) per primo.

<h4 id="the-message-ends-with-a-refresh-hint">
  `not found in marketplace` with a refresh hint
</h4>

Il suggerimento recita `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` o `The marketplace couldn't be refreshed (...)`. Claude Code non ha aggiornato il marketplace prima della ricerca, ad esempio quando si è offline, quindi la copia del catalogo potrebbe essere obsoleta. Aggiornare con il nome del marketplace, quindi installare di nuovo:

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` stampa `Successfully updated marketplace: <name>` e `/plugin marketplace update` mostra `✔ Updated 1 marketplace`. Se l'installazione riprovata stampa lo stesso messaggio, controllare il nome come [`not found in marketplace` with no hint](#the-message-has-no-hint) descrive. [When Claude Code refreshes a marketplace before an install](/docs/it/plugins/loading#when-claude-code-refreshes-a-marketplace-before-an-install) elenca gli altri casi in cui l'aggiornamento non viene eseguito.

<h4 id="the-message-has-no-hint">
  `not found in marketplace` with no hint
</h4>

Il nome è il problema più probabile. Aprire `/plugin`, andare a **Discover** e copiare il nome dall'elenco.

Prima della v2.1.232, Claude Code aggiornava il marketplace denominato solo dopo che la ricerca mancava e solo quando l'auto-aggiornamento era attivo per esso.

<h3 id="plugin-not-found-in-any-marketplace">
  `Plugin "<name>" not found in any marketplace`
</h3>

È stato eseguito `/plugin install <name>` senza `@marketplace` e nessun marketplace registrato ha quel plugin. `claude plugin install <name>` segnala `Plugin "<name>" not found in any configured marketplace`.

Senza un nome di marketplace, `claude plugin install` cerca i cataloghi che ha già e non li aggiorna per primo, e `/plugin install` aggiorna solo i marketplace che hanno l'auto-aggiornamento attivo. Nominare il marketplace e Claude Code lo aggiorna prima di cercare il plugin:

```text theme={null}
/plugin install <name>@<marketplace>
```

Quando l'installazione funziona, si vede `✓ Installed <plugin>.` in una sessione o `Successfully installed plugin: <plugin>@<marketplace>` da `claude plugin install`.

Se non si sa quale marketplace elenca il plugin, eseguire `/plugin marketplace list` per i marketplace che si hanno e sfogliare **Discover** in `/plugin` per il nome del plugin.

<h3 id="plugin-is-already-installed-globally">
  `Plugin '<name>@<marketplace>' is already installed globally`
</h3>

È stato eseguito `/plugin install` per un plugin che è già installato a livello di utente o da impostazioni gestite e Claude Code ha rifiutato con `Use '/plugin' to manage existing plugins.` Se è stato digitato il nome del plugin senza `@<marketplace>`, il messaggio omette `globally`.

Il plugin è già disponibile in ogni progetto, quindi non c'è nulla da aggiungere. Per modificare il suo [scope](/docs/it/plugins/install), abilitarlo o disabilitarlo o configurarlo, aprire `/plugin` e andare a **Installed**.

Un plugin installato solo a livello di progetto o locale non attiva questo messaggio. Claude Code consente di installarlo anche a livello di utente, quindi è disponibile in altri progetti.

`claude plugin install` nella shell stampa un messaggio diverso. Per un plugin già installato nello scope di destinazione, stampa `Plugin "<name>@<marketplace>" is already installed (scope: user)` ed esce con 0. Se la sua directory di cache è mancante, lo stesso comando la scarica di nuovo.

<h3 id="this-plugin-uses-a-source-type-your-claude-code-version-does-not-suppo">
  `This plugin uses a source type your Claude Code version does not support`
</h3>

È stato installato un plugin il cui marketplace utilizza un tipo di fonte che questa versione di Claude Code non può recuperare e Claude Code si è fermato con questo messaggio e `Update Claude Code and try again.`

Aggiornare Claude Code, quindi riprovare l'installazione. I tipi di fonte sono sul [marketplace reference](/docs/it/plugins/marketplace-reference).

<h3 id="plugin-archive-integrity-check-failed">
  `Plugin archive integrity check failed`
</h3>

È stato installato un plugin distribuito come archivio zip e Claude Code l'ha rifiutato con questa riga e `The archive was not installed.` La voce del marketplace del plugin utilizza una fonte [`archive`](/docs/it/plugins/marketplace-reference) con un pin `sha256` e il digest del file scaricato non corrisponde al pin.

Il messaggio completo è simile a questo:

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

La soluzione differisce per l'editore e l'installatore:

* **Si pubblica il plugin**: ricalcolare il digest del file esatto che l'URL serve e aggiornare il `sha256` nella voce del marketplace. Utilizzare `shasum -a 256 my-plugin.zip` o `Get-FileHash -Algorithm SHA256 my-plugin.zip` in PowerShell
* **Si installa il plugin**: eseguire `/plugin marketplace update <name>` in una sessione per aggiornare il catalogo nel caso in cui la voce sia stata corretta, quindi riprovare l'installazione. Se i digest non concordano ancora dopo l'aggiornamento, chiedere al proprietario del marketplace quale file hanno fissato prima di installare

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  `Marketplace "<name>" is registered from an untrusted source`
</h3>

Un marketplace aggiunto in precedenza ha smesso di caricarsi e così hanno fatto anche i suoi plugin. Questa riga appare nella scheda **Errors** di `/plugin` o al prossimo aggiornamento.

Il marketplace è registrato con un nome che è [riservato per i marketplace ufficiali di Anthropic](/docs/it/plugins/marketplace-reference) ma la sua fonte registrata non è un repository GitHub `anthropics`. I nomi riservati vengono ri-controllati ogni volta che un marketplace si carica o si aggiorna, quindi il marketplace e i plugin installati da esso smettono di caricarsi.

Il messaggio completo nomina il nome riservato e la soluzione:

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

La soluzione differisce per gli utenti e gli editori:

* **Si utilizza il marketplace**: nella shell, eseguire `claude plugin marketplace remove <name>`, quindi aggiungere di nuovo il marketplace dal repository ufficiale `github.com/anthropics`
* **Si pubblica un marketplace di terze parti che ha utilizzato il nome prima che diventasse riservato**: rinominarlo e chiedere agli utenti di re-aggiungerlo dalla propria fonte

Prima della v2.1.205, Claude Code controllava il nome solo quando si aggiungeva il marketplace, quindi una voce registrata prima che il suo nome diventasse riservato continuava a caricarsi.

<h3 id="plugin-has-a-corrupt-manifest-file-or-has-an-invalid-manifest-file">
  `Plugin <name> has a corrupt manifest file` or `has an invalid manifest file`
</h3>

Claude Code ha recuperato il plugin, quindi non ha potuto leggere il suo `.claude-plugin/plugin.json`. Nella shell, il `<name>` in questa riga può essere un nome di directory temporanea; il prefisso `Failed to install plugin "<name>@<marketplace>"` porta il nome reale del plugin. La formulazione dice quale controllo ha fallito:

* **`corrupt manifest file`, seguito da `JSON parse error:`**: il file non è JSON valido
* **`invalid manifest file`, seguito da `Validation errors:`**: il file si analizza ma fallisce lo schema, come `name: Invalid input` per un campo obbligatorio mancante

`claude plugin install` segnala entrambi come `Failed to install plugin "<name>@<marketplace>":` ed esce con codice 1.

L'autore del plugin deve correggere il file e il plugin non può essere installato fino ad allora:

* **Se è voi**: eseguire `claude plugin validate <plugin-directory>` nella shell per vedere lo stesso errore con il percorso offensivo, quindi correggere il file
* **Se non è voi**: segnalare il messaggio al proprietario del marketplace

<h3 id="plugin-directory-not-found-at-path">
  `Plugin directory not found at path: <path>`
</h3>

La scheda **Errors** in `/plugin` mostra questo per un plugin abilitato che il suo marketplace elenca per un percorso relativo, come `./plugins/my-plugin`, quando nessuna directory esiste in quel percorso all'interno del marketplace. Se si mantiene il marketplace, correggere il percorso `source` della voce o ripristinare la cartella. Altrimenti, segnalare il messaggio al proprietario del marketplace.

`Marketplace directory not found at path: <path>` significa che la directory del marketplace stesso è mancante al suo posto. Per un marketplace aggiunto da un percorso locale, quella directory è stata spostata o eliminata. Ripristinarla o rimuovere il marketplace e aggiungerlo di nuovo dalla sua nuova posizione.

<h3 id="no-plugins-available-or-no-marketplaces-configured">
  `No plugins available` or `No marketplaces configured`
</h3>

È stato aperto `/plugin` e la scheda **Discover** è vuota, oppure `claude plugin marketplace list` ha stampato `No marketplaces configured`.

Nessun marketplace è registrato, quindi non c'è catalogo da mostrare. In una sessione, aggiungere il marketplace ufficiale, `anthropics/claude-plugins-official`:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code stampa `Successfully added marketplace: claude-plugins-official` e **Discover** elenca i suoi plugin. La pagina [Anthropic marketplaces](/docs/it/plugins/anthropic-marketplaces) elenca gli altri marketplace che si possono aggiungere.

<h3 id="marketplace-is-already-added-from-a-different-source">
  `Marketplace "<name>" is already added from a different source`
</h3>

È stata confermata l'aggiunta di un marketplace tramite [`/plugin install <plugin> --marketplace <source>`](/docs/it/plugins/install#add-a-marketplace-and-install-in-one-command) e il catalogo che Claude Code ha recuperato da quella fonte ha lo stesso nome di un marketplace già aggiunto da una fonte diversa. Claude Code mantiene il marketplace esistente invece di sostituirlo e il plugin non viene installato.

Il messaggio completo è simile a questo:

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

Scegliere quale fonte si desidera:

* **Il marketplace già aggiunto**: installare da esso per nome con `/plugin install <plugin>@<name>`
* **La nuova fonte**: eseguire `/plugin marketplace remove <name>`, quindi riprovare l'installazione

<h3 id="cannot-add-marketplace-its-network-source-differs">
  `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings`
</h3>

È stato eseguito `marketplace add` e il catalogo in quella fonte ha lo stesso nome di un marketplace che un file di impostazioni già dichiara sotto [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces) con una fonte diversa. Claude Code rifiuta l'aggiunta e non registra nulla.

Il messaggio termina con la soluzione: la fonte deve corrispondere a quella dichiarata per questo nome nelle impostazioni, oppure si cambia la dichiarazione. Confrontare la fonte passata rispetto alla voce `extraKnownMarketplaces` per quel nome, inclusi il suo `ref`, `path` e `headers`, quindi fare uno di questi:

* **Utilizzare la fonte dichiarata**: aggiungere il marketplace dalla fonte che la voce di impostazioni nomina
* **Utilizzare la nuova fonte**: modificare o rimuovere la voce `extraKnownMarketplaces`, quindi aggiungere di nuovo il marketplace. Se le impostazioni gestite la dichiarano, chiedere all'amministratore

<h3 id="failed-to-install-from-the-plugin-menu">
  `Failed to install: <plugin> (<reason>)`
</h3>

Sono stati selezionati plugin da installare nel menu `/plugin`, nessuno di essi è stato installato e il menu si è chiuso con questo riepilogo di ciò che ha fallito.

Alcuni motivi, come l'output di git dopo un clone fallito, mostrano solo la loro prima riga. Quando tale motivo è stato abbreviato, il riepilogo termina con `Installing a plugin from its details (Enter) in /plugin shows its full error.`

Cosa fare dipende dal fatto che il riepilogo abbia abbreviato il motivo:

* Correggere ciò che il motivo tra parentesi nomina
* Quando il motivo è stato abbreviato, eseguire `/plugin`, selezionare il plugin sulla scheda **Discover** e premere **Enter** per installarlo dai suoi dettagli. Se l'installazione fallisce lì, la vista dei dettagli mostra l'errore completo

<h3 id="could-not-move-the-new-copy-of-this-plugin-version">
  `Could not move the new copy of this plugin version into <path>`
</h3>

Quando si installa un plugin, Claude Code scarica una copia fresca dei suoi file e la sposta nella cartella di quella versione nella [plugin cache](/docs/it/plugins/loading#find-plugins-on-disk). Questo messaggio significa che lo spostamento è fallito, di solito perché un altro programma stava utilizzando la cartella mentre l'installazione era in esecuzione. Il codice del file system appare tra parentesi:

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

Il messaggio dice cosa è successo alla copia che era installata prima, che dice se il plugin funziona ancora:

* `The previously installed copy was moved back`: la versione che si aveva è ancora installata
* `had to be removed first`, `was not moved back` o `could not be moved back`: quella versione del plugin non è installata fino a quando un'installazione non ha successo
* Nessuna tale frase: non c'era una copia precedente, quindi la versione non è ancora installata

Su Windows, quando un altro programma tiene la copia installata stessa, il messaggio dice invece che quella copia `could not be replaced` e che `It was not replaced and the new copy was discarded`, quindi la versione che si aveva è ancora installata.

Un elenco `Left on disk` nomina cartelle messe da parte all'interno della cache. Un'installazione successiva di quella versione o una pulizia della cache dei plugin le rimuove, quindi non è necessario eliminarle.

Per correggere l'installazione:

* Chiudere altre sessioni di Claude Code, editor e terminali che utilizzano la cartella del plugin sotto `~/.claude/plugins/cache`, quindi eseguire di nuovo l'installazione
* Quando il messaggio dice di controllare i permessi della cartella della cache dei plugin, ripristinare il permesso di scrittura sulla cartella che nomina e liberare spazio su disco, quindi eseguire di nuovo l'installazione

<h3 id="dependency-errors">
  Dependency errors
</h3>

Un plugin che dichiara dipendenze può fallire nell'installazione o installarsi e rimanere disabilitato quando una dipendenza non può essere soddisfatta. Il messaggio ti raggiunge al momento dell'installazione o al momento del caricamento:

* **Durante l'installazione**: il rifiuto torna come messaggio di errore dell'installazione
* **Quando il plugin si carica**: il problema appare in `claude plugin list` e nella scheda **Errors** di `/plugin` e Claude Code mantiene il plugin interessato disabilitato fino a quando non lo si risolve

La tabella elenca ogni messaggio e la sua soluzione. Per dichiarare le dipendenze come autore, vedere [Plugin dependencies](/docs/it/plugins/dependencies).

| Message                                                                                        | Meaning                                                                                                                  | How to resolve                                                                                                                                                                                                                                                                  |
| :--------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Dependency "<dep>" is not installed`                                                          | Una dipendenza dichiarata non è installata.                                                                              | Installarla nella shell con `claude plugin install <dep>@<marketplace>` o disinstallare il plugin. Se il marketplace della dipendenza non è ancora registrato, aggiungerlo ed eseguire `/reload-plugins` nella sessione, che installa le dipendenze mancanti che può risolvere. |
| `Dependency "<dep>" is disabled`                                                               | La dipendenza è installata ma disattivata.                                                                               | Abilitare la dipendenza o disinstallare il plugin che ne ha bisogno.                                                                                                                                                                                                            |
| `Requires "<dep>" <range>, installed <version>`                                                | La versione della dipendenza installata è al di fuori dell'intervallo dichiarato dal plugin.                             | Aggiornare la dipendenza a una versione nell'intervallo o disinstallare il plugin.                                                                                                                                                                                              |
| `<Plugin or Dependency> "<name>" has conflicting version requirements`                         | Nessuna versione soddisfa ogni intervallo che la fissa. Il messaggio elenca gli intervalli.                              | Disinstallare o aggiornare uno dei plugin in conflitto o chiedere all'autore upstream di ampliare il vincolo.                                                                                                                                                                   |
| `... has version requirements too complex to intersect` o `has an invalid version requirement` | Un intervallo non è semver valido o gli intervalli combinati non possono essere intersecati.                             | Correggere l'intervallo non valido o semplificare le catene `\|\|` lunghe.                                                                                                                                                                                                      |
| `... has no git tag satisfying <range>`                                                        | Il repository della dipendenza non ha alcun tag `<name>--v*` nell'intervallo.                                            | Controllare che i tag upstream rilascino con quella convenzione o rilassare l'intervallo.                                                                                                                                                                                       |
| `Dependency "<dep>" (required by <plugin>) is in <marketplace>, which is not in the allowlist` | La dipendenza è in un marketplace diverso e la risoluzione cross-marketplace è disattivata per impostazione predefinita. | Installare la dipendenza da soli nello stesso scope, nella shell con `claude plugin install <dep>@<marketplace>` più lo `--scope` in cui si sta installando il plugin, quindi riprovare.                                                                                        |

Per vederli a livello di programmazione, eseguire `claude plugin list --json` nella shell. I plugin con problemi portano un campo `errors` con i messaggi e un campo `errorDetails` con un `type` per ciascuno: le prime due righe sono `dependency-unsatisfied` e la terza è `dependency-version-unsatisfied`.

<h2 id="plugin-installed-but-not-working">
  Plugin installed but not working
</h2>

L'installazione è riuscita, ma le skills, gli hooks o i server del plugin non stanno facendo nulla. Iniziare con [Plugin doesn't appear or its skills don't show up](#plugin-doesnt-appear-or-its-skills-dont-show-up), che dice dove Claude Code segnala ciò che ha caricato, quindi abbinare il messaggio.

<h3 id="plugin-doesnt-appear-or-its-skills-dont-show-up">
  Plugin doesn't appear or its skills don't show up
</h3>

È stato installato un plugin e è stato digitato `/` aspettandosi le sue skills, oppure è stato chiesto a Claude di usarlo e non è successo nulla.

Controllare lo stato del plugin prima di cambiare qualcosa:

<Steps>
  <Step title="Confirm the plugin is installed and enabled">
    Eseguire `/plugin` e aprire **Installed**. Confermare che il plugin è elencato e abilitato. `claude plugin list` nella shell stampa lo stesso elenco con la versione di ogni plugin, lo scope e `Status: ✔ enabled`.
  </Step>

  <Step title="Read the Errors tab">
    Aprire la scheda **Errors** nello stesso pannello. Ogni voce accoppia un messaggio con una riga di guida. La maggior parte dei messaggi nel resto di questa sezione proviene da quella scheda.
  </Step>

  <Step title="Reload if you installed during this session">
    Se il plugin è installato e senza errori ma è stato installato durante questa sessione, eseguire `/reload-plugins`. Stampa `Reloaded:` con conteggi di plugin, skills, agent, hooks e server. Quando qualcosa non è riuscito aggiunge `N errors during load. Run /plugin for details.`
  </Step>
</Steps>

Se il plugin si carica senza errore e le sue skills non appaiono ancora, il passo successivo differisce per il proprio plugin e per quello di qualcun altro:

* **Un plugin che si sta costruendo**: vedere [Plugin loads but its skills are missing](#plugin-loads-but-its-skills-are-missing)
* **Un plugin che qualcun altro ha pubblicato**: aprire **Installed** in `/plugin` e aprire il riquadro dei dettagli del plugin, che elenca ciò che il plugin contiene. Un plugin che non elenca skills lì non ne ha da offrire quando si digita `/`

<h3 id="run-reload-plugins-to-activate">
  `Run /reload-plugins to activate.`
</h3>

Il riepilogo dell'installazione in `/plugin` è terminato con `Run /reload-plugins to activate.` invece di `Plugin is now active.`

Claude Code non ha attivato il plugin durante l'installazione, perché attivarlo avrebbe [invalidato la cache del prompt](/docs/it/prompt-caching#enabling-or-disabling-a-plugin) o perché il tentativo di attivazione è fallito.

Non è necessario digitare il comando. Il pannello si chiude e Claude Code esegue `/reload-plugins` per voi, oppure lo mette in coda fino a quando la risposta che sta trasmettendo non finisce.

Leggere ciò che quel ricaricamento stampa:

* **`Reloaded:` con conteggi di plugin, skills, agent, hooks e server**: il plugin è ora attivo. Quando qualcosa non è riuscito a caricarsi, la riga aggiunge `N errors during load. Run /plugin for details.`
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`**: il ricaricamento aggiungerebbe o rimuoverebbe un server MCP del plugin o lo strumento `LSP` e invaliderebbe la cache del prompt. Per il caso LSP la riga inizia con `This reload adds the LSP tool` o `This reload removes the LSP tool`. Eseguirlo con `--force` per attivare il plugin comunque, oppure avviare una nuova sessione

Prima della v2.1.268, un'installazione che non è stata attivata durante l'installazione è rimasta in sospeso fino a quando non è stato eseguito `/reload-plugins` da soli.

Prima della v2.1.246, il conteggio delle skills in quel riepilogo includeva solo le voci `commands/` di un plugin, quindi un ricaricamento potrebbe caricare le skills `SKILL.md` di un plugin e comunque segnalare `0 skills`.

<h3 id="plugin-not-cached-at">
  `Plugin "<name>" not cached at <path>`
</h3>

La scheda **Errors** mostra questa riga con la guida `Run /plugin to refresh the plugin cache`. Claude Code ha un record di installazione per il plugin, ma la directory a cui il record punta è mancante, ad esempio dopo aver cancellato la cache.

Reinstallare il plugin dalla shell. `claude plugin install <name>@<marketplace>` scarica di nuovo un plugin la cui directory di installazione è mancante anche se il suo record esiste:

```shell theme={null}
claude plugin install <name>@<marketplace>
```

Quindi eseguire `/reload-plugins` nella sessione. La voce della scheda **Errors** scompare e il plugin è di nuovo sotto **Installed**.

<h3 id="a-plugin-you-disabled-still-loads">
  `Disabled in ~/.claude/settings.json but still loads`
</h3>

È stato impostato un plugin su `false` in `~/.claude/settings.json` e la sua riga in `claude plugin list` o `/plugin` mostra questo messaggio seguito dalla fonte che lo abilita, come `— project settings enable it, which overrides your user setting`. Un `true` in quella fonte con precedenza più alta sta sovrascrivendo l'impostazione dell'utente.

Per rinunciare a un plugin abilitato dal progetto sulla macchina, impostare l'id su `false` in `.claude/settings.local.json`, che ha precedenza più alta rispetto al file del progetto. Per le altre fonti che il messaggio può nominare, vedere [Disabled in user settings but still loads](/docs/it/plugins/loading#disabled-in-user-settings-but-still-loads).

Se `claude plugin list` invece contrassegna il plugin `required by your org`, nessun file di impostazioni è coinvolto: l'organizzazione contrassegna quel plugin sincronizzato come richiesto su claude.ai e si carica anche se è stato disabilitato in precedenza. Vedere [Plugins synced from claude.ai](/docs/it/plugins/loading#synced-plugins).

<h3 id="plugin-is-enabled-in-project-settings-but-isnt-installed-here">
  `Plugin "<name>" is enabled in project settings but isn't installed here`
</h3>

La scheda **Errors** mostra questa riga per un plugin che il `.claude/settings.json` del progetto abilita, con la guida `Run claude plugin install <name>@<marketplace> --scope project to install it for this project`.

Le impostazioni di un repository possono abilitare un plugin per tutti coloro che lo aprono, ma non lo installano. Quando il plugin proviene da una fonte esterna come un repository GitHub o un pacchetto npm, Claude Code non lo scarica fino a quando non lo si installa da soli. Eseguire il comando dalla riga di guida nella shell, quindi ricaricare:

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

Dopo aver eseguito `/reload-plugins` nella sessione, la voce della scheda **Errors** è scomparsa e il plugin è elencato sotto **Installed**.

Se l'organizzazione pre-installa i plugin, lo fa attraverso le impostazioni gestite al suo posto. Vedere [Pre-install and require plugins](/docs/it/plugins/org#pre-install-and-require-plugins).

<h3 id="failed-to-load-hooks-from-and-hooks-that-dont-fire">
  `Failed to load hooks from <path>` and hooks that don't fire
</h3>

Gli hooks di un plugin non vengono eseguiti. O la scheda **Errors** mostra un fallimento di caricamento per loro, gli hooks si caricano e si vedono avvisi `<Event> hook error` nella trascrizione, oppure un hook si carica senza errore e non si attiva mai.

<h4 id="hooks-fail-to-load">
  Hooks fail to load
</h4>

La scheda **Errors** mostra uno di questi messaggi:

* **`Failed to load hooks from <path>: <reason>`**: `hooks/hooks.json` non è JSON valido o fallisce lo schema degli hooks. Il motivo nomina l'errore di analisi o convalida. Correggere il file. Per catturare un problema di sintassi JSON in `hooks/hooks.json` prima di pubblicare il plugin, eseguire `claude plugin validate <plugin-directory>` nella shell
* **`hooks path not found: <path>`**: il campo `hooks` del manifest nomina un file che non esiste in quel percorso relativo alla radice del plugin. Correggere il percorso o aggiungere il file

<h4 id="hook-error-notices-in-the-transcript">
  `hook error` notices in the transcript
</h4>

Un avviso della forma `... hook error: Failed with non-blocking status code: <stderr>` significa che l'hook è stato eseguito e il suo comando è fallito. Ad esempio, `Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found` significa che la shell che Claude Code ha generato non poteva trovare `node`. Installarlo o assicurarsi che sia su `PATH` del terminale da cui si avvia `claude`.

Per qualsiasi altro errore, eseguire il comando dell'hook da soli dalla directory del plugin per vedere l'output completo o catturare lo stderr completo con [debug logging](/docs/it/hooks#debug-hooks).

<h4 id="hook-loads-but-never-fires">
  Hook loads but never fires
</h4>

Se un hook si carica senza errore ma non si attiva mai, controllare la sua definizione e quindi osservarlo in esecuzione:

<Steps>
  <Step title="Check the event name">
    I nomi degli eventi sono sensibili alle maiuscole, quindi confermare che il vostro corrisponda esattamente, ad esempio `PostToolUse`.
  </Step>

  <Step title="Check the matcher">
    Confermare che il `matcher` dell'hook corrisponda al nome dello strumento.
  </Step>

  <Step title="Trigger the event on purpose">
    Per un hook `PostToolUse`, chiedere a Claude di modificare un file.
  </Step>

  <Step title="Read the debug log">
    Aprire il [debug log](/docs/it/hooks#debug-hooks), che registra quali hook hanno corrisposto. Un hook che è stato eseguito appare lì con il suo codice di uscita.
  </Step>
</Steps>

<h3 id="invalid-mcp-server-config-for-and-mcp-servers-that-dont-start">
  `Invalid MCP server config for "<server>"` and MCP servers that don't start
</h3>

Un plugin raggruppa un server MCP e la scheda **Errors** mostra `Invalid MCP server config for "<server>": <error>` oppure il server è elencato ma `/mcp` non lo mostra mai connesso.

<h4 id="invalid-mcp-server-config-for-server-error">
  `Invalid MCP server config for "<server>": <error>`
</h4>

La configurazione del server passa il controllo dello schema, ma Claude Code non può risolverla per questa sessione. Il testo dopo i due punti nomina la causa e decide la soluzione:

* **`Missing environment variables: <names>`**: impostare quelle variabili nella shell da cui si avvia Claude Code, quindi avviare una nuova sessione
* **`URL is unset or invalid`**: un'opzione `${user_config.*}` che l'URL utilizza non è impostata. Eseguire `/plugin configure <plugin>` per impostarla
* **`has an invalid MCP url`** o **`headersHelper for MCP server '<server>' references ${user_config.*}`**: la configurazione del plugin stesso è in difetto. Correggere l'`url` o `headersHelper` nella configurazione MCP del plugin o segnalarlo all'autore del plugin se il plugin non è vostro. Il caso `headersHelper` ha la sua voce sotto [plugin command references user\_config](/docs/it/errors#plugin-command-references-user-config)

<h4 id="server-is-configured-but-never-connects">
  Server is configured but never connects
</h4>

Eseguire `/mcp` per vedere lo stato del server. Quando il server è sano, `/mcp` lo elenca come connesso.

Per leggere l'errore che il server ha stampato durante l'avvio, eseguire `claude --debug` e aprire il log su `~/.claude/debug/<session-id>.txt`. Il flag `--debug` non stampa al terminale.

Una voce del server in `.mcp.json` che fallisce lo schema non appare nella scheda **Errors**. Claude Code scarta quel server e registra `Invalid MCP server config for <server> in <path>` solo in quel log di debug. Per trovare la voce senza caricare il plugin, eseguire `claude plugin validate` nella shell sulla directory del plugin, che la segnala come errore.

Prima della v2.1.281, `claude plugin validate` non controllava `.mcp.json`.

<h4 id="server-works-with-plugin-dir-but-fails-after-install">
  Server works with `--plugin-dir` but fails after install
</h4>

Siete l'autore del plugin e il server si avvia quando si carica il plugin dalla sua directory di origine con `--plugin-dir` ma fallisce una volta che il plugin è installato.

Claude Code copia un plugin installato nella sua cache, quindi un percorso che funziona solo dalla directory di origine si interrompe. Scrivere i percorsi all'interno del plugin con `${CLAUDE_PLUGIN_ROOT}`.

Per i percorsi che raggiungono al di fuori della directory del plugin, vedere [Files the plugin references outside its directory aren't found](#files-the-plugin-references-outside-its-directory-arent-found).

<h3 id="language-server-doesnt-start">
  Language server doesn't start, uses too much memory, or reports wrong diagnostics
</h3>

È stato installato un [code intelligence plugin](/docs/it/plugins/code-intelligence) e Claude non sta vedendo diagnostiche, oppure il language server sta utilizzando troppa memoria o segnalando errori che non sono reali.

<h4 id="language-server-doesn’t-start">
  Language server doesn't start
</h4>

Il plugin si connette a un binario del language server che si installa separatamente e Claude Code lo genera per nome dalla `PATH`.

La scheda **Errors** di `/plugin` mostra il fallimento con il suo motivo, come `Executable not found in $PATH: "<binary>"` e `claude --debug` lo registra come `LSP server <name> failed to start: <reason>`.

Installare il binario e confermare che sia su `PATH` del terminale da cui si avvia `claude`, ad esempio con `which typescript-language-server`. Quindi avviare una nuova sessione.

<h4 id="language-server-uses-too-much-memory">
  Language server uses too much memory
</h4>

I language server come `rust-analyzer` e `pyright` indicizzano l'intero progetto. Disabilitare il plugin con `/plugin disable <plugin>` in una sessione e affidarsi agli strumenti di ricerca integrati di Claude al suo posto.

<h4 id="false-positive-diagnostics-in-a-monorepo">
  False positive diagnostics in a monorepo
</h4>

Un language server che non è configurato per lo spazio di lavoro può segnalare importazioni non risolte per i pacchetti interni. Non c'è nulla da correggere dal lato di Claude Code e le diagnostiche non impediscono a Claude di modificare il codice.

<h2 id="build-a-plugin">
  Build a plugin
</h2>

Si sta sviluppando un plugin e lo si carica con `--plugin-dir` o lo si installa da un marketplace locale. Queste voci coprono i fallimenti che si incontrano durante lo sviluppo di un plugin. Per i controlli da eseguire dopo ogni modifica, vedere [Test and debug](/docs/it/plugins/create#test-and-debug).

Due fallimenti che raggiungono anche gli utenti di un plugin hanno le loro voci sotto [Plugin installed but not working](#plugin-installed-but-not-working):

* **Un hook che non si attiva**: vedere [hooks that don't fire](#failed-to-load-hooks-from-and-hooks-that-dont-fire)
* **Un server MCP che non si avvia**: vedere [MCP servers that don't start](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)

<h3 id="commands-path-not-found">
  `commands path not found: <path>`
</h3>

La scheda **Errors** mostra `commands path not found: <absolute path>` con la guida `Check that the path in your manifest or marketplace config is correct`. Lo stesso messaggio appare per `skills`, `agents` e `hooks`.

Claude Code ha risolto un percorso dal vostro `plugin.json` o dalla voce del marketplace rispetto alla radice del plugin e non ha trovato nulla lì. Il percorso nel messaggio è il percorso assoluto che ha controllato, quindi confrontarlo con ciò che è su disco. Correggere il percorso o creare la directory, quindi eseguire `/reload-plugins`.

I percorsi nel manifest sono relativi alla radice del plugin e iniziano con `./`. Un percorso che si risolve al di fuori della radice del plugin è segnalato come `<component> path escapes plugin directory` al suo posto e viene scartato.

<h3 id="plugin-dir-loads-a-plugin-with-no-components">
  `--plugin-dir` at a marketplace root doesn't load the plugins under `plugins/`
</h3>

È stato avviato `claude --plugin-dir <path>` e non si vede alcun errore, ma le skills, gli agent e gli hooks del plugin non sono lì.

`--plugin-dir` prende la directory radice del plugin, quella che contiene `.claude-plugin/plugin.json` e le directory dei componenti come `skills/`. Se lo si punta alla radice di un marketplace al suo posto, Claude Code non legge `marketplace.json`, quindi un plugin sotto `plugins/` non si carica e non si vede alcun errore. Prima della v2.1.281, Claude Code caricava una radice di marketplace come un plugin vuoto denominato dopo quella directory. Puntare il flag alla directory del plugin stesso:

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

Quindi aprire **Installed** in `/plugin`, dove il riquadro dei dettagli del plugin elenca i suoi componenti.

<h3 id="files-the-plugin-references-outside-its-directory-arent-found">
  Files the plugin references outside its directory aren't found
</h3>

Un plugin funziona dalla sua directory di origine con `--plugin-dir` ma fallisce dopo l'installazione, con errori su un percorso come `../shared-utils`.

Claude Code copia un plugin installato nella sua cache e lo carica da lì, quindi un percorso che raggiunge al di fuori della propria directory del plugin non punta a nulla nella cache. Spostare i file condivisi all'interno della directory del plugin o farvi riferimento attraverso un symlink al suo interno. Per dove è la cache e come i percorsi si risolvono, vedere [Find plugins on disk](/docs/it/plugins/loading#find-plugins-on-disk).

<h3 id="claude-plugin-root-shows-forward-slashes-on-windows">
  `${CLAUDE_PLUGIN_ROOT}` shows forward slashes on Windows
</h3>

Su Windows, un hook del plugin riceve `${CLAUDE_PLUGIN_ROOT}` come `C:/Users/you/...` piuttosto che `C:\Users\you\...` e uno script che si aspettava backslash si interrompe.

Claude Code esegue gli hook in forma di shell attraverso Git Bash su Windows e sostituisce la radice del plugin nella forma Win32 con barra in avanti di proposito. I builtin di Bash, gli strumenti MSYS e i binari Windows nativi accettano tutti quella forma.

Se lo script ha bisogno di backslash, passare l'hook a una delle forme che mantengono i percorsi nativi, descritte sotto [exec form and shell form](/docs/it/hooks#exec-form-and-shell-form):

* Un hook in forma exec, che genera il processo direttamente con un array `args`
* Un hook con `"shell": "powershell"`

<h3 id="plugin-loads-but-its-skills-are-missing">
  Plugin loads but its skills are missing
</h3>

Il plugin è elencato sotto **Installed** senza errori, ma le sue skills non vengono offerte quando si digita `/`.

Le skills si caricano da `skills/` alla radice del plugin e i comandi da `commands/` alla radice del plugin. Solo `plugin.json` appartiene all'interno di `.claude-plugin/` e una directory `skills/` all'interno di `.claude-plugin/` non viene scansionata. Spostare le directory alla radice del plugin ed eseguire `/reload-plugins`. Successivamente, il riquadro dei dettagli del plugin in `/plugin` elenca le skills e digitare `/` le offre.

Ogni skill è una directory contenente `SKILL.md`. Una voce `skills` nel manifest che punta a un file `SKILL.md` piuttosto che alla sua directory è segnalata come `path is a file; skills entries must be directories containing SKILL.md`.

<h3 id="skill-loads-but-claude-never-invokes-the-skill">
  Skill loads but Claude never invokes the skill
</h3>

La skill del plugin viene eseguita quando si digita il comando `/<plugin>:<skill>` ma Claude non la invoca mai in risposta a una richiesta semplice.

Controllare queste cause in ordine:

* **La skill imposta `disable-model-invocation: true`**: con quel campo impostato, solo voi potete invocare la skill. La skill del modello in [Create your first plugin](/docs/it/plugins/create#create-your-first-plugin) lo imposta. Rimuovere la riga da una skill che volete che Claude invochi da solo. [Control who invokes a skill](/docs/it/skills#control-who-invokes-a-skill) copre il campo
* **La descrizione non corrisponde a come le persone chiedono**: lavorare attraverso i controlli in [Skill not triggering](/docs/it/skills#skill-not-triggering)
* **La descrizione è troncata**: quando molte skills sono installate, Claude Code accorcia le descrizioni per adattarsi al budget dei caratteri dell'elenco, che può spogliare le parole chiave di cui Claude ha bisogno per abbinare una richiesta. Vedere [Skill descriptions are cut short](/docs/it/skills#skill-descriptions-are-cut-short)

Per misurare quanto spesso la skill si attiva su prompt realistici piuttosto che controllare uno alla volta, scrivere un caso di eval con un [grader `tool_used: Skill`](/docs/it/plugin-evals#create-your-first-eval-suite) ed eseguirlo con `claude plugin eval` dopo ogni modifica della descrizione.

<h3 id="is-not-a-plugin-or-skill-folder">
  `<directory> is not a plugin or skill folder` from `claude plugin eval init`
</h3>

È stato eseguito `claude plugin eval init` da una directory che non è la radice di un plugin, come la directory home o la radice di un repository che mantiene il plugin in una sottodirectory. `init` scrive la suite sotto la directory di lavoro, quindi si ferma invece di creare una directory `evals/` che il plugin non vedrebbe mai.

Cambiare alla radice del plugin, la directory che contiene `.claude-plugin/plugin.json` o il `SKILL.md` della skill ed eseguire di nuovo il comando. Per impalcare la suite da qualche parte d'altro di proposito, passare `--eval-dir`. Vedere [Test plugins with evals](/docs/it/plugin-evals).

<h3 id="the-userconfig-dialog-never-appears">
  The `userConfig` dialog never appears
</h3>

Il plugin dichiara opzioni `userConfig` ma nessuna finestra di dialogo di configurazione appare quando lo si installa.

L'installazione interattiva mostra la finestra di dialogo e il comando della shell prende i valori come flag al suo posto:

* **`/plugin install` in una sessione, o la scheda Discover in `/plugin`**: la finestra di dialogo fa parte di questa installazione interattiva
* **`claude plugin install` nella shell**: non chiede mai i valori di `userConfig`. Salva tutti i valori `--config KEY=VALUE` che si passano e quando le opzioni rimangono non impostate stampa `N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` Quando una qualsiasi delle opzioni non impostate è obbligatoria, `(M required)` segue `not yet set`.

Se è stato installato dalla shell, passare i valori con `--config`, un flag per opzione:

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

Quando ogni opzione è impostata, l'output di installazione non porta alcuna riga `not yet set`. Per aprire la finestra di dialogo in seguito al suo posto, eseguire `/plugin configure my-plugin@my-marketplace` in una sessione.

Se si passa una chiave `--config` che il manifest non dichiara, il plugin si installa comunque e il comando stampa `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.` seguito dalle chiavi che il plugin dichiara.

<h3 id="claude-plugin-validate-reports-errors">
  `claude plugin validate` reports errors
</h3>

È stato eseguito `claude plugin validate <path>` o `/plugin validate <path>` in una sessione e ha stampato `Found N errors` e `Validation failed`, quindi è uscito con codice 1.

Il validatore legge il manifest nel percorso che si fornisce: `.claude-plugin/plugin.json` per una directory di plugin o `.claude-plugin/marketplace.json` per una directory di marketplace. Per un marketplace, prefissa i problemi nel manifest della voce stessa con l'indice della voce, come `plugins[1] plugin.json → json: ...`.

La tabella copre i messaggi che fermano la convalida e due avvisi, `No frontmatter block found` e `Unknown field '<key>'`, che la fermano solo quando si passa `--strict`. Altri avvisi, come una descrizione mancante, non sono elencati.

| Message                                                                                                  | Cause                                                                        | Fix                                                                                                                              |
| :------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: <path>`                                                                                 | Il percorso non ha un manifest o non esiste.                                 | Eseguire il comando rispetto alla radice del plugin o del marketplace, la directory che contiene `.claude-plugin/`.              |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | La directory non ha alcun manifest `.claude-plugin/`.                        | Creare il manifest o puntare alla directory giusta.                                                                              |
| `Invalid JSON syntax: <parse error>`                                                                     | Il manifest o `hooks/hooks.json` non è JSON valido.                          | Correggere il JSON. Fino a quando non si corregge `hooks/hooks.json`, una sessione carica il plugin senza gli hook in quel file. |
| `Path not found: <path>. The runtime loader will report this as a load failure.`                         | Un percorso di componente nel manifest non esiste.                           | Correggere il percorso o creare la directory.                                                                                    |
| `Path contains ".." which could be a path traversal attempt: <path>`                                     | Un percorso di componente esce dalla directory del plugin.                   | Utilizzare percorsi all'interno della radice del plugin.                                                                         |
| `Path is a file; skills entries must be directories containing SKILL.md`                                 | Una voce `skills` punta a `SKILL.md` invece che alla sua directory.          | Puntare alla directory padre o `.` per un `SKILL.md` a livello di radice.                                                        |
| `No frontmatter block found` o `YAML frontmatter failed to parse: <error>`                               | Un file di skill, agent o comando ha frontmatter YAML mancante o non valido. | Aggiungere o correggere il frontmatter tra i delimitatori `---`. Segnalato durante la convalida di una directory di plugin.      |
| `Unknown field '<key>'`                                                                                  | Il manifest ha un campo che lo schema non definisce.                         | Rimuoverlo o utilizzare il nome che il messaggio suggerisce. Claude Code ignora i campi sconosciuti al momento del caricamento.  |

Eseguire il comando di nuovo dopo ogni correzione fino a quando non stampa alcun errore.

I campi `plugin.json` sono sul [manifest reference](/docs/it/plugins/manifest-reference) e i messaggi a livello di marketplace sono sotto [Marketplace validation errors](#marketplace-validation-errors).

<h3 id="plugin-has-conflicting-manifests">
  `Plugin <name> has conflicting manifests`
</h3>

Il plugin non si carica con `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.`

Il plugin ha il suo `plugin.json` e la sua voce del marketplace imposta `strict: false` mentre dichiara anche uno qualsiasi di `commands`, `agents`, `skills`, `hooks`, `outputStyles` o `themes`. Rimuovere quei campi dalla voce o impostare `strict: true` nella voce in modo che Claude Code li aggiunga a `plugin.json`. Vedere [Strict mode](/docs/it/plugins/marketplace-reference#strict-mode).

<h3 id="warning-no-commands-found-in-plugin-custom-directory">
  `Warning: No commands found in plugin <name> custom directory`
</h3>

Quando il plugin si carica, il log `claude --debug` su `~/.claude/debug/<session-id>.txt` registra `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` Nulla appare nella sessione o nella scheda **Errors**.

Il percorso `commands` nel manifest esiste ma non contiene file `.md` e nessun `SKILL.md` in una sottodirectory. Aggiungere i file di comando o rimuovere il percorso dal manifest.

<h2 id="host-a-marketplace">
  Host a marketplace
</h2>

Si pubblica un marketplace e un utente segnala un errore o la propria convalida fallisce. Queste voci sono per il proprietario del marketplace.

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Plugins with relative paths fail in URL-based marketplaces
</h3>

Gli utenti hanno aggiunto il marketplace con un URL `https://example.com/marketplace.json`. Le installazioni di plugin la cui `source` è un percorso relativo, come `./plugins/my-plugin`, falliscono con `its marketplace entry path does not stay inside the marketplace directory`. I plugin già installati non si caricano con `Plugin source path refused`. Entrambi i messaggi hanno una [voce di riferimento degli errori](/docs/it/errors#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory).

Quando un utente aggiunge un marketplace basato su URL, Claude Code scarica solo il file `marketplace.json` stesso. Non recupera i file dei plugin per percorso relativo da quel server, quindi un percorso relativo in una voce punta a una directory che non è mai stata scaricata. Dare a ogni voce una fonte che Claude Code può recuperare da solo, come un repository GitHub:

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

In alternativa, ospitare il marketplace in un repository git e dire agli utenti di aggiungerlo con l'URL del repository. Per una fonte git, Claude Code clona l'intero repository, quindi i percorsi relativi si risolvono. I tipi di fonte sono sul [marketplace reference](/docs/it/plugins/marketplace-reference).

<h3 id="marketplace-validation-errors">
  Marketplace validation errors
</h3>

È stato eseguito `claude plugin validate .` dalla directory del marketplace e ha segnalato errori o avvisi sul file del marketplace stesso.

`claude plugin validate` convalida anche ogni voce la cui `source` è un percorso locale e avverte quando la `version` della voce non concorda con il manifest del plugin stesso.

La tabella elenca i messaggi a livello di marketplace. I messaggi a livello di voce sono i messaggi del plugin sotto [`claude plugin validate` reports errors](#claude-plugin-validate-reports-errors), prefissati con `plugins[N] plugin.json →`.

| Message                                                                                                                  | Kind    | Fix                                                                                                                                                           |
| :----------------------------------------------------------------------------------------------------------------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Duplicate plugin name "<name>" found in marketplace`                                                                    | Error   | Dare a ogni plugin un `name` univoco.                                                                                                                         |
| `Path contains "..": <path>` sotto `plugins[N].source`                                                                   | Error   | Utilizzare percorsi relativi alla radice del marketplace senza segmenti `..`.                                                                                 |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                         | Error   | Rimuovere il carattere dal nome, come un escape o una nuova riga.                                                                                             |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                              | Error   | Rimuovere il carattere dal `name` del plugin.                                                                                                                 |
| `Marketplace has no plugins defined`                                                                                     | Warning | Aggiungere almeno una voce a `plugins`.                                                                                                                       |
| `No marketplace description provided`                                                                                    | Warning | Aggiungere una `description` a livello superiore.                                                                                                             |
| `Plugin name "<name>" is not kebab-case` sotto `plugins[N] plugin.json → name`                                           | Warning | Rinominare in lettere minuscole, cifre e trattini. Claude Code accetta altre forme, ma la sincronizzazione del marketplace di claude.ai le rifiuta.           |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"`                                                         | Warning | Aggiornare la voce per corrispondere a `plugin.json`, che è autorevole al momento dell'installazione.                                                         |
| `Marketplace name "<name>" is reserved in Claude Desktop`                                                                | Warning | Rinominare il marketplace. La sincronizzazione del marketplace gestito di Claude Desktop rifiuta `org`, `org-provisioned` e `unknown` in qualsiasi maiuscola. |
| `Marketplace name "<name>" is not accepted by Claude Desktop` o `Plugin name "<name>" is not accepted by Claude Desktop` | Warning | Rinominare a un massimo di 128 caratteri di lettere, cifre, `.`, `_` e `-`, iniziando con una lettera o una cifra.                                            |

Prima della v2.1.247, un nome di marketplace contenente caratteri di controllo o di formattazione bidirezionale era segnalato solo come `Marketplace name impersonates an official Anthropic/Claude marketplace`.

<h2 id="blocked-by-your-organization">
  Blocked by your organization
</h2>

L'organizzazione distribuisce impostazioni gestite che limitano i plugin e un comando è stato rifiutato con un messaggio di politica. Queste voci nominano l'impostazione dietro ogni rifiuto in modo da sapere cosa chiedere all'amministratore. Per il lato amministratore, vedere [Manage plugins for your organization](/docs/it/plugins/org).

<h3 id="marketplace-source-is-blocked-by-enterprise-policy">
  `Marketplace source '<source>' is blocked by enterprise policy`
</h3>

È stato eseguito `/plugin marketplace add`, `update` o un'installazione e Claude Code ha rifiutato con questa riga. Per una fonte GitHub o git, l'host segue la fonte tra parentesi, come in `'github:owner/repo' (github.com)`.

L'amministratore ha impostato `blockedMarketplaces` o `strictKnownMarketplaces` nelle impostazioni gestite e questa fonte non è consentita. Chiedere all'amministratore di consentire la fonte o aggiungere una delle fonti consentite che il messaggio elenca.

Abbinare il resto del messaggio per vedere che tipo di politica ha bloccato la fonte:

* **`Allowed sources: <list>`**: il blocco proviene dall'elenco di autorizzazione `strictKnownMarketplaces` piuttosto che dall'elenco di blocco `blockedMarketplaces`
* **`No external marketplaces are allowed.`**: l'elenco di autorizzazione `strictKnownMarketplaces` è vuoto
* **Un `Tip:` che lo shorthand assume github.com**: l'elenco di autorizzazione consente un host git per nome host e lo shorthand `owner/repo` che è stato passato punta a github.com. Se il repository si trova sull'host interno, aggiungerlo di nuovo con il suo URL completo, come `git@your-git-host.com:owner/repo.git`

Un marketplace aggiunto prima che la politica diventasse più restrittiva smette di aggiornarsi anche perché la politica si applica ad ogni aggiornamento.

<h3 id="marketplace-is-not-in-the-allowed-marketplace-list">
  `Marketplace "<name>" is not in the allowed marketplace list`
</h3>

La scheda **Errors** mostra questa riga o `Marketplace "<name>" is blocked by enterprise policy` per un marketplace già registrato.

Le stesse impostazioni gestite che bloccano una [marketplace source](#marketplace-source-is-blocked-by-enterprise-policy) si applicano al momento del caricamento. `strictKnownMarketplaces` non include questo marketplace o `blockedMarketplaces` lo nomina, quindi Claude Code smette di caricarlo e i suoi plugin. Per la variante dell'elenco di autorizzazione, la riga di guida mostra le fonti consentite o `Contact your administrator to configure allowed marketplace sources`. Per la variante dell'elenco di blocco recita `This marketplace source is explicitly blocked by your administrator`.

<h3 id="plugin-is-blocked-by-your-organizations-policy-and-cannot-be-installed">
  `Plugin "<name>" is blocked by your organization's policy and cannot be installed`
</h3>

Un'installazione è stata rifiutata con questa riga, un'abilitazione con la stessa riga che termina `cannot be enabled` o un'installazione o aggiornamento con uno che nomina il motivo: `Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy` o `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy`.

Le impostazioni gestite bloccano questo plugin, il suo marketplace o una dipendenza di cui ha bisogno. Chiedere all'amministratore quale voce si applica. Un blocco di dipendenza significa che il plugin non può installarsi fino a quando il marketplace della dipendenza non è consentito.

<h3 id="plugin-dir-is-disabled-by-your-organizations-managed-settings-disables">
  `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`
</h3>

È stato avviato `claude` con `--plugin-dir`, `--plugin-url`, `--agents` o `--mcp-config`. Claude Code è uscito con questo messaggio e `Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

L'amministratore ha impostato `disableSideloadFlags` nelle impostazioni gestite, che disattiva i flag che caricano plugin, agent e server da percorsi arbitrari. Caricare il plugin da un marketplace approvato al suo posto o chiedere all'amministratore di rimuovere l'impostazione.

Un messaggio correlato nella scheda **Errors** di `/plugin` è `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`. Le impostazioni gestite abilitano o disabilitano quel plugin per nome e Claude Code ignora la copia `--plugin-dir` in modo che il flag non possa sovrascrivere la politica.

<h3 id="plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s">
  `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`
</h3>

È stato eseguito `claude plugin init` o `claude plugin enable` e si è fermato con questa riga. Il messaggio nomina `strictKnownMarketplaces or blockedMarketplaces` e chiede all'amministratore di aggiungere `{"source":"skills-dir"}` a `strictKnownMarketplaces` o rimuoverlo da `blockedMarketplaces`.

La fonte `skills-dir` sta per i plugin che Claude Code carica dalla directory `~/.claude/skills/`. Chiedere all'amministratore di fare il cambiamento che il messaggio nomina.

<h3 id="command-sourced-plugins-are-disabled-by-your-organizations-managed-set">
  `Command-sourced plugins are disabled by your organization's managed settings`
</h3>

È stato installato o aggiornato un plugin con una fonte `command` e si è fermato con questa riga e `The plugin was not installed or updated and its command was not run.`

L'amministratore ha impostato `disableCommandPluginSources`, quindi Claude Code rifiuta di eseguire il comando dichiarato dal marketplace che produce il plugin. L'impostazione di `allowManagedHooksOnly` da solo ha lo stesso effetto quando `disableCommandPluginSources` non è impostato. Chiedere all'amministratore se il plugin può essere pubblicato da un tipo di fonte che la politica consente.

<h3 id="marketplace-is-seed-managed">
  `Marketplace '<name>' is seed-managed`
</h3>

È stato eseguito `claude plugin marketplace update <name>` e ha fallito con `Marketplace '<name>' is seed-managed (<dir>)` e un suggerimento di chiedere all'amministratore.

Un operatore ha pre-popolato questo marketplace tramite `CLAUDE_CODE_PLUGIN_SEED_DIR` e Claude Code tratta un marketplace seed-managed come di sola lettura. Un `marketplace update` in blocco lo salta e aggiorna gli altri.

Per modificare il contenuto del marketplace, chiedere alla persona che mantiene l'immagine seed di aggiornarlo. Per la procedura, vedere [Seed containers and CI](/docs/it/plugins/org#seed-containers-and-ci).

<h2 id="next-steps">
  Next steps
</h2>

* [Plugin loading reference](/docs/it/plugins/loading): perché gli ambiti, la cache e la precedenza si comportano in questo modo
* [Plugin commands reference](/docs/it/plugins/cli-reference): flag, impostazioni predefinite, output e codici di uscita per i comandi `claude plugin`
* [Install and manage plugins](/docs/it/plugins/install): i passaggi di installazione dall'inizio
* [Manage plugins for your organization](/docs/it/plugins/org#troubleshoot-policy): risoluzione dei problemi dal lato della politica per gli amministratori
