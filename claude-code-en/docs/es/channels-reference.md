> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de canales

> Construye un servidor MCP que envíe webhooks, alertas y mensajes de chat a una sesión de Claude Code. Referencia para el contrato de canal: declaración de capacidad, eventos de notificación, herramientas de respuesta, compuerta de remitente y retransmisión de permisos.

<Note>
  Los canales están en [vista previa de investigación](/docs/es/channels#research-preview). Las organizaciones de equipo y empresa deben [habilitarlos explícitamente](/docs/es/channels#enterprise-controls).
</Note>

Un canal es un servidor MCP que envía eventos a una sesión de Claude Code para que Claude pueda reaccionar a cosas que suceden fuera de la terminal.

Puedes construir un canal unidireccional o bidireccional. Los canales unidireccionales reenvían alertas, webhooks o eventos de monitoreo para que Claude actúe sobre ellos. Los canales bidireccionales como puentes de chat también [exponen una herramienta de respuesta](#expose-a-reply-tool) para que Claude pueda enviar mensajes de vuelta. Un canal con una ruta de remitente confiable también puede optar por [retransmitir solicitudes de permiso](#relay-permission-prompts) para que puedas aprobar o denegar el uso de herramientas de forma remota.

Esta página cubre:

* [Descripción general](#overview): cómo funcionan los canales
* [Lo que necesitas](#what-you-need): requisitos y pasos generales
* [Ejemplo: construir un receptor de webhook](#example-build-a-webhook-receiver): un tutorial unidireccional mínimo
* [Opciones del servidor](#server-options): los campos del constructor
* [Formato de notificación](#notification-format): la carga útil del evento y el comportamiento de entrega
* [Exponer una herramienta de respuesta](#expose-a-reply-tool): permitir que Claude envíe mensajes de vuelta
* [Compuerta de mensajes entrantes](#gate-inbound-messages): comprobaciones de remitente para prevenir inyección de solicitudes
* [Retransmitir solicitudes de permiso](#relay-permission-prompts): reenviar solicitudes de aprobación de herramientas a canales remotos

Para usar un canal existente en lugar de construir uno, consulta [Canales](/docs/es/channels). Telegram, Discord, iMessage y fakechat se incluyen en la vista previa de investigación.

<h2 id="overview">
  Descripción general
</h2>

Un canal es un servidor [MCP](https://modelcontextprotocol.io) que se ejecuta en la misma máquina que Claude Code. Claude Code lo genera como un subproceso y se comunica a través de stdio. Tu servidor de canal es el puente entre sistemas externos y la sesión de Claude Code:

* **Plataformas de chat** (Telegram, Discord): tu complemento se ejecuta localmente y sondea la API de la plataforma en busca de nuevos mensajes. Cuando alguien envía un mensaje directo a tu bot, el complemento recibe el mensaje y lo reenvía a Claude. Sin URL que exponer.
* **Webhooks** (CI, monitoreo): tu servidor escucha en un puerto HTTP local. Los sistemas externos envían POST a ese puerto, y tu servidor envía la carga útil a Claude.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-architecture.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=9a037b7da80184ae49015c0256b21a1f" alt="Diagrama de arquitectura que muestra sistemas externos conectándose a tu servidor de canal local, que se comunica con Claude Code a través de stdio" width="600" height="220" data-path="images/channel-architecture.svg" />

<h2 id="what-you-need">
  Lo que necesitas
</h2>

El único requisito difícil es el paquete [`@modelcontextprotocol/sdk`](https://www.npmjs.com/package/@modelcontextprotocol/sdk) y un tiempo de ejecución compatible con Node.js. [Bun](https://bun.sh), [Node](https://nodejs.org) y [Deno](https://deno.com) funcionan todos. Los complementos precompilados en la vista previa de investigación usan Bun, pero tu canal no tiene que hacerlo.

Tu servidor necesita:

1. Declarar la capacidad `claude/channel` para que Claude Code registre un oyente de notificación
2. Emitir eventos `notifications/claude/channel` cuando algo suceda
3. Conectarse a través del [transporte stdio](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) (Claude Code genera tu servidor como un subproceso)

Las secciones [Opciones del servidor](#server-options) y [Formato de notificación](#notification-format) cubren cada una de estas en detalle. Consulta [Ejemplo: construir un receptor de webhook](#example-build-a-webhook-receiver) para un tutorial completo.

Durante la vista previa de investigación, los canales personalizados no están en la [lista de aprobación](/docs/es/channels#supported-channels). Usa `--dangerously-load-development-channels` para probar localmente. Consulta [Prueba durante la vista previa de investigación](#test-during-the-research-preview) para obtener detalles.

<h2 id="example-build-a-webhook-receiver">
  Ejemplo: construir un receptor de webhook
</h2>

Este tutorial construye un servidor de un solo archivo que escucha solicitudes HTTP y las reenvía a tu sesión de Claude Code. Al final, cualquier cosa que pueda enviar un POST HTTP, como una canalización de CI, una alerta de monitoreo o un comando `curl`, puede enviar eventos a Claude.

Este ejemplo usa [Bun](https://bun.sh) como tiempo de ejecución por su servidor HTTP integrado y soporte de TypeScript. Puedes usar [Node](https://nodejs.org) o [Deno](https://deno.com) en su lugar; el único requisito es el [SDK de MCP](https://www.npmjs.com/package/@modelcontextprotocol/sdk).

<Steps>
  <Step title="Crear el proyecto">
    Crea un nuevo directorio e instala el SDK de MCP:

    ```bash theme={null}
    mkdir webhook-channel && cd webhook-channel
    bun add @modelcontextprotocol/sdk
    ```
  </Step>

  <Step title="Escribir el servidor de canal">
    Crea un archivo llamado `webhook.ts`. Este es tu servidor de canal completo: se conecta a Claude Code a través de stdio y escucha POSTs HTTP en el puerto 8788. Cuando llega una solicitud, envía el cuerpo a Claude como un evento de canal.

    ```ts title="webhook.ts" theme={null}
    #!/usr/bin/env bun
    import { Server } from '@modelcontextprotocol/sdk/server/index.js'
    import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'

    // Crear el servidor MCP y declararlo como un canal
    const mcp = new Server(
      { name: 'webhook', version: '0.0.1' },
      {
        // esta clave es lo que lo hace un canal — Claude Code registra un oyente para ella
        capabilities: { experimental: { 'claude/channel': {} } },
        // agregado al mensaje del sistema de Claude para que sepa cómo manejar estos eventos
        instructions: 'Los eventos del canal webhook llegan como <channel source="webhook" ...>. Son unidireccionales: léelos y actúa, no se espera respuesta.',
      },
    )

    // Conectar a Claude Code a través de stdio (Claude Code genera este proceso)
    await mcp.connect(new StdioServerTransport())

    // Iniciar un servidor HTTP que reenvíe cada POST a Claude
    Bun.serve({
      port: 8788,  // cualquier puerto abierto funciona
      // solo localhost: nada fuera de esta máquina puede hacer POST
      hostname: '127.0.0.1',
      async fetch(req) {
        const body = await req.text()
        await mcp.notification({
          method: 'notifications/claude/channel',
          params: {
            content: body,  // se convierte en el cuerpo de la etiqueta <channel>
            // cada clave se convierte en un atributo de etiqueta, p. ej. <channel path="/" method="POST">
            meta: { path: new URL(req.url).pathname, method: req.method },
          },
        })
        return new Response('ok')
      },
    })
    ```

    El archivo hace tres cosas en orden:

    * **Configuración del servidor**: crea el servidor MCP con `claude/channel` en sus capacidades, que es lo que le dice a Claude Code que esto es un canal. La cadena [`instructions`](#server-options) va al mensaje del sistema de Claude: dile a Claude qué eventos esperar, si debe responder y cómo enrutar las respuestas si debe hacerlo.
    * **Conexión stdio**: se conecta a Claude Code a través de stdin/stdout. Esto es estándar para cualquier [servidor MCP](https://modelcontextprotocol.io/docs/concepts/transports#standard-io): Claude Code lo genera como un subproceso.
    * **Oyente HTTP**: inicia un servidor web local en el puerto 8788. Cada cuerpo POST se reenvía a Claude como un evento de canal a través de `mcp.notification()`. El `content` se convierte en el cuerpo del evento, y cada entrada `meta` se convierte en un atributo en la etiqueta `<channel>`. El oyente necesita acceso a la instancia `mcp`, por lo que se ejecuta en el mismo proceso. Podrías dividirlo en módulos separados para un proyecto más grande.
  </Step>

  <Step title="Registrar tu servidor con Claude Code">
    Agrega el servidor a tu configuración de MCP para que Claude Code sepa cómo iniciarlo. Para un `.mcp.json` a nivel de proyecto en el mismo directorio, usa una ruta relativa. Para la configuración a nivel de usuario en `~/.claude.json`, usa la ruta absoluta completa para que el servidor se pueda encontrar desde cualquier proyecto:

    ```json title=".mcp.json" theme={null}
    {
      "mcpServers": {
        "webhook": { "command": "bun", "args": ["./webhook.ts"] }
      }
    }
    ```

    Claude Code lee tu configuración de MCP al iniciar y genera cada servidor como un subproceso.
  </Step>

  <Step title="Probarlo">
    Durante la vista previa de investigación, los canales personalizados no están en la lista de aprobación, así que inicia Claude Code con la bandera de desarrollo:

    ```bash theme={null}
    claude --dangerously-load-development-channels server:webhook
    ```

    La primera vez que inicies una sesión en este proyecto, Claude Code solicita consentimiento antes de usar el nuevo servidor de `.mcp.json`. El diálogo informa "Nuevo servidor MCP encontrado en este proyecto: webhook". Selecciona **Usar este servidor MCP** para continuar.

    Cuando Claude Code se inicia, lee tu configuración de MCP, genera tu `webhook.ts` como un subproceso, y el oyente HTTP se inicia automáticamente en el puerto que configuraste (8788 en este ejemplo). No necesitas ejecutar el servidor tú mismo.

    Un aviso tenue debajo del banner de inicio confirma que el canal está registrado: `Channels (experimental) messages from server:webhook inject directly in this session · restart without --dangerously-load-development-channels to stop`.

    Si ves "bloqueado por política de organización", tu administrador de organización necesita [habilitar canales](/docs/es/channels#enterprise-controls) primero.

    En una terminal separada, simula un webhook enviando un POST HTTP con un mensaje a tu servidor. Este ejemplo envía una alerta de fallo de compilación al puerto 8788 (o el puerto que configuraste):

    ```bash theme={null}
    curl -X POST localhost:8788 -d "build failed on main: https://ci.example.com/run/1234"
    ```

    La carga útil llega a tu sesión de Claude Code como una etiqueta `<channel>`:

    ```text theme={null}
    <channel source="webhook" path="/" method="POST">build failed on main: https://ci.example.com/run/1234</channel>
    ```

    En tu terminal de Claude Code, verás que Claude recibe el mensaje y comienza a responder: leyendo archivos, ejecutando comandos o lo que sea que el mensaje requiera. Este es un canal unidireccional, por lo que Claude actúa en tu sesión pero no envía nada de vuelta a través del webhook. Para agregar respuestas, consulta [Exponer una herramienta de respuesta](#expose-a-reply-tool).

    Si el evento no llega, el diagnóstico depende de lo que `curl` devolvió:

    * **`curl` tiene éxito pero nada llega a Claude**: ejecuta `/mcp` en tu sesión para verificar el estado del servidor. "Falló al conectar" generalmente significa un error de dependencia o importación en tu archivo de servidor; consulta el registro de depuración en `~/.claude/debug/<session-id>.txt` para el seguimiento de stderr.
    * **`curl` falla con "conexión rechazada"**: el puerto no está vinculado aún o un proceso antiguo de una ejecución anterior lo está manteniendo. `lsof -i :<port>` muestra qué está escuchando; `kill` el proceso antiguo antes de reiniciar tu sesión.
  </Step>
</Steps>

El [servidor fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) extiende este patrón con una interfaz web, archivos adjuntos y una herramienta de respuesta para chat bidireccional.

<h2 id="test-during-the-research-preview">
  Prueba durante la vista previa de investigación
</h2>

Durante la vista previa de investigación, cada canal debe estar en la [lista de aprobación](/docs/es/channels#research-preview) para registrarse. La bandera de desarrollo omite la lista de aprobación para entradas específicas después de un mensaje de confirmación. Este ejemplo muestra ambos tipos de entrada:

```bash theme={null}
# Prueba de un complemento que estás desarrollando
claude --dangerously-load-development-channels plugin:yourplugin@yourmarketplace

# Prueba de un servidor .mcp.json desnudo (sin envoltura de complemento aún)
claude --dangerously-load-development-channels server:webhook
```

El bypass es por entrada. Combinar esta bandera con `--channels` no extiende el bypass a las entradas `--channels`. Durante la vista previa de investigación, la lista de aprobación es curada por Anthropic, por lo que tu canal permanece en la bandera de desarrollo mientras lo construyes y pruebas.

<Note>
  Esta bandera omite solo la lista de aprobación. La política de organización `channelsEnabled` aún se aplica. No la uses para ejecutar canales de fuentes no confiables.
</Note>

<h2 id="server-options">
  Opciones del servidor
</h2>

Un canal establece estas opciones en el constructor [`Server`](https://modelcontextprotocol.io/docs/concepts/servers). Los campos `instructions` y `capabilities.tools` son [MCP estándar](https://modelcontextprotocol.io/docs/concepts/servers); `capabilities.experimental['claude/channel']` y `capabilities.experimental['claude/channel/permission']` son las adiciones específicas del canal:

| Campo                                                    | Tipo     | Descripción                                                                                                                                                                                                                                                                                                                      |
| :------------------------------------------------------- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capabilities.experimental['claude/channel']`            | `object` | Requerido. Siempre `{}`. La presencia registra el oyente de notificación.                                                                                                                                                                                                                                                        |
| `capabilities.experimental['claude/channel/permission']` | `object` | Opcional. Siempre `{}`. Declara que este canal puede recibir solicitudes de retransmisión de permisos. Cuando se declara, Claude Code reenvía solicitudes de aprobación de herramientas a tu canal para que puedas aprobar o denegar de forma remota. Consulta [Retransmitir solicitudes de permiso](#relay-permission-prompts). |
| `capabilities.tools`                                     | `object` | Solo bidireccional. Siempre `{}`. Capacidad de herramienta MCP estándar. Consulta [Exponer una herramienta de respuesta](#expose-a-reply-tool).                                                                                                                                                                                  |
| `instructions`                                           | `string` | Recomendado. Agregado al mensaje del sistema de Claude. Dile a Claude qué eventos esperar, qué significan los atributos de la etiqueta `<channel>`, si debe responder y, si es así, qué herramienta usar y qué atributo pasar de vuelta (como `chat_id`).                                                                        |

Para crear un canal unidireccional, omite `capabilities.tools`. Este ejemplo muestra una configuración bidireccional con la capacidad de canal, herramientas e instrucciones establecidas:

```ts theme={null}
import { Server } from '@modelcontextprotocol/sdk/server/index.js'

const mcp = new Server(
  { name: 'your-channel', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },  // registra el oyente de canal
      tools: {},  // omite para canales unidireccionales
    },
    // agregado al mensaje del sistema de Claude para que sepa cómo manejar tus eventos
    instructions: 'Los mensajes llegan como <channel source="your-channel" ...>. Responde con la herramienta de respuesta.',
  },
)
```

Para enviar un evento, llama a `mcp.notification()` con el método `notifications/claude/channel`. Los parámetros están en la siguiente sección.

<h2 id="notification-format">
  Formato de notificación
</h2>

Tu servidor emite `notifications/claude/channel` con dos parámetros:

| Campo     | Tipo                     | Descripción                                                                                                                                                                                                                                                                                                                               |
| :-------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content` | `string`                 | El cuerpo del evento. Entregado como el cuerpo de la etiqueta `<channel>`.                                                                                                                                                                                                                                                                |
| `meta`    | `Record<string, string>` | Opcional. Cada entrada se convierte en un atributo en la etiqueta `<channel>` para el contexto de enrutamiento como ID de chat, nombre del remitente o severidad de alerta. Las claves deben ser identificadores: solo letras, dígitos y guiones bajos. Las claves que contienen guiones u otros caracteres se descartan silenciosamente. |

Tu servidor envía eventos llamando a `mcp.notification()` en la instancia `Server`. Este ejemplo envía una alerta de fallo de compilación con dos claves meta:

```ts theme={null}
await mcp.notification({
  method: 'notifications/claude/channel',
  params: {
    content: 'build failed on main: https://ci.example.com/run/1234',
    meta: { severity: 'high', run_id: '1234' },
  },
})
```

El evento llega en el contexto de Claude envuelto en una etiqueta `<channel>`. El atributo `source` se establece automáticamente desde el nombre configurado de tu servidor:

```text theme={null}
<channel source="your-channel" severity="high" run_id="1234">
build failed on main: https://ci.example.com/run/1234
</channel>
```

Las notificaciones no se reconocen. El `await` en `mcp.notification()` se resuelve cuando el mensaje se escribe en el transporte, no cuando Claude lo ha procesado. Si la sesión no ha cargado tu servidor como un canal, o la política de la organización lo bloquea, los eventos se descartan silenciosamente sin error devuelto a tu servidor.

Si necesitas confirmación de entrega, rastrea el estado del evento en tu servidor y expón una [herramienta de respuesta](#expose-a-reply-tool) que Claude pueda llamar para reportar el estado de vuelta.

Los eventos se colan en la sesión y se procesan en orden. Si varias notificaciones llegan mientras Claude está ocupado, se entregan juntas en el siguiente turno y Claude las maneja como un grupo. Para procesar flujos de eventos independientes de forma concurrente, ejecuta sesiones separadas.

<h2 id="expose-a-reply-tool">
  Exponer una herramienta de respuesta
</h2>

Si tu canal es bidireccional, como un puente de chat en lugar de un reenviador de alertas, expón una [herramienta MCP](https://modelcontextprotocol.io/docs/concepts/tools) estándar que Claude pueda llamar para enviar mensajes de vuelta. Nada sobre el registro de herramientas es específico del canal. Una herramienta de respuesta tiene tres componentes:

1. Una entrada `tools: {}` en las capacidades del constructor `Server` para que Claude Code descubra la herramienta
2. Manejadores de herramientas que definen el esquema de la herramienta e implementan la lógica de envío
3. Una cadena `instructions` en el constructor `Server` que le dice a Claude cuándo y cómo llamar a la herramienta

Para agregar estos al [receptor de webhook anterior](#example-build-a-webhook-receiver):

<Steps>
  <Step title="Habilitar el descubrimiento de herramientas">
    En tu constructor `Server` en `webhook.ts`, agrega `tools: {}` a las capacidades para que Claude Code sepa que tu servidor ofrece herramientas:

    ```ts theme={null}
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},  // habilita el descubrimiento de herramientas
    },
    ```
  </Step>

  <Step title="Registrar la herramienta de respuesta">
    Agrega lo siguiente a `webhook.ts`. El `import` va en la parte superior del archivo con tus otras importaciones; los dos manejadores van entre el constructor `Server` y `mcp.connect()`. Esto registra una herramienta `reply` que Claude puede llamar con un `chat_id` y `text`:

    ```ts theme={null}
    // Agrega este import en la parte superior de webhook.ts
    import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

    // Claude consulta esto al iniciar para descubrir qué herramientas ofrece tu servidor
    mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [{
        name: 'reply',
        description: 'Enviar un mensaje de vuelta a través de este canal',
        // inputSchema le dice a Claude qué argumentos pasar
        inputSchema: {
          type: 'object',
          properties: {
            chat_id: { type: 'string', description: 'La conversación en la que responder' },
            text: { type: 'string', description: 'El mensaje a enviar' },
          },
          required: ['chat_id', 'text'],
        },
      }],
    }))

    // Claude llama a esto cuando quiere invocar una herramienta
    mcp.setRequestHandler(CallToolRequestSchema, async req => {
      if (req.params.name === 'reply') {
        const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
        // send() es tu salida: POST a tu plataforma de chat, o para
        // pruebas locales la transmisión SSE mostrada en el ejemplo completo a continuación.
        send(`Reply to ${chat_id}: ${text}`)
        return { content: [{ type: 'text', text: 'sent' }] }
      }
      throw new Error(`unknown tool: ${req.params.name}`)
    })
    ```
  </Step>

  <Step title="Actualizar las instrucciones">
    Actualiza la cadena `instructions` en tu constructor `Server` para que Claude sepa enrutar las respuestas de vuelta a través de la herramienta. Este ejemplo le dice a Claude que pase `chat_id` desde la etiqueta entrante:

    ```ts theme={null}
    instructions: 'Los mensajes llegan como <channel source="webhook" chat_id="...">. Responde con la herramienta de respuesta, pasando el chat_id de la etiqueta.'
    ```
  </Step>
</Steps>

Aquí está el `webhook.ts` completo con soporte bidireccional. Las respuestas salientes se transmiten a través de `GET /events` usando [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE), por lo que `curl -N localhost:8788/events` puede verlas en vivo; el chat entrante llega en `POST /`:

```ts title="webhook.ts completo con herramienta de respuesta' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

// --- Salida: escribir a cualquier oyente curl -N en /events ---
// Un puente real haría POST a tu plataforma de chat en su lugar.
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
    instructions: 'Los mensajes llegan como <channel source="webhook" chat_id="...">. Responde con la herramienta de respuesta, pasando el chat_id de la etiqueta.',
  },
)

mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Enviar un mensaje de vuelta a través de este canal',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'La conversación en la que responder' },
        text: { type: 'string', description: 'El mensaje a enviar' },
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
  idleTimeout: 0,  // no cierres flujos SSE inactivos
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: flujo SSE para que curl -N pueda ver las respuestas de Claude en vivo
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // para que curl muestre algo inmediatamente
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // POST: reenviar a Claude como un evento de canal
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

El [servidor fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) muestra un ejemplo más completo con archivos adjuntos y edición de mensajes.

<h2 id="gate-inbound-messages">
  Compuerta de mensajes entrantes
</h2>

Un canal sin compuerta es un vector de inyección de solicitudes. Cualquiera que pueda alcanzar tu punto final puede poner texto frente a Claude. Un canal que escucha una plataforma de chat o un punto final público necesita una comprobación de remitente real antes de emitir cualquier cosa.

Comprueba el remitente contra una lista de permitidos antes de llamar a `mcp.notification()`. Este ejemplo descarta cualquier mensaje de un remitente que no esté en el conjunto:

```ts theme={null}
const allowed = new Set(loadAllowlist())  // desde tu access.json o equivalente

// dentro de tu manejador de mensajes, antes de emitir:
if (!allowed.has(message.from.id)) {  // remitente, no sala
  return  // descartar silenciosamente
}
await mcp.notification({ ... })
```

Compuerta en la identidad del remitente, no en la identidad del chat o sala: `message.from.id` en el ejemplo, no `message.chat.id`. En chats grupales, estos difieren, y la compuerta en la sala permitiría que cualquiera en un grupo permitido inyecte mensajes en la sesión.

Los canales [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) y [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) se compuertan en una lista de permitidos de remitente de la misma manera. Inician la lista por emparejamiento: el usuario envía un mensaje directo al bot, el bot responde con un código de emparejamiento, el usuario lo aprueba en su sesión de Claude Code, y su ID de plataforma se agrega. Consulta cualquiera de las implementaciones para el flujo de emparejamiento completo. El canal [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) toma un enfoque diferente: detecta las propias direcciones del usuario desde la base de datos de Mensajes al iniciar y las deja pasar automáticamente, con otros remitentes agregados por identificador.

<h2 id="relay-permission-prompts">
  Retransmitir solicitudes de permiso
</h2>

Cuando Claude llama a una herramienta que necesita aprobación, se abre el diálogo del terminal local y la sesión espera. Un canal bidireccional puede optar por recibir la misma solicitud en paralelo y retransmitirla a ti en otro dispositivo. Ambos permanecen activos: puedes responder en la terminal o en tu teléfono, y Claude Code aplica cualquiera que sea la respuesta que llegue primero y cierra la otra.

La retransmisión cubre aprobaciones de uso de herramientas como `Bash`, `Write` y `Edit`. Los diálogos de confianza del proyecto y consentimiento del servidor MCP no se retransmiten; esos solo aparecen en la terminal local.

<h3 id="how-relay-works">
  Cómo funciona la retransmisión
</h3>

Cuando se abre una solicitud de permiso, el bucle de retransmisión tiene cuatro pasos:

1. Claude Code genera un ID de solicitud corto y notifica a tu servidor
2. Tu servidor reenvía la solicitud y el ID a tu aplicación de chat
3. El usuario remoto responde con un sí o no y ese ID
4. Tu manejador entrante analiza la respuesta en un veredicto, y Claude Code lo aplica solo si el ID coincide con una solicitud abierta

El diálogo del terminal local permanece abierto durante todo esto. Si alguien en la terminal responde antes de que llegue el veredicto remoto, esa respuesta se aplica en su lugar y la solicitud remota pendiente se descarta.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-permission-relay.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=97d57f128f0da55f105ab1e3a7e10240" alt="Diagrama de secuencia: Claude Code envía una notificación permission_request al servidor de canal, el servidor formatea y envía la solicitud a la aplicación de chat, el humano responde con un veredicto, y el servidor analiza esa respuesta en una notificación de permiso de vuelta a Claude Code" width="600" height="230" data-path="images/channel-permission-relay.svg" />

<h3 id="permission-request-fields">
  Campos de solicitud de permiso
</h3>

La notificación saliente de Claude Code es `notifications/claude/channel/permission_request`. Como la [notificación de canal](#notification-format), el transporte es MCP estándar pero el método y esquema son extensiones de Claude Code. El objeto `params` tiene cuatro campos de cadena que tu servidor formatea en la solicitud saliente:

| Campo           | Descripción                                                                                                                                                                                                                                                                                                                                                                                         |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `request_id`    | Cinco letras minúsculas extraídas de `a`-`z` sin `l`, por lo que nunca se lee como `1` o `I` cuando se escribe en un teléfono. Inclúyelo en tu solicitud saliente para que pueda ser repetido en la respuesta. Claude Code solo acepta un veredicto que lleve un ID que emitió. El diálogo del terminal local no muestra este ID, por lo que tu manejador saliente es la única forma de aprenderlo. |
| `tool_name`     | Nombre de la herramienta que Claude quiere usar, por ejemplo `Bash` o `Write`.                                                                                                                                                                                                                                                                                                                      |
| `description`   | Resumen legible por humanos de lo que hace esta llamada de herramienta específica, el mismo texto que muestra el diálogo del terminal local. Para una llamada Bash esto es la descripción de Claude del comando, o el comando en sí si no se dio ninguno.                                                                                                                                           |
| `input_preview` | Los argumentos de la herramienta como una cadena JSON, truncada a 200 caracteres. Para Bash esto es el comando; para Write es la ruta del archivo y un prefijo del contenido. Omítelo de tu solicitud si solo tienes espacio para un mensaje de una línea. Tu servidor decide qué mostrar.                                                                                                          |

El veredicto que tu servidor envía de vuelta es `notifications/claude/channel/permission` con dos campos: `request_id` repitiendo el ID anterior, y `behavior` establecido en `'allow'` o `'deny'`. Permitir deja que la llamada de herramienta continúe; denegar la rechaza, lo mismo que responder No en el diálogo local. Ningún veredicto afecta llamadas futuras.

<h3 id="add-relay-to-a-chat-bridge">
  Agregar retransmisión a un puente de chat
</h3>

Agregar retransmisión de permisos a un canal bidireccional requiere tres componentes:

1. Una entrada `claude/channel/permission: {}` bajo capacidades `experimental` en tu constructor `Server` para que Claude Code sepa que debe reenviar solicitudes
2. Un manejador de notificación para `notifications/claude/channel/permission_request` que formatea la solicitud y la envía a través de tu API de plataforma
3. Una comprobación en tu manejador de mensajes entrantes que reconozca `yes <id>` o `no <id>` y emita una notificación de veredicto `notifications/claude/channel/permission` en su lugar de reenviar el texto a Claude

Solo declara la capacidad si tu canal [autentica el remitente](#gate-inbound-messages), porque cualquiera que pueda responder a través de tu canal puede aprobar o denegar el uso de herramientas en tu sesión.

Para agregar estos a un puente de chat bidireccional como el ensamblado en [Exponer una herramienta de respuesta](#expose-a-reply-tool):

<Steps>
  <Step title="Declarar la capacidad de permiso">
    En tu constructor `Server`, agrega `claude/channel/permission: {}` junto a `claude/channel` bajo `experimental`:

    ```ts theme={null}
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // optar por retransmisión de permisos
      },
      tools: {},
    },
    ```
  </Step>

  <Step title="Manejar la solicitud entrante">
    Registra un manejador de notificación entre tu constructor `Server` y `mcp.connect()`. Claude Code lo llama con los [cuatro campos de solicitud](#permission-request-fields) cuando se abre un diálogo de permiso. Tu manejador formatea la solicitud para tu plataforma e incluye instrucciones para responder con el ID:

    ```ts theme={null}
    import { z } from 'zod'

    // setNotificationHandler enruta por z.literal en el campo method,
    // por lo que este esquema es tanto el validador como la clave de envío
    const PermissionRequestSchema = z.object({
      method: z.literal('notifications/claude/channel/permission_request'),
      params: z.object({
        request_id: z.string(),     // cinco letras minúsculas, incluir verbatim en tu solicitud
        tool_name: z.string(),      // p. ej. "Bash", "Write"
        description: z.string(),    // resumen legible por humanos de esta llamada
        input_preview: z.string(),  // argumentos de herramienta como JSON, truncado a ~200 caracteres
      }),
    })

    mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
      // send() es tu salida: POST a tu plataforma de chat, o para
      // pruebas locales la transmisión SSE mostrada en el ejemplo completo a continuación.
      send(
        `Claude wants to run ${params.tool_name}: ${params.description}\n\n` +
        // el ID en la instrucción es lo que tu manejador entrante analiza en el Paso 3
        `Reply "yes ${params.request_id}" or "no ${params.request_id}"`,
      )
    })
    ```
  </Step>

  <Step title="Interceptar el veredicto en tu manejador entrante">
    Tu manejador entrante es el bucle o devolución de llamada que recibe mensajes de tu plataforma: el mismo lugar donde [compuertas en remitente](#gate-inbound-messages) y emites `notifications/claude/channel` para reenviar chat a Claude. Agrega una comprobación antes de la llamada de reenvío de chat que reconozca el formato de veredicto y emita la notificación de permiso en su lugar.

    La expresión regular coincide con el formato de ID que genera Claude Code: cinco letras, nunca `l`. La bandera `/i` tolera la corrección automática del teléfono capitalizando la respuesta; minúscula el ID capturado antes de enviarlo de vuelta.

    ```ts theme={null}
    // coincide con "y abcde", "yes abcde", "n abcde", "no abcde"
    // [a-km-z] es el alfabeto de ID que usa Claude Code (minúscula, omite 'l')
    // /i tolera la corrección automática del teléfono; minúscula la captura antes de enviar
    const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i

    async function onInbound(message: PlatformMessage) {
      if (!allowed.has(message.from.id)) return  // compuerta en remitente primero

      const m = PERMISSION_REPLY_RE.exec(message.text)
      if (m) {
        // m[1] es la palabra de veredicto, m[2] es el ID de solicitud
        // emitir la notificación de veredicto de vuelta a Claude Code en lugar de chat
        await mcp.notification({
          method: 'notifications/claude/channel/permission',
          params: {
            request_id: m[2].toLowerCase(),  // normalizar en caso de autocorrect caps
            behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
          },
        })
        return  // manejado como veredicto, no también reenviar como chat
      }

      // no coincidió con formato de veredicto: caer a través de la ruta de chat normal
      await mcp.notification({
        method: 'notifications/claude/channel',
        params: { content: message.text, meta: { chat_id: String(message.chat.id) } },
      })
    }
    ```
  </Step>
</Steps>

Claude Code también mantiene abierto el diálogo del terminal local, por lo que puedes responder en cualquier lugar, y la primera respuesta que llegue se aplica. Una respuesta remota que no coincida exactamente con el formato esperado falla de una de dos maneras, y en ambos casos el diálogo permanece abierto:

* **Formato diferente**: la expresión regular de tu manejador entrante no coincide, por lo que texto como `approve it` o `yes` sin un ID cae a través como un mensaje normal a Claude.
* **Formato correcto, ID incorrecto**: tu servidor emite un veredicto, pero Claude Code no encuentra ninguna solicitud abierta con ese ID y lo descarta silenciosamente.

<h3 id="full-example">
  Ejemplo completo
</h3>

El `webhook.ts` ensamblado a continuación combina las tres extensiones de esta página: la herramienta de respuesta, la compuerta de remitente y la retransmisión de permisos. Si estás comenzando aquí, también necesitarás la [configuración del proyecto y entrada `.mcp.json`](#example-build-a-webhook-receiver) del tutorial inicial.

Para hacer ambas direcciones comprobables desde curl, el oyente HTTP sirve dos rutas:

* **`GET /events`**: mantiene abierto un flujo SSE y envía cada mensaje saliente como una línea `data:`, por lo que `curl -N` puede ver las respuestas de Claude y cualquier solicitud de permiso llegar en vivo.
* **`POST /`**: el lado entrante, el mismo manejador que antes, ahora con la comprobación de formato de veredicto insertada antes de la rama de reenvío de chat.

```ts title="webhook.ts completo con retransmisión de permisos' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'
import { z } from 'zod'

// --- Salida: escribir a cualquier oyente curl -N en /events ---
// Un puente real haría POST a tu plataforma de chat en su lugar.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

// Lista de permitidos de remitente. Para el tutorial local confiamos en el valor de encabezado único X-Sender
// "dev"; un puente real verificaría el ID de usuario de la plataforma.
const allowed = new Set(['dev'])

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // optar por retransmisión de permisos
      },
      tools: {},
    },
    instructions:
      'Los mensajes llegan como <channel source="webhook" chat_id="...">. ' +
      'Responde con la herramienta de respuesta, pasando el chat_id de la etiqueta.',
  },
)

// --- herramienta de respuesta: Claude llama a esto para enviar un mensaje de vuelta ---
mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Enviar un mensaje de vuelta a través de este canal',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'La conversación en la que responder' },
        text: { type: 'string', description: 'El mensaje a enviar' },
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

// --- retransmisión de permisos: Claude Code (no Claude) llama a esto cuando se abre un diálogo
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

// --- HTTP en :8788: GET /events transmite salida, POST enruta entrada ---
const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i
let nextId = 1

Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // no cierres flujos SSE inactivos
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: flujo SSE para que curl -N pueda ver respuestas y solicitudes en vivo
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // para que curl muestre algo inmediatamente
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // todo lo demás es entrada: compuerta en remitente primero
    const body = await req.text()
    const sender = req.headers.get('X-Sender') ?? ''
    if (!allowed.has(sender)) return new Response('forbidden', { status: 403 })

    // comprueba el formato de veredicto antes de tratar como chat
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

    // chat normal: reenviar a Claude como un evento de canal
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: { content: body, meta: { chat_id, path: url.pathname } },
    })
    return new Response('ok')
  },
})
```

Prueba la ruta de veredicto en tres terminales. La primera es tu sesión de Claude Code, iniciada con la [bandera de desarrollo](#test-during-the-research-preview) para que genere `webhook.ts`:

```bash theme={null}
claude --dangerously-load-development-channels server:webhook
```

En la segunda, transmite el lado saliente para que puedas ver las respuestas de Claude y cualquier solicitud de permiso llegar en vivo:

```bash theme={null}
curl -N localhost:8788/events
```

En la tercera, envía un mensaje que hará que Claude intente ejecutar un comando:

```bash theme={null}
curl -d "list the files in this directory" -H "X-Sender: dev" localhost:8788
```

Listar archivos es de solo lectura, por lo que Claude lo ejecuta sin aprobación. El diálogo de permiso se abre cuando Claude llama a la herramienta `reply` para enviar su respuesta de vuelta. El diálogo local se abre en tu terminal de Claude Code, y un momento después la solicitud para `mcp__webhook__reply` aparece en el flujo `/events`, incluyendo el ID de cinco letras. Apruébalo desde el lado remoto:

```bash theme={null}
curl -d "yes <id>" -H "X-Sender: dev" localhost:8788
```

El diálogo local se cierra, la herramienta `reply` se ejecuta, y la respuesta de Claude llega al flujo.

Las tres piezas específicas del canal en este archivo:

* **Capacidades** en el constructor `Server`: `claude/channel` registra el oyente de notificación, `claude/channel/permission` opta por retransmisión de permisos, `tools` permite que Claude descubra la herramienta de respuesta.
* **Rutas salientes**: el manejador de la herramienta `reply` es lo que Claude llama para respuestas conversacionales; el manejador de notificación `PermissionRequestSchema` es lo que Claude Code llama cuando se abre un diálogo de permiso. Ambos llaman a `send()` para transmitir a través de `/events`, pero se activan por diferentes partes del sistema.
* **Manejador HTTP**: `GET /events` mantiene abierto un flujo SSE para que curl pueda ver la salida en vivo; `POST` es entrada, compuerta en el encabezado `X-Sender`. Un cuerpo `yes <id>` o `no <id>` va a Claude Code como una notificación de veredicto y nunca llega a Claude; cualquier otra cosa se reenvía a Claude como un evento de canal.

<h2 id="package-as-a-plugin">
  Empaquetar como un complemento
</h2>

Para hacer tu canal instalable y compartible, envuélvelo en un [complemento](/docs/es/plugins) y publícalo en un [mercado](/docs/es/plugin-marketplaces). Los usuarios lo instalan con `/plugin install`, luego lo habilitan por sesión con `--channels plugin:<name>@<marketplace>`.

Un canal publicado en tu propio mercado aún necesita `--dangerously-load-development-channels` para ejecutarse, ya que no está en la [lista de aprobación](/docs/es/channels#supported-channels). La lista de aprobación predeterminada es los complementos de canal en `claude-plugins-official`, que Anthropic cura a su discreción. Los [formularios de envío en la aplicación](/docs/es/plugins#submit-your-plugin-to-the-community-marketplace) agregan complementos al mercado comunitario, que no está en la lista de aprobación de canales.

Si estás trabajando con un contacto de socio de Anthropic, comunícate con ellos para coordinar una lista de mercado oficial. En planes de equipo y empresa, un administrador puede incluir tu complemento en la lista [`allowedChannelPlugins`](/docs/es/channels#restrict-which-channel-plugins-can-run) de la organización, que reemplaza la lista de aprobación predeterminada de Anthropic.

<h2 id="see-also">
  Ver también
</h2>

* [Canales](/docs/es/channels) para instalar y usar Telegram, Discord, iMessage o la demostración fakechat, y para habilitar canales para una organización de equipo o empresa
* [Implementaciones de canal de trabajo](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) para código de servidor completo con flujos de emparejamiento, herramientas de respuesta y archivos adjuntos
* [MCP](/docs/es/mcp) para el protocolo subyacente que implementan los servidores de canal
* [Plugins](/docs/es/plugins) para empaquetar su canal para que los usuarios puedan instalarlo con `/plugin install`
