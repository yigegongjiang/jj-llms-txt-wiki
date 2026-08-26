> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 동적 워크플로우로 대규모 서브에이전트 조율하기

> 동적 워크플로우는 Claude가 작성한 스크립트에서 많은 서브에이전트를 조율하며, 이를 다시 실행할 수 있습니다. 코드베이스 감사, 대규모 마이그레이션, 교차 검증 연구에 사용합니다.

<Note>
  동적 워크플로우는 Claude Code v2.1.154 이상이 필요하며 모든 유료 요금제, Anthropic API 액세스, Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry에서 사용 가능합니다. Pro에서는 `/config`의 동적 워크플로우 행에서 활성화하십시오.
</Note>

동적 워크플로우는 대규모로 [서브에이전트](/docs/ko/sub-agents)를 조율하는 JavaScript 스크립트입니다. Claude는 설명한 작업에 대한 스크립트를 작성하고, 런타임이 백그라운드에서 실행하는 동안 세션은 응답성을 유지합니다.

한 번의 대화로 조율할 수 있는 것보다 더 많은 에이전트가 필요한 작업이거나, 읽고 다시 실행할 수 있는 스크립트로 조율을 코드화하려는 경우 워크플로우를 사용하십시오. 예시로는 코드베이스 전체 버그 스윕, 500개 파일 마이그레이션, 서로에 대해 소스를 교차 검증해야 하는 연구 질문, 하나에 커밋하기 전에 여러 독립적인 각도에서 작성할 가치가 있는 어려운 계획이 있습니다.

<h2 id="when-to-use-a-workflow">
  워크플로우를 언제 사용할지
</h2>

[서브에이전트](/docs/ko/sub-agents), [스킬](/docs/ko/skills), [에이전트 팀](/docs/ko/agent-teams), 그리고 워크플로우는 모두 다단계 작업을 실행할 수 있습니다. 차이점은 계획을 누가 보유하는지입니다:

|                      | 서브에이전트           | 스킬                | 에이전트 팀              | 워크플로우                 |
| :------------------- | :--------------- | :---------------- | :------------------ | :-------------------- |
| 정의                   | Claude가 생성하는 워커  | Claude가 따르는 지침    | 피어 세션을 감독하는 리드 에이전트 | 런타임이 실행하는 스크립트        |
| 다음에 무엇을 실행할지 결정하는 사람 | Claude, 차례대로     | Claude, 프롬프트를 따르며 | 리드 에이전트, 차례대로       | 스크립트                  |
| 중간 결과가 있는 위치         | Claude의 컨텍스트 윈도우 | Claude의 컨텍스트 윈도우  | 공유 작업 목록            | 스크립트 변수               |
| 반복 가능한 것             | 워커 정의            | 지침                | 팀 정의                | 조율 자체                 |
| 규모                   | 차례당 몇 가지 위임된 작업  | 서브에이전트와 동일        | 소수의 장기 실행 피어        | 실행당 수십 개에서 수백 개의 에이전트 |
| 중단                   | 차례 다시 시작         | 차례 다시 시작          | 팀원들이 계속 실행          | 같은 세션에서 재개 가능         |

워크플로우는 계획을 코드로 이동합니다. 서브에이전트, 스킬, 에이전트 팀을 사용하면 Claude가 조율자입니다: 차례대로 다음에 무엇을 생성할지 또는 할당할지 결정하고, 모든 결과는 컨텍스트 윈도우에 들어갑니다. 워크플로우 스크립트는 루프, 분기, 중간 결과를 자체적으로 보유하므로 Claude의 컨텍스트는 최종 답변만 보유합니다.

계획을 코드로 이동하면 워크플로우가 반복 가능한 품질 패턴을 적용할 수 있으며, 단순히 더 많은 에이전트를 실행하는 것이 아닙니다: 독립적인 에이전트가 서로의 발견을 적대적으로 검토한 후 보고하거나, 여러 각도에서 계획을 작성하고 서로 비교하여 단일 패스보다 더 신뢰할 수 있는 결과를 얻을 수 있습니다.

<h2 id="run-a-bundled-workflow">
  번들된 워크플로우 실행하기
</h2>

