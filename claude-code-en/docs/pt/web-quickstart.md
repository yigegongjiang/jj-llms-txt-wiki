> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comece com Claude Code na web

> Execute Claude Code na nuvem a partir do seu navegador ou telefone. Conecte um repositório GitHub, envie uma tarefa e revise o PR sem configuração local.

<Note>
  Claude Code na web está em visualização de pesquisa para usuários Pro, Max e Team, e para usuários Enterprise com assentos premium ou assentos Chat + Claude Code.
</Note>

Claude Code na web é executado em infraestrutura de nuvem gerenciada pela Anthropic em vez de sua máquina. Envie tarefas de [claude.ai/code](https://claude.ai/code) no seu navegador ou no aplicativo móvel Claude.

Você precisará de um repositório GitHub para [começar](#connect-github-and-create-an-environment). Claude o clona em uma máquina virtual isolada, faz alterações e envia uma branch para você revisar. As sessões persistem entre dispositivos, portanto uma tarefa que você inicia no seu laptop está pronta para revisar no seu telefone mais tarde.

Claude Code na web funciona bem para:

* **Tarefas paralelas**: execute várias tarefas independentes ao mesmo tempo, cada uma em sua própria sessão e branch, sem gerenciar múltiplas worktrees
* **Repositórios que você não tem localmente**: Claude clona o repositório novo a cada sessão, então você não precisa tê-lo verificado
* **Tarefas que não precisam de direcionamento frequente**: envie uma tarefa bem definida, faça outra coisa e revise o resultado quando Claude terminar
* **Perguntas sobre código e exploração**: entenda uma base de código ou rastreie como um recurso é implementado sem um checkout local

Para trabalho que precisa de sua configuração local, ferramentas ou ambiente, executar Claude Code localmente ou usar [Remote Control](/docs/pt/remote-control) é mais adequado.

<h2 id="how-sessions-run">
  Como as sessões são executadas
</h2>

Quando você envia uma tarefa:

1. **Clone e prepare**: seu repositório é clonado para uma VM gerenciada pela Anthropic, e seu [script de configuração](/docs/pt/claude-code-on-the-web#setup-scripts) é executado se configurado.
2. **Configure a rede**: o acesso à internet é definido com base no [nível de acesso](/docs/pt/claude-code-on-the-web#access-levels) do seu ambiente.
3. **Trabalhe**: Claude analisa código, faz alterações, executa testes e verifica seu trabalho. Você pode assistir e direcionar durante todo o processo, ou se afastar e voltar quando terminar.
4. **Envie a branch**: quando Claude atinge um ponto de parada, ele envia sua branch para o GitHub. Você revisa o diff, deixa comentários inline, cria um PR ou envia outra mensagem para continuar.

A sessão não fecha quando a branch é enviada. A criação de PR e edições adicionais acontecem dentro da mesma conversa.

<h2 id="compare-ways-to-run-claude-code">
  Compare as maneiras de executar Claude Code
</h2>

Claude Code se comporta da mesma forma em todos os lugares. O que muda é onde o código é executado e se sua configuração local está disponível. O aplicativo Desktop oferece sessões locais e em nuvem, portanto suas respostas abaixo dependem de qual você escolher:

|                                                | Na web                                                                                                                 | Remote Control                        | Terminal CLI        | Aplicativo Desktop             |
| :--------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------ | :------------------ | :----------------------------- |
| **O código é executado em**                    | VM de nuvem Anthropic                                                                                                  | Sua máquina                           | Sua máquina         | Sua máquina ou VM de nuvem     |
| **Você conversa de**                           | claude.ai ou aplicativo móvel                                                                                          | claude.ai ou aplicativo móvel         | Seu terminal        | A interface do Desktop         |
| **Usa sua configuração local**                 | Não, apenas repositório                                                                                                | Sim                                   | Sim                 | Sim para local, não para nuvem |
| **Requer GitHub**                              | Sim, ou [agrupe um repositório local](/docs/pt/claude-code-on-the-web#send-local-repositories-without-github) via `--cloud` | Não                                   | Não                 | Apenas para sessões em nuvem   |
| **Continua funcionando se você desconectar**   | Sim                                                                                                                    | Enquanto o terminal permanecer aberto | Não                 | Depende do tipo de sessão      |
| **[Modos de permissão](/docs/pt/permission-modes)** | Aceitar edições, Plan, Auto                                                                                            | Manual, Aceitar edições, Plan         | Todos os modos      | Depende do tipo de sessão      |
| **Acesso à rede**                              | Configurável por ambiente                                                                                              | Rede da sua máquina                   | Rede da sua máquina | Depende do tipo de sessão      |

Consulte a documentação do [quickstart do terminal](/docs/pt/quickstart), [aplicativo Desktop](/docs/pt/desktop) ou [Remote Control](/docs/pt/remote-control) para configurá-los.

<h2 id="connect-github-and-create-an-environment">
  Conecte GitHub e crie um ambiente
</h2>

A configuração é um processo único. Se você já usa a CLI do GitHub, você pode [fazer isso do seu terminal](#connect-from-your-terminal) em vez do navegador.

<Steps>
  <Step title="Visite claude.ai/code">
    Vá para [claude.ai/code](https://claude.ai/code) e faça login com sua conta Anthropic.
  </Step>

  <Step title="Instale o aplicativo Claude GitHub">
    Após fazer login, claude.ai/code solicita que você conecte o GitHub. Siga o prompt para instalar o aplicativo Claude GitHub e conceder acesso aos seus repositórios. As sessões em nuvem funcionam com repositórios GitHub existentes, portanto para iniciar um novo projeto, [crie um repositório vazio no GitHub](https://github.com/new) primeiro.
  </Step>

  <Step title="Crie seu ambiente">
    Após conectar o GitHub, você será solicitado a criar um ambiente de nuvem. O ambiente controla qual acesso à rede Claude tem durante as sessões e o que é executado quando uma nova sessão é criada. Consulte [Ferramentas instaladas](/docs/pt/claude-code-on-the-web#installed-tools) para ver o que está disponível sem nenhuma configuração.

    O formulário tem estes campos:

    * **Nome**: um rótulo de exibição. Útil quando você tem múltiplos ambientes para diferentes projetos ou níveis de acesso.
    * **Acesso à rede**: controla o que a sessão pode alcançar na internet. O padrão, `Trusted`, permite conexões com [registros de pacotes comuns](/docs/pt/claude-code-on-the-web#default-allowed-domains) como npm, PyPI e RubyGems enquanto bloqueia o acesso geral à internet.
    * **Variáveis de ambiente**: variáveis opcionais disponíveis em cada sessão, em formato `.env`. Não coloque valores entre aspas, pois as aspas são armazenadas como parte do valor. Estas são visíveis para qualquer pessoa que possa editar este ambiente.
    * **Script de configuração**: um script Bash opcional que é executado antes do Claude Code ser iniciado. Use-o para instalar ferramentas do sistema que a VM de nuvem não inclui, como `apt install -y gh`. O resultado é [armazenado em cache](/docs/pt/claude-code-on-the-web#environment-caching), portanto o script não é executado novamente a cada sessão. Consulte [Scripts de configuração](/docs/pt/claude-code-on-the-web#setup-scripts) para exemplos e dicas de depuração.

    Para um primeiro projeto, deixe os padrões e clique em **Criar ambiente**. Você pode [editá-lo depois ou criar ambientes adicionais](/docs/pt/claude-code-on-the-web#configure-your-environment) para diferentes projetos.
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Conecte do seu terminal
</h3>

Se você já usa a CLI do GitHub (`gh`), você pode configurar Claude Code na web sem abrir um navegador. Isso requer a [CLI do Claude Code](/docs/pt/quickstart). `/web-setup` lê seu token `gh` local, vincula-o à sua conta Claude e cria um ambiente de nuvem padrão se você não tiver um.

<Note>
  Organizações com [Zero Data Retention](/docs/pt/zero-data-retention) habilitado não podem usar `/web-setup` ou outros recursos de sessão em nuvem. Se a CLI do GitHub não estiver instalada ou autenticada, `/web-setup` abre o fluxo de integração do navegador.
</Note>

<Steps>
  <Step title="Autentique com a CLI do GitHub">
    No seu shell, autentique a CLI do GitHub se você ainda não o fez:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Faça login no Claude">
    Na CLI do Claude Code, execute `/login` para fazer login com sua conta claude.ai. Pule esta etapa se você já estiver conectado.
  </Step>

  <Step title="Execute /web-setup">
    Na CLI do Claude Code, execute:

    ```text theme={null}
    /web-setup
    ```

    Isso sincroniza seu token `gh` com sua conta Claude. Se você ainda não tiver um ambiente de nuvem, `/web-setup` cria um com acesso à rede Trusted e sem script de configuração. Você pode [editar o ambiente ou adicionar variáveis](/docs/pt/claude-code-on-the-web#configure-your-environment) depois. Após `/web-setup` ser concluído, você pode iniciar sessões em nuvem do seu terminal com [`--cloud`](/docs/pt/claude-code-on-the-web#from-terminal-to-web) ou configurar tarefas recorrentes com [`/schedule`](/docs/pt/routines).
  </Step>
</Steps>

<h2 id="start-a-task">
  Inicie uma tarefa
</h2>

Com GitHub conectado e um ambiente criado, você está pronto para enviar tarefas.

<Steps>
  <Step title="Selecione um repositório e branch">
    De [claude.ai/code](https://claude.ai/code) ou da aba Code no aplicativo móvel Claude, clique no seletor de repositório abaixo da caixa de entrada e escolha um repositório para Claude trabalhar. Cada repositório mostra um seletor de branch. Altere-o para iniciar Claude a partir de uma branch de recurso em vez da padrão. Você pode adicionar múltiplos repositórios para trabalhar entre eles em uma sessão.
  </Step>

  <Step title="Escolha um modo de permissão">
    O dropdown de modo ao lado da entrada padrão é **Aceitar edições**, onde Claude faz alterações e envia uma branch sem parar para aprovação. Mude para **Plan** se você quiser que Claude proponha uma abordagem e aguarde seu aval antes de editar arquivos. As sessões em nuvem não oferecem permissões Manual ou Bypass. Consulte a [lista completa de modos de permissão](/docs/pt/permission-modes#available-modes) para saber o que cada um permite.
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
| `environment`  | Nome ou ID do [ambiente](#connect-github-and-create-an-environment) para pré-selecionar.                                                                                                                   |

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
  Solucione problemas de configuração
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  Nenhum repositório aparece após conectar GitHub
</h3>

Uma sessão em nuvem pode usar qualquer repositório que a conta GitHub conectada possa ver, independentemente de quais repositórios o aplicativo Claude GitHub está instalado. Se um repositório está faltando, verifique se a conta GitHub conectada tem acesso a ele no GitHub. Se você também quiser [Auto-fix](/docs/pt/claude-code-on-the-web#auto-fix-pull-requests) para um repositório, instale o App nele: em github.com, abra **Configurações → Aplicativos → Claude → Configurar** e verifique se o repositório está listado em **Acesso ao repositório**. Repositórios privados precisam da mesma autorização que os públicos.

<h3 id="the-page-only-shows-a-github-login-button">
  A página mostra apenas um botão de login do GitHub
</h3>

As sessões em nuvem requerem uma conta GitHub conectada. Conecte através do fluxo do navegador acima, ou execute `/web-setup` do seu terminal se você usar a CLI do GitHub. Se você preferir não conectar o GitHub, consulte [Remote Control](/docs/pt/remote-control) para executar Claude Code em sua própria máquina e monitorá-lo na web.

<h3 id="not-available-for-the-selected-organization">
  "Não disponível para a organização selecionada"
</h3>

Organizações Enterprise podem precisar que um administrador habilite Claude Code na web. Entre em contato com sua equipe de conta Anthropic.

<h3 id="/web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` mostra "Nenhum comando corresponde" ou "Comando desconhecido"
</h3>

`/web-setup` é executado dentro da CLI do Claude Code, não no seu shell. Inicie `claude` primeiro, depois digite `/web-setup` no prompt.

Se você digitou dentro do Claude Code e o menu de comandos mostra `Nenhum comando corresponde "/web-setup"`, ou enviá-lo retorna `Comando desconhecido: /web-setup`, o comando está oculto porque um requisito não foi atendido. A causa geralmente é que você está autenticado com uma chave de API ou provedor de terceiros em vez de uma assinatura claude.ai. Execute `claude update` para fazer login com sua conta claude.ai.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud-or-ultraplan">
  "Não foi possível criar um ambiente de nuvem" ou "Nenhum ambiente de nuvem disponível" ao usar `--cloud` ou ultraplan
</h3>

Os recursos de sessão remota criam um ambiente de nuvem padrão automaticamente se você não tiver um. Se você vir "Não foi possível criar um ambiente de nuvem", a criação automática falhou. Se você vir "Nenhum ambiente de nuvem disponível", sua CLI é anterior à criação automática. Em qualquer caso, execute `/web-setup` na CLI do Claude Code para criar um manualmente, ou visite [claude.ai/code](https://claude.ai/code) e siga a etapa **Crie seu ambiente** acima.

<h3 id="setup-script-failed">
  Script de configuração falhou
</h3>

O script de configuração saiu com um status diferente de zero, o que bloqueia o início da sessão. Causas comuns:

* Uma instalação de pacote falhou porque o registro não está no seu [nível de acesso à rede](/docs/pt/claude-code-on-the-web#access-levels). `Trusted` cobre a maioria dos gerenciadores de pacotes; `None` bloqueia todos.
* O script faz referência a um arquivo ou caminho que não existe em um clone novo.
* Um comando que funciona localmente precisa de uma invocação diferente no Ubuntu.

Para depurar, adicione `set -x` no topo do script para ver qual comando falhou. Para comandos não críticos, acrescente `|| true` para que não bloqueiem o início da sessão.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Novas sessões travam ou expiram durante a configuração
</h3>

Se novas sessões ficarem presas na etapa do script de configuração ou falharem com um erro genérico de contêiner antes do script terminar, o script provavelmente está excedendo o orçamento de tempo de aproximadamente cinco minutos para construir o [cache de ambiente](/docs/pt/claude-code-on-the-web#environment-caching). Etapas pesadas como puxar imagens Docker grandes, sincronizar árvores de dependência completas ou baixar pesos de modelo frequentemente empurram o total além do limite, especialmente quando são executadas uma após a outra.

Para corrigir isso, reduza o script para que ele termine de forma confiável em menos de cinco minutos:

* Execute instalações independentes em paralelo com `&` e um `wait` final em vez de executá-las serialmente.
* Mova os maiores downloads para fora do script de configuração e para um [hook SessionStart](/docs/pt/claude-code-on-the-web#setup-scripts-vs-sessionstart-hooks) que os inicia em segundo plano, para que a sessão se torne utilizável enquanto eles terminam.
* Remova longas tentativas de sono do script de configuração, pois um loop de tentativa travado conta contra o orçamento.

<h3 id="session-keeps-running-after-closing-the-tab">
  A sessão continua funcionando após fechar a aba
</h3>

Isso é por design. Fechar a aba ou navegar para longe não interrompe a sessão. Ela continua funcionando em segundo plano até Claude terminar a tarefa atual, depois fica ociosa. Na barra lateral, você pode [arquivar uma sessão](/docs/pt/claude-code-on-the-web#archive-sessions) para ocultá-la de sua lista, ou [deletá-la](/docs/pt/claude-code-on-the-web#delete-sessions) para removê-la permanentemente.

<h2 id="next-steps">
  Próximos passos
</h2>

Agora que você pode enviar e revisar tarefas, estas páginas cobrem o que vem a seguir: iniciar sessões em nuvem do seu terminal, agendar trabalho recorrente e dar instruções permanentes a Claude.

* [Use Claude Code na web](/docs/pt/claude-code-on-the-web): a referência completa, incluindo teletransporte de sessões para seu terminal, scripts de configuração, variáveis de ambiente e configuração de rede
* [Routines](/docs/pt/routines): automatize trabalho em um cronograma, via chamada de API ou em resposta a eventos do GitHub
* [CLAUDE.md](/docs/pt/memory): dê a Claude instruções persistentes e contexto que carregam no início de cada sessão
* Instale o aplicativo móvel Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) para monitorar sessões do seu telefone. Da CLI do Claude Code, `/mobile` mostra um código QR.
