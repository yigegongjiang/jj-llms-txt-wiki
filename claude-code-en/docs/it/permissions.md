> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare le autorizzazioni

> Controlla cosa Claude Code può accedere e fare con regole di autorizzazione granulari, modalità e criteri gestiti.

Claude Code supporta autorizzazioni granulari in modo che Lei possa specificare esattamente cosa l'agente è autorizzato a fare e cosa non può fare. Le impostazioni di autorizzazione possono essere archiviate nel controllo della versione e distribuite a tutti gli sviluppatori della Sua organizzazione, nonché personalizzate dai singoli sviluppatori.

<h2 id="permission-system">
  Sistema di autorizzazione
</h2>

Claude Code utilizza un sistema di autorizzazione a livelli per bilanciare potenza e sicurezza:

| Tipo di strumento | Esempio               | Approvazione richiesta                                                                   | Comportamento "Sì, non chiedere più"                |
| :---------------- | :-------------------- | :--------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| Sola lettura      | Letture di file, Grep | No, all'interno della [directory di lavoro e directory aggiuntive](#working-directories) | N/A                                                 |
| Comandi Bash      | Esecuzione shell      | Sì, eccetto un insieme integrato di [comandi di sola lettura](#read-only-commands)       | Permanentemente per directory di progetto e comando |
| Modifica di file  | Edit/Write di file    | Sì                                                                                       | Fino alla fine della sessione                       |

Su un prompt di autorizzazione Bash o PowerShell, premere `Ctrl+E` per mostrare una spiegazione del comando: cosa fa, perché Claude lo sta eseguendo e cosa potrebbe andare male, etichettato come **Rischio basso**, **Rischio medio** o **Rischio alto**. Claude Code invia il comando e la descrizione del Claude stesso della chiamata al modello per generare la spiegazione solo quando si preme `Ctrl+E`, non su ogni prompt. Mostrare la spiegazione non esegue il comando; premere `Ctrl+E` di nuovo per nasconderla.

Per disattivare la scorciatoia, impostare [`permissionExplainerEnabled`](/docs/it/settings#global-config-settings) su `false` in `~/.claude.json`.

<h2 id="manage-permissions">
  Gestire le autorizzazioni
</h2>

Potete visualizzare e gestire le autorizzazioni degli strumenti di Claude Code con `/permissions`. Questa interfaccia utente elenca tutte le regole di autorizzazione e il file `settings.json` da cui provengono.

* Le regole **Allow** consentono a Claude Code di utilizzare lo strumento specificato senza approvazione manuale.
* Le regole **Ask** richiedono una conferma ogni volta che Claude Code tenta di utilizzare lo strumento specificato.
* Le regole **Deny** impediscono a Claude Code di utilizzare lo strumento specificato.

Le regole vengono valutate in ordine: deny, quindi ask, quindi allow. La prima corrispondenza in quell'ordine determina il risultato, e la specificità della regola non cambia l'ordine.

Una regola deny ampia come `Bash(aws *)` blocca ogni chiamata corrispondente, incluse le chiamate che corrispondono anche a una regola allow più ristretta come `Bash(aws s3 ls)`, quindi una regola deny non può contenere eccezioni di allowlist. La stessa precedenza si applica tra ask e allow: una regola ask corrispondente richiede una conferma anche quando una regola allow più specifica corrisponde anche alla stessa chiamata.

Le regole deny si comportano diversamente a seconda che denominino uno strumento o che limitino un modello all'interno di uno. Un nome di strumento semplice come `Bash` rimuove lo strumento dal contesto di Claude interamente, quindi Claude non lo vede mai. Una regola limitata come `Bash(rm *)` lascia lo strumento disponibile e blocca le chiamate corrispondenti quando Claude tenta di utilizzarle.

<Note>
  Le regole di autorizzazione sono applicate da Claude Code, non dal modello. Le istruzioni nel vostro prompt o in `CLAUDE.md` determinano ciò che Claude tenta di fare, ma non cambiano ciò che Claude Code consente. Per concedere o revocare l'accesso, utilizzate `/permissions`, le regole descritte qui, una [modalità di autorizzazione](/docs/it/permission-modes), o un [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

<h2 id="permission-modes">
  Modalità di autorizzazione
</h2>

Claude Code supporta diverse modalità di autorizzazione che controllano come approva le chiamate di strumenti. Vedi [Permission modes](/docs/it/permission-modes) per quando utilizzare ciascuna. Imposta `defaultMode` nei tuoi [file di impostazioni](/docs/it/settings#settings-files):

| Modalità            | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Comportamento standard: richiede l'autorizzazione al primo utilizzo di ogni strumento. Etichettato come Manual nella CLI, nelle estensioni VS Code e JetBrains, e nell'app desktop, e Claude Code accetta `manual` come alias. L'etichetta e l'alias richiedono Claude Code v2.1.200 o successivo. L'etichetta dell'app desktop non dipende dalla tua versione CLI                                                                                    |
| `acceptEdits`       | Accetta automaticamente le modifiche ai file e i comandi comuni del filesystem come `mkdir`, `touch`, `mv` e `cp` per i percorsi nella directory di lavoro o `additionalDirectories`                                                                                                                                                                                                                                                                  |
| `plan`              | Claude legge i file ed esegue comandi shell di sola lettura per esplorare ma non modifica i tuoi file sorgente. Etichettato Plan nella CLI e nell'estensione VS Code                                                                                                                                                                                                                                                                                  |
| `auto`              | Auto-approva le chiamate di strumento con controlli di sicurezza in background che verificano che le azioni si allineino con la tua richiesta                                                                                                                                                                                                                                                                                                         |
| `dontAsk`           | Nega automaticamente gli strumenti a meno che non siano pre-approvati tramite `/permissions` o regole `permissions.allow`. `AskUserQuestion`, strumenti connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) vengono negati anche se li hai consentiti                             |
| `bypassPermissions` | Salta i prompt di autorizzazione, ad eccezione di quelli forzati da regole `ask` esplicite, strumenti connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool). Le rimozioni della directory root e home come `rm -rf /` richiedono comunque un prompt come interruttore di protezione |

<Warning>
  La modalità `bypassPermissions` salta i prompt di autorizzazione, incluse le scritture in `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` e `.mvn`. Utilizza questa modalità solo in ambienti isolati come contenitori o macchine virtuali dove Claude Code non può causare danni.

  Alcuni prompt si attivano comunque in questa modalità. Le regole `ask` esplicite, gli strumenti connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) richiedono comunque un prompt. Le rimozioni che hanno come destinazione la radice del filesystem o la directory home, come `rm -rf /` e `rm -rf ~`, richiedono comunque un prompt come interruttore di protezione contro gli errori del modello, incluso quando il comando contiene sostituzione di comando con `$(...)` o backtick, o sostituzione di processo con `<(...)`. Prima della v2.1.208, solo la forma semplice, come `rm -rf ~` digitato come comando autonomo, richiedeva un prompt; i comandi che raggiungevano la rimozione attraverso una sostituzione non lo facevano.
