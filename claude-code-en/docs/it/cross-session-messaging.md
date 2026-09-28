> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Messaggi tra le tue altre sessioni di Claude Code

> Consenti a Claude di elencare e inviare messaggi alle tue altre sessioni di Claude Code su questa macchina, e raggiungi le tue sessioni su altre macchine o nel cloud.

<Note>
  La messaggistica tra sessioni richiede Claude Code v2.1.224 o successivo su macOS e Linux, incluso Linux all'interno di WSL 2. Su Windows nativo, richiede Claude Code v2.1.234 o successivo. Quando una sessione soddisfa i requisiti, la messaggistica è attiva senza nulla da abilitare. Vedi [Disponibilità](#availability) per i requisiti del provider e come confermare che una sessione lo supporta.
</Note>

La messaggistica tra sessioni consente a Claude di consegnare un messaggio da una delle tue sessioni di Claude Code a un'altra. Quando una modifica in una sessione interrompe ciò su cui un'altra sta lavorando, Claude può avvertire quella sessione prima che tu te ne accorga. Quando una sessione risolve una domanda su cui un'altra è bloccata, Claude può inviare la risposta attraverso.

Un messaggio è un pezzo di testo che un Claude scrive a un altro, mai la cronologia della conversazione del mittente o i file. Per spostare un'intera conversazione o il suo contesto, [riprendi la sessione](/docs/it/sessions#resume-a-session) invece.

