> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrare team di sessioni Claude Code

> Coordinare più istanze di Claude Code che lavorano insieme come un team, con attività condivise, messaggistica tra agenti e gestione centralizzata.

<Warning>
  I team di agenti sono sperimentali e disabilitati per impostazione predefinita. Abilitateli aggiungendo `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` al vostro [settings.json](/docs/it/settings) o all'ambiente. Senza quella variabile, nessun team viene configurato all'avvio della sessione, nessuna directory di team viene scritta, e Claude non genera o propone compagni di team. I team di agenti hanno [limitazioni note](#limitations) relative alla ripresa della sessione, al coordinamento delle attività e al comportamento di arresto.
</Warning>

I team di agenti vi permettono di coordinare più istanze di Claude Code che lavorano insieme. Una sessione agisce come il team lead, coordinando il lavoro, assegnando attività e sintetizzando i risultati. I compagni di team lavorano indipendentemente, ognuno nel proprio context window, e comunicano direttamente tra loro.

A differenza dei [subagents](/docs/it/sub-agents), che vengono eseguiti all'interno di una singola sessione e possono solo riferire al main agent, potete anche interagire direttamente con i singoli compagni di team senza passare attraverso il lead.

<Note>
  Questa pagina descrive i team di agenti a partire da v2.1.178. Con `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` impostato, la generazione di un compagno di team non richiede più un passaggio di configurazione, e la pulizia avviene automaticamente quando la sessione esce. Prima di v2.1.178, chiedevate a Claude di creare e denominare un team per primo, e Claude utilizzava i tool `TeamCreate` e `TeamDelete` per configurarlo e rimuoverlo. Entrambi i tool non esistono più. L'input `team_name` sul tool Agent è accettato ma ignorato, e il campo `team_name` in `TaskCreated`, `TaskCompleted`, e `TeammateIdle` [hook payloads](/docs/it/hooks#taskcreated) porta il nome derivato dalla sessione ed è deprecato.
</Note>

<h2 id="when-to-use-agent-teams">
  Quando utilizzare i team di agenti
</h2>

I team di agenti sono più efficaci per attività in cui l'esplorazione parallela aggiunge valore reale. Consultate gli [esempi di casi d'uso](#use-case-examples) per scenari completi. I casi d'uso più forti sono:

* **Ricerca e revisione**: più compagni di team possono investigare diversi aspetti di un problema simultaneamente, quindi condividere e mettere in discussione i risultati reciproci
* **Nuovi moduli o funzionalità**: i compagni di team possono possedere ciascuno un pezzo separato senza interferire l'uno con l'altro
* **Debug con ipotesi concorrenti**: i compagni di team testano diverse teorie in parallelo e convergono sulla risposta più velocemente
* **Coordinamento tra livelli**: modifiche che si estendono su frontend, backend e test, ciascuno posseduto da un diverso compagno di team

I team di agenti aggiungono overhead di coordinamento e utilizzano significativamente più token di una singola sessione. Funzionano meglio quando i compagni di team possono operare indipendentemente. Per attività sequenziali, modifiche dello stesso file o lavoro con molte dipendenze, una singola sessione o i [subagents](/docs/it/sub-agents) sono più efficaci.

<h3 id="compare-with-subagents">
  Confronto con i subagents
</h3>

Sia i team di agenti che i [subagents](/docs/it/sub-agents) vi permettono di parallelizzare il lavoro, ma operano diversamente. Scegliete in base al fatto che i vostri worker debbano comunicare tra loro:

<Frame caption="I subagents riportano solo i risultati al main agent e non si parlano mai. Nei team di agenti, i compagni di team condividono un elenco di attività, rivendicano il lavoro e comunicano direttamente tra loro.">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Diagramma che confronta le architetture di subagent e team di agenti. I subagents vengono generati dal main agent, svolgono il lavoro e riportano i risultati. I team di agenti si coordinano attraverso un elenco di attività condiviso, con i compagni di team che comunicano direttamente tra loro." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Diagramma che confronta le architetture di subagent e team di agenti. I subagents vengono generati dal main agent, svolgono il lavoro e riportano i risultati. I team di agenti si coordinano attraverso un elenco di attività condiviso, con i compagni di team che comunicano direttamente tra loro." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|                    | Subagents                                                        | Team di agenti                                                |
| :----------------- | :--------------------------------------------------------------- | :------------------------------------------------------------ |
| **Context**        | Context window proprio; i risultati tornano al chiamante         | Context window proprio; completamente indipendente            |
| **Comunicazione**  | Riportano i risultati solo al main agent                         | I compagni di team si messaggiano direttamente                |
| **Coordinamento**  | Il main agent gestisce tutto il lavoro                           | Elenco di attività condiviso con auto-coordinamento           |
| **Migliore per**   | Attività focalizzate dove conta solo il risultato                | Lavoro complesso che richiede discussione e collaborazione    |
| **Costo in token** | Inferiore: i risultati sono sintetizzati nel contesto principale | Superiore: ogni compagno di team è un'istanza Claude separata |

Utilizzate i subagents quando avete bisogno di worker veloci e focalizzati che riportino indietro. Utilizzate i team di agenti quando i compagni di team devono condividere i risultati, mettersi in discussione e coordinarsi autonomamente.

<h2 id="enable-agent-teams">
  Abilitare i team di agenti
</h2>

I team di agenti sono disabilitati per impostazione predefinita. Abilitateli impostando la variabile di ambiente `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` a `1`, sia nell'ambiente della shell che tramite [settings.json](/docs/it/settings):

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

<h2 id="start-your-first-agent-team">
  Avviare il vostro primo team di agenti
</h2>

Dopo aver abilitato i team di agenti, descrivete il compito e i compagni di team che desiderate in linguaggio naturale. Claude li genera e coordina il lavoro in base al vostro prompt.

Questo esempio funziona bene perché i tre ruoli sono indipendenti e possono esplorare il problema senza aspettarsi l'uno l'altro:

```text theme={null}
Sto progettando uno strumento CLI che aiuta gli sviluppatori a tracciare i commenti TODO
nel loro codebase. Genera tre compagni di team per esplorare questo da diversi angoli: uno
su UX, uno su architettura tecnica, uno che gioca l'avvocato del diavolo.
```

Da lì, Claude popola un [elenco di attività condiviso](/docs/it/interactive-mode#task-list), genera compagni di team per ogni prospettiva, li fa esplorare il problema, e sintetizza i risultati al termine.

Il terminale del lead elenca i compagni di team nel pannello dell'agente sotto l'input del prompt. Dal pannello:

* **Frecce su e giù**: selezionare un compagno di team
* **Invio**: aprire la trascrizione del compagno di team selezionato e messaggiargli direttamente
* **Escape**: interrompere il turno corrente del compagno di team selezionato

A partire dalla v2.1.199, la riga di un compagno di team inattivo rimane nel pannello mentre qualsiasi compagno di team o subagente è ancora al lavoro, quindi potete selezionarlo per rivedere la sua trascrizione o assegnargli più lavoro. Una volta che ogni agente nel pannello è inattivo, le righe inattive si nascondono dopo 30 secondi e riappaiono al turno successivo del compagno di team; il compagno di team rimane in esecuzione e indirizzabile mentre nascosto. Nella v2.1.181 fino alla v2.1.198, una riga inattiva si nascondeva 30 secondi dopo la fine del suo turno, anche mentre altri compagni di team erano ancora al lavoro; le righe inattive non sono nascoste nelle versioni precedenti alla v2.1.181.

Quando più di tre compagni di team sono inattivi contemporaneamente, le righe oltre le prime tre si comprimono in una singola riga che conta i compagni di team compressi, come `2 idle agents` quando cinque sono inattivi. Selezionatela e premete Invio per espandere le righe compresse, oppure premete Esc per comprimerle di nuovo. I compagni di team al lavoro, i compagni di team non riusciti, e il compagno di team che state visualizzando mantengono sempre le loro righe proprie.

Se desiderate che ogni compagno di team sia in un riquadro diviso proprio, consultate [Scegliere una modalità di visualizzazione](#choose-a-display-mode).

<h2 id="control-your-agent-team">
  Controllare il vostro team di agenti
</h2>

Dite al lead cosa desiderate in linguaggio naturale. Gestisce il coordinamento del team, l'assegnazione delle attività e la delega in base alle vostre istruzioni.

<h3 id="choose-a-display-mode">
  Scegliere una modalità di visualizzazione
</h3>

I team di agenti supportano due modalità di visualizzazione:

* **In-process**: tutti i compagni di team vengono eseguiti all'interno del vostro terminale principale. Utilizzate i tasti freccia su e giù nel pannello dell'agente per selezionare un compagno di team, quindi premete Invio per visualizzarlo e digitate per messaggiarlo direttamente. Funziona in qualsiasi terminale, nessuna configurazione extra richiesta.
* **Split panes**: ogni compagno di team ottiene il proprio riquadro. Potete vedere l'output di tutti contemporaneamente e fare clic su un riquadro per interagire direttamente. Richiede tmux o iTerm2.

<Note>
  `tmux` ha limitazioni note su certi sistemi operativi e tradizionalmente funziona meglio su macOS. Utilizzare `tmux -CC` in iTerm2 è il punto di ingresso suggerito in `tmux`.
</Note>

L'impostazione predefinita è `"in-process"`. Prima della versione 2.1.179 l'impostazione predefinita era `"auto"`, quindi le sessioni aggiornate che in precedenza aprivano split panes ora rimangono in un terminale a meno che non impostiate esplicitamente la modalità. Impostate `"auto"` per abilitare split panes quando state già eseguendo all'interno di una sessione tmux o il vostro terminale è iTerm2 con la CLI `it2` installata, altrimenti ricadete in in-process. L'impostazione `"tmux"` abilita la modalità split-pane e rileva automaticamente se utilizzare tmux o iTerm2 in base al vostro terminale.

A partire dalla versione 2.1.186, impostate `"iterm2"` per utilizzare esplicitamente i split panes nativi di iTerm2. Questa modalità richiede la [CLI `it2`](https://github.com/mkusaka/it2) e mostra un errore con il comando di installazione se `it2` è mancante. Il prompt di configurazione che offre di installare `it2` o passare a tmux appare sotto `"auto"` o `"tmux"` quando il vostro terminale è iTerm2 e tmux è disponibile come fallback.

Per sovrascrivere l'impostazione predefinita, impostate [`teammateMode`](/docs/it/settings#available-settings) in `~/.claude/settings.json`:

```json theme={null}
{
  "teammateMode": "auto"
}
```

Per impostare la modalità per una singola sessione, passatela come flag:

```bash theme={null}
claude --teammate-mode auto
```

La modalità split-pane richiede [tmux](https://github.com/tmux/tmux/wiki) o iTerm2 con la CLI [`it2`](https://github.com/mkusaka/it2). Per installare manualmente:

* **tmux**: installate tramite il gestore di pacchetti del vostro sistema. Consultate il [wiki di tmux](https://github.com/tmux/tmux/wiki/Installing) per istruzioni specifiche della piattaforma.
* **iTerm2**: installate la CLI [`it2`](https://github.com/mkusaka/it2), quindi abilitate l'API Python in **iTerm2 → Settings → General → Magic → Enable Python API**.

<h3 id="specify-teammates-and-models">
  Specificare compagni di team e modelli
</h3>

Claude decide il numero di compagni di team da generare in base al vostro compito, oppure potete specificare esattamente quello che desiderate:

```text theme={null}
Spawn 4 teammates to refactor these modules in parallel. Use Sonnet for
each teammate.
```

I compagni di team non ereditano la selezione `/model` del lead per impostazione predefinita. Per modificare il modello utilizzato quando il prompt non ne specifica uno, impostate **Default teammate model** in `/config`. Scegliete **Default (leader's model)** per fare in modo che i compagni di team seguano il modello attuale del lead.

I compagni di team ereditano il [livello di sforzo](/docs/it/model-config#adjust-effort-level) del lead. In modalità split-pane questo si applica dalla versione 2.1.186; le versioni precedenti non passavano lo sforzo della sessione del lead ai compagni di team in split-pane.

<h3 id="require-plan-approval-for-teammates">
  Richiedere l'approvazione del piano per i compagni di team
</h3>

Per compiti complessi o rischiosi, potete richiedere ai compagni di team di pianificare prima di implementare. Il compagno di team lavora in modalità piano di sola lettura fino a quando il lead approva il loro approccio:

```text theme={null}
Spawn an architect teammate to refactor the authentication module.
Require plan approval before they make any changes.
```

Quando un compagno di team finisce di pianificare, invia una richiesta di approvazione del piano al lead. Il lead esamina il piano e lo approva o lo rifiuta con feedback. Se rifiutato, il compagno di team rimane in modalità piano, rivede in base al feedback e lo riinvia. Una volta approvato, il compagno di team esce dalla modalità piano e inizia l'implementazione.

Il lead prende decisioni di approvazione autonomamente. Per influenzare il giudizio del lead, fornitegli criteri nel vostro prompt, come "approva solo i piani che includono la copertura dei test" o "rifiuta i piani che modificano lo schema del database".

<h3 id="talk-to-teammates-directly">
  Parlare direttamente con i compagni di team
</h3>

Ogni compagno di team è una sessione Claude Code completa e indipendente. Potete messaggiare qualsiasi compagno di team direttamente per fornire istruzioni aggiuntive, fare domande di follow-up o reindirizzare il loro approccio.

* **Modalità in-process**: utilizzate i tasti freccia su e giù nel pannello dell'agente per selezionare un compagno di team, quindi premete Invio per visualizzare la sua sessione e digitate per inviargli un messaggio. Premete `x` su un compagno di team selezionato per fermarlo. Premete Ctrl+T per attivare/disattivare l'elenco delle attività.
* **Modalità split-pane**: fate clic nel riquadro di un compagno di team per interagire direttamente con la sua sessione. Ogni compagno di team ha una visualizzazione completa del proprio terminale.

Mentre state visualizzando un compagno di team in-process, il testo semplice e le [skills](/docs/it/skills) vanno a quel compagno di team, ma i comandi integrati vengono comunque eseguiti nella sessione del lead.

Il modello e la modalità veloce di un compagno di team sono fissi quando viene generato, quindi `/model` e `/fast` cambiano solo le impostazioni del lead. A partire dalla versione 2.1.199, digitando uno di questi comandi mentre si visualizza un compagno di team viene mostrato un avviso che il cambiamento si applica al lead; le versioni precedenti lo applicavano al lead senza indicazione. `/effort` si applica comunque ai turni successivi del compagno di team visualizzato, perché i compagni di team seguono il [livello di sforzo](/docs/it/model-config#adjust-effort-level) del lead.

<h3 id="assign-and-claim-tasks">
  Assegnare e rivendicare attività
</h3>

L'elenco di attività condiviso coordina il lavoro nel team. Il lead crea attività e i compagni di team le elaborano. Le attività hanno tre stati: in sospeso, in corso e completate. Le attività possono anche dipendere da altre attività: un'attività in sospeso con dipendenze non risolte non può essere rivendicata fino a quando quelle dipendenze non sono completate.

Il lead può assegnare attività esplicitamente, oppure i compagni di team possono auto-rivendicare:

* **Il lead assegna**: dite al lead quale attività assegnare a quale compagno di team
* **Auto-rivendicazione**: dopo aver completato un'attività, un compagno di team raccoglie la prossima attività non assegnata e non bloccata da solo

La rivendicazione delle attività utilizza il file locking per prevenire race condition quando più compagni di team tentano di rivendicare la stessa attività simultaneamente.

<h3 id="shut-down-teammates">
  Spegnere i compagni di team
</h3>

Per terminare gracefully la sessione di un compagno di team, fate riferimento ad esso per nome. Ad esempio, con un compagno di team denominato ricercatore:

```text theme={null}
Ask the researcher teammate to shut down
```

Il lead invia una richiesta di arresto. Il compagno di team può approvare, uscendo gracefully, o rifiutare con una spiegazione.

Le directory condivise del team vengono pulite automaticamente quando la sessione termina, quindi non c'è un passaggio di pulizia separato. Consultate [Architecture](#architecture) per sapere quali directory vengono rimosse e quali persistono per le sessioni riprese.

<h3 id="enforce-quality-gates-with-hooks">
  Applicare quality gate con hooks
</h3>

Utilizzate [hooks](/docs/it/hooks) per applicare regole quando i compagni di team finiscono il lavoro o le attività vengono create o completate:

* [`TeammateIdle`](/docs/it/hooks#teammateidle): viene eseguito quando un compagno di team sta per andare inattivo. Uscite con codice 2 per inviare feedback e mantenere il compagno di team al lavoro.
* [`TaskCreated`](/docs/it/hooks#taskcreated): viene eseguito quando un'attività sta per essere creata. Uscite con codice 2 per prevenire la creazione e inviare feedback.
* [`TaskCompleted`](/docs/it/hooks#taskcompleted): viene eseguito quando un'attività sta per essere contrassegnata come completata. Uscite con codice 2 per prevenire il completamento e inviare feedback.

<h2 id="how-agent-teams-work">
  Come funzionano i team di agenti
</h2>

Questa sezione copre l'architettura e la meccanica dietro i team di agenti. Se desiderate iniziare a utilizzarli, consultate [Controllare il vostro team di agenti](#control-your-agent-team) sopra.

<h3 id="how-claude-starts-agent-teams">
  Come Claude avvia i team di agenti
</h3>

Un team di agenti si forma quando il primo compagno di team viene generato, con la sessione principale che agisce come lead. Ci sono due modi in cui i compagni di team vengono generati:

* **Voi richiedete compagni di team**: date a Claude un compito che beneficia dal lavoro parallelo e chiedete esplicitamente compagni di team. Claude li genera in base alle vostre istruzioni.
* **Claude propone compagni di team**: se Claude determina che il vostro compito beneficerebbe dal lavoro parallelo, potrebbe suggerire di generare compagni di team. Voi confermate prima che proceda.

In entrambi i casi, rimanete in controllo. Claude non genererà compagni di team senza la vostra approvazione.

<h3 id="architecture">
  Architettura
</h3>

Un team di agenti consiste di:

| Componente             | Ruolo                                                                                  |
| :--------------------- | :------------------------------------------------------------------------------------- |
| **Team lead**          | La sessione Claude Code principale che genera compagni di team e coordina il lavoro    |
| **Compagni di team**   | Istanze Claude Code separate che lavorano ciascuna su attività assegnate               |
| **Elenco di attività** | Elenco condiviso di elementi di lavoro che i compagni di team rivendicano e completano |
| **Mailbox**            | Sistema di messaggistica per la comunicazione tra agenti                               |

Consultate [Scegliere una modalità di visualizzazione](#choose-a-display-mode) per le opzioni di configurazione della visualizzazione. I messaggi dei compagni di team arrivano al lead automaticamente.

La mailbox di ogni agente è un file JSON in `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`. Claude Code convalida ogni voce quando legge un file mailbox. Le voci che non corrispondono al formato del messaggio vengono segnalate come errori e rimosse dal file; i messaggi validi vengono comunque consegnati. Prima della v2.1.207, una singola voce mailbox malformata causava un errore ripetuto ogni secondo e bloccava la consegna per quella mailbox fino a quando non eliminavate manualmente il file.

Il sistema gestisce le dipendenze delle attività automaticamente. Quando un compagno di team completa un'attività da cui altre attività dipendono, le attività bloccate si sbloccano senza intervento manuale.

I team e le attività sono archiviati localmente con un nome derivato dalla sessione. Il nome è `session-` seguito dai primi otto caratteri dell'ID della sessione:

* **Configurazione del team**: `~/.claude/teams/{team-name}/config.json`
* **Elenco di attività**: `~/.claude/tasks/{team-name}/`

Claude Code genera entrambi automaticamente all'avvio della sessione e li aggiorna mentre i compagni di team si uniscono, vanno inattivi o se ne vanno. La directory di configurazione del team viene rimossa quando la sessione termina. La directory dell'elenco di attività persiste localmente e non viene mai caricata, quindi le sessioni riprese mantengono le loro attività. La conservazione è governata dallo stesso [`cleanupPeriodDays`](/docs/it/settings#available-settings) che già controllate per i trascritti di sessione.

La configurazione del team contiene lo stato di runtime come gli ID di sessione e gli ID dei riquadri tmux, quindi non modificatela manualmente o pre-autorizzatela: le vostre modifiche vengono sovrascritte al prossimo aggiornamento dello stato.

Per definire ruoli di compagni di team riutilizzabili, utilizzate invece [definizioni di subagent](#use-subagent-definitions-for-teammates).

La configurazione del team contiene un array `members` con il nome di ogni compagno di team, l'ID dell'agente e il tipo di agente. I compagni di team possono leggere questo file per scoprire altri membri del team.

Non esiste un equivalente a livello di progetto della configurazione del team. Un file come `.claude/teams/teams.json` nella vostra directory di progetto non è riconosciuto come configurazione; Claude lo tratta come un file ordinario.

<h3 id="use-subagent-definitions-for-teammates">
  Utilizzare definizioni di subagent per i compagni di team
</h3>

Quando generate un compagno di team, potete fare riferimento a un tipo di [subagent](/docs/it/sub-agents) da qualsiasi [ambito di subagent](/docs/it/sub-agents#choose-the-subagent-scope): progetto, utente, plugin o definito da CLI. Questo vi permette di definire un ruolo una volta, come un security-reviewer o test-runner, e riutilizzarlo sia come subagent delegato che come compagno di team di un team di agenti.

Per utilizzare una definizione di subagent, menzionatela per nome quando chiedete a Claude di generare il compagno di team:

```text theme={null}
Genera un compagno di team utilizzando il tipo di agente security-reviewer per controllare il modulo di autenticazione.
```

Il compagno di team onora i `tools` allowlist e il `model` di quella definizione, e il corpo della definizione viene aggiunto al prompt di sistema del compagno di team come istruzioni aggiuntive piuttosto che sostituirlo. Gli strumenti di coordinamento del team come `SendMessage` e gli strumenti di gestione delle attività sono sempre disponibili per un compagno di team anche quando `tools` limita altri strumenti.

<Note>
  I campi frontmatter `skills` e `mcpServers` in una definizione di subagent non vengono applicati quando quella definizione viene eseguita come compagno di team. I compagni di team caricano skills e MCP servers dalle vostre impostazioni di progetto e utente, come una sessione regolare.
</Note>

<h3 id="permissions">
  Permessi
</h3>

I compagni di team iniziano con le impostazioni di permesso del lead. Se il lead viene eseguito con `--dangerously-skip-permissions`, lo fanno anche tutti i compagni di team. Dopo la generazione, potete cambiare le modalità dei singoli compagni di team, ma non potete impostare modalità per compagno di team al momento della generazione.

Quando un agente invia un messaggio a un altro tramite `SendMessage`, l'agente ricevente viene informato che proviene da un'altra sessione Claude, non da voi. Un compagno di team non può approvare un prompt di permesso o fornire consenso per vostro conto, e un compagno di team a cui è stata negata un'azione non può trasmetterla a un altro compagno di team per aggirare il controllo. In [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), il classificatore tratta un'approvazione inoltrata da un altro agente come input non attendibile piuttosto che come conferma da voi.

I prompt di permesso dei compagni di team risalgono alla sessione lead, quindi approvate voi stessi lì. [Approvazione del piano](#require-plan-approval-for-teammates) è l'eccezione progettata: la sessione lead concede le approvazioni del piano dei compagni di team senza un prompt separato per voi.

<h3 id="context-and-communication">
  Context e comunicazione
</h3>

Ogni compagno di team ha il proprio context window. Quando generato, un compagno di team carica lo stesso contesto di progetto di una sessione regolare: CLAUDE.md, MCP servers e skills. Riceve anche il prompt di generazione dal lead. La cronologia della conversazione del lead non viene trasferita.

**Come i compagni di team condividono le informazioni:**

* **Consegna automatica dei messaggi**: quando i compagni di team inviano messaggi, vengono consegnati automaticamente ai destinatari. Il lead non ha bisogno di eseguire il polling per gli aggiornamenti.
* **Notifiche di inattività**: quando un compagno di team finisce e si ferma, notifica automaticamente il lead. A partire dalla v2.1.198, un compagno di team la cui sessione termina a causa di un errore API notifica al lead che ha avuto un errore e include il testo dell'errore, invece di sembrare di terminare normalmente.
* **Elenco di attività condiviso**: tutti gli agenti possono vedere lo stato delle attività e rivendicare il lavoro disponibile.
* **Messaggistica dei compagni di team**: invia un messaggio a un compagno di team specifico per nome. Per raggiungere tutti, inviate un messaggio per destinatario.

Il lead assegna a ogni compagno di team un nome quando lo genera, e qualsiasi compagno di team può messaggiare qualsiasi altro per quel nome. Per ottenere nomi prevedibili che potete referenziare nei prompt successivi, dite al lead come chiamare ogni compagno di team nella vostra istruzione di generazione.

<h3 id="token-usage">
  Utilizzo dei token
</h3>

I team di agenti utilizzano significativamente più token di una singola sessione. Ogni compagno di team ha il proprio context window e l'utilizzo dei token si scala con il numero di compagni di team attivi. Per ricerca, revisione e lavoro su nuove funzionalità, i token extra di solito valgono la pena. Per compiti di routine, una singola sessione è più conveniente. Consultate i [costi dei token dei team di agenti](/docs/it/costs#agent-team-token-costs) per la guida all'utilizzo.

<h2 id="use-case-examples">
  Esempi di casi d'uso
</h2>

Questi esempi mostrano come i team di agenti gestiscono compiti in cui l'esplorazione parallela aggiunge valore.

<h3 id="run-a-parallel-code-review">
  Eseguire una revisione del codice parallela
</h3>

Un singolo revisore tende a gravitare verso un tipo di problema alla volta. Dividere i criteri di revisione in domini indipendenti significa che la sicurezza, le prestazioni e la copertura dei test ricevono tutti un'attenzione approfondita simultaneamente. Il prompt assegna a ogni compagno di team una lente distinta in modo che non si sovrappongano:

```text theme={null}
Spawn three teammates to review PR #142:
- One focused on security implications
- One checking performance impact
- One validating test coverage
Have them each review and report findings.
```

Ogni revisore lavora dalla stessa PR ma applica un filtro diverso. Il lead sintetizza i risultati tra tutti e tre dopo che finiscono.

<h3 id="investigate-with-competing-hypotheses">
  Investigare con ipotesi concorrenti
</h3>

Quando la causa principale è poco chiara, un singolo agente tende a trovare una spiegazione plausibile e smettere di cercare. Il prompt combatte questo rendendo i compagni di team esplicitamente avversari: il lavoro di ognuno non è solo investigare la propria teoria ma sfidare le altre.

```text theme={null}
Users report the app exits after one message instead of staying connected.
Spawn 5 agent teammates to investigate different hypotheses. Have them talk to
each other to try to disprove each other's theories, like a scientific
debate. Update the findings doc with whatever consensus emerges.
```

La struttura del dibattito è il meccanismo chiave qui. L'investigazione sequenziale soffre di ancoraggio: una volta che una teoria è stata esplorata, l'investigazione successiva è distorta verso di essa.

Con più investigatori indipendenti che attivamente cercano di confutarsi a vicenda, la teoria che sopravvive è molto più probabile che sia la causa principale effettiva.

<h2 id="best-practices">
  Best practices
</h2>

<h3 id="give-teammates-enough-context">
  Fornire ai compagni di team contesto sufficiente
</h3>

I compagni di team caricano il contesto del progetto automaticamente, inclusi CLAUDE.md, MCP servers e skills, ma non ereditano la cronologia della conversazione del lead. Consultate [Context e comunicazione](#context-and-communication) per i dettagli. Includete i dettagli specifici dell'attività nel prompt di generazione:

```text theme={null}
Genera un compagno di team revisore di sicurezza con il prompt: "Esamina il modulo di autenticazione
in src/auth/ per vulnerabilità di sicurezza. Concentrati sulla gestione dei token, sulla
gestione della sessione e sulla convalida dell'input. L'app utilizza token JWT archiviati in
cookie httpOnly. Segnala eventuali problemi con valutazioni di gravità."
```

<h3 id="choose-an-appropriate-team-size">
  Scegliere una dimensione del team appropriata
</h3>

Non c'è un limite rigido al numero di compagni di team, ma si applicano vincoli pratici:

* **I costi dei token si scalano linearmente**: ogni compagno di team ha il proprio context window e consuma token indipendentemente. Consultate i [costi dei token dei team di agenti](/docs/it/costs#agent-team-token-costs) per i dettagli.
* **L'overhead di coordinamento aumenta**: più compagni di team significa più comunicazione, coordinamento delle attività e potenziale per conflitti
* **Rendimenti decrescenti**: oltre un certo punto, i compagni di team aggiuntivi non accelerano il lavoro proporzionalmente

Iniziate con 3-5 compagni di team per la maggior parte dei flussi di lavoro. Questo bilancia il lavoro parallelo con il coordinamento gestibile. Gli esempi in questa guida utilizzano 3-5 compagni di team perché quell'intervallo funziona bene in diversi tipi di attività.

Avere 5-6 [attività](/docs/it/agent-teams#architecture) per compagno di team mantiene tutti produttivi senza eccessivo context switching. Se avete 15 attività indipendenti, 3 compagni di team è un buon punto di partenza.

Scalate solo quando il lavoro beneficia genuinamente dall'avere compagni di team che lavorano simultaneamente. Tre compagni di team focalizzati spesso superano cinque dispersi.

<h3 id="size-tasks-appropriately">
  Dimensionare le attività appropriatamente
</h3>

* **Troppo piccole**: l'overhead di coordinamento supera il beneficio
* **Troppo grandi**: i compagni di team lavorano troppo a lungo senza check-in, aumentando il rischio di sforzo sprecato
* **Giuste**: unità auto-contenute che producono un deliverable chiaro, come una funzione, un file di test o una revisione

<Tip>
  Il lead suddivide il lavoro in attività e le assegna ai compagni di team automaticamente. Se non sta creando abbastanza attività, chiedetegli di dividere il lavoro in pezzi più piccoli. Avere 5-6 attività per compagno di team mantiene tutti produttivi e permette al lead di riassegnare il lavoro se qualcuno rimane bloccato.
</Tip>

<h3 id="wait-for-teammates-to-finish">
  Aspettare che i compagni di team finiscano
</h3>

A volte il lead inizia a implementare le attività stesso invece di aspettare i compagni di team. Se notate questo:

```text theme={null}
Aspetta che i tuoi compagni di team completino le loro attività prima di procedere
```

<h3 id="start-with-research-and-review">
  Iniziare con ricerca e revisione
</h3>

Se siete nuovi ai team di agenti, iniziate con compiti che hanno confini chiari e non richiedono di scrivere codice: revisionare una PR, ricercare una libreria o investigare un bug. Questi compiti mostrano il valore dell'esplorazione parallela senza le sfide di coordinamento che vengono con l'implementazione parallela.

<h3 id="avoid-file-conflicts">
  Evitare conflitti di file
</h3>

Due compagni di team che modificano lo stesso file porta a sovrascritture. Suddividete il lavoro in modo che ogni compagno di team possieda un set diverso di file.

<h3 id="monitor-and-steer">
  Monitorare e sterzare
</h3>

Controllate il progresso dei compagni di team, reindirizzate gli approcci che non funzionano e sintetizzate i risultati man mano che arrivano. Lasciare un team senza supervisione per troppo tempo aumenta il rischio di sforzo sprecato.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="teammates-not-appearing">
  I compagni di team non appaiono
</h3>

Se i compagni di team non appaiono dopo aver chiesto a Claude di creare un team:

* In modalità in-process, i compagni di team appaiono nel pannello agente sotto l'input del prompt. Utilizzate i tasti freccia su e giù per selezionarne uno, quindi premete Invio per visualizzarlo.
* Una riga di compagno di team che è scomparsa dopo essere rimasta inattiva è stata nascosta, non interrotta. Le righe inattive si nascondono 30 secondi dopo che l'intero pannello diventa inattivo e riappaiono al turno successivo del compagno di team. Quando più di tre compagni di team sono inattivi, le loro righe in eccesso si comprimono in una singola riga `N idle agents` che Invio espande. Inviate un messaggio al compagno di team per nome per riportare una riga nascosta.
* Controllate che il compito che avete dato a Claude fosse abbastanza complesso da giustificare un team. Claude decide se creare compagni di team in base al compito.
* Se avete esplicitamente richiesto split panes, assicuratevi che tmux sia installato e disponibile nel vostro PATH:
  ```bash theme={null}
  which tmux
  ```
* Per iTerm2, verificate che la CLI `it2` sia installata e che l'API Python sia abilitata nelle preferenze di iTerm2.

<h3 id="too-many-permission-prompts">
  Troppi prompt di permesso
</h3>

Le richieste di permesso dei compagni di team si propagano al lead, il che può creare attrito. Pre-approvate le operazioni comuni nelle vostre [impostazioni di permesso](/docs/it/permissions) prima di generare i compagni di team per ridurre le interruzioni.

<h3 id="teammates-stopping-on-errors">
  I compagni di team si fermano su errori
</h3>

I compagni di team possono fermarsi dopo aver incontrato errori invece di recuperare. Controllate il loro output selezionando il compagno di team nel pannello agente e premendo Invio in modalità in-process, o facendo clic sul riquadro in modalità split, quindi:

* Date loro istruzioni aggiuntive direttamente
* Generate un compagno di team sostitutivo per continuare il lavoro

A partire dalla v2.1.198, un messaggio dal lead o da un altro compagno di team riattiva un compagno di team in-process che è in attesa di riprovare una richiesta API non riuscita, quindi riprova immediatamente invece di aspettare il ritardo di ripetizione completo.

<h3 id="lead-shuts-down-before-work-is-done">
  Il lead si spegne prima che il lavoro sia finito
</h3>

Il lead potrebbe decidere che il team è finito prima che tutte le attività siano effettivamente completate. Se questo accade, ditegli di continuare. Potete anche dire al lead di aspettare che i compagni di team finiscano prima di procedere se inizia a fare lavoro invece di delegare.

<h3 id="orphaned-tmux-sessions">
  Sessioni tmux orfane
</h3>

Se una sessione tmux persiste dopo che il team finisce, potrebbe non essere stata completamente pulita. Elencate le sessioni e uccidete quella creata dal team:

```bash theme={null}
tmux ls
tmux kill-session -t <session-name>
```

<h2 id="limitations">
  Limitazioni
</h2>

I team di agenti sono sperimentali. Le limitazioni attuali di cui essere consapevoli:

* **Nessuna ripresa della sessione con compagni di team in-process**: `/resume` e `/rewind` non ripristinano i compagni di team in-process. Dopo aver ripreso una sessione, il lead potrebbe tentare di messaggiare compagni di team che non esistono più. Se questo accade, dite al lead di generare nuovi compagni di team.
* **Lo stato dell'attività può ritardare**: i compagni di team a volte non riescono a contrassegnare le attività come completate, il che blocca le attività dipendenti. Se un'attività sembra bloccata, controllate se il lavoro è effettivamente fatto e aggiornate lo stato dell'attività manualmente o dite al lead di spingere il compagno di team.
* **L'arresto può essere lento**: i compagni di team finiscono la loro richiesta attuale o la chiamata dello strumento prima di spegnersi, il che può richiedere tempo.
* **Un team per sessione**: una sessione ha esattamente un team, limitato a quella sessione. Non potete creare team denominati aggiuntivi o condividere un team tra sessioni.
* **Nessun team annidato**: i compagni di team non possono generare i loro propri compagni di team. Solo il lead può gestire il team.
* **Nessun subagent in background da compagni di team in-process**: i propri subagent di un compagno di team in-process vengono eseguiti in primo piano. Chiedere uno in background, sia con `run_in_background` che con una definizione di subagent che imposta `background: true`, restituisce un errore, perché il lavoro in background di un compagno di team non può sopravvivere al processo del lead. I subagent lanciati dalla conversazione principale seguono il [default in background](/docs/it/sub-agents#run-subagents-in-foreground-or-background).
* **Il lead è fisso**: la sessione principale è il lead per tutta la sua durata. Non potete promuovere un compagno di team a lead o trasferire la leadership.
* **Permessi impostati al momento della generazione**: tutti i compagni di team iniziano con la modalità di permesso del lead. Potete cambiare le modalità dei singoli compagni di team dopo la generazione, ma non potete impostare modalità per compagno di team al momento della generazione.
* **Split panes richiedono tmux o iTerm2**: la modalità in-process predefinita funziona in qualsiasi terminale. La modalità split-pane non è supportata nel terminale integrato di VS Code, Windows Terminal o Ghostty.

<Tip>
  **`CLAUDE.md` funziona normalmente**: i compagni di team leggono i file `CLAUDE.md` dalla loro directory di lavoro. Utilizzate questo per fornire una guida specifica del progetto a tutti i compagni di team.
</Tip>

<h2 id="next-steps">
  Prossimi passi
</h2>

Esplorate approcci correlati per il lavoro parallelo e la delega:

* **Delega leggera**: i [subagents](/docs/it/sub-agents) generano agenti helper per ricerca o verifica all'interno della vostra sessione, migliore per compiti che non hanno bisogno di coordinamento tra agenti
* **Sessioni parallele manuali**: i [Git worktrees](/docs/it/worktrees) vi permettono di eseguire più sessioni Claude Code voi stessi senza coordinamento automatico del team
* **Confrontare gli approcci**: consultate il confronto [subagent vs agent team](/docs/it/features-overview#compare-similar-features) per una suddivisione fianco a fianco
