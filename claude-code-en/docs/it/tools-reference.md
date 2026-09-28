> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento degli strumenti

> Riferimento completo per gli strumenti che Claude Code può utilizzare, inclusi i requisiti di autorizzazione e il comportamento per strumento.

Claude Code ha accesso a un set di strumenti integrati che lo aiutano a comprendere e modificare la tua base di codice. I nomi degli strumenti sono le stringhe esatte che utilizzi nelle [regole di autorizzazione](/docs/it/permissions#tool-specific-permission-rules), negli [elenchi di strumenti dei subagent](/docs/it/sub-agents) e nei [matcher di hook](/docs/it/hooks).

Per controllare quali strumenti Claude può utilizzare e quando chiede prima, configura le [regole di autorizzazione](/docs/it/permissions#tool-specific-permission-rules) nelle tue impostazioni, negli [hook](/docs/it/hooks) o nell'[elenco di strumenti di un subagent](/docs/it/sub-agents#supported-frontmatter-fields). Vedi [Configurare gli strumenti con regole di autorizzazione e hook](#configure-tools-with-permission-rules-and-hooks) per ogni luogo che accetta un nome di strumento.

Per aggiungere strumenti personalizzati, connetti un [server MCP](/docs/it/mcp). Per estendere Claude con flussi di lavoro basati su prompt riutilizzabili, scrivi una [skill](/docs/it/skills), che viene eseguita attraverso lo strumento `Skill` esistente piuttosto che aggiungere una nuova voce di strumento.

<Info>
  Nei piani Pro, Max e Team, Claude Code avvia le sessioni in [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), dove un classificatore decide la maggior parte di questi prompt invece di te. La colonna `Permission required` mostra se lo strumento richiede prompt in [modalità manuale](/docs/it/permission-modes) per i percorsi all'interno della directory di lavoro. Gli strumenti di accesso ai file contrassegnati No, inclusi `Read`, `Grep` e `Glob`, richiedono comunque prompt per i percorsi al di fuori della [directory di lavoro e delle directory aggiuntive](/docs/it/permissions#working-directories). `Bash` è contrassegnato Yes ma esegue un set integrato di [comandi di sola lettura](/docs/it/permissions#read-only-commands) senza richiedere prompt.
</Info>

| Strumento              | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Autorizzazione richiesta |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------- |
| `Agent`                | Genera un [subagent](/docs/it/sub-agents) con la propria finestra di contesto per gestire un'attività. Con i [team di agenti](/docs/it/agent-teams) abilitati, una chiamata che porta un `name` può avviare un [compagno di squadra](/docs/it/agent-teams#how-claude-starts-agent-teams) invece. Vedi [Comportamento dello strumento Agent](#agent-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | No                       |
| `Artifact`             | Pubblica un file HTML o Markdown come [artifact](/docs/it/artifacts): una pagina privata e interattiva su claude.ai. Puoi condividerla con un link pubblico, o all'interno della tua organizzazione nei piani Team ed Enterprise, dove la condivisione pubblica richiede a un Owner di [abilitarla](/docs/it/artifacts#control-public-sharing). Richiede un piano Pro, Max, Team o Enterprise e autenticazione `/login`; vedi [Disponibilità](/docs/it/artifacts#availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Yes                      |
| `AskUserQuestion`      | Pone domande a scelta multipla per raccogliere requisiti o chiarire ambiguità. Le domande rimangono aperte finché non le rispondi per impostazione predefinita. Vedi [Comportamento dello strumento AskUserQuestion](#askuserquestion-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | No                       |
| `Bash`                 | Esegue comandi shell nel tuo ambiente. Vedi [Comportamento dello strumento Bash](#bash-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Yes                      |
| `CronCreate`           | Pianifica un prompt ricorrente o una tantum all'interno della sessione corrente. Le attività hanno ambito di sessione e vengono ripristinate su `--resume` o `--continue` se non scadute. Vedi [attività pianificate](/docs/it/scheduled-tasks)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | No                       |
| `CronDelete`           | Annulla un'attività pianificata per ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | No                       |
| `CronList`             | Elenca tutte le attività pianificate nella sessione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | No                       |
| `Edit`                 | Effettua modifiche mirate a file specifici. Vedi [Comportamento dello strumento Edit](#edit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Yes                      |
| `EndConversation`      | Termina la sessione, in rari casi di input abusivo sostenuto o quando chiedi a Claude di dimostrare lo strumento. Richiede Claude Code v2.1.213 o successivo. Vedi [Comportamento dello strumento EndConversation](#endconversation-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | No                       |
| `EnterPlanMode`        | Passa a Plan Mode per progettare un approccio prima di codificare                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | No                       |
| `EnterWorktree`        | Crea un [git worktree](/docs/it/worktrees) isolato e vi entra. Passa un `path` per entrare in un worktree esistente invece di crearne uno nuovo. Al primo ingresso il target può essere un worktree del repository corrente o, in uno spazio di lavoro multi-repo, di un repository annidato al suo interno. Prima della v2.1.203, un worktree di un repository annidato era rifiutato. Un `path` al di fuori di `.claude/worktrees/` richiede la tua approvazione prima di entrare, poiché sposta la directory di lavoro della sessione e l'accesso in scrittura a quella posizione. La creazione di nuovi worktree e i percorsi sotto `.claude/worktrees/` non richiedono prompt. Prima della v2.1.206, Claude entrava nei percorsi al di fuori di `.claude/worktrees/` senza un prompt. Da una sessione worktree, o da un subagent con una directory di lavoro fissata come [`isolation: worktree`](/docs/it/sub-agents#supported-frontmatter-fields), è disponibile solo il modulo `path` e il target deve trovarsi sotto `.claude/worktrees/` del repository della sessione                         | Yes                      |
| `ExitPlanMode`         | Presenta un piano per l'approvazione ed esce da Plan Mode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Yes                      |
| `ExitWorktree`         | Esce da una sessione worktree e ritorna alla directory originale. Non disponibile per i subagent che già vengono eseguiti nella propria directory di lavoro, come con [`isolation: worktree`](/docs/it/sub-agents#supported-frontmatter-fields)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | No                       |
| `Glob`                 | Trova file in base alla corrispondenza di pattern. Assente per impostazione predefinita su macOS, Linux e WSL. Vedi [Comportamento dello strumento Glob](#glob-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | No                       |
| `Grep`                 | Cerca pattern nei contenuti dei file. Assente per impostazione predefinita su macOS, Linux e WSL. Vedi [Comportamento dello strumento Grep](#grep-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | No                       |
| `ListAgents`           | Elenca gli agenti con cui Claude può inviare messaggi con `SendMessage`: subagent nella sessione, compagni di squadra del [team di agenti](/docs/it/agent-teams), le tue altre sessioni locali di Claude Code e, mentre questa sessione è connessa a [Remote Control](/docs/it/remote-control), le tue sessioni di [Claude Code sul web](/docs/it/claude-code-on-the-web) e le tue sessioni Remote Control su altre macchine. Supporta il comando `/list-agents`. Vedi [messaggistica tra sessioni](/docs/it/cross-session-messaging). Richiede Claude Code v2.1.224 o successivo e appare solo nelle sessioni in cui la [messaggistica tra sessioni è abilitata](/docs/it/cross-session-messaging#availability). Le righe dei compagni di squadra e la prima riga che mostra il nome della sessione stessa richiedono v2.1.239 o successivo                                                                                                                                                                                                                                                                            | No                       |
| `ListMcpResourcesTool` | Elenca le risorse esposte dai [server MCP](/docs/it/mcp) connessi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | No                       |
| `LSP`                  | Intelligenza del codice tramite language server: salta alle definizioni, trova riferimenti, segnala errori di tipo e avvisi. Vedi [Comportamento dello strumento LSP](#lsp-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | No                       |
| `Monitor`              | Esegue un comando in background e restituisce ogni riga di output a Claude, in modo che possa reagire alle voci di log, ai cambiamenti di file o allo stato sottoposto a polling a metà conversazione. Può anche aprire un WebSocket e trattare ogni messaggio in arrivo come un evento. Vedi [Strumento Monitor](#monitor-tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Yes                      |
| `NotebookEdit`         | Modifica le celle del notebook Jupyter. Vedi [Comportamento dello strumento NotebookEdit](#notebookedit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Yes                      |
| `PowerShell`           | Esegue comandi PowerShell in modo nativo. Vedi [Strumento PowerShell](#powershell-tool) per la disponibilità                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Yes                      |
| `PushNotification`     | Invia una notifica desktop e un push telefonico quando [Remote Control](/docs/it/remote-control) è connesso, in modo che un'attività a lunga esecuzione o un'[attività pianificata](/docs/it/scheduled-tasks) possa raggiungerti quando ti allontani. La consegna push viene eseguita tramite l'infrastruttura ospitata da Anthropic, che non è accessibile da Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform o Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | No                       |
| `Read`                 | Legge il contenuto dei file. Vedi [Comportamento dello strumento Read](#read-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | No                       |
| `ReadMcpResourceTool`  | Legge una risorsa MCP specifica per URI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | No                       |
| `RemoteTrigger`        | Crea, aggiorna, esegue ed elenca [Routine](/docs/it/routines) su claude.ai. Supporta il comando `/schedule`. Il [riferimento di input `RemoteTrigger`](/docs/it/agent-sdk/typescript#remotetrigger) documenta ogni azione e le politiche organizzative che rimuovono lo strumento. Le Routine vivono su claude.ai e richiedono un piano Pro, Max, Team o Enterprise, quindi questo strumento non è accessibile da Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform o Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | No                       |
| `ReportFindings`       | Segnala i risultati della revisione del codice come un elenco strutturato, con un file, un riepilogo e uno scenario di errore per ogni risultato, in modo che Claude Code possa renderli invece di stamparli come testo. Claude lo chiama quando le istruzioni di revisione del codice attive lo dicono. Richiede Claude Code v2.1.196 o successivo. A partire dalla v2.1.199, un risultato può anche portare uno slug `category` opzionale, come `correctness` o `test-coverage`, mostrato accanto alla posizione del file nell'elenco renderizzato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | No                       |
| `ScheduleWakeup`       | Riprogramma l'iterazione successiva di un [`/loop` auto-paced](/docs/it/scheduled-tasks#let-claude-choose-the-interval). Claude lo chiama alla fine di ogni iterazione per scegliere quando viene eseguita la successiva, tra un minuto e un'ora; non lo chiami direttamente. Per terminare il loop invece, Claude lo chiama con `stop: true`, che annulla il wakeup in sospeso. Il campo `stop` richiede Claude Code v2.1.202 o successivo. Il wakeup in sospeso appare in `session_crons` nell'[input Stop hook](/docs/it/hooks#stop-input)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | No                       |
| `SendFeedback`         | Redige un rapporto di feedback su Claude Code, coprendo un problema di prodotto o il comportamento di Claude stesso nella sessione, e lo mette in coda sulla tua macchina per te da rivedere. Claude Code non invia nulla finché non scegli di inviare la bozza. Vedi [Comportamento dello strumento SendFeedback](#sendfeedback-tool-behavior). Richiede Claude Code v2.1.238 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | No                       |
| `SendMessage`          | Invia un messaggio a un altro agente: un compagno di squadra del [team di agenti](/docs/it/agent-teams), un [subagent che riprende](/docs/it/sub-agents#resume-subagents) per ID agente o nome, o una delle tue altre sessioni di Claude Code, su questa macchina o oltre. La messaggistica di altre sessioni richiede Claude Code v2.1.224 o successivo. La [messaggistica tra sessioni](/docs/it/cross-session-messaging) copre quali sessioni Claude può raggiungere, [come appare un messaggio quando arriva](/docs/it/cross-session-messaging#what-a-message-looks-like) e [come Claude riceve un avviso quando un'altra sessione diventa inattiva](/docs/it/cross-session-messaging#get-a-notice-when-another-session-goes-idle). Claude può includere un input `summary` opzionale, tipicamente 5-10 parole, che Claude Code mostra come anteprima su una riga. Quando Claude lo omette su un [messaggio di testo semplice](/docs/it/cross-session-messaging#limitations), Claude Code utilizza la prima riga del messaggio come riepilogo. Claude Code tronca un riepilogo più lungo di 200 caratteri con un'ellissi | No                       |
| `SendUserFile`         | Invia file dalla sessione a te con una didascalia opzionale, in modo che un rapporto generato, un diagramma, uno screenshot o un artifact costruito raggiunga il tuo dispositivo invece di essere solo menzionato nella trascrizione. A partire dalla v2.1.196, l'input `display` opzionale controlla la presentazione: `render` apre il file inline nel client, `attach` mostra solo una scheda di download, e quando non impostato il client decide in base al tipo di file. Disponibile quando un client [Remote Control](/docs/it/remote-control) è connesso o in una [sessione cloud](/docs/it/claude-code-on-the-web). La consegna viene eseguita tramite l'infrastruttura ospitata da Anthropic, quindi lo strumento non è disponibile su Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry                                                                                                                                                                                                                                                                                       | No                       |
| `ShareOnboardingGuide` | Carica `ONBOARDING.md` e restituisce un link di condivisione che i compagni di squadra possono aprire in Claude Code. Chiamato da `/team-onboarding` dopo che la guida è stata scritta. Disponibile per gli abbonati a claude.ai nei piani Pro, Max, Team ed Enterprise                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Yes                      |
| `Skill`                | Esegue una [skill](/docs/it/skills#control-who-invokes-a-skill) all'interno della conversazione principale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Yes                      |
| `SubagentHandback`     | Consegna il rapporto finale di un subagent a qualunque conversazione riceva il risultato di quel subagent. Fornito solo in [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), ai subagent che lo strumento Agent esegue localmente diversi dai [fork](/docs/it/sub-agents#fork-the-current-conversation), e disponibile nel CLI del terminale, nelle estensioni IDE, nelle sessioni cloud e nell'Agent SDK; il classificatore esamina il rapporto prima che venga consegnato. Richiede Claude Code v2.1.271 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | No                       |
| `TaskCreate`           | Crea una nuova attività nell'elenco delle attività. Fornito per impostazione predefinita solo sui modelli elencati in [Disponibilità dello strumento Task](#task-tool-availability) e su altri modelli quando accetti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | No                       |
| `TaskGet`              | Recupera i dettagli completi per un'attività specifica. Fornito per impostazione predefinita solo sui modelli elencati in [Disponibilità dello strumento Task](#task-tool-availability) e su altri modelli quando accetti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | No                       |
| `TaskList`             | Elenca tutte le attività con il loro stato attuale. Fornito per impostazione predefinita solo sui modelli elencati in [Disponibilità dello strumento Task](#task-tool-availability) e su altri modelli quando accetti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | No                       |
| `TaskOutput`           | Recupera l'output da un'attività in background. Deprecato a favore di `Read` sul percorso del file di output dell'attività. Quando nessuna attività corrisponde all'ID, l'errore elenca gli agenti di background in esecuzione per ID e descrizione. Prima della v2.1.203, l'errore nominava solo l'ID mancante                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | No                       |
| `TaskStop`             | Arresta un'attività di background in esecuzione per ID. Accetta anche un compagno di squadra del [team di agenti](/docs/it/agent-teams) o un agente di background denominato per ID agente o nome. Prima della v2.1.198, accettava solo un ID di attività di background. Quando nessuna attività corrisponde all'ID, l'errore elenca gli agenti di background in esecuzione per ID e descrizione, inclusi gli agenti che un altro agente ha generato. Prima della v2.1.203, l'errore elencava i compagni di squadra in esecuzione e gli agenti denominati ma non gli agenti di background che un altro agente ha generato, quindi non potevano essere identificati o arrestati dalla conversazione principale                                                                                                                                                                                                                                                                                                                                                                                       | No                       |
| `TaskUpdate`           | Aggiorna lo stato dell'attività, le dipendenze, i dettagli o elimina le attività. Fornito per impostazione predefinita solo sui modelli elencati in [Disponibilità dello strumento Task](#task-tool-availability) e su altri modelli quando accetti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | No                       |
| `TodoWrite`            | Gestisce l'elenco di controllo delle attività della sessione. Disabilitato per impostazione predefinita a favore di `TaskCreate`, `TaskGet`, `TaskList` e `TaskUpdate`. Imposta `CLAUDE_CODE_ENABLE_TASKS=0` per riabilitarlo nelle [sessioni che hanno gli strumenti di tracciamento delle attività](#task-tool-availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | No                       |
| `ToolSearch`           | Cerca e carica strumenti differiti quando la [ricerca di strumenti](/docs/it/mcp#scale-with-mcp-tool-search) è abilitata                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | No                       |
| `WaitForMcpServers`    | Attende uno o più [server MCP](/docs/it/mcp) che si stanno ancora connettendo in background, in modo che una richiesta possa utilizzare i loro strumenti senza riavviare la sessione. Claude lo chiama quando un server necessario non è ancora connesso. Appare solo quando la [ricerca di strumenti](/docs/it/mcp#scale-with-mcp-tool-search) è disabilitata, poiché `ToolSearch` gestisce l'attesa quando è abilitata                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | No                       |
| `WebFetch`             | Recupera il contenuto da un URL specificato. Vedi [Comportamento dello strumento WebFetch](#webfetch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Yes                      |
| `WebSearch`            | Esegue ricerche web. Vedi [Comportamento dello strumento WebSearch](#websearch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Yes                      |
| `Workflow`             | Esegue un [flusso di lavoro dinamico](/docs/it/workflows): uno script che orchestra molti subagent in background e restituisce un risultato consolidato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Yes                      |
| `Write`                | Crea o sovrascrive file. Vedi [Comportamento dello strumento Write](#write-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Yes                      |

<h2 id="configure-tools-with-permission-rules-and-hooks">
  Configurare gli strumenti con regole di autorizzazione e hook
</h2>

Per la maggior parte, Claude decide quando utilizzare questi strumenti e non è necessario nominarli direttamente quando si interagisce con Claude. Si fa riferimento ai nomi degli strumenti direttamente quando si definiscono le autorizzazioni e altre configurazioni:

* in [`permissions.allow`](/docs/it/settings-reference#permissions-allow) e [`permissions.deny`](/docs/it/settings-reference#permissions-deny) nelle impostazioni, e nell'interfaccia `/permissions`
* nei flag CLI [`--allowedTools` e `--disallowedTools`](/docs/it/cli-reference)
* nelle opzioni [`allowedTools` e `disallowedTools`](/docs/it/agent-sdk/permissions#allow-and-deny-rules) dell'Agent SDK
* nel frontmatter [`allowed-tools`](/docs/it/skills#frontmatter-reference) di una skill
* nella condizione [`if`](/docs/it/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field) di un hook

Tutti questi accettano lo stesso formato di regola, `ToolName(specifier)`. Lo specifier dipende dallo strumento, e diversi strumenti condividono un formato:

| Formato della regola           | Si applica a              | Dettagli                                                                                 |
| :----------------------------- | :------------------------ | :--------------------------------------------------------------------------------------- |
| `Bash(npm run *)`              | Bash, Monitor             | [Corrispondenza del pattern di comando](/docs/it/permissions#bash)                            |
| `PowerShell(Get-ChildItem *)`  | PowerShell                | [Corrispondenza del pattern di comando](/docs/it/permissions#powershell)                      |
| `Read(~/secrets/**)`           | Read, Grep, Glob, LSP     | [Corrispondenza del pattern di percorso](/docs/it/permissions#read-and-edit)                  |
| `Edit(/src/**)`                | Edit, Write, NotebookEdit | [Corrispondenza del pattern di percorso](/docs/it/permissions#read-and-edit)                  |
| `Skill(deploy *)`              | Skill                     | [Corrispondenza del nome della skill](/docs/it/skills#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)`               | Agent                     | [Corrispondenza del tipo di subagent](/docs/it/permissions#agent-subagents)                   |
| `WebFetch(domain:example.com)` | WebFetch                  | [Corrispondenza del dominio](/docs/it/permissions#webfetch)                                   |
| `WebSearch`                    | WebSearch                 | Nessuno specifier; consenti o nega lo strumento nel suo insieme                          |

Gli strumenti non elencati qui, come `ExitPlanMode` o `ShareOnboardingGuide`, accettano solo il nome dello strumento nudo senza specifier.

Una regola di autorizzazione `Edit(...)` concede anche l'accesso in lettura allo stesso percorso, quindi non è necessaria una regola `Read(...)` corrispondente. Una regola di negazione `Read(...)` blocca anche gli strumenti Edit e Write sullo stesso percorso, inclusa la creazione di un nuovo file lì, perché entrambi gli strumenti modificano il contenuto che Claude deve essere in grado di leggere di nuovo. Il controllo di negazione `Read` richiede Claude Code v2.1.208 o successivo sulle modifiche, e v2.1.228 o successivo sulle scritture.

I campi `matcher` degli hook utilizzano nomi di strumenti nudi, non il formato tra parentesi. Vedere [matcher patterns](/docs/it/hooks#matcher-patterns) per le regole di corrispondenza. Per i nomi dei campi che ogni strumento passa a `tool_input` negli hook, vedere il [riferimento di input PreToolUse](/docs/it/hooks#pretooluse-input).

<h2 id="agent-tool-behavior">
  Comportamento dello strumento Agent
</h2>

Lo strumento Agent avvia un subagent in una finestra di contesto separata. Il subagent lavora autonomamente attraverso il suo compito, quindi restituisce il suo risultato alla conversazione principale. Il genitore non vede le chiamate agli strumenti intermedie o gli output del subagent, solo quel risultato finale. Con i [team di agent](/docs/it/agent-teams) abilitati, una chiamata che contiene un `name` può avviare un [collega del team](/docs/it/agent-teams#how-claude-starts-agent-teams), che riferisce attraverso i messaggi del team piuttosto che restituendo un risultato.

Per limitare quanti turni esegue un subagent, impostare `maxTurns` nella [definizione del subagent](/docs/it/sub-agents#supported-frontmatter-fields). Quando il subagent raggiunge il limite, Claude Code contrassegna il risultato restituito come output parziale, e Claude può [riprendere il subagent](/docs/it/sub-agents#resume-subagents) per continuare.

Lo stesso strumento Agent avvia anche [subagent biforcati](/docs/it/sub-agents#fork-the-current-conversation) ovunque la [modalità fork](/docs/it/sub-agents#turn-fork-mode-on-or-off) sia attiva. Un fork eredita la conversazione principale completa invece di iniziare da zero, viene eseguito in background a parte i [casi che rimangono in primo piano](/docs/it/sub-agents#run-subagents-in-foreground-or-background), e continua a visualizzare i prompt di autorizzazione nel vostro terminale. Il resto di questa sezione descrive i subagent non-fork.

Quali strumenti un subagent non-fork può utilizzare dipende dai campi `tools` e `disallowedTools` nella [definizione del subagent](/docs/it/sub-agents):

* **Nessun campo impostato**: il subagent eredita ogni [strumento disponibile per i subagent](/docs/it/sub-agents#available-tools).
* **Solo `tools`**: il subagent ottiene solo gli strumenti elencati.
* **Solo `disallowedTools`**: il subagent ottiene ogni strumento principale tranne quelli elencati.
* **Entrambi impostati**: `disallowedTools` ha la precedenza. Uno strumento elencato in entrambi viene rimosso.

In ogni caso, l'insieme risolto è limitato agli [strumenti disponibili per i subagent](/docs/it/sub-agents#available-tools): uno strumento che non è disponibile per i subagent non viene mai concesso, anche quando elencato in `tools`. Dove le condizioni nella voce della tabella degli strumenti `SubagentHandback` si mantengono, Claude Code fornisce anche al subagent quello strumento, anche se lo omettete da `tools` o lo elencate in `disallowedTools`.

Se ogni voce nell'elenco `tools` di un subagent non corrisponde a uno strumento utilizzabile, lo strumento Agent di solito restituisce un errore che nomina le voci invece di avviare il subagent; vedere [Agent sarebbe avviato con zero strumenti](/docs/it/errors#agent-would-be-spawned-with-zero-tools) per il messaggio e come correggere ogni voce.

L'avvio del subagent non richiede di per sé il permesso. Claude Code controlla le chiamate agli strumenti del subagent rispetto alle vostre regole di autorizzazione mentre viene eseguito.

Il luogo in cui vedete i prompt di autorizzazione di un subagent dipende dal fatto che venga eseguito in primo piano o in background. Claude Code esegue i subagent in background per impostazione predefinita, a parte i [casi che vengono eseguiti in primo piano](/docs/it/sub-agents#run-subagents-in-foreground-or-background).

* **Subagent in primo piano** mostrano gli stessi prompt di autorizzazione che vedreste nella conversazione principale, nel momento in cui avviene ogni chiamata allo strumento.
* **Subagent in background** visualizzano i prompt di autorizzazione nella vostra sessione principale a partire dalla v2.1.186. Il prompt nomina quale subagent sta chiedendo, e premere Esc nega quella singola chiamata allo strumento senza interrompere il subagent. Prima della v2.1.186, i subagent in background negavano automaticamente qualsiasi chiamata allo strumento che altrimenti richiederebbe un prompt e continuavano senza quello strumento.

Per [limitare ciò che un subagent può raggiungere](/docs/it/sub-agents#control-subagent-capabilities) in primo luogo, restringere il suo campo `tools`, ad esempio lasciando Bash fuori dall'elenco, o impostare regole di negazione nelle vostre impostazioni.

<h2 id="askuserquestion-tool-behavior">
  Comportamento dello strumento AskUserQuestion
</h2>

Claude utilizza `AskUserQuestion` per farvi domande a scelta multipla quando ha bisogno di una decisione o di un chiarimento. Rispondete selezionando un'opzione, oppure digitate il vostro testo tramite la riga `Other` o il campo delle note.

Quando rispondete digitando il vostro testo, Claude Code trasmette la risposta con una formulazione neutra in modo che Claude segua ciò che avete scritto, inclusa una richiesta di attendere o di spiegare prima.

<h3 id="question-auto-continue-timeout">
  Timeout di continuazione automatica delle domande
</h3>

Le domande rimangono aperte finché non rispondete. Se desiderate che una domanda che lasciate senza risposta si chiuda eventualmente e permetta a Claude di continuare senza di voi, impostate l'impostazione [`askUserQuestionTimeout`](/docs/it/settings-reference#askuserquestiontimeout) su `60s`, `5m`, o `10m`, sia nel vostro `settings.json` utente che dalla riga **Question auto-continue timeout** in `/config`.

Dopo che una domanda rimane così a lungo senza input, la finestra di dialogo si chiude automaticamente: invia tutte le opzioni che avevate già selezionato e comunica a Claude che potreste essere lontani dalla tastiera, quindi Claude procede secondo il suo giudizio e può riproporre la domanda in seguito. Vedete un conto alla rovescia negli ultimi 20 secondi. Premete un tasto qualsiasi per riavviare il timer; sui terminali che segnalano lo stato attivo, il passaggio alla finestra lo riavvia anche.

Il timeout si applica solo alle domande a scelta multipla di `AskUserQuestion`; i prompt di autorizzazione, inclusa l'approvazione del piano, non si risolvono mai automaticamente in caso di inattività.

<h2 id="bash-tool-behavior">
  Comportamento dello strumento Bash
</h2>

Lo strumento Bash esegue ogni comando in un processo separato.

<h3 id="what-persists-between-commands">
  Cosa persiste tra i comandi
</h3>

* Quando Claude esegue `cd` nella sessione principale, la nuova directory di lavoro viene mantenuta per i successivi comandi Bash finché rimane all'interno della directory del progetto o di una [directory di lavoro aggiuntiva](/docs/it/permissions#working-directories) che hai aggiunto con `--add-dir`, `/add-dir`, o `additionalDirectories` nelle impostazioni. Questo include i comandi che Claude esegue in risposta ai tuoi messaggi successivi.
  * Le sessioni dei subagent non mantengono mai i cambiamenti della directory di lavoro.
  * Se `cd` finisce al di fuori di quelle directory, Claude Code ripristina la directory del progetto e aggiunge `Shell cwd was reset to <dir>` al risultato dello strumento.
  * Per disabilitare questo mantenimento in modo che ogni comando Bash inizi nella directory del progetto, imposta `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1`.
* Le variabili di ambiente non persistono. Un `export` in un comando non sarà disponibile nel successivo.
* Gli alias e le funzioni shell definiti nel tuo file di avvio della shell sono disponibili. All'inizio della sessione, Claude Code esegue `~/.zshrc`, `~/.bashrc`, o `~/.profile` a seconda della tua shell, cattura gli alias, le funzioni e le opzioni shell risultanti, e le applica a ogni comando Bash.

Attiva il tuo virtualenv o ambiente conda prima di avviare Claude Code. Per fare in modo che le variabili di ambiente persistano tra i comandi Bash, imposta [`CLAUDE_ENV_FILE`](/docs/it/env-vars) su uno script shell prima di avviare Claude Code, oppure usa un [hook SessionStart](/docs/it/hooks#persist-environment-variables) per popolarlo dinamicamente.

<h3 id="timeout-and-output-limits">
  Limiti di timeout e output
</h3>

Ogni comando viene eseguito con un timeout, e Claude lo gestisce: quando ha bisogno di più tempo del valore predefinito per un comando, passa il parametro `timeout` con quella chiamata — non imposti mai un timeout per singolo comando. Due [variabili di ambiente](/docs/it/env-vars) limitano quello che Claude ottiene:

* `BASH_DEFAULT_TIMEOUT_MS` — il valore predefinito quando Claude non passa alcun timeout; due minuti per impostazione predefinita
* `BASH_MAX_TIMEOUT_MS` — con il valore predefinito, imposta il limite massimo che limita qualsiasi richiesta di Claude: il limite massimo effettivo è il maggiore dei due, dieci minuti per impostazione predefinita

<h4 id="output-limits">
  Limiti di output
</h4>

Claude Code trasmette l'output di un comando a un file di lavoro mentre il comando viene eseguito; un comando il cui output supera 5 GB viene terminato. Quando il comando termina, Claude Code legge l'output da quel file, fino alla finestra di lettura descritta di seguito. Quanto dell'output raggiunge Claude inline dipende dal fatto che Claude Code tratti il risultato come un errore:

| Risultato | Cosa ottiene Claude                                                                                                                                                                                                                                                                |
| :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Valido    | Inline fino a circa 30.000 caratteri per impostazione predefinita; oltre quello, il percorso di un file salvato nella directory della sessione e troncato oltre 64 MiB, più un'anteprima fino ai primi 2.000 caratteri, e Claude legge o cerca il file quando ha bisogno del resto |
| Errore    | Inline fino a circa 10.000 caratteri; oltre quello, un estratto testa-coda di quella dimensione ricavato dalla finestra di lettura, senza percorso del file                                                                                                                        |

Un comando che esce con codice 1 conta come risultato valido per lo strumento Bash solo quando Claude Code riconosce il codice di uscita 1 come un risultato benigno per quel comando: `grep`, `rg`, `egrep`, `fgrep`, `find`, `diff`, `test`, e `[`, più `git diff` e `git grep`. Ogni altro comando che esce con codice 1 conta come errore, anche quando il codice di uscita 1 è un risultato informativo benigno: nessuna corrispondenza per `pgrep` e `jq -e`, file che differiscono per `cmp`.

[`BASH_MAX_OUTPUT_LENGTH`](/docs/it/env-vars) imposta quanti caratteri di output Claude Code legge dal file di lavoro nel risultato di un comando: 30.000 per impostazione predefinita, fino a un limite massimo fisso di 150.000. Aumentalo quando i tuoi comandi superano regolarmente quella finestra, come una build dettagliata o un log completo della suite di test. Aumentarlo allarga la finestra di lettura, che è anche la finestra da cui viene ricavato l'estratto di un comando non riuscito. Non aumenta i limiti inline: un risultato valido oltre il limite inline arriva come percorso del file più anteprima indipendentemente da questa variabile.

Per cambiare quanto di un risultato valido Claude riceve inline, imposta invece l'impostazione [`bashOutputMaxChars`](/docs/it/settings-reference#bashoutputmaxchars), fino a 128.000 caratteri. Dimensiona il limite inline e la finestra di lettura insieme, e Claude Code ignora quindi `BASH_MAX_OUTPUT_LENGTH`. Richiede Claude Code v2.1.261 o successivo.

<h3 id="background-commands">
  Comandi in background
</h3>

Per processi a lunga esecuzione come server di sviluppo o build di watch, Claude può impostare `run_in_background: true` per avviare il comando come attività in background e continuare a lavorare mentre viene eseguito. Elenca e arresta le attività in background con `/tasks`. Dopo averlo fermato lì, o da un client connesso come l'app desktop, Claude procede invece di aspettare. Se un subagent ha avviato il comando, è quel subagent che procede.

Un comando che un [subagent in foreground](/docs/it/sub-agents#run-subagents-in-foreground-or-background) ha avviato si ferma quando quel subagent fornisce la sua risposta finale. Un comando che la conversazione principale o un subagent in background ha avviato continua a essere eseguito dopo una risposta finale. In modalità non interattiva con il flag `-p`, i [comandi in background terminano poco dopo il risultato finale dell'esecuzione](/docs/it/headless#background-tasks-at-exit).

Quando un comando raggiunge il suo timeout senza terminare, Claude Code lo sposta in background invece di fermarlo, a meno che il comando non inizi con `sleep`. Claude continua a lavorare mentre il comando continua. Claude Code applica le stesse regole di durata a un comando spostato come a qualsiasi altro comando in background, quindi termina comunque il comando di un subagent in foreground alla risposta finale di quel subagent. Impostare [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=1`](/docs/it/env-vars#variables) disabilita lo spostamento automatico in background insieme al resto della funzionalità delle attività in background.

Il risultato di un comando spostato in background indica cosa è successo:

* Quando il timeout attiva lo spostamento, il risultato lo segnala esplicitamente: `Command did not complete within its 120s timeout and was moved to the background`, con i secondi che corrispondono al timeout applicato, seguito dall'ID dell'attività e dal percorso del file in cui viene scritto l'output.
* Un `cd`, `pushd`, `popd`, o `chdir` all'interno di un comando che viene spostato in background non viene mai mantenuto: il risultato afferma `Session cwd remains <dir>; directory changes made by the backgrounded command do not apply to subsequent commands.`, quindi Claude non agisce su un cambiamento di directory che non è avvenuto.

<h3 id="memory-limit-on-linux-and-wsl">
  Limite di memoria su Linux e WSL
</h3>

Su Linux e WSL, imposta [`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](/docs/it/env-vars#variables) su una dimensione come `4G` per limitare la memoria che i comandi degli strumenti Bash, PowerShell e [Monitor](#monitor-tool) possono utilizzare, in modo che una build fuori controllo non possa consumare la memoria di cui il resto della sessione ha bisogno. Richiede Claude Code v2.1.233 o successivo. Prima della v2.1.246, i comandi dello strumento Monitor venivano eseguiti al di fuori del limite.

* Scrivi la dimensione come un numero di byte o con un suffisso `K`, `M`, `G`, o `T`. Imposta `0`, `off`, `false`, `no`, o `none` per disattivare il limite. Claude Code ignora qualsiasi altro valore che non riesce a leggere come dimensione, come `4e9`.
* Claude Code conta tutti i comandi Bash, PowerShell e Monitor della sessione rispetto al limite unico, non ogni comando per conto proprio.
* Claude Code applica il limite con un cgroup di memoria. Quando non riesce a configurare il cgroup, i comandi vengono eseguiti senza limite, e il log di debug da `claude --debug` spiega perché.
* Dopo che il primo processo che Claude Code avvia ha attivato il limite, o lo ha disattivato a causa di un valore off o di una configurazione cgroup non riuscita, Claude Code mantiene quel risultato fino al riavvio. Per applicare un valore modificato o rimosso, o una configurazione corretta, avvia `claude` di nuovo.
* Quando i comandi non riescono a stare sotto il limite, il kernel termina un comando, e nulla nel suo risultato nomina il limite.

Claude Code può anche contare altri tipi di processi che avvia rispetto allo stesso limite. Imposta [`CLAUDE_CODE_TOOL_MEMORY_CGROUP_EXCLUDE`](/docs/it/env-vars#variables) su un elenco separato da virgole dei tipi da esentare dal limite; Claude Code applica il limite a ogni tipo non nel tuo elenco. Impostalo su `none` per limitare ogni tipo, o su `all-new` per limitare solo i comandi Bash, PowerShell e Monitor tool. Richiede Claude Code v2.1.246 o successivo. I tipi che puoi nominare:

* `mcp`: [server MCP](/docs/it/mcp) locali
* `lsp`: [language server](#lsp-tool-behavior)
* `hooks`: comandi [hook](/docs/it/hooks)
* `plugin`: comandi che i [plugin](/docs/it/plugins/overview) eseguono
* `helper`: comandi helper di Claude Code, come `git`
* `agent`: processi Claude Code figlio, come [agent teammate](/docs/it/agent-teams)

Qualunque cosa tu elenchi, queste regole si applicano:

* **Nomi sconosciuti**: Claude Code ignora i nomi che non riconosce
* **Bash, PowerShell e Monitor**: Claude Code mantiene i comandi Bash, PowerShell e Monitor tool sotto il limite qualunque cosa tu elenchi
* **Variabile non impostata**: Claude Code prende l'insieme di altri tipi limitati dalla configurazione che Anthropic fornisce dal server, e quell'insieme può cambiare nel tempo, quindi imposta la variabile quando hai bisogno di un insieme che non cambia
* **Hook con gating delle autorizzazioni**: anche con ogni tipo limitato, Claude Code esclude dal limite un hook che può bloccare o cambiare l'esito di un'azione, e qualsiasi server MCP che tale hook chiama, quindi il kernel che termina un hook con gating delle autorizzazioni non può consentire l'azione che stava bloccando

<h2 id="edit-tool-behavior">
  Comportamento dello strumento Edit
</h2>

Lo strumento Edit esegue la sostituzione esatta di stringhe. Accetta un `old_string` e un `new_string` e sostituisce il primo con il secondo. Non utilizza regex o fuzzy matching.

Tre controlli devono superare affinché una modifica venga applicata. Prima di qualsiasi controllo, un percorso corrispondente a una [regola di negazione `Read`](/docs/it/permissions#tool-specific-permission-rules) viene rifiutato, inclusa la creazione di un nuovo file lì. Il rifiuto richiede Claude Code v2.1.208 o successivo.

* **Read-before-edit**: Claude legge il file nella conversazione corrente prima di modificarlo, e una lettura interrotta con un avviso [`PARTIAL view`](#read-tool-behavior) non conta. Claude Opus 4.6, Claude Haiku 4.5 e i modelli più vecchi richiedono sempre la lettura. I modelli più recenti possono modificare un file non letto quando la lettura non richiederebbe un prompt di autorizzazione e lo strumento Read è disponibile.
* **Match**: `old_string` deve apparire nel file esattamente come scritto. Una singola differenza di spazio bianco o indentazione è sufficiente per non trovare una corrispondenza.
* **Uniqueness**: `old_string` deve apparire esattamente una volta. Quando appare più di una volta, Claude fornisce una stringa più lunga con contesto circostante sufficiente per identificare un'occorrenza, oppure imposta `replace_all: true` per sostituirle tutte.

Un file che è cambiato su disco dopo che Claude l'ha letto per l'ultima volta può comunque essere modificato quando `old_string` corrisponde esattamente al contenuto attuale in modo univoco e Claude Code può leggere il file senza richiedere un prompt. La corrispondenza con il contenuto attuale del file mantiene questa sicurezza, e il risultato nota che il file contiene altre modifiche in modo che Claude lo rilegga prima di modifiche che dipendono dal contenuto circostante. In qualsiasi altro caso, come un `old_string` obsoleto o uno che corrisponde più di una volta senza `replace_all`, Claude legge di nuovo il file prima di modificarlo. La gestione rilassata di file non letti e modificati richiede Claude Code v2.1.208 o successivo; prima di ciò, Claude Code rifiutava qualsiasi modifica a un file che non aveva letto nella conversazione o che era cambiato su disco dopo la lettura.

La visualizzazione di un file con Bash soddisfa il requisito read-before-edit quando il comando è `cat`, `nl`, `bat`, `batcat`, `head`, `tail`, `sed -n 'X,Yp'`, `grep`, `egrep`, `fgrep`, o `rg` su un singolo file senza pipe o reindirizzamenti. L'output tramite pipe e altri comandi Bash non contano verso il controllo read-before-edit.

La visualizzazione di un file con Bash influisce solo sull'idoneità alla modifica, non sulle autorizzazioni. Vedere [Regole di autorizzazione Read e Edit](/docs/it/permissions#read-and-edit) per quali comandi Bash le vostre regole di negazione `Read` e `Edit` coprono.

<h2 id="endconversation-tool-behavior">
  Comportamento dello strumento EndConversation
</h2>

Lo strumento EndConversation termina la sessione corrente. Claude lo utilizza solo in due situazioni:

* come ultima risorsa contro input abusivi sostenuti, dopo i tentativi di reindirizzare la conversazione hanno fallito e dopo un avvertimento chiaro in un messaggio precedente
* quando Lei esplicitamente chiede di vedere lo strumento dimostrato e conferma che desidera terminare la sessione

La frustrazione generale, il linguaggio scurrile o un'attività che va male non si qualificano, così come le richieste di contenuti dannosi, che Claude rifiuta invece di terminare la sessione. Claude Code segue lo stesso approccio di claude.ai, che può [terminare un sottoinsieme raro di chat](https://www.anthropic.com/research/end-subset-conversations).

Dopo che Claude termina una sessione interattiva, la sessione si blocca. I nuovi prompt e la maggior parte dei comandi restituiscono `Claude ended this conversation. Start a new session (or /clear) to continue.`, e solo `/clear`, `/resume`, `/help`, `/exit` e `/feedback` continuano a funzionare. Claude Code registra la fine nella trascrizione della sessione, quindi riprendere una sessione terminata ripristina il blocco; la cronologia della sessione non viene eliminata.

Riprendere una sessione terminata in [modalità non interattiva](/docs/it/headless) con il flag `-p` genera un errore e esce con codice 1, quindi uno script non legge l'esecuzione terminata come un successo.

Lo strumento non richiede mai il permesso e gli [hook PreToolUse](/docs/it/hooks#pretooluse) non vengono eseguiti per esso. Mentre rimane qualsiasi altro strumento, non è possibile bloccarlo nemmeno: le [regole di negazione e richiesta](/docs/it/permissions#tool-specific-permission-rules) che nominano `EndConversation` non hanno effetto, e né `--disallowedTools` né un elenco `--tools` possono rimuoverlo. L'esenzione è intenzionale: lo strumento non fa nulla se non terminare la conversazione, non leggendo o modificando mai file o dati, e una salvaguardia di questo tipo vale solo se la sessione a cui si applica non può disattivarla. Quando le regole di negazione rimuovono ogni altro strumento e corrispondono anche a `EndConversation`, come fa `"*"`, Claude Code lo rimuove anche piuttosto che lasciarlo come unico strumento, a meno che una regola di autorizzazione non nomini esplicitamente `EndConversation`. Un elenco di negazione che rimuove ogni altro strumento senza corrispondere a `EndConversation` lo lascia in posizione.

Gli [agenti secondari](/docs/it/sub-agents) non ottengono mai lo strumento. Le attività in background che condividono l'elenco degli strumenti della conversazione principale lo vedono, ma chiamarlo lì non termina nulla.

Lo strumento appare solo quando tutti i seguenti elementi sono veri:

* **Versione**: Claude Code v2.1.213 o successivo.
* **Modello**: il modello della sessione è Claude Opus 4.8, Claude Sonnet 5, Claude Fable 5 o una versione successiva di una di queste famiglie.
* **Superficie**: una sessione di terminale interattiva, inclusa una sessione `claude` nel terminale integrato di un IDE, che è il modo in cui il [plugin JetBrains](/docs/it/jetbrains) lo esegue. Altre superfici non includono lo strumento, come:
  * esecuzioni non interattive `-p`
  * sessioni attraverso i pacchetti TypeScript e Python dell'[Agent SDK](/docs/it/agent-sdk/overview)
  * il pannello dell'[estensione VS Code](/docs/it/vs-code), che raggruppa la propria CLI
  * [GitHub Actions](/docs/it/github-actions)
  * [Claude Code sul web](/docs/it/claude-code-on-the-web)
* **Modalità di avvio**: non una sessione [`--bare`](/docs/it/headless#start-faster-with-bare-mode). La modalità bare carica solo gli strumenti shell e file, quindi lo strumento non viene mai registrato lì.
* **Provider**: non disponibile su [Amazon Bedrock](/docs/it/amazon-bedrock), [Claude Platform su AWS](/docs/it/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) o [Microsoft Foundry](/docs/it/microsoft-foundry), o su sessioni accedute tramite un [cloud gateway](/docs/it/claude-apps-gateway).

<h2 id="glob-tool-behavior">
  Comportamento dello strumento Glob
</h2>

Lo strumento Glob trova file in base al modello di nome. Su Windows, fa parte del set di strumenti predefinito. Su macOS, Linux e WSL, Claude Code esclude Glob e [Grep](#grep-tool-behavior) dal set di strumenti predefinito, e Claude esegue ricerche con `find` e `grep` attraverso lo strumento Bash. Nella shell di Claude questi due comandi eseguono versioni incorporate di `bfs` e `ugrep`, e le ricerche raggiungono i vostri hook e le regole di autorizzazione come chiamate `Bash`.

Su macOS, Linux e WSL, recuperate gli strumenti Glob e Grep in questi casi:

* Nominate `Glob` o `Grep` in [`--tools` o `--allowedTools`](/docs/it/cli-reference#cli-flags) quando avviate la sessione, o nelle opzioni equivalenti di [Agent SDK](/docs/it/agent-sdk/overview). Con `--tools` ottenete quelli che elencate, e nominare uno dei due strumenti in `--allowedTools` ripristina entrambi. Una regola di autorizzazione in un file di impostazioni non ha questo effetto.
* Una [regola di negazione](/docs/it/permissions#match-all-uses-of-a-tool) delle autorizzazioni, il flag `--disallowedTools`, o [`--restricted`](/docs/it/cli-reference#cli-flags) rimuove `Bash` dalla sessione.
* Un [subagent](/docs/it/sub-agents#available-tools) elenca `Glob` o `Grep` nel suo campo `tools` e omette `Bash`. Gli strumenti elencati tornano solo per quel subagent, o per l'intera sessione quando viene eseguito come agente della sessione principale attraverso [`--agent`](/docs/it/sub-agents#invoke-subagents-explicitly) o l'impostazione `agent`.

Glob supporta la sintassi glob standard incluso `**` per la corrispondenza ricorsiva delle directory:

* `**/*.js` corrisponde a tutti i file `.js` a qualsiasi profondità
* `src/**/*.ts` corrisponde a tutti i file `.ts` sotto `src/`
* `*.{json,yaml}` corrisponde ai file `.json` e `.yaml` nella directory corrente

I risultati sono ordinati per ora di modifica e limitati a 100 file. Se il limite viene raggiunto, Claude vede un flag di troncamento nel risultato e può restringere il modello.

Glob non rispetta `.gitignore` per impostazione predefinita, quindi trova file ignorati da git insieme a quelli tracciati. Questo differisce da [Grep](#grep-tool-behavior), che salta i file ignorati da git. Per fare in modo che Glob rispetti `.gitignore`, impostare `CLAUDE_CODE_GLOB_NO_IGNORE=false` prima di avviare Claude Code.

Claude Code decide il permesso per una chiamata Glob prima di verificare se la directory di ricerca esiste. Esegue comunque il controllo delle autorizzazioni di lettura per un `path` mancante al di fuori delle [directory di lavoro](/docs/it/permissions#working-directories), quindi un prompt di permesso per un percorso non significa che il percorso esista.

Un valore `pattern` o `path` che contiene un byte null restituisce un errore chiedendo a Claude di rimuoverlo.&#x20;

<h2 id="grep-tool-behavior">
  Comportamento dello strumento Grep
</h2>

Lo strumento Grep cerca pattern nei contenuti dei file. Dove [Glob](#glob-tool-behavior) trova i file per nome, Grep trova le righe al loro interno. Su macOS, Linux e WSL, Grep è assente per impostazione predefinita nelle stesse condizioni di Glob. Vedi [Comportamento dello strumento Glob](#glob-tool-behavior) per quando entrambi gli strumenti sono disponibili.

Grep è costruito su [ripgrep](https://github.com/BurntSushi/ripgrep) e utilizza la sintassi regex di ripgrep, non grep POSIX. I pattern che includono metacaratteri regex necessitano di escape. Ad esempio, trovare `interface{}` nel codice Go richiede il pattern `interface\{\}`.

Un pattern, glob, o tipo di file che ripgrep rifiuta restituisce un errore che include la diagnostica di ripgrep, così Claude può correggere l'input e cercare di nuovo. Prima della v2.1.208, Claude Code segnalava un input rifiutato come `No files found` invece di un errore, anche quando il testo cercato esisteva nei file di destinazione.

Tre modalità di output controllano ciò che viene restituito:

* `files_with_matches`: solo i percorsi dei file, nessun contenuto di riga. Questa è l'impostazione predefinita.
* `content`: righe corrispondenti con numero di file e riga. Quando il parametro `offset` dello strumento punta oltre l'ultima corrispondenza per un pattern che ha corrispondenze, Grep restituisce `No entries at this offset`, così Claude amplia o reimposta l'offset invece di concludere che il pattern non corrisponde.
* `count`: conteggio delle corrispondenze per file, seguito da un totale su tutti i file corrispondenti. Il totale copre ogni corrispondenza anche quando i parametri `head_limit` o `offset` dello strumento troncano le voci elencate per file. Prima della v2.1.208, il totale sommava solo le voci elencate.

Claude può limitare i risultati per file con il parametro `glob`, come `**/*.tsx`, o per linguaggio con il parametro `type`, come `py` o `rust`. Per impostazione predefinita, i pattern corrispondono all'interno di una singola riga. Claude può impostare `multiline: true` per corrispondere oltre i confini delle righe.

Grep rispetta `.gitignore`, quindi i file ignorati da git vengono saltati. Per cercare un file ignorato da git, Claude passa il suo percorso direttamente.

Claude Code decide il permesso per una chiamata Grep prima di verificare se il `path` di ricerca esiste. Esegue comunque il controllo del permesso di lettura per un `path` mancante al di fuori delle [directory di lavoro](/docs/it/permissions#working-directories), quindi un prompt di permesso per un percorso non significa che il percorso esista.

<h2 id="lsp-tool-behavior">
  Comportamento dello strumento LSP
</h2>

Lo strumento LSP fornisce a Claude l'intelligenza del codice da un language server in esecuzione. Dopo ogni modifica di file, segnala automaticamente errori di tipo e avvisi in modo che Claude possa correggere i problemi senza un passaggio di compilazione separato. Claude può anche chiamarlo direttamente per navigare il codice:

* Saltare alla definizione di un simbolo
* Trovare tutti i riferimenti a un simbolo
* Ottenere informazioni sul tipo in una posizione
* Elencare i simboli in un file
* Cercare un simbolo per nome nell'intera area di lavoro
* Trovare implementazioni di un'interfaccia
* Tracciare gerarchie di chiamate

Claude Code mantiene lo strumento inattivo finché non installate un [plugin di code intelligence](/docs/it/plugins/code-intelligence) per il vostro linguaggio. Nelle [sessioni cloud](/docs/it/claude-code-on-the-web), Claude Code non avvia i language server dei plugin, quindi lo strumento LSP rimane inattivo lì. Claude Code prende la configurazione del language server dal plugin e installate il binario del server voi stessi.

Claude Code restituisce un risultato di errore per ogni chiamata LSP su un file il cui language server non riesce ad avviare.

<h2 id="monitor-tool">
  Strumento Monitor
</h2>

Lo strumento Monitor consente a Claude di osservare qualcosa in background e reagire quando cambia, senza mettere in pausa la conversazione. Chiedi a Claude di:

* Monitorare un file di log e segnalare gli errori man mano che appaiono
* Eseguire il polling di una PR o di un job CI e segnalare quando il suo stato cambia
* Osservare una directory per i cambiamenti dei file
* Tracciare l'output da qualsiasi script a lunga esecuzione che gli indichi
* Connettersi a un feed WebSocket e segnalare ogni messaggio man mano che arriva

Per la maggior parte dei monitoraggi, Claude scrive un piccolo script, lo esegue in background e riceve ogni riga di output man mano che arriva. Per un server che già invia eventi, Claude può aprire una [connessione WebSocket](#websocket-source) invece di eseguire uno script.

Continui a lavorare nella stessa sessione e Claude interviene quando arriva un evento.

Ogni monitoraggio che Claude avvia ha una scadenza: 5 minuti per impostazione predefinita, al massimo 30 minuti, e al massimo 10 minuti in un'esecuzione [non interattiva](/docs/it/headless) con un singolo prompt con `-p`.

Alla scadenza il monitoraggio termina. Claude riceve un avviso, quindi può avviare di nuovo il monitoraggio se è ancora necessario.

Interrompi un monitor chiedendo a Claude di annullarlo o terminando la sessione. Quando interrompi un [subagent](/docs/it/sub-agents) che ha avviato monitor, ad esempio da `/tasks`, quei monitor si interrompono con esso.

Quando Monitor esegue un comando, utilizza le stesse [regole di autorizzazione di Bash](/docs/it/permissions#tool-specific-permission-rules), quindi i pattern `allow` e `deny` che hai impostato per Bash si applicano anche qui. Mentre la [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) è attiva, Claude Code mette da parte le regole allow che nominano `Monitor` stesso, insieme agli altri [broad allow rules che elimina](/docs/it/permission-modes#how-the-classifier-evaluates-actions), quindi il classificatore esamina i comandi Monitor nello stesso modo in cui esamina i comandi Bash.

La [sorgente WebSocket](#websocket-source) ha il suo prompt di approvazione, che il classificatore decide anche in modalità auto.

Lo strumento non è disponibile su Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Non è disponibile nemmeno quando `DISABLE_TELEMETRY` o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` è impostato.

I plugin possono dichiarare monitor che si avviano automaticamente quando il plugin è attivo, invece di chiedere a Claude di avviarli. Vedi [plugin monitors](/docs/it/plugins/components#monitors).

<h3 id="websocket-source">
  WebSocket source
</h3>

<Note>
  La sorgente WebSocket richiede Claude Code v2.1.195 o successivo.
</Note>

Quando un server invia già eventi su WebSocket, Claude può connettersi direttamente invece di scrivere uno script di polling. Ogni tipo di attività del socket diventa un evento o termina il monitoraggio:

* **Messaggi di testo**: ognuno diventa un evento, anche quando il messaggio si estende su più righe.
* **Messaggi binari**: non vengono passati. Claude riceve una riga segnaposto come `[binary frame, 512 bytes]`.
* **Messaggi più grandi di 1 MiB**: il monitoraggio termina, quindi sottoscrivi un feed filtrato dove esiste.
* **Chiusura del socket**: il monitoraggio termina e Claude riceve il codice di chiusura.

Un monitoraggio WebSocket accetta un input `ws` al posto di `command`, e una singola chiamata Monitor non può combinare i due. L'input `ws` ha due campi:

| Campo       | Obbligatorio | Descrizione                                                                                                                                                   |
| :---------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `url`       | Sì           | L'endpoint a cui connettersi. Deve essere un URL `ws://` o `wss://` senza credenziali incorporate o spazi, utilizzando solo caratteri ASCII                   |
| `protocols` | No           | Nomi di subprotocollo WebSocket da offrire durante l'handshake. Ogni voce deve essere un token di subprotocollo valido e l'elenco non può contenere duplicati |

La scadenza `timeout_ms` si applica anche a un monitoraggio WebSocket: il monitoraggio termina alla scadenza, e `TaskStop` lo annulla anticipatamente.

L'apertura di una WebSocket richiede l'approvazione; in [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) il classificatore decide invece. Il prompt non offre un'opzione per saltare i prompt futuri per lo stesso host.

Claude Code nega gli URL che puntano a un indirizzo privato, link-local o cloud-metadata, inclusi i nomi host che si risolvono in uno. Nega anche gli host in `sandbox.network.deniedDomains` e, quando [`allowManagedDomainsOnly`](/docs/it/settings-reference#sandbox-network-allowmanageddomainsonly) è impostato nelle impostazioni gestite, qualsiasi host al di fuori dell'allowlist gestito.

<h2 id="notebookedit-tool-behavior">
  Comportamento dello strumento NotebookEdit
</h2>

NotebookEdit modifica un notebook Jupyter una cella alla volta, puntando alle celle per il loro `cell_id`. Non esegue la sostituzione di stringhe attraverso il notebook come fa [Edit](#edit-tool-behavior) sui file semplici.

Tre modalità di edit controllano cosa accade alla cella target:

* `replace`: sovrascrivi la sorgente della cella. Questo è il valore predefinito.
* `insert`: aggiungi una nuova cella dopo la target. Senza `cell_id`, la nuova cella va all'inizio del notebook. Richiede `cell_type` impostato a `code` o `markdown`.
* `delete`: rimuovi la cella target.

Le regole di autorizzazione utilizzano il formato di percorso `Edit(...)`. Una regola come `Edit(notebooks/**)` copre le chiamate NotebookEdit su file in quella directory.

<h2 id="powershell-tool">
  PowerShell tool
</h2>

Lo strumento PowerShell consente a Claude di eseguire comandi PowerShell in modo nativo. Su Windows, ciò significa che i comandi vengono eseguiti in PowerShell anziché essere instradati attraverso Git Bash. La disponibilità dello strumento dipende dalla piattaforma:

* **Windows senza Git Bash**: lo strumento è abilitato automaticamente.
* **Windows con Git Bash installato**: lo strumento è abilitato per impostazione predefinita per gli account claude.ai e Console; impostare `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` per abilitarlo nelle sessioni Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, oppure `0` per disabilitarlo.
* **Linux, macOS e WSL**: lo strumento è facoltativo.

I vostri [hook PreToolUse](/docs/it/hooks#powershell) ricevono la stringa di comando dello strumento in `tool_input.command`, con gli stessi campi dello strumento Bash.

Abbinate `Bash|PowerShell` negli hook che ispezionano i comandi della shell; la [sezione di input dell'hook PowerShell](/docs/it/hooks#powershell) spiega perché abbinare solo `Bash` non è sufficiente.

<h3 id="enable-the-powershell-tool">
  Enable the PowerShell tool
</h3>

Impostare `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` nel vostro ambiente o in `settings.json`:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

Su Windows, impostare la variabile a `0` per disabilitare lo strumento. Su Linux, macOS e WSL, lo strumento richiede PowerShell 7 o versioni successive: installare `pwsh` e assicurarsi che sia nel vostro `PATH`.

Su Windows, Claude Code rileva automaticamente `pwsh.exe` per PowerShell 7+ con un fallback a `powershell.exe` per PowerShell 5.1. Quando lo strumento è abilitato, Claude tratta PowerShell come la shell principale. Lo strumento Bash rimane disponibile per gli script POSIX quando Git Bash è installato.

Claude Code avvia PowerShell con `-ExecutionPolicy Bypass` solo a livello di processo, quindi gli script `.ps1` e le importazioni di moduli funzionano sulle installazioni Windows predefinite senza modificare la politica della macchina. Il bypass a livello di processo non sostituisce la Criteri di gruppo `MachinePolicy` o `UserPolicy`, quindi le politiche aziendali rimangono in vigore. Per rispettare la politica di esecuzione effettiva della macchina, impostare `CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1`.

<h3 id="shell-selection-in-settings-hooks-and-skills">
  Shell selection in settings, hooks, and skills
</h3>

Tre impostazioni aggiuntive controllano dove viene utilizzato PowerShell:

* `"defaultShell": "powershell"` in [`settings.json`](/docs/it/settings-reference#all-settings): instrada i comandi interattivi `!` attraverso PowerShell. Richiede che lo strumento PowerShell sia abilitato.
* `"shell": "powershell"` su singoli [hook di comando](/docs/it/hooks#command-hook-fields): esegue quell'hook in PowerShell. Gli hook avviano PowerShell direttamente, quindi funziona indipendentemente da `CLAUDE_CODE_USE_POWERSHELL_TOOL`.
* `shell: powershell` nel [frontmatter delle skill](/docs/it/skills#frontmatter-reference): esegue i blocchi `` !`command` `` in PowerShell. Richiede che lo strumento PowerShell sia abilitato.

Lo stesso comportamento di ripristino della directory di lavoro della sessione principale descritto nella sezione dello strumento Bash si applica ai comandi PowerShell, inclusa la variabile di ambiente `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR`.

A partire dalla v2.1.196, il codice di uscita 1 da `grep`, `rg`, `egrep`, `fgrep`, `findstr` e `git grep` significa nessuna corrispondenza. Il codice di uscita 1 da `git diff` significa che esistono differenze. Nessuno di questi risultati viene segnalato a Claude come un errore di comando. Per `robocopy`, i codici di uscita da 0 a 7 sono risultati informativi, come file copiati o file aggiuntivi rilevati. I codici di uscita 8 o superiori contano come errori.

<h3 id="windows-encoding-and-exit-codes">
  Windows encoding and exit codes
</h3>

Su Windows, i seguenti comportamenti di codifica PowerShell e codici di uscita richiedono Claude Code v2.1.214 o versioni successive:

* Il reindirizzamento con `>` e `>>` scrive file UTF-8 su PowerShell 5.1
* Claude Code codifica il testo inviato all'input standard di un comando nativo come UTF-8
* Claude Code acquisisce l'output di errore senza sequenze di escape ANSI
* Un comando il cui processo figlio attende l'input standard riceve la fine del file anziché bloccarsi
* Il codice di uscita 1 da `where.exe` significa nessuna corrispondenza, e da `fc.exe` e `diff.exe` significa che i file differiscono, quindi quando il comando produce output, Claude Code tratta quel codice di uscita come una risposta negativa valida anziché un errore di comando. Claude Code comunque segnala una forma silenziata, come `where.exe /Q` o un reindirizzamento a `$null`, come un errore al codice di uscita 1

Prima della v2.1.214, `>` su PowerShell 5.1 scriveva file UTF-16LE, l'input non ASCII inviato arrivava come `?`, e gli script Python potevano bloccarsi con un `UnicodeEncodeError` quando stampavano caratteri non ASCII.

<h3 id="preview-limitations">
  Preview limitations
</h3>

Lo strumento PowerShell ha le seguenti limitazioni note durante l'anteprima:

* I profili PowerShell non vengono caricati
* Su Windows, il sandboxing non è supportato

<h2 id="read-tool-behavior">
  Comportamento dello strumento Read
</h2>

Lo strumento Read accetta un percorso di file e restituisce i contenuti con numeri di riga. Claude è istruito a passare sempre percorsi assoluti.

Per impostazione predefinita, Read restituisce il file dall'inizio. Quando una lettura dell'intero file supera il limite di token, Read restituisce la prima pagina con un avviso `PARTIAL view` che comunica a Claude quanto del file ha ricevuto e come leggere di più con `offset` e `limit`. Una lettura che passa un `offset` o `limit` esplicito e supera comunque il limite di token restituisce un errore.

Una lettura con un `limit` esplicito si interrompe non appena le righe selezionate superano ciò che il limite di token potrebbe mai contenere e restituisce un errore senza caricare il resto dell'intervallo. L'errore comunica a Claude di usare un `limit` più piccolo, o di cercare contenuti specifici con [Grep](#grep-tool-behavior) invece quando una singola riga è così grande. Prima della v2.1.208, Claude Code caricava l'intero intervallo in memoria prima di rifiutarlo, quindi la lettura di un file con una singola riga estremamente lunga poteva esaurire la memoria.

La lettura di un file vuoto restituisce un avviso che il file esiste ma i suoi contenuti sono vuoti, e un `offset` oltre l'ultima riga restituisce un avviso che fornisce il conteggio delle righe del file. Prima della v2.1.208, la lettura di un file vuoto restituiva invece l'avviso past-the-end.

Read gestisce diversi tipi di file oltre al testo semplice:

* **Immagini**: PNG, JPG e altri formati di immagine vengono restituiti come contenuto visivo che Claude può vedere, non come byte grezzi. Claude Code ridimensiona e ricomprime le immagini grandi per adattarsi ai limiti di dimensione dell'immagine del modello prima di inviarle, quindi Claude potrebbe vedere una versione ridotta di uno screenshot grande. A partire dalla v2.1.196, un'immagine che è ancora più grande di 500KB dopo quel ridimensionamento viene ricodificata come JPEG a qualità ridotta con le sue dimensioni in pixel invariate. Se Claude perde dettagli a livello di pixel fine in un'immagine grande, chiedetegli di ritagliare prima la regione di interesse, ad esempio con ImageMagick tramite Bash.
* **PDF**: Claude legge i file `.pdf` brevi per intero. Per i PDF più lunghi di 10 pagine, legge in intervalli con un parametro `pages`, come `"1-5"`, fino a 20 pagine alla volta.
* **Notebook Jupyter**: i file `.ipynb` restituiscono tutte le celle con i loro output, inclusi codice, markdown e visualizzazioni. Claude Code rifiuta di leggere un file notebook più grande di 100 MB; l'errore comunica a Claude come leggere una porzione del notebook invece, come una sezione di celle, con un comando shell.

Read legge solo file, non directory. Claude elenca i contenuti della directory con un comando shell come `ls`.

<h2 id="sendfeedback-tool-behavior">
  Comportamento dello strumento SendFeedback
</h2>

Il feedback redatto da Claude è una relazione di feedback su Claude Code che Claude scrive per voi. Richiede Claude Code v2.1.238 o versione successiva. Claude Code salva ogni bozza sulla vostra macchina in `~/.claude/feedback/drafts/`, e nulla raggiunge Anthropic finché non la inviate. Claude redige una bozza con lo strumento SendFeedback quando:

* Uno strumento o un comando continua a non funzionare
* Non riesce ad aiutarvi con qualcosa che avete chiesto
* Voi segnalate un errore che ha commesso, oppure lo nota da solo
* Gli chiedete di archiviare il feedback

<h3 id="what-you-see-when-claude-drafts">
  Cosa vedete quando Claude redige
</h3>

Dopo che Claude mette in coda una bozza, vedete una scheda sopra il vostro prompt con il titolo della bozza. Premete `1` per rivedere la bozza, premete `2` due volte per inviarla così com'è, oppure premete `0` per scartarla. Una bozza scartata rimane nella vostra coda. Dopo aver scartato una scheda, Claude Code vi chiede se disattivare il feedback redatto da Claude. Smette di chiedere una volta che avete rifiutato due volte.

Per impostazione predefinita, vedete al massimo tre schede in una sessione; Anthropic può regolare quel limite dal server senza una release. Dopo il limite, e ogni volta che impostate [`feedbackDrafts`](/docs/it/settings-reference#feedbackdrafts) su `quiet`, vedete solo un conteggio delle bozze in coda nel piè di pagina del prompt.

<h3 id="review-and-edit-a-draft">
  Rivedere e modificare una bozza
</h3>

Eseguite `/feedback` senza argomenti per aprire la vostra coda. Elenca ogni bozza in coda da tutte le vostre sessioni, incluse le bozze le cui schede avete scartato o non avete mai visto. Selezionate una bozza per aprirla per la revisione, dove potete:

* Modificare il titolo, l'area e i dettagli
* Impostare **Send transcript** su `yes` o `no`. Quando la trascrizione della sessione in cui Claude ha messo in coda la bozza è ancora disponibile, inizia su `yes`, che invia quella conversazione ad Anthropic; `no` invia solo la relazione
* Inviare la bozza, scartarla, o lasciarla in coda per dopo

Per scrivere una relazione voi stessi, premete `w` per la finestra di dialogo del feedback standard. `/feedback` con testo dopo di esso, e `/bug`, aprono quella finestra di dialogo direttamente.

<h3 id="send-a-draft">
  Inviare una bozza
</h3>

Quando inviate una bozza, Claude Code la invia nello stesso modo di una relazione `/feedback`, con la stessa [conservazione](/docs/it/data-usage#feedback-using-the-%2Ffeedback-command), e cancella la bozza dalla vostra macchina. Quando inviate dalla scheda, mostra `✓ Sent`; quando inviate dalla coda, si chiude con un ID di ricevuta.

La relazione contiene:

* Il vostro titolo, area e dettagli
* Informazioni sull'ambiente, come la vostra versione di Claude Code, il sistema operativo e il modello
* Gli ID delle recenti richieste API
* La trascrizione della conversazione, quando avete lasciato **Send transcript** su `yes` nella schermata di revisione. L'invio dalla scheda non include mai la trascrizione

Claude Code mantiene la vostra directory di lavoro nella bozza locale in modo da poter trovare la trascrizione, e non invia la directory.

Nelle [organizzazioni con conservazione dati zero](/docs/it/zero-data-retention#features-disabled-under-zdr), Claude Code omette lo strumento, come fa per `/feedback`. Se una sessione in tale organizzazione offre ancora lo strumento, le bozze rimangono sulla vostra macchina, e l'invio non riesce con `Feedback collection is not available for organizations with custom data retention policies.`

<h3 id="discard-or-keep-a-draft">
  Scartare o mantenere una bozza
</h3>

Quando scartate una bozza, Claude Code la cancella dalla vostra macchina. Una bozza che lasciate in coda scade dopo 30 giorni, o dopo [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays) quando è più breve. La coda contiene 10 bozze in tutte le vostre sessioni, e quando Claude mette in coda l'undicesima, Claude Code cancella la più vecchia. Quando eseguite `/exit` con bozze della sessione ancora in coda, Claude Code vi chiede se rivederle o scartarle prima di uscire.

<h3 id="turn-claude-drafted-feedback-off">
  Disattivare il feedback redatto da Claude
</h3>

Impostate **Claude-drafted feedback** su `off` in `/config`, che scrive l'impostazione [`feedbackDrafts`](/docs/it/settings-reference#feedbackdrafts), oppure impostate [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/it/env-vars) per una sessione. Con entrambi, Claude non può mettere in coda le bozze. Per mantenere la redazione attiva senza schede, impostate `feedbackDrafts` su `quiet` invece. Gli amministratori possono impostare `feedbackDrafts` nelle [impostazioni gestite](/docs/it/managed-settings), che ha la precedenza sulla vostra impostazione.

<h3 id="sessions-without-claude-drafted-feedback">
  Sessioni senza feedback redatto da Claude
</h3>

Claude Code include lo strumento nelle sessioni di terminale interattive sulla vostra macchina che utilizzano l'API Claude piuttosto che un provider cloud. Lo omette da:

* Esecuzioni non interattive `-p` e sessioni [Agent SDK](/docs/it/agent-sdk/overview), che non hanno uno schermo per rivedere la coda
* [Sessioni cloud](/docs/it/claude-code-on-the-web), che non possono scrivere nella coda sulla vostra macchina
* Sessioni su [Amazon Bedrock](/docs/it/amazon-bedrock), [Claude Platform on AWS](/docs/it/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o [Microsoft Foundry](/docs/it/microsoft-foundry)
* Sessioni in cui avete impostato [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/it/env-vars) o [`DISABLE_FEEDBACK_COMMAND=1`](/docs/it/env-vars), impostato `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` su qualsiasi valore non vuoto, o disattivato il [recupero dei flag di funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching)
* Organizzazioni che hanno disattivato il feedback sui prodotti, e [organizzazioni con conservazione dati zero](/docs/it/zero-data-retention#features-disabled-under-zdr)

<h2 id="task-tool-availability">
  Disponibilità dello strumento Task
</h2>

Gli strumenti di tracciamento delle attività, `TaskCreate`, `TaskGet`, `TaskUpdate`, `TaskList` e `TodoWrite`, sono disponibili per impostazione predefinita solo sui modelli Claude 3.x, Opus 4 fino a 4.7, Sonnet 4 fino a 4.6 e Haiku 4.5. Ovunque gli strumenti siano disponibili, si ottengono i quattro strumenti Task, oppure `TodoWrite` quando si imposta [`CLAUDE_CODE_ENABLE_TASKS=0`](/docs/it/env-vars).

Su tutti gli altri modelli, Claude Code esclude gli strumenti a meno che non si acconsenta esplicitamente. Lo stesso vale per un ID modello che Claude Code non riconosce, come un nome di modello personalizzato servito attraverso un [gateway LLM](/docs/it/llm-gateway). Sui modelli più recenti, Claude tiene traccia del lavoro multi-step senza una checklist scritta, e le definizioni e i promemoria degli strumenti occupano contesto. Senza gli strumenti, Claude non aggiunge nulla all'[elenco attività](/docs/it/interactive-mode#task-list) mentre lavora.

Se desiderate utilizzare questi strumenti su un modello che non li ha per impostazione predefinita, eseguite una delle seguenti operazioni:

* Esportate [`CLAUDE_CODE_ENABLE_TODO_TOOLS=1`](/docs/it/env-vars) prima di avviare Claude Code, ad esempio `CLAUDE_CODE_ENABLE_TODO_TOOLS=1 claude`. Claude Code fornisce quindi gli stessi strumenti su ogni modello e ogni provider
* Nominate uno degli strumenti in [`--allowedTools`](/docs/it/cli-reference#cli-flags), ad esempio `claude --allowedTools TaskCreate`
* Elencate gli strumenti in [`--tools`](/docs/it/cli-reference#cli-flags), che limita gli strumenti integrati della sessione a quelli che nomina. Includete gli strumenti desiderati insieme agli altri strumenti integrati che utilizzate
* In Agent SDK, le opzioni [`allowedTools` e `tools`](/docs/it/agent-sdk/todo-tracking#model-availability) funzionano allo stesso modo dei due flag

In [sessioni in background](/docs/it/agent-view) e in [Claude Code sul web](/docs/it/claude-code-on-the-web), Claude Code fornisce gli stessi strumenti su ogni modello, elencato o meno.

Claude Code fornisce a un subagent gli strumenti solo quando la vostra sessione li ha, anche quando il subagent esegue un modello diverso. Un membro del team [agent team](/docs/it/agent-teams) in-process segue la vostra sessione allo stesso modo, mentre un membro del team nel suo [split pane](/docs/it/agent-teams#choose-a-display-mode) viene eseguito come un processo Claude Code separato, quindi il suo modello decide. Senza gli strumenti Task, un agent coordina con il suo team attraverso messaggi invece dell'[elenco attività condiviso](/docs/it/agent-teams#assign-and-claim-tasks).

L'insieme predefinito descritto qui si applica in Claude Code v2.1.268 e versioni successive.

<h2 id="webfetch-tool-behavior">
  Comportamento dello strumento WebFetch
</h2>

WebFetch accetta un URL e un prompt che descrive cosa estrarre. Recupera la pagina, converte la risposta in Markdown quando il server restituisce HTML, ed esegue il prompt rispetto al contenuto utilizzando un modello piccolo e veloce. Per la maggior parte dei recuperi, Claude riceve la risposta di quel modello, non la pagina grezza. Il passaggio di conversione non è configurabile.

Questo rende WebFetch lossy per progettazione. Il prompt di estrazione determina cosa raggiunge Claude, quindi un risultato che dice che una pagina non menziona qualcosa potrebbe significare solo che il prompt non l'ha chiesto. Chiedete a Claude di recuperare di nuovo con un prompt più specifico, oppure utilizzate `curl` tramite Bash per la pagina non elaborata.

Alcuni comportamenti modellano la risposta che Claude riceve:

* WebFetch rifiuta `localhost` e qualsiasi altro nome host senza un punto, come un nome intranet nudo, prima di effettuare una richiesta. L'[errore che restituisce](/docs/it/errors#webfetch-cannot-fetch-localhost) dice a Claude di raggiungere i server locali con `curl` tramite Bash invece.
* Gli URL HTTP vengono automaticamente aggiornati a HTTPS.
* Le pagine grandi vengono troncate a un limite di caratteri fisso prima dell'elaborazione.
* WebFetch memorizza nella cache ogni risposta per 15 minuti per impostazione predefinita, quindi i recuperi ripetuti dello stesso URL vengono restituiti rapidamente. Su Claude Code v2.1.233 o versioni successive, impostare [`CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS`](/docs/it/env-vars#variables) per modificare il tempo in cui WebFetch mantiene ogni risposta.
* Una pagina che non ha terminato il download entro cinque minuti, inclusi tutti i reindirizzamenti che WebFetch segue, fallisce con un errore di deadline. Su Claude Code v2.1.268 o versioni successive, impostare [`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`](/docs/it/env-vars#variables) per modificare il limite, o a `0` per rimuoverlo.
* Quando un URL reindirizza a un host diverso, WebFetch restituisce un risultato di testo che nomina l'URL originale e la destinazione del reindirizzamento invece di seguirlo. Claude quindi recupera il nuovo URL con una seconda chiamata WebFetch.
* Quando il passaggio di estrazione colpisce un'API sovraccarica, Claude Code la riprova con backoff; un recupero che ancora fallisce restituisce un risultato di errore. Prima della v2.1.212, il testo di errore dell'API potrebbe raggiungere Claude come se fosse il contenuto della pagina estratta.

Nelle modalità Manual e `acceptEdits` [permission modes](/docs/it/permission-modes), WebFetch richiede prima di recuperare, ad eccezione dei domini che le vostre [permission rules](/docs/it/permissions#manage-permissions) già consentono o negano e un insieme integrato di domini di documentazione preapprovati che recuperano senza un prompt. Qualunque cosa le vostre regole consentano, un recupero passa anche il [WebFetch domain safety check](/docs/it/data-usage#webfetch-domain-safety-check) per primo; quella sezione copre cosa invia il controllo e l'impostazione che lo salta. Il prompt offre tre opzioni:

* **Yes**: approva solo questo recupero. La prossima chiamata WebFetch richiede di nuovo, anche per lo stesso dominio.
* **Yes, and don't ask again for `<domain>`**: approva il recupero e salva una regola di autorizzazione `WebFetch(domain:...)` per quel dominio in `.claude/settings.local.json` per quel repository. Vedere [come le approvazioni salvate persistono](/docs/it/permissions#permission-system). Quando la vostra organizzazione imposta [`allowManagedPermissionRulesOnly`](/docs/it/permissions#managed-only-settings), Claude Code nasconde questa opzione.
* **No, and tell Claude what to do differently**: rifiuta il recupero.

Per consentire un dominio in anticipo senza un prompt, aggiungete una regola di autorizzazione come `WebFetch(domain:example.com)`; `WebFetch(domain:*)` consente ogni dominio. Le modalità `auto` e `bypassPermissions` [permission modes](/docs/it/permissions#permission-modes) saltano il prompt, ad eccezione di un dominio che una regola `ask` esplicita corrisponde.

Una regola `WebFetch(domain:...)` esplicita in `deny`, `ask` o `allow` ha la precedenza rispetto all'insieme preapprovato, quindi potete bloccare un dominio preapprovato o richiedere un prompt per esso.

WebFetch imposta un'intestazione `User-Agent` che inizia con `Claude-User` e un'intestazione `Accept` che preferisce Markdown rispetto a HTML in modo che i server che supportano la negoziazione dei contenuti possano restituire Markdown direttamente.

I comandi in sandbox non ereditano l'insieme integrato di domini di documentazione preapprovati di WebFetch. Per consentire a un comando in sandbox di raggiungere un dominio senza un prompt, aggiungete il dominio a [`allowedDomains`](/docs/it/settings-reference#sandbox-network-alloweddomains) o consentitelo con una regola `WebFetch(domain:...)`, che il [sandbox onora anche](/docs/it/sandboxing#network-isolation). WebFetch non legge mai l'allowlist del sandbox in cambio, quindi aggiungere un dominio a un sandbox o a un allowlist di rete dell'organizzazione non impedisce a WebFetch di richiedere per esso.

<h2 id="websearch-tool-behavior">
  Comportamento dello strumento WebSearch
</h2>

WebSearch esegue una query sul backend di [web search](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) di Anthropic e restituisce titoli e URL dei risultati. Non recupera le pagine dei risultati. Per leggere una pagina che Claude trova nei risultati di ricerca, lo segue con [WebFetch](#webfetch-tool-behavior).

Lo strumento può emettere fino a otto ricerche backend per chiamata, affinando la ricerca internamente prima di restituire i risultati. Claude può limitare i risultati con `allowed_domains` per includere solo determinati host, o `blocked_domains` per escluderli. I due elenchi non possono essere combinati in una singola chiamata.

Quando la richiesta di ricerca colpisce un'API sovraccarica, Claude Code la riprova con backoff; una chiamata che comunque fallisce restituisce un risultato di errore. Prima della v2.1.212, il testo di errore dell'API potrebbe raggiungere Claude come se fossero risultati di ricerca.

Le regole di autorizzazione di WebSearch non richiedono alcun specificatore. Una voce `WebSearch` semplice in `allow` o `deny` è l'unica forma.

Il backend di ricerca non è configurabile. Per cercare con un provider diverso, aggiungi un [server MCP](/docs/it/mcp) che espone uno strumento di ricerca.

<Note>
  WebSearch è disponibile sull'API Claude e su [Claude Platform on AWS](/docs/it/claude-platform-on-aws). Su Microsoft Foundry richiede una [distribuzione ospitata su Anthropic](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options): le distribuzioni ospitate su Azure non supportano strumenti lato server, quindi la chiamata WebSearch fallisce. Su Google Cloud's Agent Platform funziona con Claude 4 e modelli successivi, inclusi Opus, Sonnet e Haiku. Amazon Bedrock non espone lo strumento di web search lato server.
</Note>

<h3 id="session-search-limit">
  Limite di ricerca della sessione
</h3>

Una sessione può effettuare al massimo 200 chiamate WebSearch, conteggiate nella conversazione principale e in ogni [subagent](/docs/it/sub-agents) che genera, quindi le ricerche effettuate da fan-out di ricerca paralleli contano rispetto allo stesso limite. Il limite richiede Claude Code v2.1.212 o successivo. Quando Claude raggiunge il limite, le ulteriori chiamate restituiscono un avviso che dice a Claude di continuare con le informazioni già raccolte, piuttosto che un errore che inviterebbe a un nuovo tentativo. Non vedi l'avviso: una chiamata limitata appare nella conversazione come una ricerca che non ha fatto nulla, e se Claude ha bisogno di più ricerche, l'avviso le dice di chiederti di aumentare il limite.

Imposta la variabile di ambiente [`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](/docs/it/env-vars) per modificare il limite; accetta un numero intero positivo, quindi il limite può essere aumentato ma non disattivato. L'esecuzione di [`/clear`](/docs/it/commands#all-commands) ripristina il conteggio. Se il lavoro che può ancora generare [subagents](/docs/it/sub-agents) sopravvive alla cancellazione, come un flusso di lavoro in esecuzione, il conteggio viene trasferito.

<h2 id="write-tool-behavior">
  Comportamento dello strumento Write
</h2>

Lo strumento Write crea un nuovo file o sovrascrive uno esistente con il contenuto completo fornito. Non aggiunge o unisce.

Se Claude deve leggere un file esistente nella conversazione corrente prima di sovrascriverlo dipende dal modello e dal file:

* Claude Opus 4.6, Claude Haiku 4.5 e i modelli più vecchi richiedono sempre la lettura, quindi una Write su un file esistente non letto fallisce con un errore.
* I modelli più recenti possono sovrascrivere un file che non hanno mai letto in questa sessione nelle stesse condizioni di [read-before-edit](#edit-tool-behavior): leggerlo non richiederebbe un prompt di autorizzazione e lo strumento Read è disponibile.
* I notebook Jupyter e i file che Claude ha letto solo parzialmente con un avviso [`PARTIAL view`](#read-tool-behavior) richiedono la lettura su ogni modello.

Questo vincolo non si applica ai nuovi file. Prima della v2.1.228, ogni modello richiedeva la lettura prima di sovrascrivere un file esistente.

Visualizzare il file con Bash soddisfa anche questo requisito secondo le stesse regole descritte in [Comportamento dello strumento Edit](#edit-tool-behavior).

Per modifiche parziali a un file esistente, Claude utilizza Edit invece di Write.

<h2 id="check-which-tools-are-available">
  Verifica quali strumenti sono disponibili
</h2>

Il tuo set esatto di strumenti dipende dal tuo provider, dalla piattaforma e dalle impostazioni. Per verificare cosa è caricato in una sessione in esecuzione, chiedi direttamente a Claude:

```text theme={null}
What tools do you have access to?
```

Claude fornisce un riepilogo conversazionale. Per i nomi esatti degli strumenti MCP, esegui `/mcp`.

<Note>
  Lo [strumento advisor](/docs/it/advisor) è uno [strumento server](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool) che l'API esegue, piuttosto che uno strumento che Claude Code implementa. Non ha un nome che puoi referenziare nelle regole di autorizzazione o nei matcher degli hook.
</Note>

<h2 id="see-also">
  Vedi anche
</h2>

* [Server MCP](/docs/it/mcp): aggiungi strumenti personalizzati connettendo server esterni
* [Autorizzazioni](/docs/it/permissions): sistema di autorizzazione, sintassi delle regole, e pattern specifici dello strumento
* [Subagent](/docs/it/sub-agents): configura l'accesso agli strumenti per i subagent
* [Hooks](/docs/it/hooks-guide): esegui comandi personalizzati prima o dopo l'esecuzione dello strumento
