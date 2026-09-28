> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 비용 및 사용량 추적

> Claude Agent SDK를 사용하여 토큰 사용량을 추적하고, 비용을 예측하며, 프롬프트 캐싱을 구성하는 방법을 알아봅니다.

Claude Agent SDK는 Claude와의 각 상호작용에 대한 상세한 토큰 사용량 정보를 제공합니다. 이 가이드는 사용량을 적절히 추적하고 비용 보고를 이해하는 방법을 설명하며, 특히 병렬 도구 사용 및 다단계 대화를 다룰 때 유용합니다.

완전한 API 문서는 [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript) 및 [Python SDK 참조](/docs/ko/agent-sdk/python)를 참조하십시오.

<Warning>
  `total_cost_usd` 및 `costUSD` 필드는 클라이언트 측 추정값이며, 권위 있는 청구 데이터가 아닙니다. SDK는 빌드 시간에 번들로 제공되는 가격 표에서 로컬로 계산하거나, [`modelPricing`](/docs/ko/settings-reference#modelpricing) 표가 적용 중인 경우를 제외하고 계산합니다. 다음의 경우에 실제 청구 금액과 차이가 날 수 있습니다:

  * 가격 변경
  * 설치된 SDK 버전이 모델을 인식하지 못함
  * 클라이언트가 모델링할 수 없는 청구 규칙 적용

  SDK가 모델링하는 청구 규칙 중 하나는 [데이터 거주지 가격 책정](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing)입니다. 응답의 `usage`가 `inference_geo: "us"`를 보고할 때, SDK는 해당 응답의 토큰 정가에 1.1을 곱합니다. 웹 검색과 같은 요청별 수수료는 곱해지지 않습니다. TypeScript Agent SDK v0.3.239 이상 또는 Python Agent SDK v0.2.144 이상이 필요합니다.

  이러한 필드는 개발 통찰력 및 대략적인 예산 책정을 위해 사용하십시오. 권위 있는 청구를 위해서는 [사용량 및 비용 API](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) 또는 [Claude 콘솔](https://platform.claude.com/usage)의 사용량 페이지를 사용하십시오. 이러한 필드에서 최종 사용자에게 청구하거나 재정적 결정을 내리지 마십시오.
</Warning>

<h2 id="understand-token-usage">
  토큰 사용량 이해하기
</h2>

TypeScript 및 Python SDK는 다양한 필드 이름으로 동일한 사용량 데이터를 제공합니다:

* **TypeScript**는 각 어시스턴트 메시지(`message.message.id`, `message.message.usage`)에 대한 단계별 토큰 분석, 결과 메시지의 `modelUsage`를 통한 모델별 비용, 그리고 결과 메시지의 누적 합계를 제공합니다.
* **Python**은 각 어시스턴트 메시지에 대한 단계별 토큰 분석을 `message.usage` 및 `message.message_id`로 제공하고, 결과 메시지의 `model_usage`를 통한 모델별 비용, 그리고 결과 메시지의 누적 합계를 `total_cost_usd`로 제공합니다.

두 SDK 모두 동일한 기본 비용 모델을 사용하며 동일한 세분성을 제공합니다. 차이점은 필드 이름 지정과 단계별 사용량이 중첩되는 위치입니다.

비용 추적은 SDK가 사용량 데이터를 어떻게 범위 지정하는지 이해하는 것에 달려 있습니다:

* **`query()` 호출:** SDK의 `query()` 함수 한 번의 호출입니다. 단일 호출은 여러 단계를 포함할 수 있습니다: Claude가 응답하고, 도구를 사용하고, 결과를 얻고, 다시 응답합니다. 각 호출은 끝에 하나의 [`result`](/docs/ko/agent-sdk/typescript#sdkresultmessage) 메시지를 생성합니다. 단, [스트리밍 입력 모드](/docs/ko/agent-sdk/streaming-vs-single-mode)에서는 하나의 `query()` 호출이 여러 사용자 턴을 수행하며 각 턴은 자신의 `result` 메시지를 내보냅니다.
* **Step:** `query()` 호출 내의 단일 요청/응답 사이클입니다. 각 단계는 토큰 사용량이 있는 어시스턴트 메시지를 생성합니다.
* **Session:** 세션 ID로 연결된 일련의 `query()` 호출입니다(`resume` 옵션 사용). 재개된 호출의 결과는 해당 호출 자신의 비용이 아닌 세션 전체의 지출을 보고합니다. [여러 호출에 걸쳐 비용 누적하기](#accumulate-costs-across-multiple-calls)에서 합계가 어떻게 이월되는지 확인하십시오.

다음 다이어그램은 단일 `query()` 호출의 메시지 스트림을 보여주며, 각 단계에서 토큰 사용량이 보고되고 끝에 누적 추정치가 표시됩니다:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" className="dark:hidden" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/message-usage-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=8ea95085abc0a6b7f55ecef498bd4d14" className="hidden dark:block" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow-dark.svg" />

<Steps>
  <Step title="각 단계는 어시스턴트 메시지를 생성합니다">
    Claude가 응답할 때, 하나 이상의 어시스턴트 메시지를 보냅니다. TypeScript에서 각 어시스턴트 메시지는 `id`와 토큰 개수(`input_tokens`, `output_tokens`)가 있는 [`usage`](https://platform.claude.com/docs/en/api/messages) 객체를 포함하는 중첩된 `BetaMessage`(`message.message`를 통해 액세스)를 포함합니다. Python에서 `AssistantMessage` 데이터클래스는 `message.usage` 및 `message.message_id`를 통해 동일한 데이터를 직접 노출합니다. Claude가 한 턴에서 여러 도구를 사용할 때, 해당 턴의 모든 메시지는 동일한 ID를 공유하므로 중복 계산을 피하기 위해 ID로 중복 제거합니다.
  </Step>

  <Step title="결과 메시지는 누적 추정치를 제공합니다">
    `query()` 호출이 완료되면, SDK는 `total_cost_usd` 및 누적 `usage`가 있는 결과 메시지를 내보냅니다. TypeScript에서는 [`SDKResultMessage`](/docs/ko/agent-sdk/typescript#sdkresultmessage)로 입력되고 Python에서는 [`ResultMessage`](/docs/ko/agent-sdk/python#resultmessage)로 입력됩니다. 예상 합계만 필요한 경우, 단계별 사용량을 무시하고 이 단일 값을 읽을 수 있습니다.

    여러 개의 독립적인 `query()` 호출을 수행하는 경우, 각 결과는 해당 개별 호출의 비용만 반영합니다. 세션을 재개하는 호출도 세션의 이전 지출을 계산합니다.

    스트리밍 입력 모드에서는 각 턴이 자신의 결과 메시지를 내보냅니다. 해당 모드에서 호출 합계를 읽는 방법은 [스트리밍 입력 모드에서 비용 추적](#track-costs-in-streaming-input-mode)을 참조하십시오.
  </Step>
</Steps>

<h2 id="track-costs-in-streaming-input-mode">
  스트리밍 입력 모드에서 비용 추적
</h2>

[스트리밍 입력 모드](/docs/ko/agent-sdk/streaming-vs-single-mode)에서 하나의 `query()` 호출은 여러 사용자 턴을 포함하며 각 턴은 자신의 결과 메시지를 내보냅니다. 결과 필드는 범위가 다릅니다:

* **`usage`**: 해당 턴만 포함하며, 그 내에서도 주 에이전트 루프만 포함하고 실행한 서브에이전트는 포함하지 않습니다.
* **`total_cost_usd` 및 `modelUsage`, 또는 Python의 `model_usage`**: 지금까지 전체 호출에 대한 누적 합계를 나타내며, 호출이 세션을 재개할 때 복원된 모든 지출을 포함합니다.

앱이 `/clear`, `/reset`, 또는 `/new`를 보내지 않는 호출에서는 결과 전체를 합산하는 대신 최신 결과에서 호출 합계를 읽습니다.

누적 합계는 앱이 이 세 명령 중 하나를 보낼 때마다 다시 시작되며, `query()` 호출 내에서는 다른 것이 이를 재설정하지 않습니다. 회계 처리에 중요한 세 가지 결과는 다음과 같습니다:

* **`/clear` 턴의 자체 결과**: 재설정 이후 실행된 것만 포함하며 새로운 `session_id`를 전달합니다.
* **이후의 모든 결과**: 해당 재설정부터 계속 계산합니다.
* **각 `/clear` 이전의 마지막 결과**: 이전 재설정 이후의 턴에 대한 합계를 보유합니다.

전체 호출을 합산하려면 각 `/clear` 이전의 마지막 결과를 호출의 최종 결과에 더합니다. `/clear` 턴의 자체 결과를 포함한 다른 모든 결과는 이후 결과로 대체됩니다.

TypeScript에서 SDK는 각 재설정 시 [`SDKConversationResetMessage`](/docs/ko/agent-sdk/typescript#sdkconversationresetmessage)도 내보내므로 스트림에서 재설정을 감지할 수 있습니다. Python에서 SDK는 마찬가지로 `ConversationResetMessage`를 내보냅니다. Python SDK v0.2.137 이전에는 Python 반복자가 해당 메시지를 삭제했으므로 해당 버전에서는 앱이 보내는 `/clear` 턴에서 재설정을 직접 계산합니다.

`maxBudgetUsd`(TypeScript) 또는 `max_budget_usd`(Python)는 호출 자체의 지출만 계산합니다: 재개된 세션에서 복원된 합계는 이에 대해 계산되지 않으며, `/clear`는 예산을 다시 시작합니다.

<h2 id="get-the-total-cost-of-a-query">
  쿼리의 총 비용 얻기
</h2>

결과 메시지는 TypeScript에서 [`SDKResultMessage`](/docs/ko/agent-sdk/typescript#sdkresultmessage)로, Python에서 [`ResultMessage`](/docs/ko/agent-sdk/python#resultmessage)로 타입이 지정되며, `query()` 호출에 대한 에이전트 루프의 끝을 표시합니다. 이 메시지에는 해당 호출의 모든 단계에 걸친 누적 예상 비용인 `total_cost_usd`가 포함됩니다. 세션을 재개하는 호출도 세션의 이전 지출을 계산합니다. 값을 읽을 때 두 가지 주의사항이 적용됩니다:

* Python에서는 필드가 선택적으로 타입이 지정되므로, 읽기 전에 `None`이 아닌지 확인하십시오.
* 성공 및 오류 결과 모두 이를 포함하지만, [세션 충돌](#recover-totals-after-a-session-crash) 후의 최종 결과는 이를 0으로 포함할 수 있습니다.

스트리밍 입력 모드에서는 [스트리밍 입력 모드에서 비용 추적](#track-costs-in-streaming-input-mode)에 설명된 대로 호출 총액을 읽으십시오.

세 가지 결과 수준 필드는 에이전트가 [서브에이전트](/docs/ko/agent-sdk/subagents)를 생성할 때 계산하는 내용이 다릅니다. 전체 트리 토큰 회계를 위해 `modelUsage` 또는 Python의 `model_usage`를 사용하십시오. `usage` 필드는 중첩이 발생하는 즉시 과소 계산됩니다.

| 필드                           | 서브에이전트 활동                                          |
| ---------------------------- | -------------------------------------------------- |
| `usage`                      | 제외됨. 최상위 에이전트 루프만 계산하므로 서브에이전트 내에서 소비된 토큰은 추가되지 않음 |
| `total_cost_usd`             | 포함됨. 최상위 루프와 함께 서브에이전트 요청을 계산                      |
| `modelUsage` / `model_usage` | 포함됨. 최상위 루프와 함께 서브에이전트 요청을 계산하며, 모델별로 분류됨          |

[단일 메시지 입력 모드](/docs/ko/agent-sdk/streaming-vs-single-mode#single-message-input)에서, 최종 턴이 끝날 때 백그라운드 서브에이전트가 여전히 실행 중인 경우, Claude Code는 [exit에서의 백그라운드 작업](/docs/ko/headless#background-tasks-at-exit)에 설명된 상한선까지 이들을 기다린 후 결과를 내보냅니다. 결과의 `total_cost_usd`, `duration_api_ms`, 및 `modelUsage` 또는 Python의 `model_usage`는 해당 대기 중에 수행된 작업을 포함합니다.

다음 예제는 `query()` 호출의 메시지 스트림을 반복하고 `result` 메시지가 도착할 때 총 비용을 인쇄합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({ prompt: "Summarize this project" })) {
      if (message.type === "result") {
        console.log(`Total cost: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, it still carried total_cost_usd and the
    // branch above has already run; connection or process failures yield
    // no result message.
    console.error(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      try:
          async for message in query(prompt="Summarize this project"):
              if isinstance(message, ResultMessage):
                  print(f"Total cost: ${message.total_cost_usd or 0}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the branch above has already run;
          # connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

서브에이전트가 `total_cost_usd`에 추가할 수 있는 금액을 제한하려면, 쿼리에서 [깊이, 동시성 및 지출 제한](/docs/ko/agent-sdk/subagents#cap-subagent-depth-concurrency-and-spend)을 설정하십시오.

<h2 id="track-per-step-and-per-model-usage">
  단계별 및 모델별 사용량 추적
</h2>

이 섹션의 예제는 TypeScript 필드 이름을 사용합니다. Python에서는 단계별 사용량에 대해 [`AssistantMessage.usage`](/docs/ko/agent-sdk/python#assistantmessage) 및 `AssistantMessage.message_id`에 해당하고, 모델별 분석에 대해 [`ResultMessage.model_usage`](/docs/ko/agent-sdk/python#resultmessage)에 해당합니다.

<h3 id="track-per-step-usage">
  단계별 사용량 추적
</h3>

각 어시스턴트 메시지에는 `id` 및 토큰 수가 있는 `usage` 객체와 함께 중첩된 `BetaMessage`(`message.message`를 통해 액세스)가 포함되어 있습니다. Claude가 도구를 병렬로 사용할 때 여러 메시지가 동일한 `id`를 동일한 사용량 데이터와 공유합니다. 이미 계산한 ID를 추적하고 중복을 건너뛰어 부풀려진 합계를 피합니다.

<Warning>
  중복 제거된 단계별 값은 입력 및 캐시 토큰에 대해 정확합니다. 단계별 `output_tokens`는 자리 표시자이므로 [결과 메시지에서 출력 토큰을 읽으십시오](#read-output-tokens-from-the-result-message).
</Warning>

다음 예제는 모든 단계에서 입력 토큰을 누적하고, 각 고유한 메인 루프 메시지 ID를 한 번만 계산하며 서브에이전트 메시지를 건너뛰고, 메인 루프를 포함하는 결과 메시지에서 출력 합계를 읽습니다:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let resultOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant" && !message.parent_tool_use_id) {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
      }
    }
    if (message.type === "result") {
      // Per-step output_tokens is a placeholder; the result message
      // carries the accumulated output total.
      resultOutputTokens = message.usage.output_tokens;
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // input total below still reflects the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${resultOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  모델별 사용량 분석
</h3>

결과 메시지에는 [`modelUsage`](/docs/ko/agent-sdk/typescript#modelusage)가 포함되어 있으며, 이는 모델 이름을 모델별 토큰 수 및 비용에 매핑합니다. 이는 여러 모델을 실행할 때(예: 서브에이전트용 Haiku 및 메인 에이전트용 Opus) 토큰이 어디로 가는지 확인하려는 경우에 유용합니다.

각 항목의 `costBasis`는 해당 모델의 최신 요청에 가격을 책정한 가격 테이블을 나타냅니다: 정가의 경우 `list`, [`modelPricing`](/docs/ko/settings-reference#modelpricing) 테이블의 경우 `managed`, 또는 모델 ID와 일치하는 항목이 없을 때 `unknown`입니다. 이 필드는 Claude Code v2.1.246 이상이 필요합니다.

다음 예제는 쿼리를 실행하고 사용된 각 모델에 대한 비용 및 토큰 분석을 출력합니다:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type !== "result") continue;

    for (const [modelName, usage] of Object.entries(message.modelUsage)) {
      console.log(`${modelName}: $${usage.costUSD.toFixed(4)}`);
      console.log(`  Input tokens: ${usage.inputTokens}`);
      console.log(`  Output tokens: ${usage.outputTokens}`);
      console.log(`  Cache read: ${usage.cacheReadInputTokens}`);
      console.log(`  Cache creation: ${usage.cacheCreationInputTokens}`);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result. If the
  // failure was an error result, the per-model breakdown above has already
  // printed; connection or process failures yield no result message.
  console.error(`Session ended with an error: ${error}`);
}
```

<h2 id="accumulate-costs-across-multiple-calls">
  여러 호출에 걸쳐 비용 누적하기
</h2>

각 `query()` 호출은 자체 결과에서 `total_cost_usd`를 반환합니다. 값을 결합하는 방식은 호출이 세션을 공유하는지 여부에 따라 달라집니다:

* **독립적인 호출, `resume` 또는 `continue` 옵션 없음**: 각 결과는 자체 호출만 포함하므로, 아래 예제처럼 총합을 직접 더해야 합니다.
* **동일한 세션을 재개하는 호출**: Claude Code는 프로세스가 정상적으로 종료될 때 세션의 총합을 [트랜스크립트](/docs/ko/sessions#where-transcripts-are-stored)에 저장하고, 나중에 호출이 세션을 재개하거나 포크할 때 복구합니다. 각 결과는 이미 세션의 이전 지출을 포함합니다. 세션 총합을 위해 최신 결과를 읽으십시오. 결과를 합산하면 복구된 지출이 중복 계산됩니다. v2.1.277 이전에는 SDK 또는 `claude -p`를 통해 재개한 세션이 총합을 0에서 시작했으므로, 각 호출의 결과는 해당 호출만 포함했습니다.

스트리밍 입력 모드에서는 [스트리밍 입력 모드에서 비용 추적하기](#track-costs-in-streaming-input-mode)에 설명된 대로 각 호출의 총합을 읽으십시오. 충돌로 끝난 호출의 경우 [세션 충돌 후 총합 복구하기](#recover-totals-after-a-session-crash)를 참조하십시오.

다음 예제는 두 개의 `query()` 호출을 순차적으로 실행하고, 각 호출의 `total_cost_usd`를 누적 총합에 더하며, 호출별 비용과 결합된 비용을 모두 출력합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 여러 query() 호출에 걸쳐 누적 비용 추적
  let totalSpend = 0;

  const prompts = [
    "Read the files in src/ and summarize the architecture",
    "List all exported functions in src/auth.ts"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({ prompt })) {
        if (message.type === "result") {
          totalSpend += message.total_cost_usd;
          console.log(`This call: $${message.total_cost_usd}`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, this call's cost was already counted;
      // connection or process failures yield no result message. Continue
      // with the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }

  console.log(`Total spend: $${totalSpend.toFixed(4)}`);
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      # 여러 query() 호출에 걸쳐 누적 비용 추적
      total_spend = 0.0

      prompts = [
          "Read the files in src/ and summarize the architecture",
          "List all exported functions in src/auth.ts",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt):
                  if isinstance(message, ResultMessage):
                      cost = message.total_cost_usd or 0
                      total_spend += cost
                      print(f"This call: ${cost}")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. If
              # the failure was an error result, this call's cost was already
              # counted; connection or process failures yield no result message.
              # Continue with the next prompt.
              print(f"Call failed: {error}")

      print(f"Total spend: ${total_spend:.4f}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="handle-errors-caching-and-output-token-counts">
  오류, 캐싱 및 출력 토큰 수 처리
</h2>

정확한 비용 추적을 위해 어시스턴트 메시지의 자리 표시자 출력 수, 실패한 대화가 소비한 토큰, 캐시 토큰 가격을 고려해야 합니다.

<h3 id="read-output-tokens-from-the-result-message">
  결과 메시지에서 출력 토큰 읽기
</h3>

Claude Code는 응답이 시작될 때 API가 보고한 사용량에서 각 어시스턴트 메시지를 구축하므로, 메시지의 `output_tokens`은 응답이 생성되기 전에 `message_start`에서 API가 보고한 수만 포함합니다. 하나의 API 응답은 여러 어시스턴트 메시지를 생성할 수 있으며, 각각은 동일한 자리 표시자를 포함합니다.

API는 응답 끝에서 실제 출력 수를 보고하며, Claude Code는 이를 결과 메시지에 추가합니다. 결과의 `usage`에서 또는 모델별 분석을 위해 `modelUsage`에서 출력 토큰을 읽습니다.

응답의 출력 수가 스트리밍되는 동안 증가하는 것을 보려면 `includePartialMessages` 또는 Python의 `include_partial_messages`를 설정하고, 각 `message_delta` 스트림 이벤트에서 `usage`를 읽습니다. TypeScript에서는 [`SDKPartialAssistantMessage`](/docs/ko/agent-sdk/typescript#sdkpartialassistantmessage)로, Python에서는 [`StreamEvent`](/docs/ko/agent-sdk/python#streamevent)로 입력됩니다.

<h3 id="track-costs-on-failed-conversations">
  실패한 대화의 비용 추적
</h3>

성공 및 오류 결과 메시지 모두 `usage` 및 `total_cost_usd`를 포함합니다. Python에서는 두 필드 모두 선택 사항으로 입력되므로, 읽기 전에 `None`이 아닌지 확인해야 합니다.

대화가 중간에 실패하면 실패 지점까지 토큰을 소비했습니다. 모든 결과 메시지에서 비용 데이터를 읽습니다. `subtype`이 `success`이든 오류 하위 유형 중 하나이든 상관없습니다. 일부 오류 결과에서 `usage`는 호출이 소비한 것보다 적게 보고합니다:

* **[세션 충돌 후](#recover-totals-after-a-session-crash) `error_during_execution`**: 모든 비용 필드가 0으로 설정될 수 있습니다.
* **`error_max_budget_usd`**: `usage`는 예산을 초과한 응답을 제외하지만, `total_cost_usd` 및 `modelUsage`는 포함합니다.

선택의 여지가 있을 때는 `usage`보다는 `total_cost_usd` 또는 `modelUsage`에서 계산합니다.

<h3 id="recover-totals-after-a-session-crash">
  세션 충돌 후 합계 복구
</h3>

Claude Code 프로세스가 충돌하면 최종 `error_during_execution` 결과를 내보내고 종료합니다. 단일 샷 및 스트리밍 입력 모드 모두에서 그렇습니다. 해당 결과는 0으로 설정된 `usage`, `total_cost_usd` 및 `modelUsage`를 포함할 수 있으므로, 이전에 도착한 것에서 호출의 합계를 복구합니다. 1단계는 이전 결과가 존재할 때마다 전체 합계를 복구합니다. 2단계의 폴백은 메인 루프의 입력 및 캐시 토큰만 복구합니다.

1. 충돌 전 턴의 결과를 사용합니다. 스트리밍 입력 모드에서는 [스트리밍 입력 모드에서 비용 추적](#track-costs-in-streaming-input-mode)에 설명된 누적 합계를 포함합니다. 해당 결과가 도움이 될 수 없을 때 2단계로 이동합니다:
   * 호출이 단일 샷이므로 이전 결과가 없습니다.
   * 충돌이 첫 번째 턴에서 발생했습니다.
   * 충돌 전 턴이 `/clear` 자체였으므로, 그 결과는 재설정만 포함합니다.
2. 대신 어시스턴트 메시지의 `usage`를 합산합니다. 각 API 응답을 한 번씩 계산합니다. [단계별 사용량 추적](#track-per-step-usage) 예제가 하는 것처럼 말입니다. 단일 샷 모드에서는 모두 합산합니다. 스트리밍 입력 모드에서는 마지막 결과 이후에 도착한 것들을 합산합니다. 이는 메인 루프의 입력 및 캐시 토큰을 제공합니다. 서브에이전트 사용량은 이 방법으로 복구할 수 없으며, 출력 토큰이나 USD 비용도 복구할 수 없습니다. [단계별 `output_tokens`은 자리 표시자](#read-output-tokens-from-the-result-message)이기 때문입니다.

<h3 id="track-cache-tokens">
  캐시 토큰 추적
</h3>

Agent SDK는 반복된 콘텐츠의 비용을 줄이기 위해 [프롬프트 캐싱](https://platform.claude.com/docs/en/build-with-claude/prompt-caching)을 자동으로 사용합니다. 캐싱을 직접 구성할 필요가 없습니다. 사용량 객체에는 캐시 추적을 위한 두 가지 추가 필드가 포함됩니다:

* `cache_creation_input_tokens`: 새 캐시 항목을 만드는 데 사용된 토큰(표준 입력 토큰보다 높은 요금으로 청구됨).
* `cache_read_input_tokens`: 기존 캐시 항목에서 읽은 토큰(감소된 요금으로 청구됨).

캐싱 절감액을 이해하려면 이들을 `input_tokens`과 별도로 추적합니다. TypeScript에서는 이러한 필드가 [`Usage`](/docs/ko/agent-sdk/typescript#usage) 객체에 입력됩니다. Python에서는 [`ResultMessage.usage`](/docs/ko/agent-sdk/python#resultmessage) 딕셔너리의 키로 나타납니다(예: `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  프롬프트 캐시 TTL을 1시간으로 연장
</h3>

사용자의 턴은 [메인 대화 TTL 버킷](/docs/ko/prompt-caching#which-ttl-each-request-gets)에 속하며, Claude Code가 인라인으로 실행하는 헬퍼와 함께 있습니다. Claude Code가 해당 대화 외부에서 수행하는 요청(예: [서브에이전트](/docs/ko/agent-sdk/subagents))은 [별도의 TTL 제어](/docs/ko/prompt-caching#choose-the-ttl-yourself)를 가집니다.

사용자의 턴에 대한 캐시 항목은 API 키로 인증하거나 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws)에서 실행할 때 기본적으로 5분 TTL을 사용합니다. 워크로드가 동일한 시스템 프롬프트 및 컨텍스트에 대해 많은 짧은 세션을 실행하고 세션 간에 5분보다 긴 간격이 있으면 캐시가 세션 간에 만료되고 각 새 세션은 전체 입력 가격을 지불합니다.

캐시 쓰기에 1시간 TTL을 요청하려면 [`ENABLE_PROMPT_CACHING_1H`](/docs/ko/env-vars) 환경 변수를 설정합니다. 셸 또는 컨테이너 환경에서 내보내거나 `options.env`를 통해 전달할 수 있습니다.

다음 예제는 Amazon Bedrock에서 실행되는 에이전트에 대해 1시간 TTL을 활성화합니다. `CLAUDE_CODE_USE_BEDROCK`을 설정하므로 [Amazon Bedrock](/docs/ko/amazon-bedrock)에 대한 작동하는 AWS 자격 증명이 필요합니다. 없으면 쿼리가 실패합니다.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import ClaudeAgentOptions, query
  import asyncio


  async def main():
      options = ClaudeAgentOptions(
          env={
              "CLAUDE_CODE_USE_BEDROCK": "1",
              "ENABLE_PROMPT_CACHING_1H": "1",
          },
      )

      async for message in query(prompt="Summarize this project", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    env: {
      ...process.env,
      CLAUDE_CODE_USE_BEDROCK: "1",
      ENABLE_PROMPT_CACHING_1H: "1",
    },
  };

  for await (const message of query({ prompt: "Summarize this project", options })) {
    console.log(message);
  }
  ```
</CodeGroup>

1시간 TTL의 캐시 쓰기는 5분 쓰기보다 높은 요금으로 청구되므로, 이를 활성화하면 더 높은 쓰기 비용으로 더 많은 캐시 읽기를 거래합니다. 자세한 내용은 [프롬프트 캐싱 가격](https://platform.claude.com/docs/en/build-with-claude/prompt-caching)을 참조하세요. 계획의 포함된 사용량 내의 Claude 구독에서는 이 변수를 설정하지 않고도 사용자의 턴에 대해 1시간 TTL을 얻으며, Claude Code가 옆에 만드는 일부 헬퍼 요청에서도 그렇습니다. Claude Code는 [사용 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)을 사용하기 시작하면 해당 턴을 5분 TTL로 떨어뜨립니다.

`ENABLE_PROMPT_CACHING_1H`은 두 버킷의 모든 요청에 대해 1시간 TTL을 요청합니다. 각 버킷에 대해 TTL을 별도로 선택하려면 대신 이러한 제어를 사용합니다. 각각은 `5m` 또는 `1h`을 사용하며 `ENABLE_PROMPT_CACHING_1H`보다 우선합니다:

* 메인 대화: `CLAUDE_CODE_PROMPT_CACHE_TTL` [환경 변수](/docs/ko/env-vars) 또는 [`promptCacheTtl`](/docs/ko/settings-reference#promptcachettl) 설정
* 그 외 모든 것: `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` 환경 변수 또는 [`subagentPromptCacheTtl`](/docs/ko/settings-reference#subagentpromptcachettl) 설정

`promptCacheTtl`을 `1h`로 설정하면 사용 크레딧을 사용하는 동안 메인 대화에서 1시간 캐시를 유지합니다. 전체 우선 순위 순서는 [TTL 직접 선택](/docs/ko/prompt-caching#choose-the-ttl-yourself)을 참조하세요.

<h2 id="related-documentation">
  관련 문서
</h2>

* [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript) - 완전한 API 문서
* [SDK 개요](/docs/ko/agent-sdk/overview) - SDK 시작하기
* [SDK 권한](/docs/ko/agent-sdk/permissions) - 도구 권한 관리
