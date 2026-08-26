> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Osservabilità con OpenTelemetry

> Esporta tracce, metriche ed eventi dall'Agent SDK al tuo backend di osservabilità utilizzando OpenTelemetry.

Quando esegui agenti in produzione, hai bisogno di visibilità su ciò che hanno fatto:

* quali strumenti hanno chiamato
* quanto tempo ha impiegato ogni richiesta al modello
* quanti token sono stati spesi
* dove si sono verificati i guasti

L'Agent SDK può esportare questi dati come tracce OpenTelemetry, metriche ed eventi di log a qualsiasi backend che accetti il Protocollo OpenTelemetry (OTLP), come Honeycomb, Datadog, Grafana, Langfuse o un collector self-hosted.

Questa guida spiega come l'SDK emette telemetria, come configurare l'esportazione e come etichettare e filtrare i dati una volta che raggiungono il tuo backend. Per leggere l'utilizzo dei token e il costo direttamente dal flusso di risposta dell'SDK invece di esportare a un backend, vedi [Traccia costo e utilizzo](/docs/it/agent-sdk/cost-tracking).

<h2 id="how-telemetry-flows-from-the-sdk">
  Come la telemetria fluisce dall'SDK
</h2>

L'Agent SDK esegue il Claude Code CLI come processo figlio e comunica con esso tramite una pipe locale. Il CLI ha la strumentazione OpenTelemetry integrata: registra span attorno a ogni richiesta al modello e esecuzione dello strumento, emette metriche per i contatori di token e costo, ed emette eventi di log strutturati per i prompt e i risultati degli strumenti. L'SDK non produce telemetria propria. Invece, passa la configurazione al processo CLI, e il CLI esporta direttamente al tuo collector.

La configurazione viene passata come variabili di ambiente. Per impostazione predefinita, il processo figlio eredita l'ambiente della tua applicazione, quindi puoi configurare la telemetria in uno di due posti:

* **Ambiente del processo:** imposta le variabili nella tua shell, container o orchestrator prima che la tua applicazione si avvii. Ogni chiamata `query()` le raccoglie automaticamente senza alcun cambio di codice. Questo è l'approccio consigliato per le distribuzioni in produzione.
* **Opzioni per chiamata:** imposta le variabili in `ClaudeAgentOptions.env` (Python) o `options.env` (TypeScript). Usa questo quando diversi agenti nello stesso processo hanno bisogno di impostazioni di telemetria diverse. In Python, `env` viene unito sopra l'ambiente ereditato. In TypeScript, `env` sostituisce completamente l'ambiente ereditato, quindi includi `...process.env` nell'oggetto che passi.

Il CLI esporta tre segnali OpenTelemetry indipendenti. Ognuno ha il suo interruttore di abilitazione e il suo esportatore, quindi puoi attivare solo quelli di cui hai bisogno.