Claude utilizza due strumenti per questo: `ListAgents` per scoprire quali agenti può raggiungere, e `SendMessage` per consegnare un messaggio a uno di essi per nome. Con lo stesso strumento `SendMessage`, Claude può anche inviare messaggi a [subagenti](/docs/it/sub-agents#resume-subagents) e ai compagni di [team di agenti](/docs/it/agent-teams) all'interno di una singola sessione o team. Questa pagina copre i messaggi tra le tue sessioni indipendenti.

<h2 id="when-to-use-cross-session-messaging">
  Quando utilizzare la messaggistica tra sessioni
</h2>

Utilizza la messaggistica quando una delle tue sessioni ha qualcosa che un'altra sessione ha bisogno a metà compito. Claude può inviare un messaggio da solo quando vede la necessità, ad esempio dopo aver apportato una modifica che influisce sul lavoro che un'altra sessione sta svolgendo, oppure puoi chiedergli di inviarne uno. I casi comuni sono:

* **Consegna un risultato**: quando una sessione scopre un cambiamento che interrompe il funzionamento o prende una decisione, Claude lo riassume per la sessione che lavora sull'area interessata, invece di doverlo rispiegare lì.
* **Coordina worktree paralleli**: quando le sessioni lavorano lo stesso repository in [worktree](/docs/it/worktrees) separati, Claude può dire alle altre sessioni cosa è stato implementato.
* **Ottieni lo stato dal lavoro a lunga esecuzione**: fai in modo che una migrazione o un'esecuzione di test riferisca alla sessione che stai osservando, oppure chiedilo tu stesso da lì. Se quella sessione è su questa macchina, Claude può anche [chiederle un avviso quando successivamente diventa inattiva o esce](#get-a-notice-when-another-session-goes-idle).
* **Messaggi tra macchine**: raggiungi una delle tue sessioni su un'altra macchina o sul web.

Utilizza la messaggistica tra sessioni indipendenti che avvii e dirigi tu stesso. Claude Code ha una funzione dedicata per ognuno degli altri modi di eseguire o raggiungere più sessioni, quindi utilizza quella costruita per quello che stai facendo:

* Per continuare una conversazione in un altro terminale, o condividere il suo contesto con una nuova sessione, [riprendi la sessione](/docs/it/sessions#resume-a-session)
* Per un team coordinato di sessioni che Claude genera e supervisiona, utilizza [team di agenti](/docs/it/agent-teams)
* Per osservare e dirigere molte sessioni da un unico posto, utilizza [agent view](/docs/it/agent-view)
* Per dirigere una sessione tu stesso dal tuo telefono o da un altro dispositivo, piuttosto che far inviare messaggi alle sessioni l'una all'altra, utilizza [Remote Control](/docs/it/remote-control)
* Per spingere eventi esterni, come risultati CI o messaggi di chat, in una sessione, utilizza [channels](/docs/it/channels)

<h2 id="message-another-session">
  Inviare un messaggio a un'altra sessione
</h2>

Quando una delle tue sessioni scopre qualcosa che un'altra sessione ha bisogno di sapere, come un risultato, uno stato o una decisione, Claude la trasmette invece di farti copiare e incollare tra i terminali. Claude scopre il destinatario con `ListAgents` e invia con `SendMessage`, quindi non chiami mai nessuno dei due strumenti tu stesso. Claude può decidere di inviare un messaggio senza essere chiesto, e puoi anche richiederne uno.

Per richiederne uno tu stesso, dì a Claude cosa vuoi che l'altra sessione sappia o faccia. Questo esempio è un prompt che digiti, non un messaggio che Claude invia:

```text wrap theme={null}
Chiedi alla sessione in esecuzione nel mio altro terminale se la migrazione è terminata
```

Claude scrive il messaggio vero e proprio, quindi il tuo prompt può lasciare il contenuto a Claude. Questo prompt chiede un riepilogo senza dettarne la formulazione, e quello che Claude invia varia:

```text wrap theme={null}
Spiega quello che abbiamo appena fatto alla sessione che sta lavorando all'API dei pagamenti
```

Per nominare il destinatario tu stesso, menziona la sessione nel tuo prompt: digita `@` seguito dalle prime lettere del nome della sessione e scegli la sessione dal typeahead, nello stesso modo in cui [@-menzioni un subagent](/docs/it/sub-agents#invoke-subagents-explicitly). Richiede Claude Code v2.1.232 o successivo. Claude Code inserisce la menzione, come `@api-worker`, e dice a Claude quale sessione nomina, quindi Claude può inviare un messaggio a quella sessione senza elencare prima le tue sessioni. Questo prompt nomina il destinatario con una menzione:

```text wrap theme={null}
Fai sapere a @api-worker che la migrazione dello schema è terminata
```

Il typeahead elenca le tue altre sessioni live su questa macchina. Due casi richiedono più delle prime lettere di un nome:

* **Una sessione oltre questa macchina**: una sessione cloud o Remote Control appare nel typeahead solo dopo che Claude ha elencato o inviato messaggi alle tue sessioni oltre questa macchina, quindi chiedi a Claude di elencarle prima.
* **Un nome con uno spazio o altri caratteri al di fuori di lettere, cifre, trattini e sottolineature**: digitalo tra virgolette doppie, come `@"release notes"`. Quando scegli la sessione dal typeahead, Claude Code inserisce le virgolette per te.

Puoi anche digitare la menzione senza il picker. Quando più di una sessione live risponde al nome menzionato, Claude ti chiede quale intendi prima di inviare.

Per vedere come appare il messaggio che Claude scrive quando arriva, incluso un esempio, vedi [come appare un messaggio](#what-a-message-looks-like).

<h3 id="message-delivery">
  Consegna del messaggio
</h3>

Il Claude ricevente legge il messaggio tra le chiamate agli strumenti durante un turno attivo, quindi uno strumento in esecuzione non viene mai interrotto. Quando la sessione ricevente è inattiva, Claude Code avvia un nuovo turno con il messaggio.

Un messaggio da un'altra sessione arriva come testo semplice. Se menziona un file o una [risorsa MCP](/docs/it/mcp#use-mcp-resources) con `@`, Claude vede la menzione come scritta e Claude Code non allega nulla, indipendentemente dal fatto che il messaggio avvii un nuovo turno o arrivi durante uno. Claude può comunque aprire un percorso menzionato sulla macchina ricevente con i suoi strumenti, soggetto alle autorizzazioni di quella sessione. Prima della v2.1.251, una menzione `@` in un messaggio che ha avviato un nuovo turno allegava il file o la risorsa MCP sul lato ricevente.

Claude Code rifiuta un messaggio nei seguenti casi:

* Il messaggio è [oltre il limite di dimensione](#limitations). Claude Code lo rifiuta nella sessione di invio, prima che parta.
* Un rapido burst a una sessione su questa macchina ha raggiunto [quello che la posta in arrivo di quella sessione accetta](#limitations). Claude Code rifiuta ulteriori messaggi a quella sessione.
* Il destinatario della risposta su questa macchina non supera un controllo di sicurezza, come un destinatario con collegamento simbolico o un endpoint che non è il processo previsto. [Rifiuto di inviare un messaggio tra sessioni](/docs/it/errors#refusing-to-send-a-cross-session-message) elenca questi controlli.
* Claude indirizza il messaggio al nome della sessione stessa, come descritto in [Vedi quali sessioni Claude può raggiungere](#see-which-sessions-claude-can-reach).

La sessione ricevente controlla ogni messaggio in arrivo rispetto ai suoi [controlli in entrata](#control-inbound-messages), e il controllo termina in uno di tre risultati:

* **Consegnato**: Claude Code passa il messaggio al Claude ricevente.
* **Trattenuto**: Claude Code mette da parte il messaggio senza consegnarlo. Un messaggio trattenuto raggiunge Claude solo quando lo approvi o un cambio di modalità o impostazioni successivo lo consente.
* **Rifiutato**: Claude Code scarta il messaggio senza consegnarlo.

Una volta consegnato, il messaggio conta verso [l'utilizzo](/docs/it/costs) come un prompt che digiti, e il Claude ricevente può rispondere al mittente nello stesso modo, tranne nel [caso cross-machine unidirezionale](#message-sessions-on-other-machines).

I confini delle autorizzazioni rimangono per sessione. Claude è istruito a non chiedere mai a un'altra sessione un'azione che è stata negata o bloccata nella sua sessione, o che le sue stesse impostazioni di autorizzazione bloccherebbero, e a instradare quel lavoro di nuovo a te. Sul lato ricevente, i [prompt di autorizzazione della sessione ricevente e le regole si applicano ancora](#how-a-session-treats-an-incoming-message) a qualsiasi cosa il messaggio chieda.

<h3 id="get-a-notice-when-another-session-goes-idle">
  Ricevi una notifica quando un'altra sessione diventa inattiva
</h3>

Claude può chiedere a una delle tue sessioni su questa macchina di inviare indietro una notifica quando quella sessione successivamente diventa inattiva o esce. Inattivo qui significa che la sessione ha terminato un turno senza nulla in coda. Usalo quando stai aspettando un'attività lunga in un'altra sessione e vuoi sapere quando è finita invece di controllare. Richiede Claude Code v2.1.236 o successivo in entrambe le sessioni.

<h4 id="ask-for-a-notice">
  Richiedi una notifica
</h4>

Dì a Claude cosa stai aspettando. Questo prompt richiede una notifica dalla sessione di migrazione:

```text wrap theme={null}
Dimmi quando la sessione di migrazione finisce quello su cui sta lavorando
```

Claude si iscrive con l'input `notify_when_idle` dello strumento `SendMessage`, allegato a un messaggio che sta inviando comunque o da solo. Da solo, Claude Code si iscrive senza avviare un turno o spendere token nella sessione osservata, e invia la notifica subito se quella sessione è già inattiva. Allegato a un messaggio, Claude Code consegna prima il messaggio e invia la notifica dopo.

<h4 id="what-each-session-shows">
  Cosa mostra ogni sessione
</h4>

La sessione osservata mostra una riga che dice che un altro processo ha chiesto di essere avvisato quando la sessione successivamente diventa inattiva. La sessione che chiede mostra la notifica come una riga che nomina la sessione osservata. La riga può includere l'ora in cui il turno di quella sessione è terminato e uno stato di una riga da quel turno. Se la sessione che chiede è inattiva, Claude Code avvia un nuovo turno con la notifica.

<h4 id="limits">
  Limiti
</h4>

La notifica è una tantum: Claude Code la invia una volta dalla sessione osservata, e nessuna delle due sessioni esegue il polling dell'altra. Se nessuna notifica arriva entro 12 ore, Claude Code abbandona l'iscrizione e lo dice a Claude, quindi non continua ad aspettare.

I [controlli in entrata](#control-inbound-messages) di ogni lato si applicano a una notifica come a un messaggio:

* **`refuse` su uno dei due lati**: nulla arriva. La sessione osservata abbandona la richiesta senza registrarla o rispondere, quindi l'iscrizione scade senza risposta dopo 12 ore, e una sessione che chiede con `refuse` non si iscrive mai.
* **`hold` su uno dei due lati**: la notifica arriva con meno. La sessione osservata lascia fuori lo stato di una riga, e la sessione che chiede mostra la notifica nella tua trascrizione senza consegnarla a Claude.

Solo il Claude nella tua conversazione principale può iscriversi, e solo alle tue sessioni su questa macchina. Quando un subagent o un compagno di squadra di un team di agenti imposta `notify_when_idle`, Claude Code non effettua alcuna iscrizione e glielo dice. Quando Claude chiede una notifica da qualsiasi altro agente, come un compagno di squadra, un subagent o una sessione oltre questa macchina, Claude Code rifiuta l'intera chiamata, incluso qualsiasi messaggio allegato, e segnala il rifiuto a Claude in modo che possa rinviare il messaggio senza la richiesta.

<h3 id="see-which-sessions-claude-can-reach">
  Vedi quali sessioni Claude può raggiungere
</h3>

Claude trova il destinatario di un messaggio da solo, quindi non hai bisogno di eseguire nulla prima di chiedergli di inviare. Per vedere tu stesso quali sessioni Claude può raggiungere, esegui il comando `/list-agents`. La prima riga, quando presente, è il nome della sessione stessa, quello che le tue altre sessioni usano per inviarle messaggi. Le righe sottostanti sono le sessioni che Claude può raggiungere:

* **Subagent**: agenti in esecuzione all'interno della sessione corrente.
* **Compagni di squadra**: i compagni di squadra del [team di agenti](/docs/it/agent-teams) della sessione stessa. Prima della v2.1.239, i compagni di squadra non apparivano nell'elenco, anche se Claude poteva già inviar loro messaggi per nome.
* **Le tue altre sessioni locali**: sessioni Claude Code in esecuzione sulla stessa macchina, incluse [sessioni in background](/docs/it/agent-view). Una sessione appare solo quando associa un [socket della posta in arrivo](#the-sessions-inbox-socket).
* **Le tue [sessioni cloud](/docs/it/claude-code-on-the-web)**: mostrate mentre questa sessione è connessa a [Remote Control](/docs/it/remote-control). Claude Code le etichetta `cloud` nell'elenco.
* **Le tue sessioni Remote Control su altre macchine**: mostrate mentre questa sessione è connessa a [Remote Control](/docs/it/remote-control), ed etichettate `Remote Control`. Claude Code mostra `offline` come lo stato di una sessione la cui connessione Remote Control è caduta.

Questa sessione non è una delle righe. Se Claude indirizza un messaggio al nome della sessione stessa, Claude Code lo rifiuta e dice a Claude che il destinatario è la sessione corrente. Prima della v2.1.239, l'elenco non mostrava il nome di questa sessione, e Claude Code segnalava un messaggio inviato a essa come un agente che non poteva trovare.

Mentre questa sessione è connessa a [Remote Control](/docs/it/remote-control), Claude Code trattiene alcuni dettagli delle tue sessioni locali dall'output `/list-agents`, senza cambiare quello che Claude stesso vede quando cerca una sessione a cui inviare un messaggio:

* **Directory di lavoro**: lascia fuori la directory di lavoro di ogni sessione locale.
* **Nomi delle sessioni**: lascia fuori qualsiasi nome di sessione che non può attribuire a una persona, quindi una riga rimasta senza nome legge `(unnamed session)`.
* **La prima riga**: lascia fuori la riga con il nome della sessione stessa a meno che tu non abbia digitato quel nome a questo terminale, con `--name` o con `/rename` e il nome, da quando hai lanciato o ripreso l'ultima volta la sessione.

Quando l'output elenca qualcosa, termina con una nota che dice che i dettagli sono stati trattenuti. Eseguire `/rename` seguito da un nome inutilizzato al terminale della sessione stessa dà a quella sessione un nome che appare nell'output.

Claude Code legge i tuoi elenchi di sessioni cloud e Remote Control dal più recente al più vecchio e si ferma dopo un numero limitato di pagine per ciascuno. Se il tuo account ha più di quelle sessioni di quante ne stiano, Claude Code non elenca le più vecchie, e Claude non può inviar loro messaggi per nome. Quando ciò accade, Claude Code lo dice nell'elenco, e Claude vede la stessa nota quando invia un messaggio.

Claude indirizza una sessione oltre questa macchina per nome, nello stesso modo di una sessione locale. Vedi [Inviare messaggi a sessioni su altre macchine](#message-sessions-on-other-machines) per come quei messaggi viaggiano.

Una sessione risponde al nome che imposti con il comando [`/rename`](/docs/it/commands) o il flag [`--name`](/docs/it/cli-reference#cli-flags). Quando non ne imposti uno, Claude Code nomina la sessione stessa. Per una sessione interattiva, questo è il nome mostrato negli [elenchi di sessioni in esecuzione](/docs/it/sessions#name-your-sessions).

Quando rinomini una sessione, Claude Code aggiorna anche il record condiviso che le tue altre sessioni usano per cercare il nome della sessione. Se non riesce ad aggiornare quel record, ti avverte nell'output `/rename` che altre sessioni potrebbero ancora mostrare il nome vecchio. Esegui la sessione con [`--debug`](/docs/it/cli-reference#cli-flags), e Claude Code registra la causa dell'aggiornamento fallito.

Quando rinomini una sessione, o avvii o riprendi una sessione interattiva, con un nome che un'altra sessione live su questa macchina già usa, Claude Code lascia il nome con la sessione che lo ha già e [rinomina il tuo a una variante](/docs/it/sessions#name-your-sessions). Le sessioni possono comunque condividere un nome, ad esempio quando una di loro esegue una versione precedente di Claude Code o il nome condiviso è uno che Claude Code ha generato. A meno che questa sessione non sia connessa a Remote Control, Claude Code mostra la directory di lavoro di ogni sessione locale nell'output `/list-agents`, quindi puoi distinguere le sessioni con lo stesso nome quando vengono eseguite in directory diverse. Claude indirizza il messaggio in uno di due modi, a seconda di quante sessioni live rispondono al nome:

* **Una sessione risponde al nome**: Claude Code consegna il messaggio solo sul nome.
* **Diverse sessioni condividono il nome, o Claude Code non poteva controllare ovunque vengono eseguite le tue sessioni**: Claude aggiunge un breve identificatore a ogni riga del suo elenco e usa l'identificatore nell'indirizzo.

<h3 id="message-sessions-on-other-machines">
  Inviare messaggi a sessioni su altre macchine
</h3>

Come un messaggio viaggia, e se passa attraverso i server Anthropic, dipende da dove viene eseguita la sessione di destinazione:

| Dove viene eseguita l'altra sessione    | Come il messaggio viaggia                                                                                                      |
| :-------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| Su questa macchina                      | Su un socket per sessione su macOS e Linux, o una named pipe per sessione su Windows nativo, mai attraverso i server Anthropic |
| Su un'altra delle tue macchine          | Attraverso i server Anthropic, arrivando sulla connessione [Remote Control](/docs/it/remote-control) di quella macchina             |
| Nel [cloud](/docs/it/claude-code-on-the-web) | Attraverso i server Anthropic, direttamente alla sessione cloud                                                                |

Avviare una conversazione con una sessione su un'altra delle tue macchine richiede Claude Code v2.1.225 o successivo e un destinatario che [appare nell'elenco](#see-which-sessions-claude-can-reach). Prima della v2.1.225, Claude poteva solo rispondere a un messaggio che arrivava da uno.

Puoi inviare un messaggio a una sessione mostrata come `offline` nell'[elenco](#see-which-sessions-claude-can-reach), una la cui connessione Remote Control è caduta. L'invio va a buon fine, ma il messaggio arriva solo dopo che la macchina di quella sessione si riconnette. Claude viene informato di questo quando invia.

La consegna sulla stessa macchina funziona ovunque la funzione sia abilitata. Ogni sessione si registra in file su disco. Quando Claude elenca o invia messaggi alle tue sessioni locali, Claude Code legge quei file per trovare le sessioni, quindi due sessioni possono raggiungersi solo quando possono vedere gli stessi file.

Un contenitore ha il suo proprio filesystem, quindi una sessione all'interno e una sessione sull'host non possono raggiungersi. Due sessioni all'interno dello stesso contenitore possono comunque inviarsi messaggi, incluso su un [runner self-hosted](/docs/it/self-hosted-environments). Una sessione all'interno di WSL 2 e una sessione Windows nativa sullo stesso computer non possono raggiungersi neanche, perché si registrano sotto directory home diverse e ascoltano su tipi di socket diversi.

Mentre questa sessione è connessa a Remote Control, quando invii un messaggio a una sessione su un'altra delle tue macchine, Claude Code mostra il messaggio nella conversazione di quella sessione sotto il nome Remote Control di questa sessione. Il Claude su quella macchina può rispondere a quel nome. Ad esempio, quando questa sessione è connessa a Remote Control come `laptop-graceful-unicorn` e invii un messaggio al tuo desktop, vedi il messaggio nella sessione del desktop sotto `laptop-graceful-unicorn`.

Se questa sessione non è connessa a Remote Control quando Claude invia a una sessione oltre questa macchina, il messaggio comunque va a buon fine, ma senza un [indirizzo di risposta](#what-a-message-looks-like), quindi il Claude ricevente non può rispondere. Claude viene informato di questo quando invia.

Per richiedere la tua approvazione prima che qualsiasi messaggio vada oltre questa macchina, imposta [`isolatePeerMachines`](#require-approval-for-cross-machine-messages).

<h2 id="how-a-session-treats-an-incoming-message">
  Come una sessione tratta un messaggio in arrivo
</h2>

Quando la sessione A invia un messaggio alla sessione B, Claude Code dice al Claude di B che il messaggio è venuto da un'altra sessione, non da te, e limita quello che il messaggio può fare:

* **Non può approvare nulla**: un messaggio da un'altra sessione non conta mai come il tuo consenso, quindi non può rispondere a un prompt di autorizzazione in sospeso per tuo conto.
* **Non può cambiare la configurazione**: Claude Code istruisce il Claude ricevente a non cambiare mai le impostazioni di autorizzazione, `CLAUDE.md` o altre configurazioni perché un'altra sessione lo ha chiesto.
* **I comandi non vengono eseguiti**: un comando nel testo del messaggio, come `/compact`, arriva come testo semplice. Claude Code non lo esegue mai.
* **I prompt di autorizzazione si attivano comunque**: se agire sul messaggio richiede un'autorizzazione che la sessione ricevente non ha, vedi lo stesso prompt che vedresti per qualsiasi altro lavoro.

<h3 id="what-a-message-looks-like">
  Come appare un messaggio
</h3>

Quando un messaggio arriva, Claude Code lo mostra nella conversazione come un'anteprima di una riga attenuata, e la riga di anteprima rimane nella conversazione dopo. L'anteprima porta il nome del mittente e la prima riga del messaggio, tagliata con `…` quando è lunga, come `› Message from @api-worker: Schema migration finished (ctrl+o to expand)`. Prima della v2.1.247, Claude Code mostrava il messaggio in arrivo per intero invece di un'anteprima.

Uno di questi mostra il testo completo:

* Premi `Ctrl+O` per aprire il [visualizzatore di trascrizioni](/docs/it/interactive-mode#transcript-viewer) e leggi il testo completo sotto il nome della sessione del mittente.
* In una sessione avviata con [`--verbose`](/docs/it/cli-reference#cli-flags), Claude Code mostra il testo completo invece dell'anteprima.

L'anteprima accorcia solo quello che vedi. Che tu lo espanda o no, Claude legge il messaggio completo.

Claude riceve il messaggio con il nome del mittente e un indirizzo di risposta, tranne per un [messaggio cross-machine unidirezionale](#message-sessions-on-other-machines), che non porta alcun indirizzo di risposta. Oltre al nome e all'indirizzo di risposta, il Claude ricevente ottiene il testo del messaggio, mai la cronologia della conversazione o i file del mittente. [Consegna del messaggio](#message-delivery) copre le menzioni `@` nel testo.

Un messaggio che un [subagente](/docs/it/sub-agents) ha scritto arriva sotto il nome della sessione di invio, con il subagente identificato nel testo del messaggio. Una risposta ad esso raggiunge la conversazione principale di quella sessione, non il subagente.

Questo esempio è un messaggio che un Claude ha scritto a un altro, come il suo testo completo legge quando lo espandi:

```text wrap theme={null}
Schema migration finished
The new column is tenant_id, and rebasing on main is safe now.
```

<h3 id="control-inbound-messages">
  Controlla i messaggi in entrata
</h3>

Imposta [`crossSessionInbound`](/docs/it/settings-reference#crosssessioninbound) per scegliere cosa una sessione fa con i messaggi in arrivo dalle tue altre sessioni:

| Valore   | Comportamento                                                                                                                                                                                                                           |
| :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `accept` | Claude Code consegna ogni messaggio a Claude                                                                                                                                                                                            |
| `hold`   | Claude Code mostra un avviso per ogni messaggio e non lo consegna. Se un `accept` successivamente si applica, secondo le [regole di precedenza](/docs/it/settings-reference#crosssessioninbound), Claude Code rilascia i messaggi trattenuti |
| `refuse` | Claude Code elimina ogni messaggio senza consegnarlo                                                                                                                                                                                    |

Oltre a modificare un file di impostazioni, puoi selezionare il valore nella riga `/config` **Messages from your other sessions**. Claude Code scrive il valore che selezioni nelle tue impostazioni utente. La riga richiede Claude Code v2.1.232 o successiva e non appare mentre le impostazioni gestite o il flag `--settings` imposta la chiave, poiché un valore di impostazioni utente non si applicherebbe allora. Claude Code rifiuta la scorciatoia `/config crossSessionInbound=value` per questa chiave.

Per vedere quale valore si applica, segui le regole di precedenza `crossSessionInbound` nel [riferimento delle impostazioni](/docs/it/settings-reference#crosssessioninbound).

Quando nessun valore si applica, Claude Code decide per messaggio dalle modalità di autorizzazione delle due sessioni. Raggruppa le sessioni che [bypassano i prompt di autorizzazione](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode) in una classe, e ogni altra sessione nell'altra. La modalità Plan conta come bypassare nelle sessioni con autorizzazioni di bypass disponibili, e [auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), `acceptEdits` e `dontAsk` contano come prompt:

* **La sessione ricevente richiede autorizzazioni**: Claude Code consegna ogni messaggio. Trattiene uno per la tua approvazione solo quando la sessione di invio si identifica come bypassando i prompt di autorizzazione.
* **La sessione ricevente bypassa i prompt di autorizzazione**: Claude Code trattiene ogni messaggio per la tua approvazione. Consegna uno solo quando la sessione di invio si identifica come bypassando anche.

Quando il default trattiene un messaggio, Claude Code apre una finestra di dialogo di approvazione nella sessione ricevente. La finestra di dialogo mostra il mittente e un'anteprima:

* **Approve** consegna quel messaggio a Claude.
* **Deny**, o chiudere la finestra di dialogo, lo elimina.
* Quando la finestra di dialogo rimane senza risposta oltre la scadenza [`dialogExpiry`](/docs/it/settings-reference#dialogexpiry), Claude Code la chiude e elimina il messaggio. La scadenza predefinita è cinque minuti.
* Mentre nessun terminale è collegato a una [sessione in background](/docs/it/agent-view), Claude Code lascia la finestra di dialogo aperta oltre la scadenza. Dopo che colleghi, se la finestra di dialogo rimane senza risposta per un intero periodo di scadenza, Claude Code la chiude e elimina il messaggio.
* Se la classe di modalità di autorizzazione di questa sessione cambia mentre i messaggi sono trattenuti, Claude Code riapplica le regole in entrata, consegna i messaggi che ora accetta, e mostra un avviso.
* Se un cambio di impostazioni fa applicare `refuse` mentre i messaggi sono trattenuti, Claude Code elimina ogni messaggio trattenuto e segnala un rifiuto a ogni mittente che può raggiungere.

Quando il mittente è una sessione sulla stessa macchina, Claude Code invia un avviso indietro ad essa quando il ricevente trattiene il messaggio, e un follow-up quando il ricevente successivamente consegna, nega o lo fa scadere. L'avviso raggiunge il Claude di invio, quindi sa di non continuare ad aspettare un messaggio che l'altra sessione non ha letto.

In una sessione di invio interattiva, l'avviso appare nella trascrizione. Un mittente [`claude -p`](/docs/it/headless) riceve in [output in streaming](/docs/it/headless#stream-responses) come un [messaggio `system` informativo](/docs/it/agent-sdk/typescript#sdkinformationalmessage). Gli avvisi ai mittenti `claude -p` richiedono Claude Code v2.1.271 o successiva.

Se il ricevente rifiuta il messaggio, l'avviso del mittente dice che il ricevente non sta accettando messaggi tra sessioni e dice al Claude del mittente di non aspettare o rinviare.

Claude Code trattiene al massimo 100 messaggi, separatamente dalla coda di consegna, e oltre quello elimina i più vecchi.

<h3 id="non-interactive-sessions">
  Sessioni non interattive
</h3>

Claude Code associa un socket della posta in arrivo per una sessione [`claude -p`](/docs/it/headless) come una interattiva, quindi un worker `-p` a lunga esecuzione può ricevere messaggi e appare nell'elenco. Quando avvii una sessione in [modalità bare](/docs/it/headless#start-faster-with-bare-mode), Claude Code non associa il socket, quindi quella sessione non può ricevere messaggi e non appare nell'elenco degli agenti.

Una sessione `-p` non può mostrare la finestra di dialogo di approvazione. Quando il [default in entrata](#control-inbound-messages) trattiene un messaggio lì, Claude Code lo mantiene per la stessa scadenza [`dialogExpiry`](/docs/it/settings-reference#dialogexpiry) che la finestra di dialogo usa, cinque minuti per impostazione predefinita:

* **Prima della scadenza**: se una modalità o un cambio di impostazioni consente il messaggio, Claude Code lo consegna.
* **Oltre la scadenza**: Claude Code elimina il messaggio e lo segnala come scaduto a un mittente che può raggiungere.

Imposta `dialogExpiry` su `"never"` per mantenere i messaggi trattenuti dal default fino alla fine della sessione. Un messaggio trattenuto da un'impostazione `hold` esplicita non scade; Claude Code lo consegna solo quando un `accept` successivamente si applica.

Quando la sessione termina con messaggi ancora trattenuti, Claude Code li segnala come scaduti a ogni mittente che può raggiungere. Prima della v2.1.225, nessuna scadenza si applicava in una sessione `-p`: un messaggio trattenuto rimaneva trattenuto a meno che un cambio di modalità di autorizzazione durante l'esecuzione lo consegnasse, e una sessione che terminava con messaggi trattenuti non segnalava nulla ai loro mittenti.

Per far sì che un worker `-p` accetti messaggi incustodito, avvialo con `crossSessionInbound` impostato su `accept` nel suo valore `--settings`. Un `accept` nelle tue impostazioni utente funziona anche ma si applica a ogni sessione che esegui.

<h3 id="the-sessions-inbox-socket">
  Il socket della posta in arrivo della sessione
</h3>

Leggi questa sezione quando una sessione che ti aspetti non è nell'elenco degli agenti, quando vuoi che uno script o un hook pubblichi in una sessione, o quando un comando in sandbox non può raggiungere il socket.

Claude Code associa un socket della posta in arrivo per ogni sessione con la messaggistica tra sessioni abilitata, dove altre sessioni sulla macchina consegnano messaggi. Il socket è un socket di dominio Unix su macOS e Linux, incluso Linux all'interno di WSL 2, e una named pipe su Windows nativo. Per quali tipi di sessione associano uno, vedi [Sessioni non interattive](#non-interactive-sessions).

Puoi trovare il percorso del socket in due posti:

* `/status` lo mostra nella riga `Peer address`. Il percorso è prefissato con `uds:`.
* Claude Code lo esporta a [hooks](/docs/it/hooks) e comandi Bash come la variabile di ambiente [`CLAUDE_CODE_MESSAGING_SOCKET`](/docs/it/env-vars#variables):
  * In una sessione che inizia con la messaggistica attiva, Claude Code esporta la variabile prima che qualsiasi hook venga eseguito, incluso `SessionStart`.
  * Ogni sessione esporta il suo socket, mai uno ereditato da una sessione genitore.

Su macOS e Linux, Claude Code limita il socket al tuo utente del sistema operativo. Su Windows nativo, richiede invece a ogni connessione di autenticarsi prima con una chiave che solo il tuo utente del sistema operativo può leggere. In entrambi i casi, su una macchina condivisa le sessioni di un altro utente non possono consegnare ad esso.

Su macOS e Linux, Claude Code rifiuta anche di creare il socket in una directory che non può accettare, ad esempio una di proprietà di un altro utente, e utilizza invece una directory privata per utente, `/tmp/cc-socks-<uid>`. Quando non può accettare alcuna directory, la sessione viene eseguita senza una posta in arrivo: Claude Code mostra un avviso, `/status` mostra `unavailable` e il motivo nella sua riga `Peer address`, e il log [`--debug`](/docs/it/cli-reference#cli-flags) registra il rifiuto completo.

Insieme al percorso del socket, Claude Code esporta un token per sessione come [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/it/env-vars#variables). Uno script che pubblica nel socket della sua stessa sessione può inviare `{"type":"auth","token":"<token>"}` come la prima riga della sua connessione, dove `<token>` è il valore di `CLAUDE_CODE_MESSAGING_TOKEN`. Se Claude Code richiede la riga dipende dalla piattaforma:

* **macOS e Linux, incluso WSL 2**: la riga è facoltativa. Claude Code accetta una connessione con o senza di essa.
* **Windows nativo**: la riga è obbligatoria. Claude Code chiude qualsiasi connessione la cui prima riga non è una riga di autenticazione valida e non consegna nulla da quella connessione.

Apri la connessione solo quando il messaggio che stai pubblicando è pronto. Claude Code chiude una connessione che non ha inviato una riga completa entro 30 secondi, quindi cattura prima l'output di un comando lento e poi apri la connessione per inviarlo.

I [messaggi propri-figli](#own-child-messages) sottostanti dicono quando Claude Code consulta il token e come tratta un messaggio che non può verificare.

<span id="own-child-messages" />Claude Code esegue i messaggi in arrivo sul socket attraverso gli stessi [controlli in entrata](#control-inbound-messages) di qualsiasi altro messaggio peer, con un'eccezione e un prerequisito:

* **Messaggi propri-figli**: quando nessun valore `crossSessionInbound` si applica, Claude Code consegna un messaggio che verifica è venuto dai processi figli della sessione stessa, come un hook o un comando Bash che pubblica di nuovo nel socket della posta in arrivo della sua stessa sessione.
  * Su Linux, incluso all'interno di WSL 2, Claude Code può verificare per evidenza di processo anche per un figlio che è già uscito. Su macOS può verificare in quel modo solo mentre il processo di pubblicazione è ancora in esecuzione, e in un contenitore dove Claude Code viene eseguito come ID processo 1 non ha alcuna evidenza di processo. Su Windows nativo non ne ha nemmeno.
  * Su macOS dopo che il processo di pubblicazione è uscito e in contenitori dove Claude Code viene eseguito come ID processo 1, quella evidenza di processo manca, e Claude Code verifica invece un figlio che ha inviato il [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/it/env-vars#variables) esportato della sessione nella riga di autenticazione che ha aperto la sua connessione. Su Windows nativo, quel token è l'unico modo in cui Claude Code verifica un messaggio proprio-figlio.
  * Quando Claude Code non può verificare in nessun modo, tratta il messaggio come qualsiasi altro che non asserisce alcuna classe di autorizzazione, quindi una sessione che bypassa i prompt di autorizzazione lo trattiene per la tua approvazione.
* **Sessioni in sandbox**: controlla se un comando Bash può raggiungere il socket dall'interno della [sandbox](/docs/it/sandboxing) con le impostazioni del socket Unix della sandbox, [`sandbox.network.allowAllUnixSockets` e `sandbox.network.allowUnixSockets`](/docs/it/settings-reference#sandbox-settings).

<h2 id="restrict-cross-session-messaging">
  Limitare la messaggistica tra sessioni
</h2>

Oltre alle impostazioni predefinite per messaggio, è possibile limitare la messaggistica in due modi. Richiedere l'approvazione prima che qualsiasi messaggio lasci la macchina, oppure disattivare la messaggistica per una sessione o un'organizzazione.

<h3 id="require-approval-for-cross-machine-messages">
  Richiedere approvazione per i messaggi tra macchine
</h3>

Impostare [`isolatePeerMachines`](/docs/it/settings-reference#isolatepeermachines) su `true` per richiedere l'approvazione esplicita prima che qualsiasi `SendMessage` raggiunga una sessione al di là di questa macchina:

```json theme={null}
{
  "isolatePeerMachines": true
}
```

Con questa impostazione, Claude Code richiede l'approvazione prima che il messaggio di Claude a una sessione al di là di questa macchina venga inviato, anche in modalità `bypassPermissions`, che ignora i normali prompt di autorizzazione. Un valore `true` da qualsiasi ambito di impostazioni si applica, quindi un file di progetto archiviato può attivare il requisito ma non disattivarlo. Claude Code non richiede l'approvazione per i messaggi tra sessioni sulla stessa macchina.

<h3 id="turn-off-cross-session-messaging">
  Disattivare la messaggistica tra sessioni
</h3>

La ricezione e l'invio sono controlli separati, quindi disattivare la direzione di cui hai bisogno, o entrambe. Utilizzare `crossSessionInbound` per i messaggi in arrivo e le regole di autorizzazione per ciò che Claude qui può inviare o elencare:

* **Interrompere la ricezione**: impostare `crossSessionInbound` su `refuse`, e Claude Code elimina i messaggi peer in arrivo senza consegnarli. Dalle impostazioni di progetto o locali, `refuse` si applica su ogni altra fonte, e dalle impostazioni utente si applica a meno che le impostazioni gestite o il flag `--settings` impostino un valore.
* **Interrompere l'invio e l'elenco**: aggiungere [regole di negazione delle autorizzazioni](/docs/it/permissions#tool-specific-permission-rules) che denominano `SendMessage` e `ListAgents`. Entrambi accettano il nome dello strumento senza specificatore.

Gli amministratori possono disattivare entrambi i lati per un'organizzazione nelle [impostazioni gestite](/docs/it/managed-settings), combinando le regole di negazione con `refuse`:

```json theme={null}
{
  "permissions": {
    "deny": ["SendMessage", "ListAgents"]
  },
  "crossSessionInbound": "refuse"
}
```

Con questa impostazione in vigore, Claude Code associa comunque il socket della posta in arrivo di ogni sessione, ma elimina ogni messaggio che arriva su di esso senza consegnare nulla a Claude. Negare `SendMessage` rimuove anche la messaggistica ai subagent e ai compagni di squadra dell'agente, poiché lo stesso strumento serve entrambi. Una sessione che rifiuta non mostra alcun cambiamento visibile, nel suo `/status` o negli elenchi di altre sessioni sulla stessa macchina, quindi per confermarlo, controllare i file di impostazioni che si applicano a quella sessione piuttosto che il suo stato.

<h2 id="availability">
  Disponibilità
</h2>

La messaggistica tra sessioni richiede Claude Code v2.1.224 o successiva su macOS, Linux e WSL 2, e v2.1.234 o successiva su Windows nativo. La disponibilità, e quali sessioni Claude può inviare messaggi, dipendono anche dal tuo sistema operativo, provider e configurazione:

* **Sistema operativo**: disponibile su macOS, Windows e Linux, incluso Linux all'interno di WSL 2.

* **Sessioni su questa macchina**: disponibile su ogni provider, inclusi Amazon Bedrock, Claude Platform su AWS, Agent Platform di Google Cloud e Microsoft Foundry, e in sessioni che vengono eseguite con [fetching dei flag di funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching) disattivato. Su quei provider, e con il fetching dei flag disattivato, la messaggistica sulla stessa macchina richiede Claude Code v2.1.248 o successiva. Claude Code consegna questi messaggi su un [socket per sessione sulla tua macchina](#the-sessions-inbox-socket), mai attraverso i server Anthropic.

  Per impedire a una sessione di riceverli, imposta [`crossSessionInbound`](#turn-off-cross-session-messaging) su `refuse`.

* **Sessioni oltre questa macchina**: Claude trova le tue sessioni [Claude Code sul web](/docs/it/claude-code-on-the-web) e le tue sessioni su altre macchine da una sessione che è connessa a Remote Control, che ha bisogno di un accesso claude.ai come autenticazione attiva di questa sessione e degli altri [requisiti di Remote Control](/docs/it/remote-control#requirements). Claude non può trovare quelle sessioni con una chiave API o su Amazon Bedrock, Claude Platform su AWS, Agent Platform di Google Cloud e Microsoft Foundry.

Per controllare una sessione, digita `/list-agents`, disponibile anche come `/peers`. Il risultato separa una sessione che non ha la funzione da una sessione dove qualcosa di più stretto ha bloccato un messaggio, come uno strumento `SendMessage` mancante o un invio rifiutato:

* **`/list-agents` non è riconosciuto**: la sessione non ha messaggistica tra sessioni. Lavora attraverso i requisiti sopra, iniziando con `claude --version` per il requisito di versione.
* **`/list-agents` funziona ma un invio non è arrivato**: la messaggistica è attiva, e qualcosa di più stretto si applica:
  * **Regole di negazione**: una [regola di autorizzazione di negazione](#turn-off-cross-session-messaging) rimuove gli strumenti `SendMessage` e `ListAgents`.
  * **Controlli in entrata**: i [controlli in entrata della sessione ricevente](#control-inbound-messages) possono trattenere o eliminare quello che invii.
  * **Sessione cloud mancante**: una sessione cloud appare solo mentre questa sessione è connessa a [Remote Control](/docs/it/remote-control).
  * **Sessione su un'altra macchina mancante**: una sessione su un'altra delle tue macchine appare solo quando viene eseguita con [Remote Control](/docs/it/remote-control) e questa sessione è connessa anche.
  * **Sessione su un'altra macchina `offline`**: un messaggio a una sessione elencata come `offline` va a buon fine, ma [arriva solo dopo che la macchina di quella sessione si riconnette](#message-sessions-on-other-machines).
  * **Sessione cloud o su un'altra macchina più vecchia mancante**: Claude Code [legge quegli elenchi di sessioni dal più recente al più vecchio e si ferma dopo un numero limitato di pagine](#see-which-sessions-claude-can-reach), quindi Claude non può inviare un messaggio a una sessione che è caduta oltre di loro per nome.
  * **Avvio di una conversazione**: [Invia messaggi a sessioni su altre macchine](#message-sessions-on-other-machines) copre l'avvio di una conversazione con una sessione oltre questa macchina.

In una sessione con messaggistica, `/status` mostra anche una riga `Peer address` con l'indirizzo della posta in arrivo della sessione stessa, o `unavailable` e il motivo quando Claude Code [non poteva impostare una posta in arrivo](#the-sessions-inbox-socket).

<h2 id="limitations">
  Limitazioni
</h2>

I limiti qui sono proprietà del canale di messaggistica stesso e si applicano ovunque la funzione viene eseguita. Per i gap di piattaforma e provider, vedi [Disponibilità](#availability) invece.

* **Solo testo semplice**: Claude invia solo testo semplice tra sessioni. I messaggi del protocollo [team di agenti](/docs/it/agent-teams) strutturati rimangono all'interno di un team.
* **La dimensione del messaggio sulla stessa macchina è limitata**: Claude Code rifiuta un messaggio a una sessione su questa macchina una volta che la sua forma serializzata supera circa un milione di caratteri. Il rifiuto [nomina le dimensioni esatte](/docs/it/errors#message-too-large-for-cross-session-delivery). Nulla raggiunge la sessione ricevente.
* **I burst rapidi a una sessione vengono rifiutati al mittente**: una volta che un burst rapido di messaggi a una sessione su questa macchina raggiunge quello che la posta in arrivo di quella sessione accetta, Claude Code rifiuta ulteriori invii nella sessione di invio. Il [rifiuto nomina il burst](/docs/it/errors#too-many-messages-to-this-session-just-now) e dice a Claude di raggruppare il resto in un messaggio o aspettare. Prima della v2.1.236, Claude Code segnalava quegli invii come inviati mentre la sessione ricevente li eliminava.
* **I loop di messaggi sono limitati**: nella sessione ricevente, Claude Code limita la velocità dei messaggi ripetuti per mittente, elimina i ripetuti identici che arrivano entro una breve finestra, e mette in coda al massimo 50 messaggi accettati per Claude da leggere. Un loop di messaggi tra due sessioni quindi si ferma da solo. Quando il limite di velocità, il controllo di ripetizione o il limite di coda elimina un messaggio da una sessione interattiva su questa macchina, Claude Code dice a quella sessione quale lo ha eliminato e dice al suo Claude di non rinviare subito.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Subagenti](/docs/it/sub-agents#resume-subagents) e [team di agenti](/docs/it/agent-teams#messages-between-agents): messaggistica all'interno di una singola sessione o team
* [Agenti in background](/docs/it/agent-view): invia e monitora le sessioni parallele che potresti inviare messaggi
* [Remote Control](/docs/it/remote-control): connetti questa sessione per raggiungere le tue sessioni su altre macchine
* [Impostazioni](/docs/it/settings-reference#all-settings): `crossSessionInbound`, `isolatePeerMachines` e `dialogExpiry`
* [Modalità di autorizzazione](/docs/it/permission-modes): le modalità dietro il default in entrata delle due classi
* [Riferimento degli strumenti](/docs/it/tools-reference): le righe `ListAgents` e `SendMessage` nella tabella degli strumenti
* [Esegui agenti in parallelo](/docs/it/agents): confronta i modi in cui Claude Code esegue più agenti
