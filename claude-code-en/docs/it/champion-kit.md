> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Champion kit

> Una guida pratica per gli ingegneri che promuovono Claude Code internamente: cosa condividere, come rispondere alle domande e come aumentare l'adozione nel tuo team.

Questa pagina è per i singoli ingegneri che stanno già utilizzando Claude Code e vogliono aiutare il loro team ad adottarlo. Copre cosa condividere, come rispondere alle domande che riceverai, una guida di trenta giorni e risposte alle preoccupazioni comuni.

L'adozione di uno strumento per sviluppatori raramente avviene a causa di un annuncio di rollout. Avviene perché qualcuno nel team inizia a utilizzare lo strumento bene, ne parla apertamente e rende facile per gli altri seguire. Il lavoro che svolgi come champion ha un effetto sproporzionato: ogni esempio che condividi accorcia la curva di apprendimento per gli ingegneri che verranno dopo di te, e ogni domanda che rispondi pubblicamente trasforma l'esperienza di una persona in qualcosa su cui l'intero team può costruire. Stai agendo come un moltiplicatore per il tuo team, non come un help desk, e questa guida è strutturata per mantenere il ruolo sostenibile su questi termini.

<h2 id="the-champion-role">
  Il ruolo di champion
</h2>

Il ruolo consiste in tre comportamenti che si rafforzano a vicenda.

| Comportamento                          | Come appare nella pratica                                                                                                                                                                          | Perché è importante                                                                                                                                                                                       |
| -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Condividi quello che scopri            | Pubblica i prompt, gli screenshot e le piccole vittorie dal tuo lavoro nei luoghi che il tuo team già legge, come un canale di ingegneria, un thread di standup o una descrizione di pull request. | Gli esempi tratti dal tuo codebase sono più persuasivi di qualsiasi documentazione esterna, perché i colleghi possono vedere esattamente come lo strumento si applica ai problemi che condividono con te. |
| Sii la persona che le persone chiedono | Quando un collega ti chiede come hai realizzato qualcosa, rispondi con il prompt effettivo che hai utilizzato in modo che possano applicarlo direttamente al loro compito.                         | Un esempio concreto e eseguibile rimuove il divario tra la curiosità e un primo utilizzo riuscito, che è dove la maggior parte degli sforzi di adozione si blocca.                                        |
| Fai crescere il cerchio                | Stabilisci un piccolo numero di abitudini ricorrenti leggere, come un canale dedicato o un thread settimanale, in modo che lo slancio continui anche quando la tua attenzione è altrove.           | L'adozione che dipende da una sola persona è fragile. L'adozione che è portata da abitudini condivise continua a crescere da sola.                                                                        |

La maggior parte di questo si adatta naturalmente al lavoro che stai già facendo. La differenza è una piccola quantità di intenzione aggiuntiva su dove vengono pubblicate le tue scoperte e come i tuoi insegnamenti si diffondono.

<h3 id="what-this-should-cost-you">
  Quanto dovrebbe costarti
</h3>

Stabilisci aspettative con te stesso e con il tuo lead. Le attività di seguito sono intese per adattarsi a una normale settimana lavorativa, e il ruolo dovrebbe rimanere un moltiplicatore del tuo lavoro esistente piuttosto che una responsabilità di supporto aggiuntiva.

| Attività                                        | Tempo per settimana | Guida                                                                                                                         |
| ----------------------------------------------- | ------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Pubblicazione di vittorie e prompt              | Circa 15 minuti     | Cattura questi al momento con uno screenshot e una o due frasi; evita di trasformarli in articoli formali.                    |
| Rispondere alle domande in un canale condiviso  | Circa 20 minuti     | Rispondi pubblicamente una volta, quindi collega a quella risposta quando la domanda si ripete.                               |
| Ospitare un thread settimanale di show-and-tell | Circa 5 minuti      | Tu pubblichi il prompt di apertura; il team fornisce il contenuto.                                                            |
| Pairing o walkthrough opzionali                 | Da 0 a 30 minuti    | Riservalo ai colleghi che sono veramente bloccati e offri il link [Quickstart](/docs/it/quickstart) prima di programmare il tempo. |

