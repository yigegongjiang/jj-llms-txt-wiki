> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autenticação

> Faça login no Claude Code e configure a autenticação para indivíduos, equipes e organizações.

Claude Code suporta múltiplos métodos de autenticação dependendo da sua configuração. Usuários individuais podem fazer login com uma conta Claude.ai, enquanto equipes podem usar Claude for Teams ou Enterprise, o Claude Console, ou um provedor de nuvem como Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Faça login no Claude Code
</h2>

Após [instalar Claude Code](/docs/pt/setup#install-claude-code), execute `claude` no seu terminal. No primeiro lançamento, Claude Code abre uma janela do navegador para você fazer login. Se você tiver definido a variável de ambiente `ANTHROPIC_API_KEY`, Claude Code pula o prompt de login e pede que você aprove a chave.

Se o navegador não abrir automaticamente, pressione `c` para copiar a URL de login para sua área de transferência, depois cole-a no seu navegador.

Se seu navegador mostrar um código de login em vez de redirecionar de volta após você se conectar, cole-o no terminal no prompt `Paste code here if prompted`. Isso acontece quando o navegador não consegue alcançar o servidor de callback local do Claude Code, o que é comum em WSL2, sessões SSH e contêineres.

Quando o login é concluído, o terminal mostra `Login successful` e solicita que você pressione `Enter` para continuar.

Você pode se autenticar com qualquer um destes tipos de conta:

* **Assinatura Claude Pro ou Max**: faça login com sua conta claude.ai. Assine em [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams ou Enterprise**: faça login com a conta claude.ai que seu administrador de equipe o convidou.
* **Claude Console**: faça login com suas credenciais do Console. Seu administrador deve ter [o convidado](#claude-console-authentication) primeiro. Você pode se conectar com ou sem [criar uma chave de API](#sign-in-without-an-api-key).
* **Provedores de nuvem**: se sua organização usa [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) ou [Microsoft Foundry](/docs/pt/microsoft-foundry), defina as variáveis de ambiente necessárias antes de executar `claude`, ou selecione **plataforma de terceiros** no prompt de login, que inicia um assistente de configuração interativa para Bedrock e Vertex AI. Nenhum login do navegador é necessário.
* **Cloud gateway**: se sua organização executa um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado, faça login com SSO corporativo através de `/login`. O token emitido pelo gateway é a única credencial da sessão.

Administradores podem direcionar qual método de login os desenvolvedores usam e exigir que logins claude.ai pertençam a uma organização específica; consulte [Restringir login à sua organização](#restrict-login-to-your-organization).

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

* **Claude for Teams**: plano de autoatendimento com recursos de colaboração, ferramentas de administração, SSO, gerenciamento de faturamento e [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) para configuração de Claude Code em toda a organização. Melhor para equipes menores.
* **Claude for Enterprise**: adiciona captura de domínio, permissões baseadas em funções e a API de conformidade. Melhor para organizações maiores com requisitos de segurança e conformidade.

<Steps>
  <Step title="Assine">
    Assine [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) ou entre em contato com vendas para [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Convide membros da equipe">
    Convide membros da equipe do painel de administração.
  </Step>

  <Step title="Instale e faça login">
    Os membros da equipe instalam Claude Code e fazem login com suas contas claude.ai.
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

<h4 id="sign-in-without-an-api-key">
  Faça login sem uma chave de API
</h4>

Você pode fazer login em sua conta do Console sem criar uma chave de API, mesmo quando sua organização não permite que desenvolvedores as criem. Escolha a conta Anthropic Console no prompt `/login` e Claude Code pergunta como você deseja fazer login. Requer Claude Code v2.1.242 ou posterior. Ambas as rotas fazem login no Console no navegador e diferem no que Claude Code armazena depois:

* **Faça login com sua conta do Console**, rotulado `(recomendado)`: Claude Code mantém o token OAuth desse login e o armazena como um [perfil Anthropic](#anthropic-profiles-and-federation-credentials). Não cria nenhuma chave de API
* **Crie uma chave de API**, rotulado `(legado)`: Claude Code cria uma chave de API do Console para você e a armazena com suas outras credenciais

Na prática, o perfil armazena um login OAuth enquanto uma chave de API é uma credencial estática: Claude Code atualiza automaticamente o login do perfil, e quando a atualização falha, as solicitações falham com [login do perfil Anthropic expirado](/docs/pt/errors#anthropic-profile-login-expired) até que você faça login novamente.

Você não obtém a escolha em cada máquina. Claude Code cria uma chave de API sem perguntar nestes casos:

* Você executa contra um provedor de nuvem, como [Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](/docs/pt/third-party-integrations) ou [Claude Platform on AWS](/docs/pt/claude-platform-on-aws)
* Qualquer arquivo de configurações define [`forceLoginOrgUUID`](#restrict-login-to-your-organization), ou define `forceLoginMethod` como `"claudeai"` ou `"console"`
* Uma fonte de configurações gerenciadas em sua máquina, como o arquivo de configurações gerenciadas, um perfil MDM ou as configurações gerenciadas pelo servidor em cache, existe mas Claude Code [não consegue lê-la](/docs/pt/managed-settings#invalid-entries-in-managed-settings) e nenhuma outra fonte gerenciada fornece uma política

Desdefina `ANTHROPIC_API_KEY` antes de fazer login sem uma chave. Um perfil escrito pelo próprio login do Console do Claude Code, ou pelo `ant auth login` da CLI do Claude Platform, é o mesmo tipo de credencial, então fazer login novamente o substitui.

Depois de fazer login sem uma chave, você tem um perfil em vez de uma chave de API armazenada:

* **Qual perfil ele escreve**: Claude Code escreve o perfil nomeado por `ANTHROPIC_PROFILE`, ou seu perfil ativo, ou `default`. Se esse perfil for um perfil de federação, Claude Code recusa o login em vez de sobrescrevê-lo
* **Do que ele faz logout**: Claude Code faz logout de qualquer login claude.ai armazenado na máquina
* **Como desfazer**: execute `/logout`, que remove e revoga a credencial que este login escreveu

Se sua organização usa [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings), elas se aplicam a este login no Claude Code v2.1.257 ou posterior.

Tudo mais sobre perfis se aplica a este login, incluindo onde ele se classifica em relação às suas outras credenciais, a linha `Profile` que você obtém em `/status`, e os recursos que precisam de um login claude.ai. Veja [Perfis Anthropic e credenciais de federação](#anthropic-profiles-and-federation-credentials).

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

<h3 id="restrict-login-to-your-organization">
  Restrinja o login à sua organização
</h3>

Para exigir que os logins claude.ai dos desenvolvedores pertençam a uma organização Anthropic específica, defina [`forceLoginMethod`](/docs/pt/settings-reference#forceloginmethod) e [`forceLoginOrgUUID`](/docs/pt/settings-reference#forceloginorguuid) em [configurações gerenciadas](/docs/pt/managed-settings). Defina `forceLoginOrgUUID` para seu ID de organização, mostrado em [configurações de administrador claude.ai](https://claude.ai/admin-settings/organization) para organizações Claude for Teams ou Enterprise. Claude Code relata um erro para um login claude.ai em qualquer outra organização e sai na inicialização se a credencial claude.ai em uso pertencer a uma organização que não esteja listada.

Para logins do Claude Console, Claude Code usa `forceLoginOrgUUID` para pré-selecionar a organização na página de login do Console quando você o define para um único ID de organização do Console, mostrado em [platform.claude.com/settings/organization](https://platform.claude.com/settings/organization). Ele não verifica a qual organização a credencial do Console resultante pertence, no login ou na inicialização, e um desenvolvedor que fez login com uma conta do Console antes de você implantar as chaves permanece conectado.

Se você definir `forceLoginOrgUUID` em qualquer arquivo de configurações, Claude Code para de oferecer o [login do Console sem chave](#sign-in-without-an-api-key) nas sessões às quais esse arquivo se aplica e cria uma chave de API em vez disso. Para direcionar os desenvolvedores para o login claude.ai em vez disso, defina `forceLoginMethod` como `"claudeai"`.

Os desenvolvedores podem fazer login de vários caminhos: o fluxo `/login` do terminal, a [extensão VS Code](/docs/pt/vs-code), o Agent SDK, `claude setup-token`, `/install-github-app`, e [login do gateway](/docs/pt/claude-apps-gateway) para organizações que roteiamthrough a cloud gateway. No Claude Code v2.1.212 ou posterior, cada caminho aplica `forceLoginMethod`; antes da v2.1.212, apenas logins de terminal aplicavam qualquer chave. Na tela de login interativa do terminal, alcançada por `/login` ou onboarding de primeira execução, Claude Code pré-seleciona um método `claudeai` ou `console` sem aplicá-lo, então mesmo com `forceLoginMethod` definido como `"claudeai"`, um desenvolvedor ainda pode completar um login do Console lá. Os caminhos diferem em `forceLoginOrgUUID`:

* **Logins de terminal, extensão VS Code e Agent SDK**: verificam `forceLoginOrgUUID` para logins de conta claude.ai
* **`claude setup-token` e `/install-github-app`**: aplicam apenas `forceLoginMethod`, então eles podem cunhar um token em uma organização diferente
* **[Login do gateway](/docs/pt/claude-apps-gateway)**: selecionado por `forceLoginMethod: "gateway"` em vez de restringido por ele, e não autentica contra uma organização Anthropic, então `forceLoginOrgUUID` não se aplica; use seu provedor de identidade do gateway para restringir o acesso

Implante as chaves através de sua ferramenta de gerenciamento de dispositivos. [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) alcançam apenas contas que já estão autenticadas em sua organização, então elas não podem redirecionar o primeiro login de um desenvolvedor. Se sua organização também distribui configurações gerenciadas pelo servidor, defina as chaves em ambos os lugares: fontes de [configurações gerenciadas](/docs/pt/server-managed-settings#settings-precedence) não se mesclam, e as configurações gerenciadas pelo servidor em cache substituem o arquivo gerenciado pelo dispositivo, exceto por alguns [exceções por chave](/docs/pt/server-managed-settings#per-key-exceptions-across-managed-sources). `forceLoginOrgUUID` e os valores `"claudeai"` e `"console"` de `forceLoginMethod` não estão entre essas exceções, então mantenha-os em ambos os lugares.

As chaves também decidem se uma sessão que não usa uma credencial de login pode iniciar. Veja [`forceLoginOrgUUID`](/docs/pt/settings-reference#forceloginorguuid) na referência de configurações para o comportamento completo.

* **`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper`**: bloqueado na inicialização, já que a associação à organização não pode ser verificada para uma credencial de ambiente
* **Sessões do provedor de nuvem como Amazon Bedrock**: não bloqueadas, porque elas autenticam contra seu provedor de nuvem. Restrinja-as através de suas políticas de IAM de nuvem
* **[Perfil Anthropic ou credenciais de federação](#anthropic-profiles-and-federation-credentials)**: não bloqueadas, e as chaves não verificam a qual organização o perfil pertence

<h2 id="credential-management">
  Gerenciamento de credenciais
</h2>

Claude Code gerencia com segurança suas credenciais de autenticação:

* **Local de armazenamento**:
  * No macOS, as credenciais são armazenadas no Keychain do macOS criptografado. Quando o Keychain rejeita a escrita, como quando está bloqueado em uma sessão SSH, Claude Code armazena seu login em `~/.claude/.credentials.json` com modo de arquivo `0600` em vez disso, o mesmo armazenamento que usa no Linux. Um login do Console que cria uma chave de API falha até que o Keychain seja gravável. Para mover seu login de volta para o Keychain, siga [as etapas de recuperação](/docs/pt/troubleshoot-install#not-logged-in-or-token-expired).
  * No Linux, as credenciais são armazenadas em `~/.claude/.credentials.json` com modo de arquivo `0600`.
  * No Windows, as credenciais são armazenadas em `%USERPROFILE%\.claude\.credentials.json` e herdam os controles de acesso do diretório do seu perfil de usuário, o que restringe o arquivo à sua conta de usuário por padrão.
  * Se você definiu a variável de ambiente `CLAUDE_CONFIG_DIR`, Claude Code mantém o arquivo `.credentials.json` sob esse diretório em vez disso, incluindo o arquivo que o fallback do macOS escreve, e chaves a entrada do Keychain do macOS para esse diretório também, então uma sessão com um `CLAUDE_CONFIG_DIR` diferente lê uma entrada diferente.
  * Claude Code gerencia `.credentials.json` através de `/login` e `/logout`. Para rotear solicitações através de um endpoint de API personalizado, defina a variável de ambiente [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) em vez disso.
* **Tipos de autenticação suportados**: credenciais claude.ai, credenciais da API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, credenciais de perfil Anthropic e [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), e tokens de sessão do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway).
* **Scripts de credenciais personalizados**: configure a configuração [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) para executar um script de shell que retorna uma chave de API.
* **Intervalos de atualização**: Claude Code executa novamente `apiKeyHelper` após cinco minutos por padrão. Defina a variável de ambiente `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` para intervalos de atualização personalizados. Consulte [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) para os outros casos em que Claude Code executa novamente o helper.
* **Aviso de helper lento**: se `apiKeyHelper` levar mais de 10 segundos para retornar uma chave, Claude Code exibe um aviso na barra de prompt mostrando o tempo decorrido. Se você vir este aviso regularmente, verifique se seu script de credenciais pode ser otimizado.
* **Falhas do helper**: quando o script sai com um erro, expira ou não imprime nada, as solicitações falham com [`Your apiKeyHelper script is failing`](/docs/pt/errors#your-apikeyhelper-script-is-failing) dentro de três tentativas. Antes da v2.1.208, as falhas do helper apareciam como um 401 genérico após cerca de dez tentativas silenciosas.

`apiKeyHelper`, `ANTHROPIC_API_KEY` e `ANTHROPIC_AUTH_TOKEN` se aplicam à CLI e às superfícies que a envolvem, incluindo a extensão VS Code, o Agent SDK e GitHub Actions. Claude Desktop e sessões na nuvem não chamam `apiKeyHelper` ou leem essas variáveis de ambiente: eles usam OAuth, exceto sessões de desktop executando uma [configuração de inferência de terceiros](/docs/pt/llm-gateway-connect#desktop-app), que se autenticam com a credencial dessa configuração.

<h3 id="renew-an-expiring-login">
  Renovar um login que está expirando
</h3>

Quando o login que você criou com `/login` está dentro de três dias de expiração, Claude Code mostra um aviso na inicialização: `Your login expires in 3 days · run /login to renew`. Requer Claude Code v2.1.203 ou posterior. Antes da v2.1.217, o aviso aparecia cinco dias antes.

Execute `/login` para renovar. O aviso é informativo e nunca bloqueia uma solicitação: a autenticação continua funcionando até que o login realmente expire. O tempo de vida do login em si não muda; o aviso antecipado é o que v2.1.203 adiciona.

Quando o login armazenado expira e não pode ser atualizado, cada solicitação de modelo falha com [`Login expired · Please run /login`](/docs/pt/errors#login-expired) até que você se conecte novamente. Antes da v2.1.206, Claude Code relatava um login expirado em solicitações de modelo como um erro de modelo em vez disso.

Você pode verificar este estado antes de uma solicitação falhar: [`/status`](/docs/pt/commands) mostra uma linha `Login` lendo `Expired — log in again`, mais a organização e o email que tem salvos para o login expirado. A linha aparece apenas quando o login claude.ai ou Claude Console salvo é a credencial ativa. A linha requer Claude Code v2.1.210 ou posterior.

O aviso aparece apenas quando um login claude.ai ou Claude Console é a credencial ativa, e não quando um provedor de nuvem, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` fornece a credencial.

Renovar antecipadamente é mais importante para sessões que são executadas sem supervisão. Uma [sessão em segundo plano na visualização de agente](/docs/pt/agent-view) ou uma sessão de [Remote Control](/docs/pt/remote-control) que sobrevive ao login para de fazer progresso uma vez que a credencial expira e não pode se recuperar até que você se conecte novamente.

<h3 id="authentication-precedence">
  Precedência de autenticação
</h3>

Quando múltiplas credenciais estão presentes, Claude Code escolhe uma nesta ordem:

1. Credenciais do provedor de nuvem, quando `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` ou `CLAUDE_CODE_USE_FOUNDRY` está definido. Consulte [integrações de terceiros](/docs/pt/third-party-integrations) para configuração.
2. Variável de ambiente `ANTHROPIC_AUTH_TOKEN`. Enviada como o cabeçalho `Authorization: Bearer`. Use isso ao rotear através de um [gateway LLM ou proxy](/docs/pt/llm-gateway) que autentica com tokens bearer em vez de chaves de API Anthropic.
3. Variável de ambiente `ANTHROPIC_API_KEY`. Enviada como o cabeçalho `X-Api-Key`. Use isso para acesso direto à API Anthropic com uma chave do [Claude Console](https://platform.claude.com). No modo interativo, você é solicitado uma vez a aprovar ou recusar a chave, e sua escolha é lembrada. Para alterá-la depois, use o toggle "Use custom API key" em `/config`. O toggle aparece apenas enquanto `ANTHROPIC_API_KEY` está definido em seu ambiente. No modo não interativo (`-p`), a chave é sempre usada quando presente.
4. Saída do script [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper). Use isso para credenciais dinâmicas ou rotativas, como tokens de curta duração obtidos de um cofre.
5. Variável de ambiente `CLAUDE_CODE_OAUTH_TOKEN`. Um token OAuth de longa duração gerado por [`claude setup-token`](#generate-a-long-lived-token). Use isso para pipelines de CI e scripts onde login do navegador não está disponível. Se você executar `/login` enquanto a variável está definida, Claude Code muda a sessão atual para o novo login, mas lê a variável novamente em cada nova sessão até que você a remova do seu perfil de shell ou do bloco `env` de um [arquivo de configurações](/docs/pt/settings).
6. Credenciais de perfil Anthropic e de federação, as credenciais que a CLI `ant` e Workload Identity Federation usam. Um perfil que `ant auth login` escreveu é classificado aqui apenas quando você o nomeia em `ANTHROPIC_PROFILE`; caso contrário, é classificado abaixo de `/login`. Consulte [Perfis Anthropic e credenciais de federação](#anthropic-profiles-and-federation-credentials).
7. Credenciais OAuth de assinatura de `/login`. Este é o padrão para usuários Claude Pro, Max, Team e Enterprise.

Uma sessão do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) assinada fica fora desta lista: é uma seleção de provedor como Amazon Bedrock ou Google Cloud's Agent Platform, e a supera. Quando uma sessão de gateway existe, a CLI se autentica com o token do gateway mesmo se `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` ou `CLAUDE_CODE_USE_FOUNDRY` está definido, e fontes de credenciais acima como o token bearer, chave de API, `apiKeyHelper` e perfis não são usados.

Se as [configurações gerenciadas](/docs/pt/managed-settings) da sua máquina definirem [`forceLoginMethod`](/docs/pt/settings-reference#forceloginmethod) como `"gateway"` ou definirem [`forceLoginGatewayUrl`](/docs/pt/settings-reference#forcelogingatewayurl), e você não selecionar um provedor de nuvem através de uma variável como `CLAUDE_CODE_USE_BEDROCK` ou `CLAUDE_CODE_USE_VERTEX`, sua sessão usa apenas o sign-in do gateway. Claude Code pula as outras fontes de credenciais e pede que você se conecte com `/login`. Consulte [Administrator policy requires a Cloud gateway sign-in](/docs/pt/errors#administrator-policy-requires-a-cloud-gateway-sign-in) para ver o que você vê com cada credencial restante. Antes da v2.1.261, ou antes da v2.1.265 em uma máquina que define apenas `forceLoginGatewayUrl`, Claude Code usava um login salvo restante nessas máquinas até que você se conectasse ao gateway.

Se você tem uma assinatura Claude ativa mas também tem `ANTHROPIC_API_KEY` definido em seu ambiente, Claude Code usa a chave de API uma vez que você a aprova. Isso pode causar falhas de autenticação se a chave pertencer a uma organização desabilitada ou expirada.

Execute `unset ANTHROPIC_API_KEY` para voltar à sua assinatura e verifique `/status` para confirmar qual método está ativo. Quando um login e uma chave de API estão ambos configurados, `/status` marca a credencial que não está em uso.

[Sessões na nuvem](/docs/pt/claude-code-on-the-web) sempre usam suas credenciais de assinatura. Se você definir `ANTHROPIC_API_KEY` ou `ANTHROPIC_AUTH_TOKEN` no ambiente na nuvem, isso não substitui suas credenciais de assinatura.

<h4 id="anthropic-profiles-and-federation-credentials">
  Perfis Anthropic e credenciais de federação
</h4>

Um perfil é um arquivo de configuração de credencial nomeado em seu [diretório de configuração Anthropic](https://platform.claude.com/docs/en/manage-claude/wif-reference#configuration-directory), por padrão `~/.config/anthropic` no macOS e Linux ou `%APPDATA%\Anthropic` no Windows. O modo de autenticação de um perfil é `oidc_federation` quando você o configura para [Workload Identity Federation (WIF)](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) ou `user_oauth` quando [`ant auth login`](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication) o escreveu ou você [se conectou a uma conta do Console sem uma chave de API](#sign-in-without-an-api-key).

Claude Code não lê perfis ou variáveis de federação em [modo bare](/docs/pt/headless#start-faster-with-bare-mode), no Claude Desktop ou em sessões na nuvem. Nessas sessões, `/status` não mostra nenhuma linha `Profile`.

Claude Code verifica três fontes nesta ordem e para na primeira que está definida. A tabela mostra o que define cada fonte e onde é classificada em relação à sua credencial `/login`.

| Fonte                  | Definido por                                                                                                                                                                 | Classificação em relação a `/login`                                                                                                                     |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Perfil nomeado         | `ANTHROPIC_PROFILE`                                                                                                                                                          | Acima, qualquer que seja o modo de autenticação do perfil                                                                                               |
| Variáveis de federação | `ANTHROPIC_FEDERATION_RULE_ID` e `ANTHROPIC_ORGANIZATION_ID`, ambas definidas                                                                                                | Acima                                                                                                                                                   |
| Perfil ativo           | O arquivo [`active_config`](https://platform.claude.com/docs/en/manage-claude/wif-reference#active-profile) em seu diretório de configuração, ou um perfil nomeado `default` | Acima quando seu modo de autenticação é `oidc_federation`; abaixo de uma credencial `/login` funcionando quando seu modo de autenticação é `user_oauth` |

A regra `user_oauth` impede que um perfil `ant auth login` deixado para trás mova suas solicitações para fora da conta em que você se conectou com `/login`. Para as variáveis de federação, Claude Code também lê as outras variáveis na [referência WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#environment-variables), como `ANTHROPIC_IDENTITY_TOKEN_FILE`, quando troca seu token de identidade. Para o formato do arquivo de perfil, consulte a [referência WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#profile-configuration-file).

Para confirmar qual fonte Claude Code escolheu, execute `/status`. Uma linha `Profile` nomeia a fonte no lugar da linha `Login method`. Quando o perfil é a credencial em uso, `Organization` e `Email` mostram sua conta.

Se você iniciar Claude Code com `--debug`, ele também escreve uma linha `Using Anthropic profile auth` com o nome da fonte no log de depuração em `~/.claude/debug/<session-id>.txt`. Quando Claude Code passa por um perfil ativo `user_oauth` porque você tem uma credencial `/login` funcionando, ele escreve um aviso no log de depuração dizendo que está usando o login claude.ai em vez disso.

Quando o login de um perfil `user_oauth` expirou e Claude Code não pode renová-lo, as solicitações falham com [Anthropic profile login expired](/docs/pt/errors#anthropic-profile-login-expired).

Recursos que precisam de seu login claude.ai, como [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) e [`/schedule`](/docs/pt/routines), não estão disponíveis enquanto uma dessas fontes está selecionada. Para impedir que Claude Code selecione uma fonte:

* **Perfil nomeado ou variáveis de federação**: desdefina `ANTHROPIC_PROFILE` ou desdefina qualquer variável de federação
* **Perfil ativo**: execute `/logout` para um perfil `user_oauth` cuja credencial atual você escreveu [conectando-se a uma conta do Console sem uma chave de API](#sign-in-without-an-api-key), execute `ant auth logout` para um cuja credencial atual `ant auth login` escreveu, ou delete o arquivo do perfil de `configs/` em seu diretório de configuração para qualquer modo de autenticação

<h3 id="generate-a-long-lived-token">
  Gere um token de longa duração
</h3>

Para pipelines de CI, scripts ou outros ambientes onde login do navegador interativo não está disponível, gere um token OAuth de um ano com `claude setup-token`:

```bash theme={null}
claude setup-token
```

O comando abre o mesmo fluxo de autorização do navegador que `/login`, e o token é impresso no terminal depois que você aprova o acesso no navegador. Ele não salva o token em lugar nenhum; copie-o e defina-o como a variável de ambiente `CLAUDE_CODE_OAUTH_TOKEN` onde você quiser se autenticar:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Este token se autentica com sua assinatura Claude e requer um plano Pro, Max, Team ou Enterprise. Ele pode apenas fazer solicitações de modelo, então não pode estabelecer sessões de [Remote Control](/docs/pt/remote-control) ou buscar [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai). Servidores MCP que você configura localmente ainda funcionam.

[Bare mode](/docs/pt/headless#start-faster-with-bare-mode) não lê `CLAUDE_CODE_OAUTH_TOKEN`. Se seu script passar `--bare`, autentique com `ANTHROPIC_API_KEY` ou um `apiKeyHelper` em vez disso.
