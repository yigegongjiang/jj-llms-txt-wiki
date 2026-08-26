> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Configura revisioni automatiche dei PR che rilevano errori logici, vulnerabilità di sicurezza e regressioni utilizzando l'analisi multi-agente dell'intero codebase

<Note>
  Code Review è in anteprima di ricerca, disponibile per gli abbonamenti [Team e Enterprise](https://claude.ai/admin-settings/claude-code). Non è disponibile per le organizzazioni con [Zero Data Retention](/docs/it/zero-data-retention) abilitato.
</Note>

Code Review analizza i tuoi pull request su GitHub e pubblica i risultati come commenti inline sulle righe di codice dove ha trovato problemi. Una flotta di agenti specializzati esamina i cambiamenti del codice nel contesto dell'intero codebase, cercando errori logici, vulnerabilità di sicurezza, edge case interrotti e regressioni sottili.

I risultati sono contrassegnati per gravità e non approvano o bloccano il tuo PR, quindi i flussi di lavoro di revisione esistenti rimangono intatti. Puoi regolare cosa Claude segnala aggiungendo un file `CLAUDE.md` o `REVIEW.md` al tuo repository.

Per eseguire Claude nella tua infrastruttura CI invece di questo servizio gestito, vedi [GitHub Actions](/docs/it/github-actions) o [GitLab CI/CD](/docs/it/gitlab-ci-cd). Per i repository su un'istanza GitHub self-hosted, vedi [GitHub Enterprise Server](/docs/it/github-enterprise-server).

Questa pagina copre:

* [Come funzionano le revisioni](#how-reviews-work)
* [Configurazione](#set-up-code-review)
* [Attivazione manuale delle revisioni](#manually-trigger-reviews) con `@claude review` e `@claude review once`
* [Personalizzazione delle revisioni](#customize-reviews) con `CLAUDE.md` e `REVIEW.md`
* [Prezzi](#pricing)
* [Risoluzione dei problemi](#troubleshooting) esecuzioni non riuscite e commenti mancanti
* [Revisione di un diff localmente](#review-a-diff-locally) con il comando `/code-review`

<Note>
  Per esaminare un diff localmente nel tuo terminale senza installare l'app GitHub, esegui il comando `/code-review` in qualsiasi sessione Claude Code. Vedi [Revisione di un diff localmente](#review-a-diff-locally).
</Note>

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

Rispondere a un commento inline non richiede a Claude di rispondere o aggiornare il PR. Per agire su un risultato, correggi il codice e fai un push. Se il PR è sottoscritto alle revisioni attivate da push, l'esecuzione successiva risolve il thread quando il problema è corretto. Per richiedere una revisione nuova senza fare un push, commenta `@claude review once` come [commento PR di primo livello](#manually-trigger-reviews).

<h3 id="check-run-output">
  Output del check run
</h3>

Oltre ai commenti di revisione inline, ogni revisione popola il check run **Claude Code Review** che appare insieme ai tuoi check CI. Espandi il suo link **Details** per vedere un riepilogo di ogni risultato in un unico posto, ordinato per gravità:

| Gravità       | File:Riga                 | Problema                                                                                      |
| ------------- | ------------------------- | --------------------------------------------------------------------------------------------- |
| 🔴 Importante | `src/auth/session.ts:142` | L'aggiornamento del token corre in parallelo con il logout, lasciando sessioni stantie attive |
| 🟡 Nit        | `src/auth/session.ts:88`  | `parseExpiry` restituisce silenziosamente 0 su input malformato                               |

Ogni risultato appare anche come un'annotazione nella scheda **Files changed**, contrassegnato direttamente sulle righe diff rilevanti. I risultati importanti vengono visualizzati con un marcatore rosso, i nit con un avviso giallo e i bug pre-esistenti con un avviso grigio. Le annotazioni e la tabella di gravità vengono scritte nel check run indipendentemente dai commenti di revisione inline, quindi rimangono disponibili anche se GitHub rifiuta un commento inline su una riga che si è spostata.

Il check run si completa sempre con una conclusione neutra, quindi non blocca mai il merge attraverso le regole di protezione del ramo. Se vuoi bloccare i merge sui risultati di Code Review, leggi il breakdown della gravità dall'output del check run nel tuo CI. L'ultima riga del testo Details è un commento leggibile da macchina che il tuo flusso di lavoro può analizzare con `gh` e jq:

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
    Segui i prompt per installare l'app GitHub di Claude nella tua organizzazione GitHub. L'app richiede questi permessi del repository:

    * **Contents**: lettura e scrittura
    * **Issues**: lettura e scrittura
    * **Pull requests**: lettura e scrittura

    Code Review utilizza l'accesso in lettura ai contenuti e l'accesso in scrittura ai pull request. L'insieme di permessi più ampio supporta anche [GitHub Actions](/docs/it/github-actions) se lo abiliti in seguito.
  </Step>

  <Step title="Seleziona i repository">
    Scegli quali repository abilitare per Code Review. Se non vedi un repository, assicurati di aver dato all'app GitHub di Claude l'accesso durante l'installazione. Puoi aggiungere altri repository in seguito.
  </Step>

  <Step title="Imposta i trigger di revisione per repository">
    Dopo il completamento della configurazione, la sezione Code Review mostra i tuoi repository in una tabella. Per ogni repository, utilizza il dropdown **Review Behavior** per scegliere quando vengono eseguite le revisioni:

    * **Once after PR creation**: la revisione viene eseguita una volta quando un PR viene aperto o contrassegnato come pronto per la revisione
    * **After every push**: la revisione viene eseguita ad ogni push al ramo PR, catturando nuovi problemi mentre il PR si evolve e risolvendo automaticamente i thread quando correggi i problemi segnalati
    * **Manual**: le revisioni iniziano solo quando qualcuno [commenta `@claude review` o `@claude review once` su un PR](#manually-trigger-reviews); `@claude review` sottoscrive anche il PR alle revisioni su push successivi

    La revisione ad ogni push esegue il maggior numero di revisioni e costa di più. La modalità manuale è utile per i repository ad alto traffico dove vuoi optare per PR specifici nella revisione, o per iniziare a revisionare i tuoi PR solo quando sono pronti.
  </Step>
</Steps>

La tabella dei repository mostra anche il costo medio per revisione per ogni repository in base all'attività recente. Utilizza il menu delle azioni della riga per attivare o disattivare Code Review per repository, o per rimuovere completamente un repository.

Per verificare la configurazione, apri un PR di test. Se hai scelto un trigger automatico, un check run denominato **Claude Code Review** appare entro pochi minuti. Se hai scelto Manual, commenta `@claude review` sul PR per avviare la prima revisione. Se non appare alcun check run, conferma che il repository è elencato nelle tue impostazioni di amministrazione e che l'app GitHub di Claude ha accesso ad esso.

<h2 id="manually-trigger-reviews">
  Attiva manualmente le revisioni
</h2>

Due comandi di commento avviano una revisione su richiesta. Entrambi funzionano indipendentemente dal trigger configurato del repository, quindi puoi usarli per optare per PR specifici nella revisione in modalità Manual o per ottenere una re-revisione immediata in altre modalità.

| Comando               | Cosa fa                                                                                        |
| :-------------------- | :--------------------------------------------------------------------------------------------- |
| `@claude review`      | Avvia una revisione e sottoscrive il PR alle revisioni attivate da push da quel momento in poi |
| `@claude review once` | Avvia una singola revisione senza sottoscrivere il PR ai push futuri                           |

Usa `@claude review once` quando vuoi feedback sullo stato attuale di un PR ma non vuoi che ogni push successivo comporti una revisione. Questo è utile per i PR di lunga durata con push frequenti, o quando vuoi un secondo parere una tantum senza cambiare il comportamento di revisione del PR.

Affinché uno dei due comandi attivi una revisione:

* Postalo come commento PR di primo livello, non come commento inline su una riga diff
* Metti il comando all'inizio del commento, con `once` sulla stessa riga se stai usando la forma one-shot
* Devi avere accesso come proprietario, membro o collaboratore al repository
* Il PR deve essere aperto

A differenza dei trigger automatici, i trigger manuali vengono eseguiti su PR bozza, poiché una richiesta esplicita segnala che vuoi la revisione ora indipendentemente dallo stato di bozza.

Se una revisione è già in esecuzione su quel PR, la richiesta viene messa in coda fino al completamento della revisione in corso. Puoi monitorare l'avanzamento tramite il check run sul PR.

<h2 id="customize-reviews">
  Personalizza le revisioni
</h2>

Code Review legge due file dal tuo repository per guidare cosa segnala. Differiscono nel modo in cui influenzano fortemente la revisione:

* **`CLAUDE.md`**: istruzioni di progetto condivise che Claude Code utilizza per tutti i compiti, non solo le revisioni. Code Review lo legge come contesto di progetto e segnala le violazioni appena introdotte come nit.
* **`REVIEW.md`**: istruzioni solo per la revisione, iniettate direttamente in ogni agente nella pipeline di revisione come priorità più alta. Usalo per cambiare cosa viene segnalato, a quale gravità e come vengono segnalati i risultati.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review legge i file `CLAUDE.md` del tuo repository e tratta le violazioni appena introdotte come risultati a [livello di nit](#severity-levels). Questo funziona bidirezionalmente: se il tuo PR cambia il codice in un modo che rende una dichiarazione `CLAUDE.md` obsoleta, Claude segnala che i documenti devono essere aggiornati anche loro.

Claude legge i file `CLAUDE.md` a ogni livello della gerarchia di directory, quindi le regole nel `CLAUDE.md` di una sottodirectory si applicano solo ai file sotto quel percorso. Vedi la [documentazione della memoria](/docs/it/memory) per ulteriori informazioni su come funziona `CLAUDE.md`.

Per una guida specifica della revisione che non vuoi applicata alle sessioni generali di Claude Code, usa [`REVIEW.md`](#review-md) invece.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` è un file alla radice del tuo repository che sostituisce il modo in cui Code Review si comporta sul tuo repository. I suoi contenuti vengono iniettati nel prompt di sistema di ogni agente nella pipeline di revisione come blocco di istruzioni con priorità più alta, prevalendo sulla guida di revisione predefinita.

Poiché viene incollato verbatim, `REVIEW.md` è istruzioni semplici: la [sintassi `@` import](/docs/it/memory#import-additional-files) non viene espansa e i file referenziati non vengono letti nel prompt. Metti le regole che vuoi applicate direttamente nel file.

<h4 id="what-you-can-tune">
  Cosa puoi regolare
</h4>

`REVIEW.md` è markdown freeform, quindi qualsiasi cosa tu possa esprimere come istruzione di revisione è in ambito. I modelli di seguito hanno il maggior impatto nella pratica.

**Gravità**: ridefinisci cosa significa 🔴 Importante per il tuo repository. La calibrazione predefinita è mirata al codice di produzione; un repository di documenti, un repository di configurazione o un prototipo potrebbe volere una definizione molto più ristretta. Dichiara esplicitamente quali classi di risultato sono Importanti e quali sono Nit al massimo. Puoi anche escalare nell'altra direzione, ad esempio trattando qualsiasi violazione di `CLAUDE.md` come Importante piuttosto che il nit predefinito.

**Volume di nit**: limita quanti commenti 🟡 Nit una singola revisione pubblica. La prosa e i file di configurazione possono essere lucidati per sempre. Un limite come "segnala al massimo cinque nit, menziona il resto come conteggio nel riepilogo" mantiene le revisioni attuabili.

**Regole di salto**: elenca percorsi, modelli di ramo e categorie di risultati dove Claude non dovrebbe pubblicare risultati. I candidati comuni sono codice generato, lockfile, dipendenze vendute e rami creati da macchine, insieme a qualsiasi cosa il tuo CI già applica come linting o controllo ortografico. Per i percorsi che meritano una revisione ma non un controllo completo, imposta un bar più alto invece di saltare completamente: "in `scripts/`, segnala solo se quasi certo e grave."

**Controlli specifici del repository**: aggiungi regole che vuoi segnalate su ogni PR, come "le nuove rotte API devono avere un test di integrazione." Poiché `REVIEW.md` viene iniettato come priorità più alta, questi atterrano più affidabilmente rispetto alle stesse regole in un lungo `CLAUDE.md`.

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

La tabella dei repository nelle impostazioni di amministrazione mostra anche il costo medio per revisione per ogni repository. Le cifre di costo del dashboard sono stime per il monitoraggio dell'attività; per la spesa accurata della fattura, fai riferimento alla tua fattura Anthropic.

<h2 id="pricing">
  Prezzi
</h2>

Code Review viene fatturato in base all'utilizzo dei token. Ogni revisione costa in media \$15-25, scalando con la dimensione del PR, la complessità del codebase e quanti problemi richiedono verifica. L'utilizzo di Code Review viene fatturato separatamente tramite [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) e non conta rispetto all'utilizzo incluso nel tuo piano.

Il trigger di revisione che scegli influisce sul costo totale:

* **Once after PR creation**: viene eseguito una volta per PR
* **After every push**: viene eseguito ad ogni push, moltiplicando il costo per il numero di push
* **Manual**: nessuna revisione fino a quando qualcuno non commenta `@claude review` su un PR

In qualsiasi modalità, commentando `@claude review` [opta il PR nelle revisioni attivate da push](#manually-trigger-reviews), quindi il costo aggiuntivo si accumula per push dopo quel commento. Per eseguire una singola revisione senza sottoscrivere ai push futuri, commenta `@claude review once` invece.

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

Per eseguire di nuovo la revisione, commenta `@claude review once` sul PR. Questo avvia una revisione nuova senza sottoscrivere il PR ai push futuri. Se il PR è già sottoscritto alle revisioni attivate da push, fare un push di un nuovo commit avvia anche una nuova revisione.

Il pulsante **Re-run** nella scheda Checks di GitHub non riattiva Code Review. Usa il comando di commento o un nuovo push invece.

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

Il comando [`/code-review`](/docs/it/commands) esamina un diff nel tuo terminale senza installare l'app GitHub. Eseguilo in qualsiasi sessione Claude Code: segnala bug di correttezza e riutilizzo, semplificazione e pulizie di efficienza. Per impostazione predefinita, la revisione locale copre i commit del tuo ramo in anticipo rispetto al suo upstream più eventuali modifiche non sottoposte a commit nell'albero di lavoro. Passa `--comment` per pubblicare i risultati come commenti PR inline, o `--fix` per applicare i risultati al tuo albero di lavoro dopo la revisione.

I [livelli di sforzo](/docs/it/model-config#adjust-effort-level) inferiori restituiscono meno risultati, più affidabili, mentre `high` fino a `max` forniscono una copertura più ampia e possono includere risultati incerti. Senza un argomento di sforzo, la revisione utilizza lo sforzo attuale della sessione. Per esaminare qualcosa di diverso dal diff predefinito, passa un target: un percorso di file, un numero PR, un nome di ramo o un intervallo di ref come `main...my-feature`. La forma dell'intervallo di ref esamina il diff sottoposto a commit che una richiesta pull da `my-feature` in `main` conterrebbe, indipendentemente da come è configurato l'upstream del ramo.

`/code-review ultra --fix` esegue la [ultrareview](/docs/it/ultrareview) più profonda nel cloud, quindi applica i suoi risultati al tuo albero di lavoro quando tornano nella tua sessione. Ultrareview utilizza il suo proprio ambito: il tuo ramo attuale rispetto al ramo predefinito del repository, più eventuali modifiche non sottoposte a commit e in staging nell'albero di lavoro.

Il comando era denominato `/simplify` prima della v2.1.147, quando applicava le correzioni per impostazione predefinita. A partire dalla v2.1.154, `/simplify` esegue una revisione separata solo per la pulizia che applica le correzioni senza cercare bug. Se hai scritto script per `/simplify` per la ricerca di bug, passa a `/code-review --fix`, che rimane invariato.

<h2 id="related-resources">
  Risorse correlate
</h2>

Code Review è progettato per funzionare insieme al resto di Claude Code. Se vuoi eseguire revisioni localmente prima di aprire un PR, hai bisogno di una configurazione self-hosted, o vuoi approfondire come `CLAUDE.md` modella il comportamento di Claude in tutti gli strumenti, queste pagine sono buoni prossimi passi:

* [Commands](/docs/it/commands): esegui `/code-review` in una sessione locale di Claude Code per controllare un diff prima di fare push
* [GitHub Actions](/docs/it/github-actions): esegui Claude nei tuoi flussi di lavoro GitHub Actions per l'automazione personalizzata oltre la revisione del codice
* [GitLab CI/CD](/docs/it/gitlab-ci-cd): integrazione Claude self-hosted per le pipeline GitLab
* [Memory](/docs/it/memory): come funzionano i file `CLAUDE.md` in Claude Code
* [Analytics](/docs/it/analytics): traccia l'utilizzo di Claude Code oltre la revisione del codice
