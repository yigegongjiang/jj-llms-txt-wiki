> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare le autorizzazioni

> Controlla cosa Claude Code può accedere e fare con regole di autorizzazione granulari, modalità e criteri gestiti.

Claude Code supporta autorizzazioni granulari in modo che Lei possa specificare esattamente cosa l'agente è autorizzato a fare e cosa non può fare. Le impostazioni di autorizzazione possono essere archiviate nel controllo della versione e distribuite a tutti gli sviluppatori della Sua organizzazione, nonché personalizzate dai singoli sviluppatori.

<h2 id="permission-system">
  Sistema di autorizzazione
</h2>

Claude Code utilizza un sistema di autorizzazione a livelli per bilanciare potenza e sicurezza. La tabella mostra, per ogni tipo di strumento, se la modalità Manuale chiede prima che l'azione venga eseguita. Gli altri [modalità di autorizzazione](#permission-modes) cambiano quali di questi ti chiedono; in modalità auto un classificatore esamina le azioni al tuo posto, e [come il classificatore valuta le azioni](/docs/it/permission-modes#how-the-classifier-evaluates-actions) elenca quali vede.

| Tipo di strumento | Esempio               | Approvazione richiesta                                                                                                  | Comportamento "Sì, non chiedere più"     |
| :---------------- | :-------------------- | :---------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- |
| Sola lettura      | Letture di file, Grep | No, all'interno della [directory di lavoro e directory aggiuntive](#working-directories)                                | N/A                                      |
| Comandi Bash      | Esecuzione shell      | Sì, eccetto un insieme integrato di [comandi di sola lettura](#read-only-commands)                                      | Permanentemente per repository e comando |
| Modifica di file  | Edit/Write di file    | Sì                                                                                                                      | Fino alla fine della sessione            |
| Web fetch         | WebFetch              | Sì, eccetto un insieme integrato di [domini di documentazione preapprovati](/docs/it/tools-reference#webfetch-tool-behavior) | Permanentemente per repository e dominio |
| Web search        | WebSearch             | Sì                                                                                                                      | Permanentemente per repository           |

Quando scegli "Sì, non chiedere più" e l'approvazione viene salvata permanentemente, come per un comando Bash o un dominio WebFetch, Claude Code salva la regola in `.claude/settings.local.json` alla radice del repository git, risolto attraverso [worktrees](/docs/it/worktrees) al checkout principale. La regola si applica alle sessioni future ovunque in quel repository, incluse le sessioni avviate in sottodirectory e in worktrees. Un'approvazione di modifica di file non viene salvata nel file: come mostra la tabella, dura fino alla fine della sessione. In alcuni casi, come al di fuori di un repository git o su Windows, Claude Code non utilizza la radice del repository; [Dove Claude Code cerca ogni file](/docs/it/settings#where-claude-code-looks-for-each-file) elenca quei casi e dove salva la regola invece.

Prima della v2.1.211, Claude Code salvava sempre la regola nella directory di avvio, quindi un'approvazione concessa in un worktree o sottodirectory non si applicava al resto del repository. Le regole che le versioni precedenti hanno salvato in una sottodirectory o worktree si applicano ancora alle sessioni avviate lì.

A volte un prompt di autorizzazione offre solo un'approvazione una tantum, senza opzione "non chiedere più" e senza opzione per consentire l'azione per il resto della sessione. Claude Code offre quelle opzioni solo quando il prompt può mostrarti tutto ciò che consentirebbero, quindi una regola che salvi da un prompt copre solo ciò che la sua opzione denominata. Quando un prompt offre solo l'approvazione una tantum, approva l'azione una volta, o aggiungi la regola tu stesso in [`/permissions`](#manage-permissions).

<h3 id="add-a-comment-when-you-answer-a-permission-prompt">
  Aggiungi un commento quando rispondi a un prompt di autorizzazione
</h3>

Puoi allegare una nota a Claude quando approvi o neghi una singola azione. Sulla maggior parte dei prompt di autorizzazione, inclusi Bash, PowerShell, file e prompt di strumenti MCP, spostati su **Sì** o **No** e premi `Tab` per aprire un campo di commento su quell'opzione. I prompt WebFetch e browser non offrono il campo. Le opzioni che consentono l'azione per il resto della sessione o salvano una regola non ne accettano una neanche.

Con il campo aperto, digita il commento e poi premi uno di questi tasti:

* `Enter`: invia la tua risposta con il commento allegato. Se lasci il campo vuoto, Claude Code invia la risposta senza un commento.
* `Tab`: chiude il campo senza rispondere. Claude Code mantiene il testo che hai digitato e lo invia comunque se rispondi con quell'opzione.
* `Shift+Tab`: su un prompt di file, come un prompt Edit o Write, chiude il campo come `Tab`. Prima della v2.1.235, premere `Shift+Tab` all'interno del campo selezionava invece l'opzione che consente l'azione per il resto della sessione, quindi Claude Code approvava l'azione per il resto della sessione e scartava il commento.

Claude Code consegna il commento diversamente a seconda di come hai risposto:

* **Sì**: Claude Code esegue l'azione, quindi invia il tuo commento a Claude dopo il risultato.
* **No**: Claude Code invia il tuo commento a Claude come motivo del rifiuto, e Claude continua a lavorare. Se selezioni **No** senza un commento su un prompt dalla conversazione principale, Claude Code interrompe il turno.

<h2 id="manage-permissions">
  Gestire le autorizzazioni
</h2>

Potete visualizzare e gestire le autorizzazioni degli strumenti di Claude Code con `/permissions`. Questa interfaccia utente elenca tutte le regole di autorizzazione e il file `settings.json` da cui provengono. Potete aprire la finestra di dialogo mentre Claude sta lavorando: quando aggiungete o rimuovete una regola, Claude Code applica la modifica a partire dalla prossima chiamata dello strumento di Claude nello stesso turno. Prima della versione 2.1.234, Claude Code metteva in coda il comando fino al termine del turno.

* Le regole **Allow** consentono a Claude Code di utilizzare lo strumento specificato senza approvazione manuale.
* Le regole **Ask** richiedono una conferma ogni volta che Claude Code tenta di utilizzare lo strumento specificato.
* Le regole **Deny** impediscono a Claude Code di utilizzare lo strumento specificato.

Le regole vengono valutate in ordine: deny, quindi ask, quindi allow. La prima corrispondenza in quell'ordine determina il risultato, e la specificità della regola non cambia l'ordine.

Una regola deny ampia come `Bash(aws *)` blocca ogni chiamata corrispondente, incluse le chiamate che corrispondono anche a una regola allow più ristretta come `Bash(aws s3 ls)`. Una regola allow non può creare un'eccezione da una regola deny. La stessa precedenza si applica tra ask e allow: una regola ask corrispondente richiede una conferma anche quando una regola allow più specifica corrisponde anche alla stessa chiamata.

Le regole deny si comportano diversamente a seconda che denominino uno strumento o che limitino un modello all'interno di uno. Un nome di strumento semplice come `Bash` rimuove lo strumento dal contesto di Claude interamente, quindi Claude non lo vede mai. Se aggiungete tale regola a metà sessione, Claude non può chiamare lo strumento dalla sua prossima chiamata dello strumento in poi; [Denying an entire tool](/docs/it/prompt-caching#denying-an-entire-tool) spiega cosa accade a una definizione che Claude ha già visto. Una regola limitata come `Bash(rm *)` lascia lo strumento disponibile e blocca le chiamate corrispondenti quando Claude tenta di utilizzarle.

La rimozione con nome semplice si applica a ogni strumento tranne [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior): una regola deny non può rimuoverlo mentre rimane qualsiasi altro strumento, e una regola ask non lo richiede mai.

<Note>
  Le regole di autorizzazione sono applicate da Claude Code, non dal modello. Le istruzioni nel vostro prompt o in `CLAUDE.md` determinano ciò che Claude tenta di fare, ma non cambiano ciò che Claude Code consente. Per concedere o revocare l'accesso, utilizzate `/permissions`, le regole descritte qui, una [modalità di autorizzazione](/docs/it/permission-modes), o un [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

Quando la [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) è disponibile per la vostra sessione, la finestra di dialogo include anche le [regole del classificatore della modalità auto](/docs/it/auto-mode-config#edit-rules-from-permissions). Selezionate la scheda **Auto mode** per visualizzarle.

<h2 id="permission-modes">
  Modalità di autorizzazione
</h2>

Claude Code supporta diverse modalità di autorizzazione che controllano come approva le chiamate di strumenti. Vedi [Permission modes](/docs/it/permission-modes) per quando utilizzare ciascuna. Per modificare la modalità in cui iniziano le sessioni, imposta `defaultMode` nei tuoi [file di impostazioni](/docs/it/settings#where-settings-live). [Which mode a session starts in](/docs/it/permission-modes#which-mode-a-session-starts-in) copre il valore predefinito integrato per ogni piano e cosa legge l'estensione VS Code.

| Modalità            | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Richiede l'autorizzazione al primo utilizzo di ogni strumento. Etichettato come Manual nella CLI, nelle estensioni VS Code e JetBrains, e nell'app desktop, e Claude Code accetta `manual` come alias. L'etichetta e l'alias richiedono Claude Code v2.1.200 o successivo. L'etichetta dell'app desktop non dipende dalla tua versione CLI                                                                                                                                                                                                                                                                                                                    |
| `acceptEdits`       | Accetta automaticamente le modifiche ai file e i comandi comuni del filesystem come `mkdir`, `touch`, `mv` e `cp` per i percorsi nella directory di lavoro o `additionalDirectories`                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `plan`              | Claude legge i file ed esegue comandi shell di sola lettura per esplorare ma non modifica i tuoi file sorgente; con [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) disponibile, i comandi approvati dal classificatore vengono eseguiti anche. Etichettato Plan nella CLI e nell'estensione VS Code                                                                                                                                                                                                                                                                                                                                       |
| `auto`              | Auto-approva le chiamate di strumento con controlli di sicurezza in background che verificano che le azioni si allineino con la tua richiesta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `dontAsk`           | Nega automaticamente ogni chiamata che altrimenti richiederebbe un prompt; le letture di file nelle tue directory di lavoro e altre azioni che non richiedono approvazione vengono comunque eseguite, così come gli strumenti pre-approvati tramite `/permissions` o regole `permissions.allow`. `AskUserQuestion`, strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool), e strumenti connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) nelle sessioni in cui tale impostazione raggiunge Claude Code vengono negati anche se li hai consentiti |
| `bypassPermissions` | Salta i prompt di autorizzazione, ad eccezione delle [azioni che nessuna modalità auto-approva](/docs/it/permission-modes#actions-no-mode-auto-approves)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

<Warning>
  In modalità `bypassPermissions`, Claude Code salta i prompt di autorizzazione, incluse le scritture in [percorsi protetti](/docs/it/permission-modes#protected-paths) come `.git` e `.claude`. I [safeguard di messaggistica tra sessioni](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode) si applicano comunque. Utilizza questa modalità solo in ambienti isolati come contenitori o macchine virtuali dove Claude Code non può causare danni.
</Warning>

Per prevenire che la modalità `bypassPermissions` o `auto` venga utilizzata, imposta `permissions.disableBypassPermissionsMode` o `permissions.disableAutoMode` su `"disable"` in qualsiasi [file di impostazioni](/docs/it/settings#where-settings-live). Questi sono più utili nelle [impostazioni gestite](#managed-settings) dove non possono essere ignorati.

<h2 id="permission-rule-syntax">
  Sintassi delle regole di autorizzazione
</h2>

Le regole di autorizzazione seguono il formato `Tool` o `Tool(specifier)`. Le parentesi all'interno dello specificatore sono letterali, quindi un comando o un percorso che le contiene non necessita di escape.

<h3 id="match-all-uses-of-a-tool">
  Corrispondere a tutti gli utilizzi di uno strumento
</h3>

Per corrispondere a tutti gli utilizzi di uno strumento, utilizzate solo il nome dello strumento senza parentesi:

| Regola     | Effetto                                       |
| :--------- | :-------------------------------------------- |
| `Bash`     | Corrisponde a tutti i comandi Bash            |
| `WebFetch` | Corrisponde a tutte le richieste di web fetch |
| `Read`     | Corrisponde a tutte le letture di file        |

`Bash(*)` è equivalente a `Bash` e corrisponde a tutti i comandi Bash. Come regola di negazione, entrambe le forme rimuovono lo strumento dal contesto di Claude.

<h3 id="use-specifiers-for-fine-grained-control">
  Utilizzare gli specificatori per il controllo granulare
</h3>

Aggiungete uno specificatore tra parentesi per corrispondere a utilizzi specifici dello strumento:

| Regola                         | Effetto                                                           |
| :----------------------------- | :---------------------------------------------------------------- |
| `Bash(npm run build)`          | Corrisponde al comando esatto `npm run build`                     |
| `Read(./.env)`                 | Corrisponde alla lettura del file `.env` nella directory corrente |
| `WebFetch(domain:example.com)` | Corrisponde alle richieste di fetch a example.com                 |

<h3 id="match-by-input-parameter">
  Corrispondere per parametro di input
</h3>

Le regole di negazione e richiesta possono corrispondere a un parametro di input di primo livello su qualsiasi strumento integrato con `Tool(param:value)`.

Per corrispondere a un parametro su uno strumento MCP, passate una regola di negazione con [`--disallowedTools`](/docs/it/cli-reference#cli-flags). Quando Claude Code carica un file di impostazioni, salta qualsiasi regola `mcp__` che ha parentesi. Claude Code elenca la regola saltata nella finestra di dialogo delle impostazioni non valide quando inizia una sessione interattiva e nell'output di [`claude doctor`](/docs/it/debug-your-config#check-resolved-settings).

Una regola di parametro corrisponde quando Claude chiama lo strumento con quel parametro impostato su quel valore esatto. Una regola di autorizzazione per un valore di parametro non stabilirerebbe che la chiamata è sicura nel complesso, quindi le regole di autorizzazione continuano a utilizzare la sintassi dello specificatore di ogni strumento. Questo funziona per qualsiasi parametro scalare che lo strumento accetta:

| Regola                         | Corrisponde                                              |
| :----------------------------- | :------------------------------------------------------- |
| `Agent(model:opus)`            | Chiamate Agent che richiedono il livello di modello Opus |
| `Agent(isolation:worktree)`    | Chiamate Agent che richiedono un git worktree            |
| `Bash(run_in_background:true)` | Chiamate Bash che vengono eseguite in background         |

La corrispondenza dei parametri segue queste regole:

* Il nome del parametro deve essere un campo diretto dell'input dello strumento, come `model` nello strumento Agent. I campi annidati all'interno di un oggetto o di un array non sono corrispondibili
* Ogni regola nomina un parametro. Per controllare sia `model` che `isolation`, scrivete due regole, `Agent(model:opus)` e `Agent(isolation:worktree)`, piuttosto che combinarle in una sola regola
* Il valore supporta `*` come carattere jolly che corrisponde a qualsiasi sequenza di caratteri, quindi `Agent(isolation:*)` corrisponde a qualsiasi valore di isolamento esplicito. Senza `*` la corrispondenza è esatta
* Un parametro che il modello omette non viene mai corrispondente, quindi `Agent(model:*)` non corrisponde a una chiamata che lascia `model` non impostato
* Il valore viene confrontato con l'input letterale che Claude invia, prima di qualsiasi normalizzazione. `Agent(model:opus)` corrisponde all'alias `opus` ma non a un ID modello completo. Eseguite con [`--verbose`](/docs/it/cli-reference) per vedere i nomi e i valori dei parametri esatti in ogni chiamata dello strumento
* Lo spazio intorno ai due punti viene ignorato

Non potete corrispondere al campo di contenuto primario di uno strumento in questo modo: `command` per Bash e PowerShell, `file_path` per Read, Edit e Write, `path` per Grep e Glob, `notebook_path` per NotebookEdit, e `url` per WebFetch. Una regola come `Bash(command:rm *)` sarebbe aggirabile da un comando composto, quindi Claude Code la ignora e emette un avviso all'avvio. Utilizzate invece `Bash(rm *)`, `Read(./path)`, o `WebFetch(domain:host)`.

<h3 id="wildcard-patterns">
  Modelli con caratteri jolly
</h3>

Un `*` in una regola Bash corrisponde a qualsiasi testo, inclusi gli spazi, quindi una regola copre una famiglia di comandi. Una regola senza `*` corrisponde a un comando esatto.

<Warning>
  Mettete il `*` dopo il sottocomando. In `git log --oneline main`, `git` è il programma e `log` è il sottocomando, la parola che determina cosa fa il programma. Claude Code corrisponde a tutto ciò che precede il primo `*` come scritto, quindi quelle parole sono ciò che limita la regola: `Bash(git log *)` consente solo comandi `git log`, e `Bash(git *)` consente ogni comando git. Claude Code [avvisa all'avvio](/docs/it/errors#has-a-wildcard-before-the-rest-of-the-command) di una regola di autorizzazione con un `*` prima del sottocomando, come `Bash(git * main)`.
</Warning>

Scrivete il comando che volete che Claude esegua senza chiedere, e sostituite le parti che variano con `*`. Con questa configurazione, Claude Code esegue script npm e commit git senza chiedere e rifiuta comandi che iniziano con `git push`. Un push scritto in un altro modo, come `git -C . push`, non corrisponde alla regola; consultate [a cosa non corrisponde una regola Bash](#bash-rule-limits).

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

Un `*` può andare ovunque nella regola: all'inizio, nel mezzo, o alla fine. Ogni riga mostra una regola, i comandi che corrisponde, e i comandi vicini che non corrisponde:

| Scrivete               | Corrisponde                                                                          | Non corrisponde                        |
| :--------------------- | :----------------------------------------------------------------------------------- | :------------------------------------- |
| `Bash(npm run build)`  | `npm run build`                                                                      | `npm run build --watch`                |
| `Bash(npm run *)`      | `npm run build`, `npm run test --watch`, `npm run`                                   | `npm install`                          |
| `Bash(git log * main)` | `git log --oneline main`, `git log -5 main`, `git log --output=<file> main`          | `git log main`, `git push origin main` |
| `Bash(git * main)`     | `git merge main`, `git push origin main`, `git -c core.fsmonitor=<script> diff main` | `git log`                              |
| `Bash(* --version)`    | `node --version`, `bash -c 'echo hi' --version`                                      | `node -v`                              |
| `Bash(ls *)`           | `ls -la`, `ls`                                                                       | `lsof`                                 |
| `Bash(ls*)`            | `ls -la`, `lsof`                                                                     |                                        |
| `Bash(* --help *)`     | `npm --help x`                                                                       | `npm --help`                           |

Tre regole di corrispondenza producono quelle righe:

* **Il `*` sta al posto di qualsiasi testo che si trova al suo posto.** In `Bash(git * main)`, sta al posto del sottocomando, quindi Claude Code corrisponde a ogni sottocomando git e a ogni opzione prima di esso. Questo include `-c`, che fa eseguire a git un programma che nominate. In `Bash(* --version)`, il `*` sta al posto del programma, quindi qualsiasi programma corrisponde.
* **Un `*` alla fine, con uno spazio prima di esso, corrisponde anche al comando nudo.** `Bash(ls *)` corrisponde a `ls`, e `Bash(git log *)` corrisponde a `git log`. Questo vale solo quando il `*` finale è l'unico carattere jolly della regola: `Bash(* --help *)` corrisponde a `npm --help x` ma non a `npm --help`.
* **Lo spazio prima di un `*` finale fa parte della regola.** `Bash(ls *)` richiede uno spazio dopo `ls`, quindi `lsof` non corrisponde. `Bash(ls*)` non ha spazio, quindi corrisponde anche a `lsof`.

Il suffisso `:*` è un modo equivalente per scrivere un carattere jolly finale, quindi `Bash(ls:*)` corrisponde agli stessi comandi di `Bash(ls *)`.

La finestra di dialogo di autorizzazione scrive la forma separata da spazi quando selezionate "Sì, non chiedere più" per un prefisso di comando. La forma `:*` è riconosciuta solo alla fine di un modello. In un modello come `Bash(git:* push)`, i due punti vengono trattati come un carattere letterale e non corrisponderanno ai comandi git.

<h3 id="tool-name-wildcards">
  Caratteri jolly nei nomi degli strumenti
</h3>

Le regole di negazione e richiesta accettano anche modelli glob nella posizione del nome dello strumento. Il modello deve corrispondere al nome completo dello strumento: `"*"` corrisponde a ogni strumento, e `"mcp__*"` corrisponde a ogni strumento MCP su tutti i server. Uno strumento corrispondente a una regola di negazione con nome semplice viene rimosso dal contesto di Claude, lo stesso di un nome di strumento semplice, inclusa l'eccezione [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior): una negazione glob non può rimuoverlo mentre rimane qualsiasi altro strumento, e una richiesta glob non lo chiede mai. Questa configurazione nega ogni strumento MCP:

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

Le regole di autorizzazione accettano globs nei nomi degli strumenti solo dopo un prefisso letterale `mcp__<server>__`. Il segmento del server deve essere privo di glob in modo che la regola nomini un server specifico che avete configurato. `mcp__puppeteer__*` corrisponde a ogni strumento dal server `puppeteer`, e `mcp__github__get_*` corrisponde ai suoi strumenti `get_`. Un glob di autorizzazione non ancorato come `"*"`, `"B*"`, o `"mcp__*"` viene saltato con un avviso e non approva automaticamente nulla.

Una regola di negazione o richiesta il cui nome dello strumento non corrisponde a nessuno strumento noto produce un avviso all'avvio per catturare gli errori di digitazione. I nomi degli strumenti contenenti `_` o `*` sono esenti dal controllo, e così pure i nomi degli strumenti che Claude Code ha rimosso, come `TaskOutput`.

L'etichetta mostrata per uno strumento nella trascrizione e nella finestra di dialogo di autorizzazione può differire dal suo nome canonico. Ad esempio, lo strumento etichettato `Stop Task` nella trascrizione ha il nome canonico `TaskStop`. Le regole di autorizzazione e i [matcher di hook](/docs/it/hooks) non corrispondono all'etichetta, quindi una regola scritta come `Stop Task` non corrisponde. Per le regole di negazione e richiesta, l'avviso all'avvio di cui sopra cattura la mancata corrispondenza. Utilizzate i nomi canonici elencati nel [riferimento degli strumenti](/docs/it/tools-reference).

<h2 id="tool-specific-permission-rules">
  Regole di autorizzazione specifiche dello strumento
</h2>

<h3 id="bash">
  Bash
</h3>

Le regole Bash corrispondono all'intero testo del comando, con `*` che rappresenta qualsiasi testo. [Modelli con caratteri jolly](#wildcard-patterns) mostra a quali comandi corrisponde ogni forma di regola e dove mettere il `*`. Il resto di questa sezione copre come Claude Code gestisce la corrispondenza per i comandi composti e i wrapper, a cosa non corrisponde una regola, i comandi di sola lettura e i reindirizzamenti.

<h4 id="compound-commands">
  Comandi composti
</h4>

<Tip>
  Claude Code è consapevole degli operatori shell, quindi una regola come `Bash(safe-cmd *)` non gli darà il permesso di eseguire il comando `safe-cmd && other-cmd`. I separatori di comando riconosciuti sono `&&`, `||`, `;`, `|`, `|&`, `&` e newline. Una regola deve corrispondere a ogni sottocomando indipendentemente.
</Tip>

Le regole deny e ask si applicano quando qualsiasi sottocomando le corrisponde, incluso un comando annidato all'interno di una subshell, una sostituzione di comando o un corpo di controllo di flusso come un ciclo `for`. Una regola ask come `Bash(git clean *)` vi chiede comunque un prompt per `cd /tmp && git clean -f` o `echo "$(git clean -f)"`, anche in [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode).

Quando `&&` o `||` non ha nulla dopo, come in `npm test &&`, Claude Code tratta il comando come non analizzabile e non lo divide in sottocomandi per la corrispondenza della regola allow, quindi una regola come `Bash(npm *)` non lo approva.

Quando approvate un comando composto con "Sì, non chiedere più", Claude Code salva una regola separata per ogni sottocomando che richiede approvazione, piuttosto che una singola regola per la stringa completa. Ad esempio, approvando `git status && npm test` salva una regola per `npm test`, quindi le future invocazioni di `npm test` vengono riconosciute indipendentemente da cosa precede `&&`. I sottocomandi come `cd` in una sottodirectory generano la loro propria regola Read per quel percorso. Fino a 5 regole possono essere salvate per un singolo comando composto.

<h4 id="process-wrappers">
  Wrapper
</h4>

Prima di corrispondere alle regole Bash, Claude Code rimuove un insieme fisso di wrapper, quindi una regola come `Bash(npm test *)` corrisponde anche a `timeout 30 npm test`. I wrapper rimossi sono `timeout`, `time`, `nice`, `nohup` e `stdbuf`, più i builtin shell `command` e `builtin`, e `noglob` di zsh. Ognuno esegue il suo argomento come il comando effettivo. Due forme correlate non vengono rimosse: la forma di query `command -v`, che cerca un comando piuttosto che eseguirlo, e `nocorrect` di zsh.

Claude Code rimuove anche un'assegnazione iniziale di determinate variabili di ambiente note come sicure, quindi `Bash(npm test *)` corrisponde a `NODE_ENV=test npm test`. Una regola allow non corrisponderà oltre un'assegnazione di qualsiasi altra variabile. Una regola deny o ask corrisponde oltre qualsiasi assegnazione iniziale, quindi `Bash(rm *)` in deny corrisponde comunque a `FOO=bar rm -rf tmp/`.

Anche `xargs` nudo viene rimosso, quindi `Bash(grep *)` corrisponde a `xargs grep pattern`. La rimozione si applica solo quando `xargs` non ha flag: un'invocazione come `xargs -n1 grep pattern` viene abbinata come comando `xargs`, quindi le regole scritte per il comando interno non la coprono.

Questo elenco di wrapper è integrato e non è configurabile. I runner dell'ambiente di sviluppo come `direnv exec`, `devbox run`, `mise exec`, `npx` e `docker exec` non sono nell'elenco. Poiché questi strumenti eseguono i loro argomenti come comando, una regola come `Bash(devbox run *)` corrisponde a qualsiasi cosa venga dopo `run`, incluso `devbox run rm -rf .`. Per approvare il lavoro all'interno di un runner dell'ambiente, scrivete una regola specifica che includa sia il runner che il comando interno, come `Bash(devbox run npm test)`. Aggiungete una regola per ogni comando interno che desiderate consentire.

I wrapper exec come `watch`, `setsid`, `ionice` e `flock` non possono essere auto-approvati da una regola di prefisso come `Bash(watch *)`, quindi in modalità Manual richiedono sempre un prompt. Lo stesso vale per `find` con `-exec` o `-delete`: una regola `Bash(find *)` non copre queste forme. Per approvare un'invocazione specifica, scrivete una regola di corrispondenza esatta per la stringa di comando completa.

<h4 id="bash-rule-limits">
  A cosa non corrisponde una regola Bash
</h4>

Una regola Bash corrisponde al testo del comando che Claude scrive, dopo che Claude Code divide i [comandi composti](#compound-commands) e rimuove i [wrapper](#process-wrappers). Non corrisponde allo stesso programma invocato in una forma diversa, quindi una regola deny o ask copre l'invocazione che Claude di solito produce e non è un confine di sicurezza intorno al programma. Queste regole in `deny` o `ask` fermano la prima forma e non le altre:

| Regola             | Ferma                      | Non ferma                                                                                             |
| :----------------- | :------------------------- | :---------------------------------------------------------------------------------------------------- |
| `Bash(curl *)`     | `curl https://example.com` | `/usr/bin/curl https://example.com`, `sh -c 'curl https://example.com'`                               |
| `Bash(rm *)`       | `rm -rf build/`            | `/bin/rm -rf build/`, `bash -c 'rm -rf build/'`                                                       |
| `Bash(git push *)` | `git push origin main`     | `git -C . push origin main`, `git -c push.default=current push origin main`, `git 'push' origin main` |

Le vostre altre regole e la modalità di autorizzazione decidono i comandi nell'ultima colonna.

Per l'applicazione del filesystem e della rete che non dipende dal testo del comando, utilizzate il [sandboxing](/docs/it/sandboxing). Per ispezionare il testo del comando completo con la vostra logica prima che venga eseguito, utilizzate un hook [PreToolUse](#extend-permissions-with-hooks).

<h4 id="read-only-commands">
  Comandi di sola lettura
</h4>

Claude Code riconosce un insieme integrato di comandi Bash come di sola lettura e li esegue senza un prompt di autorizzazione in ogni modalità, tranne per un percorso che [`permissions.blockReadsOutsideWorkingDirectories`](/docs/it/settings-reference#permissions-blockreadsoutsideworkingdirectories) protegge. L'insieme include `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` e forme di sola lettura di `git`. L'insieme non è configurabile; per richiedere un prompt per uno di questi comandi, aggiungete una regola `ask` o `deny` per esso. In modalità auto, questi comandi possono anche attendere la revisione del classificatore; vedere [come il classificatore valuta le azioni](/docs/it/permission-modes#how-the-classifier-evaluates-actions).

Un reindirizzamento come `ls > out.txt` aggiunge un controllo sul target. Vedere [Redirections](#redirections).

I modelli glob non quotati sono consentiti per i comandi il cui ogni flag è di sola lettura, quindi `ls *.ts` e `wc -l src/*.py` vengono eseguiti senza un prompt.

In modalità Manual, i comandi da questo insieme richiedono comunque un prompt in questi casi:

* **Glob non quotati per comandi con flag in grado di scrivere**: i comandi con flag in grado di scrivere o eseguire, come `find`, `sort`, `sed` e `git`, richiedono un prompt quando è presente un glob non quotato, perché il glob potrebbe espandersi a un flag come `-delete`.
* **`docker` puntato a un altro daemon**: le forme di sola lettura di `docker` richiedono un prompt quando il comando porta un flag che seleziona un daemon diverso, come `-H`, `--context` o `--url` e `--connection` di Podman.
* **`file` con flag di apertura percorso**: `file` richiede un prompt quando passa `-m`/`--magic-file` o `-f`/`--files-from`, perché questi flag fanno sì che `file` apra i percorsi denominati nel valore del flag.
* **Percorsi di rete su Windows**: un comando i cui argomenti includono un percorso di rete (UNC), come `\\server\share\file`, richiede un prompt perché l'accesso a un percorso di rete può inviare le vostre credenziali Windows all'host che nomina. Lo stesso controllo si applica ai comandi dello [strumento PowerShell](/docs/it/tools-reference#powershell-tool).
* **Comandi che l'analisi non riesce a analizzare**: quando Claude Code non riesce ad analizzare completamente un comando, chiede l'approvazione invece di trattare il comando come di sola lettura. I comandi più lunghi di 10.000 caratteri richiedono sempre un prompt perché superano quello che l'analisi analizza.

Un `cd` in un percorso all'interno della vostra directory di lavoro o di una [directory aggiuntiva](#working-directories) è anche di sola lettura, e un comando composto come `cd packages/api && ls` viene eseguito senza un prompt quando ogni parte si qualifica da sola. Queste combinazioni richiedono un prompt anche quando ogni parte è di sola lettura:

* **`cd` con `git`**: richiede un prompt quando il `cd` cambia in una directory diversa, poiché l'esecuzione di `git` in una nuova directory può eseguire gli hook di quella directory. Un `cd` il cui target si risolve alla directory di lavoro corrente è un'operazione nulla e non attiva il prompt.
* **`cd` con un reindirizzamento**: richiede un prompt quando Claude Code non riesce a determinare in quale directory il target di reindirizzamento si risolve dopo l'esecuzione di `cd`. Un comando il cui unico target di reindirizzamento è `/dev/null`, come `cd app; grep -r pattern . 2>/dev/null`, non richiede un prompt, perché `/dev/null` non dipende dalla directory di lavoro.

<Warning>
  I modelli di autorizzazione Bash che tentano di vincolare gli argomenti del comando sono fragili. Ad esempio, `Bash(curl http://github.com/ *)` intende limitare curl agli URL di GitHub, ma non corrisponderà a variazioni come:

  * Opzioni prima dell'URL: `curl -X GET http://github.com/...`
  * Protocollo diverso: `curl https://github.com/...`
  * Reindirizzamenti: `curl -L http://short.example.com/xyz`, che reindirizza a GitHub
  * Variabili: `URL=http://github.com && curl $URL`

  Per un filtraggio URL più affidabile, considerate:

  * **Limitare gli strumenti di rete Bash**: utilizzate regole deny per bloccare `curl`, `wget` e comandi simili, quindi utilizzate lo strumento WebFetch con l'autorizzazione `WebFetch(domain:github.com)` per i domini consentiti. Una regola deny non corrisponde allo stesso programma per percorso o all'interno di `sh -c`, quindi abbinate con l'[elenco di consentiti della rete sandbox](/docs/it/sandboxing#network-isolation) quando la restrizione deve valere; vedere [a cosa non corrisponde una regola Bash](#bash-rule-limits)
  * **Utilizzare hook PreToolUse**: implementate un hook che convalida gli URL nei comandi Bash e blocca i domini non consentiti
  * **Aggiungere guida CLAUDE.md**: descrivete i vostri modelli curl consentiti in `CLAUDE.md`. Questo modella quello che Claude prova ma non applica un confine, quindi abbinate con una delle opzioni sopra

  Nota che l'utilizzo di WebFetch da solo non impedisce l'accesso alla rete. Se Bash è consentito, Claude può comunque utilizzare `curl`, `wget` o altri strumenti per raggiungere qualsiasi URL.
</Warning>

<h4 id="redirections">
  Reindirizzamenti
</h4>

Quando un comando reindirizza l'output o l'input, Claude Code controlla il target di reindirizzamento rispetto alle vostre regole di file come se Claude avesse scritto o letto quel file direttamente:

* **Reindirizzamenti di output**: per `> file`, `>> file` o `2> file`, il controllo copre le vostre regole allow e deny `Edit`, i [percorsi protetti](/docs/it/permission-modes#protected-paths) e le [directory di lavoro](#working-directories). Una regola come `Bash(git commit *)` consente il comando, non il target. Un target che inizia con `~` o contiene un carattere glob richiede la vostra approvazione.
* **Reindirizzamenti di input**: per `< file`, il controllo copre le vostre regole allow e deny `Read` e le directory di lavoro. Un target al di fuori delle directory di lavoro richiede la vostra approvazione a meno che una regola allow non lo copra. Un target che contiene un modello glob, o un percorso relativo che segue un `cd` nello stesso comando, richiede la vostra approvazione anche quando una regola allow lo copre. Claude Code controlla i target di input in v2.1.257 e successivo.

I target senza file dietro di loro non vengono controllati: `/dev/null`, forme di descrittore di file come `2>&1` e `<&3`, e here-docs e here-strings.

Claude Code controlla anche i file che un comando `tee` scrive, incluso in una pipeline come `make | tee build.log`. Il controllo copre le vostre regole allow e deny `Edit`, i [percorsi protetti](/docs/it/permission-modes#protected-paths) e le [directory di lavoro](#working-directories). Una regola allow come `Bash(tee *)` non copre una destinazione al di fuori delle directory di lavoro. Claude Code controlla i target `tee` in v2.1.269 e successivo.

<h3 id="powershell">
  PowerShell
</h3>

Le regole di autorizzazione PowerShell utilizzano la stessa forma delle regole Bash. I caratteri jolly con `*` corrispondono in qualsiasi posizione, il suffisso `:*` è equivalente a un ` *` finale, e un PowerShell nudo o `PowerShell(*)` corrisponde a ogni comando. Questa configurazione consente comandi `Get-ChildItem` e `git commit` mentre blocca `Remove-Item`:

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

Gli alias comuni vengono canonicalizati prima della corrispondenza. Una regola scritta per il nome del cmdlet corrisponde anche ai suoi alias, quindi `PowerShell(Get-ChildItem *)` corrisponde a `gci`, `ls` e `dir` nonché. La corrispondenza non è sensibile alle maiuscole.

Claude Code analizza l'AST di PowerShell e controlla ogni comando in un comando composto indipendentemente. Gli operatori pipeline `|`, i separatori di istruzione `;` e su PowerShell 7+ gli operatori di catena `&&` e `||` dividono un comando composto in sottocomandi. Una regola deve corrispondere a ogni sottocomando affinché il comando composto sia consentito.

<h3 id="read-and-edit">
  Read e Edit
</h3>

Per bloccare gli strumenti di file di Claude dalla lettura di un file o directory, aggiungete una regola deny `Read` per il suo percorso, come `Read(./.env)` o `Read(./secrets/**)`; [Exclude sensitive files](/docs/it/settings-reference#exclude-sensitive-files) ha un esempio pronto da incollare.

Le regole `Edit` si applicano a tutti gli strumenti integrati che modificano i file. Claude fa un tentativo migliore per applicare le regole `Read` a tutti gli strumenti integrati che leggono file come Grep e Glob, a menzioni `@file` nei vostri prompt e alla selezione e al contesto di file aperti che un [IDE](/docs/it/vs-code#the-built-in-ide-mcp-server) connesso condivide con Claude.

Una regola deny `Read` blocca anche gli [strumenti Edit e Write](/docs/it/errors#file-is-covered-by-a-read-deny-rule) sullo stesso percorso, inclusa la creazione di un nuovo file lì. NotebookEdit non è coperto, quindi aggiungete una regola deny `Edit` per i percorsi che nessuno strumento può modificare. Il controllo richiede Claude Code v2.1.208 o successivo sulle modifiche, e v2.1.228 o successivo sulle scritture.

Claude Code controlla le autorizzazioni di file rispetto alle regole `Edit(path)` e `Read(path)` solo. Se scrivete una regola di percorso per `Write`, `NotebookEdit`, `Glob` o lo strumento legacy `MultiEdit` invece, Claude Code accetta la regola ma non la consulta mai, e [avvisa all'avvio](/docs/it/errors#is-not-matched-by-file-permission-checks), tranne per una regola `Glob` passata in `--allowedTools`. Utilizzate `Edit(docs/**)` al posto di `Write(docs/**)`, `NotebookEdit(docs/**)` o `MultiEdit(docs/**)`, e `Read(docs/**)` al posto di `Glob(docs/**)`. Claude Code non avvisa di una regola di nome strumento senza percorso, come una regola deny per `Write`; corrisponde a quella regola a livello di strumento ovunque. Richiede Claude Code v2.1.210 o successivo.

<Warning>
  Le regole deny di Read e Edit si applicano agli strumenti di file integrati di Claude, ai comandi di file che Claude Code riconosce in Bash, come `cat`, `head`, `tail`, `sed` e `tee`, e ai target dei [reindirizzamenti](#redirections) Bash come `> file` e `< file`. Non si applicano a un comando che legge file senza nominarli, come `grep -r pattern .` eseguito dalla directory che contiene il file, o a sottoprocessi arbitrari che leggono o scrivono file indirettamente, come uno script Python o Node che apre i file da solo. Per l'applicazione a livello del sistema operativo che blocca tutti i processi dall'accesso a un percorso, [abilitate la sandbox](/docs/it/sandboxing).
</Warning>

Le regole Read e Edit seguono entrambe la sintassi del modello [gitignore](https://git-scm.com/docs/gitignore) con quattro tipi di modello distinti; per i modelli di directory a segmento singolo, la profondità di corrispondenza dipende anche dal tipo di regola, descritto più avanti in questa sezione:

| Modello           | Significato                                     | Esempio                          | Corrisponde                                                               |
| ----------------- | ----------------------------------------------- | -------------------------------- | ------------------------------------------------------------------------- |
| `//path`          | Percorso assoluto dalla radice del filesystem   | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                                 |
| `~/path`          | Percorso dalla directory home                   | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                                            |
| `/path`           | Percorso relativo alla fonte delle impostazioni | `Edit(/src/**/*.ts)`             | `<primary working directory>/src/**/*.ts` nelle impostazioni del progetto |
| `path` o `./path` | Percorso relativo alla directory corrente       | `Read(*.env)`                    | `<cwd>/*.env`                                                             |

<Warning>
  Un modello come `/Users/alice/file` non è un percorso assoluto. La singola barra iniziale si ancora alla fonte delle impostazioni, non alla radice del filesystem. Utilizzate `//Users/alice/file` per i percorsi assoluti.
</Warning>

Un modello `/path` si ancora a una directory associata alla fonte delle impostazioni che lo definisce, quindi la stessa regola corrisponde a posizioni diverse a seconda di dove la mettete:

| Regola definita in                                   | `/path` si risolve a               |
| :--------------------------------------------------- | :--------------------------------- |
| Impostazioni di progetto in `.claude/settings.json`  | `<primary working directory>/path` |
| Impostazioni locali in `.claude/settings.local.json` | `<primary working directory>/path` |
| Impostazioni utente in `~/.claude/settings.json`     | `~/.claude/path`                   |
| Un file passato con `--settings <file>`              | `<directory of file>/path`         |
| Flag CLI o regole di sessione                        | `<primary working directory>/path` |

Una regola che aggiungete tramite `/permissions` segue la riga per il file di impostazioni in cui la salvate.

Le regole delle impostazioni locali si ancorano alla [primary working directory](#working-directories) della sessione, non alla radice del repository dove Claude Code [memorizza il file](#permission-system) in v2.1.211 e successivo. In una sessione avviata alla radice del repository, le due directory sono le stesse; in una sessione [worktree](/docs/it/worktrees), una regola condivisa come `Edit(/src/**)` corrisponde alla directory `src/` di quel worktree.

Una regola deny come `Read(/secrets/**)` nelle impostazioni utente blocca `~/.claude/secrets/**`, non una directory `secrets` nel vostro progetto. Per scrivere una regola nelle impostazioni utente che si applica all'interno di ogni progetto, utilizzate un percorso assoluto `//` o un percorso relativo alla home `~/` invece.

Su Windows, i percorsi vengono normalizzati in forma POSIX prima della corrispondenza. `C:\Users\alice` diventa `/c/Users/alice`, quindi utilizzate `//c/**/.env` per corrispondere ai file `.env` in qualsiasi punto su quel drive. Per corrispondere su tutti i drive, utilizzate `//**/.env`.

Esempi:

* `Edit(/docs/**)`: modifica in `<primary working directory>/docs/`, non `/docs/` o `<primary working directory>/.claude/docs/`
* `Read(~/.zshrc)`: legge il `.zshrc` della vostra directory home
* `Edit(//tmp/scratch.txt)`: modifica il percorso assoluto `/tmp/scratch.txt`
* `Read(src/**)`: come regola allow, legge da `<current-directory>/src/` solo; come regola deny o ask, corrisponde a una directory `src` a qualsiasi profondità sotto la directory corrente

Una regola corrisponde solo ai file sotto il suo ancoraggio; all'interno di quel limite, la profondità di corrispondenza dipende dalla forma del modello e, per i modelli di directory a segmento singolo, dal tipo di regola, descritto di seguito. I nomi di file nudi seguono la semantica gitignore e corrispondono a qualsiasi profondità, quindi `Read(.env)` e `Read(**/.env)` sono equivalenti:

| Regola deny                    | Blocca                                              | Non blocca                                             |
| ------------------------------ | --------------------------------------------------- | ------------------------------------------------------ |
| `Read(.env)` o `Read(**/.env)` | qualsiasi `.env` alla o sotto la directory corrente | `.env` in una directory padre o in un altro progetto   |
| `Read(//**/.env)`              | qualsiasi `.env` in qualsiasi punto del filesystem  | nulla; la regola è ancorata alla radice del filesystem |

Un modello relativo con un singolo segmento di directory, come `src/**`, corrisponde a profondità diverse a seconda del tipo di regola:

* **Regole allow**: `Edit(src/**)` corrisponde solo a `<cwd>/src` e ai file sotto di esso. Per consentire un nome di directory a qualsiasi profondità, scrivete `Edit(**/src/**)`.
* **Regole deny e ask**: `Read(secrets/**)` corrisponde a una directory denominata `secrets` a qualsiasi profondità sotto la directory corrente, quindi la regola si applica anche alle copie annidate.

Ogni altra forma di modello corrisponde alla stessa profondità in ogni tipo di regola: `Edit(/src/**)` e `Edit(src/components/**)` corrispondono solo alla loro posizione ancorata, mentre `Edit(**/src/**)` corrisponde a qualsiasi profondità.

L'esempio seguente mostra ogni forma di modello rispetto a un progetto con una directory `src/` di livello superiore e una copia annidata sotto `vendor/`:

```text theme={null}
<current-directory>/
├── src/
│   └── app.ts
└── vendor/
    └── pkg/
        └── src/
            └── lib.js
```

| Regola                                        | Corrisponde a `src/app.ts` | Corrisponde a `vendor/pkg/src/lib.js` |
| :-------------------------------------------- | :------------------------- | :------------------------------------ |
| `Edit(src/**)` come regola allow              | Sì                         | No                                    |
| `Edit(src/**)` come regola deny o ask         | Sì                         | Sì                                    |
| `Edit(/src/**)` in qualsiasi tipo di regola   | Sì                         | No                                    |
| `Edit(**/src/**)` in qualsiasi tipo di regola | Sì                         | Sì                                    |

<Note>
  Nei modelli gitignore, `*` corrisponde all'interno di un singolo segmento di percorso e può apparire in qualsiasi posizione nel modello, mentre `**` corrisponde tra le directory.
</Note>

Quando approvate un percorso di file con "Sì, non chiedere più", Claude Code sfugge ai caratteri di modello gitignore in quel percorso, come `[`, `]` e `*`, quindi la regola generata corrisponde solo al percorso letterale che avete approvato. Le regole che scrivete voi stessi non vengono sfuggite. Prima della v2.1.202, Claude Code salvava il percorso non sfuggito, quindi una regola generata per una directory denominata `[2024-06] Reports` potrebbe non corrispondere al suo stesso percorso o corrispondere a directory fratelli non intenzionali.

Non dovete sfuggire le parentesi in un percorso, quindi `Edit(./Finance (2024)/**)` corrisponde alla cartella `Finance (2024)` come scritta.

Una regola deny o ask il cui percorso non è utilizzabile come modello gitignore protegge comunque quel percorso esatto. Una regola allow con un modello non utilizzabile non approva nulla.

Una regola deny o ask che inizia con `!` è una negazione gitignore. Essa scava i percorsi che corrisponde fuori dalle regole `path` o `./path` elencate prima di essa. In un elenco `deny` di un file di impostazioni, `Read(*.env)` seguito da `Read(!sample.env)` blocca ogni file il cui nome termina in `.env` a qualsiasi profondità, tranne i file denominati `sample.env`. Una regola `!` elencata per prima non scava nulla.

Lo scavo raggiunge solo le regole dalla stessa fonte. Un `Read(!.env)` nelle impostazioni del progetto o in `--disallowedTools` non annulla un `Read(./.env)` deny dalle impostazioni gestite o da qualsiasi altro file di impostazioni.

Due limiti restringono quello che un modello `!` può scavare:

* Claude Code legge un modello `!` relativo alla directory corrente anche quando `/`, `~/` o `//` segue il `!`, quindi il modello non può raggiungere una regola ancorata con uno di questi prefissi. `Read(!~/notes/public/**)` non scava nulla da `Read(~/notes/**)`.
* Uno scavo non può riaprire un file all'interno di una directory che una regola blocca nel complesso. Con `Read(secrets/**)` e `Read(!secrets/public/**)`, Claude Code blocca comunque `secrets/public` insieme al resto di `secrets`.

Quando Claude accede a un symlink, le regole di autorizzazione controllano due percorsi: il symlink stesso e il file a cui si risolve. Le regole allow e deny trattano quella coppia diversamente: le regole allow ricadono nel richiedere un prompt, mentre le regole deny bloccano completamente.

* **Regole allow**: si applicano solo quando sia il percorso del symlink che il suo target corrispondono. Un symlink all'interno di una directory consentita che punta al di fuori di essa richiede comunque un prompt.
* **Regole deny**: si applicano quando il percorso del symlink o il suo target corrisponde. Un symlink che punta a un file negato è esso stesso negato. Ad esempio, con `Read(./project/**)` consentito e `Read(~/.ssh/**)` negato, un symlink in `./project/key` che punta a `~/.ssh/id_rsa` viene bloccato: il target non supera la regola allow e corrisponde alla regola deny.

Su macOS e Linux, una regola deny o ask scritta attraverso una directory con symlink con un modello `//`, `~/` o `/` si applica anche alla posizione reale della directory. Ad esempio, su macOS, dove `/etc` si risolve a `/private/etc`, `Read(//etc/**)` blocca anche `/private/etc/hosts`. Prima della v2.1.268, una regola deny o ask scritta attraverso una directory con symlink non si applicava a un percorso dato dalla sua posizione reale.

Quando uno strumento apre un file approvato, Claude Code [conferma che il percorso si risolve ancora alla posizione che il controllo di autorizzazione ha approvato](/docs/it/errors#refusing-after-a-symlink-changed).

Grep e Glob cercano la directory a cui l'argomento `path` si risolve. Claude Code applica le regole deny `Read` a quella directory.

<h3 id="webfetch">
  WebFetch
</h3>

Le regole WebFetch utilizzano un prefisso `domain:` e corrispondono al nome host dell'URL richiesto. La corrispondenza non è sensibile alle maiuscole, supporta caratteri jolly `*` e rimuove un punto finale da entrambi la regola e il nome host in modo che `example.com.` e `example.com` vengono trattati allo stesso modo.

* `WebFetch(domain:example.com)` corrisponde alle richieste a `example.com`
* `WebFetch(domain:*.example.com)` corrisponde a qualsiasi sottodominio a qualsiasi profondità, come `api.example.com` o `a.b.example.com`, ma non a `example.com` stesso
* `WebFetch(domain:*)` corrisponde a ogni dominio. Non è lo stesso di una regola WebFetch nuda; vedere [Allow or deny every fetch](#allow-or-deny-every-fetch)

In qualsiasi posizione diversa da un `*.` iniziale o da un `*` nudo, il carattere jolly corrisponde solo al testo tra due punti. `WebFetch(domain:example.*)` corrisponde a `example.org`, dove `*` diventa `org`, ma non a `example.evil.com`, dove `*` dovrebbe diventare `evil.com` e attraversare un punto. Questo impedisce a un carattere jolly finale di corrispondere a domini che un attaccante potrebbe registrare.

I caratteri jolly nelle regole `WebFetch` richiedono Claude Code v2.1.172 o successivo per corrispondere ai fetch.

<h4 id="allow-or-deny-every-fetch">
  Consentire o negare ogni fetch
</h4>

Una regola WebFetch nuda è il nome dello strumento senza una parte `domain:`, come `"deny": ["WebFetch"]`. Sia essa che `WebFetch(domain:*)` coprono ogni URL, ma Claude Code le applica diversamente, e solo la forma `domain:` aggiunge anche il suo dominio all'[elenco di domini consentiti o negati](/docs/it/sandboxing#network-isolation) della sandbox. Quella sezione elenca le forme di caratteri jolly che la sandbox onora e la versione che ha aggiunto `*` nudo.

Ogni riga mostra cosa fa una regola nell'elenco `allow` e nell'elenco `deny`:

| Regola               | In `allow`                                                                                                        | In `deny`                                                                                                                                                     |
| :------------------- | :---------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `WebFetch`           | Claude esegue il fetch senza chiedervi un prompt. Non cambia quali host i comandi in sandbox possono raggiungere. | Claude Code rimuove lo strumento `WebFetch`, quindi Claude non può eseguire il fetch affatto. Non cambia quali host i comandi in sandbox possono raggiungere. |
| `WebFetch(domain:*)` | Claude esegue il fetch senza chiedervi un prompt, e i comandi in sandbox possono raggiungere qualsiasi host.      | Claude Code mantiene lo strumento e rifiuta ogni fetch, e i comandi in sandbox non possono raggiungere alcun host.                                            |

Le due forme differiscono anche sulle letture degli [artifact](/docs/it/artifacts), le pagine che lo strumento Artifact pubblica su claude.ai. Una regola deny o ask WebFetch nuda non si applica a quelle letture. Una regola `domain:` che copre `claude.ai` o l'host di contenuto `*.claudeusercontent.com`, come `WebFetch(domain:claude.ai)` o `WebFetch(domain:*)`, nega ogni lettura o chiede un prompt prima di essa. Una regola [`Artifact`](/docs/it/artifacts#disable-artifacts) fa lo stesso.

Quando una regola blocca una lettura, la negazione nomina la regola. Prima della v2.1.268, una regola deny WebFetch nuda bloccava ogni lettura di artifact, e una regola ask nuda chiedeva un prompt prima di ognuna.

Per consentire a Claude di eseguire il fetch liberamente mantenendo l'elenco di consentiti della sandbox come è, utilizzate la forma nuda. Questo `settings.json` fa questo:

```json theme={null}
{
  "permissions": {
    "allow": ["WebFetch"]
  }
}
```

Quando chiedete a Claude di eseguire il fetch di una pagina, esegue il fetch senza un prompt. Quando chiedete di eseguire un `curl` [in sandbox](/docs/it/sandboxing) rispetto a un host al di fuori dell'elenco di consentiti della sandbox, Claude Code vi chiede comunque un prompt per quell'host, perché la regola nuda non ha aggiunto l'host all'elenco di consentiti.

In [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), Claude invece nomina l'host nei [domini consentiti per comando](/docs/it/sandboxing#per-command-allowed-domains-in-auto-mode) del comando per il classificatore da rivedere.

<h3 id="mcp">
  MCP
</h3>

Le regole MCP utilizzano il nome del server come configurato in Claude Code, facoltativamente seguito dal nome di uno strumento da quel server.

* `mcp__puppeteer` corrisponde a qualsiasi strumento fornito dal server `puppeteer`
* `mcp__puppeteer__*` utilizza la sintassi con caratteri jolly e corrisponde anche a tutti gli strumenti dal server `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` corrisponde allo strumento `puppeteer_navigate` fornito dal server `puppeteer`

Se la vostra organizzazione ha impostato uno strumento [connettore claude.ai](/docs/it/mcp#organization-controls-on-connector-tools) su `ask` e quella impostazione raggiunge Claude Code nella vostra sessione, le regole allow per quello strumento non hanno effetto: Claude Code chiede un prompt ad ogni chiamata, anche nelle modalità `auto` e `bypassPermissions`. In modalità `dontAsk`, che non chiede mai un prompt, Claude Code nega la chiamata invece. Gli strumenti dai connettori che Claude Code recupera da solo appaiono come `mcp__claude_ai_<server>__<tool>`.

In una sessione [Cowork](https://claude.com/docs/cowork/overview) nell'app Claude Desktop, Claude esegue i comandi shell tramite lo strumento `mcp__workspace__bash` di Cowork piuttosto che lo strumento `Bash` integrato, e Cowork fornisce allo stesso modo `mcp__workspace__web_fetch` per i web fetch. Claude Code applica anche le regole deny che nominano l'intero strumento `Bash` o `WebFetch` a questi strumenti Cowork, quindi una regola deny `Bash` gestita impedisce a Claude di eseguire comandi shell in Cowork. Quando Claude Code blocca tale chiamata, il messaggio nomina lo strumento Cowork: `Permission to use mcp__workspace__bash has been denied.` Le regole allow non si trasferiscono: Claude Code non applica mai una regola `Bash` allow a `mcp__workspace__bash`.

<h3 id="agent-subagents">
  Agent (subagents)
</h3>

Utilizzate le regole `Agent(AgentName)` per controllare quali [subagents](/docs/it/sub-agents) Claude può utilizzare:

* `Agent(Explore)` corrisponde al subagent Explore
* `Agent(Plan)` corrisponde al subagent Plan
* `Agent(my-custom-agent)` corrisponde a un subagent personalizzato denominato `my-custom-agent`

Aggiungete queste regole all'array `deny` nelle vostre impostazioni o utilizzate il flag CLI `--disallowedTools` per disabilitare agenti specifici. Per disabilitare l'agente Explore:

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

Le regole `Cd` controllano in quali directory il comando [`/cd`](/docs/it/commands) può spostare la sessione. `Cd` non è uno strumento invocabile dal modello: Claude non può chiamarlo e le regole si applicano solo quando eseguite `/cd` voi stessi.

Una regola deny `Cd` nuda disabilita `/cd` completamente. Una regola deny `Cd(<path-pattern>)` blocca i target corrispondenti. Le regole deny controllano ogni ortografia del target, incluso ogni hop symlink attraverso cui si risolve, quindi una regola scritta per un percorso blocca anche i target che si risolvono ad esso.

L'aggiunta di qualsiasi regola allow `Cd` passa `/cd` alla modalità allowlist: la directory target risolta deve corrispondere a una delle vostre regole allow, o `/cd` rifiuta. Senza regole `Cd` configurate, `/cd` mantiene il suo comportamento predefinito e vi chiede di fidarvi di una directory sconosciuta.

I modelli di percorso condividono gli ancoraggi `//`, `~/` e `/` dalle [regole Read e Edit](#read-and-edit), ma la corrispondenza è ancorata al percorso della directory intera piuttosto che nello stile gitignore. `*` corrisponde esattamente a un segmento di percorso e `**` corrisponde tra segmenti. Un `/**` finale corrisponde anche alla sua radice denominata.

| Regola                | Corrisponde                                                                           | Non corrisponde                   |
| --------------------- | ------------------------------------------------------------------------------------- | --------------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                                                          | `~/code/app/src`, `~/code`        |
| `Cd(~/code/**)`       | `~/code` e qualsiasi directory sotto di esso                                          | directory al di fuori di `~/code` |
| `Cd(**/node_modules)` | qualsiasi directory `node_modules` a qualsiasi profondità sotto la directory corrente | `node_modules/pkg`                |

<h2 id="extend-permissions-with-hooks">
  Estendere le autorizzazioni con hook
</h2>

Gli [hook di Claude Code](/docs/it/hooks-guide) consentono di registrare comandi shell personalizzati che valutano le autorizzazioni in fase di esecuzione. Quando Claude Code effettua una chiamata di strumento, gli hook PreToolUse vengono eseguiti prima del prompt di autorizzazione, per ogni strumento tranne [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior). L'output dell'hook può negare la chiamata dello strumento, forzare un prompt o saltare il prompt per consentire alla chiamata di procedere.

Le decisioni dell'hook non bypassano le regole di autorizzazione. Claude Code valuta le regole deny e ask indipendentemente da ciò che un hook PreToolUse restituisce: una regola deny corrispondente blocca la chiamata e una regola ask corrispondente richiede comunque un prompt anche quando l'hook ha restituito `"allow"` o `"ask"`. Questo preserva la precedenza deny-first descritta in [Gestire le autorizzazioni](#manage-permissions), incluse le regole deny impostate nelle impostazioni gestite.

Gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) richiedono comunque un prompt quando un hook restituisce `"allow"`, così come gli strumenti connector [che la vostra organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) nelle sessioni in cui questa impostazione raggiunge Claude Code.

Un hook di blocco ha anche la precedenza sulle regole allow. Un hook che esce con codice 2 interrompe la chiamata dello strumento prima che le regole di autorizzazione vengano valutate, quindi il blocco si applica anche quando una regola allow consentirebbe altrimenti la chiamata. Per eseguire tutti i comandi Bash senza prompt tranne alcuni che desiderate bloccare, aggiungete `"Bash"` al vostro elenco allow e registrate un hook PreToolUse che rifiuta quei comandi specifici. Vedi [Bloccare le modifiche ai file protetti](/docs/it/hooks-guide#block-edits-to-protected-files) per uno script di hook che potete adattare.

<h2 id="working-directories">
  Directory di lavoro
</h2>

Per impostazione predefinita, Claude ha accesso ai file nella directory in cui è stato avviato. Quella directory è la directory di lavoro primaria della sessione fino a quando non [spostate la sessione con `/cd`](#move-the-session-to-another-directory). Potete estendere questo accesso:

* **Durante l'avvio**: utilizzate l'argomento CLI `--add-dir <path>`
* **Durante la sessione**: utilizzate il comando `/add-dir`
* **Configurazione persistente**: aggiungete a `additionalDirectories` nei [file di impostazioni](/docs/it/settings#where-settings-live)

I file nelle directory aggiuntive seguono le stesse regole di autorizzazione della directory di lavoro originale: diventano leggibili senza prompt e le autorizzazioni di modifica dei file seguono la modalità di autorizzazione corrente.

Non potete aggiungere la maggior parte dei [percorsi di rete](/docs/it/errors#working-directory-is-a-network-path), come la condivisione UNC `\\server\share`, come directory di lavoro, perché cercarli può contattare l'host che nominano. Su Windows, mappate la condivisione a una lettera di unità e passate l'unità con `--add-dir` all'avvio.

Impostate [`permissions.blockReadsOutsideWorkingDirectories`](/docs/it/settings-reference#permissions-blockreadsoutsideworkingdirectories) per fare in modo che gli strumenti di file rifiutino i percorsi che racchiude in ogni modalità di autorizzazione. In modalità auto, Claude Code offre di attivarlo la prima volta che Claude [legge al di fuori delle directory di lavoro](/docs/it/permission-modes#first-read-outside-the-working-directories).

Nelle sessioni in background su macOS, l'host della sessione richiede l'accesso a cartelle protette come `~/Desktop`, `~/Documents` e `~/Downloads` separatamente dal vostro terminale quando Claude ha bisogno di leggere o scrivere file lì; se le letture lì falliscono con `Operation not permitted`, consultate [come concedere l'accesso alle cartelle alle sessioni in background](/docs/it/agent-view#background-sessions-can%E2%80%99t-read-desktop-documents-or-downloads-on-macos).

<h3 id="move-the-session-to-another-directory">
  Spostare la sessione in un'altra directory
</h3>

Per spostare la sessione in una directory di lavoro primaria diversa, piuttosto che [aggiungere una directory](#working-directories) insieme a quella attuale, eseguite `/cd <path>`. Claude Code mantiene la conversazione, carica il `CLAUDE.md` della nuova directory e vi chiede di [fidarvi dell'area di lavoro](#project-allow-rules-and-workspace-trust) se non avete mai lavorato lì prima. Successivamente, Claude Code [trova la sessione spostata](/docs/it/sessions#resume-a-session) quando eseguite `--resume` dalla nuova directory.

Non appena vi spostate, Claude Code applica la configurazione del progetto della nuova directory:

* Le sue impostazioni di progetto, incluse le loro regole di autorizzazione e [hooks](/docs/it/hooks)
* I suoi server [`.mcp.json`](/docs/it/mcp#project-scope), soggetti alla stessa [approvazione del server](/docs/it/mcp#project-server-approvals-and-workspace-trust) come all'avvio, e i server MCP [local-scope](/docs/it/mcp#local-scope) che avete registrato in esso
* I [plugin](/docs/it/plugins/overview) che le sue impostazioni abilitano, le sue [skills](/docs/it/skills#discovery-from-parent-and-nested-directories) e i suoi [subagents](/docs/it/sub-agents)
* I suoi valori [`env`](/docs/it/settings-reference#env), applicati sopra le variabili di ambiente dalle impostazioni della directory precedente, che rimangono in vigore

Claude Code disconnette anche i server MCP [local-scope](/docs/it/mcp#local-scope) e del progetto della directory precedente, e i server dei [plugin](/docs/it/mcp#plugin-provided-mcp-servers) che non sono più abilitati dopo lo spostamento. Prende le [directory aggiuntive](#working-directories) dalle impostazioni della nuova directory invece da quella precedente, e mantiene le directory che avete aggiunto con `--add-dir` o `/add-dir`. Gli hooks che lo spostamento attiva ricevono ancora [`${CLAUDE_PROJECT_DIR}`](/docs/it/hooks#reference-scripts-by-path) impostato alla radice del progetto dove la sessione è iniziata.

Quando la nuova directory non è ancora attendibile, Claude Code elenca nel prompt di fiducia le regole di autorizzazione, le directory aggiuntive, gli hooks e i comandi helper che le impostazioni della directory attiverebbero, così potete esaminarli prima di accettare. Se rifiutate, la sessione rimane dove si trova. Prima della v2.1.246, `/cd` non applicava le impostazioni, gli hooks, i server MCP o le skills della nuova directory fino a quando non riprendevano la sessione, e il suo prompt di fiducia non elencava cosa le impostazioni della directory attiverebbero.

Limitate o disabilitate i target di `/cd` con le regole di autorizzazione [`Cd`](#cd).

<h3 id="additional-directories-grant-file-access-not-configuration">
  Le directory aggiuntive concedono l'accesso ai file, non la configurazione
</h3>

L'aggiunta di una directory estende dove Claude può leggere e modificare i file. Non rende quella directory una radice di configurazione completa: la maggior parte della configurazione `.claude/` non viene scoperta dalle directory aggiuntive, anche se alcuni tipi vengono caricati come eccezioni.

Queste eccezioni si applicano solo alle directory aggiunte con il flag `--add-dir` o il comando `/add-dir`, incluse le directory che l'Agent SDK aggiunge attraverso il flag. Le directory elencate in `permissions.additionalDirectories` in un file di impostazioni concedono solo l'accesso ai file e non caricano nessuna delle configurazioni di seguito.

L'opzione [`additionalDirectories`](/docs/it/agent-sdk/typescript#options) dell'Agent SDK in TypeScript e l'opzione [`add_dirs`](/docs/it/agent-sdk/python#claudeagentoptions) in Python ricevono anche le eccezioni, anche se l'opzione TypeScript condivide il suo nome con la chiave delle impostazioni. L'SDK passa ogni voce a Claude Code come `--add-dir`, quindi quelle directory si comportano come directory aggiunte tramite flag. Skills, comandi e subagents da qualsiasi directory aggiunta tramite flag vengono caricati attraverso la [fonte di impostazione](/docs/it/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) `project`, quindi non vengono caricati quando escludete quella fonte con [`--setting-sources`](/docs/it/cli-reference) sulla CLI o `settingSources` nell'SDK, e la [modalità bare](/docs/it/headless#start-faster-with-bare-mode) salta i comandi e i subagents tra di essi.

I seguenti tipi di configurazione vengono caricati dalle directory `--add-dir`:

| Configurazione                                                                          | Caricato da `--add-dir`                                                                                                                                                                     |
| :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Skills](/docs/it/skills) in `.claude/skills/`                                               | Sì, con ricaricamento live                                                                                                                                                                  |
| [File di comando](/docs/it/skills#where-skills-live) in `.claude/commands/`                  | Sì, senza ricaricamento live. Quando la directory aggiunta e il vostro progetto definiscono entrambi un comando con lo stesso nome, Claude Code esegue il comando del vostro progetto       |
| [Subagents](/docs/it/sub-agents) in `.claude/agents/`                                        | Sì, senza ricaricamento live                                                                                                                                                                |
| [Impostazioni](/docs/it/settings) in `.claude/settings.json` e `.claude/settings.local.json` | Solo le chiavi `enabledPlugins` e [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces)                                                                                 |
| File [CLAUDE.md](/docs/it/memory), `.claude/rules/` e `CLAUDE.local.md`                      | Solo quando `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` è impostato. `CLAUDE.local.md` richiede inoltre la fonte di impostazione `local`, che è abilitata per impostazione predefinita |

Per caricare le skills, i comandi e i subagents da una sottodirectory della vostra [directory di lavoro primaria](#working-directories) a metà sessione, eseguite `/add-dir` con il percorso di quella sottodirectory. Claude Code li carica per il resto della sessione senza chiedervi o aggiungere una directory di lavoro, perché la sottodirectory è già leggibile. Questo richiede Claude Code v2.1.257 o successivo.

Claude Code scopre gli stili di output dalla directory di lavoro corrente e dai suoi genitori, dalla vostra directory utente in `~/.claude/` e dalle impostazioni gestite. Gli hooks e altre chiavi `.claude/settings.json` vengono caricati dalla cartella `.claude/` della directory di lavoro corrente senza fallback alla directory genitore, insieme al vostro `~/.claude/settings.json` utente e alle impostazioni gestite. `.claude/settings.local.json` viene caricato dalla radice del repository git, anche quando avviate Claude Code in una sottodirectory, tranne nei casi in cui Claude Code [non utilizza la radice del repository](/docs/it/settings#where-claude-code-looks-for-each-file), come su Windows; prima della v2.1.211, anche esso veniva caricato solo dalla directory di lavoro corrente. Le sessioni [Agent SDK](/docs/it/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) lo caricano dalla directory di lavoro in tutte le versioni.

Per condividere quella configurazione tra progetti, utilizzate uno di questi approcci:

* **Configurazione a livello utente**: posizionate i file in `~/.claude/agents/`, `~/.claude/output-styles/` o `~/.claude/settings.json` per renderli disponibili in ogni progetto
* **Plugin**: pacchetto e distribuite la configurazione come [plugin](/docs/it/plugins/overview) che i team possono installare
* **Avviate dalla directory di configurazione**: eseguite Claude Code dalla directory contenente la configurazione `.claude/` che desiderate

<h2 id="how-permissions-interact-with-sandboxing">
  Come le autorizzazioni interagiscono con il sandboxing
</h2>

Le autorizzazioni e il [sandboxing](/docs/it/sandboxing) sono livelli di sicurezza complementari:

* **Autorizzazioni** controllano quali strumenti Claude Code può utilizzare e quali file o domini può accedere. Si applicano a Bash, Read, Edit, WebFetch, MCP e ogni altro strumento, tranne per il fatto che una regola deny o ask non può bloccare [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior) mentre rimane qualsiasi altro strumento.
* **Sandboxing** fornisce l'applicazione a livello del sistema operativo che limita l'accesso al filesystem e alla rete dei comandi Bash, PowerShell e [Monitor](/docs/it/tools-reference#monitor-tool) e dei loro processi figlio.

Utilizzate entrambi per la difesa in profondità, poiché le restrizioni sandbox si applicano comunque anche se un'iniezione di prompt bypassa il processo decisionale di Claude. I percorsi e i domini dalle impostazioni sandbox e dalle regole di autorizzazione sono [uniti nella configurazione sandbox finale](/docs/it/sandboxing#permission-rules).

Quando il sandboxing è abilitato e lasciate `autoAllowBashIfSandboxed` al suo valore predefinito di `true`, i comandi Bash in sandbox vengono eseguiti senza richiedere un prompt anche se le vostre autorizzazioni includono una regola ask bare `Bash`, o la [forma equivalente `Bash(*)` ](#match-all-uses-of-a-tool): il confine della sandbox sostituisce il prompt per l'intero strumento.

In [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode), Claude Code salta questa sostituzione. Senza una regola ask, i [comandi di sola lettura incorporati](#read-only-commands) vengono comunque eseguiti senza richiedere un prompt, e qualsiasi altro comando shell passa attraverso il flusso di autorizzazione regolare mentre siete ancora in fase di pianificazione; consultate [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) per come Claude Code gestisce i comandi lì. Con una regola ask bare `Bash`, ogni comando Bash richiede un prompt, inclusi i comandi di sola lettura in sandbox, lo stesso che al di fuori del sandboxing. Prima della v2.1.212, la sostituzione si applicava anche in plan mode.

Questi controlli si applicano ancora:

* Le regole ask con ambito di contenuto come `Bash(git push *)` forzano comunque un prompt
* Le regole deny esplicite si applicano ancora
* I comandi `rm` o `rmdir` che hanno come destinazione un [percorso critico](/docs/it/permission-modes#critical-paths) passano comunque attraverso il flusso di autorizzazione regolare

I comandi che non verranno eseguiti in sandbox, come i comandi esclusi, rispettano la regola ask bare `Bash` come al solito. Consultate [modalità sandbox](/docs/it/sandboxing#sandbox-modes) per modificare questo comportamento.

<span id="managed-only-settings" />

<h2 id="managed-settings">
  Impostazioni gestite
</h2>

Per le organizzazioni che necessitano di un controllo centralizzato, gli amministratori distribuiscono impostazioni gestite che le impostazioni utente e di progetto non possono ignorare, ad eccezione di alcune [chiavi sensibili alla sicurezza](/docs/it/settings#exceptions-to-managed-settings-precedence). [Distribuire impostazioni gestite](/docs/it/managed-settings) copre i meccanismi di consegna, la precedenza all'interno del livello gestito, e le [chiavi che solo le impostazioni gestite possono impostare](/docs/it/managed-settings#managed-only-settings).

Una di queste chiavi, [`allowManagedPermissionRulesOnly`](/docs/it/settings-reference#allowmanagedpermissionrulesonly), rende le impostazioni gestite l'unica fonte di impostazioni per le regole di autorizzazione. La sua voce elenca ogni fonte che Claude Code quindi ignora.

`disableBypassPermissionsMode` è tipicamente posizionato nelle impostazioni gestite per applicare la politica organizzativa, ma funziona da qualsiasi ambito. Un utente può impostarlo nelle proprie impostazioni per bloccarsi dalla modalità bypass.

<h2 id="settings-precedence">
  Precedenza delle impostazioni
</h2>

Le regole di autorizzazione seguono la stessa [precedenza delle impostazioni](/docs/it/settings#settings-precedence) di tutte le altre impostazioni di Claude Code, con le impostazioni gestite al livello più alto: nessun altro livello, inclusi gli argomenti della riga di comando, può ignorare una regola di autorizzazione gestita.

Se uno strumento viene negato a qualsiasi livello, nessun altro livello può consentirlo. Ad esempio, un deny delle impostazioni gestite non può essere ignorato da `--allowedTools` e `--disallowedTools` può aggiungere restrizioni oltre a quelle definite dalle impostazioni gestite.

Lo stesso vale tra gli ambiti delle impostazioni: se le impostazioni utente consentono un'autorizzazione e le impostazioni di progetto la negano, la regola di negazione la blocca. Il contrario è vero anche: un deny a livello utente blocca un allow a livello di progetto, perché le regole di negazione da qualsiasi ambito vengono valutate prima delle regole di consentimento.

Gli host di embedding possono fornire ulteriori criteri gestiti tramite l'opzione SDK `managedSettings`, incluse le regole di consentimento delle autorizzazioni a meno che l'amministratore non imposti i blocchi `allowManaged*Only`; [Deliver policy to Claude Desktop sessions](/docs/it/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) illustra quando la politica dell'embedder si applica.

<h2 id="project-allow-rules-and-workspace-trust">
  Regole di autorizzazione del progetto e trust dell'area di lavoro
</h2>

Le regole `permissions.allow` e le voci `permissions.additionalDirectories` nel file `.claude/settings.json` di un progetto concedono capacità, quindi Claude Code le applica solo dopo che accettate la [finestra di dialogo di trust dell'area di lavoro](/docs/it/security#additional-safeguards) per quella cartella. La finestra di dialogo elenca le regole e le directory che la cartella concederà, in modo che possiate esaminarle prima. Le regole `deny` e `ask` non sono interessate, poiché limitano solo.

Claude Code salva il trust che accettate in base a dove lo avviate:

* In un repository, Claude Code salva il trust sulla radice del repository git, quindi il trust copre l'intero repository a parte qualsiasi repository git annidato al suo interno, come un submodule. In un [worktree](/docs/it/worktrees), utilizza la radice del checkout principale, come fa per le [regole salvate](#permission-system).
* Al di fuori di un repository, Claude Code salva il trust sulla directory da cui lo avete avviato, e il trust copre qualsiasi subdirectory di quella directory a parte un repository git annidato al suo interno, come un clone. Ogni subdirectory coperta conta quindi come una cartella il cui genitore avete fidata.
* Quando avviate dalla vostra directory home, Claude Code mantiene il trust solo per la sessione corrente e non lo scrive su disco; consultate la nota [safeguard aggiuntivi](/docs/it/security#additional-safeguards).

Claude Code mostra la finestra di dialogo di trust solo in sessioni interattive. Un'esecuzione `claude -p` o una sessione SDK non la mostra mai, e fidarsi di una cartella genitore non conta per queste regole, quindi [Cosa viene eseguito prima di fidarsi di una cartella](#what-runs-before-you-trust-a-folder) dice quale contenuto del repository Claude Code utilizza ancora in ognuna di quelle due situazioni.

<h3 id="when-your-local-settings-file-needs-trust">
  Quando il vostro file di impostazioni locali ha bisogno di trust
</h3>

`.claude/settings.local.json` è normalmente il vostro file personale, quindi Claude Code applica le sue regole di autorizzazione e directory aggiuntive senza il passaggio di trust. Quando il file è tracciato in git, o `.claude` è un symlink, Claude Code lo tratta invece come fornito dal repository e tiene in sospeso le sue regole finché non vi fidate della cartella.

Claude Code esegue git per distinguere i due casi, ed esegue git solo dopo che avete fidata la cartella: avete accettato la finestra di dialogo di trust per essa o per una directory genitore il cui trust si estende ad essa, o siete in una sessione `-p` o SDK, che conta come accettata. Fino ad allora, il luogo da cui avete avviato Claude Code decide cosa succede alle regole del file:

* **Nella vostra directory di configurazione personale:** Claude Code applica il `.claude/settings.local.json` di quella cartella subito senza eseguire git. La vostra directory di configurazione personale è la vostra directory home, o una directory il cui subdirectory `.claude` avete impostato come [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars#variables). Se quella directory `CLAUDE_CONFIG_DIR` si trova all'interno di un repository git e Claude Code [mantiene le vostre impostazioni locali alla radice del repository](/docs/it/settings#where-claude-code-looks-for-each-file) invece, tiene in sospeso le regole come in qualsiasi altra posizione.
* **In qualsiasi altra posizione:** Claude Code tiene in sospeso le regole del file come fa per le impostazioni del progetto. Una volta che il controllo è stato eseguito, Claude Code applica le regole di un file non tracciato, o di un file in una directory al di fuori di qualsiasi repository git, anche se non avete fidata quella cartella esatta.

<Note>
  L'eccezione della directory di configurazione salta solo il passaggio di trust. `~/.claude/settings.local.json` è ancora [ambito locale](/docs/it/settings#compare-the-scope-of-each-settings-file), quindi Claude Code lo legge solo in sessioni che avviate nella vostra directory home stessa, non in ogni progetto. Per applicare regole di autorizzazione su tutti i vostri progetti, aggiungetele alle vostre impostazioni utente invece: `~/.claude/settings.json`, o `$CLAUDE_CONFIG_DIR/settings.json` quando `CLAUDE_CONFIG_DIR` è impostato.
</Note>

Nelle versioni 2.1.196 fino a 2.1.199, Claude Code teneva in sospeso le regole del file anche nella vostra directory di configurazione personale e al di fuori dei repository git, e stampava l'avviso [`this workspace has not been trusted`](/docs/it/errors#workspace-has-not-been-trusted) lì. Prima della v2.1.207, Claude Code applicava le regole di un file non tracciato prima che accettaste la finestra di dialogo.

<h3 id="what-runs-before-you-trust-a-folder">
  Cosa viene eseguito prima di fidarsi di una cartella
</h3>

Ogni riga è un tipo di contenuto che un repository può fornire. Le colonne sono le due situazioni in cui non avete fidata la cartella stessa: avete fidata solo una cartella genitore, o avete eseguito `claude -p` o l'SDK lì, che non mostra mai la finestra di dialogo di trust. La colonna della cartella genitore non si applica all'interno di un [repository annidato](#project-allow-rules-and-workspace-trust): in una sessione interattiva Claude Code mostra la finestra di dialogo di trust per essa, e un'esecuzione `claude -p` o SDK lì segue la colonna `claude -p`.

| Cosa fornisce il repository                                                                                                                                                                                                                                                                                                       | Avete fidata solo una cartella genitore                                                                                                                                                                             | `claude -p` o l'SDK, cartella mai fidata                                                                                                                                                                      |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Hooks](/docs/it/hooks) nei file di impostazioni, il blocco [`env`](/docs/it/settings-reference#env) e comandi helper come [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper), e gli [hooks](/docs/it/hooks#hooks-in-skills-and-agents) di una skill del progetto e [`allowed-tools`](/docs/it/skills#pre-approve-tools-for-a-skill)               | Utilizzati                                                                                                                                                                                                          | Utilizzati. Il trust dell'area di lavoro non blocca mai `allowed-tools` di una skill in nessuna sessione                                                                                                      |
| Regole `permissions.allow` e `additionalDirectories` in `.claude/settings.json`                                                                                                                                                                                                                                                   | Non utilizzate fino a quando non accettate la finestra di dialogo di trust, che appare di nuovo elencandole                                                                                                         | Non utilizzate. Claude Code stampa un avviso [`this workspace has not been trusted`](/docs/it/errors#workspace-has-not-been-trusted) su stderr                                                                     |
| Hook nel frontmatter di un [subagent](/docs/it/sub-agents#hooks-in-subagent-frontmatter) del progetto, un plugin [`@skills-dir`](/docs/it/plugins/loading#plugins-shared-through-a-repository) del progetto, e voci [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces) dal repository o da una directory `--add-dir` | Non utilizzate, e nessuna finestra di dialogo è offerta                                                                                                                                                             | Non utilizzate                                                                                                                                                                                                |
| [`mcpServers`](/docs/it/sub-agents#scope-mcp-servers-to-a-subagent) inline nel frontmatter di un subagent dal repository o da una directory `--add-dir`. Prima della v2.1.238, Claude Code caricava questi server in entrambe le situazioni                                                                                            | Non utilizzate, e nessuna finestra di dialogo è offerta                                                                                                                                                             | Non utilizzate                                                                                                                                                                                                |
| Server in `.mcp.json`, inclusi quelli che il repository [approva nelle sue stesse impostazioni](/docs/it/mcp#project-server-approvals-and-workspace-trust)                                                                                                                                                                             | Claude Code vi chiede prima di connettersi ad essi. Le approvazioni del repository stesso non contano                                                                                                               | Connessi senza chiedere, approvati o no. L'SDK li carica solo quando `settingSources` include impostazioni del progetto. `claude mcp list` nella stessa cartella riporta comunque tale server come in sospeso |
| Un [`headersHelper`](/docs/it/mcp#trust-a-folder-before-its-headershelper-runs) su un server in `.mcp.json`. Prima della v2.1.238, Claude Code eseguiva l'helper in entrambe le situazioni                                                                                                                                             | Non eseguito fino a quando non accettate la finestra di dialogo di trust, che appare di nuovo nominando dove l'helper è dichiarato. Claude Code connette il server con i suoi soli `headers` statici fino ad allora | Non eseguito. Claude Code connette il server con i suoi soli `headers` statici e stampa una riga [`headersHelper not run`](/docs/it/errors#headershelper-not-run) per server su stderr                             |

Per le righe che hanno bisogno di questa cartella esatta fidata, fidate la cartella manualmente: impostate `projects["<path>"].hasTrustDialogAccepted` a `true` in `~/.claude.json`, dove `<path>` è la radice del repository, o la cartella stessa al di fuori di un repository. Claude Code stampa la chiave esatta nella riga del log di debug per un hook subagent saltato o un server MCP inline, nell'avviso su stderr per regole di autorizzazione saltate, e nella riga `headersHelper not run` per un helper saltato.

Prima di eseguire `claude -p` in un repository che non avete scritto, decidete cosa potrebbe eseguire sulla vostra macchina:

* Passate `--setting-sources user`, o impostate `settingSources` dell'SDK senza impostazioni del progetto, in modo che Claude Code non legga né i file di impostazioni del progetto né il suo `.mcp.json`
* Iniziate con [`--bare`](/docs/it/headless#start-faster-with-bare-mode) in modo che Claude Code non legga hook, skill, comandi personalizzati, subagent, plugin, o server `.mcp.json` dal progetto. Il blocco `env` del progetto e helper come `awsAuthRefresh` nei suoi file di impostazioni si applicano comunque, e Claude Code legge `apiKeyHelper` solo da `--settings`
* Passate `--settings '{"disableAllHooks": true}'` per [disattivare gli hook](/docs/it/hooks#disable-or-remove-hooks) per quella esecuzione. Impostarlo solo nelle vostre impostazioni utente non è sufficiente, perché le impostazioni del progetto del repository hanno precedenza sulle vostre e possono impostarlo di nuovo a `false`
* Aggiungete una voce [`disabledMcpjsonServers`](/docs/it/settings-reference#disabledmcpjsonservers) per rifiutare un server `.mcp.json` per nome in ogni tipo di sessione

<h2 id="example-configurations">
  Configurazioni di esempio
</h2>

Questo [repository](https://github.com/anthropics/claude-code/tree/main/examples/settings) include configurazioni di impostazioni iniziali per scenari di distribuzione comuni. Utilizzatele come punti di partenza e adattatele alle vostre esigenze.

<h2 id="see-also">
  Vedi anche
</h2>

* [Tutte le impostazioni](/docs/it/settings-reference#permission-settings): ogni chiave di impostazione, incluse le chiavi di autorizzazione
* [Configurare la modalità auto](/docs/it/auto-mode-config): comunicate al classificatore della modalità auto quale infrastruttura la vostra organizzazione ritiene affidabile
* [Sandboxing](/docs/it/sandboxing): isolamento del filesystem e della rete a livello del sistema operativo per i comandi Bash
* [Autenticazione](/docs/it/authentication): configurate l'accesso utente a Claude Code
* [Sicurezza](/docs/it/security): salvaguardie di sicurezza e best practice
* [Hooks](/docs/it/hooks-guide): automatizzate i flussi di lavoro ed estendete la valutazione delle autorizzazioni
