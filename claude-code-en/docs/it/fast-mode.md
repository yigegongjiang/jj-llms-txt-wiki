> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Accelera le risposte con la modalità veloce

> Ottieni risposte più veloci di Opus in Claude Code attivando la modalità veloce.

<Note>
  La modalità veloce è in [anteprima di ricerca](#research-preview). La funzione, i prezzi e la disponibilità potrebbero cambiare in base al feedback.
</Note>

La modalità veloce è una configurazione ad alta velocità per Claude Opus, che rende il modello fino a 2,5 volte più veloce a un costo per token più elevato. Attivala con `/fast` quando hai bisogno di velocità per il lavoro interattivo come l'iterazione rapida o il debug in tempo reale, e disattivala quando il costo è più importante della latenza.

La modalità veloce non è un modello diverso. Utilizza Claude Opus con una configurazione API diversa che dà priorità alla velocità rispetto all'efficienza dei costi. Ottieni la stessa qualità e capacità con risposte più veloci. La modalità veloce è supportata su Opus 4.8 e Opus 4.7. Non è disponibile su Sonnet, Haiku o altri modelli.

<Warning>
  La modalità veloce per Opus 4.7 è deprecata a partire dal 25 giugno 2026 e verrà rimossa il 24 luglio 2026. Dopo la rimozione, le richieste di modalità veloce su Opus 4.7 restituiscono un errore e non tornano a Opus 4.7 standard. Esegui la migrazione a Opus 4.8 per mantenere l'accelerazione.
</Warning>

Cosa sapere:

* Usa `/fast` per attivare/disattivare la modalità veloce in Claude Code CLI. La modalità veloce non è supportata nell'estensione VS Code.
* I prezzi della modalità veloce per MTok input/output sono \$10/\$50 su Opus 4.8 e \$30/\$150 su Opus 4.7.
* Disponibile per tutti gli utenti di Claude Code sui piani di abbonamento (Pro/Max/Team/Enterprise) e Claude Console.
* Per gli utenti di Claude Code sui piani di abbonamento (Pro/Max/Team/Enterprise), la modalità veloce è disponibile solo tramite crediti di utilizzo e non è inclusa nei limiti di velocità dell'abbonamento.

<h2 id="toggle-fast-mode">
  Attiva/disattiva la modalità veloce
</h2>

Attiva/disattiva la modalità veloce in uno di questi modi:

* Digita `/fast` e premi Tab per attivare o disattivare
* Imposta `"fastMode": true` nel tuo [file di impostazioni utente](/docs/it/settings)

Per impostazione predefinita, la modalità veloce che attivi in una sessione interattiva persiste tra le sessioni. In [modalità non interattiva](/docs/it/headless), con il flag `-p`, `/fast` funziona solo in una sessione avviata con la modalità veloce nel suo valore [`--settings`](/docs/it/cli-reference#cli-flags), ad esempio `claude -p --settings '{"fastMode": true}'`; l'attivazione/disattivazione si applica quindi solo a quella sessione e non viene salvata come impostazione predefinita, e in qualsiasi altra sessione non interattiva il comando segnala che la modalità veloce non è disponibile. Puoi configurare la modalità veloce per ripristinarsi ogni sessione. Vedi [richiedi opt-in per sessione](#require-per-session-opt-in) per i dettagli.

Per la migliore efficienza dei costi, abilita la modalità veloce all'inizio di una sessione piuttosto che passare a metà conversazione. Vedi [comprendi il compromesso di costo](#understand-the-cost-tradeoff) per i dettagli.

Quando abiliti la modalità veloce:

* Se sei su un modello diverso, Claude Code passa automaticamente a Opus
* Vedrai un messaggio di conferma: "Fast mode ON"
* Un piccolo icona `↯` appare accanto al prompt mentre la modalità veloce è attiva
* Esegui `/fast` di nuovo in qualsiasi momento per verificare se la modalità veloce è attiva o disattiva

Quando disabiliti la modalità veloce con `/fast` di nuovo, rimani su Opus. Il modello non torna al tuo modello precedente. Per passare a un modello diverso, usa `/model`.

Passare a un modello che non supporta la modalità veloce disattiva la modalità veloce. Tornare a un modello Opus supportato la attiva di nuovo quando la tua preferenza di modalità veloce salvata è attiva, la stessa preferenza da cui una nuova sessione inizia per impostazione predefinita. Con [opt-in per sessione](#require-per-session-opt-in) configurato, tornare indietro non attiva di nuovo la modalità veloce; esegui `/fast` per riattivarla. La modalità veloce non si attiva mai per una sessione la cui preferenza salvata è disattiva, e l'icona `↯` e la conferma `Fast mode ON` appaiono ogni volta che si attiva. Prima della v2.1.208, la modalità veloce rimase disattiva dopo che sei tornato indietro fino a quando non hai eseguito `/fast` di nuovo.

Opus 4.8 è il valore predefinito della modalità veloce in Claude Code v2.1.154 e successivo. Su v2.1.142 fino a v2.1.153, la modalità veloce predefinita è Opus 4.7.

<h2 id="understand-the-cost-tradeoff">
  Comprendi il compromesso di costo
</h2>

La modalità veloce ha un prezzo per token più elevato rispetto a Opus standard, con il moltiplicatore che varia in base al modello:

| Modello  | Input (MTok) | Output (MTok) |
| -------- | ------------ | ------------- |
| Opus 4.8 | \$10         | \$50          |
| Opus 4.7 | \$30         | \$150         |

I prezzi della modalità veloce sono fissi su tutta la finestra di contesto di 1M token. Per il tasso Opus standard da confrontare, consulta il [riferimento sui prezzi di Claude](https://platform.claude.com/docs/it/about-claude/pricing).

La prima volta che abiliti la modalità veloce in una conversazione, paghi il prezzo completo del token di input non memorizzato nella cache della modalità veloce per l'intero contesto della conversazione. Più avanti sei nella conversazione, più questo costa, quindi abilitare la modalità veloce dall'inizio è più economico. Il costo si applica una volta per conversazione, quindi disattivare e riattivare la modalità veloce in seguito non lo ripete. Per il meccanismo, consulta [come la modalità veloce interagisce con la cache del prompt](/docs/it/prompt-caching#turning-on-fast-mode).

<h2 id="decide-when-to-use-fast-mode">
  Decidi quando usare la modalità veloce
</h2>

La modalità veloce è migliore per il lavoro interattivo dove la latenza della risposta è più importante del costo:

* Iterazione rapida su modifiche del codice
* Sessioni di debug in tempo reale
* Lavoro sensibile al tempo con scadenze strette

La modalità standard è migliore per:

* Attività autonome lunghe dove la velocità è meno importante
* Elaborazione batch o pipeline CI/CD
* Carichi di lavoro sensibili ai costi

<h3 id="fast-mode-vs-effort-level">
  Modalità veloce rispetto al livello di sforzo
</h3>

La modalità veloce e il livello di sforzo influenzano entrambi la velocità di risposta, ma in modo diverso:

| Impostazione                    | Effetto                                                                                                |
| ------------------------------- | ------------------------------------------------------------------------------------------------------ |
| **Modalità veloce**             | Stessa qualità del modello, latenza inferiore, costo più elevato                                       |
| **Livello di sforzo inferiore** | Meno tempo di riflessione, risposte più veloci, potenzialmente qualità inferiore su attività complesse |

Puoi combinare entrambi: usa la modalità veloce con un [livello di sforzo](/docs/it/model-config#adjust-effort-level) inferiore per la massima velocità su attività semplici.

<h2 id="requirements">
  Requisiti
</h2>

La modalità veloce richiede tutti i seguenti elementi:

* **Solo API Anthropic o abbonamento**: la modalità veloce è disponibile tramite l'API Anthropic Console e per i piani di abbonamento Claude utilizzando i crediti di utilizzo. Non è disponibile su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o Claude Platform su AWS.
* **Crediti di utilizzo abilitati**: il tuo account deve avere i crediti di utilizzo abilitati, che consente la fatturazione oltre l'utilizzo incluso nel tuo piano. Per gli account individuali, abilita questo nelle tue [impostazioni di fatturazione della Console](https://platform.claude.com/settings/billing). Per Team e Enterprise, un amministratore deve abilitare i crediti di utilizzo per l'organizzazione.

<Note>
  L'utilizzo della modalità veloce viene fatturato direttamente dai crediti di utilizzo, anche se hai un utilizzo rimanente nel tuo piano. Ciò significa che i token della modalità veloce non contano rispetto all'utilizzo incluso nel tuo piano e vengono addebitati alla tariffa della modalità veloce dal primo token.
</Note>

* **Abilitazione del proprietario per Team e Enterprise**: la modalità veloce è disabilitata per impostazione predefinita per le organizzazioni Team e Enterprise. Un proprietario deve esplicitamente [abilitare la modalità veloce](#enable-fast-mode-for-your-organization) prima che gli utenti possano accedervi.

<Note>
  Se la modalità veloce non è stata abilitata per la tua organizzazione, il comando `/fast` mostrerà "Fast mode has been disabled by your organization." Se l'elenco di consentiti [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione esclude il modello Opus della modalità veloce, `/fast` viene rifiutato con "is not in your organization's allowed models". L'eccezione è una sessione già in esecuzione su un modello Opus consentito che supporta la modalità veloce: `/fast` abilita la modalità veloce sul tuo modello attuale invece di cambiare modelli.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Abilita la modalità veloce per la tua organizzazione
</h3>

Dove abiliti la modalità veloce dipende da quale prodotto utilizza la tua organizzazione:

* **Console** (clienti API): un amministratore la abilita in [Preferenze Claude Code](https://platform.claude.com/claude-code/preferences)
* **Claude AI** (Team e Enterprise): un proprietario la abilita in [Admin Settings > Claude Code](https://claude.ai/admin-settings/claude-code)

Un'altra opzione per disabilitare completamente la modalità veloce è impostare `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Vedi [Variabili di ambiente](/docs/it/env-vars).

<h3 id="require-per-session-opt-in">
  Richiedi opt-in per sessione
</h3>

Per impostazione predefinita, la modalità veloce che un utente abilita in una sessione interattiva persiste tra le sessioni: rimane attiva nelle sessioni future. Per modificare questo, imposta `fastModePerSessionOptIn` a `true` in qualsiasi [file di impostazioni](/docs/it/settings#settings-files), il che fa sì che ogni sessione inizi con la modalità veloce disattivata e richiede agli utenti di abilitarla esplicitamente con `/fast`. I proprietari sui piani [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) o [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) possono distribuirlo a livello organizzativo tramite [impostazioni gestite dal server](/docs/it/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Questo è utile per controllare i costi nelle organizzazioni in cui gli utenti eseguono più sessioni simultanee. Gli utenti possono comunque abilitare la modalità veloce con `/fast` quando hanno bisogno di velocità, ma si ripristina all'inizio di ogni nuova sessione. La preferenza della modalità veloce dell'utente è ancora salvata, quindi rimuovere questa impostazione ripristina il comportamento persistente predefinito.

<h2 id="handle-rate-limits">
  Gestisci i limiti di velocità
</h2>

La modalità veloce ha limiti di velocità separati da Opus standard. La modalità veloce su Opus 4.8 e Opus 4.7 condividono lo stesso pool di limiti di velocità: l'utilizzo su uno qualsiasi di essi attinge dagli stessi limiti. Quando raggiungi il limite di velocità della modalità veloce o esaurisci i crediti di utilizzo:

1. La modalità veloce torna automaticamente alla velocità standard
2. L'icona `↯` diventa grigia per indicare il raffreddamento
3. Continui a lavorare a velocità e prezzi standard
4. Quando il raffreddamento scade, la modalità veloce si riabilita automaticamente

Per disabilitare manualmente la modalità veloce invece di aspettare il raffreddamento, esegui `/fast` di nuovo.

<h2 id="research-preview">
  Anteprima di ricerca
</h2>

La modalità veloce è una funzione di anteprima di ricerca. Ciò significa:

* La funzione potrebbe cambiare in base al feedback
* La disponibilità e i prezzi sono soggetti a modifiche
* La configurazione API sottostante potrebbe evolversi

Segnala problemi o feedback attraverso i tuoi soliti canali di supporto Anthropic.

<h2 id="see-also">
  Vedi anche
</h2>

* [Configurazione del modello](/docs/it/model-config): cambia modelli e regola i livelli di sforzo
* [Gestisci i costi in modo efficace](/docs/it/costs): traccia l'utilizzo dei token e riduci i costi
* [Configurazione della riga di stato](/docs/it/statusline): visualizza le informazioni del modello e del contesto
