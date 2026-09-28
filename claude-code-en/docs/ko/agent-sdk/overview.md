> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 개요

> Claude Code를 라이브러리로 사용하여 프로덕션 AI 에이전트 구축하기

에이전트는 자신의 단계를 계획하고 파일을 읽거나, 명령을 실행하거나, 코드를 편집하는 도구를 호출하여 작업을 완료하는 애플리케이션입니다. Agent SDK는 Claude Code를 강화하는 동일한 도구, [에이전트 루프](/docs/ko/agent-sdk/agent-loop), 및 컨텍스트 관리를 Python 및 TypeScript로 프로그래밍할 수 있도록 제공합니다.

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Agent SDK를 다른 Claude 도구와 비교
</h2>

Agent SDK, CLI, Client SDK, Managed Agents는 에이전트를 누가 실행하는지, 무엇이 기본으로 제공되는지, 어떻게 접근하는지에 따라 다릅니다. 에이전트를 구축하고 실행하려는 방식과 일치하는 행을 찾으십시오.

| 원하는 작업                                                                    | 사용                                                                                | 제공되는 것                                                                                                                                                                                                                                                                                               |
| ------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code의 에이전트를 자신이 운영하는 프로세스에서 자신의 Python 또는 TypeScript 애플리케이션에 포함시키기 | **Agent SDK**                                                                     | Claude Code 바이너리를 실행하는 라이브러리로, 기본 제공 도구, 권한, 세션, 훅 등 Claude Code의 [기능](#capabilities)을 포함합니다.                                                                                                                                                                                                        |
| 터미널에서 대화형 개발을 수행하거나 일회성 작업 실행                                             | [**Claude Code CLI**](/docs/ko/overview)                                               | 일일 대화형 사용을 위해 구축된 터미널 인터페이스입니다.                                                                                                                                                                                                                                                                      |
| 자신의 코드에서 Claude API를 직접 호출                                                | [**Client SDK**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | 모든 Client SDK 언어에서 Claude API에 직접 액세스합니다. 도구 루프를 직접 작성하거나 Client SDK의 베타 [도구 실행기](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner)가 이를 구동하도록 할 수 있습니다.                                                                                                                     |
| Anthropic이 에이전트를 호스팅하고 Claude API를 통해 구성                                  | [**Managed Agents**](https://platform.claude.com/docs/en/managed-agents/overview) | 에이전트 루프를 실행하는 호스팅된 에이전트 하네스로, Anthropic 관리 클라우드 샌드박스 또는 자신의 인프라에 있는 [자체 호스팅 샌드박스](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes)의 세션을 포함합니다. [자신의 언어용 SDK](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk), `ant` CLI 또는 REST API에서 사용합니다. |

Python 또는 TypeScript 이외의 언어에서 동일한 에이전트 루프를 구동하려면 `-p` 플래그 및 `--output-format json`과 함께 [CLI를 서브프로세스로 실행](/docs/ko/headless)하십시오.

<h2 id="capabilities">
  기능
</h2>

이러한 Claude Code 기능은 SDK에서 사용 가능합니다:

| 기능               | 기능                                                         | 자세히 알아보기                                                                                                                                                                              |
| ---------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 기본 제공 도구         | 파일 읽기, 쓰기, 편집, 명령 실행 및 웹 검색                                | [도구 참조](/docs/ko/tools-reference)                                                                                                                                                          |
| Hooks            | 에이전트 라이프사이클의 주요 지점에서 사용자 정의 코드 실행                          | [Hooks](/docs/ko/agent-sdk/hooks)                                                                                                                                                          |
| Subagents        | 집중된 부작업을 위해 특화된 에이전트 생성                                    | [Subagents](/docs/ko/agent-sdk/subagents)                                                                                                                                                  |
| MCP              | Model Context Protocol을 통해 외부 도구 및 데이터 소스 연결               | [MCP](/docs/ko/agent-sdk/mcp)                                                                                                                                                              |
| 권한               | 어떤 도구가 자동으로 실행되는지, 어떤 도구가 승인이 필요한지 제어                      | [권한](/docs/ko/agent-sdk/permissions)                                                                                                                                                       |
| 세션               | 교환 전체에서 컨텍스트 유지, 나중에 재개 또는 포크                              | [세션](/docs/ko/agent-sdk/sessions)                                                                                                                                                          |
| Skills, 명령 및 메모리 | 프로젝트의 `.claude/` 및 `~/.claude/`에서 자동으로 로드, Claude Code와 동일 | [Skills](/docs/ko/agent-sdk/skills), [명령](/docs/ko/agent-sdk/skills#commands-in-agent-sdk-sessions), [메모리](/docs/ko/agent-sdk/modifying-system-prompts), [구성 로드](/docs/ko/agent-sdk/claude-code-features) |
| Plugins          | Skills, 에이전트, Hooks 및 MCP 서버를 패키징하고 로컬 경로로 로드              | [Plugins](/docs/ko/agent-sdk/plugins)                                                                                                                                                      |

<h2 id="get-started">
  시작하기
</h2>

[빠른 시작](/docs/ko/agent-sdk/quickstart)을 따라 SDK를 설치하고, API 키를 설정하고, 기존 코드의 버그를 찾아 수정하는 첫 번째 에이전트를 구축합니다.

<Note>
  사전에 승인되지 않은 경우, Anthropic은 제3자 개발자가 claude.ai 로그인 또는 Agent SDK로 구축한 에이전트를 포함한 제품에 대한 속도 제한을 제공하는 것을 허용하지 않습니다. 대신 [빠른 시작](/docs/ko/agent-sdk/quickstart)에 설명된 API 키 인증 방법을 사용합니다.
</Note>

<h2 id="changelog">
  변경 로그
</h2>

SDK 업데이트, 버그 수정 및 새로운 기능에 대한 전체 변경 로그를 보십시오:

* **TypeScript SDK**: [CHANGELOG.md 보기](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [CHANGELOG.md 보기](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="report-bugs">
  버그 보고
</h2>

Agent SDK에서 버그 또는 문제가 발생하면:

* **TypeScript SDK**: [GitHub에서 문제 보고](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [GitHub에서 문제 보고](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  브랜딩 지침
</h2>

Claude Agent SDK를 통합하는 파트너의 경우 Claude 브랜딩 사용은 선택 사항입니다. 제품에서 Claude를 참조할 때:

**허용됨:**

* "Claude Agent" (드롭다운 메뉴에 권장)
* "Claude" (이미 "Agents"로 표시된 메뉴 내)
* "\{YourAgentName} Powered by Claude" (기존 에이전트 이름이 있는 경우)

**허용되지 않음:**

* "Claude Code" 또는 "Claude Code Agent"
* Claude Code 브랜드 ASCII 아트 또는 Claude Code를 모방하는 시각적 요소

제품은 자체 브랜딩을 유지해야 하며 Claude Code 또는 Anthropic 제품으로 보이지 않아야 합니다. 브랜딩 준수에 대한 질문은 Anthropic [영업팀](https://www.anthropic.com/contact-sales)에 문의하십시오.

<h2 id="license-and-terms">
  라이선스 및 약관
</h2>

Claude Agent SDK의 사용은 [Anthropic의 상용 서비스 약관](https://www.anthropic.com/legal/commercial-terms)에 의해 관리되며, 이는 자신의 고객 및 최종 사용자가 사용할 수 있도록 제공하는 제품 및 서비스를 강화하기 위해 사용할 때도 포함됩니다. 단, 특정 구성 요소 또는 종속성이 해당 구성 요소의 LICENSE 파일에 표시된 대로 다른 라이선스로 적용되는 경우는 제외합니다.

<h2 id="next-steps">
  다음 단계
</h2>

이러한 리소스는 Agent SDK로 구축하기 위한 더 깊이 있는 기술 세부 정보와 예제 프로젝트를 다룹니다.

* [빠른 시작](/docs/ko/agent-sdk/quickstart): 버그를 찾고 수정하는 첫 번째 에이전트 구축하기
* [마이그레이션 가이드](/docs/ko/agent-sdk/migration-guide): Claude Code SDK 패키지에서 Agent SDK로 마이그레이션하기
* [에이전트 루프](/docs/ko/agent-sdk/agent-loop): Claude가 계획하고, 도구를 호출하고, 작업이 완료되었을 때를 결정하는 방법
* [예제 에이전트](https://github.com/anthropics/claude-agent-sdk-demos): 로컬 개발을 위한 데모 앱
* [TypeScript SDK](/docs/ko/agent-sdk/typescript): 전체 TypeScript API 참조 및 예제
* [Python SDK](/docs/ko/agent-sdk/python): 전체 Python API 참조 및 예제
* [에이전트 하네스 설계](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): Claude Code 팀이 동적 워크플로우를 사용하여 많은 서브에이전트를 한 번에 오케스트레이션하는 방법
