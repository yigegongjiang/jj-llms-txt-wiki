> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 권한 구성

> 세분화된 권한 규칙, 모드 및 관리형 정책을 통해 Claude Code가 액세스하고 수행할 수 있는 작업을 제어합니다.

Claude Code는 에이전트가 수행할 수 있는 작업과 수행할 수 없는 작업을 정확하게 지정할 수 있도록 세분화된 권한을 지원합니다. 권한 설정은 버전 제어에 체크인할 수 있으며 조직의 모든 개발자에게 배포할 수 있을 뿐만 아니라 개별 개발자가 사용자 정의할 수 있습니다.

<h2 id="permission-system">
  권한 시스템
</h2>

Claude Code는 강력함과 안전성의 균형을 맞추기 위해 계층화된 권한 시스템을 사용합니다. 다음 표는 각 도구 유형에 대해 수동 모드에서 작업 실행 전에 승인을 요청하는지 여부를 보여줍니다. 다른 [권한 모드](#permission-modes)는 이 중 어떤 것이 사용자에게 묻는지를 변경하며, [분류기가 작업을 평가하는 방식](/docs/ko/permission-modes#how-the-classifier-evaluates-actions)은 분류기가 보는 작업을 나열합니다.

| 도구 유형   | 예시            | 승인 필요                                                                        | "예, 다시 묻지 않기" 동작 |
| :------ | :------------ | :--------------------------------------------------------------------------- | :--------------- |
| 읽기 전용   | 파일 읽기, Grep   | 아니오, [작업 디렉토리 및 추가 디렉토리](#working-directories) 내에서                           | 해당 없음            |
| Bash 명령 | 셸 실행          | 예, [읽기 전용 명령](#read-only-commands)의 기본 제공 집합 제외                              | 저장소 및 명령당 영구적    |
| 파일 수정   | Edit/Write 파일 | 예                                                                            | 세션 종료까지          |
| 웹 가져오기  | WebFetch      | 예, [사전 승인된 설명서 도메인](/docs/ko/tools-reference#webfetch-tool-behavior)의 기본 제공 집합 제외 | 저장소 및 도메인당 영구적   |
| 웹 검색    | WebSearch     | 예                                                                            | 저장소당 영구적         |

"예, 다시 묻지 않기"를 선택하고 Bash 명령 또는 WebFetch 도메인과 같이 승인이 영구적으로 저장되는 경우, Claude Code는 규칙을 git 저장소의 루트에 있는 `.claude/settings.local.json`에 저장하며, [worktrees](/docs/ko/worktrees)를 통해 주 체크아웃으로 해결됩니다. 규칙은 해당 저장소의 모든 향후 세션에 적용되며, 하위 디렉토리 및 worktrees에서 시작된 세션도 포함됩니다. 파일 수정 승인은 파일에 저장되지 않습니다. 표에서 보듯이 세션 종료까지만 지속됩니다. git 저장소 외부 또는 Windows의 경우와 같은 일부 경우에는 Claude Code가 저장소 루트를 사용하지 않습니다. [Claude Code가 각 파일을 찾는 위치](/docs/ko/settings#where-claude-code-looks-for-each-file)는 이러한 경우와 규칙을 저장하는 위치를 나열합니다.

v2.1.211 이전에는 Claude Code가 항상 시작 디렉토리에 규칙을 저장했으므로 worktree 또는 하위 디렉토리에서 부여된 승인이 저장소의 나머지 부분에 적용되지 않았습니다. 이전 버전이 하위 디렉토리 또는 worktree에 저장한 규칙은 여전히 거기서 시작된 세션에 적용됩니다.

때때로 권한 프롬프트는 "다시 묻지 않기" 옵션이 없고 세션의 나머지 부분에 대해 작업을 허용하는 옵션도 없는 일회성 승인만 제공합니다. Claude Code는 프롬프트가 허용할 모든 것을 보여줄 수 있을 때만 이러한 옵션을 제공하므로 프롬프트에서 저장하는 규칙은 이름이 지정된 옵션만 포함합니다. 프롬프트가 일회성 승인만 제공하는 경우, 작업을 한 번 승인하거나 [`/permissions`](#manage-permissions)에서 규칙을 직접 추가합니다.

<h3 id="add-a-comment-when-you-answer-a-permission-prompt">
  권한 프롬프트에 답할 때 주석 추가
</h3>

작업을 승인하거나 거부할 때 Claude에 메모를 첨부할 수 있습니다. Bash, PowerShell, 파일 및 MCP 도구 프롬프트를 포함한 대부분의 권한 프롬프트에서 **예** 또는 **아니오**로 이동하고 `Tab`을 눌러 해당 옵션에서 주석 필드를 엽니다. WebFetch 및 브라우저 프롬프트는 필드를 제공하지 않습니다. 세션의 나머지 부분에 대해 작업을 허용하거나 규칙을 저장하는 옵션도 하나를 사용하지 않습니다.

필드가 열려 있으면 주석을 입력한 다음 다음 키 중 하나를 누릅니다:

* `Enter`: 주석이 첨부된 답변을 제출합니다. 필드를 비워두면 Claude Code는 주석 없이 답변을 제출합니다.
* `Tab`: 답변하지 않고 필드를 닫습니다. Claude Code는 입력한 텍스트를 유지하고 해당 옵션으로 답변하면 여전히 전송합니다.
* `Shift+Tab`: 파일 프롬프트(예: Edit 또는 Write 프롬프트)에서 `Tab`과 동일하게 필드를 닫습니다. v2.1.235 이전에는 필드 내에서 `Shift+Tab`을 누르면 세션의 나머지 부분에 대해 작업을 허용하는 옵션을 선택했으므로 Claude Code는 세션의 나머지 부분에 대해 작업을 승인하고 주석을 버렸습니다.

Claude Code는 답변 방식에 따라 주석을 다르게 전달합니다:

* **예**: Claude Code는 작업을 실행한 다음 결과 후에 Claude에 주석을 보냅니다.
* **아니오**: Claude Code는 주석을 거부 이유로 Claude에 보내고 Claude는 계속 작업합니다. 주 대화의 프롬프트에서 주석 없이 **아니오**를 선택하면 Claude Code는 턴을 중지합니다.

<h2 id="manage-permissions">
  권한 관리
</h2>

`/permissions`를 사용하여 Claude Code의 도구 권한을 보고 관리할 수 있습니다. 이 대화 상자는 모든 권한 규칙과 각 규칙이 출처한 `settings.json` 파일을 나열합니다. Claude가 작업 중일 때 이 대화 상자를 열 수 있습니다. 규칙을 추가하거나 제거하면 Claude Code는 같은 턴에서 Claude의 다음 도구 호출부터 변경 사항을 적용합니다. v2.1.234 이전에는 Claude Code가 턴이 끝날 때까지 명령을 대기열에 넣었습니다.

* **Allow** 규칙을 사용하면 Claude Code가 수동 승인 없이 지정된 도구를 사용할 수 있습니다.
* **Ask** 규칙은 Claude Code가 지정된 도구를 사용하려고 할 때마다 확인을 요청합니다.
* **Deny** 규칙은 Claude Code가 지정된 도구를 사용하지 못하도록 방지합니다.

규칙은 순서대로 평가됩니다: deny, ask, allow. 해당 순서의 첫 번째 일치 항목이 결과를 결정하며, 규칙 특이성은 순서를 변경하지 않습니다.

`Bash(aws *)`와 같은 광범위한 deny 규칙은 `Bash(aws s3 ls)`와 같은 더 좁은 allow 규칙과도 일치하는 호출을 포함하여 모든 일치하는 호출을 차단하므로, deny 규칙은 허용 목록 예외를 포함할 수 없습니다. ask와 allow 사이에도 동일한 우선순위가 적용됩니다: 일치하는 ask 규칙은 동일한 호출과도 일치하는 더 구체적인 allow 규칙이 있을 때도 프롬프트를 표시합니다.

Deny 규칙은 도구 이름을 지정하는지 또는 도구 내의 패턴 범위를 지정하는지에 따라 다르게 작동합니다. `Bash`와 같은 단순 도구 이름은 도구를 Claude의 컨텍스트에서 완전히 제거하므로 Claude는 이를 볼 수 없습니다. 세션 중간에 이러한 규칙을 추가하면 Claude는 다음 도구 호출부터 도구를 호출할 수 없습니다. [전체 도구 거부](/docs/ko/prompt-caching#denying-an-entire-tool)는 Claude가 이미 본 정의에 어떤 일이 발생하는지를 다룹니다. `Bash(rm *)`와 같은 범위 지정 규칙은 도구를 사용 가능하게 유지하고 Claude가 시도할 때 일치하는 호출을 차단합니다.

단순 이름 제거는 [`EndConversation`](/docs/ko/tools-reference#endconversation-tool-behavior)을 제외한 모든 도구에 적용됩니다: deny 규칙은 다른 도구가 남아 있는 동안 이를 제거할 수 없으며, ask 규칙은 이에 대해 프롬프트를 표시하지 않습니다.

<Note>
  권한 규칙은 모델이 아닌 Claude Code에 의해 적용됩니다. 프롬프트 또는 `CLAUDE.md`의 지시사항은 Claude가 시도하는 작업을 형성하지만, Claude Code가 허용하는 것을 변경하지는 않습니다. 액세스 권한을 부여하거나 취소하려면 `/permissions`, 여기에 설명된 규칙, [권한 모드](/docs/ko/permission-modes), 또는 [PreToolUse hook](#extend-permissions-with-hooks)을 사용하십시오.
</Note>

[자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)가 세션에서 사용 가능할 때, 이 대화 상자에는 [자동 모드 분류기 규칙](/docs/ko/auto-mode-config#edit-rules-from-permissions)도 포함됩니다. **자동 모드** 탭을 선택하여 이들을 확인하십시오.

<h2 id="permission-modes">
  권한 모드
</h2>

Claude Code는 도구 승인 방식을 제어하는 여러 권한 모드를 지원합니다. [권한 모드](/docs/ko/permission-modes)에서 각 모드를 사용할 시기를 확인합니다. 세션이 시작되는 모드를 변경하려면 [설정 파일](/docs/ko/settings#where-settings-live)에서 `defaultMode`를 설정합니다. [세션이 시작되는 모드](/docs/ko/permission-modes#which-mode-a-session-starts-in)에서는 각 플랜의 기본 설정과 VS Code 확장이 읽는 내용을 다룹니다.

| 모드                  | 설명                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | 각 도구를 처음 사용할 때 권한을 요청합니다. CLI, VS Code 및 JetBrains 확장, 데스크톱 앱에서 Manual로 표시되며, Claude Code는 `manual`을 별칭으로 허용합니다. 레이블과 별칭은 Claude Code v2.1.200 이상이 필요합니다. 데스크톱 앱의 레이블은 CLI 버전에 따라 달라지지 않습니다                                                                                                                                                                                |
| `acceptEdits`       | 작업 디렉토리 또는 `additionalDirectories`의 경로에 대해 파일 편집 및 `mkdir`, `touch`, `mv`, `cp` 등의 일반적인 파일 시스템 명령을 자동으로 수락합니다                                                                                                                                                                                                                                                              |
| `plan`              | Claude는 파일을 읽고 읽기 전용 셸 명령을 실행하여 탐색하지만 소스 파일을 편집하지 않습니다. [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 사용할 수 있으며, 분류기 승인 명령도 실행됩니다. CLI 및 VS Code 확장에서 Plan으로 표시됩니다                                                                                                                                                                                       |
| `auto`              | 배경 안전 검사를 통해 도구 호출을 자동으로 승인하여 작업이 요청과 일치하는지 확인합니다                                                                                                                                                                                                                                                                                                                          |
| `dontAsk`           | 그 외에 프롬프트를 표시할 도구 호출을 자동으로 거부합니다. 작업 디렉토리의 파일 읽기 및 승인이 필요 없는 기타 작업은 계속 실행되며, `/permissions` 또는 `permissions.allow` 규칙을 통해 사전 승인된 도구도 실행됩니다. `AskUserQuestion`, [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구, 그리고 Claude Code에 도달하는 설정에서 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)한 커넥터 도구는 허용했더라도 거부됩니다 |
| `bypassPermissions` | 권한 프롬프트를 건너뜁니다. 단, [모든 모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)은 제외됩니다                                                                                                                                                                                                                                                                       |

<Warning>
  `bypassPermissions` 모드에서 Claude Code는 `.git` 및 `.claude`와 같은 [보호된 경로](/docs/ko/permission-modes#protected-paths)에 대한 쓰기를 포함한 권한 프롬프트를 건너뜁니다. [세션 간 메시징 보안 조치](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode)는 여전히 적용됩니다. 이 모드는 Claude Code가 손상을 일으킬 수 없는 컨테이너 또는 VM과 같은 격리된 환경에서만 사용합니다.
</Warning>

`bypassPermissions` 또는 `auto` 모드가 사용되는 것을 방지하려면 [설정 파일](/docs/ko/settings#where-settings-live)에서 `permissions.disableBypassPermissionsMode` 또는 `permissions.disableAutoMode`를 `"disable"`로 설정합니다. 이들은 재정의될 수 없는 [관리형 설정](#managed-settings)에서 가장 유용합니다.

<h2 id="permission-rule-syntax">
  권한 규칙 구문
</h2>

권한 규칙은 `Tool` 또는 `Tool(specifier)` 형식을 따릅니다. 지정자 내의 괄호는 리터럴이므로, 괄호를 포함하는 명령이나 경로는 이스케이프할 필요가 없습니다.

<h3 id="match-all-uses-of-a-tool">
  도구의 모든 사용 일치
</h3>

도구의 모든 사용을 일치시키려면 괄호 없이 도구 이름만 사용합니다:

| 규칙         | 효과                  |
| :--------- | :------------------ |
| `Bash`     | 모든 Bash 명령과 일치합니다   |
| `WebFetch` | 모든 웹 가져오기 요청과 일치합니다 |
| `Read`     | 모든 파일 읽기와 일치합니다     |

`Bash(*)`는 `Bash`와 동등하며 모든 Bash 명령과 일치합니다. 거부 규칙으로서, 두 형식 모두 Claude의 컨텍스트에서 도구를 제거합니다.

<h3 id="use-specifiers-for-fine-grained-control">
  세분화된 제어를 위해 지정자 사용
</h3>

괄호 안에 지정자를 추가하여 특정 도구 사용과 일치시킵니다:

| 규칙                             | 효과                            |
| :----------------------------- | :---------------------------- |
| `Bash(npm run build)`          | 정확한 명령 `npm run build`와 일치합니다 |
| `Read(./.env)`                 | 현재 디렉토리의 `.env` 파일 읽기와 일치합니다  |
| `WebFetch(domain:example.com)` | example.com으로의 가져오기 요청과 일치합니다 |

<h3 id="match-by-input-parameter">
  입력 매개변수로 일치
</h3>

거부 및 요청 규칙은 `Tool(param:value)`를 사용하여 모든 기본 제공 도구의 최상위 입력 매개변수와 일치할 수 있습니다.

MCP 도구의 매개변수와 일치시키려면 [`--disallowedTools`](/docs/ko/cli-reference#cli-flags)를 사용하여 거부 규칙을 전달합니다. Claude Code가 설정 파일을 로드할 때, 괄호가 있는 모든 `mcp__` 규칙을 건너뜁니다. Claude Code는 대화형 세션이 시작될 때 건너뛴 규칙을 유효하지 않은 설정 대화 상자에 나열하고, [`claude doctor`](/docs/ko/debug-your-config#check-resolved-settings) 출력에도 나열합니다.

매개변수 규칙은 Claude가 해당 매개변수가 정확한 값으로 설정된 도구를 호출할 때 일치합니다. 한 매개변수 값에 대한 허용 규칙은 호출이 전반적으로 안전하다는 것을 확립하지 않으므로, 허용 규칙은 각 도구의 자체 지정자 구문을 계속 사용합니다. 이는 도구가 허용하는 모든 스칼라 매개변수에 대해 작동합니다:

| 규칙                             | 일치                          |
| :----------------------------- | :-------------------------- |
| `Agent(model:opus)`            | Opus 모델 계층을 요청하는 Agent 호출   |
| `Agent(isolation:worktree)`    | git worktree를 요청하는 Agent 호출 |
| `Bash(run_in_background:true)` | 백그라운드에서 실행되는 Bash 호출        |

매개변수 일치는 다음 규칙을 따릅니다:

* 매개변수 이름은 Agent 도구의 `model`과 같이 도구 입력의 직접 필드여야 합니다. 객체 또는 배열 내에 중첩된 필드는 일치 가능하지 않습니다
* 각 규칙은 하나의 매개변수를 지정합니다. `model`과 `isolation` 모두에 대해 게이트하려면, 한 규칙에 결합하는 대신 `Agent(model:opus)` 및 `Agent(isolation:worktree)` 두 규칙을 작성합니다
* 값은 `*`를 와일드카드로 지원하여 모든 문자 시퀀스와 일치하므로, `Agent(isolation:*)`는 모든 명시적 격리 값과 일치합니다. `*` 없으면 일치는 정확합니다
* 모델이 생략한 매개변수는 절대 일치하지 않으므로, `Agent(model:*)`는 `model`을 설정하지 않은 호출과 일치하지 않습니다
* 값은 정규화 전에 Claude가 보내는 리터럴 입력과 비교됩니다. `Agent(model:opus)`는 별칭 `opus`와 일치하지만 전체 모델 ID와는 일치하지 않습니다. [`--verbose`](/docs/ko/cli-reference)로 실행하여 각 도구 호출의 정확한 매개변수 이름과 값을 확인합니다
* 콜론 주위의 공백은 무시됩니다

도구의 기본 콘텐츠 필드는 이 방식으로 일치 가능하지 않습니다: Bash 및 PowerShell의 `command`, Read, Edit 및 Write의 `file_path`, Grep 및 Glob의 `path`, NotebookEdit의 `notebook_path`, WebFetch의 `url`. `Bash(command:rm *)`와 같은 규칙은 복합 명령으로 우회 가능하므로, Claude Code는 이를 무시하고 시작 경고를 발생시킵니다. 대신 `Bash(rm *)`, `Read(./path)` 또는 `WebFetch(domain:host)`를 사용합니다.

<h3 id="wildcard-patterns">
  와일드카드 패턴
</h3>

Bash 규칙의 `*`는 공백을 포함한 모든 텍스트와 일치하므로, 한 규칙이 명령 계열을 포함합니다. `*`가 없는 규칙은 정확한 명령 하나와 일치합니다.

<Warning>
  `*`를 부분 명령 뒤에 놓습니다. `git log --oneline main`에서 `git`는 프로그램이고 `log`는 부분 명령이며, 프로그램이 수행하는 작업을 결정하는 단어입니다. Claude Code는 첫 번째 `*` 앞의 모든 것을 작성된 대로 일치시키므로, 이러한 단어들이 규칙을 제한하는 것입니다: `Bash(git log *)`는 `git log` 명령만 허용하고, `Bash(git *)`는 모든 git 명령을 허용합니다. Claude Code는 `Bash(git * main)`과 같이 부분 명령 앞에 `*`가 있는 허용 규칙에 대해 [시작 시 경고](/docs/ko/errors#has-a-wildcard-before-the-rest-of-the-command)합니다.
</Warning>

Claude가 묻지 않고 실행하기를 원하는 명령을 작성하고, 변하는 부분을 `*`로 바꿉니다. 이 구성을 사용하면 Claude Code는 npm 스크립트와 git 커밋을 묻지 않고 실행하고 `git push`로 시작하는 명령을 거부합니다. `git -C . push`처럼 다른 방식으로 작성된 push는 일치하지 않습니다. [Bash 규칙이 일치하지 않는 것](#bash-rule-limits)을 참조하세요.

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

`*`는 규칙의 어느 곳에나 올 수 있습니다: 시작, 중간 또는 끝. 각 행은 규칙, 일치하는 명령, 일치하지 않는 근처 명령을 보여줍니다:

| 작성 내용                  | 일치                                                                                   | 일치하지 않음                                |
| :--------------------- | :----------------------------------------------------------------------------------- | :------------------------------------- |
| `Bash(npm run build)`  | `npm run build`                                                                      | `npm run build --watch`                |
| `Bash(npm run *)`      | `npm run build`, `npm run test --watch`, `npm run`                                   | `npm install`                          |
| `Bash(git log * main)` | `git log --oneline main`, `git log -5 main`, `git log --output=<file> main`          | `git log main`, `git push origin main` |
| `Bash(git * main)`     | `git merge main`, `git push origin main`, `git -c core.fsmonitor=<script> diff main` | `git log`                              |
| `Bash(* --version)`    | `node --version`, `bash -c 'echo hi' --version`                                      | `node -v`                              |
| `Bash(ls *)`           | `ls -la`, `ls`                                                                       | `lsof`                                 |
| `Bash(ls*)`            | `ls -la`, `lsof`                                                                     |                                        |
| `Bash(* --help *)`     | `npm --help x`                                                                       | `npm --help`                           |

세 가지 일치 규칙이 이러한 행을 생성합니다:

* **`*`는 해당 위치의 모든 텍스트를 나타냅니다.** `Bash(git * main)`에서, 이는 부분 명령을 나타내므로 Claude Code는 모든 git 부분 명령과 그 앞의 모든 옵션과 일치합니다. 여기에는 git이 사용자가 지정한 프로그램을 실행하도록 하는 `-c`가 포함됩니다. `Bash(* --version)`에서, `*`는 프로그램을 나타내므로 모든 프로그램이 일치합니다.
* **끝에 있는 `*`와 그 앞의 공백은 또한 단순 명령과도 일치합니다.** `Bash(ls *)`는 `ls`와 일치하고, `Bash(git log *)`는 `git log`와 일치합니다. 이는 뒤에 오는 `*`가 규칙의 유일한 와일드카드일 때만 유지됩니다: `Bash(* --help *)`는 `npm --help x`와 일치하지만 `npm --help`와는 일치하지 않습니다.
* **뒤에 오는 `*` 앞의 공백은 규칙의 일부입니다.** `Bash(ls *)`는 `ls` 뒤의 공백을 요구하므로 `lsof`는 일치하지 않습니다. `Bash(ls*)`는 공백이 없으므로 `lsof`도 일치합니다.

`:*` 접미사는 뒤에 오는 와일드카드를 작성하는 동등한 방법이므로, `Bash(ls:*)`는 `Bash(ls *)` 와 동일한 명령과 일치합니다.

권한 대화 상자는 명령 접두사에 대해 "예, 다시 묻지 않기"를 선택할 때 공백으로 구분된 형식을 작성합니다. `:*` 형식은 패턴의 끝에서만 인식됩니다. `Bash(git:* push)`와 같은 패턴에서 콜론은 리터럴 문자로 취급되며 git 명령과 일치하지 않습니다.

<h3 id="tool-name-wildcards">
  도구 이름 와일드카드
</h3>

거부 및 요청 규칙은 도구 이름 위치에서도 glob 패턴을 허용합니다. 패턴은 전체 도구 이름과 일치해야 합니다: `"*"`는 모든 도구와 일치하며, `"mcp__*"`는 모든 서버의 모든 MCP 도구와 일치합니다. 단순 이름 glob 거부 규칙과 일치하는 도구는 Claude의 컨텍스트에서 제거되며, 이는 단순 도구 이름과 동일합니다. [`EndConversation`](/docs/ko/tools-reference#endconversation-tool-behavior) 예외를 포함하여: glob 거부는 다른 도구가 남아 있는 동안 이를 제거할 수 없으며, glob 요청은 절대 이에 대해 프롬프트하지 않습니다. 이 구성은 모든 MCP 도구를 거부합니다:

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

허용 규칙은 리터럴 `mcp__<server>__` 접두사 이후에만 도구 이름 glob을 허용합니다. 서버 세그먼트는 glob이 없어야 하므로 규칙이 구성한 특정 서버를 지정합니다. `mcp__puppeteer__*`는 `puppeteer` 서버의 모든 도구와 일치하며, `mcp__github__get_*`는 해당 `get_` 도구와 일치합니다. `"*"`, `"B*"`, 또는 `"mcp__*"`와 같은 고정되지 않은 허용 glob은 경고와 함께 건너뛰어지며 자동으로 승인되지 않습니다.

도구 이름이 알려진 도구와 일치하지 않는 거부 또는 요청 규칙은 오타를 포착하기 위해 시작 경고를 생성합니다. `_` 또는 `*`를 포함하는 도구 이름은 확인에서 제외됩니다. Claude Code가 제거한 도구의 이름(예: `TaskOutput`)도 확인에서 제외됩니다.

트랜스크립트 및 권한 대화 상자에서 도구에 대해 표시되는 레이블은 정규 이름과 다를 수 있습니다. 예를 들어, 트랜스크립트에서 `Stop Task`로 표시되는 도구의 정규 이름은 `TaskStop`입니다. 권한 규칙 및 [hook 매처](/docs/ko/hooks)는 레이블을 기준으로 일치시키지 않으므로, `Stop Task`로 작성된 규칙은 일치하지 않습니다. 거부 및 요청 규칙의 경우, 위의 시작 경고가 불일치를 포착합니다. [도구 참조](/docs/ko/tools-reference)에 나열된 정규 이름을 사용합니다.

<h2 id="tool-specific-permission-rules">
  도구별 권한 규칙
</h2>

<h3 id="bash">
  Bash
</h3>

Bash 규칙은 전체 명령 텍스트와 일치하며, `*`는 모든 텍스트를 나타냅니다. [와일드카드 패턴](#wildcard-patterns)은 각 규칙 형태가 일치하는 명령과 `*`를 어디에 배치할지 보여줍니다. 이 섹션의 나머지 부분은 Claude Code가 복합 명령과 래퍼를 어떻게 일치시키는지, 규칙이 일치하지 않는 것, 읽기 전용 명령 및 리다이렉션을 다룹니다.

<h4 id="compound-commands">
  복합 명령
</h4>

<Tip>
  Claude Code는 셸 연산자를 인식하므로 `Bash(safe-cmd *)`와 같은 규칙은 `safe-cmd && other-cmd` 명령을 실행할 권한을 부여하지 않습니다. 인식되는 명령 구분자는 `&&`, `||`, `;`, `|`, `|&`, `&` 및 줄바꿈입니다. 규칙은 각 서브명령과 독립적으로 일치해야 합니다.
</Tip>

Deny 및 ask 규칙은 서브셸 내에 중첩된 명령, 명령 치환 또는 `for` 루프와 같은 제어 흐름 본문 내의 명령을 포함하여 일치하는 모든 서브명령에 적용됩니다. `Bash(git clean *)`과 같은 ask 규칙은 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서도 `cd /tmp && git clean -f` 또는 `echo "$(git clean -f)"`에 대해 여전히 프롬프트합니다.

`&&` 또는 `||` 뒤에 아무것도 없을 때(예: `npm test &&`), Claude Code는 명령을 구문 분석 불가능한 것으로 취급하고 allow 규칙 일치를 위해 서브명령으로 분할하지 않으므로 `Bash(npm *)`과 같은 규칙은 이를 승인하지 않습니다.

"예, 다시 묻지 않기"로 복합 명령을 승인하면 Claude Code는 전체 복합 문자열에 대한 단일 규칙이 아니라 승인이 필요한 각 서브명령에 대해 별도의 규칙을 저장합니다. 예를 들어, `git status && npm test`를 승인하면 `npm test`에 대한 규칙을 저장하므로 향후 `npm test` 호출은 `&&` 앞에 무엇이 있든 인식됩니다. `cd`를 서브디렉토리로 이동하는 것과 같은 서브명령은 해당 경로에 대한 자체 Read 규칙을 생성합니다. 단일 복합 명령에 대해 최대 5개의 규칙이 저장될 수 있습니다.

<h4 id="process-wrappers">
  래퍼
</h4>

Bash 규칙을 일치시키기 전에 Claude Code는 고정된 래퍼 집합을 제거하므로 `Bash(npm test *)`와 같은 규칙은 `timeout 30 npm test`도 일치합니다. 제거되는 래퍼는 `timeout`, `time`, `nice`, `nohup` 및 `stdbuf`이며, 셸 내장 `command` 및 `builtin`, 그리고 zsh의 `noglob`입니다. 각각은 인수를 실제 명령으로 실행합니다. 두 가지 관련 형식은 제거되지 않습니다: 명령을 실행하지 않고 조회하는 쿼리 형식 `command -v`, 그리고 zsh의 `nocorrect`입니다.

Claude Code는 또한 특정 알려진 안전 환경 변수의 선행 할당을 제거하므로 `Bash(npm test *)`는 `NODE_ENV=test npm test`와 일치합니다. Allow 규칙은 다른 변수의 할당을 넘어 일치하지 않습니다. Deny 또는 ask 규칙은 모든 선행 할당을 넘어 일치하므로 deny의 `Bash(rm *)`은 여전히 `FOO=bar rm -rf tmp/`와 일치합니다.

베어 `xargs`도 제거되므로 `Bash(grep *)`는 `xargs grep pattern`과 일치합니다. 제거는 `xargs`에 플래그가 없을 때만 적용됩니다: `xargs -n1 grep pattern`과 같은 호출은 `xargs` 명령으로 일치되므로 내부 명령에 대해 작성된 규칙은 이를 포함하지 않습니다.

이 래퍼 목록은 기본 제공되며 구성할 수 없습니다. `direnv exec`, `devbox run`, `mise exec`, `npx` 및 `docker exec`과 같은 개발 환경 러너는 목록에 없습니다. 이러한 도구는 인수를 명령으로 실행하므로 `Bash(devbox run *)`와 같은 규칙은 `devbox run rm -rf .`를 포함하여 `run` 뒤에 오는 모든 것과 일치합니다. 환경 러너 내에서 작업을 승인하려면 `Bash(devbox run npm test)`와 같이 러너와 내부 명령을 모두 포함하는 특정 규칙을 작성합니다. 허용하려는 각 내부 명령에 대해 하나의 규칙을 추가합니다.

`watch`, `setsid`, `ionice` 및 `flock`과 같은 Exec 래퍼는 접두사 규칙 `Bash(watch *)`로 자동 승인될 수 없으므로 Manual 모드에서 항상 프롬프트합니다. 동일한 사항이 `-exec` 또는 `-delete`를 사용하는 `find`에 적용됩니다: `Bash(find *)` 규칙은 이러한 형식을 포함하지 않습니다. 특정 호출을 승인하려면 전체 명령 문자열에 대한 정확한 일치 규칙을 작성합니다.

<h4 id="bash-rule-limits">
  Bash 규칙이 일치하지 않는 것
</h4>

Bash 규칙은 Claude Code가 [복합 명령](#compound-commands)을 분할하고 [래퍼](#process-wrappers)를 제거한 후 Claude가 작성한 명령 텍스트와 일치합니다. 다른 형식으로 호출된 동일한 프로그램과는 일치하지 않으므로 deny 또는 ask 규칙은 Claude가 일반적으로 생성하는 호출을 중지하며 프로그램 주변의 보안 경계가 아닙니다. 이러한 규칙은 `deny` 또는 `ask`에서 첫 번째 형식을 중지하고 다른 형식은 중지하지 않습니다:

| 규칙                 | 중지                         | 중지하지 않음                                                                                               |
| :----------------- | :------------------------- | :---------------------------------------------------------------------------------------------------- |
| `Bash(curl *)`     | `curl https://example.com` | `/usr/bin/curl https://example.com`, `sh -c 'curl https://example.com'`                               |
| `Bash(rm *)`       | `rm -rf build/`            | `/bin/rm -rf build/`, `bash -c 'rm -rf build/'`                                                       |
| `Bash(git push *)` | `git push origin main`     | `git -C . push origin main`, `git -c push.default=current push origin main`, `git 'push' origin main` |

마지막 열의 명령이 어떻게 처리될지는 다른 규칙과 권한 모드가 결정합니다.

명령 텍스트에 의존하지 않는 파일 시스템 및 네트워크 적용을 위해 [샌드박싱](/docs/ko/sandboxing)을 사용합니다. 실행되기 전에 자신의 논리로 전체 명령 텍스트를 검사하려면 [PreToolUse 훅](#extend-permissions-with-hooks)을 사용합니다.

<h4 id="read-only-commands">
  읽기 전용 명령
</h4>

Claude Code는 기본 제공 Bash 명령 집합을 읽기 전용으로 인식하고 [`permissions.blockReadsOutsideWorkingDirectories`](/docs/ko/settings-reference#permissions-blockreadsoutsideworkingdirectories)가 차단하는 경로를 제외하고 모든 모드에서 권한 프롬프트 없이 실행합니다. 집합에는 `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` 및 `git`의 읽기 전용 형식이 포함됩니다. 집합은 구성할 수 없습니다. 이러한 명령 중 하나에 대해 프롬프트를 요구하려면 `ask` 또는 `deny` 규칙을 추가합니다. 자동 모드에서 이러한 명령은 또한 분류기의 검토를 기다릴 수 있습니다. [분류기가 작업을 평가하는 방법](/docs/ko/permission-modes#how-the-classifier-evaluates-actions)을 참조합니다.

`ls > out.txt`와 같은 리다이렉트는 대상에 대한 확인을 추가합니다. [리다이렉션](#redirections)을 참조합니다.

따옴표 없는 glob 패턴은 모든 플래그가 읽기 전용인 명령에 대해 허용되므로 `ls *.ts` 및 `wc -l src/*.py`는 프롬프트 없이 실행됩니다.

Manual 모드에서 이 집합의 명령은 다음 경우에 여전히 프롬프트합니다:

* **쓰기 가능한 플래그가 있는 명령의 따옴표 없는 glob**: `find`, `sort`, `sed` 및 `git`과 같이 쓰기 가능하거나 실행 가능한 플래그가 있는 명령은 glob이 `-delete`와 같은 플래그로 확장될 수 있으므로 따옴표 없는 glob이 있을 때 프롬프트합니다.
* **다른 데몬을 가리키는 `docker`**: `docker`의 읽기 전용 형식은 `-H`, `--context` 또는 Podman의 `--url` 및 `--connection`과 같이 다른 데몬을 선택하는 플래그를 전달할 때 프롬프트합니다.
* **경로 열기 플래그가 있는 `file`**: `file`은 `-m`/`--magic-file` 또는 `-f`/`--files-from`을 전달할 때 프롬프트합니다. 이러한 플래그는 `file`이 플래그 값에 명명된 경로를 열도록 합니다.
* **Windows의 네트워크 경로**: `\\server\share\file`과 같은 네트워크(UNC) 경로를 포함하는 인수가 있는 명령은 네트워크 경로에 액세스하면 Windows 자격 증명을 이름이 지정된 호스트로 보낼 수 있으므로 프롬프트합니다. 동일한 확인이 [PowerShell 도구](/docs/ko/tools-reference#powershell-tool) 명령에도 적용됩니다.
* **분석이 구문 분석할 수 없는 명령**: Claude Code가 명령을 완전히 구문 분석할 수 없을 때 명령을 읽기 전용으로 취급하지 않고 승인을 요청합니다. 10,000자를 초과하는 명령은 분석이 구문 분석하는 범위를 초과하므로 항상 프롬프트합니다.

작업 디렉토리 또는 [추가 디렉토리](#working-directories) 내의 경로로의 `cd`도 읽기 전용이며, `cd packages/api && ls`와 같은 복합 명령은 각 부분이 자체적으로 적격일 때 프롬프트 없이 실행됩니다. 이러한 조합은 각 부분이 읽기 전용이어도 프롬프트합니다:

* **`cd`와 `git`**: `cd`가 다른 디렉토리로 변경될 때 프롬프트합니다. 새 디렉토리에서 `git`을 실행하면 해당 디렉토리의 훅을 실행할 수 있기 때문입니다. 대상이 현재 작업 디렉토리로 해결되는 `cd`는 작동하지 않으며 프롬프트를 트리거하지 않습니다.
* **`cd`와 리다이렉트**: Claude Code가 `cd` 실행 후 리다이렉트 대상이 해결되는 디렉토리를 결정할 수 없을 때 프롬프트합니다. 유일한 리다이렉트 대상이 `/dev/null`인 명령(예: `cd app; grep -r pattern . 2>/dev/null`)은 `/dev/null`이 작업 디렉토리에 의존하지 않으므로 프롬프트를 트리거하지 않습니다.

<Warning>
  명령 인수를 제약하려고 시도하는 Bash 권한 패턴은 취약합니다. 예를 들어, `Bash(curl http://github.com/ *)`는 curl을 GitHub URL로 제한하려고 하지만 다음과 같은 변형과는 일치하지 않습니다:

  * URL 앞의 옵션: `curl -X GET http://github.com/...`
  * 다른 프로토콜: `curl https://github.com/...`
  * 리다이렉트: `curl -L http://short.example.com/xyz` (GitHub로 리다이렉트)
  * 변수: `URL=http://github.com && curl $URL`

  더 안정적인 URL 필터링을 위해 다음을 고려합니다:

  * **Bash 네트워크 도구 제한**: deny 규칙을 사용하여 `curl`, `wget` 및 유사한 명령을 차단한 다음 허용된 도메인에 대해 `WebFetch(domain:github.com)` 권한으로 WebFetch 도구를 사용합니다. deny 규칙은 경로로 호출하거나 `sh -c` 내부에서 호출한 동일한 프로그램과는 일치하지 않으므로 제한이 반드시 유지되어야 할 때는 [샌드박스 네트워크 allowlist](/docs/ko/sandboxing#network-isolation)와 함께 사용합니다. [Bash 규칙이 일치하지 않는 것](#bash-rule-limits)을 참조합니다.
  * **PreToolUse 훅 사용**: Bash 명령의 URL을 검증하고 허용되지 않은 도메인을 차단하는 훅을 구현합니다.
  * **CLAUDE.md 지침 추가**: `CLAUDE.md`에서 허용된 curl 패턴을 설명합니다. 이는 Claude가 시도하는 것을 형성하지만 경계를 적용하지 않으므로 위의 옵션 중 하나와 함께 사용합니다.

  WebFetch만 사용하는 것은 네트워크 액세스를 방지하지 않습니다. Bash가 허용되면 Claude는 여전히 `curl`, `wget` 또는 다른 도구를 사용하여 모든 URL에 도달할 수 있습니다.
</Warning>

<h4 id="redirections">
  리다이렉션
</h4>

명령이 출력 또는 입력을 리다이렉트할 때 Claude Code는 Claude가 해당 파일을 직접 작성하거나 읽은 것처럼 리다이렉트 대상을 파일 규칙에 대해 확인합니다:

* **출력 리다이렉트**: `> file`, `>> file` 또는 `2> file`의 경우 확인은 `Edit` allow 및 deny 규칙, [보호된 경로](/docs/ko/permission-modes#protected-paths) 및 [작업 디렉토리](#working-directories)를 포함합니다. `Bash(git commit *)`와 같은 규칙은 명령을 허용하며 대상은 허용하지 않습니다. `~`로 시작하거나 glob 문자를 포함하는 대상은 승인이 필요합니다.
* **입력 리다이렉트**: `< file`의 경우 확인은 `Read` allow 및 deny 규칙과 작업 디렉토리를 포함합니다. 작업 디렉토리 외부의 대상은 allow 규칙이 이를 포함하지 않는 한 승인이 필요합니다. glob 패턴을 포함하거나 동일한 명령의 `cd` 뒤에 오는 상대 경로는 allow 규칙이 이를 포함해도 승인이 필요합니다. Claude Code는 v2.1.257 이상에서 입력 대상을 확인합니다.

파일이 없는 대상은 확인되지 않습니다: `/dev/null`, `2>&1` 및 `<&3`과 같은 파일 설명자 형식, 그리고 here-docs 및 here-strings입니다.

Claude Code는 또한 `tee` 명령이 작성하는 파일을 확인하며, `make | tee build.log`와 같은 파이프라인에서도 확인합니다. 확인은 `Edit` allow 및 deny 규칙, [보호된 경로](/docs/ko/permission-modes#protected-paths) 및 [작업 디렉토리](#working-directories)를 포함합니다. `Bash(tee *)` 같은 allow 규칙은 작업 디렉토리 외부의 대상을 포함하지 않습니다. Claude Code는 v2.1.269 이상에서 `tee` 대상을 확인합니다.

<h3 id="powershell">
  PowerShell
</h3>

PowerShell 권한 규칙은 Bash 규칙과 동일한 형태를 사용합니다. `*`를 사용한 와일드카드는 어느 위치에나 일치하고, `:*` 접미사는 후행 ` *`와 동등하며, 베어 `PowerShell` 또는 `PowerShell(*)`는 모든 명령과 일치합니다. 이 구성은 `Get-ChildItem` 및 `git commit` 명령을 허용하면서 `Remove-Item`을 차단합니다:

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

일반적인 별칭은 일치하기 전에 정규화됩니다. cmdlet 이름에 대해 작성된 규칙은 해당 별칭과도 일치하므로 `PowerShell(Get-ChildItem *)`는 `gci`, `ls` 및 `dir`과도 일치합니다. 일치는 대소문자를 구분하지 않습니다.

Claude Code는 PowerShell AST를 구문 분석하고 복합 명령의 각 명령을 독립적으로 확인합니다. 파이프라인 연산자 `|`, 문 구분자 `;` 및 PowerShell 7+ 이상의 체인 연산자 `&&` 및 `||`는 복합 명령을 서브명령으로 분할합니다. 복합 명령이 허용되려면 규칙이 모든 서브명령과 일치해야 합니다.

<h3 id="read-and-edit">
  Read 및 Edit
</h3>

Claude의 파일 도구가 파일 또는 디렉토리를 읽는 것을 차단하려면 `Read(./.env)` 또는 `Read(./secrets/**)`와 같은 경로에 대해 `Read` deny 규칙을 추가합니다. [민감한 파일 제외](/docs/ko/settings-reference#exclude-sensitive-files)에는 붙여넣기 준비가 된 예제가 있습니다.

`Edit` 규칙은 파일을 편집하는 모든 기본 제공 도구에 적용됩니다. Claude는 Grep 및 Glob과 같이 파일을 읽는 모든 기본 제공 도구에 `Read` 규칙을 적용하기 위해 최선을 다합니다. 또한 프롬프트의 `@file` 언급 및 연결된 [IDE](/docs/ko/vs-code#the-built-in-ide-mcp-server)가 Claude와 공유하는 선택 및 열린 파일 컨텍스트에도 적용합니다.

`Read` deny 규칙은 동일한 경로에서 [Edit 및 Write 도구](/docs/ko/errors#file-is-covered-by-a-read-deny-rule)도 차단하며, 새 파일 생성도 포함됩니다. NotebookEdit은 포함되지 않으므로 도구가 변경할 수 없는 경로에 대해 `Edit` deny 규칙을 추가합니다. 확인은 편집 시 Claude Code v2.1.208 이상이 필요하며, 쓰기 시 v2.1.228 이상이 필요합니다.

Claude Code는 `Edit(path)` 및 `Read(path)` 규칙에 대해서만 파일 권한을 확인합니다. `Write`, `NotebookEdit`, `Glob` 또는 레거시 `MultiEdit` 도구에 대해 경로 규칙을 작성하면 Claude Code는 규칙을 수락하지만 절대 참조하지 않으며, `--allowedTools`에 전달된 `Glob` 규칙을 제외하고 [시작 시 경고](/docs/ko/errors#is-not-matched-by-file-permission-checks)합니다. `Write(docs/**)`, `NotebookEdit(docs/**)` 또는 `MultiEdit(docs/**)` 대신 `Edit(docs/**)`를 사용하고, `Glob(docs/**)` 대신 `Read(docs/**)`를 사용합니다. Claude Code는 `Write`, `NotebookEdit` 또는 `Glob`과 같은 경로가 없는 도구 이름 규칙에 대해 경고하지 않습니다. 이는 도구 수준에서 모든 곳에서 해당 규칙과 일치합니다. v2.1.210 이상이 필요합니다.

<Warning>
  Read 및 Edit deny 규칙은 Claude의 기본 제공 파일 도구, Bash에서 Claude Code가 인식하는 `cat`, `head`, `tail`, `sed` 및 `tee`와 같은 파일 명령, 그리고 `> file` 및 `< file`과 같은 Bash [리다이렉션](#redirections)의 대상에 적용됩니다. 이들은 파일이 있는 디렉토리에서 실행한 `grep -r pattern .`처럼 파일 이름을 지정하지 않고 파일을 읽는 명령이나, 파일을 직접 여는 Python 또는 Node 스크립트처럼 파일을 간접적으로 읽거나 쓰는 임의의 서브프로세스에는 적용되지 않습니다. 경로에 대한 모든 프로세스의 액세스를 차단하는 OS 수준 적용을 위해 [샌드박싱을 활성화합니다](/docs/ko/sandboxing).
</Warning>

Read 및 Edit 규칙은 모두 [gitignore](https://git-scm.com/docs/gitignore) 패턴 구문을 사용하며 4가지 고유한 패턴 유형이 있습니다. 단일 세그먼트 디렉토리 패턴의 경우 일치 깊이는 이 섹션의 뒷부분에서 설명하는 규칙 유형에도 따릅니다:

| 패턴                 | 의미               | 예시                               | 일치                                                 |
| ------------------ | ---------------- | -------------------------------- | -------------------------------------------------- |
| `//path`           | 파일 시스템 루트의 절대 경로 | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                          |
| `~/path`           | 홈 디렉토리의 경로       | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                     |
| `/path`            | 설정 소스에 상대적인 경로   | `Edit(/src/**/*.ts)`             | 프로젝트 설정의 `<primary working directory>/src/**/*.ts` |
| `path` 또는 `./path` | 현재 디렉토리에 상대적인 경로 | `Read(*.env)`                    | `<cwd>/*.env`                                      |

<Warning>
  `/Users/alice/file`과 같은 패턴은 절대 경로가 아닙니다. 단일 슬래시는 설정 소스에 앵커됩니다. 절대 경로의 경우 `//Users/alice/file`을 사용합니다.
</Warning>

`/path` 패턴은 설정 소스가 정의된 디렉토리에 앵커되므로 동일한 규칙은 배치 위치에 따라 다른 위치와 일치합니다:

| 규칙 정의 위치                             | `/path` 해석                         |
| :----------------------------------- | :--------------------------------- |
| `.claude/settings.json`의 프로젝트 설정     | `<primary working directory>/path` |
| `.claude/settings.local.json`의 로컬 설정 | `<primary working directory>/path` |
| `~/.claude/settings.json`의 사용자 설정    | `~/.claude/path`                   |
| `--settings <file>`로 전달된 파일          | `<directory of file>/path`         |
| CLI 플래그 또는 세션 규칙                     | `<primary working directory>/path` |

`/permissions`을 통해 추가한 규칙은 저장한 설정 파일의 행을 따릅니다.

로컬 설정 규칙은 v2.1.211 이상에서 Claude Code가 [파일을 저장하는](#permission-system) 저장소 루트가 아닌 세션의 [primary working directory](#working-directories)에 앵커됩니다. 저장소 루트에서 시작된 세션에서 두 디렉토리는 동일합니다. [worktree](/docs/ko/worktrees) 세션에서 `Edit(/src/**)`와 같은 공유 규칙은 해당 worktree의 자체 `src/` 디렉토리와 일치합니다.

사용자 설정의 `Read(/secrets/**)`와 같은 deny 규칙은 프로젝트의 `secrets` 디렉토리가 아니라 `~/.claude/secrets/**`를 차단합니다. 모든 프로젝트 내에서 적용되는 사용자 설정의 규칙을 작성하려면 `//` 절대 경로 또는 `~/` 홈 상대 경로를 대신 사용합니다.

Windows에서 경로는 일치하기 전에 POSIX 형식으로 정규화됩니다. `C:\Users\alice`는 `/c/Users/alice`가 되므로 `//c/**/.env`를 사용하여 해당 드라이브의 어디든 `.env` 파일과 일치시킵니다. 모든 드라이브에서 일치시키려면 `//**/.env`를 사용합니다.

예시:

* `Edit(/docs/**)`: `<primary working directory>/docs/`의 편집 (NOT `/docs/` and NOT `<primary working directory>/.claude/docs/`)
* `Read(~/.zshrc)`: 홈 디렉토리의 `.zshrc` 읽기
* `Edit(//tmp/scratch.txt)`: 절대 경로 `/tmp/scratch.txt` 편집
* `Read(src/**)`: allow 규칙으로 `<current-directory>/src/`에서만 읽기; deny 또는 ask 규칙으로 현재 디렉토리 아래 어느 깊이에서나 `src` 디렉토리와 일치

규칙은 해당 앵커 아래의 파일만 일치합니다. 그 범위 내에서 일치 깊이는 패턴 형태와 단일 세그먼트 디렉토리 패턴의 경우 규칙 유형에 따라 달라집니다. 베어 파일명은 gitignore 의미론을 따르고 어느 깊이에서나 일치하므로 `Read(.env)` 및 `Read(**/.env)`는 동등합니다:

| Deny 규칙                         | 차단                         | 차단하지 않음                    |
| ------------------------------- | -------------------------- | -------------------------- |
| `Read(.env)` 또는 `Read(**/.env)` | 현재 디렉토리 또는 그 아래의 모든 `.env` | 상위 디렉토리 또는 다른 프로젝트의 `.env` |
| `Read(//**/.env)`               | 파일 시스템의 어디든 모든 `.env`      | 없음; 규칙은 파일 시스템 루트에 앵커됨     |

`src/**`와 같은 단일 디렉토리 세그먼트를 가진 상대 패턴은 규칙 유형에 따라 다른 깊이에서 일치합니다:

* **Allow 규칙**: `Edit(src/**)`는 `<cwd>/src` 및 그 아래의 파일만 일치합니다. 디렉토리 이름을 어느 깊이에서나 허용하려면 `Edit(**/src/**)`를 작성합니다.
* **Deny 및 ask 규칙**: `Read(secrets/**)`는 현재 디렉토리 아래 어느 깊이에서나 `secrets`라는 디렉토리와 일치하므로 규칙은 중첩된 복사본에도 적용됩니다.

다른 모든 패턴 형태는 모든 규칙 유형에서 동일한 깊이에서 일치합니다: `Edit(/src/**)` 및 `Edit(src/components/**)`는 앵커된 위치에서만 일치하고, `Edit(**/src/**)`는 어느 깊이에서나 일치합니다.

다음 예제는 최상위 `src/` 디렉토리와 `vendor/` 아래의 중첩된 복사본이 있는 프로젝트에 대해 각 패턴 형태를 보여줍니다:

```text theme={null}
<current-directory>/
├── src/
│   └── app.ts
└── vendor/
    └── pkg/
        └── src/
            └── lib.js
```

| 규칙                                   | `src/app.ts` 일치 | `vendor/pkg/src/lib.js` 일치 |
| :----------------------------------- | :-------------- | :------------------------- |
| `Edit(src/**)` as an allow rule      | 예               | 아니오                        |
| `Edit(src/**)` as a deny or ask rule | 예               | 예                          |
| `Edit(/src/**)` in any rule type     | 예               | 아니오                        |
| `Edit(**/src/**)` in any rule type   | 예               | 예                          |

<Note>
  gitignore 패턴에서 `*`는 단일 경로 세그먼트 내에서 일치하고 패턴의 어느 위치에나 나타날 수 있으며, `**`는 디렉토리 전체에서 일치합니다.
</Note>

"예, 다시 묻지 않기"로 파일 경로를 승인하면 Claude Code는 `[`, `]` 및 `*`와 같은 gitignore 패턴 문자를 해당 경로에서 이스케이프하므로 생성된 규칙은 승인한 리터럴 경로와만 일치합니다. 직접 작성한 규칙은 이스케이프되지 않습니다. v2.1.202 이전에는 Claude Code가 경로를 이스케이프되지 않은 상태로 저장했으므로 `[2024-06] Reports`라는 이름의 디렉토리에 대해 생성된 규칙이 자신의 경로와 일치하지 않거나 의도하지 않은 형제 디렉토리와 일치할 수 있었습니다.

경로에 괄호를 이스케이프할 필요는 없으므로 `Edit(./Finance (2024)/**)` 는 `Finance (2024)` 폴더와 일치합니다.

경로가 gitignore 패턴으로 사용할 수 없는 deny 또는 ask 규칙은 여전히 정확한 경로를 보호합니다. 사용할 수 없는 패턴을 가진 allow 규칙은 아무것도 승인하지 않습니다.

deny 또는 ask 패턴이 `!`로 시작하는 것은 gitignore 부정입니다. 이는 이전에 나열된 `path` 또는 `./path` 규칙에서 일치하는 경로를 제거합니다. 하나의 설정 파일의 `deny` 목록에서 `Read(*.env)` 뒤에 `Read(!sample.env)`가 오면 `.env`로 끝나는 모든 파일을 어느 깊이에서나 차단하지만 `sample.env`라는 파일은 제외합니다. 먼저 나열된 `!` 규칙은 아무것도 제거하지 않습니다.

제거는 동일한 소스의 규칙에만 도달합니다. 프로젝트 설정 또는 `--disallowedTools`의 `Read(!.env)`는 관리되는 설정 또는 다른 설정 파일의 `Read(./.env)` deny를 취소하지 않습니다.

두 가지 제한이 `!` 패턴이 제거할 수 있는 것을 좁힙니다:

* Claude Code는 `!` 뒤에 `/`, `~/` 또는 `//`가 따라오더라도 `!` 패턴을 현재 디렉토리에 상대적으로 읽으므로 패턴은 이러한 접두사 중 하나로 앵커된 규칙에 도달할 수 없습니다. `Read(!~/notes/public/**)`는 `Read(~/notes/**)`에서 아무것도 제거하지 않습니다.
* 제거는 규칙이 전체적으로 차단하는 디렉토리 내의 파일을 다시 열 수 없습니다. `Read(secrets/**)` 및 `Read(!secrets/public/**)`를 사용하면 Claude Code는 여전히 `secrets/public`을 나머지 `secrets`와 함께 차단합니다.

Claude가 심볼릭 링크에 액세스할 때 권한 규칙은 두 경로를 확인합니다: 심볼릭 링크 자체와 이것이 해결되는 파일입니다. Allow 및 deny 규칙은 해당 쌍을 다르게 취급합니다: allow 규칙은 프롬프트로 폴백하고 deny 규칙은 즉시 차단합니다.

* **Allow 규칙**: 심볼릭 링크 경로와 해당 대상이 모두 일치할 때만 적용됩니다. 허용된 디렉토리 내의 심볼릭 링크가 외부를 가리키면 여전히 프롬프트합니다.
* **Deny 규칙**: 심볼릭 링크 경로 또는 대상이 일치할 때 적용됩니다. 거부된 파일을 가리키는 심볼릭 링크는 자체적으로 거부됩니다. 예를 들어, `Read(./project/**)` allowed 및 `Read(~/.ssh/**)` denied를 사용하면 `./project/key`의 심볼릭 링크가 `~/.ssh/id_rsa`를 가리킬 때 차단됩니다: 대상이 allow 규칙에 실패하고 deny 규칙과 일치합니다.

macOS 및 Linux에서 심볼릭 링크된 디렉토리를 통해 작성된 deny 또는 ask 규칙은 `//`, `~/` 또는 `/` 패턴을 사용하여 디렉토리의 실제 위치에도 적용됩니다. 예를 들어 macOS에서 `/etc`가 `/private/etc`로 해결되는 경우 `Read(//etc/**)`는 `/private/etc/hosts`도 차단합니다. v2.1.268 이전에는 심볼릭 링크된 디렉토리를 통해 작성된 deny 또는 ask 규칙이 실제 위치로 주어진 경로에 적용되지 않았습니다.

도구가 승인된 파일을 열 때 Claude Code는 [경로가 여전히 권한 확인이 승인한 위치로 해결되는지 확인합니다](/docs/ko/errors#refusing-after-a-symlink-changed).

Grep 및 Glob은 `path` 인수가 해결되는 디렉토리를 검색합니다. Claude Code는 해당 디렉토리에 `Read` deny 규칙을 적용합니다.

<h3 id="webfetch">
  WebFetch
</h3>

WebFetch 규칙은 `domain:` 접두사를 사용하고 요청된 URL의 호스트명과 일치합니다. 일치는 대소문자를 구분하지 않으며, `*` 와일드카드를 지원하고, 규칙과 호스트명 모두에서 후행 `.`을 제거하므로 `example.com.`과 `example.com`은 동일하게 취급됩니다.

* `WebFetch(domain:example.com)`은 `example.com`으로의 요청과 일치합니다
* `WebFetch(domain:*.example.com)`은 `api.example.com` 또는 `a.b.example.com`과 같은 모든 깊이의 모든 서브도메인과 일치하지만 `example.com` 자체는 일치하지 않습니다
* `WebFetch(domain:*)`는 모든 도메인과 일치합니다. 베어 `WebFetch` 규칙과는 다릅니다. [모든 fetch 허용 또는 거부](#allow-or-deny-every-fetch)를 참조합니다

선행 `*.` 또는 베어 `*`가 아닌 다른 위치에서 와일드카드는 두 점 사이의 텍스트와만 일치합니다. `WebFetch(domain:example.*)`는 `example.org`과 일치합니다. 여기서 `*`는 `org`가 되지만 `example.evil.com`과는 일치하지 않습니다. 여기서 `*`는 `evil.com`이 되어야 하고 점을 넘어야 합니다. 이는 후행 와일드카드가 공격자가 등록할 수 있는 도메인과 일치하는 것을 방지합니다.

WebFetch 규칙의 와일드카드는 fetch를 일치시키기 위해 Claude Code v2.1.172 이상이 필요합니다.

<h4 id="allow-or-deny-every-fetch">
  모든 fetch 허용 또는 거부
</h4>

베어 `WebFetch` 규칙은 `domain:` 부분이 없는 도구 이름입니다(예: `"deny": ["WebFetch"]`). 이것과 `WebFetch(domain:*)`는 모든 URL을 포함하지만 Claude Code는 이들을 다르게 적용하며, `domain:` 형식만 해당 도메인을 샌드박스의 [허용 또는 거부된 도메인 목록](/docs/ko/sandboxing#network-isolation)에 추가합니다. 해당 섹션은 샌드박스가 지원하는 와일드카드 형식과 이를 추가한 버전을 나열합니다.

각 행은 `allow` 목록과 `deny` 목록에서 규칙이 수행하는 작업을 보여줍니다:

| 규칙                   | `allow`에서                                                    | `deny`에서                                                                                    |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| `WebFetch`           | Claude는 프롬프트 없이 fetch합니다. 샌드박스된 명령이 도달할 수 있는 호스트를 변경하지 않습니다. | Claude Code는 `WebFetch` 도구를 제거하므로 Claude는 fetch할 수 없습니다. 샌드박스된 명령이 도달할 수 있는 호스트를 변경하지 않습니다. |
| `WebFetch(domain:*)` | Claude는 프롬프트 없이 fetch하고, 샌드박스된 명령은 모든 호스트에 도달할 수 있습니다.       | Claude Code는 도구를 유지하고 각 fetch를 거부하며, 샌드박스된 명령은 어떤 호스트에도 도달할 수 없습니다.                         |

두 형식은 또한 [artifacts](/docs/ko/artifacts)의 읽기에서 다릅니다. artifacts는 Artifact 도구가 claude.ai에 게시하는 페이지입니다. 베어 `WebFetch` deny 또는 ask 규칙은 이러한 읽기에 적용되지 않습니다. `claude.ai` 또는 `*.claudeusercontent.com` 콘텐츠 호스트를 포함하는 `domain:` 규칙(예: `WebFetch(domain:claude.ai)` 또는 `WebFetch(domain:*)`)은 각 읽기를 거부하거나 이전에 프롬프트합니다. [`Artifact` 규칙](/docs/ko/artifacts#disable-artifacts)도 동일하게 수행합니다.

규칙이 읽기를 차단할 때 거부는 규칙의 이름을 지정합니다. v2.1.268 이전에는 베어 `WebFetch` deny 규칙이 모든 artifact 읽기를 차단했으며, 베어 ask 규칙은 각 읽기 이전에 프롬프트했습니다.

Claude가 자유롭게 fetch하도록 하면서 샌드박스 allowlist를 그대로 유지하려면 베어 형식을 사용합니다. 이 `settings.json`이 그렇게 합니다:

```json theme={null}
{
  "permissions": {
    "allow": ["WebFetch"]
  }
}
```

Claude에게 페이지를 fetch하도록 요청하면 프롬프트 없이 fetch합니다. Claude에게 샌드박스 allowlist 외부의 호스트에 대해 [샌드박스된](/docs/ko/sandboxing) `curl`을 실행하도록 요청하면 Claude Code는 여전히 해당 호스트에 대해 프롬프트하거나 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서 [명령별 허용 도메인](/docs/ko/sandboxing#per-command-allowed-domains-in-auto-mode)에 호스트를 이름 지정합니다. 베어 규칙이 호스트를 allowlist에 추가하지 않았기 때문입니다.

<h3 id="mcp">
  MCP
</h3>

MCP 규칙은 Claude Code에서 구성된 서버 이름을 사용하며, 선택적으로 해당 서버의 도구 이름이 뒤따릅니다.

* `mcp__puppeteer`는 `puppeteer` 서버에서 제공하는 모든 도구와 일치합니다
* `mcp__puppeteer__*`는 와일드카드 구문을 사용하며 `puppeteer` 서버의 모든 도구와도 일치합니다
* `mcp__puppeteer__puppeteer_navigate`는 `puppeteer` 서버에서 제공하는 `puppeteer_navigate` 도구와 일치합니다

조직이 [claude.ai 커넥터](/docs/ko/mcp#organization-controls-on-connector-tools) 도구를 `ask`로 설정한 경우, 해당 설정이 세션의 Claude Code에 도달하면 해당 도구에 대한 allow 규칙은 적용되지 않습니다: Claude Code는 `auto` 및 `bypassPermissions` 모드에서도 모든 호출에 대해 프롬프트합니다. 절대 프롬프트하지 않는 `dontAsk` 모드에서는 Claude Code가 호출을 거부합니다. 커넥터에서 Claude Code가 자체적으로 가져오는 도구는 `mcp__claude_ai_<server>__<tool>`로 나타납니다.

[Cowork](https://claude.com/docs/cowork/overview) 세션의 Claude Desktop 앱에서 Claude는 기본 제공 `Bash` 도구 대신 Cowork의 `mcp__workspace__bash` 도구를 통해 셸 명령을 실행하고, Cowork는 마찬가지로 웹 fetch를 위해 `mcp__workspace__web_fetch`를 제공합니다. Claude Code는 또한 전체 `Bash` 또는 `WebFetch` 도구의 이름을 지정하는 deny 규칙을 이러한 Cowork 도구에 적용하므로 관리되는 `Bash` deny 규칙은 Claude가 Cowork에서 셸 명령을 실행하는 것을 중지합니다. Claude Code가 이러한 호출을 차단할 때 메시지는 Cowork 도구의 이름을 지정합니다: `Permission to use mcp__workspace__bash has been denied.` Allow 규칙은 이월되지 않습니다: Claude Code는 절대 `Bash` allow 규칙을 `mcp__workspace__bash`에 적용하지 않습니다.

<h3 id="agent-subagents">
  Agent (subagents)
</h3>

`Agent(AgentName)` 규칙을 사용하여 Claude가 사용할 수 있는 [subagents](/docs/ko/sub-agents)를 제어합니다:

* `Agent(Explore)`는 Explore subagent와 일치합니다
* `Agent(Plan)`은 Plan subagent와 일치합니다
* `Agent(my-custom-agent)`는 `my-custom-agent`라는 사용자 정의 subagent와 일치합니다

이러한 규칙을 설정의 `deny` 배열에 추가하거나 `--disallowedTools` CLI 플래그를 사용하여 특정 에이전트를 비활성화합니다. Explore 에이전트를 비활성화하려면:

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

`Cd` 규칙은 [`/cd` 명령](/docs/ko/commands)이 세션을 이동할 수 있는 디렉토리를 제어합니다. `Cd`는 모델 호출 가능 도구가 아닙니다: Claude는 이를 호출할 수 없으며 규칙은 사용자가 직접 `/cd`를 실행할 때만 적용됩니다.

베어 `Cd` deny 규칙은 `/cd`를 완전히 비활성화합니다. `Cd(<path-pattern>)` deny 규칙은 일치하는 대상을 차단합니다. Deny 규칙은 대상의 모든 철자를 확인하며, 이것이 해결되는 각 심볼릭 링크 홉을 포함하므로 한 경로에 대해 작성된 규칙은 그것으로 해결되는 대상도 차단합니다.

`Cd` allow 규칙을 추가하면 `/cd`를 allowlist 모드로 전환합니다: 해결된 대상 디렉토리는 allow 규칙 중 하나와 일치해야 하거나 `/cd`는 거부합니다. `Cd` 규칙이 구성되지 않으면 `/cd`는 기본 동작을 유지하고 익숙하지 않은 디렉토리를 신뢰하도록 프롬프트합니다.

경로 패턴은 [Read 및 Edit 규칙](#read-and-edit)의 `//`, `~/` 및 `/` 앵커를 공유하지만 일치는 gitignore 스타일이 아닌 전체 디렉토리 경로에 앵커됩니다. `*`는 정확히 하나의 경로 세그먼트와 일치하고 `**`는 세그먼트 전체에서 일치합니다. 후행 `/**`는 또한 명명된 루트와 일치합니다.

| 규칙                    | 일치                                         | 일치하지 않음                    |
| --------------------- | ------------------------------------------ | -------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                               | `~/code/app/src`, `~/code` |
| `Cd(~/code/**)`       | `~/code` 및 그 아래의 모든 디렉토리                   | `~/code` 외부의 디렉토리          |
| `Cd(**/node_modules)` | 현재 디렉토리 아래 어느 깊이에서나 모든 `node_modules` 디렉토리 | `node_modules/pkg`         |

<h2 id="extend-permissions-with-hooks">
  훅으로 권한 확장
</h2>

[Claude Code 훅](/docs/ko/hooks-guide)을 사용하면 런타임에 권한을 평가하는 사용자 정의 셸 명령을 등록할 수 있습니다. Claude Code가 도구 호출을 수행할 때, PreToolUse 훅은 [`EndConversation`](/docs/ko/tools-reference#endconversation-tool-behavior)을 제외한 모든 도구에 대해 권한 프롬프트 전에 실행됩니다. 훅 출력은 도구 호출을 거부하거나, 프롬프트를 강제하거나, 프롬프트를 건너뛰어 호출을 진행하도록 할 수 있습니다.

훅 결정은 권한 규칙을 우회하지 않습니다. Claude Code는 PreToolUse 훅이 반환하는 값에 관계없이 deny 및 ask 규칙을 평가합니다. 일치하는 deny 규칙은 호출을 차단하고, 일치하는 ask 규칙은 훅이 `"allow"` 또는 `"ask"`를 반환했을 때도 여전히 프롬프트합니다. 이는 [권한 관리](#manage-permissions)에서 설명한 deny 우선 우선순위를 유지하며, 관리형 설정에서 설정한 deny 규칙을 포함합니다.

[`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구와 조직에서 [세션에서 해당 설정이 Claude Code에 도달하는 경우 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)한 커넥터 도구도 훅이 `"allow"`를 반환할 때 여전히 프롬프트합니다.

차단 훅은 또한 allow 규칙보다 우선합니다. 종료 코드 2로 종료되는 훅은 권한 규칙이 평가되기 전에 도구 호출을 중지하므로, allow 규칙이 호출을 허용할 수 있는 경우에도 차단이 적용됩니다. 모든 Bash 명령을 프롬프트 없이 실행하되 차단하려는 몇 가지를 제외하려면, allow 목록에 `"Bash"`를 추가하고 해당 특정 명령을 거부하는 PreToolUse 훅을 등록합니다. 적응할 수 있는 훅 스크립트는 [보호된 파일에 대한 편집 차단](/docs/ko/hooks-guide#block-edits-to-protected-files)을 참조하세요.

<h2 id="working-directories">
  작업 디렉토리
</h2>

기본적으로 Claude는 시작된 디렉토리의 파일에 액세스할 수 있습니다. 이 디렉토리는 [`/cd`](#move-the-session-to-another-directory)로 세션을 이동할 때까지 세션의 기본 작업 디렉토리입니다. 이 액세스를 확장할 수 있습니다:

* **시작 중**: `--add-dir <path>` CLI 인수 사용
* **세션 중**: `/add-dir` 명령 사용
* **영구 구성**: [설정 파일](/docs/ko/settings#where-settings-live)의 `additionalDirectories`에 추가

추가 디렉토리의 파일은 원래 작업 디렉토리와 동일한 권한 규칙을 따릅니다: 프롬프트 없이 읽을 수 있게 되며, 파일 편집 권한은 현재 권한 모드를 따릅니다.

대부분의 [네트워크 경로](/docs/ko/errors#working-directory-is-a-network-path)(예: UNC 공유 `\\server\share`)는 작업 디렉토리로 추가할 수 없습니다. 이는 조회 시 이름이 지정하는 호스트에 연결할 수 있기 때문입니다. Windows에서는 대신 공유를 드라이브 문자로 매핑하고 시작 시 `--add-dir`으로 드라이브를 전달합니다.

[`permissions.blockReadsOutsideWorkingDirectories`](/docs/ko/settings-reference#permissions-blockreadsoutsideworkingdirectories)를 설정하여 파일 도구가 모든 권한 모드에서 경계를 지은 경로를 거부하도록 합니다. 자동 모드에서 Claude Code는 Claude가 [작업 디렉토리 외부에서 읽을](/docs/ko/permission-modes#first-read-outside-the-working-directories) 때 처음으로 이를 켜도록 제안합니다.

macOS의 백그라운드 세션에서 세션 호스트는 Claude가 파일을 읽거나 쓸 필요가 있을 때 터미널과 별도로 `~/Desktop`, `~/Documents`, `~/Downloads`와 같은 보호된 폴더에 대한 액세스를 요청합니다. 읽기가 `Operation not permitted`로 실패하면 [백그라운드 세션에 폴더 액세스를 부여하는 방법](/docs/ko/agent-view#background-sessions-can%E2%80%99t-read-desktop-documents-or-downloads-on-macos)을 참조하세요.

<h3 id="move-the-session-to-another-directory">
  세션을 다른 디렉토리로 이동
</h3>

세션을 다른 기본 작업 디렉토리로 이동하려면, 현재 디렉토리 옆에 [디렉토리를 추가](#working-directories)하는 대신 `/cd <path>`를 실행합니다. Claude Code는 대화를 유지하고, 새 디렉토리의 `CLAUDE.md`를 로드하며, 이전에 작업하지 않은 경우 [작업 공간을 신뢰](#project-allow-rules-and-workspace-trust)하도록 요청합니다. 그 후 Claude Code는 새 디렉토리에서 `--resume`을 실행할 때 [이동된 세션을 찾습니다](/docs/ko/sessions#resume-a-session).

이동하는 즉시 Claude Code는 새 디렉토리의 프로젝트 구성을 적용합니다:

* 권한 규칙 및 [hooks](/docs/ko/hooks)를 포함한 프로젝트 설정
* [`.mcp.json` 서버](/docs/ko/mcp#project-scope)(시작 시와 동일한 [서버 승인](/docs/ko/mcp#project-server-approvals-and-workspace-trust) 및 [로컬 범위](/docs/ko/mcp#local-scope) MCP 서버의 적용을 받음)
* 설정이 활성화하는 [plugins](/docs/ko/plugins/overview), [skills](/docs/ko/skills#discovery-from-parent-and-nested-directories) 및 [subagents](/docs/ko/sub-agents)
* 이전 디렉토리의 설정에서 환경 변수 위에 적용되는 [`env`](/docs/ko/settings-reference#env) 값(이전 디렉토리의 설정은 계속 적용됨)

Claude Code는 또한 이전 디렉토리의 프로젝트 및 [로컬 범위](/docs/ko/mcp#local-scope) MCP 서버와 이동 후 더 이상 활성화되지 않는 [plugins](/docs/ko/mcp#plugin-provided-mcp-servers)의 서버를 연결 해제합니다. 이전 디렉토리의 설정 대신 새 디렉토리의 설정에서 [추가 디렉토리](#working-directories)를 가져오고, `--add-dir` 또는 `/add-dir`으로 추가한 디렉토리를 유지합니다. 이동이 활성화하는 Hooks는 여전히 [`${CLAUDE_PROJECT_DIR}`](/docs/ko/hooks#reference-scripts-by-path)를 세션이 시작된 프로젝트 루트로 설정하여 수신합니다.

새 디렉토리를 아직 신뢰하지 않을 때 Claude Code는 신뢰 프롬프트에서 디렉토리의 설정이 활성화할 허용 규칙, 추가 디렉토리, hooks 및 도우미 명령을 나열하므로 수락하기 전에 검토할 수 있습니다. 거부하면 세션은 현재 위치에 유지됩니다. v2.1.246 이전에는 `/cd`가 세션을 재개할 때까지 새 디렉토리의 설정, hooks, MCP 서버 또는 skills를 적용하지 않았으며, 신뢰 프롬프트는 디렉토리의 설정이 활성화할 항목을 나열하지 않았습니다.

[`Cd` 권한 규칙](#cd)으로 `/cd` 대상을 제한하거나 비활성화합니다.

<h3 id="additional-directories-grant-file-access-not-configuration">
  추가 디렉토리는 파일 액세스를 부여하며, 구성은 아닙니다
</h3>

디렉토리를 추가하면 Claude가 파일을 읽고 편집할 수 있는 위치가 확장됩니다. 해당 디렉토리를 전체 구성 루트로 만들지는 않습니다: 대부분의 `.claude/` 구성은 추가 디렉토리에서 발견되지 않지만 몇 가지 유형은 예외로 로드됩니다.

이러한 예외는 `--add-dir` 플래그 또는 `/add-dir` 명령으로 추가된 디렉토리에만 적용됩니다(Agent SDK가 플래그를 통해 추가하는 디렉토리 포함). 설정 파일의 `permissions.additionalDirectories`에 나열된 디렉토리는 파일 액세스만 부여하며 아래의 구성을 로드하지 않습니다.

Agent SDK의 TypeScript [`additionalDirectories`](/docs/ko/agent-sdk/typescript#options) 옵션 및 Python [`add_dirs`](/docs/ko/agent-sdk/python#claudeagentoptions) 옵션도 예외를 받습니다. TypeScript 옵션이 설정 키와 이름을 공유하지만 SDK는 각 항목을 Claude Code에 `--add-dir`로 전달하므로 이러한 디렉토리는 플래그로 추가된 디렉토리처럼 동작합니다. 모든 플래그로 추가된 디렉토리의 skills, 명령 및 subagents는 `project` [설정 소스](/docs/ko/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)를 통해 로드되므로 CLI에서 [`--setting-sources`](/docs/ko/cli-reference)로 해당 소스를 제외하거나 SDK에서 `settingSources`로 제외할 때 로드되지 않으며, [베어 모드](/docs/ko/headless#start-faster-with-bare-mode)는 이들 중 명령 및 subagents를 건너뜁니다.

다음 구성 유형은 `--add-dir` 디렉토리에서 로드됩니다:

| 구성                                                                                | `--add-dir`에서 로드됨                                                                                                       |
| :-------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `.claude/skills/`의 [Skills](/docs/ko/skills)                                           | 예, 라이브 리로드 포함                                                                                                           |
| `.claude/commands/`의 [Command files](/docs/ko/skills#where-skills-live)                | 예, 라이브 리로드 없음. 추가된 디렉토리와 프로젝트가 모두 동일한 이름의 명령을 정의할 때 Claude Code는 프로젝트의 명령을 실행합니다                                        |
| `.claude/agents/`의 [Subagents](/docs/ko/sub-agents)                                    | 예, 라이브 리로드 없음                                                                                                           |
| `.claude/settings.json` 및 `.claude/settings.local.json`의 [Settings](/docs/ko/settings) | `enabledPlugins` 및 [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces) 키만                         |
| [CLAUDE.md](/docs/ko/memory) 파일, `.claude/rules/` 및 `CLAUDE.local.md`                  | `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`이 설정된 경우에만. `CLAUDE.local.md`는 추가로 `local` 설정 소스가 필요하며, 이는 기본적으로 활성화됩니다 |

[기본 작업 디렉토리](#working-directories)의 하위 디렉토리에서 skills, 명령 및 subagents를 세션 중에 로드하려면 해당 하위 디렉토리의 경로로 `/add-dir`을 실행합니다. Claude Code는 하위 디렉토리가 이미 읽을 수 있으므로 프롬프트 없이 또는 작업 디렉토리를 추가하지 않고 세션의 나머지 부분에 대해 로드합니다. 이는 Claude Code v2.1.257 이상이 필요합니다.

Claude Code는 현재 작업 디렉토리 및 해당 부모, `~/.claude/`의 사용자 디렉토리 및 관리형 설정에서 출력 스타일을 발견합니다. Hooks 및 기타 `.claude/settings.json` 키는 현재 작업 디렉토리의 `.claude/` 폴더에서 로드되며 부모 디렉토리 폴백이 없고, 사용자 `~/.claude/settings.json` 및 관리형 설정과 함께 로드됩니다. `.claude/settings.local.json`은 Claude Code가 하위 디렉토리에서 시작되더라도 git 저장소 루트에서 로드되며, Windows와 같이 Claude Code가 [저장소 루트를 사용하지 않는](/docs/ko/settings#where-claude-code-looks-for-each-file) 경우는 제외됩니다. v2.1.211 이전에는 현재 작업 디렉토리에서만 로드되었습니다. [Agent SDK](/docs/ko/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) 세션은 모든 버전에서 작업 디렉토리에서 로드합니다.

프로젝트 전체에서 해당 구성을 공유하려면 다음 방법 중 하나를 사용합니다:

* **사용자 수준 구성**: `~/.claude/agents/`, `~/.claude/output-styles/` 또는 `~/.claude/settings.json`에 파일을 배치하여 모든 프로젝트에서 사용 가능하게 합니다
* **Plugins**: 팀이 설치할 수 있는 [plugin](/docs/ko/plugins/overview)으로 구성을 패키징하고 배포합니다
* **구성 디렉토리에서 시작**: 원하는 `.claude/` 구성이 포함된 디렉토리에서 Claude Code를 실행합니다

<h2 id="how-permissions-interact-with-sandboxing">
  권한이 샌드박싱과 상호 작용하는 방식
</h2>

권한과 [샌드박싱](/docs/ko/sandboxing)은 상호 보완적인 보안 계층입니다:

* **권한**은 Claude Code가 사용할 수 있는 도구와 액세스할 수 있는 파일 또는 도메인을 제어합니다. Bash, Read, Edit, WebFetch, MCP 및 다른 모든 도구에 적용되지만, deny 또는 ask 규칙은 다른 도구가 남아 있는 동안 [`EndConversation`](/docs/ko/tools-reference#endconversation-tool-behavior)을 차단할 수 없습니다.
* **샌드박싱**은 셸 명령의 파일 시스템 및 네트워크 액세스를 제한하는 OS 수준 적용을 제공합니다. Bash, PowerShell 및 [Monitor](/docs/ko/tools-reference#monitor-tool) 명령과 해당 자식 프로세스에만 적용됩니다.

심층 방어를 위해 둘 다 사용합니다. 샌드박스 제한은 프롬프트 주입이 Claude의 의사 결정을 우회하더라도 여전히 적용됩니다. 샌드박스 설정과 권한 규칙의 경로 및 도메인은 [최종 샌드박스 구성으로 병합됩니다](/docs/ko/sandboxing#permission-rules).

샌드박싱을 활성화하고 `autoAllowBashIfSandboxed`를 기본값인 `true`로 유지하면, 권한에 bare `Bash` ask 규칙이 포함되어 있거나 [동등한 `Bash(*)` 형식](#match-all-uses-of-a-tool)이 포함되어 있어도 샌드박스된 Bash 명령은 프롬프트 없이 실행됩니다. 샌드박스 경계는 전체 도구 프롬프트를 대체합니다.

[계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에서 Claude Code는 이 대체를 건너뜁니다. ask 규칙이 없으면 [기본 제공 읽기 전용 명령](#read-only-commands)은 여전히 프롬프트 없이 실행되고, 다른 모든 셸 명령은 계획 중일 때 일반 권한 흐름을 거칩니다. Claude Code가 거기서 명령을 제어하는 방식은 [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)를 참조하십시오. bare `Bash` ask 규칙이 있으면 샌드박스된 읽기 전용 명령을 포함한 모든 Bash 명령이 프롬프트되며, 이는 샌드박싱 외부와 동일합니다. v2.1.212 이전에는 대체가 계획 모드에서도 적용되었습니다.

이러한 검사는 여전히 적용됩니다:

* `Bash(git push *)`와 같은 콘텐츠 범위 ask 규칙은 여전히 프롬프트를 강제합니다
* 명시적 deny 규칙은 여전히 적용됩니다
* [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 `rm` 또는 `rmdir` 명령은 여전히 일반 권한 흐름을 거칩니다

제외된 명령과 같이 샌드박스에서 실행되지 않는 명령은 bare `Bash` ask 규칙을 일반적으로 따릅니다. 이 동작을 변경하려면 [샌드박스 모드](/docs/ko/sandboxing#sandbox-modes)를 참조하십시오.

<span id="managed-only-settings" />

<h2 id="managed-settings">
  관리형 설정
</h2>

중앙 집중식 제어가 필요한 조직의 경우, 관리자는 사용자 및 프로젝트 설정으로 재정의할 수 없는 관리형 설정을 배포합니다. 단, 몇 가지 [보안에 민감한 키](/docs/ko/settings#exceptions-to-managed-settings-precedence)는 예외입니다. [관리형 설정 배포](/docs/ko/managed-settings)에서는 전달 메커니즘, 관리형 계층 내 우선순위, 그리고 [관리형 설정만 설정할 수 있는 키](/docs/ko/managed-settings#managed-only-settings)를 다룹니다.

이러한 키 중 하나인 [`allowManagedPermissionRulesOnly`](/docs/ko/settings-reference#allowmanagedpermissionrulesonly)는 관리형 설정을 권한 규칙의 유일한 설정 소스로 만듭니다. 이 항목은 Claude Code가 무시하는 모든 소스를 나열합니다.

`disableBypassPermissionsMode`는 일반적으로 조직 정책을 적용하기 위해 관리형 설정에 배치되지만, 모든 범위에서 작동합니다. 사용자는 자신의 설정에서 이를 설정하여 자신을 우회 모드에서 잠글 수 있습니다.

<h2 id="settings-precedence">
  설정 우선순위
</h2>

권한 규칙은 다른 모든 Claude Code 설정과 동일한 [설정 우선순위](/docs/ko/settings#settings-precedence)를 따릅니다. 관리형 설정이 가장 높은 우선순위를 가지며, 명령줄 인수를 포함한 다른 수준은 관리형 권한 규칙을 재정의할 수 없습니다.

도구가 어느 수준에서든 거부되면 다른 수준은 이를 허용할 수 없습니다. 예를 들어, 관리형 설정 deny는 `--allowedTools`로 재정의할 수 없으며, `--disallowedTools`는 관리형 설정이 정의하는 것 이상의 제한을 추가할 수 있습니다.

설정 범위 전체에서도 동일하게 적용됩니다: 사용자 설정에서 권한을 허용하고 프로젝트 설정에서 거부하면, deny 규칙이 이를 차단합니다. 그 반대도 마찬가지입니다: 사용자 수준의 deny는 프로젝트 수준의 allow를 차단합니다. 왜냐하면 모든 범위의 deny 규칙이 allow 규칙보다 먼저 평가되기 때문입니다.

Embedding hosts는 SDK `managedSettings` 옵션을 통해 추가 관리형 정책을 제공할 수 있습니다. 여기에는 관리자가 `allowManaged*Only` 잠금을 설정하지 않은 경우 권한 허용 규칙이 포함됩니다. [Claude Desktop 세션에 정책 전달](/docs/ko/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions)에서 embedder 정책이 언제 적용되는지 다룹니다.

<h2 id="project-allow-rules-and-workspace-trust">
  프로젝트 허용 규칙 및 워크스페이스 신뢰
</h2>

`permissions.allow` 규칙과 프로젝트의 `.claude/settings.json`에 있는 `permissions.additionalDirectories` 항목은 기능을 부여하므로, Claude Code는 해당 폴더에 대해 [워크스페이스 신뢰 대화상자](/docs/ko/security#additional-safeguards)를 수락한 후에만 이를 적용합니다. 대화상자는 폴더가 부여할 규칙과 디렉터리를 나열하므로 먼저 검토할 수 있습니다. `deny` 및 `ask` 규칙은 영향을 받지 않습니다. 이들은 제한만 하기 때문입니다.

Claude Code는 시작 위치에 따라 수락한 신뢰를 저장합니다:

* 저장소에서 Claude Code는 git 저장소 루트를 기준으로 신뢰를 저장하므로, 신뢰는 서브모듈과 같은 중첩된 git 저장소를 제외한 전체 저장소를 포함합니다. [worktree](/docs/ko/worktrees)에서는 [저장된 규칙](#permission-system)과 마찬가지로 주 체크아웃의 루트를 사용합니다.
* 저장소 외부에서 Claude Code는 시작한 디렉터리를 기준으로 신뢰를 저장하며, 신뢰는 클론과 같은 중첩된 git 저장소를 제외한 해당 디렉터리의 모든 하위 디렉터리를 포함합니다. 포함된 각 하위 디렉터리는 상위 디렉터리를 신뢰한 폴더로 간주됩니다.
* 홈 디렉터리에서 시작하면 Claude Code는 현재 세션에만 신뢰를 유지하며 디스크에 기록하지 않습니다. [추가 보안 조치](/docs/ko/security#additional-safeguards) 참고를 참조하세요.

Claude Code는 대화형 세션에서만 신뢰 대화상자를 표시합니다. `claude -p` 실행이나 SDK 세션은 절대 표시하지 않으며, 상위 폴더를 신뢰해도 이러한 규칙에는 적용되지 않으므로, [폴더를 신뢰하기 전에 실행되는 것](#what-runs-before-you-trust-a-folder)은 이 두 가지 상황 각각에서 Claude Code가 여전히 사용하는 저장소 콘텐츠를 설명합니다.

<h3 id="when-your-local-settings-file-needs-trust">
  로컬 설정 파일이 신뢰가 필요한 경우
</h3>

`.claude/settings.local.json`은 일반적으로 사용자 자신의 파일이므로 Claude Code는 신뢰 단계 없이 허용 규칙과 추가 디렉터리를 적용합니다. 파일이 git에서 추적되거나 `.claude`가 심볼릭 링크인 경우, Claude Code는 이를 저장소 제공 파일로 취급하고 폴더를 신뢰할 때까지 규칙 적용을 보류합니다.

Claude Code는 git을 실행하여 둘을 구분하며, 폴더를 신뢰한 후에만 git을 실행합니다: 폴더에 대한 신뢰 대화상자를 수락했거나 신뢰가 확장되는 상위 디렉터리에 대해 수락했거나, 수락된 것으로 간주되는 `-p` 또는 SDK 세션에 있습니다. 그 전까지는 Claude Code를 시작한 위치가 파일의 규칙에 어떤 일이 발생하는지 결정합니다:

* **구성 홈에서:** Claude Code는 git을 실행하지 않고 즉시 해당 폴더의 `.claude/settings.local.json`을 적용합니다. 구성 홈은 홈 디렉터리이거나 `.claude` 하위 디렉터리를 [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars#variables)으로 설정한 디렉터리입니다. 해당 `CLAUDE_CONFIG_DIR` 디렉터리가 git 저장소 내부에 있고 Claude Code가 [저장소 루트에서 로컬 설정을 유지](/docs/ko/settings#where-claude-code-looks-for-each-file)하는 경우, 다른 곳처럼 규칙 적용을 보류합니다.
* **다른 곳:** Claude Code는 프로젝트 설정처럼 파일의 규칙 적용을 보류합니다. 확인이 실행된 후 Claude Code는 추적되지 않은 파일의 규칙이나 git 저장소 외부 디렉터리의 파일 규칙을 적용합니다. 비록 해당 정확한 폴더를 신뢰하지 않았더라도 말입니다.

<Note>
  구성 홈 예외는 신뢰 단계만 건너뜁니다. `~/.claude/settings.local.json`은 여전히 [로컬 범위](/docs/ko/settings#compare-the-scope-of-each-settings-file)이므로, Claude Code는 홈 디렉터리 자체에서 시작한 세션에서만 읽으며, 모든 프로젝트에서는 읽지 않습니다. 모든 프로젝트에 걸쳐 권한 규칙을 적용하려면 대신 사용자 설정에 추가하세요: `~/.claude/settings.json` 또는 `CLAUDE_CONFIG_DIR`이 설정된 경우 `$CLAUDE_CONFIG_DIR/settings.json`.
</Note>

버전 2.1.196부터 2.1.199까지는 Claude Code가 구성 홈에서도, git 저장소 외부에서도 파일의 규칙 적용을 보류했으며 그곳에서 [`this workspace has not been trusted`](/docs/ko/errors#workspace-has-not-been-trusted) 경고를 출력했습니다. v2.1.207 이전에는 Claude Code가 대화상자를 수락하기 전에 추적되지 않은 파일의 규칙을 적용했습니다.

<h3 id="what-runs-before-you-trust-a-folder">
  폴더를 신뢰하기 전에 실행되는 것
</h3>

각 행은 저장소가 제공할 수 있는 한 종류의 콘텐츠입니다. 열은 폴더 자체를 신뢰하지 않은 두 가지 상황입니다: 상위 폴더만 신뢰했거나, 신뢰 대화상자를 절대 표시하지 않는 `claude -p` 또는 SDK를 실행했습니다. 상위 폴더 열은 [중첩된 저장소](#project-allow-rules-and-workspace-trust) 내부에는 적용되지 않습니다: 대화형 세션에서 Claude Code는 신뢰 대화상자를 표시하며, `claude -p` 또는 SDK 실행은 `claude -p` 열을 따릅니다.

| 저장소가 제공하는 것                                                                                                                                                                                                                                                                       | 상위 폴더만 신뢰한 경우                                                                                            | `claude -p` 또는 SDK, 폴더를 신뢰하지 않은 경우                                                                                                |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| 설정 파일의 [Hooks](/docs/ko/hooks), [`env`](/docs/ko/settings-reference#env) 블록 및 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper)와 같은 도우미 명령, 그리고 프로젝트 스킬의 [hooks](/docs/ko/hooks#hooks-in-skills-and-agents) 및 [`allowed-tools`](/docs/ko/skills#pre-approve-tools-for-a-skill)                    | 사용됨                                                                                                      | 사용됨. 워크스페이스 신뢰는 어떤 세션에서도 스킬의 `allowed-tools`를 제한하지 않습니다                                                                           |
| `.claude/settings.json`의 `permissions.allow` 규칙 및 `additionalDirectories`                                                                                                                                                                                                         | 신뢰 대화상자를 수락할 때까지 사용되지 않으며, 대화상자는 다시 나타나 이를 나열합니다                                                         | 사용되지 않습니다. Claude Code는 [`this workspace has not been trusted`](/docs/ko/errors#workspace-has-not-been-trusted) 경고를 stderr에 출력합니다      |
| 프로젝트 [subagent](/docs/ko/sub-agents#hooks-in-subagent-frontmatter)의 Frontmatter hooks, 프로젝트 [`@skills-dir` plugin](/docs/ko/plugins/loading#plugins-shared-through-a-repository), 그리고 저장소 또는 `--add-dir` 디렉터리의 [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces) 항목 | 사용되지 않으며, 대화상자가 제공되지 않습니다                                                                                | 사용되지 않습니다                                                                                                                         |
| 저장소 또는 `--add-dir` 디렉터리의 subagent frontmatter에 있는 인라인 [`mcpServers`](/docs/ko/sub-agents#scope-mcp-servers-to-a-subagent). v2.1.238 이전에는 Claude Code가 두 상황 모두에서 이러한 서버를 로드했습니다                                                                                                         | 사용되지 않으며, 대화상자가 제공되지 않습니다                                                                                | 사용되지 않습니다                                                                                                                         |
| `.mcp.json`의 서버, 저장소가 [자체 설정에서 승인](/docs/ko/mcp#project-server-approvals-and-workspace-trust)하는 서버 포함                                                                                                                                                                                  | Claude Code는 연결하기 전에 묻습니다. 저장소의 자체 승인은 계산되지 않습니다                                                         | 승인 여부와 관계없이 묻지 않고 연결됩니다. SDK는 `settingSources`에 프로젝트 설정이 포함될 때만 로드합니다. 같은 폴더의 `claude mcp list`는 여전히 그러한 서버를 보류 중으로 보고합니다         |
| `.mcp.json`의 서버에 있는 [`headersHelper`](/docs/ko/mcp#trust-a-folder-before-its-headershelper-runs). v2.1.238 이전에는 Claude Code가 두 상황 모두에서 도우미를 실행했습니다                                                                                                                                     | 신뢰 대화상자를 수락할 때까지 실행되지 않으며, 대화상자는 도우미가 선언된 위치를 다시 이름으로 지정합니다. Claude Code는 그때까지 정적 `headers`만으로 서버를 연결합니다 | 실행되지 않습니다. Claude Code는 정적 `headers`만으로 서버를 연결하고 서버당 [`headersHelper not run`](/docs/ko/errors#headershelper-not-run) 줄을 stderr에 출력합니다 |

이 정확한 폴더를 신뢰해야 하는 행의 경우, 수동으로 신뢰하세요: `~/.claude.json`에서 `projects["<path>"].hasTrustDialogAccepted`를 `true`로 설정하세요. 여기서 `<path>`는 저장소 루트이거나 저장소 외부의 폴더입니다. Claude Code는 건너뛴 subagent hook 또는 인라인 MCP 서버의 디버그 로그 줄에, 건너뛴 허용 규칙의 stderr 경고에, 그리고 건너뛴 도우미의 `headersHelper not run` 줄에 정확한 키를 출력합니다.

작성하지 않은 저장소에서 `claude -p`를 실행하기 전에, 머신에서 실행할 수 있는 것을 결정하세요:

* `--setting-sources user`를 전달하거나, 프로젝트 설정 없이 SDK의 `settingSources`를 설정하여 Claude Code가 프로젝트의 설정 파일이나 `.mcp.json`을 읽지 않도록 합니다
* [`--bare`](/docs/ko/headless#start-faster-with-bare-mode)로 시작하여 Claude Code가 프로젝트에서 hooks, skills, 사용자 정의 명령, subagents, plugins, 또는 `.mcp.json` 서버를 읽지 않도록 합니다. 프로젝트의 `env` 블록 및 설정 파일의 `awsAuthRefresh`와 같은 도우미는 여전히 적용되며, Claude Code는 `apiKeyHelper`를 `--settings`에서만 읽습니다
* `--settings '{"disableAllHooks": true}'`를 전달하여 [해당 실행에 대해 hooks를 끕니다](/docs/ko/hooks#disable-or-remove-hooks). 사용자 설정에만 설정하는 것으로는 충분하지 않습니다. 저장소의 프로젝트 설정이 사용자 설정보다 우선하며 이를 `false`로 다시 설정할 수 있기 때문입니다
* [`disabledMcpjsonServers`](/docs/ko/settings-reference#disabledmcpjsonservers) 항목을 추가하여 모든 세션 유형에서 이름으로 `.mcp.json` 서버를 거부합니다

<h2 id="example-configurations">
  예시 구성
</h2>

이 [저장소](https://github.com/anthropics/claude-code/tree/main/examples/settings)에는 일반적인 배포 시나리오에 대한 시작 설정 구성이 포함되어 있습니다. 이를 시작점으로 사용하고 필요에 맞게 조정합니다.

<h2 id="see-also">
  참고 항목
</h2>

* [모든 설정](/docs/ko/settings-reference#permission-settings): 권한 키를 포함한 모든 설정 키
* [자동 모드 구성](/docs/ko/auto-mode-config): 자동 모드 분류기에 조직이 신뢰하는 인프라를 알려줍니다
* [샌드박싱](/docs/ko/sandboxing): Bash 명령에 대한 OS 수준 파일 시스템 및 네트워크 격리
* [인증](/docs/ko/authentication): Claude Code에 대한 사용자 액세스 설정
* [보안](/docs/ko/security): 보안 보호 및 모범 사례
* [훅](/docs/ko/hooks-guide): 워크플로우 자동화 및 권한 평가 확장
