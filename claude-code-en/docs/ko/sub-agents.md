> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 사용자 정의 subagent 만들기

> Claude Code에서 작업별 워크플로우 및 향상된 컨텍스트 관리를 위한 특화된 AI subagent를 만들고 사용합니다.

Subagent는 특정 유형의 작업을 처리하는 특화된 AI 어시스턴트입니다. 부작업이 검색 결과, 로그 또는 다시 참조하지 않을 파일 콘텐츠로 주 대화를 넘칠 때 하나를 사용하세요: subagent는 자신의 컨텍스트에서 해당 작업을 수행하고 요약만 반환합니다. 동일한 지침으로 동일한 종류의 워커를 계속 생성할 때 사용자 정의 subagent를 정의합니다.

각 subagent는 자체 컨텍스트 윈도우에서 실행되며 사용자 정의 시스템 프롬프트, 특정 도구 액세스 및 독립적인 권한을 가집니다. Claude가 subagent의 설명과 일치하는 작업을 만나면 해당 subagent에 위임하고, subagent는 독립적으로 작동하여 결과를 반환합니다. 실제로 컨텍스트 절감을 확인하려면 [컨텍스트 윈도우 시각화](/docs/ko/context-window)에서 subagent가 자신의 별도 윈도우에서 연구를 처리하는 세션을 안내합니다.

<Note>
  Subagent는 단일 세션 내에서 작동합니다. 많은 독립적인 세션을 병렬로 실행하고 한 곳에서 모니터링하려면 [background agents](/docs/ko/agent-view)를 참조하세요. 서로 메시지를 전달하는 별도의 세션의 경우 [cross-session messaging](/docs/ko/cross-session-messaging)을 참조하세요. Claude가 생성하고 감독하는 조정된 세션 팀의 경우 [agent teams](/docs/ko/agent-teams)를 참조하세요.
</Note>

Subagent는 다음을 도와줍니다:

* **컨텍스트 보존** - 탐색 및 구현을 주 대화에서 분리하여 유지
* **제약 조건 적용** - subagent가 사용할 수 있는 도구 제한
* **구성 재사용** - 사용자 수준 subagent를 통해 프로젝트 간 구성 재사용
* **동작 특화** - 특정 도메인을 위한 집중된 시스템 프롬프트
* **비용 제어** - Haiku와 같은 더 빠르고 저렴한 모델로 작업 라우팅

Claude는 각 subagent의 설명을 사용하여 작업을 위임할 시기를 결정합니다. Subagent를 만들 때 Claude가 언제 사용할지 알 수 있도록 명확한 설명을 작성하세요.

