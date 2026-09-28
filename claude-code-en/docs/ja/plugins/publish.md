> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインを公開・配布する

> Claude Code プラグインを独自のマーケットプレイスまたは Anthropic のコミュニティマーケットプレイスを通じて公開し、リリース前チェックリストとユーザーが更新を受け取る方法について説明します。

Claude Code プラグインを公開するとは、プラグインをマーケットプレイスにリストアップすることです。マーケットプレイスは、プラグインをリストアップし、各プラグインをどこから取得するかを示す JSON カタログです。これにより、他のユーザーはプラグインを名前でインストールでき、あなたの更新を受け取ることができます。独自のマーケットプレイスを実行することも、プラグインを Anthropic のコミュニティマーケットプレイスに提出することもできます。プラグインを公開せずに共有するには、プラグインのディレクトリまたは `.zip` ファイルを送信して、ユーザーが自分でロードできるようにします。

このページは、共有する準備ができている完成したプラグインの作成者向けです。

<Note>
  以下のケースは他のページで説明されています。

  * **プラグインがまだ完成していない場合**: [プラグインを作成する](/docs/ja/plugins/create)から始めてください
  * **公式マーケットプレイスにプラグインを含む CLI または SDK を保守している場合**: [CLI からプラグインを推奨する](/docs/ja/plugins/cli-hints)を参照してください
</Note>

