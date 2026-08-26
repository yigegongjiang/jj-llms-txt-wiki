> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kit di comunicazione

> Annunci di lancio, messaggi di campagna a goccia e risposte FAQ per il rollout di Claude Code nella vostra organizzazione di ingegneria.

Questa pagina è per gli amministratori e i responsabili dell'ingegneria che stanno implementando Claude Code in un team. Fornisce annunci di lancio pronti per la copia, una campagna di drip di suggerimenti e trucchi, e risposte FAQ in una riga per le domande che vi verranno poste più frequentemente.

<Note>
  Trattate tutto qui come bozza, non come copia definitiva. Riscrivete ogni messaggio con la voce della vostra organizzazione, sostituite i compiti di esempio con bug e moduli reali dal vostro codebase, e sostituite i `[segnaposti tra parentesi]` prima di inviare. Gli annunci che guidano l'adozione sono quelli che sembrano scritti da qualcuno della vostra azienda.
</Note>

<h2 id="launch-communications">
  Comunicazioni di lancio
</h2>

Un annuncio in due formati, più due varianti opzionali. Scegliete quello che si adatta meglio al vostro rollout e riscrivete da lì.

<h3 id="before-you-send">
  Prima di inviare
</h3>

Completate questa checklist prima che l'annuncio esca. Ogni elemento chiude un divario che altrimenti si trasforma in un thread di supporto nel giorno del lancio.

