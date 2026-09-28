> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインのコストと使用状況を測定する

> Claude Code プラグインのトークンコスト、人々がまだそれを使用しているかどうかを確認し、組織全体のプラグイン質問のテレメトリイベントを選択します。

プラグインが有効になっているすべてのセッションには、そのスキル、エージェント、コマンドの名前と説明が Claude のコンテキストに含まれ、プラグインが実際に使用されるかどうかに関わらず、これらのトークンはユーザーの使用量にカウントされます。このページでは、プラグインのこの数値を確認する方法、プラグインを保守している場合にそれを削減する方法、および使用状況がどこに表示されるかを示し、プラグインがまだ使用されているかどうかを判断できます。

このページはプラグイン作成者とメンテナー向けです。組織の Claude Code を管理している場合は、[フロート全体で測定する](#measure-across-a-fleet)で、すべてのマシン全体で同じ質問をカバーしています。

<Note>
  これらのケースは他のページでカバーされています：

  * **プラグインが Claude の動作をどの程度確実に変更するかをテストする**：[プラグインを evals でテストする](/docs/ja/plugin-evals)を参照してください
  * **独自のセッションのコンテキストをトリミングする**：[インストール済みプラグインを管理する](/docs/ja/plugins/install#manage-installed-plugins)と[コンテキストウィンドウ](/docs/ja/context-window)ページを参照してください
</Note>

[プラグインのコストを測定する](#measure-what-a-plugin-costs)から始めてください。

<h2 id="measure-what-a-plugin-costs">
  プラグインのコストを測定する
</h2>

プラグインが Claude のコンテキストに何を追加するかを確認するには、プラグインの名前を指定して [`claude plugin details`](/docs/ja/plugins/cli-reference#plugin-details) を実行します。これは実行中の Claude Code セッションのプロンプトではなく、シェルで実行します。プラグインはロードされている必要があります：インストール済み、スキルディレクトリ内、または同じコマンドで `--plugin-dir` で渡されている必要があります。例えば `claude --plugin-dir ./formatter plugin details formatter` のようにです。

この例は、2 つのスキル、コマンド、エージェント、フック、MCP サーバーを持つ `formatter` という名前のインストール済みプラグインを読み込みます：

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

出力の各部分は異なる質問に答えます：

* **Component inventory**：Claude Code がプラグインで見つけたもの。コマンドはスキルと一緒にカウントされるため、`format-all` は `Skills` の下に表示されます。フックと MCP サーバーはコスト推定値を取得せず、コンポーネント行もありません。プラグインの MCP ツールが何を追加するかを確認するには、プラグインが有効になっているセッションで `/context` を実行し、`MCP tools` カテゴリを読んでください。
* **Always-on**：プラグインのスキル、エージェント、コマンドの名前と説明が、プラグインが有効になっているすべてのセッションに追加するトークン。何も実行されるかどうかに関わらず、これはすべてのユーザーが持つ数値であり、削減する必要があるものです。
* **Per-component**：各行は 1 つのスキル、エージェント、またはコマンドを、その always-on シェアと on-invoke コストに分割します。on-invoke コストはそのコンポーネントが実行されるときにのみロードされる本体です。always-on 列を使用して、どのコンポーネントが最も貢献しているかを見つけます。

<h3 id="lower-the-always-on-figure">
  always-on の数値を下げる
</h3>

プラグインを保守している場合、これらの変更はそれがすべてのセッションに追加するものを削減します。それを使用するだけの場合、オプションはそれを無効にするか、アンインストールすることです。[インストール済みプラグインを管理する](/docs/ja/plugins/install#manage-installed-plugins)を参照してください。

always-on の数値は、各コンポーネントの名前とその `description` および `when_to_use` frontmatter をカウントします。それを下げるには：

* スキルとエージェントの説明を短くします。
* 大きなプラグインを分割して、ユーザーが必要なコンポーネントのみをインストールできるようにします。

スキルの説明は、Claude がリクエストと照合するものでもあるため、短いものはスキルのトリガーを停止できます。説明をトリミングした後、eval スイートで [`tool_used: Skill` grader](/docs/ja/plugin-evals#create-your-first-eval-suite) を使用してトリガーをチェックしてください。

各コンポーネントタイプが何を貢献するかについては、[プラグインコンポーネント](/docs/ja/plugins/components)を参照してください。

<h3 id="cost-shown-to-users-before-install">
  インストール前にユーザーに表示されるコスト
</h3>

公式マーケットプレイスのプラグインは、インストール前にユーザーにコストを表示します。`/plugin` では、ユーザーがマーケットプレイスのプラグインリストを参照してプラグインを選択すると、詳細ペインに **Context cost** セクションが表示され、`Every turn:` 行と `When invoked:` 行があります。always-on の数値が 2,000 トークン以上の場合、`Every turn:` 行がハイライト表示されます。

独自のマーケットプレイスのプラグインには **Context cost** セクションがありません。

<h2 id="check-whether-a-plugin-is-used">
  プラグインが使用されているかどうかを確認する
</h2>

Claude Code はプラグインの使用状況をその作成者に報告しません。使用状況はプラグインをインストールした各人のマシンに記録されるため、学べることはそれらの人との関係に依存します：

* **組織の Claude Code を管理している**：OpenTelemetry イベントと Analytics API は、すべてのマシン全体でインストールとスキルアクティベーションをカウントします。[フロート全体で測定する](#measure-across-a-fleet)を参照してください。
* **質問できるチームメイト**：各ユーザーの独自の Claude Code は、プラグインをまだ使用しているかどうかを 4 つの場所で表示します：[`/plugin` パネル](#not-used-recently-in-/plugin)、[`/skill-doctor`](#find-skills-that-never-run)、[`/doctor`](#unused-plugins-in-/doctor)、および [`/usage`](#usage-share-in-/usage)。これら 4 つはすべて、ユーザーが独自のマシン上のセッションで Claude Code プロンプトで実行するコマンドです。
* **どちらでもない**：そのプラグインの Claude Code からの使用信号がありません。

<h3 id="not-used-recently-in-/plugin">
  `/plugin` で最近使用されていない
</h3>

`/plugin` の **Installed** タブで、ユーザーがマーケットプレイスからインストールしたプラグインは、少なくとも 14 日間と 10 セッション未使用になると、**Not used recently** ヘッダーの下に移動します。プラグインの詳細には `Last used:` 行も表示されます。ユーザーがそのヘッダーと行で何をするかについては、[使用しなくなったプラグインを見つける](/docs/ja/plugins/install#find-plugins-you-no-longer-use)を参照してください。

**Not used recently** ヘッダーは以下の場合には表示されません：

* `--plugin-dir` またはスキルディレクトリからロードされたプラグイン
* 管理設定を通じて有効にされたプラグイン、または[シードディレクトリ](/docs/ja/plugins/org#seed-containers-and-ci)からマウントされたプラグイン
* テーマ、出力スタイル、モニター、またはワークフローを含むプラグイン。これらは追跡されたインボケーションなしで使用中であるため

プラグインの[言語サーバー](/docs/ja/plugins/components#lsp-servers)は、診断を提供するか、コードナビゲーションリクエストに応答するときに使用中としてカウントされるため、サーバーがセッションでアクティブな LSP プラグインは未使用としてリストされません。

ユーザーの組織が [`strictKnownMarketplaces`](/docs/ja/plugins/org#restrict-what-users-can-install) を設定する場合、ヘッダーも `Last used:` 行も表示されません。

<h3 id="find-skills-that-never-run">
  実行されないスキルを見つける
</h3>

`/skill-doctor` を実行して、各スキルのコストと使用頻度を確認します。Claude のスキルリストに含まれているが、プラグインからのスキルを含め、呼び出されたことのないスキルにフラグを立てます。

インタラクティブセッションでは、レポートは `/plugin` マネージャーの **Stats** タブで開きます。レポートがカバーする内容と利用可能な場所については、[未使用のスキルを見つける](/docs/ja/skills#find-unused-skills)を参照してください。

<h3 id="unused-plugins-in-/doctor">
  `/doctor` で未使用のプラグイン
</h3>

`/doctor` チェックアップは、各ユーザーがインストールしたスキル、MCP サーバー、プラグインをリストし、使用されなかったものを無効にすることを推奨します。[コマンドリファレンスの `/doctor`](/docs/ja/commands#all-commands) を参照してください。

<h3 id="usage-share-in-/usage">
  `/usage` での使用状況シェア
</h3>

Pro、Max、Team、または Enterprise プランでは、`/usage` の内訳は最近の使用状況をスキル、サブエージェント、プラグイン、MCP サーバーの合計のシェアとして属性付けします。[`/usage` コマンドを使用する](/docs/ja/costs#using-the-/usage-command)を参照してください。

<h2 id="measure-across-a-fleet">
  フロート全体で測定する
</h2>

組織の Claude Code を管理している場合、以下のいずれかのソースからすべてのマシン全体でプラグインのコストと使用状況を測定できます：

* **OpenTelemetry イベント**：[エクスポーターを設定](/docs/ja/monitoring-usage)した後、Claude Code はこれらを独自のバックエンドにエクスポートします。[プラグインのインストールと使用のための OpenTelemetry イベント](#pick-the-opentelemetry-event-for-each-question)を参照してください。
* **Analytics API**：エクスポーターが不要な Anthropic のレコードから提供されます。[Analytics API をクエリする](#query-the-analytics-api)を参照してください。

<h3 id="pick-the-opentelemetry-event-for-each-question">
  プラグインのインストールと使用のための OpenTelemetry イベント
</h3>

これらの OpenTelemetry イベントと属性は、バックエンドから各プラグイン質問に答えます：

| 質問                                 | OpenTelemetry イベントまたは属性                                                                                                      |
| :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| どのプラグインがインストールされ、どこからインストールされるか    | [`claude_code.plugin_installed`](/docs/ja/monitoring-usage#plugin-installed-event)、インストールごとに 1 つ                                  |
| どのプラグインが何個のセッションでアクティブであるか         | [`claude_code.plugin_loaded`](/docs/ja/monitoring-usage#plugin-loaded-event)、セッション開始時に有効なプラグインごとに 1 つ                             |
| どのスキルがアクティベートされ、どのプラグインがそれを所有しているか | [`claude_code.skill_activated`](/docs/ja/monitoring-usage#skill-activated-event)、プラグインスキルの `plugin.name` と `marketplace.name` を使用 |
| プラグインのフックが報告するもの                   | [`claude_code.hook_plugin_metrics`](/docs/ja/monitoring-usage#hook-plugin-metrics-event)、公式マーケットプレイスプラグインのフックに対してのみ発行             |
| プラグインが API 支出でコストするもの              | [コストカウンター](/docs/ja/monitoring-usage#cost-counter)の `plugin.name` と `marketplace.name`、アクティブなスキルまたはサブエージェントがプラグインに属する場合に設定        |

<h3 id="redacted-plugin-names-in-your-backend">
  バックエンドでのプラグイン名の編集
</h3>

公式マーケットプレイスのプラグインは、プラグイン名とマーケットプレイス名をバックエンドに逐語的に報告します。他のすべてのプラグインの名前は、組織独自のマーケットプレイスのプラグインを含め、デフォルトでは編集されるか省略されます。プラグインの[信頼レベル](/docs/ja/plugins/security#find-plugins-in-telemetry)がどれを決定するかを決定します。

一部のイベントで実名を取得するには、テレメトリをエクスポートするマシンで [`OTEL_LOG_TOOL_DETAILS`](/docs/ja/monitoring-usage#common-configuration-variables) 環境変数を `1` に設定します。例えば、エクスポーターを設定する同じ[管理設定](/docs/ja/monitoring-usage#administrator-configuration)の `env` ブロックで：

| イベント                                 | デフォルト                                                                                     | `OTEL_LOG_TOOL_DETAILS=1` の場合            |
| :----------------------------------- | :---------------------------------------------------------------------------------------- | :--------------------------------------- |
| `plugin_loaded`                      | `plugin.name` と `marketplace.name` はリテラル文字列 `third-party`                                 | 実名                                       |
| `plugin_installed`、`skill_activated` | `plugin.name` と `marketplace.name` は省略；`skill_activated` では `skill.name` は `custom_skill` | 実名                                       |
| コストカウンター                             | `plugin.name` は `third-party`；`marketplace.name` は不在                                      | 実 `plugin.name`；`marketplace.name` は依然不在 |

`plugin_loaded` では、`plugin_id_hash` はデフォルトで各プラグインを識別するため、個別のサードパーティプラグインをカウントできます。

<h3 id="query-the-analytics-api">
  Analytics API をクエリする
</h3>

Enterprise プランでは、Analytics API は Anthropic のレコードから「組織がどのプラグインをインストールして呼び出すか」に答え、エクスポーターは不要です。[`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) は Claude Code と Cowork 全体でプラグインごと、1 日ごとのインストールと呼び出しカウントを返し、ユーザー、RBAC グループ、または製品でグループ化できます。

プラグイン名なしで Anthropic に到達するプラグインアクティビティは、1 つの集約 `third-party` 行に表示されます。[テレメトリでプラグインを見つける](/docs/ja/plugins/security#find-plugins-in-telemetry)は、Claude Code が名前で報告するプラグインを示します。

`read:analytics` スコープを持つ API キーでリクエストを認証します。これは、[プログラムでデータにアクセスする](/docs/ja/analytics#access-data-programmatically)の下で説明されているように、プライマリオーナーが作成します。

パラメーターと応答フィールドについては、[エンドポイントリファレンス](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグインを evals でテストする](/docs/ja/plugin-evals)：プラグインがコストするだけでなく、Claude をどの程度確実に操舵するかを測定します
* [always-on の数値を下げる](#lower-the-always-on-figure)：プラグインのターンごとのコストを削減するために変更する内容
* [プラグインのセキュリティと信頼](/docs/ja/plugins/security#find-plugins-in-telemetry)：どのテレメトリフィールドがプラグイン名を持ち、いつ編集されるか
* [使用状況の監視](/docs/ja/monitoring-usage)：完全な OpenTelemetry イベントリファレンス
