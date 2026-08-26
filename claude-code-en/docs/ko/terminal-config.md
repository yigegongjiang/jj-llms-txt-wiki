> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code를 위한 터미널 구성

> Shift+Enter로 줄 바꿈 수정, Claude 완료 시 터미널 벨 설정, tmux 구성, 색상 테마 일치, Claude Code CLI에서 Vim 모드 활성화합니다.

Claude Code는 구성 없이 모든 터미널에서 작동합니다. 이 페이지는 특정 항목이 예상대로 작동하지 않을 때를 위한 것입니다. 아래에서 증상을 찾아보세요. 이미 모든 것이 올바르게 느껴진다면 이 페이지가 필요하지 않습니다.

* [Shift+Enter가 줄 바꿈 대신 제출함](#enter-multiline-prompts)
* [macOS에서 Option 키 단축키가 작동하지 않음](#enable-option-key-shortcuts-on-macos)
* [Claude 완료 시 소리 또는 알림 없음](#get-a-terminal-bell-or-notification)
* [tmux 내에서 Claude Code 실행](#configure-tmux)
* [디스플레이 깜박임 또는 스크롤백 점프](#switch-to-fullscreen-rendering)
* [프롬프트에서 Vim 키 사용](#edit-prompts-with-vim-keybindings)

이 페이지는 터미널이 Claude Code에 올바른 신호를 보내도록 하는 것에 관한 것입니다. Claude Code 자체가 응답하는 키를 변경하려면 대신 [키 바인딩](/docs/ko/keybindings)을 참조하세요.

<h2 id="enter-multiline-prompts">
  여러 줄 프롬프트 입력
</h2>

Enter를 누르면 메시지가 제출됩니다. 제출하지 않고 줄 바꿈을 추가하려면 Ctrl+J를 누르거나 `\`를 입력한 후 Enter를 누르세요. 둘 다 구성 없이 모든 터미널에서 작동합니다.

대부분의 터미널에서 Shift+Enter를 누를 수도 있지만 터미널 에뮬레이터에 따라 지원이 다릅니다:

| 터미널                                                                     | 줄 바꿈을 위한 Shift+Enter             |
| :---------------------------------------------------------------------- | :------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal | 구성 없이 작동                         |
| VS Code, Cursor, Devin Desktop, Alacritty, Zed                          | 한 번 `/terminal-setup` 실행         |
| gnome-terminal, PyCharm 및 Android Studio와 같은 JetBrains IDE              | 사용 불가; Ctrl+J 또는 `\` 다음 Enter 사용 |

VS Code, Cursor, Devin Desktop, Alacritty 및 Zed의 경우 `/terminal-setup`은 Shift+Enter 및 기타 키 바인딩을 터미널의 구성 파일에 씁니다. 기존 바인딩은 그대로 유지됩니다. `VSCode terminal Shift+Enter key binding already configured`와 같은 메시지가 표시되면 변경이 이루어지지 않았습니다. `/terminal-setup`을 tmux 또는 screen 내부가 아닌 호스트 터미널에서 직접 실행하세요. 호스트 터미널의 구성에 써야 하기 때문입니다.

VS Code, Cursor 및 Devin Desktop에서 `/terminal-setup`은 또한 두 가지 편집기 설정을 업데이트합니다. 통합 터미널에서 텍스트가 깨지는 것을 방지하기 위해 `terminal.integrated.gpuAcceleration`을 `"off"`로 설정하고, [전체 화면 모드](/docs/ko/fullscreen)에서 더 부드러운 스크롤링을 위해 `terminal.integrated.mouseWheelScrollSensitivity`를 설정합니다. GPU 가속 변경을 취소하려면 `"auto"`로 다시 설정하고 편집기 창을 다시 로드하세요.

tmux 내에서 실행 중인 경우 외부 터미널이 지원하더라도 Shift+Enter는 아래의 [tmux 구성](#configure-tmux)도 필요합니다.

줄 바꿈을 다른 키에 바인딩하거나 Enter가 줄 바꿈을 삽입하고 Shift+Enter가 제출하도록 동작을 바꾸려면 [키 바인딩 파일](/docs/ko/keybindings)에서 `chat:newline` 및 `chat:submit` 작업을 매핑하세요.

<h2 id="enable-option-key-shortcuts-on-macos">
  macOS에서 Option 키 단축키 활성화
</h2>

일부 Claude Code 단축키는 Option 키를 사용합니다. 예를 들어 줄 바꿈을 위한 Option+Enter 또는 모델을 전환하기 위한 Option+P입니다. macOS에서 대부분의 터미널은 기본적으로 Option을 수정자로 보내지 않으므로 이를 활성화할 때까지 이러한 단축키는 작동하지 않습니다. 터미널 설정은 일반적으로 "Option을 Meta 키로 사용"으로 표시됩니다. Meta는 현재 Option 또는 Alt로 표시된 키의 역사적 Unix 이름입니다.

<Tabs>
  <Tab title="Apple Terminal">
    설정 → 프로필 → 키보드를 열고 "Option을 Meta 키로 사용"을 확인하세요.

    Claude Code의 첫 실행 프롬프트에서 "줄 바꿈 및 시각적 벨을 위한 Option+Enter"를 제공하는 것을 수락했다면 이미 완료되었습니다. 해당 프롬프트는 `/terminal-setup`을 실행하여 Option을 Meta로 활성화하고 Apple Terminal 프로필에서 오디오 벨을 시각적 화면 깜박임으로 전환합니다.
  </Tab>

  <Tab title="iTerm2">
    설정 → 프로필 → 키 → 일반을 열고 왼쪽 Option 키와 오른쪽 Option 키를 "Esc+"로 설정하세요.

    iTerm2에서 `/terminal-setup`을 실행하면 설정 → 일반 → 선택 아래에서 "터미널의 애플리케이션이 클립보드에 액세스할 수 있음"을 활성화하여 `/copy` 명령이 시스템 클립보드에 쓸 수 있도록 합니다. 이 명령은 tmux 내부에서 실행되는 경우에도 iTerm2를 감지합니다. 변경 사항을 적용하려면 iTerm2를 다시 시작하세요.
  </Tab>

  <Tab title="VS Code">
    VS Code 설정에 `"terminal.integrated.macOptionIsMeta": true`를 추가하세요.
  </Tab>
</Tabs>

Ghostty, Kitty 및 기타 터미널의 경우 터미널의 구성 파일에서 Option-as-Alt 또는 Option-as-Meta 설정을 찾으세요.

<h2 id="get-a-terminal-bell-or-notification">
  터미널 벨 또는 알림 받기
</h2>

Claude가 작업을 완료하거나 권한 프롬프트를 위해 일시 중지하면 알림 이벤트를 발생시킵니다. 이를 터미널 벨 또는 데스크톱 알림으로 표시하면 긴 작업이 실행되는 동안 다른 작업으로 전환할 수 있습니다.

기본적으로 Claude Code는 Ghostty, Kitty 및 iTerm2에서만 데스크톱 알림을 보냅니다. 다른 터미널에서는 [`preferredNotifChannel`](/docs/ko/settings#available-settings)을 `"terminal_bell"`로 설정하여 대신 터미널 벨을 울리거나, 사용자 정의 소리나 명령을 위해 [알림 훅](#play-a-sound-with-a-notification-hook)을 구성하세요.

데스크톱 알림은 SSH를 통해 로컬 머신에 도달하므로 원격 세션도 여전히 알림을 받을 수 있습니다. Ghostty와 Kitty는 추가 설정 없이 OS 알림 센터로 전달합니다. iTerm2는 전달을 활성화해야 합니다:

<Steps>
  <Step title="iTerm2 알림 설정 열기">
    설정 → 프로필 → 터미널로 이동하세요.
  </Step>

  <Step title="알림 활성화">
    "Notification Center Alerts"를 확인한 후 "Filter Alerts"를 클릭하고 "Send escape sequence-generated alerts"를 활성화하세요.
  </Step>
</Steps>

알림이 여전히 나타나지 않으면 터미널 애플리케이션이 OS 설정에서 알림 권한을 가지고 있는지 확인하고, tmux 내에서 실행 중인 경우 [통과를 활성화](#configure-tmux)하세요.

<h3 id="play-a-sound-with-a-notification-hook">
  알림 훅으로 소리 재생
</h3>

모든 터미널에서 Claude가 주의가 필요할 때 소리를 재생하거나 사용자 정의 명령을 실행하는 [알림 훅](/docs/ko/hooks-guide#get-notified-when-claude-needs-input)을 구성할 수 있습니다. 훅은 기본 제공 알림과 함께 실행되며 이를 대체하지 않으므로, Warp 또는 VS Code 통합 터미널과 같이 데스크톱 알림을 받지 않는 터미널은 훅을 사용하거나 `preferredNotifChannel`을 `"terminal_bell"`로 설정할 수 있습니다.

아래 예제는 macOS에서 시스템 소리를 재생합니다. 연결된 가이드에는 macOS, Linux 및 Windows용 데스크톱 알림 명령이 있습니다.

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  tmux 구성
</h2>

Claude Code가 tmux 내에서 실행될 때 기본적으로 두 가지가 손상됩니다: Shift+Enter가 줄 바꿈을 삽입하는 대신 제출하고, 데스크톱 알림 및 [진행률 표시줄](/docs/ko/settings#available-settings)이 외부 터미널에 도달하지 않습니다. `~/.tmux.conf`에 이 줄을 추가한 후 `tmux source-file ~/.tmux.conf`를 실행하여 실행 중인 서버에 적용하세요:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

`allow-passthrough` 줄은 알림 및 진행률 업데이트가 tmux에 의해 삼켜지는 대신 외부 터미널에 도달하도록 합니다. `extended-keys` 줄은 tmux가 Shift+Enter를 일반 Enter와 구별하도록 하여 줄 바꿈 단축키가 작동하도록 합니다.

<h2 id="match-the-color-theme">
  색상 테마 일치
</h2>

`/theme` 명령을 사용하거나 `/config`의 테마 선택기를 사용하여 터미널과 일치하는 Claude Code 테마를 선택합니다. 자동 옵션을 선택하면 터미널의 밝은 또는 어두운 배경을 감지하므로 테마는 터미널이 할 때마다 OS 모양 변경을 따릅니다. Claude Code는 터미널 애플리케이션에서 설정하는 터미널의 자체 색상 구성표를 제어하지 않습니다.

인터페이스 하단에 표시되는 내용을 사용자 정의하려면 현재 모델, 작업 디렉토리, git 분기 또는 기타 컨텍스트를 표시하는 [사용자 정의 상태 줄](/docs/ko/statusline)을 구성합니다.

<h3 id="create-a-custom-theme">
  사용자 정의 테마 만들기
</h3>

<Note>
  사용자 정의 테마는 Claude Code v2.1.118 이상이 필요합니다.
</Note>

기본 제공 사전 설정 외에도 `/theme`는 정의한 모든 사용자 정의 테마와 설치된 [플러그인](/docs/ko/plugins-reference#themes)에서 제공한 모든 테마를 나열합니다. 목록 끝에서 \*\*새 사용자 정의 테마…\*\*를 선택하여 대화형으로 만들 수 있습니다. 테마 이름을 지정한 다음 개별 색상 토큰을 선택하여 재정의합니다. 사용자 정의 테마가 강조 표시되어 있는 동안 `Ctrl+E`를 눌러 편집합니다.

각 사용자 정의 테마는 `~/.claude/themes/`의 JSON 파일입니다. `.json` 확장자를 제외한 파일 이름이 테마의 슬러그이며, 테마를 선택하면 `custom:<slug>`이 테마 기본 설정으로 저장됩니다. 파일에는 세 가지 선택적 필드가 있습니다.

| 필드          | 유형     | 설명                                                                                                                               |
| :---------- | :----- | :------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | string | `/theme`에 표시되는 표시 레이블입니다. 파일 이름 슬러그로 기본 설정됩니다.                                                                                   |
| `base`      | string | 테마가 시작되는 기본 제공 사전 설정입니다: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, 또는 `light-ansi`. `dark`로 기본 설정됩니다. |
| `overrides` | object | 색상 토큰 이름을 색상 값으로 매핑합니다. 여기에 나열되지 않은 토큰은 기본 사전 설정으로 통과합니다.                                                                        |

색상 값은 `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, 또는 `ansi:<name>`을 허용합니다. 여기서 `<name>`은 `red` 또는 `cyanBright`와 같은 16가지 표준 ANSI 색상 이름 중 하나입니다. 알 수 없는 토큰과 잘못된 색상 값은 무시되므로 오타가 렌더링을 손상시킬 수 없습니다.

다음 예제는 어두운 사전 설정을 유지하지만 프롬프트 악센트, 오류 텍스트 및 성공 텍스트를 다시 칠하는 테마를 정의합니다.

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code는 `~/.claude/themes/`를 감시하고 파일이 변경되면 다시 로드하므로 편집기에서 만든 편집 사항이 다시 시작하지 않고도 실행 중인 세션에 적용됩니다.

아래는 `overrides`에서 설정할 수 있는 토큰을 다룹니다. `/theme`의 대화형 편집기는 여기에서 다루지 않은 몇 가지 단일 목적 악센트(예: 온보딩 화면 색상)를 포함한 라이브 미리보기와 함께 동일한 토큰을 표시합니다.

<Accordion title="색상 토큰 참조">
  다음 예제는 여러 그룹의 토큰을 결합합니다: 브랜드 악센트, 계획 모드 테두리, diff 배경 및 전체 화면 메시지 배경입니다.

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    텍스트 및 악센트 색상
  </h4>

  기본 브랜드 악센트와 인터페이스 전체에서 사용되는 전경 텍스트 음영을 제어합니다.

  | 토큰            | 제어 대상                            |
  | :------------ | :------------------------------- |
  | `claude`      | 기본 브랜드 악센트, 스피너 및 어시스턴트 레이블에 사용됨 |
  | `text`        | 기본 전경 텍스트                        |
  | `inverseText` | 상태 배지와 같은 색상 배경 위에 그려진 텍스트       |
  | `inactive`    | 힌트, 타임스탬프 및 비활성화된 항목과 같은 보조 텍스트  |
  | `subtle`      | 희미한 테두리 및 강조 해제된 보조 텍스트          |
  | `suggestion`  | 자동 완성 제안 및 선택기의 선택 강조            |
  | `permission`  | 권한 프롬프트 및 선택기를 포함한 대화 상자 테두리     |
  | `remember`    | 메모리 및 `CLAUDE.md` 표시기            |

  <h4 id="status-colors">
    상태 색상
  </h4>

  메시지 및 표시기 전체에서 성공, 실패 및 경고 상태를 신호합니다.

  | 토큰        | 제어 대상                  |
  | :-------- | :--------------------- |
  | `success` | 성공 메시지 및 통과한 검사        |
  | `error`   | 오류 메시지 및 실패            |
  | `warning` | 경고, 주의 메시지 및 자동 모드 테두리 |
  | `merged`  | 병합된 풀 요청 상태            |

  <h4 id="input-box-and-mode-indicators">
    입력 상자 및 모드 표시기
  </h4>

  입력 상자 테두리 색상과 권한 모드 또는 표시기가 활성화되어 있는 동안 표시되는 악센트를 설정합니다.

  | 토큰             | 제어 대상                     |
  | :------------- | :------------------------ |
  | `promptBorder` | 기본 권한 모드의 입력 상자 테두리       |
  | `planMode`     | 계획 모드 악센트 및 테두리           |
  | `autoAccept`   | 수락-편집 모드 악센트 및 테두리        |
  | `bashBorder`   | `!` 셸 명령을 입력할 때 입력 상자 테두리 |
  | `ide`          | IDE 연결 표시기                |
  | `fastMode`     | 빠른 모드 표시기                 |

  <h4 id="diff-rendering">
    Diff 렌더링
  </h4>

  파일 편집 및 검토에서 추가되고 제거된 코드를 색칠합니다.

  | 토큰                  | 제어 대상                     |
  | :------------------ | :------------------------ |
  | `diffAdded`         | 추가된 줄의 배경                 |
  | `diffRemoved`       | 제거된 줄의 배경                 |
  | `diffAddedDimmed`   | 추가된 줄 근처의 변경되지 않은 컨텍스트 배경 |
  | `diffRemovedDimmed` | 제거된 줄 근처의 변경되지 않은 컨텍스트 배경 |
  | `diffAddedWord`     | 추가된 줄 내의 단어 수준 강조         |
  | `diffRemovedWord`   | 제거된 줄 내의 단어 수준 강조         |

  <h4 id="fullscreen-mode">
    전체 화면 모드
  </h4>

  메시지가 배경 채우기를 갖는 [전체 화면 렌더링 모드](/docs/ko/fullscreen)에서만 적용됩니다.

  | 토큰                           | 제어 대상                         |
  | :--------------------------- | :---------------------------- |
  | `userMessageBackground`      | 트랜스크립트의 메시지 뒤의 배경             |
  | `userMessageBackgroundHover` | 마우스를 올리거나 확장할 때 메시지 뒤의 배경     |
  | `messageActionsBackground`   | 작업 표시줄이 열려 있을 때 선택된 메시지 뒤의 배경 |
  | `bashMessageBackgroundColor` | 트랜스크립트의 `!` 셸 명령 항목 뒤의 배경     |
  | `memoryBackgroundColor`      | 트랜스크립트의 `#` 메모리 항목 뒤의 배경      |
  | `selectionBg`                | 마우스로 선택한 텍스트의 배경              |

  <h4 id="usage-meter-and-speaker-labels">
    사용량 미터 및 스피커 레이블
  </h4>

  `/usage` 보기에 표시되는 막대와 메시지를 Claude의 메시지와 구별하는 레이블을 조정합니다.

  | 토큰                 | 제어 대상                      |
  | :----------------- | :------------------------- |
  | `rate_limit_fill`  | 사용량 미터의 채워진 부분             |
  | `rate_limit_empty` | 사용량 미터의 채워지지 않은 부분         |
  | `briefLabelYou`    | 메시지의 `You` 레이블 색상          |
  | `briefLabelClaude` | 어시스턴트 메시지의 `Claude` 레이블 색상 |

  <h4 id="shimmer-variants-and-subagent-colors">
    반짝임 변형 및 서브에이전트 색상
  </h4>

  여러 토큰에는 쌍을 이루는 반짝임 변형이 있으며, 이는 스피너의 애니메이션 그래디언트에서 사용되는 더 밝은 색상을 제공합니다. 애니메이션이 일치하지 않는 것처럼 보이면 기본 토큰과 함께 반짝임을 재정의합니다.

  * `claude` 및 `claudeShimmer`
  * `warning` 및 `warningShimmer`
  * `permission` 및 `permissionShimmer`
  * `promptBorder` 및 `promptBorderShimmer`
  * `inactive` 및 `inactiveShimmer`
  * `fastMode` 및 `fastModeShimmer`

  각 [서브에이전트](/docs/ko/sub-agents) 및 병렬 작업은 8개의 명명된 색상 중 하나로 표시되므로 트랜스크립트에서 구별할 수 있습니다. 토큰 이름은 `<color>_FOR_SUBAGENTS_ONLY` 패턴을 따릅니다. 여기서 `<color>`는 `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` 또는 `cyan`입니다. 이를 재정의하여 각 명명된 색상의 모양을 변경합니다. 예를 들어, 정의에서 `color: blue`를 가진 서브에이전트는 `blue_FOR_SUBAGENTS_ONLY` 값을 사용하여 그려집니다.

  [`ultrathink`](/docs/ko/model-config#use-ultrathink-for-one-off-deep-reasoning) 및 [`ultraplan`](/docs/ko/ultraplan) 키워드는 프롬프트 입력에서 7색 무지개 그래디언트로 렌더링됩니다. 토큰 이름은 `rainbow_<color>` 및 `rainbow_<color>_shimmer` 패턴을 따릅니다. 여기서 `<color>`는 `red`, `orange`, `yellow`, `green`, `blue`, `indigo` 또는 `violet`입니다.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  전체 화면 렌더링으로 전환
</h2>

디스플레이가 깜박이거나 Claude가 작업 중일 때 스크롤 위치가 점프하면 [전체 화면 렌더링 모드](/docs/ko/fullscreen)로 전환하세요. 터미널이 일반 스크롤백에 추가하는 대신 전체 화면 앱용으로 예약한 별도의 화면에 그려서 메모리 사용량을 평탄하게 유지하고 스크롤 및 선택을 위한 마우스 지원을 추가합니다. 이 모드에서는 터미널의 기본 스크롤백이 아닌 마우스 또는 PageUp으로 Claude Code 내에서 스크롤합니다. 검색 및 복사 방법은 [전체 화면 페이지](/docs/ko/fullscreen#search-and-review-the-conversation)를 참조하세요.

`/tui fullscreen`을 실행하여 전환하고 기본값을 저장하세요. 대화가 그대로 다시 시작되며 향후 세션은 전체 화면에서 시작됩니다. Claude Code를 시작하기 전에 `CLAUDE_CODE_NO_FLICKER` 환경 변수를 설정할 수도 있습니다:

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  큰 콘텐츠 붙여넣기
</h2>

프롬프트에 10,000자 이상을 붙여넣으면 Claude Code는 입력을 `[Pasted text]` 자리 표시자로 축소하여 입력 상자를 사용 가능하게 유지합니다. 전체 콘텐츠는 제출할 때 여전히 Claude에 전송됩니다.

VS Code 통합 터미널은 매우 큰 붙여넣기에서 Claude Code에 도달하기 전에 문자를 삭제할 수 있으므로 거기서 파일 기반 워크플로우를 선호하세요. 전체 파일 또는 긴 로그와 같은 매우 큰 입력의 경우 콘텐츠를 파일에 작성하고 붙여넣는 대신 Claude에 읽도록 요청하세요. 이렇게 하면 대화 기록을 읽을 수 있게 유지하고 Claude가 나중에 경로로 파일을 참조할 수 있습니다.

<h2 id="edit-prompts-with-vim-keybindings">
  Vim 키 바인딩으로 프롬프트 편집
</h2>

Claude Code는 프롬프트 입력을 위한 Vim 스타일 편집 모드를 포함합니다. `/config` → 편집기 모드를 통해 활성화하거나 [`editorMode`](/docs/ko/settings#available-settings)를 `~/.claude/settings.json`에서 `"vim"`으로 설정하여 활성화하세요. 편집기 모드를 `normal`로 다시 설정하여 끄세요.

Vim 모드는 `hjkl` 네비게이션, `v`/`V` 선택, 텍스트 객체를 사용한 `d`/`c`/`y`와 같은 NORMAL 모드 및 VISUAL 모드 모션과 연산자의 부분 집합을 지원합니다. 전체 키 테이블은 [Vim 편집기 모드 참조](/docs/ko/interactive-mode#vim-editor-mode)를 참조하세요. Vim 모션은 키 바인딩 파일을 통해 다시 매핑할 수 없습니다.

INSERT 모드에서 Enter를 누르면 표준 Vim과 달리 프롬프트가 여전히 제출됩니다. 대신 NORMAL 모드에서 `o` 또는 `O`를 사용하거나 Ctrl+J를 사용하여 줄 바꿈을 삽입하세요.

<h2 id="related-resources">
  관련 리소스
</h2>

* [대화형 모드](/docs/ko/interactive-mode): 전체 키보드 단축키 참조 및 Vim 키 테이블
* [키 바인딩](/docs/ko/keybindings): Enter 및 Shift+Enter를 포함한 모든 Claude Code 단축키 다시 매핑
* [전체 화면 렌더링](/docs/ko/fullscreen): 전체 화면 모드에서 스크롤, 검색 및 복사에 대한 세부 정보
* [훅 가이드](/docs/ko/hooks-guide): Linux 및 Windows용 더 많은 알림 훅 예제
* [문제 해결](/docs/ko/troubleshooting): 터미널 구성 외부의 문제에 대한 수정 사항