| Elemento                                                                                            | Perché è importante                                                                                                                                                               |
| --------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Canale `#claude-code` creato e collegato nel messaggio                                              | Dà alle domande un unico posto dove arrivare                                                                                                                                      |
| Comando di installazione testato su almeno una macchina nel vostro ambiente                         | Cattura i problemi di proxy o firewall prima che tutti li colpiscano contemporaneamente                                                                                           |
| Link di sicurezza e gestione dei dati pronto ([Data usage](/docs/it/data-usage) o l'equivalente interno) | "Dove va il mio codice?" sarà la prima risposta                                                                                                                                   |
| Un compito concreto scelto, un bug reale o un file nel vostro codebase                              | Gli esempi generici non convertono; "correggi il test instabile in `auth_test.go`" sì                                                                                             |
| Un proprietario nominato per il canale per le prime 48 ore                                          | Le domande senza risposta nel giorno del lancio uccidono lo slancio                                                                                                               |
| Un sponsor della C-suite in linea per inviare o co-firmare l'annuncio                               | I lanci inviati da un dirigente vedono costantemente tassi di adozione più elevati nella prima settimana rispetto a quelli inviati da un amministratore o da un team di strumenti |

<h3 id="the-announcement">
  L'annuncio
</h3>

Usate questo come messaggio di rollout standard a livello di organizzazione. Spiega cos'è Claude Code, fornisce un percorso di installazione di due minuti, dà ai lettori un compito concreto da provare e risponde a "dove va il mio codice?" prima che qualcuno debba chiedere.

<Tabs>
  <Tab title="Email">
    ```text theme={null}
    Subject: Claude Code è live per [Engineering / vostro team]

    Team,

    A partire da oggi avete accesso a Claude Code, un agente di codifica AI che
    funziona nel vostro terminale, legge il vostro codebase effettivo e lavora
    attraverso compiti reali da capo a fondo: debug, refactor, test, PR. Non è
    autocomplete e non è una finestra di chat. Modifica file, esegue i vostri
    comandi e chiede il permesso prima di fare qualcosa di rischioso.

    Iniziate in due minuti:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <your-repo>
        claude

    Poi eseguite /init una volta. Claude legge il vostro progetto e scrive un
    CLAUDE.md con i vostri comandi di build e convenzioni, così smettete di
    re-spiegare le basi.

    Poi provate uno di questi nel repo in cui siete già:

      - "Il test in [file] è instabile. Scopri perché e correggilo"
      - "Spiegami come [module] gestisce [X]"
      - "Guarda il mio diff funzionante e dimmi cosa è rischioso prima che lo
        spinga"

    Dove va il vostro codice: Claude Code funziona nel vostro terminale e parla
    direttamente all'API di Anthropic, senza server di terze parti nel loop.
    Chiede prima di modificare file o eseguire comandi. Secondo il nostro
    accordo Enterprise, Anthropic non utilizza il vostro codice o i vostri
    prompt per addestrare i suoi modelli.
    Dettagli: https://code.claude.com/docs/it/data-usage
             https://code.claude.com/docs/it/security

    Dove andare con le domande: #claude-code. [Owner name] lo sta monitorando
    questa settimana.

    - [Name]

    P.S. Preferite il vostro editor? C'è un'estensione VS Code e un plugin
    JetBrains. Lo stesso agente, nessun terminale richiesto.
    ```
  </Tab>

  <Tab title="Slack o Teams">
    ```markdown theme={null}
    🚀 *Claude Code è live per [team]*

    Agente di codifica AI, funziona nel vostro terminale, legge il vostro repo,
    fa lavoro reale: bug, refactor, test, PR. Chiede prima di toccare qualcosa.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd your-repo` → `claude`

    *Prima cosa da provare* → esegui `/init`, poi: "il test in [file] è
    instabile, scopri perché e correggilo."

    🔒 Funziona nel vostro terminale, parla solo all'API di Anthropic. Secondo
    il vostro piano Enterprise il vostro codice e i vostri prompt non vengono
    utilizzati per addestrare i modelli.
    Data usage → https://code.claude.com/docs/it/data-usage

    📚 Quickstart · VS Code · Corso gratuito di 1 ora
       https://code.claude.com/docs/it/quickstart
       https://code.claude.com/docs/it/vs-code
       https://anthropic.skilljar.com/claude-code-in-action

    Domande → questo thread. [Owner] è il responsabile.
    ```
  </Tab>
</Tabs>

<h3 id="executive-sponsor-variant">
  Variante sponsor esecutivo
</h3>

Inviate questo dal vostro dirigente sponsor, come il CTO, CIO o SVP Engineering, con il loro nome e dal loro account. I lanci che escono con il nome di un dirigente vedono costantemente tassi di apertura più elevati e attivazione più veloce nella prima settimana rispetto allo stesso messaggio da un amministratore o da un team di strumenti. Segnala una priorità aziendale piuttosto che un esperimento opzionale.

Questa versione è deliberatamente ridotta a una richiesta: installatelo ed eseguitelo su un compito reale. Il lavoro del dirigente è far atterrare la richiesta; l'annuncio standard e `#claude-code` gestiscono il come.

<Tabs>
  <Tab title="Email">
    ```text theme={null}
    Subject: Una cosa che vorrei che ogni ingegnere provasse questa settimana

    Team,

    Abbiamo attivato Claude Code per tutto l'ingegneria. È un agente AI che
    funziona direttamente nel vostro terminale, sul vostro codebase effettivo,
    e i risultati iniziali dei team che lo stanno già utilizzando sono
    abbastanza forti che voglio che tutti lo usino questa settimana.

    Vi chiedo dieci minuti:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <your-repo>
        claude

    Poi dategli un compito reale: il bug che state rimandando, o "spiegami come
    funziona [module]".

    Questa è tutta la richiesta. [Owner name] e il team sono in #claude-code
    per qualsiasi cosa incontriate lungo il percorso.

    - [Exec Name]
      [Title]
    ```
  </Tab>

  <Tab title="Slack o Teams">
    ```markdown theme={null}
    📣 *Da [Exec Name]: una cosa da provare questa settimana*

    Abbiamo attivato *Claude Code* per tutto l'ingegneria. I risultati iniziali
    sono abbastanza forti che sto chiedendo a tutti di dargli dieci minuti su
    lavoro reale questa settimana.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd your-repo` →
    `claude` → dategli un compito reale.

    Questo è tutto. Domande → #claude-code.
    ```
  </Tab>
</Tabs>

<h3 id="pilot-group-variant">
  Variante gruppo pilota
</h3>

Usate per un rollout in fasi. Inviate solo al gruppo pilota.

```text theme={null}
Subject: Siete nel pilota di Claude Code

[Name / team],

Siete nella prima ondata di Claude Code presso [company]. Abbiamo scelto
questo gruppo perché lo metterete su problemi reali e ci direte la verità
al riguardo.

La richiesta: usatelo su almeno un compito reale questa settimana, poi
lasciate una nota in #claude-code-pilot coprendo cosa ha funzionato, cosa
era fastidioso e cosa vi ha sorpreso. Questo feedback decide come lo
implementiamo a tutti gli altri.

[Continuate con "Iniziate in due minuti" dall'annuncio standard]

Una cosa extra per i piloti: al vostro primo cambio multi-file, premete
Shift+Tab finché non vedete "plan". Claude esporrà esattamente cosa intende
fare prima di toccare un file. È il modo più veloce per calibrare quanto
fidarsi.
```

<h3 id="champion-recruitment-dm">
  DM di reclutamento di campioni
</h3>

Dopo il lancio, inviate un DM alle due o tre persone più attive in
`#claude-code`.

```text theme={null}
Ehi [name], i tuoi post su #claude-code stanno facendo più per l'adozione
del mio annuncio. Un paio di persone mi hanno detto che il tuo [thread /
screenshot] era il motivo per cui l'hanno effettivamente provato.

Vuoi renderlo semi-ufficiale? Poco sforzo: principalmente continua a
postare quello che stai postando, più il primo accesso alle nuove funzioni
e una linea diretta al team di Anthropic. Posso condividere un breve
playbook se sei interessato.
```

<h2 id="tips-and-tricks-campaign">
  Campagna di suggerimenti e trucchi
</h2>

Messaggi Slack o Teams pronti per incollare progettati per guidare l'attivazione delle funzioni dopo il lancio. Ognuno segue lo stesso schema: un gancio, il payoff, un prompt "provalo ora" e un link alla documentazione. Distribuiteli uno o due a settimana in `#claude-code`, o scegliete la manciata che corrisponde ai gap del vostro team. Stanno da soli senza ordine richiesto.

Copiate il corpo del messaggio da ogni blocco direttamente in Slack o Teams. Sostituite i `[segnaposti tra parentesi]` prima di inviare.

<h3 id="get-started">
  Iniziare
</h3>

**Scegliere il modello giusto**

```markdown theme={null}
🎯 *Suggerimento: Abbina il modello al momento*

Usare Opus per correggere un errore di battitura brucia il calcolo. Usare
Haiku per un refactor di 12 file è chiedere un rifacimento.

Claude Code funziona sugli stessi modelli dell'app Claude, e potete
cambiare a metà sessione. *Sonnet* è il cavallo da lavoro predefinito per
il lavoro di funzionalità quotidiana, bug, test e revisioni. Raggiungete
*Opus* su refactor su larga scala, debug complicati o qualsiasi cosa ad
alto rischio. Scendete a *Haiku* per domande rapide, formattazione e
modifiche meccaniche dove la velocità vince. *Fable 5* è il modello più
capace per i vostri compiti più difficili e lunghi; non è il default, quindi
selezionatelo con `/model fable`, e notate che i contenuti di cybersecurity
e biologia ricadono automaticamente su Opus.

*Provalo ora:* digitate `/model` e scegliete Sonnet se non l'avete già
fatto. È il default giusto per la maggior parte dei compiti.

📖 Configurazione del modello → https://code.claude.com/docs/it/model-config
```

| Modello | Migliore per                                                                                                                                                                           |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Fable 5 | I compiti più difficili e lunghi. Solo opt-in: selezionatelo con `/model fable`. I contenuti di cybersecurity o biologia [ricadono su Opus](/docs/it/model-config#automatic-model-fallback) |
| Opus    | Refactor su larga scala, debug complessi, decisioni architettoniche, cambiamenti ad alto rischio                                                                                       |
| Sonnet  | Lavoro di funzionalità quotidiana, correzioni di bug, test, documentazione, revisione del codice. Default consigliato.                                                                 |
| Haiku   | Domande rapide, formattazione, modifiche meccaniche, iterazione rapida                                                                                                                 |

**Vittorie rapide da provare per prime**

```markdown theme={null}
🚀 *Suggerimento: Tre cose da provare nei vostri primi 10 minuti*

Installato Claude Code ma non siete sicuri di cosa effettivamente chiedere?
Iniziate con le cose che vi hanno infastidito tutta la settimana.

  - Correggete qualcosa di fastidioso: "il test in [file] è instabile,
    scopri perché"
  - Orientatevi nel codice che non avete scritto: "spiegami come funziona
    [module]"
  - Sanity-check prima di spingere: "guarda il mio diff funzionante e
    dimmi cosa sembra rischioso"

Nessuno di questi ha bisogno di setup. Basta `cd` nel vostro repo ed
eseguite `claude`.

*Provalo ora:* scegliete il bug che state evitando e incollate il messaggio
di errore.

📖 Quickstart → https://code.claude.com/docs/it/quickstart
```

<h3 id="project-memory">
  Memoria del progetto
</h3>

**`/init` e CLAUDE.md**

```markdown theme={null}
📁 *Suggerimento: Smettete di re-spiegare il vostro repo ogni sessione*

Dire a Claude "usiamo pnpm, non npm" per la quinta volta? C'è una soluzione
una tantum.

Eseguite `/init` una volta per repo. Claude legge la struttura del vostro
progetto e scrive un file CLAUDE.md con i vostri comandi di build,
architettura e convenzioni. Ogni sessione futura in quel repo inizia da
questo file automaticamente. Mantenetelo sotto due schermate. È un foglio
di aiuto, non documentazione.

*Provalo ora:* aprite il vostro repo principale, eseguite `claude`, digitate
`/init`. Trenta secondi, ripaga ogni sessione dopo.

📖 CLAUDE.md e memoria del progetto → https://code.claude.com/docs/it/memory
```

**Riferimenti @**

```markdown theme={null}
📎 *Suggerimento: Smettete di incollare i contenuti dei file nella chat*

Copiare 200 righe di un componente nel vostro prompt in modo che Claude
possa "vederlo"? Non dovete.

Digitate `@` poi un percorso di file. Claude tira il file direttamente nel
contesto. Funziona anche per intere directory.

> gli stili in @src/components/Button.tsx sembrano sbagliati, controlla
> contro @docs/design-system.md

*Provalo ora:* digitate `@` poi Tab. L'autocomplete vi mostra ogni file a
portata.

📖 Riferimento ai file → https://code.claude.com/docs/it/common-workflows
```

<h3 id="control-and-safety">
  Controllo e sicurezza
</h3>

**Modalità di permesso**

```markdown theme={null}
🛡️ *Suggerimento: Un tasto tra "guarda ma non toccare" e "fallo e basta"*

A volte volete che Claude chieda prima di ogni modifica. A volte volete
solo che lo faccia. Non dovreste dover scegliere uno per sempre.

*Shift+Tab* cicla attraverso quanto Claude può fare senza chiedere: *Manual*
(il valore di impostazione `default`) chiede prima di ogni azione,
*acceptEdits* lascia fluire le modifiche ai file e i comandi comuni del
filesystem mentre controlla ancora prima di altri comandi shell, e *plan*
propone cambiamenti per la vostra approvazione prima che qualcosa sia
toccato. La modalità Plan è il costruttore di fiducia, quindi iniziate da
lì per qualsiasi cosa che tocchi più file.

*Provalo ora:* al vostro prossimo refactor, premete Shift+Tab finché non
vedete "plan", poi descrivete il cambiamento. Otterrete una proposta
completa prima che un singolo file si muova.

📖 Modalità di permesso → https://code.claude.com/docs/it/permissions
```

**Checkpointing e `/rewind`**

```markdown theme={null}
⏪ *Suggerimento: C'è un pulsante di annullamento per l'intera conversazione*

Claude è andato nella direzione sbagliata tre turni fa e ora lo state
districando? Non dovete aggiustare in avanti.

`/rewind` torna a un punto precedente nella conversazione, inclusi i
cambiamenti ai file che Claude ha fatto lungo il percorso. Il checkpointing
è automatico; non dovete impostare nulla.

*Provalo ora:* premete *Esc* due volte per aprire il menu di rewind, o
digitate `/rewind`. Scegliete il punto prima che le cose andassero di
traverso.

📖 Checkpointing → https://code.claude.com/docs/it/checkpointing
```

<h3 id="connect-your-tools">
  Connettete i vostri strumenti
</h3>

**Connettori MCP**

```markdown theme={null}
🔌 *Suggerimento: Lasciate che Claude legga il vostro issue tracker così
non dovete incollare i ticket*

Incollare i ticket Jira nel terminale sembra un passo indietro. Lo è.

Un file di configurazione (`.mcp.json` alla radice del vostro progetto)
collega Claude a GitHub, Jira, Linear, o qualsiasi tracker usiate. Poi
"qual è il problema di priorità più alta assegnato a me?" e "vai avanti e
correggilo" accadono nella stessa conversazione.

*Provalo ora:* chiedete a Claude "configura un connettore MCP per
[GitHub/Jira/Linear] in questo repo". Scriverà la configurazione per voi.

📖 Connettori MCP → https://code.claude.com/docs/it/mcp
```

<h3 id="automate-your-workflows">
  Automatizzate i vostri flussi di lavoro
</h3>

**Skills**

```markdown theme={null}
⚡ *Suggerimento: Trasformate quel prompt che continuate a riscrivere in un
comando*

Digitato "riassumi quello su cui ho lavorato oggi da git log, formattalo per
standup" tre volte questa settimana? Questo è un comando slash in attesa di
accadere.

Un file SKILL.md in `.claude/skills/<name>/` diventa un prompt riutilizzabile;
digitate `/<name>` per eseguirlo. Fatene uno la seconda volta che digitate
un prompt multi-step che avete digitato prima. Percorso più facile: chiedete
a Claude di farlo per voi.

*Provalo ora:* digitate "fammi uno skill /standup che riassume quello su cui
ho lavorato oggi da git log", poi eseguite `/standup` domani mattina.

📖 Skills → https://code.claude.com/docs/it/skills
```

**Hooks**

```markdown theme={null}
🔔 *Suggerimento: Ricevete una notifica quando il vostro refactor finisce*

Seduti alla vostra scrivania a guardare Claude lavorare attraverso un
compito lungo? Avete cose migliori da fare per questi otto minuti.

Gli hooks sono comandi shell che si attivano su eventi di Claude Code. Un
hook Stop che invia una notifica desktop significa che potete avviare un
refactor lungo, allontanarvi e ricevere una notifica nel momento in cui è
finito.

*Provalo ora:* chiedete a Claude "aggiungi un hook Stop che invia una
notifica desktop quando finisci". Scriverà lo script e lo collegherà.

📖 Guida agli hooks → https://code.claude.com/docs/it/hooks-guide
```

<h3 id="day-to-day-development">
  Sviluppo giorno per giorno
</h3>

**Screenshot e immagini**

```markdown theme={null}
📸 *Suggerimento: Smettete di descrivere la finestra di dialogo di errore.
Mostratela.*

Digitare "c'è una scatola rossa che dice qualcosa su un riferimento nullo e
sta puntando alla riga 47-ish"? Fate uno screenshot.

Trascinate uno screenshot direttamente nel terminale e Claude lo vede:
finestre di dialogo di errore, mockup UI, foto di lavagna, esportazioni
Figma. *Ctrl+V* incolla dagli appunti (usate Ctrl+V anche su macOS, non
Cmd+V).

*Provalo ora:* la prossima volta che qualcosa di visivo si rompe, fate uno
screenshot e incollatelo direttamente nel prompt. Poi digitate semplicemente
"cosa c'è di sbagliato qui?"

📖 Lavorare con le immagini → https://code.claude.com/docs/it/common-workflows
```

**Flussi di lavoro Git**

```markdown theme={null}
🌿 *Suggerimento: Passate tutta la cerimonia git*

La correzione ha richiesto 5 minuti. Il messaggio di commit, il ramo e la
descrizione PR ne hanno richiesti 15. Quel rapporto è sbagliato.

Claude gestisce l'intero flusso git: commit con messaggi convenzionali,
rami, PR con riepiloghi appropriati. Una richiesta: "correggi l'off-by-one,
commit con un messaggio di commit convenzionale e apri una PR." Rivedete il
lavoro di qualcun altro? Incollate l'URL della PR e chiedete a Claude di
spiegarvi il diff.

*Provalo ora:* dopo la vostra prossima correzione, invece di passare al
vostro client git, digitate semplicemente "commit questo con un buon
messaggio e apri una PR".

📖 Creazione di pull request → https://code.claude.com/docs/it/common-workflows
```

<h3 id="share-and-scale">
  Condividete e scalate
</h3>

**Plugins**

```markdown theme={null}
📦 *Suggerimento: Qualcuno probabilmente ha già costruito quella skill*

Stai per spendere un'ora a costruire un comando `/deploy`? Controlla se
esiste già.

Le skills vengono raggruppate e condivise come plugin. `/plugin` sfoglia
cosa è disponibile e installa in un passaggio. Cinque minuti di sfogliamento
possono risparmiare un'ora di costruzione.

*Provalo ora:* digitate `/plugin` e scorrete. Troverete almeno una cosa che
non sapevate di volere.

📖 Plugins → https://code.claude.com/docs/it/plugins
```

<h3 id="security-and-admin">
  Sicurezza e amministrazione
</h3>

**Architettura di sicurezza**

```markdown theme={null}
🔐 *Suggerimento: La risposta a "è sicuro?" per la prossima volta che vi
viene chiesto*

Qualcuno nel vostro team chiederà "aspetta, dove va il mio codice?" Ecco la
versione breve che potete incollare.

Permission-first per design. Ogni modifica di file, comando shell e chiamata
esterna è controllata dalla vostra approvazione. La CLI funziona nel vostro
terminale e parla direttamente all'API di Anthropic, senza server di terze
parti, e supporta sandboxing opzionale a livello di OS per i comandi shell.
Secondo il vostro piano Enterprise, Anthropic non utilizza il vostro codice
o i vostri prompt per addestrare i suoi modelli.

*Provalo ora:* salvate questi due link per la prossima volta che la domanda
viene posta. Rispondono alla maggior parte delle domande di revisione della
sicurezza.

📖 https://code.claude.com/docs/it/security
📖 https://code.claude.com/docs/it/data-usage
```

**Best practice**

```markdown theme={null}
✅ *Suggerimento: Le 4 abitudini che separano "l'ho provato una volta" da
"lo uso quotidianamente"*

La maggior parte delle persone che rimbalzano da Claude Code ne hanno
saltata una. La maggior parte delle persone che rimangono le hanno fatte
tutte quattro nella prima settimana.

  - Iniziate in modalità plan per qualsiasi cosa che tocchi più file
  - Eseguite /init presto; il contesto si compone
  - Rivedete i diff prima di fare il commit; Claude può essere
    confidentemente sbagliato
  - Verificate i cambiamenti che toccano percorsi critici; trattate come
    un junior acuto, non un oracolo

*Provalo ora:* se ne avete fatto solo uno o due, scegliete quello che state
perdendo e fatelo al vostro prossimo compito. Pubblicate cosa è cambiato in
#claude-code.

📖 Best practice → https://code.claude.com/docs/it/best-practices
```

<h2 id="quick-reference">
  Riferimento rapido
</h2>

<h3 id="faq-responses">
  Risposte FAQ
</h3>

Risposte in una riga per le domande che vi verranno poste più frequentemente.

| Domanda                            | Risposta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Funziona in VS Code?"             | Sì. C'è un'estensione VS Code e un plugin JetBrains con le stesse funzioni, incorporati nel vostro editor. [VS Code →](/docs/it/vs-code)                                                                                                                                                                                                                                                                                                                                                                                 |
| "Devo configurare qualcosa prima?" | No. Installate, poi eseguite `claude` in qualsiasi repo. Eseguite `/init` una volta e siete a posto. [Quickstart →](/docs/it/quickstart)                                                                                                                                                                                                                                                                                                                                                                                 |
| "Dove va il mio codice?"           | La CLI funziona nel vostro terminale e invia il contesto all'API di Anthropic per l'inferenza, senza server di terze parti. Secondo il vostro piano Enterprise, il vostro codice e i vostri prompt non vengono utilizzati per addestrare i modelli. [Data usage →](/docs/it/data-usage)                                                                                                                                                                                                                                  |
| "Può vedere il mio intero repo?"   | Legge quello a cui gli date accesso. Le letture di file all'interno della vostra directory di lavoro non richiedono prompt; i prompt di permesso controllano le modifiche, i comandi shell non in sola lettura e le letture di file-tool al di fuori di quella directory. Un insieme integrato di comandi shell in sola lettura come `ls` e `cat` viene eseguito senza richiedere il permesso; limitatelo con le [regole sandbox `denyRead`](/docs/it/sandboxing#filesystem-isolation). [Permissions →](/docs/it/permissions) |
| "Come è diverso da Copilot?"       | Copilot completa le righe. Claude Code è un agente che legge file, esegue comandi e fa modifiche multi-file. [Overview →](/docs/it/overview)                                                                                                                                                                                                                                                                                                                                                                             |
| "Cosa dovrei provare per primo?"   | Un bug che state rimandando perché è tedioso. "Il test in \[file] è instabile, scopri perché." [Quickstart →](/docs/it/quickstart)                                                                                                                                                                                                                                                                                                                                                                                       |

<h3 id="prompt-templates">
  Modelli di prompt
</h3>

Condividete questi prompt iniziali con gli ingegneri che hanno installato ma non sono sicuri di cosa chiedere. Ognuno è formulato nel modo in cui verrebbe digitato in una sessione reale; sostituite i pezzi tra parentesi con file dal vostro repo.

| Compito                    | Prompt                                                                             |
| -------------------------- | ---------------------------------------------------------------------------------- |
| Correggere un bug          | "i test in \[file] stanno fallendo, scopri perché e correggili"                    |
| Comprendere il codice      | "spiegami come funziona \[module], poi dimmi dov'è il punto di ingresso"           |
| Refactor sicuro            | "refactor \[module] a \[goal], usa la modalità plan così posso rivedere prima"     |
| Scrivere test              | "scrivi test per \[file] che coprano i casi limite intorno a \[scenario]"          |
| Revisione prima del commit | "guarda il mio diff funzionante e dimmi cosa sembra rischioso"                     |
| Aprire una PR              | "correggi \[issue], scrivi un commit convenzionale e apri una PR con un riassunto" |
| Fare una skill             | "fammi uno skill /ship che esegue test e lint prima del commit"                    |
| Debug di uno stack trace   | "ecco lo stack trace, trova la causa principale, non limitarti a coprirla"         |

<Tip>
  Claude Code viene spedito frequentemente. Verificate i dettagli specifici della versione rispetto alla [pagina iniziale della documentazione](/docs/it/overview) prima di distribuire internamente.
</Tip>
