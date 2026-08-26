> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kit de comunicações

> Anúncios de lançamento, mensagens de campanha contínua e respostas de FAQ para implementar Claude Code em sua organização de engenharia.

Esta página é para administradores e líderes de engenharia que estão implementando Claude Code em um time. Ela fornece anúncios de lançamento prontos para copiar, uma campanha de dicas e truques, e respostas de uma linha para as perguntas que você será mais frequentemente questionado.

<Note>
  Trate tudo aqui como rascunho, não como cópia final. Reescreva cada mensagem na voz da sua organização, troque as tarefas de exemplo por bugs e módulos reais do seu próprio código, e substitua os `[espaços reservados entre colchetes]` antes de enviar. Os anúncios que impulsionam a adoção são aqueles que parecem ter sido escritos por alguém da sua empresa.
</Note>

<h2 id="launch-communications">
  Comunicações de lançamento
</h2>

Um anúncio em dois formatos, mais duas variantes opcionais. Escolha o que melhor se adequa ao seu lançamento e reescreva a partir daí.

<h3 id="before-you-send">
  Antes de enviar
</h3>

Trabalhe através desta lista de verificação antes do anúncio sair. Cada item fecha uma lacuna que de outra forma se torna um tópico de suporte no dia do lançamento.

| Item                                                                                                       | Por que importa                                                                                                                       |
| ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Canal `#claude-code` criado e vinculado na mensagem                                                        | Dá às perguntas um único lugar para chegar                                                                                            |
| Comando de instalação testado em pelo menos uma máquina em seu ambiente                                    | Detecta problemas de proxy ou firewall antes que todos os enfrentem de uma vez                                                        |
| Link de segurança e tratamento de dados pronto ([Uso de dados](/docs/pt/data-usage) ou seu equivalente interno) | "Para onde vai meu código?" será a primeira resposta                                                                                  |
| Uma tarefa concreta de primeiro passo escolhida, um bug real ou arquivo em seu código                      | Exemplos genéricos não convertem; "corrigir o teste instável em `auth_test.go`" funciona                                              |
| Um proprietário nomeado para o canal pelos primeiros 48 horas                                              | Perguntas não respondidas no dia do lançamento matam o impulso                                                                        |
| Um patrocinador da C-suite alinhado para enviar ou co-assinar o anúncio                                    | Lançamentos enviados por executivos consistentemente veem maior adoção na primeira semana do que aqueles enviados por administradores |

<h3 id="the-announcement">
  O anúncio
</h3>

Use isto como sua mensagem padrão de lançamento em toda a organização. Ele cobre o que é Claude Code, fornece um caminho de instalação de dois minutos, oferece aos leitores uma tarefa concreta para tentar e responde "para onde vai meu código?" antes que alguém tenha que perguntar.

