> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Executar sessões paralelas com worktrees

> Isole sessões paralelas do Claude Code em worktrees git separadas para que as alterações não colidam. Abrange o sinalizador `--worktree`, isolamento de subagentes, `.worktreeinclude`, limpeza e hooks de VCS não-git.

Uma [git worktree](https://git-scm.com/docs/git-worktree) é um diretório de trabalho separado com seus próprios arquivos e branch, compartilhando o mesmo histórico de repositório e remoto que seu checkout principal. Executar cada sessão do Claude Code em sua própria worktree significa que edições em uma sessão nunca tocam arquivos em outra, para que uma sessão possa construir um recurso enquanto uma segunda corrige um bug.

<Note>
  Worktrees exigem um repositório git; para outros sistemas de controle de versão, [configure hooks para substituir a lógica git](#non-git-version-control). No [aplicativo desktop](/docs/pt/desktop#work-in-parallel-with-sessions), selecione a opção **worktree** quando você iniciar uma sessão para dar a ela sua própria worktree.
</Note>

Worktrees são uma das várias maneiras de executar Claude em paralelo. Elas isolam edições de arquivo. [Subagentes](/docs/pt/sub-agents) dividem o trabalho dentro de uma sessão, e [mensagens entre sessões](/docs/pt/cross-session-messaging) permitem que Claude passe descobertas entre as sessões em suas worktrees. Consulte [Executar agentes em paralelo](/docs/pt/agents) para comparar as abordagens, ou pule para [Isolar subagentes com worktrees](#isolate-subagents-with-worktrees) para usar worktrees e subagentes juntos.

A maioria das sessões precisa apenas das duas primeiras seções: [inicie Claude em uma worktree](#start-claude-in-a-worktree), depois [limpe quando você sair](#clean-up-worktrees). Retorne ao resto da página quando precisar [retomar uma sessão](#resume-a-worktree-session), [alterar como as worktrees são criadas](#customize-worktree-creation), ou [depurar uma falha](#troubleshooting).

<h2 id="start-claude-in-a-worktree">
  Inicie Claude em uma worktree
</h2>

Passe `--worktree` ou `-w` com um nome para criar uma worktree isolada e iniciar Claude nela. Por padrão, a worktree é criada em `.claude/worktrees/<name>/` na raiz do seu repositório, em um novo branch nomeado `worktree-<name>`:

```bash theme={null}
claude --worktree feature-auth
```

Execute o comando novamente com um nome diferente em outro terminal para iniciar uma segunda sessão isolada. Se você omitir o nome, Claude gera um como `bright-running-fox`.

Execuções interativas exigem [confiança de workspace](/docs/pt/security): se você não tiver executado Claude no diretório antes, execute `claude` uma vez lá para aceitar o diálogo de confiança, ou `--worktree` sai com um erro solicitando que você o faça. Execuções não interativas com `-p` pulam a verificação de confiança, então `claude -p --worktree` prossegue sem ela.

<Tip>
  Adicione `.claude/worktrees/` ao seu `.gitignore` para que o conteúdo da worktree não apareça como arquivos não rastreados no seu checkout principal.
</Tip>

<h3 id="set-up-the-worktree-environment">
  Configure o ambiente da worktree
</h3>

Uma worktree é um checkout fresco, então inicialize seu ambiente de desenvolvimento lá: peça ao Claude para instalar dependências, ou execute a configuração do seu projeto você mesmo no diretório da worktree em `.claude/worktrees/`. Para levar arquivos ignorados pelo git como `.env` para cada nova worktree automaticamente, adicione um [arquivo `.worktreeinclude`](#copy-gitignored-files-into-worktrees).

<h3 id="ask-claude-to-create-a-worktree">
  Peça ao Claude para criar uma worktree
</h3>

Você também pode pedir ao Claude para "trabalhar em uma worktree" durante uma sessão, e ele cria uma com a ferramenta [`EnterWorktree`](/docs/pt/tools-reference). Uma vez em uma worktree, Claude pode alternar diretamente para outra em `.claude/worktrees/` chamando `EnterWorktree` com o caminho de destino; a worktree anterior permanece no disco intacta.

Quando Claude entra em um caminho fora do diretório `.claude/worktrees/` do repositório, Claude Code solicita sua aprovação primeiro, porque a mudança leva o diretório de trabalho da sessão, acesso de escrita e configuração do projeto como `CLAUDE.md` e configurações para esse local. Uma regra de permissão [`EnterWorktree`](/docs/pt/permissions) ou escolher "não perguntar novamente" não suprime este prompt; apenas o modo `bypassPermissions` o ignora. Antes da v2.1.206, Claude podia entrar em qualquer caminho de worktree existente sem perguntar.

<Note>
  **Caminhos de hook não seguem a worktree.** Depois que Claude entra em uma worktree, Claude Code mantém `${CLAUDE_PROJECT_DIR}` em seus [hooks](/docs/pt/hooks#reference-scripts-by-path) onde estava e passa o caminho da worktree para eles de uma forma diferente:

  * **`${CLAUDE_PROJECT_DIR}` fica no lugar**: ainda aponta para a raiz do projeto onde a sessão começou, então um comando de hook como `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` ainda executa o script no checkout principal.
  * **`cwd` segue Claude**: o campo `cwd` no [JSON de entrada](/docs/pt/hooks#common-input-fields) do hook é a raiz da worktree, e se move novamente quando Claude executa `cd`. Leia-o quando um hook precisa do caminho da worktree.
</Note>

<h2 id="clean-up-worktrees">
  Limpe worktrees
</h2>

Quando você sai de uma sessão de worktree interativa, Claude verifica a worktree para trabalho que a remoção deletaria: arquivos alterados ou não rastreados, trabalho não confirmado dentro de submódulos verificados e novos commits.

* **A worktree está limpa**: para uma sessão sem nome, Claude remove a worktree e seu branch automaticamente. Uma sessão [nomeada](/docs/pt/sessions#name-your-sessions) solicita primeiro para que você possa manter a worktree para depois
* **A worktree tem trabalho nela**: Claude solicita que você mantenha ou remova a worktree. Manter preserva o diretório e branch para que você possa retornar depois. Remover deleta o diretório da worktree e seu branch, junto com todo o trabalho neles
* **O estado da worktree não pode ser verificado**: quando Claude Code não consegue contar as alterações da worktree ou não consegue inspecionar seus checkouts de submódulo, ele solicita em vez de remover a worktree automaticamente. O prompt nomeia o que não conseguiu verificar

Execuções não interativas com `-p` não têm prompt de saída, então Claude não limpa suas worktrees, e Claude Code deixa o bloqueio que tomou em cada uma na criação em vigor até que uma [varredura de bloqueio obsoleto](#clean-up-subagent-and-background-session-worktrees) posterior o libere. Para remover uma, execute `git worktree remove`; se git recusar porque a worktree está bloqueada, execute `git worktree unlock` nela primeiro.

No Windows, remover uma worktree não deleta arquivos fora dela. Se uma pasta dentro da worktree é um link para outro lugar, como uma junção NTFS ou um symlink de diretório, Claude Code deleta apenas o link e mantém a pasta para a qual aponta. Antes da v2.1.205, remover uma worktree com um link aninhado em um subdiretório poderia deletar a pasta para a qual apontava.

<h2 id="resume-a-worktree-session">
  Retome uma sessão de worktree
</h2>

Quando você retoma uma sessão que estava dentro de uma worktree, Claude Code retorna a sessão para essa worktree. Isso vale para retomas interativas, para `--continue` e `--resume` em [modo não interativo](/docs/pt/headless) com `-p`, e para o Agent SDK. De volta dentro da worktree, Claude ainda pode sair dela com a ferramenta [`ExitWorktree`](/docs/pt/tools-reference).

Antes de retornar a sessão para sua worktree, Claude Code verifica que a worktree ainda é um checkout separado do principal, e recusa re-entrar em uma worktree que falha na verificação. Para uma git worktree, a verificação lê seus metadados git. Uma worktree sem metadados git, como uma que um hook [`WorktreeCreate`](#non-git-version-control) criou, pode passar na verificação; os casos que Claude Code ainda recusa estão listados com suas recuperações em [Claude Code recusa usar uma worktree](#claude-code-refuses-to-use-a-worktree). Para as mensagens e como recuperar de cada uma, consulte [A sessão retoma fora de sua worktree](#the-session-resumes-outside-its-worktree).

Onde você inicia e como você retoma mudam o que Claude Code re-entra:

* **Diretório de inicialização**: retome do checkout principal ou outro diretório do repositório. Claude Code re-entra em uma worktree que criou com git em `.claude/worktrees/` mesmo quando você inicia de dentro dela. Quando você inicia de dentro de qualquer outra worktree, Claude Code re-entra nela apenas se puder garantir por ela de lá: uma worktree que é seu próprio repositório, uma sem metadados git, ou um início de um subdiretório de uma worktree que você criou com `git worktree add` recusa, então inicie aquelas do checkout principal.
* **`--fork-session`**: a sessão bifurcada começa no diretório de onde você iniciou Claude, e Claude Code deixa a worktree da sessão original intacta.
* **Worktree deletada**: se o diretório da worktree não existe mais, Claude Code retoma a sessão no diretório de onde você iniciou Claude. Ele diz que a worktree se foi e limpa a vinculação de worktree da sessão.

<Note>
  Antes da v2.1.212, uma retoma não interativa ficava no diretório inicial e `ExitWorktree` relatava que não havia sessão de worktree ativa para sair.
</Note>

Quando Claude entra ou sai de uma worktree que Claude Code criou com git, a transcrição segue: Claude Code registra a sessão no novo diretório de trabalho da sessão, da mesma forma que [`/cd`](/docs/pt/commands) faz, então `/desktop` e `--resume` a encontram lá. Sair a move de volta da mesma forma. Uma worktree criada por um hook [`WorktreeCreate`](#non-git-version-control) mantém sua transcrição no diretório de inicialização. Requer Claude Code v2.1.198 ou posterior.

<h2 id="how-claude-code-enforces-isolation">
  Como Claude Code impõe isolamento
</h2>

Enquanto uma sessão está isolada em uma worktree, Claude Code bloqueia as chamadas de ferramenta que as verificações abaixo definem. As mesmas regras se aplicam se você iniciou a sessão com `--worktree`, Claude entrou em uma worktree com `EnterWorktree`, ou você retomou uma sessão de worktree.

A mesma imposição cobre cada subagente que Claude gera da sessão isolada. Aplica-se se a sessão é interativa ou executa em [segundo plano](/docs/pt/agent-view#how-file-edits-are-isolated). [Subagentes que executam em sua própria worktree](#isolate-subagents-with-worktrees) carregam as mesmas verificações. Seu histórico de versão está em [Escrever arquivos de subagente](/docs/pt/sub-agents#write-subagent-files).

Claude Code aplica quatro verificações:

* **Edições de arquivo**: Claude Code bloqueia um `Edit`, `Write`, ou `NotebookEdit` que visa um caminho no checkout principal.
* **Diretório de trabalho do comando**: Claude Code bloqueia um comando Bash, PowerShell, ou Monitor cujo diretório de trabalho resolve para o checkout principal, ou cujo diretório de trabalho não pode verificar que fica fora dele.
* **Redirecionamentos git**: Claude Code bloqueia um comando Bash ou Monitor que redireciona git para o checkout principal. O redirecionamento pode vir através de `git -C`, `--git-dir`, uma variável `GIT_DIR` ou `GIT_WORK_TREE`, ou um `cd` para o checkout principal antes de executar git.
* **Forma do comando**: Claude Code bloqueia um comando Bash ou Monitor quando não pode verificar do texto do comando que qualquer git que o comando executa fica dentro da worktree. Isso acontece, por exemplo, quando o nome do comando é computado em tempo de execução, quando a sintaxe não pode ser analisada, ou quando uma expansão como `${!name}` ou `${ command; }` poderia executar um comando que o texto não especifica. Claude Code diz a Claude como reescrever o comando recusado, como dividi-lo em comandos simples e separados. Você não pode desativar essa verificação.

As verificações se aplicam ao repositório de onde você iniciou Claude Code. Elas também cobrem o checkout principal que uma worktree vinculada está vinculada de. Para comandos PowerShell, Claude Code aplica apenas a verificação de diretório de trabalho.

Claude vê cada recusa como um erro de ferramenta que nomeia a worktree e diz como proceder. Para um comando recusado, veja [o que a mensagem de recusa significa e como limpá-la](/docs/pt/errors#command-blocked-by-the-worktree-isolation-checks).

<h2 id="isolate-subagents-with-worktrees">
  Isole subagentes com worktrees
</h2>

Subagentes podem executar em suas próprias worktrees para que edições paralelas não entrem em conflito. Peça ao Claude para "usar worktrees para seus agentes", ou torne o isolamento permanente para um [subagente personalizado](/docs/pt/sub-agents#supported-frontmatter-fields) adicionando `isolation: worktree` ao seu frontmatter.

Este subagente em `.claude/agents/` sempre executa em sua própria worktree:

```markdown theme={null}
---
name: refactorer
description: Applies mechanical refactors across many files
isolation: worktree
---

Apply the requested refactor across every affected file, then run the tests
and report the results.
```

Cada subagente obtém uma worktree temporária que Claude Code remove automaticamente quando o subagente termina sem alterações; uma worktree com alterações fica no disco até que a [varredura periódica abaixo](#clean-up-subagent-and-background-session-worktrees) possa removê-la sem perder trabalho.

As worktrees de subagentes usam a mesma [branch base](#choose-the-base-branch) que `--worktree`, então elas fazem branch da branch padrão do seu repositório a menos que `worktree.baseRef` seja definido como `"head"`.

<h3 id="clean-up-subagent-and-background-session-worktrees">
  Limpe worktrees de subagente e sessão em segundo plano
</h3>

Claude Code executa uma varredura periódica que remove worktrees que Claude criou para subagentes e [sessões em segundo plano](/docs/pt/agent-view#how-file-edits-are-isolated) uma vez que são mais antigas que sua configuração [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays), seguindo as [regras de varredura de retenção](/docs/pt/claude-directory#cleaned-up-automatically).

Quando você [coloca em segundo plano](/docs/pt/agent-view#send-the-session-to-the-background) uma sessão `--worktree`, sua worktree se torna uma worktree de sessão em segundo plano que a varredura pode remover. A varredura deixa uma worktree no lugar nestes casos:

* A worktree ainda contém trabalho: arquivos alterados ou não rastreados, ou commits não enviados.
* Um submódulo verificado na worktree contém arquivos alterados ou não rastreados, ou Claude Code não consegue inspecionar os submódulos da worktree. Esta verificação requer Claude Code v2.1.274 ou posterior.
* Um dos [quatro casos que também bloqueiam a criação de worktree](#git-lfs-content-is-missing-from-a-worktree-claude-code-created) se aplica: Claude Code não pode determinar quais drivers de filtro a configuração do repositório define, ou encontra uma configuração lá que não pode desativar.
* A worktree pertence a uma sessão `--worktree` que você não colocou em segundo plano, qualquer que seja sua idade.
* Você criou a worktree você mesmo com `git worktree add`, mesmo que depois tenha executado uma sessão `--worktree <name>` nela e colocado essa sessão em segundo plano.

Claude Code escreve um marcador nos metadados git de cada worktree que cria com git, e a varredura mantém qualquer worktree sem um, incluindo uma worktree que um hook [`WorktreeCreate`](#non-git-version-control) criou. Antes da v2.1.246, a varredura não verificava o marcador, e poderia remover uma worktree que você criou você mesmo quando um registro antigo de sessão em segundo plano apontava para ela.

Enquanto um agente está em execução, Claude Code mantém um `git worktree lock` em sua worktree para que limpeza simultânea não possa removê-la, e libera o bloqueio quando o agente termina. Claude Code mantém o mesmo bloqueio na worktree que criou para uma sessão em segundo plano enquanto a sessão executa, para que a varredura deixe a worktree no lugar e `git worktree remove` recuse removê-la.

A varredura também libera um bloqueio que Claude Code definiu para uma sessão cujo processo saiu, para que uma sessão em segundo plano morta não deixe sua worktree permanentemente bloqueada. A varredura nunca libera um bloqueio que você definiu você mesmo com `git worktree lock`. Antes da v2.1.210, um bloqueio deixado por uma sessão morta ficava no lugar até que você executasse `git worktree unlock`.

Para limpar uma worktree que a varredura mantém, execute `git worktree remove`, adicionando `--force` se a worktree tiver alterações não confirmadas ou arquivos não rastreados. Se git recusar porque a worktree está bloqueada, execute `git worktree unlock` nela primeiro.

<h2 id="customize-worktree-creation">
  Customize a criação de worktree
</h2>

Os padrões de Claude Code para criar worktrees cobrem a maioria das sessões: ele as cria em `.claude/worktrees/`, as faz branch da branch padrão do seu repositório, e faz checkout apenas de arquivos rastreados. As opções nesta seção mudam esses padrões.

<h3 id="choose-the-base-branch">
  Escolha a branch base
</h3>

Novas worktrees fazem branch da branch padrão do repositório, então a maioria das sessões não precisa dessa configuração. Defina `worktree.baseRef` em [configurações](/docs/pt/settings-reference#worktree) para fazer branch do seu trabalho atual. A configuração aceita dois valores:

* `"fresh"` (padrão): faz branch da branch padrão do repositório no remoto, geralmente `main`, para que a worktree comece de uma árvore limpa correspondendo ao remoto.
* `"head"`: faz branch do seu `HEAD` local atual, para que a worktree carregue seus commits não enviados e estado de branch de recurso. Use isso ao isolar subagentes que precisam operar em trabalho em andamento. Dentro de uma worktree, `"head"` resolve para o `HEAD` dessa worktree, não para o checkout principal.

Você não pode definir `worktree.baseRef` para um nome de branch. Para iniciar uma worktree de um branch existente específico, [crie-a com git diretamente](#manage-worktrees-manually).

Para uma base `"fresh"`, Claude Code mantém `origin/HEAD` atual: quando o repositório não foi buscado nos últimos 24 horas, ele busca a branch padrão, limitado a cinco segundos, e usa a ref localmente armazenada em cache se a busca falhar. Se nenhum remoto estiver configurado, ou `origin/HEAD` não estiver armazenado em cache localmente e não puder ser buscado, a worktree volta para seu `HEAD` local atual. Antes da v2.1.208, uma worktree fresca usava qualquer `origin/HEAD` que já estivesse armazenado em cache localmente.

Este exemplo faz cada nova worktree fazer branch do seu trabalho atual:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

<h3 id="branch-from-a-pull-request">
  Faça branch de um pull request
</h3>

Para fazer branch de um pull request ou merge request específico, passe `--worktree` o número prefixado com `#`, uma URL de pull request do GitHub, ou uma URL de merge request do GitLab como `https://gitlab.com/group/repo/-/merge_requests/123`. Claude Code busca o commit head dessa mudança de `origin` e cria a worktree em `.claude/worktrees/pr-<number>`. Cite o argumento para que seu shell não trate `#` como o início de um comentário:

```bash theme={null}
claude --worktree "#1234"
```

Claude Code lê apenas o número da URL. Ele sempre busca do remoto `origin` do seu repositório, e escolhe o caminho de busca pelo host de `origin`:

* **github.com**: busca `pull/<number>/head`
* **gitlab.com**: busca `merge-requests/<number>/head`
* **GitHub Enterprise, GitLab auto-hospedado, ou qualquer outro host**: tenta `pull/<number>/head` primeiro, depois `merge-requests/<number>/head`

Antes da v2.1.233, Claude Code aceitava apenas `#<number>` e URLs de pull request no estilo GitHub para `--worktree`, e sempre buscava `pull/<number>/head`.

<h3 id="copy-gitignored-files-into-worktrees">
  Copie arquivos ignorados pelo git em worktrees
</h3>

Uma worktree é um checkout fresco, então arquivos não rastreados como `.env` ou `.env.local` do seu repositório principal não estão presentes. Para copiá-los automaticamente quando Claude cria uma worktree, adicione um arquivo `.worktreeinclude` à raiz do seu projeto.

O arquivo usa sintaxe `.gitignore`. Apenas arquivos que correspondem a um padrão e também são ignorados pelo git são copiados, então arquivos rastreados nunca são duplicados.

Se você escrever um padrão que começa com `**/` e os arquivos que você quer estão dentro de um diretório que é ignorado pelo git como um todo, Claude Code copia-os apenas quando esse diretório em si corresponde ao padrão, ou quando o primeiro nome após o `**/` é um dos nomes no caminho do diretório. Por exemplo, se você escrever `**/.claude/skills/*.md`, esse primeiro nome é `.claude`, então Claude Code copia os arquivos correspondentes de um diretório `.claude/` ignorado. Para copiar arquivos de um diretório ignorado que um padrão `**/` não alcança, nomeie o diretório no padrão em vez disso: escreva `vendor/**/config.json` em vez de `**/config.json`. Antes da v2.1.239, Claude Code copiava arquivos de um diretório completamente ignorado para um padrão `**/` apenas quando o diretório em si correspondia ao padrão.

Este `.worktreeinclude` copia dois arquivos env e uma configuração de segredos em cada nova worktree:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Isso se aplica a cada worktree que Claude Code cria com git: worktrees `--worktree`, [worktrees de subagente](#isolate-subagents-with-worktrees), e sessões paralelas no [aplicativo desktop](/docs/pt/desktop#work-in-parallel-with-sessions). Com um hook [`WorktreeCreate`](#non-git-version-control), copie os arquivos dentro do script de hook.

<h3 id="reuse-a-worktree-name">
  Reutilize um nome de worktree
</h3>

Passar `--worktree` um nome cujo diretório já existe abre essa worktree existente em vez de criar uma nova.

Com a [base](#choose-the-base-branch) padrão `"fresh"`, uma worktree reabierta redefine para a branch padrão do repositório em vez de continuar em sua ponta antiga quando todos os seguintes se aplicam:

* Não tem alterações não confirmadas ou arquivos não rastreados.
* Ainda está na branch que Claude Code criou para ela.
* Não tem commits próprios, ou seu pull request ou merge request foi mesclado e sua branch remota foi deletada.

Claude Code detecta o caso mesclado apenas do estado git: a branch remota para a qual a worktree enviou não existe mais, e cada commit na worktree já está na branch padrão.

Em todos os outros casos, Claude Code reabre a worktree em sua ponta antiga:

* A worktree falha em qualquer uma das condições.
* Claude Code não pode verificar o estado da worktree.
* `worktree.baseRef` é `"head"`.
* O nome é uma referência de pull request ou merge request.

Antes da v2.1.208, quando você reutilizava um nome, Claude Code sempre reabia a worktree antiga em sua ponta antiga.

<h3 id="replace-worktree-creation-with-a-hook">
  Substitua a criação de worktree com um hook
</h3>

Configure um hook [`WorktreeCreate`](/docs/pt/hooks#worktreecreate) para substituir completamente a lógica padrão de `git worktree`, incluindo colocar worktrees em outro lugar que não `.claude/worktrees/`. Para um exemplo completo, consulte [Controle de versão não-git](#non-git-version-control).

<h2 id="what-worktrees-share-with-the-main-checkout">
  O que worktrees compartilham com o checkout principal
</h2>

Uma worktree obtém seus próprios arquivos e branch, mas compartilha o seguinte com o checkout principal:

* **O diretório `.git` do repositório**: comandos git em uma worktree escrevem no diretório `.git` compartilhado do repositório principal, e [sandboxing](/docs/pt/sandboxing#filesystem-isolation) permite essas escritas, então comandos como `git commit` funcionam de dentro de uma worktree com a sandbox ativada.
* **Plugins**: plugins instalados em [escopo de projeto](/docs/pt/plugins/loading#find-where-a-plugin-is-enabled) do checkout principal também carregam em worktrees do mesmo repositório, então você não precisa reinstalá-los por worktree. Requer Claude Code v2.1.200 ou posterior.
* **Aprovações de permissão**: escolher "Sim, e não pergunte novamente" para um comando Bash em uma sessão de worktree salva a regra no `.claude/settings.local.json` do checkout principal, para que se aplique no checkout principal e em cada outra worktree do repositório, e sobreviva à remoção da worktree. No Windows e nos outros casos onde Claude Code [não usa a raiz do repositório](/docs/pt/settings#where-claude-code-looks-for-each-file), a regra fica com essa worktree. Antes da v2.1.211, uma aprovação concedida em uma worktree era salva dentro dessa worktree, não se aplicava em outro lugar, e era perdida quando a worktree era removida. Consulte [onde as aprovações são salvas](/docs/pt/permissions#permission-system).
* **Skills, agentes e comandos não rastreados**: quando o checkout da worktree não tem um diretório `.claude/skills` em sua raiz, por exemplo porque seu `.claude/skills` é gitignored, Claude Code carrega as [skills de projeto](/docs/pt/skills#where-skills-live) do checkout principal na sessão da worktree. Em uma worktree com seu próprio diretório `.claude/skills`, apenas essa cópia carrega.

  A mesma leitura abrange `.claude/agents` e `.claude/commands`. Para skills, a leitura requer Claude Code v2.1.277 ou posterior.

Todos esses se aplicam se você criar a worktree com `--worktree`, com `git worktree add`, ou através do [aplicativo desktop](/docs/pt/desktop#work-in-parallel-with-sessions).

<h2 id="manage-worktrees-manually">
  Gerencie worktrees manualmente
</h2>

Crie worktrees com Git diretamente quando você precisa fazer checkout de um branch existente específico ou colocar a worktree fora do repositório.

Crie uma worktree em um novo branch:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Crie uma worktree de um branch existente, substituindo `fix-issue-456` por um branch que já existe no seu repositório:

```bash theme={null}
git worktree add ../project-bugfix fix-issue-456
```

Inicie Claude na worktree:

```bash theme={null}
cd ../project-feature-a
claude
```

Liste suas worktrees:

```bash theme={null}
git worktree list
```

Remova uma quando terminar com ela:

```bash theme={null}
git worktree remove ../project-feature-a
```

Consulte a [documentação de git worktree](https://git-scm.com/docs/git-worktree) para a referência completa de comandos.

<h2 id="non-git-version-control">
  Controle de versão não-git
</h2>

Isolamento de worktrees usa git por padrão. Para SVN, Perforce, Mercurial, ou outros sistemas, configure hooks [`WorktreeCreate` e `WorktreeRemove`](/docs/pt/hooks#worktreecreate) para fornecer lógica de criação e limpeza personalizada. Como o hook substitui o comportamento padrão do git, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) não é processado quando você usa `--worktree`. Copie quaisquer arquivos de configuração local dentro do seu script de hook.

Este hook `WorktreeCreate` lê o nome da worktree do JSON em stdin com `jq`, faz checkout de uma cópia de trabalho SVN fresca, e imprime o caminho do diretório para que Claude Code possa usá-lo como o diretório de trabalho da sessão. Adicione a configuração ao seu [`settings.json`](/docs/pt/settings#where-settings-live):

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Emparelhe-o com um hook `WorktreeRemove` para limpar quando a sessão terminar. Consulte a [referência de hooks](/docs/pt/hooks#worktreecreate) para o esquema de entrada e um exemplo de remoção.

Um hook `WorktreeCreate` também permite que você execute [`/batch`](/docs/pt/commands#all-commands) fora de um repositório git. Cada subagente `/batch` publica sua alteração com os comandos de controle de versão do seu projeto e, quando não consegue abrir uma solicitação de pull, relata o que publicou. Executar `/batch` fora de um repositório git requer Claude Code v2.1.281 ou posterior.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Claude Code relata os erros abaixo quando cria uma worktree, entra em uma na inicialização, ou retorna uma sessão retomada para uma.

<h3 id="claude-code-can’t-enter-the-worktree-at-startup">
  Claude Code não consegue entrar na worktree na inicialização
</h3>

Quando Claude Code não consegue entrar no diretório da worktree na inicialização, ele imprime um erro nomeando o caminho e sai com código 1. Isso pode acontecer quando um hook [`WorktreeCreate`](/docs/pt/hooks#worktreecreate) imprime algo diferente do diretório que criou, ou quando o diretório foi deletado após ser configurado.

<h3 id="worktree-creation-fails-on-a-symlinked-path">
  A criação de worktree falha em um caminho symlinked
</h3>

Claude Code recusa criar uma worktree quando `.claude`, `.claude/worktrees`, ou o diretório da worktree em si é um symlink, e o erro nomeia o caminho symlinked. Remova o symlink e tente novamente. Antes da v2.1.212, se o repositório já continha um symlink confirmado em um desses caminhos, a criação de worktree o seguia e poderia criar arquivos fora do repositório.

<h3 id="git-lfs-content-is-missing-from-a-worktree-claude-code-created">
  Arquivos Git LFS são arquivos de ponteiro em uma worktree que Claude Code criou
</h3>

Se você configurou [Git LFS](https://git-lfs.com) com `git lfs install --local`, uma worktree que Claude Code cria contém arquivos de ponteiro LFS em vez dos arquivos reais. O sinalizador `--local` escreve o filtro LFS no `.git/config` do repositório em vez de sua configuração git global. Um `git lfs install` simples escreve em sua configuração global e não é afetado. O mesmo se aplica a qualquer outro [driver de filtro](https://git-scm.com/docs/gitattributes) definido na configuração do repositório em si.

Claude Code pula os drivers de filtro do repositório em si quando cria uma worktree porque um driver de filtro é um comando shell, e qualquer coisa que possa escrever no repositório, incluindo Claude, poderia ter colocado um lá. Antes da v2.1.247, Claude Code executava esses drivers durante a criação de worktree.

Para obter os arquivos reais, execute `git lfs pull` dentro da worktree.

Em quatro casos raros, Claude Code não cria nenhuma worktree: não consegue dizer quais drivers de filtro a configuração do repositório define, ou encontra uma configuração lá que não consegue desativar. Corresponda o erro à sua correção:

* **`Could not read the repository git config to neutralize filter drivers`**: Claude Code não conseguiu ler o `.git/config` do repositório, por exemplo por causa de suas permissões. Corrija isso e tente novamente.
* **`The repository git config defines a filter driver whose name cannot be neutralized (contains "=" or a newline)`**: renomeie ou remova esse driver de filtro em `.git/config` e tente novamente.
* **`The repository git config has a conditional include (includeIf)`**: mova as configurações que o `includeIf` em `.git/config` puxa diretamente para esse arquivo, remova o `includeIf`, e tente novamente. Um `includeIf` em sua configuração git global não dispara isso.
* **`Git was not run: the repository's own git config sets <key>`**: a mensagem nomeia uma chave que aponta Git LFS para um programa a executar, como `lfs.customtransfer.<name>.path` ou `lfs.standalonetransferagent`. Se essa configuração é sua, mova-a para sua configuração git global. Se você não a reconhecer, remova-a da configuração git do repositório, já que uma ferramenta ou checkout que você não confia pode tê-la escrito. Tente novamente uma vez que a chave tenha desaparecido da configuração do repositório.

<h3 id="claude-code-refuses-to-use-a-worktree">
  Claude Code recusa usar uma worktree
</h3>

Um erro começando com `Refusing to use <path> as an isolation worktree` significa que Claude Code verificou a identidade git do diretório antes de adotá-lo como um checkout isolado de sessão ou subagente, e recusou. A verificação executa se Claude Code está criando a worktree, entrando em uma existente, ou reutilizando uma de uma execução anterior.

Na maioria dos casos, o resto da mensagem diz que os metadados git do diretório resolvem para o checkout principal: por exemplo, seu arquivo `.git` aponta para o diretório `.git` do repositório principal em si, ou git resolve sua árvore de trabalho para o checkout principal através de um redirecionamento `core.worktree`. De tal diretório, um comando git comum como `git reset --hard` agiria no checkout principal em vez da worktree. Claude Code também recusa quando o diretório tem uma entrada `.git` que não pode ler, em vez de assumir que a worktree é segura.

Um diretório sem metadados git, como um que seu hook [`WorktreeCreate`](#non-git-version-control) cria, passa na verificação apenas quando nenhum repositório git o contém. Se o hook cria o diretório dentro de um repositório, git resolve-o para o checkout desse repositório e Claude Code recusa-o com a mensagem `git resolves its working tree to`, então faça o hook criar seus diretórios fora de qualquer repositório.

Claude Code deixa o diretório recusado no lugar, já que pode conter trabalho. Corresponda a mensagem à sua recuperação, se segue `Refusing to use <path>` ou aparece em uma [mensagem de retoma](#the-session-resumes-outside-its-worktree); alguns finais ocorrem apenas em mensagens de retoma:

* **Diz `launch from the parent checkout` ou `Run the resume from the project checkout`**: você iniciou Claude Code de dentro da worktree. Inicie do checkout principal em vez disso; a worktree não precisa de recriação.
* **Diz `it cannot be resumed or re-entered`**: nada nesta sessão garante a worktree de onde você iniciou. Recrie-a; o diretório e seu trabalho permanecem no disco para recuperação manual, e quando a worktree tem um checkout pai, retomar de lá também funciona.
* **Diz `it contains the protected checkout`**: o diretório recusado é um pai do seu checkout principal, como seu diretório home. Não o delete. Altere o caminho da worktree, como o caminho que seu hook `WorktreeCreate` retorna ou o alvo `EnterWorktree`, para que a worktree não contenha o checkout.
* **Diz `the protected checkout <path> has a .git entry that could not be examined` ou `has git metadata that could not be resolved`**: o problema é os metadados git do checkout principal, não da worktree. Não delete a worktree, e ignore o conselho final da mensagem para recriá-la, que não se aplica a esses dois finais. Repare o checkout principal, por exemplo um problema de permissões ou uma recusa git `dubious ownership` em seu `.git`, e tente novamente.
* **Diz `its recorded path has a network spelling`**: Claude Code nunca retoma em uma worktree em um caminho de rede. Recrie a worktree em um caminho local.
* **Qualquer outro final**: a mensagem nomeia o problema e sua correção, como remover um redirecionamento `core.worktree` ou recriar a worktree; siga-a. Antes de deletar um diretório cuja mensagem diz que sua identidade git não pôde ser verificada, aborde a causa nomeada primeiro, por exemplo um link simbólico no caminho da worktree ou git em si falhando em executar, já que o diretório pode estar saudável. Quando você recriar, salve quaisquer mudanças que você precisa do diretório antigo primeiro; ele fica no disco.

<h3 id="the-session-resumes-outside-its-worktree">
  A sessão retoma fora de sua worktree
</h3>

Quando você retoma uma sessão interativamente e Claude Code não consegue retorná-la para sua worktree, Claude Code diz assim com uma das mensagens abaixo. Quando Claude Code limpa a vinculação de worktree, ele registra a limpeza na transcrição da sessão. Se você [suprimir escritas de transcrição](/docs/pt/sessions#where-transcripts-are-stored), a mensagem diz em vez disso que a vinculação não pôde ser limpa e que Claude Code re-verificará a worktree em uma retoma posterior.

| A mensagem começa com                             | O que aconteceu e o que fazer                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Your worktree <path> no longer exists`           | O diretório da worktree foi removido. A sessão continua no diretório atual sem isolamento, e Claude Code limpa a vinculação de worktree. Nenhuma ação necessária.                                                                                                                                                                                                                                                                                                        |
| `Could not verify your worktree <path> this time` | Claude Code não conseguiu verificar a worktree, geralmente por uma razão transitória; a vinculação é mantida, e a sessão continua no diretório atual sem isolamento. Retome novamente para tentar novamente; se continuar acontecendo, entre na worktree em uma nova sessão e corresponda a mensagem de recusa em [Claude Code recusa usar uma worktree](#claude-code-refuses-to-use-a-worktree), que pode nomear os metadados do checkout principal em vez da worktree. |
| `Did not re-enter your worktree <path>`           | Claude Code recusou a vinculação de worktree como insegura; ela limpa a vinculação e a sessão continua sem isolamento. A mensagem inclui a recusa específica: corresponda-a em [Claude Code recusa usar uma worktree](#claude-code-refuses-to-use-a-worktree), já que a correção é recriação para algumas recusas e mudança de caminho para outras.                                                                                                                      |
| `Could not re-enter your worktree <path>`         | Claude Code não conseguiu garantir a worktree de onde você iniciou, mais comumente porque você iniciou de dentro dela; a vinculação é mantida. O resto da mensagem nomeia a correção; corresponda-a em [Claude Code recusa usar uma worktree](#claude-code-refuses-to-use-a-worktree).                                                                                                                                                                                   |

Em [modo não interativo](/docs/pt/headless) com `-p`, e em retomas que o [Agent SDK](/docs/pt/agent-sdk/sessions) executa, Claude Code para a retoma com um erro stderr para cada recusa exceto uma worktree desaparecida, em vez de continuar sem isolamento.

Com `--output-format stream-json`, a recusa também chega em stdout como uma mensagem `result` com subtipo `error_during_execution` cujo array `errors` carrega o mesmo texto, para que uma aplicação Agent SDK receba a razão em vez de apenas uma saída não-zero. Antes da v2.1.260, uma recusa de retoma de worktree não produzia nenhuma mensagem `result`.

As mensagens tomam formas diferentes das mensagens interativas na tabela:

* `Error: cannot resume into worktree <path>: ...This session was not started.` para uma recusa que a tabela mostra como `Did not re-enter`. Claude Code limpa a vinculação de worktree antes de sair, e o erro diz assim; a próxima vez que você retomar a conversa, a sessão continua no diretório atual sem isolamento de worktree. Antes da v2.1.260, Claude Code não escrevia a vinculação limpa, então cada retentativa da mesma retoma falhava com o mesmo erro.

  Se você [suprimir escritas de transcrição](/docs/pt/sessions#where-transcripts-are-stored), a limpeza não pode ser salva. O erro então diz que o mesmo comando será recusado novamente, e nomeia `--fork-session` e iniciar uma nova conversa como formas de continuar sem a worktree.
* `Error: could not verify worktree <path> for this resume, so the resume was aborted...` para `Could not verify`
* `Error: ...The worktree binding is kept.` para `Could not re-enter`
* `Notice: the worktree <path> for this session no longer exists...` para uma worktree desaparecida; Claude Code imprime-a e continua a sessão, como uma retoma interativa faz

O final de recusa incorporado em cada erro é compartilhado com os avisos interativos, então ainda corresponde à sua entrada em [Claude Code recusa usar uma worktree](#claude-code-refuses-to-use-a-worktree).

No resultado stream-json, [`startup_failure_reason`](/docs/pt/agent-sdk/typescript#startup_failure_reason) é `worktree_unverified` para o erro `could not verify worktree` e `worktree_resume_refused` para os erros `cannot resume into worktree` e `The worktree binding is kept`. Uma aplicação pode ramificar-se nele em vez de corresponder ao texto do erro. Antes da v2.1.274, o resultado não carregava nenhum campo `startup_failure_reason`.

<h2 id="see-also">
  Veja também
</h2>

Worktrees lidam com isolamento de arquivo. As páginas relacionadas abaixo cobrem delegação de trabalho para esses checkouts isolados, passagem de descobertas entre eles, e alternância entre as sessões que você cria:

* [Subagentes](/docs/pt/sub-agents): delegue trabalho para agentes isolados dentro de uma sessão
* [Mensagens entre sessões](/docs/pt/cross-session-messaging): deixe as sessões em suas worktrees passarem descobertas uma para a outra
* [Equipes de agentes](/docs/pt/agent-teams): coordene múltiplas sessões do Claude automaticamente
* [Gerencie sessões](/docs/pt/sessions): nomeie, retome, e alterne entre conversas
* [Sessões paralelas do desktop](/docs/pt/desktop#work-in-parallel-with-sessions): sessões apoiadas por worktree no aplicativo desktop
