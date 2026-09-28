> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# MCP を使用して Claude Code をツールに接続する

> Model Context Protocol を使用して Claude Code をツールに接続する方法を学びます。

Claude Code は、AI ツール統合のためのオープンソース標準である [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction) を通じて、数百の外部ツールとデータソースに接続できます。MCP サーバーは Claude Code にツール、データベース、API へのアクセスを提供します。

別のツール（課題追跡ツールや監視ダッシュボードなど）からチャットにデータをコピーしている場合は、サーバーを接続してください。接続すると、Claude は貼り付けたものから作業する代わりに、そのシステムを直接読み取り、操作できます。

初めてサーバーを接続する場合は、ステップバイステップのウォークスルーについて [MCP クイックスタート](/docs/ja/mcp-quickstart) から始めてください。このページは完全なリファレンスです。

<h2 id="what-you-can-do-with-mcp">
  MCP でできること
</h2>

MCP サーバーが接続されている場合、Claude Code に以下のことを依頼できます：

* **課題追跡ツールから機能を実装する**：「JIRA の課題 ENG-4521 に記載されている機能を追加し、GitHub に PR を作成してください。」
* **監視データを分析する**：「Sentry と Statsig をチェックして、ENG-4521 に記載されている機能の使用状況を確認してください。」
* **データベースをクエリする**：「PostgreSQL データベースに基づいて、ENG-4521 機能を使用した 10 人のランダムなユーザーのメールアドレスを検索してください。」
* **デザインを統合する**：「Slack に投稿された新しい Figma デザインに基づいて、標準メールテンプレートを更新してください。」
* **ワークフローを自動化する**：「新機能に関するフィードバックセッションに招待する 10 人のユーザーに Gmail ドラフトを作成してください。」
* **外部イベントに対応する**：MCP サーバーは [チャネル](/docs/ja/channels) として機能することもでき、セッションにメッセージをプッシュするため、Claude は離席中に Telegram メッセージ、Discord チャット、または webhook イベントに対応できます。

<h2 id="find-and-build-mcp-servers">
  MCP サーバーを検索してビルドする
</h2>

