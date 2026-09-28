> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 스크린 리더로 Claude Code 사용하기

> VoiceOver 및 NVDA와 같은 스크린 리더, 스크린 확대기, 감소된 모션, 색맹 친화적 테마에 대한 Claude Code 설정하기.

Claude Code는 시각적 터미널 인터페이스를 일반 텍스트로 바꾸는 스크린 리더 모드를 갖추고 있습니다. 상자, 진행 애니메이션, 제자리 다시 그리기 대신 이 모드는 VoiceOver 또는 NVDA와 같은 스크린 리더가 순서대로 읽을 수 있는 레이블이 지정된 줄을 인쇄하므로 전체 대화를 진행하고, 도구 권한을 승인하고, 출력을 끝까지 검토할 수 있습니다.

스크린 리더 모드는 선택 사항입니다. 스크린 리더 대신 스크린 확대기, 감소된 모션 또는 색맹 친화적 테마를 사용하는 경우 [접근성 설정](#accessibility-settings) 표에서 `CLAUDE_CODE_ACCESSIBILITY`, `prefersReducedMotion` 또는 `theme`을 설정하십시오. 스크린 리더 모드는 터미널 인터페이스만 조정하므로 VS Code 확장 프로그램의 채팅 패널에서는 필요하지 않습니다. Claude Code v2.1.236 이상에서는 확장 프로그램이 [설정 없이 스크린 리더에 대화 활동을 알립니다](/docs/ko/vs-code#use-a-screen-reader).

<h2 id="turn-on-screen-reader-mode">
  스크린 리더 모드 켜기
</h2>

스크린 리더를 사용하는 빈도와 일치하는 방법을 선택하십시오:

* 한 세션의 경우: `claude --ax-screen-reader`를 실행합니다.
* 한 셸에서 시작된 세션의 경우: `CLAUDE_AX_SCREEN_READER` 환경 변수를 `1`로 설정합니다. Bash 또는 Zsh에서는 `export CLAUDE_AX_SCREEN_READER=1`을 실행하고, PowerShell에서는 `$env:CLAUDE_AX_SCREEN_READER = "1"`을 실행합니다. 향후 셸을 위해 유지하려면 셸 프로필에 해당 줄을 추가합니다.
* 머신의 모든 세션의 경우: 사용자 [설정 파일](/docs/ko/settings)에 `"axScreenReader": true`를 추가합니다. 이 설정은 VS Code 통합 터미널을 포함한 모든 터미널에 적용됩니다.

메서드를 결합하는 경우 Claude Code는 [`--ax-screen-reader`](/docs/ko/cli-reference#cli-flags) 플래그를 [`CLAUDE_AX_SCREEN_READER`](/docs/ko/env-vars#variables) 환경 변수보다 우선 적용하고, 환경 변수를 [`axScreenReader`](/docs/ko/settings-reference#axscreenreader) 설정보다 우선 적용합니다.

SSH를 통해 Claude Code를 사용하는 경우 Claude Code가 실행되는 원격 머신에서 환경 변수 또는 설정을 설정합니다.

Claude Code가 인쇄하는 첫 번째 줄은 모드를 확인합니다: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]` 또는 `[Screen Reader Mode: on via settings]`.

<h2 id="turn-off-screen-reader-mode">
  스크린 리더 모드 끄기
</h2>

모드를 켠 메서드를 역으로 수행합니다: 플래그 없이 시작하거나, 환경 변수를 설정 해제하거나, `axScreenReader`를 `false`로 설정합니다. `CLAUDE_AX_SCREEN_READER`를 `0`으로 설정하면 설정이 `true`일 때도 Claude Code가 모드를 꺼진 상태로 유지합니다.

<h2 id="accessibility-settings">
  접근성 설정
</h2>

다음 표는 각 접근성 옵션, 플래그, 환경 변수 또는 설정으로 설정하는 방법, 그리고 변경되는 사항을 나열합니다.

| 옵션                                                                      | 유형    | 변경되는 사항                                                                                                                                                    |
| :---------------------------------------------------------------------- | :---- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`--ax-screen-reader`](/docs/ko/cli-reference#cli-flags)                     | 플래그   | 한 세션에 대한 스크린 리더 모드입니다.                                                                                                                                     |
| [`CLAUDE_AX_SCREEN_READER`](/docs/ko/env-vars#variables)                     | 환경 변수 | 설정한 셸에서 시작된 세션에 대한 스크린 리더 모드입니다.                                                                                                                           |
| [`axScreenReader`](/docs/ko/settings-reference#axscreenreader)               | 설정    | `true`일 때 모든 세션에 대한 스크린 리더 모드입니다.                                                                                                                          |
| [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/ko/env-vars#variables)                  | 환경 변수 | Claude Code가 확인 줄 이후 스크린 리더 모드에서 첫 번째 프롬프트를 그리기 전에 대기하는 시간입니다. Claude Code v2.1.217 이상이 필요합니다.                                                             |
| [`CLAUDE_AX_PREPARK_MS`](/docs/ko/env-vars#variables)                        | 환경 변수 | Claude Code가 줄의 시작 부분에 커서를 두고 스크린 리더 모드에서 새로운 또는 변경된 줄을 작성하기 전에 대기하는 시간입니다. Claude Code v2.1.233 이상이 필요합니다.                                                |
| [`CLAUDE_CODE_ACCESSIBILITY`](/docs/ko/env-vars#variables)                   | 환경 변수 | `1`로 설정할 때 macOS Zoom과 같은 스크린 확대기에 대해 표시 상태를 유지하는 터미널 커서입니다. 커서는 입력 캐럿을 따르고, Claude Code v2.1.218 이상에서는 `/config` 및 `/plugin`과 같은 메뉴 및 패널의 강조 표시된 행을 따릅니다. |
| [`prefersReducedMotion`](/docs/ko/settings-reference#prefersreducedmotion)   | 설정    | `true`일 때 스피너, 반짝임 및 기타 애니메이션이 감소하거나 없습니다.                                                                                                                 |
| [`theme`](/docs/ko/settings-reference#theme)                                 | 설정    | 색맹 친화적인 `dark-daltonized` 및 `light-daltonized` 테마를 포함한 인터페이스 색상입니다. [`/theme`](/docs/ko/commands#all-commands)을 사용하여 하나를 선택할 수도 있습니다.                           |
| [`preferredNotifChannel`](/docs/ko/settings-reference#preferrednotifchannel) | 설정    | `"terminal_bell"` 값으로 Claude가 사용자를 기다릴 때 스크린 리더 모드 외부의 터미널 벨입니다.                                                                                           |

<h2 id="what-your-screen-reader-hears">
  스크린 리더가 듣는 내용
</h2>

스크린 리더 모드에서 Claude Code는 평문을 작성합니다:

* 인터페이스 크롬을 위한 상자 그리기 문자 없음
* 색상만으로 표시되는 신호 없음
* 변경되지 않은 콘텐츠의 다시 그리기 없음. 진행 상황 스피너는 정적 텍스트로 렌더링됨
* Claude의 답변의 표는 상자 문자 그리드 대신 `Header: value` 문장으로 읽힙니다

Claude Code는 터미널의 스크롤백에 인쇄하는 모든 것을 남겨두므로 스크린 리더의 검토 명령이나 터미널의 검색을 사용하여 이전 턴을 다시 읽을 수 있습니다. Claude Code는 스크린 리더 모드에서 [`tui` 설정](/docs/ko/settings-reference#tui)을 무시합니다. [알려진 제한 사항](#known-limitations)에 나열된 연결된 백그라운드 세션을 제외하고, [전체 화면 렌더링](/docs/ko/fullscreen) 대신 스크롤링 텍스트를 인쇄합니다.

Claude Code는 또한 스크린 리더가 따라갈 수 있도록 두 지점에서 대기합니다:

* Claude Code가 확인 줄을 인쇄한 후 스크린 리더가 줄을 완료할 수 있도록 프롬프트를 그리기 전에 3초 동안 대기합니다. 아무 키나 눌러 대기를 종료합니다. 대기 길이를 변경하려면 [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/ko/env-vars#variables)를 설정합니다.
* Claude Code가 힌트나 Claude의 답변 중 더 많은 부분과 같은 새로운 또는 변경된 줄을 작성하기 전에 커서를 줄의 시작으로 이동하고 50밀리초 동안 대기합니다. 스크린 리더는 첫 번째 문자부터 줄을 읽습니다. 입력 줄의 끝에 입력하거나 삭제한 문자는 즉시 나타납니다. 대기 길이를 변경하려면 [`CLAUDE_AX_PREPARK_MS`](/docs/ko/env-vars#variables)를 설정합니다.

트랜스크립트의 각 메시지는 스크린 리더가 발표하는 레이블로 시작하며, 이것이 무엇인지 이름을 지정합니다: 사용자의 메시지, Claude의 답변 및 생각, 도구 활동, 오류 및 경고, 그리고 프롬프트. 레이블은 또한 검색 가능하므로 터미널의 스크롤백을 검색하여 트랜스크립트의 섹션 간에 이동할 수 있습니다:

| 레이블                    | 의미                                                         |
| :--------------------- | :--------------------------------------------------------- |
| `you:`                 | 사용자의 메시지                                                   |
| `claude:`              | Claude의 답변                                                 |
| `thinking:`            | Claude의 생각                                                 |
| `tool:`                | 파일 편집이나 명령 실행과 같은 도구 활동                                    |
| `tool error:`          | 실패한 도구                                                     |
| `error:`               | 실패한 API 요청과 같은 대화의 오류                                      |
| `warning:`             | 폴백 모델로의 전환과 같은 Claude Code의 경고                             |
| `Permission Required:` | 답변을 기다리는 권한 프롬프트                                           |
| `Cost:`                | Claude Code가 종료될 때의 세션 비용 요약(계정이 [비용을 표시](/docs/ko/costs)하는 경우) |

Claude Code는 터미널 커서를 입력 캐럿에 유지하므로 스크린 리더의 현재 줄 읽기 명령은 편집 중인 프롬프트를 읽습니다.

입력 줄의 끝에 입력할 때, 또는 거기서 `Backspace`를 누를 때, Claude Code는 변경된 문자만 작성합니다. 스크린 리더는 해당 문자만 에코합니다.

[텍스트 편집 바로 가기](/docs/ko/interactive-mode#text-editing) 중 하나로 단어나 줄을 삭제할 때, Claude Code는 삭제된 텍스트를 발표합니다:

* `Ctrl+W` 또는 `Alt+D`로 단어 삭제, 또는 macOS에서 `Option+Delete` 또는 Windows에서 `Ctrl+Backspace`로 삭제
* `Ctrl+U` 또는 `Cmd+Backspace`로 줄의 시작까지 삭제
* `Ctrl+K`로 줄의 끝까지 삭제

[권한 모드](/docs/ko/permission-modes)를 `Shift+Tab`으로 순환할 때, Claude Code는 도착한 권한 모드를 발표합니다(예: `[plan mode on]` 또는 `[accept edits on]`). Claude Code는 발표를 한 번 인쇄하고 나중의 다시 그리기에서 반복하지 않습니다.

<h3 id="jump-between-turns">
  턴 간에 이동
</h3>

Claude Code는 턴 경계에서 OSC 133 셸 통합 마커를 내보내므로 터미널의 이전 프롬프트로 이동 키는 전체 트랜스크립트를 읽지 않고 턴 간에 이동합니다:

* iTerm2: Cmd+Shift+Up
* VS Code 터미널: Windows에서 Ctrl+Up, macOS에서 Cmd+Up
* Windows Terminal: 기본적으로 키 없음; 설정에서 `scrollToMark` 작업을 바인딩합니다
* Kitty 및 Ghostty: 터미널의 설명서에서 프롬프트로 이동 키를 확인합니다

macOS Terminal은 마커에 작용하지 않으며, Claude Code는 WezTerm에서 마커를 내보내지 않습니다. 이러한 터미널에서는 대신 스크롤백에서 `you:` 레이블을 검색합니다.

<h2 id="answer-menus-and-prompts">
  메뉴 및 프롬프트에 답변하기
</h2>

스크린 리더 모드에서 일반적으로 화살표 키로 탐색하는 메뉴(권한 프롬프트 포함)는 번호가 매겨진 목록이 됩니다. Claude Code는 각 옵션을 번호가 매겨진 줄로 알리고, 유효한 범위를 나타내는 `Enter selection` 프롬프트를 알립니다. 원하는 옵션의 번호를 입력하고 Enter를 누릅니다.

* Escape를 눌러 프롬프트가 `or Escape to cancel`로 끝나는 메뉴를 취소합니다.
* 목록에 없는 번호를 입력하면 Claude Code는 유효한 범위를 알리고 다시 시도할 수 있게 합니다.

스크린 리더 모드 외부에서는 슬라이더인 [`/effort`](/docs/ko/model-config#adjust-effort-level) 선택기가 동일한 종류의 번호가 매겨진 목록이 됩니다.

예/아니오 프롬프트는 두 가지 옵션 메뉴 대신 입력된 답변을 요청합니다. `y` 또는 `n`을 입력하고 Enter를 누릅니다. `yes`와 `no`도 작동합니다.

<h2 id="hear-when-claude-code-needs-you">
  Claude Code가 당신의 주의가 필요할 때 알림
</h2>

스크린 리더 모드에서 Claude Code는 터미널 벨을 울려서 당신의 주의가 필요함을 알리므로, 계속해서 대화 기록을 확인할 필요가 없습니다. 벨은 다음과 같은 경우에 울립니다:

* Claude가 답변을 완료했을 때
* 권한 프롬프트와 같이 프롬프트 또는 대화 상자에서 당신의 답변이 필요할 때
* 5초 이상 실행된 도구가 완료되었을 때

벨은 터미널의 표준 경고음입니다. 이를 끄려면 터미널 애플리케이션의 벨 설정을 변경하세요. 스크린 리더 모드 외부에서는 [`preferredNotifChannel`](/docs/ko/settings-reference#preferrednotifchannel)을 `"terminal_bell"`로 설정하여 Claude가 당신의 응답을 기다리고 있을 때 [유사한 벨](/docs/ko/terminal-config#get-a-terminal-bell-or-notification)을 받을 수 있습니다.

<h2 id="known-limitations">
  알려진 제한 사항
</h2>

일부 동작은 스크린 리더 모드에 맞게 조정되지 않습니다:

* 스크린 리더 모드는 스크린 리더가 실행 중일 때 자동으로 켜지지 않습니다.
* Claude Code는 `Shift+Tab`으로 순환하는 것 이외의 방식으로 변경된 권한 모드를 발표하지 않습니다. 예를 들어 명령에서 [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)로 진입하는 경우입니다.
* `claude attach` 또는 에이전트 보기에서 [백그라운드 세션](/docs/ko/agent-view)에 첨부하면 기본 스크롤백이 없는 터미널의 대체 화면으로 들어갑니다. 이는 [다른 첨부된 세션과 동일한 동작](/docs/ko/fullscreen)입니다. 나가려면 빈 프롬프트에서 왼쪽 화살표를 누르거나 대화 상자에 포커스가 있으면 Ctrl+Z를 누릅니다.
* Claude Code는 종료 시 인쇄하는 요약에서 비용을 발표하며, 턴당이 아닙니다.
* 스크린 리더 모드는 `-p` 플래그로 [비대화형 모드](/docs/ko/headless)를 변경하지 않습니다. 비대화형 모드는 이미 평문을 작성하며 스크립팅을 위한 대안으로 남아 있습니다.

<h2 id="report-an-issue">
  문제 보고
</h2>

스크린 리더, 확대기 또는 터미널에서 작동하지 않는 경우 [Claude Code 이슈 추적기](https://github.com/anthropics/claude-code/issues)에서 이슈를 열고 제목에 보조 기술을 언급합니다. 보고서에 운영 체제, 터미널 애플리케이션, 보조 기술 이름 및 버전을 포함합니다.
