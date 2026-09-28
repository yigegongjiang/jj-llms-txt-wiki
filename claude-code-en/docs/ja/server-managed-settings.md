> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# サーバー管理設定を構成する

> デバイス管理インフラストラクチャを必要とせずに、サーバー配信設定を通じて組織全体で Claude Code を一元的に構成します。

サーバー管理設定により、組織の所有者は claude.ai コンソールの [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) から Claude Code を一元的に構成できます。Claude Code クライアントは、ユーザーが対象となる認証情報を使用して、サーバー管理配信がサポートされているプラットフォームで認証すると、これらの設定を自動的に取得します。対象となる認証情報とプラットフォームについては、[プラットフォームの可用性](#platform-availability)を参照してください。

<Note>
  サーバー管理設定は [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) および [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise) カスタマー向けに利用可能です。
</Note>

<h2 id="requirements">
  要件
</h2>

サーバー管理設定を使用するには、以下が必要です。

* Claude for Teams または Claude for Enterprise プラン
* Claude 組織の Owner または Primary Owner ロール（設定を表示および編集するため）
* `api.anthropic.com` へのネットワークアクセス

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  サーバー管理設定とエンドポイント管理設定の選択
</h2>

Claude Code は、一元的な構成のための 2 つのアプローチをサポートしています。サーバー管理設定は Anthropic のサーバーから構成を配信します。[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms)は、ネイティブ OS ポリシー（macOS 管理設定、Windows レジストリ）または管理設定ファイルを通じてデバイスに直接配置されます。

| アプローチ                                                       | 最適な用途                           | セキュリティモデル                                                    |
| :---------------------------------------------------------- | :------------------------------ | :----------------------------------------------------------- |
| **サーバー管理設定**                                                | MDM がない組織、または管理されていないデバイス上のユーザー | Claude Code が起動時に Anthropic のサーバーから取得し、セッション中に 1 時間ごとに更新する設定 |
| **[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms)** | MDM またはエンドポイント管理がある組織           | MDM 構成プロファイル、レジストリポリシー、または管理設定ファイルを通じてデバイスに配置される設定           |

デバイスが MDM またはエンドポイント管理ソリューションに登録されている場合、エンドポイント管理設定はより強力なセキュリティ保証を提供します。これは、設定ファイルが OS レベルでユーザーの変更から保護される可能性があるためです。エンドポイント管理設定は Anthropic がホストする環境の[クラウドセッション](/docs/ja/model-config#surface-coverage)に到達しないため、開発者がクラウドセッションを実行する組織はサーバー管理設定も構成する必要があります。[自己ホスト環境](/docs/ja/self-hosted-environments)内のセッションも、ランナーイメージ内の管理設定ファイルを読み取ります。以下の[設定の優先順位](#settings-precedence)は、そのファイルが適用される場合を示しています。

<h2 id="configure-server-managed-settings">
  サーバー管理設定を構成する
</h2>

<Steps>
  <Step title="管理コンソールを開く">
    claude.ai コンソールで、[**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) に移動します。

    リンクが Claude Code ページではなく別の Admin Settings ページにリダイレクトされる場合、アカウントに必要なロールがありません。Admin およびその他の Owner 以外のロールは管理設定を表示または編集できないため、組織内の Owner または Primary Owner に変更を依頼してください。[アクセス制御](#access-control)を参照してください。
  </Step>

  <Step title="設定を定義する">
    構成を JSON として追加します。`settings.json` で利用可能な[すべての設定](/docs/ja/settings-reference#all-settings)がサポートされており、OS レベルのポリシー配信に制限されているものを除きます。[現在の制限事項](#current-limitations)でその短いリストを参照してください。これには[hooks](/docs/ja/hooks)、[環境変数](/docs/ja/env-vars)、および `allowManagedPermissionRulesOnly` などの[管理専用設定](/docs/ja/managed-settings#managed-only-settings)が含まれます。

    この例は、権限拒否リストを適用し、ユーザーが権限をバイパスするのを防ぎ、権限ルールを管理設定で定義されたものに制限します。`Bash(curl *)` ルールは、`/usr/bin/curl` や `sh -c 'curl …'` ではなく、[Claude が記述する方法](/docs/ja/permissions#bash-rule-limits)として `curl` にマッチします。コマンドテキストに依存しないネットワーク強制の場合は、[`sandbox` ブロックに `allowManagedDomainsOnly`](/docs/ja/sandboxing#configure-the-sandbox-for-your-organization) を追加してください。

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Hooks は `settings.json` と同じ形式を使用します。

    この例は、組織全体のすべてのファイル編集後に監査スクリプトを実行します。

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    hooks はシェルコマンドを実行するため、インタラクティブセッション内のユーザーは Claude Code がそれらを適用する前に[セキュリティ承認ダイアログ](#security-approval-dialogs)を表示します。

    [auto mode](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) 分類器を構成して、組織が信頼するリポジトリ、バケット、ドメインを認識させるには、同じ方法で `autoMode` ブロックを配信してください。`autoMode` エントリが分類器がブロックする内容にどのように影響するか、および `environment`、`allow`、`soft_deny`、および `hard_deny` フィールドに関する重要な警告については、[auto mode を構成する](/docs/ja/auto-mode-config)を参照してください。
  </Step>

  <Step title="保存してデプロイする">
    変更を保存します。Claude Code クライアントは、次回の起動時または 1 時間ごとのポーリングサイクルで更新された設定を受け取ります。
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  設定配信の確認
</h3>

設定が適用されていることを確認するには、ユーザーに Claude Code を再起動するよう依頼します。構成に[セキュリティ承認ダイアログ](#security-approval-dialogs)をトリガーする設定が含まれている場合、ユーザーは Claude Code がそれらを取得する次回時（次回の起動時、またはインタラクティブセッション実行中は 1 時間以内）に管理設定を説明するプロンプトを表示します。また、ユーザーに `/permissions` を実行して有効な権限ルールを表示させることで、管理権限ルールがアクティブであることを確認することもできます。

特定のマシンでフェッチ結果を確認するには、ユーザーに `claude doctor` を実行させ、`Managed settings (remote)` 行を読んでください。Claude Code v2.1.248 以降が必要です。この行は 4 つの結果のいずれかを報告します。

* 配信された設定が読み込まれた
* 組織にサーバー管理設定が構成されていない
* フェッチが失敗し、原因と キャッシュされたポリシーがまだ適用されているかどうかを表示
* Claude Code がフェッチをスキップし、理由を表示。[プラットフォーム可用性](#platform-availability)でスキップするプロバイダーと構成を参照してください

フェッチがまだ進行中の場合、行はそれを報告します。

実行中のセッションでは、`/status` はフェッチ失敗後に同じ行を表示し、サードパーティプロバイダー変数やユーザーのシェルでエクスポートされたカスタム `ANTHROPIC_BASE_URL` など、スキップされたフェッチの原因によっては表示されます。

<h3 id="access-control">
  アクセス制御
</h3>

以下のロールがサーバー管理設定を管理できます。

* **Primary Owner**
* **Owner**

設定の変更は組織内のすべてのユーザーに適用されるため、信頼できる担当者へのアクセスを制限してください。

<h3 id="managed-only-settings">
  管理専用設定
</h3>

ほとんどの[設定キー](/docs/ja/settings-reference#all-settings)は任意のスコープで機能します。いくつかのキーは管理設定からのみ読み込まれ、ユーザーまたはプロジェクト設定ファイルに配置された場合は効果がありません。権限およびプラグイン制御については[管理専用設定](/docs/ja/managed-settings#managed-only-settings)を参照するか、完全なセットについては[すべての設定](/docs/ja/settings-reference#all-settings)インデックスの Scope 列を読んでください。

<h3 id="current-limitations">
  現在の制限事項
</h3>

サーバー管理設定には、以下の制限があります。

* 設定は組織内のすべてのユーザーに均一に適用されます。グループごとの構成はまだサポートされていません。
* [`managed-mcp.json`](/docs/ja/managed-mcp) ファイルはサーバー管理設定を通じて配布することはできません。代わりに `allowedMcpServers` および `deniedMcpServers` ポリシーキーをそこに配信してください。Claude Code v2.1.259 以降では、[`managedMcpServers`](/docs/ja/managed-mcp#provide-servers-through-managed-settings) でリモートサーバーを提供することもできます。これは `http` および `sse` サーバーのみを受け入れ、ファイルが行う方法で排他的制御を行いません。

  Claude Code は、その[システムパス](/docs/ja/managed-mcp#exclusive-control-with-managed-mcp-json)にデプロイされた `managed-mcp.json` を管理設定層とは別に読み込むため、サーバー管理設定が有効な場合でもファイルが適用されます。
* `policyHelper` および `wslInheritsWindowsSettings` など、OS レベルのポリシーソースに制限されている設定は、尊重されません。代わりに MDM またはシステム `managed-settings.json` ファイルを通じてデプロイしてください。その方法でデプロイされた `policyHelper` は、その送信元が[管理層内の優先順位](/docs/ja/managed-settings#precedence-within-the-managed-tier)の下で選択されたものである場合にのみ実行されます。

<h2 id="settings-delivery">
  設定配信
</h2>

<h3 id="settings-precedence">
  設定の優先順位
</h3>

サーバー管理設定と[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms)は、Claude Code [設定階層](/docs/ja/settings#settings-precedence)の最上位を占めます。コマンドライン引数を含む他の設定レベルはこれらをオーバーライドできません。ただし、[管理設定の優先順位の例外](/docs/ja/settings#exceptions-to-managed-settings-precedence)は除きます。

管理層内では、Claude Code はデフォルトで、少なくとも 1 つのポリシーキーを配信する最初のソースを使用します。サーバー管理設定が最初にチェックされ、次にエンドポイント管理設定がチェックされます。ただし、[次に説明するキー単位の例外](#per-key-exceptions-across-managed-sources)は除きます。[Claude Code が管理ソースを組み合わせる方法](/docs/ja/managed-settings#precedence-within-the-managed-tier)には、完全なランキング、制御キーの除外、およびすべてのソースに適用されるオプトインが記載されています。

選択されたソースが MDM ポリシーまたは管理設定ファイルであり、その [`policyHelper`](/docs/ja/settings-reference#policyhelper) が管理設定を提供する場合、ヘルパーの出力はそのソースを置き換え、実行のための唯一の管理構成になります。Claude Code は、サーバー管理設定がポリシーキーを配信している間、MDM またはファイルベースの設定で構成された `policyHelper` を参照しません。

後続のフェッチでサーバー管理設定が削除されたことが判明した場合、Claude Code は次の起動時ではなく、すぐにそのヘルパーを実行します。[`policyHelper`](/docs/ja/settings-reference#policyhelper) エントリは、その実行が失敗した場合に何が起こるかをカバーしています。

管理コンソールでサーバー管理構成をクリアして、エンドポイント管理 plist またはレジストリポリシーにフォールバックする意図がある場合、[キャッシュされた設定](#fetch-and-caching-behavior)はクライアントマシンに保持され、次の成功したフェッチまで続きます。また、[次の起動時にのみ適用される](#fetch-and-caching-behavior)キー（`model` など）は、各クライアントが再起動するまで有効なままです。`/status` を実行して、どの管理ソースがアクティブであるかを確認してください。

<h3 id="per-key-exceptions-across-managed-sources">
  管理ソース全体でのキー単位の例外
</h3>

3 つの種類のキーがマージなしルールの例外です。

* **クロスソースロックキー**：サンドボックスホワイトリストロックなど、[管理設定ページに記載されている](/docs/ja/managed-settings#precedence-within-the-managed-tier)小さなキーセット。Claude Code は、管理者が管理する管理ソースがそれらを設定する場合にそれらを尊重します。ユーザーが書き込み可能な HKCU レジストリ層は除外されます。

  [`policyHelper`](/docs/ja/settings-reference#policyhelper) が管理設定を提供する場合、その出力はこれらのチェックが読み取る唯一のソースです。ただし、[`forceRemoteSettingsRefresh`](/docs/ja/settings-reference#forceremotesettingsrefresh) は除きます。これは Claude Code が起動時に管理ソースから直接読み取ります。
* **`env` ブロック**：テレメトリユニットと認証情報キーとペアになったルーティング変数を除き、以下で説明するように、管理者が管理するソース全体でキーごとにマージされます。各環境変数について、それを定義する最優先ソースが優先され、下位の管理ソースは上位のソースが設定しない変数を埋めます。したがって、エンドポイント管理 `env` エントリは、サーバー管理構成がその変数を設定しない場合、またはキャッシュされたサーバー値が[サーバー確認待ちで保留中](#fetch-and-caching-behavior)の場合に適用されます。Claude Code v2.1.223 以降が必要です。v2.1.223 より前は、Claude Code は選択されたソースの全体 `env` ブロックのみを適用します。
  * **テレメトリユニット**：`OTEL_EXPORTER_OTLP_*` エクスポーターキー、`OTEL_LOG_*` コンテンツキャプチャトグル、`OTEL_LOGS_EXPORTER`、およびベータトレーシング変数 `ENABLE_BETA_TRACING_DETAILED` と `BETA_TRACING_ENDPOINT` は、それらのいずれかを設定する最優先ソースをユニットとして従います。`otelHeadersHelper` 認証情報キーを配信するソースもユニットを要求しますが、これらの変数は選択されたソースである場合にのみ配置されます。選択されていないが、キーを配信するソースはそれらのいずれも提供せず、下位のソースがそれらを埋めるのをブロックします。いずれにせよ、1 つのソースからのエクスポーターエンドポイントは、別のソースからの認証情報とペアになることはできません。
  * **認証情報ペアのルーティング**：`apiKeyHelper` または `otelHeadersHelper` などの選択されたソースのみの認証情報キーとペアになったルーティング変数を配信するソースは、それがスロットに勝つ場合にのみそれらのルーティング変数を提供します。
* **ゲートウェイサインインキー**：Claude Code は [`forceLoginGatewayUrl`](/docs/ja/settings-reference#forcelogingatewayurl)、[`gatewayInternalNetworks`](/docs/ja/settings-reference#gatewayinternalnetworks)、または [`forceLoginMethod`](/docs/ja/settings-reference#forceloginmethod) の `"gateway"` 値をサーバー管理設定から読み取ることはありません。したがって、サーバー管理設定の値は適用されず、MDM ポリシーまたは管理設定ファイルで設定されたものも隠されません。[`managedSourcesBehavior` エントリ](/docs/ja/settings-reference#managedsourcesbehavior)は、マシン上のどの管理ソースがそれらを提供するかを説明しています。

<h3 id="fetch-and-caching-behavior">
  フェッチとキャッシング動作
</h3>

Claude Code は起動時に Anthropic のサーバーから設定をフェッチし、アクティブなセッション中は 1 時間ごとに更新をポーリングします。

[Claude apps gateway](#platform-availability) を通じてサインインしたクライアントは、ゲートウェイから設定をフェッチし、セッションが開始される前にそのフェッチを待つため、以下のリストのフェッチはそれに適用されません。[フェッチが失敗した場合の処理](#enforce-fail-closed-startup)については、「強制的にクローズされた起動を適用する」を参照してください。

**キャッシュされた設定なしの初回起動：**

* 開発者が起動時（初回実行時や `/logout` 後など）にサインインする場合、Claude Code はセッションを開く前にフェッチを最大 5 秒間待ちます。ポリシーが時間内に到着した場合、Claude Code は最初の画面からそれを適用し、[`companyAnnouncements`](/docs/ja/settings-reference#companyannouncements) をそこに表示します。ペイロードが[セキュリティ承認](#security-approval-dialogs)を必要とする場合、Claude Code は待機を終了し、開発者が承認した後にペイロードを適用します。
* その他の起動時、および 5 秒の待機がタイムアウトした場合、Claude Code はセッションを開き、フェッチが続行されるため、設定が読み込まれ、制限が有効になるまでの短い期間が経過します。
* フェッチが失敗した場合、Claude Code はサーバー管理設定なしで続行し、対話型セッションで遠隔ポリシーが適用されないことを警告します。エンドポイント管理設定は引き続き適用されます。管理ソースが [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) を設定している場合、Claude Code は代わりに終了します。

**キャッシュされた設定での後続の起動：**

* キャッシュされた設定は起動時に直ちに適用されます。ただし、キャッシュされた `modelPricing` と `managedMcpServers` の値、およびサーバーがペイロードを確認するまで Claude Code が保留する環境変数は除きます。
* キャッシュされた [`modelPricing`](/docs/ja/settings-reference#modelpricing) は、セッションのフェッチがペイロードを確認するまで適用されません。それまで、開発者が `/usage` と状態行で見るコスト数値は定価です。
* キャッシュされた [`managedMcpServers`](/docs/ja/settings-reference#managedmcpservers) ブロックは、セッションのフェッチがペイロードを確認するまで適用されません。Claude Code はそのフェッチを最大 30 秒間待ってから MCP サーバーに接続します。フェッチが失敗またはタイムアウトした場合、セッションは組織のサーバーなしで開始され、`/status` はそう言い、後続のフェッチがそれらを確認すると接続されます。初回起動を含む完全な動作については、[提供されたサーバーが接続する場合](/docs/ja/managed-mcp#when-provided-servers-connect)を参照してください。Claude Code v2.1.259 以降が必要です。
* Claude Code はバックグラウンドで新しい設定をフェッチします。
* キャッシュされた設定はネットワーク障害を通じて保持されます。起動フェッチが失敗した場合、Claude Code は対話型セッションでキャッシュされたポリシーが有効であることを警告します。
* フェッチが成功するまで、起動時に保留された値は保留されたままです。

Claude Code は、セッションのペイロードをサーバーが確認するまで、キャッシュされた `env` ブロック内の複数のカテゴリの変数を保留します。これにより、キャッシュされたプロキシ、認証局、エンドポイント、または認証情報の値がペイロードを確認する設定フェッチをリダイレクト、傍受、または再認証することを防ぎます。強化はサーバーがフェッチした設定キャッシュにのみ適用されます。MDM または `managed-settings.json` を通じてデプロイされた[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms)は影響を受けません。保留には Claude Code v2.1.198 以降が必要です。v2.1.198 より前は、全体のキャッシュされた `env` ブロックが起動時に適用されます。保留中のカテゴリは以下の通りです。

* `HTTPS_PROXY`、`NODE_EXTRA_CA_CERTS`、mTLS クライアント証明書変数 `CLAUDE_CODE_CLIENT_CERT` および `CLAUDE_CODE_CLIENT_KEY` などのプロキシと TLS 構成
* `ANTHROPIC_BASE_URL`、`CLAUDE_CODE_USE_BEDROCK` および `CLAUDE_CODE_USE_VERTEX` などのプロバイダー選択変数、`ANTHROPIC_BEDROCK_BASE_URL` などのプロバイダーエンドポイント URL を含む API ルーティングとプロバイダー選択
* `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN`、`CLAUDE_CODE_OAUTH_TOKEN` などの認証認証情報
* 構成ディレクトリセレクター `CLAUDE_CONFIG_DIR`
* Claude Code v2.1.223 以降の認証情報ソースと構成ディレクトリセレクター：`ANTHROPIC_FEDERATION_RULE_ID` および `ANTHROPIC_IDENTITY_TOKEN` などの Workload Identity Federation 変数、プロファイルと構成ディレクトリセレクター `ANTHROPIC_PROFILE` および `ANTHROPIC_CONFIG_DIR`、およびオペレーティングシステムディレクトリ変数 `HOME`、`XDG_CONFIG_HOME`、`APPDATA`、`USERPROFILE`

Claude Code は Workload Identity Federation 変数と `ANTHROPIC_PROFILE` および `ANTHROPIC_CONFIG_DIR` セレクターを起動時にのみ読み取るため、サーバー配信値はフェッチが成功した後でもセッションの認証情報ソースを切り替えません。Claude Code v2.1.223 以降でこれらのセレクターを配信するには、MDM または `managed-settings.json` などの[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms)を使用してください。`CLAUDE_CONFIG_DIR` およびオペレーティングシステムディレクトリ変数については、保留自体が保護です。キャッシュされた値は、サーバーがペイロードを確認するまで環境から外れたままです。

キャッシュされた `env` ブロック内の他のすべてのキーは起動時に適用されます。サーバーがペイロードを確認し、[セキュリティ承認](#security-approval-dialogs)が必要な場合は承認した後、保留中の変数はセッションの残りの期間に適用されます。

組織が `api.anthropic.com` に到達するためにプロキシが必要な場合、保留はサーバー配信 `env` ブロック自体にのみ影響します。MDM または `managed-settings.json` を通じた[エンドポイント管理](/docs/ja/managed-settings#delivery-mechanisms) `env` ブロック、シェル環境、または[ユーザー設定](/docs/ja/settings#where-settings-live)で設定されたプロキシは設定フェッチに到達します。エンドポイント管理ソースには Claude Code v2.1.223 以降が必要です。キャッシュされたサーバー管理プロキシ値はフェッチがそれを確認するまで保留されるため、エンドポイント管理値はキー単位で埋まり、フェッチ自体に到達します。v2.1.223 より前は、シェル環境またはユーザー設定を使用して、プロキシがキャッシュされたサーバーペイロードと一緒に適用されるようにしてください。初回起動にはキャッシュがないため、エンドポイント管理ソース、シェル環境、またはユーザー設定は初期フェッチに引き続き必要です。

Claude Code はほとんどの設定更新を実行中のセッションに再起動なしで適用します。OpenTelemetry エクスポーター構成、`model` キー、および `env` ブロックからの変数の削除を含む一部の更新は、次の起動時にのみ適用されます。

<h3 id="invalid-entries-in-delivered-settings">
  配信された設定の無効なエントリ
</h3>

ペイロードの一部がスキーマ検証に失敗した場合、Claude Code は検証エラーを表示し、残りのすべての有効な設定を適用します。[管理設定の無効なエントリ](/docs/ja/managed-settings#invalid-entries-in-managed-settings)は、削除されるもの、およびどのキーがより厳密な値にフォールバックするかを説明しています。Claude Code v2.1.169 以降が必要です。

サーバー管理配信は、これらの動作を追加します。

* `~/.claude/remote-settings.json` のキャッシュは、無効なエントリが削除された救済されたペイロードを保存します。ただし、無効な `cleanupPeriodDays` および `desktopSessionCleanupPeriodDays` 値は除きます。これらはキャッシュされたコピーに留まり、適用されることはありません。
* ペイロード内のフィールドが救済できず、ペイロードがこれらの保持キーのみではない場合、Claude Code はペイロードを拒否し、最後に受け入れられたキャッシュされた設定を保持し、`Remote settings: Settings validation failed - no fields could be salvaged` をデバッグログに書き込みます。`forceRemoteSettingsRefresh` が設定されている場合、CLI は代わりに終了します。
* [セキュリティ承認ダイアログ](#security-approval-dialogs)は救済されたペイロードを評価するため、削除された無効なエントリは承認のために提示されず、実行されません。

配信の問題をデバッグするには、`claude --debug-file <path>` を実行し、ログで `Remote settings` を検索してください。組織にロールアウトする前に、テストマシンで `claude doctor` を使用してペイロード変更を検証してください。

<h3 id="enforce-fail-closed-startup">
  強制的にクローズされた起動を適用する
</h3>

デフォルトでは、起動時にリモート設定フェッチが失敗した場合、CLI は最後の成功したフェッチからキャッシュされた設定で続行します。ただし、[Claude Code が保留する値](#fetch-and-caching-behavior)はフェッチが成功するまで保留されたままです。それを一度もフェッチしたことがないマシンでは、CLI はサーバー管理設定なしで続行し、デバイス上の[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms)は引き続き適用されます。

クライアントがキャッシュされたまたは不在のサーバー管理設定で起動するのを防ぐには、管理設定で `forceRemoteSettingsRefresh: true` を設定します。

[Claude apps gateway](#platform-availability) を通じてサインインしたクライアントは、この設定を設定しているかどうかに関わらず起動フェッチを待ち、失敗したフェッチを次のように処理します。

* ゲートウェイが出席型対話型起動に `401` で応答し、この設定がオフの場合、ゲートウェイはそのサインインを終了しました。Claude Code は [`Cloud gateway session expired — run /login to reconnect.`](/docs/ja/errors#cloud-gateway-session-expired) を出力し、ユーザーが `/login` を実行するまでゲートウェイからサインアウトしたセッションを開きます。
* フェッチが他の方法で失敗した場合、または `claude auth` サブコマンド以外の他の種類の起動の場合、クライアントはエラーで終了します。

この設定がサーバー管理設定をフェッチするセッションでアクティブな場合、CLI は起動時にリモート設定が新しくフェッチされるまでブロックされます。フェッチが失敗した場合、CLI はポリシーなしで続行するのではなく、終了します。この設定は自己永続化します。サーバーから配信されると、ローカルにもキャッシュされるため、新しいセッションの最初の成功したフェッチの前でも、後続の起動は同じ動作を適用します。[サーバー管理設定をフェッチしないセッション](#platform-availability)は待機なしで開始されます。

これを有効にするには、管理設定構成にキーを追加します。

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

[エンドポイント管理](/docs/ja/managed-settings#delivery-mechanisms)MDM プロファイルまたはシステム `managed-settings.json` ファイルでこのキーを設定して、最初の起動時にクローズされた失敗動作を適用することもできます。サーバーペイロードが配信される前です。このフラグは上記の[優先順位ルール](#settings-precedence)の例外です。Claude Code は、キャッシュされたサーバー管理ペイロードも存在する場合でも、管理者が管理する管理ソースがそれを設定する場合にそれを尊重するため、MDM 配信値はサーバー管理設定が存在する場合は無視されません。

[`policyHelper`](/docs/ja/settings-reference#policyhelper) が管理設定を提供する場合、その出力は起動後に Claude Code が読み取るキーのすべての他の管理ソースを置き換えます。Claude Code がこのキーを読み取るソースについては、[その設定エントリ](/docs/ja/settings-reference#forceremotesettingsrefresh)を参照してください。`policyHelper` エントリは、Claude Code がヘルパーを読み取るソースと実行時期を説明しています。

設定フェッチは `Cache-Control: no-cache` ヘッダーも送信するため、中間 HTTP プロキシは古い応答を提供しません。

この設定を有効にする前に、ネットワークポリシーが `api.anthropic.com` への接続を許可していることを確認してください。そのエンドポイントに到達できない場合、CLI は起動時に終了し、ユーザーは Claude Code を開始できません。

`claude auth` サブコマンド（`claude auth login` など）はこのチェックとゲートウェイ起動終了から除外されるため、期限切れの認証情報が設定フェッチが失敗する理由である場合、ユーザーは再認証できます。

<h3 id="security-approval-dialogs">
  セキュリティ承認ダイアログ
</h3>

セキュリティリスクをもたらす可能性のある特定の設定には、対話型セッションで Claude Code が適用する前に明示的なユーザー承認が必要です。

* **シェルコマンド設定**：`apiKeyHelper`、`statusLine`、`otelHeadersHelper` などのシェルコマンドを実行する設定
* **サンドボックスバイナリ設定**：`sandbox.bwrapPath`、`sandbox.socatPath`、`sandbox.ripgrep`。これらの各設定は実行可能ファイルを指し、Claude Code はその実行可能ファイルを実行します。
* **サンドボックスネットワークと分離設定**：[sandbox](/docs/ja/sandboxing) 設定。サンドボックスプロキシがトラフィックを読み取り、再ルーティング、または認証できるようにするか、サンドボックスの分離を弱める設定：`sandbox.network.tlsTerminate`、`sandbox.network.httpProxyPort`、`sandbox.network.socksProxyPort`、`sandbox.credentials`、`sandbox.allowAppleEvents`、`sandbox.enableWeakerNestedSandbox`、`sandbox.enableWeakerNetworkIsolation`、`sandbox.filesystem.disabled`、`sandbox.network.allowAllUnixSockets`、`sandbox.network.allowUnixSockets`、`sandbox.network.allowMachLookup`。`deny` ルールのみを含む `sandbox.credentials` ブロックは、プロキシに認証情報を与えずにサンドボックスを制限するため、承認は必要ありません。v2.1.251 より前は、Claude Code はこれらの設定を承認なしで適用しました。
* **カスタム環境変数**：プロキシおよびベース URL 変数など、ユーザーの承認を必要とする配信 `env` 変数。[環境変数と承認ダイアログ](#environment-variables-and-the-approval-dialog)を参照してください。
* **フック構成**：任意のフック定義

これらの設定が存在する場合、ユーザーは構成されている内容を説明するセキュリティダイアログを表示します。ユーザーは続行するために承認する必要があります。ユーザーが設定を拒否した場合、Claude Code は終了します。

[`claudeMd`](/docs/ja/settings-reference#claudemd) キーを通じて配信される管理 CLAUDE.md は、Claude Code が実行するコマンドではなく Claude の命令テキストであるため、承認は必要ありません。Claude Code は、これらの命令に従う際に Claude が使用するツールの[権限](/docs/ja/permissions)をチェックします。v2.1.260 より前は、`claudeMd` 値は承認も必要でした。

<h4 id="approval-memory">
  承認メモリ
</h4>

Claude Code は、設定ディレクトリ `~/.claude`（[`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars) を設定しない限り）に承認を記録します。記録内容は、設定フェッチが使用する認証情報によって異なります。

* **`/login` または `claude auth login` で保存された claude.ai ログイン、または[キーレスコンソールサインイン](/docs/ja/authentication#sign-in-without-an-api-key)**：組織ごとに 1 つの承認。最近承認したアカウントが保持します。
* **[Claude apps gateway](/docs/ja/claude-apps-gateway) サインイン**：ゲートウェイごとに 1 つの承認。

  同じゲートウェイからサインアウトして再度サインインした場合、承認を必要とする設定が変わらない限り、Claude Code はダイアログを再度表示しません。これらの設定が変わる場合、別のゲートウェイにサインインする場合、および同じゲートウェイの新しい証明書を受け入れる場合、Claude Code はそれを再度表示します。

  Claude Code は、平文 HTTP を介して到達されるループバック開発ゲートウェイの承認を保存しないため、ダイアログは各サインイン後に再度表示されます。
* **API キーまたは `CLAUDE_CODE_OAUTH_TOKEN` などの他の認証情報**：配信された設定の承認。その構成ディレクトリ内の設定のキャッシュされたコピーと一緒に保持されます。Claude Code は、承認を必要とする設定が変わる場合、および `/logout` または `claude auth logout` を実行する場合にダイアログを再度表示します。どちらもキャッシュされたコピーを削除します。

`sandbox.credentials` または `sandbox.network.tlsTerminate` の承認は、同じ配信設定内の [`sandbox.network.allowedDomains`](/docs/ja/settings-reference#sandbox-network-alloweddomains) エントリもカバーします。両方の設定がそのホワイトリストに作用するためです。管理者がそれらのエントリのいずれかを追加または削除する場合、ダイアログが再度表示されます。`sandbox.network.allowedDomains` は独自に承認を必要としませんが。

保存された claude.ai ログインの場合：

* サインアウトして再度サインインするか、別の組織に切り替えて後で戻る場合、これらの設定が変わらない限り、Claude Code はダイアログを再度表示しません。ただし、その間に別のアカウントがその構成ディレクトリ内のその組織のためにそれらを承認した場合は除きます。
* 別のアカウントで同じ組織にサインインする場合、設定が変わらない場合でも Claude Code はダイアログを再度表示します。そのアカウントの承認は前のものを置き換えるため、切り替え直すと Claude Code はもう一度表示します。

Claude Code は常にダイアログを表示できるわけではありません。以下の各ケースは、表示できない場合に適用される設定と、次にダイアログが表示される場合を説明しています。

* **ダイアログを表示できない対話型セッション**：Claude Code は配信された設定を適用せず、最後に承認された設定を保持します。ダイアログは、それを表示できる次のセッションに表示されます。Claude Code v2.1.211 以降が必要です。
* **`claude install` または `claude update`**：Claude Code はどちらのコマンド中もダイアログを表示しません。コマンドは最後に承認された設定で実行され、ダイアログは次の対話型セッションに表示されます。Claude Code が起動時に設定フェッチを待つ場合（[`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) が設定されている場合、または [Claude apps gateway](/docs/ja/claude-apps-gateway) デプロイメント上など）、代わりにコマンド中にダイアログを表示し、パイプからのインストール実行は失敗します。[インストール中の「Raw mode is not supported」](/docs/ja/troubleshoot-install#raw-mode-is-not-supported-during-install)を参照してください。v2.1.246 より前は、Claude Code はこれらのコマンド中もダイアログを表示しようとしました。
* **エラーがダイアログを閉じる前に答える**：Claude Code は配信された設定を適用せず、最後に承認された設定を保持します。それを表示できる次のセッションでダイアログを再度表示します。
* **`claude -p` または Agent SDK セッションなどの非対話型実行**：Claude Code はダイアログを表示できないため、配信された設定が承認を必要とする場合、その実行のためだけにそれらを適用します。[ローカルキャッシュ](#fetch-and-caching-behavior)に承認済みとして記録したり、書き込んだりしません。次の対話型セッションはダイアログを表示します。ユーザーが対話型セッションで承認するまで、各非対話型実行は起動時に設定を再度フェッチします。v2.1.207 より前は、非対話型実行は設定を承認済みとして保存したため、後続の対話型セッションはそれらのダイアログを表示しませんでした。

<h4 id="environment-variables-and-the-approval-dialog">
  環境変数と承認ダイアログ
</h4>

Claude Code は、承認ダイアログを表示せずに配信された一部の `env` 変数を適用します。以下を含みます。

* 機能とコマンドトグル
* `ANTHROPIC_MODEL`、`DISABLE_PROMPT_CACHING`、`CLAUDE_CODE_EFFORT_LEVEL` などのモデル選択と動作設定
* `DISABLE_AUTO_COMPACT` などのコンテキストウィンドウと圧縮設定
* ターミナル UI とアクセシビリティオプション
* 数値制限、予算、タイムアウト

他の配信変数は、有効になる前にユーザーの承認を必要とする場合があります。空でないプロキシ、ベース URL、または `OTEL_EXPORTER_OTLP_ENDPOINT` 値は常にそうです。配信変数が承認を必要とする場合、ダイアログはそれに名前を付けるため、ユーザーはポリシーが設定するよう求めているものを正確に見ます。v2.1.218 より前は、Claude Code は少ない変数を承認なしで適用したため、`DISABLE_AUTO_COMPACT` などの設定は空でない値でダイアログをトリガーしました。

Claude Code は、変数名ではなく配信値によって 4 つのプライバシートグルが承認を必要とするかどうかを決定します。`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`、`DISABLE_ERROR_REPORTING`、`DISABLE_TELEMETRY`、`DO_NOT_TRACK`。`1` または `true` などの真の値は、トラッキング、レポート、またはその他の非必須トラフィックをオフにするだけなので、Claude Code はユーザーに尋ねずにそれを適用します。他の空でない値の場合、Claude Code はダイアログを表示します。v2.1.218 より前は、`DO_NOT_TRACK` を除くすべてが空でない値で承認なしで適用され、`DO_NOT_TRACK` は空でない値でダイアログをトリガーしました。

Claude Code は、配信値によって [`API_FORCE_IDLE_TIMEOUT`](/docs/ja/env-vars) が承認を必要とするかどうかも決定します。真の値は[ボディアイドルタイムアウト](/docs/ja/network-config#streaming-idle-watchdogs)をオンにするだけなので、Claude Code はユーザーに尋ねずにそれを適用します。他の空でない値の場合、Claude Code はダイアログを表示します。v2.1.248 より前は、空でない値はダイアログをトリガーしました。

[`ANTHROPIC_CUSTOM_HEADERS`](/docs/ja/env-vars#variables) が承認を必要とするかどうかも配信値に依存します。`Accept-Language` などのリクエストにタグを付けるだけのヘッダーはダイアログなしで適用されます。認証情報、org または tenant セレクター、ルーティングまたはホストオーバーライド、または `Authorization`、`X-Api-Key`、`Host`、`anthropic-beta`、`X-Amzn-Bedrock-*` ヘッダーなどの API 動作ヘッダーに名前を付ける行は承認を必要とします。有効な HTTP ヘッダートークンではない名前の行、またはその値が HTTP ヘッダーが実行できない文字を含む行も同様です。チェックはヘッダー名内の単語と一致するため、`client` と `version` を含む `X-Client-Version` も承認を必要とします。v2.1.251 より前は、任意の `ANTHROPIC_CUSTOM_HEADERS` 値はそれなしで適用されました。

[`ENABLE_BETA_TRACING_DETAILED`](/docs/ja/env-vars#variables) または [`OTEL_LOG_RAW_API_BODIES`](/docs/ja/env-vars#variables) の `0` または `false` などの偽の値は、詳細なトレーシングまたは生 API ボディキャプチャをオフにするだけなので、ダイアログなしで適用されます。どちらかの変数の他の空でない値は承認を必要とします。

<h2 id="platform-availability">
  プラットフォームの可用性
</h2>

サーバー管理設定は `api.anthropic.com` への直接接続が必要です。配信にはセッションが以下のいずれかの認証情報で認証される必要があります。

* Team または Enterprise OAuth ログイン
* `CLAUDE_CODE_OAUTH_TOKEN` を通じて提供される OAuth トークン
* 直接設定された API キー
* Anthropic プロファイルの `user_oauth`（\[Anthropic プロファイル]\(/ja/authentication#anthropic-profiles-and-federation-credentials）、ただしプロファイルが Anthropic API 以外の `base_url` を設定していない場合）。Claude Code v2.1.257 以降が必要です。

[`apiKeyHelper`](/docs/ja/settings-reference#apikeyhelper) スクリプトによって返されたキーも [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) 認証情報も設定フェッチをトリガーしません。

Claude Desktop アプリの [Cowork](https://claude.com/docs/cowork/overview) セッションでは、ユーザーが Team または Enterprise アカウントでサインインしている場合でも、Claude Code は claude.ai 管理コンソールからサーバー管理設定をフェッチしません。[ポリシーが適用される場所と時期](/docs/ja/managed-settings#where-and-when-a-policy-applies) では、ユーザーのマシン上の Cowork セッションとリモート Cowork セッションにどのポリシーが適用されるかについて説明しています。claude.ai は、Cowork ユーザーが claude.ai の git リポジトリからマーケットプレイスを追加するか、Cowork タブの **Customize** から追加する場合、[`strictKnownMarketplaces`](/docs/ja/settings-reference#strictknownmarketplaces) および [`blockedMarketplaces`](/docs/ja/settings-reference#blockedmarketplaces) リストを自身で適用します。[制限がどのように機能するか](/docs/ja/plugins/org#restrict-what-users-can-install) がそのチェックについて説明しています。

シェルで `CLAUDE_CODE_USE_*` プロバイダー変数またはデフォルト以外の `ANTHROPIC_BASE_URL` をエクスポートする場合、Claude Code はセッションの設定フェッチをスキップします。[`claude doctor` と `/status` はスキップされたフェッチとその原因を報告します](#verify-settings-delivery)。

エクスポートがフェッチを防ぐため、サーバー管理 `env` ブロックでエクスポートをクリアすることはできません。[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms) `env` ブロックもフェッチを復元しません。Claude Code は管理 `env` ブロックを適用する前に適格性をチェックするため、エンドポイント管理値はセッションのプロバイダー選択を変更しますが、フェッチはスキップされたままです。

サーバー管理配信を復元するには、シェルからエクスポートを削除するか、ユーザー設定 `env` ブロックで変数を `""` に設定します。これは適格性チェックの前に適用されます。ユーザーがシェルを変更することに依存せずにポリシーを適用するには、代わりにエンドポイント管理チャネルを通じて設定を配信してください。

Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、および [Claude Platform on AWS](/docs/ja/claude-platform-on-aws) デプロイメントの場合、自己ホスト型の [Claude apps ゲートウェイ](/docs/ja/claude-apps-gateway) は同等のリモート管理設定配信を提供します。ゲートウェイにサインインしたクライアントは、`api.anthropic.com` の代わりにゲートウェイから管理設定をフェッチします。起動時の失敗セマンティクスは異なります。ゲートウェイに到達できないゲートウェイクライアントはキャッシュされた設定にフォールバックする代わりにエラーで終了しますが、時間ごとのバックグラウンド更新は両方のチャネルで fail-open です。

<h2 id="audit-logging">
  監査ログ
</h2>

設定変更の監査ログイベントは、コンプライアンス API または監査ログエクスポートを通じて利用可能です。アクセスについては、Anthropic アカウントチームにお問い合わせください。

監査イベントには、実行されたアクションのタイプ、アクションを実行したアカウントとデバイス、および前の値と新しい値への参照が含まれます。

<h2 id="security-considerations">
  セキュリティに関する考慮事項
</h2>

サーバー管理設定は一元的なポリシー適用を提供しますが、クライアント側の制御として機能し、セキュリティ境界ではありません。管理されていないデバイスでは、ユーザーは管理者または sudo アクセス権を持つ必要なく、これらをバイパスできます。

| シナリオ                                                 | 動作                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :--------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ユーザーがキャッシュされた設定ファイルを編集する                             | 改ざんされたファイルは起動時に適用されますが、[サーバーがペイロードを確認するまで Claude Code が保留する値](#fetch-and-caching-behavior)を除きます。次のサーバーフェッチで正しい設定が復元されます。ただし、[次回の起動時にのみ適用されるキー](#fetch-and-caching-behavior)（`model` や `env` ブロックに追加された変数など）は、再起動されるまで有効なままです                                                                                                                                                                                                                                             |
| ユーザーがキャッシュされた設定ファイルを削除する                             | [初回起動動作](#fetch-and-caching-behavior)が発生します                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ユーザーが変更された Claude Code バイナリを実行する                     | 変更されたクライアントを実行できるユーザーは、クライアント側の制御をバイパスできます                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ユーザーが古い Claude Code バージョンを実行する                       | サーバー管理設定より前のバージョンは、これらをフェッチまたは適用しません                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| API が利用不可                                            | キャッシュされた設定が利用可能な場合は適用されます。ただし、[フェッチが成功するまで Claude Code が保留する値](#fetch-and-caching-behavior)を除きます。キャッシュがない場合、Claude Code は次の成功したフェッチまでサーバー管理設定を適用しません。また、デバイス上の[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms)は引き続き適用されます。`forceRemoteSettingsRefresh: true` の場合、CLI は続行するのではなく終了します。ただし、[`claude auth` サブコマンド](#enforce-fail-closed-startup)を除きます。[Claude apps gateway](#platform-availability)を通じてサインインしているクライアントは、その設定がない場合、起動時に終了します。同じ `claude auth` 除外があります |
| ユーザーが別の組織で認証する                                       | 管理対象組織外のアカウントには設定が配信されません                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ユーザーが[サードパーティモデルプロバイダー](#platform-availability)を構成する | サーバー管理設定はバイパスされます。これには `CLAUDE_CODE_USE_BEDROCK`、`CLAUDE_CODE_USE_MANTLE`、`CLAUDE_CODE_USE_VERTEX`、`CLAUDE_CODE_USE_FOUNDRY`、`CLAUDE_CODE_USE_ANTHROPIC_AWS`、またはデフォルト以外の `ANTHROPIC_BASE_URL` の設定が含まれます                                                                                                                                                                                                                                                                  |
| ネットワークトラフィックが傍受またはリダイレクトされる                          | TLS 検証が無効化されたか、傍受されたトラフィックは、クライアントが受け取る設定を変更できます                                                                                                                                                                                                                                                                                                                                                                                                                         |

ローカル設定ファイル（`managed-settings.json` を含む）への編集をログに記録するには、[`ConfigChange` フック](/docs/ja/hooks#configchange)を使用してください。Claude Code は、サーバー管理設定が到着または更新されるとき、または MDM プロファイルまたはレジストリポリシーが変更されるときにこれらを実行しません。また、フックは `policy_settings` の変更をブロックできません。

ユーザーがクライアントが提供する認証情報でアクセスできる組織を制限するには、Claude ヘルプセンターの[テナント制限を使用したネットワークレベルのアクセス制御の適用](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions)を参照してください。より強力な適用保証については、MDM ソリューションに登録されているデバイスで[エンドポイント管理設定](/docs/ja/managed-settings#delivery-mechanisms)を使用してください。

<h2 id="see-also">
  関連項目
</h2>

Claude Code 構成を管理するための関連ページ：

* [すべての設定](/docs/ja/settings-reference)：すべての設定キー
* [Endpoint-managed settings](/docs/ja/managed-settings#delivery-mechanisms)：IT によってデバイスに配置される管理設定
* [Authentication](/docs/ja/authentication)：Claude Code へのユーザーアクセスのセットアップ
* [Security](/docs/ja/security)：セキュリティ保護とベストプラクティス
