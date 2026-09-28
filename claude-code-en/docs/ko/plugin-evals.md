> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# evals로 플러그인 테스트하기

> Claude Code 플러그인에 대한 eval 케이스를 작성하고, claude plugin eval로 실행하며, 결과를 채점하고, 플러그인 없는 기준선과 비교하고, CI에서 점수를 기준으로 게이트합니다.

`claude plugin eval` 셸 명령은 [플러그인](/docs/ko/plugins/overview)을 테스트 케이스 모음에 대해 실행하고 결과를 채점합니다. 각 케이스는 현실적인 프롬프트와 하나 이상의 채점자로 구성됩니다. 채점자는 Claude가 생성한 내용에 대한 통과/실패 확인입니다. 예를 들어 응답에 대한 정규식, 특정 도구가 호출되었는지 여부, 또는 두 번째 모델이 응답을 판단하는 루브릭입니다.

모음을 직접 작성할 필요는 없습니다. `claude plugin eval init`은 플러그인에 대해 질문하고, 케이스와 채점자를 제안하고, 시도하고, 파일을 작성합니다. 이미 열려 있는 세션에서 Claude에게 동일한 작업을 수행하도록 요청할 수도 있습니다.

evals를 사용하여:

* 플러그인이 Claude를 올바른 결과로 얼마나 안정적으로 유도하는지 측정합니다
* 플러그인을 변경하거나 새로운 모델이 출시될 때 회귀를 포착합니다
* 플러그인이 플러그인 없는 경우와 비교하여 무엇을 기여하는지 확인합니다

