> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eseguire agenti in parallelo

> Confronta i modi in cui Claude Code può affrontare più attività contemporaneamente: subagenti, visualizzazione agenti, team di agenti e flussi di lavoro dinamici.

[Subagenti](/docs/it/sub-agents), [visualizzazione agenti](/docs/it/agent-view), [team di agenti](/docs/it/agent-teams) e [flussi di lavoro dinamici](/docs/it/workflows) parallelizzano il lavoro in modi diversi. Quello giusto dipende dal fatto che Lei voglia rimanere in ogni conversazione Lei stesso, delegare i compiti e controllare più tardi, oppure far coordinare a Claude un gruppo di lavoratori per Lei.

| Approccio                                  | Cosa ti offre                                                                                                                                                                  | Usalo quando                                                                                                                                                                                                                                                             |
| :----------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Subagenti](/docs/it/sub-agents)                | Lavoratori delegati all'interno di una sessione che svolgono un compito secondario nel loro contesto e restituiscono un riepilogo                                              | Un compito secondario allagherebbe la Vostra conversazione principale con risultati di ricerca, log o contenuti di file che non consulterete di nuovo                                                                                                                    |
| [Visualizzazione agenti](/docs/it/agent-view)   | Una schermata per inviare e monitorare sessioni in esecuzione in background, aperta con `claude agents`. Anteprima di ricerca                                                  | Avete diversi compiti indipendenti e volete delegarli, controllare lo stato a colpo d'occhio e intervenire solo quando uno ha bisogno di voi                                                                                                                             |
| [Team di agenti](/docs/it/agent-teams)          | Più sessioni coordinate con un elenco di attività condiviso e messaggistica tra agenti, gestite da un leader. Sperimentale e disabilitato per impostazione predefinita         | Volete che Claude divida un progetto in parti, le assegni e mantenga i lavoratori sincronizzati                                                                                                                                                                          |
| [Flussi di lavoro dinamici](/docs/it/workflows) | Uno script che esegue molti subagenti e controlla i loro risultati, per un lavoro troppo grande per coordinare in un solo turno o che necessita di più di un singolo passaggio | Un compito è troppo grande per una manciata di subagenti, oppure volete che i risultati siano verificati l'uno contro l'altro: un audit a livello di codebase, una migrazione di 500 file, ricerca verificata in modo incrociato, o un piano elaborato da diversi angoli |

In ogni approccio i lavoratori sono sessioni Claude. Per coinvolgere uno strumento diverso, esponilo a Claude come [server MCP](/docs/it/mcp).

Due ulteriori strumenti supportano questo lavoro senza essere un modo per eseguire agenti stessi:

* [Worktrees](/docs/it/worktrees) danno a ogni sessione un checkout git separato, così le sessioni parallele non modificano mai gli stessi file. Usateli per le sessioni che eseguite voi stessi. La visualizzazione agenti sposta automaticamente ogni sessione inviata nel suo proprio worktree, e i subagenti che generate possono ottenerne uno anche loro.
* [`/batch`](/docs/it/commands) è una [skill](/docs/it/skills) che ha Claude dividere un grande cambiamento in 5 a 30 subagenti isolati da worktree che ciascuno apre una pull request. È un uso confezionato di subagenti e worktrees, non uno stile di coordinamento separato.

Alcuni altri strumenti eseguono Claude senza che voi guidiate ogni passaggio, ma risolvono un problema diverso rispetto alla divisione del lavoro tra agenti:

