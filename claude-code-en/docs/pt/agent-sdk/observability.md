> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Observabilidade com OpenTelemetry

> Exporte traces, métricas e eventos do Agent SDK para seu backend de observabilidade usando OpenTelemetry.

Quando você executa agentes em produção, você precisa de visibilidade sobre o que eles fizeram:

* quais ferramentas eles chamaram
* quanto tempo cada solicitação de modelo levou
* quantos tokens foram gastos
* onde ocorreram falhas

O Agent SDK pode exportar esses dados como traces, métricas e eventos de log do OpenTelemetry para qualquer backend que aceite o OpenTelemetry Protocol (OTLP), como Honeycomb, Datadog, Grafana, Langfuse ou um coletor auto-hospedado.

Este guia explica como o SDK emite telemetria, como configurar a exportação e como marcar e filtrar os dados uma vez que chegam ao seu backend. Para ler o uso de tokens e custo diretamente do fluxo de resposta do SDK em vez de exportar para um backend, consulte [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking).

<h2 id="how-telemetry-flows-from-the-sdk">
  Como a telemetria flui do SDK
</h2>

O Agent SDK executa o Claude Code CLI como um processo filho e se comunica com ele através de um pipe local. O CLI tem instrumentação OpenTelemetry integrada: ele registra spans em torno de cada solicitação de modelo e execução de ferramenta, emite métricas para contadores de token e custo, e emite eventos de log estruturados para prompts e resultados de ferramentas. O SDK não produz telemetria própria. Em vez disso, ele passa a configuração para o processo CLI, e o CLI exporta diretamente para seu coletor.

A configuração é passada como variáveis de ambiente. Por padrão, o processo filho herda o ambiente da sua aplicação, então você pode configurar a telemetria em um de dois lugares:

* **Ambiente do processo:** defina as variáveis em seu shell, container ou orquestrador antes de sua aplicação iniciar. Cada chamada `query()` as coleta automaticamente sem alteração de código. Esta é a abordagem recomendada para implantações em produção.
* **Opções por chamada:** defina as variáveis em `ClaudeAgentOptions.env` (Python) ou `options.env` (TypeScript). Use isso quando diferentes agentes no mesmo processo precisam de diferentes configurações de telemetria. Em Python, `env` é mesclado no topo do ambiente herdado. Em TypeScript, `env` substitui completamente o ambiente herdado, então inclua `...process.env` no objeto que você passa.

O CLI exporta três sinais OpenTelemetry independentes. Cada um tem seu próprio switch de ativação e seu próprio exportador, então você pode ativar apenas os que precisa.