| Segnale    | Cosa contiene                                                                             | Abilita con                                                        |
| ---------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Metrics    | Contatori per token, costo, sessioni, righe di codice e decisioni degli strumenti         | `OTEL_METRICS_EXPORTER`                                            |
| Log events | Record strutturati per ogni prompt, richiesta API, errore API e risultato dello strumento | `OTEL_LOGS_EXPORTER`                                               |
| Traces     | Span per ogni interazione, richiesta al modello, chiamata dello strumento e hook (beta)   | `OTEL_TRACES_EXPORTER` più `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

Per l'elenco completo dei nomi delle metriche, dei nomi degli eventi e degli attributi, vedi il riferimento [Monitoring](/docs/it/monitoring-usage) di Claude Code. L'Agent SDK emette gli stessi dati perché esegue lo stesso CLI. I nomi degli span sono elencati in [Leggi le tracce dell'agente](#read-agent-traces) di seguito.

<h2 id="enable-telemetry-export">
  Abilita l'esportazione della telemetria
</h2>

La telemetria è disattivata finché non imposti `CLAUDE_CODE_ENABLE_TELEMETRY=1` e scegli almeno un esportatore. La configurazione più comune invia tutti e tre i segnali su OTLP HTTP a un collector.

L'esempio seguente imposta le variabili in un dizionario e le passa attraverso `options.env`. L'agente esegue un singolo compito e il CLI esporta span, metriche ed eventi al collector in `collector.example.com` mentre il loop consuma il flusso di risposta:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions

  OTEL_ENV = {
      "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
      # Required for traces, which are in beta. Metrics and log events do not need this.
      "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA": "1",
      # Choose an exporter per signal. Use otlp for the SDK; see the Note below.
      "OTEL_TRACES_EXPORTER": "otlp",
      "OTEL_METRICS_EXPORTER": "otlp",
      "OTEL_LOGS_EXPORTER": "otlp",
      # Standard OTLP transport configuration.
      "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
      "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4318",
      "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-token",
  }


  async def main():
      options = ClaudeAgentOptions(env=OTEL_ENV)
      async for message in query(
          prompt="List the files in this directory", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const otelEnv = {
    CLAUDE_CODE_ENABLE_TELEMETRY: "1",
    // Required for traces, which are in beta. Metrics and log events do not need this.
    CLAUDE_CODE_ENHANCED_TELEMETRY_BETA: "1",
    // Choose an exporter per signal. Use otlp for the SDK; see the Note below.
    OTEL_TRACES_EXPORTER: "otlp",
    OTEL_METRICS_EXPORTER: "otlp",
    OTEL_LOGS_EXPORTER: "otlp",
    // Standard OTLP transport configuration.
    OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
    OTEL_EXPORTER_OTLP_ENDPOINT: "http://collector.example.com:4318",
    OTEL_EXPORTER_OTLP_HEADERS: "Authorization=Bearer your-token",
  };

  for await (const message of query({
    prompt: "List the files in this directory",
    // env replaces the inherited environment in TypeScript, so spread
    // process.env first to keep PATH, ANTHROPIC_API_KEY, and other variables.
    options: { env: { ...process.env, ...otelEnv } },
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Poiché il processo figlio eredita l'ambiente della tua applicazione per impostazione predefinita, puoi ottenere lo stesso risultato esportando queste variabili in un Dockerfile, manifesto Kubernetes o profilo shell e omettendo completamente `options.env`.

<Note>
  L'esportatore `console` scrive la telemetria su standard output, che l'SDK utilizza come canale di messaggi. Non impostare `console` come valore di esportatore quando esegui attraverso l'SDK. Per ispezionare la telemetria localmente, punta `OTEL_EXPORTER_OTLP_ENDPOINT` a un collector locale o a un container Jaeger all-in-one.
</Note>

<h3 id="flush-telemetry-from-short-lived-calls">
  Scarica la telemetria dalle chiamate di breve durata
</h3>

Il CLI raggruppa la telemetria ed esporta a intervalli. All'uscita pulita del processo tenta di scaricare i dati in sospeso, ma lo scaricamento è limitato da un timeout breve, quindi gli span possono comunque essere eliminati se il collector è lento a rispondere. Se il tuo processo viene terminato prima che il CLI si arresti, tutto ciò che è ancora nel buffer batch viene perso. Abbassare gli intervalli di esportazione riduce entrambe le finestre.

Per impostazione predefinita, le metriche esportano ogni 60 secondi e le tracce e i log esportano ogni 5 secondi. L'esempio seguente accorcia tutti e tre gli intervalli in modo che i dati raggiungano il collector mentre un compito breve è ancora in esecuzione:

<CodeGroup>
  ```python Python theme={null}
  OTEL_ENV = {
      # ... exporter configuration from the previous example ...
      "OTEL_METRIC_EXPORT_INTERVAL": "1000",
      "OTEL_LOGS_EXPORT_INTERVAL": "1000",
      "OTEL_TRACES_EXPORT_INTERVAL": "1000",
  }
  ```

  ```typescript TypeScript theme={null}
  const otelEnv = {
    // ... exporter configuration from the previous example ...
    OTEL_METRIC_EXPORT_INTERVAL: "1000",
    OTEL_LOGS_EXPORT_INTERVAL: "1000",
    OTEL_TRACES_EXPORT_INTERVAL: "1000",
  };
  ```
</CodeGroup>

<h2 id="read-agent-traces">
  Leggi le tracce dell'agente
</h2>

Le tracce ti danno la vista più dettagliata di un'esecuzione dell'agente. Con `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` impostato, ogni passo del ciclo dell'agente diventa uno span che puoi ispezionare nel tuo backend di tracing:

* **`claude_code.interaction`:** avvolge un singolo turno del ciclo dell'agente, dalla ricezione di un prompt alla produzione di una risposta.
* **`claude_code.llm_request`:** avvolge ogni chiamata all'API Claude, con nome del modello, latenza e conteggi dei token come attributi.
* **`claude_code.tool`:** avvolge ogni invocazione dello strumento, con span figlio per l'attesa dei permessi (`claude_code.tool.blocked_on_user`) e l'esecuzione stessa (`claude_code.tool.execution`).
* **`claude_code.hook`:** avvolge ogni esecuzione di [hook](/docs/it/agent-sdk/hooks). Richiede tracing beta dettagliato (`ENABLE_BETA_TRACING_DETAILED=1` e `BETA_TRACING_ENDPOINT`) in aggiunta alle variabili sopra.

Gli span `llm_request`, `tool` e `hook` sono figli dello span `claude_code.interaction` che li racchiude. Quando l'agente genera un subagente attraverso lo strumento Task, gli span `llm_request` e `tool` del subagente si annidano sotto lo span `claude_code.tool` dell'agente padre, quindi l'intera catena di delega appare come una traccia.

Gli span portano un attributo `session.id` per impostazione predefinita. Quando effettui diverse chiamate `query()` contro la stessa [sessione](/docs/it/agent-sdk/sessions), filtra su `session.id` nel tuo backend per vederle come una timeline. L'attributo viene omesso se `OTEL_METRICS_INCLUDE_SESSION_ID` è impostato su un valore falsy.

<Note>
  Il tracing è in beta. I nomi degli span e gli attributi possono cambiare tra le versioni. Vedi
  [Traces (beta)](/docs/it/monitoring-usage#traces-beta) nel riferimento Monitoring
  per le variabili di configurazione dell'esportatore di tracce.
</Note>

<h2 id="link-traces-to-your-application">
  Collega le tracce alla tua applicazione
</h2>

L'SDK propaga automaticamente il contesto di traccia W3C nel subprocess CLI. Quando chiami `query()` mentre uno span OpenTelemetry è attivo nella tua applicazione, l'SDK inietta `TRACEPARENT` e `TRACESTATE` nell'ambiente del processo figlio, e il CLI li legge in modo che il suo span `claude_code.interaction` diventi un figlio del tuo span. L'esecuzione dell'agente appare quindi all'interno della traccia della tua applicazione invece che come una radice disconnessa.

Quando la propagazione del contesto di traccia è abilitata, il CLI inoltra anche `TRACEPARENT` a ogni comando Bash e PowerShell che esegue. Se un comando lanciato attraverso lo strumento Bash emette i suoi stessi span OpenTelemetry, quegli span si annidano sotto lo span `claude_code.tool.execution` che avvolge il comando.

L'iniezione automatica viene saltata quando imposti `TRACEPARENT` esplicitamente in `options.env`, quindi puoi fissare un contesto padre specifico se necessario. Le sessioni CLI interattive ignorano completamente il `TRACEPARENT` in entrata; solo le esecuzioni dell'Agent SDK e `claude -p` lo rispettano. Vedi [Traces (beta)](/docs/it/monitoring-usage#traces-beta) nel riferimento Monitoring per il riferimento completo di span e attributi.

<h2 id="tag-telemetry-from-your-agent">
  Etichetta la telemetria dal tuo agente
</h2>

Per impostazione predefinita, il CLI segnala `service.name` come `claude-code`. Se esegui diversi agenti o esegui l'SDK insieme ad altri servizi che esportano allo stesso collector, sovrascrivi il nome del servizio e aggiungi attributi di risorsa in modo da poter filtrare per agente nel tuo backend.

L'esempio seguente rinomina il servizio e allega metadati di distribuzione. Questi valori vengono applicati come attributi di risorsa OpenTelemetry su ogni span, metrica ed evento che l'agente emette:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_SERVICE_NAME": "support-triage-agent",
          "OTEL_RESOURCE_ATTRIBUTES": "service.version=1.4.0,deployment.environment=production",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_SERVICE_NAME: "support-triage-agent",
      OTEL_RESOURCE_ATTRIBUTES":
        "service.version=1.4.0,deployment.environment=production",
    },
  };
  ```
