> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Encontre bugs com ultrareview

> Execute uma revisão de código profunda e multi-agente na nuvem com /code-review ultra para encontrar e verificar bugs antes de fazer merge.

<Note>
  Ultrareview é um recurso de visualização de pesquisa. O recurso, preços e disponibilidade podem mudar com base no feedback. O comando é `/code-review ultra`. Quando ultrareview está disponível para sua conta, `/ultrareview` é um alias.
</Note>

Ultrareview é uma revisão de código profunda que é executada como uma [sessão na nuvem](/docs/pt/claude-code-on-the-web) na infraestrutura da Anthropic. Quando você executa `/code-review ultra`, Claude Code inicia uma frota de agentes revisores em um sandbox na nuvem para encontrar bugs em sua branch ou pull request.

Comparado a um `/code-review` local, ultrareview oferece:

* **Sinal mais alto**: cada descoberta relatada é reproduzida e verificada independentemente, portanto os resultados se concentram em bugs reais em vez de sugestões de estilo
* **Cobertura mais ampla**: uma frota maior de agentes revisores explora a mudança em paralelo, o que expõe problemas que uma revisão local pode perder
* **Sem uso de recursos locais**: a revisão é executada inteiramente em um sandbox na nuvem, portanto seu terminal permanece livre para outro trabalho enquanto é executada

Ultrareview requer autenticação com uma conta claude.ai porque é executado como uma sessão na nuvem na infraestrutura da Anthropic. Se você está conectado apenas com uma chave de API, execute `/login` e autentique-se com claude.ai primeiro. Ultrareview não está disponível ao usar Claude Code com Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, e não está disponível para organizações que habilitaram Zero Data Retention. Quando ultrareview não está disponível, `/code-review ultra` executa uma revisão local em sua sessão.

<h2 id="run-ultrareview-from-the-cli">
  Execute ultrareview a partir da CLI
</h2>

Inicie uma revisão de qualquer repositório git:

```text theme={null}
/code-review ultra
```

