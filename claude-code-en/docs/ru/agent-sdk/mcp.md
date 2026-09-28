> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Подключение к внешним инструментам с помощью MCP

> Настройте MCP серверы для расширения вашего агента внешними инструментами. Охватывает типы транспорта, поиск инструментов для больших наборов инструментов, аутентификацию и обработку ошибок.

[Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) — это открытый стандарт для подключения AI агентов к внешним инструментам и источникам данных. С помощью MCP ваш агент может запрашивать базы данных, интегрироваться с API, такими как Slack и GitHub, и подключаться к другим сервисам без написания пользовательских реализаций инструментов.

MCP серверы могут работать как локальные процессы, подключаться через HTTP или выполняться непосредственно в вашем приложении SDK.

<Note>
  На этой странице рассматривается конфигурация MCP для Agent SDK. Чтобы добавить MCP серверы в Claude Code CLI так, чтобы они загружались в каждом проекте, см. [Области установки MCP](/docs/ru/mcp#mcp-installation-scopes).
</Note>

<h2 id="quickstart">
  Быстрый старт
</h2>

Этот пример подключается к MCP серверу [документации Claude Code](https://code.claude.com/docs) с использованием [HTTP транспорта](#http%2Fsse-servers) и использует [`allowedTools`](#allow-mcp-tools) с подстановочным знаком для разрешения всех инструментов с сервера.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the docs MCP server to explain what hooks are in Claude Code",
    options: {
      mcpServers: {
        "claude-code-docs": {
          type: "http",
          url: "https://code.claude.com/docs/mcp"
        }
      },
      allowedTools: ["mcp__claude-code-docs__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "claude-code-docs": {
                  "type": "http",
                  "url": "https://code.claude.com/docs/mcp",
              }
          },
          allowed_tools=["mcp__claude-code-docs__*"],
      )

      async for message in query(
          prompt="Use the docs MCP server to explain what hooks are in Claude Code",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Агент подключается к серверу документации, ищет информацию о hooks и возвращает результаты.

<h2 id="add-an-mcp-server">
  Добавить MCP сервер
</h2>

Вы можете настроить MCP серверы в коде при вызове `query()`, или в файле `.mcp.json`, загруженном через [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  В коде
</h3>

Передайте MCP серверы непосредственно в опции `mcpServers`. Этот пример запускает локальный файловый MCP сервер для `/Users/me/projects`. Замените этот путь на директорию на вашей машине:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List files in my project",
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "filesystem": {
                  "command": "npx",
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-filesystem",
                      "/Users/me/projects",
                  ],
              }
          },
          allowed_tools=["mcp__filesystem__*"],
      )

      async for message in query(prompt="List files in my project", options=options):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="from-a-config-file">
  Из файла конфигурации
</h3>

Создайте файл `.mcp.json` в корне вашего проекта. Файл загружается, когда включен источник настроек `project`, что происходит по умолчанию для опций `query()`. Если вы явно установите `settingSources`, включите `"project"` для загрузки этого файла. Замените `/Users/me/projects` на директорию на вашей машине:

```json theme={null}
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
    }
  }
}
```

<h2 id="connection-timing">
  Время подключения
</h2>

