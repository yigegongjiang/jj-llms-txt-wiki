> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comece com Claude Code na nuvem

> Execute Claude Code na nuvem a partir do seu navegador ou telefone. Conecte um repositório GitHub, envie uma tarefa e revise o PR sem configuração local.

<Note>
  As sessões na nuvem estão disponíveis nos planos Pro, Max e Team, e para usuários Enterprise com assentos premium ou assentos Chat + Claude Code.
</Note>

Uma sessão na nuvem executa Claude Code em infraestrutura de nuvem em vez de sua máquina, gerenciada pela Anthropic por padrão. Este guia de início rápido inicia uma a partir de [claude.ai/code](https://claude.ai/code) no seu navegador. Você também pode iniciar uma a partir do aplicativo móvel Claude, do aplicativo Desktop ou do seu terminal com `claude --cloud`.

Você precisará de um repositório GitHub para [começar](#connect-github). Claude o clona em uma máquina virtual isolada, faz alterações e envia uma branch para você revisar. As sessões persistem entre dispositivos, portanto uma tarefa que você inicia no seu laptop está pronta para revisar no seu telefone mais tarde.

As sessões na nuvem funcionam bem para:

* **Tarefas paralelas**: execute várias tarefas independentes ao mesmo tempo, cada uma em sua própria sessão e branch, sem gerenciar múltiplas worktrees
* **Repositórios que você não tem localmente**: Claude clona o repositório novo a cada sessão, então você não precisa tê-lo verificado
* **Tarefas que não precisam de direcionamento frequente**: envie uma tarefa bem definida, faça outra coisa e revise o resultado quando Claude terminar
* **Perguntas sobre código e exploração**: entenda uma base de código ou rastreie como um recurso é implementado sem um checkout local

Para trabalho que precisa de sua configuração local, ferramentas ou ambiente, executar Claude Code localmente ou usar [Remote Control](/docs/pt/remote-control) é mais adequado.

<h2 id="how-sessions-run">
  Como as sessões são executadas
</h2>

As etapas abaixo descrevem sessões hospedadas pela Anthropic. Em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments), o clone e tudo depois dele são executados nos seus próprios runners da organização, onde os limites de rede, configuração e comportamento de push são configurados pelo operador. Quando você envia uma tarefa:

1. **Clone e prepare**: seu repositório é clonado para uma VM gerenciada pela Anthropic, e seu [script de configuração](/docs/pt/cloud-environments#setup-scripts) é executado se configurado.
2. **Configure a rede**: o acesso à internet é definido com base no [nível de acesso](/docs/pt/cloud-environments#access-levels) do seu ambiente.
3. **Trabalhe**: Claude analisa código, faz alterações, executa testes e verifica seu trabalho. Você pode assistir e direcionar durante todo o processo, ou se afastar e voltar quando terminar.
4. **Envie a branch**: quando Claude atinge um ponto de parada, ele envia sua branch para o GitHub. Você revisa o diff, deixa comentários inline, cria um PR ou envia outra mensagem para continuar.

A sessão não fecha quando a branch é enviada. A criação de PR e edições adicionais acontecem dentro da mesma conversa.

<h2 id="compare-ways-to-run-claude-code">
  Compare as maneiras de executar Claude Code
</h2>

Claude Code se comporta da mesma forma em todos os lugares. O que muda é onde a sessão é executada e se sua configuração local está disponível:

|                                                | Sessão em nuvem                                                                                                        | Sessão local                                                                                                                                     | Sessão local com [Remote Control](/docs/pt/remote-control)                        |
| :--------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------- |
| **O código é executado em**                    | VM de nuvem, gerenciada pela Anthropic por padrão                                                                      | Sua máquina                                                                                                                                      | Sua máquina                                                                  |
| **Você inicia a partir de**                    | claude.ai/code, o aplicativo móvel Claude, o aplicativo Desktop com **Cloud** selecionado, ou `claude --cloud`         | Seu terminal, seu IDE, ou o aplicativo Desktop com **Local** selecionado                                                                         | Seu terminal, a extensão VS Code, ou o aplicativo Desktop                    |
| **Você conversa de**                           | claude.ai, o aplicativo móvel, ou o aplicativo Desktop                                                                 | Onde você a iniciou                                                                                                                              | claude.ai ou o aplicativo móvel, bem como onde você a iniciou                |
| **Usa sua configuração local**                 | Não, apenas repositório                                                                                                | Sim                                                                                                                                              | Sim                                                                          |
| **Requer GitHub**                              | Sim, ou [agrupe um repositório local](/docs/pt/claude-code-on-the-web#send-local-repositories-without-github) via `--cloud` | Não                                                                                                                                              | Não                                                                          |
| **Continua funcionando se você desconectar**   | Sim                                                                                                                    | Não                                                                                                                                              | Enquanto a sessão permanecer aberta em sua máquina                           |
| **[Modos de permissão](/docs/pt/permission-modes)** | Aceitar edições, Plan, Auto                                                                                            | Todos os modos no terminal; consulte [Alternar modos de permissão](/docs/pt/permission-modes#switch-permission-modes) para o IDE e aplicativo Desktop | Manual, Aceitar edições, ou Plan a partir de claude.ai e do aplicativo móvel |
| **Acesso à rede**                              | Configurável por ambiente                                                                                              | Rede da sua máquina                                                                                                                              | Rede da sua máquina                                                          |

Consulte a documentação do [quickstart do terminal](/docs/pt/quickstart), [aplicativo Desktop](/docs/pt/desktop), ou [Remote Control](/docs/pt/remote-control) para configurar sessões locais.

<h2 id="connect-github">
  Conecte GitHub
</h2>

Conectar GitHub é um processo único. Se você já usa a CLI do GitHub, você pode [fazer isso do seu terminal](#connect-from-your-terminal) em vez do navegador.

<Note>
  Em planos Team e Enterprise, a etapa **Sign in with GitHub** funciona apenas após um [Owner](/docs/pt/server-managed-settings#access-control) da sua organização Claude ativar o conector GitHub em [**Admin settings > Connectors**](https://claude.ai/admin-settings/connectors). Até então, essa etapa mostra "GitHub access is required for Claude Code on the web" em vez de um botão de login. Após o conector estar ativado, recarregue [claude.ai/code](https://claude.ai/code) e comece novamente a partir da primeira etapa. Um segundo toggle, [Quick web setup](/docs/pt/claude-code-on-the-web#github-authentication-options) em [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code), é opcional: com ele ativado, `/web-setup` funciona e a integração cria o ambiente para os membros.
</Note>

<Steps>
  <Step title="Visite claude.ai/code">
    Vá para [claude.ai/code](https://claude.ai/code) e faça login com sua conta claude.ai.
  </Step>

  <Step title="Sign in with GitHub">
    Após fazer login, claude.ai/code solicita que você conecte o GitHub. Siga o prompt, e claude.ai/code o envia para a página de autorização do GitHub. Aprove a solicitação de autorização, e o GitHub o retorna para claude.ai/code. As sessões em nuvem funcionam com repositórios GitHub existentes. Para iniciar um novo projeto, [crie um repositório vazio no GitHub](https://github.com/new) primeiro.

    Com essa conexão, uma sessão pode clonar qualquer repositório público, mas pode trabalhar em um repositório privado apenas quando o Claude GitHub App está instalado nele. [Instale o Claude GitHub App](https://github.com/apps/claude/installations/new) em cada conta GitHub ou organização cujos repositórios privados você deseja usar. Em uma organização GitHub, um proprietário da organização pode precisar aprovar a instalação. Instalar o App também ativa [Auto-fix](/docs/pt/claude-code-on-the-web#auto-fix-pull-requests), que permite que Claude responda a falhas de CI e comentários de revisão em pull requests nesses repositórios.

    Se a integração solicitar que você instale o Claude GitHub App neste ponto e você preferir fazer isso mais tarde, clique em **Skip**.
  </Step>

  <Step title="Configure seu ambiente padrão">
    Um [ambiente em nuvem](/docs/pt/cloud-environments) é a configuração salva que controla qual acesso à rede Claude tem durante as sessões e o que é executado quando uma sessão é iniciada. O que acontece após você conectar o GitHub depende do seu plano:

    * **Pro e Max**: a integração cria um ambiente chamado **Default** para você.
    * **Team e Enterprise**: a integração mostra um formulário **Create your first cloud environment**. Deixe o nome pré-preenchido e o acesso à rede inalterados e clique em **Create & finish** para criar o ambiente **Default**. Se um Owner ativou [Quick web setup](/docs/pt/claude-code-on-the-web#github-authentication-options), a integração cria **Default** para você em vez disso.

    **Default** usa [acesso à rede `Trusted`](/docs/pt/cloud-environments#access-levels): as sessões alcançam [registros de pacotes comuns](/docs/pt/cloud-environments#default-allowed-domains) e outros domínios na lista de permissões, e nada mais através da rede da sessão. Veja [Ferramentas instaladas](/docs/pt/cloud-environments#installed-tools) para o que está disponível sem nenhuma configuração.

    Para um primeiro projeto, o ambiente **Default** funciona como está. Para alterar seu acesso à rede, adicionar variáveis de ambiente ou executar um [script de configuração](/docs/pt/cloud-environments#setup-scripts) antes das sessões iniciarem, [edite-o ou crie ambientes adicionais](/docs/pt/cloud-environments#configure-your-environment).
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Conecte do seu terminal
</h3>

Se você já usa a CLI do GitHub (`gh`), você pode conectar GitHub para sessões em nuvem do seu terminal. Isso requer a [CLI do Claude Code](/docs/pt/quickstart). Em planos Team e Enterprise, `/web-setup` está disponível apenas após um Owner ativar [Quick web setup](/docs/pt/claude-code-on-the-web#github-authentication-options).

Quando você executa `/web-setup`, Claude Code lê o token que `gh auth token` imprime, pede que você confirme e envia o token para Anthropic. Anthropic o armazena criptografado com sua conta claude.ai, e suas sessões em nuvem o usam para acesso ao GitHub até você [removê-lo](#remove-the-web-setup-token). Uma sessão em nuvem que você inicia por conta própria pode então acessar qualquer repositório que esse token possa acessar, sem nenhuma instalação do Claude GitHub App. Threads em um [projeto](/docs/pt/claude-projects#set-up-github-access) ainda precisam do Claude GitHub App.

Se você já conectou o GitHub no navegador, `/web-setup` avisa que continuar substitui essa conexão para suas sessões em nuvem.

<Note>
  Organizações com [Zero Data Retention](/docs/pt/zero-data-retention) habilitado não podem usar `/web-setup` ou outros recursos de sessão em nuvem. Se a CLI do GitHub não estiver instalada ou autenticada, Claude Code abre o fluxo de integração do navegador em vez disso.
</Note>

<Steps>
  <Step title="Autentique com a CLI do GitHub">
    No seu shell, autentique a CLI do GitHub se você ainda não o fez:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Faça login no Claude">
    Na CLI do Claude Code, execute `/login` para fazer login com sua conta claude.ai. Pule esta etapa se você já estiver conectado com uma conta claude.ai. Autenticar com uma chave de API não conta. Para verificar, execute `/status` e confirme que a linha **Login method** mostra uma conta claude.ai.
  </Step>

  <Step title="Execute /web-setup">
    Na CLI do Claude Code, execute:

    ```text theme={null}
    /web-setup
    ```

    Confirme o prompt para enviar seu token `gh` para sua conta Claude. Em caso de sucesso, Claude Code imprime `Connected as <your-github-username>` e abre [claude.ai/code](https://claude.ai/code) no seu navegador. Se você ainda não tiver um ambiente em nuvem, `/web-setup` cria um com acesso à rede Trusted e sem script de configuração. Você pode [editar o ambiente ou adicionar variáveis](/docs/pt/cloud-environments#configure-your-environment) depois. Após `/web-setup` ser concluído, você pode iniciar sessões em nuvem do seu terminal com [`--cloud`](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud) ou configurar tarefas recorrentes com [`/schedule`](/docs/pt/routines).
  </Step>
</Steps>

<h4 id="remove-the-web-setup-token">
  Remova o token `/web-setup`
</h4>

Para remover o token da sua conta Claude, desconecte o GitHub em [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Desconectar deleta as credenciais do GitHub que suas sessões em nuvem usam, quer tenham vindo do navegador ou de `/web-setup`, então as sessões em nuvem perdem acesso ao GitHub até você conectar novamente. Seu `gh` local permanece conectado, e o token permanece válido no GitHub.

Para invalidar o token em si, revogue-o no GitHub. Se você fez login em `gh` através do navegador, o token pertence à entrada **GitHub CLI** em [**Settings > Applications > Authorized OAuth Apps**](https://github.com/settings/applications) no GitHub, e revogar essa entrada também desconecta a CLI do GitHub em suas máquinas. As sessões em nuvem então perdem acesso ao GitHub até você executar `gh auth login` e `/web-setup` novamente.

<h2 id="start-a-task">
  Inicie uma tarefa
</h2>

Com GitHub conectado e um ambiente criado, você está pronto para enviar tarefas.

<Steps>
  <Step title="Selecione um repositório e branch">
    De [claude.ai/code](https://claude.ai/code) ou da aba Code no aplicativo móvel Claude, clique no seletor de repositório abaixo da caixa de entrada e escolha um repositório para Claude trabalhar. Cada repositório mostra um seletor de branch. Altere-o para iniciar Claude a partir de uma branch de recurso em vez da padrão. Você pode adicionar múltiplos repositórios para trabalhar entre eles em uma sessão.
  </Step>

  <Step title="Escolha um modo de permissão">
    O dropdown de modo ao lado da entrada mostra o modo em que a sessão será executada:

    * **Auto**: um classificador revisa as ações de Claude em vez de pedir a você. Aparece quando sua organização permite modo auto e o modelo selecionado o suporta
    * **Aceitar edições**: Claude faz alterações e envia uma branch sem parar para aprovação
    * **Plan**: Claude propõe uma abordagem e aguarda sua aprovação antes de editar arquivos

    As sessões em nuvem não oferecem permissões Manual ou Bypass. Consulte a [lista completa de modos de permissão](/docs/pt/permission-modes#available-modes) para saber o que cada um permite.
  </Step>

  <Step title="Descreva a tarefa e envie">
    Digite uma descrição do que você quer e pressione Enter. Seja específico:

    * Nomeie o arquivo ou função: "Adicione um README com instruções de configuração" ou "Corrija o teste de autenticação falhando em `tests/test_auth.py`" é melhor que "corrigir testes"
    * Cole a saída de erro se você tiver
    * Descreva o comportamento esperado, não apenas o sintoma

    Claude clona os repositórios, executa seu script de configuração se configurado e começa a trabalhar. Cada tarefa recebe sua própria sessão e sua própria branch, portanto você não precisa esperar uma terminar antes de iniciar outra.
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  Pré-preenchimento de sessões
</h2>

Você pode pré-preencher o prompt, repositórios e ambiente para uma nova sessão adicionando parâmetros de consulta à URL [claude.ai/code](https://claude.ai/code). Use isso para construir integrações como um botão no seu rastreador de problemas que abre Claude Code com a descrição do problema como prompt.

| Parâmetro      | Descrição                                                                                                                                                                                                  |
| :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`       | Texto do prompt para pré-preencher na caixa de entrada. O alias `q` também é aceito.                                                                                                                       |
| `prompt_url`   | URL para buscar o texto do prompt, para prompts muito longos para incorporar em uma string de consulta. A URL deve permitir solicitações de origem cruzada. Ignorado quando `prompt` também está definido. |
| `repositories` | Lista separada por vírgula de slugs `owner/repo` para pré-selecionar. O alias `repo` também é aceito.                                                                                                      |
| `environment`  | Nome ou ID do [ambiente](#connect-github) para pré-selecionar.                                                                                                                                             |

Codifique cada valor em URL. O exemplo abaixo abre o formulário com um prompt e um repositório já selecionados:

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  Revise e itere
</h2>

Quando Claude terminar, revise as alterações, deixe feedback em linhas específicas e continue até que o diff pareça correto.

<Steps>
  <Step title="Abra a visualização de diff">
    Um indicador de diff mostra linhas adicionadas e removidas em toda a sessão, por exemplo `+42 -18`. Selecione-o para abrir a visualização de diff, com uma lista de arquivos à esquerda e alterações à direita.

    O diff compara as alterações da sessão contra seu branch base por padrão. Para comparar contra um branch diferente, selecione **Comparar contra** e escolha um.
  </Step>

  <Step title="Deixe comentários inline">
    Selecione qualquer linha no diff, digite seu feedback e pressione Enter. Os comentários se acumulam até você enviar sua próxima mensagem, então são agrupados com ela. Claude vê "em `src/auth.ts:47`, não capture o erro aqui" ao lado de sua instrução principal, portanto você não precisa descrever onde está o problema.
  </Step>

  <Step title="Crie um pull request">
    Quando o diff parecer correto, selecione **Criar PR** no topo da visualização de diff. Você pode abri-lo como um PR completo, um rascunho ou ir para a página de composição do GitHub com um título e descrição gerados.
  </Step>

  <Step title="Continue iterando após o PR">
    A sessão permanece ativa após o PR ser criado. Cole a saída de falha de CI ou comentários do revisor no chat e peça a Claude para resolvê-los. Para ter Claude monitorar o PR automaticamente, consulte [Auto-fix pull requests](/docs/pt/claude-code-on-the-web#auto-fix-pull-requests).
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  Solucionar problemas de configuração
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  Nenhum repositório aparece após conectar GitHub
</h3>

Se você conectou GitHub no navegador, as sessões podem clonar qualquer repositório público, mas um repositório privado aparece apenas quando o Claude GitHub App está instalado na conta ou organização que o possui e o acesso ao repositório da instalação o inclui. [Instale o Claude GitHub App](https://github.com/apps/claude/installations/new) lá, ou peça a um proprietário da organização para instalá-lo ou aprová-lo.

Se você conectou com `/web-setup`, as sessões acessam todos os repositórios que seu token `gh` pode acessar. Execute `gh repo view OWNER/REPO` no seu shell para verificar se seu login do GitHub CLI pode ver o repositório, e execute `/web-setup` novamente se você trocou de contas `gh` desde a conexão.

<h3 id="the-page-only-shows-a-github-login-button">
  A página mostra apenas um botão de login do GitHub
</h3>

As sessões em nuvem requerem uma conta GitHub conectada. Conecte através do fluxo do navegador acima, ou execute `/web-setup` do seu terminal se você usar o GitHub CLI. Se você preferir não conectar GitHub, consulte [Remote Control](/docs/pt/remote-control) para executar Claude Code em sua própria máquina e monitorá-lo do seu navegador ou telefone.

<h3 id="not-available-for-the-selected-organization">
  "Não disponível para a organização selecionada"
</h3>

Organizações Enterprise podem precisar que um Proprietário ative sessões em nuvem. Entre em contato com sua equipe de conta Anthropic.

<h3 id="/web-setup-says-not-signed-in-to-claude">
  `/web-setup` diz "Não conectado ao Claude"
</h3>

Se `/web-setup` responder com "Not signed in to Claude. Run /login first.", o CLI não tem um login válido em claude.ai. Isso também pode acontecer quando um login anterior expirou. Execute `/login`, conecte-se com sua conta claude.ai, depois execute `/web-setup` novamente.

<h3 id="/web-setup-warns-that-your-token-doesn’t-have-the-workflow-scope">
  `/web-setup` avisa que seu token não tem o escopo `workflow`
</h3>

Se `/web-setup` disser que seu token do GitHub CLI não tem o escopo `workflow`, você pode continuar, mas GitHub pode rejeitar alguns pushes feitos com esse token, como pushes que alteram arquivos de fluxo de trabalho do GitHub Actions. Para adicionar o escopo, execute `gh auth refresh -s workflow` no seu shell, depois execute `/web-setup` novamente.

<h3 id="web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` mostra "No commands match" ou "Unknown command"
</h3>

`/web-setup` é executado dentro do Claude Code CLI, não no seu shell. Inicie `claude` primeiro, depois digite `/web-setup` no prompt.

Se você digitou dentro do Claude Code e o menu de comandos mostra `No commands match "/web-setup"`, ou enviá-lo retorna `Unknown command: /web-setup`, o comando está oculto porque um requisito não é atendido. A causa geralmente é que você está autenticado com uma chave de API ou provedor de terceiros em vez de uma assinatura claude.ai. Execute `/login` para conectar-se com sua conta claude.ai.

Nos planos Team e Enterprise, o comando está oculto por padrão: a [alternância Quick web setup](/docs/pt/claude-code-on-the-web#github-authentication-options) está desativada até que um Proprietário a ative. Enquanto estiver desativada, [conecte GitHub do navegador](#connect-github) em vez disso.

O comando também está oculto em dois outros casos:

* Um administrador desativou sessões em nuvem para sua organização. Neste caso, enviar `/web-setup` retorna [`Cloud sessions are disabled by your organization's policy`](/docs/pt/errors#cloud-sessions-are-disabled-by-your-organizations-policy). Antes da v2.1.268, este caso também retornava `Unknown command: /web-setup`.
* Sua organização Enterprise tem [Zero Data Retention](/docs/pt/zero-data-retention) ativado, o que torna sessões em nuvem indisponíveis.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud">
  "Could not create a cloud environment" ou "No cloud environment available" ao usar `--cloud`
</h3>

Os recursos de sessão em nuvem criam um ambiente em nuvem padrão automaticamente se você não tiver um. Se você vir "Could not create a cloud environment", a criação automática falhou. Se você vir "No cloud environment available", seu CLI é anterior à criação automática. Em qualquer caso, execute `/web-setup` no Claude Code CLI, ou adicione um ambiente do [seletor de ambiente](/docs/pt/cloud-environments#configure-your-environment) em [claude.ai/code](https://claude.ai/code).

<h3 id="setup-script-failed">
  Script de configuração falhou
</h3>

O script de configuração saiu com um status diferente de zero, o que bloqueia o início da sessão. Causas comuns:

* Uma instalação de pacote falhou porque o registro não está no seu [nível de acesso à rede](/docs/pt/cloud-environments#access-levels). `Trusted` cobre a maioria dos gerenciadores de pacotes; `None` bloqueia todos eles.
* O script faz referência a um arquivo ou caminho que não existe em um clone recente.
* Um comando que funciona localmente precisa de uma invocação diferente no Ubuntu.

Para depurar, adicione `set -x` no topo do script para ver qual comando falhou. Para comandos não críticos, anexe `|| true` para que não bloqueiem o início da sessão.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Novas sessões travam ou expiram durante a configuração
</h3>

Se novas sessões ficarem presas na etapa do script de configuração ou falharem com um erro genérico de contêiner antes do script terminar, o script provavelmente está excedendo o orçamento de tempo de aproximadamente cinco minutos para construir o [cache de ambiente](/docs/pt/cloud-environments#environment-caching). Etapas pesadas, como puxar imagens Docker grandes, sincronizar árvores de dependência completas ou baixar pesos de modelo, geralmente ultrapassam o limite, especialmente quando são executadas uma após a outra.

Para corrigir isso, reduza o script para que ele termine de forma confiável em menos de cinco minutos:

* Execute instalações independentes em paralelo com `&` e um `wait` final em vez de executá-las em série.
* Mova os maiores downloads para fora do script de configuração e para um [hook SessionStart](/docs/pt/cloud-environments#setup-scripts-vs-sessionstart-hooks) que os inicia em segundo plano, para que a sessão se torne utilizável enquanto eles terminam.
* Remova longas suspensões de repetição do script de configuração, pois um loop de repetição travado conta contra o orçamento.

<h3 id="session-keeps-running-after-closing-the-tab">
  A sessão continua em execução após fechar a aba
</h3>

Isso é por design. Fechar a aba ou navegar para longe não interrompe a sessão. Ela continua em execução em segundo plano até que Claude termine a tarefa atual, depois fica ociosa. Na barra lateral, você pode [arquivar uma sessão](/docs/pt/claude-code-on-the-web#archive-sessions) para ocultá-la da sua lista, ou [deletá-la](/docs/pt/claude-code-on-the-web#delete-sessions) para removê-la permanentemente.

<h2 id="next-steps">
  Próximos passos
</h2>

Agora que você pode enviar e revisar tarefas, estas páginas cobrem o que vem a seguir: iniciar sessões em nuvem do seu terminal, agendar trabalho recorrente e dar instruções permanentes a Claude.

* [Use Claude Code na web](/docs/pt/claude-code-on-the-web): a referência completa, incluindo teletransporte de sessões para seu terminal, compartilhamento de sessão e correção automática de pull requests
* [Configure ambientes em nuvem](/docs/pt/cloud-environments): níveis de acesso à rede, variáveis de ambiente e scripts de configuração para sessões em nuvem
* [Routines](/docs/pt/routines): automatize trabalho em um cronograma, via chamada de API ou em resposta a eventos do GitHub
* [CLAUDE.md](/docs/pt/memory): dê a Claude instruções persistentes e contexto que carregam no início de cada sessão
* Instale o aplicativo móvel Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) para monitorar sessões do seu telefone. Da CLI do Claude Code, `/mobile` mostra um código QR para [claude.ai/mobile](https://claude.ai/mobile) que abre a loja de aplicativos correta para seu telefone.
