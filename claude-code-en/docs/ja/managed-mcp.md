> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 組織の MCP サーバーアクセスを制御する

> 管理対象設定ファイル、管理対象設定、許可リスト、拒否リストを使用して、ユーザーが追加または接続できる MCP サーバーを制限するか、すべてのユーザーにサーバーを提供します。

デフォルトでは、Claude Code を実行している誰もが、選択した任意の [MCP サーバー](/docs/ja/mcp) に接続できます。Anthropic は、[Anthropic Directory](https://claude.ai/directory) に追加する前に、コネクターを [リスティング基準](https://claude.com/docs/connectors/building/review-criteria) に照らして確認しますが、MCP サーバーのセキュリティ監査や管理は行いません。管理者として、組織内で実行されるサーバーを制限できます。固定された承認済みセットのデプロイから MCP 全体の無効化まで、すべてのユーザーにサーバーを提供することもできます。

これらの制限は、Claude Code が自身で読み込むサーバー（claude.ai から取得するコネクターを含む）に適用されます。デスクトップアプリがローカルセッションと SSH セッションに配信するコネクターはプロセス内で到着し、代わりに claude.ai 組織設定から管理されます。[コネクターが Claude Code に到達する方法](/docs/ja/mcp#how-connectors-reach-claude-code) は、クラウドセッションを含む各種セッションのコネクターに適用される制御を示しています。

このページでは、以下の方法について説明します。

* [必要な制御量に合致するパターンを選択する](#choose-a-pattern)
* [`managed-mcp.json` で固定サーバーセットをデプロイする](#exclusive-control-with-managed-mcp-json)（[MCP 全体を無効化する](#disable-mcp-entirely) 方法を含む）
* [管理対象設定を通じてサーバーを提供する](#provide-servers-through-managed-settings)（ユーザーが独自のサーバーを保持する場合）
* [許可リストと拒否リストでサーバーを制御する](#policy-based-control-with-allowlists-and-denylists)
* [制限がサーバーをブロックするときにユーザーが何を期待するかを伝える](#how-restrictions-appear-to-users)
* [組織が実際に使用するサーバーを監視する](#monitor-mcp-usage)

<Note>
  [セキュリティ](/docs/ja/security) ページは MCP の脅威モデルと、サーバーを承認する前に評価する方法について説明しています。[実施する内容を決定する](/docs/ja/admin-setup#decide-what-to-enforce) は、他の管理制御と並んで MCP 制限について説明しています。
</Note>

<h2 id="choose-a-pattern">
  パターンを選択する
</h2>

Claude Code は、さまざまな制限レベルをサポートしています。各パターンは、以下で説明するメカニズムの 1 つ以上を使用します。`managed-mcp.json` は固定セットをデプロイするため、`managedMcpServers` マネージド設定はユーザーが追加するサーバーと共にサーバーを提供するため、`allowedMcpServers`/`deniedMcpServers` はユーザーが設定する内容をフィルタリングするためです。

| パターン            | 機能                                                                                                                                                                             | 設定                                                                                                  |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| **MCP を無効化**    | サーバーは読み込まれません。ただし、[セッションを開始したアプリが登録するインプロセスサーバー](#exclusive-control-with-managed-mcp-json)と、[`managedMcpServers` を通じて提供するサーバー](#provide-servers-through-managed-settings)は除きます | 空のサーバーマップを含む `managed-mcp.json`                                                                     |
| **固定デプロイ**      | すべてのユーザーが同じサーバーを取得し、他のサーバーを追加できません                                                                                                                                             | 必要なサーバーを含む `managed-mcp.json`                                                                       |
| **提供されるサーバー**   | すべてのユーザーがリストされたリモートサーバーを取得し、独自のサーバーを保持します                                                                                                                                      | マネージド設定の `managedMcpServers`                                                                        |
| **承認されたカタログ**   | 承認されたサーバーのリストを公開します。ユーザーは必要なものを追加し、その他はすべてブロックされます                                                                                                                             | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                            |
| **プラグインサーバーのみ** | ユーザーは `~/.claude.json` または `.mcp.json` を通じてサーバーを追加できません。プラグインサーバーは引き続き読み込まれます                                                                                                  | [`strictPluginOnlyCustomization`](/docs/ja/settings-reference#strictpluginonlycustomization) とリストの `mcp` |
| **ソフト許可リスト**    | ユーザーが独自の設定で拡張できる許可リストを適用します                                                                                                                                                    | `allowManagedMcpServersOnly` なしの `allowedMcpServers`                                                |
| **拒否リストのみ**     | 既知の不正なサーバーをブロックし、その他はすべて許可します                                                                                                                                                  | `deniedMcpServers`                                                                                  |
| **制限なし**        | ユーザーは何でも追加できます                                                                                                                                                                 | マネージド MCP 設定をデプロイしないでください                                                                           |

<Note>
  Claude Code には、ユーザーが参照してインストールできる組み込み MCP サーバーレジストリはありません。承認されたカタログパターンの場合、承認されたリストとその `claude mcp add` コマンドを、内部 wiki などのユーザーが見つけやすい場所で共有するか、[マネージドプラグインマーケットプレイス](/docs/ja/plugins/org#restrict-what-users-can-install)を通じてプラグインとしてサーバーを配布して、ユーザーが `/plugin` から参照してインストールできるようにしてください。
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  managed-mcp.json による排他的制御
</h2>

`managed-mcp.json` ファイルをデプロイすると、Claude Code はそのファイルで定義されたサーバー、[`managedMcpServers` を通じて提供するサーバー](#provide-servers-through-managed-settings)、およびセッションを開始したアプリが登録するインプロセスサーバー（VS Code 拡張機能独自のサーバーや[デスクトップアプリが提供するコネクタ](/docs/ja/mcp#how-connectors-reach-claude-code)など）のみを読み込みます。ユーザーは、プラグイン提供のサーバーや [`--mcp-config` CLI フラグ](/docs/ja/cli-reference#cli-flags)で渡されたサーバーを含む、その他の MCP サーバーを追加、変更、または使用することはできません。このファイルは、[マネージドセットと共にそれらを許可](#allow-claude-ai-connectors-alongside-the-managed-set)しない限り、Claude Code が自身で取得する claude.ai コネクタも抑制します。

<h3 id="deploy-managed-mcp-json">
  managed-mcp.json をデプロイする
</h3>

`managed-mcp.json` はスタンドアロンファイルであるため、[サーバー管理設定](/docs/ja/server-managed-settings)を通じて配信することはできません。マネージドセットなしで管理設定を通じてサーバーを配信するには、代わりに [`managedMcpServers`](#provide-servers-through-managed-settings) を使用してください。

管理者権限を持つシステムパスに書き込むことができるすべてのプロセスがファイルをデプロイできます。フリート全体では、通常は Jamf などのデバイス管理ツール、macOS 上の構成プロファイル、Windows 上のグループポリシーまたは Intune、または Linux 上の選択したフリート管理を通じて行われます。Claude Code は以下のパスのいずれかでファイルを探します。

| プラットフォーム    | パス                                                         |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux と WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

このファイルはプロジェクト [`.mcp.json`](/docs/ja/mcp#project-scope) ファイルと同じ形式を使用します。

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  ユーザーごとの認証情報で認証する
</h3>

マシン上のすべてのユーザーがこのファイルを読むことができるため、API キーやその他の認証情報を `env` ブロックに保存しないでください。代わりに、以下のいずれかを使用してユーザーごとの認証情報を渡してください。

* [環境変数展開](/docs/ja/mcp#environment-variable-expansion-in-mcp-json)を使用して、各ユーザーの環境からシークレットを読み込む。
* [OAuth またはユーザーごとのヘッダー](/docs/ja/mcp#authenticate-with-remote-mcp-servers)を使用して、各ユーザーが自分自身として認証する。
* [`headersHelper`](/docs/ja/mcp#use-dynamic-headers-for-custom-authentication)を使用して、接続時に認証情報を生成する。

<h3 id="servers-passed-with-mcp-config-or-strict-mcp-config">
  `--mcp-config` または `--strict-mcp-config` で渡されたサーバー
</h3>

セッションが `managed-mcp.json` がデプロイされている間に `--mcp-config` を通じてサーバーを受け取る場合、ユーザーが見るものはワークステーションとクラウドセッション間で異なります。

* ワークステーション上では、Claude Code は `You cannot dynamically configure MCP servers when an enterprise MCP config is present` というメッセージで起動時に終了します。
* ファイルがデプロイされているホスト上の[クラウドセッション](/docs/ja/claude-code-on-the-web)（[セルフホストランナー](/docs/ja/self-hosted-environments-configuration#mcp-servers)など）では、Claude Code はマネージドサーバーのみで起動し、claude.ai コネクタおよびクラウドホストが `--mcp-config` を通じて配信するその他のサーバーをスキップします。セッション内のどのサーバーが除外されたかをユーザーに伝えるものはありません。Claude Code はそれらを stderr の警告で名前を付けます。これはセルフホストランナーが `debug` ログレベルで記録します。

`--strict-mcp-config` フラグはマネージドセットを置き換えるよう要求します。ユーザーがそのようなファイルがデプロイされている間にそれを渡す場合、Claude Code はワークステーション上とクラウドセッション上の両方で起動時に終了します。

<h3 id="how-allowlists-and-denylists-apply-to-the-managed-set">
  許可リストと拒否リストがマネージドセットに適用される方法
</h3>

拒否リストは `managed-mcp.json` 内のサーバーをさらにフィルタリングできます。

* `deniedMcpServers` はマネージドサーバーにも適用されるため、エントリに一致するマネージドサーバーは読み込まれません。
* ユーザー独自の `deniedMcpServers` は設定からマージされるため、ユーザーはマネージドサーバーを自分自身でブロックできます。

`allowedMcpServers` は `managed-mcp.json` 内のサーバーには適用されません。ただし 1 つの例外があります。Claude Code は定義が [`${VAR}` 展開](/docs/ja/mcp#environment-variable-expansion-in-mcp-json)を使用するサーバーを許可リストに対してチェックします。そのサーバーの有効な構成はファイルだけではなく各ユーザーの環境から来るためです。v2.1.259 より前では、許可リストが設定されている場合、すべてのマネージドサーバーが許可リストを通過する必要がありました。どのフィールドが `${VAR}` チェックをトリガーするか、およびチェックの完全な順序については、[サーバーがどのように評価されるか](#how-a-server-is-evaluated)を参照してください。

`allowedMcpServers` を使用して独自の `managed-mcp.json` サーバーの一部が読み込まれないようにしていた場合、`${VAR}` 展開を使用しない限り、各ユーザーが v2.1.259 以降の最初の起動時にそれらのサーバーが読み込まれ始めます。プロンプトや通知はありません。`deniedMcpServers` のみがそれらのサーバーから差し引かれます。それらの拒否リストエントリを追加するか、ユーザーがアップグレードする前に、グループごとに別の `managed-mcp.json` をデプロイしてください。

<h3 id="validate-the-configuration">
  構成を検証する
</h3>

ファイルが有効であることを確認するには、マネージドマシン上で 2 つのチェックを実行してください。

1. `claude mcp list` は `managed-mcp.json` 内のサーバーのみを表示します。`managedMcpServers` を通じて提供するサーバーも表示されます。2 つの他の結果は何かが間違っていることを意味します。
   * ユーザー独自のサーバーがまだ表示される場合、Claude Code はファイルを読み込んでいません。そのパスと親ディレクトリの権限を確認してください。
   * ファイルのサーバーが表示されず、`MCP config diagnostics` セクションがエンタープライズ構成の解析失敗をマークしている場合、Claude Code はファイルを読み込むか解析できません。そのセクションが名前を付けるエラーを修正してから、ユーザーに Claude Code を再起動させてください。
2. `claude mcp add --transport http test https://example.com/mcp` は `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` で失敗します。ポリシーチェックが何かに接続される前にコマンドを拒否するため、URL は実際のサーバーである必要はありません。

<h3 id="disable-mcp-entirely">
  MCP を完全に無効にする
</h3>

空のサーバーマップを含む `managed-mcp.json` をデプロイして、[セッションを開始したアプリが登録するインプロセスサーバー](#exclusive-control-with-managed-mcp-json)を除くすべての MCP サーバーをブロックします。

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add` は上記のエンタープライズポリシーエラーで失敗します。ユーザーが以前に構成したサーバーは、次回セッションを開始するときに読み込まれなくなります。ポリシーが理由であることについての警告はありません。`managedMcpServers` を通じて提供するサーバーは空のマップの下でも読み込まれるため、MCP を完全に無効にするにはそのキーも設定しないままにしてください。

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  マネージドセットと共に claude.ai コネクタを許可する
</h3>

デフォルトでは、`managed-mcp.json` をデプロイすると、Claude Code が自身で取得する [claude.ai コネクタ](/docs/ja/mcp#use-mcp-servers-from-claude-ai)（管理者が claude.ai 管理コンソールで組織用に構成したコネクタを含む）が抑制されます。`managed-mcp.json` 内のサーバーと共にそれらのコネクタを読み込むには、[マネージド設定ソース](/docs/ja/admin-setup#decide-how-settings-reach-devices)で `"allowAllClaudeAiMcps": true` を設定してください。

設定が有効になると、Claude Code は `managed-mcp.json` がデプロイされていない場合に読み込むのと同じ claude.ai コネクタを読み込みます。[許可リストと拒否リスト](#policy-based-control-with-allowlists-and-denylists)はそれらのコネクタに引き続き適用されるため、`deniedMcpServers` で特定のコネクタをブロックできます。この設定は Claude Code が自身で取得する claude.ai コネクタのみに影響します。プラグイン提供のサーバーは抑制されたままです。

クラウドセッションとデスクトップアプリのローカルおよび SSH セッションは、別の方法でコネクタを受け取ります。これは[コネクタが Claude Code に到達する方法](/docs/ja/mcp#how-connectors-reach-claude-code)で説明されています。クラウドセッションを実行するホスト上の `managed-mcp.json`（[セルフホストランナーホスト](/docs/ja/self-hosted-environments-configuration#mcp-servers)など）は、`allowAllClaudeAiMcps` を設定するかどうかに関わらず、そのセッションのコネクタを抑制します。デスクトップアプリがローカルおよび SSH セッションに配信するコネクタには `managed-mcp.json` は到達しません。

Claude Code は `allowAllClaudeAiMcps` を管理者制御のポリシー層からのみ読み込みます。サーバー管理設定、MDM デプロイされた plist または HKLM レジストリキー、またはシステム `managed-settings.json` ファイルです。ユーザーまたはプロジェクト設定に配置しても効果がないため、ユーザーは排他的制御が抑制したコネクタを再度有効にすることはできません。

<h2 id="provide-servers-through-managed-settings">
  マネージド設定を通じてサーバーを提供する
</h2>

MCP を排他的に制御することなく、すべてのユーザーにリモート MCP サーバーのセットを提供するには、[マネージド設定ソース](/docs/ja/admin-setup#decide-how-settings-reach-devices)（サーバーマネージド設定、[Claude アプリゲートウェイ](/docs/ja/claude-apps-gateway-config#what-goes-in-cli)ポリシー、MDM プロファイルまたはレジストリポリシー、または `managed-settings.json`）の `managedMcpServers` の下にリストします。ユーザーは自分で追加したサーバーを保持し、さらにあなたのサーバーを受け取ります。Claude Code v2.1.259 以降が必要です。以前のクライアントはこのキーを無視します。

値はサーバー名をキーとするオブジェクトです。各エントリは、プロジェクト [`.mcp.json`](/docs/ja/mcp#project-scope) ファイル内の HTTP または SSE サーバーと同じ形状を持ち、[リモート MCP サーバーで認証する](/docs/ja/mcp#authenticate-with-remote-mcp-servers)で説明されているオプションの `headers` および `oauth` メンバーを含みます。この例は、各ユーザーが OAuth でサインインする検索サーバーと、組織が発行するヘッダーを送信するレコードサーバーを提供します。

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

マシン上のマネージド設定を読み取ることができるすべてのユーザー（ユーザー自身を含む）は、ここで設定したヘッダー値を読み取ることができます。その全体的なオーディエンス向けに発行された認証情報を使用するか、`headers` を省略して、各ユーザーが OAuth でサインインするようにしてください。

<h3 id="what-an-entry-can-contain">
  エントリに含めることができるもの
</h3>

Claude Code は、以下のすべてのチェックに合格した場合にのみエントリを読み込みます。1 つのチェックに失敗したエントリはドロップし、`/status` で読み取ることができる通知を記録し、他のエントリは引き続き読み込みます。

* `type` は `http` または `sse` です。`.mcp.json` と同様に、`streamable-http` は `http` のエイリアスとして受け入れられます。
* `url` は `https://` URL です。Claude Code は `localhost` を指すものを含む、プレーンな `http://` URL を拒否します。
* エントリに `command`、`args`、`env`、または `headersHelper` メンバーがないため、マネージド設定ドキュメントはユーザーのマシン上で実行するプログラムを指定しません。
* 値に `${VAR}` 参照が含まれていません。Claude Code はこれらのエントリ内の環境変数を展開しないため、リテラル値を記述してください。
* サーバー名には文字、数字、ハイフン、アンダースコアのみが含まれ、キーまたは値に制御文字や非表示の書式文字が含まれていません。

Claude Desktop には同じ名前のマネージド設定があり、その値は異なるエントリ形状の配列であるため、一方を他方にコピーしないでください。Claude Code は配列形式を受け入れず、代わりに通知を記録します。

Claude アプリゲートウェイは起動時に同じチェックを実行します。[ポリシー内の MCP サーバー](/docs/ja/claude-apps-gateway-config#mcp-servers-in-a-policy)を参照してください。

<h3 id="how-provided-servers-load">
  提供されたサーバーがどのように読み込まれるか
</h3>

これらのルールは、提供されたサーバーが別のサーバー定義またはこのページの別の設定と重複する場合に何が読み込まれるかを決定します。

* 提供されたサーバーは、ローカル、プロジェクト、またはユーザースコープ内の同じ名前のサーバーよりも優先され、同じ URL を指すプラグインサーバーまたは claude.ai コネクタよりも優先されます。
* `managed-mcp.json` もデプロイする場合、Claude Code はそのサーバーと提供されたサーバーを一緒に読み込み、両方が名前を定義する場合はファイルのエントリが優先されます。
* [`strictPluginOnlyCustomization`](/docs/ja/settings-reference#strictpluginonlycustomization) が `mcp` サーフェスをロックしても、提供されたサーバーは引き続き読み込まれます。
* `deniedMcpServers` はユーザー自身の設定からのエントリを含む提供されたサーバーに適用されるため、ユーザーは自分自身のためにそれをブロックできます。提供されたサーバーは `allowedMcpServers` エントリを必要としません。

`managed-mcp.json` もデプロイしていない場合、実行ごとのフラグはその意味を保持します。

* ユーザーが同じ名前で `--mcp-config` で渡すサーバーは、その実行のために提供されたサーバーを置き換え、`allowedMcpServers` に対してチェックされます。
* `--strict-mcp-config` は、提供されたサーバーを他のすべての設定されたサーバーと一緒に除外します。

`managed-mcp.json` がデプロイされている場合、両方のフラグは [managed-mcp.json による排他的制御](#exclusive-control-with-managed-mcp-json)で説明されているように動作します。

<h3 id="what-users-can-see-and-change">
  ユーザーが見ることができ、変更できるもの
</h3>

ユーザーは提供されたサーバーを編集または削除できません。

* `claude mcp remove` は、サーバーが組織によって提供されていることを報告します。
* `managed-mcp.json` もデプロイしていない場合、ユーザーが同じ名前で追加するエントリは保存されますが、あなたのサーバーが存在する間は使用されません。
* ユーザーは [`/mcp`](/docs/ja/mcp#disable-a-server-without-removing-it) で提供されたサーバーを自分自身のためにオフにすることができます。これは **Managed MCPs** の下に提供されたサーバーをリストします。

`claude mcp get` と `/mcp` は、提供されたサーバーの URL をそのホストのみとして表示します。例えば `https://mcp.example.com/…` であり、`claude mcp get` はヘッダー名を値なしで表示します。

<h3 id="where-managedmcpservers-applies">
  `managedMcpServers` が適用される場所
</h3>

Claude Code は、[Claude Code がマネージドソースを組み合わせる方法](/docs/ja/managed-settings#how-claude-code-combines-managed-sources)の下で選択するマネージドソースから `managedMcpServers` を読み取ります。そのソースが [`managedSourcesBehavior`](/docs/ja/settings-reference#managedsourcesbehavior) を `"merge"` に設定する場合、Claude Code は代わりにすべての管理者ソースからサーバーを提供し、2 つのソースが同じ名前を定義する場合、より高いランクのソースのエントリが全体に適用されます。ユーザーが書き込み可能な HKCU レジストリから、[埋め込みホストが提供する親設定](/docs/ja/managed-settings#parent-settings-from-embedding-hosts)から、またはユーザー、プロジェクト、またはローカル設定ファイルから読み取ることはなく、警告とともにキーをドロップします。

Claude Code は、サードパーティデプロイメント上の Claude Desktop アプリの Code タブまたはアプリの Cowork セッションでキーを読み取りません。Claude Desktop はそれらのセッションの MCP サーバーを自身で提供およびロックするためです。マネージド設定がそこにキーを含む場合、`/status` と `claude doctor` はそう言います。

<h3 id="when-provided-servers-connect">
  提供されたサーバーが接続するとき
</h3>

`managedMcpServers` がサーバーマネージド設定を通じて到着する場合、そのタイミングは [フェッチとキャッシング動作](/docs/ja/server-managed-settings#fetch-and-caching-behavior)に従います。

* キャッシュされた設定を持つマシンでは、Claude Code はこのキーのキャッシュされたコピーを、サーバーがセッションの設定を確認するまで保留し、その確認を待ってから MCP サーバーを読み込みます。確認が失敗した場合、セッションは提供されたサーバーなしで続行され、`/status` はそれらが保留されていることを示します。
* マシンの最初の起動時に、まだキャッシュされたものがない場合、設定が到着する前に開始される対話型セッションは、提供されたサーバーが到着するとすぐに接続し、既に開始されている `claude -p` 実行はそれらなしで完了できます。

[ゲートウェイサインイン](/docs/ja/claude-apps-gateway-config#precedence-with-other-managed-sources)では、Claude Code はセッション開始前にポリシーを読み込むため、どちらのケースも提供されたサーバーを遅延またはスキップしません。

既に実行中の対話型セッションはキーへの編集を適用します。

* **サーバーを追加する**：Claude Code は更新された設定が到着したときにそれを接続し、再起動は不要です。
* **サーバーのエントリを変更する**：これらのセッションは新しい定義で再接続します。
* **サーバーを削除する**：実行中の対話型セッションは、変更された設定を読み取ると、それを切断します。非対話型（`-p`）実行はそれが終了するまでそれを保持します。

<h2 id="policy-based-control-with-allowlists-and-denylists">
  allowlist と denylist を使用したポリシーベースの制御
</h2>

allowlist と denylist は、設定されたサーバーのうちどれをロードできるかをフィルタリングします。これらはレジストリではなく、allowlist または denylist が適用される前に、ユーザー、プラグイン、または組織によってサーバーを追加する必要があります。

組織が `managedMcpServers` を通じて配信するサーバーは allowlist エントリなしでロードされ、[サーバーの評価方法](#how-a-server-is-evaluated)は `managed-mcp.json` サーバーについて説明しています。denylist はインプロセス `type: "sdk"` エントリを除き、どこから来たサーバーにも適用されます。

サーバーをユーザーに配信するには、[`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) または [`managedMcpServers`](#provide-servers-through-managed-settings) を使用します。両方のリストは、インプロセス `type: "sdk"` エントリを除き、[`--mcp-config` CLI フラグ](/docs/ja/cli-reference#cli-flags)で渡されたサーバーもフィルタリングします。`--strict-mcp-config` はどの設定ファイルをロードするかを制限し、どちらのリストもバイパスしません。

allowlist を権限あるものにするには、[管理設定ソース](/docs/ja/admin-setup#decide-how-settings-reach-devices)（サーバー管理設定や配信された `managed-settings.json` ファイルなど）で `allowedMcpServers` と `allowManagedMcpServersOnly: true` を一緒に設定します。

ロックはすべての管理者制御の管理ソースから適用されるため、配信されたファイル内のロックダウンは、MCP に言及しないサーバー管理設定も使用されている場合でも適用されます。ロックがオンの場合、管理 allowlist は、1 つを設定する最も高いランクの管理ソースから取得されます。ソース全体でロックと allowlist を読み取るには、Claude Code v2.1.273 以降が必要です。

[allowlist を管理設定のみに制限する](#restrict-the-allowlist-to-managed-settings-only)は設定を示しています。

`allowManagedMcpServersOnly` がない場合、ユーザー自身の `~/.claude/settings.json` を含むすべての設定スコープから allowlist がマージされるため、ユーザーは allowlist が許可するものを広げることができます。denylist はスコープに関係なくマージされます。

<Note>
  `allowManagedMcpServersOnly` は `allowManagedPermissionRulesOnly` とは別であり、後者は[権限ルール](/docs/ja/permissions#managed-settings)のみをロックダウンします。そのフラグを設定しても MCP allowlist は強制されません。
</Note>

<h3 id="match-servers-by-url-command-or-name">
  URL、コマンド、または名前でサーバーをマッチさせる
</h3>

`allowedMcpServers` と `deniedMcpServers` はエントリのリストです。各エントリは、サーバーを URL、コマンド、または名前で識別する単一のキーを持つオブジェクトです。

| キー              | マッチ対象                                | 用途                   |
| :-------------- | :----------------------------------- | :------------------- |
| `serverUrl`     | リモートサーバー URL、完全一致または `*` ワイルドカード     | HTTP および SSE サーバー    |
| `serverCommand` | stdio サーバーを開始する正確なコマンドと引数            | stdio サーバー           |
| `serverName`    | ユーザーが割り当てたラベル。完全一致のみ。ワイルドカードは展開されません | どちらのタイプでも、ただし下の警告を参照 |

`allowedMcpServers` を設定しないことは、空の配列に設定することとは異なります。

| 設定                  | 設定なし（デフォルト）    | 空の配列 `[]`                                               | 設定あり                                                            |
| :------------------ | :------------- | :------------------------------------------------------ | :-------------------------------------------------------------- |
| `allowedMcpServers` | すべてのサーバーが許可される | [組織自身のサーバー](#how-a-server-is-evaluated)を除き、サーバーは許可されません | マッチするサーバーのみが許可され、[組織自身のサーバー](#how-a-server-is-evaluated)は除外されます |
| `deniedMcpServers`  | サーバーはブロックされません | サーバーはブロックされません                                          | マッチするサーバーがブロックされます                                              |

エントリがスキーマ検証に失敗した場合の詳細は、[管理設定の無効なエントリ](/docs/ja/managed-settings#invalid-entries-in-managed-settings)を参照してください。

<Warning>
  どちらのリストでも `serverName` エントリはセキュリティ制御ではありません。名前は `claude mcp add` を実行するか設定ファイルを編集するときにユーザーが割り当てるラベルであり、基盤となるサーバーではないため、ユーザーは任意のサーバーを `github` と呼ぶことができます。claude.ai コネクタの場合、名前は claude.ai が返す表示名であり、変更される可能性があります。実際に実行されるサーバーを強制するには、`serverCommand` または `serverUrl` エントリを追加します。
</Warning>

`serverName` の検証は 2 つのリスト間で異なります。

* `deniedMcpServers` では、`serverName` は先頭または末尾に空白がない空でない任意の文字列を受け入れるため、[claude.ai コネクタ](/docs/ja/mcp#use-mcp-servers-from-claude-ai)を表示名でブロックできます。たとえば、`{ "serverName": "claude.ai Slack" }` は Slack コネクタをブロックします。deny が名前変更に対して堅牢である必要がある場合、またはコネクタ名が衝突して ` (N)` サフィックスを取得する場合は、`serverUrl` エントリを優先します。
* `allowedMcpServers` では、`serverName` は文字、数字、ハイフン、アンダースコアに限定されます。Claude Code が自身でフェッチする claude.ai コネクタを allowlist に追加するには `serverUrl` を使用します。クラウドホストが自己ホスト型セッションに配信するコネクタの場合は、代わりに[コネクタトラフィックがネットワークを離れる](/docs/ja/self-hosted-environments-deploy#connector-traffic-leaves-your-network)の下にリストされているエントリを使用します。

Claude Code がフェッチするすべての claude.ai コネクタをオフにするには、[`disableClaudeAiConnectors`](/docs/ja/mcp#disable-claude-ai-connectors)を参照してください。

<h3 id="how-a-server-is-evaluated">
  サーバーの評価方法
</h3>

サーバーをロードする前に、`managed-mcp.json` からのサーバーを含めて、Claude Code は以下の 3 つのチェックを順番に実行します。ユーザーがサーバーを再接続するか、`/mcp` で無効なサーバーをオンに戻すときに再度実行されます。インプロセス `type: "sdk"` サーバー（[セッションを開始したアプリが登録](/docs/ja/mcp#how-connectors-reach-claude-code)）は、3 つすべてをスキップします。

1. **リストをマージします。** すべての設定スコープからの allowlist と denylist エントリが 1 つの allowlist と 1 つの denylist に結合されます。`allowManagedMcpServersOnly` が `true` の場合、管理 allowlist のみが保持されます。denylist は常にすべてのスコープからマージされます。複数の管理ソースが存在する場合、[すべての管理ソースから読み取られるキー](/docs/ja/managed-settings#keys-read-from-every-admin-source)は、管理スコープのリストを提供するソースを示しています。
2. **denylist をチェックします。** URL、コマンド、または名前で denylist エントリにマッチするサーバーはブロックされます。denylist マッチをオーバーライドするものはありません。
3. **allowlist をチェックします。** `allowedMcpServers` がどこにも設定されていない場合、denylist を通過したすべてのサーバーがロードされます。設定されている場合、サーバーがマッチする必要があるものはそのタイプに依存し、以下の表に示されています。

   組織自身のサーバーはこのチェックをスキップします。すべての `managedMcpServers` エントリ、および `${VAR}` 展開を使用しない値を持つ `managed-mcp.json` エントリです。Chrome の Claude、Claude Code が実行中の VS Code または JetBrains IDE に接続する `ide` サーバー、CLI 自身が設定するサーバーなどの組み込みサーバーもスキップします。

   コマンド、引数、`env`、URL、またはヘッダーで `${VAR}` 展開を使用する `managed-mcp.json` サーバーは、ユーザー、プラグイン、`--mcp-config`、または claude.ai が追加するすべてのサーバーと同様にチェックされます。

| サーバータイプ            | マッチ時に許可される                                                                                   |
| :----------------- | :------------------------------------------------------------------------------------------- |
| リモート（HTTP または SSE） | `serverUrl` エントリ。`serverName` マッチは allowlist に `serverUrl` エントリが含まれていない場合にのみカウントされます         |
| stdio              | `serverCommand` エントリ。`serverName` マッチは allowlist に `serverCommand` エントリが含まれていない場合にのみカウントされます |

これらのチェック内で 3 つのマッチングルールが適用されます。

* **コマンドは完全にマッチします。** すべての引数、順番に。`["npx", "-y", "server"]` は `["npx", "server"]` または `["npx", "-y", "server", "--flag"]` にマッチしません。
* **`serverCommand` と `serverUrl` の値はマッチング前に展開されます。** ポリシーエントリとサーバーの設定値の両方が [`${VAR}` と `${VAR:-default}` 展開](/docs/ja/mcp#environment-variable-expansion-in-mcp-json)を通過するため、`["${HOME}/bin/server"]` として書かれたエントリは、同じ参照または展開されたパスのいずれかを使用するサーバー設定にマッチします。Windows では、`${HOME}` の代わりに `${USERPROFILE}` など、そこで設定されている環境変数を参照します。`serverName` の値は文字通りマッチし、展開されません。両側は異なる環境を読みます。[ポリシーエントリの展開方法](#how-policy-entries-expand)は、どちらであるか、および allowlist と denylist エントリがどのように異なるかについて説明しています。
* **URL は `*` ワイルドカード**をパターン内の任意の場所（スキームを含む）でサポートします。ホスト名マッチングは大文字と小文字を区別せず、末尾の FQDN ドットを無視するため、`https://Mcp.Example.com/*` は `https://mcp.example.com/api` にマッチします。パスは大文字と小文字を区別したままです。

| パターン                        | 許可                                     |
| :-------------------------- | :------------------------------------- |
| `https://mcp.example.com/*` | 特定のドメイン上のすべてのパス                        |
| `https://mcp.example.com`   | そのドメイン上のすべてのパスも。パスのないパターンは任意のパスにマッチします |
| `https://*.example.com/*`   | `example.com` の任意のサブドメイン               |
| `http://localhost:*/*`      | localhost 上の任意のポート                     |
| `*://mcp.example.com/*`     | 特定のドメインへの任意のスキーム                       |

<h4 id="how-policy-entries-expand">
  ポリシーエントリの展開方法
</h4>

サーバーの設定値は、`.mcp.json` の残りの部分と同様に、ライブプロセス環境から展開されます。ポリシーエントリは代わりにピン留めされた環境から展開されるため、プロジェクトまたはユーザー設定ファイルによって設定された変数が allowlist エントリの意味を変更することはできません。ポリシーエントリはまだ参照する任意の変数の起動シェルの値に依存するため、強制に依存するエントリには文字通りの URL とコマンドを使用します。

| エントリリスト             | 展開元                                                                               | URL エントリのスキーム、ホスト、またはパススコープを変更する展開 |
| ------------------- | --------------------------------------------------------------------------------- | ---------------------------------- |
| `allowedMcpServers` | Claude Code が開始した環境、プラス管理設定からの `env` 値                                            | Claude Code はエントリを無視します            |
| `deniedMcpServers`  | 同じ、および起動値がなく `:-default` がない変数は、ユーザーまたは管理設定など、リポジトリ外の設定ファイルから入力され、許可されるものを広げるだけです | エントリはまだマッチします                      |

Claude Code v2.1.219 以降が必要です。

<h3 id="example-configuration">
  設定例
</h3>

以下の設定は、denylist を使用したハード allowlist をセットアップします。ハイライトされた行は、リストの残りの部分がどのように評価されるかを変更し、ブロック後の吹き出しは各行を説明しています。

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **3 行目**: 最初の `serverUrl` エントリ。1 つ存在すると、すべてのリモートサーバーは URL パターンにマッチする必要があるため、ユーザーは許可された名前を与えることでリストされていないリモートサーバーを取得できません。
* **5 行目**: 最初の `serverCommand` エントリ。stdio サーバーでも同じ効果があるため、すべてのローカルサーバーはリストされたコマンドに完全にマッチする必要があります。
* **11 行目**: denylist の `serverName` エントリ。denylist エントリは常に適用されるため、`dangerous-server` という名前のサーバーは URL またはコマンドに関係なくブロックされます。

この allowlist の `serverName` エントリは、両方のトランスポートタイプがすでにより厳密なエントリを持っているため、何にもマッチしません。

以下のアコーディオンは、他の allowlist と denylist の組み合わせに対してサーバーがどのように評価されるかについて説明しています。

<Accordion title="URL のみの allowlist">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | サーバー                                               | 結果                            |
  | :------------------------------------------------- | :---------------------------- |
  | `https://mcp.example.com/api` の HTTP サーバー          | 許可：URL パターンにマッチ               |
  | `https://api.internal.example.com/mcp` の HTTP サーバー | 許可：ワイルドカードサブドメインにマッチ          |
  | `https://external.example.com/mcp` の HTTP サーバー     | ブロック：URL パターンにマッチしません         |
  | 任意のコマンドを持つ stdio サーバー                              | ブロック：マッチする名前またはコマンドエントリがありません |
</Accordion>

<Accordion title="コマンドのみの allowlist">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | サーバー                                               | 結果                     |
  | :------------------------------------------------- | :--------------------- |
  | `["npx", "-y", "approved-package"]` を持つ stdio サーバー | 許可：コマンドにマッチ            |
  | `["node", "server.js"]` を持つ stdio サーバー             | ブロック：コマンドにマッチしません      |
  | `my-api` という名前の HTTP サーバー                          | ブロック：マッチする名前エントリがありません |
</Accordion>

<Accordion title="名前とコマンドが混在した allowlist">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | サーバー                                                                   | 結果                                                |
  | :--------------------------------------------------------------------- | :------------------------------------------------ |
  | `local-tool` という名前で `["npx", "-y", "approved-package"]` を持つ stdio サーバー | 許可：コマンドにマッチ                                       |
  | `local-tool` という名前で `["node", "server.js"]` を持つ stdio サーバー             | ブロック：コマンドエントリが存在しますがマッチしません                       |
  | `github` という名前で `["node", "server.js"]` を持つ stdio サーバー                 | ブロック：stdio サーバーはコマンドエントリが存在する場合、コマンドにマッチする必要があります |
  | `github` という名前の HTTP サーバー                                              | 許可：名前にマッチ                                         |
  | `other-api` という名前の HTTP サーバー                                           | ブロック：名前がマッチしません                                   |
</Accordion>

<Accordion title="名前のみの allowlist">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | サーバー                                        | 結果              |
  | :------------------------------------------ | :-------------- |
  | `github` という名前で任意のコマンドを持つ stdio サーバー        | 許可：コマンド制限なし     |
  | `internal-tool` という名前で任意のコマンドを持つ stdio サーバー | 許可：コマンド制限なし     |
  | `github` という名前の HTTP サーバー                   | 許可：名前にマッチ       |
  | `other` という名前のサーバー                          | ブロック：名前がマッチしません |
</Accordion>

<Accordion title="denylist オーバーライド付き allowlist">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | サーバー                                          | 結果                                       |
  | :-------------------------------------------- | :--------------------------------------- |
  | `https://mcp.example.com/api` の HTTP サーバー     | 許可：allowlist URL パターンにマッチ、denylist マッチなし |
  | `https://staging.example.com/api` の HTTP サーバー | ブロック：両方にマッチしますが、denylist が優先されます         |
  | `https://other.com/mcp` の HTTP サーバー           | ブロック：allowlist にマッチしません                  |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  allowlist を管理設定のみに制限する
</h3>

管理 allowlist のみが適用されるようにするには、管理設定ファイルで `allowManagedMcpServersOnly` を設定します。

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

`allowManagedMcpServersOnly` が `true` の場合、ユーザー、プロジェクト、ローカル設定からの allowlist は無視されます。denylist はすべての設定スコープからマージされるため、ユーザーは常に自分自身のサーバーをブロックできます。

<h2 id="how-restrictions-appear-to-users">
  制限がユーザーに表示される方法
</h2>

`managed-mcp.json` がデプロイされ、セッションに `--mcp-config` サーバーもある場合にスタートアップ時にユーザーに表示される内容については、[managed-mcp.json による排他的制御](#exclusive-control-with-managed-mcp-json)を参照してください。このテーブルを使用して他のレポートを認識し、変更をロールアウトする前にユーザーが何を期待するかを伝えてください。

| 制限                                                               | ユーザーに表示される内容                                                                                                                 |
| :--------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` が存在し、ユーザーが `claude mcp add` を実行する             | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`                   |
| サーバーがデニーリストにあり、ユーザーが `claude mcp add` を実行する                      | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                                          |
| サーバーがアローリストになく、ユーザーが `claude mcp add` を実行する                      | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                                           |
| ユーザーが `managedMcpServers` のサーバーで `claude mcp remove` を実行する       | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.`                     |
| 以前に設定されたサーバーがポリシーによってブロックされるようになった                               | サーバーは `/mcp` と `claude mcp list` から消える                                                                                       |
| セッション実行中にサーバーがブロックされ、ユーザーが **Reconnect** を選択するか、`/mcp` でそれをオンに戻す | [`MCP server <name> is blocked by enterprise managed policy`](/docs/ja/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

サーバーが静かに消える場合、ユーザーはポリシーが理由であるという信号を受け取らないため、変更をロールアウトする際に影響を受けるユーザーにどのサーバーがブロックされているかを伝えてください。

<h2 id="monitor-mcp-usage">
  MCP 使用状況を監視する
</h2>

[OpenTelemetry エクスポート](/docs/ja/monitoring-usage)が設定されている場合、Claude Code はユーザーが呼び出す MCP サーバーとツールを記録できます。`OTEL_LOG_TOOL_DETAILS=1` を設定して、ツールイベントに MCP サーバーとツール名を含めます。その後、コレクターで集計して、ユーザーが実際に接続するサーバーを確認します。エクスポーターを設定し、完全なイベントスキーマについては、[監視](/docs/ja/monitoring-usage)を参照してください。

<h2 id="configuration-summary">
  設定の概要
</h2>

このページで扱うすべてのファイルと設定、それらが制御する内容、および配信方法：

| サーフェス                        | 制御内容                                                                                                                                                                                          | 保存場所                                                                                                                                       | 配信方法                                                                                                                               |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | 固定サーバーセット、排他的制御                                                                                                                                                                               | システムパス：`/Library/Application Support/ClaudeCode/`、`/etc/claude-code/`、または `C:\Program Files\ClaudeCode\`                                   | MDM、GPO、フリート管理、または管理者権限を持つプロセス。サーバー管理設定を通じて設定することはできません                                                                            |
| `managedMcpServers`          | すべてのユーザーに提供されるリモートサーバー（ユーザー自身のサーバーと並行）                                                                                                                                                        | 管理設定ソースのみ。この設定は他の場所では効果がありません                                                                                                              | [管理設定ソース](/docs/ja/admin-setup#decide-how-settings-reach-devices)：サーバー管理設定、ゲートウェイポリシー、`managed-settings.json`、MDM プロファイル、または HKLM レジストリ |
| `allowedMcpServers`          | 許可されたサーバーのホワイトリスト                                                                                                                                                                             | 任意の[設定スコープ](/docs/ja/settings#where-settings-live)。[サーバーの評価方法](#how-a-server-is-evaluated)に、複数のスコープと管理ソースからのリストがどのように組み合わされるかが記載されています          | 強制するには、[管理設定ソース](/docs/ja/admin-setup#decide-how-settings-reach-devices)：サーバー管理設定、`managed-settings.json`、MDM プロファイル、またはレジストリ           |
| `deniedMcpServers`           | ブロックされたサーバーのブラックリスト                                                                                                                                                                           | 任意の設定スコープ。[サーバーの評価方法](#how-a-server-is-evaluated)に、複数のスコープと管理ソースからのリストがどのように組み合わされるかが記載されています                                              | `allowedMcpServers` と同じ                                                                                                            |
| `allowManagedMcpServersOnly` | ホワイトリストを管理ソースのみにロック                                                                                                                                                                           | 管理設定ソースのみ。[すべての管理ソースから読み込まれるキー](/docs/ja/managed-settings#keys-read-from-every-admin-source)に、どの管理ソースがこれをオンにできるかが記載されています。この設定は他のスコープでは効果がありません | `allowedMcpServers` と同じ                                                                                                            |
| `allowAllClaudeAiMcps`       | Claude Code が自身で取得する claude.ai コネクタを `managed-mcp.json` と並行して読み込みます。[クラウドセッションを実行するホスト上の `managed-mcp.json` は、そのセッションのコネクタを依然として抑制します](#allow-claude-ai-connectors-alongside-the-managed-set) | 管理設定ソースのみ。この設定は他の場所では効果がありません                                                                                                              | `allowedMcpServers` と同じ                                                                                                            |

<h2 id="related-resources">
  関連リソース
</h2>

* [実施する内容を決定する](/docs/ja/admin-setup#decide-what-to-enforce)：MCP 制限と権限ルール、サンドボックス化、および他の管理制御
* [MCP 経由で Claude Code をツールに接続する](/docs/ja/mcp)：トランスポート、スコープ、認証を含む完全な MCP リファレンス
* [設定](/docs/ja/settings)：設定階層と管理設定がどのように優先されるか
* [サーバー管理設定](/docs/ja/server-managed-settings)：Claude.ai 管理コンソールから `allowedMcpServers` と `deniedMcpServers` を配信します
* [セキュリティ](/docs/ja/security)：これらの制御が防御する脅威モデル
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide)：SSO、SCIM、シート管理、およびロールアウトプレイブック
