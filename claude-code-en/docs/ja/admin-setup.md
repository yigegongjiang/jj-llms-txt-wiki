> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 組織向けに Claude Code をセットアップする

> Claude Code を展開する管理者向けの決定マップ。API プロバイダー、マネージド設定、ポリシー実行、使用状況監視、データ処理をカバーしています。

Claude Code は、ローカル開発者設定よりも優先されるマネージド設定を通じて組織ポリシーを実行します。これらの設定は Claude 管理コンソール、モバイルデバイス管理（MDM）システム、またはディスク上のファイルから配信します。設定は Claude が到達できるツール、コマンド、サーバー、ネットワーク宛先を制御します。

このページでは、展開の決定を順番に説明します。各行は以下のセクションと、その領域の参照ページにリンクしています。

<Note>
  SSO、SCIM プロビジョニング、シート割り当ては Claude アカウントレベルで設定されます。これらの手順については、[Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) と [シート割り当て](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) を参照してください。
</Note>

| 決定                                                        | 選択内容                      | 参照                                                                                                                                                                         |
| :-------------------------------------------------------- | :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [API プロバイダーを選択する](#choose-your-api-provider)              | Claude Code が認証される場所と課金方法 | [Authentication](/docs/ja/authentication)、[Amazon Bedrock](/docs/ja/amazon-bedrock)、[Google Cloud の Agent Platform](/docs/ja/google-vertex-ai)、[Microsoft Foundry](/docs/ja/microsoft-foundry) |
| [設定がデバイスに到達する方法を決定する](#decide-how-settings-reach-devices) | マネージドポリシーが開発者マシンに到達する方法   | [Server-managed settings](/docs/ja/server-managed-settings)、[Delivery mechanisms](/docs/ja/managed-settings#delivery-mechanisms)                                                     |
| [実行する内容を決定する](#decide-what-to-enforce)                    | どのツール、コマンド、統合が許可されるか      | [Permissions](/docs/ja/permissions)、[Sandboxing](/docs/ja/sandboxing)                                                                                                                |
| [使用状況の可視性をセットアップする](#set-up-usage-visibility)             | 支出と採用を追跡する方法              | [Analytics](/docs/ja/analytics)、[Monitoring](/docs/ja/monitoring-usage)、[Costs](/docs/ja/costs)                                                                                           |
| [データ処理を確認する](#review-data-handling)                       | データ保持とコンプライアンス体制          | [Data usage](/docs/ja/data-usage)、[Security](/docs/ja/security)                                                                                                                      |

<h2 id="choose-your-api-provider">
  API プロバイダーを選択する
</h2>

Claude Code は複数の API プロバイダーのいずれかを通じて Claude に接続します。選択は課金、認証、継承するコンプライアンス体制、および開発者が使用できる Claude Code 機能に影響します。

| プロバイダー                        | 選択する場合                                                                                     |
| :---------------------------- | :----------------------------------------------------------------------------------------- |
| Claude for Teams / Enterprise | Claude Code と claude.ai を 1 つのシート単位のサブスクリプションで実行したい場合。実行するインフラストラクチャは不要です。これがデフォルトの推奨事項です。 |
| Claude Console                | API ファーストまたは従量課金を希望する場合                                                                    |
| Amazon Bedrock                | 既存の AWS コンプライアンス制御と課金を継承したい場合                                                              |
| Google Cloud の Agent Platform | 既存の GCP コンプライアンス制御と課金を継承したい場合                                                              |
| Microsoft Foundry             | 既存の Azure コンプライアンス制御と課金を継承したい場合                                                            |

一部の Claude Code 機能には claude.ai アカウントが必要です。[web 上の Claude Code](/docs/ja/claude-code-on-the-web)、[Routines](/docs/ja/routines)、[Code Review](/docs/ja/code-review)、[Remote Control](/docs/ja/remote-control)、および [Chrome 拡張機能](/docs/ja/chrome) は、Console API キーまたはクラウドプロバイダーの認証情報だけでは利用できません。Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry を通じてデプロイする場合は、開発者が Claude for Teams または Enterprise シートも必要かどうかを検討してください。各機能ページにはプラン要件が記載されています。

認証、リージョン、機能パリティをカバーする完全なプロバイダー比較については、[エンタープライズ展開概要](/docs/ja/third-party-integrations) を参照してください。各プロバイダーの認証セットアップは [Authentication](/docs/ja/authentication) にあります。

[ネットワーク設定](/docs/ja/network-config) のプロキシとファイアウォール要件は、プロバイダーに関係なく適用されます。複数のプロバイダーの前に単一のエンドポイントを配置したい場合、または集中化されたリクエストログを記録したい場合は、[LLM gateway](/docs/ja/llm-gateway) を参照してください。

<h2 id="decide-how-settings-reach-devices">
  設定がデバイスに到達する方法を決定する
</h2>

マネージド設定は、組織ポリシーを定義します。Claude Code は以下の表に示す 4 つのソースを優先順位順にチェックします。[Claude Code がマネージドソースを組み合わせる方法](/docs/ja/managed-settings#precedence-within-the-managed-tier)は、どのソースが適用されるか、ポリシーヘルパーが何を変更するか、およびすべてのソースを構成する方法を説明しています。この表は決定マップです。

| メカニズム                   | 配信                                                                                                                                                                                                  | 優先度 | プラットフォーム      |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-- | :------------ |
| Server-managed          | claude.ai 管理コンソール、またはゲートウェイサインイン用の自己ホスト型 [Claude apps gateway](/docs/ja/claude-apps-gateway)                                                                                                             | 最高  | すべて           |
| plist / registry policy | macOS: `com.anthropic.claudecode` plist<br />Windows: `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                           | 高   | macOS、Windows |
| File-based managed      | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux と WSL: `/etc/claude-code/managed-settings.json`<br />Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | 中   | すべて           |
| Windows user registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                 | 最低  | Windows のみ    |

Claude Code はスタートアップ時に server-managed 設定をフェッチし、セッション中は 1 時間ごとに更新します。デプロイするエンドポイントインフラストラクチャはありません。claude.ai 管理コンソール経由の配信には Claude for Teams または Enterprise プランが必要です。Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry でのデプロイメントは、[Claude apps gateway](/docs/ja/claude-apps-gateway) を実行することで同じリモート配信を取得できます。または、ファイルベースまたは OS レベルのメカニズムのいずれかを代わりに使用してください。

組織が複数のプロバイダーを混在させている場合、claude.ai ユーザー向けに [server-managed settings](/docs/ja/server-managed-settings) を設定し、他のユーザーがマネージドポリシーを受け取るように [ファイルベースまたは plist/registry フォールバック](/docs/ja/managed-settings#delivery-mechanisms)を設定してください。

plist と HKLM レジストリの場所は任意のプロバイダーで機能し、書き込みに管理者権限が必要なため、改ざんに強いです。HKCU の Windows ユーザーレジストリは昇格なしで書き込み可能なため、実行チャネルではなく便利なデフォルトとして扱ってください。

デフォルトでは WSL は `/etc/claude-code` の Linux ファイルパスのみを読み取ります。同じマシン上の WSL に Windows レジストリと `C:\Program Files\ClaudeCode` ポリシーを拡張するには、これらの管理者のみが使用できる Windows ソースのいずれかで [`wslInheritsWindowsSettings: true`](/docs/ja/settings-reference#wslinheritswindowssettings) を設定してください。

どのメカニズムを選択しても、マネージド値はユーザーおよびプロジェクト設定よりも優先されます。ただし、セキュリティに関する [例外](/docs/ja/settings#exceptions-to-managed-settings-precedence)がいくつかあります。`permissions.allow` や `permissions.deny` などの配列設定は、すべてのソースからのエントリをマージするため、開発者はマネージドリストを拡張できますが、削除することはできません。`fallbackModel`、`availableModels`、および [`modelPicker`](/docs/ja/settings-reference#modelpicker) の場合、マネージド値は下位レイヤーとマージするのではなく、置き換えます。

<h3 id="wsl-sessions-in-claude-code-desktop">
  Claude Code Desktop の WSL セッション
</h3>

Windows では、[Claude Code Desktop は WSL 2 ディストリビューション内で Code セッションを実行できます](/docs/ja/desktop-wsl)。セッションの Claude Code プロセスはディストリビューション内で実行されるため、上記の WSL 検出パスを通じてマネージド設定を解決します。`wslInheritsWindowsSettings: true` が展開されていない限り、Windows のみのソースはそれに到達しません。

Claude Desktop は、`C:\Program Files\ClaudeCode\managed-settings.json` が存在する場合など、組織がマネージドしているデバイスとして検出されるデバイスでは、デフォルトで WSL セッションをオフにします。それらをオンにするには、Windows レジストリポリシーをデプロイします。これには Claude Desktop v1.19367.0 以降が必要です。

* `HKLM\SOFTWARE\Policies\Claude` の下に `disableWslSessions` という名前の値を作成し、`REG_SZ` 文字列 `false` または `REG_DWORD` `0` に設定します。この値は Claude Desktop ポリシーキーの下にあり、マネージド設定を含む `ClaudeCode` キーとは別です。HKLM の下に値をデプロイします。これには管理者権限が必要です。HKCU の下の値は WSL セッションを有効にしません。
* `C:\Program Files\ClaudeCode\managed-settings.json` をデプロイする場合は、そのままにしておいてください。`disableWslSessions` が HKLM の下で `false` になると、そのファイルが存在していても Desktop は WSL セッションを許可します。

Desktop は WSL セッションが開始されるたびにポリシーを読み取るため、デプロイ後にアプリを再起動する必要はありません。

デバイスが依然として WSL セッションを拒否する場合は、そのデバイスの Claude Desktop で **Help > Troubleshooting > Show Logs in Explorer** を開きます。これにより、ログフォルダのコピーが Downloads に保存されます。そのコピーの `main.log` で `[wslPolicyGate] denying WSL session` を検索してください。拒否の理由は括弧内に続きます。例えば `(cli-file-present)` のようにです。Claude Desktop が `.exe` インストーラーでインストールされた場合、`%APPDATA%\Claude\logs\main.log` のライブファイルを読むこともできます。

WSL セッションが有効になった後、マネージド設定をそれらに拡張します。

* HKLM レジストリまたは `C:\Program Files\ClaudeCode` ファイルを通じて `wslInheritsWindowsSettings: true` をデプロイして、WSL セッションがホストセッションと同じポリシーを継承するようにしてください。
* WSL セッション内で `/status` を実行して検証し、`Setting sources` 行を読んでください。それを解釈する方法については、[/status で出力を読む](/docs/ja/managed-settings#read-the-source-in-/status)を参照してください。

WSL 2 ユーティリティ VM 内のプロセスは、Windows 側のエンドポイント検出センサーに表示されません。ディストリビューション内のプロセスとファイルアクティビティを観察するには、エンドポイント検出ベンダーの WSL ガイダンスで、ディストリビューション内で実行できる Linux センサーと、それが必要とする除外を確認してください。Claude Code の [OpenTelemetry ツール実行テレメトリ](/docs/ja/monitoring-usage)は WSL とネイティブセッションで同じように出力されます。

<h2 id="decide-what-to-enforce">
  実装する内容を決定する
</h2>

マネージド設定は、ツール、サンドボックス実行、MCP サーバーとプラグインソース、および実行するフック を制限できます。各行は、それを駆動する設定キーを持つ制御サーフェスです。

| 制御                                                                   | 機能                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | キー設定                                                                                                                                |
| :------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| [権限ルール](/docs/ja/permissions)                                             | 特定のツールとコマンドを許可、確認、または拒否する                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `permissions.allow`、`permissions.deny`                                                                                              |
| [権限ロックダウン](/docs/ja/permissions#managed-only-settings)                    | マネージド設定を[権限ルールの唯一の設定ソース](/docs/ja/settings-reference#allowmanagedpermissionrulesonly)にします。`--dangerously-skip-permissions` を無効にする                                                                                                                                                                                                                                                                                                                                                                                 | `allowManagedPermissionRulesOnly`、`permissions.disableBypassPermissionsMode`                                                        |
| [開始権限モード](/docs/ja/permission-modes#which-mode-a-session-starts-in)       | 組み込みの開始権限モードの代わりに、開発者のターミナルセッションが開始する権限モードを選択するか、自動モードを削除します。VS Code 拡張機能は、Pro、Max、Team プランでのみ設定した `defaultMode` を読み取ります。[権限モードの切り替え](/docs/ja/permission-modes#switch-permission-modes)は、拡張機能が読み取る内容を一覧表示します                                                                                                                                                                                                                                                                                                     | `permissions.defaultMode`、`permissions.disableAutoMode`                                                                             |
| [サンドボックス](/docs/ja/sandboxing)                                            | ドメイン許可リスト付きの OS レベルのファイルシステムとネットワーク分離                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `sandbox.enabled`、`sandbox.network.allowedDomains`                                                                                  |
| [マネージドポリシー CLAUDE.md](/docs/ja/memory#deploy-organization-wide-claude-md) | すべてのセッションで読み込まれる組織全体の指示。除外できません                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | マネージドポリシーパスのファイル                                                                                                                    |
| [MCP サーバー制御](/docs/ja/managed-mcp)                                        | ユーザーが追加または接続できる MCP サーバーを制限し、固定セットをデプロイするか、リモートサーバーをすべてのユーザーに提供します                                                                                                                                                                                                                                                                                                                                                                                                                                           | `allowedMcpServers`、`deniedMcpServers`、`allowManagedMcpServersOnly`、`managedMcpServers`、またはデプロイされた `managed-mcp.json` ファイル          |
| [プラグインマーケットプレイス制御](/docs/ja/plugins/org#restrict-what-users-can-install)  | ユーザーが追加およびインストールできるマーケットプレイスソースを制限し、単一実行のためにプラグイン、エージェント、MCP サーバーをサイドロードする CLI フラグを拒否し、[`command` プラグインソース](/docs/ja/plugins/marketplace-reference#command-plugin-source)をブロックし、どのマーケットプレイスのプラグインを提案できるかをホワイトリストに登録します                                                                                                                                                                                                                                                                                            | `strictKnownMarketplaces`、`blockedMarketplaces`、`disableSideloadFlags`、`disableCommandPluginSources`、`pluginSuggestionMarketplaces` |
| [カスタマイズロックダウン](/docs/ja/settings-reference#strictpluginonlycustomization) | スキル、エージェント、フック、MCP サーバーをユーザーおよびプロジェクトソースからブロックし、プラグインまたはマネージド設定からのみ取得できるようにします。スキルをロックすると、開発者が claude.ai で有効にした[スキル](/docs/ja/skills#where-synced-skills-load)の同期も停止します                                                                                                                                                                                                                                                                                                                                           | `strictPluginOnlyCustomization`                                                                                                     |
| [claude.ai 同期を無効にする](/docs/ja/settings-reference#syncclaudeaiskills)      | Claude Code が開発者が claude.ai で有効にした[スキル](/docs/ja/skills#how-synced-skills-behave)と[プラグイン](/docs/ja/plugins/loading#synced-plugins)を読み込むのを停止します。組織の claude.ai でスキルをオフにすると、Claude Code は両方の同期を停止します。v2.1.273 以降では、既に同期したものも削除します。スキルをオフにせずにどちらか一方を停止するには、マネージド設定でそのキーを `false` に設定します                                                                                                                                                                                                                                    | `syncClaudeAiSkills`、`syncClaudeAiPlugins`                                                                                          |
| [フック制限](/docs/ja/settings-reference#allowmanagedhooksonly)                | 実行するフックを制限し、HTTP フック URL を制限します。[`allowManagedHooksOnly` で実行される内容](/docs/ja/settings-reference#what-runs-under-allowmanagedhooksonly)の完全な効果リストを参照してください                                                                                                                                                                                                                                                                                                                                                           | `allowManagedHooksOnly`、`allowedHttpHookUrls`                                                                                       |
| [ログイン強制](/docs/ja/settings-reference#forceloginmethod)                    | ログインを特定の方法または Anthropic 組織に制限します。メソッド制限は VS Code 拡張機能、Agent SDK、`claude setup-token`、`/install-github-app` 全体に適用され、ターミナルのインタラクティブログイン画面（`/login` または初回オンボーディングで到達）はメソッドを事前選択しますが強制しません。Claude Code は、ターミナル、VS Code 拡張機能、Agent SDK での claude.ai アカウントログインの組織を検証し、Claude Console ログインまたは[ゲートウェイ](/docs/ja/claude-apps-gateway)サインインではチェックしません。v2.1.212 より前は、ターミナルログインのみが両方のキーを適用していました。設定すると、`ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN`、または `apiKeyHelper` によって認証されたセッションはスタートアップでブロックされます。クラウドプロバイダーセッションは影響を受けません | `forceLoginMethod`、`forceLoginOrgUUID`                                                                                              |
| [エージェントビューを無効にする](/docs/ja/agent-view#how-background-sessions-are-hosted) | `claude agents`、`--bg`、`/background`、およびオンデマンドスーパーバイザーをオフにします                                                                                                                                                                                                                                                                                                                                                                                                                                                | `disableAgentView`                                                                                                                  |
| [企業ランチャーを構成する](/docs/ja/corporate-launcher)                               | [バックグラウンドエージェントスーパーバイザー](/docs/ja/agent-view#how-background-sessions-are-hosted)、そのワーカー、および[その他のカバーされたバックグラウンドプロセス](/docs/ja/corporate-launcher#what-the-launcher-covers)に、エージェントビューをオフにする代わりに、必須の企業ランチャーをプレフィックスします                                                                                                                                                                                                                                                                                                   | `processWrapper`                                                                                                                    |
| [モデル制限](/docs/ja/model-config#restrict-model-selection)                   | `availableModels` はピッカーに表示されるモデルをフィルタリングします。`enforceAvailableModels` を追加すると、自動選択されたデフォルトモデルも制限されます。このセッティングが CLI、ウェブ、IDE にどのように到達するかについては、[サーフェスカバレッジ](/docs/ja/model-config#surface-coverage)を参照してください                                                                                                                                                                                                                                                                                                           | `availableModels`、`enforceAvailableModels`                                                                                          |
| [エフォートキャップ](/docs/ja/settings-reference#maxeffortlevel)                   | すべてのモデルまたはモデルごとに、すべてのプロバイダーで[エフォートレベル](/docs/ja/model-config#adjust-effort-level)をキャップします                                                                                                                                                                                                                                                                                                                                                                                                                         | `maxEffortLevel`                                                                                                                    |
| [バージョンフロア](/docs/ja/settings-reference#minimumversion)                    | 自動更新が組織全体の最小値以下をインストールするのを防ぎます                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `minimumVersion`                                                                                                                    |
| [必須バージョン範囲](/docs/ja/settings-reference#requiredminimumversion)           | 実行中のバージョンが組織承認範囲外の場合、まったく起動を拒否します。ダウングレードのみをブロックする `minimumVersion` より強力です                                                                                                                                                                                                                                                                                                                                                                                                                                   | `requiredMinimumVersion`、`requiredMaximumVersion`                                                                                   |
| [テレメトリオプトアウト](/docs/ja/data-usage#telemetry-services)                     | すべてのデバイスで Anthropic バウンドの使用メトリクス、エラーレポート、およびサーベイをオフにします                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `env` に `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` を `1` に設定します。リンクされたセクションはカテゴリごとの変数を一覧表示します                                       |

メンバーが claude.ai または Anthropic API を通じてサインインし、Claude Enterprise プランを使用している場合、何もデプロイせずに組織の管理者設定からモデルを管理することもできます。

* [組織モデル制限](/docs/ja/model-config#organization-model-restrictions)：個別のモデルを無効にします。サーバー側で強制されます。
* [組織デフォルトモデル](/docs/ja/model-config#organization-default-model)：新しいセッションが開始するモデルを設定します。ユーザーは、組織がデフォルトを強制しない限り変更できます。これは限定的な組織セットで利用可能です。Anthropic アカウントチームにお問い合わせください。
* [組織エフォート制限](/docs/ja/model-config#organization-effort-limits)：ロールごとのエフォートレベルをキャップします。サーバー側で強制されます。

これらの制御は、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、または[AWS 上の Claude Platform](/docs/ja/claude-platform-on-aws)のセッションには到達しません。これらのプロバイダーでは、代わりにマネージド設定を使用してください。制限には `availableModels`、デフォルトには `model`、エフォートキャップには [`maxEffortLevel`](/docs/ja/settings-reference#maxeffortlevel) を使用します。

[Web 上の Claude Code](/docs/ja/claude-code-on-the-web)には独自の管理サーフェスがあります。管理設定のクラウド環境ページで、オーナーは[組織共有環境](/docs/ja/cloud-environments#organization-shared-environments)を作成し、メンバーのクラウドセッションの[ネットワークアクセスレベル](/docs/ja/cloud-environments#network-access)、環境変数、セットアップスクリプトを設定します。オーナーは、[claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) で組織のデフォルト環境を別途選択します。

権限ルールとサンドボックスは異なるレイヤーをカバーします。WebFetch を拒否すると Claude のフェッチツールがブロックされますが、Bash が許可されている場合、`curl` と `wget` は依然として任意の URL に到達できます。サンドボックスは、OS レベルで強制されるネットワークドメイン許可リストでそのギャップを閉じます。

これらの制御が防御する脅威モデルについては、[セキュリティ](/docs/ja/security)を参照してください。

<h2 id="set-up-usage-visibility">
  使用状況の可視性をセットアップする
</h2>

必要なレポート内容に基づいて監視を選択してください。ダッシュボード、API、支出管理は Claude for Teams または Enterprise プランと Claude Console 組織で異なるため、機能に基づいてレポートを計画する前に「利用可能性」列を確認してください。

| 機能                     | 取得内容                                                                     | 利用可能性                                                                                                                                                                                                                     | 開始場所                                                  |
| :--------------------- | :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------- |
| Usage monitoring       | セッション、ツール、トークンの OpenTelemetry エクスポート                                     | すべてのプロバイダー                                                                                                                                                                                                                | [Monitoring usage](/docs/ja/monitoring-usage)              |
| Analytics dashboard    | Teams / Enterprise でのリーダーボード付き採用度と貢献度メトリクス、Console でのユーザーごとの使用状況と支出メトリクス | Teams / Enterprise は [claude.ai/analytics](https://claude.ai/analytics/claude-code)、Console は [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                  | [Analytics](/docs/ja/analytics)                            |
| Programmatic reporting | API を通じたユーザーごとの使用状況とコストデータ                                               | Enterprise 向け [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics)、Console 向け [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) | [Costs](/docs/ja/costs#manage-costs-for-your-organization) |
| Spend controls         | 支出制限とレート制限                                                               | Teams / Enterprise の管理者設定、Console のワークスペース制限、サードパーティクラウドではクラウド予算管理またはユーザーごとの [支出制限](/docs/ja/claude-apps-gateway-spend-limits) を備えた [Claude apps gateway](/docs/ja/claude-apps-gateway)                                             | [Costs](/docs/ja/costs#manage-costs-for-your-organization) |

Teams および Enterprise では、ユーザーごとの使用状況と支出の数値は分析ダッシュボードではなく、組織の分析設定の [支出レポート](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) から取得されます。クラウドプロバイダーは AWS Cost Explorer、GCP Billing、または Azure Cost Management を通じて支出を公開します。Claude チャット、Claude Code、Cowork 全体にわたるエンタープライズ予算計画については、[Claude Enterprise 消費ガイド](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) を参照してください。

<h2 id="review-data-handling">
  データ処理を確認する
</h2>

Team、Enterprise、Claude API、およびクラウドプロバイダープランでは、Anthropic はコードまたはプロンプトでモデルをトレーニングしません。API プロバイダーが保持とコンプライアンス体制を決定します。

| トピック                     | 知っておくべきこと                                      | 開始場所                                           |
| :----------------------- | :--------------------------------------------- | :--------------------------------------------- |
| Data usage policy        | Anthropic が収集する内容、保持期間、トレーニングに使用されない内容         | [Data usage](/docs/ja/data-usage)                   |
| Zero Data Retention（ZDR） | リクエスト完了後は何も保存されません。Claude for Enterprise で利用可能 | [Zero data retention](/docs/ja/zero-data-retention) |
| Security architecture    | ネットワークモデル、暗号化、認証、監査証跡                          | [Security](/docs/ja/security)                       |

リクエストレベルの監査ログが必要な場合、またはデータの機密性によってトラフィックをルーティングしたい場合は、開発者とプロバイダーの間にゲートウェイを配置してください。自ホスト型の [Claude apps gateway](/docs/ja/claude-apps-gateway) は IdP ID を使用してリクエストごとの監査ログを記録するか、別の [LLM gateway](/docs/ja/llm-gateway) を使用してください。規制要件と認定については、[Legal and compliance](/docs/ja/legal-and-compliance) を参照してください。

<h2 id="verify-and-onboard">
  検証とオンボード
</h2>

マネージド設定を設定した後、開発者に Claude Code 内で `/status` を実行させてください。**Status** タブの `Setting sources` 行に `Enterprise managed settings` が表示され、その後に括弧内のソースが続きます。[アクティブな設定を検証](/docs/ja/managed-settings#verify-enforcement) にラベルが記載されています。

開発者が開始するのに役立つこれらのリソースを共有してください。

* [クイックスタート](/docs/ja/quickstart): インストールからプロジェクトの操作まで、最初のセッションのウォークスルー
* [一般的なワークフロー](/docs/ja/common-workflows): コードレビュー、リファクタリング、デバッグなどの日常的なタスクのパターン
* [Claude Code 101](https://academy.claude.com/courses/claude-code-101) と [Claude Code in Action](https://academy.claude.com/courses/claude-code-in-action): [Claude Academy](https://academy.claude.com/) の無料の自習型コース

ログインの問題については、開発者に [認証のトラブルシューティング](/docs/ja/troubleshoot-install#login-and-authentication) を指してください。最も一般的な修正は次のとおりです。

* `/logout` を実行してから `/login` を実行してアカウントを切り替える
* エンタープライズ認証オプションが見つからない場合は `claude update` を実行する
* 更新後にターミナルを再起動する

開発者が「You haven't been added to your organization yet」というメッセージを見た場合、そのシートには Claude Code アクセスが含まれておらず、管理コンソールで更新する必要があります。

<h2 id="next-steps">
  次のステップ
</h2>

プロバイダーと配信メカニズムを選択したら、詳細な設定に進みます。

* [Server-managed settings](/docs/ja/server-managed-settings): Claude 管理コンソールからマネージドポリシーを配信する
* [All settings](/docs/ja/settings-reference): すべての設定キー、ファイルの場所、および例
* [Which value Claude Code uses](/docs/ja/settings#which-value-claude-code-uses): マネージド、プロジェクト、ローカル、およびユーザー設定全体の優先度ルール
* [Monorepos and large repos](/docs/ja/large-codebases): 大規模リポジトリをデプロイする組織向けのディレクトリごとの設定パターン
* [Amazon Bedrock](/docs/ja/amazon-bedrock)、[Google Cloud's Agent Platform](/docs/ja/google-vertex-ai)、[Microsoft Foundry](/docs/ja/microsoft-foundry): プロバイダー固有のデプロイメント
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO、SCIM、シート管理、ロールアウトプレイブック
