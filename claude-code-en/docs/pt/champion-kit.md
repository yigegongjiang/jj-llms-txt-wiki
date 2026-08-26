> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kit do campeão

> Um guia prático para engenheiros que defendem Claude Code internamente: o que compartilhar, como responder perguntas e como aumentar a adoção na sua equipe.

Esta página é para engenheiros individuais que já estão usando Claude Code e desejam ajudar sua equipe a adotá-lo. Ela cobre o que compartilhar, como responder às perguntas que você receberá, um guia de trinta dias e respostas a preocupações comuns.

A adoção de uma ferramenta de desenvolvedor raramente acontece por causa de um anúncio de lançamento. Acontece porque alguém da equipe começa a usar a ferramenta bem, fala sobre ela abertamente e facilita para outros seguirem. O trabalho que você faz como campeão tem um efeito desproporcional: cada exemplo que você compartilha encurta a curva de aprendizado para os engenheiros que vêm depois de você, e cada pergunta que você responde em público transforma a experiência de uma pessoa em algo que toda a equipe pode construir. Você está agindo como um multiplicador para sua equipe, não como um help desk, e este guia é estruturado para manter o papel sustentável nesses termos.

<h2 id="the-champion-role">
  O papel do campeão
</h2>

O papel consiste em três comportamentos que se reforçam mutuamente.

| Comportamento                          | Como se parece na prática                                                                                                                                                                             | Por que importa                                                                                                                                                                                                   |
| -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Compartilhe o que você descobre        | Poste os prompts, capturas de tela e pequenas vitórias do seu próprio trabalho nos lugares que sua equipe já lê, como um canal de engenharia, uma thread de standup ou uma descrição de pull request. | Exemplos extraídos do seu próprio codebase são mais persuasivos do que qualquer documentação externa, porque os colegas podem ver exatamente como a ferramenta se aplica aos problemas que compartilham com você. |
| Seja a pessoa que as pessoas perguntam | Quando um colega pergunta como você realizou algo, responda com o prompt real que você usou para que ele possa aplicá-lo diretamente à sua própria tarefa.                                            | Um exemplo concreto e executável remove a lacuna entre curiosidade e um primeiro uso bem-sucedido, que é onde a maioria dos esforços de adoção estagna.                                                           |
| Cresça o círculo                       | Estabeleça um pequeno número de hábitos leves e recorrentes, como um canal dedicado ou uma thread semanal, para que o momentum continue mesmo quando sua atenção estiver em outro lugar.              | A adoção que depende de uma única pessoa é frágil. A adoção que é realizada por hábitos compartilhados continua a se compor por conta própria.                                                                    |

A maioria disso se encaixa naturalmente no trabalho que você já está fazendo. A diferença é uma pequena quantidade de intenção adicional sobre onde suas descobertas são postadas e como suas respostas se propagam.

<h3 id="what-this-should-cost-you">
  O que isso deve custar você
</h3>

Defina expectativas com você mesmo e com seu líder. As atividades abaixo são destinadas a se encaixarem em uma semana de trabalho normal, e o papel deve permanecer um multiplicador do seu trabalho existente em vez de uma responsabilidade de suporte adicional.

| Atividade                                       | Tempo por semana    | Orientação                                                                                                                        |
| ----------------------------------------------- | ------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Postando vitórias e prompts                     | Cerca de 15 minutos | Capture-os no momento com uma captura de tela e uma ou duas frases; evite transformá-los em redações formais.                     |
| Respondendo perguntas em um canal compartilhado | Cerca de 20 minutos | Responda publicamente uma vez, depois vincule de volta a essa resposta quando a pergunta se repetir.                              |
| Hospedando uma thread semanal de show-and-tell  | Cerca de 5 minutos  | Você posta o prompt de abertura; a equipe fornece o conteúdo.                                                                     |
| Emparelhamento opcional ou walkthroughs         | 0 a 30 minutos      | Reserve isso para colegas que estão genuinamente bloqueados e ofereça o link [Quickstart](/docs/pt/quickstart) antes de agendar tempo. |

