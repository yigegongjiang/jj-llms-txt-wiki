> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 권한 구성

> 권한 모드, 훅, 선언적 허용/거부 규칙을 사용하여 에이전트가 도구를 사용하는 방식을 제어합니다.

Claude Agent SDK는 Claude가 도구를 사용하는 방식을 관리하기 위한 권한 제어를 제공합니다. 권한 모드와 규칙을 사용하여 자동으로 허용되는 항목을 정의하고, [`canUseTool` 콜백](/docs/ko/agent-sdk/user-input)을 사용하여 런타임에 다른 모든 항목을 처리합니다.

<h2 id="how-permissions-are-evaluated">
  권한이 평가되는 방식
</h2>

Claude가 도구를 요청할 때 SDK는 다음 순서로 권한을 확인합니다.

<Steps>
  <Step title="Hooks">
    먼저 [hooks](/docs/ko/agent-sdk/hooks)를 실행합니다. Hook은 호출을 완전히 거부하거나 통과시킬 수 있습니다. `allow`를 반환하는 Hook은 아래의 거부 및 확인 규칙을 건너뛰지 않습니다. 이러한 규칙은 Hook 결과와 관계없이 평가됩니다. `PreToolUse` Hook allow는 [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 `rm` 또는 `rmdir` 제거를 승인할 수 없습니다.
  </Step>

  <Step title="거부 규칙">
    `deny` 규칙(`disallowed_tools` 및 [settings.json](/docs/ko/settings-reference#permission-settings)에서)을 확인합니다. 거부 규칙이 일치하면 `bypassPermissions` 모드에서도 도구가 차단됩니다. `Bash`와 같은 단순 이름 거부 규칙은 이 평가가 시작되기 전에 Claude의 컨텍스트에서 도구를 제거하므로 `Bash(rm *)`과 같은 범위가 지정된 규칙만 이 단계에서 확인됩니다.
  </Step>

  <Step title="확인 규칙">
    [settings.json](/docs/ko/settings-reference#permission-settings)에서 `ask` 규칙을 확인합니다. ask 규칙이 일치하면 `bypassPermissions` 모드에서도 호출이 확인을 위해 [`canUseTool` 콜백](/docs/ko/agent-sdk/user-input)으로 전달됩니다.

    사용자 상호작용이 필요한 도구는 동일하게 작동합니다. `AskUserQuestion` 및 서버가 [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)을 설정하는 MCP 도구는 allow 규칙이 일치할 때도 항상 콜백으로 전달됩니다. `dontAsk` 모드에서는 이 모드가 절대 프롬프트하지 않기 때문에 두 경우 모두 거부됩니다. MCP 주석에는 Claude Code v2.1.199 이상이 필요합니다.

    조직이 `ask`로 설정한 [claude.ai connector](/docs/ko/mcp#organization-controls-on-connector-tools) 도구도 이 단계에서 흐름을 떠납니다. 모든 호출은 `bypassPermissions` 모드에서도, allow 규칙이 일치할 때도 콜백으로 전달됩니다. 콜백은 `Your organization requires approval for this tool` 이유를 받습니다. `dontAsk` 모드에서는 이 모드가 절대 프롬프트하지 않기 때문에 호출이 거부됩니다.
  </Step>

  <Step title="권한 모드">
    활성 [권한 모드](#permission-modes)를 적용합니다.

    * `bypassPermissions` 모드에서 Claude Code는 이 단계에 도달하는 모든 것을 승인합니다. 단, [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 `rm` 및 `rmdir` 제거는 대신 전달됩니다.
    * `acceptEdits` 모드에서 Claude Code는 [Accept edits mode](#accept-edits-mode-acceptedits)에 나열된 파일 작업을 승인합니다.
    * `plan` 모드에서 Claude Code는 allow 규칙과 관계없이 파일 편집 및 셸 쓰기 도구를 `canUseTool` 콜백으로 보내므로 계획 중에 쓰기 작업을 자동 승인할 수 없습니다.
    * 다른 모드에서는 요청이 전달됩니다.
  </Step>

  <Step title="허용 규칙">
    `allow` 규칙(`allowed_tools` 및 settings.json에서)을 확인합니다. 규칙이 일치하면 도구가 승인됩니다. 도구가 자체적으로 승인하는 호출도 이 단계에서 규칙 없이 해결됩니다. 예를 들어 작업 디렉토리 내의 파일 읽기 또는 [읽기 전용 Bash 명령](/docs/ko/permissions#read-only-commands)입니다. `rm` 및 `rmdir` 제거는 [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 경우 allow 규칙으로 절대 승인되지 않습니다. 프롬프트하는 모드에서는 콜백에 도달하고, Claude Code v2.1.218 이상의 `auto` 모드에서는 [분류기](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)로 이동하며, `dontAsk` 모드에서는 거부됩니다.
  </Step>

  <Step title="canUseTool 콜백">
    위의 어느 것도 해결하지 못하면 결정을 위해 [`canUseTool` 콜백](/docs/ko/agent-sdk/user-input)을 호출합니다. `dontAsk` 모드에서는 이 단계를 건너뛰고 도구가 거부됩니다.

    TypeScript SDK에서 [`permissionPrompts: 'none'`](/docs/ko/agent-sdk/typescript#options)을 설정하면 콜백이 이 단계에서 호출되지 않습니다. [`PermissionRequest` hook](/docs/ko/hooks#permissionrequest)은 여전히 결정할 기회를 얻으며, 그렇지 않으면 Claude Code가 호출을 거부합니다. 이 옵션에는 Claude Code v2.1.259 이상이 필요합니다.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="위의 단계와 일치하는 6단계 권한 평가 흐름의 다이어그램: 도구 요청이 hooks, 거부 규칙, 확인 규칙, 권한 모드, 허용 규칙 및 canUseTool을 통과합니다. Hooks, 거부 규칙 및 canUseTool은 Blocked로 라우팅할 수 있습니다. 권한 모드 bypass, 허용 규칙 및 canUseTool은 Execute로 라우팅할 수 있습니다. 확인 규칙은 canUseTool로 라우팅합니다." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="위의 단계와 일치하는 6단계 권한 평가 흐름의 다이어그램: 도구 요청이 hooks, 거부 규칙, 확인 규칙, 권한 모드, 허용 규칙 및 canUseTool을 통과합니다. Hooks, 거부 규칙 및 canUseTool은 Blocked로 라우팅할 수 있습니다. 권한 모드 bypass, 허용 규칙 및 canUseTool은 Execute로 라우팅할 수 있습니다. 확인 규칙은 canUseTool로 라우팅합니다." width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

TypeScript SDK가 평가 순서를 자동 승인하도록 예상하는 구성에서 `canUseTool` 콜백을 전달하면 쿼리가 구성될 때 SDK는 Node.js 프로세스 경고를 한 번 내보냅니다. 경고의 코드는 `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`입니다. 두 가지 구성이 이를 트리거합니다.

* `permissionMode: 'bypassPermissions'`는 [모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)을 제외하고 권한 모드 단계에 도달하는 모든 호출을 자동 승인합니다.
* `"Read"`와 같은 각 단순 `allowedTools` 항목은 [모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)을 제외하고 콜백이 참조되기 전에 전체 도구를 자동 승인합니다.

`Bash(ls *)`와 같은 지정자가 있는 항목과 `acceptEdits` 모드는 이를 트리거하지 않으며, 설정 파일에서 오는 허용 규칙은 확인에 표시되지 않습니다.

`process.on('warning', ...)`으로 수신하고 코드를 일치시켜 로깅하거나 억제합니다. 모드와 규칙과 관계없이 모든 도구 호출을 제어하려면 대신 [`PreToolUse` hook](/docs/ko/agent-sdk/hooks)을 사용합니다.

이 페이지는 **허용 및 거부 규칙** 및 **권한 모드**에 중점을 둡니다. 다른 단계의 경우:

* **Hooks:** 도구 요청을 허용, 거부 또는 수정하는 사용자 정의 코드를 실행합니다. [Hooks로 실행 제어](/docs/ko/agent-sdk/hooks)를 참조하세요.
* **canUseTool 콜백:** 이전 단계에서 호출을 해결하지 못할 때 런타임에 사용자에게 승인을 요청합니다. [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input)를 참조하세요.

<h2 id="allow-and-deny-rules">
  허용 및 거부 규칙
</h2>

`allowed_tools` 및 `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`)는 위의 평가 흐름에서 허용 및 거부 규칙 목록에 항목을 추가합니다. [작업 추적 도구](/docs/ko/agent-sdk/todo-tracking#model-availability) 중 하나를 `allowed_tools`에 이름 지으면 Claude Code도 세션을 옵트인합니다. `allowed_tools`에 나열되지 않은 다른 도구는 여전히 Claude에서 사용 가능하며, 승인이 필요한 호출은 권한 모드로 넘어갑니다. 거부 규칙은 도구의 이름을 지정하는지 또는 도구 내의 패턴을 범위 지정하는지에 따라 다르게 작동합니다.

| 옵션                                | 효과                                                                                                                                                                         |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` 및 `Grep`은 자동 승인됩니다. 여기에 나열되지 않은 다른 도구는 여전히 존재하며, 승인이 필요한 호출은 권한 모드 및 `canUseTool`로 넘어갑니다.                                                                           |
| `disallowed_tools=["Bash"]`       | `Bash` 도구 정의가 요청에서 제거됩니다. Claude는 도구를 보지 못하며 시도할 수 없습니다.                                                                                                                   |
| `disallowed_tools=["Bash(rm *)"]` | `Bash`는 사용 가능한 상태로 유지됩니다. `rm *` [작성된 대로](/docs/ko/permissions#bash-rule-limits) 일치하는 호출은 `bypassPermissions`를 포함한 모든 권한 모드에서 거부됩니다. `/bin/rm`을 포함한 다른 `Bash` 호출은 권한 모드로 넘어갑니다. |
| `disallowed_tools=["*"]`          | 모든 도구 정의가 요청에서 제거됩니다. 도구 이름 글롭은 거부 규칙에서 지원됩니다: `"*"`는 모든 도구와 일치하고 `"mcp__*"`는 모든 서버의 모든 MCP 도구와 일치합니다.                                                                     |

허용 규칙은 리터럴 `mcp__<server>__` 접두사 이후에만 도구 이름 글롭을 허용합니다. 서버 세그먼트는 글롭이 없어야 하므로 규칙이 구성한 특정 서버의 이름을 지정합니다: `mcp__puppeteer__*`는 `puppeteer` 서버의 모든 도구와 일치하고, `mcp__github__get_*`는 해당 `get_` 도구와 일치합니다. `allowed_tools=["*"]` 또는 `allowed_tools=["mcp__*"]`와 같은 앵커되지 않은 항목은 시작 경고와 함께 무시되며 아무것도 자동 승인하지 않습니다.

`Read` 및 `Edit`에 대한 범위 지정 규칙은 경로 패턴을 사용합니다. `Edit(path)` 규칙은 `Write` 및 `NotebookEdit`을 포함하여 파일을 쓰는 모든 기본 제공 도구를 관리합니다. `Write(path)` 규칙은 파일 권한 검사에 의해 일치되지 않습니다.

절대 파일 시스템 경로에는 `//path`를 사용합니다: `Edit(//secrets/**)` 거부 규칙은 디스크의 `/secrets` 아래 어디든지 쓰기를 차단합니다. 단일 슬래시를 사용하면 `Edit(/secrets/**)`는 규칙의 소스에서 앵커됩니다. `allowed_tools` 또는 `disallowed_tools`를 통해 전달된 규칙의 경우, 이는 세션의 작업 디렉토리를 의미하므로 규칙은 디스크의 `/secrets`를 차단하지 않습니다. 네 가지 앵커 형식과 설정 파일의 규칙이 어떻게 해결되는지는 [Read 및 Edit 규칙](/docs/ko/permissions#read-and-edit)을 참조하십시오.

<Warning>
  **자동 승인된 도구는 `canUseTool`에 도달하지 않습니다.** `acceptEdits` 또는 `bypassPermissions`에 의해 또는 허용 규칙에 의해 이전 단계에서 승인된 도구 호출은 `canUseTool` 콜백을 건너뛰므로, 거기에 넣은 권한 검사는 해당 도구에 대해 자동으로 무시됩니다. `AskUserQuestion`, [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)로 표시된 MCP 도구, 커넥터 도구 [조직이 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), 그리고 [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 `rm` 및 `rmdir` 제거는 허용 규칙이 일치하더라도 여전히 콜백에 도달합니다. `auto` 모드에서 중요 경로 제거는 콜백 대신 [분류기](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)로 이동하고, 여기에 나열된 다른 호출은 여전히 콜백에 도달합니다. 분류기 라우팅에는 Claude Code v2.1.218 이상이 필요합니다. `dontAsk` 모드에서 이러한 호출은 콜백을 호출하지 않고 대신 거부됩니다.

  적용 범위는 항목의 형식에 따라 다릅니다: `Read` 또는 `mcp__github__get_issue`와 같은 단순 이름은 위의 예외를 제외하고 해당 도구에 대한 모든 호출을 자동 승인하고, `Bash(npm test *)`와 같은 범위 지정 규칙은 일치하는 호출만 자동 승인하며, 승인이 필요한 다른 `Bash` 호출은 여전히 콜백으로 넘어갑니다. 모든 도구 호출에서 실행되어야 하는 검사의 경우 [`PreToolUse` 훅](/docs/ko/agent-sdk/hooks)을 사용합니다: 훅은 다른 모든 단계 전에 실행되고, 훅 거부는 `bypassPermissions` 모드에서도 적용됩니다.
</Warning>

잠금된 에이전트의 경우 `allowedTools`를 `permissionMode: "dontAsk"`와 쌍으로 지정합니다:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

나열된 도구는 [모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)을 제외하고 승인되며, 프롬프트를 표시할 다른 모든 호출은 대신 거부됩니다. `default` 모드에서 승인이 필요 없는 호출은 나열 여부와 관계없이 실행됩니다. 예를 들어 [읽기 전용 Bash 명령](/docs/ko/permissions#read-only-commands), 실행 전에 묻지 않는 `Agent`와 같은 도구, 작업 디렉토리 내의 파일 읽기 등입니다. 도구를 Claude의 범위에서 완전히 제거하려면 `disallowedTools`에 단순 이름을 추가합니다.

<Warning>
  **`allowed_tools`는 `bypassPermissions`를 제한하지 않습니다.** `allowed_tools`는 나열한 도구를 사전 승인합니다. 나열되지 않은 다른 도구는 허용 규칙과 일치하지 않으며 권한 모드로 넘어가고, 여기서 `bypassPermissions`는 이들을 승인합니다. `allowed_tools=["Read"]`를 `permission_mode="bypassPermissions"`와 함께 설정하면 `Bash`, `Write`, `Edit`을 포함한 모든 도구를 여전히 승인합니다. `bypassPermissions`가 필요하지만 특정 도구를 차단하려면 `disallowed_tools`를 사용합니다.
</Warning>

`.claude/settings.json`에서 허용, 거부 및 요청 규칙을 선언적으로 구성할 수도 있습니다. 이러한 규칙은 `project` 설정 소스가 활성화될 때 읽혀지며, 기본 `query()` 옵션에 대해 활성화됩니다. `setting_sources` (TypeScript: `settingSources`)를 명시적으로 설정하면 적용되도록 `"project"`를 포함합니다. 규칙 구문은 [권한 설정](/docs/ko/settings-reference#permission-settings)을 참조하십시오.

<h2 id="permission-modes">
  권한 모드
</h2>

권한 모드는 Claude가 도구를 사용하는 방식을 전역적으로 제어합니다. `query()`를 호출할 때 권한 모드를 설정하거나 스트리밍 세션 중에 동적으로 변경할 수 있습니다.

<h3 id="available-modes">
  사용 가능한 모드
</h3>

SDK는 다음 권한 모드를 지원합니다.

| 모드                  | 설명          | 도구 동작                                                                                                                                                                                                                                                                                                                  |
| :------------------ | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | 표준 권한 동작    | 모드 기반 자동 승인 없음. 승인이 필요하고 허용 규칙과 일치하지 않는 호출은 `canUseTool` 콜백을 트리거합니다                                                                                                                                                                                                                                                    |
| `dontAsk`           | 프롬프트 대신 거부  | 그렇지 않으면 프롬프트를 표시할 모든 호출이 거부됩니다. `allowed_tools` 또는 규칙으로 승인된 호출과 `default` 모드에서 승인이 필요 없는 호출은 실행되며, 조직에서 [`ask`](/docs/ko/mcp#organization-controls-on-connector-tools)로 설정한 커넥터 도구와 사용자 상호작용이 필요한 도구, 그리고 [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 `rm` 및 `rmdir` 제거는 사전 승인했더라도 거부됩니다. `canUseTool`은 호출되지 않습니다 |
| `acceptEdits`       | 파일 편집 자동 수락 | 파일 편집 및 [파일시스템 작업](#accept-edits-mode-acceptedits)(`mkdir`, `rm`, `mv` 등)이 자동으로 승인됩니다                                                                                                                                                                                                                                  |
| `bypassPermissions` | 권한 확인 무시    | [모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)을 제외하고 도구가 권한 프롬프트 없이 실행됩니다. 주의해서 사용하세요                                                                                                                                                                                                         |
| `plan`              | 계획 모드       | Claude는 소스 파일을 편집하지 않고 탐색 및 계획을 수행합니다. 파일 편집은 자동 승인되지 않으며 `canUseTool` 콜백을 통해 프롬프트됩니다                                                                                                                                                                                                                                  |
| `auto`              | 모델 분류 승인    | 모델 분류기가 권한 프롬프트를 승인하거나 거부합니다. 가용성은 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 참조하세요                                                                                                                                                                                                               |

<Warning>
  **하위 에이전트 상속:** 하위 에이전트는 부모 세션의 권한 모드에서 실행됩니다. 단, [`AgentDefinition`](/docs/ko/agent-sdk/typescript#agentdefinition)에서 `permissionMode`를 설정하고 부모 세션이 `default`, `dontAsk` 또는 `plan` 모드에 있는 경우는 예외입니다. 이 경우에도 Claude Code는 `"bypassPermissions"` 값을 적용하지 않습니다. 하위 에이전트는 부모 세션 자체가 `bypassPermissions` 모드에 있을 때만 `bypassPermissions` 모드에서 실행됩니다. `bypassPermissions` 예외는 Claude Code v2.1.267 이상이 필요합니다.

  하위 에이전트는 주 에이전트와 다른 시스템 프롬프트를 가질 수 있으며 덜 제한된 동작을 할 수 있으므로, `bypassPermissions` 상속은 전체 자율 시스템 액세스 권한을 부여합니다. [모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)은 여전히 적용됩니다.
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
    `set_permission_mode()`(Python) 또는 `setPermissionMode()`(TypeScript)를 호출하여 세션 중간에 모드를 변경합니다. 새 모드는 모든 후속 도구 요청에 즉시 적용됩니다. 이를 통해 제한적으로 시작하여 신뢰가 쌓임에 따라 권한을 완화할 수 있습니다. 예를 들어 Claude의 초기 접근 방식을 검토한 후 `acceptEdits`로 전환할 수 있습니다.

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
  편집 수락 모드(`acceptEdits`)
</h4>

파일 작업을 자동 승인하여 Claude가 프롬프트 없이 코드를 편집할 수 있도록 합니다. 다른 도구(예: 파일시스템 작업이 아닌 Bash 명령)는 여전히 일반 권한이 필요합니다.

**자동 승인 작업:**

* 파일 편집(Edit, Write 도구)
* 파일시스템 명령: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

둘 다 작업 디렉토리 또는 `additionalDirectories` 내의 경로에만 적용됩니다. `acceptEdits` 모드에서 Claude Code는 Claude가 다음을 수행할 때 요청을 자동 승인하지 않습니다.

* 해당 범위 외의 경로에서 작업
* 보호된 경로에 쓰기
* `rm` 또는 `rmdir`로 [중요 경로](/docs/ko/permission-modes#critical-paths) 제거

**사용 시기:** Claude의 편집을 신뢰하고 더 빠른 반복을 원할 때(예: 프로토타이핑 중이거나 격리된 디렉토리에서 작업할 때).

<h4 id="don’t-ask-mode-dontask">
  묻지 않기 모드(`dontAsk`)
</h4>

`canUseTool`을 호출하지 않고 모든 권한 프롬프트를 거부로 변환합니다. `allowed_tools`, `settings.json` 허용 규칙 또는 훅으로 사전 승인된 도구와 파일 읽기(작업 디렉토리 내) 및 `Agent` 호출과 같이 `default` 모드에서 승인이 필요 없는 호출은 정상적으로 실행됩니다. 조직에서 [`ask`](/docs/ko/mcp#organization-controls-on-connector-tools)로 설정한 커넥터 도구, 사용자 상호작용이 필요한 도구, 그리고 [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 `rm` 및 `rmdir` 제거는 허용 규칙이 일치하더라도 거부됩니다. `PreToolUse` 훅 허용도 중요 경로 제거를 해제하지 않습니다.

**사용 시기:** 헤드리스 에이전트에 대해 고정된 명시적 도구 표면을 원하고 `canUseTool`이 없는 것에 대한 자동 의존보다 명확한 거부를 선호할 때.

<h4 id="bypass-permissions-mode-bypasspermissions">
  권한 무시 모드(`bypassPermissions`)
</h4>

아래 나열된 경우를 제외하고 프롬프트 없이 도구 사용을 자동 승인합니다. 훅은 여전히 실행되며 필요한 경우 작업을 차단할 수 있습니다. Linux 및 macOS에서 Claude Code는 [인식된 샌드박스](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode) 외부에서 root 또는 `sudo` 상태로 이 모드에서 시작하기를 거부하며, 첫 번째 턴 전에 쿼리가 실패합니다.

<Warning>
  극도의 주의를 기울여 사용하세요. Claude는 이 모드에서 전체 시스템 액세스 권한을 가집니다. 모든 가능한 작업을 신뢰하는 제어된 환경에서만 사용하세요.

  `allowed_tools`는 이 모드를 제한하지 않습니다. 나열한 도구뿐만 아니라 모든 도구가 승인됩니다. 다음 제어가 여전히 적용됩니다.

  * 거부 규칙, 명시적 `ask` 규칙 및 훅은 모드 확인 전에 평가되며 여전히 도구를 차단할 수 있습니다.
  * 조직에서 [`ask`](/docs/ko/mcp#organization-controls-on-connector-tools)로 설정한 커넥터 도구, 사용자 상호작용이 필요한 도구, 그리고 [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 `rm` 및 `rmdir` 제거는 여전히 `canUseTool` 콜백으로 전달됩니다.
  * [세션 간 메시징 보안 조치](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode)는 여전히 적용됩니다.
</Warning>

<h4 id="plan-mode-plan">
  계획 모드(`plan`)
</h4>

Claude는 소스 파일을 편집하지 않고 코드베이스를 탐색하고 계획을 생성합니다. 읽기 전용 도구는 `default` 권한 모드에서와 같이 실행됩니다.

파일 편집은 허용 규칙이 일치하더라도 계획 모드에서 자동 승인되지 않습니다. 대신 `canUseTool` 콜백을 통해 프롬프트됩니다. Claude Code v2.1.212 이상에서는 `touch` 및 `rm`과 같이 파일을 수정하는 셸 명령도 동일한 방식으로 `canUseTool` 콜백에 도달합니다.

`allowDangerouslySkipPermissions: true`를 `permissionMode: 'plan'`과 함께 설정하면 파일 편집 및 파일을 수정하는 셸 명령은 여전히 `canUseTool` 콜백에 도달합니다. 이 옵션을 사용하면 나중에 `setPermissionMode()`로 `bypassPermissions`로 전환할 수 있습니다.

Claude는 계획을 최종화하기 전에 요구 사항을 명확히 하기 위해 `AskUserQuestion`을 사용할 수 있습니다. 이러한 프롬프트 처리에 대해서는 [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input#handle-clarifying-questions)를 참조하세요.

**사용 시기:** Claude가 변경 사항을 실행하지 않고 제안하기를 원할 때(예: 코드 검토 중이거나 변경 사항이 적용되기 전에 승인해야 할 때).

<h2 id="related-resources">
  관련 리소스
</h2>

권한 평가 흐름의 다른 단계들:

* [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input): 대화형 승인 프롬프트 및 명확히 하는 질문
* [Hooks 가이드](/docs/ko/agent-sdk/hooks): 에이전트 라이프사이클의 주요 지점에서 사용자 정의 코드 실행
* [권한 규칙](/docs/ko/settings-reference#permission-settings): `settings.json`의 선언적 허용/거부 규칙
