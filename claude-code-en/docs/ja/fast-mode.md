> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 高速モードでレスポンスを高速化

> Claude Code で高速モードを切り替えて、Opus のレスポンスを高速化します。

<Note>
  高速モードは[リサーチプレビュー](#research-preview)段階です。機能、価格設定、および利用可能性はフィードバックに基づいて変更される可能性があります。
</Note>

高速モードは Claude Opus の高速構成で、モデルを最大 2.5 倍高速化しますが、トークンあたりのコストは高くなります。迅速な反復やライブデバッグなどのインタラクティブな作業で速度が必要な場合は `/fast` でオンにし、コストがレイテンシーより重要な場合はオフにします。

高速モードは異なるモデルではありません。Claude Opus を使用していますが、コスト効率よりも速度を優先する異なる API 構成です。同じ品質と機能が得られ、レスポンスが高速化されるだけです。高速モードは Opus 5.5、Opus 5、および Opus 4.8 でサポートされています。Sonnet、Haiku、または他のモデルでは利用できません。

Opus 4.7 は高速モードをサポートしていないため、それに切り替えると高速モードがオフになります。Opus 4.7 の高速モードは 2026 年 6 月 25 日に非推奨となり、2026 年 7 月 24 日に削除されました。

知っておくべきこと：

* Claude Code CLI で `/fast` を使用して高速モードをオンにします。[VS Code 拡張機能](/docs/ja/vs-code)は、選択したモデルが高速モードをサポートしている場合、**高速モードを切り替え**コマンドを提供します。Claude Code はそのトグルを [`fastMode` 設定](#toggle-fast-mode)に保存します。
* 高速モード価格は Opus 5.5 で入力/出力あたり $8/$40 MTok、Opus 5 および Opus 4.8 で入力/出力あたり $10/$50 MTok です。
* サブスクリプションプラン（Pro/Max/Team/Enterprise）の Claude Code ユーザーと Claude Console のすべてのユーザーが利用可能です。Team および Enterprise 組織は Owner が最初に有効にする必要があり、Console 組織はアクセスを最初にプロビジョニングする必要があります。どちらも[要件](#requirements)に記載されています。
* サブスクリプションプラン（Pro/Max/Team/Enterprise）の Claude Code ユーザーの場合、高速モードは使用量クレジットのみで利用可能であり、サブスクリプションレート制限に含まれていません。

<h2 id="toggle-fast-mode">
  高速モードの切り替え
</h2>

CLI では、次のいずれかの方法で高速モードを切り替えます：

* `/fast` を実行して、Space キーを押してオンまたはオフに切り替えてから、Enter キーを押して確認します
* [ユーザー設定ファイル](/docs/ja/settings)で `"fastMode": true` を設定する

デフォルトでは、インタラクティブセッションで有効にした高速モードはセッション全体で保持されます。高速モードを各セッションでリセットするように構成できます。詳細は[セッションごとのオプトインが必要](#require-per-session-opt-in)を参照してください。

[クラウドセッション](#use-fast-mode-in-cloud-sessions)の外では、[非インタラクティブモード](/docs/ja/headless)で `-p` フラグを使用する場合、`/fast` は [`--settings`](/docs/ja/cli-reference#cli-flags) 値で高速モードを使用して起動されたセッションでのみ機能します。例えば `claude -p --settings '{"fastMode": true}'` のように使用します。その場合、切り替えはそのセッションにのみ適用され、デフォルトとして保存されません。`-p` 形式には Claude Code v2.1.205 以降が必要です。非インタラクティブモードの他の場所では、コマンドが高速モードが利用できないことを報告します。

Claude が処理中に `/fast` を実行でき、Claude Code は処理の終了を待たずに高速モードを切り替えます。Claude Code は実行中のターンを元の速度で完了するため、速度の変更は次のターンから有効になります。現在のモデルが高速モードをサポートしていない場合、有効にするとモデルも切り替わり、Claude Code はそのターン内の次のリクエストから新しいモデルを使用します。

最高のコスト効率を得るには、会話の途中で切り替えるのではなく、セッションの開始時に高速モードを有効にします。詳細は[コストのトレードオフを理解する](#understand-the-cost-tradeoff)を参照してください。

高速モードを有効にすると：

* 現在のモデルが高速モードをサポートしていない場合、Claude Code は Opus に切り替わります
* 確認メッセージが表示されます：「Fast mode ON」
* 高速モードがアクティブな間、プロンプトの横に小さい `↯` アイコンが表示されます
* いつでも `/fast` を再度実行して、高速モードがオンかオフかを確認できます

Opus 5.5 は Claude Code v2.1.280 以降の高速モードのデフォルトです。v2.1.280 より前では、高速モードは v2.1.219 から v2.1.280 では Opus 5 がデフォルトで、v2.1.154 から v2.1.218 では Opus 4.8 がデフォルトで、v2.1.142 から v2.1.153 では Opus 4.7 がデフォルトでした。

`/fast` を再度実行して高速モードを無効にすると、Opus に留まります。別のモデルに切り替えるには、`/model` を使用します。

<h3 id="switch-models-while-fast-mode-is-on">
  高速モードがオンの状態でモデルを切り替える
</h3>

高速モードは両方向のモデル切り替えに従います：

* **切り替える**：高速モードをサポートしていないモデルに切り替えると、Claude Code は高速モードをオフにします。これには Opus 4.7 が含まれます。v2.1.221 より前では、高速モードは Opus 4.7 への切り替え後もオンのままで、API がリクエストを拒否していました。
* **戻す**：サポートされている Opus モデルに戻すと、保存された高速モード設定がオンの場合、高速モードが再度オンになります。これは新しいセッションがデフォルトで開始される同じ設定です。モデル切り替えは、保存された設定がオフのセッションでは高速モードをオンにすることはなく、[セッションごとのオプトイン](#require-per-session-opt-in)が構成されている場合、戻しても高速モードはオンになりません。`/fast` を実行して再度有効にしてください。

モデル切り替えが高速モードをオンまたはオフにするたびに、Claude Code は `Fast mode ON` または `Fast mode OFF` の確認を表示し、高速モードがオンの間は `↯` アイコンが表示されます。これは `/model` で切り替える場合、[`/config model=<model>`](/docs/ja/settings)で切り替える場合、または [Remote Control](/docs/ja/remote-control)を通じて接続されたデバイスから切り替える場合に当てはまります。

Claude Code は、モデル切り替え、再接続、または失敗した[可用性チェック](#use-fast-mode-behind-proxies-and-llm-gateways)の後、Remote Control を通じて接続されたデバイスにセッションの高速モード状態を再送信します。

<h3 id="use-fast-mode-in-cloud-sessions">
  クラウドセッションで高速モードを使用する
</h3>

高速モードは[クラウドセッション](/docs/ja/claude-code-on-the-web)で機能します。アカウントで利用可能な場合、セッションが Anthropic 管理インフラストラクチャで実行されるか、[セルフホストランナー](/docs/ja/self-hosted-environments)で実行されるかに関わらず機能します。セッションの環境で Claude Code v2.1.271 以降が必要です。

セッションで `/fast on` と入力して高速モードをオンにします。そのセッションのみオンのままで、デフォルトとして保存されません。[要件](#requirements)はクラウドセッションにも適用されます。

<h2 id="understand-the-cost-tradeoff">
  コストのトレードオフを理解する
</h2>

高速モードは標準 Opus よりもトークンあたりの価格が高くなります。

| モデル      | 入力（MTok） | 出力（MTok） |
| -------- | -------- | -------- |
| Opus 5.5 | \$8      | \$40     |
| Opus 5   | \$10     | \$50     |
| Opus 4.8 | \$10     | \$50     |

高速モード価格は完全な 1M トークンコンテキストウィンドウ全体で一定です。比較対象となる標準 Opus レートについては、[Claude 価格リファレンス](https://platform.claude.com/docs/ja/about-claude/pricing)を参照してください。

会話で初めて高速モードを有効にすると、会話コンテキスト全体に対して完全な高速モードキャッシュなし入力トークン価格を支払います。会話が進むほど、このコストは高くなるため、最初から高速モードを有効にする方が安くなります。コストは会話ごとに 1 回適用されるため、後で高速モードをオフにしてからオンに切り替えても、再度請求されることはありません。メカニズムについては、[高速モードがプロンプトキャッシュとどのように相互作用するか](/docs/ja/prompt-caching#turning-on-fast-mode)を参照してください。

<h3 id="see-where-fast-mode-spend-appears">
  高速モード支出がどこに表示されるかを確認する
</h3>

高速モード支出がどこに表示されるかは、サインイン方法によって異なるため、まず [`/status`](/docs/ja/commands)を実行して確認してください。`Claude Max アカウント`などの `ログイン方法`行が表示される場合は、Claude サブスクリプションでサインインしています。代わりに `API キー`行が表示される場合は、リクエストが Claude Console 組織に請求されます。

* **Pro と Max**：使用クレジットから高速モードの料金を支払います。claude.ai の [**設定 > 使用状況**](https://claude.ai/settings/usage)に移動します。ここで、**使用クレジット**セクションに今月の使用クレジットの支出額が表示されます。この数字には高速モードが含まれていますが、別途に分類されていません。
* **Team と Enterprise**：組織が使用クレジットから高速モード使用の料金を支払います。自分の使用クレジット支出を確認するには、[`/usage`](/docs/ja/costs#check-your-usage-credits-spend)を実行してください。組織がその支出をどこで確認するかについては、[Claude for Teams and Enterprise](/docs/ja/costs#claude-for-teams-and-enterprise)を参照してください。
* **Claude Console**：組織が API 使用の残りの部分と一緒に高速モードの料金を支払います。Console の [使用状況](https://platform.claude.com/usage)ページと [コスト](https://platform.claude.com/cost)ページで、**グループ化**メニューの \*\*速度（Research Preview）\*\*を選択して、高速モードを標準速度の使用から分離します。選択した日付範囲に高速モード使用が含まれている場合にのみ、このオプションが表示されます。

<h2 id="decide-when-to-use-fast-mode">
  高速モードの使用時期を判断する
</h2>

高速モードはレスポンスレイテンシーがコストより重要なインタラクティブな作業に最適です：

* コード変更の迅速な反復
* ライブデバッグセッション
* 厳しい期限を持つ時間に敏感な作業

標準モードは以下に適しています：

* 速度がそれほど重要でない長い自動タスク
* バッチ処理または CI/CD パイプライン
* コストに敏感なワークロード

<h3 id="fast-mode-vs-effort-level">
  高速モードと努力レベル
</h3>

高速モードと努力レベルはどちらもレスポンス速度に影響しますが、方法が異なります：

| 設定          | 効果                                  |
| ----------- | ----------------------------------- |
| **高速モード**   | 同じモデル品質、低レイテンシー、高コスト                |
| **低い努力レベル** | 思考時間が短い、レスポンスが高速、複雑なタスクでは品質が低下する可能性 |

両方を組み合わせることができます：単純なタスクで最大速度を得るために、低い[努力レベル](/docs/ja/model-config#adjust-effort-level)で高速モードを使用します。

<h2 id="requirements">
  要件
</h2>

高速モードには以下のすべてが必要です：

* **Anthropic API またはサブスクリプションのみ**：高速モードは Anthropic Console API および使用量クレジットを使用する Claude サブスクリプションプランで利用可能です。Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、または AWS 上の Claude Platform では利用できません。Console 組織は、[高速モードアクセスをプロビジョニング](#enable-fast-mode-for-your-organization)する必要もあります。
* **サブスクリプションプランで使用量クレジットが有効**：Pro、Max、Team、または Enterprise プランでは、アカウントに[使用量クレジット](/docs/ja/costs#add-usage-credits-to-your-subscription)が有効になっている必要があります。これにより、プランに含まれる使用量を超えて請求できます。有効になるまで、`/fast` は「Fast mode requires usage credits」と表示されます。有効にする方法はプランによって異なります：
  * Pro および Max では、[**Settings > Usage**](https://claude.ai/settings/usage) の **Usage credits** セクションで claude.ai で有効にするか、`/usage-credits` を実行してそのページを開きます。
  * Team および Enterprise では、請求アクセス権を持つメンバーが [**Admin settings > Usage**](https://claude.ai/admin-settings/usage) で組織の使用量クレジットを有効にし、アクセス権を持たないメンバーが `/usage-credits` を実行して組織の管理者にリクエストを送信します。

<Note>
  高速モード使用量は、プランに残りの使用量がある場合でも、使用量クレジットに直接請求されます。
</Note>

* **有料 Console 組織**：Claude Console アカウントは使用量クレジットを使用せず、組織は API 使用量の残りと同じようにトークンごとに高速モードの料金を支払います。Console の無料 Evaluation プランでは、`/fast` は「Fast mode unavailable during evaluation. Please purchase credits.」と表示されます。これをクリアするには、[Console 請求設定](https://platform.claude.com/settings/billing)でクレジットを購入します。
* **Team および Enterprise の所有者による有効化**：高速モードは Team および Enterprise 組織ではデフォルトで無効になっています。ユーザーがアクセスできるようにするには、所有者が明示的に[高速モードを有効にする](#enable-fast-mode-for-your-organization)必要があります。

<Note>
  4 つの組織設定が `/fast` で高速モードをオンにすることをブロックできます：

  * **高速モードが有効になっていない**：組織で高速モードが有効になっていない場合、`/fast` で高速モードをオンにすると「Fast mode has been disabled by your organization.」と表示されます。
  * **高速モードが管理設定によってオフにされている**：組織が [`fastMode: false`](/docs/ja/settings-reference#fastmode) を設定する[管理設定](/docs/ja/managed-settings)をデプロイしている場合、`/fast` で高速モードをオンにすると同じ「Fast mode has been disabled by your organization」メッセージが表示されます。
  * **セッションごとのオプトインが必要**：[`fastModePerSessionOptIn: true`](#require-per-session-opt-in) を設定する管理設定は、インタラクティブターミナルセッション以外のすべての場所で `/fast on` を同じメッセージで拒否します。
  * **高速モードモデルが許可されていない**：組織の [`availableModels`](/docs/ja/model-config#restrict-model-selection) 許可リストが高速モード Opus モデルを除外している場合、オンにすることは「is not in your organization's allowed models」で拒否されます。高速モードをサポートする許可された Opus モデルで既に実行中のセッションでは、`/fast` はモデルを切り替える代わりに現在のモデルで高速モードを有効にします。
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  組織の高速モードを有効にする
</h3>

組織で高速モードを有効にする場所は、組織が使用する製品によって異なります：

* **Console**（API カスタマー）：管理者が [Claude Code preferences](https://platform.claude.com/claude-code/preferences) で有効にします。高速モードは[リサーチプレビュー](#research-preview)にあるため、組織は高速モードリクエストが成功する前に高速モードアクセスをプロビジョニングする必要があります。アクセスを取得するには、アカウントマネージャーに連絡するか、[Claude API の高速モード](https://platform.claude.com/docs/en/build-with-claude/fast-mode)で説明されているようにウェイトリストに参加します。

  プロビジョニングされたアクセスがない場合、API は各高速モードリクエストを 429 で拒否し、Claude Code は各拒否を[高速モードレート制限](#handle-rate-limits)として扱います。レート制限のクールダウンとは異なり、アクセスがプロビジョニングされるまで拒否は続きます。
* **Claude AI**（Team および Enterprise）：所有者が [Admin Settings > Claude Code](https://claude.ai/admin-settings/claude-code) で有効にします

高速モードを完全に無効にするもう 1 つのオプションは、`CLAUDE_CODE_DISABLE_FAST_MODE=1` を設定することです。[環境変数](/docs/ja/env-vars)を参照してください。

<h3 id="use-fast-mode-behind-proxies-and-llm-gateways">
  プロキシと LLM ゲートウェイの背後で高速モードを使用する
</h3>

高速モードを提供する前に、Claude Code は `api.anthropic.com` への直接リクエストで組織の高速モード可用性をチェックします。チェックは [`ANTHROPIC_BASE_URL`](/docs/ja/llm-gateway-connect#set-the-base-url-and-credential) に従わないため、Claude トラフィックを [LLM ゲートウェイ](/docs/ja/llm-gateway)を通じてルーティングし、`api.anthropic.com` への直接エグレスをブロックするネットワークでは、推論リクエストが機能していてもチェックが失敗します。チェックは設定された [HTTP プロキシ](/docs/ja/network-config#proxy-configuration)を使用するため、ネットワークブロックはプロキシを通じても `api.anthropic.com` に到達できない場合にのみチェックに失敗します。

チェックが失敗すると、`/fast` は「Fast mode unavailable due to network connectivity issues」と報告され、組織で高速モードが有効になっていても、リクエストは標準速度で実行されます。過去に成功したチェックはキャッシュされた結果から機能し続けるため、ブロックされたチェックは主に新しいインストールに影響します。

同じ接続メッセージは、チェックが `api.anthropic.com` に到達しても Anthropic が拒否する認証情報を提示するオープンネットワークに表示されます。解決されたキーがゲートウェイが発行した認証情報であるセッション（[`ANTHROPIC_API_KEY`](/docs/ja/llm-gateway-connect#set-the-base-url-and-credential) に保持されているか、[`apiKeyHelper`](/docs/ja/settings-reference#apikeyhelper) によって生成される）は、そのキーでチェックを送信し、拒否されたリクエストは接続失敗として報告されます。

高速モードを復元するには、ネットワークブロックが原因の場合は `api.anthropic.com` への直接エグレスをホワイトリストに登録するか、チェックが失敗する方法に一致する変数を設定します：

* `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1` は失敗したチェックを利用可能として扱い、「disabled by your organization」応答を引き続き尊重します。ネットワークが接続を拒否する場合、または Anthropic がゲートウェイ認証情報を拒否する場合に使用します。認証情報の場合はホワイトリストに登録しても役に立ちません。何もブロックされていないためです。
* `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` はチェック全体をスキップします。ネットワークがリクエストをインターセプトする場合に使用します。

2 つのゲートウェイ構成は、組織で高速モードが有効になっている場合でも、接続メッセージではなく「Fast mode has been disabled by your organization」と報告します：

* [`ANTHROPIC_AUTH_TOKEN`](/docs/ja/llm-gateway-connect#set-the-base-url-and-credential)のみで認証するセッションはチェックをスキップします：claude.ai ログインまたは Anthropic API キーがなく、キャッシュされた成功したチェックがない場合、Claude Code はリクエストを送信せずに高速モードが組織によって無効化されていると扱います。
* チェックをインターセプトして独自のページで応答するプロキシ（例えば、HTTP 200 ブロックページを返す TLS 検査プロキシ）は、組織で高速モードが無効化されていることを示す応答として読み取られます。

どちらの場合も、`CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` を設定して高速モードを復元します。`CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` は失敗したチェックのみをバイパスし、これら両方は無効化応答を生成するため、どちらのケースにも適用されません。ホワイトリストに登録する直接エグレスは、リクエストを送信しないベアラートークンケースには役に立ちません。

変数はクライアント側のチェックのみに影響します。組織で高速モードが無効化されている場合、設定されているかどうかに関わらず、API は高速モードリクエストを拒否します。API からの拒否は、スキップ変数が設定されていても有効です。Claude Code は拒否されたリクエストを標準速度で再試行し、高速モードをオフにして、`/fast` は組織が高速モードを無効化したことを報告します。

`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` を設定すると、可用性チェックも抑制されます。以前にキャッシュされた成功したチェックがない場合、`/fast` は「Fast mode is currently unavailable」と報告します。両方のスキップ変数はその構成でも高速モードを復元します。

<h3 id="require-per-session-opt-in">
  セッションごとのオプトインが必要
</h3>

デフォルトでは、ユーザーがインタラクティブセッションで有効にした高速モードはセッション全体で保持されます。これを変更するには、任意の[設定ファイル](/docs/ja/settings#where-settings-live)で `fastModePerSessionOptIn` を `true` に設定します。これにより、各セッションは高速モードがオフで開始され、ユーザーが `/fast` で明示的に有効にする必要があります。[Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) または [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) プランの所有者は、[サーバー管理設定](/docs/ja/server-managed-settings)を通じて組織全体にこれをデプロイできます。

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

これは、ユーザーが複数の同時セッションを実行する組織でコストを制御するのに役立ちます。ユーザーの高速モード設定は保存されたままなので、この設定を削除するとデフォルトの永続的な動作が復元されます。

管理設定がキーを設定する場合、`/fast on` はインタラクティブターミナルセッションでのみ機能します。[非インタラクティブモード](/docs/ja/headless)、[VS Code 拡張機能](/docs/ja/vs-code)、および[クラウドセッション](#use-fast-mode-in-cloud-sessions)を含むその他のすべての場所では、組織が高速モードを無効化したというメッセージで拒否されます。

<h2 id="handle-rate-limits">
  レート制限の処理
</h2>

高速モードは標準 Opus とは別のレート制限があります。サポートされているすべての Opus モデルは 1 つの高速モードレート制限プールを共有します：どのモデルでの使用も同じ制限から引き出されます。高速モードレート制限に達した場合：

1. 高速モードは自動的に標準速度にフォールバックします
2. `↯` アイコンがグレーに変わってクールダウンを示します
3. 標準速度と価格で作業を続けます
4. クールダウンが終了すると、高速モードは自動的に再度有効になります

クールダウンを待つ代わりに高速モードを手動で無効にするには、`/fast` を再度実行します。

セッション中に使用量クレジットが不足した場合、Claude Code は拒否された各高速モードリクエストを標準速度と価格で再試行するため、作業を続けることができ、クールダウンはありません。拒否の表示方法はセッションタイプによって異なります：

* インタラクティブセッションでは、Claude Code は「Fast mode disabled · usage credits exhausted」通知を表示し、セッションの残りの期間、高速モードをオフにします。保存された高速モード設定は変わりません。`/fast` を実行して高速モードを再度オンにします。
* [非インタラクティブモード](/docs/ja/headless)で `--output-format stream-json` を使用する場合、および Agent SDK を通じて、Claude Code はメッセージストリーム上で同じテキストを `system` メッセージとして、サブタイプ `notification` で、使用量クレジットが不足している間、ターンごとに 1 回発行します。高速モードはオンのままです。Claude Code v2.1.221 以降が必要です。

<h2 id="research-preview">
  リサーチプレビュー
</h2>

高速モードはリサーチプレビュー機能です。これは以下を意味します：

* 機能はフィードバックに基づいて変更される可能性があります
* 利用可能性と価格設定は変更される可能性があります
* 基盤となる API 構成は進化する可能性があります

通常の Anthropic サポートチャネルを通じて問題またはフィードバックを報告してください。

<h2 id="see-also">
  関連項目
</h2>

* [モデル構成](/docs/ja/model-config)：モデルを切り替えて努力レベルを調整する
* [コストを効果的に管理する](/docs/ja/costs)：トークン使用量を追跡してコストを削減する
* [ステータスラインの構成](/docs/ja/statusline)：モデルとコンテキスト情報を表示する