</Warning>

Per prevenire che la modalità `bypassPermissions` o `auto` venga utilizzata, imposta `permissions.disableBypassPermissionsMode` o `permissions.disableAutoMode` su `"disable"` in qualsiasi [file di impostazioni](/docs/it/settings#settings-files). Questi sono più utili nelle [impostazioni gestite](#managed-settings) dove non possono essere ignorati.

<h2 id="permission-rule-syntax">
  Sintassi delle regole di autorizzazione
</h2>

Le regole di autorizzazione seguono il formato `Tool` o `Tool(specifier)`.

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

Le regole di negazione e richiesta possono corrispondere a un parametro di input di primo livello su qualsiasi strumento con `Tool(param:value)`. La regola corrisponde quando Claude chiama lo strumento con quel parametro impostato su quel valore esatto. Una regola di autorizzazione per un valore di parametro non stabilirerebbe che la chiamata è sicura nel complesso, quindi le regole di autorizzazione continuano a utilizzare la sintassi dello specificatore di ogni strumento. Questo funziona per qualsiasi parametro scalare che lo strumento accetta:

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

I campi che uno strumento già corrisponde con le sue proprie regole di canonicalizzazione non sono corrispondibili in questo modo: `command` per Bash e PowerShell, `file_path` per Read, Edit e Write, `path` per Grep e Glob, `notebook_path` per NotebookEdit, e `url` per WebFetch. Una regola come `Bash(command:rm *)` sarebbe aggirabile da un comando composto, quindi Claude Code la ignora e emette un avviso all'avvio. Utilizzate invece `Bash(rm *)`, `Read(./path)`, o `WebFetch(domain:host)`.

<h3 id="wildcard-patterns">
  Modelli con caratteri jolly
</h3>

Le regole Bash supportano modelli glob con `*`. I caratteri jolly possono apparire in qualsiasi posizione nel comando. Questa configurazione consente comandi npm e git commit mentre blocca git push:

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)",
      "Bash(git * main)",
      "Bash(* --version)",
      "Bash(* --help *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

Lo spazio prima di `*` è importante: `Bash(ls *)` corrisponde a `ls -la` ma non a `lsof`, mentre `Bash(ls*)` corrisponde a entrambi. Il suffisso `:*` è un modo equivalente per scrivere un carattere jolly finale, quindi `Bash(ls:*)` corrisponde agli stessi comandi di `Bash(ls *)`.

La finestra di dialogo di autorizzazione scrive la forma separata da spazi quando selezionate "Sì, non chiedere più" per un prefisso di comando. La forma `:*` è riconosciuta solo alla fine di un modello. In un modello come `Bash(git:* push)`, i due punti vengono trattati come un carattere letterale e non corrisponderanno ai comandi git.

<h3 id="tool-name-wildcards">
  Caratteri jolly nei nomi degli strumenti
</h3>

Le regole di negazione e richiesta accettano anche modelli glob nella posizione del nome dello strumento. Il modello deve corrispondere al nome completo dello strumento: `"*"` corrisponde a ogni strumento, e `"mcp__*"` corrisponde a ogni strumento MCP su tutti i server. Uno strumento corrispondente a una regola di negazione con nome semplice viene rimosso dal contesto di Claude, come un nome di strumento semplice. Questa configurazione nega ogni strumento MCP:

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

Una regola di negazione o richiesta il cui nome dello strumento non corrisponde a nessuno strumento noto produce un avviso all'avvio per catturare gli errori di digitazione. I nomi degli strumenti contenenti `_` o `*` sono esenti dal controllo.

L'etichetta mostrata per uno strumento nella trascrizione e nella finestra di dialogo di autorizzazione può differire dal suo nome canonico. Ad esempio, lo strumento etichettato `Stop Task` nella trascrizione ha il nome canonico `TaskStop`. Le regole di autorizzazione e i [matcher di hook](/docs/it/hooks) corrispondono solo al nome canonico, quindi una regola scritta come `Stop Task` non corrisponde. Per le regole di negazione e richiesta, l'avviso all'avvio di cui sopra cattura la mancata corrispondenza. Utilizzate i nomi canonici elencati nel [riferimento degli strumenti](/docs/it/tools-reference).

<h2 id="tool-specific-permission-rules">
  Regole di autorizzazione specifiche dello strumento
</h2>

<h3 id="bash">
  Bash
</h3>

Le regole di autorizzazione Bash supportano la corrispondenza con caratteri jolly con `*`. I caratteri jolly possono apparire in qualsiasi posizione nel comando, incluso all'inizio, nel mezzo o alla fine:

* `Bash(npm run build)` corrisponde al comando Bash esatto `npm run build`
* `Bash(npm run test *)` corrisponde ai comandi Bash che iniziano con `npm run test`
* `Bash(npm *)` corrisponde a qualsiasi comando che inizia con `npm `
* `Bash(* install)` corrisponde a qualsiasi comando che termina con ` install`
* `Bash(git * main)` corrisponde a comandi come `git checkout main` e `git log --oneline main`

Un singolo `*` corrisponde a qualsiasi sequenza di caratteri inclusi gli spazi, quindi un singolo carattere jolly può estendersi su più argomenti. `Bash(git *)` corrisponde a `git log --oneline --all` e `Bash(git * main)` corrisponde sia a `git push origin main` che a `git merge main`.

