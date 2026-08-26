> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Agent SDK로 마이그레이션

> Claude Code TypeScript 및 Python SDK를 Claude Agent SDK로 마이그레이션하기 위한 가이드

<h2 id="overview">
  개요
</h2>

Claude Code SDK의 이름이 **Claude Agent SDK**로 변경되었으며 설명서가 재구성되었습니다. 이 변경은 코딩 작업을 넘어 AI 에이전트를 구축하기 위한 SDK의 광범위한 기능을 반영합니다.

<h2 id="what’s-changed">
  변경 사항
</h2>

| 항목                 | 이전                          | 새로운                              |
| :----------------- | :-------------------------- | :------------------------------- |
| **패키지 이름 (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Python 패키지**     | `claude-code-sdk`           | `claude-agent-sdk`               |
| **설명서 위치**         | Claude Code 문서              | API 가이드 → Agent SDK 섹션           |

<Note>
  **설명서 변경 사항:** Agent SDK 설명서가 Claude Code 문서에서 API 가이드의 전용 [Agent SDK](/docs/ko/agent-sdk/overview) 섹션으로 이동되었습니다. Claude Code 문서는 이제 CLI 도구 및 자동화 기능에 중점을 두고 있습니다.
</Note>

<h2 id="migration-steps">
  마이그레이션 단계
</h2>

<h3 id="for-typescript/javascript-projects">
  TypeScript/JavaScript 프로젝트의 경우
</h3>

**1. 이전 패키지 제거:**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. 새 패키지 설치:**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. 임포트 업데이트:**

`@anthropic-ai/claude-code`에서 `@anthropic-ai/claude-agent-sdk`로 모든 임포트를 변경합니다:

```typescript theme={null}
// 이전
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// 이후
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. package.json 의존성 업데이트:**

`package.json`에 패키지가 나열되어 있으면 업데이트합니다:

이전:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

이후:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. [주요 변경 사항](#breaking-changes) 검토**

마이그레이션을 완료하는 데 필요한 코드 변경을 수행합니다.

<h3 id="for-python-projects">
  Python 프로젝트의 경우
</h3>

**1. 이전 패키지 제거:**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. 새 패키지 설치:**

```bash theme={null}
pip install claude-agent-sdk
```

**3. 임포트 업데이트:**

`claude_code_sdk`에서 `claude_agent_sdk`로 모든 임포트를 변경합니다:

```python theme={null}
# 이전
from claude_code_sdk import query, ClaudeCodeOptions

# 이후
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. 타입 이름 업데이트:**

`ClaudeCodeOptions`를 `ClaudeAgentOptions`로 변경합니다:

```python theme={null}
# 이전
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# 이후
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. [주요 변경 사항](#breaking-changes) 검토**

마이그레이션을 완료하는 데 필요한 코드 변경을 수행합니다.

<h2 id="breaking-changes">
  주요 변경 사항
</h2>

<Warning>
  격리 및 명시적 구성을 개선하기 위해 Claude Agent SDK v0.1.0은 Claude Code SDK에서 마이그레이션하는 사용자를 위한 주요 변경 사항을 도입합니다. 마이그레이션하기 전에 이 섹션을 주의 깊게 검토하십시오.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions를 ClaudeAgentOptions로 이름 변경
</h3>

**변경 사항:** Python SDK 타입 `ClaudeCodeOptions`의 이름이 `ClaudeAgentOptions`로 변경되었습니다.

**마이그레이션:**

```python theme={null}
# 이전 (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# 이후 (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**변경 이유:** 타입 이름이 이제 "Claude Agent SDK" 브랜딩과 일치하며 SDK의 명명 규칙 전체에서 일관성을 제공합니다.

<h3 id="system-prompt-no-longer-default">
  시스템 프롬프트가 더 이상 기본값이 아님
</h3>

**변경 사항:** SDK는 더 이상 기본적으로 Claude Code의 시스템 프롬프트를 사용하지 않습니다.

**마이그레이션:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 이전 (v0.0.x) - 기본적으로 Claude Code의 시스템 프롬프트 사용
  const before = query({ prompt: "Hello" });

  // 이후 (v0.1.0) - 기본적으로 최소 시스템 프롬프트 사용
  // 이전 동작을 얻으려면 Claude Code의 프리셋을 명시적으로 요청합니다:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // 또는 사용자 정의 시스템 프롬프트를 사용합니다:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # 이전 (v0.0.x) - 기본적으로 Claude Code의 시스템 프롬프트 사용
  async for message in query(prompt="Hello"):
      print(message)

  # 이후 (v0.1.0) - 기본적으로 최소 시스템 프롬프트 사용
  # 이전 동작을 얻으려면 Claude Code의 프리셋을 명시적으로 요청합니다:
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # 프리셋 사용
      ),
  ):
      print(message)

  # 또는 사용자 정의 시스템 프롬프트를 사용합니다:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**변경 이유:** SDK 애플리케이션에 더 나은 제어 및 격리를 제공합니다. 이제 Claude Code의 CLI 중심 지침을 상속하지 않고 사용자 정의 동작으로 에이전트를 구축할 수 있습니다.

