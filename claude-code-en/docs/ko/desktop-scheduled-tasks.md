> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Desktop에서 반복 작업 예약하기

> Claude Code Desktop에서 예약된 작업을 설정하여 일일 코드 리뷰, 종속성 감사 또는 아침 브리핑을 위해 Claude를 자동으로 반복 실행합니다.

예약된 작업은 선택한 시간과 빈도에 따라 새 세션을 자동으로 시작합니다. 일일 코드 리뷰, 종속성 업데이트 확인 또는 캘린더와 받은편지함에서 정보를 가져오는 아침 브리핑과 같은 반복 작업에 사용합니다.

Desktop 앱의 **Routines** 페이지를 통해 로컬 예약된 작업과 원격 [routines](/docs/ko/routines)을 모두 만들 수 있습니다. 로컬 작업은 사용자의 머신에서 실행되며 파일과 도구에 직접 접근할 수 있지만, 앱이 열려 있고 컴퓨터가 깨어 있을 때만 실행됩니다. 원격 routine은 컴퓨터가 꺼져 있어도 Anthropic 관리 클라우드 인프라에서 실행되며, API 호출이나 GitHub 이벤트에 대해서도 실행될 수 있습니다. 이 페이지는 로컬 예약된 작업을 다룹니다. 원격 routine과 해당 트리거 옵션에 대해서는 [Routines](/docs/ko/routines)을 참조하세요.

