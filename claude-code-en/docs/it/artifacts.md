> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Condividi l'output della sessione come artifact

> Gli artifact trasformano il lavoro di Claude Code in pagine live e interattive su claude.ai che puoi mantenere private, condividere con la tua organizzazione o pubblicare con un link pubblico.

<Note>
  Gli artifact sono disponibili sui piani Pro, Max, Team ed Enterprise e richiedono una sessione autenticata con [`/login`](/docs/it/setup#authenticate). Consulta [Disponibilità](#availability) per l'insieme completo dei requisiti.
</Note>

Un [artifact](https://claude.com/features/artifacts) è una pagina web live e interattiva che Claude Code pubblica dalla tua sessione a un URL privato su claude.ai. La apri in un browser e si aggiorna sul posto mentre la sessione continua. Condividila dall'intestazione della pagina quando vuoi che qualcun altro la veda.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Un artifact aperto in un browser su claude.ai/code/artifact. L'intestazione del visualizzatore mostra il titolo dell'artifact acme-funnel-fix, un pulsante Condividi e l'avatar dell'autore. Il menu Condividi è aperto con l'interruttore Condividi sempre la versione più recente, un selettore di versione che legge Condivisione versione 2, un selettore di pubblico Everyone at Acme e un pulsante Copia link. Sotto l'intestazione, la pagina dell'artifact mostra due mockup mobili affiancati, un grafico a imbuto e una riga di schede metriche." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Quando utilizzare un artifact
</h2>

Utilizza un artifact quando il testo del terminale non è il mezzo appropriato per ciò che Claude ha prodotto: output che è più facile da visualizzare e con cui interagire rispetto a leggere riga per riga. Claude costruisce la pagina da qualsiasi cosa la tua sessione possa raggiungere, incluso il tuo codebase e i dati che estrae attraverso i tuoi [strumenti connessi](/docs/it/mcp), quindi la pagina può mostrare cose che richiederebbero paragrafi per descrivere. Ad esempio, chiedi a Claude di:

* Guidare un revisore attraverso una pull request con diff annotati
* Renderizzare una dashboard dai dati che la sessione ha già estratto
* Disporre diverse opzioni di design o implementazione affiancate
* Mantenere una timeline di investigazione che si riempie mentre un'attività lunga è in esecuzione
* Inviare a un collega un link invece di incollare l'output in Slack
* Pubblicare una bacheca di stato che [estrae dati freschi attraverso connettori MCP](#pull-live-data-with-mcp-connectors) ogni volta che qualcuno la apre

Vedi [Cosa puoi costruire](#what-you-can-build) per i prompt che corrispondono a questi, e [Estrai dati live con connettori MCP](#pull-live-data-with-mcp-connectors) per il prompt della bacheca supportata da connettore.

<h3 id="what-an-artifact-is-not">
  Cosa un artifact non è
</h3>

Un artifact è una cattura del lavoro: una singola pagina autonoma senza backend, quindi non può servire più route. Per uno strumento interno ospitato con un backend, distribuiscilo sulla tua infrastruttura. Vedi [Vincoli della pagina](#page-constraints) per l'insieme completo dei limiti.

<h2 id="create-an-artifact">
  Creare un artifact
</h2>

Claude può pubblicare un artifact autonomamente quando l'output è adatto a una pagina, oppure puoi richiederne uno direttamente. Per richiederne uno, nomina la funzionalità o descrivi l'output visivo che desideri in linguaggio naturale. Un buon candidato è qualsiasi cosa più facile da vedere che da leggere come testo, come un diff annotato, un grafico o un insieme di opzioni da confrontare. I prompt seguenti sono due esempi; consulta [Cosa puoi creare](#what-you-can-build) per altri modelli.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

A meno che non specifichi una posizione, Claude scrive la pagina in un file HTML o Markdown in una directory temporanea al di fuori del tuo progetto, quindi la pubblica. La pubblicazione di un nuovo artifact passa attraverso la [modalità di autorizzazione](/docs/it/permission-modes) della tua sessione:

* **Modalità Auto**: il classificatore esamina la pubblicazione invece di chiederti, quindi Claude può pubblicare una pagina senza che tu veda un prompt. La modalità in cui le tue sessioni iniziano dipende dal tuo piano; consulta [la modalità di autorizzazione iniziale](/docs/it/permission-modes#eliminate-prompts-with-auto-mode).
* **Modalità Manuale e Accetta modifiche**: Claude Code chiede l'autorizzazione; potrebbe dire qualcosa come `Claude wants to publish deploy-failures.html, uploading it to claude.ai (Anthropic's servers) to host as the page "Deploy failures by service", private to you until you share it`. Seleziona **Sì** per pubblicare.

Dopo aver approvato un artifact una volta, Claude Code lo ripubblica senza chiedere, e chiede di nuovo in alcuni casi, inclusi quando:

* Claude dichiara una capacità di runtime per la pagina, come [connector calls](#pull-live-data-with-mcp-connectors) o [file downloads](#offer-a-file-download)
* Da allora hai [condiviso pubblicamente](#share-an-artifact)
* Da allora hai condiviso con persone specifiche o la tua organizzazione con l'ultima versione scelta come versione che i visualizzatori vedono

Dopo la prima pubblicazione, Claude stampa l'URL e il tuo browser si apre sulla nuova pagina. Se hai inviato il prompt tramite [Remote Control](/docs/it/remote-control) da claude.ai, Claude Desktop o l'app mobile Claude, nessuna scheda si apre sulla macchina che esegue la sessione. Il browser si apre lì la prossima volta che Claude pubblica l'artifact da un prompt che digiti al terminale. Premi `Ctrl+]` in qualsiasi momento per riaprire l'artifact più recente della sessione.

Claude sceglie il titolo dell'artifact e un emoji, e entrambi appaiono nella tua [galleria di artifact](#share-an-artifact) su claude.ai e nei link condivisi. Claude può anche scegliere un'icona della scheda del browser che corrisponda a ciò che la pagina è, come un grafico o un calendario. Chiedi a Claude per un titolo, un emoji o un'icona della scheda specifici se ne desideri uno.

Per impedire al browser di aprirsi automaticamente quando viene pubblicato un nuovo artifact, imposta `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` nel tuo ambiente.

Se Claude risponde che non può pubblicare, o scrive un file HTML locale senza un link, lo strumento non è abilitato per la tua sessione. Controlla i requisiti di [Disponibilità](#availability).

<h2 id="update-an-artifact">
  Aggiornare un artifact
</h2>

Chiedete a Claude di revisionare la pagina, oppure lasciate che un'attività a lunga esecuzione ripubblichi man mano che fa progressi. Claude modifica il file sottostante e ripubblica allo stesso URL.

```text wrap theme={null}
Aggiungete una suddivisione per regione sotto il grafico di riepilogo e ripubblicate.
```

Chiunque abbia la pagina aperta vede l'aggiornamento in tempo reale. Ogni pubblicazione diventa una versione, e dal controllo **Share** nell'intestazione della pagina potete scegliere quale versione vedono i visualizzatori.

Per aggiornare un artifact da una sessione diversa, fornite a Claude il suo URL, oppure allegate con [`/artifacts`](#find-an-artifact-again). Senza uno dei due, una nuova sessione crea un nuovo artifact invece di aggiornarne uno.

```text wrap theme={null}
Aggiornate https://claude.ai/code/artifact/5fbea6f3-... con i numeri di oggi.
```

<h2 id="find-an-artifact-again">
  Trovare di nuovo un artifact
</h2>

Esegui `/artifacts` in Claude Code per elencare ogni artifact che possiedi e ogni artifact condiviso con te. Selezionane uno e premi `o` per aprirlo nel tuo browser o `c` per copiare il suo link. Premi `Invio` per allegarlo alla sessione corrente; prima della v2.1.216, `Invio` lo apriva nel tuo browser. Claude Code legge l'elenco dal tuo account claude.ai, quindi funziona in una nuova sessione e dopo `/clear`, quando il link è scomparso dal terminale. Richiede Claude Code v2.1.208 o versione successiva.

<h2 id="share-an-artifact">
  Condividere un artifact
</h2>

Un nuovo artifact è visibile solo a voi. Per condividerlo, aprite l'artifact nel vostro browser e utilizzate il controllo **Share** nell'intestazione della pagina. L'intestazione contiene anche un collegamento alla vostra galleria su [claude.ai/code/artifacts](https://claude.ai/code/artifacts), che elenca ogni artifact che avete creato.

I visualizzatori della vostra organizzazione possono vedere chi ha pubblicato la pagina: su un artifact condiviso all'interno della vostra organizzazione, il vostro nome è nel menu del titolo, e su un artifact pubblico è nell'intestazione della pagina per i visualizzatori che hanno effettuato l'accesso nella vostra organizzazione. Un visualizzatore che apre un collegamento pubblico senza effettuare l'accesso, o da fuori della vostra organizzazione, vede l'etichetta `Content is user-generated and unverified.` al posto del vostro nome.

Chi potete condividere dipende dal vostro piano:

* **All'interno della vostra organizzazione**: nei piani Team e Enterprise, concedete l'accesso a persone specifiche della vostra organizzazione, o a tutti. I visualizzatori effettuano l'accesso a claude.ai come membri della vostra organizzazione per vedere la pagina.
* **Pubblicamente**: condividete un collegamento che chiunque su internet può aprire, senza richiedere l'accesso a claude.ai. Nei piani Pro e Max, un collegamento pubblico è l'unico modo per condividere un artifact. Nei piani Team e Enterprise, la condivisione pubblica è disattivata finché un Owner non la [abilita per l'organizzazione](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Permettere a qualcuno di modificare con voi
</h3>

Le persone con cui condividete sono visualizzatori per impostazione predefinita: vedono ogni versione che pubblicate ma non possono modificare la pagina. Nei piani Team e Enterprise, potete anche rendere qualcuno un editor. Nella finestra di dialogo di condivisione, aggiungete una persona e cambiate il suo ruolo da **viewer** a **editor**.

Un editor pubblica nuove versioni nello stesso modo in cui voi [aggiornate l'artifact da un'altra sessione](#update-an-artifact): forniscono a Claude l'URL dell'artifact, o lo allegano da [`/artifacts`](#find-an-artifact-again), e Claude estrae il contenuto corrente e ripubblica con le loro modifiche. Tutti coloro che hanno la pagina aperta vedono ogni aggiornamento in tempo reale.

<h2 id="read-an-artifact-shared-with-you">
  Leggere un artefatto condiviso con voi
</h2>

Quando qualcuno condivide un artefatto con voi, potete chiedere a Claude di leggerlo: fornite a Claude il suo URL, oppure allegate lo dal [`/artifacts`](#find-an-artifact-again).

Claude legge una pagina scritta da qualcun altro nello stesso modo in cui legge una pagina web con [WebFetch](/docs/it/tools-reference#webfetch-tool-behavior): ottiene un riepilogo di ciò che ha chiesto piuttosto che la pagina grezza, e il riepilogo riporta le istruzioni scritte nella pagina invece di trasmetterle. Claude Code salva anche il codice sorgente completo della pagina in un file locale, che Claude può aprire quando ha bisogno del contenuto esatto, ad esempio per ripublicare l'artefatto come [editor](#let-someone-edit-with-you).

<h2 id="collect-comments-on-an-artifact">
  Raccogliere commenti su un artefatto
</h2>

Quando condividi un artefatto all'interno della tua organizzazione, le persone con cui lo condividi possono lasciare commenti sulla pagina, e tu puoi chiedere a Claude di leggere quei commenti e rispondere. Hai bisogno di Claude Code v2.1.221 o successivo e di un piano Team o Enterprise, perché solo un artefatto che [condividi all'interno della tua organizzazione](#share-an-artifact) riceve commenti. Claude legge i commenti in due casi:

* **Tu chiedi a Claude di leggerli**: fornisci a Claude l'URL dell'artefatto e chiedi i commenti. Claude elenca ogni thread e contrassegna i commenti che qualcuno che può modificare l'artefatto ha inviato.
* **Qualcuno che può modificare l'artefatto invia un commento a Claude**: in un thread sulla pagina, invia un commento con **Send to Claude**, o menziona `@claude` in uno. In entrambi i casi, attivano il thread.

Claude può rispondere o risolvere solo un thread attivato. Gli altri thread rimangono aperti finché una persona non li risolve sulla pagina. I visualizzatori vedono ogni risposta attribuita a Claude, tramite te.

Se condividi un artefatto pubblicamente, i visualizzatori non possono commentare: la pagina dice `Comments aren't available while this Artifact is shared publicly.` Per passare un artefatto che ha già thread di commenti a un link pubblico, elimina prima i thread.

Per chiedere i commenti tu stesso, fornisci a Claude l'URL:

```text wrap theme={null}
Read the comments on https://claude.ai/code/artifact/5fbea6f3-... and make the changes the commenters ask for.
```

Se Claude ti dice che non riesce a leggere i commenti, conferma la tua versione, la tua sessione e l'impostazione del tuo feature flag:

* Stai eseguendo Claude Code v2.1.221 o successivo.
* Non sei nella tua prima sessione da quando hai installato Claude Code o aggiornato da una versione precedente a v2.1.221. In quella [prima sessione dopo un'installazione o un aggiornamento](/docs/it/env-vars#first-session-after-an-install-or-upgrade), Claude potrebbe non essere ancora in grado di leggere i commenti; avvia una nuova sessione e chiedi di nuovo.
* Non hai disattivato il recupero dei feature flag.

<h3 id="let-claude-reply-to-comments-on-its-own">
  Lascia che Claude risponda ai commenti da solo
</h3>

Dopo che la tua sessione pubblica un artefatto, Claude Code osserva quell'artefatto per i commenti finché la sessione è in esecuzione. Quando qualcuno che può modificare l'artefatto invia un commento a Claude, raggiunge la tua sessione subito, e Claude può leggere il thread e rispondere senza che tu lo chieda.

Hai bisogno di Claude Code v2.1.228 o successivo. Se hai disattivato il [recupero dei feature flag](/docs/it/env-vars#features-that-need-feature-flag-fetching), Claude Code non osserva i commenti.

La tua [modalità di autorizzazione](/docs/it/permission-modes) decide cosa fa Claude quando arriva un commento inviato:

* **Claude risponde da solo**: quando la tua modalità di autorizzazione consente a Claude di pubblicare la risposta senza chiederti, Claude legge il thread e risponde, e modifica l'artefatto quando il commento chiede una modifica. Vedi `Auto-replied to comment thread on Artifact: <name>` o `Auto-edited Artifact: <name> in response to a comment thread`.
* **Claude aspetta te**: al di fuori della plan mode, quando la pubblicazione della risposta avrebbe bisogno della tua approvazione, vedi `Comments are waiting on Artifact: <name>`. Claude quindi ti chiede l'approvazione per leggere il thread, e di nuovo per pubblicare la risposta.
* **Claude si mette in pausa in plan mode**: vedi `Comments are waiting on Artifact: <name>`, e Claude non risponde finché non esci dalla plan mode e non gli chiedi di leggere e rispondere.

Claude smette anche di rispondere da solo a un artefatto dopo aver gestito 60 commenti inviati o attivazioni di thread su quell'artefatto entro un'ora. Vedi `Comments are waiting on Artifact: <name>` una volta, e Claude riprende mentre i commenti di quell'ora invecchiano.

Esegui `/tasks` per vedere ogni artefatto che la tua sessione sta osservando, elencato come un task con aggiornamenti in tempo reale. Puoi impedire a Claude di rispondere da solo in uno qualsiasi di questi modi:

* **Premi Ctrl+C una volta al prompt inattivo**: Claude mette in pausa la risposta su ogni artefatto che la tua sessione sta osservando. Le risposte ricominceranno dopo che invii il tuo prossimo messaggio.
* **Ferma il task in `/tasks`**: Claude smette di rispondere su quell'artefatto finché non gli chiedi di riprendere le risposte lì. Pubblicare di nuovo l'artefatto non avvia di nuovo le risposte, e l'arresto si applica ancora quando riprendi la sessione in seguito.
* **Premi `Ctrl+X Ctrl+K` due volte entro 3 secondi**: l'accordo che [ferma ogni subagent di background in esecuzione](/docs/it/interactive-mode#general-controls) ferma anche Claude dal rispondere su ogni artefatto per il resto della sessione. Chiedere a Claude di riprendere le risposte non annulla questo arresto.

Se il servizio che fornisce i commenti diventa non disponibile o smette di rispondere, Claude Code continua a provare a riconnettersi per un po', quindi smette di osservare ogni artefatto che la tua sessione stava osservando.

<h2 id="pull-live-data-with-mcp-connectors">
  Estrarre dati live con i connettori MCP
</h2>

Un artifact può chiamare i [connettori MCP](/docs/it/mcp#use-mcp-servers-from-claude-ai) ogni volta che qualcuno lo visualizza, in modo che la pagina mostri dati attuali anziché uno snapshot dalla sessione che l'ha creato. Le chiamate ai connettori dagli artifact sono disponibili nei piani Pro, Max, Team ed Enterprise e richiedono Claude Code v2.1.209 o versioni successive. Nelle versioni precedenti, Claude pubblica la pagina con i dati che la sessione ha raccolto durante la creazione.

Per creare una pagina supportata da un connettore, nomina il connettore e i dati che desideri nel tuo prompt:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude dichiara quali connettori la pagina può chiamare come parte della pubblicazione, e la pagina non può chiamare connettori al di fuori di quella dichiarazione. Solo i connettori dal tuo account claude.ai si qualificano: Claude li nomina nella dichiarazione, e quando qualcuno visualizza la pagina, ogni chiamata [viene eseguita attraverso la connessione dell'account che visualizza al connettore](#how-connector-calls-work-for-viewers). I server MCP locali che configuri in Claude Code, come i server da `.mcp.json`, possono fornire dati mentre Claude crea la pagina, ma la pagina pubblicata non può chiamarli.

La pagina recupera i dati quando si carica e può aggiornarsi a intervalli o quando un visualizzatore utilizza un controllo di aggiornamento sulla pagina. Le risposte vengono memorizzate nella cache del browser del visualizzatore, quindi una pagina riaperta viene renderizzata dalle risposte memorizzate nella cache immediatamente, quindi si aggiorna con risultati freschi.

<h3 id="how-connector-calls-work-for-viewers">
  Come funzionano le chiamate ai connettori per i visualizzatori
</h3>

Quando una pagina pubblicata chiama un connettore, la chiamata utilizza l'account della persona che visualizza la pagina, non l'account della persona che l'ha pubblicata:

* **Ogni visualizzatore utilizza i propri connettori**: le chiamate passano attraverso gli strumenti connessi dell'account che visualizza, quindi due persone che aprono lo stesso dashboard possono vedere dati diversi a seconda di ciò a cui i loro account possono accedere. La pagina non vede mai le credenziali di nessuno; claude.ai effettua le chiamate per conto della pagina.
* **I visualizzatori approvano l'accesso per primo**: claude.ai chiede a ogni visualizzatore il permesso prima della prima chiamata al connettore della pagina. Un visualizzatore che rifiuta, o che non ha connesso un connettore che la pagina utilizza, vede comunque la pagina senza le sue sezioni live.
* **Le azioni utilizzano anche l'account del visualizzatore**: una pagina può offrire controlli che invocano strumenti connettore con effetti collaterali, come pubblicare un messaggio o aggiornare un problema. L'azione viene eseguita attraverso l'account di chiunque selezioni il controllo.

Quando pianifichi di condividere una pagina supportata da un connettore, chiedi a Claude di includere un messaggio di fallback in ogni sezione live che nomini il connettore di cui ha bisogno. Un visualizzatore che manca della connessione vede quindi cosa connettere invece di una sezione vuota.

Un artifact che chiama connettori non può essere condiviso a un link pubblico su nessun piano. Nei piani Team ed Enterprise, puoi mantenerlo privato o [condividerlo all'interno della tua organizzazione](#share-an-artifact). Nei piani Pro e Max, dove un link pubblico è l'unico modo per condividere, un artifact supportato da un connettore rimane privato per te.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  La pagina non mostra dati live per un visualizzatore
</h3>

Quando una pagina supportata da un connettore viene renderizzata ma le sue sezioni live rimangono vuote per qualcuno con cui l'hai condivisa, esamina queste cause:

* **Il visualizzatore non ha connesso il connettore**: i connettori sono per account, quindi ogni visualizzatore ha bisogno della propria connessione a ogni connettore che la pagina chiama. Possono aggiungerne uno in **Settings > Connectors** su claude.ai, quindi ricaricare la pagina.
* **Il visualizzatore ha rifiutato la richiesta di permesso**: un rifiuto dura per il resto di quel caricamento di pagina. Ricaricare la pagina riporta la richiesta di permesso.
* **Le chiamate ai connettori sono disattivate per l'organizzazione**: un Proprietario controlla l'interruttore [**Enable artifact connectors**](#control-connector-calls-from-artifacts) nelle impostazioni di amministrazione.
* **La pagina chiama nomi di strumenti che il connettore non espone**: le sezioni interessate rimangono vuote per tutti, incluso te. Questo può accadere quando una pagina nomina i singoli strumenti dietro un connettore di tipo gateway che espone solo alcuni dei suoi strumenti. Chiedi a Claude di correggere i nomi degli strumenti che la pagina chiama e di pubblicarla di nuovo.

  Quando Claude pubblica la pagina e gli strumenti di quel connettore sono disponibili nella tua sessione, Claude Code controlla i nomi degli strumenti che la pagina dichiara rispetto ad essi, avverte Claude sui nomi che non corrispondono, e rifiuta la pubblicazione quando nessuno corrisponde. Prima della v2.1.265, pubblicava la pagina senza controllarli.

<h2 id="offer-a-file-download">
  Offrire un download di file
</h2>

Un artifact può offrire ai visualizzatori un file generato dalla pagina, come un'esportazione CSV di una tabella o un PNG di un grafico. Il visualizzatore lo salva attraverso un controllo di download sulla pagina, come un pulsante. I download di file sono una capacità di runtime che claude.ai abilita per account, quindi Claude verifica se il vostro account la possiede prima di costruire il controllo.

I visualizzatori non possono salvare un file da un ordinario link di download o da uno script sulla pagina, perché il visualizzatore di artifact su claude.ai blocca qualsiasi download che la pagina avvia da sola, inclusi i link agli URL `data:` o `blob:`. Se una pagina ha pulsanti di download costruiti in questo modo, chiedete a Claude di ricostruirli con la capacità di download.

Per offrire un file, chiedete il controllo e il formato del file nel vostro prompt:

```text wrap theme={null}
Add a button that downloads this table as a CSV file.
```

Claude dichiara la capacità di download come parte della pubblicazione, nello stesso modo in cui [dichiara i connettori](#pull-live-data-with-mcp-connectors).

<h2 id="what-you-can-build">
  Cosa puoi costruire
</h2>

Un artifact è una singola pagina HTML, quindi tutto ciò che puoi esprimere in HTML, CSS e JavaScript inline rientra nell'ambito. I modelli seguenti sono i più comuni.

<h3 id="walk-through-a-change">
  Analizzare una modifica
</h3>

Chiedi una pagina che renderizzi un diff o una modifica di design con annotazioni accanto alle righe rilevanti, in modo che i revisori possano leggere il tuo ragionamento accanto al codice invece di ricostruirlo da una descrizione.

```text wrap theme={null}
Crea un artifact che analizzi questo PR. Renderizza il diff con annotazioni a margine e codifica per colore i risultati in base alla gravità.
```

<h3 id="compare-alternatives">
  Confrontare alternative
</h3>

Chiedi diverse varianti su una pagina in modo da poter valutarle l'una rispetto all'altra. Questo funziona per layout, copy, forme API o piani di implementazione.

```text wrap theme={null}
Crea un artifact con quattro layout distintamente diversi per il pannello delle impostazioni. Varia la densità e il raggruppamento, e disponili come una griglia con un compromesso su una riga sotto ciascuno.
```

<h3 id="tune-with-interactive-controls">
  Regolare con controlli interattivi
</h3>

Chiedi cursori, interruttori o campi di input associati a tutto ciò che stai regolando, in modo da poter esplorare i valori direttamente invece di descriverli.

```text wrap theme={null}
Costruisci un artifact con cursori per la curva di easing, la durata e il ritardo in modo da poter provare i valori su questa transizione. Mostra l'animazione dal vivo mentre li sposti.
```

<h3 id="bring-the-result-back-to-your-session">
  Riportare il risultato nella tua sessione
</h3>

Un artifact può fungere da editor leggero per una decisione che poi invii di nuovo a Claude. Chiedi un controllo di esportazione che produca testo che puoi incollare nel terminale, in modo che il risultato dell'interazione con la pagina torni nella sessione invece di rimanere sulla pagina.

```text wrap theme={null}
Crea un artifact della bacheca di triage con ogni problema aperto come una scheda trascinabile tra le colonne Now, Next, Later e Cut. Aggiungi un pulsante "Copy as prompt" che mi dia l'ordinamento finale da incollare di nuovo qui.
```

<h3 id="track-work-in-progress">
  Tracciare il lavoro in corso
</h3>

Chiedi a Claude di mantenere un artifact aggiornato mentre un'attività lunga è in esecuzione, in modo che chiunque abbia il link possa seguire senza leggere il terminale.

```text wrap theme={null}
Trasforma questo piano di migrazione in un artifact di checklist. Spunta gli elementi mentre li completi e aggiungi una nota per qualsiasi cosa tu salti.
```

<h2 id="improve-the-visual-design">
  Migliorare il design visivo
</h2>

Claude applica una skill di design integrata quando crea un artifact, quindi le pagine ottengono una palette deliberata, tipografia e layout senza necessità di prompt aggiuntivi. Quella skill cerca anche un sistema di design esistente nel vostro progetto prima di scegliere il proprio. I design token sono i valori denominati di colore, tipografia e spaziatura che il vostro sistema di design riutilizza. Per mantenere gli artifact coerenti con il branding del vostro prodotto, registrateli dove Claude può trovarli, come il [CLAUDE.md](/docs/it/memory) del progetto o un file di tema nel vostro repository:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude tratta il vostro sistema di design come priorità più alta rispetto alle sue scelte, e il vostro prompt come priorità più alta rispetto a entrambi. L'intestazione e il formato sopra sono un esempio; qualsiasi elenco chiaro di colori, font e spaziatura funziona.

Per la tipografia, Claude può caricare un carattere da Google Fonts, l'unica fonte di font esterno che una pagina artifact può caricare. Claude incorpora qualsiasi altro carattere come data URI `@font-face` e assegna a ogni carattere uno stack di fallback, quindi la pagina viene comunque visualizzata se un font non si carica. Per utilizzare un carattere specifico, nominate lo nel vostro prompt o nel vostro sistema di design.

<h2 id="draft-a-design-canvas">
  Bozza di una tela di progettazione
</h2>

Per creare un mockup di un'interfaccia utente, un flusso di schermata, una pagina di destinazione o un poster piuttosto che costruire una pagina, eseguire `/design` con un breve. Claude bozza il progetto come tavole da disegno su una tela e pubblica la tela come un artefatto Design. Il breve nomina ciò che desiderate disegnare:

```text wrap theme={null}
/design a settings screen for a mobile banking app
```

Apri l'artefatto pubblicato in un browser desktop per rivedere le tavole da disegno. Seleziona un elemento su una tavola da disegno e modificalo, e le tue modifiche vengono salvate automaticamente. Puoi esportare ogni tavola da disegno come PNG o PDF.

`/design` richiede una sessione in cui [gli artefatti sono disponibili](#availability) e Claude Code v2.1.265 o versioni successive.

<h2 id="page-constraints">
  Vincoli della pagina
</h2>

Ogni artefatto è una pagina autonoma e indipendente. Claude Code avvolge il file che pubblicate in una shell di documento HTML e lo serve secondo una rigorosa Content Security Policy (CSP), che determina ciò che la pagina può fare.

| Vincolo                 | Effetto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Richieste esterne       | La pagina può caricare caratteri tipografici da Google Fonts e script da [cinque host CDN pubblici](#allowlist-the-viewer-domain): cdnjs, unpkg, i CDN Tailwind e jQuery, e percorsi selezionati su jsDelivr come `/npm/`. La CSP blocca ogni immagine esterna e tutti gli altri script, fogli di stile e caratteri tipografici esterni, e consente alle chiamate `fetch`, XHR e WebSocket di raggiungere solo l'origine della pagina stessa e gli host di Google Fonts. Claude quindi carica qualsiasi libreria di cui la pagina ha bisogno da uno di questi CDN, incorpora inline tutto il resto del CSS e JavaScript, e incorpora le immagini come data URI. Le [chiamate Connector](#pull-live-data-with-mcp-connectors) passano attraverso claude.ai, che effettua la chiamata di rete stessa. |
| Nessun backend          | Un artefatto è una pagina statica. Non può autenticare i visualizzatori da solo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Download                | La pagina non può avviare un download da sola. Per consentire ai visualizzatori di salvare un file generato dalla pagina, Claude dichiara la capacità di download. Vedere [Offrire un download di file](#offer-a-file-download).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Pagina singola          | I link relativi non si risolvono, perché nulla viene distribuito insieme alla pagina. Per contenuti multi-sezione, Claude utilizza ancore in-page piuttosto che file separati.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Tipi di file di origine | Il file pubblicato deve essere `.html`, `.htm` o `.md`, e deve decodificarsi come UTF-8, o come UTF-16 little-endian dal suo byte-order mark. I file Markdown vengono renderizzati come pagine di documento stilizzate con codice evidenziato dalla sintassi. Un file che non si decodifica, o che contiene il carattere di sostituzione `U+FFFD`, viene [rifiutato con la riga e la colonna da correggere](/docs/it/errors#the-source-file-is-not-valid-utf-8-text).                                                                                                                                                                                                                                                                                                                                    |
| Dimensione renderizzata | La pagina renderizzata deve essere di 16 MiB o inferiore. Le immagini incorporate di grandi dimensioni sono la causa usuale quando una pubblicazione non riesce per dimensione.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

La generazione di un artefatto utilizza token di output come qualsiasi altra risposta, e una pagina stilizzata è più intensiva in termini di token rispetto allo stesso contenuto come testo di terminale. CSS inline, JavaScript per controlli interattivi, e soprattutto immagini incorporate come data URI sono i principali contributori. Per ridurre il costo in token di un artefatto:

* Preferire SVG, o HTML e CSS, per i diagrammi rispetto alle immagini raster incorporate
* Omettere l'interattività di cui non avete bisogno
* Fare in modo che la pagina riassuma grandi set di dati piuttosto che incorporarli completamente

<h2 id="availability">
  Availability
</h2>

Gli artifact richiedono ogni condizione di seguito. Quando una non è soddisfatta, Claude scrive un file HTML locale o dice che non può pubblicare.

| Requirement         | Available when                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Plan                | Pro, Max, Team o Enterprise. Sui piani Pro e Max, gli artifact sono privati per te fino a quando non li condividi, e non si applica alcuna gestione amministrativa. Sui piani Team, gli artifact sono abilitati per impostazione predefinita. Sui piani Enterprise, un Owner [li abilita](#manage-artifacts-for-your-organization) nelle impostazioni admin di claude.ai.                                                                                           |
| Authentication      | La sessione è supportata da un account claude.ai: accedi con `/login` nell'app CLI o desktop. Le sessioni Claude Tag sono autenticate attraverso l'identità dell'agente, quindi non è necessario alcun passaggio. Le sessioni che utilizzano una chiave API, [gateway token](/docs/it/llm-gateway) o credenziale del provider cloud non possono pubblicare.                                                                                                              |
| Model provider      | Anthropic API. Non disponibile su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) o [Microsoft Foundry](/docs/it/microsoft-foundry).                                                                                                                                                                                                                                                                                         |
| Organization policy | Le chiavi di crittografia gestite dal cliente (CMEK), HIPAA e [Zero Data Retention](/docs/it/zero-data-retention) non sono abilitate per l'organizzazione.                                                                                                                                                                                                                                                                                                               |
| Surface             | Claude Code CLI o l'app desktop Claude versione 1.13576.0 o successiva. Le sessioni [Claude Tag](https://claude.com/docs/claude-tag/overview) possono anche pubblicare artifact quando sia Claude Tag che gli artifact sono abilitati per l'organizzazione. Disabilitato per impostazione predefinita in contesti [Agent SDK](/docs/it/agent-sdk/overview), GitHub Action e MCP-server, e quando [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars) è impostato. |

Se gli artifact sono consentiti per la tua organizzazione dipende dalla politica della tua organizzazione, che Claude Code carica da `api.anthropic.com`. Quando Claude Code non riesce a caricare la politica, gli artifact non sono disponibili. Quando ne chiedi uno, Claude spiega il motivo.

Se è coinvolto un proxy, una VPN o un filtro web, chiedi al tuo amministratore IT di consentire il passaggio di `api.anthropic.com`. Claude Code continua a riprovare in background e gli artifact diventano disponibili una volta che la politica viene caricata e li consente.

<h2 id="disable-artifacts">
  Disattivare gli artifact
</h2>

Per disattivare gli artifact per le tue sessioni indipendentemente dall'impostazione della tua organizzazione, usa uno qualsiasi di:

| Dove                                        | Cosa fare                                                                                                                                    |
| :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------- |
| [`/config`](/docs/it/commands)                   | Disattiva la riga **Artifacts**, che scrive [`"enableArtifact": false`](/docs/it/settings-reference#enableartifact) nelle tue impostazioni utente |
| [File di impostazioni](/docs/it/settings)        | Imposta `"enableArtifact": false`. Anche il deprecato `"disableArtifact": true` disattiva gli artifact                                       |
| [Variabile di ambiente](/docs/it/env-vars)       | Imposta `CLAUDE_CODE_DISABLE_ARTIFACT=1`                                                                                                     |
| [Regola di autorizzazione](/docs/it/permissions) | Aggiungi `Artifact` a `permissions.deny`                                                                                                     |

Una volta che disattivi gli artifact in un file [`--settings`](/docs/it/cli-reference#cli-flags) o con `CLAUDE_CODE_DISABLE_ARTIFACT`, o il tuo amministratore li disattiva nelle [impostazioni gestite](/docs/it/server-managed-settings), nessun file di impostazioni li riattiva. Prima della v2.1.242, un file più in alto nella [stack di precedenza](/docs/it/settings#settings-precedence) potrebbe riattivare gli artifact anche quando un file con precedenza inferiore impostava `"enableArtifact": false`.

Puoi anche impostare `"enableArtifact": false` nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto per disattivare gli artifact per le sessioni in quel progetto. Un `"enableArtifact": true` in uno dei due file non li riattiva. L'onorare la chiave nelle impostazioni di progetto e locali richiede Claude Code v2.1.242 o successivo.

Se aggiungi una regola di negazione o richiesta `WebFetch` senza una parte `domain:`, non disattiva gli artifact né blocca le letture degli artifact. Una [regola `WebFetch(domain:claude.ai)` in `deny` o `ask` si applica alle letture degli artifact](/docs/it/permissions#allow-or-deny-every-fetch).

<h2 id="manage-artifacts-for-your-organization">
  Manage artifacts for your organization
</h2>

I proprietari sui piani Team e Enterprise controllano gli artifact dalle [impostazioni admin di claude.ai](https://claude.ai/admin-settings/claude-code). Il contenuto dell'artifact è archiviato su infrastruttura gestita da Anthropic ed è visibile solo ai membri autenticati dell'organizzazione che pubblica, a meno che l'artifact non sia [condiviso pubblicamente](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Enable or disable artifacts
</h3>

Per abilitare o disabilitare gli artifact per l'intera organizzazione, vai a [**Settings > Claude Code > Capabilities**](https://claude.ai/admin-settings/claude-code) e usa l'interruttore **Artifacts**. Sui piani Enterprise con controllo dell'accesso basato su ruoli, puoi inoltre limitare gli artifact a ruoli specifici: vai a [**Settings > Roles**](https://claude.ai/admin-settings/roles), modifica un ruolo e imposta l'autorizzazione **Artifacts** nel gruppo **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Control connector calls from artifacts
</h3>

Le [chiamate ai connettori dagli artifact](#pull-live-data-with-mcp-connectors) hanno il loro interruttore dedicato, separato dall'interruttore **Artifacts** che attiva o disattiva gli artifact. Vai a [**Settings > Capabilities**](https://claude.ai/admin-settings/capabilities) e usa l'interruttore **Enable artifact connectors**. Lo stesso interruttore governa le chiamate ai connettori dagli artifact creati nelle conversazioni di claude.ai, motivo per cui si trova sotto **Settings > Capabilities** piuttosto che sotto **Settings > Claude Code**.

<h3 id="control-public-sharing">
  Control public sharing
</h3>

La condivisione pubblica è disattivata per impostazione predefinita sui piani Team e Enterprise, quindi i membri possono condividere gli artifact solo all'interno dell'organizzazione finché un proprietario non la attiva. Per consentire ai membri di pubblicare artifact su link pubblici che chiunque può visualizzare senza accedere, vai a **Settings > Claude Code > Capabilities** e attiva **External sharing** sotto l'interruttore **Artifacts**. Disattivarla di nuovo blocca l'accesso tramite link pubblici esistenti senza modificare il pubblico di ogni artifact; l'accesso riprende se lo riabiliti.

<h3 id="set-a-retention-policy">
  Set a retention policy
</h3>

Per impostare quanto tempo gli artifact vengono conservati prima dell'eliminazione automatica, vai a [**Settings > Data & privacy controls**](https://claude.ai/admin-settings/data-privacy-controls). Puoi impostare periodi di conservazione separati per gli artifact che sono ancora privati al loro autore e gli artifact che sono stati condivisi.

<h3 id="review-the-audit-log">
  Review the audit log
</h3>

La pubblicazione, la condivisione e l'eliminazione di un artifact appaiono ciascuna nel registro di audit della tua organizzazione sotto i tipi di evento `claude_artifact_*`, la stessa famiglia utilizzata per gli artifact creati nelle conversazioni di claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Allowlist the viewer domain
</h3>

Il visualizzatore su claude.ai carica ogni artifact da un'origine sandbox `*.claudeusercontent.com`. Se la tua organizzazione limita l'accesso alla rete in uscita, aggiungi quel dominio alla tua allowlist insieme a `claude.ai`. Vedi [Requisiti di accesso alla rete](/docs/it/network-config#network-access-requirements) per l'elenco completo.

Un artifact che carica un carattere tipografico da [Google Fonts](#improve-the-visual-design) richiede anche `fonts.googleapis.com` e `fonts.gstatic.com`. Entrambi gli host sono facoltativi. Se li blocchi, gli artifact vengono renderizzati in caratteri tipografici di fallback. Blocca con un rifiuto veloce piuttosto che con un drop silenzioso in modo che la richiesta del carattere tipografico fallisca immediatamente invece di ritardare il primo rendering della pagina.

Gli artifact possono anche caricare librerie JavaScript, come React o un pacchetto di grafici, da `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` e `unpkg.com`, e da nessun altro host esterno. Se blocchi questi host, le parti di un artifact che dipendono da una libreria non funzionano, e a differenza di un carattere tipografico bloccato, una libreria bloccata non ha fallback. Blocca con un rifiuto veloce anche qui, in modo che una richiesta di libreria bloccata fallisca immediatamente piuttosto che rimanere in sospeso fino al timeout.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  List and delete artifacts with the Compliance API
</h3>

La [Compliance API](https://docs.claude.com/en/api/compliance) fornisce endpoint per elencare gli artifact di un'organizzazione, recuperare il contenuto di una versione specifica e eliminare un artifact:

| Method   | Endpoint                                                            |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

Per gli schemi di richiesta e risposta, vedi il [riferimento della Compliance API](https://docs.claude.com/en/api/compliance/code/artifacts).

<h2 id="related-resources">
  Risorse correlate
</h2>

* Sfoglia [pattern di prompting e workflow](/docs/it/prompt-library) che si abbinano agli artifact
* Trasforma un prompt di artifact che riutilizzi in una [skill](/docs/it/skills) in modo da poterlo invocare come comando
* [Connetti server MCP](/docs/it/mcp) in modo che Claude possa estrarre dati in un artifact mentre lo crea
