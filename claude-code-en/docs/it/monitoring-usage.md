> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Monitoraggio

> Scopri come abilitare e configurare OpenTelemetry per Claude Code.

Traccia l'utilizzo di Claude Code, i costi e l'attività degli strumenti in tutta l'organizzazione esportando i dati di telemetria tramite OpenTelemetry (OTel). Claude Code esporta le metriche come dati di serie temporali tramite il protocollo di metriche standard, gli eventi tramite il protocollo di log/eventi, e facoltativamente le tracce distribuite tramite il [protocollo di tracce](#traces-beta).

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

# 5. Per il debug: riduci gli intervalli di esportazione e ripristinali per l'uso in produzione
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 secondi (predefinito: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 secondi (predefinito: 5000ms)

# 6. Esegui Claude Code
claude
```

Per verificare una configurazione che esporta metriche, controlla il tuo backend per la metrica `claude_code.session.count`, che Claude Code emette quando inizia una sessione. Per verificare una configurazione solo log, invia un prompt e controlla l'evento `claude_code.user_prompt`.

Se non arriva nulla, esegui `claude --debug` e controlla il log di debug. Claude Code segnala i guasti degli esportatori che configuri come errori `[3P telemetry]`, dove 3P significa third-party. Le righe con prefisso `[Anthropic telemetry]` descrivono la [telemetria operativa separata di Anthropic](/docs/it/data-usage#telemetry-services) e non indicano un problema con la tua configurazione.

Per le opzioni di configurazione complete, consulta la [specifica OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Configurazione dell'amministratore
</h2>

Gli amministratori possono configurare le impostazioni di OpenTelemetry per tutti gli utenti tramite il [file di impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms). Consulta la [precedenza delle impostazioni](/docs/it/settings#settings-precedence) per ulteriori informazioni su come vengono applicate le impostazioni.

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

Claude Code ignora le [variabili dell'esportatore OpenTelemetry](/docs/it/settings-reference#variables-claude-code-ignores-in-env) nel `.claude/settings.json` e `.claude/settings.local.json` di un repository, quindi un repository non può usarle per attivare la telemetria, scegliere dove va, o acquisire contenuti. Impostale nelle impostazioni gestite, oppure fai in modo che ogni sviluppatore le imposti nella propria shell o in `~/.claude/settings.json`. Un repository può comunque disattivare un segnale impostando il suo selettore di esportatore, come `OTEL_LOGS_EXPORTER`, su `none`, a meno che le impostazioni gestite, un file `--settings`, o l'ambiente da cui avvii Claude Code imposti quella variabile.

Claude Code non passa le variabili di ambiente `OTEL_*` ai sottoprocessi che genera, incluso lo strumento Bash, gli hooks, i server MCP e i language server. Un'applicazione strumentata con OpenTelemetry che esegui tramite lo strumento Bash non eredita l'endpoint dell'esportatore di Claude Code o le intestazioni, quindi imposta quelle variabili direttamente nel comando se quell'applicazione ha bisogno di esportare la propria telemetria.

<h3 id="how-managed-settings-lock-the-otlp-destination">
  Come le impostazioni gestite bloccano la destinazione OTLP
</h3>

Quando imposti una variabile `OTEL_EXPORTER_OTLP_*` nelle impostazioni gestite, Claude Code rimuove le variabili conflittuali impostate dagli sviluppatori all'avvio e registra un avviso che puoi visualizzare con `claude --debug`. Ciò che rimuove dipende da quale variabile imposti:

* **Endpoint**: quando imposti `OTEL_EXPORTER_OTLP_ENDPOINT`, Claude Code rimuove ogni endpoint per segnale impostato dagli sviluppatori. Gli sviluppatori non possono puntare un segnale a un collector diverso, quindi non è necessario impostare anche le variabili di endpoint per segnale nelle impostazioni gestite.
* **Protocolli**: quando imposti `OTEL_EXPORTER_OTLP_PROTOCOL`, Claude Code rimuove ogni protocollo per segnale impostato dagli sviluppatori.
* **Credenziali**: quando imposti `OTEL_EXPORTER_OTLP_HEADERS`, `OTEL_EXPORTER_OTLP_CLIENT_KEY` o `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, Claude Code rimuove le versioni per segnale di quella variabile impostate dagli sviluppatori, più ogni variabile di endpoint impostata dagli sviluppatori, generica o per segnale, poiché quelle credenziali raggiungerebbero altrimenti un collector che le impostazioni gestite non hanno scelto.
* **Selettori di esportatore**: `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` e il beta `OTEL_TRACES_EXPORTER` seguono la precedenza normale per chiave. L'impostazione di uno sviluppatore può comunque disabilitare un segnale o passarlo all'esportatore della console, quindi imposta i selettori anche nelle impostazioni gestite se hai bisogno che siano bloccati. Tra le [fonti amministratore](/docs/it/managed-settings#precedence-within-the-managed-tier), `OTEL_LOGS_EXPORTER` segue l'[unità di telemetria](/docs/it/server-managed-settings#per-key-exceptions-across-managed-sources) mentre gli altri due selettori si uniscono per chiave. Richiede Claude Code v2.1.223 o successivo.
* **Endpoint di tracciamento beta**: con il [tracciamento beta dettagliato](#traces-beta) attivo, Claude Code esporta log e tracce a `BETA_TRACING_ENDPOINT` invece che tramite gli esportatori di log e tracce. Claude Code quindi rimuove un `BETA_TRACING_ENDPOINT` impostato dagli sviluppatori ogni volta che una di queste impostazioni gestite decide la destinazione di uno dei due segnali:

  * Un endpoint generico o di log/tracce o una credenziale
  * Un [`otelHeadersHelper`](/docs/it/settings-reference#otelheadershelper)
  * Un selettore di esportatore di log o tracce impostato su `none`, `console` o vuoto, valori che mantengono il segnale fuori da un collector
  * `CLAUDE_CODE_ENABLE_TELEMETRY` disattivato

  Un endpoint o una credenziale solo per metriche non lo rimuove. Prima della v2.1.251, un `BETA_TRACING_ENDPOINT` impostato dagli sviluppatori reindirizzava i log e le tracce che il tracciamento beta dettagliato esporta anche quando le impostazioni gestite hanno fissato il collector.

Claude Code non rimuove le variabili per segnale che imposti nelle impostazioni gestite stesse, quindi puoi instradare un segnale a un collector diverso impostando la sua variabile lì, come fa l'[esempio SIEM](#send-events-to-a-siem). Se imposti una credenziale per segnale lì, Claude Code rimuove l'endpoint impostato dagli sviluppatori per quel segnale.

Questo comportamento di rimozione cambia dove viene consegnata la telemetria, non cosa raccoglie Claude Code.

Prima della v2.1.217, ogni variabile seguiva la precedenza delle impostazioni per chiave indipendentemente, quindi un endpoint specifico del segnale impostato nelle impostazioni dell'utente o nella shell reindirizzava quel segnale lontano dal collector gestito.

Quando l'app desktop o un runner di [ambiente self-hosted](/docs/it/self-hosted-environments) avvia Claude Code e nomina un endpoint OTLP nell'ambiente che fornisce, Claude Code fissa la destinazione allo stesso modo: le variabili di telemetria del launcher rimuovono le variabili impostate dagli sviluppatori esattamente come fanno le impostazioni gestite. Claude Code non rimuove le variabili che il launcher stesso ha impostato. Richiede Claude Code v2.1.251 o successivo.

<h2 id="configuration-details">
  Dettagli della configurazione
</h2>

<h3 id="common-configuration-variables">
  Variabili di configurazione comuni
</h3>

Queste variabili configurano esportatori, endpoint e comportamento di esportazione per tutti i deployment. Se imposti una variabile di endpoint o protocollo per segnale specifico, come `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`, Claude Code la utilizza al posto della variabile generica per quel segnale. Se imposti una variabile di intestazioni per segnale specifico, come `OTEL_EXPORTER_OTLP_METRICS_HEADERS`, Claude Code la unisce con la generica `OTEL_EXPORTER_OTLP_HEADERS` per quel segnale.

Su macchine con impostazioni gestite, vedi [Come le impostazioni gestite bloccano la destinazione OTLP](#how-managed-settings-lock-the-otlp-destination) per sapere cosa Claude Code rimuove.

| Variabile di ambiente                               | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Valori di esempio                                                                                                                                                                 |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Abilita la raccolta della telemetria (obbligatorio)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `1`                                                                                                                                                                               |
| `OTEL_METRICS_EXPORTER`                             | Tipi di esportatore di metriche, separati da virgola. Usa `none` per disabilitare                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `console`, `otlp`, `prometheus`, `none`                                                                                                                                           |
| `OTEL_LOGS_EXPORTER`                                | Tipi di esportatore di log/eventi, separati da virgola. Usa `none` per disabilitare                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `console`, `otlp`, `none`                                                                                                                                                         |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protocollo per l'esportatore OTLP, si applica a tutti i segnali. Claude Code non ha un protocollo predefinito, quindi imposta questo o la variabile di protocollo specifica per segnale per ogni esportatore `otlp` che abiliti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `grpc`, `http/json`, `http/protobuf`                                                                                                                                              |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Endpoint del collettore OTLP per tutti i segnali                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `http://localhost:4317`                                                                                                                                                           |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protocollo per le metriche, sostituisce l'impostazione generale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `grpc`, `http/json`, `http/protobuf`                                                                                                                                              |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Endpoint OTLP per le metriche, sostituisce l'impostazione generale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `http://localhost:4318/v1/metrics`                                                                                                                                                |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protocollo per i log, sostituisce l'impostazione generale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `grpc`, `http/json`, `http/protobuf`                                                                                                                                              |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Endpoint OTLP per i log, sostituisce l'impostazione generale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `http://localhost:4318/v1/logs`                                                                                                                                                   |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Intestazioni di autenticazione per OTLP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `Authorization=Bearer token`                                                                                                                                                      |
| `OTEL_EXPORTER_OTLP_METRICS_HEADERS`                | Intestazioni di autenticazione per le metriche, unite con le intestazioni generali                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `Authorization=Bearer token`                                                                                                                                                      |
| `OTEL_EXPORTER_OTLP_LOGS_HEADERS`                   | Intestazioni di autenticazione per i log, unite con le intestazioni generali                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `Authorization=Bearer token`                                                                                                                                                      |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Intervallo di esportazione in millisecondi (predefinito: 60000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `5000`, `60000`                                                                                                                                                                   |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Intervallo di esportazione dei log in millisecondi (predefinito: 5000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `1000`, `10000`                                                                                                                                                                   |
| `OTEL_LOG_USER_PROMPTS`                             | Abilita la registrazione del contenuto del prompt dell'utente (predefinito: disabilitato)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `1` per abilitare                                                                                                                                                                 |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Abilita la registrazione del testo della risposta dell'assistente negli eventi `assistant_response` (predefinito: disabilitato). Se non impostato, ricade al valore di `OTEL_LOG_USER_PROMPTS`. Richiede Claude Code v2.1.193 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `1` per abilitare, `0` per mantenerlo oscurato                                                                                                                                    |
| `OTEL_LOG_TOOL_DETAILS`                             | Abilita la registrazione dei parametri dello strumento e degli argomenti di input negli eventi dello strumento e negli attributi di span di traccia: comandi Bash, nomi dei server e degli strumenti MCP, nomi delle skill, nomi dei workflow creati dall'utente e input degli strumenti. Abilita anche i nomi dei comandi personalizzati, plugin e MCP negli eventi `user_prompt` (predefinito: disabilitato). Per i server integrati di Claude Desktop, nelle sessioni di proprietà di Claude Desktop, `mcp_server_name`/`mcp_tool_name` vengono emessi su `tool_decision`/`tool_result` anche con il flag disattivato. L'eccezione richiede Claude Code v2.1.214 o successivo                                                                                                                                   | `1` per abilitare                                                                                                                                                                 |
| `OTEL_LOG_TOOL_CONTENT`                             | Abilita la registrazione del contenuto dello strumento nell'evento di span [`tool.output`](#tool-output-span-event) (predefinito: disabilitato). Gli attributi di span portano il contenuto dello strumento secondo i loro [gate specifici](#new-context-gates). Richiede [tracce](#traces-beta). Il contenuto viene troncato al limite di contenuto (60 KB per impostazione predefinita)                                                                                                                                                                                                                                                                                                                                                                                                                          | `1` per abilitare                                                                                                                                                                 |
| `OTEL_LOG_MANAGED_SETTINGS`                         | Aggiungi le impostazioni gestite oscurate e un digest SHA-256 delle impostazioni prima dell'oscuramento agli eventi [managed settings resolved](#managed-settings-resolved-event) (predefinito: disabilitato). Un valore nelle impostazioni di progetto o locali non lo attiva. Richiede Claude Code v2.1.274 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `1` per abilitare                                                                                                                                                                 |
| `OTEL_LOG_RAW_API_BODIES`                           | Emetti il corpo completo della richiesta e della risposta JSON dell'API Anthropic Messages come eventi di log `api_request_body` / `api_response_body` (predefinito: disabilitato). I corpi includono l'intera cronologia della conversazione. L'abilitazione di questa opzione implica il consenso a tutto ciò che `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS`, e `OTEL_LOG_TOOL_CONTENT` rivelerebbero                                                                                                                                                                                                                                                                                                                                                                                                      | `1` per corpi inline troncati al limite di contenuto (60 KB per impostazione predefinita), o `file:<dir>` per corpi non troncati su disco con un puntatore `body_ref` nell'evento |
| `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH`               | Limite di contenuto: la lunghezza massima degli attributi che portano contenuto come risposte del modello, contenuto dello strumento, prompt di sistema e corpi API grezzi, marcatore di troncamento incluso, in unità di codice UTF-16 (predefinito: 61440, cioè 60 KB). Il predefinito è dimensionato per backend che limitano i valori degli attributi a 64 KB; aumentalo solo se il tuo backend accetta valori più grandi, o riducilo per tagliare il volume della telemetria. Quando un limite di attributo dell'SDK OpenTelemetry, `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` o uno dei suoi varianti logrecord e span, è impostato più basso, Claude Code tronca a quel valore più piccolo in modo che il marcatore `[TRUNCATED ...]` rimanga entro il limite dell'SDK. Richiede Claude Code v2.1.214 o successivo | `262144`                                                                                                                                                                          |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Preferenza di temporalità delle metriche (predefinito: `delta`). Imposta su `cumulative` se il tuo backend prevede temporalità cumulativa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `delta`, `cumulative`                                                                                                                                                             |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervallo per l'aggiornamento delle intestazioni dinamiche (predefinito: 1740000ms / 29 minuti)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `900000`                                                                                                                                                                          |

Per i protocolli `http/protobuf` e `http/json`, Claude Code invia ogni richiesta di esportazione con un'intestazione `Content-Length`. Prima della v2.1.212, le versioni di Claude Code dalla v2.1.191 in poi inviavano queste richieste con codifica di trasferimento chunked; Azure Monitor e altri endpoint che richiedono una lunghezza dichiarata le rifiutavano con errori `411 Length Required` o `400`.

<h3 id="mtls-authentication">
  Autenticazione mTLS
</h3>

Il modo in cui configuri i certificati client per l'esportatore OTLP dipende dal protocollo OTLP in uso per quel segnale, impostato tramite `OTEL_EXPORTER_OTLP_PROTOCOL` o l'override per segnale. La stessa configurazione si applica a metriche, log e tracce.

| Protocollo                   | Variabili di certificato client                                                                                                                                                                | Fidati del CA del collettore con |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY`, e facoltativamente `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Vedi [Configurazione di rete](/docs/it/network-config#mtls-authentication)             | `NODE_EXTRA_CA_CERTS`            |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` e `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, o le varianti per segnale come `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` per usare un certificato diverso per segnale | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

Per `grpc`, l'SDK OpenTelemetry legge direttamente le variabili OTLP standard, quindi le configurazioni esistenti che impostano le variabili di metriche per segnale continuano a funzionare. Su macchine con impostazioni gestite, Claude Code [può rimuovere credenziali e endpoint per segnale impostati dallo sviluppatore](#how-managed-settings-lock-the-otlp-destination) all'avvio.

<h3 id="metrics-cardinality-control">
  Controllo della cardinalità delle metriche
</h3>

Le seguenti variabili di ambiente controllano quali attributi sono inclusi nelle metriche per gestire la cardinalità:

| Variabile di ambiente                      | Descrizione                                                                                                                                                                               | Valore predefinito | Esempio per disabilitare |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------ | ------------------------ |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Includi l'attributo session.id nelle metriche                                                                                                                                             | `true`             | `false`                  |
| `OTEL_METRICS_INCLUDE_VERSION`             | Includi l'attributo app.version nelle metriche                                                                                                                                            | `false`            | `true`                   |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Includi gli attributi user.account\_uuid e user.account\_id nelle metriche                                                                                                                | `true`             | `false`                  |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Includi l'attributo app.entrypoint nelle metriche                                                                                                                                         | `false`            | `true`                   |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Includi le chiavi da `OTEL_RESOURCE_ATTRIBUTES` come attributi sui punti dati delle metriche                                                                                              | `true`             | `false`                  |
| `OTEL_METRICS_INCLUDE_REPOSITORY`          | Includi gli attributi di identità del repository `vcs.*` [repository identity attributes](#repository-attributes) sulle metriche e gli eventi. Richiede Claude Code v2.1.269 o successivo | `false`            | `true`                   |

Una cardinalità inferiore generalmente significa prestazioni migliori e costi di archiviazione inferiori ma dati meno granulari per l'analisi.

<h3 id="traces-beta">
  Tracce (beta)
</h3>

Le tracce distribuite esportano span che collegano ogni prompt dell'utente alle richieste API e alle esecuzioni degli strumenti che attiva, in modo da poter visualizzare una richiesta completa come una singola traccia nel tuo backend di traccia.

Le tracce sono disabilitate per impostazione predefinita. Per abilitarle, imposta sia `CLAUDE_CODE_ENABLE_TELEMETRY=1` che `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, quindi imposta `OTEL_TRACES_EXPORTER` per scegliere dove vengono inviati gli span. Le tracce riutilizzano la [configurazione OTLP comune](#common-configuration-variables) per endpoint, protocollo, intestazioni e [mTLS](#mtls-authentication). Su macchine con impostazioni gestite, Claude Code [può rimuovere credenziali e endpoint per segnale impostati dallo sviluppatore](#how-managed-settings-lock-the-otlp-destination) all'avvio.

| Variabile di ambiente                 | Descrizione                                                                                      | Valori di esempio                    |
| ------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Abilita la traccia degli span (obbligatorio). `ENABLE_ENHANCED_TELEMETRY_BETA` è anche accettato | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Tipi di esportatore di tracce, separati da virgola. Usa `none` per disabilitare                  | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protocollo per le tracce, sostituisce `OTEL_EXPORTER_OTLP_PROTOCOL`                              | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Endpoint OTLP per le tracce, sostituisce `OTEL_EXPORTER_OTLP_ENDPOINT`                           | `http://localhost:4318/v1/traces`    |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS`   | Intestazioni di autenticazione per le tracce, unite con `OTEL_EXPORTER_OTLP_HEADERS`             | `Authorization=Bearer token`         |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Intervallo di esportazione batch degli span in millisecondi (predefinito: 5000)                  | `1000`, `10000`                      |

Gli span oscurano il testo del prompt dell'utente, i dettagli di input dello strumento e il contenuto dello strumento per impostazione predefinita. Imposta `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1`, e `OTEL_LOG_TOOL_CONTENT=1` per includerli.

Quando la traccia è attiva, i sottoprocessi Bash e PowerShell ereditano automaticamente una variabile di ambiente `TRACEPARENT` contenente il contesto di traccia W3C dello span di esecuzione dello strumento attivo. Ciò consente a qualsiasi sottoprocesso che legge `TRACEPARENT` di far diventare i propri span figli della stessa traccia, abilitando la traccia distribuita end-to-end attraverso script e comandi che Claude esegue.

Quando la traccia è attiva e Claude Code è connesso direttamente all'API Anthropic, ogni richiesta del modello porta un'intestazione W3C `traceparent` impostata al contesto dello span `claude_code.llm_request`, e l'intestazione `traceresponse` dell'API viene registrata come collegamento di span. Insieme questi collegano gli span lato client di Claude Code alla traccia lato server attraverso qualsiasi intermediario conforme. Le richieste HTTP MCP in uscita portano `traceparent` allo stesso modo. L'intestazione non viene inviata ai provider di terze parti.

Per impostazione predefinita, l'intestazione `traceparent` sulle richieste di modello e HTTP MCP viene inviata solo quando `ANTHROPIC_BASE_URL` non è impostato o punta all'API Anthropic, poiché alcuni proxy rifiutano intestazioni non riconosciute. La variabile `TRACEPARENT` del sottoprocesso è controllata dallo stesso switch per coerenza. Se esegui Claude Code attraverso un proxy `ANTHROPIC_BASE_URL` personalizzato e desideri che il contesto di traccia sia propagato, imposta `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

In Agent SDK e sessioni non interattive avviate con `-p`, Claude Code legge anche `TRACEPARENT` e `TRACESTATE` dal proprio ambiente quando avvia ogni span di interazione. Ciò consente a un processo di incorporamento di passare il suo contesto di traccia W3C attivo nel sottoprocesso in modo che gli span di Claude Code appaiano come figli della traccia distribuita del chiamante. Le sessioni interattive ignorano `TRACEPARENT` in entrata per evitare di ereditare accidentalmente valori ambientali da ambienti CI o container.

Il contesto di traccia in entrata si applica anche agli [eventi](#events). In sessioni Agent SDK e `-p` con `TRACEPARENT` impostato, ogni record di log di evento OTLP porta valori `trace_id` e `span_id` che lo collegano alla traccia della tua applicazione, anche quando l'esportatore di tracce non è configurato, in modo che il tuo backend di logging possa correlare gli eventi con il resto della traccia.

Un record emesso mentre un'interazione è attiva porta gli ID dello span di interazione, anche quando Claude Code lo emette al di fuori del contesto asincrono dello span, come in un callback di prompt di autorizzazione o per un record memorizzato nel buffer durante l'avvio ed esportato in seguito. Un record emesso senza uno span di interazione attivo porta gli ID `TRACEPARENT` in entrata direttamente. Prima della v2.1.214, i record emessi al di fuori del contesto asincrono dello span portavano gli ID `TRACEPARENT` in entrata al posto degli ID dello span. Prima della v2.1.212, i record di evento emessi al di fuori di uno span attivo non portavano `trace_id` o `span_id`.

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

Quando un hook `PreToolUse` [rinvia una chiamata dello strumento](/docs/it/hooks#defer-a-tool-call-for-later), Claude Code salva il contesto di traccia del turno che lo ha rinviato. Quando riprendi la sessione e lo strumento viene rieseguito, gli span dello strumento si uniscono alla traccia di quel turno precedente come figli dello span `claude_code.interaction` del turno.

<h4 id="span-attributes">
  Attributi degli span
</h4>

Ogni span porta gli [attributi standard](#standard-attributes) più un attributo `span.type` che corrisponde al suo nome. Le tabelle seguenti elencano gli attributi aggiuntivi impostati su ogni span. Gli span `llm_request`, `tool.execution`, e `hook` impostano lo stato OpenTelemetry `ERROR` quando registrano un errore; gli altri span terminano sempre con stato `UNSET`.

**`claude_code.interaction`**

| Attributo                 | Descrizione                                                                                                                                                                                                       | Controllato da          |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Testo del prompt. Il valore è `<REDACTED>` a meno che il gate non sia impostato                                                                                                                                   | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Lunghezza del prompt in caratteri                                                                                                                                                                                 |                         |
| `interaction.sequence`    | Contatore basato su 1 delle interazioni, contato per processo Claude Code piuttosto che per sessione, come descritto per [`event.sequence`](#event-correlation-attributes)                                        |                         |
| `parent.source`           | Come lo span ha ottenuto il suo genitore di traccia: `env` quando è stato genitore sotto un `TRACEPARENT` in entrata, `none` quando ha avviato la sua propria traccia. Richiede Claude Code v2.1.268 o successivo |                         |
| `interaction.duration_ms` | Durata wall-clock del turno                                                                                                                                                                                       |                         |

**`claude_code.llm_request`**

| Attributo                        | Descrizione                                                                                                                                                                                                                                                                                                         | Controllato da                 |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `model`                          | Identificatore del modello                                                                                                                                                                                                                                                                                          |                                |
| `gen_ai.system`                  | Sempre `anthropic`. Convenzione semantica OpenTelemetry GenAI                                                                                                                                                                                                                                                       |                                |
| `gen_ai.request.model`           | Stesso valore di `model`. Convenzione semantica OpenTelemetry GenAI                                                                                                                                                                                                                                                 |                                |
| `query_source`                   | Sottosistema che ha emesso la richiesta, come `repl_main_thread` o un nome di subagent                                                                                                                                                                                                                              | `ENABLE_BETA_TRACING_DETAILED` |
| `query_source_safe`              | Forma limitata di `query_source`, emessa indipendentemente dal fatto che la traccia beta dettagliata sia attiva, con valori come `repl_main_thread` o `agent.builtin.general-purpose`. `:` diventa `.` e gli agenti denominati dall'utente appaiono come `agent.custom`. Richiede Claude Code v2.1.268 o successivo |                                |
| `agent_id`                       | Identificatore del subagent o del collega che ha emesso la richiesta. Assente nella sessione principale                                                                                                                                                                                                             |                                |
| `parent_agent_id`                | Identificatore dell'agente che ha generato questo. Assente per la sessione principale e per gli agenti generati direttamente da essa                                                                                                                                                                                |                                |
| `workflow.run_id`                | Identificatore di esecuzione della [Workflow](/docs/it/workflows) tool run che ha generato questo agente, con prefisso `wf_`. Assente per gli agenti non generati da un workflow                                                                                                                                         |                                |
| `workflow.name`                  | Nome del workflow che ha generato questo agente. I nomi creati dall'utente vengono sostituiti con `custom` a meno che il gate non sia impostato                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS`        |
| `speed`                          | `fast` o `normal`                                                                                                                                                                                                                                                                                                   |                                |
| `effort`                         | [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta: `low`, `medium`, `high`, `xhigh`, o `max`. Assente quando Claude Code non invia alcun livello di sforzo, ad esempio su un modello che non supporta lo sforzo. Richiede Claude Code v2.1.274 o successivo                        |                                |
| `llm_request.context`            | `interaction`, `tool`, o `standalone` a seconda dello span genitore                                                                                                                                                                                                                                                 |                                |
| `duration_ms`                    | Durata wall-clock inclusi i tentativi                                                                                                                                                                                                                                                                               |                                |
| `ttft_ms`                        | Tempo al primo token in millisecondi                                                                                                                                                                                                                                                                                |                                |
| `first_content_ms`               | Tempo dall'inizio della richiesta al primo blocco di contenuto del tentativo riuscito, in millisecondi. Assente sulle richieste che sono ricadute nel percorso non in streaming. Richiede Claude Code v2.1.268 o successivo                                                                                         |                                |
| `input_tokens`                   | Conteggio dei token di input dal blocco di utilizzo dell'API                                                                                                                                                                                                                                                        |                                |
| `output_tokens`                  | Conteggio dei token di output                                                                                                                                                                                                                                                                                       |                                |
| `cache_read_tokens`              | Token letti dalla cache del prompt                                                                                                                                                                                                                                                                                  |                                |
| `cache_creation_tokens`          | Token scritti nella cache del prompt                                                                                                                                                                                                                                                                                |                                |
| `request_id`                     | ID della richiesta API Anthropic dall'intestazione della risposta `request-id`                                                                                                                                                                                                                                      |                                |
| `gen_ai.response.id`             | Stesso valore di `request_id`. Convenzione semantica OpenTelemetry GenAI                                                                                                                                                                                                                                            |                                |
| `client_request_id`              | `x-client-request-id` generato dal client del tentativo finale                                                                                                                                                                                                                                                      |                                |
| `attempt`                        | Tentativi totali effettuati per questa richiesta                                                                                                                                                                                                                                                                    |                                |
| `success`                        | `true` o `false`                                                                                                                                                                                                                                                                                                    |                                |
| `status_code`                    | Codice di stato HTTP quando la richiesta non è riuscita                                                                                                                                                                                                                                                             |                                |
| `error`                          | Messaggio di errore quando la richiesta non è riuscita                                                                                                                                                                                                                                                              |                                |
| `error_class`                    | Token di classe di errore breve quando la richiesta non è riuscita, come `api_timeout` o `server_overload`. Richiede Claude Code v2.1.268 o successivo                                                                                                                                                              |                                |
| `response.has_tool_call`         | `true` quando la risposta conteneva blocchi di tool-use                                                                                                                                                                                                                                                             |                                |
| `stop_reason`                    | `stop_reason` della risposta API, come `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn`, o `refusal`                                                                                                                                                                                             |                                |
| `gen_ai.response.finish_reasons` | Stesso valore di `stop_reason`, racchiuso in un array di stringhe. Convenzione semantica OpenTelemetry GenAI                                                                                                                                                                                                        |                                |

Ogni tentativo di ripetizione viene anche registrato come evento di span `gen_ai.request.attempt` con attributi `attempt` e `client_request_id`.

**`claude_code.tool`**

| Attributo             | Descrizione                                                                                                                                                                                                                                                                                                                                                           | Controllato da          |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | Nome dello strumento                                                                                                                                                                                                                                                                                                                                                  |                         |
| `tool_name_safe`      | Forma di `tool_name` che non contiene nomi scelti dall'utente. I nomi degli strumenti integrati passano verbatim. I nomi degli strumenti MCP appaiono come `mcp_other`, tranne i nomi degli strumenti che corrispondono a poche forme fisse, come gli strumenti `playwright` denominati `browser_*`, che passano verbatim. Richiede Claude Code v2.1.268 o successivo |                         |
| `bash_command_class`  | Per lo strumento Bash: categoria del primo programma del comando da un elenco fisso, come `vcs` o `package_manager`. `other` per un programma al di fuori dell'elenco, `unparsed` quando la riga non può essere analizzata. Richiede Claude Code v2.1.268 o successivo                                                                                                |                         |
| `bash_argv0`          | Per lo strumento Bash: il primo programma del comando quando è nello stesso elenco fisso, come `git` o `npm`. `other` per qualsiasi programma al di fuori dell'elenco. Richiede Claude Code v2.1.268 o successivo                                                                                                                                                     |                         |
| `duration_ms`         | Durata wall-clock inclusa l'attesa di autorizzazione e l'esecuzione                                                                                                                                                                                                                                                                                                   |                         |
| `result_tokens`       | Dimensione approssimativa in token del risultato dello strumento                                                                                                                                                                                                                                                                                                      |                         |
| `agent_id`            | Identificatore del subagent o del collega che ha eseguito lo strumento. Assente nella sessione principale                                                                                                                                                                                                                                                             |                         |
| `parent_agent_id`     | Identificatore dell'agente che ha generato questo. Assente per la sessione principale e per gli agenti generati direttamente da essa                                                                                                                                                                                                                                  |                         |
| `workflow.run_id`     | Identificatore di esecuzione della Workflow tool run che ha generato questo agente, con prefisso `wf_`. Assente per gli agenti non generati da un workflow                                                                                                                                                                                                            |                         |
| `workflow.name`       | Nome del workflow che ha generato questo agente. I nomi creati dall'utente vengono sostituiti con `custom` a meno che il gate non sia impostato                                                                                                                                                                                                                       | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | L'ID del blocco `tool_use` del modello per questa chiamata. Corrisponde al `tool_use_id` sugli eventi [tool\_result](#tool-result-event) e [tool\_decision](#tool-decision-event) e nei payload degli hook, in modo da poter unire lo span a questi record                                                                                                            |                         |
| `gen_ai.tool.call.id` | Stesso valore di `tool_use_id`. Convenzione semantica OpenTelemetry GenAI                                                                                                                                                                                                                                                                                             |                         |
| `file_path`           | Percorso del file di destinazione per gli strumenti Read, Edit e Write                                                                                                                                                                                                                                                                                                | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Stringa di comando per lo strumento Bash                                                                                                                                                                                                                                                                                                                              | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nome della skill per lo strumento Skill                                                                                                                                                                                                                                                                                                                               | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Tipo di subagent per lo strumento Agent o lo strumento Task legacy                                                                                                                                                                                                                                                                                                    | `OTEL_LOG_TOOL_DETAILS` |

<span id="tool-output-span-event" />**`tool.output` span event on `claude_code.tool`**

Se imposti `OTEL_LOG_TOOL_CONTENT=1`, le chiamate Read e Bash possono registrare un evento di span `tool.output` sullo span `claude_code.tool`. Le chiamate Edit e Write registrano uno solo quando imposti anche `OTEL_LOG_TOOL_DETAILS=1`. Quella variabile non è limitata a questi due strumenti, quindi controlla la sua [riga nella tabella di configurazione](#common-configuration-variables) per gli argomenti che aggiunge altrove.

Claude Code scrive questo evento dal ritorno riuscito di una chiamata dello strumento, quindi una chiamata che genera un errore non registra nulla, qualunque sia lo strumento. Tra le chiamate che ritornano, non registra alcun evento `tool.output` per:

* Una chiamata a qualsiasi strumento diverso da Read, Edit, Write e Bash, inclusi gli strumenti MCP e WebFetch
* Un Read che ritorna qualcosa di diverso dal testo del file, come un'immagine, un PDF o una ri-lettura di un file il cui contenuto non è cambiato
* Una chiamata Edit o Write, a meno che non imposti anche `OTEL_LOG_TOOL_DETAILS=1`

L'evento porta questi attributi, ciascuno troncato al limite di contenuto (60 KB per impostazione predefinita). `Controllato da` nomina la variabile di cui un attributo ha bisogno oltre a `OTEL_LOG_TOOL_CONTENT=1`, e per Edit e Write quella variabile controlla l'evento stesso piuttosto che l'attributo.

| Attributo      | Descrizione                                                                                                             | Controllato da                                 |
| -------------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| `content`      | Testo che lo strumento Read ha restituito, o il testo che una chiamata Write è stata chiesta di scrivere                | `OTEL_LOG_TOOL_DETAILS` per lo strumento Write |
| `output`       | Output combinato di un comando Bash, con stderr intercalato in stdout                                                   |                                                |
| `diff`         | Patch strutturata che lo strumento Edit ha applicato                                                                    | `OTEL_LOG_TOOL_DETAILS`                        |
| `file_path`    | Percorso del file di destinazione per gli strumenti Read, Edit e Write, ripetendo l'attributo di span dello stesso nome | `OTEL_LOG_TOOL_DETAILS`                        |
| `bash_command` | Stringa di comando per lo strumento Bash                                                                                | `OTEL_LOG_TOOL_DETAILS`                        |

L'attributo `tool_name` dello span genitore ti dice da quale strumento proviene un evento. Un attributo tagliato al limite di contenuto è accompagnato da `<attribute>_truncated` e `<attribute>_original_length`.

**`claude_code.tool.blocked_on_user`**

| Attributo     | Descrizione                                                                                  | Controllato da |
| ------------- | -------------------------------------------------------------------------------------------- | -------------- |
| `duration_ms` | Tempo trascorso in attesa della decisione di autorizzazione                                  |                |
| `decision`    | `accept` o `reject`                                                                          |                |
| `source`      | Fonte della decisione, corrispondente all'evento [Tool decision event](#tool-decision-event) |                |

**`claude_code.tool.execution`**

| Attributo             | Descrizione                                                                                                                                                                                                                                                                                  | Controllato da          |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Tempo trascorso nell'esecuzione del corpo dello strumento                                                                                                                                                                                                                                    |                         |
| `tool_use_id`         | Stesso valore dello span genitore `claude_code.tool`                                                                                                                                                                                                                                         |                         |
| `gen_ai.tool.call.id` | Stesso valore di `tool_use_id`. Convenzione semantica OpenTelemetry GenAI                                                                                                                                                                                                                    |                         |
| `success`             | `true` o `false`                                                                                                                                                                                                                                                                             |                         |
| `error`               | Stringa di categoria di errore quando l'esecuzione non è riuscita, come `Error:ENOENT` o `ShellError`. Contiene il messaggio di errore completo quando il gate è impostato                                                                                                                   | `OTEL_LOG_TOOL_DETAILS` |
| `error_class`         | La categoria di errore in forma di identificatore, con caratteri al di fuori di lettere, cifre e sottolineature sostituiti da `_`, come `Error_ENOENT` o `ShellError`. Contiene la categoria anche quando `error` contiene il messaggio completo. Richiede Claude Code v2.1.268 o successivo |                         |

**`claude_code.hook`**

Questo span viene emesso solo quando la traccia beta dettagliata è attiva, il che richiede `ENABLE_BETA_TRACING_DETAILED=1` e `BETA_TRACING_ENDPOINT`, una coppia che inoltre [cambia dove vanno i tuoi log e tracce](/docs/it/env-vars#variables). Imposta la coppia nella tua shell, nelle impostazioni utente o nelle impostazioni gestite; entrambe le variabili vengono ignorate nelle [impostazioni di progetto e locali](/docs/it/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` da solo non lo produce.

Nelle sessioni CLI interattive, la traccia beta dettagliata richiede anche che la tua organizzazione sia nella lista di autorizzazione per la funzione. Le sessioni Agent SDK e non interattive `-p` non richiedono l'autorizzazione.

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

<span id="new-context-gates" />

<Note>
  Attributi aggiuntivi che portano contenuto come `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input`, e `response.model_output` vengono emessi solo quando la traccia beta dettagliata è attiva. Non fanno parte dello schema di span stabile.

  Il gate su `new_context` dipende da quale span lo porta, e ogni copia viene troncata al limite di contenuto (60 KB per impostazione predefinita). Sullo span `claude_code.tool` porta il risultato di quella chiamata dello strumento, qualunque sia lo strumento, e richiede `OTEL_LOG_TOOL_CONTENT=1`. Sullo span `claude_code.interaction` porta il prompt dell'utente, e sullo span `claude_code.llm_request` i nuovi messaggi dell'utente e i risultati degli strumenti di quella richiesta. Entrambi richiedono `OTEL_LOG_USER_PROMPTS=1`.

  `user_system_prompt` richiede inoltre `OTEL_LOG_USER_PROMPTS=1`. Contiene solo il testo del prompt di sistema che fornisci tramite l'opzione SDK `systemPrompt` o i flag `--system-prompt` e `--append-system-prompt`, troncato al limite di contenuto (60 KB per impostazione predefinita), ed è emesso una volta per sessione piuttosto che per richiesta.
</Note>

<h3 id="dynamic-headers">
  Intestazioni dinamiche
</h3>

Per gli ambienti aziendali che richiedono autenticazione dinamica, puoi configurare uno script per generare intestazioni dinamicamente. Le intestazioni dinamiche si applicano solo ai protocolli `http/protobuf` e `http/json`. Con il protocollo `grpc`, Claude Code utilizza solo le variabili di intestazioni statiche, `OTEL_EXPORTER_OTLP_HEADERS` e i suoi varianti per segnale.

<h4 id="settings-configuration">
  Configurazione delle impostazioni
</h4>

Aggiungi al tuo `.claude/settings.json`, sostituendo il percorso con il tuo script:

```json theme={null}
{
  "otelHeadersHelper": "/path/to/generate-otel-headers.sh"
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

Se l'helper non riesce o stampa output che non soddisfa questi requisiti, le esportazioni non riescono e il tuo backend di telemetria non riceve nulla dalla sessione fino a quando l'helper non funziona di nuovo. Claude Code segnala l'errore in:

* Una notifica di avviso nelle sessioni interattive, [`otelHeadersHelper failed; telemetry is not being exported`](/docs/it/errors#otelheadershelper-failed), mostrata una volta per sessione quando l'helper non riesce per la prima volta
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

Claude Code allega questi valori come attributi su ogni punto dati di metrica e record di evento, oltre a inviarli nel blocco di risorse OTLP. Poiché la maggior parte dei backend di metriche espone gli attributi dei punti dati come etichette interrogabili, puoi raggruppare e filtrare le metriche direttamente per le tue chiavi personalizzate. Eccetto per gli attributi del repository `vcs.*` [repository attributes](#repository-attributes), le chiavi personalizzate non sostituiscono mai gli [attributi standard](#standard-attributes) come `user.id` o `session.id`: quando una chiave collide, Claude Code mantiene il valore integrato.

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

Imposta queste variabili di ambiente prima di eseguire `claude`. Ogni scenario seguente mostra una configurazione completa, e ogni variabile è descritta sotto [Variabili di configurazione comuni](#common-configuration-variables). Per confermare che una configurazione ha avuto effetto, controlla il tuo backend per la metrica `claude_code.session.count` dopo aver avviato una sessione; la [Guida rapida](#quick-start) copre la verifica solo log e cosa controllare quando nulla arriva.

Per il debug della console con un intervallo di esportazione di 1 secondo:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000
```

Per OTLP su gRPC:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Per Prometheus, raschiato da `http://localhost:9464/metrics`:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus
```

Su un [ambiente self-hosted](/docs/it/self-hosted-environments-reference#pass-through-session-child-metrics), la sessione lega la porta 9464 solo alla capacità predefinita del runner di uno. A capacità superiore, il runner ri-espone i contatori e i gauge della sessione sul suo endpoint `/metrics` invece.

Per inviare metriche a più esportatori:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json
```

Per inviare metriche e log a endpoint o backend diversi:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317
```

Per esportare solo metriche, senza eventi o log:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Per esportare solo eventi e log, senza metriche:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  Metriche ed eventi disponibili
</h2>

<h3 id="standard-attributes">
  Attributi standard
</h3>

Tutte le metriche e gli eventi condividono questi attributi standard:

| Attributo                                                                               | Descrizione                                                                                                                                                                                                                                             | Controllato da                                                                                     |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `session.id`                                                                            | Identificatore univoco della sessione                                                                                                                                                                                                                   | `OTEL_METRICS_INCLUDE_SESSION_ID` (predefinito: true)                                              |
| `app.version`                                                                           | Versione corrente di Claude Code                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_VERSION` (predefinito: false)                                                |
| `app.entrypoint`                                                                        | Come è stata avviata la sessione, ad esempio `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, o `claude-vscode`                                                                                                                                                    | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (predefinito: false)                                             |
| `organization.id`                                                                       | UUID dell'organizzazione (quando autenticato)                                                                                                                                                                                                           | Sempre incluso quando disponibile                                                                  |
| `user.account_uuid`                                                                     | UUID dell'account (quando autenticato)                                                                                                                                                                                                                  | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (predefinito: true)                                            |
| `user.account_id`                                                                       | ID dell'account in formato etichettato corrispondente alle API di amministrazione Anthropic (quando autenticato), ad esempio `user_01BWBeN28...`                                                                                                        | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (predefinito: true)                                            |
| `user.id`                                                                               | Identificatore anonimo casuale generato al primo avvio e persistente in `~/.claude.json`. Non contiene informazioni personali e non è derivato dal tuo account Claude. L'eliminazione del file produce un nuovo valore non correlato al prossimo avvio. | Sempre incluso                                                                                     |
| `user.email`                                                                            | Indirizzo email dell'utente, dal tuo accesso o, in una [sessione cloud](/docs/it/claude-code-on-the-web), dalle credenziali della sessione stessa                                                                                                            | Sempre incluso quando disponibile                                                                  |
| `terminal.type`                                                                         | Tipo di terminale, ad esempio `iTerm.app`, `vscode`, `cursor`, o `tmux`                                                                                                                                                                                 | Sempre incluso quando rilevato                                                                     |
| Chiavi da `OTEL_RESOURCE_ATTRIBUTES`                                                    | Attributi personalizzati che imposti, ad esempio `department` o `team.id`. Vedi [Supporto per organizzazioni multi-team](#multi-team-organization-support)                                                                                              | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (predefinito: true)                                     |
| `vcs.repository.url.full`, `vcs.owner.name`, `vcs.repository.name`, `vcs.provider.name` | L'identità del repository della sessione, derivata dal suo remote `origin`. Vedi [Attributi del repository](#repository-attributes)                                                                                                                     | `OTEL_METRICS_INCLUDE_REPOSITORY` (predefinito: false). Richiede Claude Code v2.1.269 o successivo |

Quando Claude Code è autenticato in un [gateway di app Claude](/docs/it/claude-apps-gateway), la CLI contrassegna le esportazioni con l'identità autenticata dalla sessione del gateway: `user.id` è il soggetto IdP piuttosto che un identificatore di installazione anonimo, `user.email` è l'email con cui hai effettuato l'accesso, e `user.groups` contiene l'appartenenza al gruppo IdP come stringa separata da virgole. Ogni esportazione contiene anche `identity.source: gateway-oidc`. L'identità del gateway viene applicata per ultima, quindi le chiavi `user.*` e `identity.*` impostate tramite `OTEL_RESOURCE_ATTRIBUTES` vengono ignorate nelle sessioni del gateway.

Gli eventi includono inoltre i seguenti attributi. Questi non vengono mai allegati alle metriche perché causerebbero cardinalità illimitata:

* `prompt.id`: UUID che correla un prompt dell'utente con tutti gli eventi successivi fino al prompt successivo. Vedi [Attributi di correlazione degli eventi](#event-correlation-attributes).
* `workspace.host_paths`: directory dell'area di lavoro host selezionate nell'app desktop, come array di stringhe
* `workflow.run_id`: identificatore di esecuzione, con prefisso `wf_`, sugli eventi API e degli strumenti emessi da agenti che appartengono a un'esecuzione dello strumento [Workflow](/docs/it/workflows). Il filtraggio degli eventi per un `workflow.run_id` ricostruisce le richieste API e i risultati degli strumenti di quella esecuzione. L'identificatore copre gli agenti che lo script del workflow genera e tutti gli agenti che questi generano a loro volta, ad esempio le invocazioni di skill. Corrisponde all'identificatore di esecuzione riportato nel risultato dello strumento Workflow. Assente su tutti gli altri eventi. Richiede Claude Code v2.1.202 o successivo
* `workflow.name`: nome del workflow, il `meta.name` dello script, emesso insieme a `workflow.run_id`. I nomi dei workflow incorporati vengono visualizzati letteralmente quando l'esecuzione esegue lo script incorporato non modificato. I nomi creati dall'utente, incluse le copie modificate degli script incorporati, vengono sostituiti con `custom` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato. Richiede Claude Code v2.1.202 o successivo

<h4 id="repository-attributes">
  Attributi del repository
</h4>

Imposta `OTEL_METRICS_INCLUDE_REPOSITORY=true` per etichettare metriche ed eventi con l'identità del repository della sessione, in modo che un collector condiviso possa attribuire l'utilizzo per repository. Richiede Claude Code v2.1.269 o successivo.

Claude Code deriva questi attributi una volta per sessione dal remote `origin` del repository. I remote HTTPS e SSH di un repository producono valori identici:

| Attributo                 | Valore                                                                                                                                                   |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `vcs.repository.url.full` | L'URL del browser del repository senza `.git`, ad esempio `https://github.com/example-org/example-repo`                                                  |
| `vcs.owner.name`          | Il percorso del proprietario o del gruppo, ad esempio `example-org`; omesso quando il percorso remoto ha un singolo segmento                             |
| `vcs.repository.name`     | Il nome del repository nudo, ad esempio `example-repo`                                                                                                   |
| `vcs.provider.name`       | `github`, `gitlab`, `bitbucket`, o `gitea` quando Claude Code riconosce l'host remoto o la forma dell'URL come uno di questi provider; omesso altrimenti |

I valori sono in minuscolo e le credenziali, le stringhe di query e i frammenti dall'URL remoto non compaiono mai in essi. Gli attributi vengono omessi quando la sessione non ha un remote `origin`, quando il remote non ha forma di URL, o quando l'unico repository che lo racchiude è la tua directory home.

Una chiave `vcs.*` che dichiari in [`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support) sostituisce il valore derivato per quella chiave. Se dichiari `vcs.repository.url.full`, Claude Code non legge mai il remote e riporta solo le chiavi che dichiari.

Gli attributi fluiscono solo ai tuoi esportatori; la telemetria di Anthropic elimina ogni chiave `vcs.*`.

<h3 id="metrics">
  Metriche
</h3>

Claude Code esporta le seguenti metriche. La colonna Unit mostra la stringa di unità OpenTelemetry allegata a ogni metrica; le metriche di conteggio non ne hanno.

| Nome della metrica                    | Descrizione                                                                        | Unità   |
| ------------------------------------- | ---------------------------------------------------------------------------------- | ------- |
| `claude_code.session.count`           | Conteggio delle sessioni CLI avviate                                               | nessuna |
| `claude_code.lines_of_code.count`     | Conteggio delle righe di codice modificate                                         | nessuna |
| `claude_code.pull_request.count`      | Numero di pull request create                                                      | nessuna |
| `claude_code.commit.count`            | Numero di commit git creati                                                        | nessuna |
| `claude_code.cost.usage`              | Costo della sessione Claude Code                                                   | USD     |
| `claude_code.token.usage`             | Numero di token utilizzati                                                         | token   |
| `claude_code.code_edit_tool.decision` | Conteggio delle decisioni di autorizzazione dello strumento di modifica del codice | nessuna |
| `claude_code.active_time.total`       | Tempo attivo totale                                                                | s       |

Quando `prometheus` è l'unico esportatore elencato in `OTEL_METRICS_EXPORTER`, Claude Code omette le unità `USD`, `token`, e `s` dalle metriche esportate in modo che lo scrape rimanga in formato testo Prometheus valido. I nomi delle metriche non cambiano e le configurazioni che combinano esportatori, come `otlp,prometheus`, mantengono le unità. Prima della v2.1.216, lo scrape Prometheus includeva righe `# UNIT` solo OpenMetrics che alcuni scraper rifiutavano.

<h3 id="metric-details">
  Dettagli delle metriche
</h3>

Ogni metrica include gli attributi standard elencati sopra. Le metriche con attributi aggiuntivi specifici del contesto sono annotate di seguito.

<h4 id="session-counter">
  Contatore di sessione
</h4>

Incrementato all'inizio di ogni sessione.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `start_type`: Come è stata avviata la sessione. Uno di `"fresh"`, `"resume"`, `"continue"`, o `"agents_view"`. Il valore `"agents_view"` identifica il processo dashboard `claude agents`, un'interfaccia utente locale avviata dall'utente piuttosto che una sessione conversazionale. Filtra su questo valore per separare i lanci del processo UI dalle sessioni conversazionali nei tuoi dashboard.

<h4 id="lines-of-code-counter">
  Contatore di righe di codice
</h4>

Incrementato quando il codice viene aggiunto o rimosso.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: Identificatore del modello per il modello che ha apportato la modifica (ad esempio, "claude-sonnet-5")

<h4 id="pull-request-counter">
  Contatore di pull request
</h4>

Incrementato quando Claude Code crea una pull request o una merge request tramite un comando shell o uno strumento MCP.

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
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta: `"low"`, `"medium"`, `"high"`, `"xhigh"`, o `"max"`. Assente quando Claude Code non invia alcun livello di sforzo, ad esempio su un modello che non supporta lo sforzo.
* `agent.name`: Tipo di subagent che ha emesso la richiesta. I nomi degli agenti incorporati e gli agenti dai plugin del marketplace ufficiale vengono visualizzati letteralmente. Gli altri nomi di agenti definiti dall'utente vengono sostituiti con `"custom"` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato. Assente quando la richiesta non è stata emessa da un tipo di subagent denominato.
* `skill.name`: Skill attiva per la richiesta, impostata dallo strumento Skill, da un comando `/`, o ereditata da un subagent generato. I nomi delle skill incorporati, in bundle, definiti dall'utente e dai plugin del marketplace ufficiale vengono visualizzati letteralmente. I nomi delle skill dei plugin di terze parti vengono sostituiti con `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato. Assente quando nessuna skill è attiva.
* `plugin.name`: Plugin proprietario quando la skill attiva o il subagent è fornito da un plugin. I nomi dei plugin del marketplace ufficiale vengono visualizzati letteralmente. I nomi dei plugin di terze parti vengono sostituiti con `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato. Assente quando né la skill né il subagent hanno un plugin proprietario.
* `marketplace.name`: Marketplace da cui è stato installato il plugin proprietario. Emesso solo per i plugin del marketplace ufficiale. Assente altrimenti.
* `mcp_server.name`: Server MCP il cui risultato dello strumento questa richiesta ha consumato. I nomi dei server incorporati, proxy di claude.ai e del registro ufficiale vengono visualizzati letteralmente. I nomi dei server configurati dall'utente vengono sostituiti con `"custom"` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato. Assente quando la richiesta non ha consumato alcun risultato dello strumento MCP. Prima della v2.1.222, Claude Code impostava questo attributo su ogni richiesta dopo una chiamata dello strumento MCP, non solo su richieste che consumavano un risultato dello strumento, quindi i dashboard che lo aggregano mostrano un calo dopo l'aggiornamento.
* `mcp_tool.name`: Strumento MCP il cui risultato questa richiesta ha consumato, con lo stesso comportamento di redazione e versione di `mcp_server.name`. Assente quando la richiesta non ha consumato alcun risultato dello strumento MCP.

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
  Contatore di decisioni dello strumento di modifica del codice
</h4>

Incrementato quando l'utente accetta o rifiuta l'utilizzo dello strumento Edit, Write, o NotebookEdit.

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `tool_name`: Nome dello strumento (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Decisione dell'utente (`"accept"`, `"reject"`)
* `source`: Da dove proviene la decisione. Uno di `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, o `"user_reject"`. Vedi l'[evento di decisione dello strumento](#tool-decision-event) per il significato di ogni valore.
* `language`: Linguaggio di programmazione del file modificato, ad esempio `"TypeScript"`, `"Python"`, `"JavaScript"`, o `"Markdown"`. Restituisce `"unknown"` per estensioni di file non riconosciute.

<h4 id="active-time-counter">
  Contatore di tempo attivo
</h4>

Traccia il tempo effettivo trascorso utilizzando attivamente Claude Code, escludendo il tempo di inattività. Questa metrica viene incrementata durante le interazioni dell'utente, come la digitazione e la lettura delle risposte, e durante l'elaborazione della CLI, come l'esecuzione degli strumenti e la generazione della risposta AI.

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

Quando un utente invia un prompt, Claude Code può effettuare più chiamate API ed eseguire diversi strumenti. L'attributo `prompt.id` ti consente di collegare tutti questi eventi al singolo prompt che li ha attivati.

| Attributo           | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt.id`         | Identificatore UUID v4 che collega tutti gli eventi prodotti durante l'elaborazione di un singolo prompt dell'utente                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `event.sequence`    | Contatore a base 0 per ordinare gli eventi, conteggiato per processo Claude Code piuttosto che per sessione                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `message.uuid`      | UUID del messaggio come persistente nella trascrizione della sessione, i file `~/.claude/projects/*/*.jsonl`. Presente su `assistant_response`, su `api_response_body`, e su `user_prompt` eccetto per i dispatching dei comandi, che possono produrre zero o molti messaggi. Su `assistant_response` e `api_response_body`, questo è il messaggio finale della trascrizione della risposta, da cui il `parentUuid` del turno successivo si collega. Richiede Claude Code v2.1.214 o successivo, o v2.1.274 o successivo su `api_response_body`                     |
| `client_request_id` | UUID generato dal client inviato come intestazione della richiesta `x-client-request-id`. Presente su `api_request` e `api_error` su connessioni API di prima parte; assente su backend di provider di terze parti e quando la richiesta è stata ritentata tramite il fallback non in streaming. Accoppia una richiesta con la sua risposta e rimane disponibile per i guasti come i timeout che non hanno mai prodotto un `request_id` del server. Corrisponde allo stesso attributo sul span di traccia `llm_request`. Richiede Claude Code v2.1.214 o successivo |

Per tracciare tutta l'attività attivata da un singolo prompt, filtra i tuoi eventi per un valore specifico di `prompt.id`. Questo restituisce l'evento user\_prompt, tutti gli eventi api\_request, e tutti gli eventi tool\_result che si sono verificati durante l'elaborazione di quel prompt.

`event.sequence` inizia a 0 ogni volta che un processo Claude Code si avvia e conta fino alla fine della vita di quel processo. Continua a contare attraverso `/clear`, che assegna un nuovo `session.id`. Se [riprendi una sessione senza fare un fork](/docs/it/how-claude-code-works#resume-or-fork-sessions), la sessione mantiene il suo `session.id` ma prende i suoi valori `event.sequence` dal processo che l'ha ripresa, quindi all'interno di una sessione un evento successivo può portare un valore inferiore a uno precedente, o ripetere uno. Per ordinare gli eventi di una sessione, ordina per `event.timestamp` e usa `event.sequence` per ordinare gli eventi che condividono un timestamp.

Per la ricostruzione a livello di messaggio, ogni classe di evento porta una chiave che corrisponde a un campo nella trascrizione della sessione. Il formato della voce di trascrizione è [interno a Claude Code](/docs/it/sessions#where-transcripts-are-stored) e cambia tra le versioni, quindi una pipeline che si unisce su questi campi può rompersi in qualsiasi rilascio; tratta gli join come specifici della versione piuttosto che come un contratto stabile:

* `message.uuid` su `user_prompt`, `assistant_response`, e `api_response_body`
* `request_id` sugli eventi API, persistito come `requestId` sulle voci dell'assistente della trascrizione
* `tool_use_id` su `tool_result` e `tool_decision` eventi

<h4 id="user-prompt-event">
  Evento di prompt dell'utente
</h4>

Registrato quando un utente invia un prompt.

**Nome evento**: `claude_code.user_prompt`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `prompt_length`: Lunghezza del prompt
* `prompt`: Contenuto del prompt. Redatto per impostazione predefinita. Imposta `OTEL_LOG_USER_PROMPTS=1` per includerlo
* `message.uuid`: UUID del messaggio utente risultante, corrispondente alla voce di trascrizione persistente. Assente nei dispatching dei comandi, che possono produrre zero o molti messaggi. Richiede Claude Code v2.1.214 o successivo
* `command_name`: Nome del comando quando il prompt ne invoca uno. I nomi dei comandi incorporati e in bundle come `compact` o `debug` vengono emessi così come sono; gli alias come `reset` vengono emessi come digitati piuttosto che come il nome canonico. I nomi dei comandi personalizzati, dei plugin e MCP si riducono a `custom` o `mcp` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato
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
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `response_length`: Lunghezza del testo della risposta in caratteri
* `response`: Testo della risposta, troncato al limite di contenuto (60 KB per impostazione predefinita). Redatto a `<REDACTED>` per impostazione predefinita. Imposta `OTEL_LOG_ASSISTANT_RESPONSES=1` per includerlo. Quando `OTEL_LOG_ASSISTANT_RESPONSES` non è impostato, `OTEL_LOG_USER_PROMPTS` lo controlla invece, quindi imposta `OTEL_LOG_ASSISTANT_RESPONSES=0` per mantenere le risposte redatte mentre la registrazione dei prompt è attiva
* `model`: Identificatore del modello (ad esempio, "claude-sonnet-5")
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta. Presente solo quando l'API ne restituisce uno
* `message.uuid`: UUID della voce di trascrizione finale della risposta. Una risposta API viene persistita come una voce di trascrizione per blocco di contenuto; questa è l'ultima, da cui il `parentUuid` del turno successivo si collega. Richiede Claude Code v2.1.214 o successivo
* `query_source`: Sottosistema che ha emesso la richiesta, ad esempio `"repl_main_thread"`, `"compact"`, o un nome di subagent

<h4 id="tool-result-event">
  Evento di risultato dello strumento
</h4>

Registrato quando uno strumento completa l'esecuzione. Non emesso se la chiamata dello strumento è stata rifiutata; vedi l'[evento di decisione dello strumento](#tool-decision-event) per i rifiuti.

**Nome evento**: `claude_code.tool_result`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `tool_name`: Nome dello strumento
* `tool_use_id`: Identificatore univoco per questa invocazione dello strumento. Corrisponde al `tool_use_id` passato agli hook, consentendo la correlazione tra gli eventi OTel e i dati acquisiti dagli hook.
* `success`: `"true"` o `"false"`
* `duration_ms`: Tempo di esecuzione in millisecondi
* `error_type`: Stringa di categoria di errore quando lo strumento ha fallito, ad esempio `"Error:ENOENT"` o `"ShellError"`
* `error` (quando `OTEL_LOG_TOOL_DETAILS=1`): Messaggio di errore completo quando lo strumento ha fallito
* `decision_type`: Sempre `"accept"`, poiché questo evento viene emesso solo dopo l'esecuzione dello strumento. Le chiamate rifiutate non producono un risultato dello strumento
* `decision_source`: Da dove proviene la decisione di autorizzazione. Uno di `"config"`, `"hook"`, `"user_permanent"`, o `"user_temporary"`. Vedi l'[evento di decisione dello strumento](#tool-decision-event) per il significato di ogni valore. Le fonti solo per il rifiuto `"user_abort"` e `"user_reject"` non compaiono mai su questo evento.
* `tool_input_size_bytes`: Dimensione dell'input dello strumento serializzato in JSON in byte
* `tool_result_size_bytes`: Dimensione del risultato dello strumento in byte
* `mcp_server_scope`: Identificatore dell'ambito del server MCP (per gli strumenti MCP)
* `vcs.ref.head.revision`, `vcs.ref.head.name`, `vcs.ref.head.type` (quando `OTEL_LOG_TOOL_DETAILS=1`): l'identità del commit di un'esecuzione riuscita di `git commit` eseguita dallo strumento Bash o PowerShell. `vcs.ref.head.revision` è lo SHA del commit, `vcs.ref.head.name` è il ramo su cui è stato eseguito il commit, e `vcs.ref.head.type` è `branch`. Il nome e il tipo vengono omessi quando il commit è stato effettuato su un HEAD staccato. Richiede Claude Code v2.1.269 o successivo
* `tool_parameters` (quando `OTEL_LOG_TOOL_DETAILS=1`): Stringa JSON contenente parametri specifici dello strumento. Per i server incorporati di Claude Desktop, nelle sessioni che Claude Desktop possiede, la coppia `mcp_server_name`/`mcp_tool_name` è inclusa anche con il flag disattivato, la stessa eccezione creata dall'host dell'[evento di decisione dello strumento](#tool-decision-event), richiedendo Claude Code v2.1.214 o successivo. I parametri variano in base allo strumento:
  * Per lo strumento Bash: include `bash_command`, `full_command`, `timeout`, `description`, e `dangerouslyDisableSandbox`, più `git_commit_id` e `git_branch` quando un comando `git commit` ha successo. `git_commit_id` è lo SHA del commit completo quando il commit è l'HEAD della directory di lavoro della sessione, e lo SHA abbreviato di git altrimenti. `git_branch` è il ramo su cui è stato eseguito il commit, omesso su un HEAD staccato
  * Per lo strumento Bash dell'area di lavoro dell'app desktop, che riporta anche `tool_name` come `Bash`: include solo `bash_command`, `full_command`, e `timeout`
  * Per gli strumenti MCP: include `mcp_server_name`, `mcp_tool_name`
  * Per lo strumento Skill: include `skill_name`
  * Per lo strumento Agent o lo strumento Task legacy: include `subagent_type`
* `tool_input` (quando `OTEL_LOG_TOOL_DETAILS=1`): Argomenti dello strumento serializzati in JSON. I singoli valori superiori a 512 caratteri vengono troncati e il payload completo è limitato a circa 4 K caratteri. Si applica a tutti gli strumenti inclusi gli strumenti MCP.

<h4 id="api-request-event">
  Evento di richiesta API
</h4>

Registrato per ogni richiesta API a Claude.

**Nome evento**: `claude_code.api_request`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `model`: Modello utilizzato (ad esempio, "claude-sonnet-5")
* `cost_usd`: Costo stimato in USD
* `cost_usd_micros`: Costo stimato in milionesimi di dollaro USA, emesso come numero intero
* `duration_ms`: Durata della richiesta in millisecondi
* `input_tokens`: Numero di token di input
* `output_tokens`: Numero di token di output
* `cache_read_tokens`: Numero di token letti dalla cache
* `cache_creation_tokens`: Numero di token utilizzati per la creazione della cache
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta, ad esempio `"req_011..."`. Presente solo quando l'API ne restituisce uno.
* `client_request_id`: UUID generato dal client inviato come intestazione della richiesta `x-client-request-id`; vedi la tabella [attributi di correlazione degli eventi](#event-correlation-attributes) per quando è presente. Richiede Claude Code v2.1.214 o successivo
* `speed`: `"fast"` o `"normal"`, indicando se la modalità veloce era attiva
* `query_source`: Sottosistema che ha emesso la richiesta, ad esempio `"repl_main_thread"`, `"compact"`, o un nome di subagent
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta: `"low"`, `"medium"`, `"high"`, `"xhigh"`, o `"max"`. Assente quando Claude Code non invia alcun livello di sforzo, ad esempio su un modello che non supporta lo sforzo.
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
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `model`: Modello utilizzato (ad esempio, "claude-sonnet-5")
* `error`: Messaggio di errore
* `status_code`: Codice di stato HTTP come numero. Assente per errori non HTTP come i guasti di connessione.
* `duration_ms`: Durata della richiesta in millisecondi
* `attempt`: Numero totale di tentativi effettuati, inclusa la richiesta iniziale (`1` significa che non si sono verificati tentativi)
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta, ad esempio `"req_011..."`. Presente solo quando l'API ne restituisce uno.
* `client_request_id`: UUID generato dal client inviato come intestazione della richiesta `x-client-request-id`. Disponibile anche quando un guasto come un timeout o un errore di connessione non ha mai prodotto un `request_id` del server; vedi la tabella [attributi di correlazione degli eventi](#event-correlation-attributes) per quando è presente. Richiede Claude Code v2.1.214 o successivo
* `speed`: `"fast"` o `"normal"`, indicando se la modalità veloce era attiva
* `query_source`: Sottosistema che ha emesso la richiesta, ad esempio `"repl_main_thread"`, `"compact"`, o un nome di subagent
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta. Assente quando Claude Code non invia alcun livello di sforzo, ad esempio su un modello che non supporta lo sforzo.
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
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `model`: Identificatore del modello dalla richiesta
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta, ad esempio `"req_011..."`. Presente solo quando l'API ne restituisce uno.
* `query_source`: Sottosistema che ha emesso la richiesta, ad esempio `"repl_main_thread"`, `"compact"`, o un nome di subagent. Vedi [`api_request`](#api-request-event) per le definizioni.
* `speed`: `"fast"` quando la [modalità veloce](/docs/it/fast-mode) è attiva, o `"normal"`
* `attempt`: Numero del tentativo di ripetizione. Il primo tentativo è `1`.
* `effort`: [Livello di sforzo](/docs/it/model-config#adjust-effort-level) applicato alla richiesta. Assente quando Claude Code non invia alcun livello di sforzo, ad esempio su un modello che non supporta lo sforzo.
* `server_fallback_hop`: `true` quando il fallback del modello lato server dell'API ha già ritentato questo rifiuto su un modello diverso, quindi l'utente non ha visto questo particolare rifiuto. `false` quando la richiesta è terminata in un rifiuto. Un singolo turno può emettere sia un evento hop `true` che un evento finale `false` successivo quando il modello di fallback rifiuta anche.
* `has_category`: `true` quando la risposta API conteneva un `stop_details.category` di `"cyber"`, `"bio"`, `"frontier_llm"`, o `"reasoning_extraction"`. `false` quando la risposta non conteneva alcuna categoria o un valore al di fuori di quel set. Assente quando `server_fallback_hop` è `true`, perché i blocchi hop non portano `stop_details`.
* `has_explanation`: `true` quando la risposta API conteneva un `stop_details.explanation`, altrimenti `false`. Assente quando `server_fallback_hop` è `true`.
* `category`: Il valore `stop_details.category` dalla risposta API. Uno di `"cyber"`, `"bio"`, `"frontier_llm"`, o `"reasoning_extraction"`. Presente solo quando `OTEL_LOG_TOOL_DETAILS=1` è impostato e `has_category` è `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Attribuzione di skill, plugin, agente e MCP per la richiesta. Vedi [Contatore di costo](#cost-counter) per le definizioni e il comportamento di redazione.

<h4 id="api-request-body-event">
  Evento del corpo della richiesta API
</h4>

Registrato per ogni tentativo di richiesta API quando `OTEL_LOG_RAW_API_BODIES` è impostato. Un evento viene emesso per tentativo, quindi i tentativi con parametri regolati producono ciascuno il proprio evento.

**Nome evento**: `claude_code.api_request_body`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `body`: Parametri della richiesta API Messages serializzati in JSON, come il prompt di sistema, i messaggi e gli strumenti, troncati al limite di contenuto (60 KB per impostazione predefinita). Il contenuto del pensiero esteso nei turni dell'assistente precedenti viene redatto. Emesso solo in modalità inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Percorso assoluto a un file `<dir>/<uuid>.request.json` contenente il corpo non troncato. Emesso solo in modalità file (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Lunghezza del corpo non troncato. Byte UTF-8 quando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, o unità di codice UTF-16 quando `=1`
* `body_truncated`: `"true"` quando si è verificato il troncamento inline. Assente in modalità file e quando non si è verificato alcun troncamento.
* `model`: Identificatore del modello dai parametri della richiesta
* `query_source`: Sottosistema che ha emesso la richiesta (ad esempio, `"compact"`)
* `request_body_id`: UUID che identifica il corpo della richiesta di questo tentativo. L'[evento `api_response_body`](#api-response-body-event) per il tentativo che ha successo porta lo stesso valore, quindi puoi accoppiare una risposta con la richiesta esatta che l'ha prodotta. Richiede Claude Code v2.1.274 o successivo

<h4 id="api-response-body-event">
  Evento del corpo della risposta API
</h4>

Registrato per ogni risposta API riuscita quando `OTEL_LOG_RAW_API_BODIES` è impostato.

In modalità file (`OTEL_LOG_RAW_API_BODIES=file:<dir>`), Claude Code aggiunge anche una riga JSON a `<dir>/index.jsonl` per ogni risposta riuscita, con i campi `timestamp`, `session_id`, `query_source`, `model`, `request_id`, `message_id`, `message_uuid`, `request_file`, e `response_file`. Leggilo per trovare i file di richiesta e risposta dietro un determinato messaggio di trascrizione senza interrogare il tuo backend di telemetria. Il file di indice richiede Claude Code v2.1.274 o successivo.

**Nome evento**: `claude_code.api_response_body`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `body`: Risposta API Messages serializzata in JSON, inclusi l'id, i blocchi di contenuto, l'utilizzo e il motivo dell'arresto, troncata al limite di contenuto (60 KB per impostazione predefinita). Il contenuto del pensiero esteso viene redatto. Emesso solo in modalità inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Percorso assoluto a un file `<dir>/<request_id>.response.json` contenente il corpo non troncato. Emesso solo in modalità file (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Lunghezza del corpo non troncato. Byte UTF-8 quando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, o unità di codice UTF-16 quando `=1`
* `body_truncated`: `"true"` quando si è verificato il troncamento inline. Assente in modalità file e quando non si è verificato alcun troncamento.
* `model`: Identificatore del modello
* `query_source`: Sottosistema che ha emesso la richiesta
* `request_id`: ID della richiesta API Anthropic dall'intestazione `request-id` della risposta, ad esempio `"req_011..."`. Presente solo quando l'API ne restituisce uno.
* `request_body_id`: Il `request_body_id` dell'[evento `api_request_body`](#api-request-body-event) a cui questa risposta risponde. Richiede Claude Code v2.1.274 o successivo
* `message.id`: ID del messaggio che l'API ha assegnato alla risposta, il campo `id` del corpo della risposta. Richiede Claude Code v2.1.274 o successivo
* `message.uuid`: UUID della voce di trascrizione finale della risposta. Insieme a `request_body_id`, collega un messaggio di trascrizione ai corpi di richiesta e risposta dietro di esso. Richiede Claude Code v2.1.274 o successivo

<h4 id="tool-decision-event">
  Evento di decisione dello strumento
</h4>

Registrato quando viene presa una decisione di autorizzazione dello strumento (accetta/rifiuta).

**Nome evento**: `claude_code.tool_decision`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `tool_name`: Nome dello strumento (ad esempio, "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: Identificatore univoco per questa invocazione dello strumento. Corrisponde al `tool_use_id` passato agli hook, consentendo la correlazione tra gli eventi OTel e i dati acquisiti dagli hook.
* `decision`: `"accept"` o `"reject"`
* `tool_source`: Sempre presente. La provenienza dello strumento, come un insieme chiuso di valori creati dalla CLI. Richiede Claude Code v2.1.214 o successivo
  * `"builtin"`: gli strumenti della CLI stessa
  * `"mcp"`: server MCP in generale
  * `"sdk_host_builtin_mcp"`: un server in-process incorporato in Claude Desktop stesso, in una sessione che Claude Desktop possiede. Claude Desktop possiede una sessione che ha avviato da uno dei suoi stessi punti di ingresso, `claude-desktop`, `claude-desktop-3p`, o `local-agent`, quando quella sessione non è un figlio annidato; le sessioni annidate, incluse le sessioni che Claude Code stesso genera, segnalano questi server come `"mcp"`
* `source`: Da dove proviene la decisione:
  * `"config"`: Deciso automaticamente senza chiedere, in base alle impostazioni del progetto, alle regole di consentimento o negazione nelle impostazioni personali dell'utente, alla politica gestita dall'azienda, ai flag `--allowedTools` o `--disallowedTools`, alla modalità di autorizzazione attiva, a una concessione con ambito di sessione da un prompt precedente nella stessa sessione CLI interattiva, o perché lo strumento è intrinsecamente sicuro. L'evento non indica quale di queste fonti corrisponde. Claude Code riporta anche `"config"` quando la richiesta del prompt di autorizzazione stessa non riesce, ad esempio quando il callback [`canUseTool`](/docs/it/agent-sdk/typescript#canusetool) dell'Agent SDK o lo strumento [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags) restituisce un risultato non valido, o quando il flusso di input si chiude mentre la richiesta è in sospeso. Prima della v2.1.216, Claude Code segnalava questi guasti come `"user_reject"`.
  * `"hook"`: Un hook `PreToolUse` o `PermissionRequest` ha restituito la decisione.
  * `"user_permanent"`: Emesso quando l'utente ha scelto "Sì, e non chiedere di nuovo per ..." a un prompt di autorizzazione, che salva una regola di consentimento nelle impostazioni personali. Nella CLI interattiva questo viene emesso solo per quella scelta stessa; le chiamate successive che corrispondono alla regola salvata emettono `"config"` invece. Nelle sessioni Agent SDK o non interattive `-p`, sia la scelta iniziale che le corrispondenze successive della regola emettono `"user_permanent"`. Trattato come un'accettazione.
  * `"user_temporary"`: Emesso quando l'utente ha scelto "Sì" a un prompt di autorizzazione per un'approvazione una tantum, o ha scelto un'opzione che concede l'accesso per il resto della sessione su un prompt di modifica o lettura di file. Nella CLI interattiva questo viene emesso solo per la scelta stessa; le chiamate successive consentite da quella concessione con ambito di sessione emettono `"config"` invece. Nelle sessioni Agent SDK o non interattive `-p`, sia la scelta che le corrispondenze successive emettono `"user_temporary"`. Trattato come un'accettazione.
  * `"user_abort"`: Emesso quando l'utente ha chiuso il prompt di autorizzazione senza rispondere. Nelle sessioni Agent SDK e non interattive `-p`, questo include l'interruzione del turno mentre una richiesta di autorizzazione `canUseTool` o `--permission-prompt-tool` è in sospeso; prima della v2.1.216, Claude Code segnalava quell'interruzione come `"user_reject"`. Trattato come un rifiuto.
  * `"user_reject"`: Emesso quando l'utente ha scelto "No" quando gli è stato chiesto. Nella CLI interattiva questo viene emesso solo per quella scelta stessa; le chiamate che corrispondono a una regola di negazione nelle impostazioni personali dell'utente emettono `"config"` invece. Nelle sessioni Agent SDK o non interattive `-p`, le chiamate che corrispondono a una regola di negazione nelle impostazioni personali emettono `"user_reject"`. Trattato come un rifiuto.
* `tool_parameters` (quando `OTEL_LOG_TOOL_DETAILS=1`): Stringa JSON contenente parametri specifici dello strumento. Stessa forma dell'[evento di risultato dello strumento](#tool-result-event), meno i campi post-esecuzione come `git_commit_id`. I valori possono differire da `tool_result` per una chiamata accettata se la decisione di autorizzazione riscrive l'input dello strumento tramite `updatedInput`. Usa questo attributo per vedere quale comando è stato rifiutato quando `decision` è `"reject"`.
  * Per gli strumenti `"sdk_host_builtin_mcp"`: `mcp_server_name` e `mcp_tool_name` sono inclusi anche quando `OTEL_LOG_TOOL_DETAILS` è disattivato, perché l'applicazione host definisce questi nomi; senza di loro, una chiamata rifiutata a uno di questi server incorporati sarebbe non attribuibile nel flusso predefinito. Per i server MCP configurati dall'utente, il `tool_name` dell'evento è sempre il letterale `"mcp_tool"`, e i nomi del server e dello strumento compaiono solo in `tool_parameters` con il flag attivato; il contenuto dell'argomento richiede il flag ovunque. Richiede Claude Code v2.1.214 o successivo
  * Per lo strumento Bash: include `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`. Lo strumento bash dell'area di lavoro dell'app desktop riporta anche `tool_name` come `Bash`, ma include solo `bash_command`, `full_command`, e `timeout`
  * Per gli strumenti MCP: include `mcp_server_name`, `mcp_tool_name`
  * Per lo strumento Skill: include `skill_name`
  * Per lo strumento Agent o lo strumento Task legacy: include `subagent_type`

<h4 id="permission-mode-changed-event">
  Evento di cambio della modalità di autorizzazione
</h4>

Registrato quando la modalità di autorizzazione cambia, ad esempio dal ciclo Shift+Tab, dall'uscita dalla modalità piano, o da un controllo del gate della modalità automatica.

**Nome evento**: `claude_code.permission_mode_changed`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
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
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `action`: `"login"` o `"logout"`
* `success`: `"true"` o `"false"`
* `auth_method`: Metodo di autenticazione, ad esempio `"oauth"`
* `error_category`: Tipo di errore categorico quando l'azione non è riuscita. Il messaggio di errore grezzo non viene mai incluso
* `status_code`: Codice di stato HTTP come stringa quando l'azione non è riuscita con un errore HTTP

<h4 id="mcp-server-connection-event">
  Evento di connessione del server MCP
</h4>

Registrato quando un server MCP si connette, si disconnette, o non riesce a connettersi.

**Nome evento**: `claude_code.mcp_server_connection`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `status`: `"connected"`, `"failed"`, o `"disconnected"`
* `transport_type`: Trasporto del server, ad esempio `"stdio"`, `"sse"`, o `"http"`
* `server_scope`: Ambito in cui il server è configurato, ad esempio `"user"`, `"project"`, o `"local"`
* `duration_ms`: Durata del tentativo di connessione in millisecondi
* `error_code`: Codice di errore quando la connessione non è riuscita
* `is_plugin`: `true` quando il server è fornito da un plugin, `false` altrimenti
* `plugin_id_hash` (quando `is_plugin` è `true`): Hash stabile del nome del plugin e del marketplace, per raggruppare gli eventi per plugin senza esporre il nome. Claude Code lo calcola come descritto sotto l'[evento di plugin caricato](#plugin-loaded-event)
* `plugin.name` (quando `is_plugin` è `true`): Nome del plugin che fornisce il server. Per i plugin di terze parti questo è il letterale `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1`; questo protegge i nomi dei plugin di terze parti dall'apparire nei log per impostazione predefinita. I plugin da fonti Anthropic ufficiali sono sempre identificati per nome. Gli attributi `plugin_id_hash` e `plugin.name` fluiscono al tuo backend di monitoraggio e non vengono inviati ad Anthropic
* `server_name` (quando `OTEL_LOG_TOOL_DETAILS=1`): Nome del server configurato
* `error` (quando `OTEL_LOG_TOOL_DETAILS=1`): Messaggio di errore completo quando la connessione non è riuscita

<h4 id="internal-error-event">
  Evento di errore interno
</h4>

Registrato quando Claude Code cattura un errore interno inaspettato. Solo il nome della classe di errore e un codice di stile errno vengono registrati. Il messaggio di errore e la traccia dello stack non vengono mai inclusi. Questo evento non viene emesso quando si esegue su Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry, o quando `DISABLE_ERROR_REPORTING` è impostato.

**Nome evento**: `claude_code.internal_error`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `error_name`: Nome della classe di errore, ad esempio `"TypeError"` o `"SyntaxError"`
* `error_code`: Codice errno di Node.js come `"ENOENT"` quando presente sull'errore

<h4 id="plugin-installed-event">
  Evento di plugin installato
</h4>

Registrato quando un plugin finisce di installare, sia dal comando CLI `claude plugin install` che dall'interfaccia utente interattiva `/plugin`.

**Nome evento**: `claude_code.plugin_installed`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `marketplace.is_official`: `"true"` se il marketplace è un marketplace Anthropic ufficiale, `"false"` altrimenti
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
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `plugin.name`: nome del plugin. Per i plugin al di fuori del marketplace ufficiale e del bundle incorporato il valore è `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: marketplace da cui è stato installato il plugin, quando noto. Redatto a `"third-party"` sotto la stessa condizione di `plugin.name`
* `plugin.version`: versione dal manifesto del plugin. Incluso solo quando il nome non è redatto e il manifesto dichiara una versione
* `plugin.scope`: categoria di provenienza per il plugin: `"official"`, `"community"`, `"org"`, `"user-local"`, o `"default-bundle"`
* `enabled_via`: come il plugin è venuto ad essere abilitato: `"default-enable"`, `"org-policy"`, `"admin-install"`, `"seed-mount"`, o `"user-install"`. Il valore `"admin-install"` significa che il plugin è impostato come obbligatorio o auto-installazione per la tua organizzazione in [**Impostazioni organizzazione > Plugin e skill**](https://claude.ai/admin-settings/skills?tab=inventory). Prima della v2.1.246, Claude Code segnalava questi plugin come `"user-install"` o `"seed-mount"`
* `plugin_id_hash`: hash deterministico del nome del plugin e del marketplace, inviato solo al tuo esportatore configurato. Ti consente di contare i plugin di terze parti distinti caricati nella tua flotta senza registrare i loro nomi. Per i [plugin sincronizzati da claude.ai](/docs/it/plugins/loading#synced-plugins), Claude Code esegue l'hash del nome del plugin con il nome del marketplace che claude.ai riporta per il plugin, o con `synced` altrimenti. Prima della v2.1.246, Claude Code non utilizzava il nome del marketplace che claude.ai riporta nell'hash
* `has_hooks`: se il plugin contribuisce hook
* `has_mcp`: se il plugin contribuisce server MCP
* `host_owned_mcp`: `true` quando l'host SDK gestisce le connessioni MCP di questo plugin e Claude Code ha saltato la lettura della configurazione del server MCP del plugin, `false` altrimenti. Richiede Claude Code v2.1.172 o successivo
* `skill_path_count`: numero di directory di skill che il plugin dichiara
* `command_path_count`: numero di directory di comandi che il plugin dichiara
* `agent_path_count`: numero di directory di agenti che il plugin dichiara
* `safe_mode`: `"true"` quando la sessione è stata avviata con [`--safe-mode`](/docs/it/cli-reference), `"false"` altrimenti. In modalità sicura questo evento riporta solo l'inventario configurato; i comandi, le skill, gli hook e i server MCP del plugin non si caricano. Richiede Claude Code v2.1.169 o successivo

<h4 id="skill-activated-event">
  Evento di skill attivata
</h4>

Registrato quando una skill viene invocata, sia che Claude la chiami tramite lo strumento Skill che tu la esegua come comando `/`.

**Nome evento**: `claude_code.skill_activated`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `skill.name`: Nome della skill. Per le skill definite dall'utente e dai plugin di terze parti il valore è il placeholder `"custom_skill"` a meno che `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Come la skill è stata attivata (`"user-slash"`, `"claude-proactive"`, o `"nested-skill"`)
* `skill.source`: Da dove è stata caricata la skill (ad esempio, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"` quando la skill è una skill di workflow. Assente altrimenti
* `plugin.name` (quando `OTEL_LOG_TOOL_DETAILS=1` o il plugin è da un marketplace ufficiale): Nome del plugin proprietario quando la skill è fornita da un plugin
* `marketplace.name` (quando `OTEL_LOG_TOOL_DETAILS=1` o il plugin è da un marketplace ufficiale): Marketplace da cui è stato installato il plugin proprietario, quando la skill è fornita da un plugin

<h4 id="at-mention-event">
  Evento di menzione @
</h4>

Registrato quando Claude Code risolve una menzione `@` in un prompt. Non ogni menzione emette un evento: i percorsi di uscita anticipata come i rifiuti di autorizzazione, i file di grandi dimensioni, gli allegati di riferimento PDF e i guasti dell'elenco delle directory ritornano senza registrare.

**Nome evento**: `claude_code.at_mention`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `mention_type`: Tipo di menzione (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`, `"peer"`). Il valore `"peer"` significa che hai menzionato [una delle tue altre sessioni Claude Code](/docs/it/cross-session-messaging). Richiede Claude Code v2.1.232 o successivo
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
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `model`: Modello utilizzato
* `error`: Messaggio di errore finale
* `status_code`: Codice di stato HTTP come numero. Assente per errori non HTTP.
* `total_attempts`: Numero totale di tentativi effettuati
* `total_retry_duration_ms`: Tempo totale di wall-clock su tutti i tentativi
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
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `hook_event`: tipo di evento hook, ad esempio `"PreToolUse"` o `"PostToolUse"`
* `hook_type`: tipo di implementazione dell'hook: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, o `"agent"`
* `hook_source`: dove è definito l'hook: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, o `"pluginHook"`
* `safe_mode`: `"true"` quando la sessione è stata avviata con [`--safe-mode`](/docs/it/cli-reference), `"false"` altrimenti. Richiede Claude Code v2.1.169 o successivo
* `hook_matcher` (quando `OTEL_LOG_TOOL_DETAILS=1`): la stringa matcher dalla configurazione dell'hook, quando ne è impostata una
* `plugin.name` (quando `hook_source` è `"pluginHook"`): nome del plugin che contribuisce. Per i plugin al di fuori del marketplace ufficiale e del bundle incorporato il valore è `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (quando `hook_source` è `"pluginHook"`): hash deterministico del nome del plugin e del marketplace, inviato solo al tuo esportatore configurato. Ti consente di contare i plugin che contribuiscono distinti senza registrare i loro nomi. Claude Code lo calcola come descritto sotto l'[evento di plugin caricato](#plugin-loaded-event)

<h4 id="hook-execution-start-event">
  Evento di inizio esecuzione dell'hook
</h4>

Registrato quando uno o più hook iniziano a eseguire per un evento hook.

**Nome evento**: `claude_code.hook_execution_start`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `hook_event`: Tipo di evento hook, ad esempio `"PreToolUse"` o `"PostToolUse"`
* `hook_name`: Nome completo dell'hook incluso matcher, ad esempio `"PreToolUse:Write"`
* `num_hooks`: Numero di comandi hook corrispondenti
* `managed_only`: `"true"` quando sono consentiti solo hook di politica gestita
* `hook_source`: `"policySettings"` o `"merged"`
* `safe_mode`: `"true"` quando la sessione è stata avviata con [`--safe-mode`](/docs/it/cli-reference), `"false"` altrimenti. Richiede Claude Code v2.1.169 o successivo
* `hook_definitions`: Configurazione dell'hook serializzata in JSON. Incluso solo quando sia la traccia beta dettagliata che `OTEL_LOG_TOOL_DETAILS=1` sono abilitate

<h4 id="hook-execution-complete-event">
  Evento di completamento esecuzione dell'hook
</h4>

Registrato quando tutti gli hook per un evento hook hanno terminato.

**Nome evento**: `claude_code.hook_execution_complete`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `hook_event`: Tipo di evento hook
* `hook_name`: Nome completo dell'hook incluso matcher
* `num_hooks`: Numero di comandi hook corrispondenti
* `num_success`: Conteggio che ha completato con successo
* `num_blocking`: Conteggio che ha restituito una decisione di blocco
* `num_non_blocking_error`: Conteggio che ha fallito senza bloccare
* `num_cancelled`: Conteggio annullato prima del completamento
* `total_duration_ms`: Durata di wall-clock di tutti gli hook corrispondenti
* `stdout_chars`: Caratteri totali di stdout tra gli hook corrispondenti che hanno avuto successo. Richiede Claude Code v2.1.280 o successivo
* `additional_context_chars`: Caratteri totali di `additionalContext` restituiti dagli hook corrispondenti. Richiede Claude Code v2.1.280 o successivo
* `system_message_chars`: Caratteri totali di `systemMessage` restituiti dagli hook corrispondenti. Richiede Claude Code v2.1.280 o successivo
* `initial_user_message_chars`: Caratteri totali di `initialUserMessage` restituiti dagli hook corrispondenti. Richiede Claude Code v2.1.280 o successivo
* `num_outputs_persisted`: Numero di output dell'hook oltre il [limite di 10.000 caratteri](/docs/it/hooks#json-output) che Claude Code ha salvato in un file. Richiede Claude Code v2.1.280 o successivo
* `managed_only`: `"true"` quando sono consentiti solo hook di politica gestita
* `hook_source`: `"policySettings"` o `"merged"`
* `safe_mode`: `"true"` quando la sessione è stata avviata con [`--safe-mode`](/docs/it/cli-reference), `"false"` altrimenti. Richiede Claude Code v2.1.169 o successivo
* `hook_definitions`: Configurazione dell'hook serializzata in JSON. Incluso solo quando sia la traccia beta dettagliata che `OTEL_LOG_TOOL_DETAILS=1` sono abilitate

<h4 id="hook-plugin-metrics-event">
  Evento di metriche del plugin hook
</h4>

Registrato quando un hook del plugin del marketplace ufficiale emette metriche per invocazione. Solo i plugin installati da un marketplace Anthropic ufficiale possono emettere questi. I plugin del marketplace di terze parti e gli hook configurati dall'utente non emettono a questo evento. Usa questo evento per monitorare il comportamento del plugin come i tassi di ricerca, i costi e le durate dal tuo stack di osservabilità.

**Nome evento**: `claude_code.hook_plugin_metrics`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `plugin_id`: identificatore del plugin in forma `<name>@<marketplace>`
* `hook_event`: tipo di evento hook che ha emesso le metriche
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
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `trigger`: `"auto"` o `"manual"`
* `success`: `"true"` o `"false"`
* `duration_ms`: Durata della compattazione
* `pre_tokens`: Conteggio approssimativo dei token prima della compattazione
* `post_tokens`: Conteggio approssimativo dei token dopo la compattazione
* `error`: Messaggio di errore quando la compattazione non è riuscita
* `precompute_reuse`: Impostato solo quando `trigger` è `"manual"`. La compattazione automatica può preparare un riepilogo in background prima che la finestra di contesto si riempia, e questo attributo registra se `/compact` ha riutilizzato quel riepilogo preparato. `"hit"` significa che è stato riutilizzato; `"miss_custom_instructions"`, `"miss_hook"`, e `"miss_not_ready"` danno il motivo per cui è stato calcolato un riepilogo fresco. Richiede Claude Code v2.1.153 o successivo

<h4 id="subagent-completed-event">
  Evento di completamento del subagent
</h4>

Registrato quando un [subagent](/docs/it/sub-agents) finisce e restituisce il suo risultato alla conversazione che l'ha avviato. Usalo per aggregare l'utilizzo dello strumento e il tempo di esecuzione per tipo di subagent; per aggregazioni di token o costo, usa il [contatore di token](#token-counter) e il [contatore di costo](#cost-counter) filtrati a `query_source` `"subagent"`, poiché il `total_tokens` di questo evento copre solo la richiesta finale. La categoria `"subagent"` conta anche le richieste dagli hook basati su agenti, che non emettono alcun evento di subagent.

**Nome evento**: `claude_code.subagent_completed`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"subagent_completed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `agent_type`: Il tipo di subagent. I nomi degli agenti incorporati e gli agenti dai plugin del marketplace ufficiale vengono visualizzati letteralmente; gli altri nomi di agenti vengono sostituiti con `"custom"` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato
* `agent.source`: Da dove proviene la definizione dell'agente: `built-in`, `plugin`, o la fonte di impostazioni che ha definito un agente personalizzato, ad esempio `userSettings` o `projectSettings`
* `is_built_in`: Se il subagent è un tipo di agente incorporato
* `is_async`: Se il subagent è stato eseguito in [background](/docs/it/sub-agents#run-subagents-in-foreground-or-background)
* `total_tokens`: L'impronta di token della richiesta API finale del subagent: quella singola richiesta di token di input, creazione della cache, lettura della cache e output, approssimativamente la dimensione del contesto del subagent al completamento. Non una somma nell'intera esecuzione
* `total_tool_uses`: Numero di chiamate dello strumento che il subagent ha effettuato nell'intera esecuzione
* `duration_ms`: Tempo di esecuzione in millisecondi
* `model`: Il modello in cui il subagent è stato risolto per eseguire
* `final_model`: Il modello che ha prodotto la risposta finale del subagent, che differisce da `model` dopo un cambio durante l'esecuzione come un fallback. Richiede Claude Code v2.1.212 o successivo
* `model_swapped`: Se più di un modello ha servito le richieste del subagent. Richiede Claude Code v2.1.212 o successivo
* `plugin_id_hash`, `plugin.name`: Presente per gli agenti forniti da plugin. I nomi dei plugin del marketplace ufficiale vengono visualizzati letteralmente; gli altri nomi di plugin vengono sostituiti con `"third-party"` a meno che `OTEL_LOG_TOOL_DETAILS=1` non sia impostato

<h4 id="feedback-survey-event">
  Evento di sondaggio di feedback
</h4>

Registrato quando viene mostrato o risposto un sondaggio sulla qualità della sessione. Vedi [Sondaggi sulla qualità della sessione](/docs/it/data-usage#session-quality-surveys) per quello che i sondaggi raccolgono e come controllarli.

**Nome evento**: `claude_code.feedback_survey`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `event_type`: Evento del ciclo di vita del sondaggio, ad esempio `"appeared"`, `"responded"`, o `"transcript_prompt_appeared"`
* `appearance_id`: ID univoco che collega gli eventi emessi per un'istanza di sondaggio
* `survey_type`: Quale sondaggio ha prodotto l'evento. `"session"` è il prompt di valutazione "Come sta andando Claude?"
* `response`: La selezione dell'utente su eventi `responded`
* `enabled_via_override`: `true` quando [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/it/env-vars) è impostato. Emesso come booleano, non come stringa. Presente su eventi di sondaggio `session`. Filtra su questo attributo per confermare che l'override è applicato in una flotta

<h4 id="retention-sweep-event">
  Evento di sweep di conservazione
</h4>

Registrato una volta per esecuzione dello sweep di pulizia della conservazione, che elimina [trascrizioni di sessione e altri dati dell'applicazione](/docs/it/claude-directory#cleaned-up-automatically) più vecchi dell'impostazione [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays). Claude Code esegue lo sweep in background al massimo una volta per sessione, e un'esecuzione che non elimina nulla emette comunque l'evento. Se Claude Code ha eseguito lo sweep in qualsiasi sessione sulla stessa macchina nelle ultime 24 ore, ritarda lo sweep di questa sessione di almeno 10 minuti, quindi una sessione che esce prima non emette nulla. Quando esegui `claude -p` con `--bare`, Claude Code non esegue lo sweep e non emette nulla.

Come ogni evento OTel su questa pagina, va solo al backend di telemetria che configuri. Richiede Claude Code v2.1.227 o successivo.

Quando Claude Code non può determinare in modo sicuro il periodo di conservazione, mette in pausa lo sweep e emette l'evento con `result` impostato a `"skipped"` e un `skip_reason`. Quando le [impostazioni gestite](/docs/it/server-managed-settings) impostano `cleanupPeriodDays`, il valore gestito fissa il periodo di conservazione e lo sweep viene eseguito anche quando un file di impostazioni in un ambito di priorità inferiore è rotto o non valido. Quando `managed-settings.json` stesso non può essere letto, Claude Code mette comunque in pausa lo sweep a meno che il [livello gestito](/docs/it/managed-settings#how-claude-code-combines-managed-sources) fornisca `cleanupPeriodDays` da altrove, ad esempio dalle impostazioni gestite dal server o da un drop-in `managed-settings.d/` accanto al file rotto. Gli attributi del contatore di eliminazione sono presenti solo quando `result` è `"complete"`.

**Nome evento**: `claude_code.retention_sweep`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"retention_sweep"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `result`: `"complete"` quando lo sweep è stato eseguito, `"skipped"` quando Claude Code l'ha messo in pausa
* `period_days`: Il valore `cleanupPeriodDays` dalle impostazioni unite, in giorni, o `30` quando nessuna fonte lo imposta. Su eventi saltati, il valore che lo sweep avrebbe utilizzato, calcolato dalle fonti di impostazioni che Claude Code potrebbe leggere
* `used_default`: `"true"` quando nessuna fonte di impostazioni leggibile imposta `cleanupPeriodDays`, `"false"` altrimenti. Su eventi completi, `"true"` significa che il default di 30 giorni è stato applicato
* `skip_reason`: Perché Claude Code ha messo in pausa lo sweep. Presente solo quando `result` è `"skipped"`:
  * `"user_source_disabled"`: Le impostazioni utente sono escluse, ad esempio dal flag [`--setting-sources`](/docs/it/cli-reference#cli-flags) o dall'opzione [`settingSources`](/docs/it/agent-sdk/typescript#options) dell'SDK, e nessuna fonte abilitata fornisce `cleanupPeriodDays`
  * `"settings_unknowable"`: Un file di impostazioni non poteva essere letto o analizzato, quindi `cleanupPeriodDays` o `desktopSessionCleanupPeriodDays` potrebbe essere impostato a un valore che Claude Code non può vedere
  * `"settings_invalid_key_set"`: Le impostazioni hanno errori di convalida e `cleanupPeriodDays` o `desktopSessionCleanupPeriodDays` è esplicitamente impostato, quindi il fallback al default potrebbe eliminare o mantenere file contro quell'impostazione
* `transcripts_deleted`: Numero di trascrizioni di sessione, i file `~/.claude/projects/*/*.jsonl` di livello superiore, che lo sweep ha eliminato
* `transcripts_exempted_desktop`: Numero di trascrizioni passate il periodo di conservazione che lo sweep ha mantenuto secondo la [regola di Claude Desktop e Cowork](/docs/it/claude-directory#cleaned-up-automatically). Questi non contano verso `files_past_cutoff`. Richiede Claude Code v2.1.248 o successivo
* `session_files_deleted`: Numero di artefatti che lo sweep della sessione ha eliminato: trascrizioni più file complementari per sessione come sidecar, registrazioni e risultati degli strumenti
* `artifacts_deleted`: Elementi totali che lo sweep ha eliminato nelle directory di dati che copre, inclusi i file della sessione. Alcuni sweep contano un intero albero di directory rimosso come un elemento e alcuni passaggi di pulizia non contribuiscono al contatore, quindi tratta il valore come un limite inferiore piuttosto che un conteggio esatto di file
* `files_retained_fresh`: File ispezionati e lasciati in posizione perché sono ancora entro il periodo di conservazione. Solo i sweep per file contano questi, quindi il valore è un limite inferiore; un valore diverso da zero è lo stato stazionario normale
* `files_past_cutoff`: File più vecchi del periodo di conservazione che lo sweep non ha potuto eliminare, ad esempio a causa di un errore di autorizzazione o di un file tenuto aperto. Un valore superiore a zero significa che i file hanno superato il periodo di conservazione configurato; zero non è la prova che nessuno l'ha fatto, perché una rimozione non riuscita di un'intera directory conta verso `error_count` invece
* `error_count`: Numero di errori che lo sweep ha incontrato durante l'elenco o l'eliminazione di file

<h4 id="managed-settings-resolved-event">
  Evento di impostazioni gestite risolte
</h4>

Registrato con le [impostazioni gestite](/docs/it/managed-settings) che una sessione ha risolto: una volta all'avvio della sessione, di nuovo quando le impostazioni gestite o lo stato dell'[helper di politica](/docs/it/managed-settings#compute-the-policy-with-a-helper-program) cambiano durante la sessione, e quando Claude Code rifiuta di avviare o termina la sessione per uno dei motivi che l'attributo `error.type` elenca.
Usa questo evento per trovare macchine in esecuzione su una fonte gestita inaspettata, macchine il cui helper di politica sta fallendo, e il motivo per cui una macchina ha rifiutato di avviare.
Richiede Claude Code v2.1.274 o successivo.

Per impostazione predefinita, l'evento contiene le fonti gestite e lo stato dell'helper di politica ma non le impostazioni stesse. Per aggiungere l'attributo `managed_settings.settings` redatto e il digest `managed_settings.resolved_sha256`, imposta `OTEL_LOG_MANAGED_SETTINGS=1`:

* Impostalo nel blocco `env` delle impostazioni gestite, impostazioni utente, o `--settings`, o nell'ambiente in cui avvii Claude Code. Un valore nelle impostazioni di progetto o locali non lo attiva, perché un repository clonato può scriverle.
* Le impostazioni gestite dal server possono impostarla senza mostrare la [finestra di dialogo di approvazione della sicurezza](/docs/it/server-managed-settings#security-approval-dialogs), perché la variabile aggiunge solo la tua politica redatta dell'organizzazione a un evento che la tua organizzazione riceve già.

In una sessione interattiva in una cartella che non hai [fidata](/docs/it/permissions#what-runs-before-you-trust-a-folder), Claude Code non esporta l'evento di rifiuto, perché le impostazioni di progetto e locali potrebbero puntare l'esportazione a un collector diverso prima della fiducia.

**Nome evento**: `claude_code.managed_settings_resolved`

**Attributi**:

* Tutti gli [attributi standard](#standard-attributes)
* `event.name`: `"managed_settings_resolved"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contatore per processo per ordinare gli eventi, descritto sotto [Attributi di correlazione degli eventi](#event-correlation-attributes)
* `managed_settings.trigger`: `"startup"` per l'evento di avvio della sessione, `"change"` quando le impostazioni gestite o lo stato dell'helper di politica sono cambiati più tardi nella sessione, o `"refused"` quando una politica di impostazioni gestite ha fermato la sessione. Claude Code invia un evento `change` solo quando un attributo differisce dall'ultimo evento che ha inviato, e un valore di impostazione modificato conta anche quando `OTEL_LOG_MANAGED_SETTINGS` è disattivato
* `error.type`: perché Claude Code ha fermato la sessione. Presente solo su eventi `refused`:
  * `"helper_failed"`: un'[esecuzione dell'helper di politica non è riuscita](/docs/it/settings-reference#helper-failures)
  * `"policy_invalid"`: le impostazioni gestite contengono un errore che ferma Claude Code dall'avvio, o un'origine amministrativa non è riuscita a caricarsi, quindi Claude Code non può controllare l'applicazione dell'accesso all'organizzazione
  * `"consent_rejected"`: l'utente ha rifiutato la [finestra di dialogo di approvazione della sicurezza](/docs/it/server-managed-settings#security-approval-dialogs) per le impostazioni gestite dal server
  * `"force_refresh_failed"`: il fetch delle impostazioni che [`forceRemoteSettingsRefresh`](/docs/it/settings-reference#forceremotesettingsrefresh) richiede non è riuscito
  * `"gateway_rejected"`: un [gateway di app Claude](/docs/it/claude-apps-gateway) ha risposto al caricamento delle impostazioni gestite con HTTP 403
  * `"version_below_minimum"`: questa versione di Claude Code è al di sotto di [`requiredMinimumVersion`](/docs/it/settings-reference#requiredminimumversion) o al di sopra di [`requiredMaximumVersion`](/docs/it/settings-reference#requiredmaximumversion)
  * `"_OTHER"`: il caricamento delle impostazioni gestite del gateway di app Claude non è riuscito per un altro motivo
* `managed_settings.sources`: ogni fonte gestita che fornisce almeno una [chiave di politica](/docs/it/managed-settings#how-claude-code-combines-managed-sources), priorità più alta per prima, incluse le fonti le cui chiavi non hanno effetto sotto `first-wins`. I valori sono `"remote"`, `"plist"` o `"hklm"` per la politica MDM o a livello di OS, `"file"` per i file di impostazioni gestite e drop-in, `"parent"` quando un [host di incorporamento](/docs/it/managed-settings#let-an-embedding-host-add-policy) fornisce impostazioni, e `"hkcu"` per il [valore del registro Windows HKCU](/docs/it/managed-settings#where-each-mechanism-stores-the-policy) quando Claude Code lo [legge](/docs/it/managed-settings#how-claude-code-combines-managed-sources). Una fonte che porta solo chiavi di controllo, o che Claude Code non poteva leggere, non è elencata. Emesso come array di stringhe, vuoto quando nessuna fonte gestita fornisce una chiave di politica
* `managed_settings.source_behavior`: il valore [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior) che Claude Code ha letto, `"first-wins"` o `"merge"`. `"first-wins"` quando nessuna fonte imposta la chiave
* `managed_settings.helper.state`: stato dell'helper di politica che la fonte MDM o file selezionata configura:
  * `"ok"`: l'output dell'helper serve come impostazioni gestite
  * `"bad_path"`, `"not_a_file"`, `"exit_nonzero"`, `"timed_out"`, `"oversize"`, `"parse_failed"`, `"envelope_invalid"`, o `"schema_rejected"`: l'ultima esecuzione dell'helper non è riuscita. [Helper failures](/docs/it/settings-reference#helper-failures) descrive i casi
  * `"none"`: nessun helper è configurato, o la fonte che lo configura non è una politica MDM o un file di impostazioni gestite
* `managed_settings.helper.applied`: `"output"` mentre l'output dell'helper stesso serve come impostazioni gestite, `"none"` quando non lo fa
* `managed_settings.helper.entry`: `"policyHelper"` quando Claude Code ha selezionato un [`policyHelper`](/docs/it/settings-reference#policyhelper). Assente quando non ha selezionato alcun helper
* `managed_settings.helper.path`: il [`path`](/docs/it/settings-reference#policyhelper-path) configurato dell'helper. Presente ogni volta che Claude Code ha selezionato un helper, indipendentemente dal fatto che `OTEL_LOG_MANAGED_SETTINGS` sia impostato
* `managed_settings.resolved_sha256` (quando `OTEL_LOG_MANAGED_SETTINGS=1`): SHA-256 delle impostazioni gestite risolte prima della redazione, serializzate come JSON con chiavi ordinate ricorsivamente e senza spazi. Le macchine con lo stesso digest eseguono la stessa politica. Claude Code invia il digest solo con l'opt-in perché una politica breve può essere recuperata eseguendo l'hash di ipotesi. Assente quando nessuna impostazione gestita è stata risolta, e su eventi `refused`
* `managed_settings.settings` (quando `OTEL_LOG_MANAGED_SETTINGS=1`): i nomi e la forma delle impostazioni gestite risolte come stringa JSON, con i valori redatti. Assente su eventi `refused`. Claude Code lo costruisce dal suo schema di impostazioni:

  * Un nome di impostazione che lo schema dichiara viene esportato, e una chiave che non dichiara viene lasciata fuori
  * Booleani, numeri e valori di stringa che lo schema limita a un insieme fisso di opzioni, come `permissions.defaultMode`, vengono esportati così come sono. `sandbox.network.httpProxyPort` e `sandbox.network.socksProxyPort` vengono esportati come `"[REDACTED]"`
  * Ogni altra stringa, come `model`, `apiKeyHelper`, ogni valore `env`, ogni URL, e ogni comando, viene esportato come `"[REDACTED]"`
  * I nomi delle voci di mappe, come i nomi delle variabili `env` e gli ID dei plugin, vengono esportati così come sono. Un'impostazione le cui voci lo schema non digita, come `vimInsertModeRemaps`, viene esportata come un singolo `"[REDACTED]"`, e `sandbox.ignoreViolations` viene esportato come un elenco dei suoi elenchi di percorsi senza i modelli di comando
  * Un elenco mantiene la sua lunghezza, con ogni voce redatta dalle stesse regole
  * Una regola `permissions.allow`, `permissions.deny`, o `permissions.ask` viene esportata come il suo nome dello strumento con il contenuto redatto, come `Read([REDACTED])`, quando lo strumento è incorporato in questa versione di Claude Code o è un riferimento `mcp__` come `mcp__jira__create_issue`. Qualsiasi altra regola viene esportata come `"[REDACTED]"`
  * Gli hook seguono le stesse regole, quindi i campi a opzione fissa e numerici come `type` e `timeout` vengono mostrati, mentre ogni comando, URL, `matcher`, e condizione `if` viene esportato come `"[REDACTED]"`

  Ad esempio, le impostazioni gestite con `apiKeyHelper`, due variabili `env`, e una regola di negazione vengono esportate come `{"apiKeyHelper":"[REDACTED]","env":{"HTTPS_PROXY":"[REDACTED]","CLAUDE_CODE_ENABLE_TELEMETRY":"[REDACTED]"},"permissions":{"deny":["Read([REDACTED])"]}}`.

  Claude Code taglia il valore a 8 KB di UTF-8, e il valore tagliato non è JSON valido
* `managed_settings.settings_truncated` (quando `managed_settings.settings` è presente): `true` quando Claude Code ha tagliato `managed_settings.settings` a 8 KB, `false` altrimenti. Emesso come booleano, non come stringa

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

Claude Code conta ogni risposta in streaming verso le metriche di costo e token esattamente una volta, incluso quando un gateway o proxy dietro `ANTHROPIC_BASE_URL` trasmette l'utilizzo progressivamente su più frame. Prima della v2.1.214, i flussi che contenevano utilizzo in più di un frame gonfiavano `claude_code.cost.usage` e `claude_code.token.usage` di circa una richiesta completa aggiuntiva per ogni frame aggiuntivo.

<h3 id="alerting-and-segmentation">
  Avvisi e segmentazione
</h3>

Avvisi comuni da considerare:

* Picchi di costo
* Consumo di token inusuale
* Alto volume di sessioni da utenti specifici

Tutte le metriche possono essere segmentate dagli [attributi standard](#standard-attributes). L'attributo `model` è disponibile su `claude_code.token.usage`, `claude_code.cost.usage`, e da v2.1.172, `claude_code.lines_of_code.count`.

Le suddivisioni per modello dei commit possono essere solo approssimate tramite un join con le metriche di token o di costo su `session.id`, poiché una sessione può estendersi su più modelli. Filtra il lato token o costo sulle righe in cui `query_source` è `"main"`, in modo che le richieste ausiliarie e dei subagent non attribuiscano i commit della sessione a un modello che non li ha effettuati.

<h3 id="detect-retry-exhaustion">
  Rilevare l'esaurimento dei tentativi
</h3>

Claude Code ritenta internamente le richieste API non riuscite ed emette un singolo evento `claude_code.api_error` solo dopo che rinuncia, quindi l'evento stesso è il segnale terminale per quella richiesta. I tentativi di ripetizione intermedi non vengono registrati come eventi separati.

L'attributo `attempt` sull'evento registra il numero totale di tentativi. `CLAUDE_CODE_MAX_RETRIES` ha un valore predefinito di 10 ed è limitato a 15. A partire da v2.1.199, puoi impostare `CLAUDE_CODE_RETRY_WATCHDOG` per aumentare il valore predefinito e rimuovere il limite.

Quando la richiesta esaurisce tutti i tentativi su un errore transitorio, `attempt` è uguale a uno più di quel limite effettivo: 11 per impostazione predefinita, e mai più di 16 a meno che il watchdog non sia impostato. Un valore inferiore indica un errore non ritentabile come una risposta `400`, o una causa con il suo proprio budget di tentativi più piccolo. Ad esempio, Claude Code ritenta un errore nel caricamento delle credenziali AWS o Google Cloud al massimo due volte.

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

Gli [attributi standard](#standard-attributes) su ogni evento includono l'identità dell'utente autenticato: `user.email`, `user.account_uuid`, `user.account_id`, e `organization.id` quando accedete con un account Claude o, in una [sessione cloud](/docs/it/claude-code-on-the-web), quando le credenziali della sessione stessa le portano, più `user.id` e il `session.id` per sessione. `user.id` è un identificatore con ambito di installazione, tranne nelle sessioni [Claude apps gateway](/docs/it/claude-apps-gateway), dove è il soggetto IdP dal token emesso dal gateway.

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

* `tool_result`: mantiene `mcp_server_scope` e un `tool_name` sostituito con il letterale `"mcp_tool"` per i server configurati dall'utente, omette il contenuto degli argomenti. Per i server integrati di Claude Desktop, nelle sessioni di cui Claude Desktop è proprietario, mantiene anche la coppia `mcp_server_name`/`mcp_tool_name` all'interno di `tool_parameters`, la stessa eccezione creata dall'host come `tool_decision`, richiedendo Claude Code v2.1.214 o successivo
* `tool_decision`: mantiene `tool_source` e un `tool_name` sostituito con il letterale `"mcp_tool"` per i server configurati dall'utente, omette il contenuto degli argomenti. Per i server integrati di Claude Desktop, nelle sessioni di cui Claude Desktop è proprietario, mantiene anche la coppia `mcp_server_name`/`mcp_tool_name` all'interno di `tool_parameters`; `tool_source` e la coppia di nomi richiedono entrambi Claude Code v2.1.214 o successivo
* `mcp_server_connection`: omette `server_name` e il messaggio di errore, ma mantiene `is_plugin`, `plugin_id_hash`, e `plugin.name`, con i nomi dei plugin non Anthropic sostituiti con il letterale `"third-party"`, in modo che i server forniti dai plugin rimangano distinguibili senza registrazione dettagliata

<h3 id="map-security-questions-to-events">
  Mappa le domande di sicurezza agli eventi
</h3>

Quando costruite regole di rilevamento, cercate il segnale che volete monitorare e interrogate il vostro backend per l'evento corrispondente e gli attributi:

| Segnale                                                                                                                                        | Evento                                                                               | Attributi chiave                                                                                                                                                                                                                              |
| ---------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Chiamata dello strumento consentita o negata, e da cosa                                                                                        | `tool_decision`                                                                      | `decision`, `source`, `tool_name`, `tool_parameters`                                                                                                                                                                                          |
| Escalation della modalità di autorizzazione                                                                                                    | `permission_mode_changed`                                                            | `from_mode`, `to_mode`, `trigger`                                                                                                                                                                                                             |
| Hook della politica ha bloccato un'azione                                                                                                      | `hook_execution_complete`                                                            | `hook_event`, `num_blocking`                                                                                                                                                                                                                  |
| Login, logout e guasto di autenticazione                                                                                                       | `auth`                                                                               | `action`, `success`, `error_category`                                                                                                                                                                                                         |
| Connessione del server MCP o guasto                                                                                                            | `mcp_server_connection`                                                              | `status`, `server_name`, `is_plugin`, `error_code`                                                                                                                                                                                            |
| Plugin installato e la sua fonte                                                                                                               | `plugin_installed`                                                                   | `plugin.name`, `marketplace.name`, `marketplace.is_official`                                                                                                                                                                                  |
| Comandi eseguiti e file toccati                                                                                                                | `tool_result` (eseguito) o `tool_decision` (rifiutato) con `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (`tool_result` solo)                                                                                                                                                                                          |
| Quali fonti di impostazioni gestite una macchina esegue, se il suo helper di politica è integro e perché una macchina ha rifiutato di avviarsi | `managed_settings_resolved`                                                          | `managed_settings.trigger`, `managed_settings.sources`, `managed_settings.source_behavior`, `managed_settings.helper.state`, `error.type`; `managed_settings.settings` e `managed_settings.resolved_sha256` con `OTEL_LOG_MANAGED_SETTINGS=1` |

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

Per confermare che gli eventi arrivano, inviate un prompt in una sessione in esecuzione con questa configurazione e controllate il vostro SIEM per l'evento `claude_code.user_prompt`. Se nulla arriva, eseguite `claude --debug` e controllate il log di debug per gli errori di esportazione `[3P telemetry]`.

<h2 id="backend-considerations">
  Considerazioni sul backend
</h2>

La scelta dei backend di metriche, log e tracce determina i tipi di analisi che puoi eseguire:

<h3 id="for-metrics">
  Per le metriche
</h3>

* **Database di serie temporali**: Calcoli di velocità, metriche aggregate
* **Archivi colonnari**: Query complesse, analisi di utenti univoci
* **Piattaforme di osservabilità complete**: Query avanzate, visualizzazione, avvisi

<h3 id="for-events/logs">
  Per eventi/log
</h3>

* **Sistemi di aggregazione dei log**: Ricerca full-text, analisi dei log
* **Archivi colonnari**: Analisi degli eventi strutturati
* **Piattaforme di osservabilità complete**: Correlazione tra metriche e eventi

<h3 id="for-traces">
  Per tracce
</h3>

Scegli un backend che supporti l'archiviazione di tracce distribuite e la correlazione degli span:

* **Sistemi di traccia distribuita**: Visualizzazione degli span, waterfall delle richieste, analisi della latenza
* **Piattaforme di osservabilità complete**: Ricerca di tracce e correlazione con metriche e log

Per le organizzazioni che richiedono metriche Daily/Weekly/Monthly Active User (DAU/WAU/MAU), considera backend che supportano query di valori univoci efficienti.

<h2 id="service-information">
  Informazioni sul servizio
</h2>

Tutte le metriche e gli eventi vengono esportati con i seguenti attributi di risorsa:

* `service.name`: `claude-code` per le sessioni di terminale, `claude-code-desktop` per le sessioni avviate dalla scheda Code nell'[app Claude Desktop](/docs/it/desktop)
* `service.version`: Versione corrente di Claude Code, o la versione dell'app Desktop per le sessioni della scheda Code
* `os.type`: Tipo di sistema operativo (ad esempio, `linux`, `darwin`, `windows`)
* `os.version`: Stringa della versione del sistema operativo
* `host.arch`: Architettura dell'host (ad esempio, `amd64`, `arm64`)
* `wsl.version`: Numero di versione WSL (presente solo quando si esegue su Windows Subsystem for Linux)
* Meter Name: `com.anthropic.claude_code`

Se le pipeline del collettore o i dashboard filtrano su `service.name = claude-code`, aggiungere `claude-code-desktop` al filtro per acquisire anche la telemetria dalle sessioni della scheda Code.

<h2 id="roi-measurement-resources">
  Risorse di misurazione del ROI
</h2>

Per una guida completa sulla misurazione del ritorno sull'investimento per Claude Code, inclusa la configurazione della telemetria, l'analisi dei costi, le metriche di produttività e i report automatizzati, consulta la [Guida alla misurazione del ROI di Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Questo repository fornisce configurazioni Docker Compose pronte all'uso, configurazioni Prometheus e OpenTelemetry, e modelli per generare report di produttività integrati con strumenti come Linear.

<h2 id="security-and-privacy">
  Sicurezza e privacy
</h2>

* L'esportazione OpenTelemetry al tuo backend è opt-in e richiede una configurazione esplicita. Per la telemetria operazionale separata di Anthropic e come disabilitarla, vedi [Data usage](/docs/it/data-usage#telemetry-services)
* I contenuti dei file grezzi e i frammenti di codice non sono inclusi nelle metriche o negli eventi. I percorsi di traccia degli span sono un percorso dati separato: vedi il punto `OTEL_LOG_TOOL_CONTENT` di seguito
* Quando autenticato tramite OAuth, `user.email` è incluso negli attributi di telemetria, inviato solo all'endpoint OTel che configuri, mai ad Anthropic. Se questo è una preoccupazione per la tua organizzazione, lavora con il tuo backend di telemetria per filtrare o oscurare questo campo
* Il contenuto del prompt dell'utente non viene raccolto per impostazione predefinita. Viene registrata solo la lunghezza del prompt. Per includere il contenuto del prompt, imposta `OTEL_LOG_USER_PROMPTS=1`. Sotto la traccia beta dettagliata questa variabile raggiunge più lontano del testo del prompt: controlla anche l'[attributo span `new_context`](#new-context-gates), che porta i risultati dello strumento sullo span `claude_code.llm_request`
* Il testo della risposta dell'assistente non viene raccolto per impostazione predefinita. Viene registrata solo la lunghezza della risposta. Per includere il testo della risposta, imposta `OTEL_LOG_ASSISTANT_RESPONSES=1`. Come tutti i dati OpenTelemetry da Claude Code, il testo della risposta viene inviato solo all'endpoint OTel che configuri, mai ad Anthropic. Quando questa variabile non è impostata, `OTEL_LOG_USER_PROMPTS` viene utilizzato come fallback, quindi imposta `OTEL_LOG_ASSISTANT_RESPONSES=0` se desideri il contenuto del prompt senza il contenuto della risposta
* Gli argomenti di input dello strumento e i parametri non vengono registrati per impostazione predefinita. Per includerli, imposta `OTEL_LOG_TOOL_DETAILS=1`. Per i server integrati di Claude Desktop, nelle sessioni di cui Claude Desktop è proprietario, `tool_decision` e `tool_result` portano la coppia `mcp_server_name`/`mcp_tool_name`, nomi creati dall'host piuttosto che contenuto degli argomenti, anche con il flag disattivato. L'eccezione richiede Claude Code v2.1.214 o successivo. Questi dati vengono inviati solo all'endpoint OTEL che configuri, mai ad Anthropic. Gli argomenti potrebbero comunque contenere valori sensibili, quindi configura il tuo backend di telemetria per filtrare o oscurare questi attributi secondo necessità. Quando abilitato:
  * Gli eventi `tool_result` e `tool_decision` includono un attributo `tool_parameters` con comandi Bash, nomi dei server MCP e dello strumento, e nomi delle skill. I campi come `full_command` vengono emessi non troncati
  * Gli eventi `tool_result` includono inoltre un attributo `tool_input` con percorsi di file, URL, modelli di ricerca e altri argomenti. I singoli valori superiori a 512 caratteri vengono troncati e il totale è limitato a circa 4 K caratteri
  * Gli eventi `user_prompt` includono il `command_name` verbatim per i comandi personalizzati, plugin e MCP
  * Gli span di traccia includono lo stesso attributo `tool_input` e attributi derivati dall'input come `file_path`, con lo stesso troncamento di `tool_input`
* Il contenuto dello strumento non viene registrato negli span di traccia per impostazione predefinita. Per includerlo, imposta `OTEL_LOG_TOOL_CONTENT=1`. Lo span `claude_code.tool` quindi porta un [evento span `tool.output`](#tool-output-span-event) con contenuti di file grezzi e output dei comandi Bash, troncato al limite di contenuto (60 KB per impostazione predefinita) per attributo. Il contenuto dello strumento raggiunge anche gli span attraverso [`new_context`, il cui gate differisce per span](#new-context-gates). Configura il tuo backend di telemetria per filtrare o oscurare questi attributi secondo necessità
* I corpi grezzi della richiesta e della risposta dell'API Anthropic Messages non vengono registrati per impostazione predefinita. Per includerli, imposta `OTEL_LOG_RAW_API_BODIES` nella tua shell, nelle impostazioni utente o nelle impostazioni gestite. Viene ignorato nelle [impostazioni di progetto e locali](/docs/it/settings-reference#variables-claude-code-ignores-in-env). I corpi contengono l'intera cronologia della conversazione, incluso il prompt di sistema, ogni turno precedente dell'utente e dell'assistente, e risultati degli strumenti, quindi l'abilitazione di questa opzione implica il consenso a tutto ciò che gli altri flag di contenuto `OTEL_LOG_*` rivelerebbero. Claude Code oscura sempre il contenuto del pensiero esteso di Claude in questi corpi, indipendentemente da altre impostazioni. Il valore che imposti determina come Claude Code fornisce i corpi:
  * Con `=1`, Claude Code emette eventi di log `api_request_body` e `api_response_body` per ogni chiamata API. L'attributo `body` degli eventi porta il payload serializzato in JSON, troncato al limite di contenuto (60 KB per impostazione predefinita)
  * Con `=file:<dir>`, Claude Code scrive i corpi non troncati nei file `.request.json` e `.response.json` in quella directory, e gli eventi portano un percorso `body_ref` invece del corpo inline. Spedisci la directory con un log collector o sidecar piuttosto che attraverso il flusso di telemetria

    Per ogni risposta riuscita, Claude Code aggiunge anche una riga a `index.jsonl` in quella directory, collegando il file di risposta al file di richiesta che lo ha prodotto e al messaggio di trascrizione in cui è diventato. Ogni riga non contiene contenuto di messaggio, e la sezione [evento corpo della risposta API](#api-response-body-event) elenca i suoi campi. Il file di indice richiede Claude Code v2.1.274 o successivo

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Monitoraggio di Claude Code su Amazon Bedrock
</h2>

Per una guida dettagliata al monitoraggio dell'utilizzo di Claude Code per Amazon Bedrock, consulta [Claude Code Monitoring Implementation (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
