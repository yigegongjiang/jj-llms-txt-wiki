> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guida di compatibilità del gateway Claude Code

> Mantieni un gateway LLM compatibile con Claude Code: gli endpoint che chiama, le intestazioni e i campi del corpo da inoltrare, e cosa si interrompe quando vengono rimossi.

Questa pagina documenta le richieste che Claude Code invia a un gateway, inclusi gli endpoint che chiama, le intestazioni e i campi del corpo che il gateway deve inoltrare, e quali funzionalità smettono di funzionare quando non lo fa. È scritta per gli operatori che configurano un prodotto gateway per funzionare con Claude Code.

Il [gateway delle app Claude](/docs/it/claude-apps-gateway), il gateway self-hosted di Anthropic, serve il proprio riferimento endpoint su `GET /protocol`, coprendo gli endpoint di accesso, inferenza, impostazioni gestite, scoperta dei modelli e telemetria di quel gateway. È un documento separato da questa guida.

<Note>
  * Per implementare un gateway esistente o di terze parti per la tua organizzazione, vedi [Implementare un gateway LLM](/docs/it/llm-gateway-rollout)
  * Se sei uno sviluppatore individuale che autentica Claude Code a un gateway con una credenziale che ti è stata fornita, vedi [Connettere Claude Code a un gateway LLM](/docs/it/llm-gateway-connect)
</Note>

Questa pagina copre:

