> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# コストと使用状況を追跡する

> Claude Agent SDK でトークン使用状況を追跡し、コストを見積もり、プロンプトキャッシングを設定する方法を学びます。

Claude Agent SDK は、Claude との各インタラクションの詳細なトークン使用情報を提供します。このガイドでは、使用状況を適切に追跡し、特に並列ツール使用とマルチステップ会話を扱う場合のコスト報告を理解する方法について説明します。

完全な API ドキュメントについては、[TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript)および [Python SDK リファレンス](/docs/ja/agent-sdk/python)を参照してください。

<Warning>
  `total_cost_usd` および `costUSD` フィールドはクライアント側の推定値であり、信頼できる請求データではありません。SDK は、[`modelPricing`](/docs/ja/settings-reference#modelpricing) テーブルが有効でない限り、ビルド時にバンドルされた価格表からローカルで計算します。以下の場合に実際の請求額から乖離する可能性があります。

  * 価格が変更される
  * インストールされている SDK バージョンがモデルを認識しない
  * クライアントがモデル化できない請求ルールが適用される

  SDK がモデル化する請求ルールの 1 つは、[データレジデンシー価格](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing)です。レスポンスの `usage` が `inference_geo: "us"` を報告する場合、SDK はそのレスポンスのトークンのリスト価格に 1.1 を乗算します。Web 検索などのリクエストごとの料金は乗算されません。TypeScript Agent SDK v0.3.239 以降、または Python Agent SDK v0.2.144 以降が必要です。

  これらのフィールドは開発の洞察と概算予算作成に使用してください。信頼できる請求については、[使用状況とコスト API](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) または [Claude Console](https://platform.claude.com/usage) の使用状況ページを使用してください。これらのフィールドからエンドユーザーに請求したり、財務上の決定をトリガーしたりしないでください。
</Warning>

<h2 id="understand-token-usage">
  トークン使用量を理解する
</h2>

TypeScript と Python SDK は、異なるフィールド名で同じ使用量データを公開しています。

* **TypeScript** は、各アシスタントメッセージ（`message.message.id`、`message.message.usage`）でステップごとのトークン分解を提供し、結果メッセージの `modelUsage` 経由でモデルごとのコストを提供し、結果メッセージの累積合計を提供します。
* **Python** は、各アシスタントメッセージで `message.usage` と `message.message_id` としてステップごとのトークン分解を提供し、結果メッセージの `model_usage` 経由でモデルごとのコストを提供し、結果メッセージの `total_cost_usd` として累積合計を提供します。

両方の SDK は同じ基盤となるコストモデルを使用し、同じ粒度を公開しています。違いはフィールド命名とステップごとの使用量がネストされている場所にあります。

コスト追跡は、SDK がどのように使用量データをスコープするかを理解することに依存しています。

* **`query()` 呼び出し：** SDK の `query()` 関数の 1 回の呼び出し。単一の呼び出しは複数のステップを含むことができます。Claude が応答し、ツールを使用し、結果を取得し、再度応答します。各呼び出しは最後に 1 つの [`result`](/docs/ja/agent-sdk/typescript#sdkresultmessage) メッセージを生成します。ただし、[ストリーミング入力モード](/docs/ja/agent-sdk/streaming-vs-single-mode)では、1 つの `query()` 呼び出しが複数のユーザーターンを実行し、各ターンが独自の `result` メッセージを発行します。
* **ステップ：** `query()` 呼び出し内の単一のリクエスト/レスポンスサイクル。各ステップはトークン使用量を含むアシスタントメッセージを生成します。
* **セッション：** セッション ID でリンクされた一連の `query()` 呼び出し（`resume` オプションを使用）。セッションを再開した呼び出しの結果は、その呼び出し自体だけでなく、セッション全体の支出を報告します。複数の呼び出しにわたってコストを累積する方法については、[複数の呼び出しにわたってコストを累積する](#accumulate-costs-across-multiple-calls)を参照してください。

次の図は、単一の `query()` 呼び出しからのメッセージストリームを示しており、各ステップでトークン使用量が報告され、最後に累積推定値が表示されます。

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" className="dark:hidden" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/message-usage-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=8ea95085abc0a6b7f55ecef498bd4d14" className="hidden dark:block" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow-dark.svg" />

<Steps>
  <Step title="各ステップはアシスタントメッセージを生成します">
    Claude が応答すると、1 つ以上のアシスタントメッセージを送信します。TypeScript では、各アシスタントメッセージには、ネストされた `BetaMessage`（`message.message` 経由でアクセス）が含まれており、`id` とトークン数（`input_tokens`、`output_tokens`）を含む [`usage`](https://platform.claude.com/docs/en/api/messages) オブジェクトがあります。Python では、`AssistantMessage` データクラスは `message.usage` と `message.message_id` 経由で同じデータを直接公開しています。Claude が 1 つのターンで複数のツールを使用する場合、そのターンのすべてのメッセージは同じ ID を共有するため、二重計算を避けるために ID でデデュプリケートしてください。
  </Step>

  <Step title="結果メッセージは累積推定値を提供します">
    `query()` 呼び出しが完了すると、SDK は `total_cost_usd` と累積 `usage` を含む結果メッセージを発行します。TypeScript では [`SDKResultMessage`](/docs/ja/agent-sdk/typescript#sdkresultmessage) として型付けされ、Python では [`ResultMessage`](/docs/ja/agent-sdk/python#resultmessage) として型付けされます。推定合計のみが必要な場合は、ステップごとの使用量を無視して、この単一の値を読むことができます。

    複数の独立した `query()` 呼び出しを行う場合、各結果はその個別の呼び出しのコストのみを反映します。セッションを再開した呼び出しは、セッションの以前の支出もカウントします。

    ストリーミング入力モードでは、各ターンが独自の結果メッセージを発行します。そのモードでコール合計を読む方法については、[ストリーミング入力モードでコストを追跡する](#track-costs-in-streaming-input-mode)を参照してください。
  </Step>
</Steps>

<h2 id="track-costs-in-streaming-input-mode">
  ストリーミング入力モードでコストを追跡する
</h2>

[ストリーミング入力モード](/docs/ja/agent-sdk/streaming-vs-single-mode)では、1 つの `query()` 呼び出しが複数のユーザーターンを実行し、各ターンが独自の結果メッセージを出力します。結果フィールドのスコープは異なります。

* **`usage`**: そのターンのみをカバーし、その中でもメインエージェントループのみで、実行したサブエージェントはカバーしません。
* **`total_cost_usd` と `modelUsage`、または Python の `model_usage`**: これまでの呼び出し全体の実行合計を保持し、呼び出しがセッションを再開したときに復元されたすべての支出を含みます。

アプリが `/clear`、`/reset`、または `/new` を送信しない呼び出しでは、結果全体を合計するのではなく、最新の結果を読んで呼び出し合計を取得してください。

実行合計は、アプリがこれら 3 つのコマンドのいずれかを送信するたびにリセットされ、`query()` 呼び出し内では他に何もリセットしません。会計に重要な 3 つの結果があります。

* **`/clear` ターンの独自の結果**: リセット以降に実行されたもののみをカバーし、新しい `session_id` を保持します。
* **その後のすべての結果**: そのリセットからカウントを続けます。
* **各 `/clear` の前の最後の結果**: 前のリセット以降のターンの合計を保持します。

呼び出し全体を合計するには、各 `/clear` の前の最後の結果を呼び出しの最終結果に追加します。`/clear` ターン自体を含むその他のすべての結果は、後の結果に置き換えられます。

TypeScript では、SDK は各リセットで [`SDKConversationResetMessage`](/docs/ja/agent-sdk/typescript#sdkconversationresetmessage) も出力するため、ストリームからリセットを検出できます。Python では、SDK は同様に `ConversationResetMessage` を出力します。Python SDK v0.2.137 より前では、Python イテレータはそのメッセージをドロップしたため、これらのバージョンではアプリが送信する `/clear` ターンからリセットを自分でカウントしてください。

`maxBudgetUsd`（TypeScript）または `max_budget_usd`（Python）は呼び出し自体の支出のみをカウントします。再開されたセッションから復元された合計はカウントされず、`/clear` はバジェットをリセットします。

<h2 id="get-the-total-cost-of-a-query">
  クエリの総コストを取得する
</h2>

結果メッセージは、TypeScript では [`SDKResultMessage`](/docs/ja/agent-sdk/typescript#sdkresultmessage) として、Python では [`ResultMessage`](/docs/ja/agent-sdk/python#resultmessage) として型付けされており、`query()` 呼び出しのエージェントループの終了を示します。これには `total_cost_usd` が含まれており、その呼び出し内のすべてのステップにわたる累積推定コストです。セッションを再開する呼び出しは、セッションの以前の支出もカウントします。値を読み取る場合、2 つの注意事項が適用されます。

* Python ではフィールドはオプションとして型付けされているため、読み取る前に `None` でないことを確認してください。
* 成功結果とエラー結果の両方がこれを含みますが、[セッションクラッシュ後の総コストの復旧](#recover-totals-after-a-session-crash)の最終結果はゼロになる可能性があります。

ストリーミング入力モードでは、[ストリーミング入力モードでのコスト追跡](#track-costs-in-streaming-input-mode)で説明されているように呼び出し総額を読み取ります。

3 つの結果レベルのフィールドは、エージェントが [サブエージェント](/docs/ja/agent-sdk/subagents) を生成する場合に何をカウントするかが異なります。ツリー全体のトークンアカウンティングには `modelUsage` または Python では `model_usage` を使用してください。`usage` フィールドはネストが発生するとすぐに過小カウントされます。

| フィールド                        | サブエージェントアクティビティ                                            |
| ---------------------------- | ---------------------------------------------------------- |
| `usage`                      | 除外。トップレベルのエージェントループのみをカウントするため、サブエージェント内で消費されたトークンは追加されません |
| `total_cost_usd`             | 含まれます。トップレベルループと並行してサブエージェントリクエストをカウントします                  |
| `modelUsage` / `model_usage` | 含まれます。トップレベルループと並行してサブエージェントリクエストをカウントし、モデル別に分類されます        |

[シングルメッセージ入力モード](/docs/ja/agent-sdk/streaming-vs-single-mode#single-message-input)では、最終ターンの終了時にバックグラウンドサブエージェントがまだ実行中の場合、Claude Code は [終了時のバックグラウンドタスク](/docs/ja/headless#background-tasks-at-exit)で説明されているキャップまで、結果を発行する前にそれらを待機します。結果の `total_cost_usd`、`duration_api_ms`、および `modelUsage` または Python では `model_usage` には、その待機中に実行された作業が含まれます。

次の例は、`query()` 呼び出しからのメッセージストリームを反復処理し、`result` メッセージが到着したときに総コストを出力します。

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

サブエージェントが `total_cost_usd` に追加できる量を制限するには、クエリで [深さ、同時実行性、および支出制限](/docs/ja/agent-sdk/subagents#cap-subagent-depth-concurrency-and-spend) を設定してください。

<h2 id="track-per-step-and-per-model-usage">
  ステップごと、モデルごとの使用状況を追跡する
</h2>

このセクションの例では TypeScript フィールド名を使用しています。Python では、ステップごとの使用状況に対応するフィールドは [`AssistantMessage.usage`](/docs/ja/agent-sdk/python#assistantmessage) と `AssistantMessage.message_id` であり、モデルごとの内訳に対応するフィールドは [`ResultMessage.model_usage`](/docs/ja/agent-sdk/python#resultmessage) です。

<h3 id="track-per-step-usage">
  ステップごとの使用状況を追跡する
</h3>

各アシスタントメッセージには、ネストされた `BetaMessage`（`message.message` でアクセス）が含まれており、`id` とトークン数を含む `usage` オブジェクトがあります。Claude がツールを並列で使用する場合、複数のメッセージが同じ `id` を共有し、同一の使用状況データを持ちます。既にカウントした ID を追跡し、重複をスキップして合計が膨らまないようにしてください。

<Warning>
  重複排除されたステップごとの値は、入力トークンとキャッシュトークンについては正確です。ステップごとの `output_tokens` はプレースホルダーであるため、[結果メッセージから出力トークンを読み取ってください](#read-output-tokens-from-the-result-message)。
</Warning>

次の例は、すべてのステップ全体で入力トークンを累積し、各ユニークなメインループメッセージ ID を 1 回だけカウントしてサブエージェントメッセージをスキップし、メインループをカバーする結果メッセージから出力合計を読み取ります。

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
  モデルごとの使用状況を内訳する
</h3>

結果メッセージには [`modelUsage`](/docs/ja/agent-sdk/typescript#modelusage) が含まれており、これはモデル名からモデルごとのトークン数とコストへのマップです。これは複数のモデルを実行する場合（たとえば、サブエージェント用に Haiku、メインエージェント用に Opus）に便利で、トークンがどこに使用されているかを確認したい場合に役立ちます。

各エントリの `costBasis` は、そのモデルの最新リクエストに価格を付けた価格表を示します。`list` はリスト価格、`managed` は [`modelPricing`](/docs/ja/settings-reference#modelpricing) テーブル、または `unknown` はどちらもモデル ID に一致しなかった場合です。このフィールドには Claude Code v2.1.246 以降が必要です。

次の例はクエリを実行し、使用されたモデルごとのコストとトークンの内訳を出力します。

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
  複数の呼び出しにわたってコストを累積する
</h2>

各 `query()` 呼び出しは結果に対して `total_cost_usd` を返します。値の組み合わせ方は、呼び出しがセッションを共有するかどうかによって異なります。

* **`resume` または `continue` オプションのない独立した呼び出し**：各結果は独自の呼び出しのみをカバーするため、以下の例のように合計を自分で追加する必要があります。
* **同じセッションを再開する呼び出し**：Claude Code はプロセスが正常に終了するときにセッションの合計を[トランスクリプト](/docs/ja/sessions#where-transcripts-are-stored)に保存し、後の呼び出しがセッションを再開またはフォークするときに復元します。各結果には既にセッションの以前の支出が含まれています。セッション合計については最新の結果を読み取ります。結果を合計すると、復元された支出が二重計算されます。v2.1.277 より前では、SDK または `claude -p` を通じて再開したセッションは合計をゼロで開始したため、各呼び出しの結果はその呼び出しのみをカバーしていました。

ストリーミング入力モードでは、[ストリーミング入力モードでコストを追跡する](#track-costs-in-streaming-input-mode)で説明されているように各呼び出しの合計を読み取ります。セッションクラッシュで終了した呼び出しについては、[セッションクラッシュ後に合計を復旧する](#recover-totals-after-a-session-crash)を参照してください。

以下の例は、2 つの `query()` 呼び出しを順次実行し、各呼び出しの `total_cost_usd` を実行中の合計に追加し、呼び出しごとの合計と組み合わせた合計の両方を出力します。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track cumulative cost across multiple query() calls
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
      # Track cumulative cost across multiple query() calls
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
  エラー、キャッシング、出力トークン数を処理する
</h2>

正確なコスト追跡のために、アシスタントメッセージのプレースホルダー出力カウント、失敗した会話が消費したトークン、およびキャッシュトークン価格を考慮してください。

<h3 id="read-output-tokens-from-the-result-message">
  結果メッセージから出力トークンを読み取る
</h3>

Claude Code は、レスポンスが開始されたときに API が報告した使用状況からアシスタントメッセージを構築するため、メッセージの `output_tokens` は、レスポンスが生成される前に API が `message_start` で報告したカウントのみです。1 つの API レスポンスは複数のアシスタントメッセージを生成でき、それぞれが同じプレースホルダーを保持しています。

API はレスポンスの終了時に実際の出力カウントを報告し、Claude Code はそれを結果メッセージに追加します。結果の `usage` から、または詳細なモデル別の内訳については `modelUsage` から出力トークンを読み取ります。

レスポンスの出力カウントがストリーミング中に増加するのを監視するには、`includePartialMessages` を設定するか、Python では `include_partial_messages` を設定し、各 `message_delta` ストリームイベントから `usage` を読み取ります。TypeScript では [`SDKPartialAssistantMessage`](/docs/ja/agent-sdk/typescript#sdkpartialassistantmessage) として、Python では [`StreamEvent`](/docs/ja/agent-sdk/python#streamevent) として型付けされています。

<h3 id="track-costs-on-failed-conversations">
  失敗した会話のコストを追跡する
</h3>

成功とエラーの両方の結果メッセージには `usage` と `total_cost_usd` が含まれます。Python では両方のフィールドはオプションとして型付けされているため、読み取る前に `None` でないことを確認してください。

会話が途中で失敗した場合でも、失敗の時点までトークンを消費しています。すべての結果メッセージからコストデータを読み取ります。その `subtype` が `success` であるか、エラーサブタイプの 1 つであるかに関わらず。一部のエラー結果では、`usage` は呼び出しが費やした額より少なく報告します。

* **[セッションクラッシュ後の `error_during_execution`](#recover-totals-after-a-session-crash)**: すべてのコストフィールドがゼロになる可能性があります。
* **`error_max_budget_usd`**: `usage` は予算を超えたレスポンスを除外しますが、`total_cost_usd` と `modelUsage` はそれを含みます。

選択肢がある場合は、`usage` ではなく `total_cost_usd` または `modelUsage` から計上してください。

<h3 id="recover-totals-after-a-session-crash">
  セッションクラッシュ後に合計を復旧する
</h3>

Claude Code プロセスがクラッシュすると、最終的な `error_during_execution` 結果を発行して終了します。シングルショットモードとストリーミング入力モードの両方で同様です。その結果は `usage`、`total_cost_usd`、および `modelUsage` がゼロになる可能性があるため、それより前に到着したものから呼び出しの合計を復旧してください。ステップ 1 は、以前の結果が存在する場合はいつでも完全な合計を復旧します。ステップ 2 のフォールバックは、メインループの入力とキャッシュトークンのみを復旧します。

1. クラッシュの前のターンの結果を使用します。ストリーミング入力モードでは、[ストリーミング入力モードでコストを追跡する](#track-costs-in-streaming-input-mode)で説明されている実行中の合計を保持しています。その結果が役に立たない場合はステップ 2 に進んでください。
   * 呼び出しはシングルショットであるため、以前の結果は存在しません。
   * クラッシュは最初のターンで発生しました。
   * クラッシュの前のターンは `/clear` 自体であったため、その結果はリセットのみをカバーしています。
2. 代わりに、アシスタントメッセージの `usage` を合計し、各 API レスポンスを 1 回カウントします。[ステップごとの使用状況を追跡する](#track-per-step-usage)の例が行うように。シングルショットモードでは、すべてを合計します。ストリーミング入力モードでは、最後の結果の後に到着したものを合計します。これにより、メインループの入力とキャッシュトークンが得られます。サブエージェントの使用状況はこの方法では復旧できず、出力トークンや USD コストも復旧できません。[ステップごとの `output_tokens` はプレースホルダー](#read-output-tokens-from-the-result-message)であるためです。

<h3 id="track-cache-tokens">
  キャッシュトークンを追跡する
</h3>

Agent SDK は、繰り返されるコンテンツのコストを削減するために、[プロンプトキャッシング](https://platform.claude.com/docs/en/build-with-claude/prompt-caching)を自動的に使用します。キャッシングを自分で設定する必要はありません。使用状況オブジェクトには、キャッシュ追跡用の 2 つの追加フィールドが含まれています。

* `cache_creation_input_tokens`: 新しいキャッシュエントリを作成するために使用されたトークン（標準入力トークンより高いレートで課金されます）。
* `cache_read_input_tokens`: 既存のキャッシュエントリから読み取られたトークン（削減されたレートで課金されます）。

キャッシング節約を理解するために、これらを `input_tokens` とは別に追跡してください。TypeScript では、これらのフィールドは [`Usage`](/docs/ja/agent-sdk/typescript#usage) オブジェクトで型付けされています。Python では、[`ResultMessage.usage`](/docs/ja/agent-sdk/python#resultmessage) 辞書のキーとして表示されます（例えば、`message.usage.get("cache_read_input_tokens", 0)`）。

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  プロンプトキャッシュ TTL を 1 時間に延長する
</h3>

独自のターンは、[メイン会話 TTL バケット](/docs/ja/prompt-caching#which-ttl-each-request-gets)に分類されます。Claude Code がそれらと一緒にインラインで実行するヘルパーと共に。Claude Code が会話外で行う要求（[サブエージェント](/docs/ja/agent-sdk/subagents)など）には、[別の TTL 制御](/docs/ja/prompt-caching#choose-the-ttl-yourself)があります。

独自のターンのキャッシュエントリは、API キーで認証するか、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、または [AWS 上の Claude Platform](/docs/ja/claude-platform-on-aws) で実行する場合、デフォルトで 5 分の TTL を使用します。ワークロードが同じシステムプロンプトとコンテキストに対して多くの短いセッションを実行し、セッション間に 5 分以上のギャップがある場合、キャッシュはセッション間で期限切れになり、各新しいセッションは完全な入力価格を支払います。

キャッシュ書き込みで 1 時間の TTL をリクエストするには、[`ENABLE_PROMPT_CACHING_1H`](/docs/ja/env-vars) 環境変数を設定します。シェルまたはコンテナ環境でエクスポートするか、`options.env` を通じて渡すことができます。

次の例は、Amazon Bedrock で実行されているエージェントの 1 時間 TTL を有効にします。`CLAUDE_CODE_USE_BEDROCK` を設定するため、[Amazon Bedrock](/docs/ja/amazon-bedrock) の動作する AWS 認証情報が必要です。それらがない場合、クエリは失敗します。

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

1 時間の TTL でのキャッシュ書き込みは、5 分の書き込みより高いレートで課金されるため、これを有効にすると、より高い書き込みコストとより多くのキャッシュ読み取りがトレードオフされます。詳細については、[プロンプトキャッシング価格](https://platform.claude.com/docs/en/build-with-claude/prompt-caching)を参照してください。プランの含まれる使用量内の Claude サブスクリプションでは、この変数を設定せずに独自のターンで 1 時間の TTL を取得し、Claude Code がそれらのターンを 5 分の TTL に削除するのは、[使用クレジット](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)を引き出しているときです。

`ENABLE_PROMPT_CACHING_1H` は、両方のバケット内のすべてのリクエストで 1 時間の TTL をリクエストします。各バケットの TTL を個別に選択するには、代わりにこれらの制御を使用してください。それぞれは `5m` または `1h` を取り、`ENABLE_PROMPT_CACHING_1H` より優先されます。

* メイン会話: `CLAUDE_CODE_PROMPT_CACHE_TTL` [環境変数](/docs/ja/env-vars)、または [`promptCacheTtl`](/docs/ja/settings-reference#promptcachettl) 設定
* その他すべて: `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` 環境変数、または [`subagentPromptCacheTtl`](/docs/ja/settings-reference#subagentpromptcachettl) 設定

`promptCacheTtl` を `1h` に設定すると、使用クレジットを引き出しているときにメイン会話で 1 時間のキャッシュを保持します。完全な優先順位については、[TTL を自分で選択する](/docs/ja/prompt-caching#choose-the-ttl-yourself)を参照してください。

<h2 id="related-documentation">
  関連ドキュメント
</h2>

* [TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript) - 完全な API ドキュメント
* [SDK 概要](/docs/ja/agent-sdk/overview) - SDK の使用を開始する
* [SDK パーミッション](/docs/ja/agent-sdk/permissions) - ツールパーミッションの管理
