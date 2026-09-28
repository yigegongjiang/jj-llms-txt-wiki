> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implantar ambientes auto-hospedados em produção

> Execute runners auto-hospedados em produção: endurecimento de segurança, controle de saída de rede, credenciais git, receitas Kubernetes e Compose, e solução de problemas.

<Note>
  Ambientes auto-hospedados estão em beta pública em planos Team e Enterprise; [Disponibilidade e limitações](/docs/pt/self-hosted-environments#availability-and-limitations) cobre o caminho de habilitação. Esta página cobre a execução da frota em produção; consulte o [guia de início rápido](/docs/pt/self-hosted-environments-quickstart) para seu primeiro runner e sessão.
</Note>

Um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) executa [sessões na nuvem](/docs/pt/claude-code-on-the-web) do Claude Code em runners que você implanta dentro de sua rede, e em produção essas sessões executam código direcionado pelo modelo em nome de todos que podem enviar uma sessão para o ambiente. Esta página é para o operador que leva um ambiente funcionando para produção. Ela funciona através da implantação em ordem: o que bloquear antes de conectar sistemas reais, a saída que a frota precisa, como as sessões se autenticam no seu host git, as receitas de implantação em si, e o que verificar quando as sessões se comportam mal.

<h2 id="harden-your-deployment">
  Endurecimento de sua implantação
</h2>

