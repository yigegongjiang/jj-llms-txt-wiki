> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Списки задач

> Отслеживайте и отображайте задачи с помощью Claude Agent SDK для организованного управления задачами

Отслеживание задач предоставляет структурированный способ управления задачами и отображения прогресса пользователям. Claude Agent SDK включает встроенную функциональность задач, которая помогает организовать сложные рабочие процессы и держать пользователей в курсе хода выполнения задач.

<Note>
  Начиная с TypeScript Agent SDK 0.3.142 и Claude Code v2.1.142, сеансы используют структурированные инструменты Task `TaskCreate`, `TaskUpdate`, `TaskGet` и `TaskList` вместо `TodoWrite`. Python SDK получает это изменение из Claude Code CLI, который он запускает, а не из версии пакета Python: переключение применяется после того, как этот CLI — копия, включенная в пакет pip, или та, на которую вы указываете с помощью `cli_path` — имеет версию v2.1.142 или позже. Смотрите [Миграция на инструменты Task](#migrate-to-task-tools) для информации о том, как отслеживать изменения кода. Примеры на этой странице устанавливают `CLAUDE_CODE_ENABLE_TASKS=0` для продолжения отображения `TodoWrite` для сеансов, которые еще не перешли на новую версию.
</Note>

<h3 id="todo-lifecycle">
  Жизненный цикл задач
</h3>

Задачи следуют предсказуемому жизненному циклу:

1. **Созданы** как `pending` при выявлении задач
2. **Активированы** в `in_progress` при начале работы
3. **Завершены** при успешном завершении задачи
4. **Удалены** при завершении всех задач в группе

<h3 id="when-todos-are-used">
  Когда используются задачи
</h3>

SDK создает задачи для большинства многошаговых работ, таких как:

* **Сложных многошаговых задач**, требующих 3 или более отдельных действий
* **Списков задач, предоставленных пользователем**, когда упоминаются несколько элементов
* **Нетривиальных операций**, которые выигрывают от отслеживания прогресса
* **Явных запросов**, когда пользователи просят организовать задачи

Это может пропустить задачи для очень коротких или одношаговых запросов.

<h2 id="examples">
  Примеры
</h2>

Перед запуском этих примеров установите Claude Agent SDK, следуя [краткому руководству](/docs/ru/agent-sdk/quickstart).

Каждый пример выполняется до завершения агентом и выдачи его финального сообщения результата. Если сеанс сначала достигает лимита ходов, то сообщение результата имеет подтип `error_max_turns`. Проверьте `subtype`, чтобы обнаружить это завершение.

Эти примеры используют однократные вызовы `query()`. После выдачи результата `error_max_turns`, `query()` выбрасывает ошибку, которая включает `Reached maximum number of turns`. Каждый пример оборачивает свой цикл в блок try для чистого выхода при возникновении этого события.

См. [Обработка результата](/docs/ru/agent-sdk/agent-loop#handle-the-result) для получения информации о подтипах результатов.

<h3 id="monitoring-todo-changes">
  Мониторинг изменений задач
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      // Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
      // Task tools instead and these tool_use blocks never appear.
      options: { maxTurns: 15, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
    })) {
      // Todo updates are reflected in the message stream
      if (message.type === "assistant") {
        for (const block of message.message.content) {
          if (block.type === "tool_use" && block.name === "TodoWrite") {
            const todos = block.input.todos;

            console.log("Todo Status Update:");
            todos.forEach((todo, index) => {
              const status =
                todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
              console.log(`${index + 1}. ${status} ${todo.content}`);
            });
          }
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // such as when the maxTurns limit is hit.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock


  async def main():
      try:
          async for message in query(
              prompt="Optimize my React app performance and track progress with todos",
              # Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
              # Task tools instead and these tool_use blocks never appear.
              options=ClaudeAgentOptions(max_turns=15, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
          ):
              # Todo updates are reflected in the message stream
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                          todos = block.input["todos"]

                          print("Todo Status Update:")
                          for i, todo in enumerate(todos):
                              status = (
   "✅"
   if todo["status"] == "completed"
   else "🔧"
   if todo["status"] == "in_progress"
   else "❌"
                              )
                              print(f"{i + 1}. {status} {todo['content']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # such as when the max_turns limit is hit.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="real-time-progress-display">
  Отображение прогресса в реальном времени
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  class TodoTracker {
    private todos: any[] = [];

    displayProgress() {
      if (this.todos.length === 0) return;

      const completed = this.todos.filter((t) => t.status === "completed").length;
      const inProgress = this.todos.filter((t) => t.status === "in_progress").length;
      const total = this.todos.length;

      console.log(`\nProgress: ${completed}/${total} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      this.todos.forEach((todo, index) => {
        const icon =
          todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
        const text = todo.status === "in_progress" ? todo.activeForm : todo.content;
        console.log(`${index + 1}. ${icon} ${text}`);
      });
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          // Re-enable TodoWrite, which this tracker watches for.
          options: { maxTurns: 20, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use" && block.name === "TodoWrite") {
                this.todos = block.input.todos;
                this.displayProgress();
              }
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
  const tracker = new TodoTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
  from typing import List, Dict


  class TodoTracker:
      def __init__(self):
          self.todos: List[Dict] = []

      def display_progress(self):
          if not self.todos:
              return

          completed = len([t for t in self.todos if t["status"] == "completed"])
          in_progress = len([t for t in self.todos if t["status"] == "in_progress"])
          total = len(self.todos)

          print(f"\nProgress: {completed}/{total} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for i, todo in enumerate(self.todos):
              icon = (
                  "✅"
                  if todo["status"] == "completed"
                  else "🔧"
                  if todo["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  todo["activeForm"]
                  if todo["status"] == "in_progress"
                  else todo["content"]
              )
              print(f"{i + 1}. {icon} {text}")

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  # Re-enable TodoWrite, which this tracker watches for.
                  options=ClaudeAgentOptions(max_turns=20, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                              self.todos = block.input["todos"]
                              self.display_progress()
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TodoTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="migrate-to-task-tools">
  Миграция на инструменты Task
</h2>

Инструменты Task разделяют единый вызов `TodoWrite` на `TaskCreate` для каждого нового элемента и `TaskUpdate` для каждого изменения статуса, с `TaskList` и `TaskGet`, доступными для модели для чтения текущего списка. Ваш код мониторинга по-прежнему проверяет блоки `tool_use` в потоке помощника, но поддерживает карту, индексированную по ID задачи, вместо замены всего списка при каждом вызове. Инструменты Task являются стандартными начиная с TypeScript Agent SDK 0.3.142 и Claude Code v2.1.142, поэтому изменение `options.env` не требуется.

| С `TodoWrite`                                           | С инструментами Task                                                                                                                                                                                                                                                                                         |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Один вызов инструмента переписывает весь массив `todos` | `TaskCreate` добавляет один элемент, `TaskUpdate` исправляет один элемент по `taskId`                                                                                                                                                                                                                        |
| Совпадение `block.name === "TodoWrite"`                 | Совпадение `block.name === "TaskCreate"` или `"TaskUpdate"`                                                                                                                                                                                                                                                  |
| Форма элемента: `{ content, status, activeForm }`       | Ввод `TaskCreate`: `{ subject, description, activeForm?, metadata? }`. Ввод `TaskUpdate`: `{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`. `status` это `"pending"`, `"in_progress"` или `"completed"`; установите `status: "deleted"` для удаления |
| Отобразить `block.input.todos` напрямую                 | Накопить элементы между вызовами или прочитать снимок из результата инструмента `TaskList`                                                                                                                                                                                                                   |

Назначенный ID задачи отсутствует во вводе `TaskCreate`. Он возвращается в соответствующем `tool_result` как `{ task: { id, subject } }`, поэтому захватите его из блока результата, чтобы индексировать вашу карту. Следующий пример показывает минимальное изменение цикла [Мониторинг изменений задач](#monitoring-todo-changes). Он читает только вводы `tool_use` и пропускает захват ID из блоков `tool_result`. Для отображения полного списка смотрите результат инструмента `TaskList` в потоке или накопите результаты `TaskCreate` и вводы `TaskUpdate` в карту.

Потоковый ввод `tool_use` — это необработанная форма, которую выдала модель. Claude Code исправляет некоторые близкие, но неправильные имена ключей перед выполнением, сопоставляя `id` или `task_id` с `taskId` и `active_form` с `activeForm`, но это исправление не отражается в потоке. Читайте поля ввода `TaskUpdate` защитно, как это делают примеры ниже, а не предполагайте, что каноническое имя всегда присутствует.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      options: { maxTurns: 15 },
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
              prompt="Optimize my React app performance and track progress with todos",
              options=ClaudeAgentOptions(max_turns=15),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input['subject']}")
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

<h2 id="related-documentation">
  Связанная документация
</h2>

* [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript)
* [Справочник Python SDK](/docs/ru/agent-sdk/python)
* [Потоковая передача в сравнении с одиночным режимом](/docs/ru/agent-sdk/streaming-vs-single-mode)
* [Пользовательские инструменты](/docs/ru/agent-sdk/custom-tools)