[Anthropic Directory](https://claude.ai/directory) でレビュー済みのコネクターを参照してください。Directory コネクターは Claude Code と同じ MCP インフラストラクチャを使用しているため、`claude mcp add` を使用して、そこにリストされているリモートサーバーを追加できます。

<Warning>
  接続する前に、各サーバーを信頼できることを確認してください。外部コンテンツを取得するサーバーは、[プロンプトインジェクションリスク](/docs/ja/security#protect-against-prompt-injection) にあなたを晒す可能性があります。
</Warning>

独自のサーバーをビルドするには、プロトコルの基礎については [MCP サーバーガイド](https://modelcontextprotocol.io/docs/develop/build-server) を、認証、テスト、Directory への提出については [Claude コネクター構築ドキュメント](https://claude.com/docs/connectors/building) を参照してください。

公式の [`mcp-server-dev` プラグイン](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev) を使用して、Claude にサーバーをスキャフォールドしてもらうこともできます。

<Steps>
  <Step title="プラグインをインストールする">
    Claude Code セッションで、以下を実行します：

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    インストールが失敗した場合は、Claude Code が報告するメッセージに一致させてください：

    * `Marketplace "claude-plugins-official" not found`：`/plugin marketplace add anthropics/claude-plugins-official` でマーケットプレイスを追加してから、インストールを再試行してください。
    * [プラグインがマーケットプレイスで見つかりません](/docs/ja/plugins/install#install-a-plugin)：プラグイン名を確認してください。

    インストール概要が `Run /reload-plugins to activate.` を報告する場合、Claude Code はその後、そのリロードを実行します。リロードが次のメッセージが会話を再度読み込むことになると警告する場合は、`/reload-plugins --force` を実行してください。
  </Step>

  <Step title="ビルドスキルを実行する">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude があなたのユースケースについて質問し、リモート HTTP またはローカル stdio サーバーをスキャフォールドします。
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  MCP サーバーのインストール
</h2>

MCP サーバーは、ニーズに応じてさまざまな方法で設定できます。

<h3 id="option-1-add-a-remote-http-server">
  オプション 1: リモート HTTP サーバーを追加する
</h3>

HTTP サーバーは、リモート MCP サーバーに接続するための推奨オプションです。これはクラウドベースのサービスに対して最も広くサポートされているトランスポートです。

```bash theme={null}
# 基本的な構文
claude mcp add --transport http <name> <url>

# 実際の例: Notion に接続
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Bearer トークン付きの例
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

`.mcp.json`、`~/.claude.json`、または `claude mcp add-json` で JSON を使用して MCP サーバーを設定する場合、`type` フィールドは `http` のエイリアスとして `streamable-http` を受け入れます。MCP 仕様ではこのトランスポートに `streamable-http` という名前を使用しているため、サーバードキュメントからコピーされた設定は変更なしで機能します。

`url` を持つが `type` を持たない JSON エントリは設定エラーです。Claude Code は `type` を持たないエントリを stdio サーバーとして読み込むためです。Claude Code はそのサーバーをスキップし、`MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry` と報告します。v2.1.202 より前では、Claude Code はこの設定ミスを `command: expected string, received undefined` と報告していました。

`--output-format stream-json` 実行では、Claude Code はスキップされた `--mcp-config` エントリを `system/init` イベントの [`mcp_server_errors` フィールド](/docs/ja/headless#stream-responses) でも報告するため、スクリプトはサーバーが読み込まれなかったことを検出できます。これには Claude Code v2.1.219 以降が必要です。

<h3 id="option-2-add-a-remote-sse-server">
  オプション 2: リモート SSE サーバーを追加する
</h3>

<Warning>
  SSE（Server-Sent Events）トランスポートは非推奨です。利用可能な場合は HTTP サーバーを使用してください。
</Warning>

一部のサービスは SSE エンドポイントのみを公開しています。これらを [HTTP サーバー](#option-1-add-a-remote-http-server) と同じ `claude mcp add --transport http <name> <url>` コマンドで追加してください。Claude Code は最初に HTTP トランスポートを試し、サーバーがそれを受け入れない場合は SSE に切り替わります。自動切り替えには Claude Code v2.1.265 以降が必要です。

以前のバージョンで、または SSE 経由で直接接続するには、代わりに `--transport sse` を渡してください。

```bash theme={null}
# 基本的な構文
claude mcp add --transport sse <name> <url>

# 実際の例: Asana に接続
claude mcp add --transport sse asana https://mcp.asana.com/sse

# 認証ヘッダー付きの例
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  オプション 3: ローカル stdio サーバーを追加する
</h3>

Stdio サーバーはマシン上のローカルプロセスとして実行されます。システムへの直接アクセスやカスタムスクリプトが必要なツールに最適です。

Claude Code は、生成されたサーバーの環境に `CLAUDE_PROJECT_DIR` を設定して、プロジェクトルートに設定します。これにより、サーバーは作業ディレクトリに依存することなくプロジェクト相対パスを解決できます。これは hooks が `CLAUDE_PROJECT_DIR` 変数で受け取るのと同じディレクトリです。サーバープロセス内からこれを読み取ります。例えば、Node では `process.env.CLAUDE_PROJECT_DIR`、Python では `os.environ["CLAUDE_PROJECT_DIR"]` です。

`CLAUDE_PROJECT_DIR` は安定したプロジェクトルートであり、セッション中に作業ディレクトリを追加または削除しても変わりません。ファイルシステムアクセスを許可されたディレクトリのセットに制限するサーバーは、代わりに MCP `roots/list` リクエストを実装する必要があります。Claude Code は `roots/list` にセッションの起動ディレクトリと、`--add-dir`、`/add-dir`、または `additionalDirectories` 設定で付与した [追加作業ディレクトリ](/docs/ja/permissions#working-directories) をすべて返します。Claude Code はそのセットが変わるときに `notifications/roots/list_changed` を送信します。v2.1.203 より前では、`roots/list` は起動ディレクトリのみを返し、Claude Code は `notifications/roots/list_changed` を送信していませんでした。

この変数はサーバーの環境に設定され、Claude Code 自体の環境には設定されないため、プロジェクトスコープの `.mcp.json` エントリまたはローカルまたはユーザースコープのサーバーエントリの `command` または `args` で `${VAR}` 展開を使用して参照するには、`${CLAUDE_PROJECT_DIR:-.}` などのデフォルトが必要です。プラグイン提供の MCP 設定は `${CLAUDE_PROJECT_DIR}` を直接置換し、デフォルトは必要ありません。

```bash theme={null}
# 基本的な構文
claude mcp add [options] <name> -- <command> [args...]

# 実際の例: Airtable サーバーを追加
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **重要: サーバー引数を `--` で区切る**

  Stdio サーバーの場合、`--`（ダブルダッシュ）は Claude 自体のオプション（`--transport`、`--env`、`--scope` など）をサーバーを実行するコマンドと引数から分離します。`--` の後のすべてはサーバーに変更されずに渡されます。

  例えば：

  * `claude mcp add --transport stdio myserver -- npx server` → `npx server` を実行します
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → 環境に `KEY=value` を設定して `python server.py --port 8080` を実行します

  `--` がない場合、Claude Code はサーバーのフラグ（上記の `--port` など）を独自のオプションとして解析しようとします。

  `--env` は複数の `KEY=value` ペアを受け入れます。サーバー名が `--env` の直後に来る場合、CLI は名前を別のペアとして読み込み、拒否するため、上記の例のように `--env` とサーバー名の間に少なくとも 1 つの別のオプションを配置してください。
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  オプション 4: リモート WebSocket サーバーを追加する
</h3>

WebSocket サーバーは永続的な双方向接続を保持し、Claude に予期しないイベントをプッシュするリモート MCP サーバーに適しています。サーバーがリクエストにのみ応答する場合は HTTP を使用してください。HTTP は OAuth と `claude mcp add --transport` フラグをサポートしますが、WebSocket はどちらもサポートしていないためです。

WebSocket サーバーを `.mcp.json` または `claude mcp add-json` で設定します。

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

`type: "ws"` エントリは `http` と同じ `url`、`headers`、`headersHelper`、`timeout`、`alwaysLoad` フィールドを受け入れます。認証はヘッダーのみなので、`headers` に静的トークンを渡すか、接続時に [`headersHelper`](#use-dynamic-headers-for-custom-authentication) で生成してください。`claude mcp add --transport` フラグは `ws` を受け入れません。

<h3 id="add-a-server-from-setup-instructions-written-for-another-client">
  別のクライアント向けに書かれたセットアップ指示からサーバーを追加する
</h3>

MCP サーバーは Claude Code に固有ではないため、サーバーのセットアップ指示は Claude Desktop、Cursor、または別の MCP クライアント向けに書かれている可能性があり、`claude mcp add` コマンドを提供していない場合があります。それでもサーバーを追加するには、これら 3 つのいずれかについて指示を確認してください。

* **URL**（`https://mcp.example.com/mcp` など）: サーバーはリモートです。
* **起動コマンド**（`npx -y @example/mcp-server` など）: サーバーはマシン上で実行されます。
* **`mcpServers` JSON ブロック**: 別のクライアントの設定ファイル向けに書かれた設定。

各々は [MCP サーバーのインストール](#installing-mcp-servers) の 4 つのオプションが取る入力の 1 つです。以下で持っている形状を見つけて、Claude Code が受け入れるコマンドに変換してください。各コマンドは `--scope project` または `--scope user` を追加しない限り、[ローカルスコープ](#local-scope) に書き込みます。

<h4 id="from-a-url">
  URL から
</h4>

URL はサーバーがリモートであることを意味します。`https://` エンドポイントの場合、`--transport http` で追加するか、指示が SSE を使用するエンドポイントを示している場合は [オプション 2](#option-2-add-a-remote-sse-server) に従ってください。`wss://` エンドポイントの場合、`--transport` は `ws` を受け入れないため、代わりに [オプション 4](#option-4-add-a-remote-websocket-server) を使用してください。

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

指示が API キーまたはトークンヘッダーも提供する場合、[オプション 1](#option-1-add-a-remote-http-server) に示されているように `--header` で渡してください。

<h4 id="from-an-npx-uvx-or-binary-command">
  `npx`、`uvx`、またはバイナリコマンドから
</h4>

起動コマンドはサーバーがローカル stdio プロセスとして実行されることを意味します。コマンド全体を `--` の後に配置して、Claude Code が `-y` などのフラグをサーバーを起動するコマンドに渡し、独自のオプションとして読み込まないようにします。指示が要求する環境変数を `--env` で渡します。サーバー名の後、`--` の前に渡します。

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

[オプション 3](#option-3-add-a-local-stdio-server) は `--` セパレータを完全にカバーしています。

<h4 id="from-an-mcpservers-json-block">
  `mcpServers` JSON ブロックから
</h4>

Claude Desktop などの別の MCP クライアント向けに書かれた `mcpServers` ブロックは、Claude Code が読み込むラッパーキーとエントリ形状を使用します。`claude mcp add-json` に `mcpServers` 内のオブジェクトを渡します。ラッパーではなく。2 つのエントリは最初に修復が必要です。

* **`type` のない `url`**: エンドポイントに一致するように `"type": "http"`、`"type": "sse"`、または `"type": "ws"` を追加してください。Claude Code は `type` を持たないエントリを stdio サーバーとして読み込むため、`type` のない `url` エントリは失敗します。
* **文字、数字、ハイフン、アンダースコア以外の文字を持つキー**: これらの文字のみを使用するサーバー名を選択してください。そうでない場合、キーはサーバー名です。

例えば、このブロック：

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

このコマンドになります：

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[JSON 設定から MCP サーバーを追加する](#add-mcp-servers-from-json-configuration) はシェルエスケープと `add-json` の `--scope` フラグをカバーしています。代わりにチームと共有するには、`--scope project` を追加するか、プロジェクトルートの `.mcp.json` の `mcpServers` の下にエントリを追加してコミットしてください。[プロジェクトスコープ](#project-scope) は Claude Code がそのファイルをどのように読み込み、承認するかをカバーしています。

各 `claude mcp add` と `claude mcp add-json` コマンドは `Added ...` 行を出力します。Claude Code が接続したことを確認するには、`claude mcp get <name>` を実行してください。[サーバーステータス](#server-status) はそれが表示するステータスと `.mcp.json` サーバーの承認ステップをカバーしています。

<h3 id="managing-your-servers">
  サーバーの管理
</h3>

設定されたら、これらのコマンドで MCP サーバーを管理できます。

```bash theme={null}
# すべての設定されたサーバーをリストする
claude mcp list

# 特定のサーバーの詳細を取得する
claude mcp get notion

# サーバーを削除する
claude mcp remove notion

# （Claude Code 内）サーバーステータスを確認する
/mcp
```

リモートサーバーを削除すると、Claude Code はそのサーバー用に保存した OAuth トークンとクライアント登録も削除します。

<h4 id="server-status">
  サーバーステータス
</h4>

`claude mcp add` は `Added ...` 行を出力して成功した追加を確認します。これは設定が書き込まれたことを意味します。`claude mcp list` はその後、`✔ Connected`、`! Needs authentication`、`✘ Failed to connect` などの各サーバーの横に健全性ステータスを表示します。失敗ステータスは Claude Code がそのサーバーに接続できなかったことを意味し、list コマンドが失敗したことではありません。

このリストのステータスは接続試行ではなく設定決定を報告するため、Claude Code はサーバーに接続せずにそれらを出力します。

* ``⏸ Pending approval (run `claude` to approve)``: まだ承認していない `.mcp.json` からのプロジェクトスコープサーバー。Claude Code はそれを `claude mcp list` と `claude mcp get <name>` の両方に表示します。対話的に `claude` を実行して、それを確認して承認してください。
* `✘ Rejected (see disabledMcpjsonServers in settings)`: [`disabledMcpjsonServers`](/docs/ja/settings-reference#disabledmcpjsonservers) エントリが拒否する `.mcp.json` サーバー。Claude Code はそれを `claude mcp get <name>` にのみ表示します。
* `⊘ Disabled for this project (re-enable via /mcp)`: プロジェクトの [`disabledMcpServers`](#disable-a-server-without-removing-it) リストが名前を付けるサーバー。Claude Code はそれを `claude mcp list` と `claude mcp get <name>` の両方に表示します。`/mcp` パネルからサーバーをオンに戻してください。v2.1.238 より前では、両方のコマンドが無効なサーバーに接続して健全性チェックを実行し、接続結果を報告していました。

WebSocket サーバーは `claude mcp list` 出力に表示されません。`claude mcp get <name>` または `/mcp` パネルを使用してそれらを確認してください。

<h4 id="project-server-approvals-and-workspace-trust">
  プロジェクトサーバーの承認とワークスペーストラスト
</h4>

v2.1.196 以降、`claude mcp list` と `claude mcp get` は `.mcp.json` 承認を、`claude` を実行してワークスペーストラストダイアログを受け入れるまでリポジトリにチェックインされていない設定ファイルからのみ読み込みます。クローンされたリポジトリは独自のサーバーを承認できません。プロジェクトの `.claude/settings.json` にコミットされた [`enableAllProjectMcpServers`](/docs/ja/settings-reference#enableallprojectmcpservers) または [`enabledMcpjsonServers`](/docs/ja/settings-reference#enabledmcpjsonservers) は信頼されていないフォルダでは無視され、サーバーは接続されて健全性チェックされる代わりに `⏸ Pending approval` のままです。

これらのソースからの承認は信頼されていないフォルダでも適用されます。

* ユーザー `~/.claude/settings.json`
* 管理設定
* `--settings` で渡された設定

Claude Code はまた、追跡されていない `.claude/settings.local.json` からの承認を適用しますが、ファイルが追跡されているかどうかを確認するために git を実行し、その確認は [信頼されたフォルダ](/docs/ja/permissions#project-allow-rules-and-workspace-trust) でのみ実行されます。信頼したことのないフォルダでは、Claude Code はトラストダイアログを待ってからファイルの承認を適用します。ただし、フォルダがユーザー自身の設定ホームである場合は除きます。ホームディレクトリ、または `.claude` を [`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars) として設定したディレクトリ。v2.1.207 より前では、Claude Code は信頼したことのないフォルダでも追跡されていない `.claude/settings.local.json` からの承認を適用していました。

任意の設定ファイルの `disabledMcpjsonServers` エントリはまだサーバーを拒否します。

<h4 id="server-status-detail">
  サーバーステータスの詳細
</h4>

`/mcp` で、そこにあるサーバーのメニューを含めて、[`/plugin`](/docs/ja/plugins/install) マネージャーで、以前使用したリモート HTTP または SSE サーバーは `cached` ステータス（`cached 2h ago · connects on first use · 5 tools` など）を表示できます。Claude Code は起動時に接続する代わりに、前のセッションで保存された検出キャッシュからサーバーのツールリストを読み込み、Claude Code はサーバーのツールの 1 つを Claude が最初に呼び出すときにサーバーを接続します。ツールは最初のメッセージから利用可能なため、何もする必要はありません。検出キャッシュとその `cached` ステータスには Claude Code v2.1.221 以降が必要です。

検出キャッシュはデフォルトではオフですが、段階的なロールアウトがアカウントに対して有効にしている場合を除きます。[`MCP_DISCOVERY_CACHE=1`](/docs/ja/env-vars) を設定してオンにするか、`0` を設定してロールアウトが有効にしている場合でもオフのままにしてください。v2.1.238 より前では、キャッシュはデフォルトでオンでした。

`/mcp` のサーバーのメニューの 2 つのアクションもそのサーバーのキャッシュエントリに影響します。

* **再接続**: `cached` サーバーで、Claude Code は最初のツール呼び出しではなく今すぐそれを接続し、エントリを保持します。接続されたまたは失敗したサーバーで、Claude Code はそれを再接続し、エントリも破棄します。
* **認証をクリア**: Claude Code はサーバーの認証を取り消し、エントリも破棄します。

エントリを破棄した後、Claude Code はキャッシュではなくサーバーからサーバーのツールリストを取得します。

サーバーのステータスが `✘ Failed to connect` の場合、`claude mcp list` はそのステータス行に失敗の詳細を追加し、`claude mcp get <name>` は `Issue:` 行に表示します。HTTP ステータスまたはエラーコード、およびサーバーが返したエラーテキスト。サーバーの詳細ビューは `/mcp` で同じサーバー報告テキストを `Issue:` 行に含めます。Claude Code はこの詳細から認証情報のようなテキストを編集し、展開されたサーバー URL を含めることはありません。これはシークレットを運ぶことができます。Claude Code は `✘ Connection error` ステータスに詳細を追加しません。例外テキストがそこに出力される可能性があるため、その URL を埋め込むことができます。v2.1.219 より前では、両方のコマンドはステータスコードまたはサーバーのエラーテキストなしで、単なる失敗ステータスのみを表示していました。

`/mcp` から認証を完了し、接続が HTTP ステータスまたはトランスポートエラーコードで失敗し続ける場合、Claude Code は試行後に出力するメッセージにそのコードとサーバーの URL の起点を追加します。起点はスキーム、ホスト、およびポート（URL が 1 つを名前付けする場合）です。例えば `https://mcp.example.com`。

* パスとクエリはそのメッセージに表示されません。
* ローカル、プロジェクト、またはユーザー [スコープ](#mcp-installation-scopes) のサーバー、または管理 MCP 設定のサーバーの場合、起点はその設定に書き込まれたホストを表示するため、ホストの `${VAR}` 参照はメッセージで展開されません。
* ステータスまたはエラーコードのない失敗の場合、Claude Code は起点なしでエラーテキストを表示します。

設定に空の `url` を持つリモートサーバーは `/mcp`、`claude mcp list`、[`/plugin`](/docs/ja/plugins/install) マネージャーで `not configured` として表示され、Claude Code はそれに接続しようとしません。プラグインは後で設定するコネクタのプレースホルダーエントリをこのように含めることができるため、Claude Code はそれをエラーまたはセットアップの問題として報告しません。サーバーの詳細ビューは `/mcp` で `No URL configured for this server` を読み込みます。接続するにはエントリの `url` を設定してください。v2.1.208 より前では、Claude Code は空の `url` を設定の問題として報告し、再接続を促していました。

<h4 id="configuration-warnings">
  設定警告
</h4>

Claude Code は以下の設定の問題について警告します。各エントリは Claude Code が何をチェックし、警告をクリアする方法を示しています。

* **隠れた空白**: Claude Code は MCP 設定値が隠れた先頭または末尾の空白を持つときに警告します。これはしばしば末尾の改行を持つトークンを貼り付けることから来ます。Claude Code は `command`、`url`、各 `args` エントリ、および `env` と `headers` の下の値とキー名をチェックします。Claude Code は警告を `claude mcp list` 出力と `/mcp` に表示し、影響を受けたフィールドに名前を付けます。例えば `Leading or trailing whitespace in: headers.Authorization`。Claude Code は空白をトリムしません。書き込まれたとおりに値を使用するため、設定を編集してそれを削除してください。
* **複数のスコープで同じ名前**: 異なるエンドポイントで複数の [スコープ](#mcp-installation-scopes) で同じサーバー名を定義する場合、Claude Code は `claude mcp list` 出力と `/mcp` で競合について警告します。Claude Code は OAuth サインインをエンドポイントごとに保存するため、1 つのプロジェクトで読み込まれる定義を認証すると、別の定義が読み込まれるプロジェクトで別にサインインする必要があります。必要なエンドポイントを保持し、他を `claude mcp remove <name> --scope <scope>` で削除してください。警告では、Claude Code は各スコープのエンドポイントを設定に書き込まれたとおりに引用します。[`${VAR}` 参照](#environment-variable-expansion-in-mcp-json) は展開されないため、API キーなどの解決された値を表示しません。
* **予約名**: Claude Code は `workspace`、`claude-in-chrome`、`computer-use`、`Claude Preview`、`Claude Browser` を含む組み込みサーバーの名前を予約しています。設定が予約名を持つサーバーを定義する場合、Claude Code はロード時にそれをスキップし、名前を変更するよう求める警告を表示します。`claude mcp add` は予約名を拒否します。`Claude Preview` と `Claude Browser` は両方とも [Claude Code デスクトップアプリのプレビューペイン](/docs/ja/desktop#preview-your-app) が使用する組み込みサーバーに名前を付けます。v2.1.205 より前では、`Claude Browser` は予約されていなかったため、ユーザー設定サーバーはその名前で登録できました。
* **環境変数の欠落**: サーバーの設定の [`${VAR}` 参照](#environment-variable-expansion-in-mcp-json) が設定されていない変数に名前を付け、`:-default` がない場合、Claude Code は `claude mcp list` 出力と `/mcp` で警告し、変数に名前を付けます。`${VAR}` テキストは展開されないままサーバーを読み込みます。変数を設定するか、`${VAR:-default}` フォールバックを追加してください。リモートサーバーの `url` と `headers` では、一部の認証情報変数 [空として読み込まれます](#credential-variables-that-read-as-empty) 代わりに、警告なしで。

<h4 id="tool-availability">
  ツール可用性
</h4>

`/mcp` パネルは各接続されたサーバーの横にツール数を表示し、ツール機能をアドバタイズするがツールを公開しないサーバーにフラグを立てます。

リクエストがバックグラウンドでまだ接続中のサーバーからのツールを必要とする場合、Claude はそのサーバーが接続するまで待機します。待機の方法は設定によって異なります。

* **[ツール検索](#scale-with-mcp-tool-search)（デフォルト）を使用**: 待機は `ToolSearch` 呼び出し内で発生します。
* **ツール検索なし**: Claude は代わりに `WaitForMcpServers` ツールを使用します。ツール検索なしの設定には、カスタム `ANTHROPIC_BASE_URL`、`ENABLE_TOOL_SEARCH=false`、Google Cloud の Agent Platform の Claude 4.5 世代より前のモデルが含まれます。
* **Microsoft Foundry [Azure でホストされたデプロイメント](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)**: Claude はツール検索パスで開始します。Claude Code は API からのデプロイメントのサーバー側拒否のみを検出するため、`WaitForMcpServers` ではなく。Claude Code がそのデプロイメントを [アップフロント読み込み](#scale-with-mcp-tool-search) に切り替えた後、サーバーが接続を完了するからのツールは Claude の次のリクエストで利用可能になります。

ツール検索が有効な場合、サーバーが Claude が作業中に接続を完了すると、Claude Code はサーバーのツール名を同じターンの次のリクエストで Claude にリストします。Claude はそれらのツールを検索して呼び出し、メッセージを待つことなく実行できます。

<h3 id="disable-a-server-without-removing-it">
  サーバーを削除せずに無効にする
</h3>

`/mcp` パネルでサーバーをオフに切り替えて、Claude Code がそれに接続するのを停止し、設定を失わないようにします。Claude Code はサーバーを `/mcp` にリストし、無効としてマークします。

サーバーを切り替えると、Claude Code はプロジェクトごとに `~/.claude.json` で選択を記録します。2 つのリストの 1 つで、互いに素なサーバーセットをカバーします。

* `disabledMcpServers`: ユーザー設定サーバー、プラグインサーバー、組織が [管理設定を通じて提供](/docs/ja/managed-mcp#provide-servers-through-managed-settings) するサーバー、Claude Code が [自身で取得](#how-connectors-reach-claude-code) する claude.ai コネクタ、およびデフォルトでオンの組み込みサーバーのオプトアウトリスト。Claude Code はここにリストするサーバーに接続しません。[Disable claude.ai connectors](#disable-claude-ai-connectors) で説明されているプロジェクトごとの `/mcp` トグルで claude.ai コネクタを無効にすると、Claude Code はそれをこのリストの下に表示名で書き込みます。例えば `claude.ai Slack`。
* `enabledMcpServers`: `computer-use` などのデフォルトでオフの組み込みサーバーのオプトインリスト。Claude Code はここにリストする場合にのみデフォルトオフサーバーに接続します。

Claude Code は各サーバーに対して 2 つのリストの 1 つを正確に参照するため、どちらのリストも他をオーバーライドしません。通常のサーバーを `enabledMcpServers` に追加するか、デフォルトオフの組み込みサーバーを `disabledMcpServers` に追加する場合、Claude Code はエントリを無視します。

`disabledMcpServers` と `enabledMcpServers` は [`enabledMcpjsonServers`](/docs/ja/settings-reference#enabledmcpjsonservers) と [`disabledMcpjsonServers`](/docs/ja/settings-reference#disabledmcpjsonservers) とは無関係です。これらはプロジェクトの `.mcp.json` ファイルで定義されたサーバーの承認を制御します。

<h3 id="mcp-client-runtimes">
  MCP クライアントランタイム
</h3>

Claude Code は 2 つのクライアントランタイムの 1 つを通じて MCP サーバーに接続します。v1 ランタイムは MCP TypeScript SDK 1.x に基づいています。v2 ランタイムは [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/) 上の同じコードで、MCP プロトコルリビジョン 2026-07-28 を追加します。このページの残りは両方のランタイムに適用されます。ただし、セクションが v2 ランタイムに名前を付ける場合を除きます。

Claude Code は起動するたびにランタイムを選択し、終了するまで保持します。[フィーチャーフラグを取得](/docs/ja/env-vars#features-that-need-feature-flag-fetching) するセッションでは、Claude Code v2.1.232 以降で v2 ランタイムを使用します。

フィーチャーフラグを取得しないセッションでは、Claude Code は Claude Code v2.1.274 以降でデフォルトで v2 ランタイムを使用します。

* Amazon Bedrock、Claude Platform on AWS、Google Cloud の Agent Platform、または Microsoft Foundry のセッション。ただし、Claude Code を埋め込むホストプラットフォームが [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ja/env-vars) を設定する場合を除きます
* [Claude apps gateway](/docs/ja/claude-apps-gateway) を通じてサインインしたセッション
* テレメトリまたはフィーチャーフラグ取得をオフにしたセッション。例えば `DISABLE_TELEMETRY` を使用

v2 では、Claude Code も：

* HTTP サーバーに新しいリビジョンをサポートするかどうかを尋ね、それをサポートするサーバーで使用します。フィーチャーフラグを取得するセッションで claude.ai コネクタサーバーにも尋ねます。Stdio サーバーに尋ねるか、すべてのセッションでコネクタサーバーに尋ねるには、[`MCP_PROTOCOL_NEGOTIATION`](/docs/ja/env-vars) を `auto` に設定してください。他のすべてのサーバーに v1 のように接続します。
* 新しいリビジョンのサーバーから [ストリームを保持](#notification-streams-on-the-v2-runtime) 上で `list_changed` 通知を受け取ります。
* 新しいリビジョンで接続する [チャネル](#push-messages-with-channels) サーバーを登録しません。そのリビジョンはチャネルメッセージを運ぶことができないためです。
* 予期しない発行者に名前を付ける認可応答の [MCP OAuth サインイン](#authenticate-with-remote-mcp-servers) に失敗します。

Anthropic は特定のサーバーを以前のプロトコルに保つか、Claude Code が取得するフィーチャーフラグでそのストリームをオフにすることができます。

ランタイムを自分で選択するには、[`MCP_SDK_GENERATION`](/docs/ja/env-vars) を `v1` または `v2` に設定してください。Claude Code が尋ねるかどうかを決定するには、[`MCP_PROTOCOL_NEGOTIATION`](/docs/ja/env-vars) を `auto` または `legacy` に設定してください。

<h3 id="dynamic-tool-updates">
  動的ツール更新
</h3>

Claude Code は MCP `list_changed` 通知をサポートし、MCP サーバーが切断して再接続することなく利用可能なツール、プロンプト、リソースを動的に更新できます。MCP サーバーが `list_changed` 通知を送信すると、Claude Code は自動的にそのサーバーから利用可能な機能をリフレッシュします。

リフレッシュリクエストが失敗する場合、Claude Code はサーバーの以前に検出されたツール、プロンプト、リソースを保持します。後のリフレッシュが成功するまで。v2.1.214 より前では、リフレッシュ中の一時的なエラーはサーバーのツール、プロンプト、リソースを空のリストに置き換えていました。

<h4 id="notification-streams-on-the-v2-runtime">
  v2 ランタイムの通知ストリーム
</h4>

[v2 ランタイム](#mcp-client-runtimes) では、Claude Code は新しいプロトコルリビジョンのサーバーから保持するストリーム上で `list_changed` 通知を受け取ります。ストリームが閉じると、Claude Code はそれを再度開きます。2 つの制限があります。

* **ストリームが 10 秒以内に再度閉じる**: Claude Code はそれを最大 3 回再度開き、その接続に対して停止します。
* **ストリームが 10 秒以上開いたままで、その後閉じる**。サーバーレスホストへのストリームが一般的に行うように。1 時間に 5 回再度開いた後、Claude Code は次のものまで約 6 時間待機します。

ストリームが再度開くまで、サーバーの最後に取得したツール、プロンプト、リソースを保持します。変更をより早く取得するには、`/mcp` からサーバーを再接続してください。

<h3 id="automatic-reconnection">
  自動再接続
</h3>

Claude Code はセッション中にドロップするリモートサーバーを再接続し、一時的なエラーの後に HTTP または SSE サーバーの最初の接続を再試行します。Stdio サーバーはローカルプロセスであり、Claude Code は自動的にそれらを再接続しません。

<h4 id="mid-session-drops-of-a-remote-server">
  リモートサーバーのセッション中のドロップ
</h4>

Claude Code は指数バックオフでドロップされたリモートサーバーを再接続します。最大 5 回の試行。1 秒の遅延で開始し、毎回それを 2 倍にします。表示内容は Claude Code の実行方法によって異なります。

* **対話的セッション**: `/mcp` は Claude Code が再接続している間、サーバーを保留中として表示します。5 回の失敗した試行の後、Claude Code はサーバーを失敗としてマークするか、サーバーが再度認可する必要がある場合は認証が必要として。`/mcp` から手動で再試行できます。
* **[`claude -p`](/docs/ja/headless) 実行と [Agent SDK](/docs/ja/agent-sdk/overview) セッション**: Claude Code は同じスケジュールで再接続し、試行を表示する `/mcp` パネルはありません。

<h4 id="failed-first-connections">
  失敗した最初の接続
</h4>

HTTP または SSE サーバーの最初の接続が 5xx レスポンス、接続拒否、タイムアウトなどの一時的なエラーで失敗する場合、Claude Code は最大 3 回再試行します。接続がまだ失敗する場合、Claude Code はサーバーを失敗としてマークします。Claude Code はこのように起動時と、セッション中にサーバーが追加されるときに再試行します。これには Claude Code が [クラウドセッション](/docs/ja/claude-code-on-the-web) に設定から追加するサーバーと、Agent SDK の [`setMcpServers()`](/docs/ja/agent-sdk/typescript) で追加するサーバーが含まれます。

Claude Code はこれらの場合には再試行しません。

* WebSocket サーバーの最初の接続
* 認証またはエラーが見つからない。設定変更を解決する必要があるため。[`headersHelper`](#use-dynamic-headers-for-custom-authentication) がサーバーの `Authorization` ヘッダーの唯一のソースである場合、Claude Code は認証エラーを再試行します。ヘルパーを各試行で再実行し、新しい認証情報を取得できるためです。

<h4 id="failed-discovery-requests">
  失敗した検出リクエスト
</h4>

サーバーが接続した後、Claude Code は `tools/list`、`prompts/list`、`resources/list` などの機能検出リクエストを送信します。Claude Code は一時的なネットワークまたはサーバーエラーの後、短いバックオフで最大 3 回それらのリクエストを再試行します。認証エラー、4xx レスポンス、またはリクエストタイムアウトは再試行しません。

<h4 id="how-claude-learns-that-a-server-failed">
  Claude がサーバーが失敗したことを学ぶ方法
</h4>

Claude Code が接続に失敗した設定されたサーバーについて Claude に伝えるかどうかは [ツール検索](#scale-with-mcp-tool-search)（デフォルトではオン）に依存します。

* ツール検索を使用して、Claude Code は Claude にどのサーバーが失敗したか、その接続エラーを伝えるため、Claude は応答で接続失敗を報告します。Claude Code は一致するツールを見つけない `ToolSearch` 結果に同じ情報を含めます。
* [ツール検索なしの設定](#configure-tool-search) では、Claude Code は失敗したサーバー接続を Claude に報告しません。

<h3 id="push-messages-with-channels">
  チャネルでメッセージをプッシュする
</h3>

MCP サーバーはまた、CI 結果、監視アラート、チャットメッセージなどの外部イベントに Claude が反応できるようにメッセージをセッションに直接プッシュできます。これを有効にするには、サーバーが `claude/channel` 機能を宣言し、起動時に `--channels` フラグでオプトインします。[チャネル](/docs/ja/channels) を使用して公式にサポートされているチャネルを使用するか、[チャネルリファレンス](/docs/ja/channels-reference) を参照して独自に構築してください。

[v2 ランタイム](#mcp-client-runtimes) では、[`MCP_PROTOCOL_NEGOTIATION`](/docs/ja/env-vars) を `auto` に設定し、チャネルサーバーが MCP プロトコルリビジョン 2026-07-28 をネゴシエートする場合、チャネルメッセージを配信できないため、Claude Code はそれをチャネルとして登録しません。変数を設定しないままにするか、`legacy` に設定して、stdio サーバーを以前のハンドシェイクに保ちます。

<Tip>
  ヒント：

  * `-s` または `--scope` フラグを使用して、設定が保存される場所を指定します。
    * `local`（デフォルト）: 現在のプロジェクトでのみ利用可能
    * `project`: `.mcp.json` ファイルを通じてプロジェクト内のすべてのユーザーと共有
    * `user`: すべてのプロジェクト全体で利用可能
  * `-e` または `--env` フラグで環境変数を設定します（例えば、`-e KEY=value`）
  * `--transport` と `--header` フラグは `-t` と `-H` 短形式も受け入れます
  * `MCP_TIMEOUT` 環境変数を使用して MCP サーバー起動タイムアウトを設定します（例えば、`MCP_TIMEOUT=10000 claude` は 10 秒のタイムアウトを設定します）
  * ミリ秒単位で `.mcp.json` エントリにそのサーバーの `timeout` フィールドを追加して、サーバーごとのツール実行タイムアウトを設定します。例えば `"timeout": 600000` は 10 分です。これは `MCP_TOOL_TIMEOUT` 環境変数をそのサーバーのみでオーバーライドします
  * Claude Code は MCP ツール出力が 10,000 トークンを超えるときに警告を表示し、デフォルトで出力を 25,000 トークンに制限します。制限を上げるには、`MAX_MCP_OUTPUT_TOKENS` 環境変数を設定します（例えば、`MAX_MCP_OUTPUT_TOKENS=50000`）。警告しきい値は固定です。[MCP 出力制限と警告](#mcp-output-limits-and-warnings) を参照してください
  * `/mcp` を使用して、OAuth 2.0 認証が必要なリモートサーバーで認証します
</Tip>

サーバーごとの `timeout` はツール呼び出しごとのハードウォールクロック制限であり、サーバーからの進捗通知はそれを拡張しません。1000 未満の値は無視され、`MCP_TOOL_TIMEOUT` にフォールスルーするか、その変数が設定されていない場合は約 28 時間のデフォルトにフォールスルーします。HTTP、SSE、または [claude.ai コネクタ](/docs/ja/mcp#use-mcp-servers-from-claude-ai) サーバーの場合、サーバーの最初の応答バイトまでの各リクエストをカバーする 2 つ目の、リクエストごとのタイマーもあります。Claude Code はそのタイマーを 3 つの値の最大値に設定します。60 秒、サーバーに適用されるツールタイムアウト、`MCP_TIMEOUT`。設定されていない `MCP_TOOL_TIMEOUT` の 28 時間デフォルトはその比較に入らず、60 秒未満の値はタイマーを短縮しません。Stdio と WebSocket サーバーにはリクエストごとのタイマーがありません。

少なくとも 1000 のサーバーごとの `timeout` は、以下で説明されるアイドルタイムアウトのフロアとしても機能します。Claude Code はそのサーバーのツール呼び出しをサーバーごとの `timeout` より早くアイドルのために中止しません。Claude Code v2.1.203 以降が必要です。

応答も進捗通知も送信しない MCP サーバーへのツール呼び出しは、ウォールクロック制限を待つ代わりにエラーで中止されます。アイドルタイムアウトには Claude Code v2.1.187 以降が必要です。IDE サーバーと SDK インプロセスサーバーを除くすべてのサーバータイプに適用されます。アイドルウィンドウは HTTP、SSE、WebSocket、[claude.ai コネクタ](#use-mcp-servers-from-claude-ai) サーバーの場合は 5 分、stdio サーバーの場合は 30 分にデフォルト設定されます。v2.1.203 より前では、stdio サーバーはアイドルタイムアウトから除外されていました。

[`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/ja/env-vars) 環境変数をミリ秒単位で設定してアイドルウィンドウを変更するか、`0` に設定してチェックを無効にしてください。

これらのタイムアウトは呼び出しがどのくらい実行できるかを制限し、常にどのくらいブロックするかではありません。2 分を超えて実行される主会話呼び出しは、最初にバックグラウンドタスクに移動します。[長いツール呼び出しの自動バックグラウンド化](#automatic-backgrounding-of-long-tool-calls) を参照してください。

<h3 id="automatic-backgrounding-of-long-tool-calls">
  長いツール呼び出しの自動バックグラウンド化
</h3>

主会話の MCP ツール呼び出しが 2 分後も実行中の場合、セッションをブロックする代わりにバックグラウンドタスクに移動します。Claude はタスク ID をすぐに受け取り、作業を続け、結果は呼び出しが解決するときにタスク通知として到着します。自動バックグラウンド化には Claude Code v2.1.212 以降が必要です。

タスクは [`/tasks`](/docs/ja/commands#all-commands) に表示され、そこで停止することもでき、セッションを終了しても存続しません。呼び出しがバックグラウンドで実行されている間、呼び出しごとの制限は引き続き適用されます。サーバーごとの `timeout` または [`MCP_TOOL_TIMEOUT`](/docs/ja/env-vars) で設定されたウォールクロック制限、および [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/ja/env-vars) で設定されたアイドルタイムアウト。

[`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](/docs/ja/env-vars) 環境変数をミリ秒単位で設定してしきい値を変更するか、`0` に設定して自動バックグラウンド化をオフにしてください。`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` を `1` に設定すると、それもオフになり、他のすべてのバックグラウンドタスク機能も同様です。

一部の呼び出しはバックグラウンドに移動しません。

* [サブエージェント](/docs/ja/sub-agents) からの呼び出し。Claude Code はメイン会話呼び出しのみをバックグラウンド化します
* IDE サーバーへの呼び出し
* [非対話モード](/docs/ja/headless) での呼び出し。ただし `CLAUDE_AUTO_BACKGROUND_TASKS` が `1` に設定されている場合を除きます。1 回限りの実行は結果が到着する前に終了する可能性があるため

開いている [エリシテーションダイアログ](#respond-to-mcp-elicitation-requests) を待つ呼び出しは、ダイアログが開いている間はバックグラウンド化されません。サーバーは遅いのではなく入力を待ってブロックされているため、Claude Code はダイアログが閉じるまで移動を延期します。

<h3 id="plugin-provided-mcp-servers">
  プラグイン提供の MCP サーバー
</h3>

[プラグイン](/docs/ja/plugins/overview) は、プラグインを有効にするときにツールと統合を提供する MCP サーバーをバンドルできます。プラグイン MCP サーバーはユーザー設定サーバーと同じように機能します。

**プラグイン MCP サーバーの動作方法**：

* プラグインはプラグインルートの `.mcp.json` または `plugin.json` にインラインで MCP サーバーを定義します
* プラグインを有効にすると、Claude Code は自動的にその MCP サーバーを起動します
* Claude Code は手動で設定された MCP ツールと並んでプラグイン MCP ツールを提供します
* プラグインサーバーを追加および削除するには、プラグインをインストールまたはアンインストールします。`/mcp` コマンドではなく。[インストールされたプラグインサーバーをオフに切り替える](#disable-a-server-without-removing-it) ことはできます。これにより、Claude Code がそれに接続するのを停止し、プラグインを削除することなく

**プラグイン MCP 設定の例**：

プラグインルートの `.mcp.json` で：

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

または `plugin.json` にインラインで：

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**プラグイン MCP 機能**：

* **自動ライフサイクル**: サーバーはこれらのポイントで接続および切断されます。
  * セッション起動時、Claude Code は有効なプラグインのサーバーを自動的に接続します。`/mcp` では、以前使用したリモート（HTTP または SSE）プラグインサーバーは [`cached` ステータス](#server-status-detail) を代わりに表示できます。Claude Code は Claude が最初にそのツールの 1 つを呼び出すときに接続します
  * セッション中にプラグインを有効または無効にする場合、Claude Code はその変更が適用されるときにその MCP サーバーを接続または切断します。[プラグイン変更を再起動なしで適用](/docs/ja/plugins/cli-reference#reload-plugins) はいつかを説明しています。対話的なターミナルのないセッションでは、`/reload-plugins` はプラグイン MCP サーバーを接続または切断しません。これらの変更は次のセッションで有効になります
  * リロードするとき、Claude Code は設定が変更されていないプラグインサーバーのライブ接続を保持し、Agent SDK から [セッションの MCP サーバーリストを置き換える](/docs/ja/agent-sdk/typescript#mcpsetserversresult) ときに同じことを行います。それらに名前を付けずに
  * v2.1.246 以降で [`/cd`](/docs/ja/permissions#move-the-session-to-another-directory) でセッションを移動するとき、Claude Code は新しいディレクトリの設定が有効にするプラグインのサーバーを接続し、有効でなくなったプラグインのサーバーを切断するため、移動後に `/reload-plugins` を実行する必要はありません
  * [クラウドセッション](/docs/ja/claude-code-on-the-web) では、まだ接続されていないプラグインサーバーへの MCP 呼び出し（アイドルセッションが起動した直後など）は、サーバーをオンデマンドで開始し、接続を待ちます
* **パスプレースホルダー**: `${CLAUDE_PLUGIN_ROOT}` はプラグインのインストールディレクトリに解決され、`${CLAUDE_PLUGIN_DATA}` はその [永続状態](/docs/ja/plugins/components#path-variables-and-persistent-data) ディレクトリに解決され、`${CLAUDE_PROJECT_DIR}` は安定したプロジェクトルートに解決されます。置換は以下に適用されます。
  * `stdio` サーバー: `command`、`args`、`env`
  * `http`、`sse`、`ws` サーバー: `url`、`headers`、`headersHelper`。v2.1.195 より前では、`headersHelper` はプレースホルダーをリテラル文字列として渡していました
* **ユーザー環境アクセス**: 手動で設定されたサーバーと同じ環境変数へのアクセス
* **複数のトランスポートタイプ**: stdio、SSE、HTTP、WebSocket トランスポートのサポート。ただし、トランスポートサポートはサーバーによって異なる場合があります

プラグインサーバーは `/mcp` に表示され、プラグインから来ることを示すインジケータが付きます。

**プラグイン MCP ツール名**：

プラグインにバンドルされた MCP サーバーからのツールは、呼び出し可能な名前にプラグイン名とサーバーキーの両方を含めます。完全な形式は `mcp__plugin_<plugin-name>_<server-name>__<tool-name>` です。`A-Z`、`a-z`、`0-9`、`_`、`-` の外の任意の文字は `_` に置き換えられます。`my-plugin` という名前のプラグインにバンドルされた `database-tools` サーバーの場合、`query` ツールは以下のように呼び出し可能です。

```
mcp__plugin_my-plugin_database-tools__query
```

[権限ルール](/docs/ja/permissions)、スキルの `allowed-tools` リスト、[サブエージェントの `tools` フィールド](/docs/ja/sub-agents#available-tools)、または [フック マッチャー](/docs/ja/hooks#match-mcp-tools) でツールを参照するときに、この完全な名前を使用してください。裸のサーバーキー（`mcp__database-tools__.*` など）に対して書かれたフック マッチャーは、プラグインにバンドルされたサーバーに対して発火しません。

サーバー自体は `plugin:<plugin-name>:<server-name>`（`plugin:my-plugin:database-tools` など）のスコープ付き名前で登録されます。設定されたサーバー名が予想される場所（[`mcp_tool` フックの `server` フィールド](/docs/ja/hooks#mcp-tool-hook-fields) など）でその名前を使用してください。

プラグインで MCP サーバーをバンドルする詳細については、[プラグインコンポーネントリファレンス](/docs/ja/plugins/components#mcp-servers) を参照してください。

<h2 id="mcp-installation-scopes">
  MCP インストールスコープ
</h2>

MCP サーバーは 3 つのスコープで設定できます。選択するスコープは、サーバーがロードされるプロジェクトと、設定がチームと共有されるかどうかを制御します。管理者は、[マネージド設定](#managed-mcp-configuration)を通じてすべてのユーザーに対してサーバーをデプロイまたは提供することもできます。

| スコープ                     | ロード対象       | チームと共有       | 保存場所                   |
| ------------------------ | ----------- | ------------ | ---------------------- |
| [ローカル](#local-scope)     | 現在のプロジェクトのみ | いいえ          | `~/.claude.json`       |
| [プロジェクト](#project-scope) | 現在のプロジェクトのみ | はい、バージョン管理経由 | プロジェクトルートの `.mcp.json` |
| [ユーザー](#user-scope)      | すべてのプロジェクト  | いいえ          | `~/.claude.json`       |

<h3 id="local-scope">
  ローカルスコープ
</h3>

ローカルスコープはデフォルトです。ローカルスコープのサーバーは、追加したプロジェクトでのみロードされ、あなたにプライベートなままです。Claude Code は `~/.claude.json` のそのプロジェクトのパスの下に保存するため、同じサーバーは他のプロジェクトに表示されません。個人開発サーバー、実験的な設定、またはバージョン管理に含めたくない認証情報を持つサーバーにはローカルスコープを使用してください。

<Note>
  MCP サーバーの「ローカルスコープ」という用語は、一般的なローカル設定とは異なります。MCP ローカルスコープのサーバーは `~/.claude.json`（ホームディレクトリ）に保存されますが、一般的なローカル設定は `.claude/settings.local.json`（プロジェクトディレクトリ内）を使用します。設定ファイルの場所の詳細については、[設定](/docs/ja/settings#where-settings-live)を参照してください。
</Note>

```bash theme={null}
# ローカルスコープのサーバーを追加する（デフォルト）
claude mcp add --transport http stripe https://mcp.stripe.com

# ローカルスコープを明示的に指定する
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

コマンドは現在のプロジェクトのエントリを `~/.claude.json` に書き込みます。以下の例は、`/path/to/your/project` から実行した場合の結果を示しています：

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  プロジェクトスコープ
</h3>

プロジェクトスコープのサーバーは、プロジェクトのルートディレクトリの `.mcp.json` ファイルに設定を保存することで、チーム間のコラボレーションを可能にします。プロジェクトスコープのサーバーを追加すると、Claude Code は自動的にこのファイルを作成または更新して、適切な設定構造を使用します。`.mcp.json` をバージョン管理にチェックインして、チームのすべてのメンバーが同じ MCP ツールとサービスを取得するようにしてください。

```bash theme={null}
# プロジェクトスコープのサーバーを追加する
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

結果の `.mcp.json` ファイルは標準化された形式に従います：

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

セキュリティ上の理由から、Claude Code はインタラクティブセッションで `.mcp.json` ファイルからプロジェクトスコープのサーバーを使用する前に承認を求めます。これらの承認選択をリセットするには、`claude mcp reset-project-choices` を実行してください。

`claude -p` 実行、[Agent SDK](/docs/ja/headless)セッション、および[クラウドセッション](/docs/ja/claude-code-on-the-web)では、Claude Code はそのプロンプトを表示できません。プロジェクトスコープのサーバーを確認なしでロードします。Claude Code はまた、ユーザー設定またはマネージド設定で [`skipDangerousModePermissionPrompt`](/docs/ja/settings-reference#skipdangerousmodepermissionprompt) が設定された `bypassPermissions` モードで開始したセッションではプロンプトをスキップします。とにかくサーバーを除外するには：

* [`disabledMcpjsonServers`](/docs/ja/settings-reference#disabledmcpjsonservers)に追加します。これはすべての権限モードでそれをブロックします。
* [`--setting-sources`](/docs/ja/cli-reference#cli-flags)またはSDKの `settingSources` オプションでプロジェクト設定全体を除外します。
* [`--strict-mcp-config`](/docs/ja/cli-reference#cli-flags)でセッションを開始します。Claude Code は `--mcp-config` で渡した MCP サーバーのみを使用します。Claude Code がロードしていないプロジェクトスコープのサーバーの承認プロンプトをスキップするには、Claude Code v2.1.246 以降が必要です。v2.1.246 より前では、厳密なセッションでもそれらの承認を待機していたため、バックグラウンドセッションは起動時に待機していました。マネージド MCP ファイルの下でフラグが何をするかについては、[マネージド mcp.json での排他的制御](/docs/ja/managed-mcp#exclusive-control-with-managed-mcp-json)を参照してください。

[プロジェクトサーバーの承認とワークスペーストラスト](#project-server-approvals-and-workspace-trust)は、リポジトリにコミットされた承認がワークスペーストラストとどのように相互作用するかについて説明しています。

<h3 id="user-scope">
  ユーザースコープ
</h3>

ユーザースコープのサーバーは `~/.claude.json` に保存され、クロスプロジェクトのアクセス可能性を提供し、マシン上のすべてのプロジェクト全体で利用可能になりながら、ユーザーアカウントにプライベートなままです。このスコープは、個人的なユーティリティサーバー、開発ツール、または異なるプロジェクト全体で頻繁に使用するサービスに適しています。

```bash theme={null}
# ユーザーサーバーを追加する
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  スコープの階層と優先順位
</h3>

同じサーバーが複数の場所で定義されている場合、Claude Code はそれに 1 回接続し、最も優先度の高いソースからの定義を使用します。そのソースからのサーバーエントリ全体が使用されます。フィールドはスコープ全体でマージされません。

1. ローカルスコープ
2. プロジェクトスコープ
3. ユーザースコープ
4. [プラグイン提供サーバー](/docs/ja/plugins/components#mcp-servers)
5. [claude.ai コネクタ](#use-mcp-servers-from-claude-ai)

3 つのスコープは名前で重複を照合します。プラグインとコネクタはエンドポイントで照合するため、上記のサーバーと同じ URL またはコマンドを指すものは重複として扱われます。

組織が [`managedMcpServers`](/docs/ja/managed-mcp#provide-servers-through-managed-settings)マネージド設定を通じて提供するサーバーは、これらすべての上にランクされるため、それらの 1 つがそれを複製する場合、Claude Code は組織の定義に接続します。Claude Code v2.1.259 以降が必要です。

[Desktop アプリの Code タブ](/docs/ja/desktop#mcp-servers-from-the-claude-desktop-chat-app)でローカルセッションを開く場合、`~/.claude.json`（ユーザースコープ）のトップレベルと `.mcp.json` に同じ stdio サーバー名がある場合、Code タブは `~/.claude.json` 定義を使用します。

<h3 id="environment-variable-expansion-in-mcp-json">
  `.mcp.json` での環境変数の展開
</h3>

Claude Code は `.mcp.json` ファイルの環境変数の展開をサポートしており、チームが設定を共有しながら、マシン固有のパスと API キーなどの機密値の柔軟性を維持できます。

<h4 id="supported-syntax">
  サポートされている構文
</h4>

* `${VAR}` - 環境変数 `VAR` の値に展開されます
* `${VAR:-default}` - `VAR` が設定されている場合は `VAR` に展開され、そうでない場合はデフォルトを使用します

<h4 id="expansion-locations">
  展開場所
</h4>

環境変数は以下で展開できます：

* `command` - サーバー実行可能ファイルのパス
* `args` - コマンドライン引数
* `env` - サーバーに渡される環境変数
* `url` - HTTP サーバータイプの場合
* `headers` - HTTP サーバー認証の場合

<h4 id="example-with-variable-expansion">
  変数展開を使用した例
</h4>

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

<h4 id="unset-variables-without-a-default">
  デフォルトなしの未設定変数
</h4>

参照される環境変数が設定されておらず、デフォルト値がない場合、設定はまだロードされます。Claude Code はそのサーバーに対して `claude mcp list` 出力で欠落変数の警告を報告し、展開されていない `${VAR}` テキストをそのまま使用します。変数を設定するか、`:-default` フォールバックを追加して、サーバーが意図した値で起動するようにしてください。リモートサーバーの `url` と `headers` では、一部の認証情報変数は[空として読み込まれます](#credential-variables-that-read-as-empty)。警告はありません。

<h4 id="credential-variables-that-read-as-empty">
  空として読み込まれる認証情報変数
</h4>

リモートサーバーの `url` と `headers` では、Claude Code は環境から認証情報変数を空として読み込みます。これは、プロジェクトの `.mcp.json` またはプラグインが Claude Code またはクラウドプロバイダーの認証情報をそれが指定するサーバーに送信するのを防ぎます。`Bearer ${ANTHROPIC_AUTH_TOKEN}` と書いた場合、サーバーは認証情報なしで `Bearer ` を受け取り、通常は `401` でリクエストを拒否します。Claude Code はそれを接続失敗として報告します。

対象となる名前は：

* Claude Code 独自の認証情報（`ANTHROPIC_API_KEY` や `ANTHROPIC_AUTH_TOKEN` など）
* クラウドプロバイダーの認証情報（`AWS_BEARER_TOKEN_BEDROCK` など）
* 環境が保持する他の認証情報（`HTTPS_PROXY` や `NPM_TOKEN` など）

対象となる名前は、変数を設定しているかどうかに関わらず空として読み込まれ、それに対する `:-default` フォールバックは無視されます。`ANTHROPIC_BASE_URL` などのプロバイダーベース URL はまだ展開されるため、`"url": "${ANTHROPIC_BASE_URL}/mcp"` は機能します。ただし、URL の値自体がユーザー名とパスワードなどの認証情報を埋め込んでいない限りです。

`API_KEY` などこのセットの外にある名前は、書かれたとおりに展開されます。サーバーに対象となる認証情報の 1 つを提供するには、それを自分の名前の変数にコピーして、その名前を参照してください。

リモートサーバーの `url` または `headers` が設定した対象変数を参照する場合、Claude Code はデバッグログ行でそれを指定します。行を読むには、`claude --debug-file /tmp/claude-debug.log` を実行して、そのファイルで `never expanded toward a remote server` を検索してください。

<h4 id="how-references-appear-in-/mcp-and-cli-output">
  `/mcp` と CLI 出力での参照の表示方法
</h4>

ローカル、プロジェクト、またはユーザー[スコープ](#mcp-installation-scopes)内のサーバーの場合、以下のサーフェスは `${VAR}` 参照を解決された値ではなく名前で表示します：

* サーバーの `/mcp` 詳細ビューの URL またはコマンドライン
* `claude mcp list` と `claude mcp get` の出力

`/mcp` 詳細ビューは Claude Code v2.1.268 以降でこの方法で参照を表示します。

組織が `managedMcpServers` 設定を通じて提供するサーバーの場合、これらのサーフェスは[URL のホストのみ](/docs/ja/managed-mcp#what-users-can-see-and-change)を表示します。

接続が失敗した場合に `claude mcp list`、`claude mcp get`、および `/mcp` が何を表示するかを確認するには、[サーバーステータスの詳細](#server-status-detail)を参照してください。

<h2 id="practical-examples">
  実践的な例
</h2>

<h3 id="example-connect-to-github-for-code-reviews">
  例：コードレビューのために GitHub に接続する
</h3>

GitHub のリモート MCP サーバーは、ヘッダーとして渡される GitHub 個人アクセストークンで認証します。取得するには、[GitHub トークン設定](https://github.com/settings/personal-access-tokens)を開き、Claude が操作したいリポジトリへのアクセス権を持つ新しいきめ細かいトークンを生成してから、サーバーを追加します：

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

`YOUR_GITHUB_PAT` を個人アクセストークンに置き換えてください。`claude mcp add` コマンドは認証情報を検証せずに設定を保存するため、ここではプレースホルダー値が受け入れられますが、サーバーは後で接続に失敗します。接続を確認するには、`/mcp` を実行し、サーバーが `connected` と表示されていることを確認してください。認証情報が不正なサーバーは `failed` と表示され、失敗の詳細には、サーバーが返した HTTP ステータス（401 など）が含まれます。

その後、GitHub で作業します：

```text wrap theme={null}
PR #456 をレビューして改善を提案してください
```

```text wrap theme={null}
見つけたバグの新しい課題を作成してください
```

```text wrap theme={null}
自分に割り当てられているすべてのオープン PR を表示してください
```

<h3 id="example-query-your-postgresql-database">
  例：PostgreSQL データベースをクエリする
</h3>

[DBHub](https://github.com/bytebase/dbhub)（`@bytebase/dbhub` パッケージ）は、`--dsn` で渡す接続文字列を通じて Claude をリレーショナルデータベースに接続する MCP サーバーです。Claude が実行するクエリがデータを変更できないように、接続文字列で読み取り専用データベースユーザーを使用してください：

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

サーバーが起動することを確認するには、`/mcp` を実行し、`db` が `connected` と表示されていることを確認してください。

その後、データベースを自然に照会します：

```text wrap theme={null}
今月の総収益はいくらですか？
```

```text wrap theme={null}
orders テーブルのスキーマを表示してください
```

```text wrap theme={null}
過去 90 日間に購入していない顧客を検索してください
```

<h2 id="authenticate-with-remote-mcp-servers">
  リモート MCP サーバーで認証する
</h2>

多くのクラウドベースの MCP サーバーは認証が必要です。Claude Code は安全な接続のために OAuth 2.0 をサポートしています。

Claude Code は、サーバーが `401 Unauthorized` または `403 Forbidden` で応答するとき、リモートサーバーが認証を必要としていることをマークします。Claude Code が表示する内容はサーバーによって異なります。

* サインインしていないサーバーの場合、どちらのステータスコードでも `/mcp` でフラグが立てられるため、OAuth フローを完了できます。
* [claude.ai コネクタ](#use-mcp-servers-from-claude-ai)の場合、claude.ai がセッショントークンを拒否することによる `401` はコネクタにフラグを立てません。コネクタを再認可してもログインを修正できないためです。Claude Code は代わりに[セッショントークン拒否状態](/docs/ja/errors#claude-ai-rejected-the-session-token)を表示します。
* `Authorization` ヘッダーを設定したサーバーの場合、`headers` または [`headersHelper`](#use-dynamic-headers-for-custom-authentication) を通じて、接続中の `401` または `403` はサーバーにフラグを立てません。修正する認証情報は設定した認証情報だからです。Claude Code は代わりに接続が失敗したことを報告します。設定したヘッダーが `${VAR}` 参照から設定されている場合は、その変数が Claude Code が[空として読み込む](#credential-variables-that-read-as-empty)変数の 1 つであるかどうかを確認してください。
* [クラウドセッションに配信されたコネクタ](#how-connectors-reach-claude-code)の場合、Claude Code はサインインフローを実行しません。セッションのプロキシが claude.ai で付与した認可を使用してコネクタに認証するためです。そこでコネクタが再度認可が必要な場合、セッションからではなく [claude.ai/customize/connectors](https://claude.ai/customize/connectors) で再接続してください。

既にサインインした OAuth サーバーへのリクエストが `401 Unauthorized` を返すとき、Claude Code は保存されたトークンをリフレッシュし、再接続して、リクエストを 1 回再試行します。その再試行も失敗した場合にのみ、`/mcp` でサーバーにフラグを立てます。v2.1.206 より前は、ネットワークエラーなどの一時的な理由でトークンリフレッシュが失敗した場合、リフレッシュトークンがまだ有効であっても、OAuth サーバーは残りのセッション中、認証が必要としてフラグが立てられていました。

サーバーが保存されたリフレッシュトークンを拒否するとき、Claude Code は直ちに `/mcp` を指す通知を表示します。`/mcp` を開き、サーバーで **Re-authenticate** を選択して、次のツール呼び出しが失敗する前に再度サインインしてください。

`WWW-Authenticate` ヘッダーを返すカスタムサーバーは、その認可サーバーを指し、他のリモートサーバーと同じ自動検出を取得します。

Claude Code は、1 つ以上の設定されたサーバーが認証を必要とするときにスタートアップ通知も表示するため、`/mcp` を開いて認証が必要なサーバーを検出する必要がありません。この通知には Claude Code v2.1.193 以降が必要です。Claude Code からサインインできるサーバーのみをカウントします。v2.1.218 より前は、claude.ai で接続されていない [claude.ai コネクタ](#use-mcp-servers-from-claude-ai)もカウントされていました。これらは claude.ai 設定からのみ接続できます。

通知は各サーバーを 1 回アナウンスし、そのサーバーが接続して再度サインインが必要になるまで、後の起動時のカウントから除外します。`/mcp` は依然としてサインインが必要なすべてのサーバーをリストします。

非対話モードでは `/mcp` パネルがないため、Claude Code は OAuth フローを実行できません。v2.1.196 以降、[ツール検索](#scale-with-mcp-tool-search)が有効な（デフォルト）`claude -p` または Agent SDK 実行中に設定されたサーバーが認証を必要とするとき、Claude Code はサーバーのツールが認可されるまで利用できないことを Claude に伝えます。Claude はサーバーが設定されていないかのように応答する代わりに、サインインが必要なサーバーに名前を付けることができます。対話セッションから `/mcp` または `claude mcp login <name>` でサインインを完了してください。

サーバーに `headers.Authorization` を設定し、サーバーがそのヘッダーを拒否する場合、Claude Code は OAuth にフォールバックする代わりに接続が失敗したことを報告します。トークンが MCP エンドポイントに対して有効であることを確認するか、ヘッダーを削除して OAuth フローを使用してください。

<Steps>
  <Step title="認証が必要なサーバーを追加する">
    [MCP クイックスタート](/docs/ja/mcp-quickstart#connect-a-server-that-requires-sign-in)で既に `sentry` サーバーを追加した場合、このステップをスキップしてください。同じサーバー名で同じスコープで `claude mcp add` を再度実行すると、`MCP server sentry already exists in local config` で失敗します。それ以外の場合は、以下を実行してください。

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Claude Code 内で /mcp コマンドを使用する">
    Claude Code で、以下のコマンドを使用します。

    ```text wrap theme={null}
    /mcp
    ```

    その後、ブラウザーのステップに従ってログインしてください。
  </Step>
</Steps>

<Tip>
  ヒント：

  * 認証トークンは安全に保存され、自動的にリフレッシュされます
  * `/mcp` メニューの「Clear authentication」を使用してアクセスを取り消します
  * ブラウザーが自動的に開かない場合は、提供された URL をコピーして手動で開いてください
  * 認証後、ブラウザーリダイレクトが接続エラーで失敗する場合は、ブラウザーのアドレスバーから完全なコールバック URL を貼り付けて、Claude Code に表示される URL プロンプトに入力してください
  * OAuth 認証は HTTP サーバーで機能します
</Tip>

<h3 id="authenticate-from-the-command-line">
  コマンドラインから認証する
</h3>

v2.1.186 から、`claude mcp login <name>` は設定されたサーバーの OAuth フローをシェルから直接実行するため、セッション内の `/mcp` パネルを開く必要がありません。

```bash theme={null}
claude mcp login sentry
```

後で保存された認証情報をクリアするには、`claude mcp logout <name>` を実行してください。

v2.1.191 以降、このコマンドは SSH セッション中やディスプレイサーバーのない Linux など、ローカルブラウザーが利用できない場合を検出し、ブラウザーを開こうとする代わりに認可 URL を出力します。ローカルマシンで URL を開き、ブラウザーのアドレスバーから完全なリダイレクト URL をプロンプトに貼り付けてください。このコマンドは貼り付けステップのために対話型ターミナルが必要なため、`ssh -t` で接続してください。ローカルブラウザーが検出された場合でも URL プロンプトを強制するには、`--no-browser` を渡してください。

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  固定 OAuth コールバックポートを使用する
</h3>

一部の MCP サーバーは、事前に登録された特定のリダイレクト URI が必要です。デフォルトでは、Claude Code は OAuth コールバック用にランダムに利用可能なポートを選択します。`--callback-port` を使用してポートを固定し、`http://localhost:PORT/callback` 形式の事前登録されたリダイレクト URI と一致させてください。Claude Code v2.1.229 でのサインインがリダイレクト URI の不一致で失敗する場合は、[事前設定された OAuth 認証情報を使用する](#use-pre-configured-oauth-credentials)の下のバージョンノートを参照してください。

`--callback-port` は単独で（動的クライアント登録を使用）または `--client-id` と一緒に（事前設定された認証情報を使用）使用できます。

```bash theme={null}
# 動的クライアント登録を使用した固定コールバックポート
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  事前設定された OAuth 認証情報を使用する
</h3>

一部の MCP サーバーは、動的クライアント登録による自動 OAuth セットアップをサポートしていません。「Incompatible auth server: does not support dynamic client registration」のようなエラーが表示される場合、サーバーは事前設定された認証情報が必要です。Claude Code は、動的クライアント登録の代わりにクライアント ID メタデータドキュメント（CIMD）を使用するサーバーもサポートし、これらを自動的に検出します。自動検出が失敗する場合は、サーバーの開発者ポータルを通じて OAuth アプリを登録してから、サーバーを追加するときに認証情報を提供してください。

<Steps>
  <Step title="サーバーで OAuth アプリを登録する">
    サーバーの開発者ポータルを通じてアプリを作成し、クライアント ID とクライアントシークレットをメモしてください。

    多くのサーバーはリダイレクト URI も必要とします。その場合は、ポートを選択し、`http://localhost:PORT/callback` 形式でリダイレクト URI を登録してください。次のステップで `--callback-port` と同じポートを使用してください。

    v2.1.229 では、Claude Code は代わりに `http://127.0.0.1:PORT/callback` を送信していました。登録されたリダイレクト URI と完全に一致するサーバーは、リダイレクト URI の不一致でサインインを拒否していました。Claude Code v2.1.231 は `localhost` 形式を復元しました。v2.1.229 で復旧するには、Claude Code をアップグレードするか、一時的にサーバーの登録されたリダイレクト URI に `http://127.0.0.1:PORT/callback` 形式を追加してください。
  </Step>

  <Step title="認証情報を使用してサーバーを追加する">
    以下のいずれかの方法を選択してください。`--callback-port` に使用されるポートは、利用可能な任意のポートにできます。前のステップで登録したリダイレクト URI と一致する必要があります。

    <Tabs>
      <Tab title="claude mcp add">
        `--client-id` を使用してアプリのクライアント ID を渡します。`--client-secret` フラグはマスクされた入力でシークレットをプロンプトします。

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        JSON 設定に `oauth` オブジェクトを含め、`--client-secret` を別のフラグとして渡します。

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json（コールバックポートのみ）">
        動的クライアント登録を使用しながらポートを固定するには、クライアント ID なしで `--callback-port` を使用します。

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / 環境変数">
        環境変数を通じてシークレットを設定して、対話型プロンプトをスキップします。

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Claude Code で認証する">
    Claude Code で `/mcp` を実行し、ブラウザーログインフローに従ってください。
  </Step>
</Steps>

<Tip>
  ヒント：

  * クライアントシークレットは、設定ではなく、システムキーチェーン（macOS）または認証情報ファイルに安全に保存されます
  * クライアントシークレットはサーバーを追加するときにのみ設定できます。`claude mcp login` または `/mcp` から認証するとき、Claude Code は保存されたシークレットを使用し、プロンプトを表示したり `MCP_CLIENT_SECRET` を読み込んだりしません
  * 後でシークレットを追加または変更するには、`claude mcp remove <name>` でサーバーを削除してから、`--client-secret` と同じ `--scope` で再度追加してください
  * サーバーがシークレットのないパブリック OAuth クライアントを使用する場合は、`--client-secret` なしで `--client-id` のみを使用してください
  * これらのフラグは HTTP および SSE トランスポートにのみ適用されます。stdio サーバーには効果がありません
  * `claude mcp get <name>` を使用して、OAuth 認証情報がサーバーに対して設定されていることを確認してください
</Tip>

<h3 id="override-oauth-metadata-discovery">
  OAuth メタデータ検出をオーバーライドする
</h3>

特定の OAuth 認可サーバーメタデータ URL を指して、デフォルト検出チェーンをバイパスしてください。MCP サーバーの標準エンドポイントがエラーになるとき、または内部プロキシを通じて検出をルーティングしたいときに `authServerMetadataUrl` を設定してください。デフォルトでは、Claude Code は最初に `/.well-known/oauth-protected-resource` で RFC 9728 保護リソースメタデータをチェックし、次に `/.well-known/oauth-authorization-server` で RFC 8414 認可サーバーメタデータにフォールバックします。

`.mcp.json` のサーバー設定の `oauth` オブジェクトで `authServerMetadataUrl` を設定してください。

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

URL は `https://` を使用する必要があります。メタデータ URL の `scopes_supported` は、アップストリームサーバーがアドバタイズするスコープをオーバーライドします。

<h3 id="restrict-oauth-scopes">
  OAuth スコープを制限する
</h3>

`oauth.scopes` を設定して、認可フロー中に Claude Code がリクエストするスコープをピン留めしてください。これは、アップストリーム認可サーバーがより多くのスコープをアドバタイズするときに、MCP サーバーをセキュリティチームによって承認されたサブセットに制限するサポートされた方法です。値は RFC 6749 §3.3 の `scope` パラメーター形式と一致する単一のスペース区切り文字列です。

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` は `authServerMetadataUrl` とサーバーが `/.well-known` で検出するスコープの両方より優先されます。MCP サーバーがリクエストされたスコープセットを決定できるようにするには、設定を解除のままにしてください。

v2.1.196 以降、`oauth.scopes` が設定されていない場合、Claude Code はサーバーの `WWW-Authenticate` ヘッダーまたは保護されたリソースメタデータによって提供されるスコープをリクエストし、どちらも提供しない場合は `scope` パラメーターを送信しません。自動的に検出された認可サーバーメタデータから完全な `scopes_supported` カタログをリクエストしなくなりました。そのカタログをリクエストすると、管理者のみまたはテンプレートスコープをアドバタイズするアイデンティティプロバイダーが `invalid_scope` エラーで認可リクエストを拒否していました。設定された `authServerMetadataUrl` から取得されたメタデータは、その `scopes_supported` をリクエストされたスコープとして提供します。

認可サーバーが `scopes_supported` で `offline_access` をアドバタイズする場合、Claude Code はそれをピン留めされたスコープに追加して、新しいブラウザーサインインなしでアクセストークンをリフレッシュできるようにします。

サーバーが後でツール呼び出しに対して 403 `insufficient_scope` を返す場合、呼び出しは[追加の権限が必要](/docs/ja/errors#mcp-server-needs-you-to-sign-in-again)というメッセージで失敗し、サーバーが要求するスコープに名前を付けます。サーバーは `/mcp` で認証が必要として表示されます。

そのスコープがピン留めされた `oauth.scopes` にない場合は、それを追加してから `/mcp` を実行し、サーバーを再度認証してください。Claude Code はサーバーが名前を付けたスコープではなく、ピン留めされたスコープをリクエストするため、それを追加せずに再度認証する場合、取得するトークンはまだそれを欠いています。

<h3 id="use-dynamic-headers-for-custom-authentication">
  カスタム認証に動的ヘッダーを使用する
</h3>

MCP サーバーが Kerberos、短命トークン、または内部 SSO などの OAuth 以外の認証スキームを使用する場合は、`headersHelper` を使用して接続時にリクエストヘッダーを生成してください。Claude Code はコマンドを実行し、その出力を接続ヘッダーにマージします。

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

コマンドはインラインにすることもできます。

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**要件：**

* コマンドは文字列キーと値のペアの JSON オブジェクトを stdout に書き込む必要があります
* Claude Code はコマンドをシェルで実行し、10 秒後にそれを放棄します
* Claude Code は[サーバーを設定した場所](#where-the-helper-runs)によってコマンドの作業ディレクトリを選択するため、スクリプトを絶対パスとして指定するか、`PATH` に配置してください
* 動的ヘッダーは同じ名前の静的 `headers` をオーバーライドします

Claude Code は各接続時にヘルパーを新たに実行します。セッション開始時と再接続時に、[プロジェクトおよびローカルスコープサーバーの信頼ルール](#trust-a-folder-before-its-headershelper-runs)がそれを実行できるようにします。結果をキャッシュしないため、スクリプトはトークンの再利用を担当します。

ツール呼び出しが `401 Unauthorized` または `403 Forbidden` を返す場合、Claude Code は自動的に同じルールの下でヘルパーを再実行し、新しいヘッダーで再接続し、呼び出しを 1 回再試行します。その再試行も失敗した場合にのみ、Claude Code は `/mcp` でサーバーを認証が必要としてマークします。

ヘルパーの出力に `Authorization` ヘッダーが含まれている場合、Claude Code はその認証情報をサーバーの認証として使用し、サーバーの OAuth にフォールバックしません。

サーバーが接続中にヘルパーの認証情報を拒否する場合、Claude Code はサーバーを認証が必要としてマークする代わりに、接続が失敗したことを報告します。ヘルパーが返す認証情報を修正してから、`/mcp` から再接続してヘルパーを再実行してください。

Claude Code はヘルパーを実行するときに、これらの環境変数を設定します。

| 変数                            | 値                                                                                |
| :---------------------------- | :------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | MCP サーバーの名前                                                                      |
| `CLAUDE_CODE_MCP_SERVER_URL`  | MCP サーバーの URL                                                                    |
| `CLAUDE_PLUGIN_ROOT`          | プラグインのルートディレクトリ。[プラグイン](/docs/ja/plugins/components#mcp-servers)がサーバーを提供する場合にのみ設定されます |

これらを使用して、複数の MCP サーバーに対応する単一のヘルパースクリプトを作成してください。

プラグイン提供の `headersHelper` はプラグインの [`${user_config.*}`](/docs/ja/plugins/manifest-reference#user-configuration)値を参照できません。コマンドはシェルを通じて実行されるためです。Claude Code はサーバーを設定ミスとして[エラー](/docs/ja/errors#plugin-command-references-user-config)で報告し、値を置換しません。代わりに、シェル解析されない `headers` フィールドに `${user_config.KEY}` を配置するか、ヘルパースクリプトに設定ファイルから値を読み込ませてください。v2.1.207 より前は、`headersHelper` は `${user_config.*}` 値を置換していました。

<h4 id="where-the-helper-runs">
  ヘルパーが実行される場所
</h4>

Claude Code は、サーバーを宣言する設定から `headersHelper` コマンドの作業ディレクトリを選択します。Claude が Bash で実行する `cd` はそれを移動しません。[`/cd`](/docs/ja/permissions#move-the-session-to-another-directory)はセッションのプライマリ作業ディレクトリから実行されるサーバーのみを移動します。以下の各行は、`headersHelper` コマンドの相対パスが解決される対象のディレクトリを示します。

| サーバーを設定した場所                                                                                                                                           | 作業ディレクトリ                                                            |
| :---------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------ |
| [プラグイン](/docs/ja/plugins/components#mcp-servers)                                                                                                           | プラグインのルートディレクトリ。Claude Code v2.1.195 以降が必要です                        |
| プロジェクト `.mcp.json` または [ローカルスコープ](#local-scope)サーバー                                                                                                   | サーバーが宣言されているプロジェクトディレクトリ                                            |
| プロジェクト内のエージェントファイル、SDK の `mcpServers` オプションまたは `setMcpServers()` メソッドからのサーバー、または [`--mcp-config`](/docs/ja/cli-reference)                                  | セッションの[プライマリ作業ディレクトリ](/docs/ja/permissions#working-directories)          |
| [ユーザースコープ](#user-scope)、[管理 MCP](/docs/ja/managed-mcp)、[claude.ai コネクタ](#use-mcp-servers-from-claude-ai)、またはプロジェクト外のエージェントファイル（`--add-dir` ディレクトリからのものを含む） | 設定ディレクトリ `~/.claude`（[`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars)を設定していない場合） |

v2.1.238 より前は、Claude Code はユーザースコープ、管理、および claude.ai コネクタサーバーのヘルパー、およびプロジェクト外のエージェントファイルのヘルパーも、それを開始したディレクトリから実行していました。

<h4 id="which-variables-a-helper-can-read">
  ヘルパーが読み取ることができる変数
</h4>

リポジトリまたはプラグインが提供する `headersHelper` は、書いていないコマンドなため、Claude Code はそれを環境から認証情報変数なしで実行します（`ANTHROPIC_API_KEY` など）。サーバーを設定した場所によって、これが適用されるかどうかが決まります。

* **削除される**：プロジェクト `.mcp.json` またはプラグイン内のサーバー、およびプロジェクトからのエージェントファイルまたは `--add-dir` ディレクトリからのインラインサーバー
* **削除されない**：[ユーザー](#user-scope)または[ローカルスコープ](#local-scope)、[管理 MCP](/docs/ja/managed-mcp)、[claude.ai コネクタ](#use-mcp-servers-from-claude-ai)、または SDK または [`--mcp-config`](/docs/ja/cli-reference) から提供されるサーバー、および `~/.claude/agents/` からのインラインサーバー、管理設定から、または `--agents` で渡されるもの

Git の `GIT_CONFIG_KEY_<n>` 変数を除き、Claude Code は環境から `TOKEN`、`SECRET`、`PASSWORD`、`KEY`、または `AUTH` を含む名前のような認証情報のように見える名前を持つすべての変数を削除します。したがって、`ANTHROPIC_API_KEY` と `MY_REGISTRY_TOKEN` の両方が削除されます。Claude Code は、`ANTHROPIC_CUSTOM_HEADERS` などの名前がそのパターンに従わない固定リストの認証情報変数も削除します。

これがヘルパーに適用される場合は、スクリプトにファイルまたは認証情報ストアから認証情報を読み込ませてください。サーバーの `url` が[これらの変数のいずれかを展開する](#environment-variable-expansion-in-mcp-json)場合、ヘルパーが受け取る `CLAUDE_CODE_MCP_SERVER_URL` 値にはその部分が `REDACTED` に置き換えられています。

<h4 id="trust-a-folder-before-its-headershelper-runs">
  headersHelper が実行される前にフォルダーを信頼する
</h4>

Claude Code は `headersHelper` を任意のシェルコマンドとして実行します。プロジェクト `.mcp.json` 内のサーバーまたは[ローカルスコープ](#local-scope)の場合、サーバーが宣言されているプロジェクトディレクトリの[信頼ダイアログ](/docs/ja/permissions#project-allow-rules-and-workspace-trust)を受け入れた後にのみ、ヘルパーを実行します。v2.1.238 より前は、`claude -p` または SDK セッションはこれらのヘルパーを信頼をチェックせずに実行していました。対話セッションは親フォルダーを信頼した後に 1 回実行していました。

* **カウントされない信頼**：親フォルダーの信頼、および `claude -p` または SDK セッションが[設定ファイルのフック](/docs/ja/permissions#what-runs-before-you-trust-a-folder)に対して取得する自動信頼
* **フォルダーを信頼するまで**：Claude Code は静的 `headers` のみでサーバーを接続します。`claude -p` または SDK セッションでは、1 つの [`headersHelper not run`](/docs/ja/errors#headershelper-not-run) 行を stderr に出力し、信頼を付与する方法を示します。
* **ダイアログなしで信頼する**：`~/.claude.json` で `projects["<path>"].hasTrustDialogAccepted` を `true` に設定してください。`<path>` は、[プロジェクト許可ルールとワークスペース信頼](/docs/ja/permissions#project-allow-rules-and-workspace-trust)が Claude Code がキーを設定するフォルダーです。

Claude Code は[エージェントファイル](/docs/ja/sub-agents#scope-mcp-servers-to-a-subagent)で宣言されたサーバーに同じルールを適用し、そのエージェントファイルがどこから来たかをチェックします。プロジェクト内のファイルの場合は `.claude/agents/` ディレクトリから、または `--add-dir` ディレクトリから。[そのプロジェクトまたはディレクトリ自体を信頼する](/docs/ja/permissions#what-runs-before-you-trust-a-folder)まで、Claude Code はサーバーをロードしません。したがって、そのヘルパーも実行されません。

<h2 id="add-mcp-servers-from-json-configuration">
  JSON 設定から MCP サーバーを追加する
</h2>

MCP サーバーの JSON 設定がある場合は、直接追加できます：

<Steps>
  <Step title="JSON から MCP サーバーを追加する">
    ```bash theme={null}
    # 基本的な構文
    claude mcp add-json <name> '<json>'

    # 例：JSON 設定を使用して HTTP サーバーを追加する
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # 例：JSON 設定を使用して stdio サーバーを追加する
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # 例：事前設定された OAuth 認証情報を使用して HTTP サーバーを追加する
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="サーバーが追加されたことを確認する">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  ヒント：

  * JSON がシェルで適切にエスケープされていることを確認してください
  * JSON は MCP サーバー設定スキーマに準拠する必要があります
  * `--scope user` を使用して、プロジェクト固有のサーバーの代わりにユーザー設定にサーバーを追加できます
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  Claude Desktop から MCP サーバーをインポートする
</h2>

Claude Desktop で MCP サーバーを既に設定している場合は、それらをインポートできます：

<Steps>
  <Step title="Claude Desktop からサーバーをインポートする">
    ```bash theme={null}
    # 基本的な構文 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="インポートするサーバーを選択する">
    コマンドを実行した後、インポートするサーバーを選択できる対話的なダイアログが表示されます。
  </Step>

  <Step title="サーバーがインポートされたことを確認する">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

`claude mcp` コマンドで追加されたサーバー名には、文字、数字、ハイフン、アンダースコアのみを含めることができます。Claude Desktop はその制限を適用しないため、スペースなどの他の文字を含む名前を持つ Claude Desktop サーバーはインポートできません。インポートは拒否された各名前を報告し、選択した他のサーバーはインポートします。v2.1.205 より前では、最初の無効な名前がインポートを停止し、選択されたサーバーは追加されませんでした。

<Tip>
  ヒント：

  * この機能は macOS と Windows Subsystem for Linux（WSL）でのみ機能します
  * これらのプラットフォームの標準的な場所から Claude Desktop 設定ファイルを読み取ります
  * `--scope user` フラグを使用してサーバーをユーザー設定に追加します
  * インポートされたサーバーは、名前に文字、数字、ハイフン、アンダースコアのみが含まれている場合、Claude Desktop と同じ名前を保持します。Claude Code は他の文字を含む名前を持つサーバーを報告し、スキップします
  * 同じ名前のサーバーが既に存在する場合、数値サフィックスが付与されます（例：`server_1`）
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  claude.ai から MCP サーバーを使用する
</h2>

[claude.ai](https://claude.ai) アカウントで Claude Code にログインしている場合、claude.ai に追加した MCP サーバー（[connectors](https://claude.com/docs/connectors) として知られています）は Claude Code で自動的に利用可能になります。

<Steps>
  <Step title="claude.ai で MCP サーバーを設定する">
    [claude.ai/customize/connectors](https://claude.ai/customize/connectors) でサーバーを追加します。Team および Enterprise プランでは、管理者のみがサーバーを追加できます。
  </Step>

  <Step title="MCP サーバーを認証する">
    claude.ai で必要な認証ステップを完了します。
  </Step>

  <Step title="Claude Code でサーバーを表示および管理する">
    Claude Code で、次のコマンドを使用します。

    ```text wrap theme={null}
    /mcp
    ```

    claude.ai からのサーバーはリストに表示され、claude.ai から来たことを示すインジケーターが付きます。
  </Step>
</Steps>

Claude Code は、組織が claude.ai で認証を管理している場合、`/mcp` および [`/plugin`](/docs/ja/plugins/install) マネージャーで connector を `managed` としてマークします。Managed ステータスは、Claude Code が connector に接続する方法や、組織の [tool controls](#organization-controls-on-connector-tools) を適用する方法を変更しません。

まだサインインしたことのない Connector は、claude.ai セクションの最後にある `Show unused connectors` 行の背後に折りたたまれているため、組織がプロビジョニングしたリストがパネルを満たしません。その行を選択して展開します。以前にサインインした Connector は、現在再認証が必要な場合でも表示されたままです。

claude.ai からの Connector は、アクティブな [authentication method](/docs/ja/authentication#authentication-precedence) が claude.ai サブスクリプションログインである場合にのみ取得されます。以前に `/login` を実行した場合でも、次の場合は読み込まれません。

* `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN`、または `apiKeyHelper` がアクティブ
* Amazon Bedrock や Google Cloud の Agent Platform などのサードパーティプロバイダーがアクティブ
* `ANTHROPIC_PROFILE`、フェデレーション変数、またはアクティブな [Anthropic profile](/docs/ja/authentication#anthropic-profiles-and-federation-credentials) が認証情報を提供
* `CLAUDE_CODE_OAUTH_TOKEN` が [`claude setup-token`](/docs/ja/authentication#generate-a-long-lived-token) からのトークンを保持している（モデルリクエストのみを実行できます）

`/mcp` が追加した connector をリストしない場合は、`/status` を実行してどの authentication method がアクティブであるかを確認します。その環境変数を設定解除し、`apiKeyHelper` 設定を削除するか、[profile をオフに切り替え](/docs/ja/authentication#anthropic-profiles-and-federation-credentials)、`/login` を実行して claude.ai アカウントを選択します。

一時的なネットワーク問題により、セッション開始時に connector リストが読み込まれない場合、Claude Code はバックグラウンドで最大 3 回再試行し、再試行が成功すると connector が表示されます。まだ表示されていない場合は、Claude Code を再起動してリストを再度取得します。

`/mcp` が connector を `connected · session token rejected` として表示する場合、またはその詳細ビューが [`claude.ai rejected the session token`](/docs/ja/errors#claude-ai-rejected-the-session-token) を表示する場合、claude.ai は Claude Code ログインからのトークンを拒否しました。通常、ログインの有効期限が切れて更新できなかったためです。connector を再度認証しても、connector 自体の認証が拒否されたわけではないため、この状態はクリアされません。クリアするには、以下を実行します。

1. `/login` を実行して再度サインインします。
2. `/mcp` から connector を再度接続します。

v2.1.222 より前では、Claude Code は connector を認証が必要として マークしていました。これを認証しても解決しませんでした。

Claude Code で追加したサーバーは、同じ URL を指す claude.ai connector よりも [precedence](#scope-hierarchy-and-precedence) を持ちます。これが発生すると、`/mcp` は connector を hidden としてリストし、代わりに connector を使用する場合は重複を削除する方法を表示します。

Microsoft 365、Gmail、Google Calendar などの一部の Anthropic ホスト connector は、アップストリーム ID プロバイダーが claude.ai が登録したリダイレクト URL のみを受け入れるため、Claude Code からのローカル OAuth をサポートしていません。`claude mcp add` または `.mcp.json` で追加したサーバーがこれらのホストの 1 つを指し、`/mcp` から、または `claude mcp login` でサインインすると、Claude Code は [`is Anthropic-hosted and doesn't support local OAuth`](/docs/ja/errors#anthropic-hosted-and-doesnt-support-local-oauth) を表示し、代わりに [claude.ai/customize/connectors](https://claude.ai/customize/connectors) でサービスを接続するよう指示します。

`claude mcp remove <name>` でエントリを削除し、claude.ai でサービスを接続した後、connector は Claude Code に自動的に表示されます。

<h3 id="how-connectors-reach-claude-code">
  Connector が Claude Code に到達する方法
</h3>

claude.ai connector を管理する設定は、セッションが実行される場所によって異なります。セッションの種類によっては、claude.ai 自体から connector を取得するセッションのみがあるためです。以下の各行は、1 種類のセッションで connector がどのように到達するか、およびそこで何が connector を制御するかを示しています。デスクトップアプリの [WSL sessions](/docs/ja/desktop-wsl#what-works-in-a-wsl-session) には、connector がまだ利用できないため、行がありません。

| セッションが実行される場所                                                                                                        | Connector がどのように到達するか        | 何が connector を管理するか                                                                                                                                           |
| :------------------------------------------------------------------------------------------------------------------- | :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Terminal、[VS Code](/docs/ja/vs-code)、[JetBrains](/docs/ja/jetbrains)、および [Agent SDK](/docs/ja/agent-sdk/claude-code-features) セッション | Claude Code が claude.ai から取得 | このセクションの設定および [managed MCP configuration](/docs/ja/managed-mcp)                                                                                                    |
| [Cloud sessions](/docs/ja/claude-code-on-the-web)                                                                         | クラウドホストが渡す                   | claude.ai 組織設定、および [allowlist と denylist](/docs/ja/managed-mcp#policy-based-control-with-allowlists-and-denylists) 設定がセッションに到達し、セッションを実行するホスト上の `managed-mcp.json` |
| [desktop app](/docs/ja/desktop) のローカルおよび SSH セッション                                                                        | デスクトップアプリが in-process で配信    | 組織の [connector tool controls](#organization-controls-on-connector-tools) の `blocked` エントリ                                                                     |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors)、`ENABLE_CLAUDEAI_MCP_SERVERS`、および [`allowAllClaudeAiMcps`](/docs/ja/settings-reference#allowallclaudeaimcps) は最初の行のみに作用します。Claude Code が自体で取得する connector です。他の 2 つの行は次の点で異なります。

* **Cloud sessions**: セッションに到達する `allowedMcpServers` および `deniedMcpServers` エントリ（例えば [server-managed settings](/docs/ja/server-managed-settings) を通じて）は、配信された connector もフィルタリングします。セッションのプロキシは各 connector の URL を書き直すため、connector 自体の URL 用に書かれた `serverUrl` パターンはそれと一致しません。自己ホスト環境で URL allowlist と共に配信された connector を許可するには、[Connector traffic leaves your network](/docs/ja/self-hosted-environments-deploy#connector-traffic-leaves-your-network) の下にリストされている `serverUrl` エントリを追加します。セッションを実行するホスト（例えば [self-hosted runner host](/docs/ja/self-hosted-environments-configuration#mcp-servers)）に `managed-mcp.json` が存在する場合、Claude Code は配信された connector をドロップします。`allowAllClaudeAiMcps` を設定するかどうかに関わらず。
* **Desktop app local and SSH sessions**: デスクトップアプリは connector を in-process `type: "sdk"` サーバーとして登録し、MCP 設定または `managed-mcp.json` は到達しません。ユーザーは [claude.ai/customize/connectors](https://claude.ai/customize/connectors) で connector を切断することで、自分のセッションから connector を除外します。組織は connector の [tools](#organization-controls-on-connector-tools) をブロックするか、[Claude Code in the desktop app](/docs/ja/desktop#admin-console-controls) を完全にオフにします。

<h3 id="organization-controls-on-connector-tools">
  Connector tools の組織コントロール
</h3>

組織は [claude.ai connectors](https://claude.com/docs/connectors) に tool ごとのコントロールを設定できます。Claude Code はこれらの設定をスタートアップ時に読み取り、ローカルで実行します。ただし、デスクトップアプリの [local and SSH sessions](#how-connectors-reach-claude-code) では除きます。そこでは、デスクトップアプリは connector を配信する前に `blocked` tool を保留し、`ask` 設定は Claude Code に到達しないため、セッションの通常の [permission rules](/docs/ja/permissions) をそれらの tool に適用し、すべての呼び出しでプロンプトを表示する代わりに。Claude Code が connector 自体を取得するセッションでは、`/mcp` を実行して、各 tool に適用される設定を connector で確認します。

* **Tool が `ask` に設定されている場合**: Claude Code は理由 `Your organization requires approval for this tool` ですべての呼び出しでプロンプトを表示します。プロンプトは `acceptEdits`、`auto`、および `bypassPermissions` [permission modes](/docs/ja/permissions#permission-modes) でも表示され、選択を記憶するオプションは提供されません。tool と一致する [Allow rules](/docs/ja/permissions) はプロンプトをスキップしません。プロンプトを表示しない `dontAsk` モードでは、Claude Code は呼び出しを代わりに拒否します。
* **Tool が `blocked` に設定されている場合**: Claude Code は Claude がそれを見る前に tool をフィルタリングするため、tool リストに表示されません。デスクトップアプリと claude.ai チャットは同じ `blocked` 設定を適用するため、Claude はそこでも tool を使用できず、デスクトップアプリのセッションから tool を保留しながらチャットで利用可能に保つことはできません。デスクトップアプリは tool がすべてブロックされている connector をスキップします。

<h3 id="disable-claude-ai-connectors">
  claude.ai connector を無効にする
</h3>

Claude Code は [`disableClaudeAiConnectors`](/docs/ja/settings-reference#disableclaudeaiconnectors) を、[自体で取得する](#how-connectors-reach-claude-code) connector のみに適用し、クラウドホストまたはデスクトップアプリが配信する connector には適用しません。取得する connector をオフにするには、任意の設定スコープで設定を `true` に設定します。

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

この設定は any-source-true セマンティクスを使用します。任意の設定ソースの `true` が優先されます。チェックインされたプロジェクト `.claude/settings.json` は Claude Code が自体で取得する connector をリポジトリから除外できますが、プロジェクトレベルの `false` は、ユーザーまたはポリシーレベルの `true` が無効にした connector を再度有効にすることはできません。`--mcp-config` を通じて明示的に渡されたサーバーは影響を受けません。

`ENABLE_CLAUDEAI_MCP_SERVERS` 環境変数を `false` に設定することもできます。これは現在のシェルセッションに対して同じ効果があります。

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

すべての claude.ai connector をブロックする代わりに個別の claude.ai connector をブロックするには、名前または URL パターンで [`deniedMcpServers`](/docs/ja/managed-mcp) に追加します。例えば、`serverName` エントリ `"claude.ai Slack"` は Slack connector をブロックします。また、`/mcp` を実行して、Claude Code が取得する任意の connector を現在のプロジェクトのみでオン/オフに切り替えることもできます。

<h2 id="use-claude-code-as-an-mcp-server">
  Claude Code を MCP サーバーとして使用する
</h2>

Claude Code 自体を MCP サーバーとして使用して、他のアプリケーションが接続できるようにすることができます。

```bash theme={null}
# Claude を stdio MCP サーバーとして起動する
claude mcp serve
```

コマンドは起動時に何も出力しません。stdio MCP サーバーは stdin と stdout を介して通信するため、サイレント状態でブロックされたターミナルはサーバーが実行中で、クライアントの接続を待機していることを意味します。

Claude Desktop でこれを使用するには、claude\_desktop\_config.json にこの設定を追加します。

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **実行可能ファイルパスの設定**: `command` フィールドは Claude Code 実行可能ファイルを参照する必要があります。`claude` コマンドがシステムの PATH にない場合は、実行可能ファイルへの完全なパスを指定する必要があります。

  完全なパスを見つけるには：

  ```bash theme={null}
  which claude
  ```

  次に、設定で完全なパスを使用します。

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  正しい実行可能ファイルパスがない場合、`spawn claude ENOENT` などのエラーが発生します。
</Warning>

<Tip>
  ヒント：

  * Claude Desktop では、Claude にディレクトリ内のファイルを読み取り、編集などを行うよう依頼してみてください。
  * この MCP サーバーは Claude Code のツールのみを MCP クライアントに公開するため、独自のクライアントは個別のツール呼び出しのユーザー確認を実装する責任があります。
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  MCP 出力制限と警告
</h2>

MCP ツールが大きな出力を生成する場合、Claude Code はトークン使用量を管理して、会話コンテキストが圧倒されるのを防ぐのに役立ちます。

* **出力警告閾値**：Claude Code は、MCP ツール出力が 10,000 トークンを超える場合に警告を表示します
* **設定可能な制限**：`MAX_MCP_OUTPUT_TOKENS` 環境変数を使用して、許可される最大 MCP 出力トークン数を調整できます
* **デフォルト制限**：デフォルトの最大値は 25,000 トークンです
* **スコープ**：環境変数は、独自の制限を宣言していないツールに適用されます。[`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) を設定するツールは、`MAX_MCP_OUTPUT_TOKENS` に設定されている値に関係なく、テキストコンテンツに対してその値を代わりに使用します。画像データを返すツールは、引き続き `MAX_MCP_OUTPUT_TOKENS` の対象となります
* **制限を超える場合**：画像コンテンツのない結果が制限を超える場合、Claude Code はそれをファイルに保存し、会話内でファイルパスを名前とするメッセージに置き換えます。そのため、Claude はコンテンツが必要な場合にファイルを読み取ります。ファイルはセッションの `tool-results` ディレクトリ内の [`~/.claude/projects/`](/docs/ja/claude-directory#cleaned-up-automatically) に配置されます。

大きな出力を生成するツールの制限を増やすには：

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

<h3 id="raise-the-limit-for-a-specific-tool">
  特定のツールの制限を引き上げる
</h3>

MCP サーバーを構築している場合、ツールの `tools/list` レスポンスエントリで `_meta["anthropic/maxResultSizeChars"]` を設定することで、個別のツールがデフォルトの永続化ディスク閾値より大きい結果を返すことができます。Claude Code はそのツールの閾値を注釈付きの値に引き上げます。ただし、500,000 文字のハードシーリングまでです。

これは、データベーススキーマや完全なファイルツリーなど、本質的に大きいが必要な出力を返すツールに役立ちます。注釈がない場合、デフォルト閾値を超える結果はディスクに永続化され、会話内のファイル参照に置き換えられます。

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

注釈はテキストコンテンツに対して `MAX_MCP_OUTPUT_TOKENS` とは独立して適用されるため、ユーザーはそれを宣言するツールのために環境変数を引き上げる必要はありません。画像データを返すツールは、引き続きトークン制限の対象となります。

<Warning>
  制御していない特定の MCP サーバーで出力警告が頻繁に発生する場合は、`MAX_MCP_OUTPUT_TOKENS` 制限を増やすことを検討してください。サーバー作成者に `anthropic/maxResultSizeChars` 注釈を追加するか、レスポンスをページネーションするよう依頼することもできます。注釈は画像コンテンツを返すツールには効果がありません。それらの場合、`MAX_MCP_OUTPUT_TOKENS` を引き上げることが唯一のオプションです。
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  ルートレベルのコンビネータを持つツール入力スキーマ
</h2>

一部の MCP サーバーは、ツールの入力スキーマを JSON Schema ユニオンとして宣言し、スキーマの最上位に `anyOf`、`oneOf`、または `allOf` を配置しています。Claude API はこれらのキーワードをスキーマのルートで受け入れません。ただし、`properties` 内にネストされたコンビネータは受け入れており、Claude Code はそれらを変更せずに送信します。

ルートレベルのコンビネータを持つツールは利用可能なままです。ツールを API に送信する前に、Claude Code はスキーマをフラット化して単一のオブジェクトにし、どのパラメータグループが一緒に属するかを Claude に伝える文をツールの説明の先頭に追加します。

* `allOf`：すべてのブランチからのプロパティがマージされ、各ブランチの `required` リストは引き続き適用されます
* `anyOf` と `oneOf`：すべてのブランチからのプロパティがマージされ、各ブランチの `required` リストはスキーマによって強制されるのではなく、ツールの説明に記述されます

サーバーは Claude が選択した引数を受け取るため、サーバー側で組み合わせの検証を続けてください。

Claude Code が API が受け入れるスキーマを生成できない場合、またはスキーマの書き換えを有効にするリモート設定を受け取らないデプロイメントの場合、そのツール 1 つをスキップし、理由をサーバーのログに記録し、サーバーの他のツールは利用可能なままにします。v2.1.195 より前のバージョンは、入力スキーマにルートレベルの `anyOf`、`oneOf`、または `allOf` を持つすべてのツールをスキップします。

<h2 id="tools-with-invalid-input-schemas">
  無効な入力スキーマを持つツール
</h2>

Claude API はリクエスト内のすべてのツールの入力スキーマをチェックし、いずれかのスキーマが失敗すると、リクエスト全体を 400 エラーで拒否します。そのため、スキーマが不正な形式の MCP ツール 1 つがあると、それを含むすべてのリクエストが失敗します。Claude Code はサーバーのツールを読み込む際に API のチェックのうち 2 つを自身で実行し、それらのチェックに失敗するツールを除外するため、サーバーの他のツールは動作し続けます。

* トップレベルのプロパティ名は 1 ～ 64 文字の長さで、ASCII 文字と数字、`_`、`.`、`-` のみを使用する必要があります
* スキーマは JSON Schema draft 2020-12 メタスキーマに対して有効である必要があります。Claude Code は `$schema` を宣言していないスキーマと draft 2020-12 を宣言しているスキーマにこのチェックを適用します。他の方言を宣言しているスキーマはこのチェックをスキップしますが、上記のプロパティ名チェックは引き続き適用されます

Claude Code は [ルートレベルのコンビネータの書き換え](#tool-input-schemas-with-a-root-level-combinator) の後、実際に送信するスキーマに対してチェックを実行します。

Claude Code がツールを除外する場合、その理由をサーバーのログに記録し、除外したツールとその理由を Claude に伝えるため、ツールが見つからない理由を Claude に尋ねることができます。サーバーのスキーマを修正すると、Claude Code が次にサーバーのツールを読み込むときにツールが復帰します。

Claude Code は Anthropic から取得するフィーチャーフラグを通じて除外をオンにします。[フラグ取得がオフになっているデプロイメント](/docs/ja/env-vars#features-that-need-feature-flag-fetching) または フラグが到着したことのないマシン（エアギャップマシンなど）では、Claude Code はチェックを実行してサーバーのログにどのツールが拒否されるかを記録しますが、ツールのスキーマを API に送信します。API は [ツールの位置で名前を付けた 400 エラー](/docs/ja/errors#tool-input-schema-is-invalid) でそのスキーマを含むリクエストを拒否します。v2.1.216 より前では、デプロイメントはこれらのチェックを実行していませんでした。

[ルートレベルのコンビネータ処理](#tool-input-schemas-with-a-root-level-combinator) は独立しており、フラグ取得がオフの場合またはフラグが到着したことのない場合、独自の動作を保持します。

<h2 id="require-approval-for-a-specific-tool">
  特定のツールに対して承認を要求する
</h2>

MCP サーバーを構築している場合、ツールの `tools/list` レスポンスエントリで `_meta["anthropic/requiresUserInteraction"]` を `true` に設定することで、そのツールがすべての呼び出しで明示的な承認を必要とするようにマークできます。値は JSON ブール値 `true` である必要があります。その他の値は無視されます。

Claude Code は、`acceptEdits`、`auto`、`bypassPermissions` [権限モード](/docs/ja/permissions#permission-modes)でも、そのツールの権限プロンプトをすべての呼び出しで表示し、それに対して「今後は表示しない」オプションを提供しません。ツールに一致する [許可ルール](/docs/ja/permissions#permission-rule-syntax) もプロンプトをスキップしません。プロンプトを表示しない `dontAsk` モードでは、Claude Code は呼び出しを拒否します。

プロンプトは人間に到達する必要があります。[`--permission-prompt-tool`](/docs/ja/cli-reference#cli-flags) を使用した非対話モードでは、フラグが付いたツールのプロンプトツールからの `allow` 結果は、メッセージ `MCP tool requires user interaction; not supported via --permission-prompt-tool` とともに拒否に変換されます。Agent SDK の [`canUseTool` コールバック](/docs/ja/agent-sdk/permissions) はこれらの呼び出しを受け取り、承認できます。これは、SDK アプリケーションがこれらを ユーザーに表示することが期待されているためです。

これを使用するのは、権限プロンプト自体がポイントであるツール、たとえば同意またはアクセス許可ステップなど、自動承認は人間が同意しないことを意味する場合です。同じサーバーからの他のツールは、通常の権限動作を保持します。

次の `tools/list` エントリは、1 つのツールを常に承認が必要なものとしてマークします。

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

`anthropic/requiresUserInteraction` アノテーションには Claude Code v2.1.199 以降が必要です。以前のバージョンはこれを無視し、標準的な権限フローを適用します。

[Remote Control](/docs/ja/remote-control) や [Agent SDK](/docs/ja/agent-sdk/overview) 上に構築されたアプリケーションなど、一部のサーフェスでは通常、1 タップでツール呼び出しを承認できます。このアノテーションでマークされたツールの場合、Claude Code は 1 タップアクションを保留し、代わりにツールの完全な権限プロンプトを表示するため、承認は依然としてプロンプトに答える人から得られます。

Claude Code は、安全警告を含むものや、リモートサーフェスが表示できない常に許可オプションなど、ターミナルダイアログでのみ完全にレンダリングできる権限リクエストに対して、同じ方法で 1 タップ承認を保留します。その要求にはリモートコントロールではなく、ターミナルダイアログで答えます。Claude Code v2.1.214 以降が必要です。

<h2 id="respond-to-mcp-elicitation-requests">
  MCP エリシテーション要求に応答する
</h2>

MCP サーバーは、エリシテーションを使用してタスク中に構造化された入力をリクエストできます。サーバーが独自に取得できない情報が必要な場合、Claude Code はインタラクティブなダイアログを表示し、応答をサーバーに返します。お客様側での設定は不要です。エリシテーションダイアログはサーバーがリクエストすると自動的に表示されます。

サーバーは 2 つの方法で入力をリクエストできます。

* **フォームモード**: Claude Code はサーバーで定義されたフォームフィールド（例えば、ユーザー名とパスワードプロンプト）を含むダイアログを表示します。フィールドに入力して送信します。
* **URL モード**: Claude Code はブラウザ URL を開いて認証または承認を行います。ブラウザでフローを完了してから、CLI で確認します。

URL モード では、Claude Code は URL をコマンドライン引数としてシステムの URL ハンドラーに渡し、その引数の長さに上限を設けます。コマンドラインのエスケープ後の URL がその上限を超える場合、リクエストを拒否することのみできます。`%` や `&` など、エスケープが必要なすべての文字は、上限に対して 4 倍カウントされます。その文字自体と 3 つのエスケープ文字です。これらを含まない URL は約 8,000 文字で上限に達します。3 番目の文字ごとに `%` があるパーセントエスケープで構成される URL は、約 4,000 で上限に達します。

ダイアログを表示せずにエリシテーション要求に自動応答するには、[`Elicitation` フック](/docs/ja/hooks#elicitation)を使用します。

エリシテーションを使用する MCP サーバーを構築している場合は、プロトコルの詳細とスキーマの例については [MCP エリシテーション仕様](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation)を参照してください。

<h2 id="use-mcp-resources">
  MCP リソースを使用する
</h2>

MCP サーバーは、ファイルを参照する方法と同様に、@ メンションを使用して参照できるリソースを公開できます。

<h3 id="reference-mcp-resources">
  MCP リソースを参照する
</h3>

<Steps>
  <Step title="利用可能なリソースをリストアップする">
    プロンプトで `@` を入力すると、接続されているすべての MCP サーバーから利用可能なリソースが表示されます。リソースはオートコンプリートメニューのファイルと一緒に表示されます。
  </Step>

  <Step title="特定のリソースを参照する">
    `@server:protocol://resource/path` の形式を使用してリソースを参照します。

    ```text wrap theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text wrap theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="複数のリソース参照">
    1 つのプロンプトで複数のリソースを参照できます。

    ```text wrap theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  ヒント：

  * リソースは参照されると自動的に取得され、添付ファイルとして含まれます
  * リソースパスは @ メンションオートコンプリートでファジー検索可能です
  * Claude Code は、サーバーがサポートしている場合、MCP リソースをリストアップして読み取るためのツールを自動的に提供します
  * リソースには、MCP サーバーが提供するあらゆるタイプのコンテンツ（テキスト、JSON、構造化データなど）を含めることができます
</Tip>

<h2 id="scale-with-mcp-tool-search">
  MCP ツール検索でスケーリング
</h2>

ツール検索は、Claude がツールを必要とするまでツール定義を遅延させることで、MCP コンテキスト使用量を低く保ちます。セッション開始時にはツール名とサーバー指示のみが読み込まれるため、MCP サーバーを追加してもコンテキストウィンドウへの影響は最小限です。Claude Code はサーバーごとの固定ツール上限を課しません。実用的な上限はコンテキストウィンドウの予算です。

<Note>
  ツール検索は Microsoft Foundry の[Azure でホストされているデプロイメント](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)ではサポートされていません。これらのデプロイメントはサーバー側でツール検索を拒否します。Claude Code はこの拒否を検出し、そのデプロイメント用に MCP ツールを事前に読み込みます。[`ENABLE_TOOL_SEARCH`](#configure-tool-search) はデプロイメント自体からの拒否であるため、これをオーバーライドすることはできません。
</Note>

<h3 id="for-mcp-server-authors">
  MCP サーバー作成者向け
</h3>

MCP サーバーを構築している場合、ツール検索が有効になるとサーバー指示フィールドがより有用になります。サーバー指示は、[スキル](/docs/ja/skills)の動作方法と同様に、Claude がいつツールを検索すべきかを理解するのに役立ちます。

以下を説明する明確で説明的なサーバー指示を追加してください。

* ツールが処理するタスクのカテゴリ
* Claude がツールを検索すべき時期
* サーバーが提供する主な機能

Claude Code はツール説明とサーバー指示を各 2,048 文字でデフォルトで切り詰めます。簡潔に保ち、重要な詳細は最初の方に配置してください。

セッション内のすべての MCP サーバーの上限を変更するには、[`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](/docs/ja/env-vars#variables)を文字数に設定します。この変数には Claude Code v2.1.280 以降が必要です。

<h3 id="configure-tool-search">
  ツール検索を設定する
</h3>

ツール検索はデフォルトで有効です。MCP ツールは遅延され、オンデマンドで検出されます。Claude Code は `ANTHROPIC_BASE_URL` が非ファーストパーティホストを指している場合、ツール検索を無効にします。ほとんどのプロキシは `tool_reference` ブロックを転送しないためです。`ENABLE_TOOL_SEARCH` を明示的に設定して、そのフォールバックをオーバーライドします。

[`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ja/env-vars) を設定するとツール検索がオフになります。`ENABLE_TOOL_SEARCH` を自分で設定してオーバーライドすることはできません。組織は [管理設定](/docs/ja/managed-settings)を通じて Claude Code v2.1.227 以降でツール検索をオンに保つことができます。[プリリリース機能を無効にする](/docs/ja/llm-gateway-protocol#disable-pre-release-capabilities)は、オーバーライドが適用される場所と変数が削除する内容をカバーしています。

ツール検索には `tool_reference` ブロックをサポートするモデルが必要です。Claude Sonnet 4.5、Claude Haiku 4.5、Claude Opus 4.5、およびそれ以降のモデルです。現在のリストについては、[API ドキュメントのモデル互換性](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility)を参照してください。

Google Cloud の Agent Platform では、Claude Code はモデル世代によって決定します。

* **Claude Opus 4.5、Sonnet 4.5、Haiku 4.5、およびそれ以降**: ツール検索はデフォルトでオンです。Anthropic API と同じです。
* **以前の Agent Platform モデル**: Claude Code は必要なベータヘッダーを拒否するサーバースタックのため、すべての MCP ツールを事前に読み込みます。`ENABLE_TOOL_SEARCH=true` はこれをオーバーライドしません。

v2.1.221 より前は、Claude Code は `ENABLE_TOOL_SEARCH=true` を設定しない限り、Google Cloud の Agent Platform 上のすべてのモデルに対してツール検索を無効にしていました。

`ENABLE_TOOL_SEARCH` 環境変数でツール検索の動作を制御します。

| 値        | 動作                                                                                                                                                                                                                                                              |
| :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| （未設定）    | すべての MCP ツールが遅延され、オンデマンドで読み込まれます。Google Cloud の Agent Platform の Claude 4.5 世代より前のモデル、`ANTHROPIC_BASE_URL` が非ファーストパーティホストの場合、または Azure でホストされている Microsoft Foundry デプロイメント上で事前読み込みにフォールバックします                                                                   |
| `true`   | すべての MCP ツールが遅延されます。ただし、Azure でホストされている Microsoft Foundry デプロイメント上では、サーバー側の拒否により事前読み込みが強制され、Google Cloud の Agent Platform の Claude 4.5 世代より前のモデル上では、Claude Code はツール読み込みを事前に保ちます。Claude Code はベータヘッダーをプロキシ経由で送信し、`tool_reference` ブロックをサポートしないプロキシではリクエストが失敗します |
| `auto`   | しきい値モード。Claude Code は、定義の合計がコンテキストウィンドウの 10% 未満の間は、遅延させるツールを事前に読み込み、定義が 10% に達すると、すべてを遅延させます                                                                                                                                                                    |
| `auto:N` | カスタムパーセンテージを使用したしきい値モード。`N` は 0～100 です。たとえば、5% の場合は `auto:5` です                                                                                                                                                                                                 |
| `false`  | すべての MCP ツールが事前に読み込まれ、遅延はありません                                                                                                                                                                                                                                  |

```bash theme={null}
# カスタム 5% しきい値を使用する
ENABLE_TOOL_SEARCH=auto:5 claude

# ツール検索を完全に無効にする
ENABLE_TOOL_SEARCH=false claude
```

または [settings.json `env` フィールド](/docs/ja/settings-reference#env)で値を設定します。

`ToolSearch` ツールを特別に無効にすることもできます。

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  サーバーを遅延から除外する
</h3>

サーバーのツールが常に Claude に表示され、検索ステップなしで利用可能にする場合は、そのサーバーの設定で `alwaysLoad` を `true` に設定します。そのサーバーのすべてのツールは、`ENABLE_TOOL_SEARCH` 設定に関係なく、セッション開始時にコンテキストに読み込まれます。これは、Claude がすべてのターンで必要とする少数のツール用に使用してください。事前読み込みされた各ツールは、会話に利用可能なコンテキストを消費するためです。

次の `.mcp.json` エントリは、1 つの HTTP サーバーを除外し、他のサーバーを遅延させたままにします。

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

`alwaysLoad` フィールドはすべてのサーバータイプで利用可能です。MCP サーバーは、ツールの `_meta` オブジェクトに `"anthropic/alwaysLoad": true` を含めることで、個別のツールを常に読み込まれるようにマークすることもできます。これはそのツールのみに同じ効果があります。

`alwaysLoad: true` を設定すると、スタートアップはサーバーのツールを待機します。最初のプロンプトが構築されるときに存在する必要があるため、標準の 5 秒接続タイムアウトでキャップされます。有効な [`cached` エントリ](#server-status-detail)を持つリモートサーバーは、接続せずにキャッシュからツールを供給するため、スタートアップを保持しません。他のサーバーはデフォルトでバックグラウンドで接続します。[`MCP_CONNECTION_NONBLOCKING=0`](/docs/ja/env-vars) を設定して、スタートアップがそれらも待機するようにします。

<h2 id="use-mcp-prompts-as-commands">
  MCP プロンプトをコマンドとして使用する
</h2>

MCP サーバーは Claude Code でコマンドとして利用可能になるプロンプトを公開できます。

<h3 id="execute-mcp-prompts">
  MCP プロンプトを実行する
</h3>

<Steps>
  <Step title="利用可能なプロンプトを検出する">
    `/` と入力して、MCP サーバーからのプロンプトを含む、利用可能なコマンドを確認します。Claude Code は各 MCP プロンプトを `/servername:promptname (MCP)` として一覧表示します。`/mcp__servername__promptname` と入力して実行することもできます。
  </Step>

  <Step title="引数なしでプロンプトを実行する">
    ```text wrap theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="引数付きでプロンプトを実行する">
    多くのプロンプトは引数を受け入れます。コマンドの後に空白で区切られた引数を渡します。Claude Code は引数を空白で分割するため、各引数は単一のトークンです：

    ```text wrap theme={null}
    /mcp__github__pr_review 456
    ```

    ```text wrap theme={null}
    /mcp__jira__create_issue login-bug high
    ```
  </Step>
</Steps>

<Tip>
  ヒント：

  * MCP プロンプトは接続されたサーバーから動的に検出されます
  * 引数はプロンプトの定義されたパラメータに基づいて解析されます
  * プロンプト結果は会話に直接注入されます
  * `/mcp__servername__promptname` の形式では、Claude Code はサーバー名内の `A-Z`、`a-z`、`0-9`、`_`、および `-` 以外の文字を `_` に置き換え、サーバーが宣言したプロンプト名を使用します
</Tip>

<h2 id="managed-mcp-configuration">
  管理対象 MCP 設定
</h2>

MCP サーバーへの接続をユーザーが行えるかを一元管理する必要がある組織の場合は、[管理対象 MCP 設定](/docs/ja/managed-mcp)を参照してください。`managed-mcp.json` を使用した固定サーバーセットのデプロイ、`managedMcpServers` を使用したすべてのユーザーへのサーバー提供、`allowedMcpServers` と `deniedMcpServers` によるサーバーの制限、およびサーバーがブロックされた場合にユーザーに表示される内容について説明しています。