Claude Code регистрирует серверы, которые вы передаёте в `options.mcpServers` при запуске и отправляет [сообщение инициализации](#error-handling) после разрешения первого ожидания, если оно есть. Подключение каждого сервера из `options.mcpServers` и задерживает ли он первый ход, зависит от его типа:

| Тип сервера                                                                                              | Задерживает первый ход?                               | Тайм-аут ожидания первого хода                                                                           |
| :------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| Сервер stdio или HTTP/SSE сервер без кэшированного списка инструментов                                   | Да, до подключения                                    | [`MCP_TIMEOUT`](/docs/ru/env-vars), по умолчанию 30 секунд; подключение завершается ошибкой в этот момент     |
| Удалённый сервер с кэшированным списком инструментов, сохранённый Claude Code из предыдущего подключения | Нет; кэшированные инструменты доступны с первого хода | Нет; подключается при первом вызове инструмента, и это отложенное подключение имеет собственный тайм-аут |
| In-process [SDK сервер](#sdk-mcp-servers)                                                                | Да, до подключения и получения списка инструментов    | Нет; запросы подключения и получения списка инструментов имеют собственные тайм-ауты                     |

Серверы, загруженные из [файлов конфигурации](#from-a-config-file), таких как `.mcp.json`, или из плагинов обычно показывают `pending` в сообщении инициализации. Когда `options.mcpServers` содержит сервер stdio, HTTP или SSE, первый ход ждёт эти ожидающие серверы тоже, до `MCP_TIMEOUT`. Когда `options.mcpServers` пуст или содержит только SDK серверы, первый ход ждёт до 2 секунд вместо этого:

* **С [поиском инструментов](/docs/ru/agent-sdk/tool-search), по умолчанию**: ожидание охватывает всё ещё ожидающие серверы, настроенные с [`alwaysLoad: true`](/docs/ru/mcp#exempt-a-server-from-deferral), и не остальные. Остальные продолжают подключаться в фоновом режиме. [Доступность инструментов](/docs/ru/mcp#tool-availability) описывает, как Claude достигает их инструментов после подключения.
* **Без поиска инструментов**: ожидание охватывает каждый ожидающий сервер. [Настройка поиска инструментов](/docs/ru/agent-sdk/tool-search#configure-tool-search) охватывает то, что отключает поиск инструментов. Если вы исключите инструмент `ToolSearch` из сеанса, например через `disallowedTools`, сеанс также работает без поиска инструментов.

Если вы установите `permissionPromptToolName`, первый ход также ждёт сервер этого инструмента в каждом случае, до `MCP_TIMEOUT`.

Чтобы установить ожидание первого хода самостоятельно, добавьте `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` в [опцию `env`](/docs/ru/agent-sdk/configuration#set-environment-variables), например `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"`. Первый ход затем ждёт до этого количества миллисекунд для каждого ожидающего сервера, независимо от того, доступен ли поиск инструментов. Этот срок также заменяет ожидание первого хода `MCP_TIMEOUT` для серверов stdio, HTTP и SSE в `options.mcpServers`. `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` требует Claude Code версии 2.1.274 или позже.

Серверы, которые всё ещё ожидают, когда ожидание заканчивается, продолжают подключаться в фоновом режиме. Установите переменную в `0`, чтобы пропустить ожидание. Сервер `permissionPromptToolName` сохраняет собственное ожидание `MCP_TIMEOUT` независимо от значения.

Чтобы заблокировать сам запуск на отдельной, более ранней фазе, чем ожидание первого хода, перед отправкой сообщения инициализации:

* Установите [`MCP_CONNECTION_NONBLOCKING`](/docs/ru/env-vars) в значение `0`, чтобы заблокировать всю партию подключений. Claude Code ограничивает это ожидание 5 секундами по умолчанию. Отрегулируйте ограничение с помощью переменной окружения [`MCP_CONNECT_TIMEOUT_MS`](/docs/ru/env-vars) в миллисекундах. Серверы, которые всё ещё ожидают в этот момент, продолжают подключаться в фоновом режиме.
* Установите `alwaysLoad: true` в конфигурации сервера, чтобы его инструменты были доступны с полными схемами на первом ходе, [исключены из отложенного поиска инструментов](/docs/ru/mcp#exempt-a-server-from-deferral). Claude Code ждёт при запуске инструментов этого сервера, ограничено тем же сроком, в то время как другие серверы продолжают подключаться в фоновом режиме; удалённый сервер с кэшированным списком инструментов предоставляет их без подключения, согласно таблице выше.

Сообщение `system` с подтипом `init` сообщает статус каждого сервера в момент его отправки; см. [Обработка ошибок](#error-handling) для чтения этих статусов.

<h2 id="allow-mcp-tools">
  Разрешить инструменты MCP
</h2>

Инструменты MCP требуют явного разрешения перед тем, как Claude сможет их использовать. Без разрешения Claude увидит, что инструменты доступны, но не сможет их вызывать.

<h3 id="tool-naming-convention">
  Соглашение об именовании инструментов
</h3>

Инструменты MCP следуют шаблону именования `mcp__<server-name>__<tool-name>`. Например, сервер GitHub с именем `"github"` с инструментом `list_issues` становится `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Автоматическое одобрение с allowedTools
</h3>

Используйте `allowedTools` для автоматического одобрения определённых инструментов MCP, чтобы Claude мог их использовать без запроса разрешения:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: [
        "mcp__github__*", // All tools from the github server
        "mcp__db__query", // Only the query tool from db server
        "mcp__slack__send_message" // Only send_message from slack server
      ]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=[
          "mcp__github__*",  # All tools from the github server
          "mcp__db__query",  # Only the query tool from db server
          "mcp__slack__send_message",  # Only send_message from slack server
      ],
  )
  ```
</CodeGroup>

Подстановочные знаки (`*`) позволяют вам разрешить все инструменты с сервера без перечисления каждого отдельно.

<Note>
  **Предпочитайте `allowedTools` режимам разрешений для доступа MCP.** `permissionMode: "acceptEdits"` не одобряет автоматически инструменты MCP (только редактирование файлов и команды Bash файловой системы). `permissionMode: "bypassPermissions"` одобряет автоматически инструменты MCP, но также отключает большинство других запросов безопасности, что шире, чем необходимо; см. [Как оцениваются разрешения](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) для запросов, которые остаются. Подстановочный знак в `allowedTools` предоставляет ровно тот сервер MCP, который вам нужен, и ничего больше. См. [Режимы разрешений](/docs/ru/agent-sdk/permissions#permission-modes) для полного сравнения.
</Note>

<h3 id="discover-available-tools">
  Обнаружение доступных инструментов
</h3>

Чтобы увидеть, какие инструменты предоставляет сервер MCP, проверьте документацию сервера или проверьте массив `tools` в инициализирующем сообщении `system`. Имена инструментов MCP начинаются с `mcp__`.

Claude Code выдаёт инициализирующее сообщение после [ожидания подключения на первом ходу](#connection-timing) для серверов, переданных в `options.mcpServers`, поэтому массив `tools` содержит инструменты `mcp__` каждого сервера, который подключился к этому моменту, плюс те, у которых есть [кэшированный список инструментов](#connection-timing), которые подключаются при первом использовании. Инструменты любого другого сервера, который ещё не подключился, отсутствуют; см. [Обработка ошибок](#error-handling) для чтения статуса каждого сервера.

Этот фильтр выводит имена инструментов MCP:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    mcpServers: {
      // your servers
    },
  };

  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      const mcpTools = message.tools.filter((name) => name.startsWith("mcp__"));
      console.log("Available MCP tools:", mcpTools);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              # your servers
          },
      )
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              mcp_tools = [t for t in message.data.get("tools", []) if t.startswith("mcp__")]
              print("Available MCP tools:", mcp_tools)


  asyncio.run(main())
  ```
</CodeGroup>

Вы также можете попросить Claude перечислить инструменты, доступные с сервера.

<h2 id="transport-types">
  Типы транспорта
</h2>

MCP серверы взаимодействуют с вашим агентом, используя различные протоколы транспорта. Проверьте документацию сервера, чтобы узнать, какой транспорт он поддерживает:

* Если в документации указана **команда для запуска** (например, `npx @modelcontextprotocol/server-filesystem`), используйте stdio
* Если в документации указан **URL**, используйте HTTP или SSE
* Если вы создаёте свои собственные инструменты в коде, используйте SDK MCP сервер

<h3 id="stdio-servers">
  stdio серверы
</h3>

Локальные процессы, которые взаимодействуют через stdin/stdout. Используйте это для MCP серверов, которые вы запускаете на одной машине. Для формата `.mcp.json` используйте те же поля, показанные в [From a config file](#from-a-config-file). В коде передайте команду и её аргументы. Замените `/Users/me/projects` на директорию на вашей машине:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__read_file", "mcp__filesystem__list_directory"]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          "filesystem": {
              "command": "npx",
              "args": [
                  "-y",
                  "@modelcontextprotocol/server-filesystem",
                  "/Users/me/projects",
              ],
          }
      },
      allowed_tools=["mcp__filesystem__read_file", "mcp__filesystem__list_directory"],
  )
  ```
</CodeGroup>

<h3 id="http/sse-servers">
  HTTP/SSE серверы
</h3>

Используйте HTTP или SSE для облачных MCP серверов и удалённых API. Для формата `.mcp.json` используйте те же поля, что и в примере в [HTTP headers for remote servers](#http-headers-for-remote-servers), с `"type": "sse"` для SSE сервера. В коде передайте URL сервера:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        "remote-api": {
          type: "sse",
          url: "https://api.example.com/mcp/sse",
          headers: {
            Authorization: `Bearer ${process.env.API_TOKEN}`
          }
        }
      },
      allowedTools: ["mcp__remote-api__*"]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          "remote-api": {
              "type": "sse",
              "url": "https://api.example.com/mcp/sse",
              "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
          }
      },
      allowed_tools=["mcp__remote-api__*"],
  )
  ```
</CodeGroup>

Для потокового HTTP транспорта используйте `"type": "http"` вместо этого. В `.mcp.json` и других JSON файлах конфигурации `"streamable-http"` принимается как псевдоним для `"http"`. Тип `McpHttpServerConfig` в SDK объявляет только `"http"`, поэтому используйте `"http"` для серверов, которые вы передаёте в коде.

<h3 id="sdk-mcp-servers">
  SDK MCP серверы
</h3>

Определите пользовательские инструменты непосредственно в коде вашего приложения вместо запуска отдельного процесса сервера. Подробности реализации см. в [custom tools guide](/docs/ru/agent-sdk/custom-tools).

SDK MCP сервер, зарегистрированный запросом управления [`initialize`](/docs/ru/agent-sdk/typescript#sdkcontrolinitializeresponse), начинает подключаться сразу же после обработки запроса Claude Code.

<h2 id="mcp-tool-search">
  Поиск инструментов MCP
</h2>

Когда у вас настроено много инструментов MCP, определения инструментов могут занимать значительную часть вашего контекстного окна. Поиск инструментов решает эту проблему, скрывая определения инструментов из контекста и загружая только те, которые Claude нужны для каждого хода.

Поиск инструментов включен по умолчанию. Дополнительную информацию о параметрах конфигурации, лучших практиках и использовании поиска инструментов с пользовательскими инструментами SDK см. в разделе [Поиск инструментов](/docs/ru/agent-sdk/tool-search).

<h2 id="authentication">
  Аутентификация
</h2>

Большинство серверов MCP требуют аутентификации для доступа к внешним сервисам. Передавайте учетные данные через переменные окружения в конфигурации сервера.

<h3 id="pass-credentials-via-environment-variables">
  Передача учетных данных через переменные окружения
</h3>

Используйте поле `env` для передачи ключей API, токенов и других учетных данных на сервер MCP:

<Tabs>
  <Tab title="В коде">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "api-server": {
              command: "npx",
              args: ["-y", "@your-org/api-mcp-server"],
              env: {
                API_KEY: process.env.API_KEY
              }
            }
          },
          allowedTools: ["mcp__api-server__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "api-server": {
                  "command": "npx",
                  "args": ["-y", "@your-org/api-mcp-server"],
                  "env": {"API_KEY": os.environ["API_KEY"]},
              }
          },
          allowed_tools=["mcp__api-server__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "api-server": {
          "command": "npx",
          "args": ["-y", "@your-org/api-mcp-server"],
          "env": {
            "API_KEY": "${API_KEY}"
          }
        }
      }
    }
    ```

    Синтаксис `${API_KEY}` раскрывает переменные окружения во время выполнения.
  </Tab>
</Tabs>

<h3 id="http-headers-for-remote-servers">
  HTTP-заголовки для удаленных серверов
</h3>

Для HTTP и SSE серверов передавайте заголовки аутентификации непосредственно в конфигурации сервера:

<Tabs>
  <Tab title="В коде">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "secure-api": {
              type: "http",
              url: "https://api.example.com/mcp",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__secure-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "secure-api": {
                  "type": "http",
                  "url": "https://api.example.com/mcp",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__secure-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "secure-api": {
          "type": "http",
          "url": "https://api.example.com/mcp",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```

    Синтаксис `${API_TOKEN}` раскрывает переменные окружения во время выполнения.
  </Tab>
</Tabs>

Полный рабочий пример удаленного сервера с аутентификацией через заголовки см. в разделе [Список проблем из репозитория](#list-issues-from-a-repository).

<h3 id="oauth2-authentication">
  Аутентификация OAuth2
</h3>

[Спецификация MCP поддерживает OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) для авторизации. SDK не открывает браузер и не запускает интерактивный поток OAuth. Когда настроенный сервер возвращает запрос авторизации и нет сохраненного токена, запуск агента продолжается без инструментов этого сервера, и сервер сообщает статус `needs-auth`. Массив `mcp_servers` [системного инициализирующего сообщения](/docs/ru/agent-sdk/typescript#sdksystemmessage) может по-прежнему показывать `pending` для этого сервера при его отправке. Чтобы подтвердить, требуются ли серверу учетные данные, опросите `mcpServerStatus()` в TypeScript SDK или [`get_mcp_status()`](/docs/ru/agent-sdk/python#methods) в Python.

Для предоставления учетных данных завершите поток OAuth в своем приложении и передайте полученный токен доступа в `headers` сервера:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // После завершения потока OAuth в вашем приложении.
  // Реализуйте getAccessTokenFromOAuthFlow для вашего поставщика OAuth.
  const accessToken = await getAccessTokenFromOAuthFlow();

  const options = {
    mcpServers: {
      "oauth-api": {
        type: "http",
        url: "https://api.example.com/mcp",
        headers: {
          Authorization: `Bearer ${accessToken}`
        }
      }
    },
    allowedTools: ["mcp__oauth-api__*"]
  };
  ```

  ```python Python theme={null}
  # После завершения потока OAuth в вашем приложении.
  # Реализуйте get_access_token_from_oauth_flow для вашего поставщика OAuth.
  access_token = await get_access_token_from_oauth_flow()

  options = ClaudeAgentOptions(
      mcp_servers={
          "oauth-api": {
              "type": "http",
              "url": "https://api.example.com/mcp",
              "headers": {"Authorization": f"Bearer {access_token}"},
          }
      },
      allowed_tools=["mcp__oauth-api__*"],
  )
  ```
</CodeGroup>

<h2 id="examples">
  Примеры
</h2>

<h3 id="list-issues-from-a-repository">
  Список проблем из репозитория
</h3>

Этот пример подключается к удалённому [серверу GitHub MCP](https://github.com/github/github-mcp-server) для получения списка последних проблем. Пример включает отладочное логирование для проверки подключения MCP и вызовов инструментов.

Перед запуском создайте [личный токен доступа GitHub](https://github.com/settings/personal-access-tokens) с правами на чтение репозиториев, которые вы хотите запросить, и установите его как переменную окружения:

```bash theme={null}
export GITHUB_TOKEN=YOUR_GITHUB_PAT
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          type: "http",
          url: "https://api.githubcopilot.com/mcp/",
          headers: {
            Authorization: `Bearer ${process.env.GITHUB_TOKEN}`
          }
        }
      },
      allowedTools: ["mcp__github__list_issues"]
    }
  })) {
    // Verify MCP server connected successfully
    if (message.type === "system" && message.subtype === "init") {
      console.log("MCP servers:", message.mcp_servers);
    }

    // Log when Claude calls an MCP tool
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "tool_use" && block.name.startsWith("mcp__")) {
          console.log("MCP tool called:", block.name);
        }
      }
    }

    // Print the final result
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      SystemMessage,
      AssistantMessage,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "type": "http",
                  "url": "https://api.githubcopilot.com/mcp/",
                  "headers": {"Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )

      async for message in query(
          prompt="List the 3 most recent issues in anthropics/claude-code",
          options=options,
      ):
          # Verify MCP server connected successfully
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("MCP servers:", message.data.get("mcp_servers"))

          # Log when Claude calls an MCP tool
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "name") and block.name.startswith("mcp__"):
                      print("MCP tool called:", block.name)

          # Print the final result
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

В строке `MCP servers:` статус `connected` для `github` подтверждает, что токен работает. Если Claude Code имеет [кэшированный список инструментов](#connection-timing) для сервера, статус может отображаться как `pending`, и сервер подключится при первом вызове инструмента. Если статус `failed` или `needs-auth`, см. [Обработка ошибок](#error-handling) перед тем, как доверять результату, так как Claude может вернуться к встроенным инструментам, когда сервер недоступен.

<h3 id="query-a-database">
  Запрос к базе данных
</h3>

Этот пример использует [DBHub](https://github.com/bytebase/dbhub) для запроса к базе данных Postgres. Агент автоматически обнаруживает схему базы данных, пишет SQL-запрос и возвращает результаты.

Инструмент `execute_sql` DBHub выполняет любой SQL, который выдаёт агент, включая операции записи, если вы это не ограничите. Установка `readonly = true` в [файле конфигурации DBHub](https://dbhub.ai/config/toml) заставляет DBHub отклонять операторы `INSERT`, `UPDATE`, `DELETE` и DDL, поэтому пример не может изменять ваши данные, даже если агент выдаст операцию записи. DBHub разрешает `${DATABASE_URL}` из переменных окружения процесса при загрузке конфига, поэтому строка подключения остаётся вне файла. Создайте этот файл `dbhub.toml` рядом с вашим скриптом:

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

Затем скрипт указывает DBHub на файл конфигурации вместо прямой передачи строки подключения. Перед запуском установите переменную окружения `DATABASE_URL` на вашу строку подключения. Замените значения-заполнители на детали вашей базы данных:

```bash theme={null}
export DATABASE_URL=postgresql://user:password@localhost:5432/mydb
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // dbhub.toml sets readonly = true, so execute_sql rejects writes
          args: ["-y", "@bytebase/dbhub", "--config", "dbhub.toml"]
        }
      },
      allowedTools: ["mcp__postgres__execute_sql"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "postgres": {
                  "command": "npx",
                  # dbhub.toml sets readonly = true, so execute_sql rejects writes
                  "args": [
                      "-y",
                      "@bytebase/dbhub",
                      "--config",
                      "dbhub.toml",
                  ],
              }
          },
          allowed_tools=["mcp__postgres__execute_sql"],
      )

      # Natural language query - Claude writes the SQL
      async for message in query(
          prompt="How many users signed up last week? Break it down by day.",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Обработка ошибок
</h2>

MCP серверы могут не подключиться по различным причинам: процесс сервера может быть не установлен, учетные данные могут быть неверными или удаленный сервер может быть недоступен.

Claude Code отправляет сообщение `system` с подтипом `init` в начале каждого запроса. Это сообщение включает статус подключения для каждого MCP сервера. Поле `status` может быть `"pending"`, `"connected"`, `"failed"`, `"needs-auth"` или `"disabled"`. Claude Code отправляет сообщение init после [первого ожидания подключения](#connection-timing) для серверов, переданных в `options.mcpServers`, поэтому такой сервер, который подключился в течение ожидания, показывает `"connected"`.

В сообщении init не рассматривайте `"pending"` как ошибку само по себе. Это может означать любое из следующего:

* Сервер еще не подключился. См. [как долго Claude Code ждет его перед первым ходом](#connection-timing)
* Список инструментов сервера был [получен из кэша](#connection-timing), с подключением при первом использовании
* Срок подключения истек. Такой сервер сообщает `"pending"` или `"failed"` в зависимости от времени

Проверьте `"failed"` или `"needs-auth"` для обнаружения серверов, которые не будут пригодны для использования:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Process data",
      options: {
        mcpServers: {
          // Replace dataServer with your server configuration
          "data-processor": dataServer
        }
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        const unavailableServers = message.mcp_servers.filter(
          (s) => s.status === "failed" || s.status === "needs-auth"
        );

        if (unavailableServers.length > 0) {
          console.warn("Unavailable MCP servers:", unavailableServers);
        }
      }

      if (message.type === "result" && message.subtype === "error_during_execution") {
        console.error("Execution failed");
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branch above has
    // already run; a failure to start or reach the Claude Code process
    // yields no result message. MCP servers that fail to connect don't
    // throw: use the status check above, and note that servers still
    // "pending" at init need a later status check.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      # Replace data_server with your server configuration
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      try:
          async for message in query(prompt="Process data", options=options):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  unavailable_servers = [
                      s
                      for s in message.data.get("mcp_servers", [])
                      if s.get("status") in ("failed", "needs-auth")
                  ]

                  if unavailable_servers:
                      print(f"Unavailable MCP servers: {unavailable_servers}")

              if (
                  isinstance(message, ResultMessage)
                  and message.subtype == "error_during_execution"
              ):
                  print("Execution failed")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branch above has
          # already run; a failure to start or reach the Claude Code process
          # yields no result message. MCP servers that fail to connect don't
          # raise: use the status check above, and note that servers still
          # "pending" at init need a later status check.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Статус удаленного сервера также может измениться после того, как он сообщит `"connected"`. Когда соединение с ним разрывается во время сеанса, Claude Code переводит сервер обратно в `"pending"` во время [переподключения](/docs/ru/mcp#automatic-reconnection). Последующий вызов `mcpServerStatus()` в TypeScript или [`ClaudeSDKClient.get_mcp_status()`](/docs/ru/agent-sdk/python#methods) в Python может затем сообщить `"pending"` для сервера, который вы видели подключенным ранее, без каких-либо изменений конфигурации с вашей стороны.

После пяти неудачных попыток переподключения сервер сообщает `"failed"` или `"needs-auth"`, когда ему требуется повторная авторизация. Для повторной попытки вручную вызовите [`reconnectMcpServer()`](/docs/ru/agent-sdk/typescript#methods) в TypeScript или [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/ru/agent-sdk/python#methods) в Python.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="server-shows-failed-status">
  Server shows "failed" status
</h3>

Check the `init` message to see which servers failed to connect:

<CodeGroup>
  ```typescript TypeScript theme={null}
  if (message.type === "system" && message.subtype === "init") {
    for (const server of message.mcp_servers) {
      if (server.status === "failed") {
        console.error(`Server ${server.name} failed to connect`);
      }
    }
  }
  ```

  ```python Python theme={null}
  if isinstance(message, SystemMessage) and message.subtype == "init":
      for server in message.data.get("mcp_servers", []):
          if server.get("status") == "failed":
              print(f"Server {server['name']} failed to connect")
  ```
</CodeGroup>

A `"pending"` status doesn't mean the server failed. See [Обработка ошибок](#error-handling) for the cases it covers at init. To get updated statuses later in the session, call the query's `mcpServerStatus()` method in the TypeScript SDK, or [`ClaudeSDKClient.get_mcp_status()`](/docs/ru/agent-sdk/python#methods) in Python.

Common causes:

* **Missing environment variables**: Ensure required tokens and credentials are set. For stdio servers, check the `env` field matches what the server expects.
* **Server not installed**: For `npx` commands, verify the package exists and Node.js is in your PATH.
* **Invalid connection string**: For database servers, verify the connection string format and that the database is accessible.
* **Network issues**: For remote HTTP/SSE servers, check the URL is reachable and any firewalls allow the connection.

<h3 id="tools-not-being-called">
  Tools not being called
</h3>

If Claude sees tools but doesn't use them, check that you've granted permission with `allowedTools`:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=["mcp__servername__*"],  # Auto-approve calls from this server
  )
  ```
</CodeGroup>

<h3 id="connection-timeouts">
  Connection timeouts
</h3>

MCP server connections time out after 30 seconds by default. To change how long a running tool call may take, set [`MCP_TOOL_TIMEOUT`](/docs/ru/env-vars). If your server takes longer to start, the connection fails. Raise the connection limit with the [`MCP_TIMEOUT`](/docs/ru/env-vars) environment variable, in milliseconds. For servers that need more startup time, also consider:

* Using a lighter-weight server if available
* Pre-warming the server before starting your agent
* Checking server logs for slow initialization causes

In TypeScript, you can set the tool-call limit for a single [SDK MCP server](#sdk-mcp-servers) by passing [`timeout` to `createSdkMcpServer()`](/docs/ru/agent-sdk/typescript#createsdkmcpserver).

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Tool output exceeds maximum allowed tokens
</h3>

The SDK applies the same MCP output limit as Claude Code. When a tool result with no image content is larger than 25,000 tokens, Claude Code saves the output to a file and replaces the tool result with an error message that names the file path, so the agent can read the output back in portions.

Raise the limit with the [`MAX_MCP_OUTPUT_TOKENS`](/docs/ru/env-vars) environment variable. See [MCP output limits and warnings](/docs/ru/mcp#mcp-output-limits-and-warnings) for the full behavior, including how a server can declare a higher per-tool limit with the `anthropic/maxResultSizeChars` annotation.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* **[Руководство по пользовательским инструментам](/docs/ru/agent-sdk/custom-tools)**: Создайте собственный MCP сервер, который работает в процессе вашего приложения SDK
* **[Разрешения](/docs/ru/agent-sdk/permissions)**: Контролируйте, какие MCP инструменты может использовать ваш агент, с помощью `allowedTools` и `disallowedTools`
* **[Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript)**: Полный справочник API, включая параметры конфигурации MCP
* **[Справочник Python SDK](/docs/ru/agent-sdk/python)**: Полный справочник API, включая параметры конфигурации MCP
* **[Каталог MCP серверов](https://github.com/modelcontextprotocol/servers)**: Просмотрите доступные MCP серверы для баз данных, API и многого другого
