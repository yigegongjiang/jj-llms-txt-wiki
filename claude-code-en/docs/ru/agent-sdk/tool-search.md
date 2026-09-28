> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Масштабирование на множество инструментов с помощью поиска инструментов

> Масштабируйте вашего агента на тысячи инструментов, обнаруживая и загружая только необходимое по требованию.

Tool search позволяет вашему агенту работать с сотнями или тысячами инструментов, динамически обнаруживая и загружая их по требованию. Вместо загрузки всех определений инструментов в окно контекста заранее, агент ищет в каталоге инструментов и загружает только необходимые ему инструменты.

Этот подход решает две проблемы при масштабировании библиотек инструментов:

* **Эффективность контекста:** Определения инструментов могут занимать большую часть окна контекста (50 инструментов могут использовать 10-20K токенов), оставляя меньше места для фактической работы.
* **Точность выбора инструмента:** Точность выбора инструмента снижается при загрузке более 30-50 инструментов одновременно.

<h2 id="how-tool-search-works">
  How tool search works
</h2>

Tool search включен по умолчанию, за исключением случаев, перечисленных в разделе [Configure tool search](#configure-tool-search).

Когда tool search активен, определения инструментов исключаются из окна контекста. Агент получает сводку доступных инструментов и ищет релевантные, когда задача требует возможности, которая еще не загружена. До пяти наиболее релевантных инструментов загружаются в контекст по умолчанию, где они остаются доступными для последующих ходов до тех пор, пока SDK не компактирует сообщения, в которых агент их обнаружил. После этой компактификации агент снова ищет эти инструменты, когда они ему снова понадобятся.

Tool search добавляет один дополнительный обмен данными каждый раз, когда Claude ищет инструменты, но для больших наборов инструментов это компенсируется меньшим контекстом на каждом ходу. При наличии менее \~10 инструментов, определения которых удобно помещаются в окно контекста, загрузка всего заранее обычно быстрее.

Для получения подробной информации об основном механизме API см. [Tool search в API](https://platform.claude.com/docs/ru/agents-and-tools/tool-use/tool-search-tool).

<Note>
  Tool search не поддерживается на развертываниях Microsoft Foundry, [размещенных на Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), которые отклоняют его на стороне сервера: SDK обнаруживает отклонение и загружает определения инструментов заранее для этого развертывания вместо этого. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) не может переопределить это, так как отклонение исходит от самого развертывания.
</Note>

<h2 id="configure-tool-search">
  Настройка поиска инструментов
</h2>

Tool search включен по умолчанию. Для моделей из списка неподдерживаемых моделей SDK SDK загружает определения инструментов заранее, и никакое значение `ENABLE_TOOL_SEARCH` не переопределяет это. На Google Cloud's Agent Platform SDK решает по поколению модели:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 и более поздние версии**: tool search включен по умолчанию.
* **Более ранние модели Agent Platform**: SDK загружает определения инструментов заранее, потому что их стеки обслуживания отклоняют требуемый заголовок beta. `ENABLE_TOOL_SEARCH` не может переопределить это.

До Claude Code v2.1.221 SDK отключал tool search для всех моделей на Google Cloud's Agent Platform, если вы не установили `ENABLE_TOOL_SEARCH`.

SDK также отключает tool search, когда `ANTHROPIC_BASE_URL` указывает на хост, не принадлежащий первой стороне, поскольку большинство прокси не пересылают блоки `tool_reference`. Вы можете переопределить это значение по умолчанию с помощью переменной окружения `ENABLE_TOOL_SEARCH`:

| Значение         | Поведение                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (не установлено) | Tool search включен. Определения инструментов отложены и обнаруживаются по требованию. Возвращается к загрузке заранее на моделях Google Cloud's Agent Platform более ранних версий, чем поколение Claude 4.5, на `ANTHROPIC_BASE_URL`, не принадлежащем первой стороне, или на развертывании Microsoft Foundry, размещенном на Azure.                                                                                                                   |
| `true`           | Tool search всегда включен, кроме развертывания Microsoft Foundry, размещенного на Azure, где отклонение на стороне сервера все еще вынуждает загрузку заранее, и на моделях Google Cloud's Agent Platform более ранних версий, чем поколение Claude 4.5, где SDK продолжает загружать определения инструментов заранее. SDK отправляет заголовок beta через прокси, и запросы не выполняются на прокси, которые не поддерживают блоки `tool_reference`. |
| `auto`           | Подсчитывает токены в определениях инструментов, которые tool search может отложить, и сравнивает итог с окном контекста модели. Когда итог достигает 10% окна, tool search активируется. Ниже этого SDK загружает каждое определение инструмента в контекст заранее.                                                                                                                                                                                    |
| `auto:N`         | То же, что `auto`, но с пользовательским процентом. `auto:5` активируется, когда эти определения достигают 5% окна контекста. Более низкие значения активируются раньше.                                                                                                                                                                                                                                                                                 |
| `false`          | Tool search отключен. Все определения инструментов загружаются в контекст на каждом ходу.                                                                                                                                                                                                                                                                                                                                                                |

Установка [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ru/env-vars) сохраняет tool search отключенным. Вы не можете переопределить это, установив `ENABLE_TOOL_SEARCH` самостоятельно. Ваша организация может сохранить tool search включенным через [управляемые параметры](/docs/ru/managed-settings), на Claude Code v2.1.227 или более поздней версии. [Отключение предварительных возможностей](/docs/ru/llm-gateway-protocol#disable-pre-release-capabilities) охватывает, где применяется переопределение и что переменная удаляет.

Tool search применяется ко всем зарегистрированным инструментам, независимо от того, поступают ли они с удаленных MCP серверов или [пользовательских SDK MCP серверов](/docs/ru/agent-sdk/custom-tools). Когда вы используете `auto`, SDK подсчитывает каждое определение, которое tool search может отложить, в сторону одного объединенного порога: каждый инструмент MCP, который не отмечен как [`alwaysLoad`](/docs/ru/mcp#exempt-a-server-from-deferral), с любого сервера, плюс встроенные инструменты, которые загружаются по требованию. SDK всегда загружает основные встроенные инструменты, такие как Bash, Read и Edit, заранее и не подсчитывает их в сторону порога.

Установите значение в опции `env` на `query()`. В TypeScript `env` заменяет окружение подпроцесса, поэтому распространите `...process.env`, чтобы сохранить унаследованные переменные. В Python `env` объединяется с унаследованным окружением. Этот пример подключается к удаленному MCP серверу, который предоставляет множество инструментов, предварительно одобряет все их с помощью подстановочного символа и использует `auto:5`, чтобы tool search активировался, когда определения, которые он может отложить, достигают 5% окна контекста:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Find and run the appropriate database query",
      options: {
        mcpServers: {
          "enterprise-tools": {
            // Connect to a remote MCP server
            type: "http",
            url: "https://tools.example.com/mcp"
          }
        },
        allowedTools: ["mcp__enterprise-tools__*"], // Wildcard pre-approves all tools from this server
        env: {
          ...process.env, // env replaces the subprocess environment, so keep inherited variables
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when deferrable definitions reach 5% of context
        }
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "enterprise-tools": {
                  "type": "http",
                  "url": "https://tools.example.com/mcp",
              }
          },
          allowed_tools=[
              "mcp__enterprise-tools__*"
          ],  # Wildcard pre-approves all tools from this server
          env={
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when deferrable definitions reach 5% of context
          },
      )

      try:
          async for message in query(
              prompt="Find and run the appropriate database query",
              options=options,
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Чтобы запустить этот пример, замените `https://tools.example.com/mcp` на URL вашего собственного MCP сервера. При успехе текст результата выводится на консоль.

Поскольку это вызов `query()` в один ход, SDK вызывает исключение после выдачи результата ошибки, поэтому пример оборачивает цикл в блок try. Чтобы узнать, почему запуск не удался, проверьте `subtype` сообщения результата, например `error_during_execution`, внутри цикла. Дополнительную информацию о сообщениях результатов см. в разделе [Обработка результата](/docs/ru/agent-sdk/agent-loop#handle-the-result).

<h2 id="optimize-tool-discovery">
  Optimize tool discovery
</h2>

Механизм поиска сопоставляет запросы с именами и описаниями инструментов. Имена вроде `search_slack_messages` появляются для более широкого диапазона запросов, чем `query_slack`. Описания с конкретными ключевыми словами ("Search Slack messages by keyword, channel, or date range") соответствуют большему количеству запросов, чем общие ("Query Slack").

Вы также можете добавить раздел системного приглашения, в котором перечислены доступные категории инструментов. Это дает агенту контекст о том, какие виды инструментов доступны для поиска. Передайте текст через опцию `systemPrompt` в TypeScript или `system_prompt` в Python, используя предустановку `claude_code` с `append`, которая добавляет ваш текст к приглашению предустановки вместо его замены:

<CodeGroup>
  ```typescript TypeScript theme={null}
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: "You can search for tools to interact with Slack, GitHub, and Jira."
    }
  }
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      system_prompt={
          "type": "preset",
          "preset": "claude_code",
          "append": "You can search for tools to interact with Slack, GitHub, and Jira.",
      }
  )
  ```
</CodeGroup>

Для полного набора опций системного приглашения см. [Изменение системных приглашений](/docs/ru/agent-sdk/modifying-system-prompts).

<h2 id="limits">
  Limits
</h2>

* **Максимальное количество инструментов:** 10 000 инструментов в вашем каталоге
* **Результаты поиска:** возвращает до пяти наиболее релевантных инструментов на поиск по умолчанию
* **Поддержка модели:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 и более поздние модели; см. [совместимость моделей в документации API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) для получения актуального списка. То же самое минимальное требование применяется на платформе Agent Platform Google Cloud.

<h2 id="related-documentation">
  Связанная документация
</h2>

* [Tool search в API](https://platform.claude.com/docs/ru/agents-and-tools/tool-use/tool-search-tool): Полная документация API для tool search, включая пользовательские реализации
* [Подключение MCP серверов](/docs/ru/agent-sdk/mcp): Подключение к внешним инструментам через MCP серверы
* [Пользовательские инструменты](/docs/ru/agent-sdk/custom-tools): Создавайте свои собственные инструменты с помощью SDK MCP серверов
* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript): Полный справочник API
* [Справочник Python SDK](/docs/ru/agent-sdk/python): Полный справочник API
