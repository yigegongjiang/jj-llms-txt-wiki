> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 새로운 기능

> Claude Code의 주목할 만한 기능을 매주 정리한 다이제스트로, 코드 스니펫, 데모, 그리고 그 중요성에 대한 설명을 포함합니다.

주간 개발자 다이제스트는 업무 방식을 바꿀 가능성이 가장 높은 기능들을 강조합니다. 각 항목에는 실행 가능한 코드, 짧은 데모, 그리고 전체 문서로의 링크가 포함됩니다. 모든 버그 수정 및 사소한 개선 사항은 [changelog](/docs/ko/changelog)를 참조하십시오.

<Update label="Week 28" description="2026년 7월 6–10일" tags={["v2.1.202–v2.1.206"]}>
  **Desktop의 인앱 브라우저**: Desktop의 Claude Code는 기본 제공 브라우저를 받아서 Claude가 문서, 디자인 또는 다른 사이트를 불러올 수 있으며 로컬 개발 서버 미리보기와 동일한 방식으로 페이지와 상호작용할 수 있습니다.

  이번 주의 다른 기능들: \*\*`/doctor`\*\*는 문제를 진단하고 수정할 수 있는 완전한 설정 점검이며, `/checkup`이 별칭입니다. **auto mode**는 트랜스크립트 변조를 차단하고 미해결 변수에서 `rm -rf` 전에 묻습니다. 그리고 **agent view rows**는 색상이 지정된 상태 단어와 분류기가 작성한 헤드라인을 표시합니다.

  [Week 28 다이제스트 읽기 →](/docs/ko/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="2026년 6월 29일 – 7월 3일" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**: Pro, Team Standard, 그리고 Enterprise 구독 시트의 새로운 기본 모델로, Sonnet 가격대의 최고 수준 코딩 및 도구 사용, 기본 1M 토큰 컨텍스트 윈도우, 그리고 기본적으로 활성화된 적응형 사고를 제공합니다.

  이번 주의 다른 기능들: **Chrome의 Claude**는 모든 직접 Anthropic 플랜에서 일반적으로 사용 가능합니다. **subagents는 기본적으로 백그라운드에서 실행되므로** Claude가 실행되는 동안 계속 작업합니다. **Linux의 Claude Desktop**은 Ubuntu 및 Debian에서 베타로 출시됩니다. 그리고 \*\*`/radio`\*\*는 Claude FM lo-fi 라디오를 튜닝합니다.

  [Week 27 다이제스트 읽기 →](/docs/ko/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="2026년 6월 22–26일" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**: 대화형 `/mcp` 메뉴 대신 셸에서 구성된 MCP 서버를 인증하고, 나중에 `claude mcp logout`으로 저장된 자격 증명을 지웁니다.

  이번 주의 다른 기능들: **shell mode는 명령 출력에 응답합니다** (`! npm test`는 두 번째 프롬프트 없이 설명을 받습니다); \*\*`/rewind`\*\*는 `/clear`가 실행되기 전의 대화를 재개할 수 있습니다; 그리고 **background subagents**는 이제 자동 거부 대신 주 세션에서 권한 프롬프트를 표시합니다.

  [Week 26 다이제스트 읽기 →](/docs/ko/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="2026년 6월 15–19일" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts**: 세션의 출력을 claude.ai의 라이브 공유 가능한 페이지로 변환하여 세션이 작업할 때 제자리에서 업데이트되며, 현재 Team 및 Enterprise 플랜에서 베타 버전으로 제공됩니다.

  이번 주의 다른 기능들: **deny 및 ask 규칙은 도구 매개변수와 일치합니다** `Tool(param:value)` 형식으로, 예를 들어 `Agent(model:opus)`; \*\*`/config key=value`\*\*는 프롬프트에서, `-p` 모드에서, 그리고 Remote Control에서 모든 설정을 지정합니다; 그리고 **auto mode는 로컬 작업을 버리도록 요청하지 않았을 때 파괴적인 git 명령을 차단합니다**.

  [Week 25 다이제스트 읽기 →](/docs/ko/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="2026년 6월 8–12일" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**: 프롬프트 캐시를 다시 구축하지 않고 대화 중간에 현재 세션을 새로운 작업 디렉토리로 이동합니다.

  이번 주의 다른 기능들: **sub-agents는 자신의 sub-agents를 생성할 수 있습니다** (백그라운드 체인은 5단계 깊이로 제한됨); \*\*`--safe-mode`\*\*는 문제 해결을 위해 모든 사용자 정의를 비활성화한 상태로 Claude Code를 시작합니다; 그리고 \*\*`fallbackModel`\*\*은 순서대로 시도되는 최대 3개의 폴백 모델을 구성합니다.

  [Week 24 다이제스트 읽기 →](/docs/ko/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="2026년 6월 1–5일" tags={["v2.1.158–v2.1.165"]}>
  **Amazon Bedrock, Google Cloud의 Agent Platform, 그리고 Microsoft Foundry의 Auto mode**: auto mode는 이제 Opus 4.7 및 Opus 4.8에 대해 타사 제공자에서 사용 가능하며, 권한 프롬프트를 백그라운드 안전 검사로 대체합니다.

  이번 주의 다른 기능들: **더 안전한 자동 편집**은 `acceptEdits` 모드에서 코드를 실행할 수 있는 파일을 작성하기 전에 프롬프트를 표시합니다; \*\*`/plugin list`\*\*는 설치된 플러그인을 인라인으로 출력합니다; 그리고 **version requirements**는 관리되는 배포에서 승인된 Claude Code 버전 범위를 요구할 수 있습니다.

  [Week 23 다이제스트 읽기 →](/docs/ko/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="2026년 5월 25–29일" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**: Max, Team Premium, Enterprise 종량제, 그리고 Anthropic API 계정의 새로운 기본 모델로, 기본적으로 높은 노력 수준을 제공하며 가장 어려운 작업을 위해 `/effort xhigh`를 지원합니다.

  이번 주의 다른 기능들: **dynamic workflows**는 Claude가 작성한 스크립트에서 수십 개에서 수백 개의 서브에이전트를 조율합니다. **security-guidance plugin**은 Claude가 작업하는 동안 변경 사항을 검토하여 취약점을 찾습니다. 그리고 **fast mode**는 Opus 4.8에서 실행되며 MTok당 \$10/\$50입니다.

  [Week 22 다이제스트 읽기 →](/docs/ko/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="2026년 5월 18–22일" tags={["v2.1.143–v2.1.149"]}>
  **Pro 플랜의 Auto mode**: auto mode는 이제 Pro 계정에서 실행되며 Opus와 함께 Sonnet 4.6을 지원하여 권한 프롬프트를 백그라운드 안전 검사로 대체합니다.

  이번 주의 다른 기능들: \*\*`/usage`\*\*는 스킬, 서브에이전트, 플러그인, 그리고 MCP 서버별로 플랜 제한을 유발하는 요소를 분석합니다. 새로운 **`/code-review`** 명령은 정확성 버그를 보고합니다. 그리고 **background sessions**은 `/resume`에 나타나며 고정될 때 활성 상태를 유지합니다.

  [Week 21 다이제스트 읽기 →](/docs/ko/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="2026년 5월 11–15일" tags={["v2.1.139–v2.1.142"]}>
  **Agent view**: `claude agents`는 모든 Claude Code 세션에 대해 하나의 화면을 열어서 실행 중인 것, 사용자의 입력을 기다리는 것, 완료된 것을 보여줍니다.

  이번 주의 다른 기능들: \*\*`/goal`\*\*은 완료 조건이 충족될 때까지 Claude가 여러 턴에 걸쳐 작업을 계속하도록 유지합니다. **fast mode**는 이제 기본적으로 Opus 4.7에서 실행됩니다. 그리고 **Rewind menu**는 "Summarize up to here"로 이전 컨텍스트를 압축할 수 있습니다.

  [Week 20 다이제스트 읽기 →](/docs/ko/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="2026년 5월 4–8일" tags={["v2.1.128–v2.1.136"]}>
  **플러그인이 `.zip` 아카이브 및 URL에서 로드됩니다**: `--plugin-dir`은 이제 `.zip` 파일을 허용하며, `--plugin-url`은 현재 세션에 대한 플러그인 아카이브를 가져옵니다.

  이번 주의 다른 기능들: \*\*`worktree.baseRef`\*\*는 새로운 worktree가 원격 기본값 또는 로컬 `HEAD`에서 분기할지 여부를 선택합니다. **auto mode hard deny rules**는 allow 예외와 관계없이 작업을 무조건 차단합니다. 그리고 **hooks는 활성 노력 수준을 봅니다** `effort.level` 및 `$CLAUDE_EFFORT`를 통해.

  [Week 19 다이제스트 읽기 →](/docs/ko/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="2026년 4월 27일 – 5월 1일" tags={["v2.1.120–v2.1.126"]}>
  **Git Bash 없는 Windows**: Git for Windows는 더 이상 필요하지 않으며, Claude Code는 Bash가 없을 때 PowerShell을 셸 도구로 사용합니다.

  이번 주의 다른 기능들: \*\*`claude ultrareview`\*\*는 CI 및 스크립트에 클라우드 코드 리뷰를 제공합니다. \*\*`claude project purge`\*\*는 프로젝트의 로컬 상태를 정리합니다. 그리고 **PR URL을 `/resume`에 붙여넣기**하면 이를 생성한 세션을 찾습니다.

  [Week 18 다이제스트 읽기 →](/docs/ko/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="2026년 4월 20–24일" tags={["v2.1.114–v2.1.119"]}>
  \*\*`/ultrareview`\*\*가 공개 연구 미리보기로 출시됩니다: 클라우드에서 실행되는 버그 사냥 에이전트 플릿이 발견 사항을 CLI 또는 Desktop으로 자동으로 전달합니다.

  이번 주의 다른 기능들: **session recap**은 터미널이 포커스를 잃은 동안 발생한 일을 보여줍니다. **custom themes**을 사용하면 `/theme` 또는 플러그인에서 색상 팔레트를 구축하고 배포할 수 있습니다. 그리고 **Claude Code on the web**은 새로운 세션 사이드바와 드래그 앤 드롭 레이아웃으로 재설계되었습니다.

  [Week 17 다이제스트 읽기 →](/docs/ko/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="2026년 4월 13–17일" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7**이 Max 및 Team Premium의 새로운 기본값으로 출시되며, 대부분의 코딩 작업에 권장되는 새로운 `xhigh` 노력 수준과 이를 조정할 수 있는 대화형 `/effort` 슬라이더가 포함됩니다.

  이번 주의 다른 기능들: Claude Code on the web의 **Routines**은 일정, GitHub 이벤트 또는 API 호출에서 템플릿 클라우드 에이전트를 실행합니다. **mobile push notifications**은 긴 작업이 완료되거나 Claude가 필요할 때 휴대폰에 알림을 보냅니다. `/usage`는 제한을 유발하는 요소를 표시합니다. 그리고 CLI는 네이티브 바이너리로 이동합니다.

  [Week 16 다이제스트 읽기 →](/docs/ko/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="2026년 4월 6–10일" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan**이 초기 미리보기에 진입합니다: CLI에서 클라우드의 계획을 작성하고, 웹 편집기에서 검토 및 댓글을 달고, 원격으로 실행하거나 로컬로 다시 가져옵니다. 첫 번째 실행은 이제 자동으로 클라우드 환경을 생성합니다.

  이번 주의 다른 기능들: **Monitor** 도구는 백그라운드 이벤트를 대화로 스트리밍하므로 Claude가 로그를 추적하고 실시간으로 반응할 수 있습니다. `/loop`는 간격을 생략할 때 자동으로 속도를 조절합니다. `/team-onboarding`은 설정을 재생 가능한 가이드로 패키징합니다. 그리고 `/autofix-pr`은 터미널에서 PR 자동 수정을 켭니다.

  [Week 15 다이제스트 읽기 →](/docs/ko/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="2026년 3월 30일 – 4월 3일" tags={["v2.1.86–v2.1.91"]}>
  **Computer use**가 연구 미리보기에서 CLI로 제공됩니다: Claude는 네이티브 앱을 열고, UI를 클릭하고, 터미널에서 변경 사항을 확인할 수 있습니다. GUI만 확인할 수 있는 작업을 완료하는 데 가장 좋습니다.

  이번 주의 다른 기능들: `/powerup` 대화형 레슨, 깜박임 없는 alt-screen 렌더링, 도구당 MCP 결과 크기 재정의(최대 500K), 그리고 Bash 도구의 `PATH`에 플러그인 실행 파일이 있습니다.

  [Week 14 다이제스트 읽기 →](/docs/ko/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="2026년 3월 23–27일" tags={["v2.1.83–v2.1.85"]}>
  **Auto mode**가 연구 미리보기에서 출시됩니다: 분류기가 권한 프롬프트를 처리하므로 안전한 작업은 중단 없이 실행되고 위험한 작업은 차단됩니다. 모든 것을 승인하는 것과 `--dangerously-skip-permissions` 사이의 중간 지점입니다.

  이번 주의 다른 기능들: Desktop 앱의 컴퓨터 사용, Web의 PR 자동 수정, `/`를 사용한 트랜스크립트 검색, Windows용 네이티브 PowerShell 도구, 그리고 조건부 `if` 훅입니다.

  [Week 13 다이제스트 읽기 →](/docs/ko/whats-new/2026-w13)
</Update>
