> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar ambientes na nuvem

> Configure ambientes na nuvem para sessões na nuvem do Claude Code: níveis de acesso à rede, variáveis de ambiente, scripts de configuração e cache de ambiente.

<Note>
  Ambientes na nuvem se aplicam a [sessões na nuvem](/docs/pt/claude-code-on-the-web), que estão disponíveis em planos Pro, Max e Team, e para usuários Enterprise com [assentos premium ou assentos Chat + Claude Code](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan).
</Note>

Cada [sessão na nuvem](/docs/pt/claude-code-on-the-web) é executada em um ambiente na nuvem. Você pode configurar um ambiente para permitir ou negar [acesso à rede](#access-levels), [definir variáveis de ambiente](#set-environment-variables) para a sessão, em planos Pro e Max armazenar [credenciais de API](#add-api-credentials) que as sessões usam sem vê-las, e executar um [script de configuração](#setup-scripts) antes de Claude começar a trabalhar.

Os mesmos ambientes se aplicam em qualquer lugar onde você inicie uma sessão na nuvem: o [aplicativo Desktop](/docs/pt/desktop), o [aplicativo móvel Claude](/docs/pt/mobile), seu navegador em [claude.ai/code](https://claude.ai/code), o terminal com [`claude --cloud`](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud), [rotinas](/docs/pt/routines) e [Claude Tag](https://claude.com/docs/claude-tag/overview). Cada uma dessas superfícies também pode rotear para um [ambiente auto-hospedado](/docs/pt/self-hosted-environments). [Disponibilidade e limitações](/docs/pt/self-hosted-environments#availability-and-limitations) cobre o que Claude ainda não pode usar quando uma sessão do Claude Tag é executada em um.

<Info>
  Sessões de [Remote Control](/docs/pt/remote-control) conectam as interfaces web e móvel a uma sessão em sua própria máquina, que usa a rede e os arquivos da sua máquina, não um ambiente na nuvem. Sessões de canal do Claude Tag usam ambientes no nível da organização apenas, seja [ambientes compartilhados](#organization-shared-environments) ou [ambientes auto-hospedados](/docs/pt/self-hosted-environments).
</Info>

<h2 id="the-default-environment">
  O ambiente Default
</h2>

Se você ainda não tem um ambiente, a integração configura o ambiente **Default** para você. Como depende de onde você se integra:

* **Fluxos CLI como `/web-setup`**: criam **Default** para você
* **Integração web em Pro e Max**: cria **Default** para você
* **Integração web em Team e Enterprise**: mostra um formulário **Criar seu primeiro ambiente na nuvem** a menos que um Proprietário tenha ativado [Configuração rápida da web](/docs/pt/claude-code-on-the-web#github-authentication-options); mantenha os padrões do formulário e clique em **Criar e concluir** para obter o mesmo ambiente **Default**

**Default** não carrega nenhuma configuração própria:

* [Acesso à rede **Trusted**](#access-levels): as sessões alcançam registros de pacotes e outros [domínios na lista de permissões](#default-allowed-domains), e nada mais através da rede da sessão.
* Nenhuma outra configuração: **Default** não define variáveis de ambiente ou script de configuração, portanto as sessões começam apenas com as [ferramentas pré-instaladas](#installed-tools).

Com apenas **Default** disponível, cada sessão é executada nele. Quando você tem mais de um ambiente, as sessões escolhem um por superfície:

* No aplicativo Desktop, no aplicativo móvel e em claude.ai/code, as sessões que você inicia usam o ambiente mostrado no [seletor](#configure-your-environment). Um [padrão da organização](#organization-shared-environments) definido por um Proprietário preenche a seleção quando você não escolheu um. Threads em um [projeto](/docs/pt/claude-projects#project-settings-reference) usam o ambiente definido nas configurações do projeto.
* A partir da CLI, Claude Code usa sua escolha [`/remote-env`](#select-an-environment-from-the-cli), ou volta para o ambiente hospedado pela Anthropic quando sua lista tem um, e caso contrário para o primeiro ambiente em sua lista que não é um ambiente bridge, uma entrada [Remote Control](/docs/pt/remote-control) registra para representar sua própria máquina em vez de um ambiente na nuvem. Para um [ambiente auto-hospedado](/docs/pt/self-hosted-environments), passar `--environment <environment-id>` com seu ID `ccpool_` [quando você despacha uma sessão](/docs/pt/self-hosted-environments-testing#run-the-test-loop) substitui a escolha `/remote-env` e o fallback para essa invocação. Claude Code rejeita IDs `env_` hospedados pela Anthropic passados para a flag, portanto use `/remote-env` para direcioná-los. A flag requer Claude Code v2.1.224 ou posterior.

Configure um ambiente quando o padrão não for suficiente: quando Claude precisa alcançar domínios fora da [lista de permissões padrão](#default-allowed-domains), precisa de variáveis de ambiente definidas para suas sessões, ou precisa de dependências instaladas antes de começar a trabalhar.

<h2 id="configure-your-environment">
  Configure seu ambiente
</h2>

Crie, edite e arquive ambientes a partir do seletor de ambiente, que você acessa em [claude.ai/code](https://claude.ai/code) após a [integração web](/docs/pt/web-quickstart), ou a partir da caixa de prompt no [aplicativo Desktop](/docs/pt/desktop#cloud-sessions). Os ambientes que você cria são pessoais para sua conta; [ambientes compartilhados](#organization-shared-environments) criados por um Proprietário aparecem no mesmo seletor. Veja [Ferramentas instaladas](#installed-tools) para saber o que está disponível sem nenhuma configuração.

<Steps>
  <Step title="Abra o seletor de ambiente">
    Em [claude.ai/code](https://claude.ai/code), selecione o ícone de nuvem mostrando o nome do ambiente atual, na linha acima da caixa de mensagem. Não há página de configurações ou URL direto para o seletor.

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-selector.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=cc2813a5664519eaf5a89d793ce5af26" alt="O seletor de ambiente aberto acima da caixa de mensagem em claude.ai/code. O botão de nuvem mostrando o nome do ambiente Default fica na linha acima da caixa de mensagem. O menu aberto lista uma linha Local com rótulos Download e Desktop only, uma seção Cloud onde o ambiente Default é selecionado com uma marca de seleção e mostra um ícone de engrenagem de configurações ao passar o mouse, uma opção Add cloud environment e uma seção Remote Control com instruções de configuração." width="1672" height="682" data-path="images/cloud-environment-selector.png" />
    </Frame>
  </Step>

  <Step title="Adicione ou edite um ambiente">
    Selecione **Add cloud environment**, ou passe o mouse sobre um ambiente existente e selecione o ícone de configurações que aparece à direita. O diálogo inclui o nome, nível de acesso à rede, variáveis de ambiente e script de configuração. Quando você edita um ambiente na nuvem existente em um plano Pro ou Max, o diálogo também inclui [credenciais de API](#add-api-credentials).

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-dialog.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=30d4478b31d1f879f7ee287ddab32505" alt="O diálogo New cloud environment. Um campo Name com o placeholder Default, um seletor Network access definido como Trusted com links para a política de rede e níveis de acesso, uma caixa Environment variables mostrando texto placeholder no formato .env com uma nota de que os valores são visíveis para qualquer pessoa que use o ambiente, uma caixa Setup script descrita como um script Bash que é executado quando uma nova sessão é iniciada antes do Claude Code ser lançado, e botões Cancel e Create environment." width="874" height="1372" data-path="images/cloud-environment-dialog.png" />
    </Frame>
  </Step>
</Steps>

<h3 id="set-environment-variables">
  Defina variáveis de ambiente
</h3>

As variáveis de ambiente usam o formato `.env`, um par `KEY=value` por linha. Valores simples não precisam de aspas, e se você colocar um valor entre aspas com um par correspondente, as aspas não se tornam parte do valor. Coloque entre aspas um valor que abrange várias linhas ou contém um `#`: em um valor sem aspas, `#` inicia um comentário e o resto da linha é descartado.

O exemplo a seguir define três variáveis.

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

Cada sessão copia os valores do ambiente uma vez, na inicialização, em variáveis de ambiente ordinárias que qualquer comando que Claude execute pode ler. Como as sessões em execução não releem a configuração, editar ou adicionar variáveis afeta as sessões que você inicia depois; as sessões já em execução mantêm os valores com os quais começaram.

Uma sessão na nuvem também define algumas variáveis em si mesma quando inicia. Para [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/pt/claude-code-on-the-web#manage-context), o valor que a sessão define substitui um que você adiciona aqui, portanto adicionar essa chave aqui não tem efeito.

Qualquer pessoa que use o ambiente pode ler os valores. Em planos Pro e Max, use uma [credencial de API](#add-api-credentials) em vez disso para uma chave que o proxy do agente pode anexar a uma solicitação. As [solicitações que nunca recebem uma credencial](#requests-that-never-get-the-credential) estão listadas lá.

<h3 id="add-api-credentials">
  Adicione credenciais de API
</h3>

Uma credencial de API é uma chave de API ou token que você armazena em um ambiente na nuvem para que Claude possa chamar essa API de qualquer sessão no ambiente sem ver a chave. O proxy do agente da Anthropic adiciona a chave às solicitações para os hosts que você lista, depois que cada solicitação sai da VM da sessão. A chave nunca alcança Claude, os comandos que ele executa, ou as variáveis de ambiente da sessão.

As credenciais de API estão disponíveis em planos Pro e Max. Elas ainda não estão disponíveis em planos Team ou Enterprise, portanto a seção **API credentials** não aparece no diálogo de ambiente nesses planos.

<h4 id="requirements">
  Requisitos
</h4>

Dois destes decidem se você pode adicionar uma credencial, e dois decidem se o proxy do agente pode usá-la uma vez adicionada:

* **Função**: uma função de administrador da organização em sua organização claude.ai
  * Em Team e Enterprise, Proprietários a mantêm e Administradores não
  * Em Pro e Max, você a mantém em sua própria organização
  * Sem ela, você vê uma nota em vez da lista de credenciais, até mesmo em seus próprios ambientes. Peça a um Proprietário para adicionar a credencial a um ambiente compartilhado e execute suas sessões lá
* **Tipo de ambiente**: um ambiente na nuvem hospedado pela Anthropic que já existe. Um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) não tem credenciais de API
* **Acessibilidade de API**: a API aceita conexões da internet, porque as solicitações saem da rede da Anthropic
* **Chaves de criptografia**: se sua organização usa chaves de criptografia gerenciadas pelo cliente, você não pode salvar credenciais

<h4 id="add-a-credential">
  Adicione uma credencial
</h4>

Você adiciona credenciais uma de cada vez a partir do editor de um ambiente que já existe. O diálogo para um novo ambiente não as oferece. Também não há edição. Para alterar os hosts ou o valor de uma credencial, delete-a e adicione-a novamente.

<Steps>
  <Step title="Abra as credenciais de API do ambiente">
    [Abra o ambiente para edição](#configure-your-environment) em [claude.ai/code](https://claude.ai/code). No diálogo **Update cloud environment**, encontre **API credentials** abaixo de **Environment variables**. Você vê as credenciais já no ambiente, cada uma com os hosts aos quais se aplica.
  </Step>

  <Step title="Adicione a credencial">
    Selecione **Add credential** e preencha o formulário. Mantenha o **Credential type** padrão, **Bearer**, para uma chave de API que viaja em um cabeçalho de solicitação, e preencha estes campos:

    * **Name**: um rótulo para a credencial, como `Internal billing API`
    * **Allowed websites**: os hosts da API, como `api.example.com`. Um `*.` inicial corresponde a cada subdomínio
    * **Custom headers**: uma linha para o cabeçalho que carrega a chave. A linha começa com `Authorization` como o **Name** do cabeçalho e `Bearer` como seu **Prefix**; cole a chave em si como o **Value**. Para um cabeçalho como `X-Api-Key` que usa o valor simples, altere o nome e limpe o prefixo

    Para uma API que se autentica de outra forma, escolha um **Credential type** diferente. A lista é a mesma que [Claude Tag](https://claude.com/docs/claude-tag/overview), a integração do Slack para planos Team e Enterprise, oferece para [conexões](https://claude.com/docs/claude-tag/admins/add-connections).
  </Step>

  <Step title="Salve a credencial">
    Selecione **Connect**. A credencial aparece na lista com seus hosts, salva sem o botão **Save changes** do diálogo. Você não pode visualizar o valor novamente após salvar.
  </Step>
</Steps>

Para confirmar que a credencial funciona, inicie uma sessão no ambiente e peça a Claude para chamar a API, por exemplo com `curl`. A API responde como se a chave estivesse na solicitação, e a chave não aparece nas variáveis de ambiente da sessão ou em nenhum arquivo. Se a lista marca uma credencial **Not sent** em vez disso, a nota abaixo dela diz por quê e o que fazer. Duas credenciais cujos hosts se sobrepõem sem corresponder exatamente não recebem nenhum marcador, e o proxy do agente envia apenas uma delas.

<h4 id="which-requests-get-the-credential">
  Quais solicitações recebem a credencial
</h4>

O proxy do agente anexa uma credencial a uma solicitação quando o host da solicitação corresponde a um que você listou nessa credencial. As sessões podem alcançar esses hosts mesmo quando o [nível de acesso à rede](#access-levels) do ambiente não permitiria de outra forma, exceto os [hosts que nunca recebem a credencial](#requests-that-never-get-the-credential). A credencial se aplica em cada sessão que é executada no ambiente, quem quer que a tenha iniciado, até você deletá-la.

<h4 id="requests-that-never-get-the-credential">
  Solicitações que nunca recebem a credencial
</h4>

O proxy do agente nunca anexa uma credencial que você adiciona a estas solicitações:

* **GitHub**: o [proxy do GitHub](#github-proxy) autentica solicitações para GitHub em vez disso, portanto você não precisa de uma credencial de API para isso
* **A API Anthropic e registros de pacotes públicos**: `api.anthropic.com`, `registry.npmjs.org`, `jsr.io`, `npm.jsr.io`, `pypi.org`, `files.pythonhosted.org`, `index.crates.io` e `proxy.golang.org`
* **Solicitações de script de configuração**: Claude Code se conecta ao proxy do agente quando é lançado, depois que o [script de configuração](#setup-scripts) foi executado

<h3 id="select-an-environment-from-the-cli">
  Selecione um ambiente a partir da CLI
</h3>

Execute `/remote-env` em seu terminal para escolher o ambiente padrão para sessões na nuvem que você cria a partir da CLI, como [`claude --cloud`](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud). O comando abre um seletor de seus ambientes existentes e salva sua escolha na chave `remote.defaultEnvironmentId` em suas [configurações de usuário](/docs/pt/settings#where-settings-live), portanto se aplica em cada projeto em sua máquina até você alterar, a menos que a mesma chave seja definida em uma [camada de configurações](/docs/pt/settings#settings-precedence) de precedência mais alta, como as configurações do projeto de um repositório.

Um ID de [ambiente auto-hospedado](/docs/pt/self-hosted-environments), que tem a forma `ccpool_...`, segue uma regra de origem mais rigorosa. Veja [`remote.defaultEnvironmentId`](/docs/pt/settings-reference#remote-defaultenvironmentid) para as camadas de configurações que Claude Code honra isso.

`/remote-env` apenas define o padrão: não inicia uma sessão e não pode adicionar ou editar ambientes. Gerencie-os a partir do [seletor de ambiente](#configure-your-environment).

<h3 id="archive-an-environment">
  Arquive um ambiente
</h3>

Para arquivar um de seus próprios ambientes, abra-o para edição e selecione **Archive**. Um Proprietário arquiva um [ambiente compartilhado](#organization-shared-environments) a partir da página **Cloud environments** nas configurações de administrador. Você não pode excluir um ambiente, apenas arquivá-lo.

O arquivamento afeta novas sessões, não as em execução:

* As sessões já em execução no ambiente continuam funcionando.
* O ambiente desaparece do seletor e de `/remote-env`, portanto você não pode escolhê-lo para novas sessões.
* As credenciais de API no ambiente permanecem anexadas em suas sessões em execução. Delete qualquer uma que você não queira mais antes de arquivar.
* Nenhuma nova sessão pode ser iniciada em um ambiente arquivado, em qualquer superfície. Se o ambiente era seu [padrão CLI](#select-an-environment-from-the-cli) salvo, Claude Code inicia sessões na nuvem da CLI no ambiente hospedado pela Anthropic quando sua lista tem um, e caso contrário no primeiro ambiente em sua lista que não é um [ambiente bridge Remote Control](#the-default-environment). Qualquer coisa configurada com o ambiente explicitamente, como uma [rotina](/docs/pt/routines#environments-and-network-access), não pode iniciar novas sessões nele. Aponte-a para outro ambiente.

<h3 id="organization-shared-environments">
  Ambientes compartilhados da organização
</h3>

Em planos Team e Enterprise, um Proprietário pode criar ambientes na nuvem que são compartilhados com cada membro da organização. O mesmo papel gerencia tudo mais na página **Cloud environments** do administrador, incluindo [ambientes auto-hospedados](/docs/pt/self-hosted-environments); a função Admin não pode abrir a página. A lista completa de funções que podem abrir é a para [gerenciar configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings#access-control).

Os ambientes compartilhados aparecem no [seletor de ambiente](#configure-your-environment) de cada membro sob um cabeçalho **Organization**, depois dos ambientes pessoais do membro sob **Personal**, portanto uma equipe pode padronizar uma configuração em vez de cada membro recriá-la. Selecionar o ícone de configurações de um ambiente compartilhado lá abre um resumo somente leitura de sua configuração para cada membro, Proprietários inclusos.

Um Proprietário disponibiliza um ambiente para a organização de uma de duas maneiras:

* **Crie um ambiente compartilhado**: use a página **Cloud environments** nas [configurações de administrador](https://claude.ai/admin-settings), que é também onde Proprietários editam e arquivam ambientes compartilhados. Cada um tem um nome, um [nível de acesso à rede](#access-levels), [variáveis de ambiente](#set-environment-variables) no formato `.env` e um [script de configuração](#setup-scripts).
* **Compartilhe um ambiente pessoal**: abra um de seus próprios ambientes para edição no seletor de ambiente, depois compartilhe-o a partir da linha **Who can use it**. O ambiente mantém seu ID, portanto sessões e rotinas que já o usam não são afetadas, e cada membro pode então vê-lo e iniciar sessões nele.

Os Proprietários escolhem o [ambiente padrão](#the-default-environment) da organização separadamente, em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

Cada sessão de membro em um ambiente compartilhado lê suas variáveis, portanto não inclua segredos nelas. [Credenciais de API](#add-api-credentials), que dão às sessões uma chave que elas não podem ler, ainda não estão disponíveis em planos Team ou Enterprise.

<h3 id="set-the-environment-a-claude-tag-channel-uses">
  Defina o ambiente que um canal do Claude Tag usa
</h3>

Em canais do [Claude Tag](https://claude.com/docs/claude-tag/overview), Claude trabalha como a identidade compartilhada de sua organização, não como qualquer membro, portanto as sessões de canal usam ambientes no nível da organização apenas, seja ambientes compartilhados ou [ambientes auto-hospedados](/docs/pt/self-hosted-environments). Para dar a um canal uma cadeia de ferramentas que não é [pré-instalada](#installed-tools), como .NET, um Proprietário pode criar um [ambiente compartilhado](#organization-shared-environments) a partir da página **Cloud environments** do administrador com um [script de configuração](#setup-scripts) que a instala. Aponte o canal para um ambiente de uma de duas maneiras:

* Defina um ambiente compartilhado ou auto-hospedado como o [ambiente padrão](#the-default-environment) da organização em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).
* [Fixe um a um canal](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) nas configurações de administrador do Claude Tag.

<h2 id="network-access">
  Acesso à rede
</h2>

Cada ambiente define um nível de acesso à rede, que controla as conexões de saída que suas sessões podem fazer. O nível padrão, **Trusted**, permite registros de pacotes e outros [domínios na lista de permissões](#default-allowed-domains); **Custom** usa sua própria lista de domínios.

Para alterar o acesso à rede de um ambiente, [abra-o para edição](#configure-your-environment) e use o seletor **Network access** no diálogo. Um [ambiente compartilhado](#organization-shared-environments) abre como somente leitura lá, portanto um Owner altera seu acesso à rede a partir da página **Cloud environments** nas [configurações de administrador](https://claude.ai/admin-settings). O ícone de nuvem que abre o seletor aparece nas superfícies do aplicativo listadas em [O ambiente Default](#the-default-environment) e no [editor de rotina](/docs/pt/routines#environments-and-network-access); os ambientes pessoais não têm uma página separada nas configurações de sua conta claude.ai.

<Note>
  Os conectores MCP que você ativa em uma sessão ou rotina funcionam sem adicionar seus hosts aos **Allowed domains**, porque o tráfego do conector viaja através dos servidores da Anthropic em vez da rede da sessão. Isso depende do mesmo canal vinculado à Anthropic observado em [Segurança e isolamento](/docs/pt/claude-code-on-the-web#security-and-isolation). Desative qualquer conector que você não precise para limitar quais ferramentas Claude pode alcançar.
</Note>

<h3 id="access-levels">
  Níveis de acesso
</h3>

O campo **Network access** no [diálogo de ambiente](#configure-your-environment) usa um de quatro níveis:

| Nível       | Conexões de saída                                                                                               |
| :---------- | :-------------------------------------------------------------------------------------------------------------- |
| **None**    | Sem acesso à rede de saída através da rede da sessão                                                            |
| **Trusted** | [Domínios na lista de permissões](#default-allowed-domains) apenas: registros de pacotes, GitHub, SDKs na nuvem |
| **Full**    | Qualquer domínio                                                                                                |
| **Custom**  | Sua própria lista de permissões, opcionalmente incluindo os padrões                                             |

Qualquer que seja o nível que você escolha, as sessões ainda podem alcançar estes, porque cada um usa um caminho que não passa pela lista de permissões de rede da sessão:

* GitHub, através de seu [proxy separado](#github-proxy)
* [Conectores MCP](#network-access) que você ativa, cujo tráfego viaja através dos servidores da Anthropic
* Os hosts que você listou nas [credenciais de API](#add-api-credentials) do ambiente, exceto os [hosts que nunca recebem a credencial](#requests-that-never-get-the-credential)
* A API Anthropic, para as próprias solicitações do Claude Code, até mesmo em **None**, conforme observado em [Segurança e isolamento](/docs/pt/claude-code-on-the-web#security-and-isolation)

<h3 id="allow-specific-domains">
  Permita domínios específicos
</h3>

Para permitir domínios que não estão na lista Trusted, selecione **Custom** nas configurações de acesso à rede do ambiente, depois liste um domínio por linha no campo **Allowed domains**. Este exemplo permite três hosts que um projeto interno pode precisar.

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

As sessões neste ambiente agora podem alcançar `api.example.com`, qualquer subdomínio de `internal.example.com` e `registry.example.com`, e nenhum outro domínio através da rede da sessão. [Tráfego do GitHub](#github-proxy), [tráfego do conector MCP](#network-access) e solicitações para os hosts das [credenciais de API](#add-api-credentials) do ambiente, outros que os [hosts que nunca recebem a credencial](#requests-that-never-get-the-credential), não passam por essa lista de permissões. Um `*.` inicial corresponde a cada subdomínio. Para manter também os [domínios Trusted](#default-allowed-domains), marque **Also include default list of common package managers**; deixe desmarcado para permitir apenas o que você listar.

Se sua organização usa [artefatos](/docs/pt/artifacts#availability), você não precisa de `*.frame.claudeusercontent.com` na lista para as sessões lerem. Quando a lista deixa esse host de fora, Claude Code lê o conteúdo do artefato através da conexão da sessão com a Anthropic em vez disso. Mantenha o host em uma lista de permissões em duas situações:

* **Sessões neste ambiente abrem artefatos públicos de outra organização**: Claude Code busca aqueles do host diretamente, portanto adicione-o a esta lista.
* **Você está configurando a CLI local ou um executor auto-hospedado**: mantenha o host nessa lista de permissões. Veja [requisitos de acesso à rede](/docs/pt/network-config#network-access-requirements) e os [requisitos de rede](/docs/pt/self-hosted-environments-deploy#network-requirements) auto-hospedados.

Cada ambiente tem sua própria lista de domínios permitidos; não há uma lista de permissões no nível da organização que os administradores possam enviar para os ambientes de cada membro. As [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) ainda se aplicam dentro de sessões na nuvem, mas nenhuma delas adiciona domínios à lista de permissões de rede do ambiente. Para dar a um time uma lista padrão, um Owner pode criar um [ambiente compartilhado pela organização](#organization-shared-environments) com acesso à rede **Custom** e essa lista.

<h3 id="github-proxy">
  Proxy do GitHub
</h3>

Em ambientes hospedados pela Anthropic, todas as operações do GitHub passam por um proxy dedicado que mantém suas credenciais reais do GitHub fora da VM da sessão, independentemente do [nível de acesso](#access-levels) do ambiente. As sessões em um ambiente auto-hospedado autenticam operações git com credenciais que sua implantação fornece; [Configurar git](/docs/pt/self-hosted-environments-deploy#configure-git) cobre as opções, incluindo credenciais cunhadas por sessão e uma opção de entrada para este mesmo proxy. O proxy fornece:

* **Credenciais do Git**: o cliente git dentro da VM usa uma credencial com escopo, que o proxy verifica e troca por seu token real do GitHub.
* **Solicitações de API**: solicitações das ferramentas GitHub integradas e de `gh` sob o [placeholder `proxy-injected`](#work-with-github-issues-and-pull-requests), saem com suas credenciais reais substituídas.
* **Proteção de push**: `git push` funciona apenas contra o branch de trabalho atual da sessão; clonagem, busca e operações de PR funcionam normalmente.
* **Escopo do repositório**: as solicitações de API do GitHub e de ativos de lançamento alcançam apenas repositórios anexados à sessão, portanto um script de configuração que baixa ativos de lançamento de um repositório não anexado recebe um 403.
* **Restrições de GraphQL**: o proxy serve apenas um conjunto fixado de operações de GraphQL para fluxos de trabalho de solicitação de pull. O proxy rejeita tudo mais no endpoint de GraphQL com um 403 que diz `This GraphQL query is not enabled for this session` e nomeia o fallback REST, `gh api repos/{owner}/{repo}/...`. A restrição se aplica a cada solicitação através do proxy independentemente das credenciais que você fornece, portanto um `GH_TOKEN` que você define recebe o mesmo 403. Claude não pode alcançar APIs do GitHub que existem apenas em GraphQL, como Projects v2, através do proxy.

Os arquivos confirmados de repositórios públicos chegam através de `raw.githubusercontent.com`, que o [proxy de segurança](#security-proxy) manipula em vez disso. Esse domínio está na [lista Trusted](#default-allowed-domains) padrão, portanto esses arquivos permanecem acessíveis a menos que o [nível de acesso](#access-levels) do ambiente os exclua.

<h3 id="security-proxy">
  Proxy de segurança
</h3>

As sessões na nuvem em ambientes hospedados pela Anthropic são executadas atrás de um proxy de rede HTTP/HTTPS para fins de segurança e prevenção de abuso; em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments-deploy#default-deny-egress), o tráfego de saída sai através de seu próprio limite de rede em vez disso. Todo o tráfego de internet de saída de uma sessão hospedada pela Anthropic passa por esse proxy, que fornece:

* Proteção contra solicitações maliciosas
* Limitação de taxa e prevenção de abuso
* Filtragem de conteúdo para segurança aprimorada
* Uma trilha de auditoria no nível de DNS dos nomes de host solicitados

<h2 id="what’s-available-in-cloud-sessions">
  O que está disponível em sessões na nuvem
</h2>

Em ambientes hospedados pela Anthropic, cada sessão obtém uma máquina virtual (VM) fresca executando Ubuntu 24.04 em x86\_64, independentemente de seu próprio sistema operacional e arquitetura de CPU, com seu repositório clonado e cadeias de ferramentas comuns pré-instaladas. Quando uma dependência fornece binários pré-compilados, como gems Ruby com extensões nativas ou wheels Python pré-construídos, use sua compilação x86\_64 Linux para corresponder à VM. Esta seção cobre os padrões hospedados pela Anthropic, as ferramentas GitHub integradas, como [executar testes e serviços](#run-tests-start-services-and-add-packages) e os [limites de recursos](#resource-limits) que cada VM obtém.

<Note>
  As sessões que sua organização roteia para um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) são executadas em seus próprios executores em vez disso, com as ferramentas que sua imagem de executor fornece.
</Note>

<h3 id="what-carries-over-from-your-setup">
  O que é transferido de sua configuração
</h3>

As sessões na nuvem começam a partir de um clone fresco de seu repositório. Qualquer coisa que você confirme no repositório está disponível. Qualquer coisa que você tenha instalado ou configurado apenas em sua própria máquina não está disponível na sessão. A política de sua organização chega separadamente através das [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings).

|                                                                                                                                                                                                       | Disponível em sessões na nuvem                                       | Por quê                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Seu `CLAUDE.md` do repositório                                                                                                                                                                        | Sim                                                                  | Parte do clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Seus hooks `.claude/settings.json` do repositório e regras de permissão                                                                                                                               | Sim, em uma sessão com um repositório                                | Parte do clone. Uma sessão com vários repositórios, incluindo um thread de [projeto](/docs/pt/claude-projects#what-threads-pick-up-from-your-repositories), começa acima dos clones e não os lê                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Seus servidores MCP `.mcp.json` do repositório                                                                                                                                                        | Sim, em uma sessão com um repositório                                | Parte do clone, encontrado a partir do diretório de trabalho da sessão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Seu `.claude/rules/` do repositório                                                                                                                                                                   | Sim                                                                  | Parte do clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Seu `.claude/skills/`, `.claude/agents/`, `.claude/commands/` do repositório                                                                                                                          | Sim                                                                  | Parte do clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Plugins e marketplaces declarados em seu `.claude/settings.json` do repositório                                                                                                                       | Não                                                                  | Uma sessão na nuvem não instala os plugins que um repositório ativa em [`enabledPlugins`](/docs/pt/settings-reference#enabledplugins), incluindo aqueles dos marketplaces que lista em [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces)                                                                                                                                                                                                                                                                                                                                                                                                  |
| As [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) de sua organização                                                                                                          | Sim                                                                  | Buscadas dos servidores da Anthropic quando a sessão é iniciada. Veja [Cobertura de superfície](/docs/pt/model-config#surface-coverage) para como `availableModels` é aplicado em sessões na nuvem. As configurações implantadas em seu dispositivo através de MDM ou arquivos de configurações gerenciadas não se aplicam, porque a sessão é executada em uma VM gerenciada pela Anthropic; em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments), as sessões também leem o arquivo de configurações gerenciadas na imagem do executor, por [como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) |
| Seu `~/.claude/CLAUDE.md` do usuário                                                                                                                                                                  | Não                                                                  | Vive em sua máquina, não no repositório                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Seu `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` do usuário                                                                                                                        | Não                                                                  | Vivem em sua máquina, não no repositório. Confirme-os no diretório `.claude/` do repositório em vez disso. As sessões na nuvem carregam automaticamente skills que você ativa em claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Plugins ativados apenas em suas configurações de usuário                                                                                                                                              | Não                                                                  | O `enabledPlugins` com escopo de usuário vive em `~/.claude/settings.json` em sua máquina                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Servidores MCP que você adicionou com `claude mcp add` no escopo local padrão ou no escopo de usuário                                                                                                 | Não                                                                  | Aqueles escrevem em `~/.claude.json` em sua máquina, não no repositório. Adicione o servidor com `claude mcp add --scope project`, que escreve o [`.mcp.json`](/docs/pt/mcp#project-scope) do repositório, e confirme esse arquivo. Uma sessão com um repositório o carrega                                                                                                                                                                                                                                                                                                                                                                                       |
| Variáveis de transporte em seu bloco `env` `.claude/settings.json` do repositório, como `NODE_EXTRA_CA_CERTS` e as [variáveis de certificado de cliente mTLS](/docs/pt/network-config#mtls-authentication) | Não                                                                  | O ambiente de hospedagem gerencia a conexão de API da sessão, portanto Claude Code ignora essas chaves e anota cada chave ignorada no log de depuração da sessão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Chaves de API e tokens para serviços que Claude chama                                                                                                                                                 | Em planos Pro e Max, como [credenciais de API](#add-api-credentials) | Você adiciona a chave uma vez no ambiente e o proxy do agente a anexa às solicitações para os hosts que você lista. Uma chave que o proxy do agente [não pode anexar](#requests-that-never-get-the-credential), ou qualquer chave em um plano Team ou Enterprise, fica em uma variável de ambiente                                                                                                                                                                                                                                                                                                                                                           |
| Autenticação interativa como AWS SSO                                                                                                                                                                  | Não                                                                  | Não suportado. SSO requer login baseado em navegador que não pode ser executado em uma sessão na nuvem                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

Para disponibilizar sua própria configuração em sessões na nuvem, confirme-a no repositório.

Qualquer pessoa que use o ambiente pode ler suas variáveis de ambiente e script de configuração. A nota do diálogo em **Environment variables** diz isso e avisa contra colocar segredos lá. Em planos Pro e Max, armazene uma chave que o proxy do agente pode anexar como uma [credencial de API](#add-api-credentials) em vez disso.

<h3 id="installed-tools">
  Ferramentas instaladas
</h3>

As sessões na nuvem vêm com tempos de execução de linguagem comuns, ferramentas de compilação e bancos de dados pré-instalados. A tabela abaixo resume o que está incluído por categoria.

| Categoria     | Incluído                                                               |
| :------------ | :--------------------------------------------------------------------- |
| **Python**    | Python 3.x com pip, poetry, uv, black, mypy, pytest, ruff              |
| **Node.js**   | 20, 21 e 22, com npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**      | 3.1, 3.2, 3.3 com gem, bundler, rbenv                                  |
| **PHP**       | 8.3 com Composer                                                       |
| **Java**      | OpenJDK 21 com Maven e Gradle                                          |
| **Go**        | Go com suporte a módulos                                               |
| **Rust**      | rustc e cargo                                                          |
| **C/C++**     | GCC, Clang, cmake, ninja, conan                                        |
| **Docker**    | docker, dockerd, docker compose                                        |
| **Databases** | PostgreSQL 16, Redis 7.0                                               |
| **Utilities** | git, gh, jq, yq, ripgrep, tmux, vim, nano                              |

¹ Bun está instalado mas tem [problemas de compatibilidade](#install-dependencies-with-a-sessionstart-hook) de proxy conhecidos para busca de pacotes.

Para obter as versões da maioria das ferramentas nesta tabela, peça a Claude para executar `check-tools` em uma sessão na nuvem. É um comando shell instalado na VM da sessão, não um comando que você digita com `/`; você pede a Claude porque [Claude executa todos os comandos da VM para você](#run-tests-start-services-and-add-packages). Para uma ferramenta que não relata, como Ruby, PHP, bun, PostgreSQL ou Redis, peça a Claude para executar o comando de versão próprio da ferramenta, por exemplo `psql --version`.

As versões do Node.js estão instaladas em `/opt/node20`, `/opt/node21` e `/opt/node22`, com 22 em `PATH` por padrão. Para trabalhar com uma versão diferente, peça a Claude para prepender o diretório `bin` dessa versão, como `/opt/node20/bin`, a `PATH`.

As cadeias de ferramentas fora dessa lista, como o SDK .NET, não estão pré-instaladas mesmo quando seus registros de pacotes estão na [lista de permissões padrão](#default-allowed-domains). Instale-as com um [script de configuração](#setup-scripts).

<h3 id="work-with-github-issues-and-pull-requests">
  Trabalhe com problemas e solicitações de pull do GitHub
</h3>

As sessões na nuvem incluem ferramentas GitHub integradas que permitem a Claude ler problemas, listar solicitações de pull, buscar diffs e postar comentários sem nenhuma configuração. Essas ferramentas se autenticam através do [proxy do GitHub](#github-proxy) usando qualquer método que você configurou em [opções de autenticação do GitHub](/docs/pt/claude-code-on-the-web#github-authentication-options), portanto seu token nunca entra no contêiner.

Você pode definir `GH_TOKEN` ou `GITHUB_TOKEN` você mesmo nas [configurações de ambiente](#set-environment-variables), ou deixar ambos não definidos e deixar o [proxy do GitHub](#github-proxy) autenticar para você:

* Se você definir um token, ele passa para o contêiner inalterado, portanto seus scripts e o [`gh` CLI](https://cli.github.com) do GitHub usam-no diretamente.
* Se você não definir nenhum e o [proxy do GitHub](#github-proxy) estiver manipulando a autenticação para sua sessão, ambas as variáveis leem como a string placeholder `proxy-injected` nos comandos que Claude executa, e o proxy substitui suas credenciais reais em solicitações de saída do GitHub. `gh` funciona sem um token seu, mas um script que lê `GITHUB_TOKEN` diretamente obtém o placeholder, não um token utilizável.

Um token que você define é uma variável de ambiente ordinária, portanto qualquer pessoa que use o ambiente pode lê-lo; o caminho do proxy mantém a credencial fora da configuração do ambiente e da VM da sessão.

Para verificar qual caso se aplica à sua sessão, peça a Claude para executar `echo $GH_TOKEN`.

O [`gh` CLI](https://cli.github.com) do GitHub está pré-instalado. Se você precisar de um comando `gh` que as ferramentas integradas não cobrem, como `gh release` ou `gh workflow run`, peça a Claude para executá-lo. `gh` lê `GH_TOKEN` automaticamente, portanto você não precisa executar `gh auth login`.

<h3 id="link-output-back-to-the-session">
  Vincule a saída de volta à sessão
</h3>

Cada sessão na nuvem tem uma URL de transcrição em claude.ai, e a sessão pode ler seu próprio ID a partir da variável de ambiente `CLAUDE_CODE_REMOTE_SESSION_ID`. Use isso para colocar um link rastreável em corpos de PR, mensagens de commit, posts do Slack ou relatórios gerados para que um revisor possa abrir a execução que os produziu.

Os commits que Claude cria em uma sessão na nuvem incluem um trailer git `Claude-Session: <url>`, e os corpos de PR incluem a URL da sessão em sua própria linha. Para omitir o trailer e o link do corpo de PR, defina [`attribution.sessionUrl`](/docs/pt/settings-reference#attribution-sessionurl) como `false`.

Para incluir o link da sessão em algo diferente de um commit ou PR, como uma mensagem do Slack que Claude posta ou um arquivo de relatório que ele escreve, peça a Claude para executar o seguinte comando e use sua saída. O comando converte o prefixo `cse_` no valor da variável de ambiente para o prefixo `session_` que a URL de transcrição espera:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Execute testes, inicie serviços e adicione pacotes
</h3>

Você não obtém um shell na VM da sessão. Claude executa cada comando para você, portanto expresse as tarefas nesta seção como solicitações em seu prompt.

<h4 id="run-tests">
  Execute testes
</h4>

Claude executa testes como parte do trabalho em uma tarefa. Peça por isso em seu prompt, como "corrigir os testes falhando em `tests/`" ou "executar pytest após cada alteração." Os executores de teste que vêm com as [cadeias de ferramentas pré-instaladas](#installed-tools), como pytest e cargo test, funcionam sem configuração adicional. Um executor que seu projeto declara como uma dependência, como jest, instala com suas dependências.

<h4 id="start-services">
  Inicie serviços
</h4>

PostgreSQL e Redis estão pré-instalados mas não estão em execução por padrão. Peça a Claude para iniciar o que você precisar; os comandos que ele executa são:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker está disponível para executar serviços em contêiner. Peça a Claude para executar `docker compose up` para iniciar os serviços do seu projeto. O acesso à rede para puxar imagens segue o [nível de acesso](#access-levels) do seu ambiente, e os [padrões Trusted](#default-allowed-domains) incluem Docker Hub e outros registros comuns.

Se suas imagens forem grandes ou lentas para puxar, adicione `docker compose pull` ou `docker compose build` ao seu [script de configuração](#setup-scripts). O [cache do ambiente](#environment-caching) mantém as imagens puxadas, portanto cada nova sessão as tem no disco. O cache armazena apenas arquivos, não processos em execução, portanto Claude ainda inicia os contêineres cada sessão.

<h4 id="add-packages">
  Adicione pacotes
</h4>

Para adicionar pacotes que não estão pré-instalados, use um [script de configuração](#setup-scripts). O [cache do ambiente](#environment-caching) mantém o que o script instala, portanto os pacotes que você instala lá estão disponíveis no início de cada sessão sem reinstalar cada vez. Você também pode pedir a Claude para instalar pacotes no meio da sessão, mas essas instalações não se transferem para outras sessões.

<h3 id="resource-limits">
  Limites de recursos
</h3>

As sessões na nuvem em ambientes hospedados pela Anthropic são executadas com limites de recursos aproximados que podem mudar ao longo do tempo:

* 4 vCPUs
* 16 GB de RAM
* 30 GB de disco

A VM pode parar tarefas que precisam significativamente mais memória, como grandes trabalhos de compilação ou testes com uso intensivo de memória. Para cargas de trabalho além desses limites, use [Remote Control](/docs/pt/remote-control) para executar Claude Code em seu próprio hardware, ou execute sessões na nuvem em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) em computação que sua organização opera.

<h2 id="setup-scripts">
  Scripts de configuração
</h2>

Um script de configuração é um script Bash que é executado quando uma nova sessão na nuvem é iniciada, antes do Claude Code ser lançado. Use scripts de configuração para instalar dependências, configurar ferramentas ou buscar qualquer coisa que a sessão precise que não esteja pré-instalada.

Os scripts são executados como root no Ubuntu 24.04, portanto `apt install` e a maioria dos gerenciadores de pacotes de linguagem funcionam.

Para adicionar um script de configuração, abra o diálogo de configurações do ambiente e insira seu script no campo **Setup script**.

Este exemplo instala [ShellCheck](https://www.shellcheck.net/), que não está pré-instalado.

```bash theme={null}
#!/bin/bash
apt update && apt install -y shellcheck
```

<h3 id="script-requirements">
  Requisitos do script
</h3>

Um script de configuração tem três restrições para trabalhar:

* **Saia com zero**: se o script sair com não-zero, a sessão falha ao iniciar. Anexe `|| true` a comandos não críticos para que uma falha de instalação intermitente não bloqueie a sessão.
* **Termine em cinco minutos**: mantenha o tempo de execução total do script em aproximadamente cinco minutos para que o [cache do ambiente](#environment-caching) possa ser construído. Execute instalações independentes em paralelo com `&` e `wait`, e mova qualquer download único que não se encaixe em um [hook SessionStart](#setup-scripts-vs-sessionstart-hooks) que o inicie em segundo plano.
* **Acesso à rede para instalações**: as instalações de pacotes precisam alcançar registros. O nível **Trusted** padrão cobre [registros de pacotes comuns](#default-allowed-domains) incluindo npm, PyPI, RubyGems e crates.io; com acesso à rede **None**, as instalações falham.

<h3 id="environment-caching">
  Cache do ambiente
</h3>

O script de configuração é executado na primeira vez que você inicia uma sessão em um ambiente. Depois que é concluído, a Anthropic tira um snapshot do sistema de arquivos e reutiliza esse snapshot como ponto de partida para sessões posteriores. As novas sessões começam com suas dependências, ferramentas e imagens Docker já no disco, e pulam a etapa do script de configuração. Isso mantém a inicialização rápida mesmo quando o script instala cadeias de ferramentas grandes ou puxa imagens de contêiner.

O cache é um snapshot do sistema de arquivos, portanto mantém o que o script de configuração escreve no disco e perde qualquer coisa que estava apenas em execução. Os pacotes que você instala, as imagens Docker que você puxa e os arquivos que você escreve todos se transferem. Um banco de dados que o script iniciou, uma pilha `docker compose up` ou qualquer outro processo em segundo plano não; inicie aqueles por sessão pedindo a Claude ou com um [hook SessionStart](#setup-scripts-vs-sessionstart-hooks).

O script de configuração é executado novamente para reconstruir o cache quando você altera o script de configuração do ambiente ou hosts de rede permitidos, e quando o cache atinge sua expiração após aproximadamente sete dias. Retomar uma sessão existente nunca re-executa o script de configuração.

Você não precisa ativar o cache ou gerenciar snapshots você mesmo.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Scripts de configuração vs. hooks SessionStart
</h3>

Use um script de configuração para provisionar a própria VM: cadeias de ferramentas e ferramentas CLI que não estão [pré-instaladas](#installed-tools). Use um [hook SessionStart](/docs/pt/hooks#sessionstart) para configuração de projeto que deve ser executada em qualquer lugar, nuvem e local, como `npm install`.

Os scripts de configuração e hooks SessionStart são executados em uma ordem fixa quando uma sessão na nuvem é iniciada. A tabela compara onde você os configura, quando são executados e onde são executados.

|                                | Scripts de configuração                                                                                                                                                                     | Hooks SessionStart                                                                                                                                                                                                                                   |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Onde você os configura**     | O diálogo de ambiente em [claude.ai/code](https://claude.ai/code), mais a página **Cloud environments** do administrador para [ambientes compartilhados](#organization-shared-environments) | Um [arquivo de configurações](/docs/pt/settings#where-settings-live) como seu `.claude/settings.json` do repositório; veja [O que é transferido de sua configuração](#what-carries-over-from-your-setup) para quais arquivos alcançam uma sessão na nuvem |
| **Quando eles são executados** | Antes do Claude Code ser lançado, pulado quando um [ambiente em cache](#environment-caching) existe                                                                                         | Depois que Claude Code é lançado, em cada sessão incluindo retomada                                                                                                                                                                                  |
| **Onde eles são executados**   | Sessões na nuvem apenas                                                                                                                                                                     | Sessões locais e na nuvem                                                                                                                                                                                                                            |

Se você tem hooks SessionStart em seu `~/.claude/settings.json` no nível de usuário, não espere por eles na nuvem: as configurações no nível de usuário ficam em sua máquina. Qual outro hook é executado depende de onde a sessão é executada:

* **Ambiente hospedado pela Anthropic**: Claude Code executa hooks do repositório e das [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) de sua organização.
* **[Ambiente auto-hospedado](/docs/pt/self-hosted-environments-configuration#permissions-and-tool-approval)**: Claude Code também executa os hooks que o operador semeou a partir do `~/.claude/` do host do executor, e os hooks no arquivo de configurações gerenciadas da imagem do executor quando esse arquivo é um das [fontes gerenciadas que Claude Code aplica](/docs/pt/managed-settings#how-claude-code-combines-managed-sources).

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Instale dependências com um hook SessionStart
</h3>

Para instalar dependências apenas em sessões na nuvem, emparelhe um hook SessionStart com um script que verifica onde está sendo executado.

Primeiro, adicione um hook SessionStart ao seu `.claude/settings.json` do repositório. Esta configuração diz a Claude Code para executar `scripts/install_pkgs.sh` do seu repositório sempre que uma sessão é iniciada ou retomada:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

O `matcher` limita o hook aos eventos `startup` e `resume`, e `$CLAUDE_PROJECT_DIR` resolve para a raiz do repositório, portanto o hook encontra o script independentemente do diretório de trabalho da sessão.

Em seguida, crie o script em `scripts/install_pkgs.sh`. Ele sai imediatamente fora da nuvem, depois instala suas dependências:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

A verificação `CLAUDE_CODE_REMOTE` é o que escopa a instalação para sessões na nuvem: a VM do ambiente carrega essa variável como `true`, nunca é `true` localmente, portanto em seu laptop o script sai antes de instalar qualquer coisa.

Juntos, os dois arquivos dão a cada sessão na nuvem um `npm install` e `pip install` fresco na inicialização enquanto deixam as sessões locais intocadas.

<h4 id="limitations-in-cloud-sessions">
  Limitações em sessões na nuvem
</h4>

Os hooks SessionStart se comportam da mesma forma na nuvem que localmente, com essas ressalvas:

* **Um repositório por sessão**: uma sessão com vários repositórios não carrega hooks de nenhum `.claude/settings.json` do repositório, portanto um hook SessionStart que você define lá não é executado. Instale dependências para essas sessões com um [script de configuração](#setup-scripts) em vez disso.
* **Sem escopo apenas na nuvem**: os hooks são executados em sessões locais e na nuvem. Para pular a execução local, saia cedo a menos que a variável de ambiente `CLAUDE_CODE_REMOTE` seja `true`, da forma que o [script de instalação de dependência](#install-dependencies-with-a-sessionstart-hook) faz.
* **Requer acesso à rede**: os comandos de instalação precisam alcançar registros de pacotes. Se seu ambiente usa acesso à rede **None**, esses hooks falham. A [lista de permissões padrão](#default-allowed-domains) em **Trusted** cobre npm, PyPI, RubyGems e crates.io.
* **Compatibilidade de proxy**: em ambientes hospedados pela Anthropic, todo o tráfego de saída passa por um [proxy de segurança](#security-proxy), e alguns gerenciadores de pacotes não funcionam corretamente com isso; Bun é um exemplo conhecido. Em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments-deploy#default-deny-egress), o tráfego de saída vai através de seu próprio limite de rede em vez disso.
* **Adiciona latência de inicialização**: os hooks são executados cada vez que uma sessão é iniciada ou retomada, ao contrário dos scripts de configuração que se beneficiam do [cache do ambiente](#environment-caching). Mantenha os scripts de instalação rápidos verificando se as dependências já estão presentes antes de reinstalar.

Para personalizar a imagem base, use um script de configuração para instalar o que você precisa no topo da [imagem fornecida](#installed-tools), ou execute sua própria imagem como um contêiner ao lado de Claude com `docker compose`. Substituir a imagem base inteiramente ainda não é suportado.

<h2 id="default-allowed-domains">
  Domínios permitidos padrão
</h2>

Com acesso à rede **Trusted**, as sessões podem alcançar os seguintes domínios por padrão. Os domínios marcados com `*` indicam correspondência de subdomínio curinga, portanto `*.gcr.io` permite qualquer subdomínio de `gcr.io`.

<AccordionGroup>
  <Accordion title="Serviços Anthropic">
    * api.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Controle de versão">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Registros de contêiner">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Plataformas na nuvem">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
    * \*.r2.cloudflarestorage.com
  </Accordion>

  <Accordion title="Gerenciadores de pacotes JavaScript e Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
    * jsr.io
    * npm.jsr.io
  </Accordion>

  <Accordion title="Gerenciadores de pacotes Python">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Gerenciadores de pacotes Ruby">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Gerenciadores de pacotes Rust">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Gerenciadores de pacotes Go">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="Gerenciadores de pacotes JVM">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * maven.google.com
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * plugins-artifacts.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Outros gerenciadores de pacotes">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Distribuições Linux">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Ferramentas de desenvolvimento e plataformas">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Serviços na nuvem e monitoramento">
    * http-intake.logs.datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Entrega de conteúdo e espelhos">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Schema e configuração">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Referência de sessões na nuvem](/docs/pt/claude-code-on-the-web): inicie, gerencie e compartilhe sessões na nuvem
* [Guia de início rápido de sessões na nuvem](/docs/pt/web-quickstart): conecte GitHub e inicie sua primeira sessão na nuvem
* [Claude Tag](https://claude.com/docs/claude-tag/overview): as sessões que Claude inicia do Slack são executadas nos mesmos ambientes
* [Rotinas](/docs/pt/routines): as execuções agendadas usam os mesmos ambientes e níveis de acesso à rede
* [Remote Control](/docs/pt/remote-control): execute sessões na rede e nos arquivos de sua própria máquina em vez disso
* [Ambientes auto-hospedados](/docs/pt/self-hosted-environments): execute sessões na nuvem na infraestrutura própria de sua organização
* [Hooks SessionStart](/docs/pt/hooks#sessionstart): configuração confirmada no repositório que é executada em sessões locais e na nuvem
* [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings): política da organização que alcança sessões na nuvem