워크플로우가 실제로 작동하는 것을 보는 가장 빠른 방법은 `/deep-research`를 실행하는 것입니다. 이는 Claude Code가 포함하는 [내장 워크플로우](#bundled-workflows)로 많은 소스에서 질문을 조사합니다. 세션이 자유로운 상태에서 에이전트가 백그라운드에서 일련의 단계를 진행하는 것을 보고, 차례별 기록 대신 하나의 보고서를 얻습니다.

<Steps>
  <Step title="워크플로우 실행">
    조사하려는 질문과 함께 `/deep-research`를 실행합니다. 여러 각도에서 웹 검색을 확산시키고, 찾은 소스를 가져와 교차 검증하며, 인용된 보고서를 종합합니다.

    ```text theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="워크플로우 허용">
    Claude Code가 워크플로우를 허용할지 묻습니다. **예**를 선택하여 계속합니다. 정확한 프롬프트는 권한 모드에 따라 다릅니다. [실행 전 계획 승인하기](#approve-the-plan-before-it-runs)에서 모드별 옵션을 참조하십시오.
  </Step>

  <Step title="진행 상황 보기">
    실행이 백그라운드에서 시작됩니다. `/workflows`를 실행하고, 화살표 키를 사용하여 실행을 선택한 후 Enter를 눌러 진행 상황 보기를 엽니다:

    ```text theme={null}
    /workflows
    ```

    보기는 각 단계를 에이전트 수, 토큰 총계, 경과 시간과 함께 표시합니다. 모든 단계를 드릴다운하여 해당 에이전트와 각각이 찾은 것을 확인합니다. 전체 제어 집합은 [실행 보기](#watch-the-run)를 참조하십시오.

    입력 상자 아래의 작업 패널에서도 볼 수 있습니다: 실행이 진행되는 동안 한 줄의 진행 상황 요약이 나타납니다. 아래쪽 화살표를 눌러 포커스하고, Enter를 눌러 확장합니다.
  </Step>

  <Step title="보고서 읽기">
    실행이 완료되면 보고서가 세션에 들어갑니다. 각 주장이 나온 소스를 인용하며, 교차 검증을 통과하지 못한 주장은 이미 필터링되어 있습니다.

    v2.1.196부터 검증자 에이전트가 속도 제한이나 API 오류 이후와 같이 주장을 확인할 수 없을 때, 보고서는 그 주장을 반박된 것으로 계산하는 대신 검증되지 않은 것으로 나열합니다.
  </Step>
</Steps>

자신의 작업을 위해 워크플로우를 실행하려면 [Claude가 워크플로우를 작성하도록 하고](#have-claude-write-a-workflow), 실행이 원하는 작업을 수행하면 [재사용을 위해 저장](#save-the-workflow-for-reuse)할 수 있습니다.

<h3 id="bundled-workflows">
  번들된 워크플로우
</h3>

Claude Code는 `/deep-research`를 내장 워크플로우로 포함합니다:

| 명령                          | 수행 작업                                                                                                                                                                        |
| :-------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | 여러 각도에서 질문에 대한 웹 검색을 확산시키고, 찾은 소스를 가져와 교차 검증하며, 각 주장에 투표하고, 교차 검증을 통과하지 못한 주장이 필터링된 인용된 보고서를 반환합니다. [WebSearch 도구](/docs/ko/tools-reference#websearch-tool-behavior)를 사용 가능해야 합니다 |

[직접 저장한](#save-the-workflow-for-reuse) 워크플로우는 같은 방식으로 명령이 되며 번들된 것들과 함께 `/` 자동완성에 나타납니다.

<h3 id="watch-the-run">
  실행 보기
</h3>

워크플로우는 백그라운드에서 실행되므로 에이전트가 작동하는 동안 세션은 응답성을 유지합니다. 언제든지 `/workflows`를 실행하여 실행 중이거나 완료된 워크플로우를 나열한 후 하나를 선택하여 진행 상황 보기를 엽니다.

```text theme={null}
/workflows
```

진행 상황 보기는 각 단계를 에이전트 수, 토큰 총계, 경과 시간과 함께 표시합니다. 바닥글은 각 작업의 키를 나열합니다:

| 키              | 작업                                                                                   |
| :------------- | :----------------------------------------------------------------------------------- |
| `↑` / `↓`      | 단계 또는 에이전트 선택                                                                        |
| `Enter` 또는 `→` | 선택한 단계로 드릴다운한 후 에이전트로 드릴다운하여 프롬프트, 최근 도구 호출, 결과 읽기                                   |
| `Esc` 또는 `←`   | 한 수준 뒤로. v2.1.203부터 v2.1.205까지는 `←`가 단계나 에이전트에서 뒤로 나가지 않았습니다. 해당 버전에서는 `Esc`를 사용하십시오 |
| `j` / `k`      | 에이전트 세부 정보가 오버플로우할 때 스크롤                                                             |
| `f`            | 선택한 단계의 에이전트 목록을 상태별로 필터링합니다. 다시 누르면 순환합니다                                           |
| `p`            | 실행 일시 중지 또는 재개                                                                       |
| `x`            | 선택한 에이전트 중지 또는 포커스가 실행에 있을 때 전체 워크플로우 중지                                             |
| `r`            | 선택한 실행 중인 에이전트 다시 시작                                                                 |
| `s`            | 실행의 스크립트를 명령으로 [저장](#save-the-workflow-for-reuse)                                    |

<h2 id="have-claude-write-a-workflow">
  Claude가 워크플로우를 작성하도록 하기
</h2>

두 가지 방법으로 Claude가 작업을 위한 워크플로우를 작성하도록 할 수 있습니다:

* [프롬프트에서 워크플로우 요청하기](#ask-for-a-workflow-in-your-prompt) 프롬프트에 `ultracode` 키워드를 포함하거나 자신의 말로 요청하면 Claude가 작업을 위한 워크플로우를 작성합니다.
* [ultracode로 Claude가 결정하도록 하기](#let-claude-decide-with-ultracode): `/effort ultracode`를 설정하면 Claude가 세션의 모든 실질적인 작업을 위한 워크플로우를 계획합니다.

이미 존재하는 워크플로우 명령을 실행할 수도 있습니다: `/deep-research`와 같은 [번들된 워크플로우](#bundled-workflows) 또는 [저장한](#save-the-workflow-for-reuse) 것입니다.

<h3 id="ask-for-a-workflow-in-your-prompt">
  프롬프트에서 워크플로우 요청하기
</h3>

세션의 노력 수준을 변경하지 않고 단일 작업을 워크플로우로 실행하려면 프롬프트에 `ultracode` 키워드를 포함하십시오. 자신의 말로 요청하기(예: "워크플로우 사용" 또는 "워크플로우 실행")도 작동합니다: Claude는 직접 요청을 동일한 옵트인으로 취급합니다. v2.1.160 이전에는 리터럴 트리거 키워드가 `workflow`였습니다; 자연어 요청은 두 버전 모두에서 작동합니다.

```text theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code는 입력에서 키워드를 강조하고 Claude는 차례대로 작업하는 대신 작업을 위한 워크플로우 스크립트를 작성합니다. 의도하지 않은 워크플로우를 시작한 경우 macOS에서 `Option+W` 또는 Windows와 Linux에서 `Alt+W`를 누르면 이 프롬프트에서 강조를 무시하거나, 강조된 키워드 바로 뒤에 커서가 있을 때 백스페이스를 누르십시오. 키워드가 전혀 트리거되지 않도록 하려면 `/config`에서 Ultracode keyword trigger를 끄십시오.

실행이 원하는 작업을 수행하면 나중에 [명령으로 저장](#save-the-workflow-for-reuse)할 수 있습니다.

이미 다른 방식으로 구축한 오케스트레이터가 있는 경우, 예를 들어 서브에이전트 프롬프트 폴더 또는 작업을 분산하는 스킬이 있다면 Claude에 이를 지시하고 같은 작업을 수행하는 워크플로우를 요청할 수 있습니다.

<h3 id="let-claude-decide-with-ultracode">
  ultracode로 Claude가 결정하도록 하기
</h3>

Ultracode는 `xhigh` [추론 노력](/docs/ko/model-config#adjust-effort-level)을 자동 워크플로우 조율과 결합하는 Claude Code 설정입니다. 활성화하면 Claude는 요청을 기다리는 대신 각 실질적인 작업을 위한 워크플로우를 계획합니다.

```text theme={null}
/effort ultracode
```

ultracode가 이미 활성화된 상태로 세션을 시작하려면 `claude --effort ultracode`로 실행하십시오. Claude Code v2.1.203 이상이 필요합니다.

ultracode가 활성화되면 Claude는 작업이 워크플로우를 보증하는지 결정합니다. 단일 요청은 여러 워크플로우로 변할 수 있습니다: 코드를 이해하기 위한 하나, 변경을 수행하기 위한 하나, 검증하기 위한 하나입니다. 이는 세션의 모든 작업에 적용되므로 각 요청은 더 낮은 노력 수준보다 더 많은 토큰을 사용하고 더 오래 걸립니다.

Ultracode는 현재 세션 동안 지속되며 새 세션을 시작할 때 재설정됩니다. 일상적인 작업으로 돌아갈 때 `/effort high`로 내려갑니다. `xhigh` [노력](/docs/ko/model-config#adjust-effort-level)을 지원하는 모델에서 사용 가능합니다; 다른 모델에서는 `/effort` 메뉴가 이를 제공하지 않습니다.

<h3 id="approve-the-plan-before-it-runs">
  실행 전 계획 승인하기
</h3>

CLI에서 실행별 프롬프트는 계획된 단계와 다음 옵션을 표시합니다:

* **예, 실행**: 실행 시작
* **예, `<path>`의 `<name>`에 대해 다시 묻지 않기**: 시작하고 이 프로젝트에서 이 워크플로우에 대해 이 프롬프트를 건너뜁니다
* **원본 스크립트 보기**: 결정하기 전에 스크립트 읽기
* **아니오**: 취소

`Ctrl+G`는 편집기에서 스크립트를 엽니다. `Tab`을 사용하면 실행 시작 전에 프롬프트를 조정할 수 있습니다.

이 프롬프트를 보는지 여부는 [권한 모드](/docs/ko/permission-modes)에 따라 다릅니다:

| 권한 모드                         | 프롬프트가 표시되는 경우                                                                           |
| :---------------------------- | :-------------------------------------------------------------------------------------- |
| 기본값, 편집 수락                    | 해당 워크플로우에 대해 **예, 다시 묻지 않기**를 선택하지 않은 한 모든 실행                                           |
| 자동                            | 첫 번째 실행만. 모든 **예**는 사용자 설정에 동의를 기록하고, 나중의 실행은 프롬프트 없이 시작합니다. ultracode가 활성화되면 완전히 건너뜁니다 |
| 권한 무시, `claude -p`, Agent SDK | 절대 안 됨. 실행이 즉시 시작됩니다                                                                    |

Desktop 앱에서 승인 카드는 워크플로우 이름, 단계 목록, 토큰 사용 주의와 함께 **한 번**, **항상**, **거부** 작업을 표시합니다. 진행 상황 보기는 Background tasks 사이드 패널에 나타납니다.

권한 모드는 위의 실행 프롬프트만 제어합니다. 워크플로우가 생성하는 서브에이전트는 항상 `acceptEdits` 모드에서 실행되고 세션의 모드와 관계없이 [도구 허용 목록](/docs/ko/settings#permission-settings)을 상속합니다. 파일 편집은 자동 승인됩니다.

허용 목록에 없는 셸 명령, 웹 페치, MCP 도구는 여전히 실행 중에 프롬프트할 수 있습니다. 긴 실행에서 이를 피하려면 에이전트가 필요로 하는 명령을 시작 전에 허용 목록에 추가하십시오.

`claude -p` 및 Agent SDK에서는 프롬프트할 사람이 없으므로 도구 호출은 대화형 확인 없이 구성된 권한 규칙을 따릅니다.

<h3 id="save-the-workflow-for-reuse">
  재사용을 위해 워크플로우 저장하기
</h3>

Claude가 반복할 작업을 위한 워크플로우를 작성하면 해당 실행의 스크립트를 명령으로 저장할 수 있습니다. 모든 분기에서 실행하는 검토와 같은 프로세스는 매번 같은 조율을 실행합니다.

`/workflows`를 실행하고, 유지하려는 실행을 선택한 후 `s`를 누릅니다. 저장 대화에서 Tab은 두 저장 위치 사이를 전환합니다:

* `.claude/workflows/` 프로젝트에서: 저장소를 복제하는 모든 사람과 공유
* `~/.claude/workflows/` 홈 디렉토리에서: 모든 프로젝트에서 사용 가능, 자신에게만 표시됩니다. [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)을 설정한 경우 이 위치는 해당 경로 아래의 `workflows/` 디렉토리입니다.

저장 대화는 개인 위치에 대해 확인된 경로를 표시합니다. v2.1.208 이전에는 `CLAUDE_CONFIG_DIR`이 설정된 경우에도 `~/.claude/workflows/`를 표시했습니다; 파일은 여전히 구성된 디렉토리 아래에 저장되었습니다.

Enter를 눌러 저장합니다. 워크플로우는 이후 세션에서 두 위치 중 하나에서 `/<name>`으로 실행됩니다.

v2.1.178부터 프로젝트 위치에 저장하면 작업 디렉토리와 저장소 루트 사이에 이미 존재하는 가장 가까운 `.claude/workflows/` 디렉토리에 쓰거나, 아직 존재하지 않으면 저장소 루트에 씁니다. 프로젝트 워크플로우는 또한 해당 경로를 따라 모든 `.claude/workflows/`에서 로드되며, 둘 이상이 같은 이름을 정의할 때 Claude Code는 작업 디렉토리에 가장 가까운 것을 실행합니다.

프로젝트 워크플로우와 개인 워크플로우가 이름을 공유하면 프로젝트 워크플로우가 실행됩니다.

<h3 id="pass-input-to-a-saved-workflow">
  저장된 워크플로우에 입력 전달하기
</h3>

저장된 워크플로우는 `args` 매개변수를 통해 입력을 받을 수 있습니다. 스크립트는 이를 `args`라는 전역 변수로 읽습니다. 이를 사용하여 연구 질문, 대상 경로 목록, 또는 구성 객체를 실행 시간에 제공하면 각 실행마다 스크립트를 편집할 필요가 없습니다.

다음 프롬프트는 문제 번호 목록으로 저장된 워크플로우를 실행합니다:

```text theme={null}
> Run /triage-issues on issues 1024, 1025, and 1030
```

Claude는 목록을 구조화된 데이터로 전달하므로 스크립트는 먼저 구문 분석하지 않고도 `args`에서 배열 및 객체 메서드를 직접 호출할 수 있습니다. `args`가 생략되면 스크립트 내부의 전역 변수는 `undefined`입니다.

<h2 id="example-workflow-prompts">
  예제 워크플로우 프롬프트
</h2>

워크플로우는 작업이 한 에이전트가 컨텍스트에 보유할 수 있는 것보다 크거나, 같은 단계를 많은 항목에 걸쳐 실행해야 할 때 가장 적합합니다. 아래 프롬프트는 일반적인 형태를 보여줍니다. 각각은 Claude에게 해당 작업을 위한 워크플로우를 작성하고 실행하도록 요청합니다; 스크립트를 직접 작성하지 않습니다.

<h3 id="audit-many-files-for-the-same-issue">
  많은 파일을 같은 문제에 대해 감사하기
</h3>

파일당 하나의 에이전트를 확산시킨 후 발견 사항을 수집하고 검증합니다.

```text theme={null}
> use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  검사가 통과할 때까지 계속 수정하기
</h3>

검사기를 실행하고, 실패한 것을 수정하며, 통과하거나 진행이 멈출 때까지 반복합니다.

```text theme={null}
> use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  많은 파일을 병렬로 마이그레이션하기
</h3>

마이그레이션할 파일을 발견하고, 편집이 충돌하지 않도록 각각을 격리된 복사본에서 변환하며, 각 결과를 검증합니다.

```text theme={null}
> use a workflow to migrate every component under src/components/ from styled-components to Tailwind, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  모든 변경된 파일을 검토하고 하나의 요약 작성하기
</h3>

파일당 하나의 검토자를 실행한 후 모든 발견 사항을 하나의 에이전트에 전달하여 순위를 매기고 중복을 제거합니다.

```text theme={null}
> use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  많은 소스에서 주제 연구하기
</h3>

변경 로그, 문제, 문서에 걸쳐 읽기를 확산시킨 후 종합합니다. 번들된 `/deep-research` 워크플로우가 이를 수행합니다; 더 좁은 버전을 설명할 수도 있습니다.

```text theme={null}
> use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  목록이 더 이상 증가할 때까지 문제 찾기
</h3>

라운드에서 계속 검색하고 새 라운드가 새로운 것을 찾지 못하면 중지합니다.

```text theme={null}
> use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

<h3 id="what-the-saved-script-looks-like">
  저장된 스크립트가 어떻게 보이는지
</h3>

[워크플로우를 저장](#save-the-workflow-for-reuse)하면 `.claude/workflows/`의 파일은 `meta` 블록 다음에 서브에이전트를 조율하는 스크립트 본문을 보유합니다. 일반적으로 편집할 필요가 없지만, 여기는 Claude가 생성한 것을 인식할 수 있도록 작은 것의 형태입니다:

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

본문은 최상위 `await`를 포함한 순수 JavaScript입니다. `agent()`는 하나의 서브에이전트를 생성하고 `pipeline()`은 목록의 각 항목당 하나를 실행합니다. 스크립트를 직접 편집하려면 Claude에게 변경 사항을 안내해 달라고 요청하거나, [Agent SDK 참조](/docs/ko/agent-sdk/typescript)의 Workflow 도구 항목에서 전체 옵션 집합을 참조하십시오.

<h2 id="how-a-workflow-runs">
  워크플로우가 어떻게 실행되는지
</h2>

워크플로우 런타임은 대화와 분리된 격리된 환경에서 스크립트를 실행합니다. 중간 결과는 Claude의 컨텍스트에 들어가는 대신 스크립트 변수에 남아 있습니다.

모든 실행은 `~/.claude/projects/` 아래의 세션 디렉토리에 스크립트를 작성합니다. Claude는 실행이 시작될 때 경로를 받으므로 요청할 수 있습니다. 해당 파일을 열어 Claude가 작성한 오케스트레이션을 읽거나, 이전 실행의 스크립트와 비교하거나, 편집한 후 Claude에 편집된 버전에서 다시 시작하도록 요청할 수 있습니다.

런타임은 실행이 진행되면서 각 에이전트의 결과를 추적하며, 이것이 실행을 [같은 세션 내에서 재개 가능](#resume-after-a-pause)하게 만드는 것입니다.

<h3 id="behavior-and-limits">
  동작 및 제한
</h3>

런타임은 다음 제약을 적용합니다:

| 제약                                      | 이유                                                                  |
| :-------------------------------------- | :------------------------------------------------------------------ |
| 실행 중 사용자 입력 없음                          | 에이전트 권한 프롬프트만 실행을 일시 중지할 수 있습니다. 단계 간 승인을 위해 각 단계를 자신의 워크플로우로 실행합니다 |
| 워크플로우 자체에서 직접 파일 시스템 또는 셸 액세스 없음        | 에이전트가 읽고, 쓰고, 명령을 실행합니다. 스크립트가 에이전트를 조율합니다                          |
| 최대 16개의 동시 에이전트, CPU 코어가 제한된 머신에서는 더 적음 | 로컬 리소스 사용 제한                                                        |
| 실행당 총 1,000개의 에이전트                      | 폭주 루프 방지                                                            |

<h2 id="manage-runs">
  실행 관리
</h2>

실행이 시작되면 `/workflows` 보기에서 또는 입력 상자 아래의 작업 패널에서 진행 상황 줄을 확장하여 관리합니다.

<h3 id="resume-after-a-pause">
  일시 중지 후 재개
</h3>

실행을 중지하면 재개할 수 있습니다: 이미 완료된 에이전트는 캐시된 결과를 반환하고 나머지는 라이브로 실행됩니다. `/workflows`에서 일시 중지된 실행을 재개하려면 선택하고 `p`를 누르거나, Claude에게 같은 스크립트로 워크플로우를 다시 시작하도록 요청합니다.

재개는 같은 Claude Code 세션 내에서 작동합니다. 워크플로우가 실행 중인 동안 Claude Code를 종료하면 다음 세션은 워크플로우를 새로 시작합니다.

<h3 id="cost">
  비용
</h3>

워크플로우는 많은 에이전트를 생성하므로 단일 실행은 대화에서 같은 작업을 수행하는 것보다 의미 있게 더 많은 토큰을 사용할 수 있습니다. 실행은 다른 세션과 마찬가지로 요금제의 사용량 및 속도 제한에 포함됩니다.

대규모 작업에 커밋하기 전에 지출을 측정하려면 먼저 작은 부분에서 워크플로우를 실행합니다: 전체 저장소 대신 한 디렉토리, 또는 광범위한 질문 대신 좁은 질문입니다. `/workflows` 보기는 실행이 진행되면서 각 에이전트의 토큰 사용량을 표시하며, 완료된 작업을 잃지 않고 언제든지 실행을 중지할 수 있습니다. 런타임의 [에이전트 상한](#behavior-and-limits)은 단일 실행이 생성할 수 있는 에이전트 수를 제한하여 폭주 스크립트의 비용을 제한합니다. 모든 실행을 기본적으로 더 작게 유지하려면 `/config`에서 [크기 지침을 설정](#set-a-size-guideline)합니다.

Claude Code는 또한 비정상적으로 커지는 실행에 플래그를 지정합니다. 워크플로우가 25개 이상의 에이전트를 예약하거나 예상 토큰 총계가 150만을 초과하면 입력 상자 아래의 작업 패널에서 진행 상황 줄에 `Large workflow` 경고가 표시됩니다. 경고는 실행을 중지할 수 있는 [`/workflows`](#watch-the-run)를 가리킵니다. Claude Code v2.1.203 이상이 필요합니다.

경고는 권고사항입니다: 실행을 일시 중지하거나 제한하지 않습니다. 경고를 볼 때 두 가지 설정이 변경됩니다:

* [크기 지침을 설정](#set-a-size-guideline)하면 지침의 에이전트 수가 25개 에이전트 임계값을 대체합니다.
* [ultracode](#let-claude-decide-with-ultracode)가 켜진 세션은 경고를 표시하지 않습니다. ultracode를 켜는 것이 이미 대규모 실행에 동의했기 때문입니다.

워크플로우의 모든 에이전트는 스크립트가 단계를 다른 것으로 라우팅하지 않는 한 세션의 모델을 사용합니다. 또는 [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/ko/model-config#environment-variables) 환경 변수가 설정되어 있으면 둘 다를 무시합니다. 모델 비용을 제어하려면:

* 일상적인 작업을 위해 일반적으로 더 작은 모델로 전환하는 경우 대규모 실행 전에 `/model`을 확인합니다
* 작업을 설명할 때 Claude에게 가장 강력한 것이 필요하지 않은 단계에 더 작은 모델을 사용하도록 요청합니다

<h3 id="set-a-size-guideline">
  크기 지침 설정
</h3>

`/config`의 동적 워크플로우 크기 설정은 Claude가 작성하는 워크플로우를 기본적으로 더 작은 규모로 유지합니다. Claude Code는 설정을 Claude에 조언으로 보내므로 다른 규모를 요구하는 프롬프트는 여전히 이를 무시할 수 있습니다. Claude Code v2.1.202 이상이 필요합니다.

각 값은 Claude가 작성하는 스크립트에서 목표로 하는 에이전트 수를 설정합니다.

| 값              | Claude에 전송된 지침         |
| :------------- | :--------------------- |
| `unrestricted` | 지침 없음. 이것이 기본값입니다.     |
| `small`        | 5개 미만의 에이전트를 목표로 합니다.  |
| `medium`       | 15개 미만의 에이전트를 목표로 합니다. |
| `large`        | 50개 미만의 에이전트를 목표로 합니다. |

변경 사항은 다음 프롬프트에서 적용됩니다. [런타임 에이전트 상한](#behavior-and-limits)은 설정에 관계없이 여전히 적용됩니다.

<h3 id="turn-workflows-off">
  워크플로우 끄기
</h3>

워크플로우는 CLI, Desktop 앱, IDE 확장, [비대화형 모드](/docs/ko/headless)에서 `claude -p`로, [Agent SDK](/docs/ko/agent-sdk/overview)에서 사용 가능합니다. 같은 비활성화 설정이 모든 표면에 적용됩니다.

자신을 위해 워크플로우를 끄려면:

* `/config`에서 동적 워크플로우를 끕니다. 세션 간에 지속됩니다.
* `~/.claude/settings.json`에서 `"disableWorkflows": true`를 설정합니다. 세션 간에 지속됩니다.
* `CLAUDE_CODE_DISABLE_WORKFLOWS=1`을 설정합니다. 시작 시 읽으므로 설정한 곳 어디든 적용됩니다.

전체 조직을 위해 워크플로우를 끄려면 [관리 설정](/docs/ko/server-managed-settings)에서 `"disableWorkflows": true`를 설정하거나 [Claude Code 관리 설정](https://claude.ai/admin-settings/claude-code) 페이지의 토글을 사용합니다.

워크플로우가 비활성화되면 번들된 워크플로우 명령을 사용할 수 없고, `ultracode` 키워드는 더 이상 실행을 트리거하지 않으며, `ultracode`는 `/effort` 메뉴에서 제거됩니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [에이전트를 병렬로 실행](/docs/ko/agents): 서브에이전트, 에이전트 보기, 에이전트 팀, 워크플로우 비교
* [사용자 정의 서브에이전트 만들기](/docs/ko/sub-agents): 워크플로우가 조율하는 워커 기본 요소
* [비용 관리](/docs/ko/costs): 다중 에이전트 실행이 사용량 제한에 어떻게 포함되는지