Um runner auto-hospedado executa código arbitrário direcionado pelo modelo em sua infraestrutura em nome de todos que podem enviar uma sessão para seu ambiente. Isso é qualquer membro de sua organização Anthropic, e qualquer pessoa que possa iniciar uma sessão de canal [Claude Tag](https://claude.com/docs/claude-tag/overview) em um escopo que um Owner roteou para o ambiente. Trabalhe através de cada item antes de conectar um ambiente a sistemas de produção:

* **Contêineres efêmeros por sessão**: execute cada processo runner em um contêiner ou VM fresco que é destruído quando o processo sai, com `--capacity 1` e o padrão `--drain-grace-sec 0` para que cada contêiner sirva exatamente uma sessão. Em uma capacidade mais alta, ou com uma graça de drenagem positiva, um contêiner serve múltiplas sessões do mesmo [owner bloqueado](/docs/pt/self-hosted-environments#key-concepts); consulte [Ciclo de vida do Runner](/docs/pt/self-hosted-environments#runner-lifecycle). Não reutilize um sistema de arquivos entre reinicializações do runner, exceto na configuração deliberada de [checkout pré-aquecido](#reuse-a-pre-warmed-checkout), e nunca entre owners.
* **Sem credenciais amplas na imagem**: não inclua chaves SSH de longa duração, credenciais de provedor de nuvem ou tokens de acesso pessoal que concedem mais do que uma sessão precisa. Crie credenciais usadas durante uma sessão, como tokens de push ou API, por sessão a partir de seu [script wrapper](/docs/pt/self-hosted-environments-configuration#wrapper-scripts). Para o clone inicial, que acontece antes do wrapper ser executado, use um [hook de ciclo de vida `checkout`](/docs/pt/self-hosted-environments-configuration#checkout) ou [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy); consulte [Configurar git](#configure-git).
* **Mantenha o segredo do ambiente fora dos hosts que executam sessões**: o segredo do ambiente pode registrar runners e pegar qualquer sessão enfileirada no ambiente. Em uma frota fixa, ele vive em cada host runner, onde o código de qualquer sessão pode ler o arquivo secreto. Prefira [runners sob demanda](/docs/pt/self-hosted-environments-configuration#on-demand-runners), onde o segredo fica no host do orquestrador, que nunca executa código do usuário, e cada runner recebe uma ordem de trabalho de uso único que registra exatamente um runner. Em uma frota fixa, trate o arquivo environment-secret como legível por cada sessão e gire o segredo após qualquer suspeita de comprometimento de sessão.
* **Saída de rede padrão-negar**: restrinja o tráfego de saída do contêiner runner e sessão no seu próprio limite de rede em cada ambiente; [Saída padrão-negar](#default-deny-egress) cobre o que permitir e por quê.
* **IAM de host com privilégio mínimo**: a identidade de computação anexada ao host runner, como um perfil de instância ou conta de serviço de nó, deve conceder apenas o que o próprio runner precisa. As sessões devem obter suas próprias credenciais através de seu script wrapper em vez de herdar a do host.
* **Bloqueie o endpoint de metadados da nuvem das sessões**: manter as sessões fora da identidade do host requer bloquear seu acesso ao endpoint de metadados, e as políticas de saída no nível de sub-rede não interceptam o tráfego de metadados link-local, então bloqueie-o no próprio contêiner:

  * IMDSv2 com um limite de salto de um
  * GKE Workload Identity com ocultação de metadados
  * Uma negação explícita para `169.254.169.254` no namespace de rede do contêiner da sessão

  O bloqueio se aplica ao seu script wrapper e hooks de ciclo de vida também, já que compartilham o contêiner. Autentique qualquer troca de token com o [JWT da sessão](/docs/pt/self-hosted-environments-identity) contra seu próprio serviço de token sobre saída na lista de permissões, ou use uma identidade web baseada em arquivo, como IAM Roles for Service Accounts (IRSA) no Amazon EKS.
* **Isolamento de sistema de arquivos por runner**: cada processo runner obtém seu próprio diretório de trabalho que nenhum outro processo no host pode ler ou escrever. Faça `--hooks-dir`, o script wrapper, e o `~/.claude/` do host somente leitura para a sessão, seja construído na imagem ou montado como somente leitura.
* **Dispatch não tem controle de acesso por ambiente**: qualquer membro de sua organização Anthropic pode enviar uma sessão para qualquer um de seus ambientes. Se um Owner [rotear canais Claude Tag para o ambiente](/docs/pt/cloud-environments#set-the-environment-a-claude-tag-channel-uses), qualquer pessoa que a [configuração de acesso Claude Tag](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude) admita pode iniciar sessões de canal que são executadas lá. Por padrão, isso é qualquer pessoa no workspace Slack conectado, com ou sem uma conta Claude. Trate cada host runner como alcançável para execução de código por todos que podem enviar para ele, e coloque no host runner apenas dados e credenciais que todas essas pessoas têm permissão para ler. [`--lock-to-account`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) limita qual conta as sessões de um determinado host executam, mas não reduz quem pode enviar para o ambiente. Para tornar ambientes auto-hospedados a única opção de seletor, um [Owner](/docs/pt/cloud-environments#organization-shared-environments) pode ocultar ambientes hospedados pela Anthropic para toda a organização na página [**Cloud environments**](https://claude.ai/admin-settings/cloud-environments).
* **Enforce the repo-settings guard**: escolha o modo de guard com [`--confine-repo-settings`](/docs/pt/self-hosted-environments-reference#runner-cli-flags). O padrão `warn` registra uma violação e ainda assim gera a sessão, `enforce` recusa a sessão, e `off` desabilita a varredura. O runner verifica as configurações confirmadas de cada repositório para:

  * Uma concessão que se resolve fora do workspace da própria sessão: uma entrada `additionalDirectories`, uma regra `Edit`, `Write`, ou `NotebookEdit` em `permissions.allow`, ou uma entrada `sandbox.filesystem.allowWrite` ou `allowRead`
  * Um bloco `env` não vazio
  * Uma substituição de postura do operador, como `sandbox.enabled: false`

  O guard é executado independentemente de [`--trust-workspace`](/docs/pt/self-hosted-environments-reference#runner-cli-flags), e não cobre hooks de repositório, `.mcp.json`, ou regras Bash; consulte [Permissões e aprovação de ferramentas](/docs/pt/self-hosted-environments-configuration#permissions-and-tool-approval) para onde essas concessões pertencem.

<Note>
  A lista de permissões de IP de sua organização não cobre o tráfego do runner auto-hospedado por padrão. Não confie nela como um controle de rede para tráfego de runner ou sessão; aplique saída padrão-negar no seu próprio limite de rede em vez disso, e entre em contato com sua equipe de conta Anthropic se você quiser aplicação de lista de permissões de IP para sua organização.
</Note>

<h2 id="network-requirements">
  Requisitos de rede
</h2>

O runner e os filhos da sessão que ele gera fazem conexões de saída para os hosts abaixo. Restrinja a saída do contêiner de sessão a esses hosts e aos serviços internos específicos que as sessões precisam alcançar; [Saída padrão-negar](#default-deny-egress) cobre como e por quê.

Estes hosts são sempre necessários:

| Host                                                          | Porta                                      | Usado para                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------------------------------ | :----------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                                           | 443, HTTPS; WSS apenas para o conector SCM | Plano de controle do runner e streaming de sessão, inferência de modelo, sinalizadores de recursos, análise de produtos, buscas de chave [JWKS](/docs/pt/self-hosted-environments-identity), assinatura de commit, o proxy git quando `--use-anthropic-git-proxy` está definido, e o túnel [conector SCM](/docs/pt/self-hosted-environments-reference#scm-connector-flags) do orquestrador quando `--scm-connector-host` está definido |
| Seu host git, como `github.com` ou seu host GitHub Enterprise | 443 ou 22                                  | Clonagem e push de repositórios. Não necessário se o runner usar `--use-anthropic-git-proxy`, que roteia o tráfego git através de `api.anthropic.com`.                                                                                                                                                                                                                                                                       |

Se esses hosts são necessários depende de sua configuração:

| Host                                 | Porta | Quando necessário                                                                                                                                                                                                                                                                                                                       |
| :----------------------------------- | :---- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `downloads.claude.ai`                | 443   | No tempo de instalação, quando você instala ou atualiza Claude Code no host com o instalador nativo; o próprio script `install.sh` é servido de `claude.ai`. No tempo de execução da sessão, apenas quando as sessões instalam plugins do marketplace oficial da Anthropic.                                                             |
| `storage.googleapis.com`             | 443   | No tempo de execução da sessão, para as contagens de instalação de plugin e metadados mostrados em `/plugin`.                                                                                                                                                                                                                           |
| `code.claude.com` e `claude.com`     | 443   | Buscas de documentação pelo agente claude-code-guide integrado e solicitações WebFetch pré-aprovadas durante sessões. Bloquear esses hosts apenas afeta buscas de documentação.                                                                                                                                                         |
| `*.frame.claudeusercontent.com`      | 443   | Apenas quando a [ferramenta Artifact](/docs/pt/artifacts#availability) está disponível para sessões em sua organização; os padrões variam por plano, de acordo com a tabela de disponibilidade lá. Defina `CLAUDE_CODE_DISABLE_ARTIFACT=1` no runner para manter a ferramenta desabilitada independentemente da configuração da organização. |
| `registry.npmjs.org`                 | 443   | Quando uma sessão instala um plugin, tanto para buscar pacotes de plugin de origem npm quanto para instalar dependências Node.js de um plugin, ou quando um servidor MCP lançado por `npx` é executado                                                                                                                                  |
| `http-intake.logs.us5.datadoghq.com` | 443   | Métricas operacionais da Anthropic. Apenas quando `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1` está definido; desativado por padrão em ambientes auto-hospedados.                                                                                                                                                                                |
| `browser-intake-us5-datadoghq.com`   | 443   | Uploads de relatório de erro da Anthropic, enviados apenas quando [relatório de erro](/docs/pt/data-usage#telemetry-services) está habilitado para a conta da sessão. Suprimido por `DISABLE_ERROR_REPORTING=1` ou `DISABLE_TELEMETRY=1`.                                                                                                    |

O runner não alcança `statsig.anthropic.com`, `*.sentry.io`, `claude.ai`, ou `platform.claude.com`. Esses hosts aparecem em algumas listas de verificação de rede corporativa mais antigas, mas você não precisa colocá-los na lista de permissões para tráfego de runner ou sessão: as buscas de sinalizador de recurso vão para `api.anthropic.com`, e o runner se autentica com o segredo do ambiente em vez de OAuth interativo. Dois fluxos do lado do host alcançam `claude.ai`, então execute-os a partir de um host cuja saída permite, em vez de ampliar a saída do contêiner de sessão: o instalador de uma linha busca `install.sh` de `claude.ai` no tempo de instalação, e `claude auth login` interativo, que a [configuração guiada](/docs/pt/self-hosted-environments-quickstart#set-up-an-environment-and-runner), modo assinado do `doctor`, e [dispatch de CI](/docs/pt/self-hosted-environments-testing#authenticate-from-ci) usam, faz login através de `claude.ai`, `claude.com`, e `platform.claude.com`. `mcp-proxy.anthropic.com` também não é necessário: sessões auto-hospedadas não o usam, e a entrega dos conectores claude.ai de sua organização para sessões, quando habilitada para sua organização, roteia através de `api.anthropic.com`. Consulte [Servidores MCP](/docs/pt/self-hosted-environments-configuration#mcp-servers).

<h3 id="default-deny-egress">
  Saída padrão-negar
</h3>

Implante contêineres de runner e sessão em um segmento de rede ou namespace cuja saída é limitada aos hosts na [tabela de requisitos de rede](#network-requirements), seu host git, e os serviços internos específicos que as sessões precisam alcançar. O produto não pode verificar ou aplicar isso, então aplique-o no seu próprio limite de rede em cada ambiente. O código da sessão é direcionado pelo modelo e pode tentar conexões com hosts arbitrários; a saída padrão-negar no nível de rede limita onde essas tentativas podem chegar. Isso se aplica independentemente do modo de permissão: o conjunto de ferramentas pré-aprovado padrão já inclui `Bash`, então a saída do shell é executada sem um prompt mesmo sem [modo automático](/docs/pt/self-hosted-environments-configuration#permissions-and-tool-approval).

Para detalhes sobre qual telemetria cada sessão emite e como desativá-la, consulte [Telemetria](/docs/pt/self-hosted-environments-reference#telemetry).

<h3 id="authenticate-to-an-egress-proxy">
  Autenticar em um proxy de saída
</h3>

Alguns proxies de saída corporativos exigem um cabeçalho `Proxy-Authorization` em cada conexão. O token nesse cabeçalho geralmente gira muito rápido para ser escrito na URL do proxy que você define em `HTTPS_PROXY`. Defina `HTTPS_PROXY` ou `HTTP_PROXY` para a URL do seu proxy como de costume, então defina `--proxy-authorization-command` ou `--proxy-authorization-file` para dizer ao runner onde ler o valor do cabeçalho. Ambos os sinalizadores exigem Claude Code v2.1.238 ou posterior.

<h4 id="choose-where-the-proxy-authorization-value-comes-from">
  Escolha de onde o valor `Proxy-Authorization` vem
</h4>

Escolha o sinalizador que corresponde a como você produz o token `Proxy-Authorization`:

* **[`--proxy-authorization-command <command>`](/docs/pt/self-hosted-environments-reference#runner-cli-flags)**: escolha isso para um token que você gera sob demanda. O runner executa o comando shell e usa sua stdout aparada como o valor do cabeçalho, por exemplo `Bearer <token>`.
* **[`--proxy-authorization-file <path>`](/docs/pt/self-hosted-environments-reference#runner-cli-flags)**: escolha isso para um token que outro processo gira no lugar. O runner lê o arquivo e usa seu conteúdo aparado como o valor do cabeçalho.

<h4 id="configurations-the-runner-refuses-to-start-with">
  Configurações que o runner se recusa a iniciar com
</h4>

Cada sinalizador também tem uma forma de variável de ambiente, listada ao lado dele na [referência de sinalizadores CLI do runner](/docs/pt/self-hosted-environments-reference#runner-cli-flags). Antes do runner entrar em contato com seu proxy ou o plano de controle, ele verifica os sinalizadores e suas variáveis, e se recusa a iniciar em três casos:

* **Ambos os sinalizadores definidos**: um sinalizador mais a variável de ambiente do outro sinalizador conta como definir ambos.
* **Nenhuma URL de proxy**: nem `HTTPS_PROXY` nem `HTTP_PROXY` contém uma URL `http://` ou `https://`. O runner lê ambas as variáveis em maiúsculas ou minúsculas, e não consulta `ALL_PROXY`.
* **Qualquer sinalizador passado para o subcomando orquestrador**: `self-hosted-runner orchestrator` não aceita os sinalizadores ou suas variáveis de ambiente. Passe o sinalizador para cada runner que o orquestrador inicia em vez disso.

<h4 id="what-the-runner-changes-while-a-proxy-authorization-flag-is-set">
  O que o runner muda enquanto um sinalizador de autorização de proxy está definido
</h4>

Com qualquer sinalizador definido, o runner inicia um listener próprio e envia tráfego de proxy de si mesmo, seus hooks de ciclo de vida, e suas sessões através desse listener. O listener adiciona o cabeçalho `Proxy-Authorization` no caminho para seu proxy.

* **Listener**: o listener é um proxy direto em `127.0.0.1`. O runner inicia o listener antes de se registrar no plano de controle, e sai na inicialização se o listener não puder iniciar.
* **Variáveis de proxy**: o runner reescreve qual de `HTTPS_PROXY` e `HTTP_PROXY` você definiu para que aponte para o listener. Esse valor reescrito alcança o próprio runner, seus hooks de ciclo de vida, e cada sessão que ele executa.
* **Rotação de token**: um token girado entra em vigor sem uma reinicialização. Para cada conexão que o listener abre para seu proxy, o runner executa seu comando ou lê seu arquivo novamente e adiciona o resultado como o cabeçalho.
* **Ambiente da sessão**: uma sessão alcança seu proxy apenas através do listener. No ambiente de cada sessão, o runner remove `ALL_PROXY`, remove qualquer grafia de `HTTPS_PROXY` ou `HTTP_PROXY` que você não definiu, e fixa `NO_PROXY` ao próprio valor do runner.
* **Logs**: o runner nunca registra o valor do cabeçalho.

<h2 id="configure-git">
  Configurar git
</h2>

O runner gerencia checkouts de repositório, mas não configura identidade git ou credenciais por padrão. Você controla a imagem e o ambiente de processo do runner, então você controla a configuração git. Escolha uma de duas abordagens:

* **Deixe o runner configurar git**: inicie o runner com `--configure-git` para que ele escreva a mesma identidade e configuração de assinatura de commit que as sessões hospedadas pela Anthropic usam
* **Envie configuração git em sua imagem**: defina identidade e credenciais de push você mesmo, por exemplo para fazer commit sob sua própria identidade de bot

Pisos de versão Git no host runner: [`--configure-git`](#let-the-runner-configure-git) a assinatura de commit SSH requer Git 2.34 ou mais recente, [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) requer 2.32 ou mais recente, e retomar sessões de branches enviados por [`--push-outcome-on-release`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) requer 2.29 ou mais recente. Git 2.24 é suficiente se você omitir todos os três e gerenciar a identidade git você mesmo.

<h3 id="let-the-runner-configure-git">
  Deixe o runner configurar git
</h3>

Inicie o runner com `--configure-git`, ou defina `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`, para que ele escreva configuração git global na inicialização:

* `user.name = Claude` e `user.email = noreply@anthropic.com`, correspondendo às sessões hospedadas pela Anthropic
* Assinatura de commit e tag em formato SSH, roteada através de um shim gerenciado pelo runner que assina cada commit através do serviço de assinatura da Anthropic usando as credenciais da própria sessão. As assinaturas são verificáveis no GitHub contra a chave de assinatura SSH publicada da Anthropic.
* `push.negotiate = true`, para que git pergunte ao seu host git quais commits ele já possui antes de empacotar um push. Requer Claude Code v2.1.257 ou posterior.
* `core.hooksPath` apontando para um diretório de hooks gerenciado pelo runner. Seus hooks `commit-msg` e `prepare-commit-msg` adicionam um trailer `Co-authored-by:` para o criador da sessão a cada commit, construído a partir do email em [`CCR_SESSION_ACCOUNT_EMAIL`](/docs/pt/self-hosted-environments-configuration#wrapper-scripts) e omitido quando essa variável não está definida. Se sua imagem já define `core.hooksPath`, o runner deixa sua configuração no lugar, pula a instalação desses hooks e imprime um aviso `[runner:git]`.

A assinatura de commit requer git 2.34 ou mais recente; o runner verifica na inicialização e sai com um erro se seu git for mais antigo. Este sinalizador não configura credenciais de push, que você ainda fornece na imagem.

<h3 id="ship-git-config-in-your-image">
  Envie configuração git em sua imagem
</h3>

A identidade Git é necessária para qualquer commit. Defina-a em todo o sistema em seu Dockerfile para que a configuração se aplique independentemente de qual usuário o processo runner é executado como:

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

Sem uma identidade, `git commit` falha com `Please tell me who you are` e as sessões não podem fazer progresso. Você pode usar sua própria identidade de bot em vez disso; o runner não substitui esses valores.

Não cozinhe credenciais de push de longa duração ou amplamente escopo em uma imagem de runner compartilhada: uma credencial na imagem está disponível para cada sessão que a imagem executa, quem quer que a tenha iniciado. Em vez disso, crie um token de curta duração e escopo mínimo por sessão a partir de seu [script wrapper](/docs/pt/self-hosted-environments-configuration#wrapper-scripts), usando a identidade do criador da sessão decodificada do JWT da sessão. Emparelhe-o com um contêiner por sessão efêmero, que requer `--capacity 1`, para que nenhuma credencial sobreviva à sessão que a criou; consulte a [seção de endurecimento](#harden-your-deployment).

Se você deve configurar credenciais de push no nível de imagem, por exemplo para uma chave de implantação somente leitura, escopo-as tão bem quanto seu host git permite:

* Uma chave de implantação SSH limitada a um repositório com uma reescrita `url.<base>.insteadOf`
* Um `credential.helper` que retorna um token minimamente escopo
* `GIT_SSH_COMMAND` apontando para uma chave estreitamente escopo

Qualquer mecanismo que você configure deve funcionar sem um prompt, porque o clone integrado do runner e fetch desabilitam os prompts que git, SSH, e Git Credential Manager mostrariam de outra forma:

* O runner define `GIT_TERMINAL_PROMPT=0`, para que git não peça um nome de usuário ou senha.
* O runner executa SSH com `BatchMode=yes`, anexado ao seu `GIT_SSH_COMMAND` se você definir um, para que SSH não peça uma frase-passe ou confirmação de host.
* O runner define `GCM_INTERACTIVE=never`, para que Git Credential Manager não abra um diálogo de login.
* O runner limpa `core.askPass`, então se você usar um helper askpass, defina-o através da variável de ambiente `GIT_ASKPASS` em vez disso.

Se seu host git rejeitar a credencial, ou você não configurou uma, o runner tenta novamente algumas vezes e depois falha na preparação do repositório quando o repositório é aquele para o qual a sessão envia resultados. Para um repositório que a sessão apenas lê, [Troubleshooting](#troubleshooting) cobre quando o runner o pula em vez disso. O runner não passa essas configurações para o ambiente da sessão.

Se os diretórios de checkout são possuídos por um uid diferente do processo runner, git se recusa a operar neles; adicione `safe.directory`:

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

<h3 id="use-the-anthropic-git-proxy">
  Use o proxy git da Anthropic
</h3>

Inicie o runner com `--use-anthropic-git-proxy`, ou defina `CLAUDE_RUNNER_USE_GIT_PROXY=1`, para que ele clone através do proxy git da Anthropic, autenticado com o token de curta duração da própria sessão. Para sessões de usuário comum, o proxy usa o token OAuth do GitHub ou GitHub Enterprise armazenado para o criador da sessão; para sessões de bot e agente, ele usa o token de instalação do GitHub App de sua organização. De qualquer forma, a imagem do runner não precisa de nenhuma credencial git: sem chaves SSH, sem credential helper, sem `.netrc`. Este é o mesmo caminho de autenticação que os ambientes hospedados pela Anthropic usam.

O proxy requer `--capacity 1` porque a URL do proxy é por sessão, e git 2.32 ou mais recente porque git mais antigo ignora o mecanismo de configuração que o proxy usa para isolar sessões uma da outra. O runner se recusa a iniciar se qualquer requisito não for atendido. Como o proxy busca do lado da Anthropic, seu host git deve ser alcançável a partir da infraestrutura da Anthropic, o mesmo requisito que as sessões hospedadas pela Anthropic têm; para um host git que é apenas roteável dentro de sua rede, use um [hook de ciclo de vida `checkout`](/docs/pt/self-hosted-environments-configuration#checkout) em vez disso. Cada processo runner lida com uma sessão por vez, então execute mais réplicas para paralelismo. Quando o proxy está habilitado, `--git-host-rewrite` e `--git-ssh-rewrite` não têm efeito: a URL do proxy aponta para `api.anthropic.com`, não seu host git.

O runner também relata a aceitação à Anthropic quando se registra, imprimindo `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)` na inicialização. Relatar a aceitação requer Claude Code v2.1.267 ou posterior, e versões anteriores aceitam o sinalizador sem relatá-lo ou imprimir essa linha. Cada sessão em um runner aceito usa a git gerenciada pela Anthropic ou a URL do proxy por sessão. Quando uma sessão usa a URL do proxy por sessão, o runner registra uma linha `[runner:warn]` dizendo isso.

<h3 id="rewrite-git-urls-for-private-networks">
  Reescrever URLs git para redes privadas
</h3>

URLs de repositório chegam do plano de controle como HTTPS, com o nome do host do seu host git; para GitHub Enterprise, esse é o nome do host que você configurou para a [integração GitHub Enterprise](/docs/pt/github-enterprise-server) nas configurações de admin do Claude Code em claude.ai. Dois sinalizadores repetíveis reescrevem essas URLs antes do clone:

* `--git-host-rewrite <from>=<to>`: para DNS de horizonte dividido, onde a Anthropic alcança seu host git através de um nome do host externo, mas runners devem usar um interno
* `--git-ssh-rewrite <host>`: para hosts git que apenas aceitam SSH, reescrevendo `https://<host>/owner/repo` para `git@<host>:owner/repo`

A reescrita de host é executada primeiro, então liste o nome do host interno em `--git-ssh-rewrite` se você precisar de ambos. Para controle total sobre checkout, use um [hook de ciclo de vida `checkout`](/docs/pt/self-hosted-environments-configuration#checkout).

<h2 id="build-the-runner-image">
  Construir a imagem do runner
</h2>

A Anthropic não publica uma imagem de runner pré-construída. Construa a sua própria em torno do binário `claude`, camadas em qualquer toolchain que seus repositórios precisem: runtimes de linguagem, compiladores, gerenciadores de pacotes, e sidecars [MCP](/docs/pt/mcp).

As receitas abaixo usam `--capacity 4`, para que um contêiner sirva até quatro sessões simultâneas do mesmo owner bloqueado. Isso não fornece o isolamento de contêiner por sessão na [seção de endurecimento](#harden-your-deployment): antes de conectar um ambiente a sistemas de produção, execute as receitas em `--capacity 1` com um contêiner por sessão, ou use [runners sob demanda](/docs/pt/self-hosted-environments-configuration#on-demand-runners), que também mantêm o segredo do ambiente fora dos hosts que executam sessões.

Este Dockerfile é um ponto de partida mínimo:

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

Troque `linux-x64` por `linux-arm64` se seus nós forem ARM, ou por `linux-x64-musl` ou `linux-arm64-musl` em uma imagem baseada em musl, como Alpine; consulte [Configuração Alpine Linux](/docs/pt/setup#alpine-linux-and-musl-based-distributions) para os pacotes extras que imagens musl precisam. A URL é o local de lançamento padrão do Claude Code, para que você possa verificar o binário baixado contra o manifesto assinado do lançamento conforme descrito em [Integridade binária e assinatura de código](/docs/pt/setup#binary-integrity-and-code-signing). Construa a imagem com Claude Code versão 2.1.224 ou posterior, depois envie-a para seu registro e a referencie nas receitas abaixo:

```bash theme={null}
docker build --build-arg CLAUDE_CODE_VERSION=2.1.267 -t <your-registry>/claude-runner:latest .
```

<h2 id="size-cpu-and-memory-for-sessions">
  Dimensionar CPU e memória para sessões
</h2>

Dimensione o contêiner ou host de um runner para as sessões que ele executa em vez de para o próprio processo runner. O runner em si faz polling para trabalho, prepara o checkout de cada sessão, executa seus [hooks de ciclo de vida](/docs/pt/self-hosted-environments-configuration#lifecycle-hooks), e inicia e supervisiona os processos da sessão. A carga vem das sessões: cada uma é um processo Claude Code mais o que ela inicia, como builds, suites de teste, instalações de pacotes, e [servidores MCP](/docs/pt/mcp).

Para uma sessão, comece com os seguintes valores, declarados como requisições e limites do Kubernetes ou o equivalente de sua plataforma, e trate-os como um ponto de partida em vez de um requisito:

* **Memória**: uma requisição e um limite de 4 GiB cada, que atende ao mínimo de 4 GB nos [requisitos do sistema](/docs/pt/setup#system-requirements) do Claude Code. Mantenha os dois iguais para que o agendador contabilize a memória completa do contêiner. Quando o contêiner atinge seu limite de memória, o kernel mata processos dentro dele, o que pode encerrar uma sessão no meio da tarefa.
* **CPU**: uma requisição de 2 CPUs e um limite de 4 CPUs, para que uma sessão possa explodir acima da requisição durante builds. O kernel limita um contêiner em seu limite de CPU em vez de matar processos nele, então sessões no limite são executadas mais lentamente, mas continuam sendo executadas.

Em uma especificação de contêiner Kubernetes, defina esses valores iniciais com o seguinte bloco `resources`:

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

Builds e testes são geralmente a maior e mais variável parte da carga de uma sessão, então execute um build representativo de seu repositório, meça seu pico de CPU e memória, e aumente qualquer valor inicial que deixe sem espaço para o processo Claude Code no topo desse pico.

O runner usa `--capacity` para limitar quantas sessões ele executa de uma vez. Ele não divide CPU ou memória entre elas, então as sessões em um runner compartilham a CPU e memória do contêiner. Para limitar a participação de uma sessão, aplique limites a partir de seu [script wrapper](/docs/pt/self-hosted-environments-configuration#wrapper-scripts). O que dar a um contêiner, portanto, depende de quantas sessões ele serve de uma vez:

* **Uma sessão por runner**: dê a cada contêiner os valores de uma sessão. Use este dimensionamento em `--capacity 1`, que a [seção de endurecimento](#harden-your-deployment) recomenda, e para [runners sob demanda](/docs/pt/self-hosted-environments-configuration#on-demand-runners), onde você define os valores na carga de trabalho que seu [hook `spawn-runner`](/docs/pt/self-hosted-environments-configuration#the-spawn-runner-hook) submete, como um modelo de pod de um Kubernetes Job.
* **Várias sessões por runner**: em um `--capacity` acima de um, multiplique os valores de uma sessão pela capacidade, porque até muitas sessões podem ser executadas no contêiner ao mesmo tempo. As receitas [Kubernetes](#kubernetes) e [Docker Compose](#docker-compose) executam `--capacity 4` sem limites de CPU ou memória, então adicione limites dimensionados para a capacidade que você executa.

<h2 id="kubernetes">
  Kubernetes
</h2>

O runner serve `GET /healthz` na porta 8080 por padrão, configurável com `--health-port`, então as sondas Kubernetes funcionam sem configuração extra. O endpoint retorna `200` sempre que o processo está vivo, então as sondas abaixo detectam um processo morto, não um preso; para pegar um runner que parou de fazer polling, alerte na série `last_poll_age_seconds` de [`/metrics`](/docs/pt/self-hosted-environments-reference#prometheus-metrics). O Deployment abaixo monta o segredo do ambiente a partir de um Secret Kubernetes, aponta as sondas de vivacidade e prontidão para `/healthz`, e define um período de graça de encerramento de 90 segundos. Consulte [Tempo de encerramento](#shutdown-timing) para entender por que o período de graça importa.

O manifesto não define `resources` de CPU ou memória no contêiner runner. Adicione um bloco dimensionado para a capacidade que você executa, conforme [Dimensionar CPU e memória para sessões](#size-cpu-and-memory-for-sessions) descreve.

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

O Deployment acima vive em um namespace `claude-runners`. Crie o namespace primeiro:

```bash theme={null}
kubectl create namespace claude-runners
```

Crie o Secret de suporte a partir de um arquivo local contendo o valor que você copiou na etapa [**Copy environment key**](/docs/pt/self-hosted-environments-quickstart#set-up-an-environment-and-runner) da UI de admin, para que o segredo nunca apareça no histórico do shell. Execute `(umask 077 && cat > ./environment-secret)`, cole o segredo, pressione Enter, depois Ctrl-D. Depois crie o Secret e delete o arquivo:

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

<h2 id="docker-compose">
  Docker Compose
</h2>

O serviço Compose abaixo reinicia o runner sempre que ele sai, o que cobre tanto crashes quanto a saída normal após drenagem. Uma política de reinicialização Docker reinicia o mesmo contêiner com sua camada gravável intacta, então o runner volta em um sistema de arquivos reutilizado em vez do fresco que a [postura de endurecimento](#harden-your-deployment) recomenda; use esta receita para avaliação, e para produção recrie o contêiner por execução ou use um orquestrador que faça.

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

<h2 id="shutdown-timing">
  Tempo de encerramento
</h2>

Em `SIGTERM`, o runner para de aceitar novo trabalho e, a menos que você defina [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), aguarda até `--drain-wait-sec`, zero por padrão, para que os turnos em andamento sejam concluídos, encerra a árvore de processos de cada sessão e executa o hook de ciclo de vida [`post-session`](/docs/pt/self-hosted-environments-configuration#post-session). Essa árvore de processos inclui comandos que Claude ainda estava executando na sessão.

O caminho de drenagem completo precisa de até `--session-stop-grace-sec` + `--drain-wait-sec` + `--post-session-hook-timeout-sec`, mais 15 segundos de sobrecarga fixa para limpeza de processos, mais 30 segundos adicionais quando [`--push-outcome-on-release`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) está definido. Isso é 80 segundos nos padrões, e o runner registra o total na inicialização. As sessões são drenadas em paralelo sob esse orçamento único, portanto o total não cresce com `--capacity`.

No padrão `--drain-wait-sec 0`, uma reinicialização contínua interrompe os turnos em andamento; cada sessão é retomada em outro runner, perdendo trabalho não enviado conforme descrito em [Problemas conhecidos](#additional-limitations). Defina `--drain-wait-sec` e aumente o período de carência para corresponder, para permitir que os turnos sejam concluídos primeiro.

Durante todo esse caminho, o runner continua enviando heartbeat para o plano de controle com capacidade zero, portanto a concessão de sessão não expira e não é recolocada na fila para outro runner enquanto o hook `post-session` ainda está escrevendo trabalho não confirmado. O heartbeat para logo antes do runner se desregistrar.

Dê ao runner pelo menos o total que ele registra na inicialização antes do host pará-lo. Onde você define isso depende de como seus hosts param:

* **Com um período de carência `SIGTERM`**: defina `terminationGracePeriodSeconds` no Kubernetes, `stop_grace_period` no Docker Compose, ou o equivalente do seu orquestrador para pelo menos esse total. O padrão do Kubernetes de 30 segundos é mais curto que o caminho de drenagem do runner, portanto o Kubernetes para o pod antes do runner terminar a drenagem.
* **Com [`--retire-at`](/docs/pt/self-hosted-environments-reference#runner-cli-flags)**: dimensione a margem entre o tempo de aposentadoria e o tempo de parada do host para cobrir turnos típicos, mais a retenção de tarefa de fundo que [Ciclo de vida do Runner](/docs/pt/self-hosted-environments#runner-lifecycle) descreve, mais esse mesmo total. Calcule o tempo de aposentadoria em cada inicialização, por exemplo `date +%s` mais o tempo de vida pretendido do runner.
* **Com [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)**: adicione duas partes adicionais ao total do caminho de drenagem. A primeira é os minutos que você configura. A segunda é a carência pós-lançamento que [Adiar a drenagem após o primeiro sinal](#defer-the-drain-past-the-first-signal) descreve, 75 segundos nos padrões. Com a flag definida, o runner também imprime a figura combinada na inicialização, após o total do caminho de drenagem.

<h3 id="defer-the-drain-past-the-first-signal">
  Adiar a drenagem após o primeiro sinal
</h3>

Defina [`--defer-shutdown-max-min <n>`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) se você quiser que um runner que está reiniciando continue servindo as sessões que ele mantém por até `n` minutos, em vez de drenálas no primeiro sinal. No primeiro `SIGTERM` ou `SIGINT`, o runner para de aceitar novo trabalho e continua servindo as sessões que ele mantém. Ele continua sondando para que o plano de controle não recoloque essas sessões na fila. Requer Claude Code v2.1.238 ou posterior.

<h4 id="what-happens-to-the-sessions-the-runner-holds-after-the-first-signal">
  O que acontece com as sessões que o runner mantém após o primeiro sinal
</h4>

Nos primeiros dois estágios que seguem o sinal, o runner libera sessões, e uma sessão liberada é retomada em um runner novo quando seu usuário envia sua próxima mensagem. Contando a partir do primeiro sinal, o runner passa por três estágios:

* **Pelos primeiros `n` minutos**: o runner serve suas sessões normalmente e continua aplicando `--startup-timeout-min` e `--kill-session-after-min`. Se você também definir [`--release-idle-session-min`](/docs/pt/self-hosted-environments-reference#runner-cli-flags), o runner libera qualquer sessão cujo usuário tenha estado ocioso por esse tempo; sem isso, as sessões ociosas permanecem no runner.
* **Quando os `n` minutos se esgotam**: o runner libera todas as sessões que ainda mantém, ociosas ou não. O runner aguarda o término do turno de uma sessão no meio do turno e até 60 segundos adicionais para as tarefas de fundo de um turno, antes de liberar essa sessão.
* **Quando a carência pós-lançamento se esgota**: o runner drena todas as sessões que ainda mantém, e o plano de controle recoloca cada sessão drenada na fila para outro runner imediatamente. A carência pós-lançamento começa quando os `n` minutos se esgotam e é 75 segundos nos padrões. Se você definir `--drain-wait-sec` acima de 60 segundos, a carência pós-lançamento será `--drain-wait-sec` mais 15 segundos.

Em qualquer estágio, o runner sai com 0 assim que não mantém mais sessões. Um segundo sinal encurta os estágios: o runner drena imediatamente, como faz no primeiro sinal sem `--defer-shutdown-max-min`. Uma vez que uma drenagem está em andamento, o próximo sinal força a saída do runner. Isso vale se um segundo sinal ou a carência pós-lançamento se esgotando iniciou a drenagem.

<h4 id="size-the-stop-timeout">
  Dimensionar o tempo limite de parada
</h4>

Dê ao tempo limite de parada do seu host pelo menos a soma de três partes: os `n` minutos que você configura, a carência pós-lançamento e o caminho de drenagem completo que [Tempo de encerramento](#shutdown-timing) descreve. Com as configurações padrão, a carência pós-lançamento é 75 segundos e o caminho de drenagem é 80 segundos, portanto permita `n` minutos mais 155 segundos. O runner imprime essa soma na inicialização sempre que `--defer-shutdown-max-min` está definido.

Se o tempo limite de parada se esgotar antes do runner terminar, o host mata o runner. As sessões que ele ainda mantém não recebem nenhum hook `post-session`. O runner não se desregistra, e o plano de controle recoloca as sessões na fila cerca de um minuto depois. Se você não puder dar ao tempo limite de parada essa soma, deixe `--defer-shutdown-max-min` indefinido para que o runner drene no primeiro sinal.

<h3 id="what-reaches-a-running-post-session-hook">
  O que atinge um hook post-session em execução
</h3>

O hook `post-session` e o filho da sessão Claude cada um executam em seu próprio grupo de processos POSIX, separado do runner, portanto os mecanismos de parada os alcançam de forma diferente:

* **Um `SIGTERM` enquanto o runner já está drenando**: força a saída do runner imediatamente, pulando o que resta do caminho de drenagem. Sem [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), esse é o segundo `SIGTERM` que o runner recebe. Nada sinaliza um hook `post-session` em execução no meio, portanto em um host simples onde um processo init adota órfãos, ele termina por conta própria, mas sem supervisão: seu orçamento de tempo limite não se aplica mais, e uma escrita no pipe de log fechado pode matá-lo com `SIGPIPE`, portanto um hook que precisa sobreviver a uma saída forçada lá deve redirecionar sua própria saída para um arquivo. Nas receitas de contêiner nesta página, o runner é o PID 1 do contêiner e sua saída encerra o contêiner, e sob o padrão `KillMode=control-group` do systemd, a morte em nível de cgroup atinge o hook também, conforme a entrada **Cgroup-wide kills** descreve; em ambos, trate uma saída forçada como fatal para o hook e confie no período de carência.
* **Sinais em nível de grupo de processos**, como `kill -- -<pid>` em um script wrapper, controle de trabalho de shell ou um watchdog em nível de grupo: alcançam o runner e um subprocesso de hook `checkout` no meio, que permanece anexado ao grupo deliberadamente, mas não um hook `post-session` em execução no meio ou o filho da sessão.
* **Mortes em nível de cgroup**, como o padrão `KillMode=control-group` do systemd ou o `SIGKILL` que o Kubernetes entrega para todo o contêiner quando `terminationGracePeriodSeconds` expira: alcançam tudo, incluindo o hook. O isolamento de grupo de processos não protege contra esses, razão pela qual o período de carência deve cobrir o caminho de drenagem completo.
* **O tempo limite próprio do hook**: quando um hook excede `--post-session-hook-timeout-sec`, o runner envia `SIGTERM` para todo o grupo de processos do hook, depois `SIGKILL` dois segundos depois, portanto um worker que o hook bifurcou, como tar, rsync ou git, termina com o shell wrapper em vez de sobreviver como um órfão. A supervisão do runner termina uma vez que o stdio do hook fecha: um worker que redirecionou sua própria saída para um arquivo e sobrevive ao estágio `SIGTERM` está além do alcance do runner.

Quando a drenagem começa, e novamente em uma saída forçada, o runner registra quantos hooks `post-session` ainda estão em execução, portanto você pode distinguir uma drenagem silenciosa de uma que está no meio de um snapshot.

<h2 id="keep-the-base-directory-and-capacity-identical-across-runners">
  Mantenha o diretório base e a capacidade idênticos entre runners
</h2>

Se um runner morre no meio de uma sessão, o servidor refileira a sessão e outro runner no ambiente a pega. Esse runner deriva o caminho de checkout de seu próprio `--base-dir` e `--capacity`: `--capacity 1` faz checkout diretamente sob `--base-dir`, e um `--capacity` acima de `1` usa worktrees por sessão em vez disso. Quando runners no mesmo ambiente usam valores diferentes para qualquer sinalizador, o diretório de trabalho da sessão retomada muda, e caminhos absolutos que o agente registrou anteriormente, em edições, chamadas de ferramentas, ou suas próprias notas, apontam para um local que não existe mais.

Use o mesmo `--base-dir` e `--capacity` em cada runner em um ambiente, e não use um valor por host, como um ID de instância ou nome do host.

O diretório base padrão é `/workspace`, com a exceção que a linha de referência [`--base-dir`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) registra. O runner precisa de acesso de escrita a ele. Na inicialização, antes de se registrar, o runner cria o diretório e confirma que pode escrever nele, e sai com `cannot create or write to base directory` quando não pode. Um runner iniciado como root cria o `/workspace` padrão em si. Para um runner não-root, crie o diretório e dê ao usuário do runner a propriedade antes de iniciar o runner, ou aponte `--base-dir` para um diretório que esse usuário já possui.

<h2 id="reuse-a-pre-warmed-checkout">
  Reuse a pre-warmed checkout
</h2>

Para repositórios grandes, o clone pode dominar a inicialização da sessão. Em `--capacity 1` sem um [hook `checkout`](/docs/pt/self-hosted-environments-configuration#checkout), o runner mantém um clone canônico por repositório em `<base-dir>/<repo-owner>/<repo>` e o reutiliza entre sessões: ele busca a ref solicitada, destaca `HEAD`, e redefine duramente para ela, o que é quase instantâneo quando pouco mudou. Para pular o clone frio, forneça o clone de uma de duas maneiras:

* **Clone na imagem**: construa o clone em sua imagem de runner naquele caminho. Cada contêiner fresco então começa com o clone quente sem reutilizar um disco.
* **Clone em um volume persistente**: em runners que você pré-bloqueia para a conta de um usuário com [`--lock-to-account`](/docs/pt/self-hosted-environments-reference#runner-cli-flags), aponte `--base-dir` para um volume persistente, para que o disco apenas sirva essa conta. Um runner pré-bloqueado nunca pega sessões de canal Claude Tag, então essa opção não se aplica a runners que as servem.

O que o caminho de reutilização faz e não garante:

* **Qualquer forma de clone funciona**: um clone completo, raso, ou de um único branch no caminho é usado como está. O runner nunca passa `--depth` ao buscar em um clone existente, então um pré-aquecimento completo mantém seu histórico completo e um raso permanece raso. `CLAUDE_RUNNER_FETCH_DEPTH` (`full`, `0`, ou um número; padrão 50) controla apenas o clone frio que o runner faz quando nenhum clone existe ainda.
* **Mudanças rastreadas redefinem, arquivos não rastreados persistem**: cada sessão começa a partir de uma redefinição dura que limpa as modificações rastreadas da sessão anterior, mas o runner nunca executa `git clean`, então arquivos não rastreados das sessões anteriores do owner bloqueado permanecem na árvore.
* **Diretórios por sessão também persistem**: ao lado do checkout, o runner cria entradas por sessão sob `<base-dir>/_sessions/` para cada sessão que executa. O diretório de configuração Claude da sessão contém uma cópia local da transcrição da conversa. Ao lado dele ficam os arquivos carregados da sessão, quando a sessão tem algum. O diretório da sessão também fica lá: ele contém quaisquer worktrees por sessão e checkouts do hook `checkout` enquanto a sessão é executada, e mantém tudo mais que Claude escreveu nele.

  Por padrão, o runner deixa esses em vigor quando a sessão termina, então em um disco que sobrevive ao processo do runner eles se acumulam. Cada sessão é executada como o próprio usuário do runner, então qualquer sessão posterior que o disco servir pode lê-los. Se você manter um `--base-dir` persistente, dimensione o volume para esse crescimento. O mesmo se aplica a qualquer configuração que reinicie o runner no mesmo sistema de arquivos, incluindo a [receita Docker Compose](#docker-compose).
* **Com `--remove-session-state`, diretórios por sessão não persistem**: inicie o runner com [`--remove-session-state`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) para que ele delete os diretórios por sessão de cada sessão conforme a sessão termina. A exclusão é do melhor esforço: os diretórios permanecem quando o runner é morto antes de sua limpeza ser executada. O clone canônico e arquivos que uma sessão escreveu em outro lugar no host, como o diretório temporário, permanecem independentemente.
* **Com o proxy git, a redefinição se torna um checkout**: com [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), o runner sanitiza o `.git/` do clone antes de cada sessão, mantendo o armazenamento de objetos, refs, e estado raso, mas deletando o índice, então cada sessão paga um checkout de árvore de trabalho completo em vez de uma redefinição quase instantânea; ainda nunca re-clona. Pré-aquecimentos de submódulo não são suportados sob o proxy.
* **Clones longos não precisam de workaround**: o runner limita cada operação git com um watchdog de 120 segundos sem progresso e um limite duro de 30 minutos, não um tempo limite fixo, então um clone frio lento que continua relatando progresso é concluído.

<h2 id="pin-the-version">
  Pin the version
</h2>

Cada processo filho Claude Code da sessão executa o próprio binário do runner, e o runner desativa auto-update dentro das sessões que gera, então cada sessão executa a versão que você instalou no host ou construiu na imagem. Uma atualização no nível do host entra em vigor na próxima vez que o runner inicia.

* **Para manter uma frota em uma versão**: construa a imagem com uma versão fixada, ou em um host nu instale uma versão específica e [desabilite auto-updates](/docs/pt/setup#disable-auto-updates)
* **Para atualizar**: instale a versão mais recente ou reconstrua a imagem, depois reinicie os runners
* **Plugins**: marketplaces de plugin também não auto-atualizam; defina `FORCE_AUTOUPDATE_PLUGINS=1` no ambiente do runner para deixar plugins auto-atualizarem enquanto o binário permanece fixado

<h2 id="scale-the-fleet">
  Scale the fleet
</h2>

Seu orquestrador decide quando adicionar ou remover runners. Por causa do [bloqueio de um owner por runner](/docs/pt/self-hosted-environments#runner-lifecycle), a contagem mínima de réplicas é o número de usuários e agentes Claude Tag que você espera estar ativos simultaneamente; `--capacity` controla paralelismo dentro das sessões de um owner, não entre owners.

Duas abordagens de dimensionamento estão disponíveis:

* **Frota fixa**: execute um conjunto estático de réplicas de runner e dimensione nas [métricas Prometheus](/docs/pt/self-hosted-environments-reference#prometheus-metrics) que cada runner serve
* **Runners sob demanda**: execute o subcomando `claude self-hosted-runner orchestrator`, que faz polling na Anthropic para sessões que estão enfileiradas sem runner disponível e invoca seu hook `spawn-runner` para iniciar um por sessão. Consulte [Runners sob demanda](/docs/pt/self-hosted-environments-configuration#on-demand-runners).

<h2 id="known-issues-and-limitations">
  Problemas conhecidos e limitações
</h2>

As seguintes são as limitações nesta versão, com workarounds onde um existe.

<h3 id="connector-traffic-leaves-your-network">
  Tráfego de conector sai de sua rede
</h3>

A Anthropic chama ferramentas de conector a partir de sua própria infraestrutura em vez de seu runner. Ferramentas de conector são os conectores claude.ai, como GitHub, Slack e Linear. Quando Claude usa um conector em uma sessão auto-hospedada, esse tráfego passa por `api.anthropic.com` em vez de originar dentro do seu limite de rede.

Para manter um conector fora de sessões auto-hospedadas, filtre-o com as [configurações de política `allowedMcpServers` e `deniedMcpServers`](/docs/pt/managed-mcp#policy-based-control-with-allowlists-and-denylists). Claude Code aplica essas configurações aos conectores que a Anthropic entrega bem como aos servidores que você configura a partir do host do runner e aos servidores que os usuários adicionam, então se você implantar uma lista de permissões para outros servidores, Claude Code bloqueia conectores entregues também. Para manter conectores disponíveis ao lado de uma lista de permissões baseada em URL, adicione entradas que correspondam aos caminhos de proxy da Anthropic para conectores entregues:

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

Se o tráfego de ferramentas deve permanecer dentro de sua rede, execute as ferramentas equivalentes como servidores MCP locais na imagem do runner em vez disso. Consulte [Servidores MCP](/docs/pt/self-hosted-environments-configuration#mcp-servers).

<h3 id="some-sessions-don’t-count-as-idle">
  Algumas sessões não contam como ociosas
</h3>

Uma sessão mantendo uma tarefa de fundo que nunca termina não conta como ociosa, então `--release-idle-session-min` não liberará o slot dessa sessão. Uma sessão que está aguardando uma aprovação solicitada de dentro de uma chamada de ferramenta em execução também não conta como ociosa. Sempre defina `--kill-session-after-min` ao lado dela como um backstop duro para que nenhuma sessão possa manter um slot indefinidamente.

`--kill-session-after-min` é um backstop para sessões descontroladas. Em um runner na v2.1.260 ou posterior, uma sessão que atinge o limite não é encerrada imediatamente. O runner oferece uma janela de graça, 15 minutos por padrão, que você pode alterar com [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](/docs/pt/self-hosted-environments-reference#environment-variable-only-settings):

* Se a sessão está aguardando seu usuário, o runner a libera. Se sua vez terminou e ela mantém apenas tarefas de fundo, o runner aguarda até 60 segundos para que essas tarefas terminem e então a libera. A sessão retoma quando seu usuário envia sua próxima mensagem.
* Se uma vez ainda está em execução, o runner aguarda a vez terminar, ou para a sessão aguardar seu usuário em seguida, e então a libera.
* Se a sessão ainda estiver no runner quando a janela de graça terminar, o runner a encerra, e qualquer trabalho de uma vez em execução é perdido. Uma vez aguardando uma aprovação solicitada de dentro de uma chamada de ferramenta em execução é uma maneira de uma sessão ultrapassar a janela.

Uma sessão liberada retoma de um clone fresco, então o trabalho que ela não tinha enviado se foi de qualquer forma; consulte [Sessões retomadas perdem trabalho não enviado](#additional-limitations). Antes da v2.1.260, o runner encerrava cada sessão no limite, após aguardar no máximo a janela de graça para uma vez em execução terminar.

Defina o sinalizador acima de sua sessão mais longa esperada, como `--kill-session-after-min 480` para 8 horas. Para liberar slots de conversas que ficam ociosas, use `--release-idle-session-min` em vez disso.

<h3 id="additional-limitations">
  Limitações adicionais
</h3>

* **Sessões retomadas perdem trabalho não enviado**: quando uma sessão é liberada ou seu runner é reiniciado, e o usuário envia outra mensagem, a sessão retoma em um runner fresco que clona o repositório novamente de seu branch inicial, então o trabalho que a sessão não tinha enviado se foi. Defina [`--push-outcome-on-release`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) para que o runner faça um push de melhor esforço dos branches de resultado da sessão antes de liberá-la, para que a sessão retomada comece a partir desses commits em vez disso; isso preserva trabalho confirmado, não uma árvore de trabalho suja. Antes de habilitá-lo, restrinja quem pode fazer push para refs `claude/*` no remoto de origem, por exemplo com um ruleset de branch: na retomada, o runner busca o branch previamente enviado sem verificar quem o enviou, então qualquer pessoa com acesso de push para essas refs pode colocar conteúdo no workspace retomado. O runner também descarta configuração por sessão na retomada, significando o diretório de configuração Claude da sessão e qualquer estado de shell que a sessão escreveu; `--push-outcome-on-release` não cobre esses.
* **Repositórios privados não podem ser adicionados no meio da sessão**: um repositório adicionado a uma sessão após ela ter iniciado não é clonado com credenciais em um runner auto-hospedado, então a adição falha. Selecione cada repositório que a sessão precisa quando você a cria.
* **Alguns conectores não aparecem em sessões auto-hospedadas**: um conector que você ainda não conectou nas Configurações do claude.ai não está listado em uma sessão auto-hospedada, e a sessão não o solicitará para conectar. Conecte-o nas Configurações primeiro, depois inicie uma sessão fresca. Adicionar um conector a uma sessão já em execução também não torna suas ferramentas disponíveis para Claude; inicie uma sessão fresca para pegar um conector recém-adicionado.

<h3 id="report-an-issue">
  Relatar um problema
</h3>

Para problemas com ambientes auto-hospedados, entre em contato com sua equipe de conta Anthropic.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Para diagnóstico orientado, execute o subcomando doctor no host do runner. O subcomando doctor inicia uma sessão interativa do Claude Code com os logs e o estado do runner anexados. Faça login com `claude auth login` nesse host primeiro para que a sessão possa consultar seu ambiente, seus runners e suas sessões enfileiradas. Sem esse login, por exemplo quando o host se autentica com uma chave de API, fica limitado ao endpoint de saúde local, métricas e ao log do runner, e lê o log apenas se você iniciou o runner com `--log-file`.

```bash theme={null}
claude self-hosted-runner doctor
```

Problemas comuns:

* **Runner não aparece no ambiente**: confirme que o host pode alcançar `api.anthropic.com` via HTTPS, o segredo do ambiente está atual e o relógio do host está dentro de cinco minutos da hora real; desvios maiores causam falha na autenticação. O runner registra `[runner:fatal]` com o motivo da rejeição em caso de falha de autenticação.
* **Runner sai na inicialização com `cannot create or write to base directory`**: o runner não consegue criar ou escrever em `--base-dir`, que padrão é `/workspace`. Corrija a propriedade do diretório ou aponte `--base-dir` para um caminho gravável, conforme descrito em [Keep the base directory and capacity identical across runners](#keep-the-base-directory-and-capacity-identical-across-runners). Se o runner registrar `[runner:fatal]` dizendo que a verificação do diretório base expirou, o diretório está em uma montagem NFS ou CSI travada. Verifique a saúde da montagem em vez de permissões. O runner imprime ambas essas falhas de inicialização para stderr antes de abrir `--log-file`, então procure por elas no terminal ou nos logs do contêiner da sua plataforma em vez do arquivo de log. Antes da v2.1.225, o runner não verificava o diretório base na inicialização, e essa configuração incorreta falhava nas sessões após a coleta.
* **Sessions stay queued**: cada runner online pode estar bloqueado para um proprietário diferente. Verifique a métrica `claude_code_self_hosted_runner_locked_account` [metric](/docs/pt/self-hosted-environments-reference#prometheus-metrics) de cada runner ou o campo `locked_account` de sua linha de log `[runner:health]` para ver quem a mantém. Ambos mostram o email do proprietário apenas depois que o runner recebeu um token de sessão com uma reivindicação `act.email`, que as sessões de um agente Claude Tag nunca fazem. Sem a reivindicação, o runner não emite nenhuma série `locked_account` e registra `locked_account=yes`, o que informa que o runner está bloqueado, mas não para qual proprietário. Adicione réplicas ou aguarde um runner existente drenar e reiniciar. Se o ambiente usar runners sob demanda, verifique o orquestrador; consulte [On-demand runners](/docs/pt/self-hosted-environments-configuration#on-demand-runners).
* **Sessions fail immediately after pickup**: abra a sessão em claude.ai/code para ver o erro. As causas mais comuns são [git credentials](#configure-git) ausentes na imagem do runner e ferramentas de compilação que não estão instaladas. Um diretório base não gravável interrompe o runner na inicialização em vez de falhar nas sessões. Consulte a entrada **Runner sai na inicialização com `cannot create or write to base directory`** nesta lista.
* **Sessions can't reach the network through an authenticating egress proxy**: quando a fonte que você definiu com [`--proxy-authorization-command` ou `--proxy-authorization-file`](#authenticate-to-an-egress-proxy) falha, expira após 30 segundos ou produz um valor vazio, o runner responde essa conexão com `502 Bad Gateway` e registra o motivo. O runner redige o stderr do comando nesse log e nunca registra o valor do cabeçalho. Com `--proxy-authorization-command`, execute o comando você mesmo no host para confirmar que ele imprime o valor do cabeçalho inteiro em stdout. Se o runner sair na inicialização com `could not start the proxy-authorization listener`, ele não conseguiu abrir seu listener de loopback.
* **Runner logs `Poll failed` lines containing `rejecting the malformed poll response`**: o runner recebeu uma resposta de work-poll cujo corpo não é o JSON esperado da fila, na maioria das vezes porque algo entre o runner e `api.anthropic.com`, como um proxy interceptador ou um portal cativo, respondeu com sua própria página. O runner rejeita a resposta, a conta sob o tipo `transport` da métrica `claude_code_self_hosted_runner_poll_errors_total` [metric](/docs/pt/self-hosted-environments-reference#prometheus-metrics), e tenta novamente no cronograma de falha de pesquisa descrito em [Session lifecycle](/docs/pt/self-hosted-environments#session-lifecycle). O runner continua servindo suas sessões ativas. Configure o proxy para passar respostas de `api.anthropic.com` inalteradas. Antes da v2.1.246, o runner lia tal resposta como uma fila de trabalho vazia, o que poderia encerrar suas sessões ativas ou fazer com que saísse.
* **A session's branch no longer exists on the remote**: para uma fonte git que a sessão apenas lê, o runner pula essa fonte e continua nas restantes. Para a fonte para a qual a sessão envia resultados, uma ramificação excluída, normalmente porque foi mesclada e auto-excluída, falha na sessão com um erro nomeando o repositório e a ramificação e pedindo que você restaure a ramificação e tente novamente. O runner falha na sessão com o mesmo erro quando pular deixaria sem nenhum repositório. Antes da v2.1.228, tal sessão começava em um diretório vazio.
* **A session starts without one of its repositories**: em um runner sem um [checkout hook](/docs/pt/self-hosted-environments-configuration#checkout), o host git pode recusar a verificação de acesso do runner para um repositório que a sessão apenas lê. O runner então pula esse repositório, registra uma linha `[runner:warn] could not access context source` nomeando a recusa, e inicia a sessão nos restantes.

  O runner pula apenas uma recusa clara: o host responde que o repositório não foi encontrado, git não encontra credenciais para o host, ou a autenticação falha. Uma falha de rede, um timeout ou um HTTP `403` ainda falha no início da sessão, assim como uma recusa para um repositório para o qual a sessão envia resultados. O runner ainda falha em uma sessão que pular deixaria sem nenhum repositório. Com [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), o runner pula apenas um repositório que o proxy git em si nega.

  A verificação de acesso é executada novamente cada vez que a sessão é iniciada em um runner, portanto, uma vez que a identidade git do runner tenha acesso de leitura, o próximo início clona o repositório. Antes da v2.1.274, cada uma dessas recusas falhava no início da sessão.
* **Sessions take minutes to start**: o clone inicial geralmente domina. Observe a métrica `claude_code_self_hosted_runner_session_init_duration_seconds` [metric](/docs/pt/self-hosted-environments-reference#prometheus-metrics) para confirmar e corte o clone com um [pre-warmed checkout](#reuse-a-pre-warmed-checkout) ou um `CLAUDE_RUNNER_FETCH_DEPTH` menor.
* **Turns fail with a 401**: cada sessão autentica chamadas de modelo com o token de curta duração [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/pt/self-hosted-environments-configuration#wrapper-scripts) que o runner busca da Anthropic e rotaciona sobre o stdin da sessão. Quando uma volta termina com um 401 ou 403 da API do modelo, o runner busca um token novo e o passa para a sessão. A volta com falha não é retentada.

  Quando uma busca falha, o runner registra uma linha `inference_token refresh failed` que diz quando tentará novamente, e continua tentando novamente enquanto a sessão estiver em execução.

  Se cada chamada começar a falhar cerca de 30 minutos em uma sessão, um script wrapper provavelmente sevrou o stdin da sessão, então as rotações de token não conseguem alcançá-lo; consulte [Keep stdin and file descriptor 3 attached](/docs/pt/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached).

  Antes da v2.1.274, o runner parou de tentar novamente uma busca com falha após algumas tentativas e aguardou a próxima agendada. Uma volta com falha não acionou uma busca, então cada volta falhou com um 401 até a próxima busca agendada.
* **Pod is killed mid-drain**: aumente `terminationGracePeriodSeconds` para pelo menos o valor que o runner registra na inicialização. Consulte [Shutdown timing](#shutdown-timing).

Depois que o logging é inicializado, o runner escreve seu log de ciclo de vida, incluindo linhas `[runner:fatal]`, para stdout, e saída de depuração para stderr, tudo como linhas de texto simples em vez de JSON. As falhas de inicialização descritas nas entradas de troubleshooting acima são impressas em stderr antes desse ponto. Capture ambos os fluxos com `--log-file`, que também permite que `self-hosted-runner doctor` os acompanhe, ou com a coleta de logs da sua plataforma.

O processo filho de cada sessão escreve um log de depuração separado. Em caso de falha, o runner exibe a cauda do log junto com a sessão em claude.ai/code. A menos que você tenha iniciado o runner com [`--remove-session-state`](/docs/pt/self-hosted-environments-reference#runner-cli-flags), ele também mantém o log de uma sessão com falha no disco e imprime seu caminho no log do runner.

<h2 id="what’s-next">
  O que vem a seguir
</h2>

* [Customize sessions](/docs/pt/self-hosted-environments-configuration): wrapper scripts, lifecycle hooks, on-demand runners, MCP servers, and permissions
* [Test end to end](/docs/pt/self-hosted-environments-testing): verify a new runner image from CI before promoting it
* [Reference](/docs/pt/self-hosted-environments-reference): every CLI flag, environment variable, and metric
