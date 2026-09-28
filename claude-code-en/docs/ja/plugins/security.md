> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインのセキュリティと信頼

> プラグインをインストールする前に信頼できるかどうかを判断します。マシン上でプラグインが何ができるか、プラグインをレビューする方法、削除する方法について説明します。

インストールしたClaudeコードプラグインは、ユーザー権限でマシン上で任意のコードを実行できます。

プラグインはマーケットプレイスからインストールします。マーケットプレイスはClaudeコードが取得するカタログです。一部のマーケットプレイス名は[Anthropicの独自マーケットプレイス用に予約されており](#marketplace-tiers)、その他のマーケットプレイスはすべてサードパーティです。マーケットプレイスの名前はカタログを公開する者を示しており、その中の各プラグインが何をするかは示していないため、[どのマーケットプレイスから来たプラグインでも、インストール前にレビューしてください](#review-a-plugin-before-you-install)。

プラグインをインストールするかどうかを判断している場合、またはチームが使用する前にツールをレビューしている場合は、このページをお読みください。

<Note>
  これらのケースは他のページで説明されています：

  * **Claudeコード独自のセキュリティモデル**：[セキュリティ](/docs/ja/security)を参照してください
  * **組織のプラグインの制限または要求**：[組織のプラグインを管理する](/docs/ja/plugins/org)を参照してください
  * **`security-guidance`または`claude-security`プラグイン**：このページはそれらについてではありません。[`security-guidance`](/docs/ja/security-guidance)と[`claude-security`](/docs/ja/claude-security)を参照してください
</Note>

[プラグインが何ができるか](#understand-what-a-plugin-can-do)と[どのマーケットプレイスがAnthropicのものか](#marketplace-tiers)から始めて、その後[インストール前にプラグインをレビューしてください](#review-a-plugin-before-you-install)。

<h2 id="understand-what-a-plugin-can-do">
  プラグインが何ができるかを理解する
</h2>

プラグインはユーザー権限でマシン上でコードを実行するコンテンツと、Claudeのコンテキストに指示として入るコンテンツを含むことができるため、[インストール前にプラグインをレビューしてください](#review-a-plugin-before-you-install)。インストール済みプラグインが実行できることは以下の通りです：

* **Hooks**：プラグインの[hooks](/docs/ja/hooks)はClaudeコードのライフサイクルの特定の時点（ツール呼び出しの前後など）でシェルコマンドとして実行されます。
* **MCPおよびLSPサーバー**：Claudeコードは有効なプラグインが宣言する[MCPサーバー](/docs/ja/mcp)に接続し、Claudeにそれらのツールを提供します。stdio MCPサーバーはClaudeコードがマシン上で開始するプロセスとして実行されます。Claudeコードはプラグインが宣言する言語サーバーも開始します。
* **`bin/`ディレクトリ**：Claudeコードは有効な各プラグインの`bin/`ディレクトリをBashツールのシェルの`PATH`に追加するため、Claudeのバッシュコマンドはそこの任意の実行可能ファイルを実行できます。
* **Skills、commands、およびagents**：これらはClaudeのコンテキストに指示として入るため、Claudeが既に持っているツールで何をするかに影響します。
* **更新**：プラグインをインストールしたマーケットプレイスで自動更新がオンの場合、Claudeコードはそのプラグインをバックグラウンドで更新するため、レビューしたファイルはディスク上で変更される可能性があります。[自動更新が実行される時期](/docs/ja/plugins/loading#when-auto-update-runs)にはタイミングが記載されています。マーケットプレイスごとに自動更新をオンまたはオフにするには、[プラグインを最新に保つ](/docs/ja/plugins/install#keep-plugins-updated)を参照してください。

Claudeコードの[権限ルール](/docs/ja/permissions)と[サンドボックス](/docs/ja/sandboxing)はClaudeが行うツール呼び出しをカバーしており、プラグイン自体が実行するコードはカバーしていません：

* **Hooksおよびサーバープロセス**：コマンドhooksはフルユーザー権限でシェルコマンドを実行します。ClaudeコードはhooksとMCPサーバーをサンドボックスの外で実行します。
* **Claudeのツール呼び出し**：プラグインのMCPツールへの呼び出し、およびプラグインの`bin/`から実行可能ファイルを実行するBashコマンドはツール呼び出しであるため、権限ルールが適用されます。

プラグインをインストールするとそれも有効になります。ただし、そのマニフェストまたはマーケットプレイスエントリが[`defaultEnabled: false`](/docs/ja/plugins/install#choose-an-install-scope)を設定しており、自分で有効にしていない場合を除きます。

信頼できなくなったプラグインを削除するには、[信頼できなくなったプラグインを削除する](#remove-a-plugin-you-no-longer-trust)を参照してください。

<h2 id="marketplace-tiers">
  Anthropicのマーケットプレイスを名前で識別する
</h2>

マーケットプレイスの名前は、公式、コミュニティ、またはサードパーティの3つのティアのいずれかに分類されます。Claudeコードは`github.com/anthropics/`リポジトリから取得されたマーケットプレイスに対してのみ公式およびコミュニティ名を受け入れるため、サードパーティマーケットプレイスはAnthropicのものとして提示することはできません。同僚または組織が公開するマーケットプレイスはサードパーティです。

表は各ティアに該当する名前を示しています：

| ティア     | マーケットプレイス                                                                |
| :------ | :----------------------------------------------------------------------- |
| 公式      | [公式マーケットプレイス名](#official-marketplace-names)（`claude-plugins-official`など） |
| コミュニティ  | `claude-community`、`claude-plugins-community`、および`healthcare`            |
| サードパーティ | その他すべてのマーケットプレイス                                                         |

`claude-community`カタログがプラグインをコミットSHAにピンしている場合（ほぼすべてのエントリについて行われます）、Claudeコードは異なるコミットのインストールを拒否します。

<h3 id="official-marketplace-names">
  公式マーケットプレイス名
</h3>

これらのマーケットプレイス名は公式ティアを構成しています：

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

公式、コミュニティ、およびデモマーケットプレイスの違いと、各マーケットプレイスが何をリストしているかを参照する場所については、[Anthropicのマーケットプレイス](/docs/ja/plugins/anthropic-marketplaces)を参照してください。

<h2 id="review-a-plugin-before-you-install">
  インストール前にプラグインをレビューする
</h2>

プラグインをインストールする前に、それが何を追加し、どこから来ているかを確認してください。

<Steps>
  <Step title="マーケットプレイスのソースを確認する">
    シェルで`claude plugin marketplace list`を実行して、GitHubリポジトリやディレクトリなど、各マーケットプレイスが追加されたソースを出力します。
  </Step>

  <Step title="詳細ペインを読む">
    Claudeコードセッションで`/plugin`を実行し、プラグインを選択します。詳細ペインは**Will install**セクションを表示し、プラグインのコマンド、agents、skills、hooks、およびMCPおよびLSPサーバーをリストします。Anthropicが公開されたコンポーネントデータを持たないプラグインの場合、セクションはマーケットプレイスエントリが宣言する内容、またはメモを表示します：マーケットプレイス内に保存されているプラグインの場合は`Components will be discovered at installation`、他の場所から取得されたプラグインの場合は`Component summary not available for remote plugin`。
  </Step>

  <Step title="プラグインのソースを読む">
    詳細ペインで、インストールオプションの下の**Open homepage**または**View on GitHub**を選択します。ペインがどちらも提供しない場合は、最初のステップで見つけたマーケットプレイスリポジトリを開きます。そこでプラグインのディレクトリを見つけます。**Will install**セクションはhookが存在することを示していますが、それが何を実行するかは示していないため、プラグインのディレクトリでこれらのファイルを読んでください：

    * **`hooks/hooks.json`**：各hookが実行するコマンド
    * **`.mcp.json`**：各サーバーのコマンドまたはURL
    * **`bin/`**：ディレクトリ内のすべてのファイル
  </Step>

  <Step title="プラグインに含まれるものをリストする">
    プラグインのディレクトリを保持するリポジトリをクローンしてから、シェルで`claude --plugin-dir <plugin directory> plugin details <plugin name>`を実行して、Claudeコードがそこで見つけるものを確認します。コマンドはセッションを開始せずにプラグインのファイルを読み、プラグインのskillsおよびcommands、agents、各hookのイベント付きhooks、およびMCPおよびLSPサーバーをリストする`Component inventory`を出力します。
  </Step>
</Steps>

プラグインをインストールした後、シェルで`claude plugin details <plugin name>`を実行して、`~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`の下にあるインストール済みコピーの同じ`Component inventory`を出力します。

<h3 id="remove-a-plugin-you-no-longer-trust">
  信頼できなくなったプラグインを削除する
</h3>

シェルで、インストールした`--scope`を指定して[`claude plugin uninstall <plugin>`](/docs/ja/plugins/cli-reference#plugin-uninstall)を実行します。その後、アンインストールが削除したものと残したものを確認します：

* **永続データ**：それがプラグインがインストールされた最後のスコープの場合、アンインストールは`--keep-data`を渡さない限り、プラグインの永続データディレクトリも削除します。
* **キャッシュされたファイル**：プラグインのファイルは`~/.claude/plugins/cache/`の下のディスク上に留まり、[バックグラウンドスイープが削除する](/docs/ja/plugins/loading#cleanup-of-previous-versions)前に14日間保持されます。最後のプラグインをアンインストールした後、孤立したディレクトリは別のプラグインをインストールするまで残ります。ファイルを今すぐ削除するには、`~/.claude/plugins/cache/<marketplace>/<plugin>/`の下のプラグインのディレクトリを自分で削除してください。
* **マーケットプレイス**：マーケットプレイスの所有者も信頼しない場合は、[マーケットプレイスも削除してください](/docs/ja/plugins/install#manage-marketplaces)。これにより、そこからインストールしたすべてのプラグインがアンインストールされます。

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Claude Code が拒否または警告する場合を認識する
</h2>

`/plugin` の **Discover** または **Marketplaces** タブから開く詳細ペインには、各プラグインに対して同じ信頼警告が表示されます。Claude Code は、[信頼できないマーケットプレイスソースと整合性チェック失敗](#untrusted-marketplace-sources-and-failed-integrity-checks)に該当するような場合には、警告ではなく拒否します。

<h3 id="trust-warning-before-you-install">
  インストール前の信頼警告
</h3>

警告はプラグインがどのマーケットプレイスから来たかに関わらず、同じ内容です：

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

組織が[管理設定](/docs/ja/plugins/org)で `pluginTrustMessage` を設定している場合、Claude Code はその文字列を警告に追加します。

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  信頼できないマーケットプレイスソースと整合性チェック失敗
</h3>

Claude Code は以下の場合にマーケットプレイスのロードまたはプラグインのインストールを拒否し、各ケースで独自のエラーメッセージが表示されます：

* **信頼できないマーケットプレイスソース**：マーケットプレイスが公式またはコミュニティ名を使用していても、そのソースが `github.com/anthropics/` の外にある場合、Claude Code はマーケットプレイスのロードを停止し、そこからインストールしたプラグインもロードしません。エラーは[Marketplace is registered from an untrusted source](/docs/ja/errors#marketplace-is-registered-from-an-untrusted-source)です。
* **アーカイブ整合性**：マーケットプレイスエントリが [`archive` ソース](/docs/ja/plugins/marketplace-reference#archive-plugin-source)を `sha256` ダイジェストにピンしており、ダウンロードされたファイルのダイジェストが一致しない場合、Claude Code はインストールを拒否します。エラーは[Plugin archive integrity check failed](/docs/ja/errors#plugin-archive-integrity-check-failed)です。

`sha256` ピンはコミュニティカタログのコミット SHA ピンとは別であり、コミット SHA ピンはチェックアウトする git コミットを選択します。

<h2 id="enforce-plugin-controls-for-your-organization">
  組織のプラグイン制御を実施する
</h2>

[管理設定](/docs/ja/plugins/org)を使用して、管理者はこれらのプラグイン制御を実施できます：

* マーケットプレイスソースのホワイトリストまたはブラックリスト
* プラグインを強制的に有効にする
* `--plugin-dir`および`--plugin-url`フラグと`CLAUDE_CODE_PLUGIN_DIRS`変数をオフにする
* hooksを管理設定および強制的に有効なプラグインからのものに制限する
* メンバーのclaude.aiアカウントからのプラグインがClaudeコードでロードされるのを停止する（[`syncClaudeAiPlugins`](/docs/ja/plugins/org#control-matrix)を使用）

[制御マトリックス](/docs/ja/plugins/org#control-matrix)は各キーが何をするか、何をカバーしていないかを示しています。

<h2 id="find-plugins-in-telemetry">
  テレメトリでプラグインを見つける
</h2>

組織がClaudeコードの[OpenTelemetryイベント](/docs/ja/monitoring-usage)を独自のバックエンドにエクスポートしている場合、[マーケットプレイスティア](#marketplace-tiers)はどのプラグイン名がそこに表示されるかを決定します：

* **[プラグインロードイベント](/docs/ja/monitoring-usage#plugin-loaded-event)**：イベントは公式ティアのプラグインおよびマーケットプレイス名をそのまま報告します。コミュニティおよびサードパーティティアの場合、`OTEL_LOG_TOOL_DETAILS=1`を設定しない限り、`plugin.name`および`marketplace.name`はリテラル文字列`third-party`です。
* **プラグインスコープ**：ロードされたイベントの`plugin.scope`は、管理設定が有効にするプラグインの`org`や、その他のサードパーティプラグインの`user-local`など、プラグインがどこから来たかを報告します。[プラグインロードイベント](/docs/ja/monitoring-usage#plugin-loaded-event)はすべての値をリストしています。
* **[プラグインインストールイベント](/docs/ja/monitoring-usage#plugin-installed-event)**：`OTEL_LOG_TOOL_DETAILS=1`を設定しない限り、イベントは非公式プラグインの名前フィールドを省略し、`third-party`を報告する代わりに省略します。
* **[Claude Code Analytics API](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**：Claudeコードは公式およびコミュニティティアのプラグインを名前で報告し、その他のすべてのプラグインを`third-party`として報告します。

<h2 id="next-steps">
  次のステップ
</h2>

* [組織のプラグインを管理する](/docs/ja/plugins/org)：ユーザーがインストールできるマーケットプレイスを制限し、信頼できるものを要求する
* [プラグインをインストールして管理する](/docs/ja/plugins/install)：スコープを選択する前に、プラグインの詳細ペインをレビューする
* [Anthropicのマーケットプレイス](/docs/ja/plugins/anthropic-marketplaces)：どのマーケットプレイス名がAnthropicのものか
* [セキュリティ](/docs/ja/security)：Claudeコード独自のセキュリティモデル
