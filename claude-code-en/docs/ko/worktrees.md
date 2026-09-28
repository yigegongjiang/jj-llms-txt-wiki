> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# worktree를 사용하여 병렬 세션 실행

> git worktree에서 병렬 Claude Code 세션을 격리하여 변경 사항이 충돌하지 않도록 합니다. `--worktree` 플래그, 서브에이전트 격리, `.worktreeinclude`, 정리 및 비git VCS 훅을 다룹니다.

[git worktree](https://git-scm.com/docs/git-worktree)는 자체 파일과 브랜치를 가진 별도의 작업 디렉토리이며, 메인 체크아웃과 동일한 저장소 히스토리 및 원격을 공유합니다. 각 Claude Code 세션을 자체 worktree에서 실행하면 한 세션의 편집이 다른 세션의 파일을 건드리지 않으므로, 한 세션이 기능을 구축하는 동안 두 번째 세션이 버그를 수정할 수 있습니다.

<Note>
  Worktree는 git 저장소가 필요합니다. 다른 버전 관리 시스템의 경우 [훅을 구성하여 git 로직을 대체](#non-git-version-control)합니다. [데스크톱 앱](/docs/ko/desktop#work-in-parallel-with-sessions)에서는 세션을 시작할 때 **worktree** 옵션을 선택하여 자체 worktree를 제공합니다.
</Note>

Worktree는 Claude를 병렬로 실행하는 여러 방법 중 하나입니다. 이들은 파일 편집을 격리합니다. [서브에이전트](/docs/ko/sub-agents)는 한 세션 내에서 작업을 분할하고, [크로스 세션 메시징](/docs/ko/cross-session-messaging)을 통해 Claude는 worktree의 세션 간에 발견 사항을 전달할 수 있습니다. [Claude를 병렬로 실행](/docs/ko/agents)을 참조하여 접근 방식을 비교하거나, [worktree로 서브에이전트 격리](#isolate-subagents-with-worktrees)로 건너뛰어 worktree와 서브에이전트를 함께 사용합니다.

대부분의 세션은 처음 두 섹션만 필요합니다: [worktree에서 Claude 시작](#start-claude-in-a-worktree), 그 다음 [종료 시 정리](#clean-up-worktrees). [세션 재개](#resume-a-worktree-session), [worktree 생성 방식 변경](#customize-worktree-creation) 또는 [실패 디버깅](#troubleshooting)이 필요할 때 페이지의 나머지 부분으로 돌아갑니다.

<h2 id="start-claude-in-a-worktree">
  worktree에서 Claude 시작
</h2>

`--worktree` 또는 `-w`를 이름과 함께 전달하여 격리된 worktree를 생성하고 Claude를 시작합니다. 기본적으로 worktree는 저장소 루트의 `.claude/worktrees/<name>/` 아래에 생성되며, `worktree-<name>`이라는 새 브랜치에 생성됩니다:

```bash theme={null}
claude --worktree feature-auth
```

다른 터미널에서 다른 이름으로 명령을 다시 실행하여 두 번째 격리된 세션을 시작합니다. 이름을 생략하면 Claude가 `bright-running-fox`와 같은 이름을 생성합니다.

대화형 실행에는 [작업 공간 신뢰](/docs/ko/security)가 필요합니다. 이전에 디렉토리에서 Claude를 실행하지 않았다면 해당 디렉토리에서 `claude`를 한 번 실행하여 신뢰 대화를 수락하거나, `--worktree`는 오류와 함께 종료되고 먼저 이를 수행하도록 요청합니다. `-p`를 사용한 비대화형 실행은 신뢰 확인을 건너뛰므로 `claude -p --worktree`는 이를 수행하지 않고 진행됩니다.

<Tip>
  `.claude/worktrees/`를 `.gitignore`에 추가하여 worktree 내용이 메인 체크아웃에서 추적되지 않은 파일로 나타나지 않도록 합니다.
</Tip>

<h3 id="set-up-the-worktree-environment">
  worktree 환경 설정
</h3>

Worktree는 새로운 체크아웃이므로 개발 환경을 초기화합니다. Claude에게 종속성을 설치하도록 요청하거나, `.claude/worktrees/` 아래의 worktree 디렉토리에서 프로젝트의 설정을 직접 실행합니다. `.env`와 같은 gitignored 파일을 모든 새 worktree로 자동으로 전달하려면 [`.worktreeinclude` 파일](#copy-gitignored-files-into-worktrees)을 추가합니다.

<h3 id="ask-claude-to-create-a-worktree">
  Claude에게 worktree 생성 요청
</h3>

세션 중에 Claude에게 "worktree에서 작업하기"를 요청할 수도 있으며, [`EnterWorktree`](/docs/ko/tools-reference) 도구로 하나를 생성합니다. Worktree에 들어가면 Claude는 `.claude/worktrees/` 아래의 다른 worktree로 `EnterWorktree`를 호출하여 직접 전환할 수 있습니다. 이전 worktree는 디스크에 그대로 남아 있습니다.

Claude가 저장소의 `.claude/worktrees/` 디렉토리 외부의 경로에 들어가면 Claude Code는 먼저 승인을 요청합니다. 왜냐하면 이동은 세션의 작업 디렉토리, 쓰기 액세스 및 `CLAUDE.md` 및 설정과 같은 프로젝트 구성을 해당 위치로 이동하기 때문입니다. `EnterWorktree` [권한 규칙](/docs/ko/permissions) 또는 "다시 묻지 않기"를 선택해도 이 프롬프트를 억제하지 않습니다. `bypassPermissions` 모드만 이를 건너뜁니다. v2.1.206 이전에는 Claude가 승인을 요청하지 않고 기존 worktree 경로에 들어갈 수 있었습니다.

<Note>
  **훅 경로는 worktree를 따르지 않습니다.** Claude가 worktree에 들어간 후 Claude Code는 [훅](/docs/ko/hooks#reference-scripts-by-path)에서 `${CLAUDE_PROJECT_DIR}`을 원래 위치에 유지하고 worktree 경로를 다른 방식으로 전달합니다:

  * **`${CLAUDE_PROJECT_DIR}` 유지**: 세션이 시작된 프로젝트 루트를 계속 가리키므로 `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh`와 같은 훅 명령은 메인 체크아웃에서 스크립트를 실행합니다.
  * **`cwd`는 Claude를 따릅니다**: 훅의 [입력 JSON](/docs/ko/hooks#common-input-fields)의 `cwd` 필드는 worktree 루트이며, Claude가 `cd`를 실행할 때 다시 이동합니다. 훅이 worktree 경로가 필요할 때 읽습니다.
</Note>

<h2 id="clean-up-worktrees">
  worktree 정리
</h2>

대화형 worktree 세션을 종료할 때 Claude는 worktree에서 제거로 인해 삭제될 작업을 확인합니다: 변경되거나 추적되지 않은 파일, 체크아웃된 서브모듈 내의 커밋되지 않은 작업, 그리고 새로운 커밋입니다.

* **Worktree가 깨끗함**: 이름 없는 세션의 경우 Claude는 worktree와 해당 브랜치를 자동으로 제거합니다. [이름이 지정된](/docs/ko/sessions#name-your-sessions) 세션은 나중을 위해 worktree를 유지할 수 있도록 먼저 프롬프트를 표시합니다.
* **Worktree에 작업이 있음**: Claude는 worktree를 유지하거나 제거할지 묻습니다. 유지하면 디렉토리와 브랜치가 보존되어 나중에 돌아올 수 있습니다. 제거하면 worktree 디렉토리와 해당 브랜치가 삭제되며, 그 안의 모든 작업도 함께 삭제됩니다.
* **Worktree의 상태를 확인할 수 없음**: Claude Code가 worktree의 변경 사항을 계산할 수 없거나 서브모듈 체크아웃을 검사할 수 없을 때, worktree를 자동으로 제거하지 않고 프롬프트를 표시합니다. 프롬프트는 확인할 수 없었던 항목을 명시합니다.

`-p`를 사용한 비대화형 실행에는 종료 프롬프트가 없으므로 Claude는 해당 worktree를 정리하지 않으며, Claude Code는 생성 시 각 worktree에 대해 가져온 잠금을 유지합니다. 나중 세션의 [stale-lock sweep](#clean-up-subagent-and-background-session-worktrees)이 이를 해제할 때까지 유지됩니다. 하나를 제거하려면 `git worktree remove`를 실행합니다. git이 worktree가 잠겨 있다고 거부하면 먼저 `git worktree unlock`을 실행합니다.

Windows에서 worktree를 제거해도 worktree 외부의 파일은 삭제되지 않습니다. Worktree 내부의 폴더가 다른 곳으로의 링크(예: NTFS 접합점 또는 디렉토리 심볼릭 링크)인 경우 Claude Code는 링크만 삭제하고 가리키는 폴더는 유지합니다. v2.1.205 이전에는 서브디렉토리에 중첩된 링크가 있는 worktree를 제거하면 가리키는 폴더를 삭제할 수 있었습니다.

<h2 id="resume-a-worktree-session">
  worktree 세션 재개
</h2>

Worktree 내부에 있던 세션을 재개할 때 Claude Code는 세션을 해당 worktree로 반환합니다. 이는 대화형 재개, [비대화형 모드](/docs/ko/headless)에서 `-p`를 사용한 `--continue` 및 `--resume`, 그리고 Agent SDK에 적용됩니다. Worktree 내부로 돌아가면 Claude는 여전히 [`ExitWorktree`](/docs/ko/tools-reference) 도구로 이를 종료할 수 있습니다.

Claude Code가 세션을 worktree로 반환하기 전에 worktree가 여전히 메인 체크아웃과 별도의 체크아웃인지 확인하고, 확인에 실패한 worktree로 다시 들어가기를 거부합니다. Git worktree의 경우 확인은 git 메타데이터를 읽습니다. [`WorktreeCreate` 훅](#non-git-version-control)이 생성한 것과 같은 git 메타데이터가 없는 worktree는 확인을 통과할 수 있습니다. Claude Code가 여전히 거부하는 경우는 [Claude Code가 worktree 사용을 거부](#claude-code-refuses-to-use-a-worktree)에 나열되어 있으며 복구 방법도 함께 제시됩니다. 메시지 및 각각에서 복구하는 방법은 [세션이 worktree 외부에서 재개](#the-session-resumes-outside-its-worktree)를 참조합니다.

실행 위치와 재개 방식에 따라 Claude Code가 다시 들어가는 것이 변경됩니다:

* **실행 디렉토리**: 메인 체크아웃 또는 저장소의 다른 디렉토리에서 재개합니다. Claude Code는 실행 위치 내부에서 실행하더라도 `.claude/worktrees/` 아래에서 git으로 생성한 worktree로 다시 들어갑니다. 다른 worktree 내부에서 실행할 때 Claude Code는 거기서 이를 보증할 수 있는 경우에만 다시 들어갑니다: 자체 저장소인 worktree, git 메타데이터가 없는 worktree, 또는 `git worktree add`로 생성한 worktree의 서브디렉토리에서 실행하면 거부되므로 메인 체크아웃에서 실행합니다.
* **`--fork-session`**: 포크된 세션은 Claude를 실행한 디렉토리에서 시작하며 Claude Code는 원본 세션의 worktree를 그대로 둡니다.
* **삭제된 worktree**: worktree 디렉토리가 더 이상 존재하지 않으면 Claude Code는 Claude를 실행한 디렉토리에서 세션을 재개합니다. Worktree가 없어졌음을 알려주고 세션의 worktree 바인딩을 지웁니다.

<Note>
  v2.1.212 이전에는 비대화형 재개가 시작 디렉토리에 머물렀고 `ExitWorktree`는 활성 worktree 세션이 없다고 보고했습니다.
</Note>

Claude가 Claude Code가 git으로 생성한 worktree에 들어가거나 나갈 때 트랜스크립트가 따릅니다: Claude Code는 [`/cd`](/docs/ko/commands)와 동일한 방식으로 세션의 새로운 작업 디렉토리 아래에 세션을 기록하므로 `/desktop`과 `--resume`이 거기서 이를 찾습니다. 나가면 같은 방식으로 다시 이동합니다. [`WorktreeCreate` 훅](#non-git-version-control)으로 생성된 worktree는 시작 디렉토리에 트랜스크립트를 유지합니다. Claude Code v2.1.198 이상이 필요합니다.

<h2 id="how-claude-code-enforces-isolation">
  Claude Code가 격리를 강제하는 방식
</h2>

세션이 worktree에 격리되어 있는 동안 Claude Code는 아래 확인이 정의하는 도구 호출을 차단합니다. `--worktree`로 세션을 시작했든, Claude가 `EnterWorktree`로 worktree에 들어갔든, 또는 worktree 세션을 재개했든 동일한 규칙이 적용됩니다.

동일한 강제는 격리된 세션에서 생성하는 모든 서브에이전트를 포함합니다. 세션이 대화형이든 [백그라운드](/docs/ko/agent-view#how-file-edits-are-isolated)에서 실행되든 적용됩니다. [자체 worktree에서 실행되는 서브에이전트](#isolate-subagents-with-worktrees)는 동일한 확인을 수행합니다. 해당 버전 히스토리는 [서브에이전트 파일 작성](/docs/ko/sub-agents#write-subagent-files) 아래에 있습니다.

Claude Code는 네 가지 확인을 적용합니다:

* **파일 편집**: Claude Code는 메인 체크아웃의 경로를 대상으로 하는 `Edit`, `Write` 또는 `NotebookEdit`을 차단합니다.
* **명령 작업 디렉토리**: Claude Code는 작업 디렉토리가 메인 체크아웃으로 확인되거나 그 외부에 머물러 있는지 확인할 수 없는 Bash, PowerShell 또는 Monitor 명령을 차단합니다.
* **Git 리다이렉트**: Claude Code는 git을 메인 체크아웃으로 리다이렉트하는 Bash 또는 Monitor 명령을 차단합니다. 리다이렉트는 `git -C`, `--git-dir`, `GIT_DIR` 또는 `GIT_WORK_TREE` 변수, 또는 git을 실행하기 전에 메인 체크아웃으로 `cd`를 통해 올 수 있습니다.
* **명령 형태**: Claude Code는 명령 텍스트에서 명령이 실행하는 모든 git이 worktree 내부에 머물러 있는지 확인할 수 없을 때 Bash 또는 Monitor 명령을 차단합니다. 예를 들어 명령 이름이 런타임에 계산되거나 구문을 파싱할 수 없거나 `${!name}` 또는 `${ command; }`와 같은 확장이 텍스트에서 명시하지 않은 명령을 실행할 수 있을 때 발생합니다. Claude Code는 Claude에게 거부된 명령을 다시 작성하는 방법을 알려줍니다. 예를 들어 이를 일반 별도 명령으로 분할합니다. 이 확인을 끌 수 없습니다.

확인은 Claude Code를 실행한 저장소에 적용됩니다. 또한 연결된 worktree가 연결된 메인 체크아웃도 포함합니다. PowerShell 명령의 경우 Claude Code는 작업 디렉토리 확인만 적용합니다.

Claude는 각 거부를 worktree의 이름을 지정하고 진행 방법을 설명하는 도구 오류로 봅니다. 거부된 명령의 경우 [거부 메시지의 의미와 이를 해결하는 방법](/docs/ko/errors#command-blocked-by-the-worktree-isolation-checks)을 참조하세요.

<h2 id="isolate-subagents-with-worktrees">
  worktree로 서브에이전트 격리
</h2>

서브에이전트는 자체 worktree에서 실행될 수 있으므로 병렬 편집이 충돌하지 않습니다. Claude에게 "에이전트에 worktree 사용"을 요청하거나, [사용자 정의 서브에이전트](/docs/ko/sub-agents#supported-frontmatter-fields)에 대해 frontmatter에 `isolation: worktree`를 추가하여 격리를 영구적으로 만듭니다.

`.claude/agents/`의 이 서브에이전트는 항상 자체 worktree에서 실행됩니다:

```markdown theme={null}
---
name: refactorer
description: Applies mechanical refactors across many files
isolation: worktree
---

Apply the requested refactor across every affected file, then run the tests
and report the results.
```

각 서브에이전트는 서브에이전트가 변경 없이 완료되면 Claude Code가 자동으로 제거하는 임시 worktree를 가져옵니다. 변경 사항이 있는 worktree는 아래의 [주기적 스윕](#clean-up-subagent-and-background-session-worktrees)이 작업을 잃지 않고 제거할 수 있을 때까지 디스크에 유지됩니다.

서브에이전트 worktree는 `--worktree`와 동일한 [기본 브랜치](#choose-the-base-branch)를 사용하므로, `worktree.baseRef`가 `"head"`로 설정되지 않은 한 저장소의 기본 브랜치에서 분기합니다.

<h3 id="clean-up-subagent-and-background-session-worktrees">
  서브에이전트 및 백그라운드 세션 worktree 정리
</h3>

Claude Code는 Claude가 서브에이전트 및 [백그라운드 세션](/docs/ko/agent-view#how-file-edits-are-isolated)을 위해 생성한 worktree를 [`cleanupPeriodDays`](/docs/ko/settings-reference#cleanupperioddays) 설정보다 오래되면 제거하는 주기적 스윕을 실행하며, [보존 스윕 규칙](/docs/ko/claude-directory#cleaned-up-automatically)을 따릅니다.

`--worktree` 세션을 [백그라운드](/docs/ko/agent-view#send-the-session-to-the-background)로 보낼 때 해당 worktree는 스윕이 제거할 수 있는 백그라운드 세션 worktree가 됩니다. 스윕은 다음 경우에 worktree를 제자리에 둡니다:

* Worktree가 여전히 작업을 보유하고 있습니다: 변경되거나 추적되지 않은 파일, 또는 푸시되지 않은 커밋.
* worktree의 체크아웃된 서브모듈이 변경되거나 추적되지 않은 파일을 보유하고 있거나, Claude Code가 worktree의 서브모듈을 검사할 수 없습니다. 이 확인에는 Claude Code v2.1.274 이상이 필요합니다.
* [worktree 생성을 차단하는 네 가지 경우](#git-lfs-content-is-missing-from-a-worktree-claude-code-created) 중 하나가 적용됩니다: Claude Code가 저장소 구성이 정의하는 필터 드라이버를 결정할 수 없거나, 여기서 전환할 수 없는 설정을 찾습니다.
* Worktree가 백그라운드하지 않은 `--worktree` 세션에 속합니다. 나이와 관계없이.
* `git worktree add`로 직접 worktree를 생성했습니다. 나중에 `--worktree <name>` 세션을 실행하고 해당 세션을 백그라운드했더라도.

Claude Code는 생성하는 모든 git worktree에 git 메타데이터에 마커를 작성하며, 스윕은 마커가 없는 worktree를 유지합니다. [`WorktreeCreate` 훅](#non-git-version-control)이 생성한 worktree를 포함합니다. v2.1.246 이전에는 스윕이 마커를 확인하지 않았으며, 오래된 백그라운드 세션 레코드가 가리킬 때 직접 생성한 worktree를 제거할 수 있었습니다.

에이전트가 실행 중인 동안 Claude Code는 해당 worktree에서 `git worktree lock`을 유지하여 동시 정리가 이를 제거할 수 없도록 하며, 에이전트가 완료되면 잠금을 해제합니다. Claude Code는 백그라운드된 세션을 위해 생성한 worktree에서 세션이 실행되는 동안 동일한 잠금을 유지하므로 스윕은 worktree를 제자리에 두고 `git worktree remove`는 이를 제거하기를 거부합니다.

스윕은 또한 프로세스가 종료된 세션을 위해 Claude Code가 설정한 잠금을 해제하므로 종료된 백그라운드 세션은 worktree를 영구적으로 잠긴 상태로 두지 않습니다. 스윕은 `git worktree lock`으로 직접 설정한 잠금을 절대 해제하지 않습니다. v2.1.210 이전에는 종료된 세션으로 인한 잠금이 `git worktree unlock`을 실행할 때까지 제자리에 남아 있었습니다.

스윕이 유지하는 worktree를 정리하려면 `git worktree remove`를 실행하고, worktree에 커밋되지 않은 변경 사항이나 추적되지 않은 파일이 있으면 `--force`를 추가합니다. Git이 worktree가 잠겨 있다고 거부하면 먼저 `git worktree unlock`을 실행합니다.

<h2 id="customize-worktree-creation">
  worktree 생성 사용자 정의
</h2>

Claude Code의 worktree 생성 기본값은 대부분의 세션을 포함합니다: `.claude/worktrees/` 아래에 생성하고, 저장소의 기본 브랜치에서 분기하며, 추적된 파일만 체크아웃합니다. 이 섹션의 옵션은 이러한 기본값을 변경합니다.

<h3 id="choose-the-base-branch">
  기본 브랜치 선택
</h3>

새 worktree는 저장소의 기본 브랜치에서 분기하므로 대부분의 세션은 이 설정이 필요하지 않습니다. [설정](/docs/ko/settings-reference#worktree)에서 `worktree.baseRef`를 설정하여 현재 작업에서 분기합니다. 설정은 두 가지 값을 허용합니다:

* `"fresh"` (기본값): 원격의 저장소 기본 브랜치(보통 `main`)에서 분기하므로 worktree는 원격과 일치하는 깨끗한 트리에서 시작합니다.
* `"head"`: 현재 로컬 `HEAD`에서 분기하므로 worktree는 푸시되지 않은 커밋과 기능 브랜치 상태를 유지합니다. 진행 중인 작업에서 작동해야 하는 서브에이전트를 격리할 때 사용합니다. Worktree 내부에서 `"head"`는 메인 체크아웃의 `HEAD`가 아닌 해당 worktree의 `HEAD`로 확인됩니다.

`worktree.baseRef`를 브랜치 이름으로 설정할 수 없습니다. 특정 기존 브랜치에서 worktree를 시작하려면 [git으로 직접 생성](#manage-worktrees-manually)합니다.

`"fresh"` 기본의 경우 Claude Code는 `origin/HEAD`를 최신으로 유지합니다: 저장소를 지난 24시간 동안 페치하지 않았으면 기본 브랜치를 페치하며, 5초로 제한되고 페치가 실패하면 로컬로 캐시된 ref를 사용합니다. 원격이 구성되지 않았거나 `origin/HEAD`가 로컬로 캐시되지 않았고 페치할 수 없으면 worktree는 현재 로컬 `HEAD`로 폴백합니다. v2.1.208 이전에는 새로운 worktree가 이미 로컬로 캐시된 `origin/HEAD`를 사용했습니다.

이 예제는 모든 새 worktree가 현재 작업에서 분기하도록 만듭니다:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

<h3 id="branch-from-a-pull-request">
  풀 요청에서 분기
</h3>

특정 풀 요청 또는 병합 요청에서 분기하려면 `--worktree`에 `#`이 앞에 붙은 번호, GitHub 풀 요청 URL 또는 `https://gitlab.com/group/repo/-/merge_requests/123`과 같은 GitLab 병합 요청 URL을 전달합니다. Claude Code는 `origin`에서 해당 변경의 헤드 커밋을 페치하고 `.claude/worktrees/pr-<number>`에서 worktree를 생성합니다. 셸이 `#`을 주석의 시작으로 취급하지 않도록 인수를 인용합니다:

```bash theme={null}
claude --worktree "#1234"
```

Claude Code는 URL에서 번호만 읽습니다. 항상 저장소의 `origin` 원격에서 페치하며, `origin`의 호스트로 페치 경로를 선택합니다:

* **github.com**: `pull/<number>/head` 페치
* **gitlab.com**: `merge-requests/<number>/head` 페치
* **GitHub Enterprise, 자체 관리 GitLab 또는 기타 호스트**: 먼저 `pull/<number>/head`를 시도한 다음 `merge-requests/<number>/head`를 시도

v2.1.233 이전에는 Claude Code가 `--worktree`에 대해 `#<number>` 및 GitHub 스타일 풀 요청 URL만 허용했으며, 항상 `pull/<number>/head`를 페치했습니다.

<h3 id="copy-gitignored-files-into-worktrees">
  gitignored 파일을 worktree로 복사
</h3>

Worktree는 새로운 체크아웃이므로 메인 저장소의 `.env` 또는 `.env.local`과 같은 추적되지 않은 파일이 없습니다. Claude가 worktree를 생성할 때 자동으로 복사하려면 프로젝트 루트에 `.worktreeinclude` 파일을 추가합니다.

파일은 `.gitignore` 구문을 사용합니다. 패턴과 일치하고 gitignored된 파일만 복사되므로 추적된 파일은 절대 중복되지 않습니다.

`**/`로 시작하는 패턴을 작성하고 원하는 파일이 전체적으로 gitignored된 디렉토리 내부에 있으면 Claude Code는 해당 디렉토리 자체가 패턴과 일치하거나 `**/` 후의 첫 번째 이름이 디렉토리 경로의 이름 중 하나일 때만 복사합니다. 예를 들어 `**/.claude/skills/*.md`를 작성하면 첫 번째 이름은 `.claude`이므로 Claude Code는 무시된 `.claude/` 디렉토리에서 일치하는 파일을 복사합니다. `**/` 패턴이 도달하지 않는 무시된 디렉토리에서 파일을 복사하려면 패턴에서 디렉토리의 이름을 지정합니다: `**/config.json` 대신 `vendor/**/config.json`을 작성합니다. v2.1.239 이전에는 Claude Code가 디렉토리 자체가 패턴과 일치할 때만 `**/` 패턴에 대해 전체적으로 무시된 디렉토리에서 파일을 복사했습니다.

이 `.worktreeinclude`는 두 개의 env 파일과 시크릿 구성을 각 새 worktree로 복사합니다:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

이는 Claude Code가 git으로 생성하는 모든 worktree에 적용됩니다: `--worktree` worktree, [서브에이전트 worktree](#isolate-subagents-with-worktrees) 및 [데스크톱 앱](/docs/ko/desktop#work-in-parallel-with-sessions)의 병렬 세션. [`WorktreeCreate` 훅](#non-git-version-control)을 사용하면 훅 스크립트 내에서 파일을 복사합니다.

<h3 id="reuse-a-worktree-name">
  worktree 이름 재사용
</h3>

`--worktree`에 디렉토리가 이미 존재하는 이름을 전달하면 새로운 worktree를 생성하는 대신 기존 worktree를 엽니다.

기본 `"fresh"` [기본](#choose-the-base-branch)을 사용하면 다음 모든 조건이 충족될 때 재개된 worktree는 이전 팁에서 계속하는 대신 저장소의 기본 브랜치로 재설정됩니다:

* 커밋되지 않은 변경 사항이나 추적되지 않은 파일이 없습니다.
* 여전히 Claude Code가 생성한 브랜치에 있습니다.
* 자체 커밋이 없거나 풀 요청 또는 병합 요청이 병합되고 원격 브랜치가 삭제되었습니다.

Claude Code는 git 상태만으로 병합된 경우를 감지합니다: worktree가 푸시한 원격 브랜치가 더 이상 존재하지 않으며, worktree의 모든 커밋이 이미 기본 브랜치에 있습니다.

다른 모든 경우에 Claude Code는 worktree를 이전 팁에서 재개합니다:

* Worktree가 조건 중 하나라도 실패합니다.
* Claude Code가 worktree의 상태를 확인할 수 없습니다.
* `worktree.baseRef`가 `"head"`입니다.
* 이름이 풀 요청 또는 병합 요청 참조입니다.

v2.1.208 이전에는 이름을 재사용할 때 Claude Code는 항상 이전 worktree를 이전 팁에서 재개했습니다.

<h3 id="replace-worktree-creation-with-a-hook">
  worktree 생성을 훅으로 대체
</h3>

[`WorktreeCreate` 훅](/docs/ko/hooks#worktreecreate)을 구성하여 기본 `git worktree` 로직을 완전히 대체합니다. 여기에는 worktree를 `.claude/worktrees/` 이외의 다른 곳에 배치하는 것도 포함됩니다. 완전한 예제는 [비git 버전 관리](#non-git-version-control)를 참조합니다.

<h2 id="what-worktrees-share-with-the-main-checkout">
  worktree가 메인 체크아웃과 공유하는 것
</h2>

Worktree는 자체 파일과 브랜치를 가지지만 메인 체크아웃과 다음을 공유합니다:

* **저장소의 `.git` 디렉토리**: worktree의 git 명령은 메인 저장소의 공유 `.git` 디렉토리에 쓰며, [샌드박싱](/docs/ko/sandboxing#filesystem-isolation)은 이러한 쓰기를 허용하므로 `git commit`과 같은 명령이 샌드박스가 활성화된 worktree 내부에서 작동합니다.
* **플러그인**: [프로젝트 범위](/docs/ko/plugins/loading#find-where-a-plugin-is-enabled)에서 메인 체크아웃에서 설치된 플러그인도 동일한 저장소의 worktree에 로드되므로 worktree마다 다시 설치할 필요가 없습니다. Claude Code v2.1.200 이상이 필요합니다.
* **권한 승인**: worktree 세션에서 Bash 명령에 대해 "예, 다시 묻지 않기"를 선택하면 규칙이 메인 체크아웃의 `.claude/settings.local.json`에 저장되므로 메인 체크아웃과 저장소의 다른 모든 worktree에 적용되며, worktree 제거 후에도 유지됩니다. Windows 및 Claude Code가 [저장소 루트를 사용하지 않는 다른 경우](/docs/ko/settings#where-claude-code-looks-for-each-file)에는 규칙이 해당 worktree와 함께 유지됩니다. v2.1.211 이전에는 worktree에서 부여된 승인이 해당 worktree 내부에 저장되었으며, 다른 곳에 적용되지 않았고, worktree 제거 시 손실되었습니다. [승인이 저장되는 위치](/docs/ko/permissions#permission-system)를 참조합니다.
* **추적되지 않은 skills, agents, 및 commands**: worktree 체크아웃에 루트에 `.claude/skills` 디렉토리가 없을 때(예: `.claude/skills`가 gitignored인 경우), Claude Code는 메인 체크아웃의 [프로젝트 skills](/docs/ko/skills#where-skills-live)를 worktree 세션에 로드합니다. 자체 `.claude/skills` 디렉토리가 있는 worktree에서는 해당 복사본만 로드됩니다.

  동일한 읽기 통과는 `.claude/agents` 및 `.claude/commands`를 포함합니다. Skills의 경우 읽기 통과에는 Claude Code v2.1.277 이상이 필요합니다.

이 모든 것들은 `--worktree`로 worktree를 생성했든, `git worktree add`로 생성했든, 또는 [데스크톱 앱](/docs/ko/desktop#work-in-parallel-with-sessions)을 통해 생성했든 적용됩니다.

<h2 id="manage-worktrees-manually">
  worktree 수동 관리
</h2>

특정 기존 브랜치를 체크아웃하거나 worktree를 저장소 외부에 배치해야 할 때 Git으로 직접 worktree를 생성합니다.

새 브랜치에서 worktree 생성:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

기존 브랜치에서 worktree 생성. `fix-issue-456`을 저장소에 이미 존재하는 브랜치로 바꿉니다:

```bash theme={null}
git worktree add ../project-bugfix fix-issue-456
```

worktree에서 Claude 시작:

```bash theme={null}
cd ../project-feature-a
claude
```

worktree 나열:

```bash theme={null}
git worktree list
```

완료되면 제거:

```bash theme={null}
git worktree remove ../project-feature-a
```

전체 명령 참조는 [Git worktree 문서](https://git-scm.com/docs/git-worktree)를 참조합니다.

<h2 id="non-git-version-control">
  비git 버전 관리
</h2>

Worktree 격리는 기본적으로 git을 사용합니다. SVN, Perforce, Mercurial 또는 기타 시스템의 경우 [`WorktreeCreate` 및 `WorktreeRemove` 훅](/docs/ko/hooks#worktreecreate)을 구성하여 사용자 정의 생성 및 정리 로직을 제공합니다. 훅이 기본 git 동작을 대체하므로 `--worktree`를 사용할 때 [`.worktreeinclude`](#copy-gitignored-files-into-worktrees)가 처리되지 않습니다. 훅 스크립트 내에서 대신 로컬 구성 파일을 복사합니다.

이 `WorktreeCreate` 훅은 stdin에서 `jq`를 사용하여 worktree 이름을 읽고, 새로운 SVN 작업 복사본을 체크아웃하고, Claude Code가 세션의 작업 디렉토리로 사용할 수 있도록 디렉토리 경로를 인쇄합니다. [`settings.json`](/docs/ko/settings#where-settings-live)에 구성을 추가합니다:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

세션이 끝날 때 정리하려면 `WorktreeRemove` 훅과 쌍을 이룹니다. 입력 스키마 및 제거 예제는 [훅 참조](/docs/ko/hooks#worktreecreate)를 참조합니다.

`WorktreeCreate` 훅을 사용하면 git 저장소 외부에서 [`/batch`](/docs/ko/commands#all-commands)를 실행할 수도 있습니다. 각 `/batch` 서브에이전트는 프로젝트의 버전 관리 명령으로 변경 사항을 게시하고, 풀 요청을 열 수 없을 때 대신 게시한 내용을 보고합니다. git 저장소 외부에서 `/batch`를 실행하려면 Claude Code v2.1.281 이상이 필요합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

Claude Code는 worktree를 생성하거나, 시작 시 worktree에 진입하거나, 재개된 세션을 worktree로 반환할 때 아래의 오류를 보고합니다.

<h3 id="claude-code-can’t-enter-the-worktree-at-startup">
  Claude Code가 시작 시 worktree에 진입할 수 없음
</h3>

Claude Code가 시작 시 worktree 디렉토리에 진입할 수 없으면, 경로를 명시하는 오류를 출력하고 종료 코드 1로 종료합니다. 이는 [`WorktreeCreate` hook](/docs/ko/hooks#worktreecreate)이 생성한 디렉토리 이외의 다른 것을 출력하거나, 설정 후 디렉토리가 삭제되었을 때 발생할 수 있습니다.

<h3 id="worktree-creation-fails-on-a-symlinked-path">
  Symlink 경로에서 Worktree 생성 실패
</h3>

Claude Code는 `.claude`, `.claude/worktrees` 또는 worktree 디렉토리 자체가 symlink일 때 worktree 생성을 거부하며, 오류는 symlink된 경로를 명시합니다. symlink를 제거하고 다시 시도하십시오. v2.1.212 이전에는 저장소에 이미 이러한 경로 중 하나에 커밋된 symlink가 포함되어 있으면 worktree 생성이 이를 따라가서 저장소 외부에 파일을 생성할 수 있었습니다.

<h3 id="git-lfs-content-is-missing-from-a-worktree-claude-code-created">
  Git LFS 파일이 Claude Code가 생성한 worktree의 포인터 파일임
</h3>

`git lfs install --local`로 [Git LFS](https://git-lfs.com)를 설정한 경우, Claude Code가 생성한 worktree는 실제 파일 대신 LFS 포인터 파일을 포함합니다. `--local` 플래그는 LFS 필터를 전역 git 설정이 아닌 저장소 자체의 `.git/config`에 작성합니다. 일반적인 `git lfs install`은 전역 설정에 작성되며 영향을 받지 않습니다. 저장소 자체 설정에 정의된 다른 [filter driver](https://git-scm.com/docs/gitattributes)도 동일하게 적용됩니다.

Claude Code는 worktree를 생성할 때 저장소 자체의 필터 드라이버를 건너뜁니다. 필터 드라이버는 셸 명령이며, Claude를 포함하여 저장소에 쓸 수 있는 모든 것이 거기에 하나를 넣었을 수 있기 때문입니다. v2.1.247 이전에는 Claude Code가 worktree 생성 중에 이러한 드라이버를 실행했습니다.

실제 파일을 얻으려면 worktree 내에서 `git lfs pull`을 실행하십시오.

네 가지 드문 경우에 Claude Code는 worktree를 전혀 생성하지 않습니다. 저장소의 설정이 정의하는 필터 드라이버를 알 수 없거나, 끌 수 없는 설정을 찾을 수 있습니다. 오류를 해당 해결책과 일치시키십시오:

* **`Could not read the repository git config to neutralize filter drivers`**: Claude Code가 저장소의 `.git/config`를 읽을 수 없습니다. 예를 들어 권한 때문일 수 있습니다. 이를 수정하고 다시 시도하십시오.
* **`The repository git config defines a filter driver whose name cannot be neutralized (contains "=" or a newline)`**: `.git/config`에서 해당 필터 드라이버의 이름을 바꾸거나 제거하고 다시 시도하십시오.
* **`The repository git config has a conditional include (includeIf)`**: `.git/config`의 `includeIf`가 가져오는 설정을 해당 파일로 직접 이동하고, `includeIf`를 제거한 후 다시 시도하십시오. 전역 git 설정의 `includeIf`는 이를 트리거하지 않습니다.
* **`Git was not run: the repository's own git config sets <key>`**: 메시지는 Git LFS를 실행할 프로그램으로 가리키는 키를 명시합니다. 예를 들어 `lfs.customtransfer.<name>.path` 또는 `lfs.standalonetransferagent`입니다. 해당 설정이 귀하의 것이면 전역 git 설정으로 이동하십시오. 인식하지 못하면 저장소의 git 설정에서 제거하십시오. 신뢰하지 않는 도구나 체크아웃이 작성했을 수 있기 때문입니다. 저장소의 설정에서 키가 없어지면 다시 시도하십시오.

<h3 id="claude-code-refuses-to-use-a-worktree">
  Claude Code가 worktree 사용을 거부함
</h3>

`Refusing to use <path> as an isolation worktree`로 시작하는 오류는 Claude Code가 디렉토리의 git 정체성을 확인한 후 세션 또는 하위 에이전트의 격리된 체크아웃으로 채택하기를 거부했다는 의미입니다. 확인은 Claude Code가 worktree를 생성하든, 기존 worktree에 진입하든, 이전 실행의 worktree를 재사용하든 실행됩니다.

대부분의 경우 메시지의 나머지 부분은 디렉토리의 git 메타데이터가 주 체크아웃으로 해석된다고 말합니다. 예를 들어, 해당 `.git` 파일이 주 저장소 자체의 `.git` 디렉토리를 가리키거나, git이 `core.worktree` 리다이렉트를 통해 작업 트리를 주 체크아웃으로 해석합니다. 이러한 디렉토리에서 `git reset --hard`와 같은 일반적인 git 명령은 worktree 대신 주 체크아웃에 작용합니다. Claude Code는 또한 디렉토리에 읽을 수 없는 `.git` 항목이 있을 때 거부합니다. worktree가 안전하다고 가정하기보다는 말입니다.

git 메타데이터가 전혀 없는 디렉토리(예: [`WorktreeCreate` hook](#non-git-version-control)이 생성하는 디렉토리)는 git 저장소가 포함하지 않을 때만 확인을 통과합니다. hook이 저장소 내부에 디렉토리를 생성하면, git이 해당 저장소의 체크아웃으로 해석하고 Claude Code는 `git resolves its working tree to` 메시지로 거부합니다. hook이 저장소 외부에 디렉토리를 생성하도록 하십시오.

Claude Code는 거부된 디렉토리를 제자리에 두었습니다. 작업을 보유할 수 있기 때문입니다. 메시지를 해당 복구와 일치시키십시오. `Refusing to use <path>` 다음에 나타나든 [재개 메시지](#the-session-resumes-outside-its-worktree)에 나타나든 말입니다. 일부 끝은 재개 메시지에만 나타납니다:

* **`launch from the parent checkout` 또는 `Run the resume from the project checkout`이라고 말함**: worktree 내부에서 Claude Code를 시작했습니다. 대신 주 체크아웃에서 시작하십시오. worktree는 재생성할 필요가 없습니다.
* **`it cannot be resumed or re-entered`라고 말함**: 시작한 위치에서 이 세션의 worktree를 보증하는 것이 없습니다. 재생성하십시오. 디렉토리와 해당 작업은 수동 복구를 위해 디스크에 남아 있으며, worktree에 부모 체크아웃이 있을 때 거기에서 재개하는 것도 작동합니다.
* **`it contains the protected checkout`이라고 말함**: 거부된 디렉토리는 홈 디렉토리와 같은 주 체크아웃의 부모입니다. 삭제하지 마십시오. worktree 경로를 변경하십시오. 예를 들어 `WorktreeCreate` hook이 반환하는 경로 또는 `EnterWorktree` 대상이므로 worktree가 체크아웃을 포함하지 않습니다.
* **`the protected checkout <path> has a .git entry that could not be examined` 또는 `has git metadata that could not be resolved`라고 말함**: 문제는 worktree의 git 메타데이터가 아닌 주 체크아웃의 git 메타데이터입니다. worktree를 삭제하지 마십시오. 메시지의 후행 조언을 무시하십시오. 이 두 끝에는 적용되지 않는 재생성입니다. 주 체크아웃을 복구하십시오. 예를 들어 권한 문제 또는 해당 `.git`의 git `dubious ownership` 거부입니다. 그리고 다시 시도하십시오.
* **`its recorded path has a network spelling`이라고 말함**: Claude Code는 네트워크 경로의 worktree로 재개하지 않습니다. 로컬 경로에서 worktree를 재생성하십시오.
* **다른 끝**: 메시지는 문제와 해결책을 명시합니다. 예를 들어 `core.worktree` 리다이렉트를 제거하거나 worktree를 재생성합니다. 이를 따르십시오. git 정체성을 확인할 수 없다고 말하는 메시지의 디렉토리를 삭제하기 전에 명시된 원인을 먼저 해결하십시오. 예를 들어 worktree 경로의 기호 링크 또는 git 자체 실행 실패입니다. 디렉토리가 정상일 수 있기 때문입니다. 재생성할 때 이전 디렉토리에서 필요한 변경 사항을 먼저 구출하십시오. 디스크에 남아 있습니다.

<h3 id="the-session-resumes-outside-its-worktree">
  세션이 worktree 외부에서 재개됨
</h3>

대화형으로 세션을 재개할 때 Claude Code가 worktree로 반환할 수 없으면, Claude Code는 아래의 메시지 중 하나로 이를 말합니다. Claude Code가 worktree 바인딩을 지우면, 세션 기록에 지우기를 기록합니다. [기록 쓰기를 억제](/docs/ko/sessions#where-transcripts-are-stored)하면, 메시지는 대신 바인딩을 지울 수 없으며 Claude Code가 나중에 재개할 때 worktree를 다시 확인할 것이라고 말합니다.

| 메시지 시작                                            | 발생한 일과 수행할 작업                                                                                                                                                                                                                                                                                 |
| :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Your worktree <path> no longer exists`           | worktree 디렉토리가 제거되었습니다. 세션은 격리 없이 현재 디렉토리에서 계속되며, Claude Code는 worktree 바인딩을 지웁니다. 조치가 필요하지 않습니다.                                                                                                                                                                                             |
| `Could not verify your worktree <path> this time` | Claude Code가 worktree를 확인할 수 없습니다. 일반적으로 일시적인 이유 때문입니다. 바인딩은 유지되며, 세션은 격리 없이 현재 디렉토리에서 계속됩니다. 다시 재개하여 다시 시도하십시오. 계속 발생하면 새 세션에서 worktree에 진입하고 [Claude Code가 worktree 사용을 거부함](#claude-code-refuses-to-use-a-worktree) 아래의 거부 메시지와 일치시키십시오. 이는 worktree의 메타데이터가 아닌 주 체크아웃의 메타데이터를 명시할 수 있습니다. |
| `Did not re-enter your worktree <path>`           | Claude Code가 worktree 바인딩을 안전하지 않은 것으로 거부했습니다. 바인딩을 지우고 세션은 격리 없이 계속됩니다. 메시지는 특정 거부를 포함합니다. [Claude Code가 worktree 사용을 거부함](#claude-code-refuses-to-use-a-worktree) 아래와 일치시키십시오. 일부 거부는 재생성이 필요하고 다른 거부는 경로 변경이 필요하기 때문입니다.                                                                   |
| `Could not re-enter your worktree <path>`         | Claude Code가 시작한 위치에서 worktree를 보증할 수 없습니다. 가장 일반적으로 내부에서 시작했기 때문입니다. 바인딩은 유지됩니다. 메시지의 나머지 부분은 해결책을 명시합니다. [Claude Code가 worktree 사용을 거부함](#claude-code-refuses-to-use-a-worktree) 아래와 일치시키십시오.                                                                                               |

[비대화형 모드](/docs/ko/headless)에서 `-p`를 사용하고, [Agent SDK](/docs/ko/agent-sdk/sessions)가 실행하는 재개에서, Claude Code는 사라진 worktree를 제외한 모든 거부에 대해 stderr 오류로 재개를 중지합니다. 격리 없이 계속하는 대신입니다.

`--output-format stream-json`을 사용하면, 거부도 stdout에 `result` 메시지로 도착합니다. subtype `error_during_execution`이며 `errors` 배열은 동일한 텍스트를 전달합니다. Agent SDK 애플리케이션은 0이 아닌 종료만이 아닌 이유를 받습니다. v2.1.260 이전에는 worktree 재개 거부가 `result` 메시지를 생성하지 않았습니다.

메시지는 표의 대화형 메시지와 다른 형태를 취합니다:

* `Error: cannot resume into worktree <path>: ...This session was not started.` 표가 `Did not re-enter`로 표시하는 거부의 경우입니다. Claude Code는 종료 전에 worktree 바인딩을 지우며, 오류는 이를 말합니다. 다음에 대화를 재개할 때, 세션은 worktree 격리 없이 현재 디렉토리에서 계속됩니다. v2.1.260 이전에는 Claude Code가 지워진 바인딩을 작성하지 않았으므로 동일한 재개의 모든 재시도가 동일한 오류로 실패했습니다.

  [기록 쓰기를 억제](/docs/ko/sessions#where-transcripts-are-stored)하면, 지우기를 저장할 수 없습니다. 오류는 동일한 명령이 다시 거부될 것이라고 말합니다. `--fork-session`과 worktree 없이 계속하는 방법으로 새 대화를 시작하는 것을 명시합니다.
* `Error: could not verify worktree <path> for this resume, so the resume was aborted...` `Could not verify`의 경우입니다.
* `Error: ...The worktree binding is kept.` `Could not re-enter`의 경우입니다.
* `Notice: the worktree <path> for this session no longer exists...` 사라진 worktree의 경우입니다. Claude Code는 이를 출력하고 대화형 재개처럼 세션을 계속합니다.

각 오류에 포함된 거부 끝은 대화형 공지와 공유되므로, [Claude Code가 worktree 사용을 거부함](#claude-code-refuses-to-use-a-worktree) 아래의 항목과 여전히 일치합니다.

stream-json 결과에서, [`startup_failure_reason`](/docs/ko/agent-sdk/typescript#startup_failure_reason)은 `could not verify worktree` 오류의 경우 `worktree_unverified`이고 `cannot resume into worktree` 및 `The worktree binding is kept` 오류의 경우 `worktree_resume_refused`입니다. 애플리케이션은 오류 텍스트를 일치시키는 대신 이를 기반으로 분기할 수 있습니다. v2.1.274 이전에는 결과가 `startup_failure_reason` 필드를 전달하지 않았습니다.

<h2 id="see-also">
  참고 항목
</h2>

Worktree는 파일 격리를 처리합니다. 아래의 관련 페이지는 이러한 격리된 체크아웃으로 작업을 위임하고, 그들 간에 발견 사항을 전달하고, 생성한 세션 간에 전환하는 것을 다룹니다:

* [서브에이전트](/docs/ko/sub-agents): 세션 내의 격리된 에이전트에 작업 위임
* [크로스 세션 메시징](/docs/ko/cross-session-messaging): worktree의 세션이 서로 발견 사항을 전달하도록 허용
* [에이전트 팀](/docs/ko/agent-teams): 여러 Claude 세션을 자동으로 조정
* [세션 관리](/docs/ko/sessions): 대화 이름 지정, 재개 및 전환
* [데스크톱 병렬 세션](/docs/ko/desktop#work-in-parallel-with-sessions): 데스크톱 앱의 worktree 기반 세션
