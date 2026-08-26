> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eseguire sessioni parallele con worktrees

> Isolare sessioni parallele di Claude Code in worktrees git separati in modo che i cambiamenti non si scontrino. Copre il flag `--worktree`, l'isolamento dei subagent, `.worktreeinclude`, la pulizia e gli hook VCS non-git.

Un [git worktree](https://git-scm.com/docs/git-worktree) è una directory di lavoro separata con i propri file e branch, che condivide la stessa cronologia del repository e il remote come il vostro checkout principale. Eseguire ogni sessione di Claude Code nel proprio worktree significa che le modifiche in una sessione non toccheranno mai i file in un'altra, quindi potete avere Claude che costruisce una funzionalità in un terminale mentre corregge un bug in un secondo.

Questa pagina copre l'isolamento dei worktree nella CLI. Tutto quanto segue presuppone un repository git. Per altri sistemi di controllo versione, vedere [Controllo versione non-git](#non-git-version-control). L'[app desktop](/docs/it/desktop#work-in-parallel-with-sessions) crea un worktree per ogni nuova sessione automaticamente.

I worktree sono uno dei diversi modi per eseguire Claude in parallelo. Isolano le modifiche ai file, mentre i [subagent](/docs/it/sub-agents) e i [team di agenti](/docs/it/agent-teams) coordinano il lavoro stesso. Vedere [Eseguire agenti in parallelo](/docs/it/agents) per confrontare gli approcci, o saltare direttamente a [Isolare i subagent con worktrees](#isolate-subagents-with-worktrees) per utilizzare worktree e subagent insieme.

<h2 id="start-claude-in-a-worktree">
  Avviare Claude in un worktree
</h2>

Passate `--worktree` o `-w` per creare un worktree isolato e avviare Claude in esso. Per impostazione predefinita, il worktree viene creato sotto `.claude/worktrees/<value>/` nella radice del vostro repository, su un nuovo branch denominato `worktree-<value>`:

```bash theme={null}
claude --worktree feature-auth
```

Per mettere i worktree altrove, configurate un hook [`WorktreeCreate`](#non-git-version-control). Eseguite il comando di nuovo con un nome diverso in un altro terminale per avviare una seconda sessione isolata:

```bash theme={null}
claude --worktree bugfix-123
```

Se omettete il nome, Claude ne genera uno come `bright-running-fox`:

```bash theme={null}
claude --worktree
```

Potete anche chiedere a Claude di "lavorare in un worktree" durante una sessione, e creerà uno con lo strumento [`EnterWorktree`](/docs/it/tools-reference). Una volta in un worktree, Claude può passare direttamente a un altro sotto `.claude/worktrees/` chiamando `EnterWorktree` con il percorso di destinazione. Il worktree precedente rimane su disco intatto.

Entrare in un percorso al di fuori della directory `.claude/worktrees/` del repository richiede prima la vostra approvazione, perché sposta la directory di lavoro della sessione, l'accesso in scrittura e la configurazione del progetto come `CLAUDE.md` e le impostazioni in quella posizione. Una regola di [permesso](/docs/it/permissions) `EnterWorktree` o la scelta di "non chiedere di nuovo" non sopprime questo prompt; solo la modalità `bypassPermissions` lo salta. Prima della v2.1.206, Claude poteva entrare in qualsiasi percorso di worktree esistente senza chiedere.

A partire dalla v2.1.198, entrare o uscire da un worktree trasferisce anche la trascrizione della sessione nella memoria del progetto di quella directory, nello stesso modo in cui [`/cd`](/docs/it/commands) lo fa, quindi `/desktop` e `--resume` trovano la sessione lì in seguito. I worktree creati da un hook [`WorktreeCreate`](#non-git-version-control) sono esclusi e mantengono la trascrizione nella directory di avvio.

I worktree funzionano con il [sandboxing](/docs/it/sandboxing#filesystem-isolation) abilitato: la sandbox consente scritture nella directory condivisa `.git` del repository principale in modo che comandi come `git commit` possano aggiornare i ref e l'indice dall'interno di un worktree collegato.

Prima di utilizzare `--worktree` in modo interattivo in una directory per la prima volta, accettate la finestra di dialogo di fiducia dell'area di lavoro eseguendo `claude` una volta in quella directory. Se la fiducia non è stata ancora accettata, `--worktree` esce con un errore e vi chiede di eseguire `claude` nella directory per primo. Le esecuzioni non interattive con `-p` saltano il [controllo di fiducia](/docs/it/security), quindi `claude -p --worktree` procede senza di esso.

Se Claude Code non riesce a entrare nella directory del worktree all'avvio, ad esempio perché un hook [`WorktreeCreate`](/docs/it/hooks#worktreecreate) ha stampato qualcosa di diverso dalla directory che ha creato, o perché la directory è stata eliminata dopo che è stata configurata, Claude Code stampa un errore che nomina il percorso ed esce con codice 1. Prima della v2.1.205, questo causava un crash della sessione, e con `-p` si bloccava per circa 30 secondi prima di uscire con codice 0.

I plugin installati a [ambito del progetto](/docs/it/plugins-reference#plugin-installation-scopes) dal checkout principale caricano anche nei worktree dello stesso repository, quindi non è necessario reinstallarli per ogni worktree. Questo si applica sia che creiate il worktree con `--worktree` che con `git worktree add`. Richiede Claude Code v2.1.200 o successivo.

<Tip>
  Aggiungete `.claude/worktrees/` al vostro `.gitignore` in modo che i contenuti del worktree non appaiano come file non tracciati nel vostro checkout principale.
</Tip>

<h3 id="choose-the-base-branch">
  Scegliere il branch di base
</h3>

I worktree si diramano dal branch predefinito del vostro repository, `origin/HEAD`, quindi iniziano da un albero pulito che corrisponde al remote. Quando nessun repository ha recuperato il repository negli ultimi 24 ore, Claude Code aggiorna `origin/HEAD` con un fetch del branch predefinito, limitato a cinque secondi, e utilizza il ref memorizzato nella cache locale se il fetch fallisce. Se nessun remote è configurato, o `origin/HEAD` non è memorizzato nella cache localmente e non può essere recuperato, il worktree ricade al vostro `HEAD` locale corrente.

L'aggiornamento richiede Claude Code v2.1.208 o successivo; prima di allora, un worktree nuovo utilizzava qualsiasi `origin/HEAD` fosse già memorizzato nella cache localmente.

Per diramarvisi sempre dal `HEAD` locale, impostate `worktree.baseRef` a `"head"` nelle [impostazioni](/docs/it/settings#worktree-settings). Impostare `baseRef` a `"head"` fa sì che i nuovi worktree portino i vostri commit non spinti e lo stato del feature-branch, il che è utile quando si isolano i subagent che devono operare su lavori in corso. Quando la sessione viene eseguita all'interno di un worktree collegato, `"head"` si risolve in `HEAD` di quel worktree, non nel checkout principale. L'impostazione accetta solo `"fresh"` o `"head"`, non ref git arbitrari:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Per diramarvisi da una pull request specifica, passate il numero della PR con il prefisso `#`, o un URL completo della pull request di GitHub. Claude Code recupera `pull/<number>/head` da `origin` e crea il worktree in `.claude/worktrees/pr-<number>`:

```bash theme={null}
claude --worktree "#1234"
```

Per il controllo completo su come vengono creati i worktree, configurate un hook [`WorktreeCreate`](/docs/it/hooks#worktreecreate), che sostituisce completamente la logica predefinita di `git worktree`.

<h3 id="reuse-a-worktree-name">
  Riutilizzare un nome di worktree
</h3>

Riutilizzare un nome di worktree la cui directory esiste già riprende quel worktree.

Un worktree ripreso si ripristina alla [base corrente](#choose-the-base-branch) invece di riprendere al suo vecchio tip quando tutte le seguenti condizioni sono soddisfatte:

* Non ha modifiche non committate o file non tracciati.
* È ancora sul branch che Claude Code ha creato per esso.
* Non ha mai committato, o la sua pull request è stata unita e il suo branch remoto è stato eliminato.

Prima della v2.1.208, un nome riutilizzato riprendeva sempre il vecchio worktree al suo vecchio tip.

<h2 id="copy-gitignored-files-into-worktrees">
  Copiare file gitignored nei worktrees
</h2>

Un worktree è un checkout fresco, quindi i file non tracciati come `.env` o `.env.local` dal vostro repository principale non sono presenti. Per copiarli automaticamente quando Claude crea un worktree, aggiungete un file `.worktreeinclude` alla radice del vostro progetto.

Il file utilizza la sintassi `.gitignore`. Solo i file che corrispondono a un pattern e sono anche gitignored vengono copiati, quindi i file tracciati non vengono mai duplicati.

Questo `.worktreeinclude` copia due file env e una configurazione di segreti in ogni nuovo worktree:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Questo si applica ai worktree creati con `--worktree`, [worktree dei subagent](#isolate-subagents-with-worktrees), e sessioni parallele nell'[app desktop](/docs/it/desktop#work-in-parallel-with-sessions).

<h2 id="isolate-subagents-with-worktrees">
  Isolare i subagent con worktrees
</h2>

I subagent possono eseguire nei propri worktree in modo che le modifiche parallele non entrino in conflitto. Chiedete a Claude di "usare worktree per i vostri agenti", o impostatelo permanentemente su un [subagent personalizzato](/docs/it/sub-agents#supported-frontmatter-fields) aggiungendo `isolation: worktree` al frontmatter. Ogni subagent ottiene un worktree temporaneo che viene rimosso automaticamente quando il subagent finisce senza modifiche.

I worktree dei subagent utilizzano lo stesso [ramo base](#choose-the-base-branch) di `--worktree`, quindi si diramano dal ramo predefinito del vostro repository a meno che `worktree.baseRef` non sia impostato su `"head"`.

<h2 id="clean-up-worktrees">
  Pulire i worktrees
</h2>

Quando uscite da una sessione di worktree, la pulizia dipende dal fatto che abbiate apportato modifiche:

* **Nessuna modifica non committata, nessun file non tracciato e nessun nuovo commit**: il worktree e il suo branch vengono rimossi automaticamente. Se la sessione ha un [nome](/docs/it/sessions#name-your-sessions), Claude vi chiede invece in modo da poter mantenere il worktree per dopo
* **Modifiche non committate, file non tracciati o nuovi commit esistono**: Claude vi chiede di mantenere o rimuovere il worktree. Mantenere preserva la directory e il branch in modo da poter tornare in seguito. Rimuovere elimina la directory del worktree e il suo branch, scartando tutte le modifiche non committate, i file non tracciati e i commit
* **Esecuzioni non interattive**: i worktree creati con `--worktree` insieme a `-p` non vengono puliti automaticamente poiché non c'è un prompt di uscita. Rimuoveteli con `git worktree remove`

I worktree che Claude ha creato per i subagent e le [sessioni in background](/docs/it/agent-view#how-file-edits-are-isolated) vengono rimossi automaticamente una volta che sono più vecchi della vostra impostazione [`cleanupPeriodDays`](/docs/it/settings#available-settings), a condizione che non abbiano modifiche non committate, nessun file non tracciato e nessun commit non spinto. I worktree che create con `--worktree` non vengono mai rimossi da questa scansione.

Mentre un agente è in esecuzione, Claude esegue `git worktree lock` sul suo worktree in modo che la pulizia concorrente non possa rimuoverlo. Il blocco viene rilasciato quando l'agente termina. Per pulire un worktree che la scansione mantiene, eseguite `git worktree remove`, aggiungendo `--force` se il worktree ha modifiche non committate o file non tracciati.

Su Windows, prima di rimuovere un worktree, Claude Code rimuove qualsiasi junction NTFS o symlink di directory a qualsiasi profondità al suo interno come voce di collegamento, in modo che la rimozione del worktree non elimini i file a cui punta un collegamento. Prima della v2.1.205, Claude Code rimuoveva solo i collegamenti di livello superiore come voci di collegamento, e la rimozione di un worktree con una junction annidata in una sottodirectory potrebbe eliminare i contenuti della directory a cui il collegamento puntava al di fuori del worktree.

<h2 id="manage-worktrees-manually">
  Gestire i worktrees manualmente
</h2>

Per il controllo completo sulla posizione del worktree e sulla configurazione del branch, create i worktree direttamente con Git. Questo è utile quando dovete controllare un branch esistente specifico o posizionare il worktree al di fuori del repository.

Create un worktree su un nuovo branch:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Create un worktree da un branch esistente:

```bash theme={null}
git worktree add ../project-bugfix bugfix-123
```

Avviate Claude nel worktree:

```bash theme={null}
cd ../project-feature-a && claude
```

Elencate i vostri worktree:

```bash theme={null}
git worktree list
```

Rimuovete uno quando avete finito:

```bash theme={null}
git worktree remove ../project-feature-a
```

Vedere la [documentazione di Git worktree](https://git-scm.com/docs/git-worktree) per il riferimento completo dei comandi. Ricordate di inizializzare il vostro ambiente di sviluppo in ogni nuovo worktree: installate le dipendenze, configurate gli ambienti virtuali, o eseguite qualunque cosa il vostro progetto richieda.

<h2 id="non-git-version-control">
  Controllo versione non-git
</h2>

L'isolamento dei worktree utilizza git per impostazione predefinita. Per SVN, Perforce, Mercurial o altri sistemi, configurate gli hook [`WorktreeCreate` e `WorktreeRemove`](/docs/it/hooks#worktreecreate) per fornire logica di creazione e pulizia personalizzata. Poiché l'hook sostituisce il comportamento predefinito di git, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) non viene elaborato quando utilizzate `--worktree`. Copiate i file di configurazione locali all'interno dello script dell'hook.

Questo hook `WorktreeCreate` legge il nome del worktree da stdin, controlla una copia di lavoro SVN fresca, e stampa il percorso della directory in modo che Claude Code possa usarlo come directory di lavoro della sessione:

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

<h2 id="see-also">
  Vedere anche
</h2>

I worktree gestiscono l'isolamento dei file. Le pagine correlate di seguito coprono la delega del lavoro in quei checkout isolati e il passaggio tra le sessioni che create:

* [Subagent](/docs/it/sub-agents): delegare il lavoro ad agenti isolati all'interno di una sessione
* [Team di agenti](/docs/it/agent-teams): coordinare più sessioni di Claude automaticamente
* [Gestire le sessioni](/docs/it/sessions): nominare, riprendere e passare tra conversazioni
* [Sessioni parallele desktop](/docs/it/desktop#work-in-parallel-with-sessions): sessioni supportate da worktree nell'app desktop
