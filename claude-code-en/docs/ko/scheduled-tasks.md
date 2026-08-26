> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 일정에 따라 프롬프트 실행하기

> /loop와 cron 스케줄링 도구를 사용하여 Claude Code 세션 내에서 프롬프트를 반복 실행하거나, 상태를 폴링하거나, 일회성 알림을 설정합니다.

스케줄된 작업을 사용하면 Claude가 일정한 간격으로 프롬프트를 자동으로 다시 실행할 수 있습니다. 배포를 폴링하거나, PR을 감시하거나, 오래 실행되는 빌드를 확인하거나, 나중에 세션에서 무언가를 하도록 자신에게 알림을 설정하는 데 사용합니다. 이벤트가 발생할 때 폴링하는 대신 반응하려면 [Channels](/docs/ko/channels)를 참조하세요. CI가 실패를 세션에 직접 푸시할 수 있습니다. 세션이 조건이 충족될 때까지 한 번에 한 번씩 계속 작동하도록 하려면 일정한 간격이 아닌 경우 [`/goal`](/docs/ko/goal)을 참조하세요.

작업은 세션 범위입니다. 현재 대화에 존재하며 새로운 대화를 시작할 때 중지됩니다. `--resume` 또는 `--continue`로 재개하면 [만료](#seven-day-expiry)되지 않은 모든 작업이 복원됩니다. 지난 7일 이내에 생성된 반복 작업이거나 스케줄된 시간이 아직 지나지 않은 일회성 작업입니다. 세션과 독립적으로 지속되는 스케줄링의 경우 [Routines](/docs/ko/routines)을 사용하여 Anthropic 관리 인프라에서 루틴을 생성하거나, [Desktop scheduled tasks](/docs/ko/desktop-scheduled-tasks)를 설정하거나, [GitHub Actions](/docs/ko/github-actions)를 사용하세요.

<h2 id="compare-scheduling-options">
  스케줄링 옵션 비교하기
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

<h2 id="run-a-prompt-repeatedly-with-/loop">
  /loop로 반복 프롬프트 실행하기
</h2>

`/loop` [번들 스킬](/docs/ko/commands)은 세션이 열려 있는 동안 프롬프트를 반복 실행하는 가장 빠른 방법입니다. 간격과 프롬프트는 모두 선택 사항이며, 제공하는 내용에 따라 루프의 동작이 결정됩니다.

| 제공하는 내용        | 예시                          | 동작                                                                                    |
| :------------- | :-------------------------- | :------------------------------------------------------------------------------------ |
| 간격과 프롬프트       | `/loop 5m check the deploy` | 프롬프트가 [고정 스케줄](#run-on-a-fixed-interval)에서 실행됩니다                                      |
| 프롬프트만          | `/loop check the deploy`    | 프롬프트가 각 반복에서 [Claude가 선택한 간격](#let-claude-choose-the-interval)으로 실행됩니다                |
| 간격만 또는 아무것도 없음 | `/loop`                     | [내장 유지보수 프롬프트](#run-the-built-in-maintenance-prompt)가 실행되거나, 존재하는 경우 `loop.md`가 실행됩니다 |

또한 스킬을 프롬프트로 전달할 수 있습니다. 예를 들어 `/loop 20m /review-pr 1234`는 각 반복에서 해당 스킬을 다시 실행합니다. v2.1.196부터 스케줄된 실행은 Claude가 [자체적으로 호출할 수 있도록 허용된](/docs/ko/skills#control-who-invokes-a-skill) 스킬만 실행합니다. 다음은 Claude에 일반 텍스트로 전달되며 실행되지 않습니다.

* `/permissions`, `/model`, `/clear`와 같은 내장 명령어
* [`disable-model-invocation: true`](/docs/ko/skills#frontmatter-reference)로 표시된 스킬
* [`skillOverrides`](/docs/ko/skills#override-skill-visibility-from-settings) 설정이나 `Skill` [거부 규칙](/docs/ko/skills#restrict-claude’s-skill-access)으로 Claude에서 제외된 스킬
* [MCP 프롬프트](/docs/ko/mcp#use-mcp-prompts-as-commands) (예: `/mcp__github__list_prs`); MCP 서버가 노출하는 스킬은 여전히 실행됩니다

<h3 id="run-on-a-fixed-interval">
  고정 간격으로 실행하기
</h3>

간격을 제공하면 Claude가 이를 cron 표현식으로 변환하고, 작업을 스케줄하고, 주기와 작업 ID를 확인합니다.

```text theme={null}
/loop 5m check if the deployment finished and tell me what happened
```

간격은 `30m`과 같은 선행 토큰으로 프롬프트 앞에 올 수도 있고, `every 2 hours`와 같은 절로 뒤에 올 수도 있습니다. 지원되는 단위는 초의 경우 `s`, 분의 경우 `m`, 시간의 경우 `h`, 일의 경우 `d`입니다.

cron은 1분 단위의 세분성을 가지므로 초는 가장 가까운 분으로 올림됩니다. `7m` 또는 `90m`과 같이 깔끔한 cron 단계로 매핑되지 않는 간격은 가장 가까운 간격으로 반올림되며 Claude가 선택한 것을 알려줍니다.

<h3 id="let-claude-choose-the-interval">
  Claude가 간격을 선택하도록 하기
</h3>

간격을 생략하면 Claude가 고정 cron 스케줄 대신 동적으로 간격을 선택합니다. 각 반복 후 관찰한 내용을 기반으로 1분에서 1시간 사이의 지연을 선택합니다. 빌드가 완료되거나 PR이 활성화되는 동안 짧은 대기, 보류 중인 것이 없을 때 더 긴 대기입니다. 선택된 지연과 그 이유는 각 반복이 끝날 때 출력됩니다.

아래 예시는 CI와 검토 의견을 확인하며, Claude는 PR이 조용해지면 반복 사이에 더 오래 기다립니다.

```text theme={null}
/loop check whether CI passed and address any review comments
```

동적 `/loop` 스케줄을 요청하면 Claude는 [Monitor tool](/docs/ko/tools-reference#monitor-tool)을 직접 사용할 수 있습니다. Monitor는 백그라운드 스크립트를 실행하고 각 출력 줄을 다시 스트리밍하므로 폴링을 완전히 피하고 프롬프트를 간격으로 다시 실행하는 것보다 토큰 효율적이고 반응성이 더 좋은 경우가 많습니다.

동적으로 스케줄된 루프는 다른 작업처럼 [스케줄된 작업 목록](#manage-scheduled-tasks)에 나타나므로 동일한 방식으로 나열하거나 취소할 수 있습니다. [지터 규칙](#jitter)은 적용되지 않지만 [7일 만료](#seven-day-expiry)는 적용됩니다. 루프는 시작 후 7일 후 자동으로 종료됩니다.

<Note>
  Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry에서는 간격이 없는 프롬프트가 고정 10분 스케줄로 실행됩니다.
</Note>

<h3 id="run-the-built-in-maintenance-prompt">
  내장 유지보수 프롬프트 실행하기
</h3>

프롬프트를 생략하면 Claude는 제공한 프롬프트 대신 내장 유지보수 프롬프트를 사용합니다. 각 반복에서 다음을 순서대로 진행합니다.

* 대화에서 미완료된 작업 계속하기
* 현재 브랜치의 풀 요청 관리: 검토 의견, 실패한 CI 실행, 병합 충돌
* 다른 것이 보류 중이지 않을 때 버그 사냥이나 단순화와 같은 정리 통과 실행

Claude는 해당 범위 외의 새로운 이니셔티브를 시작하지 않으며, 푸시나 삭제와 같은 되돌릴 수 없는 작업은 트랜스크립트가 이미 승인한 것을 계속할 때만 진행됩니다.

```text theme={null}
/loop
```

단순한 `/loop`는 [동적으로 선택된 간격](#let-claude-choose-the-interval)에서 이 프롬프트를 실행합니다. 고정 스케줄에서 실행하려면 `/loop 15m`과 같이 간격을 추가하세요. 내장 프롬프트를 자신의 기본값으로 바꾸려면 [loop.md로 기본 프롬프트 사용자 정의하기](#customize-the-default-prompt-with-loop-md)를 참조하세요.

<Note>
  Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry에서는 프롬프트가 없는 `/loop`가 유지보수 프롬프트를 실행하는 대신 사용 메시지를 출력합니다.
</Note>

<h3 id="customize-the-default-prompt-with-loop-md">
  loop.md로 기본 프롬프트 사용자 정의하기
</h3>

`loop.md` 파일은 내장 유지보수 프롬프트를 자신의 지침으로 바꿉니다. 이는 단순한 `/loop`에 대한 단일 기본 프롬프트를 정의하며, 별도의 스케줄된 작업 목록이 아니고, 명령줄에서 프롬프트를 제공할 때마다 무시됩니다. 추가 프롬프트를 함께 스케줄하려면 `/loop <prompt>`를 사용하거나 [Claude에게 직접 요청](#manage-scheduled-tasks)하세요.

Claude는 두 위치에서 파일을 찾고 먼저 찾은 것을 사용합니다.

| 경로                  | 범위                                      |
| :------------------ | :-------------------------------------- |
| `.claude/loop.md`   | 프로젝트 수준. 두 파일이 모두 존재할 때 우선순위를 가집니다.     |
| `~/.claude/loop.md` | 사용자 수준. 자신의 파일을 정의하지 않는 모든 프로젝트에 적용됩니다. |

파일은 필수 구조가 없는 일반 Markdown입니다. `/loop` 프롬프트를 직접 입력하는 것처럼 작성하세요. 다음 예시는 릴리스 브랜치를 건강하게 유지합니다.

```markdown title=".claude/loop.md" theme={null}
Check the `release/next` PR. If CI is red, pull the failing job log,
diagnose, and push a minimal fix. If new review comments have arrived,
address each one and resolve the thread. If everything is green and
quiet, say so in one line.
```

`loop.md`에 대한 편집은 다음 반복에서 적용되므로 루프가 실행 중인 동안 지침을 개선할 수 있습니다. 두 위치 중 어디에도 `loop.md`가 없으면 루프는 내장 유지보수 프롬프트로 폴백됩니다. 파일을 간결하게 유지하세요. 25,000바이트를 초과하는 내용은 잘립니다.

<Note>
  Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry에서는 `loop.md`가 읽혀지지 않으며 프롬프트가 없는 `/loop`가 유지보수 프롬프트를 실행하는 대신 사용 메시지를 출력합니다.
</Note>

<h3 id="stop-a-loop">
  루프 중지하기
</h3>

`/loop`가 다음 반복을 기다리는 동안 중지하려면 `Esc`를 누르세요. 이는 보류 중인 웨이크업을 지우므로 루프가 다시 실행되지 않습니다. [Claude에게 직접 요청](#manage-scheduled-tasks)하여 스케줄한 작업은 `Esc`의 영향을 받지 않으며 삭제할 때까지 유지됩니다.

[자기 속도 모드](#let-claude-choose-the-interval)에서 Claude는 작업이 완료되면 루프를 직접 종료할 수도 있습니다. Claude는 `stop: true`로 [`ScheduleWakeup` tool](/docs/ko/tools-reference)을 호출하여 보류 중인 웨이크업을 즉시 취소합니다. 반복이 재스케줄링이나 중지 없이 끝나면 Claude Code는 약 20분 후에 하나의 폴백 웨이크업을 스케줄하고 해당 반복이 재스케줄링하지 않으면 루프를 종료합니다. v2.1.202 이전에는 재스케줄링하지 않는 것이 Claude가 루프를 직접 종료할 수 있는 유일한 방법이었습니다.

고정 간격의 루프는 중지하거나 [7일이 경과](#seven-day-expiry)할 때까지 계속 실행됩니다.

<h2 id="set-a-one-time-reminder">
  일회성 알림 설정하기
</h2>

일회성 알림의 경우 `/loop`를 사용하는 대신 자연어로 원하는 것을 설명합니다. Claude는 실행 후 자신을 삭제하는 단일 실행 작업을 스케줄합니다.

```text theme={null}
remind me at 3pm to push the release branch
```

```text theme={null}
in 45 minutes, check whether the integration tests passed
```

Claude는 cron 표현식을 사용하여 실행 시간을 특정 분과 시간으로 고정하고 실행 시간을 확인합니다.

<h2 id="manage-scheduled-tasks">
  스케줄된 작업 관리하기
</h2>

Claude에게 자연어로 작업을 나열하거나 취소하도록 요청하거나 기본 도구를 직접 참조합니다.

```text theme={null}
what scheduled tasks do I have?
```

```text theme={null}
cancel the deploy check job
```

내부적으로 Claude는 다음 도구를 사용합니다.

| 도구           | 목적                                                               |
| :----------- | :--------------------------------------------------------------- |
| `CronCreate` | 새 작업을 스케줄합니다. 5필드 cron 표현식, 실행할 프롬프트, 반복 여부 또는 일회성 실행 여부를 허용합니다. |
| `CronList`   | ID, 스케줄, 프롬프트와 함께 모든 스케줄된 작업을 나열합니다.                             |
| `CronDelete` | ID로 작업을 취소합니다.                                                   |

각 스케줄된 작업에는 `CronDelete`에 전달할 수 있는 8자 ID가 있습니다. 세션은 한 번에 최대 50개의 스케줄된 작업을 보유할 수 있습니다.

<h2 id="how-scheduled-tasks-run">
  스케줄된 작업이 실행되는 방식
</h2>

스케줄러는 매초 기한이 된 작업을 확인하고 낮은 우선순위로 큐에 넣습니다. 스케줄된 프롬프트는 차례 사이에 실행되며, Claude가 응답 중일 때는 실행되지 않습니다. Claude가 작업이 기한이 될 때 바쁘면 프롬프트는 현재 차례가 끝날 때까지 기다립니다.

모든 시간은 현지 시간대로 해석됩니다. `0 9 * * *`와 같은 cron 표현식은 UTC가 아니라 Claude Code를 실행 중인 곳의 오전 9시를 의미합니다.

<h3 id="jitter">
  지터
</h3>

스케줄러는 모든 세션이 동일한 벽시계 시간에 API에 도달하는 것을 방지하기 위해 실행 시간에 결정론적 오프셋을 추가합니다:

* 반복 작업은 스케줄된 시간 이후 최대 30분 후에 실행되거나(또는 시간별보다 더 자주 실행되는 작업의 경우 간격의 최대 절반), 최대 30분으로 제한됩니다. `:00`에 스케줄된 시간별 작업은 `:30`까지 언제든지 실행될 수 있습니다.
* 시간의 맨 위 또는 맨 아래에 스케줄된 일회성 작업은 최대 90초 일찍 실행됩니다.

오프셋은 작업 ID에서 파생되므로 동일한 작업은 항상 동일한 오프셋을 가집니다. 정확한 타이밍이 중요한 경우 `0 9 * * *` 대신 `3 9 * * *`와 같이 `:00` 또는 `:30`이 아닌 분을 선택하면 일회성 지터가 적용되지 않습니다.

<h3 id="seven-day-expiry">
  7일 만료
</h3>

반복 작업은 생성 후 7일 후 자동으로 만료됩니다. 작업은 마지막으로 한 번 실행된 후 자신을 삭제합니다. 이는 잊혀진 루프가 실행될 수 있는 기간을 제한합니다. 반복 작업이 더 오래 지속되어야 하는 경우 만료되기 전에 취소하고 다시 만들거나 지속적인 스케줄링을 위해 [Routines](/docs/ko/routines) 또는 [Desktop scheduled tasks](/docs/ko/desktop-scheduled-tasks)를 사용하세요.

<h2 id="cron-expression-reference">
  Cron 표현식 참조
</h2>

`CronCreate`는 표준 5필드 cron 표현식을 허용합니다: `minute hour day-of-month month day-of-week`. 모든 필드는 와일드카드(`*`), 단일 값(`5`), 단계(`*/15`), 범위(`1-5`), 쉼표로 구분된 목록(`1,15,30`)을 지원합니다.

| 예시             | 의미                      |
| :------------- | :---------------------- |
| `*/5 * * * *`  | 5분마다                    |
| `0 * * * *`    | 매시간 정각                  |
| `7 * * * *`    | 매시간 7분                  |
| `0 9 * * *`    | 매일 오전 9시(현지 시간)         |
| `0 9 * * 1-5`  | 평일 오전 9시(현지 시간)         |
| `30 14 15 3 *` | 3월 15일 오후 2시 30분(현지 시간) |

요일은 일요일의 경우 `0` 또는 `7`, 토요일의 경우 `6`을 사용합니다. `L`, `W`, `?`와 같은 확장 구문 및 `MON` 또는 `JAN`과 같은 이름 별칭은 지원되지 않습니다.

월의 날짜와 요일이 모두 제한되면 두 필드 중 하나라도 일치하면 날짜가 일치합니다. 이는 표준 vixie-cron 의미론을 따릅니다.

<h2 id="disable-scheduled-tasks">
  스케줄된 작업 비활성화하기
</h2>

환경에서 `CLAUDE_CODE_DISABLE_CRON=1`을 설정하여 스케줄러를 완전히 비활성화합니다. cron 도구와 `/loop`를 사용할 수 없게 되며, 이미 스케줄된 모든 작업이 실행을 중지합니다. 비활성화 플래그의 전체 목록은 [Environment variables](/docs/ko/env-vars)를 참조하세요.

<h2 id="limitations">
  제한 사항
</h2>

세션 범위 스케줄링에는 고유한 제약이 있습니다.

* 작업은 Claude Code가 실행 중이고 유휴 상태일 때만 실행됩니다. 터미널을 닫거나 세션을 종료하면 작업 실행이 중지됩니다. [세션을 백그라운드로 전환](/docs/ko/agent-view#from-inside-a-session)하면 `/loop` 작업이 백그라운드 세션으로 이동되어 터미널 없이 계속 실행됩니다.
* 놓친 실행에 대한 추적 없음. 작업의 스케줄된 시간이 Claude가 오래 실행되는 요청에 바쁠 때 지나가면 Claude가 유휴 상태가 될 때 한 번 실행되며, 놓친 각 간격마다 한 번씩 실행되지 않습니다.
* 새로운 대화를 시작하면 모든 세션 범위 작업이 지워집니다. `claude --resume` 또는 `claude --continue`로 재개하면 만료되지 않은 작업이 복원됩니다. 생성 후 7일 이내의 반복 작업, 스케줄된 시간이 아직 지나지 않은 일회성 작업입니다. 백그라운드 Bash 및 모니터 작업은 재개 시 복원되지 않습니다.

무인으로 실행해야 하는 cron 기반 자동화의 경우:

* [Routines](/docs/ko/routines): Anthropic 관리 인프라에서 스케줄에 따라, API 호출을 통해, 또는 GitHub 이벤트에서 실행
* [GitHub Actions](/docs/ko/github-actions): CI에서 `schedule` 트리거 사용
* [Desktop scheduled tasks](/docs/ko/desktop-scheduled-tasks): 머신에서 로컬로 실행
