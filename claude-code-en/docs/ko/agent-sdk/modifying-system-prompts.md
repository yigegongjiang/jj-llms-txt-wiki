> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 시스템 프롬프트 수정

> `claude_code` 프리셋과 사용자 정의 시스템 프롬프트 중에서 선택하고, CLAUDE.md, 출력 스타일, append, 또는 완전히 사용자 정의된 프롬프트로 동작을 사용자 정의합니다.

시스템 프롬프트는 Claude의 동작, 기능 및 응답 스타일을 정의합니다. CLI 또는 IDE와 같은 코딩 도구에서 인간이 작업을 감시하고 조종하는 경우 `claude_code` 프리셋으로 시작합니다. 다른 표면, 정체성 또는 권한 모델을 가진 에이전트의 경우 자신의 프롬프트를 작성합니다.

<h2 id="how-system-prompts-work">
  시스템 프롬프트 작동 방식
</h2>

시스템 프롬프트는 대화 전체에서 Claude의 동작 방식을 형성하는 초기 명령 집합입니다. Agent SDK에는 이에 대한 세 가지 시작점이 있습니다:

* **최소 기본값**: TypeScript에서 `systemPrompt`를 설정하지 않거나 Python에서 `system_prompt`를 설정하지 않으면, SDK는 도구 호출을 다루지만 `claude_code` 프리셋의 나머지 콘텐츠(보안 및 안전 명령과 작업 디렉토리 및 환경에 대한 컨텍스트 포함)를 생략하는 최소 프롬프트를 사용합니다. 이는 기본적으로 Claude Code 시스템 프롬프트를 사용하는 `claude -p`와 다릅니다. CLI에서 마이그레이션하고 일치하는 동작을 원하면 `claude_code` 프리셋을 설정합니다.
* **`claude_code` 프리셋**: Claude Code CLI가 사용하는 시스템 프롬프트로, 도구 사용 명령, 보안 및 안전 명령, 작업 디렉토리 및 환경에 대한 컨텍스트를 포함합니다. TypeScript에서 `systemPrompt: { type: "preset", preset: "claude_code" }`를 설정하거나 Python에서 `system_prompt={"type": "preset", "preset": "claude_code"}`를 설정하고, 선택적으로 `append`를 사용하여 끝에 자신의 명령을 추가합니다.
* **사용자 정의 문자열**: 직접 작성한 프롬프트입니다. SDK는 제공하는 것만 전송합니다.

<h3 id="decide-on-a-starting-point">
  시작점 결정
</h3>

결정 요소는 에이전트가 Claude Code와 얼마나 유사한지입니다: 저장소에서 작동하는 코딩 에이전트로, 인간이 스트리밍 출력을 보고 작업을 조종합니다. 제품이 그것으로부터 멀어질수록, 자신의 프롬프트를 더 많이 작성하고 싶을 것입니다.

| 구축 중인 것                                                     | 사용                             | 얻을 수 있는 것                                                    |
| :---------------------------------------------------------- | :----------------------------- | :----------------------------------------------------------- |
| 인간이 보고 조종하는 CLI 또는 IDE와 같은 코딩 도구이며, Claude Code의 기본값이 원하는 것 | `claude_code` 프리셋              | Claude Code 프롬프트(도구 지침, 안전 규칙, 환경 컨텍스트 포함)                   |
| 동일한 종류의 도구에 코딩 표준, 출력 형식 또는 도메인 컨텍스트와 같은 제품별 규칙 추가          | `append`가 있는 `claude_code` 프리셋 | 위의 모든 것에 프리셋 후에 추가된 명령. 제거되는 것이 없으므로 이것이 가장 낮은 위험의 사용자 정의입니다 |
| 다른 표면, 정체성 또는 권한 모델을 가진 에이전트, 또는 비코딩 에이전트                   | 사용자 정의 프롬프트 문자열                | 작성한 것만. 에이전트가 여전히 필요로 하는 도구 지침 및 안전 명령을 교체할 책임이 있습니다         |
| 에이전트 페르소나가 없는 얇은 도구 호출 루프로, 사용자 프롬프트에서 모든 동작을 제공합니다         | `systemPrompt` 옵션 없음           | 최소 기본값: 도구 호출 지원 및 그 이상 없음                                   |

