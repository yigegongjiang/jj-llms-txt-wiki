> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência do marketplace

> Referência completa dos campos marketplace.json, entradas de plugins e dos objetos de origem do plugin e marketplace, com onde cada um é válido.

`marketplace.json` é o arquivo que define um marketplace de plugins. Ele contém o nome do marketplace, seu proprietário e uma entrada por plugin. A origem do plugin de cada entrada diz onde Claude Code busca esse plugin.

Uma origem de marketplace é um objeto separado que diz onde Claude Code busca o próprio arquivo marketplace. Você escreve um nas configurações, ou Claude Code constrói um quando você executa `claude plugin marketplace add`.

Esta referência é para mantenedores de marketplace que precisam de um nome de campo ou valor exato, e para administradores que precisam saber quais valores de `source` são válidos em [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces), [`strictKnownMarketplaces`](/docs/pt/settings-reference#strictknownmarketplaces) e [`blockedMarketplaces`](/docs/pt/plugins/org#restrict-what-users-can-install).

<Note>
  Estes casos são cobertos em outras páginas:

  * **Construir ou hospedar um marketplace**: veja [Create a marketplace](/docs/pt/plugins/create-marketplace) e [Host and maintain a marketplace](/docs/pt/plugins/host-marketplace)
  * **Receitas de lista de permissões e bloqueio**: veja [Manage plugins for your organization](/docs/pt/plugins/org)
</Note>

Encontre a seção para o que você está escrevendo ou lendo:

* **O arquivo marketplace**: [Top-level fields](#top-level-fields) e [Plugin entries](#plugin-entries)
* **A `source` de uma entrada**: [Plugin sources](#plugin-sources)
* **Um objeto `source` nas configurações**: [Marketplace sources](#marketplace-sources)
* **Saída de [`claude plugin validate <path>`](/docs/pt/plugins/cli-reference)**: [Validation messages](#validation-messages), que mapeia cada mensagem para o campo que ela nomeia

<h2 id="marketplace-file">
  Arquivo marketplace
</h2>

Salve o arquivo marketplace em `.claude-plugin/marketplace.json` no diretório do seu marketplace. Se você manter o arquivo em outro lugar no repositório, os usuários precisam declarar o marketplace em [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces) com `path` definido em sua origem, porque `claude plugin marketplace add` não tem opção para isso.

O diretório que contém `.claude-plugin/` é chamado de raiz do marketplace, e toda origem de plugin relativa se resolve a partir dele, não a partir de `.claude-plugin/`.

Cada usuário registra um marketplace por `name`, então um usuário não pode ter dois marketplaces com o mesmo nome registrados ao mesmo tempo.

Claude Code ignora uma chave de nível superior desconhecida ou uma chave de entrada de plugin em vez de rejeitá-la, então um erro de digitação carrega silenciosamente. `claude plugin validate` relata cada chave desconhecida como um aviso.

<h3 id="reserved-names">
  Nomes reservados
</h3>

Você não pode dar ao seu marketplace nenhum dos seguintes nomes:

* **Nomes de marketplace oficial**: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins` e `claude-tag-plugins`. Reservado a menos que o marketplace venha de uma [origem de marketplace](#marketplace-sources) `github` ou `git` sob `github.com/anthropics/`.
* **Nomes de marketplace comunitário**: `claude-community`, `claude-plugins-community` e `healthcare`. Reservado sob a mesma regra que os nomes oficiais.
* **Nomes de diretório de plugins**: `anthropic-plugin-directory` e `claude-plugin-directory`. Reservado sob a mesma regra que os nomes oficiais.
* **Nomes que se passam por um marketplace oficial**: nomes como `official-claude-plugins` ou `claude-plugins-v2`, e qualquer nome contendo um caractere não-ASCII. O erro é `Marketplace name impersonates an official Anthropic/Claude marketplace`. Um caractere de controle ou formatação bidirecional em um nome também relata `Marketplace name cannot contain control or bidirectional-formatting characters`.
* <span id="reserved-name-spellings" />**Outra grafia de um nome reservado**: um nome que difere de um nome reservado apenas por um ponto final, ou por um símbolo diferente de um hífen no lugar de um hífen, então `claude.code.plugins` conta como `claude-code-plugins`. `claude plugin validate` aceita tal nome; adicionar o marketplace falha com [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/pt/errors#marketplace-name-is-another-spelling-of-a-reserved-name), e um marketplace já registrado sob um para de carregar. Esta verificação requer Claude Code v2.1.280 ou posterior.
* **Nomes que Claude Code usa para plugins que não vêm de um marketplace**: `inline` para plugins carregados com [`--plugin-dir`](/docs/pt/cli-reference), `builtin` para plugins integrados, `skills-dir` para plugins carregados automaticamente de [`.claude/skills/`](/docs/pt/skills) e `synced` para plugins sincronizados de sua conta claude.ai. `claude-plugin-test` também é reservado. `skills-dir` também aparece como `{"source": "skills-dir"}` em `strictKnownMarketplaces` e `blockedMarketplaces`, descrito em [Source values valid only in policy lists](#source-values-valid-only-in-policy-lists).
* **`npm`, `pip`, `uv`, `cargo`, `github` e `gh`**: reservado em qualquer capitalização. Esta verificação requer Claude Code v2.1.275 ou posterior.
* **Nomes começando com `claudeai-`**: reservado para marketplaces hospedados em claude.ai. `claude plugin marketplace add` recusa qualquer outro marketplace que use um com `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai`.

<h2 id="top-level-fields">
  Top-level fields
</h2>

A tabela lista cada chave que Claude Code lê de `marketplace.json`. `name`, `owner` e `plugins` são obrigatórios.

| Field                                      | Type             | Description                                                                                                                                                                                                                                                                                  |
| :----------------------------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                     | string           | Identificador do marketplace. Sem espaços, caracteres de controle ou caracteres de formatação bidirecional, sem `/` ou `\`, sem `..` e não `.`. Veja [Reserved names](#reserved-names). Os usuários digitam após `@` quando instalam um plugin                                               |
| `owner`                                    | object           | Informações do mantenedor. `name` é obrigatório; `email` e `url` são opcionais                                                                                                                                                                                                               |
| `plugins`                                  | array            | [Plugin entries](#plugin-entries). Cada entrada é validada por conta própria, então uma entrada inválida não falha o marketplace                                                                                                                                                             |
| `$schema`                                  | string           | URL do JSON Schema para preenchimento automático do editor. Ignorado no tempo de carregamento                                                                                                                                                                                                |
| `description`                              | string           | Descrição do marketplace mostrada aos usuários. `claude plugin validate` avisa quando está faltando                                                                                                                                                                                          |
| `version`                                  | string           | Versão do manifesto do marketplace                                                                                                                                                                                                                                                           |
| `metadata.description`, `metadata.version` | string           | Local alternativo para `description` e `version`                                                                                                                                                                                                                                             |
| `metadata.pluginRoot`                      | string           | Diretório que nomes de origem de plugin simples se resolvem sob. Veja [Relative path plugin source](#relative-path-plugin-source). Requer Claude Code v2.1.239 ou posterior                                                                                                                  |
| `forceRemoveDeletedPlugins`                | boolean          | Quando `true`, um plugin que você remove de `plugins` é desinstalado nas máquinas dos usuários. Veja [Host and maintain a marketplace](/docs/pt/plugins/host-marketplace)                                                                                                                         |
| `allowCrossMarketplaceDependenciesOn`      | array of strings | Nomes de marketplace cujos plugins podem ser instalados como dependências dos plugins deste marketplace. Quando você instala um plugin, apenas a lista no próprio marketplace do plugin se aplica, para toda sua cadeia de dependência. Veja [Plugin dependencies](/docs/pt/plugins/dependencies) |
| `renames`                                  | object           | Mapa de um `name` de plugin anterior para seu nome atual, ou para `null` para um plugin que você removeu. Requer Claude Code v2.1.193 ou posterior. Veja [Host and maintain a marketplace](/docs/pt/plugins/host-marketplace)                                                                     |

<h2 id="plugin-entries">
  Plugin entries
</h2>

Cada objeto no array `plugins` de nível superior de `marketplace.json` nomeia um plugin e diz onde buscá-lo. `name` e `source` são obrigatórios.

Uma entrada também aceita cada campo [`plugin.json`](/docs/pt/plugins/manifest-reference), como `description`, `version`, `author`, `commands` e `hooks`. Para quando esses campos se aplicam, veja [How an entry combines with plugin.json](#entry-and-plugin-json).

A tabela lista os campos próprios da entrada e os campos de manifesto cuja significação muda em uma entrada.

| Field            | Type             | Description                                                                                                                                                                                                                                                                                                                               |
| :--------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`           | string           | Identificador do plugin, sem espaços, caracteres de controle ou caracteres de formatação bidirecional. Os usuários digitam antes de `@` quando instalam, mesmo quando o próprio `plugin.json` do plugin define um `name` diferente                                                                                                        |
| `source`         | string or object | Onde buscar o plugin. Veja [Plugin sources](#plugin-sources)                                                                                                                                                                                                                                                                              |
| `description`    | string           | Mostrado em listagens e detalhes de [`/plugin`](/docs/pt/plugins/install)                                                                                                                                                                                                                                                                      |
| `version`        | string           | String de versão para o plugin. Quando `plugin.json` também define `version`, `plugin.json` tem precedência e `claude plugin validate` avisa. Veja [Plugin loading reference](/docs/pt/plugins/loading)                                                                                                                                        |
| `category`       | string           | Categoria de forma livre para organizar o catálogo                                                                                                                                                                                                                                                                                        |
| `tags`           | array of strings | Tags de forma livre para busca                                                                                                                                                                                                                                                                                                            |
| `strict`         | boolean          | Padrão `true`. Se `plugin.json` é a fonte definitiva para os componentes do plugin. Veja [Strict mode](#strict-mode)                                                                                                                                                                                                                      |
| `relevance`      | object           | Sinais que dizem a Claude Code quando sugerir o plugin. Veja [Recommend plugins for your org](/docs/pt/plugins/relevance)                                                                                                                                                                                                                      |
| `dependencies`   | array            | Plugins que devem estar habilitados para este funcionar. Cada item é `"name"`, `"name@marketplace"` ou um objeto. Veja [Plugin dependencies](/docs/pt/plugins/dependencies)                                                                                                                                                                    |
| `defaultEnabled` | boolean          | Padrão `true`. Se o plugin começa habilitado quando o usuário não o definiu em [`enabledPlugins`](/docs/pt/settings-reference#enabledplugins). O valor da entrada tem precedência sobre `plugin.json`                                                                                                                                          |
| `displayName`    | string           | Nome legível por humanos mostrado na UI. Quando nem a entrada nem o `plugin.json` do plugin define um, os usuários veem o `name` do plugin                                                                                                                                                                                                |
| `metadata`       | object           | Objeto de forma livre para seus próprios campos. Claude Code não o lê. Requer Claude Code v2.1.222 ou posterior                                                                                                                                                                                                                           |
| `headers`        | object           | Cabeçalhos HTTP que Claude Code envia quando baixa o [archive](#archive-plugin-source) desta entrada. Um cabeçalho definido aqui substitui um cabeçalho de mesmo nome da [`headers`](#fields-by-type) da origem do marketplace. Requer Claude Code v2.1.238 ou posterior                                                                  |
| `headersHelper`  | string           | Comando que imprime os cabeçalhos de download de arquivo desta entrada como um objeto JSON, para uma credencial que expira. A entrada também deve definir [`"strict": false`](#strict-mode). Requer Claude Code v2.1.238 ou posterior. Veja [Authenticate archive downloads](/docs/pt/plugins/host-marketplace#authenticate-archive-downloads) |

<h3 id="entry-and-plugin-json">
  How an entry combines with plugin.json
</h3>

Os campos da entrada se aplicam diferentemente a um plugin buscado que tem seu próprio `.claude-plugin/plugin.json` e a um que não tem:

* **Sem `plugin.json`**: a entrada é o manifesto independentemente de `strict`. Cada campo de manifesto na entrada se aplica, incluindo [`mcpServers`, `lspServers`, `userConfig` e `channels`](/docs/pt/plugins/manifest-reference).
* **`plugin.json` presente**: `plugin.json` é o manifesto. [Strict mode](#strict-mode) decide se os seis campos de componente da entrada, `commands`, `agents`, `skills`, `hooks`, `outputStyles` e `themes`, são combinados com ele ou rejeitados como um conflito. Entrada `mcpServers`, `lspServers`, `userConfig` e `channels` não se aplicam. Declare-os em `plugin.json`.

<h4 id="hooks-in-an-entry">
  Hooks in an entry
</h4>

Escreva `hooks` de entrada como um objeto inline que mapeia nomes de eventos de hook para arrays de matcher. Se você escrever um caminho de arquivo ou um array em vez disso, `claude plugin validate` o passa. Esses hooks nunca são executados, e Claude Code relata um erro `not yet supported in a marketplace entry` para o plugin. Coloque hooks baseados em arquivo no próprio [`hooks/hooks.json`](/docs/pt/plugins/components) do plugin ou `plugin.json`.

<h4 id="display-fields">
  Display fields
</h4>

Tanto a entrada quanto o próprio `plugin.json` do plugin podem definir os campos de exibição `displayName`, `description`, `author`, `homepage`, `repository`, `license` e `keywords`. Os usuários veem esses valores em listagens e detalhes de plugins, antes e depois da instalação:

* Para um campo que você define na entrada, os usuários veem o valor da entrada, mesmo quando `plugin.json` define um diferente.
* Para um campo que a entrada deixa indefinido, os usuários veem o valor de `plugin.json`.

Antes da instalação, Claude Code pode ler `plugin.json` apenas para entradas com uma [origem de caminho relativo](#relative-path-plugin-source), cujos arquivos de plugin estão dentro do próprio marketplace. Para uma entrada com qualquer outro tipo de origem, os usuários veem apenas os campos próprios da entrada até instalarem o plugin.

<h3 id="strict-mode">
  Strict mode
</h3>

`strict` decide o que acontece quando o plugin buscado tem seu próprio `plugin.json` e a entrada também declara qualquer um dos [campos de componente](#entry-and-plugin-json): `commands`, `agents`, `skills`, `hooks`, `outputStyles` ou `themes`. Com `strict: true`, o padrão, Claude Code anexa os campos de componente da entrada a `plugin.json`, exceto `hooks`, cujos matchers substituem os do manifesto por evento. Com `strict: false`, uma entrada que declara qualquer campo de componente é um conflito, e o plugin falha ao carregar. A tabela mostra cada combinação de `strict`, `plugin.json` e campos de componente da entrada.

| `strict`         | `plugin.json` | Entry component fields | Result                                                                                                                                                                                                                                     |
| :--------------- | :------------ | :--------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| any              | absent        | any                    | A entrada é o manifesto                                                                                                                                                                                                                    |
| `true`, o padrão | present       | any                    | `plugin.json` é a autoridade. Claude Code anexa os campos de componente da entrada a ele, exceto `hooks`, cujos matchers [substituem os do manifesto por evento](/docs/pt/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`          | present       | none                   | `plugin.json` é o manifesto, como com `true`                                                                                                                                                                                               |
| `false`          | present       | one or more            | Conflito. O plugin falha ao carregar com `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components`                                                                                              |

<h2 id="plugin-sources">
  Plugin sources
</h2>

A `source` de uma entrada de plugin diz onde Claude Code busca esse plugin. É uma string de caminho relativo ou um objeto cuja própria chave `source` nomeia o tipo, então uma entrada se parece com `"source": { "source": "github", "repo": "your-org/formatter" }`.

A tabela lista cada tipo de origem de plugin e seus campos.

| Type          | Fields                           | Notes                                                                                                                                                                                                                                            |
| :------------ | :------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Relative path | a própria string                 | Um diretório dentro do marketplace, resolvido a partir da raiz do marketplace. Deve começar com `./`, a menos que você escreva um [nome simples sob `metadata.pluginRoot`](#relative-path-plugin-source). `"."` por si só significa a raiz em si |
| `github`      | `repo`, `ref`, `sha`             | Repositório GitHub na forma `owner/repo`                                                                                                                                                                                                         |
| `url`         | `url`, `ref`, `sha`              | Qualquer repositório git por URL                                                                                                                                                                                                                 |
| `git-subdir`  | `url`, `path`, `ref`, `sha`      | Um subdiretório de um repositório git, buscado com um clone parcial esparso                                                                                                                                                                      |
| `npm`         | `package`, `version`, `registry` | Pacote npm, buscado com seu cliente npm e desempacotado sem executar scripts de instalação                                                                                                                                                       |
| `archive`     | `url`, `sha256`                  | Arquivo Zip sobre HTTPS. Requer Claude Code v2.1.224 ou posterior                                                                                                                                                                                |
| `command`     | `command`, `timeout`, `mode`     | Diretório impresso por um comando que Claude Code executa na máquina do usuário. Requer Claude Code v2.1.229 ou posterior                                                                                                                        |

Os nomes `url` e `github` também são tipos de [origem de marketplace](#marketplace-sources), onde `url` significa um link direto a um arquivo `marketplace.json` em vez de um repositório git. `git` existe apenas como uma origem de marketplace, e `npm` existe como ambos. `git-subdir`, `archive` e `command` existem apenas como origens de plugin.

Use um caminho relativo para um plugin em um subdiretório do próprio repositório do marketplace. Use `git-subdir` para um subdiretório de algum outro repositório.

As origens `github`, `url` e `git-subdir` compartilham os campos `ref` e `sha`:

* **`ref`**: uma branch ou tag. Padrão para a branch padrão do repositório.
* **`sha`**: um SHA de commit completo de 40 caracteres em minúsculas. Quando você define tanto `ref` quanto `sha`, Claude Code faz checkout de `sha`. Na maioria dos hosts git, incluindo GitHub, GitLab e Bitbucket, isso significa que a instalação é bem-sucedida mesmo se a branch ou tag nomeada por `ref` foi deletada upstream, desde que o commit ainda seja alcançável do repositório. Alguns servidores, como AWS CodeCommit, não suportam buscar commits por SHA. Nesses servidores, o `ref` ainda deve existir e o commit fixado deve ser alcançável a partir dele.

Para como cada tipo é buscado, armazenado em cache e versionado, veja [Plugin loading reference](/docs/pt/plugins/loading).

<h3 id="relative-path-plugin-source">
  Relative path plugin source
</h3>

O caminho se resolve a partir da raiz do marketplace. `./plugins/formatter` é `<root>/plugins/formatter` mesmo que o arquivo marketplace esteja em `<root>/.claude-plugin/`.

Um caminho contendo `..` falha na validação. Em macOS e Linux, Claude Code recusa um caminho de entrada que contém uma barra invertida em qualquer lugar após o `./` inicial, então escreva o caminho com barras para frente.

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

Um caminho relativo se resolve apenas quando Claude Code tem os arquivos do marketplace, então verifique o tipo de [origem de marketplace](#marketplace-sources):

* **`github`, `git`, `file` e `directory`**: Claude Code tem os arquivos do marketplace.
* **`url`**: Claude Code busca apenas `marketplace.json`, então caminhos relativos não podem se resolver. Dê a cada plugin uma origem de objeto em vez disso, como `github` ou `git-subdir`.
* **`settings`**: caminhos relativos são rejeitados imediatamente.

<h4 id="bare-names-under-pluginroot">
  Bare names under pluginRoot
</h4>

Um nome simples é um único nome de diretório sem `/`, como `"formatter"`. Para escrever nomes simples em vez de caminhos `./`, defina [`metadata.pluginRoot`](#top-level-fields) para o diretório que eles se resolvem sob. Com `"pluginRoot": "./plugins"`, `"source": "formatter"` se resolve para `./plugins/formatter`. Requer Claude Code v2.1.239 ou posterior.

`metadata.pluginRoot` tem estes limites:

* Ele próprio deve ser um caminho relativo dentro do marketplace.
* Não tem efeito em uma origem que já começa com `./`.
* Uma origem que contém um `/`, como `team-a/formatter`, não é um nome simples e ainda precisa do prefixo `./`, mesmo quando `metadata.pluginRoot` está definido.

<h3 id="github-plugin-source">
  github plugin source
</h3>

`repo` leva `owner/repo`. `ref` e `sha` são opcionais.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  url plugin source
</h3>

`url` é uma URL git completa: `https://`, `http://`, `file://` ou `git@`. Um sufixo `.git` não é obrigatório, então URLs do Azure DevOps e AWS CodeCommit funcionam como escritas. Este tipo não leva o atalho `owner/repo`.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  git-subdir plugin source
</h3>

`url` aceita uma URL git completa ou atalho GitHub `owner/repo`. `path` é o subdiretório que contém o plugin, e Claude Code baixa apenas esse subdiretório.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  npm plugin source
</h3>

Uma origem `npm` leva estes campos:

* `package`: um nome de pacote, ou um nome com escopo como `@your-org/formatter`
* `version`: uma versão ou intervalo
* `registry`: uma URL de registro para um pacote que não está no registro padrão

Claude Code busca o pacote com seu cliente npm. Os scripts de instalação do pacote, como `preinstall` ou `postinstall`, nunca são executados, e suas dependências não são instaladas durante a busca. Se o pacote tiver um lockfile suportado ao lado de seu `package.json`, Claude Code instala essas [dependências de pacote Node.js](/docs/pt/plugins/loading#node-js-package-dependencies) em uma etapa separada, também com scripts desabilitados.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  archive plugin source
</h3>

`url` deve usar `https://` e não pode apontar para um host loopback, link-local ou cloud-metadata.

A raiz do plugin pode estar no topo do zip ou um diretório abaixo.

`sha256` é o resumo do arquivo como 64 caracteres hexadecimais, maiúsculos ou minúsculos. Quando você o define, Claude Code recusa um download que não corresponde.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  command plugin source
</h3>

Use uma origem `command` quando uma ferramenta instalada na máquina do usuário produz o diretório do plugin, como um IDE que renderiza seu plugin para a cadeia de ferramentas que o usuário selecionou. Claude Code executa o comando quando o usuário instala ou atualiza o plugin, e [novamente uma vez por sessão](/docs/pt/plugins/loading#when-a-command-source-re-runs), então os usuários obtêm a saída alterada da ferramenta sem reinstalar.

Uma origem `command` leva estes campos:

* `command`: um comando shell que imprime o caminho absoluto do diretório do plugin como uma linha e sai com 0. Claude Code mostra aos usuários a string inteira para revisão antes de executá-la. Escreva-a como ASCII imprimível, no máximo 500 caracteres, sem uma sequência de quatro ou mais espaços.
* `timeout`: um número inteiro de segundos de 1 a 600. Padrão para 60.
* `mode`: `copy`, o padrão, ou `link`. Veja [Copy mode and link mode](#copy-mode-and-link-mode).

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

Para como os usuários aceitam o comando, veja [Install from your shell](/docs/pt/plugins/install#install-from-your-shell). Para o que os usuários veem depois que você o alteram, veja [Change the command of a command source](/docs/pt/plugins/host-marketplace#change-the-command-of-a-command-source). Administradores desligam origens de comando com [`disableCommandPluginSources`](/docs/pt/settings-reference#disablecommandpluginsources).

<h4 id="what-the-command-must-do">
  What the command must do
</h4>

Escreva o comando para atender a estes requisitos:

* **Shell e diretório de trabalho**: Claude Code executa o comando através de `sh`, ou através de `cmd.exe` no Windows, a partir do diretório home do usuário. Dê um caminho absoluto ou um comando em `PATH`.
* **Saída**: imprima exatamente uma linha em stdout, o caminho absoluto do diretório do plugin, e saia com 0 dentro de `timeout` segundos.
* **Conteúdo do diretório**: o diretório contém o plugin completo no momento em que o comando sai. O caminho pode diferir de uma execução para a próxima.

<h4 id="output-that-fails-the-install-or-update">
  Output that fails the install or update
</h4>

A instalação ou atualização falha quando o comando sai com não-zero, executa mais tempo que `timeout`, ou imprime qualquer coisa diferente de um caminho absoluto. Também falha quando o diretório impresso é um destes:

* **Sem conteúdo de plugin**: o diretório impresso não tem conteúdo de plugin em seu nível superior, como um diretório `.claude-plugin/` ou um diretório `skills/`, `commands/`, `agents/` ou `hooks/`.
* **O diretório da própria sessão**: o diretório impresso é aquele em que Claude Code foi iniciado, ou um de seus pais.
* **Um caminho de rede**: no Windows, o caminho impresso é um caminho UNC.
* **Muito grande para copiar**: em modo copy, o diretório é maior que 256 MiB ou tem mais de 20.000 entradas.

<h4 id="copy-mode-and-link-mode">
  Copy mode and link mode
</h4>

`mode` decide se Claude Code copia o diretório impresso ou o usa no lugar:

* **`copy`**: Claude Code copia o diretório para o cache de plugins e deriva a [versão do plugin](/docs/pt/plugins/loading#how-claude-code-computes-the-version) de um hash dos arquivos copiados. Sua ferramenta pode deletar ou reescrever o diretório após o comando sair. Uma re-execução que produz arquivos idênticos conta como atualizado.
* **`link`**: Claude Code preenche a entrada de cache do plugin com um link para cada entrada de nível superior do diretório impresso e carrega os arquivos no lugar. Nada é copiado, conteúdos de arquivo não são hash, e os limites de tamanho não se aplicam. Use-o para um diretório muito grande para copiar, como uma exportação de SDK renderizada.

Um plugin em modo link tem estes requisitos:

* **Mantenha o diretório no lugar**: Claude Code carrega o plugin através dos links a cada inicialização, então o diretório impresso deve ficar onde está enquanto o plugin permanecer instalado.
* **Imprima um caminho diferente para sinalizar novo conteúdo**: a versão vem do caminho real do diretório impresso e suas entradas de nível superior, não dos arquivos dentro deles.
* **Mantenha symlinks de nível superior dentro do diretório**: a instalação falha se uma entrada de nível superior é um symlink que aponta para fora do diretório impresso.
* **Inclua `node_modules`**: Claude Code pula a [instalação de dependência de pacote Node.js](/docs/pt/plugins/loading#node-js-package-dependencies) para um plugin em modo link, então imprima um diretório que já contém os pacotes que o plugin precisa.
* **Sessões iniciadas dentro do diretório**: uma sessão iniciada no diretório impresso ou em qualquer lugar abaixo dele não carrega o plugin.
* **Não no Windows**: Claude Code recusa instalar um plugin em modo link no Windows. Declare `"mode": "copy"` lá.

<h2 id="marketplace-sources">
  Fontes do marketplace
</h2>

Uma fonte do marketplace diz onde Claude Code busca um `marketplace.json`. A CLI constrói uma para você quando você adiciona um marketplace, e você escreve uma você mesmo nas configurações:

* **[`claude plugin marketplace add`](/docs/pt/plugins/cli-reference)**: Claude Code constrói a fonte a partir da string que você passa.
* **[`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces)**: você escreve a fonte você mesmo como o objeto `source`.
* **[`strictKnownMarketplaces`](/docs/pt/settings-reference#strictknownmarketplaces) e [`blockedMarketplaces`](/docs/pt/plugins/org#restrict-what-users-can-install)**: administradores escrevem fontes nessas duas listas de política. `strictKnownMarketplaces` é a lista de permissão e `blockedMarketplaces` é a lista de bloqueio.

Os nomes de tipo `url`, `git` e `github` significam algo diferente em uma fonte do marketplace do que em uma [fonte de plugin](#plugin-sources):

| Nome do tipo | Como uma fonte do marketplace                                                                    | Como uma fonte de plugin                                              |
| :----------- | :----------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- |
| `url`        | Um link direto para um arquivo `marketplace.json`, com campos `url`, `headers` e `headersHelper` | Um repositório git para clonar, com campos `url`, `ref` e `sha`       |
| `git`        | Um repositório git para clonar, com campos `url`, `ref`, `path` e `sparsePaths`                  | Não existe                                                            |
| `github`     | Um repositório GitHub, com campos `repo`, `ref`, `path` e `sparsePaths`                          | Um repositório GitHub, com campos `repo`, `ref` e `sha`, e sem `path` |

A tabela lista cada tipo de fonte do marketplace com seus campos, a entrada `claude plugin marketplace add` que a produz, e o que ela faz em cada uma das três chaves de configurações.

| Tipo          | Campos                               | entrada `marketplace add`                                                                                                                                      | `extraKnownMarketplaces`                                         | `strictKnownMarketplaces`                                                                                                                                                                                                                                        | `blockedMarketplaces`                                              |
| :------------ | :----------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- |
| `url`         | `url`, `headers`, `headersHelper`    | Uma URL `http://` ou `https://` que não corresponde a um formulário git                                                                                        | Carrega                                                          | Permite a mesma URL                                                                                                                                                                                                                                              | Bloqueia a mesma URL                                               |
| `github`      | `repo`, `ref`, `path`, `sparsePaths` | `owner/repo`, `owner/repo@ref` ou `owner/repo#ref`                                                                                                             | Carrega                                                          | Permite o mesmo `repo`, `ref` e `path`. `repo` pode ser `owner/*`                                                                                                                                                                                                | Bloqueia o mesmo, e uma URL `git` para o mesmo repositório         |
| `git`         | `url`, `ref`, `path`, `sparsePaths`  | Uma URL `user@host:path`, ou uma URL `https://` que termina em `.git`, contém `/_git/`, ou nomeia um repositório github.com ou gitlab.com. `#ref` fixa uma ref | Carrega                                                          | Permite a mesma URL, `ref` e `path`                                                                                                                                                                                                                              | Bloqueia o mesmo, e outras grafias do mesmo repositório github.com |
| `npm`         | `package`                            | Não produzido                                                                                                                                                  | Falha ao carregar: `NPM marketplace sources not yet implemented` | Analisa mas não corresponde a nada, porque nada registra um marketplace `npm`                                                                                                                                                                                    | Analisa mas não corresponde a nada                                 |
| `file`        | `path`                               | Um caminho para um arquivo `.json`                                                                                                                             | Carrega                                                          | Permite o mesmo caminho                                                                                                                                                                                                                                          | Bloqueia o mesmo caminho                                           |
| `directory`   | `path`                               | Um caminho para um diretório                                                                                                                                   | Carrega                                                          | Permite o mesmo caminho                                                                                                                                                                                                                                          | Bloqueia o mesmo caminho                                           |
| `settings`    | `name`, `plugins`, `owner`           | Não produzido                                                                                                                                                  | Carrega                                                          | Permite uma entrada com o mesmo `name` e `plugins` idênticos                                                                                                                                                                                                     | Bloqueia o mesmo `name`                                            |
| `skills-dir`  | nenhum                               | Não produzido                                                                                                                                                  | Falha ao carregar: `Unsupported marketplace source type`         | Mantém [plugins de diretório de skills](/docs/pt/plugins/org#keep-skills-directory-plugins-loading) carregando enquanto uma lista de permissão está definida. Veja [Valores de fonte válidos apenas em listas de política](#source-values-valid-only-in-policy-lists) | Para plugins de diretório de skills de carregar                    |
| `hostPattern` | `hostPattern`                        | Não produzido                                                                                                                                                  | Falha ao carregar: `Unsupported marketplace source type`         | Permite fontes `github`, `git` e `url` cujo host corresponde                                                                                                                                                                                                     | Bloqueia essas fontes                                              |
| `pathPattern` | `pathPattern`                        | Não produzido                                                                                                                                                  | Falha ao carregar: `Unsupported marketplace source type`         | Permite fontes `file` e `directory` cujo `path` corresponde                                                                                                                                                                                                      | Bloqueia essas fontes                                              |

<h3 id="fields-by-type">
  Campos por tipo
</h3>

A tabela lista cada campo de fonte do marketplace que tem um padrão, uma restrição ou um significado específico para seu tipo.

| Campo           | Tipos           | Descrição                                                                                                                                                                                                                                                               |
| :-------------- | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`           | `url`           | Link para o arquivo `marketplace.json`. Claude Code baixa apenas esse arquivo, então os plugins do marketplace não podem usar [fontes de caminho relativo](#relative-path-plugin-source)                                                                                |
| `url`           | `git`           | O repositório git para clonar                                                                                                                                                                                                                                           |
| `headers`       | `url`           | Mapa de cabeçalhos HTTP que Claude Code envia com a busca, para hosts autenticados                                                                                                                                                                                      |
| `headersHelper` | `url`           | Comando que imprime cabeçalhos cujos valores são muito efêmeros para listar em `headers`. Requer Claude Code v2.1.238 ou posterior. Veja [Autenticar downloads de arquivo](/docs/pt/plugins/host-marketplace#authenticate-archive-downloads)                                 |
| `repo`          | `github`        | Em `marketplace add` e `extraKnownMarketplaces`, `repo` deve nomear um repositório. `marketplace add` rejeita `owner/*` como não sendo um atalho `owner/repo` válido; em `extraKnownMarketplaces` Claude Code o toma literalmente e o clone falha                       |
| `ref`           | `github`, `git` | Branch ou tag. Padrão é o branch padrão do repositório                                                                                                                                                                                                                  |
| `path`          | `github`, `git` | O caminho do arquivo do marketplace dentro do repositório. Padrão é `.claude-plugin/marketplace.json`                                                                                                                                                                   |
| `path`          | `file`          | O arquivo do marketplace em si. Claude Code o lê no local e toma o diretório dois níveis acima como a raiz do marketplace, então mantenha o arquivo em `<root>/.claude-plugin/marketplace.json`                                                                         |
| `path`          | `directory`     | A raiz do marketplace, o diretório que contém `.claude-plugin/marketplace.json`                                                                                                                                                                                         |
| `sparsePaths`   | `github`, `git` | Array de diretórios para um checkout esparso, como `[".claude-plugin", "plugins"]`. `claude plugin marketplace add --sparse` o define                                                                                                                                   |
| `skipLfs`       | `github`, `git` | Aceito e não tem efeito. Veja [Manter arquivos de plugin fora do Git LFS](/docs/pt/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs)                                                                                                                                |
| `name`          | `settings`      | Deve ser igual à chave `extraKnownMarketplaces` e não pode ser um [nome reservado](#reserved-names)                                                                                                                                                                     |
| `plugins`       | `settings`      | O catálogo inline, sem arquivo hospedado. Cada item leva `name`, `source`, `description`, `version`, `strict`, `headers` e `headersHelper`. Escreva o `source` de cada item como um tipo de objeto, porque um caminho relativo não tem repositório para resolver contra |

<h3 id="source-values-valid-only-in-policy-lists">
  Valores de fonte válidos apenas em listas de política
</h3>

`hostPattern`, `pathPattern`, `skills-dir` e a forma `owner/*` de `repo` são válidos apenas nas duas listas de política, `strictKnownMarketplaces` e `blockedMarketplaces`:

* **`hostPattern` e `pathPattern`**: expressões regulares que Claude Code testa contra uma fonte antes de buscar dela.
* **`skills-dir`**: não é uma fonte. Se você definir `strictKnownMarketplaces` de qualquer forma, [plugins de diretório de skills](/docs/pt/plugins/org#keep-skills-directory-plugins-loading) param de carregar até que você adicione `{"source": "skills-dir"}` a essa lista.
* **`owner/*`**: como um valor `repo` de `github`, corresponde a cada repositório sob exatamente esse proprietário do GitHub. Requer Claude Code v2.1.223 ou posterior.

Para ordem de correspondência, semântica exata de `ref` e receitas, veja [Gerenciar plugins para sua organização](/docs/pt/plugins/org).

<h3 id="source-objects-in-settings">
  Objetos de fonte nas configurações
</h3>

Um valor `extraKnownMarketplaces` é um mapa do nome do marketplace para um objeto com `source`. Esta entrada registra um marketplace de um repositório git em seu branch `main`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` e `blockedMarketplaces` são arrays de objetos de fonte. Esta lista de permissão admite um proprietário do GitHub e um host interno:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  Validation messages
</h2>

`claude plugin validate <path>` leva a raiz do marketplace ou o próprio arquivo marketplace. Ele imprime erros e avisos. Para códigos de saída e `--strict`, veja [plugin validate](/docs/pt/plugins/cli-reference#plugin-validate).

Uma mensagem nomeia uma entrada de plugin por seu índice, escrito como `plugins.1.source` ou `plugins[1].source`.

Uma mensagem prefixada com um índice de entrada e `plugin.json →`, como `plugins[2] plugin.json →`, é sobre os próprios arquivos desse plugin. [`claude plugin validate` relata erros](/docs/pt/plugins/troubleshooting#claude-plugin-validate-reports-errors) lista essas mensagens com suas correções.

Avisos que mencionam nomes de sinalizadores Claude Desktop indicam nomes que Claude Code aceita mas Claude Desktop rejeita, porque as regras de nome do Claude Desktop são mais rigorosas.

A tabela mapeia mensagens de nível de marketplace para o campo que cada uma é sobre.

| Message                                                                                                                                                                                        | Level   | Field                                                                                                                        |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------ | :--------------------------------------------------------------------------------------------------------------------------- |
| `Marketplace must have a name`                                                                                                                                                                 | Error   | `name` está vazio                                                                                                            |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                              | Error   | `name`                                                                                                                       |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                          | Error   | `name`                                                                                                                       |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                       | Error   | `name`. Veja [Reserved names](#reserved-names)                                                                               |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                               | Error   | `name` contém um caractere de controle, como um escape ou uma nova linha, ou um caractere de formatação bidirecional Unicode |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, e as variantes `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github` e `gh` | Error   | `name`                                                                                                                       |
| `Author name cannot be empty`                                                                                                                                                                  | Error   | `owner.name`                                                                                                                 |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                        | Error   | `plugins[i].name`                                                                                                            |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                    | Error   | `plugins[i].name`                                                                                                            |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                               | Error   | Duas entradas compartilham um `name`                                                                                         |
| `plugins.i.source: Invalid input`                                                                                                                                                              | Error   | A `source` da entrada não corresponde a nenhum tipo. Veja [Invalid input on a source](#invalid-input-on-a-source)            |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                                | Error   | Uma `source` relativa que escapa da raiz do marketplace                                                                      |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                              | Error   | `plugins[i].source`                                                                                                          |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                     | Error   | `plugins[i].headersHelper`, em uma entrada `archive`                                                                         |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                            | Error   | `renames.<old>`                                                                                                              |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                       | Error   | `renames.<old>`                                                                                                              |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                      | Warning | A chave nomeada no nível superior, sob `metadata`, em uma entrada, ou sob a `relevance` de uma entrada                       |
| `Marketplace has no plugins defined`                                                                                                                                                           | Warning | `plugins` está vazio                                                                                                         |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                             | Warning | `plugins[i].headers` ou `plugins[i].headersHelper`, em uma entrada cuja `source` não é `archive`                             |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                                   | Warning | `plugins[i].source.sha256`                                                                                                   |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                     | Warning | `plugins[i].headers.<name>`                                                                                                  |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                           | Warning | `plugins[i].source`                                                                                                          |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                                | Warning | `description`                                                                                                                |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                                | Warning | `plugins[i].version`, em uma entrada de caminho relativo                                                                     |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                     | Warning | `plugins[i].relevance`                                                                                                       |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                          | Warning | `plugins[i].metadata`                                                                                                        |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                             | Warning | `plugins[i].experimental`                                                                                                    |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                           | Warning | `name` é `org`, `org-provisioned` ou `unknown`. Claude Desktop rejeita o marketplace                                         |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                              | Warning | `name`. Claude Desktop rejeita o marketplace                                                                                 |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                   | Warning | `plugins[i].name`. Claude Desktop descarta a entrada                                                                         |

<h3 id="invalid-input-on-a-source">
  Invalid input on a source
</h3>

`Invalid input` em uma `source` significa que o objeto não correspondeu a nenhum tipo de origem. Verifique estas causas:

* Um caminho relativo que não começa com `./`, diferente de `"."` ou um [nome simples sob `metadata.pluginRoot`](#relative-path-plugin-source)
* Um `package` `npm` contendo `..`
* Um tipo de `source` que não é um das [origens de plugin](#plugin-sources)
* Um tipo conhecido com um campo obrigatório faltando ou do tipo errado, como `github` sem `repo`

<h3 id="failures-that-validation-doesn’t-catch">
  Failures that validation doesn't catch
</h3>

`claude plugin validate` não relata cada falha. Uma entrada `hooks` escrita como um caminho de arquivo ou array passa na validação, e o erro aparece apenas quando o plugin carrega, como [Hooks in an entry](#hooks-in-an-entry) descreve. Erros buscando uma `source` também aparecem apenas após a instalação, não na validação.

[`claude plugin list`](/docs/pt/plugins/cli-reference) mostra um plugin que falhou ao carregar com seu erro, e [Troubleshoot plugins](/docs/pt/plugins/troubleshooting) cobre as strings de tempo de carregamento.

<h2 id="next-steps">
  Next steps
</h2>

* [Create a marketplace](/docs/pt/plugins/create-marketplace): construa um marketplace a partir desses campos e instale dele localmente
* [Host and maintain a marketplace](/docs/pt/plugins/host-marketplace): onde colocar o arquivo e como os usuários recebem mudanças
* [Plugin manifest reference](/docs/pt/plugins/manifest-reference): os campos `plugin.json` que uma entrada pode sobrescrever
* [Manage plugins for your organization](/docs/pt/plugins/org): receitas de lista de permissões e bloqueio que usam esses valores de origem