이러한 설명들은 컨텍스트를 차지하므로 짧게 유지하세요. 내장된 subagent를 제외한 subagent의 결합된 설명이 15,000 토큰을 초과하면 Claude Code는 [총 토큰 수와 함께 시작 시 경고를 표시합니다](/docs/ko/errors#agent-descriptions-are-over-the-15000-token-limit). Subagent의 `description` 필드를 정리하고 세부 정보를 각 subagent의 시스템 프롬프트로 이동하세요. 시스템 프롬프트는 해당 subagent가 실행될 때만 로드됩니다.

<h2 id="built-in-subagents">
  내장 subagent
</h2>

Claude Code에는 Claude가 적절할 때 자동으로 사용하는 내장 subagent가 포함되어 있습니다. 각각은 부모 대화의 권한을 상속합니다. 대부분은 제한된 도구 세트로 실행됩니다.

Explore와 Plan은 연구를 빠르고 저렴하게 유지하기 위해 CLAUDE.md 파일과 git 상태 스냅샷을 건너뜁니다. 다른 모든 내장 및 [사용자 정의 subagent](#configure-subagents)는 둘 다 로드합니다. 정의가 [`omitClaudeMd`](#supported-frontmatter-fields) 필드를 설정하여 사용자, 프로젝트 및 로컬 CLAUDE.md 파일을 건너뛰지 않는 한, subagent에 도달하는 항목의 전체 분석은 [startup에서 로드되는 항목](#what-loads-at-startup)을 참조하십시오.

<Tabs>
  <Tab title="Explore">
    코드베이스 검색 및 분석에 최적화된 빠른 읽기 전용 에이전트입니다.

    * **모델**: 주 대화에서 상속되며, Claude API에서 Opus로 제한되므로 Explore는 세션에 대해 이미 선택한 모델보다 더 비싼 모델에서 실행되지 않습니다. `CLAUDE_CODE_SUBAGENT_MODEL`을 설정하고 [모든 subagent를 하나의 모델에서 실행](#run-every-subagent-on-one-model)하도록 강제하지 않는 한 그렇습니다.
    * **도구**: 읽기 전용 도구; Write 및 Edit은 거부됩니다.
    * **목적**: 파일 검색, 코드 검색, 코드베이스 탐색

    v2.1.198부터 Explore는 항상 Haiku에서 실행되는 대신 주 대화의 모델을 상속합니다. Claude API에서 상속된 모델은 Opus로 제한됩니다: 더 높은 계층의 주 대화는 Explore를 Opus에서 실행하고, Sonnet 또는 Haiku의 주 대화는 Explore를 동일한 모델에서 실행합니다. [Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 AWS의 Claude Platform](/docs/ko/third-party-integrations)과 같은 다른 공급자에서는 Explore가 주 대화의 모델을 직접 상속합니다.

    `Explore`라는 [사용자 또는 프로젝트 subagent](#choose-the-subagent-scope)는 내장 subagent를 재정의하고 자신의 `model` 필드를 유지하므로, 탐색을 더 낮은 비용의 모델에서 유지하려면 `model: haiku`를 사용하여 정의하십시오.

    Claude는 변경 없이 코드베이스를 검색하거나 이해해야 할 때 Explore에 위임합니다. 이렇게 하면 탐색 결과가 주 대화 컨텍스트에서 벗어납니다.

    Explore를 호출할 때 Claude는 철저함 수준을 지정합니다: 대상 조회의 경우 **quick**, 균형 잡힌 탐색의 경우 **medium**, 포괄적인 분석의 경우 **very thorough**.
  </Tab>

  <Tab title="Plan">
    [plan mode](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode) 중에 계획을 제시하기 전에 컨텍스트를 수집하는 데 사용되는 연구 에이전트입니다.

    * **모델**: 주 대화에서 상속되며, `CLAUDE_CODE_SUBAGENT_MODEL`을 설정하고 [모든 subagent를 하나의 모델에서 실행](#run-every-subagent-on-one-model)하도록 강제하지 않는 한 그렇습니다.
    * **도구**: 읽기 전용 도구; Write 및 Edit은 거부됩니다.
    * **목적**: 계획을 위한 코드베이스 연구

    plan mode에 있고 Claude가 코드베이스를 이해해야 할 때 연구를 Plan subagent에 위임하므로 탐색 출력이 별도의 컨텍스트 윈도우에 유지되고 주 대화는 읽기 전용으로 유지됩니다.
  </Tab>

  <Tab title="General-purpose">
    탐색과 작업 모두를 필요로 하는 복잡한 다단계 작업을 위한 유능한 에이전트입니다.

    * **모델**: [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model) 모델(설정한 경우 및 다른 것이 모델을 다른 방식으로 할당하지 않는 경우), 그렇지 않으면 주 대화의 모델; [모델 선택](#choose-a-model)은 전체 순서를 나타내고, [모든 subagent를 하나의 모델에서 실행](#run-every-subagent-on-one-model)은 변수가 이러한 소스를 재정의하는 방법을 보여줍니다.
    * **도구**: subagent에 [사용 가능한](#available-tools) 모든 도구
    * **목적**: 복잡한 연구, 다단계 작업, 코드 수정

    Claude는 작업이 탐색과 수정 모두를 필요로 하거나, 결과를 해석하기 위한 복잡한 추론이 필요하거나, 여러 종속 단계가 필요할 때 general-purpose에 위임합니다.
  </Tab>

  <Tab title="Other">
    Claude Code에는 특정 작업을 위한 추가 도우미 에이전트가 포함되어 있습니다. 이들은 일반적으로 자동으로 호출되므로 직접 사용할 필요가 없습니다.

    | 에이전트              | 모델                                                               | Claude가 사용하는 경우                                                                                                                                                                                                                        |
    | :---------------- | :--------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | claude            | 자신의 모델이 없음; Claude가 subagent로 생성할 때 [모델 순서](#choose-a-model)를 따름 | 작업이 더 특화된 에이전트에 맞지 않을 때. subagent에 [사용 가능한](#available-tools) 모든 도구가 있는 catch-all입니다. 또한 dispatched [background session](/docs/ko/agent-view)의 기본 에이전트; [시작하는 권한 모드](/docs/ko/agent-view#permission-mode-model-and-effort)는 세션이 시작된 방식에 따라 다릅니다. |
    | statusline-setup  | Sonnet                                                           | `/statusline`을 실행하여 상태 표시줄을 구성할 때                                                                                                                                                                                                      |
    | claude-code-guide | Haiku                                                            | Claude Code 기능에 대한 질문을 할 때                                                                                                                                                                                                             |
  </Tab>
</Tabs>

내장 subagent는 기본적으로 대화형 세션에 등록됩니다. 이를 제한하려면:

* 특정 내장 유형을 차단하려면 [특정 subagent 비활성화](#disable-specific-subagents)에 표시된 대로 `permissions.deny`에 추가하십시오.
* Claude가 어떤 subagent에도 위임하는 것을 방지하려면 [`permissions.deny`](/docs/ko/permissions#tool-specific-permission-rules)를 사용하여 `Agent` 도구 자체를 거부하십시오.
* 내장 `Explore` 및 `Plan` subagent만 제거하려면 [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/ko/env-vars)을 설정하십시오. Claude는 이들에게 위임하는 대신 파일을 직접 읽고 탐색합니다. Claude Code v2.1.198 이상이 필요합니다.
* [비대화형 모드](/docs/ko/headless) 및 [Agent SDK](/docs/ko/agent-sdk/overview)에서는 [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ko/env-vars)을 설정하여 모든 내장 유형을 제거하고 자신의 것만 제공하십시오.

subagent\_type을 생략하는 Agent 도구 호출은 세션에 fallback할 `general-purpose` subagent가 없을 때 [`subagent_type is required`](/docs/ko/errors#subagent-type-is-required)로 실패합니다.

이러한 내장 subagent 외에도 사용자 정의 프롬프트, 도구 제한, 권한 모드, hooks 및 skills를 사용하여 자신의 subagent를 만들 수 있습니다. 다음 섹션에서는 시작하는 방법과 subagent를 사용자 정의하는 방법을 보여줍니다.

<h2 id="quickstart-create-your-first-subagent">
  빠른 시작: 첫 번째 subagent 만들기
</h2>

Subagent는 YAML frontmatter가 있는 Markdown 파일입니다. Claude에게 작성을 요청하거나 [수동으로 파일을 작성](#write-subagent-files)할 수 있습니다.

v2.1.198부터 `/agents` 명령은 더 이상 대화형 생성 마법사를 열지 않습니다. 이를 실행하면 Claude에게 요청하거나 `.claude/agents/`를 직접 편집하라는 알림이 출력됩니다. Subagent 파일, frontmatter 필드 및 `.claude/agents/`와 `~/.claude/agents/` 위치는 변경되지 않았습니다. 터미널 마법사만 제거되었습니다.

이 연습에서는 코드를 검토하고 개선 사항을 제안하는 사용자 수준 subagent를 만듭니다.

<Steps>
  <Step title="Claude에게 subagent 생성 요청">
    Claude Code에서 원하는 subagent와 저장 위치를 설명합니다:

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude는 `name`, `description`, `tools` 목록, `model` 및 시스템 프롬프트가 포함된 파일을 작성합니다.
  </Step>

  <Step title="파일 검토">
    `~/.claude/agents/code-improver.md`를 열고 frontmatter가 요청한 내용과 일치하는지 확인합니다. 결과는 다음과 같습니다:

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    파일이 `~/.claude/agents/`에 있으므로 subagent는 머신의 모든 프로젝트에서 사용할 수 있습니다. 대신 하나의 프로젝트로 범위를 지정하려면 해당 프로젝트의 `.claude/agents/` 디렉토리로 이동합니다. [subagent 범위 선택](#choose-the-subagent-scope)에서 두 가지를 비교합니다.
  </Step>

  <Step title="시도해 보기">
    Claude에게 새 subagent에 위임하도록 요청합니다:

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude가 새 subagent에 위임하고, subagent가 코드베이스를 스캔하여 개선 제안을 반환합니다. 트랜스크립트에서 위임은 subagent의 이름 뒤에 짧은 작업 설명(예: `code-improver(Suggest code improvements)`)이 표시된 도구 호출 행으로 나타납니다.

    Claude가 새 subagent를 찾을 수 없으면 Claude Code를 다시 시작하고 다시 시도합니다. 이는 세션이 시작되기 전에 `~/.claude/agents/`가 없었을 때만 발생합니다. 실행 중인 세션은 새로 생성된 `agents` 디렉토리를 감지하지 않기 때문입니다.
  </Step>
</Steps>

이제 머신의 모든 프로젝트에서 코드베이스를 분석하고 개선 사항을 제안하는 데 사용할 수 있는 subagent가 있습니다.

subagent 파일을 수동으로 작성하거나, CLI 플래그를 통해 정의하거나, 플러그인을 통해 배포할 수도 있습니다. 다음 섹션에서는 모든 구성 옵션을 다룹니다.

<Note>
  Claude Code v2.1.197 이전 버전에서는 `/agents`가 라이브 subagent를 나열하는 **Running** 탭과 생성, 편집 및 삭제를 위한 **Library** 탭이 있는 대화형 마법사를 엽니다.&#x20;
</Note>

<h2 id="configure-subagents">
  서브에이전트 구성
</h2>

서브에이전트의 파일 위치는 누가 이를 사용할 수 있는지를 결정하며, 프론트매터는 이것이 무엇을 할 수 있는지를 결정합니다. 이 섹션에서는 서브에이전트 파일이 어디에 있는지, 그리고 지원하는 모든 필드를 다룹니다.

<h3 id="choose-the-subagent-scope">
  서브에이전트 범위 선택
</h3>

범위에 따라 서브에이전트 파일을 다른 위치에 저장합니다. 여러 서브에이전트가 같은 이름을 공유할 때, Claude Code는 우선순위가 높은 위치의 서브에이전트를 사용합니다.

| 위치                   | 범위            | 우선순위   | 생성 방법                               |
| :------------------- | :------------ | :----- | :---------------------------------- |
| 관리 설정                | 조직 전체         | 1 (최고) | [관리 설정](/docs/ko/settings)을 통해 배포        |
| `--agents` CLI 플래그   | 현재 세션         | 2      | Claude Code 시작 시 JSON 전달            |
| `.claude/agents/`    | 현재 프로젝트       | 3      | Claude에 요청하거나 파일을 수동으로 생성           |
| `~/.claude/agents/`  | 모든 프로젝트       | 4      | Claude에 요청하거나 파일을 수동으로 생성           |
| 플러그인의 `agents/` 디렉토리 | 플러그인이 활성화된 위치 | 5 (최저) | [플러그인](/docs/ko/plugins/overview)과 함께 설치 |

**프로젝트 서브에이전트** (`.claude/agents/`)는 코드베이스에 특정한 서브에이전트에 이상적입니다. 버전 관리에 체크인하여 팀이 협력적으로 사용하고 개선할 수 있습니다.

프로젝트 서브에이전트는 현재 작업 디렉토리에서 위로 걸어가며 발견되므로, 거기서 저장소 루트까지의 모든 `.claude/agents/`가 스캔됩니다. 이러한 중첩된 디렉토리 중 하나 이상이 같은 `name`을 정의할 때, Claude Code는 작업 디렉토리에 가장 가까운 정의를 사용합니다.

`--add-dir` 또는 `/add-dir`로 디렉토리를 추가할 때, Claude Code는 프로젝트 서브에이전트와 함께 해당 `.claude/agents/` 폴더도 로드합니다. 어떤 다른 구성 유형이 `--add-dir`에서 로드되는지는 [추가 디렉토리](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration)를 참조하세요. `--add-dir` 없이 프로젝트 간에 서브에이전트를 공유하려면 `~/.claude/agents/`를 사용하거나 [플러그인](/docs/ko/plugins/overview)을 사용하세요.

**사용자 서브에이전트** (`~/.claude/agents/`)는 모든 프로젝트에서 사용 가능한 개인 서브에이전트입니다.

Claude Code는 `.claude/agents/`와 `~/.claude/agents/`를 재귀적으로 스캔하므로, `agents/review/` 또는 `agents/research/`와 같은 하위 폴더로 정의를 구성할 수 있습니다. 서브디렉토리 경로는 서브에이전트가 식별되거나 호출되는 방식에 영향을 주지 않습니다. 왜냐하면 신원은 `name` 프론트매터 필드에서만 나오기 때문입니다.

전체 트리에서 `name` 값을 고유하게 유지하세요. 같은 `.claude/agents/` 디렉토리 내의 두 파일(하위 폴더 포함)이 같은 이름을 선언하면, Claude Code는 문서화된 우선순위가 아닌 파일시스템 읽기 순서로 선택된 하나만 로드합니다. 중첩된 프로젝트 디렉토리 전체에서, 작업 디렉토리에 가장 가까운 정의가 우선합니다(위에서 설명한 대로). [`/doctor`](/docs/ko/commands#all-commands) 설정 점검은 같은 디렉토리에서 이름을 공유하는 파일을 보고하고 하나를 제외한 모두의 이름을 바꾸거나 제거할 것을 제안합니다. v2.1.205 이전에는 `/doctor`가 진단 화면을 열어 중복을 나열하고 어떤 정의가 활성화되었는지 보여주었습니다.

플러그인 `agents/` 디렉토리도 재귀적으로 스캔됩니다. 프로젝트 및 사용자 범위와 달리, 플러그인의 `agents/` 디렉토리 내의 하위 폴더는 [범위가 지정된 식별자](#invoke-subagents-explicitly)의 일부가 됩니다. `my-plugin`의 `agents/review/security.md`에 있는 파일은 `my-plugin:review:security`로 등록됩니다.

**CLI 정의 서브에이전트**는 Claude Code를 시작할 때 JSON으로 전달됩니다. 이들은 해당 세션에만 존재하며 디스크에 저장되지 않으므로, 빠른 테스트나 자동화 스크립트에 유용합니다. 단일 `--agents` 호출에서 여러 서브에이전트를 정의할 수 있습니다:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

`--agents` 플래그는 `prompt` 필드와 이러한 [프론트매터](#supported-frontmatter-fields) 필드를 포함하는 JSON을 허용합니다: `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `omitClaudeMd`, 및 `isolation`. 파일 기반 서브에이전트의 마크다운 본문과 동등한 시스템 프롬프트에 `prompt`를 사용하세요. `color`과 `experimental`은 여기서 허용되지 않으며 거부되지 않고 무시됩니다.

JSON의 각 최상위 키는 에이전트의 이름입니다. 이름을 `-`로 시작하지 마세요.

Claude Code가 로드할 수 없는 값으로 수행하는 작업, 그리고 해당 확인을 건너뛰는 플래그 및 환경 변수는 [`Invalid --agents configuration`](/docs/ko/errors#invalid-agents-configuration)을 참조하세요.

**관리 서브에이전트**는 조직 관리자가 배포합니다. [관리 설정 디렉토리](/docs/ko/managed-settings#delivery-mechanisms) 내의 `.claude/agents/`에 마크다운 파일을 배치하고, 프로젝트 및 사용자 서브에이전트와 동일한 프론트매터 형식을 사용합니다. 관리 정의는 같은 이름의 프로젝트 및 사용자 서브에이전트보다 우선합니다.

**플러그인 서브에이전트**는 설치한 [플러그인](/docs/ko/plugins/overview)에서 나옵니다. 사용자 정의 서브에이전트와 함께 자동으로 로드되며 범위가 지정된 이름 아래의 @-멘션 자동완성에 나타납니다. 플러그인 서브에이전트 생성에 대한 자세한 내용은 [플러그인 구성 요소 참조](/docs/ko/plugins/components#agents)를 참조하세요.

<Note>
  보안상의 이유로, 플러그인 서브에이전트는 `hooks`, `mcpServers`, 또는 `permissionMode` 프론트매터 필드를 지원하지 않습니다. 이러한 필드는 플러그인에서 에이전트를 로드할 때 무시됩니다. 필요한 경우 에이전트 파일을 `.claude/agents/` 또는 `~/.claude/agents/`로 복사하세요. `settings.json` 또는 `settings.local.json`의 [`permissions.allow`](/docs/ko/settings-reference#permissions-allow)에 규칙을 추가할 수도 있지만, 이러한 규칙은 전체 세션에 적용되며 플러그인 서브에이전트에만 적용되지 않습니다.
</Note>

이러한 범위의 서브에이전트 정의는 [에이전트 팀](/docs/ko/agent-teams#use-subagent-definitions-for-teammates)에서도 사용 가능합니다. 팀원을 생성할 때 서브에이전트 유형을 참조할 수 있으며, Claude Code는 해당 정의의 일부를 팀원에게 적용합니다. 각 표시 모드에서 어떤 부분이 적용되는지는 [에이전트 팀](/docs/ko/agent-teams#use-subagent-definitions-for-teammates)을 참조하세요.

<h3 id="write-subagent-files">
  서브에이전트 파일 작성
</h3>

서브에이전트 파일은 구성을 위한 YAML 프론트매터를 사용하고, 그 뒤에 마크다운의 시스템 프롬프트가 옵니다:

<Note>
  Claude Code는 `~/.claude/agents/`와 `.claude/agents/`를 감시합니다. 디스크에서 서브에이전트 파일을 추가하거나 편집하거나, Claude에 작성을 요청하면, Claude Code는 몇 초 내에 변경을 감지하고 다음 위임은 재시작 없이 업데이트된 정의를 사용합니다.

  세 가지 경우는 여전히 재시작이 필요합니다:

  * 감시자는 세션이 시작될 때 존재했던 디렉토리만 다루므로, 새 `agents` 디렉토리에서 범위의 첫 번째 에이전트 파일을 생성한 후 재시작하여 로드하세요.
  * Claude Code는 `--add-dir` 또는 `/add-dir`으로 추가된 디렉토리 내의 `.claude/agents/`를 감시하지 않으므로, 거기서 서브에이전트를 추가하거나 편집한 후 재시작하여 변경을 로드하세요.
  * `--disable-slash-commands`로 시작된 세션은 이러한 디렉토리를 전혀 감시하지 않습니다.
</Note>

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

프론트매터는 서브에이전트의 메타데이터와 구성을 정의합니다. 본문은 서브에이전트의 동작을 안내하는 시스템 프롬프트가 됩니다. 서브에이전트는 이 시스템 프롬프트와 작업 디렉토리와 같은 기본 환경 세부 정보만 받으며, Claude Code 시스템 프롬프트는 받지 않습니다.

[비대화형 모드](/docs/ko/headless)에서, [`--append-subagent-system-prompt`](/docs/ko/cli-reference#cli-flags)를 전달하여 모든 서브에이전트의 시스템 프롬프트 끝에 텍스트를 추가합니다(중첩된 서브에이전트 포함, [포크된 서브에이전트](#fork-the-current-conversation) 제외. 포크된 서브에이전트는 대화의 자체 프롬프트를 재사용합니다). Claude Code v2.1.205 이상이 필요합니다. 텍스트가 너무 길어서 명령줄에 전달할 수 없으면, 파일에 저장하고 `--append-subagent-system-prompt-file`로 경로를 전달하세요. 파일 플래그는 Claude Code v2.1.261 이상이 필요합니다.

서브에이전트는 주 대화의 현재 작업 디렉토리에서 시작합니다. 서브에이전트 내에서 `cd` 명령은 Bash 또는 PowerShell 도구 호출 간에 지속되지 않으며 주 대화의 작업 디렉토리에 영향을 주지 않습니다. 서브에이전트에 저장소의 격리된 복사본을 제공하려면 [`isolation: worktree`](#supported-frontmatter-fields)를 설정하세요.

`isolation: worktree`를 가진 서브에이전트는 해당 worktree 내에서 Bash 및 PowerShell 명령을 실행합니다. 작업 디렉토리가 주 체크아웃으로 확인되는 명령(예: worktree 디렉토리가 서브에이전트 실행 중에 제거된 경우)은 오류로 실패합니다. v2.1.203 이전에는 이러한 명령이 주 체크아웃에서 실행될 수 있었습니다.

이 작업 디렉토리 확인은 Claude Code를 시작한 디렉토리를 포함하는 전체 저장소를 다룹니다. 세션이 자체 링크된 [worktree](/docs/ko/worktrees)에서 실행될 때, 확인은 해당 worktree가 링크된 주 체크아웃도 다룹니다. v2.1.210 이전에는 확인이 시작 디렉토리 자체만 다루었습니다. 작업 디렉토리가 같은 저장소의 다른 곳(예: 모노레포 하위 디렉토리에서 Claude Code를 시작했을 때 저장소 루트)으로 확인되는 명령은 실패하지 않고 거기서 실행되었습니다.

Bash 명령의 경우, Claude Code는 두 가지 방식으로 명령 자체도 확인합니다:

* git을 주 체크아웃으로 리디렉션하는 명령을 차단합니다.
* 명령 텍스트에서 명령이 실행하는 모든 git이 worktree 내에 머물러 있음을 확인할 수 없을 때 명령을 거부합니다(예: 명령 이름이 런타임에 계산될 때).

리디렉션 벡터와 형태 규칙은 [Claude Code가 격리를 적용하는 방법](/docs/ko/worktrees#how-claude-code-enforces-isolation) 아래에 나열되어 있습니다. PowerShell 명령은 작업 디렉토리 확인만 받습니다.

[모니터](/docs/ko/tools-reference#monitor-tool) 명령은 Bash 명령과 동일한 작업 디렉토리 및 명령 내용 확인을 거칩니다.

주 대화 자체가 worktree에서 격리되어 실행될 때, Claude Code는 동일한 확인을 세션과 생성하는 모든 서브에이전트에 적용합니다(`isolation: worktree` 없는 서브에이전트 포함). [Claude Code가 격리를 적용하는 방법](/docs/ko/worktrees#how-claude-code-enforces-isolation)을 참조하세요.

<h3 id="supported-frontmatter-fields">
  프론트매터 참조
</h3>

서브에이전트를 YAML [프론트매터](/docs/ko/glossary#frontmatter)로 구성합니다. 파일 상단의 `---` 마커 사이에 프론트매터를 배치하고, 닫는 `---` 뒤에 마크다운으로 시스템 프롬프트를 작성하세요. `name`과 `description`만 필수입니다.

다중 단어 필드 이름은 `maxTurns`와 `disallowedTools`와 같이 camelCase를 사용하며, 표와 정확히 일치해야 합니다. Claude Code는 인식하지 못하는 필드를 오류 보고 없이 무시합니다. 서브에이전트 파일이 로드되지 않은 이유를 알아보려면 [Claude Code가 건너뛰는 서브에이전트 파일](#subagent-files-claude-code-skips)을 참조하세요.

| 필드                | 필수  | 설명                                                                                                                                                                                                                                                                                                                                |
| :---------------- | :-- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | 예   | `code-reviewer` 또는 `reviewer-v2`와 같은 고유 식별자. [Hooks](/docs/ko/hooks#subagentstart)는 이 값을 `agent_type`으로 받습니다. 파일 이름이 일치할 필요는 없습니다. 이름은 `:`를 포함할 수 없습니다. `:`는 `my-plugin:reviewer`와 같은 [플러그인 범위 식별자](/docs/ko/plugins/overview)에 예약되어 있습니다. Claude Code는 이름을 포함하는 파일을 로드하지 않고 디버그 로그에 오류를 기록합니다. v2.1.218 이전에는 이러한 이름이 허용되었습니다               |
| `description`     | 예   | Claude가 이 서브에이전트에 위임해야 할 때                                                                                                                                                                                                                                                                                                        |
| `tools`           | 아니오 | 서브에이전트가 사용할 수 있는 [도구](#available-tools). `Read, Grep, Bash`와 같은 쉼표로 구분된 문자열 또는 YAML 목록입니다. 생략하면 서브에이전트가 사용 가능한 모든 도구를 상속합니다. 목록의 항목이 도구로 확인되지 않으면, 서브에이전트는 일반적으로 항목을 이름 지정하는 오류로 [시작에 실패](/docs/ko/errors#agent-would-be-spawned-with-zero-tools)합니다. 스킬을 컨텍스트에 미리 로드하려면 여기에 `Skill`을 나열하는 대신 `skills` 필드를 사용하세요                       |
| `disallowedTools` | 아니오 | 거부할 도구. 상속되거나 지정된 목록에서 제거됩니다. `tools`와 동일한 형식입니다. `Bash(git push *)`와 같은 지정자가 있는 항목은 여전히 [전체 도구를 제거합니다](#available-tools)                                                                                                                                                                                                         |
| `model`           | 아니오 | 사용할 [모델](#choose-a-model): `sonnet`, `opus`, `haiku`, `fable`, `claude-opus-5-5`와 같은 전체 모델 ID, 또는 `inherit`. 생략하면 Claude Code는 [서브에이전트 모델 순서](#choose-a-model)에서 모델을 선택합니다                                                                                                                                                        |
| `permissionMode`  | 아니오 | [권한 모드](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, 또는 `manual`(기본값의 별칭). `manual` 별칭은 Claude Code v2.1.200 이상이 필요합니다. [플러그인 서브에이전트](#choose-the-subagent-scope)에서 무시됩니다                                                                                                        |
| `maxTurns`        | 아니오 | 서브에이전트가 중지되기 전의 최대 에이전트 턴 수입니다. 서브에이전트가 한계에 도달하면, Claude Code는 부분으로 표시된 출력을 반환하고, Claude는 [재개](#resume-subagents)하여 계속할 수 있습니다. 부분 표시는 Claude Code v2.1.246 이상이 필요합니다                                                                                                                                                             |
| `skills`          | 아니오 | 시작 시 서브에이전트의 컨텍스트에 미리 로드할 [스킬](/docs/ko/skills). 설명만이 아닌 전체 스킬 내용이 주입됩니다. 서브에이전트는 여전히 스킬 도구를 통해 나열되지 않은 프로젝트, 사용자, 및 플러그인 스킬을 호출할 수 있습니다                                                                                                                                                                                               |
| `mcpServers`      | 아니오 | 이 서브에이전트가 사용할 수 있는 [MCP 서버](/docs/ko/mcp). 각 항목은 이미 구성된 서버를 참조하는 서버 이름(예: `"slack"`) 또는 서버 이름을 키로 하고 전체 [MCP 서버 구성](/docs/ko/mcp#installing-mcp-servers)을 값으로 하는 인라인 정의입니다. [플러그인 서브에이전트](#choose-the-subagent-scope)에서 무시됩니다                                                                                                               |
| `hooks`           | 아니오 | 이 서브에이전트로 범위가 지정된 [라이프사이클 훅](#define-hooks-for-subagents). [플러그인 서브에이전트](#choose-the-subagent-scope)에서 무시됩니다                                                                                                                                                                                                                      |
| `memory`          | 아니오 | [지속적 메모리 범위](#enable-persistent-memory): `user`, `project`, 또는 `local`. 세션 간 학습을 활성화합니다                                                                                                                                                                                                                                           |
| `background`      | 아니오 | Claude가 포그라운드에서 실행하도록 요청할 때도 이 서브에이전트를 백그라운드에 유지하려면 `true`로 설정하세요. [포크 모드](#turn-fork-mode-on-or-off)가 켜져 있으면, Claude Code는 Claude가 생성하는 서브에이전트를 이미 [백그라운드에서 실행합니다](#run-subagents-in-foreground-or-background)                                                                                                                   |
| `omitClaudeMd`    | 아니오 | 이 서브에이전트를 사용자, 프로젝트, 및 로컬 CLAUDE.md 파일 없이 시작하려면 `true`로 설정하세요. [관리 정책 파일](/docs/ko/memory#how-claude-md-files-load)은 여전히 로드되지만, [관리 서브에이전트](#choose-the-subagent-scope) 제외. [위임 프롬프트](#what-loads-at-startup)에서 필요한 모든 것을 가져오는 서브에이전트에 사용하세요. 에이전트가 `--agent` 또는 `agent` 설정을 통해 주 세션 에이전트로 실행될 때 무시됩니다. Claude Code v2.1.271 이상이 필요합니다 |
| `effort`          | 아니오 | 이 서브에이전트가 활성화될 때의 노력 수준입니다. 세션 노력 수준을 재정의합니다. 기본값: 세션에서 상속합니다. 옵션: `low`, `medium`, `high`, `xhigh`, `max`. 사용 가능한 수준은 모델에 따라 다릅니다                                                                                                                                                                                                |
| `isolation`       | 아니오 | 서브에이전트를 임시 [git worktree](/docs/ko/worktrees)에서 실행하려면 `worktree`로 설정하여 저장소의 격리된 복사본을 제공합니다. 기본적으로 부모 세션의 `HEAD`가 아닌 [기본 분기](/docs/ko/worktrees#choose-the-base-branch)에서 분기됩니다. worktree는 서브에이전트가 변경을 하지 않으면 자동으로 정리됩니다                                                                                                                     |
| `color`           | 아니오 | 작업 목록 및 기록에서 서브에이전트의 표시 색상입니다. `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, 또는 `cyan`을 허용합니다                                                                                                                                                                                                                     |
| `initialPrompt`   | 아니오 | 이 에이전트가 주 세션 에이전트로 실행될 때(`--agent` 또는 `agent` 설정을 통해) 첫 번째 사용자 턴으로 자동 제출됩니다. [명령](/docs/ko/commands)과 [스킬](/docs/ko/skills)이 처리됩니다. 사용자 제공 프롬프트에 앞에 붙습니다. [플러그인 서브에이전트](#choose-the-subagent-scope)에서 무시됩니다                                                                                                                                 |
| `experimental`    | 아니오 | 실험적 옵션의 맵입니다. `cacheTtl` 키를 `5m` 또는 `1h`로 설정하여 이 서브에이전트의 요청에 대한 [프롬프트 캐시 수명](/docs/ko/prompt-caching#choose-the-ttl-yourself)을 선택합니다. [캐시 수명 우선순위](/docs/ko/prompt-caching#choose-the-ttl-yourself)에서 프론트매터의 위치입니다. Claude Code는 다른 값을 무시하고, Claude 구독이 사용 크레딧을 사용 중일 때 `1h`를 무시하며, 서브에이전트 파일에서만 필드를 읽습니다. Claude Code v2.1.248 이상이 필요합니다   |

`cacheTtl`을 프론트매터의 최상위 수준이 아닌 `experimental` 맵 내에 작성하세요.

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

<h4 id="subagent-files-claude-code-skips">
  Claude Code가 건너뛰는 서브에이전트 파일
</h4>

Claude Code는 프로젝트, 사용자, 또는 관리 `agents` 디렉토리의 파일, 또는 `--add-dir`으로 추가한 디렉토리 아래의 파일을 세션에서 보고하지 않고 건너뜁니다. 프론트매터에 다음 문제가 있을 때:

* **`name` 없음**: Claude Code는 파일을 에이전트 옆에 보관된 문서로 취급합니다.
* **파일의 첫 번째 줄이 아닌 여는 `---`**: Claude Code는 파일을 프론트매터가 없는 것으로 읽고 문서로 취급합니다.
* **`-`로 시작하거나 `:`를 포함하는 `name`**: Claude Code는 파일을 건너뛰고 디버그 로그에 오류를 씁니다. 위의 `name` 행을 참조하세요.
* **`name`이지만 `description` 없음**: Claude Code는 파일을 건너뛰고 이유를 디버그 로그에 씁니다.
* **파싱되지 않는 YAML**: Claude Code는 파일에서 필드를 읽지 않고, 건너뛰고, 파싱 오류를 디버그 로그에 씁니다.

디버그 로그를 보려면 `--debug`로 Claude Code를 실행하세요.

[플러그인 서브에이전트](/docs/ko/plugins/components#agents)의 프론트매터에 `name`이 없거나 파싱되지 않으면 여전히 파일 이름 아래에 로드됩니다.

<h5 id="check-an-agents-directory-before-a-session">
  세션 전에 `agents` 디렉토리 확인
</h5>

프론트매터가 파싱되지 않는 `agents` 디렉토리의 파일을 찾으려면, 예를 들어 `.claude/agents` 또는 `~/.claude/agents`에 대해 `claude plugin validate`를 실행하세요. Claude Code는 [이름을 지정한 디렉토리만](/docs/ko/plugins/cli-reference#validate-a-directory) 확인하고, 프론트매터가 파싱되지만 `name`이 없는 파일은 플래그하지 않습니다. Claude Code v2.1.233 이상이 필요합니다.

<h3 id="choose-a-model">
  모델 선택
</h3>

`model` 필드는 서브에이전트가 사용하는 모델을 제어합니다:

* **모델 별칭**: 사용 가능한 별칭 중 하나를 사용합니다: `sonnet`, `opus`, `haiku`, 또는 `fable`
* **전체 모델 ID**: `claude-opus-5-5` 또는 `claude-sonnet-5`와 같은 전체 모델 ID를 사용합니다. `--model` 플래그와 동일한 값을 허용합니다
* **inherit**: 주 대화와 동일한 모델을 사용합니다

Claude가 서브에이전트를 호출할 때, 해당 특정 호출에 대해 `model` 매개변수를 전달할 수도 있습니다. Claude Code는 이 순서로 서브에이전트의 모델을 확인합니다:

1. 호출별 `model` 매개변수
2. 서브에이전트 정의의 `model` 프론트매터. `inherit`는 주 대화의 모델을 선택합니다
3. [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/ko/model-config#environment-variables) 환경 변수. 모델 별칭 또는 모델 ID로 설정할 때
4. 주 대화의 모델

두 가지 경우에, 호출별 매개변수 또는 프론트매터의 `opus`와 같은 패밀리 별칭은 [별칭이 가리키는 버전](/docs/ko/model-config#model-aliases) 대신 주 대화의 모델로 확인됩니다:

* **주 대화의 모델이 해당 패밀리에 속함**: 서브에이전트는 주 대화의 정확한 모델(모든 `[1m]` 접미사 포함)에서 실행되므로 주 대화와 동일한 [확장 컨텍스트](/docs/ko/model-config#extended-context) 윈도우를 얻습니다.
* **Claude Code가 [Anthropic API 이외의 공급자](/docs/ko/third-party-integrations)에서 주 대화의 모델 패밀리를 알 수 없음**: 이는 Claude Code가 지원 모델로 확인하지 않은 Amazon Bedrock의 [애플리케이션 추론 프로필 ARN](/docs/ko/amazon-bedrock#iam-configuration)에서 발생할 수 있습니다. 이 경우는 `opus` 별칭만 다루며, [`ANTHROPIC_DEFAULT_OPUS_MODEL`](/docs/ko/model-config#environment-variables)을 설정할 때는 적용되지 않습니다. `opus`는 설정한 모델로 확인되기 때문입니다.

`CLAUDE_CODE_SUBAGENT_MODEL`의 별칭은 항상 별칭이 가리키는 버전으로 확인되며, 주 대화의 패밀리를 이름 지을 때도 마찬가지입니다.

`CLAUDE_CODE_SUBAGENT_MODEL`을 자체로 설정하는 것은 기본 제공 Explore 및 Plan 서브에이전트가 실행되는 모델을 변경하지 않습니다. 변경하려면 [모든 서브에이전트를 한 모델에서 실행](#run-every-subagent-on-one-model)을 참조하세요.

v2.1.251 이전에는 `CLAUDE_CODE_SUBAGENT_MODEL`이 이 순서에서 먼저 나왔으며 호출별 매개변수와 프론트매터(모델 상속 포함)를 모두 재정의했습니다.

변수를 `inherit`로 설정하는 것은 설정하지 않은 것과 동일합니다. v2.1.196 이전에는 해당 값이 서브에이전트를 주 대화의 모델로 강제하고 다른 소스를 무시했습니다.

Claude Code는 호출별 매개변수, 프론트매터, 및 환경 변수 값을 조직의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록에 대해 확인합니다. 차단된 값의 경우 다른 모델로 대체합니다:

* 차단된 값이 `opus`와 같은 패밀리 별칭일 때, Claude Code는 허용 목록이 허용하는 해당 패밀리의 최신 버전에서 서브에이전트를 실행합니다. `/model`과 동일한 [대체 규칙 및 공급자 범위](/docs/ko/model-config#restrict-model-selection)를 따릅니다. v2.1.222 이전에는 Claude Code가 차단된 패밀리 별칭에 대해서도 상속된 모델에서 서브에이전트를 실행했습니다.
* 다른 차단된 값의 경우, 해당 대체가 작동하지 않는 공급자에서 또는 허용 목록이 패밀리의 버전을 허용하지 않을 때, Claude Code는 상속된 모델에서 서브에이전트를 실행합니다. `CLAUDE_CODE_SUBAGENT_MODEL`을 설정하면, Claude Code는 이러한 동일한 규칙에 따라 먼저 해당 모델을 시도합니다.

대화형 세션에서, Claude Code는 두 대체 중 하나에 대해 요청된 모델과 서브에이전트가 실행되는 모델을 이름 지정하는 경고를 표시합니다.

서브에이전트가 실행 중인 모델을 확인하려면 [`/tasks`](/docs/ko/commands)를 실행하세요. Claude Code는 서브에이전트의 행에 모델을 이름 지으며, 서브에이전트의 정의 또는 포크된 스킬이 [`effort`](#supported-frontmatter-fields)를 설정할 때 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 추가합니다. Claude Code v2.1.242 이상이 필요합니다.

호출별 `model` 매개변수는 서브에이전트가 [재개되거나 후속 메시지를 받을](#resume-subagents) 때도 적용되므로 서브에이전트는 해당 모델에 머물러 있습니다. v2.1.211 이전에는 재개가 호출별 값을 떨어뜨렸고 서브에이전트는 정의의 `model` 필드로 되돌아가거나, 없으면 주 대화의 모델로 되돌아갔습니다.

v2.1.198부터, 서브에이전트는 주 대화의 [확장 사고](/docs/ko/model-config#extended-thinking) 구성도 상속합니다. 세션에서 사고가 켜져 있으면 서브에이전트에서도 켜져 있고, 꺼져 있으면 꺼져 있습니다. 서브에이전트별 사고 설정은 없습니다. v2.1.198 이전에는 서브에이전트가 주 대화의 설정과 관계없이 확장 사고가 비활성화된 상태로 실행되었습니다.

<h4 id="run-every-subagent-on-one-model">
  모든 서브에이전트를 한 모델에서 실행
</h4>

`CLAUDE_CODE_SUBAGENT_MODEL`은 기본값이므로 서브에이전트의 정의 또는 Claude가 전달하는 모델은 여전히 우선합니다. 모든 서브에이전트에 한 모델을 적용하려면, [팀원](/docs/ko/agent-teams#specify-teammates-and-models) 및 [워크플로우 에이전트](/docs/ko/workflows)도 `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`를 `1`로 설정하세요. Claude Code v2.1.257 이상이 필요합니다.

* 두 변수를 모두 설정하면, 서브에이전트는 `CLAUDE_CODE_SUBAGENT_MODEL`의 모델에서 실행됩니다.
* `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`만 설정하면, 서브에이전트는 주 대화의 모델에서 실행됩니다.

예를 들어, 모든 서브에이전트를 Haiku에서 실행하려면 [설정 파일](/docs/ko/settings)의 `env` 블록에서 두 변수를 설정하세요:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

설정이 적용되었는지 확인하려면 서브에이전트가 실행 중일 때 [`/tasks`](/docs/ko/commands)를 실행하세요. 서브에이전트의 행은 실행 중인 모델을 표시합니다.

`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`가 [켜져](/docs/ko/env-vars) 있는 동안, Claude Code는 기본 제공 Explore 및 Plan 서브에이전트를 포함한 모든 서브에이전트 정의의 `model` 필드를 무시하고, Claude는 서브에이전트를 시작할 때 모델을 전달할 수 없습니다. 두 종류의 서브에이전트는 여전히 주 대화의 모델에서 실행됩니다:

* [포크](#fork-the-current-conversation)
* `model: inherit`를 가진 [서브에이전트에서 실행되는 스킬](/docs/ko/skills#run-skills-in-a-subagent)

`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`만 설정하면, 기본 제공 Explore 서브에이전트는 [모델 상한](#built-in-subagents)을 유지합니다.

<h3 id="control-subagent-capabilities">
  서브에이전트 기능 제어
</h3>

도구 접근, 권한 모드, 및 조건부 규칙을 통해 서브에이전트가 할 수 있는 일을 제어할 수 있습니다.

<h4 id="available-tools">
  사용 가능한 도구
</h4>

서브에이전트는 주 대화에서 사용 가능한 [기본 제공 도구](/docs/ko/tools-reference)와 MCP 도구를 상속하며, 두 필터로 좁혀집니다. 첫 번째는 모든 서브에이전트에서 짧은 도구 목록을 제거하고, 두 번째는 [백그라운드](#run-subagents-in-foreground-or-background)에서 실행되는 서브에이전트(기본값)의 기본 제공 도구 세트를 줄입니다. macOS, Linux, 및 WSL에서, 주 대화가 이들을 가지지 않을 때 서브에이전트는 Glob 및 Grep 도구를 받을 수도 있습니다. [Glob 도구 동작](/docs/ko/tools-reference#glob-tool-behavior) 아래에서 설명합니다. [포크](#fork-the-current-conversation)는 두 필터를 건너뛰고 주 대화의 정확한 도구 풀을 받습니다. 첫 번째 필터는 `tools` 필드에 나열되어 있어도 이러한 도구를 제거합니다:

* `Agent`. 서브에이전트가 [깊이 한계](#let-subagents-spawn-their-own-subagents)에 있을 때. [포크](#fork-the-current-conversation)에서 도구는 나열된 상태로 유지되지만 생성 대신 오류를 반환합니다
* `AskUserQuestion`
* `EndConversation`. 주 대화만 종료할 수 있습니다. [EndConversation 도구 동작](/docs/ko/tools-reference#endconversation-tool-behavior)을 참조하세요
* `EnterPlanMode`
* `ExitPlanMode`. 서브에이전트의 [`permissionMode`](#permission-modes)가 `plan`이 아닌 한
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

두 번째 필터는 백그라운드에서 실행되는 서브에이전트에 적용됩니다. `Agent`와 `ExitPlanMode`를 제외하고(이들은 서브에이전트가 실행되는 곳 어디든 첫 번째 필터의 조건을 따릅니다), 백그라운드 서브에이전트는 모든 MCP 도구를 유지하지만 이러한 기본 제공 도구만 유지합니다: `Read`, `Grep`, `Glob`, `LSP`, `Bash`, `PowerShell`, `Edit`, `Write`, `NotebookEdit`, `WebFetch`, `WebSearch`, `TodoWrite`, `Skill`, `ToolSearch`, `EnterWorktree`, `ExitWorktree`, `Monitor`, `TaskStop`, `SendMessage`, 및 `Artifact`. 그리고 이를 통해 보고하는 서브에이전트의 경우 [`SubagentHandoff`](/docs/ko/tools-reference). Claude Code는 백그라운드 서브에이전트에서 다른 모든 기본 제공 도구를 제거합니다. 상속되거나 `tools` 필드에 나열되어 있든 상관없이, 제거가 `tools` 목록을 [아무것도 확인하지 않는 것으로](/docs/ko/errors#agent-would-be-spawned-with-zero-tools) 남기지 않으면 오류를 보고하지 않습니다.

v2.1.280 이전에는 백그라운드 서브에이전트가 `LSP`를 사용할 수 없었습니다.

[`ListAgents`](/docs/ko/cross-session-messaging)는 다른 기본 제공 도구처럼 이러한 필터를 따릅니다. 포그라운드 서브에이전트는 세션에서 세션 간 메시징이 활성화된 경우 이를 상속하고, 백그라운드 서브에이전트는 유지하지 않습니다.

[에이전트 팀](/docs/ko/agent-teams)의 팀원은 추가로 작업 도구와 cron 도구를 유지합니다: `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `CronCreate`, `CronDelete`, 및 `CronList`.

[작업 도구가 없는 세션](/docs/ko/tools-reference#task-tool-availability)에서, Claude Code는 서브에이전트가 다른 모델을 실행하더라도 서브에이전트에 작업 도구를 제공하지 않습니다. 프로세스 내 팀원은 세션을 따르는 동일한 방식이며, [분할 창](/docs/ko/agent-teams#choose-a-display-mode)에서 자신의 팀원은 별도의 Claude Code 프로세스로 실행되므로 자신의 모델이 결정합니다.

도구를 제한하려면 `tools` 필드를 허용 목록으로 또는 `disallowedTools` 필드를 거부 목록으로 사용하세요. 이 예제는 `tools`를 사용하여 Read, Grep, Glob, 및 Bash만 허용합니다. 서브에이전트는 파일을 편집하거나, 파일을 작성하거나, MCP 도구를 사용할 수 없습니다:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

이 예제는 `disallowedTools`를 사용하여 Write와 Edit를 제외한 서브에이전트의 도구 풀을 상속합니다. 서브에이전트는 Bash, MCP 도구, 및 풀의 나머지를 유지합니다:

```yaml theme={null}
---
name: no-writes
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

둘 다 설정되면, `disallowedTools`가 먼저 적용되고, `tools`는 남은 풀에 대해 확인됩니다. 둘 다에 나열된 도구는 제거됩니다.

`tools` 목록의 아무것도 도구로 확인되지 않을 때(예: 모든 항목이 철자가 틀렸거나 서브에이전트가 사용할 수 없는 도구를 이름 지을 때), Claude Code는 일반적으로 서브에이전트 시작을 거부하고 Agent 도구는 확인되지 않은 항목을 이름 지정하는 오류를 반환합니다. [에이전트가 0개 도구로 생성될 것입니다](/docs/ko/errors#agent-would-be-spawned-with-zero-tools)를 참조하여 메시지와 각 항목을 수정하는 방법을 확인하세요. v2.1.208 이전에는 해당 서브에이전트가 도구 없이 시작되고 빈 또는 혼란스러운 결과를 반환할 수 있었습니다.

두 필드는 정확한 도구 이름 외에도 MCP 서버 수준 패턴을 허용합니다: `mcp__<server>` 또는 `mcp__<server>__*`는 이름이 지정된 서버의 모든 도구를 부여하거나 제거합니다. `disallowedTools`에서 `mcp__*`는 모든 서버의 모든 MCP 도구를 제거합니다. 이 예제는 `github` MCP 서버의 모든 도구를 제거하면서 다른 서버의 도구와 풀의 기본 제공 도구를 유지합니다:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

`Bash(git push *)`와 같은 지정자가 있는 `disallowedTools` 항목은 여전히 서브에이전트에서 전체 도구를 제거하며, 일치하는 명령만 제거하지 않습니다. Bash를 유지하고 특정 명령을 차단하려면, 설정의 `permissions.deny`에 `Bash(git push *)`와 같은 [Bash 거부 규칙](/docs/ko/permissions#bash)을 추가하세요. 규칙은 주 대화와 서브에이전트에 적용됩니다.

<h4 id="restrict-which-subagents-can-be-spawned">
  생성할 수 있는 서브에이전트 제한
</h4>

에이전트가 `claude --agent`로 주 스레드로 실행될 때, Agent 도구를 사용하여 서브에이전트를 생성할 수 있습니다. 생성할 수 있는 서브에이전트 유형을 제한하려면 `tools` 필드에서 `Agent(agent_type)` 구문을 사용하세요.

<Note>버전 2.1.63에서 Task 도구는 Agent로 이름이 바뀌었습니다. 설정 및 에이전트 정의의 기존 `Task(...)` 참조는 여전히 별칭으로 작동합니다.</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

이것은 허용 목록입니다. `worker`와 `researcher` 서브에이전트만 생성할 수 있습니다. 에이전트가 다른 유형을 생성하려고 하면 요청이 실패하고 에이전트는 프롬프트에서 허용된 유형만 봅니다. 특정 에이전트를 차단하면서 다른 모든 에이전트를 허용하려면 [`permissions.deny`](#disable-specific-subagents)를 대신 사용하세요.

제한 없이 모든 서브에이전트 생성을 허용하려면 괄호 없이 `Agent`를 사용하세요:

```yaml theme={null}
tools: Agent, Read, Bash
```

`tools` 목록에서 `Agent`를 완전히 생략하면, 에이전트는 Agent 도구로 서브에이전트를 생성할 수 없습니다.

`Agent(agent_type)` 허용 목록 구문은 `claude --agent`로 주 스레드로 실행되는 에이전트에만 적용됩니다. 서브에이전트 정의에서 `tools`에 `Agent`를 나열하면 해당 서브에이전트가 [깊이 한계](#let-subagents-spawn-their-own-subagents)가 허용하는 동안 자신의 서브에이전트를 생성할 수 있지만, 괄호 내의 모든 유형 목록은 무시됩니다.

<h4 id="scope-mcp-servers-to-a-subagent">
  MCP 서버를 서브에이전트로 범위 지정
</h4>

`mcpServers` 필드를 사용하여 주 대화에서 사용할 수 없는 [MCP](/docs/ko/mcp) 서버에 대한 접근을 서브에이전트에 제공합니다. 여기에 정의된 인라인 서버는 서브에이전트가 시작될 때 연결되며, [에이전트 파일의 폴더에 대한 신뢰 규칙](#inline-server-trust)의 적용을 받고, 완료될 때 연결이 끊깁니다. 문자열 참조는 부모 세션의 연결을 공유합니다.

<Note>
  `mcpServers` 필드는 에이전트 파일이 실행될 수 있는 두 컨텍스트에 적용됩니다:

  * Agent 도구 또는 @-멘션을 통해 생성된 서브에이전트
  * [`--agent`](#invoke-subagents-explicitly) 또는 `agent` 설정으로 시작된 주 세션

  에이전트가 주 세션일 때, 인라인 서버 정의는 [`.mcp.json`](/docs/ko/mcp)과 설정 파일의 서버와 함께 시작 시 연결되며, [에이전트 파일의 폴더에 대한 신뢰 규칙](#inline-server-trust)과 동일합니다. `/mcp`에서, 이전에 사용한 원격(HTTP 또는 SSE) 서버는 [`cached` 상태](/docs/ko/mcp#managing-your-servers) 대신 표시될 수 있습니다. Claude Code는 Claude가 처음 도구 중 하나를 호출할 때 연결합니다.
</Note>

목록의 각 항목은 인라인 서버 정의 또는 세션에서 이미 구성된 MCP 서버를 참조하는 문자열입니다:

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

인라인 정의는 `.mcp.json` 서버 항목과 동일한 스키마를 사용하며, 서버 이름으로 키 지정되고, `stdio`, `http`, `sse`, 및 `ws` 유형을 지원합니다.

MCP 서버를 주 대화에서 완전히 제외하고 도구 설명이 컨텍스트를 소비하는 것을 피하려면, `.mcp.json`이 아닌 여기에 인라인으로 정의하세요. 서브에이전트는 도구를 얻습니다. 부모 대화는 그렇지 않습니다.

<span id="inline-server-trust" />Claude Code는 프로젝트의 `.claude/agents/` 디렉토리 또는 `--add-dir` 디렉토리의 `.claude/agents/`에 있는 에이전트 파일에서 인라인 서버를 로드합니다. 에이전트 파일이 나온 폴더를 [신뢰](/docs/ko/permissions#what-runs-before-you-trust-a-folder)한 후에만 가능합니다. v2.1.238 이전에는 Claude Code가 신뢰를 확인하지 않고 이러한 서버를 로드했습니다.

* **신뢰하지 않는 것**: 부모 폴더의 신뢰, 및 `-p` 또는 SDK 세션이 [설정 파일의 훅](/docs/ko/permissions#what-runs-before-you-trust-a-folder)에 대해 얻는 자동 신뢰
* **그때까지**: Claude Code는 해당 에이전트 파일의 모든 인라인 서버를 건너뛰고 `~/.claude.json`에 대한 정확한 `projects["<path>"].hasTrustDialogAccepted` 키를 디버그 로그에 씁니다
* **`--add-dir` 디렉토리**: 신뢰된 작업 공간의 저장소 외부의 디렉토리는 자신의 신뢰 항목이 필요합니다. 작업 공간의 신뢰를 상속하지 않기 때문입니다

Claude Code는 에이전트 파일이 나온 폴더에 대한 신뢰를 확인하지 않고 두 종류의 서버를 로드합니다:

* 이미 구성한 서버를 참조하는 이름
* `~/.claude/agents/`의 에이전트 파일, `--agents` 또는 SDK `agents` 옵션으로 전달하는 파일, 또는 관리 설정이 제공하는 파일의 인라인 서버

주 세션에 적용되는 MCP 제한은 서브에이전트 프론트매터에 선언된 서버도 다룹니다:

* [`--strict-mcp-config`](/docs/ko/cli-reference) 및 [`--bare`](/docs/ko/cli-reference)
* [엔터프라이즈 관리 MCP 구성](/docs/ko/managed-mcp)
* [`allowedMcpServers` 및 `deniedMcpServers` 정책](/docs/ko/managed-mcp#policy-based-control-with-allowlists-and-denylists)

이 중 하나가 서버를 차단하면, Claude Code는 건너뛰고 차단된 서버를 이름 지정하는 경고를 표시합니다.

관리 설정 제한은 정의 방식과 관계없이 모든 서브에이전트에 적용됩니다. `--strict-mcp-config`는 `--agents` 또는 SDK `agents` 옵션을 통해 인라인으로 전달하는 서버를 필터링하지 않습니다. 이들은 명시적 호출자 입력이기 때문입니다.

<h4 id="permission-modes">
  권한 모드
</h4>

`permissionMode`를 설정하여 서브에이전트가 실행되는 권한 모드를 선택합니다. 모드의 구성 값을 사용하므로 수동 모드는 `default`입니다. 설정하지 않으면, 서브에이전트는 주 대화의 모드를 상속합니다. 주 대화의 모드는 Pro, Max, 및 Team 플랜에서 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)로 시작하거나, 설정 또는 조직이 변경하지 않으면 시작합니다.

주 대화의 권한 모드는 Claude Code가 설정한 값을 사용하는지 결정합니다:

* 주 대화가 `bypassPermissions`, `acceptEdits`, 또는 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에 있을 때, 서브에이전트는 동일한 모드에서 실행되고 Claude Code는 설정한 `permissionMode`를 무시합니다. 자동 모드에서, 분류자는 주 대화의 차단 및 허용 규칙으로 서브에이전트의 도구 호출을 평가합니다. 서브에이전트가 완료되면, 분류자는 보고서가 전달되기 전에 작업과 최종 보고서도 검토합니다. [자동 모드가 서브에이전트를 처리하는 방법](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)을 참조하세요.
* 주 대화가 `default`, `dontAsk`, 또는 `plan` 모드에 있을 때, 서브에이전트는 설정한 권한 모드에서 실행됩니다. `bypassPermissions` 제외. `bypassPermissions`를 선언하는 서브에이전트는 주 대화의 모드를 대신 유지합니다. `bypassPermissions` 예외는 Claude Code v2.1.267 이상이 필요합니다.

`permissionMode`는 이러한 값을 허용하며, `manual`을 `default`의 별칭으로 허용합니다:

| 모드                  | 동작                                                                                                                                                                                                                                                                      |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | 수동 모드: 권한 프롬프트                                                                                                                                                                                                                                                          |
| `acceptEdits`       | 자동 수락 파일 편집 및 작업 디렉토리 또는 `additionalDirectories`의 경로에 대한 일반적인 파일시스템 명령                                                                                                                                                                                                  |
| `auto`              | [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode): 백그라운드 분류자가 명령 및 보호된 디렉토리 쓰기를 검토합니다                                                                                                                                                                      |
| `dontAsk`           | 자동 거부 권한 프롬프트. 명시적으로 허용된 도구는 여전히 작동합니다. `AskUserQuestion`, [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구, 및 조직이 설정한 커넥터 도구 [`ask`](/docs/ko/mcp#organization-controls-on-connector-tools)는 설정이 Claude Code에 도달하는 세션에서 허용되었더라도 거부됩니다 |
| `bypassPermissions` | [권한 프롬프트 건너뛰기](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode). 서브에이전트는 주 대화도 이 모드에 있을 때만 이 모드에서 실행됩니다                                                                                                                                                |
| `plan`              | 계획 모드(읽기 전용 탐색)                                                                                                                                                                                                                                                         |

<h4 id="preload-skills-into-subagents">
  서브에이전트에 스킬 미리 로드
</h4>

`skills` 필드를 사용하여 시작 시 서브에이전트의 컨텍스트에 스킬 내용을 주입합니다. 이는 실행 중에 스킬을 발견하고 로드하도록 요구하지 않고 서브에이전트에 도메인 지식을 제공합니다.

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

나열된 각 스킬의 전체 내용은 시작 시 서브에이전트의 컨텍스트에 주입됩니다. 이 필드는 서브에이전트가 실행 중에 발견하고 호출할 수 있는 스킬을 제어하지 않습니다. 이 필드는 미리 로드할 스킬을 제어합니다. 이 필드 없이, 서브에이전트는 여전히 실행 중에 스킬 도구를 통해 프로젝트, 사용자, 및 플러그인 스킬을 발견하고 호출할 수 있습니다. 서브에이전트가 스킬을 완전히 호출하지 못하도록 하려면, [`tools`](#available-tools) 목록에서 `Skill`을 생략하거나 `disallowedTools`에 추가하세요.

[`disable-model-invocation: true`](/docs/ko/skills#control-who-invokes-a-skill)를 설정하는 스킬은 미리 로드할 수 없습니다. 미리 로드는 Claude가 호출할 수 있는 동일한 스킬 세트에서 그리기 때문입니다. 여기에는 번들된 `/verify` 스킬이 포함됩니다. 오직 사용자만 실행할 수 있으므로 미리 로드할 수 없습니다.

나열된 스킬이 누락되거나 비활성화되면(예: 조직의 정책에 의해), Claude Code는 건너뛰고 디버그 로그에 경고를 기록합니다.

<Note>
  이것은 [서브에이전트에서 스킬 실행](/docs/ko/skills#run-skills-in-a-subagent)의 역입니다. 서브에이전트의 `skills`를 사용하면, 서브에이전트가 시스템 프롬프트를 제어하고 스킬 내용을 로드합니다. 스킬의 `context: fork`를 사용하면, 스킬 내용이 지정한 에이전트에 주입됩니다. 두 경우 모두 서브에이전트는 대화 기록 없이 시작됩니다.
</Note>

<h4 id="enable-persistent-memory">
  지속적 메모리 활성화
</h4>

`memory` 필드는 서브에이전트에 대화 간에 지속되는 지속적 디렉토리를 제공합니다. 서브에이전트는 이 디렉토리를 사용하여 시간이 지남에 따라 지식을 구축합니다. 예를 들어 코드베이스 패턴, 디버깅 통찰력, 및 아키텍처 결정입니다.

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

메모리가 얼마나 광범위하게 적용되어야 하는지에 따라 범위를 선택합니다:

| 범위        | 위치                                            | 사용 시기                                       |
| :-------- | :-------------------------------------------- | :------------------------------------------ |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | 서브에이전트가 모든 프로젝트에서 학습을 기억해야 할 때              |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | 서브에이전트의 지식이 프로젝트 특정이고 버전 관리를 통해 공유 가능할 때    |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | 서브에이전트의 지식이 프로젝트 특정이지만 버전 관리에 체크인되지 않아야 할 때 |

서브에이전트 메모리는 [자동 메모리](/docs/ko/memory#auto-memory)의 일부입니다. `autoMemoryEnabled` 설정 또는 `CLAUDE_CODE_DISABLE_AUTO_MEMORY`로 자동 메모리를 끄면, `memory` 필드는 효과가 없고 서브에이전트는 메모리 지침 또는 아래에서 설명한 메모리 도구 접근 없이 시작됩니다.

메모리가 활성화되면:

* 서브에이전트의 시스템 프롬프트는 메모리 디렉토리에 읽고 쓰기 위한 지침을 포함합니다.
* 서브에이전트의 시스템 프롬프트는 또한 메모리 디렉토리의 `MEMORY.md`의 첫 200줄 또는 25KB(둘 중 먼저 오는 것)를 포함하며, 초과하면 `MEMORY.md`를 큐레이션하기 위한 지침을 포함합니다.
* Read, Write, 및 Edit 도구는 자동으로 활성화되어 서브에이전트가 메모리 파일을 관리할 수 있습니다.

<h5 id="persistent-memory-tips">
  지속적 메모리 팁
</h5>

* `project`는 권장되는 기본 범위입니다. 서브에이전트 지식을 버전 관리를 통해 공유 가능하게 만듭니다.
* 서브에이전트에 작업을 시작하기 전에 메모리를 확인하도록 요청하세요: "이 PR을 검토하고 이전에 본 패턴에 대해 메모리를 확인하세요."
* 작업을 완료한 후 메모리를 업데이트하도록 서브에이전트에 요청하세요: "이제 완료했으니, 배운 것을 메모리에 저장하세요." 시간이 지남에 따라 이는 서브에이전트를 더 효과적으로 만드는 지식 기반을 구축합니다.
* 메모리 지침을 서브에이전트의 마크다운 파일에 직접 포함하여 자신의 지식 기반을 적극적으로 유지하도록 합니다:

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  훅을 사용한 조건부 규칙
</h4>

도구 사용을 더 동적으로 제어하려면 `PreToolUse` 훅을 사용하여 실행 전에 작업을 검증합니다. 도구의 일부 작업을 허용하면서 다른 작업을 차단해야 할 때 유용합니다.

이 예제는 읽기 전용 데이터베이스 쿼리만 허용하는 서브에이전트를 생성합니다. `PreToolUse` 훅은 각 Bash 명령이 실행되기 전에 `command`에 지정된 스크립트를 실행합니다:

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code는 [훅 입력을 JSON으로](/docs/ko/hooks#pretooluse-input) stdin을 통해 훅 명령에 전달합니다. 검증 스크립트는 이 JSON을 읽고, Bash 명령을 추출하고, [코드 2로 종료](/docs/ko/hooks#exit-code-2-behavior-per-event)하여 쓰기 작업을 차단합니다:

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

macOS 및 Linux에서 스크립트를 실행 가능하게 만들거나, 훅이 실패하고 아무것도 차단하지 않습니다:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

규칙을 테스트하려면 서브에이전트에 `UPDATE` 문을 실행하도록 요청하세요. 스크립트는 코드 2로 종료되고, Claude Code는 명령을 차단하고, 서브에이전트는 `Blocked: Only SELECT queries are allowed` 메시지를 봅니다.

[훅 입력](/docs/ko/hooks#pretooluse-input)에 대한 전체 입력 스키마와 [종료 코드](/docs/ko/hooks#exit-code-output)를 참조하여 종료 코드가 동작에 미치는 영향을 확인하세요. Windows에서 PowerShell로 훅 스크립트를 작성하고 [PowerShell에서 훅 실행](/docs/ko/hooks#windows-powershell-tool)에 표시된 대로 훅 항목에 `shell: powershell`을 추가하세요.

<h4 id="disable-specific-subagents">
  특정 서브에이전트 비활성화
</h4>

[설정](/docs/ko/settings-reference#permission-settings)의 `deny` 배열에 추가하여 Claude가 특정 서브에이전트를 사용하지 못하도록 할 수 있습니다. `Agent(subagent-name)` 형식을 사용합니다. 여기서 `subagent-name`은 서브에이전트의 name 필드와 일치합니다.

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

이는 기본 제공 및 사용자 정의 서브에이전트 모두에 작동합니다. `--disallowedTools` CLI 플래그를 사용할 수도 있습니다:

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

[권한 문서](/docs/ko/permissions#tool-specific-permission-rules)에서 권한 규칙에 대한 자세한 내용을 참조하세요.

<h3 id="define-hooks-for-subagents">
  서브에이전트에 대한 훅 정의
</h3>

서브에이전트는 서브에이전트의 라이프사이클 중에 실행되는 [훅](/docs/ko/hooks)을 정의할 수 있습니다. 훅을 구성하는 두 가지 방법이 있습니다:

* **서브에이전트의 프론트매터에서**: 해당 서브에이전트가 활성화된 동안만 실행되는 훅을 정의합니다
* **`settings.json`에서**: 서브에이전트 내에서도 발생하는 세션 전체 훅을 정의합니다. `PreToolUse`와 `PostToolUse`와 같은 도구 이벤트는 주 대화에서와 동일한 방식으로 서브에이전트의 도구 호출에 대해 발생하고, `SubagentStart`와 `SubagentStop`은 서브에이전트가 시작되거나 완료될 때 발생합니다

[설정 파일, 관리 정책 설정, 및 플러그인](/docs/ko/hooks#hook-locations)의 훅은 모두 서브에이전트 내에서 적용되므로, `settings.json`의 `PreToolUse` 훅은 서브에이전트가 사용하는 모든 도구 전에도 실행됩니다.

<h4 id="hooks-in-subagent-frontmatter">
  서브에이전트 프론트매터의 훅
</h4>

서브에이전트의 마크다운 파일에 직접 훅을 정의합니다. 이러한 훅은 해당 특정 서브에이전트가 활성화된 동안만 실행되고 완료될 때 정리됩니다.

<Note>
  프론트매터 훅은 에이전트가 Agent 도구 또는 @-멘션을 통해 서브에이전트로 생성될 때, 그리고 에이전트가 [`--agent`](#invoke-subagents-explicitly) 또는 `agent` 설정을 통해 주 세션으로 실행될 때 발생합니다. 주 세션 경우에는 [`settings.json`](/docs/ko/hooks)에 정의된 모든 훅과 함께 실행됩니다.
</Note>

프로젝트 수준 서브에이전트의 프론트매터 훅이 실행되도록 하려면, 에이전트 파일이 포함된 폴더에 대한 [작업 공간 신뢰 대화](/docs/ko/permissions#project-allow-rules-and-workspace-trust)를 수락하세요. `~/.claude/agents/`의 사용자 수준 서브에이전트의 훅과 `--agents`로 전달하는 정의의 훅은 이 단계 없이 실행됩니다. `--add-dir`로 신뢰된 작업 공간의 저장소 외부에서 폴더를 추가한 경우, 해당 폴더를 별도로 신뢰하세요. `.claude/agents/` 훅은 작업 공간의 신뢰를 상속하지 않습니다.

폴더를 신뢰할 때까지, 서브에이전트는 여전히 실행되지만 Claude Code는 프론트매터 훅을 건너뛰고 폴더를 신뢰하는 방법을 설명하는 오류를 디버그 로그에 기록합니다. 이는 설정 파일의 훅에 대한 규칙보다 더 엄격합니다. 부모 폴더를 신뢰하는 것으로는 충분하지 않으며, `-p` 세션은 신뢰된 것으로 계산되지 않습니다. [폴더를 신뢰하기 전에 실행되는 것](/docs/ko/permissions#what-runs-before-you-trust-a-folder)은 두 가지를 비교합니다. v2.1.218 이전에는 신뢰하지 않은 폴더(비대화형 세션 포함)에서 프론트매터 훅을 실행할 수 있었습니다.

모든 [훅 이벤트](/docs/ko/hooks#hook-events)가 지원됩니다. 서브에이전트에 가장 일반적인 이벤트는:

| 이벤트           | 매처 입력 | 발생 시기                                    |
| :------------ | :---- | :--------------------------------------- |
| `PreToolUse`  | 도구 이름 | 서브에이전트가 도구를 사용하기 전                       |
| `PostToolUse` | 도구 이름 | 서브에이전트가 도구를 사용한 후                        |
| `Stop`        | (없음)  | 서브에이전트가 완료될 때(런타임에 `SubagentStop`으로 변환됨) |

이 예제는 `PreToolUse` 훅으로 Bash 명령을 검증하고 `PostToolUse`로 파일 편집 후 린터를 실행합니다:

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

에이전트가 서브에이전트로 호출될 때, 프론트매터의 `Stop` 훅은 자동으로 `SubagentStop` 이벤트로 변환됩니다.

<h4 id="project-level-hooks-for-subagent-events">
  서브에이전트 이벤트에 대한 프로젝트 수준 훅
</h4>

주 세션에서 서브에이전트 라이프사이클 이벤트에 응답하는 `settings.json`의 훅을 구성합니다.

| 이벤트             | 매처 입력      | 발생 시기             |
| :-------------- | :--------- | :---------------- |
| `SubagentStart` | 에이전트 유형 이름 | 서브에이전트가 실행을 시작할 때 |
| `SubagentStop`  | 에이전트 유형 이름 | 서브에이전트가 완료될 때     |

두 이벤트 모두 이름으로 특정 에이전트 유형을 대상으로 하는 매처를 지원합니다. 매처 값은 프로젝트 수준 및 사용자 수준 서브에이전트의 프론트매터 `name`이거나, [플러그인 서브에이전트](/docs/ko/plugins/components#agents)의 `my-plugin:db-agent`와 같은 플러그인 범위 식별자입니다. 범위가 지정된 이름은 콜론을 포함하므로 [고정되지 않은 정규식](/docs/ko/hooks#matcher-patterns)으로 평가됩니다. `^my-plugin:db-agent$`와 같이 `^`와 `$`로 고정하여 해당 에이전트만 일치시킵니다.

이 예제는 `db-agent` 서브에이전트가 시작될 때만 설정 스크립트를 실행하고 모든 서브에이전트가 중지될 때 정리 스크립트를 실행합니다:

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

하이픈이 있는 매처 `db-agent`는 Claude Code v2.1.195 이상에서 정확히 일치합니다. 이전 버전에서는 고정되지 않은 정규식으로 평가되고 `prod-db-agent`와 같이 포함하는 모든 에이전트 유형에 대해서도 발생합니다. 이러한 버전에서는 `^db-agent$`로 고정하세요.

[훅](/docs/ko/hooks)에서 전체 훅 구성 형식을 참조하세요.

<h2 id="work-with-subagents">
  Subagent 작업
</h2>

<h3 id="understand-automatic-delegation">
  자동 위임 이해
</h3>

Claude는 요청의 작업 설명, subagent 구성의 `description` 필드, 현재 컨텍스트를 기반으로 자동으로 작업을 위임합니다. 적극적인 위임을 장려하려면 subagent의 description 필드에 "use proactively"와 같은 구문을 포함합니다.

설명을 간결하게 유지합니다: Claude Code는 subagent의 결합된 설명이 [15,000토큰 제한](/docs/ko/errors#agent-descriptions-are-over-the-15000-token-limit)을 초과할 때 시작 경고를 표시하며, 여전히 모든 subagent를 로드합니다.

subagent가 [플러그인](/docs/ko/plugins/overview)에 포함되어 있으면 현실적인 프롬프트에서 Claude가 이를 얼마나 안정적으로 위임하는지 한 번에 하나씩 확인하는 대신 측정할 수 있습니다: [`claude plugin eval`](/docs/ko/plugin-evals)은 플러그인 포함 여부와 관계없이 각 프롬프트를 실행하고 결과를 점수 매깁니다.

<h3 id="invoke-subagents-explicitly">
  Subagent를 명시적으로 호출
</h3>

자동 위임이 충분하지 않을 때 subagent를 직접 요청할 수 있습니다. 일회성 제안에서 세션 전체 기본값으로 확대되는 세 가지 패턴이 있습니다:

* **자연어**: 프롬프트에서 subagent 이름을 지정합니다. Claude가 위임할지 결정합니다
* **@-mention**: 한 작업에 대해 subagent가 실행되도록 보장합니다
* **세션 전체**: 전체 세션이 `--agent` 플래그 또는 `agent` 설정을 통해 해당 subagent의 시스템 프롬프트, 도구 제한 및 모델을 사용합니다

자연어의 경우 특별한 구문이 없습니다. Subagent 이름을 지정하면 Claude는 일반적으로 위임합니다:

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**Subagent를 @-mention합니다.** `@`를 입력하고 파일을 @-mention하는 것과 동일한 방식으로 typeahead에서 subagent를 선택합니다. 이렇게 하면 Claude가 선택하도록 하는 대신 특정 subagent가 실행되도록 보장합니다:

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

전체 메시지는 여전히 Claude로 이동하며, Claude는 요청한 내용을 기반으로 subagent의 작업 프롬프트를 작성합니다. @-mention은 Claude가 호출하는 subagent를 제어하며, 받는 프롬프트는 제어하지 않습니다.

활성화된 [플러그인](/docs/ko/plugins/overview)에서 제공하는 Subagent는 typeahead에 `my-plugin:code-reviewer` 또는 플러그인이 [agents를 하위 폴더로 구성](#choose-the-subagent-scope)할 때 `my-plugin:review:security`와 같은 범위가 지정된 이름으로 나타납니다. 세션에서 현재 실행 중인 명명된 background subagent도 typeahead에 나타나며 이름 옆에 상태를 표시합니다.

선택기를 사용하지 않고 수동으로 mention을 입력할 수도 있습니다: 로컬 subagent의 경우 `@agent-<name>`, 플러그인 subagent의 경우 범위가 지정된 이름 뒤에 `@agent-`를 입력합니다. 예를 들어 `@agent-my-plugin:code-reviewer`입니다. 이 형식을 입력하는 동안 typeahead는 에이전트가 아닌 파일 일치를 표시합니다. 에이전트 mention은 제출할 때 여전히 해결됩니다.

**전체 세션을 subagent로 실행합니다.** [`--agent <name>`](/docs/ko/cli-reference)을 전달하여 주 스레드 자체가 해당 subagent의 시스템 프롬프트, 도구 제한 및 모델을 취하는 세션을 시작합니다:

```bash theme={null}
claude --agent code-reviewer
```

Subagent의 시스템 프롬프트는 [`--system-prompt`](/docs/ko/cli-reference)와 동일한 방식으로 기본 Claude Code 시스템 프롬프트를 완전히 대체합니다. `CLAUDE.md` 파일 및 프로젝트 메모리는 여전히 일반적인 메시지 흐름을 통해 로드됩니다. 에이전트의 정의가 [`omitClaudeMd`](#supported-frontmatter-fields)를 설정하더라도 로드됩니다. 에이전트 이름은 시작 헤더에 `@<name>`으로 나타나므로 활성화되었는지 확인할 수 있습니다.

이것은 내장 및 사용자 정의 subagent에서 작동하며, 세션을 재개할 때 선택이 유지됩니다: Claude Code는 에이전트의 도구 제한 및 모델을 대화와 함께 복원합니다. 세션을 재개할 때 에이전트가 더 이상 존재하지 않으면 세션은 기본 도구로 계속되며 [에이전트 이름을 지정하는 경고](/docs/ko/errors#session-agent-no-longer-available)를 표시합니다. 시스템 프롬프트의 경우 [재개된 대화에서 시스템 프롬프트 플래그](/docs/ko/cli-reference#system-prompt-flags-in-resumed-conversations)를 참조합니다.

플러그인 제공 subagent의 경우 에이전트 이름만 전달하면 Claude Code가 찾을 수 있습니다:

```bash theme={null}
claude --agent security-reviewer
```

여러 플러그인이 동일한 이름의 에이전트를 제공하는 경우 범위가 지정된 이름을 전달하여 구분합니다:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

플러그인이 에이전트를 `agents/` 디렉토리의 하위 폴더에 배치하면 범위가 지정된 이름에 하위 폴더를 포함합니다. 예를 들어 `claude --agent my-plugin:review:security`입니다.

프로젝트의 모든 세션에 대한 기본값으로 만들려면 `.claude/settings.json`에서 `agent`를 설정합니다:

```json theme={null}
{
  "agent": "code-reviewer"
}
```

CLI 플래그가 둘 다 있으면 설정을 재정의합니다.

<h3 id="run-subagents-in-foreground-or-background">
  Subagent를 foreground 또는 background에서 실행
</h3>

Subagent는 foreground 또는 background에서 실행할 수 있습니다:

* **Foreground subagent**는 완료될 때까지 주 대화를 차단합니다. 권한 프롬프트는 발생하는 대로 사용자에게 전달됩니다.
* **Background subagent**는 계속 작업하는 동안 동시에 실행됩니다. Background subagent가 권한이 필요한 도구 호출에 도달하면 Claude Code는 프롬프트를 주 세션에 표시하고 요청하는 subagent의 이름을 지정합니다. 승인하여 subagent를 계속하거나 Esc를 눌러 subagent를 중지하지 않고 해당 도구 호출을 거부합니다.

Claude가 Agent 도구로 생성하는 각 subagent에 대해 Claude Code는 적용되는 다음 경우 중 첫 번째에서 foreground 또는 background를 선택합니다:

* 진행 중인 [agent team](/docs/ko/agent-teams#limitations) 팀원이 subagent를 생성한 경우 Claude Code는 foreground에서 실행합니다. Claude Code는 정의가 [`background: true`](#supported-frontmatter-fields)를 설정하는 팀원의 subagent를 생성하려고 할 때 오류로 거부합니다. [fork 모드](#turn-fork-mode-on-or-off)가 꺼져 있고 [background 작업을 끄지 않은](/docs/ko/env-vars) 경우 Claude Code는 팀원이 `run_in_background: true`를 설정할 때도 오류로 거부합니다.
* [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/ko/env-vars)를 `1`로 설정한 경우 Claude Code는 모든 종류의 세션에서 그리고 fork 모드가 켜져 있는지 여부와 관계없이 foreground에서 subagent를 실행합니다.
* [fork 모드](#turn-fork-mode-on-or-off)가 켜져 있는 경우 (대화형 세션에서 기본값), Claude Code는 background에서 subagent를 실행하며, fork 및 non-fork subagent 모두 실행되고 Claude는 foreground를 요청할 수 없습니다.
* fork 모드가 꺼져 있는 경우 Claude는 기본적으로 background에서 subagent를 실행하고 계속하기 전에 결과가 필요할 때 foreground에서 실행합니다. Fork 모드는 `-p`를 사용한 [non-interactive 모드](/docs/ko/headless)에서 그리고 켜지 않으면 Agent SDK에서 꺼져 있습니다. 특정 subagent를 Claude가 결과를 원할 때도 background에 유지하려면 frontmatter [`background`](#supported-frontmatter-fields) 필드를 `true`로 설정합니다.

`context: fork`를 가진 skill의 경우 Claude Code는 fork 모드가 켜져 있는지 여부와 관계없이 [subagent에서 skill 실행](/docs/ko/skills#run-skills-in-a-subagent)의 규칙을 따릅니다.

Background subagent는 대화 fork를 제외하고 foreground subagent보다 [더 작은 내장 도구 세트](#available-tools)로 실행되며, [재개된](#resume-subagents) foreground subagent를 제외합니다.

Background subagent는 주 세션에서 모든 권한 프롬프트를 표시합니다. 해당 프롬프트 중 하나에 세션의 나머지 기간 동안 지속되는 선택 (예: 세션의 나머지 기간 동안 지속되는 부여)으로 답변하면 Claude Code는 주 대화를 포함한 전체 세션에 답변을 적용합니다.

Background subagent는 background [Bash 또는 PowerShell 명령](/docs/ko/tools-reference#background-commands) [실행을 해당 턴의 끝을 지나 계속 실행](/docs/ko/interactive-mode#how-backgrounding-works)할 수 있습니다. 해당 명령이 끝나면 Claude Code는 subagent에 알림을 보냅니다.

Background subagent의 결과는 나중의 턴에서 Claude에 완료 알림으로 도달합니다. Claude는 subagent의 결과를 보고하기 전에 해당 알림을 기다리며, 먼저 진행 상황을 묻는 경우 subagent가 여전히 실행 중임을 보고합니다. v2.1.211 이전에는 Claude가 때때로 완료되지 않은 background subagent의 결과를 보고했습니다.

다음을 수행할 수도 있습니다:

* fork 모드가 꺼져 있는 경우 Claude에 작업을 background 또는 foreground에서 실행하도록 요청
* **Ctrl+B**를 눌러 실행 중인 작업을 background로 이동

Claude Code는 subagent가 어떻게 종료되었는지에 따라 두 가지 방식으로 프롬프트 입력 아래의 subagent 패널에서 background subagent의 행을 지웁니다:

* Subagent가 성공적으로 완료되면 Claude Code는 해당 행을 즉시 제거하고 [화면 읽기 모드](/docs/ko/accessibility)를 제외하고 30초 동안 바닥글에 `/tasks to see subagents`를 표시합니다. 이 30초 동안 [`/tasks`](/docs/ko/commands)를 실행하고 subagent에서 `Enter`를 눌러 해당 트랜스크립트를 엽니다. v2.1.232 이전에는 Claude Code가 실패한 행과 동일하게 완료 후 30초 동안 행을 유지했으며 바닥글 힌트를 표시하지 않았습니다.
* Subagent가 실패하거나 중지하면 Claude Code는 30초 동안 행을 유지합니다. 행을 더 빨리 지우려면 행을 선택하고 `x`를 누릅니다.

완료된 background subagent는 [`/tasks`](/docs/ko/commands)에 나열된 상태로 유지되며, 완료로 표시되고 실행 중인 작업 아래로 정렬되며, 바닥글 힌트와 동일한 30초 동안 유지됩니다. 세부 정보 보기는 subagent가 완료될 때 열린 상태로 유지됩니다. 실패하거나 중지한 subagent는 목록을 떠납니다. v2.1.208 이전에는 완료된 subagent가 완료되는 순간 목록을 떠났고 세부 정보 보기가 닫혔습니다.

<h3 id="subagent-names">
  Subagent 이름
</h3>

Claude는 Agent 도구 호출에서 `name` 매개변수를 전달하여 subagent에 이름을 지정할 수 있으며, 먼저 사용자에게 묻지 않고 자체적으로 이름을 지정할 수 있습니다. 이름은 subagent를 주소 지정 가능하게 만듭니다: Claude는 완료 후 [이름으로 메시지를 보내거나 재개](#resume-subagents)할 수 있습니다.

[agent teams](/docs/ko/agent-teams)가 활성화된 대화형 세션에서 주 대화에서 `name`으로 Claude가 생성하는 subagent는 호출이 [fork](#fork-the-current-conversation)이거나 호출 자체에서 `isolation`을 전달하는 경우를 제외하고 팀원으로 시작됩니다. Subagent의 frontmatter의 `isolation` 값은 이를 방지하지 않으며 팀원은 주 세션의 작업 디렉토리에서 실행됩니다. [Claude가 agent team을 시작하는 방법](/docs/ko/agent-teams#how-claude-starts-agent-teams)을 참조합니다.

<h3 id="api-errors-in-subagents">
  Subagent의 API 오류
</h3>

[subagent의 응답을 스트림 중간에 중단](/docs/ko/errors#the-response-above-may-be-incomplete)하는 것이 있고 부분 응답에 텍스트가 포함되지만 도구 호출이 없으면 Claude Code는 실행을 종료하는 대신 subagent에 계속하도록 프롬프트합니다. 이는 대화형 세션에서도 발생합니다. 실행은 해당 연속이 사용될 때까지만 오류에서 종료됩니다.

v2.1.199부터 API 오류 (예: 사용 제한 또는 반복된 서버 오류)로 인해 실행이 종료된 subagent는 오류 텍스트를 subagent의 결과인 것처럼 반환하는 대신 해당 실패를 Claude에 보고합니다. Claude가 받는 내용은 subagent가 실행된 위치에 따라 다릅니다:

* **Foreground**: 속도 제한, 과부하 또는 서버 오류가 이미 텍스트 출력을 생성한 subagent를 중단하면 Agent 도구는 해당 부분 출력을 subagent가 중단되었으며 작업을 완료하지 못했다는 메모와 함께 반환합니다. 아무것도 생성하지 않았거나 유일한 출력이 도구 호출이었던 subagent는 [`Agent terminated early due to an API error`](/docs/ko/errors#agent-terminated-early-due-to-an-api-error)로 실패하고 오류 세부 정보가 뒤따릅니다. v2.1.199에서는 도구 호출만 있는 형태를 중단한 속도 제한, 과부하 또는 서버 오류가 중단 메모만 포함하는 빈 부분 결과를 반환했습니다.
* **Background**: subagent는 실패로 표시되며 Claude가 종료될 때 받는 메시지는 API 오류의 이름을 지정하고 subagent의 마지막 출력을 포함하므로 부분 작업이 손실되지 않습니다.

[fallback 모델 체인](/docs/ko/model-config#fallback-model-chains)을 구성하고 subagent가 체인이 다루는 실패 (예: 모델을 사용할 수 없음)를 만나면 Claude Code는 subagent를 요청을 수락하는 체인의 첫 번째 모델로 전환합니다. Subagent는 오류에서 종료되는 대신 계속 작동합니다.

기본 API 오류가 해결되면 Claude에 작업을 다시 시도하거나 [subagent를 재개](#resume-subagents)하도록 요청합니다.

<h3 id="subagent-output-scanning">
  Subagent 출력 스캔
</h3>

Claude Code는 Claude가 읽기 전에 각 subagent의 최종 보고서를 스캔합니다. Subagent는 파일, 웹 페이지 또는 명령 출력을 읽었을 수 있으며 이를 검토하지 않았으며, 해당 소스의 텍스트는 주 대화를 목표로 하는 지시를 전달할 수 있습니다. 스캔은 아무것도 제거하거나 다시 표현하지 않습니다. 보고서에서 알 수 있는 두 가지 종류의 변경을 수행합니다:

* **백슬래시 삽입**: 스캔은 `<system-reminder>` 태그 또는 `Human:` 또는 `Assistant:`로 시작하는 줄과 같은 Claude Code 자신의 출력을 모방하는 텍스트에 백슬래시를 삽입하므로 모방이 대화의 일부로 잘못 인식되는 대신 일반 텍스트로 읽힙니다.
* **마커 줄**: 스캔은 `<system-reminder>`와 같은 태그를 모방하거나 `bypassPermissions` 또는 `--dangerously-skip-permissions`와 같은 권한 설정을 언급할 때 `[harness: subagent output matched instruction-shaped pattern(s):`로 시작하는 줄을 앞에 붙입니다. 권한 설정 언급은 마커 줄을 받지만 텍스트 자체는 작성된 대로 유지됩니다.

스캔은 콘텐츠가 악의적인지 판단하지 않으며, 보고서의 지시가 할 수 있는 것을 변경하지 않습니다: 보고서가 Claude를 만드는 도구 호출은 여전히 세션의 [권한 확인](/docs/ko/permissions) 및 [샌드박싱](/docs/ko/sandboxing)을 거칩니다. [subagent가 도달할 수 있는 것을 제한](#control-subagent-capabilities)하는 것을 대체하지 않습니다.

Claude Code는 subagent 출력을 주 대화로 반환하는 보고서 아래에 헤더를 표시합니다. 헤더는 보고서 내의 지시 또는 승인 주장이 subagent의 말이며 사용자로부터 권한을 가지지 않음을 명시합니다.

[background subagent의 보고서](#run-subagents-in-foreground-or-background)는 완료 알림 내에 도달하며, 이는 사용자의 메시지가 아닌 자동화된 이벤트로 표시됩니다.

<Note>
  Subagent 출력 스캔에는 Claude Code v2.1.210 이상이 필요합니다.
</Note>

<h3 id="common-patterns">
  일반적인 패턴
</h3>

<h4 id="isolate-high-volume-operations">
  대량 작업 격리
</h4>

Subagent의 가장 효과적인 사용 중 하나는 많은 양의 출력을 생성하는 작업을 격리하는 것입니다. 테스트 실행, 문서 가져오기 또는 로그 파일 처리는 상당한 컨텍스트를 소비할 수 있습니다. 이를 subagent에 위임하면 자세한 출력이 subagent의 컨텍스트에 유지되고 관련 요약만 주 대화로 반환됩니다.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  병렬 연구 실행
</h4>

독립적인 조사의 경우 여러 subagent를 생성하여 동시에 작동하도록 합니다:

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

각 subagent는 자신의 영역을 독립적으로 탐색한 다음 Claude가 결과를 종합합니다. 이것은 연구 경로가 서로 의존하지 않을 때 가장 잘 작동합니다.

<Warning>
  Subagent가 완료되면 결과가 주 대화로 반환됩니다. 각각 자세한 결과를 반환하는 많은 subagent를 실행하면 상당한 컨텍스트를 소비할 수 있습니다.
</Warning>

지속적인 병렬 작업이 필요하거나 하나의 컨텍스트 윈도우에 맞지 않는 작업의 경우 [별도 세션](/docs/ko/agents)에서 실행하고 Claude가 [세션 간에 결과를 전달](/docs/ko/cross-session-messaging)하도록 합니다.

<h4 id="chain-subagents">
  Subagent 체인
</h4>

다단계 워크플로우의 경우 Claude에 subagent를 순차적으로 사용하도록 요청합니다. 각 subagent는 작업을 완료하고 결과를 Claude에 반환하고, Claude는 관련 컨텍스트를 다음 subagent에 전달합니다.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Subagent와 주 대화 중 선택
</h3>

**주 대화**를 사용하는 경우:

* 작업이 빈번한 왕복 또는 반복적인 개선이 필요한 경우
* 여러 단계가 상당한 컨텍스트를 공유하는 경우 (계획, 구현, 테스트)
* 빠르고 대상이 지정된 변경을 수행하는 경우
* 지연시간이 중요한 경우. [Fork](#fork-the-current-conversation)가 아닌 subagent는 새로 시작하고 컨텍스트를 수집하는 데 시간이 걸릴 수 있습니다

**Subagent**를 사용하는 경우:

* 작업이 주 컨텍스트에서 필요하지 않은 자세한 출력을 생성하는 경우
* 특정 도구 제한 또는 권한을 적용하려는 경우
* 작업이 자체 포함되어 있고 요약을 반환할 수 있는 경우

격리된 subagent 컨텍스트가 아닌 주 대화 컨텍스트에서 실행되는 재사용 가능한 프롬프트 또는 워크플로우를 원할 때 [Skills](/docs/ko/skills)를 대신 고려합니다.

대화에 이미 있는 항목에 대한 빠른 질문의 경우 subagent 대신 [`/btw`](/docs/ko/interactive-mode#side-questions-with-%2Fbtw)를 사용합니다. 전체 컨텍스트를 보지만 도구 액세스가 없으며 답변은 기록에 추가되지 않습니다.

<h3 id="let-subagents-spawn-their-own-subagents">
  Subagent가 자신의 subagent를 생성하도록 허용
</h3>

기본적으로 subagent는 주 대화 아래 최대 3개 계층까지 자신의 subagent를 생성할 수 있습니다. 깊이 제한에서 Claude Code는 [fork](#fork-the-current-conversation)를 제외한 모든 subagent에서 `Agent` 도구를 보류하므로 제한에서 subagent는 위임된 작업을 자체적으로 수행하고 하나의 요약을 반환합니다. 제한에서 fork는 상속된 도구 목록에서 `Agent`를 유지하지만 도구는 생성하는 대신 오류를 반환합니다.

중첩된 subagent는 위임된 작업이 자체적으로 병렬 하위 작업으로 분할될 때 적합합니다. 예를 들어 각 발견에 대해 검증자를 발송하는 검토자 subagent를 사용하면 중간 출력이 주 대화에 도달하지 않습니다. 최상위 subagent의 요약만 사용자에게 반환됩니다. 대화형 세션에서 background subagent를 시작한 subagent는 결과를 받기 전에 기다립니다. [Non-interactive 모드](/docs/ko/headless) 및 Agent SDK에서 시작 subagent는 기다리지 않으므로 시작 subagent가 종료된 후 완료되는 중첩된 background subagent는 주 대화에 보고합니다.

제한을 변경하려면 [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/ko/env-vars)를 주 대화 아래에서 원하는 subagent 계층 수로 설정합니다. 예를 들어 [`settings.json`](/docs/ko/settings)의 이 항목은 중첩을 2개 계층으로 제한합니다:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

이 값을 사용하면 subagent는 자신의 두 번째 계층으로 위임할 수 있으며 해당 두 번째 계층은 추가로 위임할 수 없습니다. `1`로 설정하여 중첩을 끕니다.

중첩된 subagent는 최상위 subagent와 동일한 방식으로 구성되며 동일한 [범위](#choose-the-subagent-scope)에서 해결됩니다. 한 subagent가 생성되지 않도록 하려면 (예: 읽기 전용으로 유지해야 하는 검토자) [`tools`](#available-tools) 목록에서 `Agent`를 생략하거나 `disallowedTools`에 추가합니다.

Claude Code는 중첩된 subagent를 프롬프트 입력 아래의 subagent 패널에 트리로 표시하고 패널에서 여전히 자손을 가진 각 행을 `(+N)` 개수로 표시합니다. 행을 열면 해당 subagent의 형제 및 직접 자식이 `main`으로 돌아가는 경로와 함께 표시됩니다.

<Note>
  이전 버전은 다른 기본값을 사용했습니다:

  * **v2.1.172부터 v2.1.216**: subagent는 기본적으로 중첩될 수 있으며 최대 5개 계층 깊이까지 가능했으며 제한을 변경할 수 없었습니다.
  * **v2.1.217부터 v2.1.218**: 제한이 기본값 1로 설정되어 subagent가 제한을 높이지 않으면 자신의 subagent를 생성할 수 없었습니다. v2.1.219는 기본값을 3으로 올렸습니다.
</Note>

<h3 id="concurrent-subagent-limit">
  동시 subagent 제한
</h3>

두 가지 제한이 subagent 사용을 제어하며 각각 자신의 변수를 가집니다: 이것은 너무 많은 subagent가 실행 중일 때 Claude가 더 많은 subagent를 생성하지 못하도록 하며 [깊이 제한](#let-subagents-spawn-their-own-subagents)은 subagent가 얼마나 깊게 중첩되는지를 제한합니다. 세션 동안 Claude가 생성할 수 있는 subagent의 총 수에는 제한이 없습니다.

기본적으로 세션에서 20개의 subagent가 실행 중일 때 Agent 도구로 다른 subagent를 생성하려고 하면 `Concurrent subagent limit reached`로 실패하며 오류는 Claude에 재시도하지 않도록 알립니다. 실행 중인 개수가 제한 아래로 떨어지면 생성이 다시 성공합니다. 제한을 변경하려면 [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/ko/env-vars)를 양의 정수로 설정합니다. [ultracode](/docs/ko/model-config#adjust-effort-level)가 활성화된 세션은 면제됩니다: 제한이 적용되지 않습니다. Claude Code v2.1.217 이상이 필요합니다.

제한은 Claude가 Agent 도구로 생성하는 subagent만 차단하지만 다른 실행은 동일한 슬롯을 차지합니다:

* [`/subtask`](#fork-the-current-conversation)로 시작하는 세션 내 fork는 실행 중일 때 슬롯을 차지하며 제한으로 차단되지 않습니다.
* [완료된 subagent를 재개](#resume-subagents)하면 제한을 확인하지 않고 새로운 슬롯을 차지하므로 재개는 실행 중인 개수를 제한 이상으로 밀어낼 수 있습니다.

[workflow](/docs/ko/workflows) 에이전트 및 [agent team](/docs/ko/agent-teams) 팀원과 같은 다른 기능이 실행하는 에이전트는 대신 자신의 제한을 따릅니다.

<h3 id="manage-subagent-context">
  Subagent 컨텍스트 관리
</h3>

<h4 id="what-loads-at-startup">
  시작 시 로드되는 항목
</h4>

각 subagent는 새로운 격리된 컨텍스트 윈도우로 시작합니다. 대화 기록, 이미 호출한 skills, 또는 Claude가 이미 읽은 파일을 보지 못합니다. Claude는 작업을 요약하는 위임 메시지를 작성하고 subagent는 여기서부터 작동합니다. 예외는 [fork](#fork-the-current-conversation)이며, 이는 새로 시작하는 대신 부모 대화를 상속합니다.

비fork subagent의 초기 컨텍스트에는 다음이 포함됩니다:

* **시스템 프롬프트**: 에이전트 자신의 프롬프트 및 Claude Code가 추가하는 환경 세부 정보이며, 전체 Claude Code 시스템 프롬프트는 아닙니다. 사용자 정의 subagent는 [markdown body](#write-subagent-files) 또는 `prompt` 필드에서 정의합니다. 내장 에이전트는 미리 정의된 프롬프트를 가집니다.
* **작업 메시지**: Claude가 작업을 넘길 때 작성하는 위임 프롬프트입니다.
* **CLAUDE.md 파일**: 주 대화가 로드하는 [CLAUDE.md 계층 구조](/docs/ko/memory#how-claude-md-files-load)의 모든 수준이며, `~/.claude/CLAUDE.md`, 프로젝트 규칙, `CLAUDE.local.md`, 관리되는 정책 파일 및 모든 [`AGENTS.md` 파일](/docs/ko/memory#agents-md)을 포함합니다. 내장 Explore 및 Plan 에이전트는 이를 건너뜁니다. 정의가 [`omitClaudeMd`](#supported-frontmatter-fields)를 설정하는 subagent는 관리되는 정책 파일만 로드하거나 정의가 [관리되는 설정](#choose-the-subagent-scope)에서 올 때 아무것도 로드하지 않습니다.
* **Git 상태**: subagent가 시작될 때 Claude Code가 저장소에서 읽는 스냅샷입니다. Git 저장소 외부에서 또는 스냅샷이 꺼져 있을 때 없습니다. [`includeGitInstructions`](/docs/ko/settings-reference#includegitinstructions)를 참조합니다. Explore 및 Plan은 관계없이 이를 건너뜁니다.
* **미리 로드된 skills**: 에이전트의 [`skills` 필드](#preload-skills-into-subagents)에 명명된 모든 skill의 전체 내용입니다. 내장 에이전트는 skills를 미리 로드하지 않습니다.
* **형제 명단**: `main` 및 세션의 다른 모든 명명된 에이전트를 나열하는 시스템 알림이며, 각각은 [`SendMessage`](#resume-subagents)에 대한 유효한 `to` 값입니다. Claude Code v2.1.206 이상이 필요합니다. 명단은 subagent의 도구에 `SendMessage`가 포함되고 Claude가 생성할 때 이름을 지정했거나 [agent teams](/docs/ko/agent-teams) 팀원으로 실행되는 다른 에이전트가 하나 이상 있을 때만 나타납니다. 이는 subagent가 시작될 때 촬영한 스냅샷이므로 나중에 명명된 에이전트는 나타나지 않습니다.

사용자, 프로젝트 및 로컬 CLAUDE.md 파일 없이 자신의 subagent를 시작하려면 frontmatter 또는 `--agents` JSON에서 [`omitClaudeMd: true`](#supported-frontmatter-fields)를 설정합니다.

주 대화는 여전히 이러한 subagent의 결과를 읽을 때 전체 CLAUDE.md를 가지므로 대부분의 규칙이 subagent 자체에 도달할 필요가 없습니다. 규칙이 필요한 경우 (예: "`vendor/` 디렉토리 무시"), subagent에 위임할 때 Claude에 제공하는 프롬프트에서 이를 다시 명시합니다.

git 상태를 받는 subagent를 변경할 수 없습니다. Explore 및 Plan만 이를 건너뜁니다.

일부 주 대화 상태는 비fork subagent에 도달하지 않습니다:

* **출력 스타일**: subagent는 자신의 시스템 프롬프트를 실행하므로 [출력 스타일](/docs/ko/output-styles)은 응답을 형성하지 않습니다. [fork](#fork-the-current-conversation)의 경우는 예외입니다.
* **자동 메모리**: 주 대화의 [자동 메모리](/docs/ko/memory#auto-memory)는 로드되지 않습니다. Subagent에 자신의 지속적인 메모리를 제공하려면 [`memory` 필드](#enable-persistent-memory)를 사용합니다.
* **컨텍스트 윈도우 크기**: subagent의 컨텍스트 윈도우는 부모의 컨텍스트 윈도우가 아닌 자신의 모델로 크기가 조정됩니다. 더 작은 윈도우를 가진 모델로 위임하면 해당 subagent는 더 작은 윈도우를 받습니다.

<h4 id="resume-subagents">
  Subagent 재개
</h4>

각 subagent 호출은 이전 subagent를 계속하는 대신 새로운 인스턴스를 만듭니다. 처음부터 시작하는 대신 기존 subagent의 작업을 계속하려면 Claude에 재개하도록 요청합니다.

재개된 subagent는 모든 이전 도구 호출, 결과 및 추론을 포함한 전체 대화 기록을 유지합니다. Subagent가 [자신의 background subagent를 생성](#let-subagents-spawn-their-own-subagents)한 경우 해당 기록은 실행 중일 때 전달한 결과를 포함합니다. Subagent는 새로 시작하는 대신 정확히 중단한 위치에서 계속됩니다.

* Subagent가 완료되면 Claude는 에이전트 ID를 받습니다.
* 내장 Explore 및 Plan 에이전트는 일회성이며 에이전트 ID를 반환하지 않으므로 Claude는 재개할 수 없습니다. 작업을 계속해야 할 때는 `general-purpose` 또는 사용자 정의 subagent를 사용합니다.
* Subagent가 [`maxTurns`](#supported-frontmatter-fields) 제한에서 중지되면 Claude Code는 반환된 출력을 부분으로 표시합니다. 에이전트 ID를 반환하는 subagent의 경우 Claude Code는 또한 결과에서 Claude가 중단한 위치에서 계속하도록 subagent에 메시지를 보낼 수 있음을 기록합니다.

Claude는 `SendMessage` 도구를 에이전트의 ID 또는 이름을 `to` 필드로 사용하여 재개합니다. `SendMessage`는 [agent teams](/docs/ko/agent-teams)가 활성화되어야 하는 `shutdown_request` 및 `plan_approval_response`와 같은 구조화된 팀 프로토콜 메시지를 필요로 하지 않습니다. Subagent 및 팀원 이상으로 cross-session 메시징이 활성화된 세션에서 Claude는 동일한 도구를 사용하여 [다른 Claude Code 세션](/docs/ko/cross-session-messaging)에 메시지를 보낼 수 있으며, 이 머신 또는 [그 이상](/docs/ko/cross-session-messaging#message-sessions-on-other-machines)에 있습니다.

Subagent를 재개하려면 Claude에 이전 작업을 계속하도록 요청합니다:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Claude가 완료된 subagent에 `SendMessage` 도구로 메시지를 보낼 때 새로운 `Agent` 호출 없이 background에서 자동으로 재개됩니다. `TaskStop` 도구로 Claude가 중단한 subagent도 마찬가지입니다. 중단된 실행이 종료되면 재개됩니다. 재개된 실행은 [원래 실행이 워밍한 프롬프트 캐시](/docs/ko/prompt-caching#subagents-and-the-cache)를 계속 읽을 수 있으며 subagent가 처음 실행된 위치에서 [도구 세트](#run-subagents-in-foreground-or-background)를 유지합니다.

Subagent가 `SendMessage` 도구를 가지면 해당 메시지도 보낼 수 있습니다. 대화형 세션에서 재개된 에이전트는 주 대화가 아닌 재개한 subagent에 보고합니다. 해당 subagent는 결과를 받기 전에 완료를 기다립니다. Subagent가 이를 시작한 에이전트 (예: 자신의 런처)에 메시지를 보낼 때 Claude Code는 결과를 리디렉션하지 않고 해당 에이전트를 재개합니다.

직접 중단한 subagent (예: `/tasks`에서 `x` 또는 SDK `stop_task` 요청)는 자동으로 재개되지 않습니다. Claude가 메시지를 보내면 메시지는 거부되고 Claude는 에이전트가 취소되었음을 알립니다.

[subagent 패널의 해당 행이 여전히 있는 동안](#run-subagents-in-foreground-or-background) 해당 트랜스크립트에 입력하여 직접 재개할 수 있습니다. 그 후 Claude의 메시지가 다시 자동으로 재개할 수 있습니다.

재개는 동일한 ID 아래에서 에이전트의 새로운 실행을 시작하므로 이미 실패했거나 완료된 subagent는 작업 목록 및 Agent SDK의 작업 이벤트에서 다시 실행 중으로 표시됩니다. v2.1.205 이전에는 재개된 실행이 작동하는 동안 이전의 실패했거나 완료된 상태를 계속 표시했습니다.

v2.1.199부터 `SendMessage`는 이름이 여전히 대화에서 이전에 도달한 동일한 에이전트를 참조하는지 확인합니다. 더 새로운 에이전트가 이름을 가져간 경우 (예: 이름을 재사용한 다시 생성된 background 에이전트), Claude Code는 잘못된 에이전트에 전달하는 대신 전송을 거부하며 오류는 이름이 현재 도달하는 에이전트를 보고하므로 Claude가 재대상화할 수 있습니다. 여전히 실행 중인 이전 에이전트에 도달하려면 Claude는 생성 결과의 에이전트 ID로 주소를 지정합니다. 확인은 현재 대화로 범위가 지정되며 `/clear`에서 재설정됩니다.

v2.1.198부터 subagent는 이를 시작한 에이전트의 메시지를 일반적인 작업 지시로 취급하며, 중간 작업 과정 수정을 포함하고 자신의 권한 설정 내에서 작동합니다. 메시지를 보낸 사람과 관계없이 두 가지 제한이 여전히 유지됩니다: 어떤 에이전트의 메시지도 보류 중인 권한 프롬프트에 대한 승인으로 계산되지 않으며, 어떤 에이전트 메시지도 subagent의 권한 설정, `CLAUDE.md` 또는 구성을 변경할 수 없습니다. 권한 시스템 또는 자신의 메시지만 승인을 부여할 수 있습니다.

에이전트 ID를 명시적으로 참조하려면 Claude에 ID를 요청할 수도 있으며, `~/.claude/projects/{project}/{sessionId}/subagents/`의 트랜스크립트 파일에서 ID를 찾을 수 있습니다. 각 트랜스크립트는 `agent-{agentId}.jsonl`로 저장됩니다.

Subagent 트랜스크립트는 주 대화와 독립적으로 유지됩니다:

* **주 대화 압축**: 주 대화가 압축될 때 subagent 트랜스크립트는 영향을 받지 않습니다. 별도 파일에 저장됩니다.
* **세션 지속성**: Subagent 트랜스크립트는 세션 내에서 유지됩니다. 동일한 세션을 재개하여 Claude Code를 다시 시작한 후 [subagent를 재개](#resume-subagents)할 수 있습니다.
* **자동 정리**: Claude Code는 `cleanupPeriodDays` 보존 기간 (기본값: 30일) 후 subagent 트랜스크립트를 삭제하며 [보존 스윕 규칙](/docs/ko/claude-directory#cleaned-up-automatically)을 따릅니다.

<h4 id="auto-compaction">
  자동 압축
</h4>

Subagent는 주 대화와 동일한 논리를 사용하여 자동 압축을 지원합니다. 압축은 동일한 조건에서 트리거되며, `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`는 subagent에도 적용됩니다. 재정의가 적용되는 시기는 [환경 변수](/docs/ko/env-vars)를 참조하세요.

압축 이벤트는 subagent 트랜스크립트 파일에 기록됩니다:

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

`preTokens` 값은 압축이 발생하기 전에 사용된 토큰 수를 보여줍니다.

<h2 id="fork-the-current-conversation">
  현재 대화 포크
</h2>

<Note>
  `/subtask`로 포크된 subagent를 실행합니다. 이는 Claude Code v2.1.212 이상이 필요합니다. [에이전트 보기가 꺼져 있을 때](/docs/ko/agent-view#turn-off-agent-view), `/subtask`는 사용할 수 없으며 `/fork`가 포크된 subagent를 대신 시작합니다. 그렇지 않으면 `/fork`는 전체 세션을 새로운 [background 세션](/docs/ko/agent-view#from-inside-a-session)으로 복사합니다.
</Note>

포크는 새로 시작하는 대신 지금까지의 전체 대화를 상속하는 subagent입니다. 이렇게 하면 subagent가 일반적으로 제공하는 입력 격리가 떨어집니다: 포크는 주 세션과 동일한 시스템 프롬프트, 도구, 모델 및 메시지 기록을 보므로 상황을 다시 설명할 필요 없이 부작업을 전달할 수 있습니다. 포크의 자체 도구 호출은 여전히 대화에서 벗어나고 최종 결과만 돌아오므로 주 컨텍스트 윈도우가 깨끗하게 유지됩니다. 다른 subagent가 유용하기에는 너무 많은 배경이 필요하거나 동일한 시작점에서 여러 접근 방식을 병렬로 시도하려는 경우 포크를 사용합니다.

Claude는 Agent 도구를 통해 `fork` subagent 유형을 요청하여 포크를 시작합니다. [포크 모드](#turn-fork-mode-on-or-off)로 이를 제어할 수 있으며, 이는 대화형 세션에서 기본적으로 켜져 있습니다.

`/subtask` 다음에 작업을 입력하여 포크 모드가 켜져 있는지 여부와 관계없이 직접 포크를 시작할 수 있습니다. v2.1.161부터 v2.1.211까지는 명령이 `/fork`입니다. Claude Code는 작업의 첫 단어에서 포크의 이름을 지정합니다. 다음 예제는 주 세션에서 구현을 계속하는 동안 포크가 테스트 케이스를 작성하도록 포크합니다:

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

포크는 프롬프트 입력 아래의 패널에 나타나고 계속 작업하는 동안 background에서 실행됩니다. 완료되면 결과가 주 대화의 메시지로 도착합니다. 다음 섹션에서는 포크가 실행되는 동안 포크를 관찰하고 조종하기 위한 패널 컨트롤을 다룹니다.

<h3 id="observe-and-steer-running-forks">
  실행 중인 포크 관찰 및 조종
</h3>

실행 중인 포크는 프롬프트 입력 아래의 패널에 나타나며, 주 세션에 대한 행과 각 포크에 대한 행이 있습니다.

포크가 성공적으로 완료되면 Claude Code는 해당 행을 제거합니다. Claude Code는 실패했거나 중지한 포크의 행을 30초 동안 유지합니다. [다른 background subagent와 동일합니다](#run-subagents-in-foreground-or-background). v2.1.232 이전에는 Claude Code가 완료된 포크의 행을 30초 동안 유지했습니다.

이 키를 사용하여 패널과 상호 작용합니다:

| 키         | 작업                                                                                                       |
| :-------- | :------------------------------------------------------------------------------------------------------- |
| `↑` / `↓` | 행 간 이동                                                                                                   |
| `Enter`   | 선택한 포크의 트랜스크립트를 열고 후속 메시지 전송                                                                             |
| `x`       | 실행 중인 경우 선택한 포크를 중지하거나, 더 이상 실행 중이 아닌 경우 행을 닫습니다. 주 세션 행 또는 `Enter`로 트랜스크립트를 연 포크의 행에서는 `x`가 프롬프트에 입력됩니다 |
| `Esc`     | 프롬프트 입력으로 포커스 반환                                                                                         |

포크 또는 subagent의 트랜스크립트가 열려 있으면 후속 메시지 및 [skills](/docs/ko/skills)는 해당 에이전트로 이동하지만 기본 제공 명령은 여전히 주 대화에서 실행됩니다. v2.1.199부터 해당 보기에서 `/model` 또는 `/fast`를 입력하면 보기된 에이전트의 모델이나 빠른 모드가 아닌 주 대화의 모델이나 빠른 모드를 변경한다는 알림이 표시되며, 자동으로 실행되지 않습니다.

<h3 id="how-forks-differ-from-other-subagents">
  포크와 다른 subagent의 차이점
</h3>

포크는 생성 시점의 주 세션의 모든 것을 상속합니다. 다른 subagent는 정의에서 새로 시작합니다.

|               | 포크             | 비포크 subagent                                                                           |
| :------------ | :------------- | :------------------------------------------------------------------------------------- |
| 컨텍스트          | 전체 대화 기록       | 전달하는 프롬프트를 사용한 새로운 컨텍스트                                                                |
| 시스템 프롬프트 및 도구 | 주 세션과 동일       | Subagent의 [정의 파일](#write-subagent-files)에서, [background 실행을 위해 필터링됨](#available-tools) |
| 모델            | 주 세션과 동일       | Subagent의 `model` 필드에서                                                                 |
| 권한            | 프롬프트가 터미널에 표시됨 | [Background에서 실행 중일 때 프롬프트가 주 세션에 표시됨](#run-subagents-in-foreground-or-background)     |
| 프롬프트 캐시       | 주 세션과 공유       | 별도 캐시                                                                                  |

포크의 시스템 프롬프트 및 도구 정의가 부모와 동일하기 때문에 첫 번째 요청은 부모의 [프롬프트 캐시](/docs/ko/prompt-caching#subagents-and-the-cache)를 재사용합니다. 이렇게 하면 동일한 컨텍스트가 필요한 작업에 대해 새로운 subagent를 생성하는 것보다 포크가 더 저렴합니다.

Claude가 Agent 도구를 통해 포크를 생성할 때 `isolation: "worktree"`를 전달하여 포크의 파일 편집이 체크아웃 대신 별도의 git worktree에 기록되도록 할 수 있습니다. 포크는 추가 포크를 생성할 수 없습니다.

<h3 id="turn-fork-mode-on-or-off">
  포크 모드 켜기 또는 끄기
</h3>

Claude Code는 대화형 세션에서 포크 모드를 기본적으로 켜고 `-p`를 사용한 [비대화형 모드](/docs/ko/headless) 및 Agent SDK에서는 기본적으로 끕니다. 대화형 기본값은 Claude Code v2.1.232 이상이 필요합니다. 이전 버전에서는 `CLAUDE_CODE_FORK_SUBAGENT`를 `1`로 설정하여 포크 모드를 켭니다.

Claude Code가 Agent 도구를 처리하는 방식으로 포크 모드가 켜져 있는지 알 수 있습니다:

* Claude는 `fork` subagent 유형을 요청하여 포크를 생성할 수 있습니다. Claude가 유형을 요청하지 않으면 세션에 여전히 해당 유형이 있는 경우 [general-purpose](#built-in-subagents) subagent를 가져옵니다. Explore와 같은 정의에서 생성된 Subagent는 평소대로 작동합니다.
* Claude Code는 Claude가 생성하는 subagent를 background에서 실행합니다. 포크와 비포크 subagent 모두, [foreground에 남아 있는 경우](#run-subagents-in-foreground-or-background)를 제외하고. Claude Code는 또한 Agent 도구의 `run_in_background` 매개변수를 제거하므로 Claude는 foreground를 요청할 수 없습니다.

[`CLAUDE_CODE_FORK_SUBAGENT`](/docs/ko/env-vars) 환경 변수를 설정하여 기본값을 재정의합니다:

* `1`은 비대화형 모드 및 Agent SDK에서도 포크 모드를 켭니다
* `0`은 모든 종류의 세션에서 포크 모드를 끕니다

포크 모드는 켜두되 Claude가 포크를 생성하지 못하도록 하려면 `Agent(fork)` 규칙으로 [`fork` subagent 유형을 거부합니다](#disable-specific-subagents). Claude Code는 여전히 Claude가 생성하는 subagent를 background에서 실행합니다. [foreground에 남아 있는 경우](#run-subagents-in-foreground-or-background)를 제외하고.

<h2 id="example-subagents">
  예제 subagent
</h2>

이러한 예제는 subagent를 구축하기 위한 효과적인 패턴을 보여줍니다. 시작점으로 사용하거나 Claude로 사용자 정의된 버전을 생성합니다.

<Tip>
  **모범 사례:**

  * **집중된 subagent 설계:** 각 subagent는 특정 작업에서 탁월해야 합니다
  * **각 subagent를 구분하는 설명 작성:** Claude는 설명을 사용하여 위임할 시기를 결정합니다. 각 설명이 올바른 subagent로 라우팅할 수 있을 정도로 구체적이어야 하며, 결합된 집합을 [15,000토큰 설명 예산](#understand-automatic-delegation) 내에 유지합니다
  * **도구 액세스 제한:** 보안 및 집중을 위해 필요한 권한만 부여합니다
  * **버전 제어에 체크인:** 프로젝트 subagent를 팀과 공유합니다
</Tip>

<h3 id="code-reviewer">
  코드 검토자
</h3>

수정하지 않고 코드를 검토하는 읽기 전용 subagent입니다. 이 예제는 제한된 도구 액세스(Edit 및 Write 제외)와 정확히 무엇을 찾을지 및 출력 형식을 지정하는 자세한 프롬프트를 사용하여 집중된 subagent를 설계하는 방법을 보여줍니다.

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  디버거
</h3>

문제를 분석하고 수정할 수 있는 subagent입니다. 코드 검토자와 달리 이 subagent는 버그 수정이 코드 수정을 필요로 하기 때문에 Edit을 포함합니다. 프롬프트는 진단에서 검증까지의 명확한 워크플로우를 제공합니다.

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  데이터 과학자
</h3>

데이터 분석 작업을 위한 도메인별 subagent입니다. 이 예제는 일반적인 코딩 작업 외에 특화된 워크플로우를 위해 subagent를 만드는 방법을 보여줍니다. 더 유능한 분석을 위해 명시적으로 `model: sonnet`을 설정합니다.

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  데이터베이스 쿼리 검증자
</h3>

Bash 액세스를 허용하지만 읽기 전용 SQL 쿼리만 허용하도록 명령을 검증하는 subagent입니다. 이 예제는 `tools` 필드보다 더 세밀한 제어가 필요할 때 `PreToolUse` hook을 사용하는 방법을 보여줍니다.

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code는 [hook 입력을 JSON으로](/docs/ko/hooks#pretooluse-input) stdin을 통해 hook 명령에 전달합니다. 검증 스크립트는 이 JSON을 읽고 실행 중인 명령을 추출하고 SQL 쓰기 작업 목록에 대해 확인합니다. 쓰기 작업이 감지되면 스크립트는 [종료 코드 2](/docs/ko/hooks#exit-code-2-behavior-per-event)로 종료하여 실행을 차단하고 stderr를 통해 Claude에 오류 메시지를 반환합니다.

프로젝트의 어디든지 검증 스크립트를 만듭니다. 경로는 hook 구성의 `command` 필드와 일치해야 합니다:

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

macOS 및 Linux에서 스크립트를 실행 가능하게 만듭니다:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Windows에서는 검증 스크립트를 PowerShell로 작성하고 hook 항목에 `shell: powershell`을 추가합니다. [PowerShell에서 hook 실행](/docs/ko/hooks#windows-powershell-tool)을 참조하세요.

Hook은 stdin을 통해 JSON을 받으며 Bash 명령은 `tool_input.command`에 있습니다. 종료 코드 2는 작업을 차단하고 오류 메시지를 Claude에 피드백합니다. 종료 코드 및 출력에 대한 자세한 내용은 [Hooks](/docs/ko/hooks#exit-code-output)를 참조하고 [Hook input](/docs/ko/hooks#pretooluse-input)에서 전체 입력 스키마를 확인하세요.

시스템 프롬프트는 subagent에 쓰기 요청을 거부하도록 지시하므로 hook은 백스톱입니다. subagent가 어쨌든 쓰기를 시도하면 Claude Code는 명령을 차단하고 subagent는 `Blocked: Write operations not allowed. Use SELECT queries only.` 메시지를 봅니다.

<h2 id="next-steps">
  다음 단계
</h2>

이제 subagent를 이해했으므로 다음 관련 기능을 탐색합니다:

* [플러그인으로 subagent 배포](/docs/ko/plugins/components#agents) - 팀 또는 프로젝트 간에 subagent 공유
* [Claude Code를 프로그래밍 방식으로 실행](/docs/ko/headless) - CI/CD 및 자동화를 위한 Agent SDK
* [MCP 서버 사용](/docs/ko/mcp) - Subagent에 외부 도구 및 데이터에 대한 액세스 제공