<h2 id="share-what-you-discover">
  Condividi quello che scopri
</h2>

La tua esperienza personale è il materiale più persuasivo che i tuoi colleghi incontreranno, perché è specifico per il codebase, i flussi di lavoro e i problemi che condividete tutti. La documentazione dice alle persone cosa è possibile; i tuoi post mostrano loro cosa sta effettivamente funzionando nel tuo ambiente.

<h3 id="what-is-worth-sharing">
  Cosa vale la pena condividere
</h3>

I post più utili descrivono una tecnica che un collega può riutilizzare domani piuttosto che un risultato che è già completo. Le tecniche si compongono mentre si diffondono attraverso un team; gli aggiornamenti di stato no.

Esempi di tecniche riutilizzabili:

* "Ho imparato che @-menzionare una directory funziona. Puntandola a `@src/components/` e chiedendo quali mancavano di test, ho scoperto due che avevo trascurato."
* "Plan mode (`Shift+Tab`) mostra esattamente quali file verranno toccati prima di qualsiasi modifica, ecco perché sono a mio agio nell'usarlo su codice condiviso."
* "Ho configurato un hook Stop in modo da ricevere una notifica desktop quando un'attività lunga si completa. La configurazione è nel thread."
* "Eseguire `/init` genera un `CLAUDE.md` dal repository in modo che l'assistente smetta di chiedere di nuovo delle nostre convenzioni."

<h3 id="where-to-share-it">
  Dove condividerlo
</h3>

Pubblica ovunque il tuo team già legge. L'obiettivo è posizionare gli esempi nel percorso del lavoro normale piuttosto che creare una destinazione.

| Posizione                                         | Più adatta per                                                            | Formato consigliato                                                                                |
| ------------------------------------------------- | ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Un canale `#claude-code` o di ingegneria generale | Scoperte, prompt e momenti "oggi ho imparato"                             | Uno screenshot accompagnato da una o due frasi di contesto                                         |
| Descrizioni di pull request                       | Dimostrare l'approccio su codice reale che i revisori stanno già leggendo | Una singola riga come "Claude e io abbiamo fatto questo refactor; felice di spiegare l'approccio." |
| Standup o aggiornamenti scritti settimanali       | Normalizzare l'utilizzo con lead e manager skip-level                     | Una frase che descrive un risultato concreto                                                       |
| Wiki del team o documentazione interna            | Modelli durevoli, skill personalizzate e esempi di `CLAUDE.md`            | Una breve pagina, collegata dal topic del canale in modo che rimanga scopribile                    |

<h3 id="the-format-that-works">
  Il formato che funziona
</h3>

Uno screenshot accompagnato da una singola riga di contesto, o una breve descrizione prima e dopo, è generalmente il livello di dettaglio giusto. Mantieni ogni post abbastanza breve che qualcuno che scorre ancora assorba il punto. Un lungo articolo tende ad essere salvato per dopo e dimenticato, mentre un breve post con uno screenshot tende ad essere copiato e provato.

I post di esempio di seguito illustrano il tono e la lunghezza; adattali piuttosto che copiarli verbatim.

```text theme={null}
Ho imparato oggi che @-menzionare una directory funziona. L'ho puntata a
@src/components/ e ho chiesto quali componenti mancavano di test, e ha
scoperto due che avevo dimenticato.
```

```text theme={null}
Ho configurato un hook Stop in modo da ricevere una notifica desktop quando
un'attività lunga si completa. Ho iniziato un refactor, mi sono allontanato
e sono stato notificato quando è finito. La configurazione è nel thread.
```

```text theme={null}
Plan mode è il motivo per cui sono a mio agio nell'usare questo su codice
che conta. Premi Shift+Tab finché non vedi "plan"; mostra esattamente quali
file intende toccare prima di cambiare qualcosa.
```

<h2 id="be-the-person-people-ask">
  Sii la persona che le persone chiedono
</h2>

Una volta che hai condiviso alcuni esempi, seguiranno le domande. Questo è dove il ruolo di champion ha la maggiore leva, perché una buona risposta a una persona spesso sblocca diversi altri che stanno guardando lo stesso canale.

