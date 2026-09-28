> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mensagem para suas outras sessões do Claude Code

> Deixe Claude listar e enviar mensagens para suas outras sessões do Claude Code nesta máquina, e alcance suas sessões em outras máquinas ou na web.

<Note>
  Mensagens entre sessões requerem Claude Code v2.1.224 ou posterior em macOS e Linux, incluindo Linux dentro do WSL 2. No Windows nativo, requer Claude Code v2.1.234 ou posterior. Quando uma sessão atende aos requisitos, as mensagens estão ativadas sem nada para habilitar. Consulte [Disponibilidade](#availability) para requisitos de provedor e como confirmar que uma sessão possui isso.
</Note>

Mensagens entre sessões permitem que Claude entregue uma mensagem de uma de suas sessões do Claude Code para outra. Quando uma mudança em uma sessão quebra o que outra está construindo, Claude pode avisar essa sessão antes que você perceba. Quando uma sessão resolve uma pergunta que outra está bloqueada, Claude pode enviar a resposta através.

Uma mensagem é um pedaço de texto que um Claude escreve para outro, nunca o histórico de conversa ou arquivos do remetente. Para mover uma conversa inteira ou seu contexto, [retome a sessão](/docs/pt/sessions#resume-a-session) em vez disso.

Claude usa duas ferramentas para isso: `ListAgents` para descobrir quais agentes ele pode alcançar, e `SendMessage` para entregar uma mensagem a um deles pelo nome. Com a mesma ferramenta `SendMessage`, Claude também pode enviar mensagens para [subagentes](/docs/pt/sub-agents#resume-subagents) e colegas de [equipe de agentes](/docs/pt/agent-teams) dentro de uma única sessão ou equipe. Esta página cobre mensagens entre suas sessões independentes.

<h2 id="when-to-use-cross-session-messaging">
  Quando usar cross-session messaging
</h2>

Use messaging quando uma de suas sessões tem algo que outra sessão precisa no meio da tarefa. Claude pode enviar uma mensagem por conta própria quando vê a necessidade, por exemplo após fazer uma mudança que afeta o trabalho que outra sessão está fazendo, ou você pode pedir que envie uma. Os casos comuns:

* **Entregar uma descoberta**: quando uma sessão descobre uma mudança quebrada ou toma uma decisão, Claude a resume para a sessão trabalhando na área afetada, em vez de você re-explicá-la lá.
* **Coordenar worktrees paralelos**: quando sessões trabalham o mesmo repositório em [worktrees](/docs/pt/worktrees) separadas, Claude pode dizer às outras sessões o que foi entregue.
* **Obter status de trabalho de longa duração**: ter uma migração ou execução de teste relatar de volta para a sessão que você está observando, ou pedir você mesmo de lá. Se essa sessão estiver nesta máquina, Claude também pode [pedir a ela um aviso quando ela próxima ficar ociosa ou sair](#get-a-notice-when-another-session-goes-idle).
* **Mensagem entre máquinas**: alcance uma de suas sessões em outra máquina ou na nuvem.

Use messaging entre sessões independentes que você inicia e direciona você mesmo. Claude Code tem um recurso dedicado para cada uma das outras maneiras de executar ou alcançar múltiplas sessões, então use o construído para o que você está fazendo em vez disso:

* Para continuar uma conversa em outro terminal, ou compartilhar seu contexto com uma nova sessão, [retome a sessão](/docs/pt/sessions#resume-a-session)
* Para uma equipe coordenada de sessões que Claude gera e supervisiona, use [equipes de agentes](/docs/pt/agent-teams)
* Para observar e direcionar muitas sessões de um lugar, use [visualização de agentes](/docs/pt/agent-view)
* Para direcionar uma sessão você mesmo do seu telefone ou outro dispositivo, em vez de ter sessões se enviando mensagens, use [Controle Remoto](/docs/pt/remote-control)
* Para enviar eventos externos, como resultados de CI ou mensagens de chat, para uma sessão, use [canais](/docs/pt/channels)

<h2 id="message-another-session">
  Mensagem para outra sessão
</h2>

Quando uma de suas sessões aprende algo que outra sessão precisa, como uma descoberta, um status ou uma decisão, Claude a passa em vez de você copiar e colar entre terminais. Claude descobre o alvo com `ListAgents` e envia com `SendMessage`, então você nunca chama nenhuma ferramenta você mesmo. Claude pode decidir enviar uma mensagem sem ser solicitado, e você também pode solicitar uma.

Para solicitar uma você mesmo, diga a Claude o que você quer que a outra sessão saiba ou faça. Este exemplo é um prompt que você digita, não uma mensagem que Claude envia:

```text wrap theme={null}
Ask the session running in my other terminal whether the migration finished
```

Claude escreve a mensagem real em si, então seu prompt pode deixar o conteúdo para Claude. Este prompt pede um resumo sem ditar sua redação, e o que Claude envia varia:

```text wrap theme={null}
Explain what we just did to the session working on the payments API
```

Para nomear o alvo você mesmo, mencione a sessão em seu prompt: digite `@` seguido pelas primeiras letras do nome da sessão e escolha a sessão do typeahead, da mesma forma que você [@-menciona um subagente](/docs/pt/sub-agents#invoke-subagents-explicitly). Requer Claude Code v2.1.232 ou posterior. Claude Code insere a menção, como `@api-worker`, e diz a Claude qual sessão ela nomeia, então Claude pode enviar mensagem para essa sessão sem listar suas sessões primeiro. Este prompt nomeia o alvo com uma menção:

```text wrap theme={null}
Let @api-worker know the schema migration finished
```

O typeahead lista suas outras sessões ao vivo nesta máquina. Dois casos precisam de mais do que as primeiras letras de um nome:

* **Uma sessão além desta máquina**: uma sessão na nuvem ou Remote Control aparece no typeahead apenas depois que Claude listou ou enviou mensagem para suas sessões além desta máquina, então peça a Claude para listá-las primeiro.
* **Um nome com espaço ou outros caracteres fora de letras, dígitos, hífens e sublinhados**: digite-o entre aspas duplas, como `@"release notes"`. Quando você escolhe a sessão do typeahead, Claude Code insere as aspas para você.

Você também pode digitar a menção sem o seletor. Quando mais de uma sessão ao vivo responde ao nome mencionado, Claude pergunta qual você quer dizer antes de enviar.

Para o que a mensagem que Claude escreve parece quando chega, incluindo um exemplo de uma, veja [como uma mensagem parece](#what-a-message-looks-like).

<h3 id="message-delivery">
  Entrega de mensagem
</h3>

O Claude receptor lê a mensagem entre chamadas de ferramenta durante um turno ativo, então uma ferramenta em execução nunca é interrompida. Quando a sessão receptora está ociosa, Claude Code inicia um novo turno com a mensagem.

Uma mensagem de outra sessão chega como texto simples. Se mencionar um arquivo ou um [recurso MCP](/docs/pt/mcp#use-mcp-resources) com `@`, Claude vê a menção como escrita e Claude Code não anexa nada, se a mensagem inicia um novo turno ou chega durante um. Claude ainda pode abrir um caminho mencionado na máquina receptora com suas próprias ferramentas, sujeito às permissões dessa sessão. Antes de v2.1.251, uma menção `@` em uma mensagem que iniciou um novo turno anexava o arquivo ou recurso MCP no lado receptor.

Claude Code recusa uma mensagem nos seguintes casos:

* A mensagem está [acima do limite de tamanho](#limitations). Claude Code a recusa na sessão de envio, antes de sair.
* Uma rajada rápida para uma sessão nesta máquina atingiu [o que a caixa de entrada dessa sessão aceita](#limitations). Claude Code recusa mais mensagens para essa sessão.
* O alvo de resposta nesta máquina falha em uma verificação de segurança, como um alvo com link simbólico ou um endpoint que não é o processo esperado. [Recusando enviar uma mensagem cross-session](/docs/pt/errors#refusing-to-send-a-cross-session-message) lista essas verificações.
* Claude endereça a mensagem ao nome da própria sessão, conforme descrito em [Veja quais sessões Claude pode alcançar](#see-which-sessions-claude-can-reach).

A sessão receptora verifica cada mensagem chegando contra seus próprios [controles de entrada](#control-inbound-messages), e a verificação termina em um dos três resultados:

* **Entregue**: Claude Code passa a mensagem para o Claude receptor.
* **Retida**: Claude Code coloca a mensagem de lado não entregue. Uma mensagem retida alcança Claude apenas quando você a aprova ou uma mudança de modo ou configurações posterior a permite.
* **Recusada**: Claude Code descarta a mensagem sem entregá-la.

Uma vez entregue, a mensagem conta para [uso](/docs/pt/costs) como um prompt que você digita, e o Claude receptor pode responder ao remetente da mesma forma, exceto no [caso cross-machine unidirecional](#message-sessions-on-other-machines).

Os limites de permissão permanecem por sessão. Claude é instruído nunca pedir a outra sessão uma ação que foi negada ou bloqueada em sua própria sessão, ou que suas próprias configurações de permissão bloqueariam, e rotear esse trabalho de volta para você em vez disso. No lado receptor, os [prompts de permissão da própria sessão receptora e regras ainda se aplicam](#how-a-session-treats-an-incoming-message) a qualquer coisa que a mensagem peça.

<h3 id="get-a-notice-when-another-session-goes-idle">
  Obter um aviso quando outra sessão fica ociosa
</h3>

Claude pode pedir a uma de suas sessões nesta máquina para enviar de volta um aviso quando essa sessão próxima ficar ociosa ou sair. Ocioso aqui significa que a sessão terminou um turno sem nada na fila. Use quando você está esperando uma tarefa longa em outra sessão e quer ouvir quando terminar em vez de verificar. Requer Claude Code v2.1.236 ou posterior em ambas as sessões.

<h4 id="ask-for-a-notice">
  Pedir um aviso
</h4>

Diga a Claude o que você está esperando. Este prompt pede um aviso da sessão de migração:

```text wrap theme={null}
Tell me when the migration session finishes what it's working on
```

Claude se inscreve com a entrada `notify_when_idle` da ferramenta `SendMessage`, anexada a uma mensagem que está enviando de qualquer forma ou por conta própria. Por conta própria, Claude Code se inscreve sem iniciar um turno ou gastar tokens na sessão observada, e envia o aviso imediatamente se essa sessão já estiver ociosa. Anexado a uma mensagem, Claude Code entrega a mensagem primeiro e envia o aviso depois.

<h4 id="what-each-session-shows">
  O que cada sessão mostra
</h4>

A sessão observada mostra uma linha dizendo que outro processo pediu para ser informado quando a sessão próxima ficar ociosa. A sessão solicitante mostra o aviso como uma linha nomeando a sessão observada. A linha pode incluir a hora em que o turno dessa sessão terminou e um status de uma linha desse turno. Se a sessão solicitante estiver ociosa, Claude Code inicia um novo turno com o aviso.

<h4 id="limits">
  Limites
</h4>

O aviso é único: Claude Code o envia uma vez da sessão observada, e nenhuma sessão sonda a outra. Se nenhum aviso chegar dentro de 12 horas, Claude Code descarta a inscrição e diz a Claude, então não fica esperando.

Os [controles de entrada](#control-inbound-messages) de cada lado se aplicam a um aviso como uma mensagem:

* **`refuse` em qualquer lado**: nada chega. A sessão observada descarta a solicitação sem registrar ou responder a ela, então a inscrição expira sem resposta após 12 horas, e uma sessão solicitante com `refuse` nunca se inscreve.
* **`hold` em qualquer lado**: o aviso chega com menos. A sessão observada deixa o status de uma linha de fora, e a sessão solicitante mostra o aviso em sua transcrição sem entregá-lo a Claude.

Apenas o Claude em sua conversa principal pode se inscrever, e apenas para suas sessões nesta máquina. Quando um subagente ou um colega de equipe de agentes define `notify_when_idle`, Claude Code não faz inscrição e diz a ele assim. Quando Claude pede um aviso de qualquer outro agente, como um colega, um subagente ou uma sessão além desta máquina, Claude Code recusa a chamada inteira, incluindo qualquer mensagem anexada a ela, e relata a recusa a Claude para que possa reenviar a mensagem sem a solicitação.

<h3 id="see-which-sessions-claude-can-reach">
  Veja quais sessões Claude pode alcançar
</h3>

Claude encontra o alvo de uma mensagem por conta própria, então você não precisa executar nada antes de pedir que envie. Para ver você mesmo quais sessões Claude pode alcançar, execute o comando `/list-agents`. A primeira linha, quando presente, é o nome da própria sessão, o que suas outras sessões usam para enviá-la mensagem. As linhas abaixo são as sessões que Claude pode alcançar:

* **Subagentes**: agentes executando dentro da sessão atual.
* **Colegas**: os próprios colegas de [equipe de agentes](/docs/pt/agent-teams) dessa sessão. Antes de v2.1.239, colegas não apareciam na listagem, embora Claude já pudesse enviá-los mensagem pelo nome.
* **Suas outras sessões locais**: sessões Claude Code executando na mesma máquina, incluindo [sessões em background](/docs/pt/agent-view). Uma sessão aparece apenas quando vincula um [socket de caixa de entrada](#the-sessions-inbox-socket).
* **Suas sessões na nuvem**: suas sessões [Claude Code na web](/docs/pt/claude-code-on-the-web), mostradas enquanto essa sessão está conectada a [Remote Control](/docs/pt/remote-control). Claude Code as rotula `cloud` na listagem.
* **Suas sessões Remote Control em outras máquinas**: mostradas enquanto essa sessão está conectada a [Remote Control](/docs/pt/remote-control), e rotuladas `Remote Control`. Claude Code mostra `offline` como o status de uma sessão cuja conexão Remote Control caiu.

Esta sessão não é uma das linhas. Se Claude endereça uma mensagem ao nome da própria sessão, Claude Code a recusa e diz a Claude que o alvo é a sessão atual. Antes de v2.1.239, a listagem não mostrava o nome dessa sessão, e Claude Code relatava uma mensagem enviada a ela como um agente que não conseguia encontrar.

Enquanto essa sessão está conectada a [Remote Control](/docs/pt/remote-control), Claude Code retém alguns detalhes de suas sessões locais da saída `/list-agents`, sem mudar o que Claude em si vê quando procura uma sessão para enviar mensagem:

* **Diretórios de trabalho**: deixa de fora o diretório de trabalho de cada sessão local.
* **Nomes de sessão**: deixa de fora qualquer nome de sessão que não possa atribuir a uma pessoa, então uma linha deixada sem nome lê `(unnamed session)`.
* **A primeira linha**: deixa de fora a linha com o nome dessa própria sessão a menos que você tenha digitado esse nome neste terminal, com `--name` ou com `/rename` e o nome, desde que iniciou ou retomou a sessão pela última vez.

Quando a saída lista qualquer coisa, termina com uma nota dizendo que detalhes foram retidos. Executar `/rename` seguido de um nome não utilizado em um teclado da própria sessão dá a essa sessão um nome que aparece na saída.

Claude Code lê suas listas de sessão na nuvem e Remote Control mais recentes primeiro e para após um número limitado de páginas para cada. Se sua conta tiver mais dessas sessões do que cabem, Claude Code não lista as mais antigas, e Claude não pode enviá-las mensagem pelo nome. Quando isso acontece, Claude Code diz assim na listagem, e Claude vê a mesma nota quando envia uma mensagem.

Claude endereça uma sessão além desta máquina pelo nome, da mesma forma que uma sessão local. Veja [Mensagem para sessões em outras máquinas](#message-sessions-on-other-machines) para como essas mensagens viajam.

Uma sessão responde ao nome que você define com o comando [`/rename`](/docs/pt/commands) ou a flag [`--name`](/docs/pt/cli-reference#cli-flags). Quando você não define um, Claude Code nomeia a sessão em si. Para uma sessão interativa, esse é o nome mostrado em [listagens de sessões em execução](/docs/pt/sessions#name-your-sessions).

Quando você renomeia uma sessão, Claude Code também atualiza o registro compartilhado que suas outras sessões usam para procurar o nome da sessão. Se não conseguir atualizar esse registro, avisa você na saída `/rename` que outras sessões ainda podem mostrar o nome antigo. Execute a sessão com [`--debug`](/docs/pt/cli-reference#cli-flags), e Claude Code registra a causa da atualização falhada.

Quando você renomeia uma sessão, ou inicia ou retoma uma interativa, com um nome que outra sessão ao vivo nesta máquina já usa, Claude Code deixa o nome com a sessão que já o tem e [renomeia o seu para uma variante](/docs/pt/sessions#name-your-sessions). Sessões ainda podem compartilhar um nome, por exemplo quando uma delas executa uma versão anterior de Claude Code ou o nome compartilhado é um que Claude Code gerou. A menos que essa sessão esteja conectada a Remote Control, Claude Code mostra o diretório de trabalho de cada sessão local na saída `/list-agents`, então você pode distinguir sessões com mesmo nome quando executam em diretórios diferentes. Claude endereça a mensagem de uma das duas formas, dependendo de quantas sessões ao vivo respondem ao nome:

* **Uma sessão responde ao nome**: Claude Code entrega a mensagem apenas no nome.
* **Várias sessões compartilham o nome, ou Claude Code não conseguiu verificar em todos os lugares onde suas sessões executam**: Claude adiciona um identificador curto a cada linha de sua listagem e usa o identificador no endereço.

<h3 id="message-sessions-on-other-machines">
  Mensagem para sessões em outras máquinas
</h3>

Como uma mensagem viaja, e se passa por servidores Anthropic, depende de onde a sessão alvo executa:

| Onde a outra sessão executa                         | Como a mensagem viaja                                                                                                               |
| :-------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| Nesta máquina                                       | Sobre um socket por sessão em macOS e Linux, ou um pipe nomeado por sessão no Windows nativo, nunca através de servidores Anthropic |
| Em outra de suas máquinas                           | Através de servidores Anthropic, chegando sobre a conexão [Remote Control](/docs/pt/remote-control) dessa máquina                        |
| Em [Claude Code na web](/docs/pt/claude-code-on-the-web) | Através de servidores Anthropic, direto para a sessão na nuvem                                                                      |

Iniciar uma conversa com uma sessão em outra de suas máquinas requer Claude Code v2.1.225 ou posterior e um alvo que [aparece na listagem](#see-which-sessions-claude-can-reach). Antes de v2.1.225, Claude só podia responder a uma mensagem que chegou de uma.

Você pode enviar mensagem para uma sessão mostrada como `offline` na [listagem](#see-which-sessions-claude-can-reach), uma cuja conexão Remote Control caiu. O envio passa, mas a mensagem chega apenas depois que a máquina dessa sessão se reconecta. Claude é informado disso quando envia.

A entrega na mesma máquina funciona onde quer que o recurso esteja habilitado. Cada sessão se registra em arquivos no disco. Quando Claude lista ou envia mensagem para suas sessões locais, Claude Code lê esses arquivos para encontrar as sessões, então duas sessões podem alcançar uma à outra apenas quando conseguem ver os mesmos arquivos.

Um contêiner tem seu próprio sistema de arquivos, então uma sessão dentro dele e uma sessão no host não podem alcançar uma à outra. Duas sessões dentro do mesmo contêiner ainda podem enviar mensagens uma à outra, incluindo em um [executor auto-hospedado](/docs/pt/self-hosted-environments). Uma sessão dentro de WSL 2 e uma sessão Windows nativa no mesmo computador também não podem alcançar uma à outra, porque se registram em diretórios home diferentes e escutam em tipos de socket diferentes.

Enquanto essa sessão está conectada a Remote Control, quando você envia mensagem para uma sessão em outra de suas máquinas, Claude Code mostra a mensagem na conversa dessa sessão sob o nome Remote Control dessa sessão. O Claude naquela máquina pode responder a esse nome. Por exemplo, quando essa sessão está conectada a Remote Control como `laptop-graceful-unicorn` e você envia mensagem para seu desktop, você vê a mensagem na sessão desktop sob `laptop-graceful-unicorn`.

Se essa sessão não estiver conectada a Remote Control quando Claude envia para uma sessão além desta máquina, a mensagem ainda passa, mas sem um [endereço de resposta](#what-a-message-looks-like), então o Claude receptor não pode respondê-la. Claude é informado disso quando envia.

Para exigir sua aprovação antes de qualquer mensagem ir além desta máquina, defina [`isolatePeerMachines`](#require-approval-for-cross-machine-messages).

<h2 id="how-a-session-treats-an-incoming-message">
  Como uma sessão trata uma mensagem chegando
</h2>

Quando a sessão A envia mensagem para a sessão B, Claude Code diz ao Claude de B que a mensagem veio de outra sessão, não de você, e limita o que a mensagem pode fazer:

* **Não pode aprovar nada**: uma mensagem de outra sessão nunca conta como seu consentimento, então não pode responder a um prompt de permissão pendente em seu nome.
* **Não pode mudar configuração**: Claude Code instrui o Claude receptor nunca mudar configurações de permissão, `CLAUDE.md` ou outra configuração porque outra sessão pediu.
* **Comandos não executam**: um comando no texto da mensagem, como `/compact`, chega como texto simples. Claude Code nunca o executa.
* **Prompts de permissão ainda disparam**: se agir na mensagem requer uma permissão que a sessão receptora não tem, você vê o mesmo prompt que veria para qualquer outro trabalho.

<h3 id="what-a-message-looks-like">
  Como uma mensagem parece
</h3>

Quando uma mensagem chega, Claude Code a mostra na conversa como uma prévia de uma linha fraca, e a linha de prévia fica na conversa depois. A prévia carrega o nome do remetente e a primeira linha da mensagem, cortada com `…` quando é longa, como `› Message from @api-worker: Schema migration finished (ctrl+o to expand)`. Antes de v2.1.247, Claude Code mostrava a mensagem chegando em cheio em vez de uma prévia.

Qualquer um desses mostra o texto completo:

* Pressione `Ctrl+O` para abrir o [visualizador de transcrição](/docs/pt/interactive-mode#transcript-viewer) e ler o texto completo sob o nome da sessão do remetente.
* Em uma sessão iniciada com [`--verbose`](/docs/pt/cli-reference#cli-flags), Claude Code mostra o texto completo em vez da prévia.

A prévia encurta apenas o que você vê. Se você a expande ou não, Claude lê a mensagem completa.

Claude recebe a mensagem com o nome do remetente e um endereço de resposta, exceto para uma [mensagem cross-machine unidirecional](#message-sessions-on-other-machines), que não carrega endereço de resposta. Além do nome e endereço de resposta, o Claude receptor obtém o texto da mensagem, nunca o histórico de conversa do remetente ou arquivos. [Entrega de mensagem](#message-delivery) cobre menções `@` no texto.

Uma mensagem que um [subagente](/docs/pt/sub-agents) escreveu chega sob o nome da sessão de envio, com o subagente identificado no texto da mensagem. Uma resposta a ela alcança a conversa principal dessa sessão, não o subagente.

Este exemplo é uma mensagem que um Claude escreveu para outro, como seu texto completo lê quando você o expande:

```text wrap theme={null}
Schema migration finished
The new column is tenant_id, and rebasing on main is safe now.
```

<h3 id="control-inbound-messages">
  Controlar mensagens chegando
</h3>

Defina [`crossSessionInbound`](/docs/pt/settings-reference#crosssessioninbound) para escolher o que uma sessão faz com mensagens chegando de suas outras sessões:

| Valor    | Comportamento                                                                                                                                                                                                         |
| :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `accept` | Claude Code entrega cada mensagem a Claude                                                                                                                                                                            |
| `hold`   | Claude Code mostra um aviso para cada mensagem e não a entrega. Se um `accept` depois se aplicar, por as [regras de precedência](/docs/pt/settings-reference#crosssessioninbound), Claude Code libera as mensagens retidas |
| `refuse` | Claude Code descarta cada mensagem sem entregá-la                                                                                                                                                                     |

Além de editar um arquivo de configurações, você pode selecionar o valor na linha `/config` **Messages from your other sessions**. Claude Code escreve o valor que você seleciona para suas configurações de usuário. A linha requer Claude Code v2.1.232 ou posterior e não aparece enquanto configurações gerenciadas ou a flag `--settings` define a chave, já que um valor de configurações de usuário não se aplicaria então. Claude Code rejeita o atalho `/config crossSessionInbound=value` para essa chave.

Para ver qual valor se aplica, siga as regras de precedência `crossSessionInbound` na [referência de configurações](/docs/pt/settings-reference#crosssessioninbound).

Quando nenhum valor se aplica, Claude Code decide por mensagem das duas classes de modo de permissão das sessões. Agrupa sessões que [contornam prompts de permissão](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode) em uma classe, e toda outra sessão na outra. Plan mode conta como contornando em sessões com permissões de bypass disponíveis, e [auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), `acceptEdits` e `dontAsk` contam como solicitando:

* **A sessão receptora solicita permissões**: Claude Code entrega cada mensagem. Retém uma apenas para sua aprovação quando a sessão de envio se identifica como contornando prompts de permissão.
* **A sessão receptora contorna prompts de permissão**: Claude Code retém cada mensagem para sua aprovação. Entrega uma apenas quando a sessão de envio se identifica como também contornando.

Quando o padrão retém uma mensagem, Claude Code abre um diálogo de aprovação na sessão receptora. O diálogo mostra o remetente e uma prévia:

* **Approve** entrega essa mensagem a Claude.
* **Deny**, ou descartar o diálogo, a descarta.
* Quando o diálogo fica sem resposta após o prazo [`dialogExpiry`](/docs/pt/settings-reference#dialogexpiry), Claude Code o fecha e descarta a mensagem. O prazo padrão é cinco minutos.
* Enquanto nenhum terminal está anexado a uma [sessão em background](/docs/pt/agent-view), Claude Code deixa o diálogo aberto após o prazo. Depois que você anexa, se o diálogo fica sem resposta por um período de prazo completo, Claude Code o fecha e descarta a mensagem.
* Se a classe de modo de permissão dessa sessão muda enquanto mensagens estão retidas, Claude Code re-aplica as regras de entrada, entrega as mensagens que agora aceita, e mostra um aviso.
* Se uma mudança de configurações faz `refuse` se aplicar enquanto mensagens estão retidas, Claude Code descarta cada mensagem retida e relata uma recusa a cada remetente que pode alcançar.

Quando o remetente é uma sessão na mesma máquina, Claude Code envia um aviso de volta para ela quando o receptor retém a mensagem, e um acompanhamento quando o receptor depois entrega, nega ou expira. O aviso alcança o Claude de envio, então ele sabe não continuar esperando por uma mensagem que a outra sessão não leu.

Em uma sessão de envio interativa, o aviso aparece na transcrição. Um remetente [`claude -p`](/docs/pt/headless) recebe em [saída transmitida](/docs/pt/headless#stream-responses) como uma [mensagem `system` informacional](/docs/pt/agent-sdk/typescript#sdkinformationalmessage). Avisos para remetentes `claude -p` requerem Claude Code v2.1.271 ou posterior.

Se o receptor recusa a mensagem, o aviso do remetente diz que o receptor não está aceitando mensagens cross-session e diz ao Claude do remetente não esperar ou reenviar.

Claude Code retém no máximo 100 mensagens, separadamente da fila de entrega, e além disso descarta a mais antiga.

<h3 id="non-interactive-sessions">
  Sessões não-interativas
</h3>

Claude Code vincula um socket de caixa de entrada para uma sessão [`claude -p`](/docs/pt/headless) como uma interativa, então um worker `-p` de longa duração pode receber mensagens e aparece na listagem. Quando você inicia uma sessão em [modo bare](/docs/pt/headless#start-faster-with-bare-mode), Claude Code não vincula o socket, então essa sessão não pode receber mensagens e não aparece na lista de agentes.

Uma sessão `-p` não pode mostrar o diálogo de aprovação. Quando o [padrão de entrada](#control-inbound-messages) retém uma mensagem lá, Claude Code a mantém pelo mesmo prazo [`dialogExpiry`](/docs/pt/settings-reference#dialogexpiry) que o diálogo usa, cinco minutos por padrão:

* **Antes do prazo**: se um modo ou mudança de configurações permite a mensagem, Claude Code a entrega.
* **Após o prazo**: Claude Code descarta a mensagem e a relata como expirada a um remetente que pode alcançar.

Defina `dialogExpiry` para `"never"` para manter mensagens padrão-retidas até a sessão terminar. Uma mensagem retida por uma configuração `hold` explícita não expira; Claude Code a entrega apenas quando um `accept` depois se aplica.

Quando a sessão termina com mensagens ainda retidas, Claude Code as relata como expiradas a cada remetente que pode alcançar. Antes de v2.1.225, nenhum prazo se aplicava em uma sessão `-p`: uma mensagem retida ficava retida a menos que uma mudança de modo de permissão durante a execução a entregasse, e uma sessão que terminava com mensagens retidas não relatava nada a seus remetentes.

Para deixar um worker `-p` receber mensagens desatendido, inicie-o com `crossSessionInbound` definido para `accept` em seu valor `--settings`. Um `accept` em suas configurações de usuário também funciona mas se aplica a cada sessão que você executa.

<h3 id="the-sessions-inbox-socket">
  O socket de caixa de entrada da sessão
</h3>

Leia esta seção quando uma sessão que você espera não está na lista de agentes, quando você quer um script ou hook para postar em uma sessão, ou quando um comando sandboxed não consegue alcançar o socket.

Claude Code vincula um socket de caixa de entrada para cada sessão com cross-session messaging habilitado, onde outras sessões na máquina entregam mensagens. O socket é um socket de domínio Unix em macOS e Linux, incluindo Linux dentro de WSL 2, e um pipe nomeado no Windows nativo. Para quais tipos de sessão vinculam um, veja [Sessões não-interativas](#non-interactive-sessions).

Você pode encontrar o caminho do socket em dois lugares:

* `/status` o mostra na linha `Peer address`. O caminho é prefixado com `uds:`.
* Claude Code o exporta para [hooks](/docs/pt/hooks) e comandos Bash como a variável de ambiente [`CLAUDE_CODE_MESSAGING_SOCKET`](/docs/pt/env-vars#variables):
  * Em uma sessão que inicia com messaging ativado, Claude Code exporta a variável antes de qualquer hook executar, incluindo `SessionStart`.
  * Cada sessão exporta seu próprio socket, nunca um herdado de uma sessão pai.

Em macOS e Linux, Claude Code restringe o socket ao seu usuário do sistema operacional. No Windows nativo, em vez disso requer que cada conexão se autentique primeiro com uma chave que apenas seu usuário do sistema operacional pode ler. De qualquer forma, em uma máquina compartilhada as sessões de outro usuário não podem entregar a ela.

Em macOS e Linux, Claude Code também recusa criar o socket em um diretório que não consegue aceitar, por exemplo um que outro usuário possui, e usa um diretório privado por usuário, `/tmp/cc-socks-<uid>`, em vez disso. Quando não consegue aceitar nenhum diretório, a sessão executa sem uma caixa de entrada: Claude Code mostra um aviso, `/status` mostra `unavailable` e a razão em sua linha `Peer address`, e o log [`--debug`](/docs/pt/cli-reference#cli-flags) registra a recusa completa.

Ao lado do caminho do socket, Claude Code exporta um token por sessão como [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/pt/env-vars#variables). Um script postando para o socket da sua própria sessão pode enviar `{"type":"auth","token":"<token>"}` como a primeira linha de sua conexão, onde `<token>` é o valor de `CLAUDE_CODE_MESSAGING_TOKEN`. Se Claude Code requer a linha depende da plataforma:

* **macOS e Linux, incluindo WSL 2**: a linha é opcional. Claude Code aceita uma conexão com ou sem ela.
* **Windows nativo**: a linha é obrigatória. Claude Code fecha qualquer conexão cuja primeira linha não é uma linha de autenticação válida e não entrega nada dessa conexão.

Abra a conexão apenas quando a mensagem que você está postando está pronta. Claude Code fecha uma conexão que não enviou uma linha completa dentro de 30 segundos, então capture a saída de um comando lento primeiro e depois abra a conexão para enviá-la.

As [regras own-child](#own-child-messages) abaixo dizem quando Claude Code consulta o token e como trata uma mensagem que não consegue verificar.

<span id="own-child-messages" />Claude Code executa mensagens chegando no socket através dos mesmos [controles de entrada](#control-inbound-messages) que qualquer outra mensagem peer, com uma exceção e um pré-requisito:

* **Mensagens own-child**: quando nenhum valor `crossSessionInbound` se aplica, Claude Code entrega uma mensagem que verifica veio dos processos filhos da própria sessão, como um hook ou comando Bash postando de volta para o socket da própria sessão.
  * Em Linux, incluindo dentro de WSL 2, Claude Code pode verificar por evidência de processo mesmo para um filho que já saiu. Em macOS pode verificar assim apenas enquanto o processo de postagem ainda está executando, e em um contêiner onde Claude Code executa como ID de processo 1 não tem evidência de processo. No Windows nativo também não tem.
  * Em macOS depois que o processo de postagem saiu e em contêineres onde Claude Code executa como ID de processo 1, essa evidência de processo está faltando, e Claude Code em vez disso verifica um filho que enviou o [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/pt/env-vars#variables) exportado da sessão na linha de autenticação que abriu sua conexão. No Windows nativo, esse token é a única forma que Claude Code verifica uma mensagem own-child.
  * Quando Claude Code não consegue verificar de nenhuma forma, trata a mensagem como qualquer outra que não afirma nenhuma classe de permissão, então uma sessão que contorna prompts de permissão a retém para sua aprovação.
* **Sessões sandboxed**: controle se um comando Bash pode alcançar o socket de dentro do [sandbox](/docs/pt/sandboxing) com as configurações de socket Unix do sandbox, [`sandbox.network.allowAllUnixSockets` e `sandbox.network.allowUnixSockets`](/docs/pt/settings-reference#sandbox-settings).

<h2 id="restrict-cross-session-messaging">
  Restringir cross-session messaging
</h2>

Além dos padrões por mensagem, você pode estreitar o messaging de duas formas. Exigir sua aprovação antes de qualquer mensagem sair da máquina, ou desativar o messaging para uma sessão ou uma organização.

<h3 id="require-approval-for-cross-machine-messages">
  Exigir aprovação para mensagens cross-machine
</h3>

Defina [`isolatePeerMachines`](/docs/pt/settings-reference#isolatepeermachines) para `true` para exigir sua aprovação explícita antes de qualquer `SendMessage` alcançar uma sessão além desta máquina:

```json theme={null}
{
  "isolatePeerMachines": true
}
```

Com isso definido, Claude Code pede sua aprovação antes da mensagem de Claude para uma sessão além desta máquina sair, mesmo em modo `bypassPermissions`, que pula prompts de permissão ordinários. Um `true` de qualquer escopo de configurações se aplica, então um arquivo de projeto verificado pode ativar o requisito mas não desativá-lo. Claude Code não solicita para mensagens entre sessões na mesma máquina.

<h3 id="turn-off-cross-session-messaging">
  Desativar cross-session messaging
</h3>

Receber e enviar são controles separados, então desative qualquer direção que você precise, ou ambas. Use `crossSessionInbound` para mensagens que chegam, e regras de permissão para o que Claude aqui pode enviar ou listar:

* **Parar de receber**: defina `crossSessionInbound` para `refuse`, e Claude Code descarta mensagens peer de entrada sem entregá-las. De configurações de projeto ou local, `refuse` se aplica sobre toda outra fonte, e de suas configurações de usuário se aplica a menos que configurações gerenciadas ou a flag `--settings` definam um valor.
* **Parar de enviar e listar**: adicione [regras de negação de permissão](/docs/pt/permissions#tool-specific-permission-rules) nomeando `SendMessage` e `ListAgents`. Ambas pegam o nome da ferramenta simples sem especificador.

Administradores podem desativar ambos os lados para uma organização em [configurações gerenciadas](/docs/pt/managed-settings), combinando as regras de negação com o `refuse`:

```json theme={null}
{
  "permissions": {
    "deny": ["SendMessage", "ListAgents"]
  },
  "crossSessionInbound": "refuse"
}
```

Com isso em vigor, Claude Code ainda vincula o socket de caixa de entrada de cada sessão, mas descarta cada mensagem que chega nele sem entregar nada a Claude. Negar `SendMessage` também remove messaging para subagentes e colegas de equipe de agentes, já que a mesma ferramenta serve ambos. Uma sessão recusante mostra nenhuma mudança visível, em seu próprio `/status` ou nas listagens de outras sessões na mesma máquina, então para confirmar, verifique os arquivos de configurações que se aplicam a essa sessão em vez de seu status.

<h2 id="availability">
  Disponibilidade
</h2>

Cross-session messaging requer Claude Code v2.1.224 ou posterior em macOS, Linux e WSL 2, e v2.1.234 ou posterior no Windows nativo. Disponibilidade, e quais sessões Claude pode enviar mensagem, também dependem de seu sistema operacional, provedor e configuração:

* **Sistema operacional**: disponível em macOS, Windows e Linux, incluindo Linux dentro de WSL 2.

* **Sessões nesta máquina**: disponível em cada provedor, incluindo Amazon Bedrock, Claude Platform em AWS, Google Cloud's Agent Platform e Microsoft Foundry, e em sessões que executam com [busca de flag de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching) desativada. Naqueles provedores, e com busca de flag desativada, messaging na mesma máquina requer Claude Code v2.1.248 ou posterior. Claude Code entrega essas mensagens sobre um [socket por sessão em sua máquina](#the-sessions-inbox-socket), nunca através de servidores Anthropic.

  Para parar uma sessão de recebê-las, defina [`crossSessionInbound`](#turn-off-cross-session-messaging) para `refuse`.

* **Sessões além desta máquina**: Claude encontra suas sessões [Claude Code na web](/docs/pt/claude-code-on-the-web) e suas sessões em outras máquinas de uma sessão que está conectada a Controle Remoto, que precisa de um sign-in claude.ai como autenticação ativa dessa sessão e os outros [requisitos Controle Remoto](/docs/pt/remote-control#requirements). Claude não consegue encontrar essas sessões com uma chave de API ou em Amazon Bedrock, Claude Platform em AWS, Google Cloud's Agent Platform e Microsoft Foundry.

Para verificar uma sessão, digite `/list-agents`, também disponível como `/peers`. O resultado separa uma sessão que não tem o recurso de uma sessão onde algo mais estreito bloqueou uma mensagem, como uma ferramenta `SendMessage` faltante ou um envio recusado:

* **`/list-agents` não é reconhecido**: a sessão não tem cross-session messaging. Trabalhe através dos requisitos acima, começando com `claude --version` para o requisito de versão.
* **`/list-agents` funciona mas um envio não chegou**: messaging está ativado, e algo mais estreito se aplica:
  * **Regras de negação**: uma [regra de negação de permissão](#turn-off-cross-session-messaging) remove as ferramentas `SendMessage` e `ListAgents`.
  * **Controles de entrada**: os [controles de entrada da sessão receptora](#control-inbound-messages) podem reter ou descartar o que você envia a ela.
  * **Sessão na nuvem faltando**: uma sessão na nuvem aparece apenas enquanto essa sessão está conectada a [Controle Remoto](/docs/pt/remote-control).
  * **Sessão em outra máquina faltando**: uma sessão em outra de suas máquinas aparece apenas quando executa com [Controle Remoto](/docs/pt/remote-control) e essa sessão também está conectada.
  * **Sessão em outra máquina `offline`**: uma mensagem para uma sessão listada como `offline` passa, mas [chega apenas depois que a máquina dessa sessão se reconecta](#message-sessions-on-other-machines).
  * **Sessão na nuvem ou em outra máquina mais antiga faltando**: Claude Code [lê essas listas de sessão mais recentes primeiro e para após um número limitado de páginas](#see-which-sessions-claude-can-reach), então Claude não consegue enviar mensagem para uma sessão que caiu além delas pelo nome.
  * **Iniciando uma conversa**: [Mensagem para sessões em outras máquinas](#message-sessions-on-other-machines) cobre iniciar uma conversa com uma sessão além desta máquina.

Em uma sessão com messaging, `/status` também mostra uma linha `Peer address` com o endereço de caixa de entrada da própria sessão, ou `unavailable` e a razão quando Claude Code [não conseguiu configurar uma caixa de entrada](#the-sessions-inbox-socket).

<h2 id="limitations">
  Limitações
</h2>

Os limites aqui são propriedades do próprio canal de messaging e se aplicam onde quer que o recurso execute. Para lacunas de plataforma e provedor, veja [Disponibilidade](#availability) em vez disso.

* **Apenas texto simples**: Claude envia apenas texto simples entre sessões. Mensagens de protocolo [equipe de agentes](/docs/pt/agent-teams) estruturadas ficam dentro de uma equipe.
* **O tamanho da mensagem na mesma máquina é limitado**: Claude Code recusa uma mensagem para uma sessão nesta máquina uma vez que sua forma serializada passa cerca de um milhão de caracteres. A recusa [nomeia os tamanhos exatos](/docs/pt/errors#message-too-large-for-cross-session-delivery). Nada alcança a sessão receptora.
* **Rajadas rápidas para uma sessão são recusadas no remetente**: uma vez que uma rajada rápida de mensagens para uma sessão nesta máquina atinge o que a caixa de entrada dessa sessão aceita, Claude Code recusa envios adicionais na sessão de envio. A [recusa nomeia a rajada](/docs/pt/errors#too-many-messages-to-this-session-just-now) e diz a Claude para agrupar o resto em uma mensagem ou esperar. Antes de v2.1.236, Claude Code relatava esses envios como enviados enquanto a sessão receptora os descartava.
* **Loops de mensagem são limitados**: na sessão receptora, Claude Code limita a taxa de mensagens repetidas por remetente, descarta repetições idênticas chegando dentro de uma janela curta, e enfileira no máximo 50 mensagens aceitas para Claude ler. Um loop de mensagem entre duas sessões portanto para por conta própria. Quando o limite de taxa, verificação de repetição ou limite de fila descarta uma mensagem de uma sessão interativa nesta máquina, Claude Code diz a essa sessão qual descartou e diz seu Claude não reenviar imediatamente.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Subagentes](/docs/pt/sub-agents#resume-subagents) e [equipes de agentes](/docs/pt/agent-teams#messages-between-agents): messaging dentro de uma única sessão ou equipe
* [Agentes em background](/docs/pt/agent-view): despache e monitore as sessões paralelas que você pode enviar mensagem
* [Controle Remoto](/docs/pt/remote-control): conecte essa sessão para alcançar suas sessões em outras máquinas
* [Configurações](/docs/pt/settings-reference#all-settings): `crossSessionInbound`, `isolatePeerMachines` e `dialogExpiry`
* [Modos de permissão](/docs/pt/permission-modes): os modos por trás das duas classes do padrão de entrada
* [Referência de ferramentas](/docs/pt/tools-reference): as linhas `ListAgents` e `SendMessage` na tabela de ferramentas
* [Executar agentes em paralelo](/docs/pt/agents): compare as formas que Claude Code executa múltiplos agentes
