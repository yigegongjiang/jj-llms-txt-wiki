> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude를 목표를 향해 계속 작동하게 하기

> /goal로 완료 조건을 설정하면 Claude가 조건이 충족될 때까지 여러 턴에 걸쳐 계속 작동합니다.

<Note>
  `/goal`은 Claude Code v2.1.139 이상이 필요합니다.
</Note>

`/goal` 명령은 완료 조건을 설정하고 Claude가 사용자가 각 단계를 프롬프트하지 않아도 그 조건을 향해 계속 작동하도록 합니다. 각 턴 후에 작은 빠른 모델이 조건이 충족되었는지 확인합니다. 충족되지 않으면 Claude는 제어를 사용자에게 반환하는 대신 다른 턴을 시작합니다. 조건이 충족되면 목표는 자동으로 지워집니다.

검증 가능한 최종 상태가 있는 실질적인 작업에 목표를 사용합니다:

* 모든 호출 사이트가 컴파일되고 테스트가 통과할 때까지 모듈을 새로운 API로 마이그레이션
* 모든 수용 기준이 충족될 때까지 설계 문서 구현
* 각각이 크기 예산 이하가 될 때까지 큰 파일을 집중된 모듈로 분할
* 큐가 비워질 때까지 레이블이 지정된 이슈 백로그 처리

<h2 id="compare-ways-to-keep-a-session-running">
  세션을 계속 실행하는 방법 비교
</h2>

세 가지 접근 방식이 프롬프트 사이의 현재 세션을 계속 실행합니다. 다음 턴을 시작해야 할 때를 기준으로 선택합니다:

| 접근 방식                                                               | 다음 턴 시작 시기   | 중지 시기                               |
| :------------------------------------------------------------------ | :----------- | :---------------------------------- |
| `/goal`                                                             | 이전 턴이 완료될 때  | 모델이 조건이 충족되었음을 확인할 때                |
| [`/loop`](/docs/ko/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | 시간 간격이 경과할 때 | 사용자가 중지하거나 Claude가 작업이 완료되었다고 판단할 때 |
| [Stop hook](/docs/ko/hooks-guide#prompt-based-hooks)                     | 이전 턴이 완료될 때  | 사용자의 스크립트 또는 프롬프트가 결정할 때            |

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

목표는 권한을 변경하지 않습니다. 기본 권한 모드에서 Claude는 설정에서 이미 허용하지 않는 테스트 명령과 같은 도구 호출 전에 여전히 확인을 요청합니다. 목표 턴이 무인으로 실행되도록 하려면 `/goal`을 [자동 모드](/docs/ko/auto-mode-config)와 함께 사용합니다.

각 턴 후에 평가자는 조건이 충족되었는지 여부를 설명하는 짧은 이유를 반환합니다. 가장 최근의 이유는 상태 보기 및 대화 기록에 나타나므로 Claude가 다음에 작업할 내용을 볼 수 있습니다.

<Note>
  목표는 조건이 충족되거나 `/goal clear`를 실행할 때까지 계속 실행됩니다. 인수 없이 `/goal`을 실행하면 지금까지 소비한 턴과 토큰을 볼 수 있습니다.
</Note>

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

세션이 종료될 때 여전히 활성 상태였던 목표는 `--resume` 또는 `--continue`로 해당 세션을 재개할 때 복원됩니다. 조건은 유지되지만 턴 수, 타이머, 토큰 소비 기준선은 모두 재개 시 재설정됩니다. 이미 달성되었거나 지워진 목표는 복원되지 않습니다.

<h3 id="run-non-interactively">
  비대화형으로 실행
</h3>

`/goal`은 [비대화형 모드](/docs/ko/headless), [데스크톱 앱](/docs/ko/desktop), [원격 제어](/docs/ko/remote-control)에서 작동합니다. `-p`로 목표를 설정하면 단일 호출에서 루프를 완료까지 실행합니다:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

기본 텍스트 출력을 사용하면 조건이 충족될 때까지 아무것도 출력되지 않으므로 많은 턴을 실행하는 목표는 멈춘 것처럼 보일 수 있습니다. 루프가 실행되는 동안 각 메시지를 내보내려면 `--output-format stream-json --verbose`를 추가합니다.

Ctrl+C로 프로세스를 중단하여 조건이 충족되기 전에 비대화형 목표를 중지합니다.

<h2 id="how-evaluation-works">
  평가 작동 방식
</h2>

`/goal`은 세션 범위의 [프롬프트 기반 Stop hook](/docs/ko/hooks#prompt-based-hooks) 주위의 래퍼입니다. Claude가 턴을 완료할 때마다 조건과 지금까지의 대화가 구성된 [작은 빠른 모델](/docs/ko/model-config)로 전송되며, 기본값은 Haiku입니다. 모델은 예 또는 아니오 결정과 짧은 이유를 반환합니다. "아니오"는 Claude에게 계속 작동하도록 지시하고 다음 턴의 지침으로 이유를 포함합니다. "예"는 목표를 지우고 대화 기록에 달성된 항목을 기록합니다.

평가자는 세션이 구성된 공급자에서 실행됩니다. 도구를 호출하지 않으므로 Claude가 이미 대화에서 표시한 내용만 판단할 수 있습니다.

<Note>
  평가 토큰은 공급자에 대해 구성된 작은 빠른 모델에서 청구되며 일반적으로 주 턴 소비에 비해 무시할 수 있습니다.
</Note>

<h2 id="requirements">
  요구 사항
</h2>

`/goal`은 평가자가 hooks 시스템의 일부이기 때문에 신뢰 대화를 수락한 워크스페이스에서만 실행됩니다. [`disableAllHooks`](/docs/ko/hooks#disable-or-remove-hooks)가 모든 설정 수준에서 설정되거나 관리 설정에서 [`allowManagedHooksOnly`](/docs/ko/settings#hook-configuration)가 설정되면 `/goal`을 사용할 수 없습니다. 각 경우에 명령은 조용히 아무것도 하지 않는 대신 이유를 알려줍니다.

<h2 id="see-also">
  참고 항목
</h2>

* [프롬프트를 `/loop`로 반복 실행](/docs/ko/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): 조건이 충족될 때까지가 아닌 시간 간격으로 다시 실행
* [프롬프트 기반 hooks](/docs/ko/hooks-guide#prompt-based-hooks): 사용자 정의 평가 로직이 필요할 때 자신의 Stop hook 작성
* [자동 모드](/docs/ko/auto-mode-config): 도구 호출을 자동으로 승인하여 각 목표 턴이 무인으로 실행되도록 함
* [예약 비교](/docs/ko/scheduled-tasks#compare-scheduling-options): 열린 세션과 무관하게 일정에 따라 작업 실행
