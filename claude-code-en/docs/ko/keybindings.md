> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 키보드 단축키 사용자 정의

> keybindings 구성 파일을 사용하여 Claude Code에서 키보드 단축키를 사용자 정의합니다.

Claude Code는 사용자 정의 가능한 키보드 단축키를 지원합니다. `/keybindings`를 실행하여 `~/.claude/keybindings.json`에서 구성 파일을 만들거나 열 수 있습니다.

<h2 id="configuration-file">
  구성 파일
</h2>

keybindings 구성 파일은 `bindings` 배열이 있는 객체입니다. 각 블록은 컨텍스트와 키 입력을 작업에 매핑하는 맵을 지정합니다.

<Note>keybindings 파일의 변경 사항은 자동으로 감지되고 Claude Code를 다시 시작하지 않고도 적용됩니다.</Note>

| 필드         | 설명                                |
| :--------- | :-------------------------------- |
| `$schema`  | 편집기 자동 완성을 위한 선택적 JSON Schema URL |
| `$docs`    | 선택적 설명서 URL                       |
| `bindings` | 컨텍스트별 바인딩 블록 배열                   |

이 예제는 채팅 컨텍스트에서 `Ctrl+E`를 외부 편집기를 열기에 바인딩하고 `Ctrl+U`를 바인딩 해제합니다:

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/ko/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  컨텍스트
</h2>

각 바인딩 블록은 바인딩이 적용되는 **컨텍스트**를 지정합니다:

