> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Criar um marketplace

> Crie um marketplace de plugins a partir de um arquivo marketplace.json e teste-o localmente antes de hospedá-lo.

Um marketplace de plugins é um diretório ou repositório com um arquivo `.claude-plugin/marketplace.json` que lista seus plugins e onde buscar cada um. Você envia o diretório para um host git, e qualquer pessoa com acesso o registra no Claude Code com um comando e instala seus plugins a partir dele.

Crie seu próprio marketplace quando quiser que um grupo que você escolhe, como sua equipe ou sua organização, instale seus plugins e continue recebendo suas atualizações de um catálogo que você controla. O repositório pode ser privado, pode listar quantos plugins quiser, e um administrador pode [exigir em cada máquina](/docs/pt/plugins/org).

<Note>
  Estes casos são cobertos em outras páginas:

  * **Compartilhar um plugin com algumas pessoas**: envie-lhes o diretório do plugin ou um `.zip` dele. Veja [Compartilhar um plugin sem um marketplace](/docs/pt/plugins/publish#share-a-plugin-without-a-marketplace).
  * **Oferecer um plugin para todos**: envie-o para o marketplace da comunidade da Anthropic. Veja [Enviar para o marketplace da comunidade](/docs/pt/plugins/publish#submit-to-the-community-marketplace).
  * **Usar um plugin você mesmo**: carregue-o com `--plugin-dir` ou salve-o em seu diretório de skills. Veja [Desenvolver sem um marketplace](/docs/pt/plugins/create#develop-without-a-marketplace).
</Note>

Comece com [Criar um marketplace](#create-a-marketplace) para construir um em sua própria máquina e instalar um plugin a partir dele, depois [adicione mais entradas de plugin](#add-plugin-entries).

<h2 id="create-a-marketplace">
  Criar um marketplace
</h2>

Os passos a seguir criam um marketplace em sua máquina, adicionam um plugin a ele, registram-no no Claude Code e instalam o plugin a partir dele. Este é o loop completo, e é o mesmo loop que seus usuários percorrem uma vez que você hospeda o marketplace em algum lugar que eles possam acessar. Execute cada comando em seu shell, a partir do diretório onde você quer que `my-marketplace/` seja criado.

Você precisa de um plugin para listar. O exemplo usa `my-first-plugin` de [Criar seu primeiro plugin](/docs/pt/plugins/create#create-your-first-plugin), um plugin com uma skill que você executa como `/my-first-plugin:hello`; construa-o primeiro se você não tiver um plugin ainda. Para usar um plugin seu em vez disso, substitua seu diretório e seu `name` onde quer que os passos digam `my-first-plugin`. Para o que um diretório de plugin pode conter, veja o [explorador de diretório de plugin](/docs/pt/plugins/components#explore-the-plugin-directory).

<Steps>
  <Step title="Configurar o diretório do marketplace">
    Um marketplace é um diretório com um arquivo `.claude-plugin/marketplace.json`, mais os plugins que ele lista. Crie o diretório do marketplace e sua pasta `.claude-plugin/`, depois copie seu plugin sob `plugins/`:

    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
    cp -r my-first-plugin my-marketplace/plugins/
    ```

    Verifique se o plugin é válido onde agora está, para que qualquer erro posterior seja sobre o marketplace e não sobre o plugin:

    ```bash theme={null}
    claude plugin validate ./my-marketplace/plugins/my-first-plugin
    ```

    A última linha da saída lê `✔ Validation passed`.
  </Step>

  <Step title="Criar o arquivo do marketplace">
    Salve `marketplace.json` em `my-marketplace/.claude-plugin/marketplace.json`. O arquivo requer um `name`, um `owner` e um array `plugins`.

    Cada objeto em `plugins` é uma entrada de plugin e precisa de um `name` e uma `source`. Escreva a `source` da entrada como um caminho a partir da raiz do marketplace. A raiz é `my-marketplace/`, o diretório que contém `.claude-plugin/`.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-marketplace",
      "description": "Plugins for my team",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "my-first-plugin",
          "source": "./plugins/my-first-plugin",
          "description": "A greeting plugin to learn the basics"
        }
      ]
    }
    ```
  </Step>

  <Step title="Validar o marketplace">
    Execute `claude plugin validate` no diretório do marketplace para verificar a sintaxe JSON, os campos obrigatórios e cada entrada de plugin em seu `.claude-plugin/marketplace.json`.

    ```bash theme={null}
    claude plugin validate ./my-marketplace
    ```

    Para o arquivo conforme escrito no passo 2, a última linha da saída lê `✔ Validation passed`.
  </Step>

  <Step title="Adicionar o marketplace e instalar o plugin">
    Registre o diretório como um marketplace.

    ```bash theme={null}
    claude plugin marketplace add ./my-marketplace
    ```

    O comando imprime `✔ Successfully added marketplace: my-marketplace (declared in user settings)`, o que significa que o marketplace é registrado em seu arquivo de configurações do usuário.

    Instale o plugin. O id de instalação é o `name` da entrada, um `@` e o `name` do marketplace.

    ```bash theme={null}
    claude plugin install my-first-plugin@my-marketplace
    ```

    O comando imprime `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)`.

    Dentro de uma sessão, `/plugin marketplace add ./my-marketplace` registra o marketplace da mesma forma. `/plugin install my-first-plugin@my-marketplace` abre os detalhes do plugin no painel `/plugin`, onde você o instala. Para esse fluxo, veja [Instalar e gerenciar plugins](/docs/pt/plugins/install).
  </Step>

  <Step title="Confirmar que o plugin foi carregado">
    Liste os plugins instalados.

    ```bash theme={null}
    claude plugin list
    ```

    A saída lista `my-first-plugin@my-marketplace` com `Status: ✔ enabled`.

    Para ver o que o plugin carregou, mostre seus detalhes.

    ```bash theme={null}
    claude plugin details my-first-plugin
    ```

    A seção `Component inventory` lê `Skills (1)  hello`.

    Para executar a skill, inicie uma sessão e digite `/my-first-plugin:hello`. Claude o cumprimenta. O comando tem o nome do plugin como prefixo, como o nome de toda skill de plugin faz.
  </Step>
</Steps>

<h2 id="add-plugin-entries">
  Adicionar entradas de plugin
</h2>

Cada plugin que você distribui é um objeto no array `plugins` de `marketplace.json`. Para adicionar um segundo plugin, adicione um segundo objeto. Estes campos cobrem a maioria das entradas:

* `name`: o identificador que as pessoas digitam antes de `@` quando instalam. Não pode conter espaços.
* `source`: onde Claude Code busca o plugin. Escreva uma string de caminho relativo para um plugin dentro do diretório do marketplace, como no [passo a passo](#create-a-marketplace), ou um objeto de source para um plugin fora dele. Veja [Escolher uma source de plugin](#choose-a-plugin-source).
* `description`: a linha que as pessoas veem ao lado do plugin quando navegam seu marketplace em `/plugin`.

Para a lista completa de campos, veja [Entradas de plugin](/docs/pt/plugins/marketplace-reference#plugin-entries).

Uma entrada também pode definir qualquer campo [`plugin.json`](/docs/pt/plugins/manifest-reference). Para quando os campos `plugin.json` de uma entrada se aplicam a um plugin que tem seu próprio `plugin.json`, veja [Entrada e plugin.json](/docs/pt/plugins/marketplace-reference#entry-and-plugin-json).

<h2 id="rules-for-plugin-entries">
  Regras para entradas de plugin
</h2>

A maioria das instalações falhadas de um novo marketplace vêm de um caminho relativo escrito a partir do diretório errado, ou de um nome de entrada que difere do `name` no `plugin.json` do plugin.

<h3 id="write-relative-paths-from-the-marketplace-root">
  Escrever caminhos relativos a partir da raiz do marketplace
</h3>

A raiz do marketplace é o diretório que contém `.claude-plugin/`. No [passo a passo](#create-a-marketplace), isso é `my-marketplace/`, então a `source` da entrada é `"./plugins/my-first-plugin"`. O caminho não começa dentro de `.claude-plugin/`, então não use `..` para sair dele.

Um caminho com `..` e um caminho para um diretório ausente falham em comandos diferentes:

* **Um caminho com `..`**: `claude plugin validate` relata a entrada como inválida. A mensagem começa com `Path contains "..": ./../plugins/my-first-plugin`.
* **Um caminho para um diretório que não existe**: `claude plugin validate` passa. `claude plugin install` falha com `Source path does not exist: <path>`, e `<path>` é o local absoluto que Claude Code verificou.

<h3 id="keep-the-entry-name-and-the-manifest-name-the-same">
  Manter o nome da entrada e o nome do manifesto iguais
</h3>

Um plugin do marketplace tem um `name` de entrada em `marketplace.json` e um `name` em seu próprio `plugin.json`, chamado de nome do manifesto. Cada nome aparece em lugares diferentes:

* **Nome da entrada**: o id de instalação, `<entry-name>@<marketplace>`. É o que as pessoas digitam para instalar, o que `claude plugin list` mostra, e a chave que Claude Code escreve sob [`enabledPlugins`](/docs/pt/settings-reference#enabledplugins) em seu arquivo de configurações.
* **Nome do manifesto**: o prefixo nas skills do plugin, e o nome que `claude plugin details` recebe.

Quando os dois nomes diferem e alguém instala pelo nome do manifesto, Claude Code relata `Plugin "<manifest-name>" not found in marketplace "<marketplace>"`. Mantenha os dois nomes iguais. Para mais sobre como Claude Code usa os dois nomes, veja [Referência de carregamento de plugin](/docs/pt/plugins/loading#find-where-a-plugin-came-from).

<h2 id="choose-a-plugin-source">
  Escolher uma source de plugin
</h2>

Cada entrada de plugin em `marketplace.json` tem uma `source` que diz ao Claude Code onde buscar esse plugin. Escolha a source por onde os arquivos do plugin são armazenados. A tabela lista as sources que a maioria dos proprietários de marketplace usam.

| Source           | Use quando                                                              | Valor mínimo de `source`                                                                  |
| :--------------- | :---------------------------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| Caminho relativo | Os arquivos do plugin estão dentro do próprio diretório do marketplace  | `"./plugins/my-first-plugin"`                                                             |
| `github`         | O plugin é seu próprio repositório GitHub                               | `{ "source": "github", "repo": "your-org/my-first-plugin" }`                              |
| `git-subdir`     | O plugin é um subdiretório de algum outro repositório, como um monorepo | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

Em uma source `git-subdir`, `url` recebe uma URL git ou um atalho GitHub `owner/repo`.

Um plugin também pode vir de um destes tipos de source:

* `url`: um repositório git por URL, em qualquer host
* `archive`: um arquivo zip baixado via HTTPS
* `npm`: um pacote npm
* `command`: um diretório produzido pela execução de um comando na máquina onde o plugin é instalado

Para os campos de cada tipo de source, e para fixar uma source baseada em git a um `ref` ou `sha`, veja [Plugin sources](/docs/pt/plugins/marketplace-reference#plugin-sources).

<h2 id="validate-and-test">
  Validar e testar
</h2>

Conforme você adiciona plugins, execute `claude plugin validate ./my-marketplace` em seu shell após cada edição, e instale a partir do marketplace em sua própria máquina antes de compartilhá-lo. Validação e instalação capturam problemas diferentes.

<h3 id="problems-that-validation-reports">
  Problemas que a validação relata
</h3>

`claude plugin validate` lê apenas arquivos dentro do diretório do marketplace. Relata:

* Erros de sintaxe JSON, como `json: Invalid JSON syntax: <reason>`
* Campos obrigatórios ausentes, como `owner: Invalid input`
* Um nome de marketplace com espaços, caracteres não-ASCII, ou uma forma que imita um marketplace oficial da Anthropic, como `claude-official`
* Uma `source` relativa que contém `..`
* Campos desconhecidos no nível superior ou em uma entrada de plugin, como avisos
* Problemas no `plugin.json` de cada plugin de caminho relativo, como `plugins[N] plugin.json → <field>: <message>`

Para cada mensagem que `validate` pode imprimir, veja [Mensagens de validação](/docs/pt/plugins/marketplace-reference#validation-messages). Para seus flags e códigos de saída, veja [`plugin validate`](/docs/pt/plugins/cli-reference#plugin-validate).

<h3 id="problems-that-surface-when-you-add-or-install">
  Problemas que aparecem quando você adiciona ou instala
</h3>

Problemas que `claude plugin validate` não relata aparecem quando você adiciona o marketplace ou instala a partir dele:

* **Quando você adiciona o marketplace**: os [nomes de marketplace oficiais](/docs/pt/plugins/marketplace-reference#reserved-names) exatos, como `claude-plugins-official`, passam na validação. Quando você adiciona um marketplace com um desses nomes, Claude Code o recusa com uma mensagem que começa com `The name '<name>' is reserved for official Anthropic marketplaces`.
* **Quando você instala um plugin**:
  * Claude Code primeiro busca uma source `github`, `git-subdir` ou outra remota quando você instala o plugin, então um `repo` ou `path` errado aparece então.
  * Uma `source` relativa cujo diretório não existe também falha na instalação, com `Source path does not exist: <path>`.

<h3 id="test-an-edit-to-a-plugin">
  Testar uma edição em um plugin
</h3>

No [passo a passo](#create-a-marketplace), você adicionou `my-marketplace` a partir de um diretório local com uma `source` de caminho relativo. Com essa configuração, Claude Code lê os arquivos do plugin diretamente de `my-marketplace/plugins/`. Suas edições entram em vigor no próximo início de sessão ou quando você executa `/reload-plugins` em uma sessão, sem alteração na `version` do plugin.

As pessoas que instalam a partir de seu marketplace hospedado recebem uma cópia no cache de plugins em vez disso. Para como elas recebem uma nova versão, veja [Manter usuários atualizados](/docs/pt/plugins/host-marketplace#keep-users-up-to-date).

<h3 id="remove-the-marketplace-to-start-over">
  Remover o marketplace para começar novamente
</h3>

Para remover tudo e começar novamente, execute `claude plugin marketplace remove my-marketplace` em seu shell. O comando remove o marketplace e desinstala seus plugins.

<h2 id="host-your-marketplace">
  Hospedar seu marketplace
</h2>

Uma vez que você possa instalar um plugin a partir do marketplace em sua própria máquina, como em [Criar um marketplace](#create-a-marketplace), envie o diretório do marketplace para um host git.

Seus colegas de equipe então executam `claude plugin marketplace add <owner>/<repo>` em seu shell para um repositório GitHub, ou o mesmo comando com a URL do repositório. Eles então instalam um plugin por nome como no [passo a passo](#create-a-marketplace).

Para acesso a repositório privado, atualizações, versionamento e renomeação ou remoção de entradas, veja [Hospedar e manter um marketplace](/docs/pt/plugins/host-marketplace).

<h2 id="next-steps">
  Próximos passos
</h2>

* [Hospedar e manter um marketplace](/docs/pt/plugins/host-marketplace): escolha um host, mantenha usuários atualizados e renomeie ou remova plugins com segurança
* [Referência de marketplace](/docs/pt/plugins/marketplace-reference): campos `marketplace.json` e tipos de source
* [Gerenciar plugins para sua organização](/docs/pt/plugins/org): exija seu marketplace e seus plugins em cada máquina
* [Sugerir plugins por relevância](/docs/pt/plugins/relevance): faça Claude Code sugerir um plugin do seu marketplace quando uma sessão corresponder