<h2 id="share-what-you-discover">
  Compartilhe o que você descobre
</h2>

Sua própria experiência é o material mais persuasivo que seus colegas encontrarão, porque é específico para o codebase, fluxos de trabalho e problemas que todos compartilham. A documentação diz às pessoas o que é possível; seus posts mostram a eles o que está realmente funcionando em seu ambiente.

<h3 id="what-is-worth-sharing">
  O que vale a pena compartilhar
</h3>

Os posts mais úteis descrevem uma técnica que um colega pode reutilizar amanhã em vez de um resultado que já está completo. As técnicas se compõem conforme se espalham por uma equipe; atualizações de status não.

Exemplos de técnicas reutilizáveis:

* "Aprendi que @-mencionar um diretório funciona. Apontei para `@src/components/` e perguntei quais estavam faltando testes, o que revelou dois que eu tinha negligenciado."
* "Plan mode (`Shift+Tab`) mostra exatamente quais arquivos serão tocados antes de qualquer edição ser feita, é por isso que estou confortável em usá-lo em código compartilhado."
* "Configurei um hook Stop para receber uma notificação de desktop quando uma tarefa longa é concluída. A configuração está na thread."
* "Executar `/init` gera um `CLAUDE.md` do repositório para que o assistente pare de fazer perguntas sobre nossas convenções."

<h3 id="where-to-share-it">
  Onde compartilhá-lo
</h3>

Poste onde sua equipe já lê. O objetivo é colocar exemplos no caminho do trabalho normal em vez de criar um destino.

| Local                                          | Melhor adequado para                                                    | Formato recomendado                                                                      |
| ---------------------------------------------- | ----------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| Um canal `#claude-code` ou de engenharia geral | Descobertas, prompts e momentos "aprendi hoje"                          | Uma captura de tela acompanhada de uma ou duas frases de contexto                        |
| Descrições de pull request                     | Demonstrando a abordagem em código real que os revisores já estão lendo | Uma única linha como "Claude e eu fizemos este refactor; feliz em explicar a abordagem." |
| Standups ou atualizações semanais escritas     | Normalizando o uso com líderes e gerentes de skip-level                 | Uma frase descrevendo um resultado concreto                                              |
| Wiki da equipe ou documentação interna         | Padrões duráveis, skills personalizados e exemplos de `CLAUDE.md`       | Uma página curta, vinculada do tópico do canal para que permaneça descobrível            |

<h3 id="the-format-that-works">
  O formato que funciona
</h3>

Uma captura de tela acompanhada de uma única linha de contexto, ou uma breve descrição antes e depois, é geralmente o nível certo de detalhe. Mantenha cada post curto o suficiente para que alguém rolando ainda absorva o ponto. Uma redação longa tende a ser salva para depois e esquecida, enquanto um post curto com uma captura de tela tende a ser copiado e testado.

Os posts de exemplo abaixo ilustram tom e comprimento; adapte-os em vez de copiar verbatim.

```text theme={null}
Aprendi hoje que @-mencionar um diretório funciona. Apontei para
@src/components/ e perguntei quais componentes estavam faltando testes, e
isso revelou dois que eu tinha esquecido.
```

```text theme={null}
Configurei um hook Stop para receber uma notificação de desktop quando uma
tarefa longa é concluída. Comecei um refactor, me afastei e fui notificado
quando terminou. A configuração está na thread.
```

```text theme={null}
Plan mode é a razão pela qual estou confortável em usar isso em código que
importa. Pressione Shift+Tab até ver "plan"; ele mostra exatamente quais
arquivos ele pretende tocar antes de mudar qualquer coisa.
```

<h2 id="be-the-person-people-ask">
  Seja a pessoa que as pessoas perguntam
</h2>

Depois de compartilhar alguns exemplos, as perguntas virão. É aqui que o papel do campeão tem a maior alavancagem, porque uma boa resposta para uma pessoa frequentemente desbloqueia vários outros que estão observando o mesmo canal.

