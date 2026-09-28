> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude를 목표를 향해 계속 작동하게 하기

> /goal로 완료 조건을 설정하면 Claude가 조건이 충족될 때까지 계속 작동하며, 모델이 불가능하다고 판단하거나 수정해야 할 오류가 발생하면 목표가 지워집니다.

`/goal` 명령은 완료 조건을 설정하고 Claude가 사용자가 각 단계를 프롬프트하지 않아도 그 조건을 향해 계속 작동하도록 합니다. 각 턴 후에 작은 빠른 모델이 조건이 충족되었는지 확인합니다. 조건이 아직 충족되지 않으면 Claude는 제어를 사용자에게 반환하는 대신 다른 턴을 시작합니다. 목표는 조건이 충족되면 자동으로 지워지거나, 모델이 조건을 만족하는 것이 불가능하다고 판단하거나, [수정해야 할 오류](#errors-you-have-to-fix-clear-the-goal)로 인해 턴이 실패하면 지워집니다.

검증 가능한 최종 상태가 있는 실질적인 작업에 목표를 사용합니다:

* 모든 호출 사이트가 컴파일되고 테스트가 통과할 때까지 모듈을 새로운 API로 마이그레이션
* 모든 수용 기준이 충족될 때까지 설계 문서 구현
* 각각이 크기 예산 이하가 될 때까지 큰 파일을 집중된 모듈로 분할
* 큐가 비워질 때까지 레이블이 지정된 이슈 백로그 처리

<h2 id="compare-ways-to-keep-a-session-running">
  세션을 계속 실행하는 방법 비교
</h2>

세 가지 접근 방식이 프롬프트 사이의 현재 세션을 계속 실행합니다. 다음 턴을 시작해야 할 때를 기준으로 선택합니다:

| 접근 방식                                                               | 다음 턴 시작 시기                                                                                                                      | 중지 시기                                                                                                                                             |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/goal`                                                             | 이전 턴이 완료될 때, 또는 대화형 세션에서 [유휴 체크인](#background-work-defers-evaluation) 또는 [자동 재시도](#other-errors-retry-or-pause-the-goal)가 발생할 때 | 모델이 조건이 충족되었음을 확인하거나 불가능하다고 판단할 때, 또는 [수정해야 하는 오류](#errors-you-have-to-fix-clear-the-goal)로 인해 턴이 실패할 때, 또는 [`/goal clear`](#clear-a-goal)를 실행할 때 |
| [`/loop`](/docs/ko/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | 시간 간격이 경과할 때                                                                                                                    | 사용자가 중지하거나 Claude가 작업이 완료되었다고 판단할 때                                                                                                               |
| [Stop hook](/docs/ko/hooks-guide#prompt-based-hooks)                     | 이전 턴이 완료될 때                                                                                                                     | 사용자의 스크립트 또는 프롬프트가 결정할 때                                                                                                                          |

`/goal`과 Stop hook은 모두 매 턴 후에 실행됩니다. `/goal`은 세션 범위의 단축키입니다: 조건을 입력하면 현재 세션에서만 활성화됩니다. Stop hook은 설정 파일에 있고 범위 내의 모든 세션에 적용되며 결정론적 확인을 위해 스크립트를 실행하거나 모델 평가를 위해 프롬프트를 실행할 수 있습니다.

[자동 모드](/docs/ko/auto-mode-config)는 단일 턴 내에서 도구 호출을 승인하지만 새로운 턴을 시작하지는 않습니다. Claude는 작업이 완료되었다고 판단할 때 중지합니다. `/goal`은 매 턴 후에 조건을 확인하는 별도의 평가자를 추가하므로 완료는 작업을 수행하는 모델이 아닌 새로운 모델에 의해 결정됩니다. 두 가지는 상호 보완적입니다: 자동 모드는 도구별 프롬프트를 제거하고 `/goal`은 턴별 프롬프트를 제거합니다.

<Tip>
  위의 접근 방식은 현재 세션을 계속 실행합니다. 야간 테스트나 아침 분류와 같이 열린 세션과 무관하게 실행되는 작업을 예약할 수도 있습니다. 클라우드 루틴 및 데스크톱 예약된 작업에 대해 [예약 옵션](/docs/ko/scheduled-tasks#compare-scheduling-options)을 참조하세요.
</Tip>

<h2 id="use-/goal">
  `/goal` 사용
</h2>

세션당 하나의 목표만 활성화될 수 있습니다. 동일한 명령이 인수에 따라 설정, 확인, 지웁니다.

<h3 id="set-a-goal">
  목표 설정
</h3>

`/goal` 다음에 만족하려는 조건을 입력합니다. 목표가 이미 활성화되어 있으면 새 목표가 이를 대체합니다.

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

목표를 설정하면 조건 자체를 지시문으로 하여 즉시 턴을 시작합니다. 별도의 프롬프트를 보낼 필요가 없습니다. 목표가 활성화되어 있는 동안 `◎ /goal active` 표시기가 목표가 실행된 시간을 표시합니다.

목표는 권한 모드를 변경하지 않습니다. 목표 턴이 무인으로 실행되도록 하려면 `/goal`을 [자동 모드](/docs/ko/auto-mode-config)에서 실행합니다. [수동 모드](/docs/ko/permission-modes)에서는 Claude가 설정에서 이미 허용하지 않는 위의 테스트 명령과 같은 도구 호출 전에 여전히 확인을 요청합니다.

목표가 활성화되어 있는 동안 대화 기록은 평가자가 반환하는 각 판정을 표시하며, Ctrl+O를 눌러 그 뒤의 이유를 볼 수 있습니다. 상태 보기도 가장 최근의 이유를 표시하므로 Claude가 다음에 작업할 내용을 볼 수 있습니다.

<h3 id="write-an-effective-condition">
  효과적인 조건 작성
</h3>

[평가자](#how-evaluation-works)는 Claude가 대화에서 표시한 내용에 대해 조건을 판단합니다. 독립적으로 명령을 실행하거나 파일을 읽지 않으므로 Claude의 자체 출력이 입증할 수 있는 것으로 조건을 작성합니다. "`test/auth`의 모든 테스트 통과"는 Claude가 테스트를 실행하고 결과가 평가자가 읽을 수 있도록 대화 기록에 나타나기 때문에 작동합니다.

많은 턴에 걸쳐 유지되는 조건은 일반적으로 다음을 포함합니다:

* **하나의 측정 가능한 최종 상태**: 테스트 결과, 빌드 종료 코드, 파일 수, 빈 큐
* **명시된 확인**: Claude가 이를 입증하는 방법(예: "`npm test` 종료 0" 또는 "`git status`가 깨끗함")
* **중요한 제약 조건**: 그 과정에서 변경되지 않아야 하는 모든 것(예: "다른 테스트 파일은 수정되지 않음")

조건은 최대 4,000자까지 가능합니다.

목표가 실행되는 시간을 제한하려면 조건에 턴 또는 시간 절을 포함합니다(예: `or stop after 20 turns`). Claude는 매 턴마다 해당 절에 대한 진행 상황을 보고하고 평가자는 대화에서 이를 판단합니다.

<h3 id="check-status">
  상태 확인
</h3>

인수 없이 `/goal`을 실행하여 현재 상태를 확인합니다.

```text theme={null}
/goal
```

목표가 활성화되어 있으면 상태는 다음을 표시합니다:

* 조건
* 실행된 시간
* 평가된 턴 수
* 현재 토큰 소비
* 평가자의 가장 최근 이유

턴 수와 가장 최근 이유는 첫 번째 평가가 실행된 후에 나타납니다.

목표가 활성화되지 않았지만 세션 초반에 달성된 경우 상태는 달성된 조건과 함께 지속 시간, 턴 수, 토큰 소비를 표시합니다.

<h3 id="clear-a-goal">
  목표 지우기
</h3>

`/goal clear`를 실행하여 조건이 충족되기 전에 활성 목표를 제거합니다.

```text theme={null}
/goal clear
```

Claude는 `Goal cleared:` 다음에 조건을 출력하여 확인하거나, 활성 상태인 것이 없으면 `No goal set`을 출력합니다.

`stop`, `off`, `reset`, `none`, `cancel`은 `clear`의 별칭으로 허용됩니다. `/clear`를 실행하여 새 대화를 시작하면 활성 목표도 제거됩니다.

<h3 id="resume-with-an-active-goal">
  활성 목표로 재개
</h3>

세션이 종료될 때 여전히 활성 상태였던 목표는 Claude Code가 복원합니다. Claude Code는 모든 재개 경로에서 이를 복원합니다: `--continue`, 세션 ID, 이름 또는 [대화 기록 파일 경로](/docs/ko/sessions#resume-a-session)를 사용한 `--resume`, 그리고 [세션 선택기](/docs/ko/sessions#use-the-session-picker). v2.1.239 이전에는 Claude Code가 `claude --resume` 선택기를 제외한 모든 경로에서 목표를 복원했습니다.

Claude Code는 조건을 유지하지만 턴 수, 타이머, 토큰 소비 기준선을 재설정합니다. 이미 달성되었거나 지워진 목표는 복원되지 않습니다.

<h3 id="run-non-interactively">
  비대화형으로 실행
</h3>

`/goal`은 [비대화형 모드](/docs/ko/headless), [데스크톱 앱](/docs/ko/desktop), [원격 제어](/docs/ko/remote-control)에서 작동합니다. `-p`로 목표를 설정하면 단일 호출에서 루프를 완료까지 실행합니다:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

기본 텍스트 출력을 사용하면 실행이 끝날 때까지 아무것도 출력되지 않으므로 많은 턴을 실행하는 목표는 멈춘 것처럼 보일 수 있습니다. 루프가 실행되는 동안 각 메시지를 내보내려면 `--output-format stream-json --verbose`를 추가합니다.

Ctrl+C로 프로세스를 중단하여 조건이 충족되기 전에 비대화형 목표를 중지합니다.

<h2 id="how-evaluation-works">
  평가 작동 방식
</h2>

`/goal`은 세션 범위의 [프롬프트 기반 Stop hook](/docs/ko/hooks#prompt-based-hooks) 주위의 래퍼입니다. Claude가 턴을 완료할 때마다 조건과 지금까지의 대화가 구성된 [작은 빠른 모델](/docs/ko/model-config)로 전송되며, 기본값은 Claude API의 Haiku입니다. 타사 공급자의 경우 플랫폼의 기본값에 대해 [공급자 페이지](/docs/ko/third-party-integrations)를 확인하십시오. 모델은 다음 세 가지 판정 중 하나를 반환하며, 각각 짧은 이유가 포함됩니다:

* **아직 충족되지 않음**: Claude는 계속 작동하고 다음 턴의 지침으로 이유를 사용합니다.
* **충족됨**: Claude Code는 목표를 지우고 대화 기록에 달성된 항목을 기록합니다.
* **불가능**: 평가자가 조건을 절대 만족할 수 없다고 판단했습니다. Claude Code는 목표를 지우고 이유와 함께 대화 기록에 실패한 항목을 기록합니다. 직접 지울 필요가 없습니다.

Claude가 평가자에게 계속 응답하면서 진행 상황이 없으면(여러 턴 동안 도구 사용이 없음), Claude Code는 루프를 중지하고 경고를 출력한 후 목표가 여전히 설정된 상태로 제어를 반환합니다. 평가는 다음 프롬프트 후에 재개됩니다. [hooks 가이드](/docs/ko/hooks-guide#stop-hook-hits-the-block-cap)에서 기본 메커니즘을 설명합니다.

<h3 id="when-a-turn-fails">
  턴이 실패할 때
</h3>

턴이 실패하면 오류가 수정해야 하는 오류인 경우 Claude Code는 목표를 지웁니다. 다른 모든 오류 후에는 목표가 설정된 상태로 유지됩니다.

<h4 id="errors-you-have-to-fix-clear-the-goal">
  수정해야 하는 오류가 목표를 지웁니다
</h4>

턴이 수정할 때까지 지워지지 않는 오류로 실패하면 Claude Code는 목표를 지우고 원인을 명시하는 경고를 출력합니다. 경고는 `Goal cleared after an unrecoverable error`로 시작하고 `Run /goal again to continue`로 끝납니다. 원인을 수정한 후 `/goal <condition>`으로 [목표를 다시 설정](#set-a-goal)하십시오. 네 가지 종류의 실패가 목표를 지웁니다:

* Claude Code가 자체 자격 증명을 관리할 때의 인증 실패입니다. 데스크톱 앱, VS Code 확장 프로그램 또는 [클라우드 세션](/docs/ko/claude-code-on-the-web)과 같이 호스트가 자격 증명을 관리하는 경우 Claude Code는 호스트가 자체적으로 액세스를 복원하기 때문에 목표를 활성 상태로 유지합니다.
* 소진된 크레딧 잔액
* [자동 압축](/docs/ko/model-config#set-the-auto-compact-window)이 지울 수 없는 컨텍스트 오버플로우
* 사용할 수 없는 모델

<h4 id="other-errors-retry-or-pause-the-goal">
  다른 오류는 목표를 재시도하거나 일시 중지합니다
</h4>

다른 모든 실패 후에는 목표가 설정된 상태로 유지됩니다. Claude Code v2.1.269 이상의 대화형 세션에서 Claude Code는 또한 원인을 명시하는 줄을 출력하고 자체적으로 재시도하거나 대기합니다:

* **재시도**: 과부하 서버 또는 끊어진 연결과 같이 자체적으로 지워지는 경향이 있는 실패 후에는 `Goal still active`로 시작하는 알림이 다음 시도 전의 대기 시간을 표시합니다. 3번의 자동 재시도 후에는 목표가 대신 일시 중지됩니다.
* **일시 중지**: API 속도 제한, claude.ai [사용 제한](/docs/ko/errors#youve-hit-your-session-limit) 또는 턴을 종료한 hook과 같이 재시도가 반복될 수만 있는 실패 후에는 `Goal paused`로 시작하는 알림이 원인을 명시합니다. 세션이 [사용 제한이 재설정될 때 자동으로 계속 대기](/docs/ko/interactive-mode#wait-for-a-usage-limit-to-reset) 중이면 Claude는 그때 목표를 향해 작업을 재개합니다.

언제든지 메시지를 보내 다음 턴을 즉시 시작하십시오. 자동 재시도를 끄려면 [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/ko/env-vars)를 `0`으로 설정하십시오. 이는 [체크인](#background-work-defers-evaluation)도 끕니다.

<h3 id="background-work-defers-evaluation">
  백그라운드 작업이 평가를 연기합니다
</h3>

서브에이전트 또는 백그라운드 셸 명령이 턴이 끝날 때 여전히 실행 중이면 Claude Code는 해당 턴의 평가를 건너뜁니다. 백그라운드 작업이 실행되지 않는 상태로 끝나는 다음 턴의 끝에서 평가합니다. 백그라운드 작업이 완료되면 Claude Code는 결과를 새 턴으로 Claude에게 전달하므로 프롬프트를 입력할 필요가 없습니다.

백그라운드 작업이 목표를 30분 동안 대기하게 한 후에는 체크인이 필요합니다. 체크인에서 Claude Code는 실행 중인 작업을 나열하고 Claude에게 출력을 읽고, 진행 중이면 계속 대기하고, 막힌 작업을 수정하거나 중지하도록 요청합니다. 첫 번째 체크인 후 Claude Code는 각 이후 체크인 전에 두 배씩 더 오래 대기하며, 첫 번째 간격의 최대 4배까지입니다: 기본값으로 첫 번째 체크인 후 1시간, 그 후 2시간마다입니다. Claude Code는 다음 두 가지 방법 중 하나로 기한이 된 체크인(첫 번째 포함)을 전달합니다:

* **턴이 끝날 때**: Claude Code는 작업이 여전히 실행 중인 상태로 끝나는 다음 턴의 끝에서 체크인을 전달합니다. `-p`로 시작된 세션과 같은 비대화형 세션에서는 이것이 Claude Code가 체크인을 전달하는 유일한 방법입니다.
* **세션이 유휴 상태일 때**: 대화형 세션에서 Claude Code는 다음 프롬프트를 기다리는 대신 체크인을 전달하기 위해 자체적으로 턴을 시작합니다. 백그라운드 작업이 결과를 보고하지 않고 중지된 경우 Claude Code는 Claude에게 목표를 향해 계속하도록 요청합니다. Claude Code는 프롬프트 사이에 목표당 최대 3개의 유휴 체크인을 시작합니다. 세 번째 유휴 체크인에서 Claude Code는 다른 프롬프트를 보낼 때까지 유휴 체크인이 일시 중지되었다고 말합니다. v2.1.246 이전에는 유휴 체크인이 제한되지 않았습니다. 유휴 체크인에는 Claude Code v2.1.236 이상이 필요합니다.

v2.1.239 이전에는 유휴 체크인만 이런 방식으로 백오프되었습니다. 턴 끝에서 전달된 체크인은 첫 번째 간격에서 반복되었습니다.

첫 번째 간격을 변경하려면 [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/ko/env-vars)를 설정하십시오. Claude Code는 30분 간격 대신 값을 사용하고 이후 간격을 이에 맞게 조정합니다. 체크인을 끄려면 `0`으로 설정하십시오. 이는 [자동 재시도](#other-errors-retry-or-pause-the-goal)도 끕니다.

체크인에는 Claude Code v2.1.234 이상이 필요합니다.

<h3 id="evaluation-model-and-cost">
  평가 모델 및 비용
</h3>

다른 모델에서 평가하려면 [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/ko/model-config#environment-variables)을 설정하십시오.

<Warning>
  Claude Code는 `/goal` 평가뿐만 아니라 작은 빠른 모델을 사용하는 모든 곳에서 `ANTHROPIC_DEFAULT_HAIKU_MODEL`을 읽습니다. 설정하면 Claude Code는 [`haiku` 별칭](/docs/ko/model-config#model-aliases)을 해당 모델로 확인하고 대화 요약과 같은 [백그라운드 기능](/docs/ko/costs#background-token-usage)을 실행합니다.
</Warning>

평가자는 세션이 구성된 공급자에서 실행됩니다. 도구를 호출하지 않으므로 Claude가 이미 대화에서 표시한 내용만 판단할 수 있습니다.

<Note>
  평가 토큰은 공급자에 대해 구성된 작은 빠른 모델에서 청구되며 일반적으로 주 턴 소비에 비해 무시할 수 있습니다.
</Note>

<h2 id="requirements">
  요구 사항
</h2>

Claude Code는 평가자가 hooks 시스템의 일부이기 때문에 [`settings 파일의 hooks와 동일한 워크스페이스 신뢰 규칙`](/docs/ko/permissions#what-runs-before-you-trust-a-folder)에서 `/goal`을 사용 가능하게 합니다. [`disableAllHooks`](/docs/ko/hooks#disable-or-remove-hooks)가 설정 우선순위 적용 후 `true`이거나 관리 설정에서 [`allowManagedHooksOnly`](/docs/ko/settings-reference#allowmanagedhooksonly)가 설정되면 `/goal`도 사용할 수 없습니다. 각 경우에 명령은 조용히 아무것도 하지 않는 대신 이유를 알려줍니다.

<h2 id="see-also">
  참고 항목
</h2>

* [프롬프트를 `/loop`로 반복 실행](/docs/ko/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): 조건이 충족될 때까지가 아닌 시간 간격으로 다시 실행
* [프롬프트 기반 hooks](/docs/ko/hooks-guide#prompt-based-hooks): 사용자 정의 평가 로직이 필요할 때 자신의 Stop hook 작성
* [자동 모드](/docs/ko/auto-mode-config): 도구 호출을 자동으로 승인하여 각 목표 턴이 무인으로 실행되도록 함
* [예약 비교](/docs/ko/scheduled-tasks#compare-scheduling-options): 열린 세션과 무관하게 일정에 따라 작업 실행
