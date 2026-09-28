> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Conectar Claude Code a um gateway LLM

> Aponte Claude Code para o gateway LLM da sua organização. Verifique se seu administrador já o configurou ou defina a URL base e a credencial você mesmo, depois verifique a conexão e corrija erros do gateway.

Um [gateway LLM](/docs/pt/llm-gateway) é um proxy que sua organização executa entre Claude Code e o provedor de modelo. Quando sua organização usa um, Claude Code se autentica no gateway com uma credencial que sua organização emite em vez de seu login pessoal claude.ai.

Esta página é para desenvolvedores que executam Claude Code através de um gateway que sua organização opera. Ela cobre dois caminhos: [verificar se seu administrador já o configurou para você](#check-for-an-existing-configuration) e [configurá-lo você mesmo](#configure-claude-code-yourself) quando não tiverem.

<Note>
  * Para implantar um gateway para sua organização, consulte [Implementar um gateway LLM](/docs/pt/llm-gateway-rollout)
  * Para saber o que Claude Code envia para um gateway, consulte a [referência do protocolo do gateway](/docs/pt/llm-gateway-protocol)
</Note>

<h2 id="check-for-an-existing-configuration">
  Verificar uma configuração existente
</h2>

Os administradores podem distribuir o endereço do gateway e a credencial através de [configurações gerenciadas](/docs/pt/managed-settings), gerenciamento de dispositivos ou um [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), para que Claude Code os pegue na inicialização sem nada para você configurar. Para verificar se sua organização já fez isso:

<Steps>
  <Step title="Iniciar Claude Code">
    Execute `claude`. Se ele abrir para a tela de login em vez de uma sessão, nenhuma credencial de gateway foi distribuída; [configure-a você mesmo](#configure-claude-code-yourself) abaixo.
  </Step>

  <Step title="Verificar a aba Status">
    Se Claude Code iniciou uma sessão sem mostrar a tela de login, execute `/status`, que abre na aba **Status**, e verifique duas linhas:

    * `Anthropic base URL`: esta linha aparece apenas quando um endereço de gateway está definido. Se não estiver lá, Claude Code não está apontado para o gateway; [configure-o você mesmo](#configure-claude-code-yourself) abaixo.
    * `Auth token` ou `API key`: uma linha nomeando `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY` ou um `apiKeyHelper` confirma que uma credencial de gateway está ativa. Uma linha `Login method` nomeando uma conta claude.ai em vez disso significa que a credencial não foi distribuída; [defina-a você mesmo](#set-the-credential-variable).
  </Step>

  <Step title="Enviar uma mensagem de teste">
    Feche o menu `/status` e envie qualquer prompt em Claude Code. Uma resposta normal de Claude, sem erro, confirma que a conexão do gateway funciona.
  </Step>
</Steps>

Se ambas as linhas no menu `/status` parecerem corretas, mas a mensagem para Claude falhar, consulte a [tabela de solução de problemas](#troubleshoot-gateway-errors).

<h2 id="configure-claude-code-yourself">
  Configurar Claude Code você mesmo
</h2>

Para configurar Claude Code para o gateway você mesmo, você precisa de sua equipe de gateway:

* A URL base do gateway
* Uma credencial: uma string de chave ou token, ou um comando que busca uma
  * Se sua equipe de gateway não disse qual tipo de credencial é, a seção [variável de credencial](#set-the-credential-variable) abaixo cobre o que tentar

As seções abaixo cobrem a configuração em ordem:

* [Definir a variável de credencial](#set-the-credential-variable) e [definir a URL base](#set-the-base-url-and-credential): as duas variáveis que toda conexão de gateway precisa
* [Verificar a conexão](#verify-the-connection): confirme que funciona antes de persistir qualquer coisa
* [Configurar cada superfície](#configure-each-surface): se você está usando uma superfície além do CLI Claude Code, como VS Code, veja como configurá-la com suas credenciais de gateway
* [Configuração adicional](#additional-configuration): variáveis que alguns gateways precisam além da URL base e credencial, como um cabeçalho personalizado, um auxiliar de credencial, descoberta de modelo, uma URL base em formato de provedor, ou desativar o tráfego fora do caminho do gateway. Defina estas apenas se seu administrador as nomeou ou sua rede restringe saída

<h3 id="set-the-credential-variable">
  Definir a variável de credencial
</h3>

Para autenticar Claude Code no gateway, defina sua credencial em uma variável de ambiente. Qual variável depende do que sua equipe de gateway disse:

| Definir a credencial em                                 | Use quando                                                           |
| :------------------------------------------------------ | :------------------------------------------------------------------- |
| `ANTHROPIC_AUTH_TOKEN`                                  | Sua equipe de gateway disse "bearer token" ou "Authorization header" |
| `ANTHROPIC_API_KEY`                                     | Sua equipe de gateway disse "API key" ou "x-api-key"                 |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | A credencial rotaciona ou vem de um vault                            |

Se você não foi informado qual tipo, use `ANTHROPIC_AUTH_TOKEN`; a [solicitação de verificação](#verify-the-connection) abaixo mostra como saber se você precisa mudar.

<h3 id="set-the-base-url-and-credential">
  Definir a URL base e a credencial
</h3>

Defina a URL base do gateway e a variável de credencial que você escolheu acima como variáveis de ambiente. Os exemplos usam `ANTHROPIC_AUTH_TOKEN`; troque-o por `ANTHROPIC_API_KEY` se essa for [a variável que você escolheu](#set-the-credential-variable). Você pode defini-los [em seu shell](#set-as-shell-environment-variables), que dura uma sessão de terminal, ou [em um arquivo de configurações Claude Code](#set-in-a-settings-file), que persiste em todos os lugares onde Claude Code é executado.

Para sua primeira conexão, comece com exportações de shell e execute a [solicitação de verificação](#verify-the-connection) antes de mover os valores para um arquivo de configurações.

<h4 id="set-as-shell-environment-variables">
  Definir como variáveis de ambiente do shell
</h4>

Substitua os valores pelos que sua equipe de gateway forneceu:

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
    ```
  </Tab>
</Tabs>

As exportações de shell se aplicam apenas a essa sessão de terminal e aos programas iniciados a partir dela. Um editor iniciado do dock ou menu Iniciar não as verá. Para torná-las persistentes em novos terminais, adicione as mesmas linhas ao seu perfil de shell, como `~/.zshrc`, `~/.bashrc` ou seu `$PROFILE` do PowerShell.

Se você exportar o gateway apenas em seu shell, ele não chega de forma confiável aos agentes de fundo hospedados pelo [supervisor](/docs/pt/agent-view#how-background-sessions-are-hosted); veja [como cada sessão de fundo obtém seu gateway](/docs/pt/agent-view#llm-gateway). Use um arquivo de configurações para qualquer gateway que agentes de fundo devem sempre rotear através.

<h4 id="set-in-a-settings-file">
  Definir em um arquivo de configurações
</h4>

Para fazer a configuração se aplicar em todos os lugares onde Claude Code é executado, incluindo [agentes de fundo](/docs/pt/agent-view#how-background-sessions-are-hosted), defina as variáveis no bloco `env` de um [arquivo de configurações](/docs/pt/settings) em vez de depender do seu shell. Os arquivos de configurações têm escopos diferentes:

* `~/.claude/settings.json` se aplica a todos os seus projetos. No Windows, o caminho é `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` se aplica a um projeto. Claude Code o adiciona ao seu gitignore global quando salva uma configuração lá; se você o criar manualmente ou tiver Claude escrevê-lo, adicione-o ao seu gitignore você mesmo primeiro para não cometer acidentalmente sua credencial

<Warning>
  Não coloque a credencial no `.claude/settings.json` de um projeto. Esse arquivo é confirmado e compartilhado com todos que clonam o repositório.
</Warning>

O bloco `env` se parece igual em qualquer arquivo:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

Quando uma exportação de shell e um bloco `env` de arquivo de configurações definem a mesma variável, o valor do arquivo de configurações se aplica. Execute `/status` para ver qual URL base e fonte de credencial Claude Code está usando.

<h3 id="verify-the-connection">
  Verificar a conexão
</h3>

Com as variáveis exportadas em seu shell, envie uma solicitação de um token para o gateway diretamente. Isso confirma que a URL e a credencial funcionam antes de você abrir Claude Code, para que uma falha aponte para o gateway em vez de sua configuração. Os comandos abaixo leem as variáveis do shell, então eles precisam das [exportações de shell](#set-as-shell-environment-variables) mesmo se você também colocar os valores em um arquivo de configurações.

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    curl -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
      -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
      -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

Se seu gateway espera chaves no cabeçalho `x-api-key`, substitua o cabeçalho `Authorization` por `x-api-key: $ANTHROPIC_API_KEY` no comando Bash, ou a entrada da tabela hash `"Authorization"` por `"x-api-key" = "$env:ANTHROPIC_API_KEY"` no comando PowerShell.

Uma resposta JSON que começa com `{"id":"msg_` e inclui um campo `"content":[...]` significa que o gateway é alcançável e a credencial funciona. Um erro nomeando um modelo desconhecido ainda prova que a URL e a credencial funcionam, já que o gateway autenticou a solicitação antes de rejeitar o nome do modelo; você não precisa encontrar um modelo que seu gateway serve para este teste. Um `401` significa que a credencial foi rejeitada: se você adivinhou a variável, mude para a outra e re-exporte.

<h4 id="confirm-in-claude-code">
  Confirmar em Claude Code
</h4>

Inicie `claude` do mesmo shell para que ele herde as exportações, envie uma mensagem e execute `/status`.

Na aba **Status**, a linha `Anthropic base URL` deve mostrar seu endereço de gateway, o que confirma que as solicitações estão sendo roteadas para lá; se a linha não estiver lá, a variável não chegou à sessão. Uma linha `Auth token` ou `API key` nomeando a variável que você definiu confirma que a credencial de gateway está ativa em vez de um login claude.ai salvo.

Se a mensagem falhar ou `/status` não mostrar a URL do gateway, consulte a [tabela de solução de problemas](#troubleshoot-gateway-errors) abaixo.

<h3 id="how-the-credential-variable-maps-to-a-header">
  Como a variável de credencial mapeia para um cabeçalho
</h3>

Cada variável envia a credencial em um cabeçalho HTTP diferente: `ANTHROPIC_AUTH_TOKEN` em `Authorization: Bearer`, `ANTHROPIC_API_KEY` em `x-api-key` e `apiKeyHelper` em ambos. Uma credencial na variável errada chega ao gateway em um cabeçalho que ele não lê, e a solicitação falha com `401`. Se a solicitação de verificação retornou `401`, mude para a outra variável e tente novamente.

<h3 id="conflicts-with-an-existing-login">
  Conflitos com um login existente
</h3>

Uma variável de credencial de gateway tem precedência sobre um login claude.ai salvo ou uma chave Console. Seu login claude.ai permanece salvo e não utilizado enquanto a variável está definida; desdefina a variável e Claude Code volta para ela. Com `ANTHROPIC_AUTH_TOKEN`, a variável tem precedência imediatamente. Com `ANTHROPIC_API_KEY`, você é solicitado uma vez em modo interativo para aprovar a chave antes que ela assuma o controle.

Execute `/status` para confirmar qual fonte de credencial está ativa. Se a inicialização mostrar um aviso de conflito de autenticação nomeando duas fontes, consulte a primeira linha da [tabela de solução de problemas](#troubleshoot-gateway-errors) para saber qual descartar. Para limpar um login salvo para que apenas a credencial de gateway permaneça, execute `/logout`.

<h2 id="configure-each-surface">
  Configurar cada superfície
</h2>

O CLI lê as variáveis de ambiente e arquivos de configurações acima. As outras superfícies são a extensão VS Code, o aplicativo desktop, GitHub Actions, o Agent SDK e as superfícies em nuvem, como Slack e a web; as seções abaixo cobrem se essas configurações chegam a cada uma.

<h3 id="vs-code-extension">
  Extensão VS Code
</h3>

Defina as variáveis de gateway para a [extensão VS Code](/docs/pt/vs-code) em `claudeCode.environmentVariables`, nas próprias configurações do usuário do VS Code abertas com o comando **Preferences: Open User Settings (JSON)**. A extensão verifica credenciais dessa configuração antes de iniciar, então é o lugar confiável para a credencial de gateway; valores em `~/.claude/settings.json` chegam ao processo gerado, mas não à verificação de login da própria extensão.

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

<h3 id="desktop-app">
  Aplicativo desktop
</h3>

O aplicativo desktop lê o roteamento de gateway de sua [configuração de inferência de terceiros](https://claude.com/docs/third-party/claude-desktop/gateway), não de `ANTHROPIC_BASE_URL` ou `settings.json`. Essa configuração pode vir de sua organização ou de um formulário no próprio aplicativo:

* **Distribuído por um administrador**: se sua organização [implantou a configuração](/docs/pt/llm-gateway-rollout#distribute-through-managed-settings), o aplicativo desktop roteia através do gateway sem nenhuma configuração de sua parte
* **Configurado localmente**: para dispositivos sem uma configuração distribuída por administrador, abra Help → Troubleshooting → Enable Developer Mode, que reinicia o aplicativo com um menu Developer. Em seguida, abra Developer → Configure Third-Party Inference e insira a URL base do seu gateway. Uma configuração distribuída por administrador tem precedência e torna esse formulário somente leitura

Com a configuração de gateway ativa, o aplicativo desktop executa sessões apenas em sua máquina local: o seletor de ambiente não oferece sessões SSH ou ambientes em nuvem hospedados pela Anthropic, e [Remote Control](/docs/pt/remote-control) não está disponível. Para usar Claude Code em um host remoto através do gateway, execute o CLI nesse host com [`ANTHROPIC_BASE_URL` e a credencial de gateway](#set-the-base-url-and-credential) definidos lá.

Se o aplicativo desktop mostrar `Gateway was unreachable`, o aplicativo não conseguiu alcançar a URL base configurada na inicialização; verifique a URL e o caminho de rede com o [teste curl acima](#verify-the-connection).

<h3 id="github-actions">
  GitHub Actions
</h3>

[Claude Code GitHub Actions](/docs/pt/github-actions) lê `ANTHROPIC_BASE_URL` e `ANTHROPIC_CUSTOM_HEADERS` do bloco `env` do workflow. Passe a credencial como a entrada `anthropic_api_key` da ação; a ação a define como `ANTHROPIC_API_KEY`, para que chegue ao gateway no cabeçalho `x-api-key`.

Para um gateway `x-api-key`, defina a URL base em `env` e passe a chave de gateway como a entrada:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Para um gateway de bearer token, passe o mesmo segredo duas vezes: como a entrada `anthropic_api_key` e como `ANTHROPIC_AUTH_TOKEN` no bloco `env` do workflow. A ação requer `anthropic_api_key`, `CLAUDE_CODE_OAUTH_TOKEN` ou federação de identidade de carga de trabalho antes de iniciar Claude Code, e não lê `ANTHROPIC_AUTH_TOKEN`, então a entrada está lá apenas para satisfazer essa verificação de inicialização. A variável de env é o que coloca a chave no cabeçalho `Authorization` que o gateway lê; a cópia em `x-api-key` é ignorada:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Para as outras opções de autenticação da ação, incluindo `CLAUDE_CODE_OAUTH_TOKEN` e federação de identidade de carga de trabalho, consulte [Claude Code GitHub Actions](/docs/pt/github-actions) e o [README](https://github.com/anthropics/claude-code-action#readme) da ação.

<h3 id="agent-sdk">
  Agent SDK
</h3>

O [Agent SDK](/docs/pt/agent-sdk/overview) não tem opções específicas de gateway; ele passa variáveis de ambiente para o processo Claude Code que gera. Cada SDK aceita uma opção `env` que define o ambiente do processo gerado, e os SDKs TypeScript e Python o tratam de forma diferente:

* TypeScript: o processo gerado herda o ambiente pai por padrão, mas definir `options.env` substitui o ambiente inteiramente. Espalhe `process.env` nele para manter suas variáveis de gateway.
* Python: `ClaudeAgentOptions(env=...)` mescla no topo do ambiente herdado, para que variáveis de gateway definidas no processo pai passem sem espalhar.

<CodeGroup>
  ```ts TypeScript theme={null}
  const result = query({
    prompt: "...",
    options: {
      env: {
        ...process.env,
        ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
        ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
      },
    },
  })
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
          "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
      }
  )
  ```
</CodeGroup>

<h3 id="slack-cloud-sessions-and-remote-control">
  Slack, sessões em nuvem e Remote Control
</h3>

[Claude Code no Slack](/docs/pt/slack) e [sessões em nuvem](/docs/pt/claude-code-on-the-web) sempre usam a API da Anthropic; eles não fazem parte de uma implantação de gateway. Variáveis de gateway definidas na configuração de ambiente de uma sessão em nuvem não são aplicadas. Se seu tráfego deve permanecer no gateway, não ative essas superfícies para esses usuários.

[Remote Control](/docs/pt/remote-control) e [ditado por voz](/docs/pt/voice-dictation) ambos dependem de uma identidade claude.ai: Remote Control para emparelhar uma sessão ao vivo com sua conta e ditado por voz para alcançar o endpoint de transcrição claude.ai. Eles não estão disponíveis enquanto `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou um `apiKeyHelper` está ativo. Remote Control também está desabilitado enquanto `ANTHROPIC_BASE_URL` aponta para um host não-Anthropic, então fazer login com claude.ai não é suficiente por si só. Antes da v2.1.196, uma URL base não-Anthropic não bloqueava Remote Control.

Para restaurar qualquer um dos recursos, faça login com claude.ai e desdefina as variáveis de gateway que o recurso verifica. A seção Remote Control de `claude doctor` nomeia o que está bloqueando Remote Control no momento.

* Ditado por voz: desdefina a credencial de gateway
* Remote Control: desdefina a credencial de gateway e `ANTHROPIC_BASE_URL`

<h2 id="additional-configuration">
  Configuração adicional
</h2>

Essas configurações cobrem casos além da URL base e credencial. Defina-as apenas se as instruções do seu administrador, as regras de saída da sua rede ou a [tabela de solução de problemas](#troubleshoot-gateway-errors) chamarem por uma.

<h3 id="send-additional-headers">
  Enviar cabeçalhos adicionais
</h3>

Alguns gateways roteiam ou marcam solicitações usando um cabeçalho personalizado além da credencial, por exemplo um identificador de locatário ou uma chave de roteamento. Para enviar um, defina [`ANTHROPIC_CUSTOM_HEADERS`](/docs/pt/env-vars) com um par `Name: Value` por linha. O exemplo abaixo adiciona um cabeçalho de roteamento nomeado `X-Org-Route`:

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
    ```
  </Tab>
</Tabs>

Você também pode definir `ANTHROPIC_CUSTOM_HEADERS` no bloco `env` de um arquivo de configurações. Use `\n` entre pares lá, já que strings JSON não podem abranger várias linhas:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

Nomes de cabeçalho de roteamento e locatário como esses contam como [cabeçalhos que precisam de aprovação](/docs/pt/server-managed-settings#environment-variables-and-the-approval-dialog). Quando os cabeçalhos vêm de um arquivo de configurações de projeto, Claude Code os aplica sob as [regras para quando aplica valores `env`](/docs/pt/settings-reference#when-claude-code-applies-env-values).

<h3 id="add-gateway-models-to-the-model-picker">
  Adicionar modelos de gateway ao seletor de modelo
</h3>

Com a descoberta de modelo ativada, Claude Code consulta o gateway para sua lista de modelos na inicialização e adiciona esses nomes ao seletor `/model` junto com as entradas integradas. Se você ou seu administrador definirem `replaceBuiltInOptions` em um [`modelPicker`](/docs/pt/settings-reference#modelpicker), Claude Code também oculta os nomes descobertos. Ele mantém uma linha para o modelo que a sessão já está usando.

Ative-a se seu gateway serve nomes de modelo que não estão na lista integrada de Claude Code e você quer selecioná-los do seletor. Se os modelos integrados são o que você usa, você não precisa de descoberta; seu administrador também pode já ter ativado através de configurações gerenciadas.

Para ativá-la, defina `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` em seu shell ou no bloco `env` de `~/.claude/settings.json`.

Os modelos descobertos aparecem como entradas `/model` adicionais. Cada entrada mostra a descrição que seu gateway fornece para o modelo, ou `From gateway` quando não fornece uma.

Para confirmar que a descoberta foi executada, inicie `claude --debug` e procure pelas linhas `[gatewayDiscovery]` no log de depuração em `~/.claude/debug/<session-id>.txt`. A primeira vez que a descoberta é bem-sucedida, Claude Code registra quantos modelos foram armazenados em cache, e registra novamente apenas quando a lista do gateway muda. Um `404`, timeout ou redirecionamento aparece lá também. Para quando a descoberta é executada, o que ela filtra e o formato de resposta que os gateways servem, consulte a [referência de descoberta de modelo](/docs/pt/llm-gateway-protocol#model-discovery).

<h3 id="rotate-credentials-with-apikeyhelper">
  Rotacionar credenciais com apiKeyHelper
</h3>

Um `apiKeyHelper` é um comando que Claude Code executa para buscar sua credencial de gateway, em vez de lê-la de uma variável de ambiente estática.

Use um auxiliar quando a credencial expira em um cronograma, vem de um vault ou comando SSO, ou seu administrador disse para você configurar um. Se sua credencial é uma string fixa que você define uma vez, a [variável de credencial](#set-the-credential-variable) é tudo que você precisa e você pode pular esta seção.

O auxiliar é qualquer comando de shell que imprime a credencial atual para stdout. Claude Code o executa através do seu shell do sistema, então no Windows pode ser um executável ou uma invocação do PowerShell. Faça o comando imprimir nada além da credencial. No Claude Code v2.1.227 ou posterior, um banner ou uma linha de log impressa junto com a chave faz o [auxiliar falhar](/docs/pt/errors#your-apikeyhelper-script-is-failing). Escreva o script, torne-o executável e referencie-o de `apiKeyHelper` em seu [arquivo de configurações](/docs/pt/settings):

<Tabs>
  <Tab title="Bash ou Zsh">
    Por exemplo, um script que lê de um vault:

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Referencie seu caminho em `~/.claude/settings.json`:

    ```json theme={null}
    {
      "apiKeyHelper": "~/bin/get-gateway-key.sh"
    }
    ```
  </Tab>

  <Tab title="PowerShell">
    Por exemplo, um script que lê de um vault:

    ```powershell theme={null}
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Referencie a invocação do PowerShell em `%USERPROFILE%\.claude\settings.json`, escapando as barras invertidas na string JSON:

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code armazena em cache a saída do auxiliar por cinco minutos por padrão e re-executa o auxiliar após o tempo de vida do cache expirar. Para alterar o tempo de vida, defina `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` em milissegundos, por exemplo `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` para 15 minutos.

Consulte [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) para os outros casos em que Claude Code re-executa o auxiliar.

O valor do auxiliar é enviado nos cabeçalhos `Authorization` e `x-api-key`, então funciona qualquer que seja o cabeçalho que seu gateway leia.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  Desativar tráfego fora do caminho do gateway
</h3>

O gateway carrega solicitações de modelo, mas Claude Code também envia tráfego de fundo não essencial fora do caminho do gateway, para Anthropic e para serviços de terceiros como GitHub: verificações de versão, telemetria, notas de lançamento e solicitações similares. Em uma rede que permite apenas saída para o gateway, essas solicitações falham e podem aparecer como conexões bloqueadas em seu monitoramento de saída.

Claude Code anexa uma credencial a uma solicitação de telemetria ou métricas de uso apenas quando a solicitação vai para o host ao qual a credencial pertence. Enquanto `ANTHROPIC_BASE_URL` aponta para o gateway, Claude Code envia seus eventos de telemetria para Anthropic sem sua credencial de gateway. Com uma [variável de credencial](#set-the-credential-variable) ou `apiKeyHelper` também ativa, Claude Code não relata métricas de uso para o painel [analytics](/docs/pt/analytics#access-analytics-for-api-customers) do Console. Antes de v2.1.246, Claude Code poderia anexar a credencial do gateway a solicitações de telemetria e métricas de uso destinadas a hosts Anthropic; solicitações de modelo sempre iam para o gateway com a credencial que o gateway espera.

Para desativar esse tráfego, defina `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` junto com as variáveis de gateway, no mesmo bloco de exportações de shell ou `env` do arquivo de configurações:

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
    ```
  </Tab>
</Tabs>

Definir a variável tem esses efeitos e limitações:

* Desativa atualizações automáticas, então planeje outro caminho de atualização, como seu gerenciador de pacotes ou distribuição gerenciada.
* Suprime a verificação de disponibilidade do [modo rápido](/docs/pt/fast-mode). A menos que uma verificação anterior já tenha ativado o modo rápido na máquina, `/fast` relata que o modo rápido está indisponível.
* Não afeta a [descoberta de modelo de gateway](#add-gateway-models-to-the-model-picker), que consulta apenas seu gateway. Antes de v2.1.257, a variável também parava a descoberta de atualizar, então o seletor mantinha a lista previamente armazenada em cache.
* A verificação de segurança de domínio da ferramenta WebFetch]\(/pt/data-usage#webfetch-domain-safety-check) não é afetada e ainda chama `api.anthropic.com`. Desative-a separadamente com `skipWebFetchPreflight: true` em [configurações](/docs/pt/settings) se sua rede bloquear esse host.
* Para cada fluxo de telemetria e a variável que a controla, consulte [serviços de telemetria](/docs/pt/data-usage#telemetry-services).

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  Rotear para um provedor em nuvem através de um gateway
</h3>

Essas configurações apontam Claude Code para um gateway através de uma variável de URL base específica do provedor no lugar de `ANTHROPIC_BASE_URL`. Gateways Amazon Bedrock e Google Cloud's Agent Platform aceitam formatos de solicitação nativos desses provedores; gateways Microsoft Foundry e Claude Platform on AWS aceitam o formato Anthropic Messages. Na rota Amazon Bedrock e Google Cloud's Agent Platform, Claude Code também limita os cabeçalhos beta e campos de solicitação que envia ao conjunto que o provedor aceita. Para o que seu gateway recebe em cada rota, consulte o [guia de compatibilidade de gateway](/docs/pt/llm-gateway-protocol).

Use uma apenas se sua equipe de gateway nomeou especificamente Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou Claude Platform on AWS. Se a [solicitação de verificação](#verify-the-connection) acima retornou JSON, você pode pular esta seção.

Defina o bloco para o provedor que sua equipe de gateway nomeou. As variáveis skip-auth nos blocos Amazon Bedrock, Google Cloud's Agent Platform e Claude Platform on AWS dizem a Claude Code para não assinar solicitações com credenciais do provedor em nuvem, já que o gateway as mantém. Se o gateway também precisa de seu próprio token, onde você o coloca depende do provedor:

* **Amazon Bedrock, Google Cloud's Agent Platform ou Claude Platform on AWS**: adicione `ANTHROPIC_AUTH_TOKEN` após o bloco. Claude Code o envia para o gateway como um cabeçalho `Authorization: Bearer`. Para uma credencial em um esquema ou cabeçalho diferente, use [`ANTHROPIC_CUSTOM_HEADERS`](#send-additional-headers) em vez disso. Mantenha a variável skip-auth definida de qualquer forma, já que sem ela Claude Code remove qualquer cabeçalho `Authorization` que `ANTHROPIC_AUTH_TOKEN`, um [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) ou `ANTHROPIC_CUSTOM_HEADERS` adicionaria.
* **Microsoft Foundry**: use `ANTHROPIC_FOUNDRY_API_KEY` conforme seu [bloco](#microsoft-foundry) mostra

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Deixe `AWS_BEARER_TOKEN_BEDROCK` indefinido quando o gateway emite sua própria credencial. Se você o definir, Claude Code envia essa [chave de API Amazon Bedrock](/docs/pt/amazon-bedrock#2-configure-aws-credentials) como o cabeçalho `Authorization` em vez de seu token de gateway, mesmo com `CLAUDE_CODE_SKIP_BEDROCK_AUTH` definido.

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
    export CLAUDE_CODE_USE_BEDROCK=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
    $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
    $env:CLAUDE_CODE_USE_BEDROCK = "1"
    ```
  </Tab>
</Tabs>

<h4 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h4>

Substitua o ID do projeto e a região pelos seus próprios valores. Claude Code inclui ambos no caminho de cada solicitação que envia para o gateway:

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
    $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
    $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
    $env:CLAUDE_CODE_USE_VERTEX = "1"
    $env:CLOUD_ML_REGION = "us-east5"
    ```
  </Tab>
</Tabs>

O bloco cobre roteamento e autenticação. As substituições de região e pins de modelo da [configuração do Agent Platform](/docs/pt/google-vertex-ai#4-configure-claude-code) se aplicam através de um gateway também:

* **Regiões por modelo**: se seu gateway serve alguns modelos de uma região diferente de `CLOUD_ML_REGION`, defina a variável `VERTEX_REGION_CLAUDE_*` correspondente para cada um, por exemplo `VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1`. A [referência de variáveis de ambiente](/docs/pt/env-vars) lista os nomes exatos.
* **Versões de modelo**: pin `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` e `ANTHROPIC_DEFAULT_HAIKU_MODEL` conforme em [Pin model versions](/docs/pt/google-vertex-ai#5-pin-model-versions). Definir `ANTHROPIC_DEFAULT_HAIKU_MODEL` também move tarefas de fundo como títulos de sessão para esse modelo, e essa seção explica qual modelo as executa de outra forma.
* **Capacidades de modelo**: se você pin um ID de modelo que sua versão de Claude Code não reconhece, recursos como níveis de esforço ou pensamento estendido podem permanecer desativados nele. Declare o que o modelo suporta com [`ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES`](/docs/pt/model-config#customize-pinned-model-display-and-capabilities) e seus equivalentes Sonnet e Haiku.

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Coloque a credencial do gateway em `ANTHROPIC_FOUNDRY_API_KEY`; ela é enviada para o gateway como o cabeçalho `x-api-key`. Um gateway que espera um token bearer pode usar [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/pt/env-vars) em vez disso. Claude Code envia esse valor como o cabeçalho `Authorization: Bearer`, e ele tem precedência sobre `ANTHROPIC_FOUNDRY_API_KEY` quando ambos estão definidos. Requer Claude Code v2.1.203 ou posterior.

Para um gateway que injeta seu próprio cabeçalho `Authorization`, defina `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` e deixe ambas as variáveis de credencial indefinidas. Claude Code então envia solicitações sem uma credencial do Azure e preserva o cabeçalho `Authorization` que você fornece, por exemplo através de `ANTHROPIC_CUSTOM_HEADERS`. Antes de v2.1.203, `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` sem uma chave de API deixava o cliente Microsoft Foundry incapaz de enviar solicitações.

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
    export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
    export CLAUDE_CODE_USE_FOUNDRY=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
    $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
    $env:CLAUDE_CODE_USE_FOUNDRY = "1"
    ```
  </Tab>
</Tabs>

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Consulte [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) para o ID do workspace.

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
    export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
    export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
    export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
    $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
    $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
    $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
    ```
  </Tab>
</Tabs>

<h4 id="confirm-the-provider-route">
  Confirmar a rota do provedor
</h4>

Inicie `claude` a partir do shell onde você definiu o bloco e execute `/status`. Com o bloco Amazon Bedrock, a aba **Status** mostra linhas como estas:

```text theme={null}
API provider: Amazon Bedrock
Bedrock base URL: https://llm-gateway.example.com/bedrock
AWS auth skipped
```

Os outros blocos produzem as mesmas linhas sob os nomes de seus provedores, por exemplo `Vertex base URL` e `GCP auth skipped` para Google Cloud's Agent Platform; o bloco Microsoft Foundry mostra uma linha de auth skipped apenas se você definir `CLAUDE_CODE_SKIP_FOUNDRY_AUTH`. Se você também rotear através de um proxy corporativo, uma linha `Proxy` mostra a URL do proxy. Se a linha de URL base está faltando, a variável não alcançou a sessão.

<h2 id="troubleshoot-gateway-errors">
  Solucionar erros de gateway
</h2>

Estes são os erros mais comuns ao executar Claude Code através de um gateway, com a causa do lado do gateway e a correção:

| Erro                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Causa                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Correção                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Um aviso de inicialização nomeando duas fontes de credencial e terminando em `auth may not work as expected`. Versões mais antigas mostram `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` em vez disso.                                                                                                                                                                                                                                                                               | Uma credencial de gateway e um login salvo estão ambos ativos; a variável é usada para solicitações, mas o login obsoleto pode causar comportamento de autenticação inesperado                                                                                                                                                                                                                                                                                      | Desdefina a variável para usar o login salvo, ou execute `/logout` para usar a credencial de gateway                                                                                                                                                                                                                                                                                                                                            |
| Erros `401` nomeando um token inválido ou não reconhecido                                                                                                                                                                                                                                                                                                                                                                                                                                                     | A credencial não é uma que o gateway emitiu, ou está em um cabeçalho que o gateway não lê                                                                                                                                                                                                                                                                                                                                                                           | Confirme que a variável corresponde ao seu tipo de credencial na [tabela de credencial](#set-the-credential-variable) e regenere a chave no gateway se ela foi revogada                                                                                                                                                                                                                                                                         |
| `Your apiKeyHelper script is failing`, ou `apiKeyHelper failed:` no stderr em modo não interativo                                                                                                                                                                                                                                                                                                                                                                                                             | O comando na configuração [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) não produziu uma chave utilizável, portanto as solicitações carregam uma chave de espaço reservado                                                                                                                                                                                                                                                                                  | Execute o comando diretamente para ver por que falha e autentique-se novamente com seu provedor de credencial se ele relatar uma sessão expirada; consulte [a referência de erro](/docs/pt/errors#your-apikeyhelper-script-is-failing)                                                                                                                                                                                                               |
| `Connection refused — a firewall or proxy may be blocking it (ConnectionRefused)` quando nada responde no endereço, ou `Can't reach the API server — check your internet or DNS (ENOTFOUND)` quando o nome do host não resolve, frequentemente após uma pausa silenciosa enquanto Claude Code [tenta novamente com backoff](/docs/pt/errors#automatic-retries). O código entre parênteses varia; [Unable to connect to API](/docs/pt/errors#unable-to-connect-to-api) cobre as variações de código e a redação anterior | Nada respondeu na URL base: o endereço está errado ou uma VPN ou firewall bloqueia o caminho para o gateway                                                                                                                                                                                                                                                                                                                                                         | Execute o [teste curl acima](#verify-the-connection), que falha imediatamente com a mesma causa, e confirme a URL e o caminho de rede com sua equipe de gateway                                                                                                                                                                                                                                                                                 |
| `API returned an empty or malformed response (HTTP 200)`                                                                                                                                                                                                                                                                                                                                                                                                                                                      | O gateway ou um proxy intermediário retornou uma resposta não-API, frequentemente uma página de erro HTML ou login                                                                                                                                                                                                                                                                                                                                                  | Teste com a [solicitação curl acima](#verify-the-connection); corrija a rota do gateway que retorna algo diferente de uma resposta da API Claude. [A referência de erro](/docs/pt/errors#api-returned-an-empty-or-malformed-response) explica o detalhe que a mensagem relata                                                                                                                                                                        |
| Erros `400` nomeando `context_management`, `Extra inputs are not permitted` ou outros campos não reconhecidos                                                                                                                                                                                                                                                                                                                                                                                                 | O gateway encaminha solicitações para um upstream que rejeita campos que Claude Code envia para endpoints em formato Anthropic                                                                                                                                                                                                                                                                                                                                      | Defina `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`, que suprime a maioria dos campos de pré-lançamento; consulte [passagem de recurso](/docs/pt/llm-gateway-protocol#feature-pass-through). Alguns betas não são controlados por este sinalizador; para esses, defina a variável de provedor `CLAUDE_CODE_USE_*` correspondente para que Claude Code envie apenas o que esse provedor aceita                                                          |
| Erros `400` nomeando `thinking` ou `adaptive`, como `Input tag 'adaptive' found`                                                                                                                                                                                                                                                                                                                                                                                                                              | A compilação do modelo upstream não aceita raciocínio adaptativo, que Claude Code solicita para modelos Claude 4.6 e posteriores                                                                                                                                                                                                                                                                                                                                    | Atualize o upstream do gateway. Em Opus 4.6 e Sonnet 4.6, `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` funciona em vez disso. As variáveis de capacidade de [configuração de modelo](/docs/pt/model-config) se aplicam apenas às configurações de provedor, como `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, não atrás de um gateway `ANTHROPIC_BASE_URL`                                                                                     |
| Erros `400` indicando um contexto ou limite de token nas próprias palavras do gateway, como `ContextWindowExceededError` ou `prompt token count of N exceeds the limit of M`                                                                                                                                                                                                                                                                                                                                  | O gateway impõe um contexto menor que a janela nativa do modelo e reescreve o erro upstream, para que Claude Code não o reconheça como um [erro muito longo](/docs/pt/errors#prompt-is-too-long) e não compacte e tente novamente automaticamente                                                                                                                                                                                                                        | Execute `/compact` para recuperar a sessão. Para evitar, defina `CLAUDE_CODE_AUTO_COMPACT_WINDOW` para o limite do gateway; Claude Code fixa o valor em pelo menos 100.000 tokens e no máximo a janela de contexto do modelo, para que um limite de gateway abaixo de 100.000 não possa ser correspondido e `/compact` permaneça a recuperação lá. Também defina `CLAUDE_CODE_MAX_OUTPUT_TOKENS` abaixo do limite de saída do modelo de gateway |
| Erros `400` em cada solicitação, nas próprias palavras do gateway rejeitando um esquema de entrada de ferramenta ou seu `pattern`, em Claude Code v2.1.265 até v2.1.267                                                                                                                                                                                                                                                                                                                                       | Em um lançamento gradual nessas versões, o esquema da [ferramenta Artifact](/docs/pt/artifacts#availability) carrega uma expressão regular com classes de caracteres Unicode `\p{...}`. A API Anthropic aceita, mas um gateway ou upstream que verifica o `pattern` de cada esquema de ferramenta com seu próprio mecanismo de regex rejeita toda a solicitação                                                                                                          | Atualize para v2.1.268 ou posterior, que não envia a expressão regular. Em uma versão afetada, [desative artefatos](/docs/pt/artifacts#disable-artifacts), que remove a ferramenta e seu esquema das solicitações                                                                                                                                                                                                                                    |
| Erros `400` em cada solicitação, nas próprias palavras do gateway rejeitando um tipo de ferramenta não reconhecido, como `Input tag 'advisor_20260301'`, em Claude Code v2.1.275                                                                                                                                                                                                                                                                                                                              | Em um lançamento gradual nessa versão, as solicitações carregam uma entrada de [ferramenta advisor](/docs/pt/advisor) mesmo com o advisor desativado. A API Anthropic aceita, mas um gateway ou upstream que valida tipos de ferramenta rejeita toda a solicitação; um que [encaminha campos de corpo de solicitação inalterados](/docs/pt/llm-gateway-protocol#forward-as-open-lists) passa sem ser afetado. A entrada é uma declaração que não carrega conteúdo de conversa | Atualize para v2.1.276 ou posterior, que não envia a entrada atrás de um gateway `ANTHROPIC_BASE_URL` a menos que você ative o advisor. Em v2.1.275, defina [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`](/docs/pt/env-vars), que remove a entrada das solicitações                                                                                                                                                                                         |
| Modelos faltando do seletor `/model`                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Nomes de modelo de gateway não estão na lista integrada de Claude Code, ou Claude Code está mostrando um lineup [`modelPicker`](/docs/pt/settings-reference#modelpicker) que substitui as opções integradas                                                                                                                                                                                                                                                              | Ative [descoberta de modelo de gateway](#add-gateway-models-to-the-model-picker) ou adicione nomes com as variáveis de [configuração de modelo](/docs/pt/model-config). Se Claude Code mostrar um lineup `modelPicker` substituto, adicione os modelos de gateway a ele, ou peça ao seu administrador para adicioná-los quando as configurações gerenciadas o fornecerem                                                                             |
| `/fast` relata `Fast mode unavailable due to network connectivity issues` enquanto as solicitações de inferência funcionam                                                                                                                                                                                                                                                                                                                                                                                    | A verificação de disponibilidade do [modo rápido](/docs/pt/fast-mode) vai diretamente para `api.anthropic.com` e não segue `ANTHROPIC_BASE_URL`, portanto a saída direta bloqueada falha na verificação. A mesma mensagem aparece em uma rede aberta quando a verificação apresenta uma chave emitida por gateway de `ANTHROPIC_API_KEY` ou um `apiKeyHelper` e Anthropic a rejeita                                                                                      | Coloque na lista de permissões `api.anthropic.com` se a saída estiver bloqueada, ou defina uma variável de skip; para uma chave de gateway rejeitada apenas as variáveis de skip ajudam. Consulte [usar modo rápido atrás de proxies e gateways LLM](/docs/pt/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                               |
| `/fast` relata `Fast mode has been disabled by your organization` em uma sessão autenticada com `ANTHROPIC_AUTH_TOKEN`, mesmo que a organização tenha modo rápido ativado                                                                                                                                                                                                                                                                                                                                     | A verificação de disponibilidade requer um login claude.ai ou uma chave de API Anthropic; com apenas um token de portador, Claude Code trata o modo rápido como desativado sem enviar a verificação                                                                                                                                                                                                                                                                 | Defina `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`; consulte [usar modo rápido atrás de proxies e gateways LLM](/docs/pt/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                                                                                                                                                                       |
| Claude Code pede para você fazer login mesmo que o [teste curl](#verify-the-connection) tenha sucesso                                                                                                                                                                                                                                                                                                                                                                                                         | O CLI não tem credencial própria: uma URL base alcançável não é uma, e em uma sessão interativa um bloco `env` no `.claude/settings.json` ou `.claude/settings.local.json` de um projeto se aplica apenas após o assistente de primeira execução e [prompt de confiança](/docs/pt/permissions#what-runs-before-you-trust-a-folder)                                                                                                                                       | Defina `ANTHROPIC_AUTH_TOKEN` em algum lugar que Claude Code leia antes da configuração de primeira execução: uma exportação de shell, o bloco `env` em `~/.claude/settings.json` ou configurações gerenciadas                                                                                                                                                                                                                                  |
| `ANTHROPIC_API_KEY` está definido mas ignorado, sem prompt                                                                                                                                                                                                                                                                                                                                                                                                                                                    | A chave precisa de uma aprovação única em sessões interativas, e uma chave previamente recusada é ignorada sem perguntar novamente                                                                                                                                                                                                                                                                                                                                  | Ative-a em `/config` com a opção `Use custom API key`                                                                                                                                                                                                                                                                                                                                                                                           |
| `This machine's managed settings require a first-party login`                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Configurações gerenciadas incluem `forceLoginMethod` ou `forceLoginOrgUUID`, que não podem coexistir com `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper`                                                                                                                                                                                                                                                                                              | Seu administrador deve remover `forceLoginMethod` e `forceLoginOrgUUID` das configurações gerenciadas para usar credenciais de gateway, ou remover a credencial de gateway para usar login de primeira parte. Os dois não podem ser combinados                                                                                                                                                                                                  |
| `403` com um corpo HTML como `403 Forbidden`, quando os próprios logs do gateway não mostram nenhuma solicitação recebida                                                                                                                                                                                                                                                                                                                                                                                     | Um firewall de aplicativo web ou proxy reverso na frente do gateway bloqueou o corpo da solicitação antes de chegar ao gateway. Os prompts de Claude Code incluem tags de estilo XML e código-fonte que correspondem a regras de corpo de cross-site-scripting, para que um teste curl curto passe enquanto uma sessão real não                                                                                                                                     | Isente o caminho `/v1/messages` do gateway da inspeção de corpo de solicitação. No AWS WAF esta é a regra gerenciada `CrossSiteScripting_Body`; no nginx com ModSecurity é a regra de corpo OWASP CRS equivalente                                                                                                                                                                                                                               |
| Erros de certificado ou TLS como `SSL certificate verification failed` ou `Self-signed certificate detected`, quando o [teste curl](#verify-the-connection) tem sucesso                                                                                                                                                                                                                                                                                                                                       | O runtime de Claude Code não está confiando na mesma autoridade de certificação que `curl` usa. Comum atrás de proxies de inspeção TLS corporativa                                                                                                                                                                                                                                                                                                                  | Defina `NODE_EXTRA_CA_CERTS` para o caminho do pacote CA; consulte [armazenamento de certificado CA](/docs/pt/network-config#ca-certificate-store)                                                                                                                                                                                                                                                                                                   |

Se Claude Code solicitar que você faça login repetidamente após remover a configuração de gateway, a causa é geralmente armazenamento de credencial em vez do gateway; consulte [erros de autenticação](/docs/pt/errors#authentication-errors).

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Visão geral de gateways LLM](/docs/pt/llm-gateway): o que é um gateway e como ele interage com assinaturas claude.ai
* [Implementar um gateway LLM para sua organização](/docs/pt/llm-gateway-rollout): a lista de verificação voltada para o administrador para implantar e distribuir configuração de gateway
* [Guia de compatibilidade de gateway](/docs/pt/llm-gateway-protocol): o que Claude Code envia para um gateway, incluindo os cabeçalhos e campos que o gateway deve encaminhar
* [Configurações](/docs/pt/settings): onde os arquivos de configurações vivem e como o bloco `env` é lido
* [Autenticação](/docs/pt/authentication): como variáveis de credencial, `apiKeyHelper` e login OAuth interagem
