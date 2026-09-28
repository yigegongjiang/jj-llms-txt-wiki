> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Configura revisioni automatiche dei PR che rilevano errori logici, vulnerabilità di sicurezza e regressioni utilizzando l'analisi multi-agente dell'intero codebase

<Note>
  Code Review è in anteprima di ricerca, disponibile per gli abbonamenti [Team e Enterprise](https://claude.ai/admin-settings/claude-code). Non è disponibile per le organizzazioni con [Zero Data Retention](/docs/it/zero-data-retention) abilitato. Su altri piani, è comunque possibile [esaminare un diff localmente](#review-a-diff-locally) con il comando `/code-review`.
</Note>

Code Review analizza i tuoi pull request su GitHub e pubblica i risultati come commenti inline sulle righe di codice dove ha trovato problemi. Una flotta di agenti specializzati esamina i cambiamenti del codice nel contesto dell'intero codebase, cercando errori logici, vulnerabilità di sicurezza, edge case interrotti e regressioni sottili.

I risultati sono contrassegnati per gravità e non approvano o bloccano il tuo PR, quindi i flussi di lavoro di revisione esistenti rimangono intatti. Puoi regolare cosa Claude segnala aggiungendo un file `CLAUDE.md` o `REVIEW.md` al tuo repository.

Per eseguire Claude nella tua infrastruttura CI invece di questo servizio gestito, vedi [GitHub Actions](/docs/it/github-actions) o [GitLab CI/CD](/docs/it/gitlab-ci-cd). Per i repository su un'istanza GitHub self-hosted, vedi [GitHub Enterprise Server](/docs/it/github-enterprise-server).

Questa pagina copre:

* [Come funzionano le revisioni](#how-reviews-work)
* [Configurazione](#set-up-code-review)
* [Attivazione manuale delle revisioni](#manually-trigger-reviews) con `@claude review` e `@claude review always`
* [Personalizzazione delle revisioni](#customize-reviews) con `CLAUDE.md` e `REVIEW.md`
* [Prezzi](#pricing)
* [Risoluzione dei problemi](#troubleshooting) esecuzioni non riuscite e commenti mancanti
* [Revisione di un diff localmente](#review-a-diff-locally) con il comando `/code-review`

<h2 id="how-reviews-work">
  Come funzionano le revisioni
</h2>

Una volta che un amministratore [abilita Code Review](#set-up-code-review) per la tua organizzazione, le revisioni si attivano quando un PR si apre, ad ogni push, o quando richiesto manualmente, a seconda del comportamento configurato del repository. Commentando `@claude review` [avvia le revisioni su un PR](#manually-trigger-reviews) in qualsiasi modalità.

Quando una revisione viene eseguita, più agenti analizzano il diff e il codice circostante in parallelo sull'infrastruttura Anthropic. Ogni agente cerca una classe diversa di problema, quindi un passaggio di verifica controlla i candidati rispetto al comportamento effettivo del codice per filtrare i falsi positivi. I risultati vengono deduplicati, classificati per gravità e pubblicati come commenti inline sulle righe specifiche dove sono stati trovati i problemi, con un riepilogo nel corpo della revisione. Se non vengono trovati problemi, Code Review aggiorna il check run di GitHub per mostrare che non sono stati rilevati problemi. Claude può anche pubblicare un breve commento di conferma sul PR.

Le revisioni si scalano in costo con la dimensione e la complessità del PR, completandosi in media in 20 minuti. Gli amministratori possono monitorare l'attività di revisione e la spesa tramite il [dashboard di analisi](#view-usage).

<h3 id="severity-levels">
  Livelli di gravità
</h3>

Ogni risultato è contrassegnato con un livello di gravità:

| Marcatore | Gravità       | Significato                                                           |
| :-------- | :------------ | :-------------------------------------------------------------------- |
| 🔴        | Importante    | Un bug che dovrebbe essere corretto prima del merge                   |
| 🟡        | Nit           | Un problema minore, vale la pena correggerlo ma non bloccante         |
| 🟣        | Pre-esistente | Un bug che esiste nel codebase ma non è stato introdotto da questo PR |

I risultati includono una sezione di ragionamento esteso comprimibile che puoi espandere per capire perché Claude ha segnalato il problema e come ha verificato il problema.

<h3 id="rate-and-reply-to-findings">
  Valutare e rispondere ai risultati
</h3>

Ogni commento di revisione da Claude arriva con 👍 e 👎 già allegati, quindi entrambi i pulsanti appaiono nell'interfaccia utente di GitHub per una valutazione con un clic. Fai clic su 👍 se il risultato era utile o 👎 se era sbagliato o rumoroso. Anthropic raccoglie i conteggi delle reazioni dopo il merge del PR e li utilizza per ottimizzare il revisore. Le reazioni non attivano una re-revisione o cambiano nulla sul PR.

Rispondere a un commento inline non richiede a Claude di rispondere o aggiornare il PR. Per agire su un risultato, correggi il codice e fai un push. Se il PR è sottoscritto alle revisioni attivate da push, l'esecuzione successiva risolve il thread quando il problema è corretto. Per richiedere una revisione nuova senza fare un push, commenta `@claude review` come [commento PR di primo livello](#manually-trigger-reviews).

Per dismissare un risultato senza una modifica del codice, risolvi il suo thread; rispondere non lo dismissa.

<h3 id="check-run-output">
  Output del check run
</h3>

Oltre ai commenti di revisione inline, ogni revisione popola il check run **Claude Code Review** che appare insieme ai tuoi check CI. Espandi il suo link **Details** per vedere un riepilogo di ogni risultato in un unico posto, ordinato per gravità:

| Gravità       | File:Riga                 | Problema                                                                                      |
| ------------- | ------------------------- | --------------------------------------------------------------------------------------------- |
| 🔴 Importante | `src/auth/session.ts:142` | L'aggiornamento del token corre in parallelo con il logout, lasciando sessioni stantie attive |
| 🟡 Nit        | `src/auth/session.ts:88`  | `parseExpiry` restituisce silenziosamente 0 su input malformato                               |

Ogni risultato appare anche come un'annotazione nella scheda **Files changed**, contrassegnato direttamente sulle righe diff rilevanti. I risultati importanti vengono visualizzati con un marcatore rosso, i nit con un avviso giallo e i bug pre-esistenti con un avviso grigio. Le annotazioni e la tabella di gravità vengono scritte nel check run indipendentemente dai commenti di revisione inline, quindi rimangono disponibili anche se GitHub rifiuta un commento inline su una riga che si è spostata.

Il check run si completa sempre con una conclusione neutra, quindi non blocca mai il merge attraverso le regole di protezione del ramo. Se vuoi bloccare i merge sui risultati di Code Review, leggi il breakdown della gravità dall'output del check run nel tuo CI. L'ultima riga del testo Details è un commento leggibile da macchina che il tuo flusso di lavoro può analizzare con `gh` e jq. Per trovare l'ID del check run, elenca i check run del commit con `gh api repos/OWNER/REPO/commits/<commit-sha>/check-runs --jq '.check_runs[] | {id, name}'` e prendi l'`id` del run **Claude Code Review**. Sostituisci `OWNER`, `REPO` e `CHECK_RUN_ID` con il proprietario del tuo repository, il nome del repository e quell'ID:

```bash theme={null}
gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID \
  --jq '.output.text | split("bughunter-severity: ")[1] | split(" -->")[0] | fromjson'
```

Questo restituisce un oggetto JSON con conteggi per gravità, ad esempio `{"normal": 2, "nit": 1, "pre_existing": 0}`. La chiave `normal` contiene il conteggio dei risultati Importanti; un valore diverso da zero significa che Claude ha trovato almeno un bug che vale la pena correggere prima del merge.

<h3 id="what-code-review-checks">
  Cosa controlla Code Review
</h3>

Per impostazione predefinita, Code Review si concentra sulla correttezza: bug che interromperebbero la produzione, non preferenze di formattazione o copertura di test mancante. Puoi espandere cosa controlla [aggiungendo file di guida](#customize-reviews) al tuo repository.

<h2 id="set-up-code-review">
  Configura Code Review
</h2>

Un proprietario abilita Code Review una volta per l'organizzazione e seleziona quali repository includere.

<Steps>
  <Step title="Apri le impostazioni di amministrazione di Claude Code">
    Vai a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) e trova la sezione Code Review. Hai bisogno del ruolo Proprietario o Proprietario principale nella tua organizzazione Claude e del permesso di installare GitHub App nella tua organizzazione GitHub.
  </Step>

  <Step title="Avvia la configurazione">
    Fai clic su **Setup**. Questo inizia il flusso di installazione dell'app GitHub.
  </Step>

  <Step title="Installa l'app GitHub di Claude">
    Segui i prompt per installare l'app GitHub di Claude: scegli l'organizzazione GitHub che possiede i repository che desideri revisionare, scegli a quali repository l'app può accedere e approva i permessi richiesti.

    Per revisionare una pull request, Claude legge i contenuti del tuo repository attraverso l'accesso in lettura dell'app e pubblica commenti e il [check run](#check-run-output) attraverso il suo accesso in scrittura alle pull request e ai check. Durante l'installazione, concedi un insieme di permessi più ampio condiviso da altre funzionalità di Claude, come [GitHub Actions](/docs/it/github-actions); vedi [Permessi dell'app GitHub](/docs/it/github-actions#github-app-permissions) per l'elenco completo.
  </Step>

  <Step title="Seleziona i repository">
    Scegli quali repository abilitare per Code Review. Se non vedi un repository, assicurati di aver dato all'app GitHub di Claude l'accesso durante l'installazione. Puoi aggiungere altri repository in seguito.
  </Step>

  <Step title="Imposta i trigger di revisione per repository">
    Dopo il completamento della configurazione, la sezione Code Review mostra i tuoi repository in una tabella. Per ogni repository, utilizza il dropdown **Review Behavior** per scegliere quando vengono eseguite le revisioni:

    * **Once after PR creation**: la revisione viene eseguita una volta quando un PR viene aperto o contrassegnato come pronto per la revisione
    * **After every push**: la revisione viene eseguita ad ogni push al ramo PR, catturando nuovi problemi mentre il PR si evolve e risolvendo automaticamente i thread quando correggi i problemi segnalati
    * **Manual**: aprire o eseguire il push a un PR non avvia una revisione; commenta [`@claude review`](#manually-trigger-reviews) per richiederne una, o `@claude review always` per sottoscrivere anche il PR alle revisioni su push successivi

    Qualunque opzione tu scelga, Claude reviziona una [pull request da un fork](#review-pull-requests-from-forks) solo quando qualcuno commenta `@claude review` su di essa.

    La revisione ad ogni push esegue il maggior numero di revisioni e costa di più. La modalità manuale è utile per i repository ad alto traffico dove vuoi optare per PR specifici nella revisione, o per iniziare a revisionare i tuoi PR solo quando sono pronti.
  </Step>
</Steps>

La tabella dei repository mostra anche il costo medio per revisione per ogni repository in base all'attività recente. Utilizza il menu delle azioni della riga per attivare o disattivare Code Review per repository, o per rimuovere completamente un repository.

Per verificare la configurazione, apri un PR di test. Se hai scelto un trigger automatico, un check run denominato **Claude Code Review** appare entro pochi minuti. Se hai scelto Manual, commenta `@claude review` sul PR per avviare la prima revisione. Se non appare alcun check run, conferma che il repository è elencato nelle tue impostazioni di amministrazione e che l'app GitHub di Claude ha accesso ad esso.

<h2 id="manually-trigger-reviews">
  Attiva manualmente le revisioni
</h2>

I comandi di commento avviano una revisione su richiesta. Funzionano indipendentemente dal trigger configurato del repository, quindi puoi usarli per optare per PR specifici nella revisione in modalità Manual o per ottenere una re-revisione immediata in altre modalità.

| Comando                 | Cosa fa                                                                                        |
| :---------------------- | :--------------------------------------------------------------------------------------------- |
| `@claude review`        | Avvia una singola revisione senza sottoscrivere il PR ai push futuri                           |
| `@claude review always` | Avvia una revisione e sottoscrive il PR alle revisioni attivate da push da quel momento in poi |
| `@claude review once`   | Uguale a `@claude review`: avvia una singola revisione senza sottoscrivere                     |

Usa `@claude review always` quando vuoi che ogni push successivo al PR avvii una revisione aggiornata, ad esempio su un PR ad alta priorità in un repository impostato su modalità Manual. Poiché il comando semplice non sottoscrive il PR, puoi richiedere un secondo parere una tantum senza cambiare se i push successivi attivano revisioni.

<Note>
  Prima di un aggiornamento di luglio 2026, `@claude review` sottoscriveva il PR alle revisioni attivate da push. Se hai fatto affidamento su quel comportamento, commenta `@claude review always` invece. `@claude review once` funziona ancora e si comporta allo stesso modo del comando semplice.
</Note>

Affinché uno di questi comandi attivi una revisione:

* Postalo come commento PR di primo livello, non come commento inline su una riga diff
* Metti il comando all'inizio del commento, con `once` o `always` sulla stessa riga del resto del comando
* Devi avere accesso in scrittura, manutenzione o amministratore sul repository
* Il PR deve essere aperto

Se il repository appartiene a un'organizzazione e la tua iscrizione a quell'organizzazione è privata, che è l'impostazione predefinita di GitHub, GitHub non ti identifica a Claude come membro. Claude potrebbe comunque reagire al tuo commento con 👀, ma non avvia una revisione a meno che tu non sia stato aggiunto al repository direttamente come collaboratore, anche quando un team o le autorizzazioni di base dell'organizzazione ti danno accesso in scrittura. Per risolvere questo, [rendi pubblica la tua iscrizione all'organizzazione](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-your-membership-in-organizations/publicizing-or-hiding-organization-membership) o chiedi a un amministratore del repository di aggiungerti al repository come collaboratore.

A differenza dei trigger automatici, i trigger manuali vengono eseguiti su PR bozza, poiché una richiesta esplicita segnala che vuoi la revisione ora indipendentemente dallo stato di bozza.

Se una revisione è già in esecuzione su quel PR, la richiesta viene messa in coda fino al completamento della revisione in corso. Puoi monitorare l'avanzamento tramite il check run sul PR.

<h3 id="review-pull-requests-from-forks">
  Rivedi pull request da fork
</h3>

Claude non rivede una pull request da un fork automaticamente, indipendentemente dall'impostazione **Review Behavior** del repository. Per avviarne una, commenta `@claude review` sulla pull request. I [requisiti per i comandi di commento](#manually-trigger-reviews) si applicano ancora, e l'accesso in scrittura di cui hai bisogno è al repository di base, non al fork.

Per ottenere un'altra revisione di una pull request da fork, pubblica un nuovo commento `@claude review`. `@claude review always` funziona anche, ma non sottoscrive la pull request alle revisioni su push successivi. Nient'altro che un comando di commento avvia una revisione su una pull request da fork:

* Fare clic su **Re-run** sul check run non avvia una revisione
* Fare push di nuovi commit non avvia una revisione, nemmeno in un repository impostato su **After every push**

<h2 id="customize-reviews">
  Personalizza le revisioni
</h2>

Code Review legge due file dal tuo repository per guidare cosa segnala. Differiscono nel modo in cui influenzano fortemente la revisione:

* **`CLAUDE.md`**: istruzioni di progetto condivise che Claude Code utilizza per tutti i compiti, non solo le revisioni. Code Review lo legge come contesto di progetto e segnala le violazioni appena introdotte come nit.
* **`REVIEW.md`**: istruzioni solo per la revisione, fornite agli agenti che trovano e verificano i risultati e consultate dagli agenti che classificano e segnalano i risultati. Usalo per dire cosa vuole segnalato il tuo team, a quale gravità e come vengono segnalati i risultati.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review legge i file `CLAUDE.md` del tuo repository e tratta le violazioni appena introdotte come risultati a [livello di nit](#severity-levels). Questo funziona bidirezionalmente: se il tuo PR cambia il codice in un modo che rende una dichiarazione `CLAUDE.md` obsoleta, Claude segnala che i documenti devono essere aggiornati anche loro.

Claude legge i file `CLAUDE.md` a ogni livello della gerarchia di directory, quindi le regole nel `CLAUDE.md` di una sottodirectory si applicano solo ai file sotto quel percorso. Vedi la [documentazione della memoria](/docs/it/memory) per ulteriori informazioni su come funziona `CLAUDE.md`.

Per una guida specifica della revisione che non vuoi applicata alle sessioni generali di Claude Code, usa [`REVIEW.md`](#review-md) invece.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` è un file alla radice del tuo repository che personalizza Code Review per il tuo repository. Gli agenti nella pipeline di revisione che trovano e verificano i risultati ricevono i suoi contenuti come istruzioni di revisione del tuo repository, insieme alla guida di revisione predefinita di Code Review, e gli agenti che classificano e segnalano i risultati la consultano prima di stabilire la gravità e scrivere la revisione.

Metti le regole che vuoi applicate direttamente in `REVIEW.md`.

<h4 id="what-you-can-tune">
  Cosa puoi regolare
</h4>

`REVIEW.md` è markdown freeform, quindi qualsiasi cosa tu possa esprimere come istruzione di revisione è in ambito. I modelli di seguito hanno il maggior impatto nella pratica.

**Gravità**: ridefinisci cosa significa 🔴 Importante per il tuo repository. La calibrazione predefinita è mirata al codice di produzione; un repository di documenti, un repository di configurazione o un prototipo potrebbe volere una definizione molto più ristretta. Dichiara esplicitamente quali classi di risultato sono Importanti e quali sono Nit al massimo. Puoi anche escalare nell'altra direzione, ad esempio trattando qualsiasi violazione di `CLAUDE.md` come Importante piuttosto che il nit predefinito.

**Volume di nit**: limita quanti commenti 🟡 Nit una singola revisione pubblica. La prosa e i file di configurazione possono essere lucidati per sempre. Un limite come "segnala al massimo cinque nit, menziona il resto come conteggio nel riepilogo" mantiene le revisioni attuabili.

**Regole di salto**: elenca percorsi, modelli di ramo e categorie di risultati dove Claude non dovrebbe pubblicare risultati. I candidati comuni sono codice generato, lockfile, dipendenze vendute e rami creati da macchine, insieme a qualsiasi cosa il tuo CI già applica come linting o controllo ortografico. Per i percorsi che meritano una revisione ma non un controllo completo, imposta un bar più alto invece di saltare completamente: "in `scripts/`, segnala solo se quasi certo e grave."

**Controlli specifici del repository**: aggiungi regole che vuoi segnalate su ogni PR, come "le nuove rotte API devono avere un test di integrazione." Poiché `REVIEW.md` raggiunge direttamente ogni agente di ricerca e verifica dei risultati, questi atterrano più affidabilmente rispetto alle stesse regole in un lungo `CLAUDE.md`.

**Bar di verifica**: richiedi prove prima che una classe di risultato sia pubblicata. Ad esempio, "i reclami di comportamento hanno bisogno di una citazione `file:line` nella fonte, non un'inferenza dalla denominazione" riduce i falsi positivi che altrimenti costerebbero all'autore un round trip.

**Convergenza di re-revisione**: dichiara a Claude come comportarsi quando un PR è già stato revisionato. Una regola come "dopo la prima revisione, sopprimere i nuovi nit e pubblicare solo i risultati Importanti" impedisce a una correzione di una riga di raggiungere il round sette solo per lo stile.

**Forma di riepilogo**: chiedi al corpo della revisione di aprirsi con un conteggio di una riga come `2 fattuale, 4 stile`, e di iniziare con "nessun problema fattuale" quando è il caso. L'autore vuole conoscere la forma del lavoro prima dei dettagli.

<h4 id="example">
  Esempio
</h4>

Questo `REVIEW.md` ricalibrare la gravità per un servizio backend, limita i nit, salta i file generati e aggiunge controlli specifici del repository.

```markdown theme={null}
# Istruzioni di revisione

## Cosa significa Importante qui

Riserva Importante per i risultati che interromperebbero il comportamento, perderebbero dati,
o bloccherebbero un rollback: logica scorretta, query di database non scoped, PII
nei log o nei messaggi di errore, e migrazioni che non sono retrocompatibili.
Lo stile, la denominazione e i suggerimenti di refactoring sono Nit al massimo.

## Limita i nit

Segnala al massimo cinque Nit per revisione. Se ne hai trovati di più, dichiara "più N
elementi simili" nel riepilogo invece di pubblicarli inline. Se
tutto quello che hai trovato è un Nit, inizia il riepilogo con "Nessun problema bloccante."

## Non segnalare

- Qualsiasi cosa il CI già applica: lint, formattazione, errori di tipo
- File generati sotto `src/gen/` e qualsiasi file `*.lock`
- Codice solo per test che intenzionalmente viola le regole di produzione

## Controlla sempre

- Le nuove rotte API hanno un test di integrazione
- Le righe di log non includono indirizzi email, ID utente o corpi di richiesta
- Le query di database sono scoped al tenant del chiamante
```

<h4 id="keep-it-focused">
  Mantienilo focalizzato
</h4>

La lunghezza ha un costo: un lungo `REVIEW.md` diluisce le regole che contano di più. Mantienilo alle istruzioni che cambiano il comportamento di revisione, e lascia il contesto di progetto generale in `CLAUDE.md`.

<h2 id="view-usage">
  Visualizza l'utilizzo
</h2>

Vai a [claude.ai/analytics/code-review](https://claude.ai/analytics/code-review) per vedere l'attività di Code Review in tutta la tua organizzazione. Il dashboard mostra:

| Sezione              | Cosa mostra                                                                                                                  |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| PRs reviewed         | Conteggio giornaliero dei pull request revisionati nell'intervallo di tempo selezionato                                      |
| Cost weekly          | Spesa settimanale su Code Review                                                                                             |
| Feedback             | Conteggio dei commenti di revisione che sono stati risolti automaticamente perché uno sviluppatore ha affrontato il problema |
| Repository breakdown | Conteggi per repository dei PR revisionati e dei commenti risolti                                                            |

Le cifre di costo del dashboard sono stime per il monitoraggio dell'attività. Per la spesa accurata della fattura, fai riferimento alla tua fattura Anthropic.

<h2 id="pricing">
  Prezzi
</h2>

Code Review viene fatturato in base all'utilizzo dei token. Ogni revisione costa in media \$15-25, scalando con la dimensione del PR, la complessità del codebase e quanti problemi richiedono verifica. L'utilizzo di Code Review viene fatturato separatamente tramite [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) e non conta rispetto all'utilizzo incluso nel tuo piano.

Il trigger di revisione che scegli influisce sul costo totale:

* **Once after PR creation**: viene eseguito una volta per PR
* **After every push**: viene eseguito ad ogni push, moltiplicando il costo per il numero di push
* **Manual**: nessuna revisione su PR aperti o push, quindi il costo si accumula solo dalle revisioni che qualcuno richiede

In modalità Once after PR creation o Manual, commentando `@claude review always` [opta il PR nelle revisioni attivate da push](#manually-trigger-reviews), quindi il costo aggiuntivo si accumula per push dopo quel commento. In modalità After every push, i push già attivano revisioni, quindi l'abbonamento non cambia il costo per push. Commentando `@claude review` viene eseguita una singola revisione senza sottoscrivere ai push futuri. Claude esamina una [pull request da un fork](#review-pull-requests-from-forks) solo quando qualcuno commenta `@claude review`, quindi una pull request da fork non accumula mai costo per push in nessuna modalità.

I costi appaiono sulla tua fattura Anthropic indipendentemente dal fatto che la tua organizzazione utilizzi Amazon Bedrock o Google Cloud's Agent Platform per altre funzionalità di Claude Code. Per impostare un limite di spesa mensile per Code Review, vai a [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) e configura il limite per il servizio Claude Code Review.

Monitora la spesa tramite il grafico dei costi settimanali in [analytics](#view-usage) o la colonna del costo medio per repository nelle impostazioni di amministrazione.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Le esecuzioni di revisione sono best-effort. Un'esecuzione non riuscita non blocca mai il tuo PR, ma non si ritenta nemmeno automaticamente. Questa sezione copre come recuperare da un'esecuzione non riuscita e dove cercare quando il check run segnala problemi che non riesci a trovare.

<h3 id="retrigger-a-failed-or-timed-out-review">
  Riattiva una revisione non riuscita o scaduta
</h3>

Quando l'infrastruttura di revisione incontra un errore interno o supera il limite di tempo, il check run si completa con un titolo di **Code review encountered an error** o **Code review timed out**. La conclusione è ancora neutra, quindi nulla blocca il tuo merge, ma non vengono pubblicati risultati.

Per eseguire di nuovo la revisione, commenta `@claude review` sul PR. Questo avvia una revisione nuova senza sottoscrivere il PR ai push futuri. Se il PR non è [da un fork](#review-pull-requests-from-forks), puoi invece fare clic su **Re-run** sul check **Claude Code Review** nella scheda Checks di GitHub. Un re-run avvia anche una revisione nuova senza sottoscrivere il PR.

<h3 id="review-didn’t-run-and-the-pr-shows-a-spend-cap-message">
  La revisione non è stata eseguita e il PR mostra un messaggio di limite di spesa
</h3>

Quando il limite di spesa mensile dell'organizzazione viene raggiunto, Code Review pubblica un singolo commento sul PR spiegando che la revisione è stata saltata. Le revisioni riprendono automaticamente all'inizio del prossimo periodo di fatturazione, o immediatamente quando un amministratore aumenta il limite a [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

<h3 id="find-issues-that-aren’t-showing-as-inline-comments">
  Trova problemi che non vengono visualizzati come commenti inline
</h3>

Se il titolo del check run dice che sono stati trovati problemi ma non vedi commenti di revisione inline sul diff, cerca in questi altri luoghi dove vengono visualizzati i risultati:

* **Check run Details**: fai clic su **Details** accanto al check Claude Code Review nella scheda Checks. La tabella di gravità elenca ogni risultato con il suo file, riga e riepilogo indipendentemente dal fatto che il commento inline sia stato accettato.
* **Files changed annotations**: apri la scheda **Files changed** sul PR. I risultati vengono visualizzati come annotazioni allegate direttamente alle righe diff, separate dai commenti di revisione.
* **Review body**: se hai fatto un push al PR mentre una revisione era in esecuzione, alcuni risultati potrebbero fare riferimento a righe che non esistono più nel diff attuale. Questi appaiono sotto un'intestazione **Additional findings** nel testo del corpo della revisione piuttosto che come commenti inline.

<h2 id="review-a-diff-locally">
  Revisione di un diff localmente
</h2>

Il comando [`/code-review`](/docs/it/commands) esamina un diff nel tuo terminale senza installare l'app GitHub. Segnala bug di correttezza e riutilizzo, semplificazione e pulizie di efficienza.

`/review` è un alias di `/code-review`; prima della v2.1.223, era un comando separato che eseguiva una revisione a passaggio singolo, di sola lettura di una richiesta pull GitHub.

<Steps>
  <Step title="Esegui /code-review">
    Dalla sessione in cui stai lavorando, esegui il comando:

    ```text theme={null}
    /code-review
    ```

    Esamina i commit del tuo ramo in anticipo rispetto al suo upstream più eventuali modifiche non sottoposte a commit, quindi ha bisogno di lavoro sul ramo o nell'albero di lavoro per avere qualcosa da segnalare. Per esaminare qualcos'altro, passa un target: un percorso di file, un numero PR, un nome di ramo o un intervallo di ref come `main...my-feature`.

    Puoi anche aggiungere flag:

    * `--fix`: applica i risultati al tuo albero di lavoro dopo la revisione
    * `--comment`: pubblica i risultati su una richiesta pull GitHub come commenti inline, o su una richiesta di merge GitLab come una singola nota
    * `--post`: su una revisione cloud `ultra` di una richiesta pull `github.com`, preseleziona la pubblicazione dei risultati finiti al PR nella finestra di dialogo di avvio; vedi [Pubblica i risultati alla richiesta pull](/docs/it/ultrareview#post-findings-to-the-pull-request). Richiede Claude Code v2.1.227 o successivo

    Quando passi `--comment` per una richiesta di merge GitLab, Claude Code pubblica i risultati tramite la CLI `glab` di GitLab. Richiede Claude Code v2.1.257 o successivo. Quando `glab` non è installato, Claude stampa i risultati nel terminale.

    Passa la richiesta di merge come suo URL o un riferimento `!123`. Claude Code tratta un numero nudo o un nome di ramo come una richiesta di merge solo quando l'origine del checkout è su `gitlab.com`. Su un'istanza GitLab auto-gestita, passa l'URL o il modulo `!123`.
  </Step>

  <Step title="Continua a lavorare">
    La revisione viene eseguita come un [subagent](/docs/it/sub-agents) di background con la sua finestra di contesto, quindi non riempie la tua conversazione. I risultati arrivano nella tua conversazione quando la revisione si completa.
  </Step>

  <Step title="Agisci sui risultati">
    Chiedi a Claude di correggere ciò che la revisione ha trovato. Se hai passato `--fix` o `--comment`, la revisione ha già applicato o pubblicato i suoi risultati.
  </Step>
</Steps>

Claude segnala i risultati come testo nella risposta in entrambe queste esecuzioni, anche quando un'applicazione host richiede un elenco di risultati:

* In una sessione di terminale, dove `/code-review` esegue la revisione come un [subagent con fork](/docs/it/skills#run-skills-in-a-subagent)
* In un'esecuzione `-p` con output di testo o JSON

In un'applicazione host che richiede l'elenco dei risultati, come l'[app desktop](/docs/it/desktop), Claude segnala i risultati della revisione tramite lo strumento [`ReportFindings`](/docs/it/tools-reference). Claude Code esegue il rendering del rapporto come un elenco di risultati, e ogni voce mostra la posizione del file, un riassunto di una frase e un tag di categoria come `correctness` quando il risultato ne contiene uno. Una richiesta host si applica a ogni livello di sforzo e richiede Claude Code v2.1.218 o successivo.

Quando Claude corregge i risultati segnalati più tardi nella sessione, li segnala di nuovo, e Claude Code contrassegna ogni risultato nell'elenco dei risultati aggiornato come corretto, saltato o nessun cambiamento necessario.

<h3 id="what-the-review-reads-and-edits">
  Cosa legge e modifica la revisione
</h3>

La revisione segue il tuo `CLAUDE.md` come qualsiasi sessione Claude Code, ma non legge [`REVIEW.md`](#review-md). Una revisione di background applica le sue modifiche `--fix` al di fuori dei [checkpoint](/docs/it/checkpointing#subagent-edits-not-restored) della tua sessione, quindi `/rewind` non le annulla; usa git per ripristinarle. Quando la revisione [viene eseguita in primo piano](#run-in-the-foreground), modifica il tuo albero di lavoro durante il tuo turno, quindi `/rewind` ripristina le sue modifiche come al solito.

<h3 id="tune-effort-and-arguments">
  Regola lo sforzo e gli argomenti
</h3>

Passa un [livello di sforzo](/docs/it/model-config#adjust-effort-level) per scambiare copertura con confidenza. A `low` e `medium`, la revisione segnala solo i risultati di cui è più sicura, quindi vedi meno falsi positivi; `high` fino a `max` ampliano la copertura e possono includere risultati di cui la revisione è meno sicura.

Quando non digiti un livello, la revisione riutilizza l'ultimo livello da `low` a `max` che hai digitato, anche in una sessione precedente, e Claude Code mostra un avviso come `Reusing high effort, the level you typed last time`. Digita un livello, come `/code-review high`, per cambiare ciò che i successivi eseguimenti riutilizzano; un livello che passi in un'esecuzione `-p` non interattiva non lo aggiorna. `ultra` non aggiorna né utilizza il livello ricordato. Se non hai mai digitato un livello, la revisione utilizza lo sforzo attuale della sessione. Prima della v2.1.223, un `/code-review` senza un livello utilizzava sempre lo sforzo attuale della sessione.

Dopo il livello di sforzo e i flag, Claude Code legge il resto della riga in uno dei due modi:

* **Senza `ultra`**: tutto ciò che rimane è il target della revisione, anche quando inizia con un altro nome di comando. `/code-review /fix-issue 123` esamina con `/fix-issue 123` come testo target invece di caricare `/fix-issue` come una seconda [skill in stack](/docs/it/skills#pass-arguments-to-skills). Prima della v2.1.218, un comando in stack dopo `/code-review` si espandeva come la sua propria skill.
* **Con `ultra`**: Claude Code legge una singola parola come un ramo base o un numero PR, e trasforma il testo più lungo che non nomina un ramo o PR in [una nota allegata alla revisione](/docs/it/ultrareview#pass-a-request-in-plain-words). `/code-review ultra check my auth changes` esamina il tuo ramo attuale, e Claude mette in relazione i risultati con la tua nota.

<h3 id="run-in-the-foreground">
  Esegui in primo piano
</h3>

La revisione viene eseguita in background per impostazione predefinita; prima della v2.1.218, veniva eseguita all'interno della tua conversazione. Viene eseguita in primo piano invece in casi come questi:

* Esegui `/code-review` di nuovo mentre una revisione precedente è ancora in corso
* Lo esegui in modalità non interattiva, con il flag `-p` o l'Agent SDK; Claude Code attende la revisione e include i risultati nella risposta, tranne per `ultra`, che [avvia la revisione cloud senza attendere](#escalate-to-ultrareview)
* Imposti [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/it/env-vars) su `1`, che disattiva anche tutte le altre funzioni di attività di background

<h3 id="let-claude-start-the-review">
  Lascia che Claude avvii la revisione
</h3>

Claude può avviare `/code-review` da solo. Chiedigli di esaminare le tue modifiche in linguaggio naturale e può eseguire la skill senza che tu digiti il comando, e un [compito programmato](/docs/it/scheduled-tasks) con `/code-review` come suo prompt esegue la revisione.

Un compito programmato non avvia mai la [revisione cloud](#escalate-to-ultrareview), quindi programma `/code-review` senza l'argomento `ultra`.

Per impedire sia a Claude che ai compiti programmati di avviare la revisione mantenendo `/code-review` disponibile per te da digitare, aggiungi una voce [`skillOverrides`](/docs/it/skills#override-skill-visibility-from-settings) a un [file di impostazioni](/docs/it/settings#where-settings-live) come `~/.claude/settings.json`:

```json theme={null}
{
  "skillOverrides": {
    "code-review": "user-invocable-only"
  }
}
```

Prima della v2.1.246, Claude avviava `/code-review` da solo solo dove un flag di funzionalità recuperato da Anthropic lo attivava. In [sessioni che non recuperano flag di funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching), `/code-review` veniva eseguito solo quando lo digitavi, e un `/code-review` programmato raggiungeva Claude come testo semplice.

<h3 id="escalate-to-ultrareview">
  Escalation a ultrareview
</h3>

`/code-review ultra --fix` esegue la [ultrareview](/docs/it/ultrareview) più profonda nel cloud, quindi applica i suoi risultati al tuo albero di lavoro quando tornano nella tua sessione.

Ultrareview utilizza il suo proprio ambito: il tuo ramo attuale rispetto al ramo predefinito del repository, più modifiche non sottoposte a commit e in staging nell'albero di lavoro. Per le modifiche non sottoposte a commit ai file denominati come credenziali o chiavi, come file `.env` e `*.tfvars`, Claude Code segue le regole per [caricare un repository locale in una sessione cloud](/docs/it/claude-code-on-the-web#send-local-repositories-without-github). Passa un nome di ramo, come `/code-review ultra develop`, per confrontare con una base diversa.

Quando il target è una richiesta pull `github.com`, puoi fare in modo che Claude [pubblichi i risultati finiti al PR](/docs/it/ultrareview#post-findings-to-the-pull-request) come commento dal tuo account GitHub. Richiede Claude Code v2.1.227 o successivo.

<Note>
  Ultrareview richiede l'autenticazione con un account claude.ai e non è disponibile su Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry, o per le organizzazioni con Zero Data Retention abilitato. Quando ultrareview non è disponibile, `/code-review ultra` esegue una revisione locale nella tua sessione.
</Note>

Per avviare una revisione cloud da uno script o CI, esegui `claude -p '/code-review ultra'`. Claude Code avvia la revisione e stampa un link per tracciarlo. Richiede Claude Code v2.1.218 o successivo.

Quando la revisione comporterebbe l'addebito di [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), Claude Code si ferma prima di avviare, perché la conferma di fatturazione ha bisogno di una sessione interattiva. Esegui il [sottocomando `claude ultrareview`](/docs/it/ultrareview#run-ultrareview-non-interactively); eseguendolo, accetti l'addebito.

Il comando era denominato `/simplify` prima della v2.1.147, quando applicava le correzioni per impostazione predefinita. `/simplify` esegue una revisione separata solo per la pulizia che applica le correzioni senza cercare bug. Se hai scritto script per `/simplify` per la ricerca di bug, passa a `/code-review --fix`.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Commands](/docs/it/commands): esegui `/code-review` in una sessione locale di Claude Code per controllare un diff prima di fare push
* [GitHub Actions](/docs/it/github-actions): esegui Claude nei tuoi flussi di lavoro GitHub Actions per l'automazione personalizzata oltre la revisione del codice
* [GitLab CI/CD](/docs/it/gitlab-ci-cd): integrazione Claude self-hosted per le pipeline GitLab
* [Memory](/docs/it/memory): come funzionano i file `CLAUDE.md` in Claude Code
* [Analytics](/docs/it/analytics): traccia l'utilizzo di Claude Code oltre la revisione del codice
* [How Anthropic secures its AI-native software development lifecycle](https://claude.com/blog/how-anthropic-secures-its-ai-native-software-development-lifecycle): come la revisione automatizzata si inserisce come uno strato del processo di sviluppo sicuro di Anthropic