<h3 id="answer-with-a-prompt-rather-than-an-explanation">
  Rispondi con un prompt piuttosto che con una spiegazione
</h3>

Quando un collega ti chiede come hai realizzato qualcosa, la risposta più utile è il prompt che hai effettivamente utilizzato. Impareranno di più eseguendo quel prompt contro il loro problema che da qualsiasi descrizione che potresti scrivere, e dà loro qualcosa su cui possono agire immediatamente.

```text theme={null}
Collega: Come hai fatto a trovare quella race condition?

Champion: Ho chiesto, "Il test in @tests/scheduler.test.ts è instabile, scopri
il motivo," e ha tracciato due promise non unite nello scheduler. Prova la
stessa formulazione sul tuo test.
```

<h3 id="point-at-the-feature-rather-than-the-documentation">
  Punta alla funzione piuttosto che alla documentazione
</h3>

Una risposta come "Prova plan mode, premi `Shift+Tab` finché non lo vedi" è più utile al momento che un link alla documentazione. Se la persona ha bisogno di più profondità in seguito la troverà da sola; adesso ha bisogno della singola cosa che la sblocca.

<h3 id="questions-you-are-likely-to-hear">
  Domande che probabilmente sentirai
</h3>

| Domanda                                               | Risposta suggerita                                                                                                                                                                                                                                          | Risorsa di follow-up                                    |
| ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| "Su cosa dovrei provarlo per primo?"                  | Consiglia un compito reale ma contenuto, idealmente un bug o una chore che la persona ha rimandato perché è noioso piuttosto che difficile.                                                                                                                 | [Common workflows](/docs/it/common-workflows)                |
| "Come faccio a fidarmi di lui con il mio codice?"     | Introduci plan mode: premendo `Shift+Tab` si entra, Claude propone esattamente cosa intende cambiare, e nulla viene modificato fino all'approvazione dell'utente.                                                                                           | [Permissions](/docs/it/permissions)                          |
| "Vale la pena lo sforzo di configurazione?"           | L'installazione richiede circa due minuti, viene eseguita nel terminale e non richiede alcuna estensione IDE. Eseguire `/init` una volta è sufficiente per iniziare a lavorare.                                                                             | [Quickstart](/docs/it/quickstart)                            |
| "Ha prodotto un risultato errato."                    | Incoraggiali a fornire il fallimento a Claude. Incollare il messaggio di errore o il test fallito è molto più efficace che riformulare la richiesta originale.                                                                                              | [Common workflows](/docs/it/common-workflows)                |
| "Non comprende le convenzioni del nostro codebase."   | Suggerisci di eseguire `/init` per generare un file `CLAUDE.md`, quindi aggiungi le convenzioni del team, i comandi di test e qualsiasi directory che dovrebbe essere evitata.                                                                              | [Memory](/docs/it/memory)                                    |
| "È solo autocomplete?"                                | Offri una breve dimostrazione in cui Claude spiega un file sconosciuto, traccia un bug tra i servizi o bozza un piano di migrazione. Questi compiti richiedono il ragionamento attraverso il repository piuttosto che il completamento di una singola riga. | Una dimostrazione dal vivo di due minuti                |
| "Che dire della sicurezza e della gestione dei dati?" | Rimanda questa domanda al tuo amministratore. La politica di distribuzione e gestione dei dati della tua organizzazione è già configurata, e i champion non dovrebbero improvvisare questa risposta.                                                        | [Security](/docs/it/security) · [Data usage](/docs/it/data-usage) |

<h2 id="grow-the-circle">
  Fai crescere il cerchio
</h2>

L'obiettivo non è costruire un programma o possedere un rollout. È stabilire un piccolo numero di abitudini leggere che consentono allo slancio di continuare dopo che hai smesso di guidarlo attivamente. Quando le domande nel canale vengono risposte da persone diverse da te, il ruolo ha fatto il suo lavoro.

<h3 id="patterns-that-tend-to-work">
  Modelli che tendono a funzionare
</h3>

