> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# CLI から プラグインを推奨する

> Claude Code ユーザーに対して、CLI または SDK から claude-code-hint タグを出力することで、公式マーケットプレイスのプラグインをインストールするよう促します。

CLI または SDK を保守している場合、ツールは Claude Code ユーザーに対してプラグインのインストールを促すことができます。CLI が Claude Code 内で実行されていることを検出したら、1 行の `<claude-code-hint />` タグを stderr に書き込みます。Claude Code はこの行を Bash および PowerShell ツール出力からモデルが見る前に削除し、ユーザーに 1 回限りのインストール プロンプトを表示します。

このページは、プラグインが `claude-plugins-official` または Anthropic の [公式マーケットプレイス名](/docs/ja/plugins/security#official-marketplace-names) を持つ別のマーケットプレイスにリストされている場合にのみ適用されます。コミュニティ マーケットプレイスである `claude-community` はこれに該当しません。

<Note>
  プラグインを公開するには、[プラグインを公開および配布する](/docs/ja/plugins/publish) を参照してください。
</Note>

<h2 id="emit-the-hint">
  ヒントを出力する
</h2>

`CLAUDECODE` または `CLAUDE_CODE_CHILD_SESSION` が設定されている場合にのみタグを出力して、ユーザーが CLI を直接実行するときに表示されないようにします。

Claude Code は Bash および PowerShell ツールを通じて実行するコマンドおよびフック コマンドで `CLAUDECODE=1` を設定します。v2.1.172 以降では、そこで `CLAUDE_CODE_CHILD_SESSION=1` も設定されます。変数は、どのプロセスがそれらを保持するかが異なります。

* **`CLAUDECODE`**: すべての Claude Code バージョンで設定されます。IDE 拡張機能は統合ターミナルでも設定するため、`CLAUDECODE` だけでゲートすると、ユーザーがこれらのターミナルの 1 つで CLI を直接実行する場合にもタグが出力されます
* **`CLAUDE_CODE_CHILD_SESSION`**: Claude Code 自体が開始するサブプロセスでのみ設定されます。v2.1.172 以降が必要な場合に使用します

[環境変数リファレンス](/docs/ja/env-vars) に詳細があります。

以下の例は、最も広いリーチのために `CLAUDECODE` でゲートし、公式マーケットプレイスの `example-cli` という名前のプラグインのヒントを出力します。

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  if [ -n "$CLAUDECODE" ]; then
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  fi
  ```
</CodeGroup>

公式マーケットプレイスのプラグイン名で `example-cli` を置き換えます。

Claude Code は各プラグインに対して 1 回プロンプトを表示するため、すべての呼び出しでヒントを出力できます。

エミッターをチェックするには、ターミナルで `CLAUDECODE=1 example-cli` を実行してタグ行が stderr に表示されることを確認し、変数なしで `example-cli` を実行して追加の出力がないことを確認します。

<h2 id="hint-format">
  ヒント形式
</h2>

タグは独自の行を占める必要があります。Claude Code は行の途中に埋め込まれたタグを無視します。

タグは 3 つの属性を取り、すべて必須です。

| 属性      | 説明                              |
| :------ | :------------------------------ |
| `v`     | プロトコル バージョン。`1` のみがサポートされている値です |
| `type`  | ヒント種別。`plugin` のみがサポートされている値です  |
| `value` | `name@marketplace` 形式のプラグイン識別子  |

値は二重引用符で囲むか、引用符なしにすることができます。引用符なしの値には空白を含めることはできません。

Claude Code は `v` または `type` が認識されない場合でも、行を出力から削除します。

<h2 id="check-when-the-prompt-appears">
  プロンプトが表示されるタイミングをチェックする
</h2>

プロンプトは対話型ターミナル セッションでのみ表示されます。`claude -p` 実行、サブエージェント実行、およびフック コマンド出力では、タグが削除され、プロンプトは表示されません。以下のチェックもすべてパスする必要があります。

* **公式かつインストール可能**: `value` は Claude Code がローカル コピーの公式マーケットプレイスで見つけるプラグイン、まだインストールされていないプラグイン、およびポリシーがブロックしていないプラグインを指定します
* **分析がオン**: Claude Code の分析がオフのセッション（例えば `DISABLE_TELEMETRY`、`DO_NOT_TRACK`、または `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` が設定されているセッション、または Amazon Bedrock などのサードパーティ プロバイダー上のセッション（[自動テレメトリ オプトアウト](/docs/ja/data-usage#default-behaviors-by-api-provider) が適用される）では、プロンプトが表示されません
* **頻度制限**: セッションごとに 1 つのプロンプト、ユーザーの回答に関係なくプラグインごとに 1 回のプロンプト、およびそのマシンで 100 個のプラグインに対してプロンプトが表示された後は表示されません
* **オフになっていない**: ユーザーが **いいえ、プラグイン インストール ヒントを再度表示しない** を選択していません
* **ローカル、有人セッション**: セッションのワークスペースはクラウドまたはリモート マシンではなくローカルであり、セッションは無人で実行されていません。例えば、`--cloud` で開始されたセッション、リモート コントロールを提供するセッション、またはエージェント チーム メンバーはプロンプトを表示しません

<h2 id="preview-what-the-user-sees">
  ユーザーに表示される内容をプレビューする
</h2>

[プロンプトが表示されるタイミングをチェックする](#check-when-the-prompt-appears) のチェックがパスすると、Claude Code は次のような **プラグイン推奨** ダイアログを表示します。

```text theme={null}
─────────────────────────────────────────────────────────────
  Plugin recommendation

    The example-cli command suggests installing a plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Description: Official integration for example-cli deployments

    Would you like to install it?
    ❯ 1. Yes, install
      2. No
      3. No, and don't show plugin installation hints again

─────────────────────────────────────────────────────────────
```

ダイアログは Claude が実行したシェル コマンドの最初の単語を指定するため、ユーザーは不一致を検出できます。各回答には 1 つの効果があります。

* **Yes, install**: [ユーザー スコープ](/docs/ja/plugins/install) でプラグインをインストールします
* **No, and don't show plugin installation hints again**: そのユーザーの将来のヒント プロンプトをオフにします
* **30 秒間回答なし**: **No** としてカウントされます

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグインを公開および配布する](/docs/ja/plugins/publish): 公式マーケットプレイスを含む各マーケットプレイスへのルート（ヒントが必要）
* [プラグイン コマンド リファレンス](/docs/ja/plugins/cli-reference#plugin-install): セッション外で同じプラグインをインストールするシェル コマンド
