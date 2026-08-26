> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Descubra e instale plugins pré-construídos através de marketplaces

> Encontre e instale plugins de marketplaces para estender Claude Code com novas skills, agentes e capacidades.

Plugins estendem Claude Code com skills, agentes, hooks e MCP servers. Marketplaces de plugins são catálogos que ajudam você a descobrir e instalar essas extensões sem construí-las você mesmo.

Procurando criar e distribuir seu próprio marketplace? Veja [Criar e distribuir um marketplace de plugins](/docs/pt/plugin-marketplaces).

<h2 id="how-marketplaces-work">
  Como os marketplaces funcionam
</h2>

Um marketplace é um catálogo de plugins que alguém criou e compartilhou. Usar um marketplace é um processo de duas etapas:

<Steps>
  <Step title="Adicione o marketplace">
    Isso registra o catálogo com Claude Code para que você possa navegar o que está disponível. Nenhum plugin é instalado ainda.
  </Step>

  <Step title="Instale plugins individuais">
    Navegue pelo catálogo e instale os plugins que você deseja.
  </Step>
</Steps>

Pense nisso como adicionar uma loja de aplicativos: adicionar a loja oferece acesso para navegar sua coleção, mas você ainda escolhe quais aplicativos baixar individualmente.

<h2 id="official-anthropic-marketplace">
  Marketplace oficial da Anthropic
</h2>

