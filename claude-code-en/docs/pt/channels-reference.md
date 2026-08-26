> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência de Channels

> Construa um servidor MCP que envia webhooks, alertas e mensagens de chat para uma sessão Claude Code. Referência para o contrato de channel: declaração de capacidade, eventos de notificação, ferramentas de resposta, gating de remetente e retransmissão de permissão.

<Note>
  Channels estão em [visualização de pesquisa](/docs/pt/channels#research-preview). Organizações Team e Enterprise devem [habilitá-los explicitamente](/docs/pt/channels#enterprise-controls).
</Note>

Um channel é um servidor MCP que envia eventos para uma sessão Claude Code para que Claude possa reagir a coisas que acontecem fora do terminal.

Você pode construir um channel unidirecional ou bidirecional. Channels unidirecionais encaminham alertas, webhooks ou eventos de monitoramento para Claude agir. Channels bidirecionais como pontes de chat também [expõem uma ferramenta de resposta](#expose-a-reply-tool) para que Claude possa enviar mensagens de volta. Um channel com um caminho de remetente confiável também pode optar por [retransmitir prompts de permissão](#relay-permission-prompts) para que você possa aprovar ou negar o uso de ferramentas remotamente.

Esta página cobre:

* [Visão geral](#overview): como os channels funcionam
* [O que você precisa](#what-you-need): requisitos e etapas gerais
* [Exemplo: construir um receptor de webhook](#example-build-a-webhook-receiver): um passo a passo mínimo unidirecional
* [Opções de servidor](#server-options): os campos do construtor
* [Formato de notificação](#notification-format): o payload do evento e comportamento de entrega
* [Expor uma ferramenta de resposta](#expose-a-reply-tool): deixar Claude enviar mensagens de volta
* [Gate de mensagens de entrada](#gate-inbound-messages): verificações de remetente para evitar injeção de prompt
* [Retransmitir prompts de permissão](#relay-permission-prompts): encaminhar prompts de aprovação de ferramentas para channels remotos

Para usar um channel existente em vez de construir um, consulte [Channels](/docs/pt/channels). Telegram, Discord, iMessage e fakechat estão incluídos na visualização de pesquisa.

<h2 id="overview">
  Visão geral
</h2>

Um channel é um servidor [MCP](https://modelcontextprotocol.io) que é executado na mesma máquina que Claude Code. Claude Code o spawna como um subprocesso e se comunica via stdio. Seu servidor de channel é a ponte entre sistemas externos e a sessão Claude Code:

* **Plataformas de chat** (Telegram, Discord): seu plugin é executado localmente e faz polling da API da plataforma para novas mensagens. Quando alguém envia uma DM para seu bot, o plugin recebe a mensagem e a encaminha para Claude. Nenhuma URL para expor.
* **Webhooks** (CI, monitoramento): seu servidor escuta em uma porta HTTP local. Sistemas externos fazem POST para essa porta, e seu servidor envia o payload para Claude.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-architecture.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=9a037b7da80184ae49015c0256b21a1f" alt="Diagrama de arquitetura mostrando sistemas externos se conectando ao seu servidor de channel local, que se comunica com Claude Code via stdio" width="600" height="220" data-path="images/channel-architecture.svg" />

<h2 id="what-you-need">
  O que você precisa
</h2>

O único requisito obrigatório é o pacote [`@modelcontextprotocol/sdk`](https://www.npmjs.com/package/@modelcontextprotocol/sdk) e um runtime compatível com Node.js. [Bun](https://bun.sh), [Node](https://nodejs.org) e [Deno](https://deno.com) funcionam. Os plugins pré-construídos na visualização de pesquisa usam Bun, mas seu channel não precisa.

Seu servidor precisa:

1. Declarar a capacidade `claude/channel` para que Claude Code registre um listener de notificação
2. Emitir eventos `notifications/claude/channel` quando algo acontecer
3. Conectar via [transporte stdio](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) (Claude Code spawna seu servidor como um subprocesso)

As seções [Opções de servidor](#server-options) e [Formato de notificação](#notification-format) cobrem cada uma delas em detalhes. Consulte [Exemplo: construir um receptor de webhook](#example-build-a-webhook-receiver) para um passo a passo completo.

Durante a visualização de pesquisa, channels personalizados não estão na [lista de aprovação](/docs/pt/channels#supported-channels). Use `--dangerously-load-development-channels` para testar localmente. Consulte [Testar durante a visualização de pesquisa](#test-during-the-research-preview) para detalhes.

<h2 id="example-build-a-webhook-receiver">
  Exemplo: construir um receptor de webhook
</h2>

Este passo a passo constrói um servidor de arquivo único que escuta solicitações HTTP e as encaminha para sua sessão Claude Code. No final, qualquer coisa que possa enviar um HTTP POST, como um pipeline de CI, um alerta de monitoramento ou um comando `curl`, pode enviar eventos para Claude.

Este exemplo usa [Bun](https://bun.sh) como o runtime para seu servidor HTTP integrado e suporte a TypeScript. Você pode usar [Node](https://nodejs.org) ou [Deno](https://deno.com) em vez disso; o único requisito é o [MCP SDK](https://www.npmjs.com/package/@modelcontextprotocol/sdk).

<Steps>
  <Step title="Criar o projeto">
    Crie um novo diretório e instale o MCP SDK:

    ```bash theme={null}
    mkdir webhook-channel && cd webhook-channel
    bun add @modelcontextprotocol/sdk
    ```
  </Step>

  <Step title="Escrever o servidor de channel">
    Crie um arquivo chamado `webhook.ts`. Este é seu servidor de channel inteiro: ele se conecta a Claude Code via stdio e escuta POSTs HTTP na porta 8788. Quando uma solicitação chega, ele envia o corpo para Claude como um evento de channel.

    ```ts title="webhook.ts" theme={null}
    #!/usr/bin/env bun
    import { Server } from '@modelcontextprotocol/sdk/server/index.js'
    import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'

    // Criar o servidor MCP e declará-lo como um channel
    const mcp = new Server(
      { name: 'webhook', version: '0.0.1' },
      {
        // esta chave é o que o torna um channel — Claude Code registra um listener para ela
        capabilities: { experimental: { 'claude/channel': {} } },
        // adicionado ao prompt do sistema de Claude para que ele saiba como lidar com esses eventos
        instructions: 'Events from the webhook channel arrive as <channel source="webhook" ...>. They are one-way: read them and act, no reply expected.',
      },
    )

    // Conectar a Claude Code via stdio (Claude Code spawna este processo)
    await mcp.connect(new StdioServerTransport())

    // Iniciar um servidor HTTP que encaminha cada POST para Claude
    Bun.serve({
      port: 8788,  // qualquer porta aberta funciona
      // apenas localhost: nada fora desta máquina pode fazer POST
      hostname: '127.0.0.1',
      async fetch(req) {
        const body = await req.text()
        await mcp.notification({
          method: 'notifications/claude/channel',
          params: {
            content: body,  // torna-se o corpo da tag <channel>
            // cada chave torna-se um atributo de tag, ex: <channel path="/" method="POST">
            meta: { path: new URL(req.url).pathname, method: req.method },
          },
        })
        return new Response('ok')
      },
    })
    ```

    O arquivo faz três coisas em ordem:

    * **Configuração do servidor**: cria o servidor MCP com `claude/channel` em suas capacidades, o que é o que diz a Claude Code que este é um channel. A string [`instructions`](#server-options) vai para o prompt do sistema de Claude: diga a Claude quais eventos esperar, se deve responder e como rotear respostas se deve.
    * **Conexão stdio**: conecta a Claude Code via stdin/stdout. Isto é padrão para qualquer [servidor MCP](https://modelcontextprotocol.io/docs/concepts/transports#standard-io): Claude Code o spawna como um subprocesso.
    * **Listener HTTP**: inicia um servidor web local na porta 8788. Cada corpo POST é encaminhado para Claude como um evento de channel via `mcp.notification()`. O `content` torna-se o corpo do evento, e cada entrada `meta` torna-se um atributo na tag `<channel>`. O listener precisa de acesso à instância `mcp`, então é executado no mesmo processo. Você poderia dividi-lo em módulos separados para um projeto maior.
  </Step>

  <Step title="Registrar seu servidor com Claude Code">
    Adicione o servidor à sua configuração MCP para que Claude Code saiba como iniciá-lo. Para um `.mcp.json` em nível de projeto no mesmo diretório, use um caminho relativo. Para configuração em nível de usuário em `~/.claude.json`, use o caminho absoluto completo para que o servidor possa ser encontrado de qualquer projeto:

    ```json title=".mcp.json" theme={null}
    {
      "mcpServers": {
        "webhook": { "command": "bun", "args": ["./webhook.ts"] }
      }
    }
    ```

    Claude Code lê sua configuração MCP na inicialização e spawna cada servidor como um subprocesso.
  </Step>

  <Step title="Testá-lo">
    Durante a visualização de pesquisa, channels personalizados não estão na lista de aprovação, então inicie Claude Code com a flag de desenvolvimento:

    ```bash theme={null}
    claude --dangerously-load-development-channels server:webhook
    ```

    A primeira vez que você inicia uma sessão neste projeto, Claude Code pede consentimento antes de usar o novo servidor de `.mcp.json`. O diálogo relata "New MCP server found in this project: webhook". Selecione **Use this MCP server** para continuar.

    Quando Claude Code inicia, ele lê sua configuração MCP, spawna seu `webhook.ts` como um subprocesso, e o listener HTTP inicia automaticamente na porta que você configurou (8788 neste exemplo). Você não precisa executar o servidor você mesmo.

    Um aviso atenuado abaixo do banner de inicialização confirma que o channel está registrado: `Channels (experimental) messages from server:webhook inject directly in this session · restart without --dangerously-load-development-channels to stop`.

    Se você vir "blocked by org policy," seu administrador de organização precisa [habilitar channels](/docs/pt/channels#enterprise-controls) primeiro.

    Em um terminal separado, simule um webhook enviando um HTTP POST com uma mensagem para seu servidor. Este exemplo envia um alerta de falha de CI para a porta 8788 (ou qualquer porta que você configurou):

    ```bash theme={null}
    curl -X POST localhost:8788 -d "build failed on main: https://ci.example.com/run/1234"
    ```

    O payload chega em sua sessão Claude Code como uma tag `<channel>`:

    ```text theme={null}
    <channel source="webhook" path="/" method="POST">build failed on main: https://ci.example.com/run/1234</channel>
    ```

    Em seu terminal Claude Code, você verá Claude receber a mensagem e começar a responder: lendo arquivos, executando comandos ou o que a mensagem exigir. Este é um channel unidirecional, então Claude age em sua sessão mas não envia nada de volta através do webhook. Para adicionar respostas, consulte [Expor uma ferramenta de resposta](#expose-a-reply-tool).

    Se o evento não chegar, o diagnóstico depende do que `curl` retornou:

    * **`curl` sucede mas nada chega a Claude**: execute `/mcp` em sua sessão para verificar o status do servidor. "Failed to connect" geralmente significa um erro de dependência ou importação em seu arquivo de servidor; verifique o log de debug em `~/.claude/debug/<session-id>.txt` para o rastreamento stderr.
    * **`curl` falha com "connection refused"**: a porta não está vinculada ainda ou um processo obsoleto de uma execução anterior a está mantendo. `lsof -i :<port>` mostra o que está escutando; `kill` o processo obsoleto antes de reiniciar sua sessão.
  </Step>
</Steps>

O [servidor fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) estende este padrão com uma UI web, anexos de arquivo e uma ferramenta de resposta para chat bidirecional.

<h2 id="test-during-the-research-preview">
  Testar durante a visualização de pesquisa
</h2>

Durante a visualização de pesquisa, cada channel deve estar na [lista de aprovação](/docs/pt/channels#research-preview) para se registrar. A flag de desenvolvimento contorna a lista de aprovação para entradas específicas após um prompt de confirmação. Este exemplo mostra ambos os tipos de entrada:

```bash theme={null}
# Testando um plugin que você está desenvolvendo
claude --dangerously-load-development-channels plugin:yourplugin@yourmarketplace

# Testando um servidor .mcp.json simples (sem wrapper de plugin ainda)
claude --dangerously-load-development-channels server:webhook
```

O bypass é por entrada. Combinar esta flag com `--channels` não estende o bypass para as entradas `--channels`. Durante a visualização de pesquisa, a lista de aprovação é curada pela Anthropic, então seu channel permanece na flag de desenvolvimento enquanto você constrói e testa.

<Note>
  Esta flag pula apenas a lista de aprovação. A política de organização `channelsEnabled` ainda se aplica. Não a use para executar channels de fontes não confiáveis.
</Note>

<h2 id="server-options">
  Opções de servidor
</h2>

Um channel define essas opções no construtor [`Server`](https://modelcontextprotocol.io/docs/concepts/servers). Os campos `instructions` e `capabilities.tools` são [MCP padrão](https://modelcontextprotocol.io/docs/concepts/servers); `capabilities.experimental['claude/channel']` e `capabilities.experimental['claude/channel/permission']` são as adições específicas de channel:

| Campo                                                    | Tipo     | Descrição                                                                                                                                                                                                                                                                                                                              |
| :------------------------------------------------------- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capabilities.experimental['claude/channel']`            | `object` | Obrigatório. Sempre `{}`. A presença registra o listener de notificação.                                                                                                                                                                                                                                                               |
| `capabilities.experimental['claude/channel/permission']` | `object` | Opcional. Sempre `{}`. Declara que este channel pode receber solicitações de retransmissão de permissão. Quando declarado, Claude Code encaminha prompts de aprovação de ferramentas para seu channel para que você possa aprová-los ou negá-los remotamente. Consulte [Retransmitir prompts de permissão](#relay-permission-prompts). |
| `capabilities.tools`                                     | `object` | Apenas bidirecional. Sempre `{}`. Capacidade de ferramenta MCP padrão. Consulte [Expor uma ferramenta de resposta](#expose-a-reply-tool).                                                                                                                                                                                              |
| `instructions`                                           | `string` | Recomendado. Adicionado ao prompt do sistema de Claude. Diga a Claude quais eventos esperar, o que os atributos da tag `<channel>` significam, se deve responder e, se sim, qual ferramenta usar e qual atributo passar de volta (como `chat_id`).                                                                                     |

Para criar um channel unidirecional, omita `capabilities.tools`. Este exemplo mostra uma configuração bidirecional com a capacidade de channel, ferramentas e instruções definidas:

```ts theme={null}
import { Server } from '@modelcontextprotocol/sdk/server/index.js'

const mcp = new Server(
  { name: 'your-channel', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },  // registra o listener de channel
      tools: {},  // omita para channels unidirecionais
    },
    // adicionado ao prompt do sistema de Claude para que ele saiba como lidar com seus eventos
    instructions: 'Messages arrive as <channel source="your-channel" ...>. Reply with the reply tool.',
  },
)
```

Para enviar um evento, chame `mcp.notification()` com o método `notifications/claude/channel`. Os params estão na próxima seção.

<h2 id="notification-format">
  Formato de notificação
</h2>

Seu servidor emite `notifications/claude/channel` com dois params:

| Campo     | Tipo                     | Descrição                                                                                                                                                                                                                                                                                                      |
| :-------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content` | `string`                 | O corpo do evento. Entregue como o corpo da tag `<channel>`.                                                                                                                                                                                                                                                   |
| `meta`    | `Record<string, string>` | Opcional. Cada entrada torna-se um atributo na tag `<channel>` para contexto de roteamento como ID de chat, nome do remetente ou severidade de alerta. As chaves devem ser identificadores: apenas letras, dígitos e underscores. Chaves contendo hífens ou outros caracteres são silenciosamente descartadas. |

Seu servidor envia eventos chamando `mcp.notification()` na instância `Server`. Este exemplo envia um alerta de falha de CI com duas chaves meta:

```ts theme={null}
await mcp.notification({
  method: 'notifications/claude/channel',
  params: {
    content: 'build failed on main: https://ci.example.com/run/1234',
    meta: { severity: 'high', run_id: '1234' },
  },
})
```

O evento chega no contexto de Claude envolvido em uma tag `<channel>`. O atributo `source` é definido automaticamente a partir do nome configurado do seu servidor:

```text theme={null}
<channel source="your-channel" severity="high" run_id="1234">
build failed on main: https://ci.example.com/run/1234
</channel>
```

As notificações não são reconhecidas. O `await` em `mcp.notification()` é resolvido quando a mensagem é escrita no transporte, não quando Claude a processou. Se a sessão não carregou seu servidor como um channel, ou a política de organização o bloqueia, os eventos são descartados silenciosamente sem erro retornado ao seu servidor.

Se você precisar de confirmação de entrega, rastreie o estado do evento em seu servidor e exponha uma [ferramenta de resposta](#expose-a-reply-tool) que Claude possa chamar para relatar o status de volta.

Os eventos são enfileirados na sessão e processados em ordem. Se várias notificações chegarem enquanto Claude está ocupado, elas são entregues juntas no próximo turno e Claude as trata como um grupo. Para processar fluxos de eventos independentes simultaneamente, execute sessões separadas.

<h2 id="expose-a-reply-tool">
  Expor uma ferramenta de resposta
</h2>

Se seu channel é bidirecional, como uma ponte de chat em vez de um encaminhador de alerta, exponha uma [ferramenta MCP](https://modelcontextprotocol.io/docs/concepts/tools) padrão que Claude possa chamar para enviar mensagens de volta. Nada sobre o registro da ferramenta é específico de channel. Uma ferramenta de resposta tem três componentes:

1. Uma entrada `tools: {}` em suas capacidades do construtor `Server` para que Claude Code descubra a ferramenta
2. Manipuladores de ferramentas que definem o esquema da ferramenta e implementam a lógica de envio
3. Uma string `instructions` em seu construtor `Server` que diz a Claude quando e como chamar a ferramenta

Para adicionar estes ao [receptor de webhook acima](#example-build-a-webhook-receiver):

<Steps>
  <Step title="Habilitar descoberta de ferramentas">
    Em seu construtor `Server` em `webhook.ts`, adicione `tools: {}` às capacidades para que Claude Code saiba que seu servidor oferece ferramentas:

    ```ts theme={null}
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},  // habilita descoberta de ferramentas
    },
    ```
  </Step>

  <Step title="Registrar a ferramenta de resposta">
    Adicione o seguinte a `webhook.ts`. O `import` vai no topo do arquivo com seus outros imports; os dois manipuladores vão entre o construtor `Server` e `mcp.connect()`. Isto registra uma ferramenta `reply` que Claude pode chamar com um `chat_id` e `text`:

    ```ts theme={null}
    // Adicione este import no topo de webhook.ts
    import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

    // Claude consulta isto na inicialização para descobrir quais ferramentas seu servidor oferece
    mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [{
        name: 'reply',
        description: 'Send a message back over this channel',
        // inputSchema diz a Claude quais argumentos passar
        inputSchema: {
          type: 'object',
          properties: {
            chat_id: { type: 'string', description: 'The conversation to reply in' },
            text: { type: 'string', description: 'The message to send' },
          },
          required: ['chat_id', 'text'],
        },
      }],
    }))

    // Claude chama isto quando quer invocar uma ferramenta
    mcp.setRequestHandler(CallToolRequestSchema, async req => {
      if (req.params.name === 'reply') {
        const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
        // send() é sua saída: POST para sua plataforma de chat, ou para teste local
        // o broadcast SSE mostrado no exemplo completo abaixo.
        send(`Reply to ${chat_id}: ${text}`)
        return { content: [{ type: 'text', text: 'sent' }] }
      }
      throw new Error(`unknown tool: ${req.params.name}`)
    })
    ```
  </Step>

  <Step title="Atualizar as instruções">
    Atualize a string `instructions` em seu construtor `Server` para que Claude saiba rotear respostas de volta através da ferramenta. Este exemplo diz a Claude para passar `chat_id` da tag de entrada:

    ```ts theme={null}
    instructions: 'Messages arrive as <channel source="webhook" chat_id="...">. Reply with the reply tool, passing the chat_id from the tag.'
    ```
  </Step>
</Steps>

Aqui está o `webhook.ts` completo com suporte bidirecional. Respostas de saída fluem sobre `GET /events` usando [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE), então `curl -N localhost:8788/events` pode observá-las ao vivo; chat de entrada chega em `POST /`:

```ts title="Full webhook.ts with reply tool' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

// --- Outbound: write to any curl -N listeners on /events ---
// A real bridge would POST to your chat platform instead.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},
    },
    instructions: 'Messages arrive as <channel source="webhook" chat_id="...">. Reply with the reply tool, passing the chat_id from the tag.',
  },
)

mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Send a message back over this channel',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'The conversation to reply in' },
        text: { type: 'string', description: 'The message to send' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

await mcp.connect(new StdioServerTransport())

let nextId = 1
Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // don't close idle SSE streams
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: SSE stream so curl -N can watch Claude's replies live
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // so curl shows something immediately
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // POST: forward to Claude as a channel event
    const body = await req.text()
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: {
        content: body,
        meta: { chat_id, path: url.pathname, method: req.method },
      },
    })
    return new Response('ok')
  },
})
```

O [servidor fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) mostra um exemplo mais completo com anexos de arquivo e edição de mensagens.

<h2 id="gate-inbound-messages">
  Gate de mensagens de entrada
</h2>

Um channel sem gate é um vetor de injeção de prompt. Qualquer pessoa que possa alcançar seu endpoint pode colocar texto na frente de Claude. Um channel escutando uma plataforma de chat ou um endpoint público precisa de uma verificação real de remetente antes de emitir qualquer coisa.

Verifique o remetente contra uma lista de permissão antes de chamar `mcp.notification()`. Este exemplo descarta qualquer mensagem de um remetente não no conjunto:

```ts theme={null}
const allowed = new Set(loadAllowlist())  // from your access.json or equivalent

// inside your message handler, before emitting:
if (!allowed.has(message.from.id)) {  // sender, not room
  return  // drop silently
}
await mcp.notification({ ... })
```

Gate na identidade do remetente, não na identidade do chat ou sala: `message.from.id` no exemplo, não `message.chat.id`. Em chats em grupo, estes diferem, e fazer gate na sala deixaria qualquer pessoa em um grupo com lista de permissão injetar mensagens na sessão.

Os channels [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) e [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) fazem gate em uma lista de permissão de remetente da mesma forma. Eles inicializam a lista por emparelhamento: o usuário envia uma DM para o bot, o bot responde com um código de emparelhamento, o usuário o aprova em sua sessão Claude Code, e seu ID de plataforma é adicionado. Consulte qualquer implementação para o fluxo de emparelhamento completo. O channel [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) toma uma abordagem diferente: detecta os próprios endereços do usuário do banco de dados Messages na inicialização e os deixa passar automaticamente, com outros remetentes adicionados por handle.

<h2 id="relay-permission-prompts">
  Retransmitir prompts de permissão
</h2>

Quando Claude chama uma ferramenta que precisa de aprovação, o diálogo do terminal local abre e a sessão aguarda. Um channel bidirecional pode optar por receber o mesmo prompt em paralelo e retransmiti-lo para você em outro dispositivo. Ambos permanecem ativos: você pode responder no terminal ou no seu telefone, e Claude Code aplica qualquer resposta que chegar primeiro e fecha a outra.

A retransmissão cobre aprovações de uso de ferramentas como `Bash`, `Write` e `Edit`. Diálogos de confiança de projeto e consentimento de servidor MCP não retransmitem; esses aparecem apenas no terminal local.

<h3 id="how-relay-works">
  Como a retransmissão funciona
</h3>

Quando um prompt de permissão abre, o loop de retransmissão tem quatro etapas:

1. Claude Code gera um ID de solicitação curto e notifica seu servidor
2. Seu servidor encaminha o prompt e o ID para seu aplicativo de chat
3. O usuário remoto responde com um sim ou não e esse ID
4. Seu manipulador de entrada analisa a resposta em um veredicto, e Claude Code o aplica apenas se o ID corresponder a uma solicitação aberta

O diálogo do terminal local permanece aberto durante tudo isso. Se alguém no terminal responder antes do veredicto remoto chegar, essa resposta é aplicada em vez disso e a solicitação remota pendente é descartada.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-permission-relay.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=97d57f128f0da55f105ab1e3a7e10240" alt="Diagrama de sequência: Claude Code envia uma notificação permission_request para o servidor de channel, o servidor formata e envia o prompt para o aplicativo de chat, o humano responde com um veredicto, e o servidor analisa essa resposta em uma notificação de permissão de volta para Claude Code" width="600" height="230" data-path="images/channel-permission-relay.svg" />

<h3 id="permission-request-fields">
  Campos de solicitação de permissão
</h3>

A notificação de saída de Claude Code é `notifications/claude/channel/permission_request`. Como a [notificação de channel](#notification-format), o transporte é MCP padrão mas o método e esquema são extensões de Claude Code. O objeto `params` tem quatro campos de string que seu servidor formata no prompt de saída:

| Campo           | Descrição                                                                                                                                                                                                                                                                                                                                                                           |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `request_id`    | Cinco letras minúsculas extraídas de `a`-`z` sem `l`, para que nunca leia como `1` ou `I` quando digitado em um telefone. Inclua-o em seu prompt de saída para que possa ser ecoado na resposta. Claude Code apenas aceita um veredicto que carregue um ID que emitiu. O diálogo do terminal local não exibe este ID, então seu manipulador de saída é a única maneira de aprender. |
| `tool_name`     | Nome da ferramenta que Claude quer usar, por exemplo `Bash` ou `Write`.                                                                                                                                                                                                                                                                                                             |
| `description`   | Resumo legível por humanos do que esta chamada de ferramenta específica faz, o mesmo texto que o diálogo do terminal local mostra. Para uma chamada Bash isto é a descrição de Claude do comando, ou o comando em si se nenhum foi dado.                                                                                                                                            |
| `input_preview` | Os argumentos da ferramenta como uma string JSON, truncada para 200 caracteres. Para Bash isto é o comando; para Write é o caminho do arquivo e um prefixo do conteúdo. Omita-o do seu prompt se você tiver espaço apenas para uma mensagem de uma linha. Seu servidor decide o que mostrar.                                                                                        |

O veredicto que seu servidor envia de volta é `notifications/claude/channel/permission` com dois campos: `request_id` ecoando o ID acima, e `behavior` definido como `'allow'` ou `'deny'`. Allow deixa a chamada de ferramenta prosseguir; deny a rejeita, o mesmo que responder Não no diálogo local. Nenhum veredicto afeta chamadas futuras.

<h3 id="add-relay-to-a-chat-bridge">
  Adicionar retransmissão a uma ponte de chat
</h3>

Adicionar retransmissão de permissão a um channel bidirecional leva três componentes:

1. Uma entrada `claude/channel/permission: {}` sob capacidades `experimental` em seu construtor `Server` para que Claude Code saiba encaminhar prompts
2. Um manipulador de notificação para `notifications/claude/channel/permission_request` que formata o prompt e o envia através da API da sua plataforma
3. Uma verificação em seu manipulador de mensagem de entrada que reconhece `yes <id>` ou `no <id>` e emite uma notificação de veredicto `notifications/claude/channel/permission` em vez de encaminhar o texto para Claude

Apenas declare a capacidade se seu channel [autentica o remetente](#gate-inbound-messages), porque qualquer pessoa que possa responder através do seu channel pode aprovar ou negar o uso de ferramentas em sua sessão.

Para adicionar estes a uma ponte de chat bidirecional como a montada em [Expor uma ferramenta de resposta](#expose-a-reply-tool):

<Steps>
  <Step title="Declarar a capacidade de permissão">
    Em seu construtor `Server`, adicione `claude/channel/permission: {}` ao lado de `claude/channel` sob `experimental`:

    ```ts theme={null}
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opt in to permission relay
      },
      tools: {},
    },
    ```
  </Step>

  <Step title="Manipular a solicitação de entrada">
    Registre um manipulador de notificação entre seu construtor `Server` e `mcp.connect()`. Claude Code o chama com os [quatro campos de solicitação](#permission-request-fields) quando um diálogo de permissão abre. Seu manipulador formata o prompt para sua plataforma e inclui instruções para responder com o ID:

    ```ts theme={null}
    import { z } from 'zod'

    // setNotificationHandler roteia por z.literal no campo method,
    // então este esquema é tanto o validador quanto a chave de dispatch
    const PermissionRequestSchema = z.object({
      method: z.literal('notifications/claude/channel/permission_request'),
      params: z.object({
        request_id: z.string(),     // cinco letras minúsculas, inclua verbatim em seu prompt
        tool_name: z.string(),      // ex: "Bash", "Write"
        description: z.string(),    // resumo legível por humanos desta chamada
        input_preview: z.string(),  // args da ferramenta como JSON, truncado para ~200 chars
      }),
    })

    mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
      // send() é sua saída: POST para sua plataforma de chat, ou para teste local
      // o broadcast SSE mostrado no exemplo completo abaixo.
      send(
        `Claude wants to run ${params.tool_name}: ${params.description}\n\n` +
        // o ID na instrução é o que seu manipulador de entrada analisa na Etapa 3
        `Reply "yes ${params.request_id}" or "no ${params.request_id}"`,
      )
    })
    ```
  </Step>

  <Step title="Interceptar o veredicto em seu manipulador de entrada">
    Seu manipulador de entrada é o loop ou callback que recebe mensagens de sua plataforma: o mesmo lugar onde você [faz gate no remetente](#gate-inbound-messages) e emite `notifications/claude/channel` para encaminhar chat para Claude. Adicione uma verificação antes da chamada de encaminhamento de chat que reconhece o formato de veredicto e emite a notificação de permissão em vez disso.

    A regex corresponde ao formato de ID que Claude Code gera: cinco letras, nunca `l`. A flag `/i` tolera autocorreção de telefone capitalizando a resposta; minúscula o ID capturado antes de enviá-lo de volta.

    ```ts theme={null}
    // corresponde a "y abcde", "yes abcde", "n abcde", "no abcde"
    // [a-km-z] é o alfabeto de ID que Claude Code usa (minúscula, pula 'l')
    // /i tolera autocorreção de telefone; minúscula a captura antes de enviar
    const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i

    async function onInbound(message: PlatformMessage) {
      if (!allowed.has(message.from.id)) return  // gate on sender first

      const m = PERMISSION_REPLY_RE.exec(message.text)
      if (m) {
        // m[1] é a palavra de veredicto, m[2] é o ID de solicitação
        // emita a notificação de veredicto de volta para Claude Code em vez de chat
        await mcp.notification({
          method: 'notifications/claude/channel/permission',
          params: {
            request_id: m[2].toLowerCase(),  // normalize in case of autocorrect caps
            behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
          },
        })
        return  // handled as verdict, don't also forward as chat
      }

      // didn't match verdict format: fall through to the normal chat path
      await mcp.notification({
        method: 'notifications/claude/channel',
        params: { content: message.text, meta: { chat_id: String(message.chat.id) } },
      })
    }
    ```
  </Step>
</Steps>

Claude Code também mantém o diálogo do terminal local aberto, para que você possa responder em qualquer lugar, e a primeira resposta a chegar é aplicada. Uma resposta remota que não corresponde exatamente ao formato esperado falha de uma de duas maneiras, e em ambos os casos o diálogo permanece aberto:

* **Formato diferente**: a regex do seu manipulador de entrada falha em corresponder, então texto como `approve it` ou `yes` sem um ID cai como uma mensagem normal para Claude.
* **Formato correto, ID errado**: seu servidor emite um veredicto, mas Claude Code não encontra nenhuma solicitação aberta com esse ID e o descarta silenciosamente.

<h3 id="full-example">
  Exemplo completo
</h3>

O `webhook.ts` montado abaixo combina todas as três extensões desta página: a ferramenta de resposta, gating de remetente e retransmissão de permissão. Se você está começando aqui, você também precisará da [configuração de projeto e entrada `.mcp.json`](#example-build-a-webhook-receiver) do passo a passo inicial.

Para tornar ambas as direções testáveis a partir de curl, o listener HTTP serve dois caminhos:

* **`GET /events`**: mantém um stream SSE aberto e envia cada mensagem de saída como uma linha `data:`, então `curl -N` pode observar as respostas de Claude e qualquer prompt de permissão conforme eles disparam ao vivo.
* **`POST /`**: o lado de entrada, o mesmo manipulador de antes, agora com a verificação de formato de veredicto inserida antes do ramo de encaminhamento de chat.

```ts title="Full webhook.ts with permission relay' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'
import { z } from 'zod'

// --- Outbound: write to any curl -N listeners on /events ---
// A real bridge would POST to your chat platform instead.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

// Sender allowlist. For the local walkthrough we trust the single X-Sender
// header value "dev"; a real bridge would check the platform's user ID.
const allowed = new Set(['dev'])

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opt in to permission relay
      },
      tools: {},
    },
    instructions:
      'Messages arrive as <channel source="webhook" chat_id="...">. ' +
      'Reply with the reply tool, passing the chat_id from the tag.',
  },
)

// --- reply tool: Claude calls this to send a message back ---
mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Send a message back over this channel',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'The conversation to reply in' },
        text: { type: 'string', description: 'The message to send' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

// --- permission relay: Claude Code (not Claude) calls this when a dialog opens
const PermissionRequestSchema = z.object({
  method: z.literal('notifications/claude/channel/permission_request'),
  params: z.object({
    request_id: z.string(),
    tool_name: z.string(),
    description: z.string(),
    input_preview: z.string(),
  }),
})

mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
  send(
    `Claude wants to run ${params.tool_name}: ${params.description}\n\n` +
    `Reply "yes ${params.request_id}" or "no ${params.request_id}"`,
  )
})

await mcp.connect(new StdioServerTransport())

// --- HTTP on :8788: GET /events streams outbound, POST routes inbound ---
const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i
let nextId = 1

Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // don't close idle SSE streams
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: SSE stream so curl -N can watch replies and prompts live
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // so curl shows something immediately
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // everything else is inbound: gate on sender first
    const body = await req.text()
    const sender = req.headers.get('X-Sender') ?? ''
    if (!allowed.has(sender)) return new Response('forbidden', { status: 403 })

    // check for verdict format before treating as chat
    const m = PERMISSION_REPLY_RE.exec(body)
    if (m) {
      await mcp.notification({
        method: 'notifications/claude/channel/permission',
        params: {
          request_id: m[2].toLowerCase(),
          behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
        },
      })
      return new Response('verdict recorded')
    }

    // normal chat: forward to Claude as a channel event
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: { content: body, meta: { chat_id, path: url.pathname } },
    })
    return new Response('ok')
  },
})
```

Teste o caminho de veredicto em três terminais. O primeiro é sua sessão Claude Code, iniciada com a [flag de desenvolvimento](#test-during-the-research-preview) para que ela spawne `webhook.ts`:

```bash theme={null}
claude --dangerously-load-development-channels server:webhook
```

No segundo, transmita o lado de saída para que você possa ver as respostas de Claude e qualquer prompt de permissão conforme eles disparam ao vivo:

```bash theme={null}
curl -N localhost:8788/events
```

No terceiro, envie uma mensagem que fará Claude tentar executar um comando:

```bash theme={null}
curl -d "list the files in this directory" -H "X-Sender: dev" localhost:8788
```

Listar arquivos é somente leitura, então Claude o executa sem aprovação. O diálogo de permissão abre quando Claude chama a ferramenta `reply` para enviar sua resposta de volta. O diálogo local abre em seu terminal Claude Code, e um momento depois o prompt para `mcp__webhook__reply` aparece no stream `/events`, incluindo o ID de cinco letras. Aprove-o do lado remoto:

```bash theme={null}
curl -d "yes <id>" -H "X-Sender: dev" localhost:8788
```

O diálogo local fecha, a ferramenta `reply` é executada, e a resposta de Claude chega no stream.

As três peças específicas de channel neste arquivo:

* **Capacidades** no construtor `Server`: `claude/channel` registra o listener de notificação, `claude/channel/permission` opta pela retransmissão de permissão, `tools` deixa Claude descobrir a ferramenta de resposta.
* **Caminhos de saída**: o manipulador da ferramenta `reply` é o que Claude chama para respostas conversacionais; o manipulador de notificação `PermissionRequestSchema` é o que Claude Code chama quando um diálogo de permissão abre. Ambos chamam `send()` para transmitir sobre `/events`, mas são acionados por diferentes partes do sistema.
* **Manipulador HTTP**: `GET /events` mantém um stream SSE aberto para que curl possa observar a saída ao vivo; `POST` é entrada, feita gate no cabeçalho `X-Sender`. Um corpo `yes <id>` ou `no <id>` vai para Claude Code como uma notificação de veredicto e nunca chega a Claude; qualquer outra coisa é encaminhada para Claude como um evento de channel.

<h2 id="package-as-a-plugin">
  Empacotar como um plugin
</h2>

Para tornar seu channel instalável e compartilhável, envolva-o em um [plugin](/docs/pt/plugins) e publique-o em um [marketplace](/docs/pt/plugin-marketplaces). Os usuários o instalam com `/plugin install`, então o habilitam por sessão com `--channels plugin:<name>@<marketplace>`.

Um channel publicado em seu próprio marketplace ainda precisa de `--dangerously-load-development-channels` para ser executado, já que não está na [lista de aprovação](/docs/pt/channels#supported-channels). A lista de aprovação padrão é os plugins de channel em `claude-plugins-official`, que Anthropic cura a seu critério. Os [formulários de envio no aplicativo](/docs/pt/plugins#submit-your-plugin-to-the-community-marketplace) adicionam plugins ao marketplace da comunidade, que não está na lista de aprovação de channels.

Se você está trabalhando com um contato parceiro da Anthropic, entre em contato com eles para coordenar uma listagem de marketplace oficial. Em planos Team e Enterprise, um administrador pode incluir seu plugin na lista [`allowedChannelPlugins`](/docs/pt/channels#restrict-which-channel-plugins-can-run) da organização, que substitui a lista de aprovação padrão da Anthropic.

<h2 id="see-also">
  Veja também
</h2>

* [Channels](/docs/pt/channels) para instalar e usar Telegram, Discord, iMessage ou a demo fakechat, e para habilitar channels para uma organização Team ou Enterprise
* [Implementações de channel funcionando](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) para código de servidor completo com fluxos de emparelhamento, ferramentas de resposta e anexos de arquivo
* [MCP](/docs/pt/mcp) para o protocolo subjacente que servidores de channel implementam
* [Plugins](/docs/pt/plugins) para empacotar seu channel para que os usuários possam instalá-lo com `/plugin install`