| Modello                                             | Come eseguirlo                                                                                                                                                                                                                                                                                        | Sforzo richiesto                                     |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| Un canale dedicato                                  | Crea un canale `#claude-code` (o un thread ricorrente in uno esistente), fissa il link [Quickstart](/docs/it/quickstart) e un forte esempio, e rispondi alle domande pubblicamente in modo che ogni risposta avvantaggi tutti coloro che stanno guardando.                                                 | Circa cinque minuti per configurare, quindi ambiente |
| Un thread settimanale di show-and-tell              | Ogni venerdì, pubblica "Con cosa ti ha aiutato Claude questa settimana?" Non è richiesta alcuna preparazione, diapositive o riunione; screenshot e brevi descrizioni sono sufficienti.                                                                                                                | Circa due minuti per settimana                       |
| Condividi una skill personalizzata                  | Pubblica il tuo file `.claude/skills/<name>/SKILL.md` più utile, ad esempio una skill `/ship` che esegue test e lint prima di eseguire il commit, con una descrizione di una riga. Poiché le skill sono Markdown semplice, i colleghi possono adottarle immediatamente.                               | Circa cinque minuti per skill                        |
| Genera una guida di configurazione dal tuo utilizzo | Esegui `/team-onboarding` in un progetto su cui hai trascorso tempo reale. Claude scansiona le tue sessioni recenti, i comandi e i server MCP, quindi produce una guida che un nuovo compagno di squadra può incollare come primo messaggio per riprodurre la tua configurazione. Fissala nel canale. | Circa due minuti                                     |
| Pair su un primo compito                            | Offri una singola sessione di pairing di quindici minuti a chiunque stia iniziando. Un risultato riuscito sul loro codice è più persuasivo di qualsiasi presentazione.                                                                                                                                | Circa quindici minuti per persona                    |
| Identifica il prossimo champion                     | Il collega che ti fa più domande è solitamente pronto ad assumere questo ruolo. Inoltri loro questa pagina e dividi le responsabilità del canale tra voi.                                                                                                                                             | Trascurabile                                         |

<h3 id="thirty-day-playbook">
  Guida di trenta giorni
</h3>

Se un piano vago è utile, la sequenza di seguito riflette cosa tende a funzionare nella maggior parte dei team. Adatta liberamente al tuo contesto.

<Steps>
  <Step title="Settimana 1: Semina il canale">
    Crea il canale, fissa [Quickstart](/docs/it/quickstart) e pubblica due o tre dei tuoi esempi con i prompt inclusi.

    **Segnale che sta funzionando:** alcuni colleghi reagiscono o rispondono, e almeno una domanda viene posta nel canale.
  </Step>

  <Step title="Settimana 2: Inizia il ritmo">
    Inizia il thread settimanale di show-and-tell, rispondi a ogni domanda pubblicamente e condividi una skill personalizzata o uno snippet di `CLAUDE.md`.

    **Segnale che sta funzionando:** qualcuno diverso da te pubblica un esempio del proprio.
  </Step>

  <Step title="Settimana 3: Pair e consolida">
    Offri due o tre brevi sessioni di pairing e consolida le domande e le risposte più comuni in un messaggio FAQ fissato.

    **Segnale che sta funzionando:** vedi un utilizzo ripetuto, con gli stessi colleghi che ritornano piuttosto che provare una volta e fermarsi.
  </Step>

  <Step title="Settimana 4: Passa il testimone">
    Identifica un secondo champion e condividi un breve riassunto di cosa sta funzionando e cosa no con il tuo lead o amministratore.

    **Segnale che sta funzionando:** le domande nel canale vengono risposte da persone diverse da te.
  </Step>
</Steps>

<h3 id="when-someone-wants-to-go-deeper">
  Quando qualcuno vuole approfondire
</h3>

Sei l'introduzione calorosa piuttosto che il programma di onboarding. Quando un collega passa da "dovrei provare questo" a "come divento efficace con questo," indirizzalo alle pagine [Quickstart](/docs/it/quickstart) e [Common workflows](/docs/it/common-workflows). Contengono brevi sezioni che coprono le funzioni che sono genuinamente utili ma difficili da scoprire da soli.

<h2 id="respond-to-common-concerns">
  Rispondi alle preoccupazioni comuni
</h2>

