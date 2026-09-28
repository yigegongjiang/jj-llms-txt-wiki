> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 監視

> Claude Code の OpenTelemetry を有効にして設定する方法を学びます。

OpenTelemetry (OTel) を通じてテレメトリデータをエクスポートすることで、組織全体で Claude Code の使用状況、コスト、ツールアクティビティを追跡します。Claude Code はメトリクスを標準メトリクスプロトコル経由で時系列データとしてエクスポートし、イベントをログ/イベントプロトコル経由でエクスポートし、オプションで [トレースプロトコル](#traces-beta)経由で分散トレースをエクスポートします。

<h2 id="quick-start">
  クイックスタート
</h2>

環境変数を使用して OpenTelemetry を設定します:

```bash theme={null}
# 1. テレメトリを有効にする
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. エクスポーターを選択する (両方はオプション - 必要なものだけを設定してください)
export OTEL_METRICS_EXPORTER=otlp       # オプション: otlp、prometheus、console、none
export OTEL_LOGS_EXPORTER=otlp          # オプション: otlp、console、none

# 3. OTLP エンドポイントを設定する (OTLP エクスポーター用)
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. 認証を設定する (必要な場合)
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. デバッグ用: エクスポート間隔を短縮し、本番環境での使用に向けてリセットしてください
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 秒 (デフォルト: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 秒 (デフォルト: 5000ms)

# 6. Claude Code を実行する
claude
```

メトリクスをエクスポートするセットアップを検証するには、バックエンドで `claude_code.session.count` メトリクスを確認してください。Claude Code はセッション開始時にこのメトリクスを出力します。ログのみのセットアップを検証するには、プロンプトを送信して `claude_code.user_prompt` イベントを確認してください。

何も到着しない場合は、`claude --debug` を実行してデバッグログを確認してください。Claude Code は、設定したエクスポーターからの失敗を `[3P telemetry]` エラーとして報告します。ここで 3P はサードパーティを意味します。`[Anthropic telemetry]` で始まる行は、[Anthropic の個別の運用テレメトリ](/docs/ja/data-usage#telemetry-services)について説明しており、セットアップの問題を示していません。

完全な設定オプションについては、[OpenTelemetry 仕様](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options)を参照してください。

<h2 id="administrator-configuration">
  管理者設定
</h2>

管理者は、[管理設定ファイル](/docs/ja/managed-settings#delivery-mechanisms)を通じてすべてのユーザーの OpenTelemetry 設定を設定できます。設定がどのように適用されるかについては、[設定の優先順位](/docs/ja/settings#settings-precedence)を参照してください。

管理設定の設定例:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer example-token"
  }
}
```

Claude Code は、リポジトリの `.claude/settings.json` と `.claude/settings.local.json` の [OpenTelemetry エクスポーター変数](/docs/ja/settings-reference#variables-claude-code-ignores-in-env)を無視するため、リポジトリはそれらを使用してテレメトリをオンにしたり、送信先を選択したり、コンテンツをキャプチャしたりすることはできません。管理設定で設定するか、各開発者がシェルまたは `~/.claude/settings.json` で設定してください。リポジトリは、`OTEL_LOGS_EXPORTER` などのエクスポーターセレクターを `none` に設定することでシグナルをオフにすることはできますが、管理設定、`--settings` ファイル、または Claude Code を起動する環境がその変数を設定している場合を除きます。

Claude Code は、Bash ツール、フック、MCP サーバー、言語サーバーを含む、生成するサブプロセスに `OTEL_*` 環境変数を渡しません。OpenTelemetry でインストルメント化されたアプリケーションを Bash ツール経由で実行する場合、Claude Code のエクスポーターエンドポイントまたはヘッダーを継承しないため、そのアプリケーションが独自のテレメトリをエクスポートする必要がある場合は、コマンド内でこれらの変数を直接設定してください。

<h3 id="how-managed-settings-lock-the-otlp-destination">
  管理設定が OTLP 宛先をロックする方法
</h3>

管理設定で `OTEL_EXPORTER_OTLP_*` 変数を設定すると、Claude Code は起動時に競合する開発者設定の変数を削除し、`claude --debug` で確認できる警告をログに記録します。削除される内容は、設定する変数によって異なります：

* **エンドポイント**：`OTEL_EXPORTER_OTLP_ENDPOINT` を設定すると、Claude Code はすべての開発者設定のシグナル別エンドポイントを削除します。開発者は 1 つのシグナルを別のコレクターにポイントできないため、管理設定でシグナル別エンドポイント変数も設定する必要はありません。
* **プロトコル**：`OTEL_EXPORTER_OTLP_PROTOCOL` を設定すると、Claude Code はすべての開発者設定のシグナル別プロトコルを削除します。
* **認証情報**：`OTEL_EXPORTER_OTLP_HEADERS`、`OTEL_EXPORTER_OTLP_CLIENT_KEY`、または `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE` を設定すると、Claude Code はその変数の開発者設定のシグナル別バージョンと、すべての開発者設定のエンドポイント変数（汎用またはシグナル別）を削除します。これらの認証情報が管理設定で選択されていないコレクターに到達するのを防ぐためです。
* **エクスポーターセレクター**：`OTEL_METRICS_EXPORTER`、`OTEL_LOGS_EXPORTER`、およびベータ版の `OTEL_TRACES_EXPORTER` は通常のキーごとの優先順位に従います。開発者の設定はシグナルを無効にするか、コンソールエクスポーターに切り替えることができるため、ロックが必要な場合は管理設定でセレクターも設定してください。[管理ソース](/docs/ja/managed-settings#precedence-within-the-managed-tier)全体で、`OTEL_LOGS_EXPORTER` は[テレメトリユニット](/docs/ja/server-managed-settings#per-key-exceptions-across-managed-sources)に従い、他の 2 つのセレクターはキーごとにマージされます。Claude Code v2.1.223 以降が必要です。
* **ベータ版トレーシングエンドポイント**：[詳細ベータ版トレーシング](#traces-beta)がアクティブな場合、Claude Code はログとトレースをログおよびトレースエクスポーターを通じてではなく `BETA_TRACING_ENDPOINT` にエクスポートします。したがって、Claude Code は以下の管理設定のいずれかがシグナルの宛先を決定するたびに、開発者設定の `BETA_TRACING_ENDPOINT` を削除します：

  * 汎用またはログ/トレースエンドポイントまたは認証情報
  * [`otelHeadersHelper`](/docs/ja/settings-reference#otelheadershelper)
  * `none`、`console`、または空に設定されたログまたはトレースエクスポーターセレクター。これらの値はシグナルをコレクターから外します
  * `CLAUDE_CODE_ENABLE_TELEMETRY` がオフ

  メトリクスのみのエンドポイントまたは認証情報は削除しません。v2.1.251 より前では、開発者設定の `BETA_TRACING_ENDPOINT` は、管理設定がコレクターをピン留めしている場合でも、詳細ベータ版トレーシングがエクスポートするログとトレースをリダイレクトしていました。

Claude Code は管理設定自体で設定したシグナル別変数を削除しないため、その変数をそこに設定することで 1 つのシグナルを別のコレクターにルーティングできます。[SIEM の例](#send-events-to-a-siem)がこれを行っています。そこでシグナル別認証情報を設定する場合、Claude Code はそのシグナルの開発者設定エンドポイントを削除します。

この削除動作は、テレメトリが配信される場所を変更するもので、Claude Code が収集する内容ではありません。

v2.1.217 より前では、すべての変数は独立してキーごとの設定優先順位に従っていたため、ユーザー設定またはシェルで設定されたシグナル固有のエンドポイントがそのシグナルを管理コレクターから離れた場所にリダイレクトしていました。

デスクトップアプリまたは[セルフホスト環境](/docs/ja/self-hosted-environments)ランナーが Claude Code を起動し、提供する環境で OTLP エンドポイントを指定する場合、Claude Code は同じ方法で宛先をピン留めします。ランナーのテレメトリ変数は、管理設定と同じように開発者設定の変数を削除します。Claude Code はランナー自体が設定した変数を削除しません。Claude Code v2.1.251 以降が必要です。

<h2 id="configuration-details">
  設定の詳細
</h2>

<h3 id="common-configuration-variables">
  一般的な設定変数
</h3>

これらの変数は、すべてのデプロイメントのエクスポーター、エンドポイント、エクスポート動作を設定します。`OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` などのシグナルごとのエンドポイントまたはプロトコル変数を設定した場合、Claude Code はそのシグナルの汎用変数の代わりにそれを使用します。`OTEL_EXPORTER_OTLP_METRICS_HEADERS` などのシグナルごとのヘッダー変数を設定した場合、Claude Code はそのシグナルの汎用 `OTEL_EXPORTER_OTLP_HEADERS` とマージします。管理設定を持つマシンでは、[管理設定が OTLP 宛先をロックする方法](#how-managed-settings-lock-the-otlp-destination)を参照して、Claude Code が削除するものを確認してください。

| 環境変数                                                | 説明                                                                                                                                                                                                                                                                                                                                                                                                      | 例の値                                                                                                        |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | テレメトリ収集を有効にする (必須)                                                                                                                                                                                                                                                                                                                                                                                      | `1`                                                                                                        |
| `OTEL_METRICS_EXPORTER`                             | メトリクスエクスポーターのタイプ (カンマ区切り)。`none` を使用して無効化                                                                                                                                                                                                                                                                                                                                                               | `console`、`otlp`、`prometheus`、`none`                                                                       |
| `OTEL_LOGS_EXPORTER`                                | ログ/イベントエクスポーターのタイプ (カンマ区切り)。`none` を使用して無効化                                                                                                                                                                                                                                                                                                                                                             | `console`、`otlp`、`none`                                                                                    |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | OTLP エクスポーターのプロトコル (すべてのシグナルに適用)。Claude Code にはデフォルトプロトコルがないため、有効にする各 `otlp` エクスポーターについて、これまたはシグナル固有のプロトコル変数を設定してください                                                                                                                                                                                                                                                                                   | `grpc`、`http/json`、`http/protobuf`                                                                         |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | OTLP コレクターエンドポイント (すべてのシグナル)                                                                                                                                                                                                                                                                                                                                                                            | `http://localhost:4317`                                                                                    |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | メトリクスのプロトコル (一般的な設定をオーバーライド)                                                                                                                                                                                                                                                                                                                                                                            | `grpc`、`http/json`、`http/protobuf`                                                                         |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | OTLP メトリクスエンドポイント (一般的な設定をオーバーライド)                                                                                                                                                                                                                                                                                                                                                                      | `http://localhost:4318/v1/metrics`                                                                         |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | ログのプロトコル (一般的な設定をオーバーライド)                                                                                                                                                                                                                                                                                                                                                                               | `grpc`、`http/json`、`http/protobuf`                                                                         |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | OTLP ログエンドポイント (一般的な設定をオーバーライド)                                                                                                                                                                                                                                                                                                                                                                         | `http://localhost:4318/v1/logs`                                                                            |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | OTLP の認証ヘッダー                                                                                                                                                                                                                                                                                                                                                                                            | `Authorization=Bearer token`                                                                               |
| `OTEL_EXPORTER_OTLP_METRICS_HEADERS`                | メトリクスの認証ヘッダー (一般的なヘッダーとマージ)                                                                                                                                                                                                                                                                                                                                                                             | `Authorization=Bearer token`                                                                               |
| `OTEL_EXPORTER_OTLP_LOGS_HEADERS`                   | ログの認証ヘッダー (一般的なヘッダーとマージ)                                                                                                                                                                                                                                                                                                                                                                                | `Authorization=Bearer token`                                                                               |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | エクスポート間隔 (ミリ秒単位、デフォルト: 60000)                                                                                                                                                                                                                                                                                                                                                                           | `5000`、`60000`                                                                                             |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | ログエクスポート間隔 (ミリ秒単位、デフォルト: 5000)                                                                                                                                                                                                                                                                                                                                                                          | `1000`、`10000`                                                                                             |
| `OTEL_LOG_USER_PROMPTS`                             | ユーザープロンプトコンテンツのログを有効にする (デフォルト: 無効)                                                                                                                                                                                                                                                                                                                                                                     | `1` で有効化                                                                                                   |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | `assistant_response` イベントでアシスタント応答テキストのログを有効にする (デフォルト: 無効)。設定されていない場合、`OTEL_LOG_USER_PROMPTS` の値にフォールバックします。Claude Code v2.1.193 以降が必要です                                                                                                                                                                                                                                                               | `1` で有効化、`0` でマスク状態を保持                                                                                     |
| `OTEL_LOG_TOOL_DETAILS`                             | ツールイベントおよびトレーススパン属性でツールパラメーターと入力引数のログを有効にする: Bash コマンド、MCP サーバーとツール名、スキル名、ユーザー作成ワークフロー名、ツール入力。また、`user_prompt` イベントでカスタム、プラグイン、MCP コマンド名を有効にします (デフォルト: 無効)。Claude Desktop の組み込みサーバーの場合、Claude Desktop が所有するセッションでは、フラグがオフでも `mcp_server_name`/`mcp_tool_name` は `tool_decision`/`tool_result` で出力されます。この例外には Claude Code v2.1.214 以降が必要です                                                              | `1` で有効化                                                                                                   |
| `OTEL_LOG_TOOL_CONTENT`                             | [`tool.output` スパンイベント](#tool-output-span-event)でツールコンテンツのログを有効にする (デフォルト: 無効)。スパン属性は[独自のゲート](#new-context-gates)の下でツールコンテンツを含みます。[トレース](#traces-beta)が必要です。コンテンツはコンテンツ制限で切り詰められます (デフォルト: 60 KB)                                                                                                                                                                                                       | `1` で有効化                                                                                                   |
| `OTEL_LOG_MANAGED_SETTINGS`                         | マスク処理された管理設定と、マスク処理前の設定の SHA-256 ダイジェストを [管理設定解決](#managed-settings-resolved-event)イベントに追加します (デフォルト: 無効)。プロジェクトまたはローカル設定の値はそれをオンにしません。Claude Code v2.1.274 以降が必要です                                                                                                                                                                                                                                     | `1` で有効化                                                                                                   |
| `OTEL_LOG_RAW_API_BODIES`                           | Anthropic Messages API リクエストとレスポンス JSON 全体を `api_request_body` / `api_response_body` ログイベントとして出力します (デフォルト: 無効)。ボディには会話履歴全体が含まれます。これを有効にすることは、`OTEL_LOG_USER_PROMPTS`、`OTEL_LOG_TOOL_DETAILS`、および `OTEL_LOG_TOOL_CONTENT` が明かすすべてのものに同意することを意味します                                                                                                                                                       | `1` でコンテンツ制限で切り詰められたインラインボディ (デフォルト: 60 KB)、または `file:<dir>` でディスク上の切り詰められていないボディと、イベント内の `body_ref` ポインター |
| `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH`               | コンテンツ制限: モデルレスポンス、ツールコンテンツ、システムプロンプト、生 API ボディなどのコンテンツを含む属性の最大長 (UTF-16 コード単位、デフォルト: 61440、つまり 60 KB)。デフォルトは 64 KB で属性値をキャップするバックエンド向けにサイズ設定されています。バックエンドがより大きな値を受け入れる場合はそれを上げるか、テレメトリ量を削減するために下げてください。OpenTelemetry SDK 属性制限 `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` またはそのログレコードおよびスパンバリアントがより低い値に設定されている場合、Claude Code はその小さい値で切り詰めるため、`[TRUNCATED ...]` マーカーは SDK 制限内に留まります。Claude Code v2.1.214 以降が必要です | `262144`                                                                                                   |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | メトリクスの時間性設定 (デフォルト: `delta`)。バックエンドが累積時間性を期待する場合は `cumulative` に設定                                                                                                                                                                                                                                                                                                                                      | `delta`、`cumulative`                                                                                       |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | 動的ヘッダーを更新するための間隔 (デフォルト: 1740000ms / 29 分)                                                                                                                                                                                                                                                                                                                                                              | `900000`                                                                                                   |

`http/protobuf` および `http/json` プロトコルの場合、Claude Code は各エクスポートリクエストを `Content-Length` ヘッダーで送信します。v2.1.212 より前では、v2.1.191 以降の Claude Code バージョンはこれらのリクエストをチャンク転送エンコーディングで送信していました。Azure Monitor およびその他の宣言された長さを必要とするエンドポイントは、`411 Length Required` または `400` エラーでこれらを拒否していました。

<h3 id="mtls-authentication">
  mTLS 認証
</h3>

OTLP エクスポーターのクライアント証明書を設定する方法は、そのシグナルに使用されている OTLP プロトコルに依存し、`OTEL_EXPORTER_OTLP_PROTOCOL` またはシグナルごとのオーバーライドで設定されます。同じ設定がメトリクス、ログ、トレースに適用されます。

| プロトコル                       | クライアント証明書変数                                                                                                                                                  | コレクターの CA を信頼する方法                |
| :-------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `http/protobuf`、`http/json` | `CLAUDE_CODE_CLIENT_CERT`、`CLAUDE_CODE_CLIENT_KEY`、およびオプションで `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`。[ネットワーク設定](/docs/ja/network-config#mtls-authentication)を参照       | `NODE_EXTRA_CA_CERTS`            |
| `grpc`                      | `OTEL_EXPORTER_OTLP_CLIENT_KEY` および `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`、またはシグナルごとに異なる証明書を使用するための `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` などのシグナルごとのバリアント | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

`grpc` の場合、OpenTelemetry SDK は標準 OTLP 変数を直接読み取るため、シグナルごとのメトリクス変数を設定する既存の設定は引き続き機能します。管理設定を持つマシンでは、Claude Code は[スタートアップ時に開発者が設定したシグナルごとの認証情報とエンドポイントを削除する](#how-managed-settings-lock-the-otlp-destination)可能性があります。

<h3 id="metrics-cardinality-control">
  メトリクスカーディナリティ制御
</h3>

以下の環境変数は、カーディナリティを管理するためにメトリクスに含まれる属性を制御します:

| 環境変数                                       | 説明                                                                                         | デフォルト値  | 無効化する例  |
| ------------------------------------------ | ------------------------------------------------------------------------------------------ | ------- | ------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | メトリクスに session.id 属性を含める                                                                   | `true`  | `false` |
| `OTEL_METRICS_INCLUDE_VERSION`             | メトリクスに app.version 属性を含める                                                                  | `false` | `true`  |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | メトリクスに user.account\_uuid および user.account\_id 属性を含める                                      | `true`  | `false` |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | メトリクスに app.entrypoint 属性を含める                                                               | `false` | `true`  |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | `OTEL_RESOURCE_ATTRIBUTES` からのキーをメトリクスデータポイントの属性として含める                                     | `true`  | `false` |
| `OTEL_METRICS_INCLUDE_REPOSITORY`          | メトリクスおよびイベントに `vcs.*` [リポジトリ識別属性](#repository-attributes)を含める。Claude Code v2.1.269 以降が必要です | `false` | `true`  |

カーディナリティが低いほど、一般的にパフォーマンスが向上し、ストレージコストが低くなりますが、分析用のより詳細なデータは少なくなります。

<h3 id="traces-beta">
  トレース (ベータ)
</h3>

分散トレースは、各ユーザープロンプトをそれがトリガーする API リクエストとツール実行にリンクするスパンをエクスポートします。これにより、トレーシングバックエンドで完全なリクエストを単一のトレースとして表示できます。

トレースはデフォルトでオフです。有効にするには、`CLAUDE_CODE_ENABLE_TELEMETRY=1` と `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` の両方を設定してから、`OTEL_TRACES_EXPORTER` を設定してスパンの送信先を選択します。トレースは、エンドポイント、プロトコル、ヘッダー、および [mTLS](#mtls-authentication)について [一般的な OTLP 設定](#common-configuration-variables)を再利用します。管理設定を持つマシンでは、Claude Code は[スタートアップ時に開発者が設定したシグナルごとの認証情報とエンドポイントを削除する](#how-managed-settings-lock-the-otlp-destination)可能性があります。

| 環境変数                                  | 説明                                                            | 例の値                                |
| ------------------------------------- | ------------------------------------------------------------- | ---------------------------------- |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | スパントレースを有効にする (必須)。`ENABLE_ENHANCED_TELEMETRY_BETA` も受け入れられます | `1`                                |
| `OTEL_TRACES_EXPORTER`                | トレースエクスポーターのタイプ (カンマ区切り)。`none` を使用して無効化                      | `console`、`otlp`、`none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | トレースのプロトコル (`OTEL_EXPORTER_OTLP_PROTOCOL` をオーバーライド)           | `grpc`、`http/json`、`http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | OTLP トレースエンドポイント (`OTEL_EXPORTER_OTLP_ENDPOINT` をオーバーライド)     | `http://localhost:4318/v1/traces`  |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS`   | トレースの認証ヘッダー (`OTEL_EXPORTER_OTLP_HEADERS` とマージ)               | `Authorization=Bearer token`       |
| `OTEL_TRACES_EXPORT_INTERVAL`         | スパンバッチエクスポート間隔 (ミリ秒単位、デフォルト: 5000)                            | `1000`、`10000`                     |

スパンはデフォルトでユーザープロンプトテキスト、ツール入力詳細、ツールコンテンツをマスクします。これらを含めるには、`OTEL_LOG_USER_PROMPTS=1`、`OTEL_LOG_TOOL_DETAILS=1`、および `OTEL_LOG_TOOL_CONTENT=1` を設定します。

トレースがアクティブな場合、Bash および PowerShell サブプロセスは、アクティブなツール実行スパンの W3C トレースコンテキストを含む `TRACEPARENT` 環境変数を自動的に継承します。これにより、`TRACEPARENT` を読み取るサブプロセスは、同じトレースの下に独自のスパンを親にすることができ、Claude が実行するスクリプトとコマンドを通じたエンドツーエンドの分散トレースが可能になります。

トレースがアクティブで Claude Code が Anthropic API に直接接続されている場合、各モデルリクエストは W3C `traceparent` ヘッダーを含み、これは `claude_code.llm_request` スパンのコンテキストに設定され、API の `traceresponse` ヘッダーはスパンリンクとして記録されます。これらは、Claude Code のクライアント側スパンをサーバー側トレースに接続し、準拠した仲介者を通じて接続します。アウトバウンド HTTP MCP リクエストは同じ方法で `traceparent` を含みます。ヘッダーはサードパーティプロバイダーには送信されません。

デフォルトでは、モデルおよび HTTP MCP リクエストの `traceparent` ヘッダーは、`ANTHROPIC_BASE_URL` が設定されていないか Anthropic API を指している場合にのみ送信されます。一部のプロキシは認識されないヘッダーを拒否するためです。サブプロセス `TRACEPARENT` 変数は一貫性のために同じスイッチで制御されます。カスタム `ANTHROPIC_BASE_URL` プロキシを通じて Claude Code を実行し、トレースコンテキストを伝播させたい場合は、`CLAUDE_CODE_PROPAGATE_TRACEPARENT=1` を設定します。

Agent SDK および `-p` で開始された非対話型セッションでは、Claude Code は各インタラクションスパンを開始するときに独自の環境から `TRACEPARENT` と `TRACESTATE` も読み取ります。これにより、埋め込みプロセスがアクティブな W3C トレースコンテキストをサブプロセスに渡すことができるため、Claude Code のスパンは呼び出し元の分散トレースの子として表示されます。対話型セッションは、CI またはコンテナ環境からの環境値を誤って継承するのを避けるため、インバウンド `TRACEPARENT` を無視します。

インバウンドトレースコンテキストは [イベント](#events)にも適用されます。`TRACEPARENT` が設定されている Agent SDK および `-p` セッションでは、各 OTLP イベントログレコードは `trace_id` および `span_id` 値を含み、トレースエクスポーターが設定されていない場合でも、これをアプリケーションのトレースに結合するため、ログバックエンドはイベントをトレースの残りの部分と相関させることができます。

インタラクションがアクティブな間に出力されたレコードは、インタラクションスパンの ID を含み、Claude Code がそれをスパンの非同期コンテキスト外で出力する場合でも、例えば権限プロンプトコールバックまたはスタートアップ中にバッファリングされ、後で出力されたレコードの場合でも同様です。アクティブなインタラクションスパンなしで出力されたレコードは、インバウンド `TRACEPARENT` ID を直接含みます。v2.1.214 より前では、スパンの非同期コンテキスト外で出力されたレコードは、スパンの ID の代わりにインバウンド `TRACEPARENT` ID を含んでいました。v2.1.212 より前では、アクティブなスパン外で出力されたイベントレコードは `trace_id` または `span_id` を含んでいませんでした。

<h4 id="span-hierarchy">
  スパン階層
</h4>

各ユーザープロンプトは `claude_code.interaction` ルートスパンを開始します。API 呼び出し、ツール呼び出し、フック実行はその子として記録されます。ツールスパンには 2 つの子スパンがあります: 1 つは権限決定の待機に費やされた時間用、もう 1 つは実行自体用です。Agent ツール、またはレガシー Task ツールがサブエージェントを生成する場合、サブエージェントの API とツールスパンは親の `claude_code.tool` スパンの下にネストされます。

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (詳細なベータトレースが必要)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (Agent ツール) サブエージェント claude_code.llm_request / claude_code.tool スパン
```

Agent SDK および `claude -p` セッションでは、`TRACEPARENT` が環境に設定されている場合、`claude_code.interaction` 自体が呼び出し元のスパンの子になります。

`PreToolUse` フックが[ツール呼び出しを後で実行するために延期](/docs/ja/hooks#defer-a-tool-call-for-later)する場合、Claude Code はそれを延期したターンのトレースコンテキストを保存します。セッションを再開してツールが再実行されると、ツールのスパンはそれより前のターンのトレースに結合され、ターンの `claude_code.interaction` スパンの子になります。

<h4 id="span-attributes">
  スパン属性
</h4>

すべてのスパンは [標準属性](#standard-attributes)と、その名前に一致する `span.type` 属性を持ちます。以下の表は、各スパンに設定される追加属性をリストしています。`llm_request`、`tool.execution`、および `hook` スパンは、失敗を記録するときに OpenTelemetry ステータス `ERROR` を設定します。他のスパンは常にステータス `UNSET` で終了します。

**`claude_code.interaction`**

| 属性                        | 説明                                                                                                                        | ゲート                     |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | プロンプトテキスト。ゲートが設定されていない限り、値は `<REDACTED>` です                                                                               | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | プロンプト長 (文字数)                                                                                                              |                         |
| `interaction.sequence`    | インタラクションの 1 ベースカウンター。セッションごとではなく Claude Code プロセスごとにカウントされます。[`event.sequence`](#event-correlation-attributes)で説明されているとおり |                         |
| `parent.source`           | スパンがトレース親を取得した方法: インバウンド `TRACEPARENT` の下で親になった場合は `env`、独自のトレースを開始した場合は `none`。Claude Code v2.1.268 以降が必要です              |                         |
| `interaction.duration_ms` | ターンの実時間                                                                                                                   |                         |

**`claude_code.llm_request`**

| 属性                               | 説明                                                                                                                                                                                                         | ゲート                            |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `model`                          | モデル識別子                                                                                                                                                                                                     |                                |
| `gen_ai.system`                  | 常に `anthropic`。OpenTelemetry GenAI セマンティック規約                                                                                                                                                               |                                |
| `gen_ai.request.model`           | `model` と同じ値。OpenTelemetry GenAI セマンティック規約                                                                                                                                                                 |                                |
| `query_source`                   | リクエストを発行したサブシステム。例: `repl_main_thread` またはサブエージェント名                                                                                                                                                        | `ENABLE_BETA_TRACING_DETAILED` |
| `query_source_safe`              | `query_source` の制限された形式。詳細なベータトレースがアクティブかどうかに関わらず出力されます。`repl_main_thread` または `agent.builtin.general-purpose` などの値を持ちます。`:` は `.` になり、ユーザー名のエージェントは `agent.custom` として表示されます。Claude Code v2.1.268 以降が必要です |                                |
| `agent_id`                       | リクエストを発行したサブエージェントまたはチームメイトの識別子。メインセッションでは存在しません                                                                                                                                                           |                                |
| `parent_agent_id`                | このエージェントを生成したエージェントの識別子。メインセッションおよびそこから直接生成されたエージェントでは存在しません                                                                                                                                               |                                |
| `workflow.run_id`                | このエージェントを生成した [Workflow](/docs/ja/workflows) ツール実行の実行識別子。`wf_` で始まります。ワークフローによって生成されていないエージェントでは存在しません                                                                                                          |                                |
| `workflow.name`                  | このエージェントを生成したワークフローの名前。ユーザー作成名はゲートが設定されていない限り `custom` に置き換えられます                                                                                                                                           | `OTEL_LOG_TOOL_DETAILS`        |
| `speed`                          | `fast` または `normal`                                                                                                                                                                                        |                                |
| `effort`                         | [リクエストに適用される努力レベル](/docs/ja/model-config#adjust-effort-level): `low`、`medium`、`high`、`xhigh`、または `max`。Claude Code が努力レベルを送信しない場合は存在しません。例えば、努力をサポートしていないモデルの場合。Claude Code v2.1.274 以降が必要です                    |                                |
| `llm_request.context`            | 親スパンに応じて `interaction`、`tool`、または `standalone`                                                                                                                                                             |                                |
| `duration_ms`                    | 再試行を含む実時間                                                                                                                                                                                                  |                                |
| `ttft_ms`                        | 最初のトークンまでの時間 (ミリ秒単位)                                                                                                                                                                                       |                                |
| `first_content_ms`               | リクエスト開始から成功した試行の最初のコンテンツブロックまでの時間 (ミリ秒単位)。ストリーミング以外のパスにフォールバックしたリクエストでは存在しません。Claude Code v2.1.268 以降が必要です                                                                                                 |                                |
| `input_tokens`                   | API 使用ブロックからの入力トークン数                                                                                                                                                                                       |                                |
| `output_tokens`                  | 出力トークン数                                                                                                                                                                                                    |                                |
| `cache_read_tokens`              | プロンプトキャッシュから読み取られたトークン                                                                                                                                                                                     |                                |
| `cache_creation_tokens`          | プロンプトキャッシュに書き込まれたトークン                                                                                                                                                                                      |                                |
| `request_id`                     | レスポンスヘッダーの `request-id` からの Anthropic API リクエスト ID                                                                                                                                                         |                                |
| `gen_ai.response.id`             | `request_id` と同じ値。OpenTelemetry GenAI セマンティック規約                                                                                                                                                            |                                |
| `client_request_id`              | 最終試行のクライアント生成 `x-client-request-id`                                                                                                                                                                        |                                |
| `attempt`                        | このリクエストに対して行われた総試行回数                                                                                                                                                                                       |                                |
| `success`                        | `true` または `false`                                                                                                                                                                                         |                                |
| `status_code`                    | リクエストが失敗した場合の HTTP ステータスコード                                                                                                                                                                                |                                |
| `error`                          | リクエストが失敗した場合のエラーメッセージ                                                                                                                                                                                      |                                |
| `error_class`                    | リクエストが失敗した場合の短いエラークラストークン。例: `api_timeout` または `server_overload`。Claude Code v2.1.268 以降が必要です                                                                                                              |                                |
| `response.has_tool_call`         | レスポンスにツール使用ブロックが含まれている場合は `true`                                                                                                                                                                           |                                |
| `stop_reason`                    | API レスポンス `stop_reason`。例: `end_turn`、`tool_use`、`max_tokens`、`stop_sequence`、`pause_turn`、または `refusal`                                                                                                   |                                |
| `gen_ai.response.finish_reasons` | `stop_reason` と同じ値。文字列配列でラップされています。OpenTelemetry GenAI セマンティック規約                                                                                                                                           |                                |

各再試行試行は、`attempt` および `client_request_id` 属性を持つ `gen_ai.request.attempt` スパンイベントとしても記録されます。

**`claude_code.tool`**

| 属性                    | 説明                                                                                                                                                                              | ゲート                     |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | ツール名                                                                                                                                                                            |                         |
| `tool_name_safe`      | ユーザーが選択した名前を含まない `tool_name` の形式。組み込みツール名はそのまま渡されます。MCP ツール名は `mcp_other` として表示されます。ただし、`playwright` ツールの `browser_*` など、固定の形状に一致するツール名は、そのまま渡されます。Claude Code v2.1.268 以降が必要です |                         |
| `bash_command_class`  | Bash ツールの場合: 固定リストからのコマンドの最初のプログラムのカテゴリ。例: `vcs` または `package_manager`。リスト外のプログラムの場合は `other`、行を解析できない場合は `unparsed`。Claude Code v2.1.268 以降が必要です                               |                         |
| `bash_argv0`          | Bash ツールの場合: 同じ固定リスト上にあるコマンドの最初のプログラム。例: `git` または `npm`。リスト外のプログラムの場合は `other`。Claude Code v2.1.268 以降が必要です                                                                    |                         |
| `duration_ms`         | 権限待機と実行を含む実時間                                                                                                                                                                   |                         |
| `result_tokens`       | ツール結果のおおよそのトークンサイズ                                                                                                                                                              |                         |
| `agent_id`            | ツールを実行したサブエージェントまたはチームメイトの識別子。メインセッションでは存在しません                                                                                                                                  |                         |
| `parent_agent_id`     | このエージェントを生成したエージェントの識別子。メインセッションおよびそこから直接生成されたエージェントでは存在しません                                                                                                                    |                         |
| `workflow.run_id`     | このエージェントを生成した Workflow ツール実行の実行識別子。`wf_` で始まります。ワークフローによって生成されていないエージェントでは存在しません                                                                                                |                         |
| `workflow.name`       | このエージェントを生成したワークフローの名前。ユーザー作成名はゲートが設定されていない限り `custom` に置き換えられます                                                                                                                | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | このコールのモデルの `tool_use` ブロック ID。[tool\_result](#tool-result-event) および [tool\_decision](#tool-decision-event) イベントおよびフックペイロード内の `tool_use_id` と一致するため、スパンをこれらのレコードに結合できます         |                         |
| `gen_ai.tool.call.id` | `tool_use_id` と同じ値。OpenTelemetry GenAI セマンティック規約                                                                                                                                |                         |
| `file_path`           | Read、Edit、Write ツールのターゲットファイルパス                                                                                                                                                 | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Bash ツールのコマンド文字列                                                                                                                                                                | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Skill ツールのスキル名                                                                                                                                                                  | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Agent ツールまたはレガシー Task ツールのサブエージェントタイプ                                                                                                                                           | `OTEL_LOG_TOOL_DETAILS` |

<span id="tool-output-span-event" />**`tool.output` スパンイベント (`claude_code.tool` 上)**

`OTEL_LOG_TOOL_CONTENT=1` を設定した場合、Read および Bash 呼び出しは `claude_code.tool` スパン上に `tool.output` スパンイベントを記録できます。Edit および Write 呼び出しは、`OTEL_LOG_TOOL_DETAILS=1` も設定した場合にのみ記録します。その変数はこれら 2 つのツールにスコープされていないため、設定テーブルの[その行](#common-configuration-variables)で、それが他の場所に追加する引数を確認してください。

Claude Code はツール呼び出しの成功した戻りからこのイベントを書き込むため、エラーを発生させる呼び出しは何も記録しません。戻りを行う呼び出しの中で、以下の場合は `tool.output` イベントを記録しません:

* Read、Edit、Write、Bash 以外のツール (MCP ツールおよび WebFetch を含む) への呼び出し
* 画像、PDF、または内容が変更されていないファイルの再読み込みなど、ファイルテキスト以外のものを返す Read
* `OTEL_LOG_TOOL_DETAILS=1` も設定しない限り、Edit または Write 呼び出し

イベントはこれらの属性を含み、各属性はコンテンツ制限で切り詰められます (デフォルト: 60 KB)。`Gated by` は、属性が必要とする変数を名前付けます。Edit および Write の場合、その変数は属性ではなくイベント自体をゲートします。

| 属性             | 説明                                              | ゲート                                  |
| -------------- | ----------------------------------------------- | ------------------------------------ |
| `content`      | Read ツールが返したテキスト、または Write 呼び出しが書き込むよう求めたテキスト   | `OTEL_LOG_TOOL_DETAILS` (Write ツール用) |
| `output`       | Bash コマンドの結合出力 (stderr は stdout にインターリーブ)       |                                      |
| `diff`         | Edit ツールが適用した構造化パッチ                             | `OTEL_LOG_TOOL_DETAILS`              |
| `file_path`    | Read、Edit、Write ツールのターゲットファイルパス。同じ名前のスパン属性を繰り返す | `OTEL_LOG_TOOL_DETAILS`              |
| `bash_command` | Bash ツールのコマンド文字列                                | `OTEL_LOG_TOOL_DETAILS`              |

親スパンの `tool_name` 属性は、イベントがどのツールから来たかを示します。コンテンツ制限で切り詰められた属性には、`<attribute>_truncated` および `<attribute>_original_length` が付属します。

**`claude_code.tool.blocked_on_user`**

| 属性            | 説明                                                    | ゲート |
| ------------- | ----------------------------------------------------- | --- |
| `duration_ms` | 権限決定の待機に費やされた時間                                       |     |
| `decision`    | `accept` または `reject`                                 |     |
| `source`      | 決定ソース。[Tool decision event](#tool-decision-event) と一致 |     |

**`claude_code.tool.execution`**

| 属性                    | 説明                                                                                                                                            | ゲート                     |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | ツール本体の実行に費やされた時間                                                                                                                              |                         |
| `tool_use_id`         | 親 `claude_code.tool` スパンと同じ値                                                                                                                  |                         |
| `gen_ai.tool.call.id` | `tool_use_id` と同じ値。OpenTelemetry GenAI セマンティック規約                                                                                              |                         |
| `success`             | `true` または `false`                                                                                                                            |                         |
| `error`               | 実行が失敗した場合のエラーカテゴリ文字列。例: `Error:ENOENT` または `ShellError`。ゲートが設定されている場合は完全なエラーメッセージを含む                                                          | `OTEL_LOG_TOOL_DETAILS` |
| `error_class`         | 文字、数字、アンダースコア以外の文字を `_` に置き換えた識別子形式のエラーカテゴリ。例: `Error_ENOENT` または `ShellError`。`error` が完全なメッセージを含む場合でもカテゴリを含みます。Claude Code v2.1.268 以降が必要です |                         |

**`claude_code.hook`**

このスパンは、詳細なベータトレースがアクティブな場合にのみ出力されます。これには `ENABLE_BETA_TRACING_DETAILED=1` と `BETA_TRACING_ENDPOINT` が必要です。このペアは、[ログとトレースの送信先を変更](/docs/ja/env-vars#variables)します。ペアをシェル、ユーザー設定、または管理設定で設定します。両方の変数は [プロジェクトおよびローカル設定](/docs/ja/settings-reference#variables-claude-code-ignores-in-env)では無視されます。`CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` のみでは出力されません。

対話型 CLI セッションでは、詳細なベータトレースは、組織がこの機能のホワイトリストに登録されていることも必要です。Agent SDK および非対話型 `-p` セッションはホワイトリストを必要としません。

| 属性                       | 説明                            | ゲート                     |
| ------------------------ | ----------------------------- | ----------------------- |
| `hook_event`             | フックイベントタイプ。例: `PreToolUse`    |                         |
| `hook_name`              | 完全なフック名。例: `PreToolUse:Write` |                         |
| `num_hooks`              | 実行された一致するフックコマンドの数            |                         |
| `hook_definitions`       | JSON シリアル化されたフック設定            | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms`            | すべての一致するフックの実時間               |                         |
| `num_success`            | 正常に完了したフックの数                  |                         |
| `num_blocking`           | ブロッキング決定を返したフックの数             |                         |
| `num_non_blocking_error` | ブロックなしで失敗したフックの数              |                         |
| `num_cancelled`          | 完了前にキャンセルされたフックの数             |                         |

<span id="new-context-gates" />

<Note>
  `new_context`、`system_prompt_preview`、`user_system_prompt`、`tool_input`、`response.model_output` などの追加のコンテンツを含む属性は、詳細なベータトレースがアクティブな場合にのみ出力されます。これらは安定したスパンスキーマの一部ではありません。

  `new_context` のゲートは、どのスパンがそれを含むかに依存し、各コピーはコンテンツ制限で切り詰められます (デフォルト: 60 KB)。`claude_code.tool` スパン上では、そのツール呼び出しの結果を含み、ツールに関わらず、`OTEL_LOG_TOOL_CONTENT=1` が必要です。`claude_code.interaction` スパン上では、ユーザープロンプトを含み、`claude_code.llm_request` スパン上ではそのリクエストの新しいユーザーメッセージとツール結果を含みます。どちらも `OTEL_LOG_USER_PROMPTS=1` が必要です。

  `user_system_prompt` はさらに `OTEL_LOG_USER_PROMPTS=1` が必要です。これは `systemPrompt` SDK オプションまたは `--system-prompt` および `--append-system-prompt` フラグを通じて提供するシステムプロンプトテキストのみを含み、コンテンツ制限で切り詰められ (デフォルト: 60 KB)、リクエストごとではなくセッションごとに 1 回出力されます。
</Note>

<h3 id="dynamic-headers">
  動的ヘッダー
</h3>

動的認証が必要なエンタープライズ環境では、ヘッダーを動的に生成するスクリプトを設定できます。動的ヘッダーは `http/protobuf` および `http/json` プロトコルにのみ適用されます。`grpc` プロトコルでは、Claude Code は静的なヘッダー変数 `OTEL_EXPORTER_OTLP_HEADERS` およびそのシグナルごとのバリアントのみを使用します。

<h4 id="settings-configuration">
  設定ファイルの設定
</h4>

`.claude/settings.json` に追加します。パスを独自のスクリプトに置き換えます:

```json theme={null}
{
  "otelHeadersHelper": "/path/to/generate-otel-headers.sh"
}
```

値は、スペースを含むパスを含む実行可能ファイルへのパス、またはシェルコマンドラインと引数です。Windows では、値は常にシェルを通じて実行されるため、JSON 値内にスペースを含むパスをクォートで囲みます。

<h4 id="script-requirements">
  スクリプト要件
</h4>

スクリプトは HTTP ヘッダーを表す文字列キーと値のペアを持つ有効な JSON を出力する必要があります:

```bash theme={null}
#!/bin/bash
# 例: 複数のヘッダー
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

ヘルパーが失敗するか、これらの要件を満たさない出力を出力する場合、エクスポートは失敗し、ヘルパーが再び機能するまで、セッションからテレメトリバックエンドは何も受け取りません。Claude Code は以下の場所で失敗を報告します:

* 対話型セッションの警告通知。[`otelHeadersHelper failed; telemetry is not being exported`](/docs/ja/errors#otelheadershelper-failed)。ヘルパーが最初に失敗したときにセッションごとに 1 回表示されます
* `/status` 出力
* [`--debug`](/docs/ja/cli-reference#cli-flags) で実行するか、セッション内で `/debug` を実行した後のデバッグログ
* stderr、`-p` で開始された非対話型セッション内

<h4 id="refresh-behavior">
  リフレッシュ動作
</h4>

ヘッダーヘルパースクリプトはスタートアップ時に実行され、その後定期的に実行されてトークンリフレッシュをサポートします。デフォルトでは、スクリプトは 29 分ごとに実行されます。`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS` 環境変数で間隔をカスタマイズします。

<h3 id="multi-team-organization-support">
  マルチチーム組織サポート
</h3>

複数のチームまたは部門を持つ組織は、`OTEL_RESOURCE_ATTRIBUTES` 環境変数を使用してカスタム属性を追加し、異なるグループを区別できます:

```bash theme={null}
# チーム識別用のカスタム属性を追加する
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

これらのカスタム属性はすべてのメトリクスとイベントに含まれ、以下のことが可能になります:

* チームまたは部門別にメトリクスをフィルタリングする
* コストセンターごとのコストを追跡する
* チーム固有のダッシュボードを作成する
* 特定のチームのアラートを設定する

Claude Code はこれらの値をすべてのメトリクスデータポイントとイベントレコードの属性として、OTLP リソースブロックで送信することに加えて、属性として付加します。ほとんどのメトリクスバックエンドはデータポイント属性をクエリ可能なラベルとして公開しているため、カスタムキーで直接メトリクスをグループ化およびフィルタリングできます。`vcs.*` [リポジトリ属性](#repository-attributes)を除き、カスタムキーは `user.id` または `session.id` などの [標準属性](#standard-attributes)をオーバーライドしません: キーが衝突する場合、Claude Code は組み込み値を保持します。

各カスタムキーはすべてのメトリクスシリーズのラベルになるため、高カーディナリティ値はメトリクスバックエンドのストレージコストを増加させます。カスタム属性をリソースブロックのみで送信し、データポイントラベルから省略するには、`OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false` を設定します。[メトリクスカーディナリティ制御](#metrics-cardinality-control)を参照してください。

<Warning>
  `OTEL_RESOURCE_ATTRIBUTES` 環境変数はカンマ区切りのキー=値ペアを使用し、厳密なフォーマット要件があります:

  * **スペースは許可されません**: 値にスペースを含めることはできません。例えば、`user.organizationName=My Company` は無効です
  * **フォーマット**: カンマ区切りのキー=値ペアである必要があります: `key1=value1,key2=value2`
  * **許可される文字**: 制御文字、空白、ダブルクォート、カンマ、セミコロン、バックスラッシュを除く US-ASCII 文字のみ
  * **特殊文字**: 許可された範囲外の文字はパーセントエンコードする必要があります

  スペースが必要な値の場合は、代わりにアンダースコアまたはキャメルケースを使用します。以下の例は、各形式で `org.name` を設定します:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
  export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
  ```

  許可された範囲外の文字だけでなく、任意の文字をパーセントエンコードできます。この例は、スペースとアポストロフィの両方をエンコードします:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
  ```

  値をクォートで囲むことはスペースをエスケープしません。例えば、`org.name="My Company"` は `My Company` ではなく、リテラル値 `"My Company"` (クォート付き) になります。
</Warning>

<h3 id="example-configurations">
  設定例
</h3>

`claude` を実行する前にこれらの環境変数を設定します。各シナリオ以下は完全な設定を示しており、各変数は [一般的な設定変数](#common-configuration-variables)で説明されています。設定が有効になったことを確認するには、セッションを開始した後、バックエンドで `claude_code.session.count` メトリクスを確認します。[クイックスタート](#quick-start)はログのみの検証とアクティブなものがない場合に確認する内容をカバーしています。

コンソールデバッグ (1 秒間隔):

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000
```

OTLP over gRPC:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Prometheus (`http://localhost:9464/metrics` からスクレイプ):

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus
```

[自己ホスト環境](/docs/ja/self-hosted-environments-reference#pass-through-session-child-metrics)では、セッションはランナーのデフォルト容量である 1 でのみポート 9464 をバインドします。より高い容量では、ランナーは代わりに独自の `/metrics` エンドポイントでセッションカウンターとゲージを再公開します。

複数のエクスポーターにメトリクスを送信:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json
```

メトリクスとログを異なるエンドポイントまたはバックエンドに送信:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317
```

メトリクスのみをエクスポート (イベント/ログなし):

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

イベント/ログのみをエクスポート (メトリクスなし):

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  利用可能なメトリクスとイベント
</h2>

<h3 id="standard-attributes">
  標準属性
</h3>

すべてのメトリクスとイベントは、これらの標準属性を共有します。

| 属性                                                                                   | 説明                                                                                                              | 制御対象                                                                       |
| ------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `session.id`                                                                         | 一意のセッション識別子                                                                                                     | `OTEL_METRICS_INCLUDE_SESSION_ID`（デフォルト: true）                             |
| `app.version`                                                                        | 現在の Claude Code バージョン                                                                                           | `OTEL_METRICS_INCLUDE_VERSION`（デフォルト: false）                               |
| `app.entrypoint`                                                                     | セッションの起動方法。`cli`、`sdk-cli`、`sdk-ts`、`sdk-py`、`claude-vscode` など                                                 | `OTEL_METRICS_INCLUDE_ENTRYPOINT`（デフォルト: false）                            |
| `organization.id`                                                                    | 組織 UUID（認証時）                                                                                                    | 利用可能な場合は常に含まれます                                                            |
| `user.account_uuid`                                                                  | アカウント UUID（認証時）                                                                                                 | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`（デフォルト: true）                           |
| `user.account_id`                                                                    | Anthropic 管理 API に一致するタグ付き形式のアカウント ID（認証時）。例：`user_01BWBeN28...`                                                | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`（デフォルト: true）                           |
| `user.id`                                                                            | 初回実行時に生成され、`~/.claude.json` に保持されるランダムな匿名識別子。個人情報は含まれず、Claude アカウントから派生していません。ファイルを削除すると、次回実行時に新しい無関係な値が生成されます。 | 常に含まれます                                                                    |
| `user.email`                                                                         | ユーザーのメールアドレス。サインイン時のメールアドレス、または [クラウドセッション](/docs/ja/claude-code-on-the-web) の場合はセッション自体の認証情報から取得                    | 利用可能な場合は常に含まれます                                                            |
| `terminal.type`                                                                      | ターミナルタイプ。`iTerm.app`、`vscode`、`cursor`、`tmux` など                                                                | 検出された場合は常に含まれます                                                            |
| `OTEL_RESOURCE_ATTRIBUTES` からのキー                                                     | `department` や `team.id` など、設定したカスタム属性。[マルチチーム組織サポート](#multi-team-organization-support) を参照                     | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES`（デフォルト: true）                    |
| `vcs.repository.url.full`、`vcs.owner.name`、`vcs.repository.name`、`vcs.provider.name` | セッションリポジトリの ID。`origin` リモートから派生。[リポジトリ属性](#repository-attributes) を参照                                          | `OTEL_METRICS_INCLUDE_REPOSITORY`（デフォルト: false）。Claude Code v2.1.269 以降が必要 |

Claude Code が [Claude アプリゲートウェイ](/docs/ja/claude-apps-gateway) にサインインしている場合、CLI はゲートウェイセッションの認証済みアイデンティティでエクスポートをスタンプします。`user.id` は匿名インストール識別子ではなく IdP サブジェクト、`user.email` はサインイン済みメール、`user.groups` は IdP グループメンバーシップをコンマ区切り文字列として保持します。各エクスポートは `identity.source: gateway-oidc` も保持します。ゲートウェイアイデンティティは最後に適用されるため、`OTEL_RESOURCE_ATTRIBUTES` を通じて設定された `user.*` および `identity.*` キーはゲートウェイセッションで無視されます。

イベントには、以下の追加属性が含まれます。これらはメトリクスに添付されることはありません。無制限のカーディナリティを引き起こすためです。

* `prompt.id`: ユーザープロンプトと、次のプロンプトまでのすべての後続イベントを相関させる UUID。[イベント相関属性](#event-correlation-attributes) を参照。
* `workspace.host_paths`: デスクトップアプリで選択されたホストワークスペースディレクトリ。文字列配列として
* `workflow.run_id`: [Workflow](/docs/ja/workflows) ツール実行に属するエージェントが発行する API およびツールイベントのプレフィックス `wf_` の実行識別子。イベントを 1 つの `workflow.run_id` でフィルタリングすると、その実行の API リクエストとツール結果が再構成されます。識別子は、ワークフロースクリプトが生成するエージェントと、それらが順番に生成するエージェント（スキル呼び出しなど）をカバーします。Workflow ツール結果で報告される実行識別子と一致します。他のすべてのイベントには存在しません。Claude Code v2.1.202 以降が必要
* `workflow.name`: ワークフローの名前。スクリプトの `meta.name`。`workflow.run_id` と一緒に発行されます。実行が未修正の組み込みスクリプトを実行する場合、組み込みワークフロー名はそのまま表示されます。ユーザー作成の名前（組み込みスクリプトの編集済みコピーを含む）は、`OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り `custom` に置き換えられます。Claude Code v2.1.202 以降が必要

<h4 id="repository-attributes">
  リポジトリ属性
</h4>

`OTEL_METRICS_INCLUDE_REPOSITORY=true` を設定して、メトリクスとイベントをセッションのリポジトリの ID でタグ付けします。共有コレクターが使用状況をリポジトリごとに属性付けできるようにします。Claude Code v2.1.269 以降が必要です。

Claude Code はセッションごとに 1 回、リポジトリの `origin` リモートからこれらの属性を派生させます。1 つのリポジトリの HTTPS および SSH リモートは同じ値を生成します。

| 属性                        | 値                                                                                                         |
| ------------------------- | --------------------------------------------------------------------------------------------------------- |
| `vcs.repository.url.full` | リポジトリのブラウザ URL（`.git` なし）。例：`https://github.com/example-org/example-repo`                                 |
| `vcs.owner.name`          | オーナーまたはグループパス。例：`example-org`。リモートパスが単一セグメントの場合は省略                                                        |
| `vcs.repository.name`     | 裸のリポジトリ名。例：`example-repo`                                                                                 |
| `vcs.provider.name`       | Claude Code がリモートのホストまたは URL 形状を `github`、`gitlab`、`bitbucket`、`gitea` のいずれかとして認識する場合はそのプロバイダー。それ以外の場合は省略 |

値は小文字に変換され、リモート URL からの認証情報、クエリ文字列、フラグメントは決して表示されません。セッションに `origin` リモートがない場合、リモートが URL 形状でない場合、または唯一の囲むリポジトリがホームディレクトリである場合、属性は省略されます。

[`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support) で宣言した `vcs.*` キーは、そのキーの派生値を置き換えます。`vcs.repository.url.full` を宣言した場合、Claude Code はリモートを読み取らず、宣言したキーのみを報告します。

属性は独自のエクスポーターにのみフローします。Anthropic のテレメトリはすべての `vcs.*` キーをドロップします。

<h3 id="metrics">
  メトリクス
</h3>

Claude Code は以下のメトリクスをエクスポートします。Unit 列は各メトリクスに添付される OpenTelemetry ユニット文字列を示します。カウントメトリクスには何も含まれません。

| メトリクス名                                | 説明                    | ユニット   |
| ------------------------------------- | --------------------- | ------ |
| `claude_code.session.count`           | 開始された CLI セッションのカウント  | なし     |
| `claude_code.lines_of_code.count`     | 変更されたコード行のカウント        | なし     |
| `claude_code.pull_request.count`      | 作成されたプルリクエストの数        | なし     |
| `claude_code.commit.count`            | 作成された git コミットの数      | なし     |
| `claude_code.cost.usage`              | Claude Code セッションのコスト | USD    |
| `claude_code.token.usage`             | 使用されたトークン数            | tokens |
| `claude_code.code_edit_tool.decision` | コード編集ツール権限決定のカウント     | なし     |
| `claude_code.active_time.total`       | 総アクティブ時間              | s      |

`prometheus` が `OTEL_METRICS_EXPORTER` にリストされた唯一のエクスポーターである場合、Claude Code はエクスポートされたメトリクスから `USD`、`tokens`、`s` ユニットを省略して、スクレイプが有効な Prometheus テキスト形式のままになるようにします。メトリクス名は変わらず、`otlp,prometheus` などのエクスポーターを組み合わせた設定はユニットを保持します。v2.1.216 より前では、Prometheus スクレイプには OpenMetrics のみの `# UNIT` 行が含まれていて、一部のスクレイパーが拒否していました。

<h3 id="metric-details">
  メトリクスの詳細
</h3>

各メトリクスには、上記にリストされた標準属性が含まれます。追加のコンテキスト固有の属性を持つメトリクスは以下に記載されています。

<h4 id="session-counter">
  セッションカウンター
</h4>

各セッションの開始時にインクリメントされます。

**属性**:

* すべての [標準属性](#standard-attributes)
* `start_type`: セッションの開始方法。`"fresh"`、`"resume"`、`"continue"`、または `"agents_view"` のいずれか。`"agents_view"` 値は `claude agents` ダッシュボードプロセス（会話セッションではなく、ユーザーが起動したローカル UI）を識別します。ダッシュボードで UI プロセス起動と会話セッションを分離するには、この値でフィルタリングします。

<h4 id="lines-of-code-counter">
  コード行カウンター
</h4>

コードが追加または削除されるとインクリメントされます。

**属性**:

* すべての [標準属性](#standard-attributes)
* `type`: （`"added"`、`"removed"`）
* `model`: 変更を加えたモデルのモデル識別子（例：「claude-sonnet-5」）

<h4 id="pull-request-counter">
  プルリクエストカウンター
</h4>

Claude Code がシェルコマンドまたは MCP ツールを通じてプルリクエストまたはマージリクエストを作成するとインクリメントされます。

**属性**:

* すべての [標準属性](#standard-attributes)

<h4 id="commit-counter">
  コミットカウンター
</h4>

Claude Code を介して git コミットを作成するとインクリメントされます。

**属性**:

* すべての [標準属性](#standard-attributes)

<h4 id="cost-counter">
  コストカウンター
</h4>

各 API リクエスト後にインクリメントされます。

**属性**:

* すべての [標準属性](#standard-attributes)
* `model`: モデル識別子（例：「claude-sonnet-5」）
* `query_source`: リクエストを発行したサブシステムのカテゴリ。`"main"`、`"subagent"`、または `"auxiliary"` のいずれか
* `speed`: リクエストが高速モードを使用した場合は `"fast"`。それ以外の場合は存在しません
* `effort`: リクエストに適用された [努力レベル](/docs/ja/model-config#adjust-effort-level)。`"low"`、`"medium"`、`"high"`、`"xhigh"`、または `"max"`。モデルが努力をサポートしていない場合は存在しません。
* `agent.name`: リクエストを発行したサブエージェントタイプ。組み込みエージェント名と公式マーケットプレイスプラグインのエージェントはそのまま表示されます。その他のユーザー定義エージェント名は `"custom"` に置き換えられます。`OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り。リクエストが名前付きサブエージェントタイプによって発行されなかった場合は存在しません。
* `skill.name`: リクエストに対してアクティブなスキル。Skill ツール、`/` コマンド、または生成されたサブエージェントによって継承されて設定されます。組み込み、バンドル、ユーザー定義、および公式マーケットプレイスプラグインスキル名はそのまま表示されます。サードパーティプラグインスキル名は `"third-party"` に置き換えられます。`OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り。アクティブなスキルがない場合は存在しません。
* `plugin.name`: アクティブなスキルまたはサブエージェントを提供するプラグインの所有者。公式マーケットプレイスプラグイン名はそのまま表示されます。サードパーティプラグイン名は `"third-party"` に置き換えられます。`OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り。スキルもサブエージェントも所有プラグインを持たない場合は存在しません。
* `marketplace.name`: 所有プラグインがインストールされたマーケットプレイス。公式マーケットプレイスプラグインに対してのみ発行されます。それ以外の場合は存在しません。
* `mcp_server.name`: このリクエストがツール結果を使用した MCP サーバー。組み込み、claude.ai プロキシ、および公式レジストリサーバー名はそのまま表示されます。ユーザー設定サーバー名は `"custom"` に置き換えられます。`OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り。リクエストが MCP ツール結果を使用しなかった場合は存在しません。v2.1.222 より前では、Claude Code は MCP ツール呼び出し後のすべてのリクエストにこの属性を設定していました。ツール結果を使用したリクエストのみではなく、アップグレード後のダッシュボードがこれを集計すると段階的に低下します。
* `mcp_tool.name`: このリクエストがツール結果を使用した MCP ツール。`mcp_server.name` と同じ削除およびバージョン動作を持ちます。リクエストが MCP ツール結果を使用しなかった場合は存在しません。

<h4 id="token-counter">
  トークンカウンター
</h4>

各 API リクエスト後にインクリメントされます。

**属性**:

* すべての [標準属性](#standard-attributes)
* `type`: （`"input"`、`"output"`、`"cacheRead"`、`"cacheCreation"`）
* `model`: モデル識別子（例：「claude-sonnet-5」）
* `query_source`: リクエストを発行したサブシステムのカテゴリ。`"main"`、`"subagent"`、または `"auxiliary"` のいずれか
* `speed`: リクエストが高速モードを使用した場合は `"fast"`。それ以外の場合は存在しません
* `effort`: リクエストに適用された [努力レベル](/docs/ja/model-config#adjust-effort-level)。詳細は [コストカウンター](#cost-counter) を参照。
* `agent.name`、`skill.name`、`plugin.name`、`marketplace.name`、`mcp_server.name`、`mcp_tool.name`: リクエストのスキル、プラグイン、エージェント、および MCP 属性。定義と削除動作については [コストカウンター](#cost-counter) を参照。

<h4 id="code-edit-tool-decision-counter">
  コード編集ツール決定カウンター
</h4>

ユーザーが Edit、Write、または NotebookEdit ツール使用を受け入れるか拒否するとインクリメントされます。

**属性**:

* すべての [標準属性](#standard-attributes)
* `tool_name`: ツール名（`"Edit"`、`"Write"`、`"NotebookEdit"`）
* `decision`: ユーザーの決定（`"accept"`、`"reject"`）
* `source`: 決定の出所。`"config"`、`"hook"`、`"user_permanent"`、`"user_temporary"`、`"user_abort"`、または `"user_reject"` のいずれか。各値の意味については [ツール決定イベント](#tool-decision-event) を参照。
* `language`: 編集されたファイルのプログラミング言語。`"TypeScript"`、`"Python"`、`"JavaScript"`、`"Markdown"` など。認識されないファイル拡張子の場合は `"unknown"` を返します。

<h4 id="active-time-counter">
  アクティブ時間カウンター
</h4>

Claude Code の実際の使用時間を追跡し、アイドル時間を除外します。このメトリクスは、入力や応答の読み取りなどのユーザーインタラクション中、およびツール実行や AI 応答生成などの CLI 処理中にインクリメントされます。

**属性**:

* すべての [標準属性](#standard-attributes)
* `type`: キーボードインタラクションの場合は `"user"`、ツール実行と AI 応答の場合は `"cli"`

<h3 id="events">
  イベント
</h3>

Claude Code は OpenTelemetry ログ/イベント経由で以下のイベントをエクスポートします（`OTEL_LOGS_EXPORTER` が設定されている場合）。

<h4 id="event-correlation-attributes">
  イベント相関属性
</h4>

ユーザーがプロンプトを送信すると、Claude Code は複数の API 呼び出しを行い、いくつかのツールを実行する可能性があります。`prompt.id` 属性を使用すると、これらすべてのイベントを、それらをトリガーした単一のプロンプトに結び付けることができます。

| 属性                  | 説明                                                                                                                                                                                                                                                                                                                                                               |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt.id`         | 単一のユーザープロンプト処理中に生成されたすべてのイベントをリンクする UUID v4 識別子                                                                                                                                                                                                                                                                                                                  |
| `event.sequence`    | イベントを順序付けするための 0 ベースのカウンター。セッションごとではなく Claude Code プロセスごとにカウント                                                                                                                                                                                                                                                                                                   |
| `message.uuid`      | セッショントランスクリプト（`~/.claude/projects/*/*.jsonl` ファイル）に保持されるメッセージの UUID。`assistant_response`、`api_response_body` に存在し、コマンドディスパッチを除く `user_prompt` に存在します。コマンドディスパッチはゼロまたは多くのメッセージを生成できます。`assistant_response` および `api_response_body` では、これは応答の最終トランスクリプトエントリであり、次のターンの `parentUuid` がこれからチェーンされます。Claude Code v2.1.214 以降が必要。または `api_response_body` では v2.1.274 以降 |
| `client_request_id` | `x-client-request-id` リクエストヘッダーとして送信されるクライアント生成 UUID。ファーストパーティ API 接続の `api_request` および `api_error` に存在します。サードパーティプロバイダーバックエンドおよびリクエストが非ストリーミングフォールバック経由で再試行された場合は存在しません。リクエストをその応答とペアリングし、サーバー `request_id` を生成しなかったタイムアウトなどの障害に対して利用可能なままです。`llm_request` トレーススパンの同じ属性と一致します。Claude Code v2.1.214 以降が必要                                                       |

単一のプロンプトによってトリガーされたすべてのアクティビティをトレースするには、特定の `prompt.id` 値でイベントをフィルタリングします。これにより、user\_prompt イベント、すべての api\_request イベント、およびそのプロンプト処理中に発生したすべての tool\_result イベントが返されます。

`event.sequence` は Claude Code プロセスが開始されるたびに 0 から始まり、そのプロセスの生涯にわたってカウントアップされます。`/clear` を横切ってカウントを続けます。これは新しい `session.id` を割り当てます。[セッションをフォークせずに再開](/docs/ja/how-claude-code-works#resume-or-fork-sessions) する場合、セッションは `session.id` を保持しますが、それを再開したプロセスから `event.sequence` 値を取得するため、1 つのセッション内で、後のイベントが前のイベントより低い値を持つか、1 つを繰り返す可能性があります。セッションのイベントを順序付けするには、`event.timestamp` でソートし、タイムスタンプを共有するイベントを順序付けするために `event.sequence` を使用します。

メッセージレベルの再構成の場合、各イベントクラスはセッショントランスクリプトのフィールドと一致するキーを持ちます。トランスクリプトエントリ形式は [Claude Code に内部的](/docs/ja/sessions#where-transcripts-are-stored) であり、バージョン間で変わるため、これらのフィールドで結合するパイプラインはリリースで破損する可能性があります。結合を安定した契約ではなくバージョン固有として扱います。

* `message.uuid` on `user_prompt`、`assistant_response`、および `api_response_body`
* `request_id` on the API events、persisted as `requestId` on the transcript's assistant entries
* `tool_use_id` on `tool_result` and `tool_decision` events

<h4 id="user-prompt-event">
  ユーザープロンプトイベント
</h4>

ユーザーがプロンプトを送信するとログされます。

**イベント名**: `claude_code.user_prompt`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `prompt_length`: プロンプトの長さ
* `prompt`: プロンプトコンテンツ。デフォルトではリダクションされます。`OTEL_LOG_USER_PROMPTS=1` を設定して含めます
* `message.uuid`: 結果のユーザーメッセージの UUID。保持されたトランスクリプトエントリと一致します。コマンドディスパッチには存在しません。ゼロまたは多くのメッセージを生成できます。Claude Code v2.1.214 以降が必要
* `command_name`: プロンプトがコマンドを呼び出す場合のコマンド名。`compact` や `debug` などの組み込みおよびバンドルコマンド名はそのまま発行されます。`reset` などのエイリアスは、入力されたとおりに発行されます。正規名ではなく。カスタム、プラグイン、および MCP コマンド名は、`OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り `custom` または `mcp` に折りたたまれます
* `command_source`: コマンドが存在する場合のコマンドの出所。`builtin`、`custom`、または `mcp`。プラグイン提供コマンドは `custom` として報告されます

<h4 id="assistant-response-event">
  アシスタント応答イベント
</h4>

モデルからテキストコンテンツを返す各 API リクエスト後にログされます。応答のテキストブロックのみが含まれます。思考ブロックとツール使用ブロックは除外されます。Claude Code v2.1.193 以降が必要です。

**イベント名**: `claude_code.assistant_response`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `response_length`: 応答テキストの長さ（文字数）
* `response`: 応答テキスト。コンテンツ制限（デフォルト 60 KB）で切り詰められます。デフォルトでは `<REDACTED>` にリダクションされます。`OTEL_LOG_ASSISTANT_RESPONSES=1` を設定して含めます。`OTEL_LOG_ASSISTANT_RESPONSES` が設定されていない場合、`OTEL_LOG_USER_PROMPTS` が代わりに制御するため、プロンプトログが有効な場合は応答をリダクションされたままにするために `OTEL_LOG_ASSISTANT_RESPONSES=0` を設定します
* `model`: モデル識別子（例：「claude-sonnet-5」）
* `request_id`: 応答の `request-id` ヘッダーからの Anthropic API リクエスト ID。API が返す場合のみ存在
* `message.uuid`: 応答の最終トランスクリプトエントリの UUID。API 応答はコンテンツブロックごとに 1 つのトランスクリプトエントリとして保持されます。これは最後のもので、次のターンの `parentUuid` がこれからチェーンされます。Claude Code v2.1.214 以降が必要
* `query_source`: リクエストを発行したサブシステム。`"repl_main_thread"`、`"compact"`、またはサブエージェント名など

<h4 id="tool-result-event">
  ツール結果イベント
</h4>

ツールが実行を完了するとログされます。ツール呼び出しが拒否された場合は発行されません。[ツール決定イベント](#tool-decision-event) で拒否を参照してください。

**イベント名**: `claude_code.tool_result`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `tool_name`: ツールの名前
* `tool_use_id`: このツール呼び出しの一意の識別子。フックに渡される `tool_use_id` と一致し、OTel イベントとフック取得データ間の相関を可能にします。
* `success`: `"true"` または `"false"`
* `duration_ms`: 実行時間（ミリ秒）
* `error_type`: ツールが失敗した場合のエラーカテゴリ文字列。`"Error:ENOENT"` または `"ShellError"` など
* `error`（`OTEL_LOG_TOOL_DETAILS=1` の場合）: ツールが失敗した場合の完全なエラーメッセージ
* `decision_type`: 常に `"accept"`。このイベントはツール実行後にのみ発行されるため。拒否された呼び出しはツール結果を生成しません
* `decision_source`: 権限決定の出所。`"config"`、`"hook"`、`"user_permanent"`、または `"user_temporary"` のいずれか。各値の意味については [ツール決定イベント](#tool-decision-event) を参照。拒否のみのソース `"user_abort"` および `"user_reject"` はこのイベントに表示されません。
* `tool_input_size_bytes`: JSON シリアル化されたツール入力のサイズ（バイト）
* `tool_result_size_bytes`: ツール結果のサイズ（バイト）
* `mcp_server_scope`: MCP サーバースコープ識別子（MCP ツール用）
* `vcs.ref.head.revision`、`vcs.ref.head.name`、`vcs.ref.head.type`（`OTEL_LOG_TOOL_DETAILS=1` の場合）: Bash または PowerShell ツールによって実行された成功した `git commit` のコミットアイデンティティ。`vcs.ref.head.revision` はコミット SHA、`vcs.ref.head.name` はコミットされたブランチ、`vcs.ref.head.type` は `branch`。コミットが detached HEAD で行われた場合、名前とタイプは省略されます。Claude Code v2.1.269 以降が必要
* `tool_parameters`（`OTEL_LOG_TOOL_DETAILS=1` の場合）: ツール固有のパラメーターを含む JSON 文字列。Claude Desktop の組み込みサーバーの場合、Claude Desktop が所有するセッションでは、フラグがオフの場合でも `mcp_server_name`/`mcp_tool_name` ペアが含まれます。[ツール決定イベント](#tool-decision-event) と同じホスト作成例外。Claude Code v2.1.214 以降が必要。パラメーターはツールによって異なります。
  * Bash ツール用: `bash_command`、`full_command`、`timeout`、`description`、`dangerouslyDisableSandbox` を含みます。`git commit` コマンドが成功した場合は `git_commit_id` と `git_branch` も含みます。`git_commit_id` はコミットがセッションの作業ディレクトリの HEAD である場合は完全なコミット SHA、それ以外の場合は git の短縮 SHA です。`git_branch` はコミットされたブランチ。detached HEAD では省略
  * デスクトップアプリのワークスペース Bash ツール（`tool_name` も `Bash` として報告）用: `bash_command`、`full_command`、`timeout` のみを含みます
  * MCP ツール用: `mcp_server_name`、`mcp_tool_name` を含みます
  * Skill ツール用: `skill_name` を含みます
  * Agent ツールまたはレガシー Task ツール用: `subagent_type` を含みます
* `tool_input`（`OTEL_LOG_TOOL_DETAILS=1` の場合）: JSON シリアル化されたツール引数。512 文字を超える個別の値は切り詰められ、完全なペイロードは約 4 K 文字に制限されます。MCP ツールを含むすべてのツールに適用されます。

<h4 id="api-request-event">
  API リクエストイベント
</h4>

Claude への各 API リクエストに対してログされます。

**イベント名**: `claude_code.api_request`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `model`: 使用されたモデル（例：「claude-sonnet-5」）
* `cost_usd`: 推定コスト（USD）
* `cost_usd_micros`: 推定コスト（米ドルの百万分の一）。整数として発行
* `duration_ms`: リクエスト期間（ミリ秒）
* `input_tokens`: 入力トークン数
* `output_tokens`: 出力トークン数
* `cache_read_tokens`: キャッシュから読み取られたトークン数
* `cache_creation_tokens`: キャッシュ作成に使用されたトークン数
* `request_id`: 応答の `request-id` ヘッダーからの Anthropic API リクエスト ID。`"req_011..."` など。API が返す場合のみ存在。
* `client_request_id`: `x-client-request-id` リクエストヘッダーとして送信されるクライアント生成 UUID。存在する場合については [イベント相関属性](#event-correlation-attributes) テーブルを参照。Claude Code v2.1.214 以降が必要
* `speed`: 高速モードがアクティブであったかどうかを示す `"fast"` または `"normal"`
* `query_source`: リクエストを発行したサブシステム。`"repl_main_thread"`、`"compact"`、またはサブエージェント名など
* `effort`: リクエストに適用された [努力レベル](/docs/ja/model-config#adjust-effort-level)。`"low"`、`"medium"`、`"high"`、`"xhigh"`、または `"max"`。モデルが努力をサポートしていない場合は存在しません。
* `agent.name`、`skill.name`、`plugin.name`、`marketplace.name`、`mcp_server.name`、`mcp_tool.name`: リクエストのスキル、プラグイン、エージェント、および MCP 属性。定義と削除動作については [コストカウンター](#cost-counter) を参照。

<h4 id="api-error-event">
  API エラーイベント
</h4>

Claude への API リクエストが失敗するとログされます。

**イベント名**: `claude_code.api_error`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `model`: 使用されたモデル（例：「claude-sonnet-5」）
* `error`: エラーメッセージ
* `status_code`: HTTP ステータスコード（数値）。接続障害などの非 HTTP エラーの場合は存在しません。
* `duration_ms`: リクエスト期間（ミリ秒）
* `attempt`: 実行された試行の総数。初期リクエストを含む（`1` は再試行が発生しなかったことを意味します）
* `request_id`: 応答の `request-id` ヘッダーからの Anthropic API リクエスト ID。`"req_011..."` など。API が返す場合のみ存在。
* `client_request_id`: `x-client-request-id` リクエストヘッダーとして送信されるクライアント生成 UUID。タイムアウトや接続エラーなどの障害がサーバー `request_id` を生成しなかった場合でも利用可能です。存在する場合については [イベント相関属性](#event-correlation-attributes) テーブルを参照。Claude Code v2.1.214 以降が必要
* `speed`: 高速モードがアクティブであったかどうかを示す `"fast"` または `"normal"`
* `query_source`: リクエストを発行したサブシステム。`"repl_main_thread"`、`"compact"`、またはサブエージェント名など
* `effort`: リクエストに適用された [努力レベル](/docs/ja/model-config#adjust-effort-level)。モデルが努力をサポートしていない場合は存在しません。
* `agent.name`、`skill.name`、`plugin.name`、`marketplace.name`、`mcp_server.name`、`mcp_tool.name`: リクエストのスキル、プラグイン、エージェント、および MCP 属性。定義と削除動作については [コストカウンター](#cost-counter) を参照。

<h4 id="api-refusal-event">
  API 拒否イベント
</h4>

API リクエストが `stop_reason: "refusal"` を返すとログされます。拒否は HTTP エラーではなく成功した応答ストリームで到着するため、`api_error` イベントは発火しません。このイベントにより、拒否頻度を追跡し、拒否を `api_request` および `api_error` と同じ属性でグループ化できます。

**イベント名**: `claude_code.api_refusal`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `model`: リクエストからのモデル識別子
* `request_id`: 応答の `request-id` ヘッダーからの Anthropic API リクエスト ID。`"req_011..."` など。API が返す場合のみ存在。
* `query_source`: リクエストを発行したサブシステム。`"repl_main_thread"`、`"compact"`、またはサブエージェント名など。定義については [`api_request`](#api-request-event) を参照。
* `speed`: [高速モード](/docs/ja/fast-mode) がアクティブな場合は `"fast"`、またはそれ以外の場合は `"normal"`
* `attempt`: 再試行試行番号。最初の試行は `1`。
* `effort`: リクエストに適用された [努力レベル](/docs/ja/model-config#adjust-effort-level)。モデルが努力をサポートしていない場合は存在しません。
* `server_fallback_hop`: API のサーバー側モデルフォールバックがこの拒否を別のモデルで既に再試行した場合は `true`。ユーザーはこの特定の拒否を見ませんでした。リクエストが拒否で終了した場合は `false`。単一のターンは、フォールバックモデルも拒否する場合、`true` ホップイベントと後の `false` 最終イベントの両方を発行できます。
* `has_category`: API 応答が `stop_details.category` の `"cyber"`、`"bio"`、`"frontier_llm"`、または `"reasoning_extraction"` を持つ場合は `true`。応答がカテゴリを持たないか、そのセット外の値を持つ場合は `false`。`server_fallback_hop` が `true` の場合は存在しません。ホップブロックは `stop_details` を持たないため。
* `has_explanation`: API 応答が `stop_details.explanation` を持つ場合は `true`。それ以外の場合は `false`。`server_fallback_hop` が `true` の場合は存在しません。
* `category`: API 応答からの `stop_details.category` 値。`"cyber"`、`"bio"`、`"frontier_llm"`、または `"reasoning_extraction"` のいずれか。`OTEL_LOG_TOOL_DETAILS=1` が設定され、`has_category` が `true` の場合のみ存在。
* `agent.name`、`skill.name`、`plugin.name`、`marketplace.name`、`mcp_server.name`、`mcp_tool.name`: リクエストのスキル、プラグイン、エージェント、および MCP 属性。定義と削除動作については [コストカウンター](#cost-counter) を参照。

<h4 id="api-request-body-event">
  API リクエストボディイベント
</h4>

`OTEL_LOG_RAW_API_BODIES` が設定されている場合、各 API リクエスト試行に対してログされます。試行ごとに 1 つのイベントが発行されるため、調整されたパラメーターでの再試行はそれぞれ独自のイベントを生成します。

**イベント名**: `claude_code.api_request_body`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `body`: JSON シリアル化された Messages API リクエストパラメーター。システムプロンプト、メッセージ、ツールなど。コンテンツ制限（デフォルト 60 KB）で切り詰められます。前のアシスタントターンの拡張思考コンテンツはリダクションされます。インラインモード（`OTEL_LOG_RAW_API_BODIES=1`）でのみ発行。
* `body_ref`: 切り詰められていないボディを含む `<dir>/<uuid>.request.json` ファイルへの絶対パス。ファイルモード（`OTEL_LOG_RAW_API_BODIES=file:<dir>`）でのみ発行。
* `body_length`: 切り詰められていないボディの長さ。`OTEL_LOG_RAW_API_BODIES=file:<dir>` の場合は UTF-8 バイト。`=1` の場合は UTF-16 コードユニット
* `body_truncated`: インラインの切り詰めが発生した場合は `"true"`。ファイルモードおよび切り詰めが発生しなかった場合は存在しません。
* `model`: リクエストパラメーターからのモデル識別子
* `query_source`: リクエストを発行したサブシステム（例：`"compact"`）
* `request_body_id`: この試行のリクエストボディを識別する UUID。成功した試行の [`api_response_body` イベント](#api-response-body-event) は同じ値を持つため、応答をそれを生成した正確なリクエストとペアリングできます。Claude Code v2.1.274 以降が必要

<h4 id="api-response-body-event">
  API レスポンスボディイベント
</h4>

`OTEL_LOG_RAW_API_BODIES` が設定されている場合、各成功した API レスポンスに対してログされます。

ファイルモード（`OTEL_LOG_RAW_API_BODIES=file:<dir>`）では、Claude Code は成功した応答ごとに 1 つの JSON 行を `<dir>/index.jsonl` に追加します。フィールド `timestamp`、`session_id`、`query_source`、`model`、`request_id`、`message_id`、`message_uuid`、`request_file`、`response_file` を持ちます。これを読んで、テレメトリバックエンドをクエリせずに、特定のトランスクリプトメッセージの背後にあるリクエストおよびレスポンスファイルを見つけます。インデックスファイルは Claude Code v2.1.274 以降が必要です。

**イベント名**: `claude_code.api_response_body`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `body`: JSON シリアル化された Messages API レスポンス。ID、コンテンツブロック、使用状況、停止理由を含みます。コンテンツ制限（デフォルト 60 KB）で切り詰められます。拡張思考コンテンツはリダクションされます。インラインモード（`OTEL_LOG_RAW_API_BODIES=1`）でのみ発行。
* `body_ref`: 切り詰められていないボディを含む `<dir>/<request_id>.response.json` ファイルへの絶対パス。ファイルモード（`OTEL_LOG_RAW_API_BODIES=file:<dir>`）でのみ発行。
* `body_length`: 切り詰められていないボディの長さ。`OTEL_LOG_RAW_API_BODIES=file:<dir>` の場合は UTF-8 バイト。`=1` の場合は UTF-16 コードユニット
* `body_truncated`: インラインの切り詰めが発生した場合は `"true"`。ファイルモードおよび切り詰めが発生しなかった場合は存在しません。
* `model`: モデル識別子
* `query_source`: リクエストを発行したサブシステム
* `request_id`: 応答の `request-id` ヘッダーからの Anthropic API リクエスト ID。`"req_011..."` など。API が返す場合のみ存在。
* `request_body_id`: この応答が答える [`api_request_body` イベント](#api-request-body-event) の `request_body_id`。Claude Code v2.1.274 以降が必要
* `message.id`: API がレスポンスに割り当てたメッセージ ID。レスポンスボディの `id` フィールド。Claude Code v2.1.274 以降が必要
* `message.uuid`: レスポンスの最終トランスクリプトエントリの UUID。`request_body_id` と一緒に、トランスクリプトメッセージをそれの背後にあるリクエストおよびレスポンスボディにリンクします。Claude Code v2.1.274 以降が必要

<h4 id="tool-decision-event">
  ツール決定イベント
</h4>

ツール権限決定が行われるとログされます（受け入れ/拒否）。

**イベント名**: `claude_code.tool_decision`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `tool_name`: ツールの名前（例：「Read」、「Edit」、「Write」、「NotebookEdit」）
* `tool_use_id`: このツール呼び出しの一意の識別子。フックに渡される `tool_use_id` と一致し、OTel イベントとフック取得データ間の相関を可能にします。
* `decision`: `"accept"` または `"reject"`
* `tool_source`: 常に存在します。ツールの出所。CLI 作成値の閉じたセット。Claude Code v2.1.214 以降が必要
  * `"builtin"`: CLI 自体のツール
  * `"mcp"`: 一般的に MCP サーバー
  * `"sdk_host_builtin_mcp"`: Claude Desktop 自体に組み込まれたプロセス内サーバー。Claude Desktop が所有するセッション。Claude Desktop が独自のエントリポイント `claude-desktop`、`claude-desktop-3p`、または `local-agent` から開始したセッション。そのセッションがネストされた子ではない場合。ネストされたセッション（Claude Code 自体が生成するセッションを含む）は、これらのサーバーを `"mcp"` として報告します
* `source`: 決定の出所:
  * `"config"`: プロンプトなしで自動的に決定。プロジェクト設定、ユーザーの個人設定の許可または拒否ルール、エンタープライズ管理ポリシー、`--allowedTools` または `--disallowedTools` フラグ、アクティブな権限モード、同じインタラクティブ CLI セッション内の前のプロンプトからのセッションスコープ付与、またはツールが本質的に安全であるため。イベントはこれらのソースのどれが一致したかを示しません。Claude Code は、権限プロンプトリクエスト自体が失敗した場合も `"config"` を報告します。例えば、Agent SDK の [`canUseTool`](/docs/ja/agent-sdk/typescript#canusetool) コールバックまたは [`--permission-prompt-tool`](/docs/ja/cli-reference#cli-flags) ツールが無効な結果を返す場合、またはリクエストが保留中に入力ストリームが閉じる場合。v2.1.216 より前では、Claude Code はこれらの障害を `"user_reject"` として報告していました。
  * `"hook"`: `PreToolUse` または `PermissionRequest` フックが決定を返しました。
  * `"user_permanent"`: ユーザーが権限プロンプトで「はい、今後このツールについて聞かないでください」を選択した場合に発行されます。これにより、許可ルールが個人設定に保存されます。インタラクティブ CLI では、その選択自体に対してのみ発行されます。後の呼び出しが保存されたルールと一致する場合は、代わりに `"config"` を発行します。Agent SDK または非インタラクティブ `-p` セッションでは、初期選択と後のルール一致の両方が `"user_permanent"` を発行します。受け入れとして扱われます。
  * `"user_temporary"`: ユーザーが権限プロンプトで「はい」を選択した場合、またはファイル編集または読み取りプロンプトでセッションの残りの間アクセスを許可するオプションを選択した場合に発行されます。インタラクティブ CLI では、その選択自体に対してのみ発行されます。後の呼び出しがそのセッションスコープ付与によって許可される場合は、代わりに `"config"` を発行します。Agent SDK または非インタラクティブ `-p` セッションでは、選択と後の一致の両方が `"user_temporary"` を発行します。受け入れとして扱われます。
  * `"user_abort"`: ユーザーが権限プロンプトを回答なしで却下した場合に発行されます。Agent SDK および非インタラクティブ `-p` セッションでは、`canUseTool` または `--permission-prompt-tool` 権限リクエストが保留中にターンを中断することを含みます。v2.1.216 より前では、Claude Code はその中断を `"user_reject"` として報告していました。拒否として扱われます。
  * `"user_reject"`: ユーザーがプロンプトで「いいえ」を選択した場合に発行されます。インタラクティブ CLI では、その選択自体に対してのみ発行されます。ユーザーの個人設定の拒否ルールと一致する呼び出しは、代わりに `"config"` を発行します。Agent SDK または非インタラクティブ `-p` セッションでは、個人設定の拒否ルールと一致する呼び出しは `"user_reject"` を発行します。拒否として扱われます。
* `tool_parameters`（`OTEL_LOG_TOOL_DETAILS=1` の場合）: ツール固有のパラメーターを含む JSON 文字列。[ツール結果イベント](#tool-result-event) と同じ形状。`updatedInput` を介した権限決定がツール入力を書き直す場合、受け入れられた呼び出しの値は異なる可能性があります。`decision` が `"reject"` の場合、どのコマンドが拒否されたかを確認するには、この属性を使用します。
  * `"sdk_host_builtin_mcp"` ツール用: `OTEL_LOG_TOOL_DETAILS` がオフの場合でも、ホストアプリケーションがこれらの名前を定義するため、`mcp_server_name` と `mcp_tool_name` が含まれます。これらなしでは、これらの組み込みサーバーのいずれかへの拒否された呼び出しはデフォルトストリームで属性付けできません。ユーザー設定 MCP サーバーの場合、イベントの `tool_name` は常にリテラル `"mcp_tool"`。サーバーとツール名は `tool_parameters` にのみ表示されます。フラグがオンの場合。引数コンテンツはどこでもフラグが必要です。Claude Code v2.1.214 以降が必要
  * Bash ツール用: `bash_command`、`full_command`、`timeout`、`description`、`dangerouslyDisableSandbox` を含みます。デスクトップアプリのワークスペース bash ツール（`tool_name` も `Bash` として報告）は、`bash_command`、`full_command`、`timeout` のみを含みます
  * MCP ツール用: `mcp_server_name`、`mcp_tool_name` を含みます
  * Skill ツール用: `skill_name` を含みます
  * Agent ツールまたはレガシー Task ツール用: `subagent_type` を含みます

<h4 id="permission-mode-changed-event">
  権限モード変更イベント
</h4>

権限モードが変更されるとログされます。例えば、`Shift+Tab` サイクリング、プランモード終了、または自動モードゲートチェックから。

**イベント名**: `claude_code.permission_mode_changed`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `from_mode`: 前の権限モード。例：`"default"`、`"plan"`、`"acceptEdits"`、`"auto"`、または `"bypassPermissions"`
* `to_mode`: 新しい権限モード
* `trigger`: 変更の原因。`"shift_tab"`、`"exit_plan_mode"`、`"auto_gate_denied"`、または `"auto_opt_in"` のいずれか。SDK またはブリッジから発生する遷移の場合は存在しません

<h4 id="auth-event">
  認証イベント
</h4>

`/login` または `/logout` が完了するとログされます。

**イベント名**: `claude_code.auth`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `action`: `"login"` または `"logout"`
* `success`: `"true"` または `"false"`
* `auth_method`: 認証方法。`"oauth"` など
* `error_category`: アクションが失敗した場合のカテゴリエラー種別。生のエラーメッセージは決して含まれません
* `status_code`: アクションが HTTP エラーで失敗した場合の HTTP ステータスコード（文字列）

<h4 id="mcp-server-connection-event">
  MCP サーバー接続イベント
</h4>

MCP サーバーが接続、切断、または接続に失敗するとログされます。

**イベント名**: `claude_code.mcp_server_connection`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `status`: `"connected"`、`"failed"`、または `"disconnected"`
* `transport_type`: サーバートランスポート。`"stdio"`、`"sse"`、`"http"` など
* `server_scope`: サーバーが設定されているスコープ。`"user"`、`"project"`、`"local"` など
* `duration_ms`: 接続試行期間（ミリ秒）
* `error_code`: 接続が失敗した場合のエラーコード
* `is_plugin`: サーバーがプラグインによって提供される場合は `true`。それ以外の場合は `false`
* `plugin_id_hash`（`is_plugin` が `true` の場合）: プラグイン名とマーケットプレイスの安定ハッシュ。名前を公開せずにプラグインでイベントをグループ化します。Claude Code は [プラグイン読み込みイベント](#plugin-loaded-event) で説明されているように計算します
* `plugin.name`（`is_plugin` が `true` の場合）: サーバーを提供するプラグインの名前。サードパーティプラグインの場合、この値は `OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り、リテラル文字列 `"third-party"`。公式 Anthropic ソースのプラグインは常に名前で識別されます。`plugin_id_hash` と `plugin.name` 属性は独自の監視バックエンドにフローし、Anthropic に送信されません
* `server_name`（`OTEL_LOG_TOOL_DETAILS=1` の場合）: 設定されたサーバー名
* `error`（`OTEL_LOG_TOOL_DETAILS=1` の場合）: 接続が失敗した場合の完全なエラーメッセージ

<h4 id="internal-error-event">
  内部エラーイベント
</h4>

Claude Code が予期しない内部エラーをキャッチするとログされます。エラークラス名と errno スタイルコードのみが記録されます。エラーメッセージとスタックトレースは決して含まれません。このイベントは Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry に対して実行する場合、または `DISABLE_ERROR_REPORTING` が設定されている場合は発行されません。

**イベント名**: `claude_code.internal_error`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `error_name`: エラークラス名。`"TypeError"` または `"SyntaxError"` など
* `error_code`: エラーに存在する場合の Node.js errno コード。`"ENOENT"` など

<h4 id="plugin-installed-event">
  プラグインインストール済みイベント
</h4>

プラグインがインストール完了するとログされます。`claude plugin install` CLI コマンドとインタラクティブ `/plugin` UI の両方から。

**イベント名**: `claude_code.plugin_installed`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `marketplace.is_official`: マーケットプレイスが公式 Anthropic マーケットプレイスの場合は `"true"`。それ以外の場合は `"false"`
* `install.trigger`: `"cli"` または `"ui"`
* `plugin.name`: インストールされたプラグインの名前。サードパーティマーケットプレイスの場合、`OTEL_LOG_TOOL_DETAILS=1` が設定されている場合のみ含まれます
* `plugin.version`: マーケットプレイスエントリで宣言されている場合のプラグインバージョン。サードパーティマーケットプレイスの場合、`OTEL_LOG_TOOL_DETAILS=1` が設定されている場合のみ含まれます
* `marketplace.name`: プラグインがインストールされたマーケットプレイス。サードパーティマーケットプレイスの場合、`OTEL_LOG_TOOL_DETAILS=1` が設定されている場合のみ含まれます

<h4 id="plugin-loaded-event">
  プラグイン読み込みイベント
</h4>

セッション開始時に有効なプラグインごとに 1 回ログされます。このイベントを使用して、フロート全体でアクティブなプラグインをインベントリします。`plugin_installed` はインストールアクション自体を記録するため、補完として。

**イベント名**: `claude_code.plugin_loaded`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `plugin.name`: プラグインの名前。公式マーケットプレイスと組み込みバンドルの外のプラグインの場合、値は `OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り `"third-party"`
* `marketplace.name`: プラグインがインストールされたマーケットプレイス（既知の場合）。`plugin.name` と同じ条件で `"third-party"` にリダクションされます
* `plugin.version`: プラグインマニフェストからのバージョン。名前がリダクションされず、マニフェストがバージョンを宣言する場合のみ含まれます
* `plugin.scope`: プラグインの出所カテゴリ。`"official"`、`"community"`、`"org"`、`"user-local"`、または `"default-bundle"`
* `enabled_via`: プラグインが有効になった方法。`"default-enable"`、`"org-policy"`、`"admin-install"`、`"seed-mount"`、または `"user-install"`。`"admin-install"` 値は、プラグインが [**組織設定 > プラグイン**](https://claude.ai/admin-settings/plugins) で組織に対して必須またはオートインストールに設定されていることを意味します。v2.1.246 より前では、Claude Code はこれらのプラグインを `"user-install"` または `"seed-mount"` として報告していました
* `plugin_id_hash`: プラグイン名とマーケットプレイスの決定論的ハッシュ。設定されたエクスポーターにのみ送信されます。フロート全体でロードされた異なるサードパーティプラグインをカウントできます。名前を記録せずに。[claude.ai から同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins) の場合、Claude Code はプラグイン名を claude.ai が報告するマーケットプレイス名、またはそれ以外の場合は `synced` でハッシュします。v2.1.246 より前では、Claude Code はハッシュで claude.ai が報告するマーケットプレイス名を使用していませんでした
* `has_hooks`: プラグインがフックに貢献するかどうか
* `has_mcp`: プラグインが MCP サーバーに貢献するかどうか
* `host_owned_mcp`: SDK ホストがこのプラグインの MCP 接続を管理し、Claude Code がプラグインの MCP サーバー設定の読み取りをスキップした場合は `true`。それ以外の場合は `false`。Claude Code v2.1.172 以降が必要
* `skill_path_count`: プラグインが宣言するスキルディレクトリの数
* `command_path_count`: プラグインが宣言するコマンドディレクトリの数
* `agent_path_count`: プラグインが宣言するエージェントディレクトリの数
* `safe_mode`: セッションが [`--safe-mode`](/docs/ja/cli-reference) で開始された場合は `"true"`。それ以外の場合は `"false"`。セーフモードでは、このイベントは設定されたインベントリのみを報告します。プラグインのコマンド、スキル、フック、MCP サーバーは読み込まれません。Claude Code v2.1.169 以降が必要

<h4 id="skill-activated-event">
  スキル有効化イベント
</h4>

スキルが呼び出されるとログされます。Claude が Skill ツールを通じて呼び出すか、`/` コマンドとして実行するかどうか。

**イベント名**: `claude_code.skill_activated`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `skill.name`: スキルの名前。ユーザー定義およびサードパーティプラグインスキルの場合、値は `OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り、プレースホルダー `"custom_skill"`
* `invocation_trigger`: スキルがトリガーされた方法（`"user-slash"`、`"claude-proactive"`、または `"nested-skill"`）
* `skill.source`: スキルが読み込まれた場所（例：`"bundled"`、`"userSettings"`、`"projectSettings"`、`"plugin"`）
* `skill.kind`: スキルがワークフロースキルの場合は `"workflow"`。それ以外の場合は存在しません
* `plugin.name`（`OTEL_LOG_TOOL_DETAILS=1` の場合、またはプラグインが公式マーケットプレイスからの場合）: スキルがプラグインによって提供される場合の所有プラグインの名前
* `marketplace.name`（`OTEL_LOG_TOOL_DETAILS=1` の場合、またはプラグインが公式マーケットプレイスからの場合）: スキルがプラグインによって提供される場合、所有プラグインがインストールされたマーケットプレイス

<h4 id="at-mention-event">
  @ メンションイベント
</h4>

Claude Code がプロンプト内の `@` メンションを解決するとログされます。すべてのメンションがイベントを発行するわけではありません。権限拒否、サイズ超過ファイル、PDF 参照添付、ディレクトリリスト障害などの早期終了パスはログなしで返されます。

**イベント名**: `claude_code.at_mention`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `mention_type`: メンションのタイプ（`"file"`、`"directory"`、`"agent"`、`"mcp_resource"`、`"peer"`）。`"peer"` 値は、[他の Claude Code セッション](/docs/ja/cross-session-messaging) のいずれかをメンションしたことを意味します。Claude Code v2.1.232 以降が必要
* `success`: メンションが正常に解決されたかどうか（`"true"` または `"false"`）

<h4 id="api-retries-exhausted-event">
  API 再試行枯渇イベント
</h4>

API リクエストが複数の試行後に失敗した場合、1 回ログされます。最終 `api_error` イベントと一緒に発行されます。

**イベント名**: `claude_code.api_retries_exhausted`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `model`: 使用されたモデル
* `error`: 最終エラーメッセージ
* `status_code`: HTTP ステータスコード（数値）。非 HTTP エラーの場合は存在しません。
* `total_attempts`: 実行された試行の総数
* `total_retry_duration_ms`: すべての試行にわたる総ウォールクロック時間
* `speed`: `"fast"` または `"normal"`

<h4 id="hook-registered-event">
  フック登録イベント
</h4>

セッション開始時に設定されたフックごとに 1 回ログされます。フロート全体でアクティブなフックをインベントリするには、このイベントを使用します。実行ごとの `hook_execution_start` および `hook_execution_complete` イベントの補完として。

**イベント名**: `claude_code.hook_registered`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `hook_event`: フックイベントタイプ。`"PreToolUse"` または `"PostToolUse"` など
* `hook_type`: フック実装タイプ。`"command"`、`"prompt"`、`"mcp_tool"`、`"http"`、または `"agent"`
* `hook_source`: フックが定義されている場所。`"userSettings"`、`"projectSettings"`、`"localSettings"`、`"flagSettings"`、`"policySettings"`、または `"pluginHook"`
* `safe_mode`: セッションが [`--safe-mode`](/docs/ja/cli-reference) で開始された場合は `"true"`。それ以外の場合は `"false"`。Claude Code v2.1.169 以降が必要
* `hook_matcher`（`OTEL_LOG_TOOL_DETAILS=1` の場合）: フック設定が設定されている場合のマッチャー文字列
* `plugin.name`（`hook_source` が `"pluginHook"` の場合）: 貢献するプラグインの名前。公式マーケットプレイスと組み込みバンドルの外のプラグインの場合、値は `OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り `"third-party"`
* `plugin_id_hash`（`hook_source` が `"pluginHook"` の場合）: プラグイン名とマーケットプレイスの決定論的ハッシュ。設定されたエクスポーターにのみ送信されます。名前を記録せずに、異なる貢献プラグインをカウントできます。Claude Code は [プラグイン読み込みイベント](#plugin-loaded-event) で説明されているように計算します

<h4 id="hook-execution-start-event">
  フック実行開始イベント
</h4>

1 つ以上のフックがフックイベントの実行を開始するとログされます。

**イベント名**: `claude_code.hook_execution_start`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `hook_event`: フックイベントタイプ。`"PreToolUse"` または `"PostToolUse"` など
* `hook_name`: マッチャーを含む完全なフック名。`"PreToolUse:Write"` など
* `num_hooks`: マッチするフックコマンドの数
* `managed_only`: 管理ポリシーフックのみが許可される場合は `"true"`
* `hook_source`: `"policySettings"` または `"merged"`
* `safe_mode`: セッションが [`--safe-mode`](/docs/ja/cli-reference) で開始された場合は `"true"`。それ以外の場合は `"false"`。Claude Code v2.1.169 以降が必要
* `hook_definitions`: JSON シリアル化されたフック設定。詳細ベータトレースと `OTEL_LOG_TOOL_DETAILS=1` の両方が有効な場合のみ含まれます

<h4 id="hook-execution-complete-event">
  フック実行完了イベント
</h4>

フックイベントのすべてのフックが完了するとログされます。

**イベント名**: `claude_code.hook_execution_complete`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `hook_event`: フックイベントタイプ
* `hook_name`: マッチャーを含む完全なフック名
* `num_hooks`: マッチするフックコマンドの数
* `num_success`: 正常に完了したカウント
* `num_blocking`: ブロッキング決定を返したカウント
* `num_non_blocking_error`: ブロッキングなしで失敗したカウント
* `num_cancelled`: 完了前にキャンセルされたカウント
* `total_duration_ms`: すべてのマッチするフックのウォールクロック期間
* `stdout_chars`: 成功したマッチするフック全体の stdout の総文字数。Claude Code v2.1.280 以降が必要
* `additional_context_chars`: マッチするフックによって返された `additionalContext` の総文字数。Claude Code v2.1.280 以降が必要
* `system_message_chars`: マッチするフックによって返された `systemMessage` の総文字数。Claude Code v2.1.280 以降が必要
* `initial_user_message_chars`: マッチするフックによって返された `initialUserMessage` の総文字数。Claude Code v2.1.280 以降が必要
* `num_outputs_persisted`: [10,000 文字キャップ](/docs/ja/hooks#json-output) を超えたフック出力の数。Claude Code がファイルに保存。Claude Code v2.1.280 以降が必要
* `managed_only`: 管理ポリシーフックのみが許可される場合は `"true"`
* `hook_source`: `"policySettings"` または `"merged"`
* `safe_mode`: セッションが [`--safe-mode`](/docs/ja/cli-reference) で開始された場合は `"true"`。それ以外の場合は `"false"`。Claude Code v2.1.169 以降が必要
* `hook_definitions`: JSON シリアル化されたフック設定。詳細ベータトレースと `OTEL_LOG_TOOL_DETAILS=1` の両方が有効な場合のみ含まれます

<h4 id="hook-plugin-metrics-event">
  フックプラグインメトリクスイベント
</h4>

公式マーケットプレイスプラグインフックが呼び出しごとのメトリクスを発行するとログされます。公式 Anthropic マーケットプレイスからインストールされたプラグインのみがこれを発行できます。サードパーティマーケットプレイスプラグインとユーザー設定フックはこのイベントに発行しません。このイベントを使用して、独自の可観測性スタックからプラグイン動作（検出率、コスト、期間など）を監視します。

**イベント名**: `claude_code.hook_plugin_metrics`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `plugin_id`: `<name>@<marketplace>` 形式のプラグイン識別子
* `hook_event`: メトリクスを発行したフックイベントタイプ
* 最大 20 個のプラグイン発行メトリクスキー。名前は `^[a-z][a-z0-9_]{0,39}$` と一致します。値はブール値または数値。

<h4 id="compaction-event">
  圧縮イベント
</h4>

会話圧縮が完了するとログされます。

**イベント名**: `claude_code.compaction`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `trigger`: `"auto"` または `"manual"`
* `success`: `"true"` または `"false"`
* `duration_ms`: 圧縮期間
* `pre_tokens`: 圧縮前の概算トークンカウント
* `post_tokens`: 圧縮後の概算トークンカウント
* `error`: 圧縮が失敗した場合のエラーメッセージ
* `precompute_reuse`: `trigger` が `"manual"` の場合のみ設定。自動圧縮は、コンテキストウィンドウが満杯になる前にバックグラウンドで概要を準備でき、この属性は `/compact` がその準備された概要を再利用したかどうかを記録します。`"hit"` は再利用されたことを意味します。`"miss_custom_instructions"`、`"miss_hook"`、`"miss_not_ready"` は、代わりに新しい概要が計算された理由を示します。Claude Code v2.1.153 以降が必要

<h4 id="subagent-completed-event">
  サブエージェント完了イベント
</h4>

[サブエージェント](/docs/ja/sub-agents) が完了し、それを開始した会話に結果を返すとログされます。ツール使用と実行時をサブエージェントタイプでロールアップするために使用します。トークンまたはコストロールアップの場合、`query_source` を `"subagent"` にフィルタリングした [トークンカウンター](#token-counter) および [コストカウンター](#cost-counter) を使用します。このイベントの `total_tokens` は最終リクエストのみをカバーするため。`"subagent"` カテゴリはエージェントベースのフックからのリクエストもカウントします。サブエージェントイベントは発行しません。

**イベント名**: `claude_code.subagent_completed`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"subagent_completed"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `agent_type`: サブエージェントタイプ。組み込みエージェント名と公式マーケットプレイスプラグインのエージェントはそのまま表示されます。他のエージェント名は `OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り `"custom"` に置き換えられます
* `agent.source`: エージェント定義の出所。`built-in`、`plugin`、またはカスタムエージェントを定義した設定ソース（`userSettings` や `projectSettings` など）
* `is_built_in`: サブエージェントが組み込みエージェントタイプであるかどうか
* `is_async`: サブエージェントが [バックグラウンド](/docs/ja/sub-agents#run-subagents-in-foreground-or-background) で実行されたかどうか
* `total_tokens`: サブエージェントの最終 API リクエストのトークンフットプリント。その 1 つのリクエストの入力、キャッシュ作成、キャッシュ読み取り、出力トークン。大体、完了時のサブエージェントのコンテキストサイズ。実行全体のサムではありません
* `total_tool_uses`: サブエージェントが実行全体で行ったツール呼び出しの数
* `duration_ms`: 実行時間（ミリ秒）
* `model`: サブエージェントが実行するために解決されたモデル
* `final_model`: サブエージェントの最終応答を生成したモデル。フォールバックなどの実行中スイッチ後に `model` と異なります。Claude Code v2.1.212 以降が必要
* `model_swapped`: 複数のモデルがサブエージェントのリクエストを提供したかどうか。Claude Code v2.1.212 以降が必要
* `plugin_id_hash`、`plugin.name`: プラグイン提供エージェント用に存在。公式マーケットプレイスプラグイン名はそのまま表示されます。他のプラグイン名は `OTEL_LOG_TOOL_DETAILS=1` が設定されていない限り `"third-party"` に置き換えられます

<h4 id="feedback-survey-event">
  フィードバック調査イベント
</h4>

セッション品質調査が表示または回答されるとログされます。[セッション品質調査](/docs/ja/data-usage#session-quality-surveys) で調査が収集する内容と制御方法を参照してください。

**イベント名**: `claude_code.feedback_survey`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `event_type`: 調査ライフサイクルイベント。例：`"appeared"`、`"responded"`、`"transcript_prompt_appeared"`
* `appearance_id`: 1 つの調査インスタンスに対して発行されたイベントをリンクする一意の ID
* `survey_type`: イベントを生成した調査。`"session"` は「Claude はどのように機能していますか？」評価プロンプト
* `response`: `responded` イベントのユーザーの選択
* `enabled_via_override`: [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/ja/env-vars) が設定されている場合は `true`。文字列ではなくブール値として発行。`session` 調査イベントに存在。このオーバーライドがフロート全体で適用されていることを確認するには、この属性でフィルタリングします

<h4 id="retention-sweep-event">
  保持スイープイベント
</h4>

保持クリーンアップスイープの実行ごとに 1 回ログされます。[セッショントランスクリプトおよび他のアプリケーションデータ](/docs/ja/claude-directory#cleaned-up-automatically) を [`cleanupPeriodDays`](/docs/ja/settings-reference#cleanupperioddays) 設定より古い削除します。Claude Code はバックグラウンドでスイープを実行し、セッションごとに最大 1 回。何も削除しない実行でもイベントを発行します。Claude Code が同じマシン上の任意のセッションで過去 24 時間にスイープを実行した場合、このセッションのスイープを少なくとも 10 分遅延させるため、より早く終了するセッションは何も発行しません。`claude -p` を `--bare` で実行する場合、Claude Code はスイープを実行せず、何も発行しません。

このページのすべての OTel イベントと同様に、設定したテレメトリバックエンドにのみ移動します。Claude Code v2.1.227 以降が必要です。

Claude Code が保持期間を安全に決定できない場合、スイープを一時停止し、`result` を `"skipped"` に設定し、`skip_reason` を持つイベントを発行します。[管理設定](/docs/ja/server-managed-settings) が `cleanupPeriodDays` を設定する場合、管理値は保持期間をピンし、下位優先度スコープの設定ファイルが破損または無効な場合でもスイープが実行されます。`managed-settings.json` 自体が読み取れない場合、Claude Code は [管理層](/docs/ja/managed-settings#how-claude-code-combines-managed-sources) がサーバー管理設定などの他の場所から `cleanupPeriodDays` を提供しない限り、スイープを一時停止します。破損したファイルの横にある `managed-settings.d/` ドロップイン。削除カウンター属性は `result` が `"complete"` の場合のみ存在します。

**イベント名**: `claude_code.retention_sweep`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"retention_sweep"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `result`: スイープが実行された場合は `"complete"`。Claude Code がそれを一時停止した場合は `"skipped"`
* `period_days`: マージされた設定からの `cleanupPeriodDays` 値（日数）。またはソースが設定しない場合は `30`。スキップされたイベントでは、スイープが使用した値。Claude Code が読み取れた設定ソースから計算
* `used_default`: 読み取り可能な設定ソースが `cleanupPeriodDays` を設定しない場合は `"true"`。それ以外の場合は `"false"`。完了イベントでは、`"true"` は 30 日のデフォルトが適用されたことを意味します
* `skip_reason`: Claude Code がスイープを一時停止した理由。`result` が `"skipped"` の場合のみ存在:
  * `"user_source_disabled"`: ユーザー設定は除外されます。例えば、[`--setting-sources`](/docs/ja/cli-reference#cli-flags) フラグまたは SDK の [`settingSources`](/docs/ja/agent-sdk/typescript#options) オプションによって。有効なソースが `cleanupPeriodDays` を提供しません
  * `"settings_unknowable"`: 設定ファイルが読み取れないか解析できないため、`cleanupPeriodDays` または `desktopSessionCleanupPeriodDays` が Claude Code が見ることができない値に設定されている可能性があります
  * `"settings_invalid_key_set"`: 設定に検証エラーがあり、`cleanupPeriodDays` または `desktopSessionCleanupPeriodDays` が明示的に設定されているため、デフォルトにフォールバックするとその設定に対してファイルを削除または保持する可能性があります
* `transcripts_deleted`: セッショントランスクリプト。トップレベルの `~/.claude/projects/*/*.jsonl` ファイル。スイープが削除した数
* `transcripts_exempted_desktop`: 保持期間を過ぎたトランスクリプトの数。スイープが [Claude Desktop および Cowork ルール](/docs/ja/claude-directory#cleaned-up-automatically) の下で保持。これらは `files_past_cutoff` にカウントされません。Claude Code v2.1.248 以降が必要
* `session_files_deleted`: セッションファイルスイープが削除したアーティファクトの数。トランスクリプトとサイドカー、録音、ツール結果などのセッションごとのコンパニオンファイル
* `artifacts_deleted`: データディレクトリ全体でスイープが削除した総アイテム。セッションファイルを含む。一部のスイープは削除されたディレクトリツリー全体を 1 つのアイテムとしてカウントし、いくつかのクリーンアップパスはカウンターに貢献しないため、値を正確なファイルカウントではなくフロアとして扱います
* `files_retained_fresh`: 検査され、保持期間内であるため所定の位置に残されたファイル。ファイルごとのスイープのみがこれをカウントするため、値はフロア。ゼロ以外の値は通常の定常状態です
* `files_past_cutoff`: 保持期間より古いファイル。スイープが削除に失敗。例えば、権限エラーまたは開いているファイルのため。ゼロ以上の値は、ファイルが設定された保持期間を超えたことを意味します。ゼロは、ディレクトリ削除全体の失敗が代わりに `error_count` にカウントされるため、何もしなかったことの証明ではありません
* `error_count`: スイープがファイルをリストまたは削除しながら遭遇したエラーの数

<h4 id="managed-settings-resolved-event">
  管理設定解決イベント
</h4>

セッションが解決した [管理設定](/docs/ja/managed-settings): セッション開始時に 1 回、セッション中に管理設定または [ポリシーヘルパー](/docs/ja/managed-settings#compute-the-policy-with-a-helper-program) の状態が変わるときに再度、Claude Code がセッションを開始することを拒否するか、`error.type` 属性がリストする理由の 1 つでセッションを終了するときに。
このイベントを使用して、予期しない管理ソースで実行されているマシン、ポリシーヘルパーが失敗しているマシン、マシンが開始を拒否した理由を見つけます。
Claude Code v2.1.274 以降が必要です。

デフォルトでは、イベントは管理ソースとポリシーヘルパーの状態を持ちますが、設定自体は持ちません。リダクションされた `managed_settings.settings` 属性と `managed_settings.resolved_sha256` ダイジェストを追加するには、`OTEL_LOG_MANAGED_SETTINGS=1` を設定します。

* 管理設定、ユーザー設定、または `--settings` の `env` ブロック、または Claude Code を起動する環境で設定します。プロジェクトまたはローカル設定の値は有効にしません。クローンされたリポジトリはそれらを書き込むことができるため。
* サーバー管理設定は、変数が組織が既に受け取るイベントに組織自体のリダクションされたポリシーのみを追加するため、[セキュリティ承認ダイアログ](/docs/ja/server-managed-settings#security-approval-dialogs) を表示せずに設定できます。

信頼していないフォルダ内のインタラクティブセッションでは、Claude Code は拒否イベントをエクスポートしません。プロジェクトおよびローカル設定はエクスポートを別のコレクターにポイントできるため、[信頼](/docs/ja/permissions#what-runs-before-you-trust-a-folder) する前に。

**イベント名**: `claude_code.managed_settings_resolved`

**属性**:

* すべての [標準属性](#standard-attributes)
* `event.name`: `"managed_settings_resolved"`
* `event.timestamp`: ISO 8601 タイムスタンプ
* `event.sequence`: イベント順序付けのためのプロセスごとのカウンター。[イベント相関属性](#event-correlation-attributes) で説明
* `managed_settings.trigger`: セッション開始イベントの場合は `"startup"`、セッション中に管理設定またはポリシーヘルパーの状態が変わった場合は `"change"`、管理設定ポリシーがセッションを停止した場合は `"refused"`。Claude Code は、属性が前のイベントから異なる場合のみ `change` イベントを送信し、変更された設定値は `OTEL_LOG_MANAGED_SETTINGS` がオフの場合でもカウントされます
* `error.type`: Claude Code がセッションを停止した理由。`refused` イベントにのみ存在:
  * `"helper_failed"`: [ポリシーヘルパー実行が失敗](/docs/ja/settings-reference#helper-failures)
  * `"policy_invalid"`: 管理設定にエラーが含まれており、Claude Code が開始できない、またはアドミンソースが読み込めないため、Claude Code は組織ログイン強制をチェックできません
  * `"consent_rejected"`: ユーザーがサーバー管理設定の [セキュリティ承認ダイアログ](/docs/ja/server-managed-settings#security-approval-dialogs) を拒否しました
  * `"force_refresh_failed"`: [`forceRemoteSettingsRefresh`](/docs/ja/settings-reference#forceremotesettingsrefresh) が必要とする設定フェッチが失敗しました
  * `"gateway_rejected"`: [Claude アプリゲートウェイ](/docs/ja/claude-apps-gateway) が管理設定ロードに HTTP 403 で応答しました
  * `"version_below_minimum"`: この Claude Code バージョンは [`requiredMinimumVersion`](/docs/ja/settings-reference#requiredminimumversion) より下、または [`requiredMaximumVersion`](/docs/ja/settings-reference#requiredmaximumversion) より上です
  * `"_OTHER"`: Claude アプリゲートウェイ管理設定ロードが別の理由で失敗しました
* `managed_settings.sources`: [ポリシーキー](/docs/ja/managed-settings#how-claude-code-combines-managed-sources) を少なくとも 1 つ配信するすべての管理ソース。優先度が最も高い順。`first-wins` の下で効果を持たないソースを含む。値は `"remote"`、`"plist"` または `"hklm"` は MDM または OS レベルのポリシー、`"file"` は管理設定ファイルおよびドロップイン、`"parent"` は [埋め込みホスト](/docs/ja/managed-settings#let-an-embedding-host-add-policy) が設定を提供する場合、`"hkcu"` は Claude Code が [読み取る](/docs/ja/managed-settings#how-claude-code-combines-managed-sources) 場合の [Windows HKCU レジストリ値](/docs/ja/managed-settings#where-each-mechanism-stores-the-policy)。ポリシーキーのみを持つソース、または Claude Code が読み取れなかったソースはリストされていません。文字列の配列として発行。管理ソースがポリシーキーを配信しない場合は空
* `managed_settings.source_behavior`: Claude Code が読み取った [`managedSourcesBehavior`](/docs/ja/settings-reference#managedsourcesbehavior) 値。`"first-wins"` または `"merge"`。キーが設定されていない場合は `"first-wins"`
* `managed_settings.helper.state`: 選択された MDM またはファイルソースが設定するポリシーヘルパーの状態:
  * `"ok"`: ヘルパーの出力が管理設定として機能
  * `"bad_path"`、`"not_a_file"`、`"exit_nonzero"`、`"timed_out"`、`"oversize"`、`"parse_failed"`、`"envelope_invalid"`、または `"schema_rejected"`: ヘルパーの最後の実行が失敗。[ヘルパー障害](/docs/ja/settings-reference#helper-failures) はケースを説明
  * `"none"`: ヘルパーが設定されていない、またはそれを設定するソースが MDM ポリシーまたは管理設定ファイルではない
* `managed_settings.helper.applied`: ヘルパーの独自の出力が管理設定として機能する場合は `"output"`。そうでない場合は `"none"`
* `managed_settings.helper.entry`: Claude Code が [`policyHelper`](/docs/ja/settings-reference#policyhelper) を選択した場合は `"policyHelper"`。ヘルパーを選択しなかった場合は存在しません
* `managed_settings.helper.path`: ヘルパーの設定された [`path`](/docs/ja/settings-reference#policyhelper-path)。Claude Code がヘルパーを選択したときはいつでも存在。`OTEL_LOG_MANAGED_SETTINGS` が設定されているかどうかに関わらず
* `managed_settings.resolved_sha256`（`OTEL_LOG_MANAGED_SETTINGS=1` の場合）: リダクション前の解決された管理設定の SHA-256。JSON としてシリアル化。キーは再帰的にソートされ、空白なし。同じダイジェストを持つマシンは同じポリシーを実行します。Claude Code は短いポリシーを推測をハッシュすることで回復できるため、オプトインでのみダイジェストを送信します。管理設定が解決されない場合は存在しません。`refused` イベントでは存在しません
* `managed_settings.settings`（`OTEL_LOG_MANAGED_SETTINGS=1` の場合）: 解決された管理設定の名前と形状。値はリダクションされます。JSON 文字列として。`refused` イベントでは存在しません。Claude Code はその設定スキーマから構築します:

  * スキーマが宣言する設定名はエクスポートされ、スキーマが宣言しないキーは除外されます
  * ブール値、数値、および文字列値。スキーマが `permissions.defaultMode` などの固定オプションセットに制限する場合、そのままエクスポートされます。`sandbox.network.httpProxyPort` および `sandbox.network.socksProxyPort` は `"[REDACTED]"` としてエクスポートされます
  * その他のすべての文字列。`model`、`apiKeyHelper`、すべての `env` 値、すべての URL、およびすべてのコマンドは `"[REDACTED]"` としてエクスポートされます
  * マップのエントリ名。`env` 変数名およびプラグイン ID はそのままエクスポートされます。スキーマが入力をタイプしない設定。`vimInsertModeRemaps` などは単一の `"[REDACTED]"` としてエクスポートされ、`sandbox.ignoreViolations` はコマンドパターンなしでそのパスリストのリストとしてエクスポートされます
  * リストはその長さを保持し、各エントリは同じルールでリダクションされます
  * `permissions.allow`、`permissions.deny`、または `permissions.ask` ルールは、ツール名がこのバージョンの Claude Code に組み込まれている場合、またはそのツール名がリダクションされたコンテンツを持つ `mcp__` 参照（`mcp__jira__create_issue` など）である場合、そのツール名としてエクスポートされます。その他のルールは `"[REDACTED]"` としてエクスポートされます
  * フックは同じルールに従うため、`type` および `timeout` などの固定オプションおよび数値フィールドが表示されます。各コマンド、URL、`matcher`、および `if` 条件は `"[REDACTED]"` としてエクスポートされます

  例えば、`apiKeyHelper`、2 つの `env` 変数、および拒否ルールを持つ管理設定は `{"apiKeyHelper":"[REDACTED]","env":{"HTTPS_PROXY":"[REDACTED]","CLAUDE_CODE_ENABLE_TELEMETRY":"[REDACTED]"},"permissions":{"deny":["Read([REDACTED])"]}}` としてエクスポートされます。

  Claude Code は値を 8 KB の UTF-8 で切り詰め、切り詰められた値は有効な JSON ではありません
* `managed_settings.settings_truncated`（`managed_settings.settings` が存在する場合）: Claude Code が `managed_settings.settings` を 8 KB で切り詰めた場合は `true`。それ以外の場合は `false`。ブール値として発行。文字列ではなく

<h2 id="interpret-metrics-and-events-data">
  メトリクスとイベントデータの解釈
</h2>

エクスポートされたメトリクスとイベントは、さまざまな分析をサポートします:

<h3 id="usage-monitoring">
  使用状況監視
</h3>

| メトリクス                                                         | 分析の機会                                                                        |
| ------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `claude_code.token.usage`                                     | `type` (入力/出力)、ユーザー、チーム、モデル、`skill.name`、`plugin.name`、または `agent.name` 別に分類 |
| `claude_code.session.count`                                   | 時間経過に伴う採用と関与を追跡                                                              |
| `claude_code.lines_of_code.count`                             | コード追加と削除を追跡して生産性を測定し、モデル別に分類                                                 |
| `claude_code.commit.count` & `claude_code.pull_request.count` | 開発ワークフローへの影響を理解                                                              |

<h3 id="cost-monitoring">
  コスト監視
</h3>

`claude_code.cost.usage` メトリクスは以下に役立ちます:

* チームまたは個人全体の使用トレンドを追跡する
* 最適化のための高使用セッションを特定する
* `skill.name`、`plugin.name`、および `agent.name` 属性を介して、特定のスキル、プラグイン、またはサブエージェントタイプへの支出を属性付けする

<Note>
  コストメトリクスは概算です。公式な請求データについては、API プロバイダー (Claude Console、Amazon Bedrock、または Google Cloud の Agent Platform) を参照してください。
</Note>

Claude Code は、`ANTHROPIC_BASE_URL` の背後にあるゲートウェイまたはプロキシが複数のフレーム全体で使用状況をプログレッシブにストリーミングする場合を含め、各ストリーミングレスポンスをコストおよびトークンメトリクスに対して正確に 1 回カウントします。v2.1.214 より前では、複数のフレームで使用状況を含むストリームは、`claude_code.cost.usage` と `claude_code.token.usage` を追加フレームごとにおよそ 1 つの追加フルリクエスト分だけ増加させました。

<h3 id="alerting-and-segmentation">
  アラートとセグメンテーション
</h3>

検討すべき一般的なアラート:

* コストスパイク
* 異常なトークン消費
* 特定のユーザーからの高いセッションボリューム

すべてのメトリクスは、[標準属性](#standard-attributes) でセグメント化できます。`model` 属性は `claude_code.token.usage`、`claude_code.cost.usage`、および v2.1.172 以降の `claude_code.lines_of_code.count` で利用可能です。

コミットのモデル別の内訳は、1 つのセッションが複数のモデルにまたがる可能性があるため、`session.id` でトークンまたはコストメトリクスに対して結合することによってのみ概算できます。トークンまたはコスト側をフィルタリングして、`query_source` が `"main"` である行のみにしてください。これにより、補助的なリクエストとサブエージェントリクエストが、セッションのコミットをそれらを作成しなかったモデルに属性付けしません。

<h3 id="detect-retry-exhaustion">
  再試行枯渇の検出
</h3>

Claude Code は失敗した API リクエストを内部的に再試行し、あきらめた後にのみ単一の `claude_code.api_error` イベントを出力するため、イベント自体がそのリクエストの終端信号です。中間再試行試行は個別のイベントとしてログされません。

イベントの `attempt` 属性は、試行の総数を記録します。`CLAUDE_CODE_MAX_RETRIES` はデフォルトで 10 で、15 で上限です。v2.1.199 以降では、`CLAUDE_CODE_RETRY_WATCHDOG` を設定してデフォルトを引き上げ、上限を削除できます。

リクエストが一時的なエラーのすべての再試行を枯渇させた場合、`attempt` はその有効な制限より 1 つ多くなります: デフォルトでは 11、ウォッチドッグが設定されていない限り 16 を超えることはありません。より低い値は、`400` レスポンスなどの再試行不可能なエラー、または独自のより小さい再試行予算を持つ原因を示します。たとえば、Claude Code は AWS または Google Cloud 認証情報の読み込み失敗を最大 2 回再試行します。

セッションが回復したものと停止したものを区別するには、イベントを `session.id` でグループ化し、エラーの後に後続の `api_request` イベントが存在するかどうかを確認します。

<h3 id="event-analysis">
  イベント分析
</h3>

イベントデータは Claude Code インタラクションに関する詳細な洞察を提供します:

**ツール使用パターン**: ツール結果イベントを分析して以下を特定します:

* 最も頻繁に使用されるツール
* ツール成功率
* 平均ツール実行時間
* ツールタイプ別のエラーパターン

**パフォーマンス監視**: API リクエスト期間とツール実行時間を追跡して、パフォーマンスボトルネックを特定します。

<h2 id="audit-security-events">
  監査セキュリティイベント
</h2>

OpenTelemetry イベントは Claude Code アクティビティの監査データソースです。すべてのイベントは、ツール呼び出し、MCP アクティビティ、権限決定をそれらをトリガーしたユーザーに結び付ける ID 属性を持ち、OTLP ログエクスポーターは、これらのイベントを OTLP レシーバーを持つセキュリティ情報およびイベント管理（SIEM）プラットフォーム、または SIEM にフォワードする OpenTelemetry Collector に配信できます。

<h3 id="attribute-actions-to-users">
  属性アクションをユーザーに関連付ける
</h3>

各イベントの [標準属性](#standard-attributes) には、認証されたユーザーの ID が含まれます：Claude アカウントでサインインしている場合は `user.email`、`user.account_uuid`、`user.account_id`、および `organization.id`、さらに [クラウドセッション](/docs/ja/claude-code-on-the-web) では、セッション自体の認証情報がそれらを持つ場合、`user.id` とセッションごとの `session.id`。`user.id` はインストールスコープの識別子です。ただし、[Claude apps gateway](/docs/ja/claude-apps-gateway) セッションでは、ゲートウェイが発行したトークンからの IdP サブジェクトです。

MCP ツール呼び出し、Bash コマンド、ファイル編集は、セッションを開始した開発者に属性付けられます。Claude Code は個別のサービスアカウントの下では機能しません。各イベントに記録される ID は、開発者自身の Claude アカウント、または [Claude apps gateway](/docs/ja/claude-apps-gateway) セッションでの開発者の IdP ID です。

Claude Code が直接 API キーで認証する場合、または Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry に対して認証する場合、セッションに Claude アカウントはなく、`user.id` と `session.id` のみが入力されます。これらのデプロイメントでは、`OTEL_RESOURCE_ATTRIBUTES` を使用してユーザー ID を自分で添付し、[管理設定](#administrator-configuration) ファイルまたはローンチラッパーを通じてユーザーごとに設定します。Claude apps gateway セッションはこれを必要としません：CLI は [標準属性](#standard-attributes) で説明されているように、IdP ID を自動的にスタンプします。

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

<h3 id="audit-mcp-activity">
  MCP アクティビティを監査する
</h3>

完全なコール詳細で MCP サーバーアクティビティをキャプチャするには、ログエクスポーターを有効にし、`OTEL_LOG_TOOL_DETAILS=1` を設定します。その後、各 MCP 操作は、標準 ID 属性と共にサーバー名、ツール名、呼び出し引数を含む構造化イベントを生成します：

| イベント                    | MCP に対して記録するもの                                                                                                                                        |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_server_connection` | `server_name`、`transport_type`、`server_scope`、およびエラー詳細を含むサーバー接続、切断、接続失敗                                                                               |
| `tool_result`           | `tool_name` および `mcp_server_scope` を含む各 MCP ツール呼び出し、`mcp_server_name` および `mcp_tool_name` を含む `tool_parameters` ペイロード、および呼び出し引数を含む `tool_input` ペイロード |
| `tool_decision`         | 呼び出しが許可されたか拒否されたか、および決定が設定、フック、またはユーザーから来たかどうか、および `mcp_server_name` と `mcp_tool_name` を含む `tool_parameters` ペイロード                                    |

`OTEL_LOG_TOOL_DETAILS` がない場合、これらのイベントは識別詳細を削除します：

* `tool_result`：`mcp_server_scope` と、ユーザー設定サーバーの場合はリテラル `"mcp_tool"` に編集された `tool_name` を保持し、引数コンテンツを省略します。Claude Desktop の組み込みサーバーの場合、Claude Desktop が所有するセッションでは、`tool_parameters` 内の `mcp_server_name`/`mcp_tool_name` ペアも保持します。これは `tool_decision` と同じホスト作成例外です。Claude Code v2.1.214 以降が必要です
* `tool_decision`：`tool_source` と、ユーザー設定サーバーの場合はリテラル `"mcp_tool"` に編集された `tool_name` を保持し、引数コンテンツを省略します。Claude Desktop の組み込みサーバーの場合、Claude Desktop が所有するセッションでは、`tool_parameters` 内の `mcp_server_name`/`mcp_tool_name` ペアも保持します。`tool_source` と名前ペアの両方に Claude Code v2.1.214 以降が必要です
* `mcp_server_connection`：`server_name` とエラーメッセージを省略しますが、`is_plugin`、`plugin_id_hash`、および `plugin.name` を保持し、Anthropic 以外のプラグイン名はリテラル `"third-party"` に編集されるため、プラグイン提供サーバーは詳細ログなしで区別可能なままです

<h3 id="map-security-questions-to-events">
  セキュリティの質問をイベントにマップする
</h3>

検出ルールを構築する場合、監視したいシグナルを検索し、対応するイベントと属性についてバックエンドをクエリします：

| シグナル                                               | イベント                                                                    | キー属性                                                                                                                                                                                                                                        |
| -------------------------------------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ツール呼び出しが許可または拒否され、何によって                            | `tool_decision`                                                         | `decision`、`source`、`tool_name`、`tool_parameters`                                                                                                                                                                                           |
| 権限モードのエスカレーション                                     | `permission_mode_changed`                                               | `from_mode`、`to_mode`、`trigger`                                                                                                                                                                                                             |
| ポリシーフックがアクションをブロック                                 | `hook_execution_complete`                                               | `hook_event`、`num_blocking`                                                                                                                                                                                                                 |
| ログイン、ログアウト、認証失敗                                    | `auth`                                                                  | `action`、`success`、`error_category`                                                                                                                                                                                                         |
| MCP サーバー接続または失敗                                    | `mcp_server_connection`                                                 | `status`、`server_name`、`is_plugin`、`error_code`                                                                                                                                                                                             |
| プラグインがインストールされ、そのソース                               | `plugin_installed`                                                      | `plugin.name`、`marketplace.name`、`marketplace.is_official`                                                                                                                                                                                  |
| 実行されたコマンドとタッチされたファイル                               | `tool_result`（実行）または `tool_decision`（拒否）（`OTEL_LOG_TOOL_DETAILS=1` の場合） | `tool_parameters`；`tool_input`（`tool_result` のみ）                                                                                                                                                                                            |
| マシンが実行する管理設定ソース、そのポリシーヘルパーが正常かどうか、およびマシンが起動を拒否した理由 | `managed_settings_resolved`                                             | `managed_settings.trigger`、`managed_settings.sources`、`managed_settings.source_behavior`、`managed_settings.helper.state`、`error.type`；`managed_settings.settings` および `managed_settings.resolved_sha256`（`OTEL_LOG_MANAGED_SETTINGS=1` の場合） |

Claude Code は生のイベントストリームのみを出力します。異常検出、ベースライン化、セッション間の相関、アラートは SIEM または可観測性バックエンドの責任です。

<h3 id="send-events-to-a-siem">
  SIEM にイベントを送信する
</h3>

`OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` を SIEM の OTLP レシーバーに、または SIEM のネイティブ取り込み API にフォワードする OpenTelemetry Collector に指定します。以下の管理設定の例は、MCP および Bash 監査のための完全なツール詳細を有効にして、イベントのみをエクスポートします：

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_LOG_TOOL_DETAILS": "1",
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT": "https://siem.example.com:4318/v1/logs",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-siem-token"
  }
}
```

イベントが到着したことを確認するには、この設定で実行されているセッションでプロンプトを送信し、SIEM で `claude_code.user_prompt` イベントを確認します。何も到着しない場合は、`claude --debug` を実行し、デバッグログで `[3P telemetry]` エクスポートエラーを確認します。

<h2 id="backend-considerations">
  バックエンドに関する考慮事項
</h2>

メトリクス、ログ、トレースバックエンドの選択により、実行できる分析のタイプが決まります:

<h3 id="for-metrics">
  メトリクスの場合
</h3>

* **時系列データベース**: レート計算、集約メトリクス
* **カラムナーストア**: 複雑なクエリ、一意のユーザー分析
* **フル機能の可観測性プラットフォーム**: 高度なクエリ、可視化、アラート

<h3 id="for-events/logs">
  イベント/ログの場合
</h3>

* **ログ集約システム**: 全文検索、ログ分析
* **カラムナーストア**: 構造化イベント分析
* **フル機能の可観測性プラットフォーム**: メトリクスとイベント間の相関

<h3 id="for-traces">
  トレースの場合
</h3>

分散トレースストレージとスパン相関をサポートするバックエンドを選択します:

* **分散トレースシステム**: スパン可視化、リクエストウォーターフォール、レイテンシー分析
* **フル機能の可観測性プラットフォーム**: トレース検索とメトリクスおよびログとの相関

日次/週次/月次アクティブユーザー (DAU/WAU/MAU) メトリクスが必要な組織の場合は、効率的な一意値クエリをサポートするバックエンドを検討してください。

<h2 id="service-information">
  サービス情報
</h2>

すべてのメトリクスとイベントは、以下のリソース属性でエクスポートされます:

* `service.name`: ターミナルセッションの場合は `claude-code`、[Claude Desktop アプリ](/docs/ja/desktop)のコードタブから開始されたセッションの場合は `claude-code-desktop`
* `service.version`: 現在の Claude Code バージョン、またはコードタブセッションの場合は Desktop アプリバージョン
* `os.type`: オペレーティングシステムタイプ (例: `linux`、`darwin`、`windows`)
* `os.version`: オペレーティングシステムバージョン文字列
* `host.arch`: ホストアーキテクチャ (例: `amd64`、`arm64`)
* `wsl.version`: WSL バージョン番号 (Windows Subsystem for Linux で実行している場合のみ存在)
* メーター名: `com.anthropic.claude_code`

`service.name = claude-code` でフィルタリングするコレクターパイプラインまたはダッシュボードがある場合は、コードタブセッションからのテレメトリもキャプチャするために、フィルターに `claude-code-desktop` を追加してください。

<h2 id="roi-measurement-resources">
  ROI 測定リソース
</h2>

テレメトリセットアップ、コスト分析、生産性メトリクス、自動レポート生成を含む Claude Code の投資収益率（ROI）測定に関する包括的なガイドについては、[Claude Code ROI 測定ガイド](https://github.com/anthropics/claude-code-monitoring-guide)を参照してください。このリポジトリは、すぐに使用できる Docker Compose 設定、Prometheus と OpenTelemetry セットアップ、Linear などのツールと統合された生産性レポート生成テンプレートを提供します。

<h2 id="security-and-privacy">
  セキュリティとプライバシー
</h2>

* OpenTelemetry エクスポートをバックエンドに送信することはオプトインであり、明示的な設定が必要です。Anthropic の個別の運用テレメトリーと無効化方法については、[データ使用](/docs/ja/data-usage#telemetry-services)を参照してください
* ファイルの生コンテンツとコードスニペットはメトリクスやイベントに含まれません。トレーススパンは別のデータパスです。以下の `OTEL_LOG_TOOL_CONTENT` の項目を参照してください
* OAuth 経由で認証されている場合、`user.email` はテレメトリー属性に含まれ、設定した OTel エンドポイントにのみ送信され、Anthropic には送信されません。これが組織にとって懸念事項である場合は、テレメトリーバックエンドと協力してこのフィールドをフィルタリングまたは編集してください
* ユーザープロンプトコンテンツはデフォルトでは収集されません。プロンプト長のみが記録されます。プロンプトコンテンツを含めるには、`OTEL_LOG_USER_PROMPTS=1` を設定してください。詳細なベータトレースでは、この変数はプロンプトテキストより広い範囲に達します。これは [`new_context` スパン属性](#new-context-gates)もゲートします。これは `claude_code.llm_request` スパンのツール結果を含みます
* アシスタント応答テキストはデフォルトでは収集されません。応答長のみが記録されます。応答テキストを含めるには、`OTEL_LOG_ASSISTANT_RESPONSES=1` を設定してください。Claude Code からのすべての OpenTelemetry データと同様に、応答テキストは設定した OTel エンドポイントにのみ送信され、Anthropic には送信されません。この変数が設定されていない場合、`OTEL_LOG_USER_PROMPTS` がフォールバックとして使用されるため、プロンプトコンテンツなしで応答コンテンツが必要な場合は `OTEL_LOG_ASSISTANT_RESPONSES=0` を設定してください
* ツール入力引数とパラメータはデフォルトではログに記録されません。これらを含めるには、`OTEL_LOG_TOOL_DETAILS=1` を設定してください。Claude Desktop の組み込みサーバーの場合、Claude Desktop が所有するセッションでは、`tool_decision` と `tool_result` は `mcp_server_name`/`mcp_tool_name` ペアを含みます。これはホーム作成者の名前であり、フラグがオフの場合でも引数コンテンツではありません。この例外には Claude Code v2.1.214 以降が必要です。このデータは設定した OTEL エンドポイントにのみ送信され、Anthropic には送信されません。引数には機密値が含まれる可能性があるため、テレメトリーバックエンドを設定してこれらの属性をフィルタリングまたは編集してください。有効にすると：
  * `tool_result` と `tool_decision` イベントには、Bash コマンド、MCP サーバーとツール名、スキル名を含む `tool_parameters` 属性が含まれます。`full_command` などのフィールドは切り詰められずに出力されます
  * `tool_result` イベントには、ファイルパス、URL、検索パターン、その他の引数を含む `tool_input` 属性も含まれます。512 文字を超える個別の値は切り詰められ、合計は約 4 K 文字に制限されます
  * `user_prompt` イベントには、カスタム、プラグイン、MCP コマンドの逐語的な `command_name` が含まれます
  * トレーススパンには、同じ `tool_input` 属性と `file_path` などの入力派生属性が含まれ、`tool_input` と同じ切り詰めが行われます
* ツールコンテンツはデフォルトではトレーススパンにログに記録されません。これを含めるには、`OTEL_LOG_TOOL_CONTENT=1` を設定してください。その後、`claude_code.tool` スパンは、ファイルの生コンテンツと Bash コマンド出力を含む [`tool.output` スパンイベント](#tool-output-span-event)を含みます。これは属性ごとのコンテンツ制限（デフォルトでは 60 KB）で切り詰められます。ツールコンテンツは [`new_context`](#new-context-gates) を通じてスパンに到達します。このゲートはスパンごとに異なります。テレメトリーバックエンドを設定してこれらの属性をフィルタリングまたは編集してください
* 生の Anthropic Messages API リクエストおよびレスポンスボディはデフォルトではログに記録されません。これらを含めるには、シェル、ユーザー設定、または管理設定で `OTEL_LOG_RAW_API_BODIES` を設定してください。これは [プロジェクトおよびローカル設定](/docs/ja/settings-reference#variables-claude-code-ignores-in-env)では無視されます。ボディには、システムプロンプト、すべての以前のユーザーとアシスタントのターン、ツール結果を含む完全な会話履歴が含まれるため、これを有効にすることは、他の `OTEL_LOG_*` コンテンツフラグが明かすすべてのものへの同意を意味します。Claude Code は、他の設定に関係なく、これらのボディから Claude の拡張思考コンテンツを常に編集します。設定する値は、Claude Code がボディを配信する方法を決定します：
  * `=1` の場合、Claude Code は各 API 呼び出しに対して `api_request_body` と `api_response_body` ログイベントを出力します。イベントの `body` 属性は JSON シリアル化されたペイロードを含み、コンテンツ制限（デフォルトでは 60 KB）で切り詰められます
  * `=file:<dir>` の場合、Claude Code は切り詰められていないボディをそのディレクトリの `.request.json` と `.response.json` ファイルに書き込み、イベントはインラインボディの代わりに `body_ref` パスを含みます。ディレクトリをテレメトリーストリームではなく、ログコレクターまたはサイドカーと一緒に配布してください。

    各成功したレスポンスについて、Claude Code はそのディレクトリの `index.jsonl` に 1 行を追加し、レスポンスファイルをそれを生成したリクエストファイルおよびそれが成為したトランスクリプトメッセージにリンクします。各行はメッセージコンテンツを含まず、[API レスポンスボディイベント](#api-response-body-event)セクションがそのフィールドをリストします。インデックスファイルには Claude Code v2.1.274 以降が必要です

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Amazon Bedrock での Claude Code の監視
</h2>

Amazon Bedrock での Claude Code 使用状況監視ガイダンスの詳細については、[Claude Code 監視実装（Amazon Bedrock）](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)を参照してください。
