> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estilos de saída

> Altere o papel, tom e formato de resposta do Claude Code com um estilo de saída integrado, como Conciso ou Explicativo, ou escreva um estilo personalizado.

Um estilo de saída é um conjunto de instruções que define o papel, tom e formato de resposta do Claude para cada resposta em uma sessão. Claude Code inclui quatro estilos integrados além do padrão, e você pode escrever o seu próprio.

Use um estilo de saída para alterar a forma como Claude responde e trabalha com você durante toda uma sessão, para que você não repita a solicitação em cada prompt. Por exemplo, um estilo integrado pode tornar as respostas mais curtas, adicionar uma explicação de cada alteração, ou fazer com que Claude comece o trabalho sem fazer perguntas rotineiras. Um estilo personalizado também pode transformar Claude em algo diferente de um engenheiro de software, como um assistente de redação ou um analista de dados.

* Para usar um estilo integrado, escolha um dos [estilos de saída integrados](#built-in-output-styles) e [mude para ele](#change-your-output-style).
* Para escrever suas próprias instruções, [crie um estilo de saída personalizado](#create-a-custom-output-style).

<Note>
  Um estilo de saída fornece instruções ao Claude para seguir. Não garante que algo sempre aconteça ou nunca aconteça. Algumas necessidades se encaixam em um recurso diferente:

  * Para o que Claude deve saber sobre seu projeto, use [CLAUDE.md](/docs/pt/memory).
  * Para algo que tem que acontecer toda vez, como formatação após cada edição ou bloqueio de um comando, use um [hook](/docs/pt/hooks-guide).
  * Para skills, subagentes e outras opções, consulte [Escolha entre um estilo de saída e outros recursos](#choose-between-an-output-style-and-other-features).
</Note>

<h2 id="built-in-output-styles">
  Estilos de saída integrados
</h2>

O Claude Code começa no estilo [**Default**](#default), suas instruções padrão para completar tarefas de engenharia de software. Cada um dos outros quatro estilos integrados mantém essas instruções e adiciona as suas próprias.

Esta tabela mostra o que cada estilo muda em uma sessão e quando se encaixa:

| Estilo                      | O que muda                                                                                                      | Use quando                                                                                                                         |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| [Proactive](#proactive)     | Claude começa o trabalho imediatamente e faz suposições razoáveis em vez de perguntar sobre decisões rotineiras | Você quer que Claude continue trabalhando através de decisões rotineiras, e você corrigirá o curso se uma suposição estiver errada |
| [Concise](#concise)         | As respostas começam com o resultado e omitem preâmbulo, narração e recapitulações                              | As respostas padrão são mais longas do que você quer                                                                               |
| [Explanatory](#explanatory) | Claude adiciona blocos `Insight` curtos que explicam as escolhas por trás do código que escreve                 | Você está conhecendo uma base de código ou quer o raciocínio junto com a mudança                                                   |
| [Learning](#learning)       | Claude explica suas escolhas e deixa pequenos pedaços de código para você escrever você mesmo                   | Você quer prática de codificação prática enquanto a tarefa ainda é concluída                                                       |

<h3 id="default">
  Default
</h3>

Default significa que nenhum estilo de saída está selecionado. O Claude Code não adiciona instruções de estilo, e Claude trabalha a partir do prompt do sistema padrão do Claude Code, que é escrito para tarefas de engenharia de software.

`default` aparece na lista `/output-style` com os outros estilos, então você [o seleciona da mesma forma](#change-your-output-style).

<h3 id="proactive">
  Proactive
</h3>

No estilo Proactive, Claude começa a implementar assim que você envia uma tarefa. Ele faz suposições razoáveis sobre decisões rotineiras em vez de parar para perguntar, e não muda para o modo de plano a menos que você peça um plano. Você pode redirecioná-lo em qualquer ponto.

As instruções do estilo também dizem ao Claude para verificar com você na conversa antes de uma ação que exclui dados ou altera um sistema compartilhado ou de produção. Essa verificação é uma instrução que Claude segue e é separada dos prompts de permissão.

Mudar para o estilo Proactive não altera seu [modo de permissão](/docs/pt/permission-modes). Seu modo de permissão ainda decide quais chamadas de ferramenta são executadas sem perguntar a você, então os prompts de permissão aparecem da mesma forma que antes de você mudar.

<h3 id="concise">
  Concise
</h3>

No estilo Concise, a primeira frase de uma resposta afirma o que aconteceu ou qual é a resposta. Claude omite a introdução, a narração passo a passo e o recapitulação de fechamento, e responde uma pergunta simples em uma a três frases. Ele faz o trabalho de engenharia tão completamente quanto no estilo Default. Requer Claude Code v2.1.237 ou posterior.

Claude ainda escreve em comprimento total nestes casos:

* **Qualquer coisa que você pedir**: quando você pedir uma explicação ou mais detalhes, Claude responde completamente.
* **Qualquer coisa que você precisa para agir com segurança**: relatórios de erro, saída de teste falhando, avisos de segurança e confirmações para ações destrutivas mantêm seu conteúdo completo.

<h3 id="explanatory">
  Explanatory
</h3>

No estilo Explanatory, Claude faz a tarefa da forma como faz no estilo Default e adiciona explicações curtas sobre por que fez as escolhas que fez. Cada explicação aparece na conversa, antes ou depois do código sobre o qual se trata, em um bloco rotulado `Insight`. As explicações não são escritas em seus arquivos como comentários.

Um bloco `Insight` carrega dois ou três pontos sobre sua base de código ou o código que Claude escreveu, como este após adicionar um endpoint de API:

```text theme={null}
★ Insight ─────────────────────────────────────
- Every route in this repo goes through the withAuth wrapper, so the new endpoint gets session checks without its own middleware.
- Rate limits are set per route in limits.ts, which is why this change adds an entry there rather than a global default.
─────────────────────────────────────────────────
```

<h3 id="learning">
  Learning
</h3>

No estilo Learning, Claude adiciona os mesmos blocos `Insight` que o [estilo Explanatory](#explanatory) e também pede que você escreva parte do código. Claude lida com a implementação rotineira em si. Quando chega a uma peça com uma decisão de design real, como tratamento de erro, uma estrutura de dados ou lógica de negócios com mais de uma abordagem válida, ele deixa algumas linhas para você.

Claude marca o local com um comentário `TODO(human)` no arquivo, depois envia uma solicitação que diz o que já foi construído, o que escrever e o que pesar:

```text theme={null}
● Learn by Doing

Context: The upload form is in place and calls validateFile() before accepting a file. Size and type checks work for images, but the switch statement has no handling for documents yet.

Your Task: In upload.js, implement the case "document" branch inside validateFile(). Look for TODO(human).

Guidance: Decide on a size limit for documents and whether the file extension has to match the MIME type. Return {valid: boolean, error?: string}.
```

Claude então para e espera. Escreva seu código no comentário `TODO(human)` e diga ao Claude quando terminar. Claude responde com um `Insight` sobre seu código e continua a tarefa.

<h2 id="change-your-output-style">
  Altere seu estilo de saída
</h2>

Escolha um estilo com o comando, um menu ou um arquivo de configurações. O comando e ambos os menus salvam sua escolha em `.claude/settings.local.json` no [nível do projeto local](/docs/pt/settings).

* **Comando `/output-style`**: execute `/output-style <style>` para alternar, por exemplo `/output-style concise`. Sem argumentos, o comando lista os estilos que você pode escolher e marca o atual.

  O comando também funciona em [modo não interativo](/docs/pt/headless) e sessões do Agent SDK, e do aplicativo móvel ou web via [Controle Remoto](/docs/pt/remote-control#limitations), onde você pode listar e selecionar apenas [estilos integrados](#built-in-output-styles). Requer Claude Code v2.1.269 ou posterior.
* **Menu do Terminal**: execute `/config` e selecione **Output style** para escolher um estilo de um menu.
* **Extensão VS Code**: abra o [menu de comandos](/docs/pt/vs-code#use-the-prompt-box) com `/` e selecione **Output styles** para escolher um estilo, incluindo seus estilos personalizados. Requer Claude Code v2.1.257 ou posterior.
* **Aplicativo Desktop**: defina o campo `outputStyle` em um arquivo de configurações, por exemplo `.claude/settings.local.json`, o arquivo que o menu do terminal escreve. Quando você executa `/config` lá, Claude Code [abre **Settings > Claude Code**](/docs/pt/desktop#what%E2%80%99s-not-available-in-desktop) em vez de um menu.

Para definir um estilo sem o menu, edite o campo `outputStyle` diretamente em um arquivo de configurações:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

O valor é sensível a maiúsculas e minúsculas, portanto escreva os nomes integrados como `Proactive`, `Concise`, `Explanatory` e `Learning`. Um valor que não corresponde exatamente a um nome de estilo, como `explanatory`, oferece o estilo Padrão. O comando `/output-style` ignora maiúsculas e minúsculas.

Para tornar um estilo seu padrão em todos os projetos, defina `outputStyle` em `~/.claude/settings.json`. Os arquivos de configurações próprios de um projeto [têm precedência](/docs/pt/settings#settings-precedence) sobre esse valor.

Quando você alterna estilos no meio da sessão, Claude usa o novo estilo a partir da sua próxima mensagem. Para o custo dessa primeira mensagem em cache de prompt, consulte [Alterando estilo de saída](/docs/pt/prompt-caching#changing-output-style). Antes da v2.1.251, o novo estilo era aplicado apenas após você executar `/clear` ou iniciar uma nova sessão.

<h2 id="create-a-custom-output-style">
  Crie um estilo de saída personalizado
</h2>

Um estilo de saída personalizado é um arquivo Markdown: frontmatter para metadados, depois as instruções para Claude.

Na extensão VS Code, você também pode criar o arquivo a partir do [menu **Output styles**](/docs/pt/vs-code#use-the-prompt-box) em vez de escrevê-lo manualmente. Isso requer Claude Code v2.1.261 ou posterior.

<Steps>
  <Step title="Crie um arquivo Markdown">
    Salve-o em um de três níveis. O nome do arquivo se torna o nome do estilo, a menos que você defina `name` no frontmatter.

    * Usuário: `~/.claude/output-styles`
    * Projeto: `.claude/output-styles`
    * Política gerenciada: `.claude/output-styles` dentro do [diretório de configurações gerenciadas](/docs/pt/managed-settings#delivery-mechanisms)

    Os estilos de saída do projeto são carregados de cada `.claude/output-styles/` entre o diretório de trabalho e a raiz do repositório. Quando mais de um desses diretórios aninhados define um estilo com o mesmo nome, Claude Code usa o mais próximo do diretório de trabalho.
  </Step>

  <Step title="Adicione frontmatter e instruções">
    Decida se deseja manter as instruções de engenharia de software do Claude Code. Defina `keep-coding-instructions: true` se você está mudando como Claude se comunica, mas ainda quer que ele codifique da mesma forma. Deixe de fora se Claude não estará fazendo engenharia de software.

    Este exemplo lidera cada explicação com um diagrama enquanto mantém o comportamento de codificação do Claude:

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="Mude para seu estilo">
    Execute `/output-style <style>` no terminal, ou execute `/config` e selecione seu estilo em **Output style**. Claude usa o novo estilo a partir da sua próxima mensagem. No terminal, Claude Code lê arquivos de estilo quando inicia, portanto, se você criar ou editar um durante uma sessão em execução, reinicie Claude Code para aplicar a alteração.
  </Step>
</Steps>

[Plugins](/docs/pt/plugins/manifest-reference) também podem enviar estilos de saída em um diretório `output-styles/`.

<h3 id="frontmatter">
  Referência de frontmatter
</h3>

Configure um estilo de saída com [frontmatter](/docs/pt/glossary#frontmatter) YAML entre marcadores `---` no topo do arquivo. Todos os campos são opcionais, e os nomes dos campos usam palavras minúsculas separadas por hífens. Um campo digitado incorretamente é ignorado sem um erro. Se o YAML não for analisado, o estilo ainda será carregado com seu nome de arquivo sem campos definidos; execute `claude --debug` para ver o erro de análise.

| Campo                      | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                              |
| :------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Não         | Nome do estilo de saída, mostrado no seletor `/config`. Padrão: o nome do arquivo                                                                                                                                                                                                                                                      |
| `description`              | Não         | Descrição do estilo de saída, mostrada no seletor `/config`                                                                                                                                                                                                                                                                            |
| `keep-coding-instructions` | Não         | Defina como `true` para manter as instruções integradas de engenharia de software do Claude Code junto com seu estilo. Padrão: `false`                                                                                                                                                                                                 |
| `force-for-plugin`         | Não         | Apenas estilos de saída de plugin. Defina como `true` para aplicar este estilo automaticamente sempre que o plugin estiver habilitado, sem exigir que os usuários o selecionem. Substitui a configuração `outputStyle` do usuário. Se vários plugins habilitados definirem isso, Claude Code usa o primeiro carregado. Padrão: `false` |

<span id="comparisons-to-related-features" />

<h2 id="choose-between-an-output-style-and-other-features">
  Escolha entre um estilo de saída e outros recursos
</h2>

Um estilo de saída se aplica a cada resposta em uma sessão. É uma instrução que Claude segue, portanto nada a impõe. Quando o que você quer é mais restrito do que cada resposta, ou precisa acontecer sem falha, outro recurso se encaixa melhor.

Esta tabela corresponde o que você quer ao recurso que faz isso:

| O que você quer                                                                                              | Use                                                               | Por que se encaixa                                                                                                           |
| :----------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| Cada resposta em uma certa voz, comprimento ou formato, ou Claude em um papel diferente                      | Um estilo de saída                                                | Ele se aplica a toda a sessão, e você muda de estilos com um comando                                                         |
| Claude conhecer as convenções, comandos e estrutura do seu projeto                                           | [CLAUDE.md](/docs/pt/memory)                                           | Ele contém o que Claude deve saber sobre a base de código, e permanece carregado qualquer que seja o estilo que você escolha |
| Instruções para um tipo de tarefa, como uma lista de verificação de lançamento ou um procedimento de revisão | Uma [skill](/docs/pt/skills)                                           | Claude a carrega apenas quando você a invoca ou a tarefa corresponde, portanto não molda respostas não relacionadas          |
| Algo que deve acontecer toda vez sem exceção, como formatação após cada edição ou bloqueio de um comando     | Um [hook](/docs/pt/hooks-guide)                                        | Claude Code executa um hook em um evento do ciclo de vida, portanto não depende de Claude seguir uma instrução               |
| Um assistente com suas próprias instruções, modelo e ferramentas para uma tarefa focada                      | Um [subagent](/docs/pt/sub-agents)                                     | Ele é executado em um contexto separado com seu próprio prompt do sistema e retorna um resumo para sua conversa              |
| Uma adição às instruções do Claude que você passa quando inicia Claude Code                                  | [`--append-system-prompt`](/docs/pt/cli-reference#system-prompt-flags) | Ele acrescenta ao prompt do sistema sem remover nada                                                                         |

Esses recursos se combinam. Por exemplo, você pode usar CLAUDE.md para o que Claude deve saber, um estilo de saída para como ele responde, e um hook para qualquer coisa que tenha que ser garantida. [Estenda Claude Code](/docs/pt/features-overview) compara o restante dos recursos de extensão.

<h2 id="how-output-styles-work">
  Como os estilos de saída funcionam
</h2>

Um estilo de saída altera as instruções que Claude Code fornece ao Claude.

* Claude Code envia as instruções do estilo ativo com cada solicitação.
* Os estilos de saída personalizados omitem as instruções de engenharia de software integradas do Claude Code, como como definir o escopo das alterações, escrever comentários e verificar o trabalho, a menos que `keep-coding-instructions` seja definido como `true`.

Os estilos de saída se aplicam à conversa principal e a um [fork](/docs/pt/sub-agents#fork-the-current-conversation), que herda a conversa completa e o prompt do sistema do pai. Outros [subagentes executam seu próprio prompt do sistema](/docs/pt/sub-agents#what-loads-at-startup), portanto os estilos não alteram como eles respondem.

O uso de tokens depende do estilo. As instruções de um estilo adicionam tokens de entrada, embora o cache de prompt reduza esse custo após a primeira solicitação em uma sessão.

Os estilos Explanatory e Learning integrados produzem respostas mais longas do que Default por design, o que aumenta os tokens de saída. O estilo Concise faz o oposto ao instruir Claude a manter as respostas curtas por padrão. Para estilos personalizados, o uso de tokens de saída depende do que suas instruções dizem ao Claude para produzir.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Settings](/docs/pt/settings): onde o campo `outputStyle` reside e como a precedência de configurações funciona
* [Permission modes](/docs/pt/permission-modes): como o estilo Proactive se compara ao modo automático
* [Plugins](/docs/pt/plugins/overview): empacote e distribua estilos de saída junto com skills, hooks e agents
* [Debug your configuration](/docs/pt/debug-your-config): diagnostique por que um estilo de saída não está entrando em vigor
