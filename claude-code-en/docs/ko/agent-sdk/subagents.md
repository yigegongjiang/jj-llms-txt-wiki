> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK의 서브에이전트

> Claude Agent SDK 애플리케이션에서 서브에이전트를 정의하고 호출하여 컨텍스트를 격리하고, 작업을 병렬로 실행하며, 메인 에이전트의 프롬프트에 추가하지 않고 특화된 지시사항을 적용합니다.

서브에이전트는 메인 에이전트가 초점이 맞춰진 부분 작업을 처리하기 위해 생성할 수 있는 별도의 에이전트 인스턴스입니다.
이를 사용하여 컨텍스트를 격리하고, 여러 분석을 병렬로 실행하며, 메인 에이전트의 프롬프트에 추가하지 않고 특화된 지시사항을 적용할 수 있습니다.

<h2 id="overview">
  개요
</h2>

세 가지 방법으로 서브에이전트를 생성할 수 있습니다:

* **프로그래매틱 방식**: `query()` 옵션에서 `agents` 매개변수를 사용합니다. [TypeScript](/docs/ko/agent-sdk/typescript#agentdefinition) 및 [Python](/docs/ko/agent-sdk/python#agentdefinition) 참조를 확인하세요
* **파일 시스템 기반**: `.claude/agents/` 디렉토리에 마크다운 파일로 에이전트를 정의합니다. [파일로 서브에이전트 정의하기](/docs/ko/sub-agents)를 참조하세요
* **기본 제공 범용**: Claude는 사용자가 아무것도 정의하지 않아도 Agent 도구를 통해 언제든지 기본 제공 `general-purpose` 서브에이전트를 호출할 수 있습니다

이 가이드는 SDK 애플리케이션에 권장되는 프로그래매틱 접근 방식에 중점을 둡니다.

<h2 id="benefits-of-using-subagents">
  서브에이전트 사용의 이점
</h2>

서브에이전트는 별도의 에이전트 인스턴스이므로, 작업을 위임하면 네 가지 이점을 얻습니다:

* **컨텍스트 격리**: 각 서브에이전트는 자체 대화에서 실행되며, 서브에이전트가 [포크](/docs/ko/sub-agents#fork-the-current-conversation)가 아닌 한 새로 시작됩니다. 어느 쪽이든 중간 도구 호출 및 결과는 서브에이전트 내부에 유지되며, 최종 메시지만 부모에게 반환됩니다. `research-assistant` 서브에이전트는 수십 개의 파일을 탐색할 수 있지만, 해당 콘텐츠는 주 대화에 누적되지 않습니다. 부모는 서브에이전트가 읽은 모든 파일이 아닌 간결한 요약을 받습니다. 서브에이전트의 컨텍스트에 정확히 무엇이 포함되는지는 [서브에이전트가 상속하는 것](#what-subagents-inherit)을 참조하십시오.
* **병렬화**: 여러 서브에이전트가 동시에 실행될 수 있으므로, 독립적인 부작업은 모든 작업의 합이 아닌 가장 느린 작업의 시간에 완료됩니다. 코드 검토 중에 `style-checker`, `security-scanner`, `test-coverage` 서브에이전트를 순차적으로 실행하는 대신 동시에 실행할 수 있습니다.
* **특화된 지침 및 지식**: 각 서브에이전트는 특정 전문 지식, 모범 사례 및 제약 조건이 포함된 맞춤형 시스템 프롬프트를 가질 수 있습니다. `database-migration` 서브에이전트는 SQL 모범 사례, 롤백 전략 및 데이터 무결성 검사에 대한 상세한 지식을 가질 수 있으며, 이는 주 에이전트의 지침에서는 불필요한 노이즈가 될 것입니다.
* **도구 제한**: 서브에이전트는 특정 도구로 제한될 수 있으므로, 의도하지 않은 작업의 위험을 줄입니다. `doc-reviewer` 서브에이전트는 Read 및 Grep 도구에만 액세스할 수 있으므로, 문서 파일을 분석할 수 있지만 실수로 수정할 수 없습니다.

<h2 id="create-subagents">
  서브에이전트 생성
</h2>

<h3 id="programmatic-definition-recommended">
  프로그래밍 방식 정의 (권장)
</h3>

`agents` 매개변수를 사용하여 코드에서 직접 서브에이전트를 정의합니다. Claude는 `Agent` 도구를 통해 서브에이전트를 호출합니다.

이 페이지의 대부분의 예제는 최종 결과만 출력합니다. Claude가 직접 답변하지 않고 서브에이전트에 위임했는지 확인하려면 [서브에이전트 호출 감지](#detect-subagent-invocation)를 참조하십시오.

이 예제는 두 개의 서브에이전트를 생성합니다: 읽기 전용 액세스 권한이 있는 코드 검토자와 명령을 실행할 수 있는 테스트 실행자입니다.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  AgentDefinition 구성
</h3>

| 필드                | 유형                                                          | 필수  | 설명                                                                                                                                                                                                                                             |
| :---------------- | :---------------------------------------------------------- | :-- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | 예   | 이 에이전트를 사용할 시기를 설명하는 자연어 설명                                                                                                                                                                                                                    |
| `prompt`          | `string`                                                    | 예   | 에이전트의 역할과 동작을 정의하는 시스템 프롬프트                                                                                                                                                                                                                    |
| `tools`           | `string[]`                                                  | 아니오 | 허용된 도구 이름의 배열입니다. 생략하면 [서브에이전트에서 사용 가능한 모든 도구](/docs/ko/sub-agents#available-tools)를 상속합니다                                                                                                                                                          |
| `disallowedTools` | `string[]`                                                  | 아니오 | 에이전트의 도구 집합에서 제거할 도구 이름의 배열입니다. MCP 서버 수준 패턴도 허용됩니다: `mcp__server` 또는 `mcp__server__*`는 해당 서버의 모든 도구를 제거하고, `mcp__*`는 모든 서버의 모든 MCP 도구를 제거합니다                                                                                                  |
| `model`           | `string`                                                    | 아니오 | 이 에이전트의 모델 재정의입니다. `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'` 같은 별칭 또는 전체 모델 ID를 허용합니다. `'inherit'`는 주 모델을 사용합니다. 생략하면 Claude Code는 [서브에이전트 모델 순서](/docs/ko/sub-agents#choose-a-model)에서 모델을 선택합니다                                     |
| `skills`          | `string[]`                                                  | 아니오 | 시작 시 에이전트의 컨텍스트에 미리 로드할 스킬 이름 목록입니다. 나열되지 않은 스킬은 Skill 도구를 통해 호출 가능합니다                                                                                                                                                                         |
| `memory`          | `'user' \| 'project' \| 'local'`                            | 아니오 | 이 에이전트의 메모리 소스                                                                                                                                                                                                                                 |
| `mcpServers`      | `(string \| object)[]`                                      | 아니오 | 이 에이전트에서 사용 가능한 MCP 서버(이름 또는 인라인 구성으로)                                                                                                                                                                                                         |
| `initialPrompt`   | `string`                                                    | 아니오 | 이 에이전트가 주 스레드 에이전트로 실행될 때 첫 번째 사용자 턴으로 자동 제출됩니다. 에이전트가 서브에이전트로 호출될 때는 무시됩니다                                                                                                                                                                    |
| `maxTurns`        | `number`                                                    | 아니오 | 에이전트가 중지되기 전의 최대 에이전트 턴 수입니다. 에이전트가 제한에 도달하면 Claude Code는 부분으로 표시된 출력을 반환하며, [에이전트를 재개](#resume-subagents)하여 계속할 수 있습니다. 부분 표시는 Claude Code v2.1.246 이상이 필요합니다                                                                                 |
| `background`      | `boolean`                                                   | 아니오 | 호출될 때 이 에이전트를 비차단 백그라운드 작업으로 실행합니다                                                                                                                                                                                                             |
| `omitClaudeMd`    | `boolean`                                                   | 아니오 | 이 에이전트가 서브에이전트로 실행될 때 사용자, 프로젝트 및 로컬 CLAUDE.md 파일 없이 실행합니다. 관리되는 정책 파일은 여전히 로드됩니다. 에이전트가 주 스레드 에이전트로 실행될 때는 무시됩니다. TypeScript Agent SDK v0.3.271 이상이 필요합니다. Python SDK의 [`AgentDefinition`](/docs/ko/agent-sdk/python#agentdefinition)에는 이 필드가 없습니다 |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | 아니오 | 이 에이전트의 추론 노력 수준                                                                                                                                                                                                                               |
| `permissionMode`  | `PermissionMode`                                            | 아니오 | 이 에이전트 내 도구 실행을 위한 권한 모드입니다. [서브에이전트 상속 규칙](/docs/ko/agent-sdk/permissions#available-modes)이 적용 시기를 결정합니다                                                                                                                                           |

Python SDK에서 `disallowedTools` 및 `mcpServers`와 같은 다중 단어 필드 이름은 Python의 snake\_case 규칙을 따르지 않고 와이어 형식과 일치하도록 camelCase 철자를 유지합니다. 자세한 내용은 [`AgentDefinition` 참조](/docs/ko/agent-sdk/python#agentdefinition)를 참조하십시오.

서브에이전트는 기본적으로 백그라운드에서 실행됩니다. [`run_in_background`](/docs/ko/sub-agents#run-subagents-in-foreground-or-background) 입력을 생략하는 Agent 도구 호출은 백그라운드 서브에이전트를 시작하며, Claude는 계속하기 전에 결과가 필요할 때 `run_in_background: false`를 설정합니다. 특정 에이전트에 대해 `background` 필드를 `true`로 설정하여 Claude가 요청하는 것과 관계없이 백그라운드 실행을 강제합니다. Claude Code v2.1.198 이전에는 백그라운드 기본값이 점진적으로 출시되었으며, `run_in_background`를 생략하는 Agent 도구 호출은 서브에이전트를 동기적으로 실행할 수 있었습니다.

서브에이전트는 자신의 서브에이전트를 생성할 수도 있습니다. 해당 중첩이 얼마나 깊은지, 한 번에 몇 개의 서브에이전트가 실행되는지, 쿼리가 얼마나 많이 소비하는지를 제한하려면 [서브에이전트 깊이, 동시성 및 지출 제한](#cap-subagent-depth-concurrency-and-spend)을 참조하십시오.

<h3 id="filesystem-based-definition-alternative">
  파일 시스템 기반 정의 (대안)
</h3>

`.claude/agents/` 디렉토리의 마크다운 파일로 서브에이전트를 정의할 수도 있습니다. 이 접근 방식에 대한 자세한 내용은 [Claude Code 서브에이전트 설명서](/docs/ko/sub-agents)를 참조하십시오. 프로그래밍 방식으로 정의된 에이전트는 같은 이름의 파일 시스템 기반 에이전트보다 우선합니다.

<Note>
  Claude가 `subagent_type` 없이 Agent 도구를 호출하면 기본 제공 `general-purpose` 서브에이전트를 가져오며, Claude는 자신의 에이전트를 정의하지 않은 경우에도 이를 생성할 수 있습니다. [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ko/env-vars)을 설정하면 해당 기본값이 제거되며, 이러한 호출은 [`subagent_type is required`](/docs/ko/errors#subagent-type-is-required) 오류로 실패합니다.
</Note>

<h2 id="what-subagents-inherit">
  서브에이전트가 상속하는 것
</h2>

서브에이전트가 [포크](/docs/ko/sub-agents#fork-the-current-conversation)가 아닌 경우, 그 컨텍스트 윈도우는 새로 시작되며 부모 대화가 없지만 비어있지는 않습니다. 부모에서 서브에이전트로 전달하는 유일한 콘텐츠는 Agent 도구의 프롬프트 문자열이므로, 서브에이전트가 필요로 하는 파일 경로, 오류 메시지 또는 결정 사항을 해당 프롬프트에 직접 포함시켜야 합니다.

[`SendMessage`](/docs/ko/tools-reference) 도구를 가진 서브에이전트는 세션에서 실행 중인 다른 명명된 에이전트 목록으로 시작하므로, 메시지를 보낼 수 있는 이름이 무엇인지 알 수 있습니다. Claude Code는 서브에이전트의 첫 번째 턴에 자동으로 목록을 추가합니다. [포크](/docs/ko/sub-agents#fork-the-current-conversation)는 부모 대화를 상속하기 때문에 목록을 받지 않습니다.

서브에이전트는 또한 메인 세션의 확장 사고 구성을 상속합니다.

아래 표는 포크가 아닌 서브에이전트의 컨텍스트에 포함되는 것과 제외되는 것을 나열합니다.

| 서브에이전트가 받는 것                                                                                                                                                                                    | 서브에이전트가 받지 않는 것                                    |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- |
| 자신의 시스템 프롬프트(`AgentDefinition.prompt`)와 Agent 도구의 프롬프트                                                                                                                                          | 부모의 대화 기록 또는 도구 결과                                 |
| Project CLAUDE.md ([`settingSources`](/docs/ko/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)를 통해 로드됨), [`omitClaudeMd`](#agentdefinition-configuration)를 설정하지 않는 한 | 미리 로드된 스킬 콘텐츠, `AgentDefinition.skills`에 나열된 경우 제외 |
| 도구 정의 (부모에서 상속되거나 `tools`의 부분집합, [백그라운드 실행을 위해 필터링됨](/docs/ko/sub-agents#available-tools))                                                                                                           | 부모의 시스템 프롬프트                                       |

<Note>
  부모는 서브에이전트의 최종 메시지를 Agent 도구 결과로 받지만, 자신의 응답에서 요약할 수 있습니다. 서브에이전트 출력을 사용자 대면 응답에 그대로 보존하려면, 메인 `query()` 호출에 전달하는 프롬프트 또는 `systemPrompt` 옵션에 그렇게 하도록 하는 지시사항을 포함시키십시오.

  v2.1.210 이상에서 Claude Code는 부모가 읽기 전에 [최종 메시지에서 지시사항 형태의 패턴을 스캔합니다](/docs/ko/sub-agents#subagent-output-scanning). 스캔은 세 가지 패턴 유형을 다르게 처리합니다:

  * **제어 태그 모방**: Claude Code는 `<system-reminder>` 블록과 같이 하네스만 내보내는 태그를 제자리에서 중립화합니다. 여는 꺾쇠 괄호 뒤에 백슬래시를 삽입하고 아무것도 삭제하지 않습니다.
  * **권한 구성 언급**: Claude Code는 `.claude/settings.json`, `bypassPermissions` 또는 `--dangerously-skip-permissions`와 같은 권한 구성에 대한 참조를 작성된 그대로 유지합니다.
  * **턴 마커**: `Human:` 또는 `Assistant:`로 시작하는 줄은 콜론 앞에 백슬래시를 받으므로 메시지가 대화 턴 경계를 모방할 수 없습니다.

  제어 태그 또는 권한 구성 일치의 경우, Claude Code는 일치된 패턴의 이름을 지정하는 `[harness: ...]` 마커 줄을 앞에 붙입니다. 턴 마커 일치는 마커 줄을 추가하지 않습니다. 이것이 스캔이 수행하는 유일한 수정 사항입니다. 서브에이전트의 텍스트를 제거하거나 다시 표현하지 않습니다.
</Note>

속도 제한과 같이 서브에이전트를 조기에 종료하는 API 오류는 결과로 전달되지 않습니다. [서브에이전트의 API 오류](/docs/ko/sub-agents#api-errors-in-subagents)에서 포그라운드 및 백그라운드 동작을 참조하십시오.

<h2 id="invoke-subagents">
  서브에이전트 호출
</h2>

<h3 id="automatic-invocation">
  자동 호출
</h3>

Claude는 작업과 각 서브에이전트의 `description`을 기반으로 서브에이전트를 호출할 시기를 자동으로 결정합니다. 예를 들어, "쿼리 튜닝을 위한 성능 최적화 전문가"라는 설명이 있는 `performance-optimizer` 서브에이전트를 정의하면, 프롬프트에서 쿼리 최적화를 언급할 때 Claude가 이를 호출합니다.

Claude가 작업을 올바른 서브에이전트와 일치시킬 수 있도록 명확하고 구체적인 설명을 작성하십시오.

<h3 id="explicit-invocation">
  명시적 호출
</h3>

Claude가 특정 서브에이전트를 사용하도록 보장하려면 프롬프트에서 이름으로 언급하십시오:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

이는 자동 일치를 우회하고 명명된 서브에이전트를 직접 호출합니다.

<h3 id="dynamic-agent-configuration">
  동적 에이전트 구성
</h3>

런타임 조건에 따라 에이전트 정의를 동적으로 생성할 수 있습니다. 이 예제는 다양한 엄격성 수준을 가진 보안 검토자를 생성하며, 엄격한 검토를 위해 더 강력한 모델을 사용합니다.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  서브에이전트 호출 감지
</h2>

Claude는 Agent 도구를 통해 서브에이전트를 호출합니다. 서브에이전트가 호출되는 시점을 감지하려면 `name`이 `"Agent"`인 `tool_use` 블록을 확인하면 됩니다. 서브에이전트의 컨텍스트 내에서 생성된 메시지에는 `parent_tool_use_id` 필드가 포함됩니다.

<Note>
  이 도구는 `tool_use` 블록에서는 `"Agent"`로 표시되지만 `system:init` 도구 목록에서는 `"Task"`로 표시됩니다. Claude Code v2.1.63 이전에는 `tool_use` 블록도 이를 `"Task"`로 명명했습니다. SDK 버전 간에 감지가 작동하도록 유지하려면 `block.name`에서 두 값을 모두 일치시키십시오.
</Note>

메시지 구조는 SDK마다 다릅니다. Python에서는 `message.content`를 통해 콘텐츠 블록에 직접 액세스합니다. TypeScript에서는 `SDKAssistantMessage`가 Claude API 메시지를 래핑하므로 `message.message.content`를 통해 콘텐츠에 액세스합니다.

이 예제는 스트리밍된 메시지를 반복하며 서브에이전트가 호출될 때와 후속 메시지가 해당 서브에이전트의 실행 컨텍스트 내에서 생성될 때를 기록합니다.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the code-reviewer agent to review this codebase",
    options: {
      allowedTools: ["Read", "Glob", "Grep", "Agent"],
      agents: {
        "code-reviewer": {
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  서브에이전트 재개
</h2>

서브에이전트를 재개하여 처음부터 시작하지 않고 중단된 지점에서 계속할 수 있습니다. 재개된 서브에이전트는 이전의 모든 도구 호출, 결과 및 추론을 포함한 전체 대화 기록을 유지합니다.

서브에이전트가 [`maxTurns`](#agentdefinition-configuration) 제한에 도달하여 중지되면, Claude Code는 Agent 도구 결과의 출력을 부분적으로 표시하여 Claude가 실행이 미완료임을 알 수 있도록 합니다.

서브에이전트가 완료되면, Agent 도구 결과에는 `agentId: <id>`를 포함하는 텍스트 블록이 포함됩니다. 기본 제공되는 [`Explore` 및 `Plan` 에이전트](/docs/ko/sub-agents#built-in-subagents)는 일회성이며 `agentId`를 반환하지 않으므로, 재개가 필요한 경우 사용자 정의 에이전트 또는 `general-purpose`를 사용하십시오. 서브에이전트를 프로그래밍 방식으로 재개하려면:

1. **세션 ID 캡처**: 첫 번째 쿼리 중에 메시지에서 `session_id` 추출
2. **에이전트 ID 추출**: Agent 도구 결과 텍스트에서 `agentId` 파싱
3. **세션 재개**: 두 번째 쿼리의 옵션에서 `resume: sessionId`를 전달하고, 프롬프트에 에이전트 ID를 포함합니다. 각 `query()` 호출은 기본적으로 새 세션을 시작하며, 서브에이전트의 기록에 액세스하려면 동일한 세션을 재개해야 합니다.

<Note>
  사용자 정의 에이전트를 사용할 때는 두 쿼리 모두에서 `agents` 매개변수에 동일한 에이전트 정의를 전달하십시오.
</Note>

아래 예제는 사용자 정의 `endpoint-finder` 에이전트를 정의합니다. 첫 번째 쿼리는 이를 실행하고 Agent 도구 결과에서 세션 ID와 에이전트 ID를 캡처한 다음, 두 번째 쿼리는 세션을 재개하여 첫 번째 분석의 컨텍스트가 필요한 후속 질문을 합니다.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

서브에이전트 기록은 별도의 파일에 저장되며 주 대화와 독립적으로 유지됩니다. 압축 동작 및 `cleanupPeriodDays` 정리 기간에 대해서는 [Claude Code에서 서브에이전트 재개](/docs/ko/sub-agents#resume-subagents)를 참조하십시오.

<h2 id="tool-restrictions">
  도구 제한
</h2>

`tools` 필드를 사용하여 서브에이전트가 수행할 수 있는 작업을 제한합니다:

* **`tools` 생략**: 서브에이전트는 [서브에이전트에서 사용 가능한 모든 도구](/docs/ko/sub-agents#available-tools)를 얻습니다
* **도구 나열**: 서브에이전트는 해당 도구만 얻습니다. 예를 들어 파일을 편집하면 안 되는 코드 검토자는 `["Read", "Grep", "Glob"]`을 얻습니다

생략한 도구는 서브에이전트의 세션에 전혀 포함되지 않습니다: Claude는 권한 프롬프트나 오류 없이 이를 제외하고 작동합니다.

이 예제는 코드를 검토할 수 있지만 파일을 수정하거나 명령을 실행할 수 없는 읽기 전용 분석 에이전트를 만듭니다.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
                  )
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  일반적인 도구 조합
</h3>

| 사용 사례    | 도구                                      | 설명                                       |
| :------- | :-------------------------------------- | :--------------------------------------- |
| 읽기 전용 분석 | `Read`, `Grep`, `Glob`                  | 코드를 검토할 수 있지만 수정하거나 실행할 수 없음             |
| 테스트 실행   | `Bash`, `Read`, `Grep`                  | 명령을 실행하고 출력을 분석할 수 있음                    |
| 코드 수정    | `Read`, `Edit`, `Write`, `Grep`, `Glob` | 명령 실행 없이 전체 읽기/쓰기 액세스                    |
| 전체 액세스   | 모든 도구                                   | 서브에이전트에서 사용 가능한 도구를 상속합니다(`tools` 필드 생략) |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  서브에이전트 깊이, 동시성 및 지출 제한
</h2>

<Note>
  이 섹션은 TypeScript SDK v0.3.219 및 Python SDK v0.2.127 이상, Claude Code v2.1.219 이상을 번들로 제공하는 릴리스를 설명합니다. 이전 릴리스에서는 이러한 제한 중 일부가 누락되었거나 기본값이 다르므로, 실행을 제한하기 위해 이에 의존하기 전에 업그레이드하십시오. [환경 변수 참조](/docs/ko/env-vars) 및 [턴 및 예산](/docs/ko/agent-sdk/agent-loop#turns-and-budget)에는 각 변수를 추가한 Claude Code 버전과 지출 상한의 서브에이전트 적용이 기록되어 있습니다.
</Note>

Claude는 서브에이전트를 언제 생성할지, 몇 개를 생성할지 자체적으로 결정합니다. 각 서브에이전트는 자체 API 요청을 수행하며, 이는 쿼리의 `total_cost_usd`에 계산되고, 서브에이전트는 자신의 서브에이전트를 생성할 수 있으므로 하나의 프롬프트가 에이전트 트리로 성장할 수 있습니다.

이러한 성장을 세 가지 방식으로 제한할 수 있습니다: 서브에이전트가 중첩되는 깊이, 한 번에 실행되는 개수, 그리고 전체 쿼리가 소비하는 금액입니다. [`env`](/docs/ko/agent-sdk/typescript#options) 옵션을 통해 환경 변수로 깊이 및 동시성 제한을 설정하고, 쿼리 옵션으로 지출 제한을 설정합니다:

| 제한  | 설정 방법                                                  | 기본값                                                                  | 제한에 도달했을 때 Claude Code의 동작                                                                                                                                                                                                            |
| :-- | :----------------------------------------------------- | :------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 깊이  | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/ko/env-vars) | 메인 에이전트 아래 `3`개 계층의 서브에이전트. `1`은 서브에이전트가 자신의 서브에이전트를 생성하지 못하도록 중지합니다 | 하단 계층의 서브에이전트가 생성할 수 없도록 남겨두므로 위임된 작업을 직접 수행합니다. [중첩된 서브에이전트](/docs/ko/sub-agents#let-subagents-spawn-their-own-subagents) 참조                                                                                                              |
| 동시성 | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/ko/env-vars) | `20`개의 서브에이전트가 동시에 실행되며, Agent 도구로 Claude가 생성하는 모든 서브에이전트를 계산합니다     | 실행 중인 개수가 제한 아래로 떨어질 때까지 `Concurrent subagent limit reached`를 반환하여 다른 서브에이전트 생성을 거부합니다. [ultracode](/docs/ko/model-config#adjust-effort-level)가 활성화된 세션은 거부되지 않습니다. [동시 서브에이전트 제한](/docs/ko/sub-agents#concurrent-subagent-limit) 참조            |
| 지출  | TypeScript의 `maxBudgetUsd`, Python의 `max_budget_usd`   | 제한 없음. 호출 자체의 지출을 계산하며, 서브에이전트 요청이 포함됩니다                             | 세 가지 방식으로 상한을 적용합니다: 더 이상의 서브에이전트 생성을 거부하여 `Budget limit reached`를 반환하고, 여전히 실행 중인 백그라운드 서브에이전트를 중지하며, `error_max_budget_usd` 결과 서브타입으로 쿼리를 종료합니다. 세션 전체에서 상한이 어떻게 작동하는지는 [턴 및 예산](/docs/ko/agent-sdk/agent-loop#turns-and-budget)을 참조하십시오 |

두 SDK는 `env` 옵션을 다르게 처리합니다: TypeScript SDK는 서브프로세스 환경을 이것으로 대체하므로 `PATH`와 같은 변수를 유지하기 위해 `process.env`를 이것으로 전개하고, Python SDK는 이것을 상속된 환경에 병합합니다. 이 예제는 중첩을 끄고, 한 번에 최대 5개의 서브에이전트를 허용하며, 예상 지출이 \$5에 도달하면 쿼리를 중지합니다:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

표시되는 내용은 쿼리가 도달하는 제한(있는 경우)에 따라 달라집니다:

* **지출 상한 이하**: `success`와 예상 비용이 표시됩니다.
* **지출 상한에서**: `error_max_budget_usd`가 `5` 이상의 비용과 함께 표시되고, 그 다음 오류 핸들러가 실행됩니다.
* **동시성 제한에서**: 메시지 스트림에서 `Concurrent subagent limit reached`를 전달하는 `tool_result` 블록이 표시됩니다. Claude는 Agent 도구의 결과로 동일한 블록을 수신합니다.

<h3 id="run-opus-5-with-subagents">
  Opus 5를 서브에이전트와 함께 실행
</h3>

Claude Opus 5는 이전 모델보다 더 쉽게 서브에이전트에 위임하므로, [깊이, 동시성 및 지출 제한](#cap-subagent-depth-concurrency-and-spend)은 Opus 5를 실행하는 쿼리에서 가장 중요합니다. [Opus 5 프롬프팅 가이드](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning)에는 모든 프롬프트에 추가할 수 있는 위임 지침이 있습니다. Claude Code가 자신의 지침을 추가하는지 여부는 사용하는 [시스템 프롬프트](/docs/ko/agent-sdk/modifying-system-prompts#how-system-prompts-work)에 따라 달라집니다:

* **`claude_code` 사전 설정**: 모델이 Opus 5일 때, Claude Code는 Claude에게 요청받지 않는 한 Agent 도구를 호출하지 말라고 지시하는 줄을 시스템 프롬프트에 추가합니다. Agent 도구는 계속 사용 가능합니다.
* **사용자 정의 프롬프트 또는 `systemPrompt` 없음**: Claude Code는 시스템 프롬프트를 구축하지 않으므로 해당 줄이 없습니다. 프롬프팅 가이드의 위임 지침을 자신의 프롬프트에 추가합니다.

어느 지침이든 Claude만 안내하므로 제한도 설정합니다. Claude Code는 Claude가 위임하기로 결정한 방식에 관계없이 이를 적용합니다.

<h2 id="scale-up-with-dynamic-workflows">
  동적 워크플로우로 확장
</h2>

서브에이전트는 턴당 몇 가지 위임된 작업에 적합합니다. 수십 개에서 수백 개의 에이전트를 조정하는 실행의 경우, `Workflow` 도구를 사용하세요. 이는 오케스트레이션을 대화 컨텍스트 외부에서 런타임이 실행하는 스크립트로 이동합니다. 워크플로우가 턴별 서브에이전트 위임과 어떻게 다른지는 [동적 워크플로우](/docs/ko/workflows)를 참조하세요.

`Workflow` 도구는 TypeScript Agent SDK v0.3.149 이상에서 사용 가능합니다. `allowedTools`에 `Workflow`를 포함하여 워크플로우 실행을 자동 승인합니다. 도구 입력 및 출력 스키마는 [TypeScript 참조](/docs/ko/agent-sdk/typescript#workflow)에 나열되어 있습니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude가 서브에이전트에 위임하지 않음
</h3>

Claude가 서브에이전트에 위임하지 않고 작업을 직접 완료하는 경우:

* **명시적 프롬프팅 사용**: 프롬프트에서 서브에이전트를 이름으로 언급합니다. 예를 들어 "code-reviewer 에이전트를 사용하여 인증 모듈을 확인하세요"
* **명확한 설명 작성**: 서브에이전트를 언제 사용할지 정확히 설명하여 Claude가 작업을 적절히 매칭할 수 있도록 합니다

<h3 id="filesystem-based-agents-not-loading">
  파일시스템 기반 에이전트가 로드되지 않음
</h3>

Claude Code는 `~/.claude/agents/` 및 `.claude/agents/`를 감시하며 새로운 또는 편집된 에이전트 파일을 몇 초 내에 선택합니다. 재시작이 필요하지 않습니다. 정의가 나타나지 않으면 다음 원인들을 확인하세요:

* **새로운 `agents` 디렉토리**: 감시자는 세션이 시작될 때 존재했던 디렉토리만 감시하므로 새 디렉토리의 첫 번째 파일은 세션 재시작이 필요합니다. 이것이 가장 일반적인 원인입니다.
* **잘못된 frontmatter 또는 중복된 `name`**: 파일의 YAML을 확인하고 기존 에이전트가 이미 해당 `name`을 사용하고 있는지 확인합니다.
* **`--disable-slash-commands`**: 이 플래그로 시작된 세션은 이러한 디렉토리를 감시하지 않으며 새 파일을 로드하려면 항상 재시작이 필요합니다.
* **추가된 디렉토리 아래의 파일**: Claude Code는 `add_dirs` (Python) 또는 `additionalDirectories` (TypeScript) 옵션이나 CLI의 `--add-dir` 또는 `/add-dir`로 추가된 디렉토리에서 `.claude/agents/`를 로드하지만 감시하지 않으므로 해당 위치의 새로운 또는 편집된 파일은 세션 재시작이 필요합니다.
* **동일한 이름의 프로그래매틱 에이전트**: `query()`에 전달된 `agents`는 동일한 이름의 파일시스템 에이전트를 재정의합니다.

파일 형식에 대해서는 [서브에이전트 파일 작성 방법](/docs/ko/sub-agents#write-subagent-files)을 참조하세요.

<h2 id="related-documentation">
  관련 문서
</h2>

* [Claude Code 서브에이전트](/docs/ko/sub-agents): 파일 시스템 기반 정의를 포함한 포괄적인 서브에이전트 문서
* [동적 워크플로우](/docs/ko/workflows): 한 대화에 너무 큰 작업을 위해 스크립트에서 많은 서브에이전트를 오케스트레이션합니다
* [SDK 개요](/docs/ko/agent-sdk/overview): Claude Agent SDK 시작하기