</CodeGroup>

<h2 id="attribute-actions-to-your-end-users">
  Attribuisci le azioni ai tuoi utenti finali
</h2>

Il CLI allega [attributi di identità](/docs/it/monitoring-usage#standard-attributes) a ogni evento in base alla credenziale che utilizza per chiamare Anthropic. Quando costruisci un'applicazione che serve molti utenti finali da una distribuzione, questi attributi identificano la credenziale del tuo servizio, non l'utente finale per conto del quale l'agente ha agito.

Per rendere le chiamate agli strumenti e l'attività MCP attribuibili agli utenti finali della tua applicazione, inietta l'identità dell'utente finale come attributi di risorsa su ogni chiamata `query()`. Codifica in percentuale i valori prima di interpolarli, poiché `OTEL_RESOURCE_ATTRIBUTES` [riserva virgole, spazi e segni di uguale](/docs/it/monitoring-usage#multi-team-organization-support). L'esempio seguente allega l'utente richiedente e il tenant a ogni span ed evento da una richiesta. Presuppone un oggetto `request` dal tuo framework web che contiene gli ID utente e tenant:

<CodeGroup>
  ```python Python theme={null}
  from urllib.parse import quote

  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_RESOURCE_ATTRIBUTES": f"enduser.id={quote(request.user_id)},tenant.id={quote(request.tenant_id)}",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_RESOURCE_ATTRIBUTES: `enduser.id=${encodeURIComponent(request.userId)},tenant.id=${encodeURIComponent(request.tenantId)}`,
    },
  };
  ```
</CodeGroup>

Con l'identità dell'utente finale allegata, gli eventi `tool_decision`, `tool_result`, `mcp_server_connection` e `permission_mode_changed`, che vengono esportati come record di log denominati con un prefisso `claude_code.`, diventano un audit trail per utente che potete inoltrare a una piattaforma Security Information and Event Management (SIEM). Consultate [Audit security events](/docs/it/monitoring-usage#audit-security-events) nel riferimento Monitoring per l'elenco completo degli eventi rilevanti per la sicurezza e gli attributi che ognuno porta.

<h2 id="control-sensitive-data-in-exports">
  Controlla i dati sensibili nelle esportazioni
</h2>

La telemetria è strutturale per impostazione predefinita. Durate, nomi dei modelli e nomi degli strumenti vengono registrati su ogni span; i conteggi dei token vengono registrati quando la richiesta API sottostante restituisce dati di utilizzo, quindi gli span per richieste non riuscite o interrotte possono ometterli. Il contenuto che il tuo agente legge e scrive non viene registrato per impostazione predefinita. Queste variabili opt-in aggiungono contenuto ai dati esportati:

| Variabile                 | Aggiunge                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OTEL_LOG_USER_PROMPTS=1` | Testo del prompt su eventi `claude_code.user_prompt` e sullo span `claude_code.interaction`                                                                                                                                                                                                                                                                                                                                                                                                                |
| `OTEL_LOG_TOOL_DETAILS=1` | Argomenti di input dello strumento (percorsi di file, comandi shell, pattern di ricerca) su eventi `claude_code.tool_result`                                                                                                                                                                                                                                                                                                                                                                               |
| `OTEL_LOG_TOOL_CONTENT=1` | Corpi di input e output completi dello strumento come eventi span su `claude_code.tool`, troncati a 60 KB. Richiede che il [tracing](#read-agent-traces) sia abilitato                                                                                                                                                                                                                                                                                                                                     |
| `OTEL_LOG_RAW_API_BODIES` | Richiesta e risposta JSON complete dell'API Anthropic Messages come eventi di log `claude_code.api_request_body` e `claude_code.api_response_body`. Imposta su `1` per corpi inline troncati a 60 KB, o `file:<dir>` per corpi non troncati su disco con un percorso `body_ref` nell'evento. I corpi includono l'intera cronologia della conversazione e hanno il contenuto del pensiero esteso redatto. L'abilitazione di questo implica il consenso a tutto ciò che le tre variabili sopra rivelerebbero |

Lascia questi non impostati a meno che la tua pipeline di osservabilità non sia approvata per archiviare i dati che il tuo agente gestisce. Vedi [Security and privacy](/docs/it/monitoring-usage#security-and-privacy) nel riferimento Monitoring per l'elenco completo degli attributi e il comportamento di redazione.

<h2 id="related-documentation">
  Documentazione correlata
</h2>

Queste guide coprono argomenti adiacenti per il monitoraggio e la distribuzione degli agenti:

* [Traccia costo e utilizzo](/docs/it/agent-sdk/cost-tracking): leggi i dati di token e costo dal flusso di messaggi senza un backend esterno.
* [Hosting dell'Agent SDK](/docs/it/agent-sdk/hosting): distribuisci agenti in container dove puoi impostare variabili OpenTelemetry a livello di ambiente.
* [Monitoring](/docs/it/monitoring-usage): il riferimento completo per ogni variabile di ambiente, metrica ed evento che il CLI emette.
