> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# advisor ツールで難しい判断をエスカレートする

> メインモデルをより強力な advisor モデルと組み合わせて、タスク中の重要な瞬間に Claude が相談できるようにします。

<Note>
  advisor ツールは実験的機能であり、Anthropic API が必要です。Amazon Bedrock、Claude Platform on AWS、Google Cloud の Agent Platform、Microsoft Foundry では利用できません。動作、価格設定、および利用可能性は変更される可能性があります。
</Note>

advisor ツールを使用すると、Claude はタスク中の重要な瞬間（アプローチをコミットする前、繰り返し発生するエラーで行き詰まった場合、またはタスク完了を宣言する前など）に、通常はより強力な 2 番目のモデルに相談できます。advisor は、すべてのツール呼び出しと結果を含む完全な会話を受け取り、Claude が続行する前に適用するガイダンスを返します。

advisor は Anthropic インフラストラクチャ上でサーバー側で実行され、[サーバーツール](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)として、サブスクリプションと API 課金の両方のアカウントで利用できます。どのモデルが advisor として機能するかを選択し、Claude がそれをいつ呼び出すかを決定します。

このページでは、advisor を有効にする方法、受け入れられるモデルペアリング、相談中に Claude が表示する内容、および advisor 使用量がどのように課金されるかについて説明します。

<h2 id="when-to-use-the-advisor">
  advisor を使用する場合
</h2>

advisor は、ほとんどのターンが定型的であるが、プラン品質が結果を決定する長い複数ステップのタスクに適しています。例としては、大規模なリファクタリング、エラーが繰り返し発生するデバッグセッション、および Claude が完了を宣言する前に独立して確認したいタスクが挙げられます。

