> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 전체 화면 렌더링

> 마우스 지원과 안정적인 메모리 사용으로 더 부드럽고 깜빡임 없는 렌더링 모드를 활성화합니다.

<Note>
  전체 화면 렌더링은 [연구 미리보기](#research-preview)입니다. [전체 화면에서 시작할지 또는 클래식 렌더러에서 시작할지](#fullscreen-by-default) 여부는 설정에 따라 달라집니다. 현재 대화에서 `/tui fullscreen` 또는 `/tui default`를 실행하여 전환합니다. 피드백에 따라 동작이 변경될 수 있습니다.
</Note>

전체 화면 렌더링은 Claude Code CLI의 대체 렌더링 경로로, 깜빡임을 제거하고 긴 대화에서 메모리 사용량을 일정하게 유지하며 마우스 지원을 추가합니다. `vim` 또는 `htop`처럼 터미널의 대체 화면 버퍼에 인터페이스를 그리고 현재 표시되는 메시지만 렌더링합니다. 이렇게 하면 각 업데이트 시 터미널로 전송되는 데이터의 양이 줄어듭니다.

차이는 VS Code 통합 터미널, tmux, iTerm2와 같이 렌더링 처리량이 병목인 터미널 에뮬레이터에서 가장 두드러집니다. Claude가 작업 중일 때 터미널 스크롤 위치가 맨 위로 점프하거나 도구 출력이 스트리밍될 때 화면이 깜빡인다면, 이 모드가 이를 해결합니다.

<Note>
  전체 화면이라는 용어는 Claude Code가 `vim`처럼 터미널의 그리기 표면을 차지하는 방식을 설명합니다. 터미널 창을 최대화하는 것과는 관계가 없으며 모든 창 크기에서 작동합니다.
</Note>

<h2 id="enable-fullscreen-rendering">
  전체 화면 렌더링 활성화
</h2>

Claude Code 대화 내에서 `/tui fullscreen`을 실행합니다. CLI는 [`tui` 설정](/docs/ko/settings-reference#tui)을 저장하고 대화를 유지한 채로 전체 화면으로 다시 시작하므로 컨텍스트를 잃지 않고 세션 중간에 전환할 수 있습니다. `/tui default`를 실행하여 클래식 렌더러로 다시 전환하거나, 인수 없이 `/tui`를 실행하여 활성 렌더러를 확인합니다.

[화면 읽기 모드](/docs/ko/accessibility)에서 Claude Code는 첨부된 [백그라운드 세션](/docs/ko/agent-view)을 제외하고는 항상 클래식 렌더러를 사용하며, 백그라운드 세션은 여전히 전체 화면으로 렌더링됩니다. 다른 세션에서 `/tui fullscreen`을 실행하면 Claude Code는 전환하지 않고 설명을 출력하며 저장된 `tui` 설정을 변경하지 않습니다.

Claude Code는 다시 시작된 세션으로 다음을 전달합니다:

* 화면에 표시되는 대로의 대화. [`/rewind`](/docs/ko/checkpointing#rewind-and-summarize) 후에는:
  * 세션 초반에 되감기를 실행했다면 Claude Code는 디스크에 저장된 더 긴 기록이 아닌 되감기 지점에서 다시 시작합니다. 예를 들어 마지막 세 개의 메시지를 지나 되감기했다면 다시 시작된 세션은 이들 없이 열립니다
  * 첫 번째 메시지 이전으로 되감기했다면 Claude Code는 빈 대화로 다시 시작합니다
* [권한 모드](/docs/ko/permission-modes) 및 [노력 수준](/docs/ko/model-config#adjust-effort-level)
* [`/model`](/docs/ko/model-config#setting-your-model)로 마지막으로 선택한 모델
* [`--allowed-tools` 또는 `--disallowed-tools`](/docs/ko/cli-reference#cli-flags)로 전달한 규칙, 그리고 `--agent`, `--agents`, `--append-system-prompt`, 및 `--system-prompt-snapshot` 플래그

Claude Code는 다시 시작된 프로세스로 전달할 수 없는 제한이 세션에 있으면 다시 시작을 거부합니다. 전달할 수 없는 제한은 다음을 포함합니다:

* [`--system-prompt`](/docs/ko/cli-reference#cli-flags) 교체, [`--tools`](/docs/ko/cli-reference#cli-flags) 허용 목록, 또는 [`--setting-sources`](/docs/ko/cli-reference#cli-flags)와 같은 시작 플래그
* [훅 또는 SDK 권한 업데이트](/docs/ko/hooks#permission-update-entries)가 이 세션에만 추가한 거부 또는 요청 규칙

이 경우 Claude Code는 이유와 함께 [`Cannot switch renderers in this session`](/docs/ko/errors#cannot-switch-renderers-in-this-session)을 출력합니다. 전환하거나 저장하지 않습니다.

Claude Code를 시작하기 전에 `CLAUDE_CODE_NO_FLICKER` 환경 변수를 설정할 수도 있습니다:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

[`tui`](/docs/ko/settings-reference#tui) 설정과 변수가 모두 설정되었을 때 어떻게 결합되는지는 설정의 항목을 참조하세요. [전체 화면 시작 실패](#fullscreen-renderer-didnt-finish-starting) 후에 Claude Code는 여전히 변수를 준수하지만 설정은 준수하지 않습니다. `/tui` 명령은 다시 시작된 프로세스에서 `CLAUDE_CODE_NO_FLICKER`를 지우므로 작성한 설정이 적용됩니다.

<h3 id="fullscreen-by-default">
  기본적으로 전체 화면
</h3>

첨부된 [백그라운드 세션](/docs/ko/agent-view)은 전체 화면으로 렌더링되고, [화면 읽기 모드](/docs/ko/accessibility)의 다른 세션은 클래식 렌더러를 사용합니다. 그 외에는 Claude Code는 설정과 일치하는 이 표의 첫 번째 행의 렌더러로 시작합니다:

| 상황                                                                                                                             | 시작하는 렌더러     |
| :----------------------------------------------------------------------------------------------------------------------------- | :----------- |
| [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`](/docs/ko/env-vars) 또는 `CLAUDE_CODE_NO_FLICKER=0`을 설정했습니다                                 | 클래식          |
| `CLAUDE_CODE_NO_FLICKER=1`을 설정했습니다                                                                                             | 전체 화면        |
| Claude Code가 이 머신에서 [전체 화면 시작 실패](#fullscreen-renderer-didnt-finish-starting) 후 전체 화면을 끕니다                                     | 클래식          |
| iTerm2의 [`tmux -CC` 통합 모드](#use-with-tmux)에 있거나 Windows에서 실행 중인 Claude Code에 SSH를 통해 연결되어 있습니다                                 | 클래식          |
| [`tui` 설정](/docs/ko/settings-reference#tui)을 저장했습니다                                                                                 | 설정이 지정하는 렌더러 |
| 세션이 Anthropic에서 [기능 플래그를 가져오지 않으며](/docs/ko/env-vars#features-that-need-feature-flag-fetching) Claude Code가 이 머신에서 시작 대화 제공을 중단했습니다 | 클래식          |
| 세션이 Anthropic에서 기능 플래그를 가져오지 않으며 이 머신의 첫 Claude Code 시작이 v2.1.239 이상을 실행했습니다                                                   | 전체 화면        |
| 세션이 Anthropic에서 기능 플래그를 가져오며 Claude Code를 2026년 5월 6일 이후에 처음 사용했습니다                                                            | 전체 화면        |
| 기타                                                                                                                             | 클래식          |

기능 플래그를 가져오지 않는 세션은 [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai), 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry)를 통한 세션, 그리고 원격 분석이 꺼진 세션을 포함합니다.

클래식 렌더러에서 시작하고 `tui` 설정을 저장하지 않았다면 Claude Code는 시작 시 전환을 제공하는 대화를 열 수 있습니다:

* 수락하면 Claude Code는 `/tui fullscreen`과 동일한 방식으로 다시 시작하고 동일한 세션 상태를 전달하며 다시 시작된 세션이 [성공적으로 시작](#fullscreen-renderer-didnt-finish-starting)되면 설정을 저장합니다.
* **나중에**를 선택하면 Claude Code는 이 머신에서 다시 제공하지 않습니다.
* Claude Code는 세 번의 시작에서 대화를 표시한 후 제공을 중단하며 답변 여부는 상관없습니다.

<h2 id="what-changes">
  변경 사항
</h2>

전체 화면 렌더링은 CLI가 터미널에 그리는 방식을 변경합니다. 입력 상자는 출력이 스트리밍될 때 이동하는 대신 화면 하단에 고정됩니다. Claude가 작업 중일 때 입력이 제자리에 있으면 전체 화면 렌더링이 활성화된 것입니다. 렌더 트리에는 표시되는 메시지만 유지되므로 대화 길이에 관계없이 메모리가 일정하게 유지됩니다.

대화가 터미널의 스크롤백 대신 대체 화면 버퍼에 있기 때문에 몇 가지가 다르게 작동합니다:

| 이전                          | 현재                                                    | 세부 정보                                             |
| :-------------------------- | :---------------------------------------------------- | :------------------------------------------------ |
| `Cmd+f` 또는 tmux 검색으로 텍스트 찾기 | 트랜스크립트 모드의 경우 `Ctrl+o`, 그 다음 `/`로 검색하거나 `[`로 스크롤백에 작성 | [대화 검색 및 검토](#search-and-review-the-conversation) |
| 터미널의 기본 클릭 및 드래그로 선택 및 복사   | 앱 내 선택, 마우스 릴리스 시 자동 복사                               | [마우스 사용](#use-the-mouse)                          |
| `Cmd`-클릭으로 URL 열기           | macOS에서는 `Cmd`-클릭, 다른 곳에서는 `Ctrl`-클릭                  | [마우스 사용](#use-the-mouse)                          |

마우스 캡처가 워크플로우를 방해하는 경우 깜빡임 없는 렌더링을 유지하면서 [이를 끌 수 있습니다](#keep-native-text-selection).

<h2 id="use-the-mouse">
  마우스 사용
</h2>

전체 화면 렌더링은 마우스 이벤트를 캡처하고 Claude Code 내에서 처리합니다:

* **프롬프트 입력 필드를 클릭**하여 입력 중인 텍스트의 어디든지 커서를 위치시킵니다.
* **`/` 명령어 또는 `@` 파일 목록의 제안을 클릭**하여 수락합니다. 마우스를 가져가면 커서 아래의 행이 강조됩니다.
* **선택 메뉴의 옵션을 클릭**하여 선택합니다. 이는 권한 프롬프트, `/model`, `/config` 및 옵션 목록을 표시하는 기타 대화상자를 포함합니다. 마우스를 가져가면 커서 아래의 행에 포인터가 표시됩니다.
* **다중 선택 메뉴의 옵션을 클릭**하여 토글하고, 제출 버튼을 클릭하여 선택 사항을 확인합니다. 다중 선택 질문의 `기타` 행과 같은 자유 텍스트 행을 클릭하면 입력 필드에 포커스가 이동하여 답변을 입력할 수 있습니다. Claude Code v2.1.208 이상이 필요합니다.
* **`/config` 패널에서 설정의 값을 클릭**하여 변경하고, 마우스 휠로 설정 목록을 스크롤합니다. Claude Code v2.1.271 이상이 필요합니다.
* **선택 또는 다중 선택 메뉴를 마우스 휠로 스크롤**하여 한 번에 표시되는 것보다 더 많은 옵션이 있을 때(예: 짧은 터미널 창의 `/model` 목록), 휠이 포인터가 옵션 위에 있을 때 목록을 스크롤합니다. Claude Code v2.1.280 이상이 필요합니다.
* **축소된 도구 결과를 클릭**하여 확장하고 전체 출력을 봅니다. 다시 클릭하면 축소됩니다. 도구 호출과 그 결과가 함께 확장됩니다. 더 표시할 내용이 있는 메시지만 클릭 가능합니다.
  * 클릭하면 `!` 셸 명령어의 출력도 확장되며, 이는 이전의 잘린 결과이거나 명령어 실행 중인 라이브 진행 행입니다. Claude Code v2.1.257 이상이 필요합니다.
* **macOS에서 `Cmd`를 누르거나, Linux 및 Windows에서 `Ctrl`을 누르고 URL 또는 파일 경로를 클릭**하여 엽니다. 일반 `http://` 및 `https://` URL은 브라우저에서 열리고, Edit 또는 Write 후에 인쇄된 것과 같은 도구 출력의 파일 경로는 기본 애플리케이션에서 열립니다. 수정자 없이 일반 클릭하면 링크가 열리지 않으며, 이는 기본 터미널 동작과 일치합니다.
  * Claude Code는 네트워크(UNC) 경로(예: `\\server\share\file.ts`)를 링크 없는 일반 텍스트로 렌더링합니다. 네트워크 경로를 열면 Windows 자격 증명이 해당 호스트로 전송될 수 있기 때문입니다.
  * 일부 macOS 터미널은 `Cmd`+클릭을 실행 중인 앱으로 전달하며, 터미널 마우스 프로토콜에는 `Cmd` 키를 인코딩할 방법이 없으므로 Claude Code는 일반 클릭을 받습니다. Ghostty 및 macOS의 Warp에서 Claude Code는 이를 감지하고 링크에 대한 일반 클릭으로 열 수 있으며, `Cmd`를 누르고 있으면 여전히 작동합니다.
  * VS Code 통합 터미널 및 유사한 xterm.js 기반 터미널에서 Claude Code는 터미널의 자체 링크 핸들러에 양보하며, 이는 동일한 제스처를 사용합니다.
* **클릭하고 드래그**하여 대화의 어디든지 텍스트를 선택합니다. 더블클릭하면 단어를 선택하며, iTerm2의 단어 경계와 일치하므로 파일 경로가 하나의 단위로 선택됩니다. URL을 더블클릭하면 스키마를 포함한 전체 URL이 선택됩니다. 트리플클릭하면 행을 선택합니다.
* **마우스 휠로 스크롤**하여 대화를 이동합니다.

선택된 텍스트는 마우스를 놓을 때 자동으로 클립보드에 복사됩니다. 이를 끄려면 `/config`에서 선택 시 복사를 토글합니다.

선택 시 복사가 꺼져 있으면 `Ctrl+Shift+c`를 눌러 수동으로 복사합니다. kitty, WezTerm, Ghostty 및 iTerm2와 같은 kitty 키보드 프로토콜을 지원하는 터미널에서는 `Cmd+c`도 작동합니다. 활성 선택이 있으면 `Ctrl+c`는 취소하는 대신 복사합니다.

활성 선택이 있으면 `Shift`를 누르고 화살표 키를 눌러 키보드에서 확장합니다. `Shift+↑` 및 `Shift+↓`는 선택이 위쪽 또는 아래쪽 가장자리에 도달할 때 뷰포트를 스크롤합니다. `Shift+Home` 및 `Shift+End`는 현재 행의 시작 또는 끝으로 확장합니다.

일반 프롬프트 보기에서 활성 선택에 어떤 일이 발생하는지는 누르는 키에 따라 달라집니다:

* **`Esc`**: Claude Code는 실행 중인 응답 중단 또는 열린 대화상자 해제와 같은 키의 일반적인 작업을 수행하고, 선택은 강조 표시된 상태로 유지됩니다.
* **`PgUp`, `PgDn`, `Ctrl+Home`, `Ctrl+End` 또는 `Shift`, `Alt` 또는 `Option`, 또는 `Cmd`, `Win` 또는 `Super`와 화살표, `Home` 또는 `End` 키**: 선택이 유지됩니다.
* **일반 화살표 키, `Enter` 및 입력된 문자를 포함한 다른 모든 키**: Claude Code는 선택을 지웁니다.
* **[`selection:clear`](/docs/ko/keybindings#scroll-actions)에 바인딩된 키**: Claude Code는 선택을 지우며, 키가 `Esc` 또는 다른 방식으로 유지되는 다른 키인 경우에도 마찬가지입니다. 이 작업에는 기본 바인딩이 없습니다.

[트랜스크립트 모드](#search-and-review-the-conversation)에서 거기에 나열된 탐색 및 검색 키도 선택을 유지합니다.

<h2 id="scroll-the-conversation">
  대화 스크롤하기
</h2>

전체 화면 렌더링은 앱 내에서 스크롤링을 처리합니다. 다음 단축키를 사용하여 탐색하세요:

| 단축키             | 작업                        |
| :-------------- | :------------------------ |
| `PgUp` / `PgDn` | 화면의 절반만큼 위 또는 아래로 스크롤     |
| `Ctrl+Home`     | 대화의 시작 부분으로 이동            |
| `Ctrl+End`      | 최신 메시지로 이동하고 자동 추적 다시 활성화 |
| 마우스 휠           | 한 번에 몇 줄씩 스크롤             |

[압축](/docs/ko/context-window#what-survives-compaction) 후에도 세션의 시작 부분으로 스크롤백할 수 있습니다. Claude는 압축 요약에서 계속 작동하지만, Claude Code는 반복된 압축 전체에서 전체 화면 스크롤백에서 모든 이전 메시지를 유지합니다.

MacBook 키보드와 같이 전용 `PgUp`, `PgDn`, `Home` 또는 `End` 키가 없는 키보드에서는 `Fn`을 화살표 키와 함께 누르세요: `Fn+↑`는 `PgUp`을 보내고, `Fn+↓`는 `PgDn`을 보내고, `Fn+←`는 `Home`을 보내고, `Fn+→`는 `End`를 보냅니다. `Ctrl+Fn+→`는 macOS의 Claude Code에 도달하지 않으므로 MacBook 키보드는 기본적으로 작동하는 하단으로 이동 단축키가 없습니다. 대신 다음 옵션 중 하나를 사용하세요:

* [하단으로 이동 버튼](#auto-follow)을 클릭하세요.
* 마우스 휠로 하단까지 스크롤하여 추적을 재개하세요.
* `scroll:bottom`을 키보드가 보낼 수 있는 단축키로 다시 바인딩하세요.

이러한 작업은 다시 바인딩할 수 있습니다. 기본 바인딩이 없는 반 페이지 및 전체 페이지 변형을 포함한 전체 작업 이름 목록은 [스크롤 작업](/docs/ko/keybindings#scroll-actions)을 참조하세요.

위로 스크롤하면 대화 기록 상단에 흐릿한 헤더 행이 표시되어 뷰 위로 스크롤된 가장 최근의 프롬프트를 보여줍니다. 행을 클릭하면 해당 프롬프트로 이동합니다.

<h3 id="auto-follow">
  자동 추적
</h3>

위로 스크롤하면 자동 추적이 일시 중지되어 새 출력이 사용자를 다시 하단으로 끌어당기지 않습니다. 위로 스크롤했을 때 `하단으로 이동` 버튼이 대화 기록의 하단 가장자리 위에 떠 있으며, 새 출력이 도착하면 `3개의 새 메시지`와 같은 개수를 표시합니다. 이를 클릭하거나, `Ctrl+End`를 누르거나, 하단까지 스크롤하여 추적을 재개하세요.

자동 추적이 일시 중지된 동안 응답 스트리밍이 완료되면 뷰도 스크롤한 위치에 유지됩니다.

버튼의 키보드 힌트는 키보드가 보낼 수 있는 것을 반영합니다. macOS에서는 클릭을 제안하거나 `Fn+↓`로 스크롤하도록 제안합니다. `Ctrl+End`는 Mac 키보드에서 Claude Code에 도달하지 않기 때문입니다. [`scroll:bottom`](/docs/ko/keybindings#scroll-actions)을 다시 바인딩하면 버튼이 모든 플랫폼에서 단축키를 표시합니다.

터미널이 전체 레이블을 표시하기에 너무 좁으면 버튼은 대화 기록 행 아래로 줄바꿈하는 대신 힌트를 단축합니다.

자동 추적을 완전히 끄고 뷰가 사용자가 남긴 위치에 유지되도록 하려면 `/config`를 열고 자동 스크롤을 끄기로 설정하세요. 자동 스크롤이 비활성화되면 뷰는 자동으로 하단으로 이동하지 않습니다. 응답이 필요한 권한 프롬프트 및 기타 대화 상자는 이 설정과 관계없이 여전히 뷰로 스크롤됩니다.

<h3 id="mouse-wheel-scrolling">
  마우스 휠 스크롤
</h3>

마우스 휠 스크롤을 사용하려면 터미널이 마우스 이벤트를 Claude Code로 전달해야 합니다. 대부분의 터미널은 애플리케이션이 요청할 때마다 이를 수행합니다. iTerm2는 이를 프로필별 설정으로 만듭니다: 휠이 작동하지 않지만 `PgUp`과 `PgDn`이 작동하면 설정 → 프로필 → 터미널을 열고 마우스 보고 활성화를 켜세요. 클릭 확장 및 텍스트 선택이 작동하려면 동일한 설정도 필요합니다.

마우스 휠 스크롤이 느리게 느껴지면 터미널이 배수 없이 물리적 노치당 하나의 스크롤 이벤트를 보낼 수 있습니다. Ghostty 및 더 빠른 스크롤이 활성화된 iTerm2와 같은 일부 터미널은 이미 휠 이벤트를 증폭합니다. VS Code 통합 터미널을 포함한 다른 터미널은 노치당 정확히 하나의 이벤트를 보냅니다. Claude Code는 어느 것인지 감지할 수 없습니다.

기본 스크롤 거리를 곱하려면 `CLAUDE_CODE_SCROLL_SPEED`를 설정하세요:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

`3` 값은 `vim` 및 유사한 애플리케이션의 기본값과 일치합니다. 이 설정은 `0.25`와 같이 1 미만의 분수 값을 포함하여 20까지의 양수 값을 허용하여 이미 휠 이벤트를 증폭하는 터미널에서 가속화된 트랙패드 및 휠 스크롤을 느리게 합니다.

스크롤 속도를 대화식으로 조정하려면 `/scroll-speed`를 실행하세요. 대화 상자는 열려 있는 동안 스크롤할 수 있는 눈금자를 표시하므로 변경 사항을 즉시 느낄 수 있습니다. `←` 및 `→`를 눌러 속도를 조정하고, `r`을 눌러 자동 감지된 기본값으로 재설정하고, `Enter`를 눌러 저장하세요. 대화 상자는 10까지 전체 숫자로 단계를 진행하며, 더 세밀한 제어를 지원하는 터미널에서는 0.25까지의 1/4 단계도 제공합니다.

이 명령은 `CLAUDE_CODE_SCROLL_SPEED` 환경 변수가 설정하는 동일한 값을 작성하며, `~/.claude/settings.json`에 유지됩니다. 대화 상자의 최대값은 10입니다: 환경 변수를 통해 더 높은 값을 설정하면 대화 상자는 10을 표시하고, 대화 상자에서 저장하면 10이 유지됩니다. 이 명령은 JetBrains IDE 터미널에서 사용할 수 없습니다.

기본 속도와 별도로 Claude Code는 휠을 빠르게 회전할 때 스크롤 속도를 가속화하므로 빠른 회전은 동일한 수의 느린 노치보다 더 많은 거리를 커버합니다. 가속을 끄고 노치당 일정한 속도를 유지하려면 [`settings.json`](/docs/ko/settings-reference#all-settings)에서 `wheelScrollAccelerationEnabled`를 `false`로 설정하세요. 이 설정은 Claude Code v2.1.174 이상이 필요합니다.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  JetBrains IDE 터미널에서 스크롤
</h3>

JetBrains IDE 터미널에서 Claude Code는 자체 스크롤 처리를 적용하고 `CLAUDE_CODE_SCROLL_SPEED`를 무시합니다. 터미널은 다른 에뮬레이터보다 훨씬 높은 속도로 스크롤 이벤트를 보내므로 다른 곳에서 조정된 배수는 여기서 과도합니다.

2025.2에서 터미널에는 또한 허위 화살표 키 및 잘못된 방향 이벤트를 생성하는 스크롤 휠 버그가 있습니다. Claude Code는 런타임에 이를 감지하고 자동으로 완화하므로 트랙패드 및 마우스 휠 스크롤이 구성 없이 작동합니다. 최고의 스크롤 환경을 위해 2025.3 이상으로 업그레이드하세요. Claude Code는 버그를 감지하면 처음 스크롤할 때 힌트를 표시합니다.

<h2 id="search-and-review-the-conversation">
  대화 검색 및 검토
</h2>

`Ctrl+o`는 일반 프롬프트와 트랜스크립트 모드 사이를 전환합니다.

마지막 프롬프트만 표시하고, 도구 호출의 한 줄 요약과 편집 diffstats, 그리고 최종 응답을 보여주는 더 조용한 보기를 원하면 `/focus`를 실행하세요. 이 설정은 세션 전체에 걸쳐 유지됩니다. `/focus`를 다시 실행하면 꺼집니다.

트랜스크립트 모드는 `less` 스타일의 네비게이션과 검색을 제공합니다:

| 키                                    | 작업                                                                      |
| :----------------------------------- | :---------------------------------------------------------------------- |
| `/`                                  | 검색을 엽니다. 입력하여 일치 항목을 찾고, `Enter`를 눌러 수락하고, `Esc`를 눌러 취소하고 스크롤 위치를 복원합니다 |
| `n` / `N`                            | 다음 또는 이전 일치 항목으로 이동합니다. 검색 표시줄을 닫은 후에 작동합니다                             |
| `j` / `k` 또는 `↑` / `↓`               | 한 줄 스크롤                                                                 |
| `g` / `G` 또는 `Home` / `End`          | 맨 위 또는 맨 아래로 이동                                                         |
| `{` / `}`                            | 이전 또는 다음 프롬프트로 이동                                                       |
| `Ctrl+u` / `Ctrl+d`                  | 반 페이지 스크롤                                                               |
| `Ctrl+b` / `Ctrl+f` 또는 `Space` / `b` | 전체 페이지 스크롤                                                              |
| `Ctrl+o`, `Esc`, 또는 `q`              | 트랜스크립트 모드를 종료하고 프롬프트로 돌아갑니다                                             |

터미널의 `Cmd+f`와 tmux 검색은 대화가 기본 스크롤백이 아닌 대체 화면 버퍼에 있기 때문에 대화를 볼 수 없습니다. 콘텐츠를 터미널로 다시 전달하려면 먼저 `Ctrl+o`를 눌러 트랜스크립트 모드로 들어간 다음:

* **`[`**: 모든 도구 출력이 확장된 전체 대화를 터미널의 기본 스크롤백 버퍼에 씁니다. 이제 대화는 터미널의 일반 텍스트이므로 `Cmd+f`, tmux 복사 모드 및 기타 모든 기본 도구가 검색하거나 선택할 수 있습니다. 긴 세션은 이 작업이 진행되는 동안 잠시 일시 중지될 수 있습니다. 이는 `Esc` 또는 `q`로 트랜스크립트 모드를 종료할 때까지 지속되며, 이는 전체 화면 렌더링으로 돌아갑니다. 다음 `Ctrl+o`는 새로 시작합니다.
* **`v`**: 대화를 임시 파일에 쓰고 `$VISUAL` 또는 `$EDITOR`에서 엽니다.

<h2 id="watch-your-changes-in-the-diff-panel">
  차이점 패널에서 변경 사항 확인하기
</h2>

전체 화면 렌더링에서 [`/diff`](/docs/ko/interactive-mode#review-changes-with-%2Fdiff)는 닫아야 하는 뷰어가 아니라 대화 옆에 패널을 열어서 Claude가 작업하는 동안 변경 사항이 누적되는 것을 볼 수 있습니다. 넓은 터미널에서는 Claude가 파일 편집을 시작하면 패널이 자동으로 열릴 수도 있습니다. [차이점 패널](/docs/ko/interactive-mode#diff-panel)에서는 표시되는 내용, 패널을 닫힌 상태로 유지하는 방법, 비교 대상을 변경하는 방법을 다룹니다.

<h2 id="clear-the-conversation">
  대화 지우기
</h2>

`/clear`를 실행하여 새로운 대화를 시작합니다.

화면이 깨져 보이거나 부분적으로 비어 있으면 `Ctrl+L`을 눌러 화면을 다시 그립니다. 다시 그리기는 대화와 입력을 제자리에 유지합니다.

터미널이 `Cmd+K`를 Claude Code에 전달할 때 `Cmd+K`는 `Ctrl+L`과 동일한 작업을 수행합니다. iTerm2와 Terminal.app은 `Cmd+K`를 자체적으로 처리하고 자신의 화면을 지우며, Claude Code는 지워진 화면을 감지하고 대화를 다시 그립니다. v2.1.280 이전에는 v2.1.260부터 `Ctrl+L` 또는 Claude Code에 도달하는 `Cmd+K`를 누르면 전체 화면 렌더링에서 화면이 지워졌습니다. v2.1.238 이전에는 2초 이내에 `Ctrl+L`을 두 번 누르면 `/clear`가 실행되었습니다.

<h2 id="use-with-tmux">
  tmux와 함께 사용하기
</h2>

전체 화면 렌더링은 tmux 내에서 작동하지만, 세 가지 주의사항이 있습니다.

마우스 휠 스크롤을 사용하려면 tmux의 마우스 모드가 필요합니다. `~/.tmux.conf`에서 이미 활성화되지 않았다면, 다음 줄을 추가하고 설정을 다시 로드하세요:

```bash theme={null}
set -g mouse on
```

마우스 모드가 없으면 휠 이벤트가 Claude Code 대신 tmux로 전달됩니다. `PgUp`과 `PgDn`을 사용한 키보드 스크롤은 어느 쪽이든 작동합니다. Claude Code는 마우스 모드가 꺼진 tmux를 감지하면 시작 시 일회성 힌트를 출력합니다.

전체 화면 렌더링은 iTerm2의 tmux 통합 모드와 호환되지 않습니다. 이는 `tmux -CC`로 진입하는 모드입니다. 통합 모드에서 iTerm2는 각 tmux 창을 tmux가 터미널에 그리도록 하는 대신 네이티브 분할로 렌더링합니다. 대체 화면 버퍼와 마우스 추적이 제대로 작동하지 않습니다: 마우스 휠이 작동하지 않으며, 더블 클릭으로 터미널 상태가 손상될 수 있습니다. `tmux -CC` 세션에서는 전체 화면 렌더링을 활성화하지 마세요. `-CC` 없이 iTerm2 내의 일반 tmux는 정상적으로 작동합니다.

3.6 시리즈까지의 tmux 릴리스는 동기화된 출력을 구현하지 않으므로, 해당 버전에서는 Claude Code를 터미널에서 직접 실행할 때보다 다시 그리기 중에 더 많은 깜박임이 보일 수 있습니다. Claude Code는 시작 시 터미널에서 동기화된 출력 지원을 탐지하고 터미널이 이를 보고할 때 사용합니다. tmux에서 깜박임이 보이면 최신 tmux로 업그레이드하거나 tmux 외부의 자체 터미널 탭에서 Claude Code를 실행하세요.

<h2 id="keep-native-text-selection">
  기본 텍스트 선택 유지
</h2>

마우스 캡처는 특히 SSH 또는 tmux 내에서 가장 일반적인 마찰 지점입니다. Claude Code가 마우스 이벤트를 캡처하면 터미널의 기본 선택 시 복사 기능이 작동하지 않습니다. 클릭 및 드래그로 만든 선택은 Claude Code 내부에 존재하며, 터미널의 선택 버퍼에 존재하지 않으므로 tmux 복사 모드, Kitty 힌트 및 유사한 도구가 이를 볼 수 없습니다.

Claude Code는 선택 항목을 시스템 클립보드에 기록하며, 사용하는 경로는 설정에 따라 다릅니다. 로컬 세션에서는 기본 클립보드 도구를 실행합니다:

* **macOS**: `pbcopy`
* **Linux**: Wayland의 경우 `wl-copy`, X11의 경우 `xclip` 또는 `xsel` (설치된 것). Claude Code는 클립보드와 PRIMARY 선택을 모두 기록하므로 중간 클릭 붙여넣기가 작동합니다.
* **Windows 및 WSL**: PowerShell `Set-Clipboard`

tmux 내에서는 tmux 붙여넣기 버퍼에도 기록합니다. SSH를 통해서는 OSC 52 이스케이프 시퀀스로 폴백합니다. GNU screen 내에서 Claude Code는 긴 선택 항목을 클립보드에도 복사합니다. v2.1.219 이전에는 대략 570자보다 긴 선택 항목을 복사하면 GNU screen이 base64 텍스트를 창에 인쇄했습니다. Claude Code는 각 복사 후 사용한 경로를 알려주는 토스트를 인쇄합니다.

일부 터미널은 기본적으로 OSC 52를 차단합니다. iTerm2는 Settings → General → Selection → Applications in terminal may access clipboard를 켤 때까지 차단합니다. iTerm2에서 [`/terminal-setup`](/docs/ko/terminal-config)을 실행하면 이를 활성화합니다.

일회성 기본 선택의 경우 사용할 키는 터미널에 따라 다릅니다:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor 및 Devin Desktop**: `Shift`, 또는 `terminal.integrated.macOptionClickForcesSelection` 설정이 활성화된 macOS의 경우 `Option`
* **대부분의 다른 터미널**: `Shift`

해당 키를 누른 상태에서 클릭하고 드래그합니다. 터미널이 선택을 직접 처리하므로 Claude Code에 전달하지 않으며, `Cmd+C`와 같은 복사 단축키가 선택한 항목에서 작동합니다. Claude Code는 화면 힌트에서도 올바른 키를 표시합니다.

SSH 또는 tmux 내에서 Claude Code는 연결 중인 터미널을 항상 감지할 수 없으므로 힌트는 후보 키를 나열합니다.

항상 기본 선택에 의존하는 경우 `CLAUDE_CODE_DISABLE_MOUSE=1`을 설정하여 깜박임 없는 렌더링 및 평면 메모리를 유지하면서 마우스 캡처를 거부합니다:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

마우스 캡처가 비활성화되면 `PgUp`, `PgDn`, `Ctrl+Home` 및 `Ctrl+End`를 사용한 키보드 스크롤이 계속 작동하며 터미널이 기본적으로 선택을 처리합니다. 클릭하여 커서 위치 지정, 클릭하여 도구 출력 확장, URL 클릭 및 Claude Code 내 휠 스크롤을 잃게 됩니다.

휠 스크롤을 유지하되 클릭, 드래그 및 호버 처리를 끄려면 대신 `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`을 설정합니다. Claude Code v2.1.195 이상이 필요합니다. 두 변수가 모두 설정된 경우 `CLAUDE_CODE_DISABLE_MOUSE`가 우선합니다.

클릭이 비활성화되면 Claude Code는 여전히 마우스를 캡처하므로 휠 및 터치패드가 대화를 스크롤하지만 Claude Code 내에서 왼쪽 클릭은 아무 작동도 하지 않습니다. 기본 클릭 및 드래그 선택을 위해 터미널의 키를 누른 상태로 유지해야 합니다. 오른쪽 클릭 및 중간 클릭 붙여넣기는 지원하는 터미널에서 계속 작동합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  화면에 오래된 텍스트 또는 잘못된 위치의 텍스트
</h3>

전체 화면 렌더링은 프레임 간에 변경된 셀만 전송합니다. 일부 터미널, 특히 Windows Terminal 및 기타 ConPTY 기반 호스트는 이러한 위치 지정 쓰기를 잘못 병합하여 창 크기를 조정할 때까지 이전 출력의 조각을 화면에 남깁니다.

증분 업데이트를 전송하는 대신 모든 프레임에서 모든 셀을 다시 칠하려면 [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/ko/env-vars)을 설정합니다.

Windows PowerShell에서:

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

macOS 또는 Linux에서:

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

Windows에서는 Claude Code가 이미 백그라운드 세션 및 [에이전트 보기](/docs/ko/agent-view)에 대해 자동으로 전체 다시 칠하기를 활성화하므로, 직접 시작한 대화형 전체 화면 세션에 대해서만 변수를 설정하면 됩니다.

<h3 id="fullscreen-renderer-didnt-finish-starting">
  `Claude Code의 전체 화면 렌더러가 지난번에 시작을 완료하지 못했습니다`가 시작 시 나타남
</h3>

이 머신의 전체 화면 세션이 성공적으로 시작되기 전에 충돌하면, Claude Code는 다음 세션을 클래식 렌더러에서 시작하고 두 줄 중 하나를 인쇄합니다. 세션은 첫 번째 프레임을 그린 후 10초 동안 유지되거나 `/exit`, Ctrl+C 또는 Ctrl+D로 종료되면 성공적으로 시작된 것입니다. 표시되는 줄은 이 세션 후 Claude Code가 수행하는 작업을 알려줍니다:

* 한 번의 실패한 시작 후에는 `Claude Code의 전체 화면 렌더러가 이 머신에서 지난번에 시작을 완료하지 못했습니다`가 표시됩니다. Claude Code는 다음에 시작하는 세션에서 전체 화면 렌더링을 다시 시도합니다
* 두 번의 실패한 시작 후에는 `Claude Code의 전체 화면 렌더러가 이 머신에서 반복적으로 시작에 실패했습니다`가 표시됩니다. Claude Code는 Claude Code를 업데이트하거나 `/tui fullscreen`을 실행할 때까지 클래식 렌더러를 계속 사용하며, 이후 세션에서는 아무것도 인쇄하지 않습니다

실패한 시작이 클래식 렌더러에 있는 이유인지 확인하려면 인수 없이 `/tui`를 실행합니다. 실패한 시작이 이유인 동안 `Current renderer` 줄이 그렇게 표시됩니다.

클래식 렌더러를 유지하려면 `/tui default`를 실행하여 다시 시작하지 않고 `tui` 설정을 저장합니다. 전체 화면 렌더링을 다시 시도하려면 `/tui fullscreen`을 실행합니다. 해당 세션도 시작을 완료하지 못하면 [문제를 보고합니다](#research-preview).

v2.1.236 이전에는 Claude Code가 실패한 시작 후에도 계속 전체 화면 렌더링에서 세션을 시작했습니다.

<h4 id="how-claude-code-counts-failed-starts">
  Claude Code가 실패한 시작을 계산하는 방법
</h4>

* 계산되는 세션: `tui` 설정이 그렇게 말하기 때문에, [시작 대화상자](#fullscreen-by-default)를 수락했기 때문에, 또는 Claude Code가 기본적으로 전체 화면에서 시작하기 때문에 전체 화면 렌더링에서 시작된 세션만
* `CLAUDE_CODE_NO_FLICKER=1`: 설정하면 Claude Code는 실패한 시작 후에도 해당 세션을 전체 화면으로 렌더링하고 계산하지 않습니다
* 계산 재설정: Claude Code는 Claude Code 버전당 실패한 시작을 계산하며, 성공적인 전체 화면 시작은 계산을 재설정합니다
* 시작 대화상자: 대화상자를 수락했고 다시 시작된 세션이 충돌한 경우, Claude Code는 어느 줄도 인쇄하지 않으며 이 Claude Code 버전에서 대화상자를 다시 표시하지 않습니다

<h2 id="research-preview">
  연구 미리보기
</h2>

전체 화면 렌더링은 연구 미리보기 기능입니다. 일반적인 터미널 에뮬레이터에서 테스트되었지만, 덜 일반적인 터미널이나 특이한 구성에서 렌더링 문제가 발생할 수 있습니다.

문제가 발생하면 Claude Code 내에서 `/feedback`을 실행하여 보고하거나, [claude-code GitHub 저장소](https://github.com/anthropics/claude-code/issues)에서 이슈를 열어주십시오. 터미널 에뮬레이터 이름과 버전을 포함해주십시오.

전체 화면 렌더링을 끄려면 `/tui default`를 실행하거나, 그렇게 활성화한 경우 `CLAUDE_CODE_NO_FLICKER`를 설정 해제하십시오. `/tui default`로 다시 전환하면 Claude Code는 먼저 전환한 이유를 묻는 선택적 피드백 프롬프트를 표시할 수 있습니다. 이유를 입력하고 `Enter`를 눌러 전송하거나, `Esc`를 눌러 건너뛰십시오. CLI는 어느 쪽이든 클래식 렌더러로 다시 시작됩니다. 저장된 `tui` 설정과 관계없이 클래식 렌더러를 강제하려면 `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`을 설정하십시오. 클래식 렌더러는 터미널의 기본 스크롤백에 대화를 유지하므로 `Cmd+f`와 tmux 복사 모드가 평소대로 작동합니다.

[에이전트 뷰](/docs/ko/agent-view) 또는 `claude attach`에서 열린 백그라운드 세션은 항상 전체 화면 렌더링을 사용합니다. 연결하는 터미널은 세션을 표시하기 위해 대체 화면 버퍼로 들어가고, 클래식 렌더러는 거기에 스크롤백이나 마우스 처리가 없으므로 `tui` 설정과 `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`이 적용되지 않습니다.