<Tabs>
  <Tab title="Email">
    ```text theme={null}
    Assunto: Claude Code está ativo para [Engenharia / seu time]

    Time,

    A partir de hoje você tem acesso a Claude Code, um agente de codificação de IA que funciona em
    seu terminal, lê seu código real, e trabalha através de tarefas reais de ponta a ponta: depuração, refatorações, testes, PRs. Não é autocompletar e não é
    uma janela de chat. Ele edita arquivos, executa seus comandos e pede permissão
    antes de qualquer coisa arriscada.

    Comece em dois minutos:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <seu-repositório>
        claude

    Depois execute /init uma vez. Claude lê seu projeto e escreve um CLAUDE.md com
    seus comandos de compilação e convenções, para que você pare de re-explicar o básico.

    Depois tente um destes no repositório em que você já está:

      - "O teste em [arquivo] é instável. Descubra por que e corrija"
      - "Me mostre como [módulo] lida com [X]"
      - "Olhe meu diff funcionando e me diga o que é arriscado antes de eu fazer push"

    Para onde seu código vai: Claude Code funciona em seu terminal e fala diretamente
    com a API da Anthropic, sem servidores de terceiros no meio. Ele pede antes de
    editar arquivos ou executar comandos. Sob nosso acordo Enterprise, Anthropic
    não usa seu código ou prompts para treinar seus modelos.
    Detalhes: https://code.claude.com/docs/pt/data-usage
             https://code.claude.com/docs/pt/security

    Para onde ir com perguntas: #claude-code. [Nome do proprietário] está observando
    esta semana.

    - [Nome]

    P.S. Prefere seu editor? Há uma extensão VS Code e um
    plugin JetBrains. Mesmo agente, sem terminal necessário.
    ```
  </Tab>

  <Tab title="Slack ou Teams">
    ```markdown theme={null}
    🚀 *Claude Code está ativo para [time]*

    Agente de codificação de IA, funciona em seu terminal, lê seu repositório, faz trabalho real:
    bugs, refatorações, testes, PRs. Pede antes de tocar em qualquer coisa.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd seu-repositório` → `claude`

    *Primeira coisa para tentar* → execute `/init`, depois: "o teste em [arquivo] é instável,
    descubra por que e corrija".

    🔒 Funciona em seu terminal, fala apenas com a API da Anthropic. Sob nosso
    plano Enterprise seu código e prompts não são usados para treinar modelos.
    Uso de dados → https://code.claude.com/docs/pt/data-usage

    📚 Guia de início rápido · VS Code · Curso gratuito de 1 hora
       https://code.claude.com/docs/pt/quickstart
       https://code.claude.com/docs/pt/vs-code
       https://anthropic.skilljar.com/claude-code-in-action

    Perguntas → esta thread. [Proprietário] está na ponta.
    ```
  </Tab>
</Tabs>

<h3 id="executive-sponsor-variant">
  Variante de patrocinador executivo
</h3>

Envie isto do seu executivo patrocinador, como o CTO, CIO ou SVP de Engenharia, sob seu nome e de sua conta. Lançamentos que saem sob o nome de um executivo consistentemente veem taxas de abertura mais altas e ativação mais rápida na primeira semana do que a mesma mensagem de um administrador ou time de ferramentas. Sinaliza uma prioridade da empresa em vez de um experimento opcional.

Esta versão é deliberadamente reduzida a um pedido: instale e execute em uma tarefa real. O trabalho do executivo é fazer o pedido chegar; o anúncio padrão e `#claude-code` lidam com o como.

<Tabs>
  <Tab title="Email">
    ```text theme={null}
    Assunto: Uma coisa que gostaria que cada engenheiro tentasse esta semana

    Time,

    Ativamos Claude Code para toda a engenharia. É um agente de IA
    que funciona diretamente em seu terminal, em seu código real, e os
    resultados iniciais de times que já o usam são fortes o suficiente para que eu queira
    que todos o usem esta semana.

    Estou pedindo dez minutos:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <seu-repositório>
        claude

    Depois dê a ele uma tarefa real: o bug que você vem adiando, ou "me mostre
    como [módulo] funciona".

    Esse é o pedido inteiro. [Nome do proprietário] e time estão em #claude-code para
    qualquer coisa que você encontre pelo caminho.

    - [Nome do Executivo]
      [Título]
    ```
  </Tab>

  <Tab title="Slack ou Teams">
    ```markdown theme={null}
    📣 *De [Nome do Executivo]: uma coisa para tentar esta semana*

    Ativamos *Claude Code* para toda a engenharia. Resultados iniciais são
    fortes o suficiente para que eu peça a todos que dediquem dez minutos a trabalho real esta semana.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd seu-repositório` →
    `claude` → dê a ele uma tarefa real.

    Isso é tudo. Perguntas → #claude-code.
    ```
  </Tab>
</Tabs>

<h3 id="pilot-group-variant">
  Variante de grupo piloto
</h3>

Use para um lançamento em fases. Envie apenas para a coorte piloto.

```text theme={null}
Assunto: Você está no piloto de Claude Code

[Nome / time],

Você está na primeira onda de Claude Code em [empresa]. Escolhemos este grupo
porque você o colocará em problemas reais e nos dirá a verdade sobre isso.

O pedido: use-o em pelo menos uma tarefa real esta semana, depois deixe uma nota em
#claude-code-pilot cobrindo o que funcionou, o que foi chato e o que
o surpreendeu. Esse feedback decide como implementamos para todos os outros.

[Continue com "Comece em dois minutos" do anúncio padrão]

Uma coisa extra para pilotos: em sua primeira mudança de múltiplos arquivos, pressione Shift+Tab
até ver "plan". Claude exibirá exatamente o que pretende fazer
antes de tocar em um arquivo. É a maneira mais rápida de calibrar quanto
confiar nele.
```

<h3 id="champion-recruitment-dm">
  DM de recrutamento de campeão
</h3>

Após o lançamento, envie DM para as duas ou três pessoas mais ativas em `#claude-code`.

