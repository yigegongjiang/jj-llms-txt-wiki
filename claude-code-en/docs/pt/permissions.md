> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar permissões

> Controle o que Claude Code pode acessar e fazer com regras de permissão refinadas, modos e políticas gerenciadas.

Claude Code suporta permissões refinadas para que você possa especificar exatamente o que o agente pode fazer e o que não pode. Você pode verificar as configurações de permissão no controle de versão para compartilhá-las com todos os desenvolvedores da sua organização, e cada desenvolvedor pode personalizar as suas próprias.

<h2 id="permission-system">
  Sistema de permissões
</h2>

Claude Code usa um sistema de permissões em camadas para equilibrar poder e segurança. A tabela mostra, para cada tipo de ferramenta, se o modo Manual pede aprovação antes da ação ser executada. Os outros [modos de permissão](#permission-modes) alteram quais desses pedem sua aprovação; no modo automático um classificador revisa as ações em vez de você, e [como o classificador avalia ações](/docs/pt/permission-modes#how-the-classifier-evaluates-actions) lista quais ele vê.

| Tipo de ferramenta     | Exemplo                   | Aprovação necessária                                                                                                      | Comportamento de "Sim, não pergunte novamente" |
| :--------------------- | :------------------------ | :------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------- |
| Somente leitura        | Leitura de arquivos, Grep | Não, dentro do [diretório de trabalho e diretórios adicionais](#working-directories)                                      | N/A                                            |
| Comandos Bash          | Execução de shell         | Sim, exceto um conjunto integrado de [comandos somente leitura](#read-only-commands)                                      | Permanentemente por repositório e comando      |
| Modificação de arquivo | Edit/Write de arquivos    | Sim                                                                                                                       | Até o final da sessão                          |
| Busca na web           | WebFetch                  | Sim, exceto um conjunto integrado de [domínios de documentação pré-aprovados](/docs/pt/tools-reference#webfetch-tool-behavior) | Permanentemente por repositório e domínio      |
| Pesquisa na web        | WebSearch                 | Sim                                                                                                                       | Permanentemente por repositório                |

Quando você escolhe "Sim, não pergunte novamente" e a aprovação é salva permanentemente, como para um comando Bash ou um domínio WebFetch, Claude Code salva a regra em `.claude/settings.local.json` na raiz do repositório git, resolvida através de [worktrees](/docs/pt/worktrees) para o checkout principal. A regra se aplica a futuras sessões em qualquer lugar desse repositório, incluindo sessões iniciadas em subdiretórios e em worktrees. Uma aprovação de modificação de arquivo não é salva no arquivo: como a tabela mostra, ela dura até o final da sessão. Em alguns casos, como fora de um repositório git ou no Windows, Claude Code não usa a raiz do repositório; [Onde Claude Code procura por cada arquivo](/docs/pt/settings#where-claude-code-looks-for-each-file) lista esses casos e onde ele salva a regra em vez disso.

Antes da v2.1.211, Claude Code sempre salvava a regra no diretório inicial, então uma aprovação concedida em uma worktree ou subdiretório não se aplicava ao resto do repositório. Regras que versões anteriores salvaram em um subdiretório ou worktree ainda se aplicam a sessões iniciadas lá.

Às vezes um prompt de permissão oferece apenas uma aprovação única, sem opção de "não pergunte novamente" e sem opção de permitir a ação pelo resto da sessão. Claude Code oferece essas opções apenas quando o prompt pode mostrar a você tudo o que elas permitiriam, então uma regra que você salva de um prompt cobre apenas o que sua opção nomeada. Quando um prompt oferece apenas a aprovação única, aprove a ação uma vez, ou adicione a regra você mesmo em [`/permissions`](#manage-permissions).

<h3 id="add-a-comment-when-you-answer-a-permission-prompt">
  Adicione um comentário quando você responder a um prompt de permissão
</h3>

Você pode anexar uma nota a Claude quando aprova ou nega uma ação única. Na maioria dos prompts de permissão, incluindo Bash, PowerShell, arquivo e prompts de ferramenta MCP, mova para **Sim** ou **Não** e pressione `Tab` para abrir um campo de comentário nessa opção. Prompts de WebFetch e navegador não oferecem o campo. As opções que permitem a ação pelo resto da sessão ou salvam uma regra também não aceitam uma.

Com o campo aberto, digite o comentário e depois pressione uma dessas teclas:

* `Enter`: envia sua resposta com o comentário anexado. Se você deixar o campo vazio, Claude Code envia a resposta sem um comentário.
* `Tab`: fecha o campo sem responder. Claude Code mantém o texto que você digitou e ainda o envia se você responder com essa opção.
* `Shift+Tab`: em um prompt de arquivo, como um prompt Edit ou Write, fecha o campo da mesma forma que `Tab`. Antes da v2.1.235, pressionar `Shift+Tab` dentro do campo em vez disso selecionava a opção que permite a ação pelo resto da sessão, então Claude Code aprovava a ação pelo resto da sessão e descartava o comentário.

Claude Code entrega o comentário de forma diferente dependendo de como você respondeu:

* **Sim**: Claude Code executa a ação, depois envia seu comentário a Claude após o resultado.
* **Não**: Claude Code envia seu comentário a Claude como o motivo da negação, e Claude continua trabalhando. Se você selecionar **Não** sem um comentário em um prompt da conversa principal, Claude Code interrompe a rodada.

<h2 id="manage-permissions">
  Gerenciar permissões
</h2>

Você pode visualizar e gerenciar as permissões de ferramentas do Claude Code com `/permissions`. Esta interface lista todas as regras de permissão e o arquivo `settings.json` do qual cada regra é originária. Você pode abrir a interface enquanto Claude está trabalhando: quando você adiciona ou remove uma regra, Claude Code aplica a alteração começando com a próxima chamada de ferramenta do Claude no mesmo turno. Antes da v2.1.234, Claude Code enfileirava o comando até que o turno terminasse.

* As regras **Allow** permitem que Claude Code use a ferramenta especificada sem aprovação manual.
* As regras **Ask** solicitam confirmação sempre que Claude Code tenta usar a ferramenta especificada.
* As regras **Deny** impedem que Claude Code use a ferramenta especificada.

As regras são avaliadas em ordem: deny, depois ask, depois allow. A primeira correspondência nessa ordem determina o resultado, e a especificidade da regra não altera a ordem.

Uma regra deny ampla como `Bash(aws *)` bloqueia cada chamada correspondente, incluindo chamadas que também correspondem a uma regra allow mais estreita como `Bash(aws s3 ls)`. Uma regra allow não pode criar uma exceção de uma regra deny. A mesma precedência se aplica entre ask e allow: uma regra ask correspondente solicita confirmação mesmo quando uma regra allow mais específica também corresponde à mesma chamada.

As regras deny se comportam de forma diferente dependendo se nomeiam uma ferramenta ou definem o escopo de um padrão dentro de uma. Um nome de ferramenta simples como `Bash` remove a ferramenta do contexto do Claude completamente, então Claude nunca a vê. Se você adicionar tal regra no meio da sessão, Claude não pode chamar a ferramenta a partir de sua próxima chamada de ferramenta em diante; [Negando uma ferramenta inteira](/docs/pt/prompt-caching#denying-an-entire-tool) cobre o que acontece com uma definição que Claude já viu. Uma regra com escopo como `Bash(rm *)` deixa a ferramenta disponível e bloqueia chamadas correspondentes quando Claude tenta usá-las.

A remoção com nome simples se aplica a todas as ferramentas exceto [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior): uma regra deny não pode removê-la enquanto qualquer outra ferramenta permanecer, e uma regra ask nunca solicita confirmação para ela.

<Note>
  As regras de permissão são aplicadas pelo Claude Code, não pelo modelo. As instruções em seu prompt ou `CLAUDE.md` moldam o que Claude tenta fazer, mas não alteram o que Claude Code permite. Para conceder ou revogar acesso, use `/permissions`, as regras descritas aqui, um [modo de permissão](/docs/pt/permission-modes), ou um [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

Quando o [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) está disponível para sua sessão, a interface também inclui as [regras do classificador de modo automático](/docs/pt/auto-mode-config#edit-rules-from-permissions). Selecione a aba **Auto mode** para visualizá-las.

<h2 id="permission-modes">
  Modos de permissão
</h2>

Claude Code suporta vários modos de permissão que controlam como ele aprova chamadas de ferramentas. Veja [Modos de permissão](/docs/pt/permission-modes) para quando usar cada um. Para alterar o modo em que as sessões começam, defina `defaultMode` em seus [arquivos de configuração](/docs/pt/settings#where-settings-live). [Qual modo uma sessão começa](/docs/pt/permission-modes#which-mode-a-session-starts-in) cobre o padrão integrado para cada plano e o que a extensão VS Code lê.

| Modo                | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Solicita permissão no primeiro uso de cada ferramenta. Rotulado como Manual na CLI, nas extensões VS Code e JetBrains, e no aplicativo desktop, e Claude Code aceita `manual` como um alias. O rótulo e o alias requerem Claude Code v2.1.200 ou posterior. O rótulo do aplicativo desktop não depende da sua versão da CLI                                                                                                                                                                                                                                                                                                   |
| `acceptEdits`       | Aceita automaticamente edições de arquivo e comandos comuns do sistema de arquivos como `mkdir`, `touch`, `mv` e `cp` para caminhos no diretório de trabalho ou `additionalDirectories`                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `plan`              | Claude lê arquivos e executa comandos shell somente leitura para explorar, mas não edita seus arquivos de origem; com [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) disponível, comandos aprovados pelo classificador também são executados. Rotulado como Plan na CLI e na extensão VS Code                                                                                                                                                                                                                                                                                                       |
| `auto`              | Aprova automaticamente chamadas de ferramentas com verificações de segurança em segundo plano que verificam se as ações se alinham com sua solicitação                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `dontAsk`           | Nega automaticamente toda chamada que de outra forma solicitaria permissão; leituras de arquivo em seus diretórios de trabalho e outras ações que não precisam de aprovação ainda são executadas, assim como ferramentas pré-aprovadas via `/permissions` ou regras `permissions.allow`. `AskUserQuestion`, ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) e ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) em sessões onde essa configuração chega ao Claude Code são negadas mesmo se você as permitiu |
| `bypassPermissions` | Ignora prompts de permissão, exceto pelas [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<Warning>
  No modo `bypassPermissions`, Claude Code ignora prompts de permissão, incluindo escritas em [caminhos protegidos](/docs/pt/permission-modes#protected-paths) como `.git` e `.claude`. As [proteções de mensagens entre sessões](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode) ainda se aplicam. Use este modo apenas em ambientes isolados como contêineres ou VMs onde Claude Code não pode causar danos.
</Warning>

Para evitar que o modo `bypassPermissions` ou `auto` seja usado, defina `permissions.disableBypassPermissionsMode` ou `permissions.disableAutoMode` como `"disable"` em qualquer [arquivo de configuração](/docs/pt/settings#where-settings-live). Estes são mais úteis em [configurações gerenciadas](#managed-settings) onde não podem ser substituídos.

<h2 id="permission-rule-syntax">
  Sintaxe de regra de permissão
</h2>

As regras de permissão seguem o formato `Tool` ou `Tool(specifier)`. Parênteses dentro do especificador são literais, portanto um comando ou caminho que os contém não precisa de escape.

<h3 id="match-all-uses-of-a-tool">
  Corresponder todos os usos de uma ferramenta
</h3>

Para corresponder todos os usos de uma ferramenta, use apenas o nome da ferramenta sem parênteses:

| Regra      | Efeito                                              |
| :--------- | :-------------------------------------------------- |
| `Bash`     | Corresponde a todos os comandos Bash                |
| `WebFetch` | Corresponde a todas as solicitações de busca na web |
| `Read`     | Corresponde a todas as leituras de arquivo          |

`Bash(*)` é equivalente a `Bash` e corresponde a todos os comandos Bash. Como uma regra de negação, ambas as formas removem a ferramenta do contexto do Claude.

<h3 id="use-specifiers-for-fine-grained-control">
  Use especificadores para controle refinado
</h3>

Adicione um especificador entre parênteses para corresponder a usos específicos de ferramentas:

| Regra                          | Efeito                                                     |
| :----------------------------- | :--------------------------------------------------------- |
| `Bash(npm run build)`          | Corresponde ao comando exato `npm run build`               |
| `Read(./.env)`                 | Corresponde à leitura do arquivo `.env` no diretório atual |
| `WebFetch(domain:example.com)` | Corresponde a solicitações de busca para example.com       |

<h3 id="match-by-input-parameter">
  Corresponder por parâmetro de entrada
</h3>

As regras de negação e pergunta podem corresponder a um parâmetro de entrada de nível superior em qualquer ferramenta integrada com `Tool(param:value)`.

Para corresponder a um parâmetro em uma ferramenta MCP, passe uma regra de negação com [`--disallowedTools`](/docs/pt/cli-reference#cli-flags). Quando Claude Code carrega um arquivo de configurações, ele ignora qualquer regra `mcp__` que tenha parênteses. Claude Code lista a regra ignorada no diálogo de configurações inválidas quando uma sessão interativa é iniciada, e na saída de [`claude doctor`](/docs/pt/debug-your-config#check-resolved-settings).

Uma regra de parâmetro corresponde quando Claude chama a ferramenta com esse parâmetro definido para esse valor exato. Uma regra de permissão para um valor de parâmetro não estabeleceria que a chamada é segura em geral, portanto as regras de permissão continuam a usar a sintaxe de especificador própria de cada ferramenta. Isso funciona para qualquer parâmetro escalar que a ferramenta aceita:

| Regra                          | Corresponde                                            |
| :----------------------------- | :----------------------------------------------------- |
| `Agent(model:opus)`            | Chamadas de Agent que solicitam o nível de modelo Opus |
| `Agent(isolation:worktree)`    | Chamadas de Agent que solicitam um git worktree        |
| `Bash(run_in_background:true)` | Chamadas Bash que executam em segundo plano            |

A correspondência de parâmetros segue estas regras:

* O nome do parâmetro deve ser um campo direto da entrada da ferramenta, como `model` na ferramenta Agent. Campos aninhados dentro de um objeto ou array não são correspondíveis
* Cada regra nomeia um parâmetro. Para controlar tanto `model` quanto `isolation`, escreva duas regras, `Agent(model:opus)` e `Agent(isolation:worktree)`, em vez de combiná-las em uma regra
* O valor suporta `*` como um caractere curinga que corresponde a qualquer sequência de caracteres, portanto `Agent(isolation:*)` corresponde a qualquer valor de isolamento explícito. Sem `*` a correspondência é exata
* Um parâmetro que o modelo omite nunca é correspondido, portanto `Agent(model:*)` não corresponde a uma chamada que deixa `model` não definido
* O valor é comparado com a entrada literal que Claude envia, antes de qualquer normalização. `Agent(model:opus)` corresponde ao alias `opus` mas não a um ID de modelo completo. Execute com [`--verbose`](/docs/pt/cli-reference) para ver os nomes e valores exatos dos parâmetros em cada chamada de ferramenta
* O espaço em branco ao redor do dois-pontos é ignorado

Você não pode corresponder a um campo de conteúdo primário de uma ferramenta desta forma: `command` para Bash e PowerShell, `file_path` para Read, Edit e Write, `path` para Grep e Glob, `notebook_path` para NotebookEdit, e `url` para WebFetch. Uma regra como `Bash(command:rm *)` seria contornável por um comando composto, portanto Claude Code a ignora e emite um aviso de inicialização. Use `Bash(rm *)`, `Read(./path)` ou `WebFetch(domain:host)` em vez disso.

<h3 id="wildcard-patterns">
  Padrões com caracteres curinga
</h3>

Um `*` em uma regra Bash corresponde a qualquer texto, incluindo espaços, portanto uma regra cobre uma família de comandos. Uma regra sem `*` corresponde a um comando exato.

<Warning>
  Coloque o `*` após o subcomando. Em `git log --oneline main`, `git` é o programa e `log` é o subcomando, a palavra que determina o que o programa faz. Claude Code corresponde a tudo antes do primeiro `*` como escrito, portanto essas palavras são o que limitam a regra: `Bash(git log *)` permite apenas comandos `git log`, e `Bash(git *)` permite todos os comandos git. Claude Code [avisa na inicialização](/docs/pt/errors#has-a-wildcard-before-the-rest-of-the-command) sobre uma regra de permissão com um `*` antes do subcomando, como `Bash(git * main)`.
</Warning>

Escreva o comando que você quer que Claude execute sem perguntar, e substitua as partes que variam com `*`. Com esta configuração, Claude Code executa scripts npm e commits git sem perguntar e recusa comandos que começam com `git push`. Um push escrito de outra forma, como `git -C . push`, não é correspondido; veja [o que uma regra Bash não corresponde](#bash-rule-limits).

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

Um `*` pode ir em qualquer lugar na regra: no início, no meio ou no final. Cada linha mostra uma regra, comandos que ela corresponde e comandos próximos que ela não corresponde:

| Você escreve           | Corresponde                                                                          | Não corresponde                        |
| :--------------------- | :----------------------------------------------------------------------------------- | :------------------------------------- |
| `Bash(npm run build)`  | `npm run build`                                                                      | `npm run build --watch`                |
| `Bash(npm run *)`      | `npm run build`, `npm run test --watch`, `npm run`                                   | `npm install`                          |
| `Bash(git log * main)` | `git log --oneline main`, `git log -5 main`, `git log --output=<file> main`          | `git log main`, `git push origin main` |
| `Bash(git * main)`     | `git merge main`, `git push origin main`, `git -c core.fsmonitor=<script> diff main` | `git log`                              |
| `Bash(* --version)`    | `node --version`, `bash -c 'echo hi' --version`                                      | `node -v`                              |
| `Bash(ls *)`           | `ls -la`, `ls`                                                                       | `lsof`                                 |
| `Bash(ls*)`            | `ls -la`, `lsof`                                                                     |                                        |
| `Bash(* --help *)`     | `npm --help x`                                                                       | `npm --help`                           |

Três regras de correspondência produzem essas linhas:

* **O `*` representa qualquer texto em seu lugar.** Em `Bash(git * main)`, ele representa o subcomando, portanto Claude Code corresponde a todos os subcomandos git e todas as opções antes dele. Isso inclui `-c`, que faz git executar um programa que você nomeia. Em `Bash(* --version)`, o `*` representa o programa, portanto qualquer programa corresponde.
* **Um `*` no final, com um espaço antes dele, também corresponde ao comando simples.** `Bash(ls *)` corresponde a `ls`, e `Bash(git log *)` corresponde a `git log`. Isso vale apenas quando o `*` à direita é o único caractere curinga da regra: `Bash(* --help *)` corresponde a `npm --help x` mas não a `npm --help`.
* **O espaço antes de um `*` à direita faz parte da regra.** `Bash(ls *)` requer um espaço após `ls`, portanto `lsof` não corresponde. `Bash(ls*)` não tem espaço, portanto também corresponde a `lsof`.

O sufixo `:*` é uma maneira equivalente de escrever um caractere curinga à direita, portanto `Bash(ls:*)` corresponde aos mesmos comandos que `Bash(ls *)`.

O diálogo de permissão escreve a forma separada por espaço quando você seleciona "Sim, não pergunte novamente" para um prefixo de comando. A forma `:*` é reconhecida apenas no final de um padrão. Em um padrão como `Bash(git:* push)`, o dois-pontos é tratado como um caractere literal e não corresponderá a comandos git.

<h3 id="tool-name-wildcards">
  Caracteres curinga no nome da ferramenta
</h3>

As regras de negação e pergunta também aceitam padrões glob na posição do nome da ferramenta. O padrão deve corresponder ao nome completo da ferramenta: `"*"` corresponde a todas as ferramentas, e `"mcp__*"` corresponde a todas as ferramentas MCP em todos os servidores. Uma ferramenta correspondida por uma regra de negação glob com nome simples é removida do contexto do Claude, o mesmo que um nome de ferramenta simples, incluindo a exceção [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior): uma negação glob não pode removê-la enquanto qualquer outra ferramenta permanecer, e uma pergunta glob nunca a solicita. Esta configuração nega todas as ferramentas MCP:

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

As regras de permissão aceitam globs de nome de ferramenta apenas após um prefixo literal `mcp__<server>__`. O segmento do servidor deve estar livre de glob para que a regra nomeie um servidor específico que você configurou. `mcp__puppeteer__*` corresponde a todas as ferramentas do servidor `puppeteer`, e `mcp__github__get_*` corresponde às suas ferramentas `get_`. Um glob de permissão desancorado como `"*"`, `"B*"` ou `"mcp__*"` é ignorado com um aviso e não aprova automaticamente nada.

Uma regra de negação ou pergunta cujo nome de ferramenta não corresponde a nenhuma ferramenta conhecida produz um aviso de inicialização para detectar erros de digitação. Nomes de ferramentas contendo `_` ou `*` estão isentos da verificação, e também estão os nomes de ferramentas que Claude Code removeu, como `TaskOutput`.

O rótulo mostrado para uma ferramenta na transcrição e diálogo de permissão pode diferir do seu nome canônico. Por exemplo, a ferramenta rotulada `Stop Task` na transcrição tem o nome canônico `TaskStop`. As regras de permissão e [correspondências de hook](/docs/pt/hooks) não correspondem ao rótulo, portanto uma regra escrita como `Stop Task` não corresponde. Para regras de negação e pergunta, o aviso de inicialização acima detecta a incompatibilidade. Use os nomes canônicos listados na [referência de ferramentas](/docs/pt/tools-reference).

<h2 id="tool-specific-permission-rules">
  Regras de permissão específicas da ferramenta
</h2>

<h3 id="bash">
  Bash
</h3>

As regras Bash correspondem ao texto do comando inteiro, com `*` representando qualquer texto. [Padrões com caracteres curinga](#wildcard-patterns) mostra quais comandos cada forma de regra corresponde e onde colocar o `*`. O resto desta seção cobre como Claude Code corresponde a comandos compostos e wrappers, o que uma regra não corresponde, comandos somente leitura e redirecionamentos.

<h4 id="compound-commands">
  Comandos compostos
</h4>

<Tip>
  Claude Code está ciente de operadores de shell, portanto uma regra como `Bash(safe-cmd *)` não lhe dará permissão para executar o comando `safe-cmd && other-cmd`. Os separadores de comando reconhecidos são `&&`, `||`, `;`, `|`, `|&`, `&` e quebras de linha. Uma regra deve corresponder a cada subcomando independentemente.
</Tip>

As regras deny e ask se aplicam quando qualquer subcomando as corresponde, incluindo um comando aninhado dentro de um subshell, uma substituição de comando ou um corpo de fluxo de controle como um loop `for`. Uma regra ask como `Bash(git clean *)` ainda solicita para `cd /tmp && git clean -f` ou `echo "$(git clean -f)"`, mesmo em [modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode).

Quando `&&` ou `||` não tem nada depois, como em `npm test &&`, Claude Code trata o comando como não analisável e não o divide em subcomandos para correspondência de regra allow, portanto uma regra como `Bash(npm *)` não o aprova.

Quando você aprova um comando composto com "Sim, não pergunte novamente", Claude Code salva uma regra separada para cada subcomando que requer aprovação, em vez de uma única regra para a string completa. Por exemplo, aprovar `git status && npm test` salva uma regra para `npm test`, portanto futuras invocações de `npm test` são reconhecidas independentemente do que precede o `&&`. Subcomandos como `cd` em um subdiretório geram sua própria regra Read para esse caminho. Até 5 regras podem ser salvas para um único comando composto.

<h4 id="process-wrappers">
  Wrappers
</h4>

Antes de corresponder regras Bash, Claude Code remove um conjunto fixo de wrappers, portanto uma regra como `Bash(npm test *)` também corresponde a `timeout 30 npm test`. Os wrappers removidos são `timeout`, `time`, `nice`, `nohup` e `stdbuf`, mais os shell builtins `command` e `builtin`, e `noglob` do zsh. Cada um executa seu argumento como o comando real. Duas formas relacionadas não são removidas: a forma de consulta `command -v`, que procura um comando em vez de executá-lo, e `nocorrect` do zsh.

Claude Code também remove uma atribuição inicial de certas variáveis de ambiente conhecidas como seguras, portanto `Bash(npm test *)` corresponde a `NODE_ENV=test npm test`. Uma regra allow não corresponderá além de uma atribuição de qualquer outra variável. Uma regra deny ou ask corresponde além de qualquer atribuição inicial, portanto `Bash(rm *)` em deny ainda corresponde a `FOO=bar rm -rf tmp/`.

`xargs` simples também é removido, portanto `Bash(grep *)` corresponde a `xargs grep pattern`. A remoção se aplica apenas quando `xargs` não tem flags: uma invocação como `xargs -n1 grep pattern` é correspondida como um comando `xargs`, portanto regras escritas para o comando interno não a cobrem.

Esta lista de wrapper é integrada e não é configurável. Executores de ambiente de desenvolvimento como `direnv exec`, `devbox run`, `mise exec`, `npx` e `docker exec` não estão na lista. Porque essas ferramentas executam seus argumentos como um comando, uma regra como `Bash(devbox run *)` corresponde a tudo que vem após `run`, incluindo `devbox run rm -rf .`. Para aprovar trabalho dentro de um executor de ambiente, escreva uma regra específica que inclua tanto o executor quanto o comando interno, como `Bash(devbox run npm test)`. Adicione uma regra por comando interno que você quer permitir.

Wrappers exec como `watch`, `setsid`, `ionice` e `flock` não podem ser auto-aprovados por uma regra de prefixo como `Bash(watch *)`, portanto em modo Manual sempre solicitam. O mesmo se aplica a `find` com `-exec` ou `-delete`: uma regra `Bash(find *)` não cobre essas formas. Para aprovar uma invocação específica, escreva uma regra de correspondência exata para a string de comando completa.

<h4 id="bash-rule-limits">
  O que uma regra Bash não corresponde
</h4>

Uma regra Bash corresponde ao texto do comando que Claude escreve, após Claude Code dividir [comandos compostos](#compound-commands) e remover [wrappers](#process-wrappers). Ela não corresponde ao mesmo programa invocado de uma forma diferente, portanto uma regra deny ou ask cobre a invocação que Claude geralmente produz e não é um limite de segurança ao redor do programa. Estas regras em `deny` ou `ask` param a primeira forma e não as outras:

| Regra              | Impede                     | Não impede                                                                                            |
| :----------------- | :------------------------- | :---------------------------------------------------------------------------------------------------- |
| `Bash(curl *)`     | `curl https://example.com` | `/usr/bin/curl https://example.com`, `sh -c 'curl https://example.com'`                               |
| `Bash(rm *)`       | `rm -rf build/`            | `/bin/rm -rf build/`, `bash -c 'rm -rf build/'`                                                       |
| `Bash(git push *)` | `git push origin main`     | `git -C . push origin main`, `git -c push.default=current push origin main`, `git 'push' origin main` |

Suas outras regras e o modo de permissão decidem os comandos na última coluna.

Para imposição de sistema de arquivos e rede que não depende do texto do comando, use [sandboxing](/docs/pt/sandboxing). Para inspecionar o texto do comando completo com sua própria lógica antes de ser executado, use um [hook PreToolUse](#extend-permissions-with-hooks).

<h4 id="read-only-commands">
  Comandos somente leitura
</h4>

Claude Code reconhece um conjunto integrado de comandos Bash como somente leitura e os executa sem um prompt de permissão em cada modo, exceto por um caminho que [`permissions.blockReadsOutsideWorkingDirectories`](/docs/pt/settings-reference#permissions-blockreadsoutsideworkingdirectories) protege. O conjunto inclui `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` e formas somente leitura de `git`. O conjunto não é configurável; para exigir um prompt para um desses comandos, adicione uma regra `ask` ou `deny` para ele. Em modo auto, esses comandos também podem aguardar a revisão do classificador; veja [como o classificador avalia ações](/docs/pt/permission-modes#how-the-classifier-evaluates-actions).

Um redirecionamento como `ls > out.txt` adiciona uma verificação no alvo. Veja [Redirecionamentos](#redirections).

Padrões glob sem aspas são permitidos para comandos cujas todas as flags são somente leitura, portanto `ls *.ts` e `wc -l src/*.py` são executados sem um prompt.

Em modo Manual, comandos deste conjunto ainda solicitam nestes casos:

* **Globs sem aspas para comandos com flags capazes de escrita**: comandos com flags capazes de escrita ou execução, como `find`, `sort`, `sed` e `git`, solicitam quando um glob sem aspas está presente, porque o glob poderia expandir para uma flag como `-delete`.
* **`docker` apontado para outro daemon**: formas somente leitura de `docker` solicitam quando o comando carrega uma flag que seleciona um daemon diferente, como `-H`, `--context` ou `--url` e `--connection` do Podman.
* **`file` com flags de abertura de caminho**: `file` solicita quando passa `-m`/`--magic-file` ou `-f`/`--files-from`, porque essas flags fazem `file` abrir os caminhos nomeados no valor da flag.
* **Caminhos de rede no Windows**: um comando cujos argumentos incluem um caminho de rede (UNC), como `\\server\share\file`, solicita porque acessar um caminho de rede pode enviar suas credenciais do Windows para o host que ele nomeia. A mesma verificação se aplica a comandos da [ferramenta PowerShell](/docs/pt/tools-reference#powershell-tool).
* **Comandos que a análise não consegue analisar**: quando Claude Code não consegue analisar completamente um comando, solicita aprovação em vez de tratar o comando como somente leitura. Comandos mais longos que 10.000 caracteres sempre solicitam porque excedem o que a análise analisa.

Um `cd` em um caminho dentro do seu diretório de trabalho ou um [diretório adicional](#working-directories) também é somente leitura, e um comando composto como `cd packages/api && ls` é executado sem um prompt quando cada parte se qualifica por conta própria. Estas combinações solicitam mesmo quando cada parte é somente leitura:

* **`cd` com `git`**: solicita quando o `cd` muda para um diretório diferente, já que executar `git` em um novo diretório pode executar os hooks desse diretório. Um `cd` cujo alvo se resolve para o diretório de trabalho atual é uma no-op e não dispara o prompt.
* **`cd` com um redirecionamento**: solicita quando Claude Code não consegue determinar para qual diretório o alvo de redirecionamento se resolve após o `cd` ser executado. Um comando cujo único alvo de redirecionamento é `/dev/null`, como `cd app; grep -r pattern . 2>/dev/null`, não solicita, porque `/dev/null` não depende do diretório de trabalho.

<Warning>
  Padrões de permissão Bash que tentam restringir argumentos de comando são frágeis. Por exemplo, `Bash(curl http://github.com/ *)` pretende restringir curl a URLs do GitHub, mas não corresponderá a variações como:

  * Opções antes da URL: `curl -X GET http://github.com/...`
  * Protocolo diferente: `curl https://github.com/...`
  * Redirecionamentos: `curl -L http://short.example.com/xyz`, que redireciona para GitHub
  * Variáveis: `URL=http://github.com && curl $URL`

  Para filtragem de URL mais confiável, considere:

  * **Restringir ferramentas de rede Bash**: use regras deny para bloquear `curl`, `wget` e ferramentas similares, depois use a ferramenta WebFetch com permissão `WebFetch(domain:github.com)` para domínios permitidos. Uma regra deny não corresponde ao mesmo programa por caminho ou dentro de `sh -c`, portanto combine com a [lista de permissões de rede do sandbox](/docs/pt/sandboxing#network-isolation) quando a restrição deve ser mantida; veja [o que uma regra Bash não corresponde](#bash-rule-limits)
  * **Use hooks PreToolUse**: implemente um hook que valida URLs em comandos Bash e bloqueia domínios não permitidos
  * **Adicione orientação CLAUDE.md**: descreva seus padrões curl permitidos em `CLAUDE.md`. Isso molda o que Claude tenta mas não impõe um limite, portanto combine com uma das opções acima

  Observe que usar WebFetch sozinho não impede acesso à rede. Se Bash for permitido, Claude ainda pode usar `curl`, `wget` ou outras ferramentas para alcançar qualquer URL.
</Warning>

<h4 id="redirections">
  Redirecionamentos
</h4>

Quando um comando redireciona saída ou entrada, Claude Code verifica o alvo de redirecionamento contra suas regras de arquivo como se Claude tivesse escrito ou lido esse arquivo diretamente:

* **Redirecionamentos de saída**: para `> file`, `>> file` ou `2> file`, a verificação cobre suas regras allow e deny `Edit`, [caminhos protegidos](/docs/pt/permission-modes#protected-paths) e os [diretórios de trabalho](#working-directories). Uma regra como `Bash(git commit *)` permite o comando, não o alvo. Um alvo que começa com `~` ou contém um caractere curinga precisa de sua aprovação.
* **Redirecionamentos de entrada**: para `< file`, a verificação cobre suas regras allow e deny `Read` e os diretórios de trabalho. Um alvo fora dos diretórios de trabalho precisa de sua aprovação a menos que uma regra allow o cubra. Um alvo que contém um padrão glob ou um caminho relativo que segue um `cd` no mesmo comando precisa de sua aprovação mesmo quando uma regra allow o cobre. Claude Code verifica alvos de entrada em v2.1.257 e posterior.

Alvos sem arquivo atrás deles não são verificados: `/dev/null`, formas de descritor de arquivo como `2>&1` e `<&3`, e here-docs e here-strings.

Claude Code também verifica os arquivos que um comando `tee` escreve, incluindo em um pipeline como `make | tee build.log`. A verificação cobre suas regras allow e deny `Edit`, [caminhos protegidos](/docs/pt/permission-modes#protected-paths) e os [diretórios de trabalho](#working-directories). Uma regra allow como `Bash(tee *)` não cobre um destino fora dos diretórios de trabalho. Claude Code verifica alvos `tee` em v2.1.269 e posterior.

<h3 id="powershell">
  PowerShell
</h3>

As regras de permissão PowerShell usam a mesma forma que as regras Bash. Caracteres curinga com `*` correspondem em qualquer posição, o sufixo `:*` é equivalente a um ` *` final, e um `PowerShell` simples ou `PowerShell(*)` corresponde a cada comando. Esta configuração permite comandos `Get-ChildItem` e `git commit` enquanto bloqueia `Remove-Item`:

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

Aliases comuns são canonicalizados antes da correspondência. Uma regra escrita para o nome do cmdlet também corresponde a seus aliases, portanto `PowerShell(Get-ChildItem *)` corresponde a `gci`, `ls` e `dir` também. A correspondência é insensível a maiúsculas e minúsculas.

Claude Code analisa o AST do PowerShell e verifica cada comando em um comando composto independentemente. Os operadores de pipeline `|`, separadores de instrução `;` e nos operadores de cadeia PowerShell 7+ `&&` e `||` dividem um comando composto em subcomandos. Uma regra deve corresponder a cada subcomando para que o comando composto seja permitido.

<h3 id="read-and-edit">
  Read e Edit
</h3>

Para bloquear as ferramentas de arquivo do Claude de ler um arquivo ou diretório, adicione uma regra deny `Read` para seu caminho, como `Read(./.env)` ou `Read(./secrets/**)`. [Excluir arquivos sensíveis](/docs/pt/settings-reference#exclude-sensitive-files) tem um exemplo pronto para colar.

As regras `Edit` se aplicam a todas as ferramentas integradas que editam arquivos. Claude faz uma tentativa de melhor esforço para aplicar regras `Read` a todas as ferramentas integradas que leem arquivos como Grep e Glob, a menções `@file` em seus prompts, e à seleção e contexto de arquivo aberto que um [IDE](/docs/pt/vs-code#the-built-in-ide-mcp-server) conectado compartilha com Claude.

Uma regra deny `Read` também bloqueia as [ferramentas Edit e Write](/docs/pt/errors#file-is-covered-by-a-read-deny-rule) no mesmo caminho, incluindo criar um novo arquivo lá. NotebookEdit não é coberto, portanto adicione uma regra deny `Edit` para caminhos que nenhuma ferramenta pode alterar. A verificação requer Claude Code v2.1.208 ou posterior em edições, e v2.1.228 ou posterior em escritas.

Claude Code verifica permissões de arquivo apenas contra regras `Edit(path)` e `Read(path)`. Se você escrever uma regra de caminho para `Write`, `NotebookEdit`, `Glob` ou a ferramenta legada `MultiEdit` em vez disso, Claude Code aceita a regra mas nunca a consulta, e [avisa na inicialização](/docs/pt/errors#is-not-matched-by-file-permission-checks), exceto por uma regra `Glob` passada em `--allowedTools`. Use `Edit(docs/**)` no lugar de `Write(docs/**)`, `NotebookEdit(docs/**)` ou `MultiEdit(docs/**)`, e `Read(docs/**)` no lugar de `Glob(docs/**)`. Claude Code não avisa sobre uma regra de nome de ferramenta sem caminho, como uma regra deny para `Write`; ela corresponde a essa regra no nível de ferramenta em todos os lugares. Requer Claude Code v2.1.210 ou posterior.

<Warning>
  As regras deny de Read e Edit se aplicam às ferramentas de arquivo integradas do Claude, aos comandos de arquivo que Claude Code reconhece em Bash, como `cat`, `head`, `tail`, `sed` e `tee`, e aos alvos de [redirecionamentos](#redirections) Bash como `> file` e `< file`. Elas não se aplicam a um comando que lê arquivos sem nomeá-los, como `grep -r pattern .` executado a partir do diretório que contém o arquivo, ou a subprocessos arbitrários que leem ou escrevem arquivos indiretamente, como um script Python ou Node que abre arquivos por conta própria. Para imposição em nível de SO que bloqueia todos os processos de acessar um caminho, [ative o sandbox](/docs/pt/sandboxing).
</Warning>

As regras Read e Edit usam sintaxe de padrão [gitignore](https://git-scm.com/docs/gitignore) com quatro tipos de padrão distintos; para padrões de diretório de segmento único, a profundidade de correspondência também depende do tipo de regra, descrito mais adiante nesta seção:

| Padrão             | Significado                                     | Exemplo                          | Corresponde                                                                |
| ------------------ | ----------------------------------------------- | -------------------------------- | -------------------------------------------------------------------------- |
| `//path`           | Caminho absoluto da raiz do sistema de arquivos | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                                  |
| `~/path`           | Caminho do diretório home                       | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                                             |
| `/path`            | Caminho relativo à fonte de configurações       | `Edit(/src/**/*.ts)`             | `<diretório de trabalho primário>/src/**/*.ts` em configurações de projeto |
| `path` ou `./path` | Caminho relativo ao diretório atual             | `Read(*.env)`                    | `<cwd>/*.env`                                                              |

<Warning>
  Um padrão como `/Users/alice/file` não é um caminho absoluto. A barra inicial única ancora na fonte de configurações, não na raiz do sistema de arquivos. Use `//Users/alice/file` para caminhos absolutos.
</Warning>

Um padrão `/path` ancora em um diretório associado à fonte de configurações que o define, portanto a mesma regra corresponde a locais diferentes dependendo de onde você a coloca:

| Regra definida em                                     | `/path` se resolve para                 |
| :---------------------------------------------------- | :-------------------------------------- |
| Configurações de projeto em `.claude/settings.json`   | `<diretório de trabalho primário>/path` |
| Configurações locais em `.claude/settings.local.json` | `<diretório de trabalho primário>/path` |
| Configurações de usuário em `~/.claude/settings.json` | `~/.claude/path`                        |
| Um arquivo passado com `--settings <file>`            | `<diretório do arquivo>/path`           |
| Flags CLI ou regras de sessão                         | `<diretório de trabalho primário>/path` |

Uma regra que você adiciona através de `/permissions` segue a linha para o arquivo de configurações que você a salva.

As regras de configurações locais ancoram no [diretório de trabalho primário](#working-directories) da sessão, não na raiz do repositório onde Claude Code [armazena o arquivo](#permission-system) em v2.1.211 e posterior. Em uma sessão iniciada na raiz do repositório, os dois diretórios são os mesmos; em uma sessão de [worktree](/docs/pt/worktrees), uma regra compartilhada como `Edit(/src/**)` corresponde ao diretório `src/` próprio dessa worktree.

Uma regra deny como `Read(/secrets/**)` em configurações de usuário bloqueia `~/.claude/secrets/**`, não um diretório `secrets` em seu projeto. Para escrever uma regra em configurações de usuário que se aplique dentro de cada projeto, use um caminho absoluto `//` ou um caminho relativo à home `~/` em vez disso.

No Windows, os caminhos são normalizados para forma POSIX antes da correspondência. `C:\Users\alice` se torna `/c/Users/alice`, portanto use `//c/**/.env` para corresponder arquivos `.env` em qualquer lugar nessa unidade. Para corresponder em todas as unidades, use `//**/.env`.

Exemplos:

* `Edit(/docs/**)`: edita em `<diretório de trabalho primário>/docs/`, não `/docs/` ou `<diretório de trabalho primário>/.claude/docs/`
* `Read(~/.zshrc)`: lê o `.zshrc` do seu diretório home
* `Edit(//tmp/scratch.txt)`: edita o caminho absoluto `/tmp/scratch.txt`
* `Read(src/**)`: como uma regra allow, lê de `<diretório-atual>/src/` apenas; como uma regra deny ou ask, corresponde a um diretório `src` em qualquer profundidade sob o diretório atual

Uma regra só corresponde a arquivos sob sua âncora; dentro desse limite, a profundidade de correspondência depende da forma do padrão e, para padrões de diretório de segmento único, do tipo de regra, descrito abaixo. Nomes de arquivo simples seguem semântica gitignore e correspondem em qualquer profundidade, portanto `Read(.env)` e `Read(**/.env)` são equivalentes:

| Regra deny                      | Bloqueia                                                 | Não bloqueia                                            |
| ------------------------------- | -------------------------------------------------------- | ------------------------------------------------------- |
| `Read(.env)` ou `Read(**/.env)` | qualquer `.env` no ou sob o diretório atual              | `.env` em um diretório pai ou outro projeto             |
| `Read(//**/.env)`               | qualquer `.env` em qualquer lugar do sistema de arquivos | nada; a regra é ancorada na raiz do sistema de arquivos |

Um padrão relativo com um segmento de diretório único, como `src/**`, corresponde em profundidades diferentes dependendo do tipo de regra:

* **Regras allow**: `Edit(src/**)` corresponde apenas a `<cwd>/src` e aos arquivos sob ele. Para permitir um nome de diretório em qualquer profundidade, escreva `Edit(**/src/**)`.
* **Regras deny e ask**: `Read(secrets/**)` corresponde a um diretório nomeado `secrets` em qualquer profundidade sob o diretório atual, portanto a regra também se aplica a cópias aninhadas.

Toda outra forma de padrão corresponde na mesma profundidade em cada tipo de regra: `Edit(/src/**)` e `Edit(src/components/**)` correspondem apenas em sua localização ancorada, enquanto `Edit(**/src/**)` corresponde em qualquer profundidade.

O exemplo a seguir mostra cada forma de padrão contra um projeto com um diretório `src/` de nível superior e uma cópia aninhada sob `vendor/`:

```text theme={null}
<diretório-atual>/
├── src/
│   └── app.ts
└── vendor/
    └── pkg/
        └── src/
            └── lib.js
```

| Regra                                       | Corresponde a `src/app.ts` | Corresponde a `vendor/pkg/src/lib.js` |
| :------------------------------------------ | :------------------------- | :------------------------------------ |
| `Edit(src/**)` como uma regra allow         | Sim                        | Não                                   |
| `Edit(src/**)` como uma regra deny ou ask   | Sim                        | Sim                                   |
| `Edit(/src/**)` em qualquer tipo de regra   | Sim                        | Não                                   |
| `Edit(**/src/**)` em qualquer tipo de regra | Sim                        | Sim                                   |

<Note>
  Em padrões gitignore, `*` corresponde dentro de um único segmento de caminho e pode aparecer em qualquer posição no padrão, enquanto `**` corresponde entre diretórios.
</Note>

Quando você aprova um caminho de arquivo com "Sim, não pergunte novamente", Claude Code escapa caracteres de padrão gitignore nesse caminho, como `[`, `]` e `*`, portanto a regra gerada corresponde apenas ao caminho literal que você aprovou. Regras que você escreve você mesmo não são escapadas. Antes de v2.1.202, Claude Code salvava o caminho não escapado, portanto uma regra gerada para um diretório nomeado `[2024-06] Reports` poderia falhar em corresponder seu próprio caminho ou corresponder diretórios irmãos não intencionais.

Você não precisa escapar parênteses em um caminho, portanto `Edit(./Finance (2024)/**)` corresponde à pasta `Finance (2024)` conforme escrito.

Uma regra deny ou ask cujo caminho não é utilizável como um padrão gitignore ainda protege esse caminho exato. Uma regra allow com um padrão não utilizável não aprova nada.

Uma regra deny ou ask que começa com `!` é uma negação gitignore. Ela remove os caminhos que corresponde dos `path` ou `./path` regras listadas antes dela. Em uma lista `deny` de um arquivo de configurações, `Read(*.env)` seguido por `Read(!sample.env)` bloqueia cada arquivo cujo nome termina em `.env` em qualquer profundidade, exceto arquivos nomeados `sample.env`. Uma regra `!` listada primeiro remove nada.

A remoção alcança apenas regras da mesma fonte. Um `Read(!.env)` em configurações de projeto ou em `--disallowedTools` não cancela um `Read(./.env)` deny de configurações gerenciadas ou qualquer outro arquivo de configurações.

Dois limites estreitam o que um padrão `!` pode remover:

* Claude Code lê um padrão `!` relativo ao diretório atual mesmo quando `/`, `~/` ou `//` segue o `!`, portanto o padrão não consegue alcançar uma regra ancorada com um desses prefixos. `Read(!~/notes/public/**)` remove nada de `Read(~/notes/**)`.
* Uma remoção não consegue reabrir um arquivo dentro de um diretório que uma regra bloqueia como um todo. Com `Read(secrets/**)` e `Read(!secrets/public/**)`, Claude Code ainda bloqueia `secrets/public` junto com o resto de `secrets`.

Quando Claude acessa um symlink, as regras de permissão verificam dois caminhos: o próprio symlink e o arquivo para o qual ele se resolve. As regras allow e deny tratam esse par de forma diferente: as regras allow voltam a solicitar, enquanto as regras deny bloqueiam imediatamente.

* **Regras allow**: se aplicam apenas quando tanto o caminho do symlink quanto seu alvo correspondem. Um symlink dentro de um diretório permitido que aponta para fora dele ainda solicita.
* **Regras deny**: se aplicam quando o caminho do symlink ou seu alvo correspondem. Um symlink que aponta para um arquivo negado é ele próprio negado. Por exemplo, com `Read(./project/**)` permitido e `Read(~/.ssh/**)` negado, um symlink em `./project/key` apontando para `~/.ssh/id_rsa` é bloqueado: o alvo falha na regra allow e corresponde à regra deny.

Em macOS e Linux, uma regra deny ou ask escrita através de um diretório com symlink com um padrão `//`, `~/` ou `/` também se aplica na localização real do diretório. Por exemplo, em macOS, onde `/etc` se resolve para `/private/etc`, `Read(//etc/**)` também bloqueia `/private/etc/hosts`. Antes de v2.1.268, uma regra deny ou ask escrita através de um diretório com symlink não se aplicava a um caminho dado por sua localização real.

Quando uma ferramenta abre um arquivo aprovado, Claude Code [confirma que o caminho ainda se resolve para a localização que a verificação de permissão aprovou](/docs/pt/errors#refusing-after-a-symlink-changed).

Grep e Glob pesquisam o diretório para o qual o argumento `path` se resolve. Claude Code aplica regras deny `Read` a esse diretório.

<h3 id="webfetch">
  WebFetch
</h3>

As regras WebFetch usam um prefixo `domain:` e correspondem ao hostname da URL solicitada. A correspondência é insensível a maiúsculas e minúsculas, suporta caracteres curinga `*` e remove um `.` final tanto da regra quanto do hostname para que `example.com.` e `example.com` sejam tratados da mesma forma.

* `WebFetch(domain:example.com)` corresponde a solicitações para `example.com`
* `WebFetch(domain:*.example.com)` corresponde a qualquer subdomínio em qualquer profundidade, como `api.example.com` ou `a.b.example.com`, mas não a `example.com` em si
* `WebFetch(domain:*)` corresponde a cada domínio. Não é o mesmo que uma regra `WebFetch` simples; veja [Permitir ou negar cada fetch](#allow-or-deny-every-fetch)

Em qualquer posição que não seja um `*.` inicial ou um `*` simples, o caractere curinga corresponde apenas ao texto entre dois pontos. `WebFetch(domain:example.*)` corresponde a `example.org`, onde `*` se torna `org`, mas não a `example.evil.com`, onde `*` teria que se tornar `evil.com` e cruzar um ponto. Isso impede que um caractere curinga final corresponda a domínios que um atacante poderia registrar.

Caracteres curinga em regras `WebFetch` requerem Claude Code v2.1.172 ou posterior para corresponder fetches.

<h4 id="allow-or-deny-every-fetch">
  Permitir ou negar cada fetch
</h4>

Uma regra `WebFetch` simples é o nome da ferramenta sem uma parte `domain:`, como `"deny": ["WebFetch"]`. Tanto ela quanto `WebFetch(domain:*)` cobrem cada URL, mas Claude Code as aplica de forma diferente, e apenas a forma `domain:` também adiciona seu domínio à [lista de domínios permitidos ou negados](/docs/pt/sandboxing#network-isolation) do sandbox. Essa seção lista as formas com caracteres curinga que o sandbox honra e a versão que adicionou `*` simples.

Cada linha mostra o que uma regra faz na lista `allow` e na lista `deny`:

| Regra                | Em `allow`                                                                                    | Em `deny`                                                                                                                                      |
| :------------------- | :-------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `WebFetch`           | Claude faz fetch sem solicitar você. Não muda quais hosts comandos em sandbox podem alcançar. | Claude Code remove a ferramenta `WebFetch`, portanto Claude não consegue fazer fetch. Não muda quais hosts comandos em sandbox podem alcançar. |
| `WebFetch(domain:*)` | Claude faz fetch sem solicitar você, e comandos em sandbox podem alcançar qualquer host.      | Claude Code mantém a ferramenta e recusa cada fetch, e comandos em sandbox não conseguem alcançar nenhum host.                                 |

As duas formas também diferem em leituras de [artifacts](/docs/pt/artifacts), as páginas que a ferramenta Artifact publica em claude.ai. Uma regra deny ou ask `WebFetch` simples não se aplica a essas leituras. Uma regra `domain:` cobrindo `claude.ai` ou o host de conteúdo `*.claudeusercontent.com`, como `WebFetch(domain:claude.ai)` ou `WebFetch(domain:*)`, nega cada leitura ou solicita antes dela. Uma [regra `Artifact`](/docs/pt/artifacts#disable-artifacts) faz o mesmo.

Quando uma regra bloqueia uma leitura, a negação nomeia a regra. Antes de v2.1.268, uma regra deny `WebFetch` simples bloqueava cada leitura de artifact, e uma regra ask simples solicitava antes de cada uma.

Para deixar Claude fazer fetch livremente enquanto mantém a lista de permissões do sandbox como está, use a forma simples. Este `settings.json` faz isso:

```json theme={null}
{
  "permissions": {
    "allow": ["WebFetch"]
  }
}
```

Quando você pede a Claude para fazer fetch de uma página, ele faz fetch sem um prompt. Quando você pede a ele para executar um `curl` [em sandbox](/docs/pt/sandboxing) contra um host fora da lista de permissões do sandbox, Claude Code ainda solicita você para esse host, porque a regra simples não adicionou o host à lista de permissões.

Em [modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), Claude em vez disso nomeia o host no [domínios permitidos por comando](/docs/pt/sandboxing#per-command-allowed-domains-in-auto-mode) do comando para o classificador revisar.

<h3 id="mcp">
  MCP
</h3>

As regras MCP usam o nome do servidor conforme configurado em Claude Code, opcionalmente seguido pelo nome de uma ferramenta desse servidor.

* `mcp__puppeteer` corresponde a qualquer ferramenta fornecida pelo servidor `puppeteer`
* `mcp__puppeteer__*` usa sintaxe com caracteres curinga e também corresponde a todas as ferramentas do servidor `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` corresponde à ferramenta `puppeteer_navigate` fornecida pelo servidor `puppeteer`

Se sua organização definiu uma ferramenta [conector claude.ai](/docs/pt/mcp#organization-controls-on-connector-tools) como `ask` e essa configuração chega a Claude Code em sua sessão, as regras allow para essa ferramenta não entram em vigor: Claude Code solicita em cada chamada, mesmo em modos `auto` e `bypassPermissions`. No modo `dontAsk`, que nunca solicita, Claude Code nega a chamada em vez disso. As ferramentas de conector que Claude Code busca por si próprio aparecem como `mcp__claude_ai_<server>__<tool>`.

Em uma sessão de [Cowork](https://claude.com/docs/cowork/overview) no aplicativo Claude Desktop, Claude executa comandos de shell através da ferramenta `mcp__workspace__bash` do Cowork em vez da ferramenta `Bash` integrada, e Cowork igualmente fornece `mcp__workspace__web_fetch` para web fetches. Claude Code também aplica regras deny que nomeiam a ferramenta inteira `Bash` ou `WebFetch` a essas ferramentas Cowork, portanto uma regra deny `Bash` gerenciada impede Claude de executar comandos de shell em Cowork. Quando Claude Code bloqueia tal chamada, a mensagem nomeia a ferramenta Cowork: `Permission to use mcp__workspace__bash has been denied.` As regras allow não se transferem: Claude Code nunca aplica uma regra allow `Bash` a `mcp__workspace__bash`.

<h3 id="agent-subagents">
  Agent (subagents)
</h3>

Use regras `Agent(AgentName)` para controlar quais [subagents](/docs/pt/sub-agents) Claude pode usar:

* `Agent(Explore)` corresponde ao subagent Explore
* `Agent(Plan)` corresponde ao subagent Plan
* `Agent(my-custom-agent)` corresponde a um subagent personalizado nomeado `my-custom-agent`

Adicione essas regras ao array `deny` em suas configurações ou use a flag CLI `--disallowedTools` para desabilitar agentes específicos. Para desabilitar o agente Explore:

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

As regras `Cd` controlam para quais diretórios o [comando `/cd`](/docs/pt/commands) pode mover a sessão. `Cd` não é uma ferramenta invocável pelo modelo: Claude não pode chamá-la, e as regras se aplicam apenas quando você executa `/cd` você mesmo.

Uma regra deny `Cd` simples desabilita `/cd` inteiramente. Uma regra deny `Cd(<path-pattern>)` bloqueia alvos correspondentes. As regras deny verificam cada grafia do alvo, incluindo cada salto de symlink que ele se resolve através, portanto uma regra escrita para um caminho também bloqueia alvos que se resolvem para ele.

Adicionar qualquer regra allow `Cd` muda `/cd` para modo de lista de permissões: o diretório alvo resolvido deve corresponder a uma de suas regras allow, ou `/cd` recusa. Sem regras `Cd` configuradas, `/cd` mantém seu comportamento padrão e solicita que você confie em um diretório desconhecido.

Os padrões de caminho compartilham as âncoras `//`, `~/` e `/` das [regras Read e Edit](#read-and-edit), mas a correspondência é ancorada ao caminho do diretório inteiro em vez de estilo gitignore. `*` corresponde a exatamente um segmento de caminho e `**` corresponde entre segmentos. Um `/**` final também corresponde à sua raiz nomeada.

| Regra                 | Corresponde                                                                      | Não corresponde             |
| --------------------- | -------------------------------------------------------------------------------- | --------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                                                     | `~/code/app/src`, `~/code`  |
| `Cd(~/code/**)`       | `~/code` e qualquer diretório sob ele                                            | diretórios fora de `~/code` |
| `Cd(**/node_modules)` | qualquer diretório `node_modules` em qualquer profundidade sob o diretório atual | `node_modules/pkg`          |

<h2 id="extend-permissions-with-hooks">
  Estender permissões com hooks
</h2>

Os [hooks do Claude Code](/docs/pt/hooks-guide) permitem registrar comandos de shell personalizados que avaliam permissões em tempo de execução. Quando Claude Code faz uma chamada de ferramenta, os hooks PreToolUse são executados antes do prompt de permissão, para todas as ferramentas exceto [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior). A saída do hook pode negar a chamada de ferramenta, forçar um prompt ou pular o prompt para deixar a chamada prosseguir.

As decisões do hook não contornam as regras de permissão. Claude Code avalia regras deny e ask independentemente do que um hook PreToolUse retorna: uma regra deny correspondente bloqueia a chamada, e uma regra ask correspondente ainda solicita mesmo quando o hook retornou `"allow"` ou `"ask"`. Isto preserva a precedência deny-first descrita em [Gerenciar permissões](#manage-permissions), incluindo regras deny definidas em configurações gerenciadas.

As ferramentas MCP marcadas como [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) também ainda solicitam quando um hook retorna `"allow"`, assim como as ferramentas connector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) em sessões onde essa configuração chega ao Claude Code.

Um hook de bloqueio também tem precedência sobre regras allow. Um hook que sai com código 2 interrompe a chamada de ferramenta antes das regras de permissão serem avaliadas, portanto o bloqueio se aplica mesmo quando uma regra allow permitiria a chamada. Para executar todos os comandos Bash sem prompts exceto por alguns que você quer bloqueados, adicione `"Bash"` à sua lista allow e registre um hook PreToolUse que rejeita esses comandos específicos. Veja [Bloquear edições em arquivos protegidos](/docs/pt/hooks-guide#block-edits-to-protected-files) para um script de hook que você pode adaptar.

<h2 id="working-directories">
  Diretórios de trabalho
</h2>

Por padrão, Claude tem acesso a arquivos no diretório onde você o iniciou. Esse diretório é o diretório de trabalho primário da sessão até que você [mova a sessão com `/cd`](#move-the-session-to-another-directory). Você pode estender este acesso:

* **Durante a inicialização**: use o argumento CLI `--add-dir <path>`
* **Durante a sessão**: use o comando `/add-dir`
* **Configuração persistente**: adicione a `additionalDirectories` em [arquivos de configuração](/docs/pt/settings#where-settings-live)

Arquivos em diretórios adicionais seguem as mesmas regras de permissão do diretório de trabalho original: eles se tornam legíveis sem prompts, e as permissões de edição de arquivo seguem o modo de permissão atual.

Você não pode adicionar a maioria dos [caminhos de rede](/docs/pt/errors#working-directory-is-a-network-path), como o compartilhamento UNC `\\server\share`, como diretórios de trabalho, porque procurar um pode entrar em contato com o host que ele nomeia. No Windows, mapeie o compartilhamento para uma letra de unidade e passe a unidade com `--add-dir` na inicialização.

Defina [`permissions.blockReadsOutsideWorkingDirectories`](/docs/pt/settings-reference#permissions-blockreadsoutsideworkingdirectories) para fazer com que as ferramentas de arquivo recusem os caminhos que ele delimita em cada modo de permissão. No modo automático, Claude Code oferece ativá-lo na primeira vez que Claude [lê fora dos diretórios de trabalho](/docs/pt/permission-modes#first-read-outside-the-working-directories).

Em sessões em segundo plano no macOS, o host da sessão solicita acesso a pastas protegidas como `~/Desktop`, `~/Documents` e `~/Downloads` separadamente do seu terminal quando Claude precisa ler ou escrever arquivos lá; se as leituras falharem com `Operation not permitted`, consulte [como conceder acesso a pastas para sessões em segundo plano](/docs/pt/agent-view#background-sessions-can%E2%80%99t-read-desktop-documents-or-downloads-on-macos).

<h3 id="move-the-session-to-another-directory">
  Mover a sessão para outro diretório
</h3>

Para mover a sessão para um diretório de trabalho primário diferente, em vez de [adicionar um diretório](#working-directories) ao lado do atual, execute `/cd <path>`. Claude Code mantém a conversa, carrega o `CLAUDE.md` do novo diretório e solicita que você [confie no workspace](#project-allow-rules-and-workspace-trust) se você não tiver trabalhado nele antes. Depois, Claude Code [encontra a sessão movida](/docs/pt/sessions#resume-a-session) quando você executa `--resume` do novo diretório.

Assim que você se move, Claude Code aplica a configuração do projeto do novo diretório:

* Suas configurações de projeto, incluindo suas regras de permissão e [hooks](/docs/pt/hooks)
* Seus servidores [`.mcp.json`](/docs/pt/mcp#project-scope), sujeitos à mesma [aprovação de servidor](/docs/pt/mcp#project-server-approvals-and-workspace-trust) que na inicialização, e os servidores MCP [local-scope](/docs/pt/mcp#local-scope) que você registrou nele
* Os [plugins](/docs/pt/plugins/overview) que suas configurações habilitam, suas [skills](/docs/pt/skills#discovery-from-parent-and-nested-directories) e seus [subagentes](/docs/pt/sub-agents)
* Seus valores [`env`](/docs/pt/settings-reference#env), aplicados sobre as variáveis de ambiente das configurações do diretório anterior, que permanecem em vigor

Claude Code também desconecta os servidores MCP [local-scope](/docs/pt/mcp#local-scope) do projeto do diretório anterior e os servidores dos [plugins](/docs/pt/mcp#plugin-provided-mcp-servers) que não estão mais habilitados após a mudança. Ele pega [diretórios adicionais](#working-directories) das configurações do novo diretório em vez do anterior, e mantém os diretórios que você adicionou com `--add-dir` ou `/add-dir`. Hooks que a mudança ativa ainda recebem [`${CLAUDE_PROJECT_DIR}`](/docs/pt/hooks#reference-scripts-by-path) definido para a raiz do projeto onde a sessão começou.

Quando o novo diretório ainda não é confiável, Claude Code lista no prompt de confiança as regras de permissão, diretórios adicionais, hooks e comandos auxiliares que as configurações do diretório ativariam, para que você possa revisá-los antes de aceitar. Se você recusar, a sessão permanece onde está. Antes da v2.1.246, `/cd` não aplicava as configurações, hooks, servidores MCP ou skills do novo diretório até que você retomasse a sessão, e seu prompt de confiança não listava o que as configurações do diretório ativariam.

Restrinja ou desabilite destinos `/cd` com regras de permissão [`Cd`](#cd).

<h3 id="additional-directories-grant-file-access-not-configuration">
  Diretórios adicionais concedem acesso a arquivos, não configuração
</h3>

Adicionar um diretório estende onde Claude pode ler e editar arquivos. Não faz desse diretório uma raiz de configuração completa: a maioria da configuração `.claude/` não é descoberta de diretórios adicionais, embora alguns tipos sejam carregados como exceções.

Essas exceções se aplicam apenas a diretórios adicionados com o sinalizador `--add-dir` ou o comando `/add-dir`, incluindo diretórios que o Agent SDK adiciona através do sinalizador. Diretórios listados em `permissions.additionalDirectories` em um arquivo de configuração concedem apenas acesso a arquivos e não carregam nenhuma das configurações abaixo.

O [`additionalDirectories`](/docs/pt/agent-sdk/typescript#options) do Agent SDK em TypeScript e a opção [`add_dirs`](/docs/pt/agent-sdk/python#claudeagentoptions) em Python recebem as exceções também, mesmo que a opção TypeScript compartilhe seu nome com a chave de configurações. O SDK passa cada entrada para Claude Code como `--add-dir`, para que esses diretórios se comportem como diretórios adicionados por sinalizador. Skills, comandos e subagentes de qualquer diretório adicionado por sinalizador carregam através da fonte de configuração [`project`](/docs/pt/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources), então eles não carregam quando você exclui essa fonte com [`--setting-sources`](/docs/pt/cli-reference) na CLI ou `settingSources` no SDK, e [bare mode](/docs/pt/headless#start-faster-with-bare-mode) pula os comandos e subagentes entre eles.

Os seguintes tipos de configuração são carregados de diretórios `--add-dir`:

| Configuração                                                                             | Carregado de `--add-dir`                                                                                                                                                        |
| :--------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Skills](/docs/pt/skills) em `.claude/skills/`                                                | Sim, com recarga ao vivo                                                                                                                                                        |
| [Arquivos de comando](/docs/pt/skills#where-skills-live) em `.claude/commands/`               | Sim, sem recarga ao vivo. Quando o diretório adicionado e seu projeto definem um comando com o mesmo nome, Claude Code executa o comando do seu projeto                         |
| [Subagentes](/docs/pt/sub-agents) em `.claude/agents/`                                        | Sim, sem recarga ao vivo                                                                                                                                                        |
| [Configurações](/docs/pt/settings) em `.claude/settings.json` e `.claude/settings.local.json` | Apenas chaves `enabledPlugins` e [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces)                                                                      |
| Arquivos [CLAUDE.md](/docs/pt/memory), `.claude/rules/` e `CLAUDE.local.md`                   | Apenas quando `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` está definido. `CLAUDE.local.md` adicionalmente requer a fonte de configuração `local`, que é ativada por padrão |

Para carregar as skills, comandos e subagentes de um subdiretório do seu [diretório de trabalho primário](#working-directories) no meio da sessão, execute `/add-dir` com o caminho desse subdiretório. Claude Code os carrega pelo resto da sessão sem solicitá-lo ou adicionar um diretório de trabalho, porque o subdiretório já é legível. Isso requer Claude Code v2.1.257 ou posterior.

Claude Code descobre estilos de saída do diretório de trabalho atual e seus pais, seu diretório de usuário em `~/.claude/` e configurações gerenciadas. Hooks e outras chaves `.claude/settings.json` carregam da pasta `.claude/` do diretório de trabalho atual sem fallback de diretório pai, juntamente com seu `~/.claude/settings.json` de usuário e configurações gerenciadas. `.claude/settings.local.json` carrega da raiz do repositório git, mesmo quando você inicia Claude Code em um subdiretório, exceto nos casos em que Claude Code [não usa a raiz do repositório](/docs/pt/settings#where-claude-code-looks-for-each-file), como no Windows; antes da v2.1.211, ele também carregava apenas do diretório de trabalho atual. Sessões do [Agent SDK](/docs/pt/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) o carregam do diretório de trabalho em todas as versões.

Para compartilhar essa configuração entre projetos, use uma destas abordagens:

* **Configuração em nível de usuário**: coloque arquivos em `~/.claude/agents/`, `~/.claude/output-styles/` ou `~/.claude/settings.json` para torná-los disponíveis em cada projeto
* **Plugins**: empacote e distribua configuração como um [plugin](/docs/pt/plugins/overview) que as equipes podem instalar
* **Inicie do diretório de configuração**: execute Claude Code do diretório contendo a configuração `.claude/` que você deseja

<h2 id="how-permissions-interact-with-sandboxing">
  Como as permissões interagem com sandboxing
</h2>

Permissões e [sandboxing](/docs/pt/sandboxing) são camadas de segurança complementares:

* **Permissões** controlam quais ferramentas o Claude Code pode usar e quais arquivos ou domínios ele pode acessar. Elas se aplicam a Bash, Read, Edit, WebFetch, MCP e todas as outras ferramentas, exceto que uma regra de negação ou pergunta não pode bloquear [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior) enquanto qualquer outra ferramenta permanecer.
* **Sandboxing** fornece imposição em nível do SO que restringe o acesso ao sistema de arquivos e rede dos comandos shell. Aplica-se apenas a comandos Bash, PowerShell e [Monitor](/docs/pt/tools-reference#monitor-tool) e seus processos filhos.

Use ambos para defesa em profundidade, já que as restrições de sandbox ainda se aplicam mesmo se uma injeção de prompt contornar a tomada de decisão do Claude. Caminhos e domínios tanto das configurações de sandbox quanto das regras de permissão são [mesclados na configuração final de sandbox](/docs/pt/sandboxing#permission-rules).

Quando você ativa sandboxing e deixa `autoAllowBashIfSandboxed` em seu padrão de `true`, comandos Bash em sandbox são executados sem solicitação mesmo se suas permissões incluem uma regra de pergunta simples `Bash`, ou a [forma equivalente `Bash(*)`](#match-all-uses-of-a-tool): o limite de sandbox substitui esse prompt de ferramenta inteira.

Em [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode), o Claude Code pula essa substituição. Sem uma regra de pergunta, os [comandos somente leitura integrados](#read-only-commands) ainda são executados sem solicitação, e qualquer outro comando shell passa pelo fluxo de permissão regular enquanto você ainda está planejando; veja [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) para como o Claude Code controla comandos lá. Com uma regra de pergunta simples `Bash`, cada comando Bash solicita, incluindo comandos somente leitura em sandbox, o mesmo que fora do sandboxing. Antes da v2.1.212, a substituição também se aplicava em plan mode.

Essas verificações ainda se aplicam:

* Regras de pergunta com escopo de conteúdo como `Bash(git push *)` ainda forçam uma solicitação
* Regras de negação explícitas ainda se aplicam
* Comandos `rm` ou `rmdir` que visam um [caminho crítico](/docs/pt/permission-modes#critical-paths) ainda passam pelo fluxo de permissão regular

Comandos que não serão executados em sandbox, como comandos excluídos, respeitam a regra de pergunta simples `Bash` como de costume. Veja [sandbox modes](/docs/pt/sandboxing#sandbox-modes) para alterar esse comportamento.

<span id="managed-only-settings" />

<h2 id="managed-settings">
  Configurações gerenciadas
</h2>

Para organizações que precisam de controle centralizado, administradores implantam configurações gerenciadas que configurações de usuário e projeto não podem substituir, exceto por algumas [chaves sensíveis à segurança](/docs/pt/settings#exceptions-to-managed-settings-precedence). [Implantar configurações gerenciadas](/docs/pt/managed-settings) cobre os mecanismos de entrega, precedência dentro do nível gerenciado, e as [chaves que apenas configurações gerenciadas podem definir](/docs/pt/managed-settings#managed-only-settings).

Uma dessas chaves, [`allowManagedPermissionRulesOnly`](/docs/pt/settings-reference#allowmanagedpermissionrulesonly), torna as configurações gerenciadas a única fonte de configurações de regras de permissão. Sua entrada lista todas as fontes que Claude Code então ignora.

`disableBypassPermissionsMode` é tipicamente colocado em configurações gerenciadas para impor política organizacional, mas funciona de qualquer escopo. Um usuário pode defini-lo em suas próprias configurações para se bloquear do modo bypass.

<h2 id="settings-precedence">
  Precedência de configurações
</h2>

As regras de permissão seguem a mesma [precedência de configurações](/docs/pt/settings#settings-precedence) que todas as outras configurações do Claude Code, com configurações gerenciadas sendo as mais altas: nenhum outro nível, incluindo argumentos de linha de comando, pode substituir uma regra de permissão gerenciada.

Se uma ferramenta for negada em qualquer nível, nenhum outro nível pode permitir. Por exemplo, uma negação de configurações gerenciadas não pode ser substituída por `--allowedTools`, e `--disallowedTools` pode adicionar restrições além do que as configurações gerenciadas definem.

O mesmo se aplica entre escopos de configurações: se as configurações de usuário permitirem uma permissão e as configurações de projeto a negarem, a regra de negação a bloqueia. O inverso também é verdadeiro: uma negação no nível de usuário bloqueia uma permissão no nível de projeto, porque as regras de negação de qualquer escopo são avaliadas antes das regras de permissão.

Os hosts de incorporação podem fornecer política gerenciada adicional por meio da opção `managedSettings` do SDK, incluindo regras de permissão de permissão, a menos que o administrador defina os bloqueios `allowManaged*Only`; [Entregar política para sessões do Claude Desktop](/docs/pt/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) aborda quando a política do incorporador se aplica.

<h2 id="project-allow-rules-and-workspace-trust">
  Regras de permissão do projeto e confiança do workspace
</h2>

As regras `permissions.allow` e as entradas `permissions.additionalDirectories` no `.claude/settings.json` de um projeto concedem capacidade, portanto Claude Code as aplica apenas após você aceitar o [diálogo de confiança do workspace](/docs/pt/security#additional-safeguards) para essa pasta. O diálogo lista as regras e diretórios que a pasta concederia para que você possa revisá-los primeiro. As regras `deny` e `ask` não são afetadas, pois apenas restringem.

Claude Code armazena e salva a confiança que você aceita de acordo com onde você a inicia:

* Em um repositório, Claude Code baseia a confiança na raiz do repositório git, portanto a confiança cobre todo o repositório, exceto qualquer repositório git aninhado dentro dele, como um submódulo. Em uma [worktree](/docs/pt/worktrees), ele usa a raiz do checkout principal, como faz para [regras salvas](#permission-system).
* Fora de um repositório, Claude Code baseia a confiança no diretório a partir do qual você a iniciou, e a confiança cobre qualquer subdiretório desse diretório, exceto um repositório git aninhado dentro dele, como um clone. Cada subdiretório coberto então conta como uma pasta cujo pai você confiou.
* Quando você inicia no seu diretório inicial, Claude Code mantém a confiança apenas para a sessão atual e não a escreve em disco; consulte a nota sobre [salvaguardas adicionais](/docs/pt/security#additional-safeguards).

Claude Code mostra o diálogo de confiança apenas em sessões interativas. Uma execução `claude -p` ou uma sessão SDK nunca o mostra, e confiar em uma pasta pai não conta para essas regras, portanto [O que é executado antes de você confiar em uma pasta](#what-runs-before-you-trust-a-folder) diz qual conteúdo do repositório Claude Code ainda usa em cada uma dessas duas situações.

<h3 id="when-your-local-settings-file-needs-trust">
  Quando seu arquivo de configurações local precisa de confiança
</h3>

`.claude/settings.local.json` é normalmente seu próprio arquivo, portanto Claude Code aplica suas regras de permissão e diretórios adicionais sem a etapa de confiança. Quando o arquivo é rastreado no git, ou `.claude` é um symlink, Claude Code o trata como fornecido pelo repositório e mantém suas regras até você confiar na pasta.

Claude Code executa git para distinguir os dois, e executa git apenas uma vez que você confiou na pasta: você aceitou o diálogo de confiança para ela ou para um diretório pai cuja confiança se estende a ela, ou você está em uma sessão `-p` ou SDK, que conta como aceita. Até então, onde você iniciou Claude Code decide o que acontece com as regras do arquivo:

* **No seu diretório de configuração pessoal:** Claude Code aplica o `.claude/settings.local.json` dessa pasta imediatamente sem executar git. Seu diretório de configuração pessoal é seu diretório inicial, ou um diretório cujo subdiretório `.claude` você definiu como [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars#variables). Se esse diretório `CLAUDE_CONFIG_DIR` fica dentro de um repositório git e Claude Code [mantém suas configurações locais na raiz do repositório](/docs/pt/settings#where-claude-code-looks-for-each-file), ele mantém as regras como em qualquer outro lugar.
* **Em qualquer outro lugar:** Claude Code mantém as regras do arquivo como configurações do projeto. Uma vez que a verificação foi executada, Claude Code aplica as regras de um arquivo não rastreado, ou de um arquivo em um diretório fora de qualquer repositório git, mesmo que você não tenha confiado nessa pasta exata.

<Note>
  A exceção do diretório de configuração pula apenas a etapa de confiança. `~/.claude/settings.local.json` ainda é [escopo local](/docs/pt/settings#compare-the-scope-of-each-settings-file), portanto Claude Code a lê apenas em sessões que você inicia no seu diretório inicial, não em todos os projetos. Para aplicar regras de permissão em todos os seus projetos, adicione-as às suas configurações de usuário: `~/.claude/settings.json`, ou `$CLAUDE_CONFIG_DIR/settings.json` quando `CLAUDE_CONFIG_DIR` está definido.
</Note>

Nas versões 2.1.196 a 2.1.199, Claude Code mantinha as regras do arquivo no seu diretório de configuração pessoal e fora de repositórios git também, e imprimia o aviso [`this workspace has not been trusted`](/docs/pt/errors#workspace-has-not-been-trusted) lá. Antes da v2.1.207, Claude Code aplicava as regras de um arquivo não rastreado antes de você aceitar o diálogo.

<h3 id="what-runs-before-you-trust-a-folder">
  O que é executado antes de você confiar em uma pasta
</h3>

Cada linha é um tipo de conteúdo que um repositório pode fornecer. As colunas são as duas situações em que você não confiou na pasta em si: você confiou apenas em uma pasta pai, ou executou `claude -p` ou o SDK lá, que nunca mostra o diálogo de confiança. A coluna de pasta pai não se aplica dentro de um [repositório aninhado](#project-allow-rules-and-workspace-trust): em uma sessão interativa Claude Code mostra o diálogo de confiança para ela, e uma execução `claude -p` ou SDK lá segue a coluna `claude -p`.

| O que o repositório fornece                                                                                                                                                                                                                                                                                                       | Você confiou apenas em uma pasta pai                                                                                                                                                            | `claude -p` ou o SDK, pasta nunca confiada                                                                                                                                                          |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Hooks](/docs/pt/hooks) em arquivos de configurações, o bloco [`env`](/docs/pt/settings-reference#env) e comandos auxiliares como [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper), e os [hooks](/docs/pt/hooks#hooks-in-skills-and-agents) de uma skill do projeto e [`allowed-tools`](/docs/pt/skills#pre-approve-tools-for-a-skill)           | Usado                                                                                                                                                                                           | Usado. A confiança do workspace nunca bloqueia `allowed-tools` de uma skill em nenhuma sessão                                                                                                       |
| Regras `permissions.allow` e `additionalDirectories` em `.claude/settings.json`                                                                                                                                                                                                                                                   | Não usado até você aceitar o diálogo de confiança, que aparece novamente listando-os                                                                                                            | Não usado. Claude Code imprime um aviso [`this workspace has not been trusted`](/docs/pt/errors#workspace-has-not-been-trusted) para stderr                                                              |
| Hooks de frontmatter em um [subagent](/docs/pt/sub-agents#hooks-in-subagent-frontmatter) do projeto, um plugin [`@skills-dir`](/docs/pt/plugins/loading#plugins-shared-through-a-repository) do projeto, e entradas [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces) do repositório ou de um diretório `--add-dir` | Não usado, e nenhum diálogo é oferecido                                                                                                                                                         | Não usado                                                                                                                                                                                           |
| [`mcpServers`](/docs/pt/sub-agents#scope-mcp-servers-to-a-subagent) inline no frontmatter de um subagent do repositório ou de um diretório `--add-dir`. Antes da v2.1.238, Claude Code carregava esses servidores em ambas as situações                                                                                                | Não usado, e nenhum diálogo é oferecido                                                                                                                                                         | Não usado                                                                                                                                                                                           |
| Servidores em `.mcp.json`, incluindo aqueles que o repositório [aprova em suas próprias configurações](/docs/pt/mcp#project-server-approvals-and-workspace-trust)                                                                                                                                                                      | Claude Code pergunta antes de conectá-los. As aprovações do próprio repositório não contam                                                                                                      | Conectado sem perguntar, aprovado ou não. O SDK os carrega apenas quando `settingSources` inclui configurações do projeto. `claude mcp list` na mesma pasta ainda relata tal servidor como pendente |
| Um [`headersHelper`](/docs/pt/mcp#trust-a-folder-before-its-headershelper-runs) em um servidor em `.mcp.json`. Antes da v2.1.238, Claude Code executava o auxiliar em ambas as situações                                                                                                                                               | Não executado até você aceitar o diálogo de confiança, que aparece novamente nomeando onde o auxiliar é declarado. Claude Code conecta o servidor apenas com seus `headers` estáticos até então | Não executado. Claude Code conecta o servidor com seus `headers` estáticos e imprime uma linha [`headersHelper not run`](/docs/pt/errors#headershelper-not-run) por servidor para stderr                 |

Para as linhas que precisam dessa pasta exata confiada, confie nela manualmente: defina `projects["<path>"].hasTrustDialogAccepted` como `true` em `~/.claude.json`, onde `<path>` é a raiz do repositório, ou a pasta em si fora de um repositório. Claude Code imprime a chave exata na linha de log de depuração para um hook de subagent ignorado ou servidor MCP inline, no aviso stderr para regras de permissão ignoradas, e na linha `headersHelper not run` para um auxiliar ignorado.

Antes de executar `claude -p` em um repositório que você não escreveu, decida o que ele pode executar em sua máquina:

* Passe `--setting-sources user`, ou defina o `settingSources` do SDK sem configurações do projeto, para que Claude Code não leia nem os arquivos de configurações do projeto nem seu `.mcp.json`
* Inicie com [`--bare`](/docs/pt/headless#start-faster-with-bare-mode) para que Claude Code não leia hooks, skills, comandos personalizados, subagents, plugins ou servidores `.mcp.json` do projeto. O bloco `env` do projeto e auxiliares como `awsAuthRefresh` em seus arquivos de configurações ainda se aplicam, e Claude Code lê `apiKeyHelper` apenas de `--settings`
* Passe `--settings '{"disableAllHooks": true}'` para [desativar hooks](/docs/pt/hooks#disable-or-remove-hooks) para essa execução. Defini-lo apenas em suas configurações de usuário não é suficiente, porque as configurações do projeto do repositório têm precedência sobre as suas e podem defini-lo de volta para `false`
* Adicione uma entrada [`disabledMcpjsonServers`](/docs/pt/settings-reference#disabledmcpjsonservers) para rejeitar um servidor `.mcp.json` por nome em todos os tipos de sessão

<h2 id="example-configurations">
  Configurações de exemplo
</h2>

Este [repositório](https://github.com/anthropics/claude-code/tree/main/examples/settings) inclui configurações de configuração inicial para cenários de implantação comuns. Use-as como pontos de partida e ajuste-as para suas necessidades.

<h2 id="see-also">
  Veja também
</h2>

* [Todas as configurações](/docs/pt/settings-reference#permission-settings): todas as chaves de configuração, incluindo as chaves de permissão
* [Configure auto mode](/docs/pt/auto-mode-config): diga ao classificador do modo auto qual infraestrutura sua organização confia
* [Sandboxing](/docs/pt/sandboxing): isolamento de rede e sistema de arquivos em nível de SO para comandos Bash
* [Authentication](/docs/pt/authentication): configure o acesso do usuário ao Claude Code
* [Security](/docs/pt/security): salvaguardas de segurança e melhores práticas
* [Hooks](/docs/pt/hooks-guide): automatize fluxos de trabalho e estenda avaliação de permissão
