> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 권한 구성

> 세분화된 권한 규칙, 모드 및 관리형 정책을 통해 Claude Code가 액세스하고 수행할 수 있는 작업을 제어합니다.

Claude Code는 에이전트가 수행할 수 있는 작업과 수행할 수 없는 작업을 정확하게 지정할 수 있도록 세분화된 권한을 지원합니다. 권한 설정은 버전 제어에 체크인할 수 있으며 조직의 모든 개발자에게 배포할 수 있을 뿐만 아니라 개별 개발자가 사용자 정의할 수 있습니다.

<h2 id="permission-system">
  권한 시스템
</h2>

Claude Code는 강력함과 안전성의 균형을 맞추기 위해 계층화된 권한 시스템을 사용합니다:

| 도구 유형   | 예시            | 승인 필요                                              | "예, 다시 묻지 않기" 동작    |
| :------ | :------------ | :------------------------------------------------- | :------------------ |
| 읽기 전용   | 파일 읽기, Grep   | 아니오, [작업 디렉토리 및 추가 디렉토리](#working-directories) 내에서 | 해당 없음               |
| Bash 명령 | 셸 실행          | 예, [읽기 전용 명령](#read-only-commands)의 기본 제공 집합 제외    | 프로젝트 디렉토리 및 명령당 영구적 |
| 파일 수정   | Edit/Write 파일 | 예                                                  | 세션 종료까지             |

Bash 또는 PowerShell 권한 프롬프트에서 `Ctrl+E`를 눌러 명령의 설명을 표시합니다: 명령이 수행하는 작업, Claude가 실행하는 이유, 발생할 수 있는 문제를 **낮은 위험**, **중간 위험** 또는 **높은 위험**으로 표시합니다. Claude Code는 모든 프롬프트에서가 아니라 `Ctrl+E`를 누를 때만 명령과 호출에 대한 Claude의 자체 설명을 모델로 전송하여 설명을 생성합니다. 설명을 표시해도 명령이 실행되지 않습니다. `Ctrl+E`를 다시 눌러 숨깁니다.

바로 가기를 끄려면 `~/.claude.json`에서 [`permissionExplainerEnabled`](/docs/ko/settings#global-config-settings)를 `false`로 설정합니다.

<h2 id="manage-permissions">
  권한 관리
</h2>

`/permissions`를 사용하여 Claude Code의 도구 권한을 보고 관리할 수 있습니다. 이 UI는 모든 권한 규칙과 이들이 출처한 `settings.json` 파일을 나열합니다.

* **Allow** 규칙을 사용하면 Claude Code가 수동 승인 없이 지정된 도구를 사용할 수 있습니다.
* **Ask** 규칙은 Claude Code가 지정된 도구를 사용하려고 할 때마다 확인을 요청합니다.
* **Deny** 규칙은 Claude Code가 지정된 도구를 사용하지 못하도록 방지합니다.

규칙은 순서대로 평가됩니다: deny, ask, allow. 해당 순서의 첫 번째 일치 항목이 결과를 결정하며, 규칙 특이성은 순서를 변경하지 않습니다.

`Bash(aws *)`와 같은 광범위한 deny 규칙은 `Bash(aws s3 ls)`와 같은 더 좁은 allow 규칙과도 일치하는 호출을 포함하여 모든 일치하는 호출을 차단하므로, deny 규칙은 허용 목록 예외를 포함할 수 없습니다. ask와 allow 사이에도 동일한 우선순위가 적용됩니다: 일치하는 ask 규칙은 동일한 호출과도 일치하는 더 구체적인 allow 규칙이 있을 때도 프롬프트를 표시합니다.

Deny 규칙은 도구 이름을 지정하는지 또는 도구 내의 패턴 범위를 지정하는지에 따라 다르게 작동합니다. `Bash`와 같은 단순 도구 이름은 도구를 Claude의 컨텍스트에서 완전히 제거하므로 Claude는 이를 볼 수 없습니다. `Bash(rm *)`와 같은 범위 지정 규칙은 도구를 사용 가능하게 유지하고 Claude가 시도할 때 일치하는 호출을 차단합니다.

<Note>
  권한 규칙은 모델이 아닌 Claude Code에 의해 적용됩니다. 프롬프트 또는 `CLAUDE.md`의 지시사항은 Claude가 시도하는 작업을 형성하지만, Claude Code가 허용하는 것을 변경하지는 않습니다. 액세스 권한을 부여하거나 취소하려면 `/permissions`, 여기에 설명된 규칙, [권한 모드](/docs/ko/permission-modes), 또는 [PreToolUse hook](#extend-permissions-with-hooks)을 사용하십시오.
</Note>

<h2 id="permission-modes">
  권한 모드
</h2>

Claude Code는 도구 승인 방식을 제어하는 여러 권한 모드를 지원합니다. [권한 모드](/docs/ko/permission-modes)에서 각 모드를 사용할 시기를 확인합니다. [설정 파일](/docs/ko/settings#settings-files)에서 `defaultMode`를 설정합니다:

| 모드                  | 설명                                                                                                                                                                                                                                                                               |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | 표준 동작: 각 도구를 처음 사용할 때 권한을 요청합니다. CLI, VS Code 및 JetBrains 확장, 데스크톱 앱에서 Manual로 표시되며, Claude Code는 `manual`을 별칭으로 허용합니다. 레이블과 별칭은 Claude Code v2.1.200 이상이 필요합니다. 데스크톱 앱의 레이블은 CLI 버전에 따라 달라지지 않습니다                                                                               |
| `acceptEdits`       | 작업 디렉토리 또는 `additionalDirectories`의 경로에 대해 파일 편집 및 일반적인 파일 시스템 명령(`mkdir`, `touch`, `mv`, `cp` 등)을 자동으로 수락합니다                                                                                                                                                                    |
| `plan`              | Claude는 파일을 읽고 읽기 전용 셸 명령을 실행하여 탐색하지만 소스 파일을 편집하지 않습니다. CLI 및 VS Code 확장에서 Plan으로 표시됩니다                                                                                                                                                                                          |
| `auto`              | 배경 안전 검사를 통해 도구 호출을 자동으로 승인하여 작업이 요청과 일치하는지 확인합니다                                                                                                                                                                                                                                |
| `dontAsk`           | `/permissions` 또는 `permissions.allow` 규칙을 통해 사전 승인되지 않은 한 도구를 자동으로 거부합니다. `AskUserQuestion`, 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), 그리고 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 허용했더라도 거부됩니다 |
| `bypassPermissions` | 명시적 `ask` 규칙, 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), 그리고 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구로 강제되는 권한 프롬프트를 제외한 모든 권한 프롬프트를 건너뜁니다. `rm -rf /`와 같은 루트 및 홈 디렉토리 제거도 회로 차단기로 여전히 프롬프트합니다     |

<Warning>
  `bypassPermissions` 모드는 `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn`, `.mvn`에 대한 쓰기를 포함한 권한 프롬프트를 건너뜁니다. 이 모드는 Claude Code가 손상을 일으킬 수 없는 컨테이너 또는 VM과 같은 격리된 환경에서만 사용합니다.

  이 모드에서는 몇 가지 프롬프트가 여전히 표시됩니다. 명시적 `ask` 규칙, 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), 그리고 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 여전히 프롬프트를 표시합니다. `rm -rf /` 및 `rm -rf ~`와 같은 파일 시스템 루트 또는 홈 디렉토리를 대상으로 하는 제거도 모델 오류에 대한 회로 차단기로 프롬프트합니다. 명령에 `$(...)`나 백틱을 사용한 명령 치환이나 `<(...)`를 사용한 프로세스 치환이 포함된 경우도 포함됩니다. v2.1.208 이전에는 `rm -rf ~`와 같이 자체 명령으로 입력된 일반 형식만 프롬프트를 표시했으며, 치환을 통해 제거에 도달한 명령은 표시하지 않았습니다.
</Warning>

`bypassPermissions` 또는 `auto` 모드가 사용되는 것을 방지하려면 [설정 파일](/docs/ko/settings#settings-files)에서 `permissions.disableBypassPermissionsMode` 또는 `permissions.disableAutoMode`를 `"disable"`로 설정합니다. 이들은 재정의될 수 없는 [관리형 설정](#managed-settings)에서 가장 유용합니다.

<h2 id="permission-rule-syntax">
  권한 규칙 구문
</h2>

권한 규칙은 `Tool` 또는 `Tool(specifier)` 형식을 따릅니다.

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

거부 및 요청 규칙은 `Tool(param:value)`를 사용하여 모든 도구의 최상위 입력 매개변수와 일치할 수 있습니다. 규칙은 Claude가 해당 매개변수가 정확한 값으로 설정된 도구를 호출할 때 일치합니다. 한 매개변수 값에 대한 허용 규칙은 호출이 전반적으로 안전하다는 것을 확립하지 않으므로, 허용 규칙은 각 도구의 자체 지정자 구문을 계속 사용합니다. 이는 도구가 허용하는 모든 스칼라 매개변수에 대해 작동합니다:

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

도구가 자체 정규화 규칙으로 이미 일치하는 필드는 이 방식으로 일치 가능하지 않습니다: Bash 및 PowerShell의 `command`, Read, Edit 및 Write의 `file_path`, Grep 및 Glob의 `path`, NotebookEdit의 `notebook_path`, WebFetch의 `url`. `Bash(command:rm *)`와 같은 규칙은 복합 명령으로 우회 가능하므로, Claude Code는 이를 무시하고 시작 경고를 발생시킵니다. 대신 `Bash(rm *)`, `Read(./path)` 또는 `WebFetch(domain:host)`를 사용합니다.

<h3 id="wildcard-patterns">
  와일드카드 패턴
</h3>

Bash 규칙은 `*`를 사용한 glob 패턴을 지원합니다. 와일드카드는 명령의 어느 위치에나 나타날 수 있습니다. 이 구성은 npm 및 git commit 명령을 허용하면서 git push를 차단합니다:

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)",
      "Bash(git * main)",
      "Bash(* --version)",
      "Bash(* --help *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

`*` 앞의 공백이 중요합니다: `Bash(ls *)`는 `ls -la`와 일치하지만 `lsof`와는 일치하지 않으며, `Bash(ls*)`는 둘 다 일치합니다. `:*` 접미사는 뒤에 오는 와일드카드를 작성하는 동등한 방법이므로 `Bash(ls:*)`는 `Bash(ls *)`와 동일한 명령과 일치합니다.

권한 대화 상자는 명령 접두사에 대해 "예, 다시 묻지 않기"를 선택할 때 공백으로 구분된 형식을 작성합니다. `:*` 형식은 패턴의 끝에서만 인식됩니다. `Bash(git:* push)`와 같은 패턴에서 콜론은 리터럴 문자로 취급되며 git 명령과 일치하지 않습니다.

<h3 id="tool-name-wildcards">
  도구 이름 와일드카드
</h3>

거부 및 요청 규칙은 도구 이름 위치에서도 glob 패턴을 허용합니다. 패턴은 전체 도구 이름과 일치해야 합니다: `"*"`는 모든 도구와 일치하며, `"mcp__*"`는 모든 서버의 모든 MCP 도구와 일치합니다. 단순 이름 glob 거부 규칙과 일치하는 도구는 Claude의 컨텍스트에서 제거되며, 이는 단순 도구 이름과 동일합니다. 이 구성은 모든 MCP 도구를 거부합니다:

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

도구 이름이 알려진 도구와 일치하지 않는 거부 또는 요청 규칙은 오타를 포착하기 위해 시작 경고를 생성합니다. `_` 또는 `*`를 포함하는 도구 이름은 확인에서 제외됩니다.

트랜스크립트 및 권한 대화 상자에서 도구에 대해 표시되는 레이블은 정규 이름과 다를 수 있습니다. 예를 들어, 트랜스크립트에서 `Stop Task`로 표시되는 도구의 정규 이름은 `TaskStop`입니다. 권한 규칙 및 [hook 매처](/docs/ko/hooks)는 정규 이름만 일치시키므로, `Stop Task`로 작성된 규칙은 일치하지 않습니다. 거부 및 요청 규칙의 경우, 위의 시작 경고가 불일치를 포착합니다. [도구 참조](/docs/ko/tools-reference)에 나열된 정규 이름을 사용합니다.

<h2 id="tool-specific-permission-rules">
  도구별 권한 규칙
</h2>

<h3 id="bash">
  Bash
</h3>

Bash 권한 규칙은 `*`를 사용한 와일드카드 일치를 지원합니다. 와일드카드는 명령의 시작, 중간 또는 끝을 포함하여 어느 위치에나 나타날 수 있습니다:

* `Bash(npm run build)`는 정확한 Bash 명령 `npm run build`와 일치합니다
* `Bash(npm run test *)`는 `npm run test`로 시작하는 Bash 명령과 일치합니다
* `Bash(npm *)`는 `npm `로 시작하는 모든 명령과 일치합니다
* `Bash(* install)`은 ` install`로 끝나는 모든 명령과 일치합니다
* `Bash(git * main)`은 `git checkout main` 및 `git log --oneline main`과 같은 명령과 일치합니다

단일 `*`는 공백을 포함한 모든 문자 시퀀스와 일치하므로 하나의 와일드카드가 여러 인수에 걸칠 수 있습니다. `Bash(git *)`는 `git log --oneline --all`과 일치하며, `Bash(git * main)`은 `git push origin main` 및 `git merge main`과 일치합니다.

`*`가 앞에 공백이 있는 끝에 나타날 때(예: `Bash(ls *)`), 단어 경계를 적용하여 접두사 뒤에 공백이나 문자열 끝이 필요합니다. 예를 들어, `Bash(ls *)`는 `ls -la`와 일치하지만 `lsof`와는 일치하지 않습니다. 반대로, 공백이 없는 `Bash(ls*)`는 단어 경계 제약이 없으므로 `ls -la`와 `lsof` 모두와 일치합니다.

<h4 id="compound-commands">
  복합 명령
</h4>

<Tip>
  Claude Code는 셸 연산자를 인식하므로 `Bash(safe-cmd *)`와 같은 규칙은 `safe-cmd && other-cmd` 명령을 실행할 권한을 부여하지 않습니다. 인식되는 명령 구분자는 `&&`, `||`, `;`, `|`, `|&`, `&` 및 줄바꿈입니다. 규칙은 각 서브명령과 독립적으로 일치해야 합니다.
</Tip>

"예, 다시 묻지 않기"로 복합 명령을 승인하면 Claude Code는 전체 복합 문자열에 대한 단일 규칙이 아니라 승인이 필요한 각 서브명령에 대해 별도의 규칙을 저장합니다. 예를 들어, `git status && npm test`를 승인하면 `npm test`에 대한 규칙을 저장하므로 향후 `npm test` 호출은 `&&` 앞에 무엇이 있든 인식됩니다. `cd`를 서브디렉토리로 이동하는 것과 같은 서브명령은 해당 경로에 대한 자체 Read 규칙을 생성합니다. 단일 복합 명령에 대해 최대 5개의 규칙이 저장될 수 있습니다.

<h4 id="process-wrappers">
  프로세스 래퍼
</h4>

Bash 규칙을 일치시키기 전에 Claude Code는 고정된 프로세스 래퍼 집합을 제거하므로 `Bash(npm test *)`와 같은 규칙은 `timeout 30 npm test`도 일치합니다. 인식되는 래퍼는 `timeout`, `time`, `nice`, `nohup` 및 `stdbuf`입니다.

베어 `xargs`도 제거되므로 `Bash(grep *)`는 `xargs grep pattern`과 일치합니다. 제거는 `xargs`에 플래그가 없을 때만 적용됩니다: `xargs -n1 grep pattern`과 같은 호출은 `xargs` 명령으로 일치되므로 내부 명령에 대해 작성된 규칙은 이를 포함하지 않습니다.

이 래퍼 목록은 기본 제공되며 구성할 수 없습니다. `direnv exec`, `devbox run`, `mise exec`, `npx` 및 `docker exec`과 같은 개발 환경 러너는 목록에 없습니다. 이러한 도구는 인수를 명령으로 실행하므로 `Bash(devbox run *)`와 같은 규칙은 `devbox run rm -rf .`를 포함하여 `run` 뒤에 오는 모든 것과 일치합니다. 환경 러너 내에서 작업을 승인하려면 `Bash(devbox run npm test)`와 같이 러너와 내부 명령을 모두 포함하는 특정 규칙을 작성합니다. 허용하려는 각 내부 명령에 대해 하나의 규칙을 추가합니다.

`watch`, `setsid`, `ionice` 및 `flock`과 같은 Exec 래퍼는 항상 프롬프트하며 `Bash(watch *)`와 같은 접두사 규칙으로 자동 승인될 수 없습니다. 동일한 사항이 `-exec` 또는 `-delete`를 사용하는 `find`에 적용됩니다: `Bash(find *)` 규칙은 이러한 형식을 포함하지 않습니다. 특정 호출을 승인하려면 전체 명령 문자열에 대한 정확한 일치 규칙을 작성합니다.

<h4 id="read-only-commands">
  읽기 전용 명령
</h4>

Claude Code는 기본 제공 Bash 명령 집합을 읽기 전용으로 인식하고 모든 모드에서 권한 프롬프트 없이 실행합니다. 여기에는 `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` 및 `git`의 읽기 전용 형식이 포함됩니다. 집합은 구성할 수 없습니다. 이러한 명령 중 하나에 대해 프롬프트를 요구하려면 `ask` 또는 `deny` 규칙을 추가합니다.

따옴표 없는 glob 패턴은 모든 플래그가 읽기 전용인 명령에 대해 허용되므로 `ls *.ts` 및 `wc -l src/*.py`는 프롬프트 없이 실행됩니다. `find`, `sort`, `sed` 및 `git`과 같이 쓰기 가능하거나 실행 가능한 플래그가 있는 명령은 glob이 `-delete`와 같은 플래그로 확장될 수 있으므로 따옴표 없는 glob이 있을 때 여전히 프롬프트합니다.

작업 디렉토리 또는 [추가 디렉토리](#working-directories) 내의 경로로의 `cd`도 읽기 전용입니다. `cd packages/api && ls`와 같은 복합 명령은 각 부분이 자체적으로 적격일 때 프롬프트 없이 실행됩니다. 하나의 복합 명령에서 `cd`와 `git`을 결합하면 `cd`가 다른 디렉토리로 변경될 때 프롬프트합니다. 새 디렉토리에서 `git`을 실행하면 해당 디렉토리의 훅을 실행할 수 있기 때문입니다. 대상이 현재 작업 디렉토리로 해결되는 `cd`는 작동하지 않으며 이 프롬프트를 트리거하지 않습니다.

하나의 복합 명령에서 `cd`와 출력 리다이렉트를 결합하면 Claude Code가 `cd` 실행 후 리다이렉트 대상이 해결되는 디렉토리를 결정할 수 없을 때 프롬프트합니다. 유일한 리다이렉트 대상이 `/dev/null`인 명령(예: `cd app; grep -r pattern . 2>/dev/null`)은 `/dev/null`이 작업 디렉토리에 의존하지 않으므로 이 프롬프트를 트리거하지 않습니다. v2.1.207 이전에는 `cd`를 포함하는 복합 명령이 `/dev/null`로의 리다이렉트를 포함한 모든 출력 리다이렉트에 대해 프롬프트했습니다.

<Warning>
  명령 인수를 제약하려고 시도하는 Bash 권한 패턴은 취약합니다. 예를 들어, `Bash(curl http://github.com/ *)`는 curl을 GitHub URL로 제한하려고 하지만 다음과 같은 변형과는 일치하지 않습니다:

  * URL 앞의 옵션: `curl -X GET http://github.com/...`
  * 다른 프로토콜: `curl https://github.com/...`
  * 리다이렉트: `curl -L http://bit.ly/xyz` (GitHub로 리다이렉트)
  * 변수: `URL=http://github.com && curl $URL`
  * 추가 공백: `curl  http://github.com`

  더 안정적인 URL 필터링을 위해 다음을 고려합니다:

  * **Bash 네트워크 도구 제한**: deny 규칙을 사용하여 `curl`, `wget` 및 유사한 명령을 차단한 다음 허용된 도메인에 대해 `WebFetch(domain:github.com)` 권한으로 WebFetch 도구를 사용합니다
  * **PreToolUse 훅 사용**: Bash 명령의 URL을 검증하고 허용되지 않은 도메인을 차단하는 훅을 구현합니다
  * **CLAUDE.md 지침 추가**: `CLAUDE.md`에서 허용된 curl 패턴을 설명합니다. 이는 Claude가 시도하는 것을 형성하지만 경계를 적용하지 않으므로 위의 옵션 중 하나와 함께 사용합니다

  WebFetch만 사용하는 것은 네트워크 액세스를 방지하지 않습니다. Bash가 허용되면 Claude는 여전히 `curl`, `wget` 또는 다른 도구를 사용하여 모든 URL에 도달할 수 있습니다.
</Warning>

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

`Edit` 규칙은 파일을 편집하는 모든 기본 제공 도구에 적용됩니다. Claude는 Grep 및 Glob과 같이 파일을 읽는 모든 기본 제공 도구에 `Read` 규칙을 적용하기 위해 최선을 다합니다. 또한 프롬프트의 `@file` 언급 및 연결된 [IDE](/docs/ko/vs-code#the-built-in-ide-mcp-server)가 Claude와 공유하는 선택 및 열린 파일 컨텍스트에도 적용합니다.

`Read` deny 규칙은 동일한 경로에서 [Edit 도구](/docs/ko/errors#file-is-covered-by-a-read-deny-rule)도 차단하며, 새 파일 생성도 포함됩니다. Write 및 NotebookEdit은 포함되지 않으므로 도구가 변경할 수 없는 경로에 대해 `Edit` deny 규칙을 추가합니다. Claude Code v2.1.208 이상이 필요합니다.

<Warning>
  Read 및 Edit deny 규칙은 Claude의 기본 제공 파일 도구 및 Bash에서 Claude Code가 인식하는 `cat`, `head`, `tail` 및 `sed`와 같은 파일 명령에 적용됩니다. 이들은 파일을 간접적으로 읽거나 쓰는 Python 또는 Node 스크립트와 같은 임의의 서브프로세스에는 적용되지 않습니다. 경로에 대한 모든 프로세스의 액세스를 차단하는 OS 수준 적용을 위해 [샌드박싱을 활성화합니다](/docs/ko/sandboxing).
</Warning>

Read 및 Edit 규칙은 모두 [gitignore](https://git-scm.com/docs/gitignore) 사양을 따르며 4가지 고유한 패턴 유형이 있습니다:

| 패턴                 | 의미               | 예시                               | 일치                                     |
| ------------------ | ---------------- | -------------------------------- | -------------------------------------- |
| `//path`           | 파일 시스템 루트의 절대 경로 | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`              |
| `~/path`           | 홈 디렉토리의 경로       | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`         |
| `/path`            | 설정 소스에 상대적인 경로   | `Edit(/src/**/*.ts)`             | `<project root>/src/**/*.ts` (프로젝트 설정) |
| `path` 또는 `./path` | 현재 디렉토리에 상대적인 경로 | `Read(*.env)`                    | `<cwd>/*.env`                          |

<Warning>
  `/Users/alice/file`과 같은 패턴은 절대 경로가 아닙니다. 단일 슬래시는 설정 소스에 앵커됩니다. 절대 경로의 경우 `//Users/alice/file`을 사용합니다.
</Warning>

`/path` 패턴은 설정 파일이 정의된 디렉토리에 앵커되므로 동일한 규칙은 배치 위치에 따라 다른 위치와 일치합니다:

| 규칙 정의 위치                                  | `/path` 해석                 |
| :---------------------------------------- | :------------------------- |
| `.claude/settings.json`과 같은 프로젝트 또는 로컬 설정 | `<project root>/path`      |
| `~/.claude/settings.json`의 사용자 설정         | `~/.claude/path`           |
| `--settings <file>`로 전달된 파일               | `<directory of file>/path` |
| CLI 플래그, `/permissions` 또는 세션 규칙          | `<original cwd>/path`      |

deny 규칙(예: 사용자 설정의 `Read(/secrets/**)`)은 프로젝트의 `secrets` 디렉토리가 아니라 `~/.claude/secrets/**`를 차단합니다. 모든 프로젝트 내에서 적용되는 사용자 설정의 규칙을 작성하려면 `//` 절대 경로 또는 `~/` 홈 상대 경로를 대신 사용합니다.

Windows에서 경로는 일치하기 전에 POSIX 형식으로 정규화됩니다. `C:\Users\alice`는 `/c/Users/alice`가 되므로 `//c/**/.env`를 사용하여 해당 드라이브의 어디든 `.env` 파일과 일치시킵니다. 모든 드라이브에서 일치시키려면 `//**/.env`를 사용합니다.

예시:

* `Edit(/docs/**)`: `<project>/docs/`의 편집 (NOT `/docs/` and NOT `<project>/.claude/docs/`)
* `Read(~/.zshrc)`: 홈 디렉토리의 `.zshrc` 읽기
* `Edit(//tmp/scratch.txt)`: 절대 경로 `/tmp/scratch.txt` 편집
* `Read(src/**)`: `<current-directory>/src/`에서 읽기

규칙은 해당 앵커 아래의 파일만 일치하므로 앵커는 deny 규칙이 얼마나 멀리 도달하는지를 결정합니다. 베어 파일명은 gitignore 의미론을 따르고 어느 깊이에서나 일치하므로 `Read(.env)` 및 `Read(**/.env)`는 동등합니다:

| Deny 규칙                         | 차단                         | 차단하지 않음                    |
| ------------------------------- | -------------------------- | -------------------------- |
| `Read(.env)` 또는 `Read(**/.env)` | 현재 디렉토리 또는 그 아래의 모든 `.env` | 상위 디렉토리 또는 다른 프로젝트의 `.env` |
| `Read(//**/.env)`               | 파일 시스템의 어디든 모든 `.env`      | 없음; 규칙은 파일 시스템 루트에 앵커됨     |

<Note>
  gitignore 패턴에서 `*`는 단일 디렉토리의 파일과 일치하고 `**`는 디렉토리 전체에서 재귀적으로 일치합니다. 모든 파일 액세스를 허용하려면 괄호 없이 도구 이름만 사용합니다: `Read`, `Edit` 또는 `Write`.
</Note>

"예, 다시 묻지 않기"로 파일 경로를 승인하면 Claude Code는 `[`, `]` 및 `*`와 같은 gitignore 패턴 문자를 해당 경로에서 이스케이프하므로 생성된 규칙은 승인한 리터럴 경로와만 일치합니다. 직접 작성한 규칙은 이스케이프되지 않습니다. v2.1.202 이전에는 Claude Code가 경로를 이스케이프되지 않은 상태로 저장했으므로 `[2024-06] Reports`라는 이름의 디렉토리에 대해 생성된 규칙이 자신의 경로와 일치하지 않거나 의도하지 않은 형제 디렉토리와 일치할 수 있었습니다.

Claude가 심볼릭 링크에 액세스할 때 권한 규칙은 두 경로를 확인합니다: 심볼릭 링크 자체와 이것이 해결되는 파일입니다. Allow 및 deny 규칙은 해당 쌍을 다르게 취급합니다: allow 규칙은 프롬프트로 폴백하고 deny 규칙은 즉시 차단합니다.

* **Allow 규칙**: 심볼릭 링크 경로와 해당 대상이 모두 일치할 때만 적용됩니다. 허용된 디렉토리 내의 심볼릭 링크가 외부를 가리키면 여전히 프롬프트합니다.
* **Deny 규칙**: 심볼릭 링크 경로 또는 대상이 일치할 때 적용됩니다. 거부된 파일을 가리키는 심볼릭 링크는 자체적으로 거부됩니다.

예를 들어, `Read(./project/**)` allowed 및 `Read(~/.ssh/**)` denied를 사용하면 `./project/key`의 심볼릭 링크가 `~/.ssh/id_rsa`를 가리킬 때 차단됩니다: 대상이 allow 규칙에 실패하고 deny 규칙과 일치합니다.

<h3 id="webfetch">
  WebFetch
</h3>

WebFetch 규칙은 `domain:` 접두사를 사용하고 요청된 URL의 호스트명과 일치합니다. 일치는 대소문자를 구분하지 않으며, `*` 와일드카드를 지원하고, 규칙과 호스트명 모두에서 후행 `.`을 제거하므로 `example.com.`과 `example.com`은 동일하게 취급됩니다.

* `WebFetch(domain:example.com)`은 `example.com`으로의 요청과 일치합니다
* `WebFetch(domain:*.example.com)`은 `api.example.com` 또는 `a.b.example.com`과 같은 모든 깊이의 모든 서브도메인과 일치하지만 `example.com` 자체는 일치하지 않습니다
* `WebFetch(domain:*)`는 모든 도메인과 일치하며 베어 `WebFetch` 규칙과 동등합니다

선행 `*.` 또는 베어 `*`가 아닌 다른 위치에서 와일드카드는 두 점 사이의 텍스트와만 일치합니다. `WebFetch(domain:example.*)`는 `example.org`과 일치합니다. 여기서 `*`는 `org`가 되지만 `example.evil.com`과는 일치하지 않습니다. 여기서 `*`는 `evil.com`이 되어야 하고 점을 넘어야 합니다. 이는 후행 와일드카드가 공격자가 등록할 수 있는 도메인과 일치하는 것을 방지합니다.

<h3 id="mcp">
  MCP
</h3>

MCP 규칙은 Claude Code에서 구성된 서버 이름을 사용하며, 선택적으로 해당 서버의 도구 이름이 뒤따릅니다.

* `mcp__puppeteer`는 `puppeteer` 서버에서 제공하는 모든 도구와 일치합니다
* `mcp__puppeteer__*`는 와일드카드 구문을 사용하며 `puppeteer` 서버의 모든 도구와도 일치합니다
* `mcp__puppeteer__puppeteer_navigate`는 `puppeteer` 서버에서 제공하는 `puppeteer_navigate` 도구와 일치합니다

조직이 [claude.ai 커넥터](/docs/ko/mcp#organization-controls-on-connector-tools) 도구를 `ask`로 설정한 경우, 해당 도구에 대한 allow 규칙은 적용되지 않습니다: Claude Code는 `auto` 및 `bypassPermissions` 모드에서도 모든 호출에 대해 프롬프트합니다. 절대 프롬프트하지 않는 `dontAsk` 모드에서는 Claude Code가 호출을 거부합니다. 커넥터 도구는 `mcp__claude_ai_<server>__<tool>`로 나타납니다.

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

`Cd` allow 규칙을 추가하면 `/cd`를 허용 목록 모드로 전환합니다: 해결된 대상 디렉토리는 allow 규칙 중 하나와 일치해야 하거나 `/cd`는 거부합니다. `Cd` 규칙이 구성되지 않으면 `/cd`는 기본 동작을 유지하고 익숙하지 않은 디렉토리를 신뢰하도록 프롬프트합니다.

경로 패턴은 [Read 및 Edit 규칙](#read-and-edit)의 `//`, `~/` 및 `/` 앵커를 공유하지만 일치는 gitignore 스타일이 아닌 전체 디렉토리 경로에 앵커됩니다. `*`는 정확히 하나의 경로 세그먼트와 일치하고 `**`는 세그먼트 전체에서 일치합니다. 후행 `/**`는 또한 명명된 루트와 일치합니다.

| 규칙                    | 일치                              | 일치하지 않음                    |
| --------------------- | ------------------------------- | -------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                    | `~/code/app/src`, `~/code` |
| `Cd(~/code/**)`       | `~/code` 및 그 아래의 모든 디렉토리        | `~/code` 외부의 디렉토리          |
| `Cd(**/node_modules)` | 어느 깊이에서나 모든 `node_modules` 디렉토리 | `node_modules/pkg`         |

<h2 id="extend-permissions-with-hooks">
  훅으로 권한 확장
</h2>

[Claude Code 훅](/docs/ko/hooks-guide)은 런타임에 권한 평가를 수행하기 위해 사용자 정의 셸 명령을 등록하는 방법을 제공합니다. Claude Code가 도구 호출을 할 때, PreToolUse 훅은 권한 프롬프트 전에 실행됩니다. 훅 출력은 도구 호출을 거부하거나, 프롬프트를 강제하거나, 프롬프트를 건너뛰어 호출을 진행하도록 할 수 있습니다.

훅 결정은 권한 규칙을 우회하지 않습니다. Claude Code는 훅이 `"allow"` 또는 `"ask"`를 반환한 후에도 deny 및 ask 규칙을 평가합니다. 일치하는 deny 규칙은 호출을 차단하고, 일치하는 ask 규칙은 훅이 `"allow"` 또는 `"ask"`를 반환했을 때도 여전히 프롬프트합니다. 이는 [권한 관리](#manage-permissions)에서 설명한 deny 우선 우선순위를 유지하며, 관리형 설정에서 설정한 deny 규칙을 포함합니다.

커넥터 도구([조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools))와 [`requiresUserInteraction`으로 표시된](/docs/ko/mcp#require-approval-for-a-specific-tool) MCP 도구도 훅이 `"allow"`를 반환할 때 여전히 프롬프트합니다.

차단 훅은 또한 allow 규칙보다 우선합니다. 종료 코드 2로 종료되는 훅은 권한 규칙이 평가되기 전에 도구 호출을 중지하므로 allow 규칙이 호출을 허용할 수 있는 경우에도 차단이 적용됩니다. 모든 Bash 명령을 프롬프트 없이 실행하되 차단하려는 몇 가지를 제외하려면 allow 목록에 `"Bash"`를 추가하고 해당 특정 명령을 거부하는 PreToolUse 훅을 등록합니다. 적응할 수 있는 훅 스크립트는 [보호된 파일에 대한 편집 차단](/docs/ko/hooks-guide#block-edits-to-protected-files)을 참조합니다.

<h2 id="working-directories">
  작업 디렉토리
</h2>

기본적으로 Claude는 시작된 디렉토리의 파일에 액세스할 수 있습니다. 이 액세스를 확장할 수 있습니다:

* **시작 중**: `--add-dir <path>` CLI 인수 사용
* **세션 중**: `/add-dir` 명령 사용
* **영구 구성**: [설정 파일](/docs/ko/settings#settings-files)의 `additionalDirectories`에 추가

추가 디렉토리의 파일은 원래 작업 디렉토리와 동일한 권한 규칙을 따릅니다: 프롬프트 없이 읽을 수 있게 되며, 파일 편집 권한은 현재 권한 모드를 따릅니다.

macOS의 백그라운드 세션에서 세션 호스트는 Claude가 파일을 읽거나 쓸 필요가 있을 때 터미널과 별도로 `~/Desktop`, `~/Documents`, `~/Downloads`와 같은 보호된 폴더에 대한 액세스를 요청합니다. 읽기가 `Operation not permitted`로 실패하면 [백그라운드 세션에 폴더 액세스를 부여하는 방법](/docs/ko/agent-view#background-sessions-cant-read-desktop-documents-or-downloads-on-macos)을 참조하세요.

세션의 기본 작업 디렉토리를 다른 디렉토리를 추가하는 대신 변경하려면 [`/cd`](/docs/ko/commands)를 사용합니다. `/cd` 명령은 Claude Code v2.1.169 이상이 필요합니다. `/add-dir`과 달리 세션을 재배치합니다: 새 디렉토리의 `CLAUDE.md`가 로드되고 `--resume`은 해당 위치에서 세션을 찾습니다.

<h3 id="additional-directories-grant-file-access-not-configuration">
  추가 디렉토리는 파일 액세스를 부여하며, 구성은 아닙니다
</h3>

디렉토리를 추가하면 Claude가 파일을 읽고 편집할 수 있는 위치가 확장됩니다. 해당 디렉토리를 전체 구성 루트로 만들지는 않습니다: 대부분의 `.claude/` 구성은 추가 디렉토리에서 발견되지 않지만 몇 가지 유형은 예외로 로드됩니다.

이러한 예외는 `--add-dir` 플래그 또는 `/add-dir` 명령으로 추가된 디렉토리에만 적용됩니다. 설정 파일의 `permissions.additionalDirectories`에 나열된 디렉토리는 파일 액세스만 부여하며 아래의 구성을 로드하지 않습니다.

다음 구성 유형은 `--add-dir` 디렉토리에서 로드됩니다:

| 구성                                                                                | `--add-dir`에서 로드됨                                                                                                       |
| :-------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `.claude/skills/`의 [Skills](/docs/ko/skills)                                           | 예, 라이브 리로드 포함                                                                                                           |
| `.claude/agents/`의 [Subagents](/docs/ko/sub-agents)                                    | 예                                                                                                                       |
| `.claude/settings.json` 및 `.claude/settings.local.json`의 [Settings](/docs/ko/settings) | `enabledPlugins` 및 `extraKnownMarketplaces` 키만                                                                          |
| [CLAUDE.md](/docs/ko/memory) 파일, `.claude/rules/` 및 `CLAUDE.local.md`                  | `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`이 설정된 경우에만. `CLAUDE.local.md`는 추가로 `local` 설정 소스가 필요하며, 이는 기본적으로 활성화됩니다 |

명령 및 출력 스타일은 현재 작업 디렉토리 및 해당 부모, `~/.claude/`의 사용자 디렉토리 및 관리형 설정에서 발견됩니다. Hooks 및 기타 `settings.json` 키는 현재 작업 디렉토리의 `.claude/` 폴더에서 로드되며 부모 디렉토리 폴백이 없고, 사용자 `~/.claude/settings.json` 및 관리형 설정과 함께 로드됩니다. 프로젝트 전체에서 해당 구성을 공유하려면 다음 방법 중 하나를 사용합니다:

* **사용자 수준 구성**: `~/.claude/agents/`, `~/.claude/output-styles/` 또는 `~/.claude/settings.json`에 파일을 배치하여 모든 프로젝트에서 사용 가능하게 합니다
* **Plugins**: 팀이 설치할 수 있는 [plugin](/docs/ko/plugins)으로 구성을 패키징하고 배포합니다
* **구성 디렉토리에서 시작**: 원하는 `.claude/` 구성이 포함된 디렉토리에서 Claude Code를 실행합니다

<h2 id="how-permissions-interact-with-sandboxing">
  권한이 샌드박싱과 상호 작용하는 방식
</h2>

권한과 [샌드박싱](/docs/ko/sandboxing)은 상호 보완적인 보안 계층입니다:

* **권한**은 Claude Code가 사용할 수 있는 도구와 액세스할 수 있는 파일 또는 도메인을 제어합니다. Bash, Read, Edit, WebFetch, MCP를 포함한 모든 도구에 적용됩니다.
* **샌드박싱**은 Bash 도구의 파일 시스템 및 네트워크 액세스를 제한하는 OS 수준 적용을 제공합니다. Bash 명령 및 해당 자식 프로세스에만 적용됩니다.

심층 방어를 위해 둘 다 사용합니다:

* 권한 deny 규칙은 Claude가 제한된 리소스에 액세스하려고 시도하는 것을 차단합니다
* 샌드박스 제한은 프롬프트 주입이 Claude의 의사 결정을 우회하더라도 Bash 명령이 정의된 경계 외부의 리소스에 도달하는 것을 방지합니다
* 샌드박스의 파일 시스템 제한은 [`sandbox.filesystem`](/docs/ko/sandboxing) 설정을 Read 및 Edit deny 규칙과 결합합니다. 둘 다 최종 샌드박스 경계로 병합됩니다
* 네트워크 제한은 WebFetch 권한 규칙을 샌드박스의 `allowedDomains` 및 `deniedDomains` 목록과 결합합니다

샌드박싱이 `autoAllowBashIfSandboxed: true`로 활성화되면(기본값), 권한에 `Bash` ask 규칙이 포함되어 있거나 [동등한 `Bash(*)` 형식](#match-all-uses-of-a-tool)이 포함되어 있어도 샌드박스된 Bash 명령은 프롬프트 없이 실행됩니다. 샌드박스 경계는 전체 도구 프롬프트를 대체합니다. 다음 검사가 여전히 적용됩니다:

* `Bash(git push *)`와 같은 콘텐츠 범위 ask 규칙은 여전히 프롬프트를 강제합니다
* 명시적 deny 규칙은 여전히 적용됩니다
* `/`, 홈 디렉터리 또는 기타 중요한 시스템 경로를 대상으로 하는 `rm` 또는 `rmdir` 명령은 여전히 프롬프트를 트리거합니다

제외된 명령과 같이 샌드박스에서 실행되지 않는 명령은 일반적인 `Bash` ask 규칙을 따릅니다. [샌드박스 모드](/docs/ko/sandboxing#sandbox-modes)를 참조하여 이 동작을 변경합니다.

<h2 id="managed-settings">
  관리형 설정
</h2>

Claude Code 구성에 대한 중앙 집중식 제어가 필요한 조직의 경우, 관리자는 사용자 또는 프로젝트 설정으로 재정의할 수 없는 관리형 설정을 배포할 수 있습니다. 이러한 정책 설정은 일반 설정 파일과 동일한 형식을 따르며 MDM/OS 수준 정책, 관리형 설정 파일, [서버 관리형 설정](/docs/ko/server-managed-settings) 또는 자체 호스팅 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)를 통해 전달될 수 있습니다. 전달 메커니즘 및 파일 위치는 [설정 파일](/docs/ko/settings#settings-files)을 참조합니다.

<h3 id="managed-only-settings">
  관리형 전용 설정
</h3>

다음 설정은 관리형 설정에서만 읽혀집니다. 사용자 또는 프로젝트 설정 파일에 배치하면 효과가 없습니다.

| 설정                                             | 설명                                                                                                                                                                                                                                                        |
| :--------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowAllClaudeAiMcps`                         | `true`일 때, claude.ai 커넥터는 배포된 `managed-mcp.json`과 함께 로드되며 그 배타적 제어에 의해 억제되지 않습니다. [관리형 MCP 구성](/docs/ko/managed-mcp) 참조                                                                                                                                        |
| `allowedChannelPlugins`                        | 메시지를 푸시할 수 있는 채널 플러그인의 허용 목록입니다. `channelsEnabled: true`가 필요할 때 기본 Anthropic 허용 목록을 대체합니다. [채널 플러그인이 실행될 수 있는 것을 제한합니다](/docs/ko/channels#restrict-which-channel-plugins-can-run) 참조                                                                           |
| `allowManagedHooksOnly`                        | `true`일 때, 관리형 훅, SDK 훅 및 관리형 설정 `enabledPlugins`에서 강제 활성화된 플러그인의 훅만 로드됩니다. 사용자, 프로젝트 및 다른 모든 플러그인 훅은 차단됩니다                                                                                                                                               |
| `allowManagedMcpServersOnly`                   | `true`일 때, 관리형 설정의 `allowedMcpServers`만 존중됩니다. `deniedMcpServers`는 여전히 모든 소스에서 병합됩니다. [관리형 MCP 구성](/docs/ko/managed-mcp) 참조                                                                                                                                    |
| `allowManagedPermissionRulesOnly`              | `true`일 때, 사용자 및 프로젝트 설정이 `allow`, `ask` 또는 `deny` 권한 규칙을 정의하는 것을 방지합니다. 관리형 설정의 규칙만 적용됩니다. MCP 서버 허용 목록에는 영향을 주지 않습니다. 그 경우 `allowManagedMcpServersOnly`를 설정합니다                                                                                          |
| `blockedMarketplaces`                          | 마켓플레이스 소스의 차단 목록입니다. 차단된 소스는 다운로드 전에 확인되므로 파일 시스템에 닿지 않습니다. [관리형 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions) 참조                                                                                                                  |
| `channelsEnabled`                              | 조직을 위한 [채널](/docs/ko/channels)을 허용합니다. 각 플랜의 기본값은 [엔터프라이즈 제어](/docs/ko/channels#enterprise-controls)를 참조합니다                                                                                                                                                         |
| `disableSideloadFlags`                         | `--plugin-dir`, `--plugin-url`, `--agents` 및 `--mcp-config` CLI 플래그를 시작 시 거부합니다. 이것이 없으면 사용자는 이러한 플래그를 전달하여 단일 실행을 위해 `strictKnownMarketplaces`를 우회할 수 있습니다. [`disableSideloadFlags`](/docs/ko/settings#available-settings) 참조. Claude Code v2.1.193 이상이 필요합니다 |
| `forceRemoteSettingsRefresh`                   | `true`일 때, 원격 관리형 설정이 새로 가져올 때까지 CLI 시작을 차단하고 가져오기에 실패하면 종료합니다. [실패 폐쇄 적용](/docs/ko/server-managed-settings#enforce-fail-closed-startup) 참조                                                                                                                    |
| `pluginTrustMessage`                           | 설치 전에 표시되는 플러그인 신뢰 경고에 추가되는 사용자 정의 메시지                                                                                                                                                                                                                    |
| `sandbox.filesystem.allowManagedReadPathsOnly` | `true`일 때, 관리형 설정의 `filesystem.allowRead` 경로만 존중됩니다. `denyRead`는 여전히 모든 소스에서 병합됩니다                                                                                                                                                                        |
| `sandbox.network.allowManagedDomainsOnly`      | `true`일 때, 관리형 설정의 `allowedDomains` 및 `WebFetch(domain:...)` allow 규칙만 존중됩니다. 허용되지 않은 도메인은 사용자에게 프롬프트하지 않고 자동으로 차단됩니다. 거부된 도메인은 여전히 모든 소스에서 병합됩니다                                                                                                         |
| `strictKnownMarketplaces`                      | 사용자가 추가하고 플러그인을 설치할 수 있는 플러그인 마켓플레이스 소스를 제어합니다. [관리형 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions) 참조                                                                                                                              |
| `strictPluginOnlyCustomization`                | 사용자 및 프로젝트 소스에서 스킬, 에이전트, 훅 및 MCP 서버를 차단하므로 플러그인 또는 관리형 설정에서만 올 수 있습니다. `true`는 네 가지 모두를 잠금; `["skills", "hooks"]`와 같은 배열은 명명된 것만 잠금. [`strictPluginOnlyCustomization`](/docs/ko/settings#strictpluginonlycustomization) 참조                                    |
| `wslInheritsWindowsSettings`                   | Windows HKLM 레지스트리 키 또는 `C:\Program Files\ClaudeCode\managed-settings.json`에서 `true`일 때, WSL은 `/etc/claude-code`에 추가로 Windows 정책 체인에서 관리형 설정을 읽습니다. [설정 파일](/docs/ko/settings#settings-files) 참조                                                               |

`disableBypassPermissionsMode`는 일반적으로 조직 정책을 적용하기 위해 관리형 설정에 배치되지만 모든 범위에서 작동합니다. 사용자는 자신의 설정에서 이를 설정하여 자신을 우회 모드에서 잠글 수 있습니다.

<Note>
  Team 및 Enterprise 플랜에서 Owner는 [Claude Code 관리자 설정](https://claude.ai/admin-settings/claude-code)에서 [Remote Control](/docs/ko/remote-control) 및 [웹 세션](/docs/ko/claude-code-on-the-web)을 조직 전체에서 활성화하거나 비활성화합니다. Remote Control은 [`disableRemoteControl`](/docs/ko/settings#available-settings) 설정으로 장치별로 추가로 비활성화할 수 있습니다. 웹 세션에는 장치별 관리형 설정 키가 없습니다.
</Note>

<h2 id="settings-precedence">
  설정 우선순위
</h2>

권한 규칙은 다른 모든 Claude Code 설정과 동일한 [설정 우선순위](/docs/ko/settings#settings-precedence)를 따릅니다:

1. **관리형 설정**: 명령줄 인수를 포함한 다른 수준으로 재정의할 수 없습니다
2. **명령줄 인수**: 임시 세션 재정의
3. **로컬 프로젝트 설정** (`.claude/settings.local.json`)
4. **공유 프로젝트 설정** (`.claude/settings.json`)
5. **사용자 설정** (`~/.claude/settings.json`)

도구가 어느 수준에서든 거부되면 다른 수준은 이를 허용할 수 없습니다. 예를 들어, 관리형 설정 deny는 `--allowedTools`로 재정의할 수 없으며, `--disallowedTools`는 관리형 설정이 정의하는 것 이상의 제한을 추가할 수 있습니다.

설정 범위 전체에서도 동일하게 적용됩니다: 사용자 설정에서 권한을 허용하고 프로젝트 설정에서 거부하면, deny 규칙이 이를 차단합니다. 그 반대도 마찬가지입니다: 사용자 수준의 deny는 프로젝트 수준의 allow를 차단합니다. 왜냐하면 모든 범위의 deny 규칙이 allow 규칙보다 먼저 평가되기 때문입니다.

Embedding hosts는 [`parentSettingsBehavior`](/docs/ko/settings#settings-precedence)가 `"merge"`로 설정되어 있을 때 SDK `managedSettings` 옵션을 통해 추가 관리형 정책을 제공할 수 있습니다. Embedder 값은 정책을 강화할 수 있지만 완화할 수는 없습니다.

<h2 id="project-allow-rules-and-workspace-trust">
  프로젝트 허용 규칙 및 워크스페이스 신뢰
</h2>

프로젝트의 `.claude/settings.json`에 있는 `permissions.allow` 규칙과 `permissions.additionalDirectories` 항목은 기능을 부여하므로, Claude Code는 해당 워크스페이스에 대해 [워크스페이스 신뢰 대화상자](/docs/ko/security#additional-safeguards)를 수락한 후에만 이를 적용합니다. 그 전까지는 Claude Code가 규칙을 읽지만 적용하지 않습니다. 신뢰 대화상자는 폴더가 부여할 허용 규칙과 추가 디렉터리를 나열하므로 수락하기 전에 검토할 수 있습니다. `deny` 및 `ask` 규칙은 영향을 받지 않습니다. 이들은 제한만 하기 때문입니다.

Claude Code는 워크스페이스별로 신뢰를 저장하며, git 저장소 루트를 기준으로 하거나, 저장소 외부에서는 Claude Code를 시작한 디렉터리를 기준으로 합니다. 홈 디렉터리에서 시작하면 신뢰는 현재 세션에만 유지되며 디스크에 기록되지 않습니다. [추가 보안 조치](/docs/ko/security#additional-safeguards) 참고를 참조하세요. 상위 디렉터리를 신뢰해도 중첩된 프로젝트의 허용 규칙은 적용되지 않습니다.

`.claude/settings.local.json`은 사용자 자신의 파일이므로 워크스페이스 신뢰 확인이 일반적으로 적용되지 않습니다. 저장소가 파일을 제공할 수 있는 경우(예: git에 커밋되었거나 `.claude`가 심볼릭 링크인 경우) 해당 허용 규칙과 추가 디렉터리는 프로젝트 설정처럼 신뢰 확인을 거칩니다.

Claude Code는 저장소가 파일을 제공했는지 확인하기 위해 git을 실행하며, 해당 폴더 또는 상위 디렉터리 중 하나에 대해 수락된 신뢰 대화상자로 커버되는 폴더에서만 해당 확인을 실행합니다. 아직 신뢰하지 않은 폴더의 대화형 세션에서는 아래에 설명된 대로 세션이 자신의 구성 홈에서 실행되지 않는 한 `.claude/settings.local.json`의 허용 규칙과 추가 디렉터리는 프로젝트 설정처럼 신뢰 확인을 거칩니다. 아래의 두 가지 예외 중에서 대화상자 전에는 구성 홈 예외만 적용됩니다. 이는 git을 실행할 필요가 없기 때문입니다. 디렉터리가 git 저장소 내부에 있지 않다는 것을 확인하는 것은 동일한 git 확인을 사용하므로 폴더를 커버하는 신뢰 대화상자가 수락되면 저장소 내부가 아닌 예외가 적용됩니다. v2.1.207 이전에는 추적되지 않은 `.claude/settings.local.json`이 대화상자를 수락하기 전에 해당 폴더에서 허용 규칙을 적용했습니다.

`.claude/settings.local.json`의 허용 규칙과 추가 디렉터리는 두 가지 경우에 워크스페이스 신뢰 없이도 적용됩니다:

* Claude Code를 시작한 디렉터리가 git 저장소 내부에 있지 않은 경우.
* 세션이 사용자 자신의 구성 홈에서 실행되는 경우: 홈 디렉터리 또는 `.claude` 하위 디렉터리를 [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)로 설정한 모든 디렉터리.

두 경우 모두 파일은 저장소가 제공할 수 있는 파일이 아니라 사용자가 만든 파일이며, 저장소에 커밋된 `.claude/settings.local.json`은 여전히 워크스페이스 신뢰가 필요합니다. 버전 2.1.196부터 2.1.199까지는 해당 워크스페이스에서 파일을 저장소 제공 파일로 취급하고 허용 규칙을 무시했으며 stderr에 [`this workspace has not been trusted`](/docs/ko/errors#workspace-has-not-been-trusted) 경고를 출력했습니다. 위의 두 가지 예외는 v2.1.195 이전 버전과 일치하며 v2.1.200에서 복원되었습니다.

또한 v2.1.200부터 허용 규칙이나 추가 디렉터리가 여전히 적용되지 않지만 상위 디렉터리가 이미 신뢰되어 신뢰 대화상자를 표시하지 않은 워크스페이스는 다음에 대화형으로 Claude Code를 시작할 때 대화상자를 표시합니다. 대화상자는 두 가지 선택지를 제공합니다:

* **Yes, I trust this folder**: 해당 워크스페이스에 대한 신뢰를 저장하고 같은 세션에서 규칙을 적용합니다.
* **No, continue without these permissions**: 해당 규칙을 무시한 상태로 계속 작동합니다. 대화상자는 다음 세션에 다시 나타납니다.

[비대화형 모드](/docs/ko/headless)에서 `-p`를 사용하면 대화상자가 나타나지 않고 규칙은 무시된 상태로 유지됩니다.

<h2 id="example-configurations">
  예시 구성
</h2>

이 [저장소](https://github.com/anthropics/claude-code/tree/main/examples/settings)에는 일반적인 배포 시나리오에 대한 시작 설정 구성이 포함되어 있습니다. 이를 시작점으로 사용하고 필요에 맞게 조정합니다.

<h2 id="see-also">
  참고 항목
</h2>

* [설정](/docs/ko/settings): 권한 설정 테이블을 포함한 완전한 구성 참조
* [자동 모드 구성](/docs/ko/auto-mode-config): 자동 모드 분류기에 조직이 신뢰하는 인프라를 알려줍니다
* [샌드박싱](/docs/ko/sandboxing): Bash 명령에 대한 OS 수준 파일 시스템 및 네트워크 격리
* [인증](/docs/ko/authentication): Claude Code에 대한 사용자 액세스 설정
* [보안](/docs/ko/security): 보안 보호 및 모범 사례
* [훅](/docs/ko/hooks-guide): 워크플로우 자동화 및 권한 평가 확장