"Claude Code와 다름"은 일반적으로 다음 중 하나를 의미합니다:

* **다른 표면**: 출력이 이를 트리거한 사람에 의해 터미널에서 읽혀지지 않습니다. 채팅 UI, 구조화된 출력 소비자 및 비코딩 자동화는 각각 출력이 렌더링되고 검토되는 방식과 일치하는 프롬프트가 필요합니다. CI 작업이 린트 오류를 수정하거나 diff를 검토하는 것과 같은 무인 코딩 자동화는 작업 자체가 프리셋이 작성된 것이기 때문에 여전히 프리셋에 맞습니다.
* **다른 정체성**: 에이전트는 자신을 Claude Code로 제시하지 않아야 합니다. 지원 봇, 데이터 분석 어시스턴트 또는 도메인별 에이전트는 자신의 이름, 범위 및 페르소나가 필요합니다.
* **다른 권한 모델**: 에이전트는 인간이 각 단계를 승인하지 않고 자율적으로 실행되거나 좁은 리소스 집합에서 작동합니다. Claude Code의 프롬프트는 인간이 전체 도구 세트에 액세스할 수 있는 루프에 있다고 가정합니다.
* **비코딩 작업**: Claude Code의 프롬프트 대부분은 코딩 지침입니다. 연구, 콘텐츠 또는 운영 에이전트의 경우, 해당 지침은 실제로 필요한 명령과 경쟁합니다.

[비교 표](#compare-the-four-approaches)는 각 사용자 정의 방법이 보존하는 것을 보여줍니다.

<h2 id="customize-agent-behavior">
  에이전트 동작 사용자 정의
</h2>

`append`와 사용자 정의 프롬프트 문자열은 각각 시스템 프롬프트를 직접 변경하며, 출력 스타일은 Claude Code가 모든 응답에 대해 Claude에 제공하는 명령을 변경합니다. CLAUDE.md는 다른 경로를 따릅니다: SDK가 이를 읽고 그 내용을 프로젝트 컨텍스트로 대화에 주입하므로, 선택한 시스템 프롬프트와 함께 동작을 형성합니다. [Skills](/docs/ko/agent-sdk/skills), [hooks](/docs/ko/agent-sdk/hooks), 그리고 [permissions](/docs/ko/agent-sdk/permissions)도 시스템 프롬프트 외부에서 동작을 형성하며 자체 페이지에서 다룹니다.

<h3 id="claude-md-files-for-project-level-instructions">
  프로젝트 수준 명령을 위한 CLAUDE.md 파일
</h3>

CLAUDE.md 파일은 Claude에 지속적인 프로젝트 컨텍스트와 명령을 제공합니다. SDK는 그 내용을 대화에 주입하고 시스템 프롬프트는 건드리지 않으므로, 모든 시스템 프롬프트 구성과 함께 작동합니다. CLAUDE.md에 무엇을 넣을지, 어디에 배치할지, 그리고 효과적인 명령을 작성하는 방법에 대해서는 [When to add to CLAUDE.md](/docs/ko/memory#when-to-add-to-claude-md)와 [How Claude remembers your project](/docs/ko/memory)의 나머지 부분을 참조하십시오. 이 섹션은 SDK에 특정한 내용을 다룹니다: CLAUDE.md가 로드되는 방식입니다.

SDK는 일치하는 설정 소스가 활성화될 때 CLAUDE.md를 읽습니다: `'project'`는 작업 디렉토리에서 `CLAUDE.md` 또는 `.claude/CLAUDE.md`를 로드하고, `'user'`는 `~/.claude/CLAUDE.md`를 로드합니다. 기본 `query()` 옵션은 두 소스를 모두 활성화하므로 CLAUDE.md가 자동으로 로드됩니다. TypeScript에서 `settingSources`를 명시적으로 설정하거나 Python에서 `setting_sources`를 명시적으로 설정하면, 필요한 소스를 포함하십시오. CLAUDE.md 로딩은 `claude_code` 프리셋이 아닌 설정 소스에 의해 제어됩니다.

<h4 id="load-claude-md-with-the-sdk">
  SDK와 함께 CLAUDE.md 로드
</h4>

CLAUDE.md를 로드하려면, `settingSources`를 CLAUDE.md가 있는 수준을 포함하도록 설정하십시오. 아래 예제는 프로젝트 수준 CLAUDE.md를 `claude_code` 프리셋과 함께 로드하므로, Claude는 전체 코딩 에이전트 프롬프트와 프로젝트의 규칙을 모두 가지고 있습니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []


  async def main():
      async for message in query(
          prompt="Add a new React component for user profiles",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",  # Use Claude Code's system prompt
              },
              setting_sources=["project"],  # Loads CLAUDE.md from project
          ),
      ):
          messages.append(message)


  asyncio.run(main())

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