| 컨텍스트              | 설명                                                |
| :---------------- | :------------------------------------------------ |
| `Global`          | 앱의 모든 곳에 적용됨                                      |
| `Chat`            | 주 채팅 입력 영역                                        |
| `Autocomplete`    | 자동 완성 메뉴가 열려 있음                                   |
| `Settings`        | 설정 메뉴                                             |
| `Confirmation`    | 권한 및 확인 대화 상자                                     |
| `Tabs`            | 탭 네비게이션 구성 요소                                     |
| `Help`            | 도움말 메뉴가 표시됨                                       |
| `Transcript`      | 트랜스크립트 뷰어                                         |
| `HistorySearch`   | 기록 검색 모드(Ctrl+R)                                  |
| `Task`            | 백그라운드 작업이 실행 중                                    |
| `ThemePicker`     | 테마 선택기 대화 상자                                      |
| `Attachments`     | 이미지 첨부 파일 네비게이션 선택 대화 상자                          |
| `Footer`          | 바닥글 표시기 네비게이션(작업, 팀, diff, 아티팩트)                  |
| `MessageSelector` | 되돌리기 및 요약 대화 상자 메시지 선택                            |
| `DiffDialog`      | Diff 뷰어 네비게이션                                     |
| `DiffPanel`       | [diff 패널](/docs/ko/interactive-mode#diff-panel)이 열려 있음 |
| `ModelPicker`     | 모델 선택기 노력 수준                                      |
| `EffortSlider`    | `/effort`로 열린 노력 슬라이더                             |
| `Select`          | 일반 선택/목록 구성 요소                                    |
| `Plugin`          | 플러그인 대화 상자(찾아보기, 발견, 관리)                          |
| `Agents`          | [에이전트 보기](/docs/ko/agent-view)(`claude agents`)        |
| `Scroll`          | 전체 화면 모드에서 대화 스크롤 및 텍스트 선택                        |

v2.1.205 이전에는 `/doctor` 진단 화면에 대한 `Doctor` 컨텍스트와 `doctor:fix` 작업이 존재했습니다.

<h2 id="available-actions">
  사용 가능한 작업
</h2>

작업은 `namespace:action` 형식을 따릅니다. 예를 들어 `chat:submit`은 메시지를 보내고 `app:toggleTodos`는 작업 목록을 표시합니다. 각 컨텍스트에는 사용 가능한 특정 작업이 있습니다.

<h3 id="app-actions">
  앱 작업
</h3>

`Global` 컨텍스트에서 사용 가능한 작업:

| 작업                     | 기본값       | 설명                                                                         |
| :--------------------- | :-------- | :------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C    | 현재 작업 취소                                                                   |
| `app:exit`             | Ctrl+D    | Claude Code 종료. 800ms 이내에 두 번 누르면 확인됨                                      |
| `app:redraw`           | (바인딩 해제됨) | 터미널 다시 그리기 강제                                                              |
| `app:toggleTodos`      | Ctrl+T    | Claude의 할 일 체크리스트 표시 여부 전환. 이것은 [`/tasks`](/docs/ko/commands) 백그라운드 작업 보기가 아닙니다 |
| `app:toggleTranscript` | Ctrl+O    | 상세 트랜스크립트 전환                                                               |

<h3 id="history-actions">
  기록 작업
</h3>

명령 기록을 탐색하기 위한 작업:

| 작업                 | 기본값    | 설명       |
| :----------------- | :----- | :------- |
| `history:search`   | Ctrl+R | 기록 검색 열기 |
| `history:previous` | Up     | 이전 기록 항목 |
| `history:next`     | Down   | 다음 기록 항목 |

<h3 id="chat-actions">
  채팅 작업
</h3>

`Chat` 컨텍스트에서 사용 가능한 작업:

| 작업                    | 기본값                            | 설명                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :-------------------- | :----------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `chat:cancel`         | Escape                         | 현재 입력 취소                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:clearInput`     | Ctrl+L                         | 입력과 대화를 유지하면서 전체 화면을 다시 그리기 강제합니다                                                                                                                                                                                                                                                                                                                                                                                         |
| `chat:clearScreen`    | Cmd+K                          | `chat:clearInput`과 동일합니다. [대화 지우기](/docs/ko/fullscreen#clear-the-conversation)에서 Cmd+K가 iTerm2 및 Terminal.app에서 어떻게 작동하는지 확인하세요                                                                                                                                                                                                                                                                                                |
| `chat:killAgents`     | Ctrl+X Ctrl+K                  | 이 세션의 모든 실행 중인 [백그라운드 서브에이전트](/docs/ko/sub-agents#run-subagents-in-foreground-or-background) 종료 및 이 세션의 나머지 부분에 대해 [아티팩트 자동 회신](/docs/ko/artifacts#let-claude-reply-to-comments-on-its-own) 끄기                                                                                                                                                                                                                                      |
| `chat:cycleMode`      | Shift+Tab\*                    | 권한 모드 순환                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:modelPicker`    | Meta+P                         | 모델 선택기 열기                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `chat:fastMode`       | Meta+O                         | 빠른 모드 전환                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:thinkingToggle` | Meta+T                         | 확장 사고 전환                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:submit`         | Enter                          | 메시지 제출                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `chat:queueSubmit`    | Ctrl+X Enter                   | 메시지를 제출하고 대기 표시: Claude가 작업 중일 때 Claude Code [이를 대기열에 넣고](/docs/ko/interactive-mode#queue-messages-while-claude-works) 턴을 중단하지 않습니다. `chat:submit`과 달리 자동 완성 제안이 열려 있어도 초안을 제출합니다. v2.1.247 이상 필요                                                                                                                                                                                                                              |
| `chat:sendNow`        | Ctrl+Enter, Ctrl+X Ctrl+S      | [대기 중인 메시지](/docs/ko/interactive-mode#queue-messages-while-claude-works)와 초안을 즉시 보냅니다. [Claude Code가 대기 중인 메시지를 보낼 때](/docs/ko/interactive-mode#when-claude-code-sends-what-you-queued)에서 Claude가 작업 중인 턴에 어떤 일이 발생하는지 설명합니다. 실행 중인 것이 없으면 초안을 제출하고 [셸 모드](/docs/ko/interactive-mode#shell-mode-with-prefix)에서는 명령을 대기열에만 넣습니다. 확장 키를 보고하지 않는 터미널은 `Ctrl+Enter`를 일반 `Enter`로 전달하므로 `Ctrl+X Ctrl+S`는 모든 터미널에서 작동하는 바인딩입니다. v2.1.275 이상 필요 |
| `chat:newline`        | Ctrl+J                         | 제출하지 않고 줄 바꿈 삽입                                                                                                                                                                                                                                                                                                                                                                                                           |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-          | 마지막 작업 실행 취소                                                                                                                                                                                                                                                                                                                                                                                                              |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E          | 외부 편집기에서 열기. [에이전트 보기 디스패치 입력](/docs/ko/agent-view#keyboard-shortcuts)도 이 작업의 단일 키 바인딩을 따릅니다                                                                                                                                                                                                                                                                                                                                   |
| `chat:stash`          | Ctrl+S                         | 현재 프롬프트 숨기기                                                                                                                                                                                                                                                                                                                                                                                                               |
| `chat:imagePaste`     | Ctrl+V (Windows 및 WSL에서 Alt+V) | 클립보드에서 이미지 붙여넣기. WSL에서는 두 단축키 모두 기본적으로 바인딩됩니다                                                                                                                                                                                                                                                                                                                                                                             |

\*VT 모드가 없는 Windows에서(Node \<24.2.0/\<22.17.0, Bun \<1.2.23), 기본값은 Meta+M입니다.

<h3 id="autocomplete-actions">
  자동 완성 작업
</h3>

`Autocomplete` 컨텍스트에서 사용 가능한 작업:

| 작업                      | 기본값    | 설명    |
| :---------------------- | :----- | :---- |
| `autocomplete:accept`   | Tab    | 제안 수락 |
| `autocomplete:dismiss`  | Escape | 메뉴 닫기 |
| `autocomplete:previous` | Up     | 이전 제안 |
| `autocomplete:next`     | Down   | 다음 제안 |

<h3 id="confirmation-actions">
  확인 작업
</h3>

`Confirmation` 컨텍스트에서 사용 가능한 작업:

| 작업                      | 기본값         | 설명                                                                                                                                                                             |
| :---------------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`           | Enter       | 작업 확인                                                                                                                                                                          |
| `confirm:no`            | Escape      | 작업 거부                                                                                                                                                                          |
| `confirm:previous`      | Up          | 이전 옵션                                                                                                                                                                          |
| `confirm:next`          | Down        | 다음 옵션                                                                                                                                                                          |
| `confirm:nextField`     | Tab         | 다음 필드                                                                                                                                                                          |
| `confirm:previousField` | (바인딩 해제됨)   | 이전 필드                                                                                                                                                                          |
| `confirm:toggle`        | Space       | 선택 전환                                                                                                                                                                          |
| `confirm:cycleMode`     | Shift+Tab\* | 권한 모드 순환. 파일 권한 프롬프트에서 열린 [댓글 필드](/docs/ko/permissions#add-a-comment-when-you-answer-a-permission-prompt)를 닫습니다. 열린 필드가 없으면 세션의 나머지 부분에 대해 작업을 허용하는 옵션을 선택합니다(프롬프트가 해당 옵션을 제공하는 경우) |

\*VT 모드가 없는 Windows에서(Node \<24.2.0/\<22.17.0, Bun \<1.2.23), 기본값은 Meta+M입니다.

v2.1.257 이전에는 `confirm:toggleExplanation` 작업이 Ctrl+E에 바인딩되어 있었으며 Bash 및 PowerShell 권한 프롬프트에서 모델이 생성한 명령 설명을 표시했습니다.

대화 상자는 예/아니오 질문을 하지 않을 때도 `confirm:yes` 및 `confirm:no`를 사용하여 수락하고 취소합니다. `y` 또는 `n`과 같은 단순 문자를 이 컨텍스트에 바인딩하면 해당 문자가 키로 표시되지 않는 대화 상자에도 작용합니다. `y` 및 `n`을 키로 표시하는 대화 상자는 해당 문자를 직접 읽고 바인딩이 필요하지 않습니다.

이 예제는 `y`를 `confirm:yes`에, `n`을 `confirm:no`에 바인딩합니다:

```json theme={null}
{
  "bindings": [
    {
      "context": "Confirmation",
      "bindings": {
        "y": "confirm:yes",
        "n": "confirm:no"
      }
    }
  ]
}
```

이러한 바인딩을 사용하면 [텍스트 필드](#text-fields)에 포커스가 있을 때 `y` 및 `n`은 여전히 문자로 입력됩니다.

v2.1.280 이전에는 `y`도 `confirm:yes`에, `n`도 `confirm:no`에 기본적으로 바인딩되었습니다. v2.1.280 이전에 `/keybindings`로 `keybindings.json`을 생성한 경우 파일에 두 바인딩이 모두 나열되며 해당 두 줄을 삭제할 때까지 유효합니다.

<h3 id="permission-actions">
  권한 작업
</h3>

권한 대화 상자의 `Confirmation` 컨텍스트에서 사용 가능한 작업:

| 작업                       | 기본값       | 설명                                                                   |
| :----------------------- | :-------- | :------------------------------------------------------------------- |
| `permission:toggleDebug` | (바인딩 해제됨) | 권한 디버그 정보 전환. v2.1.146에서 `app:exit`를 가렸기 때문에 이전 기본값인 Ctrl+D가 제거되었습니다 |

<h3 id="transcript-actions">
  트랜스크립트 작업
</h3>

`Transcript` 컨텍스트에서 사용 가능한 작업:

| 작업                         | 기본값               | 설명           |
| :------------------------- | :---------------- | :----------- |
| `transcript:toggleShowAll` | Ctrl+E            | 모든 콘텐츠 표시 전환 |
| `transcript:exit`          | q, Ctrl+C, Escape | 트랜스크립트 보기 종료 |

`transcript:toggleShowAll`은 클래식 렌더러에만 적용됩니다. [전체 화면 렌더링](/docs/ko/fullscreen)에서 트랜스크립트 뷰어는 모두 표시 전환을 제공하지 않습니다.

<h3 id="history-search-actions">
  기록 검색 작업
</h3>

`HistorySearch` 컨텍스트에서 사용 가능한 작업:

| 작업                         | 기본값         | 설명                    |
| :------------------------- | :---------- | :-------------------- |
| `historySearch:next`       | Ctrl+R      | 다음 일치 항목              |
| `historySearch:accept`     | Escape, Tab | 선택 수락                 |
| `historySearch:cancel`     | Ctrl+C      | 검색 취소                 |
| `historySearch:execute`    | Enter       | 선택한 명령 실행             |
| `historySearch:cycleScope` | Ctrl+S      | 범위 순환: 세션, 프로젝트, 모든 곳 |

`historySearch:next`, `historySearch:accept`, `historySearch:cancel` 및 `historySearch:execute` 기본값은 클래식 렌더러의 인라인 기록 검색에 적용되며, 이는 항상 모든 프로젝트의 프롬프트를 검색합니다. `historySearch:cycleScope`는 [전체 화면 렌더링](/docs/ko/fullscreen)에서만 적용되며, 여기서 Ctrl+R은 검색 대화 상자를 열고 Ctrl+S는 범위를 순환합니다. 대화 상자의 다른 키는 고정되어 있으며 다시 바인딩할 수 없습니다: Enter 또는 Tab은 강조 표시된 일치 항목을 프롬프트 입력에 배치하고 Esc는 취소합니다.

<h3 id="task-actions">
  작업 작업
</h3>

`Task` 컨텍스트에서 사용 가능한 작업:

| 작업                | 기본값                   | 설명                                                        |
| :---------------- | :-------------------- | :-------------------------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | 현재 작업을 백그라운드로 이동합니다. Ctrl+X Ctrl+B 조합은 tmux 접두사 충돌을 방지합니다 |

<h3 id="theme-actions">
  테마 작업
</h3>

`ThemePicker` 컨텍스트에서 사용 가능한 작업:

| 작업                               | 기본값    | 설명       |
| :------------------------------- | :----- | :------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T | 구문 강조 전환 |

<h3 id="help-actions">
  도움말 작업
</h3>

`Help` 컨텍스트에서 사용 가능한 작업:

| 작업             | 기본값    | 설명        |
| :------------- | :----- | :-------- |
| `help:dismiss` | Escape | 도움말 메뉴 닫기 |

<h3 id="tabs-actions">
  탭 작업
</h3>

`Tabs` 컨텍스트에서 사용 가능한 작업:

| 작업              | 기본값             | 설명   |
| :-------------- | :-------------- | :--- |
| `tabs:next`     | Tab, Right      | 다음 탭 |
| `tabs:previous` | Shift+Tab, Left | 이전 탭 |

<h3 id="attachments-actions">
  첨부 파일 작업
</h3>

`Attachments` 컨텍스트에서 사용 가능한 작업:

| 작업                     | 기본값               | 설명             |
| :--------------------- | :---------------- | :------------- |
| `attachments:next`     | Right             | 다음 첨부 파일       |
| `attachments:previous` | Left              | 이전 첨부 파일       |
| `attachments:remove`   | Backspace, Delete | 선택한 첨부 파일 제거   |
| `attachments:exit`     | Down, Escape      | 첨부 파일 네비게이션 종료 |

<h3 id="footer-actions">
  바닥글 작업
</h3>

`Footer` 컨텍스트에서 사용 가능한 작업:

| 작업                      | 기본값               | 설명                                                                                                             |
| :---------------------- | :---------------- | :------------------------------------------------------------------------------------------------------------- |
| `footer:next`           | Right             | 다음 바닥글 항목                                                                                                      |
| `footer:previous`       | Left              | 이전 바닥글 항목                                                                                                      |
| `footer:up`             | Up                | 바닥글에서 위로 네비게이션(맨 위에서 선택 해제)                                                                                    |
| `footer:down`           | Down              | 바닥글에서 아래로 네비게이션                                                                                                |
| `footer:openSelected`   | Enter             | 선택한 바닥글 항목 열기                                                                                                  |
| `footer:clearSelection` | Escape            | 바닥글 선택 지우기                                                                                                     |
| `footer:dismiss`        | Backspace, Delete | 바닥글에서 선택한 [아티팩트](/docs/ko/artifacts) 링크를 제거합니다. 게시된 아티팩트 자체는 영향을 받지 않습니다. 다른 바닥글 행에서는 이 키가 효과가 없습니다. v2.1.217 이상 필요 |

바닥글 항목이 선택되어 있는 동안(예: 프롬프트 아래의 에이전트 패널의 행), `Enter`는 `Chat` 컨텍스트에서 `Enter`를 `chat:queueSubmit` 또는 `chat:newline`으로 다시 바인딩한 경우에도 이를 엽니다.

`Chat` 바인딩은 `Footer` 컨텍스트가 바인딩하지 않는 키(예: `chat:cycleMode`의 Shift+Tab)에서 항목이 선택되어 있는 동안에도 계속 작동합니다.

<h3 id="message-selector-actions">
  메시지 선택기 작업
</h3>

`MessageSelector` 컨텍스트에서 사용 가능한 작업:

| 작업                       | 기본값                                       | 설명          |
| :----------------------- | :---------------------------------------- | :---------- |
| `messageSelector:up`     | Up, K, Ctrl+P                             | 목록에서 위로 이동  |
| `messageSelector:down`   | Down, J, Ctrl+N                           | 목록에서 아래로 이동 |
| `messageSelector:top`    | Ctrl+Up, Shift+Up, Meta+Up, Shift+K       | 맨 위로 이동     |
| `messageSelector:bottom` | Ctrl+Down, Shift+Down, Meta+Down, Shift+J | 맨 아래로 이동    |
| `messageSelector:select` | Enter                                     | 메시지 선택      |

<h3 id="diff-actions">
  Diff 작업
</h3>

`DiffDialog` 컨텍스트에서 사용 가능한 작업:

| 작업                    | 기본값       | 설명                                                                                                    |
| :-------------------- | :-------- | :---------------------------------------------------------------------------------------------------- |
| `diff:dismiss`        | Escape    | Diff 뷰어 닫기; 세부 정보 보기에서는 파일 목록으로 돌아갑니다                                                                 |
| `diff:previousSource` | Left      | 이전 diff 소스                                                                                            |
| `diff:nextSource`     | Right     | 다음 diff 소스                                                                                            |
| `diff:previousFile`   | Up, K     | 파일 목록의 이전 파일; 세부 정보 보기에서 한 줄 위로 스크롤                                                                   |
| `diff:nextFile`       | Down, J   | 파일 목록의 다음 파일; 세부 정보 보기에서 한 줄 아래로 스크롤                                                                  |
| `diff:viewDetails`    | Enter     | Diff 세부 정보 보기                                                                                         |
| `diff:back`           | (바인딩 해제됨) | Diff 뷰어에서 뒤로 이동. Escape는 `diff:dismiss`를 통해 뒤로 작업을 수행합니다. 세부 정보 보기에서 이전 기본값인 Left는 v2.1.203에서 제거되었습니다 |

Diff 세부 정보 보기는 또한 페이저 스타일 키를 표준 [스크롤 작업](#scroll-actions)에 바인딩합니다. 이러한 바인딩은 `DiffDialog` 컨텍스트의 일부이며 세부 정보 보기에만 적용됩니다. [스크롤 작업](#scroll-actions) 아래에 나열된 `Scroll` 컨텍스트 기본값은 변경되지 않습니다.

| 작업                    | 기본값            | 설명                |
| :-------------------- | :------------- | :---------------- |
| `scroll:pageUp`       | PageUp         | 뷰포트의 절반만큼 위로 스크롤  |
| `scroll:pageDown`     | PageDown       | 뷰포트의 절반만큼 아래로 스크롤 |
| `scroll:fullPageUp`   | Shift+Space, B | 전체 뷰포트만큼 위로 스크롤   |
| `scroll:fullPageDown` | Space          | 전체 뷰포트만큼 아래로 스크롤  |
| `scroll:top`          | G, Home        | 맨 위로 이동           |
| `scroll:bottom`       | Shift+G, End   | 맨 아래로 이동          |

<h3 id="diff-panel-actions">
  Diff 패널 작업
</h3>

`/diff`가 전체 화면 렌더링에서 열리는 [diff 패널](/docs/ko/interactive-mode#diff-panel)에 대한 작업입니다. `app:cycleDiffBase`는 패널이 열려 있는 동안 활성화되는 `DiffPanel` 컨텍스트에 있습니다. 나머지는 `Global`입니다. 패널에는 Claude Code v2.1.260 이상이 필요합니다.

| 작업                          | 기본값                  | 설명                                        |
| :-------------------------- | :------------------- | :---------------------------------------- |
| `app:toggleReplTab`         | (바인딩 해제됨)            | Diff 패널을 열거나 닫습니다. `/diff` 실행과 동일합니다      |
| `app:cycleDiffBase`         | Ctrl+X B             | 패널의 비교 기준을 순환합니다: 이 세션, 커밋되지 않음, 그 다음 브랜치 |
| `app:diffFileListUp`        | Ctrl+Up, Meta+Up     | 패널의 파일 목록이 오버플로우할 때 위로 스크롤                |
| `app:diffFileListDown`      | Ctrl+Down, Meta+Down | 패널의 파일 목록이 오버플로우할 때 아래로 스크롤               |
| `app:toggleDiffNoiseFilter` | (바인딩 해제됨)            | 패널에서 테스트 및 생성된 파일 표시 또는 숨기기               |
| `app:toggleDiffPreSession`  | (바인딩 해제됨)            | 이 세션 이전의 변경 사항 확장 또는 축소                   |

<h3 id="model-picker-actions">
  모델 선택기 작업
</h3>

`ModelPicker` 컨텍스트에서 사용 가능한 작업:

| 작업                            | 기본값   | 설명                   |
| :---------------------------- | :---- | :------------------- |
| `modelPicker:decreaseEffort`  | Left  | 노력 수준 감소             |
| `modelPicker:increaseEffort`  | Right | 노력 수준 증가             |
| `modelPicker:thisSessionOnly` | s     | 강조 표시된 모델을 이 세션에만 적용 |

<h3 id="effort-slider-actions">
  노력 슬라이더 작업
</h3>

`EffortSlider` 컨텍스트에서 사용 가능한 작업입니다. 이는 인수 없이 `/effort`를 실행할 때 열리는 슬라이더입니다. 슬라이더의 Left, Right, Enter 및 Escape 키는 다시 바인딩할 수 없습니다.

| 작업                             | 기본값 | 설명                                                                               |
| :----------------------------- | :-- | :------------------------------------------------------------------------------- |
| `effortSlider:thisSessionOnly` | s   | 포커스된 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 이 세션에만 적용합니다. v2.1.257 이상 필요 |

<h3 id="select-actions">
  선택 작업
</h3>

`Select` 컨텍스트에서 사용 가능한 작업:

| 작업                | 기본값             | 설명              |
| :---------------- | :-------------- | :-------------- |
| `select:next`     | Down, J, Ctrl+N | 다음 옵션           |
| `select:previous` | Up, K, Ctrl+P   | 이전 옵션           |
| `select:pageUp`   | PageUp          | 옵션 한 페이지 위로 이동  |
| `select:pageDown` | PageDown        | 옵션 한 페이지 아래로 이동 |
| `select:first`    | Home            | 첫 번째 옵션         |
| `select:last`     | End             | 마지막 옵션          |
| `select:accept`   | Enter           | 선택 수락           |
| `select:cancel`   | Escape          | 선택 취소           |

Claude Code는 `/skills` 메뉴에서 `select:pageUp`, `select:pageDown`, `select:first` 및 `select:last` 바인딩을 적용합니다. `/model` 선택기와 같은 대부분의 다른 목록에서 Claude Code는 바인딩에 관계없이 PageUp 및 PageDown으로 페이징하고 Home 및 End를 무시합니다.

v2.1.280 이전에는 이러한 다른 목록들이 Home, End 및 `select:first`와 `select:last` 바인딩을 무시했습니다.

<h3 id="plugin-actions">
  플러그인 작업
</h3>

`Plugin` 컨텍스트에서 사용 가능한 작업:

| 작업                | 기본값   | 설명                                        |
| :---------------- | :---- | :---------------------------------------- |
| `plugin:toggle`   | Space | 플러그인 선택 전환                                |
| `plugin:install`  | I     | 선택한 플러그인 설치                               |
| `plugin:favorite` | F     | 선택한 플러그인을 즐겨찾기로 설정하여 설치된 탭 상단 근처에 정렬되도록 함 |

<h3 id="settings-actions">
  설정 작업
</h3>

`Settings` 컨텍스트에서 사용 가능한 작업입니다. `select:accept` 및 `confirm:no` 작업은 [선택 작업](#select-actions) 및 [확인 작업](#confirmation-actions) 컨텍스트에서 재사용되며 설정별 동작을 가집니다: 변경 사항은 변경하는 즉시 각 설정에 적용되므로 Escape는 변경 사항을 거부하는 대신 변경 사항을 저장하고 패널을 닫습니다.

| 작업                | 기본값          | 설명                       |
| :---------------- | :----------- | :----------------------- |
| `settings:search` | /            | 검색 모드 진입                 |
| `settings:retry`  | R            | 사용량 데이터 다시 로드(오류 시)      |
| `select:accept`   | Enter, Space | 선택한 설정을 변경하거나 해당 서브메뉴 열기 |
| `confirm:no`      | Escape       | 패널 닫기. 변경 사항은 이미 저장됨     |

<h3 id="agents-actions">
  에이전트 작업
</h3>

`Agents` 컨텍스트에서 사용 가능한 작업입니다. 이는 `claude agents`로 열리는 [에이전트 보기](/docs/ko/agent-view)에 적용됩니다. v2.1.257 이상 필요합니다.

| 작업                  | 기본값    | 설명                                                           |
| :------------------ | :----- | :----------------------------------------------------------- |
| `agents:switchView` | Ctrl+S | [세션 그룹화](/docs/ko/agent-view#organize-the-list)를 상태와 디렉토리 사이에서 전환 |
| `agents:togglePin`  | Ctrl+T | 선택한 세션 [고정 또는 고정 해제](/docs/ko/agent-view#organize-the-list)       |

에이전트 보기가 열려 있는 동안 Claude Code는 `Agents` 컨텍스트가 바인딩하는 모든 키에 대해 `Agents` 바인딩을 사용하고 동일한 키의 `Chat` 또는 `Global` 바인딩을 무시합니다. 예를 들어 에이전트 보기에서 Ctrl+S를 누르면 기본 `chat:stash` 대신 세션 그룹화를 전환합니다.

디스패치 입력의 외부 편집기 단축키는 `Agents` 작업이 아닙니다. 에이전트 보기는 `Chat` 컨텍스트의 `chat:externalEditor` 바인딩(기본값 Ctrl+G)을 따릅니다.

바인딩은 에이전트 보기에서 단일 키 입력으로 실행되므로 `chat:externalEditor`에 바인딩된 Ctrl+X Ctrl+E 조합은 편집기를 열지 않습니다.

<h3 id="voice-actions">
  음성 작업
</h3>

[음성 받아쓰기](/docs/ko/voice-dictation)가 활성화되었을 때 `Chat` 컨텍스트에서 사용 가능한 작업:

| 작업                 | 기본값   | 설명                                 |
| :----------------- | :---- | :--------------------------------- |
| `voice:pushToTalk` | Space | 프롬프트를 받아쓰기. `/voice` 모드에 따라 누르거나 탭 |

<h3 id="scroll-actions">
  스크롤 작업
</h3>

[전체 화면 렌더링](/docs/ko/fullscreen)이 활성화되었을 때 `Scroll` 컨텍스트에서 사용 가능한 작업:

| 작업                          | 기본값                  | 설명                                                                       |
| :-------------------------- | :------------------- | :----------------------------------------------------------------------- |
| `scroll:lineUp`             | `wheelup`            | 한 줄 위로 스크롤합니다. 마우스 휠 스크롤이 이 작업을 트리거합니다                                   |
| `scroll:lineDown`           | `wheeldown`          | 한 줄 아래로 스크롤합니다. 마우스 휠 스크롤이 이 작업을 트리거합니다                                  |
| `scroll:pageUp`             | PageUp               | 뷰포트 높이의 절반만큼 위로 스크롤                                                      |
| `scroll:pageDown`           | PageDown             | 뷰포트 높이의 절반만큼 아래로 스크롤                                                     |
| `scroll:top`                | Ctrl+Home            | 대화의 시작으로 이동                                                              |
| `scroll:bottom`             | Ctrl+End             | 최신 메시지로 이동하고 자동 팔로우 다시 활성화                                               |
| `scroll:halfPageUp`         | (바인딩 해제됨)            | 뷰포트 높이의 절반만큼 위로 스크롤합니다. `scroll:pageUp`과 동일한 동작이며 vi 스타일 재바인딩을 위해 제공됨    |
| `scroll:halfPageDown`       | (바인딩 해제됨)            | 뷰포트 높이의 절반만큼 아래로 스크롤합니다. `scroll:pageDown`과 동일한 동작이며 vi 스타일 재바인딩을 위해 제공됨 |
| `scroll:fullPageUp`         | (바인딩 해제됨)            | 전체 뷰포트 높이만큼 위로 스크롤                                                       |
| `scroll:fullPageDown`       | (바인딩 해제됨)            | 전체 뷰포트 높이만큼 아래로 스크롤                                                      |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | 선택한 텍스트를 클립보드에 복사                                                        |
| `selection:clear`           | (바인딩 해제됨)            | 활성 텍스트 선택 지우기. v2.1.234 이상 필요                                            |
| `selection:extendLeft`      | Shift+Left           | 활성 선택을 한 열 왼쪽으로 확장                                                       |
| `selection:extendRight`     | Shift+Right          | 활성 선택을 한 열 오른쪽으로 확장                                                      |
| `selection:extendUp`        | Shift+Up             | 활성 선택을 한 행 위로 확장합니다. 선택이 상단 가장자리에 도달하면 뷰포트를 스크롤합니다                       |
| `selection:extendDown`      | Shift+Down           | 활성 선택을 한 행 아래로 확장합니다. 선택이 하단 가장자리에 도달하면 뷰포트를 스크롤합니다                      |
| `selection:extendLineStart` | Shift+Home           | 활성 선택을 줄의 시작으로 확장                                                        |
| `selection:extendLineEnd`   | Shift+End            | 활성 선택을 줄의 끝으로 확장                                                         |

<h2 id="keystroke-syntax">
  키 입력 구문
</h2>

<h3 id="modifiers">
  수정자
</h3>

`+` 구분자로 수정자 키를 사용합니다:

* `ctrl` 또는 `control` - Control 키
* `shift` - Shift 키
* `alt`, `opt`, `option`, 또는 `meta` - Windows 및 Linux의 Alt 키, macOS의 Option 키
* `cmd`, `command`, `super`, 또는 `win` - macOS의 Command 키, Windows의 Windows 키, Linux의 Super 키

`cmd` 그룹은 Kitty 키보드 프로토콜 또는 xterm의 `modifyOtherKeys` 모드를 지원하는 것과 같이 Super 수정자를 보고하는 터미널에서만 감지됩니다. 대부분의 터미널은 이를 전송하지 않으므로 모든 곳에서 작동하기를 원하는 바인딩에는 `ctrl` 또는 `meta`를 사용합니다.

예를 들어:

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Shift + Tab
meta+p          macOS의 Option + P, 다른 곳의 Alt + P
ctrl+shift+c    여러 수정자
```

<h3 id="uppercase-letters">
  대문자
</h3>

Claude Code는 키 이름을 대소문자를 구분하지 않고 파싱하므로 `K`는 `k`와 동일한 바인딩이고 `ctrl+K`는 `ctrl+k`와 동일합니다. Shift와 문자를 바인딩하려면 `shift+k`를 작성합니다.

<h3 id="non-us-keyboard-layouts">
  비US 키보드 레이아웃
</h3>

활성 키보드 레이아웃이 다른 문자를 입력하더라도 Ctrl 바로가기의 키 이름을 라틴 문자로 작성합니다.

Claude Code가 누른 키를 바인딩과 일치시키는 방식은 레이아웃의 종류에 따라 다릅니다:

* Cyrillic과 같은 비라틴 레이아웃에서 Claude Code는 터미널이 Kitty 키보드 프로토콜을 사용하고 해당 위치를 보고할 때 키의 US 레이아웃 위치로 Ctrl 바로가기를 일치시킵니다. 러시아어 레이아웃이 활성화된 그러한 터미널에서 Ctrl과 물리적 W 키를 누르면 `ctrl+w`가 트리거됩니다. 위치를 보고하지 않는 터미널에서 Claude Code는 키 입력에 대해 터미널이 전송하는 것과 일치합니다: ASCII 제어 코드는 라틴 바로가기를 트리거하고 Cyrillic 문자로 도착하는 키 입력은 바인딩과 일치하지 않습니다
* AZERTY와 같이 라틴 문자를 재배열하는 레이아웃에서 Claude Code는 키가 입력하는 문자와 일치하므로 Ctrl과 A로 표시된 키를 누르면 `ctrl+a`가 트리거됩니다

v2.1.247 이전에는 Ghostty, Kitty, WezTerm, iTerm2와 같이 Kitty 키보드 프로토콜을 사용하는 터미널에서 비라틴 레이아웃 아래의 Ctrl 바로가기를 누르면 해당 바인딩이 트리거되지 않았습니다.

<h3 id="chords">
  코드
</h3>

코드는 공백으로 구분된 키 입력 시퀀스입니다:

```text theme={null}
ctrl+k ctrl+s   Ctrl+K를 누르고 놓은 다음 Ctrl+S를 누릅니다
```

각 키 입력을 이전 키 입력 후 3초 이내에 누릅니다. 더 오래 기다리면 Claude Code는 코드를 취소하고 그렇게 했다는 짧은 알림을 표시합니다.

<h3 id="special-keys">
  특수 키
</h3>

* `escape` 또는 `esc` - Escape 키
* `enter` 또는 `return` - Enter 키
* `tab` - Tab 키
* `space` - 스페이스바
* `up`, `down`, `left`, `right` - 화살표 키
* `pageup`, `pagedown` - Page Up 및 Page Down 키
* `home`, `end` - Home 및 End 키
* `backspace`, `delete` - Delete 키
* `wheelup`, `wheeldown` - 마우스 휠 스크롤 이벤트

<h2 id="unbind-default-shortcuts">
  기본 단축키 바인딩 해제
</h2>

작업을 `null`로 설정하여 기본 단축키 바인딩을 해제합니다:

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

이는 코드 바인딩에도 작동합니다. 접두사를 공유하는 모든 코드를 바인딩 해제하면 해당 접두사를 단일 키 바인딩으로 사용할 수 있습니다. 활성 컨텍스트의 코드는 접두사를 예약된 상태로 유지하므로 해당 코드를 정의하는 컨텍스트에서 각 코드를 바인딩 해제해야 합니다.

Claude Code는 `ctrl+x` 접두사에서 다음과 같은 기본 코드를 바인딩합니다: `Chat`에서 `ctrl+x ctrl+k`, `ctrl+x ctrl+e`, `ctrl+x enter`, `ctrl+x ctrl+a`, `ctrl+x ctrl+s`, `ctrl+x tab`, `Task`에서 `ctrl+x ctrl+b`, `DiffPanel`에서 `ctrl+x b`. `ctrl+x enter` 코드는 v2.1.247 이상이 필요하며, `ctrl+x b`, `ctrl+x ctrl+a`, `ctrl+x tab`은 v2.1.260 이상이 필요하고, `ctrl+x ctrl+s`는 v2.1.275 이상이 필요합니다.

`ctrl+x` 자체를 단일 키 바인딩으로 되찾으려면 모두 바인딩 해제합니다:

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "DiffPanel",
      "bindings": {
        "ctrl+x b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x enter": null,
        "ctrl+x ctrl+a": null,
        "ctrl+x ctrl+s": null,
        "ctrl+x tab": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

접두사의 일부 코드만 바인딩 해제하고 모두 바인딩 해제하지 않으면 접두사를 누르면 여전히 남은 바인딩에 대한 코드 대기 모드로 진입합니다.

<h2 id="reserved-shortcuts">
  예약된 단축키
</h2>

이러한 단축키는 다시 바인딩할 수 없습니다:

| 단축키       | 이유                                                                                                                                                                                                    |
| :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ctrl+C    | 하드코딩된 중단/취소                                                                                                                                                                                           |
| Ctrl+D    | 하드코딩된 종료                                                                                                                                                                                              |
| Ctrl+M    | Claude Code는 항상 이를 Enter로 수신합니다                                                                                                                                                                       |
| Ctrl+\[   | Claude Code는 항상 이를 Escape로 수신합니다. Kitty 키보드 프로토콜을 사용하는 터미널에서는 v2.1.242 이상이 필요합니다                                                                                                                      |
| Ctrl+I    | Claude Code는 항상 이를 Tab으로 수신합니다                                                                                                                                                                        |
| Ctrl+H    | ASCII 백스페이스 바이트를 전송합니다. [Claude Code가 Windows에서 이를 읽는 방식](/docs/ko/terminal-config#fix-backspace-deleting-a-whole-word-on-windows)은 터미널과 [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE`](/docs/ko/env-vars) 환경 변수에 따라 다릅니다 |
| Caps Lock | 터미널 애플리케이션에 전달되지 않음                                                                                                                                                                                   |

<h2 id="terminal-conflicts">
  터미널 충돌
</h2>

일부 단축키는 터미널 멀티플렉서와 충돌할 수 있습니다:

| 단축키    | 충돌                       |
| :----- | :----------------------- |
| Ctrl+B | tmux 접두사(두 번 눌러서 보내기)    |
| Ctrl+A | GNU screen 접두사           |
| Ctrl+Z | Unix 프로세스 일시 중단(SIGTSTP) |

<h2 id="text-fields">
  텍스트 필드
</h2>

맨 글자, 숫자 또는 스페이스를 바인딩하면, 대화 상자나 패널 내의 텍스트 필드에서 여전히 해당 문자를 입력할 수 있습니다. 이러한 필드 중 하나는 Claude가 질문하는 `Other` 답변입니다. 필드에 포커스가 있는 동안, Ctrl, Alt 또는 Cmd 없이 누르는 인쇄 가능한 키는 필드로 이동하며, Claude Code는 바인딩과 일치시키지 않습니다.

이러한 키는 필드에 포커스가 있는 동안에도 바인딩을 실행합니다:

* Enter, Escape, Tab 및 화살표 키와 같이 문자를 입력하지 않는 키
* Ctrl, Alt 또는 Cmd와 함께 누르는 모든 키
* 이미 진행 중인 [코드](#chords)의 두 번째 키 입력

주 프롬프트에서 Claude Code는 모든 키를 `Chat`과 같은 활성 컨텍스트와 비교하고, 바인딩이 키를 가져가지 않을 때만 키를 입력합니다.

<h2 id="vim-mode-interaction">
  Vim 모드 상호 작용
</h2>

vim 모드가 `/config` → 편집기 모드를 통해 활성화되면 키바인딩과 vim 모드는 독립적으로 작동합니다:

* **Vim 모드**는 텍스트 입력 수준에서 입력을 처리합니다(커서 이동, 모드, 동작).
* **키바인딩**은 구성 요소 수준에서 작업을 처리합니다(작업 전환, 제출 등).
* vim 모드의 Escape 키는 INSERT를 NORMAL 모드로 전환합니다. `chat:cancel`을 트리거하지 않습니다.
* 대부분의 Ctrl+key 단축키는 vim 모드를 통과하여 키바인딩 시스템으로 이동합니다.
* Vim 키는 키바인딩 파일을 통해 다시 매핑할 수 없습니다. `jj`를 Escape로 매핑하는 것과 같은 두 키 INSERT 모드 시퀀스를 매핑하려면 [`vimInsertModeRemaps`](/docs/ko/interactive-mode#remap-insert-mode-key-sequences) 설정을 사용합니다.
* vim NORMAL 모드에서 `?`는 도움말 메뉴를 표시합니다(vim 동작).
* vim NORMAL 모드에서 `/`는 히스토리 검색을 열며, 표준 모드의 Ctrl+R과 동일합니다.

<h2 id="validation">
  유효성 검사
</h2>

Claude Code는 키바인딩을 검증하고 다음에 대한 경고를 표시합니다:

* 구문 분석 오류(잘못된 JSON 또는 구조)
* 잘못된 컨텍스트 이름
* 잘못된 작업 값(예: 문자열이 아니거나 `null`인 작업)
* 알 수 없는 작업 이름(예: 등록된 작업의 오타). Claude Code는 바인딩을 건너뛰고 해당 키에 대한 기본 바인딩을 유지합니다. v2.1.246 이전에는 알 수 없는 작업 이름이 있는 바인딩이 해당 키를 자동으로 비활성화했습니다
* 예약된 단축키 충돌
* 동일한 컨텍스트의 중복 바인딩

Claude Code는 파일이 로드될 때 경고를 보고하며 각각을 디버그 로그에 기록합니다. [`--debug`](/docs/ko/cli-reference#cli-flags)를 사용하여 Claude Code를 시작하면 세부 정보를 확인할 수 있습니다.
