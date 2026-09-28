> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estender Claude com skills

> Crie, gerencie e compartilhe skills para estender as capacidades do Claude no Claude Code. Inclui comandos personalizados e skills agrupadas.

Skills estendem o que Claude pode fazer. Crie um arquivo `SKILL.md` com instruções, e Claude o adiciona ao seu kit de ferramentas. Claude usa skills quando relevante, ou você pode invocar uma diretamente com `/skill-name`.

Crie uma skill quando você fica colando as mesmas instruções, checklist ou procedimento de múltiplas etapas no chat, ou quando uma seção de CLAUDE.md cresceu e se tornou um procedimento em vez de um fato. Diferentemente do conteúdo de CLAUDE.md, o corpo de uma skill é carregado apenas quando é usado, então material de referência longo custa quase nada até que você precise dele.

<Note>
  Para comandos integrados como `/help` e `/compact`, e skills agrupadas como `/debug` e `/code-review`, consulte a [referência de comandos](/docs/pt/commands).

  **Comandos personalizados foram mesclados em skills.** Um arquivo em `.claude/commands/deploy.md` e uma skill em `.claude/skills/deploy/SKILL.md` ambos criam `/deploy` e funcionam da mesma forma. Seus arquivos `.claude/commands/` existentes continuam funcionando. Skills adicionam recursos opcionais: um diretório para arquivos de suporte, frontmatter para [controlar se você ou Claude os invoca](#control-who-invokes-a-skill), e a capacidade de Claude carregá-los automaticamente quando relevante.
</Note>

As skills do Claude Code seguem o padrão aberto [Agent Skills](https://agentskills.io), que funciona em múltiplas ferramentas de IA. Claude Code estende o padrão com recursos adicionais como [controle de invocação](#control-who-invokes-a-skill), [execução de subagent](#run-skills-in-a-subagent), e [injeção de contexto dinâmico](#inject-dynamic-context). Consulte [Usando frontmatter de skill fora do Claude Code](#using-skill-frontmatter-outside-claude-code) para saber quais campos de frontmatter fazem parte do padrão e quais são extensões do Claude Code.

<h2 id="bundled-skills">
  Skills agrupadas
</h2>

Claude Code inclui um conjunto de skills agrupadas, como `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop` e `/claude-api`. Skills agrupadas são baseadas em prompt: elas fornecem ao Claude instruções detalhadas e permitem que ele orquestre o trabalho usando suas ferramentas. A maioria dos comandos integrados executa lógica fixa diretamente.

Você invoca uma skill agrupada da mesma forma que qualquer outra skill, digitando `/` seguido do nome da skill. Claude invoca algumas skills agrupadas automaticamente quando relevante; outras, incluindo `/verify`, são executadas apenas quando você as invoca, o que mantém você no controle de quando essas verificações de execução mais longa gastam tempo e tokens.

A maioria das skills agrupadas está disponível em todas as sessões. Algumas dependem de um recurso específico: `/workflow-authoring`, por exemplo, está disponível apenas quando [fluxos de trabalho dinâmicos](/docs/pt/workflows) estão habilitados.

Para desativar skills agrupadas, use a configuração [`disableBundledSkills`](/docs/pt/settings-reference#disablebundledskills).

<Note>
  A verificação de configuração [`/doctor`](/docs/pt/commands#all-commands) permanece digitável quando `disableBundledSkills` está ativado, no Claude Code v2.1.205 e posterior. Para ocultá-la, defina a variável de ambiente `DISABLE_DOCTOR_COMMAND` ou uma entrada [`skillOverrides`](#override-skill-visibility-from-settings) de `"doctor": "off"`. Antes da v2.1.205, `/doctor` era um comando integrado em vez de uma skill agrupada.
</Note>

Skills agrupadas são listadas junto com comandos integrados na [referência de comandos](/docs/pt/commands), marcadas como **Skill** na coluna Propósito.

<h3 id="run-and-verify-your-app">
  Execute e verifique seu aplicativo
</h3>

Três skills agrupadas trabalham juntas para iniciar seu aplicativo e confirmar alterações em relação ao aplicativo em execução em vez de apenas testes:

| Skill                  | Propósito                                                                                                                                    |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `/run`                 | Inicie e conduza seu aplicativo para ver uma alteração funcionando                                                                           |
| `/verify`              | Compile e execute seu aplicativo para confirmar que uma alteração de código faz o que deveria, sem recorrer a testes ou verificações de tipo |
| `/run-skill-generator` | Ensine ao `/run` e `/verify` como compilar e iniciar seu projeto                                                                             |

`/run` e `/verify` funcionam sem configuração. Eles inferem o lançamento do tipo de seu projeto (CLI, servidor, TUI, orientado por navegador) e do que está em seu README, `package.json` ou `Makefile`. Essa inferência se torna pouco confiável para projetos que precisam de algo além de um lançamento padrão: um banco de dados, um arquivo env, uma sessão gráfica, uma compilação em várias etapas.

`/run-skill-generator` registra a receita em vez disso. Ele coloca seu aplicativo em execução a partir de um ambiente limpo, captura o que funcionou (os comandos de instalação, as variáveis de ambiente, o script de lançamento) e o confirma como uma skill por projeto em `.claude/skills/run-<name>/`. Depois disso, `/run`, `/verify` e qualquer outro agente no repositório seguem a receita registrada em vez de redescobri-la. Execute `/run-skill-generator` uma vez por projeto e novamente se o processo de compilação ou lançamento mudar.

`/verify` também pode registrar sua própria receita. Quando ele precisa compilar e conduzir seu aplicativo sem uma receita registrada, ele escreve o que funcionou em `.claude/skills/verify/SKILL.md` na raiz do repositório, ou no diretório de pacote tocado em um monorepo, para que execuções posteriores e outros agentes sigam as mesmas etapas. Na raiz do repositório, a skill registrada substitui o `/verify` agrupado. Isso requer Claude Code v2.1.200 ou posterior.

Claude edita o arquivo registrado apenas quando direcionou uma execução incorretamente, como um comando que falhou ou uma etapa ausente, para que você possa confirmar o arquivo sem diffs por sessão. Antes da v2.1.205, a skill agrupada dizia ao Claude para incorporar qualquer coisa que uma execução aprendesse, o que causava conflitos de mesclagem frequentes.

<h2 id="getting-started">
  Primeiros passos
</h2>

<h3 id="create-your-first-skill">
  Crie sua primeira skill
</h3>

Este exemplo cria uma skill que resume as alterações não confirmadas em seu repositório git e sinaliza qualquer coisa arriscada. Ele puxa o diff ao vivo para o prompt antes de Claude lê-lo, para que a resposta seja fundamentada em sua árvore de trabalho real em vez do que Claude pode adivinhar a partir de arquivos abertos. Claude carrega a skill automaticamente quando você pergunta sobre suas alterações, ou você pode invocá-la diretamente com `/summarize-changes`.

<Steps>
  <Step title="Crie o diretório da skill">
    Crie um diretório para a skill em sua pasta de skills pessoais. Skills pessoais estão disponíveis em todos os seus projetos.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Escreva SKILL.md">
    Toda skill precisa de um arquivo `SKILL.md` com duas partes: frontmatter YAML entre marcadores `---` que diz a Claude quando usar a skill, e conteúdo markdown com as instruções que Claude segue quando a skill é executada. O nome do diretório se torna o comando que você digita, e a `description` ajuda Claude a decidir quando carregar a skill automaticamente.

    Salve isto em `~/.claude/skills/summarize-changes/SKILL.md`:

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    A linha `` !`git diff HEAD` `` usa [injeção de contexto dinâmico](#inject-dynamic-context): Claude Code executa o comando e substitui a linha por sua saída antes de Claude ver o conteúdo da skill, para que as instruções cheguem com o diff atual já embutido.
  </Step>

  <Step title="Teste a skill">
    Abra um projeto git, faça uma pequena edição em qualquer arquivo e inicie Claude Code executando `claude`. Você pode testar a skill de duas maneiras.

    **Deixe Claude invocá-la automaticamente** perguntando algo que corresponda à descrição:

    ```text theme={null}
    What did I change?
    ```

    **Ou invoque-a diretamente** com o nome da skill:

    ```text theme={null}
    /summarize-changes
    ```

    De qualquer forma, Claude deve responder com um breve resumo de sua edição e uma lista de riscos.
  </Step>
</Steps>

<h2 id="where-skills-live">
  Escolha onde as skills carregam
</h2>

Onde você salva uma skill decide quais sessões a carregam. Salve-a no seu diretório inicial para obtê-la em todos os projetos, confirme-a em um repositório para compartilhá-la com todos que trabalham lá, ou distribua-a através de um plugin ou configurações gerenciadas para alcançar um time inteiro.

| Localização         | Caminho                                                                                                                      | Carrega em                                                                                                                                                                                                               |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Enterprise          | `.claude/skills/<skill-name>/SKILL.md` no [diretório de configurações gerenciadas](/docs/pt/managed-settings#delivery-mechanisms) | Todos os usuários em máquinas onde sua organização a implanta                                                                                                                                                            |
| Personal            | `~/.claude/skills/<skill-name>/SKILL.md`                                                                                     | Todos os seus projetos nesta máquina, mas não em [sessões Cowork ou cloud](#skills-in-cowork-and-cloud-sessions)                                                                                                         |
| Project             | `.claude/skills/<skill-name>/SKILL.md`                                                                                       | Sessões neste repositório. Confirme-a para que seu time também a obtenha                                                                                                                                                 |
| Nested              | `<subdir>/.claude/skills/<skill-name>/SKILL.md`                                                                              | Sessões iniciadas em ou abaixo de `<subdir>`. Uma sessão iniciada acima dela carrega a skill uma vez que Claude trabalha em arquivos lá. Veja [monorepos e subdiretórios](#discovery-from-parent-and-nested-directories) |
| Diretório adicional | `.claude/skills/<skill-name>/SKILL.md` em um diretório que você passa com `--add-dir`                                        | Essa sessão. Veja [diretórios fora do projeto](#skills-from-additional-directories)                                                                                                                                      |
| Plugin              | `<plugin>/skills/<skill-name>/SKILL.md`                                                                                      | Onde quer que o [plugin](/docs/pt/plugins/overview) esteja habilitado, como `/plugin-name:skill-name`                                                                                                                         |
| Conta claude.ai     | Skills habilitadas para sua conta claude.ai                                                                                  | Sessões Cowork, sessões cloud e sessões de terminal onde você entra com essa conta. Veja [Skills sincronizadas do claude.ai](#how-synced-skills-behave)                                                                  |

As pastas de skill também seguem estas regras:

* **Pastas com symlink**: uma entrada `<skill-name>` na localização enterprise, personal ou project pode ser um symlink para um diretório em outro lugar no disco. Claude Code lê `SKILL.md` do alvo e carrega a skill uma vez mesmo que vários locais apontem para o mesmo alvo. Skills de plugin [lidam com symlinks de forma diferente](/docs/pt/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks).
* **Nome reservado**: não nomeie uma pasta de skill como `synced`, em qualquer capitalização. Claude Code usa `~/.claude/skills/synced/` para [skills baixadas do claude.ai](#where-synced-skills-load) e pula uma skill que você cria com esse nome nas localizações enterprise, personal e project.
* **Arquivos de comando**: um arquivo Markdown em `.claude/commands/` é o formato mais antigo e ainda funciona. Ele suporta o mesmo [frontmatter](#frontmatter-reference) exceto `name` e `paths`. Para encontrar o nome que você digita para invocá-lo, veja [Como uma skill obtém seu nome de comando](#how-a-skill-gets-its-command-name). Prefira uma skill para novo trabalho, já que skills também suportam [arquivos de suporte](#add-supporting-files).
* **Pasta de skill como um plugin**: adicione um `.claude-plugin/plugin.json` a uma pasta de skill e ela carrega como um [plugin](/docs/pt/plugins/loading#plugins-shared-through-a-repository) nomeado `<name>@skills-dir`, para que possa agrupar agents, hooks e servidores MCP. Em um `.claude/skills/` de projeto, isso requer aceitar primeiro o diálogo de confiança do workspace.

<h3 id="discovery-from-parent-and-nested-directories">
  Carregue skills em monorepos e subdiretórios
</h3>

Claude Code carrega skills de projeto de `.claude/skills/` no diretório onde você o inicia e em todos os diretórios pai até a raiz do repositório, então iniciar em `packages/frontend/` ainda pega skills definidas na raiz. Quando você [move a sessão com `/cd`](/docs/pt/permissions#move-the-session-to-another-directory) na v2.1.246 ou posterior, Claude Code adiciona as skills de projeto do novo diretório.

Em uma sessão executada em um [git worktree](/docs/pt/worktrees) vinculado, Claude Code pesquisa diretórios pai apenas até a raiz do worktree. No Claude Code v2.1.277 ou posterior, quando o checkout do worktree não tem um diretório `.claude/skills` em sua raiz, Claude Code carrega as skills de projeto do checkout principal. Veja [O que worktrees compartilham com o checkout principal](/docs/pt/worktrees#what-worktrees-share-with-the-main-checkout).

Skills em um diretório `.claude/skills/` abaixo de onde você iniciou não carregam na inicialização. Elas carregam na primeira vez que Claude lê ou edita um arquivo naquele subdiretório e permanecem disponíveis pelo resto da sessão. Até então elas não aparecem no menu `/` e você não pode invocá-las por nome. Para carregá-las mais cedo, execute `/add-dir` com o caminho do subdiretório, o que requer Claude Code v2.1.257 ou posterior.

Quando uma skill aninhada compartilha um nome com outra skill, ambas permanecem disponíveis. Com uma skill `deploy` na raiz do repositório e outra em `apps/web/.claude/skills/`:

* `/deploy` executa a skill da raiz. Claude Code também lista as variantes qualificadas por diretório para Claude, com uma instrução para invocar aquela cujo diretório contém os arquivos em que está trabalhando, então a skill aninhada ainda se aplica ao trabalho em `apps/web/`.
* `/apps/web:deploy` executa a skill aninhada por conta própria. Sua descrição nomeia o diretório ao qual se aplica.

<h3 id="skills-from-additional-directories">
  Carregue skills de um diretório fora do projeto
</h3>

Quando você adiciona um diretório com `--add-dir` ou `/add-dir`, Claude Code carrega as skills no `.claude/skills/` daquele diretório, junto com seu `.claude/commands/` e `.claude/agents/`. Diretórios que o Agent SDK adiciona através de [`additionalDirectories`](/docs/pt/agent-sdk/typescript#options) em TypeScript ou [`add_dirs`](/docs/pt/agent-sdk/python#claudeagentoptions) em Python carregam da mesma forma, porque o SDK os passa como `--add-dir`. A configuração `permissions.additionalDirectories` em `settings.json` concede apenas acesso a arquivos e não carrega nenhum destes.

Claude Code observa `.claude/skills/` em um diretório que você passa com `--add-dir` na inicialização, como [Edite uma skill durante uma sessão](#live-change-detection) descreve. Ele não observa o `.claude/commands/` ou `.claude/agents/` do diretório adicionado, então reinicie a sessão após alterar um arquivo lá.

Esses carregamentos dependem da [fonte de configuração](/docs/pt/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) `project`, que está ativada por padrão. Uma política [`strictPluginOnlyCustomization`](/docs/pt/settings-reference#strictpluginonlycustomization), [modo bare](/docs/pt/headless#start-faster-with-bare-mode) e [`--safe-mode`](/docs/pt/cli-reference#cli-flags) cada uma as restringe ainda mais, como essas páginas descrevem. Veja [Diretórios adicionais concedem acesso a arquivos, não configuração](/docs/pt/permissions#additional-directories-grant-file-access-not-configuration) para a tabela completa do que um diretório adicionado carrega, incluindo `CLAUDE.md` e configurações de plugin.

<h3 id="resolve-skills-that-share-a-name">
  Resolva skills que compartilham um nome
</h3>

Quando duas skills compartilham um nome, de onde cada uma veio decide qual `/name` executa. A tabela cobre as localizações enterprise, personal, project, nested, plugin e claude.ai, skills agrupadas e arquivos de comando:

| Mesmo nome em                                                                                       | Qual executa                                                                                                                                                                                                             |
| :-------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Dois de enterprise, personal e project                                                              | Enterprise sobre personal, e personal sobre project. Com `deploy` em ambos `~/.claude/skills/` e o `.claude/skills/` do projeto, `/deploy` executa a pessoal                                                             |
| Qualquer uma dessas localizações e uma [skill agrupada](#bundled-skills)                            | Sua skill substitui o comando agrupado, mas não seus aliases. Uma skill `code-review` de projeto substitui `/code-review`, e o alias agrupado `/review` nunca executa sua skill                                          |
| Uma skill e um arquivo em `.claude/commands/`                                                       | A skill                                                                                                                                                                                                                  |
| Uma skill de raiz de projeto e uma skill aninhada                                                   | Ambas carregam. Veja [monorepos e subdiretórios](#discovery-from-parent-and-nested-directories)                                                                                                                          |
| Uma skill de plugin e uma skill em qualquer uma das localizações acima                              | Ambas carregam, porque skills de plugin são nomeadas como `/plugin-name:skill-name`                                                                                                                                      |
| Qualquer uma das acima e uma skill [sincronizada de sua conta claude.ai](#how-synced-skills-behave) | A outra skill ou comando. A skill sincronizada ainda executa como `/anthropic-skills:<name>`. Veja [Quando um nome de skill sincronizada corresponde a outro comando](#when-a-synced-skill-name-matches-another-command) |

<h3 id="skills-in-cowork-and-cloud-sessions">
  Use skills em sessões Cowork e cloud
</h3>

Sessões [Cowork](https://claude.com/product/cowork) e [sessões cloud](/docs/pt/cloud-environments#what-carries-over-from-your-setup), incluindo [rotinas](/docs/pt/routines), não leem `~/.claude/skills/` em sua máquina. Tanto sessões Cowork interativas quanto agendadas carregam as skills habilitadas para sua conta claude.ai, sincronizadas no início da sessão; gerencie-as em **Customize** na barra lateral do aplicativo Desktop ou nas configurações de skills no claude.ai. Sessões cloud adicionalmente carregam skills de projeto confirmadas no `.claude/skills/` do repositório clonado.

Se uma skill existe apenas em `~/.claude/skills/` em sua máquina, Claude Code relata que a skill não foi encontrada quando uma [rotina](/docs/pt/routines) a invoca, porque cada execução de rotina inicia como uma sessão cloud nova. Para disponibilizar uma skill pessoal nessas sessões:

* Para sessões Cowork e cloud, habilite a skill para sua conta claude.ai.
* Para sessões cloud, você pode em vez disso confirmar a skill no `.claude/skills/` do repositório. Plugins declarados no `.claude/settings.json` do repositório e plugins habilitados apenas em suas configurações de usuário [não carregam em sessões cloud](/docs/pt/cloud-environments#what-carries-over-from-your-setup).

[Tarefas agendadas do Desktop](/docs/pt/desktop-scheduled-tasks) executam localmente em sua máquina, então elas carregam `~/.claude/skills/`.

<h3 id="how-synced-skills-behave">
  Skills sincronizadas do claude.ai
</h3>

Esta seção se aplica a você se usar sessões Cowork ou cloud, ou entrar no Claude Code em seu terminal com uma conta claude.ai. Nessas sessões, Claude Code carrega as skills habilitadas para sua conta claude.ai, sem nenhuma configuração da sua parte, como [Onde as skills sincronizadas carregam](#where-synced-skills-load) descreve. Essas skills incluem aquelas que você cria ou ativa em suas configurações claude.ai, skills que sua organização fornece lá, e skills integradas da Anthropic como `pdf` e `xlsx`.

Claude Code baixa uma skill sincronizada de sua conta em vez de ler um arquivo que você escreveu na máquina onde a sessão executa, então aplica regras a skills sincronizadas que não se aplicam às skills que você armazena nas [localizações de skills](#where-skills-live).

<h4 id="where-synced-skills-load">
  Onde as skills sincronizadas carregam
</h4>

Em uma sessão Cowork ou cloud, Claude Code carrega as skills habilitadas para sua conta claude.ai, e [Skills em sessões Cowork e cloud](#skills-in-cowork-and-cloud-sessions) diz como escolher quais skills essas sessões obtêm.

Em seu terminal, Claude Code sincroniza essas skills em sessões onde você entra com sua conta claude.ai. Quando a sessão inicia, Claude Code baixa as skills de sua conta em `~/.claude/skills/synced/` em segundo plano, então verifica claude.ai para mudanças a cada 10 minutos enquanto a sessão executa. Quando uma verificação encontra que uma skill foi adicionada, editada ou desativada no claude.ai, Claude Code a adiciona, atualiza ou remove na sessão em execução sem uma reinicialização. A sincronização em sessões de terminal requer Claude Code v2.1.273 ou posterior.

A sincronização nunca atrasa a inicialização, porque Claude aguarda o download de uma skill apenas quando a invoca. Uma execução [não-interativa](/docs/pt/headless) curta pode portanto terminar antes que uma skill recém-adicionada baixe, caso em que uma sessão posterior a baixa. Para fazer uma execução não-interativa baixar suas skills e aguardar a lista antes de responder ao prompt, defina [`CLAUDE_CODE_SYNC_SKILLS`](/docs/pt/env-vars#variables) como `1`.

Claude Code sincroniza apenas em uma sessão que entra com sua conta claude.ai e [busca sinalizadores de recurso da Anthropic](/docs/pt/env-vars#features-that-need-feature-flag-fetching). Ele não sincroniza nessas sessões:

* Uma sessão que não usa um sign-in armazenado por `/login`, como uma que autentica com uma chave de API, ou uma onde `ANTHROPIC_AUTH_TOKEN`, `CLAUDE_CODE_OAUTH_TOKEN` ou um script `apiKeyHelper` fornece a credencial
* Uma sessão que não busca sinalizadores de recurso, como uma no Amazon Bedrock ou uma onde você define `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* Uma sessão em [modo bare](/docs/pt/headless#start-faster-with-bare-mode) ou uma que você inicia com `--safe-mode`
* Uma sessão onde as configurações gerenciadas de sua organização [bloqueiam skills para fontes de plugin](/docs/pt/settings-reference#strictpluginonlycustomization-skills), ou uma que você inicia com uma lista [`--setting-sources`](/docs/pt/cli-reference#cli-flags) que deixa de fora `user`

Se você entrar com `/login` durante uma sessão, reinicie Claude Code para começar a sincronizar.

Skills que uma sessão anterior sincronizou permanecem no disco. Claude Code as carrega em sessões posteriores conectadas à mesma conta, mesmo quando não consegue alcançar claude.ai.

Claude Code baixa skills sincronizadas e nunca as carrega. Se você ou Claude editar um arquivo sob `~/.claude/skills/synced/`, a alteração não é salva em sua conta claude.ai, e uma sincronização posterior pode sobrescrevê-la ou removê-la. Para alterar uma skill sincronizada, atualize-a no claude.ai; a próxima sincronização baixa a nova versão.

Para ver quais skills sincronizaram, execute `/skills`. O menu as lista sob `claude.ai sync`.

Algumas skills da Anthropic, como `pdf` e `xlsx`, sempre sincronizam. Para o resto, ative ou desative uma skill em suas configurações de skills no claude.ai para alterar se ela sincroniza.

Para parar de sincronizar em uma máquina, defina [`syncClaudeAiSkills`](/docs/pt/settings-reference#syncclaudeaiskills) como `false` em suas configurações de usuário. Claude Code para de baixar, e na próxima vez que inicia move as skills que já sincronizou para `~/.claude/skills/.trash/` e não as carrega mais. Sua organização pode desativar a sincronização para todos desativando Skills no claude.ai. Para parar de sincronizar deixando Skills ativado, pode definir a mesma chave em [configurações gerenciadas](/docs/pt/managed-settings).

Se sua organização desativar Skills no claude.ai, Claude Code remove as skills baixadas e elas param de carregar. As skills removidas se movem para `~/.claude/skills/.trash/`, onde você pode recuperar os arquivos até que a [varredura de retenção](/docs/pt/claude-directory#cleaned-up-automatically) os delete. Uma vez que sua organização ativa Skills novamente, Claude Code baixa as skills que você habilitou na próxima sincronização.

<h4 id="when-a-synced-skill-name-matches-another-command">
  Quando um nome de skill sincronizada corresponde a outro comando
</h4>

Você pode invocar uma skill sincronizada por seu nome completo, `/anthropic-skills:<name>`, ou por seu nome curto, `/<name>`. Quando outro comando usa esse nome curto, `/<name>` executa o outro comando, e a skill sincronizada executa apenas como `/anthropic-skills:<name>`. Com uma skill `deploy` local e uma `deploy` sincronizada, `/deploy` executa a skill local e `/anthropic-skills:deploy` executa a sincronizada. Antes da v2.1.269, uma skill sincronizada tinha apenas seu nome curto.

O outro comando pode ser qualquer um destes:

* Um comando integrado ou uma [skill agrupada](#bundled-skills), incluindo uma que está indisponível em sua sessão, por exemplo após desativar skills agrupadas
* Uma skill em qualquer [nível local](#where-skills-live) ou um arquivo em `.claude/commands/`
* Uma skill de plugin
* Um [prompt MCP](/docs/pt/mcp#use-mcp-prompts-as-commands)

Claude Code rotula skills sincronizadas para que você possa dizer de onde vieram. O menu `/skills` e `/context` agrupam skills sincronizadas sob `claude.ai sync`, e o menu de comando `/` as marca como vindo do claude.ai.

Quando compara nomes, Claude Code ignora maiúsculas, espaçamento e caracteres invisíveis, e trata formas de compatibilidade como letras de largura completa e variantes de travessão como seus equivalentes simples. Por exemplo, uma skill sincronizada nomeada `Commit` e uma skill local nomeada `commit` contam como o mesmo nome, então `/commit` continua executando sua skill local.

Um nome que difere apenas por uma letra semelhante de outro alfabeto conta como um nome diferente, e o rótulo `claude.ai sync` é como você diferencia os dois. Essas verificações e rótulos requerem Claude Code v2.1.228 ou posterior.

<h4 id="how-claude-code-handles-the-frontmatter-of-a-synced-skill">
  Como Claude Code lida com o frontmatter de uma skill sincronizada
</h4>

Claude Code aplica duas regras ao frontmatter de uma skill sincronizada:

* Claude Code honra o frontmatter em todo tipo de sessão, então uma concessão `allowed-tools` passa pelo [fluxo de permissão](/docs/pt/permissions) normal.
* Claude Code sanitiza o texto de exibição que a skill fornece, como sua descrição. Remove caracteres de controle, e em texto que alcança Claude, como a descrição, também escapa colchetes angulares para que o texto não possa imitar a formatação interna do Claude Code. Esta sanitização requer Claude Code v2.1.228 ou posterior.

<h4 id="how-claude-code-handles-the-body-of-a-synced-skill">
  Como Claude Code lida com o corpo de uma skill sincronizada
</h4>

O que Claude Code faz com o corpo de uma skill sincronizada depende de onde a sessão executa:

* Em uma sessão cloud, o corpo mantém o comportamento que uma skill local tem, porque a sessão executa em um contêiner isolado.
* Em uma sessão Cowork em seu desktop, o corpo mantém o comportamento que uma skill local tem, exceto que Claude Code substitui cada linha de comando `!` pelo placeholder [`disableSkillShellExecution`](#inject-dynamic-context), como faz para toda skill que você fornece lá.
* Em qualquer outra sessão em sua máquina, Claude Code não executa [comandos `!`](#inject-dynamic-context), não anexa os arquivos que referências `@` nomeiam da forma que faz para uma skill local, e não substitui os placeholders `${CLAUDE_PROJECT_DIR}` e `${CLAUDE_SESSION_ID}`, então as referências `@` e ambos os placeholders alcançam Claude como texto literal. Uma linha de comando `!` alcança Claude como texto literal também, ou como esse placeholder quando `disableSkillShellExecution` está ativado. Este tratamento requer Claude Code v2.1.228 ou posterior.

<h3 id="live-change-detection">
  Edite uma skill durante uma sessão
</h3>

Claude Code observa diretórios de skill para mudanças de arquivo, exceto em [modo bare](/docs/pt/headless#start-faster-with-bare-mode). Quando você adiciona, edita ou remove uma skill sob `~/.claude/skills/`, o `.claude/skills/` do projeto, ou um `.claude/skills/` dentro de um diretório `--add-dir`, Claude Code pega a mudança dentro da sessão atual, sem uma reinicialização. Se você criar um diretório de skills de nível superior que não existia quando a sessão iniciou, reinicie Claude Code para que possa observar o novo diretório.

A detecção de mudança ao vivo cobre apenas texto `SKILL.md`. Para uma pasta de skill que também é um [plugin](/docs/pt/plugins/loading#plugins-shared-through-a-repository), mudanças em `hooks/`, `.mcp.json`, `agents/` e `output-styles/` precisam de `/reload-plugins` para entrar em vigor.

<h3 id="remove-a-skill">
  Remova uma skill
</h3>

Como você remove uma skill depende de onde ela veio:

* **Skill pessoal ou de projeto**: delete o diretório da skill, `~/.claude/skills/<skill-name>/` ou `.claude/skills/<skill-name>/`. Claude Code a [remove de `/skills` na sessão atual](#live-change-detection); conteúdo que Claude Code já carregou dela segue o [ciclo de vida do conteúdo da skill](#skill-content-lifecycle).
* **Skill enterprise**: um administrador deleta o diretório da skill de `.claude/skills/` dentro do [diretório de configurações gerenciadas](/docs/pt/managed-settings#delivery-mechanisms), por exemplo `/etc/claude-code/.claude/skills/<skill-name>/` no Linux.
* **Skill de plugin**: desabilite ou desinstale o plugin que a fornece, do menu `/plugin` ou com `/plugin uninstall <plugin-name>@<marketplace-name>`. Claude Code descarrega as skills do plugin quando [a mudança se aplica](/docs/pt/plugins/cli-reference#reload-plugins) ou quando você reinicia.
* **Skill sincronizada do claude.ai**: desative a skill para sua conta claude.ai, no mesmo lugar onde você a [habilitou](#skills-in-cowork-and-cloud-sessions). Claude Code a remove de `~/.claude/skills/synced/` na próxima vez que [sincroniza suas skills](#where-synced-skills-load). Se você deletar o diretório manualmente, a próxima sincronização o baixa novamente enquanto a skill permanece habilitada no claude.ai.
* **Skill agrupada**: defina [`disableBundledSkills`](#bundled-skills) como `true` para desativar skills agrupadas, ou defina uma skill como `"off"` em [`skillOverrides`](#override-skill-visibility-from-settings) para ocultá-la.

Para manter uma skill pessoal ou de projeto mas parar Claude de invocá-la por conta própria, defina [`disable-model-invocation: true`](#control-who-invokes-a-skill) em seu frontmatter, ou `"user-invocable-only"` em [`skillOverrides`](#override-skill-visibility-from-settings) quando você não quer editar o arquivo.

<h2 id="configure-skills">
  Configurar skills
</h2>

Skills são configuradas através de frontmatter YAML no topo de `SKILL.md` e no conteúdo markdown que segue.

<h3 id="types-of-skill-content">
  Tipos de conteúdo de skill
</h3>

Arquivos de skill podem conter qualquer instrução, mas pensar em como você quer invocá-los ajuda a guiar o que incluir:

**Conteúdo de referência** adiciona conhecimento que Claude aplica ao seu trabalho atual. Convenções, padrões, guias de estilo, conhecimento de domínio. Este conteúdo é executado inline para que Claude possa usá-lo junto com seu contexto de conversa.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**Conteúdo de tarefa** fornece a Claude instruções passo a passo para uma ação específica, como deployments, commits ou geração de código. Estas são frequentemente ações que você quer invocar diretamente com `/skill-name` em vez de deixar Claude decidir quando executá-las. Adicione `disable-model-invocation: true` para evitar que Claude a dispare automaticamente. O exemplo abaixo adiciona `context: fork`, que executa a skill em seu próprio contexto de subagent; veja [Executar skills em um subagent](#run-skills-in-a-subagent).

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

Mantenha o corpo em si conciso. Uma vez que uma skill é carregada, seu conteúdo [permanece em contexto entre turnos](#skill-content-lifecycle), então cada linha é um custo de token recorrente. Declare o que fazer em vez de narrar como ou por que, e aplique o mesmo teste de concisão que você faria para [conteúdo CLAUDE.md](/docs/pt/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Referência de frontmatter
</h3>

Configure uma skill com YAML [frontmatter](/docs/pt/glossary#frontmatter) entre marcadores `---` no topo de `SKILL.md`, e escreva as instruções da skill como Markdown após o `---` de fechamento. Os nomes de campo usam palavras minúsculas separadas por hífens, exceto `when_to_use`. Um [arquivo de comando](#where-skills-live) em `.claude/commands/` aceita os mesmos campos exceto `name` e `paths`. Este exemplo define quatro campos:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Todos os campos são opcionais. Apenas `description` é recomendado para que Claude saiba quando usar a skill. Um nome de campo deve corresponder exatamente à tabela, hífens inclusos: Claude Code ignora um campo que não reconhece sem relatar um erro.

Claude Code lê o frontmatter apenas quando a abertura `---` é a primeira linha do arquivo. Caso contrário, trata o arquivo inteiro, incluindo marcadores `---`, como conteúdo de skill. Se o YAML entre os marcadores não for analisado, a skill ainda carrega sem campos definidos; veja [Skill não disparando](#skill-not-triggering) para encontrar e corrigir o erro.

Campos booleanos aceitam `yes`, `no`, `on`, `off`, `1` e `0` em qualquer caso de letra, além de `true` e `false`. Antes da v2.1.218, Claude Code reconhecia apenas `true` e `false`.

| Campo                      | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Não         | Nome de exibição mostrado nas listagens de skills. Padrão é o nome do diretório. Veja [Como uma skill obtém seu nome de comando](#how-a-skill-gets-its-command-name) para como o campo interage com o nome que você digita para invocar a skill.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `description`              | Recomendado | O que a skill faz e quando usá-la. Claude usa isso para decidir quando aplicar a skill. Se omitido, usa a primeira linha não vazia do conteúdo markdown. Coloque o caso de uso principal primeiro: o texto combinado de `description` e `when_to_use` é truncado em 1.536 caracteres na listagem de skills para reduzir o uso de contexto.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `when_to_use`              | Não         | Contexto adicional para quando Claude deve invocar a skill, como frases de gatilho ou solicitações de exemplo. Anexado a `description` na listagem de skills e conta para o limite de 1.536 caracteres.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `argument-hint`            | Não         | Dica mostrada durante o autocomplete para indicar argumentos esperados. Exemplo: `[issue-number]` ou `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `arguments`                | Não         | Argumentos posicionais nomeados para [`$name` substitution](#available-string-substitutions) no conteúdo da skill. Aceita uma string separada por espaços ou uma lista YAML. Os nomes mapeiam para posições de argumento em ordem.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `disable-model-invocation` | Não         | Defina como `true` para evitar que Claude carregue automaticamente esta skill. Use para workflows que você quer disparar manualmente com `/name`. Também evita que a skill seja [pré-carregada em subagents](/docs/pt/sub-agents#preload-skills-into-subagents). A partir da v2.1.196, também evita que a skill seja executada quando uma [tarefa agendada](/docs/pt/scheduled-tasks) dispara com a skill como seu prompt. Padrão: `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `user-invocable`           | Não         | Defina como `false` quando apenas Claude deve invocar a skill: Claude Code a oculta do menu `/` e não a executa quando você digita `/name`. Use para conhecimento de fundo que os usuários não devem invocar diretamente. Padrão: `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `allowed-tools`            | Não         | Ferramentas que Claude pode usar sem pedir permissão durante o turno que invoca esta skill. A concessão é limpa quando você envia sua próxima mensagem. Aceita uma string separada por espaço ou vírgula, ou uma lista YAML. Veja [Pré-aprovar ferramentas para uma skill](#pre-approve-tools-for-a-skill).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `disallowed-tools`         | Não         | Ferramentas removidas do pool disponível de Claude enquanto esta skill está ativa. Use para skills autônomas que nunca devem chamar certas ferramentas, como `AskUserQuestion` para um loop de fundo. Aceita uma string separada por espaço ou vírgula, ou uma lista YAML. A restrição é limpa quando você envia sua próxima mensagem. Como regras de negação, o campo não pode remover [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior) enquanto qualquer outra ferramenta permanecer.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `model`                    | Não         | Modelo a usar quando esta skill está ativa. A substituição se aplica pelo resto do turno atual e não é salva nas configurações. O modelo de sessão é retomado quando você envia seu próximo prompt. Aceita os mesmos valores que [`/model`](/docs/pt/model-config), ou `inherit` para manter o modelo ativo. Um valor excluído pela lista de permissões [`availableModels`](/docs/pt/model-config#restrict-model-selection) da sua organização não é usado, e a sessão mantém seu modelo atual. Em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) e em [modo plano enquanto o classificador revisa comandos](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode), um modelo que o modo automático não suporta também não é usado, e a sessão mantém seu modelo atual. Com `context: fork`, o valor define o [modelo do subagent bifurcado](#run-skills-in-a-subagent) em vez disso, e um valor excluído segue as [mesmas regras que uma substituição de modelo de subagent](/docs/pt/model-config#restrict-model-selection). |
| `effort`                   | Não         | [Nível de esforço](/docs/pt/model-config#adjust-effort-level) quando esta skill está ativa. Substitui o nível de esforço da sessão. Padrão: herda da sessão. Opções: `low`, `medium`, `high`, `xhigh`, `max`; os níveis disponíveis dependem do modelo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `context`                  | Não         | Defina como `fork` para executar em um contexto de subagent bifurcado. Veja [Executar skills em um subagent](#run-skills-in-a-subagent).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agent`                    | Não         | Qual tipo de subagent usar quando `context: fork` está definido.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `background`               | Não         | Aplica-se apenas com `context: fork`. Defina como `false` para aguardar o resultado do subagent bifurcado no turno que invocou a skill, em vez de [executá-lo em segundo plano](#run-skills-in-a-subagent). Padrão: `true`. Requer Claude Code v2.1.218 ou posterior.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `hooks`                    | Não         | Hooks que Claude Code registra quando a skill é invocada e continua executando pelo resto da sessão. Veja [Hooks em skills e agents](/docs/pt/hooks#hooks-in-skills-and-agents) para o formato de configuração e a opção `once`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `paths`                    | Não         | Padrões Glob que limitam quando esta skill é ativada. Aceita uma string separada por vírgula ou uma lista YAML. Quando definido, Claude carrega a skill automaticamente apenas ao trabalhar com arquivos que correspondem aos padrões. Usa o mesmo formato que [regras específicas de caminho](/docs/pt/memory#path-specific-rules).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `shell`                    | Não         | Shell a usar para `` !`command` `` e blocos ` ```! ` nesta skill. Aceita `bash` (padrão) ou `powershell`. Definir `powershell` executa comandos shell inline via PowerShell quando a [ferramenta PowerShell](/pt/tools-reference#powershell-tool) está habilitada: está ativada por padrão no Windows sem Git Bash, ativada por padrão com Git Bash para contas claude.ai e Console, e precisa de `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` em sessões Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry e em macOS, Linux e WSL. Defina como `0` para desativar a ferramenta.                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `metadata`                 | Não         | Mapa YAML de forma livre para seus próprios dados de chave-valor, como campos de direito ou catálogo, lidos por sua própria ferramenta a partir de `SKILL.md`. Claude Code não age sobre seu conteúdo e descarta um valor que não é um mapa. Não reutilize nomes de campos de frontmatter como `paths` como chaves.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `license`                  | Não         | Licença que cobre a skill. Parte da especificação [Agent Skills](https://agentskills.io); veja [Usando frontmatter de skill fora do Claude Code](#using-skill-frontmatter-outside-claude-code). Claude Code aceita o campo mas não age sobre ele.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `compatibility`            | Não         | Requisitos de ambiente para a skill, como produtos pretendidos ou pré-requisitos do sistema, conforme definido pela especificação [Agent Skills](https://agentskills.io); veja [Usando frontmatter de skill fora do Claude Code](#using-skill-frontmatter-outside-claude-code). Aceita uma string de até 500 caracteres. Claude Code aceita o campo mas não age sobre ele.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

<h4 id="using-skill-frontmatter-outside-claude-code">
  Usando frontmatter de skill fora do Claude Code
</h4>

Claude Code aceita todos os campos na tabela acima. Fora do Claude Code, você pode usar apenas os campos na especificação [Agent Skills](https://agentskills.io):

| Caminho de distribuição                                                                                                                          | Campos de frontmatter que você pode usar                                       |
| :----------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| Skills do Claude Code em [qualquer nível](#where-skills-live), incluindo skills de [plugin](/docs/pt/plugins/overview)                                | Todos os campos na tabela acima                                                |
| Uploads de skills do claude.ai, a Skills API e empacotamento com `package_skill.py` de [anthropics/skills](https://github.com/anthropics/skills) | `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools` |

Quando você habilita uma skill pessoal para sua conta claude.ai, por exemplo para usá-la em [sessões Cowork e cloud](#skills-in-cowork-and-cloud-sessions) e rotinas, você a carrega no claude.ai, então as mesmas regras se aplicam.

Se você incluir qualquer campo que a especificação não permite, o empacotamento ou upload falha com um erro difícil em vez de ignorar o campo:

```
Unexpected key(s) in SKILL.md frontmatter: argument-hint. Allowed properties are: allowed-tools, compatibility, description, license, metadata, name
```

Restringir o frontmatter aos seis campos da especificação evita o erro de chave inesperada acima. A [especificação Agent Skills](https://agentskills.io) e os [requisitos da Skills API](https://docs.claude.com/en/api/skills-guide) definem tudo mais que esses caminhos validam. Recursos de corpo específicos do Claude Code, como [injeção de contexto dinâmico](#inject-dynamic-context), não funcionam no chat claude.ai ou através da API. Claude Code aceita todos os seis campos, então o frontmatter que segue a especificação carrega no Claude Code sem alterações.

<h4 id="how-a-skill-gets-its-command-name">
  Como uma skill obtém seu nome de comando
</h4>

O comando que você digita para invocar uma skill vem de onde o arquivo de skill vive e, para skills de plugin, também do campo `name` do frontmatter. Em uma skill pessoal ou de projeto, `name` define apenas o rótulo de exibição mostrado nas listagens de skills, e o comando ainda vem do nome do diretório. Em uma skill de plugin, `name` define o último segmento do comando e o prefixo do plugin permanece no lugar.

A tabela abaixo mostra de onde o nome do comando vem para cada layout:

| Local da skill                                                                                              | Fonte do nome do comando                                                                                               | Exemplo                                                                                                                                |
| :---------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| Diretório de skill sob `~/.claude/skills/` ou `.claude/skills/`                                             | Nome do diretório                                                                                                      | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                           |
| [Aninhado](#where-skills-live) diretório `.claude/skills/`, quando o nome entra em conflito com outra skill | Caminho do subdiretório relativo ao diretório de trabalho, depois o nome do diretório de skill                         | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                         |
| Arquivo sob `.claude/commands/`                                                                             | Nome do arquivo sem extensão                                                                                           | `.claude/commands/deploy.md` → `/deploy`                                                                                               |
| Arquivo em um subdiretório de `.claude/commands/`                                                           | Caminho do subdiretório relativo a `commands/` com cada `/` substituído por `:`, depois o nome do arquivo sem extensão | `.claude/commands/frontend/component.md` → `/frontend:component`                                                                       |
| Subdiretório `skills/` do plugin                                                                            | Frontmatter `name` ou o nome do diretório, com namespace pelo plugin                                                   | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`, ou `/my-plugin:fancy` com `name: fancy`                                      |
| `SKILL.md` raiz do plugin                                                                                   | Frontmatter `name`, com o nome do diretório do plugin como fallback                                                    | `my-plugin/SKILL.md` com `name: review` → `/my-plugin:review`. Veja [uma única skill na raiz do plugin](/docs/pt/plugins/components#skills) |
| Skill [sincronizada do claude.ai](#how-synced-skills-behave)                                                | O nome da skill em sua conta claude.ai, prefixado com `anthropic-skills:`                                              | Skill de conta `deploy` → `/anthropic-skills:deploy`, ou `/deploy` enquanto nenhum outro comando usa esse nome                         |

Em uma skill de plugin, o frontmatter `name` substitui o nome do diretório no último segmento do comando, então `my-plugin/skills/review/SKILL.md` com `name: fancy` se torna `/my-plugin:fancy`. O comando `/fancy` simples também invoca a skill a menos que outro comando já use esse nome. Se o `name` que você escreve já começa com o próprio prefixo do plugin, Claude Code não adiciona o prefixo novamente na v2.1.246 ou posterior. Por exemplo, `name: my-plugin:fancy` ainda se torna `/my-plugin:fancy`. Da v2.1.216 até v2.1.245, Claude Code duplicava o prefixo quando o `name` já o carregava.

Em [sessões não-interativas](/docs/pt/headless), os nomes `help` e `feedback` não são reservados para seus comandos built-in apenas de terminal, então uma skill de plugin com um desses nomes mantém seu comando simples lá. Todos os outros built-ins apenas de terminal, como `/login`, permanecem reservados mesmo que o comando não possa ser executado nessas sessões.

Para um `SKILL.md` raiz de plugin, não há diretório de skill para obter o nome, então `name` fornece o segmento final inteiro. Sem um campo `name`, Claude Code volta para o nome do diretório do plugin.

<h4 id="available-string-substitutions">
  Substituições de string disponíveis
</h4>

Skills suportam substituição de string para valores dinâmicos no conteúdo da skill:

| Variável                | Descrição                                                                                                                                                                                                                                                                                                                         |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | Todos os argumentos passados ao invocar a skill. Quando nenhum placeholder recebe um argumento, Claude Code os anexa como `ARGUMENTS: <value>`. Veja [Passar argumentos para skills](#pass-arguments-to-skills).                                                                                                                  |
| `$ARGUMENTS[N]`         | Acesse um argumento específico por índice baseado em 0, como `$ARGUMENTS[0]` para o primeiro argumento.                                                                                                                                                                                                                           |
| `$N`                    | Abreviação para `$ARGUMENTS[N]`, como `$0` para o primeiro argumento ou `$1` para o segundo.                                                                                                                                                                                                                                      |
| `$name`                 | Argumento nomeado declarado na lista de frontmatter [`arguments`](#frontmatter-reference). Os nomes mapeiam para posições em ordem, então com `arguments: [issue, branch]` o placeholder `$issue` se expande para o primeiro argumento e `$branch` para o segundo.                                                                |
| `${CLAUDE_SESSION_ID}`  | O ID da sessão atual. Útil para logging, criação de arquivos específicos de sessão ou correlação de saída de skill com sessões.                                                                                                                                                                                                   |
| `${CLAUDE_EFFORT}`      | O nível de esforço atual: `low`, `medium`, `high`, `xhigh` ou `max`. Ultracode não é um nível distinto e relata como `xhigh`. Use isso para adaptar instruções de skill à configuração de esforço ativo.                                                                                                                          |
| `${CLAUDE_SKILL_DIR}`   | O diretório contendo o arquivo `SKILL.md` da skill. Para skills de plugin, este é o subdiretório da skill dentro do plugin, não a raiz do plugin. Use isso em comandos de injeção bash para referenciar scripts ou arquivos agrupados com a skill, independentemente do diretório de trabalho atual.                              |
| `${CLAUDE_PROJECT_DIR}` | O diretório raiz do projeto. Este é o mesmo caminho que [hooks](/docs/pt/hooks#reference-scripts-by-path) e servidores MCP recebem como `CLAUDE_PROJECT_DIR`. Use isso para referenciar scripts ou arquivos locais do projeto, como `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, independentemente de onde a skill está instalada. |
| `${CLAUDE_PLUGIN_ROOT}` | O diretório de instalação do plugin. Substituído apenas em skills de plugin. Use isso para referenciar scripts ou arquivos agrupados em qualquer lugar do plugin, incluindo recursos compartilhados entre as skills do plugin. Veja [variáveis de ambiente do plugin](/docs/pt/plugins/manifest-reference#environment-variables).      |
| `${CLAUDE_PLUGIN_DATA}` | O [diretório de dados persistentes](/docs/pt/plugins/components#path-variables-and-persistent-data) do plugin, que sobrevive a atualizações de plugin. Substituído apenas em skills de plugin. Use isso para referenciar dependências instaladas, arquivos gerados ou caches que devem sobreviver a uma atualização.                   |

Claude Code substitui `${CLAUDE_SKILL_DIR}` e `${CLAUDE_PROJECT_DIR}` em dois lugares: o conteúdo markdown da skill e regras Bash no frontmatter [`allowed-tools`](#frontmatter-reference). Em uma skill de plugin, Claude Code substitui `${CLAUDE_PLUGIN_ROOT}` e `${CLAUDE_PLUGIN_DATA}` nos mesmos dois lugares. Usar a mesma variável em ambos os lugares permite que uma skill execute um script agrupado sem um prompt de permissão. A skill a seguir mostra o padrão:

```yaml theme={null}
---
name: render-chart
description: Render a chart from a CSV file
allowed-tools: Bash(${CLAUDE_SKILL_DIR}/scripts/render.sh *)
---

Run `${CLAUDE_SKILL_DIR}/scripts/render.sh <csv-file>` to render the chart.
```

Se esta skill está instalada em `~/.claude/skills/render-chart/`, ambas as ocorrências de `${CLAUDE_SKILL_DIR}` se expandem para esse diretório. A regra `allowed-tools` então corresponde ao comando exato que o corpo da skill diz a Claude para executar, então o script é executado sem avisar.

A substituição `${CLAUDE_PROJECT_DIR}` requer Claude Code v2.1.196 ou posterior.

Argumentos indexados usam quoting estilo shell, então envolva valores com múltiplas palavras em aspas para passá-los como um único argumento. Por exemplo, `/my-skill "hello world" second` faz `$0` se expandir para `hello world` e `$1` para `second`. O placeholder `$ARGUMENTS` sempre se expande para a string de argumento completa conforme digitada.

Um placeholder indexado sem argumento correspondente, como `$2` quando apenas um argumento foi passado, permanece no conteúdo inalterado. Um placeholder nomeado do frontmatter [`arguments`](#frontmatter-reference) sem argumento correspondente se expande para uma string vazia.

Se você passar um valor de argumento que em si contém texto como `$1` ou `$ARGUMENTS`, Claude Code o insere como texto literal e não o expande. Por exemplo, se o corpo de uma skill contém `Summarize $0` e você executa `/summarize "$ARGUMENTS from yesterday"`, Claude recebe `Summarize $ARGUMENTS from yesterday`. Claude Code ainda substitui variáveis `${CLAUDE_*}` como `${CLAUDE_SKILL_DIR}` depois de inserir os argumentos.

Para incluir um `$` literal antes de um dígito, `ARGUMENTS` ou um nome de argumento declarado, como `$1.00` em prosa, escape-o com uma barra invertida: `\$1.00`. Uma barra invertida antes de qualquer outro `$` é deixada inalterada. Apenas uma única barra invertida diretamente antes do token a escapa. Uma barra invertida duplicada como `\\$1` deixa ambas as barras invertidas no lugar, e `$1` ainda se expande para o valor do argumento. O escape de barra invertida cobre apenas esses placeholders de argumento. Uma barra invertida não evita a substituição de uma variável `${CLAUDE_*}` onde a variável se aplica.

**Exemplo usando substituições:**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  Adicionar arquivos de suporte
</h3>

Skills podem incluir múltiplos arquivos em seu diretório. Isso mantém `SKILL.md` focado no essencial enquanto permite que Claude acesse material de referência detalhado apenas quando necessário. Documentos de referência grandes, especificações de API ou coleções de exemplos não precisam carregar em contexto toda vez que a skill é executada.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Referencie arquivos de suporte de `SKILL.md` para que Claude saiba o que cada arquivo contém e quando carregá-lo:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Mantenha `SKILL.md` sob 500 linhas. Mova material de referência detalhado para arquivos separados.</Tip>

<h3 id="control-who-invokes-a-skill">
  Controlar quem invoca uma skill
</h3>

Por padrão, você e Claude podem invocar qualquer skill. Você pode digitar `/skill-name` para invocá-la diretamente, e Claude pode carregá-la automaticamente quando relevante para sua conversa. Dois campos de frontmatter permitem que você restrinja isso:

* **`disable-model-invocation: true`**: Apenas você pode invocar a skill. Use isso para workflows com efeitos colaterais ou que você quer controlar o timing, como `/commit`, `/deploy` ou `/send-slack-message`. Você não quer que Claude decida fazer deploy porque seu código parece pronto.

* **`user-invocable: false`**: Apenas Claude pode invocar a skill. Use isso para conhecimento de fundo que não é acionável como um comando. Uma skill `legacy-system-context` explica como um sistema antigo funciona. Claude deve saber disso quando relevante, mas `/legacy-system-context` não é uma ação significativa para os usuários tomarem.

Este exemplo cria uma skill de deploy que apenas você pode disparar. Se você definir `disable-model-invocation: true`, Claude não pode executar a skill automaticamente:

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Se Claude tentar mesmo assim, Claude Code bloqueia a chamada e o instrui a não reproduzir os passos de deploy de outra forma, então espere que Claude sugira executar `/deploy` você mesmo.

Aqui está como os dois campos afetam invocação e carregamento de contexto:

| Frontmatter                      | Você pode invocar | Claude pode invocar | Quando carregado em contexto                                         |
| :------------------------------- | :---------------- | :------------------ | :------------------------------------------------------------------- |
| (padrão)                         | Sim               | Sim                 | Descrição sempre em contexto, skill completa carrega quando invocada |
| `disable-model-invocation: true` | Sim               | Não                 | Descrição não em contexto, skill completa carrega quando você invoca |
| `user-invocable: false`          | Não               | Sim                 | Descrição sempre em contexto, skill completa carrega quando invocada |

<Note>
  Em uma sessão regular, descrições de skills são carregadas em contexto para que Claude saiba o que está disponível, mas conteúdo de skill completo apenas carrega quando invocado. [Subagents com skills pré-carregadas](/docs/pt/sub-agents#preload-skills-into-subagents) funcionam diferentemente: o conteúdo de skill completo é injetado na inicialização.
</Note>

<h3 id="skill-content-lifecycle">
  Ciclo de vida do conteúdo de skill
</h3>

Quando você ou Claude invocam uma skill, o conteúdo `SKILL.md` renderizado entra na conversa como uma única mensagem e permanece lá entre turnos posteriores. Esta persistência se aplica às instruções da skill, não suas permissões: uma concessão [`allowed-tools`](#pre-approve-tools-for-a-skill) é limpa quando você envia sua próxima mensagem. Claude Code não relê o arquivo de skill em turnos posteriores, então escreva orientação que deve se aplicar ao longo de uma tarefa como instruções permanentes em vez de passos únicos.

Quando Claude reinvoca uma skill cujo conteúdo renderizado é idêntico à cópia já em contexto, Claude Code adiciona uma nota curta que a skill já está carregada em vez de uma segunda cópia do conteúdo. Quando o conteúdo renderizado difere, porque os argumentos mudaram ou um comando [contexto dinâmico](#inject-dynamic-context) produziu nova saída, Claude Code anexa o conteúdo completo novamente.

[Auto-compactação](/docs/pt/how-claude-code-works#when-context-fills-up) leva skills invocadas adiante dentro de um orçamento de token. Quando a conversa é resumida para liberar contexto, Claude Code reanexa a invocação mais recente de cada skill após o resumo, mantendo os primeiros 5.000 tokens de cada. Skills reanexa compartilham um orçamento combinado de 25.000 tokens. Claude Code preenche este orçamento começando pela skill invocada mais recentemente, então skills mais antigas podem ser descartadas inteiramente após compactação se você invocou muitas em uma sessão.

Se uma skill parece parar de influenciar o comportamento após a primeira resposta, o conteúdo geralmente ainda está presente e o modelo está escolhendo outras ferramentas ou abordagens. Fortaleça a `description` da skill e as instruções para que o modelo continue preferindo-a, ou use [hooks](/docs/pt/hooks) para impor comportamento deterministicamente. Se a skill é grande ou você invocou várias outras depois dela, reinvoque-a após compactação para restaurar o conteúdo completo.

<h3 id="pre-approve-tools-for-a-skill">
  Pré-aprovar ferramentas para uma skill
</h3>

O campo `allowed-tools` concede permissão para as ferramentas listadas durante o turno que invoca a skill, para que Claude possa usá-las sem avisar você para aprovação. A concessão é limpa quando você envia sua próxima mensagem, mesmo que o conteúdo da skill [permaneça em contexto](#skill-content-lifecycle); invocar a skill novamente reaplica-a para esse turno. Não restringe quais ferramentas estão disponíveis: toda ferramenta permanece chamável, e suas [configurações de permissão](/docs/pt/permissions) ainda governam ferramentas que não estão listadas. Para pré-aprovar ferramentas para a sessão inteira em vez de um único turno, adicione regras de permissão a essas configurações de permissão em vez disso.

Confiança de workspace não bloqueia este campo. Claude Code aplica `allowed-tools` de uma skill de projeto sempre que você ou Claude invocam a skill, incluindo em uma execução `-p` em uma pasta que você nunca confiou. Uma skill pode conceder a si mesma acesso amplo a ferramentas, então revise `allowed-tools` de skills verificadas em um repositório antes de executar Claude Code lá.

Esta skill permite que Claude execute comandos git sem aprovação por uso sempre que você invoca:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Para remover ferramentas do pool disponível de Claude enquanto uma skill está ativa, liste-as em `disallowed-tools` no frontmatter da skill. A restrição é limpa quando você envia sua próxima mensagem. Como regras de negação, o campo não pode remover [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior) enquanto qualquer outra ferramenta permanecer. Para bloquear ferramentas em todas as skills e prompts, adicione regras de negação em suas [configurações de permissão](/docs/pt/permissions).

<h3 id="pass-arguments-to-skills">
  Passar argumentos para skills
</h3>

Você e Claude podem passar argumentos ao invocar uma skill. Argumentos estão disponíveis via placeholder `$ARGUMENTS`.

Esta skill corrige um problema do GitHub por número. O placeholder `$ARGUMENTS` é substituído por qualquer coisa que siga o nome da skill:

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

Quando você executa `/fix-issue 123`, Claude recebe "Fix GitHub issue 123 following our coding standards..."

Se você invocar uma skill com argumentos mas nenhum placeholder no conteúdo da skill recebe um, Claude Code anexa `ARGUMENTS: <your input>` ao final do conteúdo da skill para que Claude ainda veja o que você digitou. Um placeholder é `$ARGUMENTS`, uma forma indexada como `$1` ou um argumento nomeado. Um placeholder indexado sem argumento em sua posição permanece como texto literal e não conta como recebendo um. Um placeholder nomeado conta mesmo quando sua posição não tem argumento, porque se expande para uma string vazia.

Você também pode empilhar várias skills no início de uma mensagem. Digitar `/write-tests /fix-issue 123` carrega ambas as skills e passa o texto final `123` como `$ARGUMENTS` para cada uma delas. Antes da v2.1.199, apenas a primeira skill carregava e recebia `/fix-issue 123` como texto de argumento literal.

Claude Code expande a primeira skill mais até cinco mais empilhadas depois dela. A expansão para no primeiro token que não é uma skill invocável pelo usuário inline, então uma skill que é executada como um [subagent bifurcado](#run-skills-in-a-subagent), como [`/code-review`](/docs/pt/code-review#review-a-diff-locally), ou uma cujos argumentos podem em si começar com um comando slash, como `/loop`, também termina a execução lá. Esse token e tudo depois dele se tornam o texto de argumento para cada skill expandida. `/code-review` é executado como um subagent bifurcado a partir da v2.1.218; em versões anteriores era executado inline e empilhado.

Para acessar argumentos individuais por posição, use `$ARGUMENTS[N]` ou o mais curto `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Executar `/migrate-component SearchBar JavaScript TypeScript` substitui `$ARGUMENTS[0]` com `SearchBar`, `$ARGUMENTS[1]` com `JavaScript` e `$ARGUMENTS[2]` com `TypeScript`. A mesma skill usando a abreviação `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Padrões avançados
</h2>

<h3 id="inject-dynamic-context">
  Injetar contexto dinâmico
</h3>

A sintaxe `` !`<command>` `` executa comandos shell antes do conteúdo da skill ser enviado para Claude. A saída do comando substitui o espaço reservado, então Claude recebe dados reais, não o comando em si. Claude Code não executa esses comandos em sua máquina quando a skill é [sincronizada de sua conta claude.ai](#how-claude-code-handles-the-body-of-a-synced-skill). Esta restrição requer Claude Code v2.1.228 ou posterior.

Esta skill resume um pull request buscando dados de PR ao vivo com a CLI do GitHub. Os comandos `` !`gh pr diff` `` e outros são executados primeiro, e sua saída é inserida no prompt:

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

A substituição é executada uma vez sobre o arquivo original. A saída do comando é inserida como texto simples e não é verificada novamente para espaços reservados `` !`<command>` `` adicionais, então um comando não pode emitir um espaço reservado para uma passagem posterior expandir.

O formulário inline é reconhecido apenas quando `!` aparece no início de uma linha ou imediatamente após espaço em branco. Se `!` segue outro caractere, como em `` KEY=!`cmd` ``, o espaço reservado é deixado como texto literal e o comando não é executado.

Para comandos de múltiplas linhas, use um bloco de código cercado aberto com ` ```! ` em vez do formulário inline:

````markdown theme={null}
## Environment
```!
node --version
git status --short
```
````

Para desabilitar esse comportamento para skills e comandos personalizados de fontes de usuário, projeto, plugin ou [additional-directory](#skills-from-additional-directories), defina `"disableSkillShellExecution": true` em [settings](/docs/pt/settings). Cada comando é substituído por `[shell command execution disabled by policy]` em vez de ser executado. Skills agrupadas e gerenciadas não são afetadas. Esta configuração é mais útil em [managed settings](/docs/pt/managed-settings), onde os usuários não podem substituí-la.

Claude Code nunca executa esses comandos em sua máquina quando aparecem em skills [sincronizadas de sua conta claude.ai](#how-synced-skills-behave), independentemente desta configuração. Esta restrição requer Claude Code v2.1.228 ou posterior. [How Claude Code handles the body of a synced skill](#how-claude-code-handles-the-body-of-a-synced-skill) diz o que Claude recebe no lugar do comando em cada tipo de sessão.

<Tip>
  Para solicitar raciocínio mais profundo quando uma skill é executada, inclua `ultrathink` em qualquer lugar no conteúdo da skill. Veja [Use ultrathink for one-off deep reasoning](/docs/pt/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h4 id="how-injected-commands-run">
  Como comandos injetados são executados
</h4>

Claude Code escolhe a ferramenta que executa os comandos injetados de uma skill a partir da chave `shell` no frontmatter da skill e seu ambiente. Cada combinação executa os comandos através da ferramenta Bash ou da ferramenta PowerShell, exceto uma que falha na invocação completamente:

* `shell: powershell`, com a [ferramenta PowerShell](/docs/pt/tools-reference#powershell-tool) habilitada: os comandos são executados através da ferramenta PowerShell.
* `shell: bash` quando bash não está disponível: a invocação falha antes de qualquer comando ser executado. Isso acontece no Windows sem Git Bash. Claude Code mostra ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``.
* Qualquer outra combinação: os comandos são executados através da ferramenta Bash quando bash está disponível. Quando não está, eles são executados através da ferramenta PowerShell.

Qualquer ferramenta executa os comandos da mesma forma que executa os próprios comandos shell de Claude. Eles compartilham o diretório de trabalho, timeout e tratamento de saída:

* **Diretório de trabalho**: Claude Code executa cada comando no diretório de trabalho atual do shell da sessão. Esse diretório se move quando Claude executa `cd`. Use [`${CLAUDE_SKILL_DIR}` ou `${CLAUDE_PROJECT_DIR}`](#available-string-substitutions) em caminhos que devem ser resolvidos da mesma forma sempre.
* **stderr**: com o shell `bash` padrão, Claude Code mescla stderr em stdout. Qualquer coisa que o comando escreva em stderr aparece no texto injetado.
* **Timeout**: cada comando é executado sob o [timeout](/docs/pt/tools-reference#timeout-and-output-limits) padrão de 2 minutos da ferramenta Bash. Quando a ferramenta Bash [move um comando com timeout para o background](/docs/pt/tools-reference#background-commands), a skill ainda é renderizada. O texto injetado relata a mudança e nomeia a tarefa em background e o arquivo coletando a saída do comando. Quando o comando é um que a ferramenta Bash nunca coloca automaticamente em background, Claude Code o mata no timeout. Essa falha [aborta a invocação](#when-an-injected-command-fails).
* **Tamanho da saída**: saída além do limite inline da ferramenta Bash chega como um caminho de arquivo mais uma visualização curta, não texto truncado. [Output limits](/docs/pt/tools-reference#output-limits) cobre o limite e como ajustar cada limite.

A ferramenta PowerShell aplica o mesmo comportamento de timeout, backgrounding e output-ceiling aos comandos que executa. Veja a seção [ferramenta PowerShell](/docs/pt/tools-reference#powershell-tool) para seus detalhes.

<h4 id="when-an-injected-command-fails">
  Quando um comando injetado falha
</h4>

Um comando que falha aborta toda a invocação da skill, não apenas seu próprio espaço reservado. Claude nunca vê o conteúdo da skill para essa invocação. O aborto mostra `Shell command failed for pattern "..."`. A mensagem de erro inclui a saída do comando sob `[stderr]`.

Com o shell `bash` padrão, qualquer código de saída diferente de zero conta como uma falha. Uma exceção se aplica: Claude Code trata o código de saída 1 de [search and comparison commands](/docs/pt/tools-reference#output-limits) como um resultado normal e injeta sua saída. Códigos de saída de 2 ou superior falham mesmo para esses comandos.

Quais comandos recebem a exceção depende do shell:

* Shell `bash` padrão: os comandos listados em [Output limits](/docs/pt/tools-reference#output-limits)
* `shell: powershell`, quando a ferramenta PowerShell está habilitada: um [conjunto diferente](/docs/pt/tools-reference#shell-selection-in-settings-hooks-and-skills) que inclui `grep` e `git diff` mas não `find` ou `diff`

Com o shell `bash` padrão, acrescente `|| true` a qualquer outro comando que você espera sair com código diferente de zero. Um script de verificação que sai com 1 quando encontra problemas é um exemplo.

<h4 id="permission-checks-on-injected-commands">
  Verificações de permissão em comandos injetados
</h4>

Comandos injetados nunca solicitam permissão enquanto a skill é renderizada. Claude Code verifica cada um contra suas [regras de permissão](/docs/pt/permissions) primeiro. Um comando que uma regra de negação corresponde aborta a invocação com `Shell command permission check failed for pattern "..."`.

Fora do [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), quando a verificação de permissão de um comando retorna qualquer coisa diferente de permitir, Claude Code aborta a invocação com o mesmo erro. Isso inclui uma regra que normalmente perguntaria. Para evitar que um comando não correspondido aborte aqui, pré-aprove-o com [`allowed-tools`](#pre-approve-tools-for-a-skill). Regras de negação e pergunta ainda substituem `allowed-tools`. Veja [Manage permissions](/docs/pt/permissions#manage-permissions).

No modo automático, um comando que de outra forma precisaria de sua aprovação não aborta a invocação. A skill carrega com uma instrução dizendo a Claude para executar o comando primeiro, e a própria chamada de Claude passa pelas [verificações usuais do modo automático](/docs/pt/permission-modes#how-the-classifier-evaluates-actions). A invocação ainda aborta em uma [skill bifurcada](#run-skills-in-a-subagent) que define `agent`, e em uma sessão onde Claude não tem a [ferramenta shell que executa comandos injetados](#how-injected-commands-run).

<h3 id="run-skills-in-a-subagent">
  Executar skills em um subagente
</h3>

Adicione `context: fork` ao seu frontmatter quando você quiser que uma skill seja executada em isolamento. Claude Code inicia um novo subagente do tipo definido no campo `agent` e lhe fornece o conteúdo da skill como seu prompt. O subagente não vê seu histórico de conversa, então as instruções da skill têm que se sustentar por si mesmas.

<Note>
  Apesar do nome, uma skill com `context: fork` não é executada em um [fork da conversa atual](/docs/pt/sub-agents#fork-the-current-conversation), que entregaria ao subagente tudo o que você discutiu até agora. Quando a tarefa depende desse histórico, bifurque a conversa em vez de usar `context: fork`.
</Note>

O subagente bifurcado é executado em [background](/docs/pt/sub-agents#run-subagents-in-foreground-or-background): você continua trabalhando enquanto ele é executado, e seu resultado chega em sua conversa quando é concluído. Defina `background: false` no frontmatter para esperar o resultado na volta que invocou a skill. Antes da v2.1.218, skills bifurcadas sempre bloqueavam a volta até serem concluídas.

Claude Code também espera pelo resultado, mesmo quando a skill não define `background: false`, em casos como estes:

* Em modo não-interativo, com a flag `-p` ou o Agent SDK
* Quando você define [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/pt/env-vars) para `1`, o que também desativa todos os outros recursos de tarefa em background
* Quando você invoca uma skill bifurcada enquanto uma invocação anterior da mesma skill ainda está em execução
* Quando uma [scheduled task](/docs/pt/scheduled-tasks) dispara com a skill como seu prompt

Um fork em background também é executado com o [conjunto de ferramentas mais estreito que se aplica a subagentes em background](/docs/pt/sub-agents#run-subagents-in-foreground-or-background): o subagente da skill é um tipo de agente regular, então a isenção para subagentes que bifurcam a conversa não o cobre. Se as etapas de sua skill dependem de uma ferramenta fora desse conjunto, defina `background: false` para manter o conjunto completo de ferramentas.

Uma skill bifurcada que é executada em background aplica suas edições fora dos [checkpoints](/docs/pt/checkpointing) de sua sessão, então `/rewind` não as desfaz; use git para revertê-las.

<Warning>
  `context: fork` só faz sentido para skills com instruções explícitas. Se sua skill contém diretrizes como "use essas convenções de API" sem uma tarefa, o subagente recebe as diretrizes mas nenhum prompt acionável, e retorna sem saída significativa.
</Warning>

Skills e [subagentes](/docs/pt/sub-agents) trabalham juntos em duas direções:

| Abordagem                    | System prompt               | Tarefa                          | Também carrega                                                                                                     |
| :--------------------------- | :-------------------------- | :------------------------------ | :----------------------------------------------------------------------------------------------------------------- |
| Skill com `context: fork`    | Do tipo de agente           | Conteúdo SKILL.md               | CLAUDE.md, conforme o [startup context](/docs/pt/sub-agents#what-loads-at-startup) do agente                            |
| Subagente com campo `skills` | Corpo markdown do subagente | Mensagem de delegação de Claude | Skills pré-carregadas + CLAUDE.md, conforme o [startup context](/docs/pt/sub-agents#what-loads-at-startup) do subagente |

Com `context: fork`, você escreve a tarefa em sua skill e escolhe um tipo de agente para executá-la. Os agentes Explore e Plan integrados [pulam CLAUDE.md e git status](/docs/pt/sub-agents#what-loads-at-startup) para manter seu contexto pequeno, então uma skill bifurcada usando `agent: Explore` vê apenas o conteúdo SKILL.md e o prompt do sistema do agente. Para o inverso, onde você define um subagente personalizado que usa skills como material de referência, veja [Subagentes](/docs/pt/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Exemplo: Skill de pesquisa usando agente Explore
</h4>

Esta skill executa pesquisa em um agente Explore bifurcado. O conteúdo da skill se torna a tarefa, e o agente fornece ferramentas somente leitura otimizadas para exploração de codebase:

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

Quando esta skill é executada:

1. Um novo contexto isolado é criado
2. O subagente recebe o conteúdo da skill como seu prompt (as instruções "Research \$ARGUMENTS thoroughly")
3. O campo `agent` determina o ambiente de execução (modelo, ferramentas e permissões)
4. O subagente resume seus resultados e os retorna para sua conversa principal quando termina

O campo `agent` especifica qual configuração de subagente usar. As opções incluem agentes integrados (`Explore`, `Plan`, `general-purpose`) ou qualquer subagente personalizado de `.claude/agents/`. Se omitido, usa `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Restringir acesso de Claude às skills
</h3>

Por padrão, Claude pode invocar qualquer skill que não tenha `disable-model-invocation: true` definido. Skills que definem `allowed-tools` concedem a Claude acesso a essas ferramentas sem aprovação por uso durante a volta que invoca a skill; a concessão é limpa quando você envia sua próxima mensagem. Suas [configurações de permissão](/docs/pt/permissions) ainda governam o comportamento de aprovação de linha de base para todas as outras ferramentas. Alguns comandos integrados também estão disponíveis através da ferramenta Skill, incluindo `/init` e `/security-review`. Outros comandos integrados como `/compact` não estão.

Três maneiras de controlar quais skills Claude pode invocar:

**Desabilitar todas as skills** negando a ferramenta Skill em `/permissions`:

```text theme={null}
# Add to deny rules:
Skill
```

**Permitir ou negar skills específicas** usando [regras de permissão](/docs/pt/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Sintaxe de permissão: `Skill(name)` para correspondência exata, `Skill(name *)` para correspondência de prefixo com quaisquer argumentos.

Se sua regra `deny` nomeia um alias ou um nome não qualificado em vez do nome da própria skill, Claude Code ainda bloqueia a skill: com `Skill(review)` bloqueia o `/code-review` agrupado através de seu alias `/review`, e com `Skill(deploy)` bloqueia uma [skill aninhada](#where-skills-live) listada como `apps/web:deploy` através de seu nome não qualificado. Antes da v2.1.260, Claude Code não bloqueava uma skill aninhada listada sob seu nome qualificado quando a regra deny nomeava apenas o nome não qualificado.

Claude Code corresponde uma regra `allow` apenas contra o nome da própria skill e o nome na invocação de Claude.

**Ocultar skills individuais** adicionando `disable-model-invocation: true` ao seu frontmatter. Isso remove a skill do contexto de Claude completamente.

<Note>
  Com `user-invocable: false`, você não pode invocar a skill, mas Claude ainda pode. Para evitar que Claude a invoque através da ferramenta Skill, defina `disable-model-invocation: true`.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Substituir visibilidade de skill a partir de configurações
</h3>

A configuração `skillOverrides` controla a visibilidade de skill a partir de suas [configurações](/docs/pt/settings) em vez do frontmatter da própria skill. Use-a para skills cujo SKILL.md você não quer editar, como aquelas verificadas em um repositório de projeto compartilhado. O menu `/skills` escreve para você: destaque uma skill e pressione `Space` para alternar estados, depois `Esc` para salvar em `.claude/settings.local.json`.

Cada chave é um nome de skill e cada valor é um de quatro estados:

| Valor                   | Listado para Claude | No menu `/` |
| :---------------------- | :------------------ | :---------- |
| `"on"`                  | Nome e descrição    | Sim         |
| `"name-only"`           | Apenas nome         | Sim         |
| `"user-invocable-only"` | Oculto              | Sim         |
| `"off"`                 | Oculto              | Oculto      |

O menu `/skills` rotula o estado `"user-invocable-only"` como `user-only`.

A partir da v2.1.199, `"off"` também oculta a skill das listas de comandos anunciadas para clientes [Remote Control](/docs/pt/remote-control) e para chamadores [Agent SDK](/docs/pt/agent-sdk/skills#discover-available-commands), além do menu `/` do terminal. Invocar uma skill oculta pelo seu nome completo ainda retorna o erro `skillOverrides` em vez de executá-la.

Uma skill ausente de `skillOverrides` é tratada como `"on"`. O exemplo abaixo colapsa uma skill para seu nome e desativa outra completamente:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Algumas skills agrupadas têm aliases, como `checkup` para `/doctor`. Se você definir uma entrada `skillOverrides` sob um alias em [managed settings](/docs/pt/managed-settings) ou em um arquivo que você passa com a flag `--settings`, Claude Code a aplica à skill atrás do alias. Você só pode restringir uma skill ainda mais através de um alias, nunca torná-la mais visível, e se você também definir uma entrada sob o nome da própria skill em managed settings, essa entrada tem precedência. Antes da v2.1.260, Claude Code não aplicava uma entrada sob um alias à skill em nenhuma fonte de configurações.

Em configurações de usuário, projeto e local, Claude Code corresponde entradas apenas contra nomes de skills. Se você definir uma entrada para `review` lá, ela se aplica a uma skill nomeada `review`, não ao `/code-review` agrupado através de seu alias `/review`.

Skills de plugin não são afetadas por `skillOverrides`. Gerencie-as através de `/plugin` em vez disso.

<h3 id="find-unused-skills">
  Encontrar skills não utilizadas
</h3>

Cada skill na [listagem de skills](#skill-descriptions-are-cut-short) adiciona ao seu contexto em cada volta, independentemente de Claude nunca usá-la. Execute `/skill-doctor` para ver o que cada uma de suas skills custa e com que frequência é usada, para que você possa decidir quais desativar. Em uma sessão interativa, o relatório abre na aba **Stats** do gerenciador `/plugin`. Em [modo não-interativo](/docs/pt/headless) com `-p`, Claude Code o imprime como texto.

O relatório cobre as skills em sua sessão além de skills agrupadas e skills empresariais. Ele sinaliza skills na listagem que nunca foram invocadas e diz onde desativá-las. Das skills que ele diz onde desativar, comece com as que têm o maior custo de contexto. O relatório também lista plugins que você não usou recentemente.

`/skill-doctor` requer Claude Code v2.1.252 ou posterior e não está disponível em sessões que pulam [feature-flag fetching](/docs/pt/env-vars#features-that-need-feature-flag-fetching). Se você executar `/skill-doctor` sobre [Remote Control](/docs/pt/remote-control) de seu telefone ou navegador, Claude Code responde [`Skill usage reports are not available on this connection.`](/docs/pt/errors#skill-usage-reports-are-not-available-on-this-connection) em vez disso. Execute `/skill-doctor` no terminal na máquina onde a sessão está em execução.

<h2 id="evaluate-and-iterate-on-a-skill">
  Avaliar e iterar em uma skill
</h2>

Ver uma skill ser acionada informa que Claude a encontrou, não que ela fez o que você pretendia. Para saber que uma skill está funcionando, meça separadamente se Claude a invoca nos prompts que deveria, e se a saída corresponde ao que você espera quando o faz.

A verificação de ambas é uma comparação de linha de base. Colete alguns prompts realistas, execute cada um em uma sessão nova com a skill disponível e novamente com ela [desabilitada](#override-skill-visibility-from-settings), e compare os resultados. Uma sessão nova é importante porque o contexto restante da autoria da skill mascarará lacunas nas instruções escritas.

Duas ferramentas automatizam essa comparação. Para uma skill que é entregue em um [plugin](/docs/pt/plugins/overview), [`claude plugin eval`](/docs/pt/plugin-evals) executa cada prompt em uma sessão isolada com e sem o plugin, a classifica com avaliadores que você define ou que ela escreve para você, e sai com código não-zero abaixo de um limite para que você possa bloquear CI nela. Para iterar em uma única skill dentro de uma conversa Claude Code, o plugin skill-creator abaixo executa um loop similar com seu próprio formato `evals/evals.json`. Os dois formatos não são intercambiáveis.

<h3 id="run-evals-with-skill-creator">
  Executar evals com skill-creator
</h3>

O [plugin `skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatiza o loop de comparação dentro do Claude Code. Instale-o do marketplace oficial:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Se a instalação falhar, corresponda à mensagem que Claude Code relata:

* `Marketplace "claude-plugins-official" not found`: adicione o marketplace com `/plugin marketplace add anthropics/claude-plugins-official`, depois tente novamente a instalação.
* O plugin [não foi encontrado no marketplace](/docs/pt/plugins/install#install-a-plugin): verifique o nome do plugin.

Se o resumo da instalação relatar `Run /reload-plugins to activate.`, Claude Code então executa esse reload para você. Se o reload avisar que sua próxima mensagem releria a conversa, execute `/reload-plugins --force` para disponibilizar as skills do plugin na sessão atual. Depois peça ao Claude para avaliar uma skill existente, por exemplo `evaluate my summarize-changes skill with skill-creator`. O plugin o orienta através da escrita de casos de teste e executa o loop:

* **Casos de teste**: armazena prompts, arquivos de entrada e comportamento esperado em `evals/evals.json` dentro do diretório da skill
* **Execuções isoladas**: gera um [subagent](/docs/pt/sub-agents) por caso de teste para que cada execução comece com um contexto limpo, e registra contagem de tokens e duração
* **Classificação**: verifica cada asserção contra a saída e escreve aprovado ou reprovado com evidência em `grading.json`
* **Benchmark**: agrega taxa de aprovação, tempo e tokens para com-skill versus sem-skill em `benchmark.json` para que você possa comparar a melhoria da taxa de aprovação contra a sobrecarga de token e tempo
* **Comparação de versão**: executa um A/B cego entre duas versões da skill para que você possa confirmar que uma edição é uma melhoria antes de confirmá-la
* **Ajuste de descrição**: gera prompts de deve-acionar e não-deve-acionar, mede a taxa de acerto e propõe edições de descrição quando a skill é acionada em solicitações erradas
* **Visualizador de revisão**: abre um relatório HTML onde você inspeciona cada saída e registra feedback qualitativo que a próxima iteração lê

Para o formato do arquivo eval e o fluxo de trabalho de iteração completo, consulte [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) em agentskills.io. Para informações sobre o benchmark e modos de comparação, consulte o [skill-creator announcement](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Compartilhar skills
</h2>

Skills podem ser distribuídas em diferentes escopos dependendo do seu público:

* **Project skills**: Faça commit de `.claude/skills/` para controle de versão
* **Plugins**: Crie um diretório `skills/` em seu [plugin](/docs/pt/plugins/overview)
* **Managed**: Implante em toda a organização através de [managed settings](/docs/pt/managed-settings)

<h3 id="generate-visual-output">
  Gerar saída visual
</h3>

Skills podem agrupar e executar scripts em qualquer linguagem, dando ao Claude capacidades além do que é possível em um único prompt. Um padrão é gerar saída visual: arquivos HTML interativos que abrem em seu navegador para explorar dados, depurar ou criar relatórios.

Este exemplo cria um explorador de codebase: uma visualização de árvore interativa onde você pode expandir e recolher diretórios, ver tamanhos de arquivo em um relance e identificar tipos de arquivo por cor.

Crie o diretório Skill:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Salve isto em `~/.claude/skills/codebase-visualizer/SKILL.md`. A descrição diz ao Claude quando ativar este Skill, e as instruções dizem ao Claude para executar o script agrupado. O caminho do script usa [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) para que seja resolvido corretamente se a skill estiver instalada no nível pessoal, de projeto ou de plugin:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

Salve isto em `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Este script varre uma árvore de diretórios e gera um arquivo HTML independente com:

* Uma **barra lateral de resumo** mostrando contagem de arquivos, contagem de diretórios, tamanho total e número de tipos de arquivo
* Um **gráfico de barras** dividindo o codebase por tipo de arquivo (top 8 por tamanho)
* Uma **árvore recolhível** onde você pode expandir e recolher diretórios, com indicadores de tipo de arquivo codificados por cor

O script requer Python 3 mas usa apenas bibliotecas integradas, então não há pacotes para instalar:

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

Para testar, abra Claude Code em qualquer projeto e peça "Visualize this codebase." Claude executa o script, que imprime o caminho do arquivo gerado, como `Generated /path/to/codebase-map.html`, e o abre em seu navegador. Se você trabalha em um ambiente sem interface gráfica onde nenhum navegador abre, o caminho impresso confirma que o script foi bem-sucedido.

Este padrão funciona para qualquer saída visual: gráficos de dependência, relatórios de cobertura de testes, documentação de API ou visualizações de esquema de banco de dados. O script agrupado faz o trabalho enquanto Claude lida com a orquestração.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="skill-not-triggering">
  Skill não é acionada
</h3>

Se Claude não usar sua skill quando esperado:

1. Verifique se a descrição inclui palavras-chave que os usuários naturalmente diriam
2. Verifique se a skill aparece em `What skills are available?`
3. Tente reformular sua solicitação para corresponder mais closely à descrição
4. Invoque-a diretamente com `/skill-name` se a skill for invocável pelo usuário

Se o YAML do frontmatter estiver malformado, Claude Code carrega o corpo da skill com metadados vazios, então `/skill-name` ainda funciona, mas Claude não pode corresponder contra sua `description`. Execute com `--debug` para ver o erro de análise.

Se a skill é fornecida em um plugin, você pode medir com que frequência ela é acionada em prompts realistas em vez de verificar uma de cada vez: escreva um caso de eval com um [`tool_used: Skill` grader](/docs/pt/plugin-evals#create-your-first-eval-suite) e execute-o com `claude plugin eval` após cada mudança de descrição.

Para encontrar arquivos `SKILL.md` cujo frontmatter não é analisado, execute [`claude plugin validate`](/docs/pt/plugins/cli-reference#validate-a-directory) no diretório de skills, por exemplo `claude plugin validate .claude/skills` para skills de projeto ou `claude plugin validate ~/.claude/skills` para skills pessoais. Requer Claude Code v2.1.233 ou posterior.

<h3 id="skill-triggers-too-often">
  Skill é acionada com muita frequência
</h3>

Se Claude usar sua skill quando você não quer:

1. Torne a descrição mais específica
2. Adicione `disable-model-invocation: true` se você quiser apenas invocação manual

<h3 id="skill-descriptions-are-cut-short">
  Descrições de skill são cortadas
</h3>

Claude Code carrega uma listagem de nomes e descrições de skills no contexto para que Claude saiba o que está disponível. A listagem sempre contém todos os nomes de skills, mas se você tiver muitas skills, Claude Code encurta as descrições para se ajustar ao orçamento de caracteres da listagem, o que pode remover as palavras-chave que Claude precisa para corresponder sua solicitação. O orçamento é dimensionado em 1% da janela de contexto do modelo. Quando a listagem excede o limite, Claude Code remove descrições começando com as skills que você invoca menos, então as skills que você usa mais mantêm seu texto completo.

Execute `/doctor` para uma estimativa do custo de contexto da listagem e seus maiores contribuidores. Para encontrar skills que valem a pena desativar, execute [`/skill-doctor`](#find-unused-skills). Quando a listagem excede seu orçamento, Claude Code também escreve um aviso no log de depuração, visível com [`--debug`](/docs/pt/cli-reference#cli-flags).

A linha Skills em `/context` relata o tamanho da listagem após o orçamento ser aplicado, então corresponde ao que o modelo recebe. Antes da v2.1.196, a linha contava o texto completo de cada descrição e poderia mostrar um valor várias vezes maior que o orçamento configurado.

Para aumentar o orçamento, defina a configuração [`skillListingBudgetFraction`](/docs/pt/settings-reference#skilllistingbudgetfraction) (por exemplo, `0.02` = 2%) ou a variável de ambiente `SLASH_COMMAND_TOOL_CHAR_BUDGET` para uma contagem de caracteres fixa. Para liberar orçamento para outras skills, defina entradas de baixa prioridade como `"name-only"` em [`skillOverrides`](#override-skill-visibility-from-settings) para que elas apareçam na listagem sem uma descrição. Você também pode aparar o texto `description` e `when_to_use` na fonte: coloque o caso de uso principal primeiro, já que o texto combinado de cada entrada é limitado a 1.536 caracteres independentemente do orçamento. O limite é configurável com [`skillListingMaxDescChars`](/docs/pt/settings-reference#skilllistingmaxdescchars).

<h3 id="personal-skills-disappeared">
  Personal skills desapareceram
</h3>

Se as pastas de skills que você criou em `~/.claude/skills/` desapareceram, procure em `~/.claude/skills/.trash/`. Quando Claude Code [sincroniza skills do claude.ai](#how-synced-skills-behave), ele as baixa na subpasta separada `synced` e não move ou deleta as pastas que você cria.

Antes da v2.1.280, um arquivo chamado `manifest.json` em `~/.claude/skills/` fazia com que Claude Code movesse as pastas de skills que esse arquivo listava para uma pasta com timestamp em `~/.claude/skills/.trash/`, e essas skills paravam de carregar.

Para restaurar uma skill, mova sua pasta da pasta com timestamp de volta para `~/.claude/skills/`. Faça isso antes da [limpeza de retenção](/docs/pt/claude-directory#cleaned-up-automatically) deletar entradas de lixo, por padrão 30 dias após serem movidas para a lixeira.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* **[Depure sua configuração](/docs/pt/debug-your-config)**: diagnostique por que uma skill não está aparecendo ou sendo acionada
* **[Avaliando a qualidade de saída de skill](https://agentskills.io/skill-creation/evaluating-skills)**: o formato do arquivo eval e fluxo de trabalho de iteração em agentskills.io
* **[Melhores práticas de autoria de skill](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: orientação de escrita que se aplica em produtos Claude
* **[Subagents](/docs/pt/sub-agents)**: delegue tarefas para agents especializados
* **[Plugins](/docs/pt/plugins/overview)**: empacote e distribua skills com outras extensões
* **[Hooks](/docs/pt/hooks)**: automatize fluxos de trabalho em torno de eventos de ferramentas
* **[Memory](/docs/pt/memory)**: gerencie arquivos CLAUDE.md para contexto persistente
* **[Comandos](/docs/pt/commands)**: referência para comandos integrados e skills agrupadas
* **[Permissões](/docs/pt/permissions)**: controle acesso a ferramentas e skills
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: skills de projeto confirmadas em um repositório também são carregadas quando esse repositório é usado em um canal Claude Tag
