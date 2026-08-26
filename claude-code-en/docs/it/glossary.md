> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Glossario

> Definizioni della terminologia di Claude Code. Scopri cosa significano agentic loop, compaction, CLAUDE.md, hooks, subagents, MCP e altri concetti fondamentali.

Questo glossario definisce la terminologia di Claude Code. Ogni voce rimanda alla pagina dove il concetto è trattato in profondità. Per i concetti a livello di modello come tokens, temperature e RAG, consulta il [glossario della piattaforma](https://platform.claude.com/docs/it/about-claude/glossary).

<h2 id="a">
  A
</h2>

<h3 id="agent-teams">
  Agent teams
</h3>

Più sessioni indipendenti di Claude Code coordinate da un team lead, con un elenco di attività condiviso e messaggistica peer-to-peer. A differenza dei [subagents](#subagent), che vengono eseguiti all'interno di una singola sessione e riferiscono solo al genitore, i compagni di squadra hanno ciascuno la propria finestra di contesto e puoi interagire direttamente con uno qualsiasi di loro. Agent teams è sperimentale e deve essere abilitato impostando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`.

Scopri di più: [Run agent teams](/docs/it/agent-teams)

<h3 id="agentic-coding">
  Agentic coding
</h3>

Un flusso di lavoro in cui l'IA può leggere file, eseguire comandi e apportare modifiche autonomamente mentre tu osservi, reindizzi o ti allontani, a differenza degli assistenti basati su chat che rispondono solo con testo che devi applicare tu stesso. Claude Code è agentic perché ha [tools](#tool) che gli permettono di agire, non solo di consigliare.

Scopri di più: [How Claude Code works](/docs/it/how-claude-code-works)

<h3 id="agentic-harness">
  Agentic harness
</h3>

Gli strumenti, la gestione del contesto e l'ambiente di esecuzione che trasformano un modello di linguaggio in un agente di codifica capace. Claude Code è l'harness; Claude è il modello al suo interno. L'harness fornisce accesso ai file, esecuzione shell, gating delle autorizzazioni, caricamento della memoria e il loop che concatena le azioni insieme.

Scopri di più: [How Claude Code works](/docs/it/how-claude-code-works)

<h3 id="agentic-loop">
  Agentic loop
</h3>

Il ciclo che Claude attraversa per ogni attività: raccogliere contesto, intraprendere un'azione, verificare i risultati e ripetere fino al completamento. Ogni utilizzo di uno strumento restituisce informazioni che informano il passo successivo. Puoi interrompere il loop in qualsiasi momento per reindirizzare. La maggior parte dei punti di estensione, inclusi [hooks](#hook), [skills](#skill) e [MCP](#mcp-model-context-protocol), si collegano a fasi specifiche di questo loop.

Scopri di più: [How Claude Code works](/docs/it/how-claude-code-works#the-agentic-loop)

<h3 id="artifact">
  Artifact
</h3>

Una pagina web live e interattiva che Claude Code pubblica dalla tua sessione a un URL privato su claude.ai, così puoi vedere l'output visivamente o condividerlo invece di leggere il testo del terminale. La pagina si aggiorna sul posto quando la sessione viene ripubblicata. Gli artifact che crei da Claude Code appaiono nella stessa galleria degli artifact creati nelle conversazioni di claude.ai. La condivisione dipende dal tuo piano: su Pro e Max, un link pubblico che chiunque può aprire; su Team ed Enterprise, la condivisione all'interno della tua organizzazione, più link pubblici una volta che un Owner li abilita.

Scopri di più: [Share session output as artifacts](/docs/it/artifacts)

<h3 id="auto-memory">
  Auto memory
</h3>

Note che Claude scrive per se stesso in base alle tue correzioni e preferenze, archiviate per repository git in `~/.claude/projects/`. Tutti i worktrees dello stesso repository condividono una directory di auto memory. Le prime 200 righe o 25 KB dell'indice `MEMORY.md` si caricano all'inizio di ogni sessione. Auto memory è la controparte scritta da Claude di [CLAUDE.md](#claude-md), che scrivi tu.

Scopri di più: [Auto memory](/docs/it/memory#auto-memory)

<h3 id="auto-mode">
  Auto mode
</h3>

Una [permission mode](#permission-mode) in cui un modello classificatore separato esamina le azioni in background, così la maggior parte viene eseguita senza prompt di approvazione; le regole di richiesta esplicita comunque richiedono un prompt. Il classificatore blocca l'escalation dell'ambito, l'infrastruttura non attendibile e l'[prompt injection](#prompt-injection). Non vede mai i risultati degli strumenti, quindi le istruzioni iniettate non possono influenzare le sue decisioni.

Scopri di più: [Eliminate prompts with auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare mode
</h3>

Un flag di avvio, `--bare`, che salta l'auto-discovery di hooks, skills, plugins, server MCP, auto memory e CLAUDE.md. Solo i flag che passi esplicitamente hanno effetto. Consigliato per CI e chiamate con script dove hai bisogno di un comportamento identico tra le macchine indipendentemente dalla configurazione locale.

Scopri di più: [Avvia più velocemente con bare mode](/docs/it/headless#start-faster-with-bare-mode)

<h3 id="bundled-skills">
  Bundled skills
</h3>

Playbook basati su prompt inclusi con Claude Code, come `/batch`, `/code-review`, `/debug` e `/loop`. A differenza dei comandi built-in, che eseguono logica fissa, le bundled skills danno a Claude un prompt dettagliato e gli permettono di orchestrare il lavoro, quindi possono generare agenti, leggere file e adattarsi al tuo codebase.

Scopri di più: [Bundled skills](/docs/it/skills#bundled-skills)

<h2 id="c">
  C
</h2>

<h3 id="channel">
  Channel
</h3>

Un [server MCP](#mcp-model-context-protocol) che invia eventi nella tua sessione in esecuzione in modo che Claude possa reagire a cose che accadono mentre sei lontano dal terminale. I canali possono essere bidirezionali: Claude legge un evento in entrata e risponde attraverso lo stesso canale. Telegram, Discord e iMessage sono inclusi nell'anteprima di ricerca.

Scopri di più: [Channels](/docs/it/channels)

<h3 id="checkpoint">
  Checkpoint
</h3>

Un punto di ripristino creato ad ogni prompt che invii. Claude Code acquisisce snapshot dei file prima di ogni modifica in modo che un checkpoint possa ripristinarli. Premi `Esc` due volte o esegui `/rewind` per ripristinare il codice, la conversazione o entrambi a un punto precedente, o per riassumere parte della conversazione da un messaggio selezionato. I checkpoint sono salvati con la conversazione, quindi una sessione ripresa può ancora `/rewind` verso di essi. Sono separati da git e non tracciano le modifiche apportate tramite lo strumento Bash.

Scopri di più: [Checkpointing](/docs/it/checkpointing)

<h3 id="claude-directory">
  `.claude` directory
</h3>

La directory da cui Claude Code legge la configurazione con ambito di progetto: impostazioni, hooks, skills, subagents, regole e auto memory. Un progetto ha `.claude/` alla sua radice; i tuoi valori predefiniti a livello di utente sono in `~/.claude/`.

Scopri di più: [The `.claude` directory](/docs/it/claude-directory)

<h3 id="claude-md">
  CLAUDE.md
</h3>

Un file markdown di istruzioni persistenti che scrivi per Claude, caricato all'inizio di ogni sessione come messaggio utente dopo il prompt di sistema. Metti qui le convenzioni di progetto, le note sull'architettura e le regole "fai sempre X". CLAUDE.md sopravvive alla [compaction](#compaction) e viene riletto fresco dal disco in seguito.

Puoi posizionare CLAUDE.md a livello di progetto in `./CLAUDE.md` o `./.claude/CLAUDE.md`, a livello di utente in `~/.claude/CLAUDE.md`, o come [managed policy](#managed-settings) per la tua organizzazione. Tutti i file scoperti vengono concatenati nel contesto piuttosto che sovrascriversi a vicenda, ordinati dall'ambito più ampio al più specifico.

Scopri di più: [CLAUDE.md files](/docs/it/memory#claude-md-files)

<h3 id="command">
  Command
</h3>

Un'istruzione riutilizzabile che invochi digitando `/name` nel prompt. I comandi built-in come `/clear`, `/model` e `/compact` controllano la sessione. Puoi definire i tuoi comandi come file in `.claude/commands/`, o installarli da un [plugin](#plugin). [Skills](#skill) sono il modo consigliato per confezionare comandi multi-step.

Scopri di più: [Commands](/docs/it/commands) · [Skills](/docs/it/skills)

<h3 id="compaction">
  Compaction
</h3>

Riassunto automatico della tua conversazione quando la [context window](#context-window) si avvicina al suo limite. Gli output degli strumenti più vecchi vengono cancellati per primi, quindi la conversazione viene riassunta. CLAUDE.md a livello di radice del progetto e auto memory sopravvivono alla compaction e si ricaricano dal disco; le istruzioni date solo in conversazione potrebbero andare perse. Esegui `/compact` per attivare manualmente, opzionalmente con un focus come `/compact focus on the API changes`.

Scopri di più: [What survives compaction](/docs/it/context-window#what-survives-compaction) · [When context fills up](/docs/it/how-claude-code-works#when-context-fills-up)

<h3 id="context-window">
  Context window
</h3>

La memoria di lavoro per una sessione, che contiene la cronologia della conversazione, i contenuti dei file, gli output dei comandi, CLAUDE.md, auto memory, le skills caricate e le istruzioni di sistema. Man mano che lavori, il contesto si riempie fino a quando [compaction](#compaction) lo riassume. Esegui `/context` per vedere cosa sta usando lo spazio. Per il concetto di modello sottostante, consulta il [glossario della piattaforma](https://platform.claude.com/docs/it/about-claude/glossary#context-window).

Scopri di più: [Explore the context window](/docs/it/context-window)

<h2 id="d">
  D
</h2>

<h3 id="dispatch">
  Dispatch
</h3>

Un router di attività avviato da telefono che genera una sessione di Claude Code nell'app Desktop quando invii un'attività di codifica dall'app mobile Claude. Il tuo prompt viene instradato automaticamente allo strumento giusto. Disponibile su piani Pro e Max.

Scopri di più: [Sessions from Dispatch](/docs/it/desktop#sessions-from-dispatch)

<h2 id="e">
  E
</h2>

<h3 id="effort-level">
  Effort level
</h3>

Un'impostazione che controlla quanto del budget di thinking con ragionamento adattivo Claude utilizza ad ogni turno. Uno sforzo più elevato significa più token di thinking e ragionamento più profondo; uno sforzo inferiore è più veloce ed economico. Effort è supportato su Fable 5, su Opus 4.6 e versioni successive, e su Sonnet 4.6 e versioni successive.

Scopri di più: [Adjust effort level](/docs/it/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended thinking
</h3>

Ragionamento passo dopo passo visibile che il modello esegue prima di rispondere. Puoi regolarlo con il [effort level](#effort-level), oppure limitare i token di thinking con `MAX_THINKING_TOKENS` su modelli con un budget di thinking fisso. Il thinking appare in testo grigio corsivo nel terminale.

Scopri di più: [Use extended thinking](/docs/it/model-config#extended-thinking)

<h2 id="h">
  H
</h2>

<h3 id="hook">
  Hook
</h3>

Un gestore definito dall'utente che viene eseguito automaticamente in un punto specifico del ciclo di vita di Claude Code, come prima dell'esecuzione di uno strumento, dopo una modifica di file o all'avvio della sessione. I gestori possono essere un comando shell, un endpoint HTTP, uno strumento MCP, un prompt LLM o un subagent. Gli hooks sono deterministici: si attivano in punti di ciclo di vita fissi piuttosto che a discrezione del modello.

Una configurazione di hook ha tre livelli:

* **Hook event**: il punto del ciclo di vita
* **Matcher**: filtra quali eventi lo attivano
* **Hook handler**: cosa viene eseguito

Scopri di più: [Get started with hooks](/docs/it/hooks-guide) · [Hooks reference](/docs/it/hooks)

<h2 id="m">
  M
</h2>

<h3 id="managed-settings">
  Managed settings
</h3>

Impostazioni applicate a livello di organizzazione da IT o DevOps, fornite dai server di Anthropic tramite la console di amministrazione o distribuite ai dispositivi in un percorso a livello di OS al di fuori di `~/.claude`. Le impostazioni gestite non possono essere ignorate dalle impostazioni utente e di progetto. La distribuzione gestita dal server si applica alle [configurazioni idonee](/docs/it/server-managed-settings#platform-availability); consulta le [Considerazioni sulla sicurezza](/docs/it/server-managed-settings#security-considerations). Usalo per politiche di sicurezza, requisiti di conformità o tooling standardizzato su una flotta.

Scopri di più: [Server-managed settings](/docs/it/server-managed-settings) · [Settings files](/docs/it/settings#settings-files)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

Uno standard aperto per connettere strumenti AI a fonti di dati esterne e servizi. I server MCP danno a Claude nuovi strumenti per Slack, Jira, database, browser e centinaia di altre integrazioni. Connetti i server tramite `/mcp` o aggiungendoli a `.mcp.json`. Per il protocollo stesso, consulta il [glossario della piattaforma](https://platform.claude.com/docs/it/about-claude/glossary#mcp-model-context-protocol).

Scopri di più: [Model Context Protocol](/docs/it/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

Un meccanismo di risparmio di contesto che rinvia gli schemi degli strumenti MCP fino a quando non sono necessari. Solo i nomi degli strumenti si caricano all'avvio; Claude recupera lo schema completo su richiesta quando decide di utilizzare uno strumento specifico. Questo impedisce ai server MCP inattivi di consumare molto contesto.

Scopri di più: [Scale with MCP Tool Search](/docs/it/mcp#scale-with-mcp-tool-search)

<h2 id="n">
  N
</h2>

<h3 id="non-interactive-mode">
  Non-interactive mode
</h3>

Una modalità che esegue un singolo prompt e esce senza una sessione conversazionale, invocata con `-p` o `--print`. Usata per CI, script e piping. L'esecuzione viene comunque salvata come una sessione ripristinabile a meno che non si passi `--no-session-persistence`. L'[Agent SDK](/docs/it/agent-sdk/overview) è l'equivalente Python e TypeScript. Precedentemente chiamata headless mode.

Scopri di più: [Run Claude Code programmatically](/docs/it/headless)

<h2 id="o">
  O
</h2>

<h3 id="output-style">
  Output style
</h3>

Una configurazione che modifica il prompt di sistema di Claude per cambiare il comportamento della risposta, il tono o il formato. Gli output styles disattivano le parti specifiche dell'ingegneria del software del prompt di sistema predefinito, a differenza di [CLAUDE.md](#claude-md) che viene consegnato come messaggio utente seguendo il prompt di sistema. Gli stili built-in includono Default, Proactive, Explanatory e Learning.

Scopri di più: [Output styles](/docs/it/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Permission mode
</h3>

Il comportamento di approvazione di base per la sessione. Cicla con `Shift+Tab` nella CLI o usa il selettore di modalità in VS Code, Desktop e claude.ai. Le modalità disponibili sono `default`, `acceptEdits`, `plan`, `auto`, `dontAsk` e `bypassPermissions`.

La modalità `default` è etichettata Manual nella CLI e nelle estensioni VS Code e JetBrains, e Claude Code accetta `manual` come alias per il valore.

Scopri di più: [Scegli una modalità di autorizzazione](/docs/it/permission-modes)

<h3 id="permission-rule">
  Permission rule
</h3>

Una voce di impostazioni che consente, chiede informazioni su o nega un'invocazione di uno strumento in base al nome dello strumento e al modello di argomento. Le regole vengono valutate deny→ask→allow, il primo match vince. Le permission rules sono controlli granulari sovrapposti alla più ampia [permission mode](#permission-mode).

Scopri di più: [Configura le autorizzazioni](/docs/it/permissions)

<h3 id="plan-mode">
  Plan mode
</h3>

Una [permission mode](#permission-mode) in cui Claude ricerca e propone modifiche senza modificare i tuoi file sorgente. Può leggere, cercare ed eseguire comandi di esplorazione, quindi presenta un piano per l'approvazione prima di toccare qualsiasi cosa. Entra in plan mode con `/plan` o premendo `Shift+Tab`.

Scopri di più: [Analizza prima di modificare con plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

Un bundle di skills, hooks, subagents e server MCP confezionati come una singola unità installabile. Le plugin skills sono spaziate dei nomi come `plugin-name:skill-name` in modo che più plugin coesistano. Distribuisci i plugin tra i team tramite un [marketplace](/docs/it/plugin-marketplaces).

Scopri di più: [Plugins](/docs/it/plugins)

<h3 id="project-trust">
  Project trust
</h3>

Una finestra di dialogo che accetta una directory prima che Claude Code carichi la sua configurazione. L'accettazione viene salvata per directory di progetto, ad eccezione della directory home, dove la fiducia viene mantenuta solo per la sessione corrente e il prompt riappare ad ogni avvio. Trust gates l'auto-installazione dei plugin del marketplace e l'esecuzione degli hooks definiti dal progetto. Fidarsi di una directory significa che i suoi file `.claude/settings.json`, `.mcp.json` e altri file di configurazione hanno effetto.

Scopri di più: [La directory `.claude`](/docs/it/claude-directory)

<h3 id="prompt-injection">
  Prompt injection
</h3>

Istruzioni ostili incorporate in un file, pagina web o risultato dello strumento che tentano di reindirizzare Claude verso azioni che non hai mai chiesto. Le difese di Claude Code includono il sistema di autorizzazioni, il rilevamento dell'iniezione di comandi e la verifica della fiducia. [Auto mode](#auto-mode) aggiunge una sonda lato server che scansiona i risultati degli strumenti per contenuti sospetti e un classificatore che non vede mai i risultati degli strumenti, quindi il testo iniettato non può influenzare le sue decisioni di approvazione.

Scopri di più: [Proteggi da prompt injection](/docs/it/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

Un modo per continuare una sessione locale di Claude Code dal tuo telefono o browser tramite claude.ai. Il tuo codice rimane sulla tua macchina; solo l'interfaccia utente è remota. Diverso da Claude Code sul web, che viene eseguito in una sandbox cloud.

Scopri di più: [Remote Control](/docs/it/remote-control)

<h3 id="rules">
  Rules
</h3>

File di istruzioni modulari in `.claude/rules/` che si caricano insieme a CLAUDE.md. Una regola può essere con ambito di percorso con frontmatter YAML `paths:` in modo che si carichi solo quando Claude legge un file corrispondente, mantenendo il contesto snello fino a quando non è rilevante.

Scopri di più: [Organize rules with `.claude/rules/`](/docs/it/memory#organize-rules-with-claude/rules/)

<h2 id="s">
  S
</h2>

<h3 id="sandboxing">
  Sandboxing
</h3>

Isolamento a livello di OS del filesystem e della rete per lo strumento Bash. I comandi vengono eseguiti all'interno di un confine che definisci in anticipo, in modo che Claude possa lavorare liberamente al suo interno senza prompt di approvazione per comando. Sandboxing è un livello separato dalle [permission rules](#permission-rule).

Scopri di più: [Sandboxing](/docs/it/sandboxing)

<h3 id="session">
  Session
</h3>

Una conversazione legata alla tua directory corrente, con la sua propria [context window](#context-window) indipendente. Le sessioni possono essere riprese con `claude -c`, fork con `--fork-session` per preservare la cronologia sotto un nuovo ID di sessione, o eseguite in parallelo tra i terminali. L'esecuzione di `/clear` avvia una nuova sessione; quella precedente rimane archiviata ed è disponibile tramite `/resume`. La trascrizione di ogni sessione è archiviata in `~/.claude/projects/`.

Scopri di più: [Work with sessions](/docs/it/how-claude-code-works#work-with-sessions)

<h3 id="settings-layers">
  Settings layers
</h3>

La gerarchia da cui Claude Code legge la configurazione, in ordine di precedenza dal più alto al più basso: [managed policy](#managed-settings), argomenti della riga di comando, impostazioni locali in `.claude/settings.local.json`, impostazioni di progetto in `.claude/settings.json`, quindi impostazioni utente in `~/.claude/settings.json`. Gli array si uniscono tra i livelli; gli scalari a un livello più alto sovrascrivono quelli inferiori.

Scopri di più: [Settings files](/docs/it/settings#settings-files)

<h3 id="skill">
  Skill
</h3>

Un file `SKILL.md` contenente istruzioni, conoscenze o un flusso di lavoro che Claude aggiunge al suo toolkit. Claude carica una skill automaticamente quando rilevante, o la invochi direttamente con `/skill-name`. Le skills seguono lo standard open Agent Skills; Claude Code lo estende con il controllo dell'invocazione e l'esecuzione del subagent.

Le skills sono il successore consigliato ai comandi personalizzati. Un file in `.claude/commands/deploy.md` e uno in `.claude/skills/deploy/SKILL.md` creano entrambi `/deploy` e funzionano allo stesso modo; i file di comando esistenti continuano a funzionare.

Scopri di più: [Extend Claude with skills](/docs/it/skills)

<h3 id="subagent">
  Subagent
</h3>

Un assistente AI specializzato che viene eseguito nella sua propria context window con un prompt di sistema personalizzato, accesso specifico agli strumenti e autorizzazioni indipendenti. Lavora su un'attività delegata e restituisce un riepilogo alla conversazione principale. Usa i subagents per mantenere grandi esplorazioni fuori dal tuo contesto primario o per eseguire ricerche parallele. Diverso da [agent teams](#agent-teams), dove ogni agente è una sessione completamente indipendente con cui puoi parlare direttamente.

I subagents built-in includono Explore, Plan e general-purpose.

Scopri di più: [Create custom subagents](/docs/it/sub-agents)

<h3 id="surface">
  Surface
</h3>

Qualsiasi luogo in cui accedi a Claude Code: la CLI, VS Code, JetBrains, Desktop o claude.ai. Tutte le surface condividono lo stesso motore, quindi il tuo CLAUDE.md, le impostazioni e le skills funzionano allo stesso modo su di esse. Slack e l'estensione Chrome sono integrazioni che si connettono a una surface piuttosto che essere surface stesse.

Scopri di più: [Platforms and integrations](/docs/it/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

Un comando, `/teleport`, che tira una sessione cloud di Claude Code nel tuo terminale locale. Claude recupera il branch, carica la cronologia della conversazione e riprende dallo stato dell'ultima sessione web. La direzione inversa è `--cloud`, che invia un'attività locale da eseguire sul web.

Scopri di più: [From web to terminal](/docs/it/claude-code-on-the-web#from-web-to-terminal)

<h3 id="tool">
  Tool
</h3>

Un'azione che Claude può intraprendere: leggere un file, modificare il codice, eseguire un comando shell, cercare il web, generare un subagent. Gli strumenti sono ciò che rende Claude Code agentic. Senza di essi, Claude può solo rispondere con testo. Ogni utilizzo di uno strumento restituisce un risultato che informa la decisione successiva di Claude nel [agentic loop](#agentic-loop).

Scopri di più: [Tools available to Claude](/docs/it/tools-reference)

<h3 id="turn">
  Turn
</h3>

Una risposta completa da Claude all'interno di una [session](#session). Un turn inizia quando invii un messaggio e termina quando Claude finisce di rispondere, con un numero qualsiasi di chiamate [tool](#tool) nel mezzo. Gli [stop hooks](#hook) si attivano alla fine di ogni turn. Una sessione è composta da molti turn, e l'[agentic loop](#agentic-loop) descrive ciò che accade all'interno di uno.

Scopri di più: [How Claude Code works](/docs/it/how-claude-code-works#the-agentic-loop)

<h2 id="v">
  V
</h2>

<h3 id="verification-loop">
  Verification loop
</h3>

Come una sessione sa che il lavoro è effettivamente completato piuttosto che solo plausibile. Dai a Claude un controllo che può eseguire, come una suite di test, una build o un confronto di screenshot, e Claude itera fino a quando il controllo non passa invece di fermarsi dopo un tentativo. Un verification loop è il prerequisito per [`/goal`](/docs/it/goal), esecuzioni incustodite e [dynamic workflows](/docs/it/workflows): senza uno, l'unica cosa che decide che l'agente ha finito è l'agente stesso.

Scopri di più: [Give Claude a way to verify its work](/docs/it/best-practices#give-claude-a-way-to-verify-its-work)

<h2 id="w">
  W
</h2>

<h3 id="worktree-isolation">
  Worktree isolation
</h3>

Una modalità di isolamento che esegue Claude in un worktree git separato in `.claude/worktrees/`, abilitata con il flag `-w` o `isolation: worktree` nella configurazione del subagent. Le modifiche rimangono su un branch separato in una directory separata, in modo che gli agenti paralleli non sovrascrivano i file l'uno dell'altro.

Scopri di più: [Run parallel sessions with git worktrees](/docs/it/worktrees)

***

<h2 id="deprecated-and-renamed-terms">
  Termini deprecati e rinominati
</h2>

Questi termini appaiono in documenti più vecchi, post di blog e contenuti della comunità. Usa il nome attuale quando cerchi su questo sito.

| Old term        | Now called                                    | Notes                                |
| --------------- | --------------------------------------------- | ------------------------------------ |
| Headless mode   | [Non-interactive mode](#non-interactive-mode) | Same `-p` flag, same behavior        |
| Custom commands | [Skills](#skill)                              | `.claude/commands/` files still work |
| Slash commands  | Commands                                      | "Slash" dropped from product copy    |
