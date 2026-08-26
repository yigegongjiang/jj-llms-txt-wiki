> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code의 작동 방식

> 에이전트 루프, 내장 도구, Claude Code가 프로젝트와 상호작용하는 방식을 이해합니다.

Claude Code는 터미널에서 실행되는 에이전트 어시스턴트입니다. 코딩에 탁월하지만 명령줄에서 할 수 있는 모든 작업을 도와줄 수 있습니다: 문서 작성, 빌드 실행, 파일 검색, 주제 조사 등.

이 가이드는 핵심 아키텍처, 내장 기능, 그리고 [Claude Code를 효과적으로 사용하기 위한 팁](#work-effectively-with-claude-code)을 다룹니다. 단계별 설명서는 [일반적인 워크플로우](/docs/ko/common-workflows)를 참조하세요. skills, MCP, hooks와 같은 확장 기능은 [Claude Code 확장](/docs/ko/features-overview)을 참조하세요.

<h2 id="the-agentic-loop">
  에이전트 루프
</h2>

Claude에게 작업을 주면 세 가지 단계를 거칩니다: **컨텍스트 수집**, **작업 수행**, **결과 검증**. 이 단계들은 함께 진행됩니다. Claude는 파일을 검색하여 코드를 이해하든, 변경을 위해 편집하든, 작업을 확인하기 위해 테스트를 실행하든 전체적으로 도구를 사용합니다.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agentic-loop.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=4a30fb7ce2815012a9f27c955e2c6bb0" alt="에이전트 루프 다이어그램: 프롬프트가 Claude가 컨텍스트를 수집하고, 작업을 수행하고, 결과를 검증하고, 작업이 완료될 때까지 반복하도록 합니다. 언제든지 중단할 수 있습니다." width="720" height="280" data-path="images/agentic-loop.svg" />

루프는 사용자가 요청한 내용에 맞게 조정됩니다. 코드베이스에 대한 질문은 컨텍스트 수집만 필요할 수 있습니다. 버그 수정은 세 단계를 반복적으로 거칩니다. 리팩토링은 광범위한 검증을 포함할 수 있습니다. Claude는 이전 단계에서 배운 내용을 바탕으로 각 단계에서 필요한 것을 결정하고, 수십 개의 작업을 연결하며 그 과정에서 방향을 수정합니다.

사용자도 이 루프의 일부입니다. 언제든지 중단하여 Claude를 다른 방향으로 유도하거나, 추가 컨텍스트를 제공하거나, 다른 접근 방식을 시도하도록 요청할 수 있습니다. Claude는 자율적으로 작동하지만 사용자의 입력에 반응합니다.

에이전트 루프는 두 가지 구성 요소로 구동됩니다: 추론하는 [모델](#models)과 작용하는 [도구](#tools). Claude Code는 Claude 주변의 **에이전트 하네스** 역할을 합니다: 언어 모델을 능력 있는 코딩 에이전트로 변환하는 도구, 컨텍스트 관리, 실행 환경을 제공합니다.

<h3 id="models">
  모델
</h3>

Claude Code는 Claude 모델을 사용하여 코드를 이해하고 작업에 대해 추론합니다. Claude는 모든 언어의 코드를 읽을 수 있고, 구성 요소가 어떻게 연결되는지 이해하며, 목표를 달성하기 위해 무엇을 변경해야 하는지 파악할 수 있습니다. 복잡한 작업의 경우 작업을 단계로 나누고, 실행하고, 배운 내용을 바탕으로 조정합니다.

[여러 모델](/docs/ko/model-config)을 사용할 수 있으며 각각 다른 장단점이 있습니다. Sonnet은 대부분의 코딩 작업을 잘 처리합니다. Opus는 복잡한 아키텍처 결정을 위한 더 강력한 추론을 제공합니다. 세션 중에 `/model`로 전환하거나 `claude --model <name>`으로 시작하세요.

이 가이드에서 "Claude가 선택한다" 또는 "Claude가 결정한다"고 할 때, 모델이 추론을 수행하는 것입니다.

<h3 id="tools">
  도구
</h3>

도구는 Claude Code를 에이전트로 만드는 것입니다. 도구가 없으면 Claude는 텍스트로만 응답할 수 있습니다. 도구가 있으면 Claude는 작용할 수 있습니다: 코드를 읽고, 파일을 편집하고, 명령을 실행하고, 웹을 검색하고, 외부 서비스와 상호작용합니다. 각 도구 사용은 루프에 다시 피드백되는 정보를 반환하여 Claude의 다음 결정을 알립니다.

내장 도구는 일반적으로 다섯 가지 범주로 나뉘며, 각각은 다른 종류의 에이전시를 나타냅니다.

| 범주           | Claude가 할 수 있는 것                                                                               |
| ------------ | ---------------------------------------------------------------------------------------------- |
| **파일 작업**    | 파일 읽기, 코드 편집, 새 파일 생성, 이름 변경 및 재구성                                                             |
| **검색**       | 패턴으로 파일 찾기, 정규식으로 콘텐츠 검색, 코드베이스 탐색                                                             |
| **실행**       | 셸 명령 실행, 서버 시작, 테스트 실행, git 사용                                                                 |
| **웹**        | 웹 검색, 문서 가져오기, 오류 메시지 조회                                                                       |
| **코드 인텔리전스** | 편집 후 타입 오류 및 경고 확인, 정의로 이동, 참조 찾기 ([코드 인텔리전스 플러그인](/docs/ko/discover-plugins#code-intelligence) 필요) |

이것이 주요 기능입니다. Claude는 또한 subagents를 생성하고, 질문을 하고, 다른 오케스트레이션 작업을 위한 도구를 가지고 있습니다. 전체 목록은 [Claude가 사용할 수 있는 도구](/docs/ko/tools-reference)를 참조하세요.

Claude는 프롬프트와 그 과정에서 배운 내용을 바탕으로 사용할 도구를 선택합니다. "실패한 테스트를 수정해"라고 말하면 Claude는 다음을 수행할 수 있습니다:

1. 테스트 스위트를 실행하여 무엇이 실패하는지 확인
2. 오류 출력 읽기
3. 관련 소스 파일 검색
4. 해당 파일을 읽어 코드 이해
5. 파일을 편집하여 문제 수정
6. 테스트를 다시 실행하여 검증

각 도구 사용은 Claude에게 다음 단계를 알리는 새로운 정보를 제공합니다. 이것이 에이전트 루프의 작동입니다.

**기본 기능 확장:** 내장 도구는 기초입니다. [skills](/docs/ko/skills)로 Claude가 알 수 있는 것을 확장하고, [MCP](/docs/ko/mcp)로 외부 서비스에 연결하고, [hooks](/docs/ko/hooks)로 워크플로우를 자동화하고, [subagents](/docs/ko/sub-agents)로 작업을 위임할 수 있습니다. 이러한 확장은 핵심 에이전트 루프 위에 계층을 형성합니다. 필요에 맞는 확장을 선택하는 방법은 [Claude Code 확장](/docs/ko/features-overview)을 참조하세요.

<h2 id="what-claude-can-access">
  Claude가 접근할 수 있는 것
</h2>

이 가이드는 터미널에 중점을 둡니다. Claude Code는 또한 [VS Code](/docs/ko/vs-code), [JetBrains IDE](/docs/ko/jetbrains), 및 기타 환경에서 실행됩니다.

디렉토리에서 `claude`를 실행하면 Claude Code는 다음에 접근할 수 있습니다:

* **프로젝트.** 디렉토리 및 하위 디렉토리의 파일, 그리고 허가를 받은 다른 곳의 파일.
* **터미널.** 실행할 수 있는 모든 명령: 빌드 도구, git, 패키지 관리자, 시스템 유틸리티, 스크립트. 명령줄에서 할 수 있는 것이면 Claude도 할 수 있습니다.
* **git 상태.** 현재 브랜치, 커밋되지 않은 변경 사항, 최근 커밋 기록.
* **[CLAUDE.md](/docs/ko/memory).** 프로젝트별 지침, 규칙, Claude가 매 세션마다 알아야 할 컨텍스트를 저장하는 마크다운 파일.
* **[자동 메모리](/docs/ko/memory#auto-memory).** Claude가 작업하면서 자동으로 저장하는 학습 내용(프로젝트 패턴 및 사용자 선호도 등). MEMORY.md의 처음 200줄 또는 25KB 중 먼저 도달하는 것이 각 세션 시작 시 로드됩니다.
* **구성한 확장.** 외부 서비스를 위한 [MCP servers](/docs/ko/mcp), 워크플로우를 위한 [skills](/docs/ko/skills), 위임된 작업을 위한 [subagents](/docs/ko/sub-agents), 브라우저 상호작용을 위한 [Claude in Chrome](/docs/ko/chrome).

Claude가 전체 프로젝트를 보기 때문에 전체 프로젝트에서 작업할 수 있습니다. "인증 버그를 수정해"라고 Claude에게 요청하면 관련 파일을 검색하고, 컨텍스트를 이해하기 위해 여러 파일을 읽고, 여러 파일에 걸쳐 조정된 편집을 수행하고, 수정을 검증하기 위해 테스트를 실행하고, 요청하면 변경 사항을 커밋합니다. 이는 현재 파일만 보는 인라인 코드 어시스턴트와 다릅니다.

<h2 id="environments-and-interfaces">
  환경 및 인터페이스
</h2>

위에서 설명한 에이전트 루프, 도구, 기능은 Claude Code를 사용하는 모든 곳에서 동일합니다. 변하는 것은 코드가 실행되는 위치와 상호작용하는 방식입니다.

<h3 id="execution-environments">
  실행 환경
</h3>

Claude Code는 세 가지 환경에서 실행되며, 각각은 코드가 실행되는 위치에 대해 다른 장단점이 있습니다.

| 환경        | 코드 실행 위치          | 사용 사례                      |
| --------- | ----------------- | -------------------------- |
| **로컬**    | 사용자 머신            | 기본값. 파일, 도구, 환경에 대한 전체 접근  |
| **클라우드**  | Anthropic 관리 VM   | 작업 오프로드, 로컬에 없는 리포지토리에서 작업 |
| **원격 제어** | 사용자 머신, 브라우저에서 제어 | 웹 UI를 사용하면서 모든 것을 로컬로 유지   |

<h3 id="interfaces">
  인터페이스
</h3>

터미널, [데스크톱 앱](/docs/ko/desktop), [IDE 확장](/docs/ko/vs-code), [claude.ai/code](https://claude.ai/code), [원격 제어](/docs/ko/remote-control), [Slack](/docs/ko/slack), [CI/CD 파이프라인](/docs/ko/github-actions)을 통해 Claude Code에 접근할 수 있습니다. 인터페이스는 Claude를 보고 상호작용하는 방식을 결정하지만, 기본 에이전트 루프는 동일합니다. 전체 목록은 [Claude Code를 어디서나 사용](/docs/ko/overview#use-claude-code-everywhere)을 참조하세요.

<h2 id="work-with-sessions">
  세션으로 작업
</h2>

Claude Code는 작업하면서 대화를 로컬에 저장합니다. 각 메시지, 도구 사용, 결과가 `~/.claude/projects/` 아래의 일반 텍스트 JSONL 파일에 기록되어 [되돌리기](#undo-changes-with-checkpoints), [재개 및 포크](#resume-or-fork-sessions) 세션을 활성화합니다. Claude가 코드를 변경하기 전에 영향을 받는 파일을 스냅샷하므로 필요하면 되돌릴 수 있습니다. 경로, 보존, 이 데이터를 지우는 방법은 [`~/.claude`의 애플리케이션 데이터](/docs/ko/claude-directory#application-data)를 참조하세요.

**세션은 독립적입니다.** 각 새 세션은 이전 세션의 대화 기록 없이 새로운 컨텍스트 윈도우로 시작합니다. Claude는 [자동 메모리](/docs/ko/memory#auto-memory)를 사용하여 세션 간에 학습을 유지할 수 있으며, [CLAUDE.md](/docs/ko/memory)에 자신의 지속적인 지침을 추가할 수 있습니다.

<h3 id="work-across-branches">
  브랜치 간 작업
</h3>

각 Claude Code 대화는 현재 디렉토리에 연결된 세션입니다. `/resume` 선택기는 기본적으로 현재 worktree의 세션을 표시하며, 다른 worktrees 또는 프로젝트로 목록을 확장하는 키보드 단축키가 있습니다. 전체 선택기 단축키 목록과 이름 해석 방식은 [세션 관리](/docs/ko/sessions#use-the-session-picker)를 참조하세요.

Claude는 현재 브랜치의 파일을 봅니다. 브랜치를 전환하면 Claude는 새 브랜치의 파일을 보지만 대화 기록은 동일하게 유지됩니다. Claude는 전환 후에도 논의한 내용을 기억합니다.

세션이 디렉토리에 연결되어 있으므로 [git worktrees](/docs/ko/worktrees)를 사용하여 병렬 Claude 세션을 실행할 수 있으며, 이는 개별 브랜치에 대한 별도 디렉토리를 생성합니다.

<h3 id="resume-or-fork-sessions">
  세션 재개 또는 포크
</h3>

`claude --continue` 또는 `claude --resume`으로 세션을 재개하면 동일한 세션 ID를 사용하여 중단한 지점부터 시작합니다. 새 메시지는 기존 대화에 추가됩니다. `--fork-session` 또는 `/branch`로 포크하면 기록을 새 세션 ID로 복사하여 원본은 변경되지 않은 상태로 유지합니다.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/session-continuity.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=04ed0984a58e4127e05b3640265241a3" alt="세션 연속성: 재개는 동일한 세션을 계속하고, 포크는 새 ID로 새 브랜치를 생성합니다." width="560" height="280" data-path="images/session-continuity.svg" />

재개 플래그, `/resume` 선택기, 이름 지정, 동일한 세션이 두 터미널에서 열려 있을 때 발생하는 상황은 [세션 관리](/docs/ko/sessions)를 참조하세요.

<h3 id="the-context-window">
  컨텍스트 윈도우
</h3>

Claude의 컨텍스트 윈도우는 대화 기록, 파일 콘텐츠, 명령 출력, [CLAUDE.md](/docs/ko/memory), [자동 메모리](/docs/ko/memory#auto-memory), 로드된 skills, 시스템 지침을 보유합니다. 작업하면서 컨텍스트가 채워집니다. Claude는 자동으로 압축하지만 대화 초반의 지침이 손실될 수 있습니다. 지속적인 규칙을 CLAUDE.md에 넣고 `/context`를 실행하여 공간을 사용하는 것을 확인하세요.

컨텍스트 윈도우의 대화형 설명은 [컨텍스트 윈도우 탐색](/docs/ko/context-window)을 참조하세요.

<h4 id="when-context-fills-up">
  컨텍스트가 채워질 때
</h4>

Claude Code는 한계에 접근할 때 컨텍스트를 자동으로 관리합니다. 먼저 이전 도구 출력을 지우고, 필요하면 대화를 요약합니다. 요청과 주요 코드 스니펫은 유지되지만 대화 초반의 자세한 지침이 손실될 수 있습니다. 대화 기록에 의존하기보다는 지속적인 규칙을 CLAUDE.md에 넣으세요.

압축 중에 보존되는 것을 제어하려면 CLAUDE.md에 "Compact Instructions" 섹션을 추가하거나 `/compact`를 포커스와 함께 실행하세요 (예: `/compact focus on the API changes`).

단일 파일 또는 도구 출력이 너무 커서 각 요약 후 컨텍스트가 즉시 다시 채워지면 Claude Code는 몇 번의 시도 후 자동 압축을 중지하고 루핑 대신 오류를 표시합니다. 복구 단계는 [자동 압축이 thrashing 오류로 중지됨](/docs/ko/troubleshooting#auto-compaction-stops-with-a-thrashing-error)을 참조하세요.

`/context`를 실행하여 공간을 사용하는 것을 확인하세요. MCP 도구 정의는 기본적으로 지연되며 [도구 검색](/docs/ko/mcp#scale-with-mcp-tool-search)을 통해 요청 시 로드되므로 Claude가 특정 도구를 사용할 때까지 도구 이름만 컨텍스트를 소비합니다. `/mcp`를 실행하여 서버별 비용을 확인하세요.

<h4 id="manage-context-with-skills-and-subagents">
  skills 및 subagents로 컨텍스트 관리
</h4>

압축 외에도 다른 기능을 사용하여 컨텍스트에 로드되는 것을 제어할 수 있습니다.

[Skills](/docs/ko/skills)는 요청 시 로드됩니다. Claude는 세션 시작 시 skill 설명을 보지만 전체 콘텐츠는 skill이 사용될 때만 로드됩니다. 수동으로 호출하는 skills의 경우 `disable-model-invocation: true`를 설정하여 필요할 때까지 설명을 컨텍스트 밖으로 유지하세요. 작성하지 않은 skills의 경우 [`skillOverrides`](/docs/ko/skills#override-skill-visibility-from-settings)를 사용하여 설정에서 동일하게 수행하세요.

[Subagents](/docs/ko/sub-agents)는 주 대화와 완전히 분리된 자신의 새로운 컨텍스트를 얻습니다. 그들의 작업은 컨텍스트를 부풀리지 않습니다. 완료되면 요약을 반환합니다. 이 격리가 긴 세션에서 subagents가 도움이 되는 이유입니다.

각 기능의 비용은 [컨텍스트 비용](/docs/ko/features-overview#understand-context-costs)을 참조하고, 컨텍스트 관리 팁은 [토큰 사용 감소](/docs/ko/costs#reduce-token-usage)를 참조하세요.

<h2 id="stay-safe-with-checkpoints-and-permissions">
  체크포인트 및 권한으로 안전 유지
</h2>

Claude는 두 가지 안전 메커니즘을 가지고 있습니다: 체크포인트는 파일 변경을 취소할 수 있게 하고, 권한은 Claude가 요청 없이 할 수 있는 것을 제어합니다.

<h3 id="undo-changes-with-checkpoints">
  체크포인트로 변경 취소
</h3>

**모든 파일 편집은 되돌릴 수 있습니다.** Claude가 파일을 편집하기 전에 현재 콘텐츠를 스냅샷합니다. 문제가 발생하면 `Esc`를 두 번 눌러 이전 상태로 되돌리거나 Claude에게 취소하도록 요청하세요.

체크포인트는 git과 분리되어 있으며 대화를 재개할 때 계속 사용할 수 있습니다. 파일 변경만 다룹니다. 원격 시스템(데이터베이스, API, 배포)에 영향을 주는 작업은 체크포인트할 수 없으므로 Claude는 외부 부작용이 있는 명령을 실행하기 전에 요청합니다.

<h3 id="control-what-claude-can-do">
  Claude가 할 수 있는 것 제어
</h3>

`Shift+Tab`을 눌러 권한 모드를 순환하세요:

* **기본값**: Claude는 파일 편집 및 셸 명령 전에 요청합니다
* **자동 수락 편집**: Claude는 파일을 편집하고 `mkdir` 및 `mv`와 같은 일반적인 파일시스템 명령을 요청 없이 실행하지만 다른 명령은 여전히 요청합니다
* **Plan**: Claude는 소스 파일을 편집하지 않고 계획을 탐색하고 제안합니다
* **Auto**: Claude는 백그라운드 안전 검사로 모든 작업을 평가합니다

`.claude/settings.json`에서 특정 명령을 허용하여 Claude가 매번 요청하지 않도록 할 수 있습니다. 이는 `npm test` 또는 `git status`와 같은 신뢰할 수 있는 명령에 유용합니다. 설정은 조직 전체 정책에서 개인 선호도까지 범위를 지정할 수 있습니다. 자세한 내용은 [권한](/docs/ko/permissions)을 참조하세요.

***

<h2 id="work-effectively-with-claude-code">
  Claude Code를 효과적으로 사용하기
</h2>

이 팁들은 Claude Code에서 더 나은 결과를 얻는 데 도움이 됩니다.

<h3 id="ask-claude-code-for-help">
  Claude Code에 도움을 요청하기
</h3>

Claude Code는 사용 방법을 가르칠 수 있습니다. "hooks를 설정하려면 어떻게 하나요?" 또는 "CLAUDE.md를 구조화하는 최선의 방법은 무엇인가요?"와 같은 질문을 하면 Claude가 설명합니다.

내장 명령도 설정을 안내합니다:

* `/init`은 프로젝트를 위한 CLAUDE.md 생성을 안내합니다
* `/doctor`는 설치 및 구성 문제를 진단하고 수정할 수 있습니다

<h3 id="it’s-a-conversation">
  대화형입니다
</h3>

Claude Code는 대화형입니다. 완벽한 프롬프트가 필요하지 않습니다. 원하는 것으로 시작한 다음 개선하세요:

```text theme={null}
로그인 버그 수정
```

\[Claude가 조사하고 시도]

```text theme={null}
정확하지 않습니다. 문제는 세션 처리에 있습니다.
```

\[Claude가 접근 방식 조정]

첫 번째 시도가 맞지 않으면 다시 시작할 필요가 없습니다. 반복합니다.

<h4 id="interrupt-and-steer">
  중단 및 조종
</h4>

언제든지 Claude를 리다이렉트할 수 있습니다. 턴이 완료될 때까지 기다리거나 다시 시작할 필요 없이:

* **`Esc` 키를 누르세요** Claude를 즉시 중지합니다. 실행 중인 도구 호출이 취소되고 Claude는 다음 지시를 기다립니다.
* **수정 사항을 입력하고 `Enter`를 누르세요** 실행 중인 도구를 중지하지 않고 전송합니다. Claude는 현재 작업이 완료되는 즉시 이를 읽고 다음 단계를 결정하기 전에 조정합니다.

<h3 id="be-specific-upfront">
  처음부터 구체적으로
</h3>

초기 프롬프트가 정확할수록 필요한 수정이 적습니다. 특정 파일을 참조하고, 제약 조건을 언급하고, 예제 패턴을 지적하세요.

```text theme={null}
체크아웃 흐름이 만료된 카드를 가진 사용자에게 손상되었습니다.
문제를 찾기 위해 src/payments/를 확인하세요. 특히 토큰 새로고침.
먼저 실패하는 테스트를 작성한 다음 수정하세요.
```

모호한 프롬프트는 작동하지만 더 많은 시간을 조종하는 데 소비합니다. 위와 같은 구체적인 프롬프트는 종종 첫 번째 시도에서 성공합니다.

<h3 id="give-claude-something-to-verify-against">
  Claude가 검증할 수 있는 것을 제공하기
</h3>

Claude는 자신의 작업을 확인할 수 있을 때 더 잘 수행합니다. 테스트 케이스를 포함하고, 예상 UI의 스크린샷을 붙여넣거나, 원하는 출력을 정의하세요.

```text theme={null}
validateEmail을 구현하세요. 테스트 케이스: 'user@example.com' → true,
'invalid' → false, 'user@.com' → false. 후에 테스트를 실행하세요.
```

시각적 작업의 경우 디자인의 스크린샷을 붙여넣고 Claude에게 구현을 비교하도록 요청하세요.

<h3 id="explore-before-implementing">
  구현 전에 탐색하기
</h3>

복잡한 문제의 경우 연구와 코딩을 분리하세요. 계획 모드(`Shift+Tab` 두 번)를 사용하여 먼저 코드베이스를 분석하세요:

```text theme={null}
src/auth/를 읽고 세션을 처리하는 방법을 이해하세요.
그런 다음 OAuth 지원 추가를 위한 계획을 생성하세요.
```

계획을 검토하고 대화를 통해 개선한 다음 Claude가 구현하도록 하세요. 이 2단계 접근 방식은 코드로 바로 뛰어드는 것보다 더 나은 결과를 생성합니다.

<h3 id="delegate-don’t-dictate">
  지시하지 말고 위임하기
</h3>

능력 있는 동료에게 위임하는 것처럼 생각하세요. 컨텍스트와 방향을 제공한 다음 Claude가 세부 사항을 파악하도록 신뢰하세요:

```text theme={null}
체크아웃 흐름이 만료된 카드를 가진 사용자에게 손상되었습니다.
관련 코드는 src/payments/에 있습니다. 조사하고 수정할 수 있나요?
```

읽을 파일이나 실행할 명령을 지정할 필요가 없습니다. Claude가 파악합니다.

<h2 id="what’s-next">
  다음 단계
</h2>

<CardGroup cols={2}>
  <Card title="기능으로 확장" icon="puzzle-piece" href="/docs/ko/features-overview">
    Skills, MCP 연결, 사용자 정의 명령 추가
  </Card>

  <Card title="일반적인 워크플로우" icon="graduation-cap" href="/docs/ko/common-workflows">
    일반적인 작업을 위한 단계별 가이드
  </Card>
</CardGroup>
