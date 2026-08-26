> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Предоставьте Claude пользовательские инструменты

> Определите пользовательские инструменты с помощью встроенного MCP-сервера Agent SDK, чтобы Claude мог вызывать ваши функции, обращаться к вашим API и выполнять операции, специфичные для вашей области.

Пользовательские инструменты расширяют Agent SDK, позволяя вам определять собственные функции, которые Claude может вызывать во время разговора. Используя встроенный MCP-сервер SDK, вы можете предоставить Claude доступ к базам данных, внешним API, логике, специфичной для вашей области, или любым другим возможностям, которые требует ваше приложение.

В этом руководстве рассказывается, как определять инструменты с входными схемами и обработчиками, объединять их в MCP-сервер, передавать их в `query` и контролировать, к каким инструментам Claude может получить доступ. Оно также охватывает обработку ошибок, аннотации инструментов и возврат нетекстового содержимого, такого как изображения.

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

* **Имя:** уникальный идентификатор, который Claude использует для вызова инструмента.
* **Описание:** что делает инструмент. Claude читает это, чтобы решить, когда его вызывать.
* **Входная схема:** аргументы, которые должен предоставить Claude. В TypeScript это всегда [схема Zod](https://zod.dev/), и типы `args` обработчика автоматически выводятся из неё. В Python это словарь, отображающий имена на типы, например `{"latitude": float}`, который SDK преобразует в JSON Schema для вас. Декоратор Python также принимает полный словарь [JSON Schema](https://json-schema.org/understanding-json-schema/about) непосредственно, когда вам нужны перечисления, диапазоны, необязательные поля или вложенные объекты.
* **Обработчик:** асинхронная функция, которая запускается, когда Claude вызывает инструмент. Она получает проверенные аргументы и должна вернуть объект с:
  * `content` (обязательно): массив блоков результатов, каждый с типом `"text"`, `"image"`, `"audio"`, `"resource"` или `"resource_link"`. См. [Возврат изображений и ресурсов](#return-images-and-resources) для нетекстовых блоков.
  * `structuredContent` (необязательно): объект JSON, содержащий результат как машиночитаемые данные, возвращаемые вместе с `content`. См. [Возврат структурированных данных](#return-structured-data).
  * `isError` (необязательно): установите на `true`, чтобы сигнализировать об ошибке инструмента, чтобы Claude мог на неё реагировать. См. [Обработка ошибок](#handle-errors).

После определения инструмента оберните его в сервер с помощью [`createSdkMcpServer`](/docs/ru/agent-sdk/typescript#createsdkmcpserver) (TypeScript) или [`create_sdk_mcp_server`](/docs/ru/agent-sdk/python#create_sdk_mcp_server) (Python). Сервер работает встроенным образом внутри вашего приложения, а не как отдельный процесс.

<h3 id="weather-tool-example">
  Пример инструмента погоды
</h3>

Этот пример определяет инструмент `get_temperature` и оборачивает его в MCP-сервер. Он только настраивает инструмент; чтобы передать его в `query` и запустить его, см. [Вызов пользовательского инструмента](#call-a-custom-tool) ниже.

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

См. справку [`tool()`](/docs/ru/agent-sdk/typescript#tool) TypeScript или справку [`@tool`](/docs/ru/agent-sdk/python#tool) Python для полных деталей параметров, включая форматы входных JSON Schema и структуру возвращаемого значения.

<Tip>
  Чтобы сделать параметр необязательным: в TypeScript добавьте `.default()` к полю Zod. В Python словарь схемы рассматривает каждый ключ как обязательный, поэтому оставьте параметр вне схемы, упомяните его в строке описания и читайте его с помощью `args.get()` в обработчике. Инструмент [`get_precipitation_chance` ниже](#add-more-tools) показывает оба паттерна.
</Tip>

<h3 id="call-a-custom-tool">
  Вызов пользовательского инструмента
</h3>

Передайте созданный MCP-сервер в `query` через опцию `mcpServers`. Ключ в `mcpServers` становится сегментом `{server_name}` в полностью квалифицированном имени каждого инструмента: `mcp__{server_name}__{tool_name}`. Перечислите это имя в `allowedTools`, чтобы инструмент работал без запроса разрешения.

Эти фрагменты повторно используют `weatherServer` из [примера выше](#weather-tool-example), чтобы спросить Claude о погоде в определённом месте.

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

<h3 id="add-more-tools">
  Добавление дополнительных инструментов
</h3>

Сервер содержит столько инструментов, сколько вы перечислите в его массиве `tools`. Если на сервере более одного инструмента, вы можете перечислить каждый в `allowedTools` отдельно или использовать подстановочный знак `mcp__weather__*`, чтобы охватить каждый инструмент, который сервер предоставляет.

Пример ниже добавляет второй инструмент, `get_precipitation_chance`, к `weatherServer` из [примера инструмента погоды](#weather-tool-example) и перестраивает его с обоими инструментами в массиве.

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

Каждый инструмент в этом массиве потребляет пространство контекстного окна на каждом ходу. Если вы определяете десятки инструментов, см. [поиск инструментов](/docs/ru/agent-sdk/tool-search) для загрузки их по требованию вместо этого.

<h3 id="add-tool-annotations">
  Добавление аннотаций инструментов
</h3>

[Аннотации инструментов](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations) — это необязательные метаданные, описывающие поведение инструмента. Передайте их в качестве пятого аргумента вспомогательной функции `tool()` в TypeScript или через аргумент ключевого слова `annotations` для декоратора `@tool` в Python. Все поля подсказок являются логическими значениями.

| Поле              | По умолчанию | Значение                                                                                                                               |
| :---------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `readOnlyHint`    | `false`      | Инструмент не изменяет свою среду. Контролирует, может ли инструмент вызываться параллельно с другими инструментами только для чтения. |
| `destructiveHint` | `true`       | Инструмент может выполнять деструктивные обновления. Только информационное.                                                            |
| `idempotentHint`  | `false`      | Повторные вызовы с одинаковыми аргументами не имеют дополнительного эффекта. Только информационное.                                    |
| `openWorldHint`   | `true`       | Инструмент обращается к системам вне вашего процесса. Только информационное.                                                           |

Аннотации — это метаданные, а не принуждение. Инструмент, отмеченный как `readOnlyHint: true`, всё ещё может писать на диск, если это то, что делает обработчик. Держите аннотацию точной для обработчика.

Этот пример добавляет `readOnlyHint` к инструменту `get_temperature` из [примера инструмента погоды](#weather-tool-example).

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
  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

См. `ToolAnnotations` в справке [TypeScript](/docs/ru/agent-sdk/typescript#toolannotations) или [Python](/docs/ru/agent-sdk/python#toolannotations).

<h2 id="control-tool-access">
  Контроль доступа к инструментам
</h2>

[Пример инструмента погоды](#weather-tool-example) зарегистрировал сервер и перечислил инструменты в `allowedTools`. Этот раздел охватывает, как конструируются имена инструментов и как ограничить доступ, когда у вас есть несколько инструментов или вы хотите ограничить встроенные инструменты.

<h3 id="tool-name-format">
  Формат имени инструмента
</h3>

Когда инструменты MCP предоставляются Claude, их имена следуют определённому формату:

* Паттерн: `mcp__{server_name}__{tool_name}`
* Пример: инструмент с именем `get_temperature` на сервере `weather` становится `mcp__weather__get_temperature`

<h3 id="configure-allowed-tools">
  Настройка разрешённых инструментов
</h3>

Опция `tools` и списки разрешённых/запрещённых инструментов влияют на два уровня: доступность, которая контролирует, появляется ли инструмент в контексте Claude, и разрешение, которое контролирует, одобрен ли вызов после того, как Claude попытается его выполнить. `tools` и записи `disallowedTools` без области действия изменяют доступность. `allowedTools` и правила `disallowedTools` с областью действия изменяют только разрешение.

| Опция                     | Уровень     | Эффект                                                                                                                                                                                                                                                              |
| :------------------------ | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `tools: ["Read", "Grep"]` | Доступность | Только перечисленные встроенные инструменты находятся в контексте Claude. Неперечисленные встроенные инструменты удаляются. Инструменты MCP не затрагиваются.                                                                                                       |
| `tools: []`               | Доступность | Все встроенные инструменты удаляются. Claude может использовать только ваши инструменты MCP.                                                                                                                                                                        |
| разрешённые инструменты   | Разрешение  | Перечисленные инструменты работают без запроса разрешения. Неперечисленные инструменты остаются доступными; вызовы проходят через [поток разрешений](/docs/ru/agent-sdk/permissions).                                                                                    |
| запрещённые инструменты   | Оба         | Имя инструмента без области действия, такое как `"Bash"`, удаляет инструмент из контекста Claude, как если бы его не было в `tools`. Правило с областью действия, такое как `"Bash(rm *)"`, оставляет инструмент в контексте и отклоняет только совпадающие вызовы. |

Чтобы полностью удалить встроенный инструмент, пропустите его из `tools` или перечислите его имя без области действия в `disallowedTools` (Python: `disallowed_tools`); оба способа держат инструмент вне контекста, чтобы Claude никогда не попытался его использовать. Правило `disallowedTools` с областью действия блокирует совпадающие вызовы, но оставляет инструмент видимым, поэтому Claude может потратить ход, пытаясь его использовать. См. [Настройка разрешений](/docs/ru/agent-sdk/permissions) для полного порядка оценки.

<h2 id="handle-errors">
  Обработка ошибок
</h2>

Ошибка обработчика не останавливает цикл агента. Встроенный MCP-сервер SDK перехватывает необработанные исключения и возвращает их как результаты ошибок, поэтому то, как вы сообщаете об ошибке, определяет, что видит Claude, а не то, завершится ли запрос ошибкой:

| Что происходит                                                                                  | Результат                                                                                                                                                                              |
| :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Обработчик выбрасывает необработанное исключение                                                | MCP-сервер преобразует его в результат ошибки с исходным сообщением об исключении. Claude видит это сообщение, и цикл агента продолжается.                                             |
| Обработчик перехватывает ошибку и возвращает `isError: true` (TS) / `"is_error": True` (Python) | Claude видит сообщение, которое вы составили. Вы можете добавить контекст, которого не хватает исходному исключению, например какой запрос не удался или что попробовать вместо этого. |

В обоих случаях Claude может повторить попытку, попробовать другой инструмент или объяснить сбой. Перехватывайте ошибки самостоятельно, когда исходное сообщение об исключении недостаточно для того, чтобы Claude мог действовать.

Пример ниже перехватывает два вида сбоев внутри обработчика и составляет сообщение об ошибке, которое видит Claude. Статус HTTP, отличный от 200, перехватывается из ответа и возвращается как результат ошибки. Ошибка сети или неверный JSON перехватываются окружающим `try/except` (Python) или `try/catch` (TypeScript) и также возвращаются как результат ошибки. В обоих случаях Claude получает сообщение, которое описывает сбой, вместо простой строки исключения.

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any


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

Массив `content` в результате инструмента принимает блоки `text`, `image`, `audio`, `resource` и `resource_link`. Вы можете смешивать их в одном ответе. В TypeScript блоки audio сохраняются на диск, и Claude получает текстовый блок с сохранённым путём файла; в Python SDK удаляет блоки audio из результата инструмента и регистрирует предупреждение. Блоки resource link преобразуются в текстовый блок, содержащий имя ссылки, URI и описание.

<h3 id="images">
  Изображения
</h3>

Блок изображения содержит байты изображения встроенным образом, закодированные как base64. Нет поля URL. Чтобы вернуть изображение, которое находится по URL, получите его в обработчике, прочитайте байты ответа и закодируйте их в base64 перед возвратом. Результат обрабатывается как визуальный ввод.

| Поле       | Тип       | Примечания                                                                           |
| :--------- | :-------- | :----------------------------------------------------------------------------------- |
| `type`     | `"image"` |                                                                                      |
| `data`     | `string`  | Байты в кодировке Base64. Только сырой base64, без префикса `data:image/...;base64,` |
| `mimeType` | `string`  | Обязательно. Например `image/png`, `image/jpeg`, `image/webp`, `image/gif`           |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx


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

Блок ресурса встраивает часть содержимого, идентифицируемую по URI. URI — это метка для Claude, чтобы ссылаться на неё; фактическое содержимое находится в поле `text` или `blob` блока. Используйте это, когда ваш инструмент производит что-то, что имеет смысл адресовать по имени позже, например сгенерированный файл или запись из внешней системы.

| Поле                | Тип          | Примечания                                                                                                                                                       |
| :------------------ | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                                                                                                  |
| `resource.uri`      | `string`     | Идентификатор содержимого. Любая схема URI                                                                                                                       |
| `resource.text`     | `string`     | Содержимое, если это текст. Предоставьте это или `blob`, но не оба                                                                                               |
| `resource.blob`     | `string`     | Содержимое в кодировке base64, если это двоичное. Только TypeScript: Python SDK удаляет двоичные ресурсы из результата инструмента и регистрирует предупреждение |
| `resource.mimeType` | `string`     | Необязательно                                                                                                                                                    |

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

Эти формы блоков происходят из типа MCP `CallToolResult`. См. [спецификацию MCP](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result) для полного определения.

<h2 id="return-structured-data">
  Возврат структурированных данных
</h2>

`structuredContent` — это необязательный объект JSON на результате, отдельный от массива `content`. Используйте его для возврата сырых значений, которые Claude может читать как точные поля вместо их анализа из текстовой строки или изображения.

Когда установлен `structuredContent`, Claude получает JSON плюс любые блоки изображения или ресурса из `content`. Текстовые блоки в `content` не пересылаются, так как предполагается, что они дублируют структурированные данные. Пример ниже отображает диаграмму как блок изображения и возвращает точки данных позади неё в `structuredContent` из того же обработчика.

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
  Декоратор Python `@tool` пересылает только `content` и `is_error` из словаря возврата обработчика. Чтобы вернуть `structuredContent` из Python, запустите [автономный MCP-сервер](/docs/ru/agent-sdk/mcp) вместо встроенного сервера SDK.
</Note>

<h2 id="example-unit-converter">
  Пример: конвертер единиц
</h2>

Этот инструмент преобразует значения между единицами длины, температуры и веса. Пользователь может спросить "преобразовать 100 километров в мили" или "что такое 72°F в Цельсиях", и Claude выбирает правильный тип единицы и единицы из запроса.

Он демонстрирует два паттерна:

* **Схемы перечисления:** `unit_type` ограничен фиксированным набором значений. В TypeScript используйте `z.enum()`. В Python словарь схемы не поддерживает перечисления, поэтому требуется полный словарь JSON Schema.
* **Обработка неподдерживаемого ввода:** когда пара преобразования не найдена, обработчик возвращает `isError: true`, чтобы Claude мог сказать пользователю, что пошло не так, вместо того чтобы рассматривать сбой как нормальный результат.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # z.enum() in TypeScript becomes an "enum" constraint in JSON Schema.
  # The dict schema has no equivalent, so full JSON Schema is required.
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

После определения сервера передайте его в `query` так же, как в примере с погодой. Этот пример отправляет три разных запроса в цикле, чтобы показать, как один и тот же инструмент обрабатывает разные типы единиц. Для каждого ответа он проверяет объекты `AssistantMessage` (которые содержат вызовы инструментов, которые Claude сделал на этом ходу) и выводит каждый `ToolUseBlock` перед выводом финального текста `ResultMessage`. Это позволяет вам увидеть, когда Claude использует инструмент, а когда отвечает из своих собственных знаний.

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
          async for message in query(prompt=prompt, options=options):
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, ToolUseBlock):
                          print(f"[tool call] {block.name}({block.input})")
              elif isinstance(message, ResultMessage) and message.subtype == "success":
                  print(f"Q: {prompt}\nA: {message.result}\n")


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
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Следующие шаги
</h2>

Пользовательские инструменты оборачивают асинхронные функции в стандартный интерфейс. Вы можете смешивать паттерны на этой странице на одном сервере: один сервер может содержать инструмент базы данных, инструмент шлюза API и средство визуализации изображений рядом друг с другом.

Отсюда:

* Если ваш сервер растёт до десятков инструментов, см. [поиск инструментов](/docs/ru/agent-sdk/tool-search) для отложенной загрузки их до того, как Claude их потребует.
* Чтобы подключиться к внешним MCP-серверам (файловая система, GitHub, Slack) вместо создания собственного, см. [Подключение MCP-серверов](/docs/ru/agent-sdk/mcp).
* Чтобы контролировать, какие инструменты работают автоматически, а какие требуют одобрения, см. [Настройка разрешений](/docs/ru/agent-sdk/permissions).

<h2 id="related-documentation">
  Связанная документация
</h2>

* [Справка TypeScript SDK](/docs/ru/agent-sdk/typescript)
* [Справка Python SDK](/docs/ru/agent-sdk/python)
* [Документация MCP](https://modelcontextprotocol.io)
* [Обзор SDK](/docs/ru/agent-sdk/overview)
