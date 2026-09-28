> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 에이전트 루프의 작동 원리

> 메시지 생명주기, 도구 실행, 컨텍스트 윈도우, 그리고 SDK 에이전트를 구동하는 아키텍처를 이해합니다.

Agent SDK를 사용하면 Claude Code의 자율 에이전트 루프를 자신의 애플리케이션에 임베드할 수 있습니다. SDK는 도구, 권한, 비용 제한, 출력에 대한 프로그래밍 방식의 제어를 제공하는 독립형 패키지입니다.

TypeScript 및 Python SDK 모두 기본 Claude Code 바이너리를 번들로 제공하므로 대부분의 설치에는 별도의 Claude Code 설치가 필요하지 않습니다. 설치가 필요한 경우는 [빠른 시작의 설치 참고 사항](/docs/ko/agent-sdk/quickstart)을 참조하십시오.

에이전트를 시작하면 SDK는 [Claude Code를 구동하는 동일한 실행 루프](/docs/ko/how-claude-code-works#the-agentic-loop)를 실행합니다: Claude가 프롬프트를 평가하고, 도구를 호출하여 조치를 취하고, 결과를 받고, 작업이 완료될 때까지 반복합니다. 이 페이지에서는 해당 루프 내에서 무엇이 일어나는지 설명하므로 에이전트를 효과적으로 구축, 디버그 및 최적화할 수 있습니다.

<h2 id="the-loop-at-a-glance">
  루프 한눈에 보기
</h2>

모든 에이전트 세션은 동일한 주기를 따릅니다:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-loop-diagram.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=1c6e8f28d80dba14a7287419656f1237" className="dark:hidden" alt="에이전트 루프 다이어그램: 프롬프트가 에이전트 루프에 진입하고, Claude가 평가하여 도구 호출을 요청하거나(결과가 다시 평가로 피드백됨) 최종 답변을 반환합니다" width="720" height="212" data-path="images/agent-loop-diagram.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-loop-diagram-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=afe723c52a324d3c61fa72fb02432ab6" className="hidden dark:block" alt="에이전트 루프 다이어그램: 프롬프트가 에이전트 루프에 진입하고, Claude가 평가하여 도구 호출을 요청하거나(결과가 다시 평가로 피드백됨) 최종 답변을 반환합니다" width="720" height="212" data-path="images/agent-loop-diagram-dark.svg" />

1. **프롬프트 수신.** Claude가 프롬프트, 시스템 프롬프트, 도구 정의, 대화 기록과 함께 프롬프트를 받습니다. SDK는 세션 메타데이터를 포함하는 서브타입 `"init"`이 있는 [`SystemMessage`](#message-types)를 생성합니다.
2. **평가 및 응답.** Claude가 현재 상태를 평가하고 진행 방법을 결정합니다. 텍스트로 응답하거나, 하나 이상의 도구 호출을 요청하거나, 둘 다 할 수 있습니다. SDK는 텍스트 블록 또는 도구 호출 요청과 같은 각 콘텐츠 블록에 대해 하나씩 하나 이상의 [`AssistantMessage`](#message-types) 객체를 생성합니다.
3. **도구 실행.** SDK가 요청된 각 도구를 실행하고 결과를 수집합니다. 각 도구 결과 세트가 다음 결정을 위해 Claude에게 다시 피드백됩니다. [hooks](/docs/ko/agent-sdk/hooks)를 사용하여 도구 호출을 실행 전에 가로채고, 수정하거나, 차단할 수 있습니다.
4. **반복.** 2단계와 3단계가 주기로 반복됩니다. 각 전체 주기는 한 턴입니다. Claude는 도구 호출이 없는 응답을 생성할 때까지 도구를 계속 호출하고 결과를 처리합니다.
5. **결과 반환.** SDK는 텍스트 응답(도구 호출 없음)이 있는 최종 [`AssistantMessage`](#message-types)를 생성한 후, 최종 텍스트, 토큰 사용량, 비용, 세션 ID가 있는 [`ResultMessage`](#message-types)를 생성합니다.

간단한 질문("여기 어떤 파일이 있나요?")은 `Glob`을 호출하고 결과로 응답하는 한두 턴이 걸릴 수 있습니다. 복잡한 작업("auth 모듈을 리팩토링하고 테스트를 업데이트하세요")은 많은 턴에 걸쳐 수십 개의 도구 호출을 연결할 수 있으며, 파일을 읽고, 코드를 편집하고, 테스트를 실행하며, Claude가 각 결과에 따라 접근 방식을 조정합니다.

<h2 id="turns-and-messages">
  턴과 메시지
</h2>

턴은 루프 내의 한 왕복입니다: Claude가 도구 호출을 포함하는 출력을 생성하고, SDK가 해당 도구를 실행하고, 결과가 자동으로 Claude에게 다시 피드백됩니다. 이는 코드에 제어를 반환하지 않고 발생합니다. Claude가 도구 호출이 없는 출력을 생성할 때까지 턴이 계속되며, 이 시점에서 루프가 끝나고 최종 결과가 전달됩니다.

프롬프트 "Fix the failing tests in auth.ts"에 대한 전체 세션이 어떻게 보일 수 있는지 생각해 봅시다.

먼저 SDK가 프롬프트를 Claude에게 보내고 세션 메타데이터가 있는 [`SystemMessage`](#message-types)를 생성합니다. 그러면 루프가 시작됩니다:

1. **턴 1:** Claude가 `Bash`를 호출하여 `npm test`를 실행합니다. SDK는 도구 호출이 있는 [`AssistantMessage`](#message-types)를 생성하고, 명령을 실행한 후, 출력(3개 실패)이 있는 [`UserMessage`](#message-types)를 생성합니다.
2. **턴 2:** Claude가 `auth.ts`와 `auth.test.ts`에서 `Read`를 호출합니다. SDK가 파일 내용을 반환하고 `AssistantMessage`를 생성합니다.
3. **턴 3:** Claude가 `Edit`를 호출하여 `auth.ts`를 수정한 후, `Bash`를 호출하여 `npm test`를 다시 실행합니다. 3개 테스트 모두 통과합니다. SDK는 `AssistantMessage`를 생성합니다.
4. **최종 턴:** Claude가 도구 호출이 없는 텍스트 전용 응답을 생성합니다: "Fixed the auth bug, all three tests pass now." SDK는 이 텍스트가 있는 최종 `AssistantMessage`를 생성한 후, 비용 및 사용량이 있는 [`ResultMessage`](#message-types)를 생성합니다.

이는 4턴이었습니다: 도구 호출이 있는 3턴, 최종 텍스트 전용 응답 1턴.

`max_turns` / `maxTurns`로 루프를 제한할 수 있으며, 이는 도구 사용 턴만 계산합니다. 예를 들어, 위의 루프에서 `max_turns=2`는 편집 단계 전에 중지했을 것입니다. `max_budget_usd` / `maxBudgetUsd`를 사용하여 지출 임계값에 따라 턴을 제한할 수도 있습니다.

제한이 없으면 Claude가 자체적으로 완료될 때까지 루프가 실행되며, 이는 잘 정의된 작업에는 괜찮지만 개방형 프롬프트("improve this codebase")에서는 오래 실행될 수 있습니다. 예산을 설정하는 것은 프로덕션 에이전트에 좋은 기본값입니다. 옵션 참조는 아래의 [턴과 예산](#turns-and-budget)을 참조하세요.

<h2 id="message-types">
  메시지 타입
</h2>

루프가 실행되면 SDK는 메시지 스트림을 생성합니다. 각 메시지는 루프의 어느 단계에서 왔는지 알려주는 타입을 전달합니다. 5가지 핵심 타입은:

* **`SystemMessage`:** 세션 생명주기 이벤트. `subtype` 필드가 이를 구분합니다:

  * `"init"`: 실행을 위한 세션 메타데이터. `SessionStart` 또는 `Setup` hook이 세션 시작 중에 실행될 때, 해당 [hook 생명주기 메시지](/docs/ko/agent-sdk/typescript#sdkhookstartedmessage)는 `init` 메시지 이전에 도착합니다
  * `"compact_boundary"`: [압축](#automatic-compaction) 후에 발생합니다
  * `"informational"`: 루프의 일반 텍스트 상태 배너
  * `"worker_shutting_down"`: 호스트가 종료되거나 Remote Control이 연결 해제됨

  TypeScript에서 `"init"` 이외의 각 서브타입은 `SDKSystemMessage`의 서브타입이 아니라 [`SDKMessage` 유니온](/docs/ko/agent-sdk/typescript#sdkmessage)의 자체 타입입니다.
* **`AssistantMessage`:** Claude의 응답에서 최종 텍스트 전용 응답을 포함하여 각 콘텐츠 블록에 대해 생성됩니다. 각각은 텍스트 또는 도구 호출과 같은 단일 콘텐츠 블록을 전달하며, 한 응답의 메시지는 메시지 ID를 공유합니다.
* **`UserMessage`:** 각 도구 실행 후 Claude에게 다시 전송되는 도구 결과 콘텐츠와 함께 생성됩니다. 루프 중간에 스트리밍하는 모든 사용자 입력에 대해서도 생성됩니다.
* **`StreamEvent`:** 부분 메시지가 활성화된 경우에만 생성됩니다. 원본 API 스트리밍 이벤트(텍스트 델타, 도구 입력 청크)를 포함합니다. [스트림 응답](/docs/ko/agent-sdk/streaming-output)을 참조하세요.
* **`ResultMessage`:** 에이전트 루프의 끝을 표시합니다. 최종 텍스트 결과, 토큰 사용량, 비용, 세션 ID를 포함합니다. `subtype` 필드를 확인하여 작업이 성공했는지 또는 제한에 도달했는지 확인합니다. `prompt_suggestion`과 같은 소수의 후행 시스템 이벤트가 이후에 도착할 수 있으므로 결과에서 중단하지 말고 스트림을 완료까지 반복합니다. [결과 처리](#handle-the-result)를 참조하세요.

이 5가지 타입은 전체 에이전트 루프 생명주기를 다룹니다. 두 SDK 모두 속도 제한 상태 및 작업 알림과 같은 관찰성 이벤트도 생성하며, 이는 루프를 구동하는 데 필요하지 않습니다. 전체 목록은 [Python 메시지 타입 참조](/docs/ko/agent-sdk/python#message-types)와 [TypeScript 메시지 타입 참조](/docs/ko/agent-sdk/typescript#message-types)를 참조하세요.

<h3 id="handle-messages">
  메시지 처리
</h3>

처리하는 메시지는 구축하는 것에 따라 다릅니다:

* **최종 결과만:** `ResultMessage`를 처리하여 출력, 비용, 작업이 성공했는지 또는 제한에 도달했는지 확인합니다.
* **진행 상황 업데이트:** `AssistantMessage`를 처리하여 Claude가 각 턴에서 무엇을 하고 있는지, 어떤 도구를 호출했는지 확인합니다.
* **라이브 스트리밍:** 부분 메시지를 활성화하여(Python의 `include_partial_messages`, TypeScript의 `includePartialMessages`) 실시간으로 `StreamEvent` 메시지를 받습니다. [실시간 스트림 응답](/docs/ko/agent-sdk/streaming-output)을 참조하세요.

메시지 타입을 확인하는 방법은 SDK에 따라 다릅니다:

* **Python:** `claude_agent_sdk`에서 가져온 클래스에 대해 `isinstance()`로 메시지 타입을 확인합니다(예: `isinstance(message, ResultMessage)`).
* **TypeScript:** `type` 문자열 필드를 확인합니다(예: `message.type === "result"`). `AssistantMessage`와 `UserMessage`는 원본 API 메시지를 `.message` 필드에 래핑하므로 콘텐츠 블록은 `message.content`가 아니라 `message.message.content`에 있습니다.

<Accordion title="예제: 메시지 타입 확인 및 결과 처리">
  <CodeGroup>
    ```python Python theme={null}
    import asyncio
    from claude_agent_sdk import query, AssistantMessage, ResultMessage, TextBlock, ToolUseBlock


    async def main():
        try:
            async for message in query(prompt="Summarize this project"):
                if isinstance(message, AssistantMessage):
                    # 각 AssistantMessage는 하나의 콘텐츠 블록을 전달합니다
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(f"Claude: {block.text}")
                        elif isinstance(block, ToolUseBlock):
                            print(f"Tool call: {block.name}")
                if isinstance(message, ResultMessage):
                    if message.subtype == "success":
                        print(message.result)
                    else:
                        print(f"Stopped: {message.subtype}")
        except Exception as error:
            # 단일 query()는 오류 결과를 생성한 후 발생합니다. 실패가
            # 오류 결과였다면 위의 오류 서브타입 분기가 이미 실행되었습니다.
            # 연결 또는 프로세스 실패는 결과 메시지를 생성하지 않습니다.
            print(f"Session ended with an error: {error}")


    asyncio.run(main())
    ```

    ```typescript TypeScript theme={null}
    import { query } from "@anthropic-ai/claude-agent-sdk";

    try {
      for await (const message of query({ prompt: "Summarize this project" })) {
        if (message.type === "assistant") {
          // 각 assistant 메시지는 하나의 콘텐츠 블록을 전달합니다
          for (const block of message.message.content) {
            if (block.type === "text") {
              console.log(`Claude: ${block.text}`);
            } else if (block.type === "tool_use") {
              console.log(`Tool call: ${block.name}`);
            }
          }
        }
        if (message.type === "result") {
          if (message.subtype === "success") {
            console.log(message.result);
          } else {
            console.log(`Stopped: ${message.subtype}`);
          }
        }
      }
    } catch (error) {
      // 단일 query()는 오류 결과를 생성한 후 발생합니다. 실패가
      // 오류 결과였다면 위의 오류 서브타입 분기가 이미 실행되었습니다.
      // 연결 또는 프로세스 실패는 결과 메시지를 생성하지 않습니다.
      console.log(`Session ended with an error: ${error}`);
    }
    ```
  </CodeGroup>
</Accordion>

<h2 id="tool-execution">
  도구 실행
</h2>

도구는 에이전트에게 조치를 취할 수 있는 능력을 제공합니다. 도구가 없으면 Claude는 텍스트로만 응답할 수 있습니다. 도구를 사용하면 Claude는 파일을 읽고, 명령을 실행하고, 코드를 검색하고, 외부 서비스와 상호작용할 수 있습니다.

<h3 id="built-in-tools">
  내장 도구
</h3>

SDK는 Claude Code를 구동하는 동일한 도구를 포함합니다:

| 카테고리        | 도구                                                              | 기능                                      |
| :---------- | :-------------------------------------------------------------- | :-------------------------------------- |
| **파일 작업**   | `Read`, `Edit`, `Write`                                         | 파일 읽기, 수정, 생성                           |
| **검색**      | `Glob`, `Grep`                                                  | 패턴으로 파일 찾기, 정규식으로 콘텐츠 검색                |
| **실행**      | `Bash`                                                          | 셸 명령, 스크립트, git 작업 실행                   |
| **웹**       | `WebSearch`, `WebFetch`                                         | 웹 검색, 페이지 가져오기 및 파싱                     |
| **검색**      | `ToolSearch`                                                    | 모든 도구를 미리 로드하는 대신 필요에 따라 도구를 동적으로 찾고 로드 |
| **오케스트레이션** | `Agent`, `Skill`, `AskUserQuestion`, `TaskCreate`, `TaskUpdate` | 서브에이전트 생성, 스킬 호출, 사용자에게 질문, 작업 추적       |

[작업 추적 도구를 받지 않는 모델](/docs/ko/agent-sdk/todo-tracking#model-availability)에서 Claude Code는 사용자가 옵트인할 때만 `TaskCreate`와 `TaskUpdate`를 제공합니다.

내장 도구 외에도 다음을 수행할 수 있습니다:

* **[MCP 서버](/docs/ko/agent-sdk/mcp)로 외부 서비스 연결**(데이터베이스, 브라우저, API)
* **[사용자 정의 도구 핸들러](/docs/ko/agent-sdk/custom-tools)로 사용자 정의 도구 정의**
* **[설정 소스](/docs/ko/agent-sdk/claude-code-features)를 통해 프로젝트 스킬 로드** 재사용 가능한 워크플로우

<h3 id="tool-permissions">
  도구 권한
</h3>

Claude는 작업에 따라 호출할 도구를 결정하지만, 해당 호출이 실행되도록 허용할지 여부를 제어합니다. 특정 도구를 자동 승인하거나, 다른 도구를 완전히 차단하거나, 모든 것에 대해 승인을 요구할 수 있습니다. 3가지 옵션이 함께 작동하여 실행되는 것을 결정합니다:

* **`allowed_tools` / `allowedTools`** 나열된 도구를 자동 승인합니다. `["Read", "Glob", "Grep"]`이 허용된 도구 목록에 있는 읽기 전용 에이전트는 프롬프트 없이 해당 도구를 실행합니다. 나열되지 않은 도구는 여전히 사용 가능하며, 승인이 필요한 해당 도구에 대한 호출은 권한 모드 및 `canUseTool`로 넘어갑니다.
* **`disallowed_tools` / `disallowedTools`** 다른 설정에 관계없이 나열된 도구를 차단합니다. 도구가 실행되기 전에 규칙이 확인되는 순서는 [권한](/docs/ko/agent-sdk/permissions)을 참조하세요.
* **`permission_mode` / `permissionMode`** 원하는 인적 감시의 정도를 제어합니다. SDK는 활성 모드를 허용 및 거부 규칙과 함께 고정된 순서로 평가하며, 이는 [권한이 평가되는 방식](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)에 설명되어 있습니다. 사용 가능한 모드는 [권한 모드](#permission-mode)를 참조하세요.

`"Bash(npm *)"` 같은 규칙으로 개별 도구의 범위를 지정할 수도 있습니다. 전체 규칙 구문은 [권한](/docs/ko/agent-sdk/permissions)을 참조하세요.

도구가 거부되면 Claude는 도구 결과로 거부 메시지를 받고 일반적으로 다른 접근 방식을 시도하거나 진행할 수 없다고 보고합니다.

<h3 id="parallel-tool-execution">
  병렬 도구 실행
</h3>

Claude가 단일 턴에서 여러 도구 호출을 요청하면 두 SDK 모두 도구에 따라 동시 또는 순차적으로 실행할 수 있습니다. 읽기 전용 도구(`Read`, `Glob`, `Grep`, 읽기 전용으로 표시된 MCP 도구)는 동시에 실행할 수 있습니다. 상태를 수정하는 도구(`Edit`, `Write`, `Bash`)는 충돌을 피하기 위해 순차적으로 실행됩니다.

사용자 정의 도구는 기본적으로 순차 실행됩니다. 사용자 정의 도구에 대해 병렬 실행을 활성화하려면 주석에서 `readOnlyHint`를 설정합니다. [TypeScript](/docs/ko/agent-sdk/typescript#tool)와 [Python](/docs/ko/agent-sdk/python#tool) SDK 모두 MCP SDK의 이 필드 이름을 사용합니다.

<h2 id="control-how-the-loop-runs">
  루프 실행 방식 제어
</h2>

루프가 취하는 턴 수, 비용, Claude가 추론하는 깊이, 도구가 실행 전에 승인을 요구하는지 여부를 제한할 수 있습니다. 이 모든 것은 [`ClaudeAgentOptions`](/docs/ko/agent-sdk/python#claudeagentoptions)(Python) / [`Options`](/docs/ko/agent-sdk/typescript#options)(TypeScript)의 필드입니다.

<h3 id="turns-and-budget">
  턴과 예산
</h3>

| 옵션                                       | 제어 대상       | 기본값   |
| :--------------------------------------- | :---------- | :---- |
| 최대 턴(`max_turns` / `maxTurns`)           | 최대 도구 사용 왕복 | 제한 없음 |
| 최대 예산(`max_budget_usd` / `maxBudgetUsd`) | 중지 전 최대 비용  | 제한 없음 |

제한 중 하나에 도달하면 SDK는 해당 오류 서브타입(`error_max_turns` 또는 `error_max_budget_usd`)이 있는 `ResultMessage`를 반환합니다. 이러한 서브타입을 확인하는 방법은 [결과 처리](#handle-the-result)를 참조하고, 구문은 [`ClaudeAgentOptions`](/docs/ko/agent-sdk/python#claudeagentoptions) / [`Options`](/docs/ko/agent-sdk/typescript#options)를 참조하세요.

예산 상한은 [서브에이전트](/docs/ko/agent-sdk/subagents)를 포함합니다: 이들의 지출은 총액에 포함됩니다. 지출이 상한에 도달하면 다른 서브에이전트를 생성하는 것이 `Budget limit reached`로 실패하고, Claude Code는 여전히 실행 중인 모든 백그라운드 서브에이전트를 중지합니다. 상한 적용 동작에는 Claude Code v2.1.217 이상이 필요합니다.

[스트리밍 입력](/docs/ko/agent-sdk/streaming-vs-single-mode)을 사용하면 턴이 최대 턴 제한에서 끝날 때 실행 중인 메시지는 대기열에 남아 있습니다. Claude Code는 이를 해당 턴의 마지막 모델 호출에 추가하지 않습니다. 메시지에 대해 새로운 턴을 시작하고, 해당 턴에 대해 최대 턴 수가 다시 시작됩니다. 예산 총액은 메시지 전체에 계속 누적되며, 지출이 `maxBudgetUsd`에 도달하면 같은 대화의 이후 메시지는 `error_max_budget_usd` 결과로 끝납니다. [`/clear`](/docs/ko/agent-sdk/cost-tracking)는 예산을 다시 시작합니다.

<h3 id="effort-level">
  노력 수준
</h3>

`effort` 옵션은 Claude가 적용하는 추론의 양을 제어합니다. 낮은 노력 수준은 턴당 더 적은 토큰을 사용하고 비용을 줄입니다. 모든 모델이 노력 매개변수를 지원하는 것은 아닙니다. 어떤 모델이 지원하는지는 [노력](https://platform.claude.com/docs/en/build-with-claude/effort)을 참조하세요.

| 수준         | 동작           | 적합한 경우                                                          |
| :--------- | :----------- | :-------------------------------------------------------------- |
| `"low"`    | 최소 추론, 빠른 응답 | 파일 조회, 디렉토리 나열                                                  |
| `"medium"` | 균형 잡힌 추론     | 일상적인 편집, 표준 작업                                                  |
| `"high"`   | 철저한 분석       | 리팩토링, 디버깅                                                       |
| `"xhigh"`  | 확장된 추론 깊이    | [지원하는 모델](/docs/ko/model-config#adjust-effort-level)에서의 코딩 및 에이전트 작업 |
| `"max"`    | 최대 추론 깊이     | 깊은 분석이 필요한 다단계 문제                                               |

`effort`를 설정하지 않으면 Claude Code는 노력 수준을 자체적으로 해결하며, [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)에서 설명하는 순서대로 진행합니다.

<Note>
  `effort`는 각 응답 내에서 추론 깊이에 대한 지연 시간과 토큰 비용을 교환합니다. [확장 사고](https://platform.claude.com/docs/en/build-with-claude/extended-thinking)는 출력에서 `thinking` 블록을 생성하는 별도의 기능이며, [Python](/docs/ko/agent-sdk/python#thinkingconfig) 또는 [TypeScript](/docs/ko/agent-sdk/typescript#thinkingconfig)의 `ThinkingConfig`에 있는 `display` 필드는 해당 텍스트를 받을지 여부를 제어합니다. 이들은 독립적입니다: `effort: "low"`를 확장 사고 활성화로 설정하거나, `effort: "max"`를 비활성화로 설정할 수 있습니다.
</Note>

간단하고 잘 정의된 작업(파일 나열 또는 단일 grep 실행)을 수행하는 에이전트의 경우 비용과 지연 시간을 줄이기 위해 낮은 노력을 사용합니다. 전체 세션에 대해 최상위 `query()` 옵션에서 `effort`를 설정하거나, [`AgentDefinition`](/docs/ko/agent-sdk/subagents#agentdefinition-configuration)의 `effort` 필드로 서브에이전트당 설정하여 세션 수준을 재정의합니다.

<h3 id="permission-mode">
  권한 모드
</h3>

권한 모드 옵션(Python의 `permission_mode`, TypeScript의 `permissionMode`)은 에이전트가 도구를 사용하기 전에 승인을 요청하는지 여부를 제어합니다:

| 모드                    | 동작                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | 사용 사례                                                                           |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------ |
| `"default"`           | 승인이 필요하고 허용 규칙으로 다루지 않는 도구 호출은 `canUseTool` 콜백을 트리거합니다; 콜백이 없으면 거부                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | 사용자 정의 승인 콜백이 있는 대화형 애플리케이션                                                     |
| `"acceptEdits"`       | 파일 편집 및 일반적인 파일시스템 명령(`mkdir`, `touch`, `mv`, `cp` 등)을 자동 승인합니다; 다른 Bash 명령은 기본 규칙을 따릅니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Claude의 편집을 신뢰하고 더 빠른 반복을 원하는 경우(예: 프로토타이핑 중이거나 격리된 디렉토리에서 작업할 때)               |
| `"plan"`              | Claude는 소스 파일을 편집하지 않고 탐색하고 계획을 생성합니다; 파일 편집은 절대 자동 승인되지 않으며 `canUseTool` 콜백을 통해 프롬프트됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Claude가 변경 사항을 제안하되 실행하지 않기를 원하는 경우(예: 코드 검토 중이거나 변경 사항이 적용되기 전에 승인해야 할 때)      |
| `"dontAsk"`           | 절대 프롬프트하지 않습니다. [권한 규칙](/docs/ko/settings-reference#permission-settings)으로 사전 승인된 도구가 실행되고, `default` 모드에서 승인이 필요 없는 호출(예: 작업 디렉토리 내의 파일 읽기)도 실행됩니다; 그 외에 프롬프트되는 모든 호출은 거부됩니다. `AskUserQuestion`, 조직이 [`ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)한 커넥터 도구, 그리고 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 허용했더라도 거부됩니다                                                                                                                                                                          | 헤드리스 에이전트를 위한 고정적이고 명시적인 도구 표면을 원하고 `canUseTool`이 없을 때의 자동 거부보다 명시적 거부를 선호하는 경우 |
| `"auto"`              | 모델 분류기를 사용하여 권한 프롬프트를 승인하거나 거부합니다. 가용성 및 동작은 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                        | 도구 사용에 대한 안전 가드레일을 원하는 자율 에이전트                                                  |
| `"bypassPermissions"` | 명시적 [`ask` 규칙](/docs/ko/settings-reference#permission-settings)이 일치하는 도구, 조직이 [`ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)한 커넥터 도구, 그리고 사용자 상호작용이 필요한 도구를 제외하고 요청하지 않고 모든 허용된 도구를 실행합니다. [교차 세션 메시징 보안 조치](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode)는 여전히 적용됩니다. 우선순위 순서는 [권한이 평가되는 방식](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)을 참조하세요. TypeScript SDK에서는 `options`에서 `allowDangerouslySkipPermissions: true`도 필요합니다. Unix에서 루트로 실행할 때는 사용할 수 없습니다. 에이전트의 조치가 관심 있는 시스템에 영향을 미칠 수 없는 격리된 환경에서만 사용합니다 | CI, 컨테이너 또는 기타 격리된 환경                                                           |

대화형 애플리케이션의 경우 `"default"`를 도구 승인 콜백과 함께 사용하여 승인 프롬프트를 표시합니다. 개발 머신의 자율 에이전트의 경우 `"acceptEdits"`는 파일 편집 및 일반적인 파일시스템 명령(`mkdir`, `touch`, `mv`, `cp` 등)을 자동 승인하면서 다른 `Bash` 명령을 허용 규칙 뒤에 유지합니다. CI, 컨테이너 또는 기타 격리된 환경에 대해 `"bypassPermissions"`를 예약합니다. 전체 세부 정보는 [권한](/docs/ko/agent-sdk/permissions)을 참조하세요.

<h3 id="model">
  모델
</h3>

`model` 옵션을 설정하여 세션을 실행할 모델을 선택합니다. 자세한 내용은 [모델 선택](/docs/ko/agent-sdk/configuration#choose-a-model)을 참조하세요.

<h2 id="the-context-window">
  컨텍스트 윈도우
</h2>

컨텍스트 윈도우는 세션 중에 Claude가 사용할 수 있는 총 정보량입니다. 세션 내의 턴 사이에 재설정되지 않습니다. 모든 것이 누적됩니다: 시스템 프롬프트, 도구 정의, 대화 기록, 도구 입력, 도구 출력. 턴 전체에서 동일하게 유지되는 콘텐츠(시스템 프롬프트, 도구 정의, CLAUDE.md)는 자동으로 [프롬프트 캐시됩니다](https://platform.claude.com/docs/en/build-with-claude/prompt-caching). 이는 반복된 접두사에 대한 비용과 지연 시간을 줄입니다. 사용자 정의 시스템 프롬프트 또는 `append` 텍스트가 세션 간 캐시 재사용에 어떻게 영향을 미치는지에 대해서는 [시스템 프롬프트 수정](/docs/ko/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)을 참조하세요.

<h3 id="what-consumes-context">
  컨텍스트를 소비하는 것
</h3>

SDK에서 각 구성 요소가 컨텍스트에 어떻게 영향을 미치는지는 다음과 같습니다:

| 소스               | 로드 시기                                                             | 영향                                                                                                                                                                                                                |
| :--------------- | :---------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **시스템 프롬프트**     | 모든 요청                                                             | 작은 고정 비용, 항상 존재                                                                                                                                                                                                   |
| **CLAUDE.md 파일** | 세션 시작, [`settingSources`](/docs/ko/agent-sdk/claude-code-features)를 통해 | 모든 요청에서 전체 콘텐츠(프롬프트 캐시되므로 첫 번째 요청만 전체 비용 지불)                                                                                                                                                                      |
| **도구 정의**        | 모든 요청; MCP 스키마는 기본적으로 지연됨                                         | 기본 제공 도구 스키마는 모든 요청에서 로드됩니다. [도구 검색](/docs/ko/agent-sdk/mcp#mcp-tool-search)은 기본적으로 MCP 도구 스키마를 지연시키며, 지원되지 않는 모델 및 특정 플랫폼에서 사전 로드로 폴백합니다. 전체 매트릭스는 [도구 검색 구성](/docs/ko/agent-sdk/tool-search#configure-tool-search)을 참조하세요 |
| **대화 기록**        | 턴에 걸쳐 누적                                                          | 각 턴으로 증가: 프롬프트, 응답, 도구 입력, 도구 출력                                                                                                                                                                                  |
| **스킬 설명**        | 세션 시작, 설정 소스를 통해                                                  | 짧은 요약; 전체 콘텐츠는 호출될 때만 로드                                                                                                                                                                                          |

큰 도구 출력은 상당한 컨텍스트를 소비합니다. 큰 파일을 읽거나 자세한 출력이 있는 명령을 실행하면 단일 턴에서 수천 개의 토큰을 사용할 수 있습니다. 컨텍스트는 턴에 걸쳐 누적되므로 많은 도구 호출이 있는 더 긴 세션은 짧은 세션보다 훨씬 더 많은 컨텍스트를 구축합니다.

<h3 id="automatic-compaction">
  자동 압축
</h3>

컨텍스트 윈도우가 제한에 가까워지면 SDK는 자동으로 대화를 압축합니다: 더 오래된 기록을 요약하여 공간을 확보하고, 가장 최근의 교환과 주요 결정을 유지합니다. SDK는 이것이 발생할 때 스트림에서 `type: "system"`과 `subtype: "compact_boundary"`가 있는 메시지를 생성합니다(Python에서는 `SystemMessage`; TypeScript에서는 별도의 `SDKCompactBoundaryMessage` 타입).

압축은 더 오래된 메시지를 요약으로 바꾸므로 대화 초기의 특정 지침이 보존되지 않을 수 있습니다. 지속적인 규칙은 초기 프롬프트가 아니라 CLAUDE.md에 속합니다([`settingSources`](/docs/ko/agent-sdk/claude-code-features)를 통해 로드됨). CLAUDE.md 콘텐츠는 모든 요청에서 다시 주입되기 때문입니다.

압축 동작을 여러 가지 방법으로 사용자 정의할 수 있습니다:

* **CLAUDE.md의 요약 지침:** 압축기는 다른 컨텍스트처럼 CLAUDE.md를 읽으므로 요약할 때 보존할 내용을 알려주는 섹션을 포함할 수 있습니다. 압축기는 의도와 일치하므로 섹션 헤더는 자유 형식입니다.
* **`PreCompact` hook:** 압축 전에 사용자 정의 로직을 실행합니다(예: 전체 기록을 보관). hook은 `trigger` 필드(`manual` 또는 `auto`)를 받습니다. [hooks](/docs/ko/agent-sdk/hooks)를 참조하세요.
* **수동 압축:** `/compact`를 프롬프트 문자열로 보내 필요에 따라 압축을 트리거합니다. 이 방식으로 전송된 명령은 일반적인 SDK 입력입니다. [이름으로 명령 디스패치](/docs/ko/agent-sdk/skills#dispatch-commands-by-name)를 참조하세요.

<Accordion title="예제: CLAUDE.md의 요약 지침">
  프로젝트의 CLAUDE.md에 섹션을 추가하여 압축기에 보존할 내용을 알립니다. 헤더 이름은 특별하지 않습니다; 명확한 레이블을 사용합니다.

  ```markdown CLAUDE.md theme={null}
  # 요약 지침

  이 대화를 요약할 때 항상 보존합니다:
  - 현재 작업 목표 및 수용 기준
  - 읽거나 수정한 파일 경로
  - 테스트 결과 및 오류 메시지
  - 결정 및 그 뒤의 추론
  ```
</Accordion>

<h3 id="keep-context-efficient">
  컨텍스트 효율성 유지
</h3>

장기 실행 에이전트를 위한 몇 가지 전략:

* **서브작업에 서브에이전트 사용.** 각 서브에이전트는 새로운 대화로 시작합니다(이전 메시지 기록 없음, 자신의 시스템 프롬프트 및 CLAUDE.md 같은 프로젝트 수준 컨텍스트는 로드함). 부모의 턴을 보지 않으며, 최종 응답만 도구 결과로 부모에게 반환됩니다. 주 에이전트의 컨텍스트는 전체 서브작업 기록이 아니라 해당 요약으로 증가합니다. [서브에이전트가 상속하는 것](/docs/ko/agent-sdk/subagents#what-subagents-inherit)을 참조하세요.
* **도구를 선택적으로 사용합니다.** 모든 도구 정의는 컨텍스트 공간을 차지합니다. [`AgentDefinition`](/docs/ko/agent-sdk/subagents#agentdefinition-configuration)의 `tools` 필드를 사용하여 서브에이전트를 필요한 최소 세트로 범위를 지정합니다.
* **MCP 서버 비용을 주시합니다.** [MCP 도구 검색](/docs/ko/agent-sdk/mcp#mcp-tool-search)은 기본적으로 MCP 도구 스키마를 지연시키고 필요에 따라 로드합니다. 도구 검색이 꺼져 있거나 지원되지 않는 모델에서 폴백되었거나 특정 플랫폼에 있으면 각 MCP 서버는 모든 도구 스키마를 모든 요청에 추가하므로 많은 도구가 있는 몇 개의 서버는 에이전트가 작업을 수행하기 전에 상당한 컨텍스트를 소비할 수 있습니다. 폴백이 적용되는 구성에 대해서는 [도구 검색 구성](/docs/ko/agent-sdk/tool-search#configure-tool-search)을 참조하세요.
* **일상적인 작업에 낮은 노력을 사용합니다.** 파일을 읽거나 디렉토리를 나열하기만 하면 되는 에이전트의 경우 [노력](#effort-level)을 `"low"`로 설정합니다. 이는 토큰 사용량과 비용을 줄입니다.

기능별 컨텍스트 비용의 자세한 분석은 [컨텍스트 비용 이해](/docs/ko/features-overview#understand-context-costs)를 참조하세요.

<h2 id="sessions-and-continuity">
  세션 및 연속성
</h2>

SDK와의 각 상호작용은 세션을 생성하거나 계속합니다. `ResultMessage.session_id`에서 세션 ID를 캡처합니다(두 SDK 모두에서 사용 가능). TypeScript SDK는 또한 init `SystemMessage`의 직접 필드로 노출합니다; Python에서는 `SystemMessage.data`에 중첩됩니다.

재개할 때 이전 턴의 전체 컨텍스트가 복원됩니다: 읽은 파일, 수행된 분석, 취한 조치. 원본을 수정하지 않고 다른 접근 방식으로 분기하기 위해 세션을 포크할 수도 있습니다.

[세션 관리](/docs/ko/agent-sdk/sessions)에서 재개, 계속, 포크 패턴의 전체 가이드를 참조하세요. 상태 비저장 컨테이너 또는 서버리스 호스트 전체에서 세션을 재개하려면 [`session_store` / `sessionStore` 어댑터](/docs/ko/agent-sdk/session-storage)를 전달하여 SDK가 트랜스크립트를 자신의 백엔드로 미러링하고 다른 호스트가 이를 재개할 수 있도록 합니다. Claude Code 서브프로세스는 여전히 먼저 로컬 디스크에 씁니다. [이중 쓰기 아키텍처](/docs/ko/agent-sdk/session-storage#dual-write-architecture)에서 어느 복사본이 새 세션보다 오래 지속되는지 또는 저장소에서 재개된 실행과 로컬 복사본을 임시로 유지하는 방법을 참조하세요.

<Note>
  Python에서 `ClaudeSDKClient`는 여러 호출에 걸쳐 세션 ID를 자동으로 처리합니다. 세부 정보는 [Python SDK 참조](/docs/ko/agent-sdk/python#choosing-between-query-and-claudesdkclient)를 참조하세요.
</Note>

<h2 id="handle-the-result">
  결과 처리
</h2>

루프가 끝나면 `ResultMessage`는 무엇이 일어났는지 알려주고 출력을 제공합니다. `subtype` 필드(두 SDK 모두에서 사용 가능)는 종료 상태를 확인하는 주요 방법입니다.

| 결과 서브타입                               | 무엇이 일어났는가                                                                                   | `result` 필드 사용 가능? |
| :------------------------------------ | :------------------------------------------------------------------------------------------ | :----------------: |
| `success`                             | Claude가 정상적으로 작업을 완료했습니다                                                                    |          예         |
| `error_max_turns`                     | 완료 전에 `maxTurns` 제한에 도달했습니다                                                                 |         아니오        |
| `error_max_budget_usd`                | 완료 전에 `maxBudgetUsd` 제한에 도달했습니다                                                             |         아니오        |
| `error_during_execution`              | 오류가 루프를 중단했습니다(예: 취소된 요청)                                                                   |         아니오        |
| `error_max_structured_output_retries` | 구성된 재시도 제한 내에서 유효한 구조화된 출력이 생성되지 않았습니다: 모든 시도가 검증에 실패했거나, 모델 폴백이 성공적인 재시도 없이 완료된 출력을 취소했습니다 |         아니오        |

`result` 필드는 최종 텍스트 출력을 보유하며 `success` 변형에만 존재하므로 항상 읽기 전에 서브타입을 확인합니다.

모든 결과 서브타입은 `total_cost_usd`, `usage`, `num_turns`, `session_id`를 전달하므로 비용을 추적하고 오류 후에도 재개할 수 있습니다. 다음 두 가지를 주의해야 합니다:

* 세션 충돌 후 최종 결과는 `error_during_execution`이며, 비용 필드가 0으로 설정될 수 있고 `stop_reason`은 `null`이며, 프로세스는 이를 내보낸 후 종료됩니다. [세션 충돌 후 합계 복구](/docs/ko/agent-sdk/cost-tracking#recover-totals-after-a-session-crash)를 참조하세요.
* Python에서 `total_cost_usd`, `usage`, `model_usage`는 선택적으로 입력되므로 읽기 전에 `None`이 아닌지 확인합니다.

`usage` 필드는 주 에이전트 루프만 포함합니다. 전체 트리 토큰 및 비용 계산을 위해 Python에서 `modelUsage` 또는 `model_usage`를 사용합니다. `usage` 필드 해석에 대한 세부 정보는 [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking)을 참조하세요.

<Note>
  쿼리가 오류 결과로 끝날 때:

  * 단일 `query()` 호출은 최종 결과 메시지를 생성한 다음 `Reached maximum number of turns`와 같은 실패 텍스트를 포함하는 오류를 발생시킵니다. 발생은 의도적입니다. 코드가 이를 지나서 계속 진행해야 하는 경우 루프를 try 블록으로 래핑합니다. 기본 Claude Code 프로세스도 0이 아닌 코드로 종료됩니다.
  * 스트리밍 입력 세션은 활성 상태로 유지되며 계속 메시지를 보낼 수 있습니다. 단, 세션 충돌 후는 제외되며, 이는 최종 `error_during_execution` 결과를 내보내고 프로세스를 종료합니다.
</Note>

결과는 또한 모델이 최종 턴에서 생성을 중지한 이유를 나타내는 `stop_reason` 필드(`TypeScript에서 string | null`, Python에서 `str | None`)를 포함합니다. 일반적인 값은 `end_turn`(모델이 정상적으로 완료됨), `max_tokens`(출력 토큰 제한에 도달함), `refusal`(모델이 요청을 거부함)입니다. 루프가 생성한 오류 결과에서 `stop_reason`은 루프가 끝나기 전의 마지막 어시스턴트 응답의 값을 전달합니다. 세션 충돌 후 Claude Code가 합성하는 결과는 `null`을 전달합니다.

거부를 감지하려면 `stop_reason === "refusal"`(TypeScript) 또는 `stop_reason == "refusal"`(Python)을 확인합니다. 전체 타입은 [`SDKResultMessage`](/docs/ko/agent-sdk/typescript#sdkresultmessage)(TypeScript) 또는 [`ResultMessage`](/docs/ko/agent-sdk/python#resultmessage)(Python)을 참조하세요.

<h2 id="hooks">
  Hooks
</h2>

[Hooks](/docs/ko/agent-sdk/hooks)는 루프의 특정 지점에서 발생하는 콜백입니다: 도구가 실행되기 전, 반환된 후, 에이전트가 완료될 때 등. 일반적으로 사용되는 일부 hooks는:

| Hook                             | 발생 시기               | 일반적인 사용          |
| :------------------------------- | :------------------ | :--------------- |
| `PreToolUse`                     | 도구가 실행되기 전          | 입력 검증, 위험한 명령 차단 |
| `PostToolUse`                    | 도구가 반환된 후           | 출력 감사, 부작용 트리거   |
| `UserPromptSubmit`               | 프롬프트가 전송될 때         | 프롬프트에 추가 컨텍스트 주입 |
| `Stop`                           | 에이전트가 완료될 때         | 결과 검증, 세션 상태 저장  |
| `SubagentStart` / `SubagentStop` | 서브에이전트가 생성되거나 완료될 때 | 병렬 작업 결과 추적 및 집계 |
| `PreCompact`                     | 컨텍스트 압축 전           | 요약 전에 전체 기록 보관   |

Hooks는 에이전트의 컨텍스트 윈도우 내가 아니라 애플리케이션 프로세스에서 실행되므로 컨텍스트를 소비하지 않습니다. Hooks는 또한 루프를 단락시킬 수 있습니다: 도구 호출을 거부하는 `PreToolUse` hook은 실행을 방지하고 Claude는 거부 메시지를 대신 받습니다.

두 SDK 모두 위의 모든 이벤트를 지원합니다. TypeScript SDK는 Python이 아직 지원하지 않는 추가 이벤트를 포함합니다. 전체 이벤트 목록, SDK별 가용성, 전체 콜백 API는 [hooks로 실행 제어](/docs/ko/agent-sdk/hooks)를 참조하세요.

<h2 id="put-it-all-together">
  모두 함께 사용
</h2>

이 예제는 이 페이지의 주요 개념을 실패한 테스트를 수정하는 단일 에이전트로 결합합니다. 허용된 도구(자동 승인되므로 에이전트가 자율적으로 실행됨), 프로젝트 설정, 턴 및 추론 노력에 대한 안전 제한으로 에이전트를 구성합니다. 루프가 실행되면 잠재적 재개를 위해 세션 ID를 캡처하고, 최종 결과를 처리하고, 총 비용을 인쇄합니다.

단일 `query()` 호출이 오류 결과를 생성한 후 발생하기 때문에, 루프는 try 블록으로 래핑되어 제한에 도달하면 스크립트가 깔끔하게 종료됩니다.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def run_agent():
      session_id = None

      try:
          async for message in query(
              prompt="Find and fix the bug causing test failures in the auth module",
              options=ClaudeAgentOptions(
                  allowed_tools=[
                      "Read",
                      "Edit",
                      "Bash",
                      "Glob",
                      "Grep",
                  ],  # Listing tools here auto-approves them (no prompting)
                  setting_sources=[
                      "project"
                  ],  # Load CLAUDE.md, skills, hooks from current directory
                  max_turns=30,  # Prevent runaway sessions
                  effort="high",  # Thorough reasoning for complex debugging
              ),
          ):
              # Handle the final result
              if isinstance(message, ResultMessage):
                  session_id = message.session_id  # Save for potential resumption

                  if message.subtype == "success":
                      print(f"Done: {message.result}")
                  elif message.subtype == "error_max_turns":
                      # Agent ran out of turns. Resume with a higher limit.
                      print(f"Hit turn limit. Resume session {session_id} to continue.")
                  elif message.subtype == "error_max_budget_usd":
                      print("Hit budget limit.")
                  else:
                      print(f"Stopped: {message.subtype}")
                  if message.total_cost_usd is not None:
                      print(f"Cost: ${message.total_cost_usd:.4f}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branches above have
          # already run; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(run_agent())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Find and fix the bug causing test failures in the auth module",
      options: {
        allowedTools: ["Read", "Edit", "Bash", "Glob", "Grep"], // Listing tools here auto-approves them (no prompting)
        settingSources: ["project"], // Load CLAUDE.md, skills, hooks from current directory
        maxTurns: 30, // Prevent runaway sessions
        effort: "high" // Thorough reasoning for complex debugging
      }
    })) {
      // Save the session ID to resume later if needed
      if (message.type === "system" && message.subtype === "init") {
        sessionId = message.session_id;
      }

      // Handle the final result
      if (message.type === "result") {
        if (message.subtype === "success") {
          console.log(`Done: ${message.result}`);
        } else if (message.subtype === "error_max_turns") {
          // Agent ran out of turns. Resume with a higher limit.
          console.log(`Hit turn limit. Resume session ${sessionId} to continue.`);
        } else if (message.subtype === "error_max_budget_usd") {
          console.log("Hit budget limit.");
        } else {
          console.log(`Stopped: ${message.subtype}`);
        }
        console.log(`Cost: $${message.total_cost_usd.toFixed(4)}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branches above have
    // already run; connection or process failures yield no result message.
    console.log(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

에이전트가 성공적으로 완료되면, 예제는 에이전트의 수정 요약과 함께 `Done:` 줄을 인쇄한 다음 `Cost: $0.0312`와 같은 줄을 인쇄합니다.

<h2 id="next-steps">
  다음 단계
</h2>

이제 루프를 이해했으므로 구축하는 것에 따라 다음 위치로 이동합니다:

* **아직 에이전트를 실행하지 않았나요?** [빠른 시작](/docs/ko/agent-sdk/quickstart)으로 시작하여 SDK를 설치하고 끝에서 끝까지 실행되는 전체 예제를 확인합니다.
* **프로젝트에 연결할 준비가 되었나요?** [CLAUDE.md, skills, 파일시스템 hooks 로드](/docs/ko/agent-sdk/claude-code-features)하여 에이전트가 프로젝트 규칙을 자동으로 따르도록 합니다.
* **대화형 UI를 구축하고 있나요?** [스트리밍](/docs/ko/agent-sdk/streaming-output)을 활성화하여 루프가 실행되면서 라이브 텍스트 및 도구 호출을 표시합니다.
* **에이전트가 할 수 있는 것에 대해 더 엄격한 제어가 필요하신가요?** [권한](/docs/ko/agent-sdk/permissions)으로 도구 접근을 잠그고, [hooks](/docs/ko/agent-sdk/hooks)를 사용하여 실행 전에 도구 호출을 감사, 차단 또는 변환합니다.
* **장기 또는 비용이 많이 드는 작업을 실행하고 있나요?** 격리된 작업을 [서브에이전트](/docs/ko/agent-sdk/subagents)로 오프로드하여 주 컨텍스트를 깔끔하게 유지합니다.
* **서비스로 배포하고 있나요?** 컨테이너 및 서버리스 지침은 [Agent SDK 호스팅](/docs/ko/agent-sdk/hosting)을 참조하고, 세션을 자신의 백엔드에 유지하려면 [세션 저장소](/docs/ko/agent-sdk/session-storage)를 참조합니다.

에이전트 루프의 더 광범위한 개념적 그림(SDK 특정이 아님)은 [Claude Code의 작동 원리](/docs/ko/how-claude-code-works)를 참조하세요. Claude Code에서 루프를 설계하는 실용적인 가이드(턴 기반에서 목표 기반 및 사전 예방적 루프까지)는 블로그의 [Loop engineering: getting started with loops](https://claude.com/blog/getting-started-with-loops)를 참조하세요.
