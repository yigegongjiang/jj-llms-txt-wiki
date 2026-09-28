> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code プラグインを作成する

> 空のディレクトリから最初の Claude Code プラグインを構築し、マーケットプレイスなしでテストし、既存の .claude/ セットアップを変換します。

プラグインは、スキル、エージェント、フック、MCP サーバーのディレクトリと、プラグインに名前を付ける `plugin.json` ファイル（マニフェストと呼ばれます）です。Claude Code はディレクトリを 1 つのユニットとして読み込むため、チームメイトと共有したり、複数のプロジェクトにインストールしたり、マーケットプレイスに公開したりできます。

このページは、独自のプラグインを作成する人向けです。

<Note>
  これらのケースは他のページで説明されています。

  * **他の人のプラグインをインストールする**: [プラグインをインストールする](/docs/ja/plugins/install)を参照してください
  * **プラグインが必要かどうか確実でない**: 概要の[プラグインが必要かどうかを判断する](/docs/ja/plugins/overview#decide-whether-you-need-a-plugin)を参照してください
  * **プラグインのユーザーが claude.ai または Cowork にいる**: 同じフォルダがコンポーネントの異なるサブセットでそこにインストールされます。[claude.ai と Cowork のプラグイン](https://claude.com/docs/plugins/overview)を参照してください
</Note>

既に持っているものに一致するセクションから開始してください。

* **まだ何もない**: [最初のプラグインを作成する](#create-your-first-plugin)に従い、次に[マーケットプレイスなしで開発する](#develop-without-a-marketplace)と[テストとデバッグ](#test-and-debug)に従ってください。
* **既に `.claude/` の下にファイルがある**: 最初のプラグインのウォークスルーを一度実行してレイアウトを学び、次に[既存の `.claude/` セットアップを変換する](#convert-an-existing-claude-setup)に従ってください。

<h2 id="decide-when-to-use-a-plugin">
  プラグインを使用する時期を決定する
</h2>

スキル、エージェント、フック、MCP サーバーはすべて、プロジェクトまたはホームディレクトリでスタンドアロンで機能します。1 つのプロジェクトまたは自分だけに対応している間は、そのスタンドアロンセットアップを保持してください。チームメイトと共有したい場合、複数のプロジェクトにインストールしたい場合、またはバージョン付きリリースを公開したい場合は、プラグインを作成してください。

スタンドアロンのスキル、エージェント、フック、MCP 設定をプラグインに移動すると、それらの場所と名前が変わります。

* **ファイルの場所**: プラグインのルートと呼ばれるプラグイン独自のディレクトリの下に、`skills/`、`agents/`、`hooks/hooks.json`、`.mcp.json` として配置されます。
* **名前の付け方**: プラグインのスキルとエージェントはプラグイン名をプレフィックスとして取得します。例えば `/my-plugin:hello` のように、2 つのプラグインが衝突することなく各々 `hello` スキルを提供できます。

既存のセットアップをプラグインに移動するには、[既存の `.claude/` セットアップを変換する](#convert-an-existing-claude-setup)を参照してください。

<h2 id="create-your-first-plugin">
  最初のプラグインを作成する
</h2>

このウォークスルーでは、唯一のコンポーネントが 1 つのスキル（グリーティング）であるプラグインを作成し、`--plugin-dir` で実行します。これはインストールせずに 1 つのセッションのためにプラグインを読み込みます。プラグインは、スキル、エージェント、フック、MCP サーバーなどの[コンポーネント](/docs/ja/plugins/components)の任意の組み合わせを保持でき、どれも必須ではありません。1 つのスキルはレイアウトを示す最小限の例です。

Claude Code が[インストールされてサインインしている](/docs/ja/quickstart#step-1-install-claude-code)必要があります。

プラグインを保持したいディレクトリ（例えば `~/projects`）でターミナルを開き、これらのステップのコマンドをそこから実行してください。プラグインはどこにでも保持できます。セッションを開始するときにそのパスを Claude Code に渡すためです。

<Steps>
  <Step title="プラグインディレクトリを作成する">
    プラグインディレクトリを作成し、マニフェストを保持するための `.claude-plugin/` フォルダをその中に作成します。

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="マニフェストを書く">
    [マニフェスト](/docs/ja/plugins/manifest-reference)は `plugin.json` という名前の JSON ファイルで、Claude Code にプラグインの名前を伝え、それを説明します。これを `my-first-plugin/.claude-plugin/plugin.json` として保存してください。

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    4 つのフィールドは以下のことを行います。

    * **`name`**: 必須。プラグインを識別し、プラグインが提供するすべてのスキルとエージェントのプレフィックスになります。スペースを入れないでください。
    * **`description`**: ユーザーが `/plugin` でプラグインに対して見るテキスト。
    * **`version`**: オプション。これを設定すると、ユーザーはそれを変更するまでそのバージョンに留まります。[新しいバージョンをリリースする](/docs/ja/plugins/host-marketplace#release-a-new-version)は、いつそれを設定または省略するかを説明しています。
    * **`author`**: クレジットする人。その中の `name` は必須です。`email` と `url` はオプションです。

    他のすべてのフィールドは[マニフェストリファレンス](/docs/ja/plugins/manifest-reference#fields)にあります。

    `.claude-plugin/` の中には `plugin.json` だけが入ります。次に追加するスキルは `my-first-plugin/` の直下に、そのフォルダの隣に入ります。
  </Step>

  <Step title="スキルを追加する">
    このプラグインの唯一のコンポーネントはスキルです。各スキルは `skills/` の下のディレクトリで、`SKILL.md` ファイルを含みます。スキルのディレクトリを作成してください。

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    次に、このコンテンツで `my-first-plugin/skills/hello/SKILL.md` を作成してください。

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    `disable-model-invocation: true` の行は、Claude がスキルを独自に実行しないことを意味するため、トリガーするのはあなただけです。Claude が独自に実行したいスキルからその行を削除してください。スキルのコマンドはプラグイン名とスキルの名前を組み合わせるため、これを `/my-first-plugin:hello` として実行します。他のフロントマター フィールドについては、[スキルフロントマターリファレンス](/docs/ja/skills#frontmatter-reference)を参照してください。
  </Step>

  <Step title="プラグインを検証する">
    何かを実行する前に、マニフェストとスキルのフロントマターをチェックしてください。

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    コマンドはチェックしたマニフェストパスと `✔ Validation passed` を出力します。代わりに `✘ Validation failed` を出力する場合、その結果行の上の各行は修正するフィールドに名前を付けます。[`claude plugin validate` がエラーを報告する](/docs/ja/plugins/troubleshooting#claude-plugin-validate-reports-errors)の下で各メッセージを調べてください。
  </Step>

  <Step title="プラグインで Claude Code を実行する">
    プラグインが読み込まれたセッションを開始してください。

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Claude Code が開始したら、スキルを実行してください。

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude はグリーティングで返信します。
  </Step>
</Steps>

プラグインは `--plugin-dir` で開始したセッションでのみ読み込まれます。フラグなしで作業を続けるか、`.zip` ビルドをテストするには、[マーケットプレイスなしで開発する](#develop-without-a-marketplace)を参照してください。

<h3 id="share-the-plugin">
  プラグインを共有する
</h3>

[最初のプラグインを作成する](#create-your-first-plugin)で構築したプラグインはマシンにのみ存在します。他の人が使用する準備ができたら、それを取得する 3 つの方法があります。

* **数人に直接送信する**: プラグインのディレクトリまたはその `.zip` を提供し、何も公開する必要はありません。[マーケットプレイスなしでプラグインを共有する](/docs/ja/plugins/publish#share-a-plugin-without-a-marketplace)を参照してください。
* **独自のマーケットプレイスにリストする**: チームメイトはマーケットプレイスを一度追加し、プラグインを名前でインストールし、更新を受け取ります。[独自のマーケットプレイスを通じて公開する](/docs/ja/plugins/publish#publish-through-your-own-marketplace)を参照してください。
* **Anthropic のコミュニティマーケットプレイスに送信する**: リストされたら、そのマーケットプレイスを追加した誰もがインストールできます。[コミュニティマーケットプレイスに送信する](/docs/ja/plugins/publish#submit-to-the-community-marketplace)を参照してください。

<h3 id="plugin-layout">
  プラグインレイアウト
</h3>

スキル、エージェント、フック、MCP サーバーなどの各種[コンポーネント](/docs/ja/plugins/components)は、プラグインルートの下の固定ディレクトリに入ります。プラグインルートは `--plugin-dir` に渡すディレクトリです。使用するディレクトリのみを追加してください。完全なプラグインディレクトリをクリックして、各ファイルが何をするかを読むには、[プラグインエクスプローラー](/docs/ja/plugins/components#explore-the-plugin-directory)を開いてください。

テーブルはほとんどのプラグインが開始するディレクトリをリストし、[完全なレイアウト](/docs/ja/plugins/manifest-reference#standard-layout)は残りをリストしています。

| 場所                           | 内容                                                                                |
| :--------------------------- | :-------------------------------------------------------------------------------- |
| `.claude-plugin/plugin.json` | マニフェスト。`--plugin-dir` でプラグインを読み込み、マニフェストがない場合、Claude Code はプラグインをディレクトリの後に名前を付けます |
| `skills/`                    | スキルごとに 1 つの `<name>/SKILL.md` ディレクトリ                                              |
| `commands/`                  | フラットな Markdown ファイル、スキルの古い形式。新しいプラグインには `skills/` を使用してください                       |
| `agents/`                    | サブエージェントごとに 1 つの Markdown ファイル                                                    |
| `hooks/hooks.json`           | フック設定: トップレベルの `"hooks"` キーで、設定ファイルの `hooks` と同じ形状の値を持つ                           |
| `.mcp.json`                  | MCP サーバー定義                                                                        |

<Warning>
  `.claude-plugin/` の中には `plugin.json` だけが入ります。そこに保存されたコンポーネントは読み込まれません。

  プラグインルートはプラグイン独自のディレクトリで、`~/.claude/` 自体ではありません。`~/.claude/.mcp.json` に保存された `.mcp.json` は読み込まれません。
</Warning>

<h2 id="develop-without-a-marketplace">
  マーケットプレイスなしで開発する
</h2>

作成しているプラグインを実行するために[マーケットプレイス](/docs/ja/plugins/overview#get-plugins-from-a-marketplace)は必要ありません。代わりにディスクまたは URL から直接読み込んでください。

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session): 1 つのセッションのためにディレクトリまたは `.zip` アーカイブを読み込みます。
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session): 1 つのセッションのために URL から `.zip` アーカイブをフェッチします。
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session): `~/.claude/skills/` の下にプラグインをスキャフォルドし、すべてのセッションで読み込みます。

異なる方法で読み込まれた 2 つのプラグインが名前を共有する場合、[名前の競合](/docs/ja/plugins/loading#name-conflicts)を参照して、Claude Code がどれを保持するかを確認してください。

<h3 id="load-a-directory-or-archive-for-one-session">
  1 つのセッションのためにプラグインを読み込む
</h3>

3 つの方法で 1 つのセッションのためにプラグインを読み込むことができます。`--plugin-dir` でディスク上のディレクトリまたは `.zip` アーカイブから、`--plugin-url` で URL から、またはフラグを追加できない場合は環境変数から。各プラグインはそのセッションのみのために読み込まれ、設定には何も書き込まれません。セッション中にプラグインのファイルを編集する場合、`/reload-plugins` を実行して変更を読み込んでください。

<h4 id="from-a-directory-or-zip">
  ディレクトリまたは `.zip` から
</h4>

シェルから `claude` を開始するときに、`--plugin-dir` をプラグインのルートディレクトリまたはその `.zip` アーカイブで渡してください。複数のプラグインを読み込むためにフラグを繰り返してください。

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  プラグインのフォルダから
</h4>

複数のプラグインを 1 つの場所から読み込むには、`--plugin-dir ./plugins` のようにそれらを保持するフォルダを渡してください。プラグインのフォルダを読み込むには Claude Code v2.1.265 以降が必要です。

フォルダに `.claude-plugin/` ディレクトリがなく、トップレベルにプラグインコンポーネントがない場合、Claude Code はそれをプラグインのフォルダとして扱います。`.claude-plugin/plugin.json` マニフェストを持つ各直下のサブフォルダは、別のプラグインとして読み込まれます。フォルダ内の他のすべてのものはスキップされます。マニフェストがないサブフォルダを含めて、エラーなしでスキップされます。フォルダ内のプラグインが読み込まれない場合、そのサブフォルダに `.claude-plugin/plugin.json` があることを確認してください。

対話型セッションでは、起動後にフォルダ内のプラグインを追加および削除することもできます。

* 追加するサブフォルダは、マニフェストが存在すると新しいプラグインとして読み込まれます。
* サブフォルダを削除すると、そのプラグインはアンロードされます。

これらの変更のそれぞれについて、セッションにメッセージが表示されます。プラグインの読み込みまたはアンロードが会話の途中で[プロンプトキャッシュを無効にする](/docs/ja/prompt-caching#enabling-or-disabling-a-plugin)場合、変更は代わりに保持され、メッセージは `/reload-plugins` を実行して適用するよう指示します。

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  URL から
</h4>

シェルから `claude` を開始するときに、`--plugin-url` を `.zip` アーカイブのアドレス（例えば CI が公開するビルドアーティファクト）で渡してください。

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code は起動時にアーカイブをダウンロードします。複数を読み込むには、フラグを繰り返すか、URL をスペース区切りで 1 つの引用符付き引数で渡してください。

フラグは、制御またはトラストしているアーカイブのみを指してください。

Claude Code がアーカイブをフェッチできない場合、またはアーカイブが無効な場合、プラグインなしで開始し、プラグイン読み込みエラーを記録します。これは `/plugin` マネージャーの **Errors** タブで確認できます。

<h4 id="from-an-environment-variable">
  環境変数から
</h4>

`--plugin-dir` フラグを追加できないセッションでプラグインを読み込むには、[`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ja/env-vars#variables)環境変数にそれらの絶対パスをリストしてください。Claude Code は各パスを `--plugin-dir` パスとして読み込みます。これらのプラグインは、`--plugin-dir` で渡したものに加えて読み込まれます。[プロジェクトとローカル設定はこの変数を設定できません](/docs/ja/settings-reference#variables-claude-code-ignores-in-env)。`CLAUDE_CODE_PLUGIN_DIRS` には Claude Code v2.1.280 以降が必要です。

マネージド設定は `--plugin-dir` と `CLAUDE_CODE_PLUGIN_DIRS` をオフにできます。[1 つのセッションのためにプラグインを読み込むフラグ](/docs/ja/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)を参照してください。プラグインとそれが依存するプラグインをテストするには、[プラグインとその依存関係をローカルでテストする](/docs/ja/plugins/dependencies#test-a-plugin-and-its-dependency-locally)を参照してください。

<h3 id="scaffold-a-plugin-that-loads-every-session">
  すべてのセッションでプラグインを読み込むようにする
</h3>

個人的なスキルディレクトリは `~/.claude/skills/` です。Claude Code は、`.claude-plugin/plugin.json` を含むそこのフォルダを、フラグなしでインストールステップなしで、すべてのセッションでプラグインとして読み込みます。`claude plugin init` はこれらのプラグインの 1 つをスキャフォルドします。

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  `claude plugin init` でプラグインをスキャフォルドする
</h4>

`claude plugin init` は `~/.claude/skills/` の下にスタータープラグインを書き込みます。Claude Code v2.1.157 以降が必要です。シェルからスキャフォルドしてください。

```bash theme={null}
claude plugin init my-tool
```

コマンドは `.claude-plugin/plugin.json` とルート `SKILL.md` で `~/.claude/skills/my-tool/` を作成します。`✔ Created plugin "my-tool" at ~/.claude/skills/my-tool` に続いて `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.` を出力します。

`--with skills` を渡して、`claude plugin init` に `skills/` の下のスキルをスキャフォルドさせてください。他の `--with` 値は[プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference#plugin-init)にあります。

<h4 id="skill-names-in-a-scaffolded-plugin">
  スキャフォルドされたプラグインのスキルに名前を付ける
</h4>

`~/.claude/skills/my-tool/SKILL.md` のルートスキルは個人的なスキルでもあるため、`/my-tool:my-tool` ではなく `/my-tool` として呼び出します。プラグイン内の `skills/` の下に追加するスキルはプラグイン名プレフィックスを取得します。例えば `/my-tool:example` のように。

<h4 id="stop-loading-the-plugin">
  プラグインの読み込みを停止する
</h4>

スキャフォルドされたプラグインの読み込みを停止するには、そのディレクトリを削除するか、シェルで `claude plugin disable my-tool@skills-dir` を実行してください。`claude plugin init` が出力した `my-tool@skills-dir` 名を使用してください。ID `my-tool@skills-dir` では、`skills-dir` はマーケットプレイス名が入る場所に立ちます。プラグインはマーケットプレイスではなくスキルディレクトリから読み込まれるためです。

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  リポジトリを通じてプラグインを共有する
</h4>

`claude plugin init` はプラグインを個人的なスキルディレクトリ `~/.claude/skills/` に書き込むため、すべてのプロジェクトであなたのために読み込まれます。1 つのリポジトリのすべての人のためにプラグインを読み込むには、`.claude-plugin/plugin.json` を含む同じレイアウトを `<project>/.claude/skills/<name>/` で自分で作成してください。[リポジトリを通じて共有されるプラグイン](/docs/ja/plugins/loading#plugins-shared-through-a-repository)を参照して、Claude Code がそれを読み込む条件を確認してください。

<h2 id="test-and-debug">
  テストとデバッグ
</h2>

プラグインへの変更が表示されない場合、これらのチェックを順番に実行してください。それぞれが Claude Code がプラグインで何をしたかを伝えます。

1. シェルで `claude plugin validate <path>` を実行してください。マニフェストとすべてのスキル、エージェント、コマンドファイルのフロントマターをチェックし、`Validation passed` で終了コード 0 で終了します。`--strict` を追加して警告でも失敗するようにしてください。終了コードとディレクトリ処理は[プラグインコマンドリファレンス](/docs/ja/plugins/cli-reference#plugin-validate)にあります。
2. 実行中のセッションで `/reload-plugins` を実行して、ディスク上で行った編集を適用してください。1 つの `Reloaded:` 行をカウント付きで出力します。次に、`/plugin-name:skill` コマンドを入力するか、`/plugin` **Installed** タブでプラグインを見つけることで、スキルが読み込まれたことを確認してください。
3. 同じセッションで `/plugin` を実行してください。**Installed** タブはプラグインをリストし、プラグインの詳細では、Claude Code が見つけたコンポーネントをリストします。**Errors** タブは、読み込みに失敗したものと理由（例えば、マニフェスト内のパスが存在しない）をリストします。
4. シェルに戻り、`claude plugin list` を実行してください。セッションのみとスキルディレクトリプラグインを独自のセクションで `Status: ✔ loaded` または読み込みエラーで出力します。開発中のプラグインを含めるには、`plugin list` の前に `--plugin-dir` をそのパスで渡してください。

MCP サーバーをチェックするには、セッションで `/mcp` を実行してサーバーのステータスを確認してください。サーバーが健全な場合、`/mcp` はそれを接続済みとしてリストします。そうでない場合、[開始しない MCP サーバー](/docs/ja/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)を参照してください。

フックをチェックするには、それが一致するイベントをトリガーしてください。例えば、Claude にファイルを編集するよう求めて `PostToolUse` フックをトリガーしてください。次に[デバッグログ](/docs/ja/hooks#debug-hooks)を読んでください。これは、どのフックが一致したか、それらの終了コード、それらの出力を示します。

次のセクションは、開発中に最も可能性の高い失敗をカバーし、[トラブルシューティングページ](/docs/ja/plugins/troubleshooting#build-a-plugin)には各々の完全なエントリがあります。

<h3 id="a-component-path-isn’t-found">
  コンポーネントパスが見つからない
</h3>

`/plugin` の **Errors** タブは `<component> path not found: <path>` を表示します。例えば `commands path not found`。マニフェスト内のコンポーネントパス（`commands`、`skills`、`agents`、`hooks` など）は何も指していません。パスを修正するか、ディレクトリを作成し、セッションで `/reload-plugins` を実行してください。[`commands path not found`](/docs/ja/plugins/troubleshooting#commands-path-not-found)を参照してください。

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir` がマーケットプレイスルートにあると `plugins/` の下のプラグインが読み込まれない
</h3>

`--plugin-dir` はプラグインのルートディレクトリを取ります。`.claude-plugin/plugin.json` とコンポーネントディレクトリ（`skills/` など）を含むディレクトリです。代わりにマーケットプレイスルートを指すと、Claude Code は `marketplace.json` を読まないため、`plugins/` の下のプラグインは読み込まれず、エラーは表示されません。フラグを 1 つのプラグインのフォルダに指すか、マーケットプレイスを追加してください。[トラブルシューティングエントリ](/docs/ja/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components)を参照してください。

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  プラグインが読み込まれるがそのスキルが見つからない
</h3>

`skills/` ディレクトリが `.claude-plugin/` の内部にあるか、マニフェスト内の `skills` エントリがファイルを指しています。`skills/` をプラグインルートに移動し、各 `skills` エントリを `SKILL.md` を含むディレクトリを指すようにし、セッションで `/reload-plugins` を実行してください。[プラグインが読み込まれるがそのスキルが見つからない](/docs/ja/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing)を参照してください。

<h3 id="the-userconfig-dialog-never-appears">
  `userConfig` ダイアログが表示されない
</h3>

プラグインの [`userConfig`](/docs/ja/plugins/components#user-configuration) オプションのダイアログはセッションで `/plugin` を通じてインストールの一部です。`--plugin-dir` での読み込みはそれを表示しません。シェルの `claude plugin install` でもそうです。プラグインが読み込まれたら、セッションで `/plugin configure <plugin-name>` を実行してそれを開いてください。[`userConfig` ダイアログが表示されない](/docs/ja/plugins/troubleshooting#the-userconfig-dialog-never-appears)を参照してください。

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  プラグインが Claude の動作を変更することを確認する
</h3>

エラーなしで読み込まれるプラグインは、意図した方法で Claude を操舵できない場合があります。`claude plugin eval` はシェルで実行し、プラグインの有無でテストケースを実行し、差を採点します。[プラグインで evals をテストする](/docs/ja/plugin-evals)を参照してください。[最初の eval スイートを作成する](/docs/ja/plugin-evals#create-your-first-eval-suite)から開始してください。

<h2 id="convert-an-existing-claude-setup">
  既存の `.claude/` セットアップを変換する
</h2>

既にプロジェクトの `.claude/` ディレクトリの下にスキル、エージェント、またはフックがある場合、それらを書き直さずにプラグインに移動できます。

`.claude/` を含むディレクトリであるプロジェクトルートからこれらのステップのコマンドを実行してください。`cp` パスはそれに対して相対的であるためです。

<Steps>
  <Step title="プラグイン構造を作成する">
    プラグインディレクトリとその `.claude-plugin/` フォルダを `.claude/` の隣に作成してください。その後、プラグインをどこにでも移動できます。

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    `my-plugin/.claude-plugin/plugin.json` を作成してください。

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="既存のファイルをコピーする">
    持っている各設定ディレクトリをプラグインルートにコピーし、持っていないディレクトリのコマンドをスキップしてください。

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    `ls -a my-plugin` を実行して、コピーした各ディレクトリが `.claude-plugin` の隣に表示されることを確認してください。
  </Step>

  <Step title="フックを移動する">
    `.claude/settings.json` または `.claude/settings.local.json` にフックがある場合、フックディレクトリを作成してください。

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    `my-plugin/hooks/hooks.json` を作成し、設定ファイルから `hooks` オブジェクトをそこにコピーしてください。形式は同じです。

    この例は、Claude が書き込むまたは編集する各ファイルでリンターを実行する 1 つのフックを持つ形状を示しています。例を独自の `hooks` オブジェクトで置き換えてください。

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="移行されたプラグインをテストする">
    セッションのためにプラグインを読み込んでください。

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    新しい名前の下で各コンポーネントをチェックしてください。

    * **スキル**: `/deploy` だったスキルのために `/my-plugin:deploy` を実行してください。
    * **サブエージェント**: `reviewer` だったエージェントのために Claude に `my-plugin:reviewer` エージェントを使用するよう求めてください。
    * **フック**: 各フックが一致するイベントをトリガーしてください。

    何かが見つからない場合、[テストとデバッグ](#test-and-debug)を実行してください。
  </Step>
</Steps>

オリジナルがまだ `.claude/` の下にある間、それらはプラグインのコピーと一緒に読み込まれたままです。

* **スキルとエージェント**: 2 つのセットは衝突しません。プラグインのスキルとエージェントは `my-plugin:` プレフィックスを持つためです。`/deploy` と `/my-plugin:deploy` の両方が機能し、Claude は `reviewer` と `my-plugin:reviewer` を 2 つのサブエージェントとして見ます。
* **フック**: フックにはプレフィックスがないため、設定ファイルと `hooks/hooks.json` の両方にあるフックは、そのイベントが発火するたびに 2 回実行されます。

プラグインが機能することを確認した後、`.claude/` からオリジナルを削除し、設定ファイルから `hooks` オブジェクトを削除してください。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグインコンポーネント](/docs/ja/plugins/components): エージェント、フック、MCP サーバー、LSP サーバー、ユーザー設定をプラグインに追加する
* [プラグインで evals をテストする](/docs/ja/plugin-evals): eval ケースを書き込み、`claude plugin eval` で実行してプラグインが Claude の動作をどの程度確実に導くかをチェックする
* [プラグインを公開する](/docs/ja/plugins/publish): バージョン管理し、マーケットプレイスに入れ、コミュニティマーケットプレイスに送信する
* [claude.ai と Cowork のプラグイン](https://claude.com/docs/plugins/overview): 同じプラグインフォルダが claude.ai と Cowork にインストールされます。一部のコンポーネントは Claude Code のみです
* [プラグインマニフェストリファレンス](/docs/ja/plugins/manifest-reference): すべての `plugin.json` フィールド、パスルール、ディレクトリ
* [スキル](/docs/ja/skills): プラグインが提供するスキルを書く
* [Anthropic の claude-code リポジトリのプラグイン](https://github.com/anthropics/claude-code/tree/main/plugins): このページのレイアウトの完全な実装例。`feature-dev` と `code-review` など
