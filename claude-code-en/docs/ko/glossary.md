> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 용어집

> Claude Code 용어 정의. 에이전트 루프, 컴팩션, CLAUDE.md, 훅, 서브에이전트, MCP 및 기타 핵심 개념의 의미를 알아봅니다.

이 용어집은 Claude Code 용어를 정의합니다. 각 항목은 개념이 심층적으로 다루어지는 페이지로 연결됩니다. 토큰, 온도, RAG와 같은 모델 수준의 개념은 [플랫폼 용어집](https://platform.claude.com/docs/ko/about-claude/glossary)을 참조하십시오.

<h2 id="a">
  A
</h2>

<h3 id="agent-teams">
  Agent teams
</h3>

여러 개의 독립적인 Claude Code 세션이 팀 리더에 의해 조정되며, 공유 작업 목록과 피어 투 피어 메시징을 갖춘 구성입니다. 단일 세션 내에서 실행되고 부모에게만 보고하는 [서브에이전트](#subagent)와 달리, 팀원들은 각각 자신의 컨텍스트 윈도우를 가지며 어느 팀원과도 직접 상호작용할 수 있습니다. Agent teams는 실험적이며 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`을 설정하여 활성화해야 합니다.

자세히 알아보기: [에이전트 팀 실행](/docs/ko/agent-teams)

<h3 id="agentic-coding">
  Agentic coding
</h3>

AI가 파일을 읽고, 명령을 실행하고, 변경 사항을 자율적으로 만들 수 있는 워크플로우입니다. 사용자가 직접 적용해야 하는 텍스트만 응답하는 채팅 기반 어시스턴트와 달리, 사용자가 지켜보거나, 리디렉션하거나, 떠날 수 있습니다. Claude Code는 조언만 하는 것이 아니라 행동할 수 있게 해주는 [도구](#tool)를 가지고 있기 때문에 에이전트입니다.

자세히 알아보기: [Claude Code 작동 방식](/docs/ko/how-claude-code-works)

<h3 id="agentic-harness">
  Agentic harness
</h3>

언어 모델을 능력 있는 코딩 에이전트로 변환하는 도구, 컨텍스트 관리 및 실행 환경입니다. Claude Code는 하네스이고, Claude는 그 안의 모델입니다. 하네스는 파일 액세스, 셸 실행, 권한 게이팅, 메모리 로딩 및 작업을 함께 연결하는 루프를 제공합니다.

자세히 알아보기: [Claude Code 작동 방식](/docs/ko/how-claude-code-works)

<h3 id="agentic-loop">
  Agentic loop
</h3>

Claude가 모든 작업을 수행하는 사이클입니다: 컨텍스트 수집, 작업 수행, 결과 확인 및 완료될 때까지 반복합니다. 각 도구 사용은 다음 단계를 알려주는 정보를 반환합니다. 언제든지 루프를 중단하여 리디렉션할 수 있습니다. [훅](#hook), [스킬](#skill), [MCP](#mcp-model-context-protocol)을 포함한 대부분의 확장 포인트는 이 루프의 특정 단계에 연결됩니다.

자세히 알아보기: [Claude Code 작동 방식](/docs/ko/how-claude-code-works#the-agentic-loop)

<h3 id="artifact">
  Artifact
</h3>

Claude Code가 세션에서 claude.ai의 비공개 URL로 게시하는 라이브 대화형 웹 페이지이므로, 터미널 텍스트를 읽는 대신 시각적으로 출력을 보거나 공유할 수 있습니다. 세션이 다시 게시될 때 페이지가 제자리에서 업데이트됩니다. Claude Code에서 만든 Artifact는 claude.ai 대화에서 만든 Artifact와 동일한 갤러리에 나타납니다. 공유는 플랜에 따라 달라집니다: Pro 및 Max에서는 누구나 열 수 있는 공개 링크이고, Team 및 Enterprise에서는 조직 내 공유이며, 소유자가 활성화하면 공개 링크도 가능합니다.

자세히 알아보기: [세션 출력을 Artifact로 공유](/docs/ko/artifacts)

<h3 id="auto-memory">
  Auto memory
</h3>

Claude가 사용자의 수정 및 선호도를 기반으로 자신을 위해 작성한 노트이며, `~/.claude/projects/` 아래 git 저장소별로 저장됩니다. 동일한 저장소의 모든 worktree는 하나의 auto memory 디렉토리를 공유합니다. `MEMORY.md` 인덱스의 처음 200줄 또는 25KB가 모든 세션의 시작 시 로드됩니다. Auto memory는 사용자가 작성하는 [CLAUDE.md](#claude-md)의 Claude 작성 대응물입니다.

자세히 알아보기: [Auto memory](/docs/ko/memory#auto-memory)

<h3 id="auto-mode">
  Auto mode
</h3>

백그라운드에서 별도의 분류기 모델이 각 작업을 검토하는 [권한 모드](#permission-mode)이며, 대부분은 승인 프롬프트 없이 실행됩니다. 명시적 요청 규칙은 여전히 프롬프트를 표시합니다. 분류기는 범위 확대, 신뢰할 수 없는 인프라 및 [프롬프트 주입](#prompt-injection)을 차단합니다. 도구 결과를 보지 않으므로 주입된 지침이 결정에 영향을 미칠 수 없습니다.

자세히 알아보기: [프롬프트 제거 및 auto mode](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare mode
</h3>

자동 발견을 건너뛰는 시작 플래그 `--bare`입니다. 훅, 스킬, 플러그인, MCP 서버, auto memory 및 CLAUDE.md의 자동 발견을 건너뜁니다. 명시적으로 전달하는 플래그만 적용됩니다. CI 및 스크립트된 호출에 권장되며, 로컬 구성에 관계없이 머신 간에 동일한 동작이 필요합니다.

자세히 알아보기: [bare mode로 더 빠르게 시작](/docs/ko/headless#start-faster-with-bare-mode)

<h3 id="bundled-skills">
  Bundled skills
</h3>

Claude Code에 포함된 프롬프트 기반 플레이북입니다. `/batch`, `/code-review`, `/debug`, `/loop` 등이 있습니다. 고정 로직을 실행하는 기본 제공 명령과 달리, 번들 스킬은 Claude에 상세한 프롬프트를 제공하고 작업을 조율하도록 하므로 에이전트를 생성하고, 파일을 읽고, 코드베이스에 적응할 수 있습니다.

자세히 알아보기: [번들 스킬](/docs/ko/skills#bundled-skills)

<h2 id="c">
  C
</h2>

<h3 id="channel">
  Channel
</h3>

실행 중인 세션에 이벤트를 푸시하는 [MCP 서버](#mcp-model-context-protocol)이므로 Claude는 터미널에서 떨어져 있을 때 발생하는 일에 반응할 수 있습니다. 채널은 양방향일 수 있습니다: Claude는 인바운드 이벤트를 읽고 동일한 채널을 통해 다시 응답합니다. Telegram, Discord 및 iMessage는 연구 미리보기에 포함됩니다.

자세히 알아보기: [채널](/docs/ko/channels)

<h3 id="checkpoint">
  Checkpoint
</h3>

각 프롬프트를 전송할 때마다 생성되는 복원 지점입니다. Claude Code는 모든 편집 전에 파일을 스냅샷하므로 체크포인트가 파일을 되돌릴 수 있습니다. `Esc`를 두 번 누르거나 `/rewind`를 실행하여 코드, 대화 또는 둘 다를 이전 지점으로 복원하거나, 선택한 메시지에서 대화의 일부를 요약합니다. 체크포인트는 세션에 저장되므로 재개된 세션도 여전히 `/rewind`를 사용하여 이들로 돌아갈 수 있습니다. 이들은 git과 별개이며 Bash 도구를 통해 수행된 변경 사항을 추적하지 않습니다.

자세히 알아보기: [체크포인팅](/docs/ko/checkpointing)

<h3 id="claude-directory">
  `.claude` directory
</h3>

Claude Code가 프로젝트 범위 구성을 읽는 디렉토리입니다: 설정, 훅, 스킬, 서브에이전트, 규칙 및 auto memory. 프로젝트는 루트에 `.claude/`를 가지며, 사용자 수준 기본값은 `~/.claude/`에 있습니다.

자세히 알아보기: [`.claude` 디렉토리](/docs/ko/claude-directory)

<h3 id="claude-md">
  CLAUDE.md
</h3>

Claude를 위해 작성하는 지속적인 지침의 마크다운 파일이며, 시스템 프롬프트 이후 사용자 메시지로 모든 세션의 시작 시 로드됩니다. 프로젝트 규칙, 아키텍처 노트 및 "항상 X를 수행" 규칙을 여기에 넣습니다. 프로젝트 루트 CLAUDE.md는 [컴팩션](#compaction)을 견디고 이후 디스크에서 새로 다시 읽습니다.

CLAUDE.md를 프로젝트 범위에서 `./CLAUDE.md` 또는 `./.claude/CLAUDE.md`에, 사용자 범위에서 `~/.claude/CLAUDE.md`에, 또는 조직의 [관리 정책](#managed-settings)으로 배치할 수 있습니다. 발견된 모든 파일은 서로를 재정의하지 않고 연결되며, 가장 광범위한 범위에서 가장 구체적인 범위로 정렬됩니다.

자세히 알아보기: [CLAUDE.md 파일](/docs/ko/memory#claude-md-files)

<h3 id="command">
  Command
</h3>

프롬프트에 `/name`을 입력하여 호출하는 재사용 가능한 지침입니다. `/clear`, `/model`, `/compact`와 같은 기본 제공 명령은 세션을 제어합니다. `.claude/commands/`의 파일로 자신의 명령을 정의하거나 [플러그인](#plugin)에서 설치할 수 있습니다. [스킬](#skill)은 다단계 명령을 패키징하는 권장 방법입니다.

자세히 알아보기: [명령](/docs/ko/commands) · [스킬](/docs/ko/skills)

<h3 id="compaction">
  Compaction
</h3>

[컨텍스트 윈도우](#context-window)가 한계에 접근할 때 대화의 자동 요약입니다. 이전 도구 출력이 먼저 지워지고, 그 다음 대화가 요약됩니다. 프로젝트 루트 CLAUDE.md 및 auto memory는 컴팩션을 견디고 디스크에서 다시 로드됩니다. 대화에서만 제공된 지침은 손실될 수 있습니다. `/compact`를 수동으로 트리거하거나, `/compact focus on the API changes`와 같은 포커스를 선택적으로 사용합니다.

자세히 알아보기: [컴팩션에서 생존하는 것](/docs/ko/context-window#what-survives-compaction) · [컨텍스트가 가득 찰 때](/docs/ko/how-claude-code-works#when-context-fills-up)

<h3 id="context-window">
  Context window
</h3>

세션의 작업 메모리이며, 대화 기록, 파일 내용, 명령 출력, CLAUDE.md, auto memory, 로드된 스킬 및 시스템 지침을 보유합니다. 작업하면서 컨텍스트가 가득 찰 때까지 채워지고 [컴팩션](#compaction)이 요약합니다. `/context`를 실행하여 공간을 사용하는 것을 확인합니다. 기본 모델 개념은 [플랫폼 용어집](https://platform.claude.com/docs/ko/about-claude/glossary#context-window)을 참조하십시오.

자세히 알아보기: [컨텍스트 윈도우 탐색](/docs/ko/context-window)

<h2 id="d">
  D
</h2>

<h3 id="dispatch">
  Dispatch
</h3>

Claude 모바일 앱에서 코딩 작업을 보낼 때 Desktop 앱에서 Claude Code 세션을 생성하는 휴대폰 시작 작업 라우터입니다. 프롬프트가 올바른 도구로 자동으로 라우팅됩니다. Pro 및 Max 플랜에서 사용 가능합니다.

자세히 알아보기: [Dispatch의 세션](/docs/ko/desktop#sessions-from-dispatch)

<h2 id="e">
  E
</h2>

<h3 id="effort-level">
  Effort level
</h3>

각 턴에서 Claude가 적응형 추론 사고 예산을 얼마나 사용할지 제어하는 설정입니다. 더 높은 노력은 더 많은 사고 토큰과 더 깊은 추론을 의미합니다. 더 낮은 노력은 더 빠르고 저렴합니다. Effort는 Fable 5, Opus 4.6 이상 및 Sonnet 4.6 이상에서 지원됩니다.

자세히 알아보기: [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended thinking
</h3>

모델이 응답하기 전에 수행하는 가시적인 단계별 추론입니다. [노력 수준](#effort-level)으로 조정하거나 고정된 사고 예산이 있는 모델에서 `MAX_THINKING_TOKENS`로 사고 토큰을 제한할 수 있습니다. 사고는 터미널에서 회색 이탤릭 텍스트로 나타납니다.

자세히 알아보기: [확장 사고 사용](/docs/ko/model-config#extended-thinking)

<h2 id="h">
  H
</h2>

<h3 id="hook">
  Hook
</h3>

Claude Code의 라이프사이클의 특정 지점에서 자동으로 실행되는 사용자 정의 핸들러입니다. 도구 실행 전, 파일 편집 후 또는 세션 시작 시 등입니다. 핸들러는 셸 명령, HTTP 엔드포인트, MCP 도구, LLM 프롬프트 또는 서브에이전트일 수 있습니다. 훅은 결정론적입니다: 모델의 재량이 아니라 고정된 라이프사이클 포인트에서 발생합니다.

훅 구성에는 세 가지 수준이 있습니다:

* **Hook event**: 라이프사이클 포인트
* **Matcher**: 어떤 이벤트가 발생하는지 필터링합니다
* **Hook handler**: 실행되는 것

자세히 알아보기: [훅 시작하기](/docs/ko/hooks-guide) · [훅 참조](/docs/ko/hooks)

<h2 id="m">
  M
</h2>

<h3 id="managed-settings">
  Managed settings
</h3>

IT 또는 DevOps에 의해 조직 전체에 적용되는 설정이며, Anthropic의 서버를 통해 관리 콘솔에서 전달되거나 `~/.claude` 외부의 OS 수준 경로에 배치됩니다. 사용자 및 프로젝트 설정은 관리 설정을 재정의할 수 없습니다. 서버 관리 전달은 [적격 구성](/docs/ko/server-managed-settings#platform-availability)에 적용됩니다. [보안 고려 사항](/docs/ko/server-managed-settings#security-considerations)을 참조하십시오. 보안 정책, 규정 준수 요구 사항 또는 플릿 전체의 표준화된 도구에 사용합니다.

자세히 알아보기: [Server-managed settings](/docs/ko/server-managed-settings) · [Settings files](/docs/ko/settings#settings-files)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

AI 도구를 외부 데이터 소스 및 서비스에 연결하기 위한 개방형 표준입니다. MCP 서버는 Claude에 Slack, Jira, 데이터베이스, 브라우저 및 수백 개의 다른 통합을 위한 새로운 도구를 제공합니다. `/mcp`를 통해 또는 `.mcp.json`에 추가하여 서버를 연결합니다. 프로토콜 자체는 [플랫폼 용어집](https://platform.claude.com/docs/ko/about-claude/glossary#mcp-model-context-protocol)을 참조하십시오.

자세히 알아보기: [Model Context Protocol](/docs/ko/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

필요할 때까지 MCP 도구 스키마를 연기하는 컨텍스트 절약 메커니즘입니다. 시작 시 도구 이름만 로드됩니다. Claude는 특정 도구를 사용하기로 결정할 때 전체 스키마를 요청합니다. 이렇게 하면 유휴 MCP 서버가 많은 컨텍스트를 소비하지 않습니다.

자세히 알아보기: [MCP Tool Search로 확장](/docs/ko/mcp#scale-with-mcp-tool-search)

<h2 id="n">
  N
</h2>

<h3 id="non-interactive-mode">
  Non-interactive mode
</h3>

`-p` 또는 `--print`로 호출되는 단일 프롬프트를 실행하고 대화형 프롬프트 없이 종료하는 모드입니다. CI, 스크립트 및 파이핑에 사용됩니다. `--no-session-persistence`를 전달하지 않는 한 실행은 여전히 재개 가능한 세션으로 저장됩니다. [Agent SDK](/docs/ko/agent-sdk/overview)는 Python 및 TypeScript 동등물입니다. 이전에는 headless mode라고 불렸습니다.

자세히 알아보기: [Claude Code를 프로그래밍 방식으로 실행](/docs/ko/headless)

<h2 id="o">
  O
</h2>

<h3 id="output-style">
  Output style
</h3>

Claude의 시스템 프롬프트를 수정하여 응답 동작, 톤 또는 형식을 변경하는 구성입니다. 출력 스타일은 사용자 메시지로 전달되는 [CLAUDE.md](#claude-md)와 달리 기본 시스템 프롬프트의 소프트웨어 엔지니어링 관련 부분을 끕니다. 기본 제공 스타일에는 Default, Proactive, Explanatory 및 Learning이 포함됩니다.

자세히 알아보기: [출력 스타일](/docs/ko/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Permission mode
</h3>

세션의 기본 승인 동작입니다. CLI에서 `Shift+Tab`으로 순환하거나 VS Code, Desktop 및 claude.ai의 모드 선택기를 사용합니다. 사용 가능한 모드는 `default`, `acceptEdits`, `plan`, `auto`, `dontAsk` 및 `bypassPermissions`입니다.

`default` 모드는 CLI 및 VS Code와 JetBrains 확장에서 Manual로 표시되며, Claude Code는 값에 대한 `manual`의 별칭을 허용합니다.

자세히 알아보기: [권한 모드 선택](/docs/ko/permission-modes)

<h3 id="permission-rule">
  Permission rule
</h3>

도구 이름 및 인수 패턴을 기반으로 도구 호출을 허용, 질문 또는 거부하는 설정 항목입니다. 규칙은 deny→ask→allow로 평가되며, 첫 번째 일치가 우선합니다. 권한 규칙은 더 광범위한 [권한 모드](#permission-mode) 위에 계층화된 세밀한 제어입니다.

자세히 알아보기: [권한 구성](/docs/ko/permissions)

<h3 id="plan-mode">
  Plan mode
</h3>

Claude가 소스 파일을 편집하지 않고 변경 사항을 연구하고 제안하는 [권한 모드](#permission-mode)입니다. 읽고, 검색하고, 탐색 명령을 실행할 수 있으며, 아무것도 건드리기 전에 승인을 위한 계획을 제시합니다. `/plan`을 입력하거나 `Shift+Tab`을 눌러 plan mode에 들어갑니다.

자세히 알아보기: [plan mode로 편집 전에 분석](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

스킬, 훅, 서브에이전트 및 MCP 서버의 번들이며, 단일 설치 가능한 단위로 패키징됩니다. 플러그인 스킬은 `plugin-name:skill-name`으로 네임스페이스되므로 여러 플러그인이 공존합니다. [마켓플레이스](/docs/ko/plugin-marketplaces)를 통해 팀 전체에 플러그인을 배포합니다.

자세히 알아보기: [플러그인](/docs/ko/plugins)

<h3 id="project-trust">
  Project trust
</h3>

Claude Code가 구성을 로드하기 전에 디렉토리를 수락하는 대화입니다. 수락은 프로젝트 디렉토리별로 저장되며, 홈 디렉토리는 제외되고, 여기서 신뢰는 현재 세션에만 유지되며 각 실행 시 프롬프트가 다시 나타납니다. 신뢰는 마켓플레이스 플러그인의 자동 설치 및 프로젝트 정의 훅의 실행을 게이팅합니다. 디렉토리를 신뢰하면 `.claude/settings.json`, `.mcp.json` 및 기타 구성 파일이 적용됩니다.

자세히 알아보기: [`.claude` 디렉토리](/docs/ko/claude-directory)

<h3 id="prompt-injection">
  Prompt injection
</h3>

파일, 웹 페이지 또는 도구 결과에 포함된 적대적 지침이며, Claude를 요청하지 않은 작업으로 리디렉션하려고 시도합니다. Claude Code의 방어에는 권한 시스템, 명령 주입 탐지 및 신뢰 확인이 포함됩니다. [Auto mode](#auto-mode)는 도구 결과에서 의심스러운 내용을 스캔하는 서버 측 프로브와 도구 결과를 보지 않는 분류기를 추가하므로 주입된 텍스트가 승인 결정에 영향을 미칠 수 없습니다.

자세히 알아보기: [프롬프트 주입으로부터 보호](/docs/ko/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

claude.ai를 통해 휴대폰 또는 브라우저에서 로컬 Claude Code 세션을 계속하는 방법입니다. 코드 실행 및 파일은 머신에 남아 있습니다. 인터페이스만 원격입니다. 클라우드 샌드박스에서 실행되는 웹의 Claude Code와 다릅니다.

자세히 알아보기: [Remote Control](/docs/ko/remote-control)

<h3 id="rules">
  Rules
</h3>

CLAUDE.md와 함께 로드되는 `.claude/rules/`의 모듈식 지침 파일입니다. 규칙은 YAML `paths:` frontmatter로 경로 범위를 지정할 수 있으므로 Claude가 일치하는 파일을 읽을 때만 로드되어 관련될 때까지 컨텍스트를 유지합니다.

자세히 알아보기: [`.claude/rules/`로 규칙 구성](/docs/ko/memory#organize-rules-with-claude/rules/)

<h2 id="s">
  S
</h2>

<h3 id="sandboxing">
  Sandboxing
</h3>

Bash 도구에 대한 OS 수준 파일 시스템 및 네트워크 격리입니다. 명령은 미리 정의한 경계 내에서 실행되므로 Claude는 명령별 승인 프롬프트 없이 자유롭게 작업할 수 있습니다. 샌드박싱은 [권한 규칙](#permission-rule)과 별개의 계층입니다.

자세히 알아보기: [샌드박싱](/docs/ko/sandboxing)

<h3 id="session">
  Session
</h3>

현재 디렉토리에 연결된 대화이며, 자신의 독립적인 [컨텍스트 윈도우](#context-window)를 가집니다. 세션은 `claude -c`로 재개할 수 있고, `--fork-session`으로 포크하여 새 세션 ID 아래에 기록을 보존하거나, 터미널 전체에서 병렬로 실행할 수 있습니다. `/clear`를 실행하면 새 세션이 시작됩니다. 이전 세션은 저장된 상태로 유지되며 `/resume`을 통해 사용 가능합니다. 각 세션의 기록은 `~/.claude/projects/` 아래에 저장됩니다.

자세히 알아보기: [세션으로 작업](/docs/ko/how-claude-code-works#work-with-sessions)

<h3 id="settings-layers">
  Settings layers
</h3>

Claude Code가 구성을 읽는 계층 구조이며, 우선 순위 순서는 높음에서 낮음입니다: [관리 정책](#managed-settings), 명령줄 인수, `.claude/settings.local.json`의 로컬 설정, `.claude/settings.json`의 프로젝트 설정, 그 다음 `~/.claude/settings.json`의 사용자 설정. 배열은 계층 전체에서 병합됩니다. 스칼라는 더 높은 계층에서 더 낮은 계층을 재정의합니다.

자세히 알아보기: [설정 파일](/docs/ko/settings#settings-files)

<h3 id="skill">
  Skill
</h3>

지침, 지식 또는 Claude가 도구 키트에 추가하는 워크플로우를 포함하는 `SKILL.md` 파일입니다. Claude는 관련이 있을 때 스킬을 자동으로 로드하거나 `/skill-name`으로 직접 호출합니다. 스킬은 Agent Skills 개방형 표준을 따릅니다. Claude Code는 호출 제어 및 서브에이전트 실행으로 확장합니다.

스킬은 사용자 정의 명령의 권장 후속입니다. `.claude/commands/deploy.md`의 파일과 `.claude/skills/deploy/SKILL.md`의 파일은 모두 `/deploy`를 생성하고 동일하게 작동합니다. 기존 명령 파일은 계속 작동합니다.

자세히 알아보기: [스킬로 Claude 확장](/docs/ko/skills)

<h3 id="subagent">
  Subagent
</h3>

자신의 컨텍스트 윈도우, 사용자 정의 시스템 프롬프트, 특정 도구 액세스 및 독립적인 권한으로 실행되는 특화된 AI 어시스턴트입니다. 위임된 작업을 수행하고 주 대화에 요약을 반환합니다. 서브에이전트를 사용하여 큰 탐색을 기본 컨텍스트 밖으로 유지하거나 병렬 연구를 실행합니다. 각 에이전트가 직접 대화할 수 있는 완전한 독립적인 세션인 [agent teams](#agent-teams)와 다릅니다.

기본 제공 서브에이전트에는 Explore, Plan 및 범용이 포함됩니다.

자세히 알아보기: [사용자 정의 서브에이전트 생성](/docs/ko/sub-agents)

<h3 id="surface">
  Surface
</h3>

Claude Code에 액세스하는 모든 장소입니다: CLI, VS Code, JetBrains, Desktop 또는 claude.ai. 모든 표면은 동일한 엔진을 공유하므로 CLAUDE.md, 설정 및 스킬이 모든 표면에서 동일하게 작동합니다. Slack 및 Chrome 확장 프로그램은 표면 자체가 아니라 표면에 연결하는 통합입니다.

자세히 알아보기: [플랫폼 및 통합](/docs/ko/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

클라우드 Claude Code 세션을 로컬 터미널로 가져오는 명령 `/teleport`입니다. Claude는 분기를 가져오고, 대화 기록을 로드하고, 웹 세션의 마지막 상태에서 재개합니다. 역방향은 `--cloud`이며, 로컬 작업을 웹에서 실행하도록 보냅니다.

자세히 알아보기: [웹에서 터미널로](/docs/ko/claude-code-on-the-web#from-web-to-terminal)

<h3 id="tool">
  Tool
</h3>

Claude가 수행할 수 있는 작업입니다: 파일 읽기, 코드 편집, 셸 명령 실행, 웹 검색, 서브에이전트 생성. 도구는 Claude Code를 에이전트로 만드는 것입니다. 도구 없이 Claude는 텍스트로만 응답할 수 있습니다. 각 도구 사용은 [에이전트 루프](#agentic-loop)에서 Claude의 다음 결정을 알려주는 결과를 반환합니다.

자세히 알아보기: [Claude에서 사용 가능한 도구](/docs/ko/tools-reference)

<h3 id="turn">
  Turn
</h3>

[세션](#session) 내에서 Claude의 한 번의 완전한 응답입니다. 턴은 메시지를 보낼 때 시작되고 Claude가 응답을 마칠 때 끝나며, 그 사이에 여러 개의 [도구](#tool) 호출이 있을 수 있습니다. [Stop 훅](#hook)은 각 턴의 끝에서 실행됩니다. 세션은 많은 턴으로 구성되며, [에이전트 루프](#agentic-loop)는 한 턴 내에서 일어나는 일을 설명합니다.

자세히 알아보기: [Claude Code 작동 방식](/docs/ko/how-claude-code-works#the-agentic-loop)

<h2 id="v">
  V
</h2>

<h3 id="verification-loop">
  Verification loop
</h3>

세션이 작업이 실제로 완료되었는지 아니면 그럴듯한지만 알 수 있는 방법입니다. 테스트 스위트, 빌드 또는 스크린샷 비교와 같이 Claude가 실행할 수 있는 검사를 제공하고, Claude는 한 번의 시도 후 중단하는 대신 검사가 통과할 때까지 반복합니다. 검증 루프는 [`/goal`](/docs/ko/goal), 무인 실행 및 [동적 워크플로우](/docs/ko/workflows)의 전제 조건입니다: 검증 루프 없이는 에이전트가 완료되었다고 결정하는 유일한 것은 에이전트 자체입니다.

자세히 알아보기: [Claude에 작업 확인 방법 제공](/docs/ko/best-practices#give-claude-a-way-to-verify-its-work)

<h2 id="w">
  W
</h2>

<h3 id="worktree-isolation">
  Worktree isolation
</h3>

`.claude/worktrees/` 아래의 별도 git worktree에서 Claude를 실행하는 격리 모드이며, `-w` 플래그 또는 서브에이전트 구성의 `isolation: worktree`로 활성화됩니다. 변경 사항은 별도 디렉토리의 별도 분기에 남아 있으므로 병렬 에이전트가 서로의 파일을 덮어쓰지 않습니다.

자세히 알아보기: [git worktrees로 병렬 세션 실행](/docs/ko/worktrees)

***

<h2 id="deprecated-and-renamed-terms">
  더 이상 사용되지 않는 용어 및 이름이 변경된 용어
</h2>

이 용어는 이전 문서, 블로그 게시물 및 커뮤니티 콘텐츠에 나타납니다. 이 사이트를 검색할 때 현재 이름을 사용합니다.

| 이전 용어           | 현재 이름                                         | 참고                               |
| --------------- | --------------------------------------------- | -------------------------------- |
| Headless mode   | [Non-interactive mode](#non-interactive-mode) | 동일한 `-p` 플래그, 동일한 동작             |
| Custom commands | [Skills](#skill)                              | `.claude/commands/` 파일은 계속 작동합니다 |
| Slash commands  | Commands                                      | 제품 복사본에서 "Slash" 제거됨             |
