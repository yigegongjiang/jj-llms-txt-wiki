> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 어려운 결정을 조언자 도구로 에스컬레이션하기

> 주 모델을 더 강력한 조언자 모델과 쌍으로 만들어 Claude가 작업 중 핵심 순간에 조언자를 참고하도록 합니다.

<Note>
  조언자 도구는 실험적이며 Anthropic API가 필요합니다. Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform 또는 Microsoft Foundry에서는 사용할 수 없습니다. 동작, 가격 책정 및 가용성은 변경될 수 있습니다.
</Note>

조언자 도구를 사용하면 Claude가 작업 중 핵심 순간(예: 접근 방식을 확정하기 전, 반복되는 오류에 막혔을 때, 작업 완료를 선언하기 전)에 일반적으로 더 강력한 두 번째 모델을 참고할 수 있습니다. 조언자는 모든 도구 호출 및 결과를 포함한 전체 대화를 받고 Claude가 계속하기 전에 적용할 지침을 반환합니다.

조언자는 Anthropic의 인프라에서 서버 측으로 실행되며 [서버 도구](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)로 구독 및 API 청구 계정 모두에서 사용할 수 있습니다. 조언자로 작동할 모델을 선택하고 Claude가 호출 시기를 결정합니다.

이 페이지에서는 조언자를 활성화하는 방법, 허용되는 모델 쌍, Claude가 상담 중에 표시하는 내용, 조언자 사용이 청구되는 방식을 다룹니다.

<h2 id="when-to-use-the-advisor">
  조언자를 사용할 때
</h2>

조언자는 대부분의 턴이 일상적이지만 계획 품질이 결과를 결정하는 길고 다단계 작업에 적합합니다. 예시로는 대규모 리팩토링, 오류가 계속 반복되는 디버깅 세션, Claude가 완료를 선언하기 전에 독립적으로 확인하고 싶은 작업이 있습니다.

