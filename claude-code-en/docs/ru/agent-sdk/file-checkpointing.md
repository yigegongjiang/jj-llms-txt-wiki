> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Отмотка изменений файлов с помощью checkpointing

> Отслеживайте изменения файлов во время сеансов агента и восстанавливайте файлы в любое предыдущее состояние

File checkpointing отслеживает изменения файлов, внесённые через инструменты Write, Edit и NotebookEdit во время сеанса агента, позволяя вам отмотать файлы в любое предыдущее состояние. Хотите попробовать? Перейдите к [интерактивному примеру](#try-it-out).

С помощью checkpointing вы можете:

* **Отменить нежелательные изменения**, восстановив файлы в известное хорошее состояние
* **Исследовать альтернативы**, восстановив checkpoint и попробовав другой подход
* **Восстановиться после ошибок**, когда агент вносит неправильные изменения

<Warning>
  Отслеживаются только изменения, внесённые через инструменты Write, Edit и NotebookEdit. Изменения, внесённые через команды Bash (например, `echo > file.txt` или `sed -i`), не захватываются системой checkpoint.
</Warning>

<h2 id="how-checkpointing-works">
  Как работает checkpointing
</h2>

Когда вы включаете file checkpointing, SDK создаёт резервные копии файлов перед их изменением через инструменты Write, Edit или NotebookEdit. Пользовательские сообщения в потоке ответов включают UUID checkpoint, который вы можете использовать как точку восстановления.

Checkpoint работает с этими встроенными инструментами, которые агент использует для изменения файлов:

| Инструмент   | Описание                                                                 |
| ------------ | ------------------------------------------------------------------------ |
| Write        | Создаёт новый файл или перезаписывает существующий файл новым содержимым |
| Edit         | Вносит целевые правки в определённые части существующего файла           |
| NotebookEdit | Изменяет ячейки в Jupyter notebooks (файлы `.ipynb`)                     |

<Note>
  File rewinding восстанавливает файлы на диске в предыдущее состояние. Это не отматывает саму беседу. История беседы и контекст остаются нетронутыми после вызова `rewindFiles()` (TypeScript) или `rewind_files()` (Python).
</Note>

Система checkpoint отслеживает:

* Файлы, созданные во время сеанса
* Файлы, изменённые во время сеанса
* Исходное содержимое изменённых файлов

Когда вы отматываете к checkpoint, созданные файлы удаляются, а изменённые файлы восстанавливаются до их содержимого в этот момент.

<h2 id="implement-checkpointing">
  Реализация checkpointing
</h2>

Чтобы использовать file checkpointing, включите его в ваших параметрах, захватите UUID checkpoint из потока ответов, затем вызовите `rewindFiles()` (TypeScript) или `rewind_files()` (Python) когда вам нужно восстановить.

Следующий пример показывает полный процесс: включение checkpointing, захват UUID checkpoint и ID сеанса из потока ответов, затем возобновление сеанса позже для отмотки файлов. Каждый шаг подробно объясняется ниже. Примеры в этом разделе используют приглашение "Refactor the authentication module". Запустите их в проекте, который содержит модуль аутентификации, или измените приглашение на имена файлов, которые существуют в вашем проекте, чтобы вы могли наблюдать изменения файлов и видеть, как отмотка восстанавливает их.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeSDKClient,
      ClaudeAgentOptions,
      UserMessage,
      ResultMessage,
  )


  async def main():
      # Step 1: Enable checkpointing
      options = ClaudeAgentOptions(
          enable_file_checkpointing=True,
          permission_mode="acceptEdits",  # Auto-accept file edits without prompting
          extra_args={
              "replay-user-messages": None
          },  # Required to receive checkpoint UUIDs in the response stream
      )

      checkpoint_id = None
      session_id = None

      # Run the query and capture checkpoint UUID and session ID
      async with ClaudeSDKClient(options) as client:
          await client.query("Refactor the authentication module")

          # Step 2: Capture checkpoint UUID from the first user message
          async for message in client.receive_response():
              if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                  checkpoint_id = message.uuid
              if isinstance(message, ResultMessage) and not session_id:
                  session_id = message.session_id

      # Step 3: Later, rewind by resuming the session with an empty prompt
      if checkpoint_id and session_id:
          async with ClaudeSDKClient(
              ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
          ) as client:
              await client.query("")  # Empty prompt to open the connection
              async for message in client.receive_response():
                  await client.rewind_files(checkpoint_id)
                  break
          print(f"Rewound to checkpoint: {checkpoint_id}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  async function main() {
    // Step 1: Enable checkpointing
    const opts = {
      enableFileCheckpointing: true,
      permissionMode: "acceptEdits" as const, // Auto-accept file edits without prompting
      extraArgs: { "replay-user-messages": null } // Required to receive checkpoint UUIDs in the response stream
    };

    const response = query({
      prompt: "Refactor the authentication module",
      options: opts
    });

    let checkpointId: string | undefined;
    let sessionId: string | undefined;

    // Step 2: Capture checkpoint UUID from the first user message
    for await (const message of response) {
      if (message.type === "user" && message.uuid && !checkpointId) {
        checkpointId = message.uuid;
      }
      if ("session_id" in message && !sessionId) {
        sessionId = message.session_id;
      }
    }

    // Step 3: Later, rewind by resuming the session with an empty prompt
    if (checkpointId && sessionId) {
      const rewindQuery = query({
        prompt: "", // Empty prompt to open the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);
        break;
      }
      console.log(`Rewound to checkpoint: ${checkpointId}`);
    }
  }

  main();
  ```
</CodeGroup>

<Steps>
  <Step title="Включение checkpointing">
    Настройте параметры SDK для включения checkpointing и получения UUID checkpoint:

    | Параметр                 | Python                                      | TypeScript                                    | Описание                                                         |
    | ------------------------ | ------------------------------------------- | --------------------------------------------- | ---------------------------------------------------------------- |
    | Включить checkpointing   | `enable_file_checkpointing=True`            | `enableFileCheckpointing: true`               | Отслеживает изменения файлов для отмотки                         |
    | Получить UUID checkpoint | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Требуется для получения UUID пользовательских сообщений в потоке |

    <CodeGroup>
      ```python Python theme={null}
      options = ClaudeAgentOptions(
          enable_file_checkpointing=True,
          permission_mode="acceptEdits",
          extra_args={"replay-user-messages": None},
      )

      async with ClaudeSDKClient(options) as client:
          await client.query("Refactor the authentication module")
      ```

      ```typescript TypeScript theme={null}
      const response = query({
        prompt: "Refactor the authentication module",
        options: {
          enableFileCheckpointing: true,
          permissionMode: "acceptEdits" as const,
          extraArgs: { "replay-user-messages": null }
        }
      });
      ```
    </CodeGroup>
  </Step>

  <Step title="Захват UUID checkpoint и ID сеанса">
    С установленным параметром `replay-user-messages` (показано выше), каждое пользовательское сообщение в потоке ответов имеет UUID, который служит checkpoint.

    Для большинства случаев использования захватите UUID первого пользовательского сообщения (`message.uuid`); отмотка к нему восстанавливает все файлы в их исходное состояние. Чтобы сохранить несколько checkpoint и отмотать к промежуточным состояниям, см. [Несколько точек восстановления](#multiple-restore-points).

    Захват ID сеанса (`message.session_id`) является необязательным; вам он нужен только если вы хотите отмотать позже, после завершения потока. Если вы вызываете `rewindFiles()` немедленно, пока всё ещё обрабатываете сообщения (как это делает пример в [Checkpoint перед рискованными операциями](#checkpoint-before-risky-operations)), вы можете пропустить захват ID сеанса.

    <CodeGroup>
      ```python Python theme={null}
      checkpoint_id = None
      session_id = None

      async for message in client.receive_response():
          # Capture the first user message UUID as the checkpoint
          if isinstance(message, UserMessage) and message.uuid and checkpoint_id is None:
              checkpoint_id = message.uuid
          # Capture session ID from the result message
          if isinstance(message, ResultMessage):
              session_id = message.session_id
      ```

      ```typescript TypeScript theme={null}
      let checkpointId: string | undefined;
      let sessionId: string | undefined;

      for await (const message of response) {
        // Capture the first user message UUID as the checkpoint
        if (message.type === "user" && message.uuid && !checkpointId) {
          checkpointId = message.uuid;
        }
        // Capture session ID from any message that has it
        if ("session_id" in message) {
          sessionId = message.session_id;
        }
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Отмотка файлов">
    Чтобы отмотать после завершения потока, возобновите сеанс с пустым приглашением и вызовите `rewind_files()` (Python) или `rewindFiles()` (TypeScript) с вашим UUID checkpoint. Вы также можете отмотать во время потока; см. [Checkpoint перед рискованными операциями](#checkpoint-before-risky-operations) для этого паттерна.

    <CodeGroup>
      ```python Python theme={null}
      async with ClaudeSDKClient(
          ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
      ) as client:
          await client.query("")  # Empty prompt to open the connection
          async for message in client.receive_response():
              await client.rewind_files(checkpoint_id)
              break
      ```

      ```typescript TypeScript theme={null}
      const rewindQuery = query({
        prompt: "", // Empty prompt to open the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);
        break;
      }
      ```
    </CodeGroup>

    Если вы захватили ID сеанса и UUID checkpoint, вы также можете отмотать из CLI. Эта команда требует исполняемого файла `claude`, который поставляется с [установкой Claude Code](/docs/ru/setup) и не устанавливается пакетом SDK. SDK включает checkpointing для вас, но когда вы запускаете `claude -p` напрямую, вы должны установить переменную окружения `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`:

    ```bash theme={null}
    CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
    ```

    Флаг `--rewind-files` не отображается в выводе `claude --help`, но CLI принимает его, как показано.
  </Step>
</Steps>

<h2 id="common-patterns">
  Общие паттерны
</h2>

Эти паттерны показывают различные способы захвата и использования UUID checkpoint в зависимости от вашего случая использования.

<h3 id="checkpoint-before-risky-operations">
  Checkpoint перед рискованными операциями
</h3>

Этот паттерн сохраняет только самый последний UUID checkpoint, обновляя его перед каждым ходом агента. Если что-то пойдёт не так во время обработки, вы можете немедленно отмотать к последнему безопасному состоянию и выйти из цикла.

Перед запуском этого примера замените `your_revert_condition` (Python) или `yourRevertCondition` (TypeScript) на вашу собственную проверку, такую как обнаружение ошибок или сбой валидации; заполнитель не определён в примере.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage


  async def main():
      options = ClaudeAgentOptions(
          enable_file_checkpointing=True,
          permission_mode="acceptEdits",
          extra_args={"replay-user-messages": None},
      )

      safe_checkpoint = None

      async with ClaudeSDKClient(options) as client:
          await client.query("Refactor the authentication module")

          async for message in client.receive_response():
              # Update checkpoint before each agent turn starts
              # This overwrites the previous checkpoint. Only keep the latest
              if isinstance(message, UserMessage) and message.uuid:
                  safe_checkpoint = message.uuid

              # Decide when to revert based on your own logic
              # For example: error detection, validation failure, or user input
              if your_revert_condition and safe_checkpoint:
                  await client.rewind_files(safe_checkpoint)
                  # Exit the loop after rewinding, files are restored
                  break


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  async function main() {
    const response = query({
      prompt: "Refactor the authentication module",
      options: {
        enableFileCheckpointing: true,
        permissionMode: "acceptEdits" as const,
        extraArgs: { "replay-user-messages": null }
      }
    });

    let safeCheckpoint: string | undefined;

    for await (const message of response) {
      // Update checkpoint before each agent turn starts
      // This overwrites the previous checkpoint. Only keep the latest
      if (message.type === "user" && message.uuid) {
        safeCheckpoint = message.uuid;
      }

      // Decide when to revert based on your own logic
      // For example: error detection, validation failure, or user input
      if (yourRevertCondition && safeCheckpoint) {
        await response.rewindFiles(safeCheckpoint);
        // Exit the loop after rewinding, files are restored
        break;
      }
    }
  }

  main();
  ```
</CodeGroup>

<h3 id="multiple-restore-points">
  Несколько точек восстановления
</h3>

Если Claude вносит изменения в несколько ходов, вы можете захотеть отмотать к определённой точке, а не полностью назад. Например, если Claude рефакторит файл в ход один и добавляет тесты в ход два, вы можете захотеть сохранить рефакторинг, но отменить тесты.

Этот паттерн сохраняет все UUID checkpoint в массиве с метаданными. После завершения сеанса вы можете отмотать к любому предыдущему checkpoint:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from dataclasses import dataclass
  from datetime import datetime
  from claude_agent_sdk import (
      ClaudeSDKClient,
      ClaudeAgentOptions,
      UserMessage,
      ResultMessage,
  )


  # Store checkpoint metadata for better tracking
  @dataclass
  class Checkpoint:
      id: str
      description: str
      timestamp: datetime


  async def main():
      options = ClaudeAgentOptions(
          enable_file_checkpointing=True,
          permission_mode="acceptEdits",
          extra_args={"replay-user-messages": None},
      )

      checkpoints = []
      session_id = None

      async with ClaudeSDKClient(options) as client:
          await client.query("Refactor the authentication module")

          async for message in client.receive_response():
              if isinstance(message, UserMessage) and message.uuid:
                  checkpoints.append(
                      Checkpoint(
                          id=message.uuid,
                          description=f"After turn {len(checkpoints) + 1}",
                          timestamp=datetime.now(),
                      )
                  )
              if isinstance(message, ResultMessage) and not session_id:
                  session_id = message.session_id

      # Later: rewind to any checkpoint by resuming the session
      if checkpoints and session_id:
          target = checkpoints[0]  # Pick any checkpoint
          async with ClaudeSDKClient(
              ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
          ) as client:
              await client.query("")  # Empty prompt to open the connection
              async for message in client.receive_response():
                  await client.rewind_files(target.id)
                  break
          print(f"Rewound to: {target.description}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Store checkpoint metadata for better tracking
  interface Checkpoint {
    id: string;
    description: string;
    timestamp: Date;
  }

  async function main() {
    const opts = {
      enableFileCheckpointing: true,
      permissionMode: "acceptEdits" as const,
      extraArgs: { "replay-user-messages": null }
    };

    const response = query({
      prompt: "Refactor the authentication module",
      options: opts
    });

    const checkpoints: Checkpoint[] = [];
    let sessionId: string | undefined;

    for await (const message of response) {
      if (message.type === "user" && message.uuid) {
        checkpoints.push({
          id: message.uuid,
          description: `After turn ${checkpoints.length + 1}`,
          timestamp: new Date()
        });
      }
      if ("session_id" in message && !sessionId) {
        sessionId = message.session_id;
      }
    }

    // Later: rewind to any checkpoint by resuming the session
    if (checkpoints.length > 0 && sessionId) {
      const target = checkpoints[0]; // Pick any checkpoint
      const rewindQuery = query({
        prompt: "", // Empty prompt to open the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(target.id);
        break;
      }
      console.log(`Rewound to: ${target.description}`);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="try-it-out">
  Попробуйте
</h2>

Этот полный пример создаёт небольшой служебный файл, просит агента добавить комментарии к документации, показывает вам изменения, затем спрашивает, хотите ли вы отмотать.

Прежде чем начать, убедитесь, что у вас установлен [Claude Agent SDK](/docs/ru/agent-sdk/quickstart).

<Steps>
  <Step title="Создание тестового файла">
    Создайте новый файл с именем `utils.py` (Python) или `utils.ts` (TypeScript) и вставьте следующий код:

    <CodeGroup>
      ```python utils.py theme={null}
      def add(a, b):
          return a + b


      def subtract(a, b):
          return a - b


      def multiply(a, b):
          return a * b


      def divide(a, b):
          if b == 0:
              raise ValueError("Cannot divide by zero")
          return a / b
      ```

      ```typescript utils.ts theme={null}
      export function add(a: number, b: number): number {
        return a + b;
      }

      export function subtract(a: number, b: number): number {
        return a - b;
      }

      export function multiply(a: number, b: number): number {
        return a * b;
      }

      export function divide(a: number, b: number): number {
        if (b === 0) {
          throw new Error("Cannot divide by zero");
        }
        return a / b;
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Запуск интерактивного примера">
    Создайте новый файл с именем `try_checkpointing.py` (Python) или `try_checkpointing.ts` (TypeScript) в том же каталоге, что и ваш служебный файл, и вставьте следующий код.

    Этот скрипт просит Claude добавить комментарии к документации в ваш служебный файл, затем даёт вам возможность отмотать и восстановить оригинал.

    <CodeGroup>
      ```python try_checkpointing.py theme={null}
      import asyncio
      from claude_agent_sdk import (
          ClaudeSDKClient,
          ClaudeAgentOptions,
          UserMessage,
          ResultMessage,
      )


      async def main():
          # Configure the SDK with checkpointing enabled
          # - enable_file_checkpointing: Track file changes for rewinding
          # - permission_mode: Auto-accept file edits without prompting
          # - extra_args: Required to receive user message UUIDs in the stream
          options = ClaudeAgentOptions(
              enable_file_checkpointing=True,
              permission_mode="acceptEdits",
              extra_args={"replay-user-messages": None},
          )

          checkpoint_id = None  # Store the user message UUID for rewinding
          session_id = None  # Store the session ID for resuming

          print("Running agent to add doc comments to utils.py...\n")

          # Run the agent and capture checkpoint data from the response stream
          async with ClaudeSDKClient(options) as client:
              await client.query("Add doc comments to utils.py")

              async for message in client.receive_response():
                  # Capture the first user message UUID - this is our restore point
                  if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                      checkpoint_id = message.uuid
                  # Capture the session ID so we can resume later
                  if isinstance(message, ResultMessage):
                      session_id = message.session_id

          print("Done! Open utils.py to see the added doc comments.\n")

          # Ask the user if they want to rewind the changes
          if checkpoint_id and session_id:
              response = input("Rewind to remove the doc comments? (y/n): ")

              if response.lower() == "y":
                  # Resume the session with an empty prompt, then rewind
                  async with ClaudeSDKClient(
                      ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
                  ) as client:
                      await client.query("")  # Empty prompt opens the connection
                      async for message in client.receive_response():
                          await client.rewind_files(checkpoint_id)  # Restore files
                          break

                  print(
                      "\n✓ File restored! Open utils.py to verify the doc comments are gone."
                  )
              else:
                  print("\nKept the modified file.")


      asyncio.run(main())
      ```

      ```typescript try_checkpointing.ts theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";
      import * as readline from "readline";

      async function main() {
        // Configure the SDK with checkpointing enabled
        // - enableFileCheckpointing: Track file changes for rewinding
        // - permissionMode: Auto-accept file edits without prompting
        // - extraArgs: Required to receive user message UUIDs in the stream
        const opts = {
          enableFileCheckpointing: true,
          permissionMode: "acceptEdits" as const,
          extraArgs: { "replay-user-messages": null }
        };

        let sessionId: string | undefined; // Store the session ID for resuming
        let checkpointId: string | undefined; // Store the user message UUID for rewinding

        console.log("Running agent to add doc comments to utils.ts...\n");

        // Run the agent and capture checkpoint data from the response stream
        const response = query({
          prompt: "Add doc comments to utils.ts",
          options: opts
        });

        for await (const message of response) {
          // Capture the first user message UUID - this is our restore point
          if (message.type === "user" && message.uuid && !checkpointId) {
            checkpointId = message.uuid;
          }
          // Capture the session ID so we can resume later
          if ("session_id" in message) {
            sessionId = message.session_id;
          }
        }

        console.log("Done! Open utils.ts to see the added doc comments.\n");

        // Ask the user if they want to rewind the changes
        if (checkpointId && sessionId) {
          const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
          });

          const answer = await new Promise<string>((resolve) => {
            rl.question("Rewind to remove the doc comments? (y/n): ", resolve);
          });
          rl.close();

          if (answer.toLowerCase() === "y") {
            // Resume the session with an empty prompt, then rewind
            const rewindQuery = query({
              prompt: "", // Empty prompt opens the connection
              options: { ...opts, resume: sessionId }
            });

            for await (const msg of rewindQuery) {
              await rewindQuery.rewindFiles(checkpointId); // Restore files
              break;
            }

            console.log("\n✓ File restored! Open utils.ts to verify the doc comments are gone.");
          } else {
            console.log("\nKept the modified file.");
          }
        }
      }

      main();
      ```
    </CodeGroup>

    Этот пример демонстрирует полный рабочий процесс checkpointing:

    1. **Включение checkpointing**: настройте SDK с `enable_file_checkpointing=True` и `permission_mode="acceptEdits"` для автоматического одобрения правок файлов
    2. **Захват данных checkpoint**: по мере выполнения агента сохраняйте UUID первого пользовательского сообщения (вашу точку восстановления) и ID сеанса
    3. **Запрос на отмотку**: после завершения агента проверьте ваш служебный файл, чтобы увидеть комментарии к документации, затем решите, хотите ли вы отменить изменения
    4. **Возобновление и отмотка**: если да, возобновите сеанс с пустым приглашением и вызовите `rewind_files()` для восстановления исходного файла
  </Step>

  <Step title="Запуск примера">
    Запустите скрипт из того же каталога, что и ваш служебный файл.

    <Tip>
      Откройте ваш служебный файл (`utils.py` или `utils.ts`) в вашей IDE или редакторе перед запуском скрипта. Вы увидите, как файл обновляется в реальном времени, когда агент добавляет комментарии к документации, затем вернётся к оригиналу, когда вы выберете отмотку.
    </Tip>

    <Tabs>
      <Tab title="Python">
        ```bash theme={null}
        python try_checkpointing.py
        ```
      </Tab>

      <Tab title="TypeScript">
        ```bash theme={null}
        npx tsx try_checkpointing.ts
        ```
      </Tab>
    </Tabs>

    Вы увидите, как агент добавляет комментарии к документации, затем появится приглашение, спрашивающее, хотите ли вы отмотать. Если вы выберете да, файл будет восстановлен в его исходное состояние.
  </Step>
</Steps>

<h2 id="limitations">
  Ограничения
</h2>

File checkpointing имеет следующие ограничения:

| Ограничение                                | Описание                                                            |
| ------------------------------------------ | ------------------------------------------------------------------- |
| Только инструменты Write/Edit/NotebookEdit | Изменения, внесённые через команды Bash, не отслеживаются           |
| Один сеанс                                 | Checkpoints привязаны к сеансу, который их создал                   |
| Только содержимое файла                    | Создание, перемещение или удаление каталогов не отменяется отмоткой |
| Локальные файлы                            | Удалённые или сетевые файлы не отслеживаются                        |

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="checkpointing-options-not-recognized">
  Параметры checkpointing не распознаны
</h3>

Если `enableFileCheckpointing` или `rewindFiles()` недоступны, вы можете использовать старую версию SDK.

**Решение**: Обновитесь до последней версии SDK:

* **Python**: `pip install --upgrade claude-agent-sdk`
* **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

<h3 id="user-messages-don’t-have-uuids">
  Пользовательские сообщения не имеют UUID
</h3>

Если `message.uuid` имеет значение `undefined` или отсутствует, вы не получаете UUID checkpoint.

**Причина**: Параметр `replay-user-messages` не установлен.

**Решение**: Добавьте `extra_args={"replay-user-messages": None}` (Python) или `extraArgs: { 'replay-user-messages': null }` (TypeScript) в ваши параметры.

<h3 id="no-file-checkpoint-found-for-message-error">
  Ошибка "No file checkpoint found for message"
</h3>

Эта ошибка возникает, когда данные checkpoint не существуют для указанного UUID пользовательского сообщения.

**Частые причины**:

* File checkpointing не был включён в исходном сеансе (`enable_file_checkpointing` или `enableFileCheckpointing` не был установлен на `true`)
* Сеанс не был должным образом завершён перед попыткой возобновления и отмотки

**Решение**: Убедитесь, что `enable_file_checkpointing=True` (Python) или `enableFileCheckpointing: true` (TypeScript) был установлен в исходном сеансе, затем используйте паттерн, показанный в примерах: захватите UUID первого пользовательского сообщения, полностью завершите сеанс, затем возобновите с пустым приглашением и вызовите `rewindFiles()` один раз.

<h3 id="file-rewinding-is-not-enabled-error">
  Ошибка "File rewinding is not enabled"
</h3>

Эта ошибка возникает, когда вы пытаетесь выполнить неинтерактивную отмотку без включённого checkpointing: запуск простой команды `claude -p` с `--rewind-files`, или запуск сеанса SDK, включая возобновленный, чьи параметры не включают checkpointing. SDK устанавливает переменную окружения `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` внутренне только когда `enable_file_checkpointing` (Python) или `enableFileCheckpointing` (TypeScript) включены в сеансе, выполняющем отмотку; простой CLI никогда её не устанавливает.

**Решение**: Для простого CLI установите переменную окружения при запуске команды:

```bash theme={null}
CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
```

Для SDK установите `enable_file_checkpointing=True` (Python) или `enableFileCheckpointing: true` (TypeScript) в возобновленном сеансе, как это делается в примерах на этой странице.

<h3 id="processtransport-is-not-ready-for-writing-error">
  Ошибка "ProcessTransport is not ready for writing"
</h3>

Эта ошибка возникает, когда вы вызываете `rewindFiles()` или `rewind_files()` после завершения итерации по ответу. Соединение с процессом CLI закрывается при завершении цикла.

**Решение**: Возобновите сеанс с пустым приглашением, затем отмотайте в новом запросе:

<CodeGroup>
  ```python Python theme={null}
  # Resume session with empty prompt, then rewind
  async with ClaudeSDKClient(
      ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
  ) as client:
      await client.query("")
      async for message in client.receive_response():
          await client.rewind_files(checkpoint_id)
          break
  ```

  ```typescript TypeScript theme={null}
  // Resume session with empty prompt, then rewind
  const rewindQuery = query({
    prompt: "",
    options: { ...opts, resume: sessionId }
  });

  for await (const msg of rewindQuery) {
    await rewindQuery.rewindFiles(checkpointId);
    break;
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Следующие шаги
</h2>

* **[Sessions](/docs/ru/agent-sdk/sessions)**: узнайте, как возобновлять сеансы, что требуется для отмотки после завершения потока. Охватывает ID сеансов, возобновление бесед и разветвление сеансов.
* **[Permissions](/docs/ru/agent-sdk/permissions)**: настройте, какие инструменты может использовать Claude и как одобряются изменения файлов. Полезно, если вы хотите больше контроля над тем, когда происходят правки.
* **[TypeScript SDK reference](/docs/ru/agent-sdk/typescript)**: полный справочник API, включая все параметры для `query()` и метода `rewindFiles()`.
* **[Python SDK reference](/docs/ru/agent-sdk/python)**: полный справочник API, включая все параметры для `ClaudeAgentOptions` и метода `rewind_files()`.
