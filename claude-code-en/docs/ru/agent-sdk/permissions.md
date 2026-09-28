> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Настройка разрешений

> Контролируйте, как ваш агент использует инструменты, с помощью режимов разрешений, hooks и декларативных правил разрешения/запрета.

Claude Agent SDK предоставляет элементы управления разрешениями для управления использованием инструментов Claude. Используйте режимы разрешений и правила для определения того, что разрешено автоматически, и обратный вызов [`canUseTool`](/docs/ru/agent-sdk/user-input) для обработки всего остального во время выполнения.

<h2 id="how-permissions-are-evaluated">
  Как оцениваются разрешения
</h2>

Когда Claude запрашивает инструмент, SDK проверяет разрешения в следующем порядке:

<Steps>
  <Step title="Hooks">
    Сначала запустите [hooks](/docs/ru/agent-sdk/hooks). Hook может отклонить вызов полностью или пропустить его дальше. Hook, который возвращает `allow`, не пропускает правила deny и ask ниже; они оцениваются независимо от результата hook. Hook `PreToolUse` с allow также не может одобрить удаление `rm` или `rmdir`, нацеленное на [критический путь](/docs/ru/permission-modes#critical-paths).
  </Step>

  <Step title="Deny rules">
    Проверьте правила `deny` (из `disallowed_tools` и [settings.json](/docs/ru/settings-reference#permission-settings)). Если правило deny совпадает, инструмент блокируется, даже в режиме `bypassPermissions`. Правила deny с простым названием, такие как `Bash`, удаляют инструмент из контекста Claude перед началом этой оценки, поэтому на этом шаге проверяются только правила с областью действия, такие как `Bash(rm *)`.
  </Step>

  <Step title="Ask rules">
    Проверьте правила `ask` из [settings.json](/docs/ru/settings-reference#permission-settings). Если правило ask совпадает, вызов передается в ваш callback [`canUseTool`](/docs/ru/agent-sdk/user-input) для подтверждения, даже в режиме `bypassPermissions`.

    Инструменты, требующие взаимодействия с пользователем, ведут себя так же: `AskUserQuestion` и MCP инструменты, сервер которых устанавливает [`_meta["anthropic/requiresUserInteraction"]`](/docs/ru/mcp#require-approval-for-a-specific-tool), всегда передаются в callback, даже когда совпадает правило allow. В режиме `dontAsk` оба случая отклоняются вместо этого, потому что этот режим никогда не запрашивает. Аннотация MCP требует Claude Code v2.1.199 или позже.

    Инструменты [claude.ai connector](/docs/ru/mcp#organization-controls-on-connector-tools), для которых ваша организация установила `ask`, также покидают поток на этом шаге. Каждый вызов передается в callback, даже в режиме `bypassPermissions` и даже когда совпадает правило allow. Callback получает причину `Your organization requires approval for this tool`. В режиме `dontAsk` вызов отклоняется вместо этого, потому что этот режим никогда не запрашивает.
  </Step>

  <Step title="Permission mode">
    Примените активный [режим разрешений](#permission-modes):

    * В режиме `bypassPermissions` Claude Code одобряет все, что достигает этого шага, кроме удалений `rm` и `rmdir`, нацеленных на [критический путь](/docs/ru/permission-modes#critical-paths), которые передаются вместо этого.
    * В режиме `acceptEdits` Claude Code одобряет операции с файлами, перечисленные в разделе [Accept edits mode](#accept-edits-mode-acceptedits).
    * В режиме `plan` Claude Code отправляет инструменты редактирования файлов и записи в shell в ваш callback `canUseTool` независимо от правил allow, поэтому операции записи не могут быть автоматически одобрены при планировании.
    * В других режимах запрос передается дальше.
  </Step>

  <Step title="Allow rules">
    Проверьте правила `allow` (из `allowed_tools` и settings.json). Если правило совпадает, инструмент одобряется. Вызов, который инструмент одобряет самостоятельно, также разрешается на этом шаге без необходимости в правиле: например, чтение файла в ваших рабочих каталогах или [команда Bash только для чтения](/docs/ru/permissions#read-only-commands). Удаления `rm` и `rmdir`, нацеленные на [критический путь](/docs/ru/permission-modes#critical-paths), никогда не одобряются правилом allow: они достигают вашего callback в режимах, которые запрашивают, переходят в [классификатор](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) в режиме `auto` на Claude Code v2.1.218 или позже, и отклоняются в режиме `dontAsk`.
  </Step>

  <Step title="canUseTool callback">
    Если ни один из вышеперечисленных шагов не разрешил вызов, вызовите ваш callback [`canUseTool`](/docs/ru/agent-sdk/user-input) для принятия решения. В режиме `dontAsk` этот шаг пропускается и инструмент отклоняется.

    В TypeScript SDK, если вы установите [`permissionPrompts: 'none'`](/docs/ru/agent-sdk/typescript#options), ваш callback не вызывается на этом шаге. Hook [`PermissionRequest`](/docs/ru/hooks#permissionrequest) все еще получает возможность принять решение, и если он этого не делает, Claude Code отклоняет вызов. Опция требует Claude Code v2.1.259 или позже.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="Диаграмма шестиэтапного потока оценки разрешений, соответствующая шагам выше: запрос инструмента проходит через hooks, правила deny, правила ask, режим разрешений, правила allow и canUseTool. Hooks, правила deny и canUseTool могут маршрутизировать вниз к Blocked; обход режима разрешений, правила allow и canUseTool могут маршрутизировать вверх к Execute; правила ask маршрутизируют к canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="Диаграмма шестиэтапного потока оценки разрешений, соответствующая шагам выше: запрос инструмента проходит через hooks, правила deny, правила ask, режим разрешений, правила allow и canUseTool. Hooks, правила deny и canUseTool могут маршрутизировать вниз к Blocked; обход режима разрешений, правила allow и canUseTool могут маршрутизировать вверх к Execute; правила ask маршрутизируют к canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

Если вы передаете callback `canUseTool` в конфигурацию, где TypeScript SDK ожидает, что порядок оценки автоматически одобрит вызовы перед консультацией callback, SDK выдает предупреждение процесса Node.js один раз при построении запроса. Код предупреждения — `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Две конфигурации вызывают его:

* `permissionMode: 'bypassPermissions'`, который автоматически одобряет каждый вызов, достигающий шага режима разрешений, кроме [действий, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves)
* Каждая запись `allowedTools` без спецификатора, такая как `"Read"`, которая автоматически одобряет весь этот инструмент перед консультацией callback, кроме [действий, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves)

Записи со спецификатором, такие как `Bash(ls *)`, и режим `acceptEdits` не вызывают его, и правила allow из файлов настроек не видны для проверки.

Слушайте с помощью `process.on('warning', ...)` и сопоставьте код для логирования или подавления его. Чтобы контролировать каждый вызов инструмента независимо от режима и правил, используйте вместо этого hook [`PreToolUse`](/docs/ru/agent-sdk/hooks).

Эта страница сосредоточена на **правилах allow и deny** и **режимах разрешений**. Для других шагов:

* **Hooks:** запустите пользовательский код для разрешения, отклонения или изменения запросов инструментов. См. [Control execution with hooks](/docs/ru/agent-sdk/hooks).
* **canUseTool callback:** запросите у пользователей одобрение во время выполнения, когда ни один более ранний шаг не разрешит вызов. См. [Handle approvals and user input](/docs/ru/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Правила разрешения и запрета
</h2>

`allowed_tools` и `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) добавляют записи в списки правил разрешения и запрета в потоке оценки выше. Если вы назовете один из [инструментов отслеживания задач](/docs/ru/agent-sdk/todo-tracking#model-availability) в `allowed_tools`, Claude Code также включает сеанс. Любой другой инструмент, не указанный в `allowed_tools`, по-прежнему доступен Claude, и вызов к нему, требующий одобрения, переходит в режим разрешений. Правила запрета ведут себя по-разному в зависимости от того, называют ли они инструмент или определяют шаблон в пределах одного.

| Опция                             | Эффект                                                                                                                                                                                                                                                  |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowed_tools=["Read", "Grep"]`  | `Read` и `Grep` автоматически одобрены. Другие инструменты, не указанные здесь, по-прежнему существуют, и вызовы к ним, требующие одобрения, переходят в режим разрешений и `canUseTool`.                                                               |
| `disallowed_tools=["Bash"]`       | Определение инструмента `Bash` удаляется из запроса. Claude не видит инструмент и не может попытаться его использовать.                                                                                                                                 |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` остается доступным. Вызовы, соответствующие `rm *` [как написано](/docs/ru/permissions#bash-rule-limits), отклоняются в каждом режиме разрешений, включая `bypassPermissions`. Другие вызовы `Bash`, включая `/bin/rm`, переходят в режим разрешений. |
| `disallowed_tools=["*"]`          | Каждое определение инструмента удаляется из запроса. Глобы имен инструментов поддерживаются в правилах запрета: `"*"` соответствует каждому инструменту и `"mcp__*"` соответствует каждому инструменту MCP на всех серверах.                            |

Правила разрешения принимают глобы имен инструментов только после буквального префикса `mcp__<server>__`. Сегмент сервера должен быть свободен от глобов, чтобы правило называло конкретный сервер, который вы настроили: `mcp__puppeteer__*` соответствует каждому инструменту с сервера `puppeteer`, а `mcp__github__get_*` соответствует его инструментам `get_`. Неякорированная запись, такая как `allowed_tools=["*"]` или `allowed_tools=["mcp__*"]`, игнорируется с предупреждением при запуске и не одобряет ничего автоматически.

Правила с областью действия для `Read` и `Edit` принимают шаблон пути. Правила `Edit(path)` управляют всеми встроенными инструментами, которые записывают файлы, включая `Write` и `NotebookEdit`; правило `Write(path)` никогда не совпадает с проверками разрешений файлов.

Используйте `//path` для абсолютного пути файловой системы: правило запрета `Edit(//secrets/**)` блокирует записи в любом месте под `/secrets` на диске. С одной ведущей косой чертой `Edit(/secrets/**)` якорируется в источнике правила. Для правил, переданных через `allowed_tools` или `disallowed_tools`, это означает рабочий каталог сеанса, поэтому правило не блокирует `/secrets` на диске. См. [Правила Read и Edit](/docs/ru/permissions#read-and-edit) для четырех форм якорей и того, как правила из файлов параметров разрешаются.

<Warning>
  **Автоматически одобренные инструменты никогда не достигают `canUseTool`.** Вызов инструмента, одобренный на любом более раннем этапе, по `acceptEdits` или `bypassPermissions`, или по правилу разрешения, пропускает ваш обратный вызов `canUseTool`, поэтому проверки разрешений, которые вы там поместили, молча обходятся для этого инструмента. `AskUserQuestion`, инструменты MCP, отмеченные [`_meta["anthropic/requiresUserInteraction"]`](/docs/ru/mcp#require-approval-for-a-specific-tool), инструменты соединителя [которые ваша организация установила на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools), и удаления `rm` и `rmdir`, нацеленные на [критический путь](/docs/ru/permission-modes#critical-paths), по-прежнему достигают обратного вызова, даже когда правило разрешения совпадает. В режиме `auto` удаления критических путей переходят к [классификатору](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) вместо обратного вызова, в то время как другие вызовы, перечисленные здесь, по-прежнему достигают его; маршрутизация классификатора требует Claude Code v2.1.218 или позже. В режиме `dontAsk` эти вызовы вместо этого отклоняются, без вызова обратного вызова.

  Охват зависит от формы записи: простое имя, такое как `Read` или `mcp__github__get_issue`, автоматически одобряет каждый вызов этого инструмента, кроме исключений выше, в то время как правило с областью действия, такое как `Bash(npm test *)`, автоматически одобряет только совпадающие вызовы, и другие вызовы `Bash`, требующие одобрения, по-прежнему переходят в обратный вызов. Для проверок, которые должны выполняться при каждом вызове инструмента, используйте [хук `PreToolUse`](/docs/ru/agent-sdk/hooks): хуки выполняются перед каждым другим шагом, и отказ хука применяется даже в режиме `bypassPermissions`.
</Warning>

Для заблокированного агента объедините `allowedTools` с `permissionMode: "dontAsk"`:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

Перечисленные инструменты одобрены, кроме [действий, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves), и каждый другой вызов, который будет запрашивать, вместо этого отклоняется. Вызовы, которые не требуют одобрения в режиме `default`, выполняются независимо от того, указаны ли они в списке, такие как [команды Bash только для чтения](/docs/ru/permissions#read-only-commands), инструменты, такие как `Agent`, которые не спрашивают перед запуском, и чтение файлов в ваших рабочих каталогах. Чтобы полностью исключить инструмент из досягаемости Claude, добавьте его простое имя в `disallowedTools`.

<Warning>
  **`allowed_tools` не ограничивает `bypassPermissions`.** `allowed_tools` предварительно одобряет инструменты, которые вы указали. Другие неуказанные инструменты не совпадают ни с одним правилом разрешения и переходят в режим разрешений, где `bypassPermissions` их одобряет. Установка `allowed_tools=["Read"]` наряду с `permission_mode="bypassPermissions"` по-прежнему одобряет каждый инструмент, включая `Bash`, `Write` и `Edit`. Если вам нужен `bypassPermissions`, но вы хотите заблокировать определенные инструменты, используйте `disallowed_tools`.
</Warning>

Вы также можете настроить правила разрешения, запрета и запроса декларативно в `.claude/settings.json`. Эти правила читаются, когда источник параметра `project` включен, что он есть для параметров `query()` по умолчанию. Если вы явно установите `setting_sources` (TypeScript: `settingSources`), включите `"project"`, чтобы они применялись. См. [Параметры разрешений](/docs/ru/settings-reference#permission-settings) для синтаксиса правил.

<h2 id="permission-modes">
  Режимы разрешений
</h2>

Режимы разрешений обеспечивают глобальный контроль над тем, как Claude использует инструменты. Вы можете установить режим разрешений при вызове `query()` или изменить его динамически во время сеансов потоковой передачи.

<h3 id="available-modes">
  Доступные режимы
</h3>

SDK поддерживает эти режимы разрешений:

| Режим               | Описание                                      | Поведение инструмента                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------ | :-------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Стандартное поведение разрешений              | Нет автоматических одобрений на основе режима; вызовы, требующие одобрения и не соответствующие никакому правилу разрешения, запускают ваш обратный вызов `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `dontAsk`           | Отклонить вместо запроса                      | Любой вызов, который иначе запросил бы подтверждение, отклоняется. Вызовы, одобренные `allowed_tools` или правилами, выполняются, как и вызовы, не требующие одобрения в режиме `default`, такие как чтение файлов внутри рабочих каталогов и вызовы `Agent`. Инструменты соединителя [установленные вашей организацией на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и инструменты, требующие взаимодействия с пользователем, отклоняются даже если вы их предварительно одобрили, как и удаления `rm` и `rmdir`, нацеленные на [критический путь](/docs/ru/permission-modes#critical-paths). `canUseTool` никогда не вызывается |
| `acceptEdits`       | Автоматически принимать редактирование файлов | Редактирование файлов и [операции с файловой системой](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv` и т. д.) автоматически одобряются                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `bypassPermissions` | Обойти проверки разрешений                    | Инструменты выполняются без запросов разрешений, за исключением [действий, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves). Используйте с осторожностью                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `plan`              | Режим планирования                            | Claude исследует и планирует без редактирования исходных файлов; редактирование файлов никогда не одобряется автоматически и запрашивается через ваш обратный вызов `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `auto`              | Одобрения, классифицированные моделью         | Классификатор модели одобряет или отклоняет запросы разрешений. См. [Режим Auto](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) для получения информации о доступности                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

<Warning>
  **Наследование подагентом:** Подагент выполняется в режиме разрешений родительского сеанса, если вы не установите `permissionMode` на его [`AgentDefinition`](/docs/ru/agent-sdk/typescript#agentdefinition) и родительский сеанс находится в режиме `default`, `dontAsk` или `plan`. Даже в этом случае Claude Code никогда не применяет значение `"bypassPermissions"`. Подагент выполняется в режиме `bypassPermissions` только когда сам родительский сеанс находится в этом режиме. Исключение `bypassPermissions` требует Claude Code v2.1.267 или более поздней версии.

  Подагенты могут иметь различные системные подсказки и менее ограниченное поведение, чем ваш основной агент, поэтому наследование `bypassPermissions` предоставляет им полный автономный доступ к системе. [Действия, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves), по-прежнему применяются.
</Warning>

<h3 id="set-permission-mode">
  Установка режима разрешений
</h3>

Вы можете установить режим разрешений один раз при запуске запроса или изменить его динамически во время активного сеанса.

<Tabs>
  <Tab title="При запросе">
    Передайте `permission_mode` (Python) или `permissionMode` (TypeScript) при создании запроса. Этот режим применяется для всего сеанса, если не изменяется динамически.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Help me refactor this code",
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Set the mode here
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        for await (const message of query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Set the mode here
          }
        })) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Во время потоковой передачи">
    Вызовите `set_permission_mode()` (Python) или `setPermissionMode()` (TypeScript) для изменения режима в середине сеанса. Новый режим вступает в силу немедленно для всех последующих запросов инструментов. Это позволяет вам начать с ограничительного режима и ослабить разрешения по мере развития доверия, например переключиться на `acceptEdits` после проверки первоначального подхода Claude.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions


      async def main():
          async with ClaudeSDKClient(
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Start in default mode
              )
          ) as client:
              await client.query("Help me refactor this code")

              # Change mode dynamically mid-session
              await client.set_permission_mode("acceptEdits")

              # Process messages with the new permission mode
              async for message in client.receive_response():
                  if hasattr(message, "result"):
                      print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        const q = query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Start in default mode
          }
        });

        // Change mode dynamically mid-session
        await q.setPermissionMode("acceptEdits");

        // Process messages with the new permission mode
        for await (const message of q) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>
</Tabs>

<h3 id="mode-details">
  Детали режимов
</h3>

<h4 id="accept-edits-mode-acceptedits">
  Режим принятия редактирования (`acceptEdits`)
</h4>

Автоматически одобряет операции с файлами, чтобы Claude мог редактировать код без запроса. Другие инструменты (такие как команды Bash, которые не являются операциями с файловой системой) по-прежнему требуют обычных разрешений.

**Автоматически одобренные операции:**

* Редактирование файлов (инструменты Edit, Write)
* Команды файловой системы: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Оба применяются только к путям внутри рабочего каталога или `additionalDirectories`. В режиме `acceptEdits` Claude Code не одобряет запрос автоматически, когда Claude:

* Работает с путем вне этой области
* Записывает в защищенный путь
* Удаляет [критический путь](/docs/ru/permission-modes#critical-paths) с помощью `rm` или `rmdir`

**Используйте когда:** вы доверяете редактированиям Claude и хотите более быструю итерацию, например во время прототипирования или при работе в изолированном каталоге.

<h4 id="don’t-ask-mode-dontask">
  Режим не спрашивать (`dontAsk`)
</h4>

Преобразует любой запрос разрешения в отклонение без вызова `canUseTool`. Инструменты, предварительно одобренные `allowed_tools`, правилами разрешения в `settings.json` или hook, выполняются нормально, как и вызовы, не требующие одобрения в режиме `default`, такие как чтение файлов внутри рабочих каталогов и вызовы `Agent`. Инструменты соединителя [установленные вашей организацией на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools), инструменты, требующие взаимодействия с пользователем, и удаления `rm` и `rmdir`, нацеленные на [критический путь](/docs/ru/permission-modes#critical-paths), отклоняются даже когда правило разрешения совпадает. Разрешение hook `PreToolUse` также не очищает удаление критического пути.

**Используйте когда:** вы хотите фиксированную, явную поверхность инструментов для headless агента и предпочитаете жесткое отклонение молчаливому полаганию на отсутствие `canUseTool`.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Режим обхода разрешений (`bypassPermissions`)
</h4>

Автоматически одобряет использование инструментов без запроса, за исключением случаев, перечисленных в предупреждении ниже. Hooks по-прежнему выполняются и могут блокировать операции при необходимости. На Linux и macOS Claude Code отказывается запускаться в этом режиме от имени root или под `sudo` вне [признанной песочницы](/docs/ru/permission-modes#skip-all-checks-with-bypasspermissions-mode), и запрос завершается ошибкой перед первым ходом.

<Warning>
  Используйте с крайней осторожностью. Claude имеет полный доступ к системе в этом режиме. Используйте только в контролируемых средах, где вы доверяете всем возможным операциям.

  `allowed_tools` не ограничивает этот режим. Каждый инструмент одобрен, а не только те, которые вы указали. Эти элементы управления по-прежнему применяются:

  * Правила отклонения, явные правила `ask` и hooks оцениваются перед проверкой режима и могут по-прежнему блокировать инструмент.
  * Инструменты соединителя [установленные вашей организацией на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools), инструменты, требующие взаимодействия с пользователем, и удаления `rm` и `rmdir`, нацеленные на [критический путь](/docs/ru/permission-modes#critical-paths), по-прежнему переходят к вашему обратному вызову `canUseTool`.
  * [Защита обмена сообщениями между сеансами](/docs/ru/permission-modes#skip-all-checks-with-bypasspermissions-mode) по-прежнему применяется.
</Warning>

<h4 id="plan-mode-plan">
  Режим планирования (`plan`)
</h4>

Claude исследует кодовую базу и создает план без редактирования исходных файлов. Инструменты только для чтения выполняются так же, как в режиме разрешений `default`.

Редактирование файлов никогда не одобряется автоматически в режиме планирования, даже когда правило разрешения совпадает. Вместо этого они запрашиваются через ваш обратный вызов `canUseTool`. В Claude Code v2.1.212 или более поздней версии команды оболочки, которые изменяют файлы, такие как `touch` и `rm`, достигают вашего обратного вызова `canUseTool` таким же образом.

Если вы установите `allowDangerouslySkipPermissions: true` вместе с `permissionMode: 'plan'`, редактирование файлов и команды оболочки, которые изменяют файлы, по-прежнему достигают вашего обратного вызова `canUseTool`. Опция позволяет вам позже переключиться на `bypassPermissions` с помощью `setPermissionMode()`.

Claude может использовать `AskUserQuestion` для уточнения требований перед финализацией плана. См. [Обработка одобрений и ввода пользователя](/docs/ru/agent-sdk/user-input#handle-clarifying-questions) для обработки этих запросов.

**Используйте когда:** вы хотите, чтобы Claude предложил изменения без их выполнения, например при проверке кода или когда вам нужно одобрить изменения перед их внесением.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

Для других этапов потока оценки разрешений:

* [Обработка одобрений и ввода пользователя](/docs/ru/agent-sdk/user-input): интерактивные подсказки одобрения и уточняющие вопросы
* [Руководство hooks](/docs/ru/agent-sdk/hooks): запуск пользовательского кода в ключевых точках жизненного цикла агента
* [Правила разрешений](/docs/ru/settings-reference#permission-settings): декларативные правила разрешения/запрета в `settings.json`
