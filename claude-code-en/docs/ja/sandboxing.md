> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# サンドボックス化された Bash ツールを設定する

> Claude Code のサンドボックス化された Bash ツールがファイルシステムとネットワークの分離を提供し、より安全で自律的なエージェント実行を実現する方法について学びます。

Bash サンドボックスを使用すると、Claude はほとんどのシェルコマンドを実行できます。各コマンドの実行許可を求める代わりに、コマンドがアクセスできるファイルとネットワークドメインを定義し、オペレーティングシステムがすべての Bash、PowerShell、Monitor コマンドとその子プロセスに対してその境界を実施します。

<Note>
  dev コンテナ、カスタムコンテナ、仮想マシンなどの他の分離アプローチを比較するには、[Sandbox environments](/docs/ja/sandbox-environments) を参照してください。Bash 以外のツールの許可プロンプトを削減するには、[permission modes](/docs/ja/permission-modes) を参照してください。
</Note>

<h2 id="get-started">
  開始方法
</h2>

サンドボックスは Claude Code に組み込まれており、macOS、Linux、WSL2 で実行されます。ネイティブ Windows はサポートされていません。Windows では、Claude Code を WSL2 ディストリビューション内で実行してください。

macOS では、インストールするものはありません。サンドボックス化は組み込みの Seatbelt フレームワークを使用します。Linux と WSL2 では、サンドボックスは 2 つのパッケージに依存しており、[Linux と WSL2 をセットアップする](#set-up-linux-and-wsl2)で説明されています。まだインストールしていない場合でも、`/sandbox` で開始できます。そのパネルには、何が不足しているかが表示されます。

<Steps>
  <Step title="/sandbox を実行する">
    Claude Code セッションを開始し、`/sandbox` コマンドを実行します。

    ```text theme={null}
    /sandbox
    ```

    これにより、3 つのタブを持つサンドボックスパネルが開きます。Linux で seccomp フィルターが不足している場合は、Dependencies タブが追加されます。

    * **Mode**：サンドボックス化されたコマンドがどのように承認されるかを選択します。次のステップで説明します
    * **Overrides**：サンドボックス内で失敗するコマンドがサンドボックス化されていない状態で実行にフォールバックできるかどうかを選択します。これは [`allowUnsandboxedCommands`](/docs/ja/settings-reference#sandbox-allowunsandboxedcommands) 設定です
    * **Config**：解決されたサンドボックス設定を表示します

    パネルに Dependencies タブのみが表示される場合、必要なパッケージが不足しています。[Linux と WSL2 をセットアップする](#set-up-linux-and-wsl2)で説明されているようにインストールし、Claude Code を再起動して、`/sandbox` を再度実行してください。
  </Step>

  <Step title="モードを選択する">
    Mode タブで、自動許可または通常の許可を選択します。自動許可はサンドボックス化されたコマンドをプロンプトなしで実行し、通常の許可はコマンドがサンドボックス化されている場合でも通常の許可プロンプトを保持します。自動許可モードでもプロンプトが表示されるコマンドについては、[Sandbox modes](#sandbox-modes) を参照してください。
  </Step>

  <Step title="Bash コマンドを実行する">
    Claude にコマンド（ビルドやテストスイートなど）を実行するよう依頼します。デフォルトでは、サンドボックス内のコマンドは作業ディレクトリ、セッション一時ディレクトリ、および `--add-dir`、`/add-dir`、または `permissions.additionalDirectories` で[追加したディレクトリ](/docs/ja/permissions#additional-directories-grant-file-access-not-configuration)に書き込みできます。

    コマンドが新しいネットワークドメインにアクセスする必要がある場合、Claude Code は承認を求めるか、[自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)では分類器にリクエストを送信します。

    サンドボックス化された状態で実行できないコマンドは、通常の許可フローにフォールバックします。Claude Code はそれらの許可プロンプトを「Bash command」ではなく「Bash command (unsandboxed)」というタイトルで表示するため、どのコマンドがサンドボックス外で実行されたかを判断できます。サンドボックスが許可する内容を広げたり狭めたりするには、[サンドボックス化を設定](#configure-sandboxing)を参照してください。

    サンドボックス化されたコマンドがコンテナ内で `Operation not permitted` で失敗する場合は、[トラブルシューティング](#troubleshooting)の Bubblewrap エントリを参照してください。
  </Step>
</Steps>

パネルでモードを選択すると、Claude Code はそれをプロジェクトのローカル設定 `.claude/settings.local.json` に保存します。これは現在のプロジェクトに適用されます。Claude Code はそこに設定を保存する際に、そのファイルをグローバル gitignore に追加します。すべてのプロジェクトでサンドボックスを有効化するには、ユーザー設定 `~/.claude/settings.json` で [`sandbox.enabled`](/docs/ja/settings-reference#sandbox-enabled) を `true` に設定します。組織内のすべての開発者にサンドボックス化を実施するには、[管理設定で実施](#enforce-sandboxing-with-managed-settings)を使用します。

1 つのセッションのみでサンドボックスを変更し、設定ファイルに書き込まないようにするには、Claude Code を [`--settings`](/docs/ja/settings#change-a-setting-for-one-session) で起動します。たとえば、このコマンドは、Claude がブロックされたコマンドをサンドボックス外で再試行できないサンドボックス化されたセッションを開始します。

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

<Warning>
  デフォルトでは、依存関係が不足しているか、プラットフォームがサポートされていないためにサンドボックスが起動できない場合、Claude Code は警告を表示してサンドボックス化なしでコマンドを実行します。これをハード失敗にするには、[`sandbox.failIfUnavailable`](/docs/ja/settings-reference#sandbox-failifunavailable) を `true` に設定します。これは、セキュリティゲートとしてサンドボックス化を必要とする管理デプロイメント向けです。
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Linux と WSL2 をセットアップする
</h3>

Linux と WSL2 では、サンドボックスは 2 つのパッケージに依存しています。

* [`bubblewrap`](https://github.com/containers/bubblewrap)：ファイルシステム分離を実施する非特権サンドボックス化ツール
* [`socat`](http://www.dest-unreach.org/socat/)：サンドボックスプロキシを通じてネットワークトラフィックをルーティングするために使用されるリレー

ディストリビューションのパッケージマネージャーでインストールします。

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

依存関係が不足している場合、`/sandbox` の Dependencies タブに、`ripgrep`、`bubblewrap`、`socat`、および seccomp フィルターのうち、プラットフォームで不足しているものが表示されます。Claude Code を再起動してから `/sandbox` を実行しても Dependencies タブが表示されない場合は、すべての依存関係が存在します。

Ripgrep はネイティブ Claude Code バイナリにバンドルされています。seccomp フィルターはオプションで、Unix ドメインソケットのブロッキングを追加します。不足している場合は、`npm install -g @anthropic-ai/sandbox-runtime` でインストールしてください。

必要な依存関係が不足している場合、Dependencies タブはインストールするまで唯一のタブとして表示されます。オプションの seccomp フィルターのみが不足している場合、Dependencies タブは他のタブと並んで表示されます。依存関係チェックはスタートアップ時に実行されるため、パッケージをインストール後に Claude Code を再起動して、`/sandbox` がそれらを検出するようにしてください。

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 以降：bubblewrap がユーザー名前空間を作成できるようにする">
    Ubuntu 24.04 以降では、デフォルトの AppArmor ポリシーが bubblewrap が分離に必要とするユーザー名前空間の作成を防止します。

    WSL2 内を含む、環境がこの制限を実施しているかどうかを確認するには、`sysctl kernel.apparmor_restrict_unprivileged_userns` を実行します。コマンドが `0` を返す場合は、このステップをスキップしてください。`No such file or directory` エラーが表示される場合は、キーが存在しないため、このステップをスキップできます。`1` を返す場合は、`bwrap` にこの機能を付与する AppArmor プロファイルを追加します。

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    プロファイルは `bwrap` 自体にのみ適用され、サンドボックス内で実行されるコマンドには適用されません。AppArmor を再度読み込んで適用します。

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="WSL2 に関する注記">
    PowerShell から `wsl -l -v` で WSL バージョンを確認します。`Sandboxing requires WSL2` が表示される場合、ディストリビューションは WSL1 で実行されています。WSL2 にアップグレードするか、Claude Code をサンドボックス化なしで実行してください。

    WSL2 では、WSL は `cmd.exe`、`powershell.exe`、または `/mnt/c/` 下のものなどの Windows バイナリの起動を Windows ホストに Unix ソケット経由で渡すため、サンドボックス化されたコマンドがそれを起動できるかどうかは、サンドボックスの [Unix ソケット設定](/docs/ja/settings-reference#sandbox-network-allowunixsockets)に従います。オプションの seccomp フィルターをインストールして、最初の場所でソケットをブロックする必要があります。これらの起動を許可するには、`allowAllUnixSockets` を設定します。サンドボックスから完全に除外するには、コマンドを [`excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) に追加します。
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  サンドボックスモード
</h3>

Claude Code は 2 つのサンドボックスモードを提供します。どちらでも、サンドボックスは同じファイルシステムとネットワーク制限を実施します。違いは、サンドボックス化されたコマンドが自動承認されるか、明示的な許可が必要かだけです。

<h4 id="auto-allow-mode">
  自動許可モード
</h4>

コマンドがサンドボックス化できる場合、Claude Code はそれをサンドボックス内で実行し、許可なしに自動的に承認します。許可されていないホストへのネットワークアクセスが必要なコマンドなど、サンドボックス化できないコマンドは、通常の許可フローにフォールバックします。そこで Claude Code は [許可ルール](/docs/ja/permissions)を確認し、それらのルールが既に許可していないコマンドについてゲートを設定します。Manual モードではプロンプトが表示されるか、[自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)では分類器が使用されます。

自動許可モードでも、以下が適用されます。

* 明示的な [拒否ルール](/docs/ja/permissions)は常に尊重されます
* [重要なパス](/docs/ja/permission-modes#critical-paths)をターゲットにする `rm` または `rmdir` コマンドは、依然として通常の許可フローを通じます
* `Bash(git push *)` のようなコンテンツスコープの [ask ルール](/docs/ja/permissions)は、サンドボックス化されたコマンドでも強制的にプロンプトを表示します
* 単純な `Bash` ask ルール、または同等の `Bash(*)` 形式は、サンドボックス化されて実行されるコマンドではスキップされます。通常の許可フローにフォールバックするコマンドには依然として適用されます。[Plan Mode](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode) では、ルールはスキップされません。読み取り専用のものを含む、サンドボックス化されたコマンドのプロンプトを表示します。v2.1.212 より前では、スキップは Plan Mode でも適用されていました

<Info>
  自動許可モードは許可モード設定とは独立して動作します。ただし 3 つの例外があります。[Plan Mode](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode)、[per-command allowed domains](#per-command-allowed-domains-in-auto-mode) を含む自動モードコマンド、および自動モードでのサンドボックス化されたコマンドの [server-side classifier review](/docs/ja/permission-modes#how-the-classifier-evaluates-actions)です。「編集を受け入れる」モードでない場合でも、自動許可が有効な場合、サンドボックス化された Bash コマンドは自動的に実行されます。これは、ファイル編集ツールが通常は Manual モードでプロンプトを表示する場合でも、サンドボックス境界内のファイルを変更する Bash コマンドはプロンプトなしに実行されることを意味します。

  Plan Mode では、自動許可は承認を広げません。Claude Code が計画中にコマンドをゲートする方法については、[Plan Mode](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode) を参照してください。v2.1.212 より前では、自動許可は Plan Mode でもプロンプトなしにサンドボックス化されたコマンドを実行していました。
</Info>

<h4 id="regular-permissions-mode">
  通常の許可モード
</h4>

すべての Bash コマンドは、サンドボックス化されている場合でも、通常の許可フローを通じます。これはより多くの制御を提供しますが、より多くの承認が必要です。

<h4 id="the-unsandboxed-retry-escape-hatch">
  サンドボックス化されていない再試行エスケープハッチ
</h4>

一部のコマンドはサンドボックス内でまったく実行できません。これは、それと互換性がないツール、または許可していないホストが必要なツールなどです。Claude Code はサンドボックス違反をブロックされたコマンドの結果で報告し、サンドボックスが拒否したパス またはホストを名前で指定するため、Claude はサンドボックスがブロックしたものを確認できます。タスクを失敗させたり、サンドボックス化をオフにするよう要求したりするのではなく、Claude Code には意図的なエスケープハッチが含まれています。Claude は違反を分析し、`dangerouslyDisableSandbox` パラメータでコマンドを再試行する可能性があります。

再試行されたコマンドはサンドボックス外で実行されるため、通常の許可フローを通じます。Manual モードでは確認プロンプトが表示されます。[自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)では、分類器は基礎となるコマンドを評価します。[`permissions.blockReadsOutsideWorkingDirectories`](/docs/ja/settings-reference#permissions-blockreadsoutsideworkingdirectories) がオンの間、サンドボックス外で実行するために承認が必要な再試行はプロンプトを表示します。自動モードでもサンドボックス化されていない再試行のたびにプロンプトが表示されるようにするには、`Bash(dangerouslyDisableSandbox:true)` の [ask ルール](/docs/ja/permissions#match-by-input-parameter)を追加してください。

このエスケープハッチは、[サンドボックス設定](/docs/ja/settings-reference#sandbox-settings)で `"allowUnsandboxedCommands": false` を設定することで無効化できます。エスケープハッチが無効化されると、Claude Code は `dangerouslyDisableSandbox` パラメータを無視し、すべてのコマンドは `excludedCommands` にリストされていない限り、サンドボックス化されて実行される必要があります。`/sandbox` **Overrides** タブはこの設定を **Strict sandbox mode** として表示します。

Strict sandbox mode は Claude が実行するコマンドに適用されます。[`!` シェルモードプロンプト](/docs/ja/interactive-mode#shell-mode-with-prefix)で自分で入力するコマンドは、セッションが以下のいずれかでない限り、サンドボックス外で実行されます。

* **[バックグラウンドセッション](/docs/ja/agent-view)：** strict sandbox mode はシェルモードコマンドもカバーします
* **[`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ja/env-vars#variables) が設定された Linux セッション：** すべてのコマンドがサンドボックス化され、シェルモードコマンドを含みます

v2.1.260 より前では、strict sandbox mode はすべてのセッションでシェルモードコマンドをサンドボックス化していました。

<h4 id="temporary-directories">
  一時ディレクトリ
</h4>

セッション一時ディレクトリは、デフォルトで作業ディレクトリと並んでサンドボックス内で書き込み可能です。[ファイルシステム分離を無効化](#disable-filesystem-isolation)しない限り、Claude Code はサンドボックス化されたコマンドに対して `$TMPDIR` をこのディレクトリに設定するため、一時ファイルを書き込むツールは追加の設定なしで動作します。サンドボックス化されていないコマンドは、シェルの `$TMPDIR` を変更されずに継承するため、ファイルシステム分離がオンの間、サンドボックス化されたコマンドとサンドボックス化されていないコマンドは `$TMPDIR` を異なるディレクトリに解決します。シェルが `$TMPDIR` を設定していない場合、またはシェルが空のままの場合、サンドボックス化されていないコマンドが `$TMPDIR` を参照すると、[`CLAUDE_CODE_TMPDIR`](/docs/ja/env-vars) オーバーライドを受け取るか、設定していない場合またはオーバーライドが長いパスの場合は、オペレーティングシステムの一時ディレクトリを受け取ります。変数が空の文字列に展開されないようにするためです。2 つの間で一時ファイルを渡すには、代わりに作業ディレクトリの下に書き込んでください。

<h2 id="configure-sandboxing">
  サンドボックスの設定
</h2>

`settings.json` ファイルを通じてサンドボックスの動作をカスタマイズできます。完全な設定リファレンスについては、[設定](/docs/ja/settings-reference#sandbox-settings)を参照してください。

デフォルトでは、サンドボックス化されたコマンドは現在の作業ディレクトリ、セッション一時ディレクトリ、および `--add-dir`、`/add-dir`、または `permissions.additionalDirectories` で[追加したディレクトリ](/docs/ja/permissions#additional-directories-grant-file-access-not-configuration)に書き込むことができます。`kubectl`、`terraform`、`npm` などのサブプロセスコマンドがこれらのディレクトリ外に書き込む必要がある場合は、`sandbox.filesystem.allowWrite` を使用して特定のパスへのアクセスを許可します。

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

これらのパスは OS レベルで強制されるため、サンドボックス内で実行されるすべてのコマンド（子プロセスを含む）がそれらを尊重します。これは、`excludedCommands` でツールをサンドボックスから完全に除外するのではなく、ツールが特定の場所への書き込みアクセスを必要とする場合の推奨アプローチです。

複数の[設定スコープ](/docs/ja/settings#settings-precedence)で同じファイルシステム配列を定義する場合、Claude Code はそれらをマージし、1 つのスコープの配列を別のスコープの配列で置き換えるのではなく、すべてのスコープからのパスを組み合わせます。

CLI で [`--setting-sources`](/docs/ja/cli-reference) を使用するか、Agent SDK で [`settingSources`](/docs/ja/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) を使用してソースを除外する場合、Claude Code はサンドボックス設定を構築する際に、その `sandbox.filesystem` エントリ、`Edit` 権限ルール、および `Read` 拒否ルールを無視します。Claude Code v2.1.246 以降が必要です。

セッション中にこれらのファイルシステムリストを編集する場合、Claude Code は[実行中のセッションに変更を適用](/docs/ja/settings#when-edits-take-effect)するため、次のサンドボックス化されたコマンドは新しいパスの下で実行されます。

パスプレフィックスはパスの解決方法を制御します。

| プレフィックス           | 意味                                                             | 例                                                                      |
| :---------------- | :------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `/`               | ファイルシステムルートからの絶対パス                                             | `/tmp/build` は `/tmp/build` のままです                                      |
| `~/`              | ホームディレクトリからの相対パス                                               | `~/.kube` は `$HOME/.kube` になります                                        |
| `./` またはプレフィックスなし | プロジェクト設定の場合はプロジェクトルートからの相対パス、またはユーザー設定の場合は `~/.claude` からの相対パス | `.claude/settings.json` の `./output` は `<project-root>/output` に解決されます |

この構文は、絶対パスに `//path` を使用し、プロジェクト相対パスに `/path` を使用する[読み取りおよび編集権限ルール](/docs/ja/permissions#read-and-edit)とは異なります。サンドボックスファイルシステムパスは標準的な規則を使用します。`/tmp/build` は絶対パスです。Claude Code がこれらのパスの末尾のスラッシュまたはワイルドカードをどのように扱うかについては、[サンドボックスパスプレフィックス](/docs/ja/settings-reference#sandbox-path-prefixes)を参照してください。

`sandbox.filesystem.denyWrite` と `sandbox.filesystem.denyRead` を使用して書き込みまたは読み取りアクセスを拒否し、`sandbox.filesystem.allowRead` を使用して拒否された領域内の特定のパスを再度許可することもできます。読み取りルールが重複する場合、より狭いパスを持つルールが適用されます。

| ルール例                                                 | 結果                                                                                                                                  |
| :--------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` と `"allowRead": ["~/projects"]` | `~/projects` は読み取り可能で、ホームディレクトリの残りはブロックされたままです。より狭い許可がその拒否された領域の一部を再度開きます                                                           |
| `"allowRead": ["~/"]` と `"denyRead": ["~/.env"]`     | `~/.env` はブロックされたままで、ホームディレクトリの残りは読み取り可能です。拒否はより広い許可内に保持されるため、広い許可はシークレットを静かに再度公開することはできません                                         |
| `"allowRead": ["~/"]` と `"denyRead": ["~/**/.env"]`  | ホームディレクトリ下のすべての `.env` はブロックされたままで、残りは読み取り可能です。[ワイルドカード拒否](/docs/ja/settings-reference#sandbox-path-prefixes)は、正確なパスと同じ方法でより広い許可内に保持されます |

以下の例は、ホームディレクトリ全体からの読み取りをブロックしながら、現在のプロジェクトからの読み取りを許可します。プロジェクトの `.claude/settings.json` に配置してください。相対パス `.` はプロジェクト設定に設定が存在する場合にのみプロジェクトルートに解決されるためです。

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

同じ設定を `~/.claude/settings.json` に配置した場合、`.` は `~/.claude` に解決され、プロジェクトファイルは `denyRead` ルールによってブロックされたままになります。

サンドボックス化されたコマンドがホームディレクトリとマウントされたボリュームへの読み取りアクセスを拒否しながら、作業ディレクトリを読み取り可能に保つには、パスルールを記述する代わりに [`permissions.blockReadsOutsideWorkingDirectories`](/docs/ja/settings-reference#permissions-blockreadsoutsideworkingdirectories) を設定します。

<h3 id="disable-filesystem-isolation">
  ファイルシステム分離を無効化
</h3>

`sandbox.filesystem.disabled` を `true` に設定して、ネットワーク分離を保持しながらファイルシステム分離をスキップします。以下の例は、ネットワークドメインの許可リストを保持しながらファイルシステム分離をオフにします。

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

サンドボックスには 2 つの独立したレイヤーがあります。[ファイルシステム分離](#filesystem-isolation)はサンドボックス化されたコマンドが読み取りおよび書き込みできるパスを制御し、[ネットワーク分離](#network-isolation)はそれらが到達できるドメインを制御します。ファイルシステムレイヤーがオフの場合、サンドボックス化されたコマンドはホストファイルシステムへの無制限の読み取りおよび書き込みアクセスを取得しますが、ネットワーク出力は許可されたドメインに限定されたままです。レイヤーをオフにするのは、コマンドが書き込む内容ではなく、コマンドが接続する場所を制御するためにサンドボックスを使用する場合です。

設定はデフォルトでオフであり、サンドボックスが実行されるプラットフォーム（macOS、Linux、WSL2）に適用されます。Claude Code v2.1.216 以降が必要です。

<Warning>
  ファイルシステム分離がオフで、コマンドが自動許可される場合、サンドボックス化されたコマンドは、後続のコマンドが実行または読み取るファイル（シェルスタートアップファイル、`$PATH` 上の実行可能ファイル、`~/.claude/settings.json` など）を書き込み、次の実行時に独自のアクセスを拡大するために使用できます。`filesystem.disabled` を `true` に設定するのは、独自のアクセスをエスカレートしないと信頼できるワークロードのみです。[`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy) でネットワークドメインをロックするとリスクを狭めますが、そのロックはサンドボックス内で実行されるコマンドにのみ適用されるため、リスクは完全には除去されません。
</Warning>

<h4 id="which-settings-can-disable-it">
  どの設定がそれを無効化できるか
</h4>

ファイルシステム分離をオフにするとサンドボックス化されたコマンドが実行できることが拡大するため、Claude Code は `filesystem.disabled` をこれらの設定ソースからのみ尊重します。

* ユーザー設定、管理設定、および `--settings` CLI フラグはそれを設定できます。`.claude/settings.json` と `.claude/settings.local.json` のプロジェクト設定はできないため、チェックアウトされたプロジェクトはファイルシステム分離をオフにすることはできません。
* 管理設定が `sandbox.filesystem` をまったく設定するか、`"mode": "deny"` を持つ `sandbox.credentials.files` エントリをリストする場合、管理設定のみがキーを設定できます。これにより、管理者がデプロイしたファイルシステム制限が有効に保たれます。そのようなデプロイを緩和するには、管理設定で `"disabled": true` を設定します。
* [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ja/env-vars) が設定されている場合、Claude Code は管理設定を含むすべてのソースから `filesystem.disabled` を無視し、ファイルシステム分離をオンに保ちます。

管理された `credentials.files` エントリが `filesystem.disabled` をピンするかどうか（キーを管理設定にロックして開発者がファイルシステム分離をオフにできないようにする）は、エントリの `mode` とサンドボックスの開始時にエントリに何が起こるかによって異なります。

| 管理エントリ                                                                                         | `filesystem.disabled` をピンする | 分離がオフの場合にファイルを保護するもの                                                                   |
| ---------------------------------------------------------------------------------------------- | --------------------------- | -------------------------------------------------------------------------------------- |
| `"mode": "deny"`                                                                               | はい                          | なし。読み取りブロックはファイルシステムレイヤーの一部です                                                          |
| `"mode": "mask"`、マスクとして適用                                                                      | いいえ                         | マスキング自体。Linux と WSL2 の[センチネルコピーとプロキシ](#mask-credential-files)、macOS のサンドボックス独自の読み取りルール |
| `"mode": "mask"`、[セットアップで `deny` にフォールバック](#mask-credential-files)                             | いいえ                         | なし、`deny` と同じです。マスクできないパス（ディレクトリなど）を明示的な `deny` エントリとしてリストします。これはキーをピンします              |
| `"mode": "mask"`、[検証によって `deny` に低下](/docs/ja/managed-settings#invalid-entries-in-managed-settings) | はい、明示的な `deny` のように         | なし、`deny` と同じです                                                                        |

フォールバックはサンドボックスの開始時に発生し、Claude Code が既に設定を読み込んだ後にピンチェックが実行されるため、フォールバックされたエントリはピンしません。検証は設定の読み込み中に無効なエントリを `deny` に書き直すため、低下したエントリは `deny` として記述したものと同じようにピンします。

<h4 id="what-changes-when-filesystem-isolation-is-off">
  ファイルシステム分離がオフの場合に何が変わるか
</h4>

`filesystem.disabled` を設定すると、ファイルシステムレイヤー自体が強制する保護が解除されます。他のレイヤーが強制する保護は引き続き適用されます。

| 保護                                                                                  | ファイルシステム分離がオフの場合                                                                                            |
| ----------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `filesystem.denyRead` と [`credentials.files`](#protect-credentials) `deny` 読み取りブロック | 強制されません。ファイルシステムレイヤーは両方を適用します                                                                               |
| `credentials.envVars` `deny` と `mask` エントリ                                          | 強制されます。環境変数スクラビングはファイルシステムレイヤーから独立しています                                                                     |
| [`credentials.files` `mask` エントリ](#mask-credential-files)がマスクとして適用                  | 強制されます。マスキングはファイルシステムレイヤーから独立しています。[`deny` にフォールバック](#mask-credential-files)したエントリは強制されません。`deny` エントリと同じです |

他に 2 つのことが変わります。

* サンドボックス化されたコマンドは、セッション一時ディレクトリではなく、シェルの `$TMPDIR` を継承します。すべての一時ディレクトリは書き込み可能で、Claude Code はコマンドをセッション一時ディレクトリにリダイレクトしなくなるためです。

  Linux では、変数は親シェルでしばしば設定されていません。Bash ツールのガイダンスは、`$TMPDIR` に依存するのではなく、`mktemp -d` でスクラッチディレクトリを作成するよう Claude に指示します。
* [`autoAllowBashIfSandboxed`](/docs/ja/settings-reference#sandbox-autoallowbashifsandboxed) は引き続きデフォルトで `true` であるため、サンドボックス化されたコマンドはプロンプトなしで実行され続けます。プロンプトを表示するには `false` に設定します。

<h3 id="protect-credentials">
  認証情報を保護
</h3>

`sandbox.credentials` 設定は、サンドボックス化されたコマンドから保護するための認証情報ファイルと環境変数を宣言します。各エントリはファイルパスまたは環境変数と `mode` に名前を付けます。専用の `credentials` ブロックは、認証情報ルールを一緒にグループ化し、一般的なファイルシステムルールから分離します。

`"mode": "deny"` を持つエントリの場合、ファイルパスはサンドボックス内での読み取りに対して拒否されます。これは `filesystem.denyRead` が適用する制限と同じであり、環境変数は各サンドボックス化されたコマンドの実行前に設定解除されます。ファイル保護はファイルシステムレイヤーの一部であるため、[ファイルシステム分離を無効化](#disable-filesystem-isolation)した場合は適用されません。環境変数保護は引き続き適用されます。

以下の例は AWS 認証情報ファイルと SSH ディレクトリの読み取りをブロックし、`GITHUB_TOKEN` と `NPM_TOKEN` をサンドボックス化されたコマンドの環境から削除します。

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

環境変数エントリとファイルエントリは、[認証情報をマスク](#mask-credentials)の下で説明されている `"mode": "mask"` も受け入れます。

ファイルパスは `sandbox.filesystem.*` 設定と同じ[プレフィックスルール](/docs/ja/settings-reference#sandbox-path-prefixes)に従います。

Claude Code はセッションが読み込むすべての[設定スコープ](/docs/ja/settings#settings-precedence)から `deny` エントリをマージします。`deny` エントリはアクセスを狭めるだけなので、任意のスコープが 1 つを追加できますが、別のスコープが追加したものを削除できるスコープはありません。

[設定ソースを除外](#configure-sandboxing)する場合。

* **プロジェクトまたはローカル設定**: Claude Code はそれらの `credentials` エントリを適用しません。Claude Code v2.1.246 以降が必要です。
* **ユーザー設定**: Claude Code は `~/.claude/settings.json` の `deny` エントリを引き続き適用し、その[ファイル `mask` エントリ](#mask-credential-files)を制限として保持しますが、その[環境変数 `mask` エントリ](#mask-environment-variables)を削除します。

組み込みの認証情報拒否リストはないため、リストしたファイルと変数のみが制限されます。

`sandbox.credentials` はサンドボックス化された Bash コマンドのみに影響します。サンドボックスに関係なくすべてのサブプロセスから認証情報を削除するには、[`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ja/env-vars) を設定します。

<h3 id="mask-credentials">
  認証情報をマスク
</h3>

マスキングは、[認証情報を保護](#protect-credentials)の下の `deny` エントリよりもさらに進みます。認証情報をブロックする代わりに、Claude Code はサンドボックス化されたコマンドにプレースホルダーであるセンチネルを表示し、[サンドボックスプロキシ](#network-isolation)は許可したホストへのアウトバウンドリクエストで実際の値を交換します。ファイルの場合、置換は Linux と WSL2 の動作です。[macOS はファイルをブロック](#mask-credential-files)します。

<h4 id="mask-environment-variables">
  環境変数をマスク
</h4>

`"mode": "mask"` は認証情報を保護しながら、それで認証するツールが機能し続けるようにします。`deny` は変数を完全に削除し、これは `gh` や `npm` などの必要なツールも破壊します。Claude Code v2.1.199 以降が必要です。

`mask` を使用すると、サンドボックス化されたコマンドは実際の値の代わりにセッションごとのセンチネル値を見ます。各 `mask` エントリは `injectHosts` をリストでき、実際の値が到達できるホストです。リクエストがそれらの 1 つに対してサンドボックスを離れるとき、[サンドボックスプロキシ](#network-isolation)はセンチネルを実際の値に置き換えます。コマンドとそれがログに記録するものは実際の認証情報を保持しませんが、そのリクエストは引き続き認証されます。

プロキシはリクエストコンテンツ内の認証情報を置き換えるため、それらを見る必要があります。[`network.tlsTerminate`](/docs/ja/settings-reference#sandbox-network-tlsterminate) を設定して、プロキシが TLS 自体を終了するようにします。

それなしでは、マスキングは何も公開せずに失敗します。コマンドはセンチネルのみを見ますが、センチネルはサーバーに変更されずに到達し、認証は失敗します。Claude Code はこの設定ミスをスタートアップで報告します。

置換はヘッダーとリクエストボディをカバーします。認証情報自体ではなく、認証情報から派生した署名で認証するリクエストは、プロキシで再署名する必要があります。[AWS リクエストに再署名](#re-sign-aws-requests)は AWS でそれがどのように機能するかをカバーします。

プロキシは[ドメイン許可リスト](#network-isolation)が許可する接続にのみ注入するため、各 `injectHosts` 宛先は `network.allowedDomains` を通じても到達可能である必要があります。

以下の例は 2 つのトークンをマスクします。`GH_TOKEN` は `api.github.com` へのリクエストにのみ置き換えられ、`NPM_TOKEN` は `injectHosts` を持たず、`network.allowedDomains` のすべてのホストへのリクエストで置き換えられます。

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

<span id="ipv6-destinations-in-injecthosts" />IPv6 宛先は 2 つのリストで異なるスペルを使用してください。各リストは独自のマッチャーを持つためです。

* **`network.allowedDomains`**: [ドメインリストが使用する括弧形式](#ipv6-addresses-in-domain-lists)（`"[::1]"` など）。プロキシはこのリストをチェックして接続を許可します。
* **`injectHosts`**: 正規の圧縮形式での裸のアドレス（`"::1"` または `"2001:db8::1"` など）。プロキシは各エントリをコネクションの裸の宛先アドレスと照合し、ポートを無視するため、括弧で囲まれた、ゾーン ID、または異なる圧縮スペルは一致せず、プロキシはそこで認証情報を注入しません。

`claude doctor` は、`injectHosts` エントリが宛先と一致できないことを警告 `Sandbox credential injectHosts entries can never match their destination` でフラグします。このチェックには Claude Code v2.1.229 以降が必要です。

`deny` とは異なり、マスキングはプロキシに実際の認証情報をリストされたホストに送信することを認可するため、Claude Code はユーザーまたは管理者が制御する設定からのみそれを尊重します。ユーザー設定、管理設定、および `--settings` CLI フラグです。Claude Code はリポジトリの `.claude/settings.json` または `.claude/settings.local.json` の `mask` エントリを無視します。これらのファイルでは、`network.tlsTerminate` と [`credentials.allowPlaintextInject`](/docs/ja/settings-reference#sandbox-credentials-allowplaintextinject)（プロキシが暗号化されていないリクエストに認証情報を注入できるようにする設定）も無視します。[ユーザー設定を除外](#configure-sandboxing)する場合、Claude Code は `~/.claude/settings.json` の環境変数 `mask` エントリも削除します。

管理者が `mask` エントリ、`network.tlsTerminate`、または `credentials.allowPlaintextInject` をサーバー管理設定を通じて配信する場合、それらは[承認が必要な設定](/docs/ja/server-managed-settings#security-approval-dialogs)としてカウントされます。

同じ変数が任意のスコープで `deny` でリストされている場合、`deny` が優先されます。

マスキングはデフォルトで変数の全体値を置き換えます。これは裸のトークンに適しています。オプションのエントリフィールド（Claude Code v2.1.224 以降が必要）は、構造を持つ値を処理します。

* `extract`: Claude Code が値全体に適用する正規表現。各マッチの 1 グループでキャプチャされたテキストのみを置き換えるため、値を解析するツール（`DATABASE_URL` 接続文字列など）はサンドボックス内で引き続き機能します。パターンは少なくとも 1 つのキャプチャグループを含む必要があります。
* `onExtractNoMatch` はパターンが何もマッチしない場合に何が起こるかを制御します。
  * `warn`（デフォルト）は警告を発し、変数を未マスク状態で渡します
  * `deny` はサンドボックス内で変数を設定解除します
  * `error` は設定を修正するまでサンドボックスセットアップを停止します
* `decode: "jwt"`: JSON Web Token（JWT）を保持する変数の場合。Claude Code は値が JWT であることを検証し、それを構造的に有効な偽のトークンに置き換えるため、サンドボックス内のトークンをデコードするコードは機能し続けます。`maskClaims` を追加して、トークン全体を置き換える代わりに個別にマスクするトップレベルペイロードクレームをリストします。他のクレームは読み取り可能なままです。値が JWT として検証されない場合、またはリストされたクレームが一致しない場合、Claude Code は警告を伴って変数を未マスク状態で渡します。`decode` は `extract` と組み合わせることはできません。

設定リファレンスの [`credentials.envVars[]` 行](/docs/ja/settings-reference#sandbox-settings)で完全なフィールドリストを参照してください。

<h4 id="re-sign-aws-requests">
  AWS リクエストに再署名
</h4>

AWS リクエストはリクエストコンテンツ上に SigV4 署名を持つため、`AWS_ACCESS_KEY_ID` と `AWS_SECRET_ACCESS_KEY` を一緒にマスクします。プロキシはアクセスキーのセンチネルで SigV4 リクエストを検出し、実際の値を置き換えた後に再署名します。シークレットのみをマスクすると、プレースホルダーで署名されたリクエストが残り、プロキシが検出できないため、AWS で失敗します。Claude Code はこのケースをスタートアップで警告しますが、アクセスキー ID のみがマスクされている場合は警告しません。プロキシが再署名できない検出されたリクエスト（`x-amz-date` ヘッダーが欠落しているなど）は、壊れた署名でサーバーに到達する代わりに、プロキシエラーで失敗します。

Claude Code は、全体値をマスクする場合、従来の `AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY`、および `AWS_SESSION_TOKEN` 変数を 1 つの認証情報に自動的にリンクします。AWS 認証情報が他の名前の変数に存在する場合、[`credentials.awsPairs`](/docs/ja/settings-reference#sandbox-credentials-awspairs) で自分でグループ化します。これには Claude Code v2.1.224 以降が必要です。この例は、既に `MY_KEY_ID`、`MY_SECRET_KEY`、および `MY_SESSION_TOKEN` を全体値としてマスクする設定にペアリングを追加します。上記の[マスキング設定](#mask-environment-variables)のように。

```json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

各エントリは以下のルールに従います。

* `accessKeyIdVar` と `secretAccessKeyVar` はアクセスキー ID とシークレットキーを保持するマスク済み `envVars` エントリに名前を付けます。オプションの `sessionTokenVar` は一時認証情報のセッショントークンを保持するエントリに名前を付けます。設定されている場合、プロキシは再署名されたリクエストで実際のトークンを `x-amz-security-token` として送信します。
* 名前付きの各変数は、`extract` または `decode` なしで全体値をマスクする `mask` エントリである必要があります。
* プロキシはアクセスキー ID エントリの `injectHosts` にリストされたホストでリクエストに再署名します。
* ペアで従来の変数のいずれかに名前を付けると、自動ペアリングが置き換えられます。

`mask` エントリと同様に、`awsPairs` はユーザー設定、管理設定、および `--settings` CLI フラグからのみ尊重されます。

3 つの AWS リクエスト形式は、プロキシが再計算できない署名を持ちます。そのようなリクエストがマスク済みペアのプレースホルダーで署名されている場合、プロキシは壊れた署名を転送するのではなく失敗します。未マスク認証情報で署名されたリクエストは影響を受けません。[`credentials.sigv4`](/docs/ja/settings-reference#sandbox-credentials-sigv4) 設定（Claude Code v2.1.224 以降が必要）は、フォームごとにこれを緩和します。フォームのキーを `passthrough` に設定すると、リクエストはプレースホルダー派生署名で転送されるため、呼び出しツールは AWS 独自の拒否応答を受け取る代わりにプロキシエラーを受け取ります。`awsPairs` と同様に、`sigv4` はユーザー設定、管理設定、および `--settings` CLI フラグからのみ尊重されます。

| リクエスト形式                   | `sigv4` キー  | プロキシが再署名できない理由                             |
| :------------------------ | :---------- | :----------------------------------------- |
| aws-chunked ストリーミングアップロード | `streaming` | チャックごとの署名はシード署名から連鎖するため、再署名にはボディの書き直しが必要です |
| 署名済み URL                  | `presigned` | 署名は URL 自体に存在し、`Authorization` ヘッダーはありません  |
| SigV4A 非対称署名              | `sigv4a`    | 再計算する共有キー HMAC がありません                      |

<h4 id="mask-credential-files">
  認証情報ファイルをマスク
</h4>

ファイルエントリも `"mode": "mask"` を受け入れます。これには Claude Code v2.1.221 以降が必要です。サンドボックス化されたコマンドが見るものはプラットフォームに依存します。

* **Linux と WSL2**: サンドボックス化されたコマンドはファイルのセンチネルコピーを読み取ります。シークレットがプレースホルダー値に置き換えられたスタンドイン。[サンドボックスプロキシ](#network-isolation)は出力時に実際の値を置き換えます。
* **macOS**: サンドボックス化されたコマンドはリストされたファイルを読み取ることができません。Claude Code はセンチネルコピーを構築せず、出力時に何も置き換えないため、ファイルで認証するツールはサンドボックス内で機能しません。`deny` と同じ効果です。`deny` エントリとは異なり、[ファイルシステム分離を無効化](#disable-filesystem-isolation)した場合でも読み取りブロックは保持されます。

すべてのプラットフォームで、Claude Code は [`network.tlsTerminate`](/docs/ja/settings-reference#sandbox-network-tlsterminate) 要件と `injectHosts` を[マスク済み環境変数](#mask-environment-variables)と同じ方法で適用し、リポジトリ設定を同じ方法で無視します。[ユーザー設定を除外](#configure-sandboxing)する場合、Claude Code は `~/.claude/settings.json` のファイル `mask` エントリを制限として保持しますが、エントリはプロキシが実際の値を置き換えることをもはや認可しません。

以下の例は `~/.config/gh/hosts.yml` に保存された GitHub トークンをマスクします。以下で説明される `extract` パターンは、Claude Code にファイルのどの部分がシークレットであるかを伝えます。Linux と WSL2 では、ファイルを読み取るサンドボックス化されたコマンドはトークンの代わりにセンチネルを取得し、プロキシは `api.github.com` へのリクエストで実際のトークンを置き換えます。

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

マスクがアクティブであることを確認するには、Claude にサンドボックス化されたコマンドで `cat ~/.config/gh/hosts.yml` を実行するよう依頼します。Linux と WSL2 では出力はトークンの代わりにセンチネル値を表示し、macOS では読み取りが失敗します。

Linux と WSL2 では、`extract` パターンは `hosts.yml` の残りを読み取り可能に保つものです。Claude Code は正規表現を全体ファイルに適用し、各マッチの 1 グループでキャプチャされたテキストのみを置き換えるため、`gh` は引き続き設定を解析し、トークンのみがプレースホルダーです。`extract` を `.netrc`、JSON、YAML などのツールが解析する構造化ファイルに使用します。パターンは少なくとも 1 つのキャプチャグループを含む必要があります。`extract` なしでは、Claude Code はファイル全体のコンテンツを 1 つのセンチネル値に置き換えます。これは、単一の裸のシークレットを保持し、他に何もないファイルに適しています。

JWT を保持するファイルの場合、`extract` の代わりに、または一緒に `decode: "jwt"` を設定します。`decode` には Claude Code v2.1.224 以降が必要です。Claude Code は組み込みパターンで JWT 候補を見つけるか、設定されている場合は `extract` パターンで見つけ、各候補が JWT であることを検証し、それを構造的に有効な偽のトークンに置き換えるため、サンドボックス内でトークンをデコードするコードは機能し続けます。`maskClaims` を追加して、トークン全体を置き換える代わりに、各検証済みトークン内の名前付きトップレベルペイロードクレームのみをマスクし、他のクレームは読み取り可能なままにします。候補が検証されない場合、またはリストされたクレームが一致しない場合、以下の `onExtractNoMatch` フィールドが結果を管理します。これは、パターンが何もマッチしない場合と同じです。

2 つのオプションフィールドはマッチング動作を改善します。両方は `mode` が `mask` で `extract` または `decode` が設定されている場合にのみ適用されます。macOS では、ファイルシステム分離がオンの場合、Claude Code は `mask` エントリを `deny` として適用します。パターンが実行される前です。これらのフィールドと以下の不一致の結果は、[ファイルシステム分離を無効化](#disable-filesystem-isolation)した場合にのみ macOS で有効になります。

* `onExtractNoMatch` はマッチングがファイル内でマスクするものを見つけない場合に何が起こるかを制御します。

  * `warn`（デフォルト）は警告を発し、エントリをスキップするため、サンドボックス化されたコマンドは実際のファイルを未マスク状態で読み取ることができます。デフォルトは認証情報が合法的に存在しない可能性があるケースに適しています。シークレットが存在する可能性があるがパターンが見落とす可能性がある場合は、`deny` を使用します
  * `deny` はファイルを読み取り不可にします
  * `error` は設定を修正するまでサンドボックスセットアップを停止します

  Claude Code は、読み取りブロックが強制されない場合は常に `deny` を `error` として扱います。[ファイルシステム分離を無効化](#disable-filesystem-isolation)した場合、および任意の設定ソースからの `filesystem.allowRead` エントリがファイルのパスを再度開く場合です。
* `maskDuplicates` は、マッチした各マスク済み認証情報値（`extract` キャプチャまたは `decode` 検証済みトークン）の逐語的コピーも置き換えます。マッチした範囲外で見つかった場合、シークレットが繰り返される場所です。生の部分文字列と照合するため、短いまたは一般的な値はどこでも置き換えられます。長く、高エントロピーのシークレット用に予約します。デフォルト: false。

`mask` は単一のファイルに適用されるため、各認証情報ファイルを個別にリストします。Claude Code は `mask` エントリが安全にマスクできない場合にフォールバックして `deny` します。ディレクトリパス、グロブパターン、8 MiB より大きいファイル、または UTF-8 テキストではないファイル。代わりにディレクトリを明示的な `deny` エントリとして記述します。[どの設定がそれを無効化できるか](#which-settings-can-disable-it)の下の表は、各フォームが `filesystem.disabled` をピンするかどうか、およびファイルシステム分離がオフの場合の動作をカバーします。

<h2 id="how-sandboxing-works">
  サンドボックス化の仕組み
</h2>

<h3 id="filesystem-isolation">
  ファイルシステム分離
</h3>

サンドボックス化された Bash ツールはファイルシステムアクセスを特定のディレクトリに制限します。

* **デフォルトの書き込み動作**：現在の作業ディレクトリとそのサブディレクトリへの読み取りおよび書き込みアクセス、`--add-dir`、`/add-dir`、または [`permissions.additionalDirectories`](/docs/ja/settings-reference#permissions-additionaldirectories) で追加したディレクトリ、加えて `$TMPDIR` が指すセッション一時ディレクトリへのアクセス
* **デフォルトの読み取り動作**：特定の拒否ディレクトリを除く、コンピュータ全体への読み取りアクセス。このデフォルトは `~/.aws/credentials` や `~/.ssh/` などの認証情報ファイルの読み取りを許可することに注意してください。[`sandbox.credentials`](#protect-credentials) を使用してこれらのファイルの読み取りをブロックし、シークレット環境変数の設定を解除するか、パスを `denyRead` に追加してください。
* **ブロックされたアクセス**：明示的な許可なしに作業ディレクトリ、追加されたディレクトリ、およびセッション一時ディレクトリ外のファイルを変更できません。これには `~/.bashrc` などのシェル設定ファイルと `/bin/` のシステムバイナリが含まれます。
* **Git worktrees**：作業ディレクトリが[リンクされた git worktree](/docs/ja/worktrees)の場合、サンドボックスはメインリポジトリの共有 `.git` ディレクトリへの書き込みも許可するため、`git commit` などのコマンドが refs とインデックスを更新できます。そのディレクトリ内の `hooks/` と `config` への書き込みは引き続き拒否されます。
* **設定可能**：設定を通じてカスタム許可パスと拒否パスを定義します

ファイルシステム分離をスキップして、ネットワーク分離を維持するには、[`sandbox.filesystem.disabled`](#disable-filesystem-isolation) を設定します。

<h3 id="protected-paths">
  保護されたパス
</h3>

サンドボックス化されたコマンドが書き込みできるディレクトリ内でも、サンドボックスは Claude Code が設定とコードを読み込むファイルへの書き込みを拒否します。これらのファイルを編集できるコマンドは、自身に権限を付与したり、Claude Code がサンドボックス外で実行する hook または MCP サーバーを追加したりする可能性があります。権限システムには独自の[保護されたパス](/docs/ja/permission-modes#protected-paths)があり、ツールが実行される前に Claude Code が承認する内容を制御します。サンドボックスのリストは既に実行中のコマンドに適用されます。4 つのパスグループをカバーしています。

* **作業ディレクトリおよびその上のディレクトリ内**：`.claude` 設定ファイル、`.claude/skills`、`.claude/agents`、`.claude/commands`、`.claude/hooks` ディレクトリ、`.mcp.json`、および Claude Code が独自に実行するファイル（`.claude/workflows` や `.claude/scheduled_tasks.json` など）
* **作業ディレクトリのみ**：`.bashrc` や `.zshrc` などのシェルスタートアップファイル、`.gitconfig`、`.vscode` および `.idea` ディレクトリ、`.git` 内の `hooks` および `config`
* **作業ディレクトリをベア git リポジトリに変えるファイル**：トップレベルの `HEAD`、`objects`、`refs`、加えて `config` と `hooks`（`HEAD` が隣に存在する場合）。Linux および WSL2 では、サンドボックス化されたコマンドの実行中に表示されるトップレベルの `HEAD` ファイルまたは `objects` または `refs` ディレクトリをサンドボックスが削除します。
* **`~/.claude` または `CLAUDE_CONFIG_DIR` が指すディレクトリ内**：そのほとんどのコンテンツ、加えて `~/.claude.json` および `.credentials.json` 認証情報ストア

セッション中に保護された設定ファイルのパスにシンボリックリンクが表示される場合、サンドボックスは次のコマンドから、それが指すファイルへの書き込みも拒否します。

これらのパスの 1 つを除外する方法はありません。パスをカバーする `allowWrite` エントリまたは `Edit` 許可ルールは保護を解除しません。保護をオフにする唯一の方法は [`filesystem.disabled`](#disable-filesystem-isolation) です。これはすべてのパスのファイルシステム分離をオフにします。マシンで解決されたこれらのパスのほとんどを確認するには、`/sandbox` を実行して **Config** タブを開きます。このタブは、独自の `denyWrite` エントリと混在して、**Denied within allowed** の下にそれらをリストします。

`git merge` または `git checkout` がこれらのパスの 1 つで `unable to unlink old` で失敗する場合は、[Troubleshooting](#troubleshooting) を参照してください。

<h3 id="network-isolation">
  ネットワーク分離
</h3>

ネットワークアクセスはサンドボックス外で実行されるプロキシサーバーを通じて制御されます。

* **ドメイン制限**：Claude Code はデフォルトでドメインを事前に許可しません。コマンドが新しいドメインにアクセスする必要がある場合、Claude Code はプロンプトを表示します。[オートモード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)では、Claude は[Per-command allowed domains](#per-command-allowed-domains-in-auto-mode)に従って、コマンド自体でコマンドが必要とするホストを指定します。プロンプトで「はい」を選択すると、Claude Code は現在のセッションの残りの期間そのホストを許可し、同じホストへの後続の接続ではプロンプトを表示しません。「はい、今後は聞かない」を選択すると、Claude Code は `WebFetch(domain:...)` 許可ルールを[ローカル設定](/docs/ja/permissions#permission-system)に保存するため、そのホストは今後のセッションで許可されたままになります。
* **事前許可ドメイン**：[`allowedDomains`](/docs/ja/settings-reference#sandbox-network-alloweddomains) でドメインを事前に許可してプロンプトを完全に回避します。Claude Code は、[Permission rules](#permission-rules) で説明されているように、`WebFetch(domain:...)` 許可ルールからのドメインも事前に許可します。
* **厳密な許可リスト**：ユーザー、管理、または CLI `--settings` 設定で [`strictAllowlist`](/docs/ja/settings-reference#sandbox-network-strictallowlist) を `true` に設定した場合、Claude Code はプロンプトの代わりに、許可リスト外のホストへのサンドボックス化されたコマンドアクセスを拒否します。許可リストは、サンドボックスが他の方法でプロンプトを表示するのと同じものです。`allowedDomains` に加えて `WebFetch(domain:...)` 許可ルールからのドメイン、または `allowManagedDomainsOnly` が設定されている場合は管理設定エントリのみです。Claude Code はサンドボックス化されたコマンドのみにこれを実施します。`WebFetch` などのインプロセスツールは引き続き[権限ルール](#permission-rules)に従います。リポジトリの `.claude/settings.json` または `.claude/settings.local.json` で設定しても効果はありません。Claude Code v2.1.219 以降が必要です。
* **管理ロックダウン**：[`allowManagedDomainsOnly`](/docs/ja/settings-reference#sandbox-network-allowmanageddomainsonly) が管理設定で設定されている場合、許可されていないドメインはプロンプトの代わりに自動的にブロックされ、管理設定からの `allowedDomains` および `WebFetch(domain:...)` 許可ルールのみが尊重されます。
* **企業プロキシ**：ネットワークが発信トラフィックを企業プロキシを通じて送信する必要がある場合、[プロキシ設定](/docs/ja/network-config#proxy-configuration)で説明されているように、`HTTPS_PROXY`、`HTTP_PROXY`、`NO_PROXY` を設定の `env` ブロックで設定して、[バックグラウンドエージェント](/docs/ja/network-config#set-network-variables-in-settings-not-the-shell)もそれらを取得するようにするか、Claude Code を起動する環境で設定します。Claude Code はドメイン許可リストを実施してから、許可されたコネクションをそのアップストリームプロキシを通じてトンネルします。
* **カスタムプロキシサポート**：高度なユーザーは発信トラフィックにカスタムルールを実装できます
* **包括的なカバレッジ**：制限はすべてのスクリプト、プログラム、およびコマンドによって生成されるサブプロセスに適用されます

`WebFetch(domain:...)` ルールでは、サンドボックスは 2 つのワイルドカード形式を尊重します。`*.example.com` などの先頭の `*.` とベアの `*`。ベアの `*` 形式には Claude Code v2.1.186 以降が必要です。`WebFetch(domain:example.*)` などの他の位置のワイルドカードはフェッチに一致しますが、サンドボックス化されたコマンドには効果がありません。

<Note>
  組み込みプロキシは要求されたホスト名に基づいて許可リストを実施し、デフォルトでは TLS トラフィックを終了または検査しません。Claude Code v2.1.199 以降で利用可能な実験的な [`network.tlsTerminate`](/docs/ja/settings-reference#sandbox-network-tlsterminate) 設定により、組み込みプロキシ自体が TLS を終了するようになります。[`mask` 認証情報エントリ](#mask-credentials)にはこの動作が必要です。デフォルトの影響については [Security limitations](#security-limitations) を参照してください。脅威モデルが TLS 検査を必要とする場合は、[Custom proxy configuration](#custom-proxy-configuration) を参照してください。
</Note>

<h4 id="per-command-allowed-domains-in-auto-mode">
  オートモードでのコマンドごとの許可ドメイン
</h4>

[オートモード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)でサンドボックスがオンの場合、Claude はネットワーク承認をトリガーする代わりに、コマンド自体でコマンドが必要とするホストを指定します。サンドボックスで実行される各 Bash、PowerShell、または [Monitor](/docs/ja/tools-reference#monitor-tool) コマンドは、サンドボックスの許可リストを超えたホストのリストを持つことができます。`registry.npmjs.org` などのドメイン、`*.pythonhosted.org` などのワイルドカード、または IP アドレス（各々にオプションの `:port` 付き）。分類器はホストとコマンドを一緒にレビューします。Claude Code v2.1.271 以降が必要です。

承認されたリストはそのコマンドのみに対してそれが実行されている限りそれらのホストを開きます。セッションの許可されたホストまたは設定に何も追加されません。次のコマンドは独自のホストを指定します。

ホストを持つコマンドは、権限ルールまたはサンドボックスの[オートアロー モード](#sandbox-modes)によって承認される代わりに、分類器に送られます。[ask ルール](/docs/ja/permissions#manage-permissions)がコマンドのプロンプトを強制する場合、ターミナルの権限ダイアログはそれの隣にホストをリストし、そこで承認することは両方をカバーします。

コマンドごとのリストはサンドボックスがデフォルトで拒否するものだけを広げます。[`deniedDomains`](/docs/ja/settings-reference#sandbox-network-denieddomains) エントリはまだブロックします。[`strictAllowlist`](/docs/ja/settings-reference#sandbox-network-strictallowlist) または [`allowManagedDomainsOnly`](/docs/ja/settings-reference#sandbox-network-allowmanageddomainsonly) が許可リストをロックする場合、Claude Code はコマンドごとのリストを拒否します。

コマンドごとのリストが適用されている間、Claude Code は承認されたコマンドがリストしなかったホストへの接続をプロンプトなしで分類器チェックなしで拒否します。拒否はコマンドの結果でホストを指定し、Claude はホストが追加されたコマンドを再実行します。

<h4 id="ipv6-addresses-in-domain-lists">
  ドメインリスト内の IPv6 アドレス
</h4>

サンドボックスのドメインリストは `allowedDomains`、`deniedDomains`、およびそれらに供給する `WebFetch(domain:...)` ルールです。それらのいずれかで IPv6 アドレスに一致させるには、リテラルを括弧で囲んで記述します。`"[::1]"` はすべてのポートでそのアドレスに一致し、`"[::1]:443"` はポート 443 でのみ一致します。ポートを 1 から 65535 の数値として先頭のゼロなしで記述します。括弧で囲まれた形式には Claude Code v2.1.229 以降が必要です。v2.1.229 より前では、括弧で囲まれていないエントリの最後のコロンの後のテキストがポート番号の場合、Claude Code はそれを 1 つとして読み取ったため、`::1:443` はポート 443 でアドレス `::1` を指定しました。

IPv6 アドレスのネットワーク承認プロンプトで「はい、今後は聞かない」を選択すると、Claude Code は `WebFetch(domain:...)` ルールをアドレスを括弧で囲んで保存するため、ルールは今後のセッションでアドレスに一致し続けます。

括弧で囲まれていないエントリで 2 つ以上のコロンがある場合は曖昧です。`::1:443` は完全な IPv6 アドレスとアドレスの後にポートの両方です。Claude Code は、どの読み取りを意図したかを推測する代わりに、曖昧なスペルを保守的に実施します。

* **拒否リスト**：Claude Code はエントリが解析するすべての読み取りを拒否するため、意図した読み取りがブロックされます。解析可能な読み取りがないエントリの場合、Claude Code は何もブロックしません。
* **許可リスト**：Claude Code は記述したもの以上を許可しません。その読み取りが正常に解析される場合、曖昧なエントリをホストとポートの読み取りに書き直し、許可リストを拡大するのではなく、エントリを完全にドロップする可能性があります。

ターミナルで `claude doctor` を実行して、影響を受けるエントリを見つけます。`Sandbox network domain entries have unreliable spellings` 警告は最大 3 つを指定し、残りをカウントします。括弧で囲まれた形式で各エントリを書き直して、警告をクリアします。警告は、`@`、パスまたはクエリ文字、括弧内のワイルドカードなど、他の理由でスペルが信頼できないエントリも指定します。

<h3 id="os-level-enforcement">
  OS レベルの実施
</h3>

サンドボックス化された Bash ツールはオペレーティングシステムセキュリティプリミティブを使用します。

* **macOS**：サンドボックス実施に Seatbelt を使用します
* **Linux**：分離に [bubblewrap](https://github.com/containers/bubblewrap) を使用します
* **WSL2**：Linux と同じく bubblewrap を使用します

WSL1 は bubblewrap が WSL2 でのみ利用可能なカーネル機能を必要とするため、サポートされていません。

これらの同じプリミティブは、スタンドアロン [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) パッケージとして利用可能です。[Sandbox environments](/docs/ja/sandbox-environments#sandbox-runtime) ページでは、Claude Code プロセス全体をラップするための別のアプローチとしてこれについて説明しています。

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  サンドボックスが権限と権限モードにどのように関連するか
</h2>

サンドボックス、[権限ルール](/docs/ja/permissions)、および[権限モード](/docs/ja/permission-modes)は補完的なレイヤーです。以下のセクションでは、サンドボックスが各レイヤーとどのように相互作用するかについて説明します。

<h3 id="permission-rules">
  権限ルール
</h3>

権限ルールとサンドボックスは異なるものを制御します。

* **権限ルール**は Claude Code が使用できるツールを制御し、ツールが実行される前に評価されます。Bash、Read、Edit、WebFetch、MCP、およびその他のツールを含むすべてのツールに適用されます。ただし、deny ルールまたは ask ルールは、他のツールが残っている間は[`EndConversation`](/docs/ja/tools-reference#endconversation-tool-behavior)をブロックできません。
* **サンドボックス**は OS レベルの強制を提供し、シェルコマンドがファイルシステムおよびネットワークレベルでアクセスできるものを制限します。Bash、PowerShell、および[Monitor](/docs/ja/tools-reference#monitor-tool)コマンドとその子プロセスにのみ適用されます。

この 2 つのレイヤーは、強制方法も異なります。Claude Code は、コマンド文字列に基づいて、またはオートモードでは別の分類器がコマンドが安全かどうかについての判断に基づいて、コマンドが実行される前に権限の決定を評価します。オペレーティングシステムは、実行中のプロセスにサンドボックス境界を強制するため、モデルが実行することを選択したものに関係なく、また許可されたコマンドがその名前が示唆するもの以上のことを行う場合でも、それが保持されます。

ファイルシステムおよびネットワーク制限は、サンドボックス設定と権限ルールの両方を通じて構成されます。

| 設定またはルール                                                         | 機能                                                           |
| :--------------------------------------------------------------- | :----------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                  | 作業ディレクトリ外のパスへのサブプロセス書き込みアクセスを許可します                           |
| `sandbox.filesystem.denyWrite` および `sandbox.filesystem.denyRead` | 特定のパスへのサブプロセスアクセスをブロックします                                    |
| `sandbox.filesystem.allowRead`                                   | `denyRead` 領域内の特定のパスの読み取りを再度許可します                            |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation)   | ネットワーク分離を維持しながら、ファイルシステムレイヤーを完全にオフにします                       |
| `Edit` allow ルール                                                 | `sandbox.filesystem.allowWrite` と同じ方法で、特定のパスへの書き込みアクセスを許可します |
| `Read` および `Edit` deny ルール                                       | 特定のファイルまたはディレクトリへのアクセスをブロックします                               |
| `WebFetch(domain:...)` allow および deny ルール                        | ドメインアクセスを制御します                                               |
| サンドボックス `allowedDomains`                                         | Bash コマンドが到達できるドメインを制御します                                    |
| サンドボックス `deniedDomains`                                          | より広い `allowedDomains` ワイルドカードが許可する場合でも、特定のドメインをブロックします       |

サンドボックス設定と権限ルールの両方からのパスとドメインは、最終的なサンドボックス構成にマージされます。

[claude-code リポジトリの examples ディレクトリ](https://github.com/anthropics/claude-code/tree/main/examples/settings)には、サンドボックス固有の例を含む、一般的なデプロイメントシナリオ用のスターター設定構成が含まれています。これらを出発点として使用し、ニーズに合わせて調整してください。

<h3 id="permission-modes">
  権限モード
</h3>

`/sandbox` は[権限モード](/docs/ja/permission-modes)ではありません。権限モードは、ツール呼び出しが実行されるかどうか、および最初にプロンプトが表示されるかどうかを決定しますが、サンドボックスは Bash コマンドが実行されたら何にアクセスできるかを制限します。制御対象と、アクション単位のプロンプトに代わるものが異なります。

|                                                                 | 制御対象                       | プロンプトに代わるもの                                                                                                                                       |
| :-------------------------------------------------------------- | :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/sandbox`                                                      | Bash コマンドが実行されたら何にアクセスできるか | [オートアロー モード](#sandbox-modes)のサンドボックス境界自体                                                                                                          |
| [オートモード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) | 各ツール呼び出しが実行されるかどうか         | アクションをレビューする分類器                                                                                                                                   |
| `--dangerously-skip-permissions`                                | 各ツール呼び出しが実行されるかどうか         | なし。[保護されたパス](/docs/ja/permission-modes#protected-paths)チェックもスキップされます。[モードが自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves)は引き続き適用されます |

サンドボックスの[オートアロー モード](#sandbox-modes)は[オートモード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)とは別です。オートアロー はサンドボックス境界がそれらを含むため Bash コマンドを承認し、オートモードはアクションをレビューするために分類器を使用します。この 2 つは独立して機能し、[サンドボックス モード](#sandbox-modes)の下にリストされている例外を除いて組み合わせることができます。無人実行の分離境界を選択するには、[サンドボックス環境](/docs/ja/sandbox-environments#how-isolation-relates-to-permission-modes)を参照してください。各フラグを開始する一般的な権限モードとサンドボックスペアリングのテーブルについては、[一般的なセットアップ](/docs/ja/permission-modes#common-setups)を参照してください。

<h2 id="configure-the-sandbox-for-your-organization">
  組織のサンドボックスを設定する
</h2>

管理者はすべてのユーザーにサンドボックス化を要求し、開発者がポリシーを広げるのを防ぎ、サンドボックストラフィックを企業プロキシを通じてルーティングできます。

<h3 id="enforce-sandboxing-with-managed-settings">
  管理設定でサンドボックス化を実施する
</h3>

すべての開発者にサンドボックスを要求するには、[管理設定](/docs/ja/managed-settings#delivery-mechanisms)を通じて `sandbox` キーを配信します。MDM で管理されるファイルまたは Claude.ai の [server-managed settings](/docs/ja/server-managed-settings)を通じて配信します。

以下の管理設定構成はサンドボックスを有効化し、サンドボックスが初期化できない場合は Claude Code の起動を拒否し、モデルがサンドボックス外でコマンドを再試行するのを防止します。

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

`enabled` を超える 2 つのキーは、サンドボックスがコマンドを実行できない場合に何が起こるかを制御します。

* **`failIfUnavailable`**：Linux の bubblewrap などの不足している依存関係は、警告を表示してサンドボックス化されていない実行にフォールバックするのではなく、Claude Code の起動をブロックします
* **`allowUnsandboxedCommands: false`**：Claude Code は `dangerouslyDisableSandbox` エスケープハッチを無視するため、サンドボックス内で失敗するコマンドはサンドボックス外で再試行できません

それらと一緒に検討する価値のある 2 つの追加があります。サンドボックス化なしで実行する必要がある組織承認ツールについて `excludedCommands` を追加します。`~/.aws` や `~/.ssh` などの認証情報ディレクトリについて [`sandbox.credentials`](#protect-credentials) エントリを追加します。また、秘密環境変数についても追加します。デフォルトの読み取りポリシーはこれらを許可します。

このサンドボックス構成は Claude が実行するコマンドをサンドボックス化します。開発者は依然として [`!` シェルモードプロンプト](/docs/ja/interactive-mode#shell-mode-with-prefix)でコマンドを入力し、Claude Code の外の任意のターミナルで既に持っているのと同じアクセス権でサンドボックス外で実行できます。入力されたコマンドがサンドボックス化される場合のセッションについては、[サンドボックス化されていない再試行エスケープハッチ](#the-unsandboxed-retry-escape-hatch)を参照してください。

サンドボックスはネイティブ Windows では実行されないため、フリートに Windows ホストが含まれている場合、この設定を macOS と Linux にスコープするか、それらのユーザーに WSL2 またはコンテナ内で Claude Code を実行させてください。

<h3 id="keep-developers-from-widening-the-policy">
  開発者がポリシーを広げるのを防ぐ
</h3>

`enabled` と `failIfUnavailable` などのブール値キーの場合、Claude Code は管理値を使用し、開発者がローカルで設定したものを無視します。`excludedCommands` と `allowRead` などの配列キーの場合、Claude Code はセッションが読み込むすべてのスコープからエントリをマージするため、開発者はポリシーを広げるエントリを追加できます。

管理設定で `allowManagedReadPathsOnly` を `true` に設定して、管理設定からの `allowRead` エントリのみが尊重されるようにします。これにより、開発者は組織承認パスを超えて読み取りアクセスを広げるのを防止します。ネットワークドメインを同じ方法で管理値にロックするには、[`allowManagedDomainsOnly`](/docs/ja/settings-reference#sandbox-network-allowmanageddomainsonly)を設定します。

管理設定が `sandbox.filesystem` を設定するか、`"mode": "deny"` を含む `sandbox.credentials.files` エントリをリストする場合、管理設定のみが [`filesystem.disabled`](#disable-filesystem-isolation)を設定できるため、開発者は管理者がデプロイしたファイルシステム制限をオフにすることはできません。`mask` エントリがキーをピンするかどうかは、それがどのように解決されるかに依存します。[どの設定がそれを無効にできるか](#which-settings-can-disable-it)の下の表は 4 つのケースをカバーしています。

`excludedCommands` には同等の管理のみロックダウンがないため、開発者は常にサンドボックス外で実行する追加コマンドを追加するエントリを追加できます。管理リストを狭く保ちます。

<h3 id="custom-proxy-configuration">
  カスタムプロキシ設定
</h3>

高度なネットワークセキュリティを必要とする組織の場合、カスタムプロキシを実装して以下を行うことができます。

* HTTPS トラフィックを復号化して検査する
* カスタムフィルタリングルールを適用する
* すべてのネットワークリクエストをログに記録する
* 既存のセキュリティインフラストラクチャと統合する

Claude Code をプロキシにポイントするには、[サンドボックス設定](/docs/ja/settings-reference#sandbox-settings)でプロキシポートを設定します。

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

一部のコマンドはサンドボックス内で失敗しますが、サンドボックス外では機能します。以下の修正は最も一般的なケースをカバーしています。

* **コマンドがホスト許可なしエラーで失敗する**：多くの CLI ツールは特定のホストに到達する必要があります。プロンプトが表示されたときに許可を付与すると、ホストが許可リストに追加されるため、ツールは将来サンドボックス内で実行されます。
* **`jest` がハングまたは失敗する**：`watchman` はサンドボックスと互換性がありません。代わりに `jest --no-watchman` を実行してください。
* **Go ベースの CLI が macOS で TLS 検証に失敗する**：`gh`、`gcloud`、`terraform` などのツールは Seatbelt の下で TLS 検証に失敗する可能性があります。これらのツールを [`excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) にリストしてください。`httpProxyPort` を MITM プロキシとカスタム CA で使用している場合は、代わりに [`enableWeakerNetworkIsolation`](/docs/ja/settings-reference#sandbox-enableweakernetworkisolation) を `true` に設定してください。
* **`open`、`osascript`、またはブラウザベースの認証フローが macOS でエラー `-600` で失敗する**：サンドボックスはデフォルトで Apple Events をブロックします。ユーザー、管理、または CLI 設定で [`allowAppleEvents`](/docs/ja/settings-reference#sandbox-allowappleevents) を `true` に設定して、それらを許可してください。プロジェクト設定はこのキーでは無視されます。これを有効にするとコード実行の分離が削除されます。サンドボックス化されたコマンドはユーザープロンプトなしで他のアプリケーションをサンドボックス化されていない状態で起動でき、macOS オートメーション同意プロンプト（TCC）の対象となる実行中のアプリケーションに AppleScript コマンドを送信できるためです。または、コマンドを [`excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) に追加してください。
* **`docker` コマンドが失敗する**：`docker` はサンドボックスと互換性がありません。`docker *` を [`excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) に追加してください。
* **`pbcopy`、`xclip`、または `wl-copy` がクリップボードを更新しない**：これらのクリップボードユーティリティはサンドボックス内からシステムクリップボードに到達できず、パイプされたテキストが到達しない場合があります。

  Claude の出力をクリップボードに配置するには、Claude にレスポンスで出力するよう依頼してから、[`/copy`](/docs/ja/commands) を実行してください。`/copy` はサンドボックス化されたコマンドではなく Claude Code プロセスからクリップボードに書き込みます。

  Claude がテキストをこれらのツールの 1 つにパイプする場合、ツールを [`excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) に追加しても、その呼び出しをサンドボックス外に出すわけではありません。
* **git コマンドが `unable to unlink old` で失敗する**：`git merge`、`git checkout` などのコマンドは、サンドボックスが書き込みを拒否するファイルを置き換える必要がある場合にこのように失敗します。そのファイルが `.claude/skills` などの[保護されたパス](#protected-paths)の下にあるか、`denyWrite` エントリの 1 つの下にあるか、またはサンドボックスがコマンドに書き込みを許可するディレクトリの外にあるかどうかです。Linux と WSL2 ではエラーは `Read-only file system` で終わります。

  失敗後、Claude は[コマンドをサンドボックス外で再実行することを提案](#the-unsandboxed-retry-escape-hatch)する場合があります。その再試行を承認するか、別のターミナルで git コマンドを自分で実行してください。`allowUnsandboxedCommands` を `false` に設定している場合、Claude は再試行を提案できないため、コマンドを自分で実行してください。同じ git コマンドが頻繁に失敗する場合は、[`excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) に追加してください。
* **Bubblewrap がコンテナ内で起動に失敗する**：非特権コンテナでは、bubblewrap は新しい `/proc` ファイルシステムをマウントできないため、サンドボックス化されたコマンドは `bwrap` エラー（`Can't mount proc on /newroot/proc: Operation not permitted` など）で失敗します。[`enableWeakerNestedSandbox`](/docs/ja/settings-reference#sandbox-enableweakernestedsandbox) を `true` に設定して、内部サンドボックスがコンテナの既存の `/proc` をバインドマウントするようにしてください。このオプションは、外部コンテナが既に必要な分離境界を提供する場合にのみ使用してください。新しい `/proc` マウントが隠すサンドボックス化されたコマンドにプロセス情報を公開するためです。
* **0 バイトの読み取り専用ファイルが `.claude` 設定パスに表示され、「はい、今後は聞かない」が保存されない**：Linux と WSL2 では、サンドボックスはサンドボックス化されたコマンドが実行されている間に、まだ存在しないファイルに対する書き込み拒否を保持するために、そこに 0 バイトの読み取り専用プレースホルダーを作成します。サンドボックスはその後プレースホルダーを削除します。SIGKILL などによってセッションがそのクリーンアップが実行される前に強制終了された場合、プレースホルダーは残ります。後のセッションは毎回起動時にそれらを読み取り専用で再度バインドするため、プレースホルダーが残っている箇所では、権限の選択を保存するなどの設定書き込みが失敗します。

  `claude doctor` を実行して、残されたプレースホルダーファイルをリストアップしてください。[`Stale sandbox mask files left by a killed session`](/docs/ja/errors#stale-sandbox-mask-files-left-by-a-killed-session) 警告は最大 3 つの名前を表示し、残りをカウントします。そのプロジェクトで他の Claude Code セッションが実行されていない間に、`rm` で各ファイルを削除してください。v2.1.257 より前では、Claude Code は同じプレースホルダーを残していましたが、フラグを立てていませんでした。
* **`--dangerously-skip-permissions` が root として失敗する**：このフラグは Linux と macOS で root として実行するか sudo 経由で実行する場合にブロックされます。root アクセスと許可プロンプトなしを組み合わせるとシステム上のあらゆるファイルまたはサービスを変更できるためです。チェックは認識されたサンドボックス内で自動的にスキップされます。コンテナで自律的に実行するには、[dev container](/docs/ja/devcontainer) 設定を使用してください。これは Claude Code を非 root ユーザーとして実行します。

<h2 id="limitations">
  制限事項
</h2>

サンドボックス化はリスクを軽減しますが、完全な分離境界ではありません。ハードセキュリティ制御として依存する前に、以下の制限事項を確認してください。

<h3 id="security-limitations">
  セキュリティ上の制限
</h3>

* **ネットワークフィルタリング**：サンドボックスは、プロセスが接続できるドメインを制限します。デフォルトでは、組み込みプロキシは発信トラフィックを終了または検査しないため、暗号化された接続の内容は検査されません。実験的な [`network.tlsTerminate`](/docs/ja/settings-reference#sandbox-network-tlsterminate) 設定は、[`mask` 認証情報置換](#mask-credentials)のためにプロキシで TLS を終了しますが、コンテンツフィルタリングは追加しません。ポリシーで許可されるのは信頼できるドメインのみであることを確認する責任があります。

<Warning>
  `github.com` などの広いドメインを許可すると、データ流出のパスが作成される可能性があります。プロキシは TLS を検査せずにクライアント提供のホスト名から許可決定を行うため、サンドボックス内で実行されるコードは [ドメインフロンティング](https://en.wikipedia.org/wiki/Domain_fronting)または同様の技術を使用して許可リスト外のホストに到達する可能性があります。脅威モデルがより強力な保証を必要とする場合は、TLS を終了してトラフィックを検査し、CA 証明書をサンドボックス内にインストールする [カスタムプロキシ](#custom-proxy-configuration)を設定してください。より強力な TLS 対応ネットワーク分離は開発の活発な領域です。
</Warning>

* **Unix ソケットを通じた権限昇格**：`allowUnixSockets` 設定は、サンドボックスバイパスにつながる可能性のあるシステムサービスへのアクセスを不注意に付与する可能性があります。たとえば、`/var/run/docker.sock` へのアクセスを許可すると、Docker ソケットを通じてホストシステムへのアクセスが効果的に付与されます。サンドボックスを通じて許可する Unix ソケットを慎重に検討してください。
* **ファイルシステム権限昇格**：過度に広いファイルシステム書き込み権限は権限昇格攻撃を有効にする可能性があります。`$PATH` の実行可能ファイルを含むディレクトリ、システム設定ディレクトリ、またはユーザーシェル設定ファイル（`.bashrc` または `.zshrc`）への書き込みを許可すると、他のユーザーまたはシステムプロセスがこれらのファイルにアクセスするときに異なるセキュリティコンテキストでコード実行につながる可能性があります。
* **Linux サンドボックス強度**：Linux 実装は強力なファイルシステムとネットワーク分離を提供しますが、特権付き名前空間のない Docker 環境内、または特権のないユーザー名前空間が sysctl で無効化されている Linux ホスト上で動作できるようにする `enableWeakerNestedSandbox` モードが含まれています。このオプションはセキュリティを大幅に弱め、追加の分離が別の方法で実施される場合にのみ使用する必要があります。
* **macOS での Apple Events**：macOS サンドボックスはデフォルトで Apple Events をブロックします。`allowAppleEvents` 設定はこの制限を解除して、`open` や `osascript` などのツールが動作するようにしますが、コード実行分離を削除します。サンドボックス化されたコマンドは、ユーザープロンプトなしで他のアプリケーションをサンドボックス化されていない状態で起動でき、実行中のアプリケーションに AppleScript コマンドを送信できます。これはアプリごとの macOS オートメーション同意プロンプト（TCC）の対象です。これはユーザー、管理、または CLI 設定からのみ有効です。プロジェクト設定では有効にできません。

<h3 id="platform-and-tool-compatibility">
  プラットフォームとツールの互換性
</h3>

* **プラットフォームサポート**：macOS、Linux、WSL2 をサポートします。WSL1 とネイティブ Windows はサポートされていません。
* **パフォーマンスオーバーヘッド**：最小限ですが、一部のファイルシステム操作はわずかに遅くなる可能性があります。
* **ツール互換性**：特定のシステムアクセスパターンを必要とするツールの中には、設定調整が必要な場合や、サンドボックス外で実行する必要がある場合があります。

<h3 id="scope">
  スコープ
</h3>

サンドボックスは Bash サブプロセスを分離します。他のツールは異なる境界の下で動作します。

* **組み込みファイルツール**：Read、Edit、Write はサンドボックスを通じて実行するのではなく、権限システムを直接使用します。[permissions](/docs/ja/permissions)を参照してください。
* **コンピュータ使用**：Claude がアプリを開いてスクリーンを制御する場合、分離された環境ではなく実際のデスクトップで実行されます。アプリごとの権限プロンプトが各アプリケーションをゲートします。[CLI でのコンピュータ使用](/docs/ja/computer-use)または [Desktop でのコンピュータ使用](/docs/ja/desktop#let-claude-use-your-computer)を参照してください。
* **環境変数**：サンドボックス化された Bash コマンドはデフォルトで親プロセス環境を継承します。そこに設定されたすべての認証情報を含みます。サンドボックス化されたコマンドの特定の変数を設定解除またはマスクするには [`sandbox.credentials`](#protect-credentials)を使用するか、すべてのサブプロセスから認証情報を削除するには [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ja/env-vars)を設定してください。
* **サブエージェント**：[subagents](/docs/ja/sub-agents)は親セッションと同じプロセスで実行され、同じサンドボックス設定を使用します。親セッションでサンドボックス化が有効な場合、サブエージェント内の Bash コマンドはサンドボックス化されます。

<Warning>
  効果的なサンドボックス化にはファイルシステムとネットワークの両方の分離が必要です。ネットワーク分離がない場合、侵害されたエージェントは SSH キーなどの機密ファイルを流出させる可能性があります。ファイルシステム分離がない場合、[ファイルシステムレイヤーを無効化](#disable-filesystem-isolation)することによるものであれ、侵害されたエージェントはシステムリソースにバックドアを仕掛けてネットワークアクセスを取得する可能性があります。デフォルトを広げるときは、`allowWrite` パス、広い `allowedDomains` エントリ、または `excludedCommands` 例外が反対側の制限を元に戻さないことを確認してください。
</Warning>

<h2 id="see-also">
  関連項目
</h2>

* [Sandbox environments](/docs/ja/sandbox-environments)：組み込みサンドボックスと dev コンテナ、コンテナ、VM を比較する
* [Security](/docs/ja/security)：包括的なセキュリティ機能とベストプラクティス
* [Permissions](/docs/ja/permissions)：許可設定とアクセス制御
* [All settings](/docs/ja/settings-reference)：すべての設定キー
* [CLI reference](/docs/ja/cli-reference)：コマンドラインオプション