<h3 id="answer-with-a-prompt-rather-than-an-explanation">
  Responda com um prompt em vez de uma explicação
</h3>

Quando um colega pergunta como você realizou algo, a resposta mais útil é o prompt que você realmente usou. Ele aprenderá mais executando esse prompt contra seu próprio problema do que com qualquer descrição que você pudesse escrever, e isso lhe dá algo em que ele pode agir imediatamente.

```text theme={null}
Colega: Como você conseguiu encontrar essa condição de corrida?

Campeão: Perguntei, "O teste em @tests/scheduler.test.ts é instável, descubra
por quê," e ele rastreou duas promises não unidas no scheduler. Tente a mesma
frase no seu teste.
```

<h3 id="point-at-the-feature-rather-than-the-documentation">
  Aponte para o recurso em vez da documentação
</h3>

Uma resposta como "Tente plan mode, pressione `Shift+Tab` até vê-lo" é mais útil no momento do que um link para a documentação. Se a pessoa precisar de mais profundidade depois, ela encontrará por conta própria; agora ela precisa da única coisa que a desbloqueia.

<h3 id="questions-you-are-likely-to-hear">
  Perguntas que você provavelmente ouvirá
</h3>

| Pergunta                                       | Resposta sugerida                                                                                                                                                                                                                        | Recurso de acompanhamento                               |
| ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| "O que devo tentar primeiro?"                  | Recomende uma tarefa real mas contida, idealmente um bug ou tarefa que a pessoa tem adiado porque é tedioso em vez de difícil.                                                                                                           | [Common workflows](/docs/pt/common-workflows)                |
| "Como confio nela com meu código?"             | Introduza plan mode: pressionar `Shift+Tab` entra nele, Claude propõe exatamente o que pretende mudar, e nada é modificado até que o usuário aprove.                                                                                     | [Permissions](/docs/pt/permissions)                          |
| "Vale a pena o esforço de configuração?"       | A instalação leva aproximadamente dois minutos, é executada no terminal e não requer extensão de IDE. Executar `/init` uma vez é suficiente para começar a trabalhar.                                                                    | [Quickstart](/docs/pt/quickstart)                            |
| "Produziu um resultado incorreto."             | Encoraje-o a fornecer a falha de volta para Claude. Colar a mensagem de erro ou teste falhando é muito mais eficaz do que reformular a solicitação original.                                                                             | [Common workflows](/docs/pt/common-workflows)                |
| "Não entende as convenções do nosso codebase." | Sugira executar `/init` para gerar um arquivo `CLAUDE.md`, depois adicione as convenções da equipe, comandos de teste e quaisquer diretórios que devem ser evitados.                                                                     | [Memory](/docs/pt/memory)                                    |
| "Isso é apenas autocomplete?"                  | Ofereça uma breve demonstração em que Claude explica um arquivo desconhecido, rastreia um bug entre serviços ou elabora um plano de migração. Essas tarefas exigem raciocínio em todo o repositório em vez de completar uma única linha. | Uma demonstração ao vivo de dois minutos                |
| "E quanto à segurança e tratamento de dados?"  | Refira essa pergunta ao seu administrador. A política de implantação e tratamento de dados da sua organização já está configurada, e os campeões não devem improvisar essa resposta.                                                     | [Security](/docs/pt/security) · [Data usage](/docs/pt/data-usage) |

<h2 id="grow-the-circle">
  Cresça o círculo
</h2>

O objetivo não é construir um programa ou possuir um lançamento. É estabelecer um pequeno número de hábitos leves que permitam que o momentum continue depois que você parar de impulsioná-lo ativamente. Quando as perguntas no canal estão sendo respondidas por pessoas além de você, o papel cumpriu seu trabalho.

<h3 id="patterns-that-tend-to-work">
  Padrões que tendem a funcionar
</h3>