* Un [comando bash in background](/docs/it/interactive-mode#background-bash-commands) esegue un comando shell senza bloccare la conversazione. Non genera un agente.
* Un [subagente biforcato](/docs/it/sub-agents#fork-the-current-conversation) è un subagente che eredita il vostro contesto di conversazione completo invece di iniziare da zero. È un modo per generare un subagente, non una superficie separata.
* Una [routine](/docs/it/routines) esegue una sessione secondo una pianificazione nel cloud di Anthropic, non in parallelo sulla vostra macchina.

<Note>
  L'esecuzione di più sessioni o subagenti contemporaneamente moltiplica l'utilizzo dei token. Vedete [Costi](/docs/it/costs) per i dettagli su utilizzo e limiti di velocità.
</Note>

<h2 id="choose-an-approach">
  Scegli un approccio
</h2>

L'approccio giusto dipende da chi coordina il lavoro, dal fatto che i lavoratori debbano comunicare e dal fatto che modifichino gli stessi file:

* **Chi coordina il lavoro?**
  * Claude delega e raccoglie i risultati all'interno di una conversazione: [subagenti](/docs/it/sub-agents)
  * Tu affidi compiti indipendenti e controlli di nuovo più tardi: [visualizzazione agenti](/docs/it/agent-view)
  * Claude pianifica, assegna e supervisiona un gruppo di lavoratori: [team di agenti](/docs/it/agent-teams), sperimentale e disabilitato per impostazione predefinita
  * Uno script tiene il piano invece del giudizio turno per turno di Claude: [flussi di lavoro dinamici](/docs/it/workflows). Vedi [come i flussi di lavoro si confrontano con i subagenti e le skills](/docs/it/workflows#when-to-use-a-workflow)
* **I lavoratori hanno bisogno di parlarsi?** I subagenti riportano i risultati alla conversazione che li ha generati, e le sessioni di visualizzazione agenti riportano solo a te. I compagni di squadra in un team di agenti condividono un elenco di attività e si messaggiano direttamente.
* **I compiti toccano gli stessi file?** Isola il lavoro con [worktrees](/docs/it/worktrees). I subagenti e le sessioni che esegui tu stesso possono ciascuno utilizzare un worktree separato. I team di agenti non isolano i compagni di squadra nei worktrees, quindi [partiziona il lavoro](/docs/it/agent-teams#avoid-file-conflicts) in modo che ogni compagno di squadra possieda un set diverso di file.

<h2 id="check-on-running-work">
  Controlla il lavoro in esecuzione
</h2>

Il comando per controllare il lavoro in esecuzione dipende da quale approccio hai utilizzato:

* Per le sessioni in background, `claude agents` apre [visualizzazione agenti](/docs/it/agent-view): una schermata che mostra ogni sessione, il suo stato e quali hanno bisogno del tuo input.
* Per i subagenti nella sessione corrente, i subagenti background denominati appaiono nella typeahead @-mention con il loro stato. A partire da v2.1.198, `/agents` non apre più un pannello; stampa un avviso che punta alle posizioni dei file dei subagenti. Per [creare e modificare subagenti personalizzati](/docs/it/sub-agents#configure-subagents), chiedi a Claude o modifica i file direttamente. Nonostante il nome simile, `/agents` è separato da `claude agents`.
* Per qualsiasi cosa in esecuzione in background della sessione corrente, `/tasks` elenca ogni elemento e ti consente di controllare, collegarti o interrompere. L'elenco include anche i subagenti che hanno terminato.
* Per i flussi di lavoro dinamici, `/workflows` elenca le esecuzioni in corso e completate, la fase in cui si trova ciascuna e quanti agenti hanno terminato.

Per una visualizzazione desktop di tutte le tue sessioni, vedi [sessioni parallele nell'app desktop](/docs/it/desktop#work-in-parallel-with-sessions).

<h2 id="learn-more">
  Scopri di più
</h2>

Ogni guida di seguito copre la configurazione per un approccio:

* [Crea subagenti personalizzati](/docs/it/sub-agents): definisci specialisti riutilizzabili e controlla quali strumenti possono utilizzare.
* [Gestisci agenti con visualizzazione agenti](/docs/it/agent-view): invia sessioni, osserva il loro stato e collegati quando uno ha bisogno di te.
* [Orchestra team di agenti](/docs/it/agent-teams): configura un leader e compagni di squadra, assegna compiti e rivedi il loro lavoro.
* [Orchestra flussi di lavoro dinamici](/docs/it/workflows): esegui un flusso di lavoro in bundle o fai in modo che Claude ne scriva uno che esegua molti subagenti e verifichi i loro risultati l'uno contro l'altro.
* [Esegui sessioni parallele con worktrees](/docs/it/worktrees): avvia Claude in un checkout isolato, controlla cosa viene copiato e pulisci in seguito.
