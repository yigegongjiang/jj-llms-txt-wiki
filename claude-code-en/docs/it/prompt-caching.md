> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Come Claude Code utilizza il prompt caching

> Claude Code gestisce il prompt caching automaticamente. Scopri perché un cambio di modello attiva un turno lento senza cache, quanto costa `/compact`, perché le modifiche a CLAUDE.md non si applicano a metà sessione e come controllare il tasso di cache hit.

Il prompt caching rende Claude Code più veloce e più efficiente dal punto di vista dei costi. Senza caching, l'API rielaborerebbe la vostra cronologia completa ad ogni turno. Con il caching, riutilizza ciò che ha già elaborato e fa solo il nuovo lavoro per ciò che è cambiato.

Claude Code gestisce il prompt caching per voi, a meno che non lo [disabiliti](#disable-prompt-caching). È comunque utile sapere come funziona il prompt caching, perché alcune azioni invalidano la cache e rendono la risposta successiva più lenta e più costosa mentre la ricostruisce. Questa pagina copre quali azioni sono quelle, perché alcune impostazioni attendono un riavvio per applicarsi e come controllare le prestazioni della cache quando l'utilizzo sembra elevato.

<h2 id="how-the-cache-is-organized">
  Come è organizzata la cache
</h2>

Ogni volta che invii un messaggio in Claude Code, effettua una nuova richiesta API. Il modello non ricorda nulla tra le richieste, quindi Claude Code rinvia il contesto completo: il prompt di sistema, il contesto del tuo progetto, ogni messaggio precedente e risultato dello strumento, e il tuo nuovo messaggio. Il nuovo contenuto viene aggiunto alla fine, il che significa che la maggior parte di ogni richiesta è identica a quella precedente. Il prompt caching è il modo in cui l'API evita di rielaborare la parte che non è cambiata.

L'API memorizza nella cache abbinando l'inizio di ogni richiesta, chiamato prefisso, al contenuto che ha elaborato di recente. Su un turno normale, il prefisso è l'intera richiesta precedente e solo lo scambio più recente è nuovo. La corrispondenza è esatta, quindi una modifica in qualsiasi punto del prefisso ricalcola tutto ciò che viene dopo. Non esiste caching per file o per segmento. Vedi [come funziona il prompt caching](https://platform.claude.com/docs/it/build-with-claude/prompt-caching#how-prompt-caching-works) nel riferimento API per il meccanismo sottostante.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Quattro turni mostrati come barre orizzontali crescenti. La richiesta di ogni turno contiene tutto dal turno precedente più lo scambio più recente aggiunto alla fine. Nei turni due e tre, il prefisso invariato viene letto dalla cache e solo il nuovo scambio viene elaborato. Nel turno quattro, il prompt di sistema è cambiato, quindi il prefisso non corrisponde più e l'intera richiesta viene rielaborata e scritta." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Quattro turni mostrati come barre orizzontali crescenti. La richiesta di ogni turno contiene tutto dal turno precedente più lo scambio più recente aggiunto alla fine. Nei turni due e tre, il prefisso invariato viene letto dalla cache e solo il nuovo scambio viene elaborato. Nel turno quattro, il prompt di sistema è cambiato, quindi il prefisso non corrisponde più e l'intera richiesta viene rielaborata e scritta." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Per ottenere il massimo dall'abbinamento dei prefissi, Claude Code ordina ogni richiesta in modo che il contenuto che cambia raramente tra i turni venga per primo:

| Layer           | Contenuto                                                           | Cambia quando                                                                         |
| --------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| System prompt   | Istruzioni principali, definizioni degli strumenti, stile di output | Il set di definizioni degli strumenti caricati cambia, o Claude Code viene aggiornato |
| Project context | CLAUDE.md, memoria automatica, regole non scoped                    | La sessione inizia, o dopo `/clear` o `/compact`                                      |
| Conversation    | I tuoi messaggi, le risposte di Claude, i risultati degli strumenti | Ogni turno                                                                            |

Una modifica al layer della conversazione lascia il prompt di sistema e il contesto del progetto memorizzati nella cache. Una modifica al prompt di sistema invalida tutto, perché tutto il contenuto successivo ora si trova dietro un prefisso diverso. La terza colonna fornisce trigger comuni piuttosto che un elenco esaustivo, e le sezioni seguenti coprono l'insieme completo, incluso contenuto come lo stile di output che è fisso all'inizio della sessione.

La regola di abbinamento dei prefissi spiega la maggior parte dei comportamenti in questa pagina. [Plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) e [skill loading](/docs/it/skills), ad esempio, aggiungono le loro istruzioni come messaggi di conversazione, quindi il prefisso memorizzato nella cache rimane intatto.

Due impostazioni non fanno parte del testo del prompt, quindi non compaiono nella tabella dei layer, ma entrambe fanno parte della chiave della cache:

* **Model**: ogni modello ha la sua cache. Cambiare modelli ricalcola l'intera richiesta anche quando il contenuto è identico. Vedi [Switching models](#switching-models) di seguito.
* **Effort level**: ogni livello di sforzo ha la sua cache per lo stesso modello. Cambiarlo a metà sessione ricalcola l'intera richiesta, e Claude Code ti chiede di confermare prima di applicare il cambiamento. Vedi [Changing effort level](#changing-effort-level) di seguito.

<Tip>
  Scegli il tuo modello e il livello di sforzo all'inizio di una sessione, quindi salva `/compact` per le pause naturali tra i compiti. Meno modifiche fai a metà compito, più alto sarà il tuo tasso di cache hit.
</Tip>

<h3 id="where-the-cache-lives">
  Dove vive la cache
</h3>

Il caching avviene lato server, nell'infrastruttura che serve il tuo modello. Dove si trova dipende da come ti autentichi:

* **API key, Claude subscription, o [Claude Platform on AWS](/docs/it/claude-platform-on-aws)**: la cache vive nell'infrastruttura di Anthropic, accessibile tramite [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock o Google Cloud's Agent Platform**: la cache vive nell'infrastruttura di servizio del tuo provider cloud
* **Microsoft Foundry**: le richieste vengono instradate all'infrastruttura di Anthropic
* **Custom `ANTHROPIC_BASE_URL` o [LLM gateway](/docs/it/llm-gateway)**: la cache vive dove vengono inoltrate le tue richieste, e se il caching funziona dipende dal gateway

Per ciò che ogni provider memorizza ed elabora, vedi [data usage](/docs/it/data-usage). Ovunque viva la cache, le voci scadono dopo un periodo di inattività, e [Cache lifetime](#cache-lifetime) di seguito copre il TTL e come estenderlo.

<h2 id="actions-that-invalidate-the-cache">
  Azioni che invalidano la cache
</h2>

Queste azioni causano la mancanza di parte o tutta la cache nella richiesta successiva. Vedi un turno più lento e più costoso una sola volta, dopo il quale il nuovo prefisso viene memorizzato nella cache. La maggior parte di essi sono evitabili a metà compito una volta che sai che hanno un costo. Un cambio di modello può sembrare gratuito finché non noti il turno più lento che segue.

* [Switching models](#switching-models)
* [Changing effort level](#changing-effort-level)
* [Turning on fast mode](#turning-on-fast-mode)
* [Connecting or disconnecting an MCP server](#connecting-or-disconnecting-an-mcp-server)
* [Enabling or disabling a plugin](#enabling-or-disabling-a-plugin)
* [Denying an entire tool](#denying-an-entire-tool)
* [Compacting the conversation](#compacting-the-conversation)
* [Upgrading Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Switching models
</h3>

Ogni modello ha la sua cache. Cambiare con [`/model`](/docs/it/model-config#setting-your-model) significa che la richiesta successiva legge l'intera cronologia della conversazione senza cache hit, anche se il contenuto è identico.

L'impostazione [`opusplan` model](/docs/it/model-config#opusplan-model-setting) si risolve in Opus durante la modalità piano e Sonnet durante l'esecuzione, quindi ogni toggle della modalità piano è un cambio di modello e avvia una cache fresca.

Il [fallback automatico del modello](/docs/it/model-config#automatic-model-fallback) su Fable 5 è anche un cambio di modello. Quando un classificatore di sicurezza contrassegna una richiesta, Claude Code la riesegue sul modello Opus predefinito e la sessione continua lì.

<h3 id="changing-effort-level">
  Changing effort level
</h3>

La cache è codificata dal [livello di effort](/docs/it/model-config#adjust-effort-level) così come dal modello, quindi cambiare con `/effort` significa che la richiesta successiva legge l'intera cronologia della conversazione senza cache hit. Una volta che una conversazione è iniziata, Claude Code mostra una finestra di dialogo di conferma prima di applicare un cambio di effort che invaliderebbe la cache. Un cambio che si risolve nello stesso livello già in vigore, come impostare esplicitamente il valore predefinito del modello, salta la finestra di dialogo e mantiene la cache.

<h3 id="turning-on-fast-mode">
  Turning on fast mode
</h3>

L'abilitazione della [fast mode](/docs/it/fast-mode) aggiunge un'intestazione di richiesta che fa parte della chiave della cache, quindi la richiesta successiva legge l'intera cronologia della conversazione senza cache hit. Quei token di input non memorizzati nella cache vengono fatturati alle [tariffe della fast mode](/docs/it/fast-mode#understand-the-cost-tradeoff), motivo per cui attivarla all'inizio di una sessione costa meno che attivarla in profondità in una sessione lunga. L'abilitazione della fast mode da un modello non-Opus [cambia anche il tuo modello](#switching-models), il che avvia una cache fresca di per sé.

Il costo si applica una volta per conversazione. Dopo il primo turno della fast mode, Claude Code continua a inviare l'intestazione e varia solo l'impostazione di velocità della richiesta, che non fa parte della chiave della cache. Disattivare la fast mode, il [fallback automatico alla velocità standard](/docs/it/fast-mode#handle-rate-limits) dopo un limite di velocità, e riattivarla in seguito mantengono tutti la cache. `/clear` e `/compact` ripristinano questo, poiché ricostruiscono la cache in quei punti comunque.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Connecting or disconnecting an MCP server
</h3>

Le definizioni degli strumenti si trovano nel layer del prompt di sistema, quindi la cache si invalida quando l'insieme delle definizioni degli strumenti nella richiesta cambia tra i turni. Attivare lo [strumento advisor](/docs/it/advisor) è un'eccezione: la sua definizione si trova dopo il punto di interruzione della cache, quindi abilitare o disabilitare `/advisor` mantiene il prefisso memorizzato nella cache intatto. Se un cambio di [MCP server](/docs/it/mcp) fa questo dipende dal fatto che i suoi strumenti siano rimandati dalla [tool search](/docs/it/mcp#scale-with-mcp-tool-search) o caricati nel prefisso:

* **Strumenti rimandati**, il valore predefinito sui modelli supportati: un server che si connette, disconnette, o cambia il suo elenco di strumenti aggiunge solo nuovo contenuto e non disturba nulla che sia già memorizzato nella cache.
* **Strumenti caricati nel prefisso**: qualsiasi cambio ad essi invalida la cache. Questo accade quando [tool search non è disponibile o disabilitato](/docs/it/mcp#configure-tool-search), come su Google Cloud's Agent Platform o con un gateway `ANTHROPIC_BASE_URL` personalizzato. Accade anche per un server o uno strumento contrassegnato [`alwaysLoad`](/docs/it/mcp#exempt-a-server-from-deferral), e per le definizioni mantenute in primo piano dal [caricamento basato su soglia](/docs/it/mcp#configure-tool-search).

Quando gli strumenti si caricano nel prefisso, la causa più comune di un'invalidazione è un server che si connette o disconnette a metà sessione, il che può accadere senza alcuna azione da parte tua: il processo di un server stdio esce, una sessione HTTP scade, o un server [si riconnette automaticamente dopo un errore transitorio](/docs/it/mcp#automatic-reconnection). Un server connesso può anche inviare un [dynamic tool update](/docs/it/mcp#dynamic-tool-updates) che cambia il suo elenco di strumenti.

Modificare la tua configurazione MCP non cambia la cache di per sé. La nuova configurazione ha effetto solo dopo un riavvio, che è quando il server si connette o disconnette.

<h3 id="enabling-or-disabling-a-plugin">
  Enabling or disabling a plugin
</h3>

I [plugin](/docs/it/plugins) raggruppano diversi tipi di componenti, e il costo di un cambio dipende da quali componenti il plugin fornisce. Skills, comandi, agenti, hooks, server LSP, monitor e temi non invalidano mai la cache: qualsiasi cosa aggiungano alla richiesta viene aggiunta dopo la conversazione esistente, quindi la richiesta successiva paga per il nuovo contenuto ma legge comunque tutto ciò che lo precede dalla cache.

L'eccezione è un plugin che fornisce [MCP server](/docs/it/plugins-reference#mcp-servers). Abilitare o disabilitare uno segue le stesse regole di [connessione o disconnessione di un MCP server](#connecting-or-disconnecting-an-mcp-server): la cache sopravvive quando gli strumenti del server sono rimandati, e la richiesta successiva rilegge l'intera conversazione quando si caricano nel prefisso.

I cambiamenti dei plugin si applicano quando esegui [`/reload-plugins`](/docs/it/discover-plugins#apply-plugin-changes-without-restarting) o avvii una nuova sessione. Il costo, sia annunci aggiunti che una rilettura completa, si mostra al primo turno dopo il ricaricamento, non quando esegui `/plugin install`, `/plugin enable`, o `/plugin disable`. A partire da v2.1.163, quando un ricaricamento attiverebbe la rilettura completa, `/reload-plugins` mostra un avviso e non applica il ricaricamento. Passa `--force` per applicare comunque.

Disabilitare un plugin che hai abilitato in precedenza nella sessione ripristina la forma di richiesta precedente. Se quel prefisso è ancora entro la sua [durata della cache](#cache-lifetime), la richiesta successiva legge la voce di cache più vecchia invece di ricostruirla.

<h3 id="denying-an-entire-tool">
  Denying an entire tool
</h3>

Aggiungere un nome di strumento semplice come `Bash` o `WebFetch` come [deny rule](/docs/it/permissions#manage-permissions) rimuove completamente quello strumento dal contesto di Claude. Le definizioni degli strumenti incorporati si caricano nel layer del prompt di sistema, quindi aggiungere o rimuovere una di queste regole a metà sessione invalida la cache. La modifica ha effetto al turno successivo sia che la aggiungi tramite `/permissions` o [modificando direttamente un file di impostazioni](/docs/it/settings#when-edits-take-effect).

Solo una deny rule che corrisponde nella posizione del nome dello strumento ha questo effetto: un nome di strumento semplice, la forma equivalente `Bash(*)`, o un [tool-name glob](/docs/it/permissions#tool-name-wildcards) come `"*"`. Un glob che corrisponde solo agli strumenti MCP, come `"mcp__*"`, rimuove quegli strumenti allo stesso modo ma lascia la cache intatta quando gli strumenti corrispondenti sono [rimandati](#connecting-or-disconnecting-an-mcp-server), il valore predefinito, poiché le definizioni rimandate non erano mai nel prefisso memorizzato nella cache. Le deny rule con ambito come `Bash(rm *)`, e tutte le regole di consentimento e richiesta, non cambiano quali strumenti Claude vede. Claude Code le controlla quando Claude tenta una chiamata, lasciando il prefisso intatto.

<h3 id="compacting-the-conversation">
  Compacting the conversation
</h3>

[Compaction](/docs/it/context-window#what-survives-compaction) sostituisce la tua cronologia dei messaggi con un riepilogo. Per progettazione, questo invalida il layer della conversazione, poiché la richiesta successiva ha una cronologia nuova e più breve che non condivide un prefisso con quella vecchia. Claude Code riutilizza il layer del prompt di sistema e ricarica il contesto del progetto dal disco, che ha cache hit solo se CLAUDE.md e la memoria sono invariati dall'inizio della sessione.

Per produrre il riepilogo, Claude Code invia una richiesta una tantum con lo stesso prompt di sistema, strumenti e cronologia della tua conversazione, più un'istruzione di riepilogo aggiunta come messaggio utente finale. Poiché condivide il tuo prefisso, quella richiesta legge la cache esistente piuttosto che rielaborare la cronologia completa. La maggior parte del tempo di compaction va alla generazione del riepilogo, non a una cache miss. Il turno che segue ricostruisce la cache della conversazione solo per il riepilogo molto più breve, quindi il turno post-compaction non è la parte lenta.

<Tip>
  La compaction funziona a tuo favore quando il contesto che scardi è contenuto di cui non hai più bisogno. Per scegliere quando il suo overhead accade, esegui `/compact` a una pausa naturale nel tuo lavoro, come tra i compiti, invece di aspettare che la compaction automatica si attivi a metà compito. Se sei andato su un percorso che vuoi abbandonare completamente, [`/rewind`](#rewinding-the-conversation) a un turno precedente invece. Il rewind tronca a un prefisso che è già memorizzato nella cache, piuttosto che costruirne uno nuovo come fa la compaction.
</Tip>

<h3 id="upgrading-claude-code">
  Upgrading Claude Code
</h3>

Una nuova versione di Claude Code in genere aggiorna il prompt di sistema o le definizioni degli strumenti, quindi la prima richiesta dopo un aggiornamento ricostruisce la cache dall'inizio. [Auto-update](/docs/it/setup#auto-updates) scarica le nuove versioni in background ma le applica al prossimo avvio, mai a metà sessione, quindi lo vedi come un primo turno senza cache dopo il riavvio piuttosto che una sorpresa durante una sessione. Imposta `DISABLE_AUTOUPDATER=1` per controllare quando gli aggiornamenti si applicano.

<Note>
  [Resuming a session](/docs/it/sessions#resume-a-session) dopo un aggiornamento rielabora l'intera cronologia della conversazione senza cache hit, poiché la cronologia ora si trova dietro un prompt di sistema diverso. Il costo scala con la lunghezza della conversazione ripresa, quindi il primo turno di ritorno in una sessione lunga può essere la richiesta più costosa che invii.
</Note>

<h2 id="actions-that-keep-the-cache">
  Azioni che mantengono la cache
</h2>

Queste azioni aggiungono alla fine della conversazione o non toccano affatto la richiesta. Alcune di esse, come modificare CLAUDE.md o cambiare lo stile di output, sono anche il motivo per cui una modifica dell'impostazione attende un riavvio per applicarsi.

* [Modifica dei file nel tuo repository](#editing-files-in-your-repository)
* [Modifica di CLAUDE.md durante la sessione](#editing-claude-md-mid-session)
* [Cambio dello stile di output](#changing-output-style)
* [Cambio della modalità di autorizzazione](#changing-permission-mode)
* [Invocazione di skills e comandi](#invoking-skills-and-commands)
* [Esecuzione di `/recap`](#running-%2Frecap)
* [Riavvolgimento della conversazione](#rewinding-the-conversation)
* [Generazione di un subagent](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Modifica dei file nel tuo repository
</h3>

I contenuti dei file entrano nel contesto solo quando Claude li legge, e le letture si aggiungono alla conversazione. Modificare un file che Claude ha letto in precedenza non cambia retroattivamente la lettura precedente nella cronologia. Invece, Claude Code aggiunge un `<system-reminder>` notando che il file è cambiato, e Claude lo rilegge se necessario.

<h3 id="editing-claude-md-mid-session">
  Modifica di CLAUDE.md durante la sessione
</h3>

I tuoi file CLAUDE.md a livello di radice del progetto e a livello utente vengono letti una sola volta all'inizio della sessione e mantenuti in memoria. Modificarli durante la sessione non invalida la cache, ma la modifica non si applica nemmeno. Claude continua a lavorare con la versione che è stata caricata all'inizio della sessione. Il nuovo contenuto viene caricato al prossimo `/clear`, `/compact` o riavvio.

[I file CLAUDE.md annidati nelle sottodirectory](/docs/it/memory) e [le regole con frontmatter `paths:`](/docs/it/memory#path-specific-rules) vengono caricati in seguito, quando Claude legge per la prima volta un file corrispondente. Modificarne uno prima che venga caricato ha effetto. Dopo che viene caricato, il contenuto fa parte della cronologia della conversazione, quindi una modifica durante la sessione non lo cambia retroattivamente.

<h3 id="changing-output-style">
  Cambio dello stile di output
</h3>

[Lo stile di output](/docs/it/output-styles) fa parte del prompt di sistema, che Claude Code legge una sola volta all'inizio della sessione. Cambiarlo tramite `/config` o l'impostazione `outputStyle` durante la sessione non invalida la cache, ma il cambiamento non si applica nemmeno. Claude continua a usare lo stile che è stato caricato all'inizio della sessione. Il nuovo stile viene caricato al prossimo `/clear` o riavvio.

<h3 id="changing-permission-mode">
  Cambio della modalità di autorizzazione
</h3>

Passare tra [modalità di autorizzazione](/docs/it/permission-modes), come da predefinito ad accettare modifiche, non cambia il prompt di sistema o le definizioni degli strumenti, quindi i cambi di modalità sono sicuri per la cache. L'eccezione è la modalità piano con l'impostazione [`opusplan`](/docs/it/model-config#opusplan-model-setting) del modello, che cambia il modello tra Opus e Sonnet quando entri o esci dalla modalità piano. Questo rende il toggle della modalità un [cambio di modello](#switching-models).

<h3 id="invoking-skills-and-commands">
  Invocazione di skills e comandi
</h3>

[Skills](/docs/it/skills) e [comandi](/docs/it/commands) iniettano le loro istruzioni come messaggi utente nel punto di invocazione. Nulla prima nella conversazione cambia.

<h3 id="running-/recap">
  Esecuzione di `/recap`
</h3>

[`/recap`](/docs/it/interactive-mode#session-recap) genera un riepilogo per la visualizzazione nel tuo terminale. A differenza di `/compact`, aggiunge il riepilogo come output del comando piuttosto che sostituire la tua cronologia dei messaggi, quindi il prefisso memorizzato nella cache rimane intatto.

<h3 id="rewinding-the-conversation">
  Riavvolgimento della conversazione
</h3>

[`/rewind`](/docs/it/checkpointing) tronca la tua conversazione a un turno precedente. La cronologia rimanente è lo stesso contenuto da cui la cache è stata costruita in quel momento, e i layer del prompt di sistema e del contesto del progetto sono invariati, quindi la richiesta successiva colpisce la voce della cache precedente. Ogni turno da allora ha letto attraverso quel prefisso, che ha mantenuto la voce calda anche se il turno originale era più tempo fa del TTL.

Il ripristino dei checkpoint dei file insieme alla conversazione non ha alcun effetto separato sulla cache. I contenuti dei file entrano nel contesto solo quando Claude li legge, come [modifica dei file nel tuo repository](#editing-files-in-your-repository).

<h2 id="cache-lifetime">
  Cache lifetime
</h2>

I prefissi memorizzati nella cache scadono dopo un periodo di inattività. Ogni richiesta che colpisce la cache ripristina il timer, quindi la cache rimane calda finché continui a lavorare. Dopo un intervallo abbastanza lungo, la richiesta successiva ricalcola l'input completo e ristabilisce la cache, il che è il motivo per cui il primo turno di ritorno dopo essersi allontanato può essere notevolmente più lento.

Il time to live (TTL) controlla per quanto tempo un intervallo la cache sopravvive. L'API offre due: un TTL di cinque minuti e un [TTL di un'ora](https://platform.claude.com/docs/it/build-with-claude/prompt-caching#1-hour-cache-duration) che mantiene la cache calda attraverso pause più lunghe ma [fattura le scritture della cache a una velocità più elevata](https://platform.claude.com/docs/it/build-with-claude/prompt-caching#pricing). Claude Code sceglie il TTL per te in base a come ti autentichi, e puoi sovrascriverlo con variabili di ambiente.

<h3 id="on-a-claude-subscription">
  On a Claude subscription
</h3>

Su una Claude subscription, Claude Code richiede automaticamente il TTL di un'ora. L'utilizzo è incluso nel tuo piano piuttosto che fatturato per token, quindi il TTL più lungo non ti costa nulla in più e influisce solo su quanto tempo la tua cache rimane calda.

Se hai superato il limite di utilizzo del tuo piano e Claude Code sta attingendo ai [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), ti viene fatturato quell'utilizzo, quindi Claude Code abbassa automaticamente il TTL a cinque minuti.

<h3 id="on-an-api-key-or-third-party-provider">
  On an API key or third-party provider
</h3>

Su una API key, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform on AWS, paghi le tariffe per token, quindi il TTL rimane ai cinque minuti più economici per impostazione predefinita. Per optare per il [TTL di un'ora](https://platform.claude.com/docs/it/build-with-claude/prompt-caching#1-hour-cache-duration), imposta `ENABLE_PROMPT_CACHING_1H=1`.

Su Amazon Bedrock, il supporto del prompt caching, la lunghezza minima del prefisso memorizzabile nella cache e la disponibilità del TTL di un'ora variano a seconda del modello. Se i conteggi dei token della cache rimangono a zero, controlla [supported models, regions, and limits](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) nella documentazione di Amazon Bedrock.

<h3 id="override-the-ttl">
  Override the TTL
</h3>

Imposta `FORCE_PROMPT_CACHING_5M=1` per forzare il TTL di cinque minuti indipendentemente dall'autenticazione. Questo è utile quando stai eseguendo il debug del comportamento della cache, confrontando i due TTL, o sovrascrivendo un `ENABLE_PROMPT_CACHING_1H` impostato in [managed settings](/docs/it/settings#settings-files).

<h2 id="cache-scope">
  Cache scope
</h2>

In Claude Code, la cache è effettivamente scoped a una macchina e una directory. Il prompt di sistema incorpora la directory di lavoro, la piattaforma, la shell, la versione del sistema operativo e i percorsi della memoria automatica, quindi due sessioni in directory diverse costruiscono prefissi diversi e si perdono la cache l'una dell'altra. Questo include i worktrees dello stesso repository, poiché ogni worktree ha la sua directory di lavoro.

Le sessioni che esegui in parallelo nella stessa directory costruiscono prefissi corrispondenti e leggono la cache l'una dell'altra. Le sessioni sequenziali condividono il prefisso solo quando lo snapshot dello stato git all'avvio corrisponde, poiché il prompt di sistema cattura anche il ramo e i commit recenti.

La cache API sottostante è più ampia. Le cache sono isolate tra le organizzazioni e, su alcuni provider, [tra i workspace all'interno di un'organizzazione](https://platform.claude.com/docs/it/build-with-claude/prompt-caching#cache-storage-and-sharing). All'interno di questi confini, qualsiasi due richieste con lo stesso modello e prefisso leggono la stessa cache. Per i chiamanti dell'Agent SDK che eseguono flotte di processi automatizzati, vedi [improve prompt caching across users and machines](/docs/it/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) per sopprimere le sezioni per macchina del prompt di sistema e condividere la cache tra le macchine.

<h2 id="check-cache-performance">
  Verificare le prestazioni della cache
</h2>

Le prestazioni della cache si mostrano come due conteggi di token che l'API segnala su ogni risposta. Il modo più diretto per guardarli dal vivo è uno [statusline script](/docs/it/statusline) che legge l'oggetto `current_usage`:

| Field                         | Meaning                                                                                              |
| ----------------------------- | ---------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Token scritti nella cache su questo turno, fatturati alla velocità di scrittura della cache          |
| `cache_read_input_tokens`     | Token serviti dalla cache su questo turno, fatturati a circa il 10% della velocità di input standard |

Un alto rapporto lettura-creazione significa che il caching funziona bene. Se la creazione rimane alta turno dopo turno, qualcosa sta cambiando nel tuo prefisso. La sezione [actions that invalidate the cache](#actions-that-invalidate-the-cache) elenca le cause usuali.

Per la visibilità in un'organizzazione, l'esportatore OpenTelemetry segnala i token di lettura e creazione della cache per utente e sessione. Vedi [Monitor usage](/docs/it/monitoring-usage) per il riferimento degli attributi di metrica e evento.

<h2 id="subagents-and-the-cache">
  Subagents and the cache
</h2>

Un [subagent](/docs/it/sub-agents) avvia la sua propria conversazione con il suo prompt di sistema e set di strumenti, separato da quello del genitore. Costruisce la sua propria cache, iniziando senza cache hit sulla sua prima chiamata e riscaldandosi attraverso i suoi turni. I subagent utilizzano il TTL di cinque minuti anche su una subscription, poiché il TTL di un'ora automatico si applica alla conversazione principale.

La cache del genitore non è interessata. Dal lato del genitore, la chiamata e il risultato del subagent si aggiungono alla conversazione, lasciando il prefisso del genitore intatto.

Un [fork](/docs/it/sub-agents#fork-the-current-conversation), al contrario, eredita il prompt di sistema, gli strumenti e la cronologia della conversazione del genitore esattamente, quindi la sua prima richiesta legge la cache del genitore. La chiamata di riepilogo della compaction descritta in [Compacting the conversation](#compacting-the-conversation) utilizza lo stesso approccio di condivisione dei prefissi.

<h2 id="disable-prompt-caching">
  Disabilita prompt caching
</h2>

Disabilitare il caching è occasionalmente utile quando si esegue il debug del comportamento della cache con un modello o provider specifico. Per disattivarlo, imposta una di queste variabili di ambiente su `1`:

| Variable                        | Effect                         |
| ------------------------------- | ------------------------------ |
| `DISABLE_PROMPT_CACHING`        | Disabilita per tutti i modelli |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Disabilita per Haiku solo      |
| `DISABLE_PROMPT_CACHING_SONNET` | Disabilita per Sonnet solo     |
| `DISABLE_PROMPT_CACHING_OPUS`   | Disabilita per Opus solo       |
| `DISABLE_PROMPT_CACHING_FABLE`  | Disabilita per Fable solo      |

Per impostare la politica di caching in un'organizzazione, metti una di queste o le [TTL variables](#cache-lifetime) nel blocco `env` di [managed settings](/docs/it/settings#settings-files). Per l'uso normale, lascia il caching abilitato.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Lessons from building Claude Code: Prompt caching is everything](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): la logica di progettazione per la modalità piano, il caricamento differito degli strumenti e la compaction
* [Explore the context window](/docs/it/context-window): cosa viene caricato nel contesto e quando
* [Reduce token usage](/docs/it/costs#reduce-token-usage): strategie oltre il caching per gestire la dimensione del contesto
* [Track and reduce costs](/docs/it/agent-sdk/cost-tracking): tracciamento dei token della cache e configurazione del TTL per i chiamanti dell'Agent SDK
* [Prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching): il meccanismo API sottostante, i breakpoint e i prezzi
