> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 권한 구성

> 권한 모드, 훅, 선언적 허용/거부 규칙을 사용하여 에이전트가 도구를 사용하는 방식을 제어합니다.

Claude Agent SDK는 Claude가 도구를 사용하는 방식을 관리하기 위한 권한 제어를 제공합니다. 권한 모드와 규칙을 사용하여 자동으로 허용되는 항목을 정의하고, [`canUseTool` 콜백](/docs/ko/agent-sdk/user-input)을 사용하여 런타임에 나머지 모든 항목을 처리합니다.

<Note>
  이 페이지는 권한 모드와 규칙을 다룹니다. 사용자가 런타임에 도구 요청을 승인하거나 거부하는 대화형 승인 흐름을 구축하려면 [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input)를 참조하세요.
</Note>

<h2 id="how-permissions-are-evaluated">
  권한 평가 방식
</h2>

Claude가 도구를 요청할 때 SDK는 다음 순서로 권한을 확인합니다:

<Steps>
  <Step title="훅">
    먼저 [훅](/docs/ko/agent-sdk/hooks)을 실행합니다. 훅은 호출을 완전히 거부하거나 통과시킬 수 있습니다. `allow`를 반환하는 훅은 아래의 거부 및 요청 규칙을 건너뛰지 않습니다. 훅 결과와 관계없이 이러한 규칙들이 평가됩니다.
  </Step>

  <Step title="거부 규칙">
    `deny` 규칙(`disallowed_tools` 및 [settings.json](/docs/ko/settings#permission-settings)에서)을 확인합니다. 거부 규칙이 일치하면 `bypassPermissions` 모드에서도 도구가 차단됩니다. `Bash`와 같은 단순 이름의 거부 규칙은 이 평가가 시작되기 전에 Claude의 컨텍스트에서 도구를 제거하므로 `Bash(rm *)`와 같은 범위가 지정된 규칙만 이 단계에서 확인됩니다.
  </Step>

  <Step title="요청 규칙">
    [settings.json](/docs/ko/settings#permission-settings)에서 `ask` 규칙을 확인합니다. 요청 규칙이 일치하면 `bypassPermissions` 모드에서도 호출이 확인을 위해 [`canUseTool` 콜백](/docs/ko/agent-sdk/user-input)으로 전달됩니다.

    사용자 상호작용이 필요한 도구는 동일한 방식으로 작동합니다: `AskUserQuestion` 및 서버가 [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)을 설정하는 MCP 도구는 허용 규칙이 일치하는 경우에도 항상 콜백으로 전달됩니다. `dontAsk` 모드에서는 두 경우 모두 거부됩니다. 이 모드는 절대 프롬프트를 표시하지 않기 때문입니다. MCP 주석에는 Claude Code v2.1.199 이상이 필요합니다.

    [claude.ai 커넥터](/docs/ko/mcp#organization-controls-on-connector-tools) 도구는 조직이 `ask`로 설정한 경우 이 단계에서 흐름을 떠납니다. 모든 호출은 `bypassPermissions` 모드에서도, 허용 규칙이 일치하는 경우에도 콜백으로 전달됩니다. 콜백은 `Your organization requires approval for this tool` 이유를 받습니다. `dontAsk` 모드에서는 호출이 거부됩니다. 이 모드는 절대 프롬프트를 표시하지 않기 때문입니다.
  </Step>

  <Step title="권한 모드">
    활성 [권한 모드](#permission-modes)를 적용합니다. `bypassPermissions`는 이 단계에 도달한 모든 항목을 승인합니다. `acceptEdits`는 파일 작업을 승인합니다. `plan`은 허용 규칙과 관계없이 파일 편집 및 셸 쓰기 도구를 `canUseTool` 콜백으로 라우팅하므로 계획 중에는 쓰기 작업을 자동 승인할 수 없습니다. 다른 모드는 통과합니다.
  </Step>

  <Step title="허용 규칙">
    `allow` 규칙(`allowed_tools` 및 settings.json에서)을 확인합니다. 규칙이 일치하면 도구가 승인됩니다.
  </Step>

  <Step title="canUseTool 콜백">
    위의 어느 것으로도 해결되지 않으면 결정을 위해 [`canUseTool` 콜백](/docs/ko/agent-sdk/user-input)을 호출합니다. `dontAsk` 모드에서는 이 단계를 건너뛰고 도구가 거부됩니다.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" alt="6단계 권한 평가 흐름의 다이어그램으로, 위의 단계와 일치합니다: 도구 요청이 훅, 거부 규칙, 요청 규칙, 권한 모드, 허용 규칙 및 canUseTool을 통과합니다. 훅, 거부 규칙 및 canUseTool은 차단으로 라우팅할 수 있습니다. 권한 모드 우회, 허용 규칙 및 canUseTool은 실행으로 라우팅할 수 있습니다." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

v2.1.198부터 이 평가 순서에 도달할 수 없는 `canUseTool` 콜백을 전달하면 TypeScript SDK는 쿼리가 구성될 때 Node.js 프로세스 경고를 한 번 발생시킵니다. 경고의 코드는 `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`입니다. 두 가지 구성이 이를 트리거합니다:

* `permissionMode: 'bypassPermissions'` - 권한 모드 단계에 도달한 모든 호출을 자동 승인합니다
* `"Read"`와 같은 각 단순 `allowedTools` 항목 - 콜백이 상담되기 전에 전체 도구를 자동 승인합니다

`Bash(ls *)`와 같은 지정자가 있는 항목과 `acceptEdits` 모드는 이를 트리거하지 않으며, 설정 파일에서 오는 허용 규칙은 확인에 표시되지 않습니다.

`process.on('warning', ...)`으로 수신하고 코드를 일치시켜 로깅하거나 억제합니다. 모드 및 규칙과 관계없이 모든 도구 호출을 제어하려면 대신 [`PreToolUse` 훅](/docs/ko/agent-sdk/hooks)을 사용합니다.

이 페이지는 **허용 및 거부 규칙**과 **권한 모드**에 중점을 둡니다. 다른 단계의 경우:

* **훅:** 도구 요청을 허용, 거부 또는 수정하는 사용자 정의 코드를 실행합니다. [훅으로 실행 제어](/docs/ko/agent-sdk/hooks)를 참조하세요.
* **canUseTool 콜백:** 런타임에 사용자에게 승인을 요청합니다. 이전 단계에서 호출이 해결되지 않을 때 사용합니다. [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input)를 참조하세요.

<h2 id="allow-and-deny-rules">
  허용 및 거부 규칙
</h2>

`allowed_tools` 및 `disallowed_tools`(TypeScript: `allowedTools` / `disallowedTools`)는 위의 평가 흐름에서 허용 및 거부 규칙 목록에 항목을 추가합니다. 허용 규칙은 승인에만 영향을 미칩니다. `allowed_tools`에 나열되지 않은 도구는 여전히 Claude에서 사용 가능하며 권한 모드로 통과합니다. 거부 규칙은 도구의 이름을 지정하는지 또는 도구 내의 패턴을 범위로 지정하는지에 따라 다르게 동작합니다.

| 옵션                                | 효과                                                                                                           |
| :-------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` 및 `Grep`은 자동으로 승인됩니다. 여기에 나열되지 않은 도구는 여전히 존재하며 권한 모드 및 `canUseTool`로 통과합니다.                           |
| `disallowed_tools=["Bash"]`       | `Bash` 도구 정의가 요청에서 제거됩니다. Claude는 도구를 보지 못하며 시도할 수 없습니다.                                                     |
| `disallowed_tools=["Bash(rm *)"]` | `Bash`는 계속 사용 가능합니다. `rm *`과 일치하는 호출은 `bypassPermissions`를 포함한 모든 권한 모드에서 거부됩니다. 다른 `Bash` 호출은 권한 모드로 통과합니다. |
| `disallowed_tools=["*"]`          | 모든 도구 정의가 요청에서 제거됩니다. 도구 이름 글롭이 거부 규칙에서 지원됩니다. `"*"`는 모든 도구와 일치하고 `"mcp__*"`는 모든 서버의 모든 MCP 도구와 일치합니다.       |

허용 규칙은 리터럴 `mcp__<server>__` 접두사 이후에만 도구 이름 글롭을 허용합니다. 서버 세그먼트는 글롭이 없어야 하므로 규칙이 구성한 특정 서버의 이름을 지정합니다. `mcp__puppeteer__*`는 `puppeteer` 서버의 모든 도구와 일치하고 `mcp__github__get_*`는 해당 `get_` 도구와 일치합니다. `allowed_tools=["*"]` 또는 `allowed_tools=["mcp__*"]`와 같은 앵커되지 않은 항목은 시작 경고와 함께 무시되며 아무것도 자동 승인하지 않습니다.

`Read` 및 `Edit`에 대한 범위 규칙은 경로 패턴을 사용합니다. `Edit(path)` 규칙은 `Write` 및 `NotebookEdit`을 포함하여 파일을 쓰는 모든 기본 제공 도구를 관리합니다. `Write(path)` 규칙은 파일 권한 검사에 의해 절대 일치하지 않습니다.

절대 파일 시스템 경로에는 `//path`를 사용합니다. `Edit(//secrets/**)` 거부 규칙은 디스크의 `/secrets` 아래 어디든지 쓰기를 차단합니다. 단일 선행 슬래시를 사용하면 `Edit(/secrets/**)`는 규칙의 소스에서 앵커됩니다. `allowed_tools` 또는 `disallowed_tools`를 통해 전달된 규칙의 경우, 이는 세션의 작업 디렉토리를 의미하므로 규칙은 디스크의 `/secrets`를 차단하지 않습니다. [Read 및 Edit 규칙](/docs/ko/permissions#read-and-edit)에서 네 가지 앵커 형식과 설정 파일의 규칙이 어떻게 해결되는지 확인하세요.

<Warning>
  **자동 승인된 도구는 절대 `canUseTool`에 도달하지 않습니다.** `acceptEdits` 또는 `bypassPermissions`에 의해, 또는 허용 규칙에 의해 이전 단계에서 승인된 도구 호출은 `canUseTool` 콜백을 건너뛰므로 거기에 배치한 권한 검사는 해당 도구에 대해 자동으로 무시됩니다. `AskUserQuestion`, [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)로 표시된 MCP 도구, 및 커넥터 도구([조직이 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools))는 허용 규칙이 일치할 때에도 콜백에 도달합니다.

  적용 범위는 항목의 형식에 따라 달라집니다. `Read` 또는 `mcp__github__get_issue`와 같은 단순 이름은 해당 도구에 대한 모든 호출을 자동 승인하는 반면, `Bash(ls *)`와 같은 범위 규칙은 일치하는 호출만 자동 승인하고 다른 `Bash` 호출은 여전히 콜백으로 통과합니다. 모든 도구 호출에서 실행되어야 하는 검사의 경우 [`PreToolUse` 훅](/docs/ko/agent-sdk/hooks)을 사용합니다. 훅은 다른 모든 단계 이전에 실행되며, 훅 거부는 `bypassPermissions` 모드에서도 적용됩니다.
</Warning>

잠금된 에이전트의 경우 `allowedTools`를 `permissionMode: "dontAsk"`와 쌍으로 사용합니다. 나열된 도구는 승인되고, 위의 경고에서 항상 프롬프트하는 도구를 제외하고 다른 모든 항목은 프롬프트 대신 완전히 거부됩니다:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

<Warning>
  **`allowed_tools`는 `bypassPermissions`를 제한하지 않습니다.** `allowed_tools`는 나열한 도구만 사전 승인합니다. 나열되지 않은 도구는 허용 규칙과 일치하지 않으며 권한 모드로 통과하며, 여기서 `bypassPermissions`는 이를 승인합니다. `allowed_tools=["Read"]`를 `permission_mode="bypassPermissions"`와 함께 설정하면 `Bash`, `Write`, `Edit`을 포함한 모든 도구가 여전히 승인됩니다. `bypassPermissions`가 필요하지만 특정 도구를 차단하려면 `disallowed_tools`를 사용합니다.
</Warning>

`.claude/settings.json`에서 허용, 거부 및 요청 규칙을 선언적으로 구성할 수도 있습니다. 이러한 규칙은 `project` 설정 소스가 활성화될 때 읽혀지며, 기본 `query()` 옵션에 대해 활성화됩니다. `setting_sources`(TypeScript: `settingSources`)를 명시적으로 설정하면 적용되도록 `"project"`를 포함합니다. 규칙 구문은 [권한 설정](/docs/ko/settings#permission-settings)을 참조하세요.

<h2 id="permission-modes">
  권한 모드
</h2>

권한 모드는 Claude가 도구를 사용하는 방식에 대한 전역 제어를 제공합니다. `query()`를 호출할 때 권한 모드를 설정하거나 스트리밍 세션 중에 동적으로 변경할 수 있습니다.

<h3 id="available-modes">
  사용 가능한 모드
</h3>

SDK는 다음 권한 모드를 지원합니다:

| 모드                  | 설명          | 도구 동작                                                                                                                                                                                       |
| :------------------ | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | 표준 권한 동작    | 자동 승인 없음; 일치하지 않는 도구는 `canUseTool` 콜백을 트리거합니다                                                                                                                                               |
| `dontAsk`           | 프롬프트 대신 거부  | `allowed_tools` 또는 규칙으로 사전 승인되지 않은 항목은 거부됩니다; 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)하고 사용자 상호작용이 필요한 도구는 사전 승인했더라도 거부됩니다. `canUseTool`은 호출되지 않습니다        |
| `acceptEdits`       | 파일 편집 자동 수락 | 파일 편집 및 [파일 시스템 작업](#accept-edits-mode-acceptedits)(`mkdir`, `rm`, `mv` 등)이 자동으로 승인됩니다                                                                                                      |
| `bypassPermissions` | 권한 확인 무시    | 도구는 권한 프롬프트 없이 실행됩니다. 명시적 [`ask` 규칙](#how-permissions-are-evaluated)이 일치하는 경우, 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)하고 사용자 상호작용이 필요한 도구는 제외됩니다(주의해서 사용) |
| `plan`              | 계획 모드       | Claude는 소스 파일을 편집하지 않고 탐색 및 계획합니다. 파일 편집은 자동으로 승인되지 않으며 `canUseTool` 콜백을 통해 프롬프트합니다                                                                                                         |
| `auto`              | 모델 분류 승인    | 모델 분류기가 각 도구 호출을 승인하거나 거부합니다. [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서 가용성을 참조하세요                                                                                   |

<Warning>
  **하위 에이전트 상속:** 부모가 `bypassPermissions`, `acceptEdits` 또는 `auto`를 사용할 때 모든 하위 에이전트는 해당 모드를 상속하며 하위 에이전트별로 재정의할 수 없습니다. 하위 에이전트는 주 에이전트와 다른 시스템 프롬프트와 덜 제한된 동작을 가질 수 있으므로 `bypassPermissions`를 상속하면 전체 자율 시스템 액세스 권한이 부여됩니다. 명시적 [`ask` 규칙](#how-permissions-are-evaluated), 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), 및 사용자 상호작용이 필요한 도구는 여전히 프롬프트를 강제합니다.
</Warning>

<h3 id="set-permission-mode">
  권한 모드 설정
</h3>

쿼리를 시작할 때 권한 모드를 한 번 설정하거나 세션이 활성화된 동안 동적으로 변경할 수 있습니다.

<Tabs>
  <Tab title="쿼리 시간에">
    쿼리를 생성할 때 `permission_mode`(Python) 또는 `permissionMode`(TypeScript)를 전달합니다. 이 모드는 동적으로 변경되지 않는 한 전체 세션에 적용됩니다.

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

  <Tab title="스트리밍 중">
    `set_permission_mode()`(Python) 또는 `setPermissionMode()`(TypeScript)를 호출하여 세션 중간에 모드를 변경합니다. 새 모드는 모든 후속 도구 요청에 즉시 적용됩니다. 이를 통해 제한적으로 시작하여 신뢰가 구축됨에 따라 권한을 완화할 수 있습니다. 예를 들어 Claude의 초기 접근 방식을 검토한 후 `acceptEdits`로 전환할 수 있습니다.

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
  모드 세부 정보
</h3>

<h4 id="accept-edits-mode-acceptedits">
  편집 수락 모드 (`acceptEdits`)
</h4>

Claude가 프롬프트 없이 코드를 편집할 수 있도록 파일 작업을 자동으로 승인합니다. 다른 도구(예: 파일 시스템 작업이 아닌 Bash 명령)는 여전히 일반 권한이 필요합니다.

**자동 승인 작업:**

* 파일 편집(Edit, Write 도구)
* 파일 시스템 명령: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

둘 다 작업 디렉토리 또는 `additionalDirectories` 내의 경로에만 적용됩니다. 해당 범위 외의 경로 및 보호된 경로에 대한 쓰기는 여전히 프롬프트합니다.

**사용 시기:** Claude의 편집을 신뢰하고 프로토타이핑 중이거나 격리된 디렉토리에서 작업할 때와 같이 더 빠른 반복을 원할 때입니다.

<h4 id="don’t-ask-mode-dontask">
  요청 안 함 모드 (`dontAsk`)
</h4>

모든 권한 프롬프트를 거부로 변환합니다. `allowed_tools`, `settings.json` 허용 규칙 또는 훅으로 사전 승인된 도구는 정상적으로 실행됩니다. 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)하고 사용자 상호작용이 필요한 도구는 허용 규칙이 일치하더라도 거부됩니다. 다른 모든 항목은 `canUseTool`을 호출하지 않고 거부됩니다.

**사용 시기:** 헤드리스 에이전트에 대해 고정된 명시적 도구 표면을 원하고 `canUseTool`이 없을 때의 자동 거부보다 하드 거부를 선호할 때입니다.

<h4 id="bypass-permissions-mode-bypasspermissions">
  권한 무시 모드 (`bypassPermissions`)
</h4>

프롬프트 없이 모든 도구 사용을 자동으로 승인합니다. 훅은 여전히 실행되며 필요한 경우 작업을 차단할 수 있습니다.

<Warning>
  극도의 주의를 기울여 사용하세요. Claude는 이 모드에서 전체 시스템 액세스 권한을 가집니다. 모든 가능한 작업을 신뢰하는 제어된 환경에서만 사용하세요.

  `allowed_tools`는 이 모드를 제한하지 않습니다. 나열한 도구뿐만 아니라 모든 도구가 승인됩니다. 거부 규칙(`disallowed_tools`), 명시적 `ask` 규칙 및 훅은 모드 확인 전에 평가되며 여전히 도구를 차단할 수 있습니다. 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)하고 사용자 상호작용이 필요한 도구는 여전히 `canUseTool` 콜백으로 넘어갑니다.
</Warning>

<h4 id="plan-mode-plan">
  계획 모드 (`plan`)
</h4>

Claude는 코드베이스를 탐색하고 소스 파일을 편집하지 않고 계획을 생성합니다. 읽기 전용 도구는 기본 모드처럼 실행됩니다. 파일 편집은 계획 모드에서 자동으로 승인되지 않으며, 허용 규칙이 일치하더라도 `canUseTool` 콜백을 통해 프롬프트합니다. Claude는 계획을 최종화하기 전에 요구 사항을 명확히 하기 위해 `AskUserQuestion`을 사용할 수 있습니다. [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input#handle-clarifying-questions)에서 이러한 프롬프트 처리를 참조하세요.

**사용 시기:** Claude가 변경 사항을 실행하지 않고 제안하기를 원할 때, 예를 들어 코드 검토 중이거나 변경 사항이 적용되기 전에 승인해야 할 때입니다.

<h2 id="related-resources">
  관련 리소스
</h2>

권한 평가 흐름의 다른 단계의 경우:

* [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input): 대화형 승인 프롬프트 및 명확히 하는 질문
* [훅 가이드](/docs/ko/agent-sdk/hooks): 에이전트 수명 주기의 주요 지점에서 사용자 정의 코드 실행
* [권한 규칙](/docs/ko/settings#permission-settings): `settings.json`의 선언적 허용/거부 규칙
