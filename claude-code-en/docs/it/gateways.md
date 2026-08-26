> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eseguire Claude Code attraverso un gateway

> Instrada Claude Code attraverso un gateway auto-ospitato per credenziali centralizzate, tracciamento dell'utilizzo e controlli dei costi. Copre l'architettura, il gateway delle app Claude di Anthropic e l'utilizzo di altri prodotti gateway.

Un gateway è un proxy che la tua organizzazione esegue tra Claude Code e un provider di modelli. Claude Code invia il traffico API al gateway invece di inviarlo direttamente al provider, e il gateway lo inoltra utilizzando una credenziale che la tua organizzazione possiede. Gli sviluppatori si autenticano al gateway piuttosto che detenere credenziali del provider, quindi l'autenticazione, il tracciamento dell'utilizzo, i budget e la registrazione di audit avvengono in un unico posto che controlli.

Claude Code include un gateway auto-ospitato, [Claude apps gateway](/docs/it/claude-apps-gateway), nel binario `claude`, quindi non devi adottare un prodotto gateway separato per eseguirne uno. Se la tua organizzazione esegue già un [gateway LLM](/docs/it/llm-gateway), Claude Code funziona anche con quello.

Questa pagina copre:

* [Come un gateway si posiziona tra Claude Code e il tuo provider](#how-a-gateway-works)
* [Scelta tra Claude apps gateway e un gateway che già esegui](#choose-a-gateway)
* [Come i gateway interagiscono con gli abbonamenti claude.ai](#subscriptions-and-gateways)
* [Cosa viene configurato separatamente dal gateway](#configure-separately-from-the-gateway)

<h2 id="how-a-gateway-works">
  Come funziona un gateway
</h2>

Ogni Claude Code dello sviluppatore è indirizzato all'indirizzo del gateway e si autentica con una credenziale emessa dal gateway.

Il gateway autentica lo sviluppatore, applica qualsiasi regola di accesso e budget che configuri, e inoltra la richiesta al tuo provider con la credenziale dell'organizzazione. Il provider può essere l'API di Anthropic o un [provider cloud](/docs/it/third-party-integrations) come Amazon Bedrock, Agent Platform di Google Cloud o Microsoft Foundry; la configurazione del gateway decide. Con Claude apps gateway, o un altro gateway che espone un singolo endpoint in formato Anthropic, il cambio del provider non richiede di toccare le macchine degli sviluppatori.

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/llm-gateway-flow.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=1c1a8dcc0cfcc3a58652cc8e28cd3e20" alt="Diagramma che mostra Claude Code instradato attraverso un gateway. In una zona di macchine sviluppatore, la CLI di Claude Code e l'estensione VS Code inviano richieste all'indirizzo del gateway con una credenziale per sviluppatore. In una zona etichettata come tua infrastruttura, il gateway gestisce l'autenticazione, il tracciamento dell'utilizzo, i budget e l'instradamento, e inoltra le richieste con la credenziale della tua organizzazione. In una zona di provider di modelli, una freccia solida porta al provider che configuri, mostrato come l'API Anthropic, e frecce tratteggiate portano ad altre opzioni di provider, illustrate con Amazon Bedrock, Google Cloud e Microsoft Foundry come esempi." width="780" height="322" data-path="images/llm-gateway-flow.svg" />
</Frame>

Due tipi di credenziale sono coinvolti:

* **Credenziale dello sviluppatore**: ogni sviluppatore ne possiede una propria, emessa dal gateway. Li autentica al gateway e li identifica nel tracciamento dell'utilizzo
* **Credenziale del provider**: il gateway possiede una credenziale per il tuo account provider, condivisa da tutto il traffico inoltrato

<h2 id="choose-a-gateway">
  Scegli un gateway
</h2>

Claude Code funziona con il gateway di Anthropic o con un gateway che la tua organizzazione già esegue.

<h3 id="claude-apps-gateway">
  Claude apps gateway
</h3>

Claude apps gateway è il gateway auto-ospitato di Anthropic, incluso nel binario `claude`. Instrada ad Amazon Bedrock, Claude Platform su AWS, Google Cloud, Microsoft Foundry o l'API Anthropic come upstream. Gli sviluppatori accedono con il tuo provider di identità aziendale attraverso `/login`, il gateway applica l'accesso ai modelli e le [impostazioni gestite](/docs/it/permissions#managed-settings) per gruppo IdP, e emette metriche di utilizzo [OpenTelemetry Protocol (OTLP)](/docs/it/monitoring-usage) al tuo stack di osservabilità.

Poiché è costruito e testato insieme a ogni rilascio di Claude Code, inoltra le intestazioni e i campi di richiesta che Claude Code invia. Un gateway mantenuto separatamente ha bisogno che le sue [regole di inoltro vengano aggiornate](/docs/it/llm-gateway-protocol#forward-as-open-lists) mentre quelle intestazioni e campi cambiano con ogni rilascio; Claude apps gateway viene rilasciato con la CLI, quindi non c'è alcun elenco da mantenere aggiornato. Vedi [Disponibilità e limitazioni](/docs/it/claude-apps-gateway#availability-and-limitations) per il piccolo insieme di funzionalità che si comportano diversamente in una sessione gateway.

L'accesso al gateway è un passaggio SSO del browser, e non c'è alcun flusso di token di servizio, quindi una pipeline CI senza uno sviluppatore per approvare l'accesso non può autenticarsi attraverso di essa; configura quelli direttamente contro il tuo provider. Le sessioni Agent SDK e le esecuzioni `claude -p` su una macchina dove uno sviluppatore ha effettuato l'accesso utilizzano la sessione gateway di quella macchina e sono governate dalle sue politiche. Vedi [Pipeline CI e macchine remote](/docs/it/claude-apps-gateway#ci-pipelines-and-remote-machines).

Vedi [Claude apps gateway](/docs/it/claude-apps-gateway) per distribuirlo.

<h3 id="other-gateways">
  Altri gateway
</h3>

Se la tua organizzazione esegue già un gateway LLM o un gateway API, puoi usarlo invece. Anthropic non approva, mantiene o controlla altri prodotti gateway, e non supporta l'instradamento di Claude Code a modelli non-Claude attraverso alcun gateway. Vedi [Altri gateway LLM](/docs/it/llm-gateway) per la lista di controllo del rollout amministrativo, cosa deve implementare un gateway e come puntare Claude Code ad esso.

<h2 id="subscriptions-and-gateways">
  Abbonamenti e gateway
</h2>

Quando gli sviluppatori si connettono attraverso un gateway con una credenziale gateway, l'utilizzo viene fatturato all'account provider della tua organizzazione alle tariffe API, e i loro abbonamenti claude.ai non vengono utilizzati o addebitati. Impostare [`ANTHROPIC_AUTH_TOKEN`](/docs/it/env-vars) per un gateway che esegui, o accedere a un Claude apps gateway con `/login`, disattiva l'accesso all'abbonamento per quella sessione. Ogni richiesta inoltrata sotto quella credenziale viene addebitata all'account dietro la credenziale del provider del gateway.

L'eccezione è impostare solo `ANTHROPIC_BASE_URL`, senza credenziale gateway. Le richieste vengono comunque instradate attraverso il gateway, ma un accesso claude.ai salvato rimane la credenziale attiva, quindi i limiti di utilizzo e la fatturazione dell'abbonamento si applicano. [Altri gateway LLM](/docs/it/llm-gateway#subscriptions-and-gateways) copre quella configurazione e cosa il gateway deve inoltrare affinché funzioni.

<h2 id="configure-separately-from-the-gateway">
  Configura separatamente dal gateway
</h2>

Un gateway instrada le richieste API del modello. Poche cose che potresti aspettarti che gestisca sono configurate altrove:

* **Quale modello risponde**: scegli il modello con il comando `/model` o le [variabili di ambiente del modello](/docs/it/model-config#setting-your-model). Il gateway decide dove vanno le richieste, non quale modello seleziona lo sviluppatore. Claude apps gateway può limitare la scelta con una lista di autorizzazione `availableModels` per gruppo, ma lo sviluppatore sceglie comunque all'interno di essa.
* **Altro traffico di rete**: Claude Code stesso invia controlli di versione e download direttamente ad Anthropic, separato dal percorso del gateway. Se il flusso di telemetria del client opzionale è anche attivo dipende dal tuo provider; la [tabella dei valori predefiniti di telemetria](/docs/it/data-usage#telemetry-services) copre ogni caso. In una sessione Claude apps gateway con accesso effettuato, la credenziale gateway disabilita l'analittica legata ad Anthropic e, quando l'[inoltro di telemetria](/docs/it/claude-apps-gateway-config#telemetry) è configurato, fissa l'esportazione OTLP al gateway. La tua rete ha ancora bisogno di uscita verso i [domini richiesti](/docs/it/network-config), o imposta [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars) per disattivare i flussi opzionali.
* **Proxy HTTP aziendali**: un `HTTPS_PROXY` si posiziona tra Claude Code e ogni server con cui comunica, incluso il gateway. Se la tua rete ne richiede uno, [configura il proxy](/docs/it/network-config) in aggiunta al gateway. Per un Claude apps gateway che ospiti, [l'accesso verifica che l'host proxy sia anche su una rete privata](/docs/it/claude-apps-gateway#prerequisites); se non lo è, aggiungi l'host gateway a `NO_PROXY` in modo che la CLI si connetta ad esso direttamente.

<h2 id="next-steps">
  Passaggi successivi
</h2>

La pagina successiva dipende da chi esegue il gateway. Il gateway di Anthropic viene eseguito dal binario `claude` e ha la sua propria guida di configurazione; un gateway che la tua organizzazione già esegue ha un protocollo da implementare e una lista di controllo del rollout amministrativo.

* [Claude apps gateway](/docs/it/claude-apps-gateway) per distribuire il gateway auto-ospitato di Anthropic con accesso SSO e telemetria OTLP
* [Altri gateway LLM](/docs/it/llm-gateway) per cosa deve implementare un gateway che la tua organizzazione già esegue, e come puntare Claude Code ad esso
* [Configura Claude Code per la tua organizzazione](/docs/it/admin-setup) per le decisioni di rollout più ampie di cui un gateway è una parte