O marketplace oficial da Anthropic (`claude-plugins-official`) está automaticamente disponível quando você inicia Claude Code. Execute `/plugin` e vá para a aba **Discover** para navegar o que está disponível, ou visualize o catálogo em [claude.com/plugins](https://claude.com/plugins).

Para instalar um plugin do marketplace oficial, use `/plugin install <name>@claude-plugins-official`. Por exemplo, para instalar a integração do GitHub:

```shell theme={null}
/plugin install github@claude-plugins-official
```

Se Claude Code relatar que o plugin não foi encontrado em nenhum marketplace, seu marketplace está ausente ou desatualizado. Execute `/plugin marketplace update claude-plugins-official` para atualizá-lo, ou `/plugin marketplace add anthropics/claude-plugins-official` se você não o adicionou antes. Depois tente instalar novamente.

<Note>
  O marketplace oficial é mantido pela Anthropic, e a inclusão é a critério da Anthropic. Os formulários de envio no aplicativo adicionam plugins ao [marketplace da comunidade](#community-marketplace), não ao oficial. Para distribuir plugins independentemente, [crie seu próprio marketplace](/docs/pt/plugin-marketplaces) e compartilhe com usuários.
</Note>

O marketplace oficial inclui várias categorias de plugins:

<h3 id="code-intelligence">
  Code intelligence
</h3>

Plugins de code intelligence habilitam a ferramenta LSP integrada do Claude Code, dando a Claude a capacidade de pular para definições, encontrar referências e ver erros de tipo imediatamente após edições. Esses plugins configuram conexões do [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), a mesma tecnologia que alimenta a code intelligence do VS Code.

Esses plugins requerem que o binário do language server esteja instalado no seu sistema. Se você já tem um language server instalado, Claude pode solicitar que você instale o plugin correspondente quando abrir um projeto.

| Linguagem  | Plugin              | Binário necessário           |
| :--------- | :------------------ | :--------------------------- |
| C/C++      | `clangd-lsp`        | `clangd`                     |
| C#         | `csharp-lsp`        | `csharp-ls`                  |
| Go         | `gopls-lsp`         | `gopls`                      |
| Java       | `jdtls-lsp`         | `jdtls`                      |
| Kotlin     | `kotlin-lsp`        | `kotlin-language-server`     |
| Lua        | `lua-lsp`           | `lua-language-server`        |
| PHP        | `php-lsp`           | `intelephense`               |
| Python     | `pyright-lsp`       | `pyright-langserver`         |
| Rust       | `rust-analyzer-lsp` | `rust-analyzer`              |
| Swift      | `swift-lsp`         | `sourcekit-lsp`              |
| TypeScript | `typescript-lsp`    | `typescript-language-server` |

Você também pode [criar seu próprio plugin LSP](/docs/pt/plugins-reference#lsp-servers) para outras linguagens.

<Note>
  Se você vir `Executable not found in $PATH` na aba Errors do `/plugin` após instalar um plugin, instale o binário necessário da tabela acima.
</Note>

<h4 id="what-claude-gains-from-code-intelligence-plugins">
  O que Claude ganha com plugins de code intelligence
</h4>

Uma vez que um plugin de code intelligence está instalado e seu binário de language server está disponível, Claude ganha duas capacidades:

* **Diagnósticos automáticos**: após cada edição de arquivo que Claude faz, o language server analisa as mudanças e relata erros e avisos automaticamente. Claude vê erros de tipo, importações faltantes e problemas de sintaxe sem precisar executar um compilador ou linter. Se Claude introduzir um erro, ele percebe e corrige o problema na mesma volta. Isso não requer configuração além de instalar o plugin. Você pode ver diagnósticos inline pressionando **Ctrl+O** quando o indicador "diagnostics found" aparecer.
* **Navegação de código**: Claude pode usar o language server para pular para definições, encontrar referências, obter informações de tipo ao passar o mouse, listar símbolos, encontrar implementações e rastrear hierarquias de chamadas. Essas operações dão a Claude navegação mais precisa do que busca baseada em grep, embora a disponibilidade possa variar por linguagem e ambiente.

Se você encontrar problemas, veja [Troubleshooting de code intelligence](#code-intelligence-issues).

<h3 id="external-integrations">
  Integrações externas
</h3>

Esses plugins agrupam [MCP servers](/docs/pt/mcp) pré-configurados para que você possa conectar Claude a serviços externos sem configuração manual:

* **Controle de fonte**: `github`, `gitlab`
* **Gerenciamento de projetos**: `atlassian` (Jira/Confluence), `asana`, `linear`, `notion`
* **Design**: `figma`
* **Infraestrutura**: `vercel`, `firebase`, `supabase`
* **Comunicação**: `slack`
* **Monitoramento**: `sentry`

<h3 id="automatic-security-review">
  Revisão automática de segurança
</h3>

O plugin `security-guidance` revisa cada mudança que Claude faz em busca de vulnerabilidades comuns e instrui Claude a corrigir o que encontra na mesma sessão. Veja [Catch security issues as Claude writes code](/docs/pt/security-guidance) para o que ele verifica e como adicionar regras específicas do projeto.

<h3 id="development-workflows">
  Fluxos de trabalho de desenvolvimento
</h3>

Plugins que adicionam skills e agentes para tarefas comuns de desenvolvimento:

* **commit-commands**: Fluxos de trabalho de commit do Git incluindo commit, push e criação de PR
* **pr-review-toolkit**: Agentes especializados para revisar pull requests
* **agent-sdk-dev**: Ferramentas para construir com o Claude Agent SDK
* **plugin-dev**: Toolkit para criar seus próprios plugins

<h3 id="output-styles">
  Estilos de saída
</h3>

Customize como Claude responde:

* **explanatory-output-style**: Insights educacionais sobre escolhas de implementação
* **learning-output-style**: Modo de aprendizado interativo para construção de skills

<h2 id="community-marketplace">
  Marketplace da comunidade
</h2>

O marketplace da comunidade em [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) hospeda plugins de terceiros que passaram pela validação automatizada da Anthropic e triagem de segurança. Cada plugin é fixado a um SHA de commit específico no catálogo. Diferentemente do marketplace oficial, você o adiciona manualmente:

```shell theme={null}
/plugin marketplace add anthropics/claude-plugins-community
```

Depois instale plugins dele usando o nome de marketplace `claude-community`:

```shell theme={null}
/plugin install <plugin-name>@claude-community
```

Para enviar seu próprio plugin para o marketplace da comunidade, veja [Envie seu plugin para o marketplace da comunidade](/docs/pt/plugins#submit-your-plugin-to-the-community-marketplace) no guia de criação de plugins.

<h2 id="try-it-add-the-demo-marketplace">
  Experimente: adicione o marketplace de demonstração
</h2>

Anthropic também mantém um [marketplace de plugins de demonstração](https://github.com/anthropics/claude-code/tree/main/plugins) (`claude-code-plugins`) com plugins de exemplo que mostram o que é possível com o sistema de plugins. Diferentemente do marketplace oficial, você precisa adicionar este manualmente.

<Steps>
  <Step title="Adicione o marketplace">
    De dentro do Claude Code, execute o comando `plugin marketplace add` para o marketplace `anthropics/claude-code`:

    ```shell theme={null}
    /plugin marketplace add anthropics/claude-code
    ```

    Isso baixa o catálogo do marketplace e torna seus plugins disponíveis para você.
  </Step>

  <Step title="Navegue pelos plugins disponíveis">
    Execute `/plugin` para abrir o gerenciador de plugins. Isso abre uma interface com abas com quatro abas que você pode percorrer usando **Tab**, ou **Shift+Tab** para ir para trás:

    * **Discover**: navegue pelos plugins disponíveis de todos os seus marketplaces
    * **Installed**: visualize e gerencie seus plugins instalados
    * **Marketplaces**: adicione, remova ou atualize seus marketplaces adicionados
    * **Errors**: visualize quaisquer erros de carregamento de plugins

    Vá para a aba **Discover** para ver plugins do marketplace que você acabou de adicionar. Quando seu administrador tiver adicionado o marketplace à lista de permissões por meio da configuração gerenciada [`pluginSuggestionMarketplaces`](/docs/pt/settings#available-settings), plugins marcados como relevantes para seu diretório de trabalho atual são fixados no topo com um rótulo **suggested for this directory**.
  </Step>

  <Step title="Instale um plugin">
    Selecione um plugin para visualizar seus detalhes. O painel de detalhes mostra o que o plugin contém e quanto custa:

    * Uma estimativa de **Context cost** para que você possa ver quantos tokens o plugin adicionará à sua [janela de contexto](/docs/pt/features-overview#understand-context-costs) a cada turno (Claude Code v2.1.143 e posterior)
    * A data de **Last updated** do plugin (v2.1.144 e posterior)
    * Uma seção **Will install** listando os comandos, agentes, skills, hooks e servidores MCP e LSP do plugin, para que você possa revisar exatamente o que ele adiciona antes de instalar (v2.1.145 e posterior)

    Escolha um escopo de instalação:

    * **User scope**: instale para você em todos os projetos
    * **Project scope**: instale para todos os colaboradores neste repositório
    * **Local scope**: instale para você neste repositório apenas

    Por exemplo, selecione **commit-commands**, um plugin que adiciona skills de fluxo de trabalho git, e instale-o no seu escopo de usuário.

    Você também pode instalar diretamente da linha de comando:

    ```shell theme={null}
    /plugin install commit-commands@claude-code-plugins
    ```

    Veja [Configuration scopes](/docs/pt/settings#configuration-scopes) para aprender mais sobre escopos.
  </Step>

  <Step title="Use seu novo plugin">
    Após instalar, execute `/reload-plugins` para ativar o plugin. Skills de plugin são nomeadas com namespace pelo nome do plugin, então **commit-commands** fornece skills como `/commit-commands:commit`.

    Experimente fazendo uma mudança em um arquivo e executando:

    ```shell theme={null}
    /commit-commands:commit
    ```

    Isso prepara suas mudanças, gera uma mensagem de commit e cria o commit.

    Cada plugin funciona diferentemente. Verifique os detalhes do plugin na aba **Discover** para ver os comandos e skills que ele fornece, ou visite sua página inicial para orientação de uso.
  </Step>
</Steps>

O resto deste guia cobre todas as maneiras que você pode adicionar marketplaces, instalar plugins e gerenciar sua configuração.

<h2 id="add-marketplaces">
  Adicione marketplaces
</h2>

Use o comando `/plugin marketplace add` para adicionar marketplaces de diferentes fontes.

<Tip>
  **Atalhos**: Você pode usar `/plugin market` em vez de `/plugin marketplace` e `rm` em vez de `remove`.
</Tip>

* **Repositórios GitHub**: formato `owner/repo`, por exemplo `anthropics/claude-code`
* **URLs Git**: qualquer URL de repositório git, incluindo GitLab, Bitbucket e servidores auto-hospedados
* **Caminhos locais**: diretórios ou caminhos diretos para arquivos `marketplace.json`
* **URLs remotas**: URLs diretas para arquivos `marketplace.json` hospedados

<h3 id="add-from-github">
  Adicione do GitHub
</h3>

Adicione um repositório GitHub que contém um arquivo `.claude-plugin/marketplace.json` usando o formato `owner/repo`, onde `owner` é o nome de usuário ou organização do GitHub e `repo` é o nome do repositório.

Por exemplo, `anthropics/claude-code` refere-se ao repositório `claude-code` de propriedade de `anthropics`:

```shell theme={null}
/plugin marketplace add anthropics/claude-code
```

<h3 id="add-from-other-git-hosts">
  Adicione de outros hosts Git
</h3>

Adicione qualquer repositório git fornecendo a URL completa. Isso funciona com qualquer host Git, incluindo GitLab, Bitbucket e servidores auto-hospedados. Inclua o sufixo `.git` para que Claude Code clone o repositório em vez de tratar a URL como um link direto para um arquivo `marketplace.json` hospedado.

Inclua o prefixo `https://` também. Claude Code v2.1.196 e posterior rejeitam um host digitado sem ele, como `gitlab.com/company/plugins.git`, como um atalho `owner/repo` do GitHub inválido, e a mensagem de erro informa para adicionar o prefixo. Versões anteriores o interpretam incorretamente como um caminho de repositório do GitHub e falham no momento do clone.

Usando HTTPS:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

Usando SSH:

```shell theme={null}
/plugin marketplace add git@gitlab.com:company/plugins.git
```

Para adicionar um branch ou tag específico, acrescente `#` seguido pela ref:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git#v1.0.0
```

<h3 id="add-from-local-paths">
  Adicione de caminhos locais
</h3>

Adicione um diretório local que contém um arquivo `.claude-plugin/marketplace.json`:

```shell theme={null}
/plugin marketplace add ./my-marketplace
```

Você também pode adicionar um caminho direto para um arquivo `marketplace.json`:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace.json
```

<h3 id="add-from-remote-urls">
  Adicione de URLs remotas
</h3>

Adicione um arquivo `marketplace.json` remoto via URL:

```shell theme={null}
/plugin marketplace add https://example.com/marketplace.json
```

<Note>
  Marketplaces baseados em URL têm algumas limitações comparadas a marketplaces baseados em Git. Se você encontrar erros "path not found" ao instalar plugins, veja [Troubleshooting](/docs/pt/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces).
</Note>

<h2 id="install-plugins">
  Instale plugins
</h2>

Uma vez que você adicionou marketplaces, você pode instalar plugins diretamente:

```shell theme={null}
/plugin install plugin-name@marketplace-name
```

O comando abre os detalhes desse plugin, onde você escolhe um [escopo de instalação](/docs/pt/settings#configuration-scopes). Você vê as mesmas opções quando executa `/plugin`, vai para a aba **Discover** e pressiona **Enter** em um plugin:

* **User scope** (padrão): instale para você em todos os projetos
* **Project scope**: instale para todos os colaboradores neste repositório, o que adiciona o plugin a `.claude/settings.json`
* **Local scope**: instale para você neste repositório apenas, não compartilhado com colaboradores

Para instalar sem uma etapa interativa, use o comando shell [`claude plugin install`](/docs/pt/plugins-reference#plugin-install), que instala no escopo de usuário a menos que você passe `--scope`.

Você também pode ver plugins com escopo **managed**. Esses são instalados por administradores via [managed settings](/docs/pt/settings#settings-files) e não podem ser modificados.

<Warning>
  Certifique-se de confiar em um plugin antes de instalá-lo. Anthropic não controla quais MCP servers, arquivos ou outro software estão incluídos em plugins e não pode verificar que funcionam conforme pretendido. Verifique a página inicial de cada plugin para mais informações.
</Warning>

<h2 id="manage-installed-plugins">
  Gerencie plugins instalados
</h2>

Execute `/plugin` e vá para a aba **Installed** para visualizar, habilitar, desabilitar ou desinstalar seus plugins. A lista é agrupada por escopo e classificada para que você veja problemas primeiro: plugins com erros de carregamento ou dependências não resolvidas aparecem no topo, seguidos por seus favoritos, com plugins desabilitados dobrados atrás de um cabeçalho recolhido na parte inferior.

Da lista você pode:

* pressionar `f` para marcar como favorito ou desmarcar como favorito o plugin selecionado
* digitar para filtrar por nome ou descrição do plugin
* pressionar Enter para abrir a visualização de detalhes de um plugin e habilitar, desabilitar ou desinstalá-lo

Desinstalar um plugin que o `.claude/settings.json` de um projeto habilita pergunta qual escopo você quer dizer: desabilitá-lo apenas para você, o que escreve uma substituição em seu `.claude/settings.local.json` e deixa o plugin instalado para o projeto, ou desinstalá-lo para todos, o que o remove do `.claude/settings.json` compartilhado. Requer Claude Code v2.1.203 ou posterior. Antes de v2.1.203, o diálogo oferecia apenas a desabilitação local.

A visualização de detalhes mostra os componentes que o plugin contribui: comandos, skills, agentes, hooks, servidores MCP e servidores LSP. O mesmo inventário está disponível na linha de comando com `claude plugin details`.

A aba **Installed** também coleta plugins do marketplace que você instalou por conta própria, mas não usou em pelo menos duas semanas, em um período de pelo menos 10 sessões, sob um cabeçalho **Not used recently**. A visualização de detalhes mostra uma linha **Last used** para cada plugin. Use estes para encontrar plugins que ainda adicionam custo de inicialização e contexto, mesmo que você não os use mais, depois desabilite ou desinstale-os. Requer Claude Code v2.1.187 ou posterior.

Dois tipos de plugins nunca são listados como não utilizados:

* plugins que sua organização gerencia ou que você carrega com `--plugin-dir`
* plugins que contribuem um tema, estilo de saída, monitor ou workflow, já que entregam valor sem uma invocação para rastrear

O cabeçalho **Not used recently** e a linha **Last used** estão ambos ocultos quando sua organização restringe marketplaces com [`strictKnownMarketplaces`](/docs/pt/settings#strictknownmarketplaces).

Um [servidor de linguagem](/docs/pt/plugins#add-lsp-servers-to-your-plugin) de um plugin conta como usado quando entrega diagnósticos ou responde a uma solicitação de navegação de código, então um plugin LSP cujo servidor está ativo em suas sessões não é listado como não utilizado. Antes de v2.1.203, a atividade do servidor de linguagem não podia ser contada como uso, então plugins que contribuem um servidor LSP eram isentos do grupo inteiramente, da mesma forma que plugins de tema e estilo de saída ainda são.

A primeira sessão em uma versão que conta a atividade do servidor de linguagem também redefine o registro de uso de cada plugin LSP que ainda não havia registrado nenhum uso, então Claude Code não julga um plugin que você instalou anteriormente como não utilizado com base em dados registrados antes da atividade do servidor ser rastreada. Antes de v2.1.206, essa primeira sessão poderia listar um plugin LSP ativamente usado sob **Not used recently** e sugerir revisá-lo.

Quando você instala um plugin que declara dependências, a saída de instalação lista quais dependências foram auto-instaladas junto com ele.

Você também pode gerenciar plugins com comandos diretos.

Liste plugins instalados sem abrir o menu:

```shell theme={null}
/plugin list
```

Passe `--enabled` ou `--disabled` para mostrar apenas plugins nesse estado.

Desabilite um plugin sem desinstalá-lo:

```shell theme={null}
/plugin disable plugin-name@marketplace-name
```

Reabilite um plugin desabilitado:

```shell theme={null}
/plugin enable plugin-name@marketplace-name
```

Nestes identificadores, `plugin-name` é o `name` do plugin na [entrada do marketplace](/docs/pt/plugin-marketplaces#plugin-entries), que pode diferir do `name` no próprio `plugin.json` do plugin.

A partir do Claude Code v2.1.195, **Enable** e **Disable** na interface `/plugin` funcionam para plugins cujos dois nomes diferem, e `/plugin enable` e `/plugin disable` aceitam qualquer um dos nomes. Quando você desabilita tal plugin em uma versão anterior, Claude Code relata `already disabled` e o deixa habilitado.

Remova completamente um plugin:

```shell theme={null}
/plugin uninstall plugin-name@marketplace-name
```

A opção `--scope` permite que você direcione um escopo específico com comandos CLI:

```shell theme={null}
claude plugin install formatter@your-org --scope project
claude plugin uninstall formatter@your-org --scope project
```

<h3 id="apply-plugin-changes-without-restarting">
  Aplique mudanças de plugin sem reiniciar
</h3>

Quando você instala, habilita ou desabilita plugins durante uma sessão, execute `/reload-plugins` para ativar todas as mudanças sem reiniciar:

```shell theme={null}
/reload-plugins
```

Claude Code recarrega todos os plugins ativos e mostra contagens para plugins, skills, agentes, hooks, servidores MCP de plugin e servidores LSP de plugin.

O recarregamento tem um custo de token na próxima solicitação: componentes recém-carregados se anunciam no conteúdo anexado à conversa, enquanto o histórico existente ainda lê do cache de prompt. Um plugin que fornece servidores MCP custa mais quando suas ferramentas não são adiadas por [busca de ferramentas MCP](/docs/pt/mcp#scale-with-mcp-tool-search): a mudança invalida o cache e a próxima solicitação relê toda a conversa. Nesse caso `/reload-plugins` mostra um aviso e não aplica o recarregamento; passe `--force` para aplicar mesmo assim. Consulte [habilitando ou desabilitando um plugin](/docs/pt/prompt-caching#enabling-or-disabling-a-plugin) para obter detalhes.

<h2 id="manage-marketplaces">
  Gerencie marketplaces
</h2>

Você pode gerenciar marketplaces através da interface interativa `/plugin` ou com comandos CLI.

<h3 id="use-the-interactive-interface">
  Use a interface interativa
</h3>

Execute `/plugin` e vá para a aba **Marketplaces** para:

* Visualize todos os seus marketplaces adicionados com suas fontes e status
* Adicione novos marketplaces
* Atualize listagens de marketplace para buscar os plugins mais recentes
* Remova marketplaces que você não precisa mais

<h3 id="use-cli-commands">
  Use comandos CLI
</h3>

Você também pode gerenciar marketplaces com comandos diretos.

Liste todos os marketplaces configurados:

```shell theme={null}
/plugin marketplace list
```

Atualize listagens de plugins de um marketplace:

```shell theme={null}
/plugin marketplace update marketplace-name
```

Remova um marketplace:

```shell theme={null}
/plugin marketplace remove marketplace-name
```

<Warning>
  Remover um marketplace desinstalará quaisquer plugins que você instalou dele.
</Warning>

<h3 id="configure-auto-updates">
  Configure atualizações automáticas
</h3>

Claude Code pode atualizar automaticamente marketplaces e seus plugins instalados em segundo plano após a inicialização. Quando a atualização automática está habilitada para um marketplace, Claude Code atualiza os dados do marketplace e atualiza plugins instalados para suas versões mais recentes no disco.

Claude Code verifica atualizações de marketplace e plugins após sua sessão iniciar, com um atraso aleatório de até dez minutos, para que a sessão em execução continue usando as versões que carregou na inicialização. Se quaisquer plugins foram atualizados, você verá uma notificação solicitando que execute `/reload-plugins`, ou as novas versões carregam no seu próximo lançamento.

Alterne a atualização automática para marketplaces individuais através da UI:

1. Execute `/plugin` para abrir o gerenciador de plugins
2. Selecione **Marketplaces**
3. Escolha um marketplace da lista
4. Selecione **Enable auto-update** ou **Disable auto-update**

Marketplaces oficiais da Anthropic têm atualização automática habilitada por padrão. Marketplaces de terceiros e de desenvolvimento local têm atualização automática desabilitada por padrão.

Os administradores também podem definir `"autoUpdate": true` em cada entrada [`extraKnownMarketplaces`](/docs/pt/settings#extraknownmarketplaces) nas configurações gerenciadas para habilitar a atualização automática para um marketplace da organização sem exigir que cada usuário alterne.

Para desabilitar todas as atualizações automáticas inteiramente para Claude Code e todos os plugins, defina a variável de ambiente `DISABLE_AUTOUPDATER`. Veja [Auto updates](/docs/pt/setup#auto-updates) para detalhes.

Para manter atualizações automáticas de plugins habilitadas enquanto desabilita atualizações automáticas de Claude Code, defina `FORCE_AUTOUPDATE_PLUGINS=1` junto com `DISABLE_AUTOUPDATER`:

```bash theme={null}
export DISABLE_AUTOUPDATER=1
export FORCE_AUTOUPDATE_PLUGINS=1
```

Isso é útil quando você quer gerenciar atualizações de Claude Code manualmente mas ainda receber atualizações automáticas de plugins.

<h2 id="configure-team-marketplaces">
  Configurar marketplaces de equipe
</h2>

Administradores de equipe podem configurar instalação automática de marketplace para projetos adicionando configuração de marketplace a `.claude/settings.json`. Quando membros da equipe confiam na pasta do repositório, Claude Code os solicita a instalar esses marketplaces e plugins.

A partir de Claude Code v2.1.195, esta etapa de instalação se aplica em cada caminho que carrega plugins. Um plugin que apenas o `.claude/settings.json` do projeto habilita, e que vem de uma fonte externa como um repositório GitHub ou pacote npm, não carrega até que o membro da equipe o instale. Até então, Claude Code relata o plugin como não instalado e mostra o comando `claude plugin install` para executar.

Adicione `extraKnownMarketplaces` ao `.claude/settings.json` do seu projeto:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "my-team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Para opções de configuração completas incluindo `extraKnownMarketplaces` e `enabledPlugins`, veja [Plugin settings](/docs/pt/settings#plugin-settings).

<h2 id="security">
  Segurança
</h2>

Plugins e marketplaces são componentes altamente confiáveis que podem executar código arbitrário em sua máquina com seus privilégios de usuário. Instale apenas plugins e adicione marketplaces de fontes que você confia. Organizações podem restringir quais marketplaces os usuários podem adicionar usando [managed marketplace restrictions](/docs/pt/plugin-marketplaces#managed-marketplace-restrictions).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="/plugin-command-not-recognized">
  Comando /plugin não reconhecido
</h3>

Se você vir "unknown command" ou o comando `/plugin` não aparecer:

1. **Verifique sua versão**: Execute `claude --version` para ver o que está instalado.
2. **Atualize Claude Code**:
   * **Homebrew**: `brew upgrade claude-code`, ou `brew upgrade claude-code@latest` se você instalou esse cask
   * **npm**: `npm install -g @anthropic-ai/claude-code@latest`
   * **Native installer**: Re-execute o comando de instalação de [Setup](/docs/pt/setup)
3. **Reinicie Claude Code**: Após atualizar, reinicie seu terminal e execute `claude` novamente.

<h3 id="common-issues">
  Problemas comuns
</h3>

* **Marketplace não carregando**: Verifique se a URL está acessível e se `.claude-plugin/marketplace.json` existe no caminho
* **Falhas de instalação de plugin**: Verifique se as URLs de fonte do plugin estão acessíveis e que repositórios são públicos, ou que você tem acesso a eles
* **Arquivos não encontrados após instalação**: Plugins são copiados para um cache, então caminhos referenciando arquivos fora do diretório do plugin não funcionarão
* **Skills de plugin não aparecendo**: Limpe o cache com `rm -rf ~/.claude/plugins/cache`, reinicie Claude Code e reinstale o plugin.

Para troubleshooting detalhado com soluções, veja [Troubleshooting](/docs/pt/plugin-marketplaces#troubleshooting) no guia de marketplace. Para ferramentas de debugging, veja [Debugging and development tools](/docs/pt/plugins-reference#debugging-and-development-tools).

<h3 id="code-intelligence-issues">
  Problemas de code intelligence
</h3>

* **Language server não iniciando**: Verifique se o binário está instalado e disponível em seu `$PATH`. Verifique a aba Errors do `/plugin` para detalhes.
* **Alto uso de memória**: Language servers como `rust-analyzer` e `pyright` podem consumir memória significativa em projetos grandes. Se você experimentar problemas de memória, desabilite o plugin com `/plugin disable <plugin-name>` e confie nas ferramentas de busca integradas do Claude.
* **Diagnósticos falsos positivos em monorepos**: Language servers podem relatar erros de importação não resolvida para pacotes internos se o workspace não estiver configurado corretamente. Esses não afetam a capacidade do Claude de editar código.

<h2 id="next-steps">
  Próximos passos
</h2>

* **Construa seus próprios plugins**: Veja [Plugins](/docs/pt/plugins) para criar skills, agentes e hooks
* **Crie um marketplace**: Veja [Criar um marketplace de plugins](/docs/pt/plugin-marketplaces) para distribuir plugins para sua equipe ou comunidade
* **Referência técnica**: Veja [Plugins reference](/docs/pt/plugins-reference) para especificações completas
