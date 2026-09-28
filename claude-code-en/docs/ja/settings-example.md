> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 設定ファイルの例

> 開発者、チーム、組織向けの現実的な settings.json ファイル：1 つをコピーして、必要なキーを保持し、値を変更してください。

このページには 3 つの例 `settings.json` ファイルがあります。設定を保存する場所ごとに 1 つずつです：

* 開発者の `~/.claude/settings.json`
* チームの `.claude/settings.json`（リポジトリにコミットされたもの）
* 組織の `managed-settings.json`

それぞれが読者にとって妥当なファイルなので、形状を確認して必要な部分をコピーできます。どれも推奨されるベースラインではありません。すべての値は [設定リファレンス](/docs/ja/settings-reference) のキーのエントリから取得されており、そこには型、デフォルト値、および設定できる場所が記載されています。

各例には 2 つのタブがあります。**コピー可能な設定ファイル** は保存するファイルです。**各キーの機能** は同じファイルですが、各キーの上にコメントがあります。Claude Code は設定ファイルのコメントを受け入れないため、最初のタブからコピーしてください。

<h2 id="your-own-settings">
  独自の設定
</h2>

1 人の開発者の個人設定です。モデルと努力レベルを選択し、ターミナルを調整し、読み取り専用コマンドと 1 つのファイル読み取りを事前承認します。リストされていないすべてのものはデフォルトを保持します。このようなファイルは `~/.claude/settings.json` に保存され、開くすべてのプロジェクトに適用されます。