Quando `*` appare alla fine con uno spazio prima (come `Bash(ls *)`), applica un confine di parola, richiedendo che il prefisso sia seguito da uno spazio o dalla fine della stringa. Ad esempio, `Bash(ls *)` corrisponde a `ls -la` ma non a `lsof`. Al contrario, `Bash(ls*)` senza spazio corrisponde sia a `ls -la` che a `lsof` perché non c'è un vincolo di confine di parola.

<h4 id="compound-commands">
  Comandi composti
</h4>

<Tip>
  Claude Code è consapevole degli operatori shell, quindi una regola come `Bash(safe-cmd *)` non gli darà il permesso di eseguire il comando `safe-cmd && other-cmd`. I separatori di comando riconosciuti sono `&&`, `||`, `;`, `|`, `|&`, `&` e newline. Una regola deve corrispondere a ogni sottocomando indipendentemente.
</Tip>

Quando approvate un comando composto con "Sì, non chiedere più", Claude Code salva una regola separata per ogni sottocomando che richiede approvazione, piuttosto che una singola regola per la stringa completa. Ad esempio, approvando `git status && npm test` salva una regola per `npm test`, quindi le future invocazioni di `npm test` vengono riconosciute indipendentemente da cosa precede `&&`. I sottocomandi come `cd` in una sottodirectory generano la loro propria regola Read per quel percorso. Fino a 5 regole possono essere salvate per un singolo comando composto.

<h4 id="process-wrappers">
  Wrapper di processo
</h4>

Prima di corrispondere alle regole Bash, Claude Code rimuove un insieme fisso di wrapper di processo in modo che una regola come `Bash(npm test *)` corrisponda anche a `timeout 30 npm test`. I wrapper riconosciuti sono `timeout`, `time`, `nice`, `nohup` e `stdbuf`.

Anche `xargs` nudo viene rimosso, quindi `Bash(grep *)` corrisponde a `xargs grep pattern`. La rimozione si applica solo quando `xargs` non ha flag: un'invocazione come `xargs -n1 grep pattern` viene abbinata come comando `xargs`, quindi le regole scritte per il comando interno non la coprono.

Questo elenco di wrapper è integrato e non è configurabile. I runner dell'ambiente di sviluppo come `direnv exec`, `devbox run`, `mise exec`, `npx` e `docker exec` non sono nell'elenco. Poiché questi strumenti eseguono i loro argomenti come comando, una regola come `Bash(devbox run *)` corrisponde a qualsiasi cosa venga dopo `run`, incluso `devbox run rm -rf .`. Per approvare il lavoro all'interno di un runner dell'ambiente, scrivete una regola specifica che includa sia il runner che il comando interno, come `Bash(devbox run npm test)`. Aggiungete una regola per ogni comando interno che desiderate consentire.

I wrapper exec come `watch`, `setsid`, `ionice` e `flock` richiedono sempre un prompt e non possono essere auto-approvati da una regola di prefisso come `Bash(watch *)`. Lo stesso vale per `find` con `-exec` o `-delete`: una regola `Bash(find *)` non copre queste forme. Per approvare un'invocazione specifica, scrivete una regola di corrispondenza esatta per la stringa di comando completa.

<h4 id="read-only-commands">
  Comandi di sola lettura
</h4>

Claude Code riconosce un insieme integrato di comandi Bash come di sola lettura e li esegue senza un prompt di autorizzazione in ogni modalità. Questi includono `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` e forme di sola lettura di `git`. L'insieme non è configurabile; per richiedere un prompt per uno di questi comandi, aggiungete una regola `ask` o `deny` per esso.

I modelli glob non quotati sono consentiti per i comandi il cui ogni flag è di sola lettura, quindi `ls *.ts` e `wc -l src/*.py` vengono eseguiti senza un prompt. I comandi con flag in grado di scrivere o eseguire, come `find`, `sort`, `sed` e `git`, richiedono comunque un prompt quando è presente un glob non quotato perché il glob potrebbe espandersi a un flag come `-delete`.

Un `cd` in un percorso all'interno della vostra directory di lavoro o di una [directory aggiuntiva](#working-directories) è anche di sola lettura. Un comando composto come `cd packages/api && ls` viene eseguito senza un prompt quando ogni parte si qualifica da sola. La combinazione di `cd` con `git` in un comando composto richiede un prompt quando il `cd` cambia in una directory diversa, poiché l'esecuzione di `git` in una nuova directory può eseguire gli hook di quella directory. Un `cd` il cui target si risolve alla directory di lavoro corrente è un'operazione nulla e non attiva questo prompt.

La combinazione di `cd` con un reindirizzamento di output in un comando composto richiede anche un prompt quando Claude Code non riesce a determinare in quale directory il target di reindirizzamento si risolve dopo l'esecuzione di `cd`. Un comando il cui unico target di reindirizzamento è `/dev/null`, come `cd app; grep -r pattern . 2>/dev/null`, non attiva questo prompt, perché `/dev/null` non dipende dalla directory di lavoro. Prima della v2.1.207, un comando composto contenente `cd` richiedeva un prompt per qualsiasi reindirizzamento di output, incluso uno il cui unico target era `/dev/null`.

<Warning>
  I modelli di autorizzazione Bash che tentano di vincolare gli argomenti del comando sono fragili. Ad esempio, `Bash(curl http://github.com/ *)` intende limitare curl agli URL di GitHub, ma non corrisponderà a variazioni come:

  * Opzioni prima dell'URL: `curl -X GET http://github.com/...`
  * Protocollo diverso: `curl https://github.com/...`
  * Reindirizzamenti: `curl -L http://bit.ly/xyz`, che reindirizza a GitHub
  * Variabili: `URL=http://github.com && curl $URL`
  * Spazi extra: `curl  http://github.com`

  Per un filtraggio URL più affidabile, considerate:

  * **Limitare gli strumenti di rete Bash**: utilizzate regole deny per bloccare `curl`, `wget` e comandi simili, quindi utilizzate lo strumento WebFetch con l'autorizzazione `WebFetch(domain:github.com)` per i domini consentiti
  * **Utilizzare hook PreToolUse**: implementate un hook che convalida gli URL nei comandi Bash e blocca i domini non consentiti
  * **Aggiungere guida CLAUDE.md**: descrivete i vostri modelli curl consentiti in `CLAUDE.md`. Questo modella quello che Claude prova ma non applica un confine, quindi abbinate con una delle opzioni sopra

  Nota che l'utilizzo di WebFetch da solo non impedisce l'accesso alla rete. Se Bash è consentito, Claude può comunque utilizzare `curl`, `wget` o altri strumenti per raggiungere qualsiasi URL.
