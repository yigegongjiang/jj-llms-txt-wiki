> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Monitoramento

> Saiba como ativar e configurar OpenTelemetry para Claude Code.

Rastreie o uso, custos e atividade de ferramentas do Claude Code em toda a sua organização exportando dados de telemetria através do OpenTelemetry (OTel). Claude Code exporta métricas como dados de série temporal via protocolo de métricas padrão, eventos via protocolo de logs/eventos e, opcionalmente, rastreamentos distribuídos via [protocolo de rastreamentos](#traces-beta).

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

# 5. Para depuração: reduzir intervalos de exportação e redefini-los para uso em produção
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 segundos (padrão: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 segundos (padrão: 5000ms)

# 6. Executar Claude Code
claude
```

Para verificar uma configuração que exporta métricas, verifique seu backend para a métrica `claude_code.session.count`, que Claude Code emite quando uma sessão é iniciada. Para verificar uma configuração apenas de logs, envie um prompt e verifique o evento `claude_code.user_prompt`.

Se nada chegar, execute `claude --debug` e verifique o log de depuração. Claude Code relata falhas dos exportadores que você configura como erros `[3P telemetry]`, onde 3P significa third-party. As linhas prefixadas com `[Anthropic telemetry]` descrevem a [telemetria operacional separada da Anthropic](/docs/pt/data-usage#telemetry-services) e não indicam um problema com sua configuração.

Para opções de configuração completas, consulte a [especificação OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Configuração do administrador
</h2>

Os administradores podem configurar as definições de OpenTelemetry para todos os usuários através do [arquivo de configurações gerenciadas](/docs/pt/managed-settings#delivery-mechanisms). Consulte a [precedência de configurações](/docs/pt/settings#settings-precedence) para obter mais informações sobre como as configurações são aplicadas.

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

Claude Code ignora as [variáveis do exportador OpenTelemetry](/docs/pt/settings-reference#variables-claude-code-ignores-in-env) no `.claude/settings.json` e `.claude/settings.local.json` de um repositório, portanto um repositório não pode usá-las para ativar a telemetria, escolher para onde ela vai ou capturar conteúdo. Defina-as nas configurações gerenciadas ou faça com que cada desenvolvedor as defina no seu shell ou `~/.claude/settings.json`. Um repositório ainda pode desativar um sinal definindo seu seletor de exportador, como `OTEL_LOGS_EXPORTER`, como `none`, a menos que as configurações gerenciadas, um arquivo `--settings` ou o ambiente a partir do qual você inicia Claude Code defina essa variável.

Claude Code não passa variáveis de ambiente `OTEL_*` para os subprocessos que ele gera, incluindo a ferramenta Bash, hooks, servidores MCP e servidores de linguagem. Um aplicativo instrumentado com OpenTelemetry que você executa através da ferramenta Bash não herda o endpoint do exportador ou cabeçalhos do Claude Code, então defina essas variáveis diretamente no comando se esse aplicativo precisar exportar sua própria telemetria.

<h3 id="how-managed-settings-lock-the-otlp-destination">
  Como as configurações gerenciadas bloqueiam o destino OTLP
</h3>

Quando você define uma variável `OTEL_EXPORTER_OTLP_*` nas configurações gerenciadas, Claude Code remove variáveis conflitantes definidas pelo desenvolvedor na inicialização e registra um aviso que você pode ver com `claude --debug`. O que ele remove depende de qual variável você define:

* **Endpoints**: quando você define `OTEL_EXPORTER_OTLP_ENDPOINT`, Claude Code remove todos os endpoints por sinal definidos pelo desenvolvedor. Os desenvolvedores não podem apontar um sinal para um coletor diferente, portanto você não precisa também definir as variáveis de endpoint por sinal nas configurações gerenciadas.
* **Protocolos**: quando você define `OTEL_EXPORTER_OTLP_PROTOCOL`, Claude Code remove todos os protocolos por sinal definidos pelo desenvolvedor.
* **Credenciais**: quando você define `OTEL_EXPORTER_OTLP_HEADERS`, `OTEL_EXPORTER_OTLP_CLIENT_KEY` ou `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, Claude Code remove as versões por sinal definidas pelo desenvolvedor dessa variável, além de todas as variáveis de endpoint definidas pelo desenvolvedor, genéricas ou por sinal, já que essas credenciais de outra forma alcançariam um coletor que as configurações gerenciadas não escolheram.
* **Seletores de exportador**: `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` e o `OTEL_TRACES_EXPORTER` beta seguem a precedência normal por chave. Uma configuração do desenvolvedor ainda pode desabilitar um sinal ou alterá-lo para o exportador de console, portanto defina os seletores nas configurações gerenciadas também se você precisar que eles sejam bloqueados. Através de [fontes de administrador](/docs/pt/managed-settings#precedence-within-the-managed-tier), `OTEL_LOGS_EXPORTER` segue a [unidade de telemetria](/docs/pt/server-managed-settings#per-key-exceptions-across-managed-sources) enquanto os outros dois seletores se mesclam por chave. Requer Claude Code v2.1.223 ou posterior.
* **Endpoints de rastreamento beta**: com [rastreamento beta detalhado](#traces-beta) ativo, Claude Code exporta logs e rastreamentos para `BETA_TRACING_ENDPOINT` em vez de através dos exportadores de logs e rastreamentos. Claude Code portanto remove um `BETA_TRACING_ENDPOINT` definido pelo desenvolvedor sempre que qualquer uma dessas configurações gerenciadas decide o destino de qualquer sinal:

  * Um endpoint genérico ou de logs/rastreamentos ou credencial
  * Um [`otelHeadersHelper`](/docs/pt/settings-reference#otelheadershelper)
  * Um seletor de exportador de logs ou rastreamentos definido como `none`, `console` ou vazio, valores que mantêm o sinal fora de um coletor
  * `CLAUDE_CODE_ENABLE_TELEMETRY` desativado

  Um endpoint ou credencial apenas de métricas não o remove. Antes da v2.1.251, um `BETA_TRACING_ENDPOINT` definido pelo desenvolvedor redirecionava os logs e rastreamentos que o rastreamento beta detalhado exporta mesmo quando as configurações gerenciadas fixavam o coletor.

Claude Code não remove variáveis por sinal que você define nas configurações gerenciadas em si, portanto você pode rotear um sinal para um coletor diferente definindo sua variável lá, como o [exemplo SIEM](#send-events-to-a-siem) faz. Se você definir uma credencial por sinal lá, Claude Code remove o endpoint definido pelo desenvolvedor para esse sinal.

Este comportamento de remoção muda para onde a telemetria é entregue, não o que Claude Code coleta.

Antes da v2.1.217, cada variável seguia a precedência de configurações por chave independentemente, portanto um endpoint específico de sinal definido nas configurações do usuário ou no shell redirecionava esse sinal para longe do coletor gerenciado.

Quando o aplicativo de desktop ou um executor de [ambiente auto-hospedado](/docs/pt/self-hosted-environments) inicia Claude Code e nomeia um endpoint OTLP no ambiente que fornece, Claude Code fixa o destino da mesma forma: as variáveis de telemetria do iniciador removem variáveis definidas pelo desenvolvedor exatamente como as configurações gerenciadas fazem. Claude Code não remove variáveis que o próprio iniciador definiu. Requer Claude Code v2.1.251 ou posterior.

<h2 id="configuration-details">
  Detalhes de configuração
</h2>

<h3 id="common-configuration-variables">
  Variáveis de configuração comuns
</h3>

Essas variáveis configuram exportadores, endpoints e comportamento de exportação para todas as implantações.

Se você definir uma variável de endpoint ou protocolo por sinal, como `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`, Claude Code a usa em vez da variável genérica para esse sinal. Se você definir uma variável de cabeçalhos por sinal, como `OTEL_EXPORTER_OTLP_METRICS_HEADERS`, Claude Code a mescla com a genérica `OTEL_EXPORTER_OTLP_HEADERS` para esse sinal.

Em máquinas com configurações gerenciadas, veja [Como as configurações gerenciadas bloqueiam o destino OTLP](#how-managed-settings-lock-the-otlp-destination) para saber o que Claude Code remove.

| Variável de Ambiente                                | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Valores de Exemplo                                                                                                                                                 |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Ativa coleta de telemetria (obrigatório)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `1`                                                                                                                                                                |
| `OTEL_METRICS_EXPORTER`                             | Tipos de exportador de métricas, separados por vírgula. Use `none` para desativar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `console`, `otlp`, `prometheus`, `none`                                                                                                                            |
| `OTEL_LOGS_EXPORTER`                                | Tipos de exportador de logs/eventos, separados por vírgula. Use `none` para desativar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `console`, `otlp`, `none`                                                                                                                                          |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protocolo para exportador OTLP, aplica-se a todos os sinais. Claude Code não tem protocolo padrão, então defina isso ou a variável de protocolo específica do sinal para cada exportador `otlp` que você ativar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `grpc`, `http/json`, `http/protobuf`                                                                                                                               |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Endpoint do coletor OTLP para todos os sinais                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `http://localhost:4317`                                                                                                                                            |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protocolo para métricas, substitui configuração geral                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `grpc`, `http/json`, `http/protobuf`                                                                                                                               |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Endpoint de métricas OTLP, substitui configuração geral                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `http://localhost:4318/v1/metrics`                                                                                                                                 |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protocolo para logs, substitui configuração geral                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `grpc`, `http/json`, `http/protobuf`                                                                                                                               |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Endpoint de logs OTLP, substitui configuração geral                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `http://localhost:4318/v1/logs`                                                                                                                                    |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Cabeçalhos de autenticação para OTLP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `Authorization=Bearer token`                                                                                                                                       |
| `OTEL_EXPORTER_OTLP_METRICS_HEADERS`                | Cabeçalhos de autenticação para métricas, mesclados com os cabeçalhos gerais                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `Authorization=Bearer token`                                                                                                                                       |
| `OTEL_EXPORTER_OTLP_LOGS_HEADERS`                   | Cabeçalhos de autenticação para logs, mesclados com os cabeçalhos gerais                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `Authorization=Bearer token`                                                                                                                                       |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Intervalo de exportação em milissegundos (padrão: 60000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `5000`, `60000`                                                                                                                                                    |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Intervalo de exportação de logs em milissegundos (padrão: 5000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `1000`, `10000`                                                                                                                                                    |
| `OTEL_LOG_USER_PROMPTS`                             | Ativar registro de conteúdo de prompt do usuário (padrão: desativado)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `1` para ativar                                                                                                                                                    |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Ativar registro de texto de resposta do assistente em eventos `assistant_response` (padrão: desativado). Quando não definido, volta para o valor de `OTEL_LOG_USER_PROMPTS`. Requer Claude Code v2.1.193 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `1` para ativar, `0` para manter reduzido                                                                                                                          |
| `OTEL_LOG_TOOL_DETAILS`                             | Ativar registro de parâmetros de ferramenta e argumentos de entrada em eventos de ferramenta e atributos de span de rastreamento: comandos Bash, nomes de servidor MCP e ferramenta, nomes de skill, nomes de workflow criados pelo usuário e entrada de ferramenta. Também ativa nomes de comando customizado, plugin e MCP em eventos `user_prompt` (padrão: desativado). Para servidores integrados do Claude Desktop, em sessões que Claude Desktop possui, `mcp_server_name`/`mcp_tool_name` são emitidos em `tool_decision`/`tool_result` mesmo com o sinalizador desativado. A exceção requer Claude Code v2.1.214 ou posterior                                                                                                                                               | `1` para ativar                                                                                                                                                    |
| `OTEL_LOG_TOOL_CONTENT`                             | Ativar registro de conteúdo de ferramenta no [evento de span `tool.output`](#tool-output-span-event) (padrão: desativado). Os atributos de span carregam conteúdo de ferramenta sob [seus próprios gates](#new-context-gates). Requer [rastreamento](#traces-beta). O conteúdo é truncado no limite de conteúdo (60 KB por padrão)                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `1` para ativar                                                                                                                                                    |
| `OTEL_LOG_MANAGED_SETTINGS`                         | Adicionar as configurações gerenciadas reduzidas e um resumo SHA-256 das configurações antes da redução aos eventos [managed settings resolved](#managed-settings-resolved-event) (padrão: desativado). Um valor em configurações de projeto ou local não o ativa. Requer Claude Code v2.1.274 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `1` para ativar                                                                                                                                                    |
| `OTEL_LOG_RAW_API_BODIES`                           | Emitir o corpo JSON completo da solicitação e resposta da API Anthropic Messages como eventos de log `api_request_body` / `api_response_body` (padrão: desativado). Os corpos incluem todo o histórico de conversa. Ativar isso implica consentimento para tudo que `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS` e `OTEL_LOG_TOOL_CONTENT` revelariam                                                                                                                                                                                                                                                                                                                                                                                                                            | `1` para corpos inline truncados no limite de conteúdo (60 KB por padrão), ou `file:<dir>` para corpos não truncados em disco com um ponteiro `body_ref` no evento |
| `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH`               | Limite de conteúdo: o comprimento máximo de atributos que contêm conteúdo, como respostas de modelo, conteúdo de ferramenta, prompts do sistema e corpos de API brutos, marcador de truncamento incluído, em unidades de código UTF-16 (padrão: 61440, ou seja, 60 KB). O padrão é dimensionado para backends que limitam valores de atributo a 64 KB; aumente-o apenas se seu backend aceitar valores maiores, ou diminua-o para reduzir o volume de telemetria. Quando um limite de atributo do SDK OpenTelemetry, `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` ou uma de suas variantes de logrecord e span, é definido como menor, Claude Code trunca nesse valor menor para que o marcador `[TRUNCATED ...]` permaneça dentro do limite do SDK. Requer Claude Code v2.1.214 ou posterior | `262144`                                                                                                                                                           |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Preferência de temporalidade de métricas (padrão: `delta`). Defina como `cumulative` se seu backend espera temporalidade cumulativa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `delta`, `cumulative`                                                                                                                                              |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervalo para atualizar cabeçalhos dinâmicos (padrão: 1740000ms / 29 minutos)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `900000`                                                                                                                                                           |

Para os protocolos `http/protobuf` e `http/json`, Claude Code envia cada solicitação de exportação com um cabeçalho `Content-Length`. Antes da v2.1.212, versões do Claude Code a partir da v2.1.191 enviavam essas solicitações com codificação de transferência em chunks; Azure Monitor e outros endpoints que exigem um comprimento declarado as rejeitavam com erros `411 Length Required` ou `400`.

<h3 id="mtls-authentication">
  Autenticação mTLS
</h3>

Como você configura certificados de cliente para o exportador OTLP depende do protocolo OTLP em uso para esse sinal, definido via `OTEL_EXPORTER_OTLP_PROTOCOL` ou a substituição por sinal. A mesma configuração se aplica a métricas, logs e rastreamentos.

| Protocolo                    | Variáveis de certificado do cliente                                                                                                                                                            | Confiar na CA do coletor com     |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY` e opcionalmente `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Veja [Configuração de rede](/docs/pt/network-config#mtls-authentication)                   | `NODE_EXTRA_CA_CERTS`            |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` e `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, ou as variantes por sinal como `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` para usar um certificado diferente por sinal | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

Para `grpc`, o SDK OpenTelemetry lê as variáveis OTLP padrão diretamente, então as configurações existentes que definem as variáveis de métricas por sinal continuam funcionando. Em máquinas com configurações gerenciadas, Claude Code [pode remover credenciais e endpoints por sinal definidos pelo desenvolvedor](#how-managed-settings-lock-the-otlp-destination) na inicialização.

<h3 id="metrics-cardinality-control">
  Controle de cardinalidade de métricas
</h3>

As seguintes variáveis de ambiente controlam quais atributos são incluídos nas métricas para gerenciar a cardinalidade:

| Variável de Ambiente                       | Descrição                                                                                                                                                              | Valor Padrão | Exemplo para Desativar |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ | ---------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Incluir atributo session.id em métricas                                                                                                                                | `true`       | `false`                |
| `OTEL_METRICS_INCLUDE_VERSION`             | Incluir atributo app.version em métricas                                                                                                                               | `false`      | `true`                 |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Incluir atributos user.account\_uuid e user.account\_id em métricas                                                                                                    | `true`       | `false`                |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Incluir atributo app.entrypoint em métricas                                                                                                                            | `false`      | `true`                 |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Incluir chaves de `OTEL_RESOURCE_ATTRIBUTES` como atributos em pontos de dados de métrica                                                                              | `true`       | `false`                |
| `OTEL_METRICS_INCLUDE_REPOSITORY`          | Incluir atributos de identidade de repositório `vcs.*` [repository attributes](#repository-attributes) em métricas e eventos. Requer Claude Code v2.1.269 ou posterior | `false`      | `true`                 |

Cardinalidade mais baixa geralmente significa melhor desempenho e custos de armazenamento mais baixos, mas dados menos granulares para análise.

<h3 id="traces-beta">
  Rastreamentos (beta)
</h3>

O rastreamento distribuído exporta spans que vinculam cada prompt do usuário às solicitações de API e execuções de ferramentas que ele dispara, para que você possa visualizar uma solicitação completa como um único rastreamento no seu backend de rastreamento.

O rastreamento está desativado por padrão. Para ativá-lo, defina tanto `CLAUDE_CODE_ENABLE_TELEMETRY=1` quanto `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, depois defina `OTEL_TRACES_EXPORTER` para escolher para onde os spans são enviados. Os rastreamentos reutilizam a [configuração OTLP comum](#common-configuration-variables) para endpoint, protocolo, cabeçalhos e [mTLS](#mtls-authentication). Em máquinas com configurações gerenciadas, Claude Code [pode remover credenciais e endpoints por sinal definidos pelo desenvolvedor](#how-managed-settings-lock-the-otlp-destination) na inicialização.

| Variável de Ambiente                  | Descrição                                                                                   | Valores de Exemplo                   |
| ------------------------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Ativar rastreamento de span (obrigatório). `ENABLE_ENHANCED_TELEMETRY_BETA` também é aceito | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Tipos de exportador de rastreamentos, separados por vírgula. Use `none` para desativar      | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protocolo para rastreamentos, substitui `OTEL_EXPORTER_OTLP_PROTOCOL`                       | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Endpoint de rastreamentos OTLP, substitui `OTEL_EXPORTER_OTLP_ENDPOINT`                     | `http://localhost:4318/v1/traces`    |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS`   | Cabeçalhos de autenticação para rastreamentos, mesclados com `OTEL_EXPORTER_OTLP_HEADERS`   | `Authorization=Bearer token`         |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Intervalo de exportação de lote de span em milissegundos (padrão: 5000)                     | `1000`, `10000`                      |

Os spans reduzem o texto do prompt do usuário, detalhes de entrada de ferramenta e conteúdo de ferramenta por padrão. Defina `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1` e `OTEL_LOG_TOOL_CONTENT=1` para incluí-los.

Quando o rastreamento está ativo, subprocessos Bash e PowerShell herdam automaticamente uma variável de ambiente `TRACEPARENT` contendo o contexto de rastreamento W3C do span de execução de ferramenta ativo. Isso permite que qualquer subprocesso que leia `TRACEPARENT` coloque seus próprios spans sob o mesmo rastreamento, permitindo rastreamento distribuído de ponta a ponta através de scripts e comandos que Claude executa.

Quando o rastreamento está ativo e Claude Code está conectado diretamente à API Anthropic, cada solicitação de modelo carrega um cabeçalho W3C `traceparent` definido para o contexto do span `claude_code.llm_request`, e o cabeçalho `traceresponse` da API é registrado como um link de span. Juntos, esses conectam os spans do lado do cliente do Claude Code ao rastreamento do lado do servidor através de qualquer intermediário compatível. As solicitações HTTP MCP de saída carregam `traceparent` da mesma forma. O cabeçalho não é enviado para provedores terceirizados.

Por padrão, o cabeçalho `traceparent` em solicitações de modelo e HTTP MCP é enviado apenas quando `ANTHROPIC_BASE_URL` não está definido ou aponta para a API Anthropic, já que alguns proxies rejeitam cabeçalhos não reconhecidos. A variável `TRACEPARENT` do subprocesso é controlada pelo mesmo switch para consistência. Se você executar Claude Code através de um proxy `ANTHROPIC_BASE_URL` customizado e quiser que o contexto de rastreamento seja propagado, defina `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

No Agent SDK e sessões não-interativas iniciadas com `-p`, Claude Code também lê `TRACEPARENT` e `TRACESTATE` de seu próprio ambiente ao iniciar cada span de interação. Isso permite que um processo de incorporação passe seu contexto de rastreamento W3C ativo para o subprocesso para que os spans do Claude Code apareçam como filhos do rastreamento distribuído do chamador. Sessões interativas ignoram `TRACEPARENT` de entrada para evitar herdar acidentalmente valores ambientes de CI ou ambientes de contêiner.

O contexto de rastreamento de entrada também se aplica a [eventos](#events). No Agent SDK e sessões `-p` com `TRACEPARENT` definido, cada registro de log de evento OTLP carrega valores `trace_id` e `span_id` que o unem ao rastreamento da sua aplicação, mesmo quando o exportador de rastreamentos não está configurado, para que seu backend de logging possa correlacionar eventos com o resto do rastreamento.

Um registro emitido enquanto uma interação está ativa carrega os IDs do span de interação, mesmo quando Claude Code o emite fora do contexto assíncrono do span, como em um callback de prompt de permissão ou para um registro armazenado em buffer durante a inicialização e exportado posteriormente. Um registro emitido sem nenhum span de interação ativo carrega os IDs `TRACEPARENT` de entrada diretamente. Antes da v2.1.214, registros emitidos fora do contexto assíncrono do span carregavam os IDs `TRACEPARENT` de entrada em vez dos IDs do span. Antes da v2.1.212, registros de eventos emitidos fora de um span ativo não carregavam `trace_id` ou `span_id`.

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

Quando um hook `PreToolUse` [adia uma chamada de ferramenta](/docs/pt/hooks#defer-a-tool-call-for-later), Claude Code salva o contexto de rastreamento do turno que o adiou. Quando você retoma a sessão e a ferramenta é executada novamente, os spans da ferramenta se unem ao rastreamento daquele turno anterior como filhos do span `claude_code.interaction` do turno.

<h4 id="span-attributes">
  Atributos de span
</h4>

Cada span carrega os [atributos padrão](#standard-attributes) mais um atributo `span.type` correspondendo ao seu nome. As tabelas abaixo listam os atributos adicionais definidos em cada span. Os spans `llm_request`, `tool.execution` e `hook` definem status OpenTelemetry `ERROR` quando registram uma falha; os outros spans sempre terminam com status `UNSET`.

**`claude_code.interaction`**

| Atributo                  | Descrição                                                                                                                                                                                  | Controlado Por          |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------- |
| `user_prompt`             | Texto do prompt. O valor é `<REDACTED>` a menos que o gate esteja definido                                                                                                                 | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Comprimento do prompt em caracteres                                                                                                                                                        |                         |
| `interaction.sequence`    | Contador baseado em 1 de interações, contado por processo Claude Code em vez de por sessão, conforme descrito para [`event.sequence`](#event-correlation-attributes)                       |                         |
| `parent.source`           | Como o span obteve seu pai de rastreamento: `env` quando foi pai sob um `TRACEPARENT` de entrada, `none` quando iniciou seu próprio rastreamento. Requer Claude Code v2.1.268 ou posterior |                         |
| `interaction.duration_ms` | Duração de parede do turno                                                                                                                                                                 |                         |

**`claude_code.llm_request`**

| Atributo                         | Descrição                                                                                                                                                                                                                                                                                               | Controlado Por                 |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `model`                          | Identificador do modelo                                                                                                                                                                                                                                                                                 |                                |
| `gen_ai.system`                  | Sempre `anthropic`. Convenção semântica GenAI OpenTelemetry                                                                                                                                                                                                                                             |                                |
| `gen_ai.request.model`           | Mesmo valor que `model`. Convenção semântica GenAI OpenTelemetry                                                                                                                                                                                                                                        |                                |
| `query_source`                   | Subsistema que emitiu a solicitação, como `repl_main_thread` ou um nome de subagente                                                                                                                                                                                                                    | `ENABLE_BETA_TRACING_DETAILED` |
| `query_source_safe`              | Forma limitada de `query_source`, emitida independentemente de rastreamento beta detalhado estar ativo, com valores como `repl_main_thread` ou `agent.builtin.general-purpose`. `:` se torna `.` e agentes nomeados pelo usuário aparecem como `agent.custom`. Requer Claude Code v2.1.268 ou posterior |                                |
| `agent_id`                       | Identificador do subagente ou colega que emitiu a solicitação. Ausente na sessão principal                                                                                                                                                                                                              |                                |
| `parent_agent_id`                | Identificador do agente que gerou este. Ausente para a sessão principal e para agentes gerados diretamente a partir dela                                                                                                                                                                                |                                |
| `workflow.run_id`                | Identificador de execução da ferramenta [Workflow](/docs/pt/workflows) que gerou este agente, prefixado `wf_`. Ausente para agentes não gerados por um workflow                                                                                                                                              |                                |
| `workflow.name`                  | Nome do workflow que gerou este agente. Nomes criados pelo usuário são substituídos por `custom` a menos que o gate esteja definido                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS`        |
| `speed`                          | `fast` ou `normal`                                                                                                                                                                                                                                                                                      |                                |
| `effort`                         | [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à solicitação: `low`, `medium`, `high`, `xhigh` ou `max`. Ausente quando Claude Code não envia nível de esforço, por exemplo em um modelo que não suporta esforço. Requer Claude Code v2.1.274 ou posterior                           |                                |
| `llm_request.context`            | `interaction`, `tool` ou `standalone` dependendo do span pai                                                                                                                                                                                                                                            |                                |
| `duration_ms`                    | Duração de parede incluindo tentativas                                                                                                                                                                                                                                                                  |                                |
| `ttft_ms`                        | Tempo até o primeiro token em milissegundos                                                                                                                                                                                                                                                             |                                |
| `first_content_ms`               | Tempo desde o início da solicitação até o primeiro bloco de conteúdo da tentativa bem-sucedida, em milissegundos. Ausente em solicitações que voltaram para o caminho não-streaming. Requer Claude Code v2.1.268 ou posterior                                                                           |                                |
| `input_tokens`                   | Contagem de tokens de entrada do bloco de uso da API                                                                                                                                                                                                                                                    |                                |
| `output_tokens`                  | Contagem de tokens de saída                                                                                                                                                                                                                                                                             |                                |
| `cache_read_tokens`              | Tokens lidos do cache de prompt                                                                                                                                                                                                                                                                         |                                |
| `cache_creation_tokens`          | Tokens escritos no cache de prompt                                                                                                                                                                                                                                                                      |                                |
| `request_id`                     | ID de solicitação da API Anthropic do cabeçalho de resposta `request-id`                                                                                                                                                                                                                                |                                |
| `gen_ai.response.id`             | Mesmo valor que `request_id`. Convenção semântica GenAI OpenTelemetry                                                                                                                                                                                                                                   |                                |
| `client_request_id`              | `x-client-request-id` gerado pelo cliente da tentativa final                                                                                                                                                                                                                                            |                                |
| `attempt`                        | Total de tentativas feitas para esta solicitação                                                                                                                                                                                                                                                        |                                |
| `success`                        | `true` ou `false`                                                                                                                                                                                                                                                                                       |                                |
| `status_code`                    | Código de status HTTP quando a solicitação falhou                                                                                                                                                                                                                                                       |                                |
| `error`                          | Mensagem de erro quando a solicitação falhou                                                                                                                                                                                                                                                            |                                |
| `error_class`                    | Token de classe de erro curto quando a solicitação falhou, como `api_timeout` ou `server_overload`. Requer Claude Code v2.1.268 ou posterior                                                                                                                                                            |                                |
| `response.has_tool_call`         | `true` quando a resposta continha blocos de uso de ferramenta                                                                                                                                                                                                                                           |                                |
| `stop_reason`                    | API response `stop_reason`, como `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn` ou `refusal`                                                                                                                                                                                       |                                |
| `gen_ai.response.finish_reasons` | Mesmo valor que `stop_reason`, envolvido em um array de string. Convenção semântica GenAI OpenTelemetry                                                                                                                                                                                                 |                                |

Cada tentativa de repetição também é registrada como um evento de span `gen_ai.request.attempt` com atributos `attempt` e `client_request_id`.

**`claude_code.tool`**

| Atributo              | Descrição                                                                                                                                                                                                                                                                                                                                                       | Controlado Por          |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | Nome da ferramenta                                                                                                                                                                                                                                                                                                                                              |                         |
| `tool_name_safe`      | Forma de `tool_name` que não carrega nomes escolhidos pelo usuário. Nomes de ferramentas integradas passam verbatim. Nomes de ferramentas MCP aparecem como `mcp_other`, exceto nomes de ferramentas que correspondem a algumas formas fixas, como ferramentas `playwright` nomeadas `browser_*`, que passam verbatim. Requer Claude Code v2.1.268 ou posterior |                         |
| `bash_command_class`  | Para a ferramenta Bash: categoria do primeiro programa do comando de uma lista fixa, como `vcs` ou `package_manager`. `other` para um programa fora da lista, `unparsed` quando a linha não pode ser analisada. Requer Claude Code v2.1.268 ou posterior                                                                                                        |                         |
| `bash_argv0`          | Para a ferramenta Bash: o primeiro programa do comando quando está na mesma lista fixa, como `git` ou `npm`. `other` para qualquer programa fora da lista. Requer Claude Code v2.1.268 ou posterior                                                                                                                                                             |                         |
| `duration_ms`         | Duração de parede incluindo espera de permissão e execução                                                                                                                                                                                                                                                                                                      |                         |
| `result_tokens`       | Tamanho aproximado em tokens do resultado da ferramenta                                                                                                                                                                                                                                                                                                         |                         |
| `agent_id`            | Identificador do subagente ou colega que executou a ferramenta. Ausente na sessão principal                                                                                                                                                                                                                                                                     |                         |
| `parent_agent_id`     | Identificador do agente que gerou este. Ausente para a sessão principal e para agentes gerados diretamente a partir dela                                                                                                                                                                                                                                        |                         |
| `workflow.run_id`     | Identificador de execução da ferramenta Workflow que gerou este agente, prefixado `wf_`. Ausente para agentes não gerados por um workflow                                                                                                                                                                                                                       |                         |
| `workflow.name`       | Nome do workflow que gerou este agente. Nomes criados pelo usuário são substituídos por `custom` a menos que o gate esteja definido                                                                                                                                                                                                                             | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | O id do bloco `tool_use` do modelo para esta chamada. Corresponde ao `tool_use_id` nos eventos [tool\_result](#tool-result-event) e [tool\_decision](#tool-decision-event) e nas cargas de hook, para que você possa unir o span a esses registros                                                                                                              |                         |
| `gen_ai.tool.call.id` | Mesmo valor que `tool_use_id`. Convenção semântica GenAI OpenTelemetry                                                                                                                                                                                                                                                                                          |                         |
| `file_path`           | Caminho de arquivo alvo para ferramentas Read, Edit e Write                                                                                                                                                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | String de comando para a ferramenta Bash                                                                                                                                                                                                                                                                                                                        | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nome da skill para a ferramenta Skill                                                                                                                                                                                                                                                                                                                           | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Tipo de subagente para a ferramenta Agent ou ferramenta Task legada                                                                                                                                                                                                                                                                                             | `OTEL_LOG_TOOL_DETAILS` |

<span id="tool-output-span-event" />**`tool.output` span event on `claude_code.tool`**

Se você definir `OTEL_LOG_TOOL_CONTENT=1`, chamadas Read e Bash podem registrar um evento de span `tool.output` no span `claude_code.tool`. Chamadas Edit e Write registram um apenas quando você também define `OTEL_LOG_TOOL_DETAILS=1`. Essa variável não é limitada a essas duas ferramentas, então verifique sua [linha na tabela de configuração](#common-configuration-variables) para os argumentos que ela adiciona em outro lugar.

Claude Code escreve este evento do retorno bem-sucedido de uma chamada de ferramenta, então uma chamada que gera um erro não registra nada, qualquer que seja a ferramenta. Entre as chamadas que retornam, ela não registra nenhum evento `tool.output` para:

* Uma chamada para qualquer ferramenta que não seja Read, Edit, Write e Bash, incluindo ferramentas MCP e WebFetch
* Um Read que retorna qualquer coisa que não seja texto de arquivo, como uma imagem, um PDF ou uma releitura de um arquivo cujo conteúdo não mudou
* Uma chamada Edit ou Write, a menos que você também defina `OTEL_LOG_TOOL_DETAILS=1`

O evento carrega esses atributos, cada um truncado no limite de conteúdo (60 KB por padrão). `Controlado Por` nomeia a variável que um atributo precisa além de `OTEL_LOG_TOOL_CONTENT=1`, e para Edit e Write essa variável controla o evento em si em vez do atributo.

| Atributo       | Descrição                                                                                                  | Controlado Por                                  |
| -------------- | ---------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| `content`      | Texto que a ferramenta Read retornou, ou o texto que uma chamada Write foi solicitada a escrever           | `OTEL_LOG_TOOL_DETAILS` para a ferramenta Write |
| `output`       | Saída combinada de um comando Bash, com stderr intercalado em stdout                                       |                                                 |
| `diff`         | Patch estruturado que a ferramenta Edit aplicou                                                            | `OTEL_LOG_TOOL_DETAILS`                         |
| `file_path`    | Caminho de arquivo alvo para as ferramentas Read, Edit e Write, repetindo o atributo de span do mesmo nome | `OTEL_LOG_TOOL_DETAILS`                         |
| `bash_command` | String de comando para a ferramenta Bash                                                                   | `OTEL_LOG_TOOL_DETAILS`                         |

O atributo `tool_name` do span pai informa qual ferramenta um evento veio. Um atributo cortado no limite de conteúdo é acompanhado por `<attribute>_truncated` e `<attribute>_original_length`.

**`claude_code.tool.blocked_on_user`**

| Atributo      | Descrição                                                                                   | Controlado Por |
| ------------- | ------------------------------------------------------------------------------------------- | -------------- |
| `duration_ms` | Tempo gasto esperando a decisão de permissão                                                |                |
| `decision`    | `accept` ou `reject`                                                                        |                |
| `source`      | Fonte de decisão, correspondendo ao [Evento de decisão da ferramenta](#tool-decision-event) |                |

**`claude_code.tool.execution`**

| Atributo              | Descrição                                                                                                                                                                                                                                                                     | Controlado Por          |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Tempo gasto executando o corpo da ferramenta                                                                                                                                                                                                                                  |                         |
| `tool_use_id`         | Mesmo valor que no span pai `claude_code.tool`                                                                                                                                                                                                                                |                         |
| `gen_ai.tool.call.id` | Mesmo valor que `tool_use_id`. Convenção semântica GenAI OpenTelemetry                                                                                                                                                                                                        |                         |
| `success`             | `true` ou `false`                                                                                                                                                                                                                                                             |                         |
| `error`               | String de categoria de erro quando a execução falhou, como `Error:ENOENT` ou `ShellError`. Contém a mensagem de erro completa em vez disso quando o gate está definido                                                                                                        | `OTEL_LOG_TOOL_DETAILS` |
| `error_class`         | A categoria de erro em forma de identificador, com caracteres fora de letras, dígitos e sublinhados substituídos por `_`, como `Error_ENOENT` ou `ShellError`. Carrega a categoria mesmo quando `error` carrega a mensagem completa. Requer Claude Code v2.1.268 ou posterior |                         |

**`claude_code.hook`**

Este span aparece apenas quando rastreamento beta detalhado está ativo, o que requer `ENABLE_BETA_TRACING_DETAILED=1` e `BETA_TRACING_ENDPOINT`, um par que também [muda para onde seus logs e rastreamentos vão](/docs/pt/env-vars#variables). Defina o par em seu shell, configurações de usuário ou configurações gerenciadas; ambas as variáveis são ignoradas em [configurações de projeto e local](/docs/pt/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` sozinho não o produz.

Em sessões CLI interativas, rastreamento beta detalhado também requer que sua organização esteja na lista de permissões para o recurso. Sessões Agent SDK e não-interativas `-p` não requerem lista de permissões.

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

<span id="new-context-gates" />

<Note>
  Atributos adicionais que contêm conteúdo, como `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input` e `response.model_output`, são emitidos apenas quando rastreamento beta detalhado está ativo. Eles não fazem parte do esquema de span estável.

  O gate em `new_context` depende de qual span o carrega, e cada cópia é truncada no limite de conteúdo (60 KB por padrão). No span `claude_code.tool` ele carrega o resultado dessa chamada de ferramenta, qualquer que seja a ferramenta, e requer `OTEL_LOG_TOOL_CONTENT=1`. No span `claude_code.interaction` ele carrega o prompt do usuário, e no span `claude_code.llm_request` as novas mensagens do usuário e resultados de ferramenta dessa solicitação. Ambos requerem `OTEL_LOG_USER_PROMPTS=1`.

  `user_system_prompt` também requer `OTEL_LOG_USER_PROMPTS=1`. Ele carrega apenas o texto do prompt do sistema que você fornece através da opção SDK `systemPrompt` ou dos sinalizadores `--system-prompt` e `--append-system-prompt`, truncado no limite de conteúdo (60 KB por padrão), e é emitido uma vez por sessão em vez de por solicitação.
</Note>

<h3 id="dynamic-headers">
  Cabeçalhos dinâmicos
</h3>

Para ambientes corporativos que exigem autenticação dinâmica, você pode configurar um script para gerar cabeçalhos dinamicamente. Cabeçalhos dinâmicos se aplicam apenas aos protocolos `http/protobuf` e `http/json`. Com o protocolo `grpc`, Claude Code usa apenas as variáveis de cabeçalhos estáticos, `OTEL_EXPORTER_OTLP_HEADERS` e suas variantes por sinal.

<h4 id="settings-configuration">
  Configuração de configurações
</h4>

Adicione ao seu `.claude/settings.json`, substituindo o caminho pelo seu próprio script:

```json theme={null}
{
  "otelHeadersHelper": "/path/to/generate-otel-headers.sh"
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

Se o auxiliar falhar ou imprimir saída que não atenda a esses requisitos, as exportações falham e seu backend de telemetria não recebe nada da sessão até que o auxiliar funcione novamente. Claude Code relata a falha em:

* Uma notificação de aviso em sessões interativas, [`otelHeadersHelper failed; telemetry is not being exported`](/docs/pt/errors#otelheadershelper-failed), mostrada uma vez por sessão quando o auxiliar falha pela primeira vez
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

Claude Code anexa esses valores como atributos em cada ponto de dados de métrica e registro de evento, além de enviá-los no bloco de recurso OTLP. Como a maioria dos backends de métricas expõe atributos de ponto de dados como rótulos consultáveis, você pode agrupar e filtrar métricas por suas chaves personalizadas diretamente. Exceto pelos atributos de repositório `vcs.*` [repository attributes](#repository-attributes), chaves personalizadas nunca substituem os [atributos padrão](#standard-attributes) como `user.id` ou `session.id`: quando uma chave colide, Claude Code mantém o valor integrado.

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

Defina essas variáveis de ambiente antes de executar `claude`. Cada cenário abaixo mostra uma configuração completa, e cada variável é descrita em [Variáveis de configuração comuns](#common-configuration-variables). Para confirmar que uma configuração entrou em vigor, verifique seu backend para a métrica `claude_code.session.count` após iniciar uma sessão; o [Início rápido](#quick-start) cobre verificação apenas de logs e o que verificar quando nada chega.

Para depuração de console com intervalo de exportação de 1 segundo:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000
```

Para OTLP sobre gRPC:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Para Prometheus, raspado de `http://localhost:9464/metrics`:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus
```

Em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments-reference#pass-through-session-child-metrics), a sessão vincula a porta 9464 apenas na capacidade padrão do runner de um. Em capacidade mais alta, o runner re-expõe contadores e medidores de sessão em seu próprio endpoint `/metrics` em vez disso.

Para enviar métricas para múltiplos exportadores:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json
```

Para enviar métricas e logs para diferentes endpoints ou backends:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317
```

Para exportar apenas métricas, sem eventos ou logs:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Para exportar apenas eventos e logs, sem métricas:

```bash theme={null}
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

Todas as métricas e eventos compartilham estes atributos padrão:

| Atributo                                                                                | Descrição                                                                                                                                                                                                                                       | Controlado por                                                                              |
| --------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `session.id`                                                                            | Identificador único de sessão                                                                                                                                                                                                                   | `OTEL_METRICS_INCLUDE_SESSION_ID` (padrão: true)                                            |
| `app.version`                                                                           | Versão atual do Claude Code                                                                                                                                                                                                                     | `OTEL_METRICS_INCLUDE_VERSION` (padrão: false)                                              |
| `app.entrypoint`                                                                        | Como a sessão foi iniciada, como `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, ou `claude-vscode`                                                                                                                                                       | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (padrão: false)                                           |
| `organization.id`                                                                       | UUID da organização (quando autenticado)                                                                                                                                                                                                        | Sempre incluído quando disponível                                                           |
| `user.account_uuid`                                                                     | UUID da conta (quando autenticado)                                                                                                                                                                                                              | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (padrão: true)                                          |
| `user.account_id`                                                                       | ID da conta em formato marcado correspondendo às APIs de administração da Anthropic (quando autenticado), como `user_01BWBeN28...`                                                                                                              | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (padrão: true)                                          |
| `user.id`                                                                               | Identificador anônimo aleatório gerado na primeira execução e persistido em `~/.claude.json`. Não contém informações pessoais e não é derivado da sua conta Claude. Deletar o arquivo produz um novo valor não relacionado na próxima execução. | Sempre incluído                                                                             |
| `user.email`                                                                            | Endereço de email do usuário, do seu login ou, em uma [sessão na nuvem](/docs/pt/claude-code-on-the-web), das credenciais da própria sessão                                                                                                          | Sempre incluído quando disponível                                                           |
| `terminal.type`                                                                         | Tipo de terminal, como `iTerm.app`, `vscode`, `cursor`, ou `tmux`                                                                                                                                                                               | Sempre incluído quando detectado                                                            |
| Chaves de `OTEL_RESOURCE_ATTRIBUTES`                                                    | Atributos personalizados que você define, como `department` ou `team.id`. Veja [Suporte a organização multi-equipe](#multi-team-organization-support)                                                                                           | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (padrão: true)                                   |
| `vcs.repository.url.full`, `vcs.owner.name`, `vcs.repository.name`, `vcs.provider.name` | A identidade do repositório da sessão, derivada do seu remote `origin`. Veja [Atributos de repositório](#repository-attributes)                                                                                                                 | `OTEL_METRICS_INCLUDE_REPOSITORY` (padrão: false). Requer Claude Code v2.1.269 ou posterior |

Quando Claude Code está conectado a um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), a CLI marca as exportações com a identidade autenticada da sessão do gateway: `user.id` é o assunto do IdP em vez de um identificador de instalação anônimo, `user.email` é o email conectado, e `user.groups` carrega a associação de grupo do IdP como uma string separada por vírgulas. Cada exportação também carrega `identity.source: gateway-oidc`. A identidade do gateway é aplicada por último, então as chaves `user.*` e `identity.*` definidas através de `OTEL_RESOURCE_ATTRIBUTES` são ignoradas em sessões de gateway.

Os eventos incluem adicionalmente os seguintes atributos. Estes nunca são anexados a métricas porque causariam cardinalidade ilimitada:

* `prompt.id`: UUID correlacionando um prompt do usuário com todos os eventos subsequentes até o próximo prompt. Veja [Atributos de correlação de eventos](#event-correlation-attributes).
* `workspace.host_paths`: diretórios do workspace do host selecionados no aplicativo desktop, como um array de strings
* `workflow.run_id`: identificador de execução, prefixado com `wf_`, nos eventos de API e ferramenta emitidos por agentes que pertencem a uma execução de ferramenta [Workflow](/docs/pt/workflows). Filtrar eventos por um `workflow.run_id` reconstrói as requisições de API e resultados de ferramentas dessa execução. O identificador cobre os agentes que o script de workflow gera e quaisquer agentes que esses gerem por sua vez, como invocações de skills. Corresponde ao identificador de execução relatado no resultado da ferramenta Workflow. Ausente em todos os outros eventos. Requer Claude Code v2.1.202 ou posterior
* `workflow.name`: nome do workflow, o `meta.name` do seu script, emitido junto com `workflow.run_id`. Os nomes de workflow integrados aparecem literalmente quando a execução executa o script integrado não modificado. Nomes de autoria do usuário, incluindo cópias editadas de scripts integrados, são substituídos por `custom` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido. Requer Claude Code v2.1.202 ou posterior

<h4 id="repository-attributes">
  Atributos de repositório
</h4>

Defina `OTEL_METRICS_INCLUDE_REPOSITORY=true` para marcar métricas e eventos com a identidade do repositório da sessão, para que um coletor compartilhado possa atribuir uso por repositório. Requer Claude Code v2.1.269 ou posterior.

Claude Code deriva esses atributos uma vez por sessão do remote `origin` do repositório. Os remotes HTTPS e SSH de um repositório produzem valores idênticos:

| Atributo                  | Valor                                                                                                                                                      |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `vcs.repository.url.full` | A URL do navegador do repositório sem `.git`, como `https://github.com/example-org/example-repo`                                                           |
| `vcs.owner.name`          | O caminho do proprietário ou grupo, como `example-org`; omitido quando o caminho remoto tem um único segmento                                              |
| `vcs.repository.name`     | O nome do repositório simples, como `example-repo`                                                                                                         |
| `vcs.provider.name`       | `github`, `gitlab`, `bitbucket`, ou `gitea` quando Claude Code reconhece o host remoto ou a forma da URL como um desses provedores; omitido caso contrário |

Os valores são convertidos para minúsculas, e credenciais, strings de consulta e fragmentos da URL remota nunca aparecem neles. Os atributos são omitidos quando a sessão não tem um remote `origin`, quando o remote não é em forma de URL, ou quando o único repositório envolvente é seu diretório home.

Uma chave `vcs.*` que você declara em [`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support) substitui o valor derivado para essa chave. Se você declarar `vcs.repository.url.full`, Claude Code nunca lê o remote e relata apenas as chaves que você declara.

Os atributos fluem apenas para seus próprios exportadores; a telemetria da Anthropic descarta todas as chaves `vcs.*`.

<h3 id="metrics">
  Métricas
</h3>

Claude Code exporta as seguintes métricas. A coluna Unit mostra a string de unidade OpenTelemetry anexada a cada métrica; métricas de contagem não carregam nenhuma.

| Nome da Métrica                       | Descrição                                                           | Unidade |
| ------------------------------------- | ------------------------------------------------------------------- | ------- |
| `claude_code.session.count`           | Contagem de sessões CLI iniciadas                                   | nenhuma |
| `claude_code.lines_of_code.count`     | Contagem de linhas de código modificadas                            | nenhuma |
| `claude_code.pull_request.count`      | Número de pull requests criadas                                     | nenhuma |
| `claude_code.commit.count`            | Número de commits git criados                                       | nenhuma |
| `claude_code.cost.usage`              | Custo da sessão Claude Code                                         | USD     |
| `claude_code.token.usage`             | Número de tokens usados                                             | tokens  |
| `claude_code.code_edit_tool.decision` | Contagem de decisões de permissão da ferramenta de edição de código | nenhuma |
| `claude_code.active_time.total`       | Tempo ativo total                                                   | s       |

Quando `prometheus` é o único exportador listado em `OTEL_METRICS_EXPORTER`, Claude Code omite as unidades `USD`, `tokens`, e `s` das métricas exportadas para que o scrape permaneça em formato de texto Prometheus válido. Os nomes das métricas não mudam, e configurações que combinam exportadores, como `otlp,prometheus`, mantêm as unidades. Antes da v2.1.216, o scrape do Prometheus incluía linhas `# UNIT` apenas do OpenMetrics que alguns scrapers rejeitavam.

<h3 id="metric-details">
  Detalhes das métricas
</h3>

Cada métrica inclui os atributos padrão listados acima. Métricas com atributos adicionais específicos do contexto são anotadas abaixo.

<h4 id="session-counter">
  Contador de sessão
</h4>

Incrementado no início de cada sessão.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `start_type`: Como a sessão foi iniciada. Um de `"fresh"`, `"resume"`, `"continue"`, ou `"agents_view"`. O valor `"agents_view"` identifica o processo do dashboard `claude agents`, uma UI local iniciada pelo usuário em vez de uma sessão conversacional. Filtre neste valor para separar inicializações de processo de UI de sessões conversacionais em seus dashboards.

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

Incrementado quando Claude Code cria uma pull request ou merge request através de um comando shell ou uma ferramenta MCP.

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

Incrementado após cada requisição de API.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `model`: Identificador do modelo (por exemplo, "claude-sonnet-5")
* `query_source`: Categoria do subsistema que emitiu a requisição. Um de `"main"`, `"subagent"`, ou `"auxiliary"`
* `speed`: `"fast"` quando a requisição usou modo rápido. Ausente caso contrário
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à requisição: `"low"`, `"medium"`, `"high"`, `"xhigh"`, ou `"max"`. Ausente quando Claude Code não envia nível de esforço, por exemplo em um modelo que não suporta esforço.
* `agent.name`: Tipo de subagente que emitiu a requisição. Nomes de agentes integrados e agentes de plugins do marketplace oficial aparecem literalmente. Outros nomes de agentes definidos pelo usuário são substituídos por `"custom"` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido. Ausente quando a requisição não foi emitida por um tipo de subagente nomeado.
* `skill.name`: Skill ativa para a requisição, definida pela ferramenta Skill, um comando `/`, ou herdada por um subagente gerado. Nomes de skills integrados, agrupados, definidos pelo usuário e de plugins do marketplace oficial aparecem literalmente. Nomes de skills de plugins de terceiros são substituídos por `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido. Ausente quando nenhuma skill está ativa.
* `plugin.name`: Plugin proprietário quando a skill ativa ou subagente é fornecido por um plugin. Nomes de plugins do marketplace oficial aparecem literalmente. Nomes de plugins de terceiros são substituídos por `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido. Ausente quando nem a skill nem o subagente tem um plugin proprietário.
* `marketplace.name`: Marketplace do qual o plugin proprietário foi instalado. Emitido apenas para plugins do marketplace oficial. Ausente caso contrário.
* `mcp_server.name`: Servidor MCP cujo resultado de ferramenta esta requisição consumiu. Nomes de servidores integrados, proxied por claude.ai e do registro oficial aparecem literalmente. Nomes de servidores configurados pelo usuário são substituídos por `"custom"` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido. Ausente quando a requisição não consumiu resultado de ferramenta MCP. Antes da v2.1.222, Claude Code definia este atributo em cada requisição após uma chamada de ferramenta MCP, não apenas em requisições que consumiram um resultado de ferramenta, então dashboards que o agregam mostram uma queda após você atualizar.
* `mcp_tool.name`: Ferramenta MCP cujo resultado esta requisição consumiu, com o mesmo comportamento de redação e versão que `mcp_server.name`. Ausente quando a requisição não consumiu resultado de ferramenta MCP.

<h4 id="token-counter">
  Contador de tokens
</h4>

Incrementado após cada requisição de API.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: Identificador do modelo (por exemplo, "claude-sonnet-5")
* `query_source`: Categoria do subsistema que emitiu a requisição. Um de `"main"`, `"subagent"`, ou `"auxiliary"`
* `speed`: `"fast"` quando a requisição usou modo rápido. Ausente caso contrário
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à requisição. Veja [Contador de custo](#cost-counter) para detalhes.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribuição de skill, plugin, agente e MCP para a requisição. Veja [Contador de custo](#cost-counter) para definições e comportamento de redação.

<h4 id="code-edit-tool-decision-counter">
  Contador de decisão da ferramenta de edição de código
</h4>

Incrementado quando o usuário aceita ou rejeita o uso da ferramenta Edit, Write, ou NotebookEdit.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `tool_name`: Nome da ferramenta (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Decisão do usuário (`"accept"`, `"reject"`)
* `source`: De onde a decisão veio. Um de `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, ou `"user_reject"`. Veja o [evento de decisão de ferramenta](#tool-decision-event) para o que cada valor significa.
* `language`: Linguagem de programação do arquivo editado, como `"TypeScript"`, `"Python"`, `"JavaScript"`, ou `"Markdown"`. Retorna `"unknown"` para extensões de arquivo não reconhecidas.

<h4 id="active-time-counter">
  Contador de tempo ativo
</h4>

Rastreia o tempo real gasto usando ativamente Claude Code, excluindo tempo ocioso. Esta métrica é incrementada durante interações do usuário, como digitação e leitura de respostas, e durante processamento da CLI, como execução de ferramentas e geração de resposta de IA.

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `type`: `"user"` para interações de teclado, `"cli"` para execução de ferramentas e respostas de IA

<h3 id="events">
  Eventos
</h3>

Claude Code exporta os seguintes eventos via logs/eventos OpenTelemetry (quando `OTEL_LOGS_EXPORTER` está configurado):

<h4 id="event-correlation-attributes">
  Atributos de correlação de eventos
</h4>

Quando um usuário envia um prompt, Claude Code pode fazer múltiplas chamadas de API e executar várias ferramentas. O atributo `prompt.id` permite vincular todos esses eventos de volta ao único prompt que os acionou.

| Atributo            | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt.id`         | Identificador UUID v4 vinculando todos os eventos produzidos ao processar um único prompt do usuário                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `event.sequence`    | Contador baseado em 0 para ordenar eventos, contado por processo Claude Code em vez de por sessão                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `message.uuid`      | UUID da mensagem conforme persistida na transcrição da sessão, os arquivos `~/.claude/projects/*/*.jsonl`. Presente em `assistant_response`, em `api_response_body`, e em `user_prompt` exceto para dispatches de comando, que podem produzir zero ou muitas mensagens. Em `assistant_response` e `api_response_body`, esta é a entrada final da transcrição da resposta, da qual o `parentUuid` do próximo turno se encadeia. Requer Claude Code v2.1.214 ou posterior, ou v2.1.274 ou posterior em `api_response_body`                           |
| `client_request_id` | UUID gerado pelo cliente enviado como o header de requisição `x-client-request-id`. Presente em `api_request` e `api_error` em conexões de API de primeira parte; ausente em backends de provedores de terceiros e quando a requisição foi retentada através do fallback não-streaming. Emparelha uma requisição com sua resposta e permanece disponível para falhas como timeouts que nunca produziram um `request_id` do servidor. Corresponde ao mesmo atributo no span de rastreamento `llm_request`. Requer Claude Code v2.1.214 ou posterior |

Para rastrear toda atividade acionada por um único prompt, filtre seus eventos por um valor específico de `prompt.id`. Isto retorna o evento user\_prompt, quaisquer eventos api\_request, e quaisquer eventos tool\_result que ocorreram ao processar esse prompt.

`event.sequence` começa em 0 cada vez que um processo Claude Code inicia e conta para cima pela vida desse processo. Continua contando através de `/clear`, que atribui um novo `session.id`. Se você [retomar uma sessão sem fazer fork](/docs/pt/how-claude-code-works#resume-or-fork-sessions), a sessão mantém seu `session.id` mas toma seus valores de `event.sequence` do processo que a retomou, então dentro de uma sessão um evento posterior pode carregar um valor menor que um anterior, ou repetir um. Para ordenar os eventos de uma sessão, ordene por `event.timestamp` e use `event.sequence` para ordenar eventos que compartilham um timestamp.

Para reconstrução em nível de mensagem, cada classe de evento carrega uma chave que corresponde a um campo na transcrição da sessão. O formato de entrada da transcrição é [interno ao Claude Code](/docs/pt/sessions#where-transcripts-are-stored) e muda entre versões, então um pipeline que se une nestes campos pode quebrar em qualquer release; trate as uniões como específicas da versão em vez de um contrato estável:

* `message.uuid` em `user_prompt`, `assistant_response`, e `api_response_body`
* `request_id` nos eventos de API, persistido como `requestId` nas entradas de assistente da transcrição
* `tool_use_id` em eventos `tool_result` e `tool_decision`

<h4 id="user-prompt-event">
  Evento de prompt do usuário
</h4>

Registrado quando um usuário envia um prompt.

**Nome do Evento**: `claude_code.user_prompt`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `prompt_length`: Comprimento do prompt
* `prompt`: Conteúdo do prompt. Redatado por padrão. Defina `OTEL_LOG_USER_PROMPTS=1` para incluí-lo
* `message.uuid`: UUID da mensagem do usuário resultante, correspondendo à entrada da transcrição persistida. Ausente em dispatches de comando, que podem produzir zero ou muitas mensagens. Requer Claude Code v2.1.214 ou posterior
* `command_name`: Nome do comando quando o prompt invoca um. Nomes de comando integrados e agrupados como `compact` ou `debug` são emitidos como estão; aliases como `reset` emitem conforme digitado em vez do nome canônico. Nomes de comando personalizados, de plugin e MCP colapsam para `custom` ou `mcp` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido
* `command_source`: Origem do comando quando presente: `builtin`, `custom`, ou `mcp`. Comandos fornecidos por plugin relatam como `custom`

<h4 id="assistant-response-event">
  Evento de resposta do assistente
</h4>

Registrado após cada requisição de API que retorna conteúdo de texto do modelo. Apenas os blocos de texto da resposta são incluídos; blocos de pensamento e blocos de uso de ferramenta são excluídos. Requer Claude Code v2.1.193 ou posterior.

**Nome do Evento**: `claude_code.assistant_response`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `response_length`: Comprimento do texto de resposta em caracteres
* `response`: Texto de resposta, truncado no limite de conteúdo (60 KB por padrão). Redatado para `<REDACTED>` por padrão. Defina `OTEL_LOG_ASSISTANT_RESPONSES=1` para incluí-lo. Quando `OTEL_LOG_ASSISTANT_RESPONSES` não está definido, `OTEL_LOG_USER_PROMPTS` o controla em vez disso, então defina `OTEL_LOG_ASSISTANT_RESPONSES=0` para manter respostas redatadas enquanto o log de prompt está ativado
* `model`: Identificador do modelo (por exemplo, "claude-sonnet-5")
* `request_id`: ID de requisição de API da Anthropic do header `request-id` da resposta. Presente apenas quando a API retorna um
* `message.uuid`: UUID da entrada final da transcrição da resposta. Uma resposta de API é persistida como uma entrada de transcrição por bloco de conteúdo; esta é a última, da qual o `parentUuid` do próximo turno se encadeia. Requer Claude Code v2.1.214 ou posterior
* `query_source`: Subsistema que emitiu a requisição, como `"repl_main_thread"`, `"compact"`, ou um nome de subagente

<h4 id="tool-result-event">
  Evento de resultado de ferramenta
</h4>

Registrado quando uma ferramenta completa a execução. Não emitido se a chamada de ferramenta foi rejeitada; veja o [evento de decisão de ferramenta](#tool-decision-event) para rejeições.

**Nome do Evento**: `claude_code.tool_result`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `tool_name`: Nome da ferramenta
* `tool_use_id`: Identificador único para esta invocação de ferramenta. Corresponde ao `tool_use_id` passado para hooks, permitindo correlação entre eventos OTel e dados capturados por hook.
* `success`: `"true"` ou `"false"`
* `duration_ms`: Tempo de execução em milissegundos
* `error_type`: String de categoria de erro quando a ferramenta falhou, como `"Error:ENOENT"` ou `"ShellError"`
* `error` (quando `OTEL_LOG_TOOL_DETAILS=1`): Mensagem de erro completa quando a ferramenta falhou
* `decision_type`: Sempre `"accept"`, já que este evento é emitido apenas após a ferramenta ser executada. Chamadas rejeitadas não produzem um resultado de ferramenta
* `decision_source`: De onde a decisão de permissão veio. Um de `"config"`, `"hook"`, `"user_permanent"`, ou `"user_temporary"`. Veja o [evento de decisão de ferramenta](#tool-decision-event) para o que cada valor significa. As fontes apenas de rejeição `"user_abort"` e `"user_reject"` nunca aparecem neste evento.
* `tool_input_size_bytes`: Tamanho da entrada de ferramenta serializada em JSON em bytes
* `tool_result_size_bytes`: Tamanho do resultado da ferramenta em bytes
* `mcp_server_scope`: Identificador de escopo do servidor MCP (para ferramentas MCP)
* `vcs.ref.head.revision`, `vcs.ref.head.name`, `vcs.ref.head.type` (quando `OTEL_LOG_TOOL_DETAILS=1`): a identidade do commit de uma execução bem-sucedida de `git commit` executada pela ferramenta Bash ou PowerShell. `vcs.ref.head.revision` é o SHA do commit, `vcs.ref.head.name` é o branch no qual foi feito o commit, e `vcs.ref.head.type` é `branch`. O nome e tipo são omitidos quando o commit foi feito em um HEAD desanexado. Requer Claude Code v2.1.269 ou posterior
* `tool_parameters` (quando `OTEL_LOG_TOOL_DETAILS=1`): String JSON contendo parâmetros específicos da ferramenta. Para servidores integrados do Claude Desktop, em sessões que Claude Desktop possui, o par `mcp_server_name`/`mcp_tool_name` é incluído mesmo com a flag desativada, a mesma exceção de autoria do host que o [evento de decisão de ferramenta](#tool-decision-event), requerendo Claude Code v2.1.214 ou posterior. Os parâmetros variam por ferramenta:
  * Para ferramenta Bash: inclui `bash_command`, `full_command`, `timeout`, `description`, e `dangerouslyDisableSandbox`, mais `git_commit_id` e `git_branch` quando um comando `git commit` é bem-sucedido. `git_commit_id` é o SHA completo do commit quando o commit é o HEAD do diretório de trabalho da sessão, e o SHA abreviado do git caso contrário. `git_branch` é o branch no qual foi feito o commit, omitido em um HEAD desanexado
  * Para a ferramenta Bash do workspace do aplicativo desktop, que também relata `tool_name` como `Bash`: inclui apenas `bash_command`, `full_command`, e `timeout`
  * Para ferramentas MCP: inclui `mcp_server_name`, `mcp_tool_name`
  * Para ferramenta Skill: inclui `skill_name`
  * Para ferramenta Agent ou ferramenta Task legada: inclui `subagent_type`
* `tool_input` (quando `OTEL_LOG_TOOL_DETAILS=1`): Argumentos de ferramenta serializados em JSON. Valores individuais acima de 512 caracteres são truncados, e o payload completo é limitado a \~4 K caracteres. Aplica-se a todas as ferramentas incluindo ferramentas MCP.

<h4 id="api-request-event">
  Evento de requisição de API
</h4>

Registrado para cada requisição de API para Claude.

**Nome do Evento**: `claude_code.api_request`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `model`: Modelo usado (por exemplo, "claude-sonnet-5")
* `cost_usd`: Custo estimado em USD
* `cost_usd_micros`: Custo estimado em milionésimos de dólar americano, emitido como um inteiro
* `duration_ms`: Duração da requisição em milissegundos
* `input_tokens`: Número de tokens de entrada
* `output_tokens`: Número de tokens de saída
* `cache_read_tokens`: Número de tokens lidos do cache
* `cache_creation_tokens`: Número de tokens usados para criação de cache
* `request_id`: ID de requisição de API da Anthropic do header `request-id` da resposta, como `"req_011..."`. Presente apenas quando a API retorna um.
* `client_request_id`: UUID gerado pelo cliente enviado como o header de requisição `x-client-request-id`; veja a tabela [atributos de correlação de eventos](#event-correlation-attributes) para quando está presente. Requer Claude Code v2.1.214 ou posterior
* `speed`: `"fast"` ou `"normal"`, indicando se o modo rápido estava ativo
* `query_source`: Subsistema que emitiu a requisição, como `"repl_main_thread"`, `"compact"`, ou um nome de subagente
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à requisição: `"low"`, `"medium"`, `"high"`, `"xhigh"`, ou `"max"`. Ausente quando Claude Code não envia nível de esforço, por exemplo em um modelo que não suporta esforço.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribuição de skill, plugin, agente e MCP para a requisição. Veja [Contador de custo](#cost-counter) para definições e comportamento de redação.

<h4 id="api-error-event">
  Evento de erro de API
</h4>

Registrado quando uma requisição de API para Claude falha.

**Nome do Evento**: `claude_code.api_error`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `model`: Modelo usado (por exemplo, "claude-sonnet-5")
* `error`: Mensagem de erro
* `status_code`: Código de status HTTP como um número. Ausente para erros não-HTTP como falhas de conexão.
* `duration_ms`: Duração da requisição em milissegundos
* `attempt`: Número total de tentativas feitas, incluindo a requisição inicial (`1` significa que nenhuma retentativa ocorreu)
* `request_id`: ID de requisição de API da Anthropic do header `request-id` da resposta, como `"req_011..."`. Presente apenas quando a API retorna um.
* `client_request_id`: UUID gerado pelo cliente enviado como o header de requisição `x-client-request-id`. Disponível mesmo quando uma falha como timeout ou erro de conexão nunca produziu um `request_id` do servidor; veja a tabela [atributos de correlação de eventos](#event-correlation-attributes) para quando está presente. Requer Claude Code v2.1.214 ou posterior
* `speed`: `"fast"` ou `"normal"`, indicando se o modo rápido estava ativo
* `query_source`: Subsistema que emitiu a requisição, como `"repl_main_thread"`, `"compact"`, ou um nome de subagente
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à requisição. Ausente quando Claude Code não envia nível de esforço, por exemplo em um modelo que não suporta esforço.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribuição de skill, plugin, agente e MCP para a requisição. Veja [Contador de custo](#cost-counter) para definições e comportamento de redação.

<h4 id="api-refusal-event">
  Evento de recusa de API
</h4>

Registrado quando uma requisição de API retorna `stop_reason: "refusal"`. Recusas chegam em um stream de resposta bem-sucedido em vez de como um erro HTTP, então o evento `api_error` não dispara para elas. Este evento permite rastrear a frequência de recusa e agrupar recusas pelos mesmos atributos que `api_request` e `api_error`.

**Nome do Evento**: `claude_code.api_refusal`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `model`: Identificador do modelo da requisição
* `request_id`: ID de requisição de API da Anthropic do header `request-id` da resposta, como `"req_011..."`. Presente apenas quando a API retorna um.
* `query_source`: Subsistema que emitiu a requisição, como `"repl_main_thread"`, `"compact"`, ou um nome de subagente. Veja [`api_request`](#api-request-event) para definições.
* `speed`: Ou `"fast"` quando [Modo rápido](/docs/pt/fast-mode) está ativo, ou `"normal"`
* `attempt`: Número de tentativa de retentativa. A primeira tentativa é `1`.
* `effort`: [Nível de esforço](/docs/pt/model-config#adjust-effort-level) aplicado à requisição. Ausente quando Claude Code não envia nível de esforço, por exemplo em um modelo que não suporta esforço.
* `server_fallback_hop`: `true` quando o fallback de modelo do lado do servidor da API já retentou esta recusa em um modelo diferente, então o usuário não viu esta recusa particular. `false` quando a requisição terminou em uma recusa. Um único turno pode emitir tanto um evento de hop `true` quanto um evento final `false` posterior quando o modelo de fallback também recusa.
* `has_category`: `true` quando a resposta da API carregava um `stop_details.category` de `"cyber"`, `"bio"`, `"frontier_llm"`, ou `"reasoning_extraction"`. `false` quando a resposta não carregava categoria ou um valor fora desse conjunto. Ausente quando `server_fallback_hop` é `true`, porque blocos de hop não carregam `stop_details`.
* `has_explanation`: `true` quando a resposta da API carregava um `stop_details.explanation`, caso contrário `false`. Ausente quando `server_fallback_hop` é `true`.
* `category`: O valor `stop_details.category` da resposta da API. Um de `"cyber"`, `"bio"`, `"frontier_llm"`, ou `"reasoning_extraction"`. Presente apenas quando `OTEL_LOG_TOOL_DETAILS=1` está definido e `has_category` é `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribuição de skill, plugin, agente e MCP para a requisição. Veja [Contador de custo](#cost-counter) para definições e comportamento de redação.

<h4 id="api-request-body-event">
  Evento de corpo de requisição de API
</h4>

Registrado para cada tentativa de requisição de API quando `OTEL_LOG_RAW_API_BODIES` está definido. Um evento é emitido por tentativa, então retentativas com parâmetros ajustados cada uma produz seu próprio evento.

**Nome do Evento**: `claude_code.api_request_body`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `body`: Parâmetros de requisição da API Messages serializados em JSON, como o prompt do sistema, mensagens e ferramentas, truncados no limite de conteúdo (60 KB por padrão). Conteúdo de pensamento estendido em turnos anteriores do assistente é redatado. Emitido apenas em modo inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Caminho absoluto para um arquivo `<dir>/<uuid>.request.json` contendo o corpo não truncado. Emitido apenas em modo arquivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Comprimento do corpo não truncado. Bytes UTF-8 quando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, ou unidades de código UTF-16 quando `=1`
* `body_truncated`: `"true"` quando truncamento inline ocorreu. Ausente em modo arquivo e quando nenhum truncamento ocorreu.
* `model`: Identificador do modelo dos parâmetros de requisição
* `query_source`: Subsistema que emitiu a requisição (por exemplo, `"compact"`)
* `request_body_id`: UUID que identifica o corpo de requisição desta tentativa. O evento [`api_response_body`](#api-response-body-event) para a tentativa que é bem-sucedida carrega o mesmo valor, então você pode emparelhar uma resposta com a requisição exata que a produziu. Requer Claude Code v2.1.274 ou posterior

<h4 id="api-response-body-event">
  Evento de corpo de resposta de API
</h4>

Registrado para cada resposta de API bem-sucedida quando `OTEL_LOG_RAW_API_BODIES` está definido.

Em modo arquivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`), Claude Code também anexa uma linha JSON a `<dir>/index.jsonl` para cada resposta bem-sucedida, com os campos `timestamp`, `session_id`, `query_source`, `model`, `request_id`, `message_id`, `message_uuid`, `request_file`, e `response_file`. Leia-o para encontrar os arquivos de requisição e resposta atrás de uma determinada mensagem de transcrição sem consultar seu backend de telemetria. O arquivo de índice requer Claude Code v2.1.274 ou posterior.

**Nome do Evento**: `claude_code.api_response_body`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `body`: Resposta da API Messages serializada em JSON, incluindo o id, blocos de conteúdo, uso e razão de parada, truncada no limite de conteúdo (60 KB por padrão). Conteúdo de pensamento estendido é redatado. Emitido apenas em modo inline (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Caminho absoluto para um arquivo `<dir>/<request_id>.response.json` contendo o corpo não truncado. Emitido apenas em modo arquivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Comprimento do corpo não truncado. Bytes UTF-8 quando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, ou unidades de código UTF-16 quando `=1`
* `body_truncated`: `"true"` quando truncamento inline ocorreu. Ausente em modo arquivo e quando nenhum truncamento ocorreu.
* `model`: Identificador do modelo
* `query_source`: Subsistema que emitiu a requisição
* `request_id`: ID de requisição de API da Anthropic do header `request-id` da resposta, como `"req_011..."`. Presente apenas quando a API retorna um.
* `request_body_id`: O `request_body_id` do evento [`api_request_body`](#api-request-body-event) que esta resposta responde. Requer Claude Code v2.1.274 ou posterior
* `message.id`: ID de mensagem que a API atribuiu à resposta, o campo `id` do corpo da resposta. Requer Claude Code v2.1.274 ou posterior
* `message.uuid`: UUID da entrada final da transcrição da resposta. Junto com `request_body_id`, vincula uma mensagem de transcrição aos corpos de requisição e resposta atrás dela. Requer Claude Code v2.1.274 ou posterior

<h4 id="tool-decision-event">
  Evento de decisão de ferramenta
</h4>

Registrado quando uma decisão de permissão de ferramenta é feita (aceitar/rejeitar).

**Nome do Evento**: `claude_code.tool_decision`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `tool_name`: Nome da ferramenta (por exemplo, "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: Identificador único para esta invocação de ferramenta. Corresponde ao `tool_use_id` passado para hooks, permitindo correlação entre eventos OTel e dados capturados por hook.
* `decision`: Ou `"accept"` ou `"reject"`
* `tool_source`: Sempre presente. A proveniência da ferramenta, como um conjunto fechado de valores de autoria da CLI. Requer Claude Code v2.1.214 ou posterior
  * `"builtin"`: as próprias ferramentas da CLI
  * `"mcp"`: servidores MCP em geral
  * `"sdk_host_builtin_mcp"`: um servidor em processo integrado ao próprio Claude Desktop, em uma sessão que Claude Desktop possui. Claude Desktop possui uma sessão que iniciou de um de seus próprios pontos de entrada, `claude-desktop`, `claude-desktop-3p`, ou `local-agent`, quando essa sessão não é um filho aninhado; sessões aninhadas, incluindo sessões que Claude Code gera, relatam esses servidores como `"mcp"`
* `source`: De onde a decisão veio:
  * `"config"`: Decidido automaticamente sem solicitar, baseado em configurações de projeto, regras de permissão ou negação nas configurações pessoais do usuário, política gerenciada pela empresa, flags `--allowedTools` ou `--disallowedTools`, o modo de permissão ativo, uma concessão com escopo de sessão de um prompt anterior na mesma sessão CLI interativa, ou porque a ferramenta é inerentemente segura. O evento não indica qual dessas fontes correspondeu. Claude Code também relata `"config"` quando a própria requisição de prompt de permissão falha, por exemplo quando o callback [`canUseTool`](/docs/pt/agent-sdk/typescript#canusetool) do Agent SDK ou a ferramenta [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags) retorna um resultado inválido, ou quando o stream de entrada fecha enquanto a requisição está pendente. Antes da v2.1.216, Claude Code relatava essas falhas como `"user_reject"`.
  * `"hook"`: Um hook `PreToolUse` ou `PermissionRequest` retornou a decisão.
  * `"user_permanent"`: Emitido quando o usuário escolheu "Sim, e não pergunte novamente para ..." em um prompt de permissão, que salva uma regra de permissão em suas configurações pessoais. Na CLI interativa isto é emitido apenas para essa escolha em si; chamadas posteriores que correspondem à regra salva emitem `"config"` em vez disso. Em sessões Agent SDK ou não-interativas `-p`, tanto a escolha inicial quanto correspondências posteriores de regra emitem `"user_permanent"`. Tratado como uma aceitação.
  * `"user_temporary"`: Emitido quando o usuário escolheu "Sim" em um prompt de permissão para uma aprovação única, ou escolheu uma opção que concede acesso pelo resto da sessão em um prompt de edição ou leitura de arquivo. Na CLI interativa isto é emitido apenas para a escolha em si; chamadas posteriores permitidas por essa concessão com escopo de sessão emitem `"config"` em vez disso. Em sessões Agent SDK ou não-interativas `-p`, tanto a escolha quanto correspondências posteriores emitem `"user_temporary"`. Tratado como uma aceitação.
  * `"user_abort"`: Emitido quando o usuário descartou o prompt de permissão sem responder. Em sessões Agent SDK e não-interativas `-p`, isto inclui interromper o turno enquanto uma requisição de permissão `canUseTool` ou `--permission-prompt-tool` está pendente; antes da v2.1.216, Claude Code relatava essa interrupção como `"user_reject"`. Tratado como uma rejeição.
  * `"user_reject"`: Emitido quando o usuário escolheu "Não" quando solicitado. Na CLI interativa isto é emitido apenas para essa escolha em si; chamadas que correspondem a uma regra de negação nas configurações pessoais do usuário emitem `"config"` em vez disso. Em sessões Agent SDK ou não-interativas `-p`, chamadas que correspondem a uma regra de negação em configurações pessoais emitem `"user_reject"`. Tratado como uma rejeição.
* `tool_parameters` (quando `OTEL_LOG_TOOL_DETAILS=1`): String JSON contendo parâmetros específicos da ferramenta. Mesma forma que o [evento de resultado de ferramenta](#tool-result-event), menos campos pós-execução como `git_commit_id`. Os valores podem diferir de `tool_result` para uma chamada aceita se a decisão de permissão reescreve a entrada da ferramenta via `updatedInput`. Use este atributo para ver qual comando foi rejeitado quando `decision` é `"reject"`.
  * Para ferramentas `"sdk_host_builtin_mcp"`: `mcp_server_name` e `mcp_tool_name` são incluídos mesmo quando `OTEL_LOG_TOOL_DETAILS` está desativado, porque a aplicação host define esses nomes; sem eles, uma chamada rejeitada para um desses servidores integrados seria não atribuível no stream padrão. Para servidores MCP configurados pelo usuário, o `tool_name` do evento é sempre o literal `"mcp_tool"`, e os nomes do servidor e ferramenta aparecem apenas em `tool_parameters` com a flag ativada; conteúdo de argumento requer a flag em todos os lugares. Requer Claude Code v2.1.214 ou posterior
  * Para ferramenta Bash: inclui `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`. A ferramenta bash do workspace do aplicativo desktop também relata `tool_name` como `Bash`, mas inclui apenas `bash_command`, `full_command`, e `timeout`
  * Para ferramentas MCP: inclui `mcp_server_name`, `mcp_tool_name`
  * Para ferramenta Skill: inclui `skill_name`
  * Para ferramenta Agent ou ferramenta Task legada: inclui `subagent_type`

<h4 id="permission-mode-changed-event">
  Evento de mudança de modo de permissão
</h4>

Registrado quando o modo de permissão muda, por exemplo de ciclagem `Shift+Tab`, saída do modo de plano, ou uma verificação de gate de modo automático.

**Nome do Evento**: `claude_code.permission_mode_changed`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `from_mode`: O modo de permissão anterior, por exemplo `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, ou `"bypassPermissions"`
* `to_mode`: O novo modo de permissão
* `trigger`: O que causou a mudança. Um de `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"`, ou `"auto_opt_in"`. Ausente quando a transição origina do SDK ou bridge

<h4 id="auth-event">
  Evento de autenticação
</h4>

Registrado quando `/login` ou `/logout` é concluído.

**Nome do Evento**: `claude_code.auth`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `action`: `"login"` ou `"logout"`
* `success`: `"true"` ou `"false"`
* `auth_method`: Método de autenticação, como `"oauth"`
* `error_category`: Tipo de erro categórico quando a ação falhou. A mensagem de erro bruta nunca é incluída
* `status_code`: Código de status HTTP como uma string quando a ação falhou com um erro HTTP

<h4 id="mcp-server-connection-event">
  Evento de conexão do servidor MCP
</h4>

Registrado quando um servidor MCP se conecta, desconecta, ou falha em conectar.

**Nome do Evento**: `claude_code.mcp_server_connection`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `status`: `"connected"`, `"failed"`, ou `"disconnected"`
* `transport_type`: Transporte do servidor, como `"stdio"`, `"sse"`, ou `"http"`
* `server_scope`: Escopo no qual o servidor está configurado, como `"user"`, `"project"`, ou `"local"`
* `duration_ms`: Duração da tentativa de conexão em milissegundos
* `error_code`: Código de erro quando a conexão falhou
* `is_plugin`: `true` quando o servidor é fornecido por um plugin, `false` caso contrário
* `plugin_id_hash` (quando `is_plugin` é `true`): Hash estável do nome do plugin e marketplace, para agrupar eventos por plugin sem expor o nome. Claude Code o computa conforme descrito no [evento de plugin carregado](#plugin-loaded-event)
* `plugin.name` (quando `is_plugin` é `true`): Nome do plugin que fornece o servidor. Para plugins de terceiros isto é a string literal `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`; isto protege nomes de plugins de terceiros de aparecerem em logs por padrão. Plugins de fontes oficiais da Anthropic são sempre identificados por nome. Os atributos `plugin_id_hash` e `plugin.name` fluem para seu próprio backend de monitoramento e não são enviados para a Anthropic
* `server_name` (quando `OTEL_LOG_TOOL_DETAILS=1`): Nome do servidor configurado
* `error` (quando `OTEL_LOG_TOOL_DETAILS=1`): Mensagem de erro completa quando a conexão falhou

<h4 id="internal-error-event">
  Evento de erro interno
</h4>

Registrado quando Claude Code captura um erro interno inesperado. Apenas o nome da classe de erro e um código estilo errno são registrados. A mensagem de erro e stack trace nunca são incluídos. Este evento não é emitido ao executar contra Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry, ou quando `DISABLE_ERROR_REPORTING` está definido.

**Nome do Evento**: `claude_code.internal_error`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `error_name`: Nome da classe de erro, como `"TypeError"` ou `"SyntaxError"`
* `error_code`: Código errno do Node.js como `"ENOENT"` quando presente no erro

<h4 id="plugin-installed-event">
  Evento de plugin instalado
</h4>

Registrado quando um plugin termina de instalar, tanto do comando CLI `claude plugin install` quanto da UI interativa `/plugin`.

**Nome do Evento**: `claude_code.plugin_installed`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `marketplace.is_official`: `"true"` se o marketplace é um marketplace oficial da Anthropic, `"false"` caso contrário
* `install.trigger`: `"cli"` ou `"ui"`
* `plugin.name`: Nome do plugin instalado. Para marketplaces de terceiros isto é incluído apenas quando `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version`: Versão do plugin quando declarada na entrada do marketplace. Para marketplaces de terceiros isto é incluído apenas quando `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Marketplace do qual o plugin foi instalado. Para marketplaces de terceiros isto é incluído apenas quando `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Evento de plugin carregado
</h4>

Registrado uma vez por plugin habilitado no início da sessão. Use este evento para inventariar quais plugins estão ativos em sua frota, como complemento a `plugin_installed` que registra a ação de instalação em si.

**Nome do Evento**: `claude_code.plugin_loaded`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `plugin.name`: nome do plugin. Para plugins fora do marketplace oficial e pacote integrado o valor é `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: marketplace do qual o plugin foi instalado, quando conhecido. Redatado para `"third-party"` sob a mesma condição que `plugin.name`
* `plugin.version`: versão do manifesto do plugin. Incluído apenas quando o nome não é redatado e o manifesto declara uma versão
* `plugin.scope`: categoria de proveniência para o plugin: `"official"`, `"community"`, `"org"`, `"user-local"`, ou `"default-bundle"`
* `enabled_via`: como o plugin veio a ser habilitado: `"default-enable"`, `"org-policy"`, `"admin-install"`, `"seed-mount"`, ou `"user-install"`. O valor `"admin-install"` significa que o plugin está definido como obrigatório ou auto-instalação para sua organização em [**Configurações da Organização > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory). Antes da v2.1.246, Claude Code relatava esses plugins como `"user-install"` ou `"seed-mount"`
* `plugin_id_hash`: hash determinístico do nome do plugin e marketplace, enviado apenas para seu exportador configurado. Permite contar os plugins de terceiros distintos carregados em sua frota sem registrar seus nomes. Para [plugins sincronizados de claude.ai](/docs/pt/plugins/loading#synced-plugins), Claude Code faz hash do nome do plugin com o nome do marketplace que claude.ai relata para o plugin, ou com `synced` caso contrário. Antes da v2.1.246, Claude Code não usava o nome do marketplace que claude.ai relata no hash
* `has_hooks`: se o plugin contribui hooks
* `has_mcp`: se o plugin contribui servidores MCP
* `host_owned_mcp`: `true` quando o host SDK gerencia as conexões MCP deste plugin e Claude Code pulou a leitura da configuração do servidor MCP do plugin, `false` caso contrário. Requer Claude Code v2.1.172 ou posterior
* `skill_path_count`: número de diretórios de skill que o plugin declara
* `command_path_count`: número de diretórios de comando que o plugin declara
* `agent_path_count`: número de diretórios de agente que o plugin declara
* `safe_mode`: `"true"` quando a sessão foi iniciada com [`--safe-mode`](/docs/pt/cli-reference), `"false"` caso contrário. Em modo seguro este evento relata apenas inventário configurado; os comandos, skills, hooks e servidores MCP do plugin não carregam. Requer Claude Code v2.1.169 ou posterior

<h4 id="skill-activated-event">
  Evento de skill ativada
</h4>

Registrado quando uma skill é invocada, seja Claude a chama através da ferramenta Skill ou você a executa como um comando `/`.

**Nome do Evento**: `claude_code.skill_activated`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `skill.name`: Nome da skill. Para skills definidas pelo usuário e de plugins de terceiros o valor é o placeholder `"custom_skill"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Como a skill foi acionada (`"user-slash"`, `"claude-proactive"`, ou `"nested-skill"`)
* `skill.source`: De onde a skill foi carregada (por exemplo, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"` quando a skill é uma skill de workflow. Ausente caso contrário
* `plugin.name` (quando `OTEL_LOG_TOOL_DETAILS=1` ou o plugin é de um marketplace oficial): Nome do plugin proprietário quando a skill é fornecida por um plugin
* `marketplace.name` (quando `OTEL_LOG_TOOL_DETAILS=1` ou o plugin é de um marketplace oficial): Marketplace do qual o plugin proprietário foi instalado, quando a skill é fornecida por um plugin

<h4 id="at-mention-event">
  Evento de menção @
</h4>

Registrado quando Claude Code resolve uma menção `@` em um prompt. Nem toda menção emite um evento: caminhos de saída antecipada como negações de permissão, arquivos superdimensionados, anexos de referência PDF e falhas de listagem de diretório retornam sem registrar.

**Nome do Evento**: `claude_code.at_mention`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `mention_type`: Tipo de menção (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`, `"peer"`). O valor `"peer"` significa que você mencionou [uma de suas outras sessões Claude Code](/docs/pt/cross-session-messaging). Requer Claude Code v2.1.232 ou posterior
* `success`: Se a menção foi resolvida com sucesso (`"true"` ou `"false"`)

<h4 id="api-retries-exhausted-event">
  Evento de retentativas de API esgotadas
</h4>

Registrado uma vez quando uma requisição de API falha após mais de uma tentativa. Emitido junto com o evento `api_error` final.

**Nome do Evento**: `claude_code.api_retries_exhausted`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `model`: Modelo usado
* `error`: Mensagem de erro final
* `status_code`: Código de status HTTP como um número. Ausente para erros não-HTTP.
* `total_attempts`: Número total de tentativas feitas
* `total_retry_duration_ms`: Tempo total de wall-clock em todas as tentativas
* `speed`: `"fast"` ou `"normal"`

<h4 id="hook-registered-event">
  Evento de hook registrado
</h4>

Registrado uma vez por hook configurado no início da sessão. Use este evento para inventariar quais hooks estão ativos em sua frota, como complemento aos eventos por execução `hook_execution_start` e `hook_execution_complete`.

**Nome do Evento**: `claude_code.hook_registered`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `hook_event`: tipo de evento de hook, como `"PreToolUse"` ou `"PostToolUse"`
* `hook_type`: tipo de implementação de hook: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, ou `"agent"`
* `hook_source`: onde o hook é definido: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, ou `"pluginHook"`
* `safe_mode`: `"true"` quando a sessão foi iniciada com [`--safe-mode`](/docs/pt/cli-reference), `"false"` caso contrário. Requer Claude Code v2.1.169 ou posterior
* `hook_matcher` (quando `OTEL_LOG_TOOL_DETAILS=1`): a string de matcher da configuração do hook, quando uma está definida
* `plugin.name` (quando `hook_source` é `"pluginHook"`): nome do plugin contribuidor. Para plugins fora do marketplace oficial e pacote integrado o valor é `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (quando `hook_source` é `"pluginHook"`): hash determinístico do nome do plugin e marketplace, enviado apenas para seu exportador configurado. Permite contar plugins contribuidores distintos sem registrar seus nomes. Claude Code o computa conforme descrito no [evento de plugin carregado](#plugin-loaded-event)

<h4 id="hook-execution-start-event">
  Evento de início de execução de hook
</h4>

Registrado quando um ou mais hooks começam a executar para um evento de hook.

**Nome do Evento**: `claude_code.hook_execution_start`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `hook_event`: Tipo de evento de hook, como `"PreToolUse"` ou `"PostToolUse"`
* `hook_name`: Nome completo do hook incluindo matcher, como `"PreToolUse:Write"`
* `num_hooks`: Número de comandos de hook correspondentes
* `managed_only`: `"true"` quando apenas hooks de política gerenciada são permitidos
* `hook_source`: `"policySettings"` ou `"merged"`
* `safe_mode`: `"true"` quando a sessão foi iniciada com [`--safe-mode`](/docs/pt/cli-reference), `"false"` caso contrário. Requer Claude Code v2.1.169 ou posterior
* `hook_definitions`: Configuração de hook serializada em JSON. Incluído apenas quando rastreamento beta detalhado e `OTEL_LOG_TOOL_DETAILS=1` estão ambos habilitados

<h4 id="hook-execution-complete-event">
  Evento de conclusão de execução de hook
</h4>

Registrado quando todos os hooks para um evento de hook terminaram.

**Nome do Evento**: `claude_code.hook_execution_complete`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `hook_event`: Tipo de evento de hook
* `hook_name`: Nome completo do hook incluindo matcher
* `num_hooks`: Número de comandos de hook correspondentes
* `num_success`: Contagem que completou com sucesso
* `num_blocking`: Contagem que retornou uma decisão de bloqueio
* `num_non_blocking_error`: Contagem que falhou sem bloquear
* `num_cancelled`: Contagem cancelada antes da conclusão
* `total_duration_ms`: Duração de wall-clock de todos os hooks correspondentes
* `stdout_chars`: Total de caracteres de stdout em todos os hooks correspondentes que tiveram sucesso. Requer Claude Code v2.1.280 ou posterior
* `additional_context_chars`: Total de caracteres de `additionalContext` retornados pelos hooks correspondentes. Requer Claude Code v2.1.280 ou posterior
* `system_message_chars`: Total de caracteres de `systemMessage` retornados pelos hooks correspondentes. Requer Claude Code v2.1.280 ou posterior
* `initial_user_message_chars`: Total de caracteres de `initialUserMessage` retornados pelos hooks correspondentes. Requer Claude Code v2.1.280 ou posterior
* `num_outputs_persisted`: Número de saídas de hook acima do [limite de 10.000 caracteres](/docs/pt/hooks#json-output) que Claude Code salvou em um arquivo. Requer Claude Code v2.1.280 ou posterior
* `managed_only`: `"true"` quando apenas hooks de política gerenciada são permitidos
* `hook_source`: `"policySettings"` ou `"merged"`
* `safe_mode`: `"true"` quando a sessão foi iniciada com [`--safe-mode`](/docs/pt/cli-reference), `"false"` caso contrário. Requer Claude Code v2.1.169 ou posterior
* `hook_definitions`: Configuração de hook serializada em JSON. Incluído apenas quando rastreamento beta detalhado e `OTEL_LOG_TOOL_DETAILS=1` estão ambos habilitados

<h4 id="hook-plugin-metrics-event">
  Evento de métricas de plugin de hook
</h4>

Registrado quando um hook de plugin do marketplace oficial emite métricas por invocação. Apenas plugins instalados de um marketplace oficial da Anthropic podem emitir estes. Plugins de marketplace de terceiros e hooks configurados pelo usuário não emitem para este evento. Use este evento para monitorar comportamento de plugin como taxas de descoberta, custos e durações de sua própria pilha de observabilidade.

**Nome do Evento**: `claude_code.hook_plugin_metrics`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `plugin_id`: identificador do plugin em forma `<name>@<marketplace>`
* `hook_event`: tipo de evento de hook que emitiu as métricas
* Até 20 chaves de métrica emitidas pelo plugin. Os nomes correspondem a `^[a-z][a-z0-9_]{0,39}$`. Os valores são booleano ou número.

<h4 id="compaction-event">
  Evento de compactação
</h4>

Registrado quando a compactação de conversa é concluída.

**Nome do Evento**: `claude_code.compaction`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `trigger`: `"auto"` ou `"manual"`
* `success`: `"true"` ou `"false"`
* `duration_ms`: Duração da compactação
* `pre_tokens`: Contagem aproximada de tokens antes da compactação
* `post_tokens`: Contagem aproximada de tokens após compactação
* `error`: Mensagem de erro quando a compactação falhou
* `precompute_reuse`: Definido apenas quando `trigger` é `"manual"`. A compactação automática pode preparar um resumo em background antes da janela de contexto ficar cheia, e este atributo registra se `/compact` reutilizou esse resumo preparado. `"hit"` significa que foi reutilizado; `"miss_custom_instructions"`, `"miss_hook"`, e `"miss_not_ready"` dão a razão pela qual um resumo fresco foi computado em vez disso. Requer Claude Code v2.1.153 ou posterior

<h4 id="subagent-completed-event">
  Evento de conclusão de subagente
</h4>

Registrado quando um [subagente](/docs/pt/sub-agents) termina e retorna seu resultado para a conversa que o iniciou. Use-o para agregar uso de ferramenta e tempo de execução por tipo de subagente; para agregações de token ou custo, use o [contador de tokens](#token-counter) e [contador de custo](#cost-counter) filtrados para `query_source` `"subagent"`, já que o `total_tokens` deste evento cobre apenas a requisição final. A categoria `"subagent"` também conta requisições de hooks baseados em agente, que não emitem evento de subagente.

**Nome do Evento**: `claude_code.subagent_completed`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"subagent_completed"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `agent_type`: O tipo de subagente. Nomes de agentes integrados e agentes de plugins do marketplace oficial aparecem literalmente; outros nomes de agente são substituídos por `"custom"` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido
* `agent.source`: De onde a definição do agente veio: `built-in`, `plugin`, ou a fonte de configurações que definiu um agente personalizado, como `userSettings` ou `projectSettings`
* `is_built_in`: Se o subagente é um tipo de agente integrado
* `is_async`: Se o subagente executou em [background](/docs/pt/sub-agents#run-subagents-in-foreground-or-background)
* `total_tokens`: A pegada de token da requisição final de API do subagente: tokens de entrada, criação de cache, leitura de cache e saída dessa única requisição, aproximadamente o tamanho do contexto do subagente na conclusão. Não uma soma em toda a execução
* `total_tool_uses`: Número de chamadas de ferramenta que o subagente fez em toda a execução
* `duration_ms`: Tempo de execução em milissegundos
* `model`: O modelo que o subagente foi resolvido para executar
* `final_model`: O modelo que produziu a resposta final do subagente, que difere de `model` após uma mudança no meio da execução como um fallback. Requer Claude Code v2.1.212 ou posterior
* `model_swapped`: Se mais de um modelo serviu as requisições do subagente. Requer Claude Code v2.1.212 ou posterior
* `plugin_id_hash`, `plugin.name`: Presente para agentes fornecidos por plugin. Nomes de plugins do marketplace oficial aparecem literalmente; outros nomes de plugin são substituídos por `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1` esteja definido

<h4 id="feedback-survey-event">
  Evento de pesquisa de feedback
</h4>

Registrado quando uma pesquisa de qualidade de sessão é mostrada ou respondida. Veja [Pesquisas de qualidade de sessão](/docs/pt/data-usage#session-quality-surveys) para o que as pesquisas coletam e como controlá-las.

**Nome do Evento**: `claude_code.feedback_survey`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `event_type`: Evento do ciclo de vida da pesquisa, por exemplo `"appeared"`, `"responded"`, ou `"transcript_prompt_appeared"`
* `appearance_id`: ID único vinculando os eventos emitidos para uma instância de pesquisa
* `survey_type`: Qual pesquisa produziu o evento. `"session"` é o prompt de classificação "Como Claude está se saindo?"
* `response`: A seleção do usuário em eventos `responded`
* `enabled_via_override`: `true` quando [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/pt/env-vars) está definido. Emitido como um booleano, não uma string. Presente em eventos de pesquisa `session`. Filtre neste atributo para confirmar que a substituição é aplicada em uma frota

<h4 id="retention-sweep-event">
  Evento de varredura de retenção
</h4>

Registrado uma vez por execução da varredura de limpeza de retenção, que deleta [transcrições de sessão e outros dados de aplicação](/docs/pt/claude-directory#cleaned-up-automatically) mais antigos que a configuração [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays). Claude Code executa a varredura em background no máximo uma vez por sessão, e uma execução que não deleta nada ainda emite o evento. Se Claude Code executou a varredura em qualquer sessão na mesma máquina nos últimos 24 horas, ele atrasa a varredura desta sessão por pelo menos 10 minutos, então uma sessão que sai mais cedo não emite nada. Quando você executa `claude -p` com `--bare`, Claude Code não executa a varredura e não emite nada.

Como todo evento OTel nesta página, ele vai apenas para o backend de telemetria que você configura. Requer Claude Code v2.1.227 ou posterior.

Quando Claude Code não pode determinar com segurança o período de retenção, ele pausa a varredura e emite o evento com `result` definido para `"skipped"` e um `skip_reason`. Quando [configurações gerenciadas](/docs/pt/server-managed-settings) definem `cleanupPeriodDays`, o valor gerenciado fixa o período de retenção e a varredura é executada mesmo quando um arquivo de configurações em um escopo de prioridade mais baixa está quebrado ou inválido. Quando `managed-settings.json` em si não pode ser lido, Claude Code ainda pausa a varredura a menos que o [nível gerenciado](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) forneça `cleanupPeriodDays` de outro lugar, como configurações gerenciadas pelo servidor ou um drop-in `managed-settings.d/` ao lado do arquivo quebrado. Os atributos do contador de exclusão estão presentes apenas quando `result` é `"complete"`.

**Nome do Evento**: `claude_code.retention_sweep`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"retention_sweep"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `result`: `"complete"` quando a varredura foi executada, `"skipped"` quando Claude Code a pausou
* `period_days`: O valor `cleanupPeriodDays` das configurações mescladas, em dias, ou `30` quando nenhuma fonte o define. Em eventos pulados, o valor que a varredura teria usado, computado das fontes de configurações que Claude Code pôde ler
* `used_default`: `"true"` quando nenhuma fonte de configurações legível define `cleanupPeriodDays`, `"false"` caso contrário. Em eventos completos, `"true"` significa que o padrão de 30 dias foi aplicado
* `skip_reason`: Por que Claude Code pausou a varredura. Presente apenas quando `result` é `"skipped"`:
  * `"user_source_disabled"`: Configurações do usuário são excluídas, por exemplo pela flag [`--setting-sources`](/docs/pt/cli-reference#cli-flags) ou opção [`settingSources`](/docs/pt/agent-sdk/typescript#options) do SDK, e nenhuma fonte habilitada fornece `cleanupPeriodDays`
  * `"settings_unknowable"`: Um arquivo de configurações não pôde ser lido ou analisado, então `cleanupPeriodDays` ou `desktopSessionCleanupPeriodDays` pode estar definido para um valor que Claude Code não pode ver
  * `"settings_invalid_key_set"`: Configurações têm erros de validação e `cleanupPeriodDays` ou `desktopSessionCleanupPeriodDays` está explicitamente definido, então fazer fallback para o padrão poderia deletar ou manter arquivos contra essa configuração
* `transcripts_deleted`: Número de transcrições de sessão, os arquivos `~/.claude/projects/*/*.jsonl` de nível superior, que a varredura deletou
* `transcripts_exempted_desktop`: Número de transcrições passadas do período de retenção que a varredura manteve sob a [regra de Claude Desktop e Cowork](/docs/pt/claude-directory#cleaned-up-automatically). Estes não contam para `files_past_cutoff`. Requer Claude Code v2.1.248 ou posterior
* `session_files_deleted`: Número de artefatos que a varredura de arquivos de sessão deletou: transcrições mais arquivos complementares por sessão como sidecars, gravações e resultados de ferramentas
* `artifacts_deleted`: Total de itens que a varredura deletou em todos os diretórios de dados que cobre, incluindo os arquivos de sessão. Algumas varreduras contam uma árvore de diretório removida inteira como um item e algumas passagens de limpeza não contribuem para o contador, então trate o valor como um piso em vez de uma contagem exata de arquivos
* `files_retained_fresh`: Arquivos inspecionados e deixados em lugar porque ainda estão dentro do período de retenção. Apenas varreduras por arquivo contam estes, então o valor é um piso; um valor diferente de zero é o estado estável normal
* `files_past_cutoff`: Arquivos mais antigos que o período de retenção que a varredura falhou em deletar, por exemplo por causa de um erro de permissão ou um arquivo mantido aberto. Um valor acima de zero significa que arquivos sobreviveram ao período de retenção configurado; zero não é prova de que nenhum fez, porque uma remoção falhada de um diretório inteiro conta para `error_count` em vez disso
* `error_count`: Número de erros que a varredura encontrou ao listar ou deletar arquivos

<h4 id="managed-settings-resolved-event">
  Evento de configurações gerenciadas resolvidas
</h4>

Registrado com as [configurações gerenciadas](/docs/pt/managed-settings) que uma sessão resolveu: uma vez no início da sessão, novamente quando as configurações gerenciadas ou o [auxiliar de política](/docs/pt/managed-settings#compute-the-policy-with-a-helper-program) mudam de estado durante a sessão, e quando Claude Code recusa iniciar ou termina a sessão por uma das razões que o atributo `error.type` lista.
Use este evento para encontrar máquinas executando em uma fonte gerenciada inesperada, máquinas cujo auxiliar de política está falhando, e a razão pela qual uma máquina recusou iniciar.
Requer Claude Code v2.1.274 ou posterior.

Por padrão, o evento carrega as fontes gerenciadas e o estado do auxiliar de política mas não as configurações em si. Para adicionar o atributo `managed_settings.settings` redatado e o digest `managed_settings.resolved_sha256`, defina `OTEL_LOG_MANAGED_SETTINGS=1`:

* Defina-o no bloco `env` de configurações gerenciadas, configurações do usuário, ou `--settings`, ou no ambiente com o qual você inicia Claude Code. Um valor em configurações de projeto ou local não o ativa, porque um repositório clonado pode escrevê-los.
* Configurações gerenciadas pelo servidor podem defini-lo sem mostrar o [diálogo de aprovação de segurança](/docs/pt/server-managed-settings#security-approval-dialogs), porque a variável apenas adiciona sua própria política redatada da organização a um evento que sua organização já recebe.

Em uma sessão interativa em uma pasta que você não [confiou](/docs/pt/permissions#what-runs-before-you-trust-a-folder), Claude Code não exporta o evento de recusa.

**Nome do Evento**: `claude_code.managed_settings_resolved`

**Atributos**:

* Todos os [atributos padrão](#standard-attributes)
* `event.name`: `"managed_settings_resolved"`
* `event.timestamp`: Timestamp ISO 8601
* `event.sequence`: contador por processo para ordenar eventos, descrito em [Atributos de correlação de eventos](#event-correlation-attributes)
* `managed_settings.trigger`: `"startup"` para o evento de início de sessão, `"change"` quando as configurações gerenciadas ou o estado do auxiliar de política mudaram mais tarde na sessão, ou `"refused"` quando uma política de configurações gerenciadas parou a sessão. Claude Code envia um evento `change` apenas quando um atributo difere do último evento que enviou, e um valor de configuração alterado conta mesmo quando `OTEL_LOG_MANAGED_SETTINGS` está desativado
* `error.type`: por que Claude Code parou a sessão. Presente apenas em eventos `refused`:
  * `"helper_failed"`: uma [execução do auxiliar de política falhou](/docs/pt/settings-reference#helper-failures)
  * `"policy_invalid"`: as configurações gerenciadas contêm um erro que impede Claude Code de iniciar, ou uma fonte de administrador falhou em carregar, então Claude Code não pode verificar a imposição de login da organização
  * `"consent_rejected"`: o usuário rejeitou o [diálogo de aprovação de segurança](/docs/pt/server-managed-settings#security-approval-dialogs) para configurações gerenciadas pelo servidor
  * `"force_refresh_failed"`: a busca de configurações que [`forceRemoteSettingsRefresh`](/docs/pt/settings-reference#forceremotesettingsrefresh) requer falhou
  * `"gateway_rejected"`: um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) respondeu ao carregamento de configurações gerenciadas com HTTP 403
  * `"version_below_minimum"`: esta versão de Claude Code está abaixo de [`requiredMinimumVersion`](/docs/pt/settings-reference#requiredminimumversion) ou acima de [`requiredMaximumVersion`](/docs/pt/settings-reference#requiredmaximumversion)
  * `"_OTHER"`: o carregamento de configurações gerenciadas do gateway de aplicativos Claude falhou por outro motivo
* `managed_settings.sources`: cada fonte gerenciada que entrega pelo menos uma [chave de política](/docs/pt/managed-settings#how-claude-code-combines-managed-sources), prioridade mais alta primeiro, incluindo fontes cujas chaves não entram em efeito sob `first-wins`. Os valores são `"remote"`, `"plist"` ou `"hklm"` para a política MDM ou nível de SO, `"file"` para arquivos de configurações gerenciadas e drop-ins, `"parent"` quando um [host de incorporação](/docs/pt/managed-settings#let-an-embedding-host-add-policy) fornece configurações, e `"hkcu"` para o [valor de registro HKCU do Windows](/docs/pt/managed-settings#where-each-mechanism-stores-the-policy) quando Claude Code o [lê](/docs/pt/managed-settings#how-claude-code-combines-managed-sources). Uma fonte que carrega apenas chaves de controle, ou que Claude Code não pôde ler, não está listada. Emitido como um array de strings, vazio quando nenhuma fonte gerenciada entrega uma chave de política
* `managed_settings.source_behavior`: o valor [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior) que Claude Code leu, `"first-wins"` ou `"merge"`. `"first-wins"` quando nenhuma fonte define a chave
* `managed_settings.helper.state`: estado do auxiliar de política que a fonte MDM ou arquivo selecionada configura:
  * `"ok"`: a saída do auxiliar serve como as configurações gerenciadas
  * `"bad_path"`, `"not_a_file"`, `"exit_nonzero"`, `"timed_out"`, `"oversize"`, `"parse_failed"`, `"envelope_invalid"`, ou `"schema_rejected"`: a última execução do auxiliar falhou. [Falhas do auxiliar](/docs/pt/settings-reference#helper-failures) descreve os casos
  * `"none"`: nenhum auxiliar está configurado, ou a fonte que o configura não é uma política MDM ou arquivo de configurações gerenciadas
* `managed_settings.helper.applied`: `"output"` enquanto a saída do próprio auxiliar serve como as configurações gerenciadas, `"none"` quando não serve
* `managed_settings.helper.entry`: `"policyHelper"` quando Claude Code selecionou um [`policyHelper`](/docs/pt/settings-reference#policyhelper). Ausente quando selecionou nenhum auxiliar
* `managed_settings.helper.path`: o [`path`](/docs/pt/settings-reference#policyhelper-path) configurado do auxiliar. Presente sempre que Claude Code selecionou um auxiliar, independentemente de `OTEL_LOG_MANAGED_SETTINGS` estar definido
* `managed_settings.resolved_sha256` (quando `OTEL_LOG_MANAGED_SETTINGS=1`): SHA-256 das configurações gerenciadas resolvidas antes da redação, serializadas como JSON com chaves ordenadas recursivamente e sem espaço em branco. Máquinas com o mesmo digest executam a mesma política. Claude Code envia o digest apenas com o opt-in porque uma política curta pode ser recuperada fazendo hash de suposições. Ausente quando nenhuma configuração gerenciada foi resolvida, e em eventos `refused`
* `managed_settings.settings` (quando `OTEL_LOG_MANAGED_SETTINGS=1`): os nomes e forma das configurações gerenciadas resolvidas como uma string JSON, com os valores redatados. Ausente em eventos `refused`. Claude Code o constrói a partir de seu esquema de configurações:

  * Um nome de configuração que o esquema declara é exportado, e uma chave que não declara é deixada de fora
  * Booleanos, números e valores de string que o esquema restringe a um conjunto fixo de opções, como `permissions.defaultMode`, são exportados como estão. `sandbox.network.httpProxyPort` e `sandbox.network.socksProxyPort` são exportados como `"[REDACTED]"`
  * Toda outra string, como `model`, `apiKeyHelper`, todo valor `env`, toda URL e todo comando, é exportado como `"[REDACTED]"`
  * Os nomes de entrada de mapas, como nomes de variáveis `env` e IDs de plugin, são exportados como estão. Uma configuração cujas entradas o esquema não digita, como `vimInsertModeRemaps`, é exportada como um único `"[REDACTED]"`, e `sandbox.ignoreViolations` é exportado como uma lista de suas listas de caminho sem os padrões de comando
  * Uma lista mantém seu comprimento, com cada entrada redatada pelas mesmas regras
  * Uma regra `permissions.allow`, `permissions.deny`, ou `permissions.ask` é exportada como seu nome de ferramenta com o conteúdo redatado, como `Read([REDACTED])`, quando a ferramenta é integrada nesta versão de Claude Code ou é uma referência `mcp__` como `mcp__jira__create_issue`. Qualquer outra regra é exportada como `"[REDACTED]"`
  * Hooks seguem as mesmas regras, então campos de opção fixa e numéricos como `type` e `timeout` mostram, enquanto cada comando, URL, `matcher`, e condição `if` é exportada como `"[REDACTED]"`

  Por exemplo, configurações gerenciadas com `apiKeyHelper`, duas variáveis `env`, e uma regra de negação são exportadas como `{"apiKeyHelper":"[REDACTED]","env":{"HTTPS_PROXY":"[REDACTED]","CLAUDE_CODE_ENABLE_TELEMETRY":"[REDACTED]"},"permissions":{"deny":["Read([REDACTED])"]}}`.

  Claude Code corta o valor em 8 KB de UTF-8, e o valor cortado não é JSON válido
* `managed_settings.settings_truncated` (quando `managed_settings.settings` está presente): `true` quando Claude Code cortou `managed_settings.settings` em 8 KB, `false` caso contrário. Emitido como um booleano, não uma string

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

Claude Code conta cada resposta de streaming em relação às métricas de custo e token exatamente uma vez, incluindo quando um gateway ou proxy atrás de `ANTHROPIC_BASE_URL` transmite uso progressivamente em vários frames. Antes da v2.1.214, streams que carregavam uso em mais de um frame inflacionavam `claude_code.cost.usage` e `claude_code.token.usage` por aproximadamente uma solicitação completa extra por frame extra.

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

O atributo `attempt` no evento registra o número total de tentativas. `CLAUDE_CODE_MAX_RETRIES` tem como padrão 10 e é limitado a 15. A partir da v2.1.199, você pode definir `CLAUDE_CODE_RETRY_WATCHDOG` para aumentar o padrão e remover o limite.

Quando a solicitação esgota todas as tentativas em um erro transitório, `attempt` é igual a um a mais do que esse limite efetivo: 11 por padrão, e nunca mais de 16 a menos que o watchdog esteja definido. Um valor menor indica um erro não retentável, como uma resposta `400`, ou uma causa com seu próprio orçamento de tentativas menor. Por exemplo, Claude Code retenta uma falha ao carregar credenciais da AWS ou Google Cloud no máximo duas vezes.

Para distinguir uma sessão que se recuperou de uma que travou, agrupe eventos por `session.id` e verifique se um evento `api_request` posterior existe após o erro.

<h3 id="event-analysis">
  Análise de eventos
</h3>

Os dados de eventos descrevem cada interação do Claude Code em detalhes:

**Padrões de uso de ferramentas**: analise eventos de resultado de ferramentas para identificar:

* Ferramentas mais frequentemente usadas
* Taxas de sucesso da ferramenta
* Tempos médios de execução da ferramenta
* Padrões de erro por tipo de ferramenta

**Monitoramento de desempenho**: rastreie durações de solicitações de API e tempos de execução de ferramentas para identificar gargalos de desempenho.

<h2 id="audit-security-events">
  Auditar eventos de segurança
</h2>

Os eventos OpenTelemetry são a fonte de dados de auditoria para atividade do Claude Code. Cada evento carrega atributos de identidade que vinculam chamadas de ferramenta, atividade MCP e decisões de permissão de volta ao usuário que as acionou. O exportador de logs OTLP pode entregar esses eventos a qualquer plataforma Security Information and Event Management (SIEM) com um receptor OTLP, ou a um OpenTelemetry Collector que encaminha para seu SIEM.

<h3 id="attribute-actions-to-users">
  Atribuir ações a usuários
</h3>

Os [atributos padrão](#standard-attributes) em cada evento incluem a identidade do usuário autenticado: `user.email`, `user.account_uuid`, `user.account_id` e `organization.id` quando conectado com uma conta Claude ou, em uma [sessão na nuvem](/docs/pt/claude-code-on-the-web), quando as credenciais da própria sessão as carregam, mais `user.id` e o `session.id` por sessão. `user.id` é um identificador com escopo de instalação, exceto em sessões do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), onde é o assunto do IdP do token emitido pelo gateway.

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

* `tool_result`: mantém `mcp_server_scope` e um `tool_name` reduzido ao literal `"mcp_tool"` para servidores configurados pelo usuário, omite conteúdo de argumentos. Para servidores integrados do Claude Desktop, em sessões que Claude Desktop possui, também mantém o par `mcp_server_name`/`mcp_tool_name` dentro de `tool_parameters`, a mesma exceção de autoria de host que `tool_decision`, exigindo Claude Code v2.1.214 ou posterior
* `tool_decision`: mantém `tool_source` e um `tool_name` reduzido ao literal `"mcp_tool"` para servidores configurados pelo usuário, omite conteúdo de argumentos. Para servidores integrados do Claude Desktop, em sessões que Claude Desktop possui, também mantém o par `mcp_server_name`/`mcp_tool_name` dentro de `tool_parameters`; `tool_source` e o par de nomes exigem Claude Code v2.1.214 ou posterior
* `mcp_server_connection`: omite `server_name` e a mensagem de erro, mas mantém `is_plugin`, `plugin_id_hash` e `plugin.name`, com nomes de plugins não-Anthropic reduzidos ao literal `"third-party"`, para que servidores fornecidos por plugins permaneçam distinguíveis sem registro detalhado

<h3 id="map-security-questions-to-events">
  Mapear questões de segurança para eventos
</h3>

Ao construir regras de detecção, procure o sinal que você deseja monitorar e consulte seu backend para o evento correspondente e atributos:

| Sinal                                                                                                                                          | Evento                                                                                 | Atributos-chave                                                                                                                                                                                                                               |
| ---------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Chamada de ferramenta permitida ou negada, e por quê                                                                                           | `tool_decision`                                                                        | `decision`, `source`, `tool_name`, `tool_parameters`                                                                                                                                                                                          |
| Escalação de modo de permissão                                                                                                                 | `permission_mode_changed`                                                              | `from_mode`, `to_mode`, `trigger`                                                                                                                                                                                                             |
| Hook de política bloqueou uma ação                                                                                                             | `hook_execution_complete`                                                              | `hook_event`, `num_blocking`                                                                                                                                                                                                                  |
| Login, logout e falha de autenticação                                                                                                          | `auth`                                                                                 | `action`, `success`, `error_category`                                                                                                                                                                                                         |
| Conexão do servidor MCP ou falha                                                                                                               | `mcp_server_connection`                                                                | `status`, `server_name`, `is_plugin`, `error_code`                                                                                                                                                                                            |
| Plugin instalado e sua origem                                                                                                                  | `plugin_installed`                                                                     | `plugin.name`, `marketplace.name`, `marketplace.is_official`                                                                                                                                                                                  |
| Comandos executados e arquivos tocados                                                                                                         | `tool_result` (executado) ou `tool_decision` (rejeitado) com `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (apenas `tool_result`)                                                                                                                                                                                        |
| Quais fontes de configurações gerenciadas uma máquina executa, se seu auxiliar de política está saudável e por que uma máquina recusou iniciar | `managed_settings_resolved`                                                            | `managed_settings.trigger`, `managed_settings.sources`, `managed_settings.source_behavior`, `managed_settings.helper.state`, `error.type`; `managed_settings.settings` e `managed_settings.resolved_sha256` com `OTEL_LOG_MANAGED_SETTINGS=1` |

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

Para confirmar que os eventos chegam, envie um prompt em uma sessão executada sob essa configuração e verifique seu SIEM para o evento `claude_code.user_prompt`. Se nada chegar, execute `claude --debug` e verifique o log de depuração para erros de exportação `[3P telemetry]`.

<h2 id="backend-considerations">
  Considerações de backend
</h2>

Sua escolha de backends de métricas, logs e rastreamentos determina os tipos de análises que você pode realizar:

<h3 id="for-metrics">
  Para métricas
</h3>

* **Bancos de dados de série temporal**: Cálculos de taxa, métricas agregadas
* **Armazenamentos colunares**: Consultas complexas, análise de usuário único
* **Plataformas de observabilidade completas**: Consultas avançadas, visualização, alertas

<h3 id="for-events/logs">
  Para eventos/logs
</h3>

* **Sistemas de agregação de logs**: Busca de texto completo, análise de logs
* **Armazenamentos colunares**: Análise de eventos estruturados
* **Plataformas de observabilidade completas**: Correlação entre métricas e eventos

<h3 id="for-traces">
  Para rastreamentos
</h3>

Escolha um backend que suporte armazenamento de rastreamento distribuído e correlação de span:

* **Sistemas de rastreamento distribuído**: Visualização de span, waterfalls de solicitação, análise de latência
* **Plataformas de observabilidade completas**: Busca de rastreamento e correlação com métricas e logs

Para organizações que exigem métricas de Usuário Ativo Diário/Semanal/Mensal (DAU/WAU/MAU), considere backends que suportam consultas eficientes de valor único.

<h2 id="service-information">
  Informações de serviço
</h2>

Todas as métricas e eventos são exportados com os seguintes atributos de recurso:

* `service.name`: `claude-code` para sessões de terminal, `claude-code-desktop` para sessões iniciadas a partir da aba Code no [aplicativo Claude Desktop](/docs/pt/desktop)
* `service.version`: Versão atual do Claude Code, ou a versão do aplicativo Desktop para sessões da aba Code
* `os.type`: Tipo de sistema operacional (por exemplo, `linux`, `darwin`, `windows`)
* `os.version`: String de versão do sistema operacional
* `host.arch`: Arquitetura do host (por exemplo, `amd64`, `arm64`)
* `wsl.version`: Número de versão do WSL (apenas presente ao executar no Windows Subsystem for Linux)
* Nome do Medidor: `com.anthropic.claude_code`

Se seus pipelines de coletor ou painéis filtrarem em `service.name = claude-code`, adicione `claude-code-desktop` ao filtro para também capturar telemetria de sessões da aba Code.

<h2 id="roi-measurement-resources">
  Recursos de medição de ROI
</h2>

Para um guia abrangente sobre como medir o retorno sobre investimento para Claude Code, incluindo configuração de telemetria, análise de custo, métricas de produtividade e relatórios automatizados, consulte o [Guia de Medição de ROI do Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Este repositório fornece configurações Docker Compose prontas para uso, configurações Prometheus e OpenTelemetry, e modelos para gerar relatórios de produtividade integrados com ferramentas como Linear.

<h2 id="security-and-privacy">
  Segurança e privacidade
</h2>

* A exportação OpenTelemetry para seu backend é opt-in e requer configuração explícita. Para a telemetria operacional separada da Anthropic e como desabilitá-la, consulte [Uso de dados](/docs/pt/data-usage#telemetry-services)
* Conteúdos de arquivo brutos e trechos de código não são incluídos em métricas ou eventos. Os spans de rastreamento são um caminho de dados separado: veja o ponto `OTEL_LOG_TOOL_CONTENT` abaixo
* Quando autenticado via OAuth, `user.email` é incluído em atributos de telemetria, enviado apenas para o endpoint OTel que você configura, nunca para a Anthropic. Se isso for uma preocupação para sua organização, trabalhe com seu backend de telemetria para filtrar ou reduzir este campo
* O conteúdo do prompt do usuário não é coletado por padrão. Apenas o comprimento do prompt é registrado. Para incluir conteúdo do prompt, defina `OTEL_LOG_USER_PROMPTS=1`. Sob rastreamento beta detalhado, esta variável alcança mais do que apenas texto de prompt: ela também controla o [atributo de span `new_context`](#new-context-gates), que carrega resultados de ferramenta no span `claude_code.llm_request`
* O texto de resposta do assistente não é coletado por padrão. Apenas o comprimento da resposta é registrado. Para incluir texto de resposta, defina `OTEL_LOG_ASSISTANT_RESPONSES=1`. Como todos os dados OpenTelemetry do Claude Code, o texto de resposta é enviado apenas para o endpoint OTel que você configura, nunca para a Anthropic. Quando esta variável não está definida, `OTEL_LOG_USER_PROMPTS` é usado como fallback, portanto defina `OTEL_LOG_ASSISTANT_RESPONSES=0` se você quiser conteúdo de prompt sem conteúdo de resposta
* Argumentos de entrada de ferramenta e parâmetros não são registrados por padrão. Para incluí-los, defina `OTEL_LOG_TOOL_DETAILS=1`. Para os servidores integrados do Claude Desktop, em sessões que o Claude Desktop possui, `tool_decision` e `tool_result` carregam o par `mcp_server_name`/`mcp_tool_name`, nomes criados pelo host em vez de conteúdo de argumentos, mesmo com a flag desativada. A exceção requer Claude Code v2.1.214 ou posterior. Estes dados são enviados apenas para o endpoint OTEL que você configura, nunca para a Anthropic. Os argumentos ainda podem conter valores sensíveis, portanto configure seu backend de telemetria para filtrar ou reduzir esses atributos conforme necessário. Quando ativado:
  * Eventos `tool_result` e `tool_decision` incluem um atributo `tool_parameters` com comandos Bash, nomes de servidor MCP e ferramenta, e nomes de skill. Campos como `full_command` são emitidos sem truncamento
  * Eventos `tool_result` adicionalmente incluem um atributo `tool_input` com caminhos de arquivo, URLs, padrões de busca e outros argumentos. Valores individuais com mais de 512 caracteres são truncados e o total é limitado a \~4 K caracteres
  * Eventos `user_prompt` incluem o `command_name` verbatim para comandos customizados, plugin e MCP
  * Spans de rastreamento incluem o mesmo atributo `tool_input` e atributos derivados de entrada como `file_path`, com o mesmo truncamento que `tool_input`
* O conteúdo de ferramenta não é registrado em spans de rastreamento por padrão. Para incluí-lo, defina `OTEL_LOG_TOOL_CONTENT=1`. O span `claude_code.tool` então carrega um [evento de span `tool.output`](#tool-output-span-event) com conteúdos de arquivo brutos e saída de comando Bash, truncado no limite de conteúdo (60 KB por padrão) por atributo. O conteúdo de ferramenta também alcança spans através de [`new_context`, cujo controle difere por span](#new-context-gates). Configure seu backend de telemetria para filtrar ou reduzir esses atributos conforme necessário
* Corpos de solicitação e resposta da API Anthropic Messages brutos não são registrados por padrão. Para incluí-los, defina `OTEL_LOG_RAW_API_BODIES` em suas configurações de shell, usuário ou gerenciadas. É ignorado em [configurações de projeto e local](/docs/pt/settings-reference#variables-claude-code-ignores-in-env). Os corpos contêm o histórico de conversa completo, incluindo o prompt do sistema, cada turno anterior de usuário e assistente, e resultados de ferramenta, portanto ativar isso implica consentimento para tudo que os outros sinalizadores de conteúdo `OTEL_LOG_*` revelariam. O Claude Code sempre reduz o conteúdo de pensamento estendido do Claude desses corpos, independentemente de outras configurações. O valor que você define determina como o Claude Code entrega os corpos:
  * Com `=1`, Claude Code emite eventos de log `api_request_body` e `api_response_body` para cada chamada de API. O atributo `body` dos eventos carrega a carga útil serializada em JSON, truncada no limite de conteúdo (60 KB por padrão)
  * Com `=file:<dir>`, Claude Code escreve corpos não truncados em arquivos `.request.json` e `.response.json` sob esse diretório, e os eventos carregam um caminho `body_ref` em vez do corpo inline. Envie o diretório com um coletor de log ou sidecar em vez de através do fluxo de telemetria.

    Para cada resposta bem-sucedida, Claude Code também anexa uma linha a `index.jsonl` nesse diretório, vinculando o arquivo de resposta ao arquivo de solicitação que o produziu e à mensagem de transcrição em que se tornou. Cada linha não contém conteúdo de mensagem, e a seção [evento de corpo de resposta da API](#api-response-body-event) lista seus campos. O arquivo de índice requer Claude Code v2.1.274 ou posterior

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Monitorar Claude Code no Amazon Bedrock
</h2>

Para orientação detalhada de monitoramento de uso do Claude Code para Amazon Bedrock, consulte [Implementação de Monitoramento do Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
