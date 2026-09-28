> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code の仕組み

> agentic ループ、組み込みツール、Claude Code がプロジェクトとどのように相互作用するかを理解します。

Claude Code はターミナルで実行される agentic アシスタントです。コーディングに優れていますが、コマンドラインからできることなら何でも支援できます。ドキュメント作成、ビルド実行、ファイル検索、トピック調査など、様々なタスクに対応します。

このガイドでは、コアアーキテクチャ、組み込み機能、および [Claude Code を効果的に使用するためのヒント](#work-effectively-with-claude-code) について説明します。ステップバイステップのウォークスルーについては、[一般的なワークフロー](/docs/ja/common-workflows) を参照してください。スキル、MCP、フックなどの拡張機能については、[Claude Code を拡張する](/docs/ja/features-overview) を参照してください。

<h2 id="the-agentic-loop">
  agentic ループ
</h2>

Claude にタスクを与えると、3 つのフェーズを通じて作業します。**コンテキストの収集**、**アクションの実行**、**結果の検証** です。これらのフェーズは相互に融合します。Claude はツールを使用して、コードを理解するためのファイル検索、変更を加えるための編集、作業を確認するためのテスト実行など、様々な場面で活用します。

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agentic-loop.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=4a30fb7ce2815012a9f27c955e2c6bb0" className="dark:hidden" alt="agentic ループの図：プロンプトから Claude がコンテキストを収集し、アクションを実行し、結果を検証し、タスク完了まで繰り返します。任意の時点で中断できます。" width="720" height="280" data-path="images/agentic-loop.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agentic-loop-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=75e1d55ed76857a952f9a2dffbab02df" className="hidden dark:block" alt="agentic ループの図：プロンプトから Claude がコンテキストを収集し、アクションを実行し、結果を検証し、タスク完了まで繰り返します。任意の時点で中断できます。" width="720" height="280" data-path="images/agentic-loop-dark.svg" />

ループは、あなたが何を求めるかに応じて適応します。コードベースに関する質問は、コンテキスト収集だけで済むかもしれません。バグ修正は 3 つのフェーズすべてを繰り返し循環します。リファクタリングは広範な検証を伴うかもしれません。Claude は前のステップから学んだことに基づいて各ステップが何を必要とするかを判断し、数十のアクションを連鎖させ、途中で軌道修正します。

あなたもこのループの一部です。任意の時点で中断して Claude を別の方向に導いたり、追加のコンテキストを提供したり、別のアプローチを試すよう求めたりできます。Claude は自律的に動作しますが、あなたの入力に対して応答性を保ちます。

agentic ループは 2 つのコンポーネントによって駆動されます。推論する [モデル](#models) と、アクションを実行する [ツール](#tools) です。Claude Code はモデルの周りのレイヤーで、ツールを提供し、モデルが見るコンテキストを管理します。この周囲のレイヤーが agentic ハーネスという用語が指すものです。

<h3 id="models">
  モデル
</h3>

Claude Code は Claude モデルを使用して、コードを理解し、タスクについて推論します。Claude は任意の言語のコードを読み、コンポーネントがどのように接続されているかを理解し、目標を達成するために何を変更する必要があるかを判断できます。複雑なタスクの場合、作業をステップに分割し、実行し、学んだことに基づいて調整します。

[複数のモデル](/docs/ja/model-config) が異なるトレードオフで利用可能です。Sonnet はほとんどのコーディングタスクをうまく処理します。Opus は複雑なアーキテクチャ上の決定に対してより強力な推論を提供します。セッション中に `/model` で切り替えるか、`claude --model <name>` で開始します。

このガイドで「Claude が選択する」または「Claude が決定する」と言う場合、モデルが推論を行っています。

<h3 id="tools">
  ツール
</h3>

ツールは Claude Code を agentic にするものです。ツールがなければ、Claude はテキストで応答することしかできません。ツールがあれば、Claude はアクションを実行できます。コードを読み、ファイルを編集し、コマンドを実行し、ウェブを検索し、外部サービスと相互作用します。各ツール使用は情報を返し、ループにフィードバックされ、Claude の次の決定に情報を与えます。

組み込みツールは一般的に 5 つのカテゴリに分類され、それぞれが異なる種類のエージェンシーを表します。

| カテゴリ             | Claude ができること                                                                         |
| ---------------- | ------------------------------------------------------------------------------------- |
| **ファイル操作**       | ファイルの読み取り、コード編集、新規ファイル作成、名前変更と再編成                                                     |
| **検索**           | パターンでファイルを検索、正規表現でコンテンツを検索、コードベースを探索                                                  |
| **実行**           | シェルコマンド実行、サーバー起動、テスト実行、git 使用                                                         |
| **ウェブ**          | ウェブ検索、ドキュメント取得、エラーメッセージ検索                                                             |
| **コード インテリジェンス** | 編集後の型エラーと警告を表示、定義にジャンプ、参照を検索（[コード インテリジェンス プラグイン](/docs/ja/plugins/code-intelligence) が必要） |

これらが主な機能です。Claude には subagent の生成、質問、その他のオーケストレーションタスク用のツールもあります。完全なリストについては、[Claude が利用可能なツール](/docs/ja/tools-reference) を参照してください。

Claude はプロンプトと学んだことに基づいて、どのツールを使用するかを選択します。「失敗しているテストを修正して」と言うと、Claude は以下のようなことを行うかもしれません。

1. テストスイートを実行して、何が失敗しているかを確認
2. エラー出力を読む
3. 関連するソースファイルを検索
4. それらのファイルを読んでコードを理解
5. ファイルを編集して問題を修正
6. テストを再度実行して検証

各ツール使用は Claude に新しい情報を与え、次のステップに情報を与えます。これが agentic ループの実際の動作です。

**基本機能の拡張：** 組み込みツールが基盤です。[スキル](/docs/ja/skills) で Claude が知ることを拡張し、[MCP](/docs/ja/mcp) で外部サービスに接続し、[フック](/docs/ja/hooks) でワークフローを自動化し、[subagent](/docs/ja/sub-agents) にタスクをオフロードできます。これらの拡張は、コア agentic ループの上に層を形成します。ニーズに合った拡張を選択するためのガイダンスについては、[Claude Code を拡張する](/docs/ja/features-overview) を参照してください。

<h2 id="what-claude-can-access">
  Claude がアクセスできるもの
</h2>

ディレクトリで `claude` を実行すると、Claude Code は以下にアクセスできます。

* **プロジェクト。** ディレクトリとサブディレクトリ内のファイル、および許可を得た他の場所のファイル。
* **ターミナル。** 実行できるあらゆるコマンド。ビルドツール、git、パッケージマネージャー、システムユーティリティ、スクリプト。コマンドラインからできることなら、Claude もできます。
* **git の状態。** 現在のブランチ、コミットされていない変更、最近のコミット履歴。
* **[CLAUDE.md](/docs/ja/memory)。** プロジェクト固有の指示、規約、Claude が毎回のセッションで知っておくべきコンテキストを保存するマークダウンファイル。リポジトリに他のコーディングエージェント用の AGENTS.md がある場合、Claude は [それを読むことができます](/docs/ja/memory#agents-md)。CLAUDE.md と一緒に、または単独で読むことができます。
* **[自動メモリ](/docs/ja/memory#auto-memory)。** 作業中に Claude が自動的に保存する学習。設定など。MEMORY.md の最初の 200 行または 25KB のいずれか先に達した方が、各セッションの開始時に読み込まれます。
* **設定した拡張機能。** 外部サービス用の [MCP サーバー](/docs/ja/mcp)、ワークフロー用の [skills](/docs/ja/skills)、委譲作業用の [subagents](/docs/ja/sub-agents)、ブラウザ相互作用用の [Claude in Chrome](/docs/ja/chrome)。

Claude はプロジェクト全体を見ることができるため、プロジェクト全体で作業できます。「認証バグを修正して」と Claude に求めると、関連ファイルを検索し、複数のファイルを読んでコンテキストを理解し、それらを横断して調整された編集を行い、テストを実行して修正を検証し、求めればコミットします。これは現在のファイルのみを見るインラインコードアシスタントとは異なります。

<h2 id="environments-and-interfaces">
  環境とインターフェース
</h2>

[agentic ループ](#the-agentic-loop)、[ツール](#tools)、および機能は、Claude Code を使用するあらゆる場所で同じです。変わるのは、コードが実行される場所とそれとの相互作用方法です。

<h3 id="execution-environments">
  実行環境
</h3>

Claude Code は 3 つの環境で実行され、各環境はコード実行場所に対して異なるトレードオフを持ちます。

| 環境             | コード実行場所                                                             | ユースケース                        |
| -------------- | ------------------------------------------------------------------- | ----------------------------- |
| **ローカル**       | マシン                                                                 | デフォルト。ファイル、ツール、環境への完全なアクセス    |
| **クラウド**       | Anthropic 管理 VM、または [セルフホスト環境](/docs/ja/self-hosted-environments)（組織が運用） | タスクをオフロード、ローカルにないリポジトリで作業     |
| **リモートコントロール** | マシン、ブラウザから制御                                                        | ウェブ UI を使用しながら実行とファイルをローカルに保つ |

<h3 id="interfaces">
  インターフェース
</h3>

Claude Code には、ターミナル、[デスクトップアプリ](/docs/ja/desktop)、[IDE 拡張機能](/docs/ja/vs-code)、[claude.ai/code](https://claude.ai/code)、[リモートコントロール](/docs/ja/remote-control)、[Slack](/docs/ja/slack)、[CI/CD パイプライン](/docs/ja/github-actions) を通じてアクセスできます。インターフェースは Claude の表示方法と相互作用方法を決定しますが、基盤となる agentic ループは同じです。完全なリストについては、[Claude Code をどこでも使用する](/docs/ja/overview#use-claude-code-everywhere) を参照してください。

<h2 id="work-with-sessions">
  セッションで作業する
</h2>

Claude Code は作業中にローカルで会話を保存します。各メッセージ、ツール使用、結果は `~/.claude/projects/` の下のプレーンテキスト JSONL ファイルに書き込まれ、[巻き戻し](#undo-changes-with-checkpoints)、[再開、フォーク](#resume-or-fork-sessions) セッションが可能になります。Claude がコード変更を行う前に、影響を受けるファイルのスナップショットも作成されるため、必要に応じて元に戻すことができます。パス、保持期間、このデータをクリアする方法については、[`~/.claude` のアプリケーションデータ](/docs/ja/claude-directory#application-data) を参照してください。

**セッションは独立しています。** 各新規セッションは、前のセッションの会話履歴なしで、新しいコンテキストウィンドウで開始されます。Claude は [自動メモリ](/docs/ja/memory#auto-memory) を使用してセッション間で学習を保持でき、[CLAUDE.md](/docs/ja/memory) に独自の永続的な指示を追加できます。

<h3 id="work-across-branches">
  ブランチ間で作業する
</h3>

各 Claude Code 会話は、現在のディレクトリに結び付けられたセッションです。`/resume` ピッカーはデフォルトで現在の worktree からのセッションを表示し、キーボードショートカットを使用してリストを他の worktree またはプロジェクトに拡張できます。ピッカーショートカットの完全なリストと名前解決の仕組みについては、[セッションを管理する](/docs/ja/sessions#use-the-session-picker) を参照してください。

Claude は現在のブランチのファイルを見ます。ブランチを切り替えると、Claude は新しいブランチのファイルを見ますが、会話履歴は同じままです。Claude はブランチ切り替え後も、議論したことを覚えています。

セッションはディレクトリに結び付けられているため、[git worktrees](/docs/ja/worktrees) を使用して並列 Claude Code セッションを実行できます。これは個別のブランチ用に別のディレクトリを作成します。

<h3 id="resume-or-fork-sessions">
  セッションを再開またはフォークする
</h3>

`claude --continue` または `claude --resume` でセッションを再開すると、同じセッション ID を使用して中断したところから再開し、新しいメッセージを既存の会話に追加します。`--fork-session` または `/branch` でフォークすると、履歴を新しいセッション ID にコピーし、元のセッションは変更されません。

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/session-continuity.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=04ed0984a58e4127e05b3640265241a3" className="dark:hidden" alt="セッション継続性の図：再開は同じセッションを続行し、フォークは新しい ID で新しいブランチを作成します。" width="560" height="280" data-path="images/session-continuity.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/session-continuity-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=886a384bce8298594e43f124617ea665" className="hidden dark:block" alt="セッション継続性の図：再開は同じセッションを続行し、フォークは新しい ID で新しいブランチを作成します。" width="560" height="280" data-path="images/session-continuity-dark.svg" />

再開フラグ、`/resume` ピッカー、命名、同じセッションが 2 つのターミナルで開いている場合の動作については、[セッションを管理する](/docs/ja/sessions) を参照してください。

<h3 id="the-context-window">
  コンテキストウィンドウ
</h3>

Claude のコンテキストウィンドウは、会話履歴、ファイルコンテンツ、コマンド出力、[CLAUDE.md](/docs/ja/memory)、[自動メモリ](/docs/ja/memory#auto-memory)、読み込まれたスキル、システム指示を保持します。作業を進めると、コンテキストが満杯になります。Claude は自動的にコンパクト化しますが、会話の早い段階からの指示が失われる可能性があります。永続的なルールを CLAUDE.md に入れ、`/context` を実行してスペースを使用しているものを確認してください。

対話的なウォークスルーについては、[コンテキストウィンドウを探索する](/docs/ja/context-window) を参照してください。

<h4 id="when-context-fills-up">
  コンテキストが満杯になったとき
</h4>

Claude Code はコンテキストウィンドウの制限に近づくと、自動的にコンテキストを管理します。古いツール出力をクリアし、必要に応じて会話を要約します。リクエストと主要なコードスニペットは保持されます。会話の早い段階からの詳細な指示は失われる可能性があります。会話履歴に依存するのではなく、永続的なルールを CLAUDE.md に入れてください。

コンパクト化中に保持されるものを制御するには、CLAUDE.md に「Compact Instructions」セクションを追加するか、`/compact` をフォーカス付きで実行します（例：`/compact focus on the API changes`）。

単一のファイルまたはツール出力が非常に大きく、各要約後にコンテキストがすぐに再度満杯になる場合、Claude Code は数回の試行後に自動コンパクト化を停止し、ループする代わりにエラーを表示します。[自動コンパクト化が thrashing エラーで停止する](/docs/ja/troubleshooting#auto-compaction-stops-with-a-thrashing-error) を参照して、復旧手順を確認してください。

`/context` を実行してスペースを使用しているものを確認してください。MCP ツール定義はデフォルトで遅延され、[ツール検索](/docs/ja/mcp#scale-with-mcp-tool-search) を通じてオンデマンドで読み込まれるため、Claude が特定のツールを使用するまで、ツール名とサーバー指示のみがコンテキストを消費します。

<h4 id="manage-context-with-skills-and-subagents">
  スキルと subagent でコンテキストを管理する
</h4>

コンパクト化を超えて、他の機能を使用してコンテキストに読み込まれるものを制御できます。

[スキル](/docs/ja/skills) はオンデマンドで読み込まれます。Claude はセッション開始時にスキル説明を見ますが、完全なコンテンツはスキルが使用されるときのみ読み込まれます。手動で呼び出すスキルの場合、`disable-model-invocation: true` を設定して、必要になるまで説明をコンテキストから除外します。自分で書いていないスキルの場合、[`skillOverrides`](/docs/ja/skills#override-skill-visibility-from-settings) を使用して設定から同じことを行います。

[Subagent](/docs/ja/sub-agents) は独自のコンテキストウィンドウで動作します。Subagent は、[フォーク](/docs/ja/sub-agents#fork-the-current-conversation) でない限り新規に開始されます。フォークの場合は、これまでの会話のコピーで開始されます。どちらの場合でも、subagent のツール呼び出しはコンテキストから除外され、Claude は subagent が完了したときに要約を取得します。

各機能のコストについては [コンテキストコスト](/docs/ja/features-overview#understand-context-costs) を参照し、コンテキスト管理のヒントについては [トークン使用量を削減する](/docs/ja/costs#reduce-token-usage) を参照してください。

<h2 id="stay-safe-with-checkpoints-and-permissions">
  チェックポイントと権限で安全に保つ
</h2>

Claude には 2 つの安全メカニズムがあります。チェックポイントはファイル変更を元に戻すことができ、権限は Claude ができることを制御します。

<h3 id="undo-changes-with-checkpoints">
  チェックポイントで変更を元に戻す
</h3>

**ファイル編集はすべて可逆的です。** Claude がファイルを編集する前に、現在のコンテンツのスナップショットを作成します。何か問題が発生した場合、`Esc` を 2 回押して前の状態に巻き戻すか、Claude に元に戻すよう求めてください。

チェックポイントは git とは別であり、会話を再開するときに利用可能なままです。ファイル変更のみをカバーし、復元は [シンボリックリンクとハードリンクされたファイルをスキップします](/docs/ja/checkpointing#symlinked-and-hard-linked-paths-not-restored)。リモートシステム（データベース、API、デプロイメント）に影響するアクションはチェックポイントできません。これらは権限モードと権限ルールで制御します。

<h3 id="control-what-claude-can-do">
  Claude ができることを制御する
</h3>

権限モードを選択して、Claude が求めずにできることを設定します。`Shift+Tab` を押して権限モードをサイクルします。

* **Auto**：分類器がバックグラウンドでほとんどのアクションをレビューし、求める代わりにリスクのあるものをブロックします。Pro、Max、Team プランでは、インタラクティブターミナルと VS Code セッションの [組み込みの開始権限モード](/docs/ja/permission-modes#which-mode-a-session-starts-in) です
* **Manual**：Claude はファイル編集とシェルコマンドの前に求めます
* **Accept edits**：Claude はファイルを編集し、`mkdir` や `mv` などの一般的なファイルシステムコマンドを実行するよう求めず、他のコマンドはまだ求めます
* **Plan**：Claude はソースファイルを編集せずに探索し、プランを提案します

`.claude/settings.json` で特定のコマンドを許可することもできます。これにより、Claude は毎回求めません。これは `npm test` や `git status` などの信頼できるコマンドに便利です。設定は組織全体のポリシーから個人的な設定までスコープできます。詳細については、[権限](/docs/ja/permissions) を参照してください。

***

<h2 id="work-effectively-with-claude-code">
  Claude Code を効果的に使用する
</h2>

これらのヒントは Claude Code からより良い結果を得るのに役立ちます。特定のプロンプト、検証、計画の詳細については、[ベストプラクティス](/docs/ja/best-practices)を参照してください。

<h3 id="ask-claude-code-for-help">
  Claude Code に助言を求める
</h3>

Claude Code はそれの使い方を教えることができます。「hooks をどのようにセットアップするのか」や「CLAUDE.md を構造化する最良の方法は何か」といった質問をしてください。Claude が説明します。

組み込みコマンドもセットアップをガイドします：

* `/init` はプロジェクト用のスターター CLAUDE.md を生成します
* `/doctor` はセットアップチェックアップを実行し、インストールと設定の問題を診断し、修正することができます

<h3 id="it’s-a-conversation">
  これは会話です
</h3>

Claude Code は会話型です。完璧なプロンプトは必要ありません。やりたいことから始めて、その後改善します：

```text theme={null}
Fix the login bug
```

\[Claude が調査し、何かを試す]

```text theme={null}
That's not quite right. The issue is in the session handling.
```

\[Claude がアプローチを調整する]

最初の試みが正しくない場合、最初からやり直す必要はありません。反復します。

<h4 id="interrupt-and-steer">
  割り込みと操舵
</h4>

任意の時点で Claude をリダイレクトでき、最初からやり直す必要はありません。以下のいずれかを実行してください：

* **`Esc` キーを押す**と Claude が直ちに停止します。実行中のツール呼び出しがキャンセルされ、Claude は次の指示を待ちます。メッセージがキューに入っている場合、Claude Code は[次にそれらを送信します](/docs/ja/interactive-mode#queue-messages-while-claude-works)。
* **修正を入力して `Enter` キーを押す**と Claude を停止せずに実行します。メッセージは入力ボックスの上にキューに入っているものとして表示されます。Claude がツール呼び出しを実行している場合、それらの呼び出しが完了するとすぐに、同じターン内でメッセージを読み取り、次のステップの前に調整します。[Claude が作業している間にメッセージをキューに入れる](/docs/ja/interactive-mode#queue-messages-while-claude-works)は、他のキューに入っているエントリがいつ送信されるかについて説明しています。

<h3 id="delegate-don’t-dictate">
  指示するのではなく委譲する
</h3>

有能な同僚に委譲することを考えてください。コンテキストと方向性を提供し、その後 Claude が詳細を理解することを信頼してください：

```text theme={null}
The checkout flow is broken for users with expired cards.
The relevant code is in src/payments/. Can you investigate and fix it?
```

どのファイルを読むか、どのコマンドを実行するかを指定する必要はありません。Claude がそれを理解します。

<h2 id="what’s-next">
  次のステップ
</h2>

<CardGroup cols={2}>
  <Card title="機能で拡張する" icon="puzzle-piece" href="/docs/ja/features-overview">
    スキルと MCP 接続を追加
  </Card>

  <Card title="一般的なワークフロー" icon="graduation-cap" href="/docs/ja/common-workflows">
    典型的なタスクのステップバイステップガイド
  </Card>
</CardGroup>
