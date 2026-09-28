> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 出力スタイル

> Concise や Explanatory などの組み込み出力スタイルを使用するか、カスタムスタイルを作成して、Claude Code のロール、トーン、応答形式を変更します。

出力スタイルは、セッション内のすべての応答に対して Claude のロール、トーン、応答形式を設定する一連の指示です。Claude Code には、デフォルトの他に 4 つの組み込みスタイルが含まれており、独自のスタイルを作成することもできます。

出力スタイルを使用すると、セッション全体で Claude がどのように応答し、あなたと協力するかを変更できるため、各プロンプトでリクエストを繰り返す必要がありません。たとえば、組み込みスタイルは応答をより短くしたり、各変更の説明を追加したり、Claude が定型的な質問をせずに作業を開始したりできます。カスタムスタイルは、Claude をソフトウェアエンジニア以外のもの（ライティングアシスタントやデータアナリストなど）に変えることもできます。

* 組み込みスタイルを使用するには、[組み込み出力スタイル](#built-in-output-styles)から 1 つを選択して、[それに切り替えます](#change-your-output-style)。
* 独自の指示を作成するには、[カスタム出力スタイルを作成します](#create-a-custom-output-style)。

<Note>
  出力スタイルは Claude に従うべき指示を与えます。何かが常に起こることや決して起こらないことを保証するものではありません。一部のニーズは異なる機能に適しています。

  * Claude がプロジェクトについて知っておくべきことについては、[CLAUDE.md](/docs/ja/memory) を使用します。
  * 各編集後のフォーマットやコマンドのブロックなど、毎回必ず起こる必要があることについては、[フック](/docs/ja/hooks-guide)を使用します。
  * スキル、サブエージェント、その他のオプションについては、[出力スタイルと他の機能の選択](#choose-between-an-output-style-and-other-features)を参照してください。
</Note>

<h2 id="built-in-output-styles">
  組み込み出力スタイル
</h2>

Claude Code は [**Default**](#default) スタイルで開始します。これはソフトウェアエンジニアリングタスクを完了するための標準的な指示です。他の 4 つの組み込みスタイルはこれらの指示を保持し、独自の指示を追加します。

このテーブルは各スタイルがセッションについて何を変更するか、そしていつそれが適切かを示しています。

| スタイル                        | 変更内容                                                  | 使用する場合                                           |
| :-------------------------- | :---------------------------------------------------- | :----------------------------------------------- |
| [Proactive](#proactive)     | Claude は即座に作業を開始し、日常的な決定について質問する代わりに合理的な仮定を立てます       | Claude に日常的な決定を通じて作業を続けさせたい場合、仮定が間違っていれば方向を修正します |
| [Concise](#concise)         | レスポンスは結果で始まり、前置き、説明、および要約を省略します                       | デフォルトのレスポンスが想定より長い場合                             |
| [Explanatory](#explanatory) | Claude は、記述したコードの背後にある選択肢を説明する短い `Insight` ブロックを追加します | コードベースを学習中、または変更と一緒に推論を望む場合                      |
| [Learning](#learning)       | Claude は選択肢を説明し、自分で記述するための小さなコード片を残します                | タスクがまだ完了している間、実践的なコーディング練習を望む場合                  |

<h3 id="default">
  Default
</h3>

Default は出力スタイルが選択されていないことを意味します。Claude Code はスタイル指示を追加せず、Claude は Claude Code の標準システムプロンプトから動作します。これはソフトウェアエンジニアリングタスク用に記述されています。

`default` は他のスタイルと共に `/output-style` リストに表示されるため、[同じ方法で選択](#change-your-output-style)します。

<h3 id="proactive">
  Proactive
</h3>

Proactive スタイルでは、Claude はタスクを送信するとすぐに実装を開始します。日常的な決定について合理的な仮定を立て、質問するために停止することはなく、計画を求めない限り計画モードに切り替わりません。任意の時点でそれをリダイレクトできます。

スタイルの指示は、データを削除するか、共有またはプロダクションシステムを変更するアクションの前に、Claude がカンバセーション内でユーザーに確認するよう指示します。その確認は Claude が従う指示であり、権限プロンプトとは別です。

Proactive スタイルに切り替えても、[権限モード](/docs/ja/permission-modes)は変わりません。権限モードは依然としてユーザーに尋ねずに実行するツール呼び出しを決定するため、権限プロンプトは切り替え前と同じ方法で表示されます。

<h3 id="concise">
  Concise
</h3>

Concise スタイルでは、レスポンスの最初の文が何が起こったか、または答えが何かを述べています。Claude は導入部、ステップバイステップの説明、および終了時の要約を省略し、簡単な質問に 1 ～ 3 文で答えます。Default スタイルと同じくらい徹底的にエンジニアリング作業を行います。Claude Code v2.1.237 以降が必要です。

Claude は以下の場合でも完全な長さで記述します。

* **ユーザーが求めるもの**: 説明または詳細情報を求めると、Claude は完全に答えます。
* **安全に行動するために必要なもの**: エラーレポート、失敗したテスト出力、セキュリティ警告、および破壊的なアクションの確認は完全な内容を保持します。

<h3 id="explanatory">
  Explanatory
</h3>

Explanatory スタイルでは、Claude は Default スタイルと同じ方法でタスクを実行し、立てた選択肢の理由についての短い説明を追加します。各説明はカンバセーションに表示され、それが関連するコードの前または後に、`Insight` というラベルの付いたブロックに表示されます。説明はファイルにコメントとして記述されません。

`Insight` ブロックは、コードベースまたは Claude が記述したコードについて 2 ～ 3 つのポイントを含みます。例えば、API エンドポイントを追加した後のこのようなものです。

```text theme={null}
★ Insight ─────────────────────────────────────
- このリポジトリのすべてのルートは withAuth ラッパーを通過するため、新しいエンドポイントは独自のミドルウェアなしでセッションチェックを取得します。
- レート制限は limits.ts でルートごとに設定されるため、この変更はグローバルデフォルトではなく、そこにエントリを追加します。
─────────────────────────────────────────────────
```

<h3 id="learning">
  Learning
</h3>

Learning スタイルでは、Claude は [Explanatory スタイル](#explanatory)と同じ `Insight` ブロックを追加し、ユーザーにコードの一部を記述するよう求めます。Claude は日常的な実装を自分で処理します。エラーハンドリング、データ構造、または複数の有効なアプローチを持つビジネスロジックなど、実際の設計決定を伴う部分に到達すると、ユーザーのために数行を残します。

Claude は `TODO(human)` コメントでファイル内の場所をマークし、既に構築されたもの、記述するもの、および検討すべきことを述べるリクエストを送信します。

```text theme={null}
● Learn by Doing

Context: アップロードフォームが配置され、ファイルを受け入れる前に validateFile() を呼び出します。サイズとタイプのチェックは画像に対して機能しますが、switch ステートメントにはドキュメントの処理がまだありません。

Your Task: upload.js で、validateFile() 内の case "document" ブランチを実装します。TODO(human) を探します。

Guidance: ドキュメントのサイズ制限を決定し、ファイル拡張子が MIME タイプと一致する必要があるかどうかを決定します。{valid: boolean, error?: string} を返します。
```

Claude はその後停止し、待機します。`TODO(human)` コメントでコードを記述し、完了したことを Claude に伝えます。Claude はユーザーのコードについて 1 つの `Insight` で応答し、タスクを続行します。

<h2 id="change-your-output-style">
  出力スタイルを変更する
</h2>

コマンド、メニュー、または設定ファイルでスタイルを選択します。コマンドと両方のメニューは、選択内容を [ローカルプロジェクトレベル](/docs/ja/settings) の `.claude/settings.local.json` に保存します。

* **`/output-style` コマンド**: `/output-style <style>` を実行してスタイルを切り替えます。例えば `/output-style concise` のように使用します。引数なしで実行すると、選択可能なスタイルが一覧表示され、現在のスタイルがマークされます。

  このコマンドは [非対話型モード](/docs/ja/headless) と Agent SDK セッション、およびモバイルアプリまたは Web から [リモートコントロール](/docs/ja/remote-control#limitations) 経由でも機能します。この場合、[組み込みスタイル](#built-in-output-styles) のみをリストして選択できます。Claude Code v2.1.269 以降が必要です。
* **Terminal メニュー**: `/config` を実行し、**Output style** を選択してメニューからスタイルを選択します。
* **VS Code extension**: [コマンドメニュー](/docs/ja/vs-code#use-the-prompt-box) を `/` で開き、**Output styles** を選択してスタイルを選択します。カスタムスタイルも含まれます。Claude Code v2.1.257 以降が必要です。
* **Desktop app**: 設定ファイル（例えば `.claude/settings.local.json`、ターミナルメニューが書き込むファイル）の `outputStyle` フィールドを設定します。`/config` を実行すると、Claude Code は [**Settings > Claude Code**](/docs/ja/desktop#what%E2%80%99s-not-available-in-desktop) を開きます。メニューではなく設定画面が開きます。

メニューなしでスタイルを設定するには、設定ファイルの `outputStyle` フィールドを直接編集します。

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

値は大文字と小文字を区別するため、組み込み名は `Proactive`、`Concise`、`Explanatory`、`Learning` と記述してください。`explanatory` のようにスタイル名と完全に一致しない値を指定すると、デフォルトスタイルが適用されます。`/output-style` コマンドは大文字と小文字を区別しません。

プロジェクト全体でスタイルをデフォルトにするには、`~/.claude/settings.json` で `outputStyle` を設定します。プロジェクト独自の設定ファイルは [その値より優先されます](/docs/ja/settings#settings-precedence)。

セッション中にスタイルを切り替えると、Claude は次のメッセージから新しいスタイルを使用します。最初のメッセージのプロンプトキャッシングのコストについては、[出力スタイルの変更](/docs/ja/prompt-caching#changing-output-style) を参照してください。v2.1.251 より前は、新しいスタイルは `/clear` を実行するか新しいセッションを開始した後にのみ適用されました。

<h2 id="create-a-custom-output-style">
  カスタム出力スタイルを作成する
</h2>

カスタム出力スタイルは Markdown ファイルです。メタデータ用の frontmatter、その後に Claude の指示が続きます。

VS Code 拡張機能では、手書きではなく [**Output styles** メニュー](/docs/ja/vs-code#use-the-prompt-box) からファイルを作成することもできます。これには Claude Code v2.1.261 以降が必要です。

<Steps>
  <Step title="Markdown ファイルを作成する">
    3 つのレベルのいずれかに保存します。ファイル名がスタイル名になります。frontmatter で `name` を設定しない限り。

    * ユーザー: `~/.claude/output-styles`
    * プロジェクト: `.claude/output-styles`
    * 管理ポリシー: [管理設定ディレクトリ](/docs/ja/managed-settings#delivery-mechanisms) 内の `.claude/output-styles`

    プロジェクト出力スタイルは、作業ディレクトリとリポジトリルートの間のすべての `.claude/output-styles/` から読み込まれます。これらのネストされたディレクトリの複数が同じ名前のスタイルを定義する場合、Claude Code は作業ディレクトリに最も近いものを使用します。
  </Step>

  <Step title="Frontmatter と指示を追加する">
    Claude Code のソフトウェアエンジニアリング指示を保持するかどうかを決定します。Claude がコミュニケーション方法を変更しているがまだ同じ方法でコーディングしたい場合は `keep-coding-instructions: true` を設定します。Claude がソフトウェアエンジニアリングを行わない場合は除外します。

    この例は Claude のコーディング動作を保持しながら、すべての説明を図で始めます。

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="スタイルに切り替える">
    ターミナルで `/output-style <style>` を実行するか、`/config` を実行して **Output style** でスタイルを選択します。Claude は次のメッセージから新しいスタイルを使用します。ターミナルでは、Claude Code はスタイルファイルを起動時に読み込むため、実行中のセッション中に作成または編集した場合は、Claude Code を再起動して変更を反映させてください。
  </Step>
</Steps>

[プラグイン](/docs/ja/plugins/manifest-reference) は `output-styles/` ディレクトリで出力スタイルを配布することもできます。

<h3 id="frontmatter">
  Frontmatter リファレンス
</h3>

出力スタイルを YAML [frontmatter](/docs/ja/glossary#frontmatter) で設定します。ファイルの上部の `---` マーカーの間に記述します。すべてのフィールドはオプションであり、フィールド名はハイフンで区切られた小文字の単語を使用します。スペルが間違ったフィールドはエラーなしで無視されます。YAML が解析できない場合、スタイルはファイル名で読み込まれ、フィールドは設定されません。`claude --debug` を実行して解析エラーを確認してください。

| フィールド                      | 必須  | 説明                                                                                                                                                                              |
| :------------------------- | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`                     | いいえ | 出力スタイルの名前。`/config` ピッカーに表示されます。デフォルト: ファイル名                                                                                                                                    |
| `description`              | いいえ | 出力スタイルの説明。`/config` ピッカーに表示されます                                                                                                                                                 |
| `keep-coding-instructions` | いいえ | `true` に設定すると、Claude Code の組み込みソフトウェアエンジニアリング指示をスタイルと一緒に保持します。デフォルト: `false`                                                                                                    |
| `force-for-plugin`         | いいえ | プラグイン出力スタイルのみ。`true` に設定すると、プラグインが有効になるたびに、ユーザーが選択する必要なく、このスタイルを自動的に適用します。ユーザーの `outputStyle` 設定をオーバーライドします。複数の有効なプラグインがこれを設定する場合、Claude Code は最初に読み込まれたものを使用します。デフォルト: `false` |

<span id="comparisons-to-related-features" />

<h2 id="choose-between-an-output-style-and-other-features">
  出力スタイルと他の機能の選択
</h2>

出力スタイルはセッション内のすべての応答に適用されます。これは Claude が従う指示であるため、何も強制しません。必要なものがすべての応答より狭い範囲である場合、または確実に発生する必要がある場合は、別の機能がより適切です。

この表は、必要なものを、それを実現する機能に対応させています。

| 必要なもの                                     | 使用する機能                                                            | 適切な理由                                                        |
| :---------------------------------------- | :---------------------------------------------------------------- | :----------------------------------------------------------- |
| 特定の声、長さ、または形式のすべての応答、または異なるロールの Claude    | 出力スタイル                                                            | セッション全体に適用され、1 つのコマンドでスタイルを切り替えることができます                      |
| Claude がプロジェクトの規約、コマンド、および構造を知っている        | [CLAUDE.md](/docs/ja/memory)                                           | Claude が知るべきコードベースについての情報を保持し、選択したスタイルに関係なく読み込まれたままになります     |
| リリースチェックリストやレビュー手順など、1 種類のタスクの指示          | [skill](/docs/ja/skills)                                               | Claude はそれを呼び出すか、タスクが一致する場合にのみ読み込むため、関連のない応答を形成しません          |
| 各編集後のフォーマットやコマンドのブロックなど、例外なく毎回発生する必要があるもの | [hook](/docs/ja/hooks-guide)                                           | Claude Code はライフサイクルイベントでフック自体を実行するため、Claude が指示に従うことに依存しません |
| 焦点を絞ったタスク用に独自の指示、モデル、およびツールを備えたヘルパー       | [subagent](/docs/ja/sub-agents)                                        | 独自のシステムプロンプトを備えた別のコンテキストで実行され、会話に概要を返します                     |
| Claude Code を開始するときに渡す Claude の指示への追加     | [`--append-system-prompt`](/docs/ja/cli-reference#system-prompt-flags) | 何も削除せずにシステムプロンプトに追加します                                       |

これらの機能は組み合わせることができます。たとえば、Claude が知るべきことに CLAUDE.md を使用し、応答方法に出力スタイルを使用し、保証する必要があるものに hook を使用できます。[Claude Code を拡張](/docs/ja/features-overview)すると、残りの拡張機能が比較されます。

<h2 id="how-output-styles-work">
  出力スタイルの仕組み
</h2>

出力スタイルは Claude Code が Claude に与える指示を変更します。

* Claude Code はすべてのリクエストで有効なスタイルの指示を送信します。
* カスタム出力スタイルは、`keep-coding-instructions` が `true` に設定されていない限り、変更のスコープ、コメントの書き方、作業の検証方法など、Claude Code の組み込みソフトウェアエンジニアリング指示を除外します。

出力スタイルはメイン会話と [フォーク](/docs/ja/sub-agents#fork-the-current-conversation) に適用されます。フォークは親の完全な会話とシステムプロンプトを継承します。その他の [サブエージェントは独自のシステムプロンプトを実行](/docs/ja/sub-agents#what-loads-at-startup) するため、スタイルはそれらの応答方法を変更しません。

トークン使用量はスタイルによって異なります。スタイルの指示は入力トークンを追加しますが、プロンプトキャッシングはセッション内の最初のリクエスト後にこのコストを削減します。

組み込みの Explanatory および Learning スタイルは、設計上 Default よりも長い応答を生成し、これは出力トークンを増加させます。Concise スタイルはその逆で、Claude にデフォルトで応答を短く保つよう指示することで反対の効果を生じます。カスタムスタイルの場合、出力トークン使用量は、指示が Claude に何を生成するよう指示するかによって異なります。

<h2 id="related-resources">
  関連リソース
</h2>

* [設定](/docs/ja/settings): `outputStyle` フィールドが存在する場所と設定の優先順位の仕組み
* [権限モード](/docs/ja/permission-modes): Proactive スタイルがオートモードとどのように比較されるか
* [プラグイン](/docs/ja/plugins/overview): スキル、フック、エージェントと一緒に出力スタイルをパッケージ化して配布する
* [設定をデバッグする](/docs/ja/debug-your-config): 出力スタイルが有効にならない理由を診断する
