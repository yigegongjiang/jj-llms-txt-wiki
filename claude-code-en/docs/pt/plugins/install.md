> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Instalar e gerenciar plugins

> Instale plugins do Claude Code a partir de um marketplace em qualquer superfície que você use, escolha um escopo de instalação e atualize ou remova-os posteriormente.

Instalar um plugin adiciona suas skills, agents, hooks e servidores MCP ao Claude Code na sua máquina.

Esta página é para qualquer pessoa que use plugins em sua própria máquina ou conta, seja no terminal, no aplicativo desktop, em um IDE ou em uma sessão na nuvem: ela cobre instalação, escolha de escopo, adição de marketplaces e manutenção de plugins atualizados.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Você usa o chat claude.ai ou Cowork, não Claude Code**: veja [Plugins no claude.ai e no Cowork](https://claude.com/docs/plugins/overview)
  * **Claude Code imprimiu um erro**: encontre-o em [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting)
</Note>

Comece com [Instalar um plugin](#install-a-plugin). Se alguém lhe enviou um comando de instalação cujo nome `@` não é `claude-plugins-official`, [adicione esse marketplace](#add-a-marketplace) primeiro.

<h2 id="install-a-plugin">
  Instalar um plugin
</h2>

Como exemplo, esta seção instala [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands) do [marketplace oficial da Anthropic](/docs/pt/plugins/anthropic-marketplaces), que adiciona comandos para fazer commit, fazer push e abrir pull requests.

Os mesmos passos instalam qualquer outro plugin: substitua seu nome e o nome do seu marketplace onde quer que `commit-commands` e `claude-plugins-official` apareçam. Se esse plugin vier de um marketplace diferente, [adicione o marketplace](#add-a-marketplace) primeiro.

Escolha a aba para onde você executa o Claude Code.

<Tabs>
  <Tab title="Terminal">
    Inicie o Claude Code com `claude` no seu projeto e depois:

    <Steps>
      <Step title="Abra os detalhes do plugin com o comando de instalação">
        Execute `/plugin install` com o nome do plugin e do marketplace. Em uma sessão, este comando não instala imediatamente: ele abre o painel `/plugin` nos detalhes desse plugin para que você possa revisá-lo e escolher um escopo primeiro.

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        Para navegar, execute `/plugin` sem nome de plugin: o painel abre na aba **Discover**, que lista plugins de cada marketplace que você adicionou, e você pode digitar para pesquisar, depois pressionar **Enter** em um plugin para abrir seus detalhes.
      </Step>

      <Step title="Revise o que o plugin adiciona">
        O painel de detalhes mostra a descrição do plugin. Ele também pode mostrar:

        * **Will install**: os comandos, agents, skills, hooks e servidores MCP e LSP que o plugin adiciona.
        * **Last updated**: mostrado para um plugin no marketplace oficial da Anthropic.
        * **Context cost**: para um plugin no marketplace oficial da Anthropic, duas estimativas de tokens. **Every turn** é o que o plugin adiciona a cada mensagem que você envia, e **When invoked** é o que suas skills e agents adicionam uma vez que o Claude os carrega. As estimativas aparecem quando você abre o plugin nomeando seu marketplace, como o comando da etapa 1 faz, ou na aba **Marketplaces**. O painel de detalhes que você alcança na lista **Discover** não as mostra.

        Plugins de um marketplace local ou personalizado podem mostrar `Components will be discovered at installation` em vez disso.

        Um plugin pode executar hooks e servidores MCP, então leia o painel antes de instalar. Veja [Segurança e confiança de plugins](/docs/pt/plugins/security).
      </Step>

      <Step title="Escolha um escopo">
        Selecione uma das três opções de instalação:

        * **Install for you (user scope)**: você obtém o plugin em cada projeto nesta máquina
        * **Install for all collaborators on this repository (project scope)**: ele é habilitado para todos que trabalham neste repositório
        * **Install for you, in this repo only (local scope)**: você o obtém apenas neste repositório

        [Escolha um escopo de instalação](#choose-an-install-scope) diz qual arquivo de configurações cada um escreve e qual se aplica quando o mesmo plugin é definido em mais de um.

        Depois de selecionar um escopo, o Claude Code instala o plugin junto com qualquer dependência que ele declara e imprime um resumo de instalação.
      </Step>

      <Step title="Leia o resumo de instalação">
        A última frase do resumo diz se o plugin é utilizável nesta sessão:

        * **Active now**: `Plugin is now active.` Nenhuma recarga é necessária.
        * **Reload needed**: `Run /reload-plugins to activate.` O painel fecha e o Claude Code executa essa recarga para você. Se a recarga [invalidasse o cache de prompt](/docs/pt/prompt-caching#enabling-or-disabling-a-plugin), ela avisa e deixa o plugin pendente. Execute `/reload-plugins --force` para ativá-lo mesmo assim, o que custa uma solicitação sem cache.
        * **Load failed**: `The plugin couldn't be loaded`. Abra a aba **Errors** em `/plugin` para saber o motivo e depois veja [Após instalação: plugin não funcionando](/docs/pt/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>

      <Step title="Confirme que o plugin funciona">
        Digite `/` e procure as skills do plugin sob seu nome, na forma `/<plugin>:<skill>`. Para `commit-commands`, `/commit-commands:commit` aparece. Dois outros lugares listam o plugin também:

        * Abra a aba **Installed** em `/plugin`, que lista o plugin com seu escopo.
        * No seu shell, execute `claude plugin list`, que imprime a mesma lista com linhas `Version`, `Scope` e `Status`.

        Se `/commit-commands:commit` não aparecer, veja [Após instalação: plugin não funcionando](/docs/pt/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>
    </Steps>

    Instalar de qualquer outro marketplace requer uma etapa extra primeiro: [adicione o marketplace](#add-a-marketplace). O Claude Code adiciona o marketplace oficial da Anthropic para você na primeira vez que você inicia uma sessão de terminal interativa, é por isso que o exemplo pula essa etapa. Se você encontrou um plugin em [claude.com/marketplace](https://claude.com/marketplace), seu botão **Claude Code** copia o comando de instalação em sua [forma de shell](#install-from-your-shell), `claude plugin install <name>@claude-plugins-official`.
  </Tab>

  <Tab title="Desktop app">
    Em uma sessão local ou SSH na aba **Code** do aplicativo desktop:

    <Steps>
      <Step title="Abra o navegador de plugins">
        Clique no botão **+** ao lado da caixa de prompt e selecione **Plugins**, depois **Add plugin**. O navegador de plugins abre com os plugins de seus marketplaces.
      </Step>

      <Step title="Selecione o plugin">
        Encontre `commit-commands` e selecione-o.
      </Step>

      <Step title="Escolha um escopo">
        Escolha um [escopo](#choose-an-install-scope): sua conta de usuário, este projeto ou apenas local.
      </Step>
    </Steps>

    Para habilitar, desabilitar ou desinstalar depois, use **+ > Plugins > Manage plugins**. O navegador de plugins não está disponível nas sessões na nuvem do aplicativo desktop. Veja [Instalar plugins no aplicativo desktop](/docs/pt/desktop#install-plugins).
  </Tab>

  <Tab title="VS Code">
    No painel do Claude Code no VS Code:

    <Steps>
      <Step title="Abra Manage plugins">
        Digite `/plugins` na caixa de prompt para abrir **Manage plugins**.
      </Step>

      <Step title="Instale o plugin">
        Na aba **Plugins**, procure por `commit-commands` e clique em **Install**. Se a aba não listar plugins, adicione `anthropics/claude-plugins-official` na aba **Marketplaces** primeiro.
      </Step>

      <Step title="Escolha um escopo">
        Escolha um [escopo](#choose-an-install-scope): **Install for you**, **Install for this project** ou **Install locally**.
      </Step>
    </Steps>

    Suas alterações se aplicam a sessões abertas sem reinicialização. Veja [Gerenciar plugins no VS Code](/docs/pt/vs-code#manage-plugins).
  </Tab>

  <Tab title="Cloud session">
    Uma [sessão na nuvem](/docs/pt/cloud-environments), incluindo [o navegador em claude.ai/code](/docs/pt/claude-code-on-the-web), não tem navegador de plugins e não carrega os plugins que você instalou em sua própria máquina ou os que o `.claude/settings.json` do seu repositório ativa. Para plugins que sua organização distribui através de configurações gerenciadas, veja [Gerenciar plugins para sua organização](/docs/pt/plugins/org).

    Veja [quais partes de sua configuração também estão disponíveis em uma sessão na nuvem](/docs/pt/cloud-environments#what-carries-over-from-your-setup) para o resto de sua configuração.
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  Escolha um escopo de instalação
</h3>

O escopo de instalação de um plugin decide quem obtém o plugin e qual arquivo de configurações o registra como habilitado:

* **User scope**: o plugin é habilitado para você em cada projeto nesta máquina. A entrada vai em `enabledPlugins` em `~/.claude/settings.json`.
* **Project scope**: o plugin é habilitado para todos que trabalham neste repositório. A entrada vai em `.claude/settings.json`, que você faz commit.
* **Local scope**: o plugin é habilitado para você apenas neste repositório. A entrada vai em `.claude/settings.local.json`.

Alguns plugins são definidos por seu autor para começar desligados, através do campo [`defaultEnabled`](/docs/pt/plugins/manifest-reference#defaultenabled). Tal plugin é instalado mas permanece desligado até que você o ative com `claude plugin enable <name>` no seu shell, ou na aba **Installed** de `/plugin` em uma sessão.

Quando o mesmo plugin é definido em vários escopos, a configuração local substitui a configuração do projeto, e a configuração do projeto substitui a configuração do usuário. Veja [Encontre onde um plugin é habilitado](/docs/pt/plugins/loading#find-where-a-plugin-is-enabled) para a regra completa.

O terminal, as sessões locais do aplicativo desktop e a extensão VS Code em um computador leem os mesmos arquivos de configurações, então um plugin que você instala em escopo de usuário em qualquer um deles está disponível nos outros dois.

<h3 id="other-places-you-run-claude-code">
  JetBrains, execuções não-interativas e o Agent SDK
</h3>

Alguns lugares onde você executa o Claude Code não têm navegador de plugins próprio:

* **JetBrains IDEs**: o plugin JetBrains executa o Claude Code no terminal do IDE, então use os passos da aba **Terminal** lá.
* **`claude -p` e outras execuções não-interativas**: `/plugin` não executa, e o Claude responde `/plugin isn't available in this environment.` Plugins que você já instalou carregam. Instale e gerencie-os do seu shell com [comandos `claude plugin`](#install-from-your-shell).
* **Agent SDK**: carregue plugins através da opção de plugin do SDK. Veja [Carregar plugins no Agent SDK](/docs/pt/agent-sdk/plugins).

Se o Claude Code relatar que um plugin habilitado no `.claude/settings.json` do repositório não está instalado, veja [Habilitado nas configurações do projeto mas não instalado](/docs/pt/plugins/loading#enabled-in-project-settings-but-not-installed).

<Tip>
  Se você é um autor de plugin testando uma cópia do seu plugin em disco, inicie o Claude Code do seu shell com `--plugin-dir` para carregá-lo por uma sessão em vez de instalá-lo. Veja [Flags que carregam um plugin por uma sessão](/docs/pt/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  Plugins da sua conta claude.ai
</h3>

Sua conta claude.ai é uma fonte separada de plugins, ao lado dos marketplaces que você instala:

* **O que chega**: cada plugin que você ativa para sua conta claude.ai, e cada plugin que sua organização ativa para seus membros. Em uma sessão de terminal eles sincronizam em segundo plano cada vez que você inicia o Claude Code enquanto conectado com essa conta; em sessões do Cowork eles baixam quando a sessão inicia.
* **Onde você os vê**: em `/plugin` e `claude plugin list` sob o ID `<name>@synced`. Você pode desativar um em seu próprio escopo a menos que sua organização o exija.
* **O que não vai para o outro lado**: plugins que você instala com `/plugin` ou `claude plugin install` permanecem nesta máquina e não são adicionados à sua conta claude.ai.

Para tempo de sincronização, requisitos de login e desativar sincronização, veja [Plugins sincronizados do claude.ai](/docs/pt/plugins/loading#synced-plugins).

<h3 id="install-from-your-shell">
  Instalar do seu shell
</h3>

Execute `claude plugin install` no seu shell para instalar um plugin sem iniciar uma sessão do Claude Code, por exemplo a partir de um script de configuração.

* **Scope**: escopo de usuário por padrão. Passe `--scope project` ou `--scope local` para alterá-lo.
* **Quando os plugins carregam**: plugins que ele instala carregam na próxima vez que você inicia o Claude Code, ou quando você executa `/reload-plugins` em uma sessão que já está aberta.
* **O marketplace deve ser adicionado primeiro**: em uma máquina onde ninguém abriu uma sessão interativa do Claude Code ainda, o marketplace oficial não está registrado, então um script que instala a partir dele executa `claude plugin marketplace add anthropics/claude-plugins-official` antes da instalação.

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

O comando imprime `Successfully installed plugin: formatter@your-org (scope: project)` quando termina.

Alguns plugins instalam executando um comando que seu marketplace nomeia, chamado de [`command` source](/docs/pt/plugins/marketplace-reference#command-plugin-source). O Claude Code mostra esse comando e pede que você o aceite antes de executá-lo. Um script não tem ninguém para responder esse prompt, então passe `--yes` lá para aceitá-lo.

Para cada flag `claude plugin install`, veja [plugin install](/docs/pt/plugins/cli-reference#plugin-install).

<h2 id="add-a-marketplace">
  Adicionar um marketplace
</h2>

Você só precisa desta seção quando o plugin que deseja não está no marketplace oficial da Anthropic, por exemplo um que um colega publicou ou um do marketplace da comunidade da Anthropic.

Um marketplace é um catálogo de plugins, e o Claude Code precisa saber sobre um marketplace antes de você poder instalar a partir dele. Você adiciona um marketplace uma vez. Depois disso, seus plugins aparecem na aba **Discover** e instalam com `/plugin install <plugin>@<marketplace>` em uma sessão ou `claude plugin install <plugin>@<marketplace>` no seu shell, onde `<marketplace>` é o nome que o marketplace se registrou. Para fazer ambos em uma etapa, veja [Adicionar um marketplace e instalar em um comando](#add-a-marketplace-and-install-in-one-command).

Em uma sessão do Claude Code, execute `/plugin marketplace add` seguido pela fonte do marketplace: um repositório GitHub, um repositório git em qualquer host, um diretório ou arquivo local, ou um `marketplace.json` hospedado.

| Fonte                            | O que você digita                                                                                                                                                                                                                                      | Exemplo                                                                                                                          |
| :------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| Repositório GitHub               | `owner/repo`. Adicione `#ref` para fixar um branch ou tag.                                                                                                                                                                                             | `/plugin marketplace add anthropics/claude-code`, ou `/plugin marketplace add your-org/plugins#v1.2.0` para fixar a tag `v1.2.0` |
| Repositório Git em qualquer host | A URL de clone completa. Adicione `#ref` para fixar um branch ou tag.                                                                                                                                                                                  | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                                      |
| Diretório ou arquivo local       | Um caminho relativo ou absoluto para um diretório que contém `.claude-plugin/marketplace.json`, ou para o arquivo JSON em si. Comece um caminho relativo com `./` ou `../`, porque o Claude Code lê um `name/name` simples como um repositório GitHub. | `/plugin marketplace add ./my-marketplace`                                                                                       |
| `marketplace.json` hospedado     | Sua URL `https://`                                                                                                                                                                                                                                     | `/plugin marketplace add https://example.com/marketplace.json`                                                                   |

Do seu shell, `claude plugin marketplace add` aceita as mesmas fontes.

<Tip>
  `/plugin market` também funciona como uma forma mais curta de `/plugin marketplace`.
</Tip>

Inclua o prefixo `https://` em cada URL, ou use a forma `git@host:path` para SSH. Se você digitar um `gitlab.example.com/your-group/your-marketplace.git` simples, o Claude Code o lê como atalho GitHub `owner/repo` e o rejeita.

Quando o comando é bem-sucedido, ele imprime `Successfully added marketplace: <name>`, e os plugins do marketplace aparecem na aba **Discover** na próxima vez que você abrir `/plugin`, sem necessidade de recarga. Se falhar, combine a mensagem de erro em [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting#add-a-marketplace).

<h3 id="add-a-marketplace-and-install-in-one-command">
  Adicionar um marketplace e instalar em um comando
</h3>

Para instalar um plugin de um marketplace que você ainda não adicionou, execute `/plugin install` em uma sessão do Claude Code e nomeie a fonte do marketplace com `--marketplace`. Requer Claude Code v2.1.275 ou posterior.

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

A fonte aceita [as mesmas formas que `/plugin marketplace add`](#add-a-marketplace), como GitHub `owner/repo`, uma URL git ou um caminho local, exceto que não pode conter espaços. Dê o nome do plugin por si só, sem um sufixo `@marketplace`.

Se você ainda não adicionou esse marketplace, o Claude Code mostra a fonte que resolveu e pede que você confirme antes de adicioná-lo. Uma vez que o marketplace é adicionado, os detalhes do plugin abrem e você escolhe um [escopo de instalação](#install-a-plugin). Se a fonte corresponder a um marketplace que você já adicionou, o Claude Code pula a confirmação e abre os detalhes do plugin nesse marketplace.

<h3 id="add-a-private-marketplace">
  Adicionar um marketplace privado
</h3>

Um marketplace privado é um em um repositório que você precisa de credenciais para clonar, no GitHub ou em qualquer outro host git. Você o adiciona com o mesmo comando `/plugin marketplace add` ou `claude plugin marketplace add` que um público. O Claude Code o clona com as credenciais git já em sua máquina e nunca solicita, então cada forma de conexão tem um requisito:

* **HTTPS**: seus ajudantes de credencial git se aplicam, então o acesso que você configurou com `gh auth login`, o Keychain do macOS ou `git-credential-store` funciona. Prompts interativos são suprimidos, então um host que você nunca autenticou falha em vez de pedir uma senha.
* **SSH**: o host já deve estar em seu arquivo `known_hosts` e a chave deve funcionar sem um prompt de frase-passe, porque os prompts de impressão digital do host e frase-passe também são suprimidos.
* **Atalho GitHub `owner/repo`**: o Claude Code verifica se sua chave SSH autentica em `github.com`, depois clona sobre SSH se fizer e sobre HTTPS se não fizer. Defina [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/pt/env-vars#variables) para pular essa verificação e sempre clonar sobre HTTPS.

As mesmas credenciais se aplicam quando você executa `/plugin install`, `/plugin marketplace update` e `claude plugin update`.

Em um host GitHub Enterprise Server, veja [Marketplaces de plugins no GHES](/docs/pt/github-enterprise-server#plugin-marketplaces-on-ghes) para as credenciais que cada operação precisa.

Se sua organização registra o marketplace para você através de configurações gerenciadas, você não o adiciona. Veja [Pré-instalar e exigir plugins](/docs/pt/plugins/org#pre-install-and-require-plugins).

<h3 id="add-from-claude-ai">
  Adicionar um marketplace do claude.ai
</h3>

Em sessões de terminal onde [plugins sincronizam da sua conta claude.ai](/docs/pt/plugins/loading#synced-plugins), claude.ai também pode listar marketplaces de plugins para você, como a biblioteca de plugins da sua organização e seus uploads próprios do claude.ai. Você adiciona um desses pelo seu nome em vez de por uma fonte. Adicionar um marketplace do claude.ai requer Claude Code v2.1.273 ou posterior.

Adicione um marketplace do claude.ai do painel `/plugin` ou do seu shell:

* **Dentro de uma sessão**: execute `/plugin` e vá para a aba **Marketplaces**, que lista os marketplaces do claude.ai. Selecione um lá para adicioná-lo.
* **Do seu shell**: execute `claude plugin marketplace list`, que os imprime em uma seção `From claude.ai:`. Depois execute `claude plugin marketplace add` com a flag `--claudeai` e o nome mostrado na lista.

Por exemplo, este comando adiciona um marketplace nomeado `claudeai-organization-library`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

O Claude Code registra o marketplace sob um nome local que começa com `claudeai-`, derivado do nome que claude.ai o lista. Por exemplo, um marketplace listado como "Organization library" se torna `claudeai-organization-library`. Instale seus plugins por esse nome, por exemplo com `claude plugin install <plugin>@claudeai-organization-library`.

Se você sair, ou entrar em uma organização claude.ai diferente, o marketplace permanece configurado mas não mostra plugins, e os plugins que você já instalou a partir dele continuam carregando.

A seção `From claude.ai:` também pode listar marketplaces baseados em git compartilhados através do claude.ai, e imprime uma fonte para cada um desses. Adicione-os por essa fonte como em [Adicionar um marketplace](#add-a-marketplace), não com `--claudeai`.

<h2 id="manage-installed-plugins">
  Gerenciar plugins instalados
</h2>

A aba **Installed** em `/plugin` lista seus plugins com ações para habilitar, desabilitar, atualizar ou desinstalar cada um. Em uma sessão do Claude Code, execute `/plugin` e pressione **Tab** para alcançá-la, ou execute `/plugin enable`, `/plugin disable` ou `/plugin uninstall` para abrir o painel e fazer essa alteração lá. Plugins desabilitados são agrupados sob um cabeçalho recolhido na parte inferior da lista. Use estas teclas na lista:

* Digite para filtrar por nome ou descrição.
* Pressione **Space** para habilitar ou desabilitar o plugin selecionado, e **f** para favoritá-lo.
* Pressione **Enter** para abrir os detalhes de um plugin. O menu lá oferece **Disable plugin** ou **Enable plugin**, **Update now** e **Uninstall**. Plugins que aceitam configurações também oferecem **Configure options**.

A aba também pode mostrar plugins em escopo **Managed**. Sua organização os instalou através de [configurações gerenciadas](/docs/pt/settings#settings-files), e você não pode habilitá-los, desabilitá-los ou desinstalá-los aqui.

Para um plugin sincronizado que sua organização exige no claude.ai, veja [Gerenciar plugins sincronizados do claude.ai](#manage-plugins-synced-from-claude-ai).

Quando você fecha o painel `/plugin` com alterações pendentes que você fez nele, o Claude Code executa `/reload-plugins` para você aplicá-las. Se a recarga [invalidasse o cache de prompt](/docs/pt/prompt-caching#enabling-or-disabling-a-plugin), ela avisa e deixa as alterações pendentes. Execute `/reload-plugins --force` para aplicá-las mesmo assim.

<h3 id="manage-plugins-synced-from-claude-ai">
  Gerenciar plugins sincronizados do claude.ai
</h3>

A aba **Installed** em `/plugin` também lista os [plugins sincronizados da sua conta claude.ai](/docs/pt/plugins/loading#synced-plugins), com `synced` como sua fonte. Plugins sincronizados aparecem em sessões de terminal no Claude Code v2.1.273 ou posterior.

* **Habilitar ou desabilitar**: use a aba **Installed**, a menos que sua organização tenha marcado o plugin como obrigatório.
* **Remover**: desative o plugin no claude.ai.

Quando o Claude Code sincroniza um plugin adicionado, atualizado ou removido em uma sessão interativa, você vê `Plugins changed. Run /reload-plugins to activate.` Execute `/reload-plugins` para carregar a alteração nessa sessão, ou deixe para a próxima vez que você iniciar o Claude Code.

<h3 id="uninstall-a-plugin-the-project-enables">
  Desinstalar um plugin que o projeto habilita
</h3>

Quando você escolhe **Uninstall** para um plugin que o `.claude/settings.json` deste repositório habilita, seja na aba **Installed** ou com `/plugin uninstall`, o Claude Code pergunta se deve desabilitá-lo para você ou desinstalá-lo para todos:

* **Disable for me**: pressione **y**. O Claude Code escreve `false` para o plugin em seu `.claude/settings.local.json` e o deixa instalado para o projeto.
* **Uninstall for everyone**: pressione **u**. O Claude Code remove o plugin do `.claude/settings.json` compartilhado.

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  Veja o que um plugin instalado adiciona às suas sessões
</h3>

No seu shell, execute `claude plugin details <name>` para um plugin instalado. A linha `Always-on` é o número de tokens que o plugin adiciona a cada sessão onde está habilitado, e as linhas por componente mostram qual skill ou agent contribui mais. Para a saída completa e o que cada figura significa, veja [Medir o que um plugin custa](/docs/pt/plugins/measure#measure-what-a-plugin-costs).

<h3 id="find-plugins-you-no-longer-use">
  Encontre plugins que você não usa mais
</h3>

Na aba **Installed** em `/plugin`, plugins que você instalou e não usou recentemente aparecem sob um cabeçalho **Not used recently**, e os detalhes de cada plugin mostram uma linha **Last used**. Use esse cabeçalho e essa linha para encontrar plugins que ainda adicionam custo de inicialização e contexto, depois desabilite ou desinstale-os.

<h3 id="plugins-with-dependencies">
  Plugins com dependências
</h3>

Um plugin pode declarar outros plugins dos quais depende. Quando você instala, desabilita ou desinstala tal plugin de um marketplace, o Claude Code age sobre essas dependências também:

* **Install**: o Claude Code também instala e habilita as dependências declaradas do plugin no mesmo escopo. A mensagem de sucesso as lista.
* **Enable**: o Claude Code também habilita as dependências do plugin que estão instaladas mas desabilitadas. Se uma dependência declarada não está instalada, a habilitação falha e a mensagem diz para instalá-la primeiro.
* **Disable**: quando outro plugin habilitado ainda precisa do que você nomeou, o Claude Code recusa e imprime um comando encadeado que desabilita ambos na ordem correta.
* **Uninstall**: dependências auto-instaladas permanecem até que você execute `claude plugin prune` no seu shell; veja [plugin prune](/docs/pt/plugins/cli-reference#plugin-prune).

Se você carregou o plugin com `--plugin-dir` em vez disso, veja [Teste um plugin e sua dependência localmente](/docs/pt/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="manage-plugins-from-your-shell">
  Gerenciar plugins do seu shell
</h3>

Você também pode gerenciar plugins sem iniciar uma sessão do Claude Code. No seu shell, execute `claude plugin install`, `enable`, `disable` ou `uninstall` como comandos de terminal ordinários; eles alteram as mesmas configurações que o painel `/plugin` faz. Cada um aceita `--scope` para direcionar um escopo, e usa um escopo padrão quando você o omite:

* `enable` e `disable` agem no escopo mais específico cujas configurações já listam o plugin.
* `install` e `uninstall` agem no escopo de usuário.

Por exemplo, estes comandos desabilitam e reabilitam um plugin, depois o desinstalam no escopo do projeto:

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  Manter plugins atualizados
</h2>

Plugins atualizam automaticamente quando o marketplace do qual vieram tem auto-update ativado. Depois que uma sessão inicia, o Claude Code atualiza esses marketplaces e atualiza as cópias em disco dos plugins que você instalou a partir deles.

A sessão em execução mantém as versões que já carregou. Após uma atualização, você vê `Plugin updated: <name> · Run /reload-plugins to apply`, e a próxima sessão carrega as novas versões automaticamente.

Estes são os padrões de auto-update para cada tipo de marketplace:

* **On by default**: `claude-plugins-official` e os outros [nomes de marketplace oficial](/docs/pt/plugins/security#official-marketplace-names) exceto `knowledge-work-plugins` e `first-party-plugins`, mais [marketplaces adicionados do claude.ai](#add-from-claude-ai).
* **Off by default**: cada outro marketplace, incluindo o marketplace da comunidade, marketplaces de terceiros e marketplaces de desenvolvimento local.

Para quando auto-update executa, quais plugins ele pula e as variáveis de ambiente que o desativam, veja [Quando auto-update executa](/docs/pt/plugins/loading#when-auto-update-runs).

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  Ativar ou desativar auto-update para um marketplace
</h3>

Em uma sessão do Claude Code, execute `/plugin` e vá para a aba **Marketplaces**. Selecione o marketplace, depois selecione **Enable auto-update** ou **Disable auto-update**.

<h3 id="update-one-plugin-now">
  Atualizar um plugin agora
</h3>

Em uma sessão, abra o plugin na aba **Installed** em `/plugin` e selecione **Update now**, ou no seu shell execute `claude plugin update <plugin>@<marketplace>`.

<h3 id="auto-update-from-a-private-marketplace">
  Auto-update de um marketplace privado
</h3>

Para um marketplace privado, veja [O que auto-update em segundo plano faz com credenciais](/docs/pt/plugins/host-marketplace#what-background-auto-update-does-with-credentials) para como auto-updates em segundo plano autenticam sobre SSH e HTTPS, e [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting#add-a-marketplace) para as mensagens que você vê quando falham.

<h2 id="manage-marketplaces">
  Gerenciar marketplaces
</h2>

A aba **Marketplaces** em `/plugin` lista cada marketplace que você registrou, junto com sua fonte. Selecione um para navegar seus plugins, atualizar sua listagem, ativar ou desativar auto-update, ou removê-lo.

Você também pode listar, atualizar e remover marketplaces com comandos, do seu shell ou dentro de uma sessão:

| Ação                                   | No seu shell                              | Dentro de uma sessão                |
| :------------------------------------- | :---------------------------------------- | :---------------------------------- |
| Listar marketplaces                    | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| Atualizar a listagem de um marketplace | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| Remover um marketplace                 | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

Quando você remove um marketplace, o Claude Code desinstala cada plugin que você instalou a partir dele e remove suas entradas `enabledPlugins` de seus arquivos de configurações. A aba **Marketplaces** nomeia esses plugins antes de pedir que você confirme.

<h2 id="next-steps">
  Próximos passos
</h2>

* [Marketplaces da Anthropic](/docs/pt/plugins/anthropic-marketplaces): como os marketplaces oficial, da comunidade e de demonstração diferem e onde navegar cada um
* [Referência de carregamento de plugins](/docs/pt/plugins/loading): por que um plugin carregou, não carregou ou não mudou após uma atualização
* [Segurança e confiança de plugins](/docs/pt/plugins/security): o que revisar antes de instalar um plugin de um marketplace que você não conhece
* [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting): instalar e mensagens de erro de marketplace com suas correções
* [Criar um plugin](/docs/pt/plugins/create): construa o seu próprio
