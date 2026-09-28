> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK のトラブルシューティング

> Claude Code CLI が起動に失敗した場合、CLI プロセスが終了した場合、または構造化出力なしで成功結果が返された場合の Agent SDK エラーを修正します。

このページは、CLI 起動、CLI プロセス終了、および構造化出力に関連する Agent SDK エラーについて説明しています。このページのエントリは、表示されるエラーに対応しています。各エントリは原因と対処方法を示しています。

フック発火やスキル使用など、機能に関連する症状については、その機能のページにトラブルシューティングセクションがあります。以下の表は、各症状をカバーするセクションまたはページを示しています。

| 症状                                                                                                                                                                  | 参照先                                                                                            |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------- |
| スキルが見つからない、スキルが使用されていない、`Invalid skill name` エラー                                                                                                                    | [スキルのトラブルシューティング](/docs/ja/agent-sdk/skills#troubleshooting)                                        |
| MCP サーバーが `failed` ステータスを表示、ツールが呼び出されない、接続タイムアウト、最大許容トークン数を超えるツール出力                                                                                                 | [MCP のトラブルシューティング](/docs/ja/agent-sdk/mcp#troubleshooting)                                          |
| プラグインが読み込まれない、プラグインスキルが表示されない                                                                                                                                       | [プラグインのトラブルシューティング](/docs/ja/agent-sdk/plugins#troubleshooting)                                     |
| Claude がサブエージェントに委譲しない、ファイルシステムベースのエージェントが読み込まれない                                                                                                                   | [サブエージェントのトラブルシューティング](/docs/ja/agent-sdk/subagents#troubleshooting)                                |
| チェックポイント オプションが認識されない、UUID なしのユーザーメッセージ、`No file checkpoint found`、`File rewinding is not enabled`、`ProcessTransport is not ready for writing`                      | [ファイルチェックポイントのトラブルシューティング](/docs/ja/agent-sdk/file-checkpointing#troubleshooting)                   |
| フックが発火しない、マッチャーが期待通りにフィルタリングしない、フックタイムアウト、ツールが予期せずブロックされた、変更された入力が適用されない、Python でセッションフックが利用できない、サブエージェント権限プロンプトが増加、サブエージェントとの再帰的フックループ、`systemMessage` が出力に表示されない  | [一般的な問題を修正](/docs/ja/agent-sdk/hooks#fix-common-issues)（フックページ）                                     |
| マシンで動作するエージェントがデプロイされたサービスまたはコンテナで失敗する                                                                                                                              | [デプロイメント失敗のトラブルシューティング](/docs/ja/agent-sdk/hosting#troubleshoot-deployment-failures)                |
| `Not logged in`、`Invalid API key`、`API Error`、`429`、`There's an issue with the selected model`                                                                      | [エラーリファレンス](/docs/ja/errors#find-your-error)                                                        |
| `CLINotFoundError`、`CLIConnectionError`、`ProcessError`、`Claude Code process exited with code N`、`Claude Code returned an error result`、`structured_output` が `None` | このページの [CLI 起動](#cli-startup)、[CLI プロセス終了](#cli-process-exit)、および [構造化出力](#structured-outputs) |

<h2 id="cli-startup">
  CLI スタートアップ
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError: Claude Code not found
</h3>

Python SDK は Claude Code CLI をサブプロセスとして起動します。`claude` 実行ファイルが見つからない場合、接続は `CLINotFoundError` で失敗します。

```
Claude Code not found at: /your/configured/path
```

メッセージには、`ClaudeAgentOptions(cli_path=...)` を設定して存在しないファイルを指している場合、設定されたパスが含まれます。`cli_path` がない場合、SDK は `PATH` と一般的なインストール場所を検索し、メッセージにはプラットフォーム用のインストール手順が含まれます。

修正するには：

* Claude Code がインストールされていない場合はインストールしてください。プラットフォーム用のコマンドについては、[Claude Code のインストール](/docs/ja/setup#install-claude-code)を参照してください。
* `cli_path` を設定した場合は、ファイルが存在し、`claude` 実行ファイルであることを確認してください。
* `PATH` 解決に依存している場合は、アプリケーションが実行される環境で `claude --version` が機能することを確認してください。IDE やサービスマネージャーなど、シェルの外から起動するプロセスは、多くの場合異なる `PATH` で実行されます。

TypeScript SDK は、バンドルされたプラットフォームパッケージと `pathToClaudeCodeExecutable` に設定されたパスで CLI を探します。表示されるメッセージに一致させてください：

* `Native CLI binary for <platform>-<arch> not found`：バンドルされたプラットフォームパッケージが見つかりません。最も一般的には、インストールがオプション依存関係をスキップしたためです。オプション依存関係をスキップせずに `@anthropic-ai/claude-agent-sdk` を再インストールするか、`pathToClaudeCodeExecutable` を[ネイティブインストール](/docs/ja/setup#install-claude-code)に指定してください。`bun build --compile` で構築された単一ファイル実行ファイルでは、同じメッセージが異なる原因と修正方法を持ちます。[単一実行ファイルへのコンパイル](/docs/ja/agent-sdk/typescript#compile-to-a-single-executable)を参照してください。
* `Claude Code native binary not found at <path>` または `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`：解決されたパスのファイルが見つからないか、プロセスがアクセスできません。ファイルがそのパスに存在し、プロセスがアクセスできることを確認してください。

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError: Refusing to execute batch script
</h3>

Windows では、Python SDK が使用する CLI パスが `.bat` または `.cmd` バッチスクリプト（npm インストールが作成する `claude.cmd` シムを含む）の場合、接続は `CLIConnectionError` で失敗します。

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

この拒否は意図的なセキュリティ強化であり、インストールが壊れているわけではありません。Windows はバッチスクリプトを `cmd.exe /c` 呼び出しに書き換えて実行し、`cmd.exe` は実行時にコマンドライン全体を再解析するため、引数値は注入されたコマンドを実行できます。

ほとんどの Windows インストールはこのエラーに到達しません。Windows x64 の `claude-agent-sdk` ホイールは `claude.exe` をバンドルしており、SDK はバンドルされた CLI を優先し、次に発見できるネイティブ `claude.exe` を優先し、その後バッチシムにフォールバックします。拒否は 2 つのケースで表示されます：

* `ClaudeAgentOptions(cli_path=...)` を `.bat` または `.cmd` ファイル（npm の `claude.cmd` シムなど）に設定した場合。
* インストールにバンドルされたネイティブ `claude.exe` がない場合。たとえば、ARM64 Windows でのソースインストールで、`PATH` 上の唯一の `claude` が npm シムである場合。

修正するには、バッチスクリプトの代わりにネイティブ実行ファイルを SDK に提供してください：

* `ClaudeAgentOptions(cli_path=...)` を設定した場合は、`claude.exe` を指すか、オプションを削除してください。`cli_path` が設定されている間、SDK は検出をスキップするため、ネイティブインストールだけでは効果がありません。
* PowerShell で Claude Code をネイティブにインストールしてください：`irm https://claude.ai/install.ps1 | iex`
* x64 Windows では、`claude.exe` をバンドルする `claude-agent-sdk` ホイールをインストールしてください。

`claude-agent-sdk` 0.2.124 より前では、Python SDK はこのチェックなしで `cmd.exe` を通じてバッチスクリプトを生成していました。

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError: Failed to start Claude Code
</h3>

SDK は解決されたパスでファイルを見つけましたが、起動できませんでした。Python はこれらの失敗を `CLIConnectionError` として発生させます。TypeScript はメッセージ反復を SDK クラスを持たないエラーで拒否します。以下の表は各メッセージを何を示しているかにマップします。表示されるメッセージに一致させてください：

| メッセージ                                                             | SDK        | 意味                                 |
| ----------------------------------------------------------------- | ---------- | ---------------------------------- |
| `Failed to start Claude Code: <detail>`                           | Python     | メッセージの残りはオペレーティングシステム自体のエラーです      |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | 設定されたパスのスクリプトは実行できません              |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | バイナリは実行できません。libc の提案がメッセージに追加されます |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | その他の起動失敗                           |

両方の SDK では、通常の原因は、テキストファイル、ディレクトリ、または実行権限のないファイルなど、実行できないものを指す解決されたパスです。ネイティブバイナリメッセージの libc 提案を 1 つの可能な原因として読んでください。

どちらの SDK でも修正するには：

* 設定されたパスが `claude` 実行ファイル自体を指し、ファイルに実行権限があることを確認してください。
* カスタムパスが不要な場合は、Python で `cli_path` を削除するか、TypeScript で `pathToClaudeCodeExecutable` を削除して、SDK が独自に CLI を見つけるようにしてください。バンドルされたコピーを優先します。
* 失敗しているバイナリがコンテナイメージ内の SDK のバンドルされたコピーの場合、イメージビルド中に SDK を再インストールして、バンドルされたバイナリがコンテナのプラットフォームと一致するようにするか、実行するアーキテクチャ用にイメージを再構築してください。通常の原因は、コンテナのアーキテクチャまたは libc と一致しないバイナリ、またはイメージビルド中に実行権限を失ったバイナリです。

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError: Not connected
</h3>

Python で `ClaudeSDKClient` メソッドを呼び出す前にクライアントが接続していない場合、または接続を切断した後に呼び出すと、`CLIConnectionError` がこのメッセージで発生します：

```
Not connected. Call connect() first.
```

メッセージが言うことをしてください。他のクライアントメソッドの前に `await client.connect()` を呼び出すか、`async with ClaudeSDKClient() as client:` でクライアントを開いてください。これは入口で接続します。

<h2 id="cli-process-exit">
  CLI プロセス終了
</h2>

このセクションのエントリは、アプリケーションがそれを使用している間に Claude Code プロセスが終了したことを意味します。表示されるエラーは SDK 言語と、CLI が終了する前にエラー結果を報告したかどうかによって異なります。

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError: Command failed with exit code
</h3>

Python SDK は Claude Code プロセスが 0 以外のコードで終了すると `ProcessError` を発生させます：

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

メッセージは終了コードを 2 回述べ、`Error output` 行は固定テキストであり、プロセスの実際のエラー出力ではありません。同じ固定テキストが例外の `stderr` 属性を埋めます。例外の `exit_code` 属性がコードを持ちます。CLI が実際に stderr に書き込んだものをキャプチャするには、`ClaudeAgentOptions` で `stderr` コールバックを渡し、受け取ったものをログしてください。

単純な `ProcessError` は、CLI がエラー結果を報告せずに終了したことを意味します。CLI がエラー結果を報告した場合、SDK は代わりに [`ResultError`](/docs/ja/agent-sdk/python#resulterror) を発生させます。これは[Claude Code returned an error result](#claude-code-returned-an-error-result)で説明されています。`ResultError` は `ProcessError` をサブクラス化するため、`except ProcessError` は両方をキャッチします。異なる方法で処理するには、`except ResultError` 句を最初に配置してください。

`claude-agent-sdk` 0.2.140 より前では、Python SDK はエラー結果の終了を `ResultError` ではなく単純な `Exception` として発生させていました。

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

IDE ラッパーもこのメッセージを出力し、[エラーリファレンス](/docs/ja/errors#claude-code-process-exited-with-code-n)は VS Code およびその他のランチャーをカバーしています。このエントリは TypeScript SDK コードが受け取るものをカバーしています。SDK は 0 以外の CLI 終了を単純な `Error` として表示し、`query()` のメッセージ上の `for await` ループを拒否します。キャッチする SDK エラークラスはないため、ループを `try`/`catch` でラップし、メッセージで一致させてください：

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

CLI が stderr に書き込んだ場合、メッセージはそれのテールで終わります。完全なストリームをキャプチャするには、クエリオプションで `stderr` コールバックを渡してください。シグナルで強制終了されたプロセスは同じ形式で `Claude Code process terminated by signal <name>` を報告します。

<h3 id="claude-code-returned-an-error-result">
  Claude Code returned an error result
</h3>

両方の SDK は、CLI が終了する前にエラー結果を報告した場合、プロセス終了エラーをこのメッセージに置き換えます：

```
Claude Code returned an error result: <the CLI's own error report>
```

コロンの後のテキストは、何が間違ったかについての CLI の報告であるため、終了自体ではなくそこから始めてください。Python はこれを [`ResultError`](/docs/ja/agent-sdk/python#resulterror) として発生させます。その `data` 属性は完全なエラー結果を持ちます。TypeScript は同じメッセージ形式を持つ単純な `Error` でメッセージループを拒否します。

<h2 id="structured-outputs">
  構造化出力
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output は None ですが、結果は成功と言っています
</h3>

結果メッセージは `subtype: "success"` で終わることができますが、Python では `structured_output` は `None` であり、TypeScript では `undefined` です。実行は完了しますが、検証された出力は存在しません。これに到達する 1 つの方法は、出力が満たすことができないスキーマです。たとえば、矛盾する長さの制約があります。実行は検証エラーなしで終了し、唯一の信号は欠落している `structured_output` です。

アプリケーションコードでこの結果を失敗として扱ってください。`structured_output` を使用する前に、`subtype` が `success` であり、`structured_output` が存在することの両方を確認してください。[エラーハンドリング](/docs/ja/agent-sdk/structured-outputs#error-handling)セクションは両方の SDK のこのパターンを示しています。

正しいと思われるスキーマで繰り返し発生する場合は、スキーマが満たされることを確認し、出力が検証されるまで単純化し、制約を 1 つずつ再導入してください。

<h2 id="report-a-new-issue">
  新しい問題を報告する
</h2>

エラーがここでカバーされていない場合は、オープンな問題を確認するか、SDK リポジトリに新しい問題を提出してください：[claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) または [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues)。完全なエラーテキストと SDK バージョンを含めてください。
