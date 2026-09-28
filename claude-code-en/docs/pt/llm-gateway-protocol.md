> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guia de compatibilidade do gateway Claude Code

> Mantenha um gateway LLM compatível com Claude Code: os endpoints que ele chama, os headers e campos de corpo a encaminhar, e o que quebra quando são removidos.

Esta página documenta as solicitações que Claude Code envia para um gateway, incluindo os endpoints que ele chama, os headers e campos de corpo que o gateway deve encaminhar, e quais recursos deixam de funcionar quando não o faz. É escrita para operadores configurando um produto gateway para funcionar com Claude Code.

O [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), o gateway auto-hospedado da Anthropic, serve sua própria referência de endpoint em `GET /protocol`, cobrindo os endpoints de sign-in, inferência, configurações gerenciadas, descoberta de modelo e telemetria desse gateway. É um documento separado deste guia.

<Note>
  * Para implantar um gateway existente ou de terceiros para sua organização, consulte [Implantar um gateway LLM](/docs/pt/llm-gateway-rollout)
  * Se você é um desenvolvedor individual autenticando Claude Code em um gateway com uma credencial que lhe foi fornecida, consulte [Conectar Claude Code a um gateway LLM](/docs/pt/llm-gateway-connect)
</Note>

Esta página cobre:

* [Formatos de API](#api-formats) e os endpoints a servir para cada um
* [Comportamento do cliente por método de conexão](#how-the-connection-method-changes-client-behavior): como IDs de modelo, valores de `anthropic-beta`, campos de solicitação e padrões diferem entre os formatos e um sign-in de gateway de aplicativos Claude
* [Headers de solicitação](#request-headers): quais devem chegar ao upstream e quais seu gateway pode consumir
* [Headers de resposta](#response-headers): o que retornar para que a detecção de travamento, tentativas e exibição de limite de uso funcionem
* O [bloco de atribuição de prompt do sistema](#system-prompt-attribution-block) e como ele interage com cache de prompt
* [Passagem de recursos](#feature-pass-through): o que quebra quando headers ou campos de corpo são removidos
* [Descoberta de modelo](#model-discovery)

Esta página usa dois termos para o que seu gateway faz com cada header e campo de corpo:

* **Encaminhar inalterado**: passá-lo para o upstream byte por byte
* **Consumir**: o gateway pode lê-lo para roteamento, atribuição ou rastreamento e não precisa encaminhá-lo

Qualquer coisa não marcada como encaminhar inalterado é sua para consumir ou ignorar.

<h2 id="api-formats">
  Formatos de API
</h2>

Um gateway deve expor pelo menos um dos seguintes formatos de API para clientes Claude Code. Um cliente escolhe um formato e aponta Claude Code para seu gateway com as variáveis na coluna Selecionado pela tabela abaixo.

Google Cloud's Agent Platform é o endpoint Claude do Google Cloud, anteriormente Vertex AI; seus nomes de variáveis mantêm a grafia `VERTEX`.

| Formato                                  | Selecionado por                                              | Endpoints                                                                                                       | Encaminhar inalterado                                                                                              |
| :--------------------------------------- | :----------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                         | `/v1/messages`, `/v1/messages/count_tokens` (opcional)                                                          | headers de requisição `anthropic-beta` e `anthropic-version`                                                       |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` com `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`, `/model/{model}/count-tokens` (opcional) | campos de corpo de requisição `anthropic_beta` e `anthropic_version`                                               |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` com `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (opcional)                                        | headers de requisição `anthropic-beta` e `anthropic-version`, e o campo de corpo de requisição `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry e Claude Platform on AWS
</h3>

Microsoft Foundry e a [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) implementam o formato Anthropic Messages. Claude Code roteia para eles através de suas próprias variáveis, `ANTHROPIC_FOUNDRY_BASE_URL` e `ANTHROPIC_AWS_BASE_URL`, mas um gateway fronteando qualquer um deles implementa a linha Anthropic Messages acima. Um gateway fronteando a Claude Platform on AWS também deve encaminhar o header `anthropic-workspace-id`, que [essa plataforma requer em cada requisição](/docs/pt/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Endpoints opcionais e tráfego de inicialização
</h3>

Endpoints de contagem de tokens são os únicos opcionais: quando estão ausentes, Claude Code volta para uma estimativa baseada em caracteres do uso de contexto.

Corresponda no caminho, não na URL completa:

* Requisições de inferência postam para `/v1/messages?beta=true`
* O método Google Cloud's Agent Platform sufixos anexam ao caminho do modelo do publisher, como em `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`

Um gateway também vê tráfego de inicialização de melhor esforço que pode rejeitar sem quebrar nada. Um gateway no formato Anthropic Messages recebe uma sonda de aquecimento de conexão `HEAD /api/hello`, que Claude Code pula quando um proxy HTTP ou certificado de cliente está configurado. Um gateway no formato Amazon Bedrock recebe uma requisição `GET /inference-profiles?type=SYSTEM_DEFINED` e, quando o modelo configurado é um perfil de inferência, buscas `GET /inference-profiles/{profile}`.

A verificação de disponibilidade do [fast mode](/docs/pt/fast-mode) nunca aparece nos logs do gateway: ela chama `api.anthropic.com` diretamente em vez de seguir `ANTHROPIC_BASE_URL`, então em uma rede que bloqueia egresso direto para `api.anthropic.com`, o fast mode pode relatar um erro de conectividade enquanto a inferência através do gateway continua funcionando. A [verificação de segurança de domínio WebFetch](/docs/pt/data-usage#webfetch-domain-safety-check) também chama `api.anthropic.com` diretamente. [Use fast mode behind proxies and LLM gateways](/docs/pt/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) cobre as variáveis que o restauram.

<h3 id="streaming">
  Streaming
</h3>

Transmita respostas de inferência em stream. Claude Code lê o stream conforme ele chega, então se seu gateway armazena respostas completas antes de retransmiti-las, Claude Code trava.

Quando o cliente fala o formato Amazon Bedrock, retransmita o corpo da resposta `InvokeModelWithResponseStream` e seu header `Content-Type: application/vnd.amazon.eventstream` sem modificações, e não converta o stream para server-sent events. Veja [Streaming errors behind a gateway or proxy](/docs/pt/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

Encaminhe pings de keep-alive também. Em conexões através de `ANTHROPIC_BASE_URL` ou `ANTHROPIC_AWS_BASE_URL`, Claude Code conta cada byte que seu gateway retransmite, incluindo eventos SSE `ping` e linhas de comentário, e aborta um stream que fica silencioso por 300 segundos por padrão. Os pings do upstream são o único tráfego durante pausas de pensamento longo, então se seu gateway remove ou armazena eles, Claude Code aborta o stream durante essas pausas; [Automatic retries](/docs/pt/errors#automatic-retries) cobre o que um stream abortado relata com base em quanto a resposta havia progredido. Um upstream que não envia pings em absoluto, como o event-stream binário do Amazon Bedrock, deixa essas pausas sem nada para encaminhar. Ao traduzir de tal upstream, emita seus próprios eventos `ping` durante lacunas silenciosas. Gateways alcançados através de `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, ou `ANTHROPIC_FOUNDRY_BASE_URL` não são envolvidos por este watchdog de nível de byte, mesmo quando retransmitem o formato Anthropic Messages; lá, um [timeout ocioso de 5 minutos](/docs/pt/env-vars) aborta um stream silencioso em vez disso, e em conexões `ANTHROPIC_BEDROCK_BASE_URL` você pode adicionar o watchdog de byte com [`CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK`](/docs/pt/env-vars).

<h3 id="format-mismatch-with-the-upstream">
  Incompatibilidade de formato com o upstream
</h3>

Qual formato o cliente fala determina o que seu gateway recebe. O modo de falha comum é uma incompatibilidade entre o formato que o cliente envia para seu gateway e o formato que o provedor upstream atrás dele aceita.

* Quando o cliente fala o formato Amazon Bedrock ou Google Cloud's Agent Platform, Claude Code envia apenas o subconjunto de seu conjunto completo de capacidades que esses provedores aceitam
* Quando o cliente fala o formato Anthropic Messages, Claude Code envia o conjunto completo, mesmo que seu gateway encaminhe para um upstream Amazon Bedrock ou Google Cloud's Agent Platform

Fazer a ponte dessa diferença é trabalho do seu gateway. [Feature pass-through](#feature-pass-through) descreve o que quebra quando não faz.

Se seu upstream é Amazon Bedrock ou Google Cloud's Agent Platform, você pode evitar a ponte expondo o formato desse provedor em vez disso. [Route to a cloud provider through a gateway](/docs/pt/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) mostra a configuração do cliente para esse formato.

<h2 id="how-the-connection-method-changes-client-behavior">
  Como o método de conexão altera o comportamento do cliente
</h2>

A forma como um desenvolvedor se conecta ao seu gateway determina quais IDs de modelo, valores de `anthropic-beta` e campos de solicitação o Claude Code envia, e quais padrões ele aplica. Seu gateway vê um dos três comportamentos de cliente:

* **Formato Amazon Bedrock ou Agent Platform**: o desenvolvedor define `CLAUDE_CODE_USE_BEDROCK=1` com `ANTHROPIC_BEDROCK_BASE_URL`, ou `CLAUDE_CODE_USE_VERTEX=1` com `ANTHROPIC_VERTEX_BASE_URL`, apontando para seu gateway. Claude Code usa os IDs de modelo, campos de solicitação e padrões desse provedor.
* **Formato Anthropic Messages**: o desenvolvedor define `ANTHROPIC_BASE_URL` para seu gateway. Claude Code trata o gateway como a API Claude e não consegue dizer para qual upstream você encaminha.
* **Entrada do gateway de aplicativos Claude**: o desenvolvedor entra em um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway). Esse gateway fala o formato Anthropic Messages, mas pode rotear para qualquer upstream, então Claude Code envia apenas os valores de `anthropic-beta` e suposições de capacidade de modelo que Amazon Bedrock e Agent Platform também aceitam.

<h3 id="requests-and-defaults-by-connection-method">
  Solicitações e padrões por método de conexão
</h3>

A tabela abaixo compara os três métodos de conexão, um comportamento por linha. Ela omite Microsoft Foundry e Claude Platform on AWS, que também usam o formato Anthropic Messages, mas que Claude Code alcança através de suas próprias variáveis. Para esses, consulte as páginas [Microsoft Foundry](/docs/pt/microsoft-foundry) e [Claude Platform on AWS](/docs/pt/claude-platform-on-aws).

| Comportamento                                                                                                                | Formato Amazon Bedrock ou Agent Platform                                                                                                                                                                                             | Formato Anthropic Messages                                                                                                                                                                      | Entrada do gateway de aplicativos Claude                                                                                                |
| :--------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| IDs de modelo em solicitações por padrão                                                                                     | A forma do provedor, como `us.anthropic.claude-opus-4-8` no Amazon Bedrock                                                                                                                                                           | IDs Anthropic, como `claude-opus-4-8`                                                                                                                                                           | IDs Anthropic                                                                                                                           |
| Valores de `anthropic-beta` enviados                                                                                         | O subconjunto que Amazon Bedrock e Agent Platform aceitam                                                                                                                                                                            | O conjunto completo descrito em [feature pass-through](#feature-pass-through), a menos que o desenvolvedor defina [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities) | O subconjunto que Amazon Bedrock e Agent Platform aceitam                                                                               |
| Campos de solicitação para um ID de modelo que Claude Code não reconhece, como um alias de gateway                           | Pensamento com um orçamento fixo em vez de raciocínio adaptativo, e nenhum campo de esforço ou gerenciamento de contexto                                                                                                             | Tudo que os modelos Claude atuais aceitam na API Claude, incluindo raciocínio adaptativo, esforço e gerenciamento de contexto, que um upstream Amazon Bedrock ou Agent Platform pode rejeitar   | Mesmo que o formato Amazon Bedrock ou Agent Platform                                                                                    |
| [TTL de cache de prompt](/docs/pt/prompt-caching#choose-the-ttl-yourself) de uma hora quando um desenvolvedor opta por participar | Solicitado através do campo `ttl` em `cache_control`, sem valor beta                                                                                                                                                                 | Solicitado através do campo `ttl` mais um valor `extended-cache-ttl` em `anthropic-beta`, que você deve encaminhar                                                                              | Consulte a tabela [disponibilidade e limitações](/docs/pt/claude-apps-gateway#availability-and-limitations) do gateway de aplicativos Claude |
| Modelo para [tarefas em segundo plano](/docs/pt/costs#background-token-usage) a menos que `ANTHROPIC_DEFAULT_HAIKU_MODEL` fixe um | O modelo Sonnet padrão, ou o modelo principal uma vez que um seja selecionado, conforme as páginas [Amazon Bedrock](/docs/pt/amazon-bedrock#4-pin-model-versions) e [Agent Platform](/docs/pt/google-vertex-ai#5-pin-model-versions) descrevem | O modelo principal, ou o modelo Haiku padrão quando `ANTHROPIC_API_KEY` ou `apiKeyHelper` fornece uma chave do Console Anthropic e `ANTHROPIC_AUTH_TOKEN` não está definido                     | O modelo principal                                                                                                                      |

Para os recursos que cada conexão suporta e a telemetria que envia para Anthropic por padrão, consulte [Disponibilidade de recursos](/docs/pt/feature-availability#availability-by-model-provider) e [Comportamentos padrão por provedor de API](/docs/pt/data-usage#default-behaviors-by-api-provider).

<h3 id="settings-for-unrecognized-model-ids">
  Configurações para IDs de modelo não reconhecidos
</h3>

Duas configurações do lado do cliente alteram o que Claude Code assume para um ID de modelo que não reconhece, independentemente do método de conexão que o desenvolvedor usa:

* **Janela de contexto**: Claude Code assume 200K, ou 1M quando o ID carrega `[1m]`. Para declarar a janela real, consulte [Corrigir a janela para um gateway ou ID de modelo personalizado](/docs/pt/model-config#correct-the-window-for-a-gateway-or-custom-model-id)
* **Capacidades**: para dar a um alias de gateway as capacidades do modelo por trás dele, mapeie o ID Anthropic desse modelo para seu alias com uma entrada [`modelOverrides`](/docs/pt/errors#unrecognized-model-id-on-a-request) nas configurações que você distribui. Para onde as variáveis `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` se aplicam, consulte [feature pass-through](#feature-pass-through)

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

IDs de subagentes são gerados novamente cada vez que Claude Code gera um subagente. Agentes companheiros, os membros nomeados de uma [equipe de agentes](/docs/pt/agent-teams), reutilizam um ID estável baseado em nome entre reconexões. Em ambos os casos, o ID identifica um agente, não uma pessoa ou dispositivo, então não trate o header de ID de agente como um identificador de usuário.

Se seus desenvolvedores definirem `ANTHROPIC_CUSTOM_HEADERS`, esses headers também aparecem em solicitações.

<h3 id="gateway-hint-headers">
  Headers de dica de gateway
</h3>

Claude Code também pode enviar dicas de roteamento: fatos por solicitação que um gateway ou roteador pode usar para agendar, armazenar em cache ou atribuir uma solicitação. Requer Claude Code v2.1.273 ou posterior.

Se uma solicitação as carrega depende de onde Claude Code as envia:

* Conexão direta com a API Anthropic: enviado por padrão
* URL base personalizada: desativado por padrão, porque um proxy que rejeita headers desconhecidos falharia na solicitação. Para recebê-los, defina [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](/docs/pt/env-vars) para seus desenvolvedores, por exemplo no bloco `env` de [configurações gerenciadas](/docs/pt/managed-settings)
* Qualquer outro backend, incluindo Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry e Claude Platform on AWS: enviado apenas quando `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1` está definido

Definir `CLAUDE_CODE_GATEWAY_HINT_HEADERS` para `0` interrompe os headers em cada conexão.

Os headers carregam apenas o que as linhas abaixo listam: vocabulários fixos, nomes de ferramentas e durações, nunca texto de prompt ou conteúdo de arquivo. Cada valor é ASCII imprimível.

| Header                              | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `x-claude-code-request-class`       | Que tipo de solicitação é esta: `main` para uma volta da conversa principal, `subagent` para uma volta de um [subagente](/docs/pt/sub-agents), `workflow` para um agente executando dentro de um workflow, `compaction` para a solicitação de resumo que compacta uma conversa, ou `auxiliary` para solicitações laterais como títulos de sessão, classificadores e resumos. Enviado em cada solicitação                                                                                                                                                                            |
| `x-claude-code-agent-type`          | O tipo de subagente que emitiu a solicitação: um nome de tipo de agente integrado como `Explore`, `Plan` ou `general-purpose`, ou `custom` para um agente definido pelo usuário, `teammate` para um membro da [equipe de agentes](/docs/pt/agent-teams) executando no processo do líder, ou `fork` para um [fork](/docs/pt/sub-agents#fork-the-current-conversation). Presente apenas nas próprias voltas de um subagente; solicitações de compactação ou laterais de um subagente mantêm o ID do agente mas não carregam tipo. Um nome de agente escolhido pelo usuário nunca é enviado |
| `x-claude-code-compaction`          | Presente na solicitação que resume a conversa durante uma [compactação](/docs/pt/prompt-caching#compacting-the-conversation). O valor diz o que a acionou: `auto` quando a janela de contexto se aproximava da capacidade, `manual` para `/compact`, ou `reactive` quando a API rejeitou uma solicitação como muito longa. Ausente em todas as outras solicitações                                                                                                                                                                                                                  |
| `x-claude-code-context-compacted`   | Presente uma vez, na primeira solicitação de conversa principal após uma compactação, com os mesmos valores que `x-claude-code-compaction`. O prefixo de conversa antes desta solicitação não é mais usado, então um cache com chave nele pode ser descartado                                                                                                                                                                                                                                                                                                                  |
| `x-claude-code-prev-tool-durations` | Tempo de execução medido das chamadas de ferramenta cujos resultados esta solicitação carrega, como `<name>=<ms>;<name>=<ms>`, por exemplo `Bash=742;Read=9`. Enviado na próxima solicitação da mesma conversa após um lote de chamadas de ferramenta, da sessão principal ou de um subagente                                                                                                                                                                                                                                                                                  |

Antes de analisar `x-claude-code-prev-tool-durations`, verifique como Claude Code constrói o valor e o que deixa de fora:

* Entradas: uma por chamada de ferramenta que foi executada, na ordem em que seu resultado foi coletado, em milissegundos inteiros
* Limite: Claude Code envia no máximo 32 entradas e 4 KB, mantendo as primeiras entradas
* Codificação: nomes de ferramentas são codificados em percentual, cobrindo `%`, `;`, `=`, vírgula, espaço e qualquer caractere fora do ASCII imprimível
* Análise: dividir em `;`, depois em `=`, e decodificar cada nome
* Ausência: chamadas de compactação, solicitações laterais e a primeira solicitação de um novo prompt nunca a carregam. Não leia um header ausente como uma volta que não executou ferramentas
* Tempos: cada um exclui prompts de permissão e hooks, e chamadas de ferramenta paralelas cada uma relata seu próprio tempo, então as entradas não somam a lacuna entre solicitações

<h3 id="forward-as-open-lists">
  Encaminhar como listas abertas
</h3>

Trate os headers e campos de corpo como listas abertas, não fechadas. Claude Code ganha capacidades ao longo dos lançamentos, e elas chegam como novos valores `anthropic-beta`, novos campos de corpo de solicitação e ocasionalmente novos headers `anthropic-*` ou `x-claude-code-*`.

Ao encaminhar para um upstream no formato Anthropic, passe headers de solicitação `anthropic-*` e campos de corpo de solicitação através inalterados em vez de fazer uma lista de permissões dos que você vê hoje. Um gateway fixado a uma lista observada remove o header ou campo da próxima capacidade e quebra-o no lançamento que a introduz.

A exceção é um upstream não-Anthropic, como Amazon Bedrock ou Agent Platform do Google Cloud, onde fazer a ponte da diferença de schema é trabalho do gateway; consulte [passagem de recursos](#feature-pass-through).

<h2 id="response-headers">
  Cabeçalhos de resposta
</h2>

Claude Code lê esses cabeçalhos de resposta para detectar fluxos travados, para decidir se e quando tentar novamente, e para mostrar limites de uso. A tabela lista o que retornar para cada um. Também encaminhe corpos de resposta de erro sem modificações, para que a [recuperação de rejeição de capacidade](#automatic-retry-and-error-forwarding) do Claude Code possa corresponder à redação do erro do upstream.

| Cabeçalho                       | O que retornar e por quê                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content-type`                  | Retorne `text/event-stream` em respostas de formato Anthropic Messages transmitidas, e `application/vnd.amazon.eventstream`, sem modificações, em respostas de formato Amazon Bedrock, onde [um tipo diferente falha na solicitação](/docs/pt/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy). [Streaming](#streaming) lista quais conexões executam detecção de travamento nesses fluxos |
| `retry-after`                   | Retorne segundos inteiros em vez de uma data HTTP. Claude Code aguarda pelo menos esse tempo antes da próxima [tentativa automática](/docs/pt/errors#automatic-retries), e fora de sessões [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/pt/env-vars) um valor acima de 60 interrompe as tentativas e mostra o erro imediatamente                                                                                  |
| `x-should-retry`                | Passe o valor do upstream inalterado. Claude Code lê este cabeçalho como uma entrada ao decidir se deve tentar novamente uma solicitação com falha: `true` marca a resposta como retentável e `false` marca como não retentável. Para contagens de tentativas, backoff e quais falhas Claude Code tenta novamente, consulte [tentativas automáticas](/docs/pt/errors#automatic-retries)              |
| `anthropic-ratelimit-unified-*` | Encaminhe os valores do upstream inalterados em cada resposta. Claude Code os lê em respostas bem-sucedidas para mostrar o uso em relação aos limites do plano para desenvolvedores conectados com claude.ai, e em um `429` para distinguir um limite de plano ou limite de gastos de um acelerador temporário; consulte [limites de uso](/docs/pt/errors#usage-limits)                              |

<h2 id="system-prompt-attribution-block">
  Bloco de atribuição do prompt do sistema
</h2>

Claude Code prepara um bloco de atribuição curto para o prompt do sistema contendo a versão do cliente e uma impressão digital derivada da conversa. O endpoint `api.anthropic.com` remove o bloco antes do processamento quando ele chega inalterado como o primeiro bloco do sistema, portanto não afeta o cache de prompt de primeira parte. Qualquer outro upstream o recebe como parte do prompt.

A remoção é posicional, portanto funciona apenas quando o gateway encaminha o array `system` inalterado. Para manter o bloco fora do prompt sem perder outro conteúdo do sistema:

* Encaminhe o array `system` exatamente como recebido, mantendo o bloco primeiro: adicionar outro bloco do sistema, reordenar o array ou convertê-lo em uma única string derrota a remoção, e o bloco então chega ao modelo e à chave do cache de prompt.
* Mantenha o bloco em sua própria entrada de array: o endpoint trata um bloco mesclado que começa com o cabeçalho de atribuição como atribuição em sua totalidade e descarta tudo mesclado nele, incluindo o resto do prompt do sistema.
* Se seu gateway deve reformular o conteúdo do sistema, defina [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/pt/env-vars) para que Claude Code omita o bloco. Anthropic e os endpoints Claude dos provedores de nuvem leem o bloco para atribuição, portanto omita-o no cliente em vez de removê-lo ou movê-lo no gateway.

A variável existe para compatibilidade com gateway e cache de terceiros, não como controle de privacidade: em uma conexão direta a solicitação completa já vai para a API Anthropic de qualquer forma. Quando ambas as condições a seguir se mantêm, Claude Code mantém o bloco em solicitações do classificador de [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) mesmo quando você define a variável como `0`:

* As solicitações vão para `api.anthropic.com`, com `ANTHROPIC_BASE_URL` não definido ou nomeando esse host e nenhum provedor de terceiros selecionado.
* A credencial ativa não é uma [credencial de perfil Anthropic ou federação](/docs/pt/authentication#anthropic-profiles-and-federation-credentials).

As solicitações do classificador pulam o resto do prompt do sistema de Claude Code, portanto nessas solicitações o bloco é o único marcador no corpo da solicitação que as identifica como tráfego de Claude Code. Quando qualquer condição falha, através de um gateway LLM, em um provedor de terceiros ou com uma credencial de perfil ou federação ativa, definir `0` remove o bloco das solicitações do classificador também. Antes de v2.1.229, essa exceção não existia: definir `0` removia o bloco dessas solicitações do classificador, e quando a API recusava as solicitações não identificadas, o modo automático falhava em cada ação que enviava ao classificador.

A partir de Claude Code v2.1.181, o bloco é estável pela vida útil de uma conversa quando solicitações são roteadas através de uma URL base personalizada, portanto um cache de prompt do lado do gateway com chave no corpo completo da solicitação funciona sem desabilitá-lo, e qualquer provedor para o qual seu gateway encaminha recebe um prefixo de prompt estável. Antes de v2.1.181, o bloco incluía um token por solicitação que alterava o início do prompt do sistema em cada solicitação. Nessas versões, defina `CLAUDE_CODE_ATTRIBUTION_HEADER=0` quando seu gateway faz qualquer um destes:

* Implementa um cache de prompt com chave no corpo da solicitação.
* Encaminha solicitações para um provedor de terceiros como Amazon Bedrock, Microsoft Foundry ou Agent Platform do Google Cloud, no formato Anthropic Messages ou no próprio formato do provedor, onde o prefixo em mudança reduz a reutilização do cache de prompt nesse provedor.

<h2 id="feature-pass-through">
  Passagem de recursos
</h2>

Claude Code trata um gateway `ANTHROPIC_BASE_URL` como um endpoint no formato Anthropic e envia a ele os headers beta e campos de corpo de solicitação que envia para `api.anthropic.com`, exceto um pequeno conjunto de diagnósticos e padrões reservados para conexões diretas, como o padrão de streaming de ferramenta de granulação fina coberto abaixo. Esse conjunto varia por lançamento, então não dependa de seu conteúdo.

Capacidades que adicionam campos de corpo os emparelham com um header beta, e o par viaja junto. Um gateway que remove o header enquanto passa o corpo, ou encaminha um corpo no formato Anthropic para um upstream com um schema diferente, produz erros `400` difíceis; apenas quando ambas as metades estão ausentes juntas o recurso desativa silenciosamente. Um gateway que reescreve ou redige corpos de solicitação para inspeção de conteúdo quebra o emparelhamento da mesma forma que remover o faz, então inspecione sem modificar. A tabela observa onde um recurso se desvia do emparelhamento.

Streaming de ferramenta de granulação fina é um dos padrões de conexão direta: está desativado por padrão sempre que solicitações são roteadas através de uma URL base personalizada, e um gateway o recebe quando desenvolvedores definem [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/pt/env-vars).

| Recurso                                                                                                                                                                                                                                            | Header e par de corpo                                                                                                                                                                                         | Sintoma quando quebrado                                                                                                                                               | Remediação                                                                                                                                         |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Raciocínio adaptativo](/docs/pt/model-config#adjust-effort-level)                                                                                                                                                                                      | Sem header beta. Claude Code envia `thinking: {"type": "adaptive"}` para Claude 4.6 e posterior, e trata nomes de modelos que não reconhece, como aliases de gateway, como modelos atuais que recebem o campo | `400` nomeando o campo `thinking` ou a tag `adaptive` quando a compilação do modelo upstream não a aceita                                                             | Atualize o upstream. Em Opus 4.6 e Sonnet 4.6, desenvolvedores podem definir `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` em vez disso                |
| [Gerenciamento de contexto](https://platform.claude.com/docs/en/build-with-claude/context-editing)                                                                                                                                                 | Header beta de gerenciamento de contexto emparelhado com o campo de corpo `context_management`                                                                                                                | `400` com `Extra inputs are not permitted`. Comum quando um gateway aceita solicitações no formato Anthropic mas as encaminha para Amazon Bedrock                     | Encaminhe ambos, ou [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/pt/env-vars)                                                                     |
| [Contexto estendido](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) e [pensamento intercalado](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | Apenas headers beta, sem campo de corpo                                                                                                                                                                       | Silenciosamente indisponível quando o header é removido; o upstream nunca vê a solicitação de capacidade                                                              | Encaminhe `anthropic-beta` verbatim                                                                                                                |
| Campos de [ferramenta](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview) beta                                                                                                                                                | Headers beta relacionados a ferramentas emparelhados com campos de schema de ferramenta como `strict` e `defer_loading`                                                                                       | `400` nomeando o campo de schema de ferramenta não reconhecido quando o corpo passa sem seu header                                                                    | Encaminhe ambos, ou [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)                                                |
| [Esforço](https://platform.claude.com/docs/en/build-with-claude/effort) e [saídas estruturadas](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                          | O campo de corpo `output_config` carrega esforço, formato de saída estruturada e configurações de orçamento de tarefa; cada um emparelhado com seu próprio header beta                                        | `400` nomeando `output_config`, frequentemente `Extra inputs are not permitted`, em upstreams Bedrock e Agent Platform                                                | Encaminhe o campo e seus headers juntos                                                                                                            |
| [Prompt caching](/docs/pt/prompt-caching)                                                                                                                                                                                                               | Sem emparelhamento beta. Claude Code anexa marcadores `cache_control` a blocos `system` e a entradas `messages`, incluindo entradas `role: "system"` anexadas no meio da conversa                             | Sem erro: a conversa é cobrada como entrada não armazenada em cache a cada turno, visível como `input_tokens` alto com pouca ou nenhuma atividade de cache em `usage` | Encaminhe `cache_control` inalterado onde quer que apareça, e não converta `system` em forma de bloco ou conteúdo de mensagem para strings simples |
| [Contagem de tokens](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                         | Sem emparelhamento beta; usa o endpoint `count_tokens`                                                                                                                                                        | Sem erro: Claude Code volta a uma estimativa baseada em caracteres, então `/context` mostra contagens aproximadas                                                     | Exponha o endpoint para contagens de tokens exatas                                                                                                 |

As [variáveis](/docs/pt/model-config) `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` declaram capacidades de modelo apenas nas configurações do provedor: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, e [`CLAUDE_CODE_USE_MANTLE`](/docs/pt/amazon-bedrock#use-the-mantle-endpoint). Elas não têm efeito atrás de um gateway `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Retry automático e encaminhamento de erro
</h3>

O que Claude Code faz após uma rejeição upstream depende do que foi rejeitado:

* Quando o upstream rejeita o campo `thinking`, uma mensagem de sistema no meio da conversa, ou o marcador `cache_control` em tal mensagem, Claude Code tenta novamente a solicitação e desabilita a capacidade rejeitada pelo resto da conversa
* Quando o upstream rejeita uma [assinatura de pensamento](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), incluindo com um `400` cuja mensagem diz que o bloco está `bound to a different conversation`, Claude Code remove blocos de pensamento anteriores da solicitação, tenta novamente, e os mantém fora de cada solicitação posterior. Novas respostas ainda incluem pensamento
* Quando o gateway ou seu upstream rejeita a entrada da [ferramenta advisor](/docs/pt/advisor) em `tools` como um tipo de ferramenta não reconhecido, Claude Code tenta novamente a solicitação uma vez sem essa entrada e seu valor `anthropic-beta`. Solicitações posteriores para essa URL base deixam o advisor de fora até Claude Code sair, e `/advisor` fica indisponível para o desenvolvedor por esse tempo. Claude Code reconhece essa rejeição por uma resposta `400` ou `422` cuja mensagem nomeia o tipo de ferramenta após `Input tag`, como `Input tag 'advisor_20260301'`. Antes da v2.1.280, Claude Code não tentava novamente essa rejeição
* Claude Code não tenta novamente rejeições de gerenciamento de contexto ou campos de schema de ferramenta, então esses erros `400` chegam ao desenvolvedor

A rejeição `bound to a different conversation` vem da verificação de [pensamento preservado](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking) da API, que falha quando conteúdo de `system`, `tools`, ou `messages` anteriores difere da solicitação que produziu o pensamento. Um gateway que reescreve qualquer um desses conteúdos pode causar a rejeição em si; [Bibliotecas, proxies e gateways](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways) cobre o que passar inalterado.

A lógica de retry corresponde à redação de erro do upstream, então encaminhe corpos de resposta de erro inalterados. Um gateway que envolve erros upstream em seu próprio envelope quebra o caminho de recuperação, mesmo quando preserva o código de status, a menos que a mensagem do envelope carregue um token `capability_rejected:` estável. [O gateway de aplicativos Claude substitui esses tokens pela redação de erro dos provedores de nuvem](/docs/pt/claude-apps-gateway-config#upstream-error-messages), por exemplo `capability_rejected: prompt_too_long`.

<h3 id="disable-pre-release-capabilities">
  Desabilitar capacidades de pré-lançamento
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` impede que Claude Code envie capacidades de pré-lançamento e seus campos de corpo em cada provedor, incluindo gerenciamento de contexto e campos de ferramenta beta. A variável não afeta raciocínio adaptativo, que é selecionado por modelo em vez de por beta. Nunca suprime a capacidade OAuth que autenticação de assinatura requer.

Em Claude Code v2.1.227 ou posterior, sua organização pode manter [busca de ferramenta MCP](/docs/pt/mcp#scale-with-mcp-tool-search) ativada sob essa variável através de [configurações gerenciadas](/docs/pt/managed-settings). O que Claude Code envia com essa substituição em vigor depende de como você se conecta:

* Em uma conexão direta, ou através de um gateway definido com `ANTHROPIC_BASE_URL`, Claude Code continua enviando o header beta de busca de ferramenta, campos de ferramenta `defer_loading`, e blocos `tool_reference`, e remove o resto
* Em um provedor de nuvem, ou conectado através de um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), a substituição não tem efeito

O conjunto de capacidades que Claude Code envia cresce ao longo dos lançamentos. Para strings de header beta atuais, consulte a [referência de headers beta](https://platform.claude.com/docs/en/api/beta-headers); teste seu gateway contra novos lançamentos de Claude Code em vez de fixar a uma lista observada.

<h2 id="model-discovery">
  Descoberta de modelos
</h2>

Quando `ANTHROPIC_BASE_URL` aponta para um gateway que expõe o formato Anthropic Messages, Claude Code pode consultar o endpoint `/v1/models` do gateway na inicialização e adicionar os modelos retornados ao seletor `/model`. Se você ou seu administrador definir `replaceBuiltInOptions` em um [`modelPicker`](/docs/pt/settings-reference#modelpicker) lineup, Claude Code oculta os modelos descobertos do seletor.

Desenvolvedores o habilitam definindo [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/pt/env-vars), em seu próprio ambiente ou através de configurações gerenciadas. A descoberta está desativada por padrão para que gateways apoiados por uma chave de API compartilhada não exponham cada modelo que a chave pode acessar a cada usuário.

<h3 id="when-discovery-runs">
  Quando a descoberta é executada
</h3>

A descoberta se aplica apenas ao formato Anthropic Messages. Não é executada quando:

* Qualquer variável de provedor `CLAUDE_CODE_USE_*` é definida, mesmo se `ANTHROPIC_BASE_URL` também for definido
* `ANTHROPIC_BASE_URL` não está definido ou aponta para `api.anthropic.com`

A descoberta ainda é executada quando [o tráfego não essencial é desativado](/docs/pt/llm-gateway-connect#turn-off-traffic-outside-the-gateway-path), porque a solicitação vai apenas para seu gateway. Antes da v2.1.257, a descoberta não era executada enquanto o tráfego não essencial estava desativado.

<h3 id="request-and-response">
  Solicitação e resposta
</h3>

A solicitação é `GET /v1/models?limit=1000` com um timeout de 3 segundos por padrão, e qualquer redirecionamento é tratado como falha para que a credencial não vaze para um alvo de redirecionamento. Um gateway que responde mais lentamente do que o timeout, ou um que redireciona `/v1/models`, mesmo `http` para `https`, falha na descoberta silenciosamente; sirva o endpoint diretamente na URL base configurada.

Para dar a um gateway lento mais tempo, defina [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](/docs/pt/env-vars#variables). A variável requer Claude Code v2.1.269 ou posterior.

Claude Code envia a solicitação de descoberta com ambos os headers de credencial abaixo e omite um header cujo valor não se resolve. Enviar ambos os headers requer Claude Code v2.1.248 ou posterior. Versões anteriores enviam apenas `Authorization` quando `ANTHROPIC_AUTH_TOKEN` é definido e apenas `x-api-key` caso contrário.

* `Authorization`: `ANTHROPIC_AUTH_TOKEN` como um token bearer, caso contrário o valor [`apiKeyHelper`](/docs/pt/llm-gateway-connect#rotate-credentials-with-apikeyhelper) como um token bearer. Nesse caso, Claude Code aguarda o helper retornar antes de enviar a solicitação.
* `x-api-key`: a chave de API que Claude Code resolveu, como `ANTHROPIC_API_KEY`. Quando um valor helper é a única credencial, este header também o carrega, para que o valor chegue em ambos os headers.

Claude Code também envia qualquer header de `ANTHROPIC_CUSTOM_HEADERS`. Quando um header customizado tem um valor não vazio, Claude Code o envia no lugar de um header integrado de mesmo nome, correspondendo nomes case-insensitively.

Quando nenhum valor do header de credencial se resolve, Claude Code pula a descoberta e escreve uma linha `[gatewayDiscovery] skipped` no log de debug de uma sessão `claude --debug`. Se você fornecer uma credencial apenas através de `ANTHROPIC_CUSTOM_HEADERS`, Claude Code ainda pula a descoberta.

Claude Code lê `id`, o `display_name` opcional e a `description` opcional de cada entrada no array `data` da resposta:

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

Claude Code mantém uma entrada quando seu `id` contém `claude` ou `anthropic` em qualquer lugar na string, correspondido case-insensitively, e ignora o resto. IDs com prefixo de provedor como `vertex_ai/claude-sonnet-4-6` ou `bedrock/anthropic.claude-sonnet-4-5` passam no filtro; um ID que não contém nenhuma substring não passa. Antes da v2.1.223, Claude Code mantinha uma entrada apenas quando seu `id` começava com `claude` ou `anthropic`, o que ocultava IDs com prefixo de provedor.

<h3 id="picker-entries-and-caching">
  Entradas do seletor e cache
</h3>

O seletor é a lista de modelos interativa que abre quando um desenvolvedor executa `/model` em Claude Code. Cada entrada descoberta usa `display_name` como seu nome quando o gateway envia um que difere do `id`. Caso contrário, a entrada mostra o nome do modelo quando Claude Code [reconhece o `id`](/docs/pt/model-config#customize-pinned-model-display-and-capabilities), e o `id` quando não reconhece. Por exemplo, uma entrada com o `id` `my-gateway-claude-sonnet-4-6` e sem `display_name` aparece como `Sonnet 4.6`.

A descoberta adiciona apenas modelos que a [configuração gerenciada `availableModels`](/docs/pt/settings-reference#availablemodels) permite.

Cada entrada também mostra a `description` do modelo, recolhida em uma linha. Uma entrada sem `description` lê "Do gateway" em vez disso. Antes da v2.1.257, cada entrada descoberta lia "Do gateway".

Um ID descoberto não recebe sua própria linha quando corresponde a uma linha já no seletor:

* Mesmo ID: o ID descoberto corresponde exatamente ao ID de uma linha existente, ou os dois IDs são grafias da mesma versão [Fable](/docs/pt/model-config#work-with-fable).
* Mesmo modelo que um alias integrado: quando um ID explícito descoberto nomeia o modelo para o qual um alias integrado atualmente se resolve, o seletor mostra apenas a linha do alias. Por exemplo, enquanto `sonnet` se resolve para `claude-sonnet-5`, um `claude-sonnet-5` descoberto colapsa na linha `sonnet`, e um `claude-sonnet-4-6` descoberto ainda recebe sua própria linha. Antes da v2.1.197, Claude Code não dobrava esses IDs em linhas integradas, então `claude-sonnet-5` também recebia sua própria linha "Do gateway".

Os resultados são armazenados em cache em `~/.claude/cache/gateway-models.json`, ou `%USERPROFILE%\.claude\cache\gateway-models.json` no Windows, e atualizados em cada inicialização. Se você definir [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars), o cache fica sob esse diretório em vez disso. Se a solicitação falhar ou o gateway não implementar `/v1/models`, o seletor volta para a lista em cache da inicialização anterior ou para a lista de modelos integrada. Se seu gateway serve modelos Claude sob aliases que não correspondem ao filtro de descoberta, desenvolvedores podem adicionar esses aliases manualmente com as [variáveis de configuração de modelo](/docs/pt/model-config).

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para o resto do conjunto de documentação do gateway e as referências de API subjacentes:

* [Visão geral de gateway](/docs/pt/gateways): o que é um gateway e como escolher entre o gateway de aplicativos Claude e outro produto
* [Outros gateways LLM](/docs/pt/llm-gateway): como implantar um gateway que sua organização executa e como ele interage com assinaturas claude.ai
* [Implantar um gateway LLM para sua organização](/docs/pt/llm-gateway-rollout): a lista de verificação do administrador que usa este guia
* [Conectar Claude Code a um gateway LLM](/docs/pt/llm-gateway-connect): configuração por desenvolvedor e a tabela de solução de problemas
* [Referência de headers beta](https://platform.claude.com/docs/en/api/beta-headers): o conjunto atual de valores `anthropic-beta`
* [Messages API](https://platform.claude.com/docs/en/api/messages): o formato de API que um gateway no formato Anthropic implementa