<h3 id="settings-sources-default">
  설정 소스 기본값
</h3>

이 기본값은 v0.1.0에서 잠시 변경되었다가 되돌려졌으므로 마이그레이션 조치가 필요하지 않습니다.

**현재 동작:** `query()`에서 `settingSources`를 생략하면 CLI와 일치하는 사용자, 프로젝트 및 로컬 파일 시스템 설정이 로드됩니다. 여기에는 `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, CLAUDE.md 파일 및 사용자 정의 명령이 포함됩니다.

파일 시스템 설정에서 격리되어 실행하려면 빈 배열을 전달합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // 파일 시스템 설정이 로드되지 않음
    }
  });

  // 또는 특정 소스만 로드합니다:
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // 프로젝트 설정만
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # 파일 시스템 설정이 로드되지 않음
  ):
      print(message)

  # 또는 특정 소스만 로드합니다:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # 프로젝트 설정만
      ),
  ):
      print(message)
  ```
</CodeGroup>

격리는 특히 CI/CD 파이프라인, 배포된 애플리케이션, 테스트 환경 및 로컬 사용자 정의가 유입되지 않아야 하는 다중 테넌트 시스템에 중요합니다.

<Note>
  SDK v0.1.0은 잠시 설정이 로드되지 않는 것으로 기본값을 설정했으나 이후 릴리스에서 되돌려졌습니다. Python SDK 0.1.59 이하는 빈 목록을 옵션 생략과 동일하게 처리했으므로 `setting_sources=[]`에 의존하기 전에 업그레이드하십시오. `settingSources`가 `[]`일 때 읽히는 입력에 대해서는 [settingSources가 제어하지 않는 것](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)을 참조하십시오.
</Note>

<h2 id="why-the-rename">
  이름 변경 이유
</h2>

Claude Code SDK는 원래 코딩 작업을 위해 설계되었지만 모든 유형의 AI 에이전트를 구축하기 위한 강력한 프레임워크로 진화했습니다. 새로운 이름 "Claude Agent SDK"는 그 기능을 더 잘 반영합니다:

* 비즈니스 에이전트 구축 (법률 보조원, 재무 고문, 고객 지원)
* 전문화된 코딩 에이전트 생성 (SRE 봇, 보안 검토자, 코드 검토 에이전트)
* 도구 사용, MCP 통합 등으로 모든 도메인에 대한 사용자 정의 에이전트 개발

<h2 id="getting-help">
  도움말 받기
</h2>

마이그레이션 중에 문제가 발생하면:

**TypeScript/JavaScript의 경우:**

1. 모든 임포트가 `@anthropic-ai/claude-agent-sdk`를 사용하도록 업데이트되었는지 확인합니다
2. package.json에 새 패키지 이름이 있는지 확인합니다
3. `npm install`을 실행하여 의존성이 업데이트되었는지 확인합니다

**Python의 경우:**

1. 모든 임포트가 `claude_agent_sdk`를 사용하도록 업데이트되었는지 확인합니다
2. requirements.txt 또는 pyproject.toml에 새 패키지 이름이 있는지 확인합니다
3. `pip install claude-agent-sdk`를 실행하여 패키지가 설치되었는지 확인합니다

<h2 id="next-steps">
  다음 단계
</h2>

* [Agent SDK 개요](/docs/ko/agent-sdk/overview)를 탐색하여 사용 가능한 기능에 대해 알아봅니다
* [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript)를 확인하여 자세한 API 설명서를 봅니다
* [Python SDK 참조](/docs/ko/agent-sdk/python)를 검토하여 Python 관련 설명서를 봅니다
* [사용자 정의 도구](/docs/ko/agent-sdk/custom-tools) 및 [MCP 통합](/docs/ko/agent-sdk/mcp)에 대해 알아봅니다
