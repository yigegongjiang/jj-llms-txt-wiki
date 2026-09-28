> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 새로운 기능

> Claude Code 기능의 주간 요약으로, 코드 스니펫, 데모, 그리고 그 중요성에 대한 맥락을 포함합니다.

주간 개발자 요약은 작업 방식을 바꿀 가능성이 가장 높은 기능들을 강조합니다. 각 항목에는 실행 가능한 코드, 짧은 데모, 그리고 전체 문서로의 링크가 포함됩니다. 모든 버그 수정 및 사소한 개선 사항은 [changelog](/docs/en/changelog)를 참조하십시오.

<Update label="Week 37" description="2026년 9월 7–11일" tags={["v2.1.263–v2.1.269"]}>
  **`claude plugin eval`**: 플러그인을 테스트 케이스 모음에 대해 실행하고, 결과를 채점하며, 플러그인 없는 기준선과 비교합니다. `claude plugin eval init`은 케이스와 채점자를 자동으로 작성합니다.

  이번 주의 다른 기능: **Claude Code Desktop 창**을 자신의 창으로 팝업하고 나중에 다시 도킹할 수 있습니다. **`maxEffortLevel`** 설정은 모든 제공자의 노력 수준을 제한합니다. 그리고 **WebFetch**가 5분 이내에 다운로드를 완료하지 못한 페이지는 중단되지 않고 실패합니다.

  [Week 37 요약 읽기 →](/docs/ko/whats-new/2026-w37)
</Update>

<Update label="Week 36" description="2026년 8월 31일 – 9월 4일" tags={["v2.1.251–v2.1.261"]}>
  **Claude Fable 5.1**: 1M 토큰 컨텍스트 윈도우와 함께 Claude Code에서 사용 가능합니다.

  이번 주의 다른 기능: Pro 및 Max 플랜에서 **Desktop 앱의 컴퓨터 사용**이 macOS에서 백그라운드에서 작동하므로 계속 작업할 수 있습니다. 전체 화면 렌더링에서 \*\*`/diff`\*\*는 대화 옆에 라이브 패널을 열어 Claude가 편집할 때 새로고침됩니다. 그리고 \*\*`/skill-doctor`\*\*는 각 스킬이 컨텍스트에서 얼마나 비용이 드는지, 그리고 얼마나 자주 사용되는지 보여줍니다.

  [Week 36 요약 읽기 →](/docs/ko/whats-new/2026-w36)
</Update>

<Update label="Week 35" description="2026년 8월 24–28일" tags={["v2.1.240–v2.1.250"]}>
  **Desktop 앱에서 터미널 세션 재개**: Claude Code Desktop 프롬프트 상자에 `/resume`을 입력하여 CLI에서 시작한 모든 세션을 선택하고, 전체 대화와 컨텍스트를 그대로 유지합니다.

  이번 주의 다른 기능: **Claude 작성 피드백**은 세션에서 문제가 발생할 때 Claude가 피드백 보고서를 작성하고, 이를 검토한 후 `/feedback`에서 전송합니다. \*\*`--restricted`\*\*는 명령 실행 도구나 사용자 및 프로젝트 설정 없이 세션을 시작하므로 공유 머신의 평가 하네스에 사용됩니다. 그리고 **`modelPicker`** 설정은 `/model` 선택기가 나열하는 모델을 제어합니다.

  [Week 35 요약 읽기 →](/docs/ko/whats-new/2026-w35)
</Update>

<Update label="Week 34" description="2026년 8월 17–21일" tags={["v2.1.234–v2.1.239"]}>
  **`/design`**: Claude Design의 아트보드 워크플로우를 CLI 및 Claude Code Desktop으로 가져오는 연구 미리보기로, 아티팩트를 기반으로 하므로 Claude가 UI용 편집 가능한 아트보드를 작성하고 선택한 것을 구현합니다.

  이번 주의 다른 기능: 기본 제공 **간결한 출력 스타일**은 Claude가 결과로 시작하고 전문을 건너뜁니다. `claude remote-control`을 실행하는 모든 머신은 휴대폰의 **기기 카드**로 표시되므로 Code 탭에서 시작할 수 있습니다. 그리고 \*\*`ANTHROPIC_DEFAULT_MODEL`\*\*은 새 세션이 시작되는 모델을 설정합니다.

  [Week 34 요약 읽기 →](/docs/ko/whats-new/2026-w34)