이 페이지는 작동하는 플러그인을 가지고 있고 그 동작을 테스트하려는 플러그인 및 스킬 작성자와 CI에서 플러그인 변경을 게이트하는 팀을 위한 것입니다. 케이스 형식은 [스킬 생성자 플러그인](/docs/ko/skills#run-evals-with-skill-creator)이 사용하는 `evals/evals.json` 파일과 별개입니다. 플러그인을 만들려면 [플러그인 만들기](/docs/ko/plugins/create)를 참조하고, 플러그인의 동작이 아닌 구문 및 스키마 오류를 확인하려면 [`claude plugin validate`](/docs/ko/plugins/cli-reference#plugin-validate)를 사용합니다.

<Note>
  모든 eval 실행과 모든 판사 채점자는 계정에 대한 실제 모델 호출이며, 플랜의 사용량 또는 API 청구에 계산되므로 먼저 [요구사항](#requirements)을 확인합니다. 그런 다음 [첫 번째 eval 모음 만들기](#create-your-first-eval-suite)를 진행하거나, 이미 있는 경우 [CI에서 evals 실행](#run-evals-in-ci)으로 이동합니다.
</Note>

<h2 id="requirements">
  요구사항
</h2>

플러그인 evals를 실행하려면 다음이 필요합니다.

* Claude Code v2.1.269 이상. `claude --version`으로 확인하고 `claude update`로 업그레이드합니다.
* `plugin.json` 또는 `.claude-plugin/plugin.json` 매니페스트가 있는 플러그인 디렉토리, 또는 [스킬 디렉토리 플러그인](/docs/ko/plugins/loading#plugins-shared-through-a-repository).
* 일반적인 Claude Code 세션에서 사용하는 동일한 인증 및 모델 공급자. Eval 실행, 판사 채점 채점자, `claude plugin eval init`은 자격 증명으로 모델을 호출하므로 플랜의 사용량 제한 또는 API 청구에 계산됩니다. 명령이 비용을 보고할 때, 그 수치는 해당 호출의 [정가 추정](/docs/ko/costs)입니다.

<h2 id="how-an-eval-run-works">
  eval 실행 방식
</h2>

eval 모음은 플러그인 내부의 `evals/`라는 디렉토리에 있으며, [케이스 작성 및 개선](#write-and-refine-cases)에서 보여주는 대로 배치됩니다. 각 케이스는 [프롬프트](#set-run-limits-and-tools-in-prompt-md)와 하나 이상의 [채점자](#grade-the-result)를 포함하는 자체 하위 디렉토리입니다. 프롬프트는 플러그인을 사용하는 사람이 입력할 수 있는 것입니다. 예를 들어 스킬 중 하나가 처리해야 하는 요청입니다.

<h3 id="what-happens-in-a-run">
  실행 중 발생하는 일
</h3>

각 케이스의 실행에 대해 Claude Code는 플러그인만 로드된 새로운 [격리된](#how-runs-are-isolated) [비대화형 세션](/docs/ko/headless)을 시작하고, 프롬프트를 보내고, Claude가 완료되거나 케이스의 턴 또는 시간 제한에 도달할 때까지 작동하도록 합니다. 각 채점자는 최종 응답, 전체 기록 또는 Claude가 만든 파일을 확인하고 통과 또는 실패합니다.

<h3 id="how-a-case-is-scored">
  케이스 채점 방식
</h3>

비결정적 에이전트의 한 번의 실행은 거의 알려주지 않으므로 각 케이스는 기본적으로 3번 실행됩니다. 실행의 점수는 통과한 채점자의 비율이며, 가중치를 설정한 경우 가중치가 적용되고, 케이스의 점수는 실행 전체의 평균입니다. 케이스는 점수가 [`--threshold`](#command-options)를 충족할 때 통과하며, 기본값은 1.0입니다. 모델 호출에서 모음은 플러그인과 함께 대략 케이스 × 실행 에이전트 실행을 만들고, [플러그인 없는 기준선](#the-no-plugin-baseline)에 대해 동일한 횟수를 다시 만들며, 실행당 `llm` 또는 `baseline` 채점자당 3개의 짧은 판사 호출을 추가합니다.

<h3 id="the-no-plugin-baseline">
  플러그인 없는 기준선
</h3>

높은 점수 자체만으로는 플러그인이 도움이 되었는지 알려주지 않습니다. Claude가 플러그인 없이도 동일하게 잘 수행할 수 있기 때문입니다. 둘을 분리하기 위해 각 케이스의 실행은 기본적으로 플러그인이 로드되지 않은 상태에서 반복되며, 두 점수 `WITH`와 `W/OUT`을 얻습니다. 그들의 차이 `Δ`는 플러그인이 기여한 것입니다. 케이스가 플러그인 있음과 없음 모두에서 1.0을 점수하면, 플러그인이 통과하게 한 것이 아닙니다.

두 실행 세트를 with-arm과 without-arm이라고 합니다. [플러그인 없는 기준선과 비교](#compare-against-a-no-plugin-baseline)는 두 arm에서 채점자가 어떻게 채점되는지, 그리고 기준선을 끄는 방법을 다룹니다.

<h2 id="create-your-first-eval-suite">
  첫 번째 eval 모음 만들기
</h2>

이 연습은 자신의 플러그인에 대한 하나의 케이스를 작성하고, 실행하고, 결과를 읽습니다. 시작하기 전에 다음이 있는지 확인합니다.

* Claude Code v2.1.269 이상 및 기타 [요구사항](#requirements)
* 플러그인의 루트 디렉토리에서 열린 터미널, `plugin.json` 또는 `.claude-plugin/plugin.json`을 포함하는 디렉토리
* 테스트하려는 플러그인의 스킬 하나, 그리고 사용자가 입력할 요청으로 스킬을 트리거해야 합니다.

<Steps>
  <Step title="케이스 만들기">
    플러그인 루트에서 다음을 실행합니다.

    ```bash theme={null}
    claude plugin eval init
    ```

    Claude Code가 이 디렉토리를 아직 신뢰하지 않으면 먼저 `Trust this plugin directory?`를 묻습니다. `y`로 답합니다. 그러면 대화형 Claude Code 세션이 열립니다. Claude는 플러그인을 읽고 좋은 결과가 무엇인지 묻고, 플러그인을 트리거해야 하고 트리거하지 않아야 하는 프롬프트를 제안하고, 각각에 대해 채점자를 설계하고, 한 번 시도하여 동작을 확인하고, 프롬프트 이름을 따서 `evals/` 아래에 케이스 디렉토리를 작성합니다. Claude가 모음이 준비되었다고 말하면 `/exit` 또는 Ctrl+D로 해당 세션을 종료하여 셸로 돌아갑니다.

    플러그인 루트에서 이미 Claude Code 세션이 열려 있으면 대신 Claude에게 `claude plugin eval init`을 실행하도록 요청할 수 있습니다. Claude는 명령을 실행한 다음 해당 대화에서 동일한 질문을 합니다.

    케이스를 직접 작성하여 파일에 정확히 무엇이 포함되어 있는지 확인하려면 [케이스를 직접 작성](#write-a-case-manually)을 따르고 여기로 돌아와 실행합니다.
  </Step>

  <Step title="모음 실행">
    플러그인 루트의 셸로 돌아가서 `evals/` 아래의 모든 케이스를 실행합니다.

    ```bash theme={null}
    claude plugin eval .
    ```

    1단계에서 이 디렉토리를 이미 신뢰했으므로 실행이 즉시 시작됩니다. 대신 케이스를 직접 작성한 경우 실행은 먼저 `Trust this plugin directory? [y/N]`를 묻습니다. `y`로 답합니다. [실행이 액세스할 수 있는 것](#security)은 동의하는 것을 설명합니다.

    각 케이스는 플러그인으로 3번, 플러그인 없이 3번 실행되므로 하나의 케이스는 6번 실행됩니다. 진행 상황 라인은 각 실행이 완료될 때 인쇄되며, 해당 실행의 점수와 각 채점자의 판정이 포함됩니다.
  </Step>

  <Step title="요약 읽기">
    모음이 완료되면 요약 표가 표시되고, 그 뒤에 보고서가 간 위치가 표시됩니다.

    ```text theme={null}
    CASE        WITH  W/OUT Δ      RUNS COST    NOTES
    first-case  1.00  0.33  +0.67  6    $0.41

    1 case(s) · mean Δ +0.67 · 74s · $0.41
    Report: /Users/you/my-plugin/evals/results/2026-09-10T17-02-11-482Z/report.html
    Published: https://claude.ai/... · keep local next time with --no-publish
    ```

    `WITH`는 플러그인이 로드된 케이스의 점수이고, `W/OUT`은 로드되지 않은 점수이며, 양수 `Δ`는 플러그인이 점수를 올렸음을 의미합니다. `COST`는 모델 호출의 정가 추정이고, `NOTES`는 with-arm에서 가장 높은 가중치의 실패한 채점자의 설명 또는 실행의 오류를 보여줍니다.
  </Step>

  <Step title="보고서를 열고 반복">
    `Published:` URL 또는 `Published:` 라인이 나타나지 않을 때 `Report:` 경로를 열어 모든 실행에 대한 각 채점자의 판정과 설명, 그리고 `llm` 채점자의 경우 판사의 투표와 판단한 발췌를 확인합니다. `Published:` 라인은 계정이 [보고서를 게시](#html-report)할 수 있을 때만 나타납니다.

    가장 일반적인 첫 번째 발견은 `Δ`가 0에 가깝고 케이스의 `tool_used: Skill` 채점자가 실패하는 것입니다. 이는 Claude가 자연스러운 표현에서 스킬을 선택하지 않음을 의미합니다. 스킬의 [`description`](/docs/ko/skills#frontmatter-reference)을 조정하고, `claude plugin eval .`을 다시 실행하고, 비교합니다.

    하나의 케이스를 저렴하게 반복하려면 단일 arm을 한 번 실행합니다. 단일 실행은 노이즈가 많으므로 신뢰하기 전에 기본 3번 실행에서 변경을 확인합니다. 하나의 arm으로 표는 `WITH`, `W/OUT`, `Δ` 열 대신 `SCORE`와 `PASS%` 열을 표시합니다.

    ```bash theme={null}
    claude plugin eval . --case <case-name> --runs 1 --ablation none
    ```

    `<case-name>`을 `evals/` 아래의 디렉토리 이름 중 하나로 바꿉니다.
  </Step>
</Steps>

<h2 id="write-and-refine-cases">
  케이스 작성 및 개선
</h2>

`claude plugin eval init`이 작성하는 케이스는 열고, 변경하고, 추가할 수 있는 일반 파일입니다. 케이스는 `prompt.md`, `case.yaml` 또는 둘 다를 포함하는 플러그인의 eval 디렉토리 아래의 디렉토리입니다. 케이스를 그룹화하려면 케이스 자체가 아닌 디렉토리 아래에 중첩합니다. `graders/` 및 고정 파일과 같은 케이스 디렉토리 내부의 모든 것은 해당 케이스에 속합니다.

이것은 `claude plugin eval init`이 작성하는 레이아웃이며 새 모음에 사용할 레이아웃입니다. [eval 모음 참조](#eval-suite-reference)에는 모의 및 결과를 포함한 전체 트리가 있습니다.

```text theme={null}
my-plugin/
├── .claude-plugin/plugin.json
├── skills/...
└── evals/
    ├── first-case/
    │   ├── prompt.md          # frontmatter: case fields; body: the prompt
    │   ├── graders/
    │   │   ├── criteria.md    # frontmatter: type + options; body: rubric or pattern
    │   │   └── skill-fired.md
    │   └── case.yaml          # optional: only for context.* fields
    ├── ignores-unrelated-request/
    │   └── ...
    └── results/               # written by each run; add to .gitignore
```

<h3 id="write-a-case-manually">
  케이스를 직접 작성
</h3>

Claude가 `claude plugin eval init`으로 케이스를 작성하도록 하는 것이 권장 경로입니다. 대신 직접 작성하려면 빈 템플릿에서 시작합니다. 다음 명령은 자리 표시자 `prompt.md`와 하나의 자리 표시자 채점자가 있는 `first-case`라는 케이스를 작성하고 아무것도 실행하지 않습니다.

```bash theme={null}
claude plugin eval init --bare first-case
```

```text theme={null}
evals/first-case/
├── prompt.md            # the prompt sent to Claude, plus run limits
└── graders/
    └── criteria.md      # one grader: how to score the result
```

`prompt.md`에서 각 실행에서 Claude가 받는 메시지를 작성하고, frontmatter에서 실행의 제한 및 케이스가 사용할 수 있는 도구를 설정합니다. `evals/first-case/prompt.md`를 열고 자리 표시자 본문을 요청으로 바꿉니다. 스킬의 이름을 지정하는 대신 사용자가 입력할 방식으로 표현합니다. 이 예제는 커밋 메시지를 작성하는 스킬용입니다. 자신의 요청을 사용하세요.

```markdown theme={null}
---
max_turns: 10
allowed_tools: [Read, Glob, Grep, Skill]
---

Write me a commit message for this change: I renamed getUser to fetchUser and updated the three call sites.
```

각 실행은 빈 작업 디렉토리에서 시작하므로 작업에 필요한 모든 것을 프롬프트 자체에 넣거나 [작업 공간 설정](#add-setup-or-history-with-case-yaml)을 먼저 수행합니다. [frontmatter 필드의 전체 목록](#prompt-md-fields)은 모델, 시간 초과, 태그 및 환경 변수를 다룹니다.

`graders/` 아래의 각 파일은 실행 후 적용되는 하나의 확인입니다. `evals/first-case/graders/criteria.md`를 열고 자리 표시자를 판사 모델에 대한 루브릭으로 바꿉니다. 구체적인 PASS 및 FAIL 조건으로 작성합니다.

```markdown theme={null}
---
type: llm
---

PASS if <what a correct response contains>.
FAIL if <what a wrong or missing response looks like>.
```

그런 다음 스킬이 답변을 생성했는지 확인하는 두 번째 채점자를 추가합니다. `evals/first-case/graders/skill-fired.md`를 만들고, `your-skill-name`을 `skills/` 아래의 스킬 디렉토리 이름으로 바꿉니다. 이것은 Claude가 호출하는 이름입니다.

```markdown theme={null}
---
type: tool_used
tool: Skill
input_match: '"skill"\s*:\s*"(?:[\w-]+:)?your-skill-name"'
---
```

이것은 Claude가 실행 중에 해당 스킬을 최소 한 번 호출했을 때 통과합니다. 네임스페이스된 `plugin-name:skill-name` 형식도 포함합니다. [채점자 유형](#grader-types)은 정규식 일치 또는 파일 생성 확인과 같은 다른 확인을 나열합니다.

두 파일이 저장되면 [빠른 시작](#create-your-first-eval-suite)처럼 플러그인 루트에서 `claude plugin eval .`으로 케이스를 실행합니다.

<h3 id="set-run-limits-and-tools-in-prompt-md">
  prompt.md에서 실행 제한 및 도구 설정
</h3>

`prompt.md` frontmatter에서 케이스의 `max_turns`, `timeout_seconds`, `model`, `tags` 및 사용할 수 있는 `allowed_tools`를 설정합니다. [prompt.md frontmatter](#prompt-md-fields) 참조는 모든 필드와 기본값을 나열합니다.

Claude는 작성한 대로 본문을 정확히 받습니다. 그 안의 `@path` 언급은 파일 첨부로 확장되지 않으므로 Claude가 파일을 읽어야 하면 `allowed_tools`에서 도구를 부여합니다.

<h3 id="grade-the-result">
  채점자 선택 및 가중치
</h3>

채점자의 frontmatter는 `type`을 설정하고, 선택적으로 실행의 점수에서 더 많이 계산하는 `weight`와 기준선에 대해 채점되는 방식을 제어하는 [`arm`](#compare-against-a-no-plugin-baseline)을 설정합니다. 6가지 유형 중 `regex`, `tool_used`, `tool_order`, `file_exists`는 기록 및 파일에서 계산되며 비용이 들지 않지만, `llm` 및 `baseline`은 판사 모델을 호출하고 실행 비용에 추가됩니다.

사용자 정의 코드 채점자는 없습니다. [채점자 유형](#grader-types)은 각 유형의 옵션과 통과 조건을 나열하고, [채점자가 볼 수 있는 것](#what-a-grader-can-look-at)은 `target` 및 `focus`가 허용하는 값을 나열합니다.

`llm` 및 `baseline` 채점자의 판사는 기본적으로 작은 빠른 모델입니다. 미묘한 루브릭에 더 강한 모델을 사용하려면 `--judge-model sonnet` 또는 전체 모델 ID를 전달합니다.

<h4 id="choose-graders-that-give-a-stable-signal">
  안정적인 신호를 제공하는 채점자 선택
</h4>

`llm` 채점자는 모델에 판정을 요청하므로 실행 간에 답변이 다를 수 있으며, 읽어야 할 텍스트가 길수록 더 많이 다릅니다. 이러한 습관은 모음의 점수를 신뢰할 수 있을 정도로 안정적으로 유지합니다.

* 생성된 파일과 같은 긴 출력의 경우, 파일의 내용에 대한 `regex` 채점자로 채점합니다. 이는 매번 동일한 방식으로 전체 파일을 확인합니다. 짧은 출력에 대해 `llm` 채점자를 유지하고, 루브릭을 구체적인 PASS 및 FAIL 조건으로 작성합니다.
* 각 케이스에 결과(예: 최종 메시지 또는 생성된 파일)에 대한 하나의 채점자와 Claude가 어떻게 도달했는지(예: `tool_used` 또는 `tool_order`)에 대한 하나의 채점자를 제공합니다. 함께 답변이 올바른지 여부와 플러그인이 생성했는지 여부를 알려줍니다.
* 케이스의 `tool_used: Skill` 채점자가 통과하지만 `Δ`가 음수인 경우 플러그인 전에 판사를 의심합니다. 작은 판사 모델은 루브릭이 설명하는 것과 다르게 형식이 지정되었기 때문에 올바른 답변을 잘못 표시할 수 있습니다. `--judge-model sonnet`으로 다시 실행하고 형식이 판정을 결정하지 않도록 루브릭을 조정합니다.
* 빌드 또는 테스트가 실행 내에서 통과했는지 확인하려면 프롬프트에서 Claude에게 실행하고 결과를 파일에 작성하도록 요청하고, 해당 파일을 채점하고, 명령이 `tool_used` 채점자로 실행되었음을 주장합니다. 그 `input_match`는 명령의 이름을 지정합니다.

<h3 id="compare-against-a-no-plugin-baseline">
  플러그인 없는 기준선에 대해 채점
</h3>

플러그인이 테스트 중일 때 각 케이스는 기본적으로 두 개의 arm에서 실행됩니다. with-arm은 플러그인이 로드된 실행이고, without-arm은 플러그인이 전혀 로드되지 않은 동일한 수의 실행입니다. 요약 및 보고서는 두 점수와 `Δ`(with-arm 점수에서 without-arm 점수를 뺀 값)를 표시합니다. 비교가 필요하지 않을 때(예: 채점자를 반복할 때) 비용을 절반으로 줄이려면 `--ablation none`을 전달합니다.

두 arm 실행에서 일부 채점자는 `scored: false`로 보고됩니다. "스킬이 호출되었습니다"와 같은 확인은 플러그인 없이는 절대 통과할 수 없으므로 계산하면 without-arm이 0으로 향하고 `Δ`를 부풀립니다. 두 arm을 비교 가능하게 유지하기 위해 Claude Code는 두 arm에서 이러한 채점자를 점수에서 제외하고 with-arm에서 통과/실패 표시기로만 보고합니다. 여기에는 다음이 포함됩니다.

* `tool`이 `Skill`인 모든 `tool_used` 채점자
* `target: mock_calls`를 가진 모든 `regex` 채점자 및 `focus: mock_calls`를 가진 모든 `llm` 채점자. 각 [모의 서버](#mock-mcp-servers)가 플러그인이 선언하는 것일 때
* `arm: with-only`로 표시한 모든 채점자

세 가지 설정이 해당 제외를 변경합니다.

* **모든 채점자 제외**: 케이스의 모든 채점자가 제외된 집합에 있으면, 점수할 것이 남지 않으므로 대신 정상적으로 채점됩니다.
* **`arm: both`**: 채점자에 `arm: both`를 설정하여 관계없이 두 arm에서 채점하도록 강제합니다. 이것은 `min: 0` 및 `max: 0`으로 "스킬을 호출하지 않아야 함" 확인을 원할 때 필요합니다.
* **`--ablation none`**: `--ablation none` 아래에서는 아무것도 제외되지 않으므로 동일한 모음이 두 모드에서 다른 절대 점수를 생성할 수 있습니다.

<h3 id="use-a-different-eval-directory">
  다른 eval 디렉토리 사용
</h3>

`evals/`가 이미 다른 도구에서 사용 중인 경우 모음을 다른 디렉토리에 유지합니다. 플러그인의 `plugin.json`에 해당 디렉토리를 기록하여 모든 실행과 모든 협력자가 사용하거나 단일 실행을 위해 명령줄에서 전달할 수 있습니다.

* **`plugin.json`에서**: `"experimental": { "evals": "quality/evals" }`를 추가합니다.
* **명령줄에서**: `claude plugin eval` 및 `claude plugin eval init` 모두에 `--eval-dir quality/evals`를 전달합니다.

둘 다 설정하면 플래그의 디렉토리가 사용됩니다. `qa` 또는 `quality/evals`와 같은 일반 디렉토리 이름의 상대 경로를 제공합니다. 절대 경로 또는 `..`를 포함하는 경로는 거부됩니다. 플래그 값으로는 오류이고, 사용할 수 없는 매니페스트 값은 `Warning:` 줄을 인쇄하고 실행은 `evals/` 대신 사용합니다. 케이스, 결과 및 `init` 출력이 모두 해당 디렉토리로 이동합니다.

<h2 id="set-up-fixtures-and-mocks">
  고정 및 모의 설정
</h2>

케이스는 프롬프트 이상이 필요할 수 있습니다. 작업 공간의 파일 또는 git 저장소, 계속할 이전 대화, 또는 플러그인이 통신하는 MCP 서버의 답변입니다. 이들 각각은 실행이 반복 가능하도록 케이스 옆에 설정됩니다.

<h3 id="add-setup-or-history-with-case-yaml">
  작업 공간 또는 대화 시드
</h3>

각 실행은 빈 작업 공간에서 시작됩니다. 케이스가 프롬프트 이상이 필요할 때 `prompt.md` 옆에 `context` 블록이 있는 `case.yaml`을 추가합니다.

* **고정 파일 또는 git 저장소**: 케이스 디렉토리에 Bash 스크립트를 작성하고 `context.scaffold_script`에서 이름을 지정합니다. 스크립트는 에이전트의 샌드박스 외부에서 사용자로 실행되며 `--scaffold`를 전달할 때만 실행되므로 해당 플래그는 사용자 또는 조직이 작성한 모음에만 전달합니다.
* **이전 대화를 계속하려면**: 기록을 `.jsonl` 파일로 저장하고 `context.history_file`에서 이름을 지정합니다. 그러면 케이스의 프롬프트가 다음 사용자 턴이 됩니다.
* **Claude가 실행 중에 읽을 수 있는 고정 디렉토리**: `context.add_dirs`에 나열합니다.

`case.yaml`은 또한 `schema_version: "1.1"` 및 `name`이 필요합니다. [case.yaml 필드](#case-yaml-fields) 참조에는 전체 목록이 있습니다.

이 `case.yaml`은 스크립트에서 작업 공간을 시드하고 Claude가 `resources/` 디렉토리에서 고정을 읽도록 합니다.

```yaml theme={null}
schema_version: "1.1"
name: changelog-from-diff
tags: [smoke]
context:
  scaffold_script: fixture.sh
  add_dirs: [resources]
```

<h3 id="mock-mcp-servers">
  MCP 서버 모의
</h3>

뒤에 있는 실제 서비스 없이 MCP 도구를 호출하는 스킬이 있는 플러그인을 평가할 수 있습니다. 전체 모음에 대해 `evals/mocks/<server>/<tool>.md` 아래에 도구당 하나의 Markdown 파일을 넣거나, 하나의 케이스에 대해 케이스의 자체 `mocks/` 디렉토리 아래에 넣습니다. 여기서 `<server>`는 플러그인의 [MCP 구성](/docs/ko/plugins/components#mcp-servers)에서 서버의 이름입니다.

실행은 요청하지 않는 한 플러그인의 실제 MCP 서버를 시작하지 않습니다. Claude Code는 각 서버의 자체 이름 아래에 대체를 등록합니다. 모의 파일이 있는 도구는 그것에서 답변하고 `--allow-tools` 부여 없이 허용되며, 모의 파일이 없는 도구는 Claude에서 사용할 수 없습니다. 모의가 전혀 없는 서버는 케이스의 `mocked:` 진행 라인에 `plugin_<plugin>_<server>[not started: no mock]`으로 나타납니다.

파일의 본문은 도구가 Claude에 반환하는 것입니다. 이 모의는 `tracker`라는 서버의 `create_issue` 도구를 대신하고, Claude가 보내는 입력을 확인하고, 제목을 다시 에코합니다. `evals/mocks/tracker/create_issue.md`로 저장합니다.

```markdown theme={null}
---
expect:
  title: string
  priority: [low, medium, high]
---

Created issue #4821: {{input.title}}
```

모의 파일의 본문과 frontmatter는 이러한 옵션을 허용합니다.

* **대체**: `{{input.<field>}}`로 호출의 입력에서 필드를 삽입하고, `{{file:fixtures/{input.<field>}.json}}`으로 모의 옆의 고정 파일의 내용을 삽입합니다.
* **`expect:`**: `expect:` 블록은 입력을 보호합니다. 호출이 위반하면 실행이 점수 0으로 중단되고 이유를 기록합니다. 따라서 케이스는 플러그인이 서버에 요청한 것을 주장할 수 있습니다.
* **`error: true`**: 본문을 도구 오류로 반환하려면 `error: true`를 설정합니다.
* **`type: agent`**: 본문의 지침에서 서버로 답변하도록 작은 모델을 가지려면 `type: agent`를 설정합니다.

[모의 파일 참조](#mock-files)는 모든 키와 `_server.md` 및 `_tools.json` 파일을 나열합니다.

호출 자체를 채점하려면 채점자를 `target: mock_calls`로 지정합니다.

대신 플러그인의 실제 MCP 서버에 대해 실행하려면 이 플래그 중 하나를 전달합니다. 어느 쪽이든 해당 프로세스는 샌드박스 외부에서 사용자로 실행되며, 해당 도구는 [`--allow-tools` 부여](#grant-tools)가 필요합니다.

* **`--allow-real-servers`**: 모의하지 않은 각 서버에 대해 실제 프로세스를 시작하고, 모의 도구에서 파일로 답변하기를 계속합니다.
* **`--mocks off`**: `mocks/`를 완전히 무시하고 플러그인이 선언하는 모든 서버를 시작합니다.

<h4 id="replay-agent-mock-answers">
  에이전트 모의 답변 재생
</h4>

`type: agent` 모의는 [`--judge-model`](#command-options)에 대한 호출로 답변하므로 실행 간에 출력이 다르며 판사를 변경하면 변합니다. 실행이 오류 또는 중단 없이 완료되면 Claude Code는 결과 디렉토리 아래의 `mock-recordings/`에서 에이전트 모의가 제공한 각 답변을 저장합니다.

거기서 `ADOPT.txt`를 열어 각 기록과 복사할 `.replay/<server>/` 디렉토리를 확인합니다. 기록을 거기에 복사한 후, 나중의 실행은 모델 호출 없이 동일한 호출에서 동일한 답변을 제공합니다. 모의를 생성한 것과 함께 `mocks/.replay/`를 커밋하여 CI 실행이 반복 가능하도록 합니다.

<h2 id="run-evals">
  평가 실행
</h2>

스위트가 존재하면 `claude plugin eval`이 이를 실행합니다. target 인수로 어떤 플러그인과 케이스를 실행할지 선택하고, `--allow-tools`로 읽기 전용 세트 이상의 도구를 케이스에 부여하며, 다른 옵션으로 실행 횟수, 모델, 비용 및 출력을 제어합니다.

<h3 id="choose-what-to-evaluate">
  평가할 항목 선택
</h3>

대부분의 경우 플러그인 루트에서 `claude plugin eval .`을 실행하면 로드된 플러그인으로 스위트의 모든 케이스를 실행합니다. 단일 케이스 파일을 실행하거나 개발 중인 플러그인이 아닌 설치된 플러그인을 평가하려면 다른 target을 전달합니다:

| Target                                    | 실행되는 항목                                                                                                            |
| :---------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| `.`과 같은 플러그인의 루트 디렉토리                     | 해당 플러그인이 로드된 eval 디렉토리 아래의 모든 케이스                                                                                  |
| 단일 `prompt.md` 또는 `case.yaml` 파일          | 해당 케이스, 포함하는 플러그인이 로드됨                                                                                             |
| 설치된 플러그인 이름, `name` 또는 `name@marketplace` | 설치된 복사본의 eval 디렉토리의 케이스, 설치된 복사본이 로드됨. 결과는 현재 디렉토리의 `./evals/results/` 또는 `--eval-dir`이 있는 `./<dir>/results/`에 기록됨 |
| `name@skills-dir`                         | [skills-directory 플러그인](/docs/ko/plugins/loading#plugins-shared-through-a-repository)의 경우 동일                            |
| 생략됨                                       | 현재 디렉토리를 경로로                                                                                                       |

케이스 이름으로 필터링하려면 `--case <glob>`을 추가하고 주어진 태그 중 하나를 가진 케이스를 유지하려면 `--tag <tag>`을 추가합니다. target을 `--tag`, `--allow-tools` 및 `--json` 앞에 놓습니다. 처음 두 개는 목록을 사용하고 `--json`은 선택적 경로를 사용하므로 각각 뒤에 오는 target을 자신의 값으로 읽습니다.

<h3 id="grant-tools">
  도구 부여
</h3>

실행은 권한을 요청하기 위해 중단되지 않습니다. 부여하지 않은 권한이 필요한 기본 제공 도구(예: `Bash`, `Write`, `Edit`, `WebFetch` 및 `WebSearch`)는 세션에서 제거되므로 Claude가 전혀 호출할 수 없습니다.

실행은 케이스가 `allowed_tools`에 나열한 읽기 전용 도구(`Read`, `Glob`, `Grep`, `NotebookRead`, `Skill`, `AskUserQuestion`, `Agent`, `TodoWrite` 및 작업 도구 `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `TaskStop`)만 허용하며, `--allow-tools`로 부여하는 모든 것이 실행의 모든 케이스에 적용됩니다. 케이스가 `Bash`, `Write`, `Edit`, `WebFetch` 또는 `WebSearch`를 사용하도록 하려면 직접 부여합니다:

```bash theme={null}
claude plugin eval . --allow-tools Write Edit "Bash(npm test *)"
```

케이스가 부여하지 않은 도구를 요청했을 때 진행 출력은 이를 `not granted`로 나열합니다. [mocked](#mock-mcp-servers) MCP 서버의 도구는 권한이 필요하지 않습니다. 실제 플러그인 MCP 서버의 도구는 `--allow-real-servers` 또는 `--mocks off`로 시작된 서버와 `--allow-tools "mcp__plugin_my-plugin_github__*"`와 같은 이름으로 부여된 권한이 모두 필요합니다. 플러그인의 MCP 도구는 `mcp__plugin_<plugin>_<server>__<tool>`로 명명됩니다.

어떤 형태로든 `Bash`를 부여하면 모든 명령이 Claude Code의 [OS 수준 샌드박스](/docs/ko/sandboxing) 아래에서 실행됩니다. 쓰기는 실행의 작업 공간으로 제한되고, 홈 디렉토리와 Claude Code 구성은 읽을 수 없으며, 네트워크 액세스는 `--allow-tools "WebFetch(domain:example.com)"`으로 부여한 도메인으로 제한됩니다. 샌드박스 백엔드가 없는 머신에서 Bash 또는 PowerShell을 부여하면 Claude Code는 제한되지 않은 상태로 실행하지 않고 각 실행을 거부하며, 케이스는 실행 오류를 표시하고 일반적으로 0점을 받습니다. 기본 Windows에는 백엔드가 없으므로 WSL2 아래에서 셸 부여 스위트를 실행합니다. Linux에서는 먼저 `bubblewrap`과 `socat`을 설치합니다. [샌드박싱 필수 조건](/docs/ko/sandboxing)을 참조합니다.

<h3 id="command-options">
  명령 옵션
</h3>

이 표는 실행 횟수, 모델, 채점, 비용, 도구 부여, 모의 및 출력에 대한 옵션을 다룹니다. `claude plugin eval --help`를 실행하면 `--case`, `--tag`, `--eval-dir`, `--no-scaffold`, `--report` 및 `--verbose`도 포함하는 전체 목록을 볼 수 있습니다.

| 옵션                         | 기본값                                                                        | 효과                                                                                                                                                                       |
| :------------------------- | :------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--runs <n>`               | 각 케이스의 `runs`, 그렇지 않으면 3                                                   | 팔당 케이스당 실행                                                                                                                                                               |
| `-j`, `--concurrency <n>`  | `1`                                                                        | 최대 이 많은 에이전트 실행을 동시에 실행합니다(1\~8). 계정의 속도 제한을 공유하므로 이는 벽시계 시간을 단축하지만 해당 제한을 초과하는 처리량을 높이지는 않습니다. 결과는 케이스 순서를 유지합니다                                                        |
| `--model <model>`          | 각 케이스의 `model`, 그렇지 않으면 설정된 경우 `ANTHROPIC_MODEL`, 그렇지 않으면 Claude Code의 기본값 | 테스트 중인 에이전트의 모델. 모델 롤아웃이 플러그인 회귀로 잘못 인식되지 않도록 CI에서 고정합니다                                                                                                                 |
| `--judge-model <model>`    | 작고 빠른 모델                                                                   | `llm` 및 `baseline` 채점자의 모델                                                                                                                                               |
| `--ablation <mode>`        | 플러그인이 해결될 때 `with-without`, 그렇지 않으면 `none`                                 | 플러그인 없이 각 케이스를 실행하여 추가되는 항목을 측정할지 여부. `none`은 한 팔을 실행합니다. `with-without`은 플러그인 없는 기준선을 추가합니다                                                                             |
| `--threshold <0..1>`       | `1.0`                                                                      | 케이스는 with-arm 점수가 최소한 이 값일 때 통과합니다. 이 값 아래의 모든 케이스는 명령이 1로 종료되도록 합니다                                                                                                     |
| `--max-cost-usd <usd>`     | 상한 없음                                                                      | 실행의 정가 비용 추정에 대한 상한, 플랜 사용에 대한 상한이 아닙니다. 각 실행이 시작되기 전에 확인됩니다. 소비되면 더 이상 시작되지 않습니다. 이미 진행 중인 실행은 완료되므로 지출이 해당 실행으로 상한을 초과할 수 있습니다. 시작되지 않은 실행이 있으면 명령은 부분 결과와 함께 2로 종료됩니다 |
| `--allow-tools <tools...>` | 없음                                                                         | 읽기 전용 세트 이상의 도구를 부여합니다. [도구 부여](#grant-tools) 참조                                                                                                                         |
| `--scaffold`               | 꺼짐                                                                         | 각 케이스의 [`scaffold_script`](#add-setup-or-history-with-case-yaml) 실행                                                                                                      |
| `--trust-plugin`           | 꺼짐                                                                         | 코드와 스위트를 직접 실행할 플러그인의 첫 실행 신뢰 프롬프트를 건너뜁니다. CI에서 전달하여 작업이 프롬프트에 의해 거부되거나 대기하지 않도록 합니다. [실행이 액세스할 수 있는 항목](#security) 참조                                                   |
| `--mocks <mode>`           | `record`                                                                   | `record`는 [mocks](#mock-mcp-servers)에서 MCP 도구 호출에 응답하고, 플러그인의 실제 서버를 시작하지 않으며, 에이전트-모의 응답을 재생을 위해 저장합니다. `off`는 모의를 무시하고 플러그인의 실제 MCP 서버를 시작합니다                          |
| `--allow-real-servers`     | 꺼짐                                                                         | `--mocks record`를 사용하여 모의가 없는 서버에 대해 플러그인의 실제 MCP 서버도 시작합니다                                                                                                              |
| `--json [path]`            | 꺼짐                                                                         | [결과 문서](#json-result)를 stdout에 인쇄하거나 `.json`으로 끝나는 경로에 씁니다. 실행은 조용합니다: 진행 줄이나 요약 표가 없습니다                                                                                 |
| `--output-dir <dir>`       | `<eval dir>/results/<timestamp>/`                                          | `aggregate-result.json` 및 `report.html`이 가는 위치                                                                                                                           |
| `--no-publish`             |                                                                            | HTML 보고서를 로컬로 유지합니다. [HTML 보고서](#html-report) 참조                                                                                                                         |
| `--publish-report`         |                                                                            | Claude Code 세션이 시작한 실행과 같이 기본적으로 로컬로 유지되는 위치에서도 보고서를 게시합니다                                                                                                               |
| `--keep-temp`              | 꺼짐                                                                         | 모든 실행의 샌드박스 디렉토리를 유지하고 Claude가 생성한 항목을 디버깅하기 위해 경로를 인쇄합니다                                                                                                                |

<h3 id="run-evals-in-ci">
  CI에서 평가 실행
</h3>

CI 작업에서 `--json`으로 스위트를 실행하여 보관할 결과를 작성하고 종료 코드에서 빌드를 실패합니다. [첫 실행 신뢰 프롬프트](#security)에서 작업이 대기하지 않도록 `--trust-plugin`을 전달하고, 점수가 시간에 따라 비교 가능하도록 두 모델을 고정하며, 보고서를 로컬로 유지하고, 상한으로 비용 상한을 설정합니다:

```bash theme={null}
claude plugin eval . \
  --trust-plugin \
  --json results.json \
  --threshold 0.8 \
  --model claude-sonnet-5 \
  --judge-model claude-haiku-4-5 \
  --no-publish \
  --max-cost-usd 20
```

작업의 종료 코드는 발생한 상황을 알려줍니다:

| 종료 코드 | 의미                                                                                                                                    |
| :---- | :------------------------------------------------------------------------------------------------------------------------------------ |
| 0     | 모든 케이스가 `--threshold`에서 이상으로 점수를 받았고 모든 케이스 파일이 로드됨                                                                                   |
| 1     | 케이스가 임계값 아래로 점수를 받았거나, 케이스 파일이 로드되지 않았거나, 케이스를 찾을 수 없었거나, 실행을 시작할 수 없었거나, 플러그인 디렉토리를 신뢰하지 않고 `--trust-plugin`을 전달하지 않았거나, 옵션이 유효하지 않음 |
| 2     | 부분 실행: `--max-cost-usd` 상한에 도달했거나 첫 실행 전 또는 첫 실행 시 자격 증명이 거부됨. `results.json`은 여전히 `partial: true`와 이유와 함께 작성됨                        |
| 130   | 중단됨. 부분 결과가 작성됨                                                                                                                       |
| 143   | 종료됨(예: CI 시간 초과)                                                                                                                      |

HTML 보고서를 작성하거나 게시하는 문제는 종료 코드를 변경하지 않습니다. 케이스가 낮은 점수를 받은 이유를 보려면 `--json` 없이 로컬에서 실행하여 실행당 진행 및 채점자 줄이 인쇄되도록 합니다.

CI 러너는 또한 다음이 필요합니다:

* **설치 및 자격 증명**: CI 러너는 Claude Code 설치 및 [환경의 자격 증명](/docs/ko/authentication)(예: `ANTHROPIC_API_KEY`)이 필요합니다.
* **신뢰**: `--trust-plugin` 없이 Claude Code가 이미 신뢰하지 않는 체크아웃 디렉토리가 있는 작업은 [첫 실행 신뢰 프롬프트](#security)가 필요하며, 요청할 수 없는 실행은 종료 1로 거부됩니다.
* **CI의 `init`**: `claude plugin eval init`은 질문을 하기 위해 터미널이 필요합니다. CI에서 `claude plugin eval init --bare <name>`을 실행하여 빈 템플릿을 가져옵니다.

비용을 예측 가능하게 유지하려면 빠른 모든 변경 스위트에 판사를 호출하지 않는 채점자만 제공하고, `Δ`가 필요하지 않은 경우 `--ablation none`을 사용하며, `partial: true` 문서와 `skippedPaidGraders`가 있는 실행을 차트하는 모든 추세에서 제외합니다.

<h2 id="read-the-results">
  결과 읽기
</h2>

최소 하나의 케이스를 포함하는 모든 실행은 평가 디렉토리 내에 `results/<timestamp>/` 디렉토리를 작성하며, 여기에는 `aggregate-result.json`과 `report.html`이 포함됩니다. 경로 대상이 플러그인 아래에 있는 경우, 명명한 플러그인의 경우 현재 디렉토리 아래에 있으며, [대상 테이블](#choose-what-to-evaluate)에 표시된 대로입니다. 요약 테이블, JSON 및 보고서는 모두 동일한 결과 데이터를 렌더링합니다.

<h3 id="html-report">
  HTML 보고서
</h3>

`report.html`은 외부 요청을 하지 않는 단일 자체 포함 파일이므로 CI 작업에 첨부하거나 디스크에서 열 수 있습니다. 이 예제는 `--threshold 0.8`로 실행된 3개 케이스 스위트의 보고서 상단입니다. 표시된 비용은 정가 기준 추정치이며 모델 및 케이스 수에 따라 달라집니다:

<img src="https://mintcdn.com/claude-code/qq7LHDi_F0aeFHgk/images/plugin-eval-report.png?fit=max&auto=format&n=qq7LHDi_F0aeFHgk&q=85&s=106eb6e6a70a6565f891ea3a4564f87d" alt="평가 보고서의 상단: &#x22;Plugin effect: +33.3 pts vs baseline, improved 2, flat 1, regressed 0 of 3 cases&#x22;라고 읽는 판정 줄, 스위트 점수, 제거 델타, 기준선 점수, 임계값을 통과한 케이스 및 완벽한 실행에 대한 5개의 요약 타일, 그 다음 델타, 점수 막대 및 두 그레이더 모두 통과를 표시하는 하나의 실행이 있는 첫 번째 케이스" width="1360" height="1032" data-path="images/plugin-eval-report.png" />

위에서 아래로 읽으십시오:

* **판정 줄과 타일**은 플러그인이 전체 스위트에서 도움이 되었는지 여부를 답변합니다. 스위트 점수는 케이스별 플러그인 포함 점수의 평균이고, 제거 Δ는 해당 점수가 기준선 점수 위 또는 아래에 얼마나 떨어져 있는지를 나타내며, 케이스는 임계값을 충족한 케이스 수를 나타냅니다. 완벽한 실행은 모든 그레이더가 통과한 플러그인 포함 실행의 비율입니다.
* **각 케이스 카드**는 케이스의 자체 `Δ`와 플러그인 포함 점수를 표시하며, 막대에 임계값에 눈금이 있습니다. `Δ`가 음수인 케이스는 빨간색 왼쪽 가장자리를 가지므로 스크롤할 때 회귀가 눈에 띕니다.
* **케이스 내에서** 플러그인 포함 실행이 먼저 나오고 기준선 실행이 그 다음입니다. 각 실행은 통과 또는 실패 칩이 있는 그레이더를 나열합니다. 실패한 그레이더는 이미 설명과 함께 확장되어 있으며, `llm` 그레이더는 판정자의 투표와 표시된 증거도 표시하므로, 실행이 낮은 점수를 받은 이유를 알 수 있습니다. `tool_used: Skill`과 같이 점수에 포함되지 않는 그레이더는 `plugin-fired indicator` 배지를 포함합니다.
* **프롬프트 및 그레이더**는 실행 아래에 있으며 케이스의 프롬프트와 각 그레이더의 채점 기준 또는 패턴을 표시하므로, 스위트 없이 보고서를 읽는 사람이 무엇을 요청했는지와 무엇이 좋은 것으로 간주되었는지 볼 수 있습니다.

claude.ai 구독으로 로그인했고 [artifacts](/docs/ko/artifacts)를 계정에서 사용할 수 있는 경우, Claude Code는 보고서를 비공개 artifact로 게시하고 `Published: <url>`을 인쇄합니다. `--no-publish`를 전달하여 로컬로 유지합니다. API 키 인증과 같이 `Published:` 줄이 나타나지 않으면 로컬 파일이 보고서입니다.

Claude Code 세션이 시작한 실행(예: Claude에게 스위트를 실행하도록 요청할 때)도 로컬로 유지되며, 해당 `Report:` 줄은 `kept local`이라고 표시합니다. 해당 명령에 `--publish-report`를 추가하여 게시합니다.

<h3 id="json-result">
  JSON 결과
</h3>

`aggregate-result.json` 및 `--json` 출력은 CI 스크립트가 구문 분석할 수 있도록 `schemaVersion: 1`이 있는 버전 관리 문서입니다. 필드 이름은 camelCase이고 새 필드는 기존 필드의 이름을 바꾸지 않고 추가되므로, 인식하지 못하는 필드를 무시하도록 스크립트를 작성합니다.

다음은 게이팅 스크립트가 일반적으로 읽는 필드입니다. 문서는 또한 스위트 구성, 모든 그레이더 정의 및 설명과 증거가 있는 실행별 그레이더 결과를 포함합니다:

| 필드                                                | 의미                                                                                                                                        |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `partial`, `partialReason`                        | `cost_ceiling`, `interrupted` 또는 `auth_failed`일 때 `true`이며, 스위트가 완료되지 않았을 때입니다. 부분 결과를 추세 차트에서 제외합니다                                      |
| `aggregates.overallScore`                         | 스위트 전체의 평균 케이스 점수                                                                                                                         |
| `aggregates.casesPassed`, `aggregates.casesTotal` | `--threshold` 이상의 케이스 및 총 케이스                                                                                                             |
| `aggregates.meanDelta`                            | 케이스 전체의 평균 `Δ`, 투 암 모드 아래                                                                                                                 |
| `cases[].name`                                    | 케이스 이름                                                                                                                                    |
| `cases[].aggregates.score`                        | 케이스의 암 포함 실행 평균 점수                                                                                                                        |
| `cases[].aggregates.delta`                        | 암 포함 점수에서 암 제외 점수를 뺀 값입니다. 암이 비교 가능하지 않을 때 생략됩니다                                                                                          |
| `cases[].arms.with[].error`                       | `null` 또는 실행이 비정상적으로 종료된 이유(예: `timed out after 300s`). 시작했지만 잘못 종료된 실행도 생성된 내용에 대해 등급이 매겨지므로 null이 아닌 오류는 점수 0을 의미하지 않습니다                |
| `cases[].arms.with[].aborted`                     | [mock](#mock-mcp-servers)의 `expect:` 또는 `abort_when`이 실행을 중지했을 때 `server`, `tool` 및 `reason`과 함께 나타납니다. 실행은 0점을 받고 `error`는 `null`로 유지됩니다 |
| `cases[].arms.with[].skippedPaidGraders`          | 비용 한도가 이 실행의 판정 그레이더를 건너뛰었을 때 `true`이므로 해당 점수는 비교 가능하지 않습니다                                                                               |
| `costUsd`, `durationSeconds`, `claudeVersion`     | 판정 호출을 포함한 정가 기준 예상 비용, 벽시계 시간(초) 및 스위트를 실행한 Claude Code 버전                                                                               |

<h2 id="security">
  실행이 액세스할 수 있는 것
</h2>

`claude plugin eval`은 대상 플러그인의 스킬, 훅 및 에이전트를 로드하고 eval 모음을 머신에서 사용자로 실행합니다. 플러그인을 지정하는 것은 `claude --plugin-dir`과 동일한 신뢰 결정이므로 신뢰하는 플러그인만 평가합니다. 이 섹션에서 설명하는 격리는 테스트 중인 에이전트가 도달할 수 있는 것을 제한합니다. 플러그인의 자체 코드에 대한 경계가 아니며, 모음을 통과하는 것은 플러그인이 안전한지 여부에 대해 아무것도 말하지 않습니다.

<h3 id="trust-the-plugin-directory">
  플러그인 디렉토리 신뢰
</h3>

처음으로 디렉토리에 대해 `claude plugin eval`을 실행할 때 Claude Code는 `Trust this plugin directory?`를 묻습니다. 이미 대화형 `claude` 세션에서 신뢰 프롬프트를 수락하지 않은 경우입니다. git 저장소 내에서 예로 답하면 전체 저장소를 신뢰합니다. 대화형 세션도 마찬가지입니다. stdin 또는 stdout이 터미널이 아니거나 `--json` 아래에서 또는 `CI` 환경 변수가 `true`와 같은 참 값으로 설정되어 있을 때 실행은 물을 수 없고 종료 1로 거부됩니다. `--trust-plugin`을 전달하여 신뢰를 직접 주장합니다. 머신에서 직접 실행할 플러그인에만 해당합니다. 경로가 아닌 이름으로 지정하는 대상(설치된 플러그인 또는 스킬 디렉토리 플러그인)은 프롬프트를 건너뜁니다.

플러그인과 모음의 일부는 해당 실행을 위해 플래그를 전달할 때만 실행됩니다.

* 케이스의 [`scaffold_script`](#add-setup-or-history-with-case-yaml)는 `--scaffold`로
* [읽기 전용 세트 이상의 도구](#grant-tools)는 `--allow-tools`로
* 플러그인의 [실제 MCP 서버](#mock-mcp-servers)는 `--allow-real-servers` 또는 `--mocks off`로

케이스의 `allowed_tools` 및 스킬의 자체 `allowed-tools` frontmatter는 어느 것도 확대할 수 없습니다.

플러그인이 작성하지 않은 훅을 제공하거나 실제 MCP 서버를 시작할 때 채점자가 읽는 파일을 건드릴 수 있으므로 컨테이너 또는 CI 러너와 같은 격리된 환경에서 실행하지 않는 한 점수를 권고로 취급합니다. 훅과 서버는 에이전트의 샌드박스 외부에서 실행됩니다.

<h3 id="how-runs-are-isolated">
  실행이 격리되는 방식
</h3>

각 실행은 일회용 홈 디렉토리, 작업 디렉토리 및 Claude Code 구성을 얻고, 테스트 중인 에이전트는 플러그인만 로드된 `claude -p` 자식 프로세스로 거기서 실행됩니다. 케이스를 작성할 때 이러한 결과를 염두에 두십시오.

* **개인 또는 프로젝트 수준이 로드되지 않습니다.** 사용자 설정, 훅, `CLAUDE.md` 파일, MCP 서버, 다른 설치된 플러그인, 메모리 및 스킬이 없고, 샌드박스 위의 프로젝트 범위 `.claude/` 또는 `.mcp.json`이 읽혀지지 않습니다. 대부분의 셸 환경도 보류됩니다. [허용 목록](#prompt-md-fields) 및 `EVAL_*` 변수만 실행에 도달합니다. 플러그인이 설정이 필요하면 플러그인에 제공하거나, `scaffold_script`에서 만들거나, `EVAL_*` 변수를 전달합니다.
* **관리 정책은 여전히 실행을 제한할 수 있습니다.** 관리자가 머신에 배포한 [관리 설정](/docs/ko/managed-settings)의 제한은 실행 내에서 적용되므로 관리 머신의 결과는 해당 정책에 의해 관리되지 않은 머신과 다를 수 있습니다.
* **아티팩트 도구가 꺼져 있습니다.** [아티팩트](/docs/ko/artifacts)를 게시하는 스킬은 해당 단계 전에 생성하는 것에 대해서만 채점될 수 있습니다.
* **케이스 정의가 에이전트에서 숨겨집니다.** 실행은 eval 디렉토리를 읽을 수 없으므로 Claude는 케이스의 프롬프트, 채점자 또는 형제 케이스를 볼 수 없습니다.
* **셸 명령 외부에는 네트워크 샌드박스가 없습니다.** 부여하는 셸 명령은 샌드박스의 네트워크 규칙 아래에서 실행됩니다. `WebFetch(domain:…)` 부여는 해당 도메인에 직접 도달하고, 플러그인의 자체 훅과 시작하는 모든 실제 MCP 서버는 모든 호스트에 도달할 수 있습니다.

<h2 id="eval-suite-reference">
  Eval 모음 참조
</h2>

eval 모음이 포함할 수 있는 모든 것은 플러그인의 eval 디렉토리 아래에 있습니다. `evals/` 달리 [다른 것을 구성](#use-a-different-eval-directory)하지 않은 경우입니다. 이 트리는 `claude plugin eval`이 거기서 읽거나 작성하는 모든 파일을 보여줍니다. 케이스가 존재하려면 `prompt.md` 또는 `case.yaml`만 필요합니다.

```text theme={null}
evals/
├── <case>/                        # one directory per case; nest under a non-case directory to group
│   ├── prompt.md                  # frontmatter: case and run fields; body: the prompt
│   ├── case.yaml                  # optional: context.* fields, or the whole case in one file
│   ├── graders/
│   │   └── <name>.md              # one grader per file; frontmatter: type and options; body: rubric
│   ├── mocks/                     # optional: mocks for this case only, same layout as below
│   └── <fixtures, scripts, transcripts referenced by case.yaml>
├── mocks/                         # optional: suite-wide MCP mocks
│   ├── <server>/
│   │   ├── <tool>.md              # one mocked tool; body: the tool result
│   │   ├── _server.md             # optional: one agent that answers several tools
│   │   ├── _tools.json            # optional: saved tools/list response for real descriptions and schemas
│   │   └── fixtures/              # files inserted with {{file:fixtures/...}}
│   └── .replay/<server>/          # adopted agent-mock recordings, answered without a model call
└── results/<timestamp>/           # written by each run; add results/ to .gitignore
    ├── aggregate-result.json
    ├── report.html
    └── mock-recordings/           # agent-mock answers from clean runs, with ADOPT.txt
```

<h3 id="prompt-md-fields">
  prompt.md frontmatter
</h3>

`prompt.md` frontmatter는 이 필드를 허용합니다. 알 수 없는 키는 오류입니다.

| 필드                     | 기본값               | 목적                                                                                                                                                                                                                                                        |
| :--------------------- | :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `schema_version`       | `"1.1"`, 자동으로 설정됨 | 케이스 형식 버전. `prompt.md`로 작성된 케이스는 자동으로 설정되므로 거의 설정하지 않습니다.                                                                                                                                                                                                 |
| `name`                 | 디렉토리 이름           | 케이스 이름. `--case` 글로브는 이를 일치시키고 보고서는 이를 키로 합니다.                                                                                                                                                                                                            |
| `description`          |                   | 인간용. 실행 시간에 사용되지 않음                                                                                                                                                                                                                                       |
| `tags`                 | `[]`              | `--tag` 필터링을 위한 레이블. 태그가 일치하면 케이스가 실행됩니다.                                                                                                                                                                                                                 |
| `plugins`              | 가장 가까운 포함 플러그인    | 테스트 중인 플러그인 디렉토리, 케이스 디렉토리에 상대적. 자동 감지가 플러그인을 찾지 못할 때 `plugins: ["../.."]`를 설정합니다. [플러그인이 로드되지 않음](#the-baseline-arm-shows-no-plugin-or-delta-is-zero) 참조                                                                                                 |
| `runs`                 | `3`               | arm당 실행, 1에서 50. `--runs`이 이를 재정의합니다.                                                                                                                                                                                                                     |
| `expected_outcome`     |                   | 인간용. 실행 시간에 사용되지 않음                                                                                                                                                                                                                                       |
| `model`                | 자식 세션의 기본값        | 테스트 중인 에이전트의 모델. `--model`이 이를 재정의합니다.                                                                                                                                                                                                                    |
| `max_turns`            | `10`              | 턴 상한, 최대 200. 도달하면 실행 오류로 기록되고 일반적으로 점수를 낮추므로 관대하게 설정합니다.                                                                                                                                                                                                 |
| `timeout_seconds`      | `300`             | 실행당 벽시계 상한, 최대 3600                                                                                                                                                                                                                                       |
| `allowed_tools`        | `[]`              | 케이스가 원하는 도구(예: `[Read, Glob, Grep, Skill]`). 읽기 전용 도구는 여기에 나열될 때 부여됩니다. 다른 것의 경우 [도구 부여](#grant-tools) 참조                                                                                                                                                 |
| `append_system_prompt` |                   | 자식 세션의 시스템 프롬프트에 추가되는 텍스트                                                                                                                                                                                                                                 |
| `env`                  | `{}`              | 자식 세션을 위한 추가 환경 변수. 키는 `EVAL_[A-Z0-9_]*`와 일치해야 합니다. 다른 키는 실행을 실패합니다. 실행은 셸에서 허용 목록만 상속합니다. `PATH` 및 로케일과 같은 기본, 프록시 및 인증서 설정, 모델 공급자를 선택하고 인증하는 변수, 대부분의 `ANTHROPIC_*` 및 `CLAUDE_CODE_*` 구성, `EVAL_*`. 플러그인에 도구 체인 설정과 같은 다른 것을 전달하려면 `EVAL_*` 변수로 내보냅니다. |

<h3 id="case-yaml-fields">
  case.yaml 필드
</h3>

`case.yaml`은 YAML에서 동일한 케이스를 설명하고 다른 파일을 가리키는 필드를 추가합니다. `schema_version: "1.1"` 및 `name`이 필요합니다. `prompt.md` 필드 `description`, `tags`, `plugins`, `runs`, `expected_outcome`은 최상위 수준에 있습니다. `model`, `max_turns`, `timeout_seconds`, `allowed_tools`, `append_system_prompt`, `env`는 `execution:` 아래에 있습니다. 두 파일이 모두 존재하면 `prompt.md` frontmatter가 일치하는 `case.yaml` 필드를 재정의하고, `prompt.md` 본문이 프롬프트이며, `graders/*.md`는 `case.yaml`에 나열된 모든 채점자 후에 추가됩니다.

이 필드는 `case.yaml`에만 존재합니다.

| 필드                        | 목적                                                                                                                                              |
| :------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `context.scaffold_script` | Claude가 시작하기 전에 빈 작업 공간에서 실행되는 케이스 디렉토리의 Bash 스크립트. 고정 파일 또는 git 저장소를 만듭니다. [`--scaffold`](#add-setup-or-history-with-case-yaml)를 전달할 때만 실행됩니다. |
| `context.history_file`    | 케이스 디렉토리의 `.jsonl` 기록을 재개합니다. 케이스의 프롬프트가 다음 사용자 턴이 됩니다.                                                                                         |
| `context.add_dirs`        | Claude가 실행 중에 읽을 수 있는 케이스 디렉토리 내부의 디렉토리, 읽기 전용으로 부여됨                                                                                            |
| `execution.prompt`        | 전체 케이스를 `case.yaml`에 유지하고 `prompt.md`를 생략할 때 프롬프트                                                                                               |
| `graders`                 | 채점자 목록, 각각 `name` 및 `graders/*.md` 파일이 frontmatter에서 가져가는 동일한 키. `llm` 채점자의 경우 루브릭을 `criteria`에 넣습니다.                                           |

<h3 id="grader-frontmatter">
  채점자 frontmatter
</h3>

`graders/` 아래의 모든 채점자 파일은 frontmatter에서 이 키를 가져가고, 유형에 대한 옵션도 가져갑니다. 채점자의 이름은 `.md` 없는 파일 이름입니다.

| 키        | 기본값    | 목적                                                                                                                                         |
| :------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `type`   | 필수     | [채점자 유형](#grader-types) 중 하나                                                                                                               |
| `weight` | `1`    | 실행의 점수에서 상대 가중치. 모든 양수                                                                                                                     |
| `arm`    | 설정 안 됨 | `with-only`는 [두 arm 실행](#compare-against-a-no-plugin-baseline)에서 채점에서 채점자를 제외합니다. `both`는 Claude Code가 다른 경우 제외할 채점자를 두 arm에서 채점하도록 강제합니다. |

<h4 id="what-a-grader-can-look-at">
  채점자가 볼 수 있는 것
</h4>

`regex` 채점자는 `target`을 가져가고 `llm` 채점자는 `focus`를 가져갑니다. 둘 다 동일한 값을 허용합니다.

| 값                                | 채점자가 보는 것                                                                                                                                                                       |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `last_message`                   | Claude의 최종 응답 텍스트. 이것이 기본값입니다.                                                                                                                                                  |
| `trace`                          | 전체 세션을 JSON으로, 라인당 하나의 메시지. `regex` 채점자는 모든 메시지를 봅니다. `llm` 판사는 처음 12개와 마지막 12개를 봅니다. 그 안의 따옴표와 줄 바꿈은 JSON 이스케이프되므로 정규식은 `"` 대신 `\"`를 일치시킵니다.                                   |
| `files`                          | Claude가 실행 중에 만든 경로 목록, 라인당 하나. 내용이 아니고, 스캐폴드가 만들었거나 Claude가 수정한 파일이 아닙니다.                                                                                                      |
| `{ source: file, path: <path> }` | 실행 후 작업 공간의 한 파일의 내용. 플러그인이 생성한 것을 채점하는 데 사용합니다. PNG, JPEG, GIF 또는 WebP 파일은 `llm` 판사에게 이미지로 표시됩니다. `llm` 판사는 `.pptx` 또는 PDF와 같은 다른 바이너리 파일을 거부합니다. 이미지로 렌더링하거나 텍스트로 작성하고 채점합니다. |
| `mock_calls`                     | Claude가 [모의 MCP 도구](#mock-mcp-servers)에 한 각 호출, 입력 및 모의의 답변 포함                                                                                                                  |

<h4 id="grader-types">
  채점자 유형
</h4>

아래의 각 채점자 유형은 옵션과 통과 조건을 나열합니다.

| 유형            | 옵션                                    | 통과 조건                                                                                                                                                                     |
| :------------ | :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `regex`       | `pattern`, `flags`, `match`, `target` | JavaScript 정규식 `pattern`이 대상에서 발견됩니다. `match: not_contains`를 설정하여 부재를 요구하거나 `match: "count:N"`을 설정하여 정확히 N개의 일치를 요구합니다. 대소문자 불감을 `flags: i`에 넣습니다. 인라인 `(?i)`는 지원되지 않습니다. |
| `tool_used`   | `tool`, `input_match`, `min`, `max`   | `tool`에 대한 호출 수. JSON 인코딩된 입력이 선택적 `input_match` 정규식과 일치하는 것은 `min`(기본 1) 및 `max`(기본 무제한) 사이입니다. 도구가 절대 호출되지 않았음을 주장하려면 `min: 0` 및 `max: 0`을 설정합니다.                       |
| `tool_order`  | `before`, `after`                     | 두 도구가 모두 호출되고 첫 번째 일치하는 `before` 호출이 첫 번째 일치하는 `after` 호출보다 앞에 있습니다. 각각은 도구 이름 또는 `{ tool, input_match }`                                                                 |
| `file_exists` | `path`, `exists`                      | Claude가 만든 파일이 `path` 글로브와 일치하거나, `exists: false`로 없습니다. 실행 중에 만든 파일만 계산됩니다.                                                                                              |
| `llm`         | `criteria`, `focus`                   | 판사 모델이 최소 3번의 투표 중 2번 이상 루브릭에 대해 PASS에 투표합니다. `.md` 레이아웃에서 파일 본문이 기준입니다.                                                                                                  |
| `baseline`    | `baseline_file`, `criteria`           | 판사가 실행이 `baseline_file`(케이스 디렉토리의 `.jsonl`)의 참조 기록만큼 기준을 충족한다고 판단합니다.                                                                                                     |

<h3 id="mock-files">
  모의 파일
</h3>

`mocks/<server>/` 아래의 `<tool>.md` 파일은 하나의 도구에 답변합니다. 본문은 도구 결과이며, `{{input.<field>}}` 및 `{{file:fixtures/<name>}}` 치환이 있습니다. frontmatter는 이 키를 허용합니다.

| 키            | 기본값     | 목적                                                                                                                                                                |
| :----------- | :------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`       | `fixed` | `fixed`는 본문을 작성한 대로 반환합니다. `agent`는 본문을 작은 모델이 실행을 위해 서버를 재생하고 이전 호출을 기록으로 보는 지침으로 취급합니다.                                                                         |
| `expect`     | 설정 안 됨  | 점선 입력 경로에서 `string`, `number`, `boolean`, `array`, `object`와 같은 유형 이름, `/regex/`, 리터럴 또는 허용된 리터럴 목록으로의 맵. 위반하는 호출은 점수 0으로 실행을 중단하고 `aborted`로 서버, 도구 및 이유로 보고됩니다. |
| `error`      | `false` | `fixed`만. 본문을 도구 오류로 반환합니다.                                                                                                                                       |
| `abort_when` | 설정 안 됨  | `agent`만. 에이전트가 실행을 중단할 수 있는 유일한 조건을 나열하는 산문                                                                                                                      |

두 개의 선택적 파일은 서버의 디렉토리에서 도구 파일 옆에 있습니다.

* **`_server.md`**: 여러 도구에 답변하는 단일 `type: agent` 모의. `tools:` frontmatter 키에 나열됩니다. 동일한 도구에 대한 `<tool>.md`가 우선합니다. `expect:` 보호를 개별 `<tool>.md`에 넣고, 여기에는 아닙니다.
* **`_tools.json`**: 실제 서버에서 저장된 `tools/list` 응답이므로 모의 도구는 자리 표시자 대신 실제 설명 및 입력 스키마를 전달합니다.

케이스의 자체 `mocks/` 디렉토리는 동일한 레이아웃을 사용하고 파일별로 모음의 모의를 재정의합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

이는 저자들이 가장 자주 마주치는 문제들이며, 보이는 내용을 기준으로 정렬되어 있습니다.

<h3 id="plugin-eval-is-currently-in-early-access">
  "plugin eval is currently in early access"
</h3>

빌드가 명령어의 일반 공개 이전 버전입니다. `claude update`를 실행한 후 새로운 세션에서 명령어를 다시 실행하세요.

<h3 id="plugin-eval-is-currently-unavailable">
  "plugin eval is currently unavailable"
</h3>

Anthropic이 서버 측에서 명령어를 비활성화했습니다. 머신의 어떤 것도 이를 다시 켤 수 없습니다. `claude update`를 실행하고 나중에 새로운 세션에서 다시 시도하세요.

<h3 id="is-not-a-trusted-plugin-directory-and-this-run-cannot-stop-to-ask-you-about-it">
  "is not a trusted plugin directory, and this run cannot stop to ask you about it"
</h3>

이는 Claude Code가 아직 신뢰하지 않는 디렉토리에 대한 첫 번째 실행이며, stdin 또는 stdout이 터미널이 아니거나 `--json`을 전달했거나 `CI` 환경 변수가 `true`와 같은 참 값으로 설정되어 있기 때문에 물어볼 수 없습니다. 터미널에서 `claude plugin eval <dir>`을 한 번 실행하고 프롬프트에 답하거나, 플러그인의 코드와 스위트를 신뢰한다면 `--trust-plugin`을 전달하세요. [실행이 접근할 수 있는 것](#security)을 참조하세요.

<h3 id="no-eval-cases-found">
  "No eval cases found"
</h3>

eval 디렉토리 아래에 `<case>/prompt.md` 또는 `<case>/case.yaml`이 존재하지 않거나, `--case` 및 `--tag` 필터가 어떤 케이스와도 일치하지 않습니다. 플러그인 루트에서 실행하거나 `claude plugin eval init`을 실행하여 스위트를 생성하세요.

<h3 id="the-baseline-arm-shows-no-plugin-or-delta-is-zero">
  기준선 팔이 플러그인을 표시하지 않거나 델타가 0입니다
</h3>

기준선에 `W/OUT` 열이 없거나 케이스가 "ablation requested but no plugin resolved"로 실패하면 케이스에 대해 플러그인을 찾을 수 없습니다. 케이스에 `plugins: ["../.."]`을 추가하여 케이스 디렉토리에서 플러그인 디렉토리로의 경로를 제공하세요.

플러그인이 로드되었고 `Δ`가 여전히 `tool_used: Skill` 그레이더가 실패하는 상태에서 0에 가깝다면, 이는 보통 실제 발견을 의미하며, 스킬의 `description`이 프롬프트의 표현에 트리거되지 않음을 의미합니다. 설명을 조정하고 동일한 스위트를 다시 실행하세요.

<h3 id="agent-type-’-’-not-found-for-one-of-your-plugin’s-agents">
  플러그인의 에이전트 중 하나에 대해 "Agent type '...' not found"
</h3>

기본적으로 각 케이스는 플러그인 있이 실행되고 없이도 실행되며, 플러그인 없이 실행되는 것은 [플러그인 없는 기준선](#the-no-plugin-baseline)입니다. Claude가 기준선 실행에서 플러그인의 에이전트 중 하나를 디스패치할 때, Agent 도구 호출은 `Agent type '<plugin>:<agent-name>' not found. Available agents: ...`로 실패합니다. 목록은 [내장 서브에이전트](/docs/ko/sub-agents#built-in-subagents)와 같이 플러그인 없이 존재하는 에이전트만 이름을 지정합니다.

오류는 예상되는 것입니다. `Δ`가 플러그인 없는 기준선에 대해 플러그인의 실행을 비교하기 때문입니다. JSON 결과에서 기준선 실행은 `cases[].arms.without` 아래에 있습니다.

플러그인이 로드된 실행에서, `allowed_tools`에 `Agent`를 나열하는 케이스는 `my-plugin:code-reviewer`와 같은 네임스페이스 이름으로 플러그인의 에이전트 중 하나를 디스패치할 수 있습니다. 이는 `my-plugin`이라는 이름의 플러그인에서 `code-reviewer` 에이전트입니다. 기준선 실행을 건너뛰려면 `--ablation none`을 전달하세요.

<h3 id="everything-scores-zero-although-the-right-files-were-produced">
  올바른 파일이 생성되었음에도 불구하고 모든 것이 0점입니다
</h3>

그레이더가 생성된 경로의 목록인 `files`를 대상으로 하지만, 파일의 내용을 의도했습니다. `{ source: file, path: <path> }`를 `target` 또는 `focus`로 사용하세요. 별도로, `file_exists`는 실행 중에 생성된 파일만 계산하므로, 스캐폴드가 생성했거나 Claude가 편집한 파일은 보이지 않습니다. 내용을 등급 매기거나 `Edit`에서 `tool_used`를 사용하세요.

<h3 id="a-regex-over-the-trace-doesn’t-match-text-i-can-see">
  추적에 대한 정규식이 볼 수 있는 텍스트와 일치하지 않습니다
</h3>

* **잘못된 대상**: 기본 `target`은 추적이 아니라 `last_message`입니다.
* **JSON 이스케이핑**: `target` 추적을 수행할 때, 줄당 JSON이므로 따옴표는 `\"`로 나타납니다.
* **정규식 구문**: 정규식은 JavaScript 구문을 사용하므로 `(?i)`를 작성하는 대신 `flags`에 `i`를 넣으세요.

<h3 id="tools-are-denied-mcp-tools-are-missing-or-bash-won’t-run">
  도구가 거부되거나, MCP 도구가 누락되거나, Bash가 실행되지 않습니다
</h3>

읽기 전용 세트를 초과하는 모든 것은 `--allow-tools Bash Write`와 같은 권한이 필요합니다. 개인 MCP 서버는 실행에서 로드되지 않습니다. 플러그인의 자체 서버는 [옵트인](#mock-mcp-servers)하지 않는 한 시작되지 않으며, 그 도구는 `--allow-tools "mcp__plugin_<plugin>_<server>__*"` 권한도 필요합니다. 모의 도구는 둘 다 필요하지 않습니다.

<h3 id="the-run-exits-1-but-the-results-look-fine">
  실행이 1로 종료되지만 결과는 정상으로 보입니다
</h3>

기본 `--threshold`는 1.0이므로, 어떤 케이스가 완벽 이하로 점수를 받으면 명령어는 1로 종료됩니다. 기준과 일치하는 임계값을 설정하세요. 종료 1은 로드에 실패한 케이스 파일도 포함하며, 이는 테이블 위의 stderr에 보고됩니다.

<h3 id="json-output-path-must-end-in-json">
  "--json output path must end in .json"
</h3>

대상을 `--json` 뒤에 놓았으므로 출력 경로로 읽혔습니다. `claude plugin eval . --json`과 같이 대상을 먼저 놓거나 `--json`에 명시적 `.json` 경로를 제공하세요.

<h3 id="a-grader-shows-passed-false-under-a-run-that-scored-1-0">
  그레이더가 1.0으로 점수를 받은 실행 아래에서 passed: false를 표시합니다
</h3>

해당 그레이더는 설계상 2팔 실행에서 점수에서 제외되며, `scored` 필드는 `false`입니다. [플러그인 없는 기준선과 비교](#compare-against-a-no-plugin-baseline)를 참조하세요.

<h3 id="runs-fail-with-a-usage-limit-or-rate-limit-error-partway-through">
  실행이 중간에 사용량 제한 또는 속도 제한 오류로 실패합니다
</h3>

계정이 스위트 실행 중에 플랜의 사용량 제한 또는 API 속도 제한에 도달하면, 각 이후 실행은 해당 오류로 끝나고, 생성한 것에 대해 등급이 매겨지며, 보통 0점을 받습니다. 스위트는 여전히 완료되고 `partial`로 표시되지 않으므로, 결과는 회귀처럼 보일 수 있습니다. 점수를 신뢰하기 전에 `NOTES` 열 또는 JSON의 `cases[].arms.with[].error`에서 제한 메시지를 확인한 후, 제한이 재설정된 후 `--runs 1` 또는 `--case` 필터를 사용하여 다시 실행하세요.

<h3 id="runs-time-out-or-hit-the-turn-cap">
  실행이 시간 초과되거나 턴 상한에 도달합니다
</h3>

기본값은 10턴과 300초입니다. 더 많은 것이 필요한 작업의 경우 케이스에서 `max_turns` 및 `timeout_seconds`를 높이고, 실행당 제한이 아닌 비용 상한으로 `--max-cost-usd`를 사용하세요.

<h2 id="see-also">
  참고 항목
</h2>

* [플러그인 만들기](/docs/ko/plugins/create): 테스트 중인 플러그인을 만들고 개발 중에 `--plugin-dir`으로 로드합니다
* [플러그인 명령어 참조](/docs/ko/plugins/cli-reference#plugin-eval): `plugin eval` 및 `plugin eval init` 명령어 항목입니다. 매니페스트의 [`experimental.evals`](/docs/ko/plugins/manifest-reference#fields) 키는 매니페스트 참조에 있습니다
* [스킬](/docs/ko/skills): 스킬의 설명이 Claude가 호출할 때를 결정하는 방식. 스킬이 트리거되는지 확인하는 케이스가 측정하는 것입니다.
* [샌드박싱](/docs/ko/sandboxing): 실행에 Bash를 부여할 때 적용되는 OS 수준 샌드박스
* [플러그인 게시](/docs/ko/plugins/publish): 플러그인의 모음이 통과한 후 플러그인을 게시합니다
* [플러그인 비용 및 사용량 측정](/docs/ko/plugins/measure): 플러그인이 각 세션의 컨텍스트에 추가하는 것과 사람들이 여전히 사용하는지 여부입니다
