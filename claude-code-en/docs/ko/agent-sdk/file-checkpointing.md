> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 체크포인팅으로 파일 변경 사항 되돌리기

> 에이전트 세션 중 파일 변경 사항을 추적하고 파일을 이전의 모든 상태로 복원합니다

파일 체크포인팅은 에이전트 세션 중 Write, Edit, NotebookEdit 도구를 통해 수행된 파일 수정 사항을 추적하여 파일을 이전의 모든 상태로 되돌릴 수 있게 합니다. 직접 시도해보고 싶으신가요? [대화형 예제](#try-it-out)로 이동하세요.

체크포인팅을 사용하면 다음을 수행할 수 있습니다:

* **원치 않는 변경 사항 실행 취소** - 파일을 알려진 양호한 상태로 복원
* **대안 탐색** - 체크포인트로 복원한 후 다른 접근 방식 시도
* **오류 복구** - 에이전트가 잘못된 수정을 수행했을 때

<Warning>
  Write, Edit, NotebookEdit 도구를 통해 수행된 변경 사항만 추적됩니다. Bash 명령어(예: `echo > file.txt` 또는 `sed -i`)를 통해 수행된 변경 사항은 체크포인트 시스템에서 캡처되지 않습니다.
</Warning>

<h2 id="how-checkpointing-works">
  체크포인팅 작동 방식
</h2>

파일 체크포인팅을 활성화하면 SDK는 Write, Edit 또는 NotebookEdit 도구를 통해 파일을 수정하기 전에 파일의 백업을 생성합니다. 응답 스트림의 사용자 메시지에는 복원 지점으로 사용할 수 있는 체크포인트 UUID가 포함됩니다.

체크포인트는 에이전트가 파일을 수정하는 데 사용하는 다음의 기본 제공 도구와 함께 작동합니다:

| 도구           | 설명                                 |
| ------------ | ---------------------------------- |
| Write        | 새 파일을 생성하거나 기존 파일을 새 콘텐츠로 덮어씁니다    |
| Edit         | 기존 파일의 특정 부분에 대한 대상 편집을 수행합니다      |
| NotebookEdit | Jupyter 노트북(`.ipynb` 파일)의 셀을 수정합니다 |

<Note>
  파일 되돌리기는 디스크의 파일을 이전 상태로 복원합니다. 대화 자체를 되돌리지는 않습니다. `rewindFiles()`(TypeScript) 또는 `rewind_files()`(Python)를 호출한 후에도 대화 기록과 컨텍스트는 그대로 유지됩니다.
</Note>

체크포인트 시스템은 다음을 추적합니다:

* 세션 중에 생성된 파일
* 세션 중에 수정된 파일
* 수정된 파일의 원본 콘텐츠

체크포인트로 되돌리면 생성된 파일은 삭제되고 수정된 파일은 해당 시점의 콘텐츠로 복원됩니다.

<h2 id="implement-checkpointing">
  체크포인팅 구현
</h2>

파일 체크포인팅을 사용하려면 옵션에서 활성화하고, 응답 스트림에서 체크포인트 UUID를 캡처한 다음, 복원이 필요할 때 `rewindFiles()`(TypeScript) 또는 `rewind_files()`(Python)를 호출합니다.

다음 예제는 전체 흐름을 보여줍니다: 체크포인팅을 활성화하고, 응답 스트림에서 체크포인트 UUID와 세션 ID를 캡처한 다음, 나중에 세션을 재개하여 파일을 되돌립니다. 각 단계는 아래에서 자세히 설명됩니다. 이 섹션의 예제는 "인증 모듈 리팩토링"이라는 프롬프트를 사용합니다. 인증 모듈을 포함하는 프로젝트에서 실행하거나, 프롬프트를 변경하여 프로젝트에 존재하는 파일을 지정하면 파일 변경을 확인하고 되돌리기가 파일을 복원하는 것을 볼 수 있습니다.

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
  <Step title="체크포인팅 활성화">
    체크포인팅을 활성화하고 체크포인트 UUID를 수신하도록 SDK 옵션을 구성합니다:

    | 옵션            | Python                                      | TypeScript                                    | 설명                               |
    | ------------- | ------------------------------------------- | --------------------------------------------- | -------------------------------- |
    | 체크포인팅 활성화     | `enable_file_checkpointing=True`            | `enableFileCheckpointing: true`               | 되돌리기 위해 파일 변경 사항을 추적합니다          |
    | 체크포인트 UUID 수신 | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | 스트림에서 사용자 메시지 UUID를 가져오는 데 필요합니다 |

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

  <Step title="체크포인트 UUID 및 세션 ID 캡처">
    `replay-user-messages` 옵션이 설정되면(위에 표시됨), 응답 스트림의 각 사용자 메시지에는 체크포인트로 사용되는 UUID가 있습니다.

    대부분의 사용 사례에서 첫 번째 사용자 메시지 UUID(`message.uuid`)를 캡처합니다. 이로 되돌리면 모든 파일이 원본 상태로 복원됩니다. 여러 체크포인트를 저장하고 중간 상태로 되돌리려면 [여러 복원 지점](#multiple-restore-points)을 참조하세요.

    세션 ID(`message.session_id`)를 캡처하는 것은 선택 사항입니다. 스트림이 완료된 후 나중에 되돌리려는 경우에만 필요합니다. [체크포인트 전에 위험한 작업](#checkpoint-before-risky-operations) 예제처럼 메시지를 처리하는 동안 `rewindFiles()`를 즉시 호출하는 경우 세션 ID 캡처를 건너뛸 수 있습니다.

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

  <Step title="파일 되돌리기">
    스트림이 완료된 후 되돌리려면 빈 프롬프트로 세션을 재개하고 체크포인트 UUID와 함께 `rewind_files()`(Python) 또는 `rewindFiles()`(TypeScript)를 호출합니다. 스트림 중에도 되돌릴 수 있습니다. [체크포인트 전에 위험한 작업](#checkpoint-before-risky-operations)에서 해당 패턴을 참조하세요.

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

    세션 ID와 체크포인트 ID를 캡처한 경우 CLI에서도 되돌릴 수 있습니다. 이 명령은 [Claude Code 설치](/docs/ko/setup)에서 제공되는 `claude` 실행 파일이 필요하며 SDK 패키지에는 설치되지 않습니다. SDK는 체크포인팅을 활성화하지만, `claude -p`를 직접 실행할 때는 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 환경 변수를 설정해야 합니다:

    ```bash theme={null}
    CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
    ```

    `--rewind-files` 플래그는 `claude --help` 출력에 나타나지 않지만 CLI는 표시된 대로 이를 허용합니다.
  </Step>
</Steps>

<h2 id="common-patterns">
  일반적인 패턴
</h2>

이러한 패턴은 사용 사례에 따라 체크포인트 UUID를 캡처하고 사용하는 다양한 방법을 보여줍니다.

<h3 id="checkpoint-before-risky-operations">
  위험한 작업 전에 체크포인트
</h3>

이 패턴은 가장 최근의 체크포인트 UUID만 유지하며, 각 에이전트 턴 전에 업데이트합니다. 처리 중에 문제가 발생하면 마지막 안전한 상태로 즉시 되돌리고 루프를 벗어날 수 있습니다.

이 예제를 실행하기 전에 `your_revert_condition`(Python) 또는 `yourRevertCondition`(TypeScript)을 오류 감지 또는 유효성 검사 실패와 같은 자신의 확인으로 바꾸십시오. 플레이스홀더는 예제에서 정의되지 않습니다.

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
  여러 복원 지점
</h3>

Claude가 여러 턴에 걸쳐 변경을 수행하는 경우, 모든 방식으로 되돌리기보다는 특정 지점으로 되돌리고 싶을 수 있습니다. 예를 들어, Claude가 첫 번째 턴에서 파일을 리팩토링하고 두 번째 턴에서 테스트를 추가하는 경우, 리팩토링은 유지하되 테스트는 실행 취소하고 싶을 수 있습니다.

이 패턴은 모든 체크포인트 UUID를 메타데이터와 함께 배열에 저장합니다. 세션이 완료된 후 이전의 모든 체크포인트로 되돌릴 수 있습니다:

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
  직접 시도해보기
</h2>

이 완전한 예제는 작은 유틸리티 파일을 생성하고, 에이전트가 문서 주석을 추가하도록 하며, 변경 사항을 표시한 다음, 되돌리고 싶은지 묻습니다.

시작하기 전에 [Claude Agent SDK가 설치](/docs/ko/agent-sdk/quickstart)되어 있는지 확인하세요.

<Steps>
  <Step title="테스트 파일 생성">
    `utils.py`(Python) 또는 `utils.ts`(TypeScript)라는 새 파일을 생성하고 다음 코드를 붙여넣습니다:

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

  <Step title="대화형 예제 실행">
    유틸리티 파일과 같은 디렉토리에 `try_checkpointing.py`(Python) 또는 `try_checkpointing.ts`(TypeScript)라는 새 파일을 생성하고 다음 코드를 붙여넣습니다.

    이 스크립트는 Claude에게 유틸리티 파일에 문서 주석을 추가하도록 요청한 다음 원본을 복원하기 위해 되돌리고 싶은지 선택할 수 있는 옵션을 제공합니다.

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

    이 예제는 완전한 체크포인팅 워크플로우를 보여줍니다:

    1. **체크포인팅 활성화**: `enable_file_checkpointing=True` 및 `permission_mode="acceptEdits"`로 SDK를 구성하여 파일 편집을 자동으로 승인합니다
    2. **체크포인트 데이터 캡처**: 에이전트가 실행되는 동안 첫 번째 사용자 메시지 UUID(복원 지점) 및 세션 ID를 저장합니다
    3. **되돌리기 프롬프트**: 에이전트가 완료된 후 유틸리티 파일을 확인하여 문서 주석을 보고 변경 사항을 실행 취소할지 결정합니다
    4. **재개 및 되돌리기**: 예인 경우 빈 프롬프트로 세션을 재개하고 `rewind_files()`를 호출하여 원본 파일을 복원합니다
  </Step>

  <Step title="예제 실행">
    유틸리티 파일과 같은 디렉토리에서 스크립트를 실행합니다.

    <Tip>
      스크립트를 실행하기 전에 IDE 또는 편집기에서 유틸리티 파일(`utils.py` 또는 `utils.ts`)을 엽니다. 에이전트가 문서 주석을 추가할 때 파일이 실시간으로 업데이트되는 것을 볼 수 있으며, 되돌리기를 선택하면 원본으로 되돌아갑니다.
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

    에이전트가 문서 주석을 추가한 다음 되돌리고 싶은지 묻는 프롬프트가 표시됩니다. 예를 선택하면 파일이 원본 상태로 복원됩니다.
  </Step>
</Steps>

<h2 id="limitations">
  제한 사항
</h2>

파일 체크포인팅에는 다음과 같은 제한 사항이 있습니다:

| 제한 사항                       | 설명                                    |
| --------------------------- | ------------------------------------- |
| Write/Edit/NotebookEdit 도구만 | Bash 명령어를 통해 수행된 변경 사항은 추적되지 않습니다     |
| 동일한 세션                      | 체크포인트는 이를 생성한 세션에 연결됩니다               |
| 파일 콘텐츠만                     | 디렉토리 생성, 이동 또는 삭제는 되돌리기로 실행 취소되지 않습니다 |
| 로컬 파일                       | 원격 또는 네트워크 파일은 추적되지 않습니다              |

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="checkpointing-options-not-recognized">
  체크포인팅 옵션이 인식되지 않음
</h3>

`enableFileCheckpointing` 또는 `rewindFiles()`를 사용할 수 없는 경우 이전 SDK 버전을 사용 중일 수 있습니다.

**해결책**: 최신 SDK 버전으로 업데이트합니다:

* **Python**: `pip install --upgrade claude-agent-sdk`
* **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

<h3 id="user-messages-don’t-have-uuids">
  사용자 메시지에 UUID가 없음
</h3>

`message.uuid`가 `undefined`이거나 누락된 경우 체크포인트 UUID를 수신하지 못하고 있습니다.

**원인**: `replay-user-messages` 옵션이 설정되지 않았습니다.

**해결책**: 옵션에 `extra_args={"replay-user-messages": None}`(Python) 또는 `extraArgs: { 'replay-user-messages': null }`(TypeScript)을 추가합니다.

<h3 id="no-file-checkpoint-found-for-message-error">
  "No file checkpoint found for message" 오류
</h3>

이 오류는 지정된 사용자 메시지 UUID에 대한 체크포인트 데이터가 없을 때 발생합니다.

**일반적인 원인**:

* 원본 세션에서 파일 체크포인팅이 활성화되지 않았습니다(`enable_file_checkpointing` 또는 `enableFileCheckpointing`이 `true`로 설정되지 않음)
* 재개 및 되돌리기를 시도하기 전에 세션이 제대로 완료되지 않았습니다

**해결책**: 원본 세션에서 `enable_file_checkpointing=True`(Python) 또는 `enableFileCheckpointing: true`(TypeScript)가 설정되었는지 확인한 다음, 예제에 표시된 패턴을 사용합니다: 첫 번째 사용자 메시지 UUID를 캡처하고, 세션을 완전히 완료한 다음, 빈 프롬프트로 재개하고 `rewindFiles()`를 한 번 호출합니다.

<h3 id="file-rewinding-is-not-enabled-error">
  "File rewinding is not enabled" 오류
</h3>

이 오류는 체크포인팅이 활성화되지 않은 상태에서 비대화형 되돌리기를 시도할 때 발생합니다: `--rewind-files`를 사용하여 bare `claude -p`를 실행하거나, 체크포인팅을 활성화하지 않는 옵션이 있는 재개된 세션을 포함한 SDK 세션을 실행하는 경우입니다. SDK는 `enable_file_checkpointing`(Python) 또는 `enableFileCheckpointing`(TypeScript)이 되돌리기를 수행하는 세션에서 활성화될 때만 내부적으로 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 환경 변수를 설정합니다. bare CLI는 절대 설정하지 않습니다.

**해결책**: bare CLI의 경우 명령을 실행할 때 환경 변수를 설정합니다:

```bash theme={null}
CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
```

SDK의 경우, 이 페이지의 예제에서 수행하는 것처럼 재개된 세션에서 `enable_file_checkpointing=True`(Python) 또는 `enableFileCheckpointing: true`(TypeScript)를 설정합니다.

<h3 id="processtransport-is-not-ready-for-writing-error">
  "ProcessTransport is not ready for writing" 오류
</h3>

이 오류는 응답을 반복하는 것을 완료한 후 `rewindFiles()` 또는 `rewind_files()`를 호출할 때 발생합니다. 루프가 완료되면 CLI 프로세스에 대한 연결이 닫힙니다.

**해결책**: 빈 프롬프트로 세션을 재개한 다음 새 쿼리에서 되돌리기를 호출합니다:

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
  다음 단계
</h2>

* **[세션](/docs/ko/agent-sdk/sessions)**: 세션을 재개하는 방법을 알아봅니다. 이는 스트림이 완료된 후 되돌리기에 필요합니다. 세션 ID, 대화 재개 및 세션 포킹을 다룹니다.
* **[권한](/docs/ko/agent-sdk/permissions)**: Claude가 사용할 수 있는 도구와 파일 수정이 승인되는 방식을 구성합니다. 편집이 발생하는 시기를 더 많이 제어하려는 경우 유용합니다.
* **[TypeScript SDK 참조](/docs/ko/agent-sdk/typescript)**: `query()` 및 `rewindFiles()` 메서드의 모든 옵션을 포함한 완전한 API 참조입니다.
* **[Python SDK 참조](/docs/ko/agent-sdk/python)**: `ClaudeAgentOptions` 및 `rewind_files()` 메서드의 모든 옵션을 포함한 완전한 API 참조입니다.