<h2 id="compare-scheduling-options">
  예약 옵션 비교
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<Note>
  기본적으로 예약된 작업은 커밋되지 않은 변경 사항을 포함하여 작업 디렉토리의 현재 상태에 대해 실행됩니다. 작업을 만들 때 worktree 토글을 활성화하여 각 실행이 자신의 격리된 Git worktree를 갖도록 하면, [병렬 세션](/docs/ko/desktop#work-in-parallel-with-sessions)과 동일한 방식으로 작동합니다.
</Note>

<h2 id="create-a-scheduled-task">
  예약된 작업 만들기
</h2>

사이드바에서 **Routines**을 클릭한 다음 **New routine**을 클릭하고 **Local**을 선택합니다. 다음 필드를 구성합니다:

| 필드           | 설명                                                                                                                                                                       |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Name         | 작업의 식별자입니다. 소문자 kebab-case로 변환되며 디스크의 폴더 이름으로 사용됩니다. 작업 전체에서 고유해야 합니다.                                                                                                   |
| Description  | 작업 목록에 표시되는 짧은 요약입니다.                                                                                                                                                    |
| Instructions | 작업이 실행될 때 Claude가 수행해야 할 작업입니다. 프롬프트 상자의 다른 메시지를 작성하는 것과 동일한 방식으로 작성합니다. instructions 입력에는 권한 모드 및 모델에 대한 선택기가 포함되어 있으며, 아래에서 작업 폴더를 선택하고 격리된 worktree에서 실행할지 여부를 선택합니다. |
| Schedule     | 작업이 실행되는 빈도입니다. 아래의 [schedule options](#schedule-options)을 참조하세요.                                                                                                        |

작업을 저장하기 전에 폴더가 필요합니다. 아직 해당 폴더를 신뢰하지 않은 경우 Desktop은 저장하기 전에 폴더를 신뢰하도록 요청합니다.

모든 세션에서 원하는 작업을 설명하여 작업을 만들 수도 있습니다. 예를 들어, "매일 아침 9시에 실행되는 일일 코드 리뷰 설정"은 반복 작업을 만들고, "내일 오후 3시에 배포 확인을 상기시켜 줘"는 실행 후 자신을 비활성화하는 일회성 작업을 만듭니다.

<h2 id="schedule-options">
  예약 옵션
</h2>

Schedule 컨트롤에서 사전 설정을 선택합니다:

* **Manual**: 일정이 없으며, **Run now**를 클릭할 때만 실행됩니다. 요청 시 트리거하는 프롬프트를 저장하는 데 유용합니다.
* **Hourly**: 매시간 실행됩니다.
* **Daily**: 시간 선택기를 표시하며, 기본값은 현지 시간 오전 9:00입니다.
* **Weekdays**: Daily와 동일하지만 토요일과 일요일을 건너뜁니다.
* **Weekly**: 시간 선택기와 요일 선택기를 표시합니다.

15분마다, 매월 1일, 또는 특정 미래 시간의 일회성 실행과 같이 선택기가 제공하지 않는 간격의 경우, Desktop의 모든 세션에서 Claude에게 일정을 설정하도록 요청합니다. 일반 언어를 사용합니다. 예를 들어, "6시간마다 모든 테스트를 실행하는 작업을 예약해 줘"입니다.

<h2 id="how-scheduled-tasks-run">
  예약된 작업이 실행되는 방식
</h2>

예약된 작업은 사용자의 머신에서 실행됩니다. Desktop은 앱이 열려 있는 동안 매분 일정을 확인하고 열려 있는 수동 세션과 독립적으로 작업이 만료되면 새 세션을 시작합니다. 각 작업은 API 트래픽을 분산하기 위해 예약된 시간 이후 몇 분의 작은 지연을 받습니다. 지연은 결정론적입니다: 동일한 작업은 항상 동일한 오프셋에서 시작됩니다.

작업이 실행되면 데스크톱 알림을 받고 새 세션이 사이드바의 **Scheduled** 섹션 아래에 나타납니다. 이를 열어 Claude가 수행한 작업을 확인하고, 변경 사항을 검토하거나, 권한 프롬프트에 응답합니다. 세션은 다른 세션처럼 작동합니다: Claude는 파일을 편집하고, 명령을 실행하고, 커밋을 만들고, 풀 요청을 열 수 있습니다.

작업은 desktop 앱이 실행 중이고 컴퓨터가 깨어 있을 때만 실행됩니다. 컴퓨터가 예약된 시간을 통해 절전 모드로 전환되면 실행이 건너뜁니다. 유휴 절전을 방지하려면 **Desktop app → General** 아래의 Settings에서 **Keep computer awake**를 활성화합니다. 노트북 뚜껑을 닫으면 여전히 절전 모드로 전환됩니다. 컴퓨터가 꺼져 있어도 실행되어야 하거나 API 호출이나 GitHub 이벤트에 대해 트리거되어야 하는 작업의 경우, 대신 원격 [routine](/docs/ko/routines)을 만듭니다.

<h2 id="missed-runs">
  놓친 실행
</h2>

앱이 시작되거나 컴퓨터가 깨어나면 Desktop은 지난 7일 동안 각 작업이 놓친 실행이 있는지 확인합니다. 있으면 Desktop은 가장 최근에 놓친 시간에 대해 정확히 하나의 catch-up 실행을 시작하고 더 오래된 것은 버립니다. 6일을 놓친 일일 작업은 깨어날 때 한 번 실행됩니다. Desktop은 catch-up 실행이 시작될 때 알림을 표시합니다.

프롬프트를 작성할 때 이를 염두에 두세요. 오전 9시에 예약된 작업은 컴퓨터가 하루 종일 절전 모드였다면 오후 11시에 실행될 수 있습니다. 타이밍이 중요한 경우 프롬프트 자체에 보호 장치를 추가합니다. 예를 들어, "오늘의 커밋만 검토합니다. 오후 5시 이후이면 검토를 건너뛰고 놓친 내용의 요약만 게시합니다."

<h2 id="permissions-for-scheduled-tasks">
  예약된 작업에 대한 권한
</h2>

각 작업에는 자신의 권한 모드가 있으며, 이는 작업을 만들거나 편집할 때 설정합니다. `~/.claude/settings.json`의 Allow 규칙도 예약된 작업 세션에 적용됩니다. 작업이 Ask 모드에서 실행되고 권한이 없는 도구를 실행해야 하는 경우, 승인할 때까지 실행이 정지됩니다. 세션은 사이드바에 열려 있으므로 나중에 답변할 수 있습니다.

정지를 방지하려면 작업을 만든 후 **Run now**를 클릭하고 권한 프롬프트를 확인한 다음 각 프롬프트에 대해 "always allow"를 선택합니다. 해당 작업의 향후 실행은 프롬프트 없이 동일한 도구를 자동으로 승인합니다. 작업의 세부 정보 페이지에서 이러한 승인을 검토하고 취소할 수 있습니다.

Connector 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)된 도구와 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 매번 호출할 때마다 프롬프트를 표시하며 always-allow 옵션을 제공하지 않습니다. 이러한 도구를 호출하는 실행은 매번 정지됩니다.

<h2 id="manage-scheduled-tasks">
  예약된 작업 관리
</h2>

**Routines** 목록에서 작업을 클릭하여 세부 정보 페이지를 엽니다. 여기에서 다음을 수행할 수 있습니다:

* **Run now**: 다음 예약된 시간을 기다리지 않고 작업을 즉시 시작합니다.
* **Status**: Active와 Paused 사이를 전환하여 작업을 삭제하지 않고 예약된 실행을 일시 중지하거나 재개합니다.
* **Edit**: instructions, schedule, folder 또는 기타 설정을 변경합니다.
* **Review history**: 건너뛴 실행을 포함하여 모든 과거 실행을 확인합니다. 건너뛴 항목 위에 마우스를 올려 이유를 확인합니다: 컴퓨터가 절전 모드였거나, 이전 실행이 여전히 진행 중이거나, 다른 예약된 작업이 이미 실행 중입니다. **Show more**를 클릭하여 더 오래된 항목을 로드합니다.
* **Review allowed permissions**: **Always allowed** 패널에서 이 작업에 대해 저장된 도구 승인을 확인하고 취소합니다.
* **Delete**: 작업을 제거하고 생성한 모든 세션을 보관합니다. 확인 대화 상자에 **Also delete files on disk** 확인란이 나타납니다. 이를 확인하여 작업의 `SKILL.md` 파일과 `~/.claude/scheduled-tasks/`의 관련 데이터도 제거합니다.

Desktop의 모든 세션에서 Claude에게 요청하여 작업을 나열, 만들, 편집 및 일시 중지할 수도 있습니다. 예를 들어, "내 dependency-audit 작업을 일시 중지해 줘" 또는 "내 예약된 작업을 보여 줘"입니다. 작업을 삭제하려면 세부 정보 페이지의 **Delete** 버튼을 사용합니다.

예약된 작업은 실행 중인 세션 내에서 `update_scheduled_task` MCP 도구를 사용하여 자신의 일정이나 프롬프트를 수정할 수도 있습니다. 이를 통해 작업은 발견한 내용에 따라 자신을 재예약할 수 있습니다. 예를 들어, 릴리스 브랜치가 생성되었음을 감지할 때 코드 리뷰를 더 일찍 실행하도록 재예약합니다.

디스크에서 작업의 프롬프트를 편집하려면 `~/.claude/scheduled-tasks/<task-name>/SKILL.md`를 엽니다 (설정된 경우 [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars) 아래). 파일은 `name` 및 `description`에 대해 YAML frontmatter를 사용하며, 프롬프트는 본문입니다. 변경 사항은 다음 실행에 적용됩니다. Schedule, folder, model 및 enabled state는 이 파일에 없습니다: Edit 양식을 통해 변경하거나 Claude에게 요청합니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [Routines](/docs/ko/routines): Anthropic 관리 인프라에서 일정에 따라, API 호출을 통해 또는 GitHub 이벤트에 대응하여 작업을 실행합니다. 컴퓨터가 꺼져 있어도 실행됩니다.
* [Run prompts on a schedule](/docs/ko/scheduled-tasks): CLI에서 `/loop`를 사용한 세션 범위 예약
* [Claude Code GitHub Actions](/docs/ko/github-actions): 머신에서가 아닌 CI에서 일정에 따라 Claude를 실행합니다.
* [Use Claude Code Desktop](/docs/ko/desktop): 전체 Desktop 앱 가이드