[配布方法を選択する](#choose-how-to-distribute)から始めて、配布オプションを比較してください。既に配布方法を決めている場合は、[リリース用にプラグインを準備する](#prepare-your-plugin-for-release)に進み、その後、ユーザーに何を伝えるか、ユーザーがどのように更新を受け取るかについて、あなたの配布方法のセクションに従ってください。

<h2 id="choose-how-to-distribute">
  配布方法を選択する
</h2>

プラグインをインストールする必要があるユーザーに基づいて、配布オプションを選択してください。

| 配布方法                                                               | インストール可能なユーザー                                       | 必要なもの                                                                   | ユーザーは自動的に更新を受け取りますか？ |
| :----------------------------------------------------------------- | :-------------------------------------------------- | :---------------------------------------------------------------------- | :------------------- |
| [マーケットプレイスなし](#share-a-plugin-without-a-marketplace)               | プラグインフォルダまたはその `.zip` ファイルを送信したユーザー                 | プラグインのフォルダ                                                              | いいえ。送信されたコピーをロードします  |
| [独自のマーケットプレイス](#publish-through-your-own-marketplace)              | リポジトリにアクセスできるすべてのユーザー。リポジトリはプライベートでもかまいません          | `.claude-plugin/marketplace.json` を含む git リポジトリまたは他のホスト。プラグインをリストアップします | オフ                   |
| [Anthropic のコミュニティマーケットプレイス](#submit-to-the-community-marketplace) | `anthropics/claude-plugins-community` を追加したすべてのユーザー | プラグインディレクトリ送信フォーム経由での送信                                                 | オフ                   |

自動更新は、ユーザー側のマーケットプレイスごとの設定で、バックグラウンドで新しいバージョンを取得します。

<h2 id="prepare-your-plugin-for-release">
  リリース用にプラグインを準備する
</h2>

名前、バージョン、検証、およびマーケットプレイスからのインストールが、リリースがインストールするユーザーに対して機能するかどうかを決定します。最初のリリースの前に、そして後の各リリースの前に確認してください。

<Steps>
  <Step title="永続的な名前を選択する">
    ユーザーは `name@marketplace` でプラグインをインストール、有効化、および設定するため、名前を変更したプラグインは既存のすべてのインストールに対して異なるプラグインになります。`deploy-helper` などのケバブケース名を選択してください。`claude plugin validate` は他の形式に対して警告を出すため、これを永続的なものとして扱ってください。`plugin.json` で `displayName` を設定して、ユーザーが見るラベルを指定します。
  </Step>

  <Step title="バージョン管理方法を決定する">
    `plugin.json` で `version` を設定し、後でコミットをプッシュするときにそれを変更しない場合、`claude plugin update` は `<name> is already at the latest version (1.0.0).` と出力し、ユーザーは古いコピーを保持します。すべてのリリースで `version` をインクリメントするか、git でホストされているマーケットプレイスで `version` を省略して、Claude Code がコミット SHA を代わりに使用するようにしてください。[バージョンと更新](/docs/ja/plugins/loading#versions-and-updates)を参照してください。
  </Step>

  <Step title="検証する">
    シェルで `claude plugin validate --strict ./your-plugin` を実行してください。クリーンな実行は `✔ Validation passed` と出力します。

    * **CI 内**: `--strict` を保持してください。これは、不明なマニフェストフィールドや欠落している `version` などの警告に対して、終了コード 1 で実行を失敗させます。前のステップで `version` を省略することを選択した場合は、`--strict` を削除してください。
    * **パス**: 検証は `./` で始まらないコンポーネントパスを報告します。hook コマンドと MCP サーバー設定内では、ファイルを `${CLAUDE_PLUGIN_ROOT}/...` として参照してください。[パスルール](/docs/ja/plugins/manifest-reference#path-rules)を参照してください。
  </Step>

  <Step title="ローカルマーケットプレイスからインストールする">
    シェルで、`claude plugin marketplace add ./path-to-marketplace` でプラグインをリストアップするローカルマーケットプレイスを追加し、そこからプラグインをインストールして、セッションを開始してロードされることを確認します。

    * 最小限のマーケットプレイスについては、[マーケットプレイスを作成する](/docs/ja/plugins/create-marketplace)を参照してください。
    * インストールがソースディレクトリをロードするか、キャッシュされたコピーをロードするかを知るには、[インプレイスおよびコピーされたプラグイン](/docs/ja/plugins/loading#in-place-and-copied-plugins)を参照してください。
  </Step>

  <Step title="ユーザーが見るメタデータを入力する">
    `plugin.json` で `description`、`author`、`homepage`、および `repository` を設定し、プラグインルートに `README.md` を追加してください。`homepage` は URL として解析可能である必要があります。[マニフェストリファレンス](/docs/ja/plugins/manifest-reference#fields)はすべてのフィールドをリストアップしています。
  </Step>

  <Step title="eval スイートを実行する">
    eval スイートがある場合は、シェルで `claude plugin eval` を実行してください。プラグインのテストケースを実行し、結果をスコアリングします。これにより、プラグインを変更するときの回帰を検出します。[eval でプラグインをテストする](/docs/ja/plugin-evals)を参照してください。
  </Step>
</Steps>

<h2 id="share-a-plugin-without-a-marketplace">
  マーケットプレイスなしでプラグインを共有する
</h2>

プラグインが git リポジトリにある場合、ユーザーはそれをクローンしてチェックアウトをロードするか、シェルから `--plugin-url` をリリースに添付した `.zip` に向けて `claude` で Claude Code を開始できます。次のバージョンを取得するには、プルまたは再度ダウンロードします。リポジトリにない場合は、ディレクトリまたはその `.zip` を送信してください。ユーザーは次の 2 つの方法のいずれかでロードできます。

* **1 つのセッション用**: `claude --plugin-dir ./deploy-helper` でシェルから Claude Code を開始します。パスはクローン、解凍されたフォルダ、または `.zip` ファイル自体です。[1 つのセッション用にプラグインをロードするフラグ](/docs/ja/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)を参照してください。
* **すべてのセッション用**: プラグインディレクトリを `.claude-plugin/plugin.json` とともに `~/.claude/skills/` の下に移動して、Claude Code が[すべてのセッションでロード](/docs/ja/plugins/loading#find-where-a-plugin-came-from)するようにします。

同じリポジトリに `.claude-plugin/marketplace.json` を追加することで、ユーザーは名前でインストールでき、コマンドで更新できます。[独自のマーケットプレイスを通じて公開する](#publish-through-your-own-marketplace)を参照してください。

<h3 id="ship-a-plugin-with-your-own-tool">
  独自のツールでプラグインを配布する
</h3>

CLI または SDK を保守している場合は、プラグインをマーケットプレイスで公開し、インストーラーまたはインストール後のメッセージで、ユーザーが必要とする 2 つのコマンドを実行または出力するようにしてください。`claude plugin marketplace add <source>`、その後 `claude plugin install <name>@<marketplace>`。ユーザーがツールを使用するときのセッション内発見については、[CLI からプラグインを推奨する](/docs/ja/plugins/cli-hints)を参照してください。

<h2 id="publish-through-your-own-marketplace">
  独自のマーケットプレイスを通じて公開する
</h2>

独自のマーケットプレイスは、git リポジトリに追加された `.claude-plugin/marketplace.json` ファイルで、プラグインをリストアップします。ファイルがリポジトリに含まれると、プラグインは公開され、送信フォームはありません。ファイルをプラグイン自体のリポジトリまたは別のリポジトリに保持できます。

<h3 id="add-the-marketplace-file-to-your-repository">
  マーケットプレイスファイルをリポジトリに追加する
</h3>

プラグイン自体のリポジトリから公開するには、マーケットプレイスファイルを `.claude-plugin/` の `plugin.json` の横に保存し、`source` が `"./"` であるエントリを 1 つ持たせます。これはリポジトリルートです。エントリに `plugin.json` と同じ `name` を付けてください。[エントリ名とマニフェスト名を同じに保つ](/docs/ja/plugins/create-marketplace#keep-the-entry-name-and-the-manifest-name-the-same)を参照してください。

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

シェルで、リポジトリで `claude plugin validate .` を実行して、プッシュする前にファイルを確認してください。

[マーケットプレイスを作成する](/docs/ja/plugins/create-marketplace)は、1 つのリポジトリに複数のプラグインがある場合のレイアウトをカバーしています。

<h3 id="control-who-can-install">
  インストール可能なユーザーを制御する
</h3>

リポジトリをクローンできるすべてのユーザーがそこからインストールできるため、リポジトリがプライベートの場合、マーケットプレイスもプライベートです。git リポジトリ以外のホストについては、[マーケットプレイスをホストする](/docs/ja/plugins/host-marketplace)を参照してください。git を使用しない人を含む会社全体に到達するには、[会社全体にロールアウトする](/docs/ja/plugins/host-marketplace#roll-out-to-a-whole-company)を参照してください。

<h3 id="tell-users-how-to-install">
  ユーザーにインストール方法を伝える
</h3>

ユーザーにマーケットプレイスを追加してからプラグインをインストールするよう指示してください。シェルから、ソースと名前をあなたのものに置き換えてください。

* マーケットプレイスを 1 回追加する: `claude plugin marketplace add your-org/your-marketplace`。引数は GitHub の `owner/repo` 短縮形、URL、またはパスです
* プラグインをインストールする: `claude plugin install deploy-helper@your-marketplace`
* またはセッション内から両方を実行する: `/plugin install deploy-helper --marketplace your-org/your-marketplace`。Claude Code v2.1.275 以降が必要です。[マーケットプレイスを追加して 1 つのコマンドでインストールする](/docs/ja/plugins/install#add-a-marketplace-and-install-in-one-command)を参照してください

<h3 id="ship-updates-to-users">
  ユーザーに更新を配布する
</h3>

ユーザーはリクエストするか、マーケットプレイスで自動更新がオンの場合にリリースを受け取ります。

* **リクエスト時**: ユーザーのシェルで `claude plugin update deploy-helper@your-marketplace` を実行すると、マーケットプレイスが更新され、プラグインのバージョンが変更されたときに新しいコピーがインストールされます
* **自動更新**: デフォルトではマーケットプレイスでオフです。[自動更新をオンにする](/docs/ja/plugins/host-marketplace#turn-on-auto-update)を参照してください。オンになると、セッション開始後の遅延で `claude plugin update` と同じことを実行します

[プラグインをインストールする](/docs/ja/plugins/install)はユーザー側のコマンドをカバーし、[自動更新が実行される時期](/docs/ja/plugins/loading#when-auto-update-runs)はタイミングをカバーしています。

<h2 id="submit-to-the-community-marketplace">
  コミュニティマーケットプレイスに提出する
</h2>

Anthropic のコミュニティマーケットプレイス `claude-community` は、プラグインディレクトリ送信フォーム経由で提出されたプラグインをリストアップする公開マーケットプレイスです。

ユーザーは Claude Code セッション内で `/plugin marketplace add anthropics/claude-plugins-community` でコミュニティマーケットプレイスを追加し、`@claude-community` としてそこからインストールします。

コミュニティマーケットプレイスが公式マーケットプレイスとどのように異なるかについては、[Anthropic のマーケットプレイス](/docs/ja/plugins/anthropic-marketplaces)を参照してください。

プラグインをコミュニティマーケットプレイスに提出するには、アプリ内フォームのいずれかを使用してください。

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

claude.ai フォームには Team または Enterprise 組織と、デフォルトでオーナーが保持するディレクトリ権限が必要です。Team または Enterprise 組織に属していない個別の作成者は、代わりに Console フォームを使用できます。

シェルで、提出する前に `claude plugin validate ./your-plugin` をローカルで実行してください。`./your-plugin` をプラグインディレクトリへのパスに置き換えてください。検証が成功すると、Claude Code は `✔ Validation passed` または警告がある場合は `✔ Validation passed with warnings` と出力します。警告は検証を失敗させません。`--strict` を追加して、警告をエラーとして扱ってください。

リストアップされたプラグインは [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) カタログに表示され、ほぼすべての場合、特定のコミット SHA にピン留めされます。

提出とプラグインが `marketplace.json` に表示されるまでの間に遅延がある可能性があります。プラグインがインストール可能かどうかを確認するには、[コミュニティカタログ](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json)でその名前を検索してください。

公式マーケットプレイス `claude-plugins-official` は、これらのフォーム経由での提出を受け付けていません。Anthropic パートナー連絡先と協力している場合は、公式マーケットプレイスのリストアップについて尋ねてください。

<h2 id="ship-updates-renames-and-removals">
  更新、名前変更、および削除を配布する
</h2>

<h3 id="release-a-new-version">
  新しいバージョンをリリースする
</h3>

独自のマーケットプレイスを通じて公開し、`plugin.json` が `version` を設定している場合は、それをインクリメントしてプッシュしてください。`claude plugin update` を実行するか、自動更新がオンのユーザーは、[ユーザーに更新を配布する](#ship-updates-to-users)の下で説明されているように、新しいバージョンを受け取ります。

<h3 id="tag-a-release">
  リリースにタグを付ける
</h3>

他のプラグインがあなたのプラグインのバージョン範囲を宣言する場合、git でリリースにタグを付けてください。これらの範囲はタグに対して解決されるためです。それ以外の場合、タグは必要ありません。

タグを付けるには、プラグインディレクトリからシェルで `claude plugin tag` を実行してください。`{name}--v{version}` タグを作成します。`--push` を追加して、タグを `origin` に送信してください。[`plugin tag` リファレンス](/docs/ja/plugins/cli-reference#plugin-tag)はそのフラグをリストアップしています。

<h3 id="rename-or-remove-a-plugin">
  プラグインの名前を変更または削除する
</h3>

公開されたプラグインの `name` を変更しないでください。名前変更後、既にインストールしたユーザーはプラグインを失います。インストールは古い名前の下に記録されるためです。マーケットプレイスファイルの `renames` エントリは、代わりに既存のインストールを移行します。別のラベルが必要な場合は `displayName` を変更してください。

名前変更が避けられない場合は、マーケットプレイスファイルの `renames` マップを使用して、既存のインストールが [`Plugin "<name>" not found in marketplace`](/docs/ja/plugins/troubleshooting#plugin-not-found-in-marketplace) で失敗する代わりに移行するようにしてください。マーケットプレイスからプラグインを削除するか、完全な `renames` の詳細については、ホスティングページの [プラグインの名前を変更または削除する](/docs/ja/plugins/host-marketplace#rename-or-remove-a-plugin)を参照してください。[マーケットプレイスリファレンス](/docs/ja/plugins/marketplace-reference#top-level-fields)にはフィールドがあります。

<h2 id="declare-dependencies">
  依存関係を宣言する
</h2>

プラグインが同じマーケットプレイスから別のプラグインが有効化されている必要がある場合は、`plugin.json` の `dependencies` 配列にリストアップしてください。各エントリは、ベアネームまたは semver `version` 範囲を持つオブジェクトです。ユーザーがプラグインをインストールすると、Claude Code は依存関係もインストールして有効化します。

[プラグイン依存関係](/docs/ja/plugins/dependencies)は、範囲構文、クロスマーケットプレイス依存関係、およびユーザーが不要になった依存関係をプルーニングする方法をカバーしています。

<h2 id="next-steps">
  次のステップ
</h2>

* [マーケットプレイスをホストして保守する](/docs/ja/plugins/host-marketplace): 新しいバージョンをリリースしてユーザーを最新の状態に保つ
* [プラグイン依存関係](/docs/ja/plugins/dependencies): プラグインが依存するプラグインを宣言してバージョン管理する
* [CLI からプラグインを推奨する](/docs/ja/plugins/cli-hints): CLI のユーザーに Claude Code プラグインをインストールするよう促す
* [プラグインのコストと使用状況を測定する](/docs/ja/plugins/measure): プラグインがコンテキストでコストがいくらかかるか、そしてユーザーがそれを使用しているかどうかを確認する