</Update>

<Update label="Week 33" description="2026년 8월 10–14일" tags={["v2.1.225–v2.1.233"]}>
  **Desktop의 사용량 제한 후 자동 계속**: Claude Code Desktop에서 세션 제한에 도달하면 제한 카드에서 **제한이 재설정될 때 자동 계속**을 확인하고 제한이 재설정되면 앱이 중단된 턴을 다시 시도합니다.

  이번 주의 다른 기능: **포크 모드**는 대화형 세션에서 기본적으로 켜져 있으므로 Claude는 전체 대화를 상속하는 서브에이전트에 부작업을 위임할 수 있습니다. **GitLab** 병합 요청 URL은 `--worktree` 및 `claude agents` 보기와 함께 작동하며, 마켓플레이스는 베어 `gitlab.com` URL을 복제합니다. 그리고 프롬프트에서 \*\*`@`\*\*를 입력하면 다른 Claude 세션을 이름으로 언급합니다.

  [Week 33 요약 읽기 →](/docs/ko/whats-new/2026-w33)
</Update>

<Update label="Week 32" description="2026년 8월 3–7일" tags={["v2.1.220–v2.1.224"]}>
  **세션 간 메시징**: macOS 및 Linux에서 Claude Code 세션은 이제 서로 메시지를 보낼 수 있으므로 Claude는 한 세션에서 다른 세션으로 발견 사항이나 결정을 전달하며, 사용자가 다시 설명할 필요가 없습니다.

  이번 주의 다른 기능: **자체 호스팅 환경**은 조직이 운영하는 인프라에서 Claude Code 클라우드 세션을 실행하며, Team 및 Enterprise 플랜에서 공개 베타 상태입니다. **자동 모드**는 2026년 8월 14일부터 Pro, Max, Team 플랜의 새 세션에 대한 기본 권한 모드가 됩니다. 그리고 **VS Code 확장**은 Focus 보기를 얻습니다.

  [Week 32 요약 읽기 →](/docs/ko/whats-new/2026-w32)
</Update>

<Update label="Week 30" description="2026년 7월 20–24일" tags={["v2.1.214–v2.1.219"]}>
  **Claude Opus 5**: Claude Code의 새로운 기본 Opus 모델로, 1M 토큰 컨텍스트 윈도우와 MTok당 \$10/\$50의 빠른 모드를 제공합니다.

  이번 주의 다른 기능: **Claude Code Desktop**은 공개 베타에서 iOS Simulator 창을 열어 Claude가 앱을 실행하고 탭을 통해 이동할 수 있으며 사용자가 지켜볼 수 있습니다. **Claude Security 플러그인**은 코드베이스의 다중 에이전트 취약점 스캔을 실행하고 선택한 발견 사항을 직접 적용하는 패치로 변환합니다. 그리고 \*\*`/code-review`\*\*는 백그라운드 서브에이전트로 실행됩니다.

  [Week 30 요약 읽기 →](/docs/ko/whats-new/2026-w30)
</Update>

<Update label="Week 29" description="2026년 7월 13–17일" tags={["v2.1.207–v2.1.212"]}>
  **아티팩트가 MCP 커넥터를 호출합니다**: 게시된 아티팩트는 각 뷰어가 페이지를 열 때 자신의 MCP 커넥터를 통해 라이브 데이터를 가져오고 작업을 수행할 수 있으며, 이번 주에는 공개 공유 링크, Team 및 Enterprise의 편집자 역할, 그리고 Claude Tag 세션에서 생성된 아티팩트도 추가됩니다.

  이번 주의 다른 기능: **화면 읽기 모드**는 시각적 터미널 인터페이스를 VoiceOver 및 NVDA와 같은 화면 읽기 프로그램을 위한 일반 선형 텍스트로 바꿉니다. \*\*`/fork`\*\*는 대화를 새 백그라운드 세션으로 복사하면서 계속 작업합니다. 그리고 **자동 모드**는 더 이상 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry에서 옵트인 변수가 필요하지 않습니다.

  [Week 29 요약 읽기 →](/docs/ko/whats-new/2026-w29)
</Update>