計画する必要がほとんどない短いタスク、またはすべてのターンで最強のモデルが必要な作業では、価値が低くなります。その場合は、[メインモデルを切り替える](/docs/ja/model-config#setting-your-model)か、[advisor が opusplan およびサブエージェントとどのように比較されるか](#compare-with-related-features)を参照して、2 番目の意見を得る他の方法を確認してください。

<h2 id="enable-the-advisor">
  advisor を有効にする
</h2>

advisor モデルは 3 つの方法で設定できます。

* **`/advisor` コマンド**：セッション中に advisor を設定または変更し、デフォルトとして保存します
* **`advisorModel` 設定**：[設定ファイル](/docs/ja/settings)で永続的なデフォルトを構成します
* **`--advisor` フラグ**：起動時に単一セッションの advisor を設定します

これらのいずれかが advisor を有効にするのは、メインモデルが[それをサポートしている](#choose-an-advisor-model)セッションです。セッションが開始されると、Claude Code は `Advisor Tool (experimental) is on and may use more tokens · /advisor` 通知を表示します。advisor の使用を停止するには、[advisor をオフにする](#turn-the-advisor-off)を参照してください。

一部のプランでは、Fable を advisor として使用する場合、Fable の使用を使用クレジットに請求することへの 1 回限りの[同意](/docs/ja/model-config#fable-and-usage-credits)も必要です。その同意を与える前に何が起こるかについては、[Fable advisor と使用クレジット](#fable-advisor-and-usage-credits)を参照してください。

<h3 id="use-the-/advisor-command">
  `/advisor` コマンドを使用する
</h3>

引数なしで `/advisor` を実行して、利用可能な advisor モデルをリストするピッカーを開くか、モデルを直接渡します。

```
/advisor opus
```

コマンドは `Advisor set to` で確認し、その後に advisor モデル名が続きます。選択はユーザー設定の `advisorModel` に保存され、セッション全体で保持されます。ただし、[`advisorModel` エントリ](/docs/ja/settings-reference#advisormodel)が現在のセッションにのみ適用されるとリストしている場合は除きます。

このコマンドは、ターミナルピッカーがない場所でも機能します。[非対話型モード](/docs/ja/headless)で `-p` を使用する場合、Agent SDK 内、デスクトップアプリ内、および[リモートコントロール](/docs/ja/remote-control)経由です。これには Claude Code v2.1.260 以降が必要です。これらのサーフェスでは、

* 引数なしで `/advisor` を実行して、現在の advisor モデルとそれが受け入れるエイリアスを出力します。
* `/advisor opus` などのモデルを使用して `/advisor` を実行して、それを設定します。
* `/advisor off` を実行してそれをオフにします。

Claude Code は、組織の [`availableModels`](/docs/ja/model-config#restrict-model-selection)許可リストが除外した保存済み advisor を呼び出しません。advisor を使用するには、`/advisor` で許可されたモデルを選択してください。Claude Code は、現在のメインモデルがサポートしていない advisor を引き続き保存します。その advisor は、[`/model`](/docs/ja/model-config#setting-your-model)で[互換性のあるメインモデル](#choose-an-advisor-model)に切り替えた後にアクティブになります。API がすでに現在の会話で保存済み advisor を拒否した場合、モデルを切り替えた後でも、`/clear` または `/compact` まで、それはオフのままです。

一部のプランでは、Fable を advisor として使用する場合、Fable の使用を使用クレジットに請求することへの 1 回限りの[同意](/docs/ja/model-config#fable-and-usage-credits)も必要です。その同意を与える前に `/advisor fable` が何をするかについては、[Fable advisor と使用クレジット](#fable-advisor-and-usage-credits)を参照してください。

<h3 id="set-advisormodel-in-settings">
  設定で `advisorModel` を設定する
</h3>

セッションを開かずにデフォルトとして advisor を構成するには、設定ファイルで設定します。

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  `--advisor` フラグを使用する
</h3>

保存された設定を変更せずに単一セッションの advisor を設定するには、フラグを使用して起動します。

```bash theme={null}
claude --advisor opus
```

Claude Code はそのセッションの `advisorModel` 設定の代わりにフラグを使用します。`claude --help` に `--advisor` をリストしません。Claude Code は以下の場合、起動時にエラーで終了します。

* セッションのメインモデルが advisor をサポートしていない
* Haiku などのリクエストされたモデルが advisor として機能できない
* 組織の [`availableModels`](/docs/ja/model-config#restrict-model-selection)許可リストがリクエストされたモデルを除外している
* Fable をリクエストし、アカウントがまだ[使用クレジット同意](#fable-advisor-and-usage-credits)を必要としている

`--advisor` で[バックグラウンドセッション](/docs/ja/agent-view)を開始し、これらのいずれかが当てはまる場合、Claude Code は終了する代わりに advisor なしでセッションを開始します。

<h2 id="choose-an-advisor-model">
  アドバイザーモデルを選択する
</h2>

アドバイザーはメインモデル以上の能力を持つ必要があります。各メインモデルで受け入れられるアドバイザーは以下の通りです。

| メインモデル                | 受け入れられるアドバイザー              | 注記                                                                       |
| --------------------- | -------------------------- | ------------------------------------------------------------------------ |
| Haiku 4.5             | Fable、Opus、Sonnet          | Haiku はアドバイザーを呼び出すことはできますが、アドバイザーとして機能することはできません                         |
| Sonnet 4.6            | Fable、Opus、Sonnet          |                                                                          |
| Sonnet 5              | Fable、Opus 4.7 以降、Sonnet 5 | Sonnet 4.6 アドバイザーは拒否され、Opus 4.6 アドバイザーを使用したリクエストは API エラーで失敗します          |
| Opus 4.6              | Fable、Opus、Sonnet 5        | Sonnet 4.6 アドバイザーは拒否されます                                                 |
| Opus 4.7 または Opus 4.8 | Fable、および Opus 4.7 以降      | Opus 4.6 または Sonnet アドバイザーは拒否されます                                        |
| Opus 5.5 または Opus 5   | Fable、および Opus 5 以降        | Opus 4.6 または Sonnet アドバイザーは拒否され、API は Opus 4.7 または Opus 4.8 アドバイザーを拒否します |
| Fable 5               | Fable 5.1 または Fable 5      | Opus または Sonnet アドバイザーは拒否されます                                            |
| Fable 5.1             | Fable 5.1                  | Opus または Sonnet アドバイザーは拒否され、Fable 5 アドバイザーを使用したリクエストは API エラーで失敗します      |

Fable 5.1 には Claude Code v2.1.257 以降が必要です。両方の Fable モデルには [Fable アクセス](/docs/ja/model-config#work-with-fable) が必要です。

アドバイザーを `fable`、`opus`、または `sonnet` として設定します。これらのエイリアスは Claude Code の各モデルファミリーの組み込みデフォルトバージョンに解決され、新しい Claude Code リリースで進化します。`claude-opus-5-5` などの完全なモデル ID を渡すこともできます。

サブエージェントは設定されたアドバイザーを継承し、独自のモデルに対して同じペアリングチェックを適用します。

Claude Code はリクエストを送信する前にペアリングを検証し、API が再度検証します。

* テーブルで拒否されたアドバイザーの場合、Claude Code はそれをメインモデルのリクエストに添付しません。`/advisor` コマンド出力と通知がこれを表示します。独自のモデルがペアリングを満たすサブエージェントは、引き続きアドバイザーを使用できます。
* テーブルで API エラーで失敗するとリストされているアドバイザーの場合、Claude Code はそれを添付し、API がそれを拒否します。Claude Code はその後、アドバイザーなしでそのリクエストを再送信し、残りの会話はアドバイザーなしで実行されるため、エラーが表示されず、アドバイザー呼び出しが行われません。`/advisor` で受け入れられたアドバイザーを選択してください。変更は `/clear` または `/compact` の後と新しいセッションで有効になります。
* メインモデルまたはアドバイザーが Claude Code が認識しないモデルの場合、アドバイザーは添付されません。

<h3 id="fable-advisor-and-usage-credits">
  Fable アドバイザーと使用クレジット
</h3>

一部のプランでは、Fable の使用は使用クレジットに請求され、アドバイザーとしての Fable も同じ方法で請求されます。アカウントが [Fable の使用を使用クレジットに請求するための一度限りの同意](/docs/ja/model-config#fable-and-usage-credits) を必要とする場合、Claude Code は `/model` で Fable モデルを選択するときにそれを要求し、その同意を受け入れるまで Fable をアドバイザーとして適用しません。

それを受け入れる前に、Claude Code は `/advisor fable` を入力するか `/advisor` ピッカーで Fable を選択するときに Fable をアドバイザーとして保存しません。代わりに `/model fable` を指すようにします。`claude --advisor fable` を使用すると、Claude Code は `/model fable` を指すメッセージで起動時に終了します。[バックグラウンドセッション](#use-the-advisor-flag) では、終了する代わりにアドバイザーなしでセッションを開始します。Fable が既に `advisorModel` として保存されている場合、Claude Code はアドバイザーなしでリクエストを送信します。アドバイザーをサポートするインタラクティブセッションでは、`/model fable` を指す通知も表示されます。

同意を受け入れるには、`/model fable` を実行し、Fable で続行することを選択します。Claude Code は同意を記録し、[Fable を選択したモデルとして保存](/docs/ja/model-config#default-model-setting) します。次に、Fable をアドバイザーとして選択します。

<h3 id="common-model-pairings">
  一般的なモデルペアリング
</h3>

受け入れられたペアリングはすべて機能します。これらの組み合わせは、異なる方法でコストと能力のバランスを取ります。

| ペアリング                      | 使用する場合                                                                                  |
| -------------------------- | --------------------------------------------------------------------------------------- |
| Sonnet メイン + Opus アドバイザー   | Sonnet は日常的な作業を処理し、計画、曖昧な失敗、完了チェックを Opus にエスカレートします                                     |
| Sonnet メイン + Fable アドバイザー  | 決定ポイントで Fable ガイダンスを取得し、プロセス全体で Fable を実行しません。Fable アクセスが必要です                           |
| Haiku メイン + Opus アドバイザー    | 強力な計画を備えた最も低コストのメインモデル。Haiku 単独よりも高いコストが予想されますが、メインモデルを Sonnet または Opus に切り替えるよりは低くなります |
| Opus メイン + Opus アドバイザー     | 2 番目の Opus が最初の Opus をレビューします。コストよりも独立したチェックが重要な高リスクタスクに役立ちます                           |
| Fable メイン + Fable アドバイザー   | Fable が利用可能な場合の最高能力ペアリング。Claude Code は Fable メインモデルに Opus または Sonnet アドバイザーを適用しません      |
| Sonnet メイン + Sonnet アドバイザー | 日常的な見落としをキャッチするための低コストの 2 番目の意見                                                         |

<h2 id="when-claude-consults-the-advisor">
  Claude が advisor に相談する場合
</h2>

Claude は advisor をいつ呼び出すかを決定します。アプローチをコミットする前、エラーが繰り返し発生する場合、およびタスク完了を宣言する前に相談する傾向がありますが、タイミングはルールベースではなくモデル駆動です。

プロンプトで相談をリクエストするのと同じ方法で、プロンプトで相談をリクエストできます。例えば、`continue する前に advisor に相談してください`。advisor 呼び出しをキャップまたは強制する設定はありません。タスク中に Claude が advisor に相談する頻度を増やしたい場合は、指示で言及してください。

<h2 id="what-you-see-during-a-session">
  セッション中に表示される内容
</h2>

Claude が advisor を呼び出すと、トランスクリプトに呼び出しが進行中の advisor モデル名を含む `Advising` 行が表示されます。結果が返されると、行は advisor がガイダンスを提供したかどうかを報告します。

* **Reviewed**: この行は advisor が会話をレビューしたことを確認します。advisor が読み取り可能なガイダンスを返した場合、`Ctrl+O` を押して読むことができます。
* **Declined**: この行は `Advisor declined to advise on this request` と表示されます。advisor が理由を提供した場合、`Ctrl+O` を押して読むことができます。

Claude は一般的に advisor のガイダンスに従いますが、独自の証拠が特定の主張と矛盾する場合は適応します。推奨されたステップが試行時に失敗した場合、またはファイルの内容がアドバイスと矛盾する場合、Claude はガイダンスに無条件に従うのではなく、矛盾を表示します。

advisor は常に完全な会話を受け取り、Claude がタイミングを制御します。より多くの制御または異なる構成については、[advisor がサブエージェントおよび opusplan とどのように比較されるか](#compare-with-related-features)を参照してください。

<h2 id="cost">
  コスト
</h2>

Claude が advisor を呼び出すと、advisor モデルが会話を読むため、各呼び出しはメインモデルの使用に加えて advisor モデルのレートでトークンを消費します。これらの advisor トークンがどのように課金されるかは、お支払い方法によって異なります。

* **API 課金**: advisor トークンに対して advisor モデルの入力および出力レートで課金されます
* **サブスクリプションプラン**: advisor 使用量はプランの使用制限にカウントされます。ただし、Fable advisor は Fable 使用量が課金される計画では[使用クレジット](/docs/ja/model-config#fable-and-usage-credits)に課金されます

アカウントが使用クレジット同意を必要とする場合、Fable advisor は同意を与えるまで何も課金されません。Claude Code はそれまで[選択を適用しない](#fable-advisor-and-usage-credits)ためです。

Claude は各ターンではなく決定ポイントで advisor を呼び出すため、より高速なメインモデルをより強力な advisor と組み合わせることは、通常、より強力なモデルを全体で実行するよりもコストが低くなります。advisor 使用量は [`/usage`](/docs/ja/costs#track-your-costs)で表示されるセッション合計にカウントされます。

advisor トークンが API レスポンスでどのように報告されるかについては、Claude API ドキュメントの[使用量と課金](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing)を参照してください。

<h2 id="impact-on-prompt-caching">
  プロンプトキャッシュへの影響
</h2>

セッション中に advisor を有効または無効にしても、メインモデルの[プロンプトキャッシュ](/docs/ja/prompt-caching)は無効になりません。[モデルを切り替える](/docs/ja/prompt-caching#switching-models)とは異なり、`/advisor` を切り替えるとキャッシュされたプレフィックスはそのまま保持され、advisor が返したガイダンスは後のターンでトランスクリプトの一部としてキャッシュされます。

advisor モデル自体の会話の読み取りはキャッシュされません。各 advisor 呼び出しは完全なトランスクリプトを新たに処理し、呼び出し間での再利用はありません。

<h2 id="requirements">
  要件
</h2>

advisor ツールには、以下のすべてが必要です。

* **Anthropic API のみ**：advisor はサーバー実行ツールです。Amazon Bedrock、Claude Platform on AWS、Google Cloud の Agent Platform、または Microsoft Foundry では利用できません。[LLM ゲートウェイ](/docs/ja/llm-gateway)を通じて `ANTHROPIC_BASE_URL` で構成されている場合、利用可能性はゲートウェイがリクエストを Anthropic API に完全に転送するかどうかに依存します。ゲートウェイまたはそのアップストリームが advisor ツールを認識しない場合は、[自動リトライとエラー転送](/docs/ja/llm-gateway-protocol#automatic-retry-and-error-forwarding)を参照して、Claude Code がどのように応答するかを確認してください。
* **サポートされているメインモデル**：Fable、Opus 4.6 以降、Sonnet 4.6 以降、または Haiku 4.5。[advisor モデルを選択する](#choose-an-advisor-model)を参照して、各 advisor がどのモデルを受け入れるかを確認してください。
* **フィーチャーフラグ取得**：Claude Code は Anthropic から取得するフィーチャーフラグを通じて advisor をオンにします。`DISABLE_TELEMETRY` など、フラグ取得をオフにする変数が設定されているセッションでは、advisor はオフのままです。[フィーチャーフラグ取得が必要な機能](/docs/ja/env-vars#features-that-need-feature-flag-fetching)を参照してください。

<h2 id="turn-the-advisor-off">
  advisor をオフにする
</h2>

advisor の使用を停止するには、`/advisor off` を実行するか、`/advisor` ピッカーで **No advisor** を選択します。

```
/advisor off
```

advisor ツール全体を無効にするには、`CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1` を設定します。`/advisor` コマンドは利用できなくなり、設定された `advisorModel` は無視されます。`--advisor` フラグは受け入れられますが、効果はありません。[環境変数](/docs/ja/env-vars)を参照してください。

<h2 id="compare-with-related-features">
  関連機能との比較
</h2>

advisor は、モデルの強みを組み合わせるいくつかの方法の 1 つです。2 番目のモデルをいつ関与させるかに基づいて選択します。

| アプローチ                                                 | より強力なモデルが実行される場合                                                                                          | 開始方法                          |
| ----------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------- |
| Advisor ツール                                           | タスク中の決定ポイント                                                                                               | Claude がガイダンスが必要な場合に呼び出します    |
| [`opusplan`](/docs/ja/model-config#opusplan-model-setting) | プランモード中（[`availableModels`](/docs/ja/model-config#restrict-model-selection)で許可されている場合）、その後実行用に Sonnet に切り替わります | プランモードに入ります                   |
| [サブエージェント](/docs/ja/sub-agents#choose-a-model)（`model` 設定） | 委任されたサブタスク全体                                                                                              | Claude が委任するか、サブエージェントを呼び出します |
| [`/model`](/docs/ja/model-config#setting-your-model)       | 後続のすべてのリクエストから                                                                                            | モデルを切り替えます                    |

<h2 id="see-also">
  関連項目
</h2>

* [モデル構成](/docs/ja/model-config)：モデルを切り替え、努力レベルを設定し、`opusplan` を使用します
* [コストを効果的に管理する](/docs/ja/costs)：モデル全体のトークン使用量を追跡します
* [Claude API の Advisor ツール](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)：基盤となるサーバーツールを理解するか、Messages API から直接使用します
* [advisor 戦略](https://claude.com/blog/the-advisor-strategy)：高速メインモデルをより強力な advisor と組み合わせる理由
