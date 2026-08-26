> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Monitoramento

> Saiba como ativar e configurar OpenTelemetry para Claude Code.

Rastreie o uso, custos e atividade de ferramentas do Claude Code em toda a sua organização exportando dados de telemetria através do OpenTelemetry (OTel). Claude Code exporta métricas como dados de série temporal via protocolo de métricas padrão, eventos via protocolo de logs/eventos e, opcionalmente, rastreamentos distribuídos via [protocolo de rastreamentos](#traces-beta). Configure seus backends de métricas, logs e rastreamentos para corresponder aos seus requisitos de monitoramento.

<h2 id="quick-start">
  Início rápido
</h2>

Configure OpenTelemetry usando variáveis de ambiente:

```bash theme={null}
# 1. Ativar telemetria
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. Escolher exportadores (ambos são opcionais - configure apenas o que você precisa)
export OTEL_METRICS_EXPORTER=otlp       # Opções: otlp, prometheus, console, none
export OTEL_LOGS_EXPORTER=otlp          # Opções: otlp, console, none

# 3. Configurar endpoint OTLP (para exportador OTLP)
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. Definir autenticação (se necessário)
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. Para depuração: reduzir intervalos de exportação
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 segundos (padrão: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 segundos (padrão: 5000ms)

# 6. Executar Claude Code
claude
```

<Note>
  Os intervalos de exportação padrão são 60 segundos para métricas e 5 segundos para logs. Durante a configuração, você pode querer usar intervalos mais curtos para fins de depuração. Lembre-se de redefinir esses valores para uso em produção.
</Note>

Para opções de configuração completas, consulte a [especificação OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Configuração do administrador
</h2>

Os administradores podem configurar as definições de OpenTelemetry para todos os usuários através do [arquivo de configurações gerenciadas](/docs/pt/settings#settings-files). Isso permite controle centralizado das configurações de telemetria em toda a organização. Consulte a [precedência de configurações](/docs/pt/settings#settings-precedence) para obter mais informações sobre como as configurações são aplicadas.

Exemplo de configuração de configurações gerenciadas:

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
  As configurações gerenciadas podem ser distribuídas via MDM (Mobile Device Management) ou outras soluções de gerenciamento de dispositivos. As variáveis de ambiente definidas no arquivo de configurações gerenciadas têm alta precedência e não podem ser substituídas pelos usuários.
</Note>

Claude Code não passa variáveis de ambiente `OTEL_*` para os subprocessos que ele gera, incluindo a ferramenta Bash, hooks, servidores MCP e servidores de linguagem. Um aplicativo instrumentado com OpenTelemetry que você executa através da ferramenta Bash não herda o endpoint do exportador ou cabeçalhos do Claude Code, então defina essas variáveis diretamente no comando se esse aplicativo precisar exportar sua própria telemetria.

<h2 id="configuration-details">
  Detalhes de configuração
</h2>

<h3 id="common-configuration-variables">
  Variáveis de configuração comuns
</h3>

| Variável de Ambiente                                | Descrição                                                                                                                                                                                                                                                                                                                                                 | Valores de Exemplo                                                                                                                 |
| --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Ativa coleta de telemetria (obrigatório)                                                                                                                                                                                                                                                                                                                  | `1`                                                                                                                                |
| `OTEL_METRICS_EXPORTER`                             | Tipos de exportador de métricas, separados por vírgula. Use `none` para desativar                                                                                                                                                                                                                                                                         | `console`, `otlp`, `prometheus`, `none`                                                                                            |
| `OTEL_LOGS_EXPORTER`                                | Tipos de exportador de logs/eventos, separados por vírgula. Use `none` para desativar                                                                                                                                                                                                                                                                     | `console`, `otlp`, `none`                                                                                                          |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protocolo para exportador OTLP, aplica-se a todos os sinais                                                                                                                                                                                                                                                                                               | `grpc`, `http/json`, `http/protobuf`                                                                                               |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Endpoint do coletor OTLP para todos os sinais                                                                                                                                                                                                                                                                                                             | `http://localhost:4317`                                                                                                            |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protocolo para métricas, substitui configuração geral                                                                                                                                                                                                                                                                                                     | `grpc`, `http/json`, `http/protobuf`                                                                                               |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Endpoint de métricas OTLP, substitui configuração geral                                                                                                                                                                                                                                                                                                   | `http://localhost:4318/v1/metrics`                                                                                                 |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protocolo para logs, substitui configuração geral                                                                                                                                                                                                                                                                                                         | `grpc`, `http/json`, `http/protobuf`                                                                                               |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Endpoint de logs OTLP, substitui configuração geral                                                                                                                                                                                                                                                                                                       | `http://localhost:4318/v1/logs`                                                                                                    |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Cabeçalhos de autenticação para OTLP                                                                                                                                                                                                                                                                                                                      | `Authorization=Bearer token`                                                                                                       |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Intervalo de exportação em milissegundos (padrão: 60000)                                                                                                                                                                                                                                                                                                  | `5000`, `60000`                                                                                                                    |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Intervalo de exportação de logs em milissegundos (padrão: 5000)                                                                                                                                                                                                                                                                                           | `1000`, `10000`                                                                                                                    |
| `OTEL_LOG_USER_PROMPTS`                             | Ativar registro de conteúdo de prompt do usuário (padrão: desativado)                                                                                                                                                                                                                                                                                     | `1` para ativar                                                                                                                    |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Ativar registro de texto de resposta do assistente em eventos `assistant_response` (padrão: desativado). Quando não definido, volta para o valor de `OTEL_LOG_USER_PROMPTS`. Requer Claude Code v2.1.193 ou posterior                                                                                                                                     | `1` para ativar, `0` para manter reduzido                                                                                          |
| `OTEL_LOG_TOOL_DETAILS`                             | Ativar registro de parâmetros de ferramenta e argumentos de entrada em eventos de ferramenta e atributos de span de rastreamento: comandos Bash, nomes de servidor MCP e ferramenta, nomes de skill e entrada de ferramenta. Também ativa nomes de comando customizado, plugin e MCP em eventos `user_prompt` (padrão: desativado)                        | `1` para ativar                                                                                                                    |
| `OTEL_LOG_TOOL_CONTENT`                             | Ativar registro de conteúdo de entrada e saída de ferramenta em eventos de span (padrão: desativado). Requer [rastreamento](#traces-beta). O conteúdo é truncado em 60 KB                                                                                                                                                                                 | `1` para ativar                                                                                                                    |
| `OTEL_LOG_RAW_API_BODIES`                           | Emitir o corpo JSON completo da solicitação e resposta da API Anthropic Messages como eventos de log `api_request_body` / `api_response_body` (padrão: desativado). Os corpos incluem todo o histórico de conversa. Ativar isso implica consentimento para tudo que `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS` e `OTEL_LOG_TOOL_CONTENT` revelariam | `1` para corpos inline truncados em 60 KB, ou `file:<dir>` para corpos não truncados em disco com um ponteiro `body_ref` no evento |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Preferência de temporalidade de métricas (padrão: `delta`). Defina como `cumulative` se seu backend espera temporalidade cumulativa                                                                                                                                                                                                                       | `delta`, `cumulative`                                                                                                              |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervalo para atualizar cabeçalhos dinâmicos (padrão: 1740000ms / 29 minutos)                                                                                                                                                                                                                                                                            | `900000`                                                                                                                           |

<h3 id="mtls-authentication">
  Autenticação mTLS
</h3>

Como você configura certificados de cliente para o exportador OTLP depende do protocolo OTLP em uso para esse sinal, definido via `OTEL_EXPORTER_OTLP_PROTOCOL` ou a substituição por sinal. A mesma configuração se aplica a métricas, logs e rastreamentos.

| Protocolo                    | Variáveis de certificado do cliente                                                                                                                                                            | Confiar na CA do coletor com     |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY` e opcionalmente `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Veja [Configuração de rede](/docs/pt/network-config#mtls-authentication)                   | `NODE_EXTRA_CA_CERTS`            |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` e `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, ou as variantes por sinal como `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` para usar um certificado diferente por sinal | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

Para `grpc`, o SDK OpenTelemetry lê as variáveis OTLP padrão diretamente, então as configurações existentes que definem as variáveis de métricas por sinal continuam funcionando.

<h3 id="metrics-cardinality-control">
  Controle de cardinalidade de métricas
</h3>

As seguintes variáveis de ambiente controlam quais atributos são incluídos nas métricas para gerenciar a cardinalidade:

| Variável de Ambiente                       | Descrição                                                                                 | Valor Padrão | Exemplo para Desativar |
| ------------------------------------------ | ----------------------------------------------------------------------------------------- | ------------ | ---------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Incluir atributo session.id em métricas                                                   | `true`       | `false`                |
| `OTEL_METRICS_INCLUDE_VERSION`             | Incluir atributo app.version em métricas                                                  | `false`      | `true`                 |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Incluir atributos user.account\_uuid e user.account\_id em métricas                       | `true`       | `false`                |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Incluir atributo app.entrypoint em métricas                                               | `false`      | `true`                 |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Incluir chaves de `OTEL_RESOURCE_ATTRIBUTES` como atributos em pontos de dados de métrica | `true`       | `false`                |

Essas variáveis ajudam a controlar a cardinalidade das métricas, o que afeta os requisitos de armazenamento e o desempenho de consultas no seu backend de métricas. Cardinalidade mais baixa geralmente significa melhor desempenho e custos de armazenamento mais baixos, mas dados menos granulares para análise.

<h3 id="traces-beta">
  Rastreamentos (beta)
</h3>

O rastreamento distribuído exporta spans que vinculam cada prompt do usuário às solicitações de API e execuções de ferramentas que ele dispara, para que você possa visualizar uma solicitação completa como um único rastreamento no seu backend de rastreamento.

O rastreamento está desativado por padrão. Para ativá-lo, defina tanto `CLAUDE_CODE_ENABLE_TELEMETRY=1` quanto `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, depois defina `OTEL_TRACES_EXPORTER` para escolher para onde os spans são enviados. Os rastreamentos reutilizam a [configuração OTLP comum](#common-configuration-variables) para endpoint, protocolo, cabeçalhos e [mTLS](#mtls-authentication).

| Variável de Ambiente                  | Descrição                                                                                   | Valores de Exemplo                   |
| ------------------------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Ativar rastreamento de span (obrigatório). `ENABLE_ENHANCED_TELEMETRY_BETA` também é aceito | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Tipos de exportador de rastreamentos, separados por vírgula. Use `none` para desativar      | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protocolo para rastreamentos, substitui `OTEL_EXPORTER_OTLP_PROTOCOL`                       | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Endpoint de rastreamentos OTLP, substitui `OTEL_EXPORTER_OTLP_ENDPOINT`                     | `http://localhost:4318/v1/traces`    |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Intervalo de exportação de lote de span em milissegundos (padrão: 5000)                     | `1000`, `10000`                      |

Os spans reduzem o texto do prompt do usuário, detalhes de entrada de ferramenta e conteúdo de ferramenta por padrão. Defina `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1` e `OTEL_LOG_TOOL_CONTENT=1` para incluí-los.

Quando o rastreamento está ativo, subprocessos Bash e PowerShell herdam automaticamente uma variável de ambiente `TRACEPARENT` contendo o contexto de rastreamento W3C do span de execução de ferramenta ativo. Isso permite que qualquer subprocesso que leia `TRACEPARENT` coloque seus próprios spans sob o mesmo rastreamento, permitindo rastreamento distribuído de ponta a ponta através de scripts e comandos que Claude executa.

Quando o rastreamento está ativo e Claude Code está conectado diretamente à API Anthropic, cada solicitação de modelo carrega um cabeçalho W3C `traceparent` definido para o contexto do span `claude_code.llm_request`, e o cabeçalho `traceresponse` da API é registrado como um link de span. Juntos, esses conectam os spans do lado do cliente do Claude Code ao rastreamento do lado do servidor através de qualquer intermediário compatível. As solicitações HTTP MCP de saída carregam `traceparent` da mesma forma. O cabeçalho não é enviado para provedores terceirizados.

Por padrão, o cabeçalho `traceparent` em solicitações de modelo e HTTP MCP é enviado apenas quando `ANTHROPIC_BASE_URL` não está definido ou aponta para a API Anthropic, já que alguns proxies rejeitam cabeçalhos não reconhecidos. A variável `TRACEPARENT` do subprocesso é controlada pelo mesmo switch para consistência. Se você executar Claude Code através de um proxy `ANTHROPIC_BASE_URL` customizado e quiser que o contexto de rastreamento seja propagado, defina `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

No Agent SDK e sessões não-interativas iniciadas com `-p`, Claude Code também lê `TRACEPARENT` e `TRACESTATE` de seu próprio ambiente ao iniciar cada span de interação. Isso permite que um processo de incorporação passe seu contexto de rastreamento W3C ativo para o subprocesso para que os spans do Claude Code apareçam como filhos do rastreamento distribuído do chamador. Sessões interativas ignoram `TRACEPARENT` de entrada para evitar herdar acidentalmente valores ambientes de CI ou ambientes de contêiner.

<h4 id="span-hierarchy">
  Hierarquia de span
</h4>

Cada prompt do usuário inicia um span raiz `claude_code.interaction`. Chamadas de API, chamadas de ferramenta e execuções de hook são registradas como seus filhos. Os spans de ferramenta têm dois spans filhos próprios: um para o tempo gasto esperando uma decisão de permissão e outro para a execução em si. Quando a ferramenta Agent ou a ferramenta Task legada gera um subagente, os spans de API e ferramenta do subagente se aninham sob o span `claude_code.tool` do pai.

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (requer rastreamento beta detalhado)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (ferramenta Agent) spans claude_code.llm_request / claude_code.tool do subagente
```

No Agent SDK e sessões `claude -p`, `claude_code.interaction` em si se torna um filho do span do chamador quando `TRACEPARENT` está definido no ambiente.

<h4 id="span-attributes">
  Atributos de span
</h4>

Cada span carrega os [atributos padrão](#standard-attributes) mais um atributo `span.type` correspondendo ao seu nome. As tabelas abaixo listam os atributos adicionais definidos em cada span. Os spans `llm_request`, `tool.execution` e `hook` definem status OpenTelemetry `ERROR` quando registram uma falha; os outros spans sempre terminam com status `UNSET`.

**`claude_code.interaction`**

| Atributo                  | Descrição                                                                  | Controlado Por          |
| ------------------------- | -------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Texto do prompt. O valor é `<REDACTED>` a menos que o gate esteja definido | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Comprimento do prompt em caracteres                                        |                         |
| `interaction.sequence`    | Contador baseado em 1 de interações nesta sessão                           |                         |
| `interaction.duration_ms` | Duração de parede do turno                                                 |                         |

**`claude_code.llm_request`**

| Atributo                         | Descrição                                                                                                                                                  | Controlado Por          |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `model`                          | Identificador do modelo                                                                                                                                    |                         |
| `gen_ai.system`                  | Sempre `anthropic`. Convenção semântica GenAI OpenTelemetry                                                                                                |                         |
| `gen_ai.request.model`           | Mesmo valor que `model`. Convenção semântica GenAI OpenTelemetry                                                                                           |                         |
| `query_source`                   | Subsistema que emitiu a solicitação, como `repl_main_thread` ou um nome de subagente                                                                       |                         |
| `agent_id`                       | Identificador do subagente ou colega que emitiu a solicitação. Ausente na sessão principal                                                                 |                         |
| `parent_agent_id`                | Identificador do agente que gerou este. Ausente para a sessão principal e para agentes gerados diretamente a partir dela                                   |                         |
| `workflow.run_id`                | Identificador de execução da ferramenta [Workflow](/docs/pt/workflows) que gerou este agente, prefixado `wf_`. Ausente para agentes não gerados por um workflow |                         |
| `workflow.name`                  | Nome do workflow que gerou este agente. Nomes criados pelo usuário são substituídos por `custom` a menos que o gate esteja definido                        | `OTEL_LOG_TOOL_DETAILS` |
| `speed`                          | `fast` ou `normal`                                                                                                                                         |                         |
| `llm_request.context`            | `interaction`, `tool` ou `standalone` dependendo do span pai                                                                                               |                         |
| `duration_ms`                    | Duração de parede incluindo tentativas                                                                                                                     |                         |
| `ttft_ms`                        | Tempo até o primeiro token em milissegundos                                                                                                                |                         |
| `input_tokens`                   | Contagem de tokens de entrada do bloco de uso da API                                                                                                       |                         |
| `output_tokens`                  | Contagem de tokens de saída                                                                                                                                |                         |
| `cache_read_tokens`              | Tokens lidos do cache de prompt                                                                                                                            |                         |
| `cache_creation_tokens`          | Tokens escritos no cache de prompt                                                                                                                         |                         |
| `request_id`                     | ID de solicitação da API Anthropic do cabeçalho de resposta `request-id`                                                                                   |                         |
| `gen_ai.response.id`             | Mesmo valor que `request_id`. Convenção semântica GenAI OpenTelemetry                                                                                      |                         |
| `client_request_id`              | `x-client-request-id` gerado pelo cliente da tentativa final                                                                                               |                         |
| `attempt`                        | Total de tentativas feitas para esta solicitação                                                                                                           |                         |
| `success`                        | `true` ou `false`                                                                                                                                          |                         |
| `status_code`                    | Código de status HTTP quando a solicitação falhou                                                                                                          |                         |
| `error`                          | Mensagem de erro quando a solicitação falhou                                                                                                               |                         |
| `response.has_tool_call`         | `true` quando a resposta continha blocos de uso de ferramenta                                                                                              |                         |
| `stop_reason`                    | API response `stop_reason`, como `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn` ou `refusal`                                          |                         |
| `gen_ai.response.finish_reasons` | Mesmo valor que `stop_reason`, envolvido em um array de string. Convenção semântica GenAI OpenTelemetry                                                    |                         |

Cada tentativa de repetição também é registrada como um evento de span `gen_ai.request.attempt` com atributos `attempt` e `client_request_id`.

**`claude_code.tool`**

| Atributo              | Descrição                                                                                                                                                                                                                                          | Controlado Por          |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | Nome da ferramenta                                                                                                                                                                                                                                 |                         |
| `duration_ms`         | Duração de parede incluindo espera de permissão e execução                                                                                                                                                                                         |                         |
| `result_tokens`       | Tamanho aproximado em tokens do resultado da ferramenta                                                                                                                                                                                            |                         |
| `agent_id`            | Identificador do subagente ou colega que executou a ferramenta. Ausente na sessão principal                                                                                                                                                        |                         |
| `parent_agent_id`     | Identificador do agente que gerou este. Ausente para a sessão principal e para agentes gerados diretamente a partir dela                                                                                                                           |                         |
| `workflow.run_id`     | Identificador de execução da ferramenta Workflow que gerou este agente, prefixado `wf_`. Ausente para agentes não gerados por um workflow                                                                                                          |                         |
| `workflow.name`       | Nome do workflow que gerou este agente. Nomes criados pelo usuário são substituídos por `custom` a menos que o gate esteja definido                                                                                                                | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | O id do bloco `tool_use` do modelo para esta chamada. Corresponde ao `tool_use_id` nos eventos [tool\_result](#tool-result-event) e [tool\_decision](#tool-decision-event) e nas cargas de hook, para que você possa unir o span a esses registros |                         |
| `gen_ai.tool.call.id` | Mesmo valor que `tool_use_id`. Convenção semântica GenAI OpenTelemetry                                                                                                                                                                             |                         |
| `file_path`           | Caminho de arquivo alvo para ferramentas Read, Edit e Write                                                                                                                                                                                        | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | String de comando para a ferramenta Bash                                                                                                                                                                                                           | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nome da skill para a ferramenta Skill                                                                                                                                                                                                              | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Tipo de subagente para a ferramenta Agent ou ferramenta Task legada                                                                                                                                                                                | `OTEL_LOG_TOOL_DETAILS` |

Quando `OTEL_LOG_TOOL_CONTENT=1`, este span também registra um evento de span `tool.output` cujos atributos contêm os corpos de entrada e saída da ferramenta, truncados em 60 KB por atributo.

**`claude_code.tool.blocked_on_user`**

| Atributo      | Descrição                                                                                   | Controlado Por |
| ------------- | ------------------------------------------------------------------------------------------- | -------------- |
| `duration_ms` | Tempo gasto esperando a decisão de permissão                                                |                |
| `decision`    | `accept` ou `reject`                                                                        |                |
| `source`      | Fonte de decisão, correspondendo ao [Evento de decisão da ferramenta](#tool-decision-event) |                |

**`claude_code.tool.execution`**

| Atributo              | Descrição                                                                                                                                                              | Controlado Por          |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Tempo gasto executando o corpo da ferramenta                                                                                                                           |                         |
| `tool_use_id`         | Mesmo valor que no span pai `claude_code.tool`                                                                                                                         |                         |
| `gen_ai.tool.call.id` | Mesmo valor que `tool_use_id`. Convenção semântica GenAI OpenTelemetry                                                                                                 |                         |
| `success`             | `true` ou `false`                                                                                                                                                      |                         |
| `error`               | String de categoria de erro quando a execução falhou, como `Error:ENOENT` ou `ShellError`. Contém a mensagem de erro completa em vez disso quando o gate está definido | `OTEL_LOG_TOOL_DETAILS` |

**`claude_code.hook`**

Este span é emitido apenas quando rastreamento beta detalhado está ativo, o que requer `ENABLE_BETA_TRACING_DETAILED=1` e `BETA_TRACING_ENDPOINT` além da configuração do exportador de rastreamento acima. Em sessões CLI interativas, isso também requer que sua organização esteja na lista de permissões para o recurso. Sessões Agent SDK e não-interativas `-p` não são controladas. Não é emitido quando apenas `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` está definido.

| Atributo                 | Descrição                                                | Controlado Por          |
| ------------------------ | -------------------------------------------------------- | ----------------------- |
| `hook_event`             | Tipo de evento de hook, como `PreToolUse`                |                         |
| `hook_name`              | Nome completo do hook, como `PreToolUse:Write`           |                         |
| `num_hooks`              | Número de comandos de hook correspondentes executados    |                         |
| `hook_definitions`       | Configuração de hook serializada em JSON                 | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms`            | Duração de parede de todos os hooks correspondentes      |                         |
| `num_success`            | Contagem de hooks que completaram com sucesso            |                         |
| `num_blocking`           | Contagem de hooks que retornaram uma decisão de bloqueio |                         |
| `num_non_blocking_error` | Contagem de hooks que falharam sem bloquear              |                         |
| `num_cancelled`          | Contagem de hooks cancelados antes da conclusão          |                         |

<Note>
  Atributos adicionais que contêm conteúdo, como `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input` e `response.model_output`, são emitidos apenas quando rastreamento beta detalhado está ativo. Eles não fazem parte do esquema de span estável. `user_system_prompt` também requer `OTEL_LOG_USER_PROMPTS=1`. Ele carrega apenas o texto do prompt do sistema que você fornece através da opção SDK `systemPrompt` ou dos sinalizadores `--system-prompt` e `--append-system-prompt`, truncado em 60 KB, e é emitido uma vez por sessão em vez de por solicitação.
</Note>

<h3 id="dynamic-headers">
  Cabeçalhos dinâmicos
</h3>

Para ambientes corporativos que exigem autenticação dinâmica, você pode configurar um script para gerar cabeçalhos dinamicamente. Cabeçalhos dinâmicos se aplicam apenas aos protocolos `http/protobuf` e `http/json`. O exportador `grpc` usa apenas o valor estático `OTEL_EXPORTER_OTLP_HEADERS`.

<h4 id="settings-configuration">
  Configuração de configurações
</h4>

Adicione ao seu `.claude/settings.json`:

```json theme={null}
{
  "otelHeadersHelper": "/bin/generate_opentelemetry_headers.sh"
}
```

O valor pode ser o caminho para um arquivo executável, incluindo um caminho que contém espaços, ou uma linha de comando shell com argumentos. No Windows, o valor sempre é executado através do shell, então coloque entre aspas um caminho que contém espaços dentro do valor JSON.

<h4 id="script-requirements">
  Requisitos do script
</h4>

O script deve gerar JSON válido com pares de chave-valor de string representando cabeçalhos HTTP:

```bash theme={null}
#!/bin/bash
# Exemplo: Múltiplos cabeçalhos
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

Se o auxiliar falhar ou imprimir saída que não atenda a esses requisitos, Claude Code relata o erro em:

* Saída de `/status`
* O log de depuração, ao executar com [`--debug`](/docs/pt/cli-reference#cli-flags) ou após executar `/debug` na sessão
* stderr, em sessões não-interativas iniciadas com `-p`

<h4 id="refresh-behavior">
  Comportamento de atualização
</h4>

O script auxiliar de cabeçalhos é executado na inicialização e periodicamente depois para suportar atualização de token. Por padrão, o script é executado a cada 29 minutos. Personalize o intervalo com a variável de ambiente `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`.

<h3 id="multi-team-organization-support">
  Suporte a organizações multi-equipe
</h3>

Organizações com múltiplas equipes ou departamentos podem adicionar atributos personalizados para distinguir entre diferentes grupos usando a variável de ambiente `OTEL_RESOURCE_ATTRIBUTES`:

```bash theme={null}
# Adicionar atributos personalizados para identificação de equipe
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

Esses atributos personalizados serão incluídos em todas as métricas e eventos, permitindo que você:

* Filtre métricas por equipe ou departamento
* Rastreie custos por centro de custo
* Crie dashboards específicos de equipe
* Configure alertas para equipes específicas

Claude Code anexa esses valores como atributos em cada ponto de dados de métrica e registro de evento, além de enviá-los no bloco de recurso OTLP. Como a maioria dos backends de métricas expõe atributos de ponto de dados como rótulos consultáveis, você pode agrupar e filtrar métricas por suas chaves personalizadas diretamente. Chaves personalizadas nunca substituem os [atributos padrão](#standard-attributes) como `user.id` ou `session.id`: quando uma chave colide, Claude Code mantém o valor integrado.

Cada chave personalizada se torna um rótulo em cada série de métrica, então valores de alta cardinalidade aumentam o custo de armazenamento no seu backend de métricas. Para enviar atributos personalizados apenas no bloco de recurso e omiti-los dos rótulos de ponto de dados, defina `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false`. Veja [Controle de cardinalidade de métricas](#metrics-cardinality-control).

<Warning>
  A variável de ambiente `OTEL_RESOURCE_ATTRIBUTES` usa pares chave=valor separados por vírgula com requisitos rigorosos de formatação:

  * **Sem espaços permitidos**: os valores não podem conter espaços. Por exemplo, `user.organizationName=My Company` é inválido
  * **Formato**: deve ser pares chave=valor separados por vírgula: `key1=value1,key2=value2`
  * **Caracteres permitidos**: apenas caracteres US-ASCII excluindo caracteres de controle, espaços em branco, aspas duplas, vírgulas, ponto-e-vírgula e barras invertidas
  * **Caracteres especiais**: caracteres fora do intervalo permitido devem ser codificados em percentual

  Para um valor que precisaria de um espaço, use sublinhados ou camelCase em vez disso. Os exemplos a seguir definem `org.name` com cada forma:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
  export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
  ```

  Você pode codificar em percentual qualquer caractere, não apenas os excluídos. Este exemplo codifica tanto o espaço quanto o apóstrofo:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
  ```

  Envolver valores em aspas não escapa espaços. Por exemplo, `org.name="My Company"` resulta no valor literal `"My Company"` com as aspas incluídas, não `My Company`.
</Warning>

<h3 id="example-configurations">
  Configurações de exemplo
</h3>

Defina essas variáveis de ambiente antes de executar `claude`. Cada bloco mostra uma configuração completa para um exportador diferente ou cenário de implantação:

```bash theme={null}
# Depuração de console (intervalos de 1 segundo)
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

# Múltiplos exportadores
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json

# Diferentes endpoints/backends para métricas e logs
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317

# Apenas métricas (sem eventos/logs)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Apenas eventos/logs (sem métricas)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  Métricas e eventos disponíveis
</h2>

<h3 id="standard-attributes">
  Atributos padrão
</h3>

Todas as métricas e eventos compartilham esses atributos padrão:

| Atributo                             | Descrição                                                                                                                                                                                                                                       | Controlado Por                                            |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| `session.id`                         | Identificador de sessão único                                                                                                                                                                                                                   | `OTEL_METRICS_INCLUDE_SESSION_ID` (padrão: true)          |
| `app.version`                        | Versão atual do Claude Code                                                                                                                                                                                                                     | `OTEL_METRICS_INCLUDE_VERSION` (padrão: false)            |
| `app.entrypoint`                     | Como a sessão foi iniciada, como `cli`, `sdk-cli`, `sdk-ts`, `sdk-py` ou `claude-vscode`                                                                                                                                                        | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (padrão: false)         |
| `organization.id`                    | UUID da organização (quando autenticado)                                                                                                                                                                                                        | Sempre incluído quando disponível                         |
| `user.account_uuid`                  | UUID da conta (quando autenticado)                                                                                                                                                                                                              | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (padrão: true)        |
| `user.account_id`                    | ID da conta em formato marcado correspondendo às APIs de administrador Anthropic (quando autenticado), como `user_01BWBeN28...`                                                                                                                 | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (padrão: true)        |
| `user.id`                            | Identificador anônimo aleatório gerado na primeira execução e persistido em `~/.claude.json`. Não contém informações pessoais e não é derivado da sua conta Claude. Deletar o arquivo produz um novo valor não relacionado na próxima execução. | Sempre incluído                                           |
| `user.email`                         | Endereço de email do usuário (quando autenticado via OAuth)                                                                                                                                                                                     | Sempre incluído quando disponível                         |
| `terminal.type`                      | Tipo de terminal, como `iTerm.app`, `vscode`, `cursor` ou `tmux`                                                                                                                                                                                | Sempre incluído quando detectado                          |
| Chaves de `OTEL_RESOURCE_ATTRIBUTES` | Atributos personalizados que você define, como `department` ou `team.id`. Veja [Suporte a organização multi-equipe](#multi-team-organization-support)                                                                                           | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (padrão: true) |

Quando Claude Code está conectado a um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), a CLI marca as exportações com a identidade autenticada da sessão do gateway: `user.id` é o assunto do IdP em vez de um identificador de instalação anônimo, `user.email` é o email conectado e `user.groups` carrega a associação de grupo do IdP como uma string separada por vírgula. Cada exportação também carrega `identity.source: gateway-oidc`. A identidade do gateway é aplicada por último, então as chaves `user.*` e `identity.*` definidas através de `OTEL_RESOURCE_ATTRIBUTES` são ignoradas em sessões de gateway.

Os eventos incluem adicionalmente os seguintes atributos. Estes nunca são anexados a métricas porque causariam cardinalidade ilimitada:

* `prompt.id`: UUID correlacionando um prompt do usuário com todos os eventos subsequentes até o próximo prompt. Veja [Atributos de correlação de evento](#event-correlation-attributes).
* `workspace.host_paths`: diretórios de workspace do host selecionados no aplicativo desktop, como um array de string
* `workflow.run_id`: identificador de execução, prefixado com `wf_`, nos eventos de API e ferramenta emitidos por agentes que pertencem a uma execução de ferramenta [Workflow](/docs/pt/workflows). Filtrando eventos por um `workflow.run_id` reconstrói as solicitações de API e resultados de ferramentas dessa execução. O identificador cobre os agentes que o script de workflow gera e quaisquer agentes que esses gerem por sua vez, como invocações de skill. Ele corresponde ao identificador de execução relatado no resultado da ferramenta Workflow. Ausente em todos os outros eventos. Requer Claude Code v2.1.202 ou posterior
* `workflow.name`: nome do workflow, o `meta.name` do seu script, emitido junto com `workflow.run_id`. Nomes de workflow integrados aparecem verbatim quando a execução executa o script integrado não modificado. Nomes criados pelo usuário, incluindo cópias editadas de scripts integrados, são substituídos por `custom` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido. Requer Claude Code v2.1.202 ou posterior

<h3 id="metrics">
  Métricas
</h3>

Claude Code exporta as seguintes métricas:

| Nome da Métrica                       | Descrição                                                           | Unidade |
| ------------------------------------- | ------------------------------------------------------------------- | ------- |
| `claude_code.session.count`           | Contagem de sessões CLI iniciadas                                   | count   |
| `claude_code.lines_of_code.count`     | Contagem de linhas de código modificadas                            | count   |
| `claude_code.pull_request.count`      | Número de pull requests criados                                     | count   |
| `claude_code.commit.count`            | Número de commits git criados                                       | count   |
| `claude_code.cost.usage`              | Custo da sessão Claude Code                                         | USD     |
| `claude_code.token.usage`             | Número de tokens usados                                             | tokens  |
| `claude_code.code_edit_tool.decision` | Contagem de decisões de permissão da ferramenta de edição de código | count   |
| `claude_code.active_time.total`       | Tempo ativo total em segundos                                       | s       |

<h3 id="metric-details">
  Detalhes das métricas
</h3>

Cada métrica inclui os atributos padrão listados acima. Métricas com atributos adicionais específicos do contexto são observadas abaixo.

<h4 id="session-counter">
  Contador de sessão
</h4>

Incrementado no início de cada sessão.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `start_type`: Como a sessão foi iniciada. Um de `"fresh"`, `"resume"`, `"continue"` ou `"agents_view"`. O valor `"agents_view"` identifica o processo do painel `claude agents`, uma UI local lançada pelo usuário em vez de uma sessão conversacional. Filtre neste valor para separar lançamentos de processo de UI de sessões conversacionais em seus painéis.

<h4 id="lines-of-code-counter">
  Contador de linhas de código
</h4>

Incrementado quando código é adicionado ou removido.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: Identificador do modelo para o modelo que fez a alteração (por exemplo, "claude-sonnet-5")

<h4 id="pull-request-counter">
  Contador de pull request
</h4>

Incrementado quando Claude Code cria um pull request ou merge request através de um comando shell ou uma ferramenta MCP.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)

<h4 id="commit-counter">
  Contador de commit
</h4>

Incrementado ao criar commits git via Claude Code.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)

<h4 id="cost-counter">
  Contador de custo
</h4>

Incrementado após cada solicitação de API.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `model`: Identificador do modelo (por exemplo, "claude-sonnet-5")
* `query_source`: Categoria do subsistema que emitiu a solicitação. Um de `"main"`, `"subagent"` ou `"auxiliary"`
* `speed`: `"fast"` quando a solicitação usou modo rápido. Ausente caso contrário
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à solicitação: `"low"`, `"medium"`, `"high"`, `"xhigh"` ou `"max"`. Ausente quando o modelo não suporta esforço.
* `agent.name`: Tipo de subagente que emitiu a solicitação. Nomes de agente integrados e agentes de plugins do marketplace oficial aparecem verbatim. Outros nomes de agente definidos pelo usuário são substituídos por `"custom"`. Ausente quando a solicitação não foi emitida por um tipo de subagente nomeado.
* `skill.name`: Skill ativa para a solicitação, definida pela ferramenta Skill, um comando `/` ou herdada por um subagente gerado. Nomes de skill integrados, agrupados, definidos pelo usuário e de plugins do marketplace oficial aparecem verbatim. Nomes de skill de plugin de terceiros são substituídos por `"third-party"`. Ausente quando nenhuma skill está ativa.
* `plugin.name`: Plugin proprietário quando a skill ativa ou subagente é fornecido por um plugin. Nomes de plugin do marketplace oficial aparecem verbatim. Nomes de plugin de terceiros são substituídos por `"third-party"`. Ausente quando nem a skill nem o subagente têm um plugin proprietário.
* `marketplace.name`: Marketplace do qual o plugin proprietário foi instalado. Emitido apenas para plugins do marketplace oficial. Ausente caso contrário.
* `mcp_server.name`: Servidor MCP cuja ferramenta foi executada na rodada que produziu esta solicitação. Nomes de servidor integrados, proxied por claude.ai e do registro oficial aparecem verbatim. Nomes de servidor configurados pelo usuário são substituídos por `"custom"`. Ausente quando nenhuma ferramenta MCP foi executada.
* `mcp_tool.name`: Ferramenta MCP que foi executada na rodada que produziu esta solicitação, com a mesma redação que `mcp_server.name`. Ausente quando nenhuma ferramenta MCP foi executada.

<h4 id="token-counter">
  Contador de token
</h4>

Incrementado após cada solicitação de API.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: Identificador do modelo (por exemplo, "claude-sonnet-5")
* `query_source`: Categoria do subsistema que emitiu a solicitação. Um de `"main"`, `"subagent"` ou `"auxiliary"`
* `speed`: `"fast"` quando a solicitação usou modo rápido. Ausente caso contrário
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à solicitação. Veja [Contador de custo](#cost-counter) para detalhes.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribuição de skill, plugin, agente e MCP para a solicitação. Veja [Contador de custo](#cost-counter) para definições e comportamento de redação.

<h4 id="code-edit-tool-decision-counter">
  Contador de decisão da ferramenta de edição de código
</h4>

Incrementado quando o usuário aceita ou rejeita o uso da ferramenta Edit, Write ou NotebookEdit.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `tool_name`: Nome da ferramenta (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Decisão do usuário (`"accept"`, `"reject"`)
* `source`: Onde a decisão veio. Um de `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"` ou `"user_reject"`. Veja o [Evento de decisão da ferramenta](#tool-decision-event) para o que cada valor significa.
* `language`: Linguagem de programação do arquivo editado, como `"TypeScript"`, `"Python"`, `"JavaScript"` ou `"Markdown"`. Retorna `"unknown"` para extensões de arquivo não reconhecidas.

<h4 id="active-time-counter">
  Contador de tempo ativo
</h4>

Rastreia o tempo real gasto usando ativamente Claude Code, excluindo tempo ocioso. Essa métrica é incrementada durante interações do usuário, como digitação e leitura de respostas, e durante processamento CLI, como execução de ferramentas e geração de resposta de IA.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `type`: `"user"` para interações de teclado, `"cli"` para execução de ferramentas e respostas de IA

<h3 id="events">
  Eventos
</h3>

Claude Code exporta os seguintes eventos via logs/eventos OpenTelemetry (quando `OTEL_LOGS_EXPORTER` está configurado):

<h4 id="event-correlation-attributes">
  Atributos de correlação de evento
</h4>

Quando um usuário envia um prompt, Claude Code pode fazer múltiplas chamadas de API e executar várias ferramentas. O atributo `prompt.id` permite vincular todos esses eventos de volta ao único prompt que os acionou.

| Atributo    | Descrição                                                                                            |
| ----------- | ---------------------------------------------------------------------------------------------------- |
| `prompt.id` | Identificador UUID v4 vinculando todos os eventos produzidos ao processar um único prompt do usuário |

Para rastrear toda a atividade acionada por um único prompt, filtre seus eventos por um valor específico de `prompt.id`. Isso retorna o evento user\_prompt, quaisquer eventos api\_request e quaisquer eventos tool\_result que ocorreram ao processar esse prompt.

<Note>
  `prompt.id` é intencionalmente excluído de métricas porque cada prompt gera um ID único, o que criaria um número sempre crescente de séries temporais. Use-o apenas para análise em nível de evento e trilhas de auditoria.
</Note>

<h4 id="user-prompt-event">
  Evento de prompt do usuário
</h4>

Registrado quando um usuário envia um prompt.

**Nome do Evento**: `claude_code.user_prompt`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `prompt_length`: Comprimento do prompt
* `prompt`: Conteúdo do prompt. Reduzido por padrão. Defina `OTEL_LOG_USER_PROMPTS=1` para incluí-lo
* `command_name`: Nome do comando quando o prompt invoca um. Nomes de comando integrados e agrupados como `compact` ou `debug` são emitidos como estão; aliases como `reset` emitem como digitados em vez do nome canônico. Nomes de comando customizado, plugin e MCP colapsam para `custom` ou `mcp` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido
* `command_source`: Origem do comando quando presente: `builtin`, `custom` ou `mcp`. Comandos fornecidos por plugin relatam como `custom`

<h4 id="assistant-response-event">
  Evento de resposta do assistente
</h4>

Registrado após cada solicitação de API que retorna conteúdo de texto do modelo. Apenas os blocos de texto da resposta são incluídos; blocos de pensamento e blocos de uso de ferramenta são excluídos. Requer Claude Code v2.1.193 ou posterior.

**Nome do Evento**: `claude_code.assistant_response`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `response_length`: Comprimento do texto de resposta em caracteres
* `response`: Texto de resposta, truncado em 60 KB. Reduzido para `<REDACTED>` por padrão. Defina `OTEL_LOG_ASSISTANT_RESPONSES=1` para incluí-lo. Quando `OTEL_LOG_ASSISTANT_RESPONSES` não está definido, `OTEL_LOG_USER_PROMPTS` controla isso em vez disso, então defina `OTEL_LOG_ASSISTANT_RESPONSES=0` para manter respostas reduzidas enquanto o registro de prompt está ativado
* `model`: Identificador do modelo (por exemplo, "claude-sonnet-5")
* `request_id`: ID de solicitação da API Anthropic do cabeçalho `request-id` da resposta. Presente apenas quando a API retorna um
* `query_source`: Subsistema que emitiu a solicitação, como `"repl_main_thread"`, `"compact"` ou um nome de subagente

<h4 id="tool-result-event">
  Evento de resultado da ferramenta
</h4>

Registrado quando uma ferramenta conclui a execução. Não é emitido se a chamada da ferramenta foi rejeitada; veja o [Evento de decisão da ferramenta](#tool-decision-event) para rejeições.

**Nome do Evento**: `claude_code.tool_result`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `tool_name`: Nome da ferramenta
* `tool_use_id`: Identificador único para esta invocação de ferramenta. Corresponde ao `tool_use_id` passado para hooks, permitindo correlação entre eventos OTel e dados capturados por hook.
* `success`: `"true"` ou `"false"`
* `duration_ms`: Tempo de execução em milissegundos
* `error_type`: String de categoria de erro quando a ferramenta falhou, como `"Error:ENOENT"` ou `"ShellError"`
* `error` (quando `OTEL_LOG_TOOL_DETAILS=1`): Mensagem de erro completa quando a ferramenta falhou
* `decision_type`: Sempre `"accept"`, já que este evento é emitido apenas após a ferramenta ser executada. Chamadas rejeitadas não produzem um resultado de ferramenta
* `decision_source`: Onde a decisão de permissão veio. Um de `"config"`, `"hook"`, `"user_permanent"` ou `"user_temporary"`. Veja o [Evento de decisão da ferramenta](#tool-decision-event) para o que cada valor significa. As fontes apenas de rejeição `"user_abort"` e `"user_reject"` nunca aparecem neste evento.
* `tool_input_size_bytes`: Tamanho da entrada da ferramenta serializada em JSON em bytes
* `tool_result_size_bytes`: Tamanho do resultado da ferramenta em bytes
* `mcp_server_scope`: Identificador de escopo do servidor MCP (para ferramentas MCP)
* `tool_parameters` (quando `OTEL_LOG_TOOL_DETAILS=1`): String JSON contendo parâmetros específicos da ferramenta:
  * Para ferramenta Bash: inclui `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox` e `git_commit_id` (o SHA do commit, quando um comando `git commit` é bem-sucedido)
  * Para ferramenta WorkspaceBash: inclui `bash_command`, `full_command`, `timeout`
  * Para ferramentas MCP: inclui `mcp_server_name`, `mcp_tool_name`
  * Para ferramenta Skill: inclui `skill_name`
  * Para ferramenta Agent ou ferramenta Task legada: inclui `subagent_type`
* `tool_input` (quando `OTEL_LOG_TOOL_DETAILS=1`): Argumentos de ferramenta serializados em JSON. Valores individuais com mais de 512 caracteres são truncados, e a carga útil completa é limitada a \~4 K caracteres. Aplica-se a todas as ferramentas, incluindo ferramentas MCP.

<h4 id="api-request-event">
  Evento de solicitação de API
</h4>

Registrado para cada solicitação de API para Claude.

**Nome do Evento**: `claude_code.api_request`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `model`: Modelo usado (por exemplo, "claude-sonnet-5")
* `cost_usd`: Custo estimado em USD
* `duration_ms`: Duração da solicitação em milissegundos
* `input_tokens`: Número de tokens de entrada
* `output_tokens`: Número de tokens de saída
* `cache_read_tokens`: Número de tokens lidos do cache
* `cache_creation_tokens`: Número de tokens usados para criação de cache
* `request_id`: ID de solicitação da API Anthropic do cabeçalho `request-id` da resposta, como `"req_011..."`. Presente apenas quando a API retorna um.
* `speed`: `"fast"` ou `"normal"`, indicando se o modo rápido estava ativo
* `query_source`: Subsistema que emitiu a solicitação, como `"repl_main_thread"`, `"compact"` ou um nome de subagente
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à solicitação: `"low"`, `"medium"`, `"high"`, `"xhigh"` ou `"max"`. Ausente quando o modelo não suporta esforço.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribuição de skill, plugin, agente e MCP para a solicitação. Veja [Contador de custo](#cost-counter) para definições e comportamento de redação.

<h4 id="api-error-event">
  Evento de erro de API
</h4>

Registrado quando uma solicitação de API para Claude falha.

**Nome do Evento**: `claude_code.api_error`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `model`: Modelo usado (por exemplo, "claude-sonnet-5")
* `error`: Mensagem de erro
* `status_code`: Código de status HTTP como número. Ausente para erros não-HTTP, como falhas de conexão.
* `duration_ms`: Duração da solicitação em milissegundos
* `attempt`: Número total de tentativas feitas, incluindo a solicitação inicial (`1` significa que nenhuma tentativa ocorreu)
* `request_id`: ID de solicitação da API Anthropic do cabeçalho `request-id` da resposta, como `"req_011..."`. Presente apenas quando a API retorna um.
* `speed`: `"fast"` ou `"normal"`, indicando se o modo rápido estava ativo
* `query_source`: Subsistema que emitiu a solicitação, como `"repl_main_thread"`, `"compact"` ou um nome de subagente
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à solicitação. Ausente quando o modelo não suporta esforço.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribuição de skill, plugin, agente e MCP para a solicitação. Veja [Contador de custo](#cost-counter) para definições e comportamento de redação.

<h4 id="api-refusal-event">
  Evento de recusa de API
</h4>

Registrado quando uma solicitação de API retorna `stop_reason: "refusal"`. Recusas chegam em um fluxo de resposta bem-sucedido em vez de como um erro HTTP, então o evento `api_error` não dispara para elas. Este evento permite rastrear a frequência de recusa e agrupar recusas pelos mesmos atributos que `api_request` e `api_error`.

**Nome do Evento**: `claude_code.api_refusal`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `model`: Identificador do modelo da solicitação
* `request_id`: ID de solicitação da API Anthropic do cabeçalho `request-id` da resposta, como `"req_011..."`. Presente apenas quando a API retorna um.
* `query_source`: Subsistema que emitiu a solicitação, como `"repl_main_thread"`, `"compact"` ou um nome de subagente. Veja [`api_request`](#api-request-event) para definições.
* `speed`: Ou `"fast"` quando [Fast mode](/docs/pt/fast-mode) está ativo, ou `"normal"`
* `attempt`: Número de tentativa de repetição. A primeira tentativa é `1`.
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à solicitação. Ausente quando o modelo não suporta esforço.
* `server_fallback_hop`: `true` quando o fallback de modelo do lado do servidor da API já tentou novamente esta recusa em um modelo diferente, então o usuário não viu esta recusa particular. `false` quando a solicitação terminou em uma recusa. Uma única rodada pode emitir tanto um evento hop `true` quanto um evento final `false` posterior quando o modelo de fallback também recusa.
* `has_category`: `true` quando a resposta da API carregava um `stop_details.category` de `"cyber"`, `"bio"`, `"frontier_llm"` ou `"reasoning_extraction"`. `false` quando a resposta não carregava categoria ou um valor fora desse conjunto. Ausente quando `server_fallback_hop` é `true`, porque blocos hop não carregam `stop_details`.
* `has_explanation`: `true` quando a resposta da API carregava um `stop_details.explanation`, caso contrário `false`. Ausente quando `server_fallback_hop` é `true`.
* `category`: O valor `stop_details.category` da resposta da API. Um de `"cyber"`, `"bio"`, `"frontier_llm"` ou `"reasoning_extraction"`. Presente apenas quando `OTEL_LOG_TOOL_DETAILS=1` está definido e `has_category` é `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribuição de skill, plugin, agente e MCP para a solicitação. Veja [Contador de custo](#cost-counter) para definições e comportamento de redação.

<h4 id="api-request-body-event">
  Evento de corpo de solicitação de API
</h4>

Registrado para cada tentativa de solicitação de API quando `OTEL_LOG_RAW_API_BODIES` está definido. Um evento é emitido por tentativa, então tentativas com parâmetros ajustados cada uma produzem seu próprio evento.

**Nome do Evento**: `claude_code.api_request_body`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `body`: Parâmetros de solicitação da API Messages serializados em JSON (prompt do sistema, mensagens, ferramentas, etc.), truncados em 60 KB. Conteúdo de pensamento estendido em turnos anteriores do assistente é reduzido. Emitido apenas em modo inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Caminho absoluto para um arquivo `<dir>/<uuid>.request.json` contendo o corpo não truncado. Emitido apenas em modo arquivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Comprimento do corpo não truncado. Bytes UTF-8 quando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, ou unidades de código UTF-16 quando `=1`
* `body_truncated`: `"true"` quando truncamento inline ocorreu. Ausente em modo arquivo e quando nenhum truncamento ocorreu.
* `model`: Identificador do modelo dos parâmetros de solicitação
* `query_source`: Subsistema que emitiu a solicitação (por exemplo, `"compact"`)

<h4 id="api-response-body-event">
  Evento de corpo de resposta de API
</h4>

Registrado para cada resposta de API bem-sucedida quando `OTEL_LOG_RAW_API_BODIES` está definido.

**Nome do Evento**: `claude_code.api_response_body`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `body`: Resposta da API Messages serializada em JSON (id, blocos de conteúdo, uso, razão de parada), truncada em 60 KB. Conteúdo de pensamento estendido é reduzido. Emitido apenas em modo inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Caminho absoluto para um arquivo `<dir>/<request_id>.response.json` contendo o corpo não truncado. Emitido apenas em modo arquivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Comprimento do corpo não truncado. Bytes UTF-8 quando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, ou unidades de código UTF-16 quando `=1`
* `body_truncated`: `"true"` quando truncamento inline ocorreu. Ausente em modo arquivo e quando nenhum truncamento ocorreu.
* `model`: Identificador do modelo
* `query_source`: Subsistema que emitiu a solicitação
* `request_id`: ID de solicitação da API Anthropic do cabeçalho `request-id` da resposta, como `"req_011..."`. Presente apenas quando a API retorna um.

<h4 id="tool-decision-event">
  Evento de decisão da ferramenta
</h4>

Registrado quando uma decisão de permissão da ferramenta é feita (aceitar/rejeitar).

**Nome do Evento**: `claude_code.tool_decision`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `tool_name`: Nome da ferramenta (por exemplo, "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: Identificador único para esta invocação de ferramenta. Corresponde ao `tool_use_id` passado para hooks, permitindo correlação entre eventos OTel e dados capturados por hook.
* `decision`: Ou `"accept"` ou `"reject"`
* `source`: Onde a decisão veio:
  * `"config"`: Decidido automaticamente sem avisar, baseado em configurações de projeto, regras de permissão nas configurações pessoais do usuário, política gerenciada corporativa, sinalizadores `--allowedTools` ou `--disallowedTools`, o modo de permissão ativo, uma concessão com escopo de sessão de um prompt anterior na mesma sessão CLI interativa, ou porque a ferramenta é inerentemente segura. O evento não indica qual dessas fontes correspondeu.
  * `"hook"`: Um hook `PreToolUse` ou `PermissionRequest` retornou a decisão.
  * `"user_permanent"`: Emitido quando o usuário escolheu "Sim, e não pergunte novamente para ..." em um aviso de permissão, que salva uma regra de permissão em suas configurações pessoais. Na CLI interativa isso é emitido apenas para essa escolha em si; chamadas posteriores que correspondem à regra salva emitem `"config"` em vez disso. No Agent SDK ou sessões não-interativas `-p`, tanto a escolha inicial quanto correspondências de regra posteriores emitem `"user_permanent"`. Tratado como uma aceitação.
  * `"user_temporary"`: Emitido quando o usuário escolheu "Sim" em um aviso de permissão para uma aprovação única, ou escolheu uma das opções "... durante esta sessão" em um aviso de edição ou leitura de arquivo. Na CLI interativa isso é emitido apenas para a escolha em si; chamadas posteriores permitidas por essa concessão com escopo de sessão emitem `"config"` em vez disso. No Agent SDK ou sessões não-interativas `-p`, tanto a escolha quanto correspondências posteriores emitem `"user_temporary"`. Tratado como uma aceitação.
  * `"user_abort"`: Emitido quando o usuário descartou o aviso de permissão sem responder. Tratado como uma rejeição.
  * `"user_reject"`: Emitido quando o usuário escolheu "Não" quando solicitado. Na CLI interativa isso é emitido apenas para essa escolha em si; chamadas que correspondem a uma regra de negação nas configurações pessoais do usuário emitem `"config"` em vez disso. No Agent SDK ou sessões não-interativas `-p`, chamadas que correspondem a uma regra de negação em configurações pessoais emitem `"user_reject"`. Tratado como uma rejeição.
* `tool_parameters` (quando `OTEL_LOG_TOOL_DETAILS=1`): String JSON contendo parâmetros específicos da ferramenta. Mesma forma que o [Evento de resultado da ferramenta](#tool-result-event), menos campos pós-execução como `git_commit_id`. Os valores podem diferir de `tool_result` para uma chamada aceita se a decisão de permissão reescrever a entrada da ferramenta via `updatedInput`. Use este atributo para ver qual comando foi rejeitado quando `decision` é `"reject"`.
  * Para ferramenta Bash: inclui `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`
  * Para ferramenta WorkspaceBash: inclui `bash_command`, `full_command`, `timeout`
  * Para ferramentas MCP: inclui `mcp_server_name`, `mcp_tool_name`
  * Para ferramenta Skill: inclui `skill_name`
  * Para ferramenta Agent ou ferramenta Task legada: inclui `subagent_type`

<h4 id="permission-mode-changed-event">
  Evento de modo de permissão alterado
</h4>

Registrado quando o modo de permissão muda, por exemplo de ciclagem Shift+Tab, saída do Plan Mode ou verificação de gate de modo automático.

**Nome do Evento**: `claude_code.permission_mode_changed`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `from_mode`: O modo de permissão anterior, por exemplo `"default"`, `"plan"`, `"acceptEdits"`, `"auto"` ou `"bypassPermissions"`
* `to_mode`: O novo modo de permissão
* `trigger`: O que causou a mudança. Um de `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"` ou `"auto_opt_in"`. Ausente quando a transição se origina do SDK ou bridge

<h4 id="auth-event">
  Evento de autenticação
</h4>

Registrado quando `/login` ou `/logout` é concluído.

**Nome do Evento**: `claude_code.auth`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `action`: `"login"` ou `"logout"`
* `success`: `"true"` ou `"false"`
* `auth_method`: Método de autenticação, como `"oauth"`
* `error_category`: Tipo de erro categórico quando a ação falhou. A mensagem de erro bruta nunca é incluída
* `status_code`: Código de status HTTP como string quando a ação falhou com um erro HTTP

<h4 id="mcp-server-connection-event">
  Evento de conexão do servidor MCP
</h4>

Registrado quando um servidor MCP se conecta, desconecta ou falha ao conectar.

**Nome do Evento**: `claude_code.mcp_server_connection`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `status`: `"connected"`, `"failed"` ou `"disconnected"`
* `transport_type`: Transporte do servidor, como `"stdio"`, `"sse"` ou `"http"`
* `server_scope`: Escopo em que o servidor está configurado, como `"user"`, `"project"` ou `"local"`
* `duration_ms`: Duração da tentativa de conexão em milissegundos
* `error_code`: Código de erro quando a conexão falhou
* `is_plugin`: `true` quando o servidor é fornecido por um plugin, `false` caso contrário
* `plugin_id_hash` (quando `is_plugin` é `true`): Hash estável do nome do plugin e marketplace, para agrupar eventos por plugin sem expor o nome
* `plugin.name` (quando `is_plugin` é `true`): Nome do plugin que fornece o servidor. Para plugins de terceiros isso é a string literal `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`; isso protege nomes de plugins de terceiros de aparecerem em logs por padrão. Plugins de fontes oficiais Anthropic são sempre identificados por nome. Os atributos `plugin_id_hash` e `plugin.name` fluem para seu próprio backend de monitoramento e não são enviados para Anthropic
* `server_name` (quando `OTEL_LOG_TOOL_DETAILS=1`): Nome do servidor configurado
* `error` (quando `OTEL_LOG_TOOL_DETAILS=1`): Mensagem de erro completa quando a conexão falhou

<h4 id="internal-error-event">
  Evento de erro interno
</h4>

Registrado quando Claude Code captura um erro interno inesperado. Apenas o nome da classe de erro e um código estilo errno são registrados. A mensagem de erro e rastreamento de pilha nunca são incluídos. Este evento não é emitido ao executar contra Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, ou quando `DISABLE_ERROR_REPORTING` está definido.

**Nome do Evento**: `claude_code.internal_error`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `error_name`: Nome da classe de erro, como `"TypeError"` ou `"SyntaxError"`
* `error_code`: Código errno Node.js como `"ENOENT"` quando presente no erro

<h4 id="plugin-installed-event">
  Evento de plugin instalado
</h4>

Registrado quando um plugin termina de instalar, tanto do comando CLI `claude plugin install` quanto da UI interativa `/plugin`.

**Nome do Evento**: `claude_code.plugin_installed`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `marketplace.is_official`: `"true"` se o marketplace é um marketplace oficial Anthropic, `"false"` caso contrário
* `install.trigger`: `"cli"` ou `"ui"`
* `plugin.name`: Nome do plugin instalado. Para marketplaces de terceiros isso é incluído apenas quando `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version`: Versão do plugin quando declarada na entrada do marketplace. Para marketplaces de terceiros isso é incluído apenas quando `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace do qual o plugin foi instalado. Para marketplaces de terceiros isso é incluído apenas quando `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Evento de plugin carregado
</h4>

Registrado uma vez por plugin ativado no início da sessão. Use este evento para inventariar quais plugins estão ativos em toda a sua frota, como complemento ao `plugin_installed` que registra a ação de instalação em si.

**Nome do Evento**: `claude_code.plugin_loaded`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `plugin.name`: nome do plugin. Para plugins fora do marketplace oficial e pacote integrado o valor é `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: marketplace do qual o plugin foi instalado, quando conhecido. Reduzido para `"third-party"` sob a mesma condição que `plugin.name`
* `plugin.version`: versão do manifesto do plugin. Incluído apenas quando o nome não é reduzido e o manifesto declara uma versão
* `plugin.scope`: categoria de proveniência para o plugin: `"official"`, `"org"`, `"user-local"` ou `"default-bundle"`
* `enabled_via`: como o plugin veio a ser ativado: `"default-enable"`, `"org-policy"`, `"seed-mount"` ou `"user-install"`
* `plugin_id_hash`: hash determinístico do nome do plugin e marketplace, enviado apenas para seu exportador configurado. Permite contar quantos plugins de terceiros distintos são carregados em toda a sua frota sem registrar seus nomes
* `has_hooks`: se o plugin contribui hooks
* `has_mcp`: se o plugin contribui servidores MCP
* `host_owned_mcp`: `true` quando o host SDK gerencia as conexões MCP deste plugin e Claude Code pulou a leitura da configuração do servidor MCP do plugin, `false` caso contrário. Requer Claude Code v2.1.172 ou posterior
* `skill_path_count`: número de diretórios de skill que o plugin declara
* `command_path_count`: número de diretórios de comando que o plugin declara
* `agent_path_count`: número de diretórios de agente que o plugin declara
* `safe_mode`: `"true"` quando a sessão foi iniciada com [`--safe-mode`](/docs/pt/cli-reference), `"false"` caso contrário. Em modo seguro este evento relata apenas inventário configurado; os comandos, skills, hooks e servidores MCP do plugin não carregam. Requer Claude Code v2.1.169 ou posterior

<h4 id="skill-activated-event">
  Evento de skill ativado
</h4>

Registrado quando uma skill é invocada, seja Claude a chama através da ferramenta Skill ou você a executa como um comando `/`.

**Nome do Evento**: `claude_code.skill_activated`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `skill.name`: Nome da skill. Para skills definidas pelo usuário e de plugin de terceiros o valor é o placeholder `"custom_skill"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Como a skill foi acionada (`"user-slash"`, `"claude-proactive"` ou `"nested-skill"`)
* `skill.source`: De onde a skill foi carregada (por exemplo, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"` quando a skill é uma skill de workflow. Ausente caso contrário
* `plugin.name` (quando `OTEL_LOG_TOOL_DETAILS=1` ou o plugin é de um marketplace oficial): Nome do plugin proprietário quando a skill é fornecida por um plugin
* `marketplace.name` (quando `OTEL_LOG_TOOL_DETAILS=1` ou o plugin é de um marketplace oficial): Marketplace do qual o plugin proprietário foi instalado, quando a skill é fornecida por um plugin

<h4 id="at-mention-event">
  Evento de menção @
</h4>

Registrado quando Claude Code resolve uma menção `@` em um prompt. Nem toda menção emite um evento: caminhos de saída antecipada, como negações de permissão, arquivos superdimensionados, anexos de referência PDF e falhas de listagem de diretório retornam sem registrar.

**Nome do Evento**: `claude_code.at_mention`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `mention_type`: Tipo de menção (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`)
* `success`: Se a menção foi resolvida com sucesso (`"true"` ou `"false"`)

<h4 id="api-retries-exhausted-event">
  Evento de tentativas de API esgotadas
</h4>

Registrado uma vez quando uma solicitação de API falha após mais de uma tentativa. Emitido junto com o evento `api_error` final.

**Nome do Evento**: `claude_code.api_retries_exhausted`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `model`: Modelo usado
* `error`: Mensagem de erro final
* `status_code`: Código de status HTTP como número. Ausente para erros não-HTTP.
* `total_attempts`: Número total de tentativas feitas
* `total_retry_duration_ms`: Tempo total de parede em todas as tentativas
* `speed`: `"fast"` ou `"normal"`

<h4 id="hook-registered-event">
  Evento de hook registrado
</h4>

Registrado uma vez por hook configurado no início da sessão. Use este evento para inventariar quais hooks estão ativos em toda a sua frota, como complemento aos eventos `hook_execution_start` e `hook_execution_complete` por execução.

**Nome do Evento**: `claude_code.hook_registered`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `hook_event`: tipo de evento de hook, como `"PreToolUse"` ou `"PostToolUse"`
* `hook_type`: tipo de implementação de hook: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"` ou `"agent"`
* `hook_source`: onde o hook é definido: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"` ou `"pluginHook"`
* `safe_mode`: `"true"` quando a sessão foi iniciada com [`--safe-mode`](/docs/pt/cli-reference), `"false"` caso contrário. Requer Claude Code v2.1.169 ou posterior
* `hook_matcher` (quando `OTEL_LOG_TOOL_DETAILS=1`): a string matcher da configuração do hook, quando uma está definida
* `plugin.name` (quando `hook_source` é `"pluginHook"`): nome do plugin contribuidor. Para plugins fora do marketplace oficial e pacote integrado o valor é `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (quando `hook_source` é `"pluginHook"`): hash determinístico do nome do plugin e marketplace, enviado apenas para seu exportador configurado. Permite contar plugins contribuidores distintos sem registrar seus nomes

<h4 id="hook-execution-start-event">
  Evento de início de execução de hook
</h4>

Registrado quando um ou mais hooks começam a executar para um evento de hook.

**Nome do Evento**: `claude_code.hook_execution_start`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `hook_event`: Tipo de evento de hook, como `"PreToolUse"` ou `"PostToolUse"`
* `hook_name`: Nome completo do hook incluindo matcher, como `"PreToolUse:Write"`
* `num_hooks`: Número de comandos de hook correspondentes
* `managed_only`: `"true"` quando apenas hooks de política gerenciada são permitidos
* `hook_source`: `"policySettings"` ou `"merged"`
* `safe_mode`: `"true"` quando a sessão foi iniciada com [`--safe-mode`](/docs/pt/cli-reference), `"false"` caso contrário. Requer Claude Code v2.1.169 ou posterior
* `hook_definitions`: Configuração de hook serializada em JSON. Incluído apenas quando rastreamento beta detalhado e `OTEL_LOG_TOOL_DETAILS=1` estão ambos ativados

<h4 id="hook-execution-complete-event">
  Evento de conclusão de execução de hook
</h4>

Registrado quando todos os hooks para um evento de hook terminaram.

**Nome do Evento**: `claude_code.hook_execution_complete`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `hook_event`: Tipo de evento de hook
* `hook_name`: Nome completo do hook incluindo matcher
* `num_hooks`: Número de comandos de hook correspondentes
* `num_success`: Contagem que completou com sucesso
* `num_blocking`: Contagem que retornou uma decisão de bloqueio
* `num_non_blocking_error`: Contagem que falhou sem bloquear
* `num_cancelled`: Contagem cancelada antes da conclusão
* `total_duration_ms`: Duração de parede de todos os hooks correspondentes
* `managed_only`: `"true"` quando apenas hooks de política gerenciada são permitidos
* `hook_source`: `"policySettings"` ou `"merged"`
* `safe_mode`: `"true"` quando a sessão foi iniciada com [`--safe-mode`](/docs/pt/cli-reference), `"false"` caso contrário. Requer Claude Code v2.1.169 ou posterior
* `hook_definitions`: Configuração de hook serializada em JSON. Incluído apenas quando rastreamento beta detalhado e `OTEL_LOG_TOOL_DETAILS=1` estão ambos ativados

<h4 id="hook-plugin-metrics-event">
  Evento de métricas de plugin de hook
</h4>

Registrado quando um hook de plugin do marketplace oficial emite métricas por invocação. Apenas plugins instalados de um marketplace oficial Anthropic podem emitir esses dados. Plugins de marketplace de terceiros e hooks configurados pelo usuário não emitem para este evento. Use este evento para monitorar o comportamento do plugin, como taxas de descoberta, custos e durações de sua própria pilha de observabilidade.

**Nome do Evento**: `claude_code.hook_plugin_metrics`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `plugin_id`: identificador do plugin em forma `<name>@<marketplace>`
* `hook_event`: tipo de evento de hook que emitiu as métricas
* Até 20 chaves de métrica emitidas pelo plugin. Os nomes correspondem a `^[a-z][a-z0-9_]{0,39}$`. Os valores são booleanos ou números.

<h4 id="compaction-event">
  Evento de compactação
</h4>

Registrado quando a compactação de conversa é concluída.

**Nome do Evento**: `claude_code.compaction`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `trigger`: `"auto"` ou `"manual"`
* `success`: `"true"` ou `"false"`
* `duration_ms`: Duração da compactação
* `pre_tokens`: Contagem aproximada de tokens antes da compactação
* `post_tokens`: Contagem aproximada de tokens após compactação
* `error`: Mensagem de erro quando a compactação falhou
* `precompute_reuse`: Definido apenas quando `trigger` é `"manual"`. A compactação automática pode preparar um resumo em segundo plano antes da janela de contexto se encher, e este atributo registra se `/compact` reutilizou esse resumo preparado. `"hit"` significa que foi reutilizado; `"miss_custom_instructions"`, `"miss_hook"` e `"miss_not_ready"` dão a razão pela qual um resumo fresco foi computado em vez disso. Requer Claude Code v2.1.153 ou posterior

<h4 id="feedback-survey-event">
  Evento de pesquisa de feedback
</h4>

Registrado quando uma pesquisa de qualidade de sessão é mostrada ou respondida. Veja [Pesquisas de qualidade de sessão](/docs/pt/data-usage#session-quality-surveys) para o que as pesquisas coletam e como controlá-las.

**Nome do Evento**: `claude_code.feedback_survey`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: Contador monotonicamente crescente para ordenar eventos dentro de uma sessão
* `event_type`: Evento do ciclo de vida da pesquisa, por exemplo `"appeared"`, `"responded"` ou `"transcript_prompt_appeared"`
* `appearance_id`: ID único vinculando os eventos emitidos para uma instância de pesquisa
* `survey_type`: Qual pesquisa produziu o evento. `"session"` é o prompt de classificação "Como Claude está se saindo?"
* `response`: A seleção do usuário em eventos `responded`
* `enabled_via_override`: `true` quando [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/pt/env-vars) está definido. Emitido como um booleano, não uma string. Presente em eventos de pesquisa `session`. Filtre neste atributo para confirmar que a substituição é aplicada em toda a frota

<h2 id="interpret-metrics-and-events-data">
  Interpretar dados de métricas e eventos
</h2>

As métricas e eventos exportados suportam uma gama de análises:

<h3 id="usage-monitoring">
  Monitoramento de uso
</h3>

| Métrica                                                       | Oportunidade de Análise                                                                                  |
| ------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `claude_code.token.usage`                                     | Dividir por `type` (entrada/saída), usuário, equipe, modelo, `skill.name`, `plugin.name` ou `agent.name` |
| `claude_code.session.count`                                   | Rastrear adoção e engajamento ao longo do tempo                                                          |
| `claude_code.lines_of_code.count`                             | Medir produtividade rastreando adições e remoções de código, dividido por modelo                         |
| `claude_code.commit.count` & `claude_code.pull_request.count` | Entender o impacto nos fluxos de trabalho de desenvolvimento                                             |

<h3 id="cost-monitoring">
  Monitoramento de custo
</h3>

A métrica `claude_code.cost.usage` ajuda com:

* Rastreamento de tendências de uso entre equipes ou indivíduos
* Identificação de sessões de alto uso para otimização
* Atribuição de gastos a skills, plugins ou tipos de subagente específicos via atributos `skill.name`, `plugin.name` e `agent.name`

<Note>
  As métricas de custo são aproximações. Para dados de faturamento oficiais, consulte seu provedor de API (Claude Console, Amazon Bedrock ou Google Cloud's Agent Platform).
</Note>

<h3 id="alerting-and-segmentation">
  Alertas e segmentação
</h3>

Alertas comuns a considerar:

* Picos de custo
* Consumo incomum de tokens
* Alto volume de sessão de usuários específicos

Todas as métricas podem ser segmentadas pelos [atributos padrão](#standard-attributes). O atributo `model` está disponível em `claude_code.token.usage`, `claude_code.cost.usage` e a partir da v2.1.172, `claude_code.lines_of_code.count`.

Divisões por modelo de commits podem ser apenas aproximadas unindo contra as métricas de token ou custo em `session.id`, já que uma sessão pode abranger vários modelos. Filtre o lado do token ou custo para linhas onde `query_source` é `"main"` para que solicitações auxiliares e de subagente não atribuam os commits da sessão a um modelo que não os fez.

<h3 id="detect-retry-exhaustion">
  Detectar esgotamento de tentativas
</h3>

Claude Code retenta solicitações de API falhadas internamente e emite um único evento `claude_code.api_error` apenas depois de desistir, então o evento em si é o sinal terminal para essa solicitação. Tentativas de repetição intermediárias não são registradas como eventos separados.

O atributo `attempt` no evento registra o número total de tentativas. `CLAUDE_CODE_MAX_RETRIES` tem como padrão 10 e é limitado a 15; a partir da v2.1.199, `CLAUDE_CODE_RETRY_WATCHDOG` aumenta o padrão e remove o limite. Quando a solicitação esgota todas as tentativas em um erro transitório, `attempt` é igual a um a mais do que esse limite efetivo: 11 por padrão, e nunca mais de 16 a menos que o watchdog esteja definido. Um valor menor indica um erro não retentável, como uma resposta `400`.

Para distinguir uma sessão que se recuperou de uma que travou, agrupe eventos por `session.id` e verifique se um evento `api_request` posterior existe após o erro.

<h3 id="event-analysis">
  Análise de eventos
</h3>

Os dados de eventos fornecem insights detalhados sobre interações do Claude Code:

**Padrões de Uso de Ferramentas**: analise eventos de resultado de ferramentas para identificar:

* Ferramentas mais frequentemente usadas
* Taxas de sucesso da ferramenta
* Tempos médios de execução da ferramenta
* Padrões de erro por tipo de ferramenta

**Monitoramento de Desempenho**: rastreie durações de solicitações de API e tempos de execução de ferramentas para identificar gargalos de desempenho.

<h2 id="audit-security-events">
  Auditar eventos de segurança
</h2>

Os eventos OpenTelemetry são a fonte de dados de auditoria para atividade do Claude Code. Cada evento carrega atributos de identidade que vinculam chamadas de ferramenta, atividade MCP e decisões de permissão de volta ao usuário que as acionou. O exportador de logs OTLP pode entregar esses eventos a qualquer plataforma Security Information and Event Management (SIEM) com um receptor OTLP, ou a um OpenTelemetry Collector que encaminha para seu SIEM.

<h3 id="attribute-actions-to-users">
  Atribuir ações a usuários
</h3>

Os [atributos padrão](#standard-attributes) em cada evento incluem a identidade do usuário autenticado: `user.email`, `user.account_uuid`, `user.account_id` e `organization.id` quando conectado com uma conta Claude, mais `user.id` e o `session.id` por sessão. `user.id` é um identificador com escopo de instalação, exceto em sessões do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), onde é o assunto do IdP do token emitido pelo gateway.

Chamadas de ferramenta MCP, comandos Bash e edições de arquivo são, portanto, atribuídas ao desenvolvedor que iniciou a sessão. Claude Code não atua sob uma conta de serviço separada; a identidade registrada em cada evento é a própria conta Claude do desenvolvedor, ou a identidade do IdP do desenvolvedor em uma sessão do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway).

Quando Claude Code autentica com uma chave de API direta, ou contra Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, não há conta Claude na sessão e apenas `user.id` e `session.id` são preenchidos. Nessas implantações, anexe identidade do usuário você mesmo com `OTEL_RESOURCE_ATTRIBUTES`, definido por usuário através do arquivo de [configurações gerenciadas](#administrator-configuration) ou um wrapper de inicialização. Sessões do gateway de aplicativos Claude não precisam de nada disso: a CLI marca a identidade do IdP automaticamente, conforme descrito em [Atributos padrão](#standard-attributes).

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

<h3 id="audit-mcp-activity">
  Auditoria de atividade MCP
</h3>

Para capturar atividade do servidor MCP com detalhe completo de chamada, ative o exportador de logs e defina `OTEL_LOG_TOOL_DETAILS=1`. Cada operação MCP então produz eventos estruturados que carregam o nome do servidor, nome da ferramenta e argumentos de chamada junto com os atributos de identidade padrão:

| Evento                  | O que registra para MCP                                                                                                                                                                                             |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_server_connection` | Conexão do servidor, desconexão e falha de conexão com `server_name`, `transport_type`, `server_scope` e detalhe de erro                                                                                            |
| `tool_result`           | Cada chamada de ferramenta MCP com `tool_name` e `mcp_server_scope`, uma carga útil `tool_parameters` contendo `mcp_server_name` e `mcp_tool_name`, e uma carga útil `tool_input` contendo os argumentos de chamada |
| `tool_decision`         | Se a chamada foi permitida ou negada, se a decisão veio de config, um hook ou o usuário, e uma carga útil `tool_parameters` contendo `mcp_server_name` e `mcp_tool_name`                                            |

Sem `OTEL_LOG_TOOL_DETAILS`, esses eventos descartam o detalhe de identificação:

* `tool_result`: mantém `tool_name` e `mcp_server_scope`, omite `mcp_server_name`, `mcp_tool_name` e argumentos
* `tool_decision`: mantém `tool_name`, omite `tool_parameters`
* `mcp_server_connection`: omite `server_name` e a mensagem de erro, mas mantém `is_plugin`, `plugin_id_hash` e `plugin.name`, com nomes de plugins não-Anthropic reduzidos ao literal `"third-party"`, para que servidores fornecidos por plugins permaneçam distinguíveis sem registro detalhado

<h3 id="map-security-questions-to-events">
  Mapear questões de segurança para eventos
</h3>

Ao construir regras de detecção, procure o sinal que você deseja monitorar e consulte seu backend para o evento correspondente e atributos:

| Sinal                                                | Evento                                                                                 | Atributos-chave                                              |
| ---------------------------------------------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| Chamada de ferramenta permitida ou negada, e por quê | `tool_decision`                                                                        | `decision`, `source`, `tool_name`, `tool_parameters`         |
| Escalação de modo de permissão                       | `permission_mode_changed`                                                              | `from_mode`, `to_mode`, `trigger`                            |
| Hook de política bloqueou uma ação                   | `hook_execution_complete`                                                              | `hook_event`, `num_blocking`                                 |
| Login, logout e falha de autenticação                | `auth`                                                                                 | `action`, `success`, `error_category`                        |
| Conexão do servidor MCP ou falha                     | `mcp_server_connection`                                                                | `status`, `server_name`, `is_plugin`, `error_code`           |
| Plugin instalado e sua origem                        | `plugin_installed`                                                                     | `plugin.name`, `marketplace.name`, `marketplace.is_official` |
| Comandos executados e arquivos tocados               | `tool_result` (executado) ou `tool_decision` (rejeitado) com `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (apenas `tool_result`)       |

Claude Code emite apenas o fluxo de eventos bruto. Detecção de anomalias, linha de base, correlação entre sessões e alertas são responsabilidade do seu SIEM ou backend de observabilidade.

<h3 id="send-events-to-a-siem">
  Enviar eventos para um SIEM
</h3>

Aponte `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` para o receptor OTLP do seu SIEM, ou para um OpenTelemetry Collector que encaminha para a API de ingestão nativa do seu SIEM. O seguinte exemplo de configurações gerenciadas exporta apenas eventos, com detalhe completo de ferramenta ativado para auditoria MCP e Bash:

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
  Considerações de backend
</h2>

Sua escolha de backends de métricas, logs e rastreamentos determina os tipos de análises que você pode realizar:

<h3 id="for-metrics">
  Para métricas
</h3>

* **Bancos de dados de série temporal (por exemplo, Prometheus)**: Cálculos de taxa, métricas agregadas
* **Armazenamentos colunares (por exemplo, ClickHouse)**: Consultas complexas, análise de usuário único
* **Plataformas de observabilidade completas (por exemplo, Honeycomb, Datadog, Grafana Cloud)**: Consultas avançadas, visualização, alertas

<h3 id="for-events/logs">
  Para eventos/logs
</h3>

* **Sistemas de agregação de logs (por exemplo, Elasticsearch, Loki)**: Busca de texto completo, análise de logs
* **Armazenamentos colunares (por exemplo, ClickHouse)**: Análise de eventos estruturados
* **Plataformas de observabilidade completas (por exemplo, Honeycomb, Datadog, Grafana Cloud)**: Correlação entre métricas e eventos

<h3 id="for-traces">
  Para rastreamentos
</h3>

Escolha um backend que suporte armazenamento de rastreamento distribuído e correlação de span:

* **Sistemas de rastreamento distribuído (por exemplo, Jaeger, Zipkin, Grafana Tempo)**: Visualização de span, waterfalls de solicitação, análise de latência
* **Plataformas de observabilidade completas (por exemplo, Honeycomb, Datadog, Grafana Cloud)**: Busca de rastreamento e correlação com métricas e logs

Para organizações que exigem métricas de Usuário Ativo Diário/Semanal/Mensal (DAU/WAU/MAU), considere backends que suportam consultas eficientes de valor único.

<h2 id="service-information">
  Informações de serviço
</h2>

Todas as métricas e eventos são exportados com os seguintes atributos de recurso:

* `service.name`: `claude-code`
* `service.version`: Versão atual do Claude Code
* `os.type`: Tipo de sistema operacional (por exemplo, `linux`, `darwin`, `windows`)
* `os.version`: String de versão do sistema operacional
* `host.arch`: Arquitetura do host (por exemplo, `amd64`, `arm64`)
* `wsl.version`: Número de versão do WSL (apenas presente ao executar no Windows Subsystem for Linux)
* Nome do Medidor: `com.anthropic.claude_code`

<h2 id="roi-measurement-resources">
  Recursos de medição de ROI
</h2>

Para um guia abrangente sobre como medir o retorno sobre investimento para Claude Code, incluindo configuração de telemetria, análise de custo, métricas de produtividade e relatórios automatizados, consulte o [Guia de Medição de ROI do Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Este repositório fornece configurações Docker Compose prontas para uso, configurações Prometheus e OpenTelemetry, e modelos para gerar relatórios de produtividade integrados com ferramentas como Linear.

<h2 id="security-and-privacy">
  Segurança e privacidade
</h2>

* A exportação OpenTelemetry para seu backend é opt-in e requer configuração explícita. Para a telemetria operacional separada da Anthropic e como desabilitá-la, consulte [Uso de dados](/docs/pt/data-usage#telemetry-services)
* Conteúdos de arquivo brutos e trechos de código não são incluídos em métricas ou eventos. Os spans de rastreamento são um caminho de dados separado: veja o ponto `OTEL_LOG_TOOL_CONTENT` abaixo
* Quando autenticado via OAuth, `user.email` é incluído em atributos de telemetria. Se isso for uma preocupação para sua organização, trabalhe com seu backend de telemetria para filtrar ou reduzir este campo
* O conteúdo do prompt do usuário não é coletado por padrão. Apenas o comprimento do prompt é registrado. Para incluir conteúdo do prompt, defina `OTEL_LOG_USER_PROMPTS=1`
* O texto de resposta do assistente não é coletado por padrão. Apenas o comprimento da resposta é registrado. Para incluir texto de resposta, defina `OTEL_LOG_ASSISTANT_RESPONSES=1`. Como todos os dados OpenTelemetry do Claude Code, o texto de resposta é enviado apenas para o endpoint OTel que você configura, nunca para a Anthropic. Quando esta variável não está definida, `OTEL_LOG_USER_PROMPTS` é usado como fallback, portanto defina `OTEL_LOG_ASSISTANT_RESPONSES=0` se você quiser conteúdo de prompt sem conteúdo de resposta
* Argumentos de entrada de ferramenta e parâmetros não são registrados por padrão. Para incluí-los, defina `OTEL_LOG_TOOL_DETAILS=1`. Estes dados são enviados apenas para o endpoint OTEL que você configura, nunca para a Anthropic. Os argumentos ainda podem conter valores sensíveis, portanto configure seu backend de telemetria para filtrar ou reduzir esses atributos conforme necessário. Quando ativado:
  * Eventos `tool_result` e `tool_decision` incluem um atributo `tool_parameters` com comandos Bash, nomes de servidor MCP e ferramenta, e nomes de skill. Campos como `full_command` são emitidos sem truncamento
  * Eventos `tool_result` adicionalmente incluem um atributo `tool_input` com caminhos de arquivo, URLs, padrões de busca e outros argumentos. Valores individuais com mais de 512 caracteres são truncados e o total é limitado a \~4 K caracteres
  * Eventos `user_prompt` incluem o `command_name` verbatim para comandos customizado, plugin e MCP
  * Spans de rastreamento incluem o mesmo atributo `tool_input` e atributos derivados de entrada como `file_path`, com o mesmo truncamento que `tool_input`
* O conteúdo de entrada e saída de ferramenta não é registrado em spans de rastreamento por padrão. Para incluí-lo, defina `OTEL_LOG_TOOL_CONTENT=1`. Quando ativado, eventos de span incluem conteúdo completo de entrada e saída de ferramenta truncado em 60 KB por span. Isso pode incluir conteúdos de arquivo brutos de resultados da ferramenta Read e saída de comando Bash. Configure seu backend de telemetria para filtrar ou reduzir esses atributos conforme necessário
* Corpos de solicitação e resposta da API Anthropic Messages brutos não são registrados por padrão. Para incluí-los, defina `OTEL_LOG_RAW_API_BODIES`. Com `=1`, cada chamada de API emite eventos de log `api_request_body` e `api_response_body` cujo atributo `body` é a carga útil serializada em JSON, truncada em 60 KB. Com `=file:<dir>`, corpos não truncados são escritos em arquivos `.request.json` e `.response.json` sob esse diretório e os eventos carregam um caminho `body_ref` em vez do corpo inline. Envie o diretório com um coletor de log ou sidecar em vez de através do fluxo de telemetria. Em ambos os modos, os corpos contêm o histórico de conversa completo (prompt do sistema, cada turno anterior de usuário e assistente, resultados de ferramenta), então ativar isso implica consentimento para tudo que os outros sinalizadores de conteúdo `OTEL_LOG_*` revelariam. O conteúdo de pensamento estendido do Claude é sempre reduzido desses corpos independentemente de outras configurações

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Monitorar Claude Code no Amazon Bedrock
</h2>

Para orientação detalhada de monitoramento de uso do Claude Code para Amazon Bedrock, consulte [Implementação de Monitoramento do Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