<Update label="Week 28" description="2026년 7월 6–10일" tags={["v2.1.202–v2.1.206"]}>
  **Desktop의 인앱 브라우저**: Desktop의 Claude Code는 기본 제공 브라우저를 얻으므로 Claude는 문서, 디자인 또는 기타 사이트를 불러올 수 있으며 로컬 개발 서버 미리보기와 동일한 방식으로 페이지와 상호 작용할 수 있습니다.

  이번 주의 다른 기능: \*\*`/doctor`\*\*는 문제를 진단하고 수정할 수 있는 전체 설정 점검이며, `/checkup`이 별칭입니다. **자동 모드**는 트랜스크립트 변조를 차단하고 미해결 변수에서 `rm -rf` 전에 요청합니다. 그리고 **에이전트 보기 행**은 색상이 지정된 상태 단어와 분류기 작성 헤드라인을 표시합니다.

  [Week 28 요약 읽기 →](/docs/ko/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="2026년 6월 29일 – 7월 3일" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**: Pro, Team Standard, Enterprise 구독 좌석의 새로운 기본 모델로, 최고 수준의 코딩 및 도구 사용이 Sonnet 가격으로 제공되며, 기본 1M 토큰 컨텍스트 윈도우와 기본적으로 활성화된 적응형 사고를 제공합니다.

  이번 주의 다른 기능: **Chrome의 Claude**는 모든 직접 Anthropic 플랜에서 일반 공급됩니다. **서브에이전트는 기본적으로 백그라운드에서 실행**되므로 Claude는 실행 중에 계속 작업합니다. **Linux의 Claude Desktop**은 Ubuntu 및 Debian에서 베타 상태로 출시됩니다. 그리고 \*\*`/radio`\*\*는 Claude FM 로파이 라디오를 튜닝합니다.

  [Week 27 요약 읽기 →](/docs/ko/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="2026년 6월 22–26일" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**: 대화형 `/mcp` 메뉴 대신 셸에서 구성된 MCP 서버를 인증하고, 나중에 `claude mcp logout`으로 저장된 자격 증명을 지웁니다.

  이번 주의 다른 기능: **셸 모드는 명령 출력에 응답**하므로 (`! npm test`는 두 번째 프롬프트 없이 설명을 얻습니다). \*\*`/rewind`\*\*는 `/clear`가 실행되기 전의 대화를 재개할 수 있습니다. 그리고 **백그라운드 서브에이전트**는 이제 자동 거부 대신 주 세션에서 권한 프롬프트를 표시합니다.

  [Week 26 요약 읽기 →](/docs/ko/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="2026년 6월 15–19일" tags={["v2.1.178–v2.1.183"]}>
  **아티팩트**: 세션의 출력을 claude.ai의 라이브 공유 가능 페이지로 변환하여 세션이 작동할 때 제자리에서 업데이트되며, 현재 Team 및 Enterprise 플랜에서 베타 상태입니다.

  이번 주의 다른 기능: **거부 및 요청 규칙은 도구 매개변수와 일치**하며 `Tool(param:value)` 형식입니다(예: `Agent(model:opus)`). \*\*`/config key=value`\*\*는 프롬프트에서 모든 설정을 설정하며, `-p` 모드 및 Remote Control에서 설정합니다. 그리고 **자동 모드는 파괴적인 git 명령을 차단**하면 로컬 작업을 버리도록 요청하지 않았습니다.

  [Week 25 요약 읽기 →](/docs/ko/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="2026년 6월 8–12일" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**: 프롬프트 캐시를 다시 빌드하지 않고 대화 중간에 현재 세션을 새 작업 디렉토리로 이동합니다.

  이번 주의 다른 기능: **서브 에이전트는 자신의 서브 에이전트를 생성할 수 있습니다**(백그라운드 체인은 5단계 깊이로 제한됨). \*\*`--safe-mode`\*\*는 문제 해결을 위해 모든 사용자 정의를 비활성화하여 Claude Code를 시작합니다. 그리고 \*\*`fallbackModel`\*\*은 순서대로 시도되는 최대 3개의 폴백 모델을 구성합니다.

  [Week 24 요약 읽기 →](/docs/ko/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="2026년 6월 1–5일" tags={["v2.1.158–v2.1.165"]}>
  **Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry의 자동 모드**: 자동 모드는 이제 Opus 4.7 및 Opus 4.8의 타사 제공자에서 사용 가능하며, 권한 프롬프트를 백그라운드 안전 검사로 대체합니다.

  이번 주의 다른 기능: **더 안전한 자동 편집**은 `acceptEdits` 모드에서 코드를 실행할 수 있는 파일을 작성하기 전에 프롬프트합니다. \*\*`/plugin list`\*\*는 설치된 플러그인을 인라인으로 인쇄합니다. 그리고 **버전 요구 사항**은 관리되는 배포가 승인된 Claude Code 버전 범위를 요구하도록 합니다.

  [Week 23 요약 읽기 →](/docs/ko/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="2026년 5월 25–29일" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**: Max, Team Premium, Enterprise 종량제, Anthropic API 계정의 새로운 기본 모델로, 기본적으로 높은 노력과 가장 어려운 작업을 위한 `/effort xhigh`를 제공합니다.

  이번 주의 다른 기능: **동적 워크플로우**는 Claude가 작성한 스크립트에서 수십 개에서 수백 개의 서브에이전트를 조율합니다. **보안 지침 플러그인**은 Claude의 변경 사항을 작동하면서 취약점을 검토합니다. 그리고 **빠른 모드**는 MTok당 \$10/\$50의 Opus 4.8에서 실행됩니다.

  [Week 22 요약 읽기 →](/docs/ko/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="2026년 5월 18–22일" tags={["v2.1.143–v2.1.149"]}>
  **Pro 플랜의 자동 모드**: 자동 모드는 이제 Pro 계정에서 실행되며 Opus와 함께 Sonnet 4.6을 지원하여 권한 프롬프트를 백그라운드 안전 검사로 대체합니다.

  이번 주의 다른 기능: \*\*`/usage`\*\*는 스킬, 서브에이전트, 플러그인, MCP 서버별로 플랜 제한을 구동하는 것을 분석합니다. 새로운 **`/code-review`** 명령은 정확성 버그를 보고합니다. 그리고 **백그라운드 세션**은 `/resume`에 나타나고 고정되면 활성 상태로 유지됩니다.

  [Week 21 요약 읽기 →](/docs/ko/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="2026년 5월 11–15일" tags={["v2.1.139–v2.1.142"]}>
  **에이전트 보기**: `claude agents`는 모든 Claude Code 세션에 대해 하나의 화면을 열어 실행 중인 것, 사용자가 차단한 것, 완료된 것을 보여줍니다.

  이번 주의 다른 기능: \*\*`/goal`\*\*은 완료 조건이 유지될 때까지 Claude가 턴 전체에서 작동하도록 유지합니다. **빠른 모드**는 이제 기본적으로 Opus 4.7에서 실행됩니다. 그리고 **Rewind 메뉴**는 "여기까지 요약"으로 이전 컨텍스트를 압축할 수 있습니다.

  [Week 20 요약 읽기 →](/docs/ko/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="2026년 5월 4–8일" tags={["v2.1.128–v2.1.136"]}>
  **플러그인은 `.zip` 아카이브 및 URL에서 로드됩니다**: `--plugin-dir`은 이제 `.zip` 파일을 허용하며, `--plugin-url`은 현재 세션에 대한 플러그인 아카이브를 가져옵니다.

  이번 주의 다른 기능: \*\*`worktree.baseRef`\*\*는 새 worktree가 원격 기본값 또는 로컬 `HEAD`에서 분기할지 여부를 선택합니다. **자동 모드 하드 거부 규칙**은 허용 예외와 관계없이 작업을 무조건 차단합니다. 그리고 **훅은 활성 노력 수준을 봅니다** `effort.level` 및 `$CLAUDE_EFFORT`를 통해.

  [Week 19 요약 읽기 →](/docs/ko/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="2026년 4월 27일 – 5월 1일" tags={["v2.1.120–v2.1.126"]}>
  **Git Bash 없는 Windows**: Git for Windows는 더 이상 필요하지 않으며, Claude Code는 Bash가 없을 때 PowerShell을 셸 도구로 사용합니다.

  이번 주의 다른 기능: \*\*`claude ultrareview`\*\*는 클라우드 코드 검토를 CI 및 스크립트로 가져옵니다. \*\*`claude project purge`\*\*는 프로젝트의 로컬 상태를 정리합니다. 그리고 **PR URL을 `/resume`에 붙여넣기**하면 이를 생성한 세션을 찾습니다.

  [Week 18 요약 읽기 →](/docs/ko/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="2026년 4월 20–24일" tags={["v2.1.114–v2.1.119"]}>
  \*\*`/ultrareview`\*\*는 공개 연구 미리보기로 열립니다: 버그 사냥 에이전트 함대가 클라우드에서 실행되고 발견 사항이 CLI 또는 Desktop으로 자동으로 돌아옵니다.

  이번 주의 다른 기능: **세션 요약**은 터미널이 포커스를 잃은 동안 발생한 일을 보여줍니다. **사용자 정의 테마**는 `/theme` 또는 플러그인에서 색상 팔레트를 빌드하고 배포할 수 있습니다. 그리고 **웹의 Claude Code**는 새로운 세션 사이드바 및 드래그 앤 드롭 레이아웃으로 재설계됩니다.

  [Week 17 요약 읽기 →](/docs/ko/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="2026년 4월 13–17일" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7**은 Max 및 Team Premium의 새로운 기본값으로 출시되며, 대부분의 코딩 작업에 권장되는 설정인 새로운 `xhigh` 노력 수준과 대화형 `/effort` 슬라이더를 제공하여 조정할 수 있습니다.

  이번 주의 다른 기능: **루틴**은 웹의 Claude Code에서 일정, GitHub 이벤트 또는 API 호출에서 템플릿 클라우드 에이전트를 실행합니다. **모바일 푸시 알림**은 긴 작업이 완료되거나 Claude가 필요할 때 휴대폰에 핑을 보냅니다. `/usage`는 제한을 구동하는 것을 보여줍니다. 그리고 CLI는 기본 바이너리로 이동합니다.

  [Week 16 요약 읽기 →](/docs/ko/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="2026년 4월 6–10일" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan**은 초기 미리보기에 진입합니다: CLI에서 클라우드의 계획을 작성하고, 웹 편집기에서 검토 및 댓글을 달고, 원격으로 실행하거나 로컬로 다시 가져옵니다. 첫 번째 실행은 이제 자동으로 클라우드 환경을 생성합니다.

  이번 주의 다른 기능: **Monitor** 도구는 백그라운드 이벤트를 대화로 스트리밍하므로 Claude는 로그를 추적하고 실시간으로 반응할 수 있습니다. `/loop`는 간격을 생략할 때 자체 속도로 진행됩니다. `/team-onboarding`은 설정을 재생 가능한 가이드로 패키징합니다. 그리고 `/autofix-pr`은 터미널에서 PR 자동 수정을 켭니다.

  [Week 15 요약 읽기 →](/docs/ko/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="2026년 3월 30일 – 4월 3일" tags={["v2.1.86–v2.1.91"]}>
  **컴퓨터 사용**은 연구 미리보기에서 CLI로 옵니다: Claude는 기본 앱을 열고, UI를 클릭하고, 터미널에서 변경 사항을 확인할 수 있습니다. GUI만 확인할 수 있는 것들을 닫는 데 가장 좋습니다.

  이번 주의 다른 기능: `/powerup` 대화형 수업, 깜박임 없는 alt-screen 렌더링, 500K까지의 도구별 MCP 결과 크기 재정의, 그리고 Bash 도구의 `PATH`에 플러그인 실행 파일.

  [Week 14 요약 읽기 →](/docs/ko/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="2026년 3월 23–27일" tags={["v2.1.83–v2.1.85"]}>
  **자동 모드**는 연구 미리보기에서 출시됩니다: 분류기가 권한 프롬프트를 처리하므로 안전한 작업은 중단 없이 실행되고 위험한 작업은 차단됩니다. 모든 것을 승인하는 것과 `--dangerously-skip-permissions` 사이의 중간 지점입니다.

  이번 주의 다른 기능: Desktop 앱의 컴퓨터 사용, Web의 PR 자동 수정, `/`를 사용한 트랜스크립트 검색, Windows용 기본 PowerShell 도구, 그리고 조건부 `if` 훅.

  [Week 13 요약 읽기 →](/docs/ko/whats-new/2026-w13)
</Update>