| Signal     | O que contém                                                                                       | Ativar com                                                          |
| ---------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Metrics    | Contadores para tokens, custo, sessões, linhas de código e decisões de ferramentas                 | `OTEL_METRICS_EXPORTER`                                             |
| Log events | Registros estruturados para cada prompt, solicitação de API, erro de API e resultado de ferramenta | `OTEL_LOGS_EXPORTER`                                                |
| Traces     | Spans para cada interação, solicitação de modelo, chamada de ferramenta e hook (beta)              | `OTEL_TRACES_EXPORTER` mais `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

Para a lista completa de nomes de métricas, nomes de eventos e atributos, consulte a referência de [Monitoring](/docs/pt/monitoring-usage) do Claude Code. O Agent SDK emite os mesmos dados porque executa o mesmo CLI. Os nomes de span estão listados em [Ler traces do agente](#read-agent-traces) abaixo.

<h2 id="enable-telemetry-export">
  Ativar exportação de telemetria
</h2>

A telemetria está desativada até que você defina `CLAUDE_CODE_ENABLE_TELEMETRY=1` e escolha pelo menos um exportador. A configuração mais comum envia todos os três sinais sobre OTLP HTTP para um coletor.

O exemplo a seguir define as variáveis em um dicionário e as passa através de `options.env`. O agente executa uma única tarefa, e o CLI exporta spans, métricas e eventos para o coletor em `collector.example.com` enquanto o loop consome o fluxo de resposta:

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

Como o processo filho herda o ambiente da sua aplicação por padrão, você pode obter o mesmo resultado exportando essas variáveis em um Dockerfile, manifesto Kubernetes ou perfil de shell e omitindo `options.env` inteiramente.

<Note>
  O exportador `console` escreve telemetria na saída padrão, que o SDK usa
  como seu canal de mensagem. Não defina `console` como um valor de exportador ao executar
  através do SDK. Para inspecionar telemetria localmente, aponte
  `OTEL_EXPORTER_OTLP_ENDPOINT` para um coletor local ou um container Jaeger
  all-in-one em vez disso.
</Note>

<h3 id="flush-telemetry-from-short-lived-calls">
  Liberar telemetria de chamadas de curta duração
</h3>

O CLI agrupa telemetria e exporta em um intervalo. Em uma saída limpa do processo, ele tenta liberar dados pendentes, mas a liberação é limitada por um timeout curto, então spans ainda podem ser descartados se o coletor for lento para responder. Se seu processo for morto antes do CLI desligar, qualquer coisa ainda no buffer de lote é perdida. Reduzir os intervalos de exportação reduz ambas as janelas.

Por padrão, métricas exportam a cada 60 segundos e traces e logs exportam a cada 5 segundos. O exemplo a seguir encurta todos os três intervalos para que os dados cheguem ao coletor enquanto uma tarefa curta ainda está em execução:

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
  Ler traces do agente
</h2>

Traces fornecem a visão mais detalhada de uma execução de agente. Com `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` definido, cada etapa do loop do agente se torna um span que você pode inspecionar em seu backend de rastreamento:

* **`claude_code.interaction`:** envolve uma única volta do loop do agente, desde receber um prompt até produzir uma resposta.
* **`claude_code.llm_request`:** envolve cada chamada para a API Claude, com nome do modelo, latência e contagens de tokens como atributos.
* **`claude_code.tool`:** envolve cada invocação de ferramenta, com spans filhos para a espera de permissão (`claude_code.tool.blocked_on_user`) e a execução em si (`claude_code.tool.execution`).
* **`claude_code.hook`:** envolve cada execução de [hook](/docs/pt/agent-sdk/hooks). Requer rastreamento beta detalhado (`ENABLE_BETA_TRACING_DETAILED=1` e `BETA_TRACING_ENDPOINT`) além das variáveis acima.

Os spans `llm_request`, `tool` e `hook` são filhos do span `claude_code.interaction` envolvente. Quando o agente gera um subagente através da ferramenta Task, os spans `llm_request` e `tool` do subagente se aninham sob o span `claude_code.tool` do agente pai, então a cadeia de delegação completa aparece como um trace.

Spans carregam um atributo `session.id` por padrão. Quando você faz várias chamadas `query()` contra a mesma [sessão](/docs/pt/agent-sdk/sessions), filtre em `session.id` em seu backend para vê-las como uma linha do tempo. O atributo é omitido se `OTEL_METRICS_INCLUDE_SESSION_ID` for definido como um valor falso.

<Note>
  O rastreamento está em beta. Os nomes de span e atributos podem mudar entre versões. Consulte
  [Traces (beta)](/docs/pt/monitoring-usage#traces-beta) na referência de Monitoring
  para as variáveis de configuração do exportador de trace.
</Note>

<h2 id="link-traces-to-your-application">
  Vincular traces à sua aplicação
</h2>

O SDK propaga automaticamente o contexto de trace W3C para o subprocesso CLI. Quando você chama `query()` enquanto um span OpenTelemetry está ativo em sua aplicação, o SDK injeta `TRACEPARENT` e `TRACESTATE` no ambiente do processo filho, e o CLI os lê para que seu span `claude_code.interaction` se torne um filho do seu span. A execução do agente então aparece dentro do trace da sua aplicação em vez de como uma raiz desconectada.

Quando a propagação de contexto de trace está ativada, o CLI também encaminha `TRACEPARENT` para cada comando Bash e PowerShell que executa. Se um comando lançado através da ferramenta Bash emite seus próprios spans OpenTelemetry, esses spans se aninham sob o span `claude_code.tool.execution` que envolve o comando.

A injeção automática é ignorada quando você define `TRACEPARENT` explicitamente em `options.env`, então você pode fixar um contexto pai específico se necessário. As sessões interativas do CLI ignoram completamente `TRACEPARENT` de entrada; apenas execuções do Agent SDK e `claude -p` a honram. Consulte [Traces (beta)](/docs/pt/monitoring-usage#traces-beta) na referência de Monitoring para a referência completa de span e atributo.

<h2 id="tag-telemetry-from-your-agent">
  Marcar telemetria do seu agente
</h2>

Por padrão, o CLI relata `service.name` como `claude-code`. Se você executar vários agentes ou executar o SDK junto com outros serviços que exportam para o mesmo coletor, substitua o nome do serviço e adicione atributos de recurso para que você possa filtrar por agente em seu backend.

O exemplo a seguir renomeia o serviço e anexa metadados de implantação. Esses valores são aplicados como atributos de recurso OpenTelemetry em cada span, métrica e evento que o agente emite:

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
  Atribuir ações aos seus usuários finais
</h2>

O CLI anexa [atributos de identidade](/docs/pt/monitoring-usage#standard-attributes) a cada evento com base na credencial que usa para chamar Anthropic. Quando você constrói uma aplicação que serve muitos usuários finais a partir de uma implantação, esses atributos identificam a credencial do seu serviço, não o usuário final em nome de quem o agente agiu.

Para tornar as chamadas de ferramentas e atividade MCP atribuíveis aos usuários finais da sua aplicação, injete identidade de usuário final como atributos de recurso em cada chamada `query()`. Codifique valores em percentual antes de interpolá-los, já que `OTEL_RESOURCE_ATTRIBUTES` [reserva vírgulas, espaços e sinais de igual](/docs/pt/monitoring-usage#multi-team-organization-support). O exemplo a seguir anexa o usuário solicitante e o tenant a cada span e evento de uma solicitação. Ele assume um objeto `request` do seu framework web contendo os IDs do usuário e do tenant:

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

Com identidade de usuário final anexada, os eventos `tool_decision`, `tool_result`, `mcp_server_connection` e `permission_mode_changed`, que são exportados como registros de log nomeados com um prefixo `claude_code.`, se tornam uma trilha de auditoria por usuário que você pode encaminhar para uma plataforma de Security Information and Event Management (SIEM). Consulte [Auditar eventos de segurança](/docs/pt/monitoring-usage#audit-security-events) na referência de Monitoring para a lista completa de eventos relevantes para segurança e os atributos que cada um carrega.

<h2 id="control-sensitive-data-in-exports">
  Controlar dados sensíveis em exportações
</h2>

A telemetria é estrutural por padrão. Durações, nomes de modelos e nomes de ferramentas são registrados em cada span; contagens de tokens são registradas quando a solicitação de API subjacente retorna dados de uso, então spans para solicitações falhadas ou abortadas podem omiti-los. O conteúdo que seu agente lê e escreve não é registrado por padrão. Essas variáveis opt-in adicionam conteúdo aos dados exportados:

| Variable                  | Adiciona                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OTEL_LOG_USER_PROMPTS=1` | Texto de prompt em eventos `claude_code.user_prompt` e no span `claude_code.interaction`                                                                                                                                                                                                                                                                                                                                                                                                  |
| `OTEL_LOG_TOOL_DETAILS=1` | Argumentos de entrada de ferramenta (caminhos de arquivo, comandos de shell, padrões de pesquisa) em eventos `claude_code.tool_result`                                                                                                                                                                                                                                                                                                                                                    |
| `OTEL_LOG_TOOL_CONTENT=1` | Corpos completos de entrada e saída de ferramenta como eventos de span em `claude_code.tool`, truncados em 60 KB. Requer que [rastreamento](#read-agent-traces) esteja ativado                                                                                                                                                                                                                                                                                                            |
| `OTEL_LOG_RAW_API_BODIES` | JSON completo de solicitação e resposta da API Anthropic Messages como eventos de log `claude_code.api_request_body` e `claude_code.api_response_body`. Defina como `1` para corpos inline truncados em 60 KB, ou `file:<dir>` para corpos não truncados em disco com um caminho `body_ref` no evento. Os corpos incluem todo o histórico de conversa e têm conteúdo de pensamento estendido redatado. Ativar isso implica consentimento para tudo que as três variáveis acima revelariam |

Deixe essas não definidas a menos que seu pipeline de observabilidade seja aprovado para armazenar os dados que seu agente manipula. Consulte [Segurança e privacidade](/docs/pt/monitoring-usage#security-and-privacy) na referência de Monitoring para a lista completa de atributos e comportamento de redação.

<h2 id="related-documentation">
  Documentação relacionada
</h2>

Estes guias cobrem tópicos adjacentes para monitoramento e implantação de agentes:

* [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking): leia dados de token e custo do fluxo de mensagem sem um backend externo.
* [Hospedando o Agent SDK](/docs/pt/agent-sdk/hosting): implante agentes em containers onde você pode definir variáveis OpenTelemetry no nível do ambiente.
* [Monitoring](/docs/pt/monitoring-usage): a referência completa para cada variável de ambiente, métrica e evento que o CLI emite.
