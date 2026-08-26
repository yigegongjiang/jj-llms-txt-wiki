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
  Добавление MCP сервера
</h2>

Вы можете настроить MCP серверы в коде при вызове `query()` или в файле `.mcp.json`, загруженном через [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  В коде
</h3>

Передайте MCP серверы непосредственно в опции `mcpServers`:

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

Создайте файл `.mcp.json` в корне вашего проекта. Файл загружается, когда включен источник параметров `project`, что происходит по умолчанию для опций `query()`. Если вы явно установите `settingSources`, включите `"project"` для загрузки этого файла:

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

<h2 id="allow-mcp-tools">
  Разрешение MCP инструментов
</h2>

MCP инструменты требуют явного разрешения перед тем, как Claude сможет их использовать. Без разрешения Claude увидит, что инструменты доступны, но не сможет их вызывать.

<h3 id="tool-naming-convention">
  Соглашение об именовании инструментов
</h3>

MCP инструменты следуют шаблону именования `mcp__<server-name>__<tool-name>`. Например, сервер GitHub с именем `"github"` с инструментом `list_issues` становится `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Предоставление доступа с помощью allowedTools
</h3>

Используйте `allowedTools` для автоматического одобрения определённых MCP инструментов, чтобы Claude мог их использовать без запроса разрешения:

```typescript hidelines={1,-1} theme={null}
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

Подстановочные знаки (`*`) позволяют разрешить все инструменты с сервера без перечисления каждого отдельно.

<Note>
  **Предпочитайте `allowedTools` режимам разрешений для доступа MCP.** `permissionMode: "acceptEdits"` не одобряет автоматически MCP инструменты (только редактирование файлов и команды Bash файловой системы). `permissionMode: "bypassPermissions"` одобряет автоматически MCP инструменты, но также отключает все остальные подсказки безопасности, что шире, чем необходимо; см. [Как оцениваются разрешения](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) для подсказок, которые остаются. Подстановочный знак в `allowedTools` предоставляет доступ ровно к нужному MCP серверу и ничему больше. См. [Режимы разрешений](/docs/ru/agent-sdk/permissions#permission-modes) для полного сравнения.
</Note>

<h3 id="discover-available-tools">
  Обнаружение доступных инструментов
</h3>

Чтобы увидеть, какие инструменты предоставляет MCP сервер, проверьте документацию сервера или подключитесь к серверу и проверьте сообщение инициализации `system`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available MCP tools:", message.mcp_servers);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, SystemMessage


  async def main():
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available MCP tools:", message.data["mcp_servers"])


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="transport-types">
  Типы транспорта
</h2>

MCP серверы взаимодействуют с вашим агентом, используя различные протоколы транспорта. Проверьте документацию сервера, чтобы увидеть, какой транспорт он поддерживает:

* Если в документации указана **команда для запуска** (например, `npx @modelcontextprotocol/server-github`), используйте stdio
* Если в документации указан **URL**, используйте HTTP или SSE
* Если вы создаёте свои собственные инструменты в коде, используйте SDK MCP сервер

<h3 id="stdio-servers">
  Серверы stdio
</h3>

Локальные процессы, которые взаимодействуют через stdin/stdout. Используйте это для MCP серверов, которые вы запускаете на той же машине:

<Tabs>
  <Tab title="В коде">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues", "mcp__github__search_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues", "mcp__github__search_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

<h3 id="http/sse-servers">
  Серверы HTTP/SSE
</h3>

Используйте HTTP или SSE для облачных MCP серверов и удалённых API:

<Tabs>
  <Tab title="В коде">
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
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "remote-api": {
          "type": "sse",
          "url": "https://api.example.com/mcp/sse",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Для потокового HTTP транспорта используйте `"type": "http"` вместо этого. В `.mcp.json` и других JSON файлах конфигурации `"streamable-http"` принимается как псевдоним для `"http"`. Программный параметр `mcpServers` принимает только `"http"`.

<h3 id="sdk-mcp-servers">
  SDK MCP серверы
</h3>

Определите пользовательские инструменты непосредственно в коде вашего приложения вместо запуска отдельного процесса сервера. Подробности реализации см. в [руководстве по пользовательским инструментам](/docs/ru/agent-sdk/custom-tools).

<h2 id="mcp-tool-search">
  Поиск MCP инструментов
</h2>

Когда у вас настроено много MCP инструментов, определения инструментов могут потребить значительную часть вашего контекстного окна. Поиск инструментов решает эту проблему, скрывая определения инструментов из контекста и загружая только те, которые Claude нужны для каждого хода.

Поиск инструментов включен по умолчанию. Подробности конфигурации см. в [Поиск инструментов](/docs/ru/agent-sdk/tool-search).

Для получения дополнительной информации, включая лучшие практики и использование поиска инструментов с пользовательскими SDK инструментами, см. [руководство по поиску инструментов](/docs/ru/agent-sdk/tool-search).

<h2 id="authentication">
  Аутентификация
</h2>

Большинство MCP серверов требуют аутентификации для доступа к внешним сервисам. Передавайте учётные данные через переменные окружения в конфигурации сервера.

<h3 id="pass-credentials-via-environment-variables">
  Передача учётных данных через переменные окружения
</h3>

Используйте поле `env` для передачи ключей API, токенов и других учётных данных на MCP сервер:

<Tabs>
  <Tab title="В коде">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```

    Синтаксис `${GITHUB_TOKEN}` расширяет переменные окружения во время выполнения.
  </Tab>
</Tabs>

Полный рабочий пример с логированием отладки см. в разделе [Список проблем из репозитория](#list-issues-from-a-repository).

<h3 id="http-headers-for-remote-servers">
  HTTP заголовки для удалённых серверов
</h3>

Для серверов HTTP и SSE передавайте заголовки аутентификации непосредственно в конфигурации сервера:

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

    Синтаксис `${API_TOKEN}` расширяет переменные окружения во время выполнения.
  </Tab>
</Tabs>

<h3 id="oauth2-authentication">
  Аутентификация OAuth2
</h3>

[Спецификация MCP поддерживает OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) для авторизации. SDK не открывает браузер и не запускает интерактивный поток OAuth. Когда настроенный сервер возвращает запрос авторизации и нет сохранённого токена, выполнение агента продолжается без инструментов этого сервера, и сервер сообщается со статусом `needs-auth` в массиве `mcp_servers` [системного инициализирующего сообщения](/docs/ru/agent-sdk/typescript#sdksystemmessage). Проверьте этот массив при запуске, если ваш агент зависит от подключения конкретного сервера.

Для передачи учётных данных завершите поток OAuth в вашем приложении и передайте полученный токен доступа в заголовках сервера:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // After completing OAuth flow in your app
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
  # After completing OAuth flow in your app
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

Этот пример подключается к [GitHub MCP серверу](https://github.com/modelcontextprotocol/servers/tree/main/src/github) для списка последних проблем. Пример включает логирование отладки для проверки подключения MCP и вызовов инструментов.

Перед запуском создайте [личный токен доступа GitHub](https://github.com/settings/tokens) с областью `repo` и установите его как переменную окружения:

```bash theme={null}
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-github"],
          env: {
            GITHUB_TOKEN: process.env.GITHUB_TOKEN
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
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
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

<h3 id="query-a-database">
  Запрос к базе данных
</h3>

Этот пример использует [Postgres MCP сервер](https://github.com/modelcontextprotocol/servers/tree/main/src/postgres) для запроса к базе данных. Строка подключения передаётся как аргумент серверу. Агент автоматически обнаруживает схему базы данных, пишет SQL запрос и возвращает результаты:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Connection string from environment variable
  const connectionString = process.env.DATABASE_URL;

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // Pass connection string as argument to the server
          args: ["-y", "@modelcontextprotocol/server-postgres", connectionString]
        }
      },
      // Allow only read queries, not writes
      allowedTools: ["mcp__postgres__query"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      # Connection string from environment variable
      connection_string = os.environ["DATABASE_URL"]

      options = ClaudeAgentOptions(
          mcp_servers={
              "postgres": {
                  "command": "npx",
                  # Pass connection string as argument to the server
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-postgres",
                      connection_string,
                  ],
              }
          },
          # Allow only read queries, not writes
          allowed_tools=["mcp__postgres__query"],
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

MCP серверы могут не подключиться по различным причинам: процесс сервера может быть не установлен, учётные данные могут быть неверными или удалённый сервер может быть недоступен.

SDK отправляет сообщение `system` с подтипом `init` в начале каждого запроса. Это сообщение включает статус подключения для каждого MCP сервера. Проверьте поле `status` для обнаружения сбоев подключения перед тем, как агент начнёт работать:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Process data",
    options: {
      mcpServers: {
        "data-processor": dataServer
      }
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      const failedServers = message.mcp_servers.filter((s) => s.status !== "connected");

      if (failedServers.length > 0) {
        console.warn("Failed to connect:", failedServers);
      }
    }

    if (message.type === "result" && message.subtype === "error_during_execution") {
      console.error("Execution failed");
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      async for message in query(prompt="Process data", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              failed_servers = [
                  s
                  for s in message.data.get("mcp_servers", [])
                  if s.get("status") != "connected"
              ]

              if failed_servers:
                  print(f"Failed to connect: {failed_servers}")

          if (
              isinstance(message, ResultMessage)
              and message.subtype == "error_during_execution"
          ):
              print("Execution failed")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="server-shows-failed-status">
  Сервер показывает статус "failed"
</h3>

Проверьте сообщение `init`, чтобы увидеть, какие серверы не подключились:

```typescript theme={null}
if (message.type === "system" && message.subtype === "init") {
  for (const server of message.mcp_servers) {
    if (server.status === "failed") {
      console.error(`Server ${server.name} failed to connect`);
    }
  }
}
```

Частые причины:

* **Отсутствующие переменные окружения**: Убедитесь, что требуемые токены и учётные данные установлены. Для серверов stdio проверьте, что поле `env` соответствует тому, что ожидает сервер.
* **Сервер не установлен**: Для команд `npx` проверьте, что пакет существует и Node.js находится в вашем PATH.
* **Неверная строка подключения**: Для серверов баз данных проверьте формат строки подключения и что база данных доступна.
* **Проблемы с сетью**: Для удалённых серверов HTTP/SSE проверьте, что URL доступен и любые брандмауэры разрешают подключение.

<h3 id="tools-not-being-called">
  Инструменты не вызываются
</h3>

Если Claude видит инструменты, но не использует их, проверьте, что вы предоставили разрешение с помощью `allowedTools`:

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
  }
};
```

<h3 id="connection-timeouts">
  Тайм-ауты подключения
</h3>

Подключения серверов MCP истекают по умолчанию через 30 секунд. Если ваш сервер требует больше времени для запуска, подключение не удастся. Увеличьте лимит с помощью переменной окружения [`MCP_TIMEOUT`](/docs/ru/env-vars), в миллисекундах. Для серверов, которым требуется больше времени на запуск, также рассмотрите:

* Использование более лёгкого сервера, если доступно
* Предварительный прогрев сервера перед запуском вашего агента
* Проверку логов сервера на предмет причин медленной инициализации

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  Выходные данные инструмента превышают максимально допустимое количество токенов
</h3>

SDK применяет тот же лимит выходных данных MCP, что и Claude Code. Когда результат инструмента больше 25 000 токенов, полный выход сохраняется в файл, а результат инструмента заменяется сообщением об ошибке, которое указывает путь к файлу, чтобы агент мог прочитать выход порциями. Увеличьте лимит с помощью переменной окружения [`MAX_MCP_OUTPUT_TOKENS`](/docs/ru/env-vars). Смотрите [MCP output limits and warnings](/docs/ru/mcp#mcp-output-limits-and-warnings) для полного поведения, включая то, как сервер может объявить более высокий лимит для каждого инструмента.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* **[Руководство по пользовательским инструментам](/docs/ru/agent-sdk/custom-tools)**: Создайте свой собственный MCP сервер, который работает в процессе с вашим приложением SDK
* **[Разрешения](/docs/ru/agent-sdk/permissions)**: Контролируйте, какие MCP инструменты может использовать ваш агент с помощью `allowedTools` и `disallowedTools`
* **[Ограничения выходных данных MCP и предупреждения](/docs/ru/mcp#mcp-output-limits-and-warnings)**: Как SDK обрабатывает результаты инструментов, превышающие `MAX_MCP_OUTPUT_TOKENS`, включая резервный вариант сохранения на диск и аннотацию `anthropic/maxResultSizeChars` для каждого инструмента
* **[Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript)**: Полный справочник API, включая опции конфигурации MCP
* **[Справочник Python SDK](/docs/ru/agent-sdk/python)**: Полный справочник API, включая опции конфигурации MCP
* **[Каталог MCP серверов](https://github.com/modelcontextprotocol/servers)**: Просмотрите доступные MCP серверы для баз данных, API и многого другого
