> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude をスキルで拡張する

> Claude Code でスキルを作成、管理、共有して Claude の機能を拡張します。カスタムコマンドとバンドルされたスキルが含まれます。

スキルは Claude ができることを拡張します。`SKILL.md` ファイルに指示を記述すると、Claude はそれをツールキットに追加します。Claude は関連する場合にスキルを使用するか、`/skill-name` で直接呼び出すことができます。

同じ指示、チェックリスト、または複数ステップの手順をチャットに何度も貼り付けている場合、または CLAUDE.md のセクションが事実ではなく手順に成長している場合は、スキルを作成してください。CLAUDE.md のコンテンツとは異なり、スキルの本体は使用される場合にのみ読み込まれるため、長い参考資料は必要になるまでほぼコストがかかりません。

<Note>
  `/help` や `/compact` などの組み込みコマンド、および `/debug` や `/code-review` などのバンドルされたスキルについては、[コマンドリファレンス](/docs/ja/commands)を参照してください。

  **カスタムコマンドはスキルにマージされました。** `.claude/commands/deploy.md` のファイルと `.claude/skills/deploy/SKILL.md` のスキルの両方が `/deploy` を作成し、同じように機能します。既存の `.claude/commands/` ファイルは引き続き機能します。スキルはオプション機能を追加します。サポートファイル用のディレクトリ、[スキルを呼び出すユーザーを制御する](#control-who-invokes-a-skill)ためのフロントマター、および Claude が関連する場合に自動的にスキルを読み込む機能です。
</Note>

Claude Code スキルは [Agent Skills](https://agentskills.io) オープンスタンダードに従い、複数の AI ツール全体で機能します。Claude Code は [呼び出し制御](#control-who-invokes-a-skill)、[サブエージェント実行](#run-skills-in-a-subagent)、[動的コンテキスト注入](#inject-dynamic-context)などの追加機能でスタンダードを拡張します。[Claude Code 外でスキルフロントマターを使用する](#using-skill-frontmatter-outside-claude-code)を参照して、どのフロントマターフィールドがスタンダードの一部であり、どれが Claude Code 拡張機能であるかを確認してください。

<h2 id="bundled-skills">
  バンドルされたスキル
</h2>

Claude Code には、`/doctor`、`/code-review`、`/batch`、`/debug`、`/loop`、`/claude-api` などのバンドルされたスキルが含まれています。バンドルされたスキルはプロンプトベースです。Claude に詳細な指示を与え、ツールを使用して作業を調整させます。ほとんどの組み込みコマンドは、代わりに固定ロジックを直接実行します。

バンドルされたスキルは、他のスキルと同じ方法で呼び出します。`/` の後にスキル名を入力します。Claude は関連する場合、一部のバンドルされたスキルを自動的に呼び出します。`/verify` を含む他のスキルは、呼び出した場合にのみ実行されます。これにより、これらの長時間実行されるチェックが時間とトークンを費やすタイミングを制御できます。

ほとんどのバンドルされたスキルはすべてのセッションで利用可能です。いくつかは特定の機能に依存しています。たとえば、`/workflow-authoring` は [動的ワークフロー](/docs/ja/workflows) が有効な場合にのみ利用可能です。

バンドルされたスキルをオフにするには、[`disableBundledSkills`](/docs/ja/settings-reference#disablebundledskills) 設定を使用します。

<Note>
  [`/doctor`](/docs/ja/commands#all-commands) セットアップチェックアップは、Claude Code v2.1.205 以降で `disableBundledSkills` がオンの場合でも入力可能なままです。これを非表示にするには、`DISABLE_DOCTOR_COMMAND` 環境変数を設定するか、[`skillOverrides`](#override-skill-visibility-from-settings) エントリ `"doctor": "off"` を設定します。v2.1.205 より前では、`/doctor` はバンドルされたスキルではなく組み込みコマンドでした。
</Note>

バンドルされたスキルは、[コマンドリファレンス](/docs/ja/commands) に組み込みコマンドと一緒にリストされており、目的列に **Skill** とマークされています。

<h3 id="run-and-verify-your-app">
  アプリを実行して検証する
</h3>

3 つのバンドルされたスキルが連携して、アプリを起動し、テストだけでなく実行中のアプリに対して変更を確認します。

| スキル                    | 目的                                                             |
| :--------------------- | :------------------------------------------------------------- |
| `/run`                 | アプリを起動して駆動し、変更が機能していることを確認する                                   |
| `/verify`              | アプリをビルドして実行し、コード変更が意図したことを実行していることを確認する。テストまたは型チェックにフォールバックしない |
| `/run-skill-generator` | `/run` と `/verify` にプロジェクトをビルドして起動する方法を教える                     |

`/run` と `/verify` はセットアップなしで動作します。プロジェクトタイプ（CLI、サーバー、TUI、ブラウザ駆動）と README、`package.json`、または `Makefile` の内容から起動を推測します。その推測は、標準的な起動を超えて何かが必要なプロジェクト（データベース、env ファイル、グラフィカルセッション、マルチステップビルド）では信頼性が低くなります。

`/run-skill-generator` は代わりにレシピを記録します。クリーン環境からアプリを実行し、機能したもの（インストールコマンド、環境変数、起動スクリプト）をキャプチャし、`.claude/skills/run-<name>/` でプロジェクトごとのスキルとしてコミットします。その後、`/run`、`/verify`、およびリポジトリ内の他のエージェントは、記録されたレシピに従い、再度発見することはありません。プロジェクトごとに 1 回 `/run-skill-generator` を実行し、ビルドまたは起動プロセスが変更された場合は再度実行します。

`/verify` は独自のレシピを記録することもできます。記録されたレシピなしでアプリをビルドして駆動する必要がある場合、機能したもの（`.claude/skills/verify/SKILL.md` をリポジトリルートに、またはモノレポの場合は変更されたパッケージディレクトリに）を書き込むため、後の実行と他のエージェントは同じステップに従います。リポジトリルートでは、記録されたスキルはバンドルされた `/verify` に置き換わります。これには Claude Code v2.1.200 以降が必要です。

Claude は、失敗したコマンドや欠落したステップなど、実行を誤った場合にのみ記録されたファイルを編集するため、セッションごとの差分なしでファイルをコミットできます。v2.1.205 より前では、バンドルされたスキルは Claude に実行から学んだことをすべて折り込むよう指示し、頻繁なマージコンフリクトを引き起こしていました。

<h2 id="getting-started">
  はじめに
</h2>

<h3 id="create-your-first-skill">
  最初のスキルを作成する
</h3>

この例では、Git リポジトリ内のコミットされていない変更を要約し、危険な点にフラグを付けるスキルを作成します。ライブ diff をプロンプトに取り込んでから Claude が読むため、Claude が開いているファイルから推測できるものではなく、実際の作業ツリーに基づいた応答が得られます。Claude は、変更について質問するときに自動的にスキルを読み込むか、`/summarize-changes` で直接呼び出すことができます。

<Steps>
  <Step title="スキルディレクトリを作成する">
    個人用スキルフォルダにスキル用のディレクトリを作成します。個人用スキルはすべてのプロジェクト全体で利用できます。

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="SKILL.md を作成する">
    すべてのスキルには `SKILL.md` ファイルが必要です。このファイルには 2 つの部分があります。Claude がスキルをいつ使用するかを指定する `---` マーカー間の YAML frontmatter と、スキルが実行されるときに Claude が従う指示を含む markdown コンテンツです。ディレクトリ名は入力するコマンドになり、`description` は Claude がスキルを自動的に読み込むかどうかを決定するのに役立ちます。

    これを `~/.claude/skills/summarize-changes/SKILL.md` に保存します。

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    `` !`git diff HEAD` `` 行は[動的コンテキスト注入](#inject-dynamic-context)を使用します。Claude Code はコマンドを実行し、Claude がスキルコンテンツを見る前にこの行をその出力に置き換えるため、指示は現在の diff がすでにインライン化された状態で到着します。
  </Step>

  <Step title="スキルをテストする">
    Git プロジェクトを開き、任意のファイルに小さな編集を加えて、`claude` を実行して Claude Code を起動します。スキルは 2 つの方法でテストできます。

    **説明に一致する内容を質問して Claude に自動的に呼び出させます。**

    ```text theme={null}
    What did I change?
    ```

    **またはスキル名で直接呼び出します。**

    ```text theme={null}
    /summarize-changes
    ```

    どちらの方法でも、Claude は編集内容の短い要約とリスクのリストで応答する必要があります。
  </Step>
</Steps>

<h2 id="where-skills-live">
  スキルの読み込み場所を選択する
</h2>

スキルを保存する場所によって、どのセッションがそれを読み込むかが決まります。ホームディレクトリに保存すると、すべてのプロジェクトで読み込まれます。リポジトリにコミットすると、そこで作業するすべてのユーザーと共有できます。プラグインまたはマネージドセッティングを通じて配布すると、チーム全体に到達します。

| 場所                   | パス                                                                                                                   | 読み込まれる場所                                                                                                                                                                               |
| :------------------- | :------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Enterprise           | `.claude/skills/<skill-name>/SKILL.md` in the [managed settings directory](/docs/ja/managed-settings#delivery-mechanisms) | 組織がデプロイするマシン上のすべてのユーザー                                                                                                                                                                 |
| Personal             | `~/.claude/skills/<skill-name>/SKILL.md`                                                                             | このマシン上のすべてのプロジェクト。ただし [Cowork またはクラウドセッション](#skills-in-cowork-and-cloud-sessions) は除く                                                                                                  |
| Project              | `.claude/skills/<skill-name>/SKILL.md`                                                                               | このリポジトリ内のセッション。コミットするとチームも取得できます                                                                                                                                                       |
| Nested               | `<subdir>/.claude/skills/<skill-name>/SKILL.md`                                                                      | `<subdir>` で開始されたセッション、またはその下で開始されたセッション。その上で開始されたセッションは、Claude がそこのファイルで作業を開始すると、スキルを 1 回読み込みます。[monorepos と subdirectories](#discovery-from-parent-and-nested-directories) を参照してください |
| Additional directory | `.claude/skills/<skill-name>/SKILL.md` in a directory you pass with `--add-dir`                                      | そのセッション。[プロジェクト外のディレクトリ](#skills-from-additional-directories) を参照してください                                                                                                                |
| Plugin               | `<plugin>/skills/<skill-name>/SKILL.md`                                                                              | [プラグイン](/docs/ja/plugins/overview) が有効な場所。`/plugin-name:skill-name` として                                                                                                                     |
| claude.ai account    | claude.ai アカウント用に有効化されたスキル                                                                                           | Cowork セッション、クラウドセッション、およびそのアカウントでサインインするターミナルセッション。[claude.ai から同期されたスキル](#how-synced-skills-behave) を参照してください                                                                        |

スキルフォルダは、これらのルールにも従います：

* **シンボリックリンク付きフォルダ**: enterprise、personal、または project の場所の `<skill-name>` エントリは、ディスク上の別の場所へのシンボリックリンクにすることができます。Claude Code は、複数の場所が同じターゲットを指している場合でも、ターゲットから `SKILL.md` を読み込み、スキルを 1 回だけ読み込みます。プラグインスキルは [シンボリックリンクを異なる方法で処理します](/docs/ja/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)。
* **予約名**: スキルフォルダに `synced` という名前を付けないでください。大文字小文字は問いません。Claude Code は `~/.claude/skills/synced/` を [claude.ai からダウンロードされたスキル](#where-synced-skills-load) に使用し、enterprise、personal、および project の場所でこの名前で作成したスキルをスキップします。
* **コマンドファイル**: `.claude/commands/` 内の Markdown ファイルは古い形式ですが、まだ機能します。`name` と `paths` を除く同じ [frontmatter](#frontmatter-reference) をサポートしています。それを呼び出すために入力するコマンド名を見つけるには、[スキルがコマンド名を取得する方法](#how-a-skill-gets-its-command-name) を参照してください。新しい作業にはスキルを使用してください。スキルは [サポートファイル](#add-supporting-files) もサポートしているためです。
* **プラグインとしてのスキルフォルダ**: `.claude-plugin/plugin.json` をスキルフォルダに追加すると、`<name>@skills-dir` という名前の [プラグイン](/docs/ja/plugins/loading#plugins-shared-through-a-repository) として読み込まれます。これにより、エージェント、hooks、および MCP サーバーをバンドルできます。プロジェクトの `.claude/skills/` では、最初にワークスペーストラストダイアログを受け入れる必要があります。

<h3 id="discovery-from-parent-and-nested-directories">
  monorepos と subdirectories でスキルを読み込む
</h3>

Claude Code は、開始したディレクトリと、リポジトリルートまでのすべての親ディレクトリの `.claude/skills/` からプロジェクトスキルを読み込みます。そのため、`packages/frontend/` で開始しても、ルートで定義されたスキルが取得されます。v2.1.246 以降で [`/cd` でセッションを移動する](/docs/ja/permissions#move-the-session-to-another-directory) と、Claude Code は新しいディレクトリのプロジェクトスキルを追加します。

リンクされた [git worktree](/docs/ja/worktrees) で実行されているセッションでは、Claude Code は worktree ルートまでのみ親ディレクトリを検索します。Claude Code v2.1.277 以降では、worktree チェックアウトのルートに `.claude/skills` ディレクトリがない場合、Claude Code はメインチェックアウトのプロジェクトスキルを代わりに読み込みます。[worktrees がメインチェックアウトと共有するもの](/docs/ja/worktrees#what-worktrees-share-with-the-main-checkout) を参照してください。

開始した場所の下の `.claude/skills/` ディレクトリ内のスキルは、起動時には読み込まれません。Claude がそのサブディレクトリ内のファイルを初めて読み込むか編集するときに読み込まれ、セッションの残りの間利用可能なままです。それまでは、`/` メニューに表示されず、名前で呼び出すことはできません。より早く読み込むには、サブディレクトリのパスで `/add-dir` を実行します。これには Claude Code v2.1.257 以降が必要です。

ネストされたスキルが別のスキルと同じ名前を共有する場合、両方が利用可能なままです。リポジトリルートに `deploy` スキルがあり、`apps/web/.claude/skills/` に別のスキルがある場合：

* `/deploy` はルートスキルを実行します。Claude Code は、Claude 用のディレクトリ修飾バリアントもリストし、作業しているファイルが存在するディレクトリのスキルを呼び出すように指示するため、ネストされたスキルは `apps/web/` での作業に適用されます。
* `/apps/web:deploy` はネストされたスキルを単独で実行します。その説明は、それが適用されるディレクトリに名前を付けます。

<h3 id="skills-from-additional-directories">
  プロジェクト外のディレクトリからスキルを読み込む
</h3>

`--add-dir` または `/add-dir` でディレクトリを追加すると、Claude Code はそのディレクトリの `.claude/skills/` 内のスキルを、その `.claude/commands/` および `.claude/agents/` とともに読み込みます。Agent SDK が TypeScript の [`additionalDirectories`](/docs/ja/agent-sdk/typescript#options) または Python の [`add_dirs`](/docs/ja/agent-sdk/python#claudeagentoptions) を通じて追加するディレクトリは、SDK が `--add-dir` として渡すため、同じ方法で読み込まれます。`settings.json` の `permissions.additionalDirectories` セッティングはファイルアクセスのみを付与し、これらのいずれも読み込みません。

Claude Code は、起動時に `--add-dir` で渡したディレクトリの `.claude/skills/` を監視します。[セッション中にスキルを編集する](#live-change-detection) で説明されています。追加されたディレクトリの `.claude/commands/` または `.claude/agents/` は監視しないため、そこでファイルを変更した後はセッションを再開してください。

これらの読み込みは、デフォルトで有効な `project` [セッティングソース](/docs/ja/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) に依存します。[`strictPluginOnlyCustomization`](/docs/ja/settings-reference#strictpluginonlycustomization) ポリシー、[bare mode](/docs/ja/headless#start-faster-with-bare-mode)、および [`--safe-mode`](/docs/ja/cli-reference#cli-flags) はそれぞれ、これらのページで説明されているようにさらに制限します。追加されたディレクトリが読み込むもの（`CLAUDE.md` およびプラグインセッティングを含む）の完全なテーブルについては、[追加ディレクトリはファイルアクセスを付与し、設定は付与しない](/docs/ja/permissions#additional-directories-grant-file-access-not-configuration) を参照してください。

<h3 id="resolve-skills-that-share-a-name">
  同じ名前のスキルを解決する
</h3>

2 つのスキルが同じ名前を共有する場合、各スキルがどこから来たかが、`/name` が実行するスキルを決定します。テーブルは、enterprise、personal、project、nested、plugin、および claude.ai の場所、バンドルされたスキル、およびコマンドファイルをカバーしています：

| 同じ名前の場所                                                         | どのスキルが実行されるか                                                                                                                                               |
| :-------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| enterprise、personal、および project の 2 つ                           | Enterprise が personal より優先され、personal が project より優先されます。`~/.claude/skills/` とプロジェクトの `.claude/skills/` の両方に `deploy` がある場合、`/deploy` は personal スキルを実行します |
| これらの場所のいずれかと [バンドルされたスキル](#bundled-skills)                      | あなたのスキルがバンドルされたコマンドを置き換えますが、そのエイリアスは置き換えません。プロジェクト `code-review` スキルは `/code-review` を置き換え、バンドルされたエイリアス `/review` はあなたのスキルを実行しません                          |
| スキルと `.claude/commands/` 内のファイル                                 | スキル                                                                                                                                                        |
| プロジェクトルートスキルとネストされたスキル                                          | 両方が読み込まれます。[monorepos と subdirectories](#discovery-from-parent-and-nested-directories) を参照してください                                                           |
| プラグインスキルと上記の場所のいずれかのスキル                                         | プラグインスキルは `/plugin-name:skill-name` として名前空間化されているため、両方が読み込まれます                                                                                             |
| 上記のいずれかと [claude.ai アカウントから同期されたスキル](#how-synced-skills-behave) | 他のスキルまたはコマンド。同期されたスキルは `/anthropic-skills:<name>` として実行されます。[同期されたスキル名が別のコマンドと一致する場合](#when-a-synced-skill-name-matches-another-command) を参照してください         |

<h3 id="skills-in-cowork-and-cloud-sessions">
  Cowork およびクラウドセッションでスキルを使用する
</h3>

[Cowork](https://claude.com/product/cowork) セッションおよび [クラウドセッション](/docs/ja/cloud-environments#what-carries-over-from-your-setup)（[routines](/docs/ja/routines) を含む）は、マシン上の `~/.claude/skills/` を読み込みません。インタラクティブおよびスケジュール済み Cowork セッションの両方は、claude.ai アカウント用に有効化されたスキルを読み込みます。これらはセッション開始時に同期されます。Desktop アプリサイドバーの **Customize** またはclaude.ai のスキルセッティングから管理します。クラウドセッションは、さらにクローンされたリポジトリの `.claude/skills/` にコミットされたプロジェクトスキルを読み込みます。

スキルがマシン上の `~/.claude/skills/` にのみ存在する場合、[routine](/docs/ja/routines) がそれを呼び出すと、Claude Code はスキルが見つからないと報告します。各 routine 実行は新しいクラウドセッションとして開始されるためです。これらのセッションで personal スキルを利用可能にするには：

* Cowork およびクラウドセッションの場合、claude.ai アカウント用にスキルを有効化します。
* クラウドセッションの場合、代わりにスキルをリポジトリの `.claude/skills/` にコミットできます。リポジトリの `.claude/settings.json` で宣言されたプラグインおよびユーザーセッティングでのみ有効化されたプラグインは [クラウドセッションで読み込まれません](/docs/ja/cloud-environments#what-carries-over-from-your-setup)。

[Desktop scheduled tasks](/docs/ja/desktop-scheduled-tasks) はマシン上でローカルに実行されるため、`~/.claude/skills/` を読み込みます。

<h3 id="how-synced-skills-behave">
  claude.ai から同期されたスキル
</h3>

このセクションは、Cowork またはクラウドセッションを使用する場合、またはターミナルで claude.ai アカウントで Claude Code にサインインする場合に適用されます。これらのセッションでは、Claude Code は claude.ai アカウント用に有効化されたスキルを読み込みます。セットアップは不要です。[同期されたスキルが読み込まれる場所](#where-synced-skills-load) で説明されています。これらのスキルには、claude.ai セッティングで作成または有効化したスキル、組織がそこで提供するスキル、および `pdf` や `xlsx` などの Anthropic の組み込みスキルが含まれます。

Claude Code は、セッションが実行されるマシンで作成したファイルを読み込むのではなく、アカウントから同期されたスキルをダウンロードするため、[スキルの場所](#where-skills-live) に保存するスキルには適用されないルールを同期されたスキルに適用します。

<h4 id="where-synced-skills-load">
  同期されたスキルが読み込まれる場所
</h4>

Cowork またはクラウドセッションでは、Claude Code は claude.ai アカウント用に有効化されたスキルを読み込みます。[Cowork およびクラウドセッションでのスキル](#skills-in-cowork-and-cloud-sessions) は、これらのセッションが取得するスキルを選択する方法を説明しています。

ターミナルでは、Claude Code は claude.ai アカウントでサインインするセッションでこれらのスキルを同期します。セッションが開始されると、Claude Code はアカウントのスキルを `~/.claude/skills/synced/` にバックグラウンドでダウンロードし、セッション実行中は約 10 分ごとに claude.ai の変更をチェックします。チェックでスキルが claude.ai で追加、編集、または無効化されたことが判明すると、Claude Code は実行中のセッションでそれを追加、更新、または削除します。再起動は不要です。ターミナルセッションでの同期には Claude Code v2.1.273 以降が必要です。

同期は起動を遅延させません。Claude がスキルを呼び出すときにのみ、スキルのダウンロードを待つためです。短い [非インタラクティブ](/docs/ja/headless) 実行は、新しく追加されたスキルがダウンロードされる前に終了する可能性があります。その場合、後のセッションがそれをダウンロードします。非インタラクティブ実行がスキルをダウンロードし、プロンプトに答える前にリストを待つようにするには、[`CLAUDE_CODE_SYNC_SKILLS`](/docs/ja/env-vars#variables) を `1` に設定します。

Claude Code は、claude.ai アカウントでサインインし、[Anthropic からフィーチャーフラグを取得する](/docs/ja/env-vars#features-that-need-feature-flag-fetching) セッションでのみ同期します。これらのセッションでは同期しません：

* `/login` で保存されたサインインを使用しないセッション。例えば、API キーで認証するセッション、または `ANTHROPIC_AUTH_TOKEN`、`CLAUDE_CODE_OAUTH_TOKEN`、または `apiKeyHelper` スクリプトが認証情報を提供するセッション
* フィーチャーフラグを取得しないセッション。例えば、Amazon Bedrock 上のセッション、または `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` を設定したセッション
* [bare mode](/docs/ja/headless#start-faster-with-bare-mode) のセッション、または `--safe-mode` で開始したセッション
* 組織のマネージドセッティングが [スキルをプラグインソースにロック](/docs/ja/settings-reference#strictpluginonlycustomization-skills) するセッション、または `user` を除外する [`--setting-sources`](/docs/ja/cli-reference#cli-flags) リストで開始したセッション

セッション中に `/login` でサインインする場合、Claude Code を再起動して同期を開始します。

以前のセッションが同期したスキルはディスク上に残ります。Claude Code は、同じアカウントにサインインした後のセッションでそれらを読み込みます。claude.ai に到達できない場合でも同様です。

Claude Code は同期されたスキルをダウンロードし、アップロードしません。`~/.claude/skills/synced/` の下のファイルを編集する場合、変更は claude.ai アカウントに保存されず、後の同期で上書きまたは削除される可能性があります。同期されたスキルを変更するには、claude.ai で更新します。次の同期で新しいバージョンがダウンロードされます。

同期されたスキルを確認するには、`/skills` を実行します。メニューは `claude.ai sync` の下にそれらをリストします。

`pdf` や `xlsx` などの Anthropic のスキルの一部は常に同期します。その他については、claude.ai のスキルセッティングでスキルをオンまたはオフにして、同期するかどうかを変更します。

マシンでの同期を停止するには、ユーザーセッティングで [`syncClaudeAiSkills`](/docs/ja/settings-reference#syncclaudeaiskills) を `false` に設定します。Claude Code はダウンロードを停止し、次回起動時に既に同期したスキルを `~/.claude/skills/.trash/` に移動し、それ以上読み込みません。組織は claude.ai で Skills をオフにすることで、すべてのユーザーの同期をオフにできます。同期を停止しながら Skills をオンのままにするには、[マネージドセッティング](/docs/ja/managed-settings) で同じキーを設定できます。

組織が claude.ai で Skills をオフにする場合、Claude Code はダウンロードされたスキルを削除し、読み込みを停止します。削除されたスキルは `~/.claude/skills/.trash/` に移動されます。[保持期間スイープ](/docs/ja/claude-directory#cleaned-up-automatically) がそれらを削除するまで、ファイルを復旧できます。組織が Skills を再度オンにすると、Claude Code は次の同期で有効化したスキルをダウンロードします。

<h4 id="when-a-synced-skill-name-matches-another-command">
  同期されたスキル名が別のコマンドと一致する場合
</h4>

同期されたスキルは、その完全な名前 `/anthropic-skills:<name>` または短い名前 `/<name>` で呼び出すことができます。別のコマンドがその短い名前を使用する場合、`/<name>` は他のコマンドを実行し、同期されたスキルは `/anthropic-skills:<name>` としてのみ実行されます。ローカル `deploy` スキルと同期された `deploy` がある場合、`/deploy` はローカルスキルを実行し、`/anthropic-skills:deploy` は同期されたスキルを実行します。v2.1.269 より前では、同期されたスキルは短い名前のみを持っていました。

他のコマンドは、これらのいずれかにすることができます：

* 組み込みコマンドまたは [バンドルされたスキル](#bundled-skills)。例えば、バンドルされたスキルをオフにした後に利用できないもの
* [ローカルレベル](#where-skills-live) のスキルまたは `.claude/commands/` 内のファイル
* プラグインスキル
* [MCP プロンプト](/docs/ja/mcp#use-mcp-prompts-as-commands)

Claude Code は同期されたスキルにラベルを付けるため、どこから来たかを判断できます。`/skills` メニューと `/context` は同期されたスキルを `claude.ai sync` の下にグループ化し、`/` コマンドメニューは claude.ai から来たものとしてマークします。

名前を比較するとき、Claude Code は大文字小文字、スペース、および目に見えない文字を無視し、全幅文字とダッシュバリアントなどの互換性形式をそれらのプレーン等価物として扱います。例えば、`Commit` という名前の同期されたスキルとローカル `commit` スキルは同じ名前と見なされるため、`/commit` はローカルスキルを実行し続けます。

別のアルファベットの見た目が似た文字だけで異なる名前は異なる名前と見なされ、`claude.ai sync` ラベルは 2 つを区別する方法です。これらのチェックとラベルには Claude Code v2.1.228 以降が必要です。

<h4 id="how-claude-code-handles-the-frontmatter-of-a-synced-skill">
  Claude Code が同期されたスキルの frontmatter をどのように処理するか
</h4>

Claude Code は、同期されたスキルの frontmatter に 2 つのルールを適用します：

* Claude Code は、あらゆる種類のセッションで frontmatter を尊重するため、`allowed-tools` グラントは通常の [権限フロー](/docs/ja/permissions) を通じて進みます。
* Claude Code は、スキルが提供する表示テキスト（説明など）をサニタイズします。制御文字を削除し、Claude に到達するテキスト（説明など）では、テキストが Claude Code の内部フォーマットを模倣できないように、角括弧をエスケープします。このサニタイズには Claude Code v2.1.228 以降が必要です。

<h4 id="how-claude-code-handles-the-body-of-a-synced-skill">
  Claude Code が同期されたスキルの本体をどのように処理するか
</h4>

Claude Code が同期されたスキルの本体で何をするかは、セッションが実行される場所によって異なります：

* クラウドセッションでは、本体はローカルスキルが持つ動作を保持します。セッションは分離されたコンテナで実行されるためです。
* デスクトップ上の Cowork セッションでは、本体はローカルスキルが持つ動作を保持します。ただし、Claude Code はすべての `!` コマンドラインを [`disableSkillShellExecution` プレースホルダ](#inject-dynamic-context) に置き換えます。すべてのスキルに対して行うように、そこで提供します。
* マシン上の他のセッションでは、Claude Code は [`!` コマンド](#inject-dynamic-context) を実行しません。`@` 参照が名前を付けるファイルをローカルスキルの場合のように添付しません。`${CLAUDE_PROJECT_DIR}` および `${CLAUDE_SESSION_ID}` プレースホルダを置き換えません。そのため、`@` 参照と両方のプレースホルダは Claude にリテラルテキストとして到達します。`!` コマンドラインも `disableSkillShellExecution` がオンの場合、リテラルテキストまたはそのプレースホルダとして到達します。この処理には Claude Code v2.1.228 以降が必要です。

<h3 id="live-change-detection">
  セッション中にスキルを編集する
</h3>

Claude Code は [bare mode](/docs/ja/headless#start-faster-with-bare-mode) を除き、スキルディレクトリのファイル変更を監視します。`~/.claude/skills/`、プロジェクト `.claude/skills/`、または `--add-dir` ディレクトリ内の `.claude/skills/` の下のスキルを追加、編集、または削除すると、Claude Code は現在のセッション内で変更を取得します。再起動は不要です。セッション開始時に存在しなかったトップレベルスキルディレクトリを作成する場合、Claude Code を再起動して新しいディレクトリを監視できるようにします。

ライブ変更検出は `SKILL.md` テキストのみをカバーします。スキルフォルダが [プラグイン](/docs/ja/plugins/loading#plugins-shared-through-a-repository) でもある場合、`hooks/`、`.mcp.json`、`agents/`、および `output-styles/` への変更は `/reload-plugins` で有効になります。

<h3 id="remove-a-skill">
  スキルを削除する
</h3>

スキルを削除する方法は、どこから来たかによって異なります：

* **Personal または project スキル**: スキルのディレクトリ `~/.claude/skills/<skill-name>/` または `.claude/skills/<skill-name>/` を削除します。Claude Code は [現在のセッションの `/skills` からそれを削除します](#live-change-detection)。Claude Code が既に読み込んだコンテンツは [スキルコンテンツライフサイクル](#skill-content-lifecycle) に従います。
* **Enterprise スキル**: 管理者が [マネージドセッティングディレクトリ](/docs/ja/managed-settings#delivery-mechanisms) 内の `.claude/skills/` からスキルのディレクトリを削除します。例えば、Linux では `/etc/claude-code/.claude/skills/<skill-name>/`。
* **Plugin スキル**: `/plugin` メニューから、または `/plugin uninstall <plugin-name>@<marketplace-name>` でプラグインを無効化またはアンインストールします。Claude Code は [変更が適用される](/docs/ja/plugins/cli-reference#reload-plugins) か、再起動するときにプラグインのスキルをアンロードします。
* **claude.ai から同期されたスキル**: claude.ai のスキルセッティングで、[有効化した](#skills-in-cowork-and-cloud-sessions) 同じ場所でスキルをオフにします。Claude Code は [スキルを同期する](#where-synced-skills-load) 次回にそれを `~/.claude/skills/synced/` から削除します。代わりに手動でディレクトリを削除する場合、次の同期はスキルが claude.ai で有効なままの間、それを再度ダウンロードします。
* **バンドルされたスキル**: [`disableBundledSkills`](#bundled-skills) を `true` に設定してバンドルされたスキルをオフにするか、[`skillOverrides`](#override-skill-visibility-from-settings) で 1 つのスキルを `"off"` に設定して非表示にします。

personal または project スキルを保持しながら Claude がそれを自動的に呼び出すのを停止するには、frontmatter で [`disable-model-invocation: true`](#control-who-invokes-a-skill) を設定するか、ファイルを編集したくない場合は [`skillOverrides`](#override-skill-visibility-from-settings) で `"user-invocable-only"` を設定します。

<h2 id="configure-skills">
  スキルを設定する
</h2>

スキルは `SKILL.md` の上部にある YAML フロントマターと、その後に続くマークダウンコンテンツを通じて設定されます。

<h3 id="types-of-skill-content">
  スキルコンテンツのタイプ
</h3>

スキルファイルには任意の指示を含めることができますが、それらをどのように呼び出したいかを考えることで、何を含めるべきかを決めるのに役立ちます。

**リファレンスコンテンツ** は、Claude が現在の作業に適用する知識を追加します。規約、パターン、スタイルガイド、ドメイン知識などです。このコンテンツはインラインで実行されるため、Claude は会話コンテキストと一緒にそれを使用できます。

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**タスクコンテンツ** は、デプロイメント、コミット、コード生成など、特定のアクションのステップバイステップの指示を Claude に提供します。これらは、Claude に自動的に実行させるのではなく、`/skill-name` で直接呼び出したいアクションであることが多いです。`disable-model-invocation: true` を追加して、Claude が自動的にトリガーするのを防ぎます。以下の例は `context: fork` を追加しており、これはスキルを独自のサブエージェントコンテキストで実行します。[サブエージェントでスキルを実行する](#run-skills-in-a-subagent) を参照してください。

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

本体自体は簡潔に保ちます。スキルが読み込まれると、そのコンテンツは [ターン全体でコンテキストに留まる](#skill-content-lifecycle) ため、すべての行は繰り返されるトークンコストになります。何をするかを述べ、どのように、なぜするかを説明するのではなく、[CLAUDE.md コンテンツ](/docs/ja/best-practices#write-an-effective-claude-md) に対して適用するのと同じ簡潔性テストを適用します。

<h3 id="frontmatter-reference">
  フロントマターリファレンス
</h3>

`SKILL.md` の上部にある `---` マーカー間の YAML [フロントマター](/docs/ja/glossary#frontmatter) でスキルを設定し、閉じる `---` の後にマークダウンとしてスキルの指示を記述します。フィールド名は `when_to_use` を除いてハイフンで区切られた小文字の単語を使用します。`.claude/commands/` の [コマンドファイル](#where-skills-live) は `name` と `paths` を除いて同じフィールドを受け入れます。この例は 4 つのフィールドを設定しています。

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

すべてのフィールドはオプションです。Claude がスキルをいつ使用するかを知るために、`description` のみが推奨されます。フィールド名はテーブルと正確に一致する必要があります。ハイフンを含めて、Claude Code は認識しないフィールドをエラーを報告せずに無視します。

Claude Code はフロントマターを読み込むのは、開く `---` がファイルの最初の行である場合のみです。そうでない場合、`---` マーカーを含むファイル全体をスキルコンテンツとして扱います。マーカー間の YAML が解析されない場合、スキルはフィールドセットなしで読み込まれます。[スキルがトリガーされない](#skill-not-triggering) を参照して、エラーを見つけて修正してください。

ブール値フィールドは、`true` と `false` に加えて、任意の大文字小文字で `yes`、`no`、`on`、`off`、`1`、`0` を受け入れます。v2.1.218 より前では、Claude Code は `true` と `false` のみを認識していました。

| フィールド                      | 必須  | 説明                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------- | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | いいえ | スキルリストに表示される表示名。ディレクトリ名がデフォルトです。スキルを呼び出すために入力する名前とフィールドがどのように相互作用するかについては、[スキルがコマンド名を取得する方法](#how-a-skill-gets-its-command-name) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `description`              | 推奨  | スキルが何をするか、いつ使用するか。Claude はこれを使用してスキルを適用するかどうかを決定します。省略された場合、マークダウンコンテンツの最初の空でない行を使用します。主要なユースケースを最初に配置します。結合された `description` と `when_to_use` テキストはコンテキスト使用量を削減するためにスキルリストで 1,536 文字で切り詰められます。                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `when_to_use`              | いいえ | Claude がスキルを呼び出すべき時期に関する追加コンテキスト。トリガーフレーズやリクエスト例など。スキルリストの `description` に追加され、1,536 文字の上限にカウントされます。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `argument-hint`            | いいえ | オートコンプリート中に表示されるヒント。予想される引数を示します。例：`[issue-number]` または `[filename] [format]`。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `arguments`                | いいえ | スキルコンテンツの [`$name` 置換](#available-string-substitutions) のための名前付き位置引数。スペース区切り文字列または YAML リストを受け入れます。名前は引数位置に順序でマップされます。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `disable-model-invocation` | いいえ | Claude がこのスキルを自動的に読み込むのを防ぐために `true` に設定します。`/name` で手動でトリガーしたいワークフローに使用します。また、スキルが [サブエージェントに事前読み込みされる](/docs/ja/sub-agents#preload-skills-into-subagents) のを防ぎます。v2.1.196 以降、スキルがプロンプトとして [スケジュール済みタスク](/docs/ja/scheduled-tasks) が発火したときに実行されるのも防ぎます。デフォルト：`false`。                                                                                                                                                                                                                                                                                                                                                                                       |
| `user-invocable`           | いいえ | Claude のみがスキルを呼び出すべき場合は `false` に設定します。Claude Code はそれを `/` メニューから非表示にし、`/name` を入力したときに実行しません。ユーザーが直接呼び出すべきではないバックグラウンド知識に使用します。デフォルト：`true`。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowed-tools`            | いいえ | このスキルを呼び出すターン中に Claude が許可を求めずに使用できるツール。許可はあなたが次のメッセージを送信するときにクリアされます。スペースまたはコンマ区切り文字列、または YAML リストを受け入れます。[スキルのツールを事前承認する](#pre-approve-tools-for-a-skill) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `disallowed-tools`         | いいえ | このスキルがアクティブな間、Claude の利用可能なプールから削除されるツール。バックグラウンドループの `AskUserQuestion` など、自律的なスキルが特定のツールを呼び出すべきではない場合に使用します。スペースまたはコンマ区切り文字列、または YAML リストを受け入れます。制限はあなたが次のメッセージを送信するときにクリアされます。拒否ルールと同様に、他のツールが残っている間、フィールドは [`EndConversation`](/docs/ja/tools-reference#endconversation-tool-behavior) を削除できません。                                                                                                                                                                                                                                                                                                                                                     |
| `model`                    | いいえ | このスキルがアクティブな場合に使用するモデル。オーバーライドは現在のターンの残りに適用され、設定に保存されません。セッションモデルは次のプロンプトを送信するときに再開されます。[`/model`](/docs/ja/model-config) と同じ値、または `inherit` を受け入れてアクティブなモデルを保持します。組織の [`availableModels`](/docs/ja/model-config#restrict-model-selection) 許可リストで除外された値は使用されず、セッションは現在のモデルを保持します。[自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) では、および [分類器がコマンドをレビューしている間の計画モード](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode) では、自動モードがサポートしないモデルも使用されず、セッションは現在のモデルを保持します。`context: fork` では、値は [フォークされたサブエージェントのモデル](#run-skills-in-a-subagent) を設定し、除外された値は [サブエージェントモデルオーバーライドと同じルール](/docs/ja/model-config#restrict-model-selection) に従います。 |
| `effort`                   | いいえ | このスキルがアクティブな場合の [努力レベル](/docs/ja/model-config#adjust-effort-level)。セッション努力レベルをオーバーライドします。デフォルト：セッションから継承。オプション：`low`、`medium`、`high`、`xhigh`、`max`。利用可能なレベルはモデルに依存します。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `context`                  | いいえ | フォークされたサブエージェントコンテキストで実行するために `fork` に設定します。[サブエージェントでスキルを実行する](#run-skills-in-a-subagent) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `agent`                    | いいえ | `context: fork` が設定されている場合に使用するサブエージェントタイプ。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `background`               | いいえ | `context: fork` にのみ適用されます。スキルを呼び出すターンでフォークされたサブエージェントの結果を待つために `false` に設定します。[バックグラウンドで実行する](#run-skills-in-a-subagent) のではなく。デフォルト：`true`。Claude Code v2.1.218 以降が必要です。                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `hooks`                    | いいえ | Claude Code がスキルが呼び出されたときに登録し、セッションの残りの間実行し続けるフック。設定形式と `once` オプションについては、[スキルとエージェントのフック](/docs/ja/hooks#hooks-in-skills-and-agents) を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `paths`                    | いいえ | このスキルがアクティブ化される時期を制限する Glob パターン。コンマ区切り文字列または YAML リストを受け入れます。設定されている場合、Claude はパターンに一致するファイルで作業している場合にのみ自動的にスキルを読み込みます。[パス固有のルール](/docs/ja/memory#path-specific-rules) と同じ形式を使用します。                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `shell`                    | いいえ | このスキルの `` !`command` `` および ` ```! ` ブロックに使用するシェル。`bash`（デフォルト）または `powershell` を受け入れます。`powershell` を設定すると、[PowerShell ツール](/ja/tools-reference#powershell-tool) が有効な場合、PowerShell 経由でインラインシェルコマンドを実行します。Windows では Git Bash なしでデフォルトでオン、Git Bash では claude.ai および Console アカウントでデフォルトでオン、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry セッション、および macOS、Linux、WSL では `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` が必要です。ツールをオフにするには `0` に設定します。                                                                                                                                                                                           |
| `metadata`                 | いいえ | 権限またはカタログフィールドなど、独自のキーと値のデータ用の自由形式の YAML マップ。`SKILL.md` から独自のツーリングで読み取られます。Claude Code はその内容に対して動作せず、マップではない値を削除します。`paths` などのフロントマターフィールド名をキーとして再利用しないでください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `license`                  | いいえ | スキルをカバーするライセンス。[Agent Skills](https://agentskills.io) 仕様の一部。[Claude Code 外でスキルフロントマターを使用する](#using-skill-frontmatter-outside-claude-code) を参照してください。Claude Code はフィールドを受け入れますが、それに対して動作しません。                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `compatibility`            | いいえ | 対象製品やシステム前提条件など、スキルの環境要件。[Agent Skills](https://agentskills.io) 仕様で定義されています。[Claude Code 外でスキルフロントマターを使用する](#using-skill-frontmatter-outside-claude-code) を参照してください。最大 500 文字の文字列を受け入れます。Claude Code はフィールドを受け入れますが、それに対して動作しません。                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h4 id="using-skill-frontmatter-outside-claude-code">
  Claude Code 外でスキルフロントマターを使用する
</h4>

Claude Code はテーブル上のすべてのフィールドを受け入れます。Claude Code 外では、[Agent Skills](https://agentskills.io) 仕様のフィールドのみを使用できます。

| 配布パス                                                                                                                        | 使用できるフロントマターフィールド                                                         |
| :-------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| [任意のレベル](#where-skills-live) の Claude Code スキル。[プラグイン](/docs/ja/plugins/overview) スキルを含む                                         | テーブル上のすべてのフィールド                                                           |
| claude.ai スキルアップロード、Skills API、および [anthropics/skills](https://github.com/anthropics/skills) の `package_skill.py` でのパッケージング | `name`、`description`、`license`、`compatibility`、`metadata`、`allowed-tools` |

たとえば、[Cowork とクラウドセッション](#skills-in-cowork-and-cloud-sessions) とルーチンで使用するために個人スキルを claude.ai アカウント用に有効にする場合、それを claude.ai にアップロードするため、同じルールが適用されます。

仕様が許可しないフィールドを含める場合、パッケージングまたはアップロードはフィールドを無視する代わりにハードエラーで失敗します。

```
Unexpected key(s) in SKILL.md frontmatter: argument-hint. Allowed properties are: allowed-tools, compatibility, description, license, metadata, name
```

フロントマターを仕様の 6 つのフィールドに制限すると、上記の予期しないキーエラーを回避できます。[Agent Skills 仕様](https://agentskills.io) および [Skills API 要件](https://docs.claude.com/en/api/skills-guide) は、これらのパスが検証するその他すべてを定義します。[動的コンテキスト注入](#inject-dynamic-context) などの Claude Code のみの本体機能は、claude.ai チャットまたは API を通じて機能しません。Claude Code はすべての 6 つのフィールドを受け入れるため、仕様に従うフロントマターは変更なしに Claude Code で読み込まれます。

<h4 id="how-a-skill-gets-its-command-name">
  スキルがコマンド名を取得する方法
</h4>

スキルを呼び出すために入力するコマンドは、スキルファイルが存在する場所から、およびプラグインスキルの場合はフロントマター `name` フィールドからも来ます。個人またはプロジェクトスキルでは、`name` はスキルリストに表示される表示ラベルのみを設定し、コマンドはディレクトリ名から来ます。プラグインスキルでは、`name` はコマンドの最後のセグメントを設定し、プラグインプレフィックスは所定の位置に留まります。

以下のテーブルは、各レイアウトのコマンド名がどこから来るかを示しています。

| スキルの場所                                                                | コマンド名のソース                                                     | 例                                                                                                               |
| :-------------------------------------------------------------------- | :------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------- |
| `~/.claude/skills/` または `.claude/skills/` の下のスキルディレクトリ                | ディレクトリ名                                                       | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                    |
| [ネストされた](#where-skills-live) `.claude/skills/` ディレクトリ。別のスキルと名前が衝突する場合 | 作業ディレクトリに相対的なサブディレクトリパス。その後、スキルディレクトリ名                        | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                  |
| `.claude/commands/` の下のファイル                                           | 拡張子なしのファイル名                                                   | `.claude/commands/deploy.md` → `/deploy`                                                                        |
| `.claude/commands/` のサブディレクトリ内のファイル                                   | `commands/` に相対的なサブディレクトリパス。各 `/` を `:` に置き換え。その後、拡張子なしのファイル名 | `.claude/commands/frontend/component.md` → `/frontend:component`                                                |
| プラグイン `skills/` サブディレクトリ                                              | フロントマター `name` またはディレクトリ名。プラグインでネームスペース化                      | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`。または `name: fancy` で `/my-plugin:fancy`                 |
| プラグインルート `SKILL.md`                                                   | フロントマター `name`。フォールバックとしてプラグインディレクトリ名                         | `my-plugin/SKILL.md` と `name: review` → `/my-plugin:review`。[プラグインルートの単一スキル](/docs/ja/plugins/components#skills) を参照 |
| [claude.ai から同期されたスキル](#how-synced-skills-behave)                     | claude.ai アカウント上のスキルの名前。`anthropic-skills:` でプレフィックス化         | アカウントスキル `deploy` → `/anthropic-skills:deploy`。または他のコマンドがその名前を使用していない場合は `/deploy`                              |

プラグインスキルでは、フロントマター `name` はコマンドの最後のセグメント内のディレクトリ名を置き換えるため、`my-plugin/skills/review/SKILL.md` と `name: fancy` は `/my-plugin:fancy` になります。別の `/fancy` もスキルを呼び出します。別のコマンドがその名前をまだ使用していない場合。書き込む `name` がプラグイン独自のプレフィックスで既に始まる場合、Claude Code は v2.1.246 以降でプレフィックスを再度追加しません。たとえば、`name: my-plugin:fancy` は依然として `/my-plugin:fancy` になります。v2.1.216 から v2.1.245 まで、Claude Code は `name` がそれを既に実行していた場合、プレフィックスを倍にしました。

[非対話型セッション](/docs/ja/headless) では、名前 `help` と `feedback` はターミナルのみの組み込みコマンド用に予約されていないため、これらの名前を持つプラグインスキルはそこで裸のコマンドを保持します。他のすべてのターミナルのみの組み込みの名前（`/login` など）は、これらのセッションでコマンドを実行できないにもかかわらず、予約されたままです。

プラグインルート `SKILL.md` の場合、スキルディレクトリがないため、`name` は最後のセグメント全体を提供します。`name` フィールドがない場合、Claude Code はプラグインのディレクトリ名にフォールバックします。

<h4 id="available-string-substitutions">
  利用可能な文字列置換
</h4>

スキルはスキルコンテンツの動的値の文字列置換をサポートしています。

| 変数                      | 説明                                                                                                                                                                                                                             |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | スキルを呼び出すときに渡されたすべての引数。プレースホルダーが引数を受け取らない場合、Claude Code は `ARGUMENTS: <value>` として追加します。[スキルに引数を渡す](#pass-arguments-to-skills) を参照してください。                                                                                       |
| `$ARGUMENTS[N]`         | 0 ベースのインデックスで特定の引数にアクセスします。例：最初の引数の場合は `$ARGUMENTS[0]`。                                                                                                                                                                        |
| `$N`                    | `$ARGUMENTS[N]` の短縮形。例：最初の引数の場合は `$0`、2 番目の引数の場合は `$1`。                                                                                                                                                                        |
| `$name`                 | [`arguments`](#frontmatter-reference) フロントマターリストで宣言された名前付き引数。名前は位置に順序でマップされるため、`arguments: [issue, branch]` では、プレースホルダー `$issue` は最初の引数に展開され、`$branch` は 2 番目に展開されます。                                                          |
| `${CLAUDE_SESSION_ID}`  | 現在のセッション ID。ログ、セッション固有のファイルの作成、またはスキル出力とセッションの相関に役立ちます。                                                                                                                                                                        |
| `${CLAUDE_EFFORT}`      | 現在の努力レベル：`low`、`medium`、`high`、`xhigh`、または `max`。Ultracode は個別のレベルではなく、`xhigh` として報告されます。これを使用して、アクティブな努力設定にスキル指示を適応させます。                                                                                                      |
| `${CLAUDE_SKILL_DIR}`   | スキルの `SKILL.md` ファイルを含むディレクトリ。プラグインスキルの場合、これはプラグインルートではなく、プラグイン内のスキルのサブディレクトリです。現在の作業ディレクトリに関係なく、スキルにバンドルされたスクリプトまたはファイルを参照するために bash インジェクションコマンドで使用します。                                                                      |
| `${CLAUDE_PROJECT_DIR}` | プロジェクトルートディレクトリ。これは [フック](/docs/ja/hooks#reference-scripts-by-path) と MCP サーバーが `CLAUDE_PROJECT_DIR` として受け取るのと同じパスです。`${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh` など、スキルがインストールされている場所に関係なく、プロジェクトローカルスクリプトまたはファイルを参照するために使用します。 |
| `${CLAUDE_PLUGIN_ROOT}` | プラグインのインストールディレクトリ。プラグインスキルでのみ置換されます。プラグイン内の任意の場所にバンドルされたスクリプトまたはファイル（プラグインのスキル間で共有されるリソースを含む）を参照するために使用します。[プラグイン環境変数](/docs/ja/plugins/manifest-reference#environment-variables) を参照してください。                                       |
| `${CLAUDE_PLUGIN_DATA}` | プラグインの [永続データディレクトリ](/docs/ja/plugins/components#path-variables-and-persistent-data)。プラグイン更新を生き残ります。プラグインスキルでのみ置換されます。インストール済みの依存関係、生成されたファイル、または更新を超えて存続する必要があるキャッシュを参照するために使用します。                                                 |

Claude Code は `${CLAUDE_SKILL_DIR}` と `${CLAUDE_PROJECT_DIR}` を 2 つの場所で置換します。スキルのマークダウンコンテンツ、および [`allowed-tools`](#frontmatter-reference) フロントマターの Bash ルール。プラグインスキルでは、Claude Code は `${CLAUDE_PLUGIN_ROOT}` と `${CLAUDE_PLUGIN_DATA}` を同じ 2 つの場所で置換します。両方の場所で同じ変数を使用すると、スキルは許可プロンプトなしでバンドルされたスクリプトを実行できます。以下のスキルはパターンを示しています。

```yaml theme={null}
---
name: render-chart
description: Render a chart from a CSV file
allowed-tools: Bash(${CLAUDE_SKILL_DIR}/scripts/render.sh *)
---

Run `${CLAUDE_SKILL_DIR}/scripts/render.sh <csv-file>` to render the chart.
```

このスキルが `~/.claude/skills/render-chart/` にインストールされている場合、`${CLAUDE_SKILL_DIR}` の両方の出現はそのディレクトリに展開されます。`allowed-tools` ルールはスキル本体が Claude に実行するよう指示する正確なコマンドと一致するため、スクリプトはプロンプトなしで実行されます。

`${CLAUDE_PROJECT_DIR}` 置換には Claude Code v2.1.196 以降が必要です。

インデックス付き引数はシェルスタイルのクォートを使用するため、複数単語の値をクォートで囲んで単一の引数として渡します。たとえば、`/my-skill "hello world" second` は `$0` を `hello world` に展開し、`$1` を `second` に展開します。`$ARGUMENTS` プレースホルダーは常に入力されたとおりの完全な引数文字列に展開されます。

対応する引数がないインデックス付きプレースホルダー（1 つの引数のみが渡された場合の `$2` など）はコンテンツで変更されないままです。[`arguments`](#frontmatter-reference) フロントマターからの対応する引数がない名前付きプレースホルダーは空の文字列に展開されます。

`$1` または `$ARGUMENTS` などのテキストを含む引数値を渡す場合、Claude Code はそれをリテラルテキストとして挿入し、展開しません。たとえば、スキルの本体に `Summarize $0` が含まれており、`/summarize "$ARGUMENTS from yesterday"` を実行する場合、Claude は `Summarize $ARGUMENTS from yesterday` を受け取ります。Claude Code は、引数を挿入した後、`${CLAUDE_SKILL_DIR}` などの `${CLAUDE_*}` 変数を置換します。

数字、`ARGUMENTS`、または宣言された引数名の前にリテラル `$` を含めるには（例：散文の `$1.00`）、バックスラッシュでエスケープします。`\$1.00`。他の `$` の前のバックスラッシュは変更されないままです。トークンの直前の単一バックスラッシュのみがそれをエスケープします。`\\$1` などの二重バックスラッシュは両方のバックスラッシュを所定の位置に残し、`$1` は依然として引数値に展開されます。バックスラッシュエスケープはこれらの引数プレースホルダーのみをカバーします。バックスラッシュは、変数が適用される `${CLAUDE_*}` 変数の置換を防ぎません。

**置換を使用した例：**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  サポートファイルを追加する
</h3>

スキルはディレクトリに複数のファイルを含めることができます。これにより、`SKILL.md` は本質的なものに焦点を当てることができ、Claude は必要に応じてのみ詳細なリファレンス資料にアクセスできます。大規模なリファレンスドキュメント、API 仕様、またはサンプルコレクションは、スキルが実行されるたびにコンテキストに読み込む必要はありません。

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

`SKILL.md` からサポートファイルを参照して、Claude が各ファイルに何が含まれているか、いつそれを読み込むかを知るようにします。

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>`SKILL.md` を 500 行以下に保ちます。詳細なリファレンス資料を別のファイルに移動します。</Tip>

<h3 id="control-who-invokes-a-skill">
  スキルを呼び出すユーザーを制御する
</h3>

デフォルトでは、あなたと Claude の両方がスキルを呼び出すことができます。`/skill-name` を入力して直接呼び出すことができ、Claude は会話に関連する場合に自動的に読み込むことができます。2 つのフロントマターフィールドでこれを制限できます。

* **`disable-model-invocation: true`**：あなたのみがスキルを呼び出すことができます。`/commit`、`/deploy`、`/send-slack-message` など、副作用があるワークフロー、またはタイミングを制御したいワークフローに使用します。コードが準備完了に見えるため、Claude がデプロイすることを望みません。

* **`user-invocable: false`**：Claude のみがスキルを呼び出すことができます。アクションとして実行できないバックグラウンド知識に使用します。`legacy-system-context` スキルは古いシステムがどのように機能するかを説明します。Claude はこれが関連する場合に知るべきですが、`/legacy-system-context` はユーザーが実行する意味のあるアクションではありません。

この例は、あなたのみがトリガーできるデプロイスキルを作成します。`disable-model-invocation: true` を設定した場合、Claude はスキルを自動的に実行できません。

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Claude が試みた場合、Claude Code は呼び出しをブロックし、デプロイステップを別の方法で再現しないよう指示するため、`/deploy` を自分で実行することを提案することを期待してください。

2 つのフィールドが呼び出しとコンテキスト読み込みにどのように影響するかは次のとおりです。

| フロントマター                          | あなたが呼び出せる | Claude が呼び出せる | コンテキストに読み込まれる時期                          |
| :------------------------------- | :-------- | :------------ | :--------------------------------------- |
| （デフォルト）                          | はい        | はい            | 説明は常にコンテキストにあり、呼び出されたときに完全なスキルが読み込まれます   |
| `disable-model-invocation: true` | はい        | いいえ           | 説明はコンテキストにはなく、あなたが呼び出したときに完全なスキルが読み込まれます |
| `user-invocable: false`          | いいえ       | はい            | 説明は常にコンテキストにあり、呼び出されたときに完全なスキルが読み込まれます   |

<Note>
  通常のセッションでは、スキルの説明がコンテキストに読み込まれるため、Claude は何が利用可能かを知っていますが、完全なスキルコンテンツは呼び出されたときにのみ読み込まれます。[事前読み込みされたスキルを持つサブエージェント](/docs/ja/sub-agents#preload-skills-into-subagents) は異なります。完全なスキルコンテンツはスタートアップで注入されます。
</Note>

<h3 id="skill-content-lifecycle">
  スキルコンテンツのライフサイクル
</h3>

あなたまたは Claude がスキルを呼び出すと、レンダリングされた `SKILL.md` コンテンツが会話に単一のメッセージとして入力され、後のターン全体で留まります。この永続性はスキルの指示に適用され、その権限には適用されません。[`allowed-tools`](#pre-approve-tools-for-a-skill) 許可はあなたが次のメッセージを送信するときにクリアされます。Claude Code は後のターンでスキルファイルを再度読み込まないため、タスク全体に適用すべき指導を 1 回限りのステップではなく、スタンディング指示として記述します。

Claude が、レンダリングされたコンテンツがコンテキストに既にある複製と同じであるスキルを再呼び出すと、Claude Code はスキルが既に読み込まれていることを示す短いメモを追加します。コンテンツが異なる場合（引数が変更されたか、[動的コンテキスト](#inject-dynamic-context) コマンドが新しい出力を生成したため）、Claude Code は完全なコンテンツを再度追加します。

[自動コンパクション](/docs/ja/how-claude-code-works#when-context-fills-up) はトークン予算内で呼び出されたスキルを前方に実行します。会話が要約されてコンテキストを解放するとき、Claude Code は各スキルの最新の呼び出しを要約の後に再度アタッチし、最初の 5,000 トークンを保持します。再度アタッチされたスキルは 25,000 トークンの結合予算を共有します。Claude Code はこの予算を最近呼び出されたスキルから開始して埋めるため、セッション内で多くを呼び出した場合、古いスキルはコンパクション後に完全にドロップできます。

スキルが最初の応答の後に動作に影響を与えるのを停止しているように見える場合、コンテンツは通常まだ存在し、モデルは他のツールまたはアプローチを選択しています。スキルの `description` と指示を強化して、モデルがそれを優先し続けるようにするか、[フック](/docs/ja/hooks) を使用して動作を決定的に強制します。スキルが大きい場合、または他のスキルを呼び出した後、コンパクション後に再呼び出しして完全なコンテンツを復元します。

<h3 id="pre-approve-tools-for-a-skill">
  スキルのツールを事前承認する
</h3>

`allowed-tools` フィールドは、スキルを呼び出すターン中にリストされたツールの権限を付与するため、Claude は承認を求めることなくそれらを使用できます。許可はあなたが次のメッセージを送信するときにクリアされます。スキルコンテンツは [コンテキストに留まる](#skill-content-lifecycle) にもかかわらず。スキルを再呼び出すと、そのターンに対して再度適用されます。これはどのツールが利用可能かを制限しません。すべてのツールは呼び出し可能なままであり、[権限設定](/docs/ja/permissions) はリストされていないツールを引き続き管理します。セッション全体ではなく単一のターンのツールを事前承認するには、代わりにそれらの権限設定に許可ルールを追加します。

ワークスペーストラストはこのフィールドをゲートしません。Claude Code は、信頼したことのないフォルダで `-p` 実行を含む、あなたまたは Claude がスキルを呼び出すときはいつでも、プロジェクトスキルの `allowed-tools` を適用します。スキルは自身に広いツールアクセスを付与できるため、Claude Code をそこで実行する前に、リポジトリにチェックインされたスキルの `allowed-tools` を確認してください。

このスキルでは、スキルを呼び出すときはいつでも、Claude は許可を求めることなく git コマンドを実行できます。

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

スキルがアクティブな間、Claude の利用可能なプールからツールを削除するには、スキルのフロントマターの `disallowed-tools` にそれらをリストします。制限はあなたが次のメッセージを送信するときにクリアされます。拒否ルールと同様に、フィールドは他のツールが残っている間、[`EndConversation`](/docs/ja/tools-reference#endconversation-tool-behavior) を削除できません。すべてのスキルとプロンプト全体でツールをブロックするには、[権限設定](/docs/ja/permissions) に拒否ルールを追加します。

<h3 id="pass-arguments-to-skills">
  スキルに引数を渡す
</h3>

あなたと Claude の両方がスキルを呼び出すときに引数を渡すことができます。引数は `$ARGUMENTS` プレースホルダーを通じて利用可能です。

このスキルは GitHub の問題を番号で修正します。`$ARGUMENTS` プレースホルダーはスキル名の後に続くもので置き換えられます。

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

`/fix-issue 123` を実行すると、Claude は「Fix GitHub issue 123 following our coding standards...」を受け取ります。

スキルに引数を指定して呼び出しても、スキルのコンテンツのプレースホルダーが 1 つも受け取らない場合、Claude Code は `ARGUMENTS: <your input>` をスキルコンテンツの最後に追加するため、Claude は依然として入力したものを見ます。プレースホルダーは `$ARGUMENTS`、`$1` などのインデックス形式、または名前付き引数です。対応する引数がないインデックス付きプレースホルダーはリテラルテキストのままで、1 つを受け取ったとしてカウントされません。名前付きプレースホルダーは、位置に引数がない場合でも、空の文字列に展開されるため、カウントされます。

1 つのメッセージの開始時に複数のスキルをスタックすることもできます。`/write-tests /fix-issue 123` を入力すると、両方のスキルが読み込まれ、末尾のテキスト `123` が `$ARGUMENTS` として各スキルに渡されます。v2.1.199 より前では、最初のスキルのみが読み込まれ、`/fix-issue 123` をリテラル引数テキストとして受け取りました。

Claude Code は最初のスキルと、その後にスタックされた最大 5 つのスキルを展開します。展開は、インラインユーザー呼び出し可能スキルではない最初のトークンで停止するため、[フォークされたサブエージェント](#run-skills-in-a-subagent) として実行されるスキル（[`/code-review`](/docs/ja/code-review#review-a-diff-locally) など）、またはその引数自体がスラッシュコマンドで始まる可能性があるスキル（`/loop` など）も、そこで実行を終了します。そのトークンとその後のすべてが、展開されたすべてのスキルの引数テキストになります。v2.1.218 から `/code-review` はフォークされたサブエージェントとして実行されます。以前のバージョンではインラインで実行され、スタックされました。

位置で個別の引数にアクセスするには、`$ARGUMENTS[N]` または短い `$N` を使用します。

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

`/migrate-component SearchBar JavaScript TypeScript` を実行すると、`$ARGUMENTS[0]` が `SearchBar` に、`$ARGUMENTS[1]` が `JavaScript` に、`$ARGUMENTS[2]` が `TypeScript` に置き換えられます。`$N` 短縮形を使用する同じスキル。

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  高度なパターン
</h2>

<h3 id="inject-dynamic-context">
  動的コンテキストを注入する
</h3>

`` !`<command>` `` 構文は、スキルコンテンツが Claude に送信される前にシェルコマンドを実行します。コマンド出力がプレースホルダーを置き換えるため、Claude はコマンド自体ではなく実際のデータを受け取ります。Claude Code は、スキルが [claude.ai アカウントから同期される](#how-synced-skills-behave) 場合、マシン上でこれらのコマンドを実行しません。この制限には Claude Code v2.1.228 以降が必要です。

このスキルは GitHub CLI を使用してライブ PR データを取得することで、プルリクエストを要約します。`` !`gh pr diff` `` およびその他のコマンドが最初に実行され、その出力がプロンプトに挿入されます。

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

置換は元のファイルに対して 1 回実行されます。コマンド出力はプレーンテキストとして挿入され、さらに `` !`<command>` `` プレースホルダーについて再スキャンされないため、コマンドは後の処理で展開するプレースホルダーを出力することはできません。

インライン形式は、`!` が行の開始時または空白の直後に表示される場合にのみ認識されます。`` KEY=!`cmd` `` のように `!` が別の文字の後に続く場合、プレースホルダーはリテラルテキストとして残され、コマンドは実行されません。

複数行のコマンドの場合、インライン形式の代わりに ` ```! ` で開かれたフェンスコードブロックを使用します。

````markdown theme={null}
## Environment
```!
node --version
git status --short
```
````

ユーザー、プロジェクト、プラグイン、または [additional-directory](#skills-from-additional-directories) ソースからのスキルおよびカスタムコマンドについてこの動作を無効にするには、[settings](/docs/ja/settings) で `"disableSkillShellExecution": true` を設定します。各コマンドは実行される代わりに `[shell command execution disabled by policy]` に置き換えられます。バンドルされたスキルと管理されたスキルは影響を受けません。この設定は [managed settings](/docs/ja/managed-settings) で最も有用です。ここではユーザーはそれをオーバーライドできません。

Claude Code は、[claude.ai アカウントから同期されたスキル](#how-synced-skills-behave) に表示されるコマンドをマシン上で実行することはありません。この制限には Claude Code v2.1.228 以降が必要です。[Claude Code がどのように同期されたスキルの本体を処理するか](#how-claude-code-handles-the-body-of-a-synced-skill) は、各種セッションで Claude が受け取るコマンドの代わりになるものを説明しています。

<Tip>
  スキルが実行されるときにより深い推論をリクエストするには、スキルコンテンツの任意の場所に `ultrathink` を含めます。[1 回限りの深い推論に ultrathink を使用する](/docs/ja/model-config#use-ultrathink-for-one-off-deep-reasoning) を参照してください。
</Tip>

<h4 id="how-injected-commands-run">
  注入されたコマンドがどのように実行されるか
</h4>

Claude Code は、スキルのフロントマターの `shell` キーと環境からスキルの注入されたコマンドを実行するツールを選択します。すべての組み合わせはコマンドを Bash ツールまたは PowerShell ツールを通じて実行します。ただし、呼び出しを完全に失敗させる 1 つの組み合わせを除きます。

* `shell: powershell`、[PowerShell ツール](/docs/ja/tools-reference#powershell-tool) が有効な場合：コマンドは PowerShell ツールを通じて実行されます。
* `shell: bash` で bash が利用できない場合：コマンドが実行される前に呼び出しが失敗します。これは Git Bash のない Windows で発生します。Claude Code は ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found`` を表示します。
* その他の組み合わせ：bash が利用可能な場合、コマンドは Bash ツールを通じて実行されます。利用できない場合、PowerShell ツールを通じて実行されます。

どちらのツールも、Claude 独自のシェルコマンドを実行するのと同じ方法でコマンドを実行します。これらは作業ディレクトリ、タイムアウト、および出力処理を共有します。

* **作業ディレクトリ**：Claude Code は各コマンドをセッションシェルの現在の作業ディレクトリで実行します。Claude が `cd` を実行するとそのディレクトリが移動します。毎回同じ方法で解決する必要があるパスで [`${CLAUDE_SKILL_DIR}` または `${CLAUDE_PROJECT_DIR}`](#available-string-substitutions) を使用します。
* **stderr**：デフォルトの `bash` シェルでは、Claude Code は stderr を stdout にマージします。コマンドが stderr に書き込むすべてのものが注入されたテキストに表示されます。
* **タイムアウト**：各コマンドは Bash ツールのデフォルト 2 分 [timeout](/docs/ja/tools-reference#timeout-and-output-limits) の下で実行されます。Bash ツールが [タイムアウトしたコマンドをバックグラウンドに移動](/docs/ja/tools-reference#background-commands) する場合、スキルは引き続きレンダリングされます。注入されたテキストは移動を報告し、バックグラウンドタスクとコマンドの出力を収集するファイルに名前を付けます。コマンドが Bash ツールが自動的にバックグラウンドに移動しないコマンドの場合、Claude Code はタイムアウト時にそれを強制終了します。その失敗は [呼び出しを中止します](#when-an-injected-command-fails)。
* **出力サイズ**：Bash ツールのインライン上限を超える出力は、切り詰められたテキストではなく、ファイルパスと短いプレビューとして到着します。[出力制限](/docs/ja/tools-reference#output-limits) は上限とそれぞれの境界を調整する方法をカバーしています。

PowerShell ツールは、実行するコマンドに同じタイムアウト、バックグラウンド処理、および出力上限の動作を適用します。その詳細については、[PowerShell ツール](/docs/ja/tools-reference#powershell-tool) セクションを参照してください。

<h4 id="when-an-injected-command-fails">
  注入されたコマンドが失敗する場合
</h4>

失敗したコマンドは、スキル全体の呼び出しを中止します。独自のプレースホルダーだけではありません。Claude はその呼び出しのスキルコンテンツを見ることはありません。中止は `Shell command failed for pattern "..."` を表示します。エラーメッセージには `[stderr]` の下のコマンドの出力が含まれます。

デフォルトの `bash` シェルでは、ゼロ以外の終了コードはすべて失敗としてカウントされます。1 つの例外が適用されます。Claude Code は [検索および比較コマンド](/docs/ja/tools-reference#output-limits) からの終了コード 1 を通常の結果として扱い、その出力を注入します。終了コード 2 以上はこれらのコマンドでも失敗します。

どのコマンドが例外を取得するかはシェルによって異なります。

* デフォルト `bash` シェル：[出力制限](/docs/ja/tools-reference#output-limits) の下にリストされているコマンド
* `shell: powershell`、PowerShell ツールが有効な場合：`grep` と `git diff` を含むが `find` や `diff` は含まない [異なるセット](/docs/ja/tools-reference#shell-selection-in-settings-hooks-and-skills)

デフォルトの `bash` シェルでは、ゼロ以外で終了することが予想される他のコマンドに `|| true` を追加します。問題を見つけたときに 1 で終了するチェックスクリプトは 1 つの例です。

<h4 id="permission-checks-on-injected-commands">
  注入されたコマンドの権限チェック
</h4>

注入されたコマンドは、スキルのレンダリング中に権限を求めるプロンプトを表示することはありません。Claude Code は最初に [権限ルール](/docs/ja/permissions) に対して各コマンドをチェックします。deny ルールが一致するコマンドは `Shell command permission check failed for pattern "..."` で呼び出しを中止します。

[auto mode](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) の外では、コマンドの権限チェックが allow 以外のものを返す場合、Claude Code は同じエラーで呼び出しを中止します。これには通常あなたに尋ねるルールが含まれます。一致しないコマンドが中止されるのを防ぐには、[`allowed-tools`](#pre-approve-tools-for-a-skill) で事前に承認します。Deny および ask ルールは引き続き `allowed-tools` をオーバーライドします。[権限を管理する](/docs/ja/permissions#manage-permissions) を参照してください。

auto mode では、そうでなければあなたの承認が必要なコマンドは呼び出しを中止しません。スキルは Claude にコマンドを最初に実行するよう指示して読み込まれ、Claude 独自の呼び出しは [auto mode の通常のチェック](/docs/ja/permission-modes#how-the-classifier-evaluates-actions) を通じて進みます。呼び出しは、`agent` を設定する [フォークされたスキル](#run-skills-in-a-subagent) でも、Claude が [注入されたコマンドを実行するシェルツール](#how-injected-commands-run) を持たないセッションでも中止されます。

<h3 id="run-skills-in-a-subagent">
  スキルをサブエージェントで実行する
</h3>

スキルを分離して実行したい場合は、フロントマターに `context: fork` を追加します。Claude Code は `agent` フィールドで設定されたタイプの新しいサブエージェントを開始し、スキルコンテンツをそのプロンプトとして提供します。サブエージェントは会話履歴を見ないため、スキルの指示は独立して機能する必要があります。

<Note>
  名前にもかかわらず、`context: fork` を持つスキルは [現在の会話のフォーク](/docs/ja/sub-agents#fork-the-current-conversation) では実行されません。これはサブエージェントにこれまで議論したすべてを渡すでしょう。タスクがその履歴に依存する場合、`context: fork` を使用する代わりに会話をフォークします。
</Note>

フォークされたサブエージェントは [バックグラウンド](/docs/ja/sub-agents#run-subagents-in-foreground-or-background) で実行されます。スキルが実行されている間、あなたは作業を続けることができ、その結果は完了時に会話に到着します。フロントマターで `background: false` を設定して、代わりにスキルを呼び出した順番で結果を待ちます。v2.1.218 より前では、フォークされたスキルは常に完了するまでターンをブロックしていました。

Claude Code は、スキルが `background: false` を設定しない場合でも、次のような場合に結果を待ちます。

* `-p` フラグまたは Agent SDK を使用した非対話モード
* [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/ja/env-vars) を `1` に設定した場合。これはすべての他のバックグラウンドタスク機能もオフにします
* 同じスキルの以前の呼び出しがまだ実行中の間にフォークされたスキルを呼び出す場合
* [スケジュールされたタスク](/docs/ja/scheduled-tasks) がスキルをそのプロンプトとして発火する場合

バックグラウンドで実行されるフォークされたスキルは、[バックグラウンドサブエージェントに適用される狭いツールセット](/docs/ja/sub-agents#run-subagents-in-foreground-or-background) で編集を適用します。スキルのサブエージェントは通常のエージェントタイプなので、会話をフォークするサブエージェントの例外はそれをカバーしません。スキルのステップがそのセット外のツールに依存する場合、`background: false` を設定して完全なツールセットを保持します。

バックグラウンドで実行されるフォークされたスキルは、セッションの [checkpoints](/docs/ja/checkpointing) の外で編集を適用するため、`/rewind` はそれらを元に戻しません。git を使用してそれらを元に戻します。

<Warning>
  `context: fork` は明示的な指示を持つスキルにのみ意味があります。スキルに「これらの API 規約を使用する」などのガイドラインが含まれていて、タスクがない場合、サブエージェントはガイドラインを受け取りますが、実行可能なプロンプトはなく、意味のある出力なしで返されます。
</Warning>

スキルと [サブエージェント](/docs/ja/sub-agents) は 2 つの方向で連携します。

| アプローチ                     | システムプロンプト         | タスク             | また読み込む                                                                                         |
| :------------------------ | :---------------- | :-------------- | :--------------------------------------------------------------------------------------------- |
| `context: fork` を持つスキル    | エージェントタイプから       | SKILL.md コンテンツ  | CLAUDE.md、エージェントの [startup context](/docs/ja/sub-agents#what-loads-at-startup) に従う                  |
| `skills` フィールドを持つサブエージェント | サブエージェントのマークダウン本体 | Claude の委任メッセージ | 事前読み込みされたスキル + CLAUDE.md、サブエージェントの [startup context](/docs/ja/sub-agents#what-loads-at-startup) に従う |

`context: fork` では、スキルにタスクを記述し、それを実行するエージェントタイプを選択します。組み込みの Explore および Plan エージェントは [CLAUDE.md と git status をスキップ](/docs/ja/sub-agents#what-loads-at-startup) して、コンテキストを小さく保ちます。そのため、`agent: Explore` を使用するフォークされたスキルは SKILL.md コンテンツとエージェント独自のシステムプロンプトのみを見ます。逆に、参照資料としてスキルを使用するカスタムサブエージェントを定義する場合は、[サブエージェント](/docs/ja/sub-agents#preload-skills-into-subagents) を参照してください。

<h4 id="example-research-skill-using-explore-agent">
  例：Explore エージェントを使用した研究スキル
</h4>

このスキルはフォークされた Explore エージェントで研究を実行します。スキルコンテンツはタスクになり、エージェントはコードベース探索に最適化された読み取り専用ツールを提供します。

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

このスキルが実行される場合：

1. 新しい分離されたコンテキストが作成されます
2. サブエージェントはスキルコンテンツをそのプロンプトとして受け取ります（「Research \$ARGUMENTS thoroughly」指示）
3. `agent` フィールドは実行環境（モデル、ツール、および権限）を決定します
4. サブエージェントはその結果を要約し、完了時にメイン会話に返します

`agent` フィールドは使用するサブエージェント構成を指定します。オプションには組み込みエージェント（`Explore`、`Plan`、`general-purpose`）または `.claude/agents/` からのカスタムサブエージェントが含まれます。省略された場合、`general-purpose` を使用します。

<h3 id="restrict-claude’s-skill-access">
  Claude のスキルアクセスを制限する
</h3>

デフォルトでは、Claude は `disable-model-invocation: true` が設定されていないスキルを呼び出すことができます。`allowed-tools` を定義するスキルは、スキルを呼び出すターン中に、事前承認なしでこれらのツールへのアクセスを Claude に付与します。許可は次のメッセージを送信するときにクリアされます。[権限設定](/docs/ja/permissions) は引き続き、他のすべてのツールの基本的な承認動作を管理します。`/init` や `/security-review` を含むいくつかの組み込みコマンドも Skill ツールを通じて利用可能です。`/compact` などの他の組み込みコマンドはそうではありません。

Claude が呼び出すことができるスキルを制御する 3 つの方法：

**すべてのスキルを無効にする** には、`/permissions` で Skill ツールを deny します。

```text theme={null}
# Add to deny rules:
Skill
```

**特定のスキルを許可または拒否する** には [permission rules](/docs/ja/permissions) を使用します。

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

権限構文：正確な一致の場合は `Skill(name)`、任意の引数を持つプレフィックス一致の場合は `Skill(name *)`。

deny ルールがスキルの別名または修飾されていない名前ではなくスキル独自の名前を指定する場合、Claude Code はスキルをブロックします。`Skill(review)` でバンドルされた `/code-review` をその `/review` エイリアスを通じてブロックし、`Skill(deploy)` で [ネストされたスキル](#where-skills-live) を `apps/web:deploy` として修飾されていない名前を通じてブロックします。v2.1.260 より前では、Claude Code は deny ルールが修飾されていない名前のみを指定する場合、修飾された名前の下にリストされたネストされたスキルをブロックしませんでした。

Claude Code は `allow` ルールをスキル独自の名前と Claude の呼び出しの名前に対してのみ一致させます。

**個別のスキルを非表示にする** には、フロントマターに `disable-model-invocation: true` を追加します。これはスキルを Claude のコンテキストから完全に削除します。

<Note>
  `user-invocable: false` では、スキルを呼び出すことはできませんが、Claude はできます。Claude が Skill ツールを通じてそれを呼び出すのを防ぐには、`disable-model-invocation: true` を設定します。
</Note>

<h3 id="override-skill-visibility-from-settings">
  設定からスキルの可視性をオーバーライドする
</h3>

`skillOverrides` 設定は、スキル独自のフロントマターの代わりに [settings](/docs/ja/settings) からスキルの可視性を制御します。SKILL.md を編集したくないスキル（共有プロジェクトリポにチェックインされたものなど）に使用します。`/skills` メニューはあなたのために書き込みます。スキルをハイライトして `Space` を押して状態をサイクルし、`Esc` を押して `.claude/settings.local.json` に保存します。

各キーはスキル名で、各値は 4 つの状態の 1 つです。

| 値                       | Claude にリストされている | `/` メニューで |
| :---------------------- | :--------------- | :-------- |
| `"on"`                  | 名前と説明            | はい        |
| `"name-only"`           | 名前のみ             | はい        |
| `"user-invocable-only"` | 非表示              | はい        |
| `"off"`                 | 非表示              | 非表示       |

`/skills` メニューは `"user-invocable-only"` 状態を `user-only` とラベル付けします。

v2.1.199 以降、`"off"` はターミナル `/` メニューに加えて、[Remote Control](/docs/ja/remote-control) クライアントと [Agent SDK](/docs/ja/agent-sdk/skills#discover-available-commands) 呼び出し元に宣伝されるコマンドリストからもスキルを非表示にします。修飾された名前でスキルを呼び出すと、引き続き実行する代わりに `skillOverrides` エラーが返されます。

`skillOverrides` に存在しないスキルは `"on"` として扱われます。以下の例は 1 つのスキルをその名前に折りたたみ、別のスキルを完全にオフにします。

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

一部のバンドルされたスキルには、`/doctor` の `checkup` などのエイリアスがあります。[managed settings](/docs/ja/managed-settings) または `--settings` フラグで渡すファイルでエイリアスの下に `skillOverrides` エントリを設定する場合、Claude Code はそれをエイリアスの背後にあるスキルに適用します。エイリアスを通じてスキルをさらに制限することのみが可能で、より可視化することはできません。また、managed settings でスキル独自の名前の下にエントリも設定する場合、そのエントリが優先されます。v2.1.260 より前では、Claude Code はいかなる設定ソースでもエイリアスの下のエントリをスキルに適用しませんでした。

ユーザー、プロジェクト、およびローカル設定では、Claude Code はスキル名に対してのみエントリを一致させます。そこで `review` のエントリを設定する場合、それは `review` という名前のスキルに適用され、バンドルされた `/code-review` にはその `/review` エイリアスを通じて適用されません。

プラグインスキルは `skillOverrides` の影響を受けません。`/plugin` を通じてそれらを管理します。

<h3 id="find-unused-skills">
  未使用のスキルを見つける
</h3>

[スキルリスト](#skill-descriptions-are-cut-short) のすべてのスキルは、Claude がそれを使用するかどうかに関係なく、すべてのターンでコンテキストに追加されます。`/skill-doctor` を実行して、各スキルのコストと使用頻度を確認し、どのスキルをオフにするかを決定します。対話型セッションでは、レポートは `/plugin` マネージャーの **Stats** タブで開きます。[非対話モード](/docs/ja/headless) で `-p` を使用する場合、Claude Code はテキストとして出力します。

レポートはバンドルされたスキルとエンタープライズスキル以外のセッション内のスキルをカバーします。リストに表示されたスキルのうち、呼び出されたことのないスキルにフラグを付け、どこでそれらをオフにするかを示します。オフにする場所を示すスキルのうち、最も高いコンテキストコストを持つものから始めます。レポートは最近使用していないプラグインもリストします。

`/skill-doctor` には Claude Code v2.1.252 以降が必要で、[feature-flag fetching](/docs/ja/env-vars#features-that-need-feature-flag-fetching) をスキップするセッションでは利用できません。[Remote Control](/docs/ja/remote-control) から電話またはブラウザで `/skill-doctor` を実行する場合、Claude Code は [`Skill usage reports are not available on this connection.`](/docs/ja/errors#skill-usage-reports-are-not-available-on-this-connection) で返信します。セッションが実行されているマシンのターミナルで `/skill-doctor` を実行します。

<h2 id="evaluate-and-iterate-on-a-skill">
  スキルを評価して反復する
</h2>

スキルがトリガーされたことを確認することは、Claude がそれを見つけたことを意味しますが、意図した動作をしたことを意味しません。スキルが機能していることを知るには、Claude がそれを呼び出すべきプロンプトで実際に呼び出すかどうか、および呼び出す場合に出力が期待と一致するかどうかを個別に測定する必要があります。

両方をチェックするには、ベースライン比較を行います。現実的なプロンプトをいくつか収集し、スキルが利用可能な新しいセッションで各プロンプトを実行し、[無効化](#override-skill-visibility-from-settings)した状態でも実行して、結果を比較します。新しいセッションが重要なのは、スキルの作成時に残されたコンテキストが、書かれた指示のギャップをマスクするためです。

その比較を自動化する 2 つのツールがあります。[プラグイン](/docs/ja/plugins/overview)で配布されるスキルの場合、[`claude plugin eval`](/docs/ja/plugin-evals) は各プロンプトを分離されたセッションでプラグインの有無で実行し、定義したグレーダーまたはそれが作成したグレーダーでスコアリングし、閾値以下の場合はゼロ以外で終了するため、CI でそれをゲートできます。Claude Code 会話内の単一スキルを反復する場合、以下のスキル作成者プラグインは独自の `evals/evals.json` 形式で同様のループを実行します。2 つの形式は相互交換可能ではありません。

<h3 id="run-evals-with-skill-creator">
  skill-creator でエバルを実行する
</h3>

[`skill-creator` プラグイン](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) は Claude Code 内の比較ループを自動化します。公式マーケットプレイスからインストールします。

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

インストールが失敗した場合は、Claude Code が報告するメッセージと一致させます。

* `Marketplace "claude-plugins-official" not found`：`/plugin marketplace add anthropics/claude-plugins-official` でマーケットプレイスを追加してから、インストールを再試行します。
* [プラグインがマーケットプレイスで見つかりません](/docs/ja/plugins/install#install-a-plugin)：プラグイン名を確認します。

インストール概要が `Run /reload-plugins to activate.` を報告する場合、Claude Code はそのリロードを実行します。リロードが次のメッセージが会話を再読み込みすることを警告する場合は、`/reload-plugins --force` を実行してプラグインのスキルを現在のセッションで利用可能にします。その後、Claude に既存のスキルを評価するよう依頼します。例えば `evaluate my summarize-changes skill with skill-creator` です。プラグインはテストケースの作成をガイドし、ループを実行します。

* **テストケース**：プロンプト、入力ファイル、および期待される動作をスキルディレクトリ内の `evals/evals.json` に保存します
* **分離された実行**：テストケースごとに [サブエージェント](/docs/ja/sub-agents) を生成して、各実行がクリーンなコンテキストで開始され、トークン数と期間を記録します
* **グレーディング**：各アサーションを出力に対してチェックし、`grading.json` に証拠とともに合格または不合格を書き込みます
* **ベンチマーク**：スキルあり対スキルなしの合格率、時間、トークンを `benchmark.json` に集約して、トークンと時間のオーバーヘッドに対する合格率の改善を比較できます
* **バージョン比較**：スキルの 2 つのバージョン間でブラインド A/B テストを実行して、編集をコミットする前に改善であることを確認できます
* **説明チューニング**：トリガーすべきおよびトリガーすべきでないプロンプトを生成し、ヒット率を測定し、スキルが間違ったリクエストで起動する場合は説明編集を提案します
* **レビュービューアー**：各出力を検査し、次の反復が読み込む定性的フィードバックを記録できる HTML レポートを開きます

エバルファイル形式と完全な反復ワークフローについては、agentskills.io の [スキル出力品質の評価](https://agentskills.io/skill-creation/evaluating-skills) を参照してください。ベンチマークと比較モードの背景については、[skill-creator アナウンスメント](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills) を参照してください。

<h2 id="share-skills">
  スキルを共有する
</h2>

スキルはオーディエンスに応じて異なるスコープで配布できます。

* **プロジェクトスキル**: `.claude/skills/` をバージョン管理にコミットする
* **プラグイン**: [プラグイン](/docs/ja/plugins/overview)に `skills/` ディレクトリを作成する
* **マネージド**: [マネージド設定](/docs/ja/managed-settings)を通じて組織全体にデプロイする

<h3 id="generate-visual-output">
  ビジュアル出力を生成する
</h3>

スキルは任意の言語でスクリプトをバンドルして実行でき、Claude に単一のプロンプトでは不可能な機能を提供します。1 つのパターンはビジュアル出力を生成することです。つまり、ブラウザで開いてデータを探索したり、デバッグしたり、レポートを作成したりするためのインタラクティブな HTML ファイルです。

この例は、コードベースエクスプローラーを作成します。これはインタラクティブなツリービューで、ディレクトリを展開・折りたたんだり、ファイルサイズを一目で確認したり、ファイルタイプを色で識別したりできます。

スキルディレクトリを作成します。

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

これを `~/.claude/skills/codebase-visualizer/SKILL.md` に保存します。説明は Claude にこのスキルをいつ有効化するかを伝え、指示は Claude にバンドルされたスクリプトを実行するよう伝えます。スクリプトパスは [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) を使用するため、スキルが個人、プロジェクト、またはプラグインレベルでインストールされているかどうかに関わらず正しく解決されます。

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

これを `~/.claude/skills/codebase-visualizer/scripts/visualize.py` に保存します。このスクリプトはディレクトリツリーをスキャンし、以下を含む自己完結型の HTML ファイルを生成します。

* ファイル数、ディレクトリ数、合計サイズ、ファイルタイプ数を表示する**サマリーサイドバー**
* コードベースをファイルタイプ別（サイズ上位 8 つ）に分類する**棒グラフ**
* ディレクトリを展開・折りたたんだり、色分けされたファイルタイプインジケーターを表示したりできる**折りたたみ可能なツリー**

スクリプトは Python 3 を必要としますが、組み込みライブラリのみを使用するため、インストールするパッケージはありません。

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

テストするには、任意のプロジェクトで Claude Code を開き、「Visualize this codebase.」と尋ねます。Claude はスクリプトを実行し、生成されたファイルのパス（例：`Generated /path/to/codebase-map.html`）を出力し、ブラウザで開きます。ブラウザが開かないヘッドレス環境で作業している場合、出力されたパスはスクリプトが成功したことを確認します。

このパターンはあらゆるビジュアル出力に対応します。依存関係グラフ、テストカバレッジレポート、API ドキュメント、またはデータベーススキーマの可視化です。バンドルされたスクリプトが作業を行い、Claude がオーケストレーションを処理します。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="skill-not-triggering">
  スキルがトリガーされない
</h3>

Claude がスキルを期待通りに使用しない場合：

1. 説明にユーザーが自然に言うキーワードが含まれているか確認してください
2. スキルが「What skills are available?」に表示されていることを確認してください
3. 説明により密接に一致するようにリクエストを言い換えてみてください
4. スキルがユーザー呼び出し可能な場合は、`/skill-name` で直接呼び出してください

frontmatter YAML が不正な形式の場合、Claude Code はスキル本体を空のメタデータで読み込むため、`/skill-name` は機能しますが Claude は `description` と照合できません。`--debug` で実行してパースエラーを確認してください。

スキルがプラグインに含まれている場合、1 つずつ確認するのではなく、現実的なプロンプト全体でスキルがどのくらいの頻度でトリガーされるかを測定できます。[`tool_used: Skill` グレーダー](/docs/ja/plugin-evals#create-your-first-eval-suite)を使用して eval ケースを作成し、説明を変更するたびに `claude plugin eval` で実行してください。

frontmatter がパースされない `SKILL.md` ファイルを見つけるには、スキルディレクトリで [`claude plugin validate`](/docs/ja/plugins/cli-reference#validate-a-directory) を実行してください。例えば、プロジェクトスキルの場合は `claude plugin validate .claude/skills`、個人スキルの場合は `claude plugin validate ~/.claude/skills` です。Claude Code v2.1.233 以降が必要です。

<h3 id="skill-triggers-too-often">
  スキルが頻繁にトリガーされる
</h3>

Claude がスキルを不要な時に使用する場合：

1. 説明をより具体的にしてください
2. 手動呼び出しのみが必要な場合は `disable-model-invocation: true` を追加してください

<h3 id="skill-descriptions-are-cut-short">
  スキルの説明が短くカットされている
</h3>

Claude Code はスキル名と説明のリストをコンテキストに読み込み、Claude が利用可能なものを認識できるようにします。リストには常にすべてのスキル名が含まれていますが、スキルが多い場合、Claude Code はリストの文字予算に合わせて説明を短縮し、Claude が一致させるために必要なキーワードを削除する可能性があります。予算はモデルのコンテキストウィンドウの 1% でスケーリングされます。リストが予算を超える場合、Claude Code は最も呼び出しが少ないスキルから説明を削除するため、最も使用するスキルは完全なテキストを保持します。

`/doctor` を実行してリストのコンテキストコストとその最大の貢献者の推定値を取得してください。オフにする価値のあるスキルを見つけるには、[`/skill-doctor`](#find-unused-skills) を実行してください。リストが予算を超える場合、Claude Code はデバッグログに警告も書き込みます。これは [`--debug`](/docs/ja/cli-reference#cli-flags) で表示できます。

`/context` の Skills 行は、予算が適用された後のリストのサイズを報告するため、モデルが受け取るものと一致します。v2.1.196 より前は、この行はすべての説明の完全なテキストをカウントし、設定された予算の数倍大きい値を表示する可能性がありました。

予算を増やすには、[`skillListingBudgetFraction`](/docs/ja/settings-reference#skilllistingbudgetfraction) 設定（例：`0.02` = 2%）または `SLASH_COMMAND_TOOL_CHAR_BUDGET` 環境変数を固定文字数に設定してください。他のスキルの予算を解放するには、[`skillOverrides`](#override-skill-visibility-from-settings) で低優先度のエントリを `"name-only"` に設定して、説明なしでリストされるようにしてください。また、ソースで `description` と `when_to_use` テキストをトリミングすることもできます。各エントリの結合テキストは予算に関係なく 1,536 文字でキャップされているため、主要なユースケースを最初に配置してください。キャップは [`skillListingMaxDescChars`](/docs/ja/settings-reference#skilllistingmaxdescchars) で設定可能です。

<h3 id="personal-skills-disappeared">
  個人スキルが消えた
</h3>

`~/.claude/skills/` に作成したスキルフォルダが消えている場合は、`~/.claude/skills/.trash/` を確認してください。Claude Code が [claude.ai からスキルを同期](#how-synced-skills-behave)する場合、それらは別の `synced` サブフォルダにダウンロードされ、作成したフォルダは移動または削除されません。

v2.1.280 より前は、`~/.claude/skills/` に `manifest.json` という名前のファイルがあると、Claude Code はそのファイルがリストしたスキルフォルダを `~/.claude/skills/.trash/` の下のタイムスタンプ付きフォルダに移動し、それらのスキルは読み込まれなくなりました。

スキルを復元するには、タイムスタンプ付きフォルダからそのフォルダを `~/.claude/skills/` に戻してください。これは [retention sweep](/docs/ja/claude-directory#cleaned-up-automatically) がゴミ箱エントリを削除する前に行ってください。デフォルトではゴミ箱に移動されてから 30 日後に削除されます。

<h2 id="related-resources">
  関連リソース
</h2>

* **[設定をデバッグする](/docs/ja/debug-your-config)**：スキルが表示されない、またはトリガーされない理由を診断する
* **[スキル出力品質の評価](https://agentskills.io/skill-creation/evaluating-skills)**：agentskills.io の eval ファイル形式と反復ワークフロー
* **[スキル作成のベストプラクティス](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**：Claude 製品全体に適用される作成ガイダンス
* **[サブエージェント](/docs/ja/sub-agents)**：特化したエージェントにタスクを委任する
* **[プラグイン](/docs/ja/plugins/overview)**：他の拡張機能でスキルをパッケージ化して配布する
* **[フック](/docs/ja/hooks)**：ツールイベント周辺のワークフローを自動化する
* **[メモリ](/docs/ja/memory)**：永続的なコンテキストのための CLAUDE.md ファイルを管理する
* **[コマンド](/docs/ja/commands)**：組み込みコマンドとバンドルされたスキルのリファレンス
* **[権限](/docs/ja/permissions)**：ツールとスキルアクセスを制御する
* **[Claude Tag スキル](https://claude.com/docs/claude-tag/admins/skills-repo)**：リポジトリにコミットされたプロジェクトスキルは、そのリポジトリが Claude Tag チャネルで使用される場合にも読み込まれます
