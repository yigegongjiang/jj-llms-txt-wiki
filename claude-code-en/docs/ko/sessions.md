> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 세션 관리

> Claude Code 대화의 이름을 지정하고, 재개하고, 분기하고, 전환합니다. `--continue`, `--resume`, `--from-pr`, `/resume` 선택기, 세션 이름 지정, 대화 기록 내보내기 및 대화 기록 저장 위치를 다룹니다.

세션은 프로젝트 디렉토리에 연결된 저장된 대화입니다. Claude Code는 작업할 때 로컬에 저장하므로 중단한 지점부터 재개하거나, 다른 접근 방식을 시도하기 위해 분기하거나, 작업 간에 전환할 수 있습니다.

[데스크톱 앱](/docs/ko/desktop#work-in-parallel-with-sessions), [웹의 Claude Code](/docs/ko/claude-code-on-the-web), [VS Code 확장](/docs/ko/vs-code#resume-past-conversations)은 각각 자신의 세션 기록을 유지합니다. 이 페이지는 CLI를 다룹니다.

<h2 id="resume-a-session">
  세션 재개
</h2>

세션은 작업할 때 [로컬 대화 기록 파일](#export-and-locate-session-data)에 지속적으로 저장되므로 종료하거나 `/clear`를 실행한 후에 세션으로 돌아갈 수 있습니다. 다음 진입점을 사용합니다:

| 명령                          | 기능                                     |
| :-------------------------- | :------------------------------------- |
| `claude --continue`         | 현재 디렉토리에서 가장 최근 세션을 재개합니다              |
| `claude --resume`           | [세션 선택기](#use-the-session-picker)를 엽니다 |
| `claude --resume <name>`    | 지정된 이름의 세션을 직접 재개합니다                   |
| `claude --from-pr <number>` | 해당 풀 요청에 연결된 세션을 재개합니다                 |
| `/resume`                   | 활성 세션 내에서 다른 대화로 전환합니다                 |

[`claude -p`](/docs/ko/headless) 또는 [Agent SDK](/docs/ko/agent-sdk/overview)로 생성된 세션은 세션 선택기에 나타나지 않지만, 세션 ID를 `claude --resume <session-id>`에 전달하여 여전히 재개할 수 있습니다. 세션이 시작된 디렉토리에서 이를 실행합니다. 세션 ID 조회는 현재 프로젝트 디렉토리 및 해당 git worktree로 범위가 지정되므로, 다른 곳에서 생성된 세션은 `No conversation found with session ID: <session-id>`를 보고합니다.

<h3 id="where-the-session-picker-looks">
  세션 선택기가 찾는 위치
</h3>

세션은 프로젝트 디렉토리별로 저장됩니다. 기본적으로 세션 선택기는 현재 worktree의 대화형 세션과 `/add-dir`로 현재 디렉토리를 추가한 다른 곳에서 시작된 세션을 표시합니다. `Ctrl+W`를 사용하여 저장소의 모든 worktree로 확장하거나 `Ctrl+A`를 사용하여 이 머신의 모든 프로젝트로 확장합니다.

v2.1.169부터 [`/cd`](/docs/ko/commands)로 세션을 이동하면 새 디렉토리의 프로젝트 저장소로 재배치되므로 이후 해당 디렉토리의 선택기에 나타납니다. v2.1.196부터 이동된 세션은 충돌이나 강제 종료 후에도 이전 디렉토리의 선택기에서 제외된 상태로 유지됩니다. 이전 버전에서는 언더스코어와 같은 특수 문자가 포함된 이전 경로가 있을 때 깔끔하지 않은 종료 후 이전 디렉토리의 목록에 다시 나타날 수 있습니다.

같은 저장소의 다른 worktree에서 세션을 선택하면 그 위치에서 재개됩니다. 관련 없는 프로젝트에서 세션을 선택하면 `cd` 및 재개 명령을 클립보드에 복사합니다.

이름으로 재개하면 현재 저장소 및 해당 worktree 전체에서 확인됩니다. 두 형식 모두 정확한 일치를 찾고 다른 worktree에 있더라도 직접 재개합니다:

| 명령                       | 정확한 일치 | 모호한 이름                                        |
| :----------------------- | :----- | :-------------------------------------------- |
| `claude --resume <name>` | 직접 재개  | 이름을 검색어로 미리 채운 세션 선택기를 엽니다                    |
| `/resume <name>`         | 직접 재개  | 오류를 보고합니다. 세션 선택기를 열려면 인수 없이 `/resume`를 실행합니다 |

<h2 id="name-your-sessions">
  세션 이름 지정
</h2>

세션에 설명적인 이름을 지정하여 세션 선택기에서 찾을 수 있고 이름으로 재개할 수 있도록 합니다. 이는 여러 작업을 병렬로 진행할 때 가장 중요합니다.

| 시기       | 이름을 설정하는 방법                                                                                                        |
| :------- | :----------------------------------------------------------------------------------------------------------------- |
| 시작 시     | `claude -n auth-refactor`                                                                                          |
| 세션 중     | `/rename auth-refactor`. 이름은 프롬프트 표시줄에도 나타납니다                                                                      |
| 세션 선택기에서 | 세션을 강조 표시하고 `Ctrl+R`을 누릅니다                                                                                         |
| 계획 수락 시  | [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에서 계획을 수락하면 이미 설정하지 않은 경우 계획 내용에서 세션 이름을 지정합니다 |

세션의 이름이 지정되면 `claude --resume <name>` 또는 `/resume <name>`으로 돌아갑니다. worktree 전체에서 이름 확인이 어떻게 작동하는지는 [세션 재개](#resume-a-session)를 참조합니다.

이름을 지정하지 않은 대화형 세션도 시작할 때 기본 표시 이름을 받습니다. Claude Code v2.1.196 이상이 필요합니다. 기본값은 작업 디렉토리의 이름과 두 문자 접미사를 결합합니다(예: `my-app-3f`). 이는 [에이전트 보기](/docs/ko/agent-view) 및 `claude agents --json` 출력과 같은 실행 중인 세션의 목록에서 세션을 식별합니다.

기본값은 재개 핸들이 아닙니다: `claude --resume <name>`, `/resume <name>` 및 세션 선택기는 설정한 이름만 일치합니다. 세션의 이름을 지정하면 기본값이 바뀝니다.

<h2 id="use-the-session-picker">
  세션 선택기 사용
</h2>

세션 내에서 `/resume`을 실행하거나 인수 없이 `claude --resume`을 실행하여 대화형 세션 선택기를 엽니다. 다음 키보드 단축키를 사용하여 탐색, 검색 및 목록 확장:

| 단축키                          | 작업                                                                                                          |
| :--------------------------- | :---------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                    | 세션 간 탐색                                                                                                     |
| `→` / `←`                    | 그룹화된 세션 확장 또는 축소                                                                                            |
| `Enter`                      | 강조 표시된 세션 재개                                                                                                |
| `Space`                      | 세션 내용 미리보기. 터미널이 붙여넣기로 캡처하지 않는 경우 `Ctrl+V`도 작동합니다                                                           |
| `Ctrl+R`                     | 강조 표시된 세션 이름 바꾸기                                                                                            |
| `/` 또는 `Space` 이외의 인쇄 가능한 문자 | 검색 모드를 입력하고 세션을 필터링합니다. GitHub, GitHub Enterprise, GitLab 또는 Bitbucket 풀 또는 병합 요청 URL을 붙여넣어 이를 생성한 세션을 찾습니다 |
| `Ctrl+A`                     | 이 머신의 모든 프로젝트에서 세션을 표시합니다. 다시 누르면 현재 저장소로 돌아갑니다                                                             |
| `Ctrl+W`                     | 현재 저장소의 모든 worktree에서 세션을 표시합니다. 다시 누르면 현재 worktree로 돌아갑니다. 다중 worktree 저장소에서만 표시됩니다                        |
| `Ctrl+B`                     | 현재 git 분기의 세션으로 필터링합니다. 다시 누르면 모든 분기를 표시합니다                                                                 |
| `Esc`                        | 세션 선택기 또는 검색 모드를 종료합니다                                                                                      |

각 행은 설정된 경우 세션 이름을 표시하고, 그렇지 않으면 대화 요약 또는 첫 번째 프롬프트와 마지막 활동 이후 경과 시간, 메시지 수 및 git 분기를 표시합니다. `Ctrl+A`로 모든 프로젝트로 확장한 후 프로젝트 경로가 나타납니다.

`/branch`, `/rewind` 또는 `--fork-session`으로 생성된 분기된 세션은 루트 세션 아래에 그룹화됩니다. `→`를 눌러 그룹을 확장합니다.

<h2 id="branch-a-session">
  세션 분기
</h2>

분기는 지금까지의 대화 복사본을 만들고 이를 전환하여 원본은 그대로 유지합니다. 진행 중인 경로를 잃지 않고 다른 접근 방식을 시도하는 데 사용합니다.

세션 내에서 선택적 이름과 함께 `/branch`를 실행합니다:

```text theme={null}
/branch try-streaming-approach
```

이름을 생략하면 Claude Code는 대화의 첫 번째 프롬프트 이후로 새 분기의 이름을 지정합니다. v2.1.198부터 이는 [압축](/docs/ko/how-claude-code-works#when-context-fills-up) 이후에도 적용됩니다. 이전 버전은 압축 요약을 지나 원본 첫 번째 프롬프트를 찾는 대신 리터럴 이름 `Branched conversation`으로 폴백했습니다.

명령줄에서 `--continue` 또는 `--resume`을 `--fork-session`과 결합합니다:

```bash theme={null}
claude --continue --fork-session
```

원본 세션은 변경되지 않으며 세션 선택기에서 사용 가능하게 유지됩니다. `/branch` 확인은 두 개의 세션 ID를 인쇄합니다: 현재 있는 새 분기와 원본입니다. 원본으로 돌아가려면 해당 ID를 `/resume`에 전달하거나, 세션 선택기를 사용하거나, `/resume <original-name>`을 실행합니다. "이 세션에 대해 허용"으로 승인한 권한은 새 분기로 이월되지 않습니다. 분기 없이 두 터미널에서 같은 세션을 재개하면 두 터미널의 메시지가 하나의 대화 기록으로 인터리브됩니다.

단일 세션 내에서 체크포인트 기반 되감기는 [체크포인팅](/docs/ko/checkpointing)을 참조합니다.

<h2 id="manage-context-within-a-session">
  세션 내 컨텍스트 관리
</h2>

이 명령은 세션을 떠나지 않고 컨텍스트 윈도우에 있는 내용을 제어합니다:

* **`/clear`**: 빈 컨텍스트로 새로 시작합니다. 이전 대화는 저장되고 `/resume`으로 재개 가능하거나, 동일한 Claude Code 프로세스에서 [되감기 메뉴의 이전 세션 항목](/docs/ko/checkpointing#rewind-past-a-cleared-conversation)에서 재개할 수 있습니다
* **`/compact [instructions]`**: 기록을 요약으로 바꾸고, 선택적으로 지정한 내용에 초점을 맞춥니다
* **`/context`**: 현재 컨텍스트를 소비하는 것을 표시합니다

압축이 CLAUDE.md, 기술, 규칙과 상호 작용하는 방식은 [컨텍스트 윈도우 가이드](/docs/ko/context-window)를 참조합니다. 언제 지우기 대 압축을 사용할지에 대한 전략은 [모범 사례](/docs/ko/best-practices#manage-your-session)를 참조합니다.

<h2 id="export-and-locate-session-data">
  세션 데이터 내보내기 및 찾기
</h2>

`/export`를 실행하여 현재 대화를 클립보드에 복사하거나 일반 텍스트 파일로 저장합니다. 메시지와 도구 출력은 읽을 수 있는 텍스트로 렌더링됩니다. 파일 이름을 전달하여 해당 파일에 직접 작성합니다.

<h3 id="access-conversations-from-scripts">
  스크립트에서 대화 기록 접근
</h3>

`/export`는 사람이 읽을 수 있도록 렌더링된 기록을 생성합니다. 아래 인터페이스는 스크립트가 파싱할 수 있는 구조화된 데이터를 생성합니다. 실행 결과의 JSON, 세션의 기록 파일 경로, 또는 이벤트의 라이브 스트림입니다. 스크립트를 트리거하는 것에 따라 선택합니다:

* **Claude를 한 번 실행하고 결과 캡처**: [`--output-format json` 또는 `stream-json`](/docs/ko/headless#get-structured-output)과 함께 `claude -p`를 호출하여 비대화형 실행의 결과, 세션 ID, 사용량 및 비용을 구조화된 JSON으로 캡처합니다.
* **기존 세션에 질문하기**: 세션 ID를 [`claude -p --resume`](/docs/ko/headless#continue-conversations)에 전달하여 요약 요청과 같은 후속 프롬프트를 보내고 구조화된 응답을 캡처합니다.
* **세션 이벤트에 반응**: [hooks](/docs/ko/hooks#common-input-fields) 및 [상태 줄 명령](/docs/ko/statusline#available-data)이 입력으로 받는 `transcript_path` 필드를 읽습니다. `SessionEnd` hook은 세션이 끝날 때 기록을 보관할 수 있습니다.
* **TypeScript 또는 Python 앱에 Claude 포함**: [Agent SDK](/docs/ko/agent-sdk/overview)를 사용하여 각 메시지를 프로그래밍 방식으로 수신합니다.

아래 예제는 두 번째 인터페이스를 사용합니다. 기존 세션에 후속 프롬프트를 보내고 `jq`로 답변을 읽습니다:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  기록이 저장되는 위치
</h3>

기본적으로 기록은 `~/.claude/projects/<project>/<session-id>.jsonl`에 JSONL로 저장되며, 여기서 `<project>`는 작업 디렉토리 경로에서 파생되고 영숫자가 아닌 문자는 `-`로 대체됩니다. 각 줄은 메시지, 도구 사용 또는 메타데이터 항목에 대한 JSON 객체입니다. 항목 형식은 Claude Code 내부 형식이며 버전 간에 변경되므로 이러한 파일을 직접 파싱하는 스크립트는 모든 릴리스에서 손상될 수 있습니다. 세션 데이터를 기반으로 구축하려면 `/export` 또는 [스크립트 인터페이스](#access-conversations-from-scripts) 대신 사용합니다.

위치, 보존 및 쓰기 동작은 구성 가능합니다:

| 대상                     | 설정                                                     | 위치                      |
| ---------------------- | ------------------------------------------------------ | ----------------------- |
| `~/.claude` 외부로 저장소 이동 | [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)                    | 환경 변수                   |
| 30일 보존 기간 변경           | [`cleanupPeriodDays`](/docs/ko/settings#available-settings) | `settings.json`         |
| 모든 모드에서 기록 쓰기 억제       | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/ko/env-vars)      | 환경 변수                   |
| 한 번의 비대화형 실행에 대해 쓰기 억제 | [`--no-session-persistence`](/docs/ko/cli-reference)        | `claude -p`와 함께 CLI 플래그 |

<h2 id="see-also">
  참고 항목
</h2>

이 페이지들은 관련 세션 및 병렬 처리 메커니즘을 다룹니다:

* [Worktrees](/docs/ko/worktrees): 별도 분기에서 격리된 병렬 세션 실행
* [Checkpointing](/docs/ko/checkpointing): 코드 및 대화를 이전 지점으로 되감기
* [Context window](/docs/ko/context-window): 컨텍스트를 채우는 것과 압축 후 유지되는 것
* [Non-interactive mode](/docs/ko/headless): `claude -p` 아래의 세션 동작
