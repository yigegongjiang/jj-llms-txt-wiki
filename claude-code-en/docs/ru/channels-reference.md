> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник по каналам

> Создайте MCP-сервер, который отправляет вебхуки, оповещения и сообщения чата в сеанс Claude Code. Справочник по контракту канала: объявление возможностей, события уведомлений, инструменты ответа, проверка отправителя и трансляция разрешений.

<Note>
  Каналы находятся в [исследовательском превью](/docs/ru/channels#research-preview). Организации Team и Enterprise должны [явно их включить](/docs/ru/channels#enterprise-controls).
</Note>

Канал — это MCP-сервер, который отправляет события в сеанс Claude Code, чтобы Claude мог реагировать на события, происходящие вне терминала.

Вы можете создать односторонний или двусторонний канал. Односторонние каналы пересылают оповещения, вебхуки или события мониторинга для действия Claude. Двусторонние каналы, такие как мосты чата, также [предоставляют инструмент ответа](#expose-a-reply-tool), чтобы Claude мог отправлять сообщения обратно. Канал с доверенным путём отправителя также может согласиться на [трансляцию запросов разрешений](#relay-permission-prompts), чтобы вы могли одобрять или отклонять использование инструментов удалённо.

На этой странице рассматривается:

* [Обзор](#overview): как работают каналы
* [Что вам нужно](#what-you-need): требования и общие шаги
* [Пример: создание приёмника вебхуков](#example-build-a-webhook-receiver): минимальное пошаговое руководство в одну сторону
* [Параметры сервера](#server-options): поля конструктора
* [Формат уведомления](#notification-format): полезная нагрузка события и поведение доставки
* [Предоставление инструмента ответа](#expose-a-reply-tool): позволить Claude отправлять сообщения обратно
* [Проверка входящих сообщений](#gate-inbound-messages): проверки отправителя для предотвращения инъекции подсказок
* [Трансляция запросов разрешений](#relay-permission-prompts): пересылка запросов одобрения инструментов на удалённые каналы

Чтобы использовать существующий канал вместо создания собственного, см. [Каналы](/docs/ru/channels). Telegram, Discord, iMessage и fakechat включены в исследовательское превью.

<h2 id="overview">
  Обзор
</h2>

Канал — это [MCP](https://modelcontextprotocol.io) сервер, который работает на той же машине, что и Claude Code. Claude Code запускает его как подпроцесс и взаимодействует через stdio. Ваш сервер канала — это мост между внешними системами и сеансом Claude Code:

* **Платформы чата** (Telegram, Discord): ваш плагин работает локально и опрашивает API платформы на предмет новых сообщений. Когда кто-то отправляет личное сообщение вашему боту, плагин получает сообщение и пересылает его Claude. Нет необходимости в URL для открытия.
* **Вебхуки** (CI, мониторинг): ваш сервер прослушивает локальный HTTP-порт. Внешние системы отправляют POST на этот порт, и ваш сервер отправляет полезную нагрузку Claude.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-architecture.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=9a037b7da80184ae49015c0256b21a1f" alt="Диаграмма архитектуры, показывающая внешние системы, подключающиеся к вашему локальному серверу канала, который взаимодействует с Claude Code через stdio" width="600" height="220" data-path="images/channel-architecture.svg" />

<h2 id="what-you-need">
  Что вам нужно
</h2>

Единственное жёсткое требование — это пакет [`@modelcontextprotocol/sdk`](https://www.npmjs.com/package/@modelcontextprotocol/sdk) и совместимая с Node.js среда выполнения. [Bun](https://bun.sh), [Node](https://nodejs.org) и [Deno](https://deno.com) работают. Предварительно созданные плагины в исследовательском превью используют Bun, но ваш канал не обязательно должен.

Ваш сервер должен:

1. Объявить возможность `claude/channel`, чтобы Claude Code зарегистрировал слушатель уведомлений
2. Отправлять события `notifications/claude/channel` при возникновении чего-либо
3. Подключаться через [транспорт stdio](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) (Claude Code запускает ваш сервер как подпроцесс)

Разделы [Параметры сервера](#server-options) и [Формат уведомления](#notification-format) подробно рассматривают каждый из них. Полное пошаговое руководство см. в [Пример: создание приёмника вебхуков](#example-build-a-webhook-receiver).

Во время исследовательского превью пользовательские каналы не находятся в [одобренном списке разрешений](/docs/ru/channels#supported-channels). Используйте `--dangerously-load-development-channels` для локального тестирования. Подробности см. в [Тестирование во время исследовательского превью](#test-during-the-research-preview).

<h2 id="example-build-a-webhook-receiver">
  Пример: создание приёмника вебхуков
</h2>

Это пошаговое руководство создаёт однофайловый сервер, который прослушивает HTTP-запросы и пересылает их в ваш сеанс Claude Code. В конце концов, всё, что может отправить HTTP POST, например конвейер CI, оповещение мониторинга или команда `curl`, может отправлять события Claude.

В этом примере используется [Bun](https://bun.sh) в качестве среды выполнения благодаря встроенному HTTP-серверу и поддержке TypeScript. Вы можете использовать [Node](https://nodejs.org) или [Deno](https://deno.com); единственное требование — это [MCP SDK](https://www.npmjs.com/package/@modelcontextprotocol/sdk).

<Steps>
  <Step title="Создание проекта">
    Создайте новый каталог и установите MCP SDK:

    ```bash theme={null}
    mkdir webhook-channel && cd webhook-channel
    bun add @modelcontextprotocol/sdk
    ```
  </Step>

  <Step title="Написание сервера канала">
    Создайте файл с именем `webhook.ts`. Это ваш полный сервер канала: он подключается к Claude Code через stdio и прослушивает HTTP POST на порту 8788. Когда приходит запрос, он отправляет тело Claude как событие канала.

    ```ts title="webhook.ts" theme={null}
    #!/usr/bin/env bun
    import { Server } from '@modelcontextprotocol/sdk/server/index.js'
    import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'

    // Создание MCP-сервера и объявление его как канала
    const mcp = new Server(
      { name: 'webhook', version: '0.0.1' },
      {
        // этот ключ делает его каналом — Claude Code регистрирует слушатель для него
        capabilities: { experimental: { 'claude/channel': {} } },
        // добавляется в системную подсказку Claude, чтобы он знал, как обрабатывать эти события
        instructions: 'Events from the webhook channel arrive as <channel source="webhook" ...>. They are one-way: read them and act, no reply expected.',
      },
    )

    // Подключение к Claude Code через stdio (Claude Code запускает этот процесс)
    await mcp.connect(new StdioServerTransport())

    // Запуск HTTP-сервера, который пересылает каждый POST Claude
    Bun.serve({
      port: 8788,  // любой открытый порт работает
      // только localhost: ничто вне этой машины не может отправить POST
      hostname: '127.0.0.1',
      async fetch(req) {
        const body = await req.text()
        await mcp.notification({
          method: 'notifications/claude/channel',
          params: {
            content: body,  // становится телом тега <channel>
            // каждый ключ становится атрибутом тега, например <channel path="/" method="POST">
            meta: { path: new URL(req.url).pathname, method: req.method },
          },
        })
        return new Response('ok')
      },
    })
    ```

    Файл делает три вещи по порядку:

    * **Конфигурация сервера**: создаёт MCP-сервер с `claude/channel` в его возможностях, что говорит Claude Code, что это канал. Строка [`instructions`](#server-options) переходит в системную подсказку Claude: скажите Claude, какие события ожидать, нужно ли отвечать и как маршрутизировать ответы, если нужно.
    * **Подключение stdio**: подключается к Claude Code через stdin/stdout. Это стандартно для любого [MCP-сервера](https://modelcontextprotocol.io/docs/concepts/transports#standard-io): Claude Code запускает его как подпроцесс.
    * **Слушатель HTTP**: запускает локальный веб-сервер на порту 8788. Каждое тело POST пересылается Claude как событие канала через `mcp.notification()`. `content` становится телом события, а каждая запись `meta` становится атрибутом на теге `<channel>`. Слушателю нужен доступ к экземпляру `mcp`, поэтому он работает в том же процессе. Вы можете разделить его на отдельные модули для более крупного проекта.
  </Step>

  <Step title="Регистрация вашего сервера с Claude Code">
    Добавьте сервер в вашу конфигурацию MCP, чтобы Claude Code знал, как его запустить. Для проекта `.mcp.json` в том же каталоге используйте относительный путь. Для конфигурации уровня пользователя в `~/.claude.json` используйте полный абсолютный путь, чтобы сервер можно было найти из любого проекта:

    ```json title=".mcp.json" theme={null}
    {
      "mcpServers": {
        "webhook": { "command": "bun", "args": ["./webhook.ts"] }
      }
    }
    ```

    Claude Code читает вашу конфигурацию MCP при запуске и запускает каждый сервер как подпроцесс.
  </Step>

  <Step title="Тестирование">
    Во время исследовательского превью пользовательские каналы не находятся в списке разрешений, поэтому запустите Claude Code с флагом разработки:

    ```bash theme={null}
    claude --dangerously-load-development-channels server:webhook
    ```

    Первый раз, когда вы запускаете сеанс в этом проекте, Claude Code запрашивает согласие перед использованием нового сервера из `.mcp.json`. Диалог сообщает "New MCP server found in this project: webhook". Выберите **Use this MCP server**, чтобы продолжить.

    Когда Claude Code запускается, он читает вашу конфигурацию MCP, запускает ваш `webhook.ts` как подпроцесс, и слушатель HTTP автоматически запускается на настроенном вами порту (8788 в этом примере). Вам не нужно запускать сервер самостоятельно.

    Тусклое уведомление под баннером запуска подтверждает, что канал зарегистрирован: `Channels (experimental) messages from server:webhook inject directly in this session · restart without --dangerously-load-development-channels to stop`.

    Если вы видите "blocked by org policy", ваш администратор организации должен сначала [включить каналы](/docs/ru/channels#enterprise-controls).

    В отдельном терминале имитируйте вебхук, отправив HTTP POST с сообщением на ваш сервер. Этот пример отправляет оповещение об ошибке CI на порт 8788 (или любой другой настроенный вами порт):

    ```bash theme={null}
    curl -X POST localhost:8788 -d "build failed on main: https://ci.example.com/run/1234"
    ```

    Полезная нагрузка поступает в ваш сеанс Claude Code как тег `<channel>`:

    ```text theme={null}
    <channel source="webhook" path="/" method="POST">build failed on main: https://ci.example.com/run/1234</channel>
    ```

    В вашем терминале Claude Code вы увидите, как Claude получает сообщение и начинает отвечать: чтение файлов, выполнение команд или всё, что требует сообщение. Это односторонний канал, поэтому Claude действует в вашем сеансе, но ничего не отправляет обратно через вебхук. Чтобы добавить ответы, см. [Предоставление инструмента ответа](#expose-a-reply-tool).

    Если событие не поступает, диагностика зависит от того, что вернул `curl`:

    * **`curl` успешен, но ничего не достигает Claude**: запустите `/mcp` в вашем сеансе, чтобы проверить статус сервера. "Failed to connect" обычно означает ошибку зависимости или импорта в файле вашего сервера; проверьте журнал отладки в `~/.claude/debug/<session-id>.txt` для трассировки stderr.
    * **`curl` не удаётся с "connection refused"**: порт либо ещё не привязан, либо устаревший процесс из более раннего запуска его удерживает. `lsof -i :<port>` показывает, что прослушивается; `kill` устаревший процесс перед перезагрузкой вашего сеанса.
  </Step>
</Steps>

[Сервер fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) расширяет этот паттерн с веб-интерфейсом, вложениями файлов и инструментом ответа для двустороннего чата.

<h2 id="test-during-the-research-preview">
  Тестирование во время исследовательского превью
</h2>

Во время исследовательского превью каждый канал должен быть в [одобренном списке разрешений](/docs/ru/channels#research-preview) для регистрации. Флаг разработки обходит список разрешений для конкретных записей после подтверждающего запроса. Этот пример показывает оба типа записей:

```bash theme={null}
# Тестирование плагина, который вы разрабатываете
claude --dangerously-load-development-channels plugin:yourplugin@yourmarketplace

# Тестирование простого сервера .mcp.json (ещё нет обёртки плагина)
claude --dangerously-load-development-channels server:webhook
```

Обход выполняется для каждой записи. Объединение этого флага с `--channels` не распространяет обход на записи `--channels`. Во время исследовательского превью одобренный список разрешений курируется Anthropic, поэтому ваш канал остаётся на флаге разработки во время разработки и тестирования.

<Note>
  Этот флаг пропускает только список разрешений. Политика организации `channelsEnabled` по-прежнему применяется. Не используйте его для запуска каналов из ненадёжных источников.
</Note>

<h2 id="server-options">
  Параметры сервера
</h2>

Канал устанавливает эти параметры в конструкторе [`Server`](https://modelcontextprotocol.io/docs/concepts/servers). Поля `instructions` и `capabilities.tools` являются [стандартным MCP](https://modelcontextprotocol.io/docs/concepts/servers); `capabilities.experimental['claude/channel']` и `capabilities.experimental['claude/channel/permission']` — это дополнения, специфичные для канала:

| Поле                                                     | Тип      | Описание                                                                                                                                                                                                                                                                                                 |
| :------------------------------------------------------- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capabilities.experimental['claude/channel']`            | `object` | Обязательно. Всегда `{}`. Наличие регистрирует слушатель уведомлений.                                                                                                                                                                                                                                    |
| `capabilities.experimental['claude/channel/permission']` | `object` | Опционально. Всегда `{}`. Объявляет, что этот канал может получать запросы трансляции разрешений. При объявлении Claude Code пересылает запросы одобрения инструментов на ваш канал, чтобы вы могли одобрять или отклонять их удалённо. См. [Трансляция запросов разрешений](#relay-permission-prompts). |
| `capabilities.tools`                                     | `object` | Только двусторонний. Всегда `{}`. Стандартная возможность инструмента MCP. См. [Предоставление инструмента ответа](#expose-a-reply-tool).                                                                                                                                                                |
| `instructions`                                           | `string` | Рекомендуется. Добавляется в системную подсказку Claude. Скажите Claude, какие события ожидать, что означают атрибуты тега `<channel>`, нужно ли отвечать и если да, какой инструмент использовать и какой атрибут передать обратно (например `chat_id`).                                                |

Чтобы создать односторонний канал, опустите `capabilities.tools`. Этот пример показывает двустороннюю установку с объявленными возможностью канала, инструментами и инструкциями:

```ts theme={null}
import { Server } from '@modelcontextprotocol/sdk/server/index.js'

const mcp = new Server(
  { name: 'your-channel', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },  // регистрирует слушатель канала
      tools: {},  // опустите для односторонних каналов
    },
    // добавляется в системную подсказку Claude, чтобы он знал, как обрабатывать ваши события
    instructions: 'Messages arrive as <channel source="your-channel" ...>. Reply with the reply tool.',
  },
)
```

Чтобы отправить событие, вызовите `mcp.notification()` с методом `notifications/claude/channel`. Параметры находятся в следующем разделе.

<h2 id="notification-format">
  Формат уведомления
</h2>

Ваш сервер отправляет `notifications/claude/channel` с двумя параметрами:

| Поле      | Тип                      | Описание                                                                                                                                                                                                                                                                                                    |
| :-------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content` | `string`                 | Тело события. Доставляется как тело тега `<channel>`.                                                                                                                                                                                                                                                       |
| `meta`    | `Record<string, string>` | Опционально. Каждая запись становится атрибутом на теге `<channel>` для контекста маршрутизации, такого как ID чата, имя отправителя или серьёзность оповещения. Ключи должны быть идентификаторами: только буквы, цифры и подчёркивания. Ключи, содержащие дефисы или другие символы, молча отбрасываются. |

Ваш сервер отправляет события, вызывая `mcp.notification()` на экземпляре `Server`. Этот пример отправляет оповещение об ошибке CI с двумя ключами meta:

```ts theme={null}
await mcp.notification({
  method: 'notifications/claude/channel',
  params: {
    content: 'build failed on main: https://ci.example.com/run/1234',
    meta: { severity: 'high', run_id: '1234' },
  },
})
```

Событие поступает в контекст Claude, завёрнутое в тег `<channel>`. Атрибут `source` устанавливается автоматически из имени вашего сервера:

```text theme={null}
<channel source="your-channel" severity="high" run_id="1234">
build failed on main: https://ci.example.com/run/1234
</channel>
```

Уведомления не подтверждаются. `await` на `mcp.notification()` разрешается, когда сообщение записывается в транспорт, а не когда Claude его обработал. Если сеанс не загрузил ваш сервер как канал, или политика организации его блокирует, события молча отбрасываются без ошибки, возвращаемой вашему серверу.

Если вам нужно подтверждение доставки, отслеживайте состояние события на вашем сервере и предоставьте [инструмент ответа](#expose-a-reply-tool), который Claude может вызвать для сообщения статуса обратно.

События ставятся в очередь в сеанс и обрабатываются по порядку. Если несколько уведомлений поступают, пока Claude занят, они доставляются вместе на следующем ходу и Claude обрабатывает их как группу. Для обработки независимых потоков событий одновременно запустите отдельные сеансы.

<h2 id="expose-a-reply-tool">
  Предоставление инструмента ответа
</h2>

Если ваш канал двусторонний, например мост чата, а не пересылка оповещений, предоставьте стандартный [инструмент MCP](https://modelcontextprotocol.io/docs/concepts/tools), который Claude может вызвать для отправки сообщений обратно. Ничего в регистрации инструмента не является специфичным для канала. Инструмент ответа имеет три компонента:

1. Запись `tools: {}` в возможностях конструктора `Server`, чтобы Claude Code обнаружил инструмент
2. Обработчики инструментов, которые определяют схему инструмента и реализуют логику отправки
3. Строка `instructions` в конструкторе `Server`, которая говорит Claude, когда и как вызывать инструмент

Чтобы добавить их к [приёмнику вебхуков выше](#example-build-a-webhook-receiver):

<Steps>
  <Step title="Включение обнаружения инструментов">
    В конструкторе `Server` в `webhook.ts` добавьте `tools: {}` в возможности, чтобы Claude Code знал, что ваш сервер предлагает инструменты:

    ```ts theme={null}
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},  // включает обнаружение инструментов
    },
    ```
  </Step>

  <Step title="Регистрация инструмента ответа">
    Добавьте следующее в `webhook.ts`. `import` переходит в верхнюю часть файла с вашими другими импортами; два обработчика переходят между конструктором `Server` и `mcp.connect()`. Это регистрирует инструмент `reply`, который Claude может вызвать с `chat_id` и `text`:

    ```ts theme={null}
    // Добавьте этот импорт в верхнюю часть webhook.ts
    import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

    // Claude запрашивает это при запуске, чтобы обнаружить, какие инструменты предлагает ваш сервер
    mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [{
        name: 'reply',
        description: 'Send a message back over this channel',
        // inputSchema говорит Claude, какие аргументы передать
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

    // Claude вызывает это, когда хочет вызвать инструмент
    mcp.setRequestHandler(CallToolRequestSchema, async req => {
      if (req.params.name === 'reply') {
        const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
        // send() — это ваш исходящий: POST на вашу платформу чата, или для локального
        // тестирования трансляция SSE, показанная в полном примере ниже.
        send(`Reply to ${chat_id}: ${text}`)
        return { content: [{ type: 'text', text: 'sent' }] }
      }
      throw new Error(`unknown tool: ${req.params.name}`)
    })
    ```
  </Step>

  <Step title="Обновление инструкций">
    Обновите строку `instructions` в конструкторе `Server`, чтобы Claude знал маршрутизировать ответы обратно через инструмент. Этот пример говорит Claude передать `chat_id` из входящего тега:

    ```ts theme={null}
    instructions: 'Messages arrive as <channel source="webhook" chat_id="...">. Reply with the reply tool, passing the chat_id from the tag.'
    ```
  </Step>
</Steps>

Вот полный `webhook.ts` с двусторонней поддержкой. Исходящие ответы передаются через `GET /events` с использованием [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE), поэтому `curl -N localhost:8788/events` может смотреть их в реальном времени; входящий чат поступает на `POST /`:

```ts title="Full webhook.ts with reply tool' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

// --- Исходящий: запись для любых слушателей curl -N на /events ---
// Реальный мост отправлял бы POST на вашу платформу чата вместо этого.
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
  idleTimeout: 0,  // не закрывайте неактивные потоки SSE
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: поток SSE, чтобы curl -N мог смотреть ответы Claude в реальном времени
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // чтобы curl показал что-то сразу
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // POST: пересылка Claude как событие канала
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

[Сервер fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) показывает более полный пример с вложениями файлов и редактированием сообщений.

<h2 id="gate-inbound-messages">
  Проверка входящих сообщений
</h2>

Непроверенный канал — это вектор инъекции подсказок. Любой, кто может достичь вашей конечной точки, может поместить текст перед Claude. Канал, прослушивающий платформу чата или общедоступную конечную точку, нуждается в реальной проверке отправителя перед отправкой чего-либо.

Проверьте отправителя против списка разрешений перед вызовом `mcp.notification()`. Этот пример отбрасывает любое сообщение от отправителя, не входящего в набор:

```ts theme={null}
const allowed = new Set(loadAllowlist())  // из вашего access.json или эквивалента

// внутри вашего обработчика сообщений, перед отправкой:
if (!allowed.has(message.from.id)) {  // отправитель, не комната
  return  // отбросить молча
}
await mcp.notification({ ... })
```

Проверяйте по идентичности отправителя, а не по идентичности чата или комнаты: `message.from.id` в примере, а не `message.chat.id`. В групповых чатах они отличаются, и проверка по комнате позволила бы любому в разрешённой группе вводить сообщения в сеанс.

Каналы [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) и [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) проверяют список разрешений отправителя так же. Они загружают список путём спаривания: пользователь отправляет личное сообщение боту, бот отвечает кодом спаривания, пользователь одобряет его в своём сеансе Claude Code, и его ID платформы добавляется. Полный поток спаривания см. в любой реализации. Канал [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) использует другой подход: он обнаруживает собственные адреса пользователя из базы данных Messages при запуске и пропускает их автоматически, с другими отправителями, добавляемыми по дескриптору.

<h2 id="relay-permission-prompts">
  Трансляция запросов разрешений
</h2>

Когда Claude вызывает инструмент, требующий одобрения, открывается диалог локального терминала и сеанс ждёт. Двусторонний канал может согласиться получить тот же запрос параллельно и передать его вам на другое устройство. Оба остаются активными: вы можете ответить в терминале или на телефоне, и Claude Code применяет любой ответ, который поступит первым, и закрывает другой.

Трансляция охватывает одобрения использования инструментов, такие как `Bash`, `Write` и `Edit`. Диалоги доверия проекта и согласия MCP-сервера не передаются; они появляются только в локальном терминале.

<h3 id="how-relay-works">
  Как работает трансляция
</h3>

Когда открывается запрос разрешения, цикл трансляции имеет четыре шага:

1. Claude Code генерирует короткий ID запроса и уведомляет ваш сервер
2. Ваш сервер пересылает запрос и ID в ваше приложение чата
3. Удалённый пользователь отвечает да или нет и этот ID
4. Ваш входящий обработчик анализирует ответ в вердикт, и Claude Code применяет его только если ID совпадает с открытым запросом

Диалог локального терминала остаётся открытым на протяжении всего этого. Если кто-то в терминале ответит перед поступлением удалённого вердикта, этот ответ применяется вместо этого и ожидающий удалённый запрос отбрасывается.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-permission-relay.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=97d57f128f0da55f105ab1e3a7e10240" alt="Диаграмма последовательности: Claude Code отправляет уведомление permission_request на сервер канала, сервер форматирует и отправляет запрос в приложение чата, человек отвечает вердиктом, и сервер анализирует этот ответ в уведомление разрешения обратно в Claude Code" width="600" height="230" data-path="images/channel-permission-relay.svg" />

<h3 id="permission-request-fields">
  Поля запроса разрешения
</h3>

Исходящее уведомление от Claude Code — это `notifications/claude/channel/permission_request`. Как и [уведомление канала](#notification-format), транспорт — это стандартный MCP, но метод и схема — это расширения Claude Code. Объект `params` имеет четыре строковых поля, которые ваш сервер форматирует в исходящий запрос:

| Поле            | Описание                                                                                                                                                                                                                                                                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `request_id`    | Пять строчных букв, взятых из `a`-`z` без `l`, поэтому это никогда не читается как `1` или `I` при вводе на телефоне. Включите его в ваш исходящий запрос, чтобы его можно было повторить в ответе. Claude Code принимает только вердикт, который несёт ID, который он выдал. Диалог локального терминала не отображает этот ID, поэтому ваш исходящий обработчик — единственный способ узнать его. |
| `tool_name`     | Имя инструмента, который Claude хочет использовать, например `Bash` или `Write`.                                                                                                                                                                                                                                                                                                                    |
| `description`   | Понятное человеку резюме того, что делает этот конкретный вызов инструмента, тот же текст, который показывает диалог локального терминала. Для вызова Bash это описание Claude команды или сама команда, если описание не было дано.                                                                                                                                                                |
| `input_preview` | Аргументы инструмента как строка JSON, усечённая до 200 символов. Для Bash это команда; для Write это путь файла и префикс содержимого. Опустите его из вашего запроса, если у вас есть место только для однострочного сообщения. Ваш сервер решает, что показать.                                                                                                                                  |

Вердикт, который ваш сервер отправляет обратно, — это `notifications/claude/channel/permission` с двумя полями: `request_id`, повторяющий ID выше, и `behavior`, установленный на `'allow'` или `'deny'`. Allow позволяет вызову инструмента продолжиться; deny отклоняет его, то же самое, что ответить No в локальном диалоге. Ни один вердикт не влияет на будущие вызовы.

<h3 id="add-relay-to-a-chat-bridge">
  Добавление трансляции к мосту чата
</h3>

Добавление трансляции разрешений к двустороннему каналу требует трёх компонентов:

1. Запись `claude/channel/permission: {}` под `experimental` возможностями в конструкторе `Server`, чтобы Claude Code знал пересылать запросы
2. Обработчик уведомлений для `notifications/claude/channel/permission_request`, который форматирует запрос и отправляет его через API вашей платформы
3. Проверка в вашем входящем обработчике сообщений, которая распознаёт `yes <id>` или `no <id>` и отправляет уведомление вердикта `notifications/claude/channel/permission` вместо пересылки текста Claude

Объявляйте возможность только если ваш канал [аутентифицирует отправителя](#gate-inbound-messages), потому что любой, кто может ответить через ваш канал, может одобрять или отклонять использование инструментов в вашем сеансе.

Чтобы добавить их к двустороннему мосту чата, подобному собранному в [Предоставление инструмента ответа](#expose-a-reply-tool):

<Steps>
  <Step title="Объявление возможности разрешения">
    В конструкторе `Server` добавьте `claude/channel/permission: {}` рядом с `claude/channel` под `experimental`:

    ```ts theme={null}
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // согласитесь на трансляцию разрешений
      },
      tools: {},
    },
    ```
  </Step>

  <Step title="Обработка входящего запроса">
    Зарегистрируйте обработчик уведомлений между конструктором `Server` и `mcp.connect()`. Claude Code вызывает его с [четырьмя полями запроса](#permission-request-fields) при открытии диалога разрешения. Ваш обработчик форматирует запрос для вашей платформы и включает инструкции для ответа с ID:

    ```ts theme={null}
    import { z } from 'zod'

    // setNotificationHandler маршрутизирует по z.literal на поле method,
    // поэтому эта схема является как валидатором, так и ключом отправки
    const PermissionRequestSchema = z.object({
      method: z.literal('notifications/claude/channel/permission_request'),
      params: z.object({
        request_id: z.string(),     // пять строчных букв, включите дословно в ваш запрос
        tool_name: z.string(),      // например "Bash", "Write"
        description: z.string(),    // понятное человеку резюме этого вызова
        input_preview: z.string(),  // аргументы инструмента как JSON, усечённые до ~200 символов
      }),
    })

    mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
      // send() — это ваш исходящий: POST на вашу платформу чата, или для локального
      // тестирования трансляция SSE, показанная в полном примере ниже.
      send(
        `Claude wants to run ${params.tool_name}: ${params.description}\n\n` +
        // ID в инструкции — это то, что ваш входящий обработчик анализирует на шаге 3
        `Reply "yes ${params.request_id}" or "no ${params.request_id}"`,
      )
    })
    ```
  </Step>

  <Step title="Перехват вердикта в вашем входящем обработчике">
    Ваш входящий обработчик — это цикл или обратный вызов, который получает сообщения от вашей платформы: то же место, где вы [проверяете отправителя](#gate-inbound-messages) и отправляете `notifications/claude/channel` для пересылки чата Claude. Добавьте проверку перед вызовом пересылки чата, которая распознаёт формат вердикта и отправляет уведомление разрешения вместо этого.

    Регулярное выражение совпадает с форматом ID, который генерирует Claude Code: пять букв, никогда `l`. Флаг `/i` допускает автокоррекцию телефона, капитализирующую ответ; приведите захваченный ID в нижний регистр перед отправкой обратно.

    ```ts theme={null}
    // совпадает с "y abcde", "yes abcde", "n abcde", "no abcde"
    // [a-km-z] — это алфавит ID, который использует Claude Code (строчные, пропускает 'l')
    // /i допускает автокоррекцию телефона; приведите захват в нижний регистр перед отправкой
    const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i

    async function onInbound(message: PlatformMessage) {
      if (!allowed.has(message.from.id)) return  // сначала проверьте отправителя

      const m = PERMISSION_REPLY_RE.exec(message.text)
      if (m) {
        // m[1] — это слово вердикта, m[2] — это ID запроса
        // отправьте уведомление вердикта обратно в Claude Code вместо чата
        await mcp.notification({
          method: 'notifications/claude/channel/permission',
          params: {
            request_id: m[2].toLowerCase(),  // нормализуйте в случае автокоррекции капс
            behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
          },
        })
        return  // обработано как вердикт, не пересылайте также как чат
      }

      // не совпадает с форматом вердикта: перейдите к нормальному пути чата
      await mcp.notification({
        method: 'notifications/claude/channel',
        params: { content: message.text, meta: { chat_id: String(message.chat.id) } },
      })
    }
    ```
  </Step>
</Steps>

Claude Code также держит диалог локального терминала открытым, поэтому вы можете ответить в любом месте, и первый поступивший ответ применяется. Удалённый ответ, который не совпадает точно с ожидаемым форматом, не удаётся одним из двух способов, и в обоих случаях диалог остаётся открытым:

* **Другой формат**: регулярное выражение вашего входящего обработчика не совпадает, поэтому текст, такой как `approve it` или `yes` без ID, переходит как обычное сообщение Claude.
* **Правильный формат, неправильный ID**: ваш сервер отправляет вердикт, но Claude Code не находит открытый запрос с этим ID и молча его отбрасывает.

<h3 id="full-example">
  Полный пример
</h3>

Собранный `webhook.ts` ниже объединяет все три расширения с этой страницы: инструмент ответа, проверка отправителя и трансляция разрешений. Если вы начинаете отсюда, вам также потребуется [настройка проекта и запись `.mcp.json`](#example-build-a-webhook-receiver) из начального пошагового руководства.

Чтобы сделать обе стороны тестируемыми из curl, слушатель HTTP обслуживает два пути:

* **`GET /events`**: держит открытым поток SSE и отправляет каждое исходящее сообщение как строку `data:`, поэтому `curl -N` может смотреть ответы Claude и запросы разрешений в реальном времени.
* **`POST /`**: входящая сторона, тот же обработчик, что и раньше, теперь с проверкой формата вердикта, вставленной перед ветвью пересылки чата.

```ts title="Full webhook.ts with permission relay' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'
import { z } from 'zod'

// --- Исходящий: запись для любых слушателей curl -N на /events ---
// Реальный мост отправлял бы POST на вашу платформу чата вместо этого.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

// Список разрешений отправителя. Для локального пошагового руководства мы доверяем одному значению заголовка X-Sender
// "dev"; реальный мост проверял бы ID пользователя платформы.
const allowed = new Set(['dev'])

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // согласитесь на трансляцию разрешений
      },
      tools: {},
    },
    instructions:
      'Messages arrive as <channel source="webhook" chat_id="...">. ' +
      'Reply with the reply tool, passing the chat_id from the tag.',
  },
)

// --- инструмент reply: Claude вызывает это для отправки сообщения обратно ---
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

// --- трансляция разрешений: Claude Code (не Claude) вызывает это при открытии диалога
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

// --- HTTP на :8788: GET /events передаёт исходящий, POST маршрутизирует входящий ---
const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i
let nextId = 1

Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // не закрывайте неактивные потоки SSE
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: поток SSE, чтобы curl -N мог смотреть ответы и запросы в реальном времени
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // чтобы curl показал что-то сразу
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // всё остальное входящее: сначала проверьте отправителя
    const body = await req.text()
    const sender = req.headers.get('X-Sender') ?? ''
    if (!allowed.has(sender)) return new Response('forbidden', { status: 403 })

    // проверьте формат вердикта перед обработкой как чат
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

    // обычный чат: пересылка Claude как событие канала
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: { content: body, meta: { chat_id, path: url.pathname } },
    })
    return new Response('ok')
  },
})
```

Тестируйте путь вердикта в трёх терминалах. Первый — это ваш сеанс Claude Code, запущенный с [флагом разработки](#test-during-the-research-preview), чтобы он запустил `webhook.ts`:

```bash theme={null}
claude --dangerously-load-development-channels server:webhook
```

Во втором потоке исходящая сторона, чтобы вы могли видеть ответы Claude и любые запросы разрешений по мере их срабатывания:

```bash theme={null}
curl -N localhost:8788/events
```

В третьем отправьте сообщение, которое заставит Claude попытаться запустить команду:

```bash theme={null}
curl -d "list the files in this directory" -H "X-Sender: dev" localhost:8788
```

Перечисление файлов доступно только для чтения, поэтому Claude запускает его без одобрения. Диалог разрешения открывается, когда Claude вызывает инструмент `reply` для отправки своего ответа обратно. Локальный диалог открывается в вашем терминале Claude Code, и через момент запрос для `mcp__webhook__reply` появляется в потоке `/events`, включая пятибуквенный ID. Одобрите его с удалённой стороны:

```bash theme={null}
curl -d "yes <id>" -H "X-Sender: dev" localhost:8788
```

Локальный диалог закрывается, инструмент `reply` запускается, и ответ Claude попадает в поток.

Три специфичные для канала части в этом файле:

* **Возможности** в конструкторе `Server`: `claude/channel` регистрирует слушатель уведомлений, `claude/channel/permission` согласуется на трансляцию разрешений, `tools` позволяет Claude обнаружить инструмент ответа.
* **Исходящие пути**: обработчик инструмента `reply` — это то, что Claude вызывает для разговорных ответов; обработчик уведомлений `PermissionRequestSchema` — это то, что Claude Code вызывает при открытии диалога разрешения. Оба вызывают `send()` для трансляции через `/events`, но они запускаются разными частями системы.
* **Обработчик HTTP**: `GET /events` держит открытым поток SSE, чтобы curl мог смотреть исходящий в реальном времени; `POST` входящий, проверенный на заголовок `X-Sender`. Тело `yes <id>` или `no <id>` переходит в Claude Code как уведомление вердикта и никогда не достигает Claude; всё остальное пересылается Claude как событие канала.

<h2 id="package-as-a-plugin">
  Упаковка как плагин
</h2>

Чтобы сделать ваш канал устанавливаемым и общим, оберните его в [плагин](/docs/ru/plugins) и опубликуйте на [маркетплейс](/docs/ru/plugin-marketplaces). Пользователи устанавливают его с `/plugin install`, затем включают его за сеанс с `--channels plugin:<name>@<marketplace>`.

Канал, опубликованный на вашем собственном маркетплейсе, по-прежнему требует `--dangerously-load-development-channels` для запуска, так как он не находится в [одобренном списке разрешений](/docs/ru/channels#supported-channels). Список разрешений по умолчанию — это плагины каналов в `claude-plugins-official`, которые Anthropic курирует по своему усмотрению. [Встроенные формы отправки](/docs/ru/plugins#submit-your-plugin-to-the-community-marketplace) добавляют плагины на маркетплейс сообщества, который не находится в списке разрешений каналов.

Если вы работаете с контактом партнёра Anthropic, свяжитесь с ними, чтобы согласовать официальный список маркетплейса. На планах Team и Enterprise администратор может вместо этого включить ваш плагин в список [`allowedChannelPlugins`](/docs/ru/channels#restrict-which-channel-plugins-can-run) организации, который заменяет список разрешений Anthropic по умолчанию.

<h2 id="see-also">
  См. также
</h2>

* [Каналы](/docs/ru/channels) для установки и использования Telegram, Discord, iMessage или демонстрации fakechat, а также для включения каналов для организации Team или Enterprise
* [Рабочие реализации каналов](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) для полного кода сервера с потоками спаривания, инструментами ответа и вложениями файлов
* [MCP](/docs/ru/mcp) для базового протокола, который реализуют серверы каналов
* [Плагины](/docs/ru/plugins) для упаковки вашего канала, чтобы пользователи могли установить его с `/plugin install`
