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

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-architecture.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=9a037b7da80184ae49015c0256b21a1f" className="dark:hidden" alt="Диаграмма архитектуры, показывающая внешние системы, подключающиеся к вашему локальному серверу канала, который взаимодействует с Claude Code через stdio" width="600" height="220" data-path="images/channel-architecture.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/channel-architecture-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=ae1e494440806a6a5d74a1279e22e162" className="hidden dark:block" alt="Диаграмма архитектуры, показывающая внешние системы, подключающиеся к вашему локальному серверу канала, который взаимодействует с Claude Code через stdio" width="600" height="220" data-path="images/channel-architecture-dark.svg" />

<h2 id="what-you-need">
  Что вам нужно
</h2>

Единственное жёсткое требование — это пакет [`@modelcontextprotocol/sdk`](https://www.npmjs.com/package/@modelcontextprotocol/sdk) и совместимая с Node.js среда выполнения. [Bun](https://bun.sh), [Node](https://nodejs.org) и [Deno](https://deno.com) работают. Предварительно созданные плагины в исследовательском превью используют Bun, но ваш канал не обязательно должен.

Ваш сервер должен:

1. Объявить возможность `claude/channel`, чтобы Claude Code зарегистрировал слушатель уведомлений
2. Отправлять события `notifications/claude/channel` при возникновении чего-либо
3. Подключаться через [транспорт stdio](https://modelcontextprotocol.io/docs/concepts/transports#standard-io)

Разделы [Параметры сервера](#server-options) и [Формат уведомления](#notification-format) подробно рассматривают каждый из них. Полное пошаговое руководство см. в [Пример: создание приёмника вебхуков](#example-build-a-webhook-receiver).

Во время исследовательского превью пользовательские каналы не находятся в [одобренном списке разрешений](/docs/ru/channels#supported-channels). Используйте `--dangerously-load-development-channels` для локального тестирования. Подробности см. в [Тестирование во время исследовательского превью](#test-during-the-research-preview).

<h2 id="example-build-a-webhook-receiver">
  Пример: создание получателя webhook
</h2>

В этом пошаговом руководстве создаётся однофайловый сервер, который прослушивает HTTP-запросы и перенаправляет их в вашу сессию Claude Code. В конце концов, всё, что может отправить HTTP POST, например CI-конвейер, оповещение мониторинга или команда `curl`, сможет отправлять события в Claude.

В этом примере используется [Bun](https://bun.sh) в качестве среды выполнения благодаря встроенному HTTP-серверу и поддержке TypeScript. Вы можете использовать [Node](https://nodejs.org) или [Deno](https://deno.com); единственное требование — это [MCP SDK](https://www.npmjs.com/package/@modelcontextprotocol/sdk).

<Steps>
  <Step title="Создание проекта">
    Примеры [реле разрешений](#relay-permission-prompts) далее на этой странице импортируют `zod` напрямую, поэтому он устанавливается вместе с MCP SDK. Создайте новый каталог и установите оба:

    ```bash theme={null}
    mkdir webhook-channel && cd webhook-channel
    bun add @modelcontextprotocol/sdk zod
    ```
  </Step>

  <Step title="Написание сервера канала">
    Создайте файл с именем `webhook.ts`. Это ваш полный сервер канала: он подключается к Claude Code через stdio и прослушивает HTTP POST на порту 8788. Когда приходит запрос, он отправляет тело в Claude как событие канала.

    ```ts title="webhook.ts" theme={null}
    #!/usr/bin/env bun
    import { Server } from '@modelcontextprotocol/sdk/server/index.js'
    import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'

    // Create the MCP server and declare it as a channel
    const mcp = new Server(
      { name: 'webhook', version: '0.0.1' },
      {
        // this key is what makes it a channel — Claude Code registers a listener for it
        capabilities: { experimental: { 'claude/channel': {} } },
        // Claude Code delivers this to Claude as context when the server connects, so it knows how to handle these events
        instructions: 'Events from the webhook channel arrive as <channel source="webhook" ...>. They are one-way: read them and act, no reply expected.',
      },
    )

    // Connect to Claude Code over stdio (Claude Code spawns this process)
    await mcp.connect(new StdioServerTransport())

    // Start an HTTP server that forwards every POST to Claude
    Bun.serve({
      port: 8788,  // any open port works
      // localhost-only: nothing outside this machine can POST
      hostname: '127.0.0.1',
      async fetch(req) {
        const body = await req.text()
        await mcp.notification({
          method: 'notifications/claude/channel',
          params: {
            content: body,  // becomes the body of the <channel> tag
            // each key becomes a tag attribute, e.g. <channel path="/" method="POST">
            meta: { path: new URL(req.url).pathname, method: req.method },
          },
        })
        return new Response('ok')
      },
    })
    ```

    Файл конфигурирует сервер, подключается через stdio и запускает HTTP-слушатель в этом порядке:

    * **Конфигурация сервера**: создаёт MCP-сервер с `claude/channel` в его возможностях, что говорит Claude Code, что это канал. Claude Code доставляет строку [`instructions`](#server-options) в Claude как контекст при подключении сервера: расскажите Claude, какие события ожидать, нужно ли отвечать и как маршрутизировать ответы, если это необходимо.
    * **Подключение через stdio**: подключается к Claude Code через stdin/stdout. Это стандартно для любого [MCP-сервера](https://modelcontextprotocol.io/docs/concepts/transports#standard-io).
    * **HTTP-слушатель**: запускает локальный веб-сервер на порту 8788. Каждое тело POST перенаправляется в Claude как событие канала через `mcp.notification()`. `content` становится телом события, и каждая запись `meta` становится атрибутом на теге `<channel>`. Слушатель нуждается в доступе к экземпляру `mcp`, поэтому он работает в том же процессе. Вы можете разделить его на отдельные модули для более крупного проекта.
  </Step>

  <Step title="Регистрация вашего сервера в Claude Code">
    Добавьте сервер в вашу конфигурацию MCP, чтобы Claude Code знал, как его запустить. Для файла `.mcp.json` на уровне проекта в том же каталоге используйте относительный путь. Для конфигурации на уровне пользователя в `~/.claude.json` используйте полный абсолютный путь, чтобы сервер можно было найти из любого проекта:

    ```json title=".mcp.json" theme={null}
    {
      "mcpServers": {
        "webhook": { "command": "bun", "args": ["./webhook.ts"] }
      }
    }
    ```

    Claude Code читает вашу конфигурацию MCP при запуске и порождает каждый сервер как подпроцесс.
  </Step>

  <Step title="Тестирование">
    Во время исследовательского предпросмотра пользовательские каналы не находятся в списке разрешений, поэтому запустите Claude Code с флагом разработки:

    ```bash theme={null}
    claude --dangerously-load-development-channels server:webhook
    ```

    Claude Code сначала показывает диалоговое окно предупреждения на весь экран, в котором перечислены загружаемые вами каналы разработки. Выберите **I am using this for local development**, чтобы продолжить, или **Exit**, чтобы выйти.

    При первом запуске сессии в этом проекте Claude Code также запрашивает согласие перед использованием нового сервера из `.mcp.json`. Диалог сообщает «New MCP server found in this project: webhook». Выберите **Use this MCP server**, чтобы продолжить.

    После того как вы согласитесь, Claude Code порождает ваш `webhook.ts` как подпроцесс, и HTTP-слушатель автоматически запускается на настроенном вами порту, 8788 в этом примере. Вам не нужно запускать сервер самостоятельно.

    Тусклое уведомление под баннером запуска подтверждает, что канал зарегистрирован: `Channels (experimental) messages from server:webhook inject directly in this session · restart without --dangerously-load-development-channels to stop`.

    Если вы видите «blocked by org policy», администратор вашей организации должен сначала [включить каналы](/docs/ru/channels#enterprise-controls).

    В отдельном терминале имитируйте webhook, отправив HTTP POST с сообщением на ваш сервер. Этот пример отправляет оповещение об ошибке CI на порт 8788 (или любой другой порт, который вы настроили):

    ```bash theme={null}
    curl -X POST localhost:8788 -d "build failed on main: https://ci.example.com/run/1234"
    ```

    Полезная нагрузка поступает в контекст Claude как тег `<channel>`:

    ```text theme={null}
    <channel source="webhook" path="/" method="POST">build failed on main: https://ci.example.com/run/1234</channel>
    ```

    Ваш терминал отображает событие как однострочное резюме, `← webhook: build failed on main: https://ci.example.com/run/1234`, а не как необработанный тег. Затем вы увидите, как Claude начинает отвечать: читая файлы, выполняя команды или что-то ещё, что требует сообщение. Это односторонний канал, поэтому Claude действует в вашей сессии, но ничего не отправляет обратно через webhook. Чтобы добавить ответы, см. [Expose a reply tool](#expose-a-reply-tool).

    Если событие не поступает, диагностика зависит от того, что вернул `curl`:

    * **`curl` успешен, но ничего не достигает Claude**: запустите `/mcp` в вашей сессии, чтобы проверить статус сервера. Статус `failed` обычно означает ошибку зависимости или импорта в файле вашего сервера. Чтобы увидеть трассировку stderr, перезагрузитесь с помощью `claude --debug --dangerously-load-development-channels server:webhook` и проверьте журнал отладки в `~/.claude/debug/<session-id>.txt`.
    * **`curl` не удаётся с «connection refused»**: порт либо ещё не привязан, либо устаревший процесс из более ранней попытки его удерживает. `lsof -i :<port>` показывает, что прослушивается; `kill` устаревший процесс перед перезагрузкой вашей сессии.
  </Step>
</Steps>

Сервер [fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) расширяет этот паттерн с веб-интерфейсом, вложениями файлов и инструментом ответа для двусторонней переписки.

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

Обход выполняется для каждой записи. Объединение этого флага с `--channels` не распространяет обход на записи `--channels`. Во время исследовательского превью ваш канал не находится в одобренном списке разрешений, поэтому он остаётся на флаге разработки во время разработки и тестирования.

<Note>
  Этот флаг пропускает только список разрешений. Политика организации `channelsEnabled` по-прежнему применяется. Не используйте его для запуска каналов из ненадёжных источников.
</Note>

<h2 id="server-options">
  Параметры сервера
</h2>

Канал устанавливает эти параметры в конструкторе [`Server`](https://modelcontextprotocol.io/docs/learn/server-concepts). Поля `instructions` и `capabilities.tools` являются [стандартным MCP](https://modelcontextprotocol.io/docs/learn/server-concepts); `capabilities.experimental['claude/channel']` и `capabilities.experimental['claude/channel/permission']` — это дополнения, специфичные для канала:

| Поле                                                     | Тип                  | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------------------------------------------- | :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capabilities.experimental['claude/channel']`            | `object`             | Обязательно. Всегда `{}`. Наличие регистрирует слушатель уведомлений.                                                                                                                                                                                                                                                                                                                                                                                            |
| `capabilities.experimental['claude/channel/permission']` | `object` или `false` | Опционально. Установите значение `{}`, чтобы объявить, что этот канал может получать запросы трансляции разрешений. При объявлении Claude Code пересылает запросы одобрения инструментов на ваш канал, чтобы вы могли одобрять или отклонять их удалённо. Чтобы отказаться, опустите ключ или установите значение `false`. До версии v2.1.234 Claude Code рассматривал `false` как объявленный. См. [Трансляция запросов разрешений](#relay-permission-prompts). |
| `capabilities.tools`                                     | `object`             | Только двусторонний. Всегда `{}`. Стандартная возможность инструмента MCP. См. [Предоставление инструмента ответа](#expose-a-reply-tool).                                                                                                                                                                                                                                                                                                                        |
| `instructions`                                           | `string`             | Рекомендуется. Claude Code доставляет его Claude в качестве контекста при подключении сервера. Скажите Claude, какие события ожидать, что означают атрибуты тега `<channel>`, нужно ли отвечать и если да, какой инструмент использовать и какой атрибут передать обратно (например `chat_id`).                                                                                                                                                                  |

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
    // Claude Code доставляет это Claude в качестве контекста при подключении сервера, чтобы он знал, как обрабатывать ваши события
    instructions: 'Messages arrive as <channel source="your-channel" ...>. Reply with the reply tool.',
  },
)
```

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

Claude Code не подтверждает уведомления. `await` на `mcp.notification()` разрешается, когда сообщение записывается в транспорт, а не когда Claude его обработал. Если сеанс не загрузил ваш сервер как канал, или политика организации его блокирует, события молча отбрасываются без ошибки, возвращаемой вашему серверу.

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

Каналы [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) и [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) проверяют список разрешений отправителя так же. Они загружают список путём [спаривания](/docs/ru/channels#security). Полный поток спаривания см. в любой реализации. Канал [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) использует другой подход: он обнаруживает собственные адреса пользователя из базы данных Messages при запуске и пропускает их автоматически, с другими отправителями, добавляемыми по дескриптору.

<h2 id="relay-permission-prompts">
  Трансляция запросов разрешений
</h2>

Когда Claude вызывает инструмент, требующий одобрения, открывается диалог локального терминала и сеанс ждёт. Двусторонний канал может согласиться получить тот же запрос параллельно и передать его вам на другое устройство. Оба остаются активными: вы можете ответить в терминале или на телефоне, и Claude Code применяет любой ответ, который поступит первым, и закрывает другой.

Трансляция охватывает одобрения использования инструментов, такие как `Bash`, `Write` и `Edit`. Диалоги доверия проекта и согласия MCP-сервера не передаются; они появляются только в локальном терминале.

Claude Code v2.1.234 и позже отправляет запросы разрешений только на серверы, которые он зарегистрировал как каналы для сеанса, поэтому трансляция находится за теми же [элементами управления согласием сеанса и организации](/docs/ru/channels#security), что и доставка сообщений. Трансляция также требует, чтобы вы согласили сервер с помощью `--channels` или флага разработки, и требует, чтобы сервер объявил возможность разрешения.

<h3 id="how-relay-works">
  Как работает трансляция
</h3>

Когда открывается запрос разрешения, цикл трансляции имеет четыре шага:

1. Claude Code генерирует короткий ID запроса и уведомляет ваш сервер
2. Ваш сервер пересылает запрос и ID в ваше приложение чата
3. Удалённый пользователь отвечает да или нет и этот ID
4. Ваш входящий обработчик анализирует ответ в вердикт, и Claude Code применяет его только если ID совпадает с открытым запросом

Диалог локального терминала остаётся открытым на протяжении всего этого. Если кто-то в терминале ответит перед поступлением удалённого вердикта, этот ответ применяется вместо этого и ожидающий удалённый запрос отбрасывается.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-permission-relay.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=97d57f128f0da55f105ab1e3a7e10240" className="dark:hidden" alt="Диаграмма последовательности: Claude Code отправляет уведомление permission_request на сервер канала, сервер форматирует и отправляет запрос в приложение чата, человек отвечает вердиктом, и сервер анализирует этот ответ в уведомление разрешения обратно в Claude Code" width="600" height="230" data-path="images/channel-permission-relay.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/channel-permission-relay-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=368c8d9119a9a9cff5d826d806724842" className="hidden dark:block" alt="Диаграмма последовательности: Claude Code отправляет уведомление permission_request на сервер канала, сервер форматирует и отправляет запрос в приложение чата, человек отвечает вердиктом, и сервер анализирует этот ответ в уведомление разрешения обратно в Claude Code" width="600" height="230" data-path="images/channel-permission-relay-dark.svg" />

<h3 id="permission-request-fields">
  Поля запроса разрешения
</h3>

Исходящее уведомление от Claude Code — это `notifications/claude/channel/permission_request`. Как и [уведомление канала](#notification-format), транспорт — это стандартный MCP, но метод и схема — это расширения Claude Code. Объект `params` имеет четыре строковых поля, которые ваш сервер форматирует в исходящий запрос:

| Поле            | Описание                                                                                                                                                                                                                                                                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `request_id`    | Пять строчных букв, взятых из `a`-`z` без `l`, поэтому это никогда не читается как `1` или `I` при вводе на телефоне. Включите его в ваш исходящий запрос, чтобы его можно было повторить в ответе. Claude Code принимает только вердикт, который несёт ID, который он выдал. Диалог локального терминала не отображает этот ID, поэтому ваш исходящий обработчик — единственный способ узнать его. |
| `tool_name`     | Имя инструмента, который Claude хочет использовать, например `Bash` или `Write`.                                                                                                                                                                                                                                                                                                                    |
| `description`   | Понятное человеку резюме того, что делает этот конкретный вызов инструмента, никогда не сама команда. Для вызова Bash это описание Claude команды; когда модель не даёт описание, поле — это константа `Run shell command` и не содержит деталей команды. Отображайте `input_preview`, когда у вас есть место.                                                                                      |
| `input_preview` | Аргументы инструмента как текст в форме JSON, с ключами по полям верхнего уровня. Для Bash это команда; для Write это путь файла и содержимое. Опустите его из вашего запроса, если у вас есть место только для однострочного сообщения. Ваш сервер решает, что показать.                                                                                                                           |

Клиенты на Claude Code v2.1.211 или позже санитизируют `description` и `input_preview` перед их трансляцией. Ожидайте три изменения в тексте, который вы получите:

* Claude Code нейтрализует символы переопределения направления, невидимые символы и похожие на кавычки и угловые скобки символы.
* Claude Code складывает каждый набор пробелов в один пробел.
* Claude Code передаёт текст целиком до 3500 кодовых точек. Для более длинного значения вы получаете его начало и конец вокруг подсчитанного маркера `⋯ N code points elided ⋯`. Конец длинной команды всё ещё достигает одобряющего.

Для `input_preview` Claude Code применяет лимит 3500 к каждому полю верхнего уровня аргументов отдельно и сохраняет собственные структурные кавычки JSON. Клиенты до v2.1.211 передают `description` как есть и обрезают `input_preview` до 200 единиц UTF-16 с конечным многоточием.

Клиенты на Claude Code v2.1.234 или позже передают маркер `(value unserializable)` вместо значения поля `input_preview`, которое они не могут безопасно сериализовать, такого как циклическая структура или чрезвычайно большой массив. Вы всё ещё получаете ключ поля, и другие поля предпросмотра не изменяются.

Клиенты на Claude Code v2.1.234 или позже также маскируют учётные данные в `description` и `input_preview`. Вы получаете `[REDACTED]` вместо узнаваемого токена учётных данных поставщика, такого как ключ API или личный токен доступа. Ожидайте три эффекта маскирования при отображении полей:

* Claude Code маскирует имена ключей внутри `input_preview` а также их значения. Имя ключа, которое вы отображаете, может не совпадать с именем ключа во входных данных.
* Claude Code никогда не маскирует диапазон, который содержит синтаксис оболочки, символы пути или символы URL. Маска не может скрыть команду, путь файла или пункт назначения, который одобряется.
* Claude Code не маскирует секрет, который не имеет узнаваемого префикса, или секрет, который охватывает пробелы, такой как блок приватного ключа. Оба достигают вашего сервера без маски.

Маскирование не меняет, кто получает поля. Всё, что остаётся без маски, идёт только на серверы, которые вы согласили с помощью `--channels` или флага разработки. Рассматривайте оба поля как ненадёжные, если вы не контролируете парк клиентов.

Вердикт, который ваш сервер отправляет обратно, — это `notifications/claude/channel/permission` с двумя полями: `request_id`, повторяющий ID выше, и `behavior`, установленный на `'allow'` или `'deny'`. Allow позволяет вызову инструмента продолжиться; deny отклоняет его. Ни один вердикт не влияет на будущие вызовы.

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
        description: z.string(),    // резюме того, что делает этот вызов. Рассматривайте как ненадёжное.
        input_preview: z.string(),  // аргументы инструмента как текст в форме JSON. Рассматривайте как ненадёжное.
      }),
    })

    mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
      // send() — это ваш исходящий: POST на вашу платформу чата, или для локального
      // тестирования трансляция SSE, показанная в полном примере ниже.
      send(
        `Claude wants to run ${params.tool_name}: ${params.description}\n` +
        // input_preview содержит фактические аргументы; отображайте его, когда у вас есть место:
        // для Bash описание может быть просто "Run shell command" без деталей команды
        `${params.input_preview}\n\n` +
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

Удалённый ответ, который не совпадает точно с ожидаемым форматом, не удаётся одним из двух способов, и в обоих случаях диалог локального терминала остаётся открытым:

* **Другой формат**: регулярное выражение вашего входящего обработчика не совпадает, поэтому текст, такой как `approve it` или `yes` без ID, переходит как обычное сообщение Claude.
* **Правильный формат, неправильный ID**: ваш сервер отправляет вердикт, но Claude Code не находит открытый запрос с этим ID и молча его отбрасывает.

<h3 id="full-example">
  Полный пример
</h3>

Собранный `webhook.ts` ниже объединяет все три расширения с этой страницы: инструмент ответа, проверка отправителя и трансляция разрешений. Если вы начинаете отсюда, вам также потребуется [настройка проекта и запись `.mcp.json`](#example-build-a-webhook-receiver) из начального пошагового руководства.

Чтобы сделать обе стороны тестируемыми из curl, слушатель HTTP обслуживает два пути:

* **`GET /events`**: держит открытым поток SSE и отправляет каждое исходящее сообщение как строку `data:`, поэтому `curl -N` может смотреть ответы Claude и запросы разрешений в реальном времени.
* **`POST /`**: входящая сторона, тот же обработчик, что и раньше, теперь с проверкой формата вердикта, вставленной перед ветвью пересылки чата.

```ts title="Full webhook.ts with permission relay" expandable theme={null}
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
    `Claude wants to run ${params.tool_name}: ${params.description}\n` +
    `${params.input_preview}\n\n` +
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

Это пошаговое руководство тестирует сам диалог разрешения, поэтому после открытия сеанса нажимайте `Shift+Tab` до тех пор, пока строка состояния не покажет `⏸ manual mode on`. В автоматическом режиме классификатор решал бы вызов `reply` вместо вас, и диалог не открывался бы для удалённой стороны, чтобы ответить.

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

Чтобы сделать ваш канал устанавливаемым и общим, оберните его в [плагин](/docs/ru/plugins/overview) и опубликуйте на [маркетплейс](/docs/ru/plugins/overview). Пользователи устанавливают его с `/plugin install`, затем включают его за сеанс с `--channels plugin:<name>@<marketplace>`.

Канал, опубликованный на вашем собственном маркетплейсе, по-прежнему требует `--dangerously-load-development-channels` для запуска, так как он не находится в [одобренном списке разрешений](/docs/ru/channels#supported-channels). Список разрешений по умолчанию — это плагины каналов в `claude-plugins-official`. [Встроенные формы отправки](/docs/ru/plugins/publish#submit-to-the-community-marketplace) добавляют плагины на маркетплейс сообщества, который не находится в списке разрешений каналов.

Если вы работаете с контактом партнёра Anthropic, свяжитесь с ними, чтобы согласовать официальный список маркетплейса. На планах Team и Enterprise администратор может вместо этого включить ваш плагин в список [`allowedChannelPlugins`](/docs/ru/channels#restrict-which-channel-plugins-can-run) организации, который заменяет список разрешений Anthropic по умолчанию.

<h2 id="see-also">
  См. также
</h2>

* [Каналы](/docs/ru/channels) для установки и использования Telegram, Discord, iMessage или демонстрации fakechat, а также для включения каналов для организации Team или Enterprise
* [Рабочие реализации каналов](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) для полного кода сервера с потоками спаривания, инструментами ответа и вложениями файлов
* [MCP](/docs/ru/mcp) для базового протокола, который реализуют серверы каналов
* [Плагины](/docs/ru/plugins/overview) для упаковки вашего канала, чтобы пользователи могли установить его с `/plugin install`