<Tabs>
  <Tab title="コピー可能な設定ファイル">
    これを `~/.claude/settings.json` として保存してください。コメントのない有効な JSON なので、そのまま貼り付けて、不要なキーを削除できます。

    ```json ~/.claude/settings.json theme={null}
    {
      "model": "claude-sonnet-5",
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      "editorMode": "vim",
      "theme": "light-daltonized",
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      "spinnerTipsEnabled": false,
      "preferredNotifChannel": "terminal_bell",
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      "autoUpdatesChannel": "stable",
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>

  <Tab title="各キーの機能">
    同じファイルですが、各キーの上にコメントがあります。ここで読んでください。Claude Code は設定ファイルのコメントを受け入れないため、別のタブからコピーしてください。

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // すべてのセッションを Sonnet 5 で開始
      "model": "claude-sonnet-5",
      // Sonnet 5 をデフォルトの高レベルより上で実行します。/effort はモデルごとにレベルを保存し、--effort は単一セッションに設定します
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // プロンプトの Vim キーバインディング
      "editorMode": "vim",
      // 色覚異常対応のライトテーマ
      "theme": "light-daltonized",
      // プロンプトの下のステータスライン：モデル名とコンテキスト使用量
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // スピナーの下で回転するヒントを非表示にする
      "spinnerTipsEnabled": false,
      // 完了したタスクや待機中の権限プロンプトなどの通知のためにターミナルベルを鳴らす
      "preferredNotifChannel": "terminal_bell",
      // Claude Code が git diff を実行し、.zshrc を読み取ることを許可する（確認なし）
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // 安定チャネルから更新を取得
      "autoUpdatesChannel": "stable",
      // 20 日以上前のセッショントランスクリプトおよび他のローカルセッションデータを削除
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  チームの共有設定
</h2>

1 つのチームの共有設定は、リポジトリにコミットされるため、それをクローンした全員が同じ権限、hooks、プラグインマーケットプレイスを取得します。リポジトリのトップレベルに `.claude/settings.json` のようなファイルを保存してください。コミットする前に知っておくべきことは以下の通りです。

* **クラウドセッションもこれを読みます。** [クラウドセッション](/docs/ja/settings#settings-in-cloud-sessions)はリポジトリのクローンから開始されるため、コミットされたファイルはそこにも適用されます。
* **テレメトリは管理設定または個人設定に入ります。** Claude Code は、リポジトリの設定ファイルの [OpenTelemetry エクスポーター変数](/docs/ja/settings-reference#variables-claude-code-ignores-in-env)を無視します。ただし、テレメトリをオフにする値は除きます。組織の[管理設定](/docs/ja/monitoring-usage#administrator-configuration)、または各ユーザーの `~/.claude/settings.json` で設定してください。
* **許可ルールは信頼を待ちます。** 許可ルールと `extraKnownMarketplaces` エントリは、各ユーザーが[このフォルダ自体を信頼](/docs/ja/permissions#project-allow-rules-and-workspace-trust)した後に有効になります。親フォルダだけではなく、このフォルダ自体を信頼する必要があります。拒否ルールと確認ルールは、信頼されているセッションでもそうでないセッションでも、すべてのセッションで適用されます。
* **hook はリポジトリ内のスクリプトです。** このファイルの hook は `.claude/hooks/block-rm.sh` を実行します。[hook がどのように解決されるか](/docs/ja/hooks#how-a-hook-resolves)では、これを書く方法について説明しています。
* **ルールはコマンドとパスを記述されたとおりにマッチします。** `Bash(git push *)` は [`git -C . push`](/docs/ja/permissions#bash-rule-limits) にはマッチしません。`Read(./.env)` 単独では、ファイルツールと `cat .env` のようにファイルを名前で指定するコマンドを停止しますが、[`grep -r` をディレクトリ上で実行](/docs/ja/permissions#read-and-edit)することは停止しません。このファイルの `sandbox` ブロックはそのギャップを埋めます。sandbox は[あなたの `Read` 拒否パス](/docs/ja/settings-reference#sandbox-filesystem-denyread)をすべてのサンドボックス化されたコマンドが読み取れないものに追加するためです。

<Tabs>
  <Tab title="コピー可能な設定ファイル">
    これをリポジトリのトップレベルに `.claude/settings.json` として保存してコミットしてください。コメントのない有効な JSON なので、そのまま貼り付けて、不要なキーを削除できます。

    ```json .claude/settings.json theme={null}
    {
      "permissions": {
        "allow": [
          "Bash(npm run *)"
        ],
        "ask": [
          "Bash(git push *)"
        ],
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      "plansDirectory": "./plans"
    }
    ```
  </Tab>

  <Tab title="各キーの機能">
    各キーの上にコメントが付いた同じファイルです。ここで読んでください。Claude Code は設定ファイルのコメントを受け入れないため、他のタブからコピーしてください。

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // npm スクリプトを確認なしで実行
        "allow": [
          "Bash(npm run *)"
        ],
        // git push コマンドの前に確認
        "ask": [
          "Bash(git push *)"
        ],
        // ファイルツールとファイル読み取りコマンドによる env ファイルと secrets フォルダの読み取りを拒否
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // すべての Bash コマンドの前に、リポジトリ内のスクリプトを実行してそれをブロックできます
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      // すべてのクローンでチームのプラグインマーケットプレイスを登録
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // そのマーケットプレイスから 1 つのプラグインを有効化。GitHub リポジトリなどの外部ソースからのプラグインは、各ユーザーが 1 回インストールする必要があります
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // サンドボックスコマンド：書き込み可能なビルドディレクトリ。npm と example.com は事前に許可、他のホストはまだプロンプト表示
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      // プランファイルをリポジトリ内に保持
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  組織の管理設定
</h2>

`managed-settings.json` ファイルは、管理キーの形状を示し、各キーに対して 1 つの妥当な値を含みます。これは推奨ポリシーではありません。自分の要件に合致するキーを選択し、独自の値を設定してください。この例では、以下のキーを設定しています。

* `forceLoginMethod` と `forceLoginOrgUUID` はログイン方法と組織を固定します
* `availableModels` と `enforceAvailableModels` はセッションが使用できるモデルを制限します
* `permissions.deny` は 2 つのファイル読み取りと `curl` コマンドをブロックし（[Claude が記述する方法](/docs/ja/permissions#bash-rule-limits)）、`disableBypassPermissionsMode` は権限モードのバイパスを削除します
* [`allowManagedPermissionRulesOnly`](/docs/ja/settings-reference#allowmanagedpermissionrulesonly) と [`allowManagedMcpServersOnly`](/docs/ja/settings-reference#allowmanagedmcpserversonly) は、管理権限と MCP 許可リストのみを適用対象にします
* `allowedMcpServers` は MCP サーバーを URL で固定します
* `strictKnownMarketplaces` は 1 つのプラグインマーケットプレイスを許可します
* `sandbox` はコマンドを固定ネットワーク許可リストでサンドボックス化し、サンドボックス外での再試行を許可しません
* `requiredMinimumVersion` は最小 Claude Code バージョンを設定します
* `cleanupPeriodDays` はセッショントランスクリプトおよび他のローカルデータの保持期間を 7 日に短縮します
* `companyAnnouncements` は起動時にメッセージを表示します

管理者は、このようなファイルを `managed-settings.json` として、または MDM もしくは [サーバー管理設定](/docs/ja/server-managed-settings) を通じて同じ JSON をデプロイします。デプロイされた 1 つのファイルは、それが到達するすべてのマシンまたはアカウントに適用されます。グループに異なる値を付与するには、そのグループに異なるファイルまたはプロファイルをデプロイしてください。[サーバー管理設定はまだグループごとのポリシーをサポートしていない](/docs/ja/server-managed-settings#current-limitations) ためです。

<Tabs>
  <Tab title="コピー可能な設定ファイル">
    これを `managed-settings.json` としてデプロイするか、MDM または claude.ai コンソールを通じて同じ JSON をデプロイしてください。コメントのない有効な JSON です。例の組織 UUID、サーバー URL、およびマーケットプレイスを自分のものに置き換え、不要なキーを削除してください。

    ```json managed-settings.json theme={null}
    {
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true,
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      "requiredMinimumVersion": "2.1.150",
      "cleanupPeriodDays": 7,
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>

  <Tab title="各キーの機能">
    各キーの上にコメントが付いた同じファイルです。ここで読んでください。Claude Code は設定ファイルのコメントを受け入れないため、他のタブからコピーしてください。

    ```jsonc managed-settings.json theme={null}
    {
      // claude.ai ログインのみ、かつこの組織内のみ
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Opus と Sonnet モデルのみ。enforceAvailableModels を使用すると、デフォルトオプションもリストに従います
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // すべてのマシンで curl、プロジェクトの .env ファイル、およびそのシークレットフォルダをブロック
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // すべてのセッションから権限モードのバイパスを削除
        "disableBypassPermissionsMode": "disable"
      },
      // ユーザー、プロジェクト、およびローカル設定からの権限ルールを無視
      "allowManagedPermissionRulesOnly": true,
      // GitHub MCP サーバーのみ。ユーザーは任意のサーバーに「github」という名前を付けることができるため、名前ではなく URL で照合されます。リストに一致しないユーザーが追加したサーバーは読み込まれません。リストに URL エントリのみがある場合、すべての stdio サーバーを含みます。以下の allowManagedMcpServersOnly キーは、この管理リストのみが適用される許可リストにします
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // プラグインはこのマーケットプレイスからのみ取得可能
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Claude が実行するすべてのコマンドをサンドボックス化し、サンドボックスをセットアップできない場合は起動を拒否し、ブロックされたコマンドがサンドボックス外で再試行されることを許可しません。ネットワークは npm と GitHub に制限され、ユーザーはドメインを追加できません
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      // 2.1.150 より古いバージョンでの起動を拒否
      "requiredMinimumVersion": "2.1.150",
      // 7 日後にセッショントランスクリプトおよび他のローカルセッションデータを削除
      "cleanupPeriodDays": 7,
      // すべてのユーザーが起動時に表示するメッセージ
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
