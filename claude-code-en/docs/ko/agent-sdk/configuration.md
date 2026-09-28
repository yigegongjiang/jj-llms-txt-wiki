> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 에이전트 구성

> Agent SDK 세션 구성: 옵션 객체 작성, 모델 설정, 환경 및 제한 설정, 각 기능 옵션의 페이지 찾기.

Agent SDK 세션은 설정 파일, 환경 변수 및 세션을 시작할 때 전달하는 `options` 객체에서 구성을 읽습니다. 이 페이지에서는 `options` 객체를 작성하는 방법과 어떤 설정 파일 및 환경 변수가 제어하는지 보여줍니다.

모든 옵션의 타입과 기본값은 [`Options`](/docs/ko/agent-sdk/typescript#options) (TypeScript) 및 [`ClaudeAgentOptions`](/docs/ko/agent-sdk/python#claudeagentoptions) (Python) 참조를 확인하세요.

<h2 id="pass-options-to-a-session">
  세션에 옵션 전달
</h2>

모든 `query()` 호출은 옵션 객체를 허용합니다: TypeScript에서는 `Options`, Python에서는 `ClaudeAgentOptions`. 각 필드는 선택 사항이며, 옵션 없이 시작된 세션은 SDK의 기본값으로 실행됩니다. 아래 예제는 프로젝트의 열린 TODO를 요약하는 읽기 전용 세션을 구성합니다. 쌍은 철자가 다른 경우 TypeScript / Python로 읽습니다:

* **`model`**: 모델을 선택합니다
* **`allowedTools` / `allowed_tools`**: 읽기 전용 도구 목록을 사전 승인합니다
* **`maxTurns` / `max_turns`**: 턴 수를 제한합니다
* **`cwd`**: 작업 디렉토리를 설정합니다

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the open TODOs in this repo",
    options: {
      model: "claude-sonnet-5",
      allowedTools: ["Read", "Glob", "Grep"],
      maxTurns: 8,
      cwd: "/path/to/repo",
    },
  })) {
    if (message.type === "result" && message.subtype === "success" && !message.is_error) {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

  async def main():
      options = ClaudeAgentOptions(
          model="claude-sonnet-5",
          allowed_tools=["Read", "Glob", "Grep"],
          max_turns=8,
          cwd="/path/to/repo",
      )

      async for message in query(
          prompt="Summarize the open TODOs in this repo",
          options=options,
      ):
          if isinstance(message, ResultMessage) and not message.is_error:
              print(message.result)

  asyncio.run(main())
  ```
</CodeGroup>

`cwd`를 자신의 프로젝트 중 하나로 지정하고 예제를 실행하세요. 해당 프로젝트의 열린 TODO 요약이 결과 메시지가 도착할 때 출력됩니다.

`allowedTools` (TypeScript) 또는 `allowed_tools` (Python)는 나열된 도구를 사전 승인하므로 이들에 대한 호출은 승인을 기다리지 않고 실행됩니다. 목록 외의 도구는 계속 사용 가능합니다. Claude가 나열되지 않은 도구를 호출할 때 권한 모드는 호출 실행 여부를 결정합니다. 자세한 내용은 [허용 및 거부 규칙](/docs/ko/agent-sdk/permissions#allow-and-deny-rules)을 참조하세요.

<h2 id="load-settings-files">
  설정 파일 로드
</h2>

설정 파일은 옵션 객체 이상의 구성을 제공합니다. 두 가지 옵션이 로드 방식을 제어합니다:

* **`settingSources` / `setting_sources`**: 로드할 파일 시스템 소스를 제어합니다: 사용자, 프로젝트 및 로컬. 설정 파일 및 CLAUDE.md 파일은 이러한 소스를 통해 도착합니다.
* **`settings`**: 설정 파일 경로 또는 두 언어의 인라인 JSON 문자열을 로드하며, TypeScript는 설정 객체도 허용합니다. 전달하는 형식이 무엇이든 사용자, 프로젝트 및 로컬 파일 시스템 설정을 재정의합니다. 관리되는 정책 설정만 더 높은 순위를 가집니다. 참조는 TypeScript의 [설정 우선순위](/docs/ko/agent-sdk/typescript#settings-precedence) 및 Python의 [설정 우선순위](/docs/ko/agent-sdk/python#settings-precedence)에서 전체 우선순위 순서를 문서화합니다.

사용자, 프로젝트 및 로컬 설정을 비활성화하려면 `[]`를 전달하세요. 자세한 내용은 [SDK에서 Claude Code 기능 사용](/docs/ko/agent-sdk/claude-code-features)을 참조하세요.

<h2 id="choose-a-model">
  모델 선택
</h2>

`model` 옵션, 설정 또는 환경이 모델을 선택하지 않으면 새 세션이 [Claude Code의 기본 모델](/docs/ko/model-config#default-model-setting)에서 시작됩니다. 이러한 소스의 순서는 [모델 설정](/docs/ko/model-config#setting-your-model)을 참조하세요. 특정 모델을 고정하거나 더 빠르고 저렴한 에이전트를 위해 더 작은 모델을 선택하려면 `model`을 설정하세요. 값은 모델 별칭 또는 전체 모델 이름을 사용합니다. 별칭 및 이들이 해석되는 버전은 [모델 별칭](/docs/ko/model-config#model-aliases)에 나열되어 있습니다.

백업 모델을 지정하려면 `fallbackModel` (TypeScript) 또는 `fallback_model` (Python)을 설정하세요. 주 모델이 과부하 상태이거나 사용 불가능할 때 세션이 백업으로 전환됩니다. 주 모델은 각 사용자 턴의 시작 부분에서 다시 시도되므로 중단이 해결되면 세션이 주 모델로 돌아갑니다.

두 언어 모두에서 옵션은 단일 모델 또는 쉼표로 구분된 백업 목록을 허용합니다. 순서 및 체인 상한은 [폴백 모델 체인](/docs/ko/model-config#fallback-model-chains)을 참조하세요. TypeScript에서 `model`과 같은 폴백은 시작 시 오류를 발생시킵니다.

아래 예제는 TypeScript의 폴백 목록과 Python의 단일 폴백을 보여줍니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    model: "claude-fable-5",
    fallbackModel: "claude-opus-5,claude-sonnet-5",
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      model="claude-fable-5",
      fallback_model="claude-opus-5",
  )
  ```
</CodeGroup>

<span id="sampling-parameters" />

<Note>
  [Messages API](https://platform.claude.com/docs/en/api/messages) 요청 매개변수 `temperature`, `top_p` 및 `max_tokens`는 두 언어 모두에서 옵션 객체에 필드가 없습니다. 대신 [노력 수준](/docs/ko/agent-sdk/agent-loop#effort-level) 또는 [지출 상한](#limit-turns-and-spend)을 설정하거나, 이러한 매개변수가 필요할 때 Messages API를 호출하세요.
</Note>

<h2 id="set-environment-variables">
  환경 변수 설정
</h2>

`env` 옵션은 세션을 실행하는 Claude Code 프로세스에 대한 환경 변수를 설정합니다. 값이 상속된 환경을 대체하는지 병합하는지는 언어에 따라 다릅니다:

* **TypeScript**: `env`는 서브프로세스 환경을 대체합니다
* **Python**: SDK는 값을 상속된 환경에 병합하고 값이 상속된 값을 재정의합니다

TypeScript에서 `process.env`를 `env`에 전개하여 `PATH`, `HOME` 및 `ANTHROPIC_API_KEY`와 같은 상속된 변수를 유지하세요. `env`를 설정하지 않으면 서브프로세스는 두 언어 모두에서 환경을 상속합니다.

예제는 `ANTHROPIC_BASE_URL`을 설정하여 API 트래픽을 게이트웨이를 통해 라우팅합니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
  )
  ```
</CodeGroup>

전달하는 변수는 Claude Code 자체를 구성할 수도 있습니다. Claude Code 프로세스가 읽는 변수는 [환경 변수](/docs/ko/env-vars)를 참조하세요. API 타임아웃 및 정지 감지를 이 방식으로 조정하려면 [TypeScript 참조](/docs/ko/agent-sdk/typescript#handle-slow-or-stalled-api-responses) 또는 [Python 참조](/docs/ko/agent-sdk/python#handle-slow-or-stalled-api-responses)의 느린 또는 정지된 API 응답 처리 섹션을 따르세요.

<h2 id="set-the-working-directory">
  작업 디렉토리 설정
</h2>

`cwd`를 설정하여 특정 디렉토리에서 세션을 실행하세요. `cwd`를 설정하지 않으면 세션이 프로세스의 작업 디렉토리에서 실행됩니다. 두 SDK 모두 `cwd`에 대한 설정자가 없습니다. 다른 디렉토리에서 실행하려면 해당 `cwd`로 다른 세션을 시작하세요.

Claude Code는 작업 디렉토리를 읽어 다음을 결정합니다:

* **프로젝트 설정 및 훅**: 어느 프로젝트의 [설정 및 훅이 로드되는지](/docs/ko/agent-sdk/claude-code-features)
* **스킬**: [세션 스킬이 발견되는 위치](/docs/ko/agent-sdk/skills)
* **세션 저장소**: [저장된 세션이 어느 프로젝트에 속하는지](/docs/ko/agent-sdk/session-storage)

도구가 작업 디렉토리 외부의 파일에 도달하도록 하려면 `additionalDirectories` (TypeScript) 또는 `add_dirs` (Python)로 경로를 추가하세요. 해당 권한의 범위는 [추가 디렉토리는 파일 액세스를 부여하며 구성은 아님](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration)을 참조하세요.

<h2 id="limit-turns-and-spend">
  턴 및 지출 제한
</h2>

`maxTurns` / `max_turns` 및 `maxBudgetUsd` / `max_budget_usd`로 턴 및 지출을 제한하세요. 두 상한 모두 설정하지 않으면 꺼져 있습니다. 세션이 상한에 도달하면 실행이 서브타입이 상한을 지정하는 결과 메시지로 끝납니다: `error_max_turns` 또는 `error_max_budget_usd`. 다음에 일어나는 일은 입력 모드에 따라 다릅니다:

* **단일 샷 `query()`**: SDK는 상한 결과를 생성한 다음 발생시키므로 루프를 try 블록으로 래핑하여 오류를 지나 계속하세요
* **스트리밍 입력**: 세션은 상한 결과를 지나 살아있으며 최대 턴 수는 각 대기 중인 메시지에 대해 다시 시작됩니다. 예산 합계는 메시지 전체에 누적되며 지출이 상한에 도달하면 같은 대화의 나중 메시지는 같은 예산 결과로 끝납니다. [`/clear`](/docs/ko/agent-sdk/cost-tracking)는 예산을 다시 시작합니다

두 상한은 `0`을 다르게 처리합니다:

* **`maxTurns` / `max_turns`**: `0`은 턴 제한 없이 세션을 실행하며, 옵션을 설정하지 않은 것과 같습니다
* **`maxBudgetUsd` / `max_budget_usd`**: CLI는 시작 시 `0`을 유효하지 않은 금액으로 거부하며 세션은 실행되지 않습니다

서브에이전트 지출을 포함한 두 상한에 대한 자세한 내용은 [턴 및 예산](/docs/ko/agent-sdk/agent-loop#turns-and-budget)을 참조하세요.

<h2 id="change-configuration-mid-session">
  세션 중 구성 변경
</h2>

[스트리밍 입력](/docs/ko/agent-sdk/streaming-vs-single-mode)으로 세션을 시작할 때 실행 중에 모델 및 권한 모드를 전환할 수 있습니다. 설정자를 호출하는 위치는 언어에 따라 다릅니다:

* **TypeScript**: `query()`가 반환하는 객체의 메서드
* **Python**: [`ClaudeSDKClient`](/docs/ko/agent-sdk/python#claudesdkclient)의 메서드, `query()`는 제어 메서드 없이 일반 반복자를 반환하기 때문입니다

두 언어 모두 동일한 설정자를 가집니다:

* **`setModel()` / `set_model()`**: 모델을 전환합니다. 옵션에서 전달한 `model` 대신 [Claude Code의 기본 모델](/docs/ko/model-config#default-model-setting)로 전환하려면 모델 없이 호출하세요.
* **`setPermissionMode()` / `set_permission_mode()`**: 권한 모드를 전환합니다

TypeScript는 또한 `applyFlagSettings()` 및 `updateSettings()`를 가집니다:

* **`applyFlagSettings()`**: `await session.applyFlagSettings({ effortLevel: "high" })`처럼 런타임에 설정을 적용합니다. 메서드는 옵션 필드가 아닌 설정 파일 키를 사용하므로 스키마 및 어떤 키가 세션 중에 적용되는지 [`applyFlagSettings()` 참조](/docs/ko/agent-sdk/typescript#applyflagsettings)를 확인하세요.
* **`updateSettings()`**: 허용 목록에 있는 키 하나를 설정 파일에 씁니다. [`updateSettings()` 참조](/docs/ko/agent-sdk/typescript#updatesettings)는 각 소스가 허용하는 키와 버전 하한을 지정합니다.
  * 프로젝트의 로컬 설정 파일에 쓰려면 `"localSettings"`를 전달하세요. `await session.updateSettings("localSettings", { outputStyle: "Explanatory" })`처럼 사용합니다. 작성된 키는 세션의 다음 요청에서 적용되고 `local` 설정을 로드하는 나중 세션에 대해 지속됩니다.
  * `"userSettings"`를 전달하여 해당 소스가 허용하는 유일한 키인 `effortLevel`을 씁니다. Claude Code는 이를 세션의 현재 모델에 대한 기본 노력 수준으로 저장하며, 실행 중인 세션의 노력은 변경되지 않습니다.

아래 예제는 2턴 세션을 실행하고 턴 사이에 구성을 변경하며 각 턴에 응답한 모델을 출력합니다. TypeScript에서 프롬프트 스트림은 설정자가 실행될 때까지 두 번째 메시지를 보유하며 두 번째 턴은 새 모델에서 실행됩니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

  function userMessage(text: string): SDKUserMessage {
    return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
  }

  // Hold the second prompt until the setters have run.
  let startSecondTurn!: () => void;
  const secondTurnReady = new Promise<void>((resolve) => {
    startSecondTurn = resolve;
  });

  async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
    yield userMessage("Reply with exactly: ready");
    await secondTurnReady;
    yield userMessage("Reply with exactly: done");
  }

  const session = query({
    prompt: turnPrompts(),
    options: {
      model: "claude-sonnet-5",
    },
  });

  let turnModel = "";
  let completedTurns = 0;

  for await (const message of session) {
    if (message.type === "assistant") {
      turnModel = message.message.model;
    } else if (message.type === "result") {
      completedTurns += 1;
      if (completedTurns === 1) {
        console.log(`First turn model: ${turnModel}`);
        await session.setModel("claude-opus-5");
        await session.setPermissionMode("acceptEdits");
        startSecondTurn();
      } else {
        console.log(`Second turn model: ${turnModel}`);
        break;
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

  async def main():
      options = ClaudeAgentOptions(model="claude-sonnet-5")

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Reply with exactly: ready")
          first_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  first_model = message.model

          await client.set_model("claude-opus-5")
          await client.set_permission_mode("acceptEdits")

          await client.query("Reply with exactly: done")
          second_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  second_model = message.model

      print(f"First turn model: {first_model}")
      print(f"Second turn model: {second_model}")

  asyncio.run(main())
  ```
</CodeGroup>

Claude API에서 프로그램은 `First turn model: claude-sonnet-5`를 출력한 다음 전환 후 `Second turn model: claude-opus-5`를 출력합니다.

<Note>
  각 모델은 자체 프롬프트 캐시를 가지므로 세션 중 전환 후 다음 요청은 새 모델의 요금으로 전체 대화를 캐시되지 않은 상태로 다시 계산합니다. 자세한 내용은 [모델 전환](/docs/ko/prompt-caching#switching-models)을 참조하세요.
</Note>

<h2 id="configure-specific-features">
  특정 기능 구성
</h2>

아래 표는 각 옵션을 구성하는 기능에 매핑합니다. 이 페이지에서 다루지 않는 옵션은 [TypeScript](/docs/ko/agent-sdk/typescript#options) 및 [Python](/docs/ko/agent-sdk/python#claudeagentoptions) 참조를 참조하세요. 목표는 알지만 어떤 옵션이 이를 제공하는지 모르면 [올바른 기능 선택](/docs/ko/agent-sdk/claude-code-features#choose-the-right-feature)에서 시작하세요.

| TypeScript                | Python                      | 제어                   | 다루는 내용                                                                                                                                                                        |
| ------------------------- | --------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionMode`          | `permission_mode`           | 에이전트가 승인 없이 할 수 있는 것 | [권한 구성](/docs/ko/agent-sdk/permissions)                                                                                                                                            |
| `allowedTools`            | `allowed_tools`             | 어떤 도구 호출이 사전 승인되는지   | [권한 구성](/docs/ko/agent-sdk/permissions)                                                                                                                                            |
| `canUseTool`              | `can_use_tool`              | 도구 호출에 대한 승인 콜백      | [도구 승인 요청 처리](/docs/ko/agent-sdk/user-input#handle-tool-approval-requests)                                                                                                         |
| `systemPrompt`            | `system_prompt`             | 에이전트의 지시사항           | [시스템 프롬프트 수정](/docs/ko/agent-sdk/modifying-system-prompts)                                                                                                                         |
| `settingSources`          | `setting_sources`           | 로드할 파일 시스템 설정        | [SDK에서 Claude Code 기능 사용](/docs/ko/agent-sdk/claude-code-features)                                                                                                                 |
| `mcpServers`              | `mcp_servers`               | 외부 도구 서버             | [MCP로 외부 도구 연결](/docs/ko/agent-sdk/mcp)                                                                                                                                            |
| `agents`                  | `agents`                    | 서브에이전트 정의            | [서브에이전트](/docs/ko/agent-sdk/subagents)                                                                                                                                             |
| `hooks`                   | `hooks`                     | 라이프사이클 포인트의 콜백       | [훅](/docs/ko/agent-sdk/hooks)                                                                                                                                                      |
| `skills`                  | `skills`                    | 로드할 스킬               | [스킬로 에이전트 확장](/docs/ko/agent-sdk/skills)                                                                                                                                           |
| `plugins`                 | `plugins`                   | 로드할 플러그인             | [플러그인](/docs/ko/agent-sdk/plugins)                                                                                                                                                 |
| `outputFormat`            | `output_format`             | 구조화된 출력 스키마          | [구조화된 출력](/docs/ko/agent-sdk/structured-outputs)                                                                                                                                   |
| `resume`                  | `resume`                    | 저장된 세션 계속            | [세션](/docs/ko/agent-sdk/sessions)                                                                                                                                                  |
| `forkSession`             | `fork_session`              | 세션 분기                | [세션](/docs/ko/agent-sdk/sessions)                                                                                                                                                  |
| `sessionStore`            | `session_store`             | 외부 세션 지속성            | [세션 저장소](/docs/ko/agent-sdk/session-storage)                                                                                                                                       |
| `enableFileCheckpointing` | `enable_file_checkpointing` | 되감기 가능한 파일 편집        | [파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing)                                                                                                                                  |
| `effort`                  | `effort`                    | Claude가 응답에 투입하는 작업량 | [노력 수준](/docs/ko/agent-sdk/agent-loop#effort-level)                                                                                                                                |
| `sandbox`                 | `sandbox`                   | 도구 실행을 위한 샌드박스 동작    | [TypeScript](/docs/ko/agent-sdk/typescript#sandbox-configuration) 및 [Python](/docs/ko/agent-sdk/python#sandbox-configuration) 참조, [안전한 배포](/docs/ko/agent-sdk/secure-deployment)의 배포 컨텍스트 포함 |

<h2 id="next-steps">
  다음 단계
</h2>

구성이 작동하는 에이전트로 구성되는 것을 보려면:

* **[빠른 시작](/docs/ko/agent-sdk/quickstart)**: 첫 번째 에이전트를 처음부터 끝까지 구축하고 실행
* **[예제](/docs/ko/agent-sdk/examples)**: 구축하려는 것과 일치하는 완전하고 실행 가능한 프로젝트 또는 안내식 Claude Cookbook 레시피 찾기
* **[다중 테넌트 격리](/docs/ko/agent-sdk/hosting#multi-tenant-isolation)**: `settingSources` / `setting_sources`, `env` 및 `cwd`로 각 테넌트의 설정 및 메모리 격리
