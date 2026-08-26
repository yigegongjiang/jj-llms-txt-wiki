> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 스크린 리더로 Claude Code 사용하기

> VoiceOver 및 NVDA와 같은 스크린 리더, 스크린 확대기, 감소된 모션, 색맹 친화적 테마에 대한 Claude Code 설정하기.

Claude Code는 시각적 터미널 인터페이스를 일반 텍스트로 바꾸는 스크린 리더 모드를 갖추고 있습니다. 상자, 진행 애니메이션, 제자리 다시 그리기 대신 이 모드는 VoiceOver 또는 NVDA와 같은 스크린 리더가 순서대로 읽을 수 있는 레이블이 지정된 줄을 인쇄하므로 전체 대화를 진행하고, 도구 권한을 승인하고, 출력을 끝까지 검토할 수 있습니다.

스크린 리더 모드는 선택 사항입니다. 스크린 리더 대신 스크린 확대기, 감소된 모션 또는 색맹 친화적 테마를 사용하는 경우 [스크린 리더 모드 이외의 접근성 설정](#accessibility-settings-beyond-screen-reader-mode)을 참조하십시오.

<Note>
  스크린 리더 모드는 Claude Code v2.1.181 이상이 필요합니다. 이전 버전은 `--ax-screen-reader` 플래그를 `error: unknown option '--ax-screen-reader'`로 거부합니다.
</Note>

<h2 id="turn-on-screen-reader-mode">
  스크린 리더 모드 켜기
</h2>

스크린 리더를 사용하는 빈도와 일치하는 방법을 선택하십시오:

* 한 세션의 경우: `claude --ax-screen-reader`를 실행합니다.
* 한 셸에서 시작된 세션의 경우: `CLAUDE_AX_SCREEN_READER` 환경 변수를 `1`로 설정합니다. Bash 또는 Zsh에서는 `export CLAUDE_AX_SCREEN_READER=1`을 실행하고, PowerShell에서는 `$env:CLAUDE_AX_SCREEN_READER = "1"`을 실행합니다. 모든 셸을 포함하려면 셸 프로필에 줄을 추가합니다.
* 머신의 모든 세션의 경우: 사용자 [설정 파일](/docs/ko/settings)에 `"axScreenReader": true`를 추가합니다. 이는 VS Code 통합 터미널을 포함한 모든 터미널을 포함합니다.

<Note>
  메서드는 우선순위 순서로 나열됩니다: [`--ax-screen-reader`](/docs/ko/cli-reference#cli-flags) 플래그는 [`CLAUDE_AX_SCREEN_READER`](/docs/ko/env-vars) 환경 변수를 재정의하고, 이는 [`axScreenReader`](/docs/ko/settings#available-settings) 설정을 재정의합니다.
</Note>

SSH를 통해 Claude Code를 사용하는 경우 Claude Code가 실행되는 원격 머신에서 환경 변수 또는 설정을 설정합니다.

모드가 켜져 있으면 Claude Code가 인쇄하는 첫 번째 것은 모드를 켠 메서드의 이름을 지정하는 확인 줄입니다: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]` 또는 `[Screen Reader Mode: on via settings]`. 메서드 명명 형식은 Claude Code v2.1.206 이상이 필요합니다. Claude Code가 예를 들어 업데이트 설치를 완료하기 위해 자신을 다시 시작할 때 새 프로세스는 `CLAUDE_AX_SCREEN_READER` 환경 변수를 통해 모드를 상속하므로 사용한 메서드와 관계없이 확인 줄은 `[Screen Reader Mode: on via env]`로 읽힙니다.
이전 버전은 `[Accessible screen reader mode: on]`을 인쇄합니다.

<h2 id="turn-off-screen-reader-mode">
  스크린 리더 모드 끄기
</h2>

모드를 켠 메서드를 역으로 수행합니다: 플래그 없이 시작하거나, 환경 변수를 설정 해제하거나, `axScreenReader`를 `false`로 설정합니다. `CLAUDE_AX_SCREEN_READER=0`을 설정하면 설정이 `true`일 때도 모드가 꺼진 상태로 유지됩니다.

<h2 id="what-your-screen-reader-hears">
  스크린 리더가 듣는 것
</h2>

스크린 리더 모드에서 Claude Code는 평문을 작성합니다:

* 인터페이스 크롬에 대한 상자 그리기 문자 없음
* 색상 전용 신호 없음
* 변경되지 않은 콘텐츠의 다시 그리기 없음; 진행 스피너는 정적 텍스트로 렌더링됨
* Claude의 회신의 표는 상자 문자 그리드 대신 `Header: value` 문장으로 읽힙니다. Claude Code v2.1.198 이상이 필요합니다; 이전 버전은 스크린 리더 모드에서도 표를 그리드로 그립니다.

출력은 터미널의 스크롤백에 누적되므로 스크린 리더의 검토 명령 또는 터미널의 검색을 사용하여 이전 턴을 다시 읽을 수 있습니다.

스크린 리더 모드는 [`tui` 설정](/docs/ko/settings#available-settings)으로 [전체 화면 렌더링](/docs/ko/fullscreen)을 켜도 평문 스크롤로 렌더링됩니다; 모드가 활성화되어 있는 동안 설정은 효과가 없습니다. 첨부된 백그라운드 세션은 여전히 전체 화면으로 렌더링됩니다; [알려진 제한 사항](#known-limitations)을 참조하십시오.

트랜스크립트의 각 메시지는 스크린 리더가 발표하는 레이블로 시작하며, 메시지가 무엇인지 이름을 지정합니다: 사용자 메시지, Claude의 회신, 도구 활동, 오류 및 프롬프트. 레이블은 검색 가능하므로 터미널의 스크롤백을 검색하여 트랜스크립트의 섹션 간에 이동할 수 있습니다:

| 레이블                    | 의미                                                        |
| :--------------------- | :-------------------------------------------------------- |
| `you:`                 | 사용자 메시지                                                   |
| `claude:`              | Claude의 회신                                                |
| `tool:`                | 파일 편집 또는 명령 실행과 같은 도구 활동                                  |
| `tool error:`          | 실패한 도구                                                    |
| `error:`               | 실패한 API 요청과 같은 대화의 오류                                     |
| `Permission Required:` | 사용자의 답변을 기다리는 권한 프롬프트                                     |
| `Cost:`                | Claude Code가 종료될 때 세션 비용 요약(계정이 [비용을 표시](/docs/ko/costs)하는 경우) |

터미널 커서는 입력 캐럿을 따르므로 스크린 리더의 현재 줄 읽기 명령은 편집 중인 프롬프트로 "내가 어디에 있는가"에 답합니다.

<h3 id="jump-between-turns">
  턴 간 이동
</h3>

Claude Code는 턴 경계에서 OSC 133 셸 통합 마커를 내보내므로 터미널의 이전 프롬프트로 이동 키는 전체 트랜스크립트를 읽지 않고 턴 간에 이동합니다:

* iTerm2: Cmd+Shift+Up
* VS Code 터미널: Windows에서 Ctrl+Up, macOS에서 Cmd+Up
* Windows Terminal: 기본적으로 키가 없음; 설정에서 `scrollToMark` 작업을 바인딩합니다
* Kitty 및 Ghostty: 터미널의 설명서에서 프롬프트로 이동 키를 확인합니다

macOS Terminal은 마커에 작용하지 않으며 Claude Code는 WezTerm에서 마커를 내보내지 않습니다. 이러한 터미널에서는 스크롤백에서 `you:` 레이블을 검색합니다.

<h2 id="answer-menus-and-prompts">
  메뉴 및 프롬프트에 답하기
</h2>

스크린 리더 모드에서 일반적으로 화살표 키로 탐색하는 메뉴(권한 프롬프트 포함)는 번호가 지정된 목록이 됩니다. 각 옵션은 번호가 지정된 줄로 발표되고 유효한 범위의 이름을 지정하는 `Enter selection` 프롬프트가 뒤따릅니다. 원하는 옵션의 번호를 입력하고 Enter를 누릅니다.

* 해제 가능한 메뉴를 취소하려면: Escape를 누릅니다. 프롬프트는 `or Escape to cancel`로 끝납니다.
* 목록에 없는 번호를 입력하면: Claude Code는 유효한 범위를 발표하고 다시 시도하도록 합니다.

예/아니오 프롬프트는 두 옵션 메뉴 대신 입력된 답변을 요청합니다. `y` 또는 `n`으로 답하고 Enter를 누릅니다. `yes` 및 `no`도 작동합니다.

<h2 id="hear-when-claude-code-needs-you">
  Claude Code가 필요할 때 듣기
</h2>

스크린 리더 모드에서 Claude Code는 필요할 때 터미널 벨을 울리므로 트랜스크립트를 계속 확인할 필요가 없습니다. 벨은 다음과 같은 경우에 울립니다:

* Claude가 회신을 완료합니다
* 권한 프롬프트가 나타납니다
* 5초 이상 실행된 도구가 완료됩니다

벨은 터미널의 표준 경고입니다. 벨을 끄려면 터미널 애플리케이션에서 벨 설정을 변경합니다. 벨은 스크린 리더 모드를 요구하지 않습니다: 모드 외부에서 [`preferredNotifChannel`](/docs/ko/settings#available-settings)을 `"terminal_bell"`로 설정하여 Claude가 사용자를 기다릴 때 유사한 경고를 받습니다. [터미널 벨 또는 알림 받기](/docs/ko/terminal-config#get-a-terminal-bell-or-notification)를 참조하십시오.

<h2 id="accessibility-settings-beyond-screen-reader-mode">
  스크린 리더 모드 이외의 접근성 설정
</h2>

이러한 옵션은 스크린 리더 모드 외부의 접근성 요구 사항을 해결합니다. 모두 함께 작동합니다.

* `CLAUDE_CODE_ACCESSIBILITY` [환경 변수](/docs/ko/env-vars)는 스크린 확대기용입니다. `CLAUDE_CODE_ACCESSIBILITY=1`을 설정하여 macOS Zoom과 같은 확대기가 커서 위치를 추적할 수 있도록 기본 터미널 커서를 표시합니다.
* `prefersReducedMotion` [설정](/docs/ko/settings#available-settings)은 인터페이스의 나머지를 변경하지 않고 스피너, 반짝임 및 기타 애니메이션을 줄이거나 비활성화합니다.
* `theme` [설정](/docs/ko/settings#available-settings)은 색맹 친화적 `dark-daltonized` 및 `light-daltonized` 테마를 포함한 인터페이스 색상을 선택합니다.

<h2 id="known-limitations">
  알려진 제한 사항
</h2>

일부 동작은 스크린 리더 모드에 맞게 조정되지 않습니다:

* 스크린 리더 모드는 스크린 리더가 실행 중일 때 자동으로 켜지지 않습니다.
* [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode) 진입과 같은 모드 변경은 아직 발표되지 않습니다.
* `claude attach` 또는 에이전트 보기에서 [백그라운드 세션](/docs/ko/agent-view)에 첨부하면 기본 스크롤백이 없는 터미널의 대체 화면으로 들어갑니다. 이는 [다른 첨부된 세션과 동일한 동작](/docs/ko/fullscreen)입니다. 나가려면 빈 프롬프트에서 왼쪽 화살표를 누르거나 대화 상자에 포커스가 있으면 Ctrl+Z를 누릅니다.
* Claude Code는 종료 시 인쇄하는 요약에서 비용을 발표하며, 턴당이 아닙니다.
* 스크린 리더 모드는 `-p` 플래그로 [비대화형 모드](/docs/ko/headless)를 변경하지 않습니다. 비대화형 모드는 이미 평문을 작성하며 스크립팅을 위한 대안으로 남아 있습니다.

<h2 id="report-an-issue">
  문제 보고
</h2>

스크린 리더, 확대기 또는 터미널에서 작동하지 않는 경우 [Claude Code 이슈 추적기](https://github.com/anthropics/claude-code/issues)에서 이슈를 열고 제목에 보조 기술을 언급합니다. 보고서에 운영 체제, 터미널 애플리케이션, 보조 기술 이름 및 버전을 포함합니다.

<h2 id="related-resources">
  관련 리소스
</h2>

이 페이지는 이 페이지에서 다루는 내용에 대한 전체 참조 항목 및 관련 설정을 보유합니다:

* [설정](/docs/ko/settings#available-settings): `axScreenReader`, `prefersReducedMotion`, `theme` 및 `preferredNotifChannel` 항목
* [환경 변수](/docs/ko/env-vars): `CLAUDE_AX_SCREEN_READER` 및 `CLAUDE_CODE_ACCESSIBILITY` 항목
* [CLI 참조](/docs/ko/cli-reference#cli-flags): `--ax-screen-reader` 플래그
* [터미널 구성](/docs/ko/terminal-config): 스크린 리더 모드 외부의 벨, 알림 및 테마
* [비대화형 모드](/docs/ko/headless): 스크린 리더 모드 없이 평문을 작성하는 스크립팅된 `claude -p` 실행
