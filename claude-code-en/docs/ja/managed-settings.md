> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# マネージド設定をデプロイする

> すべての開発者のマシンにマネージド設定をデプロイします。OS ごとの配信メカニズム、Claude Code がマネージドソースを組み合わせる方法、および強制の検証方法について説明します。

マネージド設定は、組織がすべての開発者のマシンにデプロイする設定です。Claude Code はこれらを他のすべてのレベルの上に適用するため、ユーザー、プロジェクト、ローカル、または `--settings` の値は、いくつかの [セキュリティに関連する例外](/docs/ja/settings#exceptions-to-managed-settings-precedence) を除いて、これらをオーバーライドできません。これらの例外では、下位レベルからのより厳密な値がカウントされます。

このページは、マネージド設定をデプロイするか、設定が適用されない理由をデバッグする管理者向けです。強制する内容を決定するには、[強制する内容を決定する](/docs/ja/admin-setup#decide-what-to-enforce) テーブルから始めてください。claude.ai コンソールパスについては、[サーバーマネージド設定](/docs/ja/server-managed-settings) を参照してください。開発者自身の値がどのファイルに入るかについては、[設定](/docs/ja/settings) を参照してください。

<h2 id="deploy-a-managed-settings-file">
  マネージド設定ファイルをデプロイする
</h2>

これは各マシンにポリシーを配置する最速の方法です。`managed-settings.json` ファイルです。マネージド設定の配信方法をまだ選択していない場合、またはデバイスが MDM 下にあるか開発者がクラウドセッションを実行している場合は、最初に [配信メカニズムを選択する](#choose-a-delivery-mechanism) を読んでください。

<Steps>
  <Step title="managed-settings.json を作成する">
    強制することを決定したキーを保持する `managed-settings.json` を作成します。これは `settings.json` と同じ JSON 形式です。[強制する内容を決定する](/docs/ja/admin-setup#decide-what-to-enforce) テーブルは各コントロールの背後にあるキーをリストしており、[設定リファレンス](/docs/ja/settings-reference) の各エントリは、マネージドソースがそれを設定できるかどうかを示しています。このファイルは 2 つのファイル読み取りをブロックし、バイパスモードをオフにし、Claude Code がユーザー、プロジェクト、ローカルファイルおよび `--allowedTools` からの権限ルールを無視するようにします。

    ```json managed-settings.json theme={null}
    {
      "permissions": {
        "deny": [
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    ログイン方法、モデル、MCP サーバー、マーケットプレイスを含むより多くのマネージドキーの形状を示すより完全な例については、[組織のマネージド設定](/docs/ja/settings-example#an-organizations-managed-settings) を参照してください。
  </Step>

  <Step title="ファイルを各マシンに配置する">
    ファイルを `managed-settings.json` として、オペレーティングシステムのシステムディレクトリに保存します。フリート上のファイルを配置するために既に使用しているツールを使用します。

    * **macOS**: `/Library/Application Support/ClaudeCode/managed-settings.json`
    * **Linux と WSL**: `/etc/claude-code/managed-settings.json`
    * **Windows**: `C:\Program Files\ClaudeCode\managed-settings.json`
  </Step>

  <Step title="ポリシーが適用されたことを確認する">
    1 つのマシンで、Claude Code 内で `/status` を実行します。`Setting sources` 行は `Enterprise managed settings (file)` を表示します。その後、フリートの残りにロールアウトします。[ポリシーが有効であることを確認する](#check-that-a-policy-is-in-force) は、行が見つからない場合に何を確認するかについて説明しています。
  </Step>
</Steps>

<span id="managed-settings-delivery" />

<span id="delivery-mechanisms" />

<h2 id="choose-a-delivery-mechanism">
  配信メカニズムを選択する
</h2>

上記のステップのファイルは、マネージド設定をマシンに取得する 4 つの方法の 1 つです。すべてのメカニズムは `settings.json` ファイルと同じポリシーキーを持つため、[設定リファレンス](/docs/ja/settings-reference) はすべてに適用されます。いくつかのキーは特定のソースに関連付けられており、各エントリの Scope 行はどれかを示しています。

* **配信コントロール**: [`policyHelper`](/docs/ja/settings-reference#policyhelper)、[`wslInheritsWindowsSettings`](/docs/ja/settings-reference#wslinheritswindowssettings)、および [`managedSourcesBehavior`](/docs/ja/settings-reference#managedsourcesbehavior)
* **ゲートウェイログインキー**: [`forceLoginGatewayUrl`](/docs/ja/settings-reference#forcelogingatewayurl)、[`gatewayInternalNetworks`](/docs/ja/settings-reference#gatewayinternalnetworks)、および [`forceLoginMethod`](/docs/ja/settings-reference#forceloginmethod) の `"gateway"` 値

マネージド設定ファイル、MDM プロファイル、または claude.ai コンソールは、それが到達するすべてのユーザーに 1 つのポリシーを適用します。開発者の 1 つのグループに異なるポリシーを提供するには、異なるファイルまたはプロファイルをそのグループにデプロイします。claude.ai コンソール [はまだグループをターゲットにできません](/docs/ja/server-managed-settings#current-limitations)。一方、自己ホスト型の [Claude apps gateway](/docs/ja/claude-apps-gateway) は IdP グループごとにマネージド設定を配信します。

複数のメカニズムが同じマシンにポリシーを配信する場合、Claude Code はデフォルトで 1 つを使用し、他を無視します。[Claude Code がマネージドソースを組み合わせる方法](#how-claude-code-combines-managed-sources) は順序と適用される opt-in を示しています。

MDM とファイル行は一緒に endpoint-managed settings と呼ばれます。ポリシーが開発者のデバイスに保存されているためです。これは server-managed 行とは対照的です。Claude Code はそれをフェッチします。

下記のテーブルを使用して、デバイスを既に管理している方法に基づいてメカニズムを選択してください。

| メカニズム                                      | 配信方法                                                                                                                                                | Claude Code がそれを読む時期                                                                                                                               | 使用する場合                                        |
| :----------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------- |
| [サーバーマネージド設定](/docs/ja/server-managed-settings) | claude.ai 管理コンソール内、または自己ホスト型 [Claude apps gateway](/docs/ja/claude-apps-gateway) 上                                                                       | スタートアップ時にフェッチされ、1 時間ごとにポーリングされます。[ポリシーが適用される場所と時期](#where-and-when-a-policy-applies) を参照してください                                                     | 各マシンに触れずに claude.ai 組織のポリシーを変更する 1 つの場所が必要な場合 |
| MDM または OS レベルのポリシー                        | macOS 構成プロファイルまたは Windows `HKLM` レジストリ値として、Jamf、Intune、グループポリシー、または同様のツール経由。[各メカニズムがポリシーを保存する場所](#where-each-mechanism-stores-the-policy) を参照してください | スタートアップ時に読み取られ、30 分ごとに変更がチェックされます                                                                                                                  | MDM またはグループポリシーでデバイスを既に管理している場合               |
| ファイルベース                                    | 各マシンのシステムディレクトリ内の `managed-settings.json` として。[各メカニズムがポリシーを保存する場所](#where-each-mechanism-stores-the-policy) を参照してください                               | スタートアップ時に読み取られ、ファイルが変更されるとリロードされます                                                                                                                 | MDM なしのマシン、Linux ホスト、または自分で構築するイメージ           |
| HKCU レジストリ、Windows と WSL                   | Windows `HKCU` レジストリ値として。[各メカニズムがポリシーを保存する場所](#where-each-mechanism-stores-the-policy) を参照してください                                                    | スタートアップ時に読み取られ、30 分ごとに変更がチェックされます。Claude Code はそれを使用するのは、他のマネージドソースがポリシーキーを配信せず、[ホスト提供の親設定](#let-an-embedding-host-add-policy) が制限的なキーを提供しない場合のみです | マシンレベルの `HKLM` キーを書き込むことができない場合               |

Jamf、Iru、Intune、グループポリシーのスターターテンプレートは、[MDM 例リポジトリ](https://github.com/anthropics/claude-code/tree/main/examples/mdm) にあります。

`managed-mcp.json` を通じてデプロイするか、[`managedMcpServers`](/docs/ja/settings-reference#managedmcpservers) キーを通じて提供するマネージド MCP サーバーについては、[マネージド MCP 構成](/docs/ja/managed-mcp) を参照してください。

<h3 id="where-and-when-a-policy-applies">
  ポリシーが適用される場所と時期
</h3>

デプロイされたポリシーは、開発者のセッションに次のように到達します。

* **サーフェス**: 開発者のマシン上で、ターミナル、VS Code および JetBrains 拡張機能、デスクトップアプリの Code タブ、および [Agent SDK](/docs/ja/agent-sdk/typescript) セッションはこれらのソースをすべて読み取ります。Agent SDK セッションは、`settingSources` がユーザー、プロジェクト、ローカルファイルを除外する場合でも、マネージド設定をロードします。
* **クラウドセッション**: Anthropic ホスト環境のセッションはデバイスの MDM プロファイルまたはファイルを読み取らないため、ポリシーはサーバーマネージド設定から来る必要があります。[自己ホスト環境](/docs/ja/self-hosted-environments) のセッションは、デフォルトではサーバーマネージド設定がポリシーキーを配信しない場合のみ、ランナーイメージ内のマネージド設定ファイルを読み取ります。ただし、[すべての管理ソースから Claude Code が読み取るキー](#keys-read-from-every-admin-source) は除きます。[Claude Code がマネージドソースを組み合わせる方法](#how-claude-code-combines-managed-sources) は両方を適用する opt-in について説明しています。
* **Cowork セッション**: Claude Desktop アプリの [Cowork](https://claude.com/docs/cowork/overview) は Claude Code 上でセッションを実行します。Cowork セッションでは、Claude Code は Team または Enterprise アカウントでユーザーがサインインしている場合でも、claude.ai 管理コンソールからサーバーマネージド設定をフェッチしません。したがって、どのポリシーが適用されるかはセッションが実行される場所によって異なります。

  * **ユーザーのマシン上**: デフォルトでは、Cowork セッションの Claude Code はそのデバイス上の MDM または OS レベルのポリシーおよびマネージド設定ファイルを読み取るため、ポリシーをそこにデプロイします。
  * **完全な VM サンドボックス内**: Claude Desktop マネージド構成が [`requireCoworkFullVmSandbox`](https://claude.com/docs/third-party/claude-desktop/configuration#requirecoworkfullvmsandbox) を設定する場合、Claude Code は仮想マシン内で実行され、デバイスの MDM ポリシーおよびマネージド設定ファイルは存在しません。
  * **リモート Cowork セッション**: これらは Anthropic 管理 VM 上で実行され、Claude Code はデバイスポリシーを読み取ることができません。

  セッションが実行される場所に関係なく、claude.ai は管理コンソールの [`strictKnownMarketplaces`](/docs/ja/settings-reference#strictknownmarketplaces) および [`blockedMarketplaces`](/docs/ja/settings-reference#blockedmarketplaces) リストを、誰かが claude.ai 上の git リポジトリからマーケットプレイスを追加するか、Cowork タブの **Customize** から追加する場合に自動的に適用します。[制限がどのように機能するか](/docs/ja/plugins/org#restrict-what-users-can-install) はそのチェックについて説明しています。[サーフェスカバレッジ](/docs/ja/model-config#surface-coverage) テーブルは Cowork と他のサーフェスを比較しています。
* **実行中のセッション**: ほとんどの変更は、[配信メカニズムテーブル](#choose-a-delivery-mechanism) のスケジュールに従って、再起動なしで実行中のセッションに到達します。
  * [`forceRemoteSettingsRefresh`](/docs/ja/settings-reference#forceremotesettingsrefresh)、[`requiredMinimumVersion`](/docs/ja/settings-reference#requiredminimumversion)、および [いくつかのユーザー編集可能キー](/docs/ja/settings#when-edits-take-effect) への変更は、次のセッション開始時に有効になります。
  * 新規または変更された [`policyHelper`](/docs/ja/settings-reference#policyhelper) エントリは次の起動時に有効になります。ただし、起動時にサーバーマネージド設定によってシャドウされたヘルパーは、フェッチがそれらの設定が削除されたことを報告するとすぐに実行されます。
* **承認が必要な変更**: [次の起動を待つ更新](/docs/ja/server-managed-settings#fetch-and-caching-behavior) とは別に、[承認が必要な](/docs/ja/server-managed-settings#security-approval-dialogs) 設定（フックまたは `env` 変数など）へのサーバーマネージド変更は、開発者がインタラクティブセッションでダイアログを受け入れるのを待ち、IDE 拡張機能または Agent SDK がホストするセッションの現在の実行に適用されます。その他のサーバーマネージド変更は次のポーリングで適用されます。
* **長時間実行セッション**: 数週間開いたままのセッションはロールアウトに遅れることができます。[`requiredMinimumVersion`](/docs/ja/settings-reference#requiredminimumversion) は古いバイナリが開始されるのをブロックし、既に実行中のセッションを終了しません。

<span id="format-the-policy-for-each-platform" />

<h3 id="where-each-mechanism-stores-the-policy">
  各メカニズムがポリシーを保存する場所
</h3>

キーはどこでも同じですが、各メカニズムはそれらを異なる場所と形状に保存します。

* **サーバーマネージド**: Anthropic のサーバーまたはゲートウェイがポリシーを保持します。Claude Code はローカルキャッシュを保持し、スタートアップ時に適用し、[各成功したフェッチで置き換えます](/docs/ja/server-managed-settings#security-considerations)。
* **macOS 構成プロファイル**: `com.anthropic.claudecode` マネージド設定ドメイン。`managed-settings.json` と同じトップレベルキーを使用し、ネストされた設定は辞書として、リストは plist 配列として使用します。
* **Windows HKLM レジストリ**: `HKLM\SOFTWARE\Policies\ClaudeCode` の下の `Settings` という名前の `REG_SZ` または `REG_EXPAND_SZ` 値として JSON。
* **ファイルベース**: `managed-settings.json`、オプションの `managed-settings.d/` ディレクトリ、および `managed-mcp.json` をシステムディレクトリに配置します。macOS では `/Library/Application Support/ClaudeCode/`、Linux と WSL では `/etc/claude-code/`、Windows では `C:\Program Files\ClaudeCode\`。Claude Code はレガシー Windows パス `C:\ProgramData\ClaudeCode\managed-settings.json` を読み取りません。
* **Windows HKCU レジストリ**: `HKCU\SOFTWARE\Policies\ClaudeCode` の下の同じ `Settings` 値。

<h3 id="split-a-file-based-policy-across-teams">
  ファイルベースのポリシーをチーム間で分割する
</h3>

複数のチームが 1 つのポリシーの一部を所有している場合、各部分を `managed-settings.d/` 内の独自のファイルに配置します。同じシステムディレクトリ内の `managed-settings.json` の隣に配置し、1 つの共有ファイルを編集する代わりに使用します。

Claude Code は `managed-settings.json` を最初にマージし、次にディレクトリ内のすべての `*.json` ファイルをアルファベット順にマージします。ファイルに数値プレフィックスを付けて順序を制御します。例えば `10-telemetry.json` と `20-security.json`。Claude Code は隠しファイルと `.json` で終わらないファイルを無視します。

2 つのファイルが同じキーを設定する場合、Claude Code はこれらのルールで組み合わせます。

* **単一値**（`"model": "opus"` または `"cleanupPeriodDays": 7` など）: 後のファイルの値が前のファイルを置き換えます
* **リスト**（`permissions.deny` または `sandbox.network.allowedDomains` など）: 2 つのリストが組み合わされ、重複が削除されます
* **ネストされたブロック**（`env` または `sandbox` など）: 2 つのブロックはキーごとにマージされ、内部の各キーはこれらの同じルールに従います
* **`fallbackModel`**: 後のチェーンが前のチェーン全体を置き換えます
* **[`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces) および [`managedMcpServers`](/docs/ja/settings-reference#managedmcpservers)**: 同じ名前の後のエントリが前のエントリ全体を置き換えます
* **[`modelPicker`](/docs/ja/settings-reference#modelpicker)**: 後のラインアップが前のラインアップ全体を置き換えます

<span id="precedence-within-the-managed-tier" />

<span id="which-managed-source-claude-code-uses" />

<h2 id="how-claude-code-combines-managed-sources">
  Claude Code が管理されたソースを組み合わせる方法
</h2>

組織が同じマシンに複数の管理されたソースを配信する場合、[`managedSourcesBehavior`](/docs/ja/settings-reference#managedsourcesbehavior) キーは Claude Code が他のソースに対して何を行うかを決定します。

* **`"first-wins"`、デフォルト**: Claude Code は、少なくとも 1 つのポリシーキーを配信する最も高いランクのソースを使用し、[すべての管理者ソースから読み取られるキー](#keys-read-from-every-admin-source)内のキーを除いて、残りを無視します。Claude Code はスキップするソースに対して警告を表示しません。`/status` は[使用したソースとスキップしたソースを名前で示します](#read-the-source-in-/status)。
* **`"merge"`**: Claude Code はポリシーキーを配信するすべての管理者ソースを適用し、キーの種類ごとに組み合わせます。ほとんどのキーでは高いランクのソースの値が適用され、リストは和集合、ロックは最も厳しい値を取ります。[すべての管理されたソースを構成する](#compose-every-managed-source)では、キーを設定する場所と各種類のキーがどのように組み合わされるかを説明しています。Claude Code v2.1.242 以降が必要です。

両方の設定はソースを同じ方法でランク付けします。これらの用語はこのセクション全体で繰り返されます。

* **ポリシーキー**: 2 つの制御キー（[`wslInheritsWindowsSettings`](/docs/ja/settings-reference#wslinheritswindowssettings) と [`managedSourcesBehavior`](/docs/ja/settings-reference#managedsourcesbehavior)）以外のすべての設定キー。これらのキーのみを含む管理設定ファイルまたは MDM ポリシーはカウントされず、Claude Code は次のソースに進みます。
* **管理者ソース**: 以下の最初の 3 つのソースのいずれか。HKCU レジストリはユーザーが書き込み可能であり、管理者ソースではありません。

Claude Code は、最初に最優先度の順でソースをチェックします。

1. リモート設定。claude.ai から[サーバー管理設定](/docs/ja/server-managed-settings)として、または[Claude アプリゲートウェイ](/docs/ja/claude-apps-gateway)によって配信されます。Claude Code はこのソースをフェッチするのは、セッションが[適格なログインまたはキー](/docs/ja/server-managed-settings#platform-availability)で Anthropic の API に直接認証するか、`/login` でゲートウェイにサインインする場合のみです。他のプロバイダーの場合、または `ANTHROPIC_BASE_URL` が Anthropic の API 以外を指す場合、次のソースから開始します。
2. MDM または OS レベルのポリシー: macOS plist または HKLM レジストリキー
3. 管理設定ファイル、`managed-settings.d/*.json` と `managed-settings.json` をマージしたもの
4. Windows 上の HKCU レジストリ、および WSL 上で HKLM レジストリまたは Windows 管理設定ファイルが [`wslInheritsWindowsSettings`](/docs/ja/settings-reference#wslinheritswindowssettings) をオンにし、HKCU 値もそれを設定する場合。Claude Code はそれより上のソースがポリシーキーを配信せず、[ホスト提供の親設定](#let-an-embedding-host-add-policy)が制限的なキーを提供しない場合にのみ読み取ります。

このダイアグラムはランク付けを示し、いずれかの設定の下で Claude Code が最初の 3 つのソースから読み取るクロスソースキーの例を示しています。

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=53f6be49f06eff48e01422c8ae1bc2e6" className="dark:hidden" alt="リモート設定から上部を通じて MDM、管理設定ファイル、および下部の HKCU レジストリにランク付けされた 4 つの管理設定ソースを示すダイアグラム。デフォルトではポリシーキーを持つ最初のソースがポリシーを提供し、残りはスキップされます。managedSourcesBehavior をマージに設定すると、ポリシーキーを持つすべての管理者ソースが寄与し、キーの種類ごとに組み合わされ、HKCU レジストリは除外されます。サイドパネルは、サンドボックスロック、forceRemoteSettingsRefresh、変数ごとの env マージなどのクロスソースキーが、HKCU レジストリを除外するすべての管理者ソースから読み取られることを示しています。" width="680" height="330" data-path="images/managed-source-precedence.svg" />

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence-dark.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=ae407a9a08a3d680e80cf1a2af845d71" className="hidden dark:block" alt="リモート設定から上部を通じて MDM、管理設定ファイル、および下部の HKCU レジストリにランク付けされた 4 つの管理設定ソースを示すダイアグラム。デフォルトではポリシーキーを持つ最初のソースがポリシーを提供し、残りはスキップされます。managedSourcesBehavior をマージに設定すると、ポリシーキーを持つすべての管理者ソースが寄与し、キーの種類ごとに組み合わされ、HKCU レジストリは除外されます。サイドパネルは、サンドボックスロック、forceRemoteSettingsRefresh、変数ごとの env マージなどのクロスソースキーが、HKCU レジストリを除外するすべての管理者ソースから読み取られることを示しています。" width="680" height="330" data-path="images/managed-source-precedence-dark.svg" />

<h3 id="keys-read-from-every-admin-source">
  すべての管理者ソースから読み取られるキー
</h3>

デフォルトの `"first-wins"` 設定では、Claude Code はほとんどのキーを[選択したソース](#how-claude-code-combines-managed-sources)からのみ読み取り、選択したソースがそのキーを設定しないままにしている場合でも、低いランクのソースの値を無視します。

いくつかのキーは異なる動作をします。Claude Code はそれらをすべての管理者ソースから読み取るため、選択したソースが設定しない場合でも、低いランクの MDM ポリシーまたは管理設定ファイルがそれらを設定できます。Claude Code はユーザーが書き込み可能な HKCU レジストリをそのスキャンから除外します。HKCU が唯一のソースであり、ホストが親設定を提供しない場合、HKCU は選択されたソースのように適用されます。

クロスソースキーには以下が含まれます。

* `sandbox.network.allowManagedDomainsOnly` と `sandbox.filesystem.allowManagedReadPathsOnly`: いずれかの管理者ソースの `true` がロックをオンにします。ロックがオンの間、Claude Code はロックするアロウリスト `sandbox.network.allowedDomains` を `WebFetch(domain:...)` 許可ルールと一緒に、または `sandbox.filesystem.allowRead` をすべての管理者ソース全体で和集合にします。ロックがない場合、Claude Code はアロウリストを他のキーのように扱うため、`"first-wins"` では選択されていない管理者ソースのアロウリストは無視されます。
* `allowAllClaudeAiMcps`
* `allowManagedMcpServersOnly`: いずれかの管理者ソースの `true` が MCP アロウリストロックをオンにします。ロックがオンの間、管理された `allowedMcpServers` リストは 1 つを設定する最も高いランクの管理者ソースから来ます。サーバー管理リストは低いソースのリストと組み合わせるのではなく、置き換えます。

  管理者ソースがリストを設定しない場合、[親設定](#let-an-embedding-host-add-policy)がリストを提供しない限り、すべてのサーバーがデニーリストを通過してロードされます。

  ロックがない場合、Claude Code は適用する管理ソースから `allowedMcpServers` を読み取るため、`"first-wins"` では選択されていない管理者ソースのリストは無視されます。Claude Code v2.1.273 以降が必要です。
* `deniedMcpServers` と [`disableClaudeAiConnectors`](/docs/ja/settings-reference#disableclaudeaiconnectors): いずれかの管理者ソースのエントリまたは `true` が適用されます。Claude Code v2.1.273 以降が必要です。
* サンドボックスバイナリパス `sandbox.bwrapPath` と `sandbox.socatPath`
* サンドボックス `ripgrep` バイナリ、[`sandbox.ripgrep`](/docs/ja/settings-reference#sandbox-ripgrep)
* `sandbox.filesystem.disabled` と `sandbox.network.strictAllowlist`
* [`useAutoModeDuringPlan`](/docs/ja/settings-reference#useautomodeduringplan)、[`syncClaudeAiSkills`](/docs/ja/settings-reference#syncclaudeaiskills)、および [`syncClaudeAiPlugins`](/docs/ja/settings-reference#syncclaudeaiplugins)。いずれかの管理者ソースの `false` が動作をオフにします。開発者のユーザーまたはローカル設定の `false` もそれをオフにします。各キーは拒否のみができます。
* [`enableArtifact`](/docs/ja/settings-reference#enableartifact)。いずれかの管理者ソースの `false` が[Artifact ツール](/docs/ja/artifacts)をオフにします。開発者のユーザー、プロジェクト、またはローカル設定の `false` もそれをオフにし、ソースはそれをオンに戻しません。[どの下位レベルの値がまだカウントされるか](/docs/ja/settings#exceptions-to-managed-settings-precedence)を参照してください。Claude Code v2.1.242 以降が必要です。
* [`maxEffortLevel`](/docs/ja/settings-reference#maxeffortlevel)。いずれかの管理者ソースの最も低いキャップが適用されます。開発者が自分の設定または `--settings` で低いキャップを設定する場合、Claude Code はそれを適用します。ソースはキャップを上げることはできません。Claude Code v2.1.267 以降が必要です。
* `attribution` のコミットトレーラーオプトアウト、または非推奨の `includeCoAuthoredBy` から、任意のティア
* [`forceRemoteSettingsRefresh`](/docs/ja/server-managed-settings)
* `env`。管理者ソース全体で変数ごとにマージされます。各変数は、それを定義する最も高い優先度のソースから来るため、低いソースは高いソースが設定しないままにしている変数を埋めます。いくつかの変数は独自のルールに従います。[管理されたソース全体のキーごとの例外](/docs/ja/server-managed-settings#per-key-exceptions-across-managed-sources)は各変数を名前で示しています。Claude Code v2.1.223 以降が必要です。v2.1.223 より前では、Claude Code は選択されたソースの全体 `env` ブロックのみを適用しました。

[ゲートウェイログインキー](#choose-a-delivery-mechanism)は別のルールに従います。Claude Code はサーバー管理設定からそれらを読み取ることはないため、サーバー管理設定が選択されたソースである間、ポリシーキーを持つマシン上の最も高いランクの管理者ソースがそれらを提供します。それより下にランク付けされた管理者ソースの値、または HKCU レジストリの値は無視されます。

管理者ソースが `allowManagedMcpServersOnly` を設定するか、`allowedMcpServers` リストを設定し、その値が実行中でない場合、`/status` と `claude doctor` はそのソースとキーを名前で示します。

<h3 id="compose-every-managed-source">
  すべての管理されたソースを構成する
</h3>

組織が配信するすべての管理者ソースを Claude Code に適用させるには、デプロイする最も高いランクのソースで [`managedSourcesBehavior`](/docs/ja/settings-reference#managedsourcesbehavior) を `"merge"` に設定します。Claude Code はキーまたはポリシーキーを持つ最も高いランクのソースからのみキーを読み取るため、低いソースはそれ自体をそれより上のソースとのマージにオプトインすることはできず、サーバー管理設定を受け取らないマシンはその MDM プロファイルにもキーが必要です。ユーザーが書き込み可能な HKCU レジストリは別のソースとマージされることはありません。Claude Code v2.1.242 以降が必要です。

`"merge"` では、Claude Code は低いソースのリストエントリ（`permissions.allow` ルールやフックなど）をポリシーに追加するため、最も高いランクより下にランク付けされたすべてのソースが管理者の制御下にある場合にのみオンにしてください。

この表は、`"merge"` の下で Claude Code が各種類のキーをどのように組み合わせるかを示しています。[`managedSourcesBehavior` エントリ](/docs/ja/settings-reference#managedsourcesbehavior)は 3 つの行のすべてのキーを名前で示しています。制限アロウリスト、全体で取得される値、および最も高いランクのソースからのみ読み取られるキー。

| キーの種類                   | Claude Code がそれを組み合わせる方法                                                                           | 例                                                                                                                |
| :---------------------- | :------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| リスト                     | すべてのソースからのエントリを組み合わせます                                                                             | `permissions.allow`、`hooks`、`sandbox.network.allowedDomains`、`deniedMcpServers`                                  |
| ロック                     | いずれかのソースが設定する最も厳しい値を適用します。より緩い値は最も高いランクのソースからのみ適用されます                                              | `allowManagedHooksOnly`、`permissions.disableBypassPermissionsMode`、`crossSessionInbound`                         |
| 制限アロウリスト                | それを設定する最も高いランクのソースから全体でリストを取得し、低いソースからエントリを追加しません                                                  | `availableModels`、`allowedMcpServers`、`strictKnownMarketplaces`、`allowedChannelPlugins`、および `fallbackModel` チェーン |
| 全体で取得される値               | それを設定する最も高いランクのソースから全体で値を取得し、低いソースからエントリまたはフィールドを組み合わせません                                          | `sandbox.credentials.awsPairs`、`sandbox.ripgrep`                                                                 |
| 提供される MCP サーバー          | すべてのソースからサーバー名を組み合わせます。2 つのソースが同じ名前を設定する場合、最も高いランクのソースの全体エントリを適用します                                | `managedMcpServers`                                                                                              |
| 最も高いランクのソースからのみ読み取られるキー | 最も高いランクのソースがそれを設定しないままにしている場合でも、すべての低いソースのキーを無視します                                                 | `apiKeyHelper` などの認証情報ヘルパー、`forceLoginOrgUUID` などのログインピン、`modelPicker`、`permissions.defaultMode`                 |
| `env`                   | [すべての管理者ソースから読み取られるキー](#keys-read-from-every-admin-source)が説明するように、いずれかの設定の下で管理者ソース全体で変数ごとにマージされます |                                                                                                                  |
| その他のすべてのキー              | それを設定する最も高いランクのソースから値を取得します                                                                        | `model`、`cleanupPeriodDays`                                                                                      |

マシンで組み合わされたソースを確認するには、[`/status` の `Setting sources` 行を読み取ります](#read-the-source-in-/status)。そのセクションは各ラベルが何を意味するかを説明しています。

<h3 id="compute-the-policy-with-a-helper-program">
  ヘルパープログラムでポリシーを計算する
</h3>

[`policyHelper`](/docs/ja/settings-reference#policyhelper) は MDM ポリシーまたは管理設定ファイルが名前を付ける実行可能ファイルであり、Claude Code はそれを実行してスタートアップで管理設定を計算します。選択されたソースが 1 つを構成し、ヘルパーが `managedSettings` オブジェクトを出力する場合、その出力は Claude Code が読み取るものを変更します。

* **出力された `managedSettings` オブジェクトはセッションの唯一の管理設定です**。[それ以外の場合はすべての管理者ソースから読み取られるキー](#keys-read-from-every-admin-source)を含めて、[`forceRemoteSettingsRefresh` を除いて、それは独自のスタートアップルールを持っています](/docs/ja/settings-reference#forceremotesettingsrefresh)。

ヘルパー実行が失敗する場合、および 1 つが失敗したときに Claude Code が何を行うかについては、[ヘルパー失敗](/docs/ja/settings-reference#helper-failures)を参照してください。

<span id="parent-settings-from-embedding-hosts" />

<span id="control-policy-from-an-embedding-host" />

<span id="merge-policy-from-an-embedding-host" />

<h3 id="let-an-embedding-host-add-policy">
  埋め込みホストがポリシーを追加できるようにする
</h3>

Claude Desktop、IDE 拡張機能、または Agent SDK アプリなど、別のアプリケーションが Claude Code を起動する場合、そのホストは SDK `managedSettings` オプションを通じて独自の管理設定を渡すことができます。Claude Code はこれらを親設定と呼びます。

デフォルトでは、Claude Code は管理者ソースが存在する場合、親設定を無視します。サーバー管理設定、MDM または OS レベルのポリシー、または管理設定ファイル。

Claude Code が親設定を管理者ソースと一緒にマージするには、最も高い優先度の管理ソースで [`parentSettingsBehavior`](/docs/ja/settings-reference#parentsettingsbehavior) を `"merge"` に設定します。Claude Code はそのソースからのみキーを読み取ります。

Claude Code はホストの値のうち、Claude ができることを制限するものだけを保持します。知っておくべき 1 つのギャップがあります。`allowManaged*Only` ロックも設定しない限り、ホストの権限許可ルールとサンドボックスアロウリストはまだ適用されます。[親設定を制限する](/docs/ja/claude-apps-gateway#restrict-parent-settings)でロックを参照してください。

[`policyHelper`](/docs/ja/settings-reference#policyhelper) はこのキーに関係なく親マージをオフにすることができます。そのエントリは時期を説明しています。

Claude Code はこれらのチェックを親提供の値に独立して適用します。

* いずれかの管理者ソースが `allowManagedPermissionRulesOnly` を設定する場合、Claude Code は[親提供の](/docs/ja/claude-apps-gateway#restrict-parent-settings)権限許可ルールと `additionalDirectories` を読み取るときにドロップします。高い優先度のソースがキーを設定しないままにしている場合でも。キーの効果は、Claude Code が適用する管理設定、または親設定からマージすることを選択したものから来ます。
* Claude Code は管理設定で `forceLoginOrgUUID` または `allowedMcpServers` 値を適用し、親提供のものをブロックします。MCP アロウリストロックの外では、Claude Code が適用しない低い管理者ソースの値は適用されず、親のものもブロックされません。

  Claude Code v2.1.273 以降では、`allowManagedMcpServersOnly` がオンの間、1 つを設定する最も高いランクの管理者ソースからの `allowedMcpServers` リストが適用され、親のものをブロックします。[クロスソースキー](#keys-read-from-every-admin-source)として。親のリストは管理者ソースが 1 つを設定しない場合にのみ適用されます。[`managedSourcesBehavior`](/docs/ja/settings-reference#managedsourcesbehavior) エントリは `"merge"` の下で各キーを提供するソースを説明しています。v2.1.223 より前では、いずれかの管理者ソースの値が親のものをブロックしました。
* `availableModels` の場合、Claude Code は管理設定で値を適用し、親提供のリストをブロックします。
* `strictKnownMarketplaces` の場合、Claude Code は同様に管理設定で値を適用し、親提供のものをブロックします。親のリストは適用された管理ソースが 1 つを設定しない場合にのみ適用されます。Claude Code v2.1.282 以降が必要です。
* 親提供の `blockedMarketplaces` は管理ソースが設定するブロックリストに加えて適用されます。Claude Code v2.1.282 以降が必要です。

<h4 id="keep-cowork-folder-access-when-only-managed-rules-apply">
  管理されたルールのみが適用される場合に Cowork フォルダアクセスを保持する
</h4>

Claude Desktop アプリの [Cowork](https://claude.com/docs/cowork/overview) はそのセッションを Claude Code で実行し、各セッションに接続されたフォルダなどの作業フォルダへのアクセスを許可ルールを通じて付与します。セッションを起動するときに提供します。管理されたポリシーが [`allowManagedPermissionRulesOnly`](/docs/ja/settings-reference#allowmanagedpermissionrulesonly) を設定する場合、Claude Code は管理ポリシーの許可ルールのみを保持します。ホストが親設定として、`--allowedTools` として、または設定ファイルで提供する許可ルールをドロップするため、これらのフォルダへの書き込みは事前承認を失います。Cowork セッションで編集前に尋ねる場合、Cowork はプロンプトを表示できず、Claude は各書き込みをブロックされたものとして報告します。パスが保護された場所に解決されるか、接続されたフォルダの外のパスであるため。

書き込みを復元するには、Claude Code が[選択する](#precedence-within-the-managed-tier)管理ソースのそれらのフォルダに許可ルールを追加します。これらのマシン上で。MDM 管理フリートでは、それは別の管理設定ファイルではなく MDM ポリシーです。この例はファイル形式を使用し、MDM ポリシーは同じキーを取ります。`allowManagedPermissionRulesOnly` を設定したままにし、各ユーザーのホームディレクトリの `CoworkProjects` フォルダの下での編集を許可します。パスをユーザーが接続するフォルダに置き換えます。

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true,
  "permissions": {
    "allow": [
      "Edit(~/CoworkProjects/**)"
    ]
  }
}
```

ポリシーをデプロイした後、Claude は新しい Cowork セッションでそのフォルダの下にファイルを保存できます。[ルールの読み取りと編集](/docs/ja/permissions#read-and-edit)はパス構文をカバーしています。`//` 形式を含む絶対パス。

<h3 id="what-a-developer-can-change">
  開発者が変更できるもの
</h3>

開発者自身の設定ファイル、`--settings` 値、およびプロジェクトファイルは管理値をオーバーライドしません。[例外](/docs/ja/settings#exceptions-to-managed-settings-precedence)は、より厳しい下位レベルの値がカウントされることのみを許可します。これらのケースはそのルールの外にあります。

* **セッションのモデル**: 管理された `model` はロックではなくデフォルトです。`--model` と `ANTHROPIC_MODEL` はそのセッションのモデルを選択できるため、[`availableModels`](/docs/ja/settings-reference#availablemodels) をデプロイして選択を制限します。
* **ローカル管理者権限**: マシンの管理者である開発者は管理ソース自体を編集できます。これが MDM ツールがプロファイルまたはファイルをスケジュールで再デプロイできる理由であり、HKLM レジストリと macOS 管理設定ドメインが存在する理由です。
* **サーバー管理キャッシュ**: サーバー管理設定は Anthropic のサーバーから来ており、ローカルキャッシュへの編集は[次の成功したフェッチまでのみ続きます](/docs/ja/server-managed-settings#security-considerations)。
* **その他のツール**: 管理設定は Claude Code のみをバインドします。別のツールから API を呼び出す開発者はそれらの下にはありません。

<span id="verify-enforcement" />

<span id="verify-that-a-policy-is-in-force" />

<h2 id="check-that-a-policy-is-in-force">
  ポリシーが有効であることを確認する
</h2>

開発者がポリシーが適用されていないと報告している場合、またはロールアウトがフリートへのプッシュ前に完了したことを確認したい場合があります。そのマシン上の 2 つのコマンドがこれに答えます。`/status` は Claude Code が選択した管理対象ソースを表示し、`claude doctor` はドロップしたものをリストします。

<h3 id="read-the-source-in-/status">
  /status でソースを読む
</h3>

開発者のマシンで Claude Code 内で `/status` を実行し、`Setting sources` 行を読みます。管理対象ソースが有効な場合、その行は `Enterprise managed settings` をリストし、括弧内に Claude Code が選択したソースを表示します。

* `(remote)`：claude.ai またはゲートウェイからのサーバー管理設定
* `(plist)` または `(HKLM)`：MDM または OS ポリシー
* `(file)`、`(drop-ins)`、または `(file + drop-ins)`：`managed-settings.json`、ドロップイン ディレクトリ、またはその両方
* `(remote + file, merged)` または別のリスト（`, merged` で終わる）：組織が[すべての管理対象ソースを構成](#compose-every-managed-source)し、Claude Code がリストされたソースをポリシーにマージしました。下位のソースは、リストに表示されなくても `env` 変数を提供できます。Claude Code v2.1.242 以降が必要です
* `(HKCU)`：ユーザー書き込み可能なレジストリ フォールバック
* `(parent process)`：[埋め込みホスト](#let-an-embedding-host-add-policy)が制限的な設定を提供しました
* `(helper)`：選択した MDM またはファイル ソースによって構成された [`policyHelper`](/docs/ja/settings-reference#policyhelper)

Claude Code がマシン上で管理対象ソースを見つけたが選択しなかった場合、2 番目の行 `Skipped sources` が各ソースを名前で示します。これを読んで、ポリシーがマシンに到達しなかった場合と、到達したが高優先度のソースがオーバーライドした場合を区別します。Claude Code v2.1.242 以降が必要です。

ポリシーが適用されていない場合、`Setting sources` 行は 2 つの問題のどちらがあるかを示します。

* **行が見つかりません**：Claude Code はポリシー キーを配信する管理対象ソースを見つけませんでした。

  管理対象設定ファイルをデプロイした場合、OS のパスに配置されていることを確認し、制御キーのみではなく[ポリシー キー](#how-claude-code-combines-managed-sources)が含まれていることを確認します。有効な JSON ではないファイルはこの状態を生成しません。Claude Code は代わりに[起動を拒否](#find-entries-claude-code-dropped)します。

  代わりにサーバー管理設定を通じてデプロイした場合は、`claude doctor` を実行します。これは[フェッチ結果](/docs/ja/server-managed-settings#verify-settings-delivery)を報告します。
* **行が展開したソース以外のソースを名前で示す**：高優先度のソースが存在し、Claude Code があなたのソースを無視しました。`Skipped sources` がそれをリストします。[Claude Code が管理対象ソースを組み合わせる方法](#how-claude-code-combines-managed-sources)は順序を示します。

<span id="invalid-entries-in-managed-settings" />

<h3 id="find-entries-claude-code-dropped">
  Claude Code がドロップしたエントリを見つける
</h3>

管理対象設定ファイル、MDM プロファイル、レジストリ値、またはサーバー管理ペイロードがスキーマ検証に失敗した場合、Claude Code は最初に修復できる個別エントリ（無効なパーミッション ルールなど）をスキップし、各エントリに対して警告を表示してから、値がまだ失敗する最上位キーをドロップし、残りのすべての有効なキーの適用を続けます。

Claude Code は [`policyHelper`](/docs/ja/settings-reference#policyhelper) が出力する `managedSettings` に対してより厳密です。同じエントリ修復を行いますが、生き残るスキーマ違反は全体のヘルパー実行を失敗させ、起動時に Claude Code は起動を拒否します。これは非ゼロで終了するヘルパーと同じです。

管理対象設定ファイル、ドロップイン ファイル、MDM plist、または HKLM レジストリ値が存在するが JSON オブジェクトとして解析できない場合、Claude Code は起動を拒否し、別の管理者ソースが有効なポリシーを配信する場合でも[ソースを名前で示すエラー](/docs/ja/errors#managed-settings-document-could-not-be-parsed)を出力します。各ソースは次の場合にこのように失敗します。

* **管理対象設定ファイルまたはドロップイン ファイル**：ファイルが有効な JSON ではない、またはその最上位がオブジェクトではない
* **MDM plist**：macOS の `plutil` が plist が不正形式であると報告する、またはその変換されたコンテンツが JSON オブジェクトではない
* **HKLM レジストリ値**：`Settings` 値が文字列ではない、空である、または JSON オブジェクトを保持していない

3 つのソース状態はこの拒否を引き起こしません。

* 不在のファイル、プロファイル、またはレジストリ値は失敗ではありません。Claude Code はそのソースなしで実行されます。
* 空の管理対象設定ファイルは `{}` としてカウントされます。
* ユーザー書き込み可能な HKCU レジストリ キーの不正形式の値は起動をブロックしません。Claude Code は代わりに `/status` と `claude doctor` で通知として報告します。

管理対象設定ファイル、ドロップイン ファイル、または `managed-settings.d/` ディレクトリを読み取ることができず、管理者ソースがポリシーを提供しない場合、claude.ai または Claude Console 認証情報でサインインしたセッションは管理者に連絡するメッセージで起動時に終了します。

ドロップされたエントリを見つけるには、3 つの場所のいずれかを確認します。

* インタラクティブ セッションは起動時に無効なエントリをリストするダイアログを表示します。
* `-p` を使用した非インタラクティブ実行は stderr に概要を出力します。
* [`claude doctor`](/docs/ja/debug-your-config) は各無効なエントリをそのソースとフィールドでリストします。

<h4 id="keys-that-fail-closed">
  閉じた状態で失敗するキー
</h4>

無効な場合にドロップされない強制キーがいくつかあります。Claude Code は値が修正されるまでより厳密なフォールバックを適用します。テーブルは各キーに対して適用されるものを示します。

| フィールド                         | 存在するが無効な場合の動作                                                                                                                                                                                                                                                                 |
| :---------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`           | ユーザーが追加する MCP サーバーが許可されないように、値が修正されるまで空のアローリストとして適用されます。組織が [`managedMcpServers`](/docs/ja/settings-reference#managedmcpservers) を通じて配信するサーバーは引き続きロードされ、`managed-mcp.json` サーバーは[サーバーの評価方法](/docs/ja/managed-mcp#how-a-server-is-evaluated)に従ってロードされます。個別の無効なエントリは削除され、有効なサブセットが適用されます。 |
| `allowedHttpHookUrls`         | Claude Code は値を修正するまで空の管理[アローリスト](/docs/ja/settings-reference#allowedhttphookurls)を適用するため、HTTP フックは別の設定ファイルがその URL をリストしている場合にのみ実行されます。無効なエントリが 1 つだけの場合、Claude Code はそのエントリを削除し、残りを適用します。                                                                                         |
| `httpHookAllowedEnvVars`      | Claude Code は値を修正するまで空の管理[アローリスト](/docs/ja/settings-reference#httphookallowedenvvars)を適用するため、ヘッダー変数は別の設定ファイルがそれを名前で示している場合にのみ補間されます。無効なエントリが 1 つだけの場合、Claude Code はそのエントリを削除し、残りを適用します。                                                                                            |
| `allowedChannelPlugins`       | 値を修正するまで空のアローリストとして適用されるため、`--channels` に渡されるチャネル プラグインは許可されません。無効なエントリが 1 つだけの場合、それを削除し、残りを適用します。                                                                                                                                                                            |
| `strictKnownMarketplaces`     | 値が修正されるまで空のアローリストとして適用されるため、[マーケットプレイス ソース](/docs/ja/plugins/org#restrict-what-users-can-install)は許可されません。無効なエントリ、または `hostPattern` 正規表現がコンパイルされないなど適用できないエントリは削除され、有効なサブセットが適用されます。                                                                                               |
| `allowManagedHooksOnly`       | 修正されるまで `true` として扱われます。[フック制限](/docs/ja/settings-reference#allowmanagedhooksonly)が適用され、`disableCommandPluginSources` が明示的に `false` でない限り、コマンドソースのプラグインは無効になります。                                                                                                                   |
| `allowManagedMcpServersOnly`  | `true` として扱われます。                                                                                                                                                                                                                                                              |
| `disableCommandPluginSources` | `true` として扱われるため、値が修正されるまでコマンドソースのプラグインは無効のままです。                                                                                                                                                                                                                              |
| `disableSideloadFlags`        | 値が修正されるまで `true` として扱われ、[`disableSideloadFlags`](/docs/ja/settings-reference#disablesideloadflags) にリストされた効果があります。                                                                                                                                                                 |
| `availableModels`             | 修正されるまで空のアローリストとして適用されるため、デフォルト モデルのみが利用可能です。文字列以外のエントリは削除され、有効なサブセットが適用されます。                                                                                                                                                                                                 |
| `enforceAvailableModels`      | `true` として扱われます。                                                                                                                                                                                                                                                              |
| `syncClaudeAiPlugins`         | `false` として扱われるため、[claude.ai プラグイン](/docs/ja/settings-reference#syncclaudeaiplugins)の同期は値が修正されるまでオフになります。                                                                                                                                                                          |
| `forceLoginOrgUUID`           | 値が修正されるまで、組織がログインすることは許可されません。                                                                                                                                                                                                                                                |
| `gatewayInternalNetworks`     | 無効な値が最も高い管理対象ソースから来ている場合、そのマシン上の `/login` は値が修正されるまで、すべての新しい[クラウド ゲートウェイ](/docs/ja/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)サインインを拒否します。                                                                                                                |
| `crossSessionInbound`         | 最も制限的な値である `refuse` として扱われるため、値が修正されるまで[クロスセッション メッセージ](/docs/ja/cross-session-messaging#control-inbound-messages)のインバウンドは拒否されます。開発者は[警告](/docs/ja/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse)を見ます。                                                                   |
| `deniedMcpServers`            | 個別の無効なエントリは削除され、有効なサブセットが適用されます。完全に無効な値は警告とともにドロップされます。すべてのサーバーを拒否するとポリシーが名前を付けなかったサーバーがブロックされるためです。                                                                                                                                                                          |
| `blockedMarketplaces`         | 個別の無効なエントリは削除され、有効なサブセットが適用されます。`hostPattern` 正規表現がコンパイルされないなど、解析されるが決してマッチしないエントリは警告とともに保持されます。修正されるまでは何もブロックしませんが、[マーケットプレイス制限](/docs/ja/plugins/org#restrict-what-users-can-install)は有効なままです。完全に無効な値は警告とともにドロップされます。すべてのマーケットプレイスをブロックするとポリシーが名前を付けなかったソースがブロックされるためです。         |
| `sandbox.credentials`         | 回復可能な無効なエントリは `mode: "deny"` に低下し、警告が表示されます。回復不可能なエントリは削除されます。有効なエントリは適用されたままです。[管理対象設定の無効な認証情報エントリ](/docs/ja/settings-reference#invalid-credential-entries-in-managed-settings)を参照してください                                                                                          |

`allowedHttpHookUrls` と `httpHookAllowedEnvVars` は設定ファイル全体でマージされるため、管理対象リストが空の間、ユーザー、プロジェクト、またはローカル設定のエントリは引き続き適用されます。

これら 2 つのキーと `allowedChannelPlugins` のフォールバックには Claude Code v2.1.267 以降が必要です。以前のバージョンは、値またはエントリが無効な場合、キー全体をドロップします。`strictKnownMarketplaces`、`blockedMarketplaces`、および `disableSideloadFlags` のフォールバックには Claude Code v2.1.277 以降が必要です。以前のバージョンは、値またはエントリが無効な場合、キー全体をドロップします。

`requiredMinimumVersion` と `requiredMaximumVersion` は設計上オープンに失敗します。無効な値は適用されるのではなくドロップされます。

この許容度は管理対象設定にのみ適用されます。ユーザー、プロジェクト、およびローカル設定ファイルは厳密なままです。JSON またはトップレベルの形状が検証に失敗するファイルは全体として拒否され、報告されます。不正形式のパーミッション ルールなどの個別エントリが失敗する場合は、警告とともにスキップされ、ファイルの残りが適用されます。

<span id="managed-only-settings" />

<h2 id="keys-only-a-managed-source-can-set">
  マネージドソースのみが設定できるキー
</h2>

Claude Code は次のキーをマネージドソースからのみ読み取ります。ユーザーまたはプロジェクト設定ファイルに配置しても効果がありません。

ほとんどはロックです。ロックが管理するキー（権限ルールまたは `sandbox.network.allowedDomains` など）は、任意のレベルが設定できる通常のキーであり、ロックは Claude Code にマネージド値のみを尊重するよう指示します。

テーブルは権限、プラグイン、配信コントロールをカバーしています。ここにリストされていないキーについては、[設定リファレンス](/docs/ja/settings-reference#all-settings) インデックスの Scope 列は、それがマネージドのみであるかどうかを示しています。残りのマネージドのみキーには、ゲートウェイログイン URL、バージョン、ブラウザ、モバイルシミュレーター、SSH ホスト、Desktop ローカルセッション、サンドボックスバイナリパス、モデル価格、CLAUDE.md コントロールが含まれます。

| 設定                                                                                                                    | 説明                                                                                                                                                                                                                                                                                                                                                                                |
| :-------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`allowAllClaudeAiMcps`](/docs/ja/settings-reference#allowallclaudeaimcps)                                                 | Claude Code が自身でフェッチする claude.ai コネクタをデプロイされた `managed-mcp.json` と一緒にロードします。それらを抑制する代わりに                                                                                                                                                                                                                                                                                          |
| [`allowedChannelPlugins`](/docs/ja/settings-reference#allowedchannelplugins)                                               | メッセージをプッシュできるチャネルプラグインの許可リスト。設定されている場合、デフォルト Anthropic 許可リストを置き換えます。`channelsEnabled: true` が必要です。[実行できるチャネルプラグインを制限する](/docs/ja/channels#restrict-which-channel-plugins-can-run) を参照してください                                                                                                                                                                                            |
| [`allowManagedHooksOnly`](/docs/ja/settings-reference#allowmanagedhooksonly)                                               | `true` の場合、実行するフックを制限します。[`allowManagedHooksOnly` の下で実行するもの](/docs/ja/settings-reference#what-runs-under-allowmanagedhooksonly) の完全な効果リストを参照してください                                                                                                                                                                                                                                     |
| [`allowManagedMcpServersOnly`](/docs/ja/settings-reference#allowmanagedmcpserversonly)                                     | `true` の場合、マネージド設定からの `allowedMcpServers` のみが尊重されます。`deniedMcpServers` はすべてのソースからマージされます。[すべての管理ソースから読み取られるキー](#keys-read-from-every-admin-source) でどのマネージドソースがそれを設定できるかを参照し、[マネージド MCP 構成](/docs/ja/managed-mcp) を参照してください                                                                                                                                                            |
| [`allowManagedPermissionRulesOnly`](/docs/ja/settings-reference#allowmanagedpermissionrulesonly)                           | マネージド設定を権限ルールの唯一の設定ソースにします。エントリは無視するすべてのソースをリストします                                                                                                                                                                                                                                                                                                                                |
| [`blockedMarketplaces`](/docs/ja/settings-reference#blockedmarketplaces)                                                   | マーケットプレイスソースのブロックリスト。ブロックされたソースはダウンロード前にチェックされるため、ファイルシステムに触れません。[マネージドマーケットプレイス制限](/docs/ja/plugins/org#restrict-what-users-can-install) を参照してください                                                                                                                                                                                                                                    |
| [`channelsEnabled`](/docs/ja/settings-reference#channelsenabled)                                                           | 組織の [チャネル](/docs/ja/channels) を許可します。各プランのデフォルトについては [エンタープライズコントロール](/docs/ja/channels#enterprise-controls) を参照してください                                                                                                                                                                                                                                                                      |
| [`disableCommandPluginSources`](/docs/ja/settings-reference#disablecommandpluginsources)                                   | `true` の場合、[`command` プラグインソース](/docs/ja/plugins/marketplace-reference#command-plugin-source) を完全にブロックするため、マーケットプレイス宣言コマンドは実行されません。マーケットプレイス [`headersHelper` コマンド](/docs/ja/plugins/host-marketplace#authenticate-archive-downloads) もブロックします。ただし、マネージド設定自体が宣言するマーケットプレイスは除きます。設定されていない場合、`allowManagedHooksOnly` に従います。Claude Code v2.1.229 以降が必要で、`headersHelper` ブロックは v2.1.238 以降が必要です |
| [`disableSideloadFlags`](/docs/ja/settings-reference#disablesideloadflags)                                                 | スタートアップで `--plugin-dir`、`--plugin-url`、`--agents`、および `--mcp-config` フラグを拒否します。クラウドセッションでは、Claude Code はサーバーが `--mcp-config` を通じて配信した MCP サーバーをドロップします。ただし、プロセス内 `type: "sdk"` エントリは除き、セッションを開始します。Claude Code v2.1.193 以降が必要です                                                                                                                                                   |
| [`forceRemoteSettingsRefresh`](/docs/ja/settings-reference#forceremotesettingsrefresh)                                     | `true` の場合、リモートマネージド設定が新しくフェッチされるまで CLI スタートアップをブロックし、フェッチが失敗する場合は終了します。[失敗閉じ強制](/docs/ja/server-managed-settings#enforce-fail-closed-startup) を参照してください                                                                                                                                                                                                                               |
| [`managedMcpServers`](/docs/ja/settings-reference#managedmcpservers)                                                       | すべてのユーザーに独自と一緒に提供されるリモート MCP サーバー。何かをロックするのではなく、サーバーを提供します。[マネージド設定を通じてサーバーを提供する](/docs/ja/managed-mcp#provide-servers-through-managed-settings) を参照してください。Claude Code v2.1.259 以降が必要です                                                                                                                                                                                                |
| [`managedSourcesBehavior`](/docs/ja/settings-reference#managedsourcesbehavior)                                             | Claude Code が最高優先度のマネージドソースのみを適用するか、[それらすべてを構成する](#compose-every-managed-source) か                                                                                                                                                                                                                                                                                                |
| [`parentSettingsBehavior`](/docs/ja/settings-reference#parentsettingsbehavior)                                             | ホスト提供の親設定がマネージドポリシーの下でマージするかどうか                                                                                                                                                                                                                                                                                                                                                   |
| [`pluginSuggestionMarketplaces`](/docs/ja/settings-reference#pluginsuggestionmarketplaces)                                 | Claude Code がユーザーに提案できるプラグインのマーケットプレイス                                                                                                                                                                                                                                                                                                                                            |
| [`pluginTrustMessage`](/docs/ja/settings-reference#plugintrustmessage)                                                     | インストール前に表示されるプラグイン信頼警告に追加されるカスタムメッセージ                                                                                                                                                                                                                                                                                                                                             |
| [`policyHelper`](/docs/ja/settings-reference#policyhelper)                                                                 | スタートアップでマネージド設定を計算する実行可能ファイル。[ポリシーヘルパーでマネージド設定を計算する](/docs/ja/settings-reference#policyhelper) を参照してください                                                                                                                                                                                                                                                                               |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](/docs/ja/settings-reference#sandbox-filesystem-allowmanagedreadpathsonly) | `true` の場合、マネージド設定からの `filesystem.allowRead` パスのみが尊重されます。`denyRead` はすべてのソースからマージされます                                                                                                                                                                                                                                                                                             |
| [`sandbox.network.allowManagedDomainsOnly`](/docs/ja/settings-reference#sandbox-network-allowmanageddomainsonly)           | マネージド `allowedDomains` および `WebFetch(domain:...)` 許可ルールのみを尊重します。プロンプトなしで他のドメインをブロックします                                                                                                                                                                                                                                                                                            |
| [`strictKnownMarketplaces`](/docs/ja/settings-reference#strictknownmarketplaces)                                           | ユーザーが追加してプラグインをインストールできるプラグインマーケットプレイスソースを制御します。[マネージドマーケットプレイス制限](/docs/ja/plugins/org#restrict-what-users-can-install) を参照してください                                                                                                                                                                                                                                                     |
| [`strictPluginOnlyCustomization`](/docs/ja/settings-reference#strictpluginonlycustomization)                               | ユーザーおよびプロジェクトソースからのスキル、エージェント、フック、MCP サーバーをブロックします。`true` はすべて 4 つをロックし、配列はどれかに名前を付けます                                                                                                                                                                                                                                                                                            |
| [`wslInheritsWindowsSettings`](/docs/ja/settings-reference#wslinheritswindowssettings)                                     | `HKLM` レジストリまたは `C:\Program Files\ClaudeCode` の下のファイルに設定されている場合、WSL が Windows ポリシーチェーンを読み取り、そのディレクトリの下の `/etc/claude-code` を読み取るのは、マネージド設定ファイルまたはドロップインが [ポリシーキー](#how-claude-code-combines-managed-sources) を配信しない場合のみです。エントリは順序を示しています                                                                                                                                          |

<Note>
  Team および Enterprise プランでは、Owner は [Claude Code 管理設定](https://claude.ai/admin-settings/claude-code) で [リモートコントロール](/docs/ja/remote-control) および [クラウドセッション](/docs/ja/claude-code-on-the-web) を組織全体で有効または無効にします。リモートコントロールは [`disableRemoteControl`](/docs/ja/settings-reference#disableremotecontrol) 設定でデバイスごとに無効にすることもできます。クラウドセッションにはデバイスごとのマネージド設定キーがありません。

  これらの組織設定が特定のマシンに到達したかどうかを確認するには、そこで `claude doctor` を実行し、`Organization policy` 行を読みます。これは Claude Code がポリシーをロードした場所、またはロードしなかった理由を示しています。Claude Code v2.1.261 以降が必要です。実行中のセッションでは、ポリシーがロードされなかった場合、`/status` は同じ行を表示します。
</Note>

<h2 id="turn-telemetry-off-for-your-organization">
  組織のテレメトリをオフにする
</h2>

Claude Code は、Anthropic API を直接、LLM ゲートウェイを通じて、またはカスタム `ANTHROPIC_BASE_URL` を通じて使用するセッションで、デフォルトで Anthropic 運用 [テレメトリ](/docs/ja/data-usage#telemetry-services) を送信します。[API プロバイダーごとのデフォルト動作](/docs/ja/data-usage#default-behaviors-by-api-provider) はどのプロバイダーがそれを送信するかを示しています。すべての開発者が各人のシェルに依存することなく、マネージド設定の `env` ブロックを通じて `DISABLE_TELEMETRY` を配信することでオフにします。この例は、ポリシーが到達するすべてのユーザーに対して `DISABLE_TELEMETRY` を設定します。

```json theme={null}
{
  "env": {
    "DISABLE_TELEMETRY": "1"
  }
}
```

Claude Code は `1` の値を [承認ダイアログ](/docs/ja/server-managed-settings#environment-variables-and-the-approval-dialog) を表示せずに適用します。

テレメトリをオフにする場合、Claude Code はポリシーが到達する開発者の組織の [分析ダッシュボード](/docs/ja/analytics) を供給する使用データの送信を停止します。変数はフィーチャーフラグフェッチもオフにします。これにより、リモートコントロール、デフォルトオートモード、および他の [フィーチャーフラグフェッチが必要な機能](/docs/ja/env-vars#features-that-need-feature-flag-fetching) がこれらの開発者に利用できなくなります。

[ポリシーが適用される場所と時期](#where-and-when-a-policy-applies) は各サーフェスに到達する配信メカニズムを示し、[プラットフォーム可用性](/docs/ja/server-managed-settings#platform-availability) はどのセッションがサーバーマネージド設定フェッチをスキップするかを示しています。

組織がカスタマー管理暗号化キーを使用し、Claude Code をゲートウェイを通じてルーティングする場合、[プロキシとゲートウェイを構成する](/docs/ja/third-party-integrations#configure-proxies-and-gateways) はこれらのセッションがこの変数を必要とする理由を示しています。

<h2 id="see-also">
  関連項目
</h2>

* [組織向けに Claude Code をセットアップする](/docs/ja/admin-setup): 強制する内容と方法を決定します
* [サーバーマネージド設定](/docs/ja/server-managed-settings): claude.ai コンソールまたはゲートウェイからポリシーを配信します
* [マネージド MCP 構成](/docs/ja/managed-mcp): 開発者が使用できる MCP サーバーを制御します
* [すべての設定](/docs/ja/settings-reference): すべてのキー。マネージドソースがそれを設定できるかどうか
* [設定ファイルの例](/docs/ja/settings-example#an-organizations-managed-settings): マネージドキーの形状を示す完全な `managed-settings.json`
