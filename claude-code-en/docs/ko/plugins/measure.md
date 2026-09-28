> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 비용 및 사용량 측정

> Claude Code 플러그인의 토큰 비용을 측정하고, 사람들이 여전히 사용하는지 확인하며, 조직 전체 플러그인 질문을 위한 텔레메트리 이벤트를 선택합니다.

플러그인이 활성화된 모든 세션에는 플러그인의 스킬, 에이전트 및 명령의 이름과 설명이 Claude의 컨텍스트에 포함되며, 플러그인이 사용되는지 여부와 관계없이 이러한 토큰은 사용자의 사용량에 계산됩니다. 이 페이지에서는 플러그인의 해당 숫자를 확인하는 방법, 플러그인을 유지 관리하는 경우 이를 줄이는 방법, 플러그인이 여전히 사용 중인지 확인할 수 있도록 사용량이 표시되는 위치를 보여줍니다.

이 페이지는 플러그인 작성자 및 유지 관리자를 위한 것입니다. 조직을 위해 Claude Code를 관리하는 경우, [전체 플릿에서 측정](#measure-across-a-fleet)에서 모든 머신에서 동일한 질문을 다룹니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **플러그인이 Claude의 동작을 얼마나 안정적으로 변경하는지 테스트**: [플러그인을 evals로 테스트](/docs/ko/plugin-evals) 참조
  * **자신의 세션 컨텍스트 정리**: [설치된 플러그인 관리](/docs/ko/plugins/install#manage-installed-plugins) 및 [컨텍스트 윈도우](/docs/ko/context-window) 페이지 참조
</Note>

[플러그인 비용 측정](#measure-what-a-plugin-costs)부터 시작합니다.

<h2 id="measure-what-a-plugin-costs">
  플러그인 비용 측정
</h2>

플러그인이 Claude의 컨텍스트에 추가하는 내용을 확인하려면 플러그인의 이름으로 [`claude plugin details`](/docs/ko/plugins/cli-reference#plugin-details)를 실행합니다. 실행 중인 Claude Code 세션의 프롬프트가 아닌 셸에서 실행합니다. 플러그인이 로드되어야 합니다: 설치되거나, 스킬 디렉토리에 있거나, 같은 명령에서 `--plugin-dir`로 전달되어야 합니다(예: `claude --plugin-dir ./formatter plugin details formatter`).

이 예제는 두 개의 스킬, 명령, 에이전트, 훅 및 MCP 서버가 있는 `formatter`라는 설치된 플러그인을 읽습니다:

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

출력의 각 부분은 다른 질문에 답합니다:

* **Component inventory**: Claude Code가 플러그인에서 찾은 것. 명령은 스킬과 함께 계산되므로 `format-all`은 `Skills` 아래에 나타납니다. 훅과 MCP 서버는 비용 추정치를 받지 않으며 per-component 행이 없습니다. 플러그인의 MCP 도구가 추가하는 내용을 확인하려면 플러그인이 활성화된 세션에서 `/context`를 실행하고 `MCP tools` 카테고리를 읽습니다.
* **Always-on**: 플러그인이 활성화된 모든 세션에 추가되는 플러그인의 스킬, 에이전트 및 명령의 이름과 설명의 토큰입니다(아무것도 실행되지 않는지 여부). 이것은 모든 사용자가 가지고 있는 숫자이며 줄여야 할 숫자입니다.
* **Per-component**: 각 행은 하나의 스킬, 에이전트 또는 명령을 always-on 공유 및 on-invoke 비용으로 분할합니다. on-invoke 비용은 해당 구성 요소가 실행될 때만 로드되는 본문입니다. always-on 열을 사용하여 어느 구성 요소가 가장 많이 기여하는지 찾습니다.

<h3 id="lower-the-always-on-figure">
  Always-on 수치 낮추기
</h3>

플러그인을 유지 관리하는 경우, 이러한 변경 사항은 모든 세션에 추가되는 내용을 줄입니다. 플러그인만 사용하는 경우, 옵션은 비활성화하거나 제거하는 것입니다. [설치된 플러그인 관리](/docs/ko/plugins/install#manage-installed-plugins)를 참조합니다.

always-on 수치는 각 구성 요소의 이름과 `description` 및 `when_to_use` frontmatter를 계산합니다. 이를 낮추려면:

* 스킬 및 에이전트 설명을 단축합니다.
* 큰 플러그인을 분할하여 사용자가 필요한 구성 요소만 설치하도록 합니다.

스킬의 설명은 Claude가 요청과 일치시키는 것이기도 하므로, 더 짧은 설명은 스킬 트리거를 중지할 수 있습니다. 설명을 정리한 후 eval 스위트의 [`tool_used: Skill` grader](/docs/ko/plugin-evals#create-your-first-eval-suite)로 트리거를 확인합니다.

각 구성 요소 유형이 기여하는 내용은 [플러그인 구성 요소](/docs/ko/plugins/components)를 참조합니다.

<h3 id="cost-shown-to-users-before-install">
  설치 전 사용자에게 표시되는 비용
</h3>

공식 마켓플레이스의 플러그인은 설치 전에 비용을 사용자에게 표시합니다. `/plugin`에서 사용자가 마켓플레이스의 플러그인 목록을 탐색하고 플러그인을 선택하면, 세부 정보 창에 **Context cost** 섹션이 표시되며 `Every turn:` 행과 `When invoked:` 행이 있습니다. always-on 수치가 2,000개 토큰 이상일 때, `Every turn:` 행이 강조 표시되어 나타납니다.

자신의 마켓플레이스에 있는 플러그인에는 **Context cost** 섹션이 없습니다.

<h2 id="check-whether-a-plugin-is-used">
  플러그인 사용 여부 확인
</h2>

Claude Code는 플러그인의 사용량을 작성자에게 보고하지 않습니다. 사용량은 플러그인을 설치한 각 사람의 머신에 기록되므로, 배울 수 있는 내용은 해당 사람들과의 관계에 따라 달라집니다:

* **조직을 위해 Claude Code를 관리합니다**: OpenTelemetry 이벤트 및 Analytics API는 모든 머신에서 설치 및 스킬 활성화를 계산합니다. [전체 플릿에서 측정](#measure-across-a-fleet)을 참조합니다.
* **물어볼 수 있는 팀원입니다**: 각 사용자의 자신의 Claude Code는 네 곳에서 플러그인을 여전히 사용하는지 보여줍니다: [`/plugin` 패널](#not-used-recently-in-/plugin), [`/skill-doctor`](#find-skills-that-never-run), [`/doctor`](#unused-plugins-in-/doctor), 및 [`/usage`](#usage-share-in-/usage). 네 가지 모두 사용자가 자신의 머신의 세션에서 Claude Code 프롬프트에서 실행하는 명령입니다.
* **둘 다 아닙니다**: 해당 플러그인에 대해 Claude Code에서 사용량 신호가 없습니다.

<h3 id="not-used-recently-in-/plugin">
  `/plugin`에서 최근에 사용되지 않음
</h3>

`/plugin`의 **Installed** 탭에서, 사용자가 마켓플레이스에서 설치한 플러그인은 최소 14일 동안 사용되지 않고 10개 세션이 지난 후 **Not used recently** 헤더 아래로 이동합니다. 플러그인의 세부 정보에는 `Last used:` 행도 표시됩니다. 사용자가 해당 헤더 및 행으로 수행하는 작업은 [더 이상 사용하지 않는 플러그인 찾기](/docs/ko/plugins/install#find-plugins-you-no-longer-use)를 참조합니다.

**Not used recently** 헤더는 다음의 경우 나타나지 않습니다:

* `--plugin-dir`로 로드되거나 스킬 디렉토리에서 로드된 플러그인
* 관리 설정을 통해 활성화되거나 [시드 디렉토리](/docs/ko/plugins/org#seed-containers-and-ci)에서 마운트된 플러그인
* 테마, 출력 스타일, 모니터 또는 워크플로우를 포함하는 플러그인(추적된 호출 없이 사용 중이므로)

플러그인의 [언어 서버](/docs/ko/plugins/components#lsp-servers)는 진단을 제공하거나 코드 네비게이션 요청에 응답할 때 사용된 것으로 계산되므로, 서버가 세션에서 활성화된 LSP 플러그인은 사용되지 않는 것으로 나열되지 않습니다.

사용자의 조직이 [`strictKnownMarketplaces`](/docs/ko/plugins/org#restrict-what-users-can-install)를 설정하면, 헤더와 `Last used:` 행이 모두 나타나지 않습니다.

<h3 id="find-skills-that-never-run">
  실행되지 않는 스킬 찾기
</h3>

`/skill-doctor`를 실행하여 각 스킬의 비용과 사용 빈도를 확인합니다. 플러그인의 스킬을 포함하여 Claude의 스킬 목록에 있지만 호출된 적이 없는 스킬을 표시합니다.

대화형 세션에서, 보고서는 `/plugin` 관리자의 **Stats** 탭에서 열립니다. 보고서가 다루는 내용과 사용 가능한 위치는 [사용되지 않는 스킬 찾기](/docs/ko/skills#find-unused-skills)를 참조합니다.

<h3 id="unused-plugins-in-/doctor">
  `/doctor`의 사용되지 않는 플러그인
</h3>

`/doctor` 체크업은 각 사용자 설치 스킬, MCP 서버 및 플러그인을 나열하고 사용되지 않은 것들을 비활성화할 것을 권장합니다. [명령 참조의 `/doctor`](/docs/ko/commands#all-commands)를 참조합니다.

<h3 id="usage-share-in-/usage">
  `/usage`의 사용량 공유
</h3>

Pro, Max, Team 또는 Enterprise 플랜에서, `/usage` 분석은 최근 사용량을 스킬, 서브에이전트, 플러그인 및 MCP 서버에 총계의 공유로 속성을 지정합니다. [/usage 명령 사용](/docs/ko/costs#using-the-/usage-command)을 참조합니다.

<h2 id="measure-across-a-fleet">
  전체 플릿에서 측정
</h2>

조직을 위해 Claude Code를 관리하는 경우, 다음 두 소스 중 하나에서 모든 머신에서 플러그인 비용 및 사용량을 측정할 수 있습니다:

* **OpenTelemetry 이벤트**: Claude Code는 [익스포터를 구성](/docs/ko/monitoring-usage)한 후 자신의 백엔드로 이를 내보냅니다. [플러그인 설치 및 사용을 위한 OpenTelemetry 이벤트](#pick-the-opentelemetry-event-for-each-question)를 참조합니다.
* **Analytics API**: Anthropic의 기록에서 제공되며, 익스포터가 필요하지 않습니다. [Analytics API 쿼리](#query-the-analytics-api)를 참조합니다.

<h3 id="pick-the-opentelemetry-event-for-each-question">
  플러그인 설치 및 사용을 위한 OpenTelemetry 이벤트
</h3>

이러한 OpenTelemetry 이벤트 및 속성은 백엔드에서 각 플러그인 질문에 답합니다:

| 질문                          | OpenTelemetry 이벤트 또는 속성                                                                                                        |
| :-------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| 어떤 플러그인이 설치되고 어디서 설치되는가     | [`claude_code.plugin_installed`](/docs/ko/monitoring-usage#plugin-installed-event), 설치당 하나                                          |
| 어떤 플러그인이 몇 개 세션에서 활성화되는가    | [`claude_code.plugin_loaded`](/docs/ko/monitoring-usage#plugin-loaded-event), 세션 시작 시 활성화된 플러그인당 하나                                 |
| 어떤 스킬이 활성화되고 어떤 플러그인이 소유하는가 | [`claude_code.skill_activated`](/docs/ko/monitoring-usage#skill-activated-event), 플러그인 스킬의 경우 `plugin.name` 및 `marketplace.name` 포함 |
| 플러그인의 훅이 보고하는 것             | [`claude_code.hook_plugin_metrics`](/docs/ko/monitoring-usage#hook-plugin-metrics-event), 공식 마켓플레이스 플러그인의 훅에 대해서만 내보냄               |
| 플러그인이 API 지출에서 비용이 드는 것     | [비용 카운터](/docs/ko/monitoring-usage#cost-counter)의 `plugin.name` 및 `marketplace.name`, 활성 스킬 또는 서브에이전트가 플러그인에 속할 때 설정                |

<h3 id="redacted-plugin-names-in-your-backend">
  백엔드의 수정된 플러그인 이름
</h3>

공식 마켓플레이스의 플러그인은 플러그인 이름과 마켓플레이스 이름을 백엔드에 그대로 보고합니다. 다른 모든 플러그인의 이름은 기본적으로 수정되거나 생략됩니다(조직의 자신의 마켓플레이스의 플러그인 포함). 플러그인의 [신뢰 계층](/docs/ko/plugins/security#find-plugins-in-telemetry)이 어느 것을 결정합니다.

일부 이벤트에서 실제 이름을 얻으려면, 익스포터를 구성하는 동일한 [관리 설정](/docs/ko/monitoring-usage#administrator-configuration)의 `env` 블록에서 텔레메트리를 내보내는 머신에서 [`OTEL_LOG_TOOL_DETAILS`](/docs/ko/monitoring-usage#common-configuration-variables) 환경 변수를 `1`로 설정합니다:

| 이벤트                                   | 기본값                                                                                     | `OTEL_LOG_TOOL_DETAILS=1` 포함                |
| :------------------------------------ | :-------------------------------------------------------------------------------------- | :------------------------------------------ |
| `plugin_loaded`                       | `plugin.name` 및 `marketplace.name`은 리터럴 문자열 `third-party`                               | 실제 이름                                       |
| `plugin_installed`, `skill_activated` | `plugin.name` 및 `marketplace.name` 생략; `skill_activated`에서 `skill.name`은 `custom_skill` | 실제 이름                                       |
| 비용 카운터                                | `plugin.name`은 `third-party`; `marketplace.name` 없음                                     | 실제 `plugin.name`; `marketplace.name` 여전히 없음 |

`plugin_loaded`에서, `plugin_id_hash`는 여전히 기본적으로 각 플러그인을 식별하므로, 서로 다른 타사 플러그인을 계산할 수 있습니다.

<h3 id="query-the-analytics-api">
  Analytics API 쿼리
</h3>

Enterprise 플랜에서, Analytics API는 익스포터가 필요 없이 Anthropic의 기록에서 "조직이 설치하고 호출하는 플러그인"에 답합니다. [`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)는 Claude Code 및 Cowork에서 플러그인당 일일 설치 및 호출 수를 반환하며, 사용자, RBAC 그룹 또는 제품별로 그룹화할 수 있습니다.

플러그인 이름 없이 Anthropic에 도달하는 플러그인 활동은 하나의 집계 `third-party` 행에 나타납니다. [텔레메트리에서 플러그인 찾기](/docs/ko/plugins/security#find-plugins-in-telemetry)는 Claude Code가 이름으로 보고하는 플러그인을 말합니다.

`read:analytics` 범위가 있는 API 키로 요청을 인증합니다. Primary Owner는 [프로그래밍 방식으로 데이터 액세스](/docs/ko/analytics#access-data-programmatically)에서 설명한 대로 이를 생성합니다.

매개변수 및 응답 필드는 [엔드포인트 참조](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)를 참조합니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인을 evals로 테스트](/docs/ko/plugin-evals): 비용뿐만 아니라 플러그인이 Claude를 얼마나 안정적으로 조종하는지 측정합니다
* [Always-on 수치 낮추기](#lower-the-always-on-figure): 플러그인의 per-turn 비용을 줄이기 위해 플러그인에서 변경할 사항
* [플러그인 보안 및 신뢰](/docs/ko/plugins/security#find-plugins-in-telemetry): 어떤 텔레메트리 필드가 플러그인 이름을 전달하고 언제 수정되는지
* [사용량 모니터링](/docs/ko/monitoring-usage): 전체 OpenTelemetry 이벤트 참조
