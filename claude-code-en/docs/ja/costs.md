> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# コストを効果的に管理する

> トークン使用量を追跡し、チームの支出制限を設定し、コンテキスト管理、モデル選択、拡張思考設定、前処理フックを使用して Claude Code のコストを削減します。

Claude Code は API トークン消費によって課金されます。サブスクリプションプラン価格（Pro、Max、Team、Enterprise）については、[claude.com/pricing](https://claude.com/pricing) を参照してください。開発者あたりのコストは、モデル選択、コードベースサイズ、複数インスタンスの実行や自動化などの使用パターンに基づいて大きく異なります。

エンタープライズ展開全体では、平均コストは開発者 1 人あたり 1 日約 13 ドル、開発者 1 人あたり月額 150～250 ドルで、90% のユーザーの 1 日あたりのコストは 30 ドル以下です。自分のチームの支出を見積もるには、小規模なパイロットグループから始めて、以下の追跡ツールを使用してベースラインを確立してから、より広い展開を行ってください。

このページでは、[コストを追跡する方法](#track-your-costs)、[チームのコストを管理する方法](#manage-costs-for-your-organization)、および [トークン使用量を削減する方法](#reduce-token-usage) について説明します。

<h2 id="track-your-costs">
  コストを追跡する
</h2>

<h3 id="using-the-/usage-command">
  `/usage` コマンドを使用する
</h3>

<Note>
  `/usage` のセッションブロックは API トークン使用量を表示し、API ユーザーを対象としています。Claude Max および Pro サブスクライバーはサブスクリプションに使用量が含まれているため、セッションコスト数値は請求目的では関連がありません。サブスクライバーは同じ画面でプラン使用量バー、アクティビティ統計、および使用量の内訳を表示します。
</Note>

`/usage` の上部のセッションブロックは、現在のセッションの詳細なトークン使用統計を表示します。Claude Code はトークン数からドル数値をローカルで計算します。ただし、[`modelPricing`](/docs/ja/settings-reference#modelpricing) テーブルが有効な場合は除きます。管理者は組織の管理設定でテーブルを設定し、数値が契約レートを使用するようにします。テーブルが有効な場合、`Total cost` 行には `at your organization's configured rates` という注記が付きます。この数値は推定値であるため、権限のある請求については [Claude Console](https://platform.claude.com/usage) の使用量ページを参照してください。

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 20s
Total duration (wall): 6h 33m 10s
Total code changes:    0 lines added, 0 lines removed
Usage by model:
   claude-sonnet-4-6:  1.2k input, 5.3k output, 940.0k cache read, 50.0k cache write ($0.55)
```

これらの合計は `/clear` が新しいセッションを開始するとリセットされるため、次のセッションの合計コストは \$0 から始まります。v2.1.211 より前では、`/clear` 全体で累積し続け、Claude Code プロセスの存続期間中に蓄積されていました。

1.1× [データレジデンシーレート](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing) で請求される Claude API からの応答の場合、Claude Code はその応答のトークンのリスト価格に 1.1 を乗じてセッションコスト数値に反映します。Claude Code は [ステータス行のコストフィールド](/docs/ja/statusline#cost-and-duration-tracking) に同じ合計を報告し、[`--max-budget-usd`](/docs/ja/cli-reference#cli-flags) と比較します。v2.1.239 より前では、Claude Code はこれらの応答に 1.1× を適用しなかったため、セッションコスト数値は請求額より低くなっていました。

<h4 id="prompt-cache-statistics">
  プロンプトキャッシュ統計
</h4>

メイン会話の最初の API 応答の後、Claude Code はセッションブロックに `Prompt cache (main)` 行も追加します。これはセッションの [プロンプトキャッシュ](/docs/ja/prompt-caching) 使用量をまとめたものです。リクエスト数、キャッシュから提供された入力トークンのシェア、キャッシュミス、およびキャッシュが現在ウォーム状態かどうかです。Claude Code v2.1.251 以降が必要です。

```text theme={null}
Prompt cache (main):   14 requests · 91% of input tokens from cache · 2 misses (last 6m 10s ago, 310.2k tokens re-cached) · 1 expected rebuild (compaction or tool-result clearing) · warm (1h TTL, last activity 40s ago)
```

行のミス、予想される再構築、およびウォーム状態またはコールド状態の部分は以下を意味します。

* **ミス**: キャッシュが既に保持していたコンテンツを再処理したリクエスト。最後のミスの時刻と、それらのリクエストがキャッシュに書き戻したトークン数が表示されます。Claude Code は、リクエストがキャッシュから読み取ることができた内容の 5% 以上かつ最低 2,000 トークン以上を再処理した場合、そのリクエストをミスとしてカウントします。[キャッシュを無効化するアクション](/docs/ja/prompt-caching#actions-that-invalidate-the-cache) は通常の原因をリストしています。
  Claude Code が最後のミスの可能性のある原因を特定できる場合、行はそれも名前を付けます。例えば `likely cause: tool definitions changed` のようにです。可能性のある原因テキストには Claude Code v2.1.260 以降が必要です。
* **予想される再構築**: Claude Code が会話を再度書き直した場合。[圧縮](/docs/ja/prompt-caching#compacting-the-conversation) またはコンテキストから古いツール結果をクリアすることで、同じ種類のミスを予想される再構築としてカウントします。この部分は、少なくとも 1 つの予想される再構築が発生した後にのみ表示されます。
* **ウォーム状態またはコールド状態**: キャッシュされたプレフィックスが [キャッシュ有効期間](/docs/ja/prompt-caching#cache-lifetime) 内にあるかどうか。有効な TTL が表示されます。キャッシュがコールド状態の場合、行はセッションがアイドル状態だった期間を表示します。API がキャッシュトークンを報告していない場合、行は代わりに `no prompt caching reported by the API` で終わります。

カウントは API の応答のキャッシュトークンフィールドから取得されるため、行はすべてのプロバイダーとゲートウェイで機能します。メイン会話のみをカバーし、サブエージェントはカバーしません。`/clear` はセッションブロックの残りとともにリセットします。

ステータス行スクリプトは [`prompt_cache` オブジェクト](/docs/ja/statusline#prompt-cache-fields) から同じ数値を読み取ることができます。

<h4 id="plan-usage-breakdown">
  プラン使用量の内訳
</h4>

Pro、Max、Team、または Enterprise プランでは、`/usage` はプラン制限に対してカウントされるものの内訳も表示します。

* **属性**: スキル、サブエージェント、プラグイン、および個別の MCP サーバーに属性付けされた最近の使用量。それぞれが合計のパーセンテージとして表示されます。
  MCP サーバーのシェアは、そのツール結果の 1 つを消費したリクエストのみをカウントします。v2.1.222 より前では、MCP サーバーへの 1 回の呼び出しの後、Claude Code はその後のすべてのリクエストをそのサーバーに属性付けし、そのシェアを過大評価していました。
* **動作フラグ**: 長いコンテキストやキャッシュミスなどの動作。最近の使用量の 10% 以上を占める場合にフラグが付けられます。
* **ループ**: 最近実行された最も負荷の高い [`/loop` またはその他のスケジュール済みタスク](/docs/ja/scheduled-tasks) の行。合計トークン数で順序付けられ、残りのカウントが表示されます。Claude Code は各タスクの実行頻度、実行回数、合計トークンと実行ごとのトークン、および最後の実行時刻を報告します。Claude Code はタスクのプロンプトで行をキーにするため、停止して再作成したループは 1 つの行のままです。
  Claude Code v2.1.242 以降が必要です。

`d` または `w` を押して、過去 24 時間と過去 7 日間を切り替えます。数値は概算であり、このマシン上のローカルセッション履歴から計算されるため、他のデバイスまたは claude.ai からの使用量は含まれていません。

[VS Code 拡張機能](/docs/ja/vs-code#check-account-and-usage) では、属性シェアと動作フラグが Account & usage ダイアログに Day および Week トグルとともに表示されます。ループ行は含まれません。

<h4 id="check-your-usage-credits-spend">
  使用量クレジット支出を確認する
</h4>

`/usage` は [使用量クレジット](#add-usage-credits-to-your-subscription) がオンの間、使用量クレジット行も表示します。行に表示される内容はプランによって異なります。

* **Pro および Max**: 月間支出制限を設定している場合、その月間支出制限に対して測定された現在の月の支出。制限を設定していない場合、行は `Unlimited` を表示し、支出数値は表示されません。
* **Team および Enterprise**: 組織が設定した [制限](#claude-for-teams-and-enterprise) が適用される場合、現在の月の自分の支出。組織全体をカバーする制限は行に表示されません。自分の制限がない場合、行は制限なしで支出を表示します。使用量クレジットがオフの場合、`/usage` は使用量クレジット行を表示しません。

支出制限がある場合、使用量クレジットがオンになるとすぐに行が表示され、最初に使用量クレジットを支出するまで 0% を表示します。v2.1.236 より前では、`/usage` は Pro および Max プランでのみ行を表示し、支出制限のある行は何かを支出するまで非表示のままでした。

<h4 id="when-the-usage-request-fails">
  使用量リクエストが失敗した場合
</h4>

プラン制限のリクエストが失敗した場合（ほとんどの場合、使用量エンドポイントがレート制限されているため）、`/usage` は過去 60 分以内にこのマシンで読み込んだ最後の使用量バーを表示し、そのデータがいつ取得されたかを示す `Showing last-known usage` ノートが表示されます。`r` を押して再試行します。再試行が成功すると、最後に認識されたバーが新しいデータに置き換わります。過去 60 分以内のスナップショットがない場合、`/usage` は使用量エンドポイントがレート制限されていることを報告し、同じ再試行ショートカットを提供します。v2.1.208 より前では、使用量をまだ読み込んでいないセッションでレート制限されたリクエストは常にバーなしでエラーを表示していました。

<h3 id="analyze-your-usage-patterns">
  使用パターンを分析する
</h3>

[`/insights`](/docs/ja/commands#all-commands) を実行して、使用したトークン数ではなく、作業方法に関するレポートを取得します。このマシン上の最近のセッションを分析し、作業内容、誤解されたリクエストやバグのあるコードなどの摩擦点、および Claude Code をより効果的に使用するための提案をカバーする HTML レポートを作成します。1 回の実行は、まだ見たことのない最大 200 セッションを分析し、非常に短いセッションはスキップします。セッションが除外される場合、レポートヘッダーは分析されたカウントを括弧内の合計とともに表示します。例えば `200 sessions (412 total)` のようにです。

Claude Code は最新のレポートを `~/.claude/usage-data/report.html` に書き込み、各実行のタイムスタンプ付きコピーを同じディレクトリに保存するため、以前のレポートは上書きされません。Claude Code はセッションデータの残りと同じスケジュールでレポートを削除します。起動時に、[`cleanupPeriodDays`](/docs/ja/claude-directory#cleaned-up-automatically) より古いファイルを削除します。デフォルトは 30 日です。

任意のプランおよび任意のプロバイダーで `/insights` を実行できます。分析は通常のセッションと同じプロバイダーとアカウントを通じて実行され、トークンはプランまたは API 使用量に対してカウントされます。他のデバイスおよび claude.ai からのセッションは含まれていません。

<h3 id="add-usage-credits-to-your-subscription">
  サブスクリプションに使用量クレジットを追加する
</h3>

[使用量クレジット](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) を使用すると、プランの使用量制限を超えて作業を続けることができます。これらを管理するには、`/login` を通じて claude.ai サブスクリプションにサインインした後、`/usage-credits` を実行します。このコマンドは API キー認証では利用できません。
セルフサービス Enterprise 組織、Enterprise トライアル、および AWS Marketplace を通じて請求される Enterprise 組織では、コマンドには Claude Code v2.1.248 以降が必要です。以前のバージョンは [`Unknown command: /usage-credits`](/docs/ja/errors#unknown-command) で拒否します。開かれるものはロールによって異なります。

| ロール                                   | `/usage-credits` の動作                                                                                                                                        |
| :------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pro または Max サブスクライバー                  | ブラウザで [**Settings > Usage**](https://claude.ai/settings/usage) を claude.ai で開きます。**Usage credits** セクションで、使用量クレジットをオンまたはオフにし、クレジット残高、今月の支出、および月間支出制限を確認できます |
| 請求アクセス権を持つ Team または Enterprise メンバー   | 組織の使用量設定 [**Admin settings > Usage**](https://claude.ai/admin-settings/usage) をブラウザで開きます                                                                    |
| 請求アクセス権を持たない Team または Enterprise メンバー | 確認を求めてから、組織の管理者にリクエストを送信します。v2.1.211 より前では、Claude Code は確認ステップなしでリクエストを送信していました                                                                             |

請求アクセス権を持たない Team および Enterprise メンバーの場合、確認はインタラクティブセッションでのみ表示されます。`-p` フラグを使用した非インタラクティブモードおよび [Remote Control](/docs/ja/remote-control) からは、コマンドはリクエストを送信せず、インタラクティブセッションで実行するよう指示します。

以前のリクエストが管理者を待機している間に `/usage-credits` を再度実行した場合、Claude Code はリクエストが既に送信されたことを通知し、重複を送信しません。管理者がリクエストを却下した後、コマンドを再度実行すると新しいリクエストが送信されます。v2.1.222 より前では、却下されたリクエストも新しいリクエストをブロックしていました。

Pro および Max プランでは、支出制限に達しても使用量クレジットがまだ利用可能な場合、Claude Code は制限を引き上げるか削除するよう促し、CLI を離れることなく続行できます。サーバーが変更を拒否した場合、[Could not update your spend limit](/docs/ja/errors#could-not-update-your-spend-limit) を参照してください。

<h2 id="manage-costs-for-your-organization">
  組織のコストを管理する
</h2>

Claude Code にアクセスする方法によって、利用可能なコントロールが異なります。Claude for Teams または Enterprise プラン、Claude Console、またはクラウドプロバイダーです。Teams および Enterprise プランでは、使用量は各メンバーのシート割り当てから引き出されます。Console およびクラウドプロバイダーでは、使用量はトークンごとに組織に請求されます。組織がサインイン方法を混在させている場合、各開発者は認証した方法に従ってメーター化されます。

次の表は、各セットアップを、支出を確認する場所、支出をキャップする場所、およびユーザーごとの数値を取得する方法にマップしています。個別の Pro または Max プランでは、管理する組織がないため、[fast mode](/docs/ja/fast-mode#see-where-fast-mode-spend-appears) を含む [サブスクリプションに使用クレジットを追加](#add-usage-credits-to-your-subscription) の下で、独自の使用クレジット支出を追跡してください。

| セットアップ                                                                                 | 支出を確認                                                                                                                       | 支出をキャップ       | ユーザーごとのレポート                                                                                                                                                                                                       |
| :------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams または Enterprise](#claude-for-teams-and-enterprise)                    | [org analytics の支出レポート](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | 管理者設定の支出制限    | [支出レポート CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)、Enterprise の [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) |
| [Claude Console（API）](#claude-console)                                                 | [Console 使用状況ページ](https://platform.claude.com/usage)                                                                        | ワークスペース支出制限   | [Console ダッシュボード](https://platform.claude.com/claude-code)、[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                           |
| [Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry](#cloud-providers) | クラウド請求コンソール                                                                                                                 | クラウドの予算コントロール | [OpenTelemetry](/docs/ja/monitoring-usage) または [LLM gateway](/docs/ja/llm-gateway)                                                                                                                                          |

[OpenTelemetry エクスポート](/docs/ja/monitoring-usage) はすべてのセットアップで機能し、ユーザーごとのトークンおよびコストメトリクスをほぼリアルタイムで独自の可観測性スタックにストリーミングする唯一のオプションです。

<h3 id="report-spend-at-your-contracted-rates">
  契約レートで支出をレポートする
</h3>

デフォルトでは、Claude Code は開発者に表示するすべてのコスト数値を定価で計算するため、組織が契約レートを支払う場合、`/usage`、ステータス行、および OpenTelemetry の数値は請求書と一致しません。一致させるには、[`modelPricing`](/docs/ja/settings-reference#modelpricing) マネージド設定をレートに設定してください。この設定は Claude Code が報告する内容を変更し、Anthropic が請求する内容ではありません。Claude Code v2.1.242 以降が必要です。

<Steps>
  <Step title="契約からレートを取得する">
    契約からのトークンあたり百万単位のレートを入力してください。Claude Code は Claude Console から取得しないため、契約が変更されたときに設定を更新してください。
  </Step>

  <Step title="設定を書き込む">
    定価からのフラット割引に対して `multiplier` を 1 未満に設定するか、マークアップに対して 1 を超えて設定し、`overrides` の下に各モデルの 4 つのトークンあたりレートをリストするか、またはその両方を実行してください。マークアップには Claude Code v2.1.271 以降が必要です。[`modelPricing` エントリ](/docs/ja/settings-reference#modelpricing) には形状とペースト可能な例があります。
  </Step>

  <Step title="マネージド設定を通じてデプロイする">
    [マネージド設定](/docs/ja/managed-settings) として配信してください。サーバー管理設定、MDM ポリシー、`managed-settings.json`、または [ポリシーヘルパー](/docs/ja/managed-settings#compute-the-policy-with-a-helper-program)。Claude Code はユーザー、プロジェクト、ローカル設定、および `--settings` のキーを無視します。
  </Step>
</Steps>

レートが有効であることを確認するには、[マネージド設定を受け取った](/docs/ja/managed-settings#read-the-source-in-%2Fstatus) セッションで `/usage` を実行してください。Session ブロックの `Total cost` 行には `at your organization's configured rates` という注記が付きます。数値はまだ推定値であり、請求書ではありません。`/model` ピッカーのトークンあたり百万単位の価格は定価のままです。

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams および Enterprise
</h3>

Claude for Teams および Enterprise プランでは、各メンバーの Claude Code 使用量は、ローリング 5 時間ウィンドウと週間ウィンドウでリセットされるシート単位の割り当てから引き出されます。割り当ては Claude チャットおよび Cowork と共有され、そのサイズはメンバーの [シート層](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan)（Standard または Premium）に依存します。コントロールは Claude Console ではなく claude.ai 管理コンソールにあります。

* **支出を確認**: [org analytics の支出レポート](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) は、ユーザーごとおよびモデルごとの推定支出を CSV エクスポート付きで表示し、毎日更新されます。レポートは使用クレジット支出をカバーし、使用クレジットがオンになると表示されます。シート割り当て内の使用量はドルでメーター化されません。
* **採用を確認**: [analytics ダッシュボード](https://claude.ai/analytics/claude-code) は、日次アクティブユーザー、セッション、および貢献メトリクスを表示し、貢献データの CSV エクスポート付きです。[analytics でチーム使用状況を追跡](/docs/ja/analytics) を参照してください。
* **支出をキャップ**: シート割り当てはデフォルトの上限です。メンバーがそれを超えて続行できるようにするには、[使用クレジット](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) をオンにして、組織、グループ、または個別メンバーレベルで支出制限を設定します。
* **ユーザーごとの数値を取得**: Enterprise プランでは、[Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) は Claude Code を含む Claude サーフェス全体のユーザーごとの使用状況およびコストレポートを返します。Primary Owner は [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys) で `read:analytics` スコープを持つキーを作成します。Teams プランでは、[支出レポート CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) をエクスポートします。これはユーザーごとおよびモデルごとのトークン使用量と推定支出をリストします。

[Claude Enterprise 消費ガイド](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) は管理者向けの計画リファレンスです。Claude チャット、Claude Code、および Cowork 全体で消費がどのように異なるかを説明し、予算編成のためのユーザーごとのドル開始点を提供します。コーディングシートのチャットシートより多くの予算を計上してください。各 Claude Code ターンはファイルコンテンツ、ツール呼び出し、および多段階推論を含むため、1 つのデバッグセッションはチャットの 1 日分以上を消費できます。

<h3 id="claude-console">
  Claude Console
</h3>

API 組織は [ワークスペース](https://platform.claude.com/docs/en/build-with-claude/workspaces) を通じて Claude Code 支出を管理します。[ワークスペース支出制限を設定](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) して Claude Code 支出の合計をキャップし、Console で [コストと使用状況レポートを表示](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) できます。

<Note>
  Claude Code を Claude Console アカウントで初めて認証すると、「Claude Code」というワークスペースが自動的に作成されます。このワークスペースは、組織内のすべての Claude Code 使用量の一元化されたコスト追跡と管理を提供します。このワークスペースの API キーを作成することはできません。これは Claude Code 認証と使用量専用です。

  カスタムレート制限を持つ組織の場合、このワークスペースの Claude Code トラフィックは組織全体の API レート制限にカウントされます。Claude Console の Limits ページでこのワークスペースに [ワークスペースレート制限](https://platform.claude.com/docs/en/api/rate-limits#setting-lower-limits-for-workspaces) を設定して、Claude Code の共有をキャップし、他の本番ワークロードを保護できます。
</Note>

ユーザーごとのレポートについては、[Console ダッシュボード](https://platform.claude.com/claude-code) はメンバーごとの支出と受け入れられた行を表示し、[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) は [Admin API キー](https://platform.claude.com/settings/admin-keys) を使用してプログラムで同じ日次ユーザーごとのメトリクスを返します。[API カスタマー向けの analytics](/docs/ja/analytics#access-analytics-for-api-customers) を参照してください。

<h4 id="rate-limit-recommendations">
  レート制限の推奨事項
</h4>

チーム向けに Claude Code を設定する場合、組織のサイズに基づいて、これらのユーザーあたりのトークン/分（TPM）およびリクエスト/分（RPM）の推奨事項を検討してください。

| チームサイズ       | ユーザーあたり TPM | ユーザーあたり RPM |
| ------------ | ----------- | ----------- |
| 1～5 ユーザー     | 200k～300k   | 5～7         |
| 5～20 ユーザー    | 100k～150k   | 2.5～3.5     |
| 20～50 ユーザー   | 50k～75k     | 1.25～1.75   |
| 50～100 ユーザー  | 25k～35k     | 0.62～0.87   |
| 100～500 ユーザー | 15k～20k     | 0.37～0.47   |
| 500 ユーザー以上   | 10k～15k     | 0.25～0.35   |

たとえば、200 ユーザーがいる場合、各ユーザーに 20k TPM をリクエストするか、合計 400 万 TPM（200\*20,000 = 400 万）をリクエストできます。

チームサイズが大きくなるにつれて、ユーザーあたりの TPM は減少します。これは、より大きな組織では Claude Code を同時に使用するユーザーが少ない傾向があるためです。これらのレート制限は個別ユーザーレベルではなく組織レベルで適用されます。つまり、他のユーザーが積極的にサービスを使用していない場合、個別ユーザーは計算された共有量を一時的に超えて消費できます。

<Note>
  大規模グループとのライブトレーニングセッションなど、異常に高い同時使用シナリオが予想される場合は、ユーザーあたりのより高い TPM 割り当てが必要になる場合があります。
</Note>

<h3 id="cloud-providers">
  クラウドプロバイダー
</h3>

Amazon Bedrock、Google Cloud の Agent Platform、および Microsoft Foundry では、Claude Code はトークンごとにクラウドアカウントに請求され、支出コントロールはクラウドプロバイダーの請求コンソールにあります。Claude Code はクラウドからメトリクスを Anthropic に送信しないため、[analytics ダッシュボード](/docs/ja/analytics) および Claude Code Analytics API はこの使用量をカバーしません。

ユーザーごとのコスト帰属については、3 つのオプションがあります。

* **OpenTelemetry**: 各開発者のマシンから [メトリクスをエクスポート](/docs/ja/monitoring-usage) して、独自の可観測性スタックに送信します。これにより、プロバイダーに関係なく、ユーザーごとのトークンカウント、コスト、およびツールアクティビティが得られます。
* **Claude apps gateway**: セルフホストされた [Claude apps gateway](/docs/ja/claude-apps-gateway) は、ユーザーごとの使用状況帰属、トークンカウント付きの OTLP メトリクス、およびこれらのプロバイダーの [ユーザーごとの支出制限](/docs/ja/claude-apps-gateway-spend-limits) を提供します。
* **LLM gateway**: すべての Claude Code トラフィックをキーごとに支出を追跡するプロキシを通じてルーティングします。複数の大規模企業は [LiteLLM](/docs/ja/llm-gateway) を使用していると報告しており、これはオープンソースツールで [キーごとに支出を追跡](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend) します。このプロジェクトは Anthropic と提携していないため、セキュリティについて監査されていません。

<h3 id="when-a-developer-asks-about-a-limit">
  開発者が制限について質問する場合
</h3>

開発者は通常、制限に関する質問を管理者に持ち込むため、どの上限に達したかを知ることが役立ちます。これらの状況は異なることを意味します。

* **「セッション制限に達しました」または「週間制限に達しました」**: サブスクリプションプランのシートベースの使用ウィンドウ。これらのウィンドウはすべてのモデル全体で共有されるため、開発者は `/model` でモデルを切り替えてアクセスを復元することはできません。メッセージはウィンドウがリセットされるときを表示します。モデル固有の「Opus 制限に達しました」または「Sonnet 制限に達しました」メッセージの後、`/model` でそのファミリー外のモデルに切り替えると、開発者は作業を続けることができます。[使用制限エラー](/docs/ja/errors#youve-hit-your-session-limit) を参照してください。開発者がその間にできることは以下の通りです。
  * [使用クレジット](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) がオンになっている場合、`/usage-credits` を実行して割り当てを超えた使用をリクエストしてください。
  * Claude Code v2.1.234 以降では、[リセット後に中断されたタスクを自動的に待機して続行](/docs/ja/interactive-mode#wait-for-a-usage-limit-to-reset) してください。そのセクションでは、Claude Code がいつ自動的に待機を開始するか、および開発者が `/rate-limit-options` からそれを選択するときを一覧表示しています。フリート全体で Claude Code が自動的に待機を開始するかどうかを制御するには、[マネージド設定](/docs/ja/settings#settings-precedence) で [`autoContinueAtUsageLimit`](/docs/ja/settings-reference#autocontinueatusagelimit) を設定してください。
* **「個別支出制限に達しました」、「組織の月間支出制限」、または「チームの共有予算」**: 開発者のリクエストは使用クレジットに請求され、それらのクレジットは設定した支出制限に達しています。開発者が続行できるようにするには、[**管理者設定 > 使用状況**](https://claude.ai/admin-settings/usage) に移動して、メッセージが名前を付ける制限を増やしてください。メッセージがプラン リセット時間も名前を付ける場合、開発者は代わりにそれまで待つことができます。各バリアントについては、[エラー リファレンス](/docs/ja/errors#youve-hit-your-monthly-spend-limit) を参照してください。
* **[Claude apps gateway](/docs/ja/claude-apps-gateway) からの支出制限メッセージ**: 開発者はセルフホストされたゲートウェイに設定した支出上限を超過し、ゲートウェイは期間がリセットされるか上限が引き上げられるまでリクエストをブロックします。[ゲートウェイ支出制限](/docs/ja/claude-apps-gateway-spend-limits) で上限、リセットスケジュール、および開発者が見るメッセージを参照してください。
* **コンテキストまたは auto-compact 警告**: 使用制限ではありません。会話がセッションの [auto-compact ウィンドウ](/docs/ja/model-config#set-the-auto-compact-window) に近づいており、Claude Code が古い履歴を要約して領域を解放するしきい値です。開発者を [トークン使用量を削減](#reduce-token-usage) に指してください。
* **API またはクラウドプロバイダープランで予期しない高い支出**: 通常、クリアされたことのない長いセッション、または Opus がデフォルトモデルとして残されていることに遡ります。共有する最も影響の大きい習慣は、関連のないタスク間でクリアすることとジョブにモデルを一致させることの両方で、[トークン使用量を削減](#reduce-token-usage) でカバーされています。

<h3 id="agent-team-token-costs">
  エージェントチームのトークンコスト
</h3>

[エージェントチーム](/docs/ja/agent-teams) は複数の Claude Code インスタンスを生成し、各インスタンスは独自のコンテキストウィンドウを持ちます。トークン使用量はアクティブなチームメイトの数と各チームメイトが実行される期間に応じてスケーリングされます。

エージェントチームのコストを管理可能に保つには、以下を実行してください。

* チームメイトに Sonnet を使用します。これは調整タスクの機能とコストのバランスを取ります。
* チームを小さく保ちます。各チームメイトは独自のコンテキストウィンドウを実行するため、トークン使用量はおおよそチームサイズに比例します。
* スポーンプロンプトを焦点を絞ったものにします。チームメイトは CLAUDE.md、MCP サーバー、およびスキルを自動的に読み込みますが、スポーンプロンプト内のすべてが最初からコンテキストに追加されます。
* 作業が完了したらチームをシャットダウンします。アクティブなチームメイトはアイドル状態でもトークンを消費し続けます。
* エージェントチームはデフォルトで無効になっています。[settings.json](/docs/ja/settings) または環境で `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` を設定して有効にします。[エージェントチームを有効にする](/docs/ja/agent-teams#enable-agent-teams) を参照してください。

<h2 id="reduce-token-usage">
  トークン使用量を削減する
</h2>

トークンコストはコンテキストサイズに応じてスケールします。Claude が処理するコンテキストが大きいほど、より多くのトークンを使用します。Claude Code は [プロンプトキャッシング](/docs/ja/prompt-caching) を通じてコストを自動的に最適化します。プロンプトキャッシングはシステムプロンプトなどの繰り返されるコンテンツのコストを削減し、auto-compact はコンテキスト制限に近づいたときに会話履歴を要約します。

以下の戦略は、コンテキストを小さく保ち、メッセージあたりのコストを削減するのに役立ちます。

<h3 id="manage-context-proactively">
  コンテキストを積極的に管理する
</h3>

`/usage` を使用して現在のトークン使用量を確認するか、[ステータスラインを設定](/docs/ja/statusline#context-window-usage) して継続的に表示します。

* **タスク間でクリアする**: 関連のない作業に切り替えるときは `/clear` を使用して新しく開始します。古いコンテキストは後続のすべてのメッセージでトークンを無駄にします。クリアする前に `/rename` を使用してセッションに名前を付けると、後で簡単に見つけることができます。その後 `/resume` を使用して復帰します。
* **カスタムコンパクション指示を追加する**: `/compact Focus on code samples and API usage` は、要約中に保持する内容を Claude に指示します。新しいセッションでは、会話履歴がまだないため `/compact` は `Not enough messages to compact.` と出力します。

プロジェクトのルートにある CLAUDE.md ファイルでコンパクション動作をカスタマイズすることもできます。

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  適切なモデルを選択する
</h3>

Sonnet はほとんどのコーディングタスクをうまく処理でき、Opus よりもコストが低くなります。複雑なアーキテクチャの決定や多段階の推論には Opus を予約します。`/model` を使用してセッション中にモデルを切り替えるか、`/config` でデフォルトを設定します。Opus への切り替えは [セッションのモデルを継承するサブエージェント](/docs/ja/model-config#setting-your-model) にも適用されます。シンプルなサブエージェントタスクの場合は、[サブエージェント設定](/docs/ja/sub-agents#choose-a-model) で `model: haiku` を指定します。

<h3 id="reduce-mcp-server-overhead">
  MCP サーバーのオーバーヘッドを削減する
</h3>

MCP ツール定義は [デフォルトで遅延](/docs/ja/mcp#scale-with-mcp-tool-search) されるため、Claude が特定のツールを使用するまで、ツール名とサーバー指示のみがコンテキストに入ります。`/context` を実行して、何がスペースを消費しているかを確認します。

* **利用可能な場合は CLI ツールを優先する**: `gh`、`aws`、`gcloud`、`sentry-cli` などのツールは、ツールごとのリストを追加しないため、MCP サーバーよりもコンテキスト効率が高いです。Claude は CLI コマンドを直接実行できます。
* **未使用のサーバーを無効にする**: `/mcp` を実行して設定されたサーバーを確認し、積極的に使用していないサーバーを無効にします。

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  型付き言語用のコード インテリジェンスプラグインをインストールする
</h3>

[コード インテリジェンスプラグイン](/docs/ja/plugins/code-intelligence) は、Claude にテキストベースの検索の代わりに正確なシンボルナビゲーションを提供し、不慣れなコードを探索するときの不要なファイル読み取りを削減します。単一の「定義に移動」呼び出しは、複数の候補ファイルを読み取る前に grep を実行する必要があったものを置き換えます。インストールされた言語サーバーは、編集後に型エラーを自動的に報告するため、Claude はコンパイラを実行せずにエラーをキャッチします。

<h3 id="offload-processing-to-hooks-and-skills">
  hooks と skills に処理をオフロードする
</h3>

カスタム [hooks](/docs/ja/hooks) は Claude がそれを見る前にデータを前処理できます。Claude が 10,000 行のログファイルを読んでエラーを見つける代わりに、hook は `ERROR` に対して grep を実行し、一致する行のみを返すことができます。これにより、コンテキストを数万トークンから数百に削減します。

[skill](/docs/ja/skills) は Claude にドメイン知識を与えることができるため、探索する必要がありません。たとえば、「codebase-overview」skill はプロジェクトのアーキテクチャ、主要なディレクトリ、命名規則を説明できます。Claude が skill を呼び出すと、構造を理解するために複数のファイルを読むトークンを費やす代わりに、このコンテキストが即座に得られます。

たとえば、この PreToolUse hook はテスト出力をフィルタリングして失敗のみを表示します。

<Tabs>
  <Tab title="settings.json">
    これを [settings.json](/docs/ja/settings#where-settings-live) に追加して、すべての Bash コマンドの前に hook を実行します。

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    hook はこのスクリプトを呼び出します。`mkdir -p ~/.claude/hooks` でフォルダを作成し、以下のスクリプトを `~/.claude/hooks/filter-test-output.sh` として保存し、`chmod +x ~/.claude/hooks/filter-test-output.sh` で実行可能にします。コマンドがテストランナーであるかどうかをチェックし、失敗のみを表示するように変更します。

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "$input" | jq --arg filtered "$filtered_cmd" \
        '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "allow", updatedInput: (.tool_input + {command: $filtered})}}'
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

セットアップを確認するには、`/hooks` を実行して、hook が PreToolUse の下に表示されることを確認します。`claude --debug-file ./claude-debug.txt` で Claude Code を起動し、Claude に `npm test` を実行するよう依頼することもできます。hook がコマンドを書き直すと、そのログファイルには `command` と他の Bash 入力フィールドをリストする `modified tool input keys` 行が含まれます。

<h3 id="move-instructions-from-claude-md-to-skills">
  CLAUDE.md から skills に指示を移動する
</h3>

[CLAUDE.md](/docs/ja/memory) ファイルはセッション開始時にコンテキストに読み込まれます。PR レビューやデータベースマイグレーションなどの特定のワークフロー用の詳細な指示が含まれている場合、関連のない作業を行っているときでもそれらのトークンが存在します。[Skills](/docs/ja/skills) はオンデマンドでのみ呼び出されたときに読み込まれるため、特殊な指示を skills に移動することで、ベースコンテキストを小さく保ちます。必須項目のみを含めることで、CLAUDE.md を 200 行以下に保つことを目指します。

<h3 id="adjust-extended-thinking">
  拡張思考を調整する
</h3>

拡張思考はデフォルトで有効になっています。複雑な計画と推論タスクのパフォーマンスを大幅に向上させるためです。思考トークンは出力トークンとして課金され、デフォルトの予算はモデルに応じてリクエストあたり数万トークンになる可能性があります。

深い推論が不要なシンプルなタスクの場合は、`/effort` または `/model` で [努力レベル](/docs/ja/model-config#adjust-effort-level) を下げるか、`/config` で思考を無効にすることでコストを削減できます。Opus 5.5 または Fable モデルでは思考をオフにすることはできません。これらは常に拡張思考を使用します。

[固定思考予算](/docs/ja/model-config#adaptive-reasoning-and-fixed-thinking-budgets) を持つモデルでは、`MAX_THINKING_TOKENS` [環境変数](/docs/ja/env-vars) を設定することで予算を下げることもできます。たとえば `MAX_THINKING_TOKENS=8000` です。適応推論モデルはゼロ以外の予算を無視するため、代わりに努力レベルを使用します。

<h3 id="delegate-verbose-operations-to-subagents">
  冗長な操作をサブエージェントに委譲する
</h3>

テストの実行、ドキュメントの取得、またはログファイルの処理は、かなりのコンテキストを消費する可能性があります。これらを [サブエージェント](/docs/ja/sub-agents#isolate-high-volume-operations) に委譲して、冗長な出力がサブエージェントのコンテキストに留まり、要約のみがメインの会話に返されるようにします。

<h3 id="manage-agent-team-costs">
  エージェントチームのコストを管理する
</h3>

エージェントチームは、チームメイトが plan mode で実行されるときに標準セッションよりも約 7 倍多くのトークンを使用します。各チームメイトが独自のコンテキストウィンドウを維持し、別の Claude インスタンスとして実行されるためです。チームタスクを小さく自己完結させて、チームメイトあたりのトークン使用量を制限します。詳細は [エージェントチーム](/docs/ja/agent-teams) を参照してください。

<h3 id="write-specific-prompts">
  具体的なプロンプトを作成する
</h3>

「このコードベースを改善する」のような曖昧なリクエストは広範なスキャンをトリガーします。「auth.ts のログイン関数に入力検証を追加する」のような具体的なリクエストは、Claude が最小限のファイル読み取りで効率的に作業できるようにします。

<h3 id="work-efficiently-on-complex-tasks">
  複雑なタスクで効率的に作業する
</h3>

より長いまたはより複雑な作業の場合、これらの習慣は間違った方向に進むことからのトークン浪費を回避するのに役立ちます。

* **複雑なタスクに plan mode を使用する**: Shift+Tab を押して実装前に [plan mode](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode) に切り替えます。Claude はコードベースを探索し、承認のためのアプローチを提案し、初期の方向が間違っている場合の高額な再作業を防ぎます。
* **早期に方向を修正する**: Claude が間違った方向に向かい始めたら、Escape を押してすぐに停止します。`/rewind` を使用するか、Escape をダブルタップして、会話とコードを以前のチェックポイントに復元します。
* **検証ターゲットを指定する**: テストケースを含める、スクリーンショットを貼り付ける、またはプロンプトで予想される出力を定義します。Claude が独自の作業を検証できる場合、修正をリクエストする必要がある前に問題をキャッチします。
* **段階的にテストする**: 1 つのファイルを作成し、テストしてから続行します。これにより、修正が安価なときに早期に問題をキャッチします。

<h2 id="background-token-usage">
  バックグラウンドトークン使用量
</h2>

Claude Code はアイドル状態でも、バックグラウンド機能にトークンを使用します。

* **会話要約**: `claude --resume` 機能の前の会話を要約するバックグラウンドジョブ
* **コマンド処理**: `/usage` などの一部のコマンドは、ステータスを確認するためにリクエストを生成する場合があります

これらのバックグラウンドプロセスは、アクティブなインタラクションがなくても、少量のトークン（通常はセッションあたり \$0.04 未満）を消費します。

プロンプト提案がオンの場合、Claude Code は Claude が応答した後、セッションが使用しているモデルに短いリクエストを送信して、[次のプロンプトを提案](/docs/ja/interactive-mode#prompt-suggestions)します。そのリクエストは会話のプロンプトキャッシュを再利用するため、ほぼキャッシュ読み取りと少数の出力トークンです。Claude Code は[アカウントが使用量制限に近い、または達している場合、提案をスキップ](/docs/ja/interactive-mode#when-claude-code-skips-suggestions)します。これらのリクエストを停止するには、[プロンプト提案をオフにしてください](/docs/ja/interactive-mode#turn-prompt-suggestions-off)。

<h2 id="why-usage-climbs-in-a-long-session">
  長いセッションで使用量が増加する理由
</h2>

数時間開いているセッションは、アクティビティが示唆するよりもはるかに多くのプラン制限を使用する可能性があります。通常、以下のいずれかの理由によります。

* **長いコンテキスト**: Claude Code はすべてのリクエストで完全な会話を送信し、Claude がツールを使用するたびに、そのツール結果のバッチを含む別のリクエストを送信します。[プロンプトキャッシング](/docs/ja/prompt-caching)を使用すると、Claude Code はその履歴を[キャッシュされたトークンレート](https://platform.claude.com/docs/en/about-claude/pricing)で再度読み込むため、一日中開いているセッションの 1 行の質問でも、会話全体の使用量が発生します。コンテキストを小さく保つ方法については、[コンテキストを積極的に管理する](#manage-context-proactively)を参照してください。
* **キャッシュミス**: [キャッシュライフタイム](/docs/ja/prompt-caching#cache-lifetime)より長い休止後の最初のメッセージはキャッシュをミスし、完全なコンテキストを再処理します。ライフタイムはサブスクリプションで 1 時間で、[使用クレジット](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)を使用している場合は 5 分に短縮されます。API キーまたはクラウドプロバイダーでは、デフォルトで 5 分です。使用クレジットを使用しながら 1 時間のライフタイムを保つには、[TTL を自分で選択](/docs/ja/prompt-caching#choose-the-ttl-yourself)してください。Pro および Max プランでは、長い休止後に大規模なセッションを再開する場合、Claude Code は[サマリーから再開することを提案](/docs/ja/sessions#resume-from-a-summary)し、後のリクエストが完全な履歴を含まないようにします。
* **スケジュール済みタスク**: [スケジュール済みタスク](/docs/ja/scheduled-tasks)はセッションがアイドル状態でも間隔で実行され、毎回完全なコンテキストを送信します。
* **クロスセッションメッセージ**: Claude Code は[別のセッションからのメッセージ](/docs/ja/cross-session-messaging)をこのセッションがアイドル状態のときに新しいターンとして配信し、毎回完全なコンテキストを送信します。受信メッセージを配信する代わりに保持するには、[`crossSessionInbound`](/docs/ja/settings-reference#crosssessioninbound)を`hold`に設定してください。
* **ゴールチェックイン**: バックグラウンド作業がアクティブな[ゴール](/docs/ja/goal)を待機させている間、Claude Code はセッションがアイドル状態でも[その作業をチェックするよう Claude に要求](/docs/ja/goal#background-work-defers-evaluation)し、完全なコンテキストを送信する新しいターンを開始します。Claude Code はプロンプト間で最大 3 つのアイドルチェックインをゴールごとに開始します。v2.1.246 より前は、アイドルチェックインは無制限でした。チェックインをオフにするには、[`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/ja/env-vars)を`0`に設定してください。アイドルチェックインには Claude Code v2.1.236 以降が必要です。
* **エージェントチームメイト**: アクティブな[チームメイト](#agent-team-token-costs)ごとに、終了するまでトークンを消費し続けます。
* **コンパクション**: `/compact`は要約するコンテキストを読み込むため、[大規模なコンテキストをコンパクトにする](/docs/ja/prompt-caching#compacting-the-conversation)こと自体が大規模なリクエストです。継続性ではなく新しいスタートが必要な場合、`/clear`はコストがかかりません。

Pro、Max、Team、または Enterprise プランでは、`/usage`の内訳は長いコンテキストやキャッシュミスなど、最近の使用量の 10% 以上を占める動作にフラグを立て、それぞれ削減するためのヒントを提供します。

<h2 id="understanding-changes-in-claude-code-behavior">
  Claude Code の動作の変更を理解する
</h2>

Claude Code は定期的に更新を受け取り、機能の動作方法（コスト報告を含む）が変わる可能性があります。`claude --version` を実行して、現在のバージョンを確認してください。

アカウント固有の請求に関する質問については、製品内メッセンジャーを通じて Anthropic サポートにお問い合わせください。

* **サブスクリプションプラン**（Pro、Max、Team、Enterprise）：[claude.ai](https://claude.ai) にサインインし、左下のイニシャルをクリックして、**Get help** を選択してください
* **Console（API）請求**：[platform.claude.com](https://platform.claude.com) にサインインし、イニシャルをクリックして、**Get help** を選択してください

各プランで人間のエージェントに連絡できるユーザーを含む完全なフローについては、[How to get support](https://support.claude.com/en/articles/9015913-how-to-get-support) を参照してください。