예제를 실행하면, SDK는 Claude가 작업할 때 메시지를 스트리밍합니다: 시스템 초기화 메시지, 어시스턴트 메시지, 도구 결과를 전달하는 사용자 메시지, 그리고 세션 결과를 포함하는 최종 결과 메시지입니다.

CLAUDE.md는 프로젝트의 모든 세션에서 지속되며, git을 통해 팀과 공유되고, 코드 변경 없이 자동으로 검색됩니다. 빈 `settingSources` 배열을 전달하면 로드되지 않습니다.

<h3 id="output-styles-for-persistent-configurations">
  지속적인 구성을 위한 출력 스타일
</h3>

출력 스타일은 Claude의 역할, 톤, 그리고 출력 형식을 변경하는 저장된 명령 집합입니다. 마크다운 파일로 저장되며 세션 및 프로젝트 전체에서 재사용할 수 있습니다.

<h4 id="create-an-output-style">
  출력 스타일 생성
</h4>

출력 스타일은 [frontmatter](/docs/ko/output-styles#frontmatter)에 메타데이터가 있는 마크다운 파일이며, 그 뒤에 프롬프트 내용이 있습니다. 모든 프로젝트에서 사용 가능한 사용자 수준 스타일의 경우 `~/.claude/output-styles/`에 저장하거나, 팀과 커밋하고 공유할 수 있는 프로젝트 수준 스타일의 경우 저장소의 `.claude/output-styles/`에 저장하십시오.

사용자 정의 출력 스타일은 `claude_code` 프리셋의 소프트웨어 엔지니어링 명령을 자신의 명령으로 바꿉니다. 이를 유지하고 명령을 그 위에 계층화하려면, frontmatter에서 `keep-coding-instructions: true`를 설정하십시오. 이러한 명령은 Claude Code의 전체 시스템 프롬프트에만 있으므로, 이 설정은 [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/ko/env-vars#variables)로 켜거나 끌 수 있는 더 짧은 시스템 프롬프트의 세션에는 영향을 주지 않습니다. 에이전트가 여전히 소프트웨어 엔지니어링 작업을 수행할 때 이를 유지하십시오. 역할을 완전히 바꿀 때는 이를 제외하십시오.

아래 예제는 코딩 명령을 유지하는 코드 리뷰 담당자 페르소나를 정의합니다. 코드 리뷰는 Claude Code의 보안 및 코드 품질 지침의 이점을 여전히 얻기 때문입니다. 프로젝트 전체에서 사용 가능하도록 `~/.claude/output-styles/code-reviewer.md`로 저장하십시오:

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  출력 스타일 활성화
</h4>

생성되면, 다음을 통해 출력 스타일을 활성화합니다:

* **CLI**: `/output-style <style>`을 실행합니다. 예를 들어 `/output-style concise`를 실행하거나, `/config`를 실행하고 하나를 선택합니다. `/output-style` 명령은 Claude Code v2.1.269 이상이 필요합니다.
* **설정**: `.claude/settings.local.json`에서 `outputStyle`을 설정합니다
* **TypeScript SDK**: `query()`에 전달된 인라인 `settings` 객체 내에서 `outputStyle`을 설정하거나, `settings`가 이를 설정하는 설정 파일을 가리키도록 합니다. `outputStyle`은 최상위 `Options` 필드가 아닙니다:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Python SDK에서는 `settings` 옵션을 통해 `outputStyle`을 설정하며, 이는 `'{"outputStyle": "Explanatory"}'`와 같은 JSON 문자열 또는 이를 설정하는 설정 파일의 경로를 사용합니다.

**SDK 사용자를 위한 참고:** 출력 스타일은 `settingSources: ['user']` 또는 `settingSources: ['project']`(TypeScript) / `setting_sources=["user"]` 또는 `setting_sources=["project"]`(Python)을 옵션에 포함할 때 로드됩니다.

<h3 id="append-to-the-claude_code-preset">
  `claude_code` 프리셋에 추가
</h3>

Claude Code 프리셋을 `append` 속성과 함께 사용하여 모든 기본 제공 기능을 유지하면서 사용자 정의 명령을 추가할 수 있습니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []


  async def main():
      async for message in query(
          prompt="Help me write a Python function to calculate fibonacci numbers",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": "Always include detailed docstrings and type hints in Python code.",
              }
          ),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  사용자 및 머신 전체에서 프롬프트 캐싱 개선
</h4>

기본적으로, 동일한 `claude_code` 프리셋과 `append` 텍스트를 사용하는 두 세션은 다른 작업 디렉토리에서 실행되는 경우 프롬프트 캐시 항목을 공유할 수 없습니다. 이는 프리셋이 `append` 텍스트 앞의 시스템 프롬프트에 세션별 컨텍스트를 포함하기 때문입니다: 작업 디렉토리, git 저장소 여부, 플랫폼, 활성 셸, OS 버전, 그리고 자동 메모리 경로. 해당 컨텍스트의 차이는 다른 시스템 프롬프트를 생성하고 캐시 미스를 초래합니다. CLAUDE.md 내용은 SDK가 이를 시스템 프롬프트가 아닌 대화에 주입하기 때문에 시스템 프롬프트 캐시에 영향을 주지 않습니다.

세션 전체에서 시스템 프롬프트를 동일하게 만들려면, TypeScript에서 `excludeDynamicSections: true`를 설정하거나 Python에서 `"exclude_dynamic_sections": True`를 설정하십시오. 세션별 컨텍스트는 첫 번째 사용자 메시지로 이동하여, 정적 프리셋과 `append` 텍스트만 시스템 프롬프트에 남으므로 동일한 구성이 사용자 및 머신 전체에서 캐시 항목을 공유합니다.

<Note>
  `excludeDynamicSections`는 `@anthropic-ai/claude-agent-sdk` v0.2.98 이상 또는 Python의 경우 `claude-agent-sdk` v0.1.58 이상이 필요합니다. 프리셋 객체 형식에만 설정하십시오. SDK는 사용자 정의 프롬프트 대신 프리셋을 전달할 때 이를 무시합니다. 사용자 정의 프롬프트의 명령을 TypeScript SDK에서 캐시된 상태로 유지하려면, [Cache the static part of a custom prompt](#cache-the-static-part-of-a-custom-prompt)를 참조하십시오.
</Note>

다음 예제는 공유 `append` 블록을 `excludeDynamicSections`와 쌍으로 만들어 다양한 디렉토리에서 실행되는 에이전트 플릿이 동일한 캐시된 시스템 프롬프트를 재사용할 수 있도록 합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Triage the open issues in this repo",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
                  "exclude_dynamic_sections": True,
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

**트레이드오프:** 작업 디렉토리, git 저장소 플래그, 플랫폼, 활성 셸, OS 버전, 그리고 자동 메모리 경로는 여전히 Claude에 도달하지만, 시스템 프롬프트가 아닌 첫 번째 사용자 메시지의 일부로 도달합니다. 사용자 메시지의 명령은 시스템 프롬프트의 동일한 텍스트보다 약간 덜 가중치를 가지므로, Claude는 현재 디렉토리 또는 자동 메모리 경로에 대해 추론할 때 이를 덜 강하게 의존할 수 있습니다. 교차 세션 캐시 재사용이 최대한 권위 있는 환경 컨텍스트보다 더 중요할 때 이 옵션을 활성화하십시오.

비대화형 CLI 모드의 동등한 플래그는 [`--exclude-dynamic-system-prompt-sections`](/docs/ko/cli-reference)를 참조하십시오.

<h3 id="custom-system-prompts">
  사용자 정의 시스템 프롬프트
</h3>

사용자 정의 문자열을 `systemPrompt`로 제공하여 기본값을 완전히 자신의 명령으로 바꿀 수 있습니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []


  async def main():
      async for message in query(
          prompt="Create a data processing pipeline",
          options=ClaudeAgentOptions(system_prompt=custom_prompt),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

Python에서는 문자열로 전달하는 대신 `system_prompt={"type": "file", "path": "..."}` 형식으로 파일에서 큰 사용자 정의 프롬프트를 로드합니다. Python SDK는 문자열 프롬프트를 CLI 서브프로세스에 대한 하나의 명령줄 인수로 전달하므로, OS 인수 길이 제한을 초과하는 프롬프트는 API 요청이 전송되기 전에 프로세스 생성 시 실패합니다. Linux에서 오류는 `Argument list too long`입니다. 플랫폼 임계값 및 Windows 동작은 [`SystemPromptFile`](/docs/ko/agent-sdk/python#systempromptfile)을 참조하십시오.

<h4 id="cache-the-static-part-of-a-custom-prompt">
  사용자 정의 프롬프트의 정적 부분 캐시
</h4>

TypeScript SDK에서는 사용자 정의 프롬프트를 하나의 문자열 대신 문자열 배열로 전달할 수 있으며, 정적 부분과 나머지 부분 사이에 `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` 마커를 포함합니다. 프롬프트가 모든 요청에서 동일한 명령과 요청마다 변경되는 컨텍스트(예: 에이전트가 처리하는 고객 또는 티켓)를 결합할 때 이를 사용합니다. 두 부분을 하나의 문자열로 전달하면, 요청별 부분의 변경이 전체 시스템 프롬프트를 변경하므로 정적 명령도 캐시를 놓칩니다. 배열 형식은 Python SDK에서 사용할 수 없습니다. [`ClaudeAgentOptions`](/docs/ko/agent-sdk/python#claudeagentoptions)는 `system_prompt`가 허용하는 형식을 나열합니다.

<Note>
  SDK는 Claude API를 직접 호출하거나 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)에서 실행할 때만 프롬프트를 분할합니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, 또는 [LLM gateway](/docs/ko/llm-gateway-connect)와 같은 다른 모든 구성에서, 그리고 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/ko/llm-gateway-protocol#disable-pre-release-capabilities)을 설정할 때마다, SDK는 전체 프롬프트를 하나의 블록으로 전송하며, 이는 하나의 문자열을 전달하는 것과 같습니다.
</Note>

프롬프트를 분할하려면, `@anthropic-ai/claude-agent-sdk`에서 `SYSTEM_PROMPT_DYNAMIC_BOUNDARY`를 가져오고 두 부분 사이의 배열 요소로 전달합니다. SDK는 마커 앞의 문자열을 하나의 텍스트 블록으로 전송하고 마커 뒤의 문자열을 두 번째 블록으로 전송하며, 각각 자체 캐시 중단점을 가집니다. 아래 예제에서 지원 에이전트는 분류 명령을 파일에서 로드하고 각 요청에서 하나의 티켓에 대한 세부 정보를 받으므로, 명령은 캐시된 상태로 유지되고 티켓 세부 정보는 변경됩니다:

```typescript TypeScript theme={null}
import { readFile } from "node:fs/promises";
import { query, SYSTEM_PROMPT_DYNAMIC_BOUNDARY } from "@anthropic-ai/claude-agent-sdk";

// Identical on every request
const instructions = await readFile("triage-instructions.md", "utf8");
// Different on every request
const ticketContext = "Customer plan: Enterprise. Other open tickets from this customer: 3.";

for await (const message of query({
  prompt: "Triage ticket 4821",
  options: {
    systemPrompt: [instructions, SYSTEM_PROMPT_DYNAMIC_BOUNDARY, ticketContext]
  }
})) {
  // ...
}
```

[Track cache tokens](/docs/ko/agent-sdk/cost-tracking#track-cache-tokens)는 각 결과 메시지의 `cache_creation_input_tokens` 및 `cache_read_input_tokens` 필드를 설명합니다.

SDK는 다음과 같이 배열의 블록을 조립합니다:

* SDK는 마커의 각 측면에 있는 문자열을 그 사이에 빈 줄과 함께 조인하고 마커 자체를 제거하므로, 마커 텍스트는 Claude에 도달하지 않습니다.
* 마커를 두 번 이상 포함하면, 첫 번째가 분할이고 SDK는 나머지를 제거합니다.
* 마커를 생략하면, SDK는 모든 문자열을 하나의 블록으로 조인하며, 이는 하나의 문자열을 전달하는 것과 같습니다.

CLI의 [`--system-prompt` 또는 `--system-prompt-file` 플래그](/docs/ko/cli-reference#system-prompt-flags)를 사용하면, 프롬프트는 하나의 문자열이므로 마커를 전달할 배열이 없습니다. 정적 부분과 요청별 부분 사이에 `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__`만 포함된 줄을 포함하십시오. Claude Code는 프롬프트를 첫 번째 줄에서 동일한 두 블록으로 분할하고 해당 줄을 제거합니다. Claude Code v2.1.275 이상이 필요합니다.

SDK에서는 배열 형식을 선호하십시오. 이는 마커 줄 없이 경계를 전달합니다.

<h3 id="change-the-prompt-of-an-existing-session">
  기존 세션의 프롬프트 변경
</h3>

기본적으로, `resume` 또는 `continue`로 세션으로 돌아간 후 다른 `append` 또는 사용자 정의 프롬프트를 전달하면, Claude는 다음 턴에서 이를 보지 못합니다. Claude Code는 세션의 첫 번째 요청에서 시스템 프롬프트를 기록하고 세션이 압축될 때까지 해당 기록을 재사용합니다. 새 텍스트는 그 압축 후 또는 새 세션에서 적용됩니다.

<h4 id="update-claude’s-instructions-mid-session">
  Claude의 명령을 세션 중간에 업데이트
</h4>

시스템 프롬프트에 넣은 명령이 세션이 실행되는 동안 변경되어야 하는 경우, 예를 들어 사용자가 에이전트를 읽기 전용 모드로 전환했거나 앱에서 구성을 편집한 경우, `systemPrompt`를 변경하는 대신 대화에 새 명령을 보내십시오:

* **다음 메시지에서**: 보내는 다음 사용자 메시지에 새 명령을 포함합니다.
* **hook에서**: `UserPromptSubmit` 또는 `PostToolUse` [hook callback](/docs/ko/agent-sdk/hooks#outputs)에서 [`additionalContext`](/docs/ko/hooks#add-context-for-claude)를 반환하며, "작업 공간이 이제 읽기 전용입니다"와 같은 사실 진술로 작성합니다. SDK는 hook이 발생한 지점에서 텍스트를 대화에 삽입하므로, 기록된 프롬프트는 변경되지 않습니다.

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  표현을 반복할 때 기록 끄기
</h4>

프롬프트 표현을 반복하고 각 편집이 재개하는 세션에 도달하기를 원할 때, 시스템 프롬프트의 객체 형식에서 `snapshot`을 false로 설정하십시오. Claude Code는 모든 요청에서 프롬프트를 다시 빌드합니다. 필드는 TypeScript의 [`systemPrompt`](/docs/ko/agent-sdk/typescript#options)의 프리셋 및 사용자 정의 형식과 Python의 [`system_prompt`](/docs/ko/agent-sdk/python#systempromptpreset)에서 사용 가능하며, `@anthropic-ai/claude-agent-sdk` v0.3.257 이상 또는 `claude-agent-sdk` v0.2.153 이상이 필요합니다.

프로덕션에서는 기록을 켜진 상태로 유지하십시오. 기록이 꺼져 있으면, 재개된 세션에서 다른 `append` 또는 사용자 정의 프롬프트가 다음 턴에서 Claude에 도달하며, 해당 요청은 세션의 [프롬프트 캐시](/docs/ko/prompt-caching#how-the-cache-is-organized)를 재사용할 수 없습니다. API가 [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking)을 적용하는 경우, Claude는 이전 턴의 사고도 잃습니다.

[cloud sessions](/docs/ko/cloud-environments) 외부에서, `extraArgs`를 통해 `--bare`를 전달하거나 `CLAUDE_CODE_SIMPLE=1`을 설정하여 Claude Code를 [bare mode](/docs/ko/headless#start-faster-with-bare-mode)에서 시작하면, `snapshot: true`를 설정하지 않는 한 기록이 꺼집니다.

기본적으로 `append` 또는 사용자 정의 프롬프트를 기록하려면 Claude Code v2.1.265 이상이 필요하며, TypeScript Agent SDK는 v0.3.265부터 이를 번들로 제공하고 Python Agent SDK는 v0.2.153부터 제공합니다. Claude Code v2.1.268 이전에는 [feature flags를 가져오지 않는](/docs/ko/env-vars#features-that-need-feature-flag-fetching) 세션(Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry의 세션 포함)이 모든 요청에서 프롬프트를 다시 빌드했으며 `snapshot`은 효과가 없었습니다.

<h2 id="compare-the-four-approaches">
  네 가지 방식 모두 비교
</h2>

네 가지 사용자 정의 방법은 위치, 공유 방식, `claude_code` 사전에서 보존되는 내용이 다릅니다.

| 기능            | CLAUDE.md | 출력 스타일        | `systemPrompt`과 함께 추가 | 사용자 정의 `systemPrompt` |
| ------------- | --------- | ------------- | --------------------- | --------------------- |
| **지속성**       | 프로젝트별 파일  | 파일로 저장        | 세션만                   | 세션만                   |
| **재사용성**      | 프로젝트별     | 프로젝트 전체       | 코드 중복                 | 코드 중복                 |
| **관리**        | 파일 시스템    | CLI + 파일      | 코드에서                  | 코드에서                  |
| **기본 도구**     | 유지됨       | 유지됨           | 유지됨                   | 손실됨(포함되지 않은 경우)       |
| **기본 제공 안전**  | 유지됨       | 유지됨           | 유지됨                   | 추가해야 함                |
| **환경 컨텍스트**   | 자동        | 자동            | 자동                    | 제공해야 함                |
| **사용자 정의 수준** | 추가만       | 기본값 바꾸기 또는 확장 | 추가만                   | 완전 제어                 |
| **버전 제어**     | 프로젝트와 함께  | 예             | 코드와 함께                | 코드와 함께                |
| **범위**        | 프로젝트별     | 사용자 또는 프로젝트   | 코드 세션                 | 코드 세션                 |

"추가와 함께"는 TypeScript에서 `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }`를 사용하거나 Python에서 `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}`를 사용하는 것을 의미합니다. CLAUDE.md는 시스템 프롬프트 자체를 변경하지 않습니다. SDK는 프로젝트 컨텍스트로 대화에 내용을 주입합니다.

<h2 id="combine-approaches">
  접근 방식 결합
</h2>

접근 방식들은 구성됩니다. 지속적인 출력 스타일 또는 CLAUDE.md는 장기적인 동작을 설정하고, `append`는 저장된 구성을 건드리지 않으면서 세션별 지침을 그 위에 계층화합니다.

<h3 id="combine-an-output-style-with-session-specific-additions">
  출력 스타일과 세션별 추가 사항 결합
</h3>

아래 예제는 Code Reviewer 출력 스타일이 이미 활성화되어 있다고 가정합니다. `append` 블록은 세션별 초점 영역을 페르소나 위에 계층화하므로, 단일 검토 세션이 저장된 출력 스타일을 변경하지 않으면서 OAuth 및 토큰 저장소에 우선순위를 지정할 수 있습니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Assuming "Code Reviewer" output style is active (via /config or settings)
  // Add session-specific focus areas
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []


  async def main():
      async for message in query(
          prompt="Review this authentication module",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": """
                  For this review, prioritize:
                  - OAuth 2.0 compliance
                  - Token storage security
                  - Session management
                  """,
              }
          ),
      ):
          messages.append(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="see-also">
  참고 항목
</h2>

* [출력 스타일](/docs/ko/output-styles): CLI용 출력 스타일을 생성, 관리 및 공유합니다. 파일 형식 및 저장 위치를 포함합니다.
* [Claude가 프로젝트를 기억하는 방식](/docs/ko/memory): CLAUDE.md에 무엇을 넣을지, 어디에 배치할지, 효과적인 프로젝트 지침을 작성하는 방법
* [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript): `systemPrompt`, `settingSources`, `settings`를 포함한 전체 `Options` 타입
* [Python SDK 참조](/docs/ko/agent-sdk/python): `system_prompt` 및 `setting_sources`를 포함한 전체 `ClaudeAgentOptions` 타입
* [설정](/docs/ko/settings): 출력 스타일 및 기타 구성이 저장되는 위치를 포함한 `settings.json` 참조
