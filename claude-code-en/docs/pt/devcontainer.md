> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Contêineres de desenvolvimento

> Execute Claude Code dentro de um contêiner de desenvolvimento para ambientes consistentes e isolados em toda sua equipe.

Um [contêiner de desenvolvimento](https://containers.dev/), ou dev container, permite que você defina um ambiente idêntico e isolado que cada engenheiro da sua equipe possa executar. Com Claude Code instalado nesse contêiner, os comandos que Claude executa funcionam dentro dele em vez de na máquina host, enquanto as edições nos arquivos do seu projeto aparecem no seu repositório local conforme você trabalha.

Esta página aborda [instalar Claude Code em um dev container](#add-claude-code-to-your-dev-container), depois um conjunto de tópicos de configuração independentes: persistir autenticação entre reconstruções, aplicar política organizacional, restringir saída de rede e executar sem prompts de permissão. Leia os que correspondem à sua configuração.

<Warning>
  Embora o dev container forneça proteções substanciais, nenhum sistema é completamente imune a todos os ataques.
  Quando executado com `--dangerously-skip-permissions`, dev containers não impedem que um projeto malicioso exfiltre qualquer coisa acessível dentro do contêiner, incluindo as credenciais do Claude Code armazenadas em [`~/.claude`](/docs/pt/claude-directory).
  Use dev containers apenas ao desenvolver com repositórios confiáveis e monitore as atividades do Claude.
  Evite montar segredos do host como `~/.ssh` ou arquivos de credenciais de nuvem no contêiner; prefira tokens com escopo de repositório ou de curta duração.
</Warning>

<Accordion title="Como dev containers funcionam com seu editor">
  <img src="https://mintcdn.com/claude-code/YvJyjZfd9yMihr0i/images/devcontainer-architecture.svg?fit=max&auto=format&n=YvJyjZfd9yMihr0i&q=85&s=9017b1d16a446c6cc37ba562f35b9aae" className="dark:hidden" alt="Diagrama mostrando um editor no host conectando a um contêiner dev Docker. Claude Code, o terminal e ferramentas de compilação executam dentro do contêiner. O repositório do host é bind-mounted no contêiner como o workspace." width="640" height="300" data-path="images/devcontainer-architecture.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/devcontainer-architecture-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=a0a340b1f2afc6a590696102c8acaaca" className="hidden dark:block" alt="Diagrama mostrando um editor no host conectando a um contêiner dev Docker. Claude Code, o terminal e ferramentas de compilação executam dentro do contêiner. O repositório do host é bind-mounted no contêiner como o workspace." width="640" height="300" data-path="images/devcontainer-architecture-dark.svg" />

  Um dev container é executado como um contêiner Docker, seja na sua máquina ou em um host de nuvem como GitHub Codespaces. Um editor que suporta a especificação Dev Containers, como VS Code, GitHub Codespaces, um IDE JetBrains ou Cursor, se conecta a esse contêiner: você navega e edita arquivos no editor como de costume, mas o terminal integrado, servidores de linguagem e ferramentas de compilação todos executam dentro do contêiner em vez de no seu host. Editores sem suporte a dev container, como Vim simples, não fazem parte deste fluxo de trabalho.

  Claude Code é executado dentro do contêiner, então ele vê os mesmos arquivos, dependências e ferramentas que o resto da cadeia de ferramentas do seu projeto. No VS Code você pode usar o [painel de extensão Claude Code](/docs/pt/vs-code) ou executar `claude` no terminal integrado; ambos executam dentro do contêiner e compartilham a mesma configuração `~/.claude`.
</Accordion>

<h2 id="add-claude-code-to-your-dev-container">
  Adicionar Claude Code ao seu dev container
</h2>

Claude Code é instalado em qualquer dev container através do [Claude Code Dev Container Feature](https://github.com/anthropics/devcontainer-features/tree/main/src/claude-code).

As configurações funcionam com qualquer ferramenta que suporte a especificação Dev Containers, como VS Code, GitHub Codespaces ou IDEs JetBrains. Os passos abaixo usam VS Code como exemplo.

Quando você abre o contêiner no VS Code ou Codespaces, o feature também adiciona a extensão Claude Code VS Code; outros editores ignoram essa parte.

<Tip>
  Novo em dev containers? O [tutorial Dev Containers do VS Code](https://code.visualstudio.com/docs/devcontainers/tutorial) orienta você na instalação do Docker, da extensão e na abertura do seu primeiro contêiner. Para um exemplo mais completo e endurecido com firewall e volumes persistentes, veja [Experimente o contêiner de referência](#try-the-reference-container).
</Tip>

<Steps>
  <Step title="Criar ou atualizar devcontainer.json">
    Salve o seguinte como `.devcontainer/devcontainer.json` no seu repositório, ou adicione o bloco `features` ao seu arquivo existente.

    A tag de versão no final, como `:1.0`, fixa o script de instalação do feature, não a versão do Claude Code. O feature instala o Claude Code mais recente, e Claude Code se atualiza automaticamente dentro do contêiner por padrão.

    Para fixar a versão da CLI ou desabilitar auto-atualização, veja [Aplicar política organizacional](#enforce-organization-policy).

    ```json .devcontainer/devcontainer.json theme={null}
    {
      "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
      "features": {
        "ghcr.io/anthropics/devcontainer-features/claude-code:1.0": {}
      }
    }
    ```

    Substitua a linha `image` pela imagem base do seu projeto ou remova-a se seu arquivo existente usar um Dockerfile.
  </Step>

  <Step title="Reconstruir o contêiner">
    Abra a Paleta de Comandos do VS Code com `Cmd+Shift+P` no Mac ou `Ctrl+Shift+P` no Windows e Linux, e execute **Dev Containers: Rebuild Container**.

    Para outras ferramentas, siga a ação de reconstrução dessa ferramenta: veja [reconstruindo no GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-a-codespace/rebuilding-the-container-in-a-codespace), a [CLI Dev Containers](https://github.com/devcontainers/cli), ou a documentação de dev container do seu IDE.
  </Step>

  <Step title="Entrar no Claude Code">
    Abra um terminal no contêiner reconstruído e execute `claude`, depois siga o prompt de autenticação.
  </Step>
</Steps>

O que você vê no prompt de autenticação depende do seu provedor:

* **Anthropic**: entre através de um navegador com sua conta Claude ou Anthropic Console
* **[Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](/docs/pt/third-party-integrations)**: Claude Code usa suas credenciais do provedor de nuvem, sem prompt de navegador

Para provedores de nuvem, passe credenciais para o contêiner como variáveis de ambiente através de `containerEnv`, um segredo do Codespaces, ou a identidade de carga de trabalho da sua nuvem em vez de montar arquivos de credenciais do host. Veja [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) ou [Microsoft Foundry](/docs/pt/microsoft-foundry) para a cadeia de credenciais que Claude Code lê.

Veja [Escolha seu provedor de API](/docs/pt/admin-setup#choose-your-api-provider) para decidir qual caminho se adequa à sua organização.

<Note>
  Se a entrada do navegador for concluída mas o callback nunca chegar ao contêiner, copie o código mostrado no navegador e cole-o no prompt `Paste code here if prompted` no terminal. Isso pode acontecer quando o encaminhamento de porta do editor não roteia o callback localhost.
</Note>

<h2 id="persist-authentication-and-settings-across-rebuilds">
  Persistir autenticação e configurações entre reconstruções
</h2>

Por padrão, o diretório home do contêiner é descartado na reconstrução, então os engenheiros devem entrar novamente a cada vez. Claude Code armazena seu token de autenticação, configurações do usuário e histórico de sessão em [`~/.claude`](/docs/pt/claude-directory). Monte um volume nomeado nesse caminho para manter esse estado entre reconstruções.

O exemplo a seguir monta um volume no diretório home do usuário `node`:

```json devcontainer.json theme={null}
"mounts": [
  "source=claude-code-config,target=/home/node/.claude,type=volume"
]
```

Substitua `/home/node` pelo diretório home do `remoteUser` do seu contêiner. Se você montar o volume em algum lugar diferente de `~/.claude`, defina [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars) para o caminho de montagem para que Claude Code leia e escreva lá.

Para isolar o estado por projeto em vez de compartilhar um volume em todos os repositórios, inclua a variável `${devcontainerId}` no nome da fonte. A [configuração de referência](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) usa `source=claude-code-config-${devcontainerId}` para esse propósito.

No GitHub Codespaces, `~/.claude` persiste entre parar e iniciar um codespace, mas ainda é limpo quando você reconstrói o contêiner, então a montagem de volume acima se aplica lá também. Para levar autenticação entre codespaces, armazene `ANTHROPIC_API_KEY` ou um `CLAUDE_CODE_OAUTH_TOKEN` de [`claude setup-token`](/docs/pt/authentication#generate-a-long-lived-token) como um [segredo do Codespaces](https://docs.github.com/en/codespaces/managing-your-codespaces/managing-your-account-specific-secrets-for-github-codespaces); Codespaces disponibiliza segredos como variáveis de ambiente dentro do contêiner automaticamente.

<h2 id="enforce-organization-policy">
  Aplicar política organizacional
</h2>

Um dev container é um lugar conveniente para aplicar política organizacional, porque a mesma imagem e configuração executam na máquina de cada engenheiro.

Claude Code lê `/etc/claude-code/managed-settings.json` no Linux e a aplica com a precedência mais alta na [hierarquia de configurações](/docs/pt/settings#how-scopes-interact), então valores lá substituem qualquer coisa que um engenheiro defina em `~/.claude` ou no diretório `.claude/` do projeto. Copie o arquivo para o lugar certo a partir do seu Dockerfile:

```dockerfile Dockerfile theme={null}
RUN mkdir -p /etc/claude-code
COPY managed-settings.json /etc/claude-code/managed-settings.json
```

Como o Dockerfile fica no repositório, qualquer pessoa com acesso de escrita pode alterar ou remover essa etapa. Para política que engenheiros não possam contornar editando arquivos do repositório, entregue configurações gerenciadas através de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) ou seu MDM em vez disso. Veja [arquivos de configurações gerenciadas](/docs/pt/settings#settings-files) para as chaves disponíveis e os outros caminhos de entrega.

Para definir [variáveis de ambiente](/docs/pt/env-vars) que se apliquem a cada sessão do Claude Code no contêiner, adicione-as a `containerEnv` no seu `devcontainer.json`. O exemplo a seguir desativa telemetria e relatório de erros e impede que Claude Code se atualize automaticamente após a instalação:

```json devcontainer.json theme={null}
"containerEnv": {
  "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
  "DISABLE_AUTOUPDATER": "1"
}
```

O Dev Container Feature sempre instala a versão mais recente do Claude Code. Para fixar uma versão específica do Claude Code para compilações reproduzíveis, instale-o a partir do seu Dockerfile com `npm install -g @anthropic-ai/claude-code@X.Y.Z` em vez de usar o feature, e defina `DISABLE_AUTOUPDATER` como mostrado acima.

Para a lista completa de controles de política incluindo regras de permissão, restrições de ferramentas e listas de permissão de servidores MCP, veja [Configure Claude Code para sua organização](/docs/pt/admin-setup).

Para disponibilizar [servidores MCP](/docs/pt/mcp) dentro do contêiner, defina-os no [escopo do projeto](/docs/pt/mcp#mcp-installation-scopes) em um arquivo `.mcp.json` na raiz do repositório para que sejam verificados junto com sua configuração de dev container. Instale quaisquer binários dos quais servidores stdio locais dependem no seu Dockerfile, e adicione domínios de servidor remoto à sua lista de permissão de rede.

<h2 id="restrict-network-egress">
  Restringir saída de rede
</h2>

Você pode limitar o tráfego de saída do contêiner apenas aos domínios que Claude Code precisa. Veja [Requisitos de acesso à rede](/docs/pt/network-config#network-access-requirements) para os domínios de inferência e autenticação, e [Serviços de telemetria](/docs/pt/data-usage#telemetry-services) para as conexões opcionais de telemetria e relatório de erros e como desabilitá-las.

O contêiner de referência inclui um script [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh) que bloqueia todo o tráfego de saída exceto os domínios que Claude Code e suas ferramentas de desenvolvimento precisam. Executar um firewall dentro de um contêiner requer permissões extras, então a referência adiciona as capacidades `NET_ADMIN` e `NET_RAW` através de `runArgs`. O script de firewall e essas capacidades não são necessários para o próprio Claude Code: você pode deixá-los de fora e confiar em seus próprios controles de rede em vez disso.

<h2 id="run-without-permission-prompts">
  Executar sem prompts de permissão
</h2>

Como o contêiner executa Claude Code como um usuário não-root e confina a execução de comandos ao contêiner, você pode passar `--dangerously-skip-permissions` para operação autônoma. A CLI rejeita essa flag quando lançada como root, então confirme que `remoteUser` está definido para uma conta não-root.

Pular prompts de permissão remove sua oportunidade de revisar chamadas de ferramentas antes de serem executadas. Claude ainda pode modificar qualquer arquivo no workspace bind-mounted, que aparece diretamente no seu host, e alcançar qualquer coisa que a política de rede do contêiner permite. Combine essa flag com as [restrições de saída de rede](#restrict-network-egress) acima para limitar o que uma sessão contornada pode alcançar.

Se você quer menos prompts sem desabilitar verificações de segurança, considere [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) em vez disso, que tem um classificador revisando ações antes de serem executadas. Para impedir que engenheiros usem `--dangerously-skip-permissions` completamente, defina `permissions.disableBypassPermissionsMode` para `"disable"` em [configurações gerenciadas](/docs/pt/settings#permission-settings).

<h2 id="try-the-reference-container">
  Experimente o contêiner de referência
</h2>

O repositório [`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/.devcontainer) inclui um exemplo de dev container que combina a CLI, o firewall de saída, volumes persistentes e um shell baseado em Zsh. É fornecido como um exemplo funcional em vez de uma imagem base mantida; use-o para ver como as peças se encaixam antes de aplicá-las à sua própria configuração.

<Steps>
  <Step title="Instalar pré-requisitos">
    Instale VS Code e a [extensão Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).
  </Step>

  <Step title="Clonar a referência">
    Clone o [repositório Claude Code](https://github.com/anthropics/claude-code) e abra-o no VS Code.
  </Step>

  <Step title="Reabrir no contêiner">
    Quando solicitado, clique em **Reopen in Container**, ou execute **Dev Containers: Reopen in Container** na Paleta de Comandos.
  </Step>

  <Step title="Iniciar Claude Code">
    Assim que o contêiner terminar de compilar, abra um terminal com `` Ctrl+` `` e execute `claude` para entrar e iniciar sua primeira sessão.
  </Step>
</Steps>

Para usar essa configuração com seu próprio projeto, copie o diretório `.devcontainer/` para seu repositório e ajuste o Dockerfile para sua cadeia de ferramentas, ou retorne a [Adicionar Claude Code ao seu dev container](#add-claude-code-to-your-dev-container) para adicionar apenas o feature a uma configuração que você já tem.

A configuração de referência consiste em três arquivos. Nenhum deles é necessário quando você adiciona Claude Code ao seu próprio dev container através do feature, mas eles mostram uma maneira de combinar as peças.

| Arquivo                                                                                                    | Propósito                                                                      |
| ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| [`devcontainer.json`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) | Montagens de volume, capacidades `runArgs`, extensões VS Code e `containerEnv` |
| [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/Dockerfile)               | Imagem base, ferramentas de desenvolvimento e a instalação do Claude Code      |
| [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh)   | Bloqueia todo o tráfego de rede de saída exceto os domínios permitidos         |

<h2 id="next-steps">
  Próximos passos
</h2>

Assim que Claude Code estiver executando no seu dev container, as páginas abaixo cobrem o resto de um rollout organizacional: escolher um caminho de autenticação, entregar política gerenciada fora do repositório, monitorar uso e entender o que Claude Code armazena e envia.

* [Configure Claude Code para sua organização](/docs/pt/admin-setup): escolha um provedor de autenticação, decida como a política chega aos dispositivos e planeje o rollout
* [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings): entregue política gerenciada do console de administrador Claude.ai para que engenheiros não possam contorná-la editando arquivos do repositório
* [Monitore uso e atividade de auditoria](/docs/pt/monitoring-usage): exporte métricas OpenTelemetry e revise o que sua equipe está executando
* [Requisitos de acesso à rede](/docs/pt/network-config#network-access-requirements): a lista completa de domínios para proxies e firewalls
* [Serviços de telemetria e opt-out](/docs/pt/data-usage#telemetry-services): o que Claude Code envia por padrão e as variáveis de ambiente que desabilitam
* [Explore o diretório `.claude`](/docs/pt/claude-directory): o que a montagem de volume contém, incluindo credenciais, configurações e histórico de sessão
* [Ambientes sandbox](/docs/pt/sandbox-environments): compare dev containers com o sandbox Bash integrado, containers personalizados e VMs
* [Modelo de segurança](/docs/pt/security): como o sistema de permissões do Claude Code, sandboxing e proteções contra injeção de prompt se encaixam
* [Modos de permissão](/docs/pt/permission-modes): a gama completa de modo de plano para modo automático para contorno, e quando usar cada um
