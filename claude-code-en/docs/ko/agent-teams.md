> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 세션 팀 조율하기

> 공유 작업, 에이전트 간 메시징, 중앙 집중식 관리를 통해 함께 작동하는 여러 Claude Code 인스턴스를 조율합니다.

<Warning>
  에이전트 팀은 실험적이며 기본적으로 비활성화되어 있습니다. [settings.json](/docs/ko/settings)이나 환경에 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`를 추가하여 활성화합니다. 해당 변수가 없으면 세션 시작 시 팀이 설정되지 않고, 팀 디렉토리가 작성되지 않으며, Claude가 팀원을 생성하거나 제안하지 않습니다. 에이전트 팀은 세션 재개, 작업 조율, 종료 동작 관련 [알려진 제한 사항](#limitations)이 있습니다.
</Warning>

에이전트 팀을 사용하면 함께 작동하는 여러 Claude Code 인스턴스를 조율할 수 있습니다. 한 세션이 팀 리더 역할을 하여 작업을 조율하고, 작업을 할당하며, 결과를 종합합니다. 팀원들은 독립적으로 작동하며, 각각 자신의 컨텍스트 윈도우에서 작동하고, 서로 직접 통신합니다.

단일 세션 내에서 실행되고 메인 에이전트에게만 보고할 수 있는 [subagents](/docs/ko/sub-agents)와 달리, 리더를 거치지 않고 개별 팀원과 직접 상호작용할 수도 있습니다.

<Note>
  이 페이지는 v2.1.178 기준의 에이전트 팀을 설명합니다. `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`가 설정되면, 팀원을 생성할 때 더 이상 설정 단계가 필요하지 않으며, 세션이 종료될 때 정리가 자동으로 수행됩니다. v2.1.178 이전에는 먼저 Claude에게 팀을 생성하고 이름을 지정하도록 요청했으며, Claude는 `TeamCreate` 및 `TeamDelete` 도구를 사용하여 설정하고 제거했습니다. 두 도구는 더 이상 존재하지 않습니다. Agent 도구의 `team_name` 입력은 허용되지만 무시되며, `TaskCreated`, `TaskCompleted`, 및 `TeammateIdle` [hook payloads](/docs/ko/hooks#taskcreated)의 `team_name` 필드는 세션에서 파생된 이름을 전달하며 더 이상 사용되지 않습니다.
</Note>

<h2 id="when-to-use-agent-teams">
  에이전트 팀을 사용할 때
</h2>

에이전트 팀은 병렬 탐색이 실질적인 가치를 더하는 작업에 가장 효과적입니다. 전체 시나리오는 [사용 사례 예시](#use-case-examples)를 참조합니다. 가장 강력한 사용 사례는 다음과 같습니다:

* **연구 및 검토**: 여러 팀원이 문제의 다양한 측면을 동시에 조사한 후 서로의 발견을 공유하고 도전할 수 있습니다
* **새로운 모듈 또는 기능**: 팀원들이 각각 별도의 부분을 소유하면서 서로 간섭하지 않을 수 있습니다
* **경쟁하는 가설로 디버깅하기**: 팀원들이 다양한 이론을 병렬로 테스트하고 더 빠르게 답에 수렴합니다
* **교차 계층 조율**: 프론트엔드, 백엔드, 테스트에 걸친 변경 사항으로, 각각 다른 팀원이 소유합니다

에이전트 팀은 조율 오버헤드를 추가하고 단일 세션보다 훨씬 더 많은 토큰을 사용합니다. 팀원들이 독립적으로 작동할 수 있을 때 가장 잘 작동합니다. 순차적 작업, 동일 파일 편집, 또는 많은 종속성이 있는 작업의 경우 단일 세션이나 [subagents](/docs/ko/sub-agents)가 더 효과적입니다.

<h3 id="compare-with-subagents">
  subagents와 비교
</h3>

에이전트 팀과 [subagents](/docs/ko/sub-agents) 모두 작업을 병렬화할 수 있지만, 다르게 작동합니다. 워커들이 서로 통신해야 하는지 여부에 따라 선택합니다:

<Frame caption="Subagents는 결과만 메인 에이전트에게 보고하고 서로 대화하지 않습니다. 에이전트 팀에서는 팀원들이 작업 목록을 공유하고, 작업을 요청하며, 서로 직접 통신합니다.">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Subagent와 에이전트 팀 아키텍처를 비교하는 다이어그램입니다. Subagents는 메인 에이전트에 의해 생성되고, 작업을 수행하며, 결과를 보고합니다. 에이전트 팀은 공유 작업 목록을 통해 조율되며, 팀원들이 서로 직접 통신합니다." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Subagent와 에이전트 팀 아키텍처를 비교하는 다이어그램입니다. Subagents는 메인 에이전트에 의해 생성되고, 작업을 수행하며, 결과를 보고합니다. 에이전트 팀은 공유 작업 목록을 통해 조율되며, 팀원들이 서로 직접 통신합니다." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|           | Subagents                   | 에이전트 팀                    |
| :-------- | :-------------------------- | :------------------------ |
| **컨텍스트**  | 자신의 컨텍스트 윈도우; 결과는 호출자에게 반환됨 | 자신의 컨텍스트 윈도우; 완전히 독립적     |
| **통신**    | 메인 에이전트에게만 결과 보고            | 팀원들이 서로 직접 메시지 전송         |
| **조율**    | 메인 에이전트가 모든 작업 관리           | 자체 조율을 통한 공유 작업 목록        |
| **최적 용도** | 결과만 중요한 집중된 작업              | 논의와 협업이 필요한 복잡한 작업        |
| **토큰 비용** | 낮음: 결과가 메인 컨텍스트로 요약됨        | 높음: 각 팀원이 별도의 Claude 인스턴스 |

결과를 보고하는 빠르고 집중된 워커가 필요할 때는 subagents를 사용합니다. 팀원들이 발견을 공유하고, 서로 도전하며, 자체적으로 조율해야 할 때는 에이전트 팀을 사용합니다.

<h2 id="enable-agent-teams">
  에이전트 팀 활성화
</h2>

에이전트 팀은 기본적으로 비활성화되어 있습니다. `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` 환경 변수를 `1`로 설정하여 활성화합니다. 셸 환경이나 [settings.json](/docs/ko/settings)을 통해 설정할 수 있습니다:

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

<h2 id="start-your-first-agent-team">
  첫 번째 에이전트 팀 시작하기
</h2>

에이전트 팀을 활성화한 후, 자연어로 원하는 작업과 팀원들을 설명합니다. Claude는 팀원들을 생성하고 프롬프트에 따라 작업을 조율합니다.

이 예시는 세 가지 역할이 독립적이고 서로를 기다리지 않고 문제를 탐색할 수 있기 때문에 잘 작동합니다:

```text theme={null}
I'm designing a CLI tool that helps developers track TODO comments across
their codebase. Spawn three teammates to explore this from different angles:
one on UX, one on technical architecture, one playing devil's advocate.
```

그러면 Claude는 [공유 작업 목록](/docs/ko/interactive-mode#task-list)을 채우고, 각 관점에 대한 팀원들을 생성하며, 문제를 탐색하고, 완료되었을 때 발견을 종합합니다.

리더의 터미널은 프롬프트 입력 아래의 에이전트 패널에 팀원들을 나열합니다. 패널에서:

* **위아래 화살표**: 팀원 선택
* **Enter**: 선택한 팀원의 대화 기록을 열고 직접 메시지 전송
* **Escape**: 선택한 팀원의 현재 턴 중단

v2.1.199부터, 유휴 팀원의 행은 다른 팀원이나 서브에이전트가 여전히 작업 중인 동안 패널에 남아 있으므로, 이를 선택하여 대화 기록을 검토하거나 추가 작업을 보낼 수 있습니다. 패널의 모든 에이전트가 유휴 상태가 되면, 유휴 행은 30초 후 숨겨지고 팀원의 다음 턴에 다시 나타납니다. 팀원은 숨겨진 상태에서도 계속 실행되고 주소 지정 가능합니다. v2.1.181부터 v2.1.198까지는, 유휴 행이 다른 팀원들이 여전히 작업 중이더라도 자신의 턴이 끝난 후 30초 후에 숨겨졌습니다. v2.1.181 이전 버전에서는 유휴 행이 숨겨지지 않습니다.

세 명 이상의 팀원이 동시에 유휴 상태일 때, 처음 세 명을 넘는 행들은 `5명이 유휴 상태일 때 2명의 유휴 에이전트`와 같이 축소된 팀원의 수를 세는 단일 행으로 축소됩니다. 이를 선택하고 Enter를 누르면 축소된 행들이 확장되거나, Esc를 누르면 다시 축소됩니다. 작업 중인 팀원, 실패한 팀원, 그리고 현재 보고 있는 팀원은 항상 자신의 행을 유지합니다.

각 팀원이 자신의 분할 창에 있기를 원하면 [표시 모드 선택](#choose-a-display-mode)을 참조합니다.

<h2 id="control-your-agent-team">
  에이전트 팀 제어하기
</h2>

리더에게 자연어로 원하는 것을 말합니다. 지시에 따라 팀 조율, 작업 할당, 위임을 처리합니다.

<h3 id="choose-a-display-mode">
  표시 모드 선택
</h3>

에이전트 팀은 두 가지 표시 모드를 지원합니다:

* **In-process**: 모든 팀원이 메인 터미널 내에서 실행됩니다. 에이전트 패널에서 위아래 화살표 키를 사용하여 팀원을 선택한 후 Enter를 눌러 보고 입력하여 직접 메시지를 보냅니다. 모든 터미널에서 작동하며 추가 설정이 필요하지 않습니다.
* **분할 창**: 각 팀원이 자신의 창을 가집니다. 모든 사람의 출력을 한 번에 볼 수 있고 창을 클릭하여 직접 상호작용할 수 있습니다. tmux 또는 iTerm2가 필요합니다.

<Note>
  `tmux`는 특정 운영 체제에서 알려진 제한 사항이 있으며 전통적으로 macOS에서 가장 잘 작동합니다. iTerm2에서 `tmux -CC`를 사용하는 것이 `tmux`로의 권장 진입점입니다.
</Note>

기본값은 `"in-process"`입니다. v2.1.179 이전에는 기본값이 `"auto"`였으므로, 이전에 분할 창을 열었던 업그레이드된 세션은 모드를 명시적으로 설정하지 않으면 이제 한 터미널에 머물러 있습니다. `"auto"`를 설정하여 이미 tmux 세션 내에서 실행 중이거나 터미널이 iTerm2인 경우 분할 창을 활성화하고, 그렇지 않으면 in-process로 폴백합니다. `"tmux"` 설정은 분할 창 모드를 활성화하고 터미널에 따라 tmux 또는 iTerm2를 사용할지 자동으로 감지합니다.

v2.1.186부터는 `"iterm2"`를 설정하여 iTerm2 네이티브 분할 창을 명시적으로 사용합니다. 이 모드는 [`it2` CLI](https://github.com/mkusaka/it2)가 필요하며 `it2`가 누락된 경우 설치 명령과 함께 오류를 표시합니다. 터미널이 iTerm2이고 tmux를 폴백으로 사용할 수 있을 때 `"auto"` 또는 `"tmux"` 아래에 `it2`를 설치하거나 tmux로 전환할 것을 제안하는 설정 프롬프트가 나타납니다.

기본값을 재정의하려면 `~/.claude/settings.json`에서 [`teammateMode`](/docs/ko/settings#available-settings)를 설정합니다:

```json theme={null}
{
  "teammateMode": "auto"
}
```

단일 세션에 대해 모드를 설정하려면 플래그로 전달합니다:

```bash theme={null}
claude --teammate-mode auto
```

분할 창 모드는 [tmux](https://github.com/tmux/tmux/wiki) 또는 [`it2` CLI](https://github.com/mkusaka/it2)가 있는 iTerm2가 필요합니다. 수동으로 설치하려면:

* **tmux**: 시스템의 패키지 관리자를 통해 설치합니다. 플랫폼별 지침은 [tmux wiki](https://github.com/tmux/tmux/wiki/Installing)를 참조합니다.
* **iTerm2**: [`it2` CLI](https://github.com/mkusaka/it2)를 설치한 후, **iTerm2 → Settings → General → Magic → Enable Python API**에서 Python API를 활성화합니다.

<h3 id="specify-teammates-and-models">
  팀원 및 모델 지정
</h3>

Claude는 작업에 따라 생성할 팀원의 수를 결정하거나, 정확히 원하는 것을 지정할 수 있습니다:

```text theme={null}
Spawn 4 teammates to refactor these modules in parallel. Use Sonnet for
each teammate.
```

팀원들은 기본적으로 리더의 `/model` 선택을 상속하지 않습니다. 프롬프트가 지정하지 않을 때 사용되는 모델을 변경하려면, `/config`에서 **기본 팀원 모델**을 설정합니다. \*\*기본값(리더의 모델)\*\*을 선택하여 팀원들이 리더의 현재 모델을 따르도록 합니다.

팀원들은 리더의 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 상속합니다. 분할 창 모드에서는 v2.1.186부터 적용됩니다. 이전 버전은 리더의 세션 노력을 분할 창 팀원에게 전달하지 않았습니다.

<h3 id="require-plan-approval-for-teammates">
  팀원을 위한 계획 승인 요구
</h3>

복잡하거나 위험한 작업의 경우, 팀원들이 구현하기 전에 계획하도록 요구할 수 있습니다. 팀원은 리더가 접근 방식을 승인할 때까지 읽기 전용 계획 모드에서 작동합니다:

```text theme={null}
Spawn an architect teammate to refactor the authentication module.
Require plan approval before they make any changes.
```

팀원이 계획을 마치면, 리더에게 계획 승인 요청을 보냅니다. 리더는 계획을 검토하고 승인하거나 피드백과 함께 거부합니다. 거부되면, 팀원은 계획 모드에 머물러 피드백에 따라 수정하고 다시 제출합니다. 승인되면, 팀원은 계획 모드를 종료하고 구현을 시작합니다.

리더는 자율적으로 승인 결정을 내립니다. 리더의 판단에 영향을 미치려면, 프롬프트에 "테스트 커버리지를 포함하는 계획만 승인" 또는 "데이터베이스 스키마를 수정하는 계획 거부"와 같은 기준을 제공합니다.

<h3 id="talk-to-teammates-directly">
  팀원과 직접 대화하기
</h3>

각 팀원은 완전하고 독립적인 Claude Code 세션입니다. 모든 팀원에게 직접 메시지를 보내 추가 지시를 제공하고, 후속 질문을 하거나, 접근 방식을 재지정할 수 있습니다.

* **In-process 모드**: 에이전트 패널에서 위아래 화살표 키를 사용하여 팀원을 선택한 후 Enter를 눌러 세션을 보고 입력하여 메시지를 보냅니다. 선택된 팀원에서 `x`를 눌러 중지합니다. Ctrl+T를 눌러 작업 목록을 전환합니다.
* **분할 창 모드**: 팀원의 창을 클릭하여 세션과 직접 상호작용합니다. 각 팀원은 자신의 터미널의 전체 보기를 가집니다.

팀원을 보고 있는 동안, 일반 텍스트와 [skills](/docs/ko/skills)는 해당 팀원에게 전달되지만, 기본 제공 명령은 여전히 리더의 세션에서 실행됩니다.

팀원의 모델과 빠른 모드는 생성될 때 고정되므로, `/model`과 `/fast`는 리더의 설정만 변경합니다. v2.1.199부터는 팀원을 보고 있는 동안 두 명령 중 하나를 입력하면 변경이 리더에게 적용된다는 알림이 표시됩니다. 이전 버전은 표시 없이 리더에게 적용했습니다. `/effort`는 여전히 보고 있는 팀원의 이후 턴에 적용됩니다. 팀원들은 리더의 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 따르기 때문입니다.

<h3 id="assign-and-claim-tasks">
  작업 할당 및 요청
</h3>

공유 작업 목록은 팀 전체의 작업을 조율합니다. 리더는 작업을 만들고 팀원들이 이를 처리합니다. 작업은 세 가지 상태를 가집니다: 대기 중, 진행 중, 완료됨. 작업은 다른 작업에 종속될 수도 있습니다: 미해결 종속성이 있는 대기 중인 작업은 해당 종속성이 완료될 때까지 요청할 수 없습니다.

리더는 작업을 명시적으로 할당하거나 팀원들이 자체 요청할 수 있습니다:

* **리더 할당**: 리더에게 어느 작업을 어느 팀원에게 줄지 말합니다
* **자체 요청**: 작업을 마친 후, 팀원은 다음 미할당, 미차단 작업을 자체적으로 선택합니다

작업 요청은 파일 잠금을 사용하여 여러 팀원이 동시에 동일한 작업을 요청하려고 할 때 경합 조건을 방지합니다.

<h3 id="shut-down-teammates">
  팀원 종료하기
</h3>

팀원의 세션을 우아하게 종료하려면, 이름으로 참조합니다. 예를 들어, researcher라는 팀원이 있는 경우:

```text theme={null}
Ask the researcher teammate to shut down
```

리더는 종료 요청을 보냅니다. 팀원은 승인하여 우아하게 종료하거나 설명과 함께 거부할 수 있습니다.

팀의 공유 디렉토리는 세션이 끝날 때 자동으로 정리되므로, 별도의 정리 단계가 없습니다. 어떤 디렉토리가 제거되고 어떤 디렉토리가 재개된 세션을 위해 유지되는지는 [아키텍처](#architecture)를 참조합니다.

<h3 id="enforce-quality-gates-with-hooks">
  hooks로 품질 게이트 적용
</h3>

[hooks](/docs/ko/hooks)를 사용하여 팀원들이 작업을 마치거나 작업이 생성되거나 완료될 때 규칙을 적용합니다:

* [`TeammateIdle`](/docs/ko/hooks#teammateidle): 팀원이 유휴 상태가 되려고 할 때 실행됩니다. 종료 코드 2로 종료하여 피드백을 보내고 팀원을 계속 작동하게 합니다.
* [`TaskCreated`](/docs/ko/hooks#taskcreated): 작업이 생성될 때 실행됩니다. 종료 코드 2로 종료하여 생성을 방지하고 피드백을 보냅니다.
* [`TaskCompleted`](/docs/ko/hooks#taskcompleted): 작업이 완료로 표시될 때 실행됩니다. 종료 코드 2로 종료하여 완료를 방지하고 피드백을 보냅니다.

<h2 id="how-agent-teams-work">
  에이전트 팀이 어떻게 작동하는지
</h2>

이 섹션은 에이전트 팀 뒤의 아키텍처와 메커니즘을 다룹니다. 사용을 시작하려면 위의 [에이전트 팀 제어하기](#control-your-agent-team)를 참조합니다.

<h3 id="how-claude-starts-agent-teams">
  Claude가 에이전트 팀을 시작하는 방법
</h3>

에이전트 팀이 첫 번째 팀원이 생성될 때 형성되며, 메인 세션이 리더 역할을 합니다. 팀원이 생성되는 두 가지 방법이 있습니다:

* **팀원을 요청합니다**: 병렬 작업의 이점이 있는 작업을 제공하고 명시적으로 팀원을 요청합니다. Claude는 지시에 따라 팀원을 생성합니다.
* **Claude가 팀원을 제안합니다**: Claude가 작업이 병렬 작업의 이점이 있다고 판단하면, 팀원 생성을 제안할 수 있습니다. 진행하기 전에 확인합니다.

두 경우 모두 제어 권한을 유지합니다. Claude는 승인 없이 팀원을 생성하지 않습니다.

<h3 id="architecture">
  아키텍처
</h3>

에이전트 팀은 다음으로 구성됩니다:

| 구성 요소     | 역할                                    |
| :-------- | :------------------------------------ |
| **팀 리더**  | 팀원을 생성하고 작업을 조율하는 메인 Claude Code 세션   |
| **팀원들**   | 할당된 작업에서 각각 작동하는 별도의 Claude Code 인스턴스 |
| **작업 목록** | 팀원들이 요청하고 완료하는 공유 작업 항목 목록            |
| **메일박스**  | 에이전트 간 통신을 위한 메시징 시스템                 |

표시 구성 옵션은 [표시 모드 선택](#choose-a-display-mode)을 참조합니다. 팀원 메시지는 리더에게 자동으로 도착합니다.

각 에이전트의 메일박스는 `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`의 JSON 파일입니다. Claude Code는 메일박스 파일을 읽을 때 모든 항목을 검증합니다. 메시지 형식과 일치하지 않는 항목은 오류로 보고되고 파일에서 제거됩니다. 유효한 메시지는 여전히 전달됩니다. v2.1.207 이전에는 단일 형식이 잘못된 메일박스 항목으로 인해 매초 반복되는 오류가 발생했으며 파일을 수동으로 삭제할 때까지 해당 메일박스의 전달이 차단되었습니다.

시스템은 작업 종속성을 자동으로 관리합니다. 팀원이 다른 작업이 종속된 작업을 완료하면, 차단된 작업은 수동 개입 없이 차단 해제됩니다.

팀과 작업은 세션 파생 이름으로 로컬에 저장됩니다. 이름은 `session-` 뒤에 세션 ID의 처음 8자가 붙습니다:

* **팀 구성**: `~/.claude/teams/{team-name}/config.json`
* **작업 목록**: `~/.claude/tasks/{team-name}/`

Claude Code는 세션 시작 시 이 둘을 자동으로 생성하고 팀원들이 참여하거나, 유휴 상태가 되거나, 떠날 때 업데이트합니다. 팀 구성 디렉토리는 세션이 끝날 때 제거됩니다. 작업 목록 디렉토리는 로컬에 유지되며 절대 업로드되지 않으므로, 재개된 세션은 작업을 유지합니다. 보존은 세션 트랜스크립트에 대해 이미 제어하는 것과 동일한 [`cleanupPeriodDays`](/docs/ko/settings#available-settings)에 의해 관리됩니다.

팀 구성은 세션 ID 및 tmux 창 ID와 같은 런타임 상태를 보유하므로, 수동으로 편집하거나 사전 작성하지 마십시오: 다음 상태 업데이트에서 변경 사항이 덮어씌워집니다.

재사용 가능한 팀원 역할을 정의하려면, 대신 [subagent 정의 사용](#use-subagent-definitions-for-teammates)을 사용합니다.

팀 구성에는 각 팀원의 이름, 에이전트 ID, 에이전트 유형이 있는 `members` 배열이 포함됩니다. 팀원들은 이 파일을 읽어 다른 팀 멤버를 발견할 수 있습니다.

프로젝트 수준의 팀 구성 동등물은 없습니다. 프로젝트 디렉토리의 `.claude/teams/teams.json`과 같은 파일은 구성으로 인식되지 않습니다. Claude는 이를 일반 파일로 취급합니다.

<h3 id="use-subagent-definitions-for-teammates">
  팀원을 위해 subagent 정의 사용
</h3>

팀원을 생성할 때, 프로젝트, 사용자, 플러그인, 또는 CLI 정의 등 모든 [subagent 범위](/docs/ko/sub-agents#choose-the-subagent-scope)의 [subagent](/docs/ko/sub-agents) 유형을 참조할 수 있습니다. 이를 통해 보안 검토자 또는 테스트 실행자와 같은 역할을 한 번 정의하고 위임된 subagent와 에이전트 팀 팀원 모두로 재사용할 수 있습니다.

subagent 정의를 사용하려면, Claude에게 팀원을 생성하도록 요청할 때 이름으로 언급합니다:

```text theme={null}
Spawn a teammate using the security-reviewer agent type to audit the auth module.
```

팀원은 해당 정의의 `tools` 허용 목록과 `model`을 준수하며, 정의의 본문은 팀원의 시스템 프롬프트에 추가 지시로 추가되며 이를 대체하지 않습니다. `SendMessage`와 작업 관리 도구와 같은 팀 조율 도구는 `tools`가 다른 도구를 제한할 때에도 팀원이 항상 사용할 수 있습니다.

<Note>
  subagent 정의의 `skills`과 `mcpServers` frontmatter 필드는 해당 정의가 팀원으로 실행될 때 적용되지 않습니다. 팀원들은 일반 세션과 동일하게 프로젝트 및 사용자 설정에서 skills과 MCP servers를 로드합니다.
</Note>

<h3 id="permissions">
  권한
</h3>

팀원들은 리더의 권한 설정으로 시작합니다. 리더가 `--dangerously-skip-permissions`로 실행되면, 모든 팀원도 그렇게 합니다. 생성 후, 개별 팀원 모드를 변경할 수 있지만, 생성 시 팀원별 모드를 설정할 수 없습니다.

한 에이전트가 `SendMessage`를 통해 다른 에이전트에게 메시지를 보낼 때, 수신 에이전트는 사용자가 아닌 다른 Claude 세션에서 온 것으로 알려집니다. 팀원은 권한 프롬프트를 승인하거나 사용자를 대신하여 동의를 제공할 수 없으며, 작업이 거부된 팀원은 이를 다른 팀원에게 전달하여 확인을 우회할 수 없습니다. [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서, 분류기는 다른 에이전트에서 전달된 승인 주장을 사용자로부터의 확인이 아닌 신뢰할 수 없는 입력으로 취급합니다. 팀원 권한 프롬프트는 리더 세션으로 버블업되므로, 거기서 직접 승인합니다. [계획 승인](#require-plan-approval-for-teammates)은 설계된 예외입니다: 리더 세션은 사용자에게 별도의 프롬프트 없이 팀원 계획 승인을 부여합니다.

<h3 id="context-and-communication">
  컨텍스트 및 통신
</h3>

각 팀원은 자신의 컨텍스트 윈도우를 가집니다. 생성될 때, 팀원은 일반 세션과 동일한 프로젝트 컨텍스트를 로드합니다: CLAUDE.md, MCP servers, skills. 또한 리더의 생성 프롬프트를 받습니다. 리더의 대화 기록은 전달되지 않습니다.

**팀원들이 정보를 공유하는 방법:**

* **자동 메시지 전달**: 팀원들이 메시지를 보낼 때, 자동으로 수신자에게 전달됩니다. 리더가 업데이트를 폴링할 필요가 없습니다.
* **유휴 알림**: 팀원이 완료되고 중지되면, 자동으로 리더에게 알립니다. v2.1.198부터, 턴이 API 오류로 끝난 팀원은 정상적으로 완료된 것처럼 보이는 대신 실패했음을 리더에게 알리고 오류 텍스트를 포함합니다.
* **공유 작업 목록**: 모든 에이전트는 작업 상태를 보고 사용 가능한 작업을 요청할 수 있습니다.
* **팀원 메시징**: 이름으로 특정 팀원 한 명에게 메시지를 보냅니다. 모든 사람에게 도달하려면, 각 수신자에게 하나의 메시지를 보냅니다.

리더는 팀원을 생성할 때 각 팀원에게 이름을 할당하며, 모든 팀원은 그 이름으로 다른 팀원에게 메시지를 보낼 수 있습니다. 나중의 프롬프트에서 참조할 수 있는 예측 가능한 이름을 얻으려면, 생성 지시에서 각 팀원을 무엇이라고 부를지 리더에게 말합니다.

<h3 id="token-usage">
  토큰 사용
</h3>

에이전트 팀은 단일 세션보다 훨씬 더 많은 토큰을 사용합니다. 각 팀원은 자신의 컨텍스트 윈도우를 가지며, 토큰 사용은 활성 팀원의 수에 따라 증가합니다. 연구, 검토, 새로운 기능 작업의 경우, 추가 토큰은 일반적으로 가치가 있습니다. 일상적인 작업의 경우, 단일 세션이 더 비용 효율적입니다. 사용 지침은 [에이전트 팀 토큰 비용](/docs/ko/costs#agent-team-token-costs)을 참조합니다.

<h2 id="use-case-examples">
  사용 사례 예시
</h2>

이 예시들은 병렬 탐색이 가치를 더하는 작업을 에이전트 팀이 어떻게 처리하는지 보여줍니다.

<h3 id="run-a-parallel-code-review">
  병렬 코드 검토 실행
</h3>

단일 검토자는 한 번에 한 가지 유형의 문제로 기울어지는 경향이 있습니다. 검토 기준을 독립적인 도메인으로 분할하면 보안, 성능, 테스트 커버리지가 모두 동시에 철저한 주의를 받습니다. 프롬프트는 각 팀원에게 고유한 렌즈를 할당하여 겹치지 않도록 합니다:

```text theme={null}
PR #142를 검토하기 위해 세 명의 팀원을 배치합니다:
- 보안 영향에 중점을 두는 팀원
- 성능 영향을 확인하는 팀원
- 테스트 커버리지를 검증하는 팀원
각각 검토하고 발견 사항을 보고하도록 합니다.
```

각 검토자는 동일한 PR에서 작동하지만 다른 필터를 적용합니다. 리더는 모두 완료된 후 세 명 모두의 발견을 종합합니다.

<h3 id="investigate-with-competing-hypotheses">
  경쟁하는 가설로 조사하기
</h3>

근본 원인이 불명확할 때, 단일 에이전트는 그럴듯한 설명 하나를 찾고 멈추는 경향이 있습니다. 프롬프트는 팀원들을 명시적으로 적대적으로 만들어 이를 방지합니다: 각 팀원의 일은 자신의 이론을 조사하는 것뿐만 아니라 다른 팀원들에게 도전하는 것입니다.

```text theme={null}
사용자들이 앱이 연결 상태를 유지하는 대신 한 메시지 후에 종료된다고 보고합니다.
5명의 에이전트 팀원을 배치하여 다양한 가설을 조사하도록 합니다. 그들이 서로 대화하여
과학적 토론처럼 서로의 이론을 반박하려고 하도록 합니다. 발견 사항 문서를
나타나는 합의로 업데이트합니다.
```

토론 구조가 여기서 핵심 메커니즘입니다. 순차적 조사는 앵커링으로 인해 고통받습니다: 한 이론이 탐색되면, 후속 조사는 그것에 편향됩니다.

여러 독립적인 조사자가 적극적으로 서로의 이론을 반박하려고 할 때, 생존하는 이론은 실제 근본 원인일 가능성이 훨씬 높습니다.

<h2 id="best-practices">
  모범 사례
</h2>

<h3 id="give-teammates-enough-context">
  팀원에게 충분한 컨텍스트 제공
</h3>

팀원들은 CLAUDE.md, MCP servers, skills를 포함한 프로젝트 컨텍스트를 자동으로 로드하지만, 리더의 대화 기록을 상속하지 않습니다. 자세한 내용은 [컨텍스트 및 통신](#context-and-communication)을 참조합니다. 생성 프롬프트에 작업별 세부 사항을 포함합니다:

```text theme={null}
Spawn a security reviewer teammate with the prompt: "Review the authentication module
at src/auth/ for security vulnerabilities. Focus on token handling, session
management, and input validation. The app uses JWT tokens stored in
httpOnly cookies. Report any issues with severity ratings."
```

<h3 id="choose-an-appropriate-team-size">
  적절한 팀 크기 선택
</h3>

팀원의 수에 대한 하드 제한은 없지만, 실질적인 제약이 적용됩니다:

* **토큰 비용이 선형으로 증가**: 각 팀원은 자신의 컨텍스트 윈도우를 가지며 독립적으로 토큰을 소비합니다. 자세한 내용은 [에이전트 팀 토큰 비용](/docs/ko/costs#agent-team-token-costs)을 참조합니다.
* **조율 오버헤드 증가**: 더 많은 팀원은 더 많은 통신, 작업 조율, 충돌 가능성을 의미합니다
* **수익 감소**: 특정 지점을 넘으면, 추가 팀원은 작업 속도를 비례적으로 높이지 않습니다

대부분의 워크플로우에 대해 3-5명의 팀원으로 시작합니다. 이는 병렬 작업과 관리 가능한 조율의 균형을 맞춥니다. 이 가이드의 예시들은 3-5명의 팀원을 사용합니다. 이 범위는 다양한 작업 유형에서 잘 작동하기 때문입니다.

팀원당 5-6개의 [작업](/docs/ko/agent-teams#architecture)을 유지하면 과도한 컨텍스트 전환 없이 모두를 생산적으로 유지합니다. 15개의 독립적인 작업이 있으면, 3명의 팀원이 좋은 시작점입니다.

작업이 실제로 팀원들이 동시에 작동하는 것의 이점이 있을 때만 확장합니다. 세 명의 집중된 팀원은 종종 다섯 명의 산만한 팀원을 능가합니다.

<h3 id="size-tasks-appropriately">
  작업을 적절히 크기 조정
</h3>

* **너무 작음**: 조율 오버헤드가 이점을 초과합니다
* **너무 큼**: 팀원들이 체크인 없이 너무 오래 작동하여 낭비된 노력의 위험이 증가합니다
* **적절함**: 함수, 테스트 파일, 검토와 같은 명확한 결과물을 생성하는 자체 포함된 단위

<Tip>
  리더는 작업을 작업으로 나누고 팀원들에게 자동으로 할당합니다. 충분한 작업을 만들지 않으면, 작업을 더 작은 조각으로 분할하도록 요청합니다. 팀원당 5-6개의 작업을 유지하면 모두를 생산적으로 유지하고 누군가 막히면 리더가 작업을 재할당할 수 있습니다.
</Tip>

<h3 id="wait-for-teammates-to-finish">
  팀원들이 완료될 때까지 기다리기
</h3>

때때로 리더는 팀원들을 기다리지 않고 작업을 자체적으로 구현하기 시작합니다. 이를 알아차리면:

```text theme={null}
Wait for your teammates to complete their tasks before proceeding
```

<h3 id="start-with-research-and-review">
  연구 및 검토로 시작하기
</h3>

에이전트 팀을 처음 사용하는 경우, 명확한 경계가 있고 코드 작성이 필요하지 않은 작업으로 시작합니다: PR 검토, 라이브러리 연구, 또는 버그 조사. 이러한 작업은 병렬 탐색의 가치를 보여주면서 병렬 구현과 함께 오는 조율 문제 없이 보여줍니다.

<h3 id="avoid-file-conflicts">
  파일 충돌 피하기
</h3>

두 팀원이 동일한 파일을 편집하면 덮어쓰기가 발생합니다. 각 팀원이 다른 파일 집합을 소유하도록 작업을 나눕니다.

<h3 id="monitor-and-steer">
  모니터링 및 조율
</h3>

팀원들의 진행 상황을 확인하고, 작동하지 않는 접근 방식을 재지정하며, 발견이 들어올 때 종합합니다. 팀을 무인으로 너무 오래 실행하면 낭비된 노력의 위험이 증가합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="teammates-not-appearing">
  팀원이 나타나지 않음
</h3>

Claude에게 팀원을 생성하도록 요청한 후 팀원이 나타나지 않으면:

* In-process 모드에서, 팀원들이 프롬프트 입력 아래의 에이전트 패널에 나타납니다. 위아래 화살표 키를 사용하여 팀원을 선택한 후 Enter를 눌러 확인합니다.
* 유휴 상태로 앉아 있다가 사라진 팀원 행은 중지된 것이 아니라 숨겨진 것입니다. 유휴 행은 전체 패널이 유휴 상태가 된 후 30초 후에 숨겨지며 팀원의 다음 차례에 다시 나타납니다. 3명 이상의 팀원이 유휴 상태일 때, 초과 행들은 Enter로 확장할 수 있는 단일 `N idle agents` 행으로 축소됩니다. 팀원의 이름으로 메시지를 보내 숨겨진 행을 다시 표시합니다.
* Claude에게 준 작업이 팀을 보증할 만큼 복잡한지 확인합니다. Claude는 작업에 따라 팀원을 생성할지 결정합니다.
* 분할 창을 명시적으로 요청했으면, tmux가 설치되어 있고 PATH에서 사용 가능한지 확인합니다:
  ```bash theme={null}
  which tmux
  ```
* iTerm2의 경우, `it2` CLI가 설치되어 있고 Python API가 iTerm2 환경 설정에서 활성화되어 있는지 확인합니다.

<h3 id="too-many-permission-prompts">
  너무 많은 권한 프롬프트
</h3>

팀원 권한 요청이 리더로 버블업되어 마찰을 일으킬 수 있습니다. 팀원들을 생성하기 전에 [권한 설정](/docs/ko/permissions)에서 일반적인 작업을 사전 승인하여 중단을 줄입니다.

<h3 id="teammates-stopping-on-errors">
  팀원들이 오류에서 중지됨
</h3>

팀원들은 오류를 만난 후 복구하지 않고 중지할 수 있습니다. In-process 모드에서 에이전트 패널에서 팀원을 선택하고 Enter를 누르거나, 분할 모드에서 창을 클릭하여 출력을 확인한 후 다음 중 하나를 수행합니다:

* 직접 추가 지시를 제공합니다
* 작업을 계속하기 위해 대체 팀원을 생성합니다

v2.1.198 기준으로, 리더 또는 다른 팀원의 메시지는 실패한 API 요청을 재시도하기 위해 대기 중인 in-process 팀원을 깨우므로, 전체 재시도 지연을 기다리지 않고 즉시 재시도합니다.

<h3 id="lead-shuts-down-before-work-is-done">
  리더가 작업 완료 전에 종료됨
</h3>

리더는 모든 작업이 실제로 완료되기 전에 팀이 완료되었다고 결정할 수 있습니다. 이 경우 계속하도록 말합니다. 또한 리더가 위임하지 않고 작업을 시작하면 팀원들이 완료될 때까지 기다리도록 말할 수 있습니다.

<h3 id="orphaned-tmux-sessions">
  고아 tmux 세션
</h3>

팀이 끝난 후 tmux 세션이 지속되면, 완전히 정리되지 않았을 수 있습니다. 세션을 나열하고 팀에서 만든 세션을 종료합니다:

```bash theme={null}
tmux ls
tmux kill-session -t <session-name>
```

<h2 id="limitations">
  제한 사항
</h2>

에이전트 팀은 실험적입니다. 주의할 현재 제한 사항:

* **In-process 팀원과의 세션 재개 없음**: `/resume`과 `/rewind`는 in-process 팀원을 복원하지 않습니다. 세션을 재개한 후, 리더는 더 이상 존재하지 않는 팀원에게 메시지를 보내려고 시도할 수 있습니다. 이 경우 리더에게 새 팀원을 생성하도록 말합니다.
* **작업 상태가 지연될 수 있음**: 팀원들이 때때로 작업을 완료로 표시하지 못하여 종속 작업을 차단합니다. 작업이 막혀 있는 것처럼 보이면, 작업이 실제로 완료되었는지 확인하고 작업 상태를 수동으로 업데이트하거나 리더에게 팀원을 밀도록 말합니다.
* **종료가 느릴 수 있음**: 팀원들은 현재 요청이나 도구 호출을 마친 후 종료되어 시간이 걸릴 수 있습니다.
* **세션당 한 팀**: 세션은 정확히 하나의 팀을 가지며, 해당 세션으로 범위가 지정됩니다. 추가 명명된 팀을 만들거나 세션 간에 팀을 공유할 수 없습니다.
* **중첩된 팀 없음**: 팀원들은 자신의 팀원을 생성할 수 없습니다. 리더만 팀을 관리할 수 있습니다.
* **In-process 팀원의 백그라운드 서브에이전트 없음**: in-process 팀원의 자체 서브에이전트는 포그라운드에서 실행됩니다. `run_in_background`를 사용하거나 `background: true`를 설정하는 서브에이전트 정의로 백그라운드 서브에이전트를 요청하면 오류가 반환됩니다. 팀원의 백그라운드 작업은 리더의 프로세스보다 오래 지속될 수 없기 때문입니다. 주 대화에서 시작된 서브에이전트는 [백그라운드 기본값](/docs/ko/sub-agents#run-subagents-in-foreground-or-background)을 따릅니다.
* **리더가 고정됨**: 주 세션은 수명 동안 리더입니다. 팀원을 리더로 승격하거나 리더십을 이전할 수 없습니다.
* **생성 시 권한 설정**: 모든 팀원은 리더의 권한 모드로 시작합니다. 생성 후 개별 팀원 모드를 변경할 수 있지만, 생성 시 팀원별 모드를 설정할 수 없습니다.
* **분할 창은 tmux 또는 iTerm2 필요**: 기본 in-process 모드는 모든 터미널에서 작동합니다. 분할 창 모드는 VS Code의 통합 터미널, Windows Terminal, Ghostty에서 지원되지 않습니다.

<Tip>
  **`CLAUDE.md`는 정상적으로 작동합니다**: 팀원들은 작업 디렉토리에서 `CLAUDE.md` 파일을 읽습니다. 이를 사용하여 모든 팀원에게 프로젝트별 지침을 제공합니다.
</Tip>

<h2 id="next-steps">
  다음 단계
</h2>

병렬 작업 및 위임을 위한 관련 접근 방식을 탐색합니다:

* **경량 위임**: [subagents](/docs/ko/sub-agents)는 세션 내에서 연구 또는 검증을 위해 도우미 에이전트를 생성하며, 에이전트 간 조율이 필요하지 않은 작업에 더 좋습니다
* **수동 병렬 세션**: [Git worktrees](/docs/ko/worktrees)를 사용하면 자동화된 팀 조율 없이 여러 Claude Code 세션을 직접 실행할 수 있습니다
* **접근 방식 비교**: [subagent vs 에이전트 팀](/docs/ko/features-overview#compare-similar-features) 비교를 참조하여 나란히 비교합니다
