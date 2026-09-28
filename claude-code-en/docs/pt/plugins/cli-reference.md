> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência de comandos de plugin

> Referência completa para os comandos de shell do plugin claude, /plugin e /reload-plugins em uma sessão, e as flags que carregam um plugin para uma sessão.

Você executa comandos de plugin como `claude plugin` do seu shell ou script, ou como `/plugin` e `/reload-plugins` dentro de uma sessão Claude Code. Esta referência fornece as flags, padrões, saída e códigos de saída de cada comando, junto com as duas flags que carregam um plugin para uma sessão.

Execute `claude plugin --help` em sua compilação para confirmar quais subcomandos sua versão possui.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Instalar e gerenciar etapas, e onde `/plugin` é executado**: veja [Instalar e gerenciar plugins](/docs/pt/plugins/install)
  * **O que um comando muda no disco e qual escopo tem precedência**: veja [Referência de carregamento de plugin](/docs/pt/plugins/loading)
  * **O que uma mensagem de erro significa**: veja [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting)
</Note>

<h2 id="claude-plugin-commands">
  Comandos claude plugin
</h2>

Execute `claude plugin <subcommand>` do seu shell ou script, fora de uma sessão Claude Code. Estes subcomandos instalam e gerenciam plugins sem abrir o painel [`/plugin`](#plugin-in-a-session).

`claude plugins` é um alias para `claude plugin`.

Cada subcomando compartilha estes códigos de saída, argumentos de plugin e valores de escopo:

* **Códigos de saída**: `0` em caso de sucesso e `1` em caso de falha. `validate` adiciona saída `2` para um erro inesperado, e `eval` adiciona os códigos listados em [sua seção](#plugin-eval).
* **Argumentos de plugin**: um argumento `<plugin>` é um `name` de plugin ou `name@marketplace`. Quando dois marketplaces oferecem o mesmo nome, use a forma qualificada.
* **Escopos**: `--scope` aceita `user`, `project` ou `local`, e nomeia o arquivo de configurações que o comando escreve. `update` também aceita `managed`.

<h3 id="plugin-init">
  plugin init
</h3>

Crie um novo plugin em `~/.claude/skills/<name>/`. Ele carrega em sua próxima sessão como `<name>@skills-dir` sem etapa de instalação.

`new` é um alias para `init`.

Para o fluxo de trabalho criar, testar e editar que começa com este comando, veja [Criar um plugin](/docs/pt/plugins/create).

```bash theme={null}
claude plugin init <name> [options]
```

`<name>` se torna o nome do diretório sob `~/.claude/skills/` e o `name` do plugin em seu manifesto.

O comando não possui flag para outro local. Para criar um scaffold dentro de um projeto, veja [Criar um plugin](/docs/pt/plugins/create).

| Flag                     | Descrição                                                                                                  |
| :----------------------- | :--------------------------------------------------------------------------------------------------------- |
| `--description <text>`   | Descrição do manifesto                                                                                     |
| `--author <name>`        | Nome do autor. Padrão é `git config user.name`                                                             |
| `--author-email <email>` | Email do autor. Padrão é `git config user.email`                                                           |
| `--with <components...>` | Também criar arquivos iniciais para `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style` ou `channel` |
| `-f, --force`            | Sobrescrever um `.claude-plugin/` existente no destino                                                     |

Criar um plugin com arquivos de skill e hook iniciais:

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code valida o que foi escrito e imprime `Created plugin "my-helper" at ~/.claude/skills/my-helper`, seguido pelo id que ele carrega como e o comando `claude plugin disable` que o desativa.

Claude Code sai com `1` sem escrever quando não consegue criar um scaffold com segurança, e a mensagem nomeia o motivo. Estes são motivos comuns:

* Um valor desconhecido em `--with`
* Um scaffold existente no destino sem `--force`
* Uma configuração gerenciada que bloqueia plugins de diretório de skills

<h3 id="plugin-install">
  plugin install
</h3>

Instale um plugin de um marketplace que você adicionou. `i` é um alias para `install`.

```bash theme={null}
claude plugin install <plugin> [options]
```

A maioria dos plugins é instalada sem um prompt. Para um plugin cujo marketplace [executa um comando para instalá-lo](/docs/pt/plugins/host-marketplace) ou [define um `headersHelper` para seu download](/docs/pt/plugins/host-marketplace#how-users-accept-a-headershelper-command), Claude Code primeiro imprime o comando e pergunta `Run this command now? [y/N]`.

| Flag                        | Descrição                                                                                                                                                                                                                                                                                                               |
| :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | Escopo de instalação: `user`, `project` ou `local`. Padrão é `user`                                                                                                                                                                                                                                                     |
| `--config <key=value>`      | Defina uma opção [`userConfig`](/docs/pt/plugins/manifest-reference) que o manifesto do plugin declara. Repita a flag para cada opção. Requer Claude Code v2.1.147 ou posterior                                                                                                                                              |
| `-y, --yes`                 | Aceite o comando de instalação exibido sem o prompt `Run this command now?`. Ignorado quando o comando é executado dentro de uma sessão Claude Code, como da ferramenta Bash ou um hook. Requer Claude Code v2.1.229 ou posterior                                                                                       |
| `--accept-command <sha256>` | Aceite o comando de instalação exibido cujo `sha256` uma execução anterior [`--json`](#plugin-json-result) relatou em `shownCommand`, no lugar de `-y`. Não pode ser combinado com `-y`. Veja [Aceitar um comando de instalação exibido](#accept-a-displayed-install-command). Requer Claude Code v2.1.271 ou posterior |
| `--json`                    | Imprima o resultado como um objeto JSON na última linha de stdout em vez da mensagem legível por humanos, para uso em scripts. Veja [Formato de resultado JSON](#plugin-json-result). Requer Claude Code v2.1.268 ou posterior                                                                                          |

Passe `-y` do seu próprio terminal para aceitar o comando exibido sem o prompt. Aqui está o que acontece sem um TTY e quando Claude executa o comando:

* **stdin ou stdout não é um TTY, e você não passa `-y` nem `--accept-command`**: a instalação é recusada. A saída diz que o comando foi apenas exibido, e o código de saída é `1`
* **Claude executa o comando através de sua ferramenta Bash**: `-y` é ignorado. Execute o comando do seu próprio terminal

Instale um plugin para todos que clonam o projeto:

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code imprime `Successfully installed plugin: formatter@my-marketplace (scope: project)`. Quando nada novo é instalado, a saída diz por quê:

* **Já instalado nesse escopo**: a saída é `Plugin "formatter@my-marketplace" is already installed (scope: project)` e o código de saída é `0`
* **Você recusa um prompt de origem de comando**: a saída é `Aborted.` e o código de saída é `1`
* **Você recusa um prompt `headersHelper`, ou não pode ser confirmado sem um TTY**: a saída é `Aborted — the command was not run.` e o código de saída é `1`

<h4 id="plugin-json-result">
  Formato de resultado JSON
</h4>

Quando você passa `--json` para `plugin install`, a última linha de stdout é um objeto JSON. Analise apenas essa linha, porque Claude Code imprime qualquer comando que o marketplace declara antes dela.

Três campos estão sempre presentes:

* `command`: o subcomando que foi executado, como `install`
* `outcome`: `ok` ou `failed`
* `message`: uma descrição legível por humanos do resultado

Outros campos, como `pluginId`, `scope` e `failureCode`, aparecem apenas quando se aplicam.

A opção `--json` em `plugin uninstall`, `plugin update`, `plugin enable` e `plugin disable` imprime o mesmo objeto com os próprios campos desse subcomando.

Um erro de uso, como um `--scope` inválido, não imprime nenhuma linha de resultado e sai com `1` com o motivo em stderr.

<h4 id="accept-a-displayed-install-command">
  Aceitar um comando de instalação exibido
</h4>

Quando uma execução `--json` exibe um comando declarado pelo marketplace e não o executa, o resultado `failed` também carrega um objeto `shownCommand`. Seus campos incluem o comando conforme exibido, o plugin ao qual pertence e o `sha256` do comando.

Para aceitar exatamente esse comando, execute novamente com esse `sha256` como `--accept-command` do seu próprio terminal, porque a flag não tem efeito dentro de uma sessão Claude Code. Requer Claude Code v2.1.271 ou posterior.

O `sha256` conta como aceitação para exatamente esse comando, plugin e catálogo de marketplace. Se qualquer um deles mudou desde que o comando foi exibido, Claude Code não aceita o `sha256` e mostra o comando novamente. Uma mudança que a atualização do marketplace da própria execução busca também conta como tal mudança.

Se `shownCommand.acceptCommandMatched` for `false`, o `sha256` que você passou não corresponde ao comando agora exibido. Revise esse comando antes de executar novamente com seu `sha256`.

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Remova um plugin instalado de um escopo. `remove` e `rm` são aliases para `uninstall`.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| Flag                  | Descrição                                                                                                                                                                                                              |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Desinstale do escopo: `user`, `project` ou `local`. Padrão é `user`                                                                                                                                                    |
| `--keep-data`         | Preserve o diretório de dados persistentes do plugin, `~/.claude/plugins/data/<id>/`                                                                                                                                   |
| `--prune`             | Também remova [dependências](/docs/pt/plugins/dependencies) auto-instaladas que nenhum plugin restante precisa                                                                                                              |
| `-y, --yes`           | Pule o prompt de confirmação `--prune`. Necessário com `--prune` quando stdin ou stdout não é um TTY                                                                                                                   |
| `--json`              | Imprima o resultado como um objeto JSON na última linha de stdout, no [mesmo formato que `plugin install --json`](#plugin-json-result). Não pode ser combinado com `--prune`. Requer Claude Code v2.1.268 ou posterior |

Desinstale um plugin do escopo do projeto:

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code imprime `Successfully uninstalled plugin: formatter (scope: project)`. Quando o plugin não está instalado nesse escopo, o comando imprime uma linha que começa com `Failed to uninstall plugin "formatter@my-marketplace":` e sai com `1`.

<h3 id="plugin-enable">
  plugin enable
</h3>

Ative um plugin desativado. Para um [plugin sincronizado de claude.ai](/docs/pt/plugins/loading#synced-plugins), passe `<name>@synced` como o plugin.

```bash theme={null}
claude plugin enable <plugin> [options]
```

| Flag                  | Descrição                                                                                                                                                                        |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Escopo para ativar: `user`, `project` ou `local`. Auto-detectado quando omitido                                                                                                  |
| `--json`              | Imprima o resultado como um objeto JSON na última linha de stdout, no [mesmo formato que `plugin install --json`](#plugin-json-result). Requer Claude Code v2.1.268 ou posterior |

Sem `--scope`, o comando verifica seus arquivos de configurações na ordem local, projeto, usuário, e usa o primeiro escopo que menciona o plugin.

Se você passar um `--scope` onde o plugin não está declarado, o comando escreve uma substituição ou falha:

* **Um escopo que [tem precedência](/docs/pt/plugins/loading) sobre o que o declara**: Claude Code escreve uma substituição no escopo que você passou. Por exemplo, `claude plugin disable formatter --scope local` desativa um plugin ativado no projeto apenas para você
* **Qualquer outro escopo**: o comando falha com `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.`

Se o plugin já está ativado no escopo resolvido, o comando imprime `Plugin "formatter" is already enabled` e sai com `1`. Com `--json`, o resultado tem `"failureCode": "already_in_goal_state"` e `"alreadyInGoalState": true`, então um script pode tratar esse caso como sucesso.

Quando o plugin declara [dependências](/docs/pt/plugins/dependencies), Claude Code as ativa também. O comando falha nestes casos:

* **Uma dependência não está instalada**: a ativação falha e imprime o comando `claude plugin install` para cada dependência ausente
* **Uma dependência é bloqueada pela política de plugin da sua organização**: a ativação falha e nomeia a dependência bloqueada
* **Uma dependência é definida como `false` em um escopo com precedência maior que o escopo de destino**: a ativação falha. Ative a dependência nesse escopo, ou passe `--scope` para escrever lá

Reative um plugin onde quer que seja declarado:

```bash theme={null}
claude plugin enable formatter
```

Claude Code imprime `Successfully enabled plugin: formatter (scope: project)`, nomeando o escopo que detectou.

<h3 id="plugin-disable">
  plugin disable
</h3>

Desative um plugin sem desinstalá-lo. Para um [plugin sincronizado de claude.ai](/docs/pt/plugins/loading#synced-plugins), passe `<name>@synced` como o plugin.

```bash theme={null}
claude plugin disable [plugin] [options]
```

| Flag                  | Descrição                                                                                                                                                                        |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-a, --all`           | Desative todos os plugins ativados. Não pode ser combinado com um nome de plugin ou `--scope`                                                                                    |
| `-s, --scope <scope>` | Escopo para desativar: `user`, `project` ou `local`. Auto-detectado quando omitido                                                                                               |
| `--json`              | Imprima o resultado como um objeto JSON na última linha de stdout, no [mesmo formato que `plugin install --json`](#plugin-json-result). Requer Claude Code v2.1.268 ou posterior |

Sem `--scope`, o escopo é auto-detectado na mesma ordem local, projeto, usuário que [`plugin enable`](#plugin-enable).

Se você não passar nem um nome de plugin nem `--all`, Claude Code imprime `Please specify a plugin name or use --all to disable all plugins` e sai com `1`. Desativar um plugin que já está desativado imprime `Plugin "formatter" is already disabled` e sai com `1`, como [`plugin enable`](#plugin-enable) faz para um plugin já ativado.

O comando falha para um plugin que ainda é necessário:

* **Outro plugin ativado [depende](/docs/pt/plugins/dependencies) dele**: o comando falha e nomeia os dependentes para desativar primeiro
* **Sua organização o requer como um plugin sincronizado**: o comando falha e não salva nada

Desative um plugin:

```bash theme={null}
claude plugin disable formatter
```

Claude Code imprime `Successfully disabled plugin: formatter (scope: project)`.

<h3 id="plugin-update">
  plugin update
</h3>

Atualize um plugin para a versão mais recente que seu marketplace oferece. A nova versão carrega em sua próxima sessão, ou depois que você executa `/reload-plugins` em uma em execução.

```bash theme={null}
claude plugin update <plugin> [options]
```

| Flag                        | Descrição                                                                                                                                                                                                                                               |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-s, --scope <scope>`       | Escopo para atualizar: `user`, `project`, `local` ou `managed`. Padrão é o escopo em que o plugin está instalado                                                                                                                                        |
| `-y, --yes`                 | Aceite um comando de instalação alterado de um plugin [command-source](/docs/pt/plugins/host-marketplace), sem o prompt. Necessário quando stdin ou stdout não é um TTY, a menos que você passe `--accept-command`. Requer Claude Code v2.1.229 ou posterior |
| `--accept-command <sha256>` | Aceite o comando declarado pelo marketplace cujo `sha256` uma execução anterior [`--json`](#plugin-json-result) relatou em `shownCommand`, no lugar de `-y`. Não pode ser combinado com `-y`. Requer Claude Code v2.1.271 ou posterior                  |
| `--json`                    | Imprima o resultado como um objeto JSON na última linha de stdout, no [mesmo formato que `plugin install --json`](#plugin-json-result). Requer Claude Code v2.1.268 ou posterior                                                                        |

`managed` é o único escopo que você pode atualizar mas não instalar. Para plugins instalados por admin, veja [Gerenciar plugins para sua organização](/docs/pt/plugins/org).

Atualize um plugin:

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code imprime `Checking for updates for plugin "formatter@my-marketplace"…`, depois o resultado. Quando nada é mais recente, imprime `formatter is already at the latest version (1.0.0).` e sai com `0`.

Você pode passar um nome de plugin simples, que o comando corresponde contra seus plugins instalados. Quando plugins instalados de diferentes marketplaces compartilham o nome, o comando recusa a atualização e lista os comandos `plugin-name@marketplace-name` qualificados para executar. Atualizar por nome simples requer Claude Code v2.1.246 ou posterior.

<h3 id="plugin-list">
  plugin list
</h3>

Liste plugins instalados com sua versão, escopo e status.

```bash theme={null}
claude plugin list [options]
```

| Flag          | Descrição                                                                                              |
| :------------ | :----------------------------------------------------------------------------------------------------- |
| `--json`      | Imprima a lista como JSON                                                                              |
| `--available` | Também liste plugins que seus marketplaces oferecem que você não instalou. Não tem efeito sem `--json` |

Claude Code agrupa a saída legível por humanos por como cada plugin carrega:

* **`Installed plugins:`**: plugins que você instalou de um marketplace
* **`Session-only plugins (--plugin-dir / --plugin-url):`**: plugins carregados por essas flags no mesmo comando, como em `claude --plugin-dir ./my-plugin plugin list`
* **`Skills-directory plugins (.claude/skills/*):`**: plugins que Claude Code encontrou em um diretório de skills
* **`Synced from claude.ai`**: [plugins sincronizados de sua conta claude.ai](/docs/pt/plugins/loading#synced-plugins)

Sem nada em nenhum grupo, Claude Code imprime ``No plugins installed. Use `claude plugin install` to install a plugin.``

<h4 id="json-output">
  Saída JSON
</h4>

Com `--json`, Claude Code imprime um array com um objeto por instalação. Cada objeto carrega os campos abaixo. `id`, `version`, `scope`, `enabled` e `installPath` estão sempre presentes, e os outros aparecem apenas quando se aplicam.

| Campo          | Tipo             | Descrição                                                                                                                                                                                                                                                               |
| :------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`           | string           | `name@marketplace` para installs, `name@inline` para plugins de sessão única, `name@skills-dir` para plugins de diretório de skills, `name@synced` para plugins sincronizados de claude.ai                                                                              |
| `version`      | string           | Para uma instalação de marketplace, a [versão que Claude Code computou](/docs/pt/plugins/loading#versions-and-updates) na instalação. Para um plugin de sessão única, diretório de skills ou sincronizado, a `version` do manifesto, ou `unknown` quando não declara nenhuma |
| `scope`        | string           | `user`, `project`, `local` ou `managed` para installs; `user` ou `project` para plugins de diretório de skills; `session` para plugins de sessão única; `synced` para plugins sincronizados de claude.ai                                                                |
| `enabled`      | boolean          | Se o plugin está ativado em suas configurações mescladas                                                                                                                                                                                                                |
| `installPath`  | string           | Diretório do qual o plugin carrega                                                                                                                                                                                                                                      |
| `installedAt`  | string           | Timestamp ISO da instalação. Apenas installs de marketplace                                                                                                                                                                                                             |
| `lastUpdated`  | string           | Timestamp ISO da última atualização. Apenas installs de marketplace                                                                                                                                                                                                     |
| `projectPath`  | string           | Projeto ao qual a instalação pertence. Apenas escopo `project` e `local`                                                                                                                                                                                                |
| `mcpServers`   | object           | As definições de servidor MCP do plugin, quando um plugin instalado de marketplace tem alguma                                                                                                                                                                           |
| `errors`       | array of strings | Erros de carregamento, quando o plugin falhou ao carregar                                                                                                                                                                                                               |
| `notes`        | array of strings | Avisos de autoria para um plugin que carregou e funciona                                                                                                                                                                                                                |
| `errorDetails` | array of objects | Um objeto por entrada `errors`, fornecendo seu `type` de diagnóstico e os nomes aos quais se refere, como o plugin, marketplace, servidor ou arquivo. Requer Claude Code v2.1.268 ou posterior                                                                          |
| `noteDetails`  | array of objects | Os mesmos objetos de detalhe para cada entrada `notes`. Requer Claude Code v2.1.268 ou posterior                                                                                                                                                                        |

Com `--json --available`, Claude Code imprime um objeto em vez de um array. Seu campo `installed` contém o array de objetos de plugin instalado, e seu campo `available` contém um objeto por plugin de marketplace não instalado com os campos abaixo.

| Campo             | Tipo             | Descrição                                                                                                                              |
| :---------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `pluginId`        | string           | `name@marketplace`                                                                                                                     |
| `name`            | string           | O nome do plugin no marketplace                                                                                                        |
| `marketplaceName` | string           | O marketplace que o oferece                                                                                                            |
| `source`          | string or object | A [source](/docs/pt/plugins/marketplace-reference) da entrada do marketplace: uma string para um caminho relativo, um objeto caso contrário |
| `description`     | string           | A descrição da entrada, quando tem uma                                                                                                 |
| `version`         | string           | A versão da entrada, quando declara uma                                                                                                |
| `installCount`    | number           | Contagem de instalações, quando Claude Code tem uma para o plugin                                                                      |

<h3 id="plugin-details">
  plugin details
</h3>

Mostre o inventário de componentes de um plugin e seu custo de token projetado.

O plugin deve estar carregado: instalado, encontrado em um diretório de skills, ou passado com `--plugin-dir` ou `--plugin-url` no mesmo comando. O `<name>` é um `name` de plugin ou `name@marketplace`.

```bash theme={null}
claude plugin details <name>
```

O comando não toma flags além de `--help`.

Mostre o que um plugin instalado contribui:

```bash theme={null}
claude plugin details formatter
```

Claude Code imprime o nome, versão, descrição e fonte do plugin, depois estas seções:

* **`Component inventory`**: os skills, agents, hooks, servidores MCP e servidores LSP do plugin
* **`Projected token cost`**: os tokens sempre ativados que o plugin adiciona a cada sessão
* **`Per-component (rounded)`**: estimativas sempre ativadas e sob invocação para cada skill, agent e comando. Omitido quando o plugin não tem nenhum

Para o que as duas figuras de custo significam, veja [Medir custo e uso de plugin](/docs/pt/plugins/measure).

Para um plugin que não está carregado, Claude Code imprime ``Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.`` e sai com `1`.

<h3 id="plugin-prune">
  plugin prune
</h3>

Remova [dependências](/docs/pt/plugins/dependencies) auto-instaladas que nenhum plugin instalado precisa mais. O comando nunca remove um plugin que você instalou você mesmo. `autoremove` é um alias para `prune`.

```bash theme={null}
claude plugin prune [options]
```

| Flag                  | Descrição                                                                    |
| :-------------------- | :--------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Prune no escopo: `user`, `project` ou `local`. Padrão é `user`               |
| `--dry-run`           | Liste o que seria removido sem remover                                       |
| `-y, --yes`           | Pule o prompt de confirmação. Necessário quando stdin ou stdout não é um TTY |

Visualize o que um prune removeria:

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code lista as dependências órfãs e termina com `(dry run — nothing removed)`. Sem nada para remover, imprime uma linha que começa com `Nothing to prune`.

Sem `--dry-run`, o comando remove as dependências órfãs apenas depois que você confirma no prompt ou passa `-y`.

O código de saída é `0` qualquer que seja sua resposta no prompt.

O que `prune` faz depende se um terminal está anexado e se você passa `-y`:

| Terminal e flags                  | O que acontece                                                                            |
| :-------------------------------- | :---------------------------------------------------------------------------------------- |
| Terminal interativo, sem `-y`     | Lista as dependências órfãs e pergunta `Remove? [y/N]`                                    |
| Qualquer terminal, `-y`           | Remove-as e imprime `Removed N auto-installed plugins: <names>`                           |
| stdin ou stdout não-TTY, sem `-y` | Imprime a lista e ``Not a TTY — run `claude plugin prune -y` to remove.``, removendo nada |

<h3 id="plugin-eval">
  plugin eval
</h3>

Execute [casos de eval](/docs/pt/plugin-evals) de um plugin e relate resultados pontuados. Requer Claude Code v2.1.269 ou posterior.

Cada caso é um prompt mais avaliadores. Claude Code o executa várias vezes em uma sessão isolada com apenas o plugin de destino carregado, e por padrão também sem o plugin para que o relatório mostre a diferença.

Veja [Testar plugins com evals](/docs/pt/plugin-evals) para o formato de caso, avaliadores, resultados e uso de CI.

```bash theme={null}
claude plugin eval [target] [options]
```

O `target` opcional padrão é o diretório atual e toma qualquer uma destas formas:

* Um diretório de plugin
* Um único arquivo `prompt.md` ou `case.yaml`
* Um plugin instalado como `name` ou `name@marketplace`
* `name@skills-dir`

Coloque o target antes de `--tag`, `--allow-tools` e `--json`. Cada uma dessas opções toma as palavras que a seguem como seu valor, então um target escrito após uma delas é lido como uma tag, um nome de ferramenta ou o caminho de saída JSON em vez de como o target.

Esta tabela lista as opções que a maioria das execuções usa. Execute `claude plugin eval --help` para o conjunto completo, incluindo `--case`, `--tag`, `--output-dir`, `--report`, `--allow-real-servers`, `--keep-temp` e `--verbose`.

| Opção                      | Descrição                                                                                                                                                                             | Padrão                                                                                 |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------- |
| `--runs <n>`               | Execuções por caso em cada [arm](/docs/pt/plugin-evals#compare-against-a-no-plugin-baseline)                                                                                               | `runs` de cada caso, senão 3                                                           |
| `-j, --concurrency <n>`    | Sessões de agente para executar de uma vez, 1 a 8. Elas compartilham seu limite de taxa                                                                                               | `1`                                                                                    |
| `--model <model>`          | Modelo para o agente sob teste                                                                                                                                                        | `model` de cada caso, senão `ANTHROPIC_MODEL` se definido, senão padrão de Claude Code |
| `--judge-model <model>`    | Modelo para avaliadores `llm` e `baseline`                                                                                                                                            | Um modelo pequeno e rápido                                                             |
| `--ablation <mode>`        | `none` ou `with-without`. Veja [Comparar contra uma linha de base sem plugin](/docs/pt/plugin-evals#compare-against-a-no-plugin-baseline)                                                  | `with-without` quando um plugin se resolve, senão `none`                               |
| `--threshold <0..1>`       | Saia com 1 se algum caso pontuar abaixo disso                                                                                                                                         | `1.0`                                                                                  |
| `--max-cost-usd <usd>`     | Pare antes da próxima execução uma vez que o gasto atinja isso, saia com 2 e relate resultados parciais                                                                               | Sem limite                                                                             |
| `--allow-tools <tools...>` | Conceda ferramentas além do conjunto somente leitura, como `Bash`, `Write`, `Edit` ou `"mcp__plugin_<plugin>_<server>__*"`. Veja [Conceder ferramentas](/docs/pt/plugin-evals#grant-tools) |                                                                                        |
| `--scaffold`               | Execute o [`scaffold_script`](/docs/pt/plugin-evals#add-setup-or-history-with-case-yaml) de cada caso                                                                                      | Desativado                                                                             |
| `--trust-plugin`           | Pule o prompt de confiança da primeira execução, para CI. Veja [O que uma execução pode acessar](/docs/pt/plugin-evals#security)                                                           | Desativado                                                                             |
| `--mocks <mode>`           | `record` ou `off`. Veja [Mock MCP servers](/docs/pt/plugin-evals#mock-mcp-servers)                                                                                                         | `record`                                                                               |
| `--eval-dir <dir>`         | Diretório abaixo do plugin que contém os casos                                                                                                                                        | O `experimental.evals` do manifesto, senão `evals`                                     |
| `--json [path]`            | Imprima o [documento de resultado](/docs/pt/plugin-evals#json-result) para stdout, ou escreva-o em um caminho `.json`                                                                      |                                                                                        |
| `--no-publish`             | Mantenha o relatório HTML local                                                                                                                                                       |                                                                                        |

O código de saída relata como a execução terminou. Para agir sobre ele em um pipeline, veja [Executar evals em CI](/docs/pt/plugin-evals#run-evals-in-ci).

| Código de saída | Significado                                                                       |
| :-------------- | :-------------------------------------------------------------------------------- |
| `0`             | Cada caso atende ao threshold                                                     |
| `1`             | Um caso falhando, um erro de carregamento ou um diretório de plugin não confiável |
| `2`             | Uma execução parcial                                                              |
| `130`           | Interrompido                                                                      |
| `143`           | Terminado                                                                         |

<h3 id="plugin-eval-init">
  plugin eval init
</h3>

Crie um conjunto de eval para o plugin no diretório atual. Requer Claude Code v2.1.269 ou posterior. Veja [Criar seu primeiro conjunto de eval](/docs/pt/plugin-evals#create-your-first-eval-suite).

```bash theme={null}
claude plugin eval init [name] [options]
```

Em um terminal, o comando abre uma sessão Claude Code interativa para uma entrevista de autoria. Na entrevista, Claude faz o seguinte:

1. Lê o plugin
2. Pergunta o que ele deveria fazer bem
3. Propõe casos e avaliadores
4. Escreve os arquivos de caso
5. Executa os casos e revisa as notas com você para verificar que os avaliadores pontuam da forma que você faria

Com `--bare`, ou sem um terminal, o comando escreve um modelo de caso único em branco. Quando Claude executa o comando de dentro de uma sessão Claude Code, o comando imprime as instruções da entrevista para essa sessão seguir em vez de escrever um modelo.

O `name` opcional é um nome de caso. É necessário com `--bare` ou sem um terminal, porque o comando escreve o modelo em branco para esse caso. A entrevista não precisa de um.

O comando aceita estas opções:

| Opção               | Descrição                                                                                              | Padrão                                             |
| :------------------ | :----------------------------------------------------------------------------------------------------- | :------------------------------------------------- |
| `--bare`            | Escreva um `prompt.md` e `graders/criteria.md` em branco para `<name>` em vez de executar a entrevista |                                                    |
| `-i, --interactive` | Exija a entrevista. Falha sem um terminal em vez de escrever um modelo                                 |                                                    |
| `--eval-dir <dir>`  | Diretório abaixo do diretório atual para escrever casos                                                | O `experimental.evals` do manifesto, senão `evals` |

<h3 id="plugin-tag">
  plugin tag
</h3>

Crie uma tag git anotada nomeada `<name>--v<version>` para uma versão de plugin. Antes de marcar, o comando verifica se o `plugin.json` do plugin e qualquer entrada de marketplace que o liste concordam sobre a versão.

Para quando marcar uma versão, veja [Publicar um plugin](/docs/pt/plugins/publish).

```bash theme={null}
claude plugin tag [path] [options]
```

O `[path]` é o diretório do plugin, padronizando para o diretório atual. O comando encontra a entrada do marketplace caminhando para cima a partir desse diretório para um `.claude-plugin/marketplace.json` que lista o plugin.

| Flag                  | Descrição                                                                          |
| :-------------------- | :--------------------------------------------------------------------------------- |
| `--push`              | Empurre a tag para `--remote` após criá-la                                         |
| `--dry-run`           | Imprima o que seria marcado sem criar a tag                                        |
| `-f, --force`         | Pule as verificações de árvore de trabalho suja e tag já existe                    |
| `-m, --message <msg>` | Mensagem de anotação de tag. `%s` representa a versão. Padrão é `<name> <version>` |
| `--remote <name>`     | Remote para empurrar com `--push`. Padrão é `origin`                               |

Visualize a tag para um plugin em um checkout de marketplace:

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code imprime o plano:

* O nome do plugin
* A versão e qual arquivo ela veio
* A entrada de marketplace correspondente, quando há uma
* O nome da tag
* Os comandos `git tag` e `git push` que executaria

Sem `--dry-run`, Claude Code imprime `Created tag formatter--v1.0.0` e `Pushed to origin` ou o comando push para você executar você mesmo. Se o push falhar, a tag ainda é criada localmente e o comando sai com um erro.

O comando sai com `1` e imprime o motivo quando não consegue marcar com segurança. Motivos comuns são:

* Sem `version` em `plugin.json` ou na entrada do marketplace
* A tag já existe
* A árvore de trabalho está suja

<h3 id="plugin-validate">
  plugin validate
</h3>

Valide um manifesto de plugin, um manifesto de marketplace, ou os skills, agents e comandos em um diretório, e saia com um código que um trabalho de CI pode agir. Para o fluxo de trabalho criar, testar e editar, veja [Criar um plugin](/docs/pt/plugins/create). Para o que o validador verifica em cada manifesto, veja a [referência de manifesto de plugin](/docs/pt/plugins/manifest-reference) e a [referência de marketplace](/docs/pt/plugins/marketplace-reference).

```bash theme={null}
claude plugin validate <path> [options]
```

| Flag       | Descrição                                                                                                                                                     |
| :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--strict` | Trate avisos como erros, então campos não reconhecidos e metadados ausentes que o runtime tolera falham na execução. Requer Claude Code v2.1.145 ou posterior |
| `--json`   | Saída do relatório de validação como um objeto JSON com os mesmos códigos de saída. Requer Claude Code v2.1.259 ou posterior                                  |

Valide um plugin antes de fazer commit:

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h4 id="validate-a-directory">
  Validar um diretório
</h4>

O `<path>` é um arquivo de manifesto ou um diretório. Dado um diretório, Claude Code escolhe o que validar pelo que encontra lá:

* `.claude-plugin/marketplace.json`, quando existe
* Caso contrário `.claude-plugin/plugin.json`
* Caso contrário os arquivos de componente, escolhidos pelo nome do diretório. Validar arquivos de componente sem um manifesto requer Claude Code v2.1.233 ou posterior:
  * Um diretório nomeado `skills`, `agents` ou `commands`: os arquivos dentro dele
  * Um diretório nomeado `.claude`: os diretórios `skills`, `agents` e `commands` dentro dele
  * Qualquer outro diretório: esses três diretórios sob seu `.claude`

Claude Code não segue symlinks dentro do diretório que você nomeia. O que faz depende de onde o link está:

* **Um diretório `skills`, `agents` ou `commands` vinculado sob a raiz do plugin ou `.claude`**: Claude Code avisa que nada nele foi lido.
* **Uma entrada vinculada dentro de um diretório `skills`, `agents` ou `commands`**: Claude Code a pula e avisa, por diretório, quantas entradas pulou que uma sessão carregaria.
* **O diretório `skills`, `agents` ou `commands` que você nomeia é ele mesmo um symlink, ou seu diretório pai `.claude` é**: Claude Code relata um erro e não verifica nada nele. Nomeie o diretório real.

Alguns arquivos não são lidos por uma execução de validação:

* **Um `SKILL.md` na raiz do plugin**: quando você executa `claude plugin validate` contra um diretório de plugin, Claude Code não verifica um `SKILL.md` na raiz do plugin
* **Um `CLAUDE.md` na raiz do plugin**: em uma execução de plugin, Claude Code também avisa sobre um `CLAUDE.md` na raiz do plugin
* **Arquivos de plugin em uma execução de marketplace**: de um diretório de marketplace, Claude Code não abre os arquivos de skill, agent, comando ou hook dos plugins. Para encontrar erros nesses arquivos, valide cada diretório de plugin

<h4 id="output-and-exit-codes">
  Saída e códigos de saída
</h4>

Claude Code imprime o arquivo que validou, quaisquer erros e avisos com seus caminhos, e uma linha de veredicto. O código de saída segue o veredicto:

| Código de saída | Linha de veredicto                                                              | Significado                                            |
| :-------------- | :------------------------------------------------------------------------------ | :----------------------------------------------------- |
| `0`             | `Validation passed` ou `Validation passed with warnings`                        | O manifesto carrega. Com `--strict`, sem avisos também |
| `1`             | `Validation failed` ou `Validation failed (--strict treats warnings as errors)` | Um erro, ou um aviso sob `--strict`                    |
| `2`             | `Unexpected error during validation: <reason>`                                  | O validador em si falhou, como em um caminho ilegível  |

Com `--json`, Claude Code escreve o relatório para stdout como um objeto JSON com estes campos de nível superior:

* `success`: o mesmo veredicto que o código de saída fornece
* `strict`: se a execução tratou avisos como erros
* `target`: o caminho resolvido que Claude Code validou
* `manifest`: o resultado do próprio manifesto, ou `null` para uma execução sem manifesto
* `contents`: resultados por arquivo, cada um nomeando seu `file` e carregando arrays `errors`, `warnings` e `notes`

Na saída `2`, o comando não escreve nada para stdout. A mensagem de erro vai para stderr.

<h2 id="claude-plugin-marketplace-commands">
  Comandos claude plugin marketplace
</h2>

Execute `claude plugin marketplace <subcommand>` do seu shell para adicionar, listar, atualizar e remover os marketplaces dos quais você instala plugins.

* **Códigos de saída**: estes subcomandos seguem a [convenção de código de saída](#claude-plugin-commands) dos comandos de plugin
* **Escopos**: sua flag `--scope` não tem forma curta `-s`

Para o que é um marketplace e como Claude Code o armazena em cache, veja [Referência de carregamento de plugin](/docs/pt/plugins/loading).

<h3 id="plugin-marketplace-add">
  plugin marketplace add
</h3>

Adicione um marketplace de um repositório GitHub, uma URL git, um `marketplace.json` hospedado ou um caminho local, e declare-o em um arquivo de configurações.

Depois de adicioná-lo, Claude Code instala quaisquer [dependências](/docs/pt/plugins/dependencies) que seus plugins instalados estavam perdendo.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| Flag                  | Descrição                                                                                                                                                                     |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>`     | Arquivo de configurações para declarar o marketplace: `user`, `project` ou `local`. Padrão é `user`                                                                           |
| `--sparse <paths...>` | Limite o checkout git a estes diretórios, para monorepos. Apenas fontes `github` e `git`                                                                                      |
| `--claudeai`          | Leia o argumento como o nome de um [marketplace hospedado em claude.ai](/docs/pt/plugins/install#add-from-claude-ai) em vez de uma fonte. Requer Claude Code v2.1.273 ou posterior |

`<source>` toma qualquer uma das formas na tabela abaixo, e sua forma decide o tipo de fonte e como Claude Code busca o marketplace. Para o objeto de fonte resultante, veja a [referência de marketplace](/docs/pt/plugins/marketplace-reference).

| Você digita                                                                                 | Tipo de fonte | Como Claude Code a busca                                                                                                       |
| :------------------------------------------------------------------------------------------ | :------------ | :----------------------------------------------------------------------------------------------------------------------------- |
| `owner/repo`, `owner/repo#ref` ou `owner/repo@ref`                                          | `github`      | Clona o repositório GitHub, fixado a `ref` quando fornecido. Proprietário e repo devem seguir regras de nomenclatura do GitHub |
| `user@host:path[.git][#ref]`                                                                | `git`         | Clona sobre SSH                                                                                                                |
| `https://example.com/repo.git[#ref]` ou uma URL contendo `/_git/`                           | `git`         | Clona sobre HTTPS, incluindo URLs do Azure DevOps                                                                              |
| `https://github.com/owner/repo` ou `https://gitlab.com/namespace/project`                   | `git`         | Clona sobre HTTPS após anexar `.git`                                                                                           |
| Qualquer outra URL `http://` ou `https://`, incluindo um host git auto-hospedado sem `.git` | `url`         | Busca a URL como um `marketplace.json`. Para clonar um repositório lá, anexe `.git`                                            |
| `./path`, `../path`, `/path` ou `~/path` para um diretório                                  | `directory`   | Lê o diretório no local. No Windows, formas `.\`, `..\` e `C:\` também funcionam                                               |
| Os mesmos formulários de caminho, para um arquivo `.json`                                   | `file`        | Lê o arquivo no local                                                                                                          |

Para um host cujas URLs de clone não carregam o sufixo `.git`, como AWS CodeCommit, adicione o marketplace como uma entrada git em [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces). Claude Code clona uma entrada git independentemente de sua URL terminar em `.git`.

Claude Code também clona uma URL `gitlab.com` com subgrupos aninhados, como `https://gitlab.com/group/subgroup/project`.

Adicione um marketplace e compartilhe-o com o projeto:

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code imprime `Successfully added marketplace: your-marketplace (declared in project settings)`, usando o `name` do próprio manifesto do marketplace. Uma adição repetida ou uma fonte inválida imprime um destes resultados:

* **Marketplace já no disco**: a saída é `Marketplace 'your-marketplace' already on disk — declared in project settings` e o código de saída é `0`
* **Fonte não reconhecida**: a saída é `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` e o código de saída é `1`
* **Host simples como `gitlab.example.com/team/plugins`**: a adição falha como um atalho `owner/repo` inválido, e a mensagem diz para adicionar `https://` ou usar um caminho local

Adicione um [marketplace hospedado em claude.ai](/docs/pt/plugins/install#add-from-claude-ai) pelo nome impresso na seção `From claude.ai:` de `claude plugin marketplace list`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Com `--claudeai`, o comando recusa `--scope` e `--sparse`. O marketplace é hospedado para sua conta, não declarado em um arquivo de configurações, então você não pode compartilhá-lo através do `.claude/settings.json` de um projeto.

<h3 id="plugin-marketplace-list">
  plugin marketplace list
</h3>

Liste cada marketplace que você adicionou, com sua fonte.

```bash theme={null}
claude plugin marketplace list [options]
```

| Flag     | Descrição                 |
| :------- | :------------------------ |
| `--json` | Imprima a lista como JSON |

Claude Code imprime `Configured marketplaces:` e uma linha `Source:` por marketplace, ou `No marketplaces configured`.

Com `--json`, Claude Code imprime um array com um objeto por marketplace, carregando os campos abaixo. Cada campo é uma string.

| Campo             | Descrição                                                             |
| :---------------- | :-------------------------------------------------------------------- |
| `name`            | O nome do marketplace                                                 |
| `source`          | `github`, `git`, `url`, `directory`, `file` ou `claudeai`             |
| `repo`            | `owner/repo`. Apenas fontes `github`                                  |
| `url`             | A URL de clone ou busca. Apenas fontes `git` e `url`                  |
| `path`            | O caminho local. Apenas fontes `directory` e `file`                   |
| `ref`             | O branch ou tag fixado. Fontes `github` e `git`, apenas quando fixado |
| `installLocation` | Onde Claude Code armazenou em cache o marketplace                     |

Um marketplace [claude.ai](/docs/pt/plugins/install#add-from-claude-ai) adicionado não tem clone local, então sua entrada carrega seus identificadores claude.ai, `marketplaceId` e `organizationUuid`, no lugar de `installLocation`. Também carrega `scope` quando um é registrado, e `status`.

Se suas sessões de terminal [sincronizam plugins de sua conta claude.ai](/docs/pt/plugins/loading#synced-plugins), a listagem de texto termina com uma seção `From claude.ai:`. Essa seção nomeia os marketplaces que claude.ai lista para sua conta que você não adicionou, tanto baseados em git quanto hospedados. Requer Claude Code v2.1.273 ou posterior.

Para adicionar um marketplace dessa seção, veja [Adicionar um marketplace de claude.ai](/docs/pt/plugins/install#add-from-claude-ai).

A saída `--json` cobre apenas marketplaces configurados e deixa a seção de fora.

<h3 id="plugin-marketplace-remove">
  plugin marketplace remove
</h3>

Remova a declaração de um marketplace de suas configurações. `rm` é um alias para `remove`.

<Warning>
  Quando você remove um marketplace do último escopo que o declara, Claude Code também exclui seu cache e desinstala cada plugin que você instalou dele. Sem `--scope`, o comando remove a declaração de cada escopo. Para atualizar um marketplace sem perder seus plugins, execute `plugin marketplace update`.
</Warning>

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

O `<name>` é o nome do marketplace que `plugin marketplace list` mostra, não a fonte que você passou para `add`.

| Flag              | Descrição                                                                                                                                |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>` | Remova a declaração de um escopo de configurações: `user`, `project` ou `local`. Sem ele, Claude Code remove a declaração de cada escopo |

Remova um marketplace de cada escopo:

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code imprime `Successfully removed marketplace: your-marketplace`, adicionando `(from project settings)` quando você o escopo. Se você escopar para um arquivo de configurações que não declara o marketplace, o comando falha com `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.`

<h3 id="plugin-marketplace-update">
  plugin marketplace update
</h3>

Atualize um marketplace, ou cada marketplace, de sua fonte para buscar novos plugins e versões. Um marketplace adicionado com um branch ou tag `ref` atualiza para o commit mais recente desse ref, não o branch padrão do repositório.

```bash theme={null}
claude plugin marketplace update [name]
```

O comando não toma flags além de `--help`.

Atualize um marketplace:

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code imprime `Successfully updated marketplace: your-marketplace`. Quando você omite o nome, imprime uma contagem como `Successfully updated 2 marketplaces`. Sem marketplaces adicionados, imprime `No marketplaces configured` e sai com `0`.

<h2 id="plugin-in-a-session">
  /plugin em uma sessão
</h2>

Dentro de uma sessão interativa, `/plugin` abre o painel de plugin. Cada subcomando abre o painel em uma aba, executa uma ação lá ou imprime um resultado inline. `/plugins` e `/marketplace` são aliases para `/plugin`.

Você pode executar estes comandos apenas em uma sessão de terminal interativa. Em uma execução não interativa como `claude -p`, Claude Code responde que `/plugin` não está disponível neste ambiente.

Para quais superfícies têm `/plugin`, como instalar sem ele e o que cada aba do painel mostra, veja [Instalar e gerenciar plugins](/docs/pt/plugins/install).

Um `<plugin>` é um `name` de plugin ou `name@marketplace`.

A tabela abaixo lista cada forma de sessão. Os subcomandos de shell `init`, `update`, `details`, `prune`, `eval` e `eval init` não têm forma de sessão.

| Comando                                             | Aliases                                        | O que faz                                                                                                                                                                                                                                                                                                             |
| :-------------------------------------------------- | :--------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/plugin`                                           |                                                | Abre o painel na aba **Discover**. Qualquer primeira palavra não reconhecida após `/plugin` faz o mesmo                                                                                                                                                                                                               |
| `/plugin help`                                      | `/plugin --help`, `/plugin -h`                 | Mostra a lista de uso de subcomandos `/plugin`                                                                                                                                                                                                                                                                        |
| `/plugin list [--enabled\|--disabled]`              | `ls`                                           | Imprime seus plugins instalados de marketplace inline, com versão, escopo e status. Uma flag de filtro mostra apenas esse estado. Um plugin cujo estado de ativação ainda não foi aplicado é marcado `— run /reload-plugins to apply`. Requer Claude Code v2.1.163 ou posterior                                       |
| `/plugin install`                                   | `i`                                            | Abre a aba **Discover**                                                                                                                                                                                                                                                                                               |
| `/plugin install <plugin>`                          | `i`                                            | Abre os detalhes do plugin na aba **Discover**. Com `name@marketplace`, abre-os na lista desse marketplace                                                                                                                                                                                                            |
| `/plugin install <plugin> --marketplace <source>`   | `i`                                            | Adiciona o marketplace em `<source>` quando você ainda não o adicionou, pedindo para você confirmar primeiro, depois abre os detalhes do plugin. Veja [Adicionar um marketplace e instalar em um comando](/docs/pt/plugins/install#add-a-marketplace-and-install-in-one-command). Requer Claude Code v2.1.275 ou posterior |
| `/plugin manage`                                    |                                                | Abre a aba **Installed**                                                                                                                                                                                                                                                                                              |
| `/plugin stats`                                     |                                                | Abre a aba **Stats**, em sessões onde [`/skill-doctor`](/docs/pt/skills#find-unused-skills) está disponível. Em qualquer outro lugar abre o painel na aba **Discover**                                                                                                                                                     |
| `/plugin enable <plugin>`                           |                                                | Abre a aba **Installed** no plugin e o ativa                                                                                                                                                                                                                                                                          |
| `/plugin disable <plugin>`                          |                                                | Abre a aba **Installed** no plugin e o desativa                                                                                                                                                                                                                                                                       |
| `/plugin uninstall <plugin>`                        |                                                | Abre a aba **Installed** no plugin e o desinstala                                                                                                                                                                                                                                                                     |
| `/plugin configure <plugin>`                        | `config`                                       | Abre o diálogo [`userConfig`](/docs/pt/plugins/manifest-reference) do plugin, ou relata que o plugin não declara nenhum. Requer Claude Code v2.1.147 ou posterior                                                                                                                                                          |
| `/plugin validate <path>`                           |                                                | Imprime o mesmo relatório que `claude plugin validate`, inline                                                                                                                                                                                                                                                        |
| `/plugin tag [path] [--push] [--dry-run] [--force]` |                                                | Cria a tag de versão como `claude plugin tag` faz. Aceita `--push`, `--dry-run` e `--force` ou `-f`; com qualquer outra flag ou argumento extra, Claude Code imprime uso                                                                                                                                              |
| `/plugin marketplace`                               | `market`                                       | Não faz nada visível. Passe `add`, `list`, `update` ou `remove`                                                                                                                                                                                                                                                       |
| `/plugin marketplace add [source]`                  | `market add`                                   | Com uma fonte, a adiciona e relata o resultado. Sem uma, abre a entrada **Add marketplace**                                                                                                                                                                                                                           |
| `/plugin marketplace list`                          | `market list`                                  | Imprime seus nomes de marketplace inline                                                                                                                                                                                                                                                                              |
| `/plugin marketplace update [name]`                 | `market update`                                | Abre a aba **Marketplaces**. Com um nome, atualiza esse marketplace lá                                                                                                                                                                                                                                                |
| `/plugin marketplace remove [name]`                 | `market remove`, `market rm`, `marketplace rm` | Abre a aba **Marketplaces**. Com um nome, remove esse marketplace lá                                                                                                                                                                                                                                                  |

Se você nomear um plugin que não está instalado no projeto atual em `/plugin enable`, `disable`, `uninstall` ou `configure`, Claude Code imprime `Plugin "<plugin>" is not installed in this project` em vez de agir.

<h2 id="reload-plugins">
  /reload-plugins
</h2>

Aplique mudanças de plugin pendentes à sessão em execução sem reiniciá-la. Mudanças pendentes são plugins que você instalou, atualizou, ativou, desativou ou editou no disco desde que a sessão começou.

Quando você fecha o painel `/plugin` com mudanças pendentes que você fez nele, Claude Code executa `/reload-plugins` para você. Execute-o você mesmo após mudanças de plugin que acontecem fora do painel, como um comando `claude plugin` que você executou em outro terminal.

```text theme={null}
/reload-plugins [--force]
```

| Flag      | Descrição                                                                                               |
| :-------- | :------------------------------------------------------------------------------------------------------ |
| `--force` | Aplique o recarregamento mesmo quando invalidaria o cache de prompt. `force` sem dashes também funciona |

<h3 id="reload-summary">
  Resumo de recarregamento
</h3>

Claude Code recarrega cada plugin ativo e imprime uma linha de resumo, `Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers`, omitindo a contagem de servidor MCP de plugin em uma sessão sem terminal interativo. Quando qualquer plugin falhou, o resumo adiciona `N errors during load. Run /plugin for details.`

A contagem de skills cobre cada skill que um plugin fornece, tanto suas entradas `commands/` quanto suas skills `SKILL.md`. A contagem de agents é o número de agents carregados na sessão, incluindo aqueles que não vêm de plugins.

Quando as [dependências](/docs/pt/plugins/dependencies) de um plugin recarregado estão faltando, Claude Code as instala, recarrega novamente e anexa `(+ N dependencies: <names>) resolved` ao resumo.

<h3 id="reloads-that-change-mcp-tools">
  Recarregamentos que mudam ferramentas MCP
</h3>

Quando o recarregamento adicionaria ou removeria um servidor MCP de plugin ou a ferramenta `LSP`, e essa mudança invalidaria o [cache de prompt](/docs/pt/prompt-caching#enabling-or-disabling-a-plugin), Claude Code não aplica o recarregamento. Imprime uma linha como `This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.` Passe `--force` para aplicá-lo mesmo assim.

<h3 id="sessions-without-an-interactive-terminal">
  Sessões sem terminal interativo
</h3>

`/reload-plugins` também é executado em sessões sem terminal interativo, como o aplicativo desktop, o Agent SDK e [modo não interativo](/docs/pt/headless) com `-p`. Requer Claude Code v2.1.260 ou posterior.

Nessas sessões, o comando é executado apenas quando você o digita na sessão você mesmo, como no prompt `-p` ou na caixa de prompt do aplicativo desktop. Quando chega de outra forma, como através de [Remote Control](/docs/pt/remote-control) ou uma mensagem retransmitida do Slack, o comando responde `/reload-plugins isn't available over a remote connection in this session.` e não recarrega nada.

O recarregamento nessas sessões não conecta ou desconecta servidores MCP de plugin. Essas mudanças entram em vigor em sua próxima sessão.

<h2 id="flags-that-load-a-plugin-for-one-session">
  Flags que carregam um plugin para uma sessão
</h2>

Duas flags `claude` carregam um plugin para uma sessão apenas, sem instalá-lo. Ambas são repetíveis.

Autores de plugin as usam para testar um plugin antes de publicar. Para o fluxo de trabalho carregar-editar-recarregar, veja [Desenvolver sem um marketplace](/docs/pt/plugins/create#develop-without-a-marketplace).

| Flag                  | Descrição                                                                                                                                                                          | Exemplo                                                                     |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `--plugin-dir <path>` | Carregue um plugin de um diretório ou um arquivo `.zip` de um. Uma pasta de plugins carrega cada pasta filha que contém um `.claude-plugin/plugin.json`. Cada flag toma um caminho | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip`                  |
| `--plugin-url <url>`  | Busque um arquivo `.zip` de plugin de uma URL. Repita a flag, ou passe várias URLs separadas por espaço em um valor entre aspas                                                    | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

Um plugin que qualquer uma dessas flags carrega é um plugin de sessão única. `claude plugin list` o mostra como `<name>@inline` com escopo `session`, mas apenas quando a mesma flag precede o subcomando. Por exemplo, execute `claude --plugin-dir ./my-plugin plugin list`.

Quando um plugin de sessão única compartilha um nome com um plugin instalado, Claude Code carrega a cópia de sessão única para essa sessão e pula a instalada. A cópia instalada carrega em vez disso se você desativou a cópia de sessão única com `claude plugin disable <name>@inline`, ou se configurações gerenciadas bloqueiam esse nome de plugin. Para a precedência, veja [Referência de carregamento de plugin](/docs/pt/plugins/loading).

Um administrador pode rejeitar ambas as flags e pastas nomeadas na variável [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/pt/env-vars#variables), com a configuração gerenciada [`disableSideloadFlags`](/docs/pt/settings-reference#disablesideloadflags). Claude Code então imprime que a flag é desativada pelas configurações gerenciadas de sua organização e sai com `1` sem iniciar.

Do Agent SDK, a opção [`plugins`](/docs/pt/agent-sdk/plugins) é o equivalente de `--plugin-dir`.

<h2 id="next-steps">
  Próximos passos
</h2>

* [Instalar e gerenciar plugins](/docs/pt/plugins/install): as mesmas operações que etapas, com o que você vê em cada uma
* [Referência de carregamento de plugin](/docs/pt/plugins/loading): o que cada comando muda no disco e qual escopo entra em vigor
* [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting): mensagens de erro de instalação, marketplace, carregamento e validação com suas correções
* [Referência de manifesto de plugin](/docs/pt/plugins/manifest-reference): os campos que `claude plugin validate` verifica
