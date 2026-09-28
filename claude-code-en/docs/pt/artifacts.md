> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Compartilhar saída de sessão como artifacts

> Artifacts transformam o trabalho do Claude Code em páginas ao vivo e interativas no claude.ai que você pode manter privadas, compartilhar com sua organização ou publicar em um link público.

<Note>
  Artifacts estão disponíveis nos planos Pro, Max, Team e Enterprise e exigem uma sessão conectada com [`/login`](/docs/pt/setup#authenticate). Consulte [Disponibilidade](#availability) para o conjunto completo de requisitos.
</Note>

Um [artifact](https://claude.com/features/artifacts) é uma página web ao vivo e interativa que Claude Code publica de sua sessão para uma URL privada no claude.ai. Você a abre em um navegador e ela é atualizada no local conforme a sessão continua. Compartilhe-a a partir do cabeçalho da página quando quiser que outra pessoa a veja também.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Um artifact aberto em um navegador em claude.ai/code/artifact. O cabeçalho do visualizador mostra o título do artifact acme-funnel-fix, um botão Compartilhar e o avatar do autor. O menu Compartilhar está aberto com a alternância Sempre compartilhar a versão mais recente, um seletor de versão lendo Compartilhando versão 2, um seletor de público Todos na Acme e um botão Copiar link. Abaixo do cabeçalho, a página do artifact mostra dois mockups de dispositivos móveis lado a lado, um gráfico de funil e uma linha de cartões de métricas." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Quando usar um artifact
</h2>

Use um artifact quando o texto do terminal é o meio errado para o que Claude produziu: saída que é mais fácil de visualizar e interagir do que ler linha por linha. Claude constrói a página a partir de qualquer coisa que sua sessão possa alcançar, incluindo sua base de código e dados que ela extrai através de suas [ferramentas conectadas](/docs/pt/mcp), portanto a página pode mostrar coisas que levariam parágrafos para descrever. Por exemplo, peça a Claude para:

* Guiar um revisor através de um pull request com diffs anotados
* Renderizar um dashboard a partir de dados que a sessão já extraiu
* Dispor várias opções de design ou implementação lado a lado
* Manter uma linha do tempo de investigação que se preenche enquanto uma tarefa longa é executada
* Enviar a um colega um link em vez de colar saída no Slack
* Publicar um quadro de status que [extrai dados atualizados através de conectores MCP](#pull-live-data-with-mcp-connectors) cada vez que alguém o abre

Veja [O que você pode construir](#what-you-can-build) para prompts que correspondem a estes, e [Extrair dados ao vivo com conectores MCP](#pull-live-data-with-mcp-connectors) para o prompt do quadro apoiado por conectores.

<h3 id="what-an-artifact-is-not">
  O que um artifact não é
</h3>

Um artifact é uma captura de trabalho: uma página única e autossuficiente sem backend, portanto não pode servir múltiplas rotas. Para uma ferramenta interna hospedada com um backend, implante-a em sua própria infraestrutura. Veja [Restrições de página](#page-constraints) para o conjunto completo de limites.

<h2 id="create-an-artifact">
  Criar um artefato
</h2>

Claude pode publicar um artefato por conta própria quando a saída é adequada para uma página, ou você pode solicitar um diretamente. Para solicitar, nomeie o recurso ou descreva a saída visual que deseja em linguagem simples. Um bom candidato é qualquer coisa mais fácil de ver do que de ler como texto, como um diff anotado, um gráfico ou um conjunto de opções para comparar. Os prompts abaixo são dois exemplos; consulte [O que você pode construir](#what-you-can-build) para mais padrões.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

A menos que você nomeie um local, Claude escreve a página em um arquivo HTML ou Markdown em um diretório temporário fora do seu projeto, então a publica. Publicar um novo artefato passa pelo [modo de permissão](/docs/pt/permission-modes) da sua sessão:

* **Modo Auto**: o classificador revisa a publicação em vez de solicitar a você, para que Claude possa publicar uma página sem você ver um prompt. Qual modo suas sessões começam depende do seu plano; consulte [o modo de permissão inicial](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode).
* **Modos Manual e Aceitar edições**: Claude Code solicita permissão; pode dizer algo como `Claude wants to publish deploy-failures.html, uploading it to claude.ai (Anthropic's servers) to host as the page "Deploy failures by service", private to you until you share it`. Selecione **Sim** para publicar.

Depois que você aprova um artefato uma vez, Claude Code o republica sem perguntar, e pergunta novamente em alguns casos, incluindo quando:

* Claude declara uma capacidade de tempo de execução para a página, como [chamadas de conector](#pull-live-data-with-mcp-connectors) ou [downloads de arquivo](#offer-a-file-download)
* Você [compartilhou publicamente](#share-an-artifact)
* Você compartilhou com pessoas específicas ou sua organização com a versão mais recente escolhida como a versão que os visualizadores veem

Após a primeira publicação, Claude imprime a URL e seu navegador abre para a nova página. Se você enviou o prompt através de [Remote Control](/docs/pt/remote-control) do claude.ai, Claude Desktop ou do aplicativo móvel Claude, nenhuma aba abre na máquina que executa a sessão. O navegador abre lá na próxima vez que Claude publicar o artefato a partir de um prompt que você digita no terminal. Pressione `Ctrl+]` a qualquer momento para reabrir o artefato mais recente da sessão.

Claude escolhe o título do artefato e um emoji, e ambos aparecem em sua [galeria de artefatos](#share-an-artifact) no claude.ai e em links compartilhados. Claude também pode escolher um ícone de aba do navegador que corresponda ao que a página é, como um gráfico ou um calendário. Peça a Claude por um título, emoji ou ícone de aba específico se desejar um.

Para impedir que o navegador abra automaticamente quando um novo artefato é publicado, defina `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` em seu ambiente.

Se Claude responder que não pode publicar, ou escrever um arquivo HTML local sem um link, a ferramenta não está habilitada para sua sessão. Verifique os requisitos de [Disponibilidade](#availability).

<h2 id="update-an-artifact">
  Atualizar um artefato
</h2>

Peça ao Claude para revisar a página, ou deixe uma tarefa de longa duração republicar conforme faz progresso. Claude edita o arquivo subjacente e publica novamente para a mesma URL.

```text wrap theme={null}
Adicione um detalhamento por região abaixo do gráfico de resumo e republique.
```

Qualquer pessoa com a página aberta vê a atualização no lugar. Cada publicação se torna uma versão, e a partir do controle **Share** no cabeçalho da página você pode escolher qual versão os visualizadores veem.

Para atualizar um artefato de uma sessão diferente, forneça ao Claude sua URL, ou anexe-o com [`/artifacts`](#find-an-artifact-again). Sem nenhum dos dois, uma nova sessão cria um novo artefato em vez de atualizar um.

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="find-an-artifact-again">
  Encontre um artefato novamente
</h2>

Execute `/artifacts` no Claude Code para listar todos os artefatos que você possui e todos os artefatos compartilhados com você. Selecione um e pressione `o` para abri-lo no seu navegador ou `c` para copiar seu link. Pressione `Enter` para anexá-lo à sessão atual; antes da v2.1.216, `Enter` o abria no seu navegador. Claude Code lê a lista da sua conta claude.ai, portanto funciona em uma nova sessão e após `/clear`, quando o link saiu do terminal. Requer Claude Code v2.1.208 ou posterior.

<h2 id="share-an-artifact">
  Compartilhar um artefato
</h2>

Um novo artefato é visível apenas para você. Para compartilhá-lo, abra o artefato no seu navegador e use o controle **Compartilhar** no cabeçalho da página. O cabeçalho também vincula à sua galeria em [claude.ai/code/artifacts](https://claude.ai/code/artifacts), que lista todos os artefatos que você criou.

Os visualizadores em sua organização podem ver quem publicou a página: em um artefato compartilhado dentro de sua organização, seu nome está no menu de título, e em um artefato público está no cabeçalho da página para visualizadores conectados em sua organização. Um visualizador que abre um link público sem se conectar, ou de fora de sua organização, vê o rótulo `O conteúdo é gerado pelo usuário e não verificado.` em vez de seu nome.

Quem você pode compartilhar depende do seu plano:

* **Dentro de sua organização**: nos planos Team e Enterprise, conceda acesso a pessoas específicas em sua organização, ou a todos nela. Os visualizadores se conectam ao claude.ai como membros de sua organização para ver a página.
* **Publicamente**: compartilhe um link que qualquer pessoa na internet possa abrir, sem necessidade de conexão ao claude.ai. Nos planos Pro e Max, um link público é a única maneira de compartilhar um artefato. Nos planos Team e Enterprise, o compartilhamento público está desativado até que um Proprietário [o ative para a organização](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Deixar alguém editar com você
</h3>

As pessoas com quem você compartilha são visualizadores por padrão: elas veem cada versão que você publica, mas não podem alterar a página. Nos planos Team e Enterprise, você também pode tornar alguém um editor. Na caixa de diálogo de compartilhamento, adicione uma pessoa e mude sua função de **visualizador** para **editor**.

Um editor publica novas versões da mesma forma que você [atualiza o artefato de outra sessão](#update-an-artifact): ele fornece ao Claude a URL do artefato, ou o anexa de [`/artifacts`](#find-an-artifact-again), e Claude extrai o conteúdo atual e republica com suas alterações. Todos com a página aberta veem cada atualização ao vivo.

<h2 id="read-an-artifact-shared-with-you">
  Ler um artefato compartilhado com você
</h2>

Quando alguém compartilha um artefato com você, você pode fazer com que Claude o leia: forneça a Claude sua URL ou anexe-o de [`/artifacts`](#find-an-artifact-again).

Claude lê uma página que outra pessoa escreveu da mesma forma que lê uma página da web com [WebFetch](/docs/pt/tools-reference#webfetch-tool-behavior): ele obtém um resumo do que foi solicitado em vez da página bruta, e o resumo relata instruções escritas na página em vez de transmiti-las. Claude Code também salva o código-fonte completo da página em um arquivo local, que Claude pode abrir quando precisa do conteúdo exato, como para republicar o artefato como um [editor](#let-someone-edit-with-you).

<h2 id="collect-comments-on-an-artifact">
  Coletar comentários em um artefato
</h2>

Quando você compartilha um artefato dentro de sua organização, as pessoas com as quais você o compartilha podem deixar comentários na página, e você pode fazer com que Claude leia esses comentários e responda a eles. Você precisa do Claude Code v2.1.221 ou posterior e de um plano Team ou Enterprise, porque apenas um artefato que você [compartilha dentro de sua organização](#share-an-artifact) recebe comentários. Claude lê os comentários em dois casos:

* **Você pede a Claude para lê-los**: forneça a Claude a URL do artefato e peça pelos comentários. Claude lista cada thread e marca os comentários que alguém que pode editar o artefato enviou para ele.
* **Alguém que pode editar o artefato envia um comentário para Claude**: em uma thread na página, ele envia um comentário com **Send to Claude**, ou menciona `@claude` em um. De qualquer forma, ele ativa a thread.

Claude pode responder ou resolver apenas uma thread ativada. Outras threads permanecem abertas até que uma pessoa as resolva na página. Os visualizadores veem cada resposta atribuída a Claude, por meio de você.

Se você compartilhar um artefato publicamente, os visualizadores não poderão comentar nele: a página diz `Comments aren't available while this Artifact is shared publicly.` Para mudar um artefato que já tem threads de comentários para um link público, delete as threads primeiro.

Para pedir os comentários você mesmo, forneça a Claude a URL:

```text wrap theme={null}
Read the comments on https://claude.ai/code/artifact/5fbea6f3-... and make the changes the commenters ask for.
```

Se Claude disser que não consegue ler comentários, confirme sua versão, sua sessão e sua configuração de feature-flag:

* Você está executando Claude Code v2.1.221 ou posterior.
* Você não está em sua primeira sessão desde que instalou Claude Code ou atualizou de uma versão anterior à v2.1.221. Nessa [primeira sessão após uma instalação ou atualização](/docs/pt/env-vars#first-session-after-an-install-or-upgrade), Claude pode não conseguir ler comentários ainda; inicie uma nova sessão e peça novamente.
* Você não desativou a busca de feature-flag.

<h3 id="let-claude-reply-to-comments-on-its-own">
  Deixar Claude responder a comentários por conta própria
</h3>

Depois que sua sessão publica um artefato, Claude Code observa esse artefato em busca de comentários enquanto a sessão é executada. Quando alguém que pode editar o artefato envia um comentário para Claude, ele chega à sua sessão imediatamente, e Claude pode ler a thread e responder sem você pedir.

Você precisa do Claude Code v2.1.228 ou posterior. Se você desativou a [busca de feature-flag](/docs/pt/env-vars#features-that-need-feature-flag-fetching), Claude Code não observa comentários.

Seu [modo de permissão](/docs/pt/permission-modes) decide o que Claude faz quando um comentário enviado chega:

* **Claude responde por conta própria**: quando seu modo de permissão permite que Claude poste a resposta sem pedir a você, Claude lê a thread e responde, e edita o artefato quando o comentário pede uma alteração. Você vê `Auto-replied to comment thread on Artifact: <name>` ou `Auto-edited Artifact: <name> in response to a comment thread`.
* **Claude espera por você**: fora do plan mode, quando postar a resposta precisaria de sua aprovação, você vê `Comments are waiting on Artifact: <name>`. Claude então pede sua aprovação para ler a thread, e novamente para postar a resposta.
* **Claude pausa no plan mode**: você vê `Comments are waiting on Artifact: <name>`, e Claude não responde até você sair do plan mode e pedir a ele para ler e responder.

Claude também para de responder por conta própria a um artefato depois de lidar com 60 comentários enviados ou ativações de thread nesse artefato dentro de uma hora. Você vê `Comments are waiting on Artifact: <name>` uma vez, e Claude retoma conforme os comentários dessa hora envelhecem.

Execute `/tasks` para ver cada artefato que sua sessão está observando, listado como uma tarefa de atualizações ao vivo. Você pode impedir que Claude responda por conta própria de qualquer uma destas formas:

* **Pressione Ctrl+C uma vez em um prompt ocioso**: Claude pausa de responder em cada artefato que sua sessão está observando. As respostas começam novamente depois que você envia sua próxima mensagem.
* **Pare a tarefa em `/tasks`**: Claude para de responder nesse artefato até você pedir a ele para retomar as respostas lá. Publicar o artefato novamente não inicia as respostas novamente, e a parada ainda se aplica quando você retoma a sessão mais tarde.
* **Pressione `Ctrl+X Ctrl+K` duas vezes em 3 segundos**: o atalho que [para cada subagente de fundo em execução](/docs/pt/interactive-mode#general-controls) também para Claude de responder em cada artefato pelo resto da sessão. Pedir a Claude para retomar as respostas não desfaz essa parada.

Se o serviço que entrega comentários ficar indisponível ou parar de responder, Claude Code continua tentando se reconectar por um tempo, depois para de observar cada artefato que sua sessão estava observando.

<h2 id="pull-live-data-with-mcp-connectors">
  Extrair dados ao vivo com conectores MCP
</h2>

Um artefato pode chamar [conectores MCP](/docs/pt/mcp#use-mcp-servers-from-claude-ai) cada vez que alguém o visualiza, para que a página mostre dados atuais em vez de um instantâneo da sessão que a construiu. Chamadas de conectores de artefatos estão disponíveis nos planos Pro, Max, Team e Enterprise e exigem Claude Code v2.1.209 ou posterior. Em versões anteriores, Claude publica a página com os dados que a sessão coletou durante sua construção.

Para criar uma página com suporte de conectores, nomeie o conector e os dados que deseja em seu prompt:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude declara quais conectores a página pode chamar como parte da publicação, e a página não pode chamar conectores fora dessa declaração. Apenas conectores de sua conta claude.ai se qualificam: Claude os nomeia na declaração, e quando alguém visualiza a página, cada chamada [é executada através da própria conexão da conta de visualização](#how-connector-calls-work-for-viewers) para esse conector. Servidores MCP locais que você configura no Claude Code, como servidores de `.mcp.json`, podem fornecer dados enquanto Claude constrói a página, mas a página publicada não pode chamá-los.

A página busca dados quando carrega e pode atualizar em um intervalo ou quando um visualizador usa um controle de atualização na página. As respostas são armazenadas em cache no navegador do visualizador, para que uma página reabierta seja renderizada a partir das respostas em cache imediatamente e depois seja atualizada com resultados frescos.

<h3 id="how-connector-calls-work-for-viewers">
  Como as chamadas de conectores funcionam para visualizadores
</h3>

Quando uma página publicada chama um conector, a chamada usa a conta da pessoa que está visualizando a página, não a conta da pessoa que a publicou:

* **Cada visualizador usa seus próprios conectores**: as chamadas passam pelas ferramentas conectadas da conta de visualização, para que duas pessoas que abram o mesmo painel possam ver dados diferentes dependendo do que suas contas podem acessar. A página nunca vê as credenciais de ninguém; claude.ai faz as chamadas em nome da página.
* **Os visualizadores aprovam o acesso primeiro**: claude.ai pede permissão a cada visualizador antes da primeira chamada de conector da página. Um visualizador que recusa, ou que não conectou um conector que a página usa, ainda vê a página sem suas seções ao vivo.
* **As ações também usam a conta do visualizador**: uma página pode oferecer controles que invocam ferramentas de conectores com efeitos colaterais, como postar uma mensagem ou atualizar um problema. A ação passa pela conta de quem seleciona o controle.

Quando você planeja compartilhar uma página com suporte de conectores, peça a Claude para incluir uma mensagem de fallback em cada seção ao vivo que nomeie o conector que ela precisa. Um visualizador que não tem a conexão vê o que conectar em vez de uma seção vazia.

Um artefato que chama conectores não pode ser compartilhado para um link público em nenhum plano. Nos planos Team e Enterprise, você pode mantê-lo privado ou [compartilhá-lo dentro de sua organização](#share-an-artifact). Nos planos Pro e Max, onde um link público é a única maneira de compartilhar, um artefato com suporte de conectores permanece privado para você.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  A página não mostra dados ao vivo para um visualizador
</h3>

Quando uma página com suporte de conectores é renderizada mas suas seções ao vivo permanecem vazias para alguém com quem você a compartilhou, trabalhe através dessas causas:

* **O visualizador não conectou o conector**: conectores são por conta, então cada visualizador precisa de sua própria conexão para cada conector que a página chama. Ele pode adicionar um em **Settings > Connectors** em claude.ai e depois recarregar a página.
* **O visualizador recusou a solicitação de permissão**: uma recusa dura pelo resto desse carregamento de página. Recarregar a página traz a solicitação de permissão de volta.
* **As chamadas de conectores estão desativadas para a organização**: um Proprietário controla o [toggle **Enable artifact connectors**](#control-connector-calls-from-artifacts) nas configurações de administrador.
* **A página chama nomes de ferramentas que o conector não expõe**: as seções afetadas permanecem vazias para todos, incluindo você. Isso pode acontecer quando uma página nomeia as ferramentas individuais atrás de um conector estilo gateway que expõe apenas algumas de suas próprias ferramentas. Peça a Claude para corrigir os nomes das ferramentas que a página chama e publicá-la novamente.

  Quando Claude publica a página e as ferramentas desse conector estão disponíveis em sua sessão, Claude Code verifica os nomes das ferramentas que a página declara contra elas, avisa Claude sobre nomes que não correspondem e recusa a publicação quando nenhum corresponde. Antes da v2.1.265, ela publicava a página sem verificá-los.

<h2 id="offer-a-file-download">
  Ofereça um download de arquivo
</h2>

Um artefato pode oferecer aos visualizadores um arquivo que a página gera, como uma exportação CSV de uma tabela ou um PNG de um gráfico. O visualizador o salva através de um controle de download na página, como um botão. Downloads de arquivo são uma capacidade de tempo de execução que claude.ai ativa por conta, portanto Claude verifica se sua conta possui essa capacidade antes de construir o controle.

Os visualizadores não podem salvar um arquivo de um link de download comum ou de um script na página, porque o visualizador de artefatos em claude.ai bloqueia qualquer download que a página inicia por si mesma, incluindo links para URLs `data:` ou `blob:`. Se uma página tiver botões de download construídos dessa forma, peça a Claude para reconstruí-los com a capacidade de downloads.

Para oferecer um arquivo, solicite o controle e o formato do arquivo em seu prompt:

```text wrap theme={null}
Add a button that downloads this table as a CSV file.
```

Claude declara a capacidade de downloads como parte da publicação, da mesma forma que [declara conectores](#pull-live-data-with-mcp-connectors).

<h2 id="what-you-can-build">
  O que você pode construir
</h2>

Um artefato é uma única página HTML, portanto qualquer coisa que você possa expressar em HTML, CSS e JavaScript inline está no escopo. Os padrões abaixo surgem com mais frequência.

<h3 id="walk-through-a-change">
  Percorrer uma mudança
</h3>

Peça uma página que renderize um diff ou uma mudança de design com anotações ao lado das linhas relevantes, para que os revisores possam ler seu raciocínio ao lado do código em vez de reconstruí-lo a partir de uma descrição.

```text wrap theme={null}
Make an artifact that walks through this PR. Render the diff with margin annotations and color-code findings by severity.
```

<h3 id="compare-alternatives">
  Comparar alternativas
</h3>

Peça várias variantes em uma página para que você possa avaliá-las uma contra a outra. Isso funciona para layouts, cópia, formas de API ou planos de implementação.

```text wrap theme={null}
Make an artifact with four distinctly different layouts for the settings panel. Vary density and grouping, and lay them out as a grid with a one-line tradeoff under each.
```

<h3 id="tune-with-interactive-controls">
  Ajustar com controles interativos
</h3>

Peça sliders, alternâncias ou campos de entrada vinculados ao que você está ajustando, para que você possa explorar valores diretamente em vez de descrevê-los.

```text wrap theme={null}
Build an artifact with sliders for the easing curve, duration, and delay so I can try values on this transition. Show the animation live as I move them.
```

<h3 id="bring-the-result-back-to-your-session">
  Trazer o resultado de volta para sua sessão
</h3>

Um artefato pode atuar como um editor leve para uma decisão que você então devolve a Claude. Peça um controle de exportação que produza texto que você possa colar no terminal, para que o resultado de interagir com a página flua de volta para a sessão em vez de permanecer na página.

```text wrap theme={null}
Make a triage board artifact with each open issue as a draggable card across Now, Next, Later, and Cut columns. Add a "Copy as prompt" button that gives me the final ordering to paste back here.
```

<h3 id="track-work-in-progress">
  Rastrear trabalho em progresso
</h3>

Peça a Claude para manter um artefato atualizado enquanto uma tarefa longa é executada, para que qualquer pessoa com o link possa acompanhar sem ler o terminal.

```text wrap theme={null}
Turn this migration plan into a checklist artifact. Check items off as you complete them and add a note for anything you skip.
```

<h2 id="improve-the-visual-design">
  Melhorar o design visual
</h2>

Claude aplica uma skill de design integrada quando constrói um artefato, portanto as páginas recebem uma paleta deliberada, tipografia e layout sem prompting extra. Essa skill também procura por um sistema de design existente em seu projeto antes de escolher o seu próprio. Design tokens são os valores nomeados de cor, tipografia e espaçamento que seu sistema de design reutiliza. Para manter os artefatos consistentes com a marca do seu produto, registre-os onde Claude possa encontrá-los, como o [CLAUDE.md](/docs/pt/memory) do projeto ou um arquivo de tema em seu repositório:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude trata seu sistema de design como tendo precedência maior do que suas próprias escolhas, e seu prompt como tendo precedência maior do que ambas. O título e o formato acima são um exemplo; qualquer lista clara de cores, fontes e espaçamento funciona.

Para tipografia, Claude pode carregar uma fonte do Google Fonts, a única fonte de fonte externa que uma página de artefato pode carregar. Claude incorpora qualquer outra fonte como um `@font-face` data URI e fornece a cada fonte uma pilha de fallback, portanto a página ainda é renderizada se uma fonte não carregar. Para usar uma fonte específica, nomeie-a em seu prompt ou em seu sistema de design.

<h2 id="draft-a-design-canvas">
  Rascunhar uma tela de design
</h2>

Para criar um protótipo de uma interface de usuário, um fluxo de tela, uma página de destino ou um pôster em vez de construir uma página, execute `/design` com um resumo. Claude rascunha o design como pranchetas em uma tela e publica a tela como um artefato de Design. O resumo nomeia o que você deseja desenhar:

```text wrap theme={null}
/design a settings screen for a mobile banking app
```

Abra o artefato publicado em um navegador de desktop para revisar as pranchetas. Selecione um elemento em uma prancheta e altere-o, e suas edições são salvas automaticamente. Você pode exportar cada prancheta como PNG ou PDF.

`/design` requer uma sessão onde [artefatos estão disponíveis](#availability) e Claude Code v2.1.265 ou posterior.

<h2 id="page-constraints">
  Restrições de página
</h2>

Cada artefato é uma página independente e autocontida. Claude Code envolve o arquivo que você publica em um shell de documento HTML e o serve sob uma Política de Segurança de Conteúdo (CSP) rigorosa, que molda o que a página pode fazer.

| Restrição                  | Efeito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Solicitações externas      | A página pode carregar fontes tipográficas do Google Fonts e scripts de [cinco hosts CDN públicos](#allowlist-the-viewer-domain): cdnjs, unpkg, os CDNs do Tailwind e jQuery, e caminhos selecionados no jsDelivr, como `/npm/`. A CSP bloqueia todas as imagens externas e todos os outros scripts, folhas de estilo e fontes externas, e permite que chamadas `fetch`, XHR e WebSocket alcancem apenas a origem da própria página e os hosts do Google Fonts. Claude, portanto, carrega qualquer biblioteca que a página necessite de um desses CDNs, incorpora todos os outros CSS e JavaScript, e incorpora imagens como data URIs. [Chamadas do Connector](#pull-live-data-with-mcp-connectors) passam por claude.ai, que faz a chamada de rede em si. |
| Sem backend                | Um artefato é uma página estática. Ele não pode autenticar visualizadores por si só.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Downloads                  | A página não pode iniciar um download por si só. Para permitir que visualizadores salvem um arquivo que a página gera, Claude declara a capacidade de downloads. Consulte [Oferecer um download de arquivo](#offer-a-file-download).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Página única               | Links relativos não são resolvidos, porque nada é implantado junto com a página. Para conteúdo com múltiplas seções, Claude usa âncoras na página em vez de arquivos separados.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Tipos de arquivo de origem | O arquivo publicado deve ser `.html`, `.htm` ou `.md`, e deve ser decodificado como UTF-8, ou como UTF-16 little-endian pela sua marca de ordem de bytes. Arquivos Markdown são renderizados como páginas de documento estilizadas com código com destaque de sintaxe. Um arquivo que não é decodificado, ou que contém o caractere de substituição `U+FFFD`, é [recusado com a linha e coluna a corrigir](/docs/pt/errors#the-source-file-is-not-valid-utf-8-text).                                                                                                                                                                                                                                                                                             |
| Tamanho renderizado        | A página renderizada deve ter 16 MiB ou menos. Imagens incorporadas grandes são a causa usual quando uma publicação falha por tamanho.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

Gerar um artefato usa tokens de saída como qualquer outra resposta, e uma página estilizada é mais intensiva em tokens do que o mesmo conteúdo como texto de terminal. CSS incorporado, JavaScript para controles interativos e especialmente imagens incorporadas como data URIs são os principais contribuintes. Para reduzir o custo de tokens de um artefato:

* Prefira SVG, ou HTML e CSS, para diagramas em vez de imagens raster incorporadas
* Omita interatividade que você não necessite
* Faça a página resumir grandes conjuntos de dados em vez de incorporá-los completamente

<h2 id="availability">
  Disponibilidade
</h2>

Artefatos exigem todas as condições abaixo. Quando uma não é atendida, Claude escreve um arquivo HTML local ou diz que não pode publicar.

| Requisito               | Disponível quando                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Plano                   | Pro, Max, Team ou Enterprise. Em planos Pro e Max, artefatos são privados para você até que você os compartilhe, e nenhuma gestão de admin se aplica. Em planos Team, artefatos estão ativados por padrão. Em planos Enterprise, um Owner [os habilita](#manage-artifacts-for-your-organization) nas configurações de admin do claude.ai.                                                                                                   |
| Autenticação            | A sessão é apoiada por uma conta claude.ai: faça login com `/login` na CLI ou aplicativo de desktop. Sessões Claude Tag são conectadas através da identidade do agente, portanto nenhuma etapa é necessária. Sessões usando uma chave de API, [token de gateway](/docs/pt/llm-gateway) ou credencial de provedor de nuvem não podem publicar.                                                                                                    |
| Provedor de modelo      | API Anthropic. Não disponível em [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) ou [Microsoft Foundry](/docs/pt/microsoft-foundry).                                                                                                                                                                                                                                                                 |
| Política da organização | Chaves de criptografia gerenciadas pelo cliente (CMEK), HIPAA e [Retenção Zero de Dados](/docs/pt/zero-data-retention) não estão habilitadas para a organização.                                                                                                                                                                                                                                                                                 |
| Superfície              | Claude Code CLI ou aplicativo de desktop Claude versão 1.13576.0 ou posterior. Sessões [Claude Tag](https://claude.com/docs/claude-tag/overview) também podem publicar artefatos quando Claude Tag e artefatos estão habilitados para a organização. Desativado por padrão em contextos [Agent SDK](/docs/pt/agent-sdk/overview), GitHub Action e MCP-server, e quando [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars) está definido. |

Se os artefatos são permitidos para sua organização vem da política da sua organização, que Claude Code carrega de `api.anthropic.com`. Quando Claude Code não consegue carregar a política, artefatos não estão disponíveis. Quando você pede um, Claude diz por quê.

Se um proxy, VPN ou filtro web estiver envolvido, peça ao seu administrador de TI para deixar `api.anthropic.com` passar. Claude Code continua tentando novamente em segundo plano, e artefatos ficam disponíveis assim que a política é carregada e os permite.

<h2 id="disable-artifacts">
  Desabilitar artefatos
</h2>

Para desativar artefatos para suas próprias sessões independentemente da configuração de sua organização, use qualquer um dos:

| Onde                                     | O que fazer                                                                                                                                     |
| :--------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| [`/config`](/docs/pt/commands)                | Desative a linha **Artifacts**, que escreve [`"enableArtifact": false`](/docs/pt/settings-reference#enableartifact) em suas configurações de usuário |
| [Arquivo de configurações](/docs/pt/settings) | Defina `"enableArtifact": false`. O `"disableArtifact": true` descontinuado também desativa artefatos                                           |
| [Variável de ambiente](/docs/pt/env-vars)     | Defina `CLAUDE_CODE_DISABLE_ARTIFACT=1`                                                                                                         |
| [Regra de permissão](/docs/pt/permissions)    | Adicione `Artifact` a `permissions.deny`                                                                                                        |

Depois que você desativar artefatos em um arquivo [`--settings`](/docs/pt/cli-reference#cli-flags) ou com `CLAUDE_CODE_DISABLE_ARTIFACT`, ou seu administrador os desativar em [configurações gerenciadas](/docs/pt/server-managed-settings), nenhum arquivo de configurações os ativa novamente. Antes da v2.1.242, um arquivo mais alto na [pilha de precedência](/docs/pt/settings#settings-precedence) poderia ativar artefatos novamente mesmo quando um arquivo de precedência mais baixa definisse `"enableArtifact": false`.

Você também pode definir `"enableArtifact": false` no `.claude/settings.json` ou `.claude/settings.local.json` de um projeto para desativar artefatos para sessões nesse projeto. Um `"enableArtifact": true` em qualquer um dos arquivos não os ativa novamente. Honrar a chave em configurações de projeto e local requer Claude Code v2.1.242 ou posterior.

Se você adicionar uma regra de negação ou solicitação de `WebFetch` sem uma parte `domain:`, ela não desativa artefatos nem bloqueia leituras de artefatos. Uma [regra `WebFetch(domain:claude.ai)` em `deny` ou `ask` se aplica a leituras de artefatos](/docs/pt/permissions#allow-or-deny-every-fetch).

<h2 id="manage-artifacts-for-your-organization">
  Gerenciar artefatos para sua organização
</h2>

Proprietários em planos Team e Enterprise controlam artefatos a partir das [configurações de admin do claude.ai](https://claude.ai/admin-settings/claude-code). O conteúdo do artefato é armazenado em infraestrutura operada pela Anthropic e é visível apenas para membros autenticados da organização de publicação, a menos que o artefato seja [compartilhado publicamente](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Habilitar ou desabilitar artefatos
</h3>

Para habilitar ou desabilitar artefatos para toda a organização, vá para [**Settings > Claude Code > Capabilities**](https://claude.ai/admin-settings/claude-code) e use a alternância **Artifacts**. Em planos Enterprise com controle de acesso baseado em função, você pode escopo adicional de artefatos para funções específicas: vá para [**Settings > Roles**](https://claude.ai/admin-settings/roles), edite uma função e defina a permissão **Artifacts** sob o grupo **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Controlar chamadas de conector a partir de artefatos
</h3>

[Chamadas de conector a partir de artefatos](#pull-live-data-with-mcp-connectors) têm sua própria alternância, separada da alternância **Artifacts** que ativa ou desativa artefatos. Vá para [**Settings > Capabilities**](https://claude.ai/admin-settings/capabilities) e use a alternância **Enable artifact connectors**. A mesma alternância governa chamadas de conector a partir de artefatos criados em conversas do claude.ai, razão pela qual fica sob **Settings > Capabilities** em vez de **Settings > Claude Code**.

<h3 id="control-public-sharing">
  Controlar compartilhamento público
</h3>

O compartilhamento público está desativado por padrão em planos Team e Enterprise, portanto os membros podem compartilhar artefatos apenas dentro da organização até que um Proprietário o ative. Para permitir que os membros publiquem artefatos em links públicos que qualquer pessoa possa visualizar sem fazer login, vá para **Settings > Claude Code > Capabilities** e ative **External sharing** sob a alternância **Artifacts**. Desativá-lo novamente bloqueia o acesso através de links públicos existentes sem alterar o público de cada artefato; o acesso é retomado se você reativá-lo.

<h3 id="set-a-retention-policy">
  Definir uma política de retenção
</h3>

Para definir quanto tempo os artefatos são mantidos antes da exclusão automática, vá para [**Settings > Data & privacy controls**](https://claude.ai/admin-settings/data-privacy-controls). Você pode definir períodos de retenção separados para artefatos que ainda são privados para seu autor e artefatos que foram compartilhados.

<h3 id="review-the-audit-log">
  Revisar o log de auditoria
</h3>

Publicar, compartilhar e excluir um artefato aparecem cada um no log de auditoria de sua organização sob os tipos de evento `claude_artifact_*`, a mesma família usada para artefatos criados em conversas do claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Adicionar o domínio do visualizador à lista de permissões
</h3>

O visualizador em claude.ai carrega cada artefato de uma origem `*.claudeusercontent.com` em sandbox. Se sua organização restringe o acesso à rede de saída, adicione esse domínio à sua lista de permissões junto com `claude.ai`. Consulte [Requisitos de acesso à rede](/docs/pt/network-config#network-access-requirements) para a lista completa.

Um artefato que carrega uma fonte tipográfica do [Google Fonts](#improve-the-visual-design) também solicita `fonts.googleapis.com` e `fonts.gstatic.com`. Ambos os hosts são opcionais. Se você bloqueá-los, os artefatos são renderizados em fontes tipográficas de fallback. Bloqueie com uma rejeição rápida em vez de um descarte silencioso para que a solicitação de fonte falhe imediatamente em vez de atrasar a primeira renderização da página.

Os artefatos também podem carregar bibliotecas JavaScript, como React ou um pacote de gráficos, de `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` e `unpkg.com`, e de nenhum outro host externo. Se você bloquear esses hosts, as partes de um artefato que dependem de uma biblioteca não funcionam, e diferentemente de uma fonte bloqueada, uma biblioteca bloqueada não tem fallback. Bloqueie com uma rejeição rápida aqui também, para que uma solicitação de biblioteca bloqueada falhe imediatamente em vez de ficar pendente até expirar.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  Listar e excluir artefatos com a API de Conformidade
</h3>

A [API de Conformidade](https://docs.claude.com/en/api/compliance) fornece endpoints para listar os artefatos de uma organização, recuperar o conteúdo de uma versão específica e excluir um artefato:

| Método   | Endpoint                                                            |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

Para os esquemas de solicitação e resposta, consulte a [referência da API de Conformidade](https://docs.claude.com/en/api/compliance/code/artifacts).

<h2 id="related-resources">
  Recursos relacionados
</h2>

* Procure [padrões de prompting e fluxos de trabalho](/docs/pt/prompt-library) que se emparelham com artefatos
* Transforme um prompt de artefato que você reutiliza em uma [skill](/docs/pt/skills) para que você possa invocá-lo como um comando
* [Conecte servidores MCP](/docs/pt/mcp) para que Claude possa extrair dados para um artefato enquanto ele constrói a página
