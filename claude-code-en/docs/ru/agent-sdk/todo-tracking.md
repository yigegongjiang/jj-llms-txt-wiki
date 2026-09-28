> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Отслеживание задач

> Отслеживайте задачи в сеансах Agent SDK и отображайте прогресс Claude в вашем приложении с помощью структурированных вызовов инструментов

Claude Code предоставляет [инструменты отслеживания задач](/docs/ru/tools-reference#task-tool-availability) по умолчанию только на моделях, перечисленных в разделе [Доступность моделей](#model-availability). Более новые модели отслеживают многошаговую работу без письменного списка задач, поэтому на них вам не нужно ничего из этой страницы, чтобы Claude работал с многошаговыми задачами.

В сеансе, который имеет инструменты отслеживания задач, Claude ведет письменный список задач, обновляя статус каждого элемента по мере работы. Вы видите каждое изменение в потоке сообщений как структурированный вызов инструмента. Подключите сеанс только когда ваше приложение читает эти вызовы инструментов, будь то для логирования активности задач или для отображения собственного индикатора прогресса.

<h2 id="model-availability">
  Доступность моделей
</h2>

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.
</Note>

На модели, которая не имеет инструментов по умолчанию, если вы не подключите сеанс, вы не увидите блоков `tool_use` для них в потоке сообщений. Agent SDK применяет эти значения по умолчанию через двоичный файл Claude Code, который он включает. Если вы указываете `pathToClaudeCodeExecutable` (TypeScript) или `cli_path` (Python) на вашу собственную установку Claude Code, вы получаете те инструменты, которые предоставляет эта установка, в соответствии с её собственными значениями по умолчанию. Чтобы увидеть точный набор в работающем сеансе, [проверьте, какие инструменты доступны](/docs/ru/tools-reference#check-which-tools-are-available). Чтобы подключить сеанс, выполните одно из следующих действий:

* Назовите один из инструментов в опции [`allowedTools`](/docs/ru/agent-sdk/permissions#allow-and-deny-rules) (TypeScript) или `allowed_tools` (Python)
* Перечислите инструменты в опции `tools`, которая ограничивает встроенные инструменты сеанса только теми, которые она называет. Включите нужные вам инструменты вместе с другими встроенными инструментами, которые вы используете
* Установите `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` в опции `env`, как это делают примеры на этой странице. В TypeScript `env` заменяет окружение подпроцесса, поэтому распределите `...process.env` для сохранения унаследованных переменных. В Python `env` объединяется с унаследованным окружением

<h2 id="todo-lifecycle">
  Жизненный цикл задач
</h2>

Claude перемещает каждую задачу через предсказуемый жизненный цикл:

1. **Созданы**: Claude добавляет задачу как `pending` когда выявляет задачу
2. **Активированы**: Claude устанавливает задачу в `in_progress` когда начинает работу
3. **Завершены**: Claude отмечает её завершённой когда задача успешно завершается
4. **Удалены**: Claude удаляет задачу, которая ей больше не нужна, установив `status: "deleted"` в вызове `TaskUpdate`

<h2 id="when-claude-creates-todos">
  Когда Claude создаёт задачи
</h2>

В [сеансе, который имеет инструменты отслеживания задач](#model-availability), Claude создаёт задачи для большинства многошаговой работы, такой как:

* **Сложные многошаговые задачи**, требующие трёх или более отдельных действий
* **Списки задач, предоставленные пользователем**, когда упоминаются несколько элементов
* **Более длительные операции**, которые выигрывают от отслеживания прогресса
* **Явные запросы**, когда пользователи просят организовать задачи

Claude может пропустить задачи для очень коротких или одношаговых запросов.

<h2 id="examples">
  Примеры
</h2>

Перед запуском этих примеров установите Claude Agent SDK, следуя [краткому руководству](/docs/ru/agent-sdk/quickstart). Каждый пример на этой странице использует одну и ту же настройку разрешений и поведение выхода:

* **Режим разрешений**: примеры подсказок просят Claude выполнить реальную работу над проектом, поэтому каждый пример устанавливает `permissionMode: "acceptEdits"` (TypeScript) или `permission_mode="acceptEdits"` (Python) для автоматического одобрения редактирования файлов, которое производит работа. Смотрите [Режимы разрешений](/docs/ru/agent-sdk/permissions#permission-modes) для альтернатив.
* **Лимит ходов**: каждый пример работает до завершения агентом и выдачи его финального сообщения результата. Если сеанс сначала достигает лимита ходов, то сообщение результата имеет подтип `error_max_turns`. Проверьте `subtype`, чтобы обнаружить это завершение.
* **Обработка ошибок**: эти примеры используют однократные вызовы `query()`. После выдачи результата `error_max_turns`, `query()` выбрасывает ошибку, которая включает `Reached maximum number of turns`. Каждый пример оборачивает свой цикл в блок try для чистого выхода при возникновении этого события. Смотрите [Обработка результата](/docs/ru/agent-sdk/agent-loop#handle-the-result) для подтипов результатов.

<Note>
  Системные сообщения задач, [`SDKTaskNotificationMessage`](/docs/ru/agent-sdk/typescript#sdktasknotificationmessage) (TypeScript) или [`TaskNotificationMessage`](/docs/ru/agent-sdk/python#tasknotificationmessage) (Python) среди них, сообщают о фоновых задачах, таких как фоновые команды и подагенты. В потоке сообщений вы видите активность задач как блоки `tool_use` в сообщениях помощника.
</Note>

<h3 id="monitor-todo-changes">
  Мониторинг изменений задач
</h3>

Следующий пример наблюдает за потоком помощника для блоков `tool_use` `TaskCreate` и `TaskUpdate` и выводит строку `+` с предметом каждой новой задачи и строку обновления с ID задачи каждого изменения статуса и новым статусом. Используйте эту форму когда вы хотите логирование активности задач вместо отображаемого дисплея. Строки `+` не включают назначенные ID, поэтому этот логирование не может сопоставить обновления обратно их созданиям. Чтобы сохранить это соответствие, захватите ID как это делает [Отображение прогресса в реальном времени](#display-progress-in-real-time).

Потоковый ввод `tool_use` — это необработанная форма, которую выдала модель. Claude Code исправляет некоторые близкие, но неправильные имена ключей перед выполнением, сопоставляя `id` или `task_id` с `taskId` и `active_form` с `activeForm`, но это исправление не отражается в потоке. Читайте поля ввода `TaskUpdate` защитно, как это делают оба примера на этой странице, а не предполагайте, что каноническое имя всегда присутствует.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
      // Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
      options: { maxTurns: 15, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
    })) {
      if (message.type !== "assistant") continue;
      for (const block of message.message.content) {
        if (block.type !== "tool_use") continue;
        if (block.name === "TaskCreate") {
          const input = block.input as { subject: string };
          console.log(`+ ${input.subject}`);
        } else if (block.name === "TaskUpdate") {
          const input = block.input as {
            taskId?: string;
            id?: string;
            task_id?: string;
            status?: string;
          };
          const taskId = input.taskId ?? input.id ?? input.task_id;
          if (taskId && input.status) console.log(`  ${taskId} -> ${input.status}`);
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock

  async def main():
      try:
          async for message in query(
              prompt="Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
              # Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
              options=ClaudeAgentOptions(max_turns=15, permission_mode="acceptEdits", env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"}),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input.get('subject', '')}")
                  elif block.name == "TaskUpdate" and block.input.get("status"):
                      task_id = (
                          block.input.get("taskId")
                          or block.input.get("id")
                          or block.input.get("task_id")
                      )
                      if task_id:
                          print(f"  {task_id} -> {block.input['status']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="display-progress-in-real-time">
  Отображение прогресса в реальном времени
</h3>

Следующий пример наблюдает за потоком помощника для блоков `tool_use` `TaskCreate` и `TaskUpdate` и ведёт карту задач, индексированную по ID задачи в классе `TaskTracker`, переотображая сводку прогресса при каждом изменении. Сводка подсчитывает завершённые и выполняемые задачи и показывает метку `activeForm` каждого активного элемента вместо его `subject`. Используйте эту форму когда ваше приложение ведёт дисплей прогресса вместо логирования каждого события.

Назначенный ID задачи отсутствует во вводе `TaskCreate`. Claude Code доставляет структурированный вывод каждого инструмента в сообщение пользователя, которое несёт его блок `tool_result`, в поле `tool_use_result`. Для `TaskCreate`, этот объект задокументирован для TypeScript как `TaskCreateOutput` в разделе [Типы вывода инструментов](/docs/ru/agent-sdk/typescript#tool-output-types), и в Python поле является простым dict той же формы. Трекер сопоставляет каждый блок `tool_result` с его вызовом `tool_use` по `tool_use_id` и читает `task.id` из `tool_use_result` сопоставленного сообщения. Claude может прочитать список обратно с помощью `TaskList` и полные детали одной задачи с помощью `TaskGet`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  type Task = { subject: string; activeForm?: string; status: string };

  class TaskTracker {
    private tasks = new Map<string, Task>();
    private pendingCreates = new Map<string, { subject: string; activeForm?: string }>();

    displayProgress() {
      if (this.tasks.size === 0) {
        console.log("\nProgress: no open tasks\n");
        return;
      }

      const items = [...this.tasks.values()];
      const completed = items.filter((t) => t.status === "completed").length;
      const inProgress = items.filter((t) => t.status === "in_progress").length;

      console.log(`\nProgress: ${completed}/${this.tasks.size} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      for (const [id, task] of this.tasks) {
        const icon =
          task.status === "completed" ? "✅" : task.status === "in_progress" ? "🔧" : "❌";
        const text = task.status === "in_progress" && task.activeForm ? task.activeForm : task.subject;
        console.log(`${id}. ${icon} ${text}`);
      }
    }

    handleToolUse(block: { id: string; name: string; input: unknown }) {
      if (block.name === "TaskCreate") {
        const input = block.input as { subject: string; activeForm?: string; active_form?: string };
        this.pendingCreates.set(block.id, {
          subject: input.subject,
          activeForm: input.activeForm ?? input.active_form,
        });
      } else if (block.name === "TaskUpdate") {
        const input = block.input as {
          taskId?: string;
          id?: string;
          task_id?: string;
          status?: string;
          activeForm?: string;
          active_form?: string;
        };
        const taskId = input.taskId ?? input.id ?? input.task_id;
        if (!taskId) return;
        if (input.status === "deleted") {
          this.tasks.delete(taskId);
          this.displayProgress();
          return;
        }
        const task = this.tasks.get(taskId);
        if (!task) return;
        if (input.status) task.status = input.status;
        const active = input.activeForm ?? input.active_form;
        if (active) task.activeForm = active;
        this.displayProgress();
      }
    }

    handleToolResult(block: { tool_use_id: string; is_error?: boolean }, result: unknown) {
      const create = this.pendingCreates.get(block.tool_use_id);
      if (!create) return;
      this.pendingCreates.delete(block.tool_use_id);
      if (block.is_error) return;
      // The result's user message carries the tool's structured output as
      // tool_use_result; for TaskCreate that's TaskCreateOutput,
      // { task: { id, subject } }.
      const out = result as { task?: { id: string } };
      if (!out?.task?.id) return;
      this.tasks.set(out.task.id, { ...create, status: "pending" });
      this.displayProgress();
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          options: { maxTurns: 20, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use") this.handleToolUse(block);
            }
          }
          if (message.type === "user" && Array.isArray(message.message.content)) {
            for (const block of message.message.content) {
              if (block.type === "tool_result") this.handleToolResult(block, message.tool_use_result);
            }
          }
        }
      } catch (error) {
        // A single-shot query() throws after yielding an error result,
        // such as when the maxTurns limit is hit.
        console.log(`Session ended with an error: ${error}`);
      }
    }
  }

  // Usage
  const tracker = new TaskTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      AssistantMessage,
      UserMessage,
      ToolUseBlock,
      ToolResultBlock,
  )


  class TaskTracker:
      def __init__(self):
          self.tasks: dict[str, dict] = {}
          self.pending_creates: dict[str, dict] = {}

      def display_progress(self):
          if not self.tasks:
              print("\nProgress: no open tasks\n")
              return

          completed = len([t for t in self.tasks.values() if t["status"] == "completed"])
          in_progress = len([t for t in self.tasks.values() if t["status"] == "in_progress"])

          print(f"\nProgress: {completed}/{len(self.tasks)} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for task_id, task in self.tasks.items():
              icon = (
                  "✅"
                  if task["status"] == "completed"
                  else "🔧"
                  if task["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  task["activeForm"]
                  if task["status"] == "in_progress" and task.get("activeForm")
                  else task["subject"]
              )
              print(f"{task_id}. {icon} {text}")

      def handle_tool_use(self, block: ToolUseBlock):
          if block.name == "TaskCreate":
              self.pending_creates[block.id] = {
                  "subject": block.input.get("subject", ""),
                  "activeForm": block.input.get("activeForm") or block.input.get("active_form"),
              }
          elif block.name == "TaskUpdate":
              task_id = (
                  block.input.get("taskId")
                  or block.input.get("id")
                  or block.input.get("task_id")
              )
              if not task_id:
                  return
              if block.input.get("status") == "deleted":
                  self.tasks.pop(task_id, None)
                  self.display_progress()
                  return
              task = self.tasks.get(task_id)
              if not task:
                  return
              if block.input.get("status"):
                  task["status"] = block.input["status"]
              active = block.input.get("activeForm") or block.input.get("active_form")
              if active:
                  task["activeForm"] = active
              self.display_progress()

      def handle_tool_result(self, block: ToolResultBlock, tool_use_result):
          create = self.pending_creates.pop(block.tool_use_id, None)
          if create is None or block.is_error:
              return
          # The result's user message carries the tool's structured output as
          # tool_use_result; for TaskCreate that's {"task": {"id": ..., "subject": ...}}.
          task = (tool_use_result or {}).get("task") or {}
          if not task.get("id"):
              return
          self.tasks[task["id"]] = {**create, "status": "pending"}
          self.display_progress()

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  options=ClaudeAgentOptions(
                      max_turns=20,
                      permission_mode="acceptEdits",
                      env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"},
                  ),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              self.handle_tool_use(block)
                  if isinstance(message, UserMessage) and isinstance(message.content, list):
                      for block in message.content:
                          if isinstance(block, ToolResultBlock):
                              self.handle_tool_result(block, message.tool_use_result)
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TaskTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="related-documentation">
  Связанная документация
</h2>

* [Справочник Agent SDK - TypeScript](/docs/ru/agent-sdk/typescript): опции, типы и схемы инструментов для TypeScript SDK, включая типы ввода и вывода инструмента Task
* [Справочник Agent SDK - Python](/docs/ru/agent-sdk/python): опции, типы и документация инструментов для Python SDK
* [Потоковый ввод](/docs/ru/agent-sdk/streaming-vs-single-mode): два режима ввода и когда использовать потоковый ввод вместо однократных вызовов, которые используют эти примеры
* [Предоставьте Claude пользовательские инструменты](/docs/ru/agent-sdk/custom-tools): определите свои собственные инструменты с помощью встроенного MCP сервера SDK
