> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 checkpointing 回溯檔案變更

> 追蹤代理程式工作階段期間的檔案變更，並將檔案還原到任何先前的狀態

File checkpointing 追蹤在代理程式工作階段期間透過 Write、Edit 和 NotebookEdit 工具所做的檔案修改，允許您將檔案回溯到任何先前的狀態。想要試試看嗎？跳到[互動式範例](#try-it-out)。

使用 checkpointing，您可以：

* **撤銷不需要的變更**，透過將檔案還原到已知的良好狀態
* **探索替代方案**，透過還原到 checkpoint 並嘗試不同的方法
* **從錯誤中恢復**，當代理程式進行不正確的修改時

<Warning>
  只有透過 Write、Edit 和 NotebookEdit 工具所做的變更才會被追蹤。透過 Bash 命令所做的變更（例如 `echo > file.txt` 或 `sed -i`）不會被 checkpoint 系統捕捉。
</Warning>

<h2 id="how-checkpointing-works">
  checkpointing 如何運作
</h2>

當您啟用檔案 checkpointing 時，SDK 會在透過 Write、Edit 或 NotebookEdit 工具修改檔案之前建立檔案備份。回應串流中的使用者訊息包含一個 checkpoint UUID，您可以將其用作還原點。

Checkpoint 適用於代理程式用來修改檔案的這些內建工具：

| 工具           | 描述                               |
| ------------ | -------------------------------- |
| Write        | 建立新檔案或用新內容覆蓋現有檔案                 |
| Edit         | 對現有檔案的特定部分進行有針對性的編輯              |
| NotebookEdit | 修改 Jupyter 筆記本（`.ipynb` 檔案）中的儲存格 |

<Note>
  檔案回溯將磁碟上的檔案還原到先前的狀態。它不會回溯對話本身。呼叫 `rewindFiles()`（TypeScript）或 `rewind_files()`（Python）後，對話歷史記錄和上下文保持不變。
</Note>

checkpoint 系統追蹤：

* 在工作階段期間建立的檔案
* 在工作階段期間修改的檔案
* 修改檔案的原始內容

當您回溯到 checkpoint 時，建立的檔案會被刪除，修改的檔案會還原到該時間點的內容。

<h2 id="implement-checkpointing">
  實現 checkpointing
</h2>

若要使用檔案 checkpointing，請在您的選項中啟用它，從回應串流中捕捉 checkpoint UUID，然後在需要還原時呼叫 `rewindFiles()`（TypeScript）或 `rewind_files()`（Python）。

以下範例顯示完整流程：啟用 checkpointing、從回應串流中捕捉 checkpoint UUID 和工作階段 ID，然後稍後恢復工作階段以回溯檔案。下面詳細說明每個步驟。本節中的範例使用提示「重構驗證模組」。在包含驗證模組的專案中執行它們，或變更提示以命名您專案中存在的檔案，以便您可以觀看檔案變更並查看回溯如何還原它們。

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
  <Step title="啟用 checkpointing">
    配置您的 SDK 選項以啟用 checkpointing 並接收 checkpoint UUID：

    | 選項                 | Python                                      | TypeScript                                    | 描述                 |
    | ------------------ | ------------------------------------------- | --------------------------------------------- | ------------------ |
    | 啟用 checkpointing   | `enable_file_checkpointing=True`            | `enableFileCheckpointing: true`               | 追蹤檔案變更以進行回溯        |
    | 接收 checkpoint UUID | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | 需要在串流中取得使用者訊息 UUID |

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

  <Step title="捕捉 checkpoint UUID 和工作階段 ID">
    設定 `replay-user-messages` 選項後（如上所示），回應串流中的每個使用者訊息都有一個 UUID，可作為 checkpoint。

    對於大多數使用案例，捕捉第一個使用者訊息 UUID（`message.uuid`）；回溯到它會將所有檔案還原到其原始狀態。若要儲存多個 checkpoint 並回溯到中間狀態，請參閱[多個還原點](#multiple-restore-points)。

    捕捉工作階段 ID（`message.session_id`）是可選的；只有在您想要稍後回溯（在串流完成後）時才需要它。如果您在仍在處理訊息時立即呼叫 `rewindFiles()`（如[在危險操作前進行 checkpoint](#checkpoint-before-risky-operations) 中的範例所做的），您可以跳過捕捉工作階段 ID。

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

  <Step title="回溯檔案">
    若要在串流完成後回溯，請使用空提示恢復工作階段，並使用您的 checkpoint UUID 呼叫 `rewind_files()`（Python）或 `rewindFiles()`（TypeScript）。您也可以在串流期間回溯；請參閱[在危險操作前進行 checkpoint](#checkpoint-before-risky-operations) 以了解該模式。

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

    如果您捕捉了工作階段 ID 和 checkpoint ID，您也可以從 CLI 回溯。此命令需要 `claude` 可執行檔，該檔案來自[安裝 Claude Code](/docs/zh-TW/setup)，並且不是由 SDK 套件安裝的。SDK 為您啟用 checkpointing，但當您直接執行 `claude -p` 時，您必須設定 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 環境變數：

    ```bash theme={null}
    CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
    ```

    `--rewind-files` 旗標不會出現在 `claude --help` 輸出中，但 CLI 會如上所示接受它。
  </Step>
</Steps>

<h2 id="common-patterns">
  常見模式
</h2>

這些模式顯示根據您的使用案例捕捉和使用 checkpoint UUID 的不同方式。

<h3 id="checkpoint-before-risky-operations">
  在危險操作前進行 checkpoint
</h3>

此模式只保留最新的 checkpoint UUID，在每個代理程式轉向前更新它。如果在處理期間出現問題，您可以立即回溯到最後的安全狀態並跳出迴圈。

執行此範例前，請將 `your_revert_condition`（Python）或 `yourRevertCondition`（TypeScript）替換為您自己的檢查，例如錯誤偵測或驗證失敗；此範例中未定義預留位置。

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
  多個還原點
</h3>

如果 Claude 在多個轉向中進行變更，您可能想要回溯到特定點而不是一直回到開始。例如，如果 Claude 在第一個轉向中重構檔案，在第二個轉向中新增測試，您可能想要保留重構但撤銷測試。

此模式將所有 checkpoint UUID 儲存在具有中繼資料的陣列中。工作階段完成後，您可以回溯到任何先前的 checkpoint：

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
  試試看
</h2>

此完整範例建立一個小型公用程式檔案，讓代理程式新增文件註解，向您顯示變更，然後詢問您是否想要回溯。

開始之前，請確保您已[安裝 Claude Agent SDK](/docs/zh-TW/agent-sdk/quickstart)。

<Steps>
  <Step title="建立測試檔案">
    建立一個名為 `utils.py`（Python）或 `utils.ts`（TypeScript）的新檔案，並貼上以下程式碼：

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

  <Step title="執行互動式範例">
    在與您的公用程式檔案相同的目錄中建立一個名為 `try_checkpointing.py`（Python）或 `try_checkpointing.ts`（TypeScript）的新檔案，並貼上以下程式碼。

    此指令碼要求 Claude 將文件註解新增到您的公用程式檔案，然後為您提供回溯和還原原始檔案的選項。

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

    此範例演示完整的 checkpointing 工作流程：

    1. **啟用 checkpointing**：使用 `enable_file_checkpointing=True` 和 `permission_mode="acceptEdits"` 配置 SDK 以自動批准檔案編輯
    2. **捕捉 checkpoint 資料**：當代理程式執行時，儲存第一個使用者訊息 UUID（您的還原點）和工作階段 ID
    3. **提示回溯**：代理程式完成後，檢查您的公用程式檔案以查看文件註解，然後決定是否要撤銷變更
    4. **恢復和回溯**：如果是，請使用空提示恢復工作階段，並呼叫 `rewind_files()` 以還原原始檔案
  </Step>

  <Step title="執行範例">
    從與您的公用程式檔案相同的目錄執行指令碼。

    <Tip>
      在執行指令碼之前，在您的 IDE 或編輯器中開啟您的公用程式檔案（`utils.py` 或 `utils.ts`）。當代理程式新增文件註解時，您會看到檔案即時更新，然後當您選擇回溯時回復到原始檔案。
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

    您會看到代理程式新增文件註解，然後出現一個提示，詢問您是否想要回溯。如果您選擇是，檔案會還原到其原始狀態。
  </Step>
</Steps>

<h2 id="limitations">
  限制
</h2>

檔案 checkpointing 有以下限制：

| 限制                            | 描述                       |
| ----------------------------- | ------------------------ |
| 僅限 Write/Edit/NotebookEdit 工具 | 透過 Bash 命令所做的變更不會被追蹤     |
| 相同工作階段                        | Checkpoint 與建立它們的工作階段相關聯 |
| 僅限檔案內容                        | 建立、移動或刪除目錄不會透過回溯來撤銷      |
| 本機檔案                          | 遠端或網路檔案不會被追蹤             |

<h2 id="troubleshooting">
  疑難排解
</h2>

<h3 id="checkpointing-options-not-recognized">
  Checkpointing 選項無法識別
</h3>

如果 `enableFileCheckpointing` 或 `rewindFiles()` 無法使用，您可能使用的是較舊的 SDK 版本。

**解決方案**：更新到最新的 SDK 版本：

* **Python**：`pip install --upgrade claude-agent-sdk`
* **TypeScript**：`npm install @anthropic-ai/claude-agent-sdk@latest`

<h3 id="user-messages-don’t-have-uuids">
  使用者訊息沒有 UUID
</h3>

如果 `message.uuid` 是 `undefined` 或遺失，您沒有接收 checkpoint UUID。

**原因**：未設定 `replay-user-messages` 選項。

**解決方案**：將 `extra_args={"replay-user-messages": None}`（Python）或 `extraArgs: { 'replay-user-messages': null }`（TypeScript）新增到您的選項。

<h3 id="no-file-checkpoint-found-for-message-error">
  "No file checkpoint found for message" 錯誤
</h3>

當指定的使用者訊息 UUID 的 checkpoint 資料不存在時，會發生此錯誤。

**常見原因**：

* 檔案 checkpointing 未在原始工作階段上啟用（`enable_file_checkpointing` 或 `enableFileCheckpointing` 未設定為 `true`）
* 在嘗試恢復和回溯之前，工作階段未正確完成

**解決方案**：確保在原始工作階段上設定了 `enable_file_checkpointing=True`（Python）或 `enableFileCheckpointing: true`（TypeScript），然後使用範例中顯示的模式：捕捉第一個使用者訊息 UUID，完全完成工作階段，然後使用空提示恢復並呼叫 `rewindFiles()` 一次。

<h3 id="file-rewinding-is-not-enabled-error">
  "File rewinding is not enabled" 錯誤
</h3>

當您嘗試在未啟用 checkpointing 的情況下執行非互動式回溯時，會發生此錯誤：執行裸露的 `claude -p` 搭配 `--rewind-files`，或執行 SDK 工作階段（包括已恢復的工作階段），其選項未啟用 checkpointing。SDK 僅在執行回溯的工作階段上啟用 `enable_file_checkpointing`（Python）或 `enableFileCheckpointing`（TypeScript）時，才會在內部設定 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 環境變數；裸露 CLI 永遠不會設定它。

**解決方案**：對於裸露 CLI，在執行命令時設定環境變數：

```bash theme={null}
CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
```

對於 SDK，在已恢復的工作階段上設定 `enable_file_checkpointing=True`（Python）或 `enableFileCheckpointing: true`（TypeScript），如本頁面的範例所示。

<h3 id="processtransport-is-not-ready-for-writing-error">
  "ProcessTransport is not ready for writing" 錯誤
</h3>

當您在完成回應迭代後呼叫 `rewindFiles()` 或 `rewind_files()` 時，會發生此錯誤。當迴圈完成時，與 CLI 程序的連線會關閉。

**解決方案**：使用空提示恢復工作階段，然後在新查詢上呼叫回溯：

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
  後續步驟
</h2>

* **[Sessions](/docs/zh-TW/agent-sdk/sessions)**：了解如何恢復工作階段，這是在串流完成後回溯所需的。涵蓋工作階段 ID、恢復對話和工作階段分叉。
* **[Permissions](/docs/zh-TW/agent-sdk/permissions)**：配置 Claude 可以使用哪些工具以及如何批准檔案修改。如果您想更好地控制何時進行編輯，這很有用。
* **[TypeScript SDK reference](/docs/zh-TW/agent-sdk/typescript)**：完整的 API 參考，包括 `query()` 的所有選項和 `rewindFiles()` 方法。
* **[Python SDK reference](/docs/zh-TW/agent-sdk/python)**：完整的 API 參考，包括 `ClaudeAgentOptions` 的所有選項和 `rewind_files()` 方法。