Lo scetticismo sano è previsto; gli ingegneri dovrebbero essere cauti riguardo agli strumenti che toccano il loro codice. La risposta più efficace è raramente discutere il caso generale. Invece, riconosci la preoccupazione, offri un breve reframe e proponi una dimostrazione concreta sul codice della persona. La maggior parte delle preoccupazioni viene risolta da una singola esperienza riuscita.

| Preoccupazione                                             | Risposta suggerita                                                                                                                                                                                                                           | Evidenza da offrire                                                |
| ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| "Sono più veloce senza di esso."                           | È probabile che sia vero per il codice che la persona scrive abitualmente. Suggerisci di provarlo sul lavoro che tendono a evitare: file legacy, servizi sconosciuti o scaffolding di test, dove la leva è più alta.                         | Cronometra un compito noioso in entrambi i modi e confronta.       |
| "Non mi fido dell'IA per toccare il codice di produzione." | Accorda che nessun cambiamento dovrebbe arrivare senza essere letto. Plan mode combinato con la normale revisione diff significa che nulla viene applicato che l'ingegnere non ha ispezionato, lo stesso standard di qualsiasi pull request. | Dimostra plan mode su un file reale.                               |
| "Renderà gli ingegneri junior più deboli."                 | Usato bene, è un efficace spiegatore. Incoraggia gli ingegneri junior a chiedere a Claude di spiegare un file e i suoi siti di chiamata prima di chiedere di cambiare qualcosa.                                                              | Esegui "Spiega @file e dove viene chiamato da" insieme.            |
| "Ho provato una volta e ha allucinato."                    | Questo è solitamente un problema di contesto piuttosto che un problema di modello. @-menzionare i file rilevanti, eseguire `/init` e fornire l'output di errore effettivo di solito lo risolve.                                              | Riesegui il loro prompt originale con il contesto `@` appropriato. |
| "Non abbiamo tempo per imparare un altro strumento."       | Claude Code è un comando di terminale piuttosto che una piattaforma. Se non restituisce valore nella prima sessione, è ragionevole metterlo da parte.                                                                                        | Un'installazione di due minuti seguita da un bug reale.            |

<h2 id="quick-reference-sheet">
  Foglio di riferimento rapido
</h2>

Le tecniche di seguito sono quelle che in modo più affidabile spostano qualcuno da una prima prova all'uso quotidiano. Fissa questa tabella in un canale o condividila da sola.

| Tecnica                                   | Come applicarla                                                                                                                                                                             |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Fornisci il contesto giusto               | Usa riferimenti `@file` o `@directory/`, o incolla direttamente l'output di errore o log. Fornire il contesto rilevante è più efficace che un prompt elaborato.                             |
| Rivedi il piano prima della modifica      | Premi `Shift+Tab` per entrare in plan mode. Claude descriverà i cambiamenti previsti per la tua approvazione prima di eseguirli.                                                            |
| Insegnagli il tuo repository              | Esegui `/init` per generare un file `CLAUDE.md`, quindi aggiungi le tue convenzioni, i comandi di test e qualsiasi directory che non dovrebbe essere modificata. Vedi [Memory](/docs/it/memory). |
| Riutilizza un flusso di lavoro            | Salva un file `SKILL.md` in `.claude/skills/<name>/` per creare una skill `/name` che l'intero team può utilizzare. Vedi [Skills](/docs/it/skills).                                              |
| Rimani informato durante i compiti lunghi | Configura un hook Stop per ricevere una notifica desktop quando un'attività di lunga durata si completa. Vedi [Hooks](/docs/it/hooks-guide).                                                     |
| Recupera da un risultato errato           | Piuttosto che riformulare la richiesta, incolla il test fallito o la traccia dello stack a Claude e chiedile di affrontare quel fallimento specifico.                                       |
| Mantieni le modifiche chirurgiche         | Chiedi un diff, o specifica "cambia solo X." Claude rispetta l'ambito quando l'ambito è dichiarato.                                                                                         |

<Tip>
  Claude Code viene aggiornato frequentemente. Verifica i dettagli specifici della versione rispetto alla [pagina iniziale della documentazione](/docs/it/overview) prima di distribuire questo materiale internamente.
</Tip>
