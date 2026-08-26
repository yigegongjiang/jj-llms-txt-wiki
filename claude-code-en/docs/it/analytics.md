> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Traccia l'utilizzo del team con l'analittica

> Visualizza le metriche di utilizzo di Claude Code, traccia l'adozione e misura la velocità di ingegneria nel dashboard di analittica.

Claude Code fornisce dashboard di analittica per aiutare le organizzazioni a comprendere i modelli di utilizzo degli sviluppatori, tracciare le metriche di contributo e misurare come Claude Code influisce sulla velocità di ingegneria. Accedi al dashboard per il tuo piano:

| Piano                         | URL Dashboard                                                              | Include                                                                                              | Leggi di più                                          |
| ----------------------------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| Claude for Teams / Enterprise | [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code) | Metriche di utilizzo, metriche di contributo con integrazione GitHub, leaderboard, esportazione dati | [Dettagli](#access-analytics-for-team-and-enterprise) |
| API (Claude Console)          | [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | Metriche di utilizzo, tracciamento della spesa, approfondimenti del team                             | [Dettagli](#access-analytics-for-api-customers)       |

<h2 id="access-analytics-for-team-and-enterprise">
  Access analytics for Team and Enterprise
</h2>

Accedi a [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code). Gli amministratori e i proprietari possono visualizzare il dashboard.

Il dashboard di Team e Enterprise include:

* **Metriche di utilizzo**: righe di codice accettate, tasso di accettazione dei suggerimenti, utenti attivi giornalieri e sessioni
* **Metriche di contributo**: PR e righe di codice spedite con assistenza di Claude Code, con [integrazione GitHub](#enable-contribution-metrics)
* **Leaderboard**: i principali contributori classificati per utilizzo di Claude Code
* **Esportazione dati**: scarica i dati di contributo come CSV per report personalizzati

Per i conteggi dei token per utente e le stime dei costi, configura [esportazione OpenTelemetry](/docs/it/monitoring-usage), oppure esporta il [report di spesa](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) dalle impostazioni di analittica della tua organizzazione, che elenca l'utilizzo dei token e la spesa stimata in crediti di utilizzo per utente e per modello.

<h3 id="enable-contribution-metrics">
  Enable contribution metrics
</h3>

<Note>
  Le metriche di contributo sono in beta pubblica e disponibili sui piani Claude for Teams e Claude for Enterprise. Queste metriche coprono solo gli utenti all'interno della tua organizzazione claude.ai. L'utilizzo tramite l'API Claude Console o integrazioni di terze parti non è incluso.
</Note>

I dati di utilizzo e adozione sono disponibili per tutti gli account Claude for Teams e Claude for Enterprise. Le metriche di contributo richiedono una configurazione aggiuntiva per connettere la tua organizzazione GitHub.

Hai bisogno del ruolo di proprietario per configurare le impostazioni di analittica. Un amministratore GitHub deve installare l'app GitHub.

<Warning>
  Le metriche di contributo non sono disponibili per le organizzazioni con [Zero Data Retention](/docs/it/zero-data-retention) abilitato. Il dashboard di analittica mostrerà solo le metriche di utilizzo.
</Warning>

<Steps>
  <Step title="Installa l'app GitHub">
    Un amministratore GitHub installa l'app Claude GitHub sull'account GitHub della tua organizzazione su [github.com/apps/claude](https://github.com/apps/claude).
  </Step>

  <Step title="Abilita l'analittica di Claude Code">
    Un proprietario Claude accede a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) e abilita la funzione di analittica di Claude Code.
  </Step>

  <Step title="Abilita l'analittica di GitHub">
    Sulla stessa pagina, abilita l'interruttore "GitHub analytics".
  </Step>

  <Step title="Autentica con GitHub">
    Completa il flusso di autenticazione GitHub e seleziona quali organizzazioni GitHub includere nell'analisi.
  </Step>
</Steps>

I dati in genere appaiono entro 24 ore dopo l'abilitazione, con aggiornamenti giornalieri. Se non appaiono dati, potresti visualizzare uno di questi messaggi:

* **"GitHub app required"**: installa l'app GitHub per visualizzare le metriche di contributo
* **"Data processing in progress"**: torna tra qualche giorno e conferma che l'app GitHub è installata se i dati non appaiono

Le metriche di contributo supportano GitHub Cloud e GitHub Enterprise Server.

<h3 id="review-summary-metrics">
  Review summary metrics
</h3>

<Note>
  Queste metriche sono deliberatamente conservative e rappresentano una sottostima dell'impatto effettivo di Claude Code. Solo le righe e i PR dove c'è un'alta confidenza nel coinvolgimento di Claude Code vengono conteggiati.
</Note>

Il dashboard visualizza queste metriche di riepilogo in alto:

* **PRs with CC**: conteggio totale delle pull request unite che contengono almeno una riga di codice scritta con Claude Code
* **Lines of code with CC**: righe totali di codice in tutti i PR uniti che sono stati scritti con assistenza di Claude Code. Solo le "righe effettive" vengono conteggiate: righe con più di 3 caratteri dopo la normalizzazione, escludendo righe vuote e righe con solo parentesi o punteggiatura banale.
* **PRs with Claude Code (%)**: percentuale di tutti i PR uniti che contengono codice assistito da Claude Code
* **Suggestion accept rate**: percentuale di volte in cui gli utenti accettano i suggerimenti di modifica del codice di Claude Code, incluso l'utilizzo degli strumenti Edit, Write e NotebookEdit
* **Lines of code accepted**: righe totali di codice scritte da Claude Code che gli utenti hanno accettato nelle loro sessioni. Questo esclude i suggerimenti rifiutati e non traccia le eliminazioni successive.

<h3 id="explore-the-charts">
  Explore the charts
</h3>

Il dashboard include diversi grafici per visualizzare le tendenze nel tempo.

<h4 id="track-adoption">
  Track adoption
</h4>

Il grafico di adozione mostra le tendenze di utilizzo giornaliero:

* **users**: utenti attivi giornalieri
* **sessions**: numero di sessioni attive di Claude Code al giorno

<h4 id="measure-prs-per-user">
  Measure PRs per user
</h4>

Questo grafico visualizza l'attività dei singoli sviluppatori nel tempo:

* **PRs per user**: numero totale di PR uniti al giorno diviso per utenti attivi giornalieri
* **users**: utenti attivi giornalieri

Usa questo per comprendere come la produttività individuale cambia man mano che aumenta l'adozione di Claude Code.

<h4 id="view-pull-requests-breakdown">
  View pull requests breakdown
</h4>

Il grafico Pull requests mostra una suddivisione giornaliera dei PR uniti:

* **PRs with CC**: pull request contenenti codice assistito da Claude Code
* **PRs without CC**: pull request senza codice assistito da Claude Code

Attiva la visualizzazione **Lines of code** per vedere la stessa suddivisione per righe di codice anziché per conteggio di PR.

<h4 id="find-top-contributors">
  Find top contributors
</h4>

La leaderboard mostra i 10 utenti principali classificati per volume di contributo. Attiva tra:

* **Pull requests**: mostra PR con Claude Code rispetto a tutti i PR per ogni utente
* **Lines of code**: mostra righe con Claude Code rispetto a tutte le righe per ogni utente

Fai clic su **Export all users** per scaricare i dati di contributo completi per tutti gli utenti come file CSV. L'esportazione include tutti gli utenti, non solo i 10 principali visualizzati.

<h3 id="pr-attribution">
  PR attribution
</h3>

Quando le metriche di contributo sono abilitate, Claude Code analizza le pull request unite per determinare quale codice è stato scritto con assistenza di Claude Code. Questo viene fatto abbinando l'attività della sessione di Claude Code al codice in ogni PR.

<h4 id="tagging-criteria">
  Tagging criteria
</h4>

I PR sono etichettati come "with Claude Code" se contengono almeno una riga di codice scritta durante una sessione di Claude Code. Il sistema utilizza un abbinamento conservativo: solo il codice dove c'è un'alta confidenza nel coinvolgimento di Claude Code viene conteggiato come assistito.

<h4 id="attribution-process">
  Attribution process
</h4>

Quando una pull request viene unita:

1. Le righe aggiunte vengono estratte dal diff del PR
2. Le sessioni di Claude Code che hanno modificato i file corrispondenti entro una finestra temporale vengono identificate
3. Le righe del PR vengono abbinate all'output di Claude Code utilizzando più strategie
4. Le metriche vengono calcolate per le righe assistite da AI e le righe totali

Prima del confronto, le righe vengono normalizzate: gli spazi bianchi vengono eliminati, gli spazi multipli vengono compressi, le virgolette vengono standardizzate e il testo viene convertito in minuscolo.

Le pull request unite contenenti righe assistite da Claude Code sono etichettate come `claude-code-assisted` in GitHub.

<h4 id="time-window">
  Time window
</h4>

Le sessioni da 21 giorni prima a 2 giorni dopo la data di unione del PR vengono considerate per l'abbinamento dell'attribuzione.

<h4 id="excluded-files">
  Excluded files
</h4>

Alcuni file vengono automaticamente esclusi dall'analisi perché sono generati automaticamente:

* File di blocco: package-lock.json, yarn.lock, Cargo.lock e simili
* Codice generato: output Protobuf, artefatti di build, file minificati
* Directory di build: dist/, build/, node\_modules/, target/
* Fixture di test: snapshot, cassette, dati mock
* Righe superiori a 1.000 caratteri, che probabilmente sono minificate o generate

<h4 id="attribution-notes">
  Attribution notes
</h4>

Tieni presenti questi dettagli aggiuntivi quando interpreti i dati di attribuzione:

* Il codice sostanzialmente riscritto dagli sviluppatori, con una differenza superiore al 20%, non viene attribuito a Claude Code
* Le sessioni al di fuori della finestra di 21 giorni non vengono considerate
* L'algoritmo non considera il ramo di origine o di destinazione del PR quando esegue l'attribuzione

<h3 id="get-the-most-from-analytics">
  Get the most from analytics
</h3>

Usa le metriche di contributo per dimostrare il ROI, identificare i modelli di adozione e trovare i membri del team che possono aiutare gli altri a iniziare.

<h4 id="monitor-adoption">
  Monitor adoption
</h4>

Traccia il grafico di adozione e i conteggi degli utenti per identificare:

* Utenti attivi che possono condividere le migliori pratiche
* Tendenze di adozione complessiva nella tua organizzazione
* Cali nell'utilizzo che potrebbero indicare attrito o problemi

<h4 id="measure-roi">
  Measure ROI
</h4>

Le metriche di contributo aiutano a rispondere a "Vale la pena investire in questo strumento?" con dati dal tuo codebase:

* Traccia i cambiamenti nei PR per utente nel tempo man mano che aumenta l'adozione
* Confronta i PR e le righe di codice spedite con e senza Claude Code
* Usa insieme alle [metriche DORA](https://dora.dev/), alla velocità dello sprint o ad altri KPI di ingegneria per comprendere i cambiamenti derivanti dall'adozione di Claude Code

<h4 id="identify-power-users">
  Identify power users
</h4>

La leaderboard ti aiuta a trovare i membri del team con un'elevata adozione di Claude Code che possono:

* Condividere tecniche di prompt e flussi di lavoro con il team
* Fornire feedback su ciò che funziona bene
* Aiutare a integrare i nuovi utenti

<h4 id="access-data-programmatically">
  Access data programmatically
</h4>

Nel piano Enterprise, l'[API Claude Enterprise Analytics](https://platform.claude.com/docs/en/api/admin/analytics) restituisce report di coinvolgimento, utilizzo e costo per utente per la tua organizzazione su tutte le superfici Claude, incluso Claude Code. Un proprietario principale crea una chiave con l'ambito `read:analytics` su [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). L'API non è disponibile nel piano Teams.

Per interrogare i dati di contributo tramite GitHub, cerca i PR etichettati con `claude-code-assisted`.

<h2 id="access-analytics-for-api-customers">
  Access analytics for API customers
</h2>

I clienti API che utilizzano Claude Console possono accedere all'analittica su [platform.claude.com/claude-code](https://platform.claude.com/claude-code). Hai bisogno dell'autorizzazione UsageView per accedere al dashboard, che viene concessa ai ruoli Developer, Billing, Admin, Owner e Primary Owner. Per estrarre le stesse metriche giornaliere per utente a livello di programmazione, utilizza l'[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) con una chiave API Admin.

<Note>
  Le metriche di contributo con integrazione GitHub non sono attualmente disponibili per i clienti API. Il dashboard della console mostra solo le metriche di utilizzo e spesa.
</Note>

Il dashboard della console visualizza:

* **Lines of code accepted**: righe totali di codice scritte da Claude Code che gli utenti hanno accettato nelle loro sessioni. Questo esclude i suggerimenti rifiutati e non traccia le eliminazioni successive.
* **Suggestion accept rate**: percentuale di volte in cui gli utenti accettano l'utilizzo dello strumento di modifica del codice, inclusi gli strumenti Edit, Write e NotebookEdit.
* **Activity**: utenti attivi giornalieri e sessioni mostrate su un grafico.
* **Spend**: costi API giornalieri in dollari insieme al conteggio degli utenti.

<h3 id="view-team-insights">
  View team insights
</h3>

La tabella di approfondimenti del team mostra le metriche per utente:

* **Members**: tutti gli utenti che si sono autenticati a Claude Code. Gli utenti con chiave API vengono visualizzati per identificatore di chiave, gli utenti OAuth vengono visualizzati per indirizzo email.
* **Spend this month**: costi API totali per utente per il mese corrente.
* **Lines this month**: totale per utente delle righe di codice accettate per il mese corrente.

<Note>
  Le cifre di spesa nel dashboard della console sono stime a scopo di analittica. Per i costi effettivi, consulta la tua pagina di fatturazione.
</Note>

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Monitoring with OpenTelemetry](/docs/it/monitoring-usage): esporta metriche e eventi in tempo reale al tuo stack di osservabilità
* [Manage costs effectively](/docs/it/costs): imposta limiti di spesa e ottimizza l'utilizzo dei token
* [Permissions](/docs/it/permissions): configura ruoli e autorizzazioni
