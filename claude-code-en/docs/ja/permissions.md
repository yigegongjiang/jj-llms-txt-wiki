> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 権限を設定する

> きめ細かい権限ルール、モード、管理ポリシーを使用して、Claude Code がアクセスして実行できる内容を制御します。

Claude Code は、エージェントが実行できることと実行できないことを正確に指定できるようにするため、きめ細かい権限をサポートしています。権限設定はバージョン管理にチェックインでき、組織内のすべての開発者に配布できるほか、個々の開発者がカスタマイズできます。

<h2 id="permission-system">
  権限システム
</h2>

Claude Code は、パワーと安全性のバランスを取るために、段階的な権限システムを使用しています。表は、各ツールタイプについて、Manual モードがアクションの実行前に確認するかどうかを示しています。その他の[権限モード](#permission-modes)は、これらのどれがあなたに確認するかを変更します。auto モードでは、分類器があなたの代わりにアクションをレビューし、[分類器がアクションをどのように評価するか](/docs/ja/permission-modes#how-the-classifier-evaluates-actions)は、それが見るアクションをリストアップしています。

| ツールタイプ    | 例               | 承認が必要                                                                             | 「はい、今後は聞かない」の動作  |
| :-------- | :-------------- | :-------------------------------------------------------------------------------- | :--------------- |
| 読み取り専用    | ファイル読み取り、Grep   | いいえ、[作業ディレクトリと追加ディレクトリ](#working-directories)内                                    | N/A              |
| Bash コマンド | シェル実行           | はい、[読み取り専用コマンド](#read-only-commands)の組み込みセットを除く                                   | リポジトリとコマンドごとに永続的 |
| ファイル変更    | Edit/Write ファイル | はい                                                                                | セッション終了まで        |
| Web フェッチ  | WebFetch        | はい、[事前承認されたドキュメンテーションドメイン](/docs/ja/tools-reference#webfetch-tool-behavior)の組み込みセットを除く | リポジトリとドメインごとに永続的 |
| Web 検索    | WebSearch       | はい                                                                                | リポジトリごとに永続的      |

「はい、今後は聞かない」を選択し、承認が永続的に保存される場合（Bash コマンドや WebFetch ドメインなど）、Claude Code はルールを git リポジトリのルートにある `.claude/settings.local.json` に保存します。これは[worktrees](/docs/ja/worktrees)を通じてメインチェックアウトに解決されます。ルールは、そのリポジトリ内のサブディレクトリで開始されたセッションや worktrees 内のセッションを含む、そのリポジトリ内の将来のセッションに適用されます。ファイル変更の承認はファイルに保存されません。表が示すように、セッション終了まで続きます。git リポジトリの外部や Windows 上など、場合によっては Claude Code はリポジトリルートを使用しません。[Claude Code が各ファイルを探す場所](/docs/ja/settings#where-claude-code-looks-for-each-file)は、これらのケースと代わりにルールを保存する場所をリストアップしています。

v2.1.211 より前では、Claude Code は常にルールを開始ディレクトリに保存していたため、worktree またはサブディレクトリで付与された承認はリポジトリの残りの部分に適用されませんでした。以前のバージョンがサブディレクトリまたは worktree に保存したルールは、そこで開始されたセッションに引き続き適用されます。

場合によっては、権限プロンプトは 1 回限りの承認のみを提供し、「今後は聞かない」オプションもセッションの残りの部分のアクションを許可するオプションもありません。Claude Code はプロンプトがそれらが許可するすべてのものをあなたに表示できる場合にのみ、これらのオプションを提供するため、プロンプトから保存するルールは、その名前のオプションが許可するものだけをカバーします。プロンプトが 1 回限りの承認のみを提供する場合、アクションを 1 回承認するか、[`/permissions`](#manage-permissions)でルール自体を追加してください。

<h3 id="add-a-comment-when-you-answer-a-permission-prompt">
  権限プロンプトに回答するときにコメントを追加する
</h3>

権限プロンプトに回答するときに、単一のアクションを承認または拒否するときに、Claude にメモを添付できます。Bash、PowerShell、ファイル、MCP ツールプロンプトを含むほとんどの権限プロンプトで、**はい**または**いいえ**に移動し、`Tab` を押してそのオプションのコメントフィールドを開きます。WebFetch とブラウザプロンプトはフィールドを提供しません。セッションの残りの部分のアクションを許可するか、ルールを保存するオプションも 1 つを取りません。

フィールドが開いている状態で、コメントを入力してから、これらのキーのいずれかを押します。

* `Enter`：コメントが添付された状態で回答を送信します。フィールドを空のままにすると、Claude Code はコメントなしで回答を送信します。
* `Tab`：フィールドを閉じて回答しません。Claude Code は入力したテキストを保持し、そのオプションで回答する場合でも送信します。
* `Shift+Tab`：ファイルプロンプト（Edit または Write プロンプトなど）では、フィールドを `Tab` と同じように閉じます。v2.1.235 より前では、フィールド内で `Shift+Tab` を押すと、セッションの残りの部分のアクションを許可するオプションが選択されたため、Claude Code はセッションの残りの部分のアクションを承認し、コメントを破棄しました。

Claude Code は、回答方法に応じてコメントを異なる方法で配信します。

* **はい**：Claude Code はアクションを実行し、結果の後に Claude にコメントを送信します。
* **いいえ**：Claude Code はコメントを Claude に拒否の理由として送信し、Claude は作業を続けます。メインの会話からのプロンプトでコメントなしで**いいえ**を選択すると、Claude Code はターンを停止します。

<h2 id="manage-permissions">
  権限を管理する
</h2>

`/permissions` を使用して、Claude Code のツール権限を表示および管理できます。このダイアログは、すべての権限ルールと、各ルールが取得される `settings.json` ファイルをリストします。Claude が作業中にダイアログを開くことができます。ルールを追加または削除すると、Claude Code は同じターン内の Claude の次のツール呼び出しから変更を適用します。v2.1.234 より前では、Claude Code はターンが終了するまでコマンドをキューに入れていました。

* **Allow** ルールは、Claude Code が手動承認なしで指定されたツールを使用できるようにします。
* **Ask** ルールは、Claude Code が指定されたツールを使用しようとするたびに確認を促します。
* **Deny** ルールは、Claude Code が指定されたツールを使用することを防止します。

ルールは順序で評価されます。deny、ask、allow の順です。その順序での最初のマッチがアウトカムを決定し、ルールの特異性は順序を変更しません。

`Bash(aws *)` のような広い deny ルールは、`Bash(aws s3 ls)` のようなより狭い allow ルールにもマッチする呼び出しを含む、マッチするすべての呼び出しをブロックします。allow ルールは deny ルールから例外を作成することはできません。ask と allow の間にも同じ優先順位が適用されます。マッチする ask ルールは、同じ呼び出しにマッチするより具体的な allow ルールがある場合でも、プロンプトを表示します。

Deny ルールは、ツール名を指定するか、ツール内のパターンをスコープするかによって異なる動作をします。`Bash` のようなベアツール名は、ツールを Claude のコンテキストから完全に削除するため、Claude はそれを見ることはありません。セッション中にそのようなルールを追加する場合、Claude は次のツール呼び出しからそのツールを呼び出すことができません。[ツール全体を拒否する](/docs/ja/prompt-caching#denying-an-entire-tool)では、Claude が既に見た定義に何が起こるかについて説明しています。`Bash(rm *)` のようなスコープ付きルールは、ツールを利用可能なままにし、Claude が試みたときにマッチする呼び出しをブロックします。

ベア名削除はすべてのツールに適用されます（[`EndConversation`](/docs/ja/tools-reference#endconversation-tool-behavior) を除く）。deny ルールは他のツールが残っている間はそれを削除できず、ask ルールはそれに対してプロンプトを表示しません。

<Note>
  権限ルールは Claude Code によって実装されており、モデルによってではありません。プロンプトまたは `CLAUDE.md` の指示は、Claude が何をしようとするかを形作りますが、Claude Code が許可する内容は変わりません。アクセスを付与または取り消すには、`/permissions`、ここで説明されているルール、[permission mode](/docs/ja/permission-modes)、または [PreToolUse hook](#extend-permissions-with-hooks) を使用してください。
</Note>

[auto mode](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) がセッションで利用可能な場合、ダイアログには [auto mode classifier rules](/docs/ja/auto-mode-config#edit-rules-from-permissions) も含まれます。**Auto mode** タブを選択して、それらを表示します。

<h2 id="permission-modes">
  権限モード
</h2>

Claude Code は、ツール呼び出しの承認方法を制御するいくつかの権限モードをサポートしています。[権限モード](/docs/ja/permission-modes)を参照して、各モードをいつ使用するかを確認してください。セッションが開始される際のモードを変更するには、[設定ファイル](/docs/ja/settings#where-settings-live)で `defaultMode` を設定してください。[セッションが開始されるモード](/docs/ja/permission-modes#which-mode-a-session-starts-in)では、各プランの組み込みデフォルトと VS Code 拡張機能が読み込む内容について説明しています。

| モード                 | 説明                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | 各ツールの最初の使用時に権限を促します。CLI、VS Code と JetBrains 拡張機能、およびデスクトップアプリでは Manual とラベル付けされており、Claude Code は `manual` をエイリアスとして受け入れます。ラベルとエイリアスには Claude Code v2.1.200 以降が必要です。デスクトップアプリのラベルは CLI バージョンに依存しません                                                                                                                                                                                               |
| `acceptEdits`       | ファイル編集と一般的なファイルシステムコマンド（`mkdir`、`touch`、`mv`、`cp` など）を、作業ディレクトリまたは `additionalDirectories` 内のパスに対して自動的に受け入れます                                                                                                                                                                                                                                                                                    |
| `plan`              | Claude はファイルを読み取り、読み取り専用シェルコマンドを実行して探索しますが、ソースファイルを編集しません。[auto モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)が利用可能で、分類器が承認したコマンドも実行されます。CLI および VS Code 拡張機能では Plan とラベル付けされています                                                                                                                                                                                                   |
| `auto`              | バックグラウンド安全チェック付きでツール呼び出しを自動承認し、アクションがリクエストと一致することを確認します                                                                                                                                                                                                                                                                                                                                          |
| `dontAsk`           | その他の場合はプロンプトを表示するすべての呼び出しを自動的に拒否します。作業ディレクトリ内のファイル読み取りおよび承認が不要なその他のアクションは実行されます。`/permissions` または `permissions.allow` ルール経由で事前に承認されたツールも実行されます。`AskUserQuestion`、MCP ツール（[`requiresUserInteraction`](/docs/ja/mcp#require-approval-for-a-specific-tool)とマークされたもの）、およびコネクタツール（[組織が `ask` に設定したもの](/docs/ja/mcp#organization-controls-on-connector-tools)）は、その設定が Claude Code に到達するセッションでは、許可していてもすべて拒否されます |
| `bypassPermissions` | 権限プロンプトをスキップします。ただし、[どのモードも自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves)は除きます                                                                                                                                                                                                                                                                                                |

<Warning>
  `bypassPermissions` モードでは、Claude Code は権限プロンプトをスキップします。これには [保護されたパス](/docs/ja/permission-modes#protected-paths)（`.git` や `.claude` など）への書き込みも含まれます。[クロスセッションメッセージングセーフガード](/docs/ja/permission-modes#skip-all-checks-with-bypasspermissions-mode)は引き続き適用されます。このモードは、Claude Code が損害を引き起こせないコンテナや VM などの隔離された環境でのみ使用してください。
</Warning>

`bypassPermissions` または `auto` モードが使用されるのを防ぐには、任意の[設定ファイル](/docs/ja/settings#where-settings-live)で `permissions.disableBypassPermissionsMode` または `permissions.disableAutoMode` を `"disable"` に設定してください。これらは、オーバーライドできない[管理設定](#managed-settings)で最も有用です。

<h2 id="permission-rule-syntax">
  権限ルール構文
</h2>

権限ルールは、`Tool` または `Tool(specifier)` の形式に従います。スペシファイア内の括弧はリテラルであるため、括弧を含むコマンドまたはパスはエスケープが不要です。

<h3 id="match-all-uses-of-a-tool">
  ツールのすべての使用をマッチさせる
</h3>

ツールのすべての使用をマッチさせるには、括弧なしでツール名を使用します。

| ルール        | 効果                       |
| :--------- | :----------------------- |
| `Bash`     | すべての Bash コマンドをマッチさせます   |
| `WebFetch` | すべてのウェブフェッチリクエストをマッチさせます |
| `Read`     | すべてのファイル読み取りをマッチさせます     |

`Bash(*)` は `Bash` と同等で、すべての Bash コマンドをマッチさせます。拒否ルールとして、両方の形式は Claude のコンテキストからツールを削除します。

<h3 id="use-specifiers-for-fine-grained-control">
  細かい制御のためにスペシファイアを使用する
</h3>

括弧内にスペシファイアを追加して、特定のツール使用をマッチさせます。

| ルール                            | 効果                                    |
| :----------------------------- | :------------------------------------ |
| `Bash(npm run build)`          | 正確なコマンド `npm run build` をマッチさせます      |
| `Read(./.env)`                 | 現在のディレクトリの `.env` ファイルを読み取ることをマッチさせます |
| `WebFetch(domain:example.com)` | example.com へのフェッチリクエストをマッチさせます       |

<h3 id="match-by-input-parameter">
  入力パラメータでマッチさせる
</h3>

拒否ルールと確認ルールは、`Tool(param:value)` を使用して任意のビルトインツール上のトップレベル入力パラメータをマッチさせることができます。

MCP ツール上のパラメータをマッチさせるには、[`--disallowedTools`](/docs/ja/cli-reference#cli-flags) を使用して拒否ルールを渡します。Claude Code が設定ファイルを読み込むとき、括弧を持つ `mcp__` ルールをスキップします。Claude Code は、インタラクティブセッションが開始されるときに無効な設定ダイアログでスキップされたルールをリストし、[`claude doctor`](/docs/ja/debug-your-config#check-resolved-settings) 出力に表示します。

パラメータルールは、Claude がそのパラメータをその正確な値に設定してツールを呼び出すときにマッチします。1 つのパラメータ値に対する許可ルールは、その呼び出しが全体的に安全であることを確立しないため、許可ルールは各ツール独自のスペシファイア構文を使用し続けます。これはツールが受け入れるスカラーパラメータで機能します。

| ルール                            | マッチ                              |
| :----------------------------- | :------------------------------- |
| `Agent(model:opus)`            | Opus モデルティアをリクエストする Agent 呼び出し   |
| `Agent(isolation:worktree)`    | git worktree をリクエストする Agent 呼び出し |
| `Bash(run_in_background:true)` | バックグラウンドで実行される Bash 呼び出し         |

パラメータマッチングは以下のルールに従います。

* パラメータ名は Agent ツール上の `model` など、ツールの入力の直接フィールドである必要があります。オブジェクトまたは配列内にネストされたフィールドはマッチ可能ではありません
* 各ルールは 1 つのパラメータに名前を付けます。`model` と `isolation` の両方でゲートするには、1 つのルールで組み合わせるのではなく、`Agent(model:opus)` と `Agent(isolation:worktree)` の 2 つのルールを記述します
* 値は `*` をワイルドカードとしてサポートし、任意の文字シーケンスにマッチするため、`Agent(isolation:*)` は任意の明示的な isolation 値にマッチします。`*` がない場合、マッチは正確です
* モデルが省略するパラメータは決してマッチしないため、`Agent(model:*)` は `model` が設定されていない呼び出しにはマッチしません
* 値は Claude が送信するリテラル入力と比較され、正規化の前です。`Agent(model:opus)` は別名 `opus` にマッチしますが、完全なモデル ID にはマッチしません。[`--verbose`](/docs/ja/cli-reference) で実行して、各ツール呼び出しの正確なパラメータ名と値を確認してください
* コロンの周りのホワイトスペースは無視されます

ツールのプライマリコンテンツフィールドはこの方法ではマッチ可能ではありません。Bash と PowerShell の `command`、Read、Edit、Write の `file_path`、Grep と Glob の `path`、NotebookEdit の `notebook_path`、WebFetch の `url` です。`Bash(command:rm *)` のようなルールはコンパウンドコマンドでバイパス可能であるため、Claude Code はそれを無視し、スタートアップ警告を発行します。代わりに `Bash(rm *)`、`Read(./path)`、または `WebFetch(domain:host)` を使用してください。

<h3 id="wildcard-patterns">
  ワイルドカードパターン
</h3>

Bash ルールの `*` は、スペースを含む任意のテキストにマッチするため、1 つのルールはコマンドのファミリーをカバーします。`*` のないルールは 1 つの正確なコマンドにマッチします。

<Warning>
  `*` をサブコマンドの後に配置します。`git log --oneline main` では、`git` はプログラムで `log` はサブコマンドです。サブコマンドはプログラムが何をするかを決定する単語です。Claude Code は最初の `*` の前のすべてをそのまま書かれたようにマッチさせるため、これらの単語がルールを制限するものです。`Bash(git log *)` は `git log` コマンドのみを許可し、`Bash(git *)` はすべての git コマンドを許可します。Claude Code は、`Bash(git * main)` のようなサブコマンドの前に `*` を持つ許可ルールについて [スタートアップで警告](/docs/ja/errors#has-a-wildcard-before-the-rest-of-the-command) します。
</Warning>

Claude が質問なしで実行するコマンドを記述し、変わる部分を `*` に置き換えます。この設定により、Claude Code は npm スクリプトと git コミットを質問なしで実行し、`git push` で始まるコマンドを拒否します。`git -C . push` のように別の書き方をした push はマッチしません。[Bash ルールがマッチしないもの](#bash-rule-limits)を参照してください。

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

`*` はルール内の任意の場所に配置できます。開始時、中間、または終了時です。各行はルール、マッチするコマンド、マッチしない近くのコマンドを示します。

| 記述内容                   | マッチ                                                                                | マッチしない                                |
| :--------------------- | :--------------------------------------------------------------------------------- | :------------------------------------ |
| `Bash(npm run build)`  | `npm run build`                                                                    | `npm run build --watch`               |
| `Bash(npm run *)`      | `npm run build`、`npm run test --watch`、`npm run`                                   | `npm install`                         |
| `Bash(git log * main)` | `git log --oneline main`、`git log -5 main`、`git log --output=<file> main`          | `git log main`、`git push origin main` |
| `Bash(git * main)`     | `git merge main`、`git push origin main`、`git -c core.fsmonitor=<script> diff main` | `git log`                             |
| `Bash(* --version)`    | `node --version`、`bash -c 'echo hi' --version`                                     | `node -v`                             |
| `Bash(ls *)`           | `ls -la`、`ls`                                                                      | `lsof`                                |
| `Bash(ls*)`            | `ls -la`、`lsof`                                                                    |                                       |
| `Bash(* --help *)`     | `npm --help x`                                                                     | `npm --help`                          |

3 つのマッチングルールがこれらの行を生成します。

* **`*` はその場所にあるテキストの代わりになります。** `Bash(git * main)` では、サブコマンドの代わりになるため、Claude Code はすべての git サブコマンドとその前のすべてのオプションにマッチします。これには `-c` が含まれます。これは git に名前を付けたプログラムを実行させます。`Bash(* --version)` では、`*` はプログラムの代わりになるため、任意のプログラムがマッチします。
* **末尾の `*` とその前のスペースは、ベアコマンドにもマッチします。** `Bash(ls *)` は `ls` にマッチし、`Bash(git log *)` は `git log` にマッチします。これは末尾の `*` がルールの唯一のワイルドカードである場合にのみ成立します。`Bash(* --help *)` は `npm --help x` にマッチしますが `npm --help` にはマッチしません。
* **末尾の `*` の前のスペースはルールの一部です。** `Bash(ls *)` は `ls` の後にスペースが必要であるため、`lsof` はマッチしません。`Bash(ls*)` にはスペースがないため、`lsof` にもマッチします。

`:*` サフィックスは末尾のワイルドカードを記述する同等の方法であるため、`Bash(ls:*)` は `Bash(ls *)` と同じコマンドをマッチさせます。

権限ダイアログは、コマンドプレフィックスに対して「はい、今後は聞かない」を選択すると、スペース区切り形式を書き込みます。`:*` 形式はパターンの末尾でのみ認識されます。`Bash(git:* push)` のようなパターンでは、コロンはリテラル文字として扱われ、git コマンドにはマッチしません。

<h3 id="tool-name-wildcards">
  ツール名ワイルドカード
</h3>

拒否ルールと確認ルールは、ツール名の位置でもグロブパターンを受け入れます。パターンはツール名全体にマッチする必要があります。`"*"` はすべてのツールにマッチし、`"mcp__*"` はすべてのサーバー全体のすべての MCP ツールにマッチします。ベアネーム glob 拒否ルールでマッチしたツールは Claude のコンテキストから削除されます。これはベアツール名と同じです。[`EndConversation`](/docs/ja/tools-reference#endconversation-tool-behavior) 例外を含みます。glob 拒否は他のツールが残っている間はそれを削除できず、glob 確認はそれに対してプロンプトを表示しません。この設定はすべての MCP ツールを拒否します。

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

許可ルールは、リテラル `mcp__<server>__` プレフィックスの後でのみツール名 glob を受け入れます。サーバーセグメントは glob フリーである必要があるため、ルールは設定した特定のサーバーに名前を付けます。`mcp__puppeteer__*` は `puppeteer` サーバーからのすべてのツールにマッチし、`mcp__github__get_*` はその `get_` ツールにマッチします。`"*"`、`"B*"`、または `"mcp__*"` などのアンカーなし許可 glob は警告とともにスキップされ、自動承認されません。

ツール名がマッチしない既知のツールを持つ拒否ルールまたは確認ルールは、タイプミスをキャッチするためにスタートアップ警告を生成します。`_` または `*` を含むツール名はチェックから除外されます。また、Claude Code が削除したツール（`TaskOutput` など）の名前も除外されます。

トランスクリプトと権限ダイアログに表示されるツールのラベルは、その正規名と異なる場合があります。たとえば、トランスクリプトで `Stop Task` というラベルが付いているツールの正規名は `TaskStop` です。権限ルールと [hook マッチャー](/docs/ja/hooks) はラベルにはマッチしないため、`Stop Task` として記述されたルールはマッチしません。拒否ルールと確認ルールの場合、上記のスタートアップ警告がミスマッチをキャッチします。[ツール参照](/docs/ja/tools-reference) に記載されている正規名を使用してください。

<h2 id="tool-specific-permission-rules">
  ツール固有の権限ルール
</h2>

<h3 id="bash">
  Bash
</h3>

Bash 権限ルールはコマンド全体をマッチさせ、`*` は任意のテキストを表します。[ワイルドカードパターン](#wildcard-patterns)は各ルール形状がマッチするコマンドと `*` の配置場所を示しています。このセクションの残りは、Claude Code が複合コマンドとラッパーをどのようにマッチさせるか、ルールがマッチしないもの、読み取り専用コマンド、およびリダイレクションについて説明しています。

<h4 id="compound-commands">
  複合コマンド
</h4>

<Tip>
  Claude Code はシェルオペレータを認識しているため、`Bash(safe-cmd *)` のようなルールは、`safe-cmd && other-cmd` コマンドを実行する権限を与えません。認識されるコマンド区切り文字は `&&`、`||`、`;`、`|`、`|&`、`&`、および改行です。ルールは各サブコマンドを独立して個別にマッチさせる必要があります。
</Tip>

Deny ルールと ask ルールは、サブシェル内のネストされたコマンド、コマンド置換、または `for` ループなどの制御フロー本体内のコマンドを含む、マッチするサブコマンドが存在する場合に適用されます。`Bash(git clean *)` のような ask ルールは、[auto モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)でも、`cd /tmp && git clean -f` または `echo "$(git clean -f)"` に対してプロンプトを表示します。

`&&` または `||` の後に何もない場合（`npm test &&` など）、Claude Code はコマンドを解析不可能として扱い、allow ルールマッチングのためにサブコマンドに分割しないため、`Bash(npm *)` のようなルールはそれを承認しません。

「はい、今後は聞かない」で複合コマンドを承認すると、Claude Code は複合文字列全体の単一ルールではなく、承認が必要な各サブコマンドの個別ルールを保存します。たとえば、`git status && npm test` を承認すると、`npm test` のルールが保存されるため、将来の `npm test` 呼び出しは `&&` の前に何があるかに関係なく認識されます。`cd` をサブディレクトリに移動するようなサブコマンドは、そのパスの独自の Read ルールを生成します。単一の複合コマンドに対して最大 5 つのルールが保存される場合があります。

<h4 id="process-wrappers">
  ラッパー
</h4>

Bash ルールをマッチさせる前に、Claude Code は固定されたラッパーセットをストリップするため、`Bash(npm test *)` のようなルールは `timeout 30 npm test` もマッチさせます。ストリップされるラッパーは `timeout`、`time`、`nice`、`nohup`、`stdbuf`、およびシェルビルトイン `command` と `builtin`、および zsh の `noglob` です。各ラッパーは引数を実際のコマンドとして実行します。ストリップされない関連する 2 つの形式があります。コマンドを検索する `command -v` クエリ形式と、zsh の `nocorrect` です。

Claude Code はまた、既知の安全な環境変数の先頭の割り当てをストリップするため、`Bash(npm test *)` は `NODE_ENV=test npm test` にマッチします。Allow ルールは他の変数の割り当てを超えてマッチしません。Deny ルールまたは ask ルールは任意の先頭の割り当てを超えてマッチするため、`Bash(rm *)` を deny で使用すると、`FOO=bar rm -rf tmp/` にマッチします。

ベア `xargs` もストリップされるため、`Bash(grep *)` は `xargs grep pattern` にマッチします。ストリップは `xargs` にフラグがない場合にのみ適用されます。`xargs -n1 grep pattern` のような呼び出しは `xargs` コマンドとしてマッチされるため、内部コマンド用に記述されたルールはそれをカバーしません。

このラッパーリストは組み込まれており、設定不可能です。`direnv exec`、`devbox run`、`mise exec`、`npx`、`docker exec` などの開発環境ランナーはリストに含まれていません。これらのツールは引数をコマンドとして実行するため、`Bash(devbox run *)` のようなルールは `run` の後に続くものをマッチさせます。これには `devbox run rm -rf .` が含まれます。環境ランナー内での作業を承認するには、ランナーと内部コマンドの両方を含む特定のルールを記述します。例えば `Bash(devbox run npm test)`。許可する内部コマンドごとに 1 つのルールを追加します。

`watch`、`setsid`、`ionice`、`flock` などの Exec ラッパーは、`Bash(watch *)` のようなプレフィックスルールで自動承認することはできず、Manual モードでは常にプロンプトを表示します。同じことが `-exec` または `-delete` を使用する `find` にも適用されます。`Bash(find *)` ルールはこれらの形式をカバーしません。特定の呼び出しを承認するには、完全なコマンド文字列の正確一致ルールを記述します。

<h4 id="bash-rule-limits">
  Bash ルールがマッチしないもの
</h4>

Bash ルールは Claude が記述したコマンドテキストにマッチします。Claude Code が[複合コマンド](#compound-commands)を分割し、[ラッパー](#process-wrappers)をストリップした後です。同じプログラムを別の形式で呼び出した場合、マッチしません。そのため、deny または ask ルールは Claude が通常生成する呼び出しをカバーし、プログラムの周りのセキュリティ境界ではありません。`deny` または `ask` のこれらのルールは最初の形式を停止し、他の形式は停止しません。

| ルール                | 停止                         | 停止しない                                                                                               |
| :----------------- | :------------------------- | :-------------------------------------------------------------------------------------------------- |
| `Bash(curl *)`     | `curl https://example.com` | `/usr/bin/curl https://example.com`、`sh -c 'curl https://example.com'`                              |
| `Bash(rm *)`       | `rm -rf build/`            | `/bin/rm -rf build/`、`bash -c 'rm -rf build/'`                                                      |
| `Bash(git push *)` | `git push origin main`     | `git -C . push origin main`、`git -c push.default=current push origin main`、`git 'push' origin main` |

最後の列のコマンドは、他のルールと権限モードによって決定されます。

コマンドテキストに依存しないファイルシステムとネットワークの強制には、[サンドボックス](/docs/ja/sandboxing)を使用してください。実行前に独自のロジックで完全なコマンドテキストを検査するには、[PreToolUse フック](#extend-permissions-with-hooks)を使用してください。

<h4 id="read-only-commands">
  読み取り専用コマンド
</h4>

Claude Code は、Bash コマンドの組み込みセットを読み取り専用として認識し、[`permissions.blockReadsOutsideWorkingDirectories`](/docs/ja/settings-reference#permissions-blockreadsoutsideworkingdirectories)がフェンスするパスを除き、すべてのモードで権限プロンプトなしで実行します。セットには `ls`、`cat`、`echo`、`pwd`、`head`、`tail`、`grep`、`find`、`wc`、`which`、`diff`、`stat`、`du`、`cd`、および `git` の読み取り専用形式が含まれます。セットは設定不可能です。これらのコマンドの 1 つにプロンプトを要求するには、それに対して `ask` または `deny` ルールを追加します。auto モードでは、これらのコマンドは分類器のレビューを待つこともできます。[分類器がアクションを評価する方法](/docs/ja/permission-modes#how-the-classifier-evaluates-actions)を参照してください。

`ls > out.txt` などのリダイレクトはターゲットのチェックを追加します。[リダイレクション](#redirections)を参照してください。

すべてのフラグが読み取り専用であるコマンドに対しては、引用符なしのグロブパターンが許可されるため、`ls *.ts` および `wc -l src/*.py` はプロンプトなしで実行されます。

Manual モードでは、このセットのコマンドは以下の場合にもプロンプトを表示します。

* **書き込み可能なフラグを持つコマンドの引用符なしグロブ**：`find`、`sort`、`sed`、`git` などの書き込み可能または実行可能なフラグを持つコマンドは、グロブが `-delete` のようなフラグに展開される可能性があるため、引用符なしのグロブが存在する場合にプロンプトを表示します。
* **別のデーモンを指す `docker`**：読み取り専用形式の `docker` は、`-H`、`--context`、または Podman の `--url` と `--connection` などの異なるデーモンを選択するフラグを持つコマンドでプロンプトを表示します。
* **パス開放フラグを持つ `file`**：`file` は `-m`/`--magic-file` または `-f`/`--files-from` を渡す場合にプロンプトを表示します。これらのフラグにより、`file` はフラグの値で指定されたパスを開くためです。
* **Windows 上のネットワークパス**：`\\server\share\file` などのネットワーク（UNC）パスを含む引数を持つコマンドは、ネットワークパスへのアクセスが Windows 認証情報をそれが指すホストに送信する可能性があるため、プロンプトを表示します。同じチェックが [PowerShell ツール](/docs/ja/tools-reference#powershell-tool)コマンドに適用されます。
* **解析できないコマンド**：Claude Code がコマンドを完全に解析できない場合、読み取り専用として扱う代わりに承認を求めます。10,000 文字を超えるコマンドは、解析が超過するため常にプロンプトを表示します。

作業ディレクトリまたは[追加ディレクトリ](#working-directories)内のパスへの `cd` も読み取り専用です。`cd packages/api && ls` のような複合コマンドは、各部分が独立して適格である場合、プロンプトなしで実行されます。これらの組み合わせは、各部分が読み取り専用であっても、プロンプトを表示します。

* **`cd` と `git`**：`cd` が異なるディレクトリに変更される場合にプロンプトを表示します。新しいディレクトリで `git` を実行するとそのディレクトリのフックを実行できるためです。ターゲットが現在の作業ディレクトリに解決される `cd` は、操作なしであり、プロンプトをトリガーしません。
* **`cd` とリダイレクト**：Claude Code が `cd` 実行後にリダイレクトターゲットが解決するディレクトリを判定できない場合にプロンプトを表示します。唯一のリダイレクトターゲットが `/dev/null` であるコマンド（`cd app; grep -r pattern . 2>/dev/null` など）は、`/dev/null` は作業ディレクトリに依存しないため、プロンプトを表示しません。

<Warning>
  コマンド引数を制約しようとする Bash 権限パターンは脆弱です。たとえば、`Bash(curl http://github.com/ *)` は curl を GitHub URL に制限することを意図していますが、次のようなバリエーションにはマッチしません。

  * URL の前のオプション：`curl -X GET http://github.com/...`
  * 異なるプロトコル：`curl https://github.com/...`
  * リダイレクト：`curl -L http://short.example.com/xyz`（GitHub にリダイレクト）
  * 変数：`URL=http://github.com && curl $URL`

  より信頼性の高い URL フィルタリングについては、以下を検討してください。

  * **Bash ネットワークツールを制限する**：deny ルールを使用して `curl`、`wget` などのコマンドをブロックし、許可されたドメインに対して `WebFetch(domain:github.com)` 権限で WebFetch ツールを使用します。deny ルールは、同じプログラムをパスで呼び出した場合や `sh -c` 内で呼び出した場合にはマッチしないため、制限を確実に保持する必要がある場合は[サンドボックスネットワーク許可リスト](/docs/ja/sandboxing#network-isolation)と組み合わせてください。[Bash ルールがマッチしないもの](#bash-rule-limits)を参照してください。
  * **PreToolUse フックを使用する**：Bash コマンドの URL を検証し、許可されていないドメインをブロックするフックを実装します。
  * **CLAUDE.md ガイダンスを追加する**：`CLAUDE.md` で許可された curl パターンについて説明します。これは Claude が試みることを形作りますが、境界を強制しないため、上記のオプションの 1 つと組み合わせてください。

  WebFetch のみを使用しても、ネットワークアクセスは防止されません。Bash が許可されている場合、Claude は `curl`、`wget` または他のツールを使用して任意の URL に到達できます。
</Warning>

<h4 id="redirections">
  リダイレクション
</h4>

コマンドが出力または入力をリダイレクトする場合、Claude Code はリダイレクトターゲットをファイルルールに対してチェックします。Claude が直接そのファイルを書き込みまたは読み取ったかのようにです。

* **出力リダイレクト**：`> file`、`>> file`、または `2> file` の場合、チェックは `Edit` allow ルールと deny ルール、[保護されたパス](/docs/ja/permission-modes#protected-paths)、および[作業ディレクトリ](#working-directories)をカバーします。`Bash(git commit *)` のようなルールはコマンドを許可し、ターゲットは許可しません。`~` で始まるターゲットまたはグロブ文字を含むターゲットは承認が必要です。
* **入力リダイレクト**：`< file` の場合、チェックは `Read` allow ルールと deny ルール、および作業ディレクトリをカバーします。作業ディレクトリの外のターゲットは、allow ルールがカバーしない限り承認が必要です。グロブパターンを含むターゲット、または同じコマンド内の `cd` に続く相対パスは、allow ルールがカバーしている場合でも承認が必要です。Claude Code は v2.1.257 以降で入力ターゲットをチェックします。

ターゲットの背後にファイルがない場合はチェックされません。`/dev/null`、`2>&1` や `<&3` などのファイルディスクリプタ形式、および here-docs と here-strings です。

Claude Code はまた、`tee` コマンドが書き込むファイルをチェックします。これには `make | tee build.log` などのパイプラインが含まれます。チェックは `Edit` allow ルールと deny ルール、[保護されたパス](/docs/ja/permission-modes#protected-paths)、および[作業ディレクトリ](#working-directories)をカバーします。`Bash(tee *)` のような allow ルールは、作業ディレクトリの外の宛先をカバーしません。Claude Code は v2.1.269 以降で `tee` ターゲットをチェックします。

<h3 id="powershell">
  PowerShell
</h3>

PowerShell 権限ルールは Bash ルールと同じ形状を使用しています。`*` を使用したワイルドカードは任意の位置でマッチし、`:*` サフィックスは末尾の ` *` と同等であり、ベア `PowerShell` または `PowerShell(*)` はすべてのコマンドをマッチさせます。この設定により、`Get-ChildItem` および `git commit` コマンドが許可され、`Remove-Item` がブロックされます。

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

一般的なエイリアスはマッチング前に正規化されます。コマンドレット名用に記述されたルールはそのエイリアスもマッチさせるため、`PowerShell(Get-ChildItem *)` は `gci`、`ls`、`dir` もマッチさせます。マッチングは大文字と小文字を区別しません。

Claude Code は PowerShell AST を解析し、複合コマンド内の各コマンドを独立してチェックします。パイプオペレータ `|`、ステートメント区切り文字 `;`、および PowerShell 7 以降のチェーンオペレータ `&&` と `||` は複合コマンドをサブコマンドに分割します。複合コマンドが許可されるには、ルールがすべてのサブコマンドをマッチさせる必要があります。

<h3 id="read-and-edit">
  Read と Edit
</h3>

Claude のファイルツールがファイルまたはディレクトリを読み取るのをブロックするには、`Read(./.env)` または `Read(./secrets/**)` などのパスに対して `Read` deny ルールを追加します。[機密ファイルを除外](/docs/ja/settings-reference#exclude-sensitive-files)にはペースト可能な例があります。

`Edit` ルールはファイルを編集するすべての組み込みツールに適用されます。Claude は、Grep や Glob などのファイルを読み取るすべての組み込みツール、プロンプト内の `@file` メンション、および接続された [IDE](/docs/ja/vs-code#the-built-in-ide-mcp-server) が Claude と共有する選択およびオープンファイルコンテキストに `Read` ルールを適用するためにベストエフォートを試みます。

`Read` deny ルールは、同じパスの [Edit と Write ツール](/docs/ja/errors#file-is-covered-by-a-read-deny-rule)もブロックします。これには新しいファイルの作成も含まれます。NotebookEdit はカバーされないため、ツールが変更できないパスに対して `Edit` deny ルールを追加してください。チェックには編集時に Claude Code v2.1.208 以降が必要です。書き込み時には v2.1.228 以降が必要です。

Claude Code は `Edit(path)` と `Read(path)` ルールに対してのみファイル権限をチェックします。代わりに `Write`、`NotebookEdit`、`Glob`、またはレガシー `MultiEdit` ツール用にパスルールを記述する場合、Claude Code はルールを受け入れますが、それを参照することはなく、[起動時に警告](/docs/ja/errors#is-not-matched-by-file-permission-checks)を表示します。ただし、`--allowedTools` で渡された `Glob` ルールは除きます。`Write(docs/**)`、`NotebookEdit(docs/**)`、または `MultiEdit(docs/**)` の代わりに `Edit(docs/**)` を使用し、`Glob(docs/**)` の代わりに `Read(docs/**)` を使用してください。Claude Code は `Write` の deny ルールなど、パスのないツール名ルールについては警告しません。それはどこでもツールレベルでそのルールをマッチさせます。v2.1.210 以降が必要です。

<Warning>
  Read と Edit deny ルールは Claude の組み込みファイルツール、`cat`、`head`、`tail`、`sed` などの Claude Code が認識する Bash ファイルコマンド、および `> file` と `< file` などの Bash [リダイレクション](#redirections)のターゲットに適用されます。これらは、そのファイルがあるディレクトリから実行する `grep -r pattern .` のような、ファイルを名前で指定せずに読み取るコマンドや、Python または Node スクリプトがファイルを自分で開くような、ファイルを間接的に読み書きする任意のサブプロセスには適用されません。パスへのすべてのプロセスのアクセスをブロックする OS レベルの強制については、[サンドボックスを有効にしてください](/docs/ja/sandboxing)。
</Warning>

Read と Edit ルールの両方は、[gitignore](https://git-scm.com/docs/gitignore)パターン構文を使用し、4 つの異なるパターンタイプがあります。単一セグメントディレクトリパターンの場合、マッチング深度はルールタイプにも依存し、このセクションの後半で説明されています。

| パターン                | 意味                 | 例                                | マッチ                                                 |
| ------------------- | ------------------ | -------------------------------- | --------------------------------------------------- |
| `//path`            | ファイルシステムルートからの絶対パス | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                           |
| `~/path`            | ホームディレクトリからのパス     | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                      |
| `/path`             | 設定ソースからの相対パス       | `Edit(/src/**/*.ts)`             | プロジェクト設定の `<primary working directory>/src/**/*.ts` |
| `path` または `./path` | 現在のディレクトリからの相対パス   | `Read(*.env)`                    | `<cwd>/*.env`                                       |

<Warning>
  `/Users/alice/file` のようなパターンは絶対パスではありません。単一の先頭スラッシュは設定ソースにアンカーされており、ファイルシステムルートではありません。絶対パスには `//Users/alice/file` を使用してください。
</Warning>

`/path` パターンは、それを定義する設定ソースに関連付けられたディレクトリにアンカーされるため、同じルールは配置場所に応じて異なる場所にマッチします。

| ルール定義場所                               | `/path` の解決先                       |
| :------------------------------------ | :--------------------------------- |
| `.claude/settings.json` のプロジェクト設定     | `<primary working directory>/path` |
| `.claude/settings.local.json` のローカル設定 | `<primary working directory>/path` |
| `~/.claude/settings.json` のユーザー設定     | `~/.claude/path`                   |
| `--settings <file>` で渡されたファイル         | `<directory of file>/path`         |
| CLI フラグまたはセッションルール                    | `<primary working directory>/path` |

`/permissions` を通じて追加するルールは、保存先の設定ファイルの行に従います。

ローカル設定ルールは、v2.1.211 以降で Claude Code が [ファイルを保存](#permission-system)するリポジトリルートではなく、セッションの [primary working directory](#working-directories) にアンカーされます。リポジトリルートで開始されたセッションでは、2 つのディレクトリは同じです。[worktree](/docs/ja/worktrees)セッションでは、`Edit(/src/**)` などの共有ルールはそのワークツリー独自の `src/` ディレクトリをマッチさせます。

`Read(/secrets/**)` のような deny ルールをユーザー設定で記述すると、プロジェクト内の `secrets` ディレクトリではなく、`~/.claude/secrets/**` をブロックします。すべてのプロジェクト内で適用されるルールをユーザー設定で記述するには、代わりに `//` 絶対パスまたは `~/` ホーム相対パスを使用してください。

Windows では、パスはマッチング前に POSIX 形式に正規化されます。`C:\Users\alice` は `/c/Users/alice` になるため、そのドライブ上の任意の場所の `.env` ファイルをマッチさせるには `//c/**/.env` を使用します。すべてのドライブにわたってマッチさせるには、`//**/.env` を使用します。

例：

* `Edit(/docs/**)`：`<primary working directory>/docs/` での編集。`/docs/` や `<primary working directory>/.claude/docs/` ではありません
* `Read(~/.zshrc)`：ホームディレクトリの `.zshrc` の読み取り
* `Edit(//tmp/scratch.txt)`：絶対パス `/tmp/scratch.txt` の編集
* `Read(src/**)`：allow ルールとしては `<current-directory>/src/` からの読み取りのみ。deny ルールまたは ask ルールとしては、現在のディレクトリの下の任意の深さにある `src` ディレクトリにマッチします

ルールはそのアンカーの下のファイルのみをマッチさせます。その範囲内で、マッチング深度はパターン形状に依存し、単一セグメントディレクトリパターンの場合、ルールタイプにも依存します。以下で説明されています。ベアファイル名は gitignore セマンティクスに従い、任意の深さでマッチするため、`Read(.env)` と `Read(**/.env)` は同等です。

| Deny ルール                         | ブロック                    | ブロックしない                       |
| -------------------------------- | ----------------------- | ----------------------------- |
| `Read(.env)` または `Read(**/.env)` | 現在のディレクトリ以下の任意の `.env`  | 親ディレクトリまたは別のプロジェクト内の `.env`   |
| `Read(//**/.env)`                | ファイルシステム上の任意の場所の `.env` | なし。ルールはファイルシステムルートにアンカーされています |

単一ディレクトリセグメント（`src/**` など）を持つ相対パターンは、ルールタイプに応じて異なる深さでマッチします。

* **Allow ルール**：`Edit(src/**)` は `<cwd>/src` とその下のファイルのみをマッチさせます。任意の深さでディレクトリ名を許可するには、`Edit(**/src/**)` を記述してください。
* **Deny と ask ルール**：`Read(secrets/**)` は現在のディレクトリの下の任意の深さで `secrets` という名前のディレクトリをマッチさせるため、ルールはネストされたコピーにも適用されます。

他のすべてのパターン形状は、すべてのルールタイプで同じ深さでマッチします。`Edit(/src/**)` と `Edit(src/components/**)` はそのアンカーされた場所でのみマッチし、`Edit(**/src/**)` は任意の深さでマッチします。

次の例は、トップレベルの `src/` ディレクトリと `vendor/` の下のネストされたコピーを持つプロジェクトに対する各パターン形状を示しています。

```text theme={null}
<current-directory>/
├── src/
│   └── app.ts
└── vendor/
    └── pkg/
        └── src/
            └── lib.js
```

| ルール                                 | `src/app.ts` をマッチ | `vendor/pkg/src/lib.js` をマッチ |
| :---------------------------------- | :---------------- | :--------------------------- |
| Allow ルールとしての `Edit(src/**)`        | はい                | いいえ                          |
| Deny または ask ルールとしての `Edit(src/**)` | はい                | はい                           |
| 任意のルールタイプの `Edit(/src/**)`          | はい                | いいえ                          |
| 任意のルールタイプの `Edit(**/src/**)`        | はい                | はい                           |

<Note>
  gitignore パターンでは、`*` は単一のパスセグメント内でマッチし、パターン内の任意の位置に表示できます。一方、`**` はディレクトリ全体でマッチします。
</Note>

「はい、今後は聞かない」でファイルパスを承認すると、Claude Code は `[`、`]`、`*` などの gitignore パターン文字をそのパスでエスケープするため、生成されたルールは承認したリテラルパスのみをマッチさせます。自分で記述したルールはエスケープされません。v2.1.202 より前では、Claude Code はパスをエスケープなしで保存していたため、`[2024-06] Reports` という名前のディレクトリの生成されたルールは、独自のパスをマッチさせできなかったり、意図しない兄弟ディレクトリをマッチさせたりする可能性がありました。

パスに括弧をエスケープする必要はないため、`Edit(./Finance (2024)/**)` は `Finance (2024)` フォルダをそのまま記述してマッチさせます。

パスが gitignore パターンとして使用可能でない deny または ask ルールは、その正確なパスを保護します。使用可能でないパターンを持つ allow ルールは何も承認しません。

deny または ask パターンが `!` で始まる場合、gitignore 否定です。それは、その前にリストされた `path` または `./path` ルールから、それがマッチするパスを削除します。1 つの設定ファイルの `deny` リストで、`Read(*.env)` の後に `Read(!sample.env)` が続く場合、名前が `.env` で終わるすべてのファイルを任意の深さでブロックしますが、`sample.env` という名前のファイルは除きます。最初にリストされた `!` ルールは何も削除しません。

削除は同じソースからのルールにのみ到達します。プロジェクト設定または `--disallowedTools` の `Read(!.env)` は、管理設定または他の設定ファイルからの `Read(./.env)` deny をキャンセルしません。

2 つの制限は、`!` パターンが削除できるものを狭めます。

* Claude Code は、`!` の後に `/`、`~/`、または `//` が続く場合でも、`!` パターンを現在のディレクトリから相対的に読み取るため、パターンはこれらのプレフィックスの 1 つでアンカーされたルールに到達できません。`Read(!~/notes/public/**)` は `Read(~/notes/**)` から何も削除しません。
* 削除は、ルールが全体としてブロックするディレクトリ内のファイルを再度開くことはできません。`Read(secrets/**)` と `Read(!secrets/public/**)` では、Claude Code は `secrets/public` をそれ以外の `secrets` と一緒にブロックします。

Claude がシンボリックリンクにアクセスするとき、権限ルールは 2 つのパスをチェックします。シンボリックリンク自体と、それが解決するファイルです。Allow ルールと deny ルールはそのペアを異なる方法で扱います。allow ルールはプロンプトにフォールバックし、deny ルールは完全にブロックします。

* **Allow ルール**：シンボリックリンクパスとそのターゲットの両方がマッチする場合にのみ適用されます。許可されたディレクトリ内のシンボリックリンクがそれの外を指している場合でも、プロンプトが表示されます。
* **Deny ルール**：シンボリックリンクパスまたはそのターゲットのいずれかがマッチする場合に適用されます。拒否されたファイルを指すシンボリックリンク自体が拒否されます。たとえば、`Read(./project/**)` が許可され、`Read(~/.ssh/**)` が拒否されている場合、`./project/key` にあるシンボリックリンクが `~/.ssh/id_rsa` を指している場合、ターゲットが allow ルールに失敗し、deny ルールにマッチするため、ブロックされます。

ツールが承認されたファイルを開くとき、Claude Code は [パスが権限チェックが承認した場所にまだ解決されることを確認](/docs/ja/errors#refusing-after-a-symlink-changed)します。

Grep と Glob は `path` 引数が解決するディレクトリを検索します。Claude Code はそのディレクトリに `Read` deny ルールを適用します。

macOS と Linux では、シンボリックリンク付きディレクトリを通じて記述された deny または ask ルール（`//`、`~/`、または `/` パターン）は、そのディレクトリの実際の場所にも適用されます。たとえば macOS では、`/etc` が `/private/etc` に解決される場合、`Read(//etc/**)` は `/private/etc/hosts` もブロックします。v2.1.268 より前では、シンボリックリンク付きディレクトリを通じて記述された deny または ask ルールは、その実際の場所で指定されたパスに適用されませんでした。

<h3 id="webfetch">
  WebFetch
</h3>

WebFetch ルールは `domain:` プレフィックスを使用し、リクエストされた URL のホスト名に対してマッチします。マッチングは大文字と小文字を区別せず、`*` ワイルドカードをサポートし、ルールとホスト名の両方から末尾の `.` をストリップするため、`example.com.` と `example.com` は同じものとして扱われます。

* `WebFetch(domain:example.com)` は `example.com` へのリクエストをマッチさせます
* `WebFetch(domain:*.example.com)` は `api.example.com` や `a.b.example.com` などの任意の深さのサブドメインをマッチさせますが、`example.com` 自体はマッチさせません
* `WebFetch(domain:*)` はすべてのドメインをマッチさせます。ベア `WebFetch` ルールと同じではありません。[すべてのフェッチを許可または拒否](#allow-or-deny-every-fetch)を参照してください

先頭の `*.` またはベア `*` 以外の任意の位置では、ワイルドカードは 2 つのドット間のテキストのみをマッチさせます。`WebFetch(domain:example.*)` は `example.org` にマッチします。ここで `*` は `org` になりますが、`example.evil.com` にはマッチしません。ここで `*` は `evil.com` になり、ドットを越えます。これにより、末尾のワイルドカードが攻撃者が登録できるドメインをマッチさせるのを防ぎます。

WebFetch ルールのワイルドカードは、フェッチをマッチさせるために Claude Code v2.1.172 以降が必要です。

<h4 id="allow-or-deny-every-fetch">
  すべてのフェッチを許可または拒否
</h4>

ベア `WebFetch` ルールは、`"deny": ["WebFetch"]` などの `domain:` 部分のないツール名です。それと `WebFetch(domain:*)` の両方はすべての URL をカバーしますが、Claude Code はそれらを異なる方法で適用し、`domain:` 形式のみがそのドメインをサンドボックスの [許可または拒否ドメインリスト](/docs/ja/sandboxing#network-isolation)に追加します。そのセクションはサンドボックスが尊重するワイルドカード形式とそれを追加したバージョンをリストしています。

各行は、`allow` リストと `deny` リストでルールが何をするかを示しています。

| ルール                  | `allow` で                                                | `deny` で                                                                                   |
| :------------------- | :------------------------------------------------------- | :----------------------------------------------------------------------------------------- |
| `WebFetch`           | Claude はプロンプトなしでフェッチします。サンドボックス化されたコマンドが到達できるホストを変更しません。 | Claude Code は `WebFetch` ツールを削除するため、Claude はまったくフェッチできません。サンドボックス化されたコマンドが到達できるホストを変更しません。 |
| `WebFetch(domain:*)` | Claude はプロンプトなしでフェッチし、サンドボックス化されたコマンドは任意のホストに到達できます。     | Claude Code はツールを保持し、各フェッチを拒否し、サンドボックス化されたコマンドはホストに到達できません。                                |

2 つの形式は [artifacts](/docs/ja/artifacts)（Artifact ツールが claude.ai に公開するページ）の読み取りについても異なります。ベア `WebFetch` deny または ask ルールはこれらの読み取りに適用されません。`claude.ai` または `*.claudeusercontent.com` コンテンツホストをカバーする `domain:` ルール（`WebFetch(domain:claude.ai)` または `WebFetch(domain:*)` など）は、各読み取りを拒否するか、その前にプロンプトを表示します。[`Artifact` ルール](/docs/ja/artifacts#disable-artifacts)も同じことを行います。

ルールが読み取りをブロックするとき、拒否はルールを指定します。v2.1.268 より前では、ベア `WebFetch` deny ルールはすべての artifact 読み取りをブロックし、ベア ask ルールはそれぞれの前にプロンプトを表示していました。

Claude がフェッチを自由に行えるようにしながら、サンドボックス許可リストをそのままにするには、ベア形式を使用してください。この `settings.json` はそれを行います。

```json theme={null}
{
  "permissions": {
    "allow": ["WebFetch"]
  }
}
```

Claude にページをフェッチするよう求めると、プロンプトなしでフェッチします。[サンドボックス化](/docs/ja/sandboxing)された `curl` をサンドボックス許可リストの外のホストに対して実行するよう求めると、Claude Code はそのホストに対してプロンプトを表示します。ベア形式はホストを許可リストに追加しなかったためです。

[auto モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)では、Claude は代わりにホストを分類器がレビューするコマンドの [per-command allowed domains](/docs/ja/sandboxing#per-command-allowed-domains-in-auto-mode) に指定します。

<h3 id="mcp">
  MCP
</h3>

MCP ルールは Claude Code で設定されたサーバー名を使用し、オプションでそのサーバーからのツールの名前が続きます。

* `mcp__puppeteer` は `puppeteer` サーバーによって提供されるツールをマッチさせます
* `mcp__puppeteer__*` ワイルドカード構文を使用し、`puppeteer` サーバーからのすべてのツールもマッチさせます
* `mcp__puppeteer__puppeteer_navigate` は `puppeteer` サーバーによって提供される `puppeteer_navigate` ツールをマッチさせます

組織が [claude.ai コネクタ](/docs/ja/mcp#organization-controls-on-connector-tools)ツールを `ask` に設定している場合、そのツールの allow ルールは有効になりません。Claude Code は `auto` および `bypassPermissions` モードでも、すべての呼び出しでプロンプトを表示します。プロンプトを表示しない `dontAsk` モードでは、Claude Code は代わりに呼び出しを拒否します。Claude Code が自分で取得するコネクタからのツールは `mcp__claude_ai_<server>__<tool>` として表示されます。

Claude Desktop アプリの [Cowork](https://claude.com/docs/cowork/overview)セッションでは、Claude は組み込み `Bash` ツールではなく Cowork の `mcp__workspace__bash` ツールを通じてシェルコマンドを実行し、Cowork は同様に `mcp__workspace__web_fetch` をウェブフェッチに提供します。Claude Code はまた、全体の `Bash` または `WebFetch` ツールを指す deny ルールをこれらの Cowork ツールに適用するため、管理された `Bash` deny ルールは Claude が Cowork でシェルコマンドを実行するのを停止します。Claude Code がそのような呼び出しをブロックするとき、メッセージは Cowork ツールを指定します。`Permission to use mcp__workspace__bash has been denied.` Allow ルールは引き継がれません。Claude Code は `Bash` allow ルールを `mcp__workspace__bash` に適用することはありません。

<h3 id="agent-subagents">
  Agent（subagents）
</h3>

`Agent(AgentName)` ルールを使用して、Claude が使用できる [subagents](/docs/ja/sub-agents) を制御します。

* `Agent(Explore)` は Explore subagent をマッチさせます
* `Agent(Plan)` は Plan subagent をマッチさせます
* `Agent(my-custom-agent)` は `my-custom-agent` という名前のカスタム subagent をマッチさせます

これらのルールを設定の `deny` 配列に追加するか、`--disallowedTools` CLI フラグを使用して特定のエージェントを無効にします。Explore エージェントを無効にするには：

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

`Cd` ルールは、[`/cd` コマンド](/docs/ja/commands)がセッションを移動できるディレクトリを制御します。`Cd` はモデル呼び出し可能なツールではありません。Claude はそれを呼び出すことはできず、ルールは自分で `/cd` を実行する場合にのみ適用されます。

ベア `Cd` deny ルールは `/cd` を完全に無効にします。`Cd(<path-pattern>)` deny ルールはマッチするターゲットをブロックします。Deny ルールはターゲットのすべてのスペルをチェックします。これには、それが解決する各シンボリックリンクホップが含まれるため、1 つのパス用に記述されたルールは、それに解決するターゲットもブロックします。

任意の `Cd` allow ルールを追加すると、`/cd` をホワイトリストモードに切り替えます。解決されたターゲットディレクトリは、allow ルールの 1 つにマッチする必要があります。そうでない場合、`/cd` は拒否します。`Cd` ルールが設定されていない場合、`/cd` はデフォルト動作を保持し、見慣れないディレクトリを信頼するようにプロンプトを表示します。

パスパターンは [Read と Edit ルール](#read-and-edit)から `//`、`~/`、`/` アンカーを共有しますが、マッチングはディレクトリパス全体にアンカーされます。gitignore スタイルではなく、`*` は正確に 1 つのパスセグメントをマッチさせ、`**` はセグメント全体でマッチさせます。末尾の `/**` はその名前付きルートもマッチさせます。

| ルール                   | マッチ                                           | マッチしない                    |
| --------------------- | --------------------------------------------- | ------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                  | `~/code/app/src`、`~/code` |
| `Cd(~/code/**)`       | `~/code` およびその下のディレクトリ                        | `~/code` の外のディレクトリ        |
| `Cd(**/node_modules)` | 現在のディレクトリ配下の任意の深さにある任意の `node_modules` ディレクトリ | `node_modules/pkg`        |

<h2 id="extend-permissions-with-hooks">
  フックで権限を拡張する
</h2>

[Claude Code フック](/docs/ja/hooks-guide)は、実行時に権限評価を実行するカスタムシェルコマンドを登録する方法を提供します。Claude Code がツール呼び出しを行うと、PreToolUse フックは権限プロンプトの前に実行されます。ただし、[`EndConversation`](/docs/ja/tools-reference#endconversation-tool-behavior)を除くすべてのツールに対して実行されます。フック出力はツール呼び出しを拒否し、プロンプトを強制し、またはプロンプトをスキップしてコールを続行させることができます。

フック決定は権限ルールをバイパスしません。Claude Code は deny ルールと ask ルールを、フックが何を返すかに関係なく評価します。マッチする deny ルールはコールをブロックし、マッチする ask ルールはフックが `"allow"` または `"ask"` を返した場合でもプロンプトを表示します。これは、[権限を管理する](#manage-permissions)で説明されている deny 優先の優先順位を保持し、管理設定で設定された deny ルールを含みます。

[`requiresUserInteraction`](/docs/ja/mcp#require-approval-for-a-specific-tool)でマークされた MCP ツールも、フックが `"allow"` を返した場合でもプロンプトを表示します。コネクタツール（[組織が `ask` に設定](/docs/ja/mcp#organization-controls-on-connector-tools)）も同様に、その設定が Claude Code に到達するセッションではプロンプトを表示します。

ブロッキングフックは allow ルールよりも優先されます。終了コード 2 で終了するフックは、権限ルールが評価される前にツール呼び出しを停止するため、allow ルールがコールを許可する場合でもブロックが適用されます。プロンプトなしですべての Bash コマンドを実行し、ブロックしたい少数のコマンドを除外するには、allow リストに `"Bash"` を追加し、それらの特定のコマンドを拒否する PreToolUse フックを登録します。適応できるフックスクリプトについては、[保護されたファイルへの編集をブロックする](/docs/ja/hooks-guide#block-edits-to-protected-files)を参照してください。

<h2 id="working-directories">
  作業ディレクトリ
</h2>

デフォルトでは、Claude は起動されたディレクトリ内のファイルにアクセスできます。そのディレクトリはセッションの主要な作業ディレクトリであり、[`/cd` でセッションを移動](#move-the-session-to-another-directory)するまで変わりません。このアクセスを拡張できます。

* **起動時**：`--add-dir <path>` CLI 引数を使用します
* **セッション中**：`/add-dir` コマンドを使用します
* **永続的な設定**：[設定ファイル](/docs/ja/settings#where-settings-live)の `additionalDirectories` に追加します

追加ディレクトリ内のファイルは、元の作業ディレクトリと同じ権限ルールに従います。プロンプトなしで読み取り可能になり、ファイル編集権限は現在の権限モードに従います。

ほとんどの[ネットワークパス](/docs/ja/errors#working-directory-is-a-network-path)（`\\server\share` などの UNC 共有など）は、作業ディレクトリとして追加できません。これは、ルックアップがそれが指す名前のホストに接続する可能性があるためです。Windows では、代わりに共有をドライブ文字にマップし、起動時に `--add-dir` でドライブを渡します。

[`permissions.blockReadsOutsideWorkingDirectories`](/docs/ja/settings-reference#permissions-blockreadsoutsideworkingdirectories) を設定して、ファイルツールがすべての権限モードで囲まれたパスを拒否するようにします。自動モードでは、Claude Code は Claude が[作業ディレクトリの外側を読み取る](/docs/ja/permission-modes#first-read-outside-the-working-directories)最初の時点でこれをオンにするよう提案します。

macOS のバックグラウンドセッションでは、セッションホストは `~/Desktop`、`~/Documents`、`~/Downloads` などの保護されたフォルダへのアクセスをターミナルとは別に要求します。Claude がそこでファイルを読み取りまたは書き込む必要がある場合、`Operation not permitted` で読み取りが失敗する場合は、[バックグラウンドセッションにフォルダアクセスを許可する方法](/docs/ja/agent-view#background-sessions-can%E2%80%99t-read-desktop-documents-or-downloads-on-macos)を参照してください。

<h3 id="move-the-session-to-another-directory">
  セッションを別のディレクトリに移動する
</h3>

セッションを別の主要な作業ディレクトリに移動するには、現在のディレクトリの横に[ディレクトリを追加](#working-directories)するのではなく、`/cd <path>` を実行します。Claude Code は会話を保持し、新しいディレクトリの `CLAUDE.md` を読み込み、以前にそこで作業していない場合は[ワークスペースを信頼](#project-allow-rules-and-workspace-trust)するよう促します。その後、新しいディレクトリから `--resume` を実行すると、Claude Code は[移動されたセッションを検出](/docs/ja/sessions#resume-a-session)します。

移動するとすぐに、Claude Code は新しいディレクトリのプロジェクト設定を適用します。

* プロジェクト設定（権限ルールと[hooks](/docs/ja/hooks)を含む）
* [`.mcp.json` サーバー](/docs/ja/mcp#project-scope)（起動時と同じ[サーバー承認](/docs/ja/mcp#project-server-approvals-and-workspace-trust)の対象であり、それに登録した[ローカルスコープ](/docs/ja/mcp#local-scope) MCP サーバー）
* 設定が有効にする[プラグイン](/docs/ja/plugins/overview)、その[スキル](/docs/ja/skills#discovery-from-parent-and-nested-directories)、およびその[サブエージェント](/docs/ja/sub-agents)
* [`env`](/docs/ja/settings-reference#env) 値（前のディレクトリの設定から適用された環境変数の上に適用され、有効なままです）

Claude Code はまた、前のディレクトリのプロジェクトと[ローカルスコープ](/docs/ja/mcp#local-scope) MCP サーバーを切断し、移動後に有効でなくなった[プラグイン](/docs/ja/mcp#plugin-provided-mcp-servers)のサーバーを切断します。前のディレクトリの設定ではなく、新しいディレクトリの設定から[追加ディレクトリ](#working-directories)を取得し、`--add-dir` または `/add-dir` で追加したディレクトリを保持します。移動が有効にする Hooks は、セッションが開始されたプロジェクトルートに設定された[`${CLAUDE_PROJECT_DIR}`](/docs/ja/hooks#reference-scripts-by-path)を受け取ります。

新しいディレクトリがまだ信頼されていない場合、Claude Code は信頼プロンプトでディレクトリの設定が有効にするアロールール、追加ディレクトリ、hooks、およびヘルパーコマンドをリストアップするため、受け入れる前に確認できます。拒否した場合、セッションはそのままです。v2.1.246 より前では、`/cd` は再開するまで新しいディレクトリの設定、hooks、MCP サーバー、またはスキルを適用しませんでした。また、信頼プロンプトはディレクトリの設定が何を有効にするかをリストアップしませんでした。

[`Cd` 権限ルール](#cd)で `/cd` ターゲットを制限または無効にします。

<h3 id="additional-directories-grant-file-access-not-configuration">
  追加ディレクトリはファイルアクセスを許可し、設定ではありません
</h3>

ディレクトリを追加すると、Claude がファイルを読み取りおよび編集できる場所が拡張されます。そのディレクトリを完全な設定ルートにはしません。ほとんどの `.claude/` 設定は追加ディレクトリから検出されませんが、いくつかのタイプは例外として読み込まれます。

これらの例外は、`--add-dir` フラグまたは `/add-dir` コマンドで追加されたディレクトリにのみ適用されます（Agent SDK がフラグを通じて追加するディレクトリを含む）。設定ファイルの `permissions.additionalDirectories` にリストされているディレクトリは、ファイルアクセスのみを許可し、以下の設定は読み込みません。

Agent SDK の TypeScript の[`additionalDirectories`](/docs/ja/agent-sdk/typescript#options)オプションと Python の[`add_dirs`](/docs/ja/agent-sdk/python#claudeagentoptions)オプションは、TypeScript オプションが設定キーと同じ名前を共有していても、例外を受け取ります。SDK は各エントリを Claude Code に `--add-dir` として渡すため、これらのディレクトリはフラグで追加されたディレクトリのように動作します。任意のフラグで追加されたディレクトリからのスキル、コマンド、およびサブエージェントは、プロジェクト[設定ソース](/docs/ja/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)を通じて読み込まれるため、CLI で[`--setting-sources`](/docs/ja/cli-reference)を使用するか SDK で `settingSources` を使用してそのソースを除外した場合は読み込まれず、[ベアモード](/docs/ja/headless#start-faster-with-bare-mode)はそれらの中のコマンドとサブエージェントをスキップします。

次の設定タイプは `--add-dir` ディレクトリから読み込まれます。

| 設定                                                                            | `--add-dir` から読み込まれます                                                                                                      |
| :---------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| `.claude/skills/` の[スキル](/docs/ja/skills)                                          | はい、ライブリロード付き                                                                                                               |
| `.claude/commands/` の[コマンドファイル](/docs/ja/skills#where-skills-live)                 | はい、ライブリロードなし。追加されたディレクトリとプロジェクトの両方が同じ名前のコマンドを定義する場合、Claude Code はプロジェクトのコマンドを実行します                                         |
| `.claude/agents/` の[サブエージェント](/docs/ja/sub-agents)                                 | はい、ライブリロードなし                                                                                                               |
| `.claude/settings.json` および `.claude/settings.local.json` の[設定](/docs/ja/settings) | `enabledPlugins` および[`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces)キーのみ                          |
| [CLAUDE.md](/docs/ja/memory)ファイル、`.claude/rules/`、および `CLAUDE.local.md`            | `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` が設定されている場合のみ。`CLAUDE.local.md` はさらに `local` 設定ソースが必要です。これはデフォルトで有効になっています |

[主要な作業ディレクトリ](#working-directories)のサブディレクトリからスキル、コマンド、およびサブエージェントをセッション中に読み込むには、そのサブディレクトリのパスで `/add-dir` を実行します。Claude Code はサブディレクトリが既に読み取り可能であるため、プロンプトを表示したり作業ディレクトリを追加したりせずに、セッションの残りの間それらを読み込みます。これには Claude Code v2.1.257 以降が必要です。

Claude Code は現在の作業ディレクトリとその親、`~/.claude/` のユーザーディレクトリ、および管理設定から出力スタイルを検出します。Hooks およびその他の `.claude/settings.json` キーは、現在の作業ディレクトリの `.claude/` フォルダから親ディレクトリへのフォールバックなしで読み込まれ、ユーザーの `~/.claude/settings.json` および管理設定と共に読み込まれます。`.claude/settings.local.json` は git リポジトリルートから読み込まれます。Claude Code がサブディレクトリで起動された場合でも、Windows など Claude Code が[リポジトリルートを使用しない](/docs/ja/settings#where-claude-code-looks-for-each-file)場合を除きます。v2.1.211 より前では、これも現在の作業ディレクトリからのみ読み込まれました。[Agent SDK](/docs/ja/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)セッションはすべてのバージョンで作業ディレクトリから読み込みます。

その設定をプロジェクト全体で共有するには、次のいずれかのアプローチを使用します。

* **ユーザーレベルの設定**：`~/.claude/agents/`、`~/.claude/output-styles/`、または `~/.claude/settings.json` にファイルを配置して、すべてのプロジェクトで利用可能にします
* **プラグイン**：設定を[プラグイン](/docs/ja/plugins/overview)としてパッケージ化および配布し、チームがインストールできるようにします
* **設定ディレクトリから起動する**：使用する `.claude/` 設定を含むディレクトリから Claude Code を実行します

<h2 id="how-permissions-interact-with-sandboxing">
  権限とサンドボックス化がどのように相互作用するか
</h2>

権限と[サンドボックス化](/docs/ja/sandboxing)は補完的なセキュリティレイヤーです。

* **権限**は Claude Code が使用できるツール、およびアクセスできるファイルやドメインを制御します。これらは Bash、Read、Edit、WebFetch、MCP、およびその他すべてのツールに適用されます。ただし、他のツールが残っている場合、deny ルールまたは ask ルールは[`EndConversation`](/docs/ja/tools-reference#endconversation-tool-behavior)をブロックできません。
* **サンドボックス化**は OS レベルの強制を提供し、シェルコマンドのファイルシステムとネットワークアクセスを制限します。これは Bash、PowerShell、および[Monitor](/docs/ja/tools-reference#monitor-tool)コマンドとその子プロセスにのみ適用されます。

防御の多層化のために両方を使用してください。プロンプトインジェクションが Claude の意思決定をバイパスしても、サンドボックス制限は引き続き適用されます。サンドボックス設定と権限ルールからのパスとドメインは[最終的なサンドボックス構成にマージされます](/docs/ja/sandboxing#permission-rules)。

サンドボックス化を有効にして `autoAllowBashIfSandboxed` をデフォルトの `true` のままにすると、サンドボックス化された Bash コマンドは、権限に bare `Bash` ask ルール、または[同等の `Bash(*)` フォーム](#match-all-uses-of-a-tool)が含まれている場合でも、プロンプトなしで実行されます。サンドボックス境界がそのツール全体のプロンプトの代わりになります。

[プランモード](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode)では、Claude Code はこの置き換えをスキップします。ask ルールがない場合、[組み込みの読み取り専用コマンド](#read-only-commands)は引き続きプロンプトなしで実行され、その他のシェルコマンドはプランニング中に通常の権限フローを通過します。プランモードの詳細については、[プランモード](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode)を参照して、Claude Code がそこでコマンドをどのようにゲートするかを確認してください。bare `Bash` ask ルールがある場合、サンドボックス化された読み取り専用コマンドを含むすべての Bash コマンドがプロンプトされます。これはサンドボックス化の外側と同じです。v2.1.212 より前では、置き換えはプランモードでも適用されていました。

これらのチェックは引き続き適用されます。

* `Bash(git push *)` のようなコンテンツスコープの ask ルールは引き続きプロンプトを強制します
* 明示的な deny ルールは引き続き適用されます
* [重要なパス](/docs/ja/permission-modes#critical-paths)をターゲットとする `rm` または `rmdir` コマンドは引き続き通常の権限フローを通過します

除外されたコマンドなど、サンドボックス化されて実行されないコマンドは、通常どおり bare `Bash` ask ルールを尊重します。この動作を変更するには、[サンドボックスモード](/docs/ja/sandboxing#sandbox-modes)を参照してください。

<span id="managed-only-settings" />

<h2 id="managed-settings">
  管理設定
</h2>

一元的な制御が必要な組織の場合、管理者はユーザーおよびプロジェクト設定でオーバーライドできない管理設定をデプロイします。ただし、いくつかの[セキュリティに関連するキー](/docs/ja/settings#exceptions-to-managed-settings-precedence)は例外です。[管理設定をデプロイする](/docs/ja/managed-settings)では、配信メカニズム、管理層内での優先順位、および[管理設定のみが設定できるキー](/docs/ja/managed-settings#managed-only-settings)について説明しています。

これらのキーの 1 つである[`allowManagedPermissionRulesOnly`](/docs/ja/settings-reference#allowmanagedpermissionrulesonly)は、管理設定を権限ルールの唯一の設定ソースにします。そのエントリには、Claude Code がその後無視するすべてのソースが記載されています。

`disableBypassPermissionsMode` は通常、組織ポリシーを強制するために管理設定に配置されますが、任意のスコープから機能します。ユーザーは独自の設定で設定して、自分自身をバイパスモードからロックアウトできます。

<h2 id="settings-precedence">
  設定の優先順位
</h2>

権限ルールは、他のすべての Claude Code 設定と同じ[設定優先順位](/docs/ja/settings#settings-precedence)に従います。管理設定が最も高い優先度を持ちます。コマンドライン引数を含む他のレベルは、管理権限ルールをオーバーライドできません。

ツールがいずれかのレベルで拒否されている場合、他のレベルはそれを許可できません。たとえば、管理設定の deny は `--allowedTools` でオーバーライドできず、`--disallowedTools` は管理設定が定義する内容を超えて制限を追加できます。

設定スコープ全体でも同じことが当てはまります。ユーザー設定で権限が許可されており、プロジェクト設定で拒否されている場合、拒否ルールがそれをブロックします。逆も同様です。ユーザーレベルの deny がプロジェクトレベルの allow をブロックします。これは、任意のスコープからの deny ルールが allow ルールの前に評価されるためです。

埋め込みホストは、SDK の `managedSettings` オプションを介して追加の管理ポリシーを提供できます。これには、管理者が `allowManaged*Only` ロックを設定していない限り、権限許可ルールが含まれます。[Claude Desktop セッションにポリシーを配信する](/docs/ja/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions)では、埋め込み元ポリシーがいつ適用されるかについて説明しています。

<h2 id="project-allow-rules-and-workspace-trust">
  プロジェクトの許可ルールとワークスペーストラスト
</h2>

プロジェクトの `.claude/settings.json` 内の `permissions.allow` ルールと `permissions.additionalDirectories` エントリは機能を付与するため、Claude Code はそのフォルダの[ワークスペーストラストダイアログ](/docs/ja/security#additional-safeguards)を受け入れた後にのみ適用します。ダイアログにはフォルダが付与するルールとディレクトリが表示されるため、受け入れる前に確認できます。`deny` ルールと `ask` ルールは制限のみを行うため、影響を受けません。

Claude Code はワークスペーストラストをそれを起動した場所に応じてキーとして保存します。

* リポジトリ内では、Claude Code は git リポジトリルートをキーとしてトラストを保存するため、トラストはリポジトリ全体をカバーします。ただし、サブモジュールなどのネストされた git リポジトリは除きます。[worktree](/docs/ja/worktrees) では、[保存されたルール](#permission-system)の場合と同様にメインチェックアウトのルートを使用します。
* リポジトリ外では、Claude Code はそれを起動したディレクトリをキーとしてトラストを保存し、そのディレクトリのサブディレクトリをカバーします。ただし、クローンなどのネストされた git リポジトリは除きます。カバーされた各サブディレクトリは、その親を信頼したフォルダとしてカウントされます。
* ホームディレクトリから起動した場合、Claude Code はトラストを現在のセッションのみ保持し、ディスクに書き込みません。[追加のセーフガード](/docs/ja/security#additional-safeguards)に関する注記を参照してください。

Claude Code はインタラクティブセッションでのみトラストダイアログを表示します。`claude -p` 実行または SDK セッションはダイアログを表示しません。親フォルダを信頼してもこれらのルールにはカウントされないため、[フォルダを信頼する前に実行されるもの](#what-runs-before-you-trust-a-folder)は、これら 2 つの状況で Claude Code がどのリポジトリコンテンツを使用するかを説明しています。

<h3 id="when-your-local-settings-file-needs-trust">
  ローカル設定ファイルがトラストを必要とする場合
</h3>

`.claude/settings.local.json` は通常あなた自身のファイルであるため、Claude Code はトラストステップなしにその許可ルールと追加ディレクトリを適用します。ファイルが git で追跡されている場合、または `.claude` がシンボリックリンクである場合、Claude Code はそれをリポジトリ提供として扱い、フォルダを信頼するまでそのルールを保持します。

Claude Code は git を実行して 2 つを区別し、フォルダを信頼した後にのみ git を実行します。つまり、そのトラストダイアログを受け入れたか、トラストがそれに拡張される親ディレクトリを受け入れたか、`-p` または SDK セッションにいます。これはトラストが受け入れられたとしてカウントされます。それまでは、Claude Code を起動した場所がファイルのルールに何が起こるかを決定します。

* **設定ホーム内：** Claude Code は git を実行せずにそのフォルダの `.claude/settings.local.json` をすぐに適用します。設定ホームはホームディレクトリ、または `.claude` サブディレクトリを [`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars#variables) として設定したディレクトリです。その `CLAUDE_CONFIG_DIR` ディレクトリが git リポジトリ内にあり、Claude Code が[リポジトリルートでローカル設定を保持](/docs/ja/settings#where-claude-code-looks-for-each-file)する場合、他の場所と同様にルールを保持します。
* **その他の場所：** Claude Code はプロジェクト設定と同様にファイルのルールを保持します。チェックが実行されると、Claude Code は追跡されていないファイルのルール、またはどの git リポジトリの外にあるディレクトリ内のファイルのルールを適用します。その正確なフォルダを信頼していなくても同様です。

<Note>
  設定ホーム例外はトラストステップのみをスキップします。`~/.claude/settings.local.json` は依然として[ローカルスコープ](/docs/ja/settings#compare-the-scope-of-each-settings-file)であるため、Claude Code はホームディレクトリ自体で起動したセッションでのみそれを読み込み、すべてのプロジェクトでは読み込みません。すべてのプロジェクトに許可ルールを適用するには、代わりにユーザー設定に追加してください。`~/.claude/settings.json`、または `CLAUDE_CONFIG_DIR` が設定されている場合は `$CLAUDE_CONFIG_DIR/settings.json`。
</Note>

バージョン 2.1.196 から 2.1.199 では、Claude Code は設定ホームと git リポジトリ外のファイルのルールを保持し、そこに[`this workspace has not been trusted`](/docs/ja/errors#workspace-has-not-been-trusted)警告を出力していました。v2.1.207 より前では、Claude Code はダイアログを受け入れる前に追跡されていないファイルのルールを適用していました。

<h3 id="what-runs-before-you-trust-a-folder">
  フォルダを信頼する前に実行されるもの
</h3>

各行はリポジトリが提供できるコンテンツの 1 種類です。列は、フォルダ自体を信頼していない 2 つの状況です。親フォルダのみを信頼したか、そこで `claude -p` または SDK を実行しました。これはトラストダイアログを表示しません。親フォルダ列は[ネストされたリポジトリ](#project-allow-rules-and-workspace-trust)内には適用されません。インタラクティブセッションでは Claude Code はそれのトラストダイアログを表示し、`claude -p` または SDK 実行はそこで `claude -p` 列に従います。

| リポジトリが提供するもの                                                                                                                                                                                                                                                                  | 親フォルダのみを信頼した                                                                                              | `claude -p` または SDK、フォルダは信頼されていない                                                                                                |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| 設定ファイル内の[Hooks](/docs/ja/hooks)、[`env`](/docs/ja/settings-reference#env)ブロック、[`apiKeyHelper`](/docs/ja/settings-reference#apikeyhelper)などのヘルパーコマンド、およびプロジェクトスキルの[hooks](/docs/ja/hooks#hooks-in-skills-and-agents)と[`allowed-tools`](/docs/ja/skills#pre-approve-tools-for-a-skill)                    | 使用                                                                                                        | 使用。ワークスペーストラストはどのセッションでもスキルの `allowed-tools` をゲートしません                                                                            |
| `.claude/settings.json` 内の `permissions.allow` ルールと `additionalDirectories`                                                                                                                                                                                                   | トラストダイアログを受け入れるまで使用されません。ダイアログは再度表示され、それらをリストします                                                          | 使用されません。Claude Code は stderr に[`this workspace has not been trusted`](/docs/ja/errors#workspace-has-not-been-trusted)警告を出力します         |
| プロジェクト[subagent](/docs/ja/sub-agents#hooks-in-subagent-frontmatter)のフロントマターフック、プロジェクト[`@skills-dir` プラグイン](/docs/ja/plugins/loading#plugins-shared-through-a-repository)、およびリポジトリまたは `--add-dir` ディレクトリからの[`extraKnownMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces)エントリ | 使用されず、ダイアログは提供されません                                                                                       | 使用されません                                                                                                                          |
| リポジトリまたは `--add-dir` ディレクトリからの subagent のフロントマター内のインライン[`mcpServers`](/docs/ja/sub-agents#scope-mcp-servers-to-a-subagent)。v2.1.238 より前では、Claude Code はこれらのサーバーを両方の状況で読み込んでいました                                                                                                    | 使用されず、ダイアログは提供されません                                                                                       | 使用されません                                                                                                                          |
| `.mcp.json` 内のサーバー。リポジトリが[独自の設定で承認](/docs/ja/mcp#project-server-approvals-and-workspace-trust)するものを含む                                                                                                                                                                              | Claude Code は接続する前にあなたに尋ねます。リポジトリ独自の承認はカウントされません                                                          | 承認されているかどうかに関わらず接続されます。SDK はセッティングソースがプロジェクト設定を含む場合にのみそれらを読み込みます。同じフォルダの `claude mcp list` はそのようなサーバーを保留中として報告します                |
| `.mcp.json` 内のサーバー上の[`headersHelper`](/docs/ja/mcp#trust-a-folder-before-its-headershelper-runs)。v2.1.238 より前では、Claude Code はヘルパーを両方の状況で実行していました                                                                                                                                   | トラストダイアログを受け入れるまで実行されません。ダイアログは再度表示され、ヘルパーが宣言されている場所を名前で指定します。Claude Code はそれまでサーバーを静的 `headers` のみで接続します | 実行されません。Claude Code はサーバーを静的 `headers` のみで接続し、サーバーごとに stderr に[`headersHelper not run`](/docs/ja/errors#headershelper-not-run)行を出力します |

このフォルダを信頼する必要がある行については、手動で信頼してください。`~/.claude.json` で `projects["<path>"].hasTrustDialogAccepted` を `true` に設定します。`<path>` はリポジトリルート、またはリポジトリ外のフォルダ自体です。Claude Code はスキップされた subagent フックまたはインライン MCP サーバーのデバッグログ行、スキップされた許可ルールの stderr 警告、およびスキップされたヘルパーの `headersHelper not run` 行に正確なキーを出力します。

あなたが書いていないリポジトリで `claude -p` を実行する前に、マシンで実行される可能性があるものを決定してください。

* `--setting-sources user` を渡すか、SDK の `settingSources` をプロジェクト設定なしで設定して、Claude Code がプロジェクトの設定ファイルも `.mcp.json` も読み込まないようにします
* [`--bare`](/docs/ja/headless#start-faster-with-bare-mode)で起動して、Claude Code がプロジェクトから hooks、skills、カスタムコマンド、subagents、プラグイン、または `.mcp.json` サーバーを読み込まないようにします。プロジェクトの `env` ブロックと `awsAuthRefresh` などのヘルパーはその設定ファイルで依然として適用され、Claude Code は `apiKeyHelper` を `--settings` からのみ読み込みます
* `--settings '{"disableAllHooks": true}'` を渡して、その実行の[hooks をオフにします](/docs/ja/hooks#disable-or-remove-hooks)。ユーザー設定のみで設定するのは十分ではありません。リポジトリのプロジェクト設定があなたのものより優先され、それを `false` に戻すことができるためです
* [`disabledMcpjsonServers`](/docs/ja/settings-reference#disabledmcpjsonservers)エントリをすべてのセッションタイプで名前で `.mcp.json` サーバーを拒否するために追加します

<h2 id="example-configurations">
  設定例
</h2>

この[リポジトリ](https://github.com/anthropics/claude-code/tree/main/examples/settings)には、一般的なデプロイメントシナリオのスターター設定が含まれています。これらを出発点として使用し、ニーズに合わせて調整してください。

<h2 id="see-also">
  関連項目
</h2>

* [すべての設定](/docs/ja/settings-reference#permission-settings)：権限キーを含むすべての設定キー
* [auto モードを設定する](/docs/ja/auto-mode-config)：auto モード分類器に組織が信頼するインフラストラクチャを伝えます
* [サンドボックス](/docs/ja/sandboxing)：Bash コマンドの OS レベルのファイルシステムとネットワーク分離
* [認証](/docs/ja/authentication)：Claude Code へのユーザーアクセスを設定します
* [セキュリティ](/docs/ja/security)：セキュリティ保護とベストプラクティス
* [フック](/docs/ja/hooks-guide)：ワークフローを自動化し、権限評価を拡張します
