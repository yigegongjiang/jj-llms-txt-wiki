> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Monitoraggio

> Scopri come abilitare e configurare OpenTelemetry per Claude Code.

Traccia l'utilizzo di Claude Code, i costi e l'attività degli strumenti in tutta l'organizzazione esportando i dati di telemetria tramite OpenTelemetry (OTel). Claude Code esporta le metriche come dati di serie temporali tramite il protocollo di metriche standard, gli eventi tramite il protocollo di log/eventi, e facoltativamente le tracce distribuite tramite il [protocollo di tracce](#traces-beta). Configura i tuoi backend di metriche, log e tracce per corrispondere ai tuoi requisiti di monitoraggio.

<h2 id="quick-start">
  Avvio rapido
</h2>

Configura OpenTelemetry utilizzando variabili di ambiente:

```bash theme={null}
# 1. Abilita la telemetria
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. Scegli gli esportatori (entrambi sono facoltativi - configura solo ciò di cui hai bisogno)
export OTEL_METRICS_EXPORTER=otlp       # Opzioni: otlp, prometheus, console, none
export OTEL_LOGS_EXPORTER=otlp          # Opzioni: otlp, console, none

# 3. Configura l'endpoint OTLP (per l'esportatore OTLP)
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. Imposta l'autenticazione (se richiesta)
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. Per il debug: riduci gli intervalli di esportazione
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 secondi (predefinito: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 secondi (predefinito: 5000ms)

# 6. Esegui Claude Code
claude
```

<Note>
  Gli intervalli di esportazione predefiniti sono 60 secondi per le metriche e 5 secondi per i log. Durante la configurazione, potresti voler utilizzare intervalli più brevi per scopi di debug. Ricordati di ripristinare questi valori per l'uso in produzione.
</Note>