| Padrão                                                   | Como executá-lo                                                                                                                                                                                                                                                             | Esforço necessário                                      |
| -------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| Um canal dedicado                                        | Crie um canal `#claude-code` (ou uma thread recorrente em um existente), fixe o link [Quickstart](/docs/pt/quickstart) e um exemplo forte, e responda perguntas publicamente para que cada resposta beneficie todos que estão observando.                                        | Cerca de cinco minutos para configurar, depois ambiente |
| Uma thread semanal de show-and-tell                      | Toda sexta-feira, poste "Com o que Claude ajudou você esta semana?" Nenhuma preparação, slides ou reunião são necessários; capturas de tela e descrições curtas são suficientes.                                                                                            | Cerca de dois minutos por semana                        |
| Compartilhe um skill personalizado                       | Poste seu arquivo `.claude/skills/<name>/SKILL.md` mais útil, por exemplo um skill `/ship` que executa testes e lint antes de fazer commit, com uma descrição de uma linha. Como skills são Markdown simples, colegas podem adotá-los imediatamente.                        | Cerca de cinco minutos por skill                        |
| Gere um guia de configuração a partir do seu próprio uso | Execute `/team-onboarding` em um projeto em que você passou tempo real. Claude verifica suas sessões recentes, comandos e servidores MCP, depois produz um guia que um novo colega pode colar como sua primeira mensagem para reproduzir sua configuração. Fixe-o no canal. | Cerca de dois minutos                                   |
| Emparelhe em uma primeira tarefa                         | Ofereça uma única sessão de emparelhamento de quinze minutos para qualquer pessoa começando. Um resultado bem-sucedido em seu próprio código é mais persuasivo do que qualquer apresentação.                                                                                | Cerca de quinze minutos por pessoa                      |
| Identifique o próximo campeão                            | O colega que mais lhe faz perguntas geralmente está pronto para assumir esse papel. Encaminhe-lhe esta página e divida as responsabilidades do canal entre vocês.                                                                                                           | Negligenciável                                          |

<h3 id="thirty-day-playbook">
  Guia de trinta dias
</h3>

Se um plano solto for útil, a sequência abaixo reflete o que tende a funcionar na maioria das equipes. Ajuste livremente para se adequar ao seu contexto.

<Steps>
  <Step title="Semana 1: Semeie o canal">
    Crie o canal, fixe o [Quickstart](/docs/pt/quickstart) e poste dois ou três de seus próprios exemplos com os prompts incluídos.

    **Sinal de que está funcionando:** alguns colegas reagem ou respondem, e pelo menos uma pergunta é feita no canal.
  </Step>

  <Step title="Semana 2: Comece o ritmo">
    Comece a thread semanal de show-and-tell, responda a todas as perguntas publicamente e compartilhe um skill personalizado ou trecho de `CLAUDE.md`.

    **Sinal de que está funcionando:** alguém além de você posta um exemplo do seu próprio.
  </Step>

  <Step title="Semana 3: Emparelhe e consolide">
    Ofereça duas ou três sessões curtas de emparelhamento e consolide as perguntas e respostas mais comuns em uma mensagem de FAQ fixada.

    **Sinal de que está funcionando:** você vê uso repetido, com os mesmos colegas retornando em vez de tentar uma vez e parar.
  </Step>

  <Step title="Semana 4: Passe adiante">
    Identifique um segundo campeão e compartilhe um breve resumo do que está funcionando e do que não está com seu líder ou administrador.

    **Sinal de que está funcionando:** as perguntas no canal estão sendo respondidas por pessoas além de você.
  </Step>
</Steps>

<h3 id="when-someone-wants-to-go-deeper">
  Quando alguém quer ir mais fundo
</h3>

Você é a introdução calorosa em vez do programa de onboarding. Quando um colega passa de "devo tentar isso" para "como me torno eficaz com isso," aponte-o para as páginas [Quickstart](/docs/pt/quickstart) e [Common workflows](/docs/pt/common-workflows). Elas contêm seções curtas cobrindo os recursos que são genuinamente úteis mas difíceis de descobrir por conta própria.

<h2 id="respond-to-common-concerns">
  Responda a preocupações comuns
</h2>

