> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento comandi plugin

> Riferimento completo per i comandi shell del plugin claude, /plugin e /reload-plugins in una sessione, e i flag che caricano un plugin per una sola sessione.

Esegui i comandi plugin come `claude plugin` dalla tua shell o da uno script, oppure come `/plugin` e `/reload-plugins` all'interno di una sessione Claude Code. Questo riferimento fornisce i flag, i valori predefiniti, l'output e i codici di uscita di ogni comando, insieme ai due flag che caricano un plugin per una sola sessione.

Esegui `claude plugin --help` sulla tua build per confermare quali sottocomandi ha la tua versione.

<Note>
  Questi casi sono trattati su altre pagine:

  * **Installa e gestisci i passaggi, e dove `/plugin` viene eseguito**: vedi [Installa e gestisci i plugin](/docs/it/plugins/install)
  * **Cosa un comando cambia su disco e quale ambito ha la precedenza**: vedi [Riferimento caricamento plugin](/docs/it/plugins/loading)
  * **Cosa significa un messaggio di errore**: vedi [Risolvi i problemi dei plugin](/docs/it/plugins/troubleshooting)
</Note>

<h2 id="claude-plugin-commands">
  Comandi claude plugin
</h2>

Esegui `claude plugin <subcommand>` dalla tua shell o da uno script, al di fuori di una sessione Claude Code. Questi sottocomandi installano e gestiscono i plugin senza aprire il pannello [`/plugin`](#plugin-in-a-session).

`claude plugins` è un alias per `claude plugin`.

Ogni sottocomando condivide questi codici di uscita, argomenti plugin e valori di ambito:

* **Codici di uscita**: `0` in caso di successo e `1` in caso di errore. `validate` aggiunge l'uscita `2` per un errore inaspettato, e `eval` aggiunge i codici elencati nella [sua sezione](#plugin-eval).
* **Argomenti plugin**: un argomento `<plugin>` è un plugin `name` o `name@marketplace`. Quando due marketplace offrono lo stesso nome, usa la forma qualificata.
* **Ambiti**: `--scope` accetta `user`, `project` o `local`, e nomina il file di impostazioni in cui il comando scrive. `update` accetta anche `managed`.

<h3 id="plugin-init">
  plugin init
</h3>

Crea lo scaffolding di un nuovo plugin in `~/.claude/skills/<name>/`. Si carica nella tua prossima sessione come `<name>@skills-dir` senza alcun passaggio di installazione.

`new` è un alias per `init`.

Per il flusso di lavoro di creazione, test e modifica che inizia con questo comando, vedi [Crea un plugin](/docs/it/plugins/create).

```bash theme={null}
claude plugin init <name> [options]
```

`<name>` diventa il nome della directory sotto `~/.claude/skills/` e il `name` del plugin nel suo manifest.

Il comando non ha un flag per un'altra posizione. Per creare lo scaffolding all'interno di un progetto, vedi [Crea un plugin](/docs/it/plugins/create).

| Flag                     | Descrizione                                                                                       |
| :----------------------- | :------------------------------------------------------------------------------------------------ |
| `--description <text>`   | Descrizione del manifest                                                                          |
| `--author <name>`        | Nome dell'autore. Predefinito a `git config user.name`                                            |
| `--author-email <email>` | Email dell'autore. Predefinito a `git config user.email`                                          |
| `--with <components...>` | Crea anche file starter per `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style` o `channel` |
| `-f, --force`            | Sovrascrivi un `.claude-plugin/` esistente al target                                              |

Crea lo scaffolding di un plugin con file skill e hook starter:

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code convalida ciò che ha scritto e stampa `Created plugin "my-helper" at ~/.claude/skills/my-helper`, seguito dall'id con cui si carica e dal comando `claude plugin disable` che lo disattiva.

Claude Code esce con `1` senza scrivere quando non può creare lo scaffolding in sicurezza, e il messaggio nomina il motivo. Questi sono motivi comuni:

* Un valore `--with` sconosciuto
* Uno scaffolding esistente al target senza `--force`
* Un'impostazione gestita che blocca i plugin della directory skills

<h3 id="plugin-install">
  plugin install
</h3>

Installa un plugin da un marketplace che hai aggiunto. `i` è un alias per `install`.

```bash theme={null}
claude plugin install <plugin> [options]
```

La maggior parte dei plugin si installa senza un prompt. Per un plugin il cui entry del marketplace [esegue un comando per installarlo](/docs/it/plugins/host-marketplace) o [imposta un `headersHelper` per il suo download](/docs/it/plugins/host-marketplace#how-users-accept-a-headershelper-command), Claude Code prima stampa il comando e chiede `Run this command now? [y/N]`.

| Flag                        | Descrizione                                                                                                                                                                                                                                                                                                                                             |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-s, --scope <scope>`       | Ambito di installazione: `user`, `project` o `local`. Predefinito a `user`                                                                                                                                                                                                                                                                              |
| `--config <key=value>`      | Imposta un'opzione [`userConfig`](/docs/it/plugins/manifest-reference) che il manifest del plugin dichiara. Ripeti il flag per ogni opzione. Richiede Claude Code v2.1.147 o successivo                                                                                                                                                                      |
| `-y, --yes`                 | Accetta il comando di installazione visualizzato senza il prompt `Run this command now?`. Ignorato quando il comando viene eseguito all'interno di una sessione Claude Code, ad esempio dallo strumento Bash o da un hook. Richiede Claude Code v2.1.229 o successivo                                                                                   |
| `--accept-command <sha256>` | Accetta il comando di installazione visualizzato il cui `sha256` un'esecuzione precedente [`--json`](#plugin-json-result) ha riportato in `shownCommand`, al posto di `-y`. Non può essere combinato con `-y`. Vedi [Accetta un comando di installazione visualizzato](#accept-a-displayed-install-command). Richiede Claude Code v2.1.271 o successivo |
| `--json`                    | Stampa il risultato come un oggetto JSON sull'ultima riga di stdout invece del messaggio leggibile, per l'uso negli script. Vedi [Formato risultato JSON](#plugin-json-result). Richiede Claude Code v2.1.268 o successivo                                                                                                                              |

Passa `-y` dal tuo terminale per accettare il comando visualizzato senza il prompt. Ecco cosa succede senza un TTY e quando Claude esegue il comando:

* **stdin o stdout non è un TTY, e non passi né `-y` né `--accept-command`**: l'installazione viene rifiutata. L'output dice che il comando è stato solo visualizzato, e il codice di uscita è `1`
* **Claude esegue il comando attraverso il suo strumento Bash**: `-y` viene ignorato. Esegui il comando dal tuo terminale invece

Installa un plugin per tutti coloro che clonano il progetto:

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code stampa `Successfully installed plugin: formatter@my-marketplace (scope: project)`. Quando nulla di nuovo viene installato, l'output dice perché:

* **Già installato a quell'ambito**: l'output è `Plugin "formatter@my-marketplace" is already installed (scope: project)` e il codice di uscita è `0`
* **Rifiuti un prompt di origine comando**: l'output è `Aborted.` e il codice di uscita è `1`
* **Rifiuti un prompt `headersHelper`, o non può essere confermato senza un TTY**: l'output è `Aborted — the command was not run.` e il codice di uscita è `1`

<h4 id="plugin-json-result">
  Formato risultato JSON
</h4>

Quando passi `--json` a `plugin install`, l'ultima riga di stdout è un oggetto JSON. Analizza solo quella riga, perché Claude Code stampa qualsiasi comando che il marketplace dichiara prima di essa.

Tre campi sono sempre presenti:

* `command`: il sottocomando che è stato eseguito, ad esempio `install`
* `outcome`: `ok` o `failed`
* `message`: una descrizione leggibile del risultato

Altri campi, come `pluginId`, `scope` e `failureCode`, appaiono solo quando si applicano.

Un errore di utilizzo, come uno `--scope` non valido, non stampa alcuna riga di risultato ed esce con `1` con il motivo su stderr.

<h4 id="accept-a-displayed-install-command">
  Accetta un comando di installazione visualizzato
</h4>

Quando un'esecuzione `--json` visualizza un comando dichiarato dal marketplace e non lo esegue, il risultato `failed` porta anche un oggetto `shownCommand`. I suoi campi includono il comando come visualizzato, il plugin a cui appartiene, e lo `sha256` del comando.

Per accettare esattamente quel comando, esegui di nuovo con quello `sha256` come `--accept-command` dal tuo terminale, perché il flag non ha effetto all'interno di una sessione Claude Code. Richiede Claude Code v2.1.271 o successivo.

Lo `sha256` conta come accettazione per esattamente quel comando, plugin e catalogo marketplace. Se uno di essi è cambiato dal momento in cui il comando è stato visualizzato, Claude Code non accetta lo `sha256` e mostra di nuovo il comando. Un cambiamento che l'aggiornamento del marketplace della stessa esecuzione recupera conta anche come tale cambiamento.

Se `shownCommand.acceptCommandMatched` è `false`, lo `sha256` che hai passato non corrisponde al comando ora visualizzato. Rivedi quel comando prima di eseguire di nuovo con il suo `sha256`.

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Rimuovi un plugin installato da un ambito. `remove` e `rm` sono alias per `uninstall`.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| Flag                  | Descrizione                                                                                                                                                                                                                     |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-s, --scope <scope>` | Disinstalla dall'ambito: `user`, `project` o `local`. Predefinito a `user`                                                                                                                                                      |
| `--keep-data`         | Preserva la directory di dati persistenti del plugin, `~/.claude/plugins/data/<id>/`                                                                                                                                            |
| `--prune`             | Rimuovi anche le [dipendenze](/docs/it/plugins/dependencies) auto-installate che nessun plugin rimanente necessita                                                                                                                   |
| `-y, --yes`           | Salta il prompt di conferma `--prune`. Richiesto con `--prune` quando stdin o stdout non è un TTY                                                                                                                               |
| `--json`              | Stampa il risultato come un oggetto JSON sull'ultima riga di stdout, nello [stesso formato di `plugin install --json`](#plugin-json-result). Non può essere combinato con `--prune`. Richiede Claude Code v2.1.268 o successivo |

Disinstalla un plugin dall'ambito del progetto:

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code stampa `Successfully uninstalled plugin: formatter (scope: project)`. Quando il plugin non è installato a quell'ambito, il comando stampa una riga che inizia con `Failed to uninstall plugin "formatter@my-marketplace":` ed esce con `1`.

<h3 id="plugin-enable">
  plugin enable
</h3>

Abilita un plugin disabilitato. Per un [plugin sincronizzato da claude.ai](/docs/it/plugins/loading#synced-plugins), passa `<name>@synced` come plugin.

```bash theme={null}
claude plugin enable <plugin> [options]
```

| Flag                  | Descrizione                                                                                                                                                                             |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Ambito in cui abilitare: `user`, `project` o `local`. Auto-rilevato quando omesso                                                                                                       |
| `--json`              | Stampa il risultato come un oggetto JSON sull'ultima riga di stdout, nello [stesso formato di `plugin install --json`](#plugin-json-result). Richiede Claude Code v2.1.268 o successivo |

Senza `--scope`, il comando controlla i tuoi file di impostazioni nell'ordine local, project, user, e usa il primo ambito che menziona il plugin.

Se passi uno `--scope` dove il plugin non è dichiarato, il comando scrive un override o fallisce:

* **Un ambito che [ha precedenza](/docs/it/plugins/loading) su quello dichiarante**: Claude Code scrive un override all'ambito che hai passato. Ad esempio, `claude plugin disable formatter --scope local` disattiva un plugin abilitato a livello di progetto solo per te
* **Qualsiasi altro ambito**: il comando fallisce con `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.`

Se il plugin è già abilitato all'ambito risolto, il comando stampa `Plugin "formatter" is already enabled` ed esce con `1`. Con `--json`, il risultato ha `"failureCode": "already_in_goal_state"` e `"alreadyInGoalState": true`, quindi uno script può trattare quel caso come successo.

Quando il plugin dichiara [dipendenze](/docs/it/plugins/dependencies), Claude Code le abilita anche. Il comando fallisce in questi casi:

* **Una dipendenza non è installata**: enable fallisce e stampa il comando `claude plugin install` per ogni dipendenza mancante
* **Una dipendenza è bloccata dalla politica dei plugin della tua organizzazione**: enable fallisce e nomina la dipendenza bloccata
* **Una dipendenza è impostata a `false` a un ambito con precedenza più alta dell'ambito target**: enable fallisce. Abilita la dipendenza a quell'ambito, o passa `--scope` per scrivere lì

Riabilita un plugin ovunque sia dichiarato:

```bash theme={null}
claude plugin enable formatter
```

Claude Code stampa `Successfully enabled plugin: formatter (scope: project)`, nominando l'ambito che ha rilevato.

<h3 id="plugin-disable">
  plugin disable
</h3>

Disabilita un plugin senza disinstallarlo. Per un [plugin sincronizzato da claude.ai](/docs/it/plugins/loading#synced-plugins), passa `<name>@synced` come plugin.

```bash theme={null}
claude plugin disable [plugin] [options]
```

| Flag                  | Descrizione                                                                                                                                                                             |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-a, --all`           | Disabilita ogni plugin abilitato. Non può essere combinato con un nome di plugin o `--scope`                                                                                            |
| `-s, --scope <scope>` | Ambito in cui disabilitare: `user`, `project` o `local`. Auto-rilevato quando omesso                                                                                                    |
| `--json`              | Stampa il risultato come un oggetto JSON sull'ultima riga di stdout, nello [stesso formato di `plugin install --json`](#plugin-json-result). Richiede Claude Code v2.1.268 o successivo |

Senza `--scope`, l'ambito è auto-rilevato nello stesso ordine local, project, user di [`plugin enable`](#plugin-enable).

Se non passi né un nome di plugin né `--all`, Claude Code stampa `Please specify a plugin name or use --all to disable all plugins` ed esce con `1`. Disabilitare un plugin che è già disabilitato stampa `Plugin "formatter" is already disabled` ed esce con `1`, come [`plugin enable`](#plugin-enable) fa per un plugin già abilitato.

Il comando fallisce per un plugin che è ancora richiesto:

* **Un altro plugin abilitato [dipende da](/docs/it/plugins/dependencies) esso**: il comando fallisce e nomina i dipendenti da disabilitare prima
* **La tua organizzazione lo richiede come plugin sincronizzato**: il comando fallisce e non salva nulla

Disabilita un plugin:

```bash theme={null}
claude plugin disable formatter
```

Claude Code stampa `Successfully disabled plugin: formatter (scope: project)`.

<h3 id="plugin-update">
  plugin update
</h3>

Aggiorna un plugin all'ultima versione che il suo marketplace offre. La nuova versione si carica nella tua prossima sessione, o dopo che esegui `/reload-plugins` in una sessione in esecuzione.

```bash theme={null}
claude plugin update <plugin> [options]
```

| Flag                        | Descrizione                                                                                                                                                                                                                                                    |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | Ambito da aggiornare: `user`, `project`, `local` o `managed`. Predefinito all'ambito in cui il plugin è installato                                                                                                                                             |
| `-y, --yes`                 | Accetta un comando di installazione modificato da un plugin [command-source](/docs/it/plugins/host-marketplace), senza il prompt. Richiesto quando stdin o stdout non è un TTY, a meno che non passi `--accept-command`. Richiede Claude Code v2.1.229 o successivo |
| `--accept-command <sha256>` | Accetta il comando dichiarato dal marketplace il cui `sha256` un'esecuzione precedente [`--json`](#plugin-json-result) ha riportato in `shownCommand`, al posto di `-y`. Non può essere combinato con `-y`. Richiede Claude Code v2.1.271 o successivo         |
| `--json`                    | Stampa il risultato come un oggetto JSON sull'ultima riga di stdout, nello [stesso formato di `plugin install --json`](#plugin-json-result). Richiede Claude Code v2.1.268 o successivo                                                                        |

`managed` è l'unico ambito che puoi aggiornare ma non installare. Per i plugin installati dall'amministratore, vedi [Gestisci i plugin per la tua organizzazione](/docs/it/plugins/org).

Aggiorna un plugin:

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code stampa `Checking for updates for plugin "formatter@my-marketplace"…`, poi il risultato. Quando nulla è più recente, stampa `formatter is already at the latest version (1.0.0).` ed esce con `0`.

Puoi passare un nome di plugin semplice, che il comando abbina ai tuoi plugin installati. Quando i plugin installati da diversi marketplace condividono il nome, il comando rifiuta l'aggiornamento ed elenca i comandi `plugin-name@marketplace-name` qualificati da eseguire invece. L'aggiornamento per nome semplice richiede Claude Code v2.1.246 o successivo.

<h3 id="plugin-list">
  plugin list
</h3>

Elenca i plugin installati con la loro versione, ambito e stato.

```bash theme={null}
claude plugin list [options]
```

| Flag          | Descrizione                                                                                                |
| :------------ | :--------------------------------------------------------------------------------------------------------- |
| `--json`      | Stampa l'elenco come JSON                                                                                  |
| `--available` | Elenca anche i plugin che i tuoi marketplace offrono che non hai installato. Non ha effetto senza `--json` |

Claude Code raggruppa l'output leggibile per come ogni plugin si carica:

* **`Installed plugins:`**: plugin che hai installato da un marketplace
* **`Session-only plugins (--plugin-dir / --plugin-url):`**: plugin caricati da quei flag nello stesso comando, come in `claude --plugin-dir ./my-plugin plugin list`
* **`Skills-directory plugins (.claude/skills/*):`**: plugin che Claude Code ha trovato in una directory skills
* **`Synced from claude.ai`**: [plugin sincronizzati dal tuo account claude.ai](/docs/it/plugins/loading#synced-plugins)

Con nulla in nessun gruppo, Claude Code stampa ``No plugins installed. Use `claude plugin install` to install a plugin.``

<h4 id="json-output">
  Output JSON
</h4>

Con `--json`, Claude Code stampa un array con un oggetto per installazione. Ogni oggetto porta i campi sottostanti. `id`, `version`, `scope`, `enabled` e `installPath` sono sempre presenti, e gli altri appaiono solo quando si applicano.

| Campo          | Tipo             | Descrizione                                                                                                                                                                                                                                                                      |
| :------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`           | string           | `name@marketplace` per installazioni, `name@inline` per plugin solo sessione, `name@skills-dir` per plugin della directory skills, `name@synced` per plugin sincronizzati da claude.ai                                                                                           |
| `version`      | string           | Per un'installazione marketplace, la [versione che Claude Code ha calcolato](/docs/it/plugins/loading#versions-and-updates) all'installazione. Per un plugin solo sessione, della directory skills o sincronizzato, il `version` del manifest, o `unknown` quando non ne dichiara uno |
| `scope`        | string           | `user`, `project`, `local` o `managed` per installazioni; `user` o `project` per plugin della directory skills; `session` per plugin solo sessione; `synced` per plugin sincronizzati da claude.ai                                                                               |
| `enabled`      | boolean          | Se il plugin è abilitato nelle tue impostazioni unite                                                                                                                                                                                                                            |
| `installPath`  | string           | Directory da cui il plugin si carica                                                                                                                                                                                                                                             |
| `installedAt`  | string           | Timestamp ISO dell'installazione. Solo installazioni marketplace                                                                                                                                                                                                                 |
| `lastUpdated`  | string           | Timestamp ISO dell'ultimo aggiornamento. Solo installazioni marketplace                                                                                                                                                                                                          |
| `projectPath`  | string           | Progetto a cui appartiene l'installazione. Solo ambito `project` e `local`                                                                                                                                                                                                       |
| `mcpServers`   | object           | Le definizioni del server MCP del plugin, quando un plugin installato marketplace ne ha                                                                                                                                                                                          |
| `errors`       | array of strings | Errori di caricamento, quando il plugin non è riuscito a caricarsi                                                                                                                                                                                                               |
| `notes`        | array of strings | Avvertimenti di authoring per un plugin che si è caricato e funziona                                                                                                                                                                                                             |
| `errorDetails` | array of objects | Un oggetto per voce `errors`, fornendo il suo `type` diagnostico e i nomi a cui si riferisce, come il plugin, marketplace, server o file. Richiede Claude Code v2.1.268 o successivo                                                                                             |
| `noteDetails`  | array of objects | Gli stessi oggetti di dettaglio per ogni voce `notes`. Richiede Claude Code v2.1.268 o successivo                                                                                                                                                                                |

Con `--json --available`, Claude Code stampa un oggetto invece di un array. Il suo campo `installed` contiene l'array di oggetti plugin installati, e il suo campo `available` contiene un oggetto per plugin marketplace non installato con i campi sottostanti.

| Campo             | Tipo             | Descrizione                                                                                                                            |
| :---------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `pluginId`        | string           | `name@marketplace`                                                                                                                     |
| `name`            | string           | Il nome del plugin nel marketplace                                                                                                     |
| `marketplaceName` | string           | Il marketplace che lo offre                                                                                                            |
| `source`          | string or object | La [source](/docs/it/plugins/marketplace-reference) dell'entry del marketplace: una stringa per un percorso relativo, un oggetto altrimenti |
| `description`     | string           | La descrizione dell'entry, quando ne ha una                                                                                            |
| `version`         | string           | La versione dell'entry, quando ne dichiara una                                                                                         |
| `installCount`    | number           | Conteggio delle installazioni, quando Claude Code ne ha uno per il plugin                                                              |

<h3 id="plugin-details">
  plugin details
</h3>

Mostra l'inventario dei componenti di un plugin e il suo costo token previsto.

Il plugin deve essere caricato: installato, trovato in una directory skills, o passato con `--plugin-dir` o `--plugin-url` nello stesso comando. Il `<name>` è un plugin `name` o `name@marketplace`.

```bash theme={null}
claude plugin details <name>
```

Il comando non accetta flag oltre a `--help`.

Mostra cosa contribuisce un plugin installato:

```bash theme={null}
claude plugin details formatter
```

Claude Code stampa il nome, la versione, la descrizione e la source del plugin, poi queste sezioni:

* **`Component inventory`**: le skills, gli agents, gli hooks, i server MCP e i server LSP del plugin
* **`Projected token cost`**: i token sempre attivi che il plugin aggiunge a ogni sessione
* **`Per-component (rounded)`**: stime sempre attive e on-invoke per ogni skill, agent e comando. Omesso quando il plugin non ne ha

Per cosa significano le due figure di costo, vedi [Misura il costo e l'utilizzo del plugin](/docs/it/plugins/measure).

Per un plugin che non è caricato, Claude Code stampa ``Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.`` ed esce con `1`.

<h3 id="plugin-prune">
  plugin prune
</h3>

Rimuovi le [dipendenze](/docs/it/plugins/dependencies) auto-installate che nessun plugin installato necessita più. Il comando non rimuove mai un plugin che hai installato tu stesso. `autoremove` è un alias per `prune`.

```bash theme={null}
claude plugin prune [options]
```

| Flag                  | Descrizione                                                               |
| :-------------------- | :------------------------------------------------------------------------ |
| `-s, --scope <scope>` | Prune all'ambito: `user`, `project` o `local`. Predefinito a `user`       |
| `--dry-run`           | Elenca cosa verrebbe rimosso senza rimuoverlo                             |
| `-y, --yes`           | Salta il prompt di conferma. Richiesto quando stdin o stdout non è un TTY |

Anteprima cosa un prune rimuoverebbe:

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code elenca le dipendenze orfane e termina con `(dry run — nothing removed)`. Con nulla da rimuovere, stampa una riga che inizia con `Nothing to prune`.

Senza `--dry-run`, il comando rimuove le dipendenze orfane solo dopo che confermi al prompt o passi `-y`.

Il codice di uscita è `0` qualunque sia la tua risposta al prompt.

Cosa fa `prune` dipende dal fatto che un terminale sia collegato e se passi `-y`:

| Terminale e flag                | Cosa succede                                                                                    |
| :------------------------------ | :---------------------------------------------------------------------------------------------- |
| Terminale interattivo, no `-y`  | Elenca le dipendenze orfane e chiede `Remove? [y/N]`                                            |
| Qualsiasi terminale, `-y`       | Le rimuove e stampa `Removed N auto-installed plugins: <names>`                                 |
| stdin o stdout non-TTY, no `-y` | Stampa l'elenco e ``Not a TTY — run `claude plugin prune -y` to remove.``, non rimuovendo nulla |

<h3 id="plugin-eval">
  plugin eval
</h3>

Esegui i [casi eval](/docs/it/plugin-evals) di un plugin e riporta i risultati valutati. Richiede Claude Code v2.1.269 o successivo.

Ogni caso è un prompt più grader. Claude Code lo esegue più volte in una sessione isolata con solo il plugin target caricato, e per impostazione predefinita anche senza il plugin in modo che il rapporto mostri la differenza.

Vedi [Testa i plugin con evals](/docs/it/plugin-evals) per il formato del caso, i grader, i risultati e l'utilizzo in CI.

```bash theme={null}
claude plugin eval [target] [options]
```

Il `target` opzionale predefinito è la directory corrente e accetta uno di questi moduli:

* Una directory plugin
* Un singolo file `prompt.md` o `case.yaml`
* Un plugin installato come `name` o `name@marketplace`
* `name@skills-dir`

Metti il target prima di `--tag`, `--allow-tools` e `--json`. Ognuna di queste opzioni prende le parole che la seguono come suo valore, quindi un target scritto dopo una di esse viene letto come un tag, un nome di strumento o il percorso di output JSON invece che come target.

Questa tabella elenca le opzioni che la maggior parte delle esecuzioni usa. Esegui `claude plugin eval --help` per l'insieme completo, inclusi `--case`, `--tag`, `--output-dir`, `--report`, `--allow-real-servers`, `--keep-temp` e `--verbose`.

| Opzione                    | Descrizione                                                                                                                                                                 | Predefinito                                                                                                  |
| :------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `--runs <n>`               | Esecuzioni per caso in ogni [arm](/docs/it/plugin-evals#compare-against-a-no-plugin-baseline)                                                                                    | Il `runs` di ogni caso, altrimenti 3                                                                         |
| `-j, --concurrency <n>`    | Sessioni di agent da eseguire contemporaneamente, da 1 a 8. Condividono il tuo limite di velocità                                                                           | `1`                                                                                                          |
| `--model <model>`          | Modello per l'agent in test                                                                                                                                                 | Il `model` di ogni caso, altrimenti `ANTHROPIC_MODEL` se impostato, altrimenti il predefinito di Claude Code |
| `--judge-model <model>`    | Modello per i grader `llm` e `baseline`                                                                                                                                     | Un modello piccolo e veloce                                                                                  |
| `--ablation <mode>`        | `none` o `with-without`. Vedi [Confronta con una baseline senza plugin](/docs/it/plugin-evals#compare-against-a-no-plugin-baseline)                                              | `with-without` quando un plugin si risolve, altrimenti `none`                                                |
| `--threshold <0..1>`       | Esci con 1 se un caso qualsiasi punteggia sotto questo                                                                                                                      | `1.0`                                                                                                        |
| `--max-cost-usd <usd>`     | Ferma prima della prossima esecuzione una volta che la spesa raggiunge questo, esci con 2 e riporta risultati parziali                                                      | Nessun limite                                                                                                |
| `--allow-tools <tools...>` | Concedi strumenti oltre al set di sola lettura, come `Bash`, `Write`, `Edit` o `"mcp__plugin_<plugin>_<server>__*"`. Vedi [Concedi strumenti](/docs/it/plugin-evals#grant-tools) |                                                                                                              |
| `--scaffold`               | Esegui lo [`scaffold_script`](/docs/it/plugin-evals#add-setup-or-history-with-case-yaml) di ogni caso                                                                            | Off                                                                                                          |
| `--trust-plugin`           | Salta il prompt di fiducia della prima esecuzione, per CI. Vedi [Cosa può accedere un'esecuzione](/docs/it/plugin-evals#security)                                                | Off                                                                                                          |
| `--mocks <mode>`           | `record` o `off`. Vedi [Mock dei server MCP](/docs/it/plugin-evals#mock-mcp-servers)                                                                                             | `record`                                                                                                     |
| `--eval-dir <dir>`         | Directory sotto il plugin che contiene i casi                                                                                                                               | L'`experimental.evals` del manifest, altrimenti `evals`                                                      |
| `--json [path]`            | Stampa il [documento risultato](/docs/it/plugin-evals#json-result) su stdout, o scrivilo in un percorso `.json`                                                                  |                                                                                                              |
| `--no-publish`             | Mantieni il rapporto HTML locale                                                                                                                                            |                                                                                                              |

Il codice di uscita riporta come è terminata l'esecuzione. Per agire su di esso in una pipeline, vedi [Esegui evals in CI](/docs/it/plugin-evals#run-evals-in-ci).

| Codice di uscita | Significato                                                                      |
| :--------------- | :------------------------------------------------------------------------------- |
| `0`              | Ogni caso soddisfa la soglia                                                     |
| `1`              | Un caso fallito, un errore di caricamento o una directory plugin non attendibile |
| `2`              | Un'esecuzione parziale                                                           |
| `130`            | Interrotto                                                                       |
| `143`            | Terminato                                                                        |

<h3 id="plugin-eval-init">
  plugin eval init
</h3>

Crea una suite eval per il plugin nella directory corrente. Richiede Claude Code v2.1.269 o successivo. Vedi [Crea la tua prima suite eval](/docs/it/plugin-evals#create-your-first-eval-suite).

```bash theme={null}
claude plugin eval init [name] [options]
```

In un terminale, il comando apre una sessione Claude Code interattiva per un'intervista di authoring. Nell'intervista, Claude fa quanto segue:

1. Legge il plugin
2. Ti chiede cosa dovrebbe fare bene
3. Propone casi e grader
4. Scrive i file dei casi
5. Esegue i casi e rivede i voti con te per verificare che i grader puntino come faresti tu

Con `--bare`, o senza un terminale, il comando scrive un template di caso singolo vuoto. Quando Claude esegue il comando dall'interno di una sessione Claude Code, il comando stampa le istruzioni dell'intervista per quella sessione da seguire piuttosto che scrivere un template.

Il `name` opzionale è un nome di caso. È richiesto con `--bare` o senza un terminale, perché il comando scrive il template vuoto per quel caso. L'intervista non ne ha bisogno.

Il comando accetta queste opzioni:

| Opzione             | Descrizione                                                                                      | Predefinito                                             |
| :------------------ | :----------------------------------------------------------------------------------------------- | :------------------------------------------------------ |
| `--bare`            | Scrivi un `prompt.md` e `graders/criteria.md` vuoti per `<name>` invece di eseguire l'intervista |                                                         |
| `-i, --interactive` | Richiedi l'intervista. Fallisce senza un terminale invece di scrivere un template                |                                                         |
| `--eval-dir <dir>`  | Directory sotto la directory corrente in cui scrivere i casi                                     | L'`experimental.evals` del manifest, altrimenti `evals` |

<h3 id="plugin-tag">
  plugin tag
</h3>

Crea un tag git annotato denominato `<name>--v<version>` per un rilascio di plugin. Prima di taggare, il comando controlla che il `plugin.json` del plugin e qualsiasi entry del marketplace che lo elenca siano d'accordo sulla versione.

Per quando taggare un rilascio, vedi [Pubblica un plugin](/docs/it/plugins/publish).

```bash theme={null}
claude plugin tag [path] [options]
```

Il `[path]` è la directory del plugin, predefinito alla directory corrente. Il comando trova l'entry del marketplace camminando su da quella directory a un `.claude-plugin/marketplace.json` che elenca il plugin.

| Flag                  | Descrizione                                                                                  |
| :-------------------- | :------------------------------------------------------------------------------------------- |
| `--push`              | Spingi il tag a `--remote` dopo averlo creato                                                |
| `--dry-run`           | Stampa cosa verrebbe taggato senza creare il tag                                             |
| `-f, --force`         | Salta i controlli dell'albero di lavoro sporco e del tag già esistente                       |
| `-m, --message <msg>` | Messaggio di annotazione del tag. `%s` sta per la versione. Predefinito a `<name> <version>` |
| `--remote <name>`     | Remote a cui spingere con `--push`. Predefinito a `origin`                                   |

Anteprima il tag per un plugin in un checkout marketplace:

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code stampa il piano:

* Il nome del plugin
* La versione e da quale file proviene
* L'entry del marketplace corrispondente, quando ce n'è uno
* Il nome del tag
* I comandi `git tag` e `git push` che eseguirebbe

Senza `--dry-run`, Claude Code stampa `Created tag formatter--v1.0.0` e sia `Pushed to origin` o il comando push da eseguire tu stesso. Se il push fallisce, il tag è ancora creato localmente e il comando esce con un errore.

Il comando esce con `1` e stampa il motivo quando non può taggare in sicurezza. I motivi comuni sono:

* Nessun `version` in `plugin.json` o nell'entry del marketplace
* Il tag esiste già
* L'albero di lavoro è sporco

<h3 id="plugin-validate">
  plugin validate
</h3>

Convalida un manifest plugin, un manifest marketplace, o le skills, gli agents e i comandi in una directory, ed esci con un codice su cui un lavoro CI può agire. Per il flusso di lavoro di creazione, test e modifica, vedi [Crea un plugin](/docs/it/plugins/create). Per cosa il validatore controlla in ogni manifest, vedi il [riferimento manifest plugin](/docs/it/plugins/manifest-reference) e il [riferimento marketplace](/docs/it/plugins/marketplace-reference).

```bash theme={null}
claude plugin validate <path> [options]
```

| Flag       | Descrizione                                                                                                                                                                           |
| :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--strict` | Tratta gli avvertimenti come errori, quindi i campi non riconosciuti e i metadati mancanti che il runtime tollera falliscono l'esecuzione. Richiede Claude Code v2.1.145 o successivo |
| `--json`   | Stampa il rapporto di convalida come un oggetto JSON con gli stessi codici di uscita. Richiede Claude Code v2.1.259 o successivo                                                      |

Convalida un plugin prima di eseguirne il commit:

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h4 id="validate-a-directory">
  Convalida una directory
</h4>

Il `<path>` è un file manifest o una directory. Data una directory, Claude Code sceglie cosa convalidare da quello che trova lì:

* `.claude-plugin/marketplace.json`, quando esiste
* Altrimenti `.claude-plugin/plugin.json`
* Altrimenti i file dei componenti, scelti dal nome della directory. La convalida dei file dei componenti senza un manifest richiede Claude Code v2.1.233 o successivo:
  * Una directory denominata `skills`, `agents` o `commands`: i file al suo interno
  * Una directory denominata `.claude`: le directory `skills`, `agents` e `commands` al suo interno
  * Qualsiasi altra directory: quelle tre directory sotto il suo `.claude`

Claude Code non segue i symlink all'interno della directory che nomini. Cosa fa dipende da dove si trova il link:

* **Una directory `skills`, `agents` o `commands` collegata sotto la root del plugin o `.claude`**: Claude Code avverte che nulla in essa è stato letto.
* **Un'entry collegata all'interno di una directory `skills`, `agents` o `commands`**: Claude Code la salta e avverte, per directory, quante entry ha saltato che una sessione caricherà.
* **La directory `skills`, `agents` o `commands` che nomini è essa stessa un symlink, o la sua directory padre `.claude` è**: Claude Code riporta un errore e non controlla nulla in essa. Nomina la directory reale invece.

Alcuni file non vengono letti da un'esecuzione di convalida:

* **Un `SKILL.md` alla root del plugin**: quando esegui `claude plugin validate` su una directory plugin, Claude Code non controlla un `SKILL.md` alla root del plugin
* **Un `CLAUDE.md` alla root del plugin**: in un'esecuzione plugin, Claude Code avverte anche di un `CLAUDE.md` alla root del plugin
* **File plugin in un'esecuzione marketplace**: da una directory marketplace, Claude Code non apre i file skill, agent, command o hook dei plugin. Per trovare errori in quei file, convalida ogni directory plugin

<h4 id="output-and-exit-codes">
  Output e codici di uscita
</h4>

Claude Code stampa il file che ha convalidato, eventuali errori e avvertimenti con i loro percorsi, e una riga di verdetto. Il codice di uscita segue il verdetto:

| Codice di uscita | Riga di verdetto                                                               | Significato                                                      |
| :--------------- | :----------------------------------------------------------------------------- | :--------------------------------------------------------------- |
| `0`              | `Validation passed` o `Validation passed with warnings`                        | Il manifest si carica. Con `--strict`, nemmeno avvertimenti      |
| `1`              | `Validation failed` o `Validation failed (--strict treats warnings as errors)` | Un errore, o un avvertimento sotto `--strict`                    |
| `2`              | `Unexpected error during validation: <reason>`                                 | Il validatore stesso ha fallito, come su un percorso illeggibile |

Con `--json`, Claude Code scrive il rapporto su stdout come un oggetto JSON con questi campi di livello superiore:

* `success`: lo stesso verdetto che il codice di uscita fornisce
* `strict`: se l'esecuzione ha trattato gli avvertimenti come errori
* `target`: il percorso risolto che Claude Code ha convalidato
* `manifest`: il risultato del manifest stesso, o `null` per un'esecuzione senza manifest
* `contents`: risultati per file, ognuno nominando il suo `file` e portando array `errors`, `warnings` e `notes`

All'uscita `2`, il comando non scrive nulla su stdout. Il messaggio di errore va su stderr.

<h2 id="claude-plugin-marketplace-commands">
  Comandi claude plugin marketplace
</h2>

Esegui `claude plugin marketplace <subcommand>` dalla tua shell per aggiungere, elencare, aggiornare e rimuovere i marketplace da cui installi i plugin.

* **Codici di uscita**: questi sottocomandi seguono la [convenzione del codice di uscita](#claude-plugin-commands) dei comandi plugin
* **Ambiti**: il loro flag `--scope` non ha una forma breve `-s`

Per cosa è un marketplace e come Claude Code lo memorizza nella cache, vedi [Riferimento caricamento plugin](/docs/it/plugins/loading).

<h3 id="plugin-marketplace-add">
  plugin marketplace add
</h3>

Aggiungi un marketplace da un repository GitHub, un URL git, un `marketplace.json` ospitato o un percorso locale, e dichiaralo in un file di impostazioni.

Dopo averlo aggiunto, Claude Code installa qualsiasi [dipendenza](/docs/it/plugins/dependencies) che i tuoi plugin installati stavano perdendo.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| Flag                  | Descrizione                                                                                                                                                                      |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>`     | File di impostazioni in cui dichiarare il marketplace: `user`, `project` o `local`. Predefinito a `user`                                                                         |
| `--sparse <paths...>` | Limita il checkout git a queste directory, per monorepo. Solo fonti `github` e `git`                                                                                             |
| `--claudeai`          | Leggi l'argomento come il nome di un [marketplace ospitato su claude.ai](/docs/it/plugins/install#add-from-claude-ai) invece di una fonte. Richiede Claude Code v2.1.273 o successivo |

`<source>` accetta uno qualsiasi dei moduli nella tabella sottostante, e il suo modulo decide il tipo di fonte e come Claude Code recupera il marketplace. Per l'oggetto source risultante, vedi il [riferimento marketplace](/docs/it/plugins/marketplace-reference).

| Digiti                                                                                     | Tipo di fonte | Come Claude Code lo recupera                                                                                              |
| :----------------------------------------------------------------------------------------- | :------------ | :------------------------------------------------------------------------------------------------------------------------ |
| `owner/repo`, `owner/repo#ref` o `owner/repo@ref`                                          | `github`      | Clona il repository GitHub, fissato a `ref` quando fornito. Owner e repo devono seguire le regole di denominazione GitHub |
| `user@host:path[.git][#ref]`                                                               | `git`         | Clona su SSH                                                                                                              |
| `https://example.com/repo.git[#ref]`, o un URL contenente `/_git/`                         | `git`         | Clona su HTTPS, inclusi gli URL di Azure DevOps                                                                           |
| `https://github.com/owner/repo` o `https://gitlab.com/namespace/project`                   | `git`         | Clona su HTTPS dopo aver aggiunto `.git`                                                                                  |
| Qualsiasi altro URL `http://` o `https://`, incluso un host git auto-ospitato senza `.git` | `url`         | Recupera l'URL come `marketplace.json`. Per clonare un repository lì, aggiungi `.git`                                     |
| `./path`, `../path`, `/path` o `~/path` a una directory                                    | `directory`   | Legge la directory in posizione. Su Windows, anche i moduli `.\`, `..\` e `C:\` funzionano                                |
| Gli stessi moduli di percorso, a un file `.json`                                           | `file`        | Legge il file in posizione                                                                                                |

Per un host i cui URL di clone non portano il suffisso `.git`, come AWS CodeCommit, aggiungi il marketplace come entry git in [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces). Claude Code clona un'entry git indipendentemente dal fatto che il suo URL termini in `.git`.

Claude Code clona anche un URL `gitlab.com` con sottogruppi annidati, come `https://gitlab.com/group/subgroup/project`.

Aggiungi un marketplace e condividilo con il progetto:

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code stampa `Successfully added marketplace: your-marketplace (declared in project settings)`, usando il `name` dal manifest del marketplace stesso. Un'aggiunta ripetuta o una fonte non valida stampa uno di questi risultati:

* **Marketplace già su disco**: l'output è `Marketplace 'your-marketplace' already on disk — declared in project settings` e il codice di uscita è `0`
* **Fonte non riconosciuta**: l'output è `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` e il codice di uscita è `1`
* **Host semplice come `gitlab.example.com/team/plugins`**: l'aggiunta fallisce come una scorciatoia `owner/repo` non valida, e il messaggio ti dice di aggiungere `https://` o usare un percorso locale

Aggiungi un [marketplace ospitato su claude.ai](/docs/it/plugins/install#add-from-claude-ai) per il nome stampato nella sezione `From claude.ai:` di `claude plugin marketplace list`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Con `--claudeai`, il comando rifiuta `--scope` e `--sparse`. Il marketplace è ospitato per il tuo account, non dichiarato in un file di impostazioni, quindi non puoi condividerlo attraverso il `.claude/settings.json` di un progetto.

<h3 id="plugin-marketplace-list">
  plugin marketplace list
</h3>

Elenca ogni marketplace che hai aggiunto, con la sua fonte.

```bash theme={null}
claude plugin marketplace list [options]
```

| Flag     | Descrizione               |
| :------- | :------------------------ |
| `--json` | Stampa l'elenco come JSON |

Claude Code stampa `Configured marketplaces:` e una riga `Source:` per marketplace, o `No marketplaces configured`.

Con `--json`, Claude Code stampa un array con un oggetto per marketplace, portando i campi sottostanti. Ogni campo è una stringa.

| Campo             | Descrizione                                                        |
| :---------------- | :----------------------------------------------------------------- |
| `name`            | Il nome del marketplace                                            |
| `source`          | `github`, `git`, `url`, `directory`, `file` o `claudeai`           |
| `repo`            | `owner/repo`. Solo fonti `github`                                  |
| `url`             | L'URL di clone o recupero. Solo fonti `git` e `url`                |
| `path`            | Il percorso locale. Solo fonti `directory` e `file`                |
| `ref`             | Il ramo o tag fissato. Fonti `github` e `git`, solo quando fissato |
| `installLocation` | Dove Claude Code ha memorizzato nella cache il marketplace         |

Un marketplace [claude.ai](/docs/it/plugins/install#add-from-claude-ai) aggiunto non ha un clone locale, quindi la sua entry porta i suoi identificatori claude.ai, `marketplaceId` e `organizationUuid`, al posto di `installLocation`. Porta anche `scope` quando uno è registrato, e `status`.

Se le tue sessioni di terminale [sincronizzano i plugin dal tuo account claude.ai](/docs/it/plugins/loading#synced-plugins), l'elenco di testo termina con una sezione `From claude.ai:`. Quella sezione nomina i marketplace che claude.ai elenca per il tuo account che non hai aggiunto, sia basati su git che ospitati. Richiede Claude Code v2.1.273 o successivo.

Per aggiungere un marketplace da quella sezione, vedi [Aggiungi un marketplace da claude.ai](/docs/it/plugins/install#add-from-claude-ai).

L'output `--json` copre solo i marketplace configurati e lascia la sezione fuori.

<h3 id="plugin-marketplace-remove">
  plugin marketplace remove
</h3>

Rimuovi la dichiarazione di un marketplace dalle tue impostazioni. `rm` è un alias per `remove`.

<Warning>
  Quando rimuovi un marketplace dall'ultimo ambito che lo dichiara, Claude Code elimina anche la sua cache e disinstalla ogni plugin che hai installato da esso. Senza `--scope`, il comando rimuove la dichiarazione da ogni ambito. Per aggiornare un marketplace senza perdere i suoi plugin, esegui `plugin marketplace update`.
</Warning>

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

Il `<name>` è il nome del marketplace che `plugin marketplace list` mostra, non la fonte che hai passato a `add`.

| Flag              | Descrizione                                                                                                                                            |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>` | Rimuovi la dichiarazione da un ambito di impostazioni: `user`, `project` o `local`. Senza di esso, Claude Code rimuove la dichiarazione da ogni ambito |

Rimuovi un marketplace da ogni ambito:

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code stampa `Successfully removed marketplace: your-marketplace`, aggiungendo `(from project settings)` quando lo hai scoped. Se scopi a un file di impostazioni che non dichiara il marketplace, il comando fallisce con `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.`

<h3 id="plugin-marketplace-update">
  plugin marketplace update
</h3>

Aggiorna un marketplace, o ogni marketplace, dalla sua fonte per recuperare nuovi plugin e versioni. Un marketplace aggiunto con un ramo o tag `ref` si aggiorna al commit più recente di quel ref, non al ramo predefinito del repository.

```bash theme={null}
claude plugin marketplace update [name]
```

Il comando non accetta flag oltre a `--help`.

Aggiorna un marketplace:

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code stampa `Successfully updated marketplace: your-marketplace`. Quando ometti il nome, stampa un conteggio come `Successfully updated 2 marketplaces`. Senza marketplace aggiunti, stampa `No marketplaces configured` ed esce con `0`.

<h2 id="plugin-in-a-session">
  /plugin in una sessione
</h2>

All'interno di una sessione interattiva, `/plugin` apre il pannello plugin. Ogni sottocomando apre il pannello su una scheda, esegue un'azione lì, o stampa un risultato inline. `/plugins` e `/marketplace` sono alias per `/plugin`.

Puoi eseguire questi comandi solo in una sessione di terminale interattiva. In un'esecuzione non interattiva come `claude -p`, Claude Code risponde che `/plugin` non è disponibile in questo ambiente.

Per quali superfici hanno `/plugin`, come installare senza di esso, e cosa mostra ogni scheda del pannello, vedi [Installa e gestisci i plugin](/docs/it/plugins/install).

Un `<plugin>` è un plugin `name` o `name@marketplace`.

La tabella sottostante elenca ogni forma di sessione. I sottocomandi shell `init`, `update`, `details`, `prune`, `eval` e `eval init` non hanno una forma di sessione.

| Comando                                             | Alias                                          | Cosa fa                                                                                                                                                                                                                                                                                                       |
| :-------------------------------------------------- | :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/plugin`                                           |                                                | Apre il pannello sulla scheda **Discover**. Qualsiasi prima parola non riconosciuta dopo `/plugin` fa lo stesso                                                                                                                                                                                               |
| `/plugin help`                                      | `/plugin --help`, `/plugin -h`                 | Mostra l'elenco di utilizzo dei sottocomandi `/plugin`                                                                                                                                                                                                                                                        |
| `/plugin list [--enabled\|--disabled]`              | `ls`                                           | Stampa i tuoi plugin installati da marketplace inline, con versione, ambito e stato. Un flag di filtro mostra solo quello stato. Un plugin il cui stato di abilitazione non è stato ancora applicato è contrassegnato `— run /reload-plugins to apply`. Richiede Claude Code v2.1.163 o successivo            |
| `/plugin install`                                   | `i`                                            | Apre la scheda **Discover**                                                                                                                                                                                                                                                                                   |
| `/plugin install <plugin>`                          | `i`                                            | Apre i dettagli del plugin nella scheda **Discover**. Con `name@marketplace`, li apre nell'elenco di quel marketplace                                                                                                                                                                                         |
| `/plugin install <plugin> --marketplace <source>`   | `i`                                            | Aggiunge il marketplace a `<source>` quando non l'hai ancora aggiunto, chiedendoti di confermare prima, poi apre i dettagli del plugin. Vedi [Aggiungi un marketplace e installa in un comando](/docs/it/plugins/install#add-a-marketplace-and-install-in-one-command). Richiede Claude Code v2.1.275 o successivo |
| `/plugin manage`                                    |                                                | Apre la scheda **Installed**                                                                                                                                                                                                                                                                                  |
| `/plugin stats`                                     |                                                | Apre la scheda **Stats**, in sessioni dove [`/skill-doctor`](/docs/it/skills#find-unused-skills) è disponibile. Altrove apre il pannello sulla scheda **Discover**                                                                                                                                                 |
| `/plugin enable <plugin>`                           |                                                | Apre la scheda **Installed** al plugin e lo abilita                                                                                                                                                                                                                                                           |
| `/plugin disable <plugin>`                          |                                                | Apre la scheda **Installed** al plugin e lo disabilita                                                                                                                                                                                                                                                        |
| `/plugin uninstall <plugin>`                        |                                                | Apre la scheda **Installed** al plugin e lo disinstalla                                                                                                                                                                                                                                                       |
| `/plugin configure <plugin>`                        | `config`                                       | Apre la finestra di dialogo [`userConfig`](/docs/it/plugins/manifest-reference) del plugin, o riporta che il plugin non ne dichiara nessuno. Richiede Claude Code v2.1.147 o successivo                                                                                                                            |
| `/plugin validate <path>`                           |                                                | Stampa lo stesso rapporto di `claude plugin validate`, inline                                                                                                                                                                                                                                                 |
| `/plugin tag [path] [--push] [--dry-run] [--force]` |                                                | Crea il tag di rilascio come `claude plugin tag` fa. Accetta `--push`, `--dry-run` e `--force` o `-f`; con qualsiasi altro flag o un argomento extra, Claude Code stampa l'utilizzo                                                                                                                           |
| `/plugin marketplace`                               | `market`                                       | Non fa nulla di visibile. Passa `add`, `list`, `update` o `remove`                                                                                                                                                                                                                                            |
| `/plugin marketplace add [source]`                  | `market add`                                   | Con una fonte, la aggiunge e riporta il risultato. Senza una, apre l'input **Add marketplace**                                                                                                                                                                                                                |
| `/plugin marketplace list`                          | `market list`                                  | Stampa i nomi dei tuoi marketplace inline                                                                                                                                                                                                                                                                     |
| `/plugin marketplace update [name]`                 | `market update`                                | Apre la scheda **Marketplaces**. Con un nome, aggiorna quel marketplace lì                                                                                                                                                                                                                                    |
| `/plugin marketplace remove [name]`                 | `market remove`, `market rm`, `marketplace rm` | Apre la scheda **Marketplaces**. Con un nome, rimuove quel marketplace lì                                                                                                                                                                                                                                     |

Se nomini un plugin che non è installato nel progetto corrente in `/plugin enable`, `disable`, `uninstall` o `configure`, Claude Code stampa `Plugin "<plugin>" is not installed in this project` invece di agire.

<h2 id="reload-plugins">
  /reload-plugins
</h2>

Applica le modifiche ai plugin in sospeso alla sessione in esecuzione senza riavviarla. Le modifiche in sospeso sono plugin che hai installato, aggiornato, abilitato, disabilitato o modificato su disco da quando la sessione è iniziata.

Quando chiudi il pannello `/plugin` con modifiche in sospeso che hai fatto in esso, Claude Code esegue `/reload-plugins` per te. Eseguilo tu stesso dopo le modifiche ai plugin che accadono al di fuori del pannello, come un comando `claude plugin` che hai eseguito in un altro terminale.

```text theme={null}
/reload-plugins [--force]
```

| Flag      | Descrizione                                                                                                    |
| :-------- | :------------------------------------------------------------------------------------------------------------- |
| `--force` | Applica il ricaricamento anche quando invaliderebbe la cache del prompt. `force` senza trattini funziona anche |

<h3 id="reload-summary">
  Riepilogo ricaricamento
</h3>

Claude Code ricarica ogni plugin attivo e stampa una riga di riepilogo, `Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers`, omettendo il conteggio del server MCP del plugin in una sessione senza un terminale interattivo. Quando un plugin qualsiasi ha fallito, il riepilogo aggiunge `N errors during load. Run /plugin for details.`

Il conteggio delle skills copre ogni skill che un plugin fornisce, sia le sue entry `commands/` che le skills `SKILL.md`. Il conteggio degli agents è il numero di agent caricati nella sessione, inclusi quelli che non provengono da plugin.

Quando le [dipendenze](/docs/it/plugins/dependencies) di un plugin ricaricato sono mancanti, Claude Code le installa, ricarica di nuovo, e aggiunge `(+ N dependencies: <names>) resolved` al riepilogo.

<h3 id="reloads-that-change-mcp-tools">
  Ricaricamenti che cambiano gli strumenti MCP
</h3>

Quando il ricaricamento aggiungerebbe o rimuoverebbe un server MCP del plugin o lo strumento `LSP`, e quel cambiamento invaliderebbe la [cache del prompt](/docs/it/prompt-caching#enabling-or-disabling-a-plugin), Claude Code non applica il ricaricamento. Stampa una riga come `This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.` Passa `--force` per applicarlo comunque.

<h3 id="sessions-without-an-interactive-terminal">
  Sessioni senza un terminale interattivo
</h3>

`/reload-plugins` viene eseguito anche in sessioni senza un terminale interattivo, come l'app desktop, l'Agent SDK e la [modalità non interattiva](/docs/it/headless) con `-p`. Richiede Claude Code v2.1.260 o successivo.

In quelle sessioni, il comando viene eseguito solo quando lo digiti nella sessione tu stesso, come nel prompt `-p` o nella casella del prompt dell'app desktop. Quando arriva in un altro modo, come attraverso [Remote Control](/docs/it/remote-control) o un messaggio inoltrato da Slack, il comando risponde `/reload-plugins isn't available over a remote connection in this session.` e non ricarica nulla.

Il ricaricamento in quelle sessioni non connette o disconnette i server MCP del plugin. Quei cambiamenti hanno effetto nella tua prossima sessione.

<h2 id="flags-that-load-a-plugin-for-one-session">
  Flag che caricano un plugin per una sola sessione
</h2>

Due flag `claude` caricano un plugin per una sola sessione, senza installarlo. Entrambi sono ripetibili.

Gli autori di plugin li usano per testare un plugin prima di pubblicarlo. Per il flusso di lavoro carica-modifica-ricarica, vedi [Sviluppa senza un marketplace](/docs/it/plugins/create#develop-without-a-marketplace).

| Flag                  | Descrizione                                                                                                                                                                                   | Esempio                                                                     |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `--plugin-dir <path>` | Carica un plugin da una directory o un archivio `.zip` di uno. Una cartella di plugin carica ogni cartella figlio che contiene un `.claude-plugin/plugin.json`. Ogni flag accetta un percorso | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip`                  |
| `--plugin-url <url>`  | Recupera un archivio `.zip` plugin da un URL. Ripeti il flag, o passa più URL separati da spazi in un valore tra virgolette                                                                   | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

Un plugin che uno di questi flag carica è un plugin solo sessione. `claude plugin list` lo mostra come `<name>@inline` con ambito `session`, ma solo quando lo stesso flag precede il sottocomando. Ad esempio, esegui `claude --plugin-dir ./my-plugin plugin list`.

Quando un plugin solo sessione condivide un nome con un plugin installato, Claude Code carica la copia solo sessione per quella sessione e salta quella installata. La copia installata si carica invece se hai disabilitato la copia solo sessione con `claude plugin disable <name>@inline`, o se le impostazioni gestite bloccano quel nome di plugin. Per la precedenza, vedi [Riferimento caricamento plugin](/docs/it/plugins/loading).

Un amministratore può rifiutare entrambi i flag, e le cartelle nominate nella variabile [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/it/env-vars#variables), con l'impostazione gestita [`disableSideloadFlags`](/docs/it/settings-reference#disablesideloadflags). Claude Code stampa che il flag è disabilitato dalle impostazioni gestite della tua organizzazione ed esce con `1` senza avviare.

Dall'Agent SDK, l'opzione [`plugins`](/docs/it/agent-sdk/plugins) è l'equivalente di `--plugin-dir`.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Installa e gestisci i plugin](/docs/it/plugins/install): le stesse operazioni dei passaggi, con quello che vedi in ognuno
* [Riferimento caricamento plugin](/docs/it/plugins/loading): cosa ogni comando cambia su disco e quale ambito ha effetto
* [Risolvi i problemi dei plugin](/docs/it/plugins/troubleshooting): messaggi di errore di installazione, marketplace, caricamento e convalida con le loro correzioni
* [Riferimento manifest plugin](/docs/it/plugins/manifest-reference): i campi che `claude plugin validate` controlla
