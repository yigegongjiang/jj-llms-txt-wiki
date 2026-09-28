> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Come Claude Code utilizza il prompt caching

> Claude Code gestisce il prompt caching automaticamente. Scopri perché un cambio di modello attiva un turno lento senza cache, quanto costa `/compact`, perché le modifiche a CLAUDE.md non si applicano a metà sessione e come controllare il tasso di cache hit.

Il prompt caching rende Claude Code più veloce e più efficiente dal punto di vista dei costi. Senza caching, l'API rielaborerebbe la vostra cronologia completa ad ogni turno. Con il caching, riutilizza ciò che ha già elaborato, fattura la rilettura al [tasso di token memorizzati nella cache](https://platform.claude.com/docs/en/about-claude/pricing) e elabora completamente solo ciò che è cambiato.

Claude Code gestisce il prompt caching per voi, a meno che non lo [disabiliti](#disable-prompt-caching). È comunque utile sapere come funziona il prompt caching, perché alcune azioni invalidano la cache e rendono la risposta successiva più lenta e più costosa mentre la ricostruisce. Questa pagina copre quali azioni sono quelle, perché alcune impostazioni attendono un riavvio per applicarsi e come controllare le prestazioni della cache quando l'utilizzo sembra elevato.

<h2 id="how-the-cache-is-organized">
  Come è organizzata la cache
</h2>

Ogni volta che inviate un messaggio in Claude Code, viene effettuata una nuova richiesta API. Il modello non ricorda nulla tra le richieste, quindi Claude Code invia di nuovo il contesto completo: il prompt di sistema, il contesto del vostro progetto, ogni messaggio precedente e risultato dello strumento, e il vostro nuovo messaggio. I nuovi contenuti vengono aggiunti alla fine, il che significa che la maggior parte di ogni richiesta è identica a quella precedente. Il prompt caching è il modo in cui l'API evita di rielaborare la parte che non è cambiata.

L'API memorizza nella cache confrontando l'inizio di ogni richiesta, chiamato prefisso, con il contenuto che ha elaborato di recente. In un turno normale, il prefisso è l'intera richiesta precedente e solo lo scambio più recente è nuovo. La corrispondenza è esatta, quindi una modifica in qualsiasi punto del prefisso ricalcola tutto ciò che viene dopo. Non esiste una cache per file o per segmento. Consultate [come funziona il prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#how-prompt-caching-works) nel riferimento API per il meccanismo sottostante.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Quattro turni mostrati come barre orizzontali crescenti. La richiesta di ogni turno contiene tutto dal turno precedente più lo scambio più recente aggiunto alla fine. Nei turni due e tre, il prefisso invariato viene letto dalla cache e solo il nuovo scambio viene elaborato. Nel turno quattro, il prompt di sistema è cambiato, quindi il prefisso non corrisponde più e l'intera richiesta viene rielaborata e scritta." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Quattro turni mostrati come barre orizzontali crescenti. La richiesta di ogni turno contiene tutto dal turno precedente più lo scambio più recente aggiunto alla fine. Nei turni due e tre, il prefisso invariato viene letto dalla cache e solo il nuovo scambio viene elaborato. Nel turno quattro, il prompt di sistema è cambiato, quindi il prefisso non corrisponde più e l'intera richiesta viene rielaborata e scritta." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Per ottenere il massimo dalla corrispondenza del prefisso, Claude Code ordina ogni richiesta in modo che il contenuto che cambia raramente tra i turni venga per primo:

| Layer           | Contenuto                                                             | Cambia quando                                               |
| --------------- | --------------------------------------------------------------------- | ----------------------------------------------------------- |
| System prompt   | Istruzioni principali, definizioni degli strumenti                    | L'insieme delle definizioni degli strumenti caricati cambia |
| Project context | CLAUDE.md, memoria automatica, regole non scoped                      | La sessione inizia, oppure dopo `/clear` o `/compact`       |
| Conversation    | I vostri messaggi, le risposte di Claude, i risultati degli strumenti | Ogni turno                                                  |

Una modifica al layer della conversazione lascia il prompt di sistema e il contesto del progetto memorizzati nella cache. Una modifica al prompt di sistema invalida tutto, perché tutto il contenuto successivo si trova ora dietro un prefisso diverso. La terza colonna fornisce trigger comuni piuttosto che un elenco esaustivo, e le sezioni seguenti coprono l'insieme completo.

La regola di corrispondenza del prefisso spiega la maggior parte dei comportamenti su questa pagina. [Plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) e [skill loading](/docs/it/skills), ad esempio, aggiungono le loro istruzioni come messaggi di conversazione, quindi il prefisso memorizzato nella cache rimane intatto.

Due impostazioni non compaiono nella tabella dei layer ma influiscono comunque su ciò che rimane memorizzato nella cache:

* **Model**: ogni modello ha la propria cache. Cambiare modello ricalcola l'intera richiesta anche quando il contenuto è identico. Consultate [Switching models](#switching-models) di seguito.
* **Effort level**: sulla maggior parte dei modelli, ogni livello di sforzo ha la propria cache, quindi cambiare lo sforzo a metà sessione ricalcola l'intera richiesta. Su Opus 5.5 e Fable 5.1 con una chiave API o un abbonamento Claude, la cache rimane intatta per impostazione predefinita. Consultate [Changing effort level](#changing-effort-level) di seguito.

<Tip>
  Scegliete il vostro modello e il livello di sforzo all'inizio di una sessione, quindi riservate `/compact` per le pause naturali tra i compiti. Meno modifiche apportate a metà compito, più alto sarà il vostro tasso di cache hit.
</Tip>

<h3 id="where-the-cache-lives">
  Dove vive la cache
</h3>

La memorizzazione nella cache avviene lato server, nell'infrastruttura che serve il vostro modello. Dove si trova dipende da come vi autenticate:

* **API key, Claude subscription, o [Claude Platform on AWS](/docs/it/claude-platform-on-aws)**: la cache vive nell'infrastruttura di Anthropic, accessibile tramite [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock o Google Cloud's Agent Platform**: la cache vive nell'infrastruttura di servizio del vostro provider cloud
* **Microsoft Foundry**: dipende dall'[hosting option](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) della distribuzione. Le distribuzioni ospitate su Azure vengono servite sull'infrastruttura Azure; le distribuzioni ospitate su Anthropic vengono servite sull'infrastruttura di Anthropic
* **Custom `ANTHROPIC_BASE_URL` o [LLM gateway](/docs/it/llm-gateway)**: la cache vive ovunque le vostre richieste vengono inoltrate, e se il caching funziona dipende dal gateway

Claude Code aggiunge anche il contesto di sistema a metà conversazione, come notifiche di cambio file, e contrassegna quel blocco per la memorizzazione nella cache su ogni provider e connessione a meno che non impostiate [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/it/llm-gateway-protocol#disable-pre-release-capabilities), nel qual caso quel blocco viene inviato senza cache.

All'endpoint proprio del provider, Amazon Bedrock e il suo [Mantle endpoint](/docs/it/amazon-bedrock#use-the-mantle-endpoint), Google Cloud's Agent Platform, e Microsoft Foundry memorizzano nella cache il blocco nello stesso modo in cui lo fa Claude API.

Quando le vostre richieste passano attraverso un [LLM gateway](/docs/it/llm-gateway), un `ANTHROPIC_BASE_URL` personalizzato, o un override di URL di base del provider cloud come [`ANTHROPIC_BEDROCK_BASE_URL`](/docs/it/env-vars), ciò che rimane memorizzato nella cache dipende da come il gateway gestisce i [marcatori `cache_control`](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#explicit-cache-breakpoints) che Claude Code invia:

* **Li inoltra invariati**: il blocco e la vostra conversazione vengono memorizzati nella cache nello stesso modo dell'endpoint proprio del provider.
* **Rifiuta la richiesta contrassegnata con un errore `400` che nomina `cache_control`**: Claude Code invia di nuovo la richiesta con il marcatore spostato dal blocco al vostro ultimo messaggio di conversazione, e lo mantiene lì per il resto della conversazione. Il blocco viene fatturato come input non memorizzato nella cache; la vostra conversazione rimane memorizzata nella cache.
* **Rimuove i marcatori mentre restituisce il successo**: l'intera cronologia della conversazione viene fatturata come input non memorizzato nella cache ad ogni turno. Un gateway che converte il contenuto del sistema in forma di blocco in una stringa semplice rilascia il marcatore nello stesso modo.

Per ciò che ogni provider memorizza ed elabora, consultate [data usage](/docs/it/data-usage). Ovunque viva la cache, le voci scadono dopo un periodo di inattività, e [Cache lifetime](#cache-lifetime) di seguito copre il TTL e come estenderlo.

<h2 id="actions-that-invalidate-the-cache">
  Azioni che invalidano la cache
</h2>

Queste azioni causano la mancanza della cache nella richiesta successiva, in parte o completamente. Vedrai un turno più lento e più costoso una sola volta, dopo il quale il nuovo prefisso viene memorizzato nella cache. La maggior parte di esse è evitabile durante un'attività una volta che conosci il loro costo. Un cambio di modello può sembrare gratuito finché non noti il turno più lento che segue.

* [Cambio di modelli](#switching-models)
* [Modifica del livello di sforzo](#changing-effort-level)
* [Attivazione della modalità veloce](#turning-on-fast-mode)
* [Connessione o disconnessione di un server MCP](#connecting-or-disconnecting-an-mcp-server)
* [Abilitazione o disabilitazione di un plugin](#enabling-or-disabling-a-plugin)
* [Negazione di uno strumento completo](#denying-an-entire-tool)
* [Compattazione della conversazione](#compacting-the-conversation)
* [Accumulo di molte immagini](#accumulating-many-images)
* [Aggiornamento di Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Cambio di modelli
</h3>

Ogni modello ha la propria cache. Passare con [`/model`](/docs/it/model-config#setting-your-model) significa che la richiesta successiva legge l'intera cronologia della conversazione senza cache hit, anche se il contenuto è identico.

Quando esegui `/model` nel terminale, Claude Code ti chiede di confermare il cambio solo mentre la cache è ancora calda e il nuovo modello non è quello che ha prodotto l'ultima risposta. La cache rimane calda per un [cache TTL](#cache-lifetime) dopo che Claude Code ha inviato l'ultima richiesta in questa conversazione o dopo che Claude ha risposto. Una volta trascorso quel tempo, la cache è scaduta, quindi Claude Code passa senza chiedere.

Prima della v2.1.238, Claude Code non controllava il cache TTL e chiedeva anche dopo che la cache era scaduta.

Puoi anche richiedere questa conferma o saltarla con un [hook PreModelSwitch](/docs/it/hooks#premodelswitch-decision-control).

L'[impostazione del modello `opusplan`](/docs/it/model-config#opusplan-model-setting) si risolve in Opus durante la modalità piano e Sonnet durante l'esecuzione, quindi ogni attivazione/disattivazione della modalità piano è un cambio di modello e avvia una cache nuova.

[Il fallback automatico del modello](/docs/it/model-config#automatic-model-fallback) sui modelli Fable, Opus 5.5 e Opus 5 è anche un cambio di modello. Quando un classificatore di sicurezza contrassegna una richiesta in una categoria che ha un modello di fallback, Claude Code riesegue la richiesta su quel modello e la sessione continua lì.

Quando il frontmatter di una skill o di un comando nomina un [`model`](/docs/it/skills#frontmatter-reference) diverso dal modello corrente della sessione, quel turno è anche un cambio di modello: la richiesta successiva legge l'intera cronologia della conversazione senza cache hit. Il modello della sessione riprende al tuo prossimo prompt. Una skill `context: fork` imposta il [modello del subagent con fork](/docs/it/skills#run-skills-in-a-subagent) invece.

<h3 id="changing-effort-level">
  Modifica del livello di sforzo
</h3>

Sulla maggior parte dei modelli, modificare il [livello di sforzo](/docs/it/model-config#adjust-effort-level) a metà sessione significa che la richiesta successiva legge l'intera cronologia della conversazione senza cache hit. Mentre la cache è ancora calda, Claude Code ti chiede di confermare il cambio prima.

Su Opus 5.5 e Fable 5.1 con una chiave API o un abbonamento Claude, modificare lo sforzo mantiene la cache, e Claude Code applica il nuovo livello senza chiedere. Questo non si applica su Amazon Bedrock, su Google Cloud's Agent Platform, o su un [gateway di app Claude](/docs/it/claude-apps-gateway), o quando imposti [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/it/llm-gateway-protocol#disable-pre-release-capabilities) o la tua organizzazione ha una configurazione HIPAA.

Prima della v2.1.260, modificare lo sforzo su Fable 5.1 con una chiave API o un abbonamento Claude invalidava anche la cache.

<h3 id="turning-on-fast-mode">
  Attivazione della modalità veloce
</h3>

L'abilitazione della [modalità veloce](/docs/it/fast-mode) aggiunge un'intestazione di richiesta che fa parte della chiave di cache, quindi la prima richiesta che Claude Code invia con la modalità veloce attiva legge l'intera cronologia della conversazione senza cache hit. Claude Code imposta quell'intestazione una volta quando inizia un turno e la mantiene per l'intero turno, quindi quando attivi la modalità veloce mentre Claude sta lavorando, il cache miss dall'intestazione si verifica sulla prima richiesta del tuo turno successivo. Quei token di input non memorizzati nella cache vengono fatturati alle [tariffe della modalità veloce](/docs/it/fast-mode#understand-the-cost-tradeoff), motivo per cui attivare la modalità veloce all'inizio di una sessione costa meno che attivarla in profondità in una sessione lunga. Se il tuo modello attuale non supporta la modalità veloce, l'abilitazione della modalità veloce [cambia anche il tuo modello](#switching-models), e quel cambio avvia una cache nuova dalla richiesta successiva nel turno in esecuzione.

Il costo si applica una volta per conversazione. Dopo il primo turno in modalità veloce, Claude Code continua a inviare l'intestazione e varia solo l'impostazione di velocità della richiesta, che non fa parte della chiave di cache. Disattivare la modalità veloce, il [fallback automatico alla velocità standard](/docs/it/fast-mode#handle-rate-limits) dopo un limite di velocità, e riattivarla in seguito mantengono tutti la cache. Se [esaurisci i crediti di utilizzo](/docs/it/fast-mode#handle-rate-limits) a metà sessione, Claude Code ritenta ogni richiesta in modalità veloce rifiutata alla velocità standard nello stesso modo, quindi questo fallback mantiene anche la cache. `/clear` e `/compact` ripristinano questo, poiché ricostruiscono la cache in quei punti comunque.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Connessione o disconnessione di un server MCP
</h3>

Le definizioni degli strumenti si trovano nel livello del prompt di sistema, quindi la cache si invalida quando l'insieme delle definizioni degli strumenti nella richiesta cambia tra i turni. Attivare/disattivare lo [strumento advisor](/docs/it/advisor) è un'eccezione: la sua definizione si trova dopo il punto di interruzione della cache, quindi abilitare o disabilitare `/advisor` mantiene il prefisso memorizzato nella cache intatto. Se un cambio di [server MCP](/docs/it/mcp) fa questo dipende dal fatto che i suoi strumenti siano differiti dalla [ricerca degli strumenti](/docs/it/mcp#scale-with-mcp-tool-search) o caricati nel prefisso:

* **Strumenti differiti**, l'impostazione predefinita sui modelli supportati: un server che si connette, si disconnette, o cambia il suo elenco di strumenti aggiunge solo nuovo contenuto e non disturba nulla già memorizzato nella cache.
* **Strumenti caricati nel prefisso**: qualsiasi modifica a essi invalida la cache. Questo accade quando la [ricerca degli strumenti non è disponibile o è disabilitata](/docs/it/mcp#configure-tool-search), ad esempio sui modelli di Google Cloud's Agent Platform precedenti alla generazione Claude 4.5, con un gateway `ANTHROPIC_BASE_URL` personalizzato, o su una distribuzione Microsoft Foundry [ospitata su Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) una volta che Claude Code rileva che la distribuzione rifiuta la ricerca degli strumenti. Accade anche per un server o uno strumento contrassegnato [`alwaysLoad`](/docs/it/mcp#exempt-a-server-from-deferral), e per le definizioni mantenute in primo piano dal [caricamento basato su soglia](/docs/it/mcp#configure-tool-search).

Quando gli strumenti si caricano nel prefisso, la causa più comune di un'invalidazione è un server che si connette o si disconnette a metà sessione, il che può accadere senza alcuna azione da parte tua: il processo di un server stdio esce, una sessione HTTP scade, o un server [si riconnette automaticamente dopo un errore transitorio](/docs/it/mcp#automatic-reconnection). Un server connesso può anche inviare un [aggiornamento dinamico dello strumento](/docs/it/mcp#dynamic-tool-updates) che cambia il suo elenco di strumenti.

Modificare la tua configurazione MCP non cambia di per sé la cache. La nuova configurazione ha effetto solo dopo un riavvio, che è quando il server si connette o si disconnette.

<h3 id="enabling-or-disabling-a-plugin">
  Abilitazione o disabilitazione di un plugin
</h3>

Quando abiliti o disabiliti un [plugin](/docs/it/plugins/overview), il costo del cambio dipende da quali tipi di componenti fornisce il plugin. I casi seguenti coprono ogni tipo di componente, quando Claude Code applica il cambio, e cosa accade quando disabiliti di nuovo un plugin nella stessa sessione.

<h4 id="plugin-components-that-keep-the-cache">
  Componenti del plugin che mantengono la cache
</h4>

Claude Code non invalida mai la cache per le skill, i comandi, gli agenti, gli hook, i monitor o i temi di un plugin. Aggiunge il loro contenuto dopo la conversazione esistente, quindi la richiesta successiva paga per quel contenuto e legge comunque tutto ciò che lo precede dalla cache.

<h4 id="plugins-that-provide-mcp-servers">
  Plugin che forniscono server MCP
</h4>

Quando abiliti o disabiliti un plugin che fornisce [server MCP](/docs/it/plugins/components#mcp-servers), Claude Code segue le stesse regole di quando [connetti o disconnetti un server MCP](#connecting-or-disconnecting-an-mcp-server):

* Se Claude Code differisce gli strumenti del server, mantiene la cache.
* Se Claude Code li carica nel prefisso, la richiesta successiva rilegge l'intera conversazione.

<h4 id="code-intelligence-plugins">
  Plugin di intelligenza del codice
</h4>

Quando abiliti un [plugin di intelligenza del codice](/docs/it/plugins/code-intelligence), Claude ottiene lo [strumento LSP](/docs/it/tools-reference#lsp-tool-behavior).

<h4 id="when-plugin-changes-apply">
  Quando i cambiamenti del plugin si applicano
</h4>

Un cambio che fai nel menu `/plugin` passa attraverso [`/reload-plugins`](/docs/it/plugins/cli-reference#reload-plugins), che Claude Code esegue per te quando chiudi il menu. Paghi il costo, sia annunci aggiunti che una rilettura completa, al primo turno dopo che il cambio si applica. Claude Code può anche applicare un cambio da solo:

* Per un plugin con una fonte `command`, Claude Code [può ricaricare il plugin stesso](/docs/it/plugins/loading#when-a-command-source-re-runs).
* Quando [installi un plugin dall'interfaccia `/plugin`](/docs/it/plugins/install#install-a-plugin), Claude Code può attivarlo durante l'installazione. Il riepilogo dell'installazione ti dice se l'ha fatto.
* Quando [sposti la sessione con `/cd`](/docs/it/permissions#move-the-session-to-another-directory) su v2.1.246 o successiva, Claude Code applica i plugin che le impostazioni della nuova directory abilitano come parte dello spostamento, senza l'avviso di rilettura completa che tiene un `/reload-plugins`.
* Nelle sessioni interattive, quando aggiungi o rimuovi un plugin in una [cartella di plugin](/docs/it/plugins/create#load-a-directory-or-archive-for-one-session) che hai passato con `--plugin-dir`, il cambio si applica subito. Se applicarlo attiverebbe una rilettura completa, Claude Code trattiene il cambio e mostra un avviso per eseguire `/reload-plugins`. Richiede Claude Code v2.1.265 o successiva.

Quando `/reload-plugins` viene eseguito e il ricaricamento attiverebbe una rilettura completa, Claude Code mostra un avviso e non applica il ricaricamento. Esegui `/reload-plugins --force` per applicarlo comunque.

`/reload-plugins` viene eseguito anche in sessioni senza un terminale interattivo, come l'app desktop, l'Agent SDK, e la [modalità non interattiva](/docs/it/headless) con `-p`, quando lo digiti direttamente nella sessione. Richiede Claude Code v2.1.260 o successiva.

In quelle sessioni il ricaricamento applica tutto tranne i cambiamenti del server MCP del plugin, che [hanno effetto nella tua sessione successiva](/docs/it/plugins/cli-reference#reload-plugins) e quindi non costano mai una rilettura completa a metà sessione.

<h4 id="plugins-you-enable-and-then-disable-in-one-session">
  Plugin che abiliti e poi disabiliti in una sessione
</h4>

Quando disabiliti un plugin che hai abilitato in precedenza nella sessione, Claude Code ripristina la forma di richiesta precedente. Se quel prefisso è ancora entro la sua [durata della cache](#cache-lifetime), la richiesta successiva legge la voce di cache più vecchia invece di ricostruire.

<h3 id="denying-an-entire-tool">
  Negazione di uno strumento completo
</h3>

Se aggiungi un nome di strumento semplice come `Bash` o `WebFetch` come [regola di negazione](/docs/it/permissions#manage-permissions), Claude non può chiamare quello strumento dalla tua richiesta successiva in poi, sia che tu aggiunga la regola tramite `/permissions` o [modificando direttamente un file di impostazioni](/docs/it/settings#when-edits-take-effect). Questo include una regola che aggiungi tramite `/permissions` nel mezzo di un turno.

Quando la [ricerca degli strumenti](/docs/it/mcp#scale-with-mcp-tool-search) è attiva, che è l'impostazione predefinita sui modelli supportati, le definizioni degli strumenti nella richiesta non cambiano e il prefisso memorizzato nella cache sopravvive. Quando la ricerca degli strumenti non è disponibile o è disabilitata, Claude Code rimuove la definizione dalla richiesta successiva, il che invalida la cache, e così fa anche la rimozione della regola in seguito.

Solo una regola di negazione che corrisponde nella posizione del nome dello strumento blocca uno strumento in questo modo: un nome di strumento semplice, la forma equivalente `Bash(*)`, o un [glob del nome dello strumento](/docs/it/permissions#tool-name-wildcards) come `"*"`. Un glob che corrisponde solo agli strumenti MCP, come `"mcp__*"`, blocca quegli strumenti nello stesso modo. Le regole di negazione con ambito come `Bash(rm *)`, e tutte le regole di consentimento e richiesta, non cambiano quali strumenti Claude vede. Claude Code le controlla quando Claude tenta una chiamata, lasciando il prefisso intatto.

<h3 id="compacting-the-conversation">
  Compattazione della conversazione
</h3>

La [compattazione](/docs/it/context-window#what-survives-compaction) sostituisce la cronologia dei tuoi messaggi con un riepilogo. Per progettazione, questo invalida il livello della conversazione, poiché la richiesta successiva ha una cronologia nuova e più breve che non condivide un prefisso con quella vecchia. Claude Code riutilizza il livello del prompt di sistema a meno che la conversazione non sia stata [ripresa mantenendo un prompt di sistema che altrimenti sarebbe cambiato](#resuming-a-session); in quel caso la prima compattazione passa al prompt corrente e quel livello si ricostruisce una volta. Ricarica il contesto del progetto dal disco, che cache-hit solo se CLAUDE.md e la memoria sono invariati da quando la sessione è iniziata.

Per produrre il riepilogo, Claude Code invia una richiesta separata con lo stesso prompt di sistema, strumenti e cronologia della tua conversazione, più un'istruzione di riepilogo aggiunta come messaggio utente finale. Mentre la cache è calda, quella richiesta legge il tuo prefisso dalla cache, quindi un `/compact` a metà sessione costa una frazione di quello che la dimensione del contesto suggerisce e spende la maggior parte del suo tempo generando il riepilogo.

Dopo una pausa più lunga della [durata della cache](#cache-lifetime), non c'è cache rimasta da leggere, quindi la richiesta di riepilogo rielabora la cronologia completa come input non memorizzato nella cache. Questo è il motivo per cui `/compact` costa di più quando [riprendi una sessione vecchia](/docs/it/sessions#resume-from-a-summary). In entrambi i casi caldi e freddi, il turno dopo la compattazione ricostruisce la cache della conversazione solo per il riepilogo molto più breve, quindi quel turno non è la parte lenta.

<Tip>
  La compattazione funziona a tuo favore quando il contesto che scarta è contenuto che non ti serve più. Per scegliere quando il suo sovraccarico accade, esegui `/compact` a una pausa naturale nel tuo lavoro, ad esempio tra le attività, invece di aspettare che la compattazione automatica si attivi a metà attività. Se sei andato su un percorso che vuoi abbandonare completamente, [`/rewind`](#rewinding-the-conversation) a un turno precedente. Il riavvolgimento tronca a un prefisso che è già memorizzato nella cache, piuttosto che costruirne uno nuovo come fa la compattazione.
</Tip>

<h3 id="accumulating-many-images">
  Accumulo di molte immagini
</h3>

L'API limita quante immagini e PDF ogni richiesta può contenere. Per i numeri attuali, vedi [Limiti di richiesta](https://platform.claude.com/docs/en/build-with-claude/vision#request-limits) nella documentazione dell'API. Claude Code limita anche la dimensione totale delle immagini e dei PDF in una richiesta, quindi gli screenshot grandi raggiungono il limite con meno immagini di quelli piccoli.

Quando la richiesta successiva supererebbe uno dei due limiti, Claude Code rimuove un batch delle immagini e dei PDF più vecchi da quello che invia, il che lascia spazio per altri prima di dover rimuovere di nuovo. Claude non può più vedere le immagini rimosse. Se Claude ne ha bisogno di nuovo, condividila di nuovo.

La rimozione di immagini cambia i messaggi che le contenevano, quindi la richiesta successiva rielabora la conversazione dal primo di quei messaggi in poi. Poiché Claude Code rimuove un batch alla volta, vedi un turno più lento per batch piuttosto che uno con ogni nuovo screenshot.

<h3 id="upgrading-claude-code">
  Aggiornamento di Claude Code
</h3>

Una nuova versione di Claude Code in genere aggiorna il prompt di sistema o le definizioni degli strumenti, quindi la prima conversazione che inizi dopo un aggiornamento costruisce la sua cache da zero. L'[aggiornamento automatico](/docs/it/setup#auto-updates) scarica le nuove versioni in background ma le applica al prossimo avvio, mai a metà sessione, quindi lo vedi come un primo turno non memorizzato nella cache dopo il riavvio piuttosto che una sorpresa durante una sessione. Imposta `DISABLE_AUTOUPDATER=1` per controllare quando gli aggiornamenti si applicano.

<Note>
  Per quello che costa riprendere una conversazione che hai iniziato prima dell'aggiornamento, vedi [Ripresa di una sessione](#resuming-a-session).
</Note>

<h2 id="actions-that-keep-the-cache">
  Azioni che mantengono la cache
</h2>

Queste azioni aggiungono alla fine della conversazione o non toccano affatto la richiesta. Alcune di esse, come la modifica di CLAUDE.md, mantengono la cache per lo stesso motivo per cui la modifica non raggiunge la sessione in esecuzione fino a `/clear`, `/compact` o un riavvio.

* [Modifica di file nel tuo repository](#editing-files-in-your-repository)
* [Modifica di CLAUDE.md durante la sessione](#editing-claude-md-mid-session)
* [Modifica della modalità di autorizzazione](#changing-permission-mode)
* [Modifica dello stile di output](#changing-output-style)
* [Invocazione di skills e comandi](#invoking-skills-and-commands)
* [Esecuzione di `/recap`](#running-%2Frecap)
* [Ripristino della conversazione](#rewinding-the-conversation)
* [Generazione di un subagent](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Modifica di file nel tuo repository
</h3>

I contenuti dei file entrano nel contesto solo quando Claude li legge, e le letture si aggiungono alla conversazione. La modifica di un file che Claude ha precedentemente letto non cambia retroattivamente la lettura precedente nella cronologia. Invece, Claude Code aggiunge un `<system-reminder>` che nota il cambio del file, e Claude lo rilegge se necessario.

<h3 id="editing-claude-md-mid-session">
  Modifica di CLAUDE.md durante la sessione
</h3>

I tuoi file CLAUDE.md a livello di radice del progetto e a livello utente vengono letti una sola volta all'inizio della sessione e mantenuti in memoria. La modifica di questi file durante la sessione non invalida la cache, ma la modifica non si applica nemmeno. Claude continua a lavorare con la versione che è stata caricata all'inizio della sessione. Il nuovo contenuto viene caricato al prossimo `/clear`, `/compact` o riavvio.

[I file CLAUDE.md annidati nelle sottodirectory](/docs/it/memory) e [le regole con frontmatter `paths:`](/docs/it/memory#path-specific-rules) vengono caricati successivamente, quando Claude legge per la prima volta un file corrispondente. La modifica di uno prima che venga caricato ha effetto. Dopo che viene caricato, il contenuto fa parte della cronologia della conversazione, quindi una modifica durante la sessione non lo cambia retroattivamente.

<h3 id="changing-permission-mode">
  Modifica della modalità di autorizzazione
</h3>

Il passaggio tra [modalità di autorizzazione](/docs/it/permission-modes), ad esempio da Manuale ad accettazione di modifiche, non cambia il prompt di sistema o le definizioni degli strumenti, quindi i cambi di modalità sono sicuri per la cache. L'eccezione è la modalità plan con l'impostazione del modello [`opusplan`](/docs/it/model-config#opusplan-model-setting), che commuta il modello tra Opus e Sonnet quando entri o esci dalla modalità plan. Questo rende il toggle della modalità un [cambio di modello](#switching-models).

<h3 id="changing-output-style">
  Modifica dello stile di output
</h3>

Quando cambi [stili di output](/docs/it/output-styles) durante la sessione con [`/output-style`](/docs/it/output-styles#change-your-output-style), `/config` o l'impostazione `outputStyle`, Claude utilizza il nuovo stile a partire dal tuo prossimo messaggio. Claude Code fornisce le istruzioni del nuovo stile come messaggio nella conversazione, quindi quella richiesta legge comunque il prompt di sistema e la conversazione precedente dalla cache.

Prima della v2.1.251, un cambio di stile durante la sessione manteneva la cache ma non si applicava fino a quando non eseguivi `/clear` o non avviavi una nuova sessione.

<h3 id="invoking-skills-and-commands">
  Invocazione di skills e comandi
</h3>

[Skills](/docs/it/skills) e [comandi](/docs/it/commands) iniettano le loro istruzioni come messaggi utente nel punto di invocazione. Nulla di precedente nella conversazione cambia. Una skill o un comando il cui frontmatter nomina un `model` può essere un [cambio di modello](#switching-models) per quel turno.

<h3 id="running-/recap">
  Esecuzione di `/recap`
</h3>

[`/recap`](/docs/it/interactive-mode#session-recap) genera un riepilogo per la visualizzazione nel tuo terminale. A differenza di `/compact`, aggiunge il riepilogo come output del comando piuttosto che sostituire la cronologia dei tuoi messaggi, quindi il prefisso memorizzato nella cache rimane intatto.

<h3 id="rewinding-the-conversation">
  Ripristino della conversazione
</h3>

[`/rewind`](/docs/it/checkpointing) tronca la tua conversazione fino a un turno precedente. La cronologia rimanente è lo stesso contenuto da cui la cache è stata costruita in quel momento, e il prompt di sistema e i livelli di contesto del progetto rimangono invariati, quindi la richiesta successiva raggiunge la voce di cache precedente. Ogni turno da allora ha letto attraverso quel prefisso, che ha mantenuto la voce attiva anche se il turno originale era più tempo fa rispetto al TTL.

Il ripristino dei checkpoint dei file insieme alla conversazione non ha alcun effetto separato sulla cache. I contenuti dei file entrano nel contesto solo quando Claude li legge, come nella [modifica di file nel tuo repository](#editing-files-in-your-repository).

<h2 id="resuming-a-session">
  Ripresa di una sessione
</h2>

Quando [riprendete una sessione](/docs/it/sessions#resume-a-session), Claude Code invia di nuovo l'intera conversazione e la richiesta legge dalla cache qualsiasi parte del suo prefisso che rimane invariata e ancora entro la [durata della cache](#cache-lifetime). La tabella dei livelli in cima a questa pagina indica quali modifiche apporta ogni livello.

Il prompt di sistema cambierebbe dopo un [aggiornamento di Claude Code](#upgrading-claude-code) o con un testo [`--append-system-prompt`](/docs/it/cli-reference#system-prompt-flags) diverso alla ripresa. Per impostazione predefinita, la conversazione ripresa mantiene il prompt di sistema con cui è stata avviata, quindi la sua cronologia rimane dietro lo stesso prompt e la modifica ha effetto una volta che la conversazione viene compattata o in una nuova conversazione. [System prompt flags in resumed conversations](/docs/it/cli-reference#system-prompt-flags-in-resumed-conversations) copre i casi in cui Claude Code ricostruisce il prompt su ogni richiesta.

<h2 id="cache-lifetime">
  Cache lifetime
</h2>

I prefissi memorizzati nella cache scadono dopo un periodo di inattività. Ogni richiesta che colpisce la cache ripristina il timer, quindi la cache rimane calda finché continui a lavorare. Dopo un intervallo abbastanza lungo, la richiesta successiva ricalcola l'input completo e ristabilisce la cache, il che è il motivo per cui il primo turno di ritorno dopo essersi allontanato può essere notevolmente più lento.

Su un piano Pro o Max, quando riprendi una sessione di grandi dimensioni dopo una lunga pausa, Claude Code [offre di riprendere da un riepilogo](/docs/it/sessions#resume-from-a-summary) in modo che le richieste successive non portino la cronologia completa.

Il time to live (TTL) controlla per quanto tempo un intervallo la cache sopravvive. L'API offre due: un TTL di cinque minuti e un [TTL di un'ora](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#1-hour-cache-duration) che mantiene la cache calda attraverso pause più lunghe ma [fattura le scritture della cache a una velocità più elevata](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). Il TTL più lungo aiuta quando lasci una sessione inattiva e torni ad essa, perché salti la rielaborazione che un prefisso scaduto comporta. Costa di più su brevi raffiche di lavoro che non rimangono mai inattive oltre cinque minuti, dove si applica la velocità di scrittura più elevata e la durata della cache più lunga rimane inutilizzata.

<h3 id="which-ttl-each-request-gets">
  Which TTL each request gets
</h3>

Claude Code decide il TTL per richiesta, e ogni richiesta rientra in uno di due bucket fissi:

* **Main conversation**: i tuoi turni interattivi, le esecuzioni non interattive `-p` e i turni Agent SDK, più gli helper che Claude Code esegue inline con essi
* **Everything else**: le richieste che Claude Code effettua al di fuori di quella conversazione, come [subagents](/docs/it/sub-agents), [workflows](/docs/it/workflows), [teammates](/docs/it/agent-teams) in-process, fork, compaction e titoli di sessione

A meno che tu non scelga un TTL tu stesso, Claude Code richiede il TTL di un'ora solo su una Claude subscription entro l'utilizzo incluso nel tuo piano. Lì richiede l'ora per la conversazione principale, più un piccolo insieme di richieste helper che Anthropic controlla lato server. Questa tabella fornisce il TTL predefinito di ogni bucket in entrambi i tipi di fatturazione.

| Request bucket    | Claude subscription, within plan usage                                         | Usage credits, API key, or cloud provider |
| ----------------- | ------------------------------------------------------------------------------ | ----------------------------------------- |
| Main conversation | One hour                                                                       | Five minutes                              |
| Everything else   | Five minutes, except the server-controlled helper requests, which get one hour | Five minutes                              |

Una volta superato il limite di utilizzo del tuo piano e Claude Code attinge ai [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), ti viene fatturato quell'utilizzo, quindi Claude Code abbassa la conversazione principale al TTL di cinque minuti più economico. Per mantenere il TTL di un'ora lì, [scegli il TTL tu stesso](#choose-the-ttl-yourself).

<h3 id="choose-the-ttl-yourself">
  Choose the TTL yourself
</h3>

Puoi impostare un TTL per uno dei due bucket. Ogni controllo accetta `5m` o `1h`, e Claude Code ignora qualsiasi altro valore.

* **Main conversation**: l'impostazione [`promptCacheTtl`](/docs/it/settings-reference#promptcachettl), o la variabile di ambiente [environment variable](/docs/it/env-vars) `CLAUDE_CODE_PROMPT_CACHE_TTL`
* **Everything else**: l'impostazione [`subagentPromptCacheTtl`](/docs/it/settings-reference#subagentpromptcachettl), o la variabile di ambiente `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`

Entrambe le impostazioni e entrambe le variabili di ambiente richiedono Claude Code v2.1.242 o successivo. Se accedi con una API key o utilizzi un cloud provider, imposta `promptCacheTtl` su `1h` per dare alla conversazione principale una cache di un'ora. Le richieste al di fuori di essa mantengono il default di cinque minuti fino a quando non scegli un TTL per quel bucket anche.

Quando si applica più di un controllo, Claude Code prende la prima corrispondenza in questo ordine:

1. `FORCE_PROMPT_CACHING_5M=1`, che forza cinque minuti per entrambi i bucket
2. La variabile di ambiente del bucket
3. L'impostazione del bucket
4. Per le richieste di un subagent, il valore `cacheTtl` nel campo frontmatter [`experimental`](/docs/it/sub-agents#supported-frontmatter-fields) del subagent, che richiede Claude Code v2.1.248 o successivo. Claude Code ignora un `1h` lì mentre la tua Claude subscription sta utilizzando usage credits
5. `ENABLE_PROMPT_CACHING_1H=1`, che richiede un'ora per entrambi i bucket
6. Il [default per il bucket della richiesta](#which-ttl-each-request-gets)

Imposta `FORCE_PROMPT_CACHING_5M=1` quando stai eseguendo il debug del comportamento della cache, confrontando i due TTL, o sovrascrivendo un TTL più lungo impostato in [managed settings](/docs/it/managed-settings).

Per confermare quale TTL le scritture della cache della tua conversazione principale hanno utilizzato, esegui `claude -p "hello" --output-format json` e leggi `usage.cache_creation` nel risultato. Claude Code segnala le scritture della cache di un'ora sotto `ephemeral_1h_input_tokens` e le scritture della cache di cinque minuti sotto `ephemeral_5m_input_tokens`.

Attraverso un gateway LLM che imposti con `ANTHROPIC_BASE_URL`, parte della richiesta di un'ora viaggia nell'intestazione `anthropic-beta`, quindi configura il gateway per [inoltrare quell'intestazione invariata](/docs/it/llm-gateway-protocol#request-headers). Il TTL di un'ora non è disponibile attraverso il [Claude apps gateway](/docs/it/claude-apps-gateway#availability-and-limitations). Su Amazon Bedrock, il supporto del prompt caching, la lunghezza minima del prefisso memorizzabile nella cache e la disponibilità del TTL di un'ora variano a seconda del modello. Se i conteggi dei token della cache rimangono a zero, controlla [supported models, regions, and limits](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) nella documentazione di Amazon Bedrock.

<h2 id="cache-scope">
  Cache scope
</h2>

In Claude Code, la cache è effettivamente scoped a una macchina e una directory. Ogni conversazione porta con sé la directory di lavoro, la piattaforma, la shell e la versione del sistema operativo, e il prompt di sistema nomina i tuoi percorsi di memoria automatica, quindi due sessioni in directory diverse costruiscono prefissi diversi e si perdono la cache l'una dell'altra. Questo include i worktrees dello stesso repository, poiché ogni worktree ha la sua directory di lavoro.

Le sessioni che esegui in parallelo nella stessa directory costruiscono prefissi corrispondenti e leggono la cache l'una dell'altra. Le sessioni sequenziali condividono il prefisso solo quando lo snapshot dello stato git all'avvio corrisponde, poiché ogni conversazione porta anche il ramo e i commit recenti da quello snapshot.

La cache API sottostante è più ampia. Le cache sono isolate tra le organizzazioni e, su alcuni provider, [tra i workspace all'interno di un'organizzazione](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#cache-storage-and-sharing). All'interno di questi confini, qualsiasi due richieste con lo stesso modello e prefisso leggono la stessa cache. Per i chiamanti dell'Agent SDK che eseguono flotte di processi automatizzati, vedi [improve prompt caching across users and machines](/docs/it/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) per sopprimere le sezioni per macchina del prompt di sistema e condividere la cache tra le macchine.

<h2 id="check-cache-performance">
  Verificare le prestazioni della cache
</h2>

Le prestazioni della cache si mostrano come due conteggi di token che l'API segnala su ogni risposta. Il modo più diretto per guardarli dal vivo è uno [statusline script](/docs/it/statusline) che legge l'oggetto `current_usage`:

| Field                         | Meaning                                                                                                                                                                                                           |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Token scritti nella cache su questo turno, fatturati alla velocità di scrittura della cache                                                                                                                       |
| `cache_read_input_tokens`     | Token serviti dalla cache su questo turno, fatturati al [tasso di token memorizzato nella cache](https://platform.claude.com/docs/en/about-claude/pricing) del modello, inferiore alla velocità di input standard |

Un alto rapporto lettura-creazione significa che il caching funziona bene. Se la creazione rimane alta turno dopo turno, qualcosa sta cambiando nel tuo prefisso. La sezione [actions that invalidate the cache](#actions-that-invalidate-the-cache) elenca le cause usuali.

Per un riepilogo per sessione, esegui `/usage`. Dopo la prima risposta della conversazione principale, Claude Code aggiunge una [`Prompt cache (main)` line](/docs/it/costs#prompt-cache-statistics) al blocco Session, mostrando il rapporto di hit della sessione, il conteggio dei miss e se la cache è calda in questo momento. Uno script statusline può leggere gli stessi numeri dall'[oggetto `prompt_cache`](/docs/it/statusline#prompt-cache-fields). Entrambi richiedono Claude Code v2.1.251 o successivo.

La riga `Prompt cache (main)` nomina anche la probabile causa dell'ultimo miss quando Claude Code riesce a identificarne una, ad esempio `likely cause: tool definitions changed`. Il testo della probabile causa richiede Claude Code v2.1.260 o successivo.

Per la visibilità in un'organizzazione, l'esportatore OpenTelemetry segnala i token di lettura e creazione della cache per utente e sessione. Vedi [Monitor usage](/docs/it/monitoring-usage) per il riferimento degli attributi di metrica e evento.

<h2 id="subagents-and-the-cache">
  Subagent e la cache
</h2>

Un [subagent](/docs/it/sub-agents) avvia la sua propria conversazione con il suo prompt di sistema e set di strumenti, separato da quello del genitore. La sua prima richiesta non legge la cache del genitore, perché i due prefissi differiscono, e riscalda una cache propria attraverso i suoi turni. I subagent rimangono al di fuori del [bucket TTL](#which-ttl-each-request-gets) della conversazione principale, quindi ottengono cinque minuti anche su una subscription fino a quando non [scegli uno più lungo](#choose-the-ttl-yourself).

La cache del genitore non è interessata. Dal lato del genitore, la chiamata e il risultato del subagent si aggiungono alla conversazione, lasciando il prefisso del genitore intatto.

Un [fork](/docs/it/sub-agents#fork-the-current-conversation), al contrario, eredita il prompt di sistema, gli strumenti e la cronologia della conversazione del genitore esattamente, quindi la sua prima richiesta legge la cache del genitore.

Altre richieste possono anche leggere un prefisso che una richiesta precedente ha memorizzato nella cache:

* **Session copies**: una sessione che [copi con `/fork`](/docs/it/agent-view#copy-the-session-with-%2Ffork) riceve la sua istruzione di isolamento come messaggio alla fine della conversazione copiata, quindi la cache che la conversazione originale ha costruito rimane intatta.
* **Compaction**: la chiamata di riepilogo descritta in [Compacting the conversation](#compacting-the-conversation) utilizza lo stesso approccio di condivisione dei prefissi.
* **Resumed subagents**: quando Claude [riprende un subagent](/docs/it/sub-agents#resume-subagents), la prima richiesta dell'esecuzione ripresa può leggere la cache che l'esecuzione originale ha riscaldato.
* **Workflow fan-outs**: in un [workflow fan-out](/docs/it/workflows#prompt-caching-in-a-fan-out) di agenti con lo stesso prefisso, Claude Code tiene tutti tranne il primo per un massimo di 5 secondi per impostazione predefinita, quindi le loro prime richieste possono leggere il prefisso che il primo agente ha memorizzato nella cache.

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

Per impostare la politica di caching in un'organizzazione, metti una di queste o le [TTL variables](#cache-lifetime) nel blocco `env` di [managed settings](/docs/it/managed-settings). Per l'uso normale, lascia il caching abilitato.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Lessons from building Claude Code: Prompt caching is everything](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): la logica di progettazione per la modalità piano, il caricamento differito degli strumenti e la compaction
* [Explore the context window](/docs/it/context-window): cosa viene caricato nel contesto e quando
* [Reduce token usage](/docs/it/costs#reduce-token-usage): strategie oltre il caching per gestire la dimensione del contesto
* [Track and reduce costs](/docs/it/agent-sdk/cost-tracking): tracciamento dei token della cache e configurazione del TTL per i chiamanti dell'Agent SDK
* [Prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching): il meccanismo API sottostante, i breakpoint e i prezzi
