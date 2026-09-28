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

| 명령                                  | 기능                                                                          |
| :---------------------------------- | :-------------------------------------------------------------------------- |
| `claude --continue`                 | 현재 디렉토리에서 가장 최근 대화를 다시 엽니다                                                  |
| `claude --resume`                   | [세션 선택기](#use-the-session-picker)를 엽니다                                      |
| `claude --resume <name>`            | 지정된 이름의 세션을 직접 재개합니다                                                        |
| `claude --resume <transcript-path>` | 해당 절대 경로의 `.jsonl` [대화 기록 파일](#where-transcripts-are-stored)에 저장된 대화를 재개합니다 |
| `claude --from-pr <number>`         | 해당 풀 요청에 연결된 세션으로 필터링된 세션 선택기를 엽니다                                          |
| `/resume`                           | 활성 세션 내에서 다른 대화로 전환합니다                                                      |

Claude Code는 [`claude -p`](/docs/ko/headless) 또는 [Agent SDK](/docs/ko/agent-sdk/overview)로 생성된 세션을 세션 선택기 및 `claude --continue`에서 제외합니다. 세션 ID를 `claude --resume <session-id>`에 전달하여 여전히 재개할 수 있습니다. `claude --continue`를 사용하면 Claude Code는 [첫 번째 프롬프트가 `/loop`인 세션](#where-the-session-picker-looks)도 건너뜁니다. [`claude -p --continue`](/docs/ko/headless#continue-conversations)를 실행하면 Claude Code는 `-p`, SDK 및 `/loop` 세션을 포함합니다.

`claude --continue`는 완료된 [백그라운드 세션](/docs/ko/agent-view)을 열지만 여전히 실행 중인 세션은 열지 않습니다. 완료된 백그라운드 세션을 열려면 Claude Code v2.1.257 이상이 필요합니다. 가장 최근 대화가 [백그라운드로 이동](/docs/ko/agent-view#send-the-session-to-the-background)한 세션이고 여전히 그곳에서 실행 중인 경우 Claude Code는 `Your most recent conversation is running in the background`와 해당 세션의 ID로 종료됩니다. [`claude agents`](/docs/ko/agent-view#attach-to-a-session)에서 세션에 연결하거나 `claude --resume`을 실행하여 다른 세션을 선택합니다.

모든 디렉토리에서 `claude --resume <session-id>`를 실행할 수 있습니다. Claude Code는 현재 프로젝트 디렉토리 및 해당 git worktree에서 ID를 먼저 찾은 다음 이 머신의 다른 모든 프로젝트에서 찾으므로 다른 곳에서 시작되었거나 [`/cd`](/docs/ko/commands)로 이동한 세션을 찾습니다. 교차 프로젝트 검색은 정확히 하나의 다른 프로젝트가 해당 ID에 대한 메시지가 있는 대화 기록을 보유할 때만 ID를 확인하므로 손으로 복사한 중복은 Claude Code가 임의의 복사본을 재개하지 않고 찾을 수 없음을 보고하게 합니다. 저장된 세션이 ID와 일치하지 않으면 Claude Code는 `No conversation found with session ID: <session-id>`를 보고합니다. v2.1.223 이전에는 조회가 현재 프로젝트 디렉토리 및 해당 git worktree에서 중지되었으므로 세션이 마지막으로 작업한 디렉토리에서 재개해야 했습니다.

<h3 id="what-a-resumed-session-restores">
  재개된 세션이 복원하는 것
</h3>

재개된 세션은 대화와 함께 저장된 상태를 복원합니다:

* 대화 기록: 도구 호출 및 결과를 포함한 전체 기록입니다. 이전 프로세스가 종료될 때(예: 충돌) 여전히 실행 중이던 도구는 재개할 때 완료되거나 다시 실행되지 않습니다. Claude는 호출이 결과가 기록되기 전에 중단된 것으로 표시되고 다시 실행하기 전에 효과가 있었는지 확인하도록 지시받으며, [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](/docs/ko/env-vars#variables)이 설정되지 않은 경우입니다. v2.1.281 이전에는 Claude Code가 중단된 호출을 대화에서 삭제하거나 사용자가 중단한 것으로 Claude에게 표시했습니다.
* 모델: 세션은 사용 중이던 모델에서 계속됩니다. 모델이 폐기되었거나 `availableModels`에서 허용되지 않을 때, 시작 시 `--model` 플래그 또는 `ANTHROPIC_MODEL` 계열 환경 변수가 모델을 선택할 때, 또는 [Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry](/docs/ko/third-party-integrations)와 같이 공급자별 배포 ID를 사용하는 공급자에서는 모델이 복원되지 않습니다. [모델 구성](/docs/ko/model-config#setting-your-model)에서 해결 순서를 참조하세요.
* 에이전트: [`--agent`](/docs/ko/sub-agents#invoke-subagents-explicitly) 또는 `agent` 설정으로 시작된 세션은 해당 에이전트로 계속되며 도구 제한 및 모델을 유지합니다. 재개할 때 `--agent`를 전달하여 다른 에이전트를 선택합니다. 두 경우 모두 시스템 프롬프트는 [재개된 대화의 시스템 프롬프트 플래그](/docs/ko/cli-reference#system-prompt-flags-in-resumed-conversations)를 참조하세요. Claude Code는 두 위치에서 에이전트를 찾습니다: 세션의 원본 디렉토리(해당 워크스페이스를 [신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)한 경우) 및 재개하는 디렉토리이므로 프로젝트 범위 에이전트는 다른 디렉토리에서 재개할 때도 로드됩니다. Claude Code가 두 위치 모두에서 에이전트를 찾지 못하면 세션은 기본 도구로 재개되고 [에이전트 이름을 지정하는 경고](/docs/ko/errors#session-agent-no-longer-available)를 표시합니다.
* 권한 모드: 터미널에서 `claude --continue`, `claude --resume <session-id>` 또는 이름이 한 세션과 일치할 때 `-p` 없이 `claude --resume <name>`으로 재개하면 Claude Code는 세션이 있던 권한 모드를 복원합니다. 단, [재개 시 권한 모드](#permission-mode-on-resume)의 경우는 제외되며, 이는 세션 선택기, `/resume` 및 `claude -p`로 재개하는 경우도 포함합니다. `--permission-mode` 또는 `--dangerously-skip-permissions`를 전달하여 복원된 모드를 재정의합니다.
* 활성 목표: 세션이 종료될 때 여전히 활성이던 [목표](/docs/ko/goal#resume-with-an-active-goal)는 이월됩니다. 해당 턴 수, 타이머 및 토큰 지출 기준선이 재설정됩니다.
* 예약된 작업: [만료되지 않은 작업](/docs/ko/scheduled-tasks#limitations)이 복원됩니다. 백그라운드 Bash 및 모니터 작업은 복원되지 않습니다.

원본 시작의 모든 구성 플래그가 복원되는 것은 아닙니다. 세션이 `--mcp-config`, `--settings`, `--plugin-dir`, `--fallback-model` 또는 `--add-dir`로 추가된 디렉토리에 의존하는 경우 재개할 때 다시 전달합니다. 세션 중간에 `/add-dir`로 추가된 디렉토리도 복원되지 않지만 세션 선택기는 여전히 이를 사용하여 세션을 찾습니다. `settings.json` 및 `settings.local.json`과 같은 표준 설정 파일은 시작 시 다시 읽히므로 이들 파일에 있는 구성은 다시 전달할 필요가 없습니다. `--system-prompt` 및 `--append-system-prompt`는 [재개된 대화의 시스템 프롬프트 플래그](/docs/ko/cli-reference#system-prompt-flags-in-resumed-conversations)를 참조하세요.

<h4 id="permission-mode-on-resume">
  재개 시 권한 모드
</h4>

Claude Code가 재개된 세션을 시작하는 권한 모드는 재개 방식에 따라 달라집니다:

* 터미널: `claude --continue`, `claude --resume <session-id>` 또는 이름이 한 세션과 일치할 때 `-p` 없이 `claude --resume <name>`. Claude Code는 세션이 있던 권한 모드를 복원합니다. 단, 표의 경우는 제외됩니다. `--permission-mode` 또는 `--dangerously-skip-permissions`를 전달하여 복원된 모드를 재정의합니다.
* 비대화형: `claude -p --resume` 또는 `claude -p --continue`. Claude Code는 새로운 `claude -p` 실행이 시작될 권한 모드로 실행을 시작합니다. 단, 계획 모드에서 종료된 세션은 [아래 조건](#resume-in-plan-mode-with-p)에서 계획 모드로 재개됩니다.
* VS Code: 확장의 대화 패널입니다. 표는 계획 모드에서 종료된 대화만 다룹니다. 나머지는 [과거 대화 재개](/docs/ko/vs-code#resume-past-conversations)를 참조하세요.
* 시작 시 세션 선택기: `claude --resume` 단독, `claude --from-pr` 또는 이름이 여러 세션과 일치할 때 [세션 선택기](#use-the-session-picker)에서 선택한 세션입니다. Claude Code는 저장된 권한 모드를 복원하지 않습니다. 동일한 명령줄에서 새 세션을 시작할 권한 모드로 세션을 시작합니다.
* 세션 내 `/resume`(인수 있음 또는 없음): Claude Code는 저장된 권한 모드를 복원하지 않습니다. 전환하는 대화는 현재 세션이 있는 권한 모드에서 계속됩니다.

비대화형 및 VS Code 경로에서 계획 모드 복원은 Claude Code v2.1.246 이상이 필요합니다. 각 행은 세션이 종료된 권한 모드, 재개하는 터미널, 비대화형 및 VS Code 경로 중 어느 것인지, 그리고 Claude Code가 재개된 세션을 시작하는 권한 모드를 나타냅니다.

| 세션이 종료된 모드          | 재개 방식                                        | 재개 후 권한 모드                                                                                                                                                                                                                                                    |
| :------------------ | :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `bypassPermissions` | 터미널                                          | 새 세션이 시작될 권한 모드입니다. [권한을 다시 우회](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode)하려면 시작 시 해당 플래그 중 하나 또는 [사용자, `--settings` 또는 관리 설정](/docs/ko/settings-reference#permissions-defaultmode)의 `permissions.defaultMode: "bypassPermissions"`로 활성화합니다 |
| `plan`              | 터미널                                          | 새 세션이 시작될 권한 모드입니다                                                                                                                                                                                                                                            |
| `auto`              | 터미널                                          | `auto`(계정이 여전히 [자동 모드 요구 사항](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)을 충족하는 경우에만)                                                                                                                                                               |
| Manual              | 터미널                                          | 새 세션이 [기본 제공 기본값](/docs/ko/permission-modes#which-mode-a-session-starts-in)에서 자동 모드로 시작될 때 수동입니다. 설정 파일의 `defaultMode`가 [적용](/docs/ko/permission-modes#which-mode-a-session-starts-in)되면 Claude Code는 재개된 세션을 해당 모드로 시작합니다                                              |
| `plan`              | 비대화형([아래 조건](#resume-in-plan-mode-with-p)에서) | 계획 모드                                                                                                                                                                                                                                                         |
| 모든 모드               | 비대화형(다른 모든 경우)                               | 새로운 `claude -p` 실행이 시작될 권한 모드입니다                                                                                                                                                                                                                              |
| `plan`              | VS Code                                      | 계획 모드([VS Code 페이지의 예외](/docs/ko/vs-code#resume-past-conversations) 포함)                                                                                                                                                                                            |

<h5 id="resume-in-plan-mode-with-p">
  `-p`로 계획 모드에서 재개
</h5>

`claude -p --resume` 또는 `claude -p --continue` 실행은 네 가지 조건이 모두 충족될 때만 계획 모드에서 재개됩니다:

* [`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags)을 전달하여 Claude Code가 승인을 위해 계획을 제시할 수 있습니다
* `--permission-mode` 또는 `--dangerously-skip-permissions`를 전달하지 않습니다
* `--fork-session`을 전달하지 않습니다
* 실행이 [채널](/docs/ko/channels)을 통해 시작되지 않습니다

<h3 id="resume-from-a-summary">
  요약에서 재개
</h3>

Pro 또는 Max 플랜에서 약 1시간 이상 비활성 상태이고 100,000토큰을 초과하는 세션을 재개하면 Claude Code는 대화를 복원한 다음 첫 번째 메시지를 보내기 전에 대화 상자를 엽니다. 세션의 [프롬프트 캐시](/docs/ko/prompt-caching#cache-lifetime)는 그때쯤 만료되므로 다음 요청은 대화 상자의 옵션 중 어느 것을 선택하든 전체 기록을 한 번 처리합니다.

대화 상자는 세션을 계속하는 세 가지 방법을 제공합니다. 각 방법은 대화의 얼마나 많은 부분을 이후 요청으로 전달하는지에 따라 다르며, 이는 모든 세부 사항을 유지하고 요청당 더 적은 토큰을 보내는 것 사이의 트레이드오프입니다:

* **요약에서 재개**: [`/compact`](/docs/ko/context-window#what-survives-compaction)를 즉시 실행합니다. Claude Code는 전체 기록에 대해 하나의 요약 요청을 보낸 다음 기록을 요약, 가장 최근 교환 및 최대 5개의 최근에 읽은 파일로 바꿉니다. 이후 요청은 전체 기록 대신 요약을 전달합니다.
* **전체 세션을 그대로 재개**: 대화를 변경하지 않고 로드합니다. 첫 번째 메시지를 보낸 후 Claude Code는 전체 기록을 다시 처리하고 다시 캐시한 다음 캐시가 따뜻한 상태로 유지되는 동안 이후 요청에서 캐시에서 다시 읽습니다.
* **다시 묻지 않기**: 전체 세션을 재개하고 모든 향후 재개에서 대화 상자 표시를 중지합니다.

그대로 재개하면 대화의 모든 세부 사항을 사용할 수 있으며, 요청당 비용은 대화의 크기에 따라 확장됩니다. 요약에서 재개하면 요약이 전체 기록 대신 전달되므로 이후 각 요청에서 비용이 적게 들지만 요약이 생략한 것은 더 이상 Claude의 컨텍스트에 없습니다. [긴 세션에서 사용량이 증가하는 이유](/docs/ko/costs#why-usage-climbs-in-a-long-session)를 참조하여 요청당 비용이 어디서 나오는지 확인하세요.

<h3 id="where-the-session-picker-looks">
  세션 선택기가 찾는 위치
</h3>

Claude Code는 세션을 프로젝트 디렉토리별로 저장합니다. 기본적으로 세션 선택기는 다음을 표시합니다:

* 현재 worktree의 세션([백그라운드 세션](/docs/ko/agent-view) 포함, 목록에서 `bg`로 표시됨)
* `/add-dir`로 현재 디렉토리를 추가한 다른 곳에서 시작된 세션

`Ctrl+W`를 사용하여 저장소의 모든 worktree로 확장하거나 `Ctrl+A`를 사용하여 이 머신의 모든 프로젝트로 확장합니다.

첫 번째 프롬프트가 [`/loop`](/docs/ko/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) 명령인 세션은 선택기에 나타나지 않으며 `claude --continue`도 이를 건너뜁니다. 대화 후반에 `/loop`를 실행해도 세션이 숨겨지지 않습니다. v2.1.211 이전에는 대화 초반에 `/loop` 실행이 선택기에서 세션을 영구적으로 숨겼습니다.

[`/cd`](/docs/ko/commands)로 세션을 이동하면 새 디렉토리의 프로젝트 저장소로 재배치되므로 이후 해당 디렉토리의 선택기에 나타납니다. v2.1.196부터 이동된 세션은 충돌이나 강제 종료 후에도 이전 디렉토리의 선택기에서 제외된 상태로 유지됩니다. 이전 버전에서는 언더스코어와 같은 특수 문자가 포함된 이전 경로가 있을 때 깔끔하지 않은 종료 후 이전 디렉토리의 목록에 다시 나타날 수 있습니다.

같은 저장소의 다른 worktree에서 세션을 선택하면 그 위치에서 재개됩니다. 세션의 자체 worktree가 더 이상 존재하지 않으면 Claude Code는 [현재 디렉토리에서 재개](/docs/ko/worktrees#resume-a-worktree-session)합니다. 관련 없는 프로젝트에서 세션을 선택하면 Claude Code는 `cd` 및 재개 명령을 클립보드에 복사합니다. 해당 프로젝트의 디렉토리가 더 이상 존재하지 않으면 Claude Code는 실패할 `cd` 명령을 복사하는 대신 현재 디렉토리에서 세션을 재개합니다.

이름으로 재개하면 현재 저장소 및 해당 worktree 전체에서 확인됩니다. 두 형식 모두 정확한 일치를 찾고 다른 worktree에 있더라도 직접 재개합니다:

| 명령                       | 정확한 일치 | 모호한 이름                                        |
| :----------------------- | :----- | :-------------------------------------------- |
| `claude --resume <name>` | 직접 재개  | 이름을 검색어로 미리 채운 세션 선택기를 엽니다                    |
| `/resume <name>`         | 직접 재개  | 오류를 보고합니다. 세션 선택기를 열려면 인수 없이 `/resume`를 실행합니다 |

<h2 id="name-your-sessions">
  세션 이름 지정
</h2>

세션에 설명적인 이름을 지정하여 세션 선택기에서 찾을 수 있고 이름으로 재개할 수 있도록 합니다. 이는 여러 작업을 병렬로 진행할 때 가장 중요합니다.

| 시기                      | 이름을 설정하는 방법                                                                                                                             |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| 시작 시                    | `claude -n auth-refactor`                                                                                                               |
| 세션 중                    | `/rename auth-refactor`. 이름은 프롬프트 표시줄에도 나타납니다                                                                                           |
| 세션 선택기에서                | 세션을 강조 표시하고 `Ctrl+R`을 누릅니다                                                                                                              |
| 계획 수락 시                 | [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에서 계획을 수락하면 이미 설정하지 않은 경우 계획 내용에 따라 세션에 생성된 제목을 부여합니다               |
| claude.ai 또는 Claude 앱에서 | [원격 제어 세션](/docs/ko/remote-control#connect-from-another-device)의 이름을 변경합니다. Claude Code는 CLI에서 동일한 이름을 적용합니다. Claude Code v2.1.221 이상이 필요합니다 |
| 데스크톱 앱에서                | [데스크톱 앱](/docs/ko/desktop#work-in-parallel-with-sessions)에서 세션의 이름을 변경합니다                                                                    |

CLI 경로를 통해 또는 claude.ai에서 세션의 이름을 지정한 후 `claude --resume <name>` 또는 `/resume <name>`으로 돌아갑니다. 데스크톱 앱 세션은 앱에서 재개되며, 앱은 자체 세션 기록을 유지합니다. worktree 전체에서 이름 확인이 어떻게 작동하는지는 [세션 재개](#resume-a-session)를 참조합니다.

이 머신의 다른 활성 세션이 이미 사용 중인 이름으로 대화형 세션을 시작하거나 재개하거나, 세션의 이름을 그러한 이름으로 변경하면, Claude Code는 이미 해당 이름을 가진 세션에 이름을 남기고, 사용자의 세션 이름을 `auth-refactor-graceful-unicorn`과 같은 두 단어 접미사가 있는 변형으로 변경하고 알려줍니다. 직접 선택하려면 새 이름으로 `/rename`을 실행합니다. v2.1.232 이전에는 두 세션 모두 이름을 유지했습니다.

Claude Code가 중복 이름을 변경하지 않는 세 가지 경우가 있으므로 목록에서 동일한 이름을 가진 두 세션을 볼 수 있습니다:

* AI 생성 제목이나 기본 표시 이름을 확인하지 않습니다.
* 시작 시 [백그라운드](/docs/ko/agent-view#from-your-shell) 또는 `-p` 세션의 `--name`을 확인하지 않습니다.
* 이전 버전의 Claude Code에서 세션의 이름을 변경할 수 없습니다.

이름을 지정하지 않은 세션도 Claude Code가 할당하는 두 가지 레이블을 받습니다. 생성된 제목만 재개 핸들로 작동합니다:

* 기본 표시 이름: 이름을 지정하지 않은 대화형 세션도 시작할 때 기본 표시 이름을 받습니다. Claude Code v2.1.196 이상이 필요합니다. 기본값은 작업 디렉토리의 이름과 두 문자 접미사를 결합합니다(예: `my-app-3f`). 이는 [에이전트 보기](/docs/ko/agent-view) 및 `claude agents --json` 출력과 같은 실행 중인 세션의 목록에서 세션을 식별합니다. 기본값은 재개 핸들이 아닙니다. `claude --resume` 또는 `/resume`에 전달하면 Claude Code는 세션을 찾지 못합니다. 세션의 이름을 지정하면 해당 목록에서 기본값이 바뀌고, 계획을 수락해도 마찬가지입니다.
* 생성된 제목: 세션의 이름을 지정하지 않으면 Claude Code는 세션 제목을 생성합니다. 제목은 첫 번째 프롬프트의 짧은 요약으로, 일반적으로 Haiku 클래스 모델인 소형/빠른 모델에 대한 백그라운드 요청으로 작성됩니다. 계획을 수락하면 계획 기반 제목으로 바뀝니다. 세션의 이름을 지정하면 생성된 제목이 바뀝니다. 첫 번째 프롬프트 제목은 [세션 선택기](#use-the-session-picker)와 이름이 설정되지 않았을 때 상태 표시줄 [`session_name`](/docs/ko/statusline) 필드에 표시됩니다. 계획 제목은 동일한 두 위치에 표시되며 기본 표시 이름을 대신하는 실행 중인 세션의 목록에도 표시됩니다. 두 제목 중 하나를 `claude --resume` 또는 `/resume`에 전달할 수 있으며, Claude Code는 설정한 이름과 동일한 방식으로 확인합니다.

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

각 행은 설정된 경우 세션 이름을 표시하고, 그렇지 않으면 AI 생성 세션 제목, 대화 요약 또는 첫 번째 프롬프트와 마지막 활동 이후 경과 시간, git 분기 및 파일 크기를 표시합니다. `Ctrl+A`로 모든 프로젝트로 확장하면 각 세션의 프로젝트 경로도 표시됩니다.

`/branch` 또는 `--fork-session`으로 생성된 세션은 자신의 세션 ID를 가지며 별도의 행으로 나타납니다. 선택기가 동일한 세션에 대해 둘 이상의 항목을 찾으면 단일 행 아래에 그룹화됩니다. `→`를 눌러 그룹을 확장합니다.

Claude Code가 `claude --resume` 선택기에서 선택한 세션을 로드할 수 없으면 [`Failed to resume the conversation`](/docs/ko/errors#failed-to-resume-the-conversation) 메시지를 인쇄하고 재시도 명령을 표시한 후 코드 1로 종료됩니다. 세션 내의 `/resume` 선택기에서 Claude Code는 실패를 보고하고 현재 대화는 계속 실행됩니다.

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

`/branch` 확인은 두 개의 세션 ID를 인쇄합니다: 현재 있는 새 분기와 원본입니다. 원본은 디스크에서 변경되지 않으며 세션 선택기에서 사용 가능하게 유지됩니다. `/resume <original-name>`을 사용하거나 해당 ID를 `/resume`에 전달하여 원본으로 돌아갑니다.

`/branch`는 대화 기록을 복사하고 실행 중인 Claude Code 프로세스를 이로 전환합니다. 이 구분은 분기가 상속하는 항목을 결정합니다:

| 상태                                                                                                                                              | `/branch` 이후                                                                                                    |
| :---------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| 대화 기록                                                                                                                                           | `/branch`를 실행한 시점까지 분기로 복사됨                                                                                     |
| "이 세션에 대해 허용" 권한 부여                                                                                                                             | 이월됨; 분기는 같은 프로세스에서 실행되므로 기존 부여가 계속 적용됩니다. `--fork-session`으로 별도 프로세스로 포크하면 새 프로세스는 이를 시작하지 않으며 해당 위치에서 다시 승인합니다 |
| 진행 중인 [백그라운드 서브에이전트](/docs/ko/sub-agents#run-subagents-in-foreground-or-background) 및 [백그라운드 Bash 명령](/docs/ko/interactive-mode#background-bash-commands) | 계속 실행됩니다. 이들의 출력은 원본 세션이 아닌 전환한 새 분기에 나타납니다                                                                     |
| [Remote Control](/docs/ko/remote-control) 연결                                                                                                         | 연결 유지됩니다. 세션에 연결된 휴대폰 또는 브라우저는 사용자와 함께 분기로 이동하고 해당 위치에서 새 메시지를 계속 수신합니다                                         |

포크하지 않고 두 터미널에서 같은 세션을 재개하면 두 터미널의 메시지가 하나의 대화 기록으로 인터리브됩니다. 단일 세션 내에서 체크포인트 기반 되감기는 [체크포인팅](/docs/ko/checkpointing)을 참조합니다.

<h2 id="manage-context-within-a-session">
  세션 내 컨텍스트 관리
</h2>

이 명령은 세션을 떠나지 않고 컨텍스트 윈도우에 있는 내용을 제어합니다:

* **`/clear`**: 빈 컨텍스트로 새로 시작합니다. Claude Code는 이전 대화를 저장하며, `/resume`으로 재개하거나, 동일한 Claude Code 프로세스에서 [되감기 메뉴의 이전 세션 항목](/docs/ko/checkpointing#rewind-past-a-cleared-conversation)에서 재개할 수 있습니다. 인수가 없으면 새 대화는 `--name` 또는 `/rename`으로 설정한 이름을 유지하지만 AI가 생성한 세션 제목은 유지하지 않습니다. 대신 떠나는 대화의 이름을 지정하려면 `/clear release-prep`처럼 이름을 전달하면 새 대화가 이름 없이 시작됩니다
* **`/compact [instructions]`**: 기록을 요약으로 바꾸고, 선택적으로 지정한 내용에 초점을 맞춥니다
* **`/context`**: 현재 컨텍스트를 소비하는 것을 표시합니다

압축이 CLAUDE.md, 기술, 규칙과 상호 작용하는 방식은 [컨텍스트 윈도우 가이드](/docs/ko/context-window)를 참조합니다. 언제 지우기 대 압축을 사용할지에 대한 전략은 [모범 사례](/docs/ko/best-practices#manage-your-session)를 참조합니다.

<h2 id="export-and-locate-session-data">
  세션 데이터 내보내기 및 찾기
</h2>

`/export`를 실행하여 현재 대화를 클립보드에 복사하거나 일반 텍스트 파일로 저장합니다. 메시지와 도구 출력은 읽을 수 있는 텍스트로 렌더링됩니다. 파일 이름을 전달하여 메뉴를 건너뛰고 해당 파일에 직접 작성합니다.

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

기본적으로 Claude Code는 기록을 `~/.claude/projects/<project>/<session-id>.jsonl`에 JSONL로 저장합니다. 여기서 `<project>`는 작업 디렉토리 경로에서 파생되고 영숫자가 아닌 문자는 `-`로 대체됩니다. 변환된 이름이 200자를 초과하는 작업 디렉토리의 경우 Claude Code는 이름을 200자로 자르고 전체 경로의 해시를 추가하므로 디렉토리 이름이 파일 시스템 제한 내에 유지됩니다.

각 줄은 메시지, 도구 사용 또는 메타데이터 항목에 대한 JSON 객체입니다. 항목 형식은 Claude Code 내부 형식이며 버전 간에 변경되므로 이러한 파일을 직접 파싱하는 스크립트는 모든 릴리스에서 손상될 수 있습니다. 세션 데이터를 기반으로 구축하려면 `/export` 또는 [스크립트 인터페이스](#access-conversations-from-scripts) 대신 사용합니다.

위치, 보존 및 쓰기 동작은 구성 가능합니다:

| 대상                                                                                       | 설정                                                                                          | 위치                            |
| ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ----------------------------- |
| `~/.claude` 외부로 저장소 이동                                                                   | [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)                                                         | 환경 변수                         |
| [`<project>` 디렉토리 이름 직접 지정](#name-the-project-directory-yourself)                        | [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ko/env-vars)                                              | 환경 변수                         |
| 30일 보존 기간 변경                                                                             | [`cleanupPeriodDays`](/docs/ko/settings-reference#cleanupperioddays)                             | `settings.json`               |
| [Claude Desktop 및 Cowork 기록](/docs/ko/claude-directory#cleaned-up-automatically)에 대한 나이 제한 설정 | [`desktopSessionCleanupPeriodDays`](/docs/ko/settings-reference#desktopsessioncleanupperioddays) | 사용자 설정, 관리 설정 또는 `--settings` |
| 모든 모드에서 기록 쓰기 억제                                                                         | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/ko/env-vars)                                           | 환경 변수                         |
| 한 번의 비대화형 실행에 대해 쓰기 억제                                                                   | [`--no-session-persistence`](/docs/ko/cli-reference)                                             | `claude -p`와 함께 CLI 플래그       |

<h3 id="delete-session-data">
  세션 데이터 삭제
</h3>

기록은 [보존 정리 규칙](/docs/ko/claude-directory#cleaned-up-automatically)에 따라 만료됩니다. 프로젝트의 기록 및 관련 상태를 더 빨리 삭제하려면 [`claude project purge`](/docs/ko/claude-directory#clear-local-data)를 실행합니다. [`claude rm <id>`](/docs/ko/agent-view#what-deleting-a-session-removes)로 [백그라운드 세션](/docs/ko/agent-view)을 삭제하면 해당 기록은 디스크에 남아 있으며 `claude --resume`을 통해 계속 사용할 수 있습니다.

<h3 id="name-the-project-directory-yourself">
  프로젝트 디렉토리 이름 직접 지정
</h3>

기본적으로 Claude Code는 전체 작업 디렉토리 경로에서 `<project>` 이름을 파생합니다. 이름을 직접 선택하려면 `CLAUDE_CONFIG_DIR`과 함께 `CLAUDE_CODE_PROJECT_DIR_NAME`을 설정합니다. Claude Code는 해당 세션의 기록 및 [자동 메모리](/docs/ko/memory#auto-memory)를 사용자 이름 아래에 저장합니다. 이는 Claude Code를 포함하고 각 세션에 자체 구성 디렉토리를 제공하는 호스트에 적합합니다. Claude Code v2.1.234 이상이 필요합니다.

예를 들어 이 시작은 테넌트 A의 데이터를 `/srv/tenant-a` 아래에 유지하고 프로젝트 디렉토리 이름을 `work`로 지정합니다:

```bash theme={null}
CLAUDE_CONFIG_DIR=/srv/tenant-a CLAUDE_CODE_PROJECT_DIR_NAME=work claude
```

Claude Code는 세션의 기록을 `/srv/tenant-a/projects/work/`에 쓰고 자동 메모리를 `/srv/tenant-a/projects/work/memory/`에 쓰며, 작업 디렉토리가 무엇이든 상관없습니다.

설정할 때 세 가지 규칙이 적용됩니다:

* **`CLAUDE_CONFIG_DIR`도 설정**: 이름이 작업 디렉토리에 따라 변하지 않으므로 기본 `~/.claude` 아래에서는 모든 프로젝트의 기록 및 자동 메모리를 하나의 디렉토리로 병합합니다. Claude Code는 `CLAUDE_CONFIG_DIR`이 설정되지 않으면 `CLAUDE_CODE_PROJECT_DIR_NAME`을 무시합니다.
* **1-64자의 문자, 숫자, 하이픈 또는 밑줄 사용**: `con`과 같은 Windows 장치 이름을 사용하지 마십시오. Claude Code는 다른 값을 무시하고 파생된 이름을 사용합니다.
* **`claude`를 시작하는 셸 환경에서 설정**: Claude Code는 시작 시 해당 환경에서 한 번만 읽으므로 설정 파일의 `env` 블록은 설정할 수 없습니다.

구성 디렉토리의 프로젝트 디렉토리 이름을 지정한 후에는 해당 이름으로 계속 시작합니다. 동일한 `CLAUDE_CONFIG_DIR`으로 Claude Code를 시작하지만 `CLAUDE_CODE_PROJECT_DIR_NAME` 없이 시작하면 파생된 디렉토리를 다시 읽고 씁니다. 사용자 이름 아래에 저장된 세션은 디스크에 남아 있습니다. [세션 선택기](#use-the-session-picker)에서 `Ctrl+A`를 누르면 해당 구성 디렉토리 아래의 모든 프로젝트 디렉토리(고정된 디렉토리 포함)의 세션을 나열하고, 어떤 방식으로 시작하든 [`claude --resume <session-id>`](#resume-a-session)는 두 이름 중 하나로 저장된 세션을 찾습니다.

<h2 id="see-also">
  참고 항목
</h2>

이 페이지들은 관련 세션 및 병렬 처리 메커니즘을 다룹니다:

* [Worktrees](/docs/ko/worktrees): 별도 분기에서 격리된 병렬 세션 실행
* [Checkpointing](/docs/ko/checkpointing): 코드 및 대화를 이전 지점으로 되감기
* [Context window](/docs/ko/context-window): 컨텍스트를 채우는 것과 압축 후 유지되는 것
* [Non-interactive mode](/docs/ko/headless): `claude -p` 아래의 세션 동작