* [Formati API](#api-formats) e gli endpoint da servire per ciascuno
* [Comportamento del client per metodo di connessione](#how-the-connection-method-changes-client-behavior): come gli ID modello, i valori `anthropic-beta`, i campi di richiesta e i valori predefiniti differiscono tra i formati e un accesso al gateway delle app Claude
* [Intestazioni di richiesta](#request-headers): quali devono raggiungere l'upstream e quali il tuo gateway può consumare
* [Intestazioni di risposta](#response-headers): cosa restituire affinché il rilevamento dei blocchi, i tentativi e la visualizzazione del limite di utilizzo funzionino
* Il [blocco di attribuzione del prompt di sistema](#system-prompt-attribution-block) e come interagisce con la memorizzazione nella cache dei prompt
* [Passaggio delle funzionalità](#feature-pass-through): cosa si interrompe quando le intestazioni o i campi del corpo vengono rimossi
* [Scoperta dei modelli](#model-discovery)

Questa pagina utilizza due termini per quello che il tuo gateway fa con ogni intestazione e campo del corpo:

* **Inoltra invariato**: passalo all'upstream byte per byte
* **Consuma**: il gateway può leggerlo per il routing, l'attribuzione o la traccia e non è necessario inoltrarlo

Qualsiasi cosa non contrassegnata come inoltrata invariata è tua da consumare o ignorare.

<h2 id="api-formats">
  Formati API
</h2>

Un gateway deve esporre almeno uno dei seguenti formati API ai client Claude Code. Un client sceglie un formato e punta Claude Code al vostro gateway con le variabili nella colonna Selezionato da della tabella sottostante.

Google Cloud's Agent Platform è l'endpoint Claude di Google Cloud, precedentemente Vertex AI; i nomi delle variabili mantengono l'ortografia `VERTEX`.

| Formato                                  | Selezionato da                                               | Endpoint                                                                                                         | Inoltra invariato                                                                                                    |
| :--------------------------------------- | :----------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                         | `/v1/messages`, `/v1/messages/count_tokens` (opzionale)                                                          | header di richiesta `anthropic-beta` e `anthropic-version`                                                           |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` con `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`, `/model/{model}/count-tokens` (opzionale) | campi del corpo della richiesta `anthropic_beta` e `anthropic_version`                                               |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` con `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (opzionale)                                        | header di richiesta `anthropic-beta` e `anthropic-version`, e il campo del corpo della richiesta `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry e Claude Platform on AWS
</h3>

Microsoft Foundry e la [Claude Platform on AWS](/docs/it/claude-platform-on-aws) implementano il formato Anthropic Messages. Claude Code instrada verso di loro attraverso le loro variabili, `ANTHROPIC_FOUNDRY_BASE_URL` e `ANTHROPIC_AWS_BASE_URL`, ma un gateway che le fronteggiano implementa la riga Anthropic Messages sopra. Un gateway che fronteggia Claude Platform on AWS deve anche inoltrare l'header `anthropic-workspace-id`, che [quella piattaforma richiede su ogni richiesta](/docs/it/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Endpoint opzionali e traffico di avvio
</h3>

Gli endpoint di conteggio dei token sono gli unici opzionali: quando sono assenti, Claude Code ricade su una stima basata su caratteri dell'utilizzo del contesto.

Abbinate in base al percorso, non all'URL completo:

* Le richieste di inferenza vengono inviate a `/v1/messages?beta=true`
* Il metodo Google Cloud's Agent Platform i suffissi si allegano al percorso del modello dell'editore, come in `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`

Un gateway vede anche traffico di avvio best-effort che può rifiutare senza rompere nulla. Un gateway in formato Anthropic Messages riceve una sonda di riscaldamento della connessione `HEAD /api/hello`, che Claude Code salta quando è configurato un proxy HTTP o un certificato client. Un gateway in formato Amazon Bedrock riceve una richiesta `GET /inference-profiles?type=SYSTEM_DEFINED` e, quando il modello configurato è un profilo di inferenza, ricerche `GET /inference-profiles/{profile}`.

Il controllo di disponibilità della [fast mode](/docs/it/fast-mode) non appare mai nei log del gateway: chiama `api.anthropic.com` direttamente piuttosto che seguire `ANTHROPIC_BASE_URL`, quindi su una rete che blocca l'uscita diretta verso `api.anthropic.com`, la fast mode può segnalare un errore di connettività mentre l'inferenza attraverso il gateway continua a funzionare. Il [controllo di sicurezza del dominio WebFetch](/docs/it/data-usage#webfetch-domain-safety-check) chiama anche `api.anthropic.com` direttamente. [Utilizzare fast mode dietro proxy e gateway LLM](/docs/it/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) copre le variabili che la ripristinano.

<h3 id="streaming">
  Streaming
</h3>

Trasmettete in streaming le risposte di inferenza. Claude Code legge il flusso mentre arriva, quindi se il vostro gateway memorizza nel buffer le risposte complete prima di inoltrarle, Claude Code si blocca.

Quando il client parla il formato Amazon Bedrock, inoltrate il corpo della risposta `InvokeModelWithResponseStream` e il suo header `Content-Type: application/vnd.amazon.eventstream` senza modifiche, e non convertite il flusso in server-sent events. Vedete [Errori di streaming dietro un gateway o proxy](/docs/it/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

Inoltrate anche i ping keep-alive. Sulle connessioni attraverso `ANTHROPIC_BASE_URL` o `ANTHROPIC_AWS_BASE_URL`, Claude Code conta ogni byte che il vostro gateway inoltra, inclusi gli eventi SSE `ping` e le righe di commento, e interrompe un flusso che rimane silenzioso per 300 secondi per impostazione predefinita. I ping dell'upstream sono l'unico traffico durante le pause di pensiero lunghe, quindi se il vostro gateway li elimina o li memorizza nel buffer, Claude Code interrompe il flusso durante quelle pause; [Tentativi automatici](/docs/it/errors#automatic-retries) copre ciò che un flusso interrotto segnala in base a quanto la risposta era progredita. Un upstream che non invia affatto ping, come l'event-stream binario di Amazon Bedrock, lascia quelle pause senza nulla da inoltrare. Quando si traduce da un tale upstream, emettete i vostri stessi eventi `ping` durante i gap silenziosi. I gateway raggiunti attraverso `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, o `ANTHROPIC_FOUNDRY_BASE_URL` non sono avvolti da questo watchdog a livello di byte, anche quando inoltrano il formato Anthropic Messages; lì, un [timeout di inattività di 5 minuti](/docs/it/env-vars) interrompe un flusso silenzioso invece, e sulle connessioni `ANTHROPIC_BEDROCK_BASE_URL` potete aggiungere il watchdog di byte con [`CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK`](/docs/it/env-vars).

<h3 id="format-mismatch-with-the-upstream">
  Mancata corrispondenza di formato con l'upstream
</h3>

Il formato che il client parla determina ciò che il vostro gateway riceve. La modalità di errore comune è una mancata corrispondenza tra il formato che il client invia al vostro gateway e il formato che il provider upstream dietro di esso accetta.

* Quando il client parla il formato Amazon Bedrock o Google Cloud's Agent Platform, Claude Code invia solo il sottoinsieme del suo set di capacità completo che quei provider accettano
* Quando il client parla il formato Anthropic Messages, Claude Code invia il set completo, anche se il vostro gateway inoltra a un upstream Amazon Bedrock o Google Cloud's Agent Platform

Colmare quella differenza è il compito del vostro gateway. [Passaggio delle funzionalità](#feature-pass-through) descrive cosa si rompe quando non lo fa.

Se il vostro upstream è Amazon Bedrock o Google Cloud's Agent Platform, potete evitare il bridging esponendo il formato di quel provider. [Instradare a un provider cloud attraverso un gateway](/docs/it/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) mostra la configurazione del client per quel formato.

<h2 id="how-the-connection-method-changes-client-behavior">
  Come il metodo di connessione cambia il comportamento del client
</h2>

Il modo in cui uno sviluppatore si connette al vostro gateway determina quali ID modello, valori `anthropic-beta` e campi di richiesta Claude Code invia, e quali impostazioni predefinite applica. Il vostro gateway vede uno di tre comportamenti del client:

* **Formato Amazon Bedrock o Agent Platform**: lo sviluppatore imposta `CLAUDE_CODE_USE_BEDROCK=1` con `ANTHROPIC_BEDROCK_BASE_URL`, oppure `CLAUDE_CODE_USE_VERTEX=1` con `ANTHROPIC_VERTEX_BASE_URL`, puntando al vostro gateway. Claude Code utilizza gli ID modello, i campi di richiesta e le impostazioni predefinite di quel provider.
* **Formato Anthropic Messages**: lo sviluppatore imposta `ANTHROPIC_BASE_URL` al vostro gateway. Claude Code tratta il gateway come l'API Claude e non può determinare quale upstream state inoltrando.
* **Accesso al gateway delle app Claude**: lo sviluppatore accede a un [gateway delle app Claude](/docs/it/claude-apps-gateway). Quel gateway parla il formato Anthropic Messages ma può instradare a qualsiasi upstream, quindi Claude Code invia solo i valori `anthropic-beta` e le assunzioni sulle capacità del modello che anche Amazon Bedrock e Agent Platform accettano.

<h3 id="requests-and-defaults-by-connection-method">
  Richieste e impostazioni predefinite per metodo di connessione
</h3>

La tabella seguente confronta i tre metodi di connessione, un comportamento per riga. Omette Microsoft Foundry e Claude Platform su AWS, che utilizzano anche il formato Anthropic Messages ma che Claude Code raggiunge attraverso le proprie variabili. Per quelli, consultate le pagine [Microsoft Foundry](/docs/it/microsoft-foundry) e [Claude Platform su AWS](/docs/it/claude-platform-on-aws).

| Comportamento                                                                                                                      | Formato Amazon Bedrock o Agent Platform                                                                                                                                                                                         | Formato Anthropic Messages                                                                                                                                                                              | Accesso al gateway delle app Claude                                                                                                    |
| :--------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| ID modello nelle richieste per impostazione predefinita                                                                            | La forma del provider, come `us.anthropic.claude-opus-4-8` su Amazon Bedrock                                                                                                                                                    | ID Anthropic, come `claude-opus-4-8`                                                                                                                                                                    | ID Anthropic                                                                                                                           |
| Valori `anthropic-beta` inviati                                                                                                    | Il sottoinsieme che Amazon Bedrock e Agent Platform accettano                                                                                                                                                                   | L'insieme completo descritto in [feature pass-through](#feature-pass-through), a meno che lo sviluppatore non imposti [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities)     | Il sottoinsieme che Amazon Bedrock e Agent Platform accettano                                                                          |
| Campi di richiesta per un ID modello che Claude Code non riconosce, come un alias del gateway                                      | Thinking con un budget fisso anziché ragionamento adattivo, e nessun campo di gestione dello sforzo o del contesto                                                                                                              | Tutto ciò che i modelli Claude attuali accettano sull'API Claude, incluso il ragionamento adattivo, lo sforzo e la gestione del contesto, che un upstream Amazon Bedrock o Agent Platform può rifiutare | Uguale al formato Amazon Bedrock o Agent Platform                                                                                      |
| [TTL della cache dei prompt](/docs/it/prompt-caching#choose-the-ttl-yourself) di un'ora quando uno sviluppatore acconsente              | Richiesto attraverso il campo `ttl` in `cache_control`, senza valore beta                                                                                                                                                       | Richiesto attraverso il campo `ttl` più un valore `extended-cache-ttl` in `anthropic-beta`, che dovete inoltrare                                                                                        | Consultate la tabella [disponibilità e limitazioni](/docs/it/claude-apps-gateway#availability-and-limitations) del gateway delle app Claude |
| Modello per [attività in background](/docs/it/costs#background-token-usage) a meno che `ANTHROPIC_DEFAULT_HAIKU_MODEL` non ne fissi uno | Il modello Sonnet predefinito, o il modello principale una volta selezionato, come descrivono le pagine [Amazon Bedrock](/docs/it/amazon-bedrock#4-pin-model-versions) e [Agent Platform](/docs/it/google-vertex-ai#5-pin-model-versions) | Il modello principale, o il modello Haiku predefinito quando `ANTHROPIC_API_KEY` o `apiKeyHelper` fornisce una chiave Anthropic Console e `ANTHROPIC_AUTH_TOKEN` non è impostato                        | Il modello principale                                                                                                                  |

Per le funzionalità che ogni connessione supporta e la telemetria che invia ad Anthropic per impostazione predefinita, consultate [Disponibilità delle funzionalità](/docs/it/feature-availability#availability-by-model-provider) e [Comportamenti predefiniti per provider API](/docs/it/data-usage#default-behaviors-by-api-provider).

<h3 id="settings-for-unrecognized-model-ids">
  Impostazioni per ID modello non riconosciuti
</h3>

Due impostazioni lato client cambiano ciò che Claude Code assume per un ID modello che non riconosce, indipendentemente dal metodo di connessione utilizzato dallo sviluppatore:

* **Finestra di contesto**: Claude Code assume 200K, o 1M quando l'ID contiene `[1m]`. Per dichiarare la finestra reale, consultate [Correggere la finestra per un gateway o un ID modello personalizzato](/docs/it/model-config#correct-the-window-for-a-gateway-or-custom-model-id)
* **Capacità**: per dare a un alias del gateway le capacità del modello dietro di esso, mappate l'ID Anthropic di quel modello al vostro alias con una voce [`modelOverrides`](/docs/it/errors#unrecognized-model-id-on-a-request) nelle impostazioni che distribuite. Per dove si applicano le variabili `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES`, consultate [feature pass-through](#feature-pass-through)

<h2 id="request-headers">
  Intestazioni delle richieste
</h2>

Claude Code include queste intestazioni sulle richieste API. I nomi delle intestazioni non fanno distinzione tra maiuscole e minuscole sul filo. Inoltrare `anthropic-version` e `anthropic-beta` invariate, più `anthropic-workspace-id` quando l'upstream è [Claude Platform on AWS](/docs/it/claude-platform-on-aws); il resto il gateway può consumare per il routing, l'attribuzione e il tracciamento, e non è necessario inoltrare.

| Intestazione                    | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Authorization`, `x-api-key`    | La credenziale del gateway dello sviluppatore, in una o entrambe le intestazioni a seconda di quale [variabile di credenziale](/docs/it/llm-gateway-connect#set-the-credential-variable) impostano                                                                                                                                                                                                                                                                                                                 |
| `anthropic-version`             | Versione API, attualmente `2023-06-01`. Le richieste in formato Amazon Bedrock e Agent Platform di Google Cloud portano anche il campo del corpo `anthropic_version`, il cui valore è la stringa del dialetto del provider, non il valore di questa intestazione                                                                                                                                                                                                                                              |
| `anthropic-beta`                | Valori di capacità separati da virgole per la richiesta. Inoltrare l'intestazione verbatim; non creare un elenco di consentiti dei singoli valori, perché l'insieme cambia con i rilasci di Claude Code. Quando lo sviluppatore si autentica con un accesso claude.ai, che è possibile quando `ANTHROPIC_BASE_URL` è impostato senza una variabile di credenziale del gateway, questa intestazione porta anche una capacità OAuth che l'upstream richiede, e rimuoverla fa fallire quelle richieste con `401` |
| `x-claude-code-session-id`      | Un identificatore univoco per la sessione Claude Code corrente. Usarlo per aggregare tutte le richieste da una sessione senza analizzare i corpi delle richieste                                                                                                                                                                                                                                                                                                                                              |
| `x-claude-code-agent-id`        | Identificatore del [subagent](/docs/it/sub-agents) che ha emesso la richiesta, presente solo sulle richieste da un agente che Claude Code ha generato all'interno della sessione. Usarlo con l'ID della sessione per attribuire il costo agli agenti paralleli                                                                                                                                                                                                                                                     |
| `x-claude-code-parent-agent-id` | Identificatore dell'agente che ha generato l'agente richiedente, presente solo per gli agenti annidati                                                                                                                                                                                                                                                                                                                                                                                                        |

Gli ID dei subagent vengono generati freschi ogni volta che Claude Code genera un subagent. Gli agenti compagni, i membri denominati di un [team di agenti](/docs/it/agent-teams), riutilizzano un ID stabile basato sul nome tra le riconnessioni. In entrambi i casi l'ID identifica un agente, non una persona o un dispositivo, quindi non trattate l'intestazione dell'ID dell'agente come un identificatore utente.

Se i vostri sviluppatori impostano `ANTHROPIC_CUSTOM_HEADERS`, quelle intestazioni appaiono anche sulle richieste.

<h3 id="gateway-hint-headers">
  Intestazioni di suggerimento del gateway
</h3>

Claude Code può anche inviare suggerimenti di routing: fatti per richiesta che un gateway o router può utilizzare per pianificare, memorizzare nella cache o attribuire una richiesta. Richiede Claude Code v2.1.273 o successivo.

Se una richiesta li porta dipende da dove Claude Code li invia:

* Connessione diretta all'API Anthropic: inviati per impostazione predefinita
* URL di base personalizzato: disattivato per impostazione predefinita, perché un proxy che rifiuta intestazioni sconosciute farebbe fallire la richiesta. Per riceverli, impostare [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](/docs/it/env-vars) per i vostri sviluppatori, ad esempio nel blocco `env` delle [impostazioni gestite](/docs/it/managed-settings)
* Qualsiasi altro backend, inclusi Amazon Bedrock, Agent Platform di Google Cloud, Microsoft Foundry e Claude Platform on AWS: inviati solo quando `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1` è impostato

Impostare `CLAUDE_CODE_GATEWAY_HINT_HEADERS` su `0` interrompe le intestazioni su ogni connessione.

Le intestazioni portano solo ciò che le righe sottostanti elencano: vocabolari fissi, nomi di strumenti e durate, mai testo del prompt o contenuti di file. Ogni valore è ASCII stampabile.

| Intestazione                        | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `x-claude-code-request-class`       | Che tipo di richiesta è questa: `main` per un turno della conversazione principale, `subagent` per un turno di un [subagent](/docs/it/sub-agents), `workflow` per un agente in esecuzione all'interno di un flusso di lavoro, `compaction` per la richiesta di riepilogo che compatta una conversazione, o `auxiliary` per richieste laterali come titoli di sessione, classificatori e riepiloghi. Inviato su ogni richiesta                                                                                                                                                            |
| `x-claude-code-agent-type`          | Il tipo di subagent che ha emesso la richiesta: un nome di tipo di agente integrato come `Explore`, `Plan` o `general-purpose`, o `custom` per un agente definito dall'utente, `teammate` per un membro del [team di agenti](/docs/it/agent-teams) in esecuzione nel processo del leader, o `fork` per un [fork](/docs/it/sub-agents#fork-the-current-conversation). Presente solo sui turni propri di un subagent; la compattazione o le richieste laterali di un subagent mantengono l'ID dell'agente ma non portano alcun tipo. Un nome di agente scelto dall'utente non viene mai inviato |
| `x-claude-code-compaction`          | Presente sulla richiesta che riepiloga la conversazione durante una [compattazione](/docs/it/prompt-caching#compacting-the-conversation). Il valore dice cosa l'ha attivata: `auto` quando la finestra di contesto si avvicinava alla capacità, `manual` per `/compact`, o `reactive` quando l'API ha rifiutato una richiesta come troppo lunga. Assente su ogni altra richiesta                                                                                                                                                                                                         |
| `x-claude-code-context-compacted`   | Presente una volta, sulla prima richiesta di conversazione principale dopo una compattazione, con gli stessi valori di `x-claude-code-compaction`. Il prefisso della conversazione prima di questa richiesta non viene più utilizzato, quindi una cache basata su di esso può essere eliminata                                                                                                                                                                                                                                                                                      |
| `x-claude-code-prev-tool-durations` | Tempo di esecuzione misurato delle chiamate di strumento i cui risultati questa richiesta porta, come `<name>=<ms>;<name>=<ms>`, ad esempio `Bash=742;Read=9`. Inviato sulla richiesta successiva della stessa conversazione dopo un batch di chiamate di strumento, dalla sessione principale o da un subagent                                                                                                                                                                                                                                                                     |

Prima di analizzare `x-claude-code-prev-tool-durations`, controllare come Claude Code costruisce il valore e cosa lascia fuori:

* Voci: una per ogni chiamata di strumento che è stata eseguita, nell'ordine in cui il suo risultato è stato raccolto, in millisecondi interi
* Limite: Claude Code invia al massimo 32 voci e 4 KB, mantenendo le prime voci
* Codifica: i nomi degli strumenti sono codificati in percentuale, coprendo `%`, `;`, `=`, virgola, spazio e qualsiasi carattere al di fuori di ASCII stampabile
* Analisi: dividere su `;`, quindi su `=`, e decodificare ogni nome
* Assenza: le chiamate di compattazione, le richieste laterali e la prima richiesta di un nuovo prompt non la portano mai. Non leggere un'intestazione mancante come un turno che non ha eseguito alcuno strumento
* Tempi: ognuno esclude i prompt di autorizzazione e gli hook, e le chiamate di strumento parallele ciascuna segnalano il proprio tempo, quindi le voci non si sommano al divario tra le richieste

<h3 id="forward-as-open-lists">
  Inoltrare come elenchi aperti
</h3>

Trattate le intestazioni e i campi del corpo come elenchi aperti, non chiusi. Claude Code guadagna capacità nei rilasci, e arrivano come nuovi valori `anthropic-beta`, nuovi campi del corpo della richiesta, e occasionalmente nuove intestazioni `anthropic-*` o `x-claude-code-*`.

Quando inoltrate a un upstream in formato Anthropic, passate le intestazioni di richiesta `anthropic-*` e i campi del corpo della richiesta invariati piuttosto che creare un elenco di consentiti di quelli che vedete oggi. Un gateway bloccato a un elenco osservato rimuove l'intestazione o il campo della prossima capacità e lo interrompe nel rilascio che lo introduce.

L'eccezione è un upstream non-Anthropic come Amazon Bedrock o Agent Platform di Google Cloud, dove colmare la differenza dello schema è il compito del gateway; vedere [passaggio delle funzionalità](#feature-pass-through).

<h2 id="response-headers">
  Intestazioni di risposta
</h2>

Claude Code legge queste intestazioni di risposta per rilevare flussi bloccati, per decidere se e quando riprovare, e per mostrare i limiti di utilizzo. La tabella elenca cosa restituire per ciascuna. Inoltra anche i corpi delle risposte di errore senza modifiche, in modo che il [recupero dal rifiuto di capacità](#automatic-retry-and-error-forwarding) di Claude Code possa corrispondere alla formulazione dell'errore upstream.

| Intestazione                    | Cosa restituire e perché                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `content-type`                  | Restituisci `text/event-stream` sulle risposte in formato Anthropic Messages in streaming, e `application/vnd.amazon.eventstream`, senza modifiche, sulle risposte in formato Amazon Bedrock, dove [un tipo diverso non riesce nella richiesta](/docs/it/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy). [Streaming](#streaming) elenca quali connessioni eseguono il rilevamento di stallo su questi flussi |
| `retry-after`                   | Restituisci secondi interi anziché una data HTTP. Claude Code attende almeno quel tempo prima del prossimo [tentativo automatico](/docs/it/errors#automatic-retries), e al di fuori delle sessioni [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/it/env-vars) un valore superiore a 60 interrompe i tentativi e mostra l'errore immediatamente                                                                                         |
| `x-should-retry`                | Passa il valore upstream senza modifiche. Claude Code legge questa intestazione come uno degli input quando decide se riprovare una richiesta non riuscita: `true` contrassegna la risposta come ritentabile e `false` la contrassegna come non ritentabile. Per i conteggi dei tentativi, il backoff e quali errori Claude Code riprova, vedi [tentativi automatici](/docs/it/errors#automatic-retries)                 |
| `anthropic-ratelimit-unified-*` | Inoltra i valori upstream senza modifiche su ogni risposta. Claude Code li legge sulle risposte riuscite per mostrare l'utilizzo rispetto ai limiti del piano agli sviluppatori che hanno effettuato l'accesso con claude.ai, e su un `429` per distinguere un limite del piano o un limite di spesa da una limitazione temporanea; vedi [limiti di utilizzo](/docs/it/errors#usage-limits)                              |

<h2 id="system-prompt-attribution-block">
  Blocco di attribuzione del prompt di sistema
</h2>

Claude Code antepone un breve blocco di attribuzione al prompt di sistema contenente la versione del client e un'impronta digitale derivata dalla conversazione. L'endpoint `api.anthropic.com` rimuove il blocco prima dell'elaborazione quando arriva invariato come primo blocco di sistema, quindi non influisce sul prompt caching di prima parte. Qualsiasi altro upstream lo riceve come parte del prompt.

La rimozione è posizionale, quindi funziona solo quando il gateway inoltra l'array `system` invariato. Per mantenere il blocco fuori dal prompt senza perdere altri contenuti di sistema:

* Inoltrare l'array `system` esattamente come ricevuto, mantenendo il blocco per primo: anteporre un altro blocco di sistema, riordinare l'array o convertirlo in una singola stringa annulla la rimozione, e il blocco raggiunge quindi il modello e la chiave della cache del prompt.
* Mantenere il blocco nella sua voce di array separata: l'endpoint tratta un blocco unito che inizia con l'intestazione di attribuzione come attribuzione nella sua interezza e scarta tutto ciò che vi è stato unito, incluso il resto del prompt di sistema.
* Se il vostro gateway deve rimodellare il contenuto di sistema, impostare [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/it/env-vars) in modo che Claude Code ometta il blocco. Anthropic e gli endpoint Claude dei provider cloud leggono il blocco per l'attribuzione, quindi ometterlo nel client piuttosto che rimuoverlo o spostarlo nel gateway.

La variabile esiste per la compatibilità con gateway e caching di terze parti, non come controllo della privacy: su una connessione diretta la richiesta completa va comunque all'API Anthropic in entrambi i casi. Quando entrambe queste condizioni si verificano, Claude Code mantiene il blocco sulle richieste del classificatore in [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) anche quando impostate la variabile su `0`:

* Le richieste vanno a `api.anthropic.com`, con `ANTHROPIC_BASE_URL` non impostato o che nomina quell'host e nessun provider di terze parti selezionato.
* La credenziale attiva non è una [credenziale di profilo Anthropic o di federazione](/docs/it/authentication#anthropic-profiles-and-federation-credentials).

Le richieste del classificatore saltano il resto del prompt di sistema di Claude Code, quindi su quelle richieste il blocco è l'unico marcatore nel corpo della richiesta che le identifica come traffico Claude Code. Quando una delle condizioni fallisce, attraverso un gateway LLM, su un provider di terze parti, o con una credenziale di profilo o federazione attiva, impostare `0` rimuove il blocco anche dalle richieste del classificatore. Prima della v2.1.229, questa eccezione non esisteva: impostare `0` rimuoveva il blocco da quelle richieste del classificatore, e quando l'API rifiutava le richieste non identificate, la modalità auto falliva su ogni azione che inviava al classificatore.

Da Claude Code v2.1.181, il blocco è stabile per la durata di una conversazione quando le richieste instradano attraverso un URL di base personalizzato, quindi una cache del prompt lato gateway basata sul corpo della richiesta completa funziona senza disabilitarla, e qualsiasi provider a cui il vostro gateway inoltra riceve un prefisso di prompt stabile. Prima di v2.1.181 il blocco includeva un token per richiesta che cambiava l'inizio del prompt di sistema ad ogni richiesta. Su quelle versioni, impostare `CLAUDE_CODE_ATTRIBUTION_HEADER=0` quando il vostro gateway fa una di queste cose:

* Implementa una cache del prompt basata sul corpo della richiesta.
* Inoltra richieste a un provider di terze parti come Amazon Bedrock, Microsoft Foundry, o Google Cloud's Agent Platform, nel formato Anthropic Messages o nel formato proprio del provider, dove il prefisso mutevole riduce il riutilizzo della cache del prompt su quel provider.

<h2 id="feature-pass-through">
  Passaggio delle funzionalità
</h2>

Claude Code tratta un gateway `ANTHROPIC_BASE_URL` come un endpoint in formato Anthropic e gli invia le intestazioni beta e i campi del corpo della richiesta che invia a `api.anthropic.com`, tranne un piccolo insieme di diagnostica e impostazioni predefinite riservate alle connessioni dirette, come l'impostazione predefinita di streaming fine degli strumenti coperta di seguito. Questo insieme varia per rilascio, quindi non dipendete dal suo contenuto.

Le capacità che aggiungono campi del corpo li associano a un'intestazione beta, e la coppia viaggia insieme. Un gateway che rimuove l'intestazione mentre passa il corpo, o inoltra un corpo in formato Anthropic a un upstream con uno schema diverso, produce errori `400` difficili; solo quando entrambe le metà sono assenti insieme la funzionalità si disattiva silenziosamente. Un gateway che riscrive o redige i corpi delle richieste per l'ispezione del contenuto interrompe l'associazione allo stesso modo della rimozione, quindi ispezionare senza modificare. La tabella nota dove una funzionalità si discosta dall'associazione.

Lo streaming fine degli strumenti è una delle impostazioni predefinite della connessione diretta: è disattivato per impostazione predefinita ogni volta che le richieste instradano attraverso un URL di base personalizzato, e un gateway lo riceve quando gli sviluppatori impostano [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/it/env-vars).

| Funzionalità                                                                                                                                                                                                                                  | Intestazione e coppia del corpo                                                                                                                                                                                                  | Sintomo quando interrotto                                                                                                                                                               | Rimedio                                                                                                                                    |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| [Ragionamento adattivo](/docs/it/model-config#adjust-effort-level)                                                                                                                                                                                 | Nessuna intestazione beta. Claude Code invia `thinking: {"type": "adaptive"}` per Claude 4.6 e successivi, e tratta i nomi dei modelli che non riconosce, come gli alias del gateway, come modelli attuali che ricevono il campo | `400` che nomina il campo `thinking` o il tag `adaptive` quando la build del modello upstream non lo accetta                                                                            | Aggiornare l'upstream. Su Opus 4.6 e Sonnet 4.6, gli sviluppatori possono invece impostare `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1`       |
| [Gestione del contesto](https://platform.claude.com/docs/en/build-with-claude/context-editing)                                                                                                                                                | L'intestazione beta di gestione del contesto si associa al campo del corpo `context_management`                                                                                                                                  | `400` con `Extra inputs are not permitted`. Comune quando un gateway accetta richieste in formato Anthropic ma le inoltra a Amazon Bedrock                                              | Inoltrare entrambi, o [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/it/env-vars)                                                           |
| [Contesto esteso](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) e [pensiero interleaved](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | Solo intestazioni beta, nessun campo del corpo                                                                                                                                                                                   | Silenziosamente non disponibile quando l'intestazione viene rimossa; l'upstream non vede mai la richiesta di capacità                                                                   | Inoltrare `anthropic-beta` verbatim                                                                                                        |
| Beta [tool fields](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                                    | Le intestazioni beta relative agli strumenti si associano ai campi dello schema dello strumento come `strict` e `defer_loading`                                                                                                  | `400` che nomina il campo dello schema dello strumento non riconosciuto quando il corpo passa attraverso senza la sua intestazione                                                      | Inoltrare entrambi, o [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)                                      |
| [Sforzo](https://platform.claude.com/docs/en/build-with-claude/effort) e [output strutturati](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                       | Il campo del corpo `output_config` contiene impostazioni di sforzo, formato di output strutturato e budget dei compiti; ciascuno si associa con la sua intestazione beta                                                         | `400` che nomina `output_config`, spesso `Extra inputs are not permitted`, su upstream Amazon Bedrock e Google Cloud's Agent Platform                                                   | Inoltrare il campo e le sue intestazioni insieme                                                                                           |
| [Prompt caching](/docs/it/prompt-caching)                                                                                                                                                                                                          | Nessun accoppiamento beta. Claude Code allega marcatori `cache_control` ai blocchi `system` e alle voci `messages`, incluse le voci `role: "system"` aggiunte a metà conversazione                                               | Nessun errore: la conversazione viene fatturata come input non memorizzato in cache ad ogni turno, visibile come `input_tokens` elevati con poca o nessuna attività di cache in `usage` | Inoltrare `cache_control` invariato ovunque appaia, e non convertire il contenuto del blocco `system` o del messaggio in stringhe semplici |
| [Conteggio dei token](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                   | Nessun accoppiamento beta; utilizza l'endpoint `count_tokens`                                                                                                                                                                    | Nessun errore: Claude Code ricade a una stima basata su caratteri, quindi `/context` mostra conteggi approssimativi                                                                     | Esporre l'endpoint per conteggi di token esatti                                                                                            |

Le variabili `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [variables](/docs/it/model-config) dichiarano le capacità del modello solo nelle configurazioni del provider: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, e [`CLAUDE_CODE_USE_MANTLE`](/docs/it/amazon-bedrock#use-the-mantle-endpoint). Non hanno effetto dietro un gateway `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Ritentativo automatico e inoltro degli errori
</h3>

Ciò che Claude Code fa dopo un rifiuto upstream dipende da ciò che è stato rifiutato:

* Quando l'upstream rifiuta il campo `thinking`, un messaggio di sistema a metà conversazione, o il marcatore `cache_control` su tale messaggio, Claude Code ritenta la richiesta e disabilita la capacità rifiutata per il resto della conversazione
* Quando l'upstream rifiuta una [thinking signature](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), incluso con un `400` il cui messaggio dice che il blocco è `bound to a different conversation`, Claude Code rimuove i blocchi di pensiero precedenti dalla richiesta, ritenta, e li mantiene fuori da ogni richiesta successiva. Le nuove risposte includono ancora il pensiero
* Quando il gateway o il suo upstream rifiuta la voce dello [strumento advisor](/docs/it/advisor) in `tools` come tipo di strumento non riconosciuto, Claude Code ritenta la richiesta una volta senza quella voce e il suo valore `anthropic-beta`. Le richieste successive a quell'URL di base lasciano l'advisor fuori fino a quando Claude Code esce, e `/advisor` non è disponibile allo sviluppatore per quel tempo. Claude Code riconosce questo rifiuto da una risposta `400` o `422` il cui messaggio nomina il tipo di strumento dopo `Input tag`, come `Input tag 'advisor_20260301'`. Prima della v2.1.280, Claude Code non ritentava questo rifiuto
* Claude Code non ritenta i rifiuti di gestione del contesto o di campi dello schema dello strumento, quindi quegli errori `400` raggiungono lo sviluppatore

Il rifiuto `bound to a different conversation` proviene dal controllo [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking) dell'API, che fallisce quando il contenuto di `system`, `tools`, o precedenti `messages` differisce dalla richiesta che ha prodotto il pensiero. Un gateway che riscrive uno qualsiasi di quel contenuto può causare il rifiuto stesso; [Libraries, proxies, and gateways](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways) copre ciò che passare attraverso invariato.

La logica di ritentativo corrisponde alla formulazione dell'errore dell'upstream, quindi inoltrare i corpi della risposta di errore invariati. Un gateway che avvolge gli errori upstream nel suo involucro interrompe il percorso di recupero, anche quando preserva il codice di stato, a meno che il messaggio dell'involucro non contenga un token `capability_rejected:` stabile. [Il gateway delle app Claude sostituisce questi token per la formulazione degli errori dei provider cloud](/docs/it/claude-apps-gateway-config#upstream-error-messages), ad esempio `capability_rejected: prompt_too_long`.

<h3 id="disable-pre-release-capabilities">
  Disabilitare le capacità pre-release
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` impedisce a Claude Code di inviare capacità pre-release e i loro campi del corpo su ogni provider, inclusa la gestione del contesto e i campi dello strumento beta. La variabile non influisce sul ragionamento adattivo, che è selezionato dal modello piuttosto che da beta. Non sopprime mai la capacità OAuth che l'autenticazione della sottoscrizione richiede.

Su Claude Code v2.1.227 o successivo, la vostra organizzazione può mantenere [MCP tool search](/docs/it/mcp#scale-with-mcp-tool-search) attivo sotto questa variabile attraverso [managed settings](/docs/it/managed-settings). Ciò che Claude Code invia con questo override in atto dipende da come vi connettete:

* Su una connessione diretta, o attraverso un gateway impostato con `ANTHROPIC_BASE_URL`, Claude Code continua a inviare l'intestazione beta di ricerca degli strumenti, i campi dello strumento `defer_loading`, e i blocchi `tool_reference`, e rimuove il resto
* Su un provider cloud, o accedendo attraverso un [gateway delle app Claude](/docs/it/claude-apps-gateway), l'override non ha effetto

L'insieme delle capacità che Claude Code invia cresce nei rilasci. Per le stringhe di intestazione beta attuali, vedere il [riferimento delle intestazioni beta](https://platform.claude.com/docs/en/api/beta-headers); testare il vostro gateway contro i nuovi rilasci di Claude Code piuttosto che bloccare a un elenco osservato.

<h2 id="model-discovery">
  Scoperta dei modelli
</h2>

Quando `ANTHROPIC_BASE_URL` punta a un gateway che espone il formato Anthropic Messages, Claude Code può interrogare l'endpoint `/v1/models` del gateway all'avvio e aggiungere i modelli restituiti al selettore `/model`. Se voi o il vostro amministratore impostate `replaceBuiltInOptions` in una lineup [`modelPicker`](/docs/it/settings-reference#modelpicker), Claude Code nasconde i modelli scoperti dal selettore.

Gli sviluppatori lo abilitano impostando [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/it/env-vars), nel loro ambiente o attraverso le impostazioni gestite. La scoperta è disattivata per impostazione predefinita in modo che i gateway supportati da una chiave API condivisa non espongano ogni modello a cui la chiave può accedere a ogni utente.

<h3 id="when-discovery-runs">
  Quando viene eseguita la scoperta
</h3>

La scoperta si applica solo al formato Anthropic Messages. Non viene eseguita quando:

* Qualsiasi variabile del provider `CLAUDE_CODE_USE_*` è impostata, anche se `ANTHROPIC_BASE_URL` è anche impostato
* `ANTHROPIC_BASE_URL` non è impostato o punta a `api.anthropic.com`

La scoperta viene comunque eseguita quando [il traffico non essenziale è disattivato](/docs/it/llm-gateway-connect#turn-off-traffic-outside-the-gateway-path), perché la richiesta va solo al vostro gateway. Prima della v2.1.257, la scoperta non veniva eseguita mentre il traffico non essenziale era disattivato.

<h3 id="request-and-response">
  Richiesta e risposta
</h3>

La richiesta è `GET /v1/models?limit=1000` con un timeout di 3 secondi per impostazione predefinita, e qualsiasi reindirizzamento è trattato come fallimento in modo che la credenziale non possa trapelare a una destinazione di reindirizzamento. Un gateway che risponde più lentamente del timeout, o uno che reindirizza `/v1/models`, anche da `http` a `https`, fallisce la scoperta silenziosamente; servire l'endpoint direttamente all'URL di base configurato.

Per dare a un gateway lento più tempo, impostate [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](/docs/it/env-vars#variables). La variabile richiede Claude Code v2.1.269 o successivo.

Claude Code invia la richiesta di scoperta con entrambe le intestazioni di credenziale sottostanti e omette un'intestazione il cui valore non si risolve. L'invio di entrambe le intestazioni richiede Claude Code v2.1.248 o successivo. Le versioni precedenti inviano solo `Authorization` quando `ANTHROPIC_AUTH_TOKEN` è impostato e solo `x-api-key` altrimenti.

* `Authorization`: `ANTHROPIC_AUTH_TOKEN` come token bearer, altrimenti il valore [`apiKeyHelper`](/docs/it/llm-gateway-connect#rotate-credentials-with-apikeyhelper) come token bearer. In questo caso Claude Code attende che l'helper restituisca prima di inviare la richiesta.
* `x-api-key`: la chiave API che Claude Code ha risolto, come `ANTHROPIC_API_KEY`. Quando un valore helper è l'unica credenziale, questa intestazione lo trasporta anche, in modo che il valore arrivi in entrambe le intestazioni.

Claude Code invia anche qualsiasi intestazione da `ANTHROPIC_CUSTOM_HEADERS`. Quando un'intestazione personalizzata ha un valore non vuoto, Claude Code la invia al posto di un'intestazione integrata con lo stesso nome, abbinando i nomi senza distinzione tra maiuscole e minuscole.

Quando nessun valore dell'intestazione di credenziale si risolve, Claude Code salta la scoperta e scrive una riga `[gatewayDiscovery] skipped` nel registro di debug di una sessione `claude --debug`. Se fornite una credenziale solo attraverso `ANTHROPIC_CUSTOM_HEADERS`, Claude Code salta comunque la scoperta.

Claude Code legge `id`, il `display_name` opzionale e la `description` opzionale da ogni voce nell'array `data` della risposta:

```json theme={null}
{
  "data": [
    {
      "id": "claude-sonnet-4-6",
      "display_name": "Claude Sonnet 4.6",
      "description": "Default model for everyday coding tasks"
    },
    { "id": "claude-opus-4-8" }
  ]
}
```

Claude Code mantiene una voce quando il suo `id` contiene `claude` o `anthropic` in qualsiasi punto della stringa, abbinato senza distinzione tra maiuscole e minuscole, e ignora il resto. Gli ID con prefisso del provider come `vertex_ai/claude-sonnet-4-6` o `bedrock/anthropic.claude-sonnet-4-5` superano il filtro; un ID che non contiene nessuna delle due sottostringhe non lo fa. Prima della v2.1.223, Claude Code manteneva una voce solo quando il suo `id` iniziava con `claude` o `anthropic`, il che nascondeva gli ID con prefisso del provider.

<h3 id="picker-entries-and-caching">
  Voci del selettore e caching
</h3>

Il selettore è l'elenco interattivo dei modelli che si apre quando uno sviluppatore esegue `/model` in Claude Code. Ogni voce scoperta utilizza `display_name` come nome quando il gateway ne invia una che differisce dall'`id`. Altrimenti la voce mostra il nome del modello quando Claude Code [riconosce l'`id`](/docs/it/model-config#customize-pinned-model-display-and-capabilities), e l'`id` quando non lo fa. Ad esempio, una voce con l'`id` `my-gateway-claude-sonnet-4-6` e nessun `display_name` appare come `Sonnet 4.6`.

La scoperta aggiunge solo i modelli che l'[impostazione gestita `availableModels`](/docs/it/settings-reference#availablemodels) consente.

Ogni voce mostra anche la `description` del modello, compressa in una riga. Una voce senza una `description` legge "From gateway" invece. Prima della v2.1.257, ogni voce scoperta leggeva "From gateway".

Un ID scoperto non ottiene la sua propria riga quando corrisponde a una riga già nel selettore:

* Stesso ID: l'ID scoperto corrisponde esattamente all'ID di una riga esistente, oppure i due ID sono ortografie della stessa versione [Fable](/docs/it/model-config#work-with-fable).
* Stesso modello di un alias integrato: quando un ID esplicito scoperto nomina il modello a cui un alias integrato attualmente si risolve, il selettore mostra solo la riga dell'alias. Ad esempio, mentre `sonnet` si risolve in `claude-sonnet-5`, un `claude-sonnet-5` scoperto si comprime nella riga `sonnet`, e un `claude-sonnet-4-6` scoperto ottiene comunque la sua propria riga. Prima della v2.1.197, Claude Code non piegava questi ID nelle righe integrate, quindi `claude-sonnet-5` otteneva anche la sua propria riga "From gateway".

I risultati vengono memorizzati nella cache in `~/.claude/cache/gateway-models.json`, o `%USERPROFILE%\.claude\cache\gateway-models.json` su Windows, e aggiornati ad ogni avvio. Se impostate [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars), la cache si trova invece in quella directory. Se la richiesta fallisce o il gateway non implementa `/v1/models`, il selettore ricade all'elenco memorizzato nella cache dall'avvio precedente o all'elenco dei modelli integrati. Se il vostro gateway serve modelli Claude con alias che non corrispondono al filtro di scoperta, gli sviluppatori possono aggiungere quegli alias manualmente con le [variabili di configurazione del modello](/docs/it/model-config).

<h2 id="related-resources">
  Risorse correlate
</h2>

Per il resto della serie di documentazione del gateway e i riferimenti API sottostanti:

* [Panoramica dei gateway](/docs/it/gateways): cos'è un gateway e come scegliere tra l'app gateway Claude e un altro prodotto
* [Altri gateway LLM](/docs/it/llm-gateway): come implementare un gateway che la vostra organizzazione gestisce e come interagisce con le sottoscrizioni claude.ai
* [Distribuire un gateway LLM per la vostra organizzazione](/docs/it/llm-gateway-rollout): la checklist dell'amministratore che utilizza questa guida
* [Connettere Claude Code a un gateway LLM](/docs/it/llm-gateway-connect): configurazione per sviluppatore e la tabella di risoluzione dei problemi
* [Riferimento delle intestazioni beta](https://platform.claude.com/docs/en/api/beta-headers): l'insieme attuale dei valori `anthropic-beta`
* [API Messages](https://platform.claude.com/docs/en/api/messages): il formato API che un gateway in formato Anthropic implementa