Per le opzioni di configurazione complete, consulta la [specifica OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Configurazione dell'amministratore
</h2>

Gli amministratori possono configurare le impostazioni di OpenTelemetry per tutti gli utenti tramite il [file di impostazioni gestite](/docs/it/settings#settings-files). Ciò consente il controllo centralizzato delle impostazioni di telemetria in tutta l'organizzazione. Consulta la [precedenza delle impostazioni](/docs/it/settings#settings-precedence) per ulteriori informazioni su come vengono applicate le impostazioni.

Esempio di configurazione delle impostazioni gestite:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer example-token"
  }
}
```

<Note>
  Le impostazioni gestite possono essere distribuite tramite MDM (Mobile Device Management) o altre soluzioni di gestione dei dispositivi. Le variabili di ambiente definite nel file di impostazioni gestite hanno una precedenza elevata e non possono essere sovrascritte dagli utenti.
</Note>

Claude Code non passa le variabili di ambiente `OTEL_*` ai sottoprocessi che genera, incluso lo strumento Bash, gli hooks, i server MCP e i language server. Un'applicazione strumentata con OpenTelemetry che esegui tramite lo strumento Bash non eredita l'endpoint dell'esportatore di Claude Code o le intestazioni, quindi imposta quelle variabili direttamente nel comando se quell'applicazione ha bisogno di esportare la propria telemetria.

<h2 id="configuration-details">
  Dettagli della configurazione
</h2>

<h3 id="common-configuration-variables">
  Variabili di configurazione comuni
</h3>

| Variabile di ambiente                               | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                   | Valori di esempio                                                                                                             |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Abilita la raccolta della telemetria (obbligatorio)                                                                                                                                                                                                                                                                                                                                                           | `1`                                                                                                                           |
| `OTEL_METRICS_EXPORTER`                             | Tipi di esportatore di metriche, separati da virgola. Usa `none` per disabilitare                                                                                                                                                                                                                                                                                                                             | `console`, `otlp`, `prometheus`, `none`                                                                                       |
| `OTEL_LOGS_EXPORTER`                                | Tipi di esportatore di log/eventi, separati da virgola. Usa `none` per disabilitare                                                                                                                                                                                                                                                                                                                           | `console`, `otlp`, `none`                                                                                                     |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protocollo per l'esportatore OTLP, si applica a tutti i segnali                                                                                                                                                                                                                                                                                                                                               | `grpc`, `http/json`, `http/protobuf`                                                                                          |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Endpoint del collettore OTLP per tutti i segnali                                                                                                                                                                                                                                                                                                                                                              | `http://localhost:4317`                                                                                                       |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protocollo per le metriche, sostituisce l'impostazione generale                                                                                                                                                                                                                                                                                                                                               | `grpc`, `http/json`, `http/protobuf`                                                                                          |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Endpoint OTLP per le metriche, sostituisce l'impostazione generale                                                                                                                                                                                                                                                                                                                                            | `http://localhost:4318/v1/metrics`                                                                                            |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protocollo per i log, sostituisce l'impostazione generale                                                                                                                                                                                                                                                                                                                                                     | `grpc`, `http/json`, `http/protobuf`                                                                                          |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Endpoint OTLP per i log, sostituisce l'impostazione generale                                                                                                                                                                                                                                                                                                                                                  | `http://localhost:4318/v1/logs`                                                                                               |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Intestazioni di autenticazione per OTLP                                                                                                                                                                                                                                                                                                                                                                       | `Authorization=Bearer token`                                                                                                  |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Intervallo di esportazione in millisecondi (predefinito: 60000)                                                                                                                                                                                                                                                                                                                                               | `5000`, `60000`                                                                                                               |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Intervallo di esportazione dei log in millisecondi (predefinito: 5000)                                                                                                                                                                                                                                                                                                                                        | `1000`, `10000`                                                                                                               |
| `OTEL_LOG_USER_PROMPTS`                             | Abilita la registrazione del contenuto del prompt dell'utente (predefinito: disabilitato)                                                                                                                                                                                                                                                                                                                     | `1` per abilitare                                                                                                             |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Abilita la registrazione del testo della risposta dell'assistente negli eventi `assistant_response` (predefinito: disabilitato). Se non impostato, ricade al valore di `OTEL_LOG_USER_PROMPTS`. Richiede Claude Code v2.1.193 o successivo                                                                                                                                                                    | `1` per abilitare, `0` per mantenere redatto                                                                                  |
| `OTEL_LOG_TOOL_DETAILS`                             | Abilita la registrazione dei parametri dello strumento e degli argomenti di input negli eventi dello strumento e negli attributi di span di traccia: comandi Bash, nomi dei server MCP e dello strumento, nomi delle skill e input dello strumento. Abilita anche i nomi dei comandi personalizzati, plugin e MCP negli eventi `user_prompt` (predefinito: disabilitato)                                      | `1` per abilitare                                                                                                             |
| `OTEL_LOG_TOOL_CONTENT`                             | Abilita la registrazione del contenuto di input e output dello strumento negli eventi di span (predefinito: disabilitato). Richiede [tracce](#traces-beta). Il contenuto viene troncato a 60 KB                                                                                                                                                                                                               | `1` per abilitare                                                                                                             |
| `OTEL_LOG_RAW_API_BODIES`                           | Emetti il corpo completo della richiesta e della risposta JSON dell'API Anthropic Messages come eventi di log `api_request_body` / `api_response_body` (predefinito: disabilitato). I corpi includono l'intera cronologia della conversazione. L'abilitazione di questa opzione implica il consenso a tutto ciò che `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS`, e `OTEL_LOG_TOOL_CONTENT` rivelerebbero | `1` per corpi inline troncati a 60 KB, o `file:<dir>` per corpi non troncati su disco con un puntatore `body_ref` nell'evento |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Preferenza di temporalità delle metriche (predefinito: `delta`). Imposta su `cumulative` se il tuo backend prevede temporalità cumulativa                                                                                                                                                                                                                                                                     | `delta`, `cumulative`                                                                                                         |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervallo per l'aggiornamento delle intestazioni dinamiche (predefinito: 1740000ms / 29 minuti)                                                                                                                                                                                                                                                                                                              | `900000`                                                                                                                      |

<h3 id="mtls-authentication">
  Autenticazione mTLS
</h3>

Il modo in cui configuri i certificati client per l'esportatore OTLP dipende dal protocollo OTLP in uso per quel segnale, impostato tramite `OTEL_EXPORTER_OTLP_PROTOCOL` o l'override per segnale. La stessa configurazione si applica a metriche, log e tracce.

| Protocollo                   | Variabili di certificato client                                                                                                                                                                | Fidati del CA del collettore con |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY`, e facoltativamente `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Vedi [Configurazione di rete](/docs/it/network-config#mtls-authentication)             | `NODE_EXTRA_CA_CERTS`            |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` e `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, o le varianti per segnale come `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` per usare un certificato diverso per segnale | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

Per `grpc`, l'SDK OpenTelemetry legge direttamente le variabili OTLP standard, quindi le configurazioni esistenti che impostano le variabili di metriche per segnale continuano a funzionare.

<h3 id="metrics-cardinality-control">
  Controllo della cardinalità delle metriche
</h3>

Le seguenti variabili di ambiente controllano quali attributi sono inclusi nelle metriche per gestire la cardinalità:

| Variabile di ambiente                      | Descrizione                                                                                  | Valore predefinito | Esempio per disabilitare |
| ------------------------------------------ | -------------------------------------------------------------------------------------------- | ------------------ | ------------------------ |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Includi l'attributo session.id nelle metriche                                                | `true`             | `false`                  |
| `OTEL_METRICS_INCLUDE_VERSION`             | Includi l'attributo app.version nelle metriche                                               | `false`            | `true`                   |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Includi gli attributi user.account\_uuid e user.account\_id nelle metriche                   | `true`             | `false`                  |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Includi l'attributo app.entrypoint nelle metriche                                            | `false`            | `true`                   |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Includi le chiavi da `OTEL_RESOURCE_ATTRIBUTES` come attributi sui punti dati delle metriche | `true`             | `false`                  |

Queste variabili aiutano a controllare la cardinalità delle metriche, che influisce sui requisiti di archiviazione e sulle prestazioni delle query nel backend delle metriche. Una cardinalità inferiore generalmente significa prestazioni migliori e costi di archiviazione inferiori, ma dati meno granulari per l'analisi.

<h3 id="traces-beta">
  Tracce (beta)
</h3>

Le tracce distribuite esportano span che collegano ogni prompt dell'utente alle richieste API e alle esecuzioni degli strumenti che attiva, in modo da poter visualizzare una richiesta completa come una singola traccia nel tuo backend di traccia.

Le tracce sono disabilitate per impostazione predefinita. Per abilitarle, imposta sia `CLAUDE_CODE_ENABLE_TELEMETRY=1` che `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, quindi imposta `OTEL_TRACES_EXPORTER` per scegliere dove vengono inviati gli span. Le tracce riutilizzano la [configurazione OTLP comune](#common-configuration-variables) per endpoint, protocollo, intestazioni e [mTLS](#mtls-authentication).

| Variabile di ambiente                 | Descrizione                                                                                      | Valori di esempio                    |
| ------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Abilita la traccia degli span (obbligatorio). `ENABLE_ENHANCED_TELEMETRY_BETA` è anche accettato | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Tipi di esportatore di tracce, separati da virgola. Usa `none` per disabilitare                  | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protocollo per le tracce, sostituisce `OTEL_EXPORTER_OTLP_PROTOCOL`                              | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Endpoint OTLP per le tracce, sostituisce `OTEL_EXPORTER_OTLP_ENDPOINT`                           | `http://localhost:4318/v1/traces`    |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Intervallo di esportazione batch degli span in millisecondi (predefinito: 5000)                  | `1000`, `10000`                      |

Gli span redattano il testo del prompt dell'utente, i dettagli di input dello strumento e il contenuto dello strumento per impostazione predefinita. Imposta `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1`, e `OTEL_LOG_TOOL_CONTENT=1` per includerli.

Quando la traccia è attiva, i sottoprocessi Bash e PowerShell ereditano automaticamente una variabile di ambiente `TRACEPARENT` contenente il contesto di traccia W3C dello span di esecuzione dello strumento attivo. Ciò consente a qualsiasi sottoprocesso che legge `TRACEPARENT` di far diventare i propri span figli della stessa traccia, abilitando la traccia distribuita end-to-end attraverso script e comandi che Claude esegue.

Quando la traccia è attiva e Claude Code è connesso direttamente all'API Anthropic, ogni richiesta del modello porta un'intestazione W3C `traceparent` impostata al contesto dello span `claude_code.llm_request`, e l'intestazione `traceresponse` dell'API viene registrata come collegamento di span. Insieme questi collegano gli span lato client di Claude Code alla traccia lato server attraverso qualsiasi intermediario conforme. Le richieste HTTP MCP in uscita portano `traceparent` allo stesso modo. L'intestazione non viene inviata ai provider di terze parti.

Per impostazione predefinita, l'intestazione `traceparent` sulle richieste di modello e HTTP MCP viene inviata solo quando `ANTHROPIC_BASE_URL` non è impostato o punta all'API Anthropic, poiché alcuni proxy rifiutano intestazioni non riconosciute. La variabile `TRACEPARENT` del sottoprocesso è controllata dallo stesso switch per coerenza. Se esegui Claude Code attraverso un proxy `ANTHROPIC_BASE_URL` personalizzato e desideri che il contesto di traccia sia propagato, imposta `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

In Agent SDK e sessioni non interattive avviate con `-p`, Claude Code legge anche `TRACEPARENT` e `TRACESTATE` dal proprio ambiente quando avvia ogni span di interazione. Ciò consente a un processo di incorporamento di passare il suo contesto di traccia W3C attivo nel sottoprocesso in modo che gli span di Claude Code appaiano come figli della traccia distribuita del chiamante. Le sessioni interattive ignorano `TRACEPARENT` in entrata per evitare di ereditare accidentalmente valori ambientali da ambienti CI o container.

<h4 id="span-hierarchy">
  Gerarchia degli span
</h4>

Ogni prompt dell'utente avvia uno span radice `claude_code.interaction`. Le chiamate API, le chiamate agli strumenti e le esecuzioni degli hook vengono registrate come suoi figli. Gli span degli strumenti hanno due span figli propri: uno per il tempo trascorso in attesa di una decisione di autorizzazione e uno per l'esecuzione stessa. Quando lo strumento Agent o lo strumento Task legacy genera un subagent, gli span API e degli strumenti del subagent si annidano sotto lo span `claude_code.tool` del genitore.

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (richiede traccia beta dettagliata)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (Strumento Agent) span claude_code.llm_request / claude_code.tool del subagent
```

In Agent SDK e sessioni `claude -p`, `claude_code.interaction` stesso diventa un figlio dello span del chiamante quando `TRACEPARENT` è impostato nell'ambiente.

<h4 id="span-attributes">
  Attributi degli span
</h4>

Ogni span porta gli [attributi standard](#standard-attributes) più un attributo `span.type` che corrisponde al suo nome. Le tabelle seguenti elencano gli attributi aggiuntivi impostati su ogni span. Gli span `llm_request`, `tool.execution`, e `hook` impostano lo stato OpenTelemetry `ERROR` quando registrano un errore; gli altri span terminano sempre con stato `UNSET`.

**`claude_code.interaction`**

| Attributo                 | Descrizione                                                                     | Controllato da          |
| ------------------------- | ------------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Testo del prompt. Il valore è `<REDACTED>` a meno che il gate non sia impostato | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Lunghezza del prompt in caratteri                                               |                         |
| `interaction.sequence`    | Contatore basato su 1 delle interazioni in questa sessione                      |                         |
| `interaction.duration_ms` | Durata wall-clock del turno                                                     |                         |

**`claude_code.llm_request`**

| Attributo                        | Descrizione                                                                                                                                                                 | Controllato da          |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `model`                          | Identificatore del modello                                                                                                                                                  |                         |
| `gen_ai.system`                  | Sempre `anthropic`. Convenzione semantica OpenTelemetry GenAI                                                                                                               |                         |
| `gen_ai.request.model`           | Stesso valore di `model`. Convenzione semantica OpenTelemetry GenAI                                                                                                         |                         |
| `query_source`                   | Sottosistema che ha emesso la richiesta, come `repl_main_thread` o un nome di subagent                                                                                      |                         |
| `agent_id`                       | Identificatore del subagent o del collega che ha emesso la richiesta. Assente nella sessione principale                                                                     |                         |
| `parent_agent_id`                | Identificatore dell'agente che ha generato questo. Assente per la sessione principale e per gli agenti generati direttamente da essa                                        |                         |
| `workflow.run_id`                | Identificatore di esecuzione della [Workflow](/docs/it/workflows) tool run che ha generato questo agente, con prefisso `wf_`. Assente per gli agenti non generati da un workflow |                         |
| `workflow.name`                  | Nome del workflow che ha generato questo agente. I nomi creati dall'utente vengono sostituiti con `custom` a meno che il gate non sia impostato                             | `OTEL_LOG_TOOL_DETAILS` |
| `speed`                          | `fast` o `normal`                                                                                                                                                           |                         |
| `llm_request.context`            | `interaction`, `tool`, o `standalone` a seconda dello span genitore                                                                                                         |                         |
| `duration_ms`                    | Durata wall-clock inclusi i tentativi                                                                                                                                       |                         |
| `ttft_ms`                        | Tempo al primo token in millisecondi                                                                                                                                        |                         |
| `input_tokens`                   | Conteggio dei token di input dal blocco di utilizzo dell'API                                                                                                                |                         |
| `output_tokens`                  | Conteggio dei token di output                                                                                                                                               |                         |
| `cache_read_tokens`              | Token letti dalla cache del prompt                                                                                                                                          |                         |
| `cache_creation_tokens`          | Token scritti nella cache del prompt                                                                                                                                        |                         |
| `request_id`                     | ID della richiesta API Anthropic dall'intestazione della risposta `request-id`                                                                                              |                         |
| `gen_ai.response.id`             | Stesso valore di `request_id`. Convenzione semantica OpenTelemetry GenAI                                                                                                    |                         |
| `client_request_id`              | `x-client-request-id` generato dal client del tentativo finale                                                                                                              |                         |
| `attempt`                        | Tentativi totali effettuati per questa richiesta                                                                                                                            |                         |
| `success`                        | `true` o `false`                                                                                                                                                            |                         |
| `status_code`                    | Codice di stato HTTP quando la richiesta non è riuscita                                                                                                                     |                         |
| `error`                          | Messaggio di errore quando la richiesta non è riuscita                                                                                                                      |                         |
| `response.has_tool_call`         | `true` quando la risposta conteneva blocchi di tool-use                                                                                                                     |                         |
| `stop_reason`                    | `stop_reason` della risposta API, come `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn`, o `refusal`                                                     |                         |
| `gen_ai.response.finish_reasons` | Stesso valore di `stop_reason`, racchiuso in un array di stringhe. Convenzione semantica OpenTelemetry GenAI                                                                |                         |

Ogni tentativo di ripetizione viene anche registrato come evento di span `gen_ai.request.attempt` con attributi `attempt` e `client_request_id`.

**`claude_code.tool`**

| Attributo             | Descrizione                                                                                                                                                                                                                                                | Controllato da          |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | Nome dello strumento                                                                                                                                                                                                                                       |                         |
| `duration_ms`         | Durata wall-clock inclusa l'attesa di autorizzazione e l'esecuzione                                                                                                                                                                                        |                         |
| `result_tokens`       | Dimensione approssimativa in token del risultato dello strumento                                                                                                                                                                                           |                         |
| `agent_id`            | Identificatore del subagent o del collega che ha eseguito lo strumento. Assente nella sessione principale                                                                                                                                                  |                         |
| `parent_agent_id`     | Identificatore dell'agente che ha generato questo. Assente per la sessione principale e per gli agenti generati direttamente da essa                                                                                                                       |                         |
| `workflow.run_id`     | Identificatore di esecuzione della Workflow tool run che ha generato questo agente, con prefisso `wf_`. Assente per gli agenti non generati da un workflow                                                                                                 |                         |
| `workflow.name`       | Nome del workflow che ha generato questo agente. I nomi creati dall'utente vengono sostituiti con `custom` a meno che il gate non sia impostato                                                                                                            | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | L'ID del blocco `tool_use` del modello per questa chiamata. Corrisponde al `tool_use_id` sugli eventi [tool\_result](#tool-result-event) e [tool\_decision](#tool-decision-event) e nei payload degli hook, in modo da poter unire lo span a questi record |                         |
| `gen_ai.tool.call.id` | Stesso valore di `tool_use_id`. Convenzione semantica OpenTelemetry GenAI                                                                                                                                                                                  |                         |
| `file_path`           | Percorso del file di destinazione per gli strumenti Read, Edit e Write                                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Stringa di comando per lo strumento Bash                                                                                                                                                                                                                   | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nome della skill per lo strumento Skill                                                                                                                                                                                                                    | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Tipo di subagent per lo strumento Agent o lo strumento Task legacy                                                                                                                                                                                         | `OTEL_LOG_TOOL_DETAILS` |

Quando `OTEL_LOG_TOOL_CONTENT=1`, questo span registra anche un evento di span `tool.output` i cui attributi contengono i corpi di input e output dello strumento, troncati a 60 KB per attributo.

**`claude_code.tool.blocked_on_user`**

| Attributo     | Descrizione                                                                                  | Controllato da |
| ------------- | -------------------------------------------------------------------------------------------- | -------------- |
| `duration_ms` | Tempo trascorso in attesa della decisione di autorizzazione                                  |                |
| `decision`    | `accept` o `reject`                                                                          |                |
| `source`      | Fonte della decisione, corrispondente all'evento [Tool decision event](#tool-decision-event) |                |

**`claude_code.tool.execution`**

| Attributo             | Descrizione                                                                                                                                                                | Controllato da          |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Tempo trascorso nell'esecuzione del corpo dello strumento                                                                                                                  |                         |
| `tool_use_id`         | Stesso valore dello span genitore `claude_code.tool`                                                                                                                       |                         |
| `gen_ai.tool.call.id` | Stesso valore di `tool_use_id`. Convenzione semantica OpenTelemetry GenAI                                                                                                  |                         |
| `success`             | `true` o `false`                                                                                                                                                           |                         |
| `error`               | Stringa di categoria di errore quando l'esecuzione non è riuscita, come `Error:ENOENT` o `ShellError`. Contiene il messaggio di errore completo quando il gate è impostato | `OTEL_LOG_TOOL_DETAILS` |

**`claude_code.hook`**

Questo span viene emesso solo quando la traccia beta dettagliata è attiva, il che richiede `ENABLE_BETA_TRACING_DETAILED=1` e `BETA_TRACING_ENDPOINT` oltre alla configurazione dell'esportatore di tracce sopra. Nelle sessioni CLI interattive, ciò richiede anche che la tua organizzazione sia nella lista di autorizzazione per la funzione. Le sessioni Agent SDK e non interattive `-p` non sono controllate. Non viene emesso quando è impostato solo `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`.

| Attributo                | Descrizione                                                            | Controllato da          |
| ------------------------ | ---------------------------------------------------------------------- | ----------------------- |
| `hook_event`             | Tipo di evento hook, come `PreToolUse`                                 |                         |
| `hook_name`              | Nome completo dell'hook, come `PreToolUse:Write`                       |                         |
| `num_hooks`              | Numero di comandi hook corrispondenti eseguiti                         |                         |
| `hook_definitions`       | Configurazione dell'hook serializzata in JSON                          | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms`            | Durata wall-clock di tutti gli hook corrispondenti                     |                         |
| `num_success`            | Conteggio degli hook completati con successo                           |                         |
| `num_blocking`           | Conteggio degli hook che hanno restituito una decisione di blocco      |                         |
| `num_non_blocking_error` | Conteggio degli hook che non hanno avuto esito positivo senza bloccare |                         |
| `num_cancelled`          | Conteggio degli hook annullati prima del completamento                 |                         |

<Note>
  Attributi aggiuntivi che portano contenuto come `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input`, e `response.model_output` vengono emessi solo quando la traccia beta dettagliata è attiva. Non fanno parte dello schema di span stabile. `user_system_prompt` richiede inoltre `OTEL_LOG_USER_PROMPTS=1`. Contiene solo il testo del prompt di sistema che fornisci tramite l'opzione SDK `systemPrompt` o i flag `--system-prompt` e `--append-system-prompt`, troncato a 60 KB, ed è emesso una volta per sessione piuttosto che per richiesta.
</Note>

<h3 id="dynamic-headers">
  Intestazioni dinamiche
</h3>

Per gli ambienti aziendali che richiedono autenticazione dinamica, puoi configurare uno script per generare intestazioni dinamicamente. Le intestazioni dinamiche si applicano solo ai protocolli `http/protobuf` e `http/json`. L'esportatore `grpc` utilizza solo il valore statico `OTEL_EXPORTER_OTLP_HEADERS`.

<h4 id="settings-configuration">
  Configurazione delle impostazioni
</h4>

Aggiungi al tuo `.claude/settings.json`:

```json theme={null}
{
  "otelHeadersHelper": "/bin/generate_opentelemetry_headers.sh"
}
```

Il valore può essere il percorso di un file eseguibile, incluso un percorso che contiene spazi, o una riga di comando della shell con argomenti. Su Windows, il valore viene sempre eseguito attraverso la shell, quindi racchiudi tra virgolette un percorso che contiene spazi all'interno del valore JSON.

<h4 id="script-requirements">
  Requisiti dello script
</h4>

Lo script deve generare JSON valido con coppie chiave-valore di stringhe che rappresentano intestazioni HTTP:

```bash theme={null}
#!/bin/bash
# Esempio: Intestazioni multiple
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

Se l'helper non riesce o stampa output che non soddisfa questi requisiti, Claude Code segnala l'errore in:

* Output di `/status`
* Il log di debug, quando si esegue con [`--debug`](/docs/it/cli-reference#cli-flags) o dopo aver eseguito `/debug` nella sessione
* stderr, in sessioni non interattive avviate con `-p`

<h4 id="refresh-behavior">
  Comportamento di aggiornamento
</h4>

Lo script dell'helper di intestazioni viene eseguito all'avvio e periodicamente in seguito per supportare l'aggiornamento dei token. Per impostazione predefinita, lo script viene eseguito ogni 29 minuti. Personalizza l'intervallo con la variabile di ambiente `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`.

<h3 id="multi-team-organization-support">
  Supporto per organizzazioni multi-team
</h3>

Le organizzazioni con più team o dipartimenti possono aggiungere attributi personalizzati per distinguere tra diversi gruppi utilizzando la variabile di ambiente `OTEL_RESOURCE_ATTRIBUTES`:

```bash theme={null}
# Aggiungi attributi personalizzati per l'identificazione del team
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

Questi attributi personalizzati verranno inclusi in tutte le metriche e gli eventi, permettendoti di:

* Filtrare le metriche per team o dipartimento
* Tracciare i costi per centro di costo
* Creare dashboard specifici per team
* Configurare avvisi per team specifici

Claude Code allega questi valori come attributi su ogni punto dati di metrica e record di evento, oltre a inviarli nel blocco di risorse OTLP. Poiché la maggior parte dei backend di metriche espone gli attributi dei punti dati come etichette interrogabili, puoi raggruppare e filtrare le metriche direttamente per le tue chiavi personalizzate. Le chiavi personalizzate non sostituiscono mai gli [attributi standard](#standard-attributes) come `user.id` o `session.id`: quando una chiave collide, Claude Code mantiene il valore integrato.

Ogni chiave personalizzata diventa un'etichetta su ogni serie di metriche, quindi i valori ad alta cardinalità aumentano il costo di archiviazione nel tuo backend di metriche. Per inviare attributi personalizzati solo nel blocco di risorse e ometterli dalle etichette dei punti dati, imposta `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false`. Vedi [Controllo della cardinalità delle metriche](#metrics-cardinality-control).

<Warning>
  La variabile di ambiente `OTEL_RESOURCE_ATTRIBUTES` utilizza coppie chiave=valore separate da virgola con requisiti di formattazione rigorosi:

  * **Nessuno spazio consentito**: i valori non possono contenere spazi. Ad esempio, `user.organizationName=My Company` non è valido
  * **Formato**: deve essere coppie chiave=valore separate da virgola: `key1=value1,key2=value2`
  * **Caratteri consentiti**: solo caratteri US-ASCII escludendo caratteri di controllo, spazi bianchi, virgolette doppie, virgole, punti e virgola e barre rovesciate
  * **Caratteri speciali**: i caratteri al di fuori dell'intervallo consentito devono essere codificati in percentuale

  Per un valore che avrebbe bisogno di uno spazio, usa sottolineature o camelCase invece. I seguenti esempi impostano `org.name` con ogni forma:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
  export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
  ```

  Puoi codificare in percentuale qualsiasi carattere, non solo quelli esclusi. Questo esempio codifica sia lo spazio che l'apostrofo:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
  ```

  Racchiudere i valori tra virgolette non sfugge agli spazi. Ad esempio, `org.name="My Company"` risulta nel valore letterale `"My Company"` con le virgolette incluse, non `My Company`.
</Warning>

<h3 id="example-configurations">
  Configurazioni di esempio
</h3>

Imposta queste variabili di ambiente prima di eseguire `claude`. Ogni blocco mostra una configurazione completa per un diverso esportatore o scenario di distribuzione:

```bash theme={null}
# Debug della console (intervalli di 1 secondo)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000

# OTLP/gRPC
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Prometheus
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus

# Esportatori multipli
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json

# Endpoint/backend diversi per metriche e log
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317

# Solo metriche (nessun evento/log)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Solo eventi/log (nessuna metrica)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  Metriche e eventi disponibili
</h2>

<h3 id="standard-attributes">
  Attributi standard
</h3>

Tutte le metriche e gli eventi condividono questi attributi standard:

| Attributo                            | Descrizione                                                                                                                                                                                                                                             | Controllato da                                                 |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| `session.id`                         | Identificatore di sessione univoco                                                                                                                                                                                                                      | `OTEL_METRICS_INCLUDE_SESSION_ID` (predefinito: true)          |
| `app.version`                        | Versione corrente di Claude Code                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_VERSION` (predefinito: false)            |
| `app.entrypoint`                     | Come la sessione è stata avviata, come `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, o `claude-vscode`                                                                                                                                                          | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (predefinito: false)         |
| `organization.id`                    | UUID dell'organizzazione (quando autenticato)                                                                                                                                                                                                           | Sempre incluso quando disponibile                              |
| `user.account_uuid`                  | UUID dell'account (quando autenticato)                                                                                                                                                                                                                  | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (predefinito: true)        |
| `user.account_id`                    | ID dell'account in formato taggato corrispondente alle API di amministrazione Anthropic (quando autenticato), come `user_01BWBeN28...`                                                                                                                  | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (predefinito: true)        |
| `user.id`                            | Identificatore anonimo casuale generato al primo avvio e persistente in `~/.claude.json`. Non contiene informazioni personali e non è derivato dal tuo account Claude. L'eliminazione del file produce un nuovo valore non correlato al prossimo avvio. | Sempre incluso                                                 |
| `user.email`                         | Indirizzo email dell'utente (quando autenticato tramite OAuth)                                                                                                                                                                                          | Sempre incluso quando disponibile                              |
| `terminal.type`                      | Tipo di terminale, come `iTerm.app`, `vscode`, `cursor`, o `tmux`                                                                                                                                                                                       | Sempre incluso quando rilevato                                 |
| Chiavi da `OTEL_RESOURCE_ATTRIBUTES` | Attributi personalizzati che imposti, come `department` o `team.id`. Vedi [Supporto per organizzazioni multi-team](#multi-team-organization-support)                                                                                                    | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (predefinito: true) |

Quando Claude Code è connesso a un [gateway di app Claude](/docs/it/claude-apps-gateway), la CLI contrassegna le esportazioni con l'identità autenticata dalla sessione del gateway: `user.id` è il soggetto IdP piuttosto che un identificatore di installazione anonimo, `user.email` è l'email connessa, e `user.groups` contiene l'appartenenza al gruppo IdP come stringa separata da virgole. Ogni esportazione contiene anche `identity.source: gateway-oidc`. L'identità del gateway viene applicata per ultima, quindi le chiavi `user.*` e `identity.*` impostate tramite `OTEL_RESOURCE_ATTRIBUTES` vengono ignorate nelle sessioni del gateway.

Gli eventi includono inoltre i seguenti attributi. Questi non vengono mai allegati alle metriche perché causerebbero cardinalità illimitata:

* `prompt.id`: UUID che correla un prompt dell'utente con tutti gli eventi successivi fino al prompt successivo. Vedi [Attributi di correlazione degli eventi](#event-correlation-attributes).
* `workspace.host_paths`: directory dell'area di lavoro host selezionate nell'app desktop, come array di stringhe
* `workflow.run_id`: identificatore di esecuzione, con prefisso `wf_`, sugli eventi API e degli strumenti emessi da agenti che appartengono a un'esecuzione dello strumento [Workflow](/docs/it/workflows). Filtrando gli eventi per un `workflow.run_id` si ricostruisce l'esecuzione delle richieste API e dei risultati degli strumenti. L'identificatore copre gli agenti che lo script del workflow genera e tutti gli agenti che questi generano a loro volta, come le invocazioni di skill. Corrisponde all'identificatore di esecuzione segnalato nel risultato dello strumento Workflow. Assente su tutti gli altri eventi. Richiede Claude Code v2.1.202 o successivo
* `workflow.name`: nome del workflow, il `meta.name` del suo script, emesso insieme a `workflow.run_id`. I nomi dei workflow incorporati vengono visualizzati così come sono quando l'esecuzione esegue lo script incorporato non modificato. I nomi creati dall'utente, incluse le copie modificate degli script incorporati, vengono sostituiti con `custom` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato. Richiede Claude Code v2.1.202 o successivo

<h3 id="metrics">
  Metriche
</h3>

Claude Code esporta le seguenti metriche:

| Nome della metrica                    | Descrizione                                                                        | Unità  |
| ------------------------------------- | ---------------------------------------------------------------------------------- | ------ |
| `claude_code.session.count`           | Conteggio delle sessioni CLI avviate                                               | count  |
| `claude_code.lines_of_code.count`     | Conteggio delle righe di codice modificate                                         | count  |
| `claude_code.pull_request.count`      | Numero di pull request create                                                      | count  |
| `claude_code.commit.count`            | Numero di commit git creati                                                        | count  |
| `claude_code.cost.usage`              | Costo della sessione Claude Code                                                   | USD    |
| `claude_code.token.usage`             | Numero di token utilizzati                                                         | tokens |
| `claude_code.code_edit_tool.decision` | Conteggio delle decisioni di autorizzazione dello strumento di modifica del codice | count  |
| `claude_code.active_time.total`       | Tempo attivo totale in secondi                                                     | s      |

<h3 id="metric-details">
  Dettagli delle metriche
</h3>

Ogni metrica include gli attributi standard elencati sopra. Le metriche con attributi aggiuntivi specifici del contesto sono indicate di seguito.

<h4 id="session-counter">
  Contatore di sessione
</h4>

Incrementato all'inizio di ogni sessione.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `start_type`: Come la sessione è stata avviata. Uno di `"fresh"`, `"resume"`, `"continue"`, o `"agents_view"`. Il valore `"agents_view"` identifica il processo della dashboard `claude agents`, un'interfaccia utente locale avviata dall'utente piuttosto che una sessione conversazionale. Filtra su questo valore per separare i lanci del processo UI dalle sessioni conversazionali nei tuoi dashboard.

<h4 id="lines-of-code-counter">
  Contatore di righe di codice
</h4>

Incrementato quando il codice viene aggiunto o rimosso.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: Identificatore del modello per il modello che ha effettuato la modifica (ad esempio, "claude-sonnet-5")

<h4 id="pull-request-counter">
  Contatore di pull request
</h4>

Incrementato quando Claude Code crea una pull request o merge request tramite un comando shell o uno strumento MCP.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)

<h4 id="commit-counter">
  Contatore di commit
</h4>

Incrementato quando si creano commit git tramite Claude Code.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)

<h4 id="cost-counter">
  Contatore di costo
</h4>

Incrementato dopo ogni richiesta API.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `model`: Identificatore del modello (ad esempio, "claude-sonnet-5")
* `query_source`: Categoria del sottosistema che ha emesso la richiesta. Uno di `"main"`, `"subagent"`, o `"auxiliary"`
* `speed`: `"fast"` quando la richiesta ha utilizzato la modalità veloce. Assente altrimenti
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta: `"low"`, `"medium"`, `"high"`, `"xhigh"`, o `"max"`. Assente quando il modello non supporta lo sforzo.
* `agent.name`: Tipo di subagent che ha emesso la richiesta. I nomi degli agenti incorporati e i nomi degli agenti dai plugin del marketplace ufficiale vengono visualizzati così come sono. Gli altri nomi degli agenti definiti dall'utente vengono sostituiti con `"custom"`. Assente quando la richiesta non è stata emessa da un tipo di subagent denominato.
* `skill.name`: Skill attiva per la richiesta, impostata dallo strumento Skill, da un comando `/`, o ereditata da un subagent generato. I nomi delle skill incorporate, in bundle, definite dall'utente e dei plugin del marketplace ufficiale vengono visualizzati così come sono. I nomi delle skill dei plugin di terze parti vengono sostituiti con `"third-party"`. Assente quando nessuna skill è attiva.
* `plugin.name`: Plugin proprietario quando la skill attiva o il subagent è fornito da un plugin. I nomi dei plugin del marketplace ufficiale vengono visualizzati così come sono. I nomi dei plugin di terze parti vengono sostituiti con `"third-party"`. Assente quando né la skill né il subagent hanno un plugin proprietario.
* `marketplace.name`: Marketplace da cui è stato installato il plugin proprietario. Emesso solo per i plugin del marketplace ufficiale. Assente altrimenti.
* `mcp_server.name`: Server MCP il cui strumento è stato eseguito nel turno che ha prodotto questa richiesta. I nomi dei server incorporati, proxy di claude.ai e del registro ufficiale vengono visualizzati così come sono. I nomi dei server configurati dall'utente vengono sostituiti con `"custom"`. Assente quando nessuno strumento MCP è stato eseguito.
* `mcp_tool.name`: Strumento MCP che è stato eseguito nel turno che ha prodotto questa richiesta, con la stessa redazione di `mcp_server.name`. Assente quando nessuno strumento MCP è stato eseguito.

<h4 id="token-counter">
  Contatore di token
</h4>

Incrementato dopo ogni richiesta API.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: Identificatore del modello (ad esempio, "claude-sonnet-5")
* `query_source`: Categoria del sottosistema che ha emesso la richiesta. Uno di `"main"`, `"subagent"`, o `"auxiliary"`
* `speed`: `"fast"` quando la richiesta ha utilizzato la modalità veloce. Assente altrimenti
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta. Vedi [Contatore di costo](#cost-counter) per i dettagli.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Attribuzione di skill, plugin, agente e MCP per la richiesta. Vedi [Contatore di costo](#cost-counter) per le definizioni e il comportamento di redazione.

<h4 id="code-edit-tool-decision-counter">
  Contatore di decisione dello strumento di modifica del codice
</h4>

Incrementato quando l'utente accetta o rifiuta l'utilizzo dello strumento Edit, Write o NotebookEdit.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `tool_name`: Nome dello strumento (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Decisione dell'utente (`"accept"`, `"reject"`)
* `source`: Fonte della decisione. Uno di `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, o `"user_reject"`. Vedi l'[Evento di decisione dello strumento](#tool-decision-event) per il significato di ogni valore.
* `language`: Linguaggio di programmazione del file modificato, come `"TypeScript"`, `"Python"`, `"JavaScript"`, o `"Markdown"`. Restituisce `"unknown"` per estensioni di file non riconosciute.

<h4 id="active-time-counter">
  Contatore di tempo attivo
</h4>

Traccia il tempo effettivo trascorso utilizzando attivamente Claude Code, escludendo il tempo di inattività. Questa metrica viene incrementata durante le interazioni dell'utente, come la digitazione e la lettura delle risposte, e durante l'elaborazione CLI, come l'esecuzione degli strumenti e la generazione della risposta AI.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `type`: `"user"` per le interazioni da tastiera, `"cli"` per l'esecuzione degli strumenti e le risposte AI

<h3 id="events">
  Eventi
</h3>

Claude Code esporta i seguenti eventi tramite log/eventi OpenTelemetry (quando `OTEL_LOGS_EXPORTER` è configurato):

<h4 id="event-correlation-attributes">
  Attributi di correlazione degli eventi
</h4>

Quando un utente invia un prompt, Claude Code potrebbe effettuare più chiamate API ed eseguire diversi strumenti. L'attributo `prompt.id` ti consente di collegare tutti questi eventi al singolo prompt che li ha attivati.

| Attributo   | Descrizione                                                                                                          |
| ----------- | -------------------------------------------------------------------------------------------------------------------- |
| `prompt.id` | Identificatore UUID v4 che collega tutti gli eventi prodotti durante l'elaborazione di un singolo prompt dell'utente |

Per tracciare tutta l'attività attivata da un singolo prompt, filtra i tuoi eventi per un valore specifico di `prompt.id`. Questo restituisce l'evento user\_prompt, eventuali eventi api\_request, e eventuali eventi tool\_result che si sono verificati durante l'elaborazione di quel prompt.

<Note>
  `prompt.id` è intenzionalmente escluso dalle metriche perché ogni prompt genera un ID univoco, il che creerebbe un numero sempre crescente di serie temporali. Usalo solo per l'analisi a livello di evento e i trail di audit.
</Note>

<h4 id="user-prompt-event">
  Evento di prompt dell'utente
</h4>

Registrato quando un utente invia un prompt.

**Nome evento**: `claude_code.user_prompt`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `prompt_length`: Lunghezza del prompt
* `prompt`: Contenuto del prompt. Redatto per impostazione predefinita. Imposta `OTEL_LOG_USER_PROMPTS=1` per includerlo
* `command_name`: Nome del comando quando il prompt ne invoca uno. I nomi dei comandi incorporati e in bundle come `compact` o `debug` vengono emessi così come sono; gli alias come `reset` vengono emessi come digitati piuttosto che il nome canonico. I nomi dei comandi personalizzati, plugin e MCP si riducono a `custom` o `mcp` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato
* `command_source`: Origine del comando quando presente: `builtin`, `custom`, o `mcp`. I comandi forniti dai plugin vengono segnalati come `custom`

<h4 id="assistant-response-event">
  Evento di risposta dell'assistente
</h4>

Registrato dopo ogni richiesta API che restituisce contenuto di testo dal modello. Solo i blocchi di testo della risposta sono inclusi; i blocchi di pensiero e i blocchi di utilizzo dello strumento sono esclusi. Richiede Claude Code v2.1.193 o successivo.

**Nome evento**: `claude_code.assistant_response`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `response_length`: Lunghezza del testo della risposta in caratteri
* `response`: Testo della risposta, troncato a 60 KB. Redatto a `<REDACTED>` per impostazione predefinita. Imposta `OTEL_LOG_ASSISTANT_RESPONSES=1` per includerlo. Quando `OTEL_LOG_ASSISTANT_RESPONSES` non è impostato, `OTEL_LOG_USER_PROMPTS` lo controlla invece, quindi imposta `OTEL_LOG_ASSISTANT_RESPONSES=0` per mantenere le risposte redatte mentre la registrazione dei prompt è attiva
* `model`: Identificatore del modello (ad esempio, "claude-sonnet-5")
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta. Presente solo quando l'API ne restituisce uno
* `query_source`: Sottosistema che ha emesso la richiesta, come `"repl_main_thread"`, `"compact"`, o un nome di subagent

<h4 id="tool-result-event">
  Evento di risultato dello strumento
</h4>

Registrato quando uno strumento completa l'esecuzione. Non emesso se la chiamata dello strumento è stata rifiutata; vedi l'[Evento di decisione dello strumento](#tool-decision-event) per i rifiuti.

**Nome evento**: `claude_code.tool_result`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `tool_name`: Nome dello strumento
* `tool_use_id`: Identificatore univoco per questa invocazione dello strumento. Corrisponde al `tool_use_id` passato agli hooks, consentendo la correlazione tra gli eventi OTel e i dati acquisiti dagli hooks.
* `success`: `"true"` o `"false"`
* `duration_ms`: Tempo di esecuzione in millisecondi
* `error_type`: Stringa di categoria di errore quando lo strumento non è riuscito, come `"Error:ENOENT"` o `"ShellError"`
* `error` (quando `OTEL_LOG_TOOL_DETAILS=1`): Messaggio di errore completo quando lo strumento non è riuscito
* `decision_type`: Sempre `"accept"`, poiché questo evento viene emesso solo dopo l'esecuzione dello strumento. Le chiamate rifiutate non producono un risultato dello strumento
* `decision_source`: Fonte della decisione di autorizzazione. Uno di `"config"`, `"hook"`, `"user_permanent"`, o `"user_temporary"`. Vedi l'[Evento di decisione dello strumento](#tool-decision-event) per il significato di ogni valore. Le fonti solo per il rifiuto `"user_abort"` e `"user_reject"` non compaiono mai su questo evento.
* `tool_input_size_bytes`: Dimensione dell'input dello strumento serializzato in JSON in byte
* `tool_result_size_bytes`: Dimensione del risultato dello strumento in byte
* `mcp_server_scope`: Identificatore dell'ambito del server MCP (per gli strumenti MCP)
* `tool_parameters` (quando `OTEL_LOG_TOOL_DETAILS=1`): Stringa JSON contenente parametri specifici dello strumento:
  * Per lo strumento Bash: include `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`, e `git_commit_id` (lo SHA del commit, quando un comando `git commit` ha successo)
  * Per lo strumento WorkspaceBash: include `bash_command`, `full_command`, `timeout`
  * Per gli strumenti MCP: include `mcp_server_name`, `mcp_tool_name`
  * Per lo strumento Skill: include `skill_name`
  * Per lo strumento Agent o lo strumento Task legacy: include `subagent_type`
* `tool_input` (quando `OTEL_LOG_TOOL_DETAILS=1`): Argomenti dello strumento serializzati in JSON. I singoli valori superiori a 512 caratteri vengono troncati, e il payload completo è limitato a circa 4 K caratteri. Si applica a tutti gli strumenti inclusi gli strumenti MCP.

<h4 id="api-request-event">
  Evento di richiesta API
</h4>

Registrato per ogni richiesta API a Claude.

**Nome evento**: `claude_code.api_request`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `model`: Modello utilizzato (ad esempio, "claude-sonnet-5")
* `cost_usd`: Costo stimato in USD
* `duration_ms`: Durata della richiesta in millisecondi
* `input_tokens`: Numero di token di input
* `output_tokens`: Numero di token di output
* `cache_read_tokens`: Numero di token letti dalla cache
* `cache_creation_tokens`: Numero di token utilizzati per la creazione della cache
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta, come `"req_011..."`. Presente solo quando l'API ne restituisce uno.
* `speed`: `"fast"` o `"normal"`, indicando se la modalità veloce era attiva
* `query_source`: Sottosistema che ha emesso la richiesta, come `"repl_main_thread"`, `"compact"`, o un nome di subagent
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta: `"low"`, `"medium"`, `"high"`, `"xhigh"`, o `"max"`. Assente quando il modello non supporta lo sforzo.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Attribuzione di skill, plugin, agente e MCP per la richiesta. Vedi [Contatore di costo](#cost-counter) per le definizioni e il comportamento di redazione.

<h4 id="api-error-event">
  Evento di errore API
</h4>

Registrato quando una richiesta API a Claude non riesce.

**Nome evento**: `claude_code.api_error`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `model`: Modello utilizzato (ad esempio, "claude-sonnet-5")
* `error`: Messaggio di errore
* `status_code`: Codice di stato HTTP come numero. Assente per errori non-HTTP come i guasti di connessione.
* `duration_ms`: Durata della richiesta in millisecondi
* `attempt`: Numero totale di tentativi effettuati, inclusa la richiesta iniziale (`1` significa che non si sono verificati tentativi)
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta, come `"req_011..."`. Presente solo quando l'API ne restituisce uno.
* `speed`: `"fast"` o `"normal"`, indicando se la modalità veloce era attiva
* `query_source`: Sottosistema che ha emesso la richiesta, come `"repl_main_thread"`, `"compact"`, o un nome di subagent
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta. Assente quando il modello non supporta lo sforzo.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Attribuzione di skill, plugin, agente e MCP per la richiesta. Vedi [Contatore di costo](#cost-counter) per le definizioni e il comportamento di redazione.

<h4 id="api-refusal-event">
  Evento di rifiuto API
</h4>

Registrato quando una richiesta API restituisce `stop_reason: "refusal"`. I rifiuti arrivano su un flusso di risposta riuscito piuttosto che come errore HTTP, quindi l'evento `api_error` non si attiva per loro. Questo evento ti consente di tracciare la frequenza dei rifiuti e raggruppare i rifiuti per gli stessi attributi di `api_request` e `api_error`.

**Nome evento**: `claude_code.api_refusal`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `model`: Identificatore del modello dalla richiesta
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta, come `"req_011..."`. Presente solo quando l'API ne restituisce uno.
* `query_source`: Sottosistema che ha emesso la richiesta, come `"repl_main_thread"`, `"compact"`, o un nome di subagent. Vedi [`api_request`](#api-request-event) per le definizioni.
* `speed`: `"fast"` quando [Fast mode](/docs/it/fast-mode) è attivo, o `"normal"`
* `attempt`: Numero del tentativo di ripetizione. Il primo tentativo è `1`.
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta. Assente quando il modello non supporta lo sforzo.
* `server_fallback_hop`: `true` quando il fallback del modello lato server dell'API ha già ritentato questo rifiuto su un modello diverso, quindi l'utente non ha visto questo particolare rifiuto. `false` quando la richiesta è terminata in un rifiuto. Un singolo turno può emettere sia un evento hop `true` che un successivo evento finale `false` quando il modello di fallback rifiuta anche.
* `has_category`: `true` quando la risposta API ha portato una `stop_details.category` di `"cyber"`, `"bio"`, `"frontier_llm"`, o `"reasoning_extraction"`. `false` quando la risposta non ha portato alcuna categoria o un valore al di fuori di quel set. Assente quando `server_fallback_hop` è `true`, perché i blocchi hop non portano `stop_details`.
* `has_explanation`: `true` quando la risposta API ha portato una `stop_details.explanation`, altrimenti `false`. Assente quando `server_fallback_hop` è `true`.
* `category`: Il valore `stop_details.category` dalla risposta API. Uno di `"cyber"`, `"bio"`, `"frontier_llm"`, o `"reasoning_extraction"`. Presente solo quando `OTEL_LOG_TOOL_DETAILS=1` è impostato e `has_category` è `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Attribuzione di skill, plugin, agente e MCP per la richiesta. Vedi [Contatore di costo](#cost-counter) per le definizioni e il comportamento di redazione.

<h4 id="api-request-body-event">
  Evento di corpo della richiesta API
</h4>

Registrato per ogni tentativo di richiesta API quando `OTEL_LOG_RAW_API_BODIES` è impostato. Un evento viene emesso per tentativo, quindi i tentativi con parametri regolati producono ciascuno il proprio evento.

**Nome evento**: `claude_code.api_request_body`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `body`: Parametri della richiesta dell'API Messages serializzati in JSON (prompt di sistema, messaggi, strumenti, ecc.), troncati a 60 KB. Il contenuto del pensiero esteso nei turni dell'assistente precedenti viene redatto. Emesso solo in modalità inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Percorso assoluto a un file `<dir>/<uuid>.request.json` contenente il corpo non troncato. Emesso solo in modalità file (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Lunghezza del corpo non troncato. Byte UTF-8 quando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, o unità di codice UTF-16 quando `=1`
* `body_truncated`: `"true"` quando si è verificato il troncamento inline. Assente in modalità file e quando non si è verificato alcun troncamento.
* `model`: Identificatore del modello dai parametri della richiesta
* `query_source`: Sottosistema che ha emesso la richiesta (ad esempio, `"compact"`)

<h4 id="api-response-body-event">
  Evento di corpo della risposta API
</h4>

Registrato per ogni risposta API riuscita quando `OTEL_LOG_RAW_API_BODIES` è impostato.

**Nome evento**: `claude_code.api_response_body`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `body`: Risposta dell'API Messages serializzata in JSON (id, blocchi di contenuto, utilizzo, motivo di arresto), troncata a 60 KB. Il contenuto del pensiero esteso viene redatto. Emesso solo in modalità inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Percorso assoluto a un file `<dir>/<request_id>.response.json` contenente il corpo non troncato. Emesso solo in modalità file (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Lunghezza del corpo non troncato. Byte UTF-8 quando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, o unità di codice UTF-16 quando `=1`
* `body_truncated`: `"true"` quando si è verificato il troncamento inline. Assente in modalità file e quando non si è verificato alcun troncamento.
* `model`: Identificatore del modello
* `query_source`: Sottosistema che ha emesso la richiesta
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta, come `"req_011..."`. Presente solo quando l'API ne restituisce uno.

<h4 id="tool-decision-event">
  Evento di decisione dello strumento
</h4>

Registrato quando viene presa una decisione di autorizzazione dello strumento (accetta/rifiuta).

**Nome evento**: `claude_code.tool_decision`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `tool_name`: Nome dello strumento (ad esempio, "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: Identificatore univoco per questa invocazione dello strumento. Corrisponde al `tool_use_id` passato agli hooks, consentendo la correlazione tra gli eventi OTel e i dati acquisiti dagli hooks.
* `decision`: `"accept"` o `"reject"`
* `source`: Fonte della decisione:
  * `"config"`: Deciso automaticamente senza richiedere, in base alle impostazioni del progetto, alle regole di autorizzazione nelle impostazioni personali dell'utente, alla politica gestita dall'azienda, ai flag `--allowedTools` o `--disallowedTools`, alla modalità di autorizzazione attiva, a una concessione con ambito di sessione da un prompt precedente nella stessa sessione CLI interattiva, o perché lo strumento è intrinsecamente sicuro. L'evento non indica quale di queste fonti corrisponde.
  * `"hook"`: Un hook `PreToolUse` o `PermissionRequest` ha restituito la decisione.
  * `"user_permanent"`: Emesso quando l'utente ha scelto "Sì, e non chiedere di nuovo per ..." in un prompt di autorizzazione, il che salva una regola di autorizzazione alle sue impostazioni personali. Nella CLI interattiva questo viene emesso solo per quella scelta stessa; le chiamate successive che corrispondono alla regola salvata emettono `"config"` invece. Nelle sessioni Agent SDK o non interattive `-p`, sia la scelta iniziale che le corrispondenze di regole successive emettono `"user_permanent"`. Trattato come un'accettazione.
  * `"user_temporary"`: Emesso quando l'utente ha scelto "Sì" in un prompt di autorizzazione per un'approvazione una tantum, o ha scelto una delle opzioni "... durante questa sessione" in un prompt di modifica o lettura di file. Nella CLI interattiva questo viene emesso solo per la scelta stessa; le chiamate successive consentite da quella concessione con ambito di sessione emettono `"config"` invece. Nelle sessioni Agent SDK o non interattive `-p`, sia la scelta che le corrispondenze successive emettono `"user_temporary"`. Trattato come un'accettazione.
  * `"user_abort"`: Emesso quando l'utente ha chiuso il prompt di autorizzazione senza rispondere. Trattato come un rifiuto.
  * `"user_reject"`: Emesso quando l'utente ha scelto "No" quando richiesto. Nella CLI interattiva questo viene emesso solo per quella scelta stessa; le chiamate che corrispondono a una regola di negazione nelle impostazioni personali dell'utente emettono `"config"` invece. Nelle sessioni Agent SDK o non interattive `-p`, le chiamate che corrispondono a una regola di negazione nelle impostazioni personali emettono `"user_reject"`. Trattato come un rifiuto.
* `tool_parameters` (quando `OTEL_LOG_TOOL_DETAILS=1`): Stringa JSON contenente parametri specifici dello strumento. Stessa forma dell'[Evento di risultato dello strumento](#tool-result-event), meno i campi post-esecuzione come `git_commit_id`. I valori possono differire da `tool_result` per una chiamata accettata se la decisione di autorizzazione riscrive l'input dello strumento tramite `updatedInput`. Usa questo attributo per vedere quale comando è stato rifiutato quando `decision` è `"reject"`.
  * Per lo strumento Bash: include `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`
  * Per lo strumento WorkspaceBash: include `bash_command`, `full_command`, `timeout`
  * Per gli strumenti MCP: include `mcp_server_name`, `mcp_tool_name`
  * Per lo strumento Skill: include `skill_name`
  * Per lo strumento Agent o lo strumento Task legacy: include `subagent_type`

<h4 id="permission-mode-changed-event">
  Evento di cambio della modalità di autorizzazione
</h4>

Registrato quando la modalità di autorizzazione cambia, ad esempio da ciclo Shift+Tab, uscita dalla Plan Mode, o un controllo del gate della modalità automatica.

**Nome evento**: `claude_code.permission_mode_changed`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `from_mode`: La modalità di autorizzazione precedente, ad esempio `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, o `"bypassPermissions"`
* `to_mode`: La nuova modalità di autorizzazione
* `trigger`: Cosa ha causato il cambio. Uno di `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"`, o `"auto_opt_in"`. Assente quando la transizione proviene dall'SDK o dal bridge

<h4 id="auth-event">
  Evento di autenticazione
</h4>

Registrato quando `/login` o `/logout` si completa.

**Nome evento**: `claude_code.auth`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `action`: `"login"` o `"logout"`
* `success`: `"true"` o `"false"`
* `auth_method`: Metodo di autenticazione, come `"oauth"`
* `error_category`: Tipo di errore categorico quando l'azione non è riuscita. Il messaggio di errore grezzo non viene mai incluso
* `status_code`: Codice di stato HTTP come stringa quando l'azione non è riuscita con un errore HTTP

<h4 id="mcp-server-connection-event">
  Evento di connessione del server MCP
</h4>

Registrato quando un server MCP si connette, si disconnette o non riesce a connettersi.

**Nome evento**: `claude_code.mcp_server_connection`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `status`: `"connected"`, `"failed"`, o `"disconnected"`
* `transport_type`: Trasporto del server, come `"stdio"`, `"sse"`, o `"http"`
* `server_scope`: Ambito in cui il server è configurato, come `"user"`, `"project"`, o `"local"`
* `duration_ms`: Durata del tentativo di connessione in millisecondi
* `error_code`: Codice di errore quando la connessione non è riuscita
* `is_plugin`: `true` quando il server è fornito da un plugin, `false` altrimenti
* `plugin_id_hash` (quando `is_plugin` è `true`): Hash stabile del nome del plugin e del marketplace, per raggruppare gli eventi per plugin senza esporre il nome
* `plugin.name` (quando `is_plugin` è `true`): Nome del plugin che fornisce il server. Per i plugin di terze parti questo è la stringa letterale `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1`; questo protegge i nomi dei plugin di terze parti dall'apparire nei log per impostazione predefinita. I plugin da fonti ufficiali Anthropic sono sempre identificati per nome. Gli attributi `plugin_id_hash` e `plugin.name` fluiscono al tuo backend di monitoraggio e non vengono inviati ad Anthropic
* `server_name` (quando `OTEL_LOG_TOOL_DETAILS=1`): Nome del server configurato
* `error` (quando `OTEL_LOG_TOOL_DETAILS=1`): Messaggio di errore completo quando la connessione non è riuscita

<h4 id="internal-error-event">
  Evento di errore interno
</h4>

Registrato quando Claude Code cattura un errore interno inaspettato. Viene registrato solo il nome della classe di errore e un codice di stile errno. Il messaggio di errore e la traccia dello stack non vengono mai inclusi. Questo evento non viene emesso quando si esegue su Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry, o quando `DISABLE_ERROR_REPORTING` è impostato.

**Nome evento**: `claude_code.internal_error`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `error_name`: Nome della classe di errore, come `"TypeError"` o `"SyntaxError"`
* `error_code`: Codice errno di Node.js come `"ENOENT"` quando presente nell'errore

<h4 id="plugin-installed-event">
  Evento di plugin installato
</h4>

Registrato quando un plugin finisce di installarsi, sia dal comando CLI `claude plugin install` che dall'interfaccia utente interattiva `/plugin`.

**Nome evento**: `claude_code.plugin_installed`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `marketplace.is_official`: `"true"` se il marketplace è un marketplace ufficiale Anthropic, `"false"` altrimenti
* `install.trigger`: `"cli"` o `"ui"`
* `plugin.name`: Nome del plugin installato. Per i marketplace di terze parti questo è incluso solo quando `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version`: Versione del plugin quando dichiarata nella voce del marketplace. Per i marketplace di terze parti questo è incluso solo quando `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace da cui è stato installato il plugin. Per i marketplace di terze parti questo è incluso solo quando `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Evento di plugin caricato
</h4>

Registrato una volta per ogni plugin abilitato all'avvio della sessione. Usa questo evento per inventariare quali plugin sono attivi nella tua flotta, come complemento a `plugin_installed` che registra l'azione di installazione stessa.

**Nome evento**: `claude_code.plugin_loaded`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `plugin.name`: Nome del plugin. Per i plugin al di fuori del marketplace ufficiale e del bundle incorporato il valore è `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace da cui è stato installato il plugin, quando noto. Redatto a `"third-party"` nella stessa condizione di `plugin.name`
* `plugin.version`: Versione dal manifesto del plugin. Incluso solo quando il nome non è redatto e il manifesto dichiara una versione
* `plugin.scope`: Categoria di provenienza per il plugin: `"official"`, `"org"`, `"user-local"`, o `"default-bundle"`
* `enabled_via`: Come il plugin è stato abilitato: `"default-enable"`, `"org-policy"`, `"seed-mount"`, o `"user-install"`
* `plugin_id_hash`: Hash deterministico del nome del plugin e del marketplace, inviato solo al tuo esportatore configurato. Ti consente di contare quanti plugin di terze parti distinti sono caricati nella tua flotta senza registrare i loro nomi
* `has_hooks`: Se il plugin contribuisce hooks
* `has_mcp`: Se il plugin contribuisce server MCP
* `host_owned_mcp`: `true` quando l'host SDK gestisce le connessioni MCP di questo plugin e Claude Code ha saltato la lettura della configurazione del server MCP del plugin, `false` altrimenti. Richiede Claude Code v2.1.172 o successivo
* `skill_path_count`: Numero di directory di skill che il plugin dichiara
* `command_path_count`: Numero di directory di comandi che il plugin dichiara
* `agent_path_count`: Numero di directory di agenti che il plugin dichiara
* `safe_mode`: `"true"` quando la sessione è stata avviata con [`--safe-mode`](/docs/it/cli-reference), `"false"` altrimenti. In modalità sicura questo evento segnala solo l'inventario configurato; i comandi, le skill, gli hooks e i server MCP del plugin non si caricano. Richiede Claude Code v2.1.169 o successivo

<h4 id="skill-activated-event">
  Evento di skill attivata
</h4>

Registrato quando una skill viene invocata, sia che Claude la chiami tramite lo strumento Skill sia che tu la esegua come comando `/`.

**Nome evento**: `claude_code.skill_activated`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `skill.name`: Nome della skill. Per le skill definite dall'utente e di terze parti il valore è il placeholder `"custom_skill"` a meno che `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Come la skill è stata attivata (`"user-slash"`, `"claude-proactive"`, o `"nested-skill"`)
* `skill.source`: Da dove è stata caricata la skill (ad esempio, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"` quando la skill è una skill di workflow. Assente altrimenti
* `plugin.name` (quando `OTEL_LOG_TOOL_DETAILS=1` o il plugin è da un marketplace ufficiale): Nome del plugin proprietario quando la skill è fornita da un plugin
* `marketplace.name` (quando `OTEL_LOG_TOOL_DETAILS=1` o il plugin è da un marketplace ufficiale): Marketplace del plugin proprietario da cui è stato installato, quando la skill è fornita da un plugin

<h4 id="at-mention-event">
  Evento di menzione @
</h4>

Registrato quando Claude Code risolve una menzione `@` in un prompt. Non ogni menzione emette un evento: i percorsi di uscita anticipata come i rifiuti di autorizzazione, i file di grandi dimensioni, gli allegati di riferimento PDF e i guasti dell'elenco delle directory vengono restituiti senza registrazione.

**Nome evento**: `claude_code.at_mention`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `mention_type`: Tipo di menzione (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`)
* `success`: Se la menzione è stata risolta con successo (`"true"` o `"false"`)

<h4 id="api-retries-exhausted-event">
  Evento di tentativi API esauriti
</h4>

Registrato una volta quando una richiesta API non riesce dopo più di un tentativo. Emesso insieme all'evento `api_error` finale.

**Nome evento**: `claude_code.api_retries_exhausted`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `model`: Modello utilizzato
* `error`: Messaggio di errore finale
* `status_code`: Codice di stato HTTP come numero. Assente per errori non-HTTP.
* `total_attempts`: Numero totale di tentativi effettuati
* `total_retry_duration_ms`: Tempo wall-clock totale tra tutti i tentativi
* `speed`: `"fast"` o `"normal"`

<h4 id="hook-registered-event">
  Evento di hook registrato
</h4>

Registrato una volta per ogni hook configurato all'avvio della sessione. Usa questo evento per inventariare quali hook sono attivi nella tua flotta, come complemento agli eventi per esecuzione `hook_execution_start` e `hook_execution_complete`.

**Nome evento**: `claude_code.hook_registered`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `hook_event`: Tipo di evento hook, come `"PreToolUse"` o `"PostToolUse"`
* `hook_type`: Tipo di implementazione dell'hook: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, o `"agent"`
* `hook_source`: Dove è definito l'hook: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, o `"pluginHook"`
* `safe_mode`: `"true"` quando la sessione è stata avviata con [`--safe-mode`](/docs/it/cli-reference), `"false"` altrimenti. Richiede Claude Code v2.1.169 o successivo
* `hook_matcher` (quando `OTEL_LOG_TOOL_DETAILS=1`): La stringa matcher dalla configurazione dell'hook, quando è impostata
* `plugin.name` (quando `hook_source` è `"pluginHook"`): Nome del plugin che contribuisce. Per i plugin al di fuori del marketplace ufficiale e del bundle incorporato il valore è `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (quando `hook_source` è `"pluginHook"`): Hash deterministico del nome del plugin e del marketplace, inviato solo al tuo esportatore configurato. Ti consente di contare i plugin che contribuiscono distinti senza registrare i loro nomi

<h4 id="hook-execution-start-event">
  Evento di inizio esecuzione dell'hook
</h4>

Registrato quando uno o più hook iniziano a eseguirsi per un evento hook.

**Nome evento**: `claude_code.hook_execution_start`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `hook_event`: Tipo di evento hook, come `"PreToolUse"` o `"PostToolUse"`
* `hook_name`: Nome completo dell'hook incluso il matcher, come `"PreToolUse:Write"`
* `num_hooks`: Numero di comandi hook corrispondenti
* `managed_only`: `"true"` quando sono consentiti solo gli hook della politica gestita
* `hook_source`: `"policySettings"` o `"merged"`
* `safe_mode`: `"true"` quando la sessione è stata avviata con [`--safe-mode`](/docs/it/cli-reference), `"false"` altrimenti. Richiede Claude Code v2.1.169 o successivo
* `hook_definitions`: Configurazione dell'hook serializzata in JSON. Incluso solo quando sia la traccia beta dettagliata che `OTEL_LOG_TOOL_DETAILS=1` sono abilitati

<h4 id="hook-execution-complete-event">
  Evento di completamento esecuzione dell'hook
</h4>

Registrato quando tutti gli hook per un evento hook hanno terminato.

**Nome evento**: `claude_code.hook_execution_complete`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `hook_event`: Tipo di evento hook
* `hook_name`: Nome completo dell'hook incluso il matcher
* `num_hooks`: Numero di comandi hook corrispondenti
* `num_success`: Conteggio che ha completato con successo
* `num_blocking`: Conteggio che ha restituito una decisione di blocco
* `num_non_blocking_error`: Conteggio che non ha avuto esito positivo senza bloccare
* `num_cancelled`: Conteggio annullato prima del completamento
* `total_duration_ms`: Durata wall-clock di tutti gli hook corrispondenti
* `managed_only`: `"true"` quando sono consentiti solo gli hook della politica gestita
* `hook_source`: `"policySettings"` o `"merged"`
* `safe_mode`: `"true"` quando la sessione è stata avviata con [`--safe-mode`](/docs/it/cli-reference), `"false"` altrimenti. Richiede Claude Code v2.1.169 o successivo
* `hook_definitions`: Configurazione dell'hook serializzata in JSON. Incluso solo quando sia la traccia beta dettagliata che `OTEL_LOG_TOOL_DETAILS=1` sono abilitati

<h4 id="hook-plugin-metrics-event">
  Evento di metriche plugin hook
</h4>

Registrato quando un hook plugin del marketplace ufficiale emette metriche per invocazione. Solo i plugin installati da un marketplace ufficiale Anthropic possono emettere questi. I plugin di marketplace di terze parti e gli hook configurati dall'utente non emettono a questo evento. Usa questo evento per monitorare il comportamento del plugin come i tassi di ricerca, i costi e le durate dal tuo stack di osservabilità.

**Nome evento**: `claude_code.hook_plugin_metrics`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `plugin_id`: Identificatore del plugin in forma `<name>@<marketplace>`
* `hook_event`: Tipo di evento hook che ha emesso le metriche
* Fino a 20 chiavi di metriche emesse dal plugin. I nomi corrispondono a `^[a-z][a-z0-9_]{0,39}$`. I valori sono booleani o numeri.

<h4 id="compaction-event">
  Evento di compattazione
</h4>

Registrato quando la compattazione della conversazione si completa.

**Nome evento**: `claude_code.compaction`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `trigger`: `"auto"` o `"manual"`
* `success`: `"true"` o `"false"`
* `duration_ms`: Durata della compattazione
* `pre_tokens`: Conteggio approssimativo dei token prima della compattazione
* `post_tokens`: Conteggio approssimativo dei token dopo la compattazione
* `error`: Messaggio di errore quando la compattazione non è riuscita
* `precompute_reuse`: Impostato solo quando `trigger` è `"manual"`. La compattazione automatica può preparare un riepilogo in background prima che la finestra di contesto si riempia, e questo attributo registra se `/compact` ha riutilizzato quel riepilogo preparato. `"hit"` significa che è stato riutilizzato; `"miss_custom_instructions"`, `"miss_hook"`, e `"miss_not_ready"` forniscono il motivo per cui è stato calcolato un riepilogo fresco. Richiede Claude Code v2.1.153 o successivo

<h4 id="feedback-survey-event">
  Evento di sondaggio di feedback
</h4>

Registrato quando viene mostrato o risposto a un sondaggio sulla qualità della sessione. Vedi [Sondaggi sulla qualità della sessione](/docs/it/data-usage#session-quality-surveys) per sapere cosa raccolgono i sondaggi e come controllarli.

**Nome evento**: `claude_code.feedback_survey`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contatore monotonicamente crescente per ordinare gli eventi all'interno di una sessione
* `event_type`: Evento del ciclo di vita del sondaggio, ad esempio `"appeared"`, `"responded"`, o `"transcript_prompt_appeared"`
* `appearance_id`: ID univoco che collega gli eventi emessi per un'istanza di sondaggio
* `survey_type`: Quale sondaggio ha prodotto l'evento. `"session"` è il prompt di valutazione "Come sta andando Claude?"
* `response`: La selezione dell'utente su eventi `responded`
* `enabled_via_override`: `true` quando [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/it/env-vars) è impostato. Emesso come booleano, non come stringa. Presente su eventi di sondaggio `session`. Filtra su questo attributo per confermare che l'override è applicato in tutta la flotta

<h2 id="interpret-metrics-and-events-data">
  Interpretazione dei dati di metriche e eventi
</h2>

Le metriche e gli eventi esportati supportano una gamma di analisi:

<h3 id="usage-monitoring">
  Monitoraggio dell'utilizzo
</h3>

| Metrica                                                       | Opportunità di analisi                                                                                  |
| ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `claude_code.token.usage`                                     | Suddividi per `type` (input/output), utente, team, modello, `skill.name`, `plugin.name`, o `agent.name` |
| `claude_code.session.count`                                   | Traccia l'adozione e l'engagement nel tempo                                                             |
| `claude_code.lines_of_code.count`                             | Misura la produttività tracciando le aggiunte e le rimozioni di codice, suddivise per modello           |
| `claude_code.commit.count` & `claude_code.pull_request.count` | Comprendi l'impatto sui flussi di lavoro di sviluppo                                                    |

<h3 id="cost-monitoring">
  Monitoraggio dei costi
</h3>

La metrica `claude_code.cost.usage` aiuta con:

* Tracciare i trend di utilizzo tra team o individui
* Identificare sessioni ad alto utilizzo per l'ottimizzazione
* Attribuire la spesa a skill, plugin, o tipi di subagent specifici tramite gli attributi `skill.name`, `plugin.name`, e `agent.name`

<Note>
  Le metriche di costo sono approssimazioni. Per i dati di fatturazione ufficiali, consulta il tuo provider API (Claude Console, Amazon Bedrock, o Google Cloud's Agent Platform).
</Note>

<h3 id="alerting-and-segmentation">
  Avvisi e segmentazione
</h3>

Avvisi comuni da considerare:

* Picchi di costo
* Consumo di token inusuale
* Alto volume di sessioni da utenti specifici

Tutte le metriche possono essere segmentate dagli [attributi standard](#standard-attributes). L'attributo `model` è disponibile su `claude_code.token.usage`, `claude_code.cost.usage`, e da v2.1.172, `claude_code.lines_of_code.count`.

Per-model breakdowns of commits can only be approximated by joining against the token or cost metrics on `session.id`, since one session can span multiple models. Filter the token or cost side to rows where `query_source` is `"main"` so auxiliary and subagent requests don't attribute the session's commits to a model that didn't make them.

<h3 id="detect-retry-exhaustion">
  Rilevare l'esaurimento dei tentativi
</h3>

Claude Code ritenta internamente le richieste API non riuscite ed emette un singolo evento `claude_code.api_error` solo dopo che rinuncia, quindi l'evento stesso è il segnale terminale per quella richiesta. I tentativi di ripetizione intermedi non vengono registrati come eventi separati.

L'attributo `attempt` sull'evento registra il numero totale di tentativi. `CLAUDE_CODE_MAX_RETRIES` ha un valore predefinito di 10 ed è limitato a 15; a partire da v2.1.199, `CLAUDE_CODE_RETRY_WATCHDOG` aumenta il valore predefinito e rimuove il limite. Quando la richiesta esaurisce tutti i tentativi su un errore transitorio, `attempt` è uguale a uno più di quel limite effettivo: 11 per impostazione predefinita, e mai più di 16 a meno che il watchdog non sia impostato. Un valore inferiore indica un errore non ritentabile come una risposta `400`.

Per distinguere una sessione che si è ripresa da una che si è bloccata, raggruppa gli eventi per `session.id` e verifica se esiste un evento `api_request` successivo dopo l'errore.

<h3 id="event-analysis">
  Analisi degli eventi
</h3>

I dati degli eventi forniscono informazioni dettagliate sulle interazioni di Claude Code:

**Modelli di utilizzo dello strumento**: analizza gli eventi di risultato dello strumento per identificare:

* Strumenti più frequentemente utilizzati
* Tassi di successo dello strumento
* Tempi di esecuzione medi dello strumento
* Modelli di errore per tipo di strumento

**Monitoraggio delle prestazioni**: traccia le durate delle richieste API e i tempi di esecuzione dello strumento per identificare i colli di bottiglia delle prestazioni.

<h2 id="audit-security-events">
  Audit degli eventi di sicurezza
</h2>

Gli eventi OpenTelemetry sono la fonte di dati di audit per l'attività di Claude Code. Ogni evento porta attributi di identità che collegano le chiamate agli strumenti, l'attività MCP e le decisioni di autorizzazione all'utente che le ha attivate. L'esportatore di log OTLP può fornire questi eventi a qualsiasi piattaforma SIEM (Security Information and Event Management) con un ricevitore OTLP, o a un OpenTelemetry Collector che inoltra al vostro SIEM.

<h3 id="attribute-actions-to-users">
  Attribuisci le azioni agli utenti
</h3>

Gli [attributi standard](#standard-attributes) su ogni evento includono l'identità dell'utente autenticato: `user.email`, `user.account_uuid`, `user.account_id`, e `organization.id` quando accedete con un account Claude, più `user.id` e il `session.id` per sessione. `user.id` è un identificatore con ambito di installazione, tranne nelle sessioni [Claude apps gateway](/docs/it/claude-apps-gateway), dove è il soggetto IdP dal token emesso dal gateway.

Le chiamate agli strumenti MCP, i comandi Bash e le modifiche ai file sono quindi attribuite allo sviluppatore che ha avviato la sessione. Claude Code non agisce con un account di servizio separato; l'identità registrata su ogni evento è l'account Claude dello sviluppatore, o l'identità IdP dello sviluppatore in una sessione [Claude apps gateway](/docs/it/claude-apps-gateway).

Quando Claude Code si autentica con una chiave API diretta, o contro Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry, non c'è un account Claude nella sessione e solo `user.id` e `session.id` vengono popolati. In questi deployment, allegate l'identità dell'utente voi stessi con `OTEL_RESOURCE_ATTRIBUTES`, impostato per utente tramite il file [impostazioni gestite](#administrator-configuration) o un wrapper di avvio. Le sessioni Claude apps gateway non hanno bisogno di nulla di tutto questo: la CLI applica l'identità IdP automaticamente, come descritto in [Attributi standard](#standard-attributes).

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

<h3 id="audit-mcp-activity">
  Audit dell'attività MCP
</h3>

Per acquisire l'attività del server MCP con dettagli completi della chiamata, abilitate l'esportatore di log e impostate `OTEL_LOG_TOOL_DETAILS=1`. Ogni operazione MCP produce quindi eventi strutturati che portano il nome del server, il nome dello strumento e gli argomenti della chiamata insieme agli attributi di identità standard:

| Evento                  | Cosa registra per MCP                                                                                                                                                                                                  |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_server_connection` | Connessione del server, disconnessione e guasto di connessione con `server_name`, `transport_type`, `server_scope`, e dettagli dell'errore                                                                             |
| `tool_result`           | Ogni chiamata dello strumento MCP con `tool_name` e `mcp_server_scope`, un payload `tool_parameters` contenente `mcp_server_name` e `mcp_tool_name`, e un payload `tool_input` contenente gli argomenti della chiamata |
| `tool_decision`         | Se la chiamata è stata consentita o negata, se la decisione proveniva da config, un hook, o l'utente, e un payload `tool_parameters` contenente `mcp_server_name` e `mcp_tool_name`                                    |

Senza `OTEL_LOG_TOOL_DETAILS`, questi eventi riducono il dettaglio identificativo:

* `tool_result`: mantiene `tool_name` e `mcp_server_scope`, omette `mcp_server_name`, `mcp_tool_name`, e argomenti
* `tool_decision`: mantiene `tool_name`, omette `tool_parameters`
* `mcp_server_connection`: omette `server_name` e il messaggio di errore, ma mantiene `is_plugin`, `plugin_id_hash`, e `plugin.name`, con i nomi dei plugin non Anthropic redatti al letterale `"third-party"`, in modo che i server forniti dai plugin rimangono distinguibili senza registrazione dettagliata

<h3 id="map-security-questions-to-events">
  Mappa le domande di sicurezza agli eventi
</h3>

Quando costruite regole di rilevamento, cercate il segnale che volete monitorare e interrogate il vostro backend per l'evento corrispondente e gli attributi:

| Segnale                                                 | Evento                                                                               | Attributi chiave                                             |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------ |
| Chiamata dello strumento consentita o negata, e da cosa | `tool_decision`                                                                      | `decision`, `source`, `tool_name`, `tool_parameters`         |
| Escalation della modalità di autorizzazione             | `permission_mode_changed`                                                            | `from_mode`, `to_mode`, `trigger`                            |
| Hook della politica ha bloccato un'azione               | `hook_execution_complete`                                                            | `hook_event`, `num_blocking`                                 |
| Login, logout e guasto di autenticazione                | `auth`                                                                               | `action`, `success`, `error_category`                        |
| Connessione del server MCP o guasto                     | `mcp_server_connection`                                                              | `status`, `server_name`, `is_plugin`, `error_code`           |
| Plugin installato e la sua fonte                        | `plugin_installed`                                                                   | `plugin.name`, `marketplace.name`, `marketplace.is_official` |
| Comandi eseguiti e file toccati                         | `tool_result` (eseguito) o `tool_decision` (rifiutato) con `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (`tool_result` solo)         |

Claude Code emette solo il flusso di eventi grezzo. Il rilevamento delle anomalie, il baselining, la correlazione tra sessioni e gli avvisi sono responsabilità del vostro SIEM o backend di osservabilità.

<h3 id="send-events-to-a-siem">
  Invia gli eventi a un SIEM
</h3>

Puntate `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` al ricevitore OTLP del vostro SIEM, o a un OpenTelemetry Collector che inoltra all'API di ingestione nativa del vostro SIEM. Il seguente esempio di impostazioni gestite esporta solo gli eventi, con dettagli completi dello strumento abilitati per il controllo MCP e Bash:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_LOG_TOOL_DETAILS": "1",
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT": "https://siem.example.com:4318/v1/logs",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-siem-token"
  }
}
```

<h2 id="backend-considerations">
  Considerazioni sul backend
</h2>

La scelta dei backend di metriche, log e tracce determina i tipi di analisi che puoi eseguire:

<h3 id="for-metrics">
  Per le metriche
</h3>

* **Database di serie temporali (ad esempio, Prometheus)**: Calcoli di velocità, metriche aggregate
* **Archivi colonnari (ad esempio, ClickHouse)**: Query complesse, analisi di utenti univoci
* **Piattaforme di osservabilità complete (ad esempio, Honeycomb, Datadog, Grafana Cloud)**: Query avanzate, visualizzazione, avvisi

<h3 id="for-events/logs">
  Per eventi/log
</h3>

* **Sistemi di aggregazione dei log (ad esempio, Elasticsearch, Loki)**: Ricerca full-text, analisi dei log
* **Archivi colonnari (ad esempio, ClickHouse)**: Analisi degli eventi strutturati
* **Piattaforme di osservabilità complete (ad esempio, Honeycomb, Datadog, Grafana Cloud)**: Correlazione tra metriche e eventi

<h3 id="for-traces">
  Per tracce
</h3>

Scegli un backend che supporti l'archiviazione di tracce distribuite e la correlazione degli span:

* **Sistemi di traccia distribuita (ad esempio, Jaeger, Zipkin, Grafana Tempo)**: Visualizzazione degli span, waterfall delle richieste, analisi della latenza
* **Piattaforme di osservabilità complete (ad esempio, Honeycomb, Datadog, Grafana Cloud)**: Ricerca di tracce e correlazione con metriche e log

Per le organizzazioni che richiedono metriche Daily/Weekly/Monthly Active User (DAU/WAU/MAU), considera backend che supportano query di valori univoci efficienti.

<h2 id="service-information">
  Informazioni sul servizio
</h2>

Tutte le metriche e gli eventi vengono esportati con i seguenti attributi di risorsa:

* `service.name`: `claude-code`
* `service.version`: Versione corrente di Claude Code
* `os.type`: Tipo di sistema operativo (ad esempio, `linux`, `darwin`, `windows`)
* `os.version`: Stringa della versione del sistema operativo
* `host.arch`: Architettura dell'host (ad esempio, `amd64`, `arm64`)
* `wsl.version`: Numero di versione WSL (presente solo quando si esegue su Windows Subsystem for Linux)
* Meter Name: `com.anthropic.claude_code`

<h2 id="roi-measurement-resources">
  Risorse di misurazione del ROI
</h2>

Per una guida completa sulla misurazione del ritorno sull'investimento per Claude Code, inclusa la configurazione della telemetria, l'analisi dei costi, le metriche di produttività e i report automatizzati, consulta la [Guida alla misurazione del ROI di Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Questo repository fornisce configurazioni Docker Compose pronte all'uso, configurazioni Prometheus e OpenTelemetry, e modelli per generare report di produttività integrati con strumenti come Linear.

<h2 id="security-and-privacy">
  Sicurezza e privacy
</h2>

* L'esportazione OpenTelemetry al tuo backend è opt-in e richiede una configurazione esplicita. Per la telemetria operazionale separata di Anthropic e come disabilitarla, vedi [Data usage](/docs/it/data-usage#telemetry-services)
* I contenuti dei file grezzi e i frammenti di codice non sono inclusi nelle metriche o negli eventi. I percorsi di traccia degli span sono un percorso dati separato: vedi il punto `OTEL_LOG_TOOL_CONTENT` di seguito
* Quando autenticato tramite OAuth, `user.email` è incluso negli attributi di telemetria. Se questo è una preoccupazione per la tua organizzazione, lavora con il tuo backend di telemetria per filtrare o redarre questo campo
* Il contenuto del prompt dell'utente non viene raccolto per impostazione predefinita. Viene registrata solo la lunghezza del prompt. Per includere il contenuto del prompt, imposta `OTEL_LOG_USER_PROMPTS=1`
* Il testo della risposta dell'assistente non viene raccolto per impostazione predefinita. Viene registrata solo la lunghezza della risposta. Per includere il testo della risposta, imposta `OTEL_LOG_ASSISTANT_RESPONSES=1`. Come tutti i dati OpenTelemetry da Claude Code, il testo della risposta viene inviato solo all'endpoint OTel che configuri, mai ad Anthropic. Quando questa variabile non è impostata, `OTEL_LOG_USER_PROMPTS` viene utilizzato come fallback, quindi imposta `OTEL_LOG_ASSISTANT_RESPONSES=0` se desideri il contenuto del prompt senza il contenuto della risposta
* Gli argomenti di input dello strumento e i parametri non vengono registrati per impostazione predefinita. Per includerli, imposta `OTEL_LOG_TOOL_DETAILS=1`. Questi dati vengono inviati solo all'endpoint OTEL che configuri, mai ad Anthropic. Gli argomenti potrebbero comunque contenere valori sensibili, quindi configura il tuo backend di telemetria per filtrare o redarre questi attributi secondo necessità. Quando abilitato:
  * Gli eventi `tool_result` e `tool_decision` includono un attributo `tool_parameters` con comandi Bash, nomi dei server MCP e dello strumento, e nomi delle skill. I campi come `full_command` vengono emessi non troncati
  * Gli eventi `tool_result` includono inoltre un attributo `tool_input` con percorsi di file, URL, modelli di ricerca e altri argomenti. I singoli valori superiori a 512 caratteri vengono troncati e il totale è limitato a circa 4 K caratteri
  * Gli eventi `user_prompt` includono il `command_name` verbatim per i comandi personalizzati, plugin e MCP
  * Gli span di traccia includono lo stesso attributo `tool_input` e attributi derivati dall'input come `file_path`, con lo stesso troncamento di `tool_input`
* Il contenuto di input e output dello strumento non viene registrato negli span di traccia per impostazione predefinita. Per includerlo, imposta `OTEL_LOG_TOOL_CONTENT=1`. Quando abilitato, gli eventi di span includono il contenuto completo di input e output dello strumento troncato a 60 KB per span. Questo può includere contenuti di file grezzi dai risultati dello strumento Read e output dei comandi Bash. Configura il tuo backend di telemetria per filtrare o redarre questi attributi secondo necessità
* I corpi grezzi della richiesta e della risposta dell'API Anthropic Messages non vengono registrati per impostazione predefinita. Per includerli, imposta `OTEL_LOG_RAW_API_BODIES`. Con `=1`, ogni chiamata API emette eventi di log `api_request_body` e `api_response_body` il cui attributo `body` è il payload serializzato in JSON, troncato a 60 KB. Con `=file:<dir>`, i corpi non troncati vengono scritti in file `.request.json` e `.response.json` in quella directory e gli eventi portano un percorso `body_ref` invece del corpo inline. Spedisci la directory con un log collector o sidecar piuttosto che attraverso il flusso di telemetria. In entrambe le modalità, i corpi contengono l'intera cronologia della conversazione (prompt di sistema, ogni turno precedente dell'utente e dell'assistente, risultati degli strumenti), quindi l'abilitazione di questa opzione implica il consenso a tutto ciò che gli altri flag di contenuto `OTEL_LOG_*` rivelerebbero. Il contenuto del pensiero esteso di Claude viene sempre redatto da questi corpi indipendentemente da altre impostazioni

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Monitoraggio di Claude Code su Amazon Bedrock
</h2>

Per una guida dettagliata al monitoraggio dell'utilizzo di Claude Code per Amazon Bedrock, consulta [Claude Code Monitoring Implementation (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
