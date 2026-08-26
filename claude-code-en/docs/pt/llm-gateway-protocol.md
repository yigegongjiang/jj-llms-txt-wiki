> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência do protocolo do gateway

> O contrato de API entre Claude Code e um gateway LLM: endpoints, headers e campos de corpo para encaminhar, degradação de recursos quando campos são removidos, headers de atribuição para rastreamento de custos e descoberta de modelos.

Esta página documenta as solicitações que Claude Code envia para um gateway, incluindo os endpoints que ele chama, os headers e campos de corpo que o gateway deve encaminhar, e quais recursos deixam de funcionar quando não o faz. É escrita para operadores que configuram um produto gateway para funcionar com Claude Code.

Um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) em execução fornece uma versão legível por máquina deste contrato em `GET /protocol`, cobrindo os mesmos requisitos de encaminhamento mais os endpoints específicos do gateway de aplicativos Claude para login SSO, entrega de configurações gerenciadas e telemetria. O gateway de aplicativos Claude é executado a partir do mesmo binário `claude` que a CLI, portanto o [guia de início rápido do gateway de aplicativos Claude](/docs/pt/claude-apps-gateway#quickstart) é o caminho mais curto para uma instância em execução da qual você pode buscar a especificação.

<Note>
  * Para implantar um gateway existente ou de terceiros para sua organização, consulte [Implantar um gateway LLM](/docs/pt/llm-gateway-rollout)
  * Se você é um desenvolvedor individual autenticando Claude Code em um gateway com uma credencial que lhe foi fornecida, consulte [Conectar Claude Code a um gateway LLM](/docs/pt/llm-gateway-connect)
</Note>

Esta página cobre:

* [Formatos de API](#api-formats) e os endpoints a servir para cada um
* [Headers de solicitação](#request-headers): quais devem chegar ao upstream e quais seu gateway pode consumir
* O [bloco de atribuição do prompt do sistema](#system-prompt-attribution-block) e como ele interage com o cache de prompt
* [Passagem de recursos](#feature-pass-through): o que quebra quando headers ou campos de corpo são removidos
* [Descoberta de modelos](#model-discovery)

Esta página usa dois termos para o que seu gateway faz com cada header e campo de corpo:

* **Encaminhar inalterado**: passá-lo para o upstream byte por byte
* **Consumir**: o gateway pode lê-lo para roteamento, atribuição ou rastreamento e não precisa encaminhá-lo

Qualquer coisa não marcada como encaminhar inalterado é sua para consumir ou ignorar.

<h2 id="api-formats">
  Formatos de API
</h2>

Um gateway deve expor pelo menos um dos seguintes formatos de API para clientes Claude Code. Qual formato Claude Code fala é determinado pela configuração do cliente: a variável na coluna Selecionado por da tabela abaixo aponta Claude Code para seu gateway nesse formato. Google Cloud's Agent Platform é o endpoint Claude do Google Cloud, anteriormente Vertex AI; seus nomes de variáveis mantêm a grafia `VERTEX`.

| Formato                                  | Selecionado por                                              | Endpoints                                                                | Encaminhar inalterado                                                                                                |
| :--------------------------------------- | :----------------------------------------------------------- | :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                         | `/v1/messages`, `/v1/messages/count_tokens` (opcional)                   | headers de solicitação `anthropic-beta` e `anthropic-version`                                                        |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` com `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`    | campos de corpo de solicitação `anthropic_beta` e `anthropic_version`                                                |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` com `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (opcional) | headers de solicitação `anthropic-beta` e `anthropic-version`, e o campo de corpo de solicitação `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry e Claude Platform on AWS
</h3>

Microsoft Foundry e a [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) implementam o formato Anthropic Messages. Claude Code roteia para eles através de suas próprias variáveis, `ANTHROPIC_FOUNDRY_BASE_URL` e `ANTHROPIC_AWS_BASE_URL`, mas um gateway fronteando qualquer um deles implementa a linha Anthropic Messages acima. Um gateway fronteando a Claude Platform on AWS também deve encaminhar o header `anthropic-workspace-id`, que [essa plataforma requer em cada solicitação](/docs/pt/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Endpoints opcionais e tráfego de inicialização
</h3>

Endpoints de contagem de tokens são os únicos opcionais: quando estão ausentes, Claude Code estima o uso de contexto localmente. Solicitações de inferência são postadas em `/v1/messages?beta=true`, então corresponda no caminho, não na URL completa. O método Google Cloud's Agent Platform anexa sufixos ao caminho do modelo do editor, como em `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`.

Um gateway também vê tráfego de inicialização de melhor esforço que pode rejeitar sem quebrar nada: uma sonda de conectividade `HEAD /`, e em gateways no formato Amazon Bedrock uma solicitação `GET /inference-profiles?type=SYSTEM_DEFINED`.

<h3 id="streaming">
  Streaming
</h3>

Respostas de inferência devem fazer streaming. Claude Code consome eventos enviados pelo servidor conforme chegam, então um gateway que armazena em buffer respostas completas antes de retransmiti-las congela o cliente.

<h3 id="format-mismatch-with-the-upstream">
  Incompatibilidade de formato com o upstream
</h3>

Qual formato o cliente fala determina o que seu gateway recebe. O modo de falha comum é uma incompatibilidade entre o formato que o cliente envia para seu gateway e o formato que o provedor upstream atrás dele aceita.

* Quando o cliente fala o formato Amazon Bedrock ou Google Cloud's Agent Platform, Claude Code envia apenas o subconjunto de seu conjunto completo de capacidades que esses provedores aceitam
* Quando o cliente fala o formato Anthropic Messages, Claude Code envia o conjunto completo, mesmo que seu gateway encaminhe para um upstream Amazon Bedrock ou Google Cloud's Agent Platform

Fazer essa ponte é trabalho do seu gateway. [Passagem de recursos](#feature-pass-through) descreve o que quebra quando não o faz.

<h2 id="request-headers">
  Headers de solicitação
</h2>

Claude Code inclui esses headers em solicitações de API. Nomes de headers não diferenciam maiúsculas de minúsculas no fio. Encaminhe `anthropic-version` e `anthropic-beta` inalterados, mais `anthropic-workspace-id` quando o upstream é a [Claude Platform on AWS](/docs/pt/claude-platform-on-aws); o resto o gateway pode consumir para roteamento, atribuição e rastreamento, e não precisa encaminhar.

| Header                          | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Authorization`, `x-api-key`    | A credencial do gateway do desenvolvedor, em um ou ambos os headers dependendo de qual [variável de credencial](/docs/pt/llm-gateway-connect#set-the-credential-variable) eles definiram                                                                                                                                                                                                                                                                                                             |
| `anthropic-version`             | Versão da API, atualmente `2023-06-01`. Solicitações no formato Amazon Bedrock e Agent Platform do Google Cloud também carregam o campo de corpo `anthropic_version`, cujo valor é a string de dialeto do provedor, não o valor deste header                                                                                                                                                                                                                                                    |
| `anthropic-beta`                | Valores de capacidade separados por vírgula para a solicitação. Encaminhe o header verbatim; não faça uma lista de permissões de valores individuais, porque o conjunto muda com lançamentos de Claude Code. Quando o desenvolvedor se autentica com um login claude.ai, que é possível quando `ANTHROPIC_BASE_URL` é definido sem uma variável de credencial de gateway, este header também carrega uma capacidade OAuth que o upstream requer, e removê-lo falha essas solicitações com `401` |
| `x-claude-code-session-id`      | Um identificador único para a sessão atual de Claude Code. Use-o para agregar todas as solicitações de uma sessão sem analisar corpos de solicitação                                                                                                                                                                                                                                                                                                                                            |
| `x-claude-code-agent-id`        | Identificador do [subagente](/docs/pt/sub-agents) que emitiu a solicitação, presente apenas em solicitações de um agente que Claude Code gerou dentro da sessão. Use-o com o ID da sessão para atribuir custo a agentes paralelos                                                                                                                                                                                                                                                                    |
| `x-claude-code-parent-agent-id` | Identificador do agente que gerou o agente solicitante, presente apenas para agentes aninhados                                                                                                                                                                                                                                                                                                                                                                                                  |

IDs de subagentes são gerados novamente para cada geração. Agentes companheiros, os membros nomeados de uma [equipe de agentes](/docs/pt/agent-teams), reutilizam um ID estável baseado em nome entre reconexões. Em ambos os casos, o ID identifica um agente, não uma pessoa ou dispositivo, então não trate o header de ID de agente como um identificador de usuário.

Se seus desenvolvedores definirem `ANTHROPIC_CUSTOM_HEADERS`, esses headers também aparecem em solicitações.

<h3 id="forward-as-open-lists">
  Encaminhar como listas abertas
</h3>

Trate os headers e campos de corpo como listas abertas, não fechadas. Claude Code ganha capacidades ao longo dos lançamentos, e elas chegam como novos valores `anthropic-beta`, novos campos de corpo de solicitação e ocasionalmente novos headers `anthropic-*` ou `x-claude-code-*`.

Ao encaminhar para um upstream no formato Anthropic, passe headers de solicitação `anthropic-*` e campos de corpo de solicitação através inalterados em vez de fazer uma lista de permissões dos que você vê hoje. Um gateway fixado a uma lista observada remove o header ou campo da próxima capacidade e quebra-o no lançamento que a introduz.

A exceção é um upstream não-Anthropic, como Amazon Bedrock ou Agent Platform do Google Cloud, onde fazer a ponte da diferença de schema é trabalho do gateway; consulte [passagem de recursos](#feature-pass-through).

<h2 id="system-prompt-attribution-block">
  Bloco de atribuição do prompt do sistema
</h2>

Claude Code prepara um bloco de atribuição curto para o prompt do sistema contendo a versão do cliente e uma impressão digital derivada da conversa. O endpoint `api.anthropic.com` remove o bloco antes do processamento quando ele chega inalterado como o primeiro bloco do sistema, portanto não afeta o cache de prompt de primeira parte. Qualquer outro upstream o recebe como parte do prompt.

A remoção é posicional, portanto funciona apenas quando o gateway encaminha o array `system` inalterado. Para manter o bloco fora do prompt sem perder outro conteúdo do sistema:

* Encaminhe o array `system` exatamente como recebido, mantendo o bloco primeiro: adicionar outro bloco do sistema, reordenar o array ou convertê-lo em uma única string derrota a remoção, e o bloco então chega ao modelo e à chave do cache de prompt.
* Mantenha o bloco em sua própria entrada de array: o endpoint trata um bloco mesclado que começa com o cabeçalho de atribuição como atribuição em sua totalidade e descarta tudo mesclado nele, incluindo o resto do prompt do sistema.
* Se seu gateway deve reformular o conteúdo do sistema, defina [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/pt/env-vars) para que Claude Code omita o bloco. Anthropic e os endpoints Claude dos provedores de nuvem leem o bloco para atribuição, portanto omita-o no cliente em vez de removê-lo ou movê-lo no gateway.

Solicitações que chegam ao endpoint inalteradas não são afetadas.

A partir de Claude Code v2.1.181, o bloco é estável pela vida útil de uma conversa quando solicitações são roteadas através de uma URL base personalizada, então um cache de prompt do lado do gateway com chave no corpo completo da solicitação funciona sem desabilitá-lo. Antes de v2.1.181, o bloco incluía um token por solicitação; nessas versões, defina `CLAUDE_CODE_ATTRIBUTION_HEADER=0` se seu gateway implementar tal cache.

<h2 id="feature-pass-through">
  Passagem de recursos
</h2>

Claude Code trata um gateway `ANTHROPIC_BASE_URL` como um endpoint no formato Anthropic e envia a ele os headers beta e campos de corpo de solicitação que envia para `api.anthropic.com`, exceto um pequeno conjunto de diagnósticos e padrões reservados para conexões diretas, como o padrão de streaming de ferramenta de granulação fina coberto abaixo. Esse conjunto varia por lançamento, então não dependa de seu conteúdo.

Capacidades que adicionam campos de corpo os emparelham com um header beta, e o par viaja junto. Um gateway que remove o header enquanto passa o corpo, ou encaminha um corpo no formato Anthropic para um upstream com um schema diferente, produz erros `400` difíceis; apenas quando ambas as metades estão ausentes juntas o recurso desativa silenciosamente. Um gateway que reescreve ou redige corpos de solicitação para inspeção de conteúdo quebra o emparelhamento da mesma forma que remover o faz, então inspecione sem modificar. A tabela observa onde um recurso se desvia do emparelhamento.

Streaming de ferramenta de granulação fina é um dos padrões de conexão direta: está desativado por padrão sempre que solicitações são roteadas através de uma URL base personalizada, e um gateway o recebe quando desenvolvedores definem [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/pt/env-vars).

| Recurso                                                                                                                                                                                                                                            | Header e par de corpo                                                                                                                                                                                         | Sintoma quando quebrado                                                                                                                           | Remediação                                                                                                                          |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------- |
| [Raciocínio adaptativo](/docs/pt/model-config#adjust-effort-level)                                                                                                                                                                                      | Sem header beta. Claude Code envia `thinking: {"type": "adaptive"}` para Claude 4.6 e posterior, e trata nomes de modelos que não reconhece, como aliases de gateway, como modelos atuais que recebem o campo | `400` nomeando o campo `thinking` ou a tag `adaptive` quando a compilação do modelo upstream não a aceita                                         | Atualize o upstream. Em Opus 4.6 e Sonnet 4.6, desenvolvedores podem definir `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` em vez disso |
| [Gerenciamento de contexto](https://platform.claude.com/docs/en/build-with-claude/context-management)                                                                                                                                              | Header beta de gerenciamento de contexto emparelhado com o campo de corpo `context_management`                                                                                                                | `400` com `Extra inputs are not permitted`. Comum quando um gateway aceita solicitações no formato Anthropic mas as encaminha para Amazon Bedrock | Encaminhe ambos, ou [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/pt/env-vars)                                                      |
| [Contexto estendido](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) e [pensamento intercalado](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | Apenas headers beta, sem campo de corpo                                                                                                                                                                       | Silenciosamente indisponível quando o header é removido; o upstream nunca vê a solicitação de capacidade                                          | Encaminhe `anthropic-beta` verbatim                                                                                                 |
| Campos de [ferramenta](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview) beta                                                                                                                                                | Headers beta relacionados a ferramentas emparelhados com campos de schema de ferramenta como `strict` e `defer_loading`                                                                                       | `400` nomeando o campo de schema de ferramenta não reconhecido quando o corpo passa sem seu header                                                | Encaminhe ambos, ou `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`                                                                      |
| [Esforço](https://platform.claude.com/docs/en/build-with-claude/effort) e [saídas estruturadas](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                          | O campo de corpo `output_config` carrega esforço, formato de saída estruturada e configurações de orçamento de tarefa; cada um emparelhado com seu próprio header beta                                        | `400` nomeando `output_config`, frequentemente `Extra inputs are not permitted`, em upstreams Bedrock e Agent Platform                            | Encaminhe o campo e seus headers juntos                                                                                             |
| [Contagem de tokens](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                         | Sem emparelhamento beta; usa o endpoint `count_tokens`                                                                                                                                                        | Claude Code volta a estimar o uso de contexto localmente                                                                                          | Exponha o endpoint se quiser contagens exatas                                                                                       |

As [variáveis](/docs/pt/model-config) `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` declaram capacidades de modelo apenas nas configurações do provedor: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, e [`CLAUDE_CODE_USE_MANTLE`](/docs/pt/amazon-bedrock#use-the-mantle-endpoint). Elas não têm efeito atrás de um gateway `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Retry automático e encaminhamento de erro
</h3>

Claude Code tenta novamente automaticamente após algumas rejeições upstream e desabilita a capacidade rejeitada pelo resto da conversa. Rejeições do campo `thinking`, de [assinaturas de pensamento](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), e de mensagens de sistema no meio da conversa todas se recuperam dessa forma. Rejeições de gerenciamento de contexto e campo de schema de ferramenta não tentam novamente; esses erros `400` chegam ao desenvolvedor.

A lógica de retry corresponde à redação de erro do upstream, então encaminhe corpos de resposta de erro inalterados. Um gateway que envolve erros upstream em seu próprio envelope quebra o caminho de recuperação mesmo quando preserva o código de status.

<h3 id="disable-pre-release-capabilities">
  Desabilitar capacidades de pré-lançamento
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` impede que Claude Code envie capacidades de pré-lançamento e seus campos de corpo em cada provedor, incluindo gerenciamento de contexto e campos de ferramenta beta. Não afeta raciocínio adaptativo, que é selecionado por modelo em vez de por beta, e nunca suprime a capacidade OAuth que autenticação de assinatura requer.

O conjunto de capacidades que Claude Code envia cresce ao longo dos lançamentos. Para strings de header beta atuais, consulte a [referência de headers beta](https://platform.claude.com/docs/en/api/beta-headers); teste seu gateway contra novos lançamentos de Claude Code em vez de fixar a uma lista observada.

<h2 id="model-discovery">
  Descoberta de modelos
</h2>

Quando `ANTHROPIC_BASE_URL` aponta para um gateway que expõe o formato Anthropic Messages, Claude Code pode consultar o endpoint `/v1/models` do gateway na inicialização e adicionar os modelos retornados ao seletor `/model`.

Desenvolvedores o habilitam definindo [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/pt/env-vars), em seu próprio ambiente ou através de configurações gerenciadas. A descoberta está desativada por padrão para que gateways apoiados por uma chave de API compartilhada não exponham cada modelo que a chave pode acessar a cada usuário. Isso requer Claude Code v2.1.129 ou posterior.

<h3 id="when-discovery-runs">
  Quando a descoberta é executada
</h3>

A descoberta se aplica apenas ao formato Anthropic Messages. Não é executada quando:

* Qualquer variável de provedor `CLAUDE_CODE_USE_*` é definida, mesmo se `ANTHROPIC_BASE_URL` também for definido
* `ANTHROPIC_BASE_URL` não está definido ou aponta para `api.anthropic.com`
* Tráfego não essencial está desabilitado, através de [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars) ou política organizacional

<h3 id="request-and-response">
  Solicitação e resposta
</h3>

A solicitação é `GET /v1/models?limit=1000` com um timeout de 3 segundos, e qualquer redirecionamento é tratado como falha para que a credencial não vaze para um alvo de redirecionamento. Um gateway que responde lentamente ou redireciona `/v1/models`, mesmo `http` para `https`, falha na descoberta silenciosamente; sirva o endpoint diretamente na URL base configurada.

A solicitação de descoberta envia exatamente um header de credencial:

* `ANTHROPIC_AUTH_TOKEN` como um token bearer, quando definido
* Caso contrário, a chave de API resolvida, incluindo um valor [`apiKeyHelper`](/docs/pt/llm-gateway-connect#rotate-credentials-with-apikeyhelper), no header `x-api-key`

Isso difere de solicitações de inferência, que enviam um valor helper em ambos os headers. Um gateway que autentica `/v1/models` deve aceitar `x-api-key` para implantações helper. Qualquer header de `ANTHROPIC_CUSTOM_HEADERS` também é incluído.

Claude Code lê `id` e o `display_name` opcional de cada entrada no array `data` da resposta, e ignora entradas cujo `id` não começa com `claude` ou `anthropic`:

```json theme={null}
{
  "data": [
    { "id": "claude-sonnet-4-6", "display_name": "Claude Sonnet 4.6" },
    { "id": "claude-opus-4-8" }
  ]
}
```

<h3 id="picker-entries-and-caching">
  Entradas do seletor e cache
</h3>

O seletor é a lista de modelos interativa que abre quando um desenvolvedor executa `/model` em Claude Code. Cada entrada descoberta é rotulada "Do gateway" e usa `display_name` quando fornecido. A [configuração gerenciada `availableModels`](/docs/pt/settings#available-settings) limita o que a descoberta pode adicionar.

Um ID descoberto é ignorado quando corresponde exatamente a uma linha já no seletor, ou quando tanto o ID descoberto quanto o existente se resolvem para [Fable](/docs/pt/model-config#work-with-fable-5). A partir de Claude Code v2.1.197, um ID explícito descoberto também é incorporado em uma entrada integrada quando ambos se resolvem para o mesmo modelo. Linhas integradas são chaveadas em aliases como `sonnet`, então um ID descoberto explícito do modelo para o qual o alias atualmente se resolve, como `claude-sonnet-5`, colapsa na linha `sonnet`, enquanto um ID para o qual o alias não se resolve, como `claude-sonnet-4-6`, ainda adiciona sua própria linha "Do gateway" ao lado da entrada integrada.

Os resultados são armazenados em cache em `~/.claude/cache/gateway-models.json`, ou `%USERPROFILE%\.claude\cache\gateway-models.json` no Windows, e atualizados em cada inicialização. Se a solicitação falhar ou o gateway não implementar `/v1/models`, o seletor volta para a lista em cache da inicialização anterior ou para a lista de modelos integrada. Se seu gateway serve modelos Claude sob aliases que não correspondem ao filtro de descoberta, desenvolvedores podem adicionar esses aliases manualmente com as [variáveis de configuração de modelo](/docs/pt/model-config).

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para o resto do conjunto de documentação do gateway e as referências de API subjacentes:

* [Visão geral de gateway](/docs/pt/gateways): o que é um gateway e como escolher entre o gateway de aplicativos Claude e outro produto
* [Outros gateways LLM](/docs/pt/llm-gateway): como implantar um gateway que sua organização executa e como ele interage com assinaturas claude.ai
* [Implantar um gateway LLM para sua organização](/docs/pt/llm-gateway-rollout): a lista de verificação do administrador que usa este contrato
* [Conectar Claude Code a um gateway LLM](/docs/pt/llm-gateway-connect): configuração por desenvolvedor e a tabela de solução de problemas
* [Referência de headers beta](https://platform.claude.com/docs/en/api/beta-headers): o conjunto atual de valores `anthropic-beta`
* [Messages API](https://platform.claude.com/docs/en/api/messages): o formato de API que um gateway no formato Anthropic implementa
