> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implante um gateway LLM para sua organização

> Implante um produto de gateway para Claude Code: configure-o para encaminhar o que Claude Code envia, emita credenciais de desenvolvedor, distribua a configuração através de configurações gerenciadas e verifique a implantação.

Esta página orienta um administrador através da implantação de um gateway LLM para Claude Code. Ela assume que você tem um produto de gateway implantado que atende aos [requisitos do gateway](#gateway-requirements). A implantação ou operação de qualquer produto específico não é abordada aqui; implante o seu seguindo a documentação do seu fornecedor.

<Note>
  * Para conectar Claude Code em sua própria máquina a um gateway existente, consulte [Conectar Claude Code a um gateway LLM](/docs/pt/llm-gateway-connect)
  * Para saber o que Claude Code envia a um gateway e o que encaminhar, consulte a [referência do protocolo do gateway](/docs/pt/llm-gateway-protocol)
</Note>

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Para concluir a implantação, você precisará de:

* Um gateway implantado em sua infraestrutura, servindo HTTPS no endereço exato que você distribuirá aos desenvolvedores, não em um endereço que redireciona para ele, e configurado para rotear nomes de modelos Claude para seu provedor
* Uma credencial de provedor para o gateway encaminhar com:
  * Para a API Anthropic: uma chave de API do [Claude Console](https://platform.claude.com/settings/keys)
  * Para um provedor de nuvem: credenciais de nuvem com acesso ao modelo. Consulte os pré-requisitos na página [Amazon Bedrock](/docs/pt/amazon-bedrock#prerequisites), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai#prerequisites) ou [Microsoft Foundry](/docs/pt/microsoft-foundry#prerequisites)
* Uma maneira de entregar arquivos de configurações para máquinas de desenvolvedores, como MDM ou gerenciamento de configuração
  * Se você ainda não tiver uma, [como as configurações chegam aos dispositivos](/docs/pt/admin-setup#decide-how-settings-reach-devices) compara as opções

<h3 id="gateway-requirements">
  Requisitos do gateway
</h3>

Qualquer que seja o produto que fornece o gateway, ele deve:

* **Aceitar um formato de API suportado**: um dos formatos na [tabela de formatos de API](/docs/pt/llm-gateway-protocol#api-formats). As etapas de implantação abaixo assumem a API de Mensagens Anthropic em `POST /v1/messages`, que a maioria dos gateways serve
* **Transmitir respostas**: passar eventos enviados pelo servidor conforme chegam em vez de armazenar em buffer a resposta inteira
* **Rotear nomes de modelos Claude**: mapear cada nome que os desenvolvedores usam para um modelo upstream. Claude Code envia um nome de modelo como `claude-sonnet-4-6` em cada solicitação; na maioria dos produtos de gateway o mapeamento é uma lista de modelos ou tabela de roteamento na própria configuração do gateway
* **Encaminhar cabeçalhos e corpo sem alterações**: passar `anthropic-beta`, `anthropic-version` e o corpo da solicitação em ambas as direções; a [tabela de passagem de recursos](/docs/pt/llm-gateway-protocol#feature-pass-through) mapeia cada um para o recurso que quebra sem ele
* **Retornar erros upstream não modificados**: a recuperação automática do Claude Code corresponde à redação do erro, portanto envolver erros no próprio envelope do gateway quebra isso
* **Isentar o caminho da inspeção WAF do corpo da solicitação**: os prompts do Claude Code carregam código-fonte e tags de estilo XML que correspondem às regras do corpo de cross-site-scripting; um WAF na frente do gateway retorna `403` em sessões reais enquanto solicitações de teste curtas passam

Opcionalmente, sirva `GET /v1/models` para que Claude Code possa preencher o seletor de modelo do seu gateway com [descoberta de modelo](/docs/pt/llm-gateway-protocol#model-discovery).&#x20;

<h2 id="rollout-steps">
  Etapas de implantação
</h2>

A implantação leva cinco etapas, cada uma com um ponto de verificação:

1. [Confirme que o gateway roteia seus modelos](#confirm-the-gateway-routes-your-models)
2. [Emita uma credencial para cada desenvolvedor](#issue-developer-credentials)
3. [Teste Claude Code contra o gateway](#test-claude-code-against-the-gateway)
4. [Distribua a URL base e as credenciais](#distribute-the-configuration)
5. [Verifique a partir de uma máquina de desenvolvedor](#verify-the-rollout)

As etapas envolvem três credenciais diferentes, e os pontos de verificação as nomeiam por espaço reservado para que você possa dizer qual é a culpada quando algo falha:

| Credencial                           | Quem a detém                                                                                                   | Espaço reservado nos pontos de verificação                   |
| :----------------------------------- | :------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------- |
| Credencial do provedor               | O gateway, que a encaminha para o provedor upstream                                                            | Configurado no gateway; nunca aparece em comandos do cliente |
| Credencial administrativa do gateway | Você, se seu produto de gateway emitir uma para sua interface de administrador ou teste                        | `<gateway-key>`                                              |
| Chave do desenvolvedor               | Cada desenvolvedor, emitido pelo gateway em [Emita credenciais de desenvolvedor](#issue-developer-credentials) | `<developer-key>`                                            |

<h3 id="confirm-the-gateway-routes-your-models">
  Confirme que o gateway roteia seus modelos
</h3>

Seu gateway já deve estar configurado com sua credencial de provedor, ouvindo em sua URL base e encaminhando solicitações para a API do seu provedor. Teste que o caminho funciona de ponta a ponta com uma solicitação mínima, substituindo dois valores de sua implantação:

* `<gateway-key>` é qualquer credencial que permite chamar o gateway agora: uma chave administrativa, uma chave de teste ou sua própria chave de desenvolvedor se você já tiver emitido uma. Nem todo produto de gateway tem uma credencial de administrador separada; se o seu não tiver, emita uma chave de desenvolvedor para você em [Emita credenciais de desenvolvedor](#issue-developer-credentials) primeiro
* `model` é um nome de modelo Claude que seu gateway está configurado para rotear. O exemplo usa `claude-sonnet-4-6`; substitua um nome que você configurou

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <gateway-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <gateway-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Ponto de verificação**: um `200` com um campo `content` significa que o gateway alcançou o provedor com esse nome de modelo. Um `404` significa que esse nome não é roteado no gateway; um `401` do provedor significa que a credencial do provedor do gateway está errada.

Repita a solicitação uma vez por nome de modelo Claude na configuração de roteamento do seu gateway. Um nome que o gateway não roteia retorna `404` para qualquer desenvolvedor que o selecione, portanto teste cada nome antes da implantação.

<Note>
  Evite servir o gateway atrás de um redirecionamento. Um redirecionamento pode descartar o corpo da solicitação ou remover o cabeçalho de credencial em solicitações de inferência, e [descoberta de modelo](/docs/pt/llm-gateway-protocol#model-discovery) trata qualquer redirecionamento como uma falha para que a credencial não possa vazar para um alvo de redirecionamento.
</Note>

<h3 id="issue-developer-credentials">
  Emita credenciais de desenvolvedor
</h3>

Cada desenvolvedor precisa de sua própria chave de gateway para autenticar. Crie uma credencial por desenvolvedor no gateway, seguindo a documentação de gerenciamento de credenciais do seu produto.

Confirme que uma chave recém-emitida funciona contra o gateway com a mesma solicitação que [Confirme que o gateway roteia seus modelos](#confirm-the-gateway-routes-your-models), substituindo `<gateway-key>` pela nova `<developer-key>`:

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <developer-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Ponto de verificação**: um `200` com um campo `content` significa que a chave do desenvolvedor alcança o gateway e o gateway a encaminha. Um `401` aqui, quando [a etapa anterior](#confirm-the-gateway-routes-your-models) foi bem-sucedida, significa que a chave do desenvolvedor está errada ou ainda não entrou em vigor no gateway.

Emitir uma chave por desenvolvedor em vez de uma chave compartilhada é o que torna a atribuição de uso por desenvolvedor e o offboarding individual funcionarem. A variável de ambiente que contém a chave depende de qual cabeçalho o gateway lê. Para um gateway que verifica credenciais no cabeçalho `Authorization: Bearer`, os desenvolvedores definem sua chave em `ANTHROPIC_AUTH_TOKEN`. Para um gateway que lê chaves do cabeçalho `x-api-key`, os desenvolvedores definem `ANTHROPIC_API_KEY` em vez disso; a [tabela de credenciais](/docs/pt/llm-gateway-connect#set-the-credential-variable) cobre o mapeamento.

<h3 id="test-claude-code-against-the-gateway">
  Teste Claude Code contra o gateway
</h3>

Execute Claude Code através do gateway você mesmo antes de distribuir qualquer coisa, usando a mesma configuração que a implantação entregará em toda a frota. Digite-os diretamente em um terminal, não em um arquivo `.env` ou arquivo de configurações; eles duram apenas para esta sessão de terminal, portanto fechá-la retorna sua máquina à sua configuração normal. Use `ANTHROPIC_API_KEY` em vez de `ANTHROPIC_AUTH_TOKEN` se seu gateway lê o cabeçalho `x-api-key`:

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN="<developer-key>"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "<developer-key>"
    ```
  </Tab>
</Tabs>

Em seguida, envie um prompt único através do gateway:

```bash theme={null}
claude -p "Reply with one word: connected"
```

**Ponto de verificação**: o prompt retorna uma resposta e a solicitação aparece no log do gateway como um `POST` para o caminho `/v1/messages` com status `200`. Claude Code anexa uma string de consulta como `?beta=true`, portanto corresponda no caminho, não na URL completa. Duas mensagens de falha apontam em direções diferentes:

* `Not logged in`: verifique o log do gateway para distinguir as duas causas. Se estiver vazio, nenhuma credencial alcançou a sessão e nenhuma solicitação saiu da máquina; re-execute as exportações no shell que você está testando. Se mostrar uma solicitação rejeitada com `x-api-key` no corpo `401`, o gateway espera chaves nesse cabeçalho em vez disso; mude para `ANTHROPIC_API_KEY`
* `Failed to authenticate. API Error: 401` significa que uma credencial foi enviada e rejeitada, e o log do gateway diz onde: um `401` nomeando `api.anthropic.com` ou o endpoint do seu provedor significa que o gateway alcançou o upstream mas sua credencial de provedor foi rejeitada, portanto a chave do desenvolvedor funcionou e a credencial do provedor que o gateway detém está errada ou é um espaço reservado

Uma URL base errada ou inacessível produz um sintoma diferente: Claude Code [tenta novamente a conexão com backoff](/docs/pt/errors#automatic-retries) e pode ficar sem saída por vários minutos antes de relatar um erro. Se o comando parecer travar, verifique o log do gateway em vez de esperar; nenhuma solicitação chegando significa que `ANTHROPIC_BASE_URL` não aponta para o gateway.

<h3 id="distribute-the-configuration">
  Distribua a configuração
</h3>

Cada máquina de desenvolvedor precisa do endereço do gateway e de uma credencial. Você pode distribuí-los centralmente através de [configurações gerenciadas](/docs/pt/settings#settings-files), para que os desenvolvedores não configurem nada, ou entregue aos desenvolvedores os valores para definir eles mesmos.

<h4 id="what-to-distribute">
  O que distribuir
</h4>

O mesmo conjunto de variáveis se aplica qualquer que seja o caminho que você escolha. A maioria das implantações só precisa de `ANTHROPIC_BASE_URL` e uma credencial; inclua as linhas condicionais quando sua configuração de gateway as exigir.

| Variável ou configuração                                                                                                                                                                                                       | O que faz                                                                                                                                                                                            | Incluir quando                                                                                                                                                                                                                                                                                                                                                                |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_BASE_URL`                                                                                                                                                                                                           | Envia as solicitações de API do Claude Code para o gateway em vez de `api.anthropic.com`                                                                                                             | Sempre                                                                                                                                                                                                                                                                                                                                                                        |
| `apiKeyHelper`, ou uma credencial em `ANTHROPIC_AUTH_TOKEN` ou `ANTHROPIC_API_KEY`                                                                                                                                             | Autentica cada solicitação ao gateway. O auxiliar executa um comando para buscar a chave; as variáveis mantêm uma chave estática, enviada como `Authorization: Bearer` e `x-api-key` respectivamente | Sempre; uma das três                                                                                                                                                                                                                                                                                                                                                          |
| `ANTHROPIC_CUSTOM_HEADERS`                                                                                                                                                                                                     | Adiciona cabeçalhos HTTP extras a cada solicitação de API                                                                                                                                            | Seu gateway requer um cabeçalho de locatário ou roteamento em cada solicitação                                                                                                                                                                                                                                                                                                |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY`                                                                                                                                                                                   | Consulta `/v1/models` do gateway na inicialização e adiciona os nomes retornados ao seletor `/model`                                                                                                 | Seu gateway serve `/v1/models` e você quer que os seletores dos desenvolvedores sejam preenchidos a partir dele                                                                                                                                                                                                                                                               |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`                                                                                                                                                                                       | Para Claude Code de enviar cabeçalhos de capacidade pré-lançamento e campos de corpo                                                                                                                 | Seu gateway encaminha para um upstream Bedrock ou Vertex que rejeita campos beta; consulte [Requisitos do gateway](#gateway-requirements)                                                                                                                                                                                                                                     |
| `ANTHROPIC_MODEL` ou [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/pt/model-config)                                                                                                                                                       | Define qual nome de modelo Claude Code solicita para a sessão principal e para tráfego de fundo                                                                                                      | Seu gateway roteia nomes de modelos que não correspondem aos padrões do Claude Code, ou você roteia [funcionalidade de fundo](/docs/pt/costs#background-token-usage) para um modelo diferente. Rotear tanto os nomes de substituição quanto os nomes padrão do Claude Code no gateway, já que algumas sub-chamadas podem solicitar o nome padrão independentemente da substituição |
| `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, `ANTHROPIC_FOUNDRY_BASE_URL` ou `ANTHROPIC_AWS_BASE_URL` com as [variáveis para esse provedor](/docs/pt/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) | Aponte Claude Code para o gateway através de uma URL base específica do provedor. Bedrock e Vertex também mudam para o formato de solicitação nativo desses provedores                               | Seu gateway está na frente de Bedrock, Vertex, Foundry ou da Plataforma Claude no AWS; consulte [Formatos de API](/docs/pt/llm-gateway-protocol#api-formats)                                                                                                                                                                                                                       |

<h4 id="distribute-through-managed-settings">
  Distribua através de configurações gerenciadas
</h4>

Entregue as variáveis através do bloco `env` de um [arquivo de configurações gerenciadas](/docs/pt/settings#settings-files), enviado por MDM, política de registro ou gerenciamento de configuração:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

Adicione as variáveis condicionais da tabela ao mesmo bloco `env`. Um `ANTHROPIC_BASE_URL` gerenciado é imposto e não pode ser substituído pela exportação de shell de um desenvolvedor, já que Claude Code o aplica sobre o ambiente do processo e configurações de precedência inferior.

Não inclua `forceLoginMethod` ou `forceLoginOrgUUID` em configurações gerenciadas junto com uma credencial de gateway. No Claude Code v2.1.146 e posterior, qualquer chave bloqueia `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` e `apiKeyHelper` na inicialização, portanto os desenvolvedores veem `This machine's managed settings require a first-party login` e não podem prosseguir.&#x20;

A entrega de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings#platform-availability) requer uma conexão direta com `api.anthropic.com`, portanto não alcança sessões roteadas por gateway. As implantações de gateway usam este caminho de configurações gerenciadas baseado em arquivo, que impõe as mesmas chaves.

Para a credencial, distribua um comando [`apiKeyHelper`](/docs/pt/llm-gateway-connect#rotate-credentials-with-apikeyhelper) no arquivo de configurações gerenciadas conforme mostrado acima; o comando autentica seu armazenamento de segredos como o desenvolvedor local, portanto cada máquina recebe sua própria chave. Alternativamente, entregue a cada desenvolvedor sua chave através do seu processo de segredos existente e peça-lhes para definir `ANTHROPIC_AUTH_TOKEN` eles mesmos.

Alguns ambientes precisam de entrega separada:

* O aplicativo de desktop lê o roteamento do gateway apenas de sua configuração de inferência de terceiros entregue por MDM; implante esse arquivo junto com configurações gerenciadas para que as sessões de desktop também roteiem através do gateway. Consulte a [documentação de configuração de terceiros do desktop](https://claude.com/docs/third-party/claude-desktop/configuration) e a [documentação de gateway do desktop](https://claude.com/docs/third-party/claude-desktop/gateway)
* Os executores de CI precisam de `ANTHROPIC_BASE_URL` e a credencial definida no [ambiente do executor](/docs/pt/llm-gateway-connect#configure-each-surface)
* WSL em máquinas Windows gerenciadas lê as configurações gerenciadas do Windows apenas quando [`wslInheritsWindowsSettings`](/docs/pt/settings#available-settings) é `true`

<h4 id="hand-developers-the-values-to-set-themselves">
  Entregue aos desenvolvedores os valores para definir eles mesmos
</h4>

Se você não tiver distribuição de configurações gerenciadas em vigor, envie a cada desenvolvedor o que ele precisa para seguir a [página de conexão](/docs/pt/llm-gateway-connect#configure-claude-code-yourself):

* A URL do gateway
* Sua credencial pessoal
* **Qual variável colocar a credencial em**: `ANTHROPIC_AUTH_TOKEN` para um gateway de token portador, ou `ANTHROPIC_API_KEY` para um gateway `x-api-key`. Dizer aos desenvolvedores qual economiza o trial-and-error descrito na [página de conexão](/docs/pt/llm-gateway-connect#set-the-credential-variable)
* Quaisquer variáveis condicionais da [tabela O que distribuir](#what-to-distribute), com seus valores

A [página de conexão](/docs/pt/llm-gateway-connect#configure-claude-code-yourself) orienta os desenvolvedores através da definição de cada uma.

**Ponto de verificação**: em uma máquina de desenvolvedor, `claude` inicia uma sessão sem mostrar a tela de login, já que a credencial distribuída satisfaz a autenticação. Em seguida, execute `/status` e abra a aba **Status**: a linha `Anthropic base URL` mostra o endereço do gateway, e para distribuição gerenciada a linha `Setting sources` inclui configurações gerenciadas. Uma tela de login, ou uma linha `Anthropic base URL` ausente, significa que a configuração não alcançou a máquina.

<h3 id="verify-the-rollout">
  Verifique a implantação
</h3>

Confirme que tudo funciona a partir de uma máquina de desenvolvedor, não do host do gateway, para que o teste cubra o caminho de rede que os desenvolvedores usam. Envie uma solicitação de streaming, que verifica o endpoint, passagem de streaming e roteamento de modelo de uma vez:

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    curl -N -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $body = '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    $body | curl.exe -N -X POST "https://llm-gateway.example.com/v1/messages" `
      -H "Authorization: Bearer <developer-key>" `
      -H "anthropic-version: 2023-06-01" `
      -H "content-type: application/json" `
      --data-binary '@-'
    ```
  </Tab>
</Tabs>

Você deve ver linhas `data:` chegarem incrementalmente. A resposta inteira chegando de uma vez após uma pausa significa que o gateway está armazenando em buffer, o que paralisa Claude Code; um `404` significa que o nome do modelo não é roteado. Repita por nome de modelo.

Em seguida, inicie `claude` e envie uma mensagem. Cada sintoma nesta etapa tem uma causa:

* Um prompt de login significa uma lacuna de credencial. Execute `/status` e abra a aba **Status**: quando a linha `Setting sources` não inclui configurações gerenciadas, a distribuição não alcançou a máquina; quando inclui, a credencial do desenvolvedor não foi entregue, portanto defina `ANTHROPIC_AUTH_TOKEN` ou o `apiKeyHelper`
* Erros `Failed to authenticate` significam que o gateway está rejeitando solicitações; seu log diz qual credencial falhou. Uma rejeição que o gateway registra em si nomeia a chave do desenvolvedor, enquanto um `401` de `api.anthropic.com` ou do endpoint do seu provedor significa que a credencial do provedor que o gateway detém foi rejeitada
* Um prompt de aprovação única para a chave é esperado no primeiro uso quando o gateway espera chaves no cabeçalho `x-api-key`, definido como `ANTHROPIC_API_KEY`. Com `ANTHROPIC_AUTH_TOKEN`, nenhum prompt aparece e a variável assume silenciosamente; um login claude.ai previamente salvo está inativo para essa sessão

Finalmente, verifique os logs do gateway para a mensagem que você enviou: a credencial identifica o desenvolvedor, e o [cabeçalho `x-claude-code-session-id`](/docs/pt/llm-gateway-protocol#request-headers) agrupa solicitações por sessão. Se os recursos falharem com os [sintomas de solução de problemas](/docs/pt/llm-gateway-connect#troubleshoot-gateway-errors), o gateway está removendo cabeçalhos ou reescrevendo erros; consulte os [requisitos do gateway](#gateway-requirements) acima.

<h2 id="maintain-the-gateway">
  Mantenha o gateway
</h2>

Após a implantação, três tipos de mudança alcançam o gateway ao longo do tempo. Cada um tem um sintoma a observar e uma ação a tomar.

| Mudança                                                                                              | Sintoma quando o gateway não acompanhou                                                                                                                                        | Ação                                                                                                                                                                                                                                                                           |
| :--------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Novos lançamentos do Claude Code adicionam valores `anthropic-beta` e campos de corpo de solicitação | Os desenvolvedores relatam erros `400` nomeando um novo campo depois que atualizam Claude Code; consulte [passagem de recursos](/docs/pt/llm-gateway-protocol#feature-pass-through) | Encaminhe cabeçalhos `anthropic-*` e corpos de solicitação verbatim em vez de usar lista de permissões; teste novos lançamentos do Claude Code contra o gateway antes de alcançarem os desenvolvedores                                                                         |
| Novos modelos Claude ficam disponíveis                                                               | Os desenvolvedores selecionando um novo nome de modelo obtêm `404`; o seletor `/model` não o lista                                                                             | Adicione o nome do modelo à configuração de roteamento do gateway, em seguida, re-execute a [verificação de roteamento](#confirm-the-gateway-routes-your-models). Se você distribuir `ANTHROPIC_MODEL` ou as variáveis de modelo padrão, atualize as configurações gerenciadas |
| Credenciais expiram ou precisam de rotação                                                           | Todas as solicitações de desenvolvedor começam a falhar com `401` do upstream                                                                                                  | Rotacione a credencial do provedor do gateway em seu próprio cronograma; as chaves do desenvolvedor giram no gateway, e um [`apiKeyHelper`](/docs/pt/llm-gateway-connect#rotate-credentials-with-apikeyhelper) lida com rotação por desenvolvedor sem redistribuir configurações    |

Ao dimensionar limites de taxa por chave, leve em conta o cliente [tentando novamente falhas transitórias](/docs/pt/errors#automatic-retries), incluindo respostas `429`, até 10 vezes com backoff, honrando `Retry-After`. Mantenha a [referência do protocolo](/docs/pt/llm-gateway-protocol) como o contrato para o que cada lançamento do Claude Code envia.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Conectar Claude Code a um gateway LLM](/docs/pt/llm-gateway-connect): as etapas de configuração voltadas para o desenvolvedor, com configuração por superfície e uma tabela de solução de problemas que você pode entregar aos desenvolvedores
* [Referência do protocolo do gateway](/docs/pt/llm-gateway-protocol): o contrato de fio para operadores de gateway, cobrindo endpoints, cabeçalhos para encaminhar e a tabela de passagem de recursos
* [Arquivos de configurações e precedência](/docs/pt/settings#settings-files): como configurações gerenciadas, de projeto e de usuário se combinam, e onde o arquivo gerenciado vai em cada plataforma
* [Configure Claude Code para sua organização](/docs/pt/admin-setup): a implantação mais ampla da qual este gateway é uma parte, incluindo imposição de política, visibilidade de uso e tratamento de dados
