> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Предоставьте Claude пользовательские инструменты

> Определите пользовательские инструменты с помощью встроенного MCP-сервера Agent SDK, чтобы Claude мог вызывать ваши функции, обращаться к вашим API и выполнять операции, специфичные для вашей области.

Пользовательские инструменты расширяют Agent SDK, позволяя вам определять собственные функции, которые Claude может вызывать во время разговора. Используя встроенный MCP-сервер SDK, вы можете предоставить Claude доступ к базам данных, внешним API, логике, специфичной для вашей области, или любым другим возможностям, которые требует ваше приложение.

<h2 id="quick-reference">
  Краткая справка
</h2>

| Если вы хотите...                                         | Сделайте это                                                                                                                                                                                                                          |
| :-------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Определить инструмент                                     | Используйте [`@tool`](/docs/ru/agent-sdk/python#tool) (Python) или [`tool()`](/docs/ru/agent-sdk/typescript#tool) (TypeScript) с именем, описанием, схемой и обработчиком. См. [Создание пользовательского инструмента](#create-a-custom-tool). |
| Зарегистрировать инструмент с Claude                      | Оберните в `create_sdk_mcp_server` / `createSdkMcpServer` и передайте в `mcpServers` в `query()`. См. [Вызов пользовательского инструмента](#call-a-custom-tool).                                                                     |
| Предварительно одобрить инструмент                        | Добавьте в разрешённые инструменты. См. [Настройка разрешённых инструментов](#configure-allowed-tools).                                                                                                                               |
| Удалить встроенный инструмент из контекста Claude         | Передайте массив `tools`, содержащий только встроенные инструменты, которые вы хотите. См. [Настройка разрешённых инструментов](#configure-allowed-tools).                                                                            |
| Позволить Claude вызывать инструменты параллельно         | Установите `readOnlyHint: true` на инструментах без побочных эффектов. См. [Добавление аннотаций инструментов](#add-tool-annotations).                                                                                                |
| Контролировать сообщение об ошибке, которое читает Claude | Верните `isError: true` для составления сообщения вместо выброса необработанного исключения. См. [Обработка ошибок](#handle-errors).                                                                                                  |
| Вернуть изображения или файлы                             | Используйте блоки `image` или `resource` в массиве содержимого. См. [Возврат изображений и ресурсов](#return-images-and-resources).                                                                                                   |
| Вернуть результат в формате машиночитаемого JSON          | Установите `structuredContent` на результат. См. [Возврат структурированных данных](#return-structured-data).                                                                                                                         |
| Масштабировать до множества инструментов                  | Используйте [поиск инструментов](/docs/ru/agent-sdk/tool-search) для загрузки инструментов по требованию.                                                                                                                                  |

<h2 id="create-a-custom-tool">
  Создание пользовательского инструмента
</h2>

Инструмент определяется четырьмя частями, передаваемыми в качестве аргументов вспомогательной функции [`tool()`](/docs/ru/agent-sdk/typescript#tool) в TypeScript или декоратору [`@tool`](/docs/ru/agent-sdk/python#tool) в Python:

* **Name:** уникальный идентификатор, который Claude использует для вызова инструмента.
* **Description:** описание того, что делает инструмент. Claude читает это, чтобы решить, когда его вызывать.
* **Input schema:** аргументы, которые должен предоставить Claude. В TypeScript это всегда [Zod schema](https://zod.dev/), и `args` обработчика автоматически типизируются из него. В Python это словарь, отображающий имена на типы, например `{"latitude": float}`, который SDK преобразует в JSON Schema для вас. Декоратор Python также принимает полный словарь [JSON Schema](https://json-schema.org/understanding-json-schema/about) непосредственно, когда вам нужны перечисления, диапазоны, необязательные поля или вложенные объекты.
* **Handler:** асинхронная функция, которая запускается, когда Claude вызывает инструмент. Она получает проверенные аргументы и должна возвращать объект с:
  * `content` (обязательно): массив блоков результатов, каждый с `type` равным `"text"`, `"image"`, `"audio"`, `"resource"` или `"resource_link"`. Смотрите [Return images and resources](#return-images-and-resources) для блоков, отличных от текста.
  * `structuredContent` (необязательно): объект JSON, содержащий результат как машиночитаемые данные, возвращаемые вместе с `content`. Смотрите [Return structured data](#return-structured-data).
  * `isError` (необязательно): установите значение `true`, чтобы сигнализировать об ошибке инструмента, чтобы Claude мог на неё реагировать. Смотрите [Handle errors](#handle-errors).

После определения инструмента оберните его в сервер с помощью [`createSdkMcpServer`](/docs/ru/agent-sdk/typescript#createsdkmcpserver) (TypeScript) или [`create_sdk_mcp_server`](/docs/ru/agent-sdk/python#create_sdk_mcp_server) (Python). Сервер работает внутри процесса вашего приложения, а не как отдельный процесс.

<h3 id="weather-tool-example">
  Пример инструмента Weather
</h3>

Этот пример определяет инструмент `get_temperature` и оборачивает его в MCP сервер. Он только настраивает инструмент; чтобы передать его в `query` и запустить его, смотрите [Call a custom tool](#call-a-custom-tool) ниже.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  import httpx
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # Define a tool: name, description, input schema, handler
  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
  )
  async def get_temperature(args: dict[str, Any]) -> dict[str, Any]:
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "current": "temperature_2m",
                  "temperature_unit": "fahrenheit",
              },
          )
          data = response.json()

      # Return a content array - Claude sees this as the tool result
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Temperature: {data['current']['temperature_2m']}°F",
              }
          ]
      }


  # Wrap the tool in an in-process MCP server
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  // Define a tool: name, description, input schema, handler
  const getTemperature = tool(
    "get_temperature",
    "Get the current temperature at a location",
    {
      latitude: z.number().describe("Latitude coordinate"), // .describe() adds a field description Claude sees
      longitude: z.number().describe("Longitude coordinate")
    },
    async (args) => {
      // args is typed from the schema: { latitude: number; longitude: number }
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`
      );
      const data: any = await response.json();

      // Return a content array - Claude sees this as the tool result
      return {
        content: [{ type: "text", text: `Temperature: ${data.current.temperature_2m}°F` }]
      };
    }
  );

  // Wrap the tool in an in-process MCP server
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature]
  });
  ```
</CodeGroup>

Смотрите справку [`tool()`](/docs/ru/agent-sdk/typescript#tool) TypeScript или справку [`@tool`](/docs/ru/agent-sdk/python#tool) Python для полных деталей параметров, включая форматы входных JSON Schema и структуру возвращаемого значения.

<Tip>
  Чтобы сделать параметр необязательным: в TypeScript добавьте `.default()` к полю Zod. В Python словарь schema рассматривает каждый ключ как обязательный, поэтому оставьте параметр вне schema, упомяните его в строке описания и прочитайте его с помощью `args.get()` в обработчике. Инструмент [`get_precipitation_chance` ниже](#add-more-tools) показывает оба паттерна.
</Tip>

<h3 id="call-a-custom-tool">
  Вызов пользовательского инструмента
</h3>

Передайте MCP сервер, который вы создали, в `query` через опцию `mcpServers`. Ключ в `mcpServers` становится сегментом `{server_name}` в полностью квалифицированном имени каждого инструмента: `mcp__{server_name}__{tool_name}`. Перечислите это имя в `allowedTools`, чтобы инструмент работал без запроса разрешения.

Эти фрагменты повторно используют `weatherServer` из [примера инструмента Weather](#weather-tool-example), чтобы спросить Claude о погоде в определённом месте.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"weather": weather_server},
          allowed_tools=["mcp__weather__get_temperature"],
      )

      async for message in query(
          prompt="What's the temperature in San Francisco?",
          options=options,
      ):
          # ResultMessage is the final message after all tool calls complete
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "What's the temperature in San Francisco?",
    options: {
      mcpServers: { weather: weatherServer },
      allowedTools: ["mcp__weather__get_temperature"]
    }
  })) {
    // "result" is the final message after all tool calls complete
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Объедините этот фрагмент с определениями инструмента и сервера из [примера инструмента weather](#weather-tool-example) в одном файле, затем запустите его с помощью `python weather.py` для Python или `npx tsx weather.ts` для TypeScript. Claude вызывает `get_temperature` и скрипт выводит однострочный ответ с текущей температурой в Сан-Франциско.

<h3 id="add-more-tools">
  Добавление дополнительных инструментов
</h3>

Сервер содержит столько инструментов, сколько вы перечислите в его массиве `tools`. Если на сервере более одного инструмента, вы можете перечислить каждый в `allowedTools` отдельно или использовать подстановочный знак `mcp__weather__*`, чтобы охватить каждый инструмент, который сервер предоставляет.

Пример ниже определяет второй инструмент, `get_precipitation_chance`, и заменяет определение `weatherServer` из [примера инструмента weather](#weather-tool-example) на то, которое перечисляет оба инструмента в массиве.

<CodeGroup>
  ```python Python theme={null}
  # Define a second tool for the same server
  @tool(
      "get_precipitation_chance",
      "Get the hourly precipitation probability for a location. "
      "Optionally pass 'hours' (1-24) to control how many hours to return.",
      {"latitude": float, "longitude": float},
  )
  async def get_precipitation_chance(args: dict[str, Any]) -> dict[str, Any]:
      # 'hours' isn't in the schema - read it with .get() to make it optional
      hours = args.get("hours", 12)
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "hourly": "precipitation_probability",
                  "forecast_days": 1,
              },
          )
          data = response.json()
      chances = data["hourly"]["precipitation_probability"][:hours]

      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Next {hours} hours: {'%, '.join(map(str, chances))}%",
              }
          ]
      }


  # Rebuild the server with both tools in the array
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature, get_precipitation_chance],
  )
  ```

  ```typescript TypeScript theme={null}
  // Define a second tool for the same server
  const getPrecipitationChance = tool(
    "get_precipitation_chance",
    "Get the hourly precipitation probability for a location",
    {
      latitude: z.number(),
      longitude: z.number(),
      hours: z
        .number()
        .int()
        .min(1)
        .max(24)
        .default(12) // .default() makes the parameter optional
        .describe("How many hours of forecast to return")
    },
    async (args) => {
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&hourly=precipitation_probability&forecast_days=1`
      );
      const data: any = await response.json();
      const chances = data.hourly.precipitation_probability.slice(0, args.hours);

      return {
        content: [{ type: "text", text: `Next ${args.hours} hours: ${chances.join("%, ")}%` }]
      };
    }
  );

  // Rebuild the server with both tools in the array
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature, getPrecipitationChance]
  });
  ```
</CodeGroup>

[Tool search](/docs/ru/agent-sdk/tool-search) включен по умолчанию и откладывает SDK MCP инструменты: Claude видит имя каждого инструмента в компактном списке и загружает его полную схему по требованию. С отключённым поиском инструментов каждый инструмент в этом массиве потребляет пространство контекстного окна на каждом ходу. В TypeScript передайте `alwaysLoad: true` в аргументе `extras` функции [`tool()`](/docs/ru/agent-sdk/typescript#tool) или в опциях [`createSdkMcpServer()`](/docs/ru/agent-sdk/typescript#createsdkmcpserver), чтобы сохранить полную схему инструмента в начальном приглашении.

<h3 id="add-tool-annotations">
  Добавление аннотаций инструмента
</h3>

[Tool annotations](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations) — это необязательные метаданные, описывающие поведение инструмента. Передайте их в качестве пятого аргумента вспомогательной функции `tool()` в TypeScript или через аргумент ключевого слова `annotations` для декоратора `@tool` в Python. Все поля подсказок являются логическими значениями.

| Field             | Default | Meaning                                                                                                                                |
| :---------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| `readOnlyHint`    | `false` | Инструмент не изменяет свою среду. Контролирует, может ли инструмент вызываться параллельно с другими инструментами только для чтения. |
| `destructiveHint` | `true`  | Инструмент может выполнять деструктивные обновления. Только информационное.                                                            |
| `idempotentHint`  | `false` | Повторные вызовы с одинаковыми аргументами не имеют дополнительного эффекта. Только информационное.                                    |
| `openWorldHint`   | `true`  | Инструмент достигает систем вне вашего процесса. Только информационное.                                                                |

Аннотации — это метаданные, а не принуждение. Инструмент, отмеченный как `readOnlyHint: true`, всё ещё может писать на диск, если это то, что делает обработчик. Держите аннотацию точной для обработчика.

Этот пример добавляет `readOnlyHint` к инструменту `get_temperature` из [примера инструмента weather](#weather-tool-example).

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import tool, ToolAnnotations


  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
      annotations=ToolAnnotations(
          readOnlyHint=True
      ),  # Lets Claude batch this with other read-only calls
  )
  async def get_temperature(args):
      return {"content": [{"type": "text", "text": "..."}]}
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

Смотрите `ToolAnnotations` в справке [TypeScript](/docs/ru/agent-sdk/typescript#toolannotations) или [Python](/docs/ru/agent-sdk/python#toolannotations).

<h2 id="control-tool-access">
  Управление доступом к инструментам
</h2>

[Пример инструмента weather](#weather-tool-example) зарегистрировал сервер и перечислил инструменты в `allowedTools`. В этом разделе рассматривается, как ограничить доступ, когда у вас есть несколько инструментов или вы хотите ограничить встроенные инструменты. Информацию о том, как конструируются имена инструментов, см. в разделе [Вызов пользовательского инструмента](#call-a-custom-tool).

<h3 id="configure-allowed-tools">
  Настройка разрешённых инструментов
</h3>

Опция `tools` и списки разрешённых/запрещённых инструментов влияют на два уровня: доступность, которая контролирует, появляется ли инструмент в контексте Claude, и разрешение, которое контролирует, одобрен ли вызов после того, как Claude попытается его выполнить. `tools` и записи `disallowedTools` с простым именем изменяют доступность. `allowedTools` и правила `disallowedTools` с областью действия изменяют разрешение. Если вы назовёте один из [инструментов отслеживания задач](/docs/ru/agent-sdk/todo-tracking#model-availability) в `allowedTools`, Claude Code также включит сеанс.

| Опция                     | Уровень     | Эффект                                                                                                                                                                                                                                                                                                              |
| :------------------------ | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `tools: ["Read", "Grep"]` | Доступность | Только перечисленные встроенные инструменты находятся в контексте Claude. Неперечисленные встроенные инструменты удаляются. Инструменты MCP не затрагиваются.                                                                                                                                                       |
| `tools: []`               | Доступность | Все встроенные инструменты удаляются. Claude может использовать только ваши инструменты MCP.                                                                                                                                                                                                                        |
| разрешённые инструменты   | Разрешение  | Перечисленные инструменты работают без запроса разрешения. Другие неперечисленные инструменты остаются доступными; вызовы проходят через [поток разрешений](/docs/ru/agent-sdk/permissions).                                                                                                                             |
| запрещённые инструменты   | Оба         | Простое имя инструмента, такое как `"Bash"`, удаляет инструмент из контекста Claude, как если бы вы его опустили из `tools`. Правило с областью действия, такое как `"Bash(rm *)"`, оставляет инструмент в контексте и отклоняет только вызовы, которые совпадают [как написано](/docs/ru/permissions#bash-rule-limits). |

Чтобы полностью удалить встроенный инструмент, опустите его из `tools` или перечислите его простое имя в `disallowedTools` (Python: `disallowed_tools`); оба варианта держат инструмент вне контекста, поэтому Claude никогда не попытается его использовать. Правило `disallowedTools` с областью действия блокирует совпадающие вызовы, но оставляет инструмент видимым, поэтому Claude может потратить ход, пытаясь его использовать. Полный порядок оценки см. в разделе [Настройка разрешений](/docs/ru/agent-sdk/permissions).

<h2 id="handle-errors">
  Обработка ошибок
</h2>

Ошибка обработчика не останавливает цикл агента. MCP-сервер SDK в процессе перехватывает необработанные исключения и возвращает их как результаты ошибок, поэтому то, как вы сообщаете об ошибке, определяет, что читает Claude, а не то, завершится ли запрос неудачей:

| Что происходит                                                                                  | Результат                                                                                                                                                                              |
| :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Обработчик выбрасывает необработанное исключение                                                | MCP-сервер преобразует его в результат ошибки, содержащий исходное сообщение об исключении. Claude видит это сообщение, и цикл агента продолжается.                                    |
| Обработчик перехватывает ошибку и возвращает `isError: true` (TS) / `"is_error": True` (Python) | Claude видит сообщение, которое вы составили. Вы можете добавить контекст, которого не хватает исходному исключению, например какой запрос не удался или что попробовать вместо этого. |

В обоих случаях Claude может повторить попытку, попробовать другой инструмент или объяснить сбой. Перехватывайте ошибки самостоятельно, когда исходное сообщение об исключении недостаточно для того, чтобы Claude мог действовать.

Приведённый ниже пример перехватывает два вида сбоев внутри обработчика и составляет сообщение об ошибке, которое читает Claude. Статус HTTP, отличный от 200, перехватывается из ответа и возвращается как результат ошибки. Ошибка сети или неверный JSON перехватываются окружающим блоком `try/except` (Python) или `try/catch` (TypeScript) и также возвращаются как результат ошибки. В обоих случаях Claude получает сообщение, которое описывает сбой вместо простой строки исключения.

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any
  from claude_agent_sdk import tool


  @tool(
      "fetch_data",
      "Fetch data from an API",
      {"endpoint": str},  # Simple schema
  )
  async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
      try:
          async with httpx.AsyncClient() as client:
              response = await client.get(args["endpoint"])
              if response.status_code != 200:
                  # Return the failure as a tool result so Claude can react to it.
                  # is_error marks this as a failed call rather than odd-looking data.
                  return {
                      "content": [
                          {
                              "type": "text",
                              "text": f"API error: {response.status_code} {response.reason_phrase}",
                          }
                      ],
                      "is_error": True,
                  }

              data = response.json()
              return {"content": [{"type": "text", "text": json.dumps(data, indent=2)}]}
      except Exception as e:
          # Composes the message Claude reads. An uncaught exception would
          # reach Claude as the raw str(e) with no context.
          return {
              "content": [{"type": "text", "text": f"Failed to fetch data: {str(e)}"}],
              "is_error": True,
          }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_data",
    "Fetch data from an API",
    {
      endpoint: z.string().url().describe("API endpoint URL")
    },
    async (args) => {
      try {
        const response = await fetch(args.endpoint);

        if (!response.ok) {
          // Return the failure as a tool result so Claude can react to it.
          // isError marks this as a failed call rather than odd-looking data.
          return {
            content: [
              {
                type: "text",
                text: `API error: ${response.status} ${response.statusText}`
              }
            ],
            isError: true
          };
        }

        const data = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(data, null, 2)
            }
          ]
        };
      } catch (error) {
        // Composes the message Claude reads. An uncaught throw would
        // reach Claude as the raw error message with no context.
        return {
          content: [
            {
              type: "text",
              text: `Failed to fetch data: ${error instanceof Error ? error.message : String(error)}`
            }
          ],
          isError: true
        };
      }
    }
  );
  ```
</CodeGroup>

<h2 id="return-images-and-resources">
  Возврат изображений и ресурсов
</h2>

Массив `content` в результате инструмента принимает блоки `text`, `image`, `audio`, `resource` и `resource_link`. Вы можете смешивать их в одном ответе. В TypeScript SDK сохраняет аудиоблоки на диск, и Claude получает текстовый блок с сохранённым путём файла; в Python SDK удаляет аудиоблоки из результата инструмента и регистрирует предупреждение.

Claude получает каждый блок ссылки на ресурс как текстовый блок, содержащий имя ссылки, URI и описание. В TypeScript ваше приложение также получает сами ссылки как [`resourceLinks`](/docs/ru/agent-sdk/typescript#sdkmcpresourcelink) в `tool_use_result` сообщения пользователя; в Python SDK преобразует их в текст перед тем, как CLI увидит результат, поэтому ключ Python [`resourceLinks`](/docs/ru/agent-sdk/python#usermessage) никогда не создаётся для встроенных инструментов.

<h3 id="images">
  Изображения
</h3>

Блок изображения содержит байты изображения встроенными, закодированными в base64. Поля URL нет. Чтобы вернуть изображение, которое находится по URL, получите его в обработчике, прочитайте байты ответа и закодируйте их в base64 перед возвратом. Результат обрабатывается как визуальный ввод.

| Поле       | Тип       | Примечания                                                                         |
| :--------- | :-------- | :--------------------------------------------------------------------------------- |
| `type`     | `"image"` |                                                                                    |
| `data`     | `string`  | Байты в кодировке Base64. Только raw base64, без префикса `data:image/...;base64,` |
| `mimeType` | `string`  | Обязательно. Например `image/png`, `image/jpeg`, `image/webp`, `image/gif`         |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx
  from claude_agent_sdk import tool


  # Define a tool that fetches an image from a URL and returns it to Claude
  @tool("fetch_image", "Fetch an image from a URL and return it to Claude", {"url": str})
  async def fetch_image(args):
      async with httpx.AsyncClient() as client:  # Fetch the image bytes
          response = await client.get(args["url"])

      return {
          "content": [
              {
                  "type": "image",
                  "data": base64.b64encode(response.content).decode(
                      "ascii"
                  ),  # Base64-encode the raw bytes
                  "mimeType": response.headers.get(
                      "content-type", "image/png"
                  ),  # Read MIME type from the response
              }
          ]
      }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_image",
    "Fetch an image from a URL and return it to Claude",
    {
      url: z.string().url()
    },
    async (args) => {
      const response = await fetch(args.url); // Fetch the image bytes
      const buffer = Buffer.from(await response.arrayBuffer()); // Read into a Buffer for base64 encoding
      const mimeType = response.headers.get("content-type") ?? "image/png";

      return {
        content: [
          {
            type: "image",
            data: buffer.toString("base64"), // Base64-encode the raw bytes
            mimeType
          }
        ]
      };
    }
  );
  ```
</CodeGroup>

<h3 id="resources">
  Ресурсы
</h3>

Блок ресурса встраивает фрагмент содержимого, идентифицируемый URI. URI — это метка для ссылки Claude; фактическое содержимое находится в поле `text` или `blob` блока. Используйте это, когда ваш инструмент создаёт что-то, что имеет смысл адресовать по имени позже, например сгенерированный файл или запись из внешней системы.

| Поле                | Тип          | Примечания                                                                                                                                                              |
| :------------------ | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                                                                                                         |
| `resource.uri`      | `string`     | Идентификатор содержимого. Любая схема URI                                                                                                                              |
| `resource.text`     | `string`     | Содержимое, если это текст. Укажите это или `blob`, но не оба                                                                                                           |
| `resource.blob`     | `string`     | Содержимое в кодировке base64, если это двоичные данные. Только TypeScript: Python SDK удаляет двоичные ресурсы из результата инструмента и регистрирует предупреждение |
| `resource.mimeType` | `string`     | Необязательно                                                                                                                                                           |

Этот пример показывает блок ресурса, возвращаемый из обработчика инструмента. URI `file:///tmp/report.md` — это метка, на которую Claude может ссылаться позже; SDK не читает из этого пути.

<CodeGroup>
  ```typescript TypeScript theme={null}
  return {
    content: [
      {
        type: "resource",
        resource: {
          uri: "file:///tmp/report.md", // Label for Claude to reference, not a path the SDK reads
          mimeType: "text/markdown",
          text: "# Report\n..." // The actual content, inline
        }
      }
    ]
  };
  ```

  ```python Python theme={null}
  return {
      "content": [
          {
              "type": "resource",
              "resource": {
                  "uri": "file:///tmp/report.md",  # Label for Claude to reference, not a path the SDK reads
                  "mimeType": "text/markdown",
                  "text": "# Report\n...",  # The actual content, inline
              },
          }
      ]
  }
  ```
</CodeGroup>

Эти формы блоков происходят из типа MCP `CallToolResult`. Полное определение см. в [спецификации MCP](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result).

<h2 id="return-structured-data">
  Возврат структурированных данных
</h2>

`structuredContent` — это необязательный объект JSON в результате, отдельный от массива `content`. Используйте его для возврата необработанных значений, которые Claude может читать как точные поля вместо их анализа из текстовой строки или изображения.

Когда `structuredContent` установлен, Claude получает JSON плюс любые блоки изображений или ресурсов из `content`. Текстовые блоки в `content` не передаются, так как предполагается, что они дублируют структурированные данные. Пример ниже отображает диаграмму как блок изображения и возвращает точки данных за ней в `structuredContent` из того же обработчика. В фрагменте `chartPngBuffer` — это `Buffer`, содержащий отрендеренные байты PNG.

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "image",
      data: chartPngBuffer.toString("base64"),
      mimeType: "image/png"
    }
  ],
  structuredContent: {
    series: "temperature_2m",
    unit: "fahrenheit",
    points: [62.1, 63.4, 65.0, 64.2]
  }
};
```

<Note>
  Декоратор Python `@tool` передает только `content` и `is_error` из словаря возврата обработчика. Чтобы вернуть `structuredContent` из Python, запустите [автономный MCP сервер](/docs/ru/agent-sdk/mcp) вместо встроенного SDK сервера.
</Note>

<h2 id="example-unit-converter">
  Пример: конвертер единиц
</h2>

Этот инструмент преобразует значения между единицами длины, температуры и веса. Пользователь может попросить "конвертировать 100 километров в мили" или "сколько это 72°F в Цельсиях", и Claude выбирает правильный тип единиц и единицы из запроса.

Это демонстрирует два паттерна:

* **Enum schemas:** `unit_type` ограничен фиксированным набором значений. В TypeScript используйте `z.enum()`. В Python словарь schema не поддерживает enums, поэтому требуется полный JSON Schema словарь.
* **Обработка неподдерживаемого ввода:** когда пара преобразования не найдена, обработчик возвращает `isError: true`, чтобы Claude мог сообщить пользователю, что пошло не так, вместо того чтобы рассматривать сбой как нормальный результат.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # z.enum() в TypeScript становится ограничением "enum" в JSON Schema.
  # Словарь schema не имеет эквивалента, поэтому требуется полный JSON Schema.
  @tool(
      "convert_units",
      "Convert a value from one unit to another",
      {
          "type": "object",
          "properties": {
              "unit_type": {
                  "type": "string",
                  "enum": ["length", "temperature", "weight"],
                  "description": "Category of unit",
              },
              "from_unit": {
                  "type": "string",
                  "description": "Unit to convert from, e.g. kilometers, fahrenheit, pounds",
              },
              "to_unit": {"type": "string", "description": "Unit to convert to"},
              "value": {"type": "number", "description": "Value to convert"},
          },
          "required": ["unit_type", "from_unit", "to_unit", "value"],
      },
  )
  async def convert_units(args: dict[str, Any]) -> dict[str, Any]:
      conversions = {
          "length": {
              "kilometers_to_miles": lambda v: v * 0.621371,
              "miles_to_kilometers": lambda v: v * 1.60934,
              "meters_to_feet": lambda v: v * 3.28084,
              "feet_to_meters": lambda v: v * 0.3048,
          },
          "temperature": {
              "celsius_to_fahrenheit": lambda v: (v * 9) / 5 + 32,
              "fahrenheit_to_celsius": lambda v: (v - 32) * 5 / 9,
              "celsius_to_kelvin": lambda v: v + 273.15,
              "kelvin_to_celsius": lambda v: v - 273.15,
          },
          "weight": {
              "kilograms_to_pounds": lambda v: v * 2.20462,
              "pounds_to_kilograms": lambda v: v * 0.453592,
              "grams_to_ounces": lambda v: v * 0.035274,
              "ounces_to_grams": lambda v: v * 28.3495,
          },
      }

      key = f"{args['from_unit']}_to_{args['to_unit']}"
      fn = conversions.get(args["unit_type"], {}).get(key)

      if not fn:
          return {
              "content": [
                  {
                      "type": "text",
                      "text": f"Unsupported conversion: {args['from_unit']} to {args['to_unit']}",
                  }
              ],
              "is_error": True,
          }

      result = fn(args["value"])
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"{args['value']} {args['from_unit']} = {result:.4f} {args['to_unit']}",
              }
          ]
      }


  converter_server = create_sdk_mcp_server(
      name="converter",
      version="1.0.0",
      tools=[convert_units],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  const convert = tool(
    "convert_units",
    "Convert a value from one unit to another",
    {
      unit_type: z.enum(["length", "temperature", "weight"]).describe("Category of unit"),
      from_unit: z
        .string()
        .describe("Unit to convert from, e.g. kilometers, fahrenheit, pounds"),
      to_unit: z.string().describe("Unit to convert to"),
      value: z.number().describe("Value to convert")
    },
    async (args) => {
      type Conversions = Record<string, Record<string, (v: number) => number>>;

      const conversions: Conversions = {
        length: {
          kilometers_to_miles: (v) => v * 0.621371,
          miles_to_kilometers: (v) => v * 1.60934,
          meters_to_feet: (v) => v * 3.28084,
          feet_to_meters: (v) => v * 0.3048
        },
        temperature: {
          celsius_to_fahrenheit: (v) => (v * 9) / 5 + 32,
          fahrenheit_to_celsius: (v) => ((v - 32) * 5) / 9,
          celsius_to_kelvin: (v) => v + 273.15,
          kelvin_to_celsius: (v) => v - 273.15
        },
        weight: {
          kilograms_to_pounds: (v) => v * 2.20462,
          pounds_to_kilograms: (v) => v * 0.453592,
          grams_to_ounces: (v) => v * 0.035274,
          ounces_to_grams: (v) => v * 28.3495
        }
      };

      const key = `${args.from_unit}_to_${args.to_unit}`;
      const fn = conversions[args.unit_type]?.[key];

      if (!fn) {
        return {
          content: [
            {
              type: "text",
              text: `Unsupported conversion: ${args.from_unit} to ${args.to_unit}`
            }
          ],
          isError: true
        };
      }

      const result = fn(args.value);
      return {
        content: [
          {
            type: "text",
            text: `${args.value} ${args.from_unit} = ${result.toFixed(4)} ${args.to_unit}`
          }
        ]
      };
    }
  );

  const converterServer = createSdkMcpServer({
    name: "converter",
    version: "1.0.0",
    tools: [convert]
  });
  ```
</CodeGroup>

После определения сервера передайте его в `query` так же, как в примере с погодой. Этот пример отправляет три разных запроса в цикле, чтобы показать, как один и тот же инструмент обрабатывает разные типы единиц. Для каждого ответа он проверяет объекты `AssistantMessage` (которые содержат вызовы инструментов, которые Claude сделал во время этого хода) и выводит каждый `ToolUseBlock` перед выводом финального текста `ResultMessage`. Это позволяет вам увидеть, когда Claude использует инструмент, а когда отвечает из собственных знаний.

Поскольку [tool search](/docs/ru/agent-sdk/tool-search) включен по умолчанию, вывод также может включать вызов `ToolSearch`, когда Claude загружает отложенную schema инструмента.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      AssistantMessage,
      ToolUseBlock,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"converter": converter_server},
          allowed_tools=["mcp__converter__convert_units"],
      )

      prompts = [
          "Convert 100 kilometers to miles.",
          "What is 72°F in Celsius?",
          "How many pounds is 5 kilograms?",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt, options=options):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              print(f"[tool call] {block.name}({block.input})")
                  elif isinstance(message, ResultMessage) and message.subtype == "success":
                      print(f"Q: {prompt}\nA: {message.result}\n")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. Only success
              # results are printed above, so handle the failure here and continue with
              # the next prompt.
              print(f"Call failed: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompts = [
    "Convert 100 kilometers to miles.",
    "What is 72°F in Celsius?",
    "How many pounds is 5 kilograms?"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({
        prompt,
        options: {
          mcpServers: { converter: converterServer },
          allowedTools: ["mcp__converter__convert_units"]
        }
      })) {
        if (message.type === "assistant") {
          for (const block of message.message.content) {
            if (block.type === "tool_use") {
              console.log(`[tool call] ${block.name}`, block.input);
            }
          }
        } else if (message.type === "result" && message.subtype === "success") {
          console.log(`Q: ${prompt}\nA: ${message.result}\n`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. Only success
      // results are logged above, so handle the failure here and continue with
      // the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Следующие шаги
</h2>

Вы можете комбинировать паттерны на этой странице на одном сервере: один сервер может содержать инструмент базы данных, инструмент шлюза API и средство визуализации изображений рядом друг с другом.

Отсюда:

* Если ваш сервер вырастет до десятков инструментов, см. [поиск инструментов](/docs/ru/agent-sdk/tool-search) для отложенной загрузки их до момента, когда Claude их потребует.
* Для подключения к внешним MCP серверам (файловая система, GitHub, Slack) вместо создания собственных, см. [Подключение MCP серверов](/docs/ru/agent-sdk/mcp).
* Для управления тем, какие инструменты запускаются автоматически в сравнении с требующими одобрения, см. [Настройка разрешений](/docs/ru/agent-sdk/permissions).
