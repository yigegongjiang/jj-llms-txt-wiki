> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK のホスティング

> Agent SDK を本番環境にデプロイする：サブプロセスアーキテクチャ、セッション永続化、スケーリング、可観測性、Docker、Kubernetes、サンドボックスプロバイダー向けのマルチテナント分離。

Agent SDK は `claude` CLI サブプロセスを起動・監視します。このサブプロセスはシェル、作業ディレクトリ、ディスク上のセッションファイルを所有しています。ホスティングはステートレス API ラッパーのホスティングとは異なります。実行中のエージェントはすべて、ローカル状態に結びついた長寿命プロセスであり、これがリソース割り当て、セッション永続化、テナント間のスケーリング方法を決定します。

このページは独自のインフラストラクチャでのセルフホスティングについて説明しています。デプロイ可能な Dockerfile と Kubernetes マニフェストについては、[ホスティングクックブック](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)を参照してください。

エージェントループ自体を独自のインフラストラクチャで実行する必要がない場合は、[Managed Agents](https://platform.claude.com/docs/en/managed-agents/overview) の使用を検討してください。Anthropic がエージェントループをホストし、アプリケーションはクライアント SDK または REST API を通じてイベントを送信し、ストリーミング結果を受け取ります。ツール実行は Anthropic 管理クラウドサンドボックスまたは独自のインフラストラクチャ上の[セルフホスト型サンドボックス](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes)で実行されます。

<h2 id="the-subprocess-model">
  サブプロセスモデル
</h2>

このページのすべてのホスティング決定は、SDK がエージェントをどのように実行するかに基づいています。コードが `query()` を呼び出すと、SDK は別の `claude` CLI プロセスを生成し、stdio 経由で通信します。そのサブプロセスはシェル、作業ディレクトリ、およびローカルディスク上の JSONL セッショントランスクリプトを所有しています。

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" className="dark:hidden" alt="Request flow: client to your app, which spawns a claude CLI subprocess over stdio inside the container; the subprocess writes to local disk and calls api.anthropic.com over HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/hosting-subprocess-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=3fdeff3d7f44b2b67762668acfbb25f5" className="hidden dark:block" alt="Request flow: client to your app, which spawns a claude CLI subprocess over stdio inside the container; the subprocess writes to local disk and calls api.anthropic.com over HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess-dark.svg" />

1 つのエージェントセッションは 1 つのサブプロセスにマップされます。N 個の同時セッションを実行することは、N 個のサブプロセスを実行することを意味し、各サブプロセスは独自のプロセスツリーとトランスクリプトファイルを持ちます。デフォルトでは、すべてがアプリケーションの作業ディレクトリを継承します。セッションが個別のファイルシステムを必要とする場合は、各セッションの `query()` 呼び出しのオプションで異なる `cwd` を渡します。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the files in this directory",
    options: { cwd: "/work/session-a" },
  })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt="Summarize the files in this directory",
          options=ClaudeAgentOptions(cwd="/work/session-a"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

このページの TypeScript の例はトップレベルの `await` を使用しているため、`.mts` ファイルとして保存するか、`package.json` で `"type": "module"` を設定してください。

<h3 id="state-that-lives-on-local-disk">
  ローカルディスクに存在する状態
</h3>

3 種類のエージェント状態がデフォルトでコンテナのファイルシステムに存在します。これらのいずれも、コンテナの再起動、スケールダウン、または別のノードへの移動を経ても保持されません。

| 状態                  | デフォルトの場所                                                                                             |
| ------------------- | ---------------------------------------------------------------------------------------------------- |
| セッショントランスクリプト       | `~/.claude/projects/`、または `CLAUDE_CONFIG_DIR` が設定されている場合は `CLAUDE_CONFIG_DIR` の下の `projects/` ディレクトリ |
| `CLAUDE.md` メモリファイル | ユーザーティアの場合は `~/.claude/CLAUDE.md`、プロジェクトティアの場合はセッションの作業ディレクトリ                                        |
| 作業ディレクトリアーティファクト    | セッションの作業ディレクトリ                                                                                       |

トランスクリプトをホスト間で永続化するには、[`SessionStore` アダプター](/docs/ja/agent-sdk/session-storage)を設定します。メモリファイルおよび他の作業ディレクトリアーティファクトは、マウントされたボリュームやオブジェクトストア同期などの独自のストレージ戦略が必要です。

セッション、再開、およびフォークが API レベルでどのように機能するかについては、[セッション](/docs/ja/agent-sdk/sessions)を参照してください。

<h2 id="choose-a-session-pattern">
  セッションパターンを選択する
</h2>

これら 4 つのパターンはセッションライフサイクルをカバーしています。コンテナがそれを提供するセッションに対してどのくらいの期間存在するかです。コンテナが実行される場所については、[ホスティングクックブック](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb)に、ローカル Docker、Modal、Kubernetes 向けの[デプロイ可能なコード](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)があります。ここでセッションパターンを選択し、クックブックからデプロイメントターゲットを選択してください。

<h3 id="ephemeral-sessions">
  エフェメラルセッション
</h3>

各ユーザータスク用にコンテナを作成し、タスクが完了したら破棄します。ワンオフタスクに最適です。ユーザーはタスクが完了している間も AI と対話できますが、完了後はコンテナが破棄されます。

例のワークロードには、バグ調査と修正、請求書と領収書の抽出、ドキュメント翻訳、メディア変換が含まれます。

コンテナは、`TASK_PROMPT` 環境変数からタスクを読み取り、SDK を呼び出して終了する 1 回限りのエントリポイントを実行します。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompt = process.env.TASK_PROMPT!;
  for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt=os.environ["TASK_PROMPT"],
          options=ClaudeAgentOptions(max_turns=20),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

スクリプトは到着時に各メッセージを出力します。これには、タスクがターン制限内に完了したときに `subtype` が `success` である結果メッセージが含まれます。代わりにタスクが 20 ターン制限に達した場合、結果メッセージの `subtype` は `error_max_turns` であり、`query()` 呼び出しはそれを生成した後にエラーを発生させるため、コンテナがクリーンに終了する必要がある場合はループを try ブロックでラップしてください。エラーサブタイプについては、[結果を処理する](/docs/ja/agent-sdk/agent-loop#handle-the-result)を参照してください。

<h3 id="long-running-sessions">
  長時間実行セッション
</h3>

永続的なコンテナインスタンスを実行し、多くの場合、コンテナごとに複数の SDK プロセスをホストして、継続的な作業を提供します。自律的なアクションを実行するエージェント、コンテンツを提供するエージェント、または大量のメッセージストリームを処理するエージェントに最適です。

例のワークロードには、受信メールをトリアージして応答するメールエージェント、コンテナポートを通じてユーザーが編集可能なサイトをホストするサイトビルダー、Slack などのプラットフォームからの継続的なトラフィックを処理するチャットボットが含まれます。

コンテナは HTTP または WebSocket エンドポイントを公開し、各アクティブセッションを長時間実行されるクエリとその背後にあるサブプロセスにマップします。TypeScript では、[`streamInput()`](/docs/ja/agent-sdk/typescript#query-object)を使用してアクティブセッションにターンを追加し、[`startup()`](/docs/ja/agent-sdk/typescript#startup)を使用して受信トラフィック前にサブプロセスをプリウォームします。Python では、[`ClaudeSDKClient`](/docs/ja/agent-sdk/python#claudesdkclient)を使用してセッションをターン全体で開いたままにします。コンテナのサイズを、メモリに保持できる最大数の同時セッションに対応できるようにしてください。

<h3 id="hybrid-sessions">
  ハイブリッドセッション
</h3>

スタートアップ時に[`SessionStore`](/docs/ja/agent-sdk/session-storage)から水和し、更新を戻すエフェメラルコンテナ。多くのインタラクションにまたがるが、その間はアイドル状態になるセッションに最適です。コンテナはアイドル期間中にスピンダウンし、ユーザーが戻ってきたときにスピンバックアップします。

例のワークロードには、断続的なチェックインを伴う個人プロジェクトマネージャー、数時間にわたって一時停止および再開する深い調査、インタラクション全体でチケット履歴を読み込むカスタマーサポートエージェントが含まれます。

プロバイダーのアイドルタイムアウトをユーザーが戻ってくることを期待する頻度に合わせて調整します。`SessionStore` が設定されていない状態でコンテナをシャットダウンするとトランスクリプトが失われるため、ストアはこのパターンでは必須であり、オプションではありません。

パターンは、共有ストアが接続された ID でセッションを再開することに基づいています。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // an object store, key-value store, database, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, SessionStore
  import asyncio

  user_input: str = ...
  session_id: str = ...              # looked up from your database by user
  session_store: SessionStore = ...  # an object store, key-value store, database, or your own adapter


  async def main():
      async for message in query(
          prompt=user_input,
          options=ClaudeAgentOptions(
              resume=session_id,
              session_store=session_store,
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="multi-agent-container">
  マルチエージェントコンテナ
</h3>

1 つのコンテナ内で複数の SDK サブプロセスを実行します。エージェントが密接に協力する必要がある場合に最適です。たとえば、エージェントが共有環境で相互作用するマルチエージェントシミュレーションです。

各エージェントに独自の作業ディレクトリを与えて、相互にファイルを上書きしないようにし、設定読み込みを分離して、エージェント別の `CLAUDE.md` ファイルがエージェント間でリークしないようにします。具体的なオプションについては、[マルチテナント分離](#multi-tenant-isolation)を参照してください。

<h2 id="provision-the-container">
  コンテナをプロビジョニングする
</h2>

<h3 id="container-based-sandboxing">
  コンテナベースのサンドボックス
</h3>

プロセス分離、リソース制限、ネットワーク制御、および一時的なファイルシステムのために、SDK をサンドボックス化されたコンテナ内で実行します。

プロバイダーを選択する際に回答すべき質問：

* **サンドボックスを実行する者**：サンドボックス・アズ・ア・サービスプロバイダーはインフラストラクチャを運用しますが、セルフホスト型オプションは自分のサーバーで実行するソフトウェアを提供します。
* **コールドスタートレイテンシー**：「サンドボックスを作成」から「最初のリクエストを受け入れる準備ができた」までの時間。一時的なパターンは 1 秒未満の起動が必要です。長時間実行パターンはより長い時間を許容します。
* **永続ストレージ**：プロバイダーが耐久性のあるボリュームを提供するか、一時的なディスクのみを提供するか。ハイブリッドパターンは、サンドボックス内またはその隣のいずれかで、どこかに耐久性のあるストレージが必要です。
* **価格モデル**：秒単位、リクエスト単位、または定額時間単位の課金。秒単位の価格設定はバースト的な一時的ワークロードに適しています。時間単位は長時間実行セッションに適しています。
* **ネットワーク**：カスタム出力ルール、アウトバウンドプロキシ、および規制環境向けのプライベート VPC ピアリングのサポート。

Docker、gVisor、Firecracker などのセルフホスト型オプションと詳細な分離設定については、[分離テクノロジー](/docs/ja/agent-sdk/secure-deployment#isolation-technologies)を参照してください。

<h3 id="runtime-dependencies">
  ランタイム依存関係
</h3>

コンテナには SDK の言語ランタイムが必要です：

* Python SDK の場合は Python 3.10 以上、または TypeScript SDK の場合は Node.js 18 以上
* TypeScript SDK と Python SDK の両方は、ほとんどのインストールに対してネイティブ Claude Code バイナリをバンドルしており、生成された CLI は別の Node.js インストールを必要としません。別のネイティブ Claude Code インストールが必要なインストールについては、[クイックスタートのインストール注記](/docs/ja/agent-sdk/quickstart)を参照してください。

バンドルされたバイナリは SDK パッケージバージョンに固定されているため、SDK を更新することが CLI を更新する方法です。SDK は semver に従います：パッチリリースは継続的に取得し、マイナーを取得する前に [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) または [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) チェンジログを確認してください。

<h3 id="resources">
  リソース
</h3>

新しく起動されたインスタンスごとに、1 GiB RAM、5 GiB ディスク、および 1 CPU は合理的な開始点です。メモリ使用量はセッション長とツールアクティビティとともに増加するため、アイドルベースラインではなく、実際に必要なセッション長と同時実行性に合わせてサイズを設定してください。ホストあたりのエージェント数を計算する方法については、[スケーリングと同時実行](#scaling-and-concurrency)を参照してください。

<h3 id="network">
  ネットワーク
</h3>

SDK は `api.anthropic.com` へのアウトバウンド HTTPS、または Amazon Bedrock または Google Cloud の Agent Platform で実行する場合はプロバイダーのリージョナルエンドポイントが必要です。エージェントが [MCP サーバー](/docs/ja/agent-sdk/mcp)または外部ツールを使用する場合、それらのエンドポイントへのアウトバウンドアクセスも必要です。本番環境では、ドメイン許可リストを適用し、認証情報を挿入し、リクエストをログに記録するエグレスプロキシを通じてアウトバウンドトラフィックをルーティングしてください。完全なパターンについては、[セキュアデプロイメント](/docs/ja/agent-sdk/secure-deployment)を参照してください。

インバウンドトラフィックの場合、コンテナ上の HTTP または WebSocket ポートを公開します。アプリケーションはそのポートでクライアントリクエストを処理し、内部的に SDK を呼び出します。サブプロセス自体はネットワーク上でリッスンしません。

<h2 id="handle-production-concerns">
  本番環境の懸念事項に対応する
</h2>

自己ホスト型エージェントをリリースする前に、これらの決定事項を検討してください。

<h3 id="session-and-state-persistence">
  セッションと状態の永続化
</h3>

デフォルトのローカルディスクは、再起動、スケールダウン、または別のノードへの移動時に失われます。ユーザーが再開することを期待するセッションについては、トランスクリプトを [`SessionStore` アダプター](/docs/ja/agent-sdk/session-storage) を使用して耐久性のあるストレージにミラーリングしてください。[リファレンス実装](/docs/ja/agent-sdk/session-storage#reference-implementations) では、オブジェクトストア、キーバリューストア、データベース用のサンプルアダプター、および独自のアダプター用の適合性スイートを参照してください。

`SessionStore` の動作について知っておくべき 3 つのことがあります。

* **トランスクリプトのみ**: `SessionStore` はトランスクリプトをミラーリングし、`CLAUDE.md` メモリファイルや他の作業ディレクトリアーティファクトはミラーリングしません。共有ボリュームをマウントするか、それらを別途同期してください。
* **ミラーリング、置き換えではない**: サブプロセスはまずローカルディスクに書き込み、SDK は各バッチのコピーをストアに転送します。新しいセッションのローカルトランスクリプトは実行より長く存続します。ストアから再開された実行は終了時にローカルコピーを削除するため、ストアが唯一の耐久性のあるコピーを保持します。[デュアルライトアーキテクチャ](/docs/ja/agent-sdk/session-storage#dual-write-architecture) を参照してください。
* **`mirror_error` メッセージ**: SDK がバッチをストアに配信できない場合、バッチをドロップし、`{ type: "system", subtype: "mirror_error" }` メッセージを発行し、クエリを続行します。ストアの耐久性が重要な場合は、これらについてアラートを設定してください。再試行とタイムアウト動作については、[ミラーライトはベストエフォート](/docs/ja/agent-sdk/session-storage#mirror-writes-are-best-effort) を参照してください。

<h3 id="observability">
  可観測性
</h3>

Agent SDK エージェントは長時間実行されるプロセスであり、多くの API ラウンドトリップにわたってツール呼び出しを生成します。テレメトリがなければ、どのツールが実行されたか、どのくらい時間がかかったか、またはセッションがどこで停止したかを確認することはできません。

SDK は環境から OpenTelemetry 設定を継承します。コンテナーまたはオーケストレーターレベルで OTEL 環境変数を設定して、すべての `query()` 呼び出しがスパン、メトリクス、およびログイベントをコレクターにエクスポートするようにしてください。以下の例は、3 つのシグナルすべてに対して OTLP エクスポートを有効にします。`CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` はトレースにのみ必要です。メトリクスとログのみをエクスポートする場合は省略してください。

```bash title=".env" theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

プロンプトテキストとツール入力はデフォルトではエクスポートに含まれません。オプトインフラグについては [エクスポートで機密データを制御する](/docs/ja/agent-sdk/observability#control-sensitive-data-in-exports) を参照し、完全なシグナルカタログについては [可観測性](/docs/ja/agent-sdk/observability) を参照してください。

<h3 id="auth-and-secrets">
  認証とシークレット
</h3>

ホスティング時に重要な 3 つの認証上の懸念があります。

* **Anthropic API**: サブプロセスは環境から `ANTHROPIC_API_KEY` を読み取ります。シークレットマネージャーから供給するか、`ANTHROPIC_BASE_URL` を設定してモデル呼び出しをコンテナー外でキーを注入するプロキシを通じてルーティングしてください。プロキシパターンについては [認証情報管理](/docs/ja/agent-sdk/secure-deployment#credential-management) を参照し、サポートされている認証方法については [SDK クイックスタートのセットアップ](/docs/ja/agent-sdk/quickstart#setup) を参照してください。
* **インバウンド**: エージェントコンテナーの前のゲートウェイに認証を配置してください。エージェントは事前認証されたリクエストを受け取る必要があり、ユーザートークンを検証するコンポーネントであってはいけません。
* **アウトバウンドツール**: ツール認証情報をエージェント環境から除外してください。アウトバウンド呼び出しをプロキシを通じてルーティングし、リクエストがコンテナーを離れた後に API キーを注入してください。エージェントが呼び出しを行い、プロキシが認証情報を追加します。

<h3 id="scaling-and-concurrency">
  スケーリングと並行処理
</h3>

各セッションは独自のサブプロセスで実行されるため、ホスト上の並行処理はそのホストの RAM が保持できるサブプロセスの数によって制限されます。

このフォーミュラを使用して各ホストのサイズを設定してください。

```text theme={null}
agents per host = (host RAM - overhead) / (per-session RAM ceiling)
```

セッションごとの上限を測定するには、代表的なセッションをターゲット長まで実行し、予想されるツール負荷の下で実行し、ピーク RSS を記録してください。[リソース](#resources) の 1 GiB の開始ポイントはフロアであり、上限ではありません。

水平スケーリングのルーティングはパターンによって異なります。コンテナーが多くのセッションを保持する長時間実行セッションの場合、ロードバランサーの背後にあるコンテナープールを実行し、`sessionId` での一貫性ハッシュを使用して各セッションを 1 つのコンテナーにピン留めしてください。ピン留めされたセッションは、削除されるか、コンテナーが再起動されるまで、同じコンテナーに、したがって同じ実行中のサブプロセスに継続的にヒットします。

<h3 id="cost">
  コスト
</h3>

Anthropic トークンコストは通常、コンテナーインフラストラクチャコストを 1 桁以上上回ります。最小限にプロビジョニングされたコンテナーは 1 時間あたり約 \$0.05 で実行されますが、単一の長いエージェントセッションはトークンで数ドルを費やす可能性があります。セッションごとのトークンアカウンティングについては、[コスト追跡](/docs/ja/agent-sdk/cost-tracking) を参照してください。

<h3 id="multi-tenant-isolation">
  マルチテナント分離
</h3>

デフォルト SDK 動作は、ファイルシステムから設定と `CLAUDE.md` メモリファイルを読み取ります。複数のテナントにサービスを提供する共有コンテナーでは、これらのファイルは 1 つのテナントのコンテキストを別のテナントのセッションにリークする可能性があります。

共有コンテナー内でテナントを分離するには、以下を実行してください。

* TypeScript で `settingSources: []` を渡すか、Python で `setting_sources=[]` を渡して、ユーザー、プロジェクト、およびローカル設定をスキップしてください。
* `env` で `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` を設定してください。[自動メモリ](/docs/ja/memory#auto-memory) は `~/.claude/projects/<project>/memory/` にあり、`settingSources` に関係なくシステムプロンプトに読み込まれます。`settingSources` が制御しないその他の入力については、[settingSources が制御しないもの](/docs/ja/agent-sdk/claude-code-features#what-settingsources-does-not-control) を参照してください。
* `CLAUDE_CONFIG_DIR` をテナントごとのディレクトリにポイントして、テナントが `~/.claude.json` グローバル設定を共有しないようにしてください。各設定ディレクトリが 1 つの作業ディレクトリにサービスを提供し、テナント間で [`SessionStore`](/docs/ja/agent-sdk/session-storage) を共有しない場合、`env` で [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ja/sessions#name-the-project-directory-yourself) を設定して、その下のトランスクリプトパスを短く保つこともできます。TypeScript Agent SDK v0.3.234 以降、または Python Agent SDK v0.2.140 以降が必要です。
* テナントごとの作業ディレクトリを使用してください。すべての `query()` 呼び出しで `cwd` を明示的に渡してください。
* プロキシで異なるアウトバウンド IP、認証情報、またはドメイン許可リストなどのテナントごとのエグレスルールを適用して、侵害されたテナントが別のテナントのアウトバウンドポリシーを介してデータを流出させることができないようにしてください。

以下の例は、設定、自動メモリ、設定ディレクトリ、および作業ディレクトリオプションを一緒に適用します。`tenantDir` と `configDir` を構築して、各テナントが他のテナントが読み取ることができないパスを取得するようにしてください。TypeScript では、`env` はサブプロセス環境を置き換えるため、`PATH` や `ANTHROPIC_API_KEY` などの継承された変数を保持するために `...process.env` を展開してください。Python では、`env` は継承された環境の上にマージされます。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio

  prompt: str = ...
  tenant_dir: str = ...
  config_dir: str = ...


  async def main():
      async for message in query(
          prompt=prompt,
          options=ClaudeAgentOptions(
              cwd=tenant_dir,
              setting_sources=[],
              env={
                  "CLAUDE_CONFIG_DIR": config_dir,
                  "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="known-limitations">
  既知の制限事項
</h2>

デプロイメント設計でこれらを考慮してください。

| 制限事項                                  | 対応方法                                                                                                                                                                                  |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| トップレベルのセッションタイムアウトがない                 | セッションは自動的にタイムアウトしません。TypeScript の `maxTurns` または Python の `max_turns` を設定して、エージェントがツール使用ラウンドトリップを実行する回数を制限してから停止させます。                                                                 |
| 長いセッションでのメモリ増加                        | セッション長を制限するか、サブプロセスを定期的にリサイクルしてください。[スケーリングと並行処理](#scaling-and-concurrency)を参照してください。                                                                                                 |
| 大規模な並列サブエージェントのファンアウトがレート制限に達する可能性がある | 1 つの広いディスパッチを発行するのではなく、作業をより小さなバッチに分割してください。                                                                                                                                          |
| サブエージェントごとのウォールクロック期限がない              | 各[サブエージェント](/docs/ja/agent-sdk/subagents)を `AgentDefinition` の `maxTurns` で制限してください。`CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` はサブエージェントが出力を生成しなくなったときに発火するスタール監視タイマーを設定します。これは総実行時間の期限ではありません。 |

<h2 id="troubleshoot-deployment-failures">
  デプロイメント失敗のトラブルシューティング
</h2>

マシン上で動作するエージェントがデプロイされたサービスで失敗する場合は、このセクションを使用してください。以下の各項目は失敗の名前を示し、それをカバーするエントリへのリンクを提供します。

* **サービス開始時に CLI が見つからない**: Python では、コンテナまたはサービスマネージャーがアプリケーションをシェルとは異なる `PATH` で実行するため、ローカルで機能するインストールがプロセスに表示されません。TypeScript では、イメージビルドが SDK のオプション依存関係をスキップしたか、`pathToClaudeCodeExecutable` がイメージに存在しないファイルを指しています。[Claude Code が見つかりません](/docs/ja/agent-sdk/troubleshooting#clinotfounderror-claude-code-not-found)を参照してください。
* **イメージに CLI が存在するが起動しない**: Claude Code は、コンテナのアーキテクチャまたは libc と一致しないバイナリから起動できないか、イメージビルド中に実行権限を失ったファイルから起動できません。[Claude Code の起動に失敗](/docs/ja/agent-sdk/troubleshooting#cliconnectionerror-failed-to-start-claude-code)を参照してください。
* **Claude Code プロセスが実行中に終了する**: アプリケーションが受け取るエラーは、SDK 言語と CLI が最初にエラー結果を報告したかどうかによって異なります。[CLI プロセス終了](/docs/ja/agent-sdk/troubleshooting#cli-process-exit)の下のエントリは各メッセージをカバーしています。

<h2 id="next-steps">
  次のステップ
</h2>

* [ホスティングクックブック](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb)：Docker、Modal、および Kubernetes 用の[デプロイ可能なコード](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)を含むノートブックのウォークスルー。
* [セッションストレージ](/docs/ja/agent-sdk/session-storage)：`SessionStore` アダプターを使用してホスト間でトランスクリプトを永続化します。
* [可観測性](/docs/ja/agent-sdk/observability)：OTEL トレース、メトリクス、およびログをコレクターにエクスポートします。
* [セキュアデプロイメント](/docs/ja/agent-sdk/secure-deployment)：ネットワーク制御、認証情報管理、および分離強化。
* [コスト追跡](/docs/ja/agent-sdk/cost-tracking)：セッションごとのトークンおよびコスト会計。