O ceticismo saudável é esperado; engenheiros devem ser cautelosos com ferramentas que tocam seu código. A resposta mais eficaz raramente é argumentar o caso geral. Em vez disso, reconheça a preocupação, ofereça um breve reframe e proponha uma demonstração concreta no código da própria pessoa. A maioria das preocupações é resolvida por uma única experiência bem-sucedida.

| Preocupação                                       | Resposta sugerida                                                                                                                                                                                                             | Evidência a oferecer                                          |
| ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| "Sou mais rápido sem ela."                        | Isso é provavelmente verdade para código que a pessoa escreve rotineiramente. Sugira tentar em trabalho que ela tende a evitar: arquivos legados, serviços desconhecidos ou scaffolding de teste, onde a alavancagem é maior. | Cronometra uma tarefa tedioso de ambas as maneiras e compara. |
| "Não confio em IA para tocar código de produção." | Concorde que nenhuma mudança deve ser aplicada sem ser lida. Plan mode combinado com revisão de diff normal significa que nada é aplicado que o engenheiro não inspecionou, o mesmo padrão de qualquer pull request.          | Demonstre plan mode em um arquivo real.                       |
| "Isso tornará engenheiros juniores mais fracos."  | Usado bem, é um explicador eficaz. Encoraje engenheiros juniores a pedir a Claude para explicar um arquivo e seus locais de chamada antes de pedir para mudar qualquer coisa.                                                 | Execute "Explain @file and where it is called from" juntos.   |
| "Tentei uma vez e alucinava."                     | Isso é geralmente um problema de contexto em vez de um problema de modelo. @-mencionar os arquivos relevantes, executar `/init` e fornecer a saída de erro real geralmente resolve.                                           | Re-execute seu prompt original com contexto `@` apropriado.   |
| "Não temos tempo para aprender outra ferramenta." | Claude Code é um comando de terminal em vez de uma plataforma. Se não retornar valor na primeira sessão, é razoável deixá-lo de lado.                                                                                         | Uma instalação de dois minutos seguida por um bug real.       |

<h2 id="quick-reference-sheet">
  Folha de referência rápida
</h2>

As técnicas abaixo são as que mais confiabilidade movem alguém de um primeiro teste para uso diário. Fixe esta tabela em um canal ou compartilhe-a por conta própria.

| Técnica                                      | Como aplicá-la                                                                                                                                                                         |
| -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Forneça o contexto certo                     | Use referências `@file` ou `@directory/`, ou cole a saída de erro ou log diretamente. Fornecer contexto relevante é mais eficaz do que prompting elaborado.                            |
| Revise o plano antes da edição               | Pressione `Shift+Tab` para entrar em plan mode. Claude descreverá as mudanças pretendidas para sua aprovação antes de executá-las.                                                     |
| Ensine ao repositório                        | Execute `/init` para gerar um arquivo `CLAUDE.md`, depois adicione suas convenções, comandos de teste e quaisquer diretórios que não devem ser modificados. Veja [Memory](/docs/pt/memory). |
| Reutilize um fluxo de trabalho               | Salve um arquivo `SKILL.md` em `.claude/skills/<name>/` para criar um skill `/name` que toda a equipe pode usar. Veja [Skills](/docs/pt/skills).                                            |
| Mantenha-se informado durante tarefas longas | Configure um hook Stop para receber uma notificação de desktop quando uma tarefa de longa duração é concluída. Veja [Hooks](/docs/pt/hooks-guide).                                          |
| Recupere-se de um resultado incorreto        | Em vez de reformular a solicitação, cole o teste falhando ou stack trace de volta para Claude e peça para ele abordar essa falha específica.                                           |
| Mantenha edições cirúrgicas                  | Peça um diff, ou especifique "apenas mude X." Claude respeita o escopo quando o escopo é declarado.                                                                                    |

<Tip>
  Claude Code é atualizado frequentemente. Verifique detalhes específicos da versão contra a [página inicial da documentação](/docs/pt/overview) antes de distribuir este material internamente.
</Tip>
