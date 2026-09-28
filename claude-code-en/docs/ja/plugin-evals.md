> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# evals でプラグインをテストする

> Claude Code プラグイン用の eval ケースを作成し、claude plugin eval で実行し、結果をグレード化し、プラグインなしのベースラインと比較し、CI でスコアをゲートする。

`claude plugin eval` シェルコマンドは [プラグイン](/docs/ja/plugins/overview) をテストケースのスイートに対して実行し、結果をスコア化します。各ケースは現実的なプロンプトと 1 つ以上のグレーダーで構成されます。グレーダーは Claude が生成したものに対する合格/不合格チェックで、返信に対する正規表現、特定のツールが呼び出されたかどうか、または第 2 のモデルが返信を判定するルーブリックなどです。

スイートを手動で作成する必要はありません。`claude plugin eval init` はプラグインについて質問し、ケースとグレーダーを提案し、それらを試し、ファイルを作成します。既に開いているセッションから Claude に同じことを行うよう依頼することもできます。

evals を使用して以下を実行します。

* プラグインがどの程度確実に Claude を正しい結果に導くかを測定する
* プラグインを変更したり新しいモデルがリリースされたりしたときの回帰を検出する
* プラグインなしの場合と比較してプラグインが何を貢献しているかを確認する

このページはプラグインとスキル作成者向けで、動作するプラグインがあり、その動作をテストしたい場合、および CI でプラグイン変更をゲートするチーム向けです。そのケース形式は [skill-creator プラグイン](/docs/ja/skills#run-evals-with-skill-creator) が使用する `evals/evals.json` ファイルとは別です。プラグインを作成するには [プラグインを作成する](/docs/ja/plugins/create) を参照してください。プラグインの動作ではなく構文とスキーマエラーをチェックするには、[`claude plugin validate`](/docs/ja/plugins/cli-reference#plugin-validate) を使用します。

<Note>
  すべての eval 実行とすべてのジャッジグレーダーは、アカウント上の実際のモデル呼び出しで、プランの使用量または API 請求に対してカウントされます。そのため、最初に [要件](#requirements) を確認してください。その後、[最初の eval スイートを作成](#create-your-first-eval-suite) するか、既にスイートがある場合は [CI で evals を実行](#run-evals-in-ci) に進んでください。
</Note>

<h2 id="requirements">
  要件
</h2>

プラグイン evals を実行するには、以下が必要です。

* Claude Code v2.1.269 以降。`claude --version` で確認し、`claude update` でアップグレードしてください。
* `plugin.json` または `.claude-plugin/plugin.json` マニフェストを含むプラグインディレクトリ、または [skills-directory プラグイン](/docs/ja/plugins/loading#plugins-shared-through-a-repository)。
* 通常の Claude Code セッションで使用するのと同じ認証とモデルプロバイダー。Eval 実行、judge-scored graders、および `claude plugin eval init` はあなたの認証情報でモデルを呼び出すため、プランの使用量制限または API 請求に対してカウントされます。コマンドがコストを報告する場合、その数値はそれらの呼び出しの [定価見積もり](/docs/ja/costs) です。

<h2 id="how-an-eval-run-works">
  eval 実行の仕組み
</h2>

eval スイートはプラグイン内の `evals/` というディレクトリに存在し、[ケースを作成して改善する](#write-and-refine-cases) に示すようにレイアウトされます。各ケースは [プロンプト](#set-run-limits-and-tools-in-prompt-md) と 1 つ以上の [グレーダー](#grade-the-result) を含む独自のサブディレクトリです。プロンプトは、プラグインを使用している人が入力するようなもので、そのスキルの 1 つが処理すべきリクエストなどです。

<h3 id="what-happens-in-a-run">
  実行中に何が起こるか
</h3>

ケースの各実行について、Claude Code は新しい [分離された](#how-runs-are-isolated) [非対話型セッション](/docs/ja/headless) を開始し、プラグインのみをロードし、プロンプトを送信し、Claude が完了するか、ケースのターン制限または時間制限に達するまで動作させます。その後、各グレーダーは最終的な返信、トランスクリプト、または Claude が作成したファイルをチェックし、合格または不合格を判定します。

<h3 id="how-a-case-is-scored">
  ケースのスコア化方法
</h3>

非決定論的なエージェントの 1 回の実行では、ほとんど情報が得られないため、各ケースはデフォルトで 3 回実行されます。実行のスコアは、重み付けを設定した場合は重み付けされた、合格したグレーダーの割合であり、ケースのスコアは実行全体の平均です。ケースは、そのスコアが [`--threshold`](#command-options)（デフォルトは 1.0）を満たすときに合格します。モデル呼び出しでは、スイートはおおよそ cases × runs のエージェント実行をプラグインで行い、[プラグインなしベースライン](#the-no-plugin-baseline) でも同じ数だけ行い、さらに実行ごとに `llm` または `baseline` グレーダーごとに 3 つの短いジャッジ呼び出しを行います。

<h3 id="the-no-plugin-baseline">
  プラグインなしベースライン
</h3>

プラグインなしでも Claude が同じくらい上手くいく可能性があるため、単独のスコアが高いだけではプラグインが役に立ったことを示しません。この 2 つを分離するために、各ケースの実行はデフォルトでプラグインをロードせずに繰り返され、2 つのスコア `WITH` と `W/OUT` が得られます。その差 `Δ` は、プラグインが貢献したものです。ケースがプラグインの有無にかかわらず 1.0 でスコアされた場合、プラグインはそれが合格した理由ではありません。2 つの実行セットは with-arm と without-arm と呼ばれます。[プラグインなしベースラインと比較する](#compare-against-a-no-plugin-baseline) では、グレーダーがそれらの間でどのようにスコア化されるか、およびベースラインをオフにする方法について説明します。

<h2 id="create-your-first-eval-suite">
  最初の eval スイートを作成する
</h2>

このチュートリアルは、独自のプラグイン用に 1 つのケースを作成し、実行し、結果を読みます。開始する前に、以下があることを確認してください。

* Claude Code v2.1.269 以降およびその他の [要件](#requirements)
* プラグインのルートディレクトリで開いているターミナル（`plugin.json` または `.claude-plugin/plugin.json` を含むディレクトリ）
* テストしたいプラグイン内の 1 つのスキルと、ユーザーが入力すべきリクエスト（それがスキルをトリガーすべき）

<Steps>
  <Step title="ケースを作成する">
    プラグインルートから、以下を実行します。

    ```bash theme={null}
    claude plugin eval init
    ```

    Claude Code がこのディレクトリをまだ信頼していない場合、最初に `Trust this plugin directory?` と尋ねます。`y` で答えてください。対話型 Claude Code セッションが開きます。Claude はプラグインを読み、良い結果がどのようなものかを尋ね、プラグインをトリガーすべき、またはトリガーすべきでないプロンプトを提案し、各プロンプト用のグレーダーを設計し、それらを 1 回パイロットして動作を確認し、`evals/` の下に 1 つのケースディレクトリをプロンプトの後に作成します。Claude がスイートの準備ができたことを伝えたら、`/exit` または Ctrl+D でそのセッションを終了してシェルに戻ります。

    プラグインルートで既に Claude Code セッションが開いている場合は、代わりにそこで Claude に `claude plugin eval init` を実行するよう依頼できます。Claude はコマンドを実行し、その会話で同じ質問をします。

    ケースを自分で作成して、ファイルが正確に何を含むかを確認したい場合は、[ケースを手動で作成する](#write-a-case-manually) に従い、ここに戻ってそれを実行します。
  </Step>

  <Step title="スイートを実行する">
    プラグインルートのシェルに戻り、`evals/` の下のすべてのケースを実行します。

    ```bash theme={null}
    claude plugin eval .
    ```

    ステップ 1 でこのディレクトリを既に信頼しているため、実行はすぐに開始されます。代わりにケースを手動で作成した場合、実行は最初に `Trust this plugin directory? [y/N]` と尋ねます。`y` で答えてください。[実行がアクセスできるもの](#security) は、同意していることを説明します。

    各ケースはプラグイン付きで 3 回、プラグインなしで 3 回実行されるため、1 つのケースは 6 回の実行です。各実行が完了すると、その実行のスコアと各グレーダーの判定を含む進捗行が出力されます。
  </Step>

  <Step title="サマリーを読む">
    スイートが完了すると、サマリーテーブルが表示され、その後にレポートの場所が表示されます。

    ```text theme={null}
    CASE        WITH  W/OUT Δ      RUNS COST    NOTES
    first-case  1.00  0.33  +0.67  6    $0.41

    1 case(s) · mean Δ +0.67 · 74s · $0.41
    Report: /Users/you/my-plugin/evals/results/2026-09-10T17-02-11-482Z/report.html
    Published: https://claude.ai/... · keep local next time with --no-publish
    ```

    `WITH` はプラグインをロードしたケースのスコア、`W/OUT` はロードしないでのスコア、正の `Δ` はプラグインがスコアを上げたことを意味します。`COST` はモデル呼び出しの定価見積もりで、`NOTES` は最も高い重みの失敗したグレーダーの説明、または with-arm の実行エラーを示します。
  </Step>

  <Step title="レポートを開いて反復する">
    `Published:` URL、または `Published:` 行が表示されない場合は `Report:` パスを開いて、すべての実行のすべてのグレーダーの判定と説明を確認し、`llm` グレーダーについてはジャッジの投票と判定した抜粋を確認します。`Published:` 行は、アカウントが [レポートを公開](#html-report) できる場合にのみ表示されます。

    最初の一般的な発見は、ケースの `tool_used: Skill` グレーダーが失敗している `Δ` がほぼゼロで、Claude が自然な表現でスキルを選択していないことを意味します。スキルの [`description`](/docs/ja/skills#frontmatter-reference) を調整し、`claude plugin eval .` を再度実行し、比較します。

    1 つのケースを安く反復するには、1 つの arm を 1 回実行します。1 回の実行はノイズが多いため、信頼する前にデフォルトの 3 回で変更を確認してください。1 つの arm では、テーブルは `WITH`、`W/OUT`、`Δ` の列の代わりに `SCORE` と `PASS%` の列を表示します。

    ```bash theme={null}
    claude plugin eval . --case <case-name> --runs 1 --ablation none
    ```

    `<case-name>` を `evals/` の下のディレクトリ名の 1 つに置き換えます。
  </Step>
</Steps>

<h2 id="write-and-refine-cases">
  ケースを作成して改善する
</h2>

`claude plugin eval init` が作成するケースは、開いて変更し、追加できるプレーンファイルです。ケースはプラグインの eval ディレクトリの下のディレクトリで、`prompt.md`、`case.yaml`、またはその両方を含みます。ケースをグループ化するには、それ自体がケースではないディレクトリの下にネストします。`graders/` やフィクスチャファイルなど、ケースディレクトリ内のすべてはそのケースに属します。

これは `claude plugin eval init` が作成するレイアウトで、新しいスイートに使用するレイアウトです。[eval スイートリファレンス](#eval-suite-reference) には、モックと結果を含む完全なツリーがあります。

```text theme={null}
my-plugin/
├── .claude-plugin/plugin.json
├── skills/...
└── evals/
    ├── first-case/
    │   ├── prompt.md          # frontmatter: case fields; body: the prompt
    │   ├── graders/
    │   │   ├── criteria.md    # frontmatter: type + options; body: rubric or pattern
    │   │   └── skill-fired.md
    │   └── case.yaml          # optional: only for context.* fields
    ├── ignores-unrelated-request/
    │   └── ...
    └── results/               # written by each run; add to .gitignore
```

<h3 id="write-a-case-manually">
  ケースを手動で作成する
</h3>

Claude にケースを `claude plugin eval init` で作成させることが推奨パスです。代わりに自分で作成するには、空のテンプレートから開始します。次のコマンドは、プレースホルダー `prompt.md` と 1 つのプレースホルダーグレーダーを含む `first-case` という名前のケースを作成し、何も実行しません。

```bash theme={null}
claude plugin eval init --bare first-case
```

```text theme={null}
evals/first-case/
├── prompt.md            # the prompt sent to Claude, plus run limits
└── graders/
    └── criteria.md      # one grader: how to score the result
```

`prompt.md` では、各実行で Claude が受け取るメッセージを作成し、frontmatter で実行の制限とケースが使用できるツールを設定します。`evals/first-case/prompt.md` を開き、プレースホルダー本文をスキルの 1 つが処理すべきリクエストに置き換えます。ユーザーが入力するであろう方法で表現されます。この例はコミットメッセージを作成するスキル用です。独自のリクエストを使用してください。

```markdown theme={null}
---
max_turns: 10
allowed_tools: [Read, Glob, Grep, Skill]
---

Write me a commit message for this change: I renamed getUser to fetchUser and updated the three call sites.
```

各実行は空の作業ディレクトリで開始されるため、タスクに必要なものをプロンプト自体に入れるか、[ワークスペースまたは履歴をセットアップする](#add-setup-or-history-with-case-yaml) 最初に。[frontmatter フィールドの完全なリスト](#prompt-md-fields) は、モデル、タイムアウト、タグ、および環境変数をカバーしています。

`graders/` の下の各ファイルは、実行後に適用される 1 つのチェックです。`evals/first-case/graders/criteria.md` を開き、プレースホルダーをジャッジモデル用のルーブリックに置き換えます。具体的な PASS および FAIL 条件として作成されます。

```markdown theme={null}
---
type: llm
---

PASS if <what a correct response contains>.
FAIL if <what a wrong or missing response looks like>.
```

その後、スキルが答えを生成したかどうかをチェックする 2 番目のグレーダーを追加します。`evals/first-case/graders/skill-fired.md` を作成し、`your-skill-name` をスキルのディレクトリ名（`skills/` の下）に置き換えます。これは Claude が呼び出す名前です。

```markdown theme={null}
---
type: tool_used
tool: Skill
input_match: '"skill"\s*:\s*"(?:[\w-]+:)?your-skill-name"'
---
```

これは Claude がその実行中にそのスキルを少なくとも 1 回呼び出した場合に合格します。これには、その名前空間付き `plugin-name:skill-name` 形式も含まれます。[グレーダータイプ](#grader-types) は、正規表現のマッチングやファイルが作成されたことの確認など、利用可能な他のチェックをリストします。

両方のファイルを保存したら、[クイックスタート](#create-your-first-eval-suite) が行うように、プラグインルートから `claude plugin eval .` でケースを実行します。

<h3 id="set-run-limits-and-tools-in-prompt-md">
  prompt.md で実行制限とツールを設定する
</h3>

`prompt.md` frontmatter でケースの `max_turns`、`timeout_seconds`、`model`、`tags`、および使用可能な `allowed_tools` を設定します。[prompt.md frontmatter](#prompt-md-fields) リファレンスはすべてのフィールドとそのデフォルトをリストします。Claude は本文を正確に作成したとおりに受け取ります。その中の `@path` メンションはファイル添付に展開されないため、Claude がファイルを読む必要がある場合は、`allowed_tools` でツールを付与します。

<h3 id="grade-the-result">
  グレーダーを選択して重み付けする
</h3>

グレーダーの frontmatter はその `type` を設定し、オプションで実行のスコアでより多くをカウントする `weight` と、ベースラインに対してどのようにスコア化されるかを制御する [`arm`](#compare-against-a-no-plugin-baseline) を設定します。6 つのタイプのうち、`regex`、`tool_used`、`tool_order`、`file_exists` はトランスクリプトとファイルから計算され、コストはかかりませんが、`llm` と `baseline` はジャッジモデルを呼び出し、実行のコストに追加されます。

カスタムコードグレーダーはありません。[グレーダータイプ](#grader-types) は各タイプのオプションと合格条件をリストし、[グレーダーが見ることができるもの](#what-a-grader-can-look-at) は `target` と `focus` が受け入れる値をリストします。

`llm` および `baseline` グレーダーのジャッジはデフォルトで小さく高速なモデルです。ニュアンスのあるルーブリックに対してより強力なものを使用するには、`--judge-model sonnet` または完全なモデル ID を渡します。

<h4 id="choose-graders-that-give-a-stable-signal">
  安定した信号を与えるグレーダーを選択する
</h4>

`llm` グレーダーはモデルに判定を求めるため、その答えは実行間で異なる可能性があり、読む必要があるテキストが長いほど異なります。これらの習慣はスイートのスコアを十分に安定させて信頼できるようにします。

* 生成されたファイルなどの長い出力については、ファイルの内容に対する `regex` グレーダーでグレード化します。これは毎回同じ方法でファイル全体をチェックします。短い出力には `llm` グレーダーを保持し、ルーブリックを具体的な PASS および FAIL 条件として作成します。
* 各ケースに、最終メッセージや生成されたファイルなどの結果に対する 1 つのグレーダーと、`tool_used` や `tool_order` など Claude がそこに到達した方法に対する 1 つのグレーダーを付与します。一緒に、答えが正しかったかどうかと、プラグインがそれを生成したかどうかの両方を示します。
* ケースの `tool_used: Skill` グレーダーが合格しているが `Δ` が負の場合、プラグインの前にジャッジを疑います。小さいジャッジモデルは、ルーブリックが説明する内容と異なる形式であるため、正しい答えを間違いとマークできます。`--judge-model sonnet` で再実行し、形式が判定を決定しないようにルーブリックを厳しくします。
* ビルドまたはテストが実行内で合格したことを確認するには、プロンプトで Claude にそれを実行し、結果をファイルに書き込むよう依頼し、そのファイルをグレード化し、コマンドが `tool_used` グレーダーで実行されたことを主張します。その `input_match` はコマンドに名前を付けます。

<h3 id="compare-against-a-no-plugin-baseline">
  プラグインなしベースラインに対してスコア化する
</h3>

プラグインがテスト中の場合、各ケースはデフォルトで 2 つの arm で実行されます。with-arm はプラグインをロードした実行で、without-arm はプラグインなしで同じ数の実行です。サマリーとレポートは両方のスコアと `Δ`（with-arm スコアから without-arm スコアを引いたもの）を表示します。比較が不要な場合（グレーダーを反復するなど）、`--ablation none` を渡してコストを半減させ、with-arm のみを実行します。

2 つの arm 実行では、一部のグレーダーは `scored: false` で報告されます。「スキルが呼び出された」などのチェックはプラグインなしでは決して合格できないため、カウントすると without-arm がゼロに向かい、`Δ` を膨らませます。2 つの arm を比較可能に保つために、Claude Code はそのようなグレーダーを両方の arm のスコアから除外し、with-arm でそれらを合格/不合格インジケーターのみとして報告します。これには以下が含まれます。

* `tool` が `Skill` である各 `tool_used` グレーダー
* `target: mock_calls` を持つ各 `regex` グレーダーと、各 [モック化されたサーバー](#mock-mcp-servers) がケース内にあり、プラグインが宣言するものである場合の `focus: mock_calls` を持つ各 `llm` グレーダー
* `arm: with-only` でマークするグレーダー

3 つの設定がその除外を変更します。

* **すべてのグレーダーが除外される**: ケース内のすべてのグレーダーがこれらの 1 つである場合、スコア化するものが何も残らないため、代わりに通常スコア化されます。
* **`arm: both`**: グレーダーに `arm: both` を設定して、それに関わらず両方の arm でスコア化します。これは「スキルを呼び出してはいけない」チェックに `min: 0` と `max: 0` を使用する場合に必要です。
* **`--ablation none`**: `--ablation none` の下では何も除外されないため、同じスイートは 2 つのモードで異なる絶対スコアを生成できます。

<h3 id="use-a-different-eval-directory">
  別の eval ディレクトリを使用する
</h3>

`evals/` が既に別のツールで使用されている場合は、スイートを別のディレクトリに保持します。プラグインの `plugin.json` にそのディレクトリを記録して、すべての実行とすべての共同作業者がそれを使用するようにするか、単一の実行のためにコマンドラインで渡すことができます。

* **`plugin.json` で**: `"experimental": { "evals": "quality/evals" }` を追加します。
* **コマンドラインで**: `claude plugin eval` と `claude plugin eval init` の両方に `--eval-dir quality/evals` を渡します。

両方を設定した場合、フラグのディレクトリが使用されます。`qa` または `quality/evals` などのプレーンディレクトリ名の相対パスを指定します。絶対パスまたは `..` を含むパスは受け入れられません。フラグ値としてはエラーで、マニフェスト値として使用できない場合は `Warning:` 行が出力され、実行は `evals/` を使用します。ケース、結果、および `init` 出力はすべてそのディレクトリに移動します。

<h2 id="set-up-fixtures-and-mocks">
  フィクスチャとモックをセットアップする
</h2>

ケースはプロンプト以上のものが必要な場合があります。ワークスペース内のファイルまたは git リポジトリ、続行する以前の会話、またはプラグインが通信する MCP サーバーからの回答。これらのそれぞれはケースの横にセットアップされるため、実行は繰り返し可能なままです。

<h3 id="add-setup-or-history-with-case-yaml">
  ワークスペースまたは会話をシードする
</h3>

各実行は空のワークスペースで開始されます。ケースがプロンプト以上のものが必要な場合は、`context` ブロックを含む `case.yaml` を `prompt.md` の横に追加します。

* **フィクスチャファイルまたは git リポジトリ**: ケースディレクトリに Bash スクリプトを作成し、`context.scaffold_script` で名前を付けます。スクリプトはエージェントのサンドボックスの外で、あなたとして実行され、`--scaffold` を渡すときのみ実行されるため、そのフラグはあなたまたはあなたの組織が作成したスイートに対してのみ渡します。
* **以前の会話を続行する**: トランスクリプトを `.jsonl` ファイルとして保存し、`context.history_file` で名前を付けます。ケースのプロンプトは次のユーザーターンになります。
* **実行中に Claude が読むことができるフィクスチャディレクトリ**: `context.add_dirs` にそれらをリストします。

`case.yaml` には `schema_version: "1.1"` と `name` も必要です。[case.yaml フィールド](#case-yaml-fields) リファレンスには完全なリストがあります。

この `case.yaml` はスクリプトからワークスペースをシードし、Claude が `resources/` ディレクトリからフィクスチャを読むことができるようにします。

```yaml theme={null}
schema_version: "1.1"
name: changelog-from-diff
tags: [smoke]
context:
  scaffold_script: fixture.sh
  add_dirs: [resources]
```

<h3 id="mock-mcp-servers">
  MCP サーバーをモックする
</h3>

スキルが MCP ツールを呼び出すプラグインを評価できます。その背後にある実際のサービスなしで。スイート全体の場合は `evals/mocks/<server>/<tool>.md` の下に 1 つのツールごとに 1 つの Markdown ファイルを配置するか、1 つのケースの場合はケース独自の `mocks/` ディレクトリの下に配置します。`<server>` はプラグインの [MCP 設定](/docs/ja/plugins/components#mcp-servers) のサーバーの名前です。

実行は、要求しない限り、プラグインの実際の MCP サーバーを開始しません。Claude Code は各サーバー独自の名前の下にスタンドインを登録します。モックファイルを持つツールはそれから答え、`--allow-tools` 付与なしで許可され、モックファイルを持たないツールは Claude で利用できません。モックがまったくないサーバーは、ケースの `mocked:` 進捗行に `plugin_<plugin>_<server>[not started: no mock]` として表示されます。

ファイルの本文は、ツールが Claude に返すものです。このモックは `tracker` という名前のサーバー上の `create_issue` ツールの代わりになり、Claude が送信する入力をチェックし、タイトルをエコーバックします。`evals/mocks/tracker/create_issue.md` として保存します。

```markdown theme={null}
---
expect:
  title: string
  priority: [low, medium, high]
---

Created issue #4821: {{input.title}}
```

モックファイルの本文とフロントマターは、これらのオプションを受け入れます。

* **置換**: `{{input.<field>}}` で呼び出しの入力からフィールドを挿入し、`{{file:fixtures/{input.<field>}.json}}` でモックの横のフィクスチャファイルの内容を挿入します。
* **`expect:`**: `expect:` ブロックは入力を保護します。呼び出しがそれに違反する場合、実行はスコア 0 で中止され、理由が記録されます。そのため、ケースはプラグインがサーバーに何を求めたかを主張できます。
* **`error: true`**: `error: true` を設定して本文をツールエラーとして返します。
* **`type: agent`**: `type: agent` を設定して小さいモデルが本文の指示からサーバーとして答えるようにします。

[モックファイルリファレンス](#mock-files) はすべてのキーと `_server.md` および `_tools.json` ファイルをリストします。

呼び出し自体をグレード化するには、グレーダーを `target: mock_calls` に指します。

プラグインの実際の MCP サーバーに対して実行するには、これらのフラグの 1 つを渡します。どちらの方法でも、これらのプロセスはあなたとして実行され、実行のサンドボックスの外で、それらのツールは [`--allow-tools` 付与](#grant-tools) が必要です。

* **`--allow-real-servers`**: モックしていない各サーバーの実際のプロセスを開始し、モックされたツールからの回答を続けます。
* **`--mocks off`**: `mocks/` を完全に無視し、プラグインが宣言するすべてのサーバーを開始します。

<h4 id="replay-agent-mock-answers">
  エージェントモック回答を再生する
</h4>

`type: agent` モックは [`--judge-model`](#command-options) への呼び出しで答えるため、その出力は実行間で異なり、ジャッジを変更すると変わります。実行がエラーまたは中止なしで完了すると、Claude Code は各回答をエージェントモックが結果ディレクトリの `mock-recordings/` の下に与えたものを保存します。

`ADOPT.txt` をそこで開いて、各記録と `.replay/<server>/` ディレクトリを確認し、モックの横にコピーします。記録をそこにコピーした後、後の実行はモデル呼び出しなしで同じ呼び出しから同じ答えを返します。`mocks/.replay/` を `mocks/` の残りと一緒にコミットして、CI 実行が繰り返し可能になるようにします。

<h2 id="run-evals">
  evals を実行する
</h2>

スイートが存在すると、`claude plugin eval` はそれを実行します。ターゲット引数でどのプラグインとケースを実行するかを選択し、`--allow-tools` でケースが読み取り専用セット以上に必要とするツールを付与し、他のオプションで実行数、モデル、コスト、出力を制御します。

<h3 id="choose-what-to-evaluate">
  評価対象を選択する
</h3>

ほとんどの場合、プラグインルートから `claude plugin eval .` を実行します。これはスイート内のすべてのケースをロードされたプラグインで実行します。単一のケースファイルを実行するか、開発中のプラグインではなくインストール済みのプラグインを評価するには、別のターゲットを渡します。

| ターゲット                                           | 実行内容                                                                                                                           |
| :---------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| プラグインのルートディレクトリ（`.` など）                         | その eval ディレクトリの下のすべてのケース（そのプラグインをロード）                                                                                          |
| 単一の `prompt.md` または `case.yaml` ファイル            | そのケース（その囲むプラグインをロード）                                                                                                           |
| インストール済みプラグイン（名前、`name` または `name@marketplace`） | インストール済みコピーの eval ディレクトリのケース（インストール済みコピーをロード）。結果は現在のディレクトリの `./evals/results/` または `--eval-dir` で `./<dir>/results/` に書き込まれます。 |
| `name@skills-dir`                               | [skills-directory プラグイン](/docs/ja/plugins/loading#plugins-shared-through-a-repository) の場合も同じ                                       |
| 省略                                              | 現在のディレクトリをパスとして                                                                                                                |

`--case <glob>` を追加してケース名でフィルタリングし、`--tag <tag>` を使用して指定されたタグのいずれかを持つケースを保持します。ターゲットを `--tag`、`--allow-tools`、`--json` の前に配置します。最初の 2 つはリストを取り、`--json` はオプションのパスを取るため、それぞれは後に続くターゲットを独自の値として読み取ります。

<h3 id="grant-tools">
  ツールを付与する
</h3>

実行は許可を求めるために停止することはありません。付与しなかった許可が必要な組み込みツール（`Bash`、`Write`、`Edit`、`WebFetch`、`WebSearch` など）はセッションから削除されるため、Claude はそれらをまったく呼び出すことができません。

実行は、ケースが `allowed_tools` にリストする読み取り専用ツール（`Read`、`Glob`、`Grep`、`NotebookRead`、`Skill`、`AskUserQuestion`、`Agent`、`TodoWrite`、およびタスクツール `TaskCreate`、`TaskGet`、`TaskList`、`TaskUpdate`、`TaskStop`）と、`--allow-tools` で付与するもの（スイート内のすべてのケースに適用）のみを許可します。ケースが `Bash`、`Write`、`Edit`、`WebFetch`、`WebSearch` を使用できるようにするには、自分で付与します。

```bash theme={null}
claude plugin eval . --allow-tools Write Edit "Bash(npm test *)"
```

ケースが付与しなかったツールを要求した場合、進捗出力は `not granted` としてリストします。[モックされた](#mock-mcp-servers) MCP サーバー上のツールは許可が不要です。実際のプラグイン MCP サーバー上のツールは、サーバーが開始されている必要があります（`--allow-real-servers` または `--mocks off` で）、および `--allow-tools "mcp__plugin_my-plugin_github__*"` などの名前による付与。プラグインの MCP ツールは `mcp__plugin_<plugin>_<server>__<tool>` という名前です。

任意の形式で `Bash` を付与すると、すべてのコマンドは Claude Code の [OS レベルサンドボックス](/docs/ja/sandboxing) の下で実行されます。書き込みはランの作業スペースに限定され、ホームディレクトリと Claude Code 設定は読み取り不可で、ネットワークアクセスは `--allow-tools "WebFetch(domain:example.com)"` で付与するドメインに限定されます。サンドボックスバックエンドのないマシンで Bash または PowerShell を付与すると、Claude Code は各実行を拒否し、ケースは実行エラーを表示し、通常はスコア 0 になります。ネイティブ Windows にはバックエンドがないため、WSL2 の下でシェル付与スイートを実行します。Linux では、最初に `bubblewrap` と `socat` をインストールします。[サンドボックスの前提条件](/docs/ja/sandboxing) を参照してください。

<h3 id="command-options">
  コマンドオプション
</h3>

この表は、実行数、モデル、スコアリング、コスト、ツール付与、モック、出力のオプションをカバーしています。`claude plugin eval --help` を実行して完全なリストを確認します。これには `--case`、`--tag`、`--eval-dir`、`--no-scaffold`、`--report`、`--verbose` も含まれます。

| オプション                      | デフォルト                                                                              | 効果                                                                                                                                      |
| :------------------------- | :--------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| `--runs <n>`               | 各ケースの `runs`、またはそれ以外は 3                                                            | arm ごとのケースごとの実行                                                                                                                         |
| `-j`, `--concurrency <n>`  | `1`                                                                                | 最大 1 から 8 のエージェント実行を同時に実行します。アカウントのレート制限を共有するため、これはそのレート制限を超えるスループットを上げるのではなく、壁時間を短縮します。結果はケース順を保持します。                                  |
| `--model <model>`          | 各ケースの `model`、またはそれ以外は `ANTHROPIC_MODEL` が設定されている場合はそれ、またはそれ以外は Claude Code のデフォルト | テスト中のエージェント用のモデル。CI でモデルロールアウトがプラグイン回帰と間違われないようにそれを固定します。                                                                               |
| `--judge-model <model>`    | 小さく高速なモデル                                                                          | `llm` および `baseline` グレーダー用のモデル                                                                                                         |
| `--ablation <mode>`        | プラグインが解決する場合は `with-without`、それ以外は `none`                                          | 何を追加するかを測定するために、プラグインなしで各ケースを実行するかどうか。`none` は 1 つの arm を実行します。`with-without` はプラグインなしベースラインを追加します。                                     |
| `--threshold <0..1>`       | `1.0`                                                                              | ケースが with-arm スコアがこれ以上の場合に合格します。これ以下のケースはコマンドを終了 1 にします。                                                                                |
| `--max-cost-usd <usd>`     | 上限なし                                                                               | 実行の定価コスト見積もりの上限（プラン使用量ではなく）。各実行開始前にチェックされます。使用後、さらに何も開始されません。既に進行中の実行は完了するため、支出は上限を超える可能性があります。未開始の実行が残っている場合、コマンドは部分的な結果で終了 2 になります。   |
| `--allow-tools <tools...>` | なし                                                                                 | 読み取り専用セット以上のツールを付与します。[ツールを付与する](#grant-tools) を参照してください。                                                                               |
| `--scaffold`               | オフ                                                                                 | 各ケースの [`scaffold_script`](#add-setup-or-history-with-case-yaml) を実行します。                                                                 |
| `--trust-plugin`           | オフ                                                                                 | 自分で実行するコードとスイートを持つプラグインの最初の実行信頼プロンプトをスキップします。CI でジョブがプロンプトで拒否されたり待機したりしないようにそれを渡します。[実行がアクセスできるもの](#security) を参照してください。                |
| `--mocks <mode>`           | `record`                                                                           | `record` は [モック](#mock-mcp-servers) から MCP ツール呼び出しに答え、プラグインの実際のサーバーを開始せず、再生用にエージェントモック回答を保存します。`off` はモックを無視し、プラグインの実際の MCP サーバーを開始します。 |
| `--allow-real-servers`     | オフ                                                                                 | `--mocks record` で、モックを持たないサーバーのプラグインの実際の MCP サーバーも開始します。                                                                               |
| `--json [path]`            | オフ                                                                                 | [結果ドキュメント](#json-result) を stdout に出力するか、`.json` で終わるパスに書き込みます。実行は静かです。進捗行またはサマリーテーブルはありません。                                            |
| `--output-dir <dir>`       | `<eval dir>/results/<timestamp>/`                                                  | `aggregate-result.json` と `report.html` が行く場所                                                                                           |
| `--no-publish`             |                                                                                    | HTML レポートをローカルに保持します。[HTML レポート](#html-report) を参照してください。                                                                               |
| `--publish-report`         |                                                                                    | Claude Code セッションが開始した実行など、デフォルトではローカルに留まる場所でもレポートを公開します。                                                                               |
| `--keep-temp`              | オフ                                                                                 | すべての実行のサンドボックスディレクトリを保持し、そのパスを出力します（Claude が生成したものをデバッグするため）。                                                                           |

<h3 id="run-evals-in-ci">
  CI で evals を実行する
</h3>

CI ジョブで、`--json` でスイートを実行して結果をアーカイブ用に書き込み、終了コードでビルドを失敗させます。ジョブが [最初の実行信頼プロンプト](#security) で待機しないように `--trust-plugin` を渡し、スコアが時間とともに比較可能になるように両方のモデルを固定し、レポートをローカルに保持し、コスト上限を上限として設定します。

```bash theme={null}
claude plugin eval . \
  --trust-plugin \
  --json results.json \
  --threshold 0.8 \
  --model claude-sonnet-5 \
  --judge-model claude-haiku-4-5 \
  --no-publish \
  --max-cost-usd 20
```

ジョブの終了コードは何が起こったかを示します。

| 終了コード | 意味                                                                                                                   |
| :---- | :------------------------------------------------------------------------------------------------------------------- |
| 0     | すべてのケースが `--threshold` 以上でスコア化され、すべてのケースファイルがロードされました。                                                               |
| 1     | ケースが閾値以下でスコア化された、ケースファイルがロードに失敗した、ケースが見つからない、実行を開始できない、プラグインディレクトリが信頼されておらず `--trust-plugin` が渡されなかった、またはオプションが無効です。 |
| 2     | 部分的な実行。`--max-cost-usd` 上限に達した、または認証情報が最初の実行の前または時点で拒否されました。`results.json` は `partial: true` と理由で書き込まれます。            |
| 130   | 中断。部分的な結果が書き込まれます。                                                                                                   |
| 143   | 終了（CI タイムアウトなど）。                                                                                                     |

HTML レポートの書き込みまたは公開の問題は終了コードを変更しません。ケースがなぜ低くスコア化されたかを確認するには、ローカルで `--json` なしで実行して、実行ごとの進捗とグレーダー行が出力されるようにします。

CI ランナーは以下が必要です。

* **インストールと認証情報**：CI ランナーは Claude Code インストールと [環境の認証情報](/docs/ja/authentication)（`ANTHROPIC_API_KEY` など）が必要です。
* **信頼**：`--trust-plugin` なしで、チェックアウトディレクトリを Claude Code がまだ信頼していないジョブは、[最初の実行信頼プロンプト](#security) が必要で、質問できない実行は終了 1 で拒否されます。
* **CI での `init`**：`claude plugin eval init` はあなたの質問をするためにターミナルが必要です。CI では、`claude plugin eval init --bare <name>` を実行して空のテンプレートを取得します。

コストを予測可能に保つために、クイックな毎変更スイートにはジャッジを呼び出さないグレーダーのみを付与し、`Δ` が不要な場所で `--ablation none` を使用し、`partial: true` ドキュメントと `skippedPaidGraders` を持つ実行をあなたがチャートするトレンドから除外します。

<h2 id="read-the-results">
  結果を読む
</h2>

少なくとも 1 つのケースを持つすべての実行は、eval ディレクトリ内に `results/<timestamp>/` ディレクトリを書き込み、`aggregate-result.json` と `report.html` を含みます。パスターゲットの場合はプラグインの下。プラグインに名前を付けた場合は現在のディレクトリの下。[ターゲットテーブル](#choose-what-to-evaluate) に示すように。サマリーテーブル、JSON、レポートはすべて同じ結果データをレンダリングします。

<h3 id="html-report">
  HTML レポート
</h3>

`report.html` は単一の自己完結型ファイルで、外部リクエストを行わないため、CI ジョブに添付したり、ディスクから開いたりできます。この例は、`--threshold 0.8` で実行された 3 ケーススイートのレポートの上部です。表示されるコストは定価見積もりで、モデルとケース数によって異なります。

<img src="https://mintcdn.com/claude-code/qq7LHDi_F0aeFHgk/images/plugin-eval-report.png?fit=max&auto=format&n=qq7LHDi_F0aeFHgk&q=85&s=106eb6e6a70a6565f891ea3a4564f87d" alt="eval レポートの上部。「Plugin effect: +33.3 pts vs baseline, improved 2, flat 1, regressed 0 of 3 cases」と読む判定行、スイートスコア、アブレーション デルタ、ベースラインスコア、閾値を通過するケース、完全な実行の 5 つのサマリータイル、その後、デルタ、スコアバー、2 つのグレーダーが両方とも合格を示す 1 つの実行を持つ最初のケース。" width="1360" height="1032" data-path="images/plugin-eval-report.png" />

上から下に読みます。

* **判定行とタイル** は、プラグインがスイート全体で役に立ったかどうかに答えます。スイートスコアはケースごとのプラグイン付きスコアの平均、アブレーション Δ はそれがベースラインスコアの上または下にどの程度座っているか、ケースは何が閾値を満たしたかをカウントします。完全な実行はすべてのグレーダーが合格した with-plugin 実行の共有です。
* **各ケースカード** はケース独自の `Δ` とプラグイン付きスコアを表示し、バーの閾値にティックを付けます。`Δ` が負のケースは赤い左端を取得するため、スクロール時に回帰が目立ちます。
* **ケース内** では、プラグイン付き実行が最初に来て、ベースライン実行が後に来ます。各実行はグレーダーを合格または不合格チップでリストします。失敗したグレーダーは既に説明で展開されており、`llm` グレーダーはジャッジの投票と判定した証拠も表示します。これはランがなぜ低くスコア化されたかを見つける場所です。スコアに向かわないグレーダー（`tool_used: Skill` など）は `plugin-fired indicator` バッジを持ちます。
* **プロンプトとグレーダー** はケースの下に表示され、各グレーダーのルーブリックまたはパターンを表示するため、スイートなしでレポートを読む人は何が尋ねられたか、何が良いとしてカウントされたかを見ることができます。

claude.ai サブスクリプションでサインインしており、[アーティファクト](/docs/ja/artifacts) がアカウントで利用可能な場合、Claude Code はレポートをプライベートアーティファクトとして公開し、`Published: <url>` を出力します。`--no-publish` を渡してローカルに保持します。`Published:` 行が表示されない場合（API キー認証など）、ローカルファイルがレポートです。

Claude Code セッションが開始した実行（Claude にスイートを実行するよう依頼するなど）もローカルに留まり、その `Report:` 行は `kept local` と言います。そのコマンドに `--publish-report` を追加して公開します。

<h3 id="json-result">
  JSON 結果
</h3>

`aggregate-result.json` および `--json` 出力は、CI スクリプトが解析するための `schemaVersion: 1` を持つバージョン付きドキュメントです。フィールド名は camelCase で、新しいフィールドは既存のフィールドを名前変更せずに追加されるため、認識しないフィールドを無視するようにスクリプトを作成します。

これらはゲートスクリプトが通常読むフィールドです。ドキュメントはスイート設定、すべてのグレーダー定義、および説明と証拠を持つ実行ごとのグレーダー結果も含みます。

| フィールド                                             | 意味                                                                                                                                     |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| `partial`, `partialReason`                        | `cost_ceiling`、`interrupted`、`auth_failed` で `true` の場合、スイートが完了しませんでした。部分的な結果をトレンドチャートから除外します。                                         |
| `aggregates.overallScore`                         | スイート全体のケーススコアの平均                                                                                                                       |
| `aggregates.casesPassed`, `aggregates.casesTotal` | `--threshold` 以上のケース、および合計                                                                                                             |
| `aggregates.meanDelta`                            | ケース全体の平均 `Δ`（2 arm モード下）                                                                                                               |
| `cases[].name`                                    | ケース名                                                                                                                                   |
| `cases[].aggregates.score`                        | ケースの平均 with-arm 実行スコア                                                                                                                  |
| `cases[].aggregates.delta`                        | with-arm スコアから without-arm スコアを引いたもの。arm が比較可能でない場合は省略。                                                                                |
| `cases[].arms.with[].error`                       | `null`、またはランが異常に終了した理由（`timed out after 300s` など）。開始したが悪く終了した実行は、生成されたものに対してグレード化されるため、null 以外のエラーはスコア 0 を意味しません。                      |
| `cases[].arms.with[].aborted`                     | [モック](#mock-mcp-servers) の `expect:` または `abort_when` が実行を停止した場合に存在し、`server`、`tool`、`reason` を持ちます。実行はスコア 0 で、`error` は `null` のままです。 |
| `cases[].arms.with[].skippedPaidGraders`          | コスト上限がこの実行のジャッジグレーダーをスキップした場合は `true`。そのスコアは比較可能ではありません。                                                                               |
| `costUsd`, `durationSeconds`, `claudeVersion`     | 定価でのジャッジ呼び出しを含む推定コスト、壁時間秒、スイートを実行した Claude Code バージョン                                                                                  |

<h2 id="security">
  実行がアクセスできるもの
</h2>

`claude plugin eval` はターゲットプラグインのスキル、フック、エージェントを読み込み、あなたのマシン上で、あなたとして eval スイートを実行します。プラグインを指定することは `claude --plugin-dir` と同じ信頼判断であるため、信頼できるプラグインのみを評価してください。このセクションで説明する分離は、テスト中のエージェントが到達できるものを制限します。これはプラグイン自体のコードに対する境界ではなく、スイートが合格したことは、プラグインが安全であるかどうかについて何も述べていません。

<h3 id="trust-the-plugin-directory">
  プラグインディレクトリを信頼する
</h3>

初めて `claude plugin eval` をディレクトリに対して実行する場合、Claude Code は何かを読み込む前に「このプラグインディレクトリを信頼しますか？」と尋ねます。ただし、インタラクティブな `claude` セッションでそこで既に信頼プロンプトを受け入れている場合は除きます。Git リポジトリ内では、「はい」と答えるとリポジトリ全体が信頼され、インタラクティブセッションでも同様です。stdin または stdout がターミナルでない場合、`--json` の下では、`CI` 環境変数が `true` などの真の値に設定されている場合、実行は尋ねることができず、終了コード 1 で拒否されます。`--trust-plugin` を渡して、自分のマシンで実行するプラグインのみについて、信頼を自分で主張してください。パスとして与えるのではなく、インストール済みプラグインまたはスキルディレクトリプラグインを意味する名前を指定したターゲットは、プロンプトをスキップします。

プラグインとスイートの一部は、その実行のためにそのフラグを渡す場合にのみ実行されます。

* ケースの [`scaffold_script`](#add-setup-or-history-with-case-yaml) は `--scaffold` で実行されます
* [読み取り専用セット以外のツール](#grant-tools)は `--allow-tools` で実行されます
* プラグインの[実際の MCP サーバー](#mock-mcp-servers)は `--allow-real-servers` または `--mocks off` で実行されます

ケースの `allowed_tools` とスキル自体の `allowed-tools` frontmatter は、これらのいずれも拡大することはできません。

プラグインが作成していないフックを配布する場合、または実際の MCP サーバーを開始する場合は、コンテナまたは CI ランナーなどの分離された環境で実行しない限り、スコアを参考情報として扱ってください。フックとサーバーはエージェントのサンドボックスの外で実行され、グレーダーが読み取るファイルに触れる可能性があるためです。

<h3 id="how-runs-are-isolated">
  実行がどのように分離されるか
</h3>

各実行は、使い捨てのホームディレクトリ、作業ディレクトリ、Claude Code 設定を取得し、テスト中のエージェントはそこで `claude -p` 子プロセスとして実行され、プラグインのみが読み込まれます。ケースを作成する際は、これらの結果を念頭に置いてください。

* **個人またはプロジェクトレベルのものは読み込まれません。** ユーザー設定、フック、`CLAUDE.md` ファイル、MCP サーバー、その他のインストール済みプラグイン、メモリ、スキルは存在せず、サンドボックス上のプロジェクトスコープの `.claude/` または `.mcp.json` は読み取られません。シェル環境のほとんども保留されます。[許可リスト](#prompt-md-fields)と `EVAL_*` 変数のみが実行に到達します。プラグインがセットアップを必要とする場合は、プラグインに含める、`scaffold_script` で作成する、または `EVAL_*` 変数を渡してください。
* **管理ポリシーは実行を制限できます。** 管理者がマシンに展開した[管理設定](/docs/ja/managed-settings)の制限は実行内に適用されるため、管理マシン上の結果は、そのポリシーによって管理されていないマシンと異なる場合があります。
* **アーティファクトツールはオフです。** [アーティファクト](/docs/ja/artifacts)を公開するスキルは、そのステップの前に生成するものについてのみグレード可能です。
* **ケース定義はエージェントから隠されています。** 実行は eval ディレクトリを読み取ることができないため、Claude はケースのプロンプト、グレーダー、または兄弟ケースを見ることができません。
* **シェルコマンド外のネットワークサンドボックスはありません。** 付与するシェルコマンドはサンドボックスのネットワークルールの下で実行されます。`WebFetch(domain:…)` 付与はそのドメインに直接到達し、プラグイン自体のフックと開始する実際の MCP サーバーは任意のホストに到達できます。

<h2 id="eval-suite-reference">
  Eval スイートリファレンス
</h2>

eval スイートが含むすべてのコンテンツは、プラグインの eval ディレクトリ `evals/` の下に存在します。ただし、[別のディレクトリを設定](#use-a-different-eval-directory)している場合を除きます。このツリーは、`claude plugin eval` がそこで読み書きするすべてのファイルを示しています。ケースが存在するには、`prompt.md` または `case.yaml` のいずれかが必須です。

```text theme={null}
evals/
├── <case>/                        # ケースごとに 1 つのディレクトリ。グループ化するために非ケースディレクトリの下にネストします
│   ├── prompt.md                  # frontmatter: case と run フィールド。本文: プロンプト
│   ├── case.yaml                  # オプション: context.* フィールド、または 1 つのファイル内の全ケース
│   ├── graders/
│   │   └── <name>.md              # ファイルごとに 1 つのグレーダー。frontmatter: type と options。本文: ルーブリック
│   ├── mocks/                     # オプション: このケースのみのモック。以下と同じレイアウト
│   └── <case.yaml で参照されるフィクスチャ、スクリプト、トランスクリプト>
├── mocks/                         # オプション: スイート全体の MCP モック
│   ├── <server>/
│   │   ├── <tool>.md              # 1 つのモック化されたツール。本文: ツール結果
│   │   ├── _server.md             # オプション: 複数のツールに答える 1 つのエージェント
│   │   ├── _tools.json            # オプション: 実際の説明とスキーマのために保存された tools/list レスポンス
│   │   └── fixtures/              # {{file:fixtures/...}} で挿入されるファイル
│   └── .replay/<server>/          # 採用されたエージェント-モック記録。モデル呼び出しなしで応答
└── results/<timestamp>/           # 各実行で書き込まれます。results/ を .gitignore に追加してください
    ├── aggregate-result.json
    ├── report.html
    └── mock-recordings/           # クリーン実行からのエージェント-モック応答。ADOPT.txt 付き
```

<h3 id="prompt-md-fields">
  prompt.md frontmatter
</h3>

`prompt.md` frontmatter は以下のフィールドを受け入れます。未知のキーはエラーです。

| フィールド                  | デフォルト        | 目的                                                                                                                                                                                                                                                         |
| :--------------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `schema_version`       | `"1.1"`。自動設定 | ケース形式バージョン。`prompt.md` として記述されたケースは自動的に取得されるため、ほとんど設定しません                                                                                                                                                                                                  |
| `name`                 | ディレクトリ名      | ケース名。`--case` グロブはこれにマッチし、レポートはこれをキーにします                                                                                                                                                                                                                   |
| `description`          |              | 人間向け。実行時には使用されません                                                                                                                                                                                                                                          |
| `tags`                 | `[]`         | `--tag` フィルタリング用のラベル。任意のタグがマッチすればケースが実行されます                                                                                                                                                                                                                |
| `plugins`              | 最も近い囲むプラグイン  | テスト対象のプラグインディレクトリ。ケースディレクトリからの相対パス。自動検出がプラグインを見つけられない場合は `plugins: ["../.."]` を設定してください。[プラグインが読み込まれない](#the-baseline-arm-shows-no-plugin-or-delta-is-zero)を参照                                                                                             |
| `runs`                 | `3`          | アーム当たりの実行数。1 から 50。`--runs` でオーバーライドされます                                                                                                                                                                                                                   |
| `expected_outcome`     |              | 人間向け。実行時には使用されません                                                                                                                                                                                                                                          |
| `model`                | 子セッションのデフォルト | テスト対象のエージェント用モデル。`--model` でオーバーライドされます                                                                                                                                                                                                                    |
| `max_turns`            | `10`         | ターンキャップ。最大 200。これに達するとランエラーとして記録され、通常はスコアを低下させるため、寛容に設定してください                                                                                                                                                                                              |
| `timeout_seconds`      | `300`        | 実行ごとの壁時間キャップ。最大 3600                                                                                                                                                                                                                                       |
| `allowed_tools`        | `[]`         | ケースが必要とするツール。例えば `[Read, Glob, Grep, Skill]`。読み取り専用ツールはここにリストされると付与されます。その他については、[ツールを付与](#grant-tools)を参照してください                                                                                                                                           |
| `append_system_prompt` |              | 子セッションのシステムプロンプトに追加されるテキスト                                                                                                                                                                                                                                 |
| `env`                  | `{}`         | 子セッション用の追加環境変数。キーは `EVAL_[A-Z0-9_]*` にマッチする必要があります。その他のキーは実行を失敗させます。実行は、シェルからのホワイトリストのみを継承します。`PATH` とロケール、プロキシと証明書設定、モデルプロバイダーを選択・認証する変数、ほとんどの `ANTHROPIC_*` と `CLAUDE_CODE_*` 設定、および `EVAL_*` です。プラグインに他のもの（ツールチェーン設定など）を渡すには、`EVAL_*` 変数としてエクスポートしてください |

<h3 id="case-yaml-fields">
  case.yaml フィールド
</h3>

`case.yaml` は YAML でケースを説明する別の方法または補足です。他のファイルを指すフィールドを追加します。`schema_version: "1.1"` と `name` が必須です。`prompt.md` フィールドの `description`、`tags`、`plugins`、`runs`、`expected_outcome` はトップレベルに配置されます。`model`、`max_turns`、`timeout_seconds`、`allowed_tools`、`append_system_prompt`、`env` は `execution:` の下に配置されます。両方のファイルが存在する場合、`prompt.md` frontmatter は一致する `case.yaml` フィールドをオーバーライドし、`prompt.md` 本文がプロンプトになり、`graders/*.md` は `case.yaml` にリストされたグレーダーの後に追加されます。

これらのフィールドは `case.yaml` にのみ存在します。

| フィールド                     | 目的                                                                                                                                                   |
| :------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `context.scaffold_script` | ケースディレクトリ内の Bash スクリプト。Claude が開始する前に空のワークスペースで実行され、フィクスチャファイルまたは git リポジトリを作成します。[`--scaffold`](#add-setup-or-history-with-case-yaml) を渡すときのみ実行されます |
| `context.history_file`    | ケースディレクトリ内の `.jsonl` トランスクリプト。再開するために使用されます。ケースのプロンプトは次のユーザーターンになります                                                                                 |
| `context.add_dirs`        | ケースディレクトリ内のディレクトリ。Claude が実行中に読み取ることができます。読み取り専用で付与されます                                                                                              |
| `execution.prompt`        | プロンプト。ケース全体を `case.yaml` に保持し、`prompt.md` を省略する場合                                                                                                    |
| `graders`                 | グレーダーのリスト。各グレーダーは `name` と `graders/*.md` ファイルが frontmatter で取得するのと同じキーを持ちます。`llm` グレーダーの場合、ルーブリックを `criteria` に配置します                                |

<h3 id="grader-frontmatter">
  グレーダー frontmatter
</h3>

`graders/` の下のすべてのグレーダーファイルは、frontmatter でこれらのキーと、そのタイプのオプションを取得します。グレーダーの名前は `.md` なしのファイル名です。

| キー       | デフォルト | 目的                                                                                                                                              |
| :------- | :---- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`   | 必須    | [グレーダータイプ](#grader-types)の 1 つ                                                                                                                  |
| `weight` | `1`   | 実行のスコアにおける相対的な重み。任意の正の数                                                                                                                         |
| `arm`    | 未設定   | `with-only` は [2 アーム実行](#compare-against-a-no-plugin-baseline)でグレーダーをスコアリングから除外します。`both` は Claude Code が除外する可能性があるグレーダーを両方のアームでスコアリングするよう強制します |

<h4 id="what-a-grader-can-look-at">
  グレーダーが見ることができるもの
</h4>

`regex` グレーダーは `target` を取得し、`llm` グレーダーは `focus` を取得します。両方とも同じ値を受け入れます。

| 値                                | グレーダーが見るもの                                                                                                                                                                                   |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `last_message`                   | Claude の最終応答テキスト。これがデフォルトです                                                                                                                                                                  |
| `trace`                          | JSON としてのセッション。1 行に 1 つのメッセージ。`regex` グレーダーはすべてのメッセージを見ます。`llm` ジャッジは最初の 12 と最後の 12 を見ます。その中の引用符と改行は JSON エスケープされるため、regex は `"` ではなく `\"` にマッチします                                           |
| `files`                          | 実行中に Claude が作成したパスのリスト。1 行に 1 つ。その内容ではなく、スキャフォルドが作成したファイルや Claude が単に変更したファイルではありません                                                                                                        |
| `{ source: file, path: <path> }` | 実行後のワークスペース内の 1 つのファイルの内容。プラグインが生成したものをグレードするために使用します。PNG、JPEG、GIF、または WebP ファイルは `llm` ジャッジに画像として表示されます。`llm` ジャッジは `.pptx` または PDF などの他のバイナリファイルを拒否します。画像にレンダリングするか、テキストとして書き出してグレードしてください |
| `mock_calls`                     | Claude が [モック化された MCP ツール](#mock-mcp-servers)に行った各呼び出し。その入力とモックの応答を含みます                                                                                                                      |

<h4 id="grader-types">
  グレーダータイプ
</h4>

以下の各グレーダータイプは、そのオプションと合格時を列挙しています。

| タイプ           | オプション                              | 合格時                                                                                                                                                                                    |
| :------------ | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `regex`       | `pattern`、`flags`、`match`、`target` | JavaScript regex `pattern` がターゲットで見つかります。`match: not_contains` を設定して不在を要求するか、`match: "count:N"` を設定して正確に N 回のマッチを要求します。大文字小文字を区別しないようにするには `flags: i` に配置します。インライン `(?i)` はサポートされていません |
| `tool_used`   | `tool`、`input_match`、`min`、`max`   | JSON エンコードされた入力がオプションの `input_match` regex にマッチする `tool` への呼び出し数が `min`（デフォルト 1）と `max`（デフォルト無制限）の間です。ツールが呼び出されなかったことを主張するには、`min: 0` と `max: 0` の両方を設定します                             |
| `tool_order`  | `before`、`after`                   | 両方のツールが呼び出され、最初にマッチする `before` 呼び出しが最初にマッチする `after` 呼び出しより前です。各々はツール名または `{ tool, input_match }` です                                                                                   |
| `file_exists` | `path`、`exists`                    | Claude が作成したファイルが `path` グロブにマッチするか、`exists: false` で何もマッチしません。実行中に作成されたファイルのみがカウントされます                                                                                                |
| `llm`         | `criteria`、`focus`                 | ジャッジモデルが少なくとも 3 回中 2 回のルーブリックで PASS に投票します。`.md` レイアウトではファイル本文が criteria です                                                                                                            |
| `baseline`    | `baseline_file`、`criteria`         | ジャッジが実行が `baseline_file`（ケースディレクトリ内の `.jsonl`）の参照トランスクリプトと同じくらい criteria を満たしていることを検出します                                                                                               |

<h3 id="mock-files">
  モックファイル
</h3>

`mocks/<server>/` の下の `<tool>.md` ファイルは 1 つのツールに答えます。その本文はツール結果で、`{{input.<field>}}` と `{{file:fixtures/<name>}}` の置換があります。その frontmatter は以下のキーを受け入れます。

| キー           | デフォルト   | 目的                                                                                                                                                                  |
| :----------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`       | `fixed` | `fixed` は本文をそのまま返します。`agent` は本文を、実行のためにサーバーをプレイする小さなモデルの指示として扱い、以前の呼び出しを履歴として見ます                                                                                   |
| `expect`     | 未設定     | ドット記法の入力パスから `string`、`number`、`boolean`、`array`、`object` などのタイプ名、`/regex/`、リテラル、または許可されたリテラルのリストへのマップ。それに違反する呼び出しはスコア 0 で実行を中止し、サーバー、ツール、理由を含む `aborted` として報告されます |
| `error`      | `false` | `fixed` のみ。本文をツールエラーとして返します                                                                                                                                         |
| `abort_when` | 未設定     | `agent` のみ。エージェントが実行を中止できる唯一の条件をリストする散文                                                                                                                             |

2 つのオプションファイルがサーバーのディレクトリ内のツールファイルの横に配置されます。

* **`_server.md`**: 複数のツールに答える単一の `type: agent` モック。その `tools:` frontmatter キーにリストされています。同じツール用の `<tool>.md` が優先されます。`expect:` ガードを個別の `<tool>.md` に配置し、ここには配置しません
* **`_tools.json`**: 実際のサーバーから保存された `tools/list` レスポンス。モック化されたツールが許可的なプレースホルダーの代わりに実際の説明と入力スキーマを持つようにします

ケース独自の `mocks/` ディレクトリは同じレイアウトを使用し、スイートのモックをファイルごとにオーバーライドします。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

これらは著者が最も頻繁に遭遇する問題であり、表示される内容に基づいてキーが付けられています。

<h3 id="plugin-eval-is-currently-in-early-access">
  「plugin eval is currently in early access」
</h3>

ビルドはコマンドの一般提供より前のものです。`claude update` を実行してから、新しいセッションでコマンドを再度実行してください。

<h3 id="plugin-eval-is-currently-unavailable">
  「plugin eval is currently unavailable」
</h3>

Anthropic がサーバー側でコマンドをオフにしています。マシン上の何もそれをオンに戻すことはできません。`claude update` を実行して、後で新しいセッションで再度試してください。

<h3 id="is-not-a-trusted-plugin-directory-and-this-run-cannot-stop-to-ask-you-about-it">
  「is not a trusted plugin directory, and this run cannot stop to ask you about it」
</h3>

これは Claude Code がまだ信頼していないディレクトリに対する最初の実行であり、stdin または stdout がターミナルでないか、`--json` を渡したか、`CI` 環境変数が `true` などの真の値に設定されているため、質問することができません。ターミナルで `claude plugin eval <dir>` を一度実行してプロンプトに答えるか、プラグインのコードとスイートを信頼する場合は `--trust-plugin` を渡してください。[実行がアクセスできるもの](#security)を参照してください。

<h3 id="no-eval-cases-found">
  「No eval cases found」
</h3>

eval ディレクトリの下に `<case>/prompt.md` または `<case>/case.yaml` が存在しないか、`--case` および `--tag` フィルターがケースと一致しません。プラグインルートから実行するか、`claude plugin eval init` を実行してスイートを作成してください。

<h3 id="the-baseline-arm-shows-no-plugin-or-delta-is-zero">
  ベースラインアームにプラグインが表示されない、またはデルタがゼロ
</h3>

サマリーに `W/OUT` 列がない場合、またはケースが「ablation requested but no plugin resolved」で失敗する場合、ケースのプラグインが見つかりませんでした。`plugins: ["../.."]` をケースに追加し、ケースディレクトリからプラグインディレクトリへのパスを指定してください。

プラグインが読み込まれ、`Δ` が `tool_used: Skill` グレーダーが失敗している場合でもゼロに近い場合、これは通常、スキルの `description` がプロンプトの表現でトリガーされていないことを意味する実際の発見です。説明を調整して、同じスイートを再度実行してください。

<h3 id="agent-type-’-’-not-found-for-one-of-your-plugin’s-agents">
  プラグインのエージェントの 1 つに対して「Agent type '...' not found」
</h3>

デフォルトでは、各ケースはプラグインを使用して実行され、プラグインなしで実行されます。プラグインなしの実行は[プラグインなしベースライン](#the-no-plugin-baseline)です。Claude がベースライン実行でプラグインのエージェントの 1 つをディスパッチする場合、Agent ツール呼び出しは `Agent type '<plugin>:<agent-name>' not found. Available agents: ...` で失敗します。リストは、[組み込みサブエージェント](/docs/ja/sub-agents#built-in-subagents)など、プラグインなしで存在するエージェントのみを名前付けします。

エラーは予期されています。`Δ` はプラグインの実行をベースラインと比較するためです。JSON 結果では、ベースライン実行は `cases[].arms.without` の下にあります。

プラグインが読み込まれた実行では、`allowed_tools` に `Agent` をリストするケースは、`my-plugin:code-reviewer` など、プラグイン内のエージェントの名前空間付き名前でプラグインのエージェントの 1 つをディスパッチできます。`my-plugin` という名前のプラグイン内の `code-reviewer` エージェント。ベースライン実行をスキップするには、`--ablation none` を渡してください。

<h3 id="everything-scores-zero-although-the-right-files-were-produced">
  正しいファイルが生成されたにもかかわらず、すべてがゼロスコアになる
</h3>

グレーダーが `files`（作成されたパスのリスト）をターゲットにしているが、ファイルの内容を意図していました。`{ source: file, path: <path> }` を `target` または `focus` として使用してください。別途、`file_exists` は実行中に作成されたファイルのみをカウントするため、スキャフォルドが作成したファイルまたは Claude が編集のみしたファイルは見えません。その内容をグレードするか、`Edit` で `tool_used` を使用してください。

<h3 id="a-regex-over-the-trace-doesn’t-match-text-i-can-see">
  トレース上の正規表現が表示されるテキストと一致しない
</h3>

* **ターゲットが間違っている**: デフォルトの `target` はトレースではなく `last_message` です。
* **JSON エスケープ**: `target` をトレースにする場合、行ごとに JSON であるため、引用符は `\"` として表示されます。
* **正規表現構文**: 正規表現は JavaScript 構文を使用するため、`(?i)` を記述するのではなく、`flags` に `i` を入れてください。

<h3 id="tools-are-denied-mcp-tools-are-missing-or-bash-won’t-run">
  ツールが拒否される、MCP ツールが見つからない、または Bash が実行されない
</h3>

読み取り専用セット以外のすべてのものには、`--allow-tools Bash Write` などの許可が必要です。個人用 MCP サーバーは実行中に読み込まれません。プラグイン自体のサーバーは、[オプトイン](#mock-mcp-servers)しない限り開始されず、それらのツールは `--allow-tools "mcp__plugin_<plugin>_<server>__*"` 許可も必要です。モック化されたツールはどちらも必要ありません。

<h3 id="the-run-exits-1-but-the-results-look-fine">
  実行が 1 で終了するが、結果は問題ないように見える
</h3>

デフォルトの `--threshold` は 1.0 であるため、ケースが完璧以下のスコアを取得するとコマンドは 1 で終了します。必要なスコアに一致するしきい値を設定してください。終了 1 は、読み込みに失敗したケースファイルもカバーしており、テーブルの上の stderr で報告されます。

<h3 id="json-output-path-must-end-in-json">
  「--json output path must end in .json」
</h3>

`--json` の後にターゲットを配置したため、出力パスとして読み取られました。`claude plugin eval . --json` のようにターゲットを最初に配置するか、`--json` に明示的な `.json` パスを指定してください。

<h3 id="a-grader-shows-passed-false-under-a-run-that-scored-1-0">
  グレーダーが 1.0 のスコアを取得した実行の下で passed: false を表示する
</h3>

そのグレーダーは設計上、2 アーム実行でスコアから除外され、その `scored` フィールドは `false` です。[プラグインなしベースラインと比較](#compare-against-a-no-plugin-baseline)を参照してください。

<h3 id="runs-fail-with-a-usage-limit-or-rate-limit-error-partway-through">
  実行がスイートの途中で使用量制限またはレート制限エラーで失敗する
</h3>

アカウントがプランの使用量制限に達するか、スイート実行中に API レート制限に達した場合、その後の各実行はそのエラーで終了し、生成されたものに基づいてグレード化され、通常はスコア 0 になります。スイートはまだ完了し、`partial` としてマークされないため、結果は回帰のように見える可能性があります。スコアを信頼する前に `NOTES` 列または JSON の `cases[].arms.with[].error` で制限メッセージを確認してから、制限がリセットされた後に再度実行してください。`--runs 1` または `--case` フィルターを使用して、制限内に留まる必要がある場合は使用してください。

<h3 id="runs-time-out-or-hit-the-turn-cap">
  実行がタイムアウトするか、ターンキャップに達する
</h3>

デフォルトは 10 ターンと 300 秒です。より多くを必要とするタスクの場合、ケースで `max_turns` と `timeout_seconds` を上げ、厳密な実行ごとの制限ではなく、`--max-cost-usd` をコスト上限として使用してください。

<h2 id="see-also">
  関連項目も参照
</h2>

* [プラグインを作成する](/docs/ja/plugins/create): テストしているプラグインを構築し、開発中に `--plugin-dir` でロードします
* [プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference#plugin-eval): `plugin eval` および `plugin eval init` コマンドエントリ。マニフェストの [`experimental.evals`](/docs/ja/plugins/manifest-reference#fields) キーはマニフェストリファレンスにあります
* [スキル](/docs/ja/skills): スキルの説明が Claude がそれを呼び出すときを決定する方法。これはスキルがトリガーされるかどうかをチェックするケースが測定しているものです。
* [サンドボックス](/docs/ja/sandboxing): 実行に Bash を付与するときに適用される OS レベルサンドボックス
* [プラグインを公開する](/docs/ja/plugins/publish): スイートが合格したら、プラグインを公開します
* [プラグインのコストと使用状況を測定する](/docs/ja/plugins/measure): プラグインが各セッションのコンテキストに追加するもの、および人々がそれを使用し続けているかどうか
