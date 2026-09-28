> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Solucionar problemas de plugins

> Corrija erros de plugins no Claude Code. Encontre a mensagem exata que você viu, agrupada por estágio desde onde /plugin é executado até a instalação e política da organização.

Esta página lista mensagens de erro e sintomas para plugins do Claude Code e para marketplaces, os catálogos dos quais o Claude Code instala plugins. Cada entrada fornece a causa, uma correção e o que você vê após a correção funcionar.

Quando uma mensagem nomeia um plugin ou marketplace, a entrada mostra um espaço reservado como `<name>` em vez disso.

Use esta página se você instalar plugins, construí-los, hospedar um marketplace ou administrar plugins para uma organização.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Por que escopos, o cache e precedência se comportam da maneira que fazem**: leia [Plugin loading reference](/docs/pt/plugins/loading)
  * **Procurando por um sinalizador, campo ou comando**: use a [plugin commands reference](/docs/pt/plugins/cli-reference), a [manifest reference](/docs/pt/plugins/manifest-reference), ou a [marketplace reference](/docs/pt/plugins/marketplace-reference)
</Note>

Procure pela mensagem exata que você viu. Cada mensagem é listada sob o estágio que a produz, o que nem sempre é o comando que você executou. Por exemplo, uma instalação pode falhar porque um marketplace está faltando, então essa mensagem está sob [Add a marketplace](#add-a-marketplace).

<h2 id="find-where-/plugin-runs">
  Find where `/plugin` runs
</h2>

`/plugin` é um comando que você digita dentro de uma sessão de terminal do Claude Code em execução, e abre um painel interativo. As entradas nesta seção cobrem os lugares onde você pode digitá-lo, mas ele não pode ser executado, e os comandos que não existem.

<h3 id="plugin-isnt-available-in-this-environment">
  `/plugin isn't available in this environment`
</h3>

Você digitou `/plugin` em algum lugar que não seja uma sessão de terminal do Claude Code, e Claude respondeu com esta linha em vez de abrir qualquer coisa.

Você recebe esta resposta em uma sessão que não tem terminal para desenhar o painel `/plugin`: [non-interactive mode](/docs/pt/headless) com `claude -p`, o Agent SDK, a aba Code do aplicativo Claude desktop, o painel de extensão do VS Code e o navegador em claude.ai/code.

No painel de extensão do VS Code, apenas uma linha `/plugin` com algo depois dela, como `/plugin install <plugin>@<marketplace>`, recebe esta resposta. `/plugin` ou `/plugins` digitados sozinhos abre o diálogo **Manage plugins**.

Instale o plugin a partir da superfície em que você está:

* **Aplicativo Claude desktop, sessão local ou SSH**: clique no botão **+** ao lado do prompt, depois **Plugins**, depois **Add plugin** para abrir o [plugin browser](/docs/pt/desktop#install-plugins)
* **Extensão VS Code**: use a aba **VS Code** em [Install a plugin](/docs/pt/plugins/install#install-a-plugin)
* **Claude Code na web ou uma sessão cloud desktop**: uma sessão cloud não tem navegador de plugins. Veja a aba **Cloud session** em [Install a plugin](/docs/pt/plugins/install#install-a-plugin) para o que uma sessão cloud carrega
* **Um terminal que você tem acesso**: execute `claude` e digite `/plugin` lá, ou execute `claude plugin install <plugin>@<marketplace>` no seu shell sem iniciar uma sessão

Quando uma instalação de terminal funciona, `/plugin` imprime um resumo de instalação que começa com `✓ Installed <plugin>.` e `claude plugin install` imprime `Successfully installed plugin: <plugin>@<marketplace>`.

<h3 id="zsh-no-such-file-or-directory-plugin">
  `zsh: no such file or directory: /plugin`
</h3>

Você digitou `/plugin ...` em um prompt de shell, e o shell relatou que nenhum arquivo chamado `/plugin` existe. Bash relata `bash: /plugin: No such file or directory`.

`/plugin` é um comando que você digita dentro de uma sessão do Claude Code, não em um prompt de shell. Inicie uma sessão e digite o mesmo comando lá:

```shell theme={null}
claude
```

Depois, no prompt do Claude Code:

```text theme={null}
/plugin install <plugin>@<marketplace>
```

Uma instalação bem-sucedida imprime um resumo que começa com `✓ Installed <plugin>.` Se a instalação em si falhar, sua mensagem está em [Add a marketplace](#add-a-marketplace) ou [Install a plugin](#install-a-plugin).

Para instalar a partir do shell sem iniciar uma sessão, execute `claude plugin install <plugin>@<marketplace>` em vez disso.

<h3 id="the-term-plugin-is-not-recognized-as-the-name-of-a-cmdlet">
  `The term '/plugin' is not recognized as the name of a cmdlet`
</h3>

Você digitou `/plugin ...` em um prompt do PowerShell, e `/plugin` é um comando do Claude Code, não um programa. Bash e Zsh relatam [sua própria forma deste erro](#zsh-no-such-file-or-directory-plugin).

Use um destes em vez disso:

* Execute `claude`, depois digite `/plugin` no prompt do Claude Code
* Execute `claude plugin install <plugin>@<marketplace>` no PowerShell sem iniciar uma sessão

<h3 id="claude-command-not-found-after-claude-plugin">
  `claude: command not found` after `claude plugin ...`
</h3>

Você executou `claude plugin install ...` no seu shell, e o shell não conseguiu encontrar `claude` em absoluto. No Windows a mensagem é `'claude' is not recognized as the name of a cmdlet` ou `'claude' is not recognized as an internal or external command`.

A causa não é o comando do plugin. Ou o Claude Code não está instalado, ou seu diretório de instalação não está no seu `PATH` neste shell. Siga [`command not found: claude` after installation](/docs/pt/troubleshoot-install#command-not-found-claude-after-installation), depois tente novamente o comando do plugin.

<h3 id="unknown-command-and-command-spellings-that-dont-exist">
  `Unknown command` and command spellings that don't exist
</h3>

Você digitou um comando de plugin que viu em algum lugar e recebeu `Unknown command: /<name>` em uma sessão, ou `error: unknown command '<name>'` ou `error: unknown option '<flag>'` do binário `claude` no seu shell.

Vários comandos estão em uso que o Claude Code não possui. A tabela abaixo mapeia cada um para o comando real. A [plugin commands reference](/docs/pt/plugins/cli-reference) lista todos os subcomandos e sinalizadores.

| Você digitou                               | O que Claude Code diz                                                        | Use em vez disso                                                                                                                                  |
| :----------------------------------------- | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `claude plugin add <source>`               | `error: unknown command 'add'`                                               | `claude plugin marketplace add <source>` para adicionar um marketplace, ou `claude plugin install <plugin>@<marketplace>` para instalar um plugin |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'`                                          | `claude plugin install <plugin>@<marketplace> --scope project`                                                                                    |
| `/install <plugin>`                        | `Unknown command: /install`                                                  | `/plugin install <plugin>@<marketplace>`                                                                                                          |
| `/plugin add <source>`                     | O painel `/plugin` abre na aba **Discover**                                  | `/plugin marketplace add <source>`                                                                                                                |
| `marketplace.anthropic.com` como uma fonte | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | `anthropics/claude-plugins-official` para o marketplace oficial                                                                                   |

Estes comandos parecem errados mas funcionam:

* `claude plugins` é um alias de `claude plugin`
* `claude plugin remove` é um alias de `claude plugin uninstall`
* `/plugins` e `/marketplace` em uma sessão abrem o mesmo painel que `/plugin`

<h2 id="add-a-marketplace">
  Add a marketplace
</h2>

Um marketplace é um catálogo que você adiciona ao Claude Code a partir de um repositório git, uma URL ou um caminho local. Estas entradas cobrem as mensagens que você recebe quando adicionar um falha ou uma atualização posterior falha.

<h3 id="marketplace-claude-plugins-official-not-found">
  `Marketplace "claude-plugins-official" not found`
</h3>

Você executou `/plugin install <plugin>@claude-plugins-official` em uma sessão, e Claude Code relatou que não tem um marketplace com esse nome.

O marketplace oficial não está registrado nesta máquina ainda. Claude Code normalmente o registra por conta própria na primeira vez que você inicia uma sessão de terminal interativa. Ele não foi executado ainda se você só usou Claude Code através da extensão VS Code, e pula ou adia essa etapa:

* Quando uma política bloqueia a fonte
* Quando `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` está definido
* Após uma tentativa falhada que está aguardando para tentar novamente

Os comandos de shell `claude plugin` nunca o registram para você.

Adicione-o, depois tente novamente a instalação:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: claude-plugins-official`, e `/plugin marketplace list` mostra o marketplace com sua fonte.

Para qualquer outro nome de marketplace nesta mensagem, veja [`Marketplace "<name>" not found`](#marketplace-not-found).

A mesma string também aparece na aba **Errors** do `/plugin`, a lista de falhas de carregamento do painel, quando um plugin listado em suas configurações nomeia um marketplace que você não adicionou.

<h3 id="marketplace-not-found">
  `Marketplace "<name>" not found`
</h3>

Você executou `/plugin install <plugin>@<name>` em uma sessão, frequentemente a partir de uma linha de instalação que alguém enviou para você, e Claude Code relatou que não tem um marketplace com esse nome.

Se o nome começar com `claudeai-`, o marketplace é hospedado em claude.ai, e você o adiciona pelo nome a partir do seu shell com `claude plugin marketplace add --claudeai <name>`. Veja [Add a marketplace from claude.ai](/docs/pt/plugins/install#add-from-claude-ai).

Para qualquer outro nome, uma linha de instalação nomeia um marketplace mas não diz onde o marketplace é hospedado, e Claude Code não tem um índice para procurar um nome de marketplace. Peça a quem enviou a linha pela fonte do marketplace, que é um GitHub `owner/repo`, uma URL git ou um caminho. Depois [adicione o marketplace](/docs/pt/plugins/install#add-a-marketplace) e execute a linha de instalação novamente.

Um marketplace que alguém envia para você é de terceiros, então [revise o plugin antes de instalá-lo](/docs/pt/plugins/security#review-a-plugin-before-you-install).

Se você já adicionou o marketplace, verifique a ortografia contra `/plugin marketplace list`.

<h3 id="invalid-marketplace-source-format">
  `Invalid marketplace source format`
</h3>

Você executou `/plugin marketplace add <source>` ou `claude plugin marketplace add <source>`, e Claude Code respondeu `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`.

Claude Code aceita uma fonte em uma destas formas:

* Um atalho GitHub `owner/repo`
* Uma URL `https://` ou `http://`
* Uma URL SSH `user@host:path`
* Um caminho local começando com `./`, `../`, `/` ou `~`

Um nome simples como `claude-plugins-official` não corresponde a nenhum deles. Nem um nome de host simples como `marketplace.anthropic.com`.

Redigite a fonte em uma das formas aceitas:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: <name>` quando a adição funciona.

<h3 id="is-not-a-valid-github-owner-repo-shorthand">
  `'<source>' is not a valid GitHub owner/repo shorthand`
</h3>

Você passou uma fonte com uma barra que não é `owner/repo`, como `github.com/owner/repo` ou um caminho `gitlab.example.com/group/project`. Claude Code a recusou com esta mensagem e uma lista de formas aceitas.

O atalho `owner/repo` é apenas para GitHub e tem que seguir as regras de nomenclatura do GitHub, então um nome de host ou um segmento de caminho extra falha. Passe a fonte na forma que corresponde a onde o marketplace é hospedado:

* **Um repositório em qualquer host**: a URL de clone completa
* **Um `marketplace.json` hospedado**: sua URL `https://`
* **Um checkout local**: `./path` ou um caminho absoluto

Por exemplo, para adicionar o marketplace oficial pela sua URL de clone, em uma sessão:

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

Uma adição bem-sucedida imprime `Successfully added marketplace: <name>`.

<h3 id="path-does-not-exist">
  `Path does not exist: <path>`
</h3>

Você passou um caminho local para `marketplace add`, e nada existe nesse caminho. Um caminho relativo resolve contra seu diretório atual.

Verifique o caminho resolvido na mensagem. Depois execute o comando a partir do diretório do qual o caminho relativo começa, ou passe um caminho absoluto para o diretório do marketplace. Uma adição bem-sucedida imprime `Successfully added marketplace: <name>`.

Claude Code aceita um diretório que contém `.claude-plugin/marketplace.json`, ou um caminho para um arquivo `.json`. Um caminho para qualquer outro arquivo falha com `File path must point to a .json file (marketplace.json)`.

<h3 id="marketplace-file-not-found-at-claude-plugin-marketplace-json">
  `Marketplace file not found at <path>/.claude-plugin/marketplace.json`
</h3>

Claude Code clonou ou baixou o marketplace mas não encontrou `marketplace.json` no caminho esperado dentro dele. O comando add relata como `Failed to add marketplace: Marketplace file not found at ...`.

O local padrão é `.claude-plugin/marketplace.json` na raiz do repositório, e a [marketplace reference](/docs/pt/plugins/marketplace-reference) lista os locais aceitos.

A correção difere para o proprietário e para todos os outros:

* **Você é o proprietário do marketplace**: coloque o arquivo nesse local e re-adicione o marketplace
* **Alguém mais o hospeda**: peça ao proprietário pela fonte exata que eles publicam

<h3 id="ssh-authentication-failed-or-https-authentication-failed">
  `SSH authentication failed` or `HTTPS authentication failed`
</h3>

Você adicionou ou atualizou um marketplace a partir de um repositório git, e o clone falhou com `Failed to clone marketplace repository:` seguido por uma destas linhas.

Primeiro verifique o repositório em si: um `owner/repo` digitado errado, um repositório que não existe ou um repositório privado que você não consegue ver também termina nesta mensagem. Abra a URL do repositório no seu navegador, ou execute `git ls-remote <url>` no seu terminal, para confirmar que existe e você tem acesso.

Se o repositório está correto, a causa é credenciais. Claude Code executa git com prompts interativos desabilitados, então não consegue pedir uma senha, uma frase de passagem de chave ou uma credencial da maneira que seu terminal faria. Se git precisa fazer um prompt, você vê `fatal: Cannot prompt because user interactivity has been disabled` ou `terminal prompts disabled` no erro original. Apenas credenciais que já funcionam de forma não-interativa têm sucesso:

* **SSH**: `ssh -T git@<host>` deve ter sucesso sem pedir uma frase de passagem, e o host já deve estar em `known_hosts`
* **HTTPS**: seu auxiliar de credencial deve manter um token para o host. Para GitHub, execute `gh auth login` e `gh auth setup-git`. Para outro host, armazene um token de acesso pessoal no seu auxiliar de credencial git. Teste com `git ls-remote <url>`

Uma vez que `git ls-remote` tenha sucesso no seu terminal sem um prompt, execute a adição ou atualização novamente. Uma adição bem-sucedida imprime `Successfully added marketplace: <name>`. Uma atualização bem-sucedida imprime `Successfully updated marketplace: <name>` a partir do seu shell, ou `✔ Updated 1 marketplace` em uma sessão.

Para fazer Claude Code pular SSH para fontes GitHub `owner/repo`, defina `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`. Sem isso, Claude Code clona essas fontes sobre SSH quando uma chave SSH para `github.com` parece estar configurada, e volta para HTTPS quando o clone SSH falha.

Para o que as atualizações automáticas de fundo podem e não podem fazer com suas credenciais, veja [What background auto-update does with credentials](/docs/pt/plugins/host-marketplace#what-background-auto-update-does-with-credentials).

<h3 id="ssh-host-key-is-not-in-your-known-hosts-file">
  `SSH host key is not in your known_hosts file`
</h3>

Você adicionou um marketplace sobre SSH a partir de um host ao qual nunca se conectou, e o clone falhou com esta linha e uma dica `ssh -T git@<host>`. Para um host cuja chave mudou, a mensagem é `SSH host key has changed` com uma dica `ssh-keygen -R <host>` em vez disso.

Claude Code clona com `StrictHostKeyChecking=yes`, então recusa um host cuja chave você ainda não aceitou em vez de aceitar a chave automaticamente. Conecte uma vez a partir do seu terminal para aceitar a impressão digital, depois tente novamente:

```shell theme={null}
ssh -T git@github.com
```

Para um repositório público, adicione o marketplace pela sua URL `https://` em vez disso para evitar SSH completamente.

<h3 id="command-git-not-found-or-is-in-an-unsafe-location">
  `Command 'git' not found or is in an unsafe location`
</h3>

No Windows, você adicionou um marketplace e Claude Code relatou `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)`.

Claude Code procura por `git` no seu `PATH` e recusa executar um encontrado apenas no diretório atual. Para corrigi-lo, instale Git e tente novamente:

<Steps>
  <Step title="Install Git for Windows">
    Instale Git for Windows para que `git` esteja no seu `PATH`.
  </Step>

  <Step title="Open a new terminal">
    Abra um novo terminal para que o `PATH` atualizado se aplique.
  </Step>

  <Step title="Confirm git runs">
    Confirme que `git --version` imprime uma versão.
  </Step>

  <Step title="Retry the add">
    Execute o comando `marketplace add` novamente.
  </Step>
</Steps>

<h3 id="git-clone-timed-out-after-120s">
  `Git clone timed out after 120s`
</h3>

Você adicionou ou atualizou um marketplace, e falhou com `Git clone timed out after 120s`, seguido por uma dica para definir `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`.

Clonar um marketplace, e re-clonar um para atualizá-lo, recebe 120 segundos por padrão. Para um repositório grande ou uma conexão lenta, aumente o limite. O valor está em milissegundos:

<Tabs>
  <Tab title="Bash or Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
    ```
  </Tab>
</Tabs>

Depois tente novamente no mesmo shell.

Se o repositório é um monorepo, limite o checkout aos diretórios que você nomeia com `claude plugin marketplace add <source> --sparse <paths>`.

<h3 id="marketplace-updates-keep-failing-offline">
  Marketplace updates keep failing offline
</h3>

Você trabalha em um ambiente onde o host git do marketplace é inacessível, e cada sessão repete uma atualização falhada em segundo plano. Seu checkout existente do marketplace permanece no lugar e a inicialização não é atrasada.

Cada sessão, para um marketplace com [auto-update on](/docs/pt/plugins/loading#which-marketplaces-and-plugins-auto-update), Claude Code verifica o host git do marketplace para novos commits em segundo plano. Quando essa verificação não consegue alcançar o host, ela tenta clonar o marketplace novamente, e offline esse clone também falha.

Defina esta variável para pular a tentativa de re-clone e continuar usando o checkout existente quando a verificação não conseguir alcançar o host:

<Tabs>
  <Tab title="Bash or Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
    ```
  </Tab>
</Tabs>

Com a variável definida, Claude Code pula o re-clone apenas para um checkout que já contém `.claude-plugin/marketplace.json`. Um marketplace que nunca foi clonado ou cujo clone parou no meio ainda recebe a tentativa de clone, então adicione-o uma vez enquanto online.

Para uma implantação totalmente offline, pré-popule o diretório de plugins no tempo de construção da imagem com `CLAUDE_CODE_PLUGIN_SEED_DIR` em vez disso, seguindo [Seed containers and CI](/docs/pt/plugins/org#seed-containers-and-ci).

<h3 id="marketplace-add-fails-on-a-github-enterprise-server-host">
  Marketplace add fails on a GitHub Enterprise Server host
</h3>

Você adicionou um marketplace a partir de uma URL do GitHub Enterprise Server (GHES) e recebeu um erro de política, ou o adicionou a partir de claude.ai e recebeu um erro de acesso ao GitHub.

Ambos os casos estão na página GHES:

* [A policy error](/docs/pt/github-enterprise-server#marketplace-add-fails-with-a-policy-error) significa que sua organização restringiu fontes de marketplace e um administrador precisa adicionar um `hostPattern` para o host
* [A GitHub access error on claude.ai](/docs/pt/github-enterprise-server#marketplace-add-on-claude-ai-fails-with-a-github-access-error) significa que sua própria conta GitHub Enterprise ainda não está conectada

<h2 id="install-a-plugin">
  Install a plugin
</h2>

Você adicionou um marketplace e executou uma instalação, e a instalação parou com uma mensagem em vez de instalar qualquer coisa. Estas entradas cobrem essas mensagens. Elas também cobrem as mensagens relacionadas que aparecem mais tarde na aba **Errors** do `/plugin`, ou como uma aba **Discover** vazia, quando um plugin ou seu marketplace não conseguem ser encontrados, lidos ou confiáveis.

<h3 id="plugin-not-found-in-marketplace">
  `Plugin "<name>" not found in marketplace "<marketplace>"`
</h3>

Você executou `/plugin install <name>@<marketplace>` ou `claude plugin install <name>@<marketplace>`, e o nome do plugin não está na cópia do catálogo desse marketplace em sua máquina.

`claude plugin install` no seu shell imprime a mesma mensagem quando você não adicionou o marketplace em absoluto. Se `claude plugin marketplace update <marketplace>` então responde `Marketplace '<marketplace>' not found`, [adicione o marketplace](#add-a-marketplace) primeiro.

<h4 id="the-message-ends-with-a-refresh-hint">
  `not found in marketplace` with a refresh hint
</h4>

A dica lê `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` ou `The marketplace couldn't be refreshed (...)`. Claude Code não atualizou o marketplace antes da busca, como quando você está offline, então sua cópia do catálogo pode estar desatualizada. Atualize com o nome do marketplace, depois instale novamente:

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` imprime `Successfully updated marketplace: <name>`, e `/plugin marketplace update` mostra `✔ Updated 1 marketplace`. Se a instalação retentada imprime a mesma mensagem, verifique o nome como [`not found in marketplace` with no hint](#the-message-has-no-hint) descreve. [When Claude Code refreshes a marketplace before an install](/docs/pt/plugins/loading#when-claude-code-refreshes-a-marketplace-before-an-install) lista os outros casos onde a atualização não é executada.

<h4 id="the-message-has-no-hint">
  `not found in marketplace` with no hint
</h4>

O nome é o problema mais provável. Abra `/plugin`, vá para **Discover** e copie o nome da lista.

Antes da v2.1.232, Claude Code atualizava o marketplace nomeado apenas após a busca falhar, e apenas quando auto-update estava ativado para ele.

<h3 id="plugin-not-found-in-any-marketplace">
  `Plugin "<name>" not found in any marketplace`
</h3>

Você executou `/plugin install <name>` sem `@marketplace`, e nenhum marketplace registrado tem esse plugin. `claude plugin install <name>` relata `Plugin "<name>" not found in any configured marketplace`.

Sem um nome de marketplace, `claude plugin install` procura nos catálogos que já tem e não os atualiza primeiro, e `/plugin install` atualiza apenas marketplaces que têm auto-update ativado. Nomeie o marketplace, e Claude Code o atualiza antes de procurar o plugin:

```text theme={null}
/plugin install <name>@<marketplace>
```

Quando a instalação funciona, você vê `✓ Installed <plugin>.` em uma sessão, ou `Successfully installed plugin: <plugin>@<marketplace>` a partir de `claude plugin install`.

Se você não sabe qual marketplace lista o plugin, execute `/plugin marketplace list` para os marketplaces que você tem, e navegue **Discover** em `/plugin` para o nome do plugin.

<h3 id="plugin-is-already-installed-globally">
  `Plugin '<name>@<marketplace>' is already installed globally`
</h3>

Você executou `/plugin install` para um plugin que já está instalado no escopo do usuário ou pelas configurações gerenciadas, e Claude Code recusou com `Use '/plugin' to manage existing plugins.` Se você digitou o nome do plugin sem `@<marketplace>`, a mensagem omite `globally`.

O plugin já está disponível em cada projeto, então não há nada a adicionar. Para alterar seu [scope](/docs/pt/plugins/install), habilitá-lo ou desabilitá-lo, ou configurá-lo, abra `/plugin` e vá para **Installed**.

Um plugin instalado apenas no escopo do projeto ou local não dispara esta mensagem. Claude Code permite que você o instale no escopo do usuário também, então está disponível em outros projetos.

`claude plugin install` no seu shell imprime uma mensagem diferente. Para um plugin já instalado no escopo de destino, imprime `Plugin "<name>@<marketplace>" is already installed (scope: user)` e sai com 0. Se seu diretório de cache está faltando, o mesmo comando o re-baixa.

<h3 id="this-plugin-uses-a-source-type-your-claude-code-version-does-not-suppo">
  `This plugin uses a source type your Claude Code version does not support`
</h3>

Você instalou um plugin cujo marketplace usa um tipo de fonte que esta versão do Claude Code não consegue buscar, e Claude Code parou com esta mensagem e `Update Claude Code and try again.`

Atualize Claude Code, depois tente novamente a instalação. Tipos de fonte estão na [marketplace reference](/docs/pt/plugins/marketplace-reference).

<h3 id="plugin-archive-integrity-check-failed">
  `Plugin archive integrity check failed`
</h3>

Você instalou um plugin que é distribuído como um arquivo zip, e Claude Code o recusou com esta linha e `The archive was not installed.` A entrada do marketplace do plugin usa uma [`archive` source](/docs/pt/plugins/marketplace-reference) com um pin `sha256`, e o digest do arquivo baixado não corresponde ao pin.

A mensagem completa se parece com isto:

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

A correção difere para o publicador e para o instalador:

* **Você publica o plugin**: recompute o digest do arquivo exato que a URL serve e atualize o `sha256` na entrada do marketplace. Use `shasum -a 256 my-plugin.zip`, ou `Get-FileHash -Algorithm SHA256 my-plugin.zip` no PowerShell
* **Você instala o plugin**: execute `/plugin marketplace update <name>` em uma sessão para atualizar o catálogo caso a entrada tenha sido corrigida, depois tente novamente a instalação. Se os digests ainda discordam após a atualização, peça ao proprietário do marketplace qual arquivo eles fixaram antes de instalar

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  `Marketplace "<name>" is registered from an untrusted source`
</h3>

Um marketplace que você adicionou anteriormente parou de carregar, e também seus plugins. Esta linha aparece na aba **Errors** do `/plugin` ou na próxima atualização.

O marketplace está registrado sob um nome que é [reservado para marketplaces oficiais da Anthropic](/docs/pt/plugins/marketplace-reference), mas sua fonte registrada não é um repositório GitHub `anthropics`. Nomes reservados são re-verificados toda vez que um marketplace carrega ou atualiza, então o marketplace e os plugins instalados a partir dele param de carregar.

A mensagem completa nomeia o nome reservado e a correção:

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

A correção difere para usuários e publicadores:

* **Você usa o marketplace**: no seu shell, execute `claude plugin marketplace remove <name>`, depois adicione o marketplace novamente a partir do repositório oficial `github.com/anthropics`
* **Você publica um marketplace de terceiros que usou o nome antes de ele se tornar reservado**: renomeie-o e peça aos usuários para re-adicioná-lo a partir de sua fonte

Antes da v2.1.205, Claude Code verificava o nome apenas quando você adicionava o marketplace, então uma entrada registrada antes de seu nome se tornar reservado continuava carregando.

<h3 id="plugin-has-a-corrupt-manifest-file-or-has-an-invalid-manifest-file">
  `Plugin <name> has a corrupt manifest file` or `has an invalid manifest file`
</h3>

Claude Code buscou o plugin, depois falhou ao ler seu `.claude-plugin/plugin.json`. No shell, o `<name>` nesta linha pode ser um nome de diretório temporário; o prefixo `Failed to install plugin "<name>@<marketplace>"` carrega o nome real do plugin. A redação diz qual verificação falhou:

* **`corrupt manifest file`, seguido por `JSON parse error:`**: o arquivo não é JSON válido
* **`invalid manifest file`, seguido por `Validation errors:`**: o arquivo analisa mas falha no schema, como `name: Invalid input` para um campo obrigatório faltando

`claude plugin install` relata qualquer um como `Failed to install plugin "<name>@<marketplace>":` e sai com código 1.

O autor do plugin tem que corrigir o arquivo, e o plugin não pode ser instalado até então:

* **Se é você**: execute `claude plugin validate <plugin-directory>` no seu shell para ver o mesmo erro com o caminho ofensivo, depois corrija o arquivo
* **Se não é você**: relate a mensagem ao proprietário do marketplace

<h3 id="plugin-directory-not-found-at-path">
  `Plugin directory not found at path: <path>`
</h3>

A aba **Errors** em `/plugin` mostra isto para um plugin habilitado que seu marketplace lista por um caminho relativo, como `./plugins/my-plugin`, quando nenhum diretório existe nesse caminho dentro do marketplace. Se você mantém o marketplace, corrija o caminho `source` da entrada ou restaure a pasta. Caso contrário, relate a mensagem ao proprietário do marketplace.

`Marketplace directory not found at path: <path>` significa que o próprio diretório do marketplace está faltando em vez disso. Para um marketplace que você adicionou a partir de um caminho local, esse diretório se moveu ou foi deletado. Restaure-o, ou remova o marketplace e adicione-o novamente a partir de seu novo local.

<h3 id="no-plugins-available-or-no-marketplaces-configured">
  `No plugins available` or `No marketplaces configured`
</h3>

Você abriu `/plugin` e a aba **Discover** está vazia, ou `claude plugin marketplace list` imprimiu `No marketplaces configured`.

Nenhum marketplace está registrado, então não há catálogo para mostrar. Em uma sessão, adicione o marketplace oficial, `anthropics/claude-plugins-official`:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: claude-plugins-official`, e **Discover** lista seus plugins. A página [Anthropic marketplaces](/docs/pt/plugins/anthropic-marketplaces) lista os outros marketplaces que você pode adicionar.

<h3 id="marketplace-is-already-added-from-a-different-source">
  `Marketplace "<name>" is already added from a different source`
</h3>

Você confirmou adicionar um marketplace através de [`/plugin install <plugin> --marketplace <source>`](/docs/pt/plugins/install#add-a-marketplace-and-install-in-one-command), e o catálogo que Claude Code buscou dessa fonte tem o mesmo nome que um marketplace que você já adicionou a partir de uma fonte diferente. Claude Code mantém o marketplace existente em vez de substituí-lo, e o plugin não é instalado.

A mensagem completa se parece com isto:

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

Escolha qual fonte você quer:

* **O marketplace que você já adicionou**: instale a partir dele pelo nome com `/plugin install <plugin>@<name>`
* **A nova fonte**: execute `/plugin marketplace remove <name>`, depois tente novamente a instalação

<h3 id="cannot-add-marketplace-its-network-source-differs">
  `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings`
</h3>

Você executou `marketplace add`, e o catálogo nessa fonte tem o mesmo nome que um marketplace que um arquivo de configurações já declara sob [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces) com uma fonte diferente. Claude Code recusa a adição e não registra nada.

A mensagem termina com a correção: a fonte deve corresponder à que é declarada para este nome nas configurações, ou você muda a declaração. Compare a fonte que você passou contra a entrada `extraKnownMarketplaces` para esse nome, incluindo seu `ref`, `path` e `headers`, depois faça um destes:

* **Use a fonte declarada**: adicione o marketplace a partir da fonte que a entrada de configurações nomeia
* **Use a nova fonte**: edite ou remova a entrada `extraKnownMarketplaces`, depois adicione o marketplace novamente. Se as configurações gerenciadas a declaram, peça ao seu administrador

<h3 id="failed-to-install-from-the-plugin-menu">
  `Failed to install: <plugin> (<reason>)`
</h3>

Você selecionou plugins para instalar no menu `/plugin`, nenhum deles foi instalado, e o menu fechou com este resumo do que falhou.

Algumas razões, como a saída do git após um clone falhado, mostram apenas sua primeira linha. Quando tal razão foi encurtada, o resumo termina com `Installing a plugin from its details (Enter) in /plugin shows its full error.`

O que fazer depende de se o resumo encurtou a razão:

* Corrija o que a razão entre parênteses nomeia
* Quando a razão foi encurtada, execute `/plugin`, selecione o plugin na aba **Discover** e pressione **Enter** para instalá-lo a partir de seus detalhes. Se a instalação falhar lá, a visualização de detalhes mostra o erro completo

<h3 id="could-not-move-the-new-copy-of-this-plugin-version">
  `Could not move the new copy of this plugin version into <path>`
</h3>

Quando você instala um plugin, Claude Code baixa uma cópia fresca de seus arquivos e a move para a pasta dessa versão no [plugin cache](/docs/pt/plugins/loading#find-plugins-on-disk). Esta mensagem significa que a movimentação falhou, geralmente porque outro programa estava usando a pasta enquanto a instalação era executada. O código do sistema de arquivos aparece entre parênteses:

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

A mensagem diz o que aconteceu com a cópia que foi instalada antes, o que lhe diz se o plugin ainda funciona:

* `The previously installed copy was moved back`: a versão que você tinha ainda está instalada
* `had to be removed first`, `was not moved back` ou `could not be moved back`: essa versão do plugin não está instalada até que uma instalação tenha sucesso
* Nenhuma tal sentença: não havia cópia anterior, então a versão não está instalada ainda

No Windows, quando outro programa mantém a cópia instalada em si, a mensagem em vez disso diz que essa cópia `could not be replaced` e que `It was not replaced and the new copy was discarded`, então a versão que você tinha ainda está instalada.

Uma lista `Left on disk` nomeia pastas postas de lado dentro do cache. Uma instalação posterior dessa versão ou uma limpeza de cache de plugin as remove, então você não precisa deletá-las.

Para corrigir a instalação:

* Feche outras sessões do Claude Code, editores e terminais que estão usando a pasta do plugin sob `~/.claude/plugins/cache`, depois execute a instalação novamente
* Quando a mensagem diz para verificar as permissões da pasta de cache do plugin, restaure sua permissão de escrita na pasta que ela nomeia e libere espaço em disco, depois execute a instalação novamente

<h3 id="dependency-errors">
  Dependency errors
</h3>

Um plugin que declara dependências pode falhar ao instalar, ou instalar e permanecer desabilitado, quando uma dependência não consegue ser satisfeita. A mensagem chega até você no tempo de instalação ou no tempo de carregamento:

* **Durante a instalação**: a recusa volta como a mensagem de erro da instalação
* **Quando o plugin carrega**: o problema aparece em `claude plugin list` e na aba **Errors** do `/plugin`, e Claude Code mantém o plugin afetado desabilitado até que você o resolva

A tabela lista cada mensagem e sua correção. Para declarar dependências como um autor, veja [Plugin dependencies](/docs/pt/plugins/dependencies).

| Mensagem                                                                                        | Significado                                                                                                 | Como resolver                                                                                                                                                                                                                                                                 |
| :---------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Dependency "<dep>" is not installed`                                                           | Uma dependência declarada não está instalada.                                                               | Instale-a no seu shell com `claude plugin install <dep>@<marketplace>`, ou desinstale o plugin. Se o marketplace da dependência ainda não está registrado, adicione-o e execute `/reload-plugins` em sua sessão, que instala as dependências faltantes que consegue resolver. |
| `Dependency "<dep>" is disabled`                                                                | A dependência está instalada mas desligada.                                                                 | Habilite a dependência, ou desinstale o plugin que a precisa.                                                                                                                                                                                                                 |
| `Requires "<dep>" <range>, installed <version>`                                                 | A versão da dependência instalada está fora do intervalo declarado do plugin.                               | Atualize a dependência para uma versão no intervalo, ou desinstale o plugin.                                                                                                                                                                                                  |
| `<Plugin or Dependency> "<name>" has conflicting version requirements`                          | Nenhuma versão satisfaz cada intervalo que a fixa. A mensagem lista os intervalos.                          | Desinstale ou atualize um dos plugins conflitantes, ou peça ao autor upstream para ampliar sua restrição.                                                                                                                                                                     |
| `... has version requirements too complex to intersect` ou `has an invalid version requirement` | Um intervalo não é semver válido, ou os intervalos combinados não conseguem ser intersectados.              | Corrija o intervalo inválido ou simplifique cadeias `\|\|` longas.                                                                                                                                                                                                            |
| `... has no git tag satisfying <range>`                                                         | O repositório da dependência não tem uma tag `<name>--v*` no intervalo.                                     | Verifique que o upstream marca lançamentos com essa convenção, ou relaxe o intervalo.                                                                                                                                                                                         |
| `Dependency "<dep>" (required by <plugin>) is in <marketplace>, which is not in the allowlist`  | A dependência está em um marketplace diferente, e a resolução entre marketplaces está desligada por padrão. | Instale a dependência você mesmo no mesmo escopo, no seu shell com `claude plugin install <dep>@<marketplace>` mais o `--scope` em que você está instalando o plugin, depois tente novamente.                                                                                 |

Para ver estes programaticamente, execute `claude plugin list --json` no seu shell. Plugins com problemas carregam um campo `errors` com as mensagens e um campo `errorDetails` com um `type` para cada: as duas primeiras linhas são `dependency-unsatisfied` e a terceira é `dependency-version-unsatisfied`.

<h2 id="plugin-installed-but-not-working">
  Plugin installed but not working
</h2>

A instalação teve sucesso, mas as skills, hooks ou servidores do plugin não estão fazendo nada. Comece com [Plugin doesn't appear or its skills don't show up](#plugin-doesnt-appear-or-its-skills-dont-show-up), que lhe diz onde Claude Code relata o que carregou, depois corresponda a mensagem.

<h3 id="plugin-doesnt-appear-or-its-skills-dont-show-up">
  Plugin doesn't appear or its skills don't show up
</h3>

Você instalou um plugin e digitou `/` esperando suas skills, ou pediu a Claude para usá-lo, e nada aconteceu.

Verifique o estado do plugin antes de mudar qualquer coisa:

<Steps>
  <Step title="Confirm the plugin is installed and enabled">
    Execute `/plugin` e abra **Installed**. Confirme que o plugin está listado e habilitado. `claude plugin list` no seu shell imprime a mesma lista com a versão de cada plugin, escopo e `Status: ✔ enabled`.
  </Step>

  <Step title="Read the Errors tab">
    Abra a aba **Errors** no mesmo painel. Cada entrada emparelha uma mensagem com uma linha de orientação. A maioria das mensagens no resto desta seção vem dessa aba.
  </Step>

  <Step title="Reload if you installed during this session">
    Se o plugin está instalado e sem erros mas você o instalou durante esta sessão, execute `/reload-plugins`. Imprime `Reloaded:` com contagens de plugins, skills, agents, hooks e servidores. Quando algo falhou ao carregar, adiciona `N errors during load. Run /plugin for details.`
  </Step>
</Steps>

Se o plugin carrega sem erro e suas skills ainda não aparecem, o próximo passo difere para seu próprio plugin e para o de alguém:

* **Um plugin que você está construindo**: veja [Plugin loads but its skills are missing](#plugin-loads-but-its-skills-are-missing)
* **Um plugin que alguém publicou**: abra **Installed** em `/plugin` e abra o painel de detalhes do plugin, que lista o que o plugin contém. Um plugin que não lista skills lá não tem nenhuma para oferecer quando você digita `/`

<h3 id="run-reload-plugins-to-activate">
  `Run /reload-plugins to activate.`
</h3>

O resumo de instalação em `/plugin` terminou com `Run /reload-plugins to activate.` em vez de `Plugin is now active.`

Claude Code não ativou o plugin durante a instalação, seja porque ativá-lo [invalidaria o cache de prompt](/docs/pt/prompt-caching#enabling-or-disabling-a-plugin) ou porque a tentativa de ativação falhou.

Você não precisa digitar o comando. O painel fecha e Claude Code executa `/reload-plugins` para você, ou o coloca na fila até que a resposta que está sendo transmitida termine.

Leia o que esse reload imprime:

* **`Reloaded:` com contagens de plugins, skills, agents, hooks e servidores**: o plugin agora está ativo. Quando algo falhou ao carregar, a linha adiciona `N errors during load. Run /plugin for details.`
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`**: o reload adicionaria ou removeria um servidor MCP de plugin, ou a ferramenta `LSP`, e invalidaria seu cache de prompt. Para o caso LSP a linha começa `This reload adds the LSP tool` ou `This reload removes the LSP tool`. Execute-o com `--force` para ativar o plugin mesmo assim, ou inicie uma nova sessão

Antes da v2.1.268, uma instalação que não ativou durante a instalação permanecia pendente até que você executasse `/reload-plugins` você mesmo.

Antes da v2.1.246, a contagem de skills nesse resumo incluía apenas entradas `commands/` de um plugin, então um reload poderia carregar as skills `SKILL.md` de um plugin e ainda relatar `0 skills`.

<h3 id="plugin-not-cached-at">
  `Plugin "<name>" not cached at <path>`
</h3>

A aba **Errors** mostra esta linha com a orientação `Run /plugin to refresh the plugin cache`. Claude Code tem um registro de instalação para o plugin, mas o diretório que o registro aponta está faltando, por exemplo após você limpar o cache.

Reinstale o plugin a partir do seu shell. `claude plugin install <name>@<marketplace>` re-baixa um plugin cujo diretório de instalação está faltando mesmo que seu registro exista:

```shell theme={null}
claude plugin install <name>@<marketplace>
```

Depois execute `/reload-plugins` em sua sessão. A entrada da aba **Errors** desaparece e o plugin está de volta sob **Installed**.

<h3 id="a-plugin-you-disabled-still-loads">
  `Disabled in ~/.claude/settings.json but still loads`
</h3>

Você definiu um plugin como `false` em `~/.claude/settings.json`, e sua linha em `claude plugin list` ou `/plugin` mostra esta mensagem seguida pela fonte que o habilita, como `— project settings enable it, which overrides your user setting`. Um `true` nessa fonte de precedência mais alta está sobrescrevendo sua configuração de usuário.

Para optar por não usar um plugin habilitado por projeto em sua máquina, defina o id como `false` em `.claude/settings.local.json`, que tem precedência mais alta que o arquivo do projeto. Para as outras fontes que a mensagem pode nomear, veja [Disabled in user settings but still loads](/docs/pt/plugins/loading#disabled-in-user-settings-but-still-loads).

Se `claude plugin list` em vez disso marca o plugin `required by your org`, nenhum arquivo de configurações está envolvido: sua organização marca esse plugin sincronizado como obrigatório em claude.ai, e ele carrega mesmo que você o tenha desabilitado antes. Veja [Plugins synced from claude.ai](/docs/pt/plugins/loading#synced-plugins).

<h3 id="plugin-is-enabled-in-project-settings-but-isnt-installed-here">
  `Plugin "<name>" is enabled in project settings but isn't installed here`
</h3>

A aba **Errors** mostra esta linha para um plugin que o `.claude/settings.json` do seu projeto habilita, com a orientação `Run claude plugin install <name>@<marketplace> --scope project to install it for this project`.

As configurações de um repositório podem habilitar um plugin para todos que o abrem, mas não o instalam. Quando o plugin vem de uma fonte externa como um repositório GitHub ou um pacote npm, Claude Code não o baixa até que você o instale você mesmo. Execute o comando da linha de orientação no seu shell, depois recarregue:

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

Depois que você executar `/reload-plugins` em sua sessão, a entrada da aba **Errors** se foi e o plugin está listado sob **Installed**.

Se sua organização pré-instala plugins para você, ela o faz através de configurações gerenciadas em vez disso. Veja [Pre-install and require plugins](/docs/pt/plugins/org#pre-install-and-require-plugins).

<h3 id="failed-to-load-hooks-from-and-hooks-that-dont-fire">
  `Failed to load hooks from <path>` and hooks that don't fire
</h3>

Os hooks de um plugin não são executados. Ou a aba **Errors** mostra uma falha de carregamento para eles, os hooks carregam e você vê avisos `<Event> hook error` na transcrição, ou um hook carrega sem erro e nunca dispara.

<h4 id="hooks-fail-to-load">
  Hooks fail to load
</h4>

A aba **Errors** mostra uma destas mensagens:

* **`Failed to load hooks from <path>: <reason>`**: `hooks/hooks.json` não é JSON válido ou falha no schema de hooks. A razão nomeia o erro de análise ou validação. Corrija o arquivo. Para capturar um problema de sintaxe JSON em `hooks/hooks.json` antes de publicar o plugin, execute `claude plugin validate <plugin-directory>` no seu shell
* **`hooks path not found: <path>`**: o campo `hooks` do manifesto nomeia um arquivo que não existe nesse caminho relativo à raiz do plugin. Corrija o caminho ou adicione o arquivo

<h4 id="hook-error-notices-in-the-transcript">
  `hook error` notices in the transcript
</h4>

Um aviso da forma `... hook error: Failed with non-blocking status code: <stderr>` significa que o hook foi executado e seu comando falhou. Por exemplo, `Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found` significa que o shell que Claude Code gerou não conseguiu encontrar `node`. Instale-o, ou certifique-se de que está no `PATH` do terminal a partir do qual você inicia `claude`.

Para qualquer outro erro, execute o comando do hook você mesmo a partir do diretório do plugin para ver a saída completa, ou capture o stderr completo com [debug logging](/docs/pt/hooks#debug-hooks).

<h4 id="hook-loads-but-never-fires">
  Hook loads but never fires
</h4>

Se um hook carrega sem erro mas nunca dispara, verifique sua definição e depois observe-o ser executado:

<Steps>
  <Step title="Check the event name">
    Nomes de eventos são sensíveis a maiúsculas, então confirme que o seu corresponde exatamente, por exemplo `PostToolUse`.
  </Step>

  <Step title="Check the matcher">
    Confirme que o `matcher` do hook corresponde ao nome da ferramenta.
  </Step>

  <Step title="Trigger the event on purpose">
    Para um hook `PostToolUse`, peça a Claude para editar um arquivo.
  </Step>

  <Step title="Read the debug log">
    Abra o [debug log](/docs/pt/hooks#debug-hooks), que registra quais hooks corresponderam. Um hook que foi executado aparece lá com seu código de saída.
  </Step>
</Steps>

<h3 id="invalid-mcp-server-config-for-and-mcp-servers-that-dont-start">
  `Invalid MCP server config for "<server>"` and MCP servers that don't start
</h3>

Um plugin agrupa um servidor MCP, e a aba **Errors** mostra `Invalid MCP server config for "<server>": <error>`, ou o servidor está listado mas `/mcp` nunca o mostra conectado.

<h4 id="invalid-mcp-server-config-for-server-error">
  `Invalid MCP server config for "<server>": <error>`
</h4>

A configuração do servidor passa na verificação de schema, mas Claude Code não consegue resolvê-la para esta sessão. O texto após os dois pontos nomeia a causa e decide a correção:

* **`Missing environment variables: <names>`**: defina essas variáveis no shell a partir do qual você inicia Claude Code, depois inicie uma nova sessão
* **`URL is unset or invalid`**: uma opção `${user_config.*}` que a URL usa não está definida. Execute `/plugin configure <plugin>` para defini-la
* **`has an invalid MCP url`** ou **`headersHelper for MCP server '<server>' references ${user_config.*}`**: a configuração do próprio plugin está em falta. Corrija a `url` ou `headersHelper` em sua configuração MCP do plugin, ou relate ao autor do plugin se o plugin não é seu. O caso `headersHelper` tem sua própria entrada em [plugin command references user\_config](/docs/pt/errors#plugin-command-references-user-config)

<h4 id="server-is-configured-but-never-connects">
  Server is configured but never connects
</h4>

Execute `/mcp` para ver o status do servidor. Quando o servidor está saudável, `/mcp` o lista como conectado.

Para ler o erro que o servidor imprimiu ao iniciar, execute `claude --debug` e abra o log em `~/.claude/debug/<session-id>.txt`. O sinalizador `--debug` não imprime no terminal.

Uma entrada de servidor em `.mcp.json` que falha no schema não aparece na aba **Errors**. Claude Code descarta esse servidor e registra `Invalid MCP server config for <server> in <path>` apenas nesse log de debug. Para encontrar a entrada sem carregar o plugin, execute `claude plugin validate` no seu shell no diretório do plugin, que a relata como um erro.

Antes da v2.1.281, `claude plugin validate` não verificava `.mcp.json`.

<h4 id="server-works-with-plugin-dir-but-fails-after-install">
  Server works with `--plugin-dir` but fails after install
</h4>

Você é o autor do plugin, e o servidor inicia quando você carrega o plugin a partir de seu diretório de origem com `--plugin-dir` mas falha uma vez que o plugin está instalado.

Claude Code copia um plugin instalado em seu cache, então um caminho que só funciona a partir do diretório de origem quebra. Escreva caminhos dentro do plugin com `${CLAUDE_PLUGIN_ROOT}`.

Para caminhos que alcançam fora do diretório do plugin, veja [Files the plugin references outside its directory aren't found](#files-the-plugin-references-outside-its-directory-arent-found).

<h3 id="language-server-doesnt-start">
  Language server doesn't start, uses too much memory, or reports wrong diagnostics
</h3>

Você instalou um [code intelligence plugin](/docs/pt/plugins/code-intelligence) e Claude não está vendo diagnósticos, ou o servidor de linguagem está usando muita memória ou relatando erros que não são reais.

<h4 id="language-server-doesn’t-start">
  Language server doesn't start
</h4>

O plugin se conecta a um binário de servidor de linguagem que você instala separadamente, e Claude Code o gera por nome de comando a partir do seu `PATH`.

A aba **Errors** do `/plugin` mostra a falha com sua razão, como `Executable not found in $PATH: "<binary>"`, e `claude --debug` a registra como `LSP server <name> failed to start: <reason>`.

Instale o binário e confirme que está no `PATH` do terminal a partir do qual você inicia `claude`, por exemplo com `which typescript-language-server`. Depois inicie uma nova sessão.

<h4 id="language-server-uses-too-much-memory">
  Language server uses too much memory
</h4>

Servidores de linguagem como `rust-analyzer` e `pyright` indexam o projeto inteiro. Desabilite o plugin com `/plugin disable <plugin>` em uma sessão e confie nas ferramentas de busca integradas do Claude em vez disso.

<h4 id="false-positive-diagnostics-in-a-monorepo">
  False positive diagnostics in a monorepo
</h4>

Um servidor de linguagem que não está configurado para o workspace pode relatar importações não resolvidas para pacotes internos. Não há nada a corrigir no lado do Claude Code, e os diagnósticos não impedem Claude de editar código.

<h2 id="build-a-plugin">
  Build a plugin
</h2>

Você está desenvolvendo um plugin e carregando-o com `--plugin-dir` ou instalando-o a partir de um marketplace local. Estas entradas cobrem as falhas que você encontra ao desenvolver um plugin. Para as verificações serem executadas após cada mudança, veja [Test and debug](/docs/pt/plugins/create#test-and-debug).

Duas falhas que também alcançam os usuários de um plugin têm suas entradas em [Plugin installed but not working](#plugin-installed-but-not-working):

* **Um hook que não dispara**: veja [hooks that don't fire](#failed-to-load-hooks-from-and-hooks-that-dont-fire)
* **Um servidor MCP que não inicia**: veja [MCP servers that don't start](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)

<h3 id="commands-path-not-found">
  `commands path not found: <path>`
</h3>

A aba **Errors** mostra `commands path not found: <absolute path>` com a orientação `Check that the path in your manifest or marketplace config is correct`. A mesma mensagem aparece para `skills`, `agents` e `hooks`.

Claude Code resolveu um caminho a partir de seu `plugin.json` ou entrada de marketplace contra a raiz do plugin e não encontrou nada lá. O caminho na mensagem é o caminho absoluto que verificou, então compare-o com o que está no disco. Corrija o caminho ou crie o diretório, depois execute `/reload-plugins`.

Caminhos no manifesto são relativos à raiz do plugin e começam com `./`. Um caminho que resolve fora da raiz do plugin é relatado como `<component> path escapes plugin directory` em vez disso e é descartado.

<h3 id="plugin-dir-loads-a-plugin-with-no-components">
  `--plugin-dir` at a marketplace root doesn't load the plugins under `plugins/`
</h3>

Você iniciou `claude --plugin-dir <path>` e não vê erro, mas as skills, agents e hooks do plugin não estão lá.

`--plugin-dir` leva o diretório raiz do plugin, aquele que contém `.claude-plugin/plugin.json` e os diretórios de componentes como `skills/`. Se você apontá-lo para uma raiz de marketplace em vez disso, Claude Code não lê `marketplace.json`, então um plugin sob `plugins/` não carrega, e você não vê erro. Antes da v2.1.281, Claude Code carregava uma raiz de marketplace como um plugin vazio nomeado após esse diretório. Aponte o sinalizador para o diretório do plugin em si:

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

Depois abra **Installed** em `/plugin`, onde o painel de detalhes do plugin lista seus componentes.

<h3 id="files-the-plugin-references-outside-its-directory-arent-found">
  Files the plugin references outside its directory aren't found
</h3>

Um plugin funciona a partir de seu diretório de origem com `--plugin-dir` mas falha após instalar, com erros sobre um caminho como `../shared-utils`.

Claude Code copia um plugin instalado em seu cache e o carrega de lá, então um caminho que alcança fora do próprio diretório do plugin aponta para nada no cache. Mova os arquivos compartilhados dentro do diretório do plugin, ou os referencie através de um symlink dentro dele. Para onde o cache está e como caminhos resolvem, veja [Find plugins on disk](/docs/pt/plugins/loading#find-plugins-on-disk).

<h3 id="claude-plugin-root-shows-forward-slashes-on-windows">
  `${CLAUDE_PLUGIN_ROOT}` shows forward slashes on Windows
</h3>

No Windows, um hook de plugin recebe `${CLAUDE_PLUGIN_ROOT}` como `C:/Users/you/...` em vez de `C:\Users\you\...`, e um script que esperava barras invertidas quebra.

Claude Code executa hooks de forma de shell através do Git Bash no Windows e substitui a raiz do plugin na forma Win32 com barra para frente de propósito. Builtins Bash, ferramentas MSYS e binários Windows nativos todos aceitam essa forma.

Se seu script precisa de barras invertidas, mude o hook para uma das formas que mantêm caminhos nativos, descritas em [exec form and shell form](/docs/pt/hooks#exec-form-and-shell-form):

* Um hook de forma exec, que gera o processo diretamente com um array `args`
* Um hook com `"shell": "powershell"`

<h3 id="plugin-loads-but-its-skills-are-missing">
  Plugin loads but its skills are missing
</h3>

Seu plugin está listado sob **Installed** sem erros, mas suas skills não são oferecidas quando você digita `/`.

Skills carregam a partir de `skills/` na raiz do plugin e comandos a partir de `commands/` na raiz do plugin. Apenas `plugin.json` pertence dentro de `.claude-plugin/`, e um diretório `skills/` dentro de `.claude-plugin/` não é verificado. Mova os diretórios para a raiz do plugin e execute `/reload-plugins`. Depois, o painel de detalhes do plugin em `/plugin` lista as skills, e digitar `/` as oferece.

Cada skill é um diretório contendo `SKILL.md`. Uma entrada `skills` no manifesto que aponta para um arquivo `SKILL.md` em vez de seu diretório é relatada como `path is a file; skills entries must be directories containing SKILL.md`.

<h3 id="skill-loads-but-claude-never-invokes-the-skill">
  Skill loads but Claude never invokes the skill
</h3>

A skill do seu plugin é executada quando você digita seu comando `/<plugin>:<skill>`, mas Claude nunca a invoca em resposta a um pedido simples.

Verifique estas causas em ordem:

* **A skill define `disable-model-invocation: true`**: com esse campo definido, apenas você pode invocar a skill. A skill de modelo em [Create your first plugin](/docs/pt/plugins/create#create-your-first-plugin) a define. Remova a linha de uma skill que você quer que Claude invoque por conta própria. [Control who invokes a skill](/docs/pt/skills#control-who-invokes-a-skill) cobre o campo
* **A descrição não corresponde a como as pessoas pedem**: trabalhe através das verificações em [Skill not triggering](/docs/pt/skills#skill-not-triggering)
* **A descrição está truncada**: quando muitas skills estão instaladas, Claude Code encurta descrições para caber na listagem do orçamento de caracteres, o que pode remover as palavras-chave que Claude precisa para corresponder a um pedido. Veja [Skill descriptions are cut short](/docs/pt/skills#skill-descriptions-are-cut-short)

Para medir com que frequência a skill dispara em prompts realistas em vez de verificar um de cada vez, escreva um caso de eval com um [grader `tool_used: Skill`](/docs/pt/plugin-evals#create-your-first-eval-suite) e execute-o com `claude plugin eval` após cada mudança de descrição.

<h3 id="is-not-a-plugin-or-skill-folder">
  `<directory> is not a plugin or skill folder` from `claude plugin eval init`
</h3>

Você executou `claude plugin eval init` a partir de um diretório que não é a raiz de um plugin, como seu diretório inicial ou a raiz de um repositório que mantém o plugin em um subdiretório. `init` escreve a suite sob o diretório de trabalho, então para em vez de criar um diretório `evals/` que o plugin nunca veria.

Mude para a raiz do plugin, o diretório que contém `.claude-plugin/plugin.json` ou o `SKILL.md` da skill, e execute o comando novamente. Para estruturar a suite em outro lugar de propósito, passe `--eval-dir`. Veja [Test plugins with evals](/docs/pt/plugin-evals).

<h3 id="the-userconfig-dialog-never-appears">
  The `userConfig` dialog never appears
</h3>

Seu plugin declara opções `userConfig`, mas nenhum diálogo de configuração aparece quando você o instala.

A instalação interativa mostra o diálogo, e o comando de shell leva os valores como sinalizadores em vez disso:

* **`/plugin install` em uma sessão, ou a aba Discover em `/plugin`**: o diálogo é parte desta instalação interativa
* **`claude plugin install` no seu shell**: nunca pede valores `userConfig`. Salva qualquer valor `--config KEY=VALUE` que você passa, e quando opções permanecem não definidas imprime `N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` Quando qualquer uma das opções não definidas é obrigatória, `(M required)` segue `not yet set`.

Se você instalou a partir do shell, passe os valores com `--config`, um sinalizador por opção:

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

Quando cada opção está definida, a saída de instalação não carrega nenhuma linha `not yet set`. Para abrir o diálogo depois em vez disso, execute `/plugin configure my-plugin@my-marketplace` em uma sessão.

Se você passar uma chave `--config` que o manifesto não declara, o plugin ainda instala, e o comando imprime `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.` seguido pelas chaves que o plugin declara.

<h3 id="claude-plugin-validate-reports-errors">
  `claude plugin validate` reports errors
</h3>

Você executou `claude plugin validate <path>`, ou `/plugin validate <path>` em uma sessão, e imprimiu `Found N errors` e `Validation failed`, depois saiu com código 1.

O validador lê o manifesto no caminho que você fornece: `.claude-plugin/plugin.json` para um diretório de plugin, ou `.claude-plugin/marketplace.json` para um diretório de marketplace. Para um marketplace, ele prefixos problemas no manifesto próprio de uma entrada com o índice de entrada, como `plugins[1] plugin.json → json: ...`.

A tabela cobre as mensagens que param a validação e dois avisos, `No frontmatter block found` e `Unknown field '<key>'`, que a param apenas quando você passa `--strict`. Outros avisos, como uma descrição faltando, não estão listados.

| Mensagem                                                                                                 | Causa                                                                            | Correção                                                                                                         |
| :------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `File not found: <path>`                                                                                 | O caminho não tem manifesto, ou não existe.                                      | Execute o comando contra a raiz do plugin ou marketplace, o diretório que contém `.claude-plugin/`.              |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | O diretório não tem manifesto `.claude-plugin/`.                                 | Crie o manifesto, ou aponte para o diretório correto.                                                            |
| `Invalid JSON syntax: <parse error>`                                                                     | O manifesto, ou `hooks/hooks.json`, não é JSON válido.                           | Corrija o JSON. Até que você corrija `hooks/hooks.json`, uma sessão carrega o plugin sem os hooks nesse arquivo. |
| `Path not found: <path>. The runtime loader will report this as a load failure.`                         | Um caminho de componente no manifesto não existe.                                | Corrija o caminho ou crie o diretório.                                                                           |
| `Path contains ".." which could be a path traversal attempt: <path>`                                     | Um caminho de componente escapa do diretório do plugin.                          | Use caminhos dentro da raiz do plugin.                                                                           |
| `Path is a file; skills entries must be directories containing SKILL.md`                                 | Uma entrada `skills` aponta para `SKILL.md` em vez de seu diretório.             | Aponte para o diretório pai, ou `.` para um `SKILL.md` no nível raiz.                                            |
| `No frontmatter block found` ou `YAML frontmatter failed to parse: <error>`                              | Um arquivo de skill, agent ou comando tem frontmatter YAML faltando ou inválido. | Adicione ou corrija o frontmatter entre delimitadores `---`. Relatado ao validar um diretório de plugin.         |
| `Unknown field '<key>'`                                                                                  | O manifesto tem um campo que o schema não define.                                | Remova-o, ou use o nome que a mensagem sugere. Claude Code ignora campos desconhecidos no tempo de carregamento. |

Execute o comando novamente após cada correção até que imprima sem erros.

Os campos `plugin.json` estão na [manifest reference](/docs/pt/plugins/manifest-reference), e as mensagens no nível de marketplace estão em [Marketplace validation errors](#marketplace-validation-errors).

<h3 id="plugin-has-conflicting-manifests">
  `Plugin <name> has conflicting manifests`
</h3>

O plugin falha ao carregar com `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.`

O plugin tem seu próprio `plugin.json`, e sua entrada de marketplace define `strict: false` enquanto também declara qualquer um de `commands`, `agents`, `skills`, `hooks`, `outputStyles` ou `themes`. Remova esses campos da entrada, ou defina `strict: true` na entrada para que Claude Code os acrescente a `plugin.json`. Veja [Strict mode](/docs/pt/plugins/marketplace-reference#strict-mode).

<h3 id="warning-no-commands-found-in-plugin-custom-directory">
  `Warning: No commands found in plugin <name> custom directory`
</h3>

Quando o plugin carrega, o log `claude --debug` em `~/.claude/debug/<session-id>.txt` registra `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` Nada aparece na sessão ou na aba **Errors**.

O caminho `commands` no manifesto existe mas não contém arquivos `.md` e nenhum `SKILL.md` em um subdiretório. Adicione os arquivos de comando, ou remova o caminho do manifesto.

<h2 id="host-a-marketplace">
  Host a marketplace
</h2>

Você publica um marketplace e um usuário relata um erro, ou sua própria validação falha. Estas entradas são para o proprietário do marketplace.

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Plugins with relative paths fail in URL-based marketplaces
</h3>

Os usuários adicionaram seu marketplace com uma URL `https://example.com/marketplace.json`. As instalações de plugins cuja `source` é um caminho relativo, como `./plugins/my-plugin`, falham com `its marketplace entry path does not stay inside the marketplace directory`. Os plugins já instalados falham ao carregar com `Plugin source path refused`. Ambas as mensagens têm uma [entrada de referência de erro](/docs/pt/errors#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory).

Quando um usuário adiciona um marketplace baseado em URL, Claude Code baixa apenas o arquivo `marketplace.json` em si. Não busca arquivos de plugin por caminho relativo desse servidor, então um caminho relativo em uma entrada aponta para um diretório que nunca foi buscado. Dê a cada entrada uma fonte que Claude Code consegue buscar por conta própria, como um repositório GitHub:

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

Alternativamente, hospede o marketplace em um repositório git e diga aos usuários para adicioná-lo com a URL do repositório. Para uma fonte git, Claude Code clona o repositório inteiro, então caminhos relativos resolvem. Tipos de fonte estão na [marketplace reference](/docs/pt/plugins/marketplace-reference).

<h3 id="marketplace-validation-errors">
  Marketplace validation errors
</h3>

Você executou `claude plugin validate .` a partir de seu diretório de marketplace e relatou erros ou avisos no arquivo do marketplace em si.

`claude plugin validate` também valida cada entrada cuja `source` é um caminho local e avisa quando a `version` da entrada discorda do manifesto próprio do plugin.

A tabela lista as mensagens no nível de marketplace. As mensagens no nível de entrada são as mensagens de plugin em [`claude plugin validate` reports errors](#claude-plugin-validate-reports-errors), prefixadas com `plugins[N] plugin.json →`.

| Mensagem                                                                                                                  | Tipo  | Correção                                                                                                                                             |
| :------------------------------------------------------------------------------------------------------------------------ | :---- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Duplicate plugin name "<name>" found in marketplace`                                                                     | Erro  | Dê a cada plugin um `name` único.                                                                                                                    |
| `Path contains "..": <path>` sob `plugins[N].source`                                                                      | Erro  | Use caminhos relativos à raiz do marketplace sem segmentos `..`.                                                                                     |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                          | Erro  | Remova o caractere do nome, como um escape ou uma nova linha.                                                                                        |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                               | Erro  | Remova o caractere do `name` do plugin.                                                                                                              |
| `Marketplace has no plugins defined`                                                                                      | Aviso | Adicione pelo menos uma entrada a `plugins`.                                                                                                         |
| `No marketplace description provided`                                                                                     | Aviso | Adicione uma `description` no nível superior.                                                                                                        |
| `Plugin name "<name>" is not kebab-case` sob `plugins[N] plugin.json → name`                                              | Aviso | Renomeie para letras minúsculas, dígitos e hífens. Claude Code aceita outras formas, mas a sincronização de marketplace de claude.ai as rejeita.     |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"`                                                          | Aviso | Atualize a entrada para corresponder a `plugin.json`, que é autoritário no tempo de instalação.                                                      |
| `Marketplace name "<name>" is reserved in Claude Desktop`                                                                 | Aviso | Renomeie o marketplace. A sincronização de marketplace gerenciada do Claude Desktop rejeita `org`, `org-provisioned` e `unknown` em qualquer casing. |
| `Marketplace name "<name>" is not accepted by Claude Desktop` ou `Plugin name "<name>" is not accepted by Claude Desktop` | Aviso | Renomeie para no máximo 128 caracteres de letras, dígitos, `.`, `_` e `-`, começando com uma letra ou dígito.                                        |

Antes da v2.1.247, um nome de marketplace contendo caracteres de controle ou formatação bidirecional era relatado apenas como `Marketplace name impersonates an official Anthropic/Claude marketplace`.

<h2 id="blocked-by-your-organization">
  Blocked by your organization
</h2>

Sua organização implanta configurações gerenciadas que restringem plugins, e um comando foi recusado com uma mensagem de política. Estas entradas nomeiam a configuração por trás de cada recusa para que você saiba o que pedir ao seu administrador. Para o lado do administrador, veja [Manage plugins for your organization](/docs/pt/plugins/org).

<h3 id="marketplace-source-is-blocked-by-enterprise-policy">
  `Marketplace source '<source>' is blocked by enterprise policy`
</h3>

Você executou `/plugin marketplace add`, `update` ou uma instalação, e Claude Code recusou com esta linha. Para uma fonte GitHub ou git, o host segue a fonte entre parênteses, como em `'github:owner/repo' (github.com)`.

Seu administrador definiu `blockedMarketplaces` ou `strictKnownMarketplaces` em configurações gerenciadas, e esta fonte não é permitida. Peça ao seu administrador para permitir a fonte, ou adicione uma das fontes permitidas que a mensagem lista.

Corresponda o resto da mensagem para ver que tipo de política bloqueou a fonte:

* **`Allowed sources: <list>`**: o bloqueio vem da lista de permissões `strictKnownMarketplaces` em vez da lista de bloqueio `blockedMarketplaces`
* **`No external marketplaces are allowed.`**: a lista de permissões `strictKnownMarketplaces` está vazia
* **Uma `Tip:` que o atalho assume github.com**: a lista de permissões permite um host git pelo nome de host, e o atalho `owner/repo` que você passou aponta para github.com. Se o repositório vive em seu host interno, adicione-o novamente com sua URL completa, como `git@your-git-host.com:owner/repo.git`

Um marketplace que você adicionou antes da política se tornar mais restritiva para de atualizar também, porque a política se aplica em cada atualização.

<h3 id="marketplace-is-not-in-the-allowed-marketplace-list">
  `Marketplace "<name>" is not in the allowed marketplace list`
</h3>

A aba **Errors** mostra esta linha, ou `Marketplace "<name>" is blocked by enterprise policy`, para um marketplace que você já tem registrado.

As mesmas configurações gerenciadas que bloqueiam uma [marketplace source](#marketplace-source-is-blocked-by-enterprise-policy) se aplicam no tempo de carregamento. `strictKnownMarketplaces` não inclui este marketplace, ou `blockedMarketplaces` o nomeia, então Claude Code para de carregá-lo e seus plugins. Para a variante de lista de permissões, a linha de orientação mostra as fontes permitidas, ou `Contact your administrator to configure allowed marketplace sources`. Para a variante de lista de bloqueio lê `This marketplace source is explicitly blocked by your administrator`.

<h3 id="plugin-is-blocked-by-your-organizations-policy-and-cannot-be-installed">
  `Plugin "<name>" is blocked by your organization's policy and cannot be installed`
</h3>

Uma instalação foi recusada com esta linha, uma habilitação com a mesma linha terminando `cannot be enabled`, ou uma instalação ou atualização com uma nomeando a razão: `Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy`, ou `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy`.

As configurações gerenciadas bloqueiam este plugin, seu marketplace ou uma dependência que precisa. Peça ao seu administrador qual entrada se aplica. Uma dependência bloqueada significa que o plugin não consegue instalar até que o marketplace da dependência seja permitido.

<h3 id="plugin-dir-is-disabled-by-your-organizations-managed-settings-disables">
  `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`
</h3>

Você iniciou `claude` com `--plugin-dir`, `--plugin-url`, `--agents` ou `--mcp-config`. Claude Code saiu com esta mensagem e `Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

Seu administrador definiu `disableSideloadFlags` em configurações gerenciadas, que desliga os sinalizadores que carregam plugins, agents e servidores a partir de caminhos arbitrários. Carregue o plugin a partir de um marketplace aprovado em vez disso, ou peça ao seu administrador para remover a configuração.

Uma mensagem relacionada na aba **Errors** do `/plugin` é `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`. As configurações gerenciadas habilitam ou desabilitam esse plugin pelo nome, e Claude Code ignora sua cópia `--plugin-dir` dele para que o sinalizador não possa sobrescrever a política.

<h3 id="plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s">
  `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`
</h3>

Você executou `claude plugin init` ou `claude plugin enable`, e parou com esta linha. A mensagem nomeia `strictKnownMarketplaces or blockedMarketplaces` e pede ao seu administrador para adicionar `{"source":"skills-dir"}` a `strictKnownMarketplaces` ou removê-lo de `blockedMarketplaces`.

A fonte `skills-dir` representa plugins que Claude Code carrega a partir de seu diretório `~/.claude/skills/`. Peça ao seu administrador para fazer a mudança que a mensagem nomeia.

<h3 id="command-sourced-plugins-are-disabled-by-your-organizations-managed-set">
  `Command-sourced plugins are disabled by your organization's managed settings`
</h3>

Você instalou ou atualizou um plugin com uma fonte `command`, e parou com esta linha e `The plugin was not installed or updated and its command was not run.`

Seu administrador definiu `disableCommandPluginSources`, então Claude Code recusa executar o comando declarado pelo marketplace que produz o plugin. Definir `allowManagedHooksOnly` sozinho tem o mesmo efeito quando `disableCommandPluginSources` não está definido. Peça ao seu administrador se o plugin pode ser publicado a partir de um tipo de fonte que a política permite.

<h3 id="marketplace-is-seed-managed">
  `Marketplace '<name>' is seed-managed`
</h3>

Você executou `claude plugin marketplace update <name>`, e falhou com `Marketplace '<name>' is seed-managed (<dir>)` e uma dica para pedir ao seu admin.

Um operador pré-populou este marketplace através de `CLAUDE_CODE_PLUGIN_SEED_DIR`, e Claude Code trata um marketplace gerenciado por seed como somente leitura. Uma atualização em massa `marketplace update` o pula e atualiza os outros.

Para mudar o conteúdo do marketplace, peça à pessoa que mantém a imagem de seed para atualizá-lo. Para o procedimento, veja [Seed containers and CI](/docs/pt/plugins/org#seed-containers-and-ci).

<h2 id="next-steps">
  Next steps
</h2>

* [Plugin loading reference](/docs/pt/plugins/loading): por que escopos, o cache e precedência se comportam da maneira que fazem
* [Plugin commands reference](/docs/pt/plugins/cli-reference): sinalizadores, padrões, saída e códigos de saída para os comandos `claude plugin`
* [Install and manage plugins](/docs/pt/plugins/install): os passos de instalação desde o início
* [Manage plugins for your organization](/docs/pt/plugins/org#troubleshoot-policy): solução de problemas do lado da política para administradores
