> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autenticação

> Faça login no Claude Code e configure a autenticação para indivíduos, equipes e organizações.

Claude Code suporta múltiplos métodos de autenticação dependendo da sua configuração. Usuários individuais podem fazer login com uma conta Claude.ai, enquanto equipes podem usar Claude for Teams ou Enterprise, o Claude Console, ou um provedor de nuvem como Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Faça login no Claude Code
</h2>

Após [instalar Claude Code](/docs/pt/setup#install-claude-code), execute `claude` no seu terminal. No primeiro lançamento, Claude Code abre uma janela do navegador para você fazer login.

Se o navegador não abrir automaticamente, pressione `c` para copiar a URL de login para sua área de transferência, depois cole-a no seu navegador.

Se seu navegador mostrar um código de login em vez de redirecionar de volta após você se conectar, cole-o no terminal no prompt `Paste code here if prompted`. Isso acontece quando o navegador não consegue alcançar o servidor de callback local do Claude Code, o que é comum em WSL2, sessões SSH e contêineres.

Quando o login é concluído, o terminal mostra `Login successful` e solicita que você pressione `Enter` para continuar.

Você pode se autenticar com qualquer um destes tipos de conta:

* **Assinatura Claude Pro ou Max**: faça login com sua conta Claude.ai. Assine em [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams ou Enterprise**: faça login com a conta Claude.ai que seu administrador de equipe o convidou.
* **Claude Console**: faça login com suas credenciais do Console. Seu administrador deve ter [o convidado](#claude-console-authentication) primeiro.
* **Provedores de nuvem**: se sua organização usa [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) ou [Microsoft Foundry](/docs/pt/microsoft-foundry), defina as variáveis de ambiente necessárias antes de executar `claude`, ou selecione **plataforma de terceiros** no prompt de login, que inicia um assistente de configuração interativa para Bedrock e Vertex AI. Nenhum login do navegador é necessário.
* **Cloud gateway**: se sua organização executa um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado, faça login com SSO corporativo através de `/login`. O token emitido pelo gateway é a única credencial da sessão.

Administradores podem restringir login interativo com as configurações gerenciadas [`forceLoginMethod` e `forceLoginOrgUUID`](/docs/pt/settings#available-settings). Quando qualquer uma delas é definida, sessões autenticadas por `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` são bloqueadas na inicialização; sessões de provedores de nuvem não são afetadas.

Para fazer logout e se autenticar novamente, digite `/logout` no prompt do Claude Code. Fazer logout também redefine seu estado de configuração de primeiro lançamento, portanto, na próxima vez que você executar `claude`, ele o guiará novamente pelo login e configuração.

Se você está tendo problemas para fazer login, consulte [solução de problemas de autenticação](/docs/pt/troubleshoot-install#login-and-authentication).

<h2 id="set-up-team-authentication">
  Configure a autenticação da equipe
</h2>

Para equipes e organizações, você pode configurar o acesso ao Claude Code de uma destas formas:

* [Claude for Teams ou Enterprise](#claude-for-teams-or-enterprise), recomendado para a maioria das equipes
* [Claude Console](#claude-console-authentication)
* [Claude apps gateway](/docs/pt/claude-apps-gateway), um gateway auto-hospedado que faz login dos desenvolvedores com seu IdP e roteia a inferência para o provedor de nuvem que você configurar
* [Amazon Bedrock](/docs/pt/amazon-bedrock)
* [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai)
* [Microsoft Foundry](/docs/pt/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams ou Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) e [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise) fornecem a melhor experiência para organizações usando Claude Code. Os membros da equipe obtêm acesso tanto ao Claude Code quanto ao Claude na web com faturamento centralizado e gerenciamento de equipe.

* **Claude for Teams**: plano de autoatendimento com recursos de colaboração, ferramentas de administração e gerenciamento de faturamento. Melhor para equipes menores.
* **Claude for Enterprise**: adiciona SSO, captura de domínio, permissões baseadas em funções, API de conformidade e configurações de política gerenciada para configurações de Claude Code em toda a organização. Melhor para organizações maiores com requisitos de segurança e conformidade.

<Steps>
  <Step title="Assine">
    Assine [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) ou entre em contato com vendas para [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Convide membros da equipe">
    Convide membros da equipe do painel de administração.
  </Step>

  <Step title="Instale e faça login">
    Os membros da equipe instalam Claude Code e fazem login com suas contas Claude.ai.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Autenticação do Claude Console
</h3>

Para organizações que preferem faturamento baseado em API, você pode configurar o acesso através do Claude Console.

<Steps>
  <Step title="Crie ou use uma conta do Console">
    Use sua conta Claude Console existente ou crie uma nova.
  </Step>

  <Step title="Adicione usuários">
    Você pode adicionar usuários através de qualquer um dos métodos:

    * Convide usuários em massa de dentro do Console: Settings -> Members -> Invite
    * [Configure SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="Atribua funções">
    Ao convidar usuários, atribua uma das seguintes:

    * **Função Claude Code**: usuários podem apenas criar chaves de API do Claude Code
    * **Função Developer**: usuários podem criar qualquer tipo de chave de API
  </Step>

  <Step title="Usuários completam a configuração">
    Cada usuário convidado precisa:

    * Aceitar o convite do Console
    * [Verificar requisitos do sistema](/docs/pt/setup#system-requirements)
    * [Instalar Claude Code](/docs/pt/setup#install-claude-code)
    * Fazer login com credenciais da conta do Console
  </Step>
</Steps>

<h3 id="cloud-provider-authentication">
  Autenticação do provedor de nuvem
</h3>

Para equipes usando Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry:

<Steps>
  <Step title="Siga a configuração do provedor">
    Siga a [documentação do Amazon Bedrock](/docs/pt/amazon-bedrock), [documentação do Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) ou [documentação do Microsoft Foundry](/docs/pt/microsoft-foundry).
  </Step>

  <Step title="Distribua a configuração">
    Distribua as variáveis de ambiente e instruções para gerar credenciais de nuvem para seus usuários. Leia mais sobre como [gerenciar a configuração aqui](/docs/pt/settings).
  </Step>

  <Step title="Instale Claude Code">
    Os usuários podem [instalar Claude Code](/docs/pt/setup#install-claude-code).
  </Step>
</Steps>

<h2 id="credential-management">
  Gerenciamento de credenciais
</h2>

Claude Code gerencia com segurança suas credenciais de autenticação:

* **Local de armazenamento**:
  * No macOS, as credenciais são armazenadas no Keychain do macOS criptografado.
  * No Linux, as credenciais são armazenadas em `~/.claude/.credentials.json` com modo de arquivo `0600`.
  * No Windows, as credenciais são armazenadas em `%USERPROFILE%\.claude\.credentials.json` e herdam os controles de acesso do diretório do seu perfil de usuário, o que restringe o arquivo à sua conta de usuário por padrão.
  * Se você definiu a variável de ambiente `CLAUDE_CONFIG_DIR` no Linux ou Windows, o arquivo `.credentials.json` fica sob esse diretório em vez disso.
  * Claude Code gerencia `.credentials.json` através de `/login` e `/logout`. Para rotear solicitações através de um endpoint de API personalizado, defina a variável de ambiente [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) em vez disso.
* **Tipos de autenticação suportados**: credenciais Claude.ai, credenciais da API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth e tokens de sessão do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway).
* **Scripts de credenciais personalizados**: a configuração [`apiKeyHelper`](/docs/pt/settings#available-settings) pode ser configurada para executar um script de shell que retorna uma chave de API.
* **Intervalos de atualização**: por padrão, `apiKeyHelper` é chamado após 5 minutos ou em resposta HTTP 401. Defina a variável de ambiente `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` para intervalos de atualização personalizados.
* **Aviso de helper lento**: se `apiKeyHelper` levar mais de 10 segundos para retornar uma chave, Claude Code exibe um aviso na barra de prompt mostrando o tempo decorrido. Se você vir este aviso regularmente, verifique se seu script de credenciais pode ser otimizado.
* **Falhas do helper**: quando o script sai com um erro, expira ou não imprime nada, as solicitações falham com [`Your apiKeyHelper script is failing`](/docs/pt/errors#your-apikeyhelper-script-is-failing) dentro de três tentativas. Antes da v2.1.208, as falhas do helper apareciam como um 401 genérico após cerca de dez tentativas silenciosas.

`apiKeyHelper`, `ANTHROPIC_API_KEY` e `ANTHROPIC_AUTH_TOKEN` se aplicam à CLI e às superfícies que a envolvem, incluindo a extensão VS Code, o Agent SDK e GitHub Actions. Claude Desktop e sessões na nuvem não chamam `apiKeyHelper` ou leem essas variáveis de ambiente: eles usam OAuth, exceto sessões de desktop executando uma [configuração de inferência de terceiros](/docs/pt/llm-gateway-connect#desktop-app), que se autenticam com a credencial dessa configuração.

<h3 id="renew-an-expiring-login">
  Renovar um login que está expirando
</h3>

Quando o login que você criou com `/login` está dentro de cinco dias de expiração, Claude Code mostra um aviso na inicialização: `Your login expires in 3 days · run /login to renew`. Requer Claude Code v2.1.203 ou posterior.

Execute `/login` para renovar. O aviso é informativo e nunca bloqueia uma solicitação: a autenticação continua funcionando até que o login realmente expire. O tempo de vida do login em si não muda; o aviso antecipado é o que v2.1.203 adiciona.

Quando o login armazenado expira e não pode ser atualizado, cada solicitação falha com [`Login expired · Please run /login`](/docs/pt/errors#login-expired) até que você se conecte novamente. Antes da v2.1.206, um login expirado aparecia como um erro de modelo em vez disso.

O aviso aparece apenas quando um login claude.ai ou Claude Console é a credencial ativa, e não quando um provedor de nuvem, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` fornece a credencial.

Renovar antecipadamente é mais importante para sessões que são executadas sem supervisão. Uma [sessão em segundo plano na visualização de agente](/docs/pt/agent-view) ou uma sessão de [Remote Control](/docs/pt/remote-control) que sobrevive ao login para de fazer progresso uma vez que a credencial expira e não pode se recuperar até que você se conecte novamente.

<h3 id="authentication-precedence">
  Precedência de autenticação
</h3>

Quando múltiplas credenciais estão presentes, Claude Code escolhe uma nesta ordem:

1. Credenciais do provedor de nuvem, quando `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` ou `CLAUDE_CODE_USE_FOUNDRY` está definido. Consulte [integrações de terceiros](/docs/pt/third-party-integrations) para configuração.
2. Variável de ambiente `ANTHROPIC_AUTH_TOKEN`. Enviada como o cabeçalho `Authorization: Bearer`. Use isso ao rotear através de um [gateway LLM ou proxy](/docs/pt/llm-gateway) que autentica com tokens bearer em vez de chaves de API Anthropic.
3. Variável de ambiente `ANTHROPIC_API_KEY`. Enviada como o cabeçalho `X-Api-Key`. Use isso para acesso direto à API Anthropic com uma chave do [Claude Console](https://platform.claude.com). No modo interativo, você é solicitado uma vez a aprovar ou recusar a chave, e sua escolha é lembrada. Para alterá-la depois, use o toggle "Use custom API key" em `/config`. O toggle aparece apenas enquanto `ANTHROPIC_API_KEY` está definido em seu ambiente. No modo não interativo (`-p`), a chave é sempre usada quando presente.
4. Saída do script [`apiKeyHelper`](/docs/pt/settings#available-settings). Use isso para credenciais dinâmicas ou rotativas, como tokens de curta duração obtidos de um cofre.
5. Variável de ambiente `CLAUDE_CODE_OAUTH_TOKEN`. Um token OAuth de longa duração gerado por [`claude setup-token`](#generate-a-long-lived-token). Use isso para pipelines de CI e scripts onde login do navegador não está disponível.
6. Credenciais OAuth de assinatura de `/login`. Este é o padrão para usuários Claude Pro, Max, Team e Enterprise.

Uma sessão do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) assinada fica fora desta lista: é uma seleção de provedor como Amazon Bedrock ou Google Cloud's Agent Platform, e a supera. Quando uma sessão de gateway existe, a CLI se autentica com o token do gateway mesmo se `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` ou `CLAUDE_CODE_USE_FOUNDRY` está definido, e as entradas de token bearer, chave de API e `apiKeyHelper` acima não são usadas.

Se você tem uma assinatura Claude ativa mas também tem `ANTHROPIC_API_KEY` definido em seu ambiente, a chave de API tem precedência uma vez aprovada. Isso pode causar falhas de autenticação se a chave pertencer a uma organização desabilitada ou expirada. Execute `unset ANTHROPIC_API_KEY` para voltar à sua assinatura e verifique `/status` para confirmar qual método está ativo. A linha `Login method` mostra sua conta de assinatura, e uma linha `API key` aparece quando uma chave de API está em uso.

[Claude Code na Web](/docs/pt/claude-code-on-the-web) sempre usa suas credenciais de assinatura. Se você definir `ANTHROPIC_API_KEY` ou `ANTHROPIC_AUTH_TOKEN` no ambiente sandbox, isso não substitui suas credenciais de assinatura.

<h3 id="generate-a-long-lived-token">
  Gere um token de longa duração
</h3>

Para pipelines de CI, scripts ou outros ambientes onde login do navegador interativo não está disponível, gere um token OAuth de um ano com `claude setup-token`:

```bash theme={null}
claude setup-token
```

O comando o guia através da autorização OAuth e imprime um token no terminal. Ele não salva o token em lugar nenhum; copie-o e defina-o como a variável de ambiente `CLAUDE_CODE_OAUTH_TOKEN` onde você quiser se autenticar:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Este token se autentica com sua assinatura Claude e requer um plano Pro, Max, Team ou Enterprise. Ele é limitado apenas a inferência e não pode estabelecer sessões de [Remote Control](/docs/pt/remote-control).

[Bare mode](/docs/pt/headless#start-faster-with-bare-mode) não lê `CLAUDE_CODE_OAUTH_TOKEN`. Se seu script passar `--bare`, autentique com `ANTHROPIC_API_KEY` ou um `apiKeyHelper` em vez disso.
