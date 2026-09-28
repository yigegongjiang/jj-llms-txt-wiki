> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Accelera le risposte con la modalità veloce

> Ottieni risposte più veloci di Opus in Claude Code attivando la modalità veloce.

<Note>
  La modalità veloce è in [anteprima di ricerca](#research-preview). La funzione, i prezzi e la disponibilità potrebbero cambiare in base al feedback.
</Note>

La modalità veloce è una configurazione ad alta velocità per Claude Opus, che rende il modello fino a 2,5 volte più veloce a un costo per token più elevato. Attivala con `/fast` quando hai bisogno di velocità per il lavoro interattivo come l'iterazione rapida o il debug in tempo reale, e disattivala quando il costo è più importante della latenza.

La modalità veloce non è un modello diverso. Utilizza Claude Opus con una configurazione API diversa che dà priorità alla velocità rispetto all'efficienza dei costi. Ottieni la stessa qualità e capacità con risposte più veloci. La modalità veloce è supportata su Opus 5.5, Opus 5 e Opus 4.8. Non è disponibile su Sonnet, Haiku o altri modelli.

Opus 4.7 non supporta la modalità veloce, quindi il passaggio ad esso disattiva la modalità veloce. La modalità veloce per Opus 4.7 è stata deprecata il 25 giugno 2026 e rimossa il 24 luglio 2026.

Cosa sapere:

* Usa `/fast` per attivare/disattivare la modalità veloce in Claude Code CLI. L'[estensione VS Code](/docs/it/vs-code) offre un comando **Toggle fast mode** quando il modello selezionato supporta la modalità veloce. Claude Code salva tale attivazione/disattivazione nella tua impostazione [`fastMode`](#toggle-fast-mode).
* I prezzi della modalità veloce per MTok input/output sono \$8/\$40 su Opus 5.5 e \$10/\$50 su Opus 5 e Opus 4.8.
* Disponibile per gli utenti di Claude Code sui piani di abbonamento (Pro/Max/Team/Enterprise) e su Claude Console. Le organizzazioni Team ed Enterprise hanno bisogno che un Owner lo abiliti per primo, e le organizzazioni Console hanno bisogno che l'accesso sia fornito per primo, entrambi descritti in [Requisiti](#requirements).
* Per gli utenti di Claude Code sui piani di abbonamento (Pro/Max/Team/Enterprise), la modalità veloce è disponibile solo tramite crediti di utilizzo e non è inclusa nei limiti di velocità dell'abbonamento.

<h2 id="toggle-fast-mode">
  Attiva/disattiva la modalità veloce
</h2>

Attiva/disattiva la modalità veloce in uno di questi modi:

* Esegui `/fast`, premi Spazio per attivare o disattivare, quindi premi Invio per confermare
* Imposta `"fastMode": true` nel tuo [file di impostazioni utente](/docs/it/settings)

Per impostazione predefinita, la modalità veloce che attivi in una sessione interattiva persiste tra le sessioni. Puoi configurare la modalità veloce per ripristinarsi ogni sessione. Vedi [richiedi opt-in per sessione](#require-per-session-opt-in) per i dettagli.

Al di fuori di una [sessione cloud](#use-fast-mode-in-cloud-sessions), in [modalità non interattiva](/docs/it/headless) con il flag `-p`, `/fast` funziona solo in una sessione avviata con la modalità veloce nel suo valore [`--settings`](/docs/it/cli-reference#cli-flags), ad esempio `claude -p --settings '{"fastMode": true}'`; l'attivazione/disattivazione si applica quindi solo a quella sessione e non viene salvata come impostazione predefinita. In qualsiasi altra sessione non interattiva, il comando segnala che la modalità veloce non è disponibile. Il modulo `-p` richiede Claude Code v2.1.205 o successivo.

Puoi eseguire `/fast` mentre Claude sta lavorando, e Claude Code attiva/disattiva la modalità veloce senza aspettare che il turno termini. Claude Code completa il turno in esecuzione alla sua velocità originale, quindi il cambio di velocità ha effetto dal tuo turno successivo. Se il tuo modello attuale non supporta la modalità veloce, attivarla comporta anche il passaggio del tuo modello, e Claude Code utilizza il nuovo modello dalla sua prossima richiesta in quel turno.

Per la migliore efficienza dei costi, abilita la modalità veloce all'inizio di una sessione piuttosto che passare a metà conversazione. Vedi [comprendi il compromesso di costo](#understand-the-cost-tradeoff) per i dettagli.

Quando abiliti la modalità veloce:

* Se il tuo modello attuale non supporta la modalità veloce, Claude Code passa a Opus
* Vedrai un messaggio di conferma: "Fast mode ON"
* Un piccolo icona `↯` appare accanto al prompt mentre la modalità veloce è attiva
* Esegui `/fast` di nuovo in qualsiasi momento per verificare se la modalità veloce è attiva o disattiva

Opus 5.5 è il valore predefinito della modalità veloce in Claude Code v2.1.280 e successivo. Prima di v2.1.280, la modalità veloce predefinita era Opus 5 da v2.1.219, a Opus 4.8 su v2.1.154 fino a v2.1.218, e a Opus 4.7 su v2.1.142 fino a v2.1.153.

Quando disabiliti la modalità veloce con `/fast` di nuovo, rimani su Opus. Per passare a un modello diverso, usa `/model`.

<h3 id="switch-models-while-fast-mode-is-on">
  Cambia modello mentre la modalità veloce è attiva
</h3>

La modalità veloce segue i tuoi cambi di modello in entrambe le direzioni:

* **Cambia modello**: quando passi a un modello che non supporta la modalità veloce, Claude Code disattiva la modalità veloce. Questo include Opus 4.7; prima di v2.1.221, la modalità veloce rimase attiva dopo un passaggio a Opus 4.7 e l'API rifiutò le richieste.
* **Torna indietro**: tornare a un modello Opus supportato attiva di nuovo la modalità veloce quando la tua preferenza di modalità veloce salvata è attiva, la stessa preferenza da cui una nuova sessione inizia per impostazione predefinita. Un cambio di modello non attiva mai la modalità veloce per una sessione la cui preferenza salvata è disattiva, e con [opt-in per sessione](#require-per-session-opt-in) configurato, tornare indietro non attiva di nuovo la modalità veloce; esegui `/fast` per riattivarla.

Ogni volta che un cambio di modello attiva o disattiva la modalità veloce, Claude Code mostra una conferma `Fast mode ON` o `Fast mode OFF`, e l'icona `↯` appare mentre la modalità veloce è attiva. Questo vale sia che tu cambi con `/model`, con [`/config model=<model>`](/docs/it/settings), o da un dispositivo connesso tramite [Remote Control](/docs/it/remote-control).

Claude Code invia di nuovo lo stato della modalità veloce della sessione ai dispositivi connessi tramite Remote Control dopo un cambio di modello, una riconnessione, o un [controllo di disponibilità](#use-fast-mode-behind-proxies-and-llm-gateways) non riuscito.

<h3 id="use-fast-mode-in-cloud-sessions">
  Usa la modalità veloce in sessioni cloud
</h3>

La modalità veloce funziona in [sessioni cloud](/docs/it/claude-code-on-the-web) quando è disponibile nel tuo account, sia che la sessione sia eseguita su infrastruttura gestita da Anthropic o su un [runner self-hosted](/docs/it/self-hosted-environments). Richiede Claude Code v2.1.271 o successivo nell'ambiente della sessione.

Digita `/fast on` nella sessione per attivare la modalità veloce. Rimane attiva solo per quella sessione e non viene salvata come impostazione predefinita. I [requisiti](#requirements) si applicano anche nelle sessioni cloud.

<h2 id="understand-the-cost-tradeoff">
  Comprendi il compromesso di costo
</h2>

La modalità veloce ha un prezzo per token più elevato rispetto a Opus standard:

| Modello  | Input (MTok) | Output (MTok) |
| -------- | ------------ | ------------- |
| Opus 5.5 | \$8          | \$40          |
| Opus 5   | \$10         | \$50          |
| Opus 4.8 | \$10         | \$50          |

I prezzi della modalità veloce sono fissi su tutta la finestra di contesto di 1M token. Per il tasso Opus standard da confrontare, consulta il [riferimento sui prezzi di Claude](https://platform.claude.com/docs/it/about-claude/pricing).

La prima volta che abiliti la modalità veloce in una conversazione, paghi il prezzo completo del token di input non memorizzato nella cache della modalità veloce per l'intero contesto della conversazione. Più avanti sei nella conversazione, più questo costa, quindi abilitare la modalità veloce dall'inizio è più economico. Il costo si applica una volta per conversazione, quindi disattivare e riattivare la modalità veloce in seguito non lo ripete. Per il meccanismo, consulta [come la modalità veloce interagisce con la cache del prompt](/docs/it/prompt-caching#turning-on-fast-mode).

<h3 id="see-where-fast-mode-spend-appears">
  Vedi dove appare la spesa della modalità veloce
</h3>

Vedi la spesa della modalità veloce in un posto diverso a seconda di come hai effettuato l'accesso, quindi prima esegui [`/status`](/docs/it/commands) per verificare. Se mostra una riga `Login method` come `Claude Max account`, hai effettuato l'accesso con un abbonamento Claude. Se mostra invece una riga `API key`, le tue richieste vengono fatturate a un'organizzazione Claude Console.

* **Pro e Max**: paghi la modalità veloce dai tuoi crediti di utilizzo. Vai a [**Settings > Usage**](https://claude.ai/settings/usage) su claude.ai, dove la sezione **Usage credits** mostra quanto hai speso in crediti di utilizzo questo mese. Quella cifra include la modalità veloce ma non la separa singolarmente.
* **Team ed Enterprise**: la tua organizzazione paga l'utilizzo della modalità veloce dai suoi crediti di utilizzo. Per vedere la tua spesa in crediti di utilizzo, esegui [`/usage`](/docs/it/costs#check-your-usage-credits-spend). Per vedere dove la tua organizzazione vede quella spesa, consulta [Claude per Teams ed Enterprise](/docs/it/costs#claude-for-teams-and-enterprise).
* **Claude Console**: la tua organizzazione paga la modalità veloce insieme al resto dell'utilizzo della sua API. Sulle pagine [Usage](https://platform.claude.com/usage) e [Cost](https://platform.claude.com/cost) di Console, seleziona **Speed (Research Preview)** nel menu **Group by** per separare la modalità veloce dall'utilizzo a velocità standard. Vedi quell'opzione solo quando l'intervallo di date selezionato include l'utilizzo della modalità veloce.

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

* **Solo API Anthropic o abbonamento**: la modalità veloce è disponibile tramite l'API Anthropic Console e per i piani di abbonamento Claude utilizzando i crediti di utilizzo. Non è disponibile su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o Claude Platform su AWS. Le organizzazioni Console devono anche avere [accesso alla modalità veloce provisioning](#enable-fast-mode-for-your-organization).
* **Crediti di utilizzo abilitati per i piani di abbonamento**: su un piano Pro, Max, Team o Enterprise, il tuo account deve avere i [crediti di utilizzo](/docs/it/costs#add-usage-credits-to-your-subscription) abilitati, che consente la fatturazione oltre l'utilizzo incluso nel tuo piano. Finché non sono abilitati, `/fast` mostra "Fast mode requires usage credits". Come abilitarli dipende dal tuo piano:
  * Su Pro e Max, abilitali nella sezione **Usage credits** di [**Settings > Usage**](https://claude.ai/settings/usage) su claude.ai, oppure esegui `/usage-credits` per aprire quella pagina.
  * Su Team e Enterprise, un membro con accesso alla fatturazione li abilita per l'organizzazione in [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), e un membro senza accesso esegue `/usage-credits` per inviare una richiesta agli amministratori dell'organizzazione.

<Note>
  L'utilizzo della modalità veloce viene fatturato direttamente dai crediti di utilizzo, anche se hai un utilizzo rimanente nel tuo piano.
</Note>

* **Organizzazione Console a pagamento**: gli account Claude Console non utilizzano crediti di utilizzo e la tua organizzazione paga la modalità veloce per token insieme al resto dell'utilizzo dell'API. Nel piano Evaluation gratuito della Console, `/fast` mostra "Fast mode unavailable during evaluation. Please purchase credits." Per cancellarlo, acquista crediti nelle tue [impostazioni di fatturazione della Console](https://platform.claude.com/settings/billing).
* **Abilitazione del proprietario per Team e Enterprise**: la modalità veloce è disabilitata per impostazione predefinita per le organizzazioni Team e Enterprise. Un proprietario deve esplicitamente [abilitare la modalità veloce](#enable-fast-mode-for-your-organization) prima che gli utenti possano accedervi.

<Note>
  Quattro impostazioni organizzative possono bloccare l'attivazione della modalità veloce con `/fast`:

  * **Modalità veloce non abilitata**: se la modalità veloce non è stata abilitata per la tua organizzazione, l'attivazione della modalità veloce con `/fast` mostra "Fast mode has been disabled by your organization."
  * **Modalità veloce disattivata dalle impostazioni gestite**: se la tua organizzazione distribuisce [impostazioni gestite](/docs/it/managed-settings) che impostano [`fastMode: false`](/docs/it/settings-reference#fastmode), l'attivazione della modalità veloce con `/fast` mostra lo stesso messaggio "Fast mode has been disabled by your organization".
  * **Opt-in per sessione richiesto**: le impostazioni gestite che impostano [`fastModePerSessionOptIn: true`](#require-per-session-opt-in) rifiutano `/fast on` con lo stesso messaggio ovunque tranne in una sessione di terminale interattiva.
  * **Modello modalità veloce non consentito**: se l'elenco di consentiti [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione esclude il modello Opus della modalità veloce, l'attivazione viene rifiutata con "is not in your organization's allowed models". In una sessione già in esecuzione su un modello Opus consentito che supporta la modalità veloce, `/fast` abilita invece la modalità veloce sul tuo modello attuale senza cambiare modelli.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Abilita la modalità veloce per la tua organizzazione
</h3>

Dove abiliti la modalità veloce dipende da quale prodotto utilizza la tua organizzazione:

* **Console** (clienti API): un amministratore la abilita in [Preferenze Claude Code](https://platform.claude.com/claude-code/preferences). La modalità veloce è in [anteprima di ricerca](#research-preview), quindi la tua organizzazione deve anche avere accesso alla modalità veloce provisioning prima che le richieste di modalità veloce abbiano successo. Per ottenere l'accesso, contatta il tuo account manager o iscriviti alla lista d'attesa, come descritto in [modalità veloce sull'API Claude](https://platform.claude.com/docs/en/build-with-claude/fast-mode).

  Senza accesso provisioning, l'API rifiuta ogni richiesta di modalità veloce con un 429, e Claude Code tratta ogni rifiuto come un [limite di velocità della modalità veloce](#handle-rate-limits). A differenza del cooldown di un limite di velocità, i rifiuti continuano fino a quando l'accesso non viene provisioning.
* **Claude AI** (Team e Enterprise): un proprietario la abilita in [Admin Settings > Claude Code](https://claude.ai/admin-settings/claude-code)

Un'altra opzione per disabilitare completamente la modalità veloce è impostare `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Vedi [Variabili di ambiente](/docs/it/env-vars).

<h3 id="use-fast-mode-behind-proxies-and-llm-gateways">
  Usa la modalità veloce dietro proxy e gateway LLM
</h3>

Prima di offrire la modalità veloce, Claude Code verifica la disponibilità della modalità veloce della tua organizzazione con una richiesta diretta a `api.anthropic.com`. Il controllo non segue [`ANTHROPIC_BASE_URL`](/docs/it/llm-gateway-connect#set-the-base-url-and-credential), quindi su una rete che instrada il traffico Claude attraverso un [gateway LLM](/docs/it/llm-gateway) e blocca l'uscita diretta a `api.anthropic.com`, il controllo fallisce anche se le richieste di inferenza funzionano. Il controllo utilizza un [proxy HTTP](/docs/it/network-config#proxy-configuration) configurato, quindi un blocco di rete fallisce il controllo solo dove `api.anthropic.com` è irraggiungibile anche attraverso il proxy.

Quando il controllo fallisce, `/fast` segnala "Fast mode unavailable due to network connectivity issues", e le richieste vengono eseguite a velocità standard, anche quando la tua organizzazione ha la modalità veloce abilitata. Un controllo che ha avuto successo in passato continua a funzionare dal suo risultato memorizzato nella cache, quindi un controllo bloccato influisce principalmente sulle nuove installazioni.

Lo stesso messaggio di connettività appare su una rete aperta quando il controllo raggiunge `api.anthropic.com` ma presenta una credenziale che Anthropic rifiuta. Una sessione la cui chiave risolta è una credenziale emessa dal gateway, contenuta in [`ANTHROPIC_API_KEY`](/docs/it/llm-gateway-connect#set-the-base-url-and-credential) o prodotta da un [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper), invia il controllo con quella chiave, e la richiesta rifiutata viene segnalata come un errore di connettività.

Per ripristinare la modalità veloce, consenti l'uscita diretta a `api.anthropic.com` dove un blocco di rete è la causa, o imposta qualsiasi variabile che corrisponda a come il controllo fallisce:

* `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1` tratta un controllo fallito come disponibile e onora comunque una risposta "disabilitato dalla tua organizzazione". Usalo quando la tua rete rifiuta la connessione, o quando Anthropic rifiuta una credenziale del gateway; l'allowlisting non aiuta il caso della credenziale, poiché nulla è bloccato.
* `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` salta completamente il controllo. Usalo quando la tua rete intercetta la richiesta piuttosto che rifiutarla.

Due configurazioni di gateway segnalano "Fast mode has been disabled by your organization" piuttosto che il messaggio di connettività, anche quando la tua organizzazione ha la modalità veloce abilitata:

* Una sessione che si autentica con [`ANTHROPIC_AUTH_TOKEN`](/docs/it/llm-gateway-connect#set-the-base-url-and-credential) da sola salta il controllo: senza un accesso a claude.ai o una chiave API Anthropic, e senza un controllo precedentemente memorizzato nella cache, Claude Code tratta la modalità veloce come disabilitata dalla tua organizzazione senza inviare la richiesta.
* Un proxy che intercetta il controllo e risponde con la sua stessa pagina, ad esempio un proxy che ispeziona TLS restituendo una pagina di blocco HTTP 200, viene letto come una risposta che dice che la tua organizzazione ha la modalità veloce disabilitata.

In entrambi i casi, imposta `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` per ripristinare la modalità veloce. `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` non si applica a nessuno di questi casi, poiché bypassa solo i controlli falliti e entrambi questi producono una risposta disabilitata. L'allowlisting dell'uscita diretta non aiuta il caso del token bearer, che non invia mai la richiesta.

Le variabili influiscono solo sul controllo lato client. Quando la tua organizzazione ha la modalità veloce disabilitata, l'API rifiuta le richieste di modalità veloce indipendentemente dal fatto che siano impostate o meno. Un rifiuto dall'API rimane anche con una variabile skip impostata. Claude Code ritenta la richiesta rifiutata a velocità standard, disattiva la modalità veloce, e `/fast` segnala che la tua organizzazione ha disabilitato la modalità veloce.

L'impostazione di `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` sopprime anche il controllo di disponibilità. Senza un controllo precedentemente memorizzato nella cache, `/fast` segnala "Fast mode is currently unavailable"; entrambe le variabili skip ripristinano la modalità veloce in quella configurazione anche.

<h3 id="require-per-session-opt-in">
  Richiedi opt-in per sessione
</h3>

Per impostazione predefinita, la modalità veloce che un utente abilita in una sessione interattiva persiste tra le sessioni. Per modificare questo, imposta `fastModePerSessionOptIn` a `true` in qualsiasi [file di impostazioni](/docs/it/settings#where-settings-live), il che fa sì che ogni sessione inizi con la modalità veloce disattivata e richiede agli utenti di abilitarla esplicitamente con `/fast`. I proprietari sui piani [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) o [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) possono distribuirlo a livello organizzativo tramite [impostazioni gestite dal server](/docs/it/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Questo è utile per controllare i costi nelle organizzazioni in cui gli utenti eseguono più sessioni simultanee. La preferenza della modalità veloce dell'utente è ancora salvata, quindi rimuovere questa impostazione ripristina il comportamento persistente predefinito.

Quando le impostazioni gestite impostano la chiave, `/fast on` funziona solo in una sessione di terminale interattiva. Ovunque altro, inclusa la [modalità non interattiva](/docs/it/headless), l'[estensione VS Code](/docs/it/vs-code) e le [sessioni cloud](#use-fast-mode-in-cloud-sessions), viene rifiutato con un messaggio che la tua organizzazione ha disabilitato la modalità veloce.

<h2 id="handle-rate-limits">
  Gestisci i limiti di velocità
</h2>

La modalità veloce ha limiti di velocità separati da Opus standard. Tutti i modelli Opus supportati condividono un pool di limiti di velocità della modalità veloce: l'utilizzo su uno qualsiasi di essi attinge dagli stessi limiti. Quando raggiungi il limite di velocità della modalità veloce:

1. La modalità veloce torna automaticamente alla velocità standard
2. L'icona `↯` diventa grigia per indicare il raffreddamento
3. Continui a lavorare a velocità e prezzi standard
4. Quando il raffreddamento scade, la modalità veloce si riabilita automaticamente

Per disabilitare manualmente la modalità veloce invece di aspettare il raffreddamento, esegui `/fast` di nuovo.

Se esaurisci i crediti di utilizzo durante una sessione, Claude Code ritenta ogni richiesta della modalità veloce rifiutata a velocità e prezzi standard, quindi continui a lavorare e non c'è raffreddamento. Il modo in cui vedi il rifiuto dipende dal tipo di sessione:

* In una sessione interattiva, Claude Code mostra una notifica "Fast mode disabled · usage credits exhausted" e disattiva la modalità veloce per il resto della sessione. La tua preferenza di modalità veloce salvata non cambia; esegui `/fast` per riattivare la modalità veloce.
* In [modalità non interattiva](/docs/it/headless) con `--output-format stream-json` e tramite l'Agent SDK, Claude Code emette lo stesso testo sul flusso di messaggi come messaggio `system` con sottotipo `notification`, una volta per turno mentre sei senza crediti di utilizzo. La modalità veloce rimane attiva. Richiede Claude Code v2.1.221 o successivo.

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
