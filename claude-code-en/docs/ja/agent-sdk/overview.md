> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK の概要

> Claude Code をライブラリとして使用して、本番環境対応の AI エージェントを構築します

エージェントは、独自のステップを計画し、ファイルを読み取る、コマンドを実行する、またはコードを編集するツールを呼び出すことでタスクを完了するアプリケーションです。Agent SDK は、Claude Code を強化する同じツール、[エージェントループ](/docs/ja/agent-sdk/agent-loop)、およびコンテキスト管理を提供し、Python と TypeScript でプログラム可能です。

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Agent SDK と他の Claude ツールの比較
</h2>

Agent SDK、CLI、Client SDK、および Managed Agents は、エージェントを実行するのは誰か、何が組み込まれているか、どのようにアクセスするかが異なります。構築して実行する方法に合致する行を見つけてください。

| 対象                                                                       | 使用するツール                                                                           | 取得内容                                                                                                                                                                                                                                                                                                          |
| ------------------------------------------------------------------------ | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code のエージェントを自分で操作するプロセスで、独自の Python または TypeScript アプリケーションに埋め込む | **Agent SDK**                                                                     | Claude Code バイナリを実行するライブラリで、組み込みツール、権限、セッション、hooks などの Claude Code の[機能](#capabilities)を備えています。                                                                                                                                                                                                               |
| ターミナルから対話的な開発またはワンオフタスクを実行する                                             | [**Claude Code CLI**](/docs/ja/overview)                                               | 日常的な対話的使用のために構築されたターミナルインターフェース。                                                                                                                                                                                                                                                                              |
| 独自のコードから Claude API を直接呼び出す                                              | [**Client SDK**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | クライアント SDK 言語のいずれからでも Claude API に直接アクセスできます。ツールループを自分で記述するか、クライアント SDK のベータ版[ツールランナー](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner)に駆動させることができます。                                                                                                                               |
| Anthropic がエージェントをホストし、Claude API を通じて構成する                               | [**Managed Agents**](https://platform.claude.com/docs/en/managed-agents/overview) | エージェントループを実行するホストされたエージェントハーネスで、Anthropic が管理するクラウドサンドボックスまたは独自のインフラストラクチャ上の[セルフホストサンドボックス](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes)のセッションを備えています。[言語用の SDK](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk)、`ant` CLI、または REST API から使用できます。 |

Python または TypeScript 以外の言語から同じエージェントループを駆動するには、[`-p` フラグと `--output-format json` を使用して CLI をサブプロセスとして実行](/docs/ja/headless)してください。

<h2 id="capabilities">
  機能
</h2>

これらの Claude Code 機能は SDK で利用可能です。

| 機能              | 機能                                                           | 詳細を学ぶ                                                                                                                                                                                 |
| --------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 組み込みツール         | ファイルの読み取り、書き込み、編集、コマンド実行、ウェブ検索                               | [ツールリファレンス](/docs/ja/tools-reference)                                                                                                                                                      |
| Hooks           | エージェントライフサイクルの重要なポイントでカスタムコードを実行                             | [Hooks](/docs/ja/agent-sdk/hooks)                                                                                                                                                          |
| Subagents       | 特定のサブタスク用に特化したエージェントを生成                                      | [Subagents](/docs/ja/agent-sdk/subagents)                                                                                                                                                  |
| MCP             | Model Context Protocol を介して外部ツールとデータソースを接続                   | [MCP](/docs/ja/agent-sdk/mcp)                                                                                                                                                              |
| 権限              | どのツールが自動的に実行されるか、どのツールが承認を必要とするかを制御                          | [権限](/docs/ja/agent-sdk/permissions)                                                                                                                                                       |
| セッション           | 複数の交換にわたってコンテキストを維持し、後で再開またはフォーク                             | [セッション](/docs/ja/agent-sdk/sessions)                                                                                                                                                       |
| Skills、コマンド、メモリ | プロジェクトの `.claude/` と `~/.claude/` から自動的に読み込み、Claude Code と同じ | [Skills](/docs/ja/agent-sdk/skills)、[コマンド](/docs/ja/agent-sdk/skills#commands-in-agent-sdk-sessions)、[メモリ](/docs/ja/agent-sdk/modifying-system-prompts)、[設定読み込み](/docs/ja/agent-sdk/claude-code-features) |
| Plugins         | Skills、エージェント、hooks、MCP サーバーをパッケージ化し、ローカルパスで読み込み             | [Plugins](/docs/ja/agent-sdk/plugins)                                                                                                                                                      |

<h2 id="get-started">
  はじめに
</h2>

[クイックスタート](/docs/ja/agent-sdk/quickstart)に従って、SDK をインストールし、API キーを設定し、既存のコード内のバグを見つけて修正するエージェントを構築してください。

<Note>
  事前に承認されていない限り、Anthropic は、Agent SDK 上に構築されたエージェントを含む、サードパーティの開発者が claude.ai ログインまたはレート制限を提供することを許可していません。代わりに、[クイックスタート](/docs/ja/agent-sdk/quickstart)で説明されている API キー認証方法を使用してください。
</Note>

<h2 id="changelog">
  変更ログ
</h2>

SDK の更新、バグ修正、および新機能の完全な変更ログを表示します。

* **TypeScript SDK**: [CHANGELOG.md を表示](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [CHANGELOG.md を表示](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="report-bugs">
  バグの報告
</h2>

Agent SDK でバグまたは問題が発生した場合：

* **TypeScript SDK**: [GitHub で問題を報告](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [GitHub で問題を報告](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  ブランドガイドライン
</h2>

Claude Agent SDK を統合するパートナーの場合、Claude ブランドの使用はオプションです。製品で Claude を参照する場合：

**許可されています：**

* 「Claude Agent」（ドロップダウンメニューに推奨）
* 「Claude」（既に「Agents」というラベルが付いたメニュー内の場合）
* 「\{YourAgentName} Powered by Claude」（既存のエージェント名がある場合）

**許可されていません：**

* 「Claude Code」または「Claude Code Agent」
* Claude Code ブランドの ASCII アートまたは Claude Code を模倣する視覚要素

製品は独自のブランドを維持し、Claude Code または任意の Anthropic 製品のように見えるべきではありません。ブランドコンプライアンスに関する質問については、Anthropic [営業チーム](https://www.anthropic.com/contact-sales)に連絡してください。

<h2 id="license-and-terms">
  ライセンスと利用規約
</h2>

Claude Agent SDK の使用は、[Anthropic の商用利用規約](https://www.anthropic.com/legal/commercial-terms)によって管理されます。これは、Claude Agent SDK を使用して、独自のカスタマーおよびエンドユーザーに利用可能にする製品およびサービスを強化する場合を含みます。ただし、特定のコンポーネントまたは依存関係が、そのコンポーネントの LICENSE ファイルに示されているように異なるライセンスの対象である場合を除きます。

<h2 id="next-steps">
  次のステップ
</h2>

これらのリソースは、Agent SDK を使用して構築するための、より深い技術的詳細とサンプルプロジェクトをカバーしています。

* [クイックスタート](/docs/ja/agent-sdk/quickstart)：バグを見つけて修正する最初のエージェントを構築します
* [マイグレーションガイド](/docs/ja/agent-sdk/migration-guide)：Claude Code SDK パッケージから Agent SDK へマイグレーションします
* [エージェントループ](/docs/ja/agent-sdk/agent-loop)：Claude がどのように計画を立て、ツールを呼び出し、タスクが完了したかを判断するか
* [エージェントの例](https://github.com/anthropics/claude-agent-sdk-demos)：ローカル開発用のデモアプリ
* [TypeScript SDK](/docs/ja/agent-sdk/typescript)：完全な TypeScript API リファレンスと例
* [Python SDK](/docs/ja/agent-sdk/python)：完全な Python API リファレンスと例
* [エージェントハーネス設計](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code)：Claude Code チームが多くのサブエージェントを一度にオーケストレーションするために動的ワークフローをどのように使用するか