```text theme={null}
Ei [nome], seus posts em #claude-code estão fazendo mais pela adoção do que meu
anúncio fez. Algumas pessoas me disseram que sua [thread / screenshot]
foi o motivo pelo qual realmente tentaram.

Quer tornar isso semi-oficial? Pouco esforço: principalmente continue postando o que
você está postando, mais primeiro acesso a novos recursos e uma linha direta para o
time da Anthropic. Posso compartilhar um playbook curto se você estiver dentro.
```

<h2 id="tips-and-tricks-campaign">
  Campanha de dicas e truques
</h2>

Mensagens prontas para colar no Slack ou Teams projetadas para impulsionar a ativação de recursos após o lançamento. Cada uma segue o mesmo padrão: um gancho, o resultado, um prompt "tente agora" e um link de documentação. Distribua uma ou duas por semana em `#claude-code`, ou escolha o punhado que corresponde às lacunas do seu time. Elas funcionam independentemente sem ordem necessária.

Copie o corpo da mensagem de cada bloco diretamente no Slack ou Teams. Substitua `[espaços reservados entre colchetes]` antes de enviar.

<h3 id="get-started">
  Comece
</h3>

**Escolhendo o modelo certo**

```markdown theme={null}
🎯 *Dica: Combine o modelo ao momento*

Usar Opus para corrigir um erro de digitação queima computação. Usar Haiku para uma refatoração de 12 arquivos
é pedir por uma refeita.

Claude Code funciona nos mesmos modelos que o aplicativo Claude, e você pode alternar
no meio da sessão. *Sonnet* é o padrão de trabalho para trabalho de recursos cotidianos,
bugs, testes e revisões. Recorra a *Opus* em refatorações grandes, depuração complicada,
ou qualquer coisa de alto risco. Desça para *Haiku* para perguntas rápidas,
formatação e edições mecânicas onde a velocidade vence. *Fable 5* é o modelo mais
capaz para suas tarefas mais difíceis e de longa duração; não é o
padrão, então selecione-o com `/model fable`, e observe que conteúdo de cibersegurança e
biologia volta automaticamente para Opus.

*Tente agora:* digite `/model` e escolha Sonnet se você ainda não o fez. É
o padrão certo para a maioria das tarefas.

📖 Configuração de modelo → https://code.claude.com/docs/pt/model-config
```

