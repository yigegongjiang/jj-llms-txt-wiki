> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência de manifesto de plugin

> Referência completa para plugin.json: cada campo com seu tipo e padrão, formas de caminho aceitas e os esquemas de userConfig e variáveis de ambiente.

Um manifesto de plugin é o arquivo `plugin.json` no diretório `.claude-plugin/` de um plugin. Ele contém os metadados do plugin e os valores de [`userConfig`](#user-configuration) que Claude Code solicita ao usuário. Também declara qualquer componente que você define inline ou mantém fora de seu [local padrão](#standard-layout).

Esta referência é para criadores de plugins e para proprietários de marketplace que colocam campos de componentes em uma entrada de marketplace.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Aprender a construir um plugin**: comece com [Criar um plugin](/docs/pt/plugins/create)
  * **O que cada componente faz em tempo de execução**: veja [Componentes de plugin](/docs/pt/plugins/components)
</Note>

Comece na seção que corresponde ao que você está procurando:

* Um campo: a [tabela Campos](#fields) fornece o tipo de cada campo, se é obrigatório, seu padrão e o que aceita. [Regras de caminho](#path-rules) cobre o prefixo `./` e contenção para cada caminho de componente
* Uma opção `userConfig` ou uma entrada `channels`: os esquemas [Configuração do usuário](#user-configuration) e [Canais](#channels)
* `${CLAUDE_PLUGIN_ROOT}` ou outra variável que um plugin pode referenciar: [Variáveis de ambiente](#environment-variables)
* Onde os arquivos de cada componente vão: [Layout padrão](#standard-layout)
* Uma mensagem de `claude plugin validate`: a [página de solução de problemas](/docs/pt/plugins/troubleshooting) lista cada mensagem com sua correção e links para as seções relevantes nesta página

<h2 id="manifest-file">
  Arquivo de manifesto
</h2>

O manifesto é opcional. Sem ele, Claude Code carrega os componentes que encontra no [layout padrão](#standard-layout). O nome do plugin vem da entrada do marketplace ou do nome do diretório quando você carrega o plugin com `--plugin-dir`.

Escreva um manifesto quando quiser metadados, um componente fora de seu diretório padrão, `userConfig` ou uma definição de componente inline.

Salve o manifesto em `.claude-plugin/plugin.json` sob a raiz do plugin. Coloque todos os outros arquivos do plugin na raiz do plugin, não dentro de `.claude-plugin/`. Isso inclui `skills/`, `commands/` e `hooks/`.

O exemplo a seguir define a maioria das chaves na [tabela Campos](#fields). Ele passa na validação em um diretório de plugin que contém cada caminho referenciado.

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

<h3 id="unrecognized-fields">
  Campos não reconhecidos
</h3>

Uma chave de nível superior não reconhecida é removida, e uma chave não reconhecida dentro de uma opção `userConfig`, entrada `channels`, configuração `lspServers` ou entrada `monitors` é rejeitada:

* **Campos de nível superior**: o campo é removido e o plugin carrega. `claude plugin validate` relata cada campo de nível superior não reconhecido como um aviso
* **Objetos estritos**: opções `userConfig`, entradas `channels`, configurações `lspServers` e entradas `monitors` são estritas. Uma chave desconhecida dentro de uma é um erro, e o plugin não carrega

<h3 id="validate-the-manifest">
  Validar o manifesto
</h3>

`claude plugin validate` é a verificação autoritária para um manifesto. Execute-o do seu shell contra o diretório do plugin:

```bash theme={null}
claude plugin validate ./my-plugin
```

O comando relata um destes resultados:

* **`Validation passed`**: o manifesto carrega
* **`Validation passed with warnings`**: o manifesto carrega, mas o validador encontrou algo para corrigir, como um campo de nível superior desconhecido que Claude Code remove, um `name` que não está em kebab-case, ou um `version`, `description` ou `author` ausente. Passe `--strict` para transformar avisos em falhas em CI
* **`Validation failed`**: o manifesto tem uma incompatibilidade de tipo, um caminho que está faltando ou escapa da raiz do plugin, ou uma chave desconhecida dentro de uma opção `userConfig`, entrada `channels`, configuração `lspServers` ou entrada `monitors`. Claude Code relata o mesmo problema quando carrega o plugin

<h2 id="fields">
  Campos
</h2>

A tabela lista as chaves de nível superior em `plugin.json`. `name` é a única chave obrigatória. Quando um nome de campo é um link, a seção vinculada tem suas regras completas.

Para chaves de componentes como `commands` e `hooks`, [Formas de caminho de componente](#component-path-forms) mostra cada forma aceita com um exemplo, e cada caminho segue as [regras de caminho](#path-rules) para o prefixo `./`, extensões e contenção.

| Campo                                | Tipo                                     | Descrição                                                                                                                                                                                                                                                                                                                          |
| :----------------------------------- | :--------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                            | String                                   | URL do JSON Schema para autocompletar do editor. Claude Code a ignora no tempo de carregamento                                                                                                                                                                                                                                     |
| [`name`](#name)                      | String                                   | Identificador do plugin, obrigatório. Use kebab-case. Cada componente é namespaced sob ele                                                                                                                                                                                                                                         |
| [`displayName`](#displayname)        | String                                   | Nome mostrado na UI no lugar de `name`                                                                                                                                                                                                                                                                                             |
| [`version`](#version)                | String                                   | String de versão. Configurá-la mantém os usuários nessa versão até você alterá-la                                                                                                                                                                                                                                                  |
| `description`                        | String                                   | Explicação breve do que o plugin fornece                                                                                                                                                                                                                                                                                           |
| `author`                             | Object                                   | `name`, que é obrigatório, mais `email` e `url` opcionais                                                                                                                                                                                                                                                                          |
| `homepage`                           | String                                   | URL de documentação. Deve ser analisada como uma URL, ou o plugin falha ao carregar                                                                                                                                                                                                                                                |
| `repository`                         | String                                   | URL do repositório de origem. Não validada                                                                                                                                                                                                                                                                                         |
| `license`                            | String                                   | Identificador SPDX como `MIT` ou `Apache-2.0`                                                                                                                                                                                                                                                                                      |
| `keywords`                           | Array de strings                         | Tags de descoberta                                                                                                                                                                                                                                                                                                                 |
| [`metadata`](#metadata)              | Object                                   | Objeto de forma livre para seus próprios dados. Claude Code não o lê                                                                                                                                                                                                                                                               |
| [`defaultEnabled`](#defaultenabled)  | Boolean                                  | Se o plugin inicia habilitado quando o usuário não o configurou. Padrão é `true`                                                                                                                                                                                                                                                   |
| [`dependencies`](#dependencies)      | Array de strings ou objetos              | Plugins que devem estar habilitados para este funcionar                                                                                                                                                                                                                                                                            |
| [`settings`](#settings)              | Object                                   | Configurações que Claude Code aplica enquanto o plugin está habilitado. Apenas `agent` e `subagentStatusLine` têm efeito                                                                                                                                                                                                           |
| [`userConfig`](#user-configuration)  | Object                                   | Valores que Claude Code solicita ao usuário quando o plugin está habilitado                                                                                                                                                                                                                                                        |
| [`channels`](#channels)              | Array de objetos                         | Canais de mensagem que o plugin fornece, cada um vinculado a um de seus servidores MCP                                                                                                                                                                                                                                             |
| `skills`                             | Caminho, ou array de caminhos            | Diretórios para escanear em busca de skills, cada um um diretório de pastas `<name>/SKILL.md` ou uma pasta contendo `SKILL.md` diretamente. `"."` nomeia a raiz do plugin. Adiciona ao scan padrão `skills/`                                                                                                                       |
| [`commands`](#commands)              | Caminho, array de caminhos, ou objeto    | Arquivos de comando `.md` planos, diretórios deles, ou um mapa de objeto de nome de comando para `source` ou `content`. Substitui o scan padrão `commands/`                                                                                                                                                                        |
| `agents`                             | Caminho, ou array de caminhos            | Arquivos de agente `.md`. Diretórios não são aceitos. Substitui o scan padrão `agents/`                                                                                                                                                                                                                                            |
| [`hooks`](#hooks)                    | Caminho, objeto, ou array de qualquer um | Arquivos hook `.json` ou configuração de hook inline. Carregados junto com `hooks/hooks.json`                                                                                                                                                                                                                                      |
| [`mcpServers`](#mcpservers)          | Caminho, objeto, ou array de qualquer um | Arquivos de configuração MCP `.json`, bundles `.mcpb` ou `.dxt`, ou configurações de servidor inline com chave por nome. Carregados junto com `.mcp.json`; um nome de servidor declarado depois substitui um anterior                                                                                                              |
| [`lspServers`](#lspservers)          | Caminho, objeto, ou array de qualquer um | Arquivos de configuração LSP `.json` ou configurações de servidor inline com chave por nome. Carregados junto com `.lsp.json`                                                                                                                                                                                                      |
| `outputStyles`                       | Caminho, ou array de caminhos            | Arquivos de estilo de saída ou diretórios. Substitui o scan padrão `output-styles/`                                                                                                                                                                                                                                                |
| `workflows`                          | Caminho, ou array de caminhos            | Arquivos de [Workflow](/docs/pt/workflows#distribute-a-workflow-in-a-plugin) `.js` ou diretórios. Substitui o scan padrão `workflows/`                                                                                                                                                                                                  |
| `experimental`                       | Object                                   | Contêiner para `themes`, `monitors` e `evals`, cuja forma de manifesto ainda pode mudar                                                                                                                                                                                                                                            |
| `experimental.themes`                | Caminho, ou array de caminhos            | Arquivos de tema ou diretórios. Substitui o scan padrão `themes/`. Uma chave `themes` de nível superior ainda carrega, com um aviso `claude plugin validate`                                                                                                                                                                       |
| [`experimental.monitors`](#monitors) | Caminho, ou array inline                 | Um arquivo `.json` contendo o array de monitors, ou o próprio array. Padrão é `monitors/monitors.json`. Uma chave `monitors` de nível superior ainda carrega, com um aviso `claude plugin validate`. Monitors executam apenas em sessões interativas, e não no Amazon Bedrock, Agent Platform do Google Cloud ou Microsoft Foundry |
| `experimental.evals`                 | Caminho, ou array de caminhos            | Diretório que contém os [casos de eval](/docs/pt/plugin-evals#use-a-different-eval-directory) do plugin quando não é o padrão `evals/`. `claude plugin eval --eval-dir` o substitui                                                                                                                                                     |

Na coluna Tipo, um caminho é uma string relativa à raiz do plugin, como `"./custom/commands"`.

<h3 id="name">
  `name`
</h3>

O identificador do plugin. Deve ser não vazio, sem espaços, `@`, `:`, separadores de caminho, caracteres de controle ou caracteres de formatação bidirecional; use kebab-case.

Claude Code namespaces cada componente sob ele, então um agente `reviewer` no plugin `deploy-tools` aparece como `deploy-tools:reviewer`.

<h3 id="displayname">
  `displayName`
</h3>

O nome mostrado na UI no lugar de `name`. Pode conter espaços e qualquer capitalização, e não é usado para namespacing ou lookup.

Para um plugin instalado do marketplace, um `displayName` na [entrada do marketplace](/docs/pt/plugins/marketplace-reference#plugin-entries) tem precedência sobre este valor.

<h3 id="version">
  `version`
</h3>

Uma string de versão, não verificada contra semver. Configurá-la fixa o plugin nessa versão até você alterá-la; veja [Versões e atualizações](/docs/pt/plugins/loading#versions-and-updates). Um plugin com uma [`command` source](/docs/pt/plugins/marketplace-reference), um plugin de um [marketplace hospedado em claude.ai](/docs/pt/plugins/install#add-from-claude-ai) e um plugin [carregado no local](/docs/pt/plugins/loading#find-plugins-on-disk) de um marketplace adicionado como um diretório local não são fixados por este campo.

<h3 id="metadata">
  `metadata`
</h3>

Um objeto de forma livre para seus próprios dados, como campos de catálogo ou direito. Claude Code não o lê. Requer Claude Code v2.1.222 ou posterior.

<h3 id="defaultenabled">
  `defaultEnabled`
</h3>

Se o plugin inicia habilitado quando o usuário não o configurou em [`enabledPlugins`](/docs/pt/settings-reference#enabledplugins). Padrão é `true`. Um plugin que um plugin habilitado depende inicia habilitado independentemente. O mesmo campo na entrada do marketplace substitui este.

Uma vez que a entrada `enabledPlugins` de um usuário é escrita, ela persiste entre atualizações de plugin, então alterar `defaultEnabled` em uma versão posterior não altera a configuração para um usuário existente.

<h3 id="dependencies">
  `dependencies`
</h3>

Plugins que devem estar habilitados para este funcionar. Cada entrada é `"name"`, `"name@marketplace"` ou `{ "name": "...", "marketplace": "...", "version": "..." }`. Nomes simples resolvem contra o próprio marketplace deste plugin. Veja [restrições de dependência](/docs/pt/plugins/dependencies).

<h3 id="settings">
  `settings`
</h3>

Configurações que Claude Code aplica enquanto o plugin está habilitado. Apenas `agent` e `subagentStatusLine` têm efeito; outras chaves são removidas no carregamento. Um `settings.json` na raiz do plugin tem precedência sobre esta chave. Veja [Configurações padrão](/docs/pt/plugins/components#default-settings).

<h2 id="component-path-forms">
  Formas de caminho de componente
</h2>

Cada chave de componente aceita um caminho relativo à raiz do plugin. `hooks`, `mcpServers`, `lspServers` e `experimental.monitors` também aceitam configuração inline, `commands` também aceita um mapa de objeto, e `mcpServers` também aceita caminhos de bundle MCP e URLs. Os exemplos a seguir mostram cada forma aceita uma vez. Para o que cada componente faz em tempo de execução, veja [Componentes de plugin](/docs/pt/plugins/components).

<h3 id="path-only-fields">
  Campos apenas de caminho
</h3>

`agents`, `skills`, `outputStyles`, `workflows` e `experimental.themes` recebem um caminho ou um array de caminhos. Entradas `agents` devem ser arquivos `.md`, e entradas `skills` devem ser diretórios. Os outros três aceitam um diretório ou um arquivo.

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

<h3 id="commands">
  `commands`
</h3>

`commands` recebe um caminho, um array de caminhos, ou um mapa de objeto. Um caminho nomeia um arquivo de comando `.md` plano ou um diretório. No mapa de objeto, cada chave se torna o nome do comando após o prefixo do plugin. Por exemplo, `"about"` no plugin `deploy-tools` executa como `/deploy-tools:about`.

Cada valor define exatamente um de `source` ou `content`, e uma entrada que define ambos ou nenhum falha na validação. Os outros campos nesta tabela são opcionais:

| Campo          | Tipo             | Descrição                                                             |
| :------------- | :--------------- | :-------------------------------------------------------------------- |
| `source`       | string           | Caminho para o arquivo Markdown do comando, relativo à raiz do plugin |
| `content`      | string           | Markdown inline para o corpo do comando, em vez de `source`           |
| `description`  | string           | Descrição mostrada para o comando                                     |
| `argumentHint` | string           | Dica de argumento mostrada após o nome do comando, como `[file]`      |
| `model`        | string           | Modelo padrão para o comando                                          |
| `allowedTools` | array de strings | Ferramentas que o comando pode usar sem solicitar                     |

Este mapa declara um comando de um arquivo e um de conteúdo inline:

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

<h3 id="hooks">
  `hooks`
</h3>

`hooks` recebe um caminho de arquivo `.json`, um objeto de hooks inline na mesma forma que [`hooks` em `settings.json`](/docs/pt/hooks#configuration), ou um array misturando ambos. Para eventos de hook e campos de handler, veja a [referência de hooks](/docs/pt/hooks#hook-events).

Claude Code mescla o que você declara com `hooks/hooks.json` quando esse arquivo existe.

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

<h3 id="mcpservers">
  `mcpServers`
</h3>

`mcpServers` recebe um caminho de arquivo `.json`, um caminho de bundle MCP ou URL, um mapa inline, ou um array misturando-os. Para campos de configuração de servidor, veja [servidores MCP fornecidos por plugin](/docs/pt/mcp#plugin-provided-mcp-servers).

Claude Code carrega `.mcp.json` na raiz do plugin primeiro, depois cada forma declarada em ordem. Um nome de servidor declarado depois substitui um anterior.

Um valor `mcpServers` recebe uma destas formas:

| Forma                      | Valor de exemplo                                                                       | O que Claude Code faz                                                                                      |
| :------------------------- | :------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| Caminho de arquivo `.json` | `"./mcp/servers.json"`                                                                 | Lê o arquivo como um mapa `mcpServers`                                                                     |
| Caminho de bundle MCP      | `"./bundle.mcpb"`                                                                      | Extrai o bundle `.mcpb` ou `.dxt` em `.mcpb-cache/` sob a raiz do plugin e lê sua configuração de servidor |
| URL de bundle MCP          | `"https://example.com/server.mcpb"`                                                    | Baixa o bundle em `.mcpb-cache/`, depois o lê                                                              |
| Mapa inline                | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | Usa o mapa como configurações de servidor com chave por nome                                               |

Um caminho de bundle ou URL deve terminar em `.mcpb` ou `.dxt`. Qualquer outra extensão falha na validação.

<h3 id="lspservers">
  `lspServers`
</h3>

`lspServers` recebe um caminho de arquivo `.json`, um mapa inline de nome de servidor para configuração, ou um array de qualquer um.

Claude Code carrega `.lsp.json` na raiz do plugin primeiro, depois cada configuração declarada em ordem. Um nome de servidor declarado depois substitui um anterior.

Cada configuração de servidor é um objeto estrito com estes campos. Uma chave desconhecida falha na validação.

| Campo                   | Obrigatório | Descrição                                                                                                                                                                                                   |
| :---------------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`               | Sim         | Binário do servidor de linguagem. Sem espaços a menos que o valor comece com `/`; coloque argumentos em `args`                                                                                              |
| `extensionToLanguage`   | Sim         | Mapa de extensão de arquivo para ID de linguagem LSP, pelo menos uma entrada. As chaves começam com um ponto, como `".go"`                                                                                  |
| `args`                  | Não         | Argumentos passados para o servidor                                                                                                                                                                         |
| `transport`             | Não         | Transporte de comunicação: `stdio` (padrão) ou `socket`. Claude Code aceita `socket` mas executa cada servidor sobre stdio, então as regras do protocolo stdout se aplicam a todos os servidores            |
| `env`                   | Não         | Variáveis de ambiente para o processo do servidor                                                                                                                                                           |
| `initializationOptions` | Não         | Opções enviadas na solicitação de inicialização                                                                                                                                                             |
| `settings`              | Não         | Configurações enviadas por `workspace/didChangeConfiguration`                                                                                                                                               |
| `workspaceFolder`       | Não         | Caminho da pasta de workspace para o servidor                                                                                                                                                               |
| `startupTimeout`        | Não         | Milissegundos para esperar pela inicialização, um inteiro positivo                                                                                                                                          |
| `shutdownTimeout`       | Não         | Milissegundos para esperar por um desligamento gracioso, um inteiro positivo. Quando o tempo limite decorre, Claude Code encerra o processo do servidor. Quando não definido, nenhum tempo limite se aplica |
| `restartOnCrash`        | Não         | Se deve reiniciar o servidor após ele falhar. Padrão é `true`. Defina como `false` para deixar um servidor que falhou parado em vez de reiniciá-lo                                                          |
| `maxRestarts`           | Não         | Tentativas de reinicialização antes de desistir, zero ou mais                                                                                                                                               |
| `diagnostics`           | Não         | Se deve enviar diagnósticos para o contexto após edições. Padrão é `true`                                                                                                                                   |

Esta configuração inline executa `gopls` para arquivos `.go`:

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

Para os servidores de linguagem que Anthropic publica como plugins e como os servidores se comportam em tempo de execução, veja [Inteligência de código](/docs/pt/plugins/code-intelligence).

<h3 id="monitors">
  `monitors`
</h3>

`experimental.monitors` recebe um caminho de arquivo `.json` ou o array inline. Quando você omite a chave, Claude Code carrega `monitors/monitors.json` se existir.

Cada entrada é um objeto estrito com estes campos.

| Campo         | Obrigatório | Descrição                                                                                                                                                                       |
| :------------ | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | Sim         | Identificador único dentro do plugin                                                                                                                                            |
| `command`     | Sim         | Comando shell que Claude Code executa como um processo de background persistente no diretório de trabalho da sessão                                                             |
| `description` | Sim         | Resumo breve mostrado no painel de tarefas e resumos de notificação                                                                                                             |
| `when`        | Não         | Com `"always"`, o padrão, o monitor inicia no início da sessão e no recarregamento do plugin. Com `"on-skill-invoke:<skill>"`, ele inicia a primeira vez que essa skill executa |

Este array inline declara um monitor que inicia a primeira vez que a skill `deploy` executa:

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

Um `command` de monitor não pode referenciar `${user_config.*}`. Veja [Campos que executam através de um shell](#fields-that-run-through-a-shell).

<h2 id="path-rules">
  Regras de caminho
</h2>

Cada caminho de componente em um manifesto é relativo à raiz do plugin e deve começar com `./`. Um caminho como `commands/foo.md` falha na validação. `skills` e `mcpServers` cada um aceitam uma forma fora dessa regra:

* **`skills`**: também aceita `"."`. Ambos `"."` e `"./"` denotam a raiz do plugin. Antes de v2.1.221, `"."` falhou na validação do manifesto, então use `"./"` quando o plugin deve carregar em versões anteriores
* **`mcpServers`**: também aceita uma URL de bundle `https://`

<h3 id="containment-and-existence">
  Contenção e existência
</h3>

Cada caminho de componente deve resolver dentro da raiz do plugin e deve existir. `claude plugin validate` não verifica os caminhos `outputStyles`, `lspServers`, `monitors` ou `themes`, então um caminho ruim nesses campos falha apenas quando o plugin carrega:

* **Contenção**: um caminho que resolve fora da raiz do plugin não carrega, e a aba **Errors** do `/plugin` mostra `<component> path escapes plugin directory: <path>`. Um caminho contendo `..` é o caso usual, e `claude plugin validate` o relata como `Path contains ".." which could be a path traversal attempt`
* **Existência**: um caminho que não existe não carrega, e a aba **Errors** do `/plugin` mostra `<component> path not found: <path>`. `claude plugin validate` o relata como `Path not found`

<h3 id="how-each-key-combines-with-its-default-location">
  Como cada chave se combina com seu local padrão
</h3>

Cada chave de componente substitui seu local padrão, adiciona a ele, ou mescla com ele:

* **Substitui o padrão**: `commands`, `agents`, `outputStyles`, `workflows`, `experimental.themes`, `experimental.monitors`. Quando você define `commands`, o diretório padrão `commands/` não é escaneado. Para manter o padrão e adicionar mais, liste-o explicitamente: `"commands": ["./commands/", "./extras/"]`
* **Adiciona ao padrão**: `skills`. O diretório `skills/` ainda é escaneado, e os diretórios listados carregam junto com ele
* **Mescla**: `hooks`, `mcpServers`, `lspServers`. O arquivo padrão carrega primeiro, e o que o manifesto declara mescla nele, conforme descrito em [Formas de caminho de componente](#component-path-forms)

Se um plugin tem uma pasta padrão como `commands/` e também define a chave de manifesto que a substitui, Claude Code carrega os caminhos do manifesto e não a pasta. `claude plugin list` e a interface `/plugin` então mostram o aviso `Default <folder>/ folder is ignored because the manifest sets "<key>"`.

Para evitar o aviso, defina a chave para um caminho dentro dessa pasta: `"commands": ["./commands/deploy.md"]` nomeia um arquivo na pasta padrão e não produz aviso.

<h2 id="user-configuration">
  Configuração do usuário
</h2>

`userConfig` declara valores que Claude Code solicita ao usuário quando o plugin está habilitado, para que os usuários não editem `settings.json` eles mesmos.

As chaves são identificadores feitos de letras, dígitos e underscores, e não podem começar com um dígito.

Cada valor é um objeto estrito com estes campos. Uma chave desconhecida falha na validação.

| Campo         | Obrigatório | Descrição                                                                                                                                                                                              |
| :------------ | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`        | Sim         | Um de `string`, `number`, `boolean`, `directory` ou `file`                                                                                                                                             |
| `title`       | Sim         | Rótulo mostrado no diálogo de configuração                                                                                                                                                             |
| `description` | Sim         | Texto de ajuda mostrado sob o campo                                                                                                                                                                    |
| `required`    | Não         | Se `true`, o diálogo de configuração não aceita um valor vazio                                                                                                                                         |
| `default`     | Não         | Valor usado quando o usuário não fornece nada: uma string, número, booleano ou array de strings                                                                                                        |
| `options`     | Não         | Para `string`, os valores que o campo aceita, mostrados como um picker em `/config`. Veja [Limitar um campo a opções fixas](#limit-a-field-to-fixed-options). Requer Claude Code v2.1.271 ou posterior |
| `multiple`    | Não         | Para `string`, permite um array de strings                                                                                                                                                             |
| `sensitive`   | Não         | Se `true`, mascara entrada e armazena o valor em armazenamento seguro em vez de `settings.json`                                                                                                        |
| `min` / `max` | Não         | Limites para `number`                                                                                                                                                                                  |

Cada opção de cada plugin habilitado também aparece como uma linha no painel `/config`, exceto opções `sensitive` e listas `multiple`. As linhas `/config` requerem Claude Code v2.1.269 ou posterior.

Este `userConfig` declara um endpoint e um token mascarado:

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

<h3 id="limit-a-field-to-fixed-options">
  Limitar um campo a opções fixas
</h3>

Defina `options` em um campo `userConfig` para fazer os usuários escolherem seu valor de uma lista fixa.

Para limitar um campo `tone` a três opções, liste-as em `options` e defina `default` para uma delas:

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

Se você declarar `options` em qualquer campo, usuários em versões Claude Code antes de v2.1.271 não podem carregar o plugin.

`options` se aplica a um campo `string` que não é `multiple` ou `sensitive`. Defina `default` para um dos valores listados, ou defina `required: true` para que o usuário escolha um. Cada opção é um rótulo simples de 1 a 64 caracteres, e `claude plugin validate`, que você executa no seu shell, relata qualquer coisa que rejeita. Um plugin cujas `options` quebram essas regras falha ao carregar.

<h3 id="where-values-are-stored">
  Onde os valores são armazenados
</h3>

Valores não sensíveis são salvos em [`pluginConfigs`](/docs/pt/settings-reference#pluginconfigs) no `settings.json` do usuário. Valores sensíveis vão para o armazenamento de credenciais seguro da plataforma. A [página de configurações](/docs/pt/settings-reference#pluginconfigs) lista quais arquivos de configurações `pluginConfigs` é lido.

<h3 id="reference-a-saved-value">
  Referenciar um valor salvo
</h3>

Referencie um valor salvo onde o plugin precisa dele, em uma de duas formas:

* **`${user_config.KEY}`**: substituído em configuração de servidor MCP, configuração de servidor LSP, hook `args` em [forma exec](/docs/pt/hooks#exec-form-and-shell-form), e conteúdo de skill e agente. Em conteúdo de skill e agente, apenas valores não sensíveis são substituídos, e um valor sensível lá se torna um placeholder
* **`CLAUDE_PLUGIN_OPTION_<KEY>`**: exportado para processos de hook para cada opção, com `<KEY>` em maiúsculas. Um hook em forma shell lê `$CLAUDE_PLUGIN_OPTION_API_TOKEN` para `api_token`

<h3 id="fields-that-run-through-a-shell">
  Campos que executam através de um shell
</h3>

Comandos de hook em forma shell, comandos de monitor e MCP [`headersHelper`](/docs/pt/mcp#use-dynamic-headers-for-custom-authentication) rejeitam `${user_config.*}`. Um componente que o referencia em um desses campos falha com um [erro](/docs/pt/errors#plugin-command-references-user-config) em vez de executar, porque o valor do campo é passado para um shell que re-analisaria o valor substituído.

A tabela mostra como o valor pode chegar a cada um desses campos.

| Campo                           | Como o valor pode chegar a ele                                                                                                                                                                                                       |
| :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Comandos de hook em forma shell | Use [forma exec](/docs/pt/hooks#exec-form-and-shell-form) com `args`, ou leia `CLAUDE_PLUGIN_OPTION_<KEY>` do ambiente do hook                                                                                                            |
| Comandos de monitor             | Não através de Claude Code. Processos de monitor não recebem `CLAUDE_PLUGIN_OPTION_<KEY>`, então o script de monitor tem que obter o valor por conta própria                                                                         |
| MCP `headersHelper`             | Não através de Claude Code. O ambiente do helper carrega `CLAUDE_PLUGIN_ROOT`, `CLAUDE_CODE_MCP_SERVER_NAME` e `CLAUDE_CODE_MCP_SERVER_URL` mas nenhum valor de opção, então o script helper tem que obter o valor por conta própria |

<h2 id="channels">
  Canais
</h2>

`channels` declara os canais de mensagem que um plugin fornece, como uma ponte para um aplicativo de chat. Quando você declara um, Claude Code pode solicitar a configuração do canal quando o plugin está habilitado. Para como o servidor injeta mensagens, veja a [referência de canais](/docs/pt/channels-reference#package-as-a-plugin).

Cada entrada é um objeto estrito vinculado a um dos servidores MCP do plugin, com estes campos:

| Campo         | Obrigatório | Descrição                                                                                                                                                                   |
| :------------ | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server`      | Sim         | Chave do servidor MCP em `mcpServers` deste plugin ao qual o canal se vincula                                                                                               |
| `displayName` | Não         | Nome mostrado no título do diálogo de configuração. Padrão é o nome do servidor                                                                                             |
| `userConfig`  | Não         | Opções para solicitar, na mesma forma que [top-level `userConfig`](#user-configuration). Valores salvos substituem em referências `${user_config.KEY}` no `env` do servidor |

Este manifesto vincula um canal ao servidor MCP `telegram` do plugin e solicita um token de bot que substitui no `env` do servidor:

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

<h2 id="environment-variables">
  Variáveis de ambiente
</h2>

Claude Code fornece três variáveis de caminho para componentes de plugin. Referencie-as como `${NAME}` nos campos listados em [Onde cada variável resolve](#where-each-variable-resolves), e leia-as como variáveis de ambiente nos processos que as recebem.

| Variável                | Resolve para                                                                                                                                                                                                               | Use-a para                                                          |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------ |
| `${CLAUDE_PLUGIN_ROOT}` | Caminho absoluto da versão instalada do plugin                                                                                                                                                                             | Scripts, binários e arquivos de configuração agrupados com o plugin |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`, criado na primeira referência e mantido entre atualizações de plugin. `<id>` é o identificador do plugin com cada caractere diferente de uma letra, dígito, `_` ou `-` substituído por `-` | Dependências instaladas como `node_modules`, código gerado e caches |
| `${CLAUDE_PROJECT_DIR}` | A raiz do projeto                                                                                                                                                                                                          | Scripts e arquivos de configuração locais do projeto                |

`${CLAUDE_PLUGIN_ROOT}` muda quando o plugin atualiza, então não escreva estado lá. Para onde a raiz se move e quando o diretório antigo é limpo, veja a [página de carregamento](/docs/pt/plugins/loading).

Quando você desinstala o plugin do último lugar onde está instalado, o diretório `${CLAUDE_PLUGIN_DATA}` é deletado a menos que você passe [`--keep-data`](/docs/pt/plugins/cli-reference).

<h3 id="where-each-variable-resolves">
  Onde cada variável resolve
</h3>

Em cada componente de plugin, referências `${...}` resolvem inline em campos específicos, e alguns componentes também recebem as variáveis em seu ambiente de processo:

| Componente de plugin                | Campos onde `${...}` resolve                | Exportado para o processo                                                                       |
| :---------------------------------- | :------------------------------------------ | :---------------------------------------------------------------------------------------------- |
| Comandos de hook                    | Em qualquer lugar em `command` e `args`     | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR` e `CLAUDE_PLUGIN_OPTION_<KEY>` |
| Comandos de monitor                 | Em qualquer lugar em `command`              | Não exportado                                                                                   |
| Servidores MCP `stdio`              | `command`, `args`, `env`                    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`                                                      |
| Servidores MCP `http`, `sse`, `ws`  | `url`, `headers`, `headersHelper`           | Não aplicável                                                                                   |
| Servidores LSP                      | `command`, `args`, `env`, `workspaceFolder` | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`                                |
| Conteúdo de skill, comando e agente | Em qualquer lugar no corpo Markdown         | Não aplicável                                                                                   |

As variáveis não estão presentes no ambiente de comandos que Claude executa através da ferramenta Bash, na sessão principal ou em um subagente. Em conteúdo de skill, comando e agente, escreva a referência `${...}` no corpo Markdown em vez disso, e Claude Code substitui o caminho inline quando carrega o conteúdo.

<h3 id="quoting-and-path-separators">
  Citação e separadores de caminho
</h3>

Mantenha cada caminho substituído um único argumento:

* **Comandos de hook**: use [forma exec](/docs/pt/hooks#exec-form-and-shell-form) com `args` para que cada caminho seja um argumento sem citação
* **Hooks em forma shell e comandos de monitor**: envolva a variável em aspas duplas para que um caminho com espaços permaneça uma palavra

Este hook em forma shell executa um script agrupado com o plugin:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

No Windows, os caminhos substituídos usam barras para frente para que um shell não leia barras invertidas como escapes.

<h2 id="standard-layout">
  Layout padrão
</h2>

Cada tipo de componente tem um local padrão sob a raiz do plugin, usado quando o manifesto não aponta para outro lugar.

| Componente       | Local padrão                 | Conteúdo                                                                                                                                                                                                                                                                                                                                                         |
| :--------------- | :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Manifesto        | `.claude-plugin/plugin.json` | Metadados e configuração do plugin. Opcional                                                                                                                                                                                                                                                                                                                     |
| Skills           | `skills/`                    | Um `<name>/SKILL.md` por skill. Um plugin com `SKILL.md` em sua raiz, sem `skills/` e sem chave `skills` carrega como uma única skill                                                                                                                                                                                                                            |
| Comandos         | `commands/`                  | Arquivos de comando Markdown planos. Prefira `skills/` para novos plugins                                                                                                                                                                                                                                                                                        |
| Agentes          | `agents/`                    | Arquivos Markdown de agente. Subpastas são parte do [nome do agente](/docs/pt/plugins/components#agents)                                                                                                                                                                                                                                                              |
| Hooks            | `hooks/hooks.json`           | Configuração de hook                                                                                                                                                                                                                                                                                                                                             |
| Servidores MCP   | `.mcp.json`                  | Definições de servidor MCP                                                                                                                                                                                                                                                                                                                                       |
| Servidores LSP   | `.lsp.json`                  | Configurações de servidor LSP                                                                                                                                                                                                                                                                                                                                    |
| Estilos de saída | `output-styles/`             | Arquivos de estilo de saída Markdown                                                                                                                                                                                                                                                                                                                             |
| Workflows        | `workflows/`                 | Arquivos de workflow `.js`                                                                                                                                                                                                                                                                                                                                       |
| Temas            | `themes/`                    | Arquivos de tema JSON                                                                                                                                                                                                                                                                                                                                            |
| Monitors         | `monitors/monitors.json`     | O array de monitors                                                                                                                                                                                                                                                                                                                                              |
| Executáveis      | `bin/`                       | Arquivos aqui estão no `PATH` da ferramenta Bash enquanto o plugin está habilitado, então Claude os executa como comandos simples. claude.ai e Cowork não instalam um plugin que tem este diretório, incluindo um que você [distribui através das configurações de organização claude.ai](/docs/pt/plugins/host-marketplace#distribute-through-organization-settings) |
| Configurações    | `settings.json`              | Padrões `agent` e `subagentStatusLine` aplicados enquanto o plugin está habilitado                                                                                                                                                                                                                                                                               |

Um plugin que usa cada local padrão, mais uma pasta `scripts/` que seus hooks chamam, é disposto assim:

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

Para clicar através deste layout e ler o que cada arquivo faz, abra o [explorador de plugin](/docs/pt/plugins/components#explore-the-plugin-directory).

Um `CLAUDE.md` na raiz do plugin não é carregado como contexto, e `claude plugin validate` avisa quando encontra um. Para incluir instruções que carregam no contexto de Claude, coloque-as em uma skill.

<h2 id="marketplace-entries-and-the-manifest">
  Entradas de marketplace e o manifesto
</h2>

Uma [entrada de marketplace](/docs/pt/plugins/marketplace-reference) aceita cada campo nesta página junto com [seus próprios campos](/docs/pt/plugins/marketplace-reference#plugin-entries), incluindo `strict`.

O campo `strict` decide se a entrada pode adicionar componentes a um plugin que tem seu próprio `plugin.json`. Padrão é `true`.

<h3 id="how-entry-fields-combine-with-plugin-json">
  Como campos de entrada se combinam com `plugin.json`
</h3>

A entrada serve como o manifesto, adiciona componentes a ele, ou entra em conflito com ele:

* **Sem `plugin.json`**: a entrada é o manifesto, independentemente de `strict`. Hooks de entrada carregam apenas na forma de objeto inline. Para um caminho de arquivo ou array lá, a aba **Errors** do `/plugin` mostra um erro `not yet supported in a marketplace entry`
* **`plugin.json` presente, `strict` não definido ou `true`**: Claude Code carrega o manifesto e anexa `commands`, `agents`, `skills`, `outputStyles` e `themes` da entrada a ele. Para `hooks`, os matchers da entrada para um evento substituem os matchers do manifesto para esse mesmo evento, e eventos que apenas o manifesto declara mantêm os deles
* **`plugin.json` presente, `strict: false`**: uma entrada que declara qualquer um de `commands`, `agents`, `skills`, `hooks`, `outputStyles` ou `themes` é um conflito, e o plugin falha ao carregar com `Plugin <name> has conflicting manifests`

Quando uma [entrada de marketplace cuja `source` é a raiz do marketplace](/docs/pt/plugins/marketplace-reference) lista subdiretórios `skills` específicos, apenas esses subdiretórios carregam, e o diretório padrão `skills/` do plugin não é escaneado. Uma chave `skills` no manifesto em vez disso [adiciona ao padrão](#how-each-key-combines-with-its-default-location).

<h3 id="metadata-precedence">
  Precedência de metadados
</h3>

Alguns campos de metadados têm uma precedência fixa independentemente de `strict`:

* **`defaultEnabled` e campos de exibição**: o `defaultEnabled` da entrada e seus [campos de exibição](/docs/pt/plugins/marketplace-reference#entry-and-plugin-json) como `displayName` substituem os do manifesto
* **`version`**: o `version` do manifesto substitui o da entrada
* **`name`**: quando a entrada lista o plugin sob um `name` diferente do manifesto, `enabledPlugins` usa o nome da entrada, e componentes são namespaced sob o nome do manifesto

Para a tabela de precedência completa, veja [Modo estrito](/docs/pt/plugins/marketplace-reference).

<h2 id="next-steps">
  Próximos passos
</h2>

* [Adicionar componentes a um plugin](/docs/pt/plugins/components): o que cada componente faz em tempo de execução, com um exemplo que valida
* [Referência de marketplace](/docs/pt/plugins/marketplace-reference): os campos de entrada que um marketplace pode definir para seu plugin
* [Referência de comandos de plugin](/docs/pt/plugins/cli-reference#plugin-validate): flags e saída de `claude plugin validate`
* [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting#claude-plugin-validate-reports-errors): cada mensagem de validação com sua correção
