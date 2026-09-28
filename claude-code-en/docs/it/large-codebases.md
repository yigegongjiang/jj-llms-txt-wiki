> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare Claude Code in un monorepo o in un codebase di grandi dimensioni

> Configura Claude Code per monorepo e codebase a singolo albero di grandi dimensioni con file CLAUDE.md annidati, worktree sparse, code intelligence e skills per pacchetto in modo che Claude rimanga focalizzato sul codice su cui stai lavorando.

Un codebase di grandi dimensioni può essere un repository con milioni di righe o un monorepo con molti pacchetti. Claude Code funziona a qualsiasi dimensione, ma man mano che il codebase cresce, i valori predefiniti ottimizzati per progetti più piccoli possono riempire la finestra di contesto con istruzioni e letture di file non correlate al compito, consumando token e degradando le prestazioni di Claude.

Questa guida mostra ai singoli sviluppatori e ai team di ingegneria come limitare Claude alla parte del codebase che un compito tocca. Ogni sezione nota se un'impostazione è personale per la tua macchina o impegnata nel repository.

<h2 id="what-this-guide-covers">
  Cosa copre questa guida
</h2>

La [tabella sottostante](#settings-on-this-page) elenca ogni impostazione e cosa realizza. L'[albero dei file dopo di essa](#the-example-monorepo) è il monorepo di esempio a cui si riferiscono tutti gli esempi di codice su questa pagina.

<h3 id="settings-on-this-page">
  Settings on this page
</h3>

Ogni impostazione sottostante è indipendente. Si stratificano piuttosto che si sostituiscono a vicenda, quindi applica quelle che si adattano al tuo repository. [Scegli dove avviare Claude](#choose-where-to-start-claude) determina dove vivono i tuoi file di impostazioni, quindi leggilo per primo. [Mettilo insieme](#put-it-together) mostra tutti loro combinati.

| Voglio                                                                                                        | Usa                                                                                             |
| :------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------- |
| Caricare solo le convenzioni per il codice che tocchi, invece di un file radice che copra ogni sottosistema   | File [CLAUDE.md](#layer-claude-md-files-by-directory) per directory                             |
| Escludere file CLAUDE.md per pacchetti in cui non lavori mai                                                  | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files)                                       |
| Impedire a Claude di aprire output di build, codice generato e dipendenze vendute                             | Regole di negazione [`Read`](#block-reads-of-generated-and-vendored-code) in `permissions.deny` |
| Trovare la definizione di un simbolo o i chiamanti attraverso il language server invece di scansionare i file | Un [plugin di code intelligence](#reduce-file-reads-with-code-intelligence)                     |
| Controllare solo le directory di cui ha bisogno un compito quando Claude crea un worktree                     | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need)                              |
| Leggere e modificare un pacchetto fratello o un altro repository dalla stessa sessione                        | [`--add-dir`](#grant-access-across-packages-or-repositories) o `additionalDirectories`          |
| Dare a Claude procedure specifiche di un'area che si caricano solo quando rilevanti                           | [Skills](#add-per-directory-skills) per directory                                               |
| Sostituire molti file CLAUDE.md per directory con un set di convenzioni che tutti installano                  | Un [plugin](#centralize-conventions-when-layering-stops-scaling) in un marketplace interno      |

<Tip>
  Per tecniche di flusso di lavoro che mantengono il contesto piccolo in qualsiasi repository, come [eseguire l'esplorazione in un subagent](/docs/it/best-practices#use-subagents-for-investigation) in modo che le letture di file rimangono fuori dalla conversazione principale, vedi [Best practices per Claude Code](/docs/it/best-practices). Per distribuire una configurazione di base a ogni sviluppatore nella tua organizzazione, vedi [Configurare Claude Code per la tua organizzazione](/docs/it/admin-setup).
</Tip>

<h3 id="the-example-monorepo">
  The example monorepo
</h3>

Gli esempi in questa pagina si riferiscono a un monorepo con tre pacchetti. Gli stessi modelli funzionano in un codebase a singolo albero di grandi dimensioni: dove un esempio usa `packages/api/`, sostituisci con la tua directory di sottosistema come `src/backend/` o `lib/core/`.

```text theme={null}
monorepo/
  CLAUDE.md                     # istruzioni radice
  packages/
    api/
      CLAUDE.md                 # istruzioni specifiche dell'API
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # istruzioni specifiche del frontend
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # istruzioni della libreria condivisa
      src/
```

<h2 id="choose-where-to-start-claude">
  Scegli dove avviare Claude
</h2>

Dove avvii `claude` determina quali file Claude può leggere e modificare senza una concessione di autorizzazione aggiuntiva, quali file CLAUDE.md si caricano nel contesto all'avvio e quali impostazioni di progetto si applicano.

| Avvia da              | Accesso ai file                                  | CLAUDE.md caricato all'avvio                                                          | Usa quando                                             |
| :-------------------- | :----------------------------------------------- | :------------------------------------------------------------------------------------ | :----------------------------------------------------- |
| Radice del repository | Ogni file                                        | Solo radice; i file di sottodirectory si caricano su richiesta quando Claude legge lì | I compiti si estendono su più pacchetti o sottosistemi |
| Una sottodirectory    | Solo quel sottoalbero, finché non concedi di più | Quella directory più ogni antenato                                                    | Il lavoro è limitato a un pacchetto o sottosistema     |

Le impostazioni di progetto in `.claude/settings.json` non vengono ereditate dalle directory padre come fanno i file CLAUDE.md. Per sapere quale directory `.claude/settings.json` una sessione legge, vedi [dove Claude Code cerca ogni file](/docs/it/settings#where-claude-code-looks-for-each-file).

Ogni sezione sottostante afferma se il suo file di impostazioni appartiene alla radice del repository o alla sottodirectory da cui avvii, e se è impegnato o mantenuto localmente.

<h2 id="layer-claude-md-files-by-directory">
  Stratifica file CLAUDE.md per directory
</h2>

In un codebase di grandi dimensioni, un singolo CLAUDE.md alla radice del repository tende a crescere per coprire le convenzioni di ogni sottosistema, consumando contesto su istruzioni non correlate al compito attuale, o rimanere troppo generico per essere utile. Dividere le istruzioni tra file per directory significa che Claude carica le regole a livello di repository più solo le convenzioni per il codice su cui stai lavorando.

Claude Code carica ogni file [CLAUDE.md](/docs/it/memory) dalla tua directory di lavoro e da ogni directory padre all'avvio, quindi carica il file di ogni sottodirectory su richiesta quando legge file lì. Un file radice imposta regole a livello di repository e ogni sottodirectory aggiunge la sua.

Una divisione comune è due livelli:

* **Root `CLAUDE.md`**: istruzioni che si applicano ovunque, come standard di codifica e convenzioni di commit
* **Per-sottodirectory `CLAUDE.md`**: convenzioni specifiche dello stack di quell'area. In un monorepo è uno per pacchetto. In un albero singolo di grandi dimensioni è uno per sottosistema come `src/db/` o `src/api/`

Impegna questi file nel repository in modo che i compagni di squadra li ereditino. Il proprietario di ogni directory in genere mantiene il suo file.

Per ridurre un file già archiviato, esegui il [controllo `/doctor`](/docs/it/memory#my-claude-md-is-too-large). Il root `CLAUDE.md` contiene le regole che si applicano in ogni pacchetto:

```markdown CLAUDE.md theme={null}
Esegui gli script del pacchetto dalla directory del pacchetto, non dalla radice del monorepo.
Prefissa i soggetti dei commit con il nome del pacchetto, ad esempio `api: add rate limiting`.
Non modificare mai i file sotto packages/*/generated/. Esegui `npm run codegen` nel pacchetto invece.
```

Il `CLAUDE.md` di ogni sottodirectory, qui `packages/api/CLAUDE.md`, aggiunge le convenzioni specifiche di quell'area:

```markdown packages/api/CLAUDE.md theme={null}
Copia `.env.example` in `.env` prima di eseguire qualsiasi cosa. I test e il server dev falliscono senza di esso.
Scrivi le query del database con il query builder Knex. Non mettere mai stringhe SQL grezze nei gestori di route.
Non modificare mai una migrazione dopo che è stata unita. Aggiungi una nuova migrazione invece.
```

Quando avvii Claude da `packages/api/`, carica sia `packages/api/CLAUDE.md` che il root `CLAUDE.md`. Claude vede le istruzioni locali insieme alle regole a livello di repository, senza istruzioni da `packages/web/` nel contesto. Lo stesso vale per qualsiasi sottodirectory in un albero non-monorepo. Per confermare quali file sono stati caricati, esegui `/context` e controlla l'elenco sotto **Memory files**.

Alcuni modi per mantenere i file attuali man mano che il codebase e i modelli cambiano:

* **Rivedi nelle pull request**: tratta le modifiche CLAUDE.md come qualsiasi altro cambio di documentazione in modo che le convenzioni tracciino il codice
* **Rivedi dopo i rilasci di modelli principali**: le istruzioni che hanno aggirato una limitazione di un modello più vecchio possono diventare overhead una volta che un modello più nuovo gestisce il caso da solo. Ad esempio, una regola che forza refactor a file singolo può essere eliminata una volta che la limitazione è scomparsa
* **Aggiungi un hook Stop che propone aggiornamenti**: un [`Stop` hook](/docs/it/hooks#stop) riceve il percorso della trascrizione della sessione quando Claude finisce di rispondere, quindi uno script può rivedere la sessione e proporre aggiornamenti CLAUDE.md mentre il divario che ha esposto è fresco

Per ulteriori informazioni su come i file CLAUDE.md si caricano e interagiscono, vedi [Memory e istruzioni di progetto](/docs/it/memory).

<h3 id="choose-between-per-directory-claude-md-and-path-scoped-rules">
  Scegli tra per-directory CLAUDE.md e regole con ambito di percorso
</h3>

I file `CLAUDE.md` per directory e le [regole con ambito di percorso](/docs/it/memory#path-specific-rules) sotto `.claude/rules/` ti permettono entrambi di indirizzare le istruzioni a parte dell'albero. Differiscono in dove vive il file e quando si carica.

| Approccio                                        | Posizione del file                         | Si carica quando                                                                            | Usa quando                                                                                                |
| :----------------------------------------------- | :----------------------------------------- | :------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------- |
| Per-directory `CLAUDE.md`                        | Dentro la directory, insieme al suo codice | All'avvio quando avviato da quella directory, o su richiesta quando Claude legge un file lì | I proprietari della directory mantengono le loro convenzioni; le istruzioni sono versionate con il codice |
| Regola con ambio di percorso in `.claude/rules/` | `.claude/` centrale alla radice del repo   | Quando Claude lavora con un file che corrisponde al glob `paths:` della regola              | Vuoi tutte le convenzioni in un posto, o la stessa regola si applica a molti percorsi sparsi              |

Per un confronto che copre anche le skills, vedi [Confronta funzionalità simili](/docs/it/features-overview#compare-similar-features).

<h3 id="exclude-irrelevant-claude-md-files">
  Escludi file CLAUDE.md irrilevanti
</h3>

Quando avvii Claude dalla radice del repository, il CLAUDE.md di ogni sottodirectory si carica non appena Claude legge un file in quella directory. L'impostazione `claudeMdExcludes` salta file specifici per percorso o modello glob in modo che non si carichino mai.

Usalo per directory in cui non lavori mai, come pacchetti di altri team, codice legacy o sottoalberi venduti. L'elenco di esclusione è statico, non un interruttore per compito. Per focalizzarsi su un pacchetto oggi e un altro domani, [avvia Claude da quella directory del pacchetto](#choose-where-to-start-claude) invece di modificare le esclusioni.

Se vuoi solo queste esclusioni per te, metti l'impostazione in `.claude/settings.local.json`. Claude Code aggiunge quel file al tuo gitignore globale quando salva un'impostazione lì. Poiché lo stai creando a mano qui, aggiungilo al tuo gitignore tu stesso. I modelli usano la sintassi glob abbinata ai percorsi di file assoluti, quindi inizia i modelli in stile relativo con `**/` per abbinare ovunque nell'albero. L'esempio sottostante esclude un pacchetto di proprietà di un altro team:

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/web/**"
  ]
}
```

Questo salta ogni CLAUDE.md e file di regole sotto quel pacchetto. Il root `CLAUDE.md` e i pacchetti su cui lavori ancora si caricano normalmente.

Questi modelli coprono altri casi comuni:

* `"**/packages/*/CLAUDE.md"`: esclude il CLAUDE.md di ogni pacchetto mantenendo la radice
* `"**/packages/legacy-*/**"`: esclude ogni pacchetto il cui nome corrisponde al glob, incluse le regole
* `"/home/user/monorepo/legacy/CLAUDE.md"`: esclude un file specifico per percorso assoluto

I file CLAUDE.md della politica gestita non possono essere esclusi, quindi le istruzioni a livello di organizzazione si applicano sempre. Puoi impostare `claudeMdExcludes` a qualsiasi [ambito di impostazioni](/docs/it/settings#where-settings-live): utente, progetto, locale o gestito. Gli array si uniscono tra gli ambiti, quindi un team può impostare i valori predefiniti a livello di progetto mentre gli individui aggiungono override locali.

Per la documentazione completa dell'esclusione, vedi [Escludi file CLAUDE.md specifici](/docs/it/memory#exclude-specific-claude-md-files).

<h2 id="reduce-what-claude-reads">
  Riduci quello che Claude legge
</h2>

Le istruzioni sono solo parte di quello che finisce nel contesto di Claude. Le letture di file sono un altro costo che cresce con il codebase. Le impostazioni sottostanti bloccano le letture di percorsi irrilevanti e sostituiscono le scansioni esaustive con ricerche del language server.

<h3 id="block-reads-of-generated-and-vendored-code">
  Blocca le letture di codice generato e venduto
</h3>

Le ricerche di contenuto di Claude rispettano `.gitignore` per impostazione predefinita, quindi i percorsi già elencati lì, come `node_modules/`, `dist/` e `build/`, rimangono fuori dai risultati di ricerca senza configurazione aggiuntiva.

Per i percorsi che sono archiviati, come un SDK venduto o codice generato impegnato, aggiungi regole di negazione `Read` in `permissions.deny` per impedire a Claude di aprire quei file.

Le regole di negazione possono coprire tutti coloro che lavorano nel repository, solo voi, o ogni sessione sulla macchina, a seconda di quale file di impostazioni vi inserite:

* **Tutti coloro che lavorano nel repository**: impegnate le regole a `.claude/settings.json`, alla radice del repository se avviate Claude lì, o in ogni `.claude/` del pacchetto se avviate da sottodirectory. Come altre impostazioni di progetto su questa pagina, quel file non viene ereditato dalle directory padre.
* **Solo voi**: usate `.claude/settings.local.json` alla radice del repository, che si carica in ogni sessione CLI all'interno del repository indipendentemente dalla directory di avvio, tranne nei casi in cui Claude Code [non utilizza la radice del repository](/docs/it/settings#where-claude-code-looks-for-each-file), come su Windows. I modelli relativi come l'esempio `Read(./**/vendor/**/*)` ancora [si ancorano alla directory di lavoro corrente della sessione](/docs/it/permissions#read-and-edit) piuttosto che alla radice del repository, quindi se avviate sessioni da sottodirectory, scrivete le regole in questo file come percorsi assoluti `//`, come `Read(//absolute/path/to/repo/**/vendor/**/*)`. Prima della v2.1.211, `.claude/settings.local.json` si caricava anche solo dalla directory di avvio.
* **Tutti, applicato in ogni sessione**: impostate le regole in [impostazioni gestite](/docs/it/managed-settings), che le impostazioni utente e progetto non possono ignorare.

L'esempio sottostante blocca gli artefatti di build e un SDK venduto. I suoi modelli di directory terminano con `/**/*` piuttosto che `/**` in modo che ogni regola copra tutto all'interno della directory ma non la directory stessa. Claude può quindi ancora elencare quelle directory o cambiarvi dentro, ad esempio con `ls dist` o `cd build`.

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)",
      "Read(./**/*.generated.*)",
      "Read(./**/vendor/**/*)"
    ]
  }
}
```

Le regole di negazione coprono gli strumenti di file integrati di Claude. In Bash, coprono i comandi di file che Claude Code riconosce, come `cat`, `head`, `grep` e `find`, quando un percorso negato appare come argomento, e il target di un [reindirizzamento](/docs/it/permissions#redirections) come `< file`. Claude Code fa anche un tentativo di massimo sforzo per lasciare i percorsi negati fuori dai risultati degli strumenti Grep e Glob integrati. Una ricerca Bash come `grep -r` o `find` su una directory che contiene file negati li include comunque nel suo output.

Le regole di negazione non coprono i sottoprocessi che aprono file da soli. Per la sintassi del modello completo, vedi [Regole di autorizzazione Read e Edit](/docs/it/permissions#read-and-edit).

<h3 id="reduce-file-reads-with-code-intelligence">
  Riduci le letture di file con code intelligence
</h3>

In un codebase di grandi dimensioni, trovare dove un simbolo è definito o usato può costare molte letture di file e chiamate grep. I [plugin di code intelligence](/docs/it/plugins/code-intelligence) collegano Claude a un language server in modo che possa saltare alle definizioni, trovare riferimenti e far emergere errori di tipo direttamente invece di scansionare l'albero.

Il marketplace ufficiale ha plugin per TypeScript, Python, Go, Rust e altri linguaggi comuni. Eseguite il comando sottostante all'interno di una sessione Claude Code per installare il plugin TypeScript:

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

Se l'installazione fallisce, abbinate il messaggio che Claude Code segnala:

* `Marketplace "claude-plugins-official" not found`: aggiungete il marketplace con `/plugin marketplace add anthropics/claude-plugins-official`, quindi riprovate l'installazione.
* Il plugin [non è trovato nel marketplace](/docs/it/plugins/install#install-a-plugin): controllate il nome del plugin.

Per abilitare un plugin per tutti nel repository piuttosto che installarlo voi stessi, aggiungetelo all'impostazione di progetto [`enabledPlugins`](/docs/it/settings-reference#plugin-settings).

I plugin di code intelligence richiedono il binario del language server del linguaggio su ogni macchina dello sviluppatore. Vedi [quale binario richiede ogni linguaggio](/docs/it/plugins/code-intelligence). L'installazione dal marketplace ufficiale richiede accesso di rete a GitHub, dove è ospitato il marketplace. Su una rete ristretta, [aggiungete il marketplace da un host Git interno o da un percorso locale](/docs/it/plugins/install#add-a-marketplace) invece.

Questo si abbina bene con `claudeMdExcludes` e le regole `Read` di negazione sopra. Quelli mantengono il contenuto irrilevante fuori dal contesto, e code intelligence impedisce a Claude di leggere attraverso quello che rimane per individuare una definizione.

<h2 id="scope-worktrees-and-file-access">
  Ambito worktrees e accesso ai file
</h2>

Queste impostazioni controllano cosa è su disco nei worktrees e quali directory Claude può leggere e scrivere oltre il tuo punto di partenza.

<h3 id="check-out-only-the-directories-you-need">
  Controlla solo le directory di cui hai bisogno
</h3>

Il flag `--worktree` avvia una sessione in un nuovo git worktree in modo che i cambiamenti rimangono isolati dal tuo checkout principale. Per impostazione predefinita controlla l'intero repository. In un repository di grandi dimensioni, l'impostazione `worktree.sparsePaths` usa git sparse-checkout per scrivere solo le directory elencate più i file a livello di radice su disco, in modo che i worktrees si avviino più velocemente e usino meno spazio.

Se tutti coloro che lavorano in questa directory hanno bisogno degli stessi percorsi, impegna l'impostazione a `.claude/settings.json`. Per aggiungere percorsi per te, usa `.claude/settings.local.json`: gli elenchi si uniscono tra gli ambiti, quindi un file locale può aggiungere percorsi all'elenco impegnato ma non rimuoverli.

I JSON esempi su questa pagina mostrano un'impostazione alla volta. Se il tuo `.claude/settings.json` contiene già altre chiavi, come le regole `permissions.deny` sopra, aggiungi la chiave `worktree` accanto a loro piuttosto che sostituire il file. [Mettilo insieme](#put-it-together) mostra il risultato combinato.

L'esempio sottostante mostra il file impegnato:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

Quando Claude crea un worktree, controlla solo `.claude/`, `packages/api/` e `packages/shared/` invece dell'albero completo. I percorsi in `sparsePaths` sono relativi alla radice del repository, indipendentemente da quale sottodirectory avvii Claude. Qualsiasi percorso di directory funziona qui, non solo le radici dei pacchetti.

Questo è particolarmente utile per [l'isolamento del worktree del subagent](/docs/it/worktrees#isolate-subagents-with-worktrees). I subagent sono istanze Claude parallele generate per sottocompiti, e ognuno che viene eseguito in un worktree ottiene un checkout leggero invece dell'albero completo. Tutti i worktrees in una sessione condividono lo stesso `sparsePaths`, quindi se un subagent ha bisogno di `packages/api/` e un altro ha bisogno di `packages/web/`, elencali entrambi.

Elenca le directory in `sparsePaths`, non i singoli file. I file a livello di radice come `package.json`, `tsconfig.base.json` e i file di lock vengono sempre controllati insieme alle directory che elenchi. Le directory a livello di radice non lo sono, quindi includi `.claude` nell'elenco se vuoi che il `.claude/settings.json` della radice del repository, `.claude/rules/` e le skills siano disponibili dentro il worktree. Per skills di progetto, agenti e comandi, vedi [Cosa i worktrees condividono con il checkout principale](/docs/it/worktrees#what-worktrees-share-with-the-main-checkout).

Sparse checkout richiede a git di abilitare `extensions.worktreeConfig` nel `.git/config` condiviso del repository mentre esiste un worktree sparse. Claude Code rimuove quella voce dopo che l'ultimo worktree viene rimosso, ma solo se Claude Code l'ha aggiunta. Non rimuove mai un valore che hai impostato tu stesso. Prima della v2.1.207, la voce rimaneva dopo che l'ultimo worktree veniva rimosso, e gli strumenti basati su go-git come `tea` non riuscivano ad aprire il repository finché non eseguivi `git config --unset extensions.worktreeConfig`.

Per evitare di duplicare directory di grandi dimensioni come `node_modules` tra i worktrees, abbina `sparsePaths` con `symlinkDirectories` nello stesso `.claude/settings.json`:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

Questo crea un symlink da `node_modules/` di ogni worktree alla copia del repository principale piuttosto che duplicarlo su disco.

<Note>
  Le impostazioni `sparsePaths` e `symlinkDirectories` vengono lette dalla tua directory di avvio prima che il worktree venga creato. Dopo la creazione, la directory di lavoro della sessione è la radice del worktree, non la sottodirectory da cui hai avviato. Le impostazioni di progetto dentro il worktree quindi si caricano da `.claude/settings.json` della radice del worktree, la copia controllata del file della radice del repository. Metti qualsiasi altra impostazione di cui hai bisogno dentro i worktrees, come regole di autorizzazione o hook, in `.claude/settings.json` della radice del repository.
</Note>

Per il riferimento completo delle impostazioni del worktree, vedi [Impostazioni del worktree](/docs/it/settings-reference#worktree).

<h3 id="grant-access-across-packages-or-repositories">
  Concedi accesso tra pacchetti o repository
</h3>

Questa sezione si applica quando avvii Claude da una sottodirectory, o quando un compito si estende su più checkouts. Se avvii dalla radice del repository in un albero singolo di grandi dimensioni, Claude ha già accesso a ogni file e puoi saltare questo.

Quando avvii Claude da `packages/api/`, può leggere e scrivere file all'interno di quella directory. Se un compito richiede cambiamenti tra pacchetti, come aggiornare un tipo condiviso che sia `api` che `web` importano, devi concedere accesso alla directory fratello. Lo stesso meccanismo concede accesso a un repository controllato separatamente.

L'impostazione `additionalDirectories` in `.claude/settings.json` dà a Claude accesso alle directory al di fuori della directory di lavoro. L'esempio sottostante concede accesso a due pacchetti fratelli:

```json packages/api/.claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

I percorsi relativi si risolvono rispetto alla directory da cui avvii Claude. Con questa configurazione, Claude può leggere e modificare file in `packages/shared/` e `packages/web/` mentre lavora da `packages/api/`.

Puoi anche concedere accesso in fase di esecuzione senza modificare le impostazioni passando `--add-dir` quando avvii Claude:

```bash theme={null}
claude --add-dir ../shared
```

Comunque aggiungi una directory, Claude può leggere e modificare file in essa. Se il CLAUDE.md della directory, i file `.claude/rules/` e le skills si caricano anche dipende da come l'hai aggiunta:

| Aggiunto con                          | Carica CLAUDE.md e regole                     | Carica skills |
| :------------------------------------ | :-------------------------------------------- | :------------ |
| Impostazione `additionalDirectories`  | Mai                                           | Mai           |
| Flag `--add-dir` o comando `/add-dir` | Solo con la variabile di ambiente sottostante | Sì            |

Per caricare file CLAUDE.md e regole da una directory aggiunta con `--add-dir` o `/add-dir`, imposta la variabile di ambiente `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

La variabile di ambiente non ha effetto sulle directory elencate nell'impostazione `additionalDirectories`. Vedi [Carica da directory aggiuntive](/docs/it/memory#load-from-additional-directories) per i dettagli.

Per directory fratello che tutti in questa area hanno bisogno, impegna `additionalDirectories` a `.claude/settings.json`. Per una selezione personale o un accesso una tantum, usa `.claude/settings.local.json` o passa `--add-dir` all'avvio.

<h2 id="add-per-directory-skills">
  Aggiungere skills per directory
</h2>

Qualsiasi sottodirectory può definire [skills](/docs/it/skills) limitate al proprio stack. Una skill si carica su richiesta quando Claude determina che è rilevante, quindi gli strumenti specifici dell'API non consumano contesto durante il lavoro frontend.

Le skills si trovano sotto `.claude/skills/` all'interno della directory. Eseguine il commit insieme al codice di quell'area in modo che chiunque cloni il repository le ottenga. In un monorepo questo può essere un set di skills per package. In un grande codebase a singolo albero è un set per sottosistema come `src/db/.claude/skills/`.

Crea una directory skill all'interno della sottodirectory:

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

Quindi scrivi `SKILL.md` all'interno di quella directory, qui `packages/api/.claude/skills/api-testing/SKILL.md`. Questo esempio insegna a Claude i pattern di testing del package API:

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: Testing patterns for the API package. Use when writing or modifying tests in packages/api/.
---

## Test structure

Tests are in `src/__tests__/` mirroring the `src/` directory structure.
Each route file has a corresponding `.test.ts` file.

## Running tests

- All tests: `npm test`
- Single file: `npm test -- src/__tests__/routes/users.test.ts`
- Watch mode: `npm test -- --watch`

## Test utilities

- `src/__tests__/helpers/db.ts`: provides `setupTestDb()` and `teardownTestDb()` for database tests
- `src/__tests__/helpers/auth.ts`: provides `createTestUser()` and `getAuthToken()` for authenticated endpoints

## Patterns

- Use `supertest` for HTTP assertions, not raw fetch
- Always wrap database tests in a transaction that rolls back
- Mock external services in `src/__tests__/mocks/`
```

Una diversa sottodirectory contiene diverse skills nello stesso modo: `packages/web/.claude/skills/component-patterns/` descrive le convenzioni dei componenti del frontend invece del testing. Quando Claude lavora su un file in `packages/api/`, carica la skill api-testing. Quando lavora in `packages/web/`, carica component-patterns invece. Le skills di nessuna directory si caricano durante i compiti dell'altra.

Puoi anche limitare una skill per pattern di file invece che per posizionamento. Il [campo frontmatter `paths`](/docs/it/skills#frontmatter-reference) accetta pattern glob, e Claude carica la skill automaticamente solo quando lavora con file corrispondenti. Usalo per una skill che si trova nella `.claude/skills/` della radice del repository ma si applica solo a certi file ovunque appaiano, come una skill di migrazione del database limitata a `**/migrations/**`.

Per ulteriori informazioni sulla creazione e organizzazione delle skills, vedi [Skills](/docs/it/skills).

<h3 id="keep-skills-discoverable">
  Mantieni le skills individuabili
</h3>

Con skills sparse in molte directory, l'elenco da cui Claude sceglie può crescere molto. Claude sceglie una skill leggendo il nome e la descrizione di ogni skill scoperta, e solo il contenuto completo della skill scelta si carica nel contesto. Questa sezione spiega come mantenere quell'elenco piccolo.

Quali skills sono in ambito dipende da dove avvii Claude:

* **Da una sottodirectory come `packages/api/`**: skills da quella directory, ogni parent fino alla radice del repository, e i livelli utente e azienda
* **Dalla radice del repository**: skills della radice, più skills da ogni sottodirectory che Claude tocca durante la sessione, che possono accumularsi nelle centinaia
* **Dopo aver aggiunto un sibling con [`--add-dir`](#grant-access-across-packages-or-repositories)**: le skills di quel sibling si caricano anche. L'impostazione `additionalDirectories` concede solo accesso ai file e non carica skills

I nomi si caricano sempre, ma [quando ce ne sono molti, alcune skills perdono completamente le loro descrizioni](/docs/it/skills#skill-descriptions-are-cut-short), il che può eliminare le parole chiave che Claude usa per decidere se una skill si applica. Mantieni le descrizioni brevi e inizia con parole che una richiesta conterrebbe, come "writing or modifying tests in `packages/api/`".

Per skills che molte directory condividono, come convenzioni PR o una checklist di deploy, posizionale nella `.claude/skills/` della radice del repository in modo che si carichino da qualsiasi directory di avvio. Quando le skills condivise hanno bisogno della propria cronologia delle versioni o devono funzionare tra repository, pacchettale come un [plugin](/docs/it/plugins/overview) invece. Le skills dei plugin usano uno spazio dei nomi `plugin-name:skill-name`, quindi non collidono mai con le skills per directory. Un team di piattaforma può versionalizzarle e aggiornarle in un unico posto.

Per trovare quali skills rimangono inutilizzate, abilita l'[esportatore di log](/docs/it/monitoring-usage) OpenTelemetry e imposta `OTEL_LOG_TOOL_DETAILS=1` in modo che i nomi delle skills siano registrati verbatim invece di essere redatti. L'evento [`skill_activated`](/docs/it/monitoring-usage#skill-activated-event) registra ogni invocazione nel suo attributo `skill.name`, e `invocation_trigger` registra se un comando, Claude, o una skill nidificata l'ha invocata, il che ti dice cosa consolidare o ritirare.

<h2 id="centralize-conventions-when-layering-stops-scaling">
  Centralizza le convenzioni quando la stratificazione smette di scalare
</h2>

I file CLAUDE.md per directory possono diventare difficili da governare man mano che il codebase cresce. Le convenzioni si allontanano, i file diventano obsoleti, e nessuno possiede la radice. Risolvere questo in genere spetta al team che mantiene la configurazione di Claude Code del repository piuttosto che a ogni sviluppatore che lavora nella sua area.

Sposta le convenzioni e il contenuto di riferimento fuori da CLAUDE.md sempre caricato e in meccanismi che si caricano su richiesta:

* [Skills](/docs/it/skills): materiale di riferimento che Claude carica solo quando rilevante per il compito
* [Plugins](/docs/it/plugins/overview): bundle versionati di skills, hooks e comandi che un team di piattaforma possiede centralmente
* [MCP servers](/docs/it/mcp): se la tua organizzazione già esegue una ricerca di codice o un indice RAG sul repository, esponilo come uno strumento MCP in modo che Claude lo interroghi invece di leggere i file direttamente

Vedi [impostazioni gestite dal server o gestite dall'endpoint](/docs/it/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings) per come i team di piattaforma possono applicare questi centralmente.

<h3 id="recommend-the-right-plugin-at-session-start">
  Consiglia il plugin giusto all'avvio della sessione
</h3>

Una volta che le convenzioni vivono nei plugin, un compagno di squadra che avvia Claude in una parte sconosciuta dell'albero non ha segnale su quale plugin i proprietari di quell'area mantengono. Un [`SessionStart` hook](/docs/it/hooks#sessionstart) può colmare quel divario, poiché Claude Code aggiunge il testo semplice che l'hook stampa su stdout al contesto di Claude prima del primo prompt.

Ad esempio, puoi scrivere uno script che legge la directory di avvio dall'[input dell'hook](/docs/it/hooks#common-input-fields), la cerca in una mappa percorso-a-plugin impegnata al repository, e stampa la raccomandazione per Claude da trasmettere nella sua prima risposta. Vedi [Automatizza azioni con hook](/docs/it/hooks-guide) per scrivere e registrare l'hook.

<h2 id="put-it-together">
  Mettere tutto insieme
</h2>

La configurazione combinata di seguito utilizza il layout monorepo. Gli stessi file funzionano per qualsiasi sottodirectory in un grande albero singolo. Il `.claude/settings.json` di ogni sottodirectory deve essere autonomo piuttosto che stratificato su un file radice.

L'esempio esegue il commit di `worktree`, `additionalDirectories` e le regole di negazione `Read` in `.claude/settings.json` in modo che ogni sviluppatore in `packages/api/` ottenga lo stesso accesso ai sibling, percorsi sparsi ed esclusioni. Il file di seguito è la configurazione per area impegnata per `packages/api/`:

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)"
    ]
  }
}
```

Poiché questa sessione inizia da `packages/api/`, i file CLAUDE.md dei pacchetti sibling sono già fuori portata, quindi `claudeMdExcludes` non è necessario qui. Aggiungetelo invece al `.claude/settings.local.json` della radice del repository se avviate anche sessioni dalla radice.

La voce `additionalDirectories` si applica quando avviate Claude da `packages/api/` direttamente. All'interno di un worktree creato da questa sessione, la directory di lavoro è la radice del worktree, quindi questo file di impostazioni non si carica. I pacchetti sibling sono già raggiungibili all'interno del worktree senza di esso, ma le regole di negazione hanno bisogno di una seconda copia nel `.claude/settings.json` della radice del repository in modo che le sessioni worktree le raccolgano, come la [nota sulle impostazioni worktree](#check-out-only-the-directories-you-need) descrive:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)"
    ]
  }
}
```

Dopo la configurazione, il repository ha questo layout:

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # deny rules for worktree sessions
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # worktree, additionalDirectories, deny rules
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

Con questa configurazione, avviando Claude da `packages/api/`:

* Carica il CLAUDE.md radice e `packages/api/CLAUDE.md`, salta `packages/web/CLAUDE.md`
* Può leggere e modificare file in `packages/api/` e `packages/shared/`
* Salta le letture dell'output di build sotto `dist/` e `build/` in `packages/api/`
* Ha la skill api-testing disponibile su richiesta
* Crea worktree contenenti `.claude/`, `packages/api/`, `packages/shared/` e file a livello radice, con le regole di negazione applicate nel worktree dal file di impostazioni radice

<h2 id="scope-and-plan-changes-that-span-packages">
  Ambito e pianifica i cambiamenti che si estendono tra pacchetti
</h2>

La configurazione sopra controlla cosa Claude vede. Quando un singolo cambiamento tocca diversi pacchetti, come aggiornare un tipo condiviso insieme a ogni sito di chiamata che lo usa, come scopi e sequenzi il compito influisce anche sul risultato.

Due tecniche aiutano a mantenere un cambiamento tra pacchetti coerente:

* **Dai a Claude l'intero cambiamento in una sessione**: consegnare la modifica condivisa e i suoi siti di chiamata insieme mantiene le decisioni dietro ogni modifica coerenti, piuttosto che derivarle di nuovo per pacchetto
* **Pianifica prima di modificare**: [pianifica prima](/docs/it/best-practices#explore-first-then-plan-then-code) in [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode), e Claude scrive il piano in un file. Una lunga sessione tra pacchetti [compatta il suo contesto](/docs/it/context-window#what-survives-compaction) lungo il percorso. Claude Code reiniezione il file del piano dopo ogni compattazione, quindi il piano sopravvive dove la cronologia della conversazione potrebbe non farlo

<h2 id="next-steps">
  Passaggi successivi
</h2>

Una volta che questa configurazione è in atto, puoi affinarla:

* Usa [hooks](/docs/it/hooks-guide) per eseguire linter o type-checker per directory dopo che Claude modifica i file
* Rivedi [Gestisci i costi in modo efficace](/docs/it/costs) per capire come la dimensione del codebase influisce sull'utilizzo dei token e come impostare i limiti di spesa prima di un rollout più ampio
* Leggi [Come Claude Code funziona nei codebase di grandi dimensioni](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start) sul blog di Claude per i modelli di rollout organizzativo e i modelli di proprietà che si trovano sopra la configurazione per repository su questa pagina