계획할 것이 거의 없는 짧은 작업이나 모든 턴에 가장 강력한 모델이 필요한 작업에서는 가치가 적습니다. 이러한 경우 [주 모델을 전환](/docs/ko/model-config#setting-your-model)하거나 [조언자와 opusplan 및 서브에이전트 비교](#compare-with-related-features)를 참고하여 두 번째 의견을 얻는 다른 방법을 확인하세요.

<h2 id="enable-the-advisor">
  조언자 활성화
</h2>

조언자 모델을 세 가지 방법으로 설정할 수 있습니다:

* **`/advisor` 명령**: 세션 중간에 조언자를 설정 또는 변경하고 기본값으로 저장
* **`advisorModel` 설정**: [설정 파일](/docs/ko/settings)에서 지속적인 기본값 구성
* **`--advisor` 플래그**: 시작 시 단일 세션에 대해 조언자 설정

이 중 하나가 조언자 모델을 설정하면 주 모델이 [이를 지원](#choose-an-advisor-model)하는 세션에 대해 조언자가 활성화됩니다. 사용을 중지하려면 [조언자 끄기](#turn-the-advisor-off)를 참고하세요.

<Note>
  Fable 5를 조언자로 사용하려면 Claude Code v2.1.170 이상과 조직의 [Fable 5 액세스](/docs/ko/model-config#work-with-fable-5)가 필요합니다.
</Note>

<h3 id="use-the-/advisor-command">
  `/advisor` 명령 사용
</h3>

인수 없이 `/advisor`를 실행하여 사용 가능한 조언자 모델을 나열하는 선택기를 열거나 모델을 직접 전달하세요:

```
/advisor opus
```

선택 항목은 사용자 설정의 `advisorModel`에 저장되고 세션 전체에서 유지됩니다. 조직의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록이 저장된 조언자 모델을 제외하면 `/advisor`로 허용된 모델을 선택할 때까지 조언자가 호출되지 않습니다. 현재 주 모델이 조언자를 지원하지 않으면 선택 항목이 여전히 저장되고 [`/model`](/docs/ko/model-config#setting-your-model)을 사용하여 [호환되는 주 모델](#choose-an-advisor-model)로 전환할 때 활성화됩니다.

<h3 id="set-advisormodel-in-settings">
  설정에서 `advisorModel` 설정
</h3>

세션을 열지 않고 조언자를 기본값으로 구성하려면 설정 파일에서 설정하세요:

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  `--advisor` 플래그 사용
</h3>

저장된 설정을 변경하지 않고 단일 세션에 대해 조언자를 설정하려면 플래그를 사용하여 시작하세요:

```bash theme={null}
claude --advisor opus
```

플래그는 해당 세션에 대해 `advisorModel` 설정보다 우선합니다. 세션의 주 모델이 조언자를 지원하지 않으면 오류로 종료되거나, 요청된 조언자 모델이 조직의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록에서 제외되면 오류로 종료됩니다.

<h2 id="choose-an-advisor-model">
  조언자 모델 선택
</h2>

조언자는 주 모델 이상의 기능을 가져야 합니다. 각 주 모델에 대해 허용되는 조언자는 다음과 같습니다:

| 주 모델                | 허용되는 조언자                  | 참고                                                                                                          |
| ------------------- | ------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Haiku 4.5           | Fable, Opus, Sonnet       | Haiku는 조언자를 호출할 수 있지만 조언자로 작동할 수 없습니다                                                                       |
| Sonnet 4.6          | Fable, Opus, Sonnet       |                                                                                                             |
| Sonnet 5            | Fable, Opus, Sonnet 5     | Sonnet 4.6 조언자는 거부됩니다                                                                                       |
| Opus 4.6            | Fable, Opus, Sonnet 5     | Sonnet 5와 Opus 4.6은 동등한 기능으로 평가되므로 Opus 4.6 주 모델은 Sonnet 5 조언자를 허용합니다                                       |
| Opus 4.7 이상         | Fable, Opus 4.7, Opus 4.8 | Opus 4.7과 Opus 4.8은 동등한 기능으로 평가되므로 둘 다 다른 하나를 조언자로 허용합니다. Opus 4.6 또는 Sonnet 5 조언자를 가진 Opus 4.7 주 모델은 거부됩니다 |
| Fable 5 (v2.1.170+) | Fable                     | Opus 또는 Sonnet 조언자는 거부됩니다                                                                                   |

Fable 5는 주 모델로 작동하든 조언자로 작동하든 Claude Code v2.1.170 이상과 Fable 5 액세스가 필요합니다.

조언자를 `opus`, `sonnet` 또는 `fable`로 설정하세요. 이러한 별칭은 각 모델의 최신 버전으로 확인됩니다. `claude-opus-4-8`과 같은 전체 모델 ID를 전달할 수도 있습니다.

하위 에이전트는 구성된 조언자를 상속하고 자신의 모델에 대해 동일한 쌍 확인을 적용합니다.

Claude Code는 요청을 보내기 전에 쌍을 검증합니다:

* 조언자가 주 모델보다 기능이 낮으면 조언자가 주 모델의 요청에 첨부되지 않습니다. `/advisor` 명령 출력과 알림에 이것이 표시됩니다. 자신의 모델이 쌍 확인을 만족하는 하위 에이전트는 여전히 조언자를 사용할 수 있습니다.
* 주 모델 또는 조언자가 Claude Code가 인식하지 못하는 모델이면 조언자가 첨부되지 않습니다.

<h3 id="common-model-pairings">
  일반적인 모델 쌍
</h3>

허용되는 모든 쌍이 작동합니다. 이러한 조합은 비용과 기능을 다양한 방식으로 균형을 맞춥니다:

| 쌍                     | 사용 시기                                                                                                      |
| --------------------- | ---------------------------------------------------------------------------------------------------------- |
| Sonnet 주 + Opus 조언자   | Sonnet은 일상적인 작업을 처리하고 계획, 모호한 실패, 완료 확인을 Opus로 에스컬레이션합니다                                                   |
| Sonnet 주 + Fable 조언자  | 전체 Fable 5를 실행하지 않고 결정 지점에서 Fable 5 지침을 받습니다. v2.1.170 이상과 Fable 5 액세스가 필요합니다                              |
| Haiku 주 + Opus 조언자    | 강력한 계획을 갖춘 가장 저렴한 주 모델입니다. Haiku 단독보다 비용이 높지만 주 모델을 Sonnet 또는 Opus로 전환하는 것보다 낮을 것으로 예상됩니다                  |
| Opus 주 + Opus 조언자     | 두 번째 Opus가 첫 번째를 검토합니다. 비용보다 독립적인 확인이 더 중요한 높은 위험도 작업에 유용합니다                                               |
| Fable 주 + Fable 조언자   | Fable 5를 사용할 수 있을 때 가장 높은 기능 쌍(v2.1.170+). Fable은 Opus 및 Sonnet보다 높은 계층이므로 Fable 주 모델에 대해 유일하게 허용되는 조언자입니다 |
| Sonnet 주 + Sonnet 조언자 | 일상적인 간과를 포착하기 위한 저비용 두 번째 의견                                                                               |

<h2 id="when-claude-consults-the-advisor">
  Claude가 조언자를 참고할 때
</h2>

Claude는 조언자를 호출할 시기를 결정합니다. 접근 방식을 확정하기 전, 오류가 계속 반복될 때, 작업 완료를 선언하기 전에 참고하는 경향이 있지만 타이밍은 규칙 기반이 아닌 모델 기반입니다.

프롬프트에서 `consult the advisor before you continue`와 같이 다른 도구를 요청하는 것과 같은 방식으로 상담을 요청할 수 있습니다. 조언자 호출을 제한하거나 강제하는 설정은 없습니다. 작업 중에 Claude가 더 자주 또는 덜 자주 참고하기를 원하면 지침에서 말하세요.

<h2 id="what-you-see-during-a-session">
  세션 중에 표시되는 내용
</h2>

Claude가 조언자를 호출하면 대화 기록에 호출이 진행 중인 동안 조언자 모델 이름이 있는 `Advising` 줄이 표시됩니다. 결과가 반환되면 줄은 조언자가 대화를 검토했음을 확인합니다. `Ctrl+O`를 눌러 확장하고 조언자의 전체 지침을 읽으세요.

Claude는 일반적으로 조언자의 지침을 따르지만 자신의 증거가 특정 주장과 모순될 때 적응합니다. 권장 단계가 시도했을 때 실패하거나 파일 내용이 조언과 모순되면 Claude는 지침을 무조건 따르기보다는 충돌을 표시합니다.

조언자는 항상 전체 대화를 받고 Claude가 타이밍을 제어합니다. 더 많은 제어 또는 다른 구성을 원하면 [조언자와 서브에이전트 및 opusplan 비교](#compare-with-related-features)를 참고하세요.

<h2 id="cost">
  비용
</h2>

각 조언자 호출은 대화를 조언자 모델로 보내므로 주 모델 사용 외에도 조언자 모델의 요금으로 토큰을 소비합니다. API 청구를 사용하면 조언자 토큰은 조언자 모델의 입력 및 출력 요금으로 청구됩니다. 구독 계획에서 조언자 사용은 계획의 사용 한도에 포함됩니다.

Claude는 모든 턴이 아닌 결정 지점에서 조언자를 호출하므로 더 빠른 주 모델을 더 강력한 조언자와 쌍으로 만드는 것이 일반적으로 전체 강력한 모델을 실행하는 것보다 비용이 적습니다. 조언자 사용은 [`/usage`](/docs/ko/costs#track-your-costs)에 표시된 세션 합계에 포함됩니다.

조언자 토큰이 API 응답에서 보고되는 방식에 대해서는 Claude API 설명서의 [사용량 및 청구](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing)를 참고하세요.

<h2 id="impact-on-prompt-caching">
  프롬프트 캐싱에 미치는 영향
</h2>

세션 중간에 조언자를 활성화하거나 비활성화해도 주 모델의 [프롬프트 캐시](/docs/ko/prompt-caching)가 무효화되지 않습니다. [모델 또는 노력 수준 변경](/docs/ko/prompt-caching#actions-that-invalidate-the-cache)과 달리 `/advisor` 토글은 캐시된 접두사를 유지하고 조언자가 반환한 지침은 나중의 턴에서 대화 기록의 일부로 캐시됩니다.

조언자 모델 자체의 대화 읽기는 캐시되지 않습니다. 각 조언자 호출은 전체 대화를 새로 처리하며 호출 간에 재사용이 없습니다.

<h2 id="requirements">
  요구 사항
</h2>

조언자 도구는 다음 모두를 요구합니다:

* **Anthropic API만**: 조언자는 서버 실행 도구입니다. Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform 또는 Microsoft Foundry에서는 사용할 수 없습니다. `ANTHROPIC_BASE_URL`로 구성된 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 가용성은 게이트웨이가 요청을 Anthropic API로 그대로 전달하는지 여부에 따라 달라집니다.
* **지원되는 주 모델**: Opus 4.6 이상, Sonnet 4.6 이상 또는 Haiku 4.5. Fable 5도 Claude Code v2.1.170 이상에서 적합합니다.

<h2 id="turn-the-advisor-off">
  조언자 끄기
</h2>

조언자 사용을 중지하고 저장된 `advisorModel`을 지우려면 `/advisor off`를 실행하거나 `/advisor` 선택기에서 **No advisor**를 선택하세요:

```
/advisor off
```

조언자 도구를 완전히 비활성화하려면 `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`을 설정하세요. `/advisor` 명령을 사용할 수 없게 되며 구성된 `advisorModel`은 무시됩니다. `--advisor` 플래그는 허용되지만 효과가 없습니다. 이를 전달하는 기존 스크립트는 오류 없이 계속 작동합니다. [환경 변수](/docs/ko/env-vars)를 참고하세요.

<h2 id="compare-with-related-features">
  관련 기능과 비교
</h2>

조언자는 모델 강점을 결합하는 여러 방법 중 하나입니다. 더 강력한 모델을 언제 포함할지에 따라 선택하세요.

| 접근 방식                                                 | 더 강력한 모델이 실행되는 시기                                                                                       | 시작 방식                       |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------- | --------------------------- |
| 조언자 도구                                                | 작업 중간의 결정 지점에서                                                                                          | Claude가 지침이 필요할 때 호출합니다     |
| [`opusplan`](/docs/ko/model-config#opusplan-model-setting) | [계획 모드 중 `availableModels`에서 허용될 때](/docs/ko/model-config#restrict-model-selection), 그 다음 실행을 위해 Sonnet으로 전환 | 계획 모드를 입력합니다                |
| [서브에이전트](/docs/ko/sub-agents#choose-a-model) (`model` 설정)  | 전체 위임된 부작업에 대해                                                                                          | Claude가 위임하거나 서브에이전트를 호출합니다 |
| [`/model`](/docs/ko/model-config#setting-your-model)       | 이후의 모든 턴에 대해                                                                                            | 모델을 전환합니다                   |

<h2 id="see-also">
  참고 항목
</h2>

* [모델 구성](/docs/ko/model-config): 모델 전환, 노력 수준 설정, `opusplan` 사용
* [비용 효과적으로 관리](/docs/ko/costs): 모델 전체의 토큰 사용량 추적
* [Claude API의 조언자 도구](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool): 기본 서버 도구 이해 또는 Messages API에서 직접 사용
* [조언자 전략](https://claude.com/blog/the-advisor-strategy): 빠른 주 모델을 더 강력한 조언자와 쌍으로 만드는 이유