</Warning>

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

Le regole `Edit` si applicano a tutti gli strumenti integrati che modificano i file. Claude fa un tentativo migliore per applicare le regole `Read` a tutti gli strumenti integrati che leggono file come Grep e Glob, a menzioni `@file` nei vostri prompt e alla selezione e al contesto di file aperti che un [IDE](/docs/it/vs-code#the-built-in-ide-mcp-server) connesso condivide con Claude.

Una regola deny `Read` blocca anche lo [strumento Edit](/docs/it/errors#file-is-covered-by-a-read-deny-rule) sullo stesso percorso, inclusa la creazione di un nuovo file lì. Write e NotebookEdit non sono coperti, quindi aggiungete una regola deny `Edit` per i percorsi che nessuno strumento può modificare. Richiede Claude Code v2.1.208 o successivo.

<Warning>
  Le regole deny di Read e Edit si applicano agli strumenti di file integrati di Claude e ai comandi di file che Claude Code riconosce in Bash, come `cat`, `head`, `tail` e `sed`. Non si applicano a sottoprocessi arbitrari che leggono o scrivono file indirettamente, come uno script Python o Node che apre i file da solo. Per l'applicazione a livello del sistema operativo che blocca tutti i processi dall'accesso a un percorso, [abilitate la sandbox](/docs/it/sandboxing).
</Warning>

Le regole Read e Edit seguono entrambe la specifica [gitignore](https://git-scm.com/docs/gitignore) con quattro tipi di modello distinti:

| Modello           | Significato                                     | Esempio                          | Corrisponde                                                  |
| ----------------- | ----------------------------------------------- | -------------------------------- | ------------------------------------------------------------ |
| `//path`          | Percorso assoluto dalla radice del filesystem   | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                    |
| `~/path`          | Percorso dalla directory home                   | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                               |
| `/path`           | Percorso relativo alla fonte delle impostazioni | `Edit(/src/**/*.ts)`             | `<project root>/src/**/*.ts` nelle impostazioni del progetto |
| `path` o `./path` | Percorso relativo alla directory corrente       | `Read(*.env)`                    | `<cwd>/*.env`                                                |

<Warning>
  Un modello come `/Users/alice/file` non è un percorso assoluto. La singola barra iniziale si ancora alla fonte delle impostazioni, non alla radice del filesystem. Utilizzate `//Users/alice/file` per i percorsi assoluti.
</Warning>

Un modello `/path` si ancora alla directory associata al file di impostazioni che lo definisce, quindi la stessa regola corrisponde a posizioni diverse a seconda di dove la mettete:

| Regola definita in                                              | `/path` si risolve a       |
| :-------------------------------------------------------------- | :------------------------- |
| Impostazioni di progetto o locali, come `.claude/settings.json` | `<project root>/path`      |
| Impostazioni utente in `~/.claude/settings.json`                | `~/.claude/path`           |
| Un file passato con `--settings <file>`                         | `<directory of file>/path` |
| Flag CLI, `/permissions` o regole di sessione                   | `<original cwd>/path`      |

Una regola deny come `Read(/secrets/**)` nelle impostazioni utente blocca `~/.claude/secrets/**`, non una directory `secrets` nel vostro progetto. Per scrivere una regola nelle impostazioni utente che si applica all'interno di ogni progetto, utilizzate un percorso assoluto `//` o un percorso relativo alla home `~/` invece.

Su Windows, i percorsi vengono normalizzati in forma POSIX prima della corrispondenza. `C:\Users\alice` diventa `/c/Users/alice`, quindi utilizzate `//c/**/.env` per corrispondere ai file `.env` in qualsiasi punto su quel drive. Per corrispondere su tutti i drive, utilizzate `//**/.env`.

Esempi:

* `Edit(/docs/**)`: modifica in `<project>/docs/`, non `/docs/` o `<project>/.claude/docs/`
* `Read(~/.zshrc)`: legge il `.zshrc` della vostra directory home
* `Edit(//tmp/scratch.txt)`: modifica il percorso assoluto `/tmp/scratch.txt`
* `Read(src/**)`: legge da `<current-directory>/src/`

Una regola corrisponde solo ai file sotto il suo ancoraggio, quindi l'ancoraggio determina quanto lontano una regola deny raggiunge. I nomi di file nudi seguono la semantica gitignore e corrispondono a qualsiasi profondità, quindi `Read(.env)` e `Read(**/.env)` sono equivalenti:

| Regola deny                    | Blocca                                              | Non blocca                                             |
| ------------------------------ | --------------------------------------------------- | ------------------------------------------------------ |
| `Read(.env)` o `Read(**/.env)` | qualsiasi `.env` alla o sotto la directory corrente | `.env` in una directory padre o in un altro progetto   |
| `Read(//**/.env)`              | qualsiasi `.env` in qualsiasi punto del filesystem  | nulla; la regola è ancorata alla radice del filesystem |

<Note>
  Nei modelli gitignore, `*` corrisponde ai file in un singolo segmento di percorso e può apparire in qualsiasi posizione nel modello, mentre `**` corrisponde tra le directory. Per consentire l'accesso a tutti i file, utilizzate solo il nome dello strumento senza parentesi: `Read`, `Edit` o `Write`.
</Note>

Quando approvate un percorso di file con "Sì, non chiedere più", Claude Code sfugge ai caratteri di modello gitignore in quel percorso, come `[`, `]` e `*`, quindi la regola generata corrisponde solo al percorso letterale che avete approvato. Le regole che scrivete voi stessi non vengono sfuggite. Prima della v2.1.202, Claude Code salvava il percorso non sfuggito, quindi una regola generata per una directory denominata `[2024-06] Reports` potrebbe non corrispondere al suo stesso percorso o corrispondere a directory fratelli non intenzionali.

Quando Claude accede a un symlink, le regole di autorizzazione controllano due percorsi: il symlink stesso e il file a cui si risolve. Le regole allow e deny trattano quella coppia diversamente: le regole allow ricadono nel richiedere un prompt, mentre le regole deny bloccano completamente.

* **Regole allow**: si applicano solo quando sia il percorso del symlink che il suo target corrispondono. Un symlink all'interno di una directory consentita che punta al di fuori di essa richiede comunque un prompt.
* **Regole deny**: si applicano quando il percorso del symlink o il suo target corrisponde. Un symlink che punta a un file negato è esso stesso negato.

Ad esempio, con `Read(./project/**)` consentito e `Read(~/.ssh/**)` negato, un symlink in `./project/key` che punta a `~/.ssh/id_rsa` viene bloccato: il target non supera la regola allow e corrisponde alla regola deny.

<h3 id="webfetch">
  WebFetch
</h3>

Le regole WebFetch utilizzano un prefisso `domain:` e corrispondono al nome host dell'URL richiesto. La corrispondenza non è sensibile alle maiuscole, supporta caratteri jolly `*` e rimuove un punto finale da entrambi la regola e il nome host in modo che `example.com.` e `example.com` vengono trattati allo stesso modo.

* `WebFetch(domain:example.com)` corrisponde alle richieste a `example.com`
* `WebFetch(domain:*.example.com)` corrisponde a qualsiasi sottodominio a qualsiasi profondità, come `api.example.com` o `a.b.example.com`, ma non a `example.com` stesso
* `WebFetch(domain:*)` corrisponde a ogni dominio ed è equivalente a una regola WebFetch nuda

In qualsiasi posizione diversa da un `*.` iniziale o da un `*` nudo, il carattere jolly corrisponde solo al testo tra due punti. `WebFetch(domain:example.*)` corrisponde a `example.org`, dove `*` diventa `org`, ma non a `example.evil.com`, dove `*` dovrebbe diventare `evil.com` e attraversare un punto. Questo impedisce a un carattere jolly finale di corrispondere a domini che un attaccante potrebbe registrare.

<h3 id="mcp">
  MCP
</h3>

Le regole MCP utilizzano il nome del server come configurato in Claude Code, facoltativamente seguito dal nome di uno strumento da quel server.

* `mcp__puppeteer` corrisponde a qualsiasi strumento fornito dal server `puppeteer`
* `mcp__puppeteer__*` utilizza la sintassi con caratteri jolly e corrisponde anche a tutti gli strumenti dal server `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` corrisponde allo strumento `puppeteer_navigate` fornito dal server `puppeteer`

Se la vostra organizzazione ha impostato uno strumento [connettore claude.ai](/docs/it/mcp#organization-controls-on-connector-tools) su `ask`, le regole allow per quello strumento non hanno effetto: Claude Code richiede un prompt ad ogni chiamata, anche nelle modalità `auto` e `bypassPermissions`. In modalità `dontAsk`, che non richiede mai un prompt, Claude Code nega la chiamata invece. Gli strumenti connettore appaiono come `mcp__claude_ai_<server>__<tool>`.

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

| Regola                | Corrisponde                                               | Non corrisponde                   |
| --------------------- | --------------------------------------------------------- | --------------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                              | `~/code/app/src`, `~/code`        |
| `Cd(~/code/**)`       | `~/code` e qualsiasi directory sotto di esso              | directory al di fuori di `~/code` |
| `Cd(**/node_modules)` | qualsiasi directory `node_modules` a qualsiasi profondità | `node_modules/pkg`                |

<h2 id="extend-permissions-with-hooks">
  Estendere le autorizzazioni con hook
</h2>

Gli [hook di Claude Code](/docs/it/hooks-guide) forniscono un modo per registrare comandi shell personalizzati per eseguire la valutazione delle autorizzazioni in fase di esecuzione. Quando Claude Code effettua una chiamata di strumento, gli hook PreToolUse vengono eseguiti prima del prompt di autorizzazione. L'output dell'hook può negare la chiamata dello strumento, forzare un prompt o saltare il prompt per consentire alla chiamata di procedere.

Le decisioni dell'hook non bypassano le regole di autorizzazione. Claude Code valuta le regole deny e ask indipendentemente da ciò che un hook PreToolUse restituisce: una regola deny corrispondente blocca la chiamata e una regola ask corrispondente richiede comunque un prompt anche quando l'hook ha restituito `"allow"` o `"ask"`. Questo preserva la precedenza deny-first descritta in [Gestire le autorizzazioni](#manage-permissions), incluse le regole deny impostate nelle impostazioni gestite.

Gli strumenti Connector [che la vostra organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) richiedono comunque un prompt quando un hook restituisce `"allow"`.

Un hook di blocco ha anche la precedenza sulle regole allow. Un hook che esce con codice 2 interrompe la chiamata dello strumento prima che le regole di autorizzazione vengono valutate, quindi il blocco si applica anche quando una regola allow consentirebbe altrimenti la chiamata. Per eseguire tutti i comandi Bash senza prompt tranne alcuni che desiderate bloccare, aggiungete `"Bash"` al vostro elenco allow e registrate un hook PreToolUse che rifiuta quei comandi specifici. Vedi [Bloccare le modifiche ai file protetti](/docs/it/hooks-guide#block-edits-to-protected-files) per uno script di hook che potete adattare.

<h2 id="working-directories">
  Directory di lavoro
</h2>

Per impostazione predefinita, Claude ha accesso ai file nella directory in cui è stato avviato. Potete estendere questo accesso:

* **Durante l'avvio**: utilizzate l'argomento CLI `--add-dir <path>`
* **Durante la sessione**: utilizzate il comando `/add-dir`
* **Configurazione persistente**: aggiungete a `additionalDirectories` nei [file di impostazioni](/docs/it/settings#settings-files)

I file nelle directory aggiuntive seguono le stesse regole di autorizzazione della directory di lavoro originale: diventano leggibili senza prompt e le autorizzazioni di modifica dei file seguono la modalità di autorizzazione corrente.

Nelle sessioni in background su macOS, l'host della sessione richiede l'accesso a cartelle protette come `~/Desktop`, `~/Documents` e `~/Downloads` separatamente dal vostro terminale quando Claude ha bisogno di leggere o scrivere file lì; se le letture lì falliscono con `Operation not permitted`, consultate [come concedere l'accesso alle cartelle alle sessioni in background](/docs/it/agent-view#background-sessions-can't-read-desktop-documents-or-downloads-on-macos).

Per modificare la directory di lavoro primaria della sessione invece di aggiungerne un'altra, utilizzate [`/cd`](/docs/it/commands). Il comando `/cd` richiede Claude Code v2.1.169 o successivo. A differenza di `/add-dir`, trasferisce la sessione: il `CLAUDE.md` della nuova directory viene caricato e `--resume` trova la sessione da lì.

<h3 id="additional-directories-grant-file-access-not-configuration">
  Le directory aggiuntive concedono l'accesso ai file, non la configurazione
</h3>

L'aggiunta di una directory estende dove Claude può leggere e modificare i file. Non rende quella directory una radice di configurazione completa: la maggior parte della configurazione `.claude/` non viene scoperta dalle directory aggiuntive, anche se alcuni tipi vengono caricati come eccezioni.

Queste eccezioni si applicano solo alle directory aggiunte con il flag `--add-dir` o il comando `/add-dir`. Le directory elencate in `permissions.additionalDirectories` in un file di impostazioni concedono solo l'accesso ai file e non caricano nessuna delle configurazioni di seguito.

I seguenti tipi di configurazione vengono caricati dalle directory `--add-dir`:

| Configurazione                                                                          | Caricato da `--add-dir`                                                                                                                                                                  |
| :-------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/it/skills) in `.claude/skills/`                                               | Sì, con ricaricamento live                                                                                                                                                               |
| [Subagents](/docs/it/sub-agents) in `.claude/agents/`                                        | Sì                                                                                                                                                                                       |
| [Impostazioni](/docs/it/settings) in `.claude/settings.json` e `.claude/settings.local.json` | Solo le chiavi `enabledPlugins` e `extraKnownMarketplaces`                                                                                                                               |
| File [CLAUDE.md](/docs/it/memory), `.claude/rules/` e `CLAUDE.local.md`                      | Solo quando `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` è impostato. `CLAUDE.local.md` richiede inoltre l'impostazione `local` source, che è abilitata per impostazione predefinita |

I comandi e gli stili di output vengono scoperti dalla directory di lavoro corrente e dai suoi genitori, dalla vostra directory utente in `~/.claude/` e dalle impostazioni gestite. Hooks e altre chiavi `settings.json` vengono caricati dalla cartella `.claude/` della directory di lavoro corrente senza fallback alla directory genitore, insieme al vostro `~/.claude/settings.json` utente e alle impostazioni gestite. Per condividere quella configurazione tra progetti, utilizzate uno di questi approcci:

* **Configurazione a livello utente**: posizionate i file in `~/.claude/agents/`, `~/.claude/output-styles/` o `~/.claude/settings.json` per renderli disponibili in ogni progetto
* **Plugin**: pacchetto e distribuite la configurazione come [plugin](/docs/it/plugins) che i team possono installare
* **Avviate dalla directory di configurazione**: eseguite Claude Code dalla directory contenente la configurazione `.claude/` che desiderate

<h2 id="how-permissions-interact-with-sandboxing">
  Come le autorizzazioni interagiscono con il sandboxing
</h2>

Le autorizzazioni e il [sandboxing](/docs/it/sandboxing) sono livelli di sicurezza complementari:

* **Autorizzazioni** controllano quali strumenti Claude Code può utilizzare e quali file o domini può accedere. Si applicano a tutti gli strumenti, inclusi Bash, Read, Edit, WebFetch e MCP.
* **Sandboxing** fornisce l'applicazione a livello del sistema operativo che limita l'accesso al filesystem e alla rete dello strumento Bash. Si applica solo ai comandi Bash e ai loro processi figlio.

Utilizzate entrambi per la difesa in profondità:

* Le regole deny di autorizzazione impediscono a Claude di tentare anche di accedere alle risorse limitate
* Le restrizioni sandbox impediscono ai comandi Bash di raggiungere risorse al di fuori dei confini definiti, anche se un'iniezione di prompt bypassa il processo decisionale di Claude
* Le restrizioni del filesystem nella sandbox combinano le impostazioni [`sandbox.filesystem`](/docs/it/sandboxing) con le regole deny di Read e Edit; entrambe vengono unite nel confine sandbox finale
* Le restrizioni di rete combinano le regole di autorizzazione WebFetch con gli elenchi `allowedDomains` e `deniedDomains` della sandbox

Quando il sandboxing è abilitato con `autoAllowBashIfSandboxed: true`, che è l'impostazione predefinita, i comandi Bash in sandbox vengono eseguiti senza richiedere un prompt anche se le vostre autorizzazioni includono una regola ask bare `Bash`, o la [forma equivalente `Bash(*)` ](#match-all-uses-of-a-tool): il confine della sandbox sostituisce il prompt per l'intero strumento. Questi controlli si applicano ancora:

* Le regole ask con ambito di contenuto come `Bash(git push *)` forzano comunque un prompt
* Le regole deny esplicite si applicano ancora
* I comandi `rm` o `rmdir` che hanno come destinazione `/`, la vostra directory home o altri percorsi critici del sistema attivano comunque un prompt

I comandi che non verranno eseguiti in sandbox, come i comandi esclusi, rispettano la regola ask bare `Bash` come al solito. Consultate [modalità sandbox](/docs/it/sandboxing#sandbox-modes) per modificare questo comportamento.

<h2 id="managed-settings">
  Impostazioni gestite
</h2>

Per le organizzazioni che necessitano di un controllo centralizzato sulla configurazione di Claude Code, gli amministratori possono distribuire impostazioni gestite che non possono essere ignorate dalle impostazioni utente o di progetto. Queste impostazioni di criteri seguono lo stesso formato dei file di impostazioni regolari e possono essere fornite tramite criteri MDM/a livello del sistema operativo, file di impostazioni gestite, [impostazioni gestite dal server](/docs/it/server-managed-settings), o un [gateway di app Claude](/docs/it/claude-apps-gateway) auto-ospitato. Vedi [file di impostazioni](/docs/it/settings#settings-files) per i meccanismi di consegna e i percorsi dei file.

<h3 id="managed-only-settings">
  Impostazioni solo gestite
</h3>

Le seguenti impostazioni sono efficaci solo nelle impostazioni gestite. Posizionarle nei file di impostazioni utente o di progetto non ha alcun effetto.

| Impostazione                                   | Descrizione                                                                                                                                                                                                                                                                                                                                     |
| :--------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowAllClaudeAiMcps`                         | Quando `true`, i connettori claude.ai si caricano insieme a un `managed-mcp.json` distribuito invece di essere soppressi dal suo controllo esclusivo. Vedi [Configurazione MCP gestita](/docs/it/managed-mcp)                                                                                                                                        |
| `allowedChannelPlugins`                        | Elenco di autorizzazione dei plugin di canale che possono inviare messaggi. Sostituisce l'elenco di autorizzazione Anthropic predefinito quando impostato. Richiede `channelsEnabled: true`. Vedi [Limitare quali plugin di canale possono essere eseguiti](/docs/it/channels#restrict-which-channel-plugins-can-run)                                |
| `allowManagedHooksOnly`                        | Quando `true`, solo gli hook gestiti, gli hook SDK e gli hook dai plugin force-enabled nelle impostazioni gestite `enabledPlugins` vengono caricati. Gli hook utente, progetto e tutti gli altri plugin vengono bloccati                                                                                                                        |
| `allowManagedMcpServersOnly`                   | Quando `true`, solo `allowedMcpServers` dalle impostazioni gestite sono rispettati. `deniedMcpServers` si unisce comunque da tutte le fonti. Vedi [Configurazione MCP gestita](/docs/it/managed-mcp)                                                                                                                                                 |
| `allowManagedPermissionRulesOnly`              | Quando `true`, impedisce alle impostazioni utente e di progetto di definire regole di autorizzazione `allow`, `ask` o `deny`. Si applicano solo le regole nelle impostazioni gestite. Non influisce sull'elenco di autorizzazione del server MCP; per questo, impostare `allowManagedMcpServersOnly`                                            |
| `blockedMarketplaces`                          | Elenco di blocco delle fonti del marketplace. Le fonti bloccate vengono controllate prima del download, quindi non toccano mai il filesystem. Vedi [restrizioni marketplace gestite](/docs/it/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                  |
| `channelsEnabled`                              | Consenti [channels](/docs/it/channels) per l'organizzazione. Vedi [controlli aziendali](/docs/it/channels#enterprise-controls) per l'impostazione predefinita su ogni piano                                                                                                                                                                               |
| `disableSideloadFlags`                         | Rifiuta i flag CLI `--plugin-dir`, `--plugin-url`, `--agents` e `--mcp-config` all'avvio. Senza questo, gli utenti possono aggirare `strictKnownMarketplaces` per una singola esecuzione passando questi flag. Vedi [`disableSideloadFlags`](/docs/it/settings#available-settings). Richiede Claude Code v2.1.193 o successivo                       |
| `forceRemoteSettingsRefresh`                   | Quando `true`, blocca l'avvio della CLI fino a quando le impostazioni gestite remote non vengono recuperate di recente ed esce se il recupero non riesce. Vedi [applicazione fail-closed](/docs/it/server-managed-settings#enforce-fail-closed-startup)                                                                                              |
| `pluginTrustMessage`                           | Messaggio personalizzato aggiunto all'avviso di fiducia del plugin mostrato prima dell'installazione                                                                                                                                                                                                                                            |
| `sandbox.filesystem.allowManagedReadPathsOnly` | Quando `true`, solo i percorsi `filesystem.allowRead` dalle impostazioni gestite sono rispettati. `denyRead` si unisce comunque da tutte le fonti                                                                                                                                                                                               |
| `sandbox.network.allowManagedDomainsOnly`      | Quando `true`, solo `allowedDomains` e le regole allow `WebFetch(domain:...)` dalle impostazioni gestite sono rispettate. I domini non consentiti vengono bloccati automaticamente senza richiedere all'utente. I domini negati si uniscono comunque da tutte le fonti                                                                          |
| `strictKnownMarketplaces`                      | Controlla quali marketplace di plugin gli utenti possono aggiungere e installare plugin da. Vedi [restrizioni marketplace gestite](/docs/it/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                                                                    |
| `strictPluginOnlyCustomization`                | Blocca skills, agents, hooks e server MCP da fonti utente e di progetto, in modo che possano provenire solo da plugin o impostazioni gestite. `true` blocca tutte e quattro le superfici; un array come `["skills", "hooks"]` blocca solo quelle denominate. Vedi [`strictPluginOnlyCustomization`](/docs/it/settings#strictpluginonlycustomization) |
| `wslInheritsWindowsSettings`                   | Quando `true` nella chiave del registro HKLM di Windows o in `C:\Program Files\ClaudeCode\managed-settings.json`, WSL legge le impostazioni gestite dalla catena di criteri di Windows oltre a `/etc/claude-code`. Vedi [File di impostazioni](/docs/it/settings#settings-files)                                                                     |

`disableBypassPermissionsMode` è tipicamente posizionato nelle impostazioni gestite per applicare la politica organizzativa, ma funziona da qualsiasi ambito. Un utente può impostarlo nelle proprie impostazioni per bloccarsi dalla modalità bypass.

<Note>
  Su piani Team e Enterprise, un Owner abilita o disabilita [Remote Control](/docs/it/remote-control) e [sessioni web](/docs/it/claude-code-on-the-web) a livello organizzativo nelle [impostazioni di amministrazione di Claude Code](https://claude.ai/admin-settings/claude-code). Remote Control può inoltre essere disabilitato per dispositivo con l'impostazione [`disableRemoteControl`](/docs/it/settings#available-settings). Le sessioni web non hanno chiavi di impostazioni gestite per dispositivo.
</Note>

<h2 id="settings-precedence">
  Precedenza delle impostazioni
</h2>

Le regole di autorizzazione seguono la stessa [precedenza delle impostazioni](/docs/it/settings#settings-precedence) di tutte le altre impostazioni di Claude Code:

1. **Impostazioni gestite**: non possono essere ignorate da nessun altro livello, inclusi gli argomenti della riga di comando
2. **Argomenti della riga di comando**: override temporanei della sessione
3. **Impostazioni di progetto locale** (`.claude/settings.local.json`)
4. **Impostazioni di progetto condivise** (`.claude/settings.json`)
5. **Impostazioni utente** (`~/.claude/settings.json`)

Se uno strumento viene negato a qualsiasi livello, nessun altro livello può consentirlo. Ad esempio, un deny delle impostazioni gestite non può essere ignorato da `--allowedTools` e `--disallowedTools` può aggiungere restrizioni oltre a quelle definite dalle impostazioni gestite.

Lo stesso vale tra gli ambiti delle impostazioni: se le impostazioni utente consentono un'autorizzazione e le impostazioni di progetto la negano, la regola di negazione la blocca. Il contrario è vero anche: un deny a livello utente blocca un allow a livello di progetto, perché le regole di negazione da qualsiasi ambito vengono valutate prima delle regole di consentimento.

Gli host di embedding possono fornire ulteriori criteri gestiti tramite l'opzione SDK `managedSettings` quando [`parentSettingsBehavior`](/docs/it/settings#settings-precedence) è impostato su `"merge"`; i valori dell'embedder possono irrigidire la politica ma non allentarla.

<h2 id="project-allow-rules-and-workspace-trust">
  Regole di autorizzazione del progetto e trust dell'area di lavoro
</h2>

Le regole `permissions.allow` e le voci `permissions.additionalDirectories` nel file `.claude/settings.json` di un progetto concedono capacità, quindi Claude Code le applica solo dopo che accettate la [finestra di dialogo di trust dell'area di lavoro](/docs/it/security#additional-safeguards) per quell'area di lavoro. Fino ad allora, Claude Code legge le regole ma non le applica. La finestra di dialogo di trust elenca le regole di autorizzazione e le directory aggiuntive che la cartella concederà, in modo che possiate esaminarle prima di accettare. Le regole `deny` e `ask` non sono interessate, poiché limitano solo.

Claude Code salva il trust per area di lavoro, identificato dalla radice del repository git o, al di fuori di un repository, dalla directory da cui avete avviato Claude Code. Quando avviate dalla vostra directory home, il trust viene mantenuto solo per la sessione corrente e non viene scritto su disco; consultate la nota [safeguard aggiuntivi](/docs/it/security#additional-safeguards). Fidarsi di una directory padre non applica le regole di autorizzazione di un progetto annidato.

`.claude/settings.local.json` è il vostro file personale, quindi il controllo di trust dell'area di lavoro di solito non si applica ad esso. Quando un repository potrebbe aver fornito il file, ad esempio quando è committato in git o `.claude` è un symlink, le sue regole di autorizzazione e directory aggiuntive passano attraverso il controllo di trust come le impostazioni del progetto.

Claude Code esegue git per verificare se il repository ha fornito il file, ed esegue quel controllo solo in una cartella coperta da una finestra di dialogo di trust accettata, per quella cartella o per una delle sue directory padre. In una sessione interattiva in una cartella che non avete ancora fidata, le regole di autorizzazione e le directory aggiuntive in `.claude/settings.local.json` passano attraverso il controllo di trust come le impostazioni del progetto fino a quando non accettate la finestra di dialogo, a meno che la sessione non venga eseguita nella vostra directory di configurazione personale come descritto di seguito. Delle due eccezioni di seguito, solo l'eccezione della directory di configurazione si applica prima della finestra di dialogo, perché non ha bisogno di eseguire git. Determinare che una directory non si trova all'interno di un repository git utilizza lo stesso controllo git, quindi l'eccezione non-all'interno-di-un-repository ha effetto una volta che una finestra di dialogo di trust che copre la cartella è accettata. Prima della v2.1.207, un `.claude/settings.local.json` non tracciato applicava le sue regole di autorizzazione in quella cartella prima che accettaste la finestra di dialogo.

Le regole di autorizzazione e le directory aggiuntive in `.claude/settings.local.json` si applicano anche senza trust dell'area di lavoro in due casi:

* La directory da cui avete avviato Claude Code non si trova all'interno di un repository git.
* La sessione viene eseguita nella vostra directory di configurazione personale: la vostra directory home o qualsiasi directory il cui subdirectory `.claude` avete impostato come [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars).

In entrambi i casi il file è uno che avete creato piuttosto che uno che un repository potrebbe aver fornito, e un `.claude/settings.local.json` committato nel repository richiede comunque il trust dell'area di lavoro. Le versioni 2.1.196 fino a 2.1.199 hanno trattato il file come fornito dal repository in quelle aree di lavoro, hanno ignorato le sue regole di autorizzazione e hanno stampato un avviso [`this workspace has not been trusted`](/docs/it/errors#workspace-has-not-been-trusted) su stderr. Le due eccezioni di cui sopra corrispondono alla v2.1.195 e versioni precedenti e sono state ripristinate nella v2.1.200.

Inoltre a partire dalla v2.1.200, un'area di lavoro le cui regole di autorizzazione o directory aggiuntive ancora non vengono applicate, ma che non ha mai mostrato la finestra di dialogo di trust perché una directory padre era già fidata, mostra la finestra di dialogo la prossima volta che avviate Claude Code lì in modo interattivo. La finestra di dialogo offre due scelte:

* **Sì, mi fido di questa cartella**: salva il trust per quell'area di lavoro e applica le regole nella stessa sessione.
* **No, continua senza questi permessi**: continua a lavorare con quelle regole ignorate. La finestra di dialogo appare di nuovo nella sessione successiva.

In [modalità non interattiva](/docs/it/headless) con `-p`, nessuna finestra di dialogo appare e le regole rimangono ignorate.

<h2 id="example-configurations">
  Configurazioni di esempio
</h2>

Questo [repository](https://github.com/anthropics/claude-code/tree/main/examples/settings) include configurazioni di impostazioni iniziali per scenari di distribuzione comuni. Utilizzatele come punti di partenza e adattatele alle vostre esigenze.

<h2 id="see-also">
  Vedi anche
</h2>

* [Settings](/docs/it/settings): riferimento di configurazione completo inclusa la tabella delle impostazioni di autorizzazione
* [Configure auto mode](/docs/it/auto-mode-config): comunicate al classificatore della modalità auto quale infrastruttura la vostra organizzazione ritiene affidabile
* [Sandboxing](/docs/it/sandboxing): isolamento del filesystem e della rete a livello del sistema operativo per i comandi Bash
* [Authentication](/docs/it/authentication): configurate l'accesso utente a Claude Code
* [Security](/docs/it/security): salvaguardie di sicurezza e best practice
* [Hooks](/docs/it/hooks-guide): automatizzate i flussi di lavoro ed estendete la valutazione delle autorizzazioni
