> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Configure análises automatizadas de PR que detectam erros de lógica, vulnerabilidades de segurança e regressões usando análise multi-agente de sua base de código completa

<Note>
  Code Review está em visualização de pesquisa, disponível para assinaturas [Team e Enterprise](https://claude.ai/admin-settings/claude-code). Não está disponível para organizações com [Zero Data Retention](/docs/pt/zero-data-retention) ativado. Em outros planos, você ainda pode [revisar um diff localmente](#review-a-diff-locally) com o comando `/code-review`.
</Note>

Code Review analisa seus pull requests do GitHub e publica descobertas como comentários inline nas linhas de código onde encontrou problemas. Uma frota de agentes especializados examina as alterações de código no contexto de sua base de código completa, procurando por erros de lógica, vulnerabilidades de segurança, casos extremos quebrados e regressões sutis.

As descobertas são marcadas por severidade e não aprovam ou bloqueiam seu PR, portanto os fluxos de trabalho de revisão existentes permanecem intactos. Você pode ajustar o que Claude sinaliza adicionando um arquivo `CLAUDE.md` ou `REVIEW.md` ao seu repositório.

Para executar Claude em sua própria infraestrutura de CI em vez deste serviço gerenciado, consulte [GitHub Actions](/docs/pt/github-actions) ou [GitLab CI/CD](/docs/pt/gitlab-ci-cd). Para repositórios em uma instância GitHub auto-hospedada, consulte [GitHub Enterprise Server](/docs/pt/github-enterprise-server).

Esta página cobre:

* [Como as revisões funcionam](#how-reviews-work)
* [Configuração](#set-up-code-review)
* [Acionando revisões manualmente](#manually-trigger-reviews) com `@claude review` e `@claude review always`
* [Personalizando revisões](#customize-reviews) com `CLAUDE.md` e `REVIEW.md`
* [Preços](#pricing)
* [Troubleshooting](#troubleshooting) execuções falhadas e comentários ausentes
* [Revisando um diff localmente](#review-a-diff-locally) com o comando `/code-review`

<h2 id="how-reviews-work">
  Como as revisões funcionam
</h2>

Depois que um administrador [ativa Code Review](#set-up-code-review) para sua organização, as revisões são acionadas quando um PR é aberto, em cada push ou quando solicitado manualmente, dependendo do comportamento configurado do repositório. Comentar `@claude review` [inicia uma revisão em um PR](#manually-trigger-reviews) em qualquer modo.

Quando uma revisão é executada, vários agentes analisam o diff e o código circundante em paralelo na infraestrutura da Anthropic. Cada agente procura por uma classe diferente de problema, então uma etapa de verificação verifica os candidatos contra o comportamento real do código para filtrar falsos positivos. Os resultados são desduplicados, classificados por severidade e publicados como comentários inline nas linhas específicas onde os problemas foram encontrados, com um resumo no corpo da revisão. Se nenhum problema for encontrado, Code Review atualiza a execução de verificação do GitHub para mostrar que nenhum problema foi detectado. Claude também pode publicar um breve comentário de confirmação no PR.

As revisões escalam em custo com o tamanho e complexidade do PR, completando em média em 20 minutos. Os administradores podem monitorar a atividade de revisão e gastos através do [painel de análise](#view-usage).

<h3 id="severity-levels">
  Níveis de severidade
</h3>

Cada descoberta é marcada com um nível de severidade:

| Marcador | Severidade    | Significado                                                             |
| :------- | :------------ | :---------------------------------------------------------------------- |
| 🔴       | Importante    | Um bug que deve ser corrigido antes de fazer merge                      |
| 🟡       | Nit           | Um problema menor, vale a pena corrigir mas não é bloqueante            |
| 🟣       | Pré-existente | Um bug que existe na base de código mas não foi introduzido por este PR |

As descobertas incluem uma seção de raciocínio estendido recolhível que você pode expandir para entender por que Claude sinalizou o problema e como verificou o problema.

<h3 id="rate-and-reply-to-findings">
  Avaliar e responder a descobertas
</h3>

Cada comentário de revisão do Claude chega com 👍 e 👎 já anexados para que ambos os botões apareçam na interface do GitHub para classificação com um clique. Clique em 👍 se a descoberta foi útil ou 👎 se estava errada ou ruidosa. A Anthropic coleta contagens de reações após o PR ser mesclado e as usa para ajustar o revisor. As reações não acionam uma re-revisão ou alteram nada no PR.

Responder a um comentário inline não solicita que Claude responda ou atualize o PR. Para agir em uma descoberta, corrija o código e faça push. Se o PR estiver inscrito em revisões acionadas por push, a próxima execução resolve a thread quando o problema for corrigido. Para solicitar uma revisão nova sem fazer push, comente `@claude review` como um [comentário de PR de nível superior](#manually-trigger-reviews).

Para descartar uma descoberta sem uma alteração de código, resolva sua thread; responder não a descarta.

<h3 id="check-run-output">
  Saída de execução de verificação
</h3>

Além dos comentários de revisão inline, cada revisão popula a execução de verificação **Claude Code Review** que aparece junto com suas verificações de CI. Expanda seu link **Details** para ver um resumo de cada descoberta em um único lugar, classificado por severidade:

| Severidade    | Arquivo:Linha             | Problema                                                                 |
| ------------- | ------------------------- | ------------------------------------------------------------------------ |
| 🔴 Importante | `src/auth/session.ts:142` | Atualização de token corre com logout, deixando sessões obsoletas ativas |
| 🟡 Nit        | `src/auth/session.ts:88`  | `parseExpiry` retorna silenciosamente 0 em entrada malformada            |

Cada descoberta também aparece como uma anotação na aba **Files changed**, marcada diretamente nas linhas de diff relevantes. As descobertas Importantes são renderizadas com um marcador vermelho, nits com um aviso amarelo e bugs pré-existentes com um aviso cinza. Anotações e a tabela de severidade são escritas na execução de verificação independentemente dos comentários de revisão inline, portanto permanecem disponíveis mesmo se GitHub rejeitar um comentário inline em uma linha que se moveu.

A execução de verificação sempre é concluída com uma conclusão neutra, portanto nunca bloqueia a mesclagem através de regras de proteção de branch. Se você deseja bloquear mesclagens em descobertas de Code Review, leia o detalhamento de severidade da saída de execução de verificação em seu próprio CI. A última linha do texto Details é um comentário legível por máquina que seu fluxo de trabalho pode analisar com `gh` e jq. Para encontrar a ID de execução de verificação, liste as execuções de verificação do commit com `gh api repos/OWNER/REPO/commits/<commit-sha>/check-runs --jq '.check_runs[] | {id, name}'` e pegue a `id` da execução `Claude Code Review`. Substitua `OWNER`, `REPO` e `CHECK_RUN_ID` pelo proprietário do seu repositório, nome do repositório e essa ID:

```bash theme={null}
gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID \
  --jq '.output.text | split("bughunter-severity: ")[1] | split(" -->")[0] | fromjson'
```

Isso retorna um objeto JSON com contagens por severidade, por exemplo `{"normal": 2, "nit": 1, "pre_existing": 0}`. A chave `normal` contém a contagem de descobertas Importantes; um valor diferente de zero significa que Claude encontrou pelo menos um bug que vale a pena corrigir antes da mesclagem.

<h3 id="what-code-review-checks">
  O que Code Review verifica
</h3>

Por padrão, Code Review se concentra em correção: bugs que quebrariam a produção, não preferências de formatação ou cobertura de testes ausente. Você pode expandir o que verifica [adicionando arquivos de orientação](#customize-reviews) ao seu repositório.

<h2 id="set-up-code-review">
  Configurar Code Review
</h2>

Um Owner ativa Code Review uma vez para a organização e seleciona quais repositórios incluir.

<Steps>
  <Step title="Abrir configurações de administrador do Claude Code">
    Vá para [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) e encontre a seção Code Review. Você precisa da função Owner ou Primary Owner em sua organização Claude e permissão para instalar GitHub Apps em sua organização GitHub.
  </Step>

  <Step title="Iniciar configuração">
    Clique em **Setup**. Isso inicia o fluxo de instalação do GitHub App.
  </Step>

  <Step title="Instalar o Claude GitHub App">
    Siga os prompts para instalar o Claude GitHub App: escolha a organização GitHub que possui os repositórios que você deseja revisar, escolha quais repositórios o app pode acessar e aprove as permissões solicitadas.

    Para revisar um pull request, Claude lê o conteúdo do seu repositório através do acesso de leitura do app e publica comentários e a [execução de verificação](#check-run-output) através de seu acesso de escrita a pull requests e verificações. Durante a instalação, você concede um conjunto de permissões mais amplo compartilhado por outros recursos Claude, como [GitHub Actions](/docs/pt/github-actions); consulte [Permissões do GitHub App](/docs/pt/github-actions#github-app-permissions) para a lista completa.
  </Step>

  <Step title="Selecionar repositórios">
    Escolha quais repositórios ativar para Code Review. Se você não vir um repositório, certifique-se de que deu ao Claude GitHub App acesso a ele durante a instalação. Você pode adicionar mais repositórios mais tarde.
  </Step>

  <Step title="Definir gatilhos de revisão por repo">
    Após a conclusão da configuração, a seção Code Review mostra seus repositórios em uma tabela. Para cada repositório, use o dropdown **Review Behavior** para escolher quando as revisões são executadas:

    * **Once after PR creation**: a revisão é executada uma vez quando um PR é aberto ou marcado como pronto para revisão
    * **After every push**: a revisão é executada em cada push para o branch do PR, detectando novos problemas conforme o PR evolui e resolvendo automaticamente threads quando você corrige problemas sinalizados
    * **Manual**: abrir ou fazer push para um PR não inicia uma revisão; comente [`@claude review`](#manually-trigger-reviews) para solicitar uma, ou `@claude review always` para também inscrever o PR em revisões em pushes subsequentes

    Qualquer que seja a opção escolhida, Claude revisa um [pull request de um fork](#review-pull-requests-from-forks) apenas quando alguém comenta `@claude review` nele.

    Revisar em cada push executa a maioria das revisões e custa mais. O modo Manual é útil para repositórios de alto tráfego onde você deseja optar PRs específicos para revisão, ou para começar a revisar seus PRs apenas quando estiverem prontos.
  </Step>
</Steps>

A tabela de repositórios também mostra o custo médio por revisão para cada repo com base na atividade recente. Use o menu de ações de linha para ativar ou desativar Code Review por repositório, ou para remover um repositório completamente.

Para verificar a configuração, abra um PR de teste. Se você escolheu um gatilho automático, uma execução de verificação chamada **Claude Code Review** aparece em alguns minutos. Se você escolheu Manual, comente `@claude review` no PR para iniciar a primeira revisão. Se nenhuma execução de verificação aparecer, confirme que o repositório está listado em suas configurações de administrador e que o Claude GitHub App tem acesso a ele.

<h2 id="manually-trigger-reviews">
  Acionando revisões manualmente
</h2>

Comandos de comentário iniciam uma revisão sob demanda. Eles funcionam independentemente do gatilho configurado do repositório, portanto você pode usá-los para optar PRs específicos para revisão no modo Manual ou para obter uma re-revisão imediata em outros modos.

| Comando                 | O que faz                                                                           |
| :---------------------- | :---------------------------------------------------------------------------------- |
| `@claude review`        | Inicia uma única revisão sem inscrever o PR em pushes futuros                       |
| `@claude review always` | Inicia uma revisão e inscreve o PR em revisões acionadas por push a partir de então |
| `@claude review once`   | O mesmo que `@claude review`: inicia uma única revisão sem inscrever                |

Use `@claude review always` quando você deseja que cada push subsequente para o PR inicie uma revisão atualizada, como em um PR de alta prioridade em um repositório definido para modo Manual. Como o comando simples não inscreve o PR, você pode solicitar uma segunda opinião única sem alterar se pushes posteriores acionam revisões.

<Note>
  Antes de uma atualização de julho de 2026, `@claude review` inscrevia o PR em revisões acionadas por push. Se você dependia desse comportamento, comente `@claude review always` em vez disso. `@claude review once` ainda funciona e se comporta da mesma forma que o comando simples.
</Note>

Para qualquer um desses comandos acionar uma revisão:

* Poste-o como um comentário de PR de nível superior, não um comentário inline em uma linha de diff
* Coloque o comando no início do comentário, com `once` ou `always` na mesma linha que o resto do comando
* Você deve ter permissão de escrita, manutenção ou administrador no repositório
* O PR deve estar aberto

Se o repositório pertencer a uma organização e sua associação nessa organização for privada, que é o padrão do GitHub, o GitHub não o identifica para Claude como um membro. Claude ainda pode reagir ao seu comentário com 👀, mas não inicia uma revisão a menos que você tenha sido adicionado ao repositório diretamente como colaborador, mesmo quando uma equipe ou as permissões base da organização lhe dão acesso de escrita. Para corrigir isso, [torne sua associação à organização pública](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-your-membership-in-organizations/publicizing-or-hiding-organization-membership) ou peça a um administrador do repositório para adicioná-lo ao repositório como colaborador.

Diferentemente dos gatilhos automáticos, os gatilhos manuais são executados em PRs de rascunho, já que uma solicitação explícita sinaliza que você deseja a revisão agora independentemente do status de rascunho.

Se uma revisão já estiver em execução nesse PR, a solicitação é enfileirada até que a revisão em andamento seja concluída. Você pode monitorar o progresso através da execução de verificação no PR.

<h3 id="review-pull-requests-from-forks">
  Revisar pull requests de forks
</h3>

Claude não revisa um pull request de um fork automaticamente, independentemente da configuração **Review Behavior** do repositório. Para iniciar um, comente `@claude review` no pull request. Os [requisitos para comandos de comentário](#manually-trigger-reviews) ainda se aplicam, e o acesso de escrita que você precisa é para o repositório base, não para o fork.

Para obter outra revisão de um pull request de fork, poste um novo comentário `@claude review`. `@claude review always` também funciona, mas não inscreve o pull request em revisões em pushes posteriores. Nada além de um comando de comentário inicia uma revisão em um pull request de fork:

* Clicar em **Re-run** na execução de verificação não inicia uma revisão
* Fazer push de novos commits não inicia uma revisão, mesmo em um repositório definido para **After every push**

<h2 id="customize-reviews">
  Personalizar revisões
</h2>

Code Review lê dois arquivos do seu repositório para orientar o que sinaliza. Eles diferem em como influenciam fortemente a revisão:

* **`CLAUDE.md`**: instruções de projeto compartilhadas que Claude Code usa para todas as tarefas, não apenas revisões. Code Review o lê como contexto de projeto e sinaliza violações recém-introduzidas como nits.
* **`REVIEW.md`**: instruções exclusivas de revisão, fornecidas aos agentes que encontram e verificam descobertas e consultadas pelos agentes que classificam e relatam descobertas. Use-o para dizer o que sua equipe quer sinalizado, em qual severidade e como as descobertas são relatadas.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review lê seus arquivos `CLAUDE.md` do repositório e trata violações recém-introduzidas como descobertas de [nível nit](#severity-levels). Isso funciona bidirecionalmente: se seu PR altera o código de uma forma que torna uma declaração `CLAUDE.md` desatualizada, Claude sinaliza que os docs precisam ser atualizados também.

Claude lê arquivos `CLAUDE.md` em cada nível de sua hierarquia de diretórios, portanto as regras no `CLAUDE.md` de um subdiretório se aplicam apenas aos arquivos sob esse caminho. Consulte a [documentação de memory](/docs/pt/memory) para mais informações sobre como `CLAUDE.md` funciona.

Para orientação específica de revisão que você não deseja aplicada a sessões gerais do Claude Code, use [`REVIEW.md`](#review-md) em vez disso.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` é um arquivo na raiz do seu repositório que personaliza Code Review para seu repo. Os agentes no pipeline de revisão que encontram e verificam descobertas recebem seu conteúdo como instruções de revisão do seu repositório, ao lado da orientação de revisão padrão do Code Review, e os agentes que classificam e relatam descobertas o consultam antes de definir severidade e escrever a revisão.

Coloque as regras que você deseja aplicadas diretamente em `REVIEW.md`.

<h4 id="what-you-can-tune">
  O que você pode ajustar
</h4>

`REVIEW.md` é markdown de forma livre, portanto qualquer coisa que você possa expressar como uma instrução de revisão está no escopo. Os padrões abaixo têm o maior impacto na prática.

**Severidade**: redefina o que 🔴 Importante significa para seu repo. A calibração padrão visa código de produção; um repo de docs, um repo de config ou um protótipo pode querer uma definição muito mais estreita. Declare explicitamente quais classes de descoberta são Importantes e quais são Nit no máximo. Você também pode escalar na outra direção, por exemplo tratando qualquer violação de `CLAUDE.md` como Importante em vez do nit padrão.

**Volume de nit**: limite quantos comentários 🟡 Nit uma única revisão publica. Prosa e arquivos de config podem ser polidos para sempre. Um limite como "relatar no máximo cinco nits, mencionar o resto como uma contagem no resumo" mantém as revisões acionáveis.

**Regras de pulo**: liste caminhos, padrões de branch e categorias de descoberta onde Claude não deve publicar descobertas. Candidatos comuns são código gerado, lockfiles, dependências vendidas e branches de autoria de máquina, junto com qualquer coisa que seu CI já aplique como linting ou verificação ortográfica. Para caminhos que justificam alguma revisão mas não escrutínio completo, defina uma barra mais alta em vez de pular completamente: "em `scripts/`, relatar apenas se próximo de certo e severo."

**Verificações específicas do repo**: adicione regras que você deseja sinalizadas em cada PR, como "novas rotas de API devem ter um teste de integração." Como `REVIEW.md` alcança cada agente de descoberta e verificação diretamente, essas chegam mais confiávelmente do que as mesmas regras em um `CLAUDE.md` longo.

**Barra de verificação**: exija evidência antes de uma classe de descoberta ser publicada. Por exemplo, "reivindicações de comportamento precisam de uma citação `file:line` na fonte, não uma inferência de nomenclatura" reduz falsos positivos que de outra forma custariam ao autor uma volta.

**Convergência de re-revisão**: diga a Claude como se comportar quando um PR já foi revisado. Uma regra como "após a primeira revisão, suprima nits novos e publique descobertas Importantes apenas" impede que uma correção de uma linha chegue à sétima rodada apenas por estilo.

**Forma de resumo**: peça para o corpo da revisão abrir com uma contagem de uma linha como `2 factual, 4 style`, e liderar com "sem problemas factuais" quando esse for o caso. O autor quer saber a forma do trabalho antes dos detalhes.

<h4 id="example">
  Exemplo
</h4>

Este `REVIEW.md` recalibra severidade para um serviço backend, limita nits, pula arquivos gerados e adiciona verificações específicas do repo.

```markdown theme={null}
# Instruções de revisão

## O que Importante significa aqui

Reserve Importante para descobertas que quebrariam comportamento, vazariam dados
ou bloqueariam um rollback: lógica incorreta, consultas de banco de dados sem escopo, PII
em logs ou mensagens de erro, e migrações que não são compatíveis com versões anteriores. Estilo, nomenclatura e sugestões de refatoração são Nit no máximo.

## Limite os nits

Relatar no máximo cinco Nits por revisão. Se você encontrou mais, diga "mais N
itens similares" no resumo em vez de publicá-los inline. Se tudo que você encontrou é um Nit, lidere o resumo com "Sem problemas bloqueantes."

## Não relatar

- Qualquer coisa que CI já aplique: lint, formatação, erros de tipo
- Arquivos gerados sob `src/gen/` e qualquer arquivo `*.lock`
- Código apenas de teste que intencionalmente viola regras de produção

## Sempre verificar

- Novas rotas de API têm um teste de integração
- Linhas de log não incluem endereços de email, IDs de usuário ou corpos de solicitação
- Consultas de banco de dados estão no escopo do chamador do tenant
```

<h4 id="keep-it-focused">
  Mantenha-o focado
</h4>

O comprimento tem um custo: um `REVIEW.md` longo dilui as regras que mais importam. Mantenha-o em instruções que alteram o comportamento de revisão e deixe contexto geral do projeto em `CLAUDE.md`.

<h2 id="view-usage">
  Ver uso
</h2>

Vá para [claude.ai/analytics/code-review](https://claude.ai/analytics/code-review) para ver a atividade de Code Review em toda sua organização. O painel mostra:

| Seção                | O que mostra                                                                                                       |
| :------------------- | :----------------------------------------------------------------------------------------------------------------- |
| PRs reviewed         | Contagem diária de pull requests revisados durante o intervalo de tempo selecionado                                |
| Cost weekly          | Gasto semanal em Code Review                                                                                       |
| Feedback             | Contagem de comentários de revisão que foram resolvidos automaticamente porque um desenvolvedor abordou o problema |
| Repository breakdown | Contagens por repo de PRs revisados e comentários resolvidos                                                       |

Os números de custo do painel são estimativas para monitorar atividade. Para gasto preciso de fatura, consulte sua fatura da Anthropic.

<h2 id="pricing">
  Preços
</h2>

Code Review é faturado com base no uso de tokens. Cada revisão custa em média \$15-25, escalando com o tamanho do PR, complexidade da base de código e quantos problemas requerem verificação. O uso de Code Review é faturado separadamente através de [créditos de uso](https://support.claude.com/pt/articles/12429409-extra-usage-for-paid-claude-plans) e não conta contra o uso incluído do seu plano.

O gatilho de revisão que você escolhe afeta o custo total:

* **Once after PR creation**: é executado uma vez por PR
* **After every push**: é executado em cada push, multiplicando o custo pelo número de pushes
* **Manual**: sem revisões em aberto ou push, portanto o custo acumula apenas de revisões que alguém solicita

Em modo Once after PR creation ou Manual, comentar `@claude review always` [opta o PR em revisões acionadas por push](#manually-trigger-reviews), portanto custo adicional acumula por push após esse comentário. Em modo After every push, pushes já acionam revisões, portanto a assinatura não muda o custo por push. Comentar `@claude review` executa uma única revisão sem se inscrever em pushes futuros. Claude revisa um [pull request de um fork](#review-pull-requests-from-forks) apenas quando alguém comenta `@claude review`, portanto um pull request de fork nunca acumula custo por push em nenhum modo.

Os custos aparecem em sua fatura da Anthropic independentemente de sua organização usar Amazon Bedrock ou Google Cloud's Agent Platform para outros recursos do Claude Code. Para definir um limite de gasto mensal para Code Review, vá para [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) e configure o limite para o serviço Claude Code Review.

Monitore gastos através do gráfico de custo semanal em [analytics](#view-usage) ou da coluna de custo médio por repo nas configurações de administrador.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

As execuções de revisão são do melhor esforço. Uma execução falhada nunca bloqueia seu PR, mas também não tenta novamente por conta própria. Esta seção cobre como se recuperar de uma execução falhada e onde procurar quando a execução de verificação relata problemas que você não consegue encontrar.

<h3 id="retrigger-a-failed-or-timed-out-review">
  Retrigger uma revisão falhada ou com tempo limite excedido
</h3>

Quando a infraestrutura de revisão atinge um erro interno ou excede seu limite de tempo, a execução de verificação é concluída com um título de **Code review encountered an error** ou **Code review timed out**. A conclusão ainda é neutra, portanto nada bloqueia sua mesclagem, mas nenhuma descoberta é publicada.

Para executar a revisão novamente, comente `@claude review` no PR. Isso inicia uma revisão nova sem inscrever o PR em pushes futuros. Se o PR não for [de um fork](#review-pull-requests-from-forks), você pode clicar em **Re-run** na verificação **Claude Code Review** na aba Checks do GitHub. Uma re-execução também inicia uma revisão nova sem inscrever o PR.

<h3 id="review-didn’t-run-and-the-pr-shows-a-spend-cap-message">
  Revisão não foi executada e o PR mostra uma mensagem de limite de gasto
</h3>

Quando o limite de gasto mensal de sua organização é atingido, Code Review publica um único comentário no PR explicando que a revisão foi ignorada. As revisões retomam automaticamente no início do próximo período de faturamento, ou imediatamente quando um administrador aumenta o limite em [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

<h3 id="find-issues-that-aren’t-showing-as-inline-comments">
  Encontrar problemas que não aparecem como comentários inline
</h3>

Se o título da execução de verificação disser que problemas foram encontrados mas você não vir comentários de revisão inline no diff, procure nestes outros locais onde as descobertas são exibidas:

* **Check run Details**: clique em **Details** ao lado da verificação Claude Code Review na aba Checks. A tabela de severidade lista cada descoberta com seu arquivo, linha e resumo independentemente de o comentário inline ter sido aceito.
* **Files changed annotations**: abra a aba **Files changed** no PR. As descobertas são renderizadas como anotações anexadas diretamente às linhas de diff, separadas dos comentários de revisão.
* **Review body**: se você fez push para o PR enquanto uma revisão estava em execução, algumas descobertas podem fazer referência a linhas que não existem mais no diff atual. Essas aparecem sob um cabeçalho **Additional findings** no texto do corpo da revisão em vez de como comentários inline.

<h2 id="review-a-diff-locally">
  Revisar um diff localmente
</h2>

O comando [`/code-review`](/docs/pt/commands) revisa um diff em seu terminal sem instalar o GitHub App. Ele relata bugs de correção e reutilização, simplificação e limpezas de eficiência.

`/review` é um alias de `/code-review`; antes da v2.1.223, era um comando separado que executava uma revisão de uma única passagem, somente leitura, de um pull request do GitHub.

<Steps>
  <Step title="Executar /code-review">
    Na sessão onde você está trabalhando, execute o comando:

    ```text theme={null}
    /code-review
    ```

    Ele revisa os commits de sua branch à frente de sua upstream mais quaisquer alterações não confirmadas, portanto precisa de trabalho na branch ou na árvore de trabalho para ter algo a relatar. Para revisar algo diferente, passe um alvo: um caminho de arquivo, um número de PR, um nome de branch ou um intervalo de ref como `main...my-feature`.

    Você também pode adicionar sinalizadores:

    * `--fix`: aplica as descobertas à sua árvore de trabalho após a revisão
    * `--comment`: publica as descobertas em um pull request do GitHub como comentários inline, ou em um merge request do GitLab como uma única nota
    * `--post`: em uma revisão `ultra` na nuvem de um pull request `github.com`, pré-seleciona a publicação das descobertas concluídas para o PR no diálogo de inicialização; veja [Publicar descobertas no pull request](/docs/pt/ultrareview#post-findings-to-the-pull-request). Requer Claude Code v2.1.227 ou posterior

    Quando você passa `--comment` para um merge request do GitLab, Claude Code publica as descobertas através do CLI `glab` do GitLab. Requer Claude Code v2.1.257 ou posterior. Quando `glab` não está instalado, Claude imprime as descobertas no terminal.

    Passe o merge request como sua URL ou uma referência `!123`. Claude Code trata um número simples ou nome de branch como um merge request apenas quando a origem do checkout está em `gitlab.com`. Em uma instância GitLab auto-gerenciada, passe a URL ou forma `!123`.
  </Step>

  <Step title="Continue trabalhando">
    A revisão é executada como um [subagent](/docs/pt/sub-agents) em segundo plano com sua própria janela de contexto, portanto não preenche sua conversa. As descobertas chegam em sua conversa quando a revisão é concluída.
  </Step>

  <Step title="Agir sobre as descobertas">
    Peça ao Claude para corrigir o que a revisão encontrou. Se você passou `--fix` ou `--comment`, a revisão já aplicou ou publicou suas descobertas.
  </Step>
</Steps>

Claude relata as descobertas como texto na resposta em ambas essas execuções, mesmo quando um aplicativo host solicita uma lista de descobertas:

* Em uma sessão de terminal, onde `/code-review` executa a revisão como um [subagent bifurcado](/docs/pt/skills#run-skills-in-a-subagent)
* Em uma execução `-p` com saída de texto ou JSON

Em um aplicativo host que solicita a lista de descobertas, como o [aplicativo desktop](/docs/pt/desktop), Claude relata as descobertas da revisão através da ferramenta [`ReportFindings`](/docs/pt/tools-reference). Claude Code renderiza o relatório como uma lista de descobertas, e cada entrada mostra a localização do arquivo, um resumo de uma frase e uma tag de categoria como `correctness` quando a descoberta carrega uma. Uma solicitação de host se aplica em cada nível de esforço e requer Claude Code v2.1.218 ou posterior.

Quando Claude corrige descobertas relatadas posteriormente na sessão, ele as relata novamente, e Claude Code marca cada descoberta na lista de descobertas atualizada como corrigida, ignorada ou sem alteração necessária.

<h3 id="what-the-review-reads-and-edits">
  O que a revisão lê e edita
</h3>

A revisão segue seu `CLAUDE.md` como qualquer sessão Claude Code, mas não lê [`REVIEW.md`](#review-md). Uma revisão em segundo plano aplica suas edições `--fix` fora dos [checkpoints](/docs/pt/checkpointing#subagent-edits-not-restored) de sua sessão, portanto `/rewind` não as desfaz; use git para revertê-las. Quando a revisão [é executada em primeiro plano](#run-in-the-foreground), ela edita sua árvore de trabalho durante sua própria vez, portanto `/rewind` restaura suas edições como de costume.

<h3 id="tune-effort-and-arguments">
  Ajustar esforço e argumentos
</h3>

Passe um [nível de esforço](/docs/pt/model-config#adjust-effort-level) para trocar cobertura por confiança. Em `low` e `medium`, a revisão relata apenas as descobertas em que tem mais confiança, portanto você vê menos falsos positivos; `high` até `max` ampliam a cobertura e podem incluir descobertas em que a revisão tem menos certeza.

Quando você não digita um nível, a revisão reutiliza o último nível de `low` até `max` que você digitou, mesmo em uma sessão anterior, e Claude Code mostra um aviso como `Reusing high effort, the level you typed last time`. Digite um nível, como `/code-review high`, para alterar o que as execuções posteriores reutilizam; um nível que você passa em uma execução `-p` não interativa não o atualiza. `ultra` nem atualiza nem usa o nível lembrado. Se você nunca digitou um nível, a revisão usa o esforço atual da sessão. Antes da v2.1.223, um `/code-review` sem um nível sempre usava o esforço atual da sessão.

Após o nível de esforço e sinalizadores, Claude Code lê o resto da linha de uma de duas maneiras:

* **Sem `ultra`**: tudo o que resta é o alvo da revisão, mesmo quando começa com outro nome de comando. `/code-review /fix-issue 123` revisa com `/fix-issue 123` como texto alvo em vez de carregar `/fix-issue` como um [skill empilhado](/docs/pt/skills#pass-arguments-to-skills) separado. Antes da v2.1.218, um comando empilhado após `/code-review` se expandia como seu próprio skill.
* **Com `ultra`**: Claude Code lê uma única palavra como uma branch base ou número de PR, e transforma texto mais longo que não nomeia uma branch ou PR em [uma nota anexada à revisão](/docs/pt/ultrareview#pass-a-request-in-plain-words). `/code-review ultra check my auth changes` revisa sua branch atual, e Claude relaciona as descobertas à sua nota.

<h3 id="run-in-the-foreground">
  Executar em primeiro plano
</h3>

A revisão é executada em segundo plano por padrão; antes da v2.1.218, era executada dentro de sua conversa. Ela é executada em primeiro plano em casos como estes:

* Você executa `/code-review` novamente enquanto uma revisão anterior ainda está em andamento
* Você a executa em modo não interativo, com o sinalizador `-p` ou o Agent SDK; Claude Code aguarda a revisão e inclui as descobertas na resposta, exceto para `ultra`, que [inicia a revisão na nuvem sem aguardar](#escalate-to-ultrareview)
* Você define [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/pt/env-vars) como `1`, o que também desativa todos os outros recursos de tarefa em segundo plano

<h3 id="let-claude-start-the-review">
  Deixar Claude iniciar a revisão
</h3>

Claude pode iniciar `/code-review` por conta própria. Peça-lhe para revisar suas alterações em linguagem simples e ele pode executar o skill sem você digitar o comando, e uma [tarefa agendada](/docs/pt/scheduled-tasks) com `/code-review` como seu prompt executa a revisão.

Uma tarefa agendada nunca inicia a [revisão na nuvem](#escalate-to-ultrareview), portanto agende `/code-review` sem o argumento `ultra`.

Para impedir que Claude e tarefas agendadas iniciem a revisão enquanto mantêm `/code-review` disponível para você digitar, adicione uma entrada [`skillOverrides`](/docs/pt/skills#override-skill-visibility-from-settings) a um [arquivo de configurações](/docs/pt/settings#where-settings-live) como `~/.claude/settings.json`:

```json theme={null}
{
  "skillOverrides": {
    "code-review": "user-invocable-only"
  }
}
```

Antes da v2.1.246, Claude iniciava `/code-review` por conta própria apenas onde um sinalizador de recurso obtido da Anthropic o ativava. Em [sessões que não buscam sinalizadores de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching), `/code-review` era executado apenas quando você o digitava, e um `/code-review` agendado chegava ao Claude como texto simples.

<h3 id="escalate-to-ultrareview">
  Escalar para ultrareview
</h3>

`/code-review ultra --fix` executa a [ultrareview](/docs/pt/ultrareview) mais profunda na nuvem, depois aplica suas descobertas à sua árvore de trabalho quando elas retornam em sua sessão.

Ultrareview usa seu próprio escopo: sua branch atual contra a branch padrão do repositório, mais alterações não confirmadas e preparadas na árvore de trabalho. Para alterações não confirmadas em arquivos nomeados como credenciais ou chaves, como arquivos `.env` e `*.tfvars`, Claude Code segue as regras para [carregar um repositório local para uma sessão na nuvem](/docs/pt/claude-code-on-the-web#send-local-repositories-without-github). Passe um nome de branch, como `/code-review ultra develop`, para comparar contra uma base diferente.

Quando o alvo é um pull request `github.com`, você pode fazer com que Claude [publique as descobertas concluídas no PR](/docs/pt/ultrareview#post-findings-to-the-pull-request) como um comentário de sua conta GitHub. Requer Claude Code v2.1.227 ou posterior.

<Note>
  Ultrareview requer autenticação com uma conta claude.ai e não está disponível no Amazon Bedrock, na Agent Platform do Google Cloud, no Microsoft Foundry, ou para organizações com Zero Data Retention habilitado. Quando ultrareview não está disponível, `/code-review ultra` executa uma revisão local em sua sessão.
</Note>

Para iniciar uma revisão na nuvem a partir de um script ou CI, execute `claude -p '/code-review ultra'`. Claude Code inicia a revisão e imprime um link para rastreá-la. Requer Claude Code v2.1.218 ou posterior.

Quando a revisão cobraria [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), Claude Code para antes de iniciar, porque a confirmação de cobrança precisa de uma sessão interativa. Execute o [subcomando `claude ultrareview`](/docs/pt/ultrareview#run-ultrareview-non-interactively); ao executá-lo, você consente com a cobrança.

O comando foi nomeado `/simplify` antes da v2.1.147, quando aplicava correções por padrão. `/simplify` executa uma revisão separada apenas de limpeza que aplica correções sem procurar por bugs. Se você criou scripts com `/simplify` para busca de bugs, mude para `/code-review --fix`.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Commands](/docs/pt/commands): execute `/code-review` em uma sessão local do Claude Code para verificar um diff antes de fazer push
* [GitHub Actions](/docs/pt/github-actions): execute Claude em seus próprios fluxos de trabalho do GitHub Actions para automação personalizada além de code review
* [GitLab CI/CD](/docs/pt/gitlab-ci-cd): integração Claude auto-hospedada para pipelines GitLab
* [Memory](/docs/pt/memory): como arquivos `CLAUDE.md` funcionam em Claude Code
* [Analytics](/docs/pt/analytics): rastreie o uso de Claude Code além de code review
* [How Anthropic secures its AI-native software development lifecycle](https://claude.com/blog/how-anthropic-secures-its-ai-native-software-development-lifecycle): como a revisão automatizada se encaixa como uma camada do processo de desenvolvimento seguro de software nativo de IA da Anthropic
