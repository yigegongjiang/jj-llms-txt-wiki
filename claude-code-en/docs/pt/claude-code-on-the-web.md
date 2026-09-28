> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code na nuvem

> Execute sessões Claude Code na nuvem a partir do seu navegador, telefone, aplicativo desktop ou terminal, mova-as com --cloud e --teleport, e corrija automaticamente pull requests.

<Note>
  As sessões em nuvem estão disponíveis nos planos Pro, Max e Team, e para usuários Enterprise com assentos premium ou assentos Chat + Claude Code.
</Note>

Uma sessão em nuvem é uma sessão Claude Code que é executada em infraestrutura em nuvem em vez de em sua máquina. Por padrão, ela é executada em infraestrutura que a Anthropic gerencia, ou no [ambiente auto-hospedado](/docs/pt/self-hosted-environments) da sua organização quando roteada para lá. A sessão continua em execução depois que você fecha seu laptop, e você pode verificá-la ou direcioná-la a partir de qualquer dispositivo.

Você pode iniciar uma sessão em nuvem a partir de qualquer uma dessas superfícies:

* **Navegador**: [claude.ai/code](https://claude.ai/code), também chamado Claude Code na web
* **Móvel**: a aba **Code** no [aplicativo Claude](/docs/pt/mobile)
* **Aplicativo desktop**: selecione **Cloud** em vez de **Local** quando você [inicia uma sessão](/docs/pt/desktop#run-long-running-tasks-in-the-cloud)
* **Terminal**: [`claude --cloud`](#from-terminal-to-cloud)
* **Rotinas**: [execuções agendadas e acionadas](/docs/pt/routines) cada uma é executada como uma sessão em nuvem

Para que Claude inicie e acompanhe muitas sessões em nuvem para um corpo de trabalho, use um [projeto](/docs/pt/claude-projects). Uma sessão em seu terminal, seu IDE, ou o aplicativo Desktop com **Local** selecionado é executada em sua própria máquina. Para direcionar uma dessas sessões locais a partir do seu telefone ou navegador, use [Controle Remoto](/docs/pt/remote-control).

<Tip>
  Novo em sessões em nuvem? Comece com [Começar](/docs/pt/web-quickstart) para conectar sua conta GitHub e enviar sua primeira tarefa.
</Tip>

Esta página cobre:

* [Ambientes em nuvem](#cloud-environments): onde as sessões são executadas e onde configurar isso
* [Opções de autenticação do GitHub](#github-authentication-options): duas maneiras de conectar o GitHub
* [Mover tarefas entre terminal e nuvem](#move-tasks-between-terminal-and-cloud) com `--cloud` e `--teleport`
* [Trabalhar com sessões](#work-with-sessions): modos de permissão, revisão, compartilhamento, arquivamento, exclusão
* [Corrigir automaticamente pull requests](#auto-fix-pull-requests): responder automaticamente a falhas de CI e comentários de revisão
* [Segurança e isolamento](#security-and-isolation): como as sessões são isoladas
* [Limitações](#limitations): limites de taxa e restrições de plataforma

<h2 id="cloud-environments">
  Ambientes em nuvem
</h2>

Cada sessão em nuvem é executada em um [ambiente em nuvem](/docs/pt/cloud-environments), a configuração salva que controla acesso à rede, variáveis de ambiente e scripts de configuração. Se você ainda não tem um ambiente, o onboarding configura um ambiente **Default** com [acesso à rede **Trusted**](/docs/pt/cloud-environments#access-levels), criando-o para você ou pedindo que você o crie. Veja [O ambiente Default](/docs/pt/cloud-environments#the-default-environment) para saber qual desses acontece no seu plano e como as sessões escolhem um ambiente quando você tem mais de um.

Os mesmos ambientes se aplicam em qualquer lugar que você inicie uma sessão em nuvem: a web, o terminal, [Claude Tag](https://claude.com/docs/claude-tag/overview), [routines](/docs/pt/routines) e os aplicativos móvel e Desktop. As sessões de canal Claude Tag usam apenas ambientes no nível da organização, seja [ambientes compartilhados](/docs/pt/cloud-environments#organization-shared-environments) ou [ambientes auto-hospedados](/docs/pt/self-hosted-environments).

Veja [Configurar ambientes em nuvem](/docs/pt/cloud-environments) para alterar o que um ambiente permite, definir variáveis ou adicionar um script de configuração, e [Ferramentas instaladas](/docs/pt/cloud-environments#installed-tools) para o que as sessões incluem sem nenhuma configuração.

<h2 id="github-authentication-options">
  Opções de autenticação do GitHub
</h2>

As sessões em nuvem precisam de acesso aos seus repositórios GitHub para clonar código e enviar branches. Você pode conceder acesso de duas maneiras:

| Método           | Como você se conecta                                                                            | Repositórios que as sessões podem alcançar                                                                      | Melhor para                                                                      |
| :--------------- | :---------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- |
| **GitHub App**   | Autorize o Claude GitHub App durante [onboarding na web](/docs/pt/web-quickstart)                    | Qualquer repositório público e repositórios privados nos quais o Claude GitHub App está instalado               | Onboarding no navegador; equipes que desejam [Auto-fix](#auto-fix-pull-requests) |
| **`/web-setup`** | Execute `/web-setup` em seu terminal para enviar seu token CLI `gh` local para sua conta Claude | Qualquer repositório que seu token `gh` possa acessar, independentemente de o Claude GitHub App estar instalado | Desenvolvedores individuais que já usam `gh`                                     |

A instalação do Claude GitHub App em um repositório também habilita [Auto-fix](#auto-fix-pull-requests) para pull requests nele.

As threads em um [projeto](/docs/pt/claude-projects) precisam que o Claude GitHub App esteja instalado em cada repositório que elas clonam, independentemente do método com o qual você se conectou. Consulte [Configurar acesso ao GitHub](/docs/pt/claude-projects#set-up-github-access).

Para saber como `/schedule` verifica o acesso ao repositório antes de criar uma routine, consulte [Repositórios e permissões de branch](/docs/pt/routines#repositories-and-branch-permissions). Consulte [Conectar a partir do seu terminal](/docs/pt/web-quickstart#connect-from-your-terminal) para o passo a passo de `/web-setup`, incluindo o que `/web-setup` armazena e como removê-lo.

Quick web setup é uma configuração de organização que permite que membros conectem o GitHub com `/web-setup`, pula o prompt de instalação do Claude GitHub App durante o onboarding do navegador e faz com que o onboarding do navegador crie o [ambiente **Default**](/docs/pt/cloud-environments#the-default-environment) para eles em vez de mostrar o formulário de ambiente. Nos planos Team e Enterprise está desabilitado por padrão, o que oculta `/web-setup`. Um [Owner](/docs/pt/server-managed-settings#access-control) o ativa com o toggle **Quick web setup** em [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code).

<Note>
  Organizações com [Zero Data Retention](/docs/pt/zero-data-retention) habilitado não podem usar `/web-setup` ou outros recursos de sessão em nuvem.
</Note>

<h2 id="move-tasks-between-terminal-and-cloud">
  Mover tarefas entre terminal e nuvem
</h2>

Esses fluxos de trabalho requerem o [Claude Code CLI](/docs/pt/quickstart) conectado à mesma conta claude.ai. Você pode iniciar novas sessões em nuvem a partir do seu terminal, ou puxar sessões em nuvem para seu terminal para continuar localmente. As sessões em nuvem persistem mesmo se você fechar seu laptop, e você pode monitorá-las de qualquer lugar, incluindo o aplicativo móvel Claude.

<Note>
  A partir do CLI, a transferência de sessão é unidirecional: você pode puxar sessões em nuvem para seu terminal com `--teleport`, mas não pode enviar uma sessão de terminal existente para a nuvem. O sinalizador `--cloud` com uma descrição de tarefa cria uma nova sessão em nuvem para seu repositório atual; com `-p` e um ID de sessão ou URL claude.ai/code, ele [enfileira uma mensagem naquela sessão existente](/docs/pt/claude-code-on-the-web#send-follow-ups-from-the-cli). O [aplicativo Desktop](/docs/pt/desktop#continue-in-another-surface) fornece um menu **Continue in** que pode enviar uma sessão local para a nuvem.
</Note>

<h3 id="from-terminal-to-cloud">
  Do terminal para a nuvem
</h3>

Inicie uma sessão em nuvem a partir da linha de comando com o sinalizador `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Isso cria uma nova sessão em nuvem em claude.ai. A VM em nuvem clona o remoto GitHub do seu diretório atual na sua branch atual, não seu checkout local, então envie primeiro se você tiver commits locais. Veja [Envie repositórios locais sem GitHub](#send-local-repositories-without-github) para os casos em que Claude Code carrega seu repositório local em vez de clonar.

`--cloud` funciona com um repositório por vez. A tarefa é executada na nuvem enquanto você continua trabalhando localmente. A ortografia mais antiga `--remote` ainda funciona como um alias descontinuado para `--cloud`.

Enquanto o contêiner em nuvem inicia, o CLI mostra uma lista de verificação ao vivo das etapas de configuração, como clonar o repositório e executar seu [script de configuração](/docs/pt/cloud-environments#setup-scripts). Ele enfileira mensagens que você digita durante o provisionamento e as envia assim que a sessão estiver pronta.

<Note>
  `--cloud` cria sessões em nuvem. `--remote-control` não está relacionado: permite que você monitore e dirija uma sessão CLI local a partir de claude.ai ou do aplicativo Claude. Veja [Remote Control](/docs/pt/remote-control).
</Note>

Abra a sessão em claude.ai ou no aplicativo móvel Claude para verificar o progresso ou interagir diretamente. De lá você pode orientar Claude, fornecer feedback ou responder perguntas como em qualquer outra conversa.

Se Claude fizer uma pergunta e a sessão ficar ociosa, você ainda pode responder quando voltar, até [expiração do ambiente](#environment-expired), e a sessão continua a partir de sua resposta.

<h4 id="tips-for-cloud-tasks">
  Dicas para tarefas em nuvem
</h4>

**Planeje localmente, execute na nuvem**: para tarefas complexas, inicie Claude em plan mode para colaborar na abordagem, depois envie o trabalho para a nuvem:

```bash theme={null}
claude --permission-mode plan
```

Em plan mode, Claude lê arquivos, executa comandos para explorar e propõe um plano sem editar código-fonte. Depois de estar satisfeito, salve o plano no repositório, confirme e envie para que a VM em nuvem possa cloná-lo. Depois inicie uma sessão em nuvem para execução autônoma:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

**Execute tarefas em paralelo**: cada comando `--cloud` cria sua própria sessão em nuvem que é executada independentemente. Você pode iniciar múltiplas tarefas e todas serão executadas simultaneamente em sessões separadas:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Quando uma sessão é concluída, você pode criar um PR a partir de claude.ai/code ou [teleportar](#from-cloud-to-terminal) a sessão para seu terminal para continuar trabalhando.

<h4 id="send-local-repositories-without-github">
  Envie repositórios locais sem GitHub
</h4>

Quando você executa `claude --cloud` a partir de um repositório que não tem um remoto git, ou a partir de um repositório github.com no qual o Claude GitHub App não está instalado, Claude Code agrupa seu repositório local e o carrega diretamente para a sessão em nuvem. Isso se aplica mesmo se você conectou GitHub com `/web-setup`. O pacote inclui seu histórico completo de repositório em todas as branches, mais quaisquer alterações não confirmadas em arquivos rastreados.

Em macOS, Linux e WSL, Claude Code deixa alterações não confirmadas em arquivos nomeados como credenciais ou chaves fora do upload e nomeia os arquivos que deixou de fora. Isso cobre arquivos `.env`, arquivos Terraform `*.tfvars` e arquivos de chave como `id_rsa` e `*.pem`. A sessão inicia com a versão confirmada de cada um, ou sem o arquivo se nenhum estiver confirmado. Em um worktree vinculado, submódulo ou layout similar, Claude Code carrega essas alterações com o resto e nomeia os arquivos que carrega.

Para carregar um pacote mesmo quando Claude Code clonaría do remoto, defina `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Os repositórios agrupados devem atender a esses limites:

* O diretório deve ser um repositório git com pelo menos um commit
* O repositório agrupado deve estar abaixo de 100 MB. Repositórios maiores voltam a agrupar apenas a branch atual, depois a um snapshot único e compactado da árvore de trabalho, e falham apenas se o snapshot ainda for muito grande
* Arquivos não rastreados não estão incluídos; execute `git add` em arquivos que você deseja que a sessão em nuvem veja
* As sessões criadas a partir de um pacote podem enviar de volta para um remoto GitHub apenas quando sua [conexão GitHub](#github-authentication-options) tem acesso de push para esse repositório

<h3 id="send-follow-ups-from-the-cli">
  Envie follow-ups a partir do CLI
</h3>

Depois que uma sessão em nuvem está em execução, em qualquer lugar que ela seja executada, envie uma mensagem de follow-up a partir do CLI `claude` em qualquer máquina onde você esteja conectado com `claude auth login`. O CLI autentica com suas credenciais de conta Anthropic e não envia nenhum estado de sessão local, então o comando não precisa ser executado a partir da máquina que iniciou a sessão, e é o mesmo em cada shell, incluindo PowerShell.

O comando publica uma mensagem e sai:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

O CLI enfileira a mensagem na sessão e sai sem esperar por uma resposta. Use-o para orientar uma sessão de longa duração, enfileire a próxima etapa enquanto a atual ainda está terminando, ou envie follow-ups a partir de um [script de CI](/docs/pt/self-hosted-environments-testing#run-the-test-loop). Você também pode canalizar a mensagem em stdin em vez de passá-la como um argumento: `echo "your message" | claude -p --cloud <session-id>`.

Para `<session-id>`, passe o ID simples, como `session_...` ou `cse_...`, ou a URL `claude.ai/code/<id>` da sessão, com ou sem o esquema ou string de consulta. Encontre o ID em sua lista de sessões em claude.ai/code.

<Note>
  `--cloud` requer uma conta Anthropic. Não está disponível quando Claude Code está configurado para Amazon Bedrock, Google Cloud's Agent Platform ou outro provedor de terceiros. Um [gateway LLM](/docs/pt/llm-gateway) configurado apenas através de `ANTHROPIC_BASE_URL` não conta como um provedor de terceiros para esta verificação, mas você ainda precisa entrar com `claude auth login`. A política `allow_remote_sessions` da sua organização também deve estar habilitada. Um Owner pode ativá-la nas configurações de administrador do Claude Code em claude.ai/admin-settings/claude-code.
</Note>

<h4 id="output-and-errors">
  Saída e erros
</h4>

No sucesso, o comando imprime o ID da sessão e um link para visualizar a sessão:

```
Sent to cloud session.
Session ID: session_01DiUkqY2kzbUbDmW1w96rfi
View: https://claude.ai/code/session_01DiUkqY2kzbUbDmW1w96rfi?from=cli&m=0
```

Passe `--output-format json` para um resultado legível por máquina: `{ok, session_id, url}` no sucesso, ou `{ok: false, session_id, error}` quando o envio falha, por exemplo quando a sessão está faltando ou arquivada. Erros de configuração, como um provedor não suportado ou uma política de organização desabilitada, imprimem em stderr sem JSON. `--output-format stream-json` não é suportado com `--cloud <session-id>`.

O CLI prefixos erros com `Error: `. Uma entrega falhada é envolvida como `failed to send message to cloud session <id>: <reason>`.

| Mensagem                                                                                                                    | O que significa                                                                                                                                                                                                                                                                                                                |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Cloud sessions aren't available with <provider>. They run on Anthropic's infrastructure and require an Anthropic account.` | Claude Code está configurado para um provedor de terceiros. A mensagem nomeia o provedor com o rótulo que sua configuração usa, como `Amazon Bedrock` ou `Google Vertex AI`. Remova a configuração desse provedor, por exemplo desconfigurar `CLAUDE_CODE_USE_BEDROCK`, e entre com uma conta Anthropic (`claude auth login`). |
| `Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.`                | A política de organização `allow_remote_sessions` está desabilitada.                                                                                                                                                                                                                                                           |
| `Couldn't verify your organization's policy for cloud sessions. Check your network connection and try again.`               | Claude Code não conseguiu buscar a política de sua organização, então recusa o envio em vez de assumir que as sessões em nuvem são permitidas. Verifique sua conexão de rede e tente novamente.                                                                                                                                |
| `Attaching to an existing cloud session is not enabled for your account.`                                                   | Você executou `--cloud <session-id>` sem `-p`. Envie a mensagem com `claude -p "your message" --cloud <session-id>`.                                                                                                                                                                                                           |
| `Session not found: <id>`                                                                                                   | O ID ou URL não corresponde a uma sessão que você pode acessar. Verifique-o contra a URL claude.ai/code da sessão.                                                                                                                                                                                                             |
| `cloud session <id> is archived and cannot accept new messages`                                                             | A sessão foi arquivada. Inicie uma nova sessão em vez disso.                                                                                                                                                                                                                                                                   |

<h3 id="from-cloud-to-terminal">
  Da nuvem para o terminal
</h3>

Puxe uma sessão em nuvem para seu terminal usando qualquer um destes:

* **Usando `--teleport`**: a partir da linha de comando, execute `claude --teleport` para um seletor de sessão interativo, ou `claude --teleport <session-id>` para retomar uma sessão específica diretamente. Se você tiver alterações não confirmadas, será solicitado que você as guarde primeiro.
* **Usando `/teleport`**: dentro de uma sessão CLI existente, execute `/teleport` ou `/tp` para abrir o mesmo seletor de sessão sem reiniciar Claude Code.
* **De `/tasks`**: execute `/tasks` para ver suas sessões em segundo plano, depois pressione `t` para teleportar para uma.
* **De claude.ai/code**: selecione **Open in > Terminal** no menu de sessão para copiar um comando que você pode colar em seu terminal.
* **De dentro da sessão em nuvem**: digite `/teleport` e Claude Code responde com o comando exato `claude --teleport <session-id>` para essa sessão, pronto para ser executado a partir de um checkout do repositório. Requer Claude Code v2.1.223 ou posterior no ambiente da sessão.

Quando você teleporta uma sessão, Claude verifica se você está no repositório correto, busca e faz checkout da branch da sessão em nuvem e carrega o histórico completo da conversa em seu terminal. O terminal obtém sua própria cópia da sessão: novo trabalho lá fica local e não aparece na sessão em nuvem em claude.ai ou no aplicativo móvel Claude. Para continuar orientando a partir do seu telefone após teleportar, inicie [`/remote-control`](/docs/pt/remote-control) na sessão local.

`--teleport` é distinto de `--resume`. `--resume` reabre uma conversa do histórico local desta máquina e não lista sessões em nuvem; `--teleport` puxa uma sessão em nuvem e sua branch.

<h4 id="teleport-requirements">
  Requisitos de teleportação
</h4>

Teleport verifica esses requisitos antes de retomar uma sessão. Se algum requisito não for atendido, você verá um erro ou será solicitado a resolver o problema.

| Requisito           | Detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Estado git limpo    | Seu diretório de trabalho não deve ter alterações não confirmadas. Teleport solicita que você guarde as alterações se necessário.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Repositório correto | Você deve executar `--teleport` a partir de um checkout do mesmo repositório, não de um fork. Se você executá-lo a partir de um checkout de um repositório diferente, Claude Code mostra um erro que nomeia tanto o repositório da sessão quanto o repositório do seu checkout. Antes da v2.1.219, o erro não nomeava o repositório do seu checkout. Se Claude Code não conseguir analisar seu remoto em um nome de host, por exemplo um alias de host SSH como `git@work:owner/repo.git`, ele pede que você confirme, e aceita o checkout quando o proprietário do remoto e o nome do repositório correspondem ao repositório da sessão. |
| Branch disponível   | A branch da sessão em nuvem deve ter sido enviada para o remoto. Teleport busca e faz checkout automaticamente.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Mesma conta         | Você deve estar autenticado na mesma conta claude.ai usada na sessão em nuvem.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

<h4 id="teleport-is-unavailable">
  `--teleport` não está disponível
</h4>

Teleport requer autenticação de assinatura claude.ai. Se você estiver autenticado via chave de API, execute `/login` para entrar com sua conta claude.ai em vez disso. Se o erro nomear seu provedor em vez disso, as sessões em nuvem não estão disponíveis através de provedores de terceiros; veja a [tabela de erros](#output-and-errors). Se você já estiver conectado via claude.ai e `--teleport` ainda não estiver disponível, sua organização pode ter desabilitado as sessões em nuvem.

<h2 id="work-with-sessions">
  Trabalhar com sessões
</h2>

As sessões aparecem na barra lateral em claude.ai/code. De lá, você pode revisar alterações, compartilhar com colegas de equipe, arquivar trabalho concluído ou excluir sessões permanentemente.

<h3 id="take-back-a-queued-message">
  Recuperar uma mensagem enfileirada
</h3>

Se você enviar uma mensagem enquanto Claude está trabalhando, a mensagem fica enfileirada até que Claude a leia. Para recuperar uma mensagem enfileirada, clique no ✕ nela. O texto retorna à caixa de mensagem para que você possa editá-lo ou enviar algo diferente.

Se Claude já tiver lido a mensagem, ela permanece na conversa.

<h3 id="manage-context">
  Gerenciar contexto
</h3>

As sessões em nuvem suportam [comandos integrados](/docs/pt/commands) que produzem saída de texto. Comandos que só funcionam na interface do terminal, como `/plugin` ou `/resume`, não estão disponíveis. Comandos que abrem um seletor ou painel no terminal se comportam de forma diferente nas sessões em nuvem:

* **`/model`, `/effort`, `/color` e `/rename`**: passe o valor como um argumento, por exemplo `/model sonnet`, em vez de abrir o seletor do terminal ou controle deslizante. Os formulários de argumento exigem Claude Code v2.1.205 ou posterior no ambiente da sessão e seguem as [notas de disponibilidade](/docs/pt/commands#all-commands) de cada comando.
* **`/fast`**: alterna o [modo rápido](/docs/pt/fast-mode#use-fast-mode-in-cloud-sessions) para a sessão quando o modo rápido está [disponível em sua conta](/docs/pt/fast-mode#requirements). Requer Claude Code v2.1.271 ou posterior no ambiente da sessão.
* **`/config`**: no seu navegador em claude.ai/code, abre a seção Claude Code de suas configurações em vez de definir um valor, e o texto após o comando, incluindo `key=value`, é ignorado. Para alterar uma configuração para uma sessão em nuvem, defina uma [variável de ambiente](/docs/pt/cloud-environments#set-environment-variables) no ambiente, ou em uma sessão com um repositório, confirme a chave no `.claude/settings.json` desse repositório. [Configurações em sessões em nuvem](/docs/pt/settings#settings-in-cloud-sessions) lista o que cada sessão lê.

Para gerenciamento de contexto especificamente:

| Comando    | Funciona em sessões em nuvem | Notas                                                                                                             |
| :--------- | :--------------------------- | :---------------------------------------------------------------------------------------------------------------- |
| `/compact` | Sim                          | Resume a conversa para liberar contexto. Aceita instruções de foco opcionais como `/compact keep the test output` |
| `/context` | Sim                          | Mostra o que está atualmente na janela de contexto                                                                |
| `/clear`   | Não                          | Inicie uma nova sessão na barra lateral                                                                           |

A compactação automática é executada automaticamente quando a janela de contexto se aproxima da capacidade. As sessões em nuvem definem [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/pt/env-vars) por si só, portanto a compactação é acionada no meio da [janela de compactação automática](/docs/pt/model-config#set-the-auto-compact-window) em vez de quando a janela se enche. Esse valor substitui um que você adiciona em suas [variáveis de ambiente](/docs/pt/cloud-environments#set-environment-variables), portanto adicionar a variável lá não altera quando a compactação é acionada.

Para alterar a janela de compactação automática, defina [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/pt/env-vars) em suas variáveis de ambiente, ou execute [`/autocompact`](/docs/pt/commands#all-commands) com uma contagem de tokens em uma sessão onde a variável não está definida.

[Subagentes](/docs/pt/sub-agents) funcionam da mesma forma que funcionam localmente. Claude pode gerá-los com a ferramenta Agent para descarregar pesquisa ou trabalho paralelo em uma janela de contexto separada, mantendo a conversa principal mais leve. Subagentes definidos no `.claude/agents/` do seu repositório são detectados automaticamente.

[Equipes de agentes](/docs/pt/agent-teams) estão desativadas por padrão, mas podem ser habilitadas adicionando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` às suas [variáveis de ambiente](/docs/pt/cloud-environments#set-environment-variables).

<h3 id="permission-modes-in-cloud-sessions">
  Modos de permissão em sessões em nuvem
</h3>

Você escolhe o [modo de permissão](/docs/pt/permission-modes) de uma sessão em nuvem no [menu suspenso de modo](/docs/pt/permission-modes#switch-permission-modes), tanto quando você cria a tarefa quanto enquanto a sessão é executada. Quando você reabre uma sessão cujo [ambiente hospedado pela Anthropic expirou](#environment-expired), ou envia uma mensagem para uma sessão que um executor auto-hospedado [liberou enquanto estava ocioso](/docs/pt/self-hosted-environments-reference#runner-cli-flags), Claude Code retoma a sessão no modo de permissão em que estava.

<h3 id="review-changes">
  Revisar alterações
</h3>

Cada sessão mostra um indicador de diff com linhas adicionadas e removidas, como `+42 -18`. Selecione-o para abrir a visualização de diff, deixe comentários inline em linhas específicas e envie-os para Claude com sua próxima mensagem.

A visualização de diff compara as alterações da sessão em relação ao seu branch base por padrão. Para comparar com qualquer outro branch no repositório, selecione **Compare against** e escolha um.

Claude Code calcula esses diffs, incluindo os diffs por arquivo mostrados conforme Claude edita, a partir do conteúdo bruto do blob git, portanto os drivers de diff e filtros `textconv` configurados no repositório não se aplicam. Para um arquivo em um repositório que não é um dos checkouts da própria sessão, como um clonado dentro do workspace durante a sessão, o diff por arquivo mostra a edição de Claude em si em vez de uma comparação git.

Consulte [Revisar e iterar](/docs/pt/web-quickstart#review-and-iterate) para o passo a passo completo, incluindo criação de PR. Para fazer Claude monitorar o PR para falhas de CI e comentários de revisão automaticamente, consulte [Corrigir automaticamente pull requests](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Compartilhar sessões
</h3>

Para compartilhar uma sessão, alterne sua visibilidade de acordo com os tipos de conta abaixo. Depois disso, compartilhe o link da sessão como está. Os destinatários veem o estado mais recente quando abrem o link, mas sua visualização não é atualizada em tempo real.

<h4 id="share-from-an-enterprise-or-team-account">
  Compartilhar de uma conta Enterprise ou Team
</h4>

Para contas Enterprise e Team, as duas opções de visibilidade são **Private** e **Team**. A visibilidade de Team torna a sessão visível para outros membros de sua organização claude.ai. As sessões do [Claude no Slack](/docs/pt/slack) são compartilhadas automaticamente com visibilidade de Team.

A verificação de acesso ao repositório está habilitada por padrão, com base na conta do GitHub conectada à conta do destinatário. O nome de exibição de sua conta é visível para todos os destinatários com acesso.

<h4 id="share-from-a-max-or-pro-account">
  Compartilhar de uma conta Max ou Pro
</h4>

Para contas Max e Pro, as duas opções de visibilidade são **Private** e **Public**. A visibilidade Public torna a sessão visível para qualquer usuário conectado a claude.ai.

Verifique sua sessão para conteúdo sensível antes de compartilhar. As sessões podem conter código e credenciais de repositórios privados do GitHub. A verificação de acesso ao repositório não está habilitada por padrão.

Para exigir que os destinatários tenham acesso ao repositório ou para ocultar seu nome de sessões compartilhadas, vá para [**Settings > Claude Code > Sharing settings**](https://claude.ai/settings/claude-code).

<h3 id="archive-sessions">
  Arquivar sessões
</h3>

Você pode arquivar sessões para manter sua lista de sessões organizada. As sessões arquivadas ficam ocultas da lista de sessões padrão, mas podem ser visualizadas filtrando por sessões arquivadas.

Para arquivar uma sessão, passe o mouse sobre a sessão na barra lateral e selecione o ícone de arquivo.

<h3 id="delete-sessions">
  Excluir sessões
</h3>

Excluir uma sessão remove permanentemente a sessão e seus dados. Esta ação não pode ser desfeita. Você pode excluir uma sessão de duas maneiras:

* **Na barra lateral**: filtre por sessões arquivadas, passe o mouse sobre a sessão que deseja excluir e selecione o ícone de exclusão
* **No menu da sessão**: abra uma sessão, selecione o menu suspenso ao lado do título da sessão e selecione **Delete**

Você será solicitado a confirmar antes de uma sessão ser excluída.

<h2 id="auto-fix-pull-requests">
  Corrigir automaticamente pull requests
</h2>

Claude pode observar um pull request e responder automaticamente a falhas de CI e comentários de revisão. Claude se inscreve na atividade do GitHub no PR, e quando uma verificação falha ou um revisor deixa um comentário, Claude investiga e envia uma correção se uma for clara.

<Note>
  Auto-fix requer que o Claude GitHub App esteja instalado em seu repositório. Se você ainda não fez isso, instale-o a partir da [página do GitHub App](https://github.com/apps/claude).
</Note>

Existem algumas maneiras de ativar auto-fix dependendo de onde o PR veio e qual dispositivo você está usando:

* **PRs criados em uma sessão na nuvem**: abra a sessão em claude.ai/code, abra a barra de status de CI e selecione **Auto-fix**
* **A partir do seu terminal**: execute [`/autofix-pr`](/docs/pt/commands) enquanto estiver na branch do PR. Claude Code detecta o PR aberto com `gh`, gera uma sessão na nuvem e ativa auto-fix em uma etapa
* **A partir do aplicativo móvel**: diga a Claude para corrigir automaticamente o PR, por exemplo "watch this PR and fix any CI failures or review comments"
* **Qualquer PR existente**: cole a URL do PR em uma sessão e diga a Claude para corrigir automaticamente

Auto-fix é um toggle por PR. Para parar de monitorar, abra a barra de status de CI na sessão em claude.ai/code e desmarque o toggle **Auto-fix**, ou diga a Claude para parar de observar o PR.

<h3 id="how-claude-responds-to-pr-activity">
  Como Claude responde à atividade de PR
</h3>

Quando auto-fix está ativo, Claude recebe eventos do GitHub para o PR incluindo novos comentários de revisão e falhas de verificação de CI. Para cada evento, Claude investiga e decide como proceder:

* **Correções claras**: se Claude está confiante em uma correção e ela não entra em conflito com instruções anteriores, Claude faz a alteração, envia e explica o que foi feito na sessão
* **Solicitações ambíguas**: se um comentário de revisor pode ser interpretado de múltiplas maneiras ou envolve algo arquitetonicamente significativo, Claude pergunta a você antes de agir
* **Eventos duplicados ou sem ação**: se um evento é duplicado ou não requer alteração, Claude o anota na sessão e continua

GitHub não emite um webhook quando a branch base avança e cria um conflito de merge, então auto-fix não pode reagir a conflitos por conta própria. Para resolver um conflito, abra a sessão e peça a Claude para fazer rebase.

Claude pode responder a threads de comentários de revisão no GitHub como parte da resolução deles. Essas respostas são postadas usando sua conta GitHub, então aparecem sob seu nome de usuário, mas cada resposta é rotulada como vindo de Claude Code para que os revisores saibam que foi escrita pelo agente e não por você diretamente.

<Warning>
  Se seu repositório usa automação acionada por comentário, como Atlantis, Terraform Cloud ou GitHub Actions personalizadas que são executadas em eventos `issue_comment`, esteja ciente de que Claude pode responder em seu nome, o que pode acionar esses fluxos de trabalho. Revise a automação de seu repositório antes de ativar auto-fix e considere desabilitar auto-fix para repositórios onde um comentário de PR pode implantar infraestrutura ou executar operações privilegiadas.
</Warning>

<h2 id="security-and-isolation">
  Segurança e isolamento
</h2>

Cada sessão em nuvem é separada de sua máquina e de outras sessões através de várias camadas:

* **Máquinas virtuais isoladas**: cada sessão é executada em uma VM isolada gerenciada pela Anthropic. As sessões que sua organização roteia para um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) são executadas em sua própria infraestrutura em vez disso, onde o isolamento é responsabilidade de sua implantação
* <span id="default-allowed-domains" />**Controles de acesso à rede**: em ambientes hospedados pela Anthropic, o acesso à rede é limitado por padrão e pode ser desabilitado. Veja [Acesso à rede](/docs/pt/cloud-environments#network-access) para os níveis de acesso, os [domínios padrão permitidos](/docs/pt/cloud-environments#default-allowed-domains) e o tráfego que não passa pela lista de permissões. Em um ambiente auto-hospedado, você restringe a saída da sessão em seu próprio limite de rede. Ao executar com acesso à rede desabilitado, Claude Code ainda pode se comunicar com a API Anthropic, o que pode permitir que dados saiam da VM.
* **Proteção de credenciais**: em ambientes hospedados pela Anthropic, credenciais git e chaves de assinatura ficam fora da sandbox, e um proxy autentica em nome da sessão com credenciais com escopo. Em um ambiente auto-hospedado, sua implantação fornece credenciais git; veja [Configure git](/docs/pt/self-hosted-environments-deploy#configure-git)
* **Credenciais de API**: em ambientes hospedados pela Anthropic nos planos Pro e Max, chaves que você [adiciona a um ambiente em nuvem](/docs/pt/cloud-environments#add-api-credentials) ficam fora da sandbox da mesma forma, anexadas a solicitações correspondentes depois que saem da sessão. Um ambiente auto-hospedado não tem credenciais de API, e os planos Team e Enterprise ainda não têm
* **Análise segura**: o código é analisado e modificado dentro do ambiente isolado da sessão antes de criar PRs

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Para erros de API de tempo de execução que aparecem na conversa como `API Error: 500`, `529 Overloaded`, `429` ou `Prompt is too long`, veja a [referência de erros](/docs/pt/errors). Esses erros e suas correções são compartilhados com o CLI e aplicativo Desktop. As seções abaixo cobrem problemas específicos para sessões em nuvem.

<h3 id="session-creation-failed">
  Session creation failed
</h3>

Se uma nova sessão falha ao iniciar com `Session creation failed` ou fica presa em provisioning, Claude Code não conseguiu alocar uma VM para a sessão.

* Verifique [status.claude.com](https://status.claude.com) para incidentes de sessão em nuvem
* Tente novamente após um minuto, já que a capacidade é provisionada sob demanda
* Confirme que sua conexão GitHub pode acessar o repositório seguindo [No repositories appear after connecting GitHub](/docs/pt/web-quickstart#no-repositories-appear-after-connecting-github)

<h3 id="unable-to-get-organization-uuid">
  Unable to get organization UUID
</h3>

`claude --cloud` e `claude --teleport` requerem entrada com uma conta claude.ai. Se você autenticar com uma chave de API, ou seus detalhes de conta armazenados estão obsoletos, esses comandos falham com `Unable to get organization UUID` ou uma mensagem de que a autenticação de chave de API não é suficiente. Com autenticação de chave de API ou detalhes de conta obsoletos, executar `claude --teleport` sem um ID de sessão mostra `Error loading Claude Code sessions` no seletor de sessão em vez de qualquer mensagem, e a mesma correção se aplica.

Execute `/login` para entrar com sua conta claude.ai, depois tente novamente o comando. Se o erro nomear seu provedor em vez disso, veja a [tabela de erros](#output-and-errors): as sessões em nuvem não estão disponíveis através de provedores de terceiros.

<h3 id="remote-control-session-expired-or-access-denied">
  Remote Control session expired or access denied
</h3>

`--teleport` se conecta através da mesma infraestrutura de sessão Remote Control que as sessões em nuvem usam, então erros de autenticação e expiração de sessão aparecem com wording de Remote Control. Você pode ver `Remote Control session expired` ou `Access denied`. O token de conexão é de curta duração e limitado à sua conta.

* Execute `/login` localmente para atualizar suas credenciais, depois reconecte
* Confirme que você está conectado à mesma conta que possui a sessão
* Se você ver `Remote Control may not be available for this organization`, um Owner não habilitou as sessões em nuvem para sua organização

<h3 id="environment-expired">
  Environment expired
</h3>

As sessões em nuvem param após um período de inatividade e a VM da sessão é recuperada. Uma sessão é considerada inativa enquanto aguarda você aprovar uma chamada de ferramenta [MCP connector](/docs/pt/cloud-environments#network-access) ou entrar em um servidor MCP, e pode expirar durante essa espera.

Reabra a sessão de [claude.ai/code](https://claude.ai/code) para provisionar uma VM fresca com seu histórico de conversa restaurado. O trabalho em segundo plano que ainda estava em execução quando a VM foi recuperada, como subagentes e comandos shell, não é restaurado.

<h2 id="limitations">
  Limitações
</h2>

Antes de confiar em sessões em nuvem para um fluxo de trabalho, leve em conta essas restrições:

* **Limites de taxa**: sessões em nuvem compartilham limites de taxa com todo o outro uso de Claude e Claude Code dentro de sua conta. Executar múltiplas tarefas em paralelo consome mais limites de taxa proporcionalmente. Não há cobrança de computação separada para a VM em nuvem.
* **Autenticação de repositório**: você pode apenas mover uma sessão em nuvem para seu terminal quando está autenticado na mesma conta
* **Restrições de plataforma**: clonagem de repositório e criação de pull request requerem GitHub. Instâncias [GitHub Enterprise Server](/docs/pt/github-enterprise-server) auto-hospedadas são suportadas para planos Team e Enterprise. Você pode enviar um repositório GitLab, Bitbucket ou outro repositório não-GitHub para uma sessão em nuvem como um [pacote local](#send-local-repositories-without-github) definindo `CCR_FORCE_BUNDLE=1`, mas a sessão não pode enviar resultados de volta para esse remoto
* **IP allowlist da organização**: sessões em nuvem chamam a API Anthropic a partir de infraestrutura gerenciada pela Anthropic, não de sua rede, enquanto sessões em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) a chamam a partir de sua própria rede. Se sua organização tem [IP allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) habilitado, cada sessão em nuvem hospedada pela Anthropic falha com um erro de autenticação. O mesmo se aplica a [Code Review](/docs/pt/code-review) e a [routines](/docs/pt/routines) que são executadas em ambientes hospedados pela Anthropic; uma routine roteada para um ambiente auto-hospedado chama a API a partir de sua própria rede. Entre em contato com [suporte Anthropic](https://support.claude.com/) para isentar serviços hospedados pela Anthropic do allowlist de IP de sua organização.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Ambientes em nuvem](/docs/pt/cloud-environments): configure acesso à rede, variáveis de ambiente e scripts de configuração para sessões em nuvem
* [Projetos](/docs/pt/claude-projects): uma conversa onde Claude coordena sessões em nuvem paralelas em seus repositórios e relata de volta
* [Ultrareview](/docs/pt/ultrareview): execute uma revisão de código profunda multi-agente em uma sandbox em nuvem
* [Routines](/docs/pt/routines): automatize trabalho em um cronograma, via chamada de API ou em resposta a eventos do GitHub
* [Configuração de hooks](/docs/pt/hooks): execute scripts em eventos do ciclo de vida da sessão
* [Todas as configurações](/docs/pt/settings-reference): todas as opções de configuração
* [Segurança](/docs/pt/security): garantias de isolamento e tratamento de dados
* [Uso de dados](/docs/pt/data-usage): o que Anthropic retém de sessões em nuvem
* [Claude Tag](https://claude.com/docs/claude-tag/overview): um @Claude gerenciado pela organização no Slack que é executado na mesma infraestrutura em nuvem
