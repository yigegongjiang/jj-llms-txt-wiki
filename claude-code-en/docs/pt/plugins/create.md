> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Criar um plugin Claude Code

> Crie seu primeiro plugin Claude Code a partir de um diretório vazio, teste-o sem um marketplace e converta uma configuração .claude/ existente.

Um plugin é um diretório de skills, agents, hooks e servidores MCP, mais um arquivo `plugin.json`, chamado de manifest, que nomeia o plugin. Claude Code carrega o diretório como uma unidade, para que você possa compartilhá-lo com colegas de equipe, instalá-lo em vários projetos ou publicá-lo em um marketplace.

Esta página é para pessoas que escrevem seus próprios plugins.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Instalando o plugin de alguém**: veja [Instalar plugins](/docs/pt/plugins/install)
  * **Não tem certeza se precisa de um plugin**: veja [Decidir se você precisa de um plugin](/docs/pt/plugins/overview#decide-whether-you-need-a-plugin) na visão geral
  * **Os usuários do seu plugin estão em claude.ai ou em Cowork**: a mesma pasta instala lá com um subconjunto diferente de componentes. Veja [Plugins em claude.ai e em Cowork](https://claude.com/docs/plugins/overview)
</Note>

Comece pela seção que corresponde ao que você já tem:

* **Nada ainda**: siga [Criar seu primeiro plugin](#create-your-first-plugin), depois [Desenvolver sem um marketplace](#develop-without-a-marketplace) e [Testar e depurar](#test-and-debug).
* **Arquivos sob `.claude/` já existem**: faça o passo a passo do primeiro plugin uma vez para aprender o layout, depois siga [Converter uma configuração `.claude/` existente](#convert-an-existing-claude-setup).

<h2 id="decide-when-to-use-a-plugin">
  Decidir quando usar um plugin
</h2>

Skills, agents, hooks e servidores MCP funcionam todos de forma independente em seu projeto ou diretório inicial. Mantenha essa configuração independente enquanto ela serve um projeto ou apenas você. Crie um plugin quando quiser compartilhar a configuração com colegas de equipe, instalá-la em vários projetos ou publicar versões lançadas.

Quando você move skills, agents, hooks e configuração MCP independentes para um plugin, sua localização e nomes mudam:

* **Onde os arquivos vão**: sob o diretório próprio do plugin, chamado de raiz do plugin, como `skills/`, `agents/`, `hooks/hooks.json` e `.mcp.json`.
* **Como são nomeados**: skills e agents do plugin recebem o nome do plugin como prefixo, como `/my-plugin:hello`, para que dois plugins possam cada um fornecer uma skill `hello` sem colidir.

Para mover uma configuração existente para um plugin, veja [Converter uma configuração `.claude/` existente](#convert-an-existing-claude-setup).

<h2 id="create-your-first-plugin">
  Criar seu primeiro plugin
</h2>

Neste passo a passo, você cria um plugin cujo único componente é uma skill, uma saudação, e a executa com `--plugin-dir`, que carrega um plugin para uma sessão sem instalá-lo. Um plugin pode conter qualquer mistura de [componentes](/docs/pt/plugins/components), como skills, agents, hooks e servidores MCP, e nenhum é obrigatório; uma skill é o exemplo menor que mostra o layout.

Você precisa ter Claude Code [instalado e conectado](/docs/pt/quickstart#step-1-install-claude-code).

Abra um terminal no diretório onde você deseja manter o plugin, como `~/projects`, e execute os comandos nessas etapas a partir dele. Você pode manter um plugin em qualquer lugar, porque você passa seu caminho para Claude Code quando inicia uma sessão.

<Steps>
  <Step title="Criar o diretório do plugin">
    Crie o diretório do plugin, com uma pasta `.claude-plugin/` dentro dele para conter o manifest:

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="Escrever o manifest">
    O [manifest](/docs/pt/plugins/manifest-reference) é um arquivo JSON chamado `plugin.json` que diz ao Claude Code o nome do plugin e o descreve. Salve este em `my-first-plugin/.claude-plugin/plugin.json`:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    Os quatro campos fazem isto:

    * **`name`**: obrigatório. Identifica o plugin e se torna o prefixo em cada skill e agent que o plugin fornece. Não coloque espaços nele.
    * **`description`**: o texto que os usuários veem para o plugin em `/plugin`.
    * **`version`**: opcional. Configurá-lo mantém os usuários nessa versão até que você a altere; [Lançar uma nova versão](/docs/pt/plugins/host-marketplace#release-a-new-version) diz quando configurá-la ou omiti-la.
    * **`author`**: quem creditar. `name` é obrigatório dentro dele; `email` e `url` são opcionais.

    Todos os outros campos estão na [referência do manifest](/docs/pt/plugins/manifest-reference#fields).

    Apenas `plugin.json` vai dentro de `.claude-plugin/`. A skill que você adiciona a seguir vai diretamente sob `my-first-plugin/`, ao lado dessa pasta.
  </Step>

  <Step title="Adicionar uma skill">
    O único componente deste plugin é uma skill. Cada skill é um diretório sob `skills/` que contém um arquivo `SKILL.md`. Crie o diretório da skill:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Depois crie `my-first-plugin/skills/hello/SKILL.md` com este conteúdo:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    A linha `disable-model-invocation: true` significa que Claude não executa a skill por conta própria, então apenas você a dispara. Remova essa linha de uma skill que você quer que Claude execute por conta própria. O comando da skill combina o nome do plugin e o nome da skill, então você executa este como `/my-first-plugin:hello`. Para os outros campos do frontmatter, veja a [referência do frontmatter da skill](/docs/pt/skills#frontmatter-reference).
  </Step>

  <Step title="Validar o plugin">
    Verifique o manifest e o frontmatter da skill antes de executar qualquer coisa:

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    O comando imprime o caminho do manifest que verificou e `✔ Validation passed`. Se imprimir `✘ Validation failed` em vez disso, cada linha acima dessa linha de resultado nomeia o campo a corrigir. Procure cada mensagem em [`claude plugin validate` relata erros](/docs/pt/plugins/troubleshooting#claude-plugin-validate-reports-errors).
  </Step>

  <Step title="Executar Claude Code com o plugin">
    Inicie uma sessão com o plugin carregado:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Assim que Claude Code iniciar, execute a skill:

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude responde com uma saudação.
  </Step>
</Steps>

O plugin carrega apenas em sessões que você inicia com `--plugin-dir`. Para continuar trabalhando nele sem a flag, ou para testar uma compilação `.zip`, veja [Desenvolver sem um marketplace](#develop-without-a-marketplace).

<h3 id="share-the-plugin">
  Compartilhar seu plugin
</h3>

Um plugin que você construiu com [Criar seu primeiro plugin](#create-your-first-plugin) existe apenas em sua máquina. Quando estiver pronto para outras pessoas, há três maneiras de entregá-lo a elas:

* **Envie-o para algumas pessoas diretamente**: dê a elas o diretório do plugin ou um `.zip` dele, e nada precisa ser publicado. Veja [Compartilhar um plugin sem um marketplace](/docs/pt/plugins/publish#share-a-plugin-without-a-marketplace).
* **Liste-o em seu próprio marketplace**: colegas de equipe adicionam seu marketplace uma vez e instalam o plugin por nome, e recebem suas atualizações. Veja [Publicar através de seu próprio marketplace](/docs/pt/plugins/publish#publish-through-your-own-marketplace).
* **Envie-o para o marketplace da comunidade da Anthropic**: uma vez listado, qualquer pessoa que adicione esse marketplace pode instalá-lo. Veja [Enviar para o marketplace da comunidade](/docs/pt/plugins/publish#submit-to-the-community-marketplace).

<h3 id="plugin-layout">
  Layout do plugin
</h3>

Cada tipo de [componente](/docs/pt/plugins/components), como skills, agents, hooks e servidores MCP, vai em um diretório fixo sob a raiz do plugin, que é o diretório que você passa para `--plugin-dir`. Adicione apenas os diretórios que você usa. Para clicar através de um diretório de plugin completo e ler o que cada arquivo faz, abra o [explorador de plugin](/docs/pt/plugins/components#explore-the-plugin-directory).

A tabela lista os diretórios com os quais a maioria dos plugins começa, e o [layout completo](/docs/pt/plugins/manifest-reference#standard-layout) lista o resto.

| Localização                  | Conteúdo                                                                                                                               |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `.claude-plugin/plugin.json` | O manifest. Quando você carrega um plugin com `--plugin-dir` e ele não tem um manifest, Claude Code nomeia o plugin após seu diretório |
| `skills/`                    | Um diretório `<name>/SKILL.md` por skill                                                                                               |
| `commands/`                  | Arquivos Markdown simples, a forma mais antiga de skills. Use `skills/` para novos plugins                                             |
| `agents/`                    | Um arquivo Markdown por subagent                                                                                                       |
| `hooks/hooks.json`           | Configuração de hook: uma chave `"hooks"` de nível superior cujo valor tem a mesma forma que `hooks` em um arquivo de configurações    |
| `.mcp.json`                  | Definições de servidor MCP                                                                                                             |

<Warning>
  Apenas `plugin.json` vai dentro de `.claude-plugin/`. Componentes salvos lá não carregam.

  A raiz do plugin é o diretório próprio do plugin, não `~/.claude/` em si. Um `.mcp.json` salvo em `~/.claude/.mcp.json` não carrega.
</Warning>

<h2 id="develop-without-a-marketplace">
  Desenvolver sem um marketplace
</h2>

Você não precisa de um [marketplace](/docs/pt/plugins/overview#get-plugins-from-a-marketplace) para executar um plugin que está escrevendo. Carregue-o diretamente do disco ou de uma URL em vez disso:

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session): carrega um diretório ou arquivo `.zip` para uma sessão.
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session): busca um arquivo `.zip` de uma URL para uma sessão.
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session): estrutura um plugin sob `~/.claude/skills/` que carrega a cada sessão.

Se dois plugins carregados de maneiras diferentes compartilharem um nome, veja [Conflitos de nome](/docs/pt/plugins/loading#name-conflicts) para saber qual Claude Code mantém.

<h3 id="load-a-directory-or-archive-for-one-session">
  Carregar um plugin para uma sessão
</h3>

Você pode carregar um plugin para uma única sessão de três maneiras: de um diretório ou arquivo `.zip` no disco com `--plugin-dir`, de uma URL com `--plugin-url`, ou de uma variável de ambiente quando você não pode adicionar uma flag. Cada plugin carrega apenas para essa sessão, e nada é escrito em suas configurações para ele. Quando você edita os arquivos do plugin durante a sessão, execute `/reload-plugins` para carregar as alterações.

<h4 id="from-a-directory-or-zip">
  De um diretório ou `.zip`
</h4>

Quando você inicia `claude` a partir de seu shell, passe `--plugin-dir` com o diretório raiz do plugin ou um arquivo `.zip` dele. Repita a flag para carregar vários plugins:

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  De uma pasta de plugins
</h4>

Para carregar vários plugins de um lugar, passe uma pasta que os contenha, como `--plugin-dir ./plugins`. Carregar uma pasta de plugins requer Claude Code v2.1.265 ou posterior.

Se a pasta não tiver um diretório `.claude-plugin/` e nenhum componente de plugin em seu nível superior, Claude Code a trata como uma pasta de plugins. Cada subpasta imediata que tenha um manifest `.claude-plugin/plugin.json` então carrega como um plugin separado. Tudo mais na pasta é ignorado sem um erro, incluindo uma subpasta que não tenha um manifest. Se um plugin na pasta não carregar, verifique se sua subpasta tem um `.claude-plugin/plugin.json`.

Em uma sessão interativa, você também pode adicionar e remover plugins na pasta após a inicialização:

* Uma subpasta que você adiciona carrega como um novo plugin assim que seu manifest existe.
* Quando você remove uma subpasta, seu plugin descarrega.

Uma mensagem aparece na sessão para cada uma dessas alterações. Se carregar ou descarregar um plugin no meio da conversa [invalidaria o cache de prompt](/docs/pt/prompt-caching#enabling-or-disabling-a-plugin), a alteração é mantida em vez disso, e a mensagem diz a você para executar `/reload-plugins` para aplicá-la.

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  De uma URL
</h4>

Quando você inicia `claude` a partir de seu shell, passe `--plugin-url` com o endereço de um arquivo `.zip`, como um artefato de compilação que seu CI publica:

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code baixa o arquivo na inicialização. Para carregar vários, repita a flag ou passe as URLs separadas por espaço em um argumento entre aspas.

Aponte a flag apenas para arquivos que você controla ou confia.

Se Claude Code não conseguir buscar o arquivo, ou o arquivo for inválido, ele inicia sem o plugin e registra um erro de carregamento de plugin que você pode revisar na aba **Errors** do gerenciador `/plugin`.

<h4 id="from-an-environment-variable">
  De uma variável de ambiente
</h4>

Para carregar plugins em uma sessão onde você não pode adicionar a flag `--plugin-dir`, liste seus caminhos absolutos na variável de ambiente [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/pt/env-vars#variables) em vez disso. Claude Code carrega cada caminho como carrega um caminho `--plugin-dir`. Esses plugins carregam além de qualquer um que você passe com `--plugin-dir`. [As configurações de projeto e local não podem definir essa variável](/docs/pt/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_PLUGIN_DIRS` requer Claude Code v2.1.280 ou posterior.

As configurações gerenciadas podem desativar `--plugin-dir` e `CLAUDE_CODE_PLUGIN_DIRS`. Veja [Flags que carregam um plugin para uma sessão](/docs/pt/plugins/cli-reference#flags-that-load-a-plugin-for-one-session). Para testar um plugin junto com um plugin do qual depende, veja [Testar um plugin e sua dependência localmente](/docs/pt/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="scaffold-a-plugin-that-loads-every-session">
  Fazer um plugin carregar em cada sessão
</h3>

Seu diretório de skills pessoal é `~/.claude/skills/`. Claude Code carrega qualquer pasta lá que contenha um `.claude-plugin/plugin.json` como um plugin em cada sessão, sem flag e sem etapa de instalação. `claude plugin init` estrutura um desses plugins para você.

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  Estruturar o plugin com `claude plugin init`
</h4>

`claude plugin init` escreve um plugin inicial sob `~/.claude/skills/`. Requer Claude Code v2.1.157 ou posterior. Estruture um a partir de seu shell:

```bash theme={null}
claude plugin init my-tool
```

O comando cria `~/.claude/skills/my-tool/` com um `.claude-plugin/plugin.json` e um `SKILL.md` raiz. Ele imprime `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool` seguido por `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.`

Passe `--with skills` para ter `claude plugin init` estruturar uma skill sob `skills/` para você. Os outros valores `--with` estão na [referência de comandos de plugin](/docs/pt/plugins/cli-reference#plugin-init).

<h4 id="skill-names-in-a-scaffolded-plugin">
  Nomear as skills do plugin
</h4>

A skill raiz em `~/.claude/skills/my-tool/SKILL.md` também é uma skill pessoal, então você a invoca como `/my-tool`, não `/my-tool:my-tool`. Skills que você adiciona sob `skills/` dentro do plugin recebem o prefixo do nome do plugin, como `/my-tool:example`.

<h4 id="stop-loading-the-plugin">
  Parar de carregar o plugin
</h4>

Para parar de carregar um plugin estruturado, delete seu diretório, ou execute `claude plugin disable my-tool@skills-dir` em seu shell com o nome `my-tool@skills-dir` que `claude plugin init` imprimiu. No ID `my-tool@skills-dir`, `skills-dir` fica no lugar onde um nome de marketplace estaria, porque o plugin carrega de seu diretório de skills em vez de um marketplace.

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  Compartilhar o plugin através de um repositório
</h4>

`claude plugin init` escreve o plugin em seu diretório de skills pessoal em `~/.claude/skills/`, para que carregue para você em cada projeto. Para fazer um plugin carregar para todos em um repositório, crie o mesmo layout você mesmo em `<project>/.claude/skills/<name>/`, incluindo seu `.claude-plugin/plugin.json`. Veja [Plugins compartilhados através de um repositório](/docs/pt/plugins/loading#plugins-shared-through-a-repository) para as condições sob as quais Claude Code o carrega.

<h2 id="test-and-debug">
  Testar e depurar
</h2>

Quando uma alteração em seu plugin não aparece, trabalhe através dessas verificações em ordem. Cada uma diz a você o que Claude Code fez com o plugin:

1. Em seu shell, execute `claude plugin validate <path>`. Verifica o manifest e o frontmatter de cada arquivo de skill, agent e command, e sai com `0` em `Validation passed`. Adicione `--strict` para falhar em avisos também. Códigos de saída e manipulação de diretório estão na [referência de comandos de plugin](/docs/pt/plugins/cli-reference#plugin-validate).
2. Na sessão em execução, execute `/reload-plugins` para aplicar edições que você fez no disco. Imprime uma linha `Reloaded:` com contagens. Depois confirme que uma skill carregou digitando seu comando `/plugin-name:skill`, ou encontrando o plugin na aba **Installed** de `/plugin`.
3. Na mesma sessão, execute `/plugin`. A aba **Installed** lista seu plugin e, nos detalhes do plugin, os componentes que Claude Code encontrou. A aba **Errors** lista o que falhou ao carregar e por quê, como um caminho em seu manifest que não existe.
4. De volta em seu shell, execute `claude plugin list`. Imprime plugins de sessão única e diretório de skills em suas próprias seções com `Status: ✔ loaded` ou o erro de carregamento. Para incluir o plugin que você está desenvolvendo, passe `--plugin-dir` com seu caminho antes de `plugin list`.

Para verificar um servidor MCP, execute `/mcp` na sessão para ver o status do servidor. Quando o servidor está saudável, `/mcp` o lista como conectado. Se não estiver, veja [Servidores MCP que não iniciam](/docs/pt/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Para verificar um hook, dispare o evento que ele corresponde. Por exemplo, peça a Claude para editar um arquivo para disparar um hook `PostToolUse`. Depois leia o [log de depuração](/docs/pt/hooks#debug-hooks), que mostra quais hooks corresponderam, seus códigos de saída e sua saída.

As próximas seções cobrem as falhas que você provavelmente encontrará ao desenvolver, e a [página de troubleshooting](/docs/pt/plugins/troubleshooting#build-a-plugin) tem a entrada completa para cada uma.

<h3 id="a-component-path-isn’t-found">
  Um caminho de componente não é encontrado
</h3>

A aba **Errors** de `/plugin` mostra `<component> path not found: <path>`, por exemplo `commands path not found`. Um caminho de componente em seu manifest, como `commands`, `skills`, `agents` ou `hooks`, aponta para nada. Corrija o caminho ou crie o diretório, depois execute `/reload-plugins` na sessão. Veja [`commands path not found`](/docs/pt/plugins/troubleshooting#commands-path-not-found).

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir` em uma raiz de marketplace não carrega os plugins sob `plugins/`
</h3>

`--plugin-dir` leva o diretório raiz do plugin, aquele que contém `.claude-plugin/plugin.json` e os diretórios de componentes como `skills/`. Se você apontá-lo para uma raiz de marketplace em vez disso, Claude Code não lê `marketplace.json`, então um plugin sob `plugins/` não carrega, e você não vê nenhum erro. Aponte a flag para a pasta de um plugin, ou adicione o marketplace. Veja [a entrada de troubleshooting](/docs/pt/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components).

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  O plugin carrega mas suas skills estão faltando
</h3>

O diretório `skills/` está dentro de `.claude-plugin/`, ou uma entrada `skills` no manifest aponta para um arquivo. Mova `skills/` para a raiz do plugin, aponte cada entrada `skills` para um diretório que contenha `SKILL.md`, e execute `/reload-plugins` na sessão. Veja [Plugin carrega mas suas skills estão faltando](/docs/pt/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing).

<h3 id="the-userconfig-dialog-never-appears">
  O diálogo `userConfig` nunca aparece
</h3>

O diálogo para as opções [`userConfig`](/docs/pt/plugins/components#user-configuration) do seu plugin faz parte da instalação através de `/plugin` em uma sessão. Carregar com `--plugin-dir` não o mostra, e nem `claude plugin install` no shell. Com o plugin carregado, execute `/plugin configure <plugin-name>` na sessão para abri-lo. Veja [O diálogo `userConfig` nunca aparece](/docs/pt/plugins/troubleshooting#the-userconfig-dialog-never-appears).

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  Verificar que o plugin muda o comportamento de Claude
</h3>

Um plugin que carrega sem erros ainda pode falhar em orientar Claude da maneira que você pretende. `claude plugin eval`, que você executa em seu shell, executa seus casos de teste com e sem o plugin e pontua a diferença. Veja [Testar plugins com evals](/docs/pt/plugin-evals), começando com [Criar seu primeiro conjunto de eval](/docs/pt/plugin-evals#create-your-first-eval-suite).

<h2 id="convert-an-existing-claude-setup">
  Converter uma configuração `.claude/` existente
</h2>

Se você já tem skills, agents ou hooks sob um diretório `.claude/` de um projeto, você pode movê-los para um plugin sem reescrevê-los.

Execute os comandos nessas etapas a partir da raiz do projeto, que é o diretório que contém `.claude/`, porque os caminhos `cp` são relativos a ele.

<Steps>
  <Step title="Criar a estrutura do plugin">
    Crie o diretório do plugin e sua pasta `.claude-plugin/` ao lado de `.claude/`. Você pode mover o plugin para qualquer lugar depois.

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Crie `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Copiar seus arquivos existentes">
    Copie cada diretório de configuração que você tem para a raiz do plugin, e pule o comando para qualquer diretório que você não tenha.

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    Execute `ls -a my-plugin` para confirmar que cada diretório que você copiou aparece ao lado de `.claude-plugin`.
  </Step>

  <Step title="Mover seus hooks">
    Se você tem hooks em `.claude/settings.json` ou `.claude/settings.local.json`, crie um diretório de hooks:

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    Crie `my-plugin/hooks/hooks.json` e copie o objeto `hooks` de seu arquivo de configurações para ele. O formato é o mesmo.

    Este exemplo mostra a forma com um hook que executa um linter em cada arquivo que Claude escreve ou edita. Substitua o exemplo pelo seu próprio objeto `hooks`.

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Testar o plugin migrado">
    Carregue o plugin para uma sessão:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Verifique cada componente sob seu novo nome:

    * **Skills**: execute `/my-plugin:deploy` para uma skill que era `/deploy`.
    * **Subagents**: peça a Claude para usar o agent `my-plugin:reviewer` para um agent que era `reviewer`.
    * **Hooks**: dispare o evento que cada hook corresponde.

    Se algo está faltando, trabalhe através de [Testar e depurar](#test-and-debug).
  </Step>
</Steps>

Enquanto os originais ainda estão sob `.claude/`, eles permanecem carregados ao lado das cópias do plugin:

* **Skills e agents**: os dois conjuntos não colidem, porque as skills e agents do plugin carregam o prefixo `my-plugin:`. `/deploy` e `/my-plugin:deploy` funcionam ambos, e Claude vê `reviewer` e `my-plugin:reviewer` como dois subagents.
* **Hooks**: hooks não têm prefixo, então um hook que está em seu arquivo de configurações e em `hooks/hooks.json` executa duas vezes cada vez que seu evento dispara.

Depois que você confirmou que o plugin funciona, delete os originais de `.claude/` e remova o objeto `hooks` de seu arquivo de configurações.

<h2 id="next-steps">
  Próximos passos
</h2>

* [Componentes de plugin](/docs/pt/plugins/components): adicione agents, hooks, servidores MCP, servidores LSP e configuração de usuário ao seu plugin
* [Testar plugins com evals](/docs/pt/plugin-evals): escreva casos de eval e execute-os com `claude plugin eval` para verificar com que confiabilidade o plugin orienta o comportamento de Claude
* [Publicar um plugin](/docs/pt/plugins/publish): versione-o, coloque-o em um marketplace e envie-o para o marketplace da comunidade
* [Plugins em claude.ai e em Cowork](https://claude.com/docs/plugins/overview): a mesma pasta de plugin instala em claude.ai e em Cowork. Alguns componentes são apenas Claude Code
* [Referência do manifest do plugin](/docs/pt/plugins/manifest-reference): cada campo `plugin.json`, regra de caminho e diretório
* [Skills](/docs/pt/skills): escreva as skills que seu plugin fornece
* [Plugins da Anthropic no repositório claude-code](https://github.com/anthropics/claude-code/tree/main/plugins): exemplos completos trabalhados do layout nesta página, como `feature-dev` e `code-review`
