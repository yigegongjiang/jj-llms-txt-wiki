> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 빠른 시작

> Python 또는 TypeScript Agent SDK를 사용하여 자율적으로 작동하는 AI 에이전트를 구축하기 시작합니다

Agent SDK를 사용하여 코드를 읽고, 버그를 찾고, 수동 개입 없이 모두 자동으로 버그를 수정하는 AI 에이전트를 구축합니다.

**수행할 작업:**

1. Agent SDK를 사용하여 프로젝트 설정
2. 버그가 있는 코드가 포함된 파일 생성
3. 버그를 자동으로 찾고 수정하는 에이전트 실행

<h2 id="prerequisites">
  필수 조건
</h2>

* **Node.js 18+** 또는 **Python 3.10+**
* **Anthropic 계정** ([여기서 가입](https://platform.claude.com/))

<h2 id="setup">
  설정
</h2>

<Steps>
  <Step title="프로젝트 폴더 생성">
    이 빠른 시작을 위한 새 디렉토리를 생성합니다:

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    자신의 프로젝트의 경우 모든 폴더에서 SDK를 실행할 수 있습니다. 기본적으로 해당 디렉토리 및 하위 디렉토리의 파일에 액세스할 수 있습니다.
  </Step>

  <Step title="SDK 설치">
    언어에 맞는 Agent SDK 패키지를 설치합니다:

    <Tabs>
      <Tab title="TypeScript (새 프로젝트)">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        `package.json`에서 `"type": "module"`을 설정하면 에이전트 스크립트가 최상위 `await`를 사용할 수 있으며, [tsx](https://tsx.is)는 TypeScript 파일을 직접 실행합니다.
      </Tab>

      <Tab title="TypeScript (기존 프로젝트)">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.is)는 TypeScript 파일을 직접 실행합니다. 프로젝트가 CommonJS를 사용하는 경우 에이전트 스크립트의 이름을 `agent.ts` 대신 `agent.mts`로 지정합니다. `.mts` 확장자는 tsx가 파일을 ES 모듈로 처리하도록 하므로 전체 프로젝트를 ES 모듈로 변환하지 않고도 최상위 `await`가 작동합니다. 이 빠른 시작의 나중에 생성 및 실행 단계에서 `agent.ts` 대신 `agent.mts`를 사용합니다.
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/)는 가상 환경을 자동으로 처리하는 빠른 Python 패키지 관리자입니다:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        가상 환경을 생성하고 활성화한 다음 패키지를 설치합니다.

        macOS 또는 Linux에서:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Windows에서:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        PowerShell이 실행 정책 오류로 `Activate.ps1`을 차단하는 경우 먼저 `Set-ExecutionPolicy -Scope Process RemoteSigned`를 실행합니다.
      </Tab>
    </Tabs>

    <Note>
      TypeScript SDK는 선택적 종속성으로 플랫폼용 네이티브 Claude Code 바이너리를 번들로 제공하므로 Claude Code를 별도로 설치할 필요가 없습니다.
    </Note>
  </Step>

  <Step title="API 키 설정">
    [Claude 콘솔](https://platform.claude.com/)에서 API 키를 가져온 다음 에이전트를 실행할 셸에서 환경 변수로 설정합니다:

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows (PowerShell)">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    SDK는 에이전트를 실행하는 프로세스의 환경에서 키를 읽습니다. `.env` 파일을 자동으로 로드하지 않습니다. 키를 `.env` 파일에 보관하는 경우 SDK를 호출하기 전에 `dotenv` 패키지 등으로 직접 로드합니다.

    SDK는 또한 타사 API 공급자를 통한 인증을 지원합니다:

    * **Amazon Bedrock**: `CLAUDE_CODE_USE_BEDROCK=1` 환경 변수를 설정하고 AWS 자격 증명을 구성합니다
    * **Claude Platform on AWS**: `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` 및 `ANTHROPIC_AWS_WORKSPACE_ID`를 설정한 다음 AWS 자격 증명을 구성합니다
    * **Google Cloud의 Agent Platform**: `CLAUDE_CODE_USE_VERTEX=1` 환경 변수를 설정하고 Google Cloud 자격 증명을 구성합니다
    * **Microsoft Azure**: `CLAUDE_CODE_USE_FOUNDRY=1` 환경 변수를 설정하고 Azure 자격 증명을 구성합니다

    [Amazon Bedrock](/docs/ko/amazon-bedrock), [Claude Platform on AWS](/docs/ko/claude-platform-on-aws), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry)의 설정 가이드를 참조하여 자세한 내용을 확인합니다.

    <Note>
      이전에 승인되지 않은 경우 Anthropic은 타사 개발자가 claude.ai 로그인 또는 Claude Agent SDK를 기반으로 구축된 에이전트를 포함한 제품에 대한 속도 제한을 제공하는 것을 허용하지 않습니다. 대신 이 문서에 설명된 API 키 인증 방법을 사용하십시오.
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  버그가 있는 파일 생성
</h2>

이 빠른 시작은 코드에서 버그를 찾고 수정할 수 있는 에이전트를 구축하는 과정을 안내합니다. 먼저 에이전트가 수정할 의도적인 버그가 있는 파일이 필요합니다. `my-agent` 디렉토리에 `utils.py`를 생성하고 다음 코드를 붙여넣습니다:

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

이 코드에는 두 가지 버그가 있습니다:

1. `calculate_average([])`는 0으로 나누기 오류로 충돌합니다
2. `get_user_name(None)`은 TypeError로 충돌합니다

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  버그를 찾고 수정하는 에이전트 구축
</h2>

Python SDK를 사용하는 경우 `agent.py`를 생성하거나 TypeScript의 경우 `agent.ts`를 생성합니다. 기존 프로젝트가 CommonJS를 사용하는 경우 `agent.ts` 대신 `agent.mts`를 사용합니다:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

이 코드에는 세 가지 주요 부분이 있습니다:

1. **`query`**: 에이전틱 루프를 생성하는 주요 진입점입니다. 비동기 반복자를 반환하므로 `async for`를 사용하여 Claude가 작동할 때 메시지를 스트리밍합니다. [Python](/docs/ko/agent-sdk/python#query) 또는 [TypeScript](/docs/ko/agent-sdk/typescript#query) SDK 참조에서 전체 API를 참조합니다.

2. **`prompt`**: Claude가 수행할 작업입니다. Claude는 작업을 기반으로 사용할 도구를 파악합니다.

3. **`options`**: 에이전트의 구성입니다. 이 예제는 `allowedTools`를 사용하여 `Read`, `Edit` 및 `Glob`을 사전 승인하고 `permissionMode: "acceptEdits"`를 사용하여 파일 변경을 자동 승인합니다. 다른 옵션에는 `systemPrompt`, `mcpServers` 등이 포함됩니다. [Python](/docs/ko/agent-sdk/python#claudeagentoptions) 또는 [TypeScript](/docs/ko/agent-sdk/typescript#options)의 모든 옵션을 참조합니다.

`async for` 루프는 Claude가 생각하고, 도구를 호출하고, 결과를 관찰하고, 다음에 할 일을 결정할 때 계속 실행됩니다. 각 반복은 메시지를 생성합니다: Claude의 추론, 도구 호출, 도구 결과 또는 최종 결과입니다. SDK는 오케스트레이션(도구 실행, 컨텍스트 관리, 재시도)을 처리하므로 스트림을 사용하기만 하면 됩니다. Claude가 작업을 완료하거나 오류가 발생하면 루프가 종료됩니다.

루프 내의 메시지 처리는 인간이 읽을 수 있는 출력을 필터링합니다. 필터링 없이는 시스템 초기화 및 내부 상태를 포함한 원시 메시지 객체가 표시되며, 이는 디버깅에는 유용하지만 그 외에는 번거롭습니다.

<Note>
  이 예제는 스트리밍을 사용하여 실시간으로 진행 상황을 표시합니다. 실시간 출력이 필요하지 않은 경우(예: 백그라운드 작업 또는 CI 파이프라인의 경우) 모든 메시지를 한 번에 수집할 수 있습니다. 자세한 내용은 [스트리밍 대 단일 턴 모드](/docs/ko/agent-sdk/streaming-vs-single-mode)를 참조합니다.
</Note>

<h3 id="run-your-agent">
  에이전트 실행
</h3>

에이전트가 준비되었습니다. 다음 명령으로 실행합니다:

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    스크립트의 이름을 `agent.mts`로 지정한 경우 `npx tsx agent.mts`를 실행합니다.
  </Tab>

  <Tab title="Python (uv)">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python (pip)">
    가상 환경이 여전히 활성화된 상태에서:

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

실행 후 `utils.py`를 확인합니다. 빈 목록과 null 사용자를 처리하는 방어적 코드가 표시됩니다. 에이전트는 자율적으로:

1. **읽기** `utils.py`를 읽어 코드 이해
2. **분석** 논리를 분석하고 충돌을 일으킬 엣지 케이스 식별
3. **편집** 파일을 편집하여 적절한 오류 처리 추가

이것이 Agent SDK를 다르게 만드는 것입니다: Claude는 구현을 요청하는 대신 도구를 직접 실행합니다.

<Note>
  "API key not found"가 표시되면 에이전트를 실행할 셸에서 `ANTHROPIC_API_KEY` 환경 변수를 설정했는지 확인합니다. SDK는 `.env` 파일을 자동으로 로드하지 않습니다. 자세한 내용은 [전체 문제 해결 가이드](/docs/ko/troubleshooting)를 참조합니다.
</Note>

<h3 id="try-other-prompts">
  다른 프롬프트 시도
</h3>

에이전트가 설정되었으므로 다른 프롬프트를 시도합니다:

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  에이전트 사용자 정의
</h3>

옵션을 변경하여 에이전트의 동작을 수정할 수 있습니다. 다음은 몇 가지 예입니다:

**웹 검색 기능 추가:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Claude에 사용자 정의 시스템 프롬프트 제공:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**터미널에서 명령 실행:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

`Bash`가 활성화된 상태에서 다음을 시도합니다: `"Write unit tests for utils.py, run them, and fix any failures"`

<h2 id="key-concepts">
  주요 개념
</h2>

**도구**는 에이전트가 수행할 수 있는 작업을 제어합니다:

| 도구                                     | 에이전트가 수행할 수 있는 작업 |
| -------------------------------------- | ----------------- |
| `Read`, `Glob`, `Grep`                 | 읽기 전용 분석          |
| `Read`, `Edit`, `Glob`                 | 코드 분석 및 수정        |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | 완전 자동화            |

**권한 모드**는 원하는 인간 감독의 양을 제어합니다:

| 모드                  | 동작                                                                                                                                                                                                        | 사용 사례                    |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| `acceptEdits`       | 파일 편집 및 일반적인 파일 시스템 명령을 자동 승인하고 다른 작업을 요청합니다                                                                                                                                                              | 신뢰할 수 있는 개발 워크플로우        |
| `plan`              | 읽기 전용 도구를 실행합니다. 파일 편집은 자동 승인되지 않으며 `canUseTool` 콜백에 도달합니다                                                                                                                                                | 실행 승인 전 작업 범위 지정         |
| `dontAsk`           | `allowedTools`에 없는 모든 것을 거부합니다. 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools) 및 사용자 상호작용이 필요한 도구는 나열했더라도 거부됩니다                                                                | 잠금된 헤드리스 에이전트            |
| `auto`              | 모델 분류기가 각 도구 호출을 승인하거나 거부합니다                                                                                                                                                                              | 안전 가드레일이 있는 자율 에이전트      |
| `bypassPermissions` | 명시적 [`ask` 규칙](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)이 일치하는 도구, 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools) 및 사용자 상호작용이 필요한 도구를 제외하고 프롬프트 없이 모든 도구를 실행합니다 | 샌드박스 CI, 완전히 신뢰할 수 있는 환경 |
| `default`           | 승인을 처리하기 위해 `canUseTool` 콜백이 필요합니다                                                                                                                                                                        | 사용자 정의 승인 흐름             |

위의 예제는 `acceptEdits` 모드를 사용하며, 이는 파일 작업을 자동 승인하므로 에이전트가 대화형 프롬프트 없이 실행될 수 있습니다. 사용자에게 승인을 요청하려면 `default` 모드를 사용하고 사용자 입력을 수집하는 [`canUseTool` 콜백](/docs/ko/agent-sdk/user-input)을 제공합니다. 더 많은 제어를 위해 [권한](/docs/ko/agent-sdk/permissions)을 참조합니다.

<h2 id="next-steps">
  다음 단계
</h2>

이제 첫 번째 에이전트를 생성했으므로 기능을 확장하고 사용 사례에 맞게 조정하는 방법을 알아봅니다:

* **[권한](/docs/ko/agent-sdk/permissions)**: 에이전트가 수행할 수 있는 작업과 승인이 필요한 시기를 제어합니다
* **[Hooks](/docs/ko/agent-sdk/hooks)**: 도구 호출 전후에 사용자 정의 코드를 실행합니다
* **[세션](/docs/ko/agent-sdk/sessions)**: 컨텍스트를 유지하는 다중 턴 에이전트를 구축합니다
* **[MCP 서버](/docs/ko/agent-sdk/mcp)**: 데이터베이스, 브라우저, API 및 기타 외부 시스템에 연결합니다
* **[호스팅](/docs/ko/agent-sdk/hosting)**: Docker, 클라우드 및 CI/CD에 에이전트를 배포합니다
* **[예제 에이전트](https://github.com/anthropics/claude-agent-sdk-demos)**: 완전한 예제를 참조합니다: 이메일 어시스턴트, 연구 에이전트 등
