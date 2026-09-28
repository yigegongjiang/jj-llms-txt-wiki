> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestire le sessioni

> Assegnare nomi, riprendere, creare rami e passare tra conversazioni di Claude Code. Copre `--continue`, `--resume`, `--from-pr`, il selezionatore `/resume`, la denominazione delle sessioni, l'esportazione dei trascritti e dove vengono archiviati i trascritti.

Una sessione è una conversazione salvata legata a una directory di progetto. Claude Code la archivia localmente mentre lavori, così puoi riprendere da dove hai lasciato, creare un ramo per provare un approccio diverso, o passare tra attività.

L'[app desktop](/docs/it/desktop#work-in-parallel-with-sessions), [Claude Code sul web](/docs/it/claude-code-on-the-web) e l'[estensione VS Code](/docs/it/vs-code#resume-past-conversations) mantengono ciascuno la propria cronologia delle sessioni. Questa pagina copre la CLI.

<h2 id="resume-a-session">
  Riprendere una sessione
</h2>

Le sessioni vengono salvate continuamente in [file di trascritto locali](#export-and-locate-session-data) mentre lavori, quindi puoi tornare a una dopo aver chiuso o eseguito `/clear`. Usa questi punti di ingresso:

| Comando                             | Cosa fa                                                                                                                        |
| :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `claude --continue`                 | Riprende la conversazione più recente nella directory corrente                                                                 |
| `claude --resume`                   | Apre il [selezionatore di sessioni](#use-the-session-picker)                                                                   |
| `claude --resume <name>`            | Riprende direttamente la sessione denominata                                                                                   |
| `claude --resume <transcript-path>` | Riprende la conversazione archiviata nel file di [trascritto](#where-transcripts-are-stored) `.jsonl` a quel percorso assoluto |
| `claude --from-pr <number>`         | Apre il selezionatore di sessioni filtrato alle sessioni collegate a quella pull request                                       |
| `/resume`                           | Passa a una conversazione diversa da una sessione attiva                                                                       |

Claude Code lascia fuori dal selezionatore di sessioni e da `claude --continue` le sessioni create con [`claude -p`](/docs/it/headless) o l'[Agent SDK](/docs/it/agent-sdk/overview). Puoi comunque riprenderne una passando il suo ID di sessione a `claude --resume <session-id>`. Con `claude --continue`, Claude Code salta anche le [sessioni il cui primo prompt era `/loop`](#where-the-session-picker-looks). Quando esegui [`claude -p --continue`](/docs/it/headless#continue-conversations), Claude Code include le sessioni `-p`, SDK e `/loop`.

`claude --continue` apre una [sessione in background](/docs/it/agent-view) che è terminata, ma non una ancora in esecuzione; l'apertura di sessioni in background terminate richiede Claude Code v2.1.257 o successiva. Se la tua conversazione più recente è una che hai [spostato in background](/docs/it/agent-view#send-the-session-to-the-background) ed è ancora in esecuzione lì, Claude Code esce con `Your most recent conversation is running in the background` e l'ID di quella sessione. Collegati alla sessione da [`claude agents`](/docs/it/agent-view#attach-to-a-session), oppure esegui `claude --resume` per sceglierne un'altra.

Puoi eseguire `claude --resume <session-id>` da qualsiasi directory: Claude Code cerca l'ID nella directory del progetto corrente e nei suoi git worktrees per primo, poi in ogni altro progetto su questa macchina, quindi trova una sessione che è stata avviata altrove o spostata con [`/cd`](/docs/it/commands). La ricerca tra progetti risolve l'ID solo quando esattamente un altro progetto contiene un trascritto con messaggi per esso, quindi un duplicato copiato manualmente fa sì che Claude Code segnali non trovato piuttosto che riprendere una copia arbitraria. Se nessuna sessione archiviata corrisponde all'ID, Claude Code segnala `No conversation found with session ID: <session-id>`. Prima della v2.1.223, la ricerca si fermava alla directory del progetto corrente e ai suoi git worktrees, quindi dovevi riprendere dalla directory in cui la sessione aveva lavorato per ultimo.

<h3 id="what-a-resumed-session-restores">
  Cosa ripristina una sessione ripresa
</h3>

Una sessione ripresa ripristina la conversazione insieme allo stato salvato in essa:

* Cronologia della conversazione: la cronologia completa, incluse le chiamate agli strumenti e i risultati. Uno strumento che era ancora in esecuzione quando il processo precedente è terminato, ad esempio in un arresto anomalo, non finisce o non viene eseguito di nuovo quando riprendi. Claude vede la chiamata contrassegnata come interrotta prima che il suo risultato fosse registrato e gli viene detto di verificare se ha avuto effetto prima di eseguirla di nuovo, a meno che [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](/docs/it/env-vars#variables) non sia impostato. Prima della v2.1.281, Claude Code eliminava la chiamata interrotta dalla conversazione o la mostrava a Claude come una che hai interrotto.
* Modello: la sessione continua sul modello che stava utilizzando. Il modello non viene ripristinato quando è stato ritirato o non è consentito da `availableModels`, quando un flag `--model` o una variabile di ambiente della famiglia `ANTHROPIC_MODEL` ne seleziona uno al lancio, o su provider che utilizzano ID di distribuzione specifici del provider, come [Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry](/docs/it/third-party-integrations); vedi [configurazione del modello](/docs/it/model-config#setting-your-model) per l'ordine di risoluzione.
* Agent: una sessione avviata con [`--agent`](/docs/it/sub-agents#invoke-subagents-explicitly) o l'impostazione `agent` continua come quell'agent, mantenendo le sue restrizioni degli strumenti e il modello. Passa `--agent` quando riprendi per sceglierne uno diverso; per il prompt di sistema in entrambi i casi, vedi [Flag del prompt di sistema nelle conversazioni riprese](/docs/it/cli-reference#system-prompt-flags-in-resumed-conversations). Claude Code cerca l'agent in due posti: la directory originale della sessione, a condizione che tu abbia [fiducia in quell'area di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust), e poi la directory da cui riprendi, quindi un agent con ambito di progetto si carica ancora quando riprendi da un'altra directory. Se Claude Code non trova l'agent in nessuno dei due posti, la sessione riprende con gli strumenti predefiniti e mostra un [avviso che nomina l'agent](/docs/it/errors#session-agent-no-longer-available).
* Modalità di autorizzazione: se riprendi da un terminale con `claude --continue`, `claude --resume <session-id>` o `claude --resume <name>` quando il nome corrisponde a una sessione, senza `-p`, Claude Code ripristina la modalità di autorizzazione in cui era la sessione, tranne nei casi in [modalità di autorizzazione al ripristino](#permission-mode-on-resume), che copre anche il selezionatore di sessioni, `/resume` e la ripresa con `claude -p`. Passa `--permission-mode` o `--dangerously-skip-permissions` per ignorare la modalità ripristinata.
* Obiettivo attivo: un [obiettivo](/docs/it/goal#resume-with-an-active-goal) che era ancora attivo quando la sessione è terminata si trasferisce; il suo conteggio dei turni, il timer e la linea di base della spesa di token si ripristinano.
* Attività pianificate: le [attività che non sono scadute](/docs/it/scheduled-tasks#limitations) vengono ripristinate. Le attività Bash in background e le attività di monitoraggio non lo sono.

Non ogni flag di configurazione dal lancio originale viene ripristinato. Se la sessione dipendeva da `--mcp-config`, `--settings`, `--plugin-dir`, `--fallback-model` o directory aggiunte con `--add-dir`, passale di nuovo quando riprendi; le directory aggiunte a metà sessione con `/add-dir` non vengono ripristinate neanche, anche se il selezionatore di sessioni le usa ancora per individuare la sessione. I file di impostazioni standard, come `settings.json` e `settings.local.json`, vengono riletti al lancio, quindi la configurazione che vive in essi non ha bisogno di essere passata di nuovo. Per `--system-prompt` e `--append-system-prompt`, vedi [Flag del prompt di sistema nelle conversazioni riprese](/docs/it/cli-reference#system-prompt-flags-in-resumed-conversations).

<h4 id="permission-mode-on-resume">
  Modalità di autorizzazione al ripristino
</h4>

La modalità di autorizzazione in cui Claude Code avvia una sessione ripresa dipende da come la riprendi:

* Terminale: `claude --continue`, `claude --resume <session-id>` o `claude --resume <name>` quando il nome corrisponde a una sessione, senza `-p`. Claude Code ripristina la modalità di autorizzazione in cui era la sessione, tranne nei casi nella tabella. Passa `--permission-mode` o `--dangerously-skip-permissions` per ignorare la modalità ripristinata.
* Non interattivo: `claude -p --resume` o `claude -p --continue`. Claude Code avvia l'esecuzione nella modalità di autorizzazione in cui una nuova esecuzione `claude -p` si avvierebbe, tranne che una sessione che è terminata in modalità piano riprende in modalità piano secondo le [condizioni di seguito](#resume-in-plan-mode-with-p).
* VS Code: il pannello di conversazione dell'estensione. La tabella copre solo una conversazione che è terminata in modalità piano; per il resto, vedi [riprendere conversazioni passate](/docs/it/vs-code#resume-past-conversations).
* Selezionatore di sessioni al lancio: una sessione che selezioni dal [selezionatore di sessioni](#use-the-session-picker), che tu l'abbia aperto con `claude --resume` da solo, `claude --from-pr` o un nome che corrisponde a più di una sessione. Claude Code non ripristina la modalità di autorizzazione archiviata. Avvia la sessione nella modalità di autorizzazione in cui avvierebbe una nuova sessione dalla stessa riga di comando.
* `/resume` dentro una sessione, con o senza argomento: Claude Code non ripristina la modalità di autorizzazione archiviata. La conversazione a cui passi continua nella modalità di autorizzazione in cui è la tua sessione corrente.

Il ripristino della modalità piano sui percorsi non interattivi e VS Code richiede Claude Code v2.1.246 o successiva. Ogni riga nomina la modalità di autorizzazione in cui la sessione è terminata, quale dei percorsi terminale, non interattivo e VS Code la riprendi, e la modalità di autorizzazione in cui Claude Code avvia la sessione ripresa.

| La sessione è terminata in | Come la riprendi                                                                 | Modalità di autorizzazione dopo il ripristino                                                                                                                                                                                                                                                                                                                                                          |
| :------------------------- | :------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bypassPermissions`        | Terminale                                                                        | La modalità di autorizzazione in cui una nuova sessione si avvierebbe. Per [ignorare le autorizzazioni](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode) di nuovo, abilitala al lancio con uno dei suoi flag di lancio o `permissions.defaultMode: "bypassPermissions"` in [impostazioni utente, `--settings` o impostazioni gestite](/docs/it/settings-reference#permissions-defaultmode) |
| `plan`                     | Terminale                                                                        | La modalità di autorizzazione in cui una nuova sessione si avvierebbe                                                                                                                                                                                                                                                                                                                                  |
| `auto`                     | Terminale                                                                        | `auto`, solo quando il tuo account soddisfa ancora i [requisiti della modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                                                                                                                                                            |
| Manuale                    | Terminale                                                                        | Manuale quando una nuova sessione si avvierebbe in modalità auto dal [default integrato](/docs/it/permission-modes#which-mode-a-session-starts-in). Quando un `defaultMode` da un file di impostazioni [ha effetto](/docs/it/permission-modes#which-mode-a-session-starts-in), Claude Code avvia la sessione ripresa in quella modalità invece                                                                   |
| `plan`                     | Non interattivo, secondo le [condizioni di seguito](#resume-in-plan-mode-with-p) | Modalità piano                                                                                                                                                                                                                                                                                                                                                                                         |
| Qualsiasi modalità         | Non interattivo, in qualsiasi altro caso                                         | La modalità di autorizzazione in cui una nuova esecuzione `claude -p` si avvierebbe                                                                                                                                                                                                                                                                                                                    |
| `plan`                     | VS Code                                                                          | Modalità piano, con [le eccezioni nella pagina VS Code](/docs/it/vs-code#resume-past-conversations)                                                                                                                                                                                                                                                                                                         |

<h5 id="resume-in-plan-mode-with-p">
  Riprendere in modalità piano con `-p`
</h5>

Un'esecuzione `claude -p --resume` o `claude -p --continue` riprende in modalità piano solo quando tutte e quattro le condizioni sono soddisfatte:

* Passi [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags), in modo che Claude Code possa presentare il piano per l'approvazione
* Non passi `--permission-mode` o `--dangerously-skip-permissions`
* Non passi `--fork-session`
* L'esecuzione non è avviata attraverso [canali](/docs/it/channels)

<h3 id="resume-from-a-summary">
  Riprendere da un riepilogo
</h3>

Su un piano Pro o Max, quando riprendi una sessione che è stata inattiva per più di circa un'ora ed è superiore a 100.000 token, Claude Code ripristina la conversazione e quindi apre una finestra di dialogo prima che tu invii il tuo primo messaggio. La [cache del prompt](/docs/it/prompt-caching#cache-lifetime) della sessione è scaduta entro allora, quindi la richiesta successiva elabora la cronologia completa una volta indipendentemente da quale opzione della finestra di dialogo scegli.

La finestra di dialogo offre tre modi per continuare la sessione. Differiscono in quanto della conversazione ciascuno trasporta in avanti nelle richieste successive, il che è un compromesso tra mantenere ogni dettaglio e inviare meno token per richiesta:

* **Riprendi da riepilogo**: esegue [`/compact`](/docs/it/context-window#what-survives-compaction) immediatamente. Claude Code invia una richiesta di riepilogazione sulla cronologia completa, quindi sostituisce la cronologia con il riepilogo, i tuoi scambi più recenti e fino a cinque file letti di recente. Le richieste successive trasportano il riepilogo invece della cronologia completa.
* **Riprendi la sessione completa così com'è**: carica la conversazione invariata. Dopo che invii il tuo primo messaggio, Claude Code rielabora e memorizza nuovamente nella cache la cronologia completa, quindi la rilegge dalla cache nelle richieste successive mentre la cache rimane calda.
* **Non chiedermi più**: riprende la sessione completa e smette di mostrare la finestra di dialogo su tutti i futuri ripristini.

Riprendere così com'è mantiene ogni dettaglio della conversazione disponibile, a un costo per richiesta che scala con la dimensione della conversazione. Riprendere dal riepilogo costa meno su ogni richiesta successiva perché trasporta il riepilogo invece della cronologia completa, ma qualunque cosa il riepilogo ometta non è più nel contesto di Claude. Vedi [perché l'utilizzo aumenta in una sessione lunga](/docs/it/costs#why-usage-climbs-in-a-long-session) per sapere da dove viene quel costo per richiesta.

<h3 id="where-the-session-picker-looks">
  Dove il selezionatore di sessioni cerca
</h3>

Claude Code archivia le sessioni per directory di progetto. Per impostazione predefinita, il selezionatore di sessioni mostra:

* Sessioni dal worktree corrente, incluse le [sessioni in background](/docs/it/agent-view), che sono contrassegnate `bg` nell'elenco
* Sessioni avviate altrove che hanno aggiunto la directory corrente con `/add-dir`

Usa `Ctrl+W` per ampliare a tutti i worktree del repository o `Ctrl+A` per ampliare a ogni progetto su questa macchina.

Le sessioni il cui primo prompt era un comando [`/loop`](/docs/it/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) non appaiono nel selezionatore, e `claude --continue` le salta anche. L'esecuzione di `/loop` più tardi in una conversazione non nasconde la sessione. Prima della v2.1.211, un'esecuzione `/loop` all'inizio di una conversazione nascondeva la sessione dal selezionatore in modo permanente.

Lo spostamento di una sessione con [`/cd`](/docs/it/commands) la trasferisce nell'archivio del progetto della nuova directory, quindi appare nel selezionatore di quella directory in seguito. A partire dalla v2.1.196, una sessione spostata rimane fuori dal selezionatore della directory precedente anche dopo un arresto anomalo o un'uscita forzata. Nelle versioni precedenti, potrebbe anche riapparire nell'elenco della directory precedente dopo un'uscita non pulita quando il percorso precedente conteneva caratteri speciali come trattini bassi.

Quando selezioni una sessione da un altro worktree dello stesso repository, Claude Code la riprende sul posto; quando il worktree della sessione non esiste più, Claude Code [la riprende nella tua directory corrente](/docs/it/worktrees#resume-a-worktree-session). Quando selezioni una sessione da un progetto non correlato, Claude Code copia un comando `cd` e resume negli appunti. Se la directory di quel progetto non esiste più, Claude Code riprende la sessione nella tua directory corrente piuttosto che copiare un comando `cd` che fallirebbe.

La ripresa per nome si risolve nel repository corrente e nei suoi worktree. Entrambi i moduli cercano una corrispondenza esatta e la riprendono direttamente anche se si trova in un worktree diverso:

| Comando                  | Corrispondenza esatta | Nome ambiguo                                                                                |
| :----------------------- | :-------------------- | :------------------------------------------------------------------------------------------ |
| `claude --resume <name>` | Riprende direttamente | Apre il selezionatore di sessioni con il nome pre-compilato come termine di ricerca         |
| `/resume <name>`         | Riprende direttamente | Segnala un errore; esegui `/resume` senza argomenti per aprire il selezionatore di sessioni |

<h2 id="name-your-sessions">
  Assegnare nomi alle sessioni
</h2>

Dai alle sessioni nomi descrittivi in modo che siano trovabili nel selezionatore di sessioni e riprendibili per nome. Questo è particolarmente importante quando stai lavorando su più attività in parallelo.

| Quando                         | Come impostare il nome                                                                                                                                                                    |
| :----------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| All'avvio                      | `claude -n auth-refactor`                                                                                                                                                                 |
| Durante una sessione           | `/rename auth-refactor`. Il nome appare anche sulla barra del prompt                                                                                                                      |
| Dal selezionatore di sessioni  | Evidenzia una sessione e premi `Ctrl+R`                                                                                                                                                   |
| Sull'accettazione del piano    | Accettare un piano in [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) assegna un nome alla sessione in base al piano a meno che non ne abbia già impostato uno   |
| Da claude.ai o dall'app Claude | Rinomina una sessione [Remote Control](/docs/it/remote-control#connect-from-another-device); Claude Code applica lo stesso nome nella CLI. Richiede Claude Code v2.1.221 o versione successiva |
| Dall'app desktop               | Rinomina una sessione nell'[app desktop](/docs/it/desktop#work-in-parallel-with-sessions)                                                                                                      |

Una volta che una sessione è denominata tramite una rotta CLI o da claude.ai, torna a essa con `claude --resume <name>` o `/resume <name>`; una sessione dell'app desktop si riprende nell'app, che mantiene la propria cronologia delle sessioni. Vedi [Riprendere una sessione](#resume-a-session) per come il comportamento della risoluzione dei nomi funziona nei worktrees.

Quando avvii o riprendi una sessione interattiva con un nome che un'altra sessione live su questa macchina utilizza già, o rinomini una sessione con tale nome, Claude Code lascia il nome con la sessione che lo ha già, rinomina la tua in una variante con un suffisso di due parole, come `auth-refactor-graceful-unicorn`, e te lo comunica. Esegui `/rename` con un nuovo nome se preferisci sceglierne uno tu stesso. Prima della v2.1.232, entrambe le sessioni mantenevano il nome.

In tre casi Claude Code non rinomina il duplicato, quindi puoi comunque vedere due sessioni con lo stesso nome negli elenchi:

* Non controlla i titoli generati dall'IA o i nomi di visualizzazione predefiniti.
* Non controlla il `--name` di una sessione [background](/docs/it/agent-view#from-your-shell) o `-p` all'avvio.
* Non può rinominare una sessione su una versione precedente di Claude Code.

Le sessioni che non nomini ricevono comunque due etichette che Claude Code assegna. Solo il titolo generato funziona come handle di ripresa:

* Nome di visualizzazione predefinito: le sessioni interattive che non nomini mai ricevono comunque un nome di visualizzazione predefinito quando si avviano. Richiede Claude Code v2.1.196 o versione successiva. Il valore predefinito combina il nome della directory di lavoro con un suffisso di due caratteri, ad esempio `my-app-3f`, e identifica la sessione negli elenchi delle sessioni in esecuzione, come la [agent view](/docs/it/agent-view) e l'output di `claude agents --json`. Il valore predefinito non è un handle di ripresa. Se lo passi a `claude --resume` o `/resume`, Claude Code non trova la sessione. Denominare la sessione sostituisce il valore predefinito in quegli elenchi, così come accettare un piano.
* Titolo generato: se non nomini una sessione, Claude Code genera un titolo di sessione per essa. Il titolo è un breve riassunto del tuo primo prompt, scritto da una richiesta in background al modello piccolo/veloce, normalmente un modello della classe Haiku. Un'esecuzione `claude -p` che avvii direttamente da una shell o script non ne riceve uno.

  Accettare un piano sostituisce il titolo del primo prompt con un titolo basato sul piano. Denominare la sessione lo sostituisce anche.

  Vedi il titolo del primo prompt nel [selezionatore di sessioni](#use-the-session-picker) e nel campo [`session_name`](/docs/it/statusline) della statusline quando nessun nome è impostato. Il titolo del piano appare negli stessi due posti e anche negli elenchi delle sessioni in esecuzione, dove prende il posto del nome di visualizzazione predefinito.

  Puoi passare uno dei due titoli a `claude --resume` o `/resume`, e Claude Code lo risolve nello stesso modo di un nome che hai impostato.

<h2 id="use-the-session-picker">
  Usare il selezionatore di sessioni
</h2>

Esegui `/resume` all'interno di una sessione, o `claude --resume` senza argomenti, per aprire il selezionatore di sessioni interattivo. Usa queste scorciatoie da tastiera per navigare, cercare e ampliare l'elenco:

| Scorciatoia                                             | Azione                                                                                                                                                                             |
| :------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                               | Naviga tra le sessioni                                                                                                                                                             |
| `→` / `←`                                               | Espandi o comprimi le sessioni raggruppate                                                                                                                                         |
| `Enter`                                                 | Riprendi la sessione evidenziata                                                                                                                                                   |
| `Space`                                                 | Anteprima del contenuto della sessione. `Ctrl+V` funziona anche su terminali che non lo catturano come incolla                                                                     |
| `Ctrl+R`                                                | Rinomina la sessione evidenziata                                                                                                                                                   |
| `/` o qualsiasi carattere stampabile diverso da `Space` | Entra in modalità di ricerca e filtra le sessioni. Incolla un URL di pull o merge request di GitHub, GitHub Enterprise, GitLab o Bitbucket per trovare la sessione che l'ha creato |
| `Ctrl+A`                                                | Mostra le sessioni da tutti i progetti su questa macchina. Premi di nuovo per tornare al repository corrente                                                                       |
| `Ctrl+W`                                                | Mostra le sessioni da tutti i worktree del repository corrente. Premi di nuovo per tornare al worktree corrente. Mostrato solo nei repository multi-worktree                       |
| `Ctrl+B`                                                | Filtra le sessioni dal ramo git corrente. Premi di nuovo per mostrare tutti i rami                                                                                                 |
| `Esc`                                                   | Esci dal selezionatore di sessioni o dalla modalità di ricerca                                                                                                                     |

Ogni riga mostra il nome della sessione se impostato, altrimenti il titolo della sessione generato dall'IA, il riassunto della conversazione o il primo prompt, insieme al tempo dall'ultima attività, al ramo git e alla dimensione del file. Amplia a tutti i progetti con `Ctrl+A` per visualizzare anche il percorso del progetto di ogni sessione.

Le sessioni create con `/branch` o `--fork-session` ottengono i propri ID di sessione e appaiono come righe separate. Quando il selezionatore trova più di una voce per la stessa sessione, le raggruppa sotto una singola riga. Premi `→` per espandere un gruppo.

Se Claude Code non riesce a caricare la sessione che selezioni dal selezionatore `claude --resume`, stampa [`Failed to resume the conversation`](/docs/it/errors#failed-to-resume-the-conversation) con un comando per riprovare, quindi esce con codice 1. Dal selezionatore `/resume` all'interno di una sessione, Claude Code segnala l'errore e la tua conversazione corrente continua a funzionare.

<h2 id="branch-a-session">
  Creare un ramo di una sessione
</h2>

La creazione di un ramo crea una copia della conversazione finora e ti passa in essa, lasciando l'originale intatto. Usalo per provare un approccio diverso senza perdere il percorso su cui eri.

Da dentro una sessione, esegui `/branch` con un nome opzionale:

```text theme={null}
/branch try-streaming-approach
```

Se ometti il nome, Claude Code assegna un nome al nuovo ramo in base al primo prompt nella conversazione. A partire dalla v2.1.198 questo si applica anche dopo la [compattazione](/docs/it/how-claude-code-works#when-context-fills-up); le versioni precedenti ricadevano nel nome letterale `Branched conversation` invece di guardare oltre il riepilogo della compattazione al prompt originale iniziale.

Dalla riga di comando, combina `--continue` o `--resume` con `--fork-session`:

```bash theme={null}
claude --continue --fork-session
```

La conferma di `/branch` stampa due ID di sessione: il nuovo ramo in cui sei ora e l'originale. L'originale rimane invariato su disco e rimane nel selezionatore di sessioni; torna ad esso con `/resume <original-name>` o passando il suo ID a `/resume`.

`/branch` copia il trascritto e passa il processo Claude Code in esecuzione a scrivere su di esso. Quella distinzione determina ciò che il ramo eredita:

| Stato                                                                                                                                                                      | Dopo `/branch`                                                                                                                                                                                                                         |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cronologia della conversazione                                                                                                                                             | Copiata nel ramo fino al punto in cui hai eseguito `/branch`                                                                                                                                                                           |
| Concessioni di permessi "Allow for this session"                                                                                                                           | Trasferite; il ramo viene eseguito nello stesso processo, quindi i tuoi permessi esistenti si applicano ancora. Se crei un fork in un processo separato con `--fork-session`, il nuovo processo inizia senza di essi e li riapprovi lì |
| [Subagenti in background](/docs/it/sub-agents#run-subagents-in-foreground-or-background) e [comandi Bash in background](/docs/it/interactive-mode#background-bash-commands) in corso | Continuano a essere eseguiti. Il loro output appare nel nuovo ramo in cui sei passato, non nella sessione originale                                                                                                                    |
| Connessione [Remote Control](/docs/it/remote-control)                                                                                                                           | Rimane connessa. Un telefono o browser connesso alla sessione ti segue nel ramo e continua a ricevere nuovi messaggi lì                                                                                                                |

Se riprendi la stessa sessione in due terminali senza creare un fork, i messaggi di entrambi si intercalano in un unico trascritto. Per il rewind basato su checkpoint all'interno di una singola sessione, vedi [Checkpointing](/docs/it/checkpointing).

<h2 id="manage-context-within-a-session">
  Gestire il contesto all'interno di una sessione
</h2>

Questi comandi controllano cosa c'è nella finestra di contesto senza lasciare la sessione:

* **`/clear`**: ricomincia da capo con un contesto vuoto. Claude Code salva la conversazione precedente; riprendila con `/resume`, oppure, nello stesso processo Claude Code, dalla [voce della sessione precedente del menu rewind](/docs/it/checkpointing#rewind-past-a-cleared-conversation). Senza argomenti, la nuova conversazione mantiene un nome che hai impostato con `--name` o `/rename`, ma non un titolo di sessione generato dall'IA. Per nominare invece la conversazione che stai lasciando, passa il nome, come in `/clear release-prep`; la nuova conversazione inizia quindi senza nome
* **`/compact [instructions]`**: sostituisci la cronologia con un riassunto, facoltativamente focalizzato su ciò che specifichi
* **`/context`**: mostra cosa sta attualmente consumando contesto

Per come la compattazione interagisce con CLAUDE.md, skills e regole, vedi la [guida della finestra di contesto](/docs/it/context-window). Per strategie su quando cancellare rispetto a compattare, vedi [Best practices](/docs/it/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Esportare e individuare i dati della sessione
</h2>

Esegui `/export` per aprire un menu che ti consente di copiare la conversazione corrente negli appunti o salvarla come file di testo semplice, con messaggi e output degli strumenti renderizzati come testo leggibile. Passa un nome file per saltare il menu e scrivere direttamente in quel file.

<h3 id="access-conversations-from-scripts">
  Accedere alle conversazioni da script
</h3>

`/export` produce una trascrizione renderizzata per una persona da leggere. Le interfacce di seguito producono dati strutturati per uno script da analizzare: un risultato JSON da un'esecuzione, il percorso al file di trascrizione di una sessione, o un flusso live di eventi. Scegli in base a ciò che attiva lo script:

* **Esegui Claude una volta e cattura il risultato**: invoca `claude -p` con [`--output-format json` o `stream-json`](/docs/it/headless#get-structured-output) per catturare il risultato, l'ID della sessione, l'utilizzo e il costo di un'esecuzione non interattiva come JSON strutturato.
* **Poni una domanda a una sessione esistente**: passa un ID di sessione a [`claude -p --resume`](/docs/it/headless#continue-conversations) per inviare un prompt di follow-up, come una richiesta di riepilogo, e catturare la risposta strutturata.
* **Reagisci agli eventi della sessione**: leggi il campo `transcript_path` che [hooks](/docs/it/hooks#common-input-fields) e [comandi della riga di stato](/docs/it/statusline#available-data) ricevono come input. Un hook `SessionEnd` può archiviare la trascrizione quando una sessione termina.
* **Incorpora Claude in un'app TypeScript o Python**: usa l'[Agent SDK](/docs/it/agent-sdk/overview) per ricevere ogni messaggio a livello di programmazione.

L'esempio di seguito utilizza la seconda interfaccia. Invia un prompt di follow-up a una sessione esistente e legge la risposta con `jq`:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Dove vengono archiviati i trascritti
</h3>

Per impostazione predefinita, Claude Code archivia i trascritti come JSONL in `~/.claude/projects/<project>/<session-id>.jsonl`, dove `<project>` è il percorso della directory di lavoro con caratteri non alfanumerici sostituiti da `-`. Per una directory di lavoro il cui nome convertito supera i 200 caratteri, Claude Code tronca il nome a 200 caratteri e aggiunge un hash del percorso completo, in modo che il nome della directory rimanga entro i limiti del file system.

Ogni riga è un oggetto JSON per un messaggio, uso dello strumento o voce di metadati. Il formato della voce è interno a Claude Code e cambia tra le versioni, quindi gli script che analizzano direttamente questi file possono interrompersi in qualsiasi rilascio. Per costruire sui dati della sessione, utilizza `/export` o le [interfacce di script](#access-conversations-from-scripts) invece.

La posizione, la conservazione e il comportamento di scrittura sono configurabili:

| Per                                                                                                                   | Imposta                                                                                     | Dove                                                      |
| --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| Sposta l'archiviazione da `~/.claude`                                                                                 | [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars)                                                         | Variabile di ambiente                                     |
| [Assegna un nome alla directory `<project>` tu stesso](#name-the-project-directory-yourself)                          | [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/it/env-vars)                                              | Variabile di ambiente                                     |
| Cambia la conservazione di 30 giorni                                                                                  | [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays)                             | `settings.json`                                           |
| Imposta un limite di età per [i trascritti di Claude Desktop e Cowork](/docs/it/claude-directory#cleaned-up-automatically) | [`desktopSessionCleanupPeriodDays`](/docs/it/settings-reference#desktopsessioncleanupperioddays) | Impostazioni utente, impostazioni gestite, o `--settings` |
| Sopprimere le scritture di trascritto in tutte le modalità                                                            | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/it/env-vars)                                           | Variabile di ambiente                                     |
| Sopprimere le scritture per un'esecuzione non interattiva                                                             | [`--no-session-persistence`](/docs/it/cli-reference)                                             | Flag CLI con `claude -p`                                  |

<h3 id="delete-session-data">
  Eliminare i dati della sessione
</h3>

I trascritti invecchiano secondo le [regole di pulizia della conservazione](/docs/it/claude-directory#cleaned-up-automatically). Per eliminare i trascritti di un progetto e lo stato correlato più rapidamente, esegui [`claude project purge`](/docs/it/claude-directory#clear-local-data). Se elimini una [sessione in background](/docs/it/agent-view) con [`claude rm <id>`](/docs/it/agent-view#what-deleting-a-session-removes), il suo trascritto rimane su disco e rimane disponibile tramite `claude --resume`.

<h3 id="name-the-project-directory-yourself">
  Assegna un nome alla directory del progetto tu stesso
</h3>

Per impostazione predefinita, Claude Code deriva il nome `<project>` dall'intero percorso della directory di lavoro. Per scegliere il nome tu stesso, imposta `CLAUDE_CODE_PROJECT_DIR_NAME` insieme a `CLAUDE_CONFIG_DIR`. Claude Code quindi archivia i trascritti di quella sessione e la [memoria automatica](/docs/it/memory#auto-memory) sotto il tuo nome. Questo è adatto a un host che incorpora Claude Code e assegna a ogni sessione la propria directory di configurazione. Richiede Claude Code v2.1.234 o successivo.

Ad esempio, questo avvio mantiene i dati del tenant A in `/srv/tenant-a` e assegna un nome alla sua directory del progetto `work`:

```bash theme={null}
CLAUDE_CONFIG_DIR=/srv/tenant-a CLAUDE_CODE_PROJECT_DIR_NAME=work claude
```

Claude Code scrive i trascritti della sessione in `/srv/tenant-a/projects/work/` e la sua memoria automatica in `/srv/tenant-a/projects/work/memory/`, indipendentemente dalla directory di lavoro.

Tre regole si applicano quando lo imposti:

* **Imposta anche `CLAUDE_CONFIG_DIR`**: il nome non varia con la directory di lavoro, quindi sotto il valore predefinito `~/.claude` unirebbe i trascritti e la memoria automatica di ogni progetto in una directory. Claude Code ignora `CLAUDE_CODE_PROJECT_DIR_NAME` quando `CLAUDE_CONFIG_DIR` non è impostato.
* **Usa 1-64 lettere, cifre, trattini o sottolineature**: non utilizzare un nome di dispositivo Windows come `con`. Claude Code ignora qualsiasi altro valore e utilizza il nome derivato.
* **Impostalo nell'ambiente shell che avvia `claude`**: Claude Code lo legge una sola volta all'avvio da quell'ambiente, quindi un blocco `env` in un file di impostazioni non può impostarlo.

Una volta che hai assegnato un nome alla directory del progetto di una directory di configurazione, continua ad avviare con quel nome. Se avvii Claude Code con lo stesso `CLAUDE_CONFIG_DIR` ma senza `CLAUDE_CODE_PROJECT_DIR_NAME`, legge e scrive di nuovo la directory derivata. Le sessioni archiviate con il tuo nome rimangono su disco: premi `Ctrl+A` nel [selettore di sessione](#use-the-session-picker) per elencare le sessioni da ogni directory del progetto in quella directory di configurazione, inclusa quella fissata, e in qualunque modo tu avvii, [`claude --resume <session-id>`](#resume-a-session) trova una sessione archiviata con uno dei due nomi.

<h2 id="see-also">
  Vedi anche
</h2>

Queste pagine coprono la meccanica correlata delle sessioni e del parallelismo:

* [Worktrees](/docs/it/worktrees): esegui sessioni parallele isolate su rami separati
* [Checkpointing](/docs/it/checkpointing): riavvolgi il codice e la conversazione a un punto precedente
* [Context window](/docs/it/context-window): cosa riempie il contesto e cosa sopravvive alla compattazione
* [Non-interactive mode](/docs/it/headless): comportamento della sessione in `claude -p`
