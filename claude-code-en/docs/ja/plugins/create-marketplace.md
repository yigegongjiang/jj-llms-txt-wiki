> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# マーケットプレイスを作成する

> marketplace.json ファイルからプラグインマーケットプレイスを構築し、ホストする前にローカルでテストします。

プラグインマーケットプレイスは、`.claude-plugin/marketplace.json` ファイルを含むディレクトリまたはリポジトリで、プラグインをリストアップし、各プラグインをどこから取得するかを指定します。ディレクトリを git ホストにプッシュすると、アクセス権を持つ誰もが 1 つのコマンドで Claude Code に登録し、カタログからプラグインをインストールできます。

チーム、組織など、選択したグループがプラグインをインストールし、制御するカタログから継続的に更新を受け取るようにしたい場合は、独自のマーケットプレイスを作成します。リポジトリはプライベートにすることができ、好きなだけ多くのプラグインをリストアップでき、管理者は [すべてのマシンで必須にすることができます](/docs/ja/plugins/org)。

<Note>
  以下のケースは他のページで説明されています：

  * **1 つのプラグインを少数の人と共有する**：プラグインのディレクトリまたはその `.zip` を送信します。[マーケットプレイスなしでプラグインを共有する](/docs/ja/plugins/publish#share-a-plugin-without-a-marketplace)を参照してください。
  * **プラグインを誰もが利用できるようにする**：Anthropic のコミュニティマーケットプレイスに送信します。[コミュニティマーケットプレイスに送信する](/docs/ja/plugins/publish#submit-to-the-community-marketplace)を参照してください。
  * **プラグインを自分で使用する**：`--plugin-dir` で読み込むか、スキルディレクトリに保存します。[マーケットプレイスなしで開発する](/docs/ja/plugins/create#develop-without-a-marketplace)を参照してください。
</Note>

[マーケットプレイスを作成する](#create-a-marketplace)から始めて、自分のマシンにマーケットプレイスを構築し、そこからプラグインをインストールしてから、[プラグインエントリを追加します](#add-plugin-entries)。

<h2 id="create-a-marketplace">
  マーケットプレイスを作成する
</h2>

以下の手順は、マシン上にマーケットプレイスを作成し、プラグインを追加し、Claude Code に登録し、そこからプラグインをインストールします。これが全体のループであり、マーケットプレイスをホストした後、ユーザーが実行するループと同じです。`my-marketplace/` を作成したいディレクトリから、シェルですべてのコマンドを実行します。

リストアップするプラグインが必要です。例では [最初のプラグインを作成する](/docs/ja/plugins/create#create-your-first-plugin)の `my-first-plugin` を使用します。これは `/my-first-plugin:hello` として実行する 1 つのスキルを持つプラグインです。まだプラグインがない場合は、最初に構築してください。代わりに独自のプラグインを使用する場合は、手順で `my-first-plugin` と書かれている場所で、そのディレクトリと `name` に置き換えてください。プラグインディレクトリに含まれる内容については、[プラグインディレクトリエクスプローラー](/docs/ja/plugins/components#explore-the-plugin-directory)を参照してください。

<Steps>
  <Step title="マーケットプレイスディレクトリを設定する">
    マーケットプレイスは `.claude-plugin/marketplace.json` ファイルを含むディレクトリと、リストアップするプラグインです。マーケットプレイスディレクトリとその `.claude-plugin/` フォルダを作成し、プラグインを `plugins/` の下にコピーします：

    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
    cp -r my-first-plugin my-marketplace/plugins/
    ```

    プラグインが現在の場所で有効であることを確認して、後のエラーがマーケットプレイスについてであり、プラグインについてではないことを確認します：

    ```bash theme={null}
    claude plugin validate ./my-marketplace/plugins/my-first-plugin
    ```

    出力の最後の行は `✔ Validation passed` と表示されます。
  </Step>

  <Step title="マーケットプレイスファイルを作成する">
    `marketplace.json` を `my-marketplace/.claude-plugin/marketplace.json` に保存します。ファイルには `name`、`owner`、および `plugins` 配列が必要です。

    `plugins` の各オブジェクトはプラグインエントリであり、`name` と `source` が必要です。エントリの `source` をマーケットプレイスルートからのパスとして記述します。ルートは `my-marketplace/` で、`.claude-plugin/` を含むディレクトリです。

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-marketplace",
      "description": "Plugins for my team",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "my-first-plugin",
          "source": "./plugins/my-first-plugin",
          "description": "A greeting plugin to learn the basics"
        }
      ]
    }
    ```
  </Step>

  <Step title="マーケットプレイスを検証する">
    マーケットプレイスディレクトリで `claude plugin validate` を実行して、JSON 構文、必須フィールド、および `.claude-plugin/marketplace.json` の各プラグインエントリを確認します。

    ```bash theme={null}
    claude plugin validate ./my-marketplace
    ```

    ステップ 2 で記述されたファイルの場合、出力の最後の行は `✔ Validation passed` と表示されます。
  </Step>

  <Step title="マーケットプレイスを追加してプラグインをインストールする">
    ディレクトリをマーケットプレイスとして登録します。

    ```bash theme={null}
    claude plugin marketplace add ./my-marketplace
    ```

    コマンドは `✔ Successfully added marketplace: my-marketplace (declared in user settings)` と出力します。これはマーケットプレイスがユーザー設定ファイルに記録されたことを意味します。

    プラグインをインストールします。インストール ID はエントリの `name`、`@`、およびマーケットプレイスの `name` です。

    ```bash theme={null}
    claude plugin install my-first-plugin@my-marketplace
    ```

    コマンドは `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)` と出力します。

    セッション内では、`/plugin marketplace add ./my-marketplace` がマーケットプレイスを同じ方法で登録します。`/plugin install my-first-plugin@my-marketplace` は `/plugin` パネルでプラグインの詳細を開き、そこでインストールします。そのフローについては、[プラグインをインストールして管理する](/docs/ja/plugins/install)を参照してください。
  </Step>

  <Step title="プラグインが読み込まれたことを確認する">
    インストール済みプラグインをリストアップします。

    ```bash theme={null}
    claude plugin list
    ```

    出力は `my-first-plugin@my-marketplace` を `Status: ✔ enabled` でリストアップします。

    プラグインが読み込んだ内容を確認するには、その詳細を表示します。

    ```bash theme={null}
    claude plugin details my-first-plugin
    ```

    `Component inventory` セクションは `Skills (1)  hello` と表示されます。

    スキルを実行するには、セッションを開始して `/my-first-plugin:hello` を入力します。Claude があなたに挨拶します。コマンドはプラグインの名前をプレフィックスとして持ち、すべてのプラグインスキルの名前がそうであるように。
  </Step>
</Steps>

<h2 id="add-plugin-entries">
  プラグインエントリを追加する
</h2>

配布するすべてのプラグインは、`marketplace.json` の `plugins` 配列内の 1 つのオブジェクトです。2 番目のプラグインを追加するには、2 番目のオブジェクトを追加します。これらのフィールドはほとんどのエントリをカバーします：

* `name`：インストール時に `@` の前に入力する識別子。スペースを含むことはできません。
* `source`：Claude Code がプラグインを取得する場所。[チュートリアル](#create-a-marketplace)のようにマーケットプレイスディレクトリ内のプラグインの相対パス文字列、またはその外のプラグインのソースオブジェクトを記述します。[プラグインソースを選択する](#choose-a-plugin-source)を参照してください。
* `description`：ユーザーが `/plugin` でマーケットプレイスを参照するときにプラグインの横に表示される行。

完全なフィールドリストについては、[プラグインエントリ](/docs/ja/plugins/marketplace-reference#plugin-entries)を参照してください。

エントリは、任意の [`plugin.json`](/docs/ja/plugins/manifest-reference) フィールドも設定できます。エントリの `plugin.json` フィールドが独自の `plugin.json` を持つプラグインに適用される場合については、[エントリと plugin.json](/docs/ja/plugins/marketplace-reference#entry-and-plugin-json)を参照してください。

<h2 id="rules-for-plugin-entries">
  プラグインエントリのルール
</h2>

新しいマーケットプレイスからのほとんどの失敗したインストールは、相対パスが間違ったディレクトリから記述されているか、エントリ名がプラグインの `plugin.json` の `name` と異なることが原因です。

<h3 id="write-relative-paths-from-the-marketplace-root">
  マーケットプレイスルートから相対パスを記述する
</h3>

マーケットプレイスルートは `.claude-plugin/` を含むディレクトリです。[チュートリアル](#create-a-marketplace)では、それは `my-marketplace/` なので、エントリの `source` は `"./plugins/my-first-plugin"` です。パスは `.claude-plugin/` 内から始まらないため、それを離れるために `..` を使用しないでください。

`..` を含むパスと存在しないディレクトリへのパスは異なるコマンドで失敗します：

* **`..` を含むパス**：`claude plugin validate` はエントリを無効として報告します。メッセージは `Path contains "..": ./../plugins/my-first-plugin` で始まります。
* **存在しないディレクトリへのパス**：`claude plugin validate` は成功します。`claude plugin install` は `Source path does not exist: <path>` で失敗し、`<path>` は Claude Code が確認した絶対位置です。

<h3 id="keep-the-entry-name-and-the-manifest-name-the-same">
  エントリ名とマニフェスト名を同じに保つ
</h3>

マーケットプレイスプラグインは `marketplace.json` のエントリ `name` と独自の `plugin.json` の `name` を持ち、マニフェスト名と呼ばれます。各名前は異なる場所に表示されます：

* **エントリ名**：インストール ID、`<entry-name>@<marketplace>`。これはインストール時に入力する内容、`claude plugin list` が表示する内容、および Claude Code が設定ファイルの [`enabledPlugins`](/docs/ja/settings-reference#enabledplugins) の下に記述するキーです。
* **マニフェスト名**：プラグインのスキルのプレフィックス、および `claude plugin details` が受け取る名前。

2 つの名前が異なり、誰かがマニフェスト名でインストールすると、Claude Code は `Plugin "<manifest-name>" not found in marketplace "<marketplace>"` を報告します。2 つの名前を同じに保ちます。Claude Code が 2 つの名前をどのように使用するかについての詳細については、[プラグイン読み込みリファレンス](/docs/ja/plugins/loading#find-where-a-plugin-came-from)を参照してください。

<h2 id="choose-a-plugin-source">
  プラグインソースを選択する
</h2>

`marketplace.json` の各プラグインエントリには、Claude Code がそのプラグインを取得する場所を指示する `source` があります。プラグインのファイルが保存されている場所によってソースを選択します。テーブルはマーケットプレイス所有者が最も使用するソースをリストアップします。

| ソース          | 使用する場合                          | 最小限の `source` 値                                                                           |
| :----------- | :------------------------------ | :---------------------------------------------------------------------------------------- |
| 相対パス         | プラグインのファイルがマーケットプレイスディレクトリ内にある  | `"./plugins/my-first-plugin"`                                                             |
| `github`     | プラグインが独自の GitHub リポジトリである       | `{ "source": "github", "repo": "your-org/my-first-plugin" }`                              |
| `git-subdir` | プラグインがモノレポなど他のリポジトリのサブディレクトリである | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

`git-subdir` ソースでは、`url` は git URL または `owner/repo` GitHub ショートハンドを受け取ります。

プラグインは、これらのソースタイプの 1 つからも取得できます：

* `url`：任意のホスト上の git リポジトリ（URL による）
* `archive`：HTTPS 経由でダウンロードされた zip ファイル
* `npm`：npm パッケージ
* `command`：プラグインがインストールされているマシンでコマンドを実行して生成されたディレクトリ

すべてのソースタイプのフィールド、および git ベースのソースを `ref` または `sha` にピン留めするには、[プラグインソース](/docs/ja/plugins/marketplace-reference#plugin-sources)を参照してください。

<h2 id="validate-and-test">
  検証とテスト
</h2>

プラグインを追加するときは、編集後にシェルで `claude plugin validate ./my-marketplace` を実行し、共有する前に独自のマシンのマーケットプレイスからインストールします。検証とインストールは異なる問題をキャッチします。

<h3 id="problems-that-validation-reports">
  検証が報告する問題
</h3>

`claude plugin validate` はマーケットプレイスディレクトリ内のファイルのみを読み取ります。以下を報告します：

* JSON 構文エラー（`json: Invalid JSON syntax: <reason>` として）
* `owner: Invalid input` などの必須フィールドの欠落
* スペース、非 ASCII 文字、または `claude-official` などの公式 Anthropic マーケットプレイスを模倣する形式を持つマーケットプレイス名
* `..` を含む相対 `source`
* トップレベルまたはプラグインエントリの未知のフィールド（警告として）
* 各相対パスプラグインの `plugin.json` の問題（`plugins[N] plugin.json → <field>: <message>` として）

`validate` が出力できるすべてのメッセージについては、[検証メッセージ](/docs/ja/plugins/marketplace-reference#validation-messages)を参照してください。そのフラグと終了コードについては、[`plugin validate`](/docs/ja/plugins/cli-reference#plugin-validate)を参照してください。

<h3 id="problems-that-surface-when-you-add-or-install">
  マーケットプレイスを追加またはインストールするときに表示される問題
</h3>

`claude plugin validate` が報告しない問題は、マーケットプレイスを追加またはそこからインストールするときに表示されます：

* **マーケットプレイスを追加するとき**：`claude-plugins-official` などの正確な[公式マーケットプレイス名](/docs/ja/plugins/marketplace-reference#reserved-names)は検証に合格します。これらの名前の 1 つを持つマーケットプレイスを追加すると、Claude Code は `The name '<name>' is reserved for official Anthropic marketplaces` で始まるメッセージで拒否します。
* **プラグインをインストールするとき**：
  * Claude Code は、プラグインをインストールするときに `github`、`git-subdir`、または他のリモートソースを最初に取得するため、間違った `repo` または `path` はその時点で表示されます。
  * ディレクトリが存在しない相対 `source` も、`Source path does not exist: <path>` でインストール時に失敗します。

<h3 id="test-an-edit-to-a-plugin">
  プラグインの編集をテストする
</h3>

[チュートリアル](#create-a-marketplace)では、相対パス `source` を持つローカルディレクトリから `my-marketplace` を追加しました。そのセットアップでは、Claude Code は `my-marketplace/plugins/` からプラグインのファイルを直接読み取ります。編集は次のセッション開始時または `reload-plugins` をセッション内で実行するときに有効になり、プラグインの `version` に変更はありません。

ホストされたマーケットプレイスからインストールする人は、代わりにプラグインキャッシュにコピーを取得します。新しいバージョンを受け取る方法については、[ユーザーを最新に保つ](/docs/ja/plugins/host-marketplace#keep-users-up-to-date)を参照してください。

<h3 id="remove-the-marketplace-to-start-over">
  マーケットプレイスを削除して最初からやり直す
</h3>

すべてを削除して最初からやり直すには、シェルで `claude plugin marketplace remove my-marketplace` を実行します。コマンドはマーケットプレイスを削除し、そのプラグインをアンインストールします。

<h2 id="host-your-marketplace">
  マーケットプレイスをホストする
</h2>

[マーケットプレイスを作成する](#create-a-marketplace)のように、独自のマシンのマーケットプレイスからプラグインをインストールできるようになったら、マーケットプレイスディレクトリを git ホストにプッシュします。

チームメイトは、GitHub リポジトリの場合はシェルで `claude plugin marketplace add <owner>/<repo>` を実行するか、リポジトリ URL で同じコマンドを実行します。その後、[チュートリアル](#create-a-marketplace)のように名前でプラグインをインストールします。

プライベートリポジトリアクセス、更新、バージョン管理、およびエントリの名前変更または削除については、[マーケットプレイスをホストして維持する](/docs/ja/plugins/host-marketplace)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

* [マーケットプレイスをホストして維持する](/docs/ja/plugins/host-marketplace)：ホストを選択し、ユーザーを最新に保ち、プラグインを安全に名前変更または削除します
* [マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference)：`marketplace.json` フィールドとソースタイプ
* [組織のプラグインを管理する](/docs/ja/plugins/org)：すべてのマシンでマーケットプレイスとそのプラグインを必須にします
* [関連性によってプラグインを提案する](/docs/ja/plugins/relevance)：セッションが一致するときにマーケットプレイスからプラグインを提案するように Claude Code に指示します
