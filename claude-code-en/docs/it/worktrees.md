> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eseguire sessioni parallele con worktrees

> Isolare sessioni parallele di Claude Code in worktrees git separati in modo che i cambiamenti non si scontrino. Copre il flag `--worktree`, l'isolamento dei subagent, `.worktreeinclude`, la pulizia e gli hook VCS non-git.

Un [git worktree](https://git-scm.com/docs/git-worktree) è una directory di lavoro separata con i propri file e branch, che condivide la stessa cronologia del repository e il remote come il vostro checkout principale. Eseguire ogni sessione di Claude Code nel proprio worktree significa che le modifiche in una sessione non toccheranno mai i file in un'altra, quindi una sessione può costruire una funzionalità mentre una seconda corregge un bug.

<Note>
  I worktree richiedono un repository git; per altri sistemi di controllo versione, [configurate gli hook per sostituire la logica git](#non-git-version-control). Nell'[app desktop](/docs/it/desktop#work-in-parallel-with-sessions), selezionate l'opzione **worktree** quando avviate una sessione per darle il proprio worktree.
</Note>

I worktree sono uno dei diversi modi per eseguire Claude in parallelo. Isolano le modifiche ai file. I [subagent](/docs/it/sub-agents) dividono il lavoro all'interno di una sessione, e la [messaggistica tra sessioni](/docs/it/cross-session-messaging) consente a Claude di passare i risultati tra le sessioni nei vostri worktree. Vedere [Eseguire agenti in parallelo](/docs/it/agents) per confrontare gli approcci, o saltare direttamente a [Isolare i subagent con worktrees](#isolate-subagents-with-worktrees) per utilizzare worktree e subagent insieme.

La maggior parte delle sessioni ha bisogno solo delle prime due sezioni: [avviare Claude in un worktree](#start-claude-in-a-worktree), quindi [pulire quando uscite](#clean-up-worktrees). Tornate al resto della pagina quando dovete [riprendere una sessione](#resume-a-worktree-session), [cambiare come vengono creati i worktree](#customize-worktree-creation), o [eseguire il debug di un errore](#troubleshooting).

<h2 id="start-claude-in-a-worktree">
  Avviare Claude in un worktree
</h2>

Passate `--worktree` o `-w` con un nome per creare un worktree isolato e avviare Claude in esso. Per impostazione predefinita, il worktree viene creato sotto `.claude/worktrees/<name>/` nella radice del vostro repository, su un nuovo branch denominato `worktree-<name>`:

```bash theme={null}
claude --worktree feature-auth
```

Eseguite il comando di nuovo con un nome diverso in un altro terminale per avviare una seconda sessione isolata. Se omettete il nome, Claude ne genera uno come `bright-running-fox`.

Le esecuzioni interattive richiedono [fiducia dell'area di lavoro](/docs/it/security): se non avete mai eseguito Claude nella directory prima, eseguite `claude` una volta lì per accettare la finestra di dialogo di fiducia, oppure `--worktree` esce con un errore che vi chiede di farlo. Le esecuzioni non interattive con `-p` saltano il controllo di fiducia, quindi `claude -p --worktree` procede senza di esso.

<Tip>
  Aggiungete `.claude/worktrees/` al vostro `.gitignore` in modo che i contenuti del worktree non appaiano come file non tracciati nel vostro checkout principale.
</Tip>

<h3 id="set-up-the-worktree-environment">
  Configurare l'ambiente del worktree
</h3>

Un worktree è un checkout fresco, quindi inizializzate il vostro ambiente di sviluppo lì: chiedete a Claude di installare le dipendenze, oppure eseguite il setup del vostro progetto voi stessi nella directory del worktree sotto `.claude/worktrees/`. Per portare file gitignored come `.env` in ogni nuovo worktree automaticamente, aggiungete un [file `.worktreeinclude`](#copy-gitignored-files-into-worktrees).

<h3 id="ask-claude-to-create-a-worktree">
  Chiedere a Claude di creare un worktree
</h3>

Potete anche chiedere a Claude di "lavorare in un worktree" durante una sessione, e creerà uno con lo strumento [`EnterWorktree`](/docs/it/tools-reference). Una volta in un worktree, Claude può passare direttamente a un altro sotto `.claude/worktrees/` chiamando `EnterWorktree` con il percorso di destinazione; il worktree precedente rimane su disco intatto.

Quando Claude entra in un percorso al di fuori della directory `.claude/worktrees/` del repository, Claude Code chiede prima la vostra approvazione, perché lo spostamento porta la directory di lavoro della sessione, l'accesso in scrittura e la configurazione del progetto come `CLAUDE.md` e le impostazioni in quella posizione. Una regola di [permesso](/docs/it/permissions) `EnterWorktree` o la scelta di "non chiedere di nuovo" non sopprime questo prompt; solo la modalità `bypassPermissions` lo salta. Prima della v2.1.206, Claude poteva entrare in qualsiasi percorso di worktree esistente senza chiedere.

<Note>
  **I percorsi degli hook non seguono il worktree.** Dopo che Claude entra in un worktree, Claude Code mantiene `${CLAUDE_PROJECT_DIR}` nei vostri [hook](/docs/it/hooks#reference-scripts-by-path) dove era e passa il percorso del worktree a loro in un modo diverso:

  * **`${CLAUDE_PROJECT_DIR}` rimane fermo**: punta ancora alla radice del progetto dove la sessione è iniziata, quindi un comando hook come `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` esegue ancora lo script nel checkout principale.
  * **`cwd` segue Claude**: il campo `cwd` nel [JSON di input](/docs/it/hooks#common-input-fields) dell'hook è la radice del worktree, e si sposta di nuovo quando Claude esegue `cd`. Leggetelo quando un hook ha bisogno del percorso del worktree.
</Note>

<h2 id="clean-up-worktrees">
  Pulire i worktree
</h2>

Quando uscite da una sessione di worktree interattiva, Claude controlla il worktree per il lavoro che la rimozione eliminerebbe: file modificati o non tracciati, lavoro non committato all'interno di submodule estratti, e nuovi commit.

* **Il worktree è pulito**: per una sessione senza nome, Claude rimuove il worktree e il suo branch automaticamente. Una sessione [denominata](/docs/it/sessions#name-your-sessions) vi chiede prima in modo da poter mantenere il worktree per dopo
* **Il worktree ha lavoro in esso**: Claude vi chiede di mantenere o rimuovere il worktree. Mantenere preserva la directory e il branch in modo da poter tornare in seguito. Rimuovere elimina la directory del worktree e il suo branch, insieme a tutto il lavoro in essi
* **Lo stato del worktree non può essere verificato**: quando Claude Code non riesce a contare i cambiamenti del worktree o non riesce a ispezionare i checkout dei suoi submodule, vi chiede piuttosto che rimuovere il worktree automaticamente. Il prompt nomina ciò che non ha potuto controllare

Le esecuzioni non interattive con `-p` non hanno un prompt di uscita, quindi Claude non pulisce i loro worktree, e Claude Code lascia il blocco che ha preso su ognuno al momento della creazione in posizione fino a quando una [scansione di blocco stale](#clean-up-subagent-and-background-session-worktrees) successiva della sessione lo rilascia. Per rimuoverne uno, eseguite `git worktree remove`; se git rifiuta perché il worktree è bloccato, eseguite prima `git worktree unlock` su di esso.

Su Windows, rimuovere un worktree non elimina i file al di fuori di esso. Se una cartella all'interno del worktree è un collegamento a qualcos'altro, come una junction NTFS o un symlink di directory, Claude Code elimina solo il collegamento e mantiene la cartella a cui punta. Prima della v2.1.205, rimuovere un worktree con un collegamento annidato in una sottodirectory potrebbe eliminare la cartella a cui puntava.

<h2 id="resume-a-worktree-session">
  Riprendere una sessione di worktree
</h2>

Quando riprendete una sessione che era all'interno di un worktree, Claude Code restituisce la sessione a quel worktree. Questo vale per le riprese interattive, per `--continue` e `--resume` in [modalità non interattiva](/docs/it/headless) con `-p`, e per l'Agent SDK. Di nuovo all'interno del worktree, Claude può ancora uscirne con lo strumento [`ExitWorktree`](/docs/it/tools-reference).

Prima di restituire la sessione al suo worktree, Claude Code verifica che il worktree sia ancora un checkout separato da quello principale, e rifiuta di rientrare in un worktree che fallisce il controllo. Per un git worktree, il controllo legge i suoi metadati git. Un worktree senza metadati git, come uno che un hook [`WorktreeCreate`](#non-git-version-control) ha creato, può passare il controllo; i casi che Claude Code ancora rifiuta sono elencati con i loro recuperi sotto [Claude Code rifiuta di usare un worktree](#claude-code-refuses-to-use-a-worktree). Per i messaggi e come recuperare da ognuno, vedere [La sessione riprende al di fuori del suo worktree](#the-session-resumes-outside-its-worktree).

Dove lanciate da, e come riprendete, cambiano cosa Claude Code rientra:

* **Directory di lancio**: riprendete dal checkout principale o da un'altra directory del repository. Claude Code rientra in un worktree che ha creato con git sotto `.claude/worktrees/` anche quando lanciate da dentro di esso. Quando lanciate da dentro qualsiasi altro worktree, Claude Code rientra in esso solo se può garantirlo da lì: un worktree che è il suo proprio repository, uno senza metadati git, o un lancio da una sottodirectory di un worktree che avete creato con `git worktree add` rifiuta, quindi lanciate quelli dal checkout principale.
* **`--fork-session`**: la sessione biforcata inizia nella directory da cui avete lanciato Claude, e Claude Code lascia il worktree della sessione originale intatto.
* **Worktree eliminato**: se la directory del worktree non esiste più, Claude Code riprende la sessione nella directory da cui avete lanciato Claude. Vi dice che il worktree è scomparso e cancella il binding del worktree della sessione.

<Note>
  Prima della v2.1.212, una ripresa non interattiva rimaneva nella directory di avvio e `ExitWorktree` segnalava che non c'era una sessione di worktree attiva da cui uscire.
</Note>

Quando Claude entra o esce da un worktree che Claude Code ha creato con git, la trascrizione segue: Claude Code registra la sessione sotto la nuova directory di lavoro della sessione, nello stesso modo in cui [`/cd`](/docs/it/commands) lo fa, quindi `/desktop` e `--resume` la trovano lì. Uscire la sposta di nuovo nello stesso modo. Un worktree creato da un hook [`WorktreeCreate`](#non-git-version-control) mantiene la sua trascrizione nella directory di lancio. Richiede Claude Code v2.1.198 o successivo.

<h2 id="how-claude-code-enforces-isolation">
  Come Claude Code applica l'isolamento
</h2>

Mentre una sessione è isolata in un worktree, Claude Code blocca le chiamate di strumento che i controlli di seguito definiscono. Le stesse regole si applicano se avete avviato la sessione con `--worktree`, Claude è entrato in un worktree con `EnterWorktree`, o avete ripreso una sessione di worktree.

Lo stesso enforcement copre ogni subagent che Claude genera dalla sessione isolata. Si applica se la sessione è interattiva o viene eseguita in [background](/docs/it/agent-view#how-file-edits-are-isolated). I [subagent che vengono eseguiti nel loro proprio worktree](#isolate-subagents-with-worktrees) portano gli stessi controlli. La loro cronologia delle versioni è sotto [Scrivere file di subagent](/docs/it/sub-agents#write-subagent-files).

Claude Code applica quattro controlli:

* **Modifiche ai file**: Claude Code blocca un `Edit`, `Write`, o `NotebookEdit` che ha come target un percorso nel checkout principale.
* **Directory di lavoro del comando**: Claude Code blocca un comando Bash, PowerShell, o Monitor la cui directory di lavoro si risolve nel checkout principale, o la cui directory di lavoro non può verificare che rimanga al di fuori di esso.
* **Reindirizzamenti git**: Claude Code blocca un comando Bash o Monitor che reindirizza git nel checkout principale. Il reindirizzamento può provenire attraverso `git -C`, `--git-dir`, una variabile `GIT_DIR` o `GIT_WORK_TREE`, o un `cd` nel checkout principale prima di eseguire git.
* **Forma del comando**: Claude Code blocca un comando Bash o Monitor quando non può verificare dal testo del comando che qualsiasi git che il comando esegue rimane all'interno del worktree. Questo accade, ad esempio, quando il nome del comando è calcolato a runtime, quando la sintassi non può essere analizzata, o quando un'espansione come `${!name}` o `${ command; }` potrebbe eseguire un comando che il testo non esplicita. Claude Code dice a Claude come riscrivere il comando rifiutato, come dividerlo in comandi semplici e separati. Non potete disattivare questo controllo.

I controlli si applicano al repository da cui avete lanciato Claude Code. Coprono anche il checkout principale da cui un worktree collegato è collegato. Per i comandi PowerShell, Claude Code applica solo il controllo della directory di lavoro.

Claude vede ogni rifiuto come un errore di strumento che nomina il worktree e dice come procedere. Per un comando rifiutato, consultate [cosa significa il messaggio di rifiuto e come cancellarlo](/docs/it/errors#command-blocked-by-the-worktree-isolation-checks).

<h2 id="isolate-subagents-with-worktrees">
  Isolare i subagent con worktree
</h2>

I subagent possono eseguire nei loro propri worktree in modo che le modifiche parallele non entrino in conflitto. Chiedete a Claude di "usare worktree per i vostri agenti", o rendete l'isolamento permanente per un [subagent personalizzato](/docs/it/sub-agents#supported-frontmatter-fields) aggiungendo `isolation: worktree` al suo frontmatter.

Questo subagent in `.claude/agents/` viene sempre eseguito nel suo proprio worktree:

```markdown theme={null}
---
name: refactorer
description: Applies mechanical refactors across many files
isolation: worktree
---

Apply the requested refactor across every affected file, then run the tests
and report the results.
```

Ogni subagent ottiene un worktree temporaneo che Claude Code rimuove automaticamente quando il subagent finisce senza modifiche; un worktree con modifiche rimane su disco fino a quando la [scansione periodica di seguito](#clean-up-subagent-and-background-session-worktrees) può rimuoverlo senza perdere lavoro.

I worktree dei subagent utilizzano lo stesso [branch di base](#choose-the-base-branch) di `--worktree`, quindi si diramano dal branch predefinito del vostro repository a meno che `worktree.baseRef` non sia impostato su `"head"`.

<h3 id="clean-up-subagent-and-background-session-worktrees">
  Pulire i worktree dei subagent e della sessione in background
</h3>

Claude Code esegue una scansione periodica che rimuove i worktree che Claude ha creato per i subagent e le [sessioni in background](/docs/it/agent-view#how-file-edits-are-isolated) una volta che sono più vecchi della vostra impostazione [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays), seguendo le [regole della scansione di conservazione](/docs/it/claude-directory#cleaned-up-automatically).

Quando [inviate in background](/docs/it/agent-view#send-the-session-to-the-background) una sessione `--worktree`, il suo worktree diventa un worktree di sessione in background che la scansione può rimuovere. La scansione lascia un worktree in posizione in questi casi:

* Il worktree contiene ancora lavoro: file modificati o non tracciati, o commit non spinti.
* Un submodule estratto nel worktree contiene file modificati o non tracciati, oppure Claude Code non può ispezionare i submodule del worktree. Questo controllo richiede Claude Code v2.1.274 o successivo.
* Uno dei [quattro casi che bloccano anche la creazione del worktree](#git-lfs-content-is-missing-from-a-worktree-claude-code-created) si applica: Claude Code non può determinare quali driver di filtro la configurazione del repository definisce, o trova un'impostazione lì che non può disattivare.
* Il worktree appartiene a una sessione `--worktree` che non avete inviato in background, indipendentemente dalla sua età.
* Avete creato il worktree voi stessi con `git worktree add`, anche se poi avete eseguito una sessione `--worktree <name>` in esso e l'avete inviata in background.

Claude Code scrive un marcatore nei metadati git di ogni worktree che crea con git, e la scansione mantiene qualsiasi worktree senza uno, incluso un worktree che un hook [`WorktreeCreate`](#non-git-version-control) ha creato. Prima della v2.1.246, la scansione non controllava il marcatore, e poteva rimuovere un worktree che avete creato voi stessi quando un vecchio record di sessione in background puntava a esso.

Mentre un agente è in esecuzione, Claude Code mantiene un `git worktree lock` sul suo worktree in modo che la pulizia concorrente non possa rimuoverlo, e rilascia il blocco quando l'agente finisce. Claude Code mantiene lo stesso blocco sul worktree che ha creato per una sessione inviata in background mentre la sessione viene eseguita, quindi la scansione lascia il worktree in posizione e `git worktree remove` rifiuta di rimuoverlo.

La scansione rilascia anche un blocco che Claude Code ha impostato per una sessione il cui processo è uscito, quindi una sessione in background uccisa non lascia il suo worktree permanentemente bloccato. La scansione non rilascia mai un blocco che avete impostato voi stessi con `git worktree lock`. Prima della v2.1.210, un blocco lasciato da una sessione uccisa rimaneva in posizione fino a quando non eseguivate `git worktree unlock`.

Per pulire un worktree che la scansione mantiene, eseguite `git worktree remove`, aggiungendo `--force` se il worktree ha modifiche non committate o file non tracciati. Se git rifiuta perché il worktree è bloccato, eseguite prima `git worktree unlock` su di esso.

<h2 id="customize-worktree-creation">
  Personalizzare la creazione del worktree
</h2>

I valori predefiniti di Claude Code per la creazione di worktree coprono la maggior parte delle sessioni: li crea sotto `.claude/worktrees/`, li dirama dal branch predefinito del vostro repository, e controlla solo i file tracciati. Le opzioni in questa sezione cambiano quei valori predefiniti.

<h3 id="choose-the-base-branch">
  Scegliere il branch di base
</h3>

I nuovi worktree si diramano dal branch predefinito del repository, quindi la maggior parte delle sessioni non ha bisogno di questa impostazione. Impostate `worktree.baseRef` nelle [impostazioni](/docs/it/settings-reference#worktree) per diramarvisi dal vostro lavoro corrente. L'impostazione accetta due valori:

* `"fresh"` (predefinito): dirama dal branch predefinito del repository sul remote, solitamente `main`, quindi il worktree inizia da un albero pulito che corrisponde al remote.
* `"head"`: dirama dal vostro `HEAD` locale corrente, quindi il worktree porta i vostri commit non spinti e lo stato del feature-branch. Usate questo quando isolate i subagent che devono operare su lavori in corso. All'interno di un worktree, `"head"` si risolve in `HEAD` di quel worktree, non nel checkout principale.

Non potete impostare `worktree.baseRef` a un nome di branch. Per avviare un worktree da un branch esistente specifico, [createlo con git direttamente](#manage-worktrees-manually).

Per una base `"fresh"`, Claude Code mantiene `origin/HEAD` corrente: quando il repository non è stato recuperato negli ultimi 24 ore, recupera il branch predefinito, limitato a cinque secondi, e utilizza il ref memorizzato nella cache locale se il recupero fallisce. Se nessun remote è configurato, o `origin/HEAD` non è memorizzato nella cache localmente e non può essere recuperato, il worktree ricade al vostro `HEAD` locale corrente. Prima della v2.1.208, un worktree fresco utilizzava qualsiasi `origin/HEAD` fosse già memorizzato nella cache localmente.

Questo esempio fa sì che ogni nuovo worktree si dirama dal vostro lavoro corrente:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

<h3 id="branch-from-a-pull-request">
  Diramazione da una pull request
</h3>

Per diramarvisi da una pull request o merge request specifica, passate a `--worktree` il numero con prefisso `#`, un URL di pull request di GitHub, o un URL di merge request di GitLab come `https://gitlab.com/group/repo/-/merge_requests/123`. Claude Code recupera il commit head di quel cambiamento da `origin` e crea il worktree in `.claude/worktrees/pr-<number>`. Quotate l'argomento in modo che la vostra shell non tratti `#` come l'inizio di un commento:

```bash theme={null}
claude --worktree "#1234"
```

Claude Code legge solo il numero dall'URL. Recupera sempre da `origin` del vostro repository, e sceglie il percorso di recupero dall'host di `origin`:

* **github.com**: recupera `pull/<number>/head`
* **gitlab.com**: recupera `merge-requests/<number>/head`
* **GitHub Enterprise, GitLab auto-gestito, o qualsiasi altro host**: prova prima `pull/<number>/head`, poi `merge-requests/<number>/head`

Prima della v2.1.233, Claude Code accettava solo `#<number>` e URL di pull request in stile GitHub per `--worktree`, e recuperava sempre `pull/<number>/head`.

<h3 id="copy-gitignored-files-into-worktrees">
  Copiare file gitignored nei worktree
</h3>

Un worktree è un checkout fresco, quindi i file non tracciati come `.env` o `.env.local` dal vostro repository principale non sono presenti. Per copiarli automaticamente quando Claude crea un worktree, aggiungete un file `.worktreeinclude` alla radice del vostro progetto.

Il file utilizza la sintassi `.gitignore`. Solo i file che corrispondono a un pattern e sono anche gitignored vengono copiati, quindi i file tracciati non vengono mai duplicati.

Se scrivete un pattern che inizia con `**/` e i file che volete sono all'interno di una directory che è gitignored nel suo insieme, Claude Code li copia solo quando quella directory stessa corrisponde al pattern, o quando il primo nome dopo `**/` è uno dei nomi nel percorso della directory. Ad esempio, se scrivete `**/.claude/skills/*.md`, quel primo nome è `.claude`, quindi Claude Code copia i file corrispondenti da una directory `.claude/` ignorata. Per copiare file da una directory ignorata che un pattern `**/` non raggiunge, nominate la directory nel pattern: scrivete `vendor/**/config.json` piuttosto che `**/config.json`. Prima della v2.1.239, Claude Code copiava file da una directory completamente ignorata per un pattern `**/` solo quando la directory stessa corrispondeva al pattern.

Questo `.worktreeinclude` copia due file env e una configurazione di segreti in ogni nuovo worktree:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Questo si applica a ogni worktree che Claude Code crea con git: worktree `--worktree`, [worktree dei subagent](#isolate-subagents-with-worktrees), e sessioni parallele nell'[app desktop](/docs/it/desktop#work-in-parallel-with-sessions). Con un hook [`WorktreeCreate`](#non-git-version-control), copiate i file all'interno dello script dell'hook.

<h3 id="reuse-a-worktree-name">
  Riutilizzare un nome di worktree
</h3>

Passare a `--worktree` un nome la cui directory esiste già apre quel worktree esistente invece di crearne uno nuovo.

Con il `"fresh"` [base](#choose-the-base-branch) predefinito, un worktree riaperto si ripristina al branch predefinito del repository invece di continuare al suo vecchio tip quando tutte le seguenti condizioni sono soddisfatte:

* Non ha modifiche non committate o file non tracciati.
* È ancora sul branch che Claude Code ha creato per esso.
* Non ha commit propri, o la sua pull request o merge request è stata unita e il suo branch remoto è stato eliminato.

Claude Code rileva il caso unito dallo stato git da solo: il branch remoto a cui il worktree ha spinto non esiste più, e ogni commit nel worktree è già sul branch predefinito.

In ogni altro caso, Claude Code riapre il worktree al suo vecchio tip:

* Il worktree fallisce una qualsiasi delle condizioni.
* Claude Code non può verificare lo stato del worktree.
* `worktree.baseRef` è `"head"`.
* Il nome è un riferimento a una pull request o merge request.

Prima della v2.1.208, quando riutilizzavate un nome, Claude Code riapriva sempre il vecchio worktree al suo vecchio tip.

<h3 id="replace-worktree-creation-with-a-hook">
  Sostituire la creazione del worktree con un hook
</h3>

Configurate un hook [`WorktreeCreate`](/docs/it/hooks#worktreecreate) per sostituire completamente la logica predefinita di `git worktree`, incluso il posizionamento dei worktree da qualche parte diverso da `.claude/worktrees/`. Per un esempio completo, vedere [Controllo versione non-git](#non-git-version-control).

<h2 id="what-worktrees-share-with-the-main-checkout">
  Cosa i worktree condividono con il checkout principale
</h2>

Un worktree ottiene i suoi propri file e branch, ma condivide quanto segue con il checkout principale:

* **La directory `.git` del repository**: i comandi git in un worktree scrivono nella directory `.git` condivisa del repository principale, e il [sandboxing](/docs/it/sandboxing#filesystem-isolation) consente quelle scritture, quindi comandi come `git commit` funzionano da dentro un worktree con la sandbox abilitata.
* **Plugin**: i plugin installati a [ambito del progetto](/docs/it/plugins/loading#find-where-a-plugin-is-enabled) dal checkout principale caricano anche nei worktree dello stesso repository, quindi non è necessario reinstallarli per ogni worktree. Richiede Claude Code v2.1.200 o successivo.
* **Approvazioni di permesso**: scegliere "Sì, e non chiedere di nuovo" per un comando Bash in una sessione di worktree salva la regola nel `.claude/settings.local.json` del checkout principale, quindi si applica nel checkout principale e in ogni altro worktree del repository, e sopravvive alla rimozione del worktree. Su Windows e negli altri casi in cui Claude Code [non utilizza la radice del repository](/docs/it/settings#where-claude-code-looks-for-each-file), la regola rimane con quel worktree. Prima della v2.1.211, un'approvazione concessa in un worktree era salvata all'interno di quel worktree, non si applicava altrove, e era persa quando il worktree era rimosso. Vedere [dove le approvazioni sono salvate](/docs/it/permissions#permission-system).
* **Skill, agenti e comandi non tracciati**: quando il checkout del worktree non ha una directory `.claude/skills` alla sua radice, ad esempio perché il vostro `.claude/skills` è gitignored, Claude Code carica le [skill del progetto](/docs/it/skills#where-skills-live) del checkout principale nella sessione del worktree. In un worktree con la sua propria directory `.claude/skills`, carica solo quella copia.

  Lo stesso read-through copre `.claude/agents` e `.claude/commands`. Per le skill, il read-through richiede Claude Code v2.1.277 o successivo.

Tutti questi si applicano indipendentemente dal fatto che creiate il worktree con `--worktree`, con `git worktree add`, o attraverso l'[app desktop](/docs/it/desktop#work-in-parallel-with-sessions).

<h2 id="manage-worktrees-manually">
  Gestire i worktree manualmente
</h2>

Create i worktree con Git direttamente quando dovete controllare un branch esistente specifico o posizionare il worktree al di fuori del repository.

Create un worktree su un nuovo branch:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Create un worktree da un branch esistente, sostituendo `fix-issue-456` con un branch che esiste già nel vostro repository:

```bash theme={null}
git worktree add ../project-bugfix fix-issue-456
```

Avviate Claude nel worktree:

```bash theme={null}
cd ../project-feature-a
claude
```

Elencate i vostri worktree:

```bash theme={null}
git worktree list
```

Rimuovete uno quando avete finito:

```bash theme={null}
git worktree remove ../project-feature-a
```

Vedere la [documentazione di Git worktree](https://git-scm.com/docs/git-worktree) per il riferimento completo dei comandi.

<h2 id="non-git-version-control">
  Controllo versione non-git
</h2>

L'isolamento dei worktree utilizza git per impostazione predefinita. Per SVN, Perforce, Mercurial, o altri sistemi, configurate gli hook [`WorktreeCreate` e `WorktreeRemove`](/docs/it/hooks#worktreecreate) per fornire logica di creazione e pulizia personalizzata. Poiché l'hook sostituisce il comportamento predefinito di git, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) non viene elaborato quando utilizzate `--worktree`. Copiate i file di configurazione locali all'interno dello script dell'hook.

Questo hook `WorktreeCreate` legge il nome del worktree dal JSON su stdin con `jq`, controlla una copia di lavoro SVN fresca, e stampa il percorso della directory in modo che Claude Code possa usarlo come directory di lavoro della sessione. Aggiungete la configurazione al vostro [`settings.json`](/docs/it/settings#where-settings-live):

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Abbinarlo a un hook `WorktreeRemove` per pulire quando la sessione termina. Vedere il [riferimento degli hook](/docs/it/hooks#worktreecreate) per lo schema di input e un esempio di rimozione.

Un hook `WorktreeCreate` vi consente anche di eseguire [`/batch`](/docs/it/commands#all-commands) al di fuori di un repository git. Ogni subagent `/batch` pubblica quindi la sua modifica con i comandi di controllo versione del vostro progetto e, quando non riesce ad aprire una pull request, segnala invece ciò che ha pubblicato. L'esecuzione di `/batch` al di fuori di un repository git richiede Claude Code v2.1.281 o versione successiva.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Claude Code segnala gli errori di seguito quando crea un worktree, entra in uno all'avvio, o restituisce una sessione ripresa a uno.

<h3 id="claude-code-can’t-enter-the-worktree-at-startup">
  Claude Code non può entrare nel worktree all'avvio
</h3>

Quando Claude Code non può entrare nella directory del worktree all'avvio, stampa un errore che nomina il percorso ed esce con codice 1. Questo può accadere quando un hook [`WorktreeCreate`](/docs/it/hooks#worktreecreate) stampa qualcosa di diverso dalla directory che ha creato, o quando la directory è stata eliminata dopo che è stata configurata.

<h3 id="worktree-creation-fails-on-a-symlinked-path">
  La creazione del worktree fallisce su un percorso symlinked
</h3>

Claude Code rifiuta di creare un worktree quando `.claude`, `.claude/worktrees`, o la directory del worktree stesso è un symlink, e l'errore nomina il percorso symlinked. Rimuovete il symlink e riprovate. Prima della v2.1.212, se il repository conteneva già un symlink committato in uno di quei percorsi, la creazione del worktree lo seguiva e poteva creare file al di fuori del repository.

<h3 id="git-lfs-content-is-missing-from-a-worktree-claude-code-created">
  I file Git LFS sono file puntatore in un worktree che Claude Code ha creato
</h3>

Se configurate [Git LFS](https://git-lfs.com) con `git lfs install --local`, un worktree che Claude Code crea contiene file puntatore LFS invece dei file reali. Il flag `--local` scrive il filtro LFS nel `.git/config` del repository stesso piuttosto che nella vostra configurazione git globale. Un semplice `git lfs install` scrive nella vostra configurazione globale e non è interessato. Lo stesso si applica a qualsiasi altro [driver di filtro](https://git-scm.com/docs/gitattributes) definito nel config del repository stesso.

Claude Code salta i driver di filtro del repository stesso quando crea un worktree perché un driver di filtro è un comando shell, e qualsiasi cosa che possa scrivere nel repository, incluso Claude, potrebbe averlo messo lì. Prima della v2.1.247, Claude Code eseguiva quei driver durante la creazione del worktree.

Per ottenere i file reali, eseguite `git lfs pull` all'interno del worktree.

In quattro rari casi, Claude Code non crea alcun worktree: non può dire quali driver di filtro la configurazione del repository definisce, o trova un'impostazione lì che non può disattivare. Abbinate l'errore alla sua correzione:

* **`Could not read the repository git config to neutralize filter drivers`**: Claude Code non poteva leggere il `.git/config` del repository, ad esempio a causa dei suoi permessi. Riparate quello e riprovate.
* **`The repository git config defines a filter driver whose name cannot be neutralized (contains "=" or a newline)`**: rinominate o rimuovete quel driver di filtro in `.git/config` e riprovate.
* **`The repository git config has a conditional include (includeIf)`**: spostate le impostazioni che `includeIf` in `.git/config` estrae direttamente in quel file, rimuovete `includeIf`, e riprovate. Un `includeIf` nella vostra configurazione git globale non attiva questo.
* **`Git was not run: the repository's own git config sets <key>`**: il messaggio nomina una chiave che punta Git LFS a un programma da eseguire, come `lfs.customtransfer.<name>.path` o `lfs.standalonetransferagent`. Se quell'impostazione è vostra, spostatela nella vostra configurazione git globale. Se non la riconoscete, rimuovetela dalla configurazione git del repository, poiché uno strumento o checkout che non fidate potrebbe averla scritta. Riprovate una volta che la chiave è scomparsa dalla configurazione del repository.

<h3 id="claude-code-refuses-to-use-a-worktree">
  Claude Code rifiuta di usare un worktree
</h3>

Un errore che inizia con `Refusing to use <path> as an isolation worktree` significa che Claude Code ha controllato l'identità git della directory prima di adottarla come checkout isolato di una sessione o subagent, e l'ha rifiutata. Il controllo viene eseguito se Claude Code sta creando il worktree, entrando in uno esistente, o riutilizzandone uno da un'esecuzione precedente.

Nella maggior parte dei casi il resto del messaggio dice che i metadati git della directory si risolvono nel checkout principale: ad esempio, il suo file `.git` punta alla directory `.git` del repository principale stesso, o git risolve il suo albero di lavoro al checkout principale attraverso un reindirizzamento `core.worktree`. Da una tale directory, un comando git ordinario come `git reset --hard` agirebbe sul checkout principale invece che sul worktree. Claude Code rifiuta anche quando la directory ha una voce `.git` che non può leggere, piuttosto che assumere che il worktree sia sicuro.

Una directory senza metadati git affatto, come una che il vostro hook [`WorktreeCreate`](#non-git-version-control) crea, passa il controllo solo quando nessun repository git la contiene. Se l'hook crea la directory all'interno di un repository, git la risolve al checkout di quel repository e Claude Code la rifiuta con il messaggio `git resolves its working tree to`, quindi fate in modo che l'hook crei le sue directory al di fuori di qualsiasi repository.

Claude Code lascia la directory rifiutata in posizione, poiché potrebbe contenere lavoro. Abbinate il messaggio al suo recupero, se segue `Refusing to use <path>` o appare in un [messaggio di ripresa](#the-session-resumes-outside-its-worktree); alcuni finali si verificano solo nei messaggi di ripresa:

* **Dice `launch from the parent checkout` o `Run the resume from the project checkout`**: avete lanciato Claude Code da dentro il worktree. Lanciate dal checkout principale; il worktree non ha bisogno di ricreazione.
* **Dice `it cannot be resumed or re-entered`**: nulla in questa sessione garantisce il worktree da dove avete lanciato. Ricreate; la directory e il suo lavoro rimangono su disco per il recupero manuale, e quando il worktree ha un checkout genitore, riprendere da lì funziona anche.
* **Dice `it contains the protected checkout`**: la directory rifiutata è un genitore del vostro checkout principale, come la vostra home directory. Non la eliminate. Cambiate il percorso del worktree, come il percorso che il vostro hook `WorktreeCreate` restituisce o il target di `EnterWorktree`, in modo che il worktree non contenga il checkout.
* **Dice `the protected checkout <path> has a .git entry that could not be examined` o `has git metadata that could not be resolved`**: il problema è nei metadati git del checkout principale, non nel worktree. Non eliminate il worktree, e ignorate il consiglio finale del messaggio di ricrearlo, che non si applica a questi due finali. Riparate il checkout principale, ad esempio un problema di permessi o un rifiuto git `dubious ownership` sul suo `.git`, e riprovate.
* **Dice `its recorded path has a network spelling`**: Claude Code non riprende mai in un worktree a un percorso di rete. Ricreate il worktree a un percorso locale.
* **Qualsiasi altro finale**: il messaggio nomina il problema e la sua correzione, come rimuovere un reindirizzamento `core.worktree` o ricreate il worktree; seguitelo. Prima di eliminare una directory il cui messaggio dice che la sua identità git non potrebbe essere verificata, affrontate la causa nominata prima, ad esempio un collegamento simbolico nel percorso del worktree o git stesso che non riesce a eseguire, poiché la directory potrebbe essere sana. Quando ricreate, salvate i cambiamenti di cui avete bisogno dalla vecchia directory prima; rimane su disco.

<h3 id="the-session-resumes-outside-its-worktree">
  La sessione riprende al di fuori del suo worktree
</h3>

Quando riprendete una sessione in modo interattivo e Claude Code non può restituirla al suo worktree, Claude Code lo dice con uno dei messaggi di seguito. Quando Claude Code cancella il binding del worktree, lo registra nella trascrizione della sessione. Se [sopprimete le scritture della trascrizione](/docs/it/sessions#where-transcripts-are-stored), il messaggio dice invece che il binding non potrebbe essere cancellato e che Claude Code ricontrollerà il worktree su una ripresa successiva.

| Il messaggio inizia con                           | Cosa è successo e cosa fare                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Your worktree <path> no longer exists`           | La directory del worktree è stata rimossa. La sessione continua nella directory corrente senza isolamento, e Claude Code cancella il binding del worktree. Nessuna azione necessaria.                                                                                                                                                                                                                                                                                                                 |
| `Could not verify your worktree <path> this time` | Claude Code non poteva verificare il worktree, solitamente per una ragione transitoria; il binding è mantenuto, e la sessione continua nella directory corrente senza isolamento. Riprendete di nuovo per riprovare; se continua a succedere, entrate nel worktree in una nuova sessione e abbinate il messaggio di rifiuto sotto [Claude Code rifiuta di usare un worktree](#claude-code-refuses-to-use-a-worktree), che può nominare i metadati del checkout principale piuttosto che del worktree. |
| `Did not re-enter your worktree <path>`           | Claude Code ha rifiutato il binding del worktree come non sicuro; cancella il binding e la sessione continua senza isolamento. Il messaggio include il rifiuto specifico: abbiatelo sotto [Claude Code rifiuta di usare un worktree](#claude-code-refuses-to-use-a-worktree), poiché la correzione è ricreazione per alcuni rifiuti e un cambio di percorso per altri.                                                                                                                                |
| `Could not re-enter your worktree <path>`         | Claude Code non poteva garantire il worktree da dove avete lanciato, più comunemente perché avete lanciato da dentro di esso; il binding è mantenuto. Il resto del messaggio nomina la correzione; abbiatelo sotto [Claude Code rifiuta di usare un worktree](#claude-code-refuses-to-use-a-worktree).                                                                                                                                                                                                |

In [modalità non interattiva](/docs/it/headless) con `-p`, e su riprese che l'[Agent SDK](/docs/it/agent-sdk/sessions) esegue, Claude Code interrompe la ripresa con un errore stderr per ogni rifiuto tranne un worktree scomparso, invece di continuare senza isolamento.

Con `--output-format stream-json`, il rifiuto arriva anche su stdout come un messaggio `result` con sottotipo `error_during_execution` il cui array `errors` porta lo stesso testo, quindi un'applicazione Agent SDK riceve il motivo piuttosto che solo un'uscita non zero. Prima della v2.1.260, un rifiuto di ripresa del worktree non produceva alcun messaggio `result`.

I messaggi assumono forme diverse dai messaggi interattivi nella tabella:

* `Error: cannot resume into worktree <path>: ...This session was not started.` per un rifiuto che la tabella mostra come `Did not re-enter`. Claude Code cancella il binding del worktree prima di uscire, e l'errore lo dice; la prossima volta che riprendete la conversazione, la sessione continua nella directory corrente senza isolamento del worktree. Prima della v2.1.260, Claude Code non scriveva il binding cancellato, quindi ogni ritentativo della stessa ripresa falliva con lo stesso errore.

  Se [sopprimete le scritture della trascrizione](/docs/it/sessions#where-transcripts-are-stored), il clear non può essere salvato. L'errore dice allora che lo stesso comando sarà rifiutato di nuovo, e nomina `--fork-session` e l'avvio di una nuova conversazione come modi per continuare senza il worktree.
* `Error: could not verify worktree <path> for this resume, so the resume was aborted...` per `Could not verify`
* `Error: ...The worktree binding is kept.` per `Could not re-enter`
* `Notice: the worktree <path> for this session no longer exists...` per un worktree scomparso; Claude Code lo stampa e continua la sessione, come una ripresa interattiva fa

Il finale di rifiuto incorporato in ogni errore è condiviso con gli avvisi interattivi, quindi corrisponde ancora alla sua voce sotto [Claude Code rifiuta di usare un worktree](#claude-code-refuses-to-use-a-worktree).

Nel risultato stream-json, [`startup_failure_reason`](/docs/it/agent-sdk/typescript#startup_failure_reason) è `worktree_unverified` per l'errore `could not verify worktree` e `worktree_resume_refused` per gli errori `cannot resume into worktree` e `The worktree binding is kept`. Un'applicazione può ramificarsi su di esso invece di abbinare il testo dell'errore. Prima della v2.1.274, il risultato non portava alcun campo `startup_failure_reason`.

<h2 id="see-also">
  Vedere anche
</h2>

I worktree gestiscono l'isolamento dei file. Le pagine correlate di seguito coprono la delega del lavoro in quei checkout isolati, il passaggio dei risultati tra loro, e il passaggio tra le sessioni che create:

* [Subagent](/docs/it/sub-agents): delegare il lavoro ad agenti isolati all'interno di una sessione
* [Messaggistica tra sessioni](/docs/it/cross-session-messaging): lasciare che le sessioni nei vostri worktree si passino i risultati
* [Team di agenti](/docs/it/agent-teams): coordinare più sessioni di Claude automaticamente
* [Gestire le sessioni](/docs/it/sessions): nominare, riprendere, e passare tra conversazioni
* [Sessioni parallele desktop](/docs/it/desktop#work-in-parallel-with-sessions): sessioni supportate da worktree nell'app desktop
