> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gateway LLM altri

> Instrada Claude Code attraverso un gateway LLM che la tua organizzazione già esegue. Copre il collegamento di Claude Code a un gateway, il rollout per la tua organizzazione e cosa Claude Code invia a un gateway.

Questa sezione copre l'utilizzo di un prodotto gateway che la tua organizzazione già esegue, piuttosto che [gateway di app Claude](/docs/it/claude-apps-gateway). Per informazioni su cosa sia un gateway, come si posiziona tra Claude Code e il tuo provider e come scegliere tra gateway di app Claude e un altro prodotto, vedi la [panoramica del gateway](/docs/it/gateways).

<Note>
  * Se sei uno sviluppatore che si connette a un gateway esistente: [collega Claude Code al tuo gateway](/docs/it/llm-gateway-connect)
  * Se sei un amministratore che esegue il rollout di un gateway per la tua organizzazione: [distribuisci e distribuisci un gateway](/docs/it/llm-gateway-rollout)
  * Se stai configurando un prodotto gateway: il [riferimento del protocollo gateway](/docs/it/llm-gateway-protocol)
</Note>

Qualsiasi gateway che espone un [formato API supportato](/docs/it/llm-gateway-protocol#api-formats) funziona. Anthropic non approva, mantiene o controlla i prodotti gateway di terze parti e non supporta l'instradamento di Claude Code a modelli non-Claude attraverso alcun gateway. Distribuisci il gateway seguendo la sua documentazione, quindi completa il lato Claude Code con i [passaggi di rollout di seguito](#roll-out-a-gateway).

<h2 id="what-a-gateway-provides">
  Cosa fornisce un gateway
</h2>

Un gateway offre alla tua organizzazione un unico posto per gestire:

* **Credenziali**: la chiave del provider rimane lato server; gli sviluppatori mantengono invece le credenziali del gateway
* **Tracciamento dell'utilizzo**: attribuisci l'utilizzo per sviluppatore o team, indipendentemente da quale provider serve la richiesta
* **Controlli dei costi**: applica budget e limiti di velocità in un unico posto
* **Registrazione di audit**: registra ogni richiesta di modello per la conformità
* **Cambio di provider**: cambia il provider nella configurazione del gateway, senza toccare le macchine degli sviluppatori

Tutti questi, tranne il cambio di provider, si applicano sia che l'upstream sia l'API di Anthropic che un [provider cloud](/docs/it/third-party-integrations). Il cambio di provider senza riconfigurare le macchine degli sviluppatori dipende anche dal gateway che espone un singolo [endpoint in formato Anthropic](/docs/it/llm-gateway-protocol#api-formats) indipendentemente dall'upstream; un gateway che espone il formato proprio di un provider lega la configurazione del client a quel provider.

Il compromesso è che il gateway diventa un'infrastruttura che la tua organizzazione gestisce. Claude Code aggiunge funzionalità con ogni rilascio, e un gateway che non le inoltra interrompe le funzionalità corrispondenti, quindi il prodotto gateway deve essere mantenuto aggiornato man mano che Claude Code evolve. Il [riferimento del protocollo gateway](/docs/it/llm-gateway-protocol) copre cosa inoltrare.

<h2 id="roll-out-a-gateway">
  Esegui il rollout di un gateway
</h2>

Quando sei pronto a eseguire il rollout di un gateway LLM alla tua organizzazione, la sequenza è la stessa indipendentemente dal prodotto gateway che scegli:

1. Distribuisci il gateway e dagli la tua credenziale del provider, in modo che possa autenticare le richieste che inoltra.
2. Emetti a ogni sviluppatore una credenziale del gateway, in modo che l'utilizzo sia attribuito allo sviluppatore e l'offboarding revoca una credenziale.
3. Distribuisci la configurazione attraverso un [file di impostazioni gestite](/docs/it/settings#settings-files) e il tuo strumento di segreti, in modo che ogni macchina riceva l'URL di base e una credenziale. Quando entrambi sono distribuiti, gli sviluppatori non configurano nulla. Se non hai la distribuzione delle impostazioni in atto, gli sviluppatori seguono la [pagina di connessione](/docs/it/llm-gateway-connect) per impostare le variabili stessi.
4. Fai in modo che ogni sviluppatore [verifichi la configurazione in Claude Code](/docs/it/llm-gateway-connect#check-for-an-existing-configuration), in modo che i problemi di distribuzione emergano prima che dipendano dal gateway.

[Esegui il rollout di un gateway LLM per la tua organizzazione](/docs/it/llm-gateway-rollout) esamina ogni passaggio e mostra i file di configurazione da distribuire a ogni passaggio. Il gateway è una parte della configurazione dell'organizzazione; per l'applicazione delle politiche, la visibilità dell'utilizzo e le decisioni sulla gestione dei dati, vedi [Configura Claude Code per la tua organizzazione](/docs/it/admin-setup).

<h2 id="subscriptions-and-gateways">
  Abbonamenti e gateway
</h2>

Mentre una [variabile di credenziale del gateway](/docs/it/llm-gateway-connect#set-the-credential-variable) o `apiKeyHelper` è attiva, l'abbonamento a claude.ai di uno sviluppatore non viene utilizzato: la credenziale sostituisce l'accesso all'abbonamento per quella sessione, e i limiti di utilizzo dell'abbonamento non si applicano. Quel traffico viene fatturato per token a chiunque possieda la credenziale che il gateway inoltra, come l'account Anthropic Console della tua organizzazione, o il tuo account Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry quando il gateway instrada lì.

[`ANTHROPIC_BASE_URL`](/docs/it/llm-gateway-connect#set-the-base-url-and-credential) è la variabile che punta Claude Code al gateway. Impostare solo quella variabile, senza una credenziale del gateway, non sostituisce l'abbonamento. Le richieste vengono comunque instradate attraverso il gateway, ma un accesso a claude.ai salvato rimane la credenziale attiva, quindi i suoi limiti di utilizzo e la fatturazione si applicano. I gateway che passano questo traffico ad Anthropic devono inoltrare la capacità OAuth in `anthropic-beta`; vedi il [riferimento delle intestazioni della richiesta](/docs/it/llm-gateway-protocol#request-headers).

<h2 id="related-pages">
  Pagine correlate
</h2>

* [Panoramica del gateway](/docs/it/gateways): come funziona un gateway e come scegliere tra gateway di app Claude e un altro prodotto
* [Gateway di app Claude](/docs/it/claude-apps-gateway): il gateway auto-ospitato di Anthropic con accesso SSO e telemetria OTLP
* [Collega Claude Code a un gateway LLM](/docs/it/llm-gateway-connect): imposta l'URL di base e la credenziale sulla tua macchina, con configurazione per superficie e una tabella di risoluzione dei problemi
* [Esegui il rollout di un gateway LLM per la tua organizzazione](/docs/it/llm-gateway-rollout): la checklist dell'amministratore per distribuire un gateway, emettere credenziali sviluppatore e distribuire impostazioni gestite
* [Riferimento del protocollo gateway](/docs/it/llm-gateway-protocol): cosa Claude Code invia a un gateway, per gli operatori che ne configurano uno, coprendo endpoint, intestazioni da inoltrare e pass-through delle funzionalità