| Modelo  | Melhor para                                                                                                                                                                                       |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Fable 5 | As tarefas mais difíceis e de longa duração. Apenas com opt-in: selecione com `/model fable`. Conteúdo de cibersegurança ou biologia [volta para Opus](/docs/pt/model-config#automatic-model-fallback) |
| Opus    | Refatorações em larga escala, depuração complexa, decisões de arquitetura, mudanças de alto risco                                                                                                 |
| Sonnet  | Trabalho de recursos cotidianos, correções de bugs, testes, documentação, revisão de código. Padrão recomendado.                                                                                  |
| Haiku   | Perguntas rápidas, formatação, edições mecânicas, iteração rápida                                                                                                                                 |

**Vitórias rápidas para tentar primeiro**

```markdown theme={null}
🚀 *Dica: Três coisas para tentar em seus primeiros 10 minutos*

Instalou Claude Code mas não tem certeza do que realmente pedir? Comece com as
coisas que vêm te incomodando a semana toda.

  - Corrija algo chato: "o teste em [arquivo] é instável, descubra por que"
  - Oriente-se em código que você não escreveu: "me mostre como [módulo] funciona"
  - Verifique a sanidade antes de fazer push: "olhe meu diff funcionando e me diga o que
    parece arriscado"

Nenhuma dessas precisa de configuração. Apenas `cd` para seu repositório e execute `claude`.

*Tente agora:* escolha o bug que você vem evitando e cole a mensagem de erro.

📖 Guia de início rápido → https://code.claude.com/docs/pt/quickstart
```

<h3 id="project-memory">
  Memória do projeto
</h3>

**`/init` e CLAUDE.md**

```markdown theme={null}
📁 *Dica: Pare de re-explicar seu repositório a cada sessão*

Dizendo a Claude "usamos pnpm, não npm" pela quinta vez? Há uma
correção única.

Execute `/init` uma vez por repositório. Claude lê a estrutura do seu projeto e escreve um
arquivo CLAUDE.md com seus comandos de compilação, arquitetura e convenções.
Cada sessão futura naquele repositório começa a partir deste arquivo automaticamente. Mantenha
em menos de duas telas. É uma cola de referência, não documentação.

*Tente agora:* abra seu repositório principal, execute `claude`, digite `/init`. Trinta
segundos, compensa cada sessão depois.

📖 CLAUDE.md e memória do projeto → https://code.claude.com/docs/pt/memory
```

**Referências com @**

```markdown theme={null}
📎 *Dica: Pare de colar conteúdo de arquivo no chat*

Copiando 200 linhas de um componente em seu prompt para que Claude possa "ver"? 
Você não precisa.

Digite `@` depois um caminho de arquivo. Claude puxa o arquivo diretamente para o contexto.
Funciona para diretórios inteiros também.

> os estilos em @src/components/Button.tsx parecem errados, verifique contra
> @docs/design-system.md

*Tente agora:* digite `@` depois Tab. Autocompletar mostra cada arquivo ao alcance.

📖 Referenciando arquivos → https://code.claude.com/docs/pt/common-workflows
```

<h3 id="control-and-safety">
  Controle e segurança
</h3>

**Modos de permissão**

```markdown theme={null}
🛡️ *Dica: Um pressionamento de tecla entre "olhar mas não tocar" e "apenas faça"*

Às vezes você quer que Claude peça antes de cada edição. Às vezes você apenas quer
que ele envie. Você não deveria ter que escolher um para sempre.

*Shift+Tab* alterna quanto de liberdade Claude recebe: *Manual* (a
configuração padrão `default`) pede antes de cada ação, *acceptEdits* deixa edições de arquivo e comandos comuns do sistema de arquivos
fluirem enquanto ainda verifica antes de outros comandos shell, e *plan*
propõe mudanças para sua aprovação antes de qualquer coisa ser tocada. Plan mode é
o construtor de confiança, então comece lá para qualquer coisa tocando múltiplos arquivos.

*Tente agora:* em sua próxima refatoração, pressione Shift+Tab até ver "plan",
depois descreva a mudança. Você receberá uma proposta completa antes de um único arquivo
se mover.

📖 Modos de permissão → https://code.claude.com/docs/pt/permissions
```

**Checkpointing e `/rewind`**

```markdown theme={null}
⏪ *Dica: Há um botão desfazer para a conversa inteira*

Claude foi pelo caminho errado três turnos atrás e agora você está desembaraçando?
Você não precisa consertar para frente.

`/rewind` volta a um ponto anterior na conversa, incluindo as
mudanças de arquivo que Claude fez pelo caminho. Checkpointing é automático; você
não configura nada.

*Tente agora:* pressione *Esc* duas vezes para abrir o menu de rewind, ou digite `/rewind`.
Escolha o ponto antes das coisas darem errado.

📖 Checkpointing → https://code.claude.com/docs/pt/checkpointing
```

<h3 id="connect-your-tools">
  Conecte suas ferramentas
</h3>

**Conectores MCP**

```markdown theme={null}
🔌 *Dica: Deixe Claude ler seu rastreador de problemas para que você não tenha que colar tickets*

Colar tickets do Jira no terminal parece um passo para trás.
É.

Um arquivo de configuração (`.mcp.json` na raiz do seu projeto) conecta Claude ao GitHub,
Jira, Linear, ou qualquer rastreador que você use. Depois "qual é o problema
de prioridade máxima atribuído a mim?" e "vá em frente e corrija" acontecem na mesma
conversa.

*Tente agora:* peça a Claude "configure um conector MCP para [GitHub/Jira/Linear]
neste repositório". Ele escreverá a configuração para você.

📖 Conectores MCP → https://code.claude.com/docs/pt/mcp
```

<h3 id="automate-your-workflows">
  Automatize seus fluxos de trabalho
</h3>

**Skills**

```markdown theme={null}
⚡ *Dica: Transforme aquele prompt que você fica redigitando em um comando*

Digitou "resuma o que trabalhei hoje a partir do git log, formate para standup"
três vezes esta semana? Esse é um slash command esperando para acontecer.

Um arquivo SKILL.md em `.claude/skills/<nome>/` se torna um prompt reutilizável; digite
`/nome` para executá-lo. Faça um na segunda vez que você digita um prompt multi-passo
que você digitou antes. Caminho mais fácil: peça a Claude para fazer para você.

*Tente agora:* digite "faça-me uma skill /standup que resuma o que trabalhei
hoje a partir do git log", depois execute `/standup` amanhã de manhã.

📖 Skills → https://code.claude.com/docs/pt/skills
```

**Hooks**

```markdown theme={null}
🔔 *Dica: Receba um ping quando sua refatoração terminar*

Sentado em sua mesa observando Claude trabalhar através de uma tarefa longa? Você tem
coisas melhores para fazer por esses oito minutos.

Hooks são comandos shell que disparam em eventos de Claude Code. Um hook Stop que
envia uma notificação de desktop significa que você pode iniciar uma refatoração longa, se afastar,
e receber um ping no momento em que termina.

*Tente agora:* peça a Claude "adicione um hook Stop que envie uma notificação de desktop
quando você terminar". Ele escreverá o script e conectará.

📖 Guia de hooks → https://code.claude.com/docs/pt/hooks-guide
```

<h3 id="day-to-day-development">
  Desenvolvimento dia a dia
</h3>

**Screenshots e imagens**

```markdown theme={null}
📸 *Dica: Pare de descrever o diálogo de erro. Apenas mostre.*

Digitando "há uma caixa vermelha que diz algo sobre uma referência nula
e está apontando para a linha 47-ish"? Faça uma screenshot.

Arraste uma screenshot diretamente para o terminal e Claude a vê: diálogos de erro, mockups de UI,
fotos de quadro branco, exportações do Figma. *Ctrl+V* cola da área de transferência (use Ctrl+V no macOS também, não Cmd+V).

*Tente agora:* na próxima vez que algo visual quebrar, faça uma screenshot e cole
diretamente no prompt. Depois apenas digite "o que está errado aqui?"

📖 Trabalhando com imagens → https://code.claude.com/docs/pt/common-workflows
```

**Fluxos de trabalho Git**

```markdown theme={null}
🌿 *Dica: Passe toda a cerimônia git*

A correção levou 5 minutos. A mensagem de commit, branch e descrição de PR
levaram 15. Essa proporção está errada.

Claude lida com o fluxo git completo: commits com mensagens convencionais,
branches, PRs com resumos apropriados. Um pedido: "corrija o off-by-one, commit
com uma mensagem de commit convencional e abra um PR." Revisando o trabalho de alguém?
Cole a URL do PR e peça a Claude para te mostrar o diff.

*Tente agora:* após sua próxima correção, em vez de alternar para seu cliente git,
apenas digite "commit isto com uma boa mensagem e abra um PR".

📖 Criando pull requests → https://code.claude.com/docs/pt/common-workflows
```

<h3 id="share-and-scale">
  Compartilhe e escale
</h3>

**Plugins**

```markdown theme={null}
📦 *Dica: Alguém provavelmente já construiu essa skill*

Prestes a gastar uma hora construindo um comando `/deploy`? Verifique se já
existe.

Skills são agrupadas e compartilhadas como plugins. `/plugin` navega o que está
disponível e instala em um passo. Cinco minutos de navegação podem economizar uma hora de construção.

*Tente agora:* digite `/plugin` e role. Você encontrará pelo menos uma
coisa que não sabia que queria.

📖 Plugins → https://code.claude.com/docs/pt/plugins
```

<h3 id="security-and-admin">
  Segurança e administração
</h3>

**Arquitetura de segurança**

```markdown theme={null}
🔐 *Dica: A resposta para "isto é seguro?" para a próxima vez que você for perguntado*

Alguém no seu time vai perguntar "espera, para onde vai meu código?"
Aqui está a versão curta que você pode colar.

Permissão-primeiro por design. Cada edição de arquivo, comando shell e chamada externa
é controlada por sua aprovação. O CLI funciona em seu terminal e fala
diretamente com a API da Anthropic, sem servidores de terceiros, e suporta
sandboxing opcional no nível do SO para comandos shell. Sob nosso plano Enterprise,
Anthropic não usa seu código ou prompts para treinar seus modelos.

*Tente agora:* salve esses dois links para a próxima vez que a pergunta surgir.
Eles respondem a maioria das perguntas de revisão de segurança.

📖 https://code.claude.com/docs/pt/security
📖 https://code.claude.com/docs/pt/data-usage
```

**Melhores práticas**

```markdown theme={null}
✅ *Dica: Os 4 hábitos que separam "tentei uma vez" de "uso diariamente"*

A maioria das pessoas que desistem de Claude Code pulou um destes. A maioria das pessoas
que continuam fizeram todos os quatro na primeira semana.

  - Comece em plan mode para qualquer coisa tocando múltiplos arquivos
  - Execute /init cedo; contexto se compõe
  - Revise diffs antes de fazer commit; Claude pode estar confiante e errado
  - Verifique mudanças que tocam caminhos críticos; trate como um junior afiado,
    não um oráculo

*Tente agora:* se você fez apenas um ou dois destes, escolha o que você está
perdendo e faça em sua próxima tarefa. Poste o que mudou em #claude-code.

📖 Melhores práticas → https://code.claude.com/docs/pt/best-practices
```

<h2 id="quick-reference">
  Referência rápida
</h2>

<h3 id="faq-responses">
  Respostas de FAQ
</h3>

Respostas de uma linha para as perguntas que você será mais frequentemente questionado.

| Pergunta                            | Resposta                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Funciona em VS Code?"              | Sim. Há uma extensão VS Code e um plugin JetBrains com os mesmos recursos, incorporados em seu editor. [VS Code →](/docs/pt/vs-code)                                                                                                                                                                                                                                                                                                                                     |
| "Preciso configurar algo primeiro?" | Não. Instale, depois execute `claude` em qualquer repositório. Execute `/init` uma vez e você está pronto. [Guia de início rápido →](/docs/pt/quickstart)                                                                                                                                                                                                                                                                                                                |
| "Para onde vai meu código?"         | O CLI funciona em seu terminal e envia contexto para a API da Anthropic para inferência, sem servidores de terceiros. Sob seu plano Enterprise, seu código e prompts não são usados para treinar modelos. [Uso de dados →](/docs/pt/data-usage)                                                                                                                                                                                                                          |
| "Pode ver meu repositório inteiro?" | Ele lê o que você dá acesso. Leituras de arquivo dentro de seu diretório de trabalho não solicitam; prompts de permissão controlam edições, comandos shell não somente leitura e leituras de ferramentas de arquivo fora daquele diretório. Um conjunto integrado de comandos shell somente leitura como `ls` e `cat` é executado sem solicitar; restrinja com [regras de sandbox `denyRead`](/docs/pt/sandboxing#filesystem-isolation). [Permissões →](/docs/pt/permissions) |
| "Como isto é diferente do Copilot?" | Copilot autocompletar linhas. Claude Code é um agente que lê arquivos, executa comandos e faz edições de múltiplos arquivos. [Visão geral →](/docs/pt/overview)                                                                                                                                                                                                                                                                                                          |
| "O que devo tentar primeiro?"       | Um bug que você vem adiando porque é tedioso. "O teste em \[arquivo] é instável, descubra por que." [Guia de início rápido →](/docs/pt/quickstart)                                                                                                                                                                                                                                                                                                                       |

<h3 id="prompt-templates">
  Modelos de prompt
</h3>

Compartilhe estes prompts iniciais com engenheiros que instalaram mas não têm certeza do que pedir. Cada um é fraseado da maneira que seria digitado em uma sessão real; substitua as peças entre colchetes com arquivos do seu próprio repositório.

| Tarefa                       | Prompt                                                                                  |
| ---------------------------- | --------------------------------------------------------------------------------------- |
| Corrija um bug               | "os testes em \[arquivo] estão falhando, descubra por que e corrija"                    |
| Entenda código               | "me mostre como \[módulo] funciona, depois me diga onde está o ponto de entrada"        |
| Refatoração segura           | "refatore \[módulo] para \[objetivo], use plan mode para que eu possa revisar primeiro" |
| Escreva testes               | "escreva testes para \[arquivo] que cubram os casos extremos em torno de \[cenário]"    |
| Revise antes de fazer commit | "olhe meu diff funcionando e me diga o que parece arriscado"                            |
| Abra um PR                   | "corrija \[problema], escreva um commit convencional e abra um PR com um resumo"        |
| Faça uma skill               | "faça-me uma skill /ship que execute testes e lint antes de fazer commit"               |
| Depure um stack trace        | "aqui está o stack trace, encontre a causa raiz, não apenas coloque um curativo"        |

<Tip>
  Claude Code é lançado frequentemente. Verifique detalhes específicos da versão contra a [página inicial da documentação](/docs/pt/overview) antes de distribuir internamente.
</Tip>
