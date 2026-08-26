> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estendi Claude Code

> Comprendi quando utilizzare CLAUDE.md, Skills, subagents, hooks, MCP e plugins.

Claude Code combina un modello che ragiona sul vostro codice con [strumenti integrati](/docs/it/how-claude-code-works#tools) per operazioni su file, ricerca, esecuzione e accesso web. Gli strumenti integrati coprono la maggior parte dei compiti di codifica. Questa guida copre il livello di estensione: funzionalità che aggiungete per personalizzare ciò che Claude conosce, collegarlo a servizi esterni e automatizzare i flussi di lavoro.

<Note>
  Per informazioni su come funziona il ciclo agentico principale, consultate [How Claude Code works](/docs/it/how-claude-code-works).
</Note>

**Siete nuovi a Claude Code?** Iniziate con [CLAUDE.md](/docs/it/memory) per le convenzioni del progetto, quindi aggiungete altre estensioni [quando emergono trigger specifici](#build-your-setup-over-time).

<h2 id="overview">
  Panoramica
</h2>

Le estensioni si collegano a diverse parti del ciclo agentico:

* **[CLAUDE.md](/docs/it/memory)** aggiunge contesto persistente che Claude vede in ogni sessione
* **[Skills](/docs/it/skills)** aggiungono conoscenze riutilizzabili e flussi di lavoro invocabili
* **[Code intelligence](/docs/it/tools-reference#lsp-tool-behavior)** collega Claude a un language server per la navigazione a livello di simbolo e gli errori di tipo in tempo reale
* **[MCP](/docs/it/mcp)** collega Claude a servizi e strumenti esterni
* **[Subagents](/docs/it/sub-agents)** eseguono i loro propri cicli in contesto isolato, restituendo riassunti
* **[Agent teams](/docs/it/agent-teams)** coordinano più sessioni indipendenti con compiti condivisi e messaggistica peer-to-peer
* **[Hooks](/docs/it/hooks-guide)** si attivano su eventi del ciclo di vita e possono eseguire uno script, una richiesta HTTP, un prompt o un subagent
* **[Plugins](/docs/it/plugins)** e **[marketplaces](/docs/it/plugin-marketplaces)** confezionano e distribuiscono queste funzionalità

[Skills](/docs/it/skills) sono l'estensione più flessibile. Una skill è un file markdown contenente conoscenze, flussi di lavoro o istruzioni. Potete invocare skills con un comando come `/deploy`, oppure Claude può caricarle automaticamente quando rilevante. Le skills possono essere eseguite nella vostra conversazione attuale o in un contesto isolato tramite subagents.

<h2 id="match-features-to-your-goal">
  Abbinate le funzionalità al vostro obiettivo
</h2>

Le funzionalità vanno dal contesto sempre attivo che Claude vede in ogni sessione, alle capacità su richiesta che voi o Claude potete invocare, all'automazione in background che viene eseguita su eventi specifici. La tabella seguente mostra ciò che è disponibile e quando ogni funzionalità ha senso.

| Funzionalità                                                   | Cosa fa                                                                  | Quando utilizzarla                                                                           | Esempio                                                                                                    |
| -------------------------------------------------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| **CLAUDE.md**                                                  | Contesto persistente caricato in ogni conversazione                      | Convenzioni del progetto, regole "fai sempre X"                                              | "Usa pnpm, non npm. Esegui i test prima di fare il commit."                                                |
| **Skill**                                                      | Istruzioni, conoscenze e flussi di lavoro che Claude può utilizzare      | Contenuto riutilizzabile, documenti di riferimento, compiti ripetibili                       | `/deploy` esegue la vostra checklist di distribuzione; skill di documentazione API con pattern di endpoint |
| **Subagent**                                                   | Contesto di esecuzione isolato che restituisce risultati riassunti       | Isolamento del contesto, compiti paralleli, worker specializzati                             | Compito di ricerca che legge molti file ma restituisce solo i risultati chiave                             |
| **[Agent teams](/docs/it/agent-teams)**                             | Coordinare più sessioni Claude Code indipendenti                         | Ricerca parallela, sviluppo di nuove funzionalità, debug con ipotesi concorrenti             | Generare revisori per controllare sicurezza, prestazioni e test simultaneamente                            |
| **[Code intelligence](/docs/it/tools-reference#lsp-tool-behavior)** | Navigazione e diagnostica del language server                            | Linguaggi tipizzati, grandi basi di codice dove grep è lento o impreciso                     | Saltare alla definizione di un simbolo invece di leggere l'intero file                                     |
| **MCP**                                                        | Collegamento a servizi esterni                                           | Dati o azioni esterne                                                                        | Interrogare il vostro database, inviare a Slack, controllare un browser                                    |
| **Hook**                                                       | Script, richiesta HTTP, prompt o subagent attivati da eventi             | Automazione che deve essere eseguita su ogni evento corrispondente                           | Eseguire ESLint dopo ogni modifica di file                                                                 |
| **[Artifact](/docs/it/artifacts)**                                  | Pubblicare l'output della sessione come pagina web privata e interattiva | Output che desiderate vedere o condividere visivamente piuttosto che come testo di terminale | Una timeline di incidente che si aggiorna mentre Claude indaga                                             |

**[Plugins](/docs/it/plugins)** sono il livello di confezionamento. Un plugin raggruppa skills, hooks, subagents e server MCP in una singola unità installabile. Le skills dei plugin hanno uno spazio dei nomi (come `/my-plugin:review`) in modo che più plugin possano coesistere. Utilizzate i plugin quando desiderate riutilizzare la stessa configurazione su più repository o distribuire ad altri tramite un **[marketplace](/docs/it/plugin-marketplaces)**.

<h3 id="build-your-setup-over-time">
  Costruite la vostra configurazione nel tempo
</h3>

Non è necessario configurare tutto in anticipo. Ogni funzionalità ha un trigger riconoscibile, e la maggior parte dei team le aggiunge approssimativamente in questo ordine:

| Trigger                                                                                       | Aggiungete                                                                                                   |
| :-------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| Claude sbaglia una convenzione o un comando due volte                                         | Aggiungetelo a [CLAUDE.md](/docs/it/memory)                                                                       |
| Continuate a digitare lo stesso prompt per avviare un compito                                 | Salvatelo come [skill](/docs/it/skills) invocabile dall'utente                                                    |
| Incollate lo stesso playbook o procedura multi-step in chat per la terza volta                | Catturatelo come [skill](/docs/it/skills)                                                                         |
| Continuate a copiare dati da una scheda del browser che Claude non può vedere                 | Collegate quel sistema come [server MCP](/docs/it/mcp)                                                            |
| Claude legge molti file per trovare dove un simbolo è definito o utilizzato                   | Installate un [plugin di code intelligence](/docs/it/discover-plugins#code-intelligence) per il vostro linguaggio |
| Un compito secondario inonda la vostra conversazione con output che non consulterete di nuovo | Indirizzatelo attraverso un [subagent](/docs/it/sub-agents)                                                       |
| Desiderate che qualcosa accada ogni volta senza chiedere                                      | Scrivete un [hook](/docs/it/hooks-guide)                                                                          |
| Un secondo repository ha bisogno della stessa configurazione                                  | Confezionatelo come [plugin](/docs/it/plugins)                                                                    |

Gli stessi trigger vi dicono quando aggiornare ciò che avete già. Un errore ripetuto o un commento di revisione ricorrente è una modifica di CLAUDE.md, non una correzione una tantum in chat. Un flusso di lavoro che continuate a modificare manualmente è una skill che ha bisogno di un'altra revisione.

<h3 id="compare-similar-features">
  Confrontate funzionalità simili
</h3>

Alcune funzionalità possono sembrare simili. Per una spiegazione più approfondita sulla scelta tra di esse, consultate [Steering Claude Code: when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more) sul blog. Ecco come distinguerle.

<Tabs>
  <Tab title="Skill vs Subagent">
    Skills e subagents risolvono problemi diversi:

    * **Skills** sono contenuti riutilizzabili che potete caricare in qualsiasi contesto
    * **Subagents** sono worker isolati che vengono eseguiti separatamente dalla vostra conversazione principale

    | Aspetto                                                      | Skill                                                    | Subagent                                                                            |
    | ------------------------------------------------------------ | -------------------------------------------------------- | ----------------------------------------------------------------------------------- |
    | **Cosa è**                                                   | Istruzioni, conoscenze o flussi di lavoro riutilizzabili | Worker isolato con il suo proprio contesto                                          |
    | **Vantaggio principale**                                     | Condividere contenuti tra contesti                       | Isolamento del contesto. Il lavoro avviene separatamente, solo il riassunto ritorna |
    | **Impatto della [finestra di contesto](/docs/it/context-window)** | Si aggiunge alla vostra finestra principale              | Utilizza una finestra separata con i suoi token di input e output                   |
    | **Migliore per**                                             | Materiale di riferimento, flussi di lavoro invocabili    | Compiti che leggono molti file, lavoro parallelo, worker specializzati              |

    **Le skills possono essere di riferimento o di azione.** Le skills di riferimento forniscono conoscenze che Claude utilizza durante la vostra sessione (come la vostra guida di stile API). Le skills di azione dicono a Claude di fare qualcosa di specifico (come `/deploy` che esegue il vostro flusso di lavoro di distribuzione).

    **Utilizzate un subagent** quando avete bisogno di isolamento del contesto o quando la vostra finestra di contesto si sta riempiendo. Il subagent potrebbe leggere dozzine di file o eseguire ricerche estese, ma la vostra conversazione principale riceve solo un riassunto. Poiché il lavoro del subagent non consuma il vostro contesto principale, questo è utile anche quando non avete bisogno che il lavoro intermedio rimanga visibile. I subagents personalizzati possono avere le loro proprie istruzioni e possono precaricare skills.

    **Possono combinarsi.** Un subagent può precaricare skills specifiche (campo `skills:`). Una skill può essere eseguita in contesto isolato utilizzando `context: fork`. Consultate [Skills](/docs/it/skills) per i dettagli.
  </Tab>

  <Tab title="CLAUDE.md vs Skill">
    Entrambi memorizzano istruzioni, ma si caricano diversamente e servono scopi diversi.

    | Aspetto                           | CLAUDE.md                      | Skill                                                 |
    | --------------------------------- | ------------------------------ | ----------------------------------------------------- |
    | **Si carica**                     | Ogni sessione, automaticamente | Su richiesta                                          |
    | **Può includere file**            | Sì, con importazioni `@path`   | Sì, con importazioni `@path`                          |
    | **Può attivare flussi di lavoro** | No                             | Sì, con `/<name>`                                     |
    | **Migliore per**                  | Regole "fai sempre X"          | Materiale di riferimento, flussi di lavoro invocabili |

    **Mettetelo in CLAUDE.md** se Claude dovrebbe sempre saperlo: convenzioni di codifica, comandi di build, struttura del progetto, regole "non fare mai X".

    **Mettetelo in una skill** se è materiale di riferimento di cui Claude ha bisogno a volte (documentazione API, guide di stile) o un flusso di lavoro che attivate con `/<name>` (deploy, review, release).

    **Regola pratica:** Mantenete CLAUDE.md sotto 200 righe. Se sta crescendo, spostate il contenuto di riferimento in skills o dividetelo in file [`.claude/rules/`](/docs/it/memory#organize-rules-with-claude%2Frules%2F).
  </Tab>

  <Tab title="CLAUDE.md vs Rules vs Skills">
    Tutti e tre memorizzano istruzioni, ma si caricano diversamente:

    | Aspetto          | CLAUDE.md                                 | `.claude/rules/`                                           | Skill                                                 |
    | ---------------- | ----------------------------------------- | ---------------------------------------------------------- | ----------------------------------------------------- |
    | **Si carica**    | Ogni sessione                             | Ogni sessione, o quando vengono aperti file corrispondenti | Su richiesta, quando invocato o rilevante             |
    | **Ambito**       | Intero progetto                           | Può essere limitato a percorsi di file                     | Specifico del compito                                 |
    | **Migliore per** | Convenzioni e comandi di build principali | Linee guida specifiche del linguaggio o della directory    | Materiale di riferimento, flussi di lavoro ripetibili |

    **Utilizzate CLAUDE.md** per istruzioni di cui ogni sessione ha bisogno: comandi di build, convenzioni di test, architettura del progetto.

    **Utilizzate rules** per mantenere CLAUDE.md focalizzato. Le rules con [frontmatter `paths`](/docs/it/memory#path-specific-rules) si caricano solo quando Claude lavora con file corrispondenti, risparmiando contesto.

    **Utilizzate skills** per contenuti di cui Claude ha bisogno solo a volte, come documentazione API o una checklist di distribuzione che attivate con `/<name>`.
  </Tab>

  <Tab title="Subagent vs Agent team">
    Entrambi parallelizzano il lavoro, ma sono architettonicamente diversi:

    * **Subagents** vengono eseguiti all'interno della vostra sessione e riportano i risultati al vostro contesto principale
    * **Agent teams** sono sessioni Claude Code indipendenti che comunicano tra loro

    | Aspetto             | Subagent                                                         | Agent team                                                       |
    | ------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------- |
    | **Contesto**        | Finestra di contesto propria; i risultati ritornano al chiamante | Finestra di contesto propria; completamente indipendente         |
    | **Comunicazione**   | Riporta i risultati solo all'agente principale                   | I compagni di squadra si messaggiano direttamente                |
    | **Coordinamento**   | L'agente principale gestisce tutto il lavoro                     | Elenco di compiti condiviso con auto-coordinamento               |
    | **Migliore per**    | Compiti focalizzati dove conta solo il risultato                 | Lavoro complesso che richiede discussione e collaborazione       |
    | **Costo del token** | Inferiore: i risultati sono riassunti al contesto principale     | Superiore: ogni compagno di squadra è un'istanza Claude separata |

    **Utilizzate un subagent** quando avete bisogno di un worker veloce e focalizzato: ricercare una domanda, verificare un'affermazione, rivedere un file. Il subagent fa il lavoro e restituisce un riassunto. La vostra conversazione principale rimane pulita.

    **Utilizzate un agent team** quando i compagni di squadra hanno bisogno di condividere i risultati, sfidare l'uno l'altro e coordinarsi in modo indipendente. Gli agent teams sono migliori per la ricerca con ipotesi concorrenti, la revisione del codice parallela e lo sviluppo di nuove funzionalità dove ogni compagno di squadra possiede un pezzo separato.

    **Punto di transizione:** Se state eseguendo subagents paralleli ma raggiungete limiti di contesto, o se i vostri subagents hanno bisogno di comunicare tra loro, gli agent teams sono il passo naturale successivo.

    <Note>
      Gli agent teams sono sperimentali e disabilitati per impostazione predefinita. Consultate [agent teams](/docs/it/agent-teams) per la configurazione e le limitazioni attuali.
    </Note>
  </Tab>

  <Tab title="MCP vs Skill">
    MCP collega Claude a servizi esterni. Le skills estendono ciò che Claude conosce, incluso come utilizzare efficacemente quei servizi.

    | Aspetto      | MCP                                                          | Skill                                                                                    |
    | ------------ | ------------------------------------------------------------ | ---------------------------------------------------------------------------------------- |
    | **Cosa è**   | Protocollo per il collegamento a servizi esterni             | Conoscenze, flussi di lavoro e materiale di riferimento                                  |
    | **Fornisce** | Accesso a strumenti e dati                                   | Conoscenze, flussi di lavoro, materiale di riferimento                                   |
    | **Esempi**   | Integrazione Slack, query di database, controllo del browser | Checklist di revisione del codice, flusso di lavoro di distribuzione, guida di stile API |

    Questi risolvono problemi diversi e funzionano bene insieme:

    **MCP** dà a Claude strumenti costruiti appositamente per un sistema esterno, con la connessione e l'autenticazione gestite dal server.

    **Skills** danno a Claude conoscenze su come utilizzare efficacemente quegli strumenti, più flussi di lavoro che potete attivare con `/<name>`. Una skill potrebbe includere lo schema del database del vostro team e i pattern di query, o un flusso di lavoro `/post-to-slack` con le regole di formattazione dei messaggi del vostro team.

    Esempio: Un server MCP collega Claude al vostro database. Una skill insegna a Claude il vostro modello di dati, i pattern di query comuni e quali tabelle utilizzare per diversi compiti.
  </Tab>

  <Tab title="Hook vs Skill">
    Un hook si attiva su un evento del ciclo di vita; una skill viene caricata nel contesto affinché Claude la applichi.

    | Aspetto                | Hook                                                                                  | Skill                                                                                      |
    | ---------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
    | **Viene eseguito**     | Un comando shell, una richiesta HTTP, un prompt LLM o un subagent                     | Istruzioni che Claude legge e segue                                                        |
    | **Attivato da**        | [Eventi del ciclo di vita](/docs/it/hooks#hook-events) come `PostToolUse` o `SessionStart` | Voi digitate `/<name>`, o Claude abbina la descrizione al vostro compito                   |
    | **Determinismo**       | Si attiva sempre sul suo evento; il trigger è garantito                               | Claude interpreta le istruzioni; il risultato può variare                                  |
    | **Costo del contesto** | Zero a meno che l'hook non restituisca output                                         | La descrizione si carica ogni sessione; il contenuto completo si carica quando utilizzato  |
    | **Migliore per**       | Linting dopo le modifiche, blocco di comandi non sicuri, logging, notifiche           | Flussi di lavoro che richiedono ragionamento, materiale di riferimento, compiti multi-step |

    **Utilizzate un hook** quando l'azione deve accadere nello stesso modo ogni volta e non ha bisogno che Claude pensi. Ad esempio: formattazione al salvataggio, rifiuto di `rm -rf /`, invio di un messaggio Slack quando una sessione termina.

    **Utilizzate una skill** quando Claude dovrebbe decidere come applicare i passaggi, o quando il contenuto è conoscenza piuttosto che uno script. Ad esempio: una checklist `/release`, la vostra guida di stile API, un playbook di debug.

    **Mettete i guardrail negli hooks.** Un'istruzione come "non modificare mai `.env`" in CLAUDE.md o una skill è una richiesta, non una garanzia. Un hook `PreToolUse` che blocca la modifica è un'applicazione. Se una regola deve valere ogni volta, fatene un hook piuttosto che un'istruzione di prompt.

    **L'output dell'hook entra nel contesto.** Un hook `PostToolUse` che esegue il vostro linter alimenta i risultati come testo che Claude legge; una skill `/fix-lint` dice a Claude come risolverli.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  Comprendete come le funzionalità si stratificano
</h3>

Le funzionalità possono essere definite a più livelli: a livello di utente, per progetto, tramite plugins o tramite politiche gestite. Potete anche annidare file CLAUDE.md in sottodirectory o posizionare skills in pacchetti specifici di un monorepo. Quando la stessa funzionalità esiste a più livelli, ecco come si stratificano:

* **I file CLAUDE.md** sono additivi: tutti i livelli contribuiscono contenuti al contesto di Claude simultaneamente. I file dalla vostra directory di lavoro e sopra si caricano all'avvio; le sottodirectory si caricano mentre lavorate in esse. Quando le istruzioni entrano in conflitto, Claude usa il giudizio per riconciliarle, con istruzioni più specifiche che tipicamente hanno la precedenza. Consultate [come i file CLAUDE.md si caricano](/docs/it/memory#how-claude-md-files-load).
* **Skills e subagents** si sovrascrivono per nome: quando lo stesso nome esiste a più livelli, una definizione vince in base alla priorità (gestito > utente > progetto per skills; gestito > flag CLI > progetto > utente > plugin per subagents). Le skills dei plugin sono [con spazio dei nomi](/docs/it/plugins#add-skills-to-your-plugin) per evitare conflitti. Consultate [scoperta delle skills](/docs/it/skills#where-skills-live) e [ambito del subagent](/docs/it/sub-agents#choose-the-subagent-scope).
* **I server MCP** si sovrascrivono per nome: locale > progetto > utente. Consultate [ambito MCP](/docs/it/mcp#scope-hierarchy-and-precedence).
* **Hooks** si uniscono: tutti gli hooks registrati si attivano per i loro eventi corrispondenti indipendentemente dalla fonte. Consultate [hooks](/docs/it/hooks-guide).

<h3 id="combine-features">
  Combinate le funzionalità
</h3>

Ogni estensione risolve un problema diverso: CLAUDE.md gestisce il contesto sempre attivo, le skills gestiscono conoscenze e flussi di lavoro su richiesta, MCP gestisce connessioni esterne, i subagents gestiscono l'isolamento e gli hooks gestiscono l'automazione. Le configurazioni reali le combinano in base al vostro flusso di lavoro.

Ad esempio, potreste utilizzare CLAUDE.md per convenzioni del progetto, una skill per il vostro flusso di lavoro di distribuzione, MCP per connettervi al vostro database e un hook per eseguire il linting dopo ogni modifica. Ogni funzionalità gestisce ciò che fa meglio.

| Pattern                | Come funziona                                                                                                | Esempio                                                                                              |
| ---------------------- | ------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- |
| **Skill + MCP**        | MCP fornisce la connessione; una skill insegna a Claude come utilizzarla bene                                | MCP si connette al vostro database, una skill documenta lo schema e i pattern di query               |
| **Skill + Subagent**   | Una skill genera subagents per il lavoro parallelo                                                           | La skill `/audit` avvia subagents di sicurezza, prestazioni e stile che lavorano in contesto isolato |
| **CLAUDE.md + Skills** | CLAUDE.md contiene regole sempre attive; le skills contengono materiale di riferimento caricato su richiesta | CLAUDE.md dice "segui le nostre convenzioni API," una skill contiene la guida di stile API completa  |
| **Hook + MCP**         | Un hook attiva azioni esterne tramite MCP                                                                    | L'hook post-modifica invia una notifica Slack quando Claude modifica file critici                    |

<h2 id="understand-context-costs">
  Comprendete i costi del contesto
</h2>

Ogni funzionalità che aggiungete consuma parte del contesto di Claude. Troppo può riempire la vostra finestra di contesto, ma può anche aggiungere rumore che rende Claude meno efficace; le skills potrebbero non attivarsi correttamente, o Claude potrebbe perdere traccia delle vostre convenzioni. Comprendere questi compromessi vi aiuta a costruire una configurazione efficace. Per una visualizzazione interattiva di come queste funzionalità si combinano in una sessione in esecuzione, consultate [Esplora la finestra di contesto](/docs/it/context-window).

<h3 id="context-cost-by-feature">
  Costo del contesto per funzionalità
</h3>

Ogni funzionalità ha una strategia di caricamento e un costo di contesto diversi:

| Funzionalità          | Quando si carica                          | Cosa si carica                                                  | Costo del contesto                                          |
| --------------------- | ----------------------------------------- | --------------------------------------------------------------- | ----------------------------------------------------------- |
| **CLAUDE.md**         | Inizio della sessione                     | Contenuto completo                                              | Ogni richiesta                                              |
| **Skills**            | Inizio della sessione + quando utilizzate | Descrizioni all'inizio, contenuto completo quando utilizzate    | Basso (descrizioni ogni richiesta)\*                        |
| **Server MCP**        | Inizio della sessione                     | Nomi degli strumenti; schemi completi su richiesta              | Basso fino a quando non viene utilizzato uno strumento      |
| **Code intelligence** | Dopo le modifiche ai file e su richiesta  | Diagnostica dopo le modifiche; posizioni dei simboli su ricerca | Basso; riduce le letture di file altrove                    |
| **Subagents**         | Quando generati                           | Contesto fresco con skills specificate                          | Isolato dalla sessione principale                           |
| **Hooks**             | Al trigger                                | Niente (viene eseguito esternamente)                            | Zero, a meno che l'hook non restituisca contesto aggiuntivo |

\*Per impostazione predefinita, le descrizioni delle skills si caricano all'inizio della sessione in modo che Claude possa decidere quando utilizzarle. Impostate `disable-model-invocation: true` nel frontmatter di una skill per nasconderla completamente a Claude fino a quando non la invocate manualmente. Questo riduce il costo del contesto a zero per le skills che invocate solo voi. Per una skill che non avete scritto, impostate [`skillOverrides`](/docs/it/skills#override-skill-visibility-from-settings) nelle impostazioni per fare lo stesso senza modificare il suo file.

<h3 id="understand-how-features-load">
  Comprendete come le funzionalità si caricano
</h3>

Ogni funzionalità si carica in diversi punti della vostra sessione. Le schede seguenti spiegano quando ogni funzionalità si carica e cosa entra nel contesto.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" alt="Caricamento del contesto: CLAUDE.md si carica all'inizio della sessione e rimane in ogni richiesta. I nomi degli strumenti MCP si caricano all'inizio con schemi completi rinviati fino all'uso. Le skills caricano descrizioni all'inizio, contenuto completo all'invocazione. I subagents ottengono contesto isolato. Gli hooks vengono eseguiti esternamente." width="720" height="382" data-path="images/context-loading.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **Quando:** Inizio della sessione

    **Cosa si carica:** Contenuto completo di tutti i file CLAUDE.md (livelli gestito, utente e progetto).

    **Eredità:** Claude legge i file CLAUDE.md dalla vostra directory di lavoro fino alla radice e scopre quelli annidati nelle sottodirectory mentre accede a quei file. Consultate [Come i file CLAUDE.md si caricano](/docs/it/memory#how-claude-md-files-load) per i dettagli.

    <Tip>Mantenete CLAUDE.md sotto 200 righe. Spostate il materiale di riferimento in skills, che si caricano su richiesta.</Tip>
  </Tab>

  <Tab title="Skills">
    Le skills sono capacità extra nel toolkit di Claude. Possono essere materiale di riferimento (come una guida di stile API) o flussi di lavoro invocabili che attivate con `/<name>` (come `/deploy`). Claude Code include [skills raggruppate](/docs/it/commands) come `/code-review`, `/batch` e `/debug` che funzionano subito. Potete anche crearne di vostre. Claude utilizza le skills quando appropriato, oppure potete invocarne una direttamente.

    **Quando:** Dipende dalla configurazione della skill. Per impostazione predefinita, le descrizioni si caricano all'inizio della sessione e il contenuto completo si carica quando utilizzate. Per le skills solo utente (`disable-model-invocation: true`), niente si carica fino a quando non le invocate.

    **Cosa si carica:** Per le skills invocabili dal modello, Claude vede nomi e descrizioni in ogni richiesta. Quando invocate una skill con `/<name>` o Claude la carica automaticamente, il contenuto completo si carica nella vostra conversazione.

    **Come Claude sceglie le skills:** Claude abbina il vostro compito alle descrizioni delle skills per decidere quali sono rilevanti. Se le descrizioni sono vaghe o si sovrappongono, Claude potrebbe caricare la skill sbagliata o perderne una che aiuterebbe. Per dire a Claude di utilizzare una skill specifica, invocatela con `/<name>`. Le skills con `disable-model-invocation: true` sono invisibili a Claude fino a quando non le invocate.

    **Costo del contesto:** Basso fino a quando non vengono utilizzate. Le skills solo utente hanno costo zero fino a quando non vengono invocate.

    **Nei subagents:** Le skills funzionano diversamente nei subagents. Invece del caricamento su richiesta, le skills elencate nel campo `skills:` dell'agente vengono completamente precaricate nel suo contesto all'avvio. I subagents possono comunque scoprire e invocare skills di progetto, utente e plugin non elencate attraverso lo strumento Skill.

    <Tip>Utilizzate `disable-model-invocation: true` per le skills con effetti collaterali. Questo risparmia contesto e assicura che solo voi le attiviate.</Tip>
  </Tab>

  <Tab title="Server MCP">
    **Quando:** Inizio della sessione.

    **Cosa si carica:** Nomi degli strumenti dai server connessi. Gli schemi JSON completi rimangono rinviati fino a quando Claude non ha bisogno di uno strumento specifico.

    **Costo del contesto:** [Ricerca degli strumenti](/docs/it/mcp#scale-with-mcp-tool-search) è abilitata per impostazione predefinita, quindi gli strumenti MCP inattivi consumano contesto minimo.

    <Tip>Eseguite `/mcp` per vedere lo stato della connessione e i costi dei token per server. Claude Code [si riconnette automaticamente ai server remoti](/docs/it/mcp#automatic-reconnection) se si disconnettono, e potete disconnettere i server che non state utilizzando attivamente.</Tip>
  </Tab>

  <Tab title="Code intelligence">
    **Quando:** Dopo le modifiche ai file e su richiesta quando Claude naviga il codice.

    **Cosa si carica:** Errori di tipo e avvisi dopo ogni modifica ai file. Informazioni di definizione, riferimento e tipo quando Claude cerca un simbolo.

    **Costo del contesto:** Basso. Le ricerche di simboli spesso sostituiscono letture di file ampie, quindi l'uso netto del contesto può diminuire.

    <Tip>Lo strumento LSP è inattivo fino a quando non installate un [plugin di code intelligence](/docs/it/discover-plugins#code-intelligence) per il vostro linguaggio.</Tip>
  </Tab>

  <Tab title="Subagents">
    **Quando:** Su richiesta, quando voi o Claude ne generate uno per un compito.

    **Cosa si carica:** Contesto fresco e isolato contenente:

    * Il prompt di sistema dell'agente, non il prompt di sistema completo di Claude Code
    * Contenuto completo delle skills elencate nel campo `skills:` dell'agente
    * CLAUDE.md e stato git, eccetto gli agenti Explore e Plan integrati [che omettono entrambi](/docs/it/sub-agents#what-loads-at-startup)
    * Qualsiasi contesto che l'agente principale passa nel prompt

    **Costo del contesto:** Isolato dalla sessione principale. I subagents non ereditano la vostra cronologia di conversazione o le skills invocate.

    <Tip>Utilizzate i subagents per il lavoro che non ha bisogno del vostro contesto di conversazione completo. Il loro isolamento previene il gonfiore della vostra sessione principale.</Tip>
  </Tab>

  <Tab title="Hooks">
    **Quando:** Al trigger. Gli hooks si attivano su eventi del ciclo di vita specifici come esecuzione dello strumento, confini della sessione, invio del prompt, richieste di autorizzazione e compattazione. Consultate [Hooks](/docs/it/hooks) per l'elenco completo.

    **Cosa si carica:** Niente per impostazione predefinita. Gli hooks vengono eseguiti al di fuori della conversazione principale.

    **Costo del contesto:** Zero, a meno che l'hook non restituisca output che viene aggiunto come messaggi alla vostra conversazione.

    <Tip>Gli hooks sono ideali per effetti collaterali (linting, logging) che non hanno bisogno di influenzare il contesto di Claude.</Tip>
  </Tab>
</Tabs>

<h2 id="learn-more">
  Scopri di più
</h2>

Ogni funzionalità ha la sua propria guida con istruzioni di configurazione, esempi e opzioni di configurazione.

<CardGroup cols={2}>
  <Card title="CLAUDE.md" icon="file-lines" href="/docs/it/memory">
    Memorizzate il contesto del progetto, le convenzioni e le istruzioni
  </Card>

  <Card title="Skills" icon="brain" href="/docs/it/skills">
    Date a Claude competenze di dominio e flussi di lavoro riutilizzabili
  </Card>

  <Card title="Subagents" icon="users" href="/docs/it/sub-agents">
    Delegate il lavoro a contesto isolato
  </Card>

  <Card title="Agent teams" icon="network" href="/docs/it/agent-teams">
    Coordinate più sessioni che lavorano in parallelo
  </Card>

  <Card title="MCP" icon="plug" href="/docs/it/mcp">
    Collegate Claude a servizi esterni
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/it/hooks-guide">
    Automatizzate i flussi di lavoro con gli hooks
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/it/plugins">
    Confezionate e condividete set di funzionalità
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/it/plugin-marketplaces">
    Ospitate e distribuite raccolte di plugin
  </Card>
</CardGroup>