Sem argumentos, ultrareview revisa o diff entre sua branch atual e a branch padrão, incluindo mudanças não confirmadas e preparadas. Para mudanças não confirmadas em arquivos nomeados como credenciais ou chaves, como arquivos `.env` e `*.tfvars`, Claude Code segue as regras para [carregar um repositório local em uma sessão na nuvem](/docs/pt/claude-code-on-the-web#send-local-repositories-without-github).

Para uma revisão de branch, Claude Code agrupa o estado do repositório e o carrega em um sandbox remoto; quando você [revisa uma pull request](#review-a-pull-request), Claude Code não carrega nada de sua máquina.

Antes de iniciar, Claude Code mostra um diálogo de confirmação com o escopo da revisão, suas execuções gratuitas restantes e o custo estimado; para uma revisão de branch, o escopo inclui a contagem de arquivos e linhas. Depois que você confirmar, a revisão continua em segundo plano enquanto você continua usando sua sessão.

O comando é executado apenas quando você o invoca com `/code-review ultra`; Claude não inicia um ultrareview por conta própria.

<h3 id="review-against-a-different-base">
  Revisar contra uma base diferente
</h3>

Para comparar contra uma base diferente da branch padrão, passe o nome da branch. Este exemplo revisa sua branch atual contra `develop`:

```text theme={null}
/code-review ultra develop
```

A branch base não precisa existir em seu clone local; Claude Code a busca de `origin`. Se o nome tiver um erro de digitação, Claude Code sugere o nome de branch mais próximo no erro.

Um ID de commit ou tag também funciona como a base, e a revisão então cobre as mudanças em sua branch desde esse commit.

<h3 id="review-a-pull-request">
  Revisar uma pull request
</h3>

Para revisar uma pull request do GitHub em vez de uma branch local, passe o número da PR:

```text theme={null}
/code-review ultra 1234
```

O comando também aceita `#1234`, `PR 1234` e URLs de PR coladas; uma URL colada deve apontar para o repositório em seu diretório atual.

No modo PR, o sandbox remoto clona a pull request diretamente do host em vez de agrupar sua árvore de trabalho local. O modo PR funciona com repositórios em `github.com` e em instâncias do [GitHub Enterprise Server](/docs/pt/github-enterprise-server) que um proprietário conectou ao Claude Code.

Para repositórios em `github.com`, o sandbox clona com a conta do GitHub conectada à sua conta Claude, portanto a conta deve ser capaz de ler o repositório da PR. Claude Code verifica isso antes de criar a sessão na nuvem, a menos que você tenha definido [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars#variables), e recusa o lançamento quando [nenhuma conta está conectada](/docs/pt/errors#no-github-account-is-connected-to-your-claude-account) ou [a conta não consegue ver o repositório](/docs/pt/errors#your-connected-github-account-cant-see-the-repository); a recusa nomeia a correção. Antes da v2.1.248, Claude Code não verificava isso antes do lançamento.

Execute [`/web-setup`](/docs/pt/web-quickstart#connect-from-your-terminal) para conectar seu login do GitHub CLI à sua conta Claude.

<h3 id="post-findings-to-the-pull-request">
  Postar descobertas na pull request
</h3>

No Claude Code v2.1.227 ou posterior, quando você revisa uma pull request em `github.com`, você pode fazer com que Claude poste as descobertas concluídas na PR como um único comentário simples de sua própria conta GitHub. O comentário não é uma revisão ou uma aprovação, e termina com uma nota "Gerado por Claude Code". Quando você revisa uma branch ou uma pull request do GitHub Enterprise Server, Claude Code mostra as descobertas em sua sessão apenas.

Claude Code nunca posta a menos que você escolha nessa execução, e `--no-post` é o padrão. Postar é uma escolha que você faz para cada execução:

* **Interativo**: no diálogo de lançamento, selecione **Executar e postar as descobertas na PR como eu**. Se você adicionar `--post` ao comando, como em `/code-review ultra 1234 --post`, Claude Code pré-seleciona essa escolha e ainda pergunta antes de iniciar.
* **Não interativo**: execute o [subcomando `claude ultrareview`](#run-ultrareview-non-interactively) com `--post`. Você consente com a postagem ao executar o subcomando com a flag, portanto Claude Code posta sem perguntar. Em uma execução `claude -p '/code-review ultra'`, Claude Code sai antes das descobertas chegarem, portanto não posta nada; use o subcomando em vez disso.

Claude Code não posta de sua máquina. Ele envia o ID da sessão da revisão para a API Anthropic, que posta as descobertas armazenadas da revisão como o comentário através da conta GitHub que você conectou ao Claude. Postar requer o mesmo login claude.ai que a revisão em si, e não está disponível em provedores de terceiros ou quando você define [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars).

Em uma sessão interativa, Claude Code inicia a postagem quando as descobertas chegam, portanto mantenha a sessão aberta até que a revisão seja concluída. Claude Code mantém a escolha de postagem apenas nessa sessão. Se a sessão terminar antes da revisão ser concluída, Claude Code não posta nada, mesmo se você retomar a conversa mais tarde.

Quando a postagem é concluída, Claude informa o resultado:

* **Postado**: Claude fornece um link para o comentário.
* **Já postado**: uma postagem anterior da mesma revisão já colocou o comentário na PR, portanto Claude o vincula à pull request em vez de postar novamente.
* **Falhou**: Claude informa por quê, e as descobertas permanecem em seu terminal para que você possa postá-las manualmente.

<h3 id="pass-a-request-in-plain-words">
  Passar um pedido em palavras simples
</h3>

No Claude Code v2.1.218 ou posterior, você também pode descrever no que está trabalhando em palavras simples:

```text theme={null}
/code-review ultra check my auth changes
```

A revisão ainda cobre sua branch atual, o mesmo escopo que executar sem argumento. Claude mantém seu texto como uma nota, mostrada no diálogo de lançamento, e relaciona as descobertas a ele quando chegam.

Claude Code trata seu texto como uma nota apenas quando tem mais de uma palavra e não é um nome de branch ou referência de PR. Ele lê uma única palavra como um nome de branch ou referência de PR, portanto um nome de branch digitado incorretamente recebe o erro de branch mais próximo de [Revisar contra uma base diferente](#review-against-a-different-base) em vez de iniciar com uma nota. Se seu texto combinar uma referência de PR com outras palavras, como `check PR 123 again`, Claude Code também não inicia; ele pede que você execute novamente com apenas o número da PR para revisar essa PR, ou sem a referência para revisar sua branch atual.

<Tip>
  Se seu repositório for muito grande para agrupar, Claude Code o solicita a usar o modo PR. Envie sua branch e abra uma PR de rascunho, depois execute `/code-review ultra <PR-number>`.
</Tip>

<h3 id="diff-limits-and-fallbacks">
  Limites de diff e fallbacks
</h3>

Ultrareview verifica o diff antes de qualquer trabalho de revisão ser executado e informa quando não consegue revisá-lo como está:

* **Diff muito grande**: uma revisão de branch pode incluir até 500 arquivos alterados e 8.000 linhas alteradas por padrão. Os valores exatos podem mudar, e a [recusa](/docs/pt/errors#diff-is-too-large-for-ultrareview) nomeia os em vigor, o tamanho do seu diff e os arquivos com mais linhas alteradas. Claude Code recusa uma pull request muito grande da mesma forma, nomeando suas contagens de arquivo e linha, mas não o detalhamento por arquivo
* **Nada para revisar**: quando o diff contra a base está vazio, ultrareview recusa e nomeia a branch ou commit com o qual comparou e o caso em que você está, como estar na própria branch base sem nada não confirmado, ou uma branch cujos commits já fazem parte da base. Também sugere a maneira de sair para esse caso, como mudar para a branch com seu trabalho, preparar ou confirmar edições locais, ou passar uma base diferente
* **Primeiro commit**: o primeiro commit de um repositório não tem nada anterior para comparar, portanto ultrareview revisa cada arquivo nele depois que você confirma no diálogo de lançamento. Se você tiver arquivos não rastreados, ele recusa em vez disso e informa para `git add` os que você deseja revisados. Os mesmos limites de tamanho se aplicam.

  Um primeiro commit é revisado integralmente apenas após essa confirmação, portanto o subcomando `claude ultrareview` e `claude -p` o recusam e o apontam para uma sessão interativa em vez disso. Requer Claude Code v2.1.277 ou posterior
* **Sem base de mesclagem**: quando sua branch não compartilha histórico com a branch base, ou o repositório não tem branch base para comparar, ultrareview revisa cada arquivo rastreado no repositório em vez disso. O fallback requer um clone completo e aplica os mesmos limites de tamanho. Ele é lançado apenas quando você confirma no diálogo de lançamento ou executa o subcomando `claude ultrareview` você mesmo. Em `claude -p` e em qualquer outro lugar onde nenhum desses acontece, ultrareview recusa, diz que a revisão cobriria cada arquivo, e o aponta para uma sessão interativa.

  Em um checkout sem branches ou outras refs, como um HEAD desanexado criado ao fazer checkout de `FETCH_HEAD` após buscar uma URL, Claude Code [recusa a revisão](/docs/pt/errors#your-checkout-has-no-branches) e sugere criar uma branch primeiro

<h2 id="pricing-and-free-runs">
  Preços e execuções gratuitas
</h2>

Ultrareview é um recurso premium que é cobrado contra créditos de uso em vez do uso incluído em seu plano.

| Plano             | Execuções gratuitas incluídas | Após execuções gratuitas                                                                                          |
| ----------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Pro               | 3 execuções gratuitas         | cobrado como [créditos de uso](https://support.claude.com/pt/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max               | 3 execuções gratuitas         | cobrado como [créditos de uso](https://support.claude.com/pt/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team e Enterprise | nenhuma                       | cobrado como [créditos de uso](https://support.claude.com/pt/articles/12429409-extra-usage-for-paid-claude-plans) |

* **Execuções gratuitas**: as três execuções Pro e Max são uma alocação única por conta e não são renovadas.
* **Custo por revisão**: após usar as execuções gratuitas, normalmente \$5 a \$25 em créditos de uso dependendo do tamanho da mudança, correspondendo à estimativa que o diálogo de lançamento mostra antes de cada execução.
* **Quando uma execução é contada**: assim que a sessão na nuvem é iniciada. Uma revisão que você interrompe no início ou que falha em ser concluída ainda usa uma execução gratuita; uma revisão paga é cobrada apenas pela parte que foi executada.

Como ultrareview sempre é cobrado como créditos de uso fora das execuções gratuitas, sua conta ou organização deve ter créditos de uso habilitados antes de poder iniciar uma revisão paga. Se os créditos de uso não estiverem habilitados, Claude Code bloqueia o lançamento, e como você os ativa depende do seu acesso de faturamento:

* Se você puder gerenciar o faturamento da sua conta, Claude Code o vincula às configurações de faturamento onde você pode ativar os créditos de uso.
* Nos planos Team e Enterprise, membros sem acesso de faturamento enviam uma solicitação da CLI pedindo ao seu administrador para ativar os créditos de uso.

Você também pode executar `/usage-credits` para verificar ou alterar sua configuração de créditos de uso.

Claude Code pede que você confirme o faturamento de créditos de uso uma vez por conversa: quando você inicia uma nova conversa, por exemplo com `/clear`, Claude Code mostra a confirmação novamente para a próxima revisão paga.

<h2 id="track-a-running-review">
  Acompanhe uma revisão em execução
</h2>

Uma revisão normalmente leva 5 a 10 minutos. A revisão é executada como uma tarefa em segundo plano, portanto você pode continuar trabalhando em sua sessão, iniciar outros comandos ou fechar o terminal completamente. Se você escolheu [postar as descobertas para a solicitação de pull](#post-findings-to-the-pull-request), mantenha a sessão aberta até que a revisão termine; se a sessão terminar primeiro, Claude Code não publica nada.

Use `/tasks` para ver revisões em execução e concluídas, abrir a visualização de detalhes de uma revisão ou parar uma revisão em andamento. Se você parar uma revisão, Claude Code arquiva a sessão na nuvem e não retorna descobertas parciais.

Claude também pode informar que uma revisão foi interrompida ou que sua sessão não foi encontrada:

* Se a sessão na nuvem da revisão for interrompida ou [arquivada](/docs/pt/claude-code-on-the-web#archive-sessions) no claude.ai antes da revisão terminar, Claude informa que foi interrompida.
* Se a sessão na nuvem da revisão foi deletada, ou você se conectou a uma conta Claude ou organização diferente desde que a iniciou, Claude informa que a sessão não foi encontrada.
* Se você trocou de conta, a revisão ainda pode terminar sob a conta que a iniciou. Se a revisão ainda estiver em execução, conecte-se novamente como essa conta e retome a conversa com `claude --resume` para revinculá-la.

Quando a revisão termina, Claude Code mostra as descobertas verificadas como uma notificação em sua sessão. Cada descoberta inclui a localização do arquivo e uma explicação do problema para que você possa pedir ao Claude para corrigi-lo diretamente.

<h2 id="run-ultrareview-non-interactively">
  Execute ultrareview de forma não interativa
</h2>

Use o subcomando `claude ultrareview` para iniciar um ultrareview a partir de CI ou um script sem uma sessão interativa. O subcomando inicia a mesma revisão que `/code-review ultra`, bloqueia até que a revisão remota termine e imprime as descobertas para stdout.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Sem argumentos, o subcomando revisa o diff entre sua branch atual e a branch padrão, com o mesmo [fallback de repositório inteiro](#diff-limits-and-fallbacks) que `/code-review ultra` quando não existe base de mesclagem. Passe um número de PR para revisar uma pull request, ou uma branch base para revisar em relação a ela; o [tratamento de branch base](#review-against-a-different-base) corresponde ao comando interativo.

Você consente com o fallback de repositório inteiro e com o aviso de faturamento e termos quando executa o subcomando, portanto a execução começa sem aguardar entrada. Executar você mesmo é o que conta como consentimento. Quando Claude executa o subcomando para você, por exemplo através da ferramenta Bash, Claude Code recusa a revisão de repositório inteiro.

No Claude Code v2.1.218 ou posterior, você também pode iniciar a revisão na nuvem executando `/code-review ultra` em uma sessão não interativa, por exemplo `claude -p '/code-review ultra'`. Claude Code inicia a revisão e imprime um link de rastreamento sem aguardar as descobertas, diferentemente de `claude ultrareview`, que bloqueia até que elas cheguem. Quando a revisão faturaria créditos de uso, Claude Code para antes de iniciar e aponta você para `claude ultrareview`, porque a confirmação de faturamento precisa de uma sessão interativa. Antes da v2.1.218, `/code-review ultra` em uma sessão não interativa executava uma revisão local.

As mensagens de progresso e a URL da sessão ao vivo vão para stderr para que stdout permaneça analisável. Use esses sinalizadores para controlar a saída, o tempo limite e se deve postar as descobertas:

| Sinalizador           | Descrição                                                                                                                                                                                                                                                                                           |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--json`              | Imprima a carga útil bruta `bugs.json` em vez das descobertas formatadas                                                                                                                                                                                                                            |
| `--timeout <minutes>` | Minutos máximos para aguardar a conclusão da revisão. Padrão é 45                                                                                                                                                                                                                                   |
| `--post`              | [Poste as descobertas terminadas](#post-findings-to-the-pull-request) para a pull request como um comentário simples de sua conta GitHub. Funciona em destinos de pull request `github.com`; em outros destinos, Claude Code ignora o sinalizador e avisa. Requer Claude Code v2.1.227 ou posterior |
| `--no-post`           | Não poste as descobertas. Este é o padrão, e se você passar ambos os sinalizadores, Claude Code não posta. Requer Claude Code v2.1.227 ou posterior                                                                                                                                                 |

Executar `claude ultrareview` requer a mesma autenticação e configuração de créditos de uso que `/code-review ultra`.

O subcomando sai com um dos três códigos:

* **0**: a revisão foi concluída, com ou sem descobertas
* **1**: a revisão falhou ao iniciar ou foi interrompida antes de terminar, a sessão na nuvem apresentou erro ou o tempo limite decorrido
* **130**: você interrompeu o subcomando com Ctrl-C

Se você interromper o subcomando, a revisão remota continua em execução; siga a URL da sessão impressa em stderr para observá-la no navegador.

Com `--post`, o subcomando inicia a postagem logo após imprimir as descobertas, e imprime o link para stderr.

* Se a execução falhar, for interrompida ou expirar o tempo limite, ou se você interrompê-la, o subcomando não posta nada.
* Se a revisão for concluída mas o comentário não for postado, Claude Code imprime o motivo para stderr, e as descobertas permanecem em stdout para que você possa postá-las manualmente.

Para revisões automáticas em pull requests do GitHub, [Code Review](/docs/pt/code-review) integra-se diretamente com seu repositório e publica descobertas como comentários inline de PR sem uma etapa de CLI.

<h2 id="how-ultrareview-compares-to-/code-review">
  Como ultrareview se compara a /code-review
</h2>

Ambas as revisões examinam código, mas você as usa em diferentes estágios do seu fluxo de trabalho.

|              | `/code-review`                                                  | `/code-review ultra`                                                                    |
| ------------ | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| Alvo         | seu diff de trabalho, um pull request, uma branch ou um caminho | seu diff de trabalho ou um pull request                                                 |
| Execuções    | localmente em sua sessão                                        | em um sandbox na nuvem                                                                  |
| Profundidade | escala com o argumento de esforço                               | frota multi-agente com verificação independente                                         |
| Duração      | segundos a alguns minutos                                       | aproximadamente 5 a 10 minutos                                                          |
| Custo        | conta para uso normal                                           | execuções gratuitas, depois aproximadamente \$5 a \$25 por revisão como créditos de uso |
| Melhor para  | feedback rápido durante iteração                                | confiança pré-merge em mudanças substanciais                                            |

Use `/code-review` para feedback rápido enquanto trabalha, ou passe um número de PR para revisar um pull request de um colega antes de aprová-lo. Use `/code-review ultra` antes de fazer merge de uma mudança substancial quando você quer uma passagem mais profunda que capture problemas que uma revisão local pode perder.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Use Claude Code na nuvem](/docs/pt/claude-code-on-the-web): aprenda como funcionam as sessões na nuvem e os sandboxes na nuvem
* [Gerencie custos efetivamente](/docs/pt/costs): acompanhe o uso e defina limites de gastos
