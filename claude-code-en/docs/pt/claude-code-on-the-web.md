> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code na web

> Configure ambientes em nuvem, scripts de configuração, acesso à rede e Docker na sandbox da Anthropic. Mova sessões entre web e terminal com `--cloud` e `--teleport`.

<Note>
  Claude Code na web está em visualização de pesquisa para usuários Pro, Max e Team, e para usuários Enterprise com assentos premium ou assentos Chat + Claude Code.
</Note>

Claude Code na web executa tarefas em infraestrutura em nuvem gerenciada pela Anthropic em [claude.ai/code](https://claude.ai/code). As sessões persistem mesmo se você fechar seu navegador, e você pode monitorá-las a partir do aplicativo móvel Claude.

<Tip>
  Novo no Claude Code na web? Comece com [Começar](/docs/pt/web-quickstart) para conectar sua conta GitHub e enviar sua primeira tarefa.
</Tip>

Esta página cobre:

* [Opções de autenticação do GitHub](#github-authentication-options): duas maneiras de conectar o GitHub
* [O ambiente em nuvem](#the-cloud-environment): qual configuração é transferida, quais ferramentas estão instaladas e como configurar ambientes
* [Scripts de configuração](#setup-scripts) e gerenciamento de dependências
* [Acesso à rede](#network-access): níveis, proxies e a lista de permissões padrão
* [Mover tarefas entre web e terminal](#move-tasks-between-web-and-terminal) com `--cloud` e `--teleport`
* [Trabalhar com sessões](#work-with-sessions): revisar, compartilhar, arquivar, deletar
* [Corrigir automaticamente pull requests](#auto-fix-pull-requests): responder automaticamente a falhas de CI e comentários de revisão
* [Segurança e isolamento](#security-and-isolation): como as sessões são isoladas
* [Limitações](#limitations): limites de taxa e restrições de plataforma

<h2 id="github-authentication-options">
  Opções de autenticação do GitHub
</h2>

As sessões em nuvem precisam de acesso aos seus repositórios GitHub para clonar código e enviar branches. Você pode conceder acesso de duas maneiras:

| Método           | Como funciona                                                                                        | Melhor para                                                                      |
| :--------------- | :--------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- |
| **GitHub App**   | Autorize o Claude GitHub App durante [onboarding na web](/docs/pt/web-quickstart).                        | Onboarding no navegador; equipes que desejam [Auto-fix](#auto-fix-pull-requests) |
| **`/web-setup`** | Execute `/web-setup` em seu terminal para sincronizar seu token CLI `gh` local com sua conta Claude. | Desenvolvedores individuais que já usam `gh`                                     |

<Note>
  Com qualquer método, uma sessão em nuvem pode acessar qualquer repositório que a conta GitHub conectada possa ver, não apenas os repositórios nos quais o Claude GitHub App está instalado. A instalação do App habilita webhooks de PR para [Auto-fix](#auto-fix-pull-requests); não é um controle de acesso no nível da sessão. Para restringir quais repositórios sua equipe pode alcançar a partir de sessões em nuvem, restrinja o acesso no próprio GitHub, por exemplo, limitando a associação de equipe ou repositório para as contas GitHub conectadas.
</Note>

Qualquer método funciona. [`/schedule`](/docs/pt/routines) verifica qualquer forma de acesso e solicita que você execute `/web-setup` se nenhum estiver configurado. Veja [Conectar a partir do seu terminal](/docs/pt/web-quickstart#connect-from-your-terminal) para o passo a passo de `/web-setup`.

O GitHub App é necessário para [Auto-fix](#auto-fix-pull-requests), que usa o App para receber webhooks de PR. Se você conectar com `/web-setup` e depois quiser Auto-fix, instale o App nesses repositórios.

Administradores de Team e Enterprise podem desabilitar `/web-setup` com o toggle Quick web setup em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

<Note>
  Organizações com [Zero Data Retention](/docs/pt/zero-data-retention) habilitado não podem usar `/web-setup` ou outros recursos de sessão em nuvem.
</Note>

<h2 id="the-cloud-environment">
  O ambiente em nuvem
</h2>

Cada sessão é executada em uma VM gerenciada pela Anthropic com seu repositório clonado. Esta seção cobre o que está disponível quando uma sessão inicia e como personalizá-lo.

<h3 id="what’s-available-in-cloud-sessions">
  O que está disponível em sessões em nuvem
</h3>

As sessões em nuvem começam a partir de um clone fresco do seu repositório. Qualquer coisa confirmada no repositório está disponível. Qualquer coisa que você tenha instalado ou configurado apenas em sua própria máquina não está disponível na sessão. A política da sua organização chega separadamente através de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings).

|                                                                                           | Disponível em sessões em nuvem | Por quê                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------------------------------------------------------------------------------- | :----------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Seu `CLAUDE.md` do repositório                                                            | Sim                            | Parte do clone                                                                                                                                                                                                                                                                                                                                                                    |
| Seus hooks `.claude/settings.json` do repositório                                         | Sim                            | Parte do clone                                                                                                                                                                                                                                                                                                                                                                    |
| Seus servidores MCP `.mcp.json` do repositório                                            | Sim                            | Parte do clone                                                                                                                                                                                                                                                                                                                                                                    |
| Seu `.claude/rules/` do repositório                                                       | Sim                            | Parte do clone                                                                                                                                                                                                                                                                                                                                                                    |
| Seu `.claude/skills/`, `.claude/agents/`, `.claude/commands/` do repositório              | Sim                            | Parte do clone                                                                                                                                                                                                                                                                                                                                                                    |
| Plugins declarados em `.claude/settings.json`                                             | Sim                            | Instalados no início da sessão a partir do [marketplace](/docs/pt/plugin-marketplaces) que você declarou. Requer acesso à rede para alcançar a fonte do marketplace                                                                                                                                                                                                                    |
| [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) da sua organização | Sim                            | Obtidas dos servidores da Anthropic quando a sessão inicia. Veja [Cobertura de superfície](/docs/pt/model-config#surface-coverage) para como `availableModels` é aplicado em sessões em nuvem. As configurações implantadas em seu dispositivo através de MDM ou arquivos de configurações gerenciadas não se aplicam, porque a sessão é executada em uma VM gerenciada pela Anthropic |
| Seu `CLAUDE.md` do usuário `~/.claude/`                                                   | Não                            | Vive em sua máquina, não no repositório                                                                                                                                                                                                                                                                                                                                           |
| Suas `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` do usuário           | Não                            | Vivem em sua máquina, não no repositório. Confirme-os no diretório `.claude/` do repositório. Skills que você habilita em claude.ai são carregadas em sessões em nuvem automaticamente                                                                                                                                                                                            |
| Plugins habilitados apenas em suas configurações de usuário                               | Não                            | `enabledPlugins` com escopo de usuário vive em `~/.claude/settings.json`. Declare-os em `.claude/settings.json` do repositório                                                                                                                                                                                                                                                    |
| Servidores MCP que você adicionou com `claude mcp add`                                    | Não                            | Aqueles escrevem em sua configuração de usuário local, não no repositório. Declare o servidor em [`.mcp.json`](/docs/pt/mcp#project-scope)                                                                                                                                                                                                                                             |
| Tokens de API estáticos e credenciais                                                     | Não                            | Nenhum armazenamento de segredos dedicado existe ainda. Veja abaixo                                                                                                                                                                                                                                                                                                               |
| Autenticação interativa como AWS SSO                                                      | Não                            | Não suportado. SSO requer login baseado em navegador que não pode ser executado em uma sessão em nuvem                                                                                                                                                                                                                                                                            |

Para disponibilizar sua própria configuração em sessões em nuvem, confirme-a no repositório; a política da organização chega separadamente através de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings).

Um armazenamento de segredos dedicado ainda não está disponível. Tanto variáveis de ambiente quanto scripts de configuração são armazenados na configuração de ambiente, visíveis para qualquer pessoa que possa editar esse ambiente. Se você precisar de segredos em uma sessão em nuvem, adicione-os como variáveis de ambiente com essa visibilidade em mente.

<h3 id="installed-tools">
  Ferramentas instaladas
</h3>

As sessões em nuvem vêm com tempos de execução de linguagem comuns, ferramentas de compilação e bancos de dados pré-instalados. A tabela abaixo resume o que está incluído por categoria.

| Categoria           | Incluído                                                                       |
| :------------------ | :----------------------------------------------------------------------------- |
| **Python**          | Python 3.x com pip, poetry, uv, black, mypy, pytest, ruff                      |
| **Node.js**         | 20, 21 e 22 via nvm, com npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**            | 3.1, 3.2, 3.3 com gem, bundler, rbenv                                          |
| **PHP**             | 8.4 com Composer                                                               |
| **Java**            | OpenJDK 21 com Maven e Gradle                                                  |
| **Go**              | estável mais recente com suporte a módulos                                     |
| **Rust**            | rustc e cargo                                                                  |
| **C/C++**           | GCC, Clang, cmake, ninja, conan                                                |
| **Docker**          | docker, dockerd, docker compose                                                |
| **Bancos de dados** | PostgreSQL 16, Redis 7.0                                                       |
| **Utilitários**     | git, jq, yq, ripgrep, tmux, vim, nano                                          |

¹ Bun está instalado mas tem [problemas de compatibilidade com proxy](#install-dependencies-with-a-sessionstart-hook) conhecidos para busca de pacotes.

Para versões exatas, peça a Claude para executar `check-tools` em uma sessão em nuvem. Este comando existe apenas em sessões em nuvem.

<h3 id="work-with-github-issues-and-pull-requests">
  Trabalhar com problemas e pull requests do GitHub
</h3>

As sessões em nuvem incluem ferramentas GitHub integradas que permitem que Claude leia problemas, liste pull requests, busque diffs e poste comentários sem nenhuma configuração. Essas ferramentas autenticam através do [proxy GitHub](#github-proxy) usando qualquer método que você configurou em [Opções de autenticação do GitHub](#github-authentication-options), então seu token nunca entra no contêiner.

Você pode definir `GH_TOKEN` ou `GITHUB_TOKEN` você mesmo em [configurações de ambiente](#configure-your-environment), ou deixar ambos não definidos e deixar o [proxy GitHub](#github-proxy) autenticar para você:

* Se você definir um token, ele passa através do contêiner inalterado, então `gh` e seus scripts o usam diretamente.
* Se você não definir nenhum, o contêiner define ambas as variáveis para a string de espaço reservado `proxy-injected` e o proxy substitui suas credenciais reais em solicitações GitHub de saída. `gh` funciona sem um token seu, mas um script que lê `GITHUB_TOKEN` diretamente obtém o espaço reservado, não um token utilizável.

Para verificar qual caso se aplica à sua sessão, peça a Claude para executar `echo $GH_TOKEN`.

O CLI `gh` não está pré-instalado. Se você precisar de um comando `gh` que as ferramentas integradas não cobrem, como `gh release` ou `gh workflow run`, instale e autentique você mesmo:

<Steps>
  <Step title="Instale gh em seu script de configuração">
    Adicione `apt update && apt install -y gh` ao seu [script de configuração](#setup-scripts).
  </Step>

  <Step title="Forneça um token se o proxy não estiver tratando a autenticação">
    Se `echo $GH_TOKEN` imprime `proxy-injected`, o [proxy GitHub](#github-proxy) autentica `gh` para você e esta etapa é desnecessária. Caso contrário, adicione uma variável de ambiente `GH_TOKEN` às suas [configurações de ambiente](#configure-your-environment) com um token de acesso pessoal do GitHub. `gh` lê `GH_TOKEN` automaticamente, então nenhuma etapa `gh auth login` é necessária.
  </Step>
</Steps>

<h3 id="link-output-back-to-the-session">
  Vincule a saída de volta à sessão
</h3>

Cada sessão em nuvem tem uma URL de transcrição em claude.ai, e a sessão pode ler seu próprio ID a partir da variável de ambiente `CLAUDE_CODE_REMOTE_SESSION_ID`. Use isso para colocar um link rastreável em corpos de PR, mensagens de commit, posts do Slack ou relatórios gerados para que um revisor possa abrir a execução que os produziu.

A partir da v2.1.179, commits que Claude cria em uma sessão web incluem um trailer git `Claude-Session: <url>`, e corpos de PR incluem a URL da sessão em sua própria linha. A partir da v2.1.182, defina [`attribution.sessionUrl`](/docs/pt/settings#attribution-settings) como `false` para omitir o trailer e o link do corpo de PR.

Para incluir o link da sessão em algo diferente de um commit ou PR, como uma mensagem do Slack que Claude posta ou um arquivo de relatório que ele escreve, peça a Claude para executar o seguinte comando e use sua saída. O comando converte o prefixo `cse_` no valor da variável de ambiente para o prefixo `session_` que a URL de transcrição espera:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Execute testes, inicie serviços e adicione pacotes
</h3>

Claude executa testes como parte do trabalho em uma tarefa. Peça no seu prompt, como "fix the failing tests in `tests/`" ou "run pytest after each change." Executores de teste como pytest, jest e cargo test estão pré-instalados e funcionam sem configuração adicional.

PostgreSQL e Redis estão pré-instalados mas não estão em execução por padrão. Peça a Claude para iniciar cada um durante a sessão:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker está disponível para executar serviços em contêiner. Peça a Claude para executar `docker compose up` para iniciar os serviços do seu projeto. O acesso à rede para puxar imagens segue o [nível de acesso](#access-levels) do seu ambiente, e os [Padrões confiáveis](#default-allowed-domains) incluem Docker Hub e outros registros comuns.

Se suas imagens são grandes ou lentas para puxar, adicione `docker compose pull` ou `docker compose build` ao seu [script de configuração](#setup-scripts). As imagens puxadas são salvas no [ambiente em cache](#environment-caching), então cada nova sessão as tem no disco. O cache armazena apenas arquivos, não processos em execução, então Claude ainda inicia os contêineres cada sessão.

Para adicionar pacotes que não estão pré-instalados, use um [script de configuração](#setup-scripts). A saída do script é [armazenada em cache](#environment-caching), então os pacotes que você instala lá estão disponíveis no início de cada sessão sem reinstalar cada vez. Você também pode pedir a Claude para instalar pacotes durante a sessão, mas essas instalações não persistem entre sessões.

<h3 id="resource-limits">
  Limites de recursos
</h3>

As sessões em nuvem são executadas com limites de recursos aproximados que podem mudar ao longo do tempo:

* 4 vCPUs
* 16 GB de RAM
* 30 GB de disco

Tarefas que requerem significativamente mais memória, como grandes trabalhos de compilação ou testes com uso intensivo de memória, podem falhar ou ser encerradas. Para cargas de trabalho além desses limites, use [Remote Control](/docs/pt/remote-control) para executar Claude Code em seu próprio hardware.

<h3 id="configure-your-environment">
  Configure seu ambiente
</h3>

Os ambientes controlam [acesso à rede](#network-access), variáveis de ambiente e o [script de configuração](#setup-scripts) que é executado antes de uma sessão iniciar. Veja [Ferramentas instaladas](#installed-tools) para o que está disponível sem nenhuma configuração. Você pode gerenciar ambientes a partir da interface web ou do terminal:

| Ação                            | Como                                                                                                                                                                                                                        |
| :------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Adicione um ambiente            | Selecione o ambiente atual para abrir o seletor, depois selecione **Add environment**. O diálogo inclui nome, nível de acesso à rede, variáveis de ambiente e script de configuração.                                       |
| Edite um ambiente               | Selecione o ícone de nuvem mostrando o nome do ambiente atual para abrir o seletor, passe o mouse sobre um ambiente e clique no ícone de configurações que aparece à direita.                                               |
| Arquive um ambiente             | Abra o ambiente para edição e selecione **Archive**. Ambientes arquivados ficam ocultos do seletor mas as sessões existentes continuam em execução.                                                                         |
| Defina o padrão para `--remote` | Execute `/remote-env` em seu terminal. Se você tiver um único ambiente, este comando mostra sua configuração atual. `/remote-env` apenas seleciona o padrão; adicione, edite e arquive ambientes a partir da interface web. |

As variáveis de ambiente usam formato `.env` com um par `KEY=value` por linha. Não envolva valores em aspas, já que as aspas são armazenadas como parte do valor. Este exemplo define três variáveis:

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

<h3 id="organization-shared-environments">
  Ambientes compartilhados da organização
</h3>

Proprietários e administradores em planos Team e Enterprise podem criar ambientes em nuvem que são compartilhados com cada membro da organização. Ambientes compartilhados aparecem no seletor de ambiente de cada membro ao lado dos seus pessoais, então um time pode padronizar em uma configuração em vez de cada membro recriá-la.

Gerencie ambientes compartilhados a partir da página **Cloud environments** em [configurações de administrador](https://claude.ai/admin-settings). De lá você pode:

* Criar, editar e arquivar ambientes compartilhados. Cada um tem os mesmos campos que um ambiente pessoal: um nome, um [nível de acesso à rede](#access-levels), [variáveis de ambiente](#configure-your-environment) em formato `.env`, e um [script de configuração](#setup-scripts).
* Defina o ambiente padrão para a organização.

Valores em um ambiente compartilhado chegam às sessões de cada membro naquele ambiente. Como ambientes pessoais, ambientes compartilhados não têm armazenamento de segredos dedicado, então não inclua segredos.

<h2 id="setup-scripts">
  Scripts de configuração
</h2>

Um script de configuração é um script Bash que é executado quando uma nova sessão em nuvem inicia, antes de Claude Code ser lançado. Use scripts de configuração para instalar dependências, configurar ferramentas ou buscar qualquer coisa que a sessão precise que não esteja pré-instalada.

Os scripts são executados como root no Ubuntu 24.04, então `apt install` e a maioria dos gerenciadores de pacotes de linguagem funcionam.

Para adicionar um script de configuração, abra o diálogo de configurações de ambiente e insira seu script no campo **Setup script**.

Este exemplo instala o CLI `gh`, que não está pré-instalado:

```bash theme={null}
#!/bin/bash
apt update && apt install -y gh
```

Se o script sair com código diferente de zero, a sessão falha ao iniciar. Acrescente `|| true` a comandos não críticos para evitar bloquear a sessão em uma falha de instalação intermitente.

Mantenha o tempo de execução total do script abaixo de aproximadamente cinco minutos para que o [cache de ambiente](#environment-caching) possa ser construído. Execute instalações independentes em paralelo com `&` e `wait`. Se um único download não se encaixar no limite de cinco minutos, mova-o para um [hook SessionStart](#setup-scripts-vs-sessionstart-hooks) que o inicia em segundo plano.

<Note>
  Scripts de configuração que instalam pacotes precisam de acesso à rede para alcançar registros. O acesso à rede padrão **Trusted** permite conexões com [domínios comuns permitidos](#default-allowed-domains) incluindo npm, PyPI, RubyGems e crates.io. Os scripts falharão ao instalar pacotes se seu ambiente usar acesso à rede **None**.
</Note>

<h3 id="environment-caching">
  Armazenamento em cache de ambiente
</h3>

O script de configuração é executado na primeira vez que você inicia uma sessão em um ambiente. Depois que é concluído, a Anthropic tira um snapshot do sistema de arquivos e reutiliza esse snapshot como ponto de partida para sessões posteriores. Novas sessões começam com suas dependências, ferramentas e imagens Docker já no disco, e a etapa de script de configuração é ignorada. Isso mantém a inicialização rápida mesmo quando o script instala grandes cadeias de ferramentas ou puxa imagens de contêiner.

O cache captura arquivos, não processos em execução. Qualquer coisa que o script de configuração escreve no disco é transferida. Serviços ou contêineres que ele inicia não são, então inicie-os por sessão pedindo a Claude ou com um [hook SessionStart](#setup-scripts-vs-sessionstart-hooks).

O script de configuração é executado novamente para reconstruir o cache quando você altera o script de configuração do ambiente ou hosts de rede permitidos, e quando o cache atinge sua expiração após aproximadamente sete dias. Retomar uma sessão existente nunca executa novamente o script de configuração.

Você não precisa habilitar armazenamento em cache ou gerenciar snapshots você mesmo.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Scripts de configuração vs. SessionStart hooks
</h3>

Use um script de configuração para instalar coisas que a nuvem precisa mas seu laptop já tem, como um tempo de execução de linguagem ou ferramenta CLI. Use um [SessionStart hook](/docs/pt/hooks#sessionstart) para configuração de projeto que deve ser executada em todos os lugares, nuvem e local, como `npm install`.

Ambos são executados no início de uma sessão, mas pertencem a lugares diferentes:

|                | Scripts de configuração                                                                                   | SessionStart hooks                                                         |
| -------------- | --------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Anexado a      | O ambiente em nuvem                                                                                       | Seu repositório                                                            |
| Configurado em | Interface do usuário do ambiente em nuvem                                                                 | `.claude/settings.json` em seu repositório                                 |
| Executa        | Antes de Claude Code ser lançado, quando nenhum [ambiente em cache](#environment-caching) está disponível | Depois de Claude Code ser lançado, em todas as sessões incluindo retomadas |
| Escopo         | Apenas ambientes em nuvem                                                                                 | Ambientes locais e em nuvem                                                |

Os SessionStart hooks também podem ser definidos em seu `~/.claude/settings.json` no nível do usuário localmente, mas as configurações no nível do usuário não são transferidas para sessões em nuvem. Na nuvem, hooks vêm do repositório e das [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) da sua organização.

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Instale dependências com um SessionStart hook
</h3>

Para instalar dependências apenas em sessões em nuvem, adicione um SessionStart hook ao `.claude/settings.json` do seu repositório:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

Crie o script em `scripts/install_pkgs.sh` e torne-o executável com `chmod +x`. A variável de ambiente `CLAUDE_CODE_REMOTE` é definida como `true` em sessões em nuvem, então você pode usá-la para pular execução local:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

Os SessionStart hooks têm algumas limitações em sessões em nuvem:

* **Sem escopo apenas para nuvem**: os hooks são executados em sessões locais e em nuvem. Para pular execução local, verifique a variável de ambiente `CLAUDE_CODE_REMOTE` conforme mostrado acima.
* **Requer acesso à rede**: os comandos de instalação precisam alcançar registros de pacotes. Se seu ambiente usar acesso à rede **None**, esses hooks falham. A [lista de permissões padrão](#default-allowed-domains) em **Trusted** cobre npm, PyPI, RubyGems e crates.io.
* **Compatibilidade com proxy**: todo o tráfego de saída passa por um [proxy de segurança](#security-proxy). Alguns gerenciadores de pacotes não funcionam corretamente com este proxy. Bun é um exemplo conhecido.
* **Adiciona latência de inicialização**: os hooks são executados cada vez que uma sessão inicia ou é retomada, diferentemente de scripts de configuração que se beneficiam do [armazenamento em cache de ambiente](#environment-caching). Mantenha os scripts de instalação rápidos verificando se as dependências já estão presentes antes de reinstalar.

Para persistir variáveis de ambiente para comandos Bash subsequentes, escreva no arquivo em `$CLAUDE_ENV_FILE`. Veja [SessionStart hooks](/docs/pt/hooks#sessionstart) para detalhes.

Substituir a imagem base pela sua própria imagem Docker ainda não é suportado. Use um script de configuração para instalar o que você precisa no topo da [imagem fornecida](#installed-tools), ou execute sua imagem como um contêiner ao lado de Claude com `docker compose`.

<h2 id="network-access">
  Acesso à rede
</h2>

O acesso à rede controla conexões de saída do ambiente em nuvem. Cada ambiente especifica um nível de acesso, e você pode estendê-lo com domínios permitidos personalizados. O padrão é **Trusted**, que permite registros de pacotes e outros [domínios na lista de permissões](#default-allowed-domains).

Para alterar o acesso à rede de um ambiente, [abra-o para edição](#configure-your-environment) e use o seletor **Network access** no diálogo. Não há uma página Environments separada. O ícone de nuvem aparece em qualquer lugar que você inicie uma sessão em nuvem ou configure uma [routine](/docs/pt/routines#environments-and-network-access).

<Note>
  O tráfego do conector MCP é roteado através dos servidores da Anthropic, então os conectores que você habilita em uma sessão ou routine funcionam sem adicionar seus hosts aos **Allowed domains**. Os conectores são configurados por sessão ou por routine; remova qualquer um que você não precise para limitar quais ferramentas Claude pode alcançar. Isso depende do mesmo canal vinculado à Anthropic observado em [Segurança e isolamento](#security-and-isolation).
</Note>

<h3 id="access-levels">
  Níveis de acesso
</h3>

Escolha um nível de acesso quando você criar ou editar um ambiente:

| Nível       | Conexões de saída                                                                                               |
| :---------- | :-------------------------------------------------------------------------------------------------------------- |
| **None**    | Sem acesso à rede de saída                                                                                      |
| **Trusted** | [Domínios na lista de permissões](#default-allowed-domains) apenas: registros de pacotes, GitHub, SDKs em nuvem |
| **Full**    | Qualquer domínio                                                                                                |
| **Custom**  | Sua própria lista de permissões, opcionalmente incluindo os padrões                                             |

As operações do GitHub usam um [proxy separado](#github-proxy) que é independente desta configuração.

<h3 id="allow-specific-domains">
  Permita domínios específicos
</h3>

Para permitir domínios que não estão na lista Trusted, selecione **Custom** nas configurações de acesso à rede do ambiente. Um campo **Allowed domains** aparece. Insira um domínio por linha:

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Use `*.` para correspondência de subdomínio curinga. Marque **Also include default list of common package managers** para manter os [domínios Trusted](#default-allowed-domains) junto com suas entradas personalizadas, ou deixe desmarcado para permitir apenas o que você listar.

Os domínios permitidos são configurados por ambiente. Não há uma lista de permissões no nível da organização que os Proprietários possam enviar para os ambientes de todos os usuários; [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) podem restringir sessões em nuvem, mas não podem adicionar domínios permitidos.

<h3 id="github-proxy">
  Proxy GitHub
</h3>

Para segurança, todas as operações do GitHub passam por um serviço de proxy dedicado que mantém suas credenciais reais do GitHub fora da sandbox. O proxy autentica dois tipos de tráfego:

* Interações Git: o cliente git dentro da sandbox usa uma credencial com escopo personalizado, que o proxy verifica e traduz para seu token de autenticação GitHub real
* Solicitações da API do GitHub: o proxy substitui suas credenciais reais em solicitações das ferramentas GitHub integradas e de `gh` quando sua sessão define o placeholder `proxy-injected` descrito em [Trabalhe com problemas e pull requests do GitHub](#work-with-github-issues-and-pull-requests)

O proxy também restringe operações git push para a branch de trabalho atual por segurança, e permite clonagem, busca e operações de PR enquanto mantém limites de segurança.

O proxy limita solicitações de API do GitHub e de ativos de lançamento a repositórios anexados à sessão, independentemente do [nível de acesso à rede](#access-levels) do ambiente. Scripts de configuração que baixam ativos de lançamento de repositórios não anexados retornam um 403. Arquivos confirmados de repositórios públicos são buscados através de `raw.githubusercontent.com`, que o [proxy de segurança](#security-proxy) trata em vez disso. Esse domínio está na [lista Trusted](#default-allowed-domains) padrão, então os arquivos permanecem acessíveis a menos que o [nível de acesso](#access-levels) do ambiente o exclua.

<h3 id="security-proxy">
  Proxy de segurança
</h3>

Os ambientes são executados atrás de um proxy de rede HTTP/HTTPS para fins de segurança e prevenção de abuso. Todo o tráfego de Internet de saída passa por este proxy, que fornece:

* Proteção contra solicitações maliciosas
* Limitação de taxa e prevenção de abuso
* Filtragem de conteúdo para segurança aprimorada
* Uma trilha de auditoria em nível de DNS de nomes de host solicitados

<h3 id="default-allowed-domains">
  Domínios padrão permitidos
</h3>

Ao usar acesso à rede **Trusted**, os seguintes domínios são permitidos por padrão. Domínios marcados com `*` indicam correspondência de subdomínio curinga, então `*.gcr.io` permite qualquer subdomínio de `gcr.io`.

<AccordionGroup>
  <Accordion title="Serviços Anthropic">
    * api.anthropic.com
    * statsig.anthropic.com
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

  <Accordion title="Plataformas em nuvem">
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
  </Accordion>

  <Accordion title="Gerenciadores de pacotes JavaScript e Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
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
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
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

  <Accordion title="Serviços em nuvem e monitoramento">
    * statsig.com
    * [www.statsig.com](http://www.statsig.com)
    * api.statsig.com
    * sentry.io
    * \*.sentry.io
    * downloads.sentry-cdn.com
    * http-intake.logs.datadoghq.com
    * browser-intake-us5-datadoghq.com
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

<h2 id="move-tasks-between-web-and-terminal">
  Mover tarefas entre web e terminal
</h2>

Esses fluxos de trabalho requerem o [Claude Code CLI](/docs/pt/quickstart) conectado à mesma conta claude.ai. Você pode iniciar novas sessões em nuvem a partir do seu terminal, ou puxar sessões em nuvem para seu terminal para continuar localmente. As sessões em nuvem persistem mesmo se você fechar seu laptop, e você pode monitorá-las de qualquer lugar, incluindo o aplicativo móvel Claude.

<Note>
  A partir do CLI, a transferência de sessão é unidirecional: você pode puxar sessões em nuvem para seu terminal com `--teleport`, mas não pode enviar uma sessão de terminal existente para a web. O sinalizador `--cloud` cria uma nova sessão em nuvem para seu repositório atual. O [aplicativo Desktop](/docs/pt/desktop#continue-in-another-surface) fornece um menu Continue in que pode enviar uma sessão local para a web.
</Note>

<h3 id="from-terminal-to-web">
  Do terminal para a web
</h3>

Inicie uma sessão em nuvem a partir da linha de comando com o sinalizador `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Isso cria uma nova sessão em nuvem em claude.ai. A sessão clona o remoto GitHub do seu diretório atual na sua branch atual, então envie primeiro se você tiver commits locais, já que a VM clona do GitHub em vez de sua máquina. `--cloud` funciona com um repositório por vez. A tarefa é executada na nuvem enquanto você continua trabalhando localmente. A ortografia mais antiga `--remote` ainda funciona como um alias descontinuado para `--cloud`.

A partir da v2.1.195, o CLI mostra uma lista de verificação ao vivo das etapas de configuração, como clonar o repositório e executar seu [script de configuração](#setup-scripts), enquanto o contêiner em nuvem é iniciado. As mensagens que você digita enquanto o contêiner está sendo provisionado são enfileiradas e enviadas assim que a sessão estiver pronta.

<Note>
  `--cloud` cria sessões em nuvem. `--remote-control` não está relacionado: expõe uma sessão CLI local para monitoramento a partir da web. Veja [Remote Control](/docs/pt/remote-control).
</Note>

Use `/tasks` no Claude Code CLI para verificar o progresso, ou abra a sessão em claude.ai ou no aplicativo móvel Claude para interagir diretamente. De lá você pode orientar Claude, fornecer feedback ou responder perguntas como em qualquer outra conversa.

<h4 id="tips-for-cloud-tasks">
  Dicas para tarefas em nuvem
</h4>

**Planeje localmente, execute remotamente**: para tarefas complexas, inicie Claude em plan mode para colaborar na abordagem, depois envie o trabalho para a nuvem:

```bash theme={null}
claude --permission-mode plan
```

Em plan mode, Claude lê arquivos, executa comandos para explorar e propõe um plano sem editar código-fonte. Depois de estar satisfeito, salve o plano no repositório, confirme e envie para que a VM em nuvem possa cloná-lo. Depois inicie uma sessão em nuvem para execução autônoma:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

Este padrão oferece controle sobre a estratégia enquanto permite que Claude execute autonomamente na nuvem.

**Planeje na nuvem com ultraplan**: para rascunhar e revisar o plano em si em uma sessão web, use [ultraplan](/docs/pt/ultraplan). Claude gera o plano em Claude Code na web enquanto você continua trabalhando, depois você comenta em seções em seu navegador e escolhe executar remotamente ou enviar o plano de volta para seu terminal.

**Execute tarefas em paralelo**: cada comando `--cloud` cria sua própria sessão em nuvem que é executada independentemente. Você pode iniciar múltiplas tarefas e todas serão executadas simultaneamente em sessões separadas:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Monitore todas as sessões com `/tasks` no Claude Code CLI. Quando uma sessão é concluída, você pode criar um PR a partir da interface web ou [teleportar](#from-web-to-terminal) a sessão para seu terminal para continuar trabalhando.

<h4 id="send-local-repositories-without-github">
  Envie repositórios locais sem GitHub
</h4>

Quando você executa `claude --cloud` a partir de um repositório que não está conectado ao GitHub, Claude Code agrupa seu repositório local e o carrega diretamente para a sessão em nuvem. O pacote inclui seu histórico completo de repositório em todas as branches, mais quaisquer alterações não confirmadas em arquivos rastreados.

Este fallback é ativado automaticamente quando o acesso ao GitHub não está disponível. Para forçá-lo mesmo quando o GitHub está conectado, defina `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Os repositórios agrupados devem atender a esses limites:

* O diretório deve ser um repositório git com pelo menos um commit
* O repositório agrupado deve estar abaixo de 100 MB. Repositórios maiores voltam a agrupar apenas a branch atual, depois a um snapshot único e compactado da árvore de trabalho, e falham apenas se o snapshot ainda for muito grande
* Arquivos não rastreados não estão incluídos; execute `git add` em arquivos que você deseja que a sessão em nuvem veja
* As sessões criadas a partir de um pacote não podem enviar de volta para um remoto a menos que você também tenha [autenticação do GitHub](#github-authentication-options) configurada

<h3 id="from-web-to-terminal">
  Da web para o terminal
</h3>

Puxe uma sessão em nuvem para seu terminal usando qualquer um destes:

* **Usando `--teleport`**: a partir da linha de comando, execute `claude --teleport` para um seletor de sessão interativo, ou `claude --teleport <session-id>` para retomar uma sessão específica diretamente. Se você tiver alterações não confirmadas, será solicitado que você as guarde primeiro.
* **Usando `/teleport`**: dentro de uma sessão CLI existente, execute `/teleport` ou `/tp` para abrir o mesmo seletor de sessão sem reiniciar Claude Code.
* **De `/tasks`**: execute `/tasks` para ver suas sessões em segundo plano, depois pressione `t` para teleportar para uma.
* **Da interface web**: selecione **Open in CLI** para copiar um comando que você pode colar em seu terminal.

Quando você teleporta uma sessão, Claude verifica se você está no repositório correto, busca e faz checkout da branch da sessão em nuvem e carrega o histórico completo da conversa em seu terminal.

`--teleport` é distinto de `--resume`. `--resume` reabre uma conversa do histórico local desta máquina e não lista sessões em nuvem; `--teleport` puxa uma sessão em nuvem e sua branch.

<h4 id="teleport-requirements">
  Requisitos de teleportação
</h4>

Teleport verifica esses requisitos antes de retomar uma sessão. Se algum requisito não for atendido, você verá um erro ou será solicitado a resolver o problema.

| Requisito           | Detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Estado git limpo    | Seu diretório de trabalho não deve ter alterações não confirmadas. Teleport solicita que você guarde as alterações se necessário.                                                                                                                                                                                                                                                                                                                                                   |
| Repositório correto | Você deve executar `--teleport` a partir de um checkout do mesmo repositório, não de um fork. A partir da v2.1.199, Claude Code aceita um checkout mesmo quando não consegue analisar o remoto em um nome de host, como um alias de host SSH como `git@work:owner/repo.git` ou uma forma abreviada reescrita por `insteadOf`. Ele mostra um prompt de confirmação primeiro, e apenas quando o proprietário do remoto e o nome do repositório correspondem ao repositório da sessão. |
| Branch disponível   | A branch da sessão em nuvem deve ter sido enviada para o remoto. Teleport busca e faz checkout automaticamente.                                                                                                                                                                                                                                                                                                                                                                     |
| Mesma conta         | Você deve estar autenticado na mesma conta claude.ai usada na sessão em nuvem.                                                                                                                                                                                                                                                                                                                                                                                                      |

<h4 id="teleport-is-unavailable">
  `--teleport` não está disponível
</h4>

Teleport requer autenticação de assinatura claude.ai. Se você estiver autenticado via chave de API, Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, execute `/login` para entrar com sua conta claude.ai. Se você já estiver conectado via claude.ai e `--teleport` ainda não estiver disponível, sua organização pode ter desabilitado sessões em nuvem.

<h2 id="work-with-sessions">
  Trabalhar com sessões
</h2>

As sessões aparecem na barra lateral em claude.ai/code. De lá você pode revisar alterações, compartilhar com colegas de equipe, arquivar trabalho concluído ou deletar sessões permanentemente.

<h3 id="manage-context">
  Gerenciar contexto
</h3>

As sessões em nuvem suportam [comandos integrados](/docs/pt/commands) que produzem saída de texto. Comandos que apenas executam na interface do terminal, como `/plugin` ou `/resume`, não estão disponíveis. Comandos que abrem um seletor ou painel no terminal se comportam de forma diferente em sessões em nuvem:

* **`/model`, `/effort`, `/fast`, `/color` e `/rename`**: passam o valor como um argumento, por exemplo `/model sonnet`, em vez de abrir o seletor de terminal ou controle deslizante. As formas de argumento requerem Claude Code v2.1.205 ou posterior no ambiente da sessão e seguem as [notas de disponibilidade](/docs/pt/commands#all-commands) de cada comando: `/effort` relata `Not applied` enquanto um [esforço padrão de lançamento](/docs/pt/model-config#adjust-effort-level) de modelo está em vigor, e `/fast` funciona apenas em uma sessão que começou com o modo rápido ativado.
* **`/config`**: na web, abre a seção Claude Code de suas configurações em vez de definir um valor, e o texto após o comando, incluindo `key=value`, é ignorado. Para alterar as configurações de uma sessão em nuvem, use [variáveis de ambiente](#configure-your-environment) ou confirme [arquivos de configurações](/docs/pt/settings) no repositório.

Para gerenciamento de contexto especificamente:

| Comando    | Funciona em sessões em nuvem | Notas                                                                                                             |
| :--------- | :--------------------------- | :---------------------------------------------------------------------------------------------------------------- |
| `/compact` | Sim                          | Resume a conversa para liberar contexto. Aceita instruções de foco opcionais como `/compact keep the test output` |
| `/context` | Sim                          | Mostra o que está atualmente na janela de contexto                                                                |
| `/clear`   | Não                          | Inicie uma nova sessão a partir da barra lateral                                                                  |

A auto-compactação é executada automaticamente quando a janela de contexto se aproxima da capacidade. Para acioná-la mais cedo, defina [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/pt/env-vars) em suas [variáveis de ambiente](#configure-your-environment). Por exemplo, `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` compacta em 70% de capacidade em vez de esperar até que a janela esteja quase cheia. Para alterar o tamanho efetivo da janela para cálculos de compactação, use [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/pt/env-vars).

[Subagentes](/docs/pt/sub-agents) funcionam da mesma forma que localmente. Claude pode gerá-los com a ferramenta Task para descarregar pesquisa ou trabalho paralelo em uma janela de contexto separada, mantendo a conversa principal mais leve. Subagentes definidos em seu `.claude/agents/` do repositório são coletados automaticamente.

[Equipes de agentes](/docs/pt/agent-teams) estão desabilitadas por padrão mas podem ser habilitadas adicionando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` às suas [variáveis de ambiente](#configure-your-environment).

<h3 id="review-changes">
  Revise alterações
</h3>

Cada sessão mostra um indicador de diff com linhas adicionadas e removidas, como `+42 -18`. Selecione-o para abrir a visualização de diff, deixe comentários inline em linhas específicas e envie-os para Claude com sua próxima mensagem. Veja [Review and iterate](/docs/pt/web-quickstart#review-and-iterate) para o passo a passo completo incluindo criação de PR. Para ter Claude monitorar o PR para falhas de CI e comentários de revisão automaticamente, veja [Auto-fix pull requests](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Compartilhe sessões
</h3>

Para compartilhar uma sessão, alterne sua visibilidade de acordo com os tipos de conta abaixo. Depois disso, compartilhe o link da sessão como está. Os destinatários veem o estado mais recente quando abrem o link, mas sua visualização não é atualizada em tempo real.

<h4 id="share-from-an-enterprise-or-team-account">
  Compartilhe de uma conta Enterprise ou Team
</h4>

Para contas Enterprise e Team, as duas opções de visibilidade são **Private** e **Team**. A visibilidade Team torna a sessão visível para outros membros de sua organização claude.ai. [Claude in Slack](/docs/pt/slack) as sessões são automaticamente compartilhadas com visibilidade Team.

A verificação de acesso ao repositório é habilitada por padrão, com base na conta GitHub conectada à conta do destinatário. O nome de exibição de sua conta é visível para todos os destinatários com acesso.

<h4 id="share-from-a-max-or-pro-account">
  Compartilhe de uma conta Max ou Pro
</h4>

Para contas Max e Pro, as duas opções de visibilidade são **Private** e **Public**. A visibilidade Public torna a sessão visível para qualquer usuário conectado a claude.ai.

Verifique sua sessão para conteúdo sensível antes de compartilhar. As sessões podem conter código e credenciais de repositórios GitHub privados. A verificação de acesso ao repositório não é habilitada por padrão.

Para exigir que os destinatários tenham acesso ao repositório, ou para ocultar seu nome de sessões compartilhadas, vá para Settings > Claude Code > Sharing settings.

<h3 id="archive-sessions">
  Arquive sessões
</h3>

Você pode arquivar sessões para manter sua lista de sessões organizada. As sessões arquivadas ficam ocultas da lista de sessões padrão mas podem ser visualizadas filtrando por sessões arquivadas.

Para arquivar uma sessão, passe o mouse sobre a sessão na barra lateral e selecione o ícone de arquivo.

<h3 id="delete-sessions">
  Delete sessões
</h3>

Deletar uma sessão remove permanentemente a sessão e seus dados. Esta ação não pode ser desfeita. Você pode deletar uma sessão de duas maneiras:

* **Da barra lateral**: filtre por sessões arquivadas, depois passe o mouse sobre a sessão que deseja deletar e selecione o ícone de exclusão
* **Do menu de sessão**: abra uma sessão, selecione o menu suspenso ao lado do título da sessão e selecione **Delete**

Você será solicitado a confirmar antes de uma sessão ser deletada.

<h2 id="auto-fix-pull-requests">
  Corrigir automaticamente pull requests
</h2>

Claude pode observar um pull request e responder automaticamente a falhas de CI e comentários de revisão. Claude se inscreve na atividade do GitHub no PR, e quando uma verificação falha ou um revisor deixa um comentário, Claude investiga e envia uma correção se uma for clara.

<Note>
  Auto-fix requer que o Claude GitHub App esteja instalado em seu repositório. Se você ainda não fez isso, instale-o a partir da [página do GitHub App](https://github.com/apps/claude) ou quando solicitado durante [setup](/docs/pt/web-quickstart#connect-github-and-create-an-environment).
</Note>

Existem algumas maneiras de ativar auto-fix dependendo de onde o PR veio e qual dispositivo você está usando:

* **PRs criados em Claude Code na web**: abra a barra de status de CI e selecione **Auto-fix**
* **A partir do seu terminal**: execute [`/autofix-pr`](/docs/pt/commands) enquanto estiver na branch do PR. Claude Code detecta o PR aberto com `gh`, gera uma sessão web e ativa auto-fix em uma etapa
* **A partir do aplicativo móvel**: diga a Claude para corrigir automaticamente o PR, por exemplo "watch this PR and fix any CI failures or review comments"
* **Qualquer PR existente**: cole a URL do PR em uma sessão e diga a Claude para corrigir automaticamente

Auto-fix é um toggle por PR. Para parar de monitorar, abra a barra de status de CI na sessão web e desmarque o toggle **Auto-fix**, ou diga a Claude para parar de observar o PR.

<h3 id="how-claude-responds-to-pr-activity">
  Como Claude responde à atividade de PR
</h3>

Quando auto-fix está ativo, Claude recebe eventos do GitHub para o PR incluindo novos comentários de revisão e falhas de verificação de CI. Para cada evento, Claude investiga e decide como proceder:

* **Correções claras**: se Claude está confiante em uma correção e ela não entra em conflito com instruções anteriores, Claude faz a alteração, envia e explica o que foi feito na sessão
* **Solicitações ambíguas**: se um comentário de revisor pode ser interpretado de múltiplas maneiras ou envolve algo arquitetonicamente significativo, Claude pergunta a você antes de agir
* **Eventos duplicados ou sem ação**: se um evento é duplicado ou não requer alteração, Claude o anota na sessão e continua

GitHub não emite um webhook quando a branch base avança e cria um conflito de merge, então auto-fix não pode reagir a conflitos por conta própria. Para resolver um conflito, abra a sessão e peça a Claude para fazer rebase.

Claude pode responder a threads de comentários de revisão no GitHub como parte da resolução deles. Essas respostas são postadas usando sua conta GitHub, então aparecem sob seu nome de usuário, mas cada resposta é rotulada como vindo de Claude Code para que os revisores saibam que foi escrita pelo agente e não por você diretamente.

<Warning>
  Se seu repositório usa automação acionada por comentário, como Atlantis, Terraform Cloud ou GitHub Actions personalizadas que são executadas em eventos `issue_comment`, esteja ciente de que Claude pode responder em seu nome, o que pode acionar esses fluxos de trabalho. Revise a automação de seu repositório antes de ativar auto-fix e considere desabilitar auto-fix para repositórios onde um comentário de PR pode implantar infraestrutura ou executar operações privilegiadas.
</Warning>

<h2 id="security-and-isolation">
  Segurança e isolamento
</h2>

Cada sessão em nuvem é separada de sua máquina e de outras sessões através de várias camadas:

* **Máquinas virtuais isoladas**: cada sessão é executada em uma VM isolada gerenciada pela Anthropic
* **Controles de acesso à rede**: o acesso à rede é limitado por padrão e pode ser desabilitado. Ao executar com acesso à rede desabilitado, Claude Code ainda pode se comunicar com a API Anthropic, o que pode permitir que dados saiam da VM.
* **Proteção de credenciais**: credenciais sensíveis como credenciais git ou chaves de assinatura nunca estão dentro da sandbox com Claude Code. A autenticação é tratada através de um proxy seguro usando credenciais com escopo.
* **Análise segura**: o código é analisado e modificado dentro de VMs isoladas antes de criar PRs

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Para erros de API de tempo de execução que aparecem na conversa como `API Error: 500`, `529 Overloaded`, `429` ou `Prompt is too long`, veja a [referência de erros](/docs/pt/errors). Esses erros e suas correções são compartilhados com o CLI e aplicativo Desktop. As seções abaixo cobrem problemas específicos para sessões em nuvem.

<h3 id="session-creation-failed">
  Session creation failed
</h3>

Se uma nova sessão falha ao iniciar com `Session creation failed` ou fica presa em provisioning, Claude Code não conseguiu alocar um ambiente em nuvem.

* Verifique [status.claude.com](https://status.claude.com) para incidentes de sessão em nuvem
* Tente novamente após um minuto, já que a capacidade é provisionada sob demanda
* Confirme que seu repositório é acessível. A conta GitHub conectada deve ter acesso ao repositório no GitHub, seja através da autorização do Claude GitHub App ou um token `gh` sincronizado via `/web-setup`. Instalar o App no repositório não é necessário. Veja [Opções de autenticação do GitHub](#github-authentication-options).

<h3 id="remote-control-session-expired-or-access-denied">
  Remote Control session expired or access denied
</h3>

`--teleport` se conecta através da mesma infraestrutura de sessão Remote Control que as sessões em nuvem usam, então erros de autenticação e expiração de sessão aparecem com wording de Remote Control. Você pode ver `Remote Control session expired` ou `Access denied`. O token de conexão é de curta duração e limitado à sua conta.

* Execute `/login` localmente para atualizar suas credenciais, depois reconecte
* Confirme que você está conectado à mesma conta que possui a sessão
* Se você ver `Remote Control may not be available for this organization`, um Owner não habilitou sessões em nuvem para sua organização

<h3 id="environment-expired">
  Environment expired
</h3>

As sessões em nuvem param após um período de inatividade e o ambiente subjacente é recuperado. A partir de um terminal local, isso aparece como `Could not resume session ... its environment has expired. Creating a fresh session instead.` Na web, a sessão é marcada como expirada na lista de sessões.

Reabra a sessão de [claude.ai/code](https://claude.ai/code) para provisionar um ambiente fresco com seu histórico de conversa restaurado.

<h2 id="limitations">
  Limitações
</h2>

Antes de confiar em sessões em nuvem para um fluxo de trabalho, leve em conta essas restrições:

* **Limites de taxa**: Claude Code na web compartilha limites de taxa com todo o outro uso de Claude e Claude Code dentro de sua conta. Executar múltiplas tarefas em paralelo consome mais limites de taxa proporcionalmente. Não há cobrança de computação separada para a VM em nuvem.
* **Autenticação de repositório**: você pode apenas mover sessões de web para local quando está autenticado na mesma conta
* **Restrições de plataforma**: clonagem de repositório e criação de pull request requerem GitHub. Instâncias [GitHub Enterprise Server](/docs/pt/github-enterprise-server) auto-hospedadas são suportadas para planos Team e Enterprise. GitLab, Bitbucket e outros repositórios não-GitHub podem ser enviados para sessões em nuvem como um [pacote local](#send-local-repositories-without-github), mas a sessão não pode enviar resultados de volta para o remoto
* **IP allowlist da organização**: as sessões em nuvem chamam a API Anthropic a partir de infraestrutura gerenciada pela Anthropic, não de sua rede. Se sua organização tem [IP allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) habilitado, cada sessão em nuvem falha com um erro de autenticação. O mesmo se aplica a [Code Review](/docs/pt/code-review) e [Routines](/docs/pt/routines). Entre em contato com [suporte Anthropic](https://support.claude.com/) para isentar serviços hospedados pela Anthropic do allowlist de IP de sua organização.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Ultraplan](/docs/pt/ultraplan): rascunhe um plano em uma sessão em nuvem e revise-o em seu navegador
* [Ultrareview](/docs/pt/ultrareview): execute uma revisão de código profunda multi-agente em uma sandbox em nuvem
* [Routines](/docs/pt/routines): automatize trabalho em um cronograma, via chamada de API ou em resposta a eventos do GitHub
* [Configuração de hooks](/docs/pt/hooks): execute scripts em eventos do ciclo de vida da sessão
* [Referência de configurações](/docs/pt/settings): todas as opções de configuração
* [Segurança](/docs/pt/security): garantias de isolamento e tratamento de dados
* [Uso de dados](/docs/pt/data-usage): o que Anthropic retém de sessões em nuvem
* [Claude Tag](https://claude.com/docs/claude-tag/overview): um @Claude gerenciado pela organização no Slack que é executado no mesmo ambiente em nuvem
