> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Compartilhar saída de sessão como artefatos

> Artefatos transformam o trabalho do Claude Code em páginas ao vivo e interativas no claude.ai que você pode manter privadas, compartilhar com sua organização ou publicar em um link público.

<Note>
  Artefatos estão disponíveis nos planos Pro, Max, Team e Enterprise e exigem uma sessão conectada com [`/login`](/docs/pt/setup#authenticate). Consulte [Disponibilidade](#availability) para o conjunto completo de requisitos.
</Note>

Um artefato é uma página da web ao vivo e interativa que Claude Code publica de sua sessão para uma URL privada no claude.ai. Você a abre em um navegador e ela é atualizada no local conforme a sessão continua. Compartilhe-a do cabeçalho da página quando quiser que alguém mais a veja também. Por exemplo, use um artefato para guiar um revisor através de uma solicitação de pull com diffs anotados, criar um painel a partir de dados de sessão ou manter uma linha do tempo de investigação que se preenche conforme Claude trabalha.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Um artefato aberto em um navegador em claude.ai/code/artifact. O cabeçalho do visualizador mostra o título do artefato acme-funnel-fix, um botão Compartilhar e o avatar do autor. O menu Compartilhar está aberto com a alternância Sempre compartilhar a versão mais recente, um seletor de versão lendo Compartilhando versão 2, um seletor de público Todos na Acme e um botão Copiar link. Abaixo do cabeçalho, a página do artefato mostra dois mockups de celular lado a lado, um gráfico de funil e uma linha de cartões de métrica." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Quando usar um artefato
</h2>

Use um artefato quando o texto do terminal é o meio errado para o que Claude produziu: saída que é mais fácil de visualizar e interagir do que ler linha por linha. Claude constrói a página a partir de qualquer coisa que sua sessão possa alcançar, incluindo sua base de código e dados que ela extrai através de suas [ferramentas conectadas](/docs/pt/mcp), para que a página possa mostrar coisas que levariam parágrafos para descrever. Por exemplo, peça a Claude para:

* Guiar um revisor através de uma solicitação de pull com diffs anotados
* Renderizar um painel a partir de dados que a sessão já extraiu
* Dispor várias opções de design ou implementação lado a lado
* Manter uma linha do tempo de investigação que se preenche enquanto uma tarefa longa é executada
* Enviar a um colega de trabalho um link em vez de colar a saída no Slack
* Publicar um quadro de status que [extrai dados atualizados através de conectores MCP](#pull-live-data-with-mcp-connectors) cada vez que alguém o abre

Consulte [O que você pode construir](#what-you-can-build) para prompts que correspondem a esses, e [Extrair dados ao vivo com conectores MCP](#pull-live-data-with-mcp-connectors) para o prompt do quadro com suporte de conectores.

<h3 id="what-an-artifact-is-not">
  O que um artefato não é
</h3>

Um artefato é uma captura de trabalho, não um aplicativo. É uma página única e autossuficiente sem backend, portanto não pode armazenar entrada de formulário ou servir múltiplas rotas, e seu único caminho para dados externos quando alguém o visualiza é [chamar conectores MCP](#pull-live-data-with-mcp-connectors). Para uma ferramenta interna hospedada com um backend, implante-a em sua própria infraestrutura. Consulte [Restrições de página](#page-constraints) para o conjunto completo de limites.

<h2 id="create-an-artifact">
  Criar um artefato
</h2>

Claude pode publicar um artefato por conta própria quando a saída se adequa a uma página, ou você pode pedir um diretamente. Para pedir, nomeie o recurso ou descreva a saída visual que você deseja em linguagem simples. Um bom candidato é qualquer coisa mais fácil de ver do que ler como texto, como um diff anotado, um gráfico ou um conjunto de opções para comparar. Os prompts abaixo são dois exemplos; consulte [O que você pode construir](#what-you-can-build) para mais padrões.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

Claude escreve a página em um arquivo HTML ou Markdown em seu projeto e depois a publica. Antes de publicar um novo artefato, Claude Code pede permissão; pode dizer algo como `Claude wants to publish "Deploy failures by service" (deploy-failures.html) to a private page on claude.ai`. Republicar um artefato que você já aprovou não solicita novamente.

Selecione **Sim** para publicar. Claude imprime a URL e seu navegador abre para a nova página. Pressione `Ctrl+]` a qualquer momento para reabrir o artefato mais recente do terminal.

Claude escolhe o título do artefato e um emoji para seu ícone de aba do navegador. Ambos aparecem em sua [galeria de artefatos](#share-an-artifact) em claude.ai e em links compartilhados, portanto peça a Claude para usar um título ou ícone específico se quiser um.

Para impedir que o navegador abra automaticamente quando um novo artefato é publicado, defina `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` em seu ambiente.

Se Claude responder que não pode publicar ou escrever um arquivo HTML local sem um link, a ferramenta não está habilitada para sua sessão. Verifique os requisitos de [Disponibilidade](#availability).

<h2 id="update-an-artifact">
  Atualizar um artefato
</h2>

Peça a Claude para revisar a página ou deixe uma tarefa de longa duração republicar conforme faz progresso. Claude edita o arquivo subjacente e publica novamente para a mesma URL.

```text wrap theme={null}
Add a per-region breakdown below the summary chart and republish.
```

Qualquer pessoa com a página aberta vê a atualização no local. Cada publicação se torna uma versão, e a partir do controle **Compartilhar** no cabeçalho da página você pode escolher qual versão os visualizadores veem.

Para atualizar um artefato de uma sessão diferente, dê a Claude a URL do artefato e peça para revisá-lo. Sem a URL, uma nova sessão sempre cria um novo artefato em vez de atualizar um existente.

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="share-an-artifact">
  Compartilhar um artefato
</h2>

Um novo artefato é visível apenas para você. Para compartilhá-lo, abra o artefato no seu navegador e use o controle **Compartilhar** no cabeçalho da página. O cabeçalho o identifica como o autor do artefato, portanto qualquer pessoa com quem você o compartilhar pode ver quem publicou a página. Ele também vincula à sua galeria em [claude.ai/code/artifacts](https://claude.ai/code/artifacts), que lista todos os artefatos que você criou.

Com quem você pode compartilhar depende do seu plano:

* **Dentro de sua organização**: nos planos Team e Enterprise, conceda acesso a pessoas específicas em sua organização ou a todos nela. Os visualizadores fazem login em claude.ai como membros de sua organização para ver a página.
* **Publicamente**: compartilhe um link que qualquer pessoa na internet possa abrir, sem necessidade de login em claude.ai. Nos planos Pro e Max, um link público é a única maneira de compartilhar um artefato. Nos planos Team e Enterprise, o compartilhamento público está desativado até que um Proprietário [o ative para a organização](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Deixar alguém editar com você
</h3>

As pessoas com quem você compartilha são visualizadores por padrão: elas veem cada versão que você publica, mas não podem alterar a página. Nos planos Team e Enterprise, você também pode tornar alguém um editor. Na caixa de diálogo de compartilhamento, adicione uma pessoa e mude sua função de **visualizador** para **editor**.

Um editor publica novas versões da mesma forma que você [atualiza o artefato de outra sessão](#update-an-artifact): ele fornece a URL do artefato a Claude em sua própria sessão, e Claude extrai o conteúdo atual e republica com suas alterações. Todos com a página aberta veem cada atualização em tempo real.

<h2 id="pull-live-data-with-mcp-connectors">
  Extrair dados ao vivo com conectores MCP
</h2>

Um artefato pode chamar [conectores MCP](/docs/pt/mcp#use-mcp-servers-from-claude-ai) cada vez que alguém o visualiza, para que a página mostre dados atuais em vez de um instantâneo da sessão que a construiu. Chamadas de conectores de artefatos estão disponíveis nos planos Pro, Max, Team e Enterprise e exigem Claude Code v2.1.209 ou posterior. Em versões anteriores, Claude publica a página com os dados que a sessão coletou durante sua construção.

Para criar uma página com suporte de conectores, nomeie o conector e os dados que deseja em seu prompt:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude declara quais conectores a página pode chamar como parte da publicação, e a página não pode chamar conectores fora dessa declaração. Apenas conectores de sua conta claude.ai se qualificam: Claude os nomeia na declaração, e quando alguém visualiza a página, cada chamada [é executada através da conexão da conta de visualização com esse conector](#how-connector-calls-work-for-viewers). Servidores MCP locais que você configura no Claude Code, como servidores de `.mcp.json`, podem fornecer dados enquanto Claude constrói a página, mas a página publicada não pode chamá-los.

A página busca dados quando carrega e pode atualizar em um intervalo ou quando um visualizador usa um controle de atualização na página. As respostas são armazenadas em cache no navegador do visualizador, para que uma página reabierta seja renderizada a partir das respostas em cache imediatamente e depois seja atualizada com resultados frescos.

<h3 id="how-connector-calls-work-for-viewers">
  Como as chamadas de conectores funcionam para visualizadores
</h3>

Quando uma página publicada chama um conector, a chamada usa a conta da pessoa que está visualizando a página, não a conta da pessoa que a publicou:

* **Cada visualizador usa seus próprios conectores**: as chamadas passam pelas ferramentas conectadas da conta de visualização, para que duas pessoas abrindo o mesmo painel possam ver dados diferentes dependendo do que suas contas podem acessar. A página nunca vê as credenciais de ninguém; claude.ai faz as chamadas em nome da página.
* **Visualizadores aprovam o acesso primeiro**: claude.ai pede permissão a cada visualizador antes da primeira chamada de conector da página. Um visualizador que recusa, ou que não conectou um conector que a página usa, ainda vê a página sem suas seções ao vivo.
* **Ações também usam a conta do visualizador**: uma página pode oferecer controles que invocam ferramentas de conectores com efeitos colaterais, como postar uma mensagem ou atualizar um problema. A ação passa pela conta de quem seleciona o controle.

Quando você planeja compartilhar uma página com suporte de conectores, peça ao Claude para incluir uma mensagem de fallback em cada seção ao vivo que nomeie o conector que ela precisa. Um visualizador que não tem a conexão então vê o que conectar em vez de uma seção vazia.

Um artefato que chama conectores não pode ser compartilhado para um link público em nenhum plano. Nos planos Team e Enterprise, você pode mantê-lo privado ou [compartilhá-lo dentro de sua organização](#share-an-artifact). Nos planos Pro e Max, onde um link público é a única maneira de compartilhar, um artefato com suporte de conectores permanece privado para você.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  A página não mostra dados ao vivo para um visualizador
</h3>

Quando uma página com suporte de conectores é renderizada mas suas seções ao vivo permanecem vazias para alguém com quem você a compartilhou, trabalhe através dessas causas:

* **O visualizador não conectou o conector**: conectores são por conta, então cada visualizador precisa de sua própria conexão com cada conector que a página chama. Eles podem adicionar um em **Settings > Connectors** em claude.ai e depois recarregar a página.
* **O visualizador recusou a solicitação de permissão**: uma recusa dura pelo resto desse carregamento de página. Recarregar a página traz a solicitação de permissão de volta.
* **As chamadas de conectores estão desativadas para a organização**: um Proprietário controla o [alternador **Enable artifact connectors**](#control-connector-calls-from-artifacts) nas configurações de administrador.

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

A partir do Claude Code v2.1.183, Claude aplica uma skill de design integrada quando constrói um artefato, portanto as páginas recebem uma paleta deliberada, tipografia e layout sem prompting extra. Essa skill também procura por um sistema de design existente em seu projeto antes de escolher o seu próprio. Para manter os artefatos consistentes com a marca do seu produto, registre seus tokens de design onde Claude possa encontrá-los, como o [CLAUDE.md](/docs/pt/memory) do projeto ou um arquivo de tema em seu repositório:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude trata seu sistema de design como tendo precedência mais alta do que suas próprias escolhas, e seu prompt como tendo precedência mais alta do que ambos. O cabeçalho e o formato acima são um exemplo; qualquer lista clara de cores, fontes e espaçamento funciona.

<h2 id="page-constraints">
  Restrições de página
</h2>

Cada artefato é uma página única e autossuficiente. Claude Code envolve o arquivo que você publica em um shell de documento HTML e o serve sob uma Política de Segurança de Conteúdo (CSP) rigorosa, que molda o que a página pode fazer.

| Restrição                  | Efeito                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sem solicitações externas  | O CSP bloqueia scripts, folhas de estilo, fontes e imagens carregadas de qualquer outro host, junto com chamadas `fetch`, XHR e WebSocket. Claude incorpora CSS e JavaScript e incorpora imagens como URIs de dados para que a página seja renderizada sem nenhuma solicitação externa. [As chamadas do Connector](#pull-live-data-with-mcp-connectors) são a exceção: a página as entrega ao claude.ai, que faz a chamada de rede em si. |
| Sem backend                | Um artefato é uma página estática. Não pode armazenar dados enviados através de um formulário ou autenticar visualizadores por si só. Sua única maneira de buscar dados quando alguém visualiza é [chamando conectores MCP](#pull-live-data-with-mcp-connectors), não uma API própria.                                                                                                                                                    |
| Página única               | Links relativos não são resolvidos, porque nada é implantado ao lado da página. Para conteúdo de múltiplas seções, Claude usa âncoras na página em vez de arquivos separados.                                                                                                                                                                                                                                                             |
| Tipos de arquivo de origem | O arquivo publicado deve ser `.html`, `.htm` ou `.md`. Arquivos Markdown são renderizados como HTML estilizado.                                                                                                                                                                                                                                                                                                                           |
| Tamanho renderizado        | A página renderizada deve ter 16 MiB ou menos. Imagens incorporadas grandes são a causa usual quando uma publicação falha por tamanho.                                                                                                                                                                                                                                                                                                    |

Gerar um artefato usa tokens de saída como qualquer outra resposta, e uma página estilizada é mais intensiva em tokens do que o mesmo conteúdo como texto de terminal. CSS inline, JavaScript para controles interativos e especialmente imagens incorporadas como URIs de dados são os principais contribuintes. Para reduzir o custo de token de um artefato:

* Prefira SVG ou HTML e CSS para diagramas em vez de imagens raster incorporadas
* Omita interatividade que você não precisa
* Faça a página resumir grandes conjuntos de dados em vez de incorporá-los completamente

<h2 id="availability">
  Disponibilidade
</h2>

Artefatos exigem todas as condições abaixo. Quando uma não é atendida, Claude escreve um arquivo HTML local ou diz que não pode publicar.

| Requisito               | Disponível quando                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plano                   | Pro, Max, Team ou Enterprise. Em planos Pro e Max, artefatos são privados para você até que você os compartilhe, e nenhuma gestão de admin se aplica. Em planos Team, artefatos estão ativados por padrão. Em planos Enterprise, um Owner [os habilita](#manage-artifacts-for-your-organization) nas configurações de admin do claude.ai.                                                                                                                                |
| Autenticação            | A sessão é apoiada por uma conta claude.ai: faça login com `/login` na CLI ou aplicativo de desktop. Sessões Claude Tag são conectadas através da identidade do agente, portanto nenhuma etapa é necessária. Sessões usando uma chave de API, [token de gateway](/docs/pt/llm-gateway) ou credencial de provedor de nuvem não podem publicar.                                                                                                                                 |
| Provedor de modelo      | API Anthropic. Não disponível em [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) ou [Microsoft Foundry](/docs/pt/microsoft-foundry).                                                                                                                                                                                                                                                                                              |
| Política da organização | Chaves de criptografia gerenciadas pelo cliente (CMEK), HIPAA e [Retenção Zero de Dados](/docs/pt/zero-data-retention) não estão habilitadas para a organização.                                                                                                                                                                                                                                                                                                              |
| Superfície              | Claude Code CLI versão 2.1.183 ou posterior, ou aplicativo de desktop Claude versão 1.13576.0 ou posterior. Sessões [Claude Tag](https://claude.com/docs/claude-tag/overview) também podem publicar artefatos quando Claude Tag e artefatos estão habilitados para a organização. Desativado por padrão em contextos [Agent SDK](/docs/pt/agent-sdk/overview), GitHub Action e MCP-server, e quando [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars) está definido. |

<h2 id="disable-artifacts">
  Desabilitar artefatos
</h2>

Para desativar artefatos para suas próprias sessões independentemente da configuração de sua organização, use qualquer um dos:

| Método                                   | Configuração                             |
| :--------------------------------------- | :--------------------------------------- |
| [Arquivo de configurações](/docs/pt/settings) | `"disableArtifact": true`                |
| [Variável de ambiente](/docs/pt/env-vars)     | `CLAUDE_CODE_DISABLE_ARTIFACT=1`         |
| [Regra de permissão](/docs/pt/permissions)    | Adicione `Artifact` a `permissions.deny` |

<h2 id="manage-artifacts-for-your-organization">
  Gerenciar artefatos para sua organização
</h2>

Proprietários em planos Team e Enterprise controlam artefatos a partir das [configurações de admin do claude.ai](https://claude.ai/admin-settings/claude-code). O conteúdo do artefato é armazenado em infraestrutura operada pela Anthropic e é visível apenas para membros autenticados da organização de publicação, a menos que o artefato seja [compartilhado publicamente](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Habilitar ou desabilitar artefatos
</h3>

Para habilitar ou desabilitar artefatos para toda a organização, vá para **Settings > Claude Code > Capabilities** e use a alternância **Artifacts**. Em planos Enterprise com controle de acesso baseado em função, você pode escopo adicional de artefatos para funções específicas: vá para **Settings > Roles**, edite uma função e defina a permissão **Artifacts** sob o grupo **Claude Code**.

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

Para definir quanto tempo os artefatos são mantidos antes da exclusão automática, vá para **Settings > Data & privacy controls**. Você pode definir períodos de retenção separados para artefatos que ainda são privados para seu autor e artefatos que foram compartilhados.

<h3 id="review-the-audit-log">
  Revisar o log de auditoria
</h3>

Publicar, compartilhar e excluir um artefato aparecem cada um no log de auditoria de sua organização sob os tipos de evento `claude_artifact_*`, a mesma família usada para artefatos criados em conversas do claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Adicionar o domínio do visualizador à lista de permissões
</h3>

O visualizador em claude.ai carrega cada artefato de uma origem `*.claudeusercontent.com` em sandbox. Se sua organização restringe o acesso à rede de saída, adicione esse domínio à sua lista de permissões junto com `claude.ai`. Consulte [Requisitos de acesso à rede](/docs/pt/network-config#network-access-requirements) para a lista completa.

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
