> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# インストールとログインのトラブルシューティング

> Claude Code のインストールまたはサインイン時に、コマンドが見つからない、PATH、権限、ネットワーク、認証エラーを修正します。

インストールが失敗した場合、またはサインインできない場合は、以下からエラーを見つけてください。Claude Code が動作している場合のランタイム問題については、[トラブルシューティング](/docs/ja/troubleshooting)を参照してください。設定が適用されない、またはフックが発火しないなどの設定の問題については、[設定をデバッグする](/docs/ja/debug-your-config)を参照してください。

<h2 id="find-your-error">
  エラーを見つける
</h2>

表示されているエラーメッセージまたは症状を修正方法と照合してください：

| 表示内容                                                                                                | 解決方法                                                                                                                                  |
| :-------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `command not found: claude` または `'claude' is not recognized`                                        | [PATH を修正する](#command-not-found-claude-after-installation)                                                                            |
| `syntax error near unexpected token '<'`                                                            | [インストールスクリプトが HTML を返す](#install-script-returns-html-instead-of-a-shell-script)                                                       |
| `curl: (22) The requested URL returned error: 403`                                                  | [インストールスクリプトが 403 を返す](#install-script-returns-html-instead-of-a-shell-script)                                                        |
| `curl: (23)` または `curl: (56) Failure writing output to destination`                                 | [接続性を確認するか、別のインストーラーを使用する](#curl-56-failure-writing-output-to-destination)                                                            |
| Linux でのインストール中に `Killed` または `Installation was killed before it could finish (exit code 137)`      | [メモリを解放するか、スワップスペースを追加する](#install-killed-on-low-memory-linux-servers)                                                                |
| インストール中に `Raw mode is not supported`                                                                | [インストーラーを再実行する](#raw-mode-is-not-supported-during-install)                                                                            |
| `TLS connect error` または `SSL/TLS secure channel`                                                    | [CA 証明書を更新する](#tls-or-ssl-connection-errors)                                                                                          |
| `Failed to fetch version` またはダウンロードサーバーに到達できない                                                      | [ネットワークとプロキシ設定を確認する](#check-network-connectivity)                                                                                     |
| `irm is not recognized` または `The token '&&' is not a valid statement separator`                     | [シェルに適切なコマンドを使用する](#wrong-install-command-on-windows)                                                                                 |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                                  | [Homebrew を更新する](#homebrew-cask-unavailable-or-outdated)                                                                              |
| `'bash' is not recognized as the name of a cmdlet`                                                  | [Windows インストーラーコマンドを使用する](#wrong-install-command-on-windows)                                                                         |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                    | [Windows インストーラーコマンドを使用する](#wrong-install-command-on-windows)                                                                         |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`                   | [シェルをインストールする](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                        |
| `Claude Code does not support 32-bit Windows`                                                       | [Windows PowerShell を開く（x86 エントリではなく）](#claude-code-does-not-support-32-bit-windows)                                                  |
| `The process cannot access the file ... because it is being used by another process`                | [ダウンロードフォルダをクリアして再試行する](#the-process-cannot-access-the-file-during-windows-install)                                                   |
| `Error loading shared library`                                                                      | [システムに対応したバイナリバリアント](#linux-musl-or-glibc-binary-mismatch)                                                                            |
| `Illegal instruction`                                                                               | [アーキテクチャまたは CPU 命令セットの不一致](#illegal-instruction)                                                                                      |
| WSL での `cannot execute binary file: Exec format error`                                              | [WSL1 ネイティブバイナリ回帰](#exec-format-error-on-wsl1)                                                                                        |
| PowerShell インストーラーが完了しても `claude` が見つからないか古いバージョンが表示される                                             | [インストールディレクトリを PATH に追加する](#verify-your-path)、その後新しいターミナルを開く                                                                          |
| macOS での `dyld: Symbol not found`、`dyld: cannot load`、または `Abort trap`                              | [バイナリ互換性](#dyld-cannot-load-on-macos)                                                                                                 |
| `claude update` が `Checking for updates` の後にハングするか、`claude doctor` が出力なしでハングする                      | [シェル設定パスのディレクトリを移動する](#claude-update-or-claude-doctor-hangs)                                                                          |
| `Invoke-Expression` または `iex` が HTML タグまたは CSS をクォートするパースエラー、または `ParseException` を伴う `ParserError` | [インストールスクリプトが HTML を返す](#install-script-returns-html-instead-of-a-shell-script)                                                       |
| `running scripts is disabled on this system` または `PSSecurityException`                              | [npm shims の実行を許可する](#running-scripts-is-disabled-on-this-system)                                                                     |
| `Error: claude native binary not installed`                                                         | [npm インストールを完了する](#native-binary-not-found-after-npm-install)                                                                         |
| npm の更新または再インストール中に `npm error code ENOTEMPTY`                                                      | [残されたパッケージディレクトリを削除する](#npm-enotempty-during-update-or-reinstall)                                                                     |
| Windows では、インストールコマンドがスクリプトテキストを出力し、何もインストールされない                                                    | [完全なインストールコマンドを実行する](#wrong-install-command-on-windows)                                                                               |
| `App unavailable in region`                                                                         | Claude Code はお客様の国では利用できません。[サポートされている国](https://www.anthropic.com/supported-countries)を参照してください。                                     |
| `unable to get local issuer certificate`                                                            | [企業 CA 証明書を設定する](#tls-or-ssl-connection-errors)                                                                                       |
| `OAuth error` または `403 Forbidden`                                                                   | [認証を修正する](#login-and-authentication)                                                                                                  |
| セットアップ中に `Unable to connect to Anthropic services`                                                  | エラーリファレンスの [Unable to connect to Anthropic services](/docs/ja/errors#unable-to-connect-to-anthropic-services) を参照してください                    |
| `Could not load the default credentials` または `Could not load credentials from any providers`        | [Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry 認証情報](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` または `CredentialUnavailableError`                     | [Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry 認証情報](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`、`529 Overloaded`、`429`、またはその他の 4xx および 5xx エラー（上記以外）                               | [エラーリファレンス](/docs/ja/errors)を参照してください                                                                                                      |

問題がリストに記載されていない場合は、以下の診断チェックを実行して、原因を特定してください。

<Tip>
  ターミナルをスキップしたい場合は、[Claude Code Desktop アプリ](/docs/ja/desktop-quickstart)を使用して、グラフィカルインターフェイスを通じて Claude Code をインストールして使用できます。[macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs)、[Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs)用にダウンロードして、コマンドラインセットアップなしでコーディングを開始してください。Linux では、[Linux インストール手順](/docs/ja/desktop-linux)に従って apt でアプリをインストールしてください。
</Tip>

<h2 id="run-diagnostic-checks">
  診断チェックを実行する
</h2>

<h3 id="check-network-connectivity">
  ネットワーク接続を確認する
</h3>

インストーラーは `downloads.claude.ai` からダウンロードします。到達可能であることを確認してください：

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    curl -sI https://downloads.claude.ai/claude-code-releases/latest
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    curl.exe -sI https://downloads.claude.ai/claude-code-releases/latest
    ```

    PowerShell は `curl` を `Invoke-WebRequest` にエイリアスしており、`-sI` フラグを拒否するため、`curl.exe` を明示的に呼び出してください。
  </Tab>
</Tabs>

最初の行が `200` ステータスを表示している場合、サーバーに到達しました。macOS と Linux では `HTTP/2 200` が表示され、Windows に含まれる `curl.exe` からは `HTTP/1.1 200 OK` が表示されます。その他の結果は原因を示しています：

* `403`：通常、プロキシまたはネットワークフィルターがホストをブロックしているか、Claude Code が[お客様の地域では利用できません](https://www.anthropic.com/supported-countries)
* `5xx`：通常、一時的なサービスの問題です。数分待ってから再試行してください

出力がない、`Could not resolve host`、または接続タイムアウトが表示される場合、ネットワークが接続をブロックしています。一般的な原因：

* `downloads.claude.ai` をブロックしている企業ファイアウォールまたはプロキシ
* 地域的なネットワーク制限：VPN または別のネットワークを試してください
* TLS/SSL の問題：システムの CA 証明書を更新するか、`HTTPS_PROXY` が設定されているかどうかを確認してください

企業プロキシの背後にいる場合は、インストール前に `HTTPS_PROXY` と `HTTP_PROXY` をプロキシのアドレスに設定してください。プロキシ URL がわからない場合は IT チームに問い合わせるか、ブラウザのプロキシ設定を確認してください。

この例は両方のプロキシ変数を設定してから、プロキシを通じてインストーラーを実行します：

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  PATH を確認する
</h3>

インストールが成功しても、`claude` を実行するときに `command not found` または `not recognized` エラーが表示される場合、インストールディレクトリが PATH に含まれていません。シェルは PATH にリストされているディレクトリ内のプログラムを検索し、インストーラーは macOS/Linux では `~/.local/bin/claude` に、Windows では `%USERPROFILE%\.local\bin\claude.exe` に `claude` を配置します。

<Note>
  [VS Code 拡張機能](/docs/ja/vs-code)は `claude` をこの場所に配置しません。拡張機能ディレクトリ内に CLI のプライベートコピーをバンドルし、独自のチャットパネル用に使用し、PATH に追加しません。拡張機能のみをインストールした場合、`~/.local/bin/claude` は存在しません。ターミナルから `claude` を使用するには[スタンドアロンインストール](/docs/ja/setup)を実行してから、以下を続行してください。
</Note>

インストールディレクトリが PATH に含まれているかどうかを確認するには、PATH エントリをリストして `local/bin` でフィルタリングしてください：

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    これが `/Users/you/.local/bin` または `/home/you/.local/bin` を出力する場合、ディレクトリは PATH に含まれており、[競合するインストールを確認する](#check-for-conflicting-installations)にスキップできます。出力がない場合は、シェル設定に追加してください。

    macOS のデフォルトである Zsh の場合：

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    ほとんどの Linux ディストリビューションのデフォルトである Bash の場合：

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    または、ターミナルを閉じて再度開いてください。

    fish や Nushell などの他のシェルの場合は、シェル独自の設定構文を使用して `~/.local/bin` を PATH に追加してから、ターミナルを再起動してください。

    修正が機能したことを確認してください：

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    出力がない場合は、インストールディレクトリをユーザー PATH に追加してください：

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    変更を有効にするためにターミナルを再起動してください。

    修正が機能したことを確認してください：

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    出力がない場合は、システム設定を開き、環境変数に移動して、`%USERPROFILE%\.local\bin` をユーザー PATH 変数に追加してください。ターミナルを再起動してください。

    修正が機能したことを確認してください：

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  競合するインストールを確認する
</h3>

複数の Claude Code インストールはバージョンの不一致または予期しない動作を引き起こす可能性があります。インストールされているものを確認してください：

<Tabs>
  <Tab title="macOS/Linux">
    PATH に見つかったすべての `claude` バイナリをリストします：

    ```bash theme={null}
    which -a claude
    ```

    これが何も出力しない場合、`claude` はまだ PATH にありません。[PATH を確認する](#verify-your-path)に戻ってください。

    `claude` バイナリが来ることができる 3 つの場所を確認してください。`~/.local/bin/claude` はネイティブインストーラー、`~/.claude/local/` は Claude Code の古いバージョンによって作成されたレガシーローカル npm インストール、npm グローバルリストは `-g` インストールを示します：

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    ネイティブインストールは `~/.local/share/claude/versions/` へのシンボリックリンクを表示します。このパスで自分で作成したスクリプトまたはシンボリックリンクはカスタムランチャーであり、[自動更新はそのまま残します](/docs/ja/setup#auto-updates)。

    `ls` コマンドが `No such file or directory` を出力する場合、それはエラーではありません。その場所に何もインストールされていないことを意味するため、次のチェックに進んでください。

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    PATH に見つかったすべての `claude` バイナリをリストします：

    ```powershell theme={null}
    where.exe claude
    ```

    ネイティブインストーラーがバイナリを配置したかどうかを確認してください：

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

複数のインストールが見つかった場合は、1 つだけを保持してください。macOS/Linux の `~/.local/bin/claude` または Windows の `%USERPROFILE%\.local\bin\claude.exe` でのネイティブインストールが推奨されます。余分なものを削除してください：

npm グローバルインストールをアンインストールします：

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

レガシーローカル npm インストールを削除します：

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    rm -rf ~/.claude/local
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
    ```
  </Tab>
</Tabs>

macOS で Homebrew インストールを削除します。`claude-code@latest` cask をインストールした場合は、その名前に置き換えてください：

```bash theme={null}
brew uninstall --cask claude-code
```

Windows で WinGet インストールを削除します：

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  ディレクトリ権限を確認する
</h3>

インストーラーは macOS と Linux の `~/.local/bin/` と `~/.claude/` への書き込みアクセスが必要です。Windows ではインストール場所は `%USERPROFILE%` の下にあり、デフォルトではユーザーが書き込み可能なため、このセクションはそこではほとんど適用されません。

ディレクトリが書き込み可能かどうかを確認してください：

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

いずれかのディレクトリが書き込み可能でない場合は、インストールディレクトリを作成し、ユーザーを所有者として設定してください：

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  バイナリが機能することを確認する
</h3>

`claude --version` がバージョンを出力しても `claude` がクラッシュまたはハングする場合は、これらのチェックを実行して原因を特定してください。`claude --version` がコマンドが見つからないと言う場合は、最初に [PATH を確認する](#verify-your-path)に移動してください。以下のコマンドは `claude` が PATH にあることを前提としています。

バイナリが存在し、実行可能であることを確認してください：

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    ls -la "$(command -v claude)"
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Get-Command claude | Select-Object Source
    ```
  </Tab>
</Tabs>

Linux では、不足している共有ライブラリを確認してください。`ldd` が不足しているライブラリを表示する場合は、システムパッケージをインストールする必要があるかもしれません。Alpine Linux およびその他の musl ベースのディストリビューションについては、[Alpine Linux セットアップ](/docs/ja/setup#alpine-linux-and-musl-based-distributions)を参照してください。

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

バイナリが実行できることを確認してください：

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  インストールの一般的な問題
</h2>

これらは最も頻繁に遭遇するインストール問題とその解決策です。

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  インストールスクリプトがシェルスクリプトではなく HTML を返す
</h3>

インストールコマンドを実行すると、次のいずれかのエラーが表示される場合があります。

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

PowerShell では、同じ問題が返されたページを指すパースエラーとして表示され、`iex` が HTML と CSS を PowerShell として実行しようとします。

```text theme={null}
iex : At line:1 char:2310
+ ... igin="anonymous"/><script type="text/javascript">!function(o,c){var n ...
Missing argument in parameter list.
...
```

表現は PowerShell のバージョンとシステム言語によって異なります。`Missing expression after unary operator '--'` または `ParserError` と `ParseException` が表示される場合があります。引用符で囲まれたテキスト内の HTML タグまたは CSS はこの失敗を示します。代わりに `-OutFile install.ps1` でダウンロードしても、保存されたファイルは同じウェブページなので、それは役に立ちません。

リクエストのルーティング方法によっては、HTML ボディなしで 403 が表示される場合があります。

```text theme={null}
curl: (22) The requested URL returned error: 403
```

これらはすべて、インストール URL がインストールスクリプトではなく HTML ページまたはエラーステータスを返したことを意味します。HTML ページに「App unavailable in region」と表示されている場合、Claude Code はお客様の国では利用できません。[サポートされている国](https://www.anthropic.com/supported-countries)を参照してください。

ボディなしの 403 は多くの場合同じ原因ですが、企業プロキシまたはファイアウォールがダウンロードをブロックしている場合もあります。サポートされている国にいるのに 403 が表示される場合は、以下の代替インストーラーを試す前に[ネットワーク接続を確認](#check-network-connectivity)してください。これらは同じホストに到達するためです。

それ以外の場合、これはネットワークの問題、地域的なルーティング、または一時的なサービス中断が原因で発生する可能性があります。

**解決策：**

1. **別のインストール方法を使用する**：

   macOS では、Homebrew 経由でインストールします。

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Windows では、WinGet 経由でインストールします。

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

   次に `claude --version` を実行して確認します。コマンドは `2.1.211 (Claude Code)` などのバージョン番号を出力します。シェルが `claude` が見つからないと報告する場合は、新しいターミナルウィンドウを開いて再試行してください。インストール元のセッションは古い `PATH` を保持しています。

2. **数分後に再試行する**：この問題は多くの場合一時的です。待機して元のコマンドを再度試してください。

<h3 id="command-not-found-claude-after-installation">
  インストール後の `command not found: claude`
</h3>

インストールは完了しましたが、`claude` が機能しません。正確なエラーはプラットフォームによって異なります。

| プラットフォーム    | エラーメッセージ                                                               |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

これは、インストールディレクトリがシェルの検索パスにないことを意味します。各プラットフォームの修正については、[PATH を確認](#verify-your-path)を参照してください。

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

`curl ... | bash` コマンドはスクリプトをダウンロードして Bash にパイプして実行します。このエラーと関連する `curl: (23) Failure writing output to destination` は、Bash がスクリプト全体を受け取らなかったことを意味します。終了コード 56 はダウンロード自体が中断されたことを示し、終了コード 23 は curl がパイプに受け取ったものを書き込めなかったことを示します。通常は Bash が早期に終了したためです。

[ネットワーク接続を確認](#check-network-connectivity)のチェックで `downloads.claude.ai` に到達できることをテストしてください。サーバーに到達した場合、元の失敗は一時的である可能性があります。インストールコマンドを再試行してください。[別のインストール方法を試す](/docs/ja/setup#install-claude-code)こともできます。

<h3 id="homebrew-cask-unavailable-or-outdated">
  Homebrew cask が利用できないか古い
</h3>

Homebrew は、Homebrew cask インデックスのローカルコピーが cask の公開前の場合、`Error: Cask 'claude-code' is unavailable: No Cask with this name exists` を報告します。インデックスを更新して再試行してください。

```bash theme={null}
brew update
brew install --cask claude-code
```

Homebrew が予想より古い Claude Code バージョンをインストールする場合、通常は同じ古いインデックスが原因です。`claude-code` cask は安定チャネルを追跡し、通常は最新リリースの約 1 週間遅れています。最新バージョンを実行するには、代わりに `brew install --cask claude-code@latest` を実行してください。2 つの cask の違いについては、[リリースチャネルを設定](/docs/ja/setup#configure-release-channel)を参照してください。

<h3 id="tls-or-ssl-connection-errors">
  TLS または SSL 接続エラー
</h3>

`curl: (35) TLS connect error`、`schannel: next InitializeSecurityContext failed`、または PowerShell の `Could not establish trust relationship for the SSL/TLS secure channel` などのエラーは、TLS ハンドシェイク失敗を示します。

**解決策：**

1. **システム CA 証明書を更新する**：

   Ubuntu/Debian では：

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   macOS では、システム curl は Keychain トラストストアを使用します。macOS 自体を更新するとルート証明書が更新されます。

2. **Windows では、インストーラーを実行する前に PowerShell で TLS 1.2 を有効にする**：
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **プロキシまたはファイアウォールの干渉を確認する**：TLS 検査を実行する企業プロキシはこれらのエラーを引き起こす可能性があります。`unable to get local issuer certificate` と `SELF_SIGNED_CERT_IN_CHAIN` を含みます。インストールステップでは、インストールダウンロードが企業プロキシの CA を信頼するようにします。

   <Tabs>
     <Tab title="macOS/Linux">
       ```bash theme={null}
       curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
       ```
     </Tab>

     <Tab title="Windows PowerShell">
       PowerShell インストーラーは .NET 経由でダウンロードされ、Windows 証明書ストアに対して TLS を検証します。プロキシの CA 証明書がまだそこにない場合は、IT チームに Windows ストアに追加するよう依頼してから、インストーラーを実行してください。

       ```powershell theme={null}
       irm https://claude.ai/install.ps1 | iex
       ```
     </Tab>
   </Tabs>

   インストール後の Claude Code 自体については、`NODE_EXTRA_CA_CERTS` を設定して API リクエストが同じバンドルを信頼するようにします。

   <Tabs>
     <Tab title="macOS/Linux">
       ```bash theme={null}
       export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
       ```
     </Tab>

     <Tab title="Windows PowerShell">
       ```powershell theme={null}
       $env:NODE_EXTRA_CA_CERTS = 'C:\path\to\corporate-ca.pem'
       ```
     </Tab>
   </Tabs>

   証明書ファイルがない場合は IT チームに依頼してください。直接接続で試して、プロキシが原因であることを確認することもできます。

4. **Windows では、ブロックされた失効確認を回避する**。エラー `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` と `CRYPT_E_REVOCATION_OFFLINE (0x80092013)` は curl がサーバーに到達したが、ネットワークが証明書失効ルックアップをブロックしていることを意味します。これは企業ファイアウォールの背後で一般的です。失敗しているコマンドが `install.cmd` をダウンロードする `curl` の場合、`--ssl-revoke-best-effort` を追加してコマンドプロンプトから再実行してください。
   ```batch theme={null}
   curl --ssl-revoke-best-effort -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
   ```
   スクリプト自体のダウンロードが同じエラーに遭遇すると、自動的にベストエフォート失効確認で再試行されるため、フラグは自分で実行するコマンドにのみ必要です。ベストエフォート確認は到達不可能な失効サーバーを許容しますが、失効していることが判明している証明書は引き続き拒否します。これはブラウザが失効を処理する方法と一致します。PowerShell から PowerShell インストーラーを実行することで curl の失効確認を完全に回避することもできます。これは .NET 経由でダウンロードされ、失効サーバーが到達不可能な場合は失敗しません。
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   `winget install Anthropic.ClaudeCode` でインストールすることもできます。これは curl を完全に回避します。

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

インストーラーがダウンロードサーバーに到達できませんでした。これは通常、`downloads.claude.ai` がネットワークでブロックされていることを意味します。[ネットワーク接続を確認](#check-network-connectivity)を参照してください。

<h3 id="wrong-install-command-on-windows">
  Windows での間違ったインストールコマンド
</h3>

`'irm' is not recognized`、`The token '&&' is not a valid statement separator`、`A parameter cannot be found that matches parameter name 'fsSL'`、または `'bash' is not recognized as the name of a cmdlet` が表示される場合、別のシェルまたはオペレーティングシステムのインストールコマンドをコピーしました。コマンドがスクリプトのテキストを出力する場合、インストールの一部のみを実行しました。

* **`irm` が認識されない**：CMD にいます。PowerShell ではありません。2 つのオプションがあります。

  スタートメニューで「PowerShell」を検索して PowerShell を開き、元のインストールコマンドを実行します。

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  または CMD にとどまり、CMD インストーラーを代わりに使用します。

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` が有効なステートメント区切り文字ではない**：PowerShell にいますが、CMD インストーラーコマンドを実行しました。PowerShell インストーラーを使用してください。
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**：Windows PowerShell で macOS/Linux `curl -fsSL ... | bash` インストーラーを実行しました。`curl` は `Invoke-WebRequest` のエイリアスであり、`-fsSL` フラグを拒否します。代わりに PowerShell インストーラーを使用してください。
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` が認識されない**：Windows で macOS/Linux インストーラーを実行しました。代わりに PowerShell インストーラーを使用してください。
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **コマンドがスクリプトテキストを出力する**：ダウンロード部分を実行せずに、それを実行する部分なしでコマンドを実行しました。`irm https://claude.ai/install.ps1` 単独でダウンロードされたスクリプトをターミナルに出力します。`iex` にパイプして実行します。

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  CMD では、`-o` なしの `curl -fsSL https://claude.ai/install.cmd` はバッチスクリプトを保存する代わりに出力します。完全なコマンドを実行します。

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

どのインストーラーを使用するにしても、それが機能したことを確認します。新しいターミナルを開いて `claude --version` を実行します。これは `2.1.211 (Claude Code)` などのバージョン番号を出力します。

<h3 id="running-scripts-is-disabled-on-this-system">
  `running scripts is disabled on this system`
</h3>

Windows で npm 経由で Claude Code をインストールまたは実行すると、`SecurityError` で失敗する場合があります。

```text theme={null}
npm : File C:\Program Files\nodejs\npm.ps1 cannot be loaded because running scripts is disabled on this system. For more information, see about_Execution_Policies at https:/go.microsoft.com/fwlink/?LinkID=135170.
...
    + CategoryInfo          : SecurityError: (:) [], PSSecurityException
```

npm インストール後に `claude` を実行すると、同じエラーが `claude.ps1` に名前を付けます。PowerShell の実行ポリシーは npm がそのコマンド用に作成する `.ps1` ランチャースクリプトをブロックしています。ポリシーはスクリプトファイルに適用されるため、PowerShell インストーラー `irm https://claude.ai/install.ps1 | iex` には影響しません。これはダウンロードされたテキストを直接実行します。

**解決策：**

1. **ユーザーのローカルで作成されたスクリプトを許可してから再試行します**。
   ```powershell theme={null}
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```
2. **`.cmd` ランチャーを呼び出す**：`npm.cmd` と `claude.cmd` は同じ仕事をし、ポリシーはそれらをカバーしません。
3. **npm の代わりに [PowerShell インストーラー](/docs/ja/setup#install-claude-code)を使用する**。バイナリではなく `.ps1` スクリプトをインストールします。

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  Windows インストール中の `The process cannot access the file`
</h3>

PowerShell インストーラーが `Failed to download binary: The process cannot access the file ... because it is being used by another process` で失敗する場合、インストーラーは `%USERPROFILE%\.claude\downloads` に書き込めませんでした。これは通常、前のインストール試行がまだ実行中であるか、アンチウイルスソフトウェアがそのフォルダー内の部分的にダウンロードされたバイナリをスキャンしていることを意味します。

インストーラーを実行している他の PowerShell ウィンドウを閉じ、アンチウイルススキャンがファイルを解放するまで待機します。次にダウンロードフォルダーを削除してインストーラーを再度実行します。

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  メモリ不足の Linux サーバーでインストールが強制終了される
</h3>

インストール中の `Killed` メッセージは通常、Linux メモリ不足（OOM）キラーがシステムのメモリ不足のため `claude install` ステップを終了したことを意味します。これは小規模な VPS とクラウドインスタンスで一般的です。インストールスクリプトは原因を報告し、終了コード 137 で終了します。この例では、行番号とプロセス ID はリリースと実行によって異なります。

```text theme={null}
Setting up Claude Code...
bash: line 183: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

インストールには約 512 MB の空きメモリが必要で、Claude Code を実行するにはさらに多くが必要です。[システム要件](/docs/ja/setup#system-requirements)を参照してください。

**解決策：**

1. **サーバーの RAM が限られている場合はスワップスペースを追加する**。スワップはディスク領域をオーバーフロー メモリとして使用し、物理 RAM が少ない場合でもインストールを完了できます。

   2 GB スワップファイルを作成して有効にします。

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   次にインストールを再試行します。

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **インストール前に他のプロセスを閉じてメモリを解放する**。

3. **可能であれば、より大きなインスタンスを使用する**。Claude Code には最低 4 GB の RAM が必要です。

<h3 id="install-hangs-in-docker">
  Docker でインストールがハングする
</h3>

Docker コンテナーで Claude Code をインストールする場合、root として `/` にインストールするとハングが発生する可能性があります。

**解決策：**

1. **インストーラーを実行する前に作業ディレクトリを設定する**。`/` から実行すると、インストーラーはファイルシステム全体をスキャンし、過度なメモリ使用が発生します。`WORKDIR` を設定すると、スキャンが小さなディレクトリに制限されます。
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Docker Desktop を使用している場合は Docker にメモリを追加する**。ビルドコンテナーは Docker Desktop 仮想マシンに割り当てられたメモリを共有するため、Docker Desktop で **Settings > Resources** を開き、メモリ制限を上げて、ビルドを再実行します。

<h3 id="raw-mode-is-not-supported-during-install">
  インストール中の `Raw mode is not supported`
</h3>

組織の[サーバー管理設定](/docs/ja/server-managed-settings)に[セキュリティ承認](/docs/ja/server-managed-settings#security-approval-dialogs)が必要な変更が含まれている場合、Claude Code バージョン 2.1.246 より前は `claude install` 中に承認ダイアログを表示しようとします。ダイアログは stdin のターミナルが必要です。インストーラーが `curl -fsSL https://claude.ai/install.sh | bash` のようにパイプから `claude install` を実行する場合、stdin はターミナルではなくパイプであるため、インストールは `Raw mode is not supported` を含むエラーで失敗します。

Claude Code v2.1.246 以降は、`claude install` または `claude update` 中にダイアログを表示しません。コマンドは最後に承認した設定で実行され、Claude Code は次の対話型セッションでダイアログを表示します。組織のスタートアップ構成が[設定フェッチを待つ](/docs/ja/server-managed-settings#enforce-fail-closed-startup)場合（`forceRemoteSettingsRefresh` を設定する場合など）、ダイアログはこれらのコマンド中に表示され、パイプから実行されたインストールは引き続き失敗します。

他のすべての構成では、インストーラーを再実行すると、古いバージョンをインストールするよう求めた場合でも、スクリプトは最新リリースの `install` コマンドを実行するため、このエラーを回避できます。プラットフォームのコマンドを再実行してください。

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

`claude --version` は再実行がインストールしたバージョンを出力します。

<h3 id="claude-update-or-claude-doctor-hangs">
  `claude update` または `claude doctor` がハングする
</h3>

`claude update` と `claude doctor` は、古い `claude` エイリアス `~/.zshrc`、`~/.bashrc`、`~/.config/fish/config.fish` のシェル構成ファイルをスキャンします。macOS では、存在する `~/.bash_profile`、`~/.bash_login`、`~/.profile` の最初のものです。`ZDOTDIR` を設定する場合、Zsh ファイルは代わりに `$ZDOTDIR/.zshrc` です。これらのパスの 1 つがディレクトリの場合、Claude Code はそれをスキップし、両方のコマンドが正常に完了します。v2.1.214 より前では、これらのパスの 1 つにあるディレクトリは両方のコマンドをハングさせ、`/status` のシステム診断セクションを空白のままにしました。`claude doctor` は出力なしでハングしました。`claude update` はハングしました。`Checking for updates` を出力した直後。

以前のバージョンでハングに遭遇した場合は、ディレクトリを見つけてください。このコマンドの出力では、`d` で始まる行がそのパスをディレクトリとしてマークします。`No such file or directory` 行は、そのパスに何も存在せず、原因ではないことを意味します。

```bash theme={null}
ls -ld ~/.zshrc ~/.bashrc ~/.bash_profile ~/.bash_login ~/.profile ~/.config/fish/config.fish
```

ディレクトリを移動するか、v2.1.214 以降に更新してください。`claude update` は影響を受けるバージョンでハングするため、代わりに[インストールスクリプト](/docs/ja/setup#install-claude-code)を再実行して更新してください。

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop が Windows で `claude` コマンドをオーバーライドする
</h3>

古いバージョンの Claude Desktop をインストールした場合、`WindowsApps` ディレクトリに `Claude.exe` を登録して、Claude Code CLI より PATH の優先度を高くする可能性があります。`claude` を実行すると、CLI ではなくデスクトップアプリが開きます。

Claude Desktop を最新バージョンに更新して、この問題を修正してください。

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Windows の Claude Code には Git for Windows（bash 用）または PowerShell のいずれかが必要です
</h3>

Git for Windows はオプションです。Claude Code は Git Bash がない場合、[PowerShell ツール](/docs/ja/tools-reference#powershell-tool)を使用するため、このエラーはどちらのシェルも見つからなかったことを意味します。

**PowerShell が PATH にない場合**、デフォルトの場所は `C:\Windows\System32\WindowsPowerShell\v1.0\` です。そのディレクトリを `PATH` に追加するか、`pwsh` を提供する [PowerShell 7](https://aka.ms/powershell) をインストールしてください。

**Git for Windows をインストールする代わりに**、[git-scm.com/downloads/win](https://git-scm.com/downloads/win) からダウンロードしてください。セットアップ中に「Add to PATH」を選択してください。インストール後にターミナルを再起動してください。インストールすると Bash ツールが有効になり、Bash ベースのスクリプトとツーリングを操作する場合に便利です。

**Git が既にインストールされている**が Claude Code が見つけられない場合は、その場所を Claude Code がチェックする場所と比較してください。`CLAUDE_CODE_GIT_BASH_PATH` が設定されていない場合、Claude Code は次の順序で `bash.exe` を探します。

1. デフォルトのインストール場所 `C:\Program Files\Git` と `C:\Program Files (x86)\Git`。
2. `PATH` 上の `git`。その Git インストールから `bin\bash.exe` を使用します。

ステップ 2 では、Claude Code は Claude Code を起動したフォルダーに存在する `git`、またはそのフォルダーの下のパスで `node_modules` または `.venv` や `env` などの仮想環境フォルダーを含むパスをスキップします。例えば、`C:\dev\env\myproject` から起動した場合の `C:\dev\env\myproject\Git`。これにより、Claude Code がプロジェクトがそこに配置した実行可能ファイルを実行することを防ぎます。Git がそのような場所にある場合は、`CLAUDE_CODE_GIT_BASH_PATH` をそれに指します。

**Claude Code を特定の Git インストールに指す**には、PowerShell で `where.exe git` を実行して見つけ、そのインストールから `bin\bash.exe` パスを [settings.json ファイル](/docs/ja/settings)の `CLAUDE_CODE_GIT_BASH_PATH` として設定します。

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

**`CLAUDE_CODE_GIT_BASH_PATH` が正しいパスに設定されており、ファイルが存在する**が Claude Code がそれを使用しない場合は、まずファイルの名前を確認してください。Claude Code は `bash.exe`、`sh.exe`、`bash`、または `sh` という名前のファイルのみを受け入れます。Git for Windows の `git-bash.exe` ランチャーなど、他の名前では、変数を無視して Git Bash を自動検出し、`--debug` で表示される警告をログに記録します。存在しないパスは同じフォールバックと警告を取得します。v2.1.219 より前では、Claude Code は名前をチェックせずに既存のファイルを使用し、パスが存在しない場合は `Claude Code was unable to find CLAUDE_CODE_GIT_BASH_PATH path` で起動時に終了しました。

ファイルの名前が正しい場合、AppLocker、グループポリシーソフトウェア制限ポリシー、EDR エージェントなどのエンドポイントセキュリティソフトウェアが干渉している可能性があります。IT チームに `claude.exe` と、`cmd.exe` や `bash.exe` を含むそれが生成するプロセスをエンドポイント保護ポリシーでホワイトリストに登録するよう依頼してください。

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code は 32 ビット Windows をサポートしていません
</h3>

Windows のスタートメニューには 2 つの PowerShell エントリがあります。`Windows PowerShell` と `Windows PowerShell (x86)`。x86 エントリは 32 ビットプロセスとして実行され、64 ビットマシンでもこのエラーをトリガーします。どちらの場合かを確認するには、エラーを生成した同じウィンドウで次を実行します。

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

これが `True` を出力する場合、オペレーティングシステムは問題ありません。ウィンドウを閉じ、x86 サフィックスなしで `Windows PowerShell` を開き、インストールコマンドを再度実行します。

これが `False` を出力する場合、32 ビット版の Windows を使用しています。Claude Code には 64 ビットオペレーティングシステムが必要です。[システム要件](/docs/ja/setup#system-requirements)を参照してください。

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Linux musl または glibc バイナリの不一致
</h3>

インストール後に `libstdc++.so.6` や `libgcc_s.so.1` などの共有ライブラリが見つからないというエラーが表示される場合、インストーラーはシステムの間違ったバイナリバリアントをダウンロードした可能性があります。

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

これは、musl クロスコンパイルパッケージがインストールされている glibc ベースのシステムで発生する可能性があり、インストーラーがシステムを musl として誤検出します。

**解決策：**

1. **システムが使用する libc を確認する**：
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   `GNU libc` または `GLIBC` に言及する出力は glibc を意味します。`musl` に言及する出力は musl を意味します。

2. **glibc にいるが musl バイナリを取得した場合**、インストールを削除して再インストールしてください。`https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json` のマニフェストを使用して正しいバイナリを手動でダウンロードすることもできます。`ldd --version` と `ls /lib/libc.musl*` の出力を含む [GitHub issue](https://github.com/anthropics/claude-code/issues) をファイルしてください。

3. **実際に musl にいる場合**（Alpine Linux など）、必要なパッケージをインストールしてください。
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```
   Alpine では、`ripgrep` はコミュニティリポジトリにあります。`apk` がパッケージが見つからないと報告する場合は、[Alpine Linux セットアップ](/docs/ja/setup#alpine-linux-and-musl-based-distributions)を参照してください。

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

`claude` または インストーラーを実行すると `Illegal instruction` が出力される場合、ネイティブバイナリはプロセッサーがサポートしていない CPU 命令を使用しています。2 つの異なる原因があります。

**アーキテクチャの不一致。** インストーラーは間違ったバイナリをダウンロードしました。例えば ARM サーバーで x86。macOS または Linux で `uname -m` を使用して確認するか、PowerShell で `$env:PROCESSOR_ARCHITECTURE` を使用してください。結果が受け取ったバイナリと一致しない場合は、出力を含む [GitHub issue](https://github.com/anthropics/claude-code/issues) をファイルしてください。

**AVX 命令セットが見つかりません。** アーキテクチャは正しいが `Illegal instruction` が表示される場合、CPU はおそらく AVX またはバイナリが必要とする別の命令がありません。これは約 2013 年以前の Intel および AMD プロセッサー、および ハイパーバイザーが AVX をゲストに渡さない仮想マシンに影響します。

VPS または VM では、`grep -m1 -ow avx /proc/cpuinfo` を実行します。空の結果は AVX がゲストで利用できないことを意味します。

ネイティブバイナリの回避策はありません。ステータスについては [issue #50384](https://github.com/anthropics/claude-code/issues/50384) を追跡し、報告する際に Linux で `grep -m1 "model name" /proc/cpuinfo` または macOS で `sysctl -n machdep.cpu.brand_string` から CPU モデルを含めてください。

代替インストール方法は同じネイティブバイナリをダウンロードし、どちらの原因も解決しません。

<h3 id="dyld-cannot-load-on-macos">
  macOS での `dyld: cannot load`
</h3>

インストール中に `dyld: Symbol not found`、`dyld: cannot load`、または `Abort trap: 6` が表示される場合、バイナリは macOS バージョンまたはハードウェアと互換性がありません。

`libicucore` を参照する `Symbol not found` エラーは、macOS バージョンがバイナリがサポートするより古いことを意味します。

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

ローダーは代わりにバイナリのロードコマンドを拒否する可能性があります。これは macOS バージョンが古すぎることも意味します。

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

**解決策：**

1. **macOS バージョンを確認する**：Claude Code には macOS 13.0 以降が必要です。Apple メニューを開き、「このマックについて」を選択してバージョンを確認してください。

2. **古いバージョンを使用している場合は macOS を更新する**。バイナリは古い macOS バージョンがサポートしていないロードコマンドとシステムライブラリを使用しています。Homebrew などの代替インストール方法は同じバイナリをダウンロードし、このエラーを解決しません。

<h3 id="exec-format-error-on-wsl1">
  WSL1 での `Exec format error`
</h3>

WSL で `claude` を実行すると `cannot execute binary file: Exec format error` が出力される場合、WSL1 にいて、[issue #38788](https://github.com/anthropics/claude-code/issues/38788) で追跡されている既知のネイティブバイナリ回帰に遭遇しています。バイナリのプログラムヘッダーが WSL1 のローダーが処理できない方法で変更されました。

最もクリーンな修正は、PowerShell からディストリビューションを WSL2 に変換することです。

```powershell theme={null}
wsl --set-version <DistroName> 2
```

WSL1 にとどまる必要がある場合は、動的リンカーを通じてバイナリを呼び出します。ホームディレクトリが異なる場合はパスを置き換えて、WSL 内の `~/.bashrc` にこの関数を追加します。

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

次に `source ~/.bashrc` を実行して `claude` を再試行します。

<h3 id="npm-install-errors-in-wsl">
  WSL での npm インストールエラー
</h3>

これらの問題は、WSL 内で `npm install -g` を使用して Claude Code をインストールした場合に適用されます。[ネイティブインストーラー](/docs/ja/setup)を使用した場合は、このセクションをスキップしてください。

**OS またはプラットフォーム検出の問題。** npm がインストール中にプラットフォームの不一致を報告する場合、WSL はおそらく Windows `npm` を選択しています。最初に `npm config set os linux` を実行してから、`npm install -g @anthropic-ai/claude-code --force` でインストールしてください。`sudo` を使用しないでください。

**`claude` を実行するときの `exec: node: not found`。** WSL 環境はおそらく Node.js の Windows インストールを使用しています。`which npm` と `which node` で確認してください。`/mnt/c/` で始まるパスは Windows バイナリで、Linux パスは `/usr/` で始まります。これを修正するには、Linux ディストリビューションのパッケージマネージャーまたは [`nvm`](https://github.com/nvm-sh/nvm) 経由で Node をインストールしてください。

**nvm バージョンの競合。** WSL と Windows の両方に nvm がインストールされている場合、WSL でノードバージョンを切り替えると、WSL がデフォルトで Windows PATH をインポートし、Windows nvm が優先されるため、破損する可能性があります。最も一般的な原因は、nvm がシェルに読み込まれていないことです。nvm ローダーを `~/.bashrc` または `~/.zshrc` に追加します。

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

または現在のセッションで読み込みます。

```bash theme={null}
source ~/.nvm/nvm.sh
```

nvm が読み込まれているが Windows パスが引き続き優先される場合は、Linux ノードパスを明示的に先頭に追加します。

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  `appendWindowsPath = false` 経由で Windows PATH インポートを無効にすることは避けてください。これは WSL から Windows 実行可能ファイルを呼び出す機能を破壊します。同様に、Windows 開発に使用する場合は Windows から Node.js をアンインストールすることは避けてください。
</Warning>

<h3 id="permission-errors-during-installation">
  インストール中の権限エラー
</h3>

ネイティブインストーラーが権限エラーで失敗する場合、ターゲットディレクトリが書き込み可能でない可能性があります。[ディレクトリ権限を確認](#check-directory-permissions)を参照してください。

以前に npm でインストールしており、npm 固有の権限エラーに遭遇している場合は、ネイティブインストーラーに切り替えてください。

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  npm インストール後にネイティブバイナリが見つからない
</h3>

`@anthropic-ai/claude-code` npm パッケージは、`@anthropic-ai/claude-code-darwin-arm64` などのプラットフォーム固有のオプション依存関係としてネイティブバイナリをダウンロードします。npm はパッケージの postinstall スクリプトを実行し、そのバイナリを `claude` コマンドとしてコピーします。実行されるまで、`claude` はプレースホルダースクリプトです。ダウンロードまたは postinstall ステップのいずれかがスキップされた場合、プレースホルダーは残り、macOS と Linux で `claude` を実行すると出力されます。

```text theme={null}
Error: claude native binary not installed.

Either postinstall did not run (--ignore-scripts, some pnpm configs)
or the platform-native optional dependency was not downloaded
(--omit=optional).

Run the postinstall manually (adjust path for local vs global install):
  node node_modules/@anthropic-ai/claude-code/install.cjs

Or reinstall without --ignore-scripts / --omit=optional.
```

Windows では、`bin/claude.exe` はその同じシェルスクリプトプレースホルダーであり、実際の実行可能ファイルではないため、PowerShell と CMD はこのメッセージを出力する代わりにファイルを実行できないと報告します。

次の原因を確認してください。

* **オプション依存関係が無効になっている。** npm インストールコマンドから `--omit=optional` を削除し、pnpm から `--no-optional` を削除し、yarn から `--ignore-optional` を削除し、`.npmrc` が `optional=false` を設定していないことを確認してから、再インストールしてください。ネイティブバイナリはオプション依存関係としてのみ配信されるため、スキップされた場合は JavaScript フォールバックがなく、`install.cjs` を再度実行してもダウンロードされなかったバイナリを配置できません。
* **インストールスクリプトが無効になっている。** `--ignore-scripts` と一部の pnpm 構成は postinstall ステップをスキップしますが、プラットフォームパッケージはダウンロードします。メッセージが示唆するように `node node_modules/@anthropic-ai/claude-code/install.cjs` を実行するか、フラグなしで再インストールしてください。postinstall が環境で実行できない場合、`node node_modules/@anthropic-ai/claude-code/cli-wrapper.cjs` はダウンロードされたパッケージを見つけてそれを起動します。各起動時に余分なノードプロセスのコストで。ラッパーが `Could not find native binary package` を出力する代わりに、プラットフォームパッケージはダウンロードされなかったため、上記のオプション依存関係の原因を最初に修正してください。
* **サポートされていないプラットフォーム。** プリビルドバイナリは `darwin-arm64`、`darwin-x64`、`linux-x64`、`linux-arm64`、`linux-x64-musl`、`linux-arm64-musl`、`win32-x64`、`win32-arm64` に対して公開されています。Claude Code は他のプラットフォーム用のバイナリを出荷しません。[システム要件](/docs/ja/setup#system-requirements)を参照してください。FreeBSD では、インストーラーはプラットフォームをサポートされていないと報告します。v2.1.205 より前では、FreeBSD を Linux として扱い、実行できないバイナリをダウンロードしました。
* **企業 npm ミラーがプラットフォームパッケージを欠落している。** レジストリがメタパッケージに加えて 8 つの `@anthropic-ai/claude-code-*` プラットフォームパッケージすべてをミラーしていることを確認してください。

<h3 id="npm-enotempty-during-update-or-reinstall">
  npm `ENOTEMPTY` エラー（更新または再インストール中）
</h3>

既存のインストール上で `npm install -g @anthropic-ai/claude-code` を実行すると、npm は古いパッケージディレクトリを移動しようとしている間に失敗する可能性があります。

```text theme={null}
npm error code ENOTEMPTY
npm error syscall rename
npm error path /home/you/.nvm/versions/node/v22.13.1/lib/node_modules/@anthropic-ai/claude-code
npm error dest /home/you/.nvm/versions/node/v22.13.1/lib/node_modules/@anthropic-ai/.claude-code-tVWAnUUt
npm error errno -39
npm error ENOTEMPTY: directory not empty, rename '...'
```

`npm error path` 行は npm が移動できなかったディレクトリに名前を付けます。そのディレクトリと、以前の中断された実行が残す可能性のある隣接する `.claude-code-*` ディレクトリを削除してください。以下のコマンドは `npm root -g` でグローバルパッケージディレクトリを見つけます。`npm error path` 行が名前を付けるディレクトリが `npm root -g` が出力するディレクトリの下にない場合（例えば nvm でノードバージョンを切り替えた場合）、エラーが名前を付けるディレクトリを削除してください。

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    rm -rf "$(npm root -g)/@anthropic-ai/claude-code"
    ```

    次に、残っているテンポラリディレクトリを削除します。Zsh が `no matches found` を出力する場合、削除するものはありませんでした。

    ```bash theme={null}
    rm -rf "$(npm root -g)/@anthropic-ai/.claude-code-"*
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Recurse -Force "$(npm root -g)/@anthropic-ai/claude-code", "$(npm root -g)/@anthropic-ai/.claude-code-*"
    ```
  </Tab>
</Tabs>

次に再インストールします。

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

`claude --version` で確認します。これは `2.1.211 (Claude Code)` などのバージョン番号を出力します。

<h2 id="login-and-authentication">
  ログインと認証
</h2>

これらのセクションはログイン失敗、OAuth エラー、およびトークンの問題に対処します。

<h3 id="reset-your-login">
  ログインをリセットする
</h3>

ログインが失敗し、原因が明らかでない場合、クリーンな再認証がほとんどの場合を解決します：

1. `/logout` を実行して完全にサインアウトしてください
2. Claude Code を閉じてください
3. `claude` で再起動して、認証プロセスを再度完了してください

ログイン中にブラウザが自動的に開かない場合は、`c` を押して OAuth URL をクリップボードにコピーしてから、手動でブラウザに貼り付けてください。これは、URL が狭いまたは SSH ターミナルで行をまたいでラップされ、直接クリックできない場合にも機能します。

<h3 id="oauth-error-invalid-code">
  OAuth エラー：無効なコード
</h3>

`OAuth error: Invalid code. Please make sure the full code was copied` が表示される場合、ログインコードが期限切れになったか、コピー貼り付け中に切り詰められました。

**解決策：**

* ブラウザが開いた後、Enter キーを押して迅速にログインを完了してください
* ブラウザが自動的に開かない場合は、`c` を入力して完全な URL をコピーしてください
* リモート/SSH セッションを使用している場合、ブラウザは間違ったマシンで開く可能性があります。ターミナルに表示されている URL をコピーして、代わりにローカルブラウザで開いてください。

<h3 id="403-forbidden-after-login">
  ログイン後の 403 Forbidden
</h3>

ログイン後に `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}` が表示される場合：

* **Claude Pro/Max ユーザー**：[claude.ai/settings](https://claude.ai/settings) でサブスクリプションがアクティブであることを確認してください
* **Anthropic Console ユーザー**：アカウントに「Claude Code」または「Developer」ロールがあることを確認してください。管理者は Anthropic Console の設定 → メンバーで割り当てます。
* **プロキシの背後**：企業プロキシは API リクエストに干渉する可能性があります。[ネットワーク設定](/docs/ja/network-config) を参照してプロキシセットアップを確認してください。

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  このオーガニゼーションはアクティブなサブスクリプションで無効になっています
</h3>

アクティブな Claude サブスクリプションがあるにもかかわらず `API Error: 400 ... "This organization has been disabled"` が表示される場合、`ANTHROPIC_API_KEY` 環境変数がサブスクリプションをオーバーライドしています。これは、前の雇用主またはプロジェクトからの古い API キーがシェルプロファイルに設定されている場合に一般的に発生します。

`ANTHROPIC_API_KEY` が存在し、承認されている場合、Claude Code はサブスクリプションの OAuth 認証情報の代わりにそのキーを使用します。`-p` フラグを使用した非対話モードでは、存在する場合、キーは常に使用されます。[認証の優先順位](/docs/ja/authentication#authentication-precedence) を参照して、完全な解決順序を確認してください。

代わりにサブスクリプションを使用するには、環境変数を設定解除し、シェルプロファイルから削除してください：

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    unset ANTHROPIC_API_KEY
    claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item Env:ANTHROPIC_API_KEY
    claude
    ```
  </Tab>
</Tabs>

`~/.zshrc`、`~/.bashrc`、または `~/.profile` で `export ANTHROPIC_API_KEY=...` 行を確認して削除し、変更を永続的にしてください。Windows では、`$PROFILE` の PowerShell プロファイルと `ANTHROPIC_API_KEY` のユーザー環境変数を確認してください。Claude Code 内で `/status` を実行して、どの認証方法がアクティブであるかを確認してください。

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  WSL2、SSH、またはコンテナでの OAuth ログイン失敗
</h3>

Claude Code が WSL2 で実行されている場合、SSH 経由でリモートマシンで実行されている場合、またはコンテナ内で実行されている場合、ブラウザは通常、別のホストで開き、そのリダイレクトは Claude Code のローカルコールバックサーバーに到達できません。サインイン後、ブラウザは自動的にリダイレクトされるのではなく、ログインコードを表示します。ターミナルの `Paste code here if prompted` プロンプトにそのコードを貼り付けてログインを完了してください。

WSL2 からブラウザがまったく開かない場合は、`BROWSER` 環境変数を Windows ブラウザパスに設定してください：

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

または、対話型ログインプロンプトで `c` を押して OAuth URL をコピーするか、`claude auth login` が出力する URL をコピーして、ローカルマシンのブラウザで開いてください。

対話型プロンプトにコードを貼り付けても何もしない場合、ターミナルの貼り付けバインディングはおそらく入力フィールドに到達していません。ターミナルの別の貼り付けショートカット（Windows Terminal では右クリックまたは Shift+Insert）を試すか、標準入力から貼り付けられたコードを読み取る `claude auth login` を使用してください：

```bash theme={null}
claude auth login
```

このフォールバックは、ネイティブ Windows またはコードを対話型プロンプトに貼り付けるのが失敗するその他のターミナルにも適用されます。

<h3 id="not-logged-in-or-token-expired">
  ログインしていないか、トークンが期限切れ
</h3>

Claude Code がセッション後に再度ログインするよう求める場合、OAuth トークンが期限切れになった可能性があります。

`/login` を実行して再認証してください。これが頻繁に発生する場合は、トークン検証が正しいタイムスタンプに依存するため、システムクロックが正確であることを確認してください。

1 台のマシン上の並列セッションは保存されたログインを共有し、その更新を調整して、1 つのプロセスだけが一度にトークンを更新するようにします。v2.1.211 より前では、マシンをスリープから起動すると、2 つのセッションが同じトークンで更新される可能性があり、これは保存されたログインを取り消し、すべてのオープンセッションに一度にログインするよう求めました。

macOS では、Claude Code は認証情報をログイン Keychain に保存します。Keychain が書き込みを拒否する場合（SSH セッションでロックされている場合、またはパスワードがアカウントパスワードと同期していない場合など）、Claude Code は代わりにログインをプレーンテキスト `~/.claude/.credentials.json` ファイルに保存します。Keychain が再び書き込み可能になるまで、API キーを作成する Console ログインは失敗します。

Keychain を再び書き込み可能にし、ログインを暗号化された Keychain に戻すには：

<Steps>
  <Step title="Keychain アクセスを確認する">
    `claude doctor` を実行して Keychain アクセスを確認してください。Keychain が書き込みを拒否する場合、レポートは `macOS Keychain is not writable` で始まる警告をリストし、その後に推奨される修正を示します。レポートに Keychain 警告がリストされていない場合、Keychain は書き込み可能であり、最後のステップにスキップできます。
  </Step>

  <Step title="Keychain をロック解除する">
    ```bash theme={null}
    security unlock-keychain ~/Library/Keychains/login.keychain-db
    ```

    コマンドが Keychain パスワードを要求したら入力し、`claude doctor` を再度実行してください。ロック解除が成功した場合、レポートは Keychain 警告をリストしなくなります。
  </Step>

  <Step title="ロック解除が役に立たない場合は Keychain パスワードを再同期する">
    Keychain Access を開き、`login` キーチェーンを選択して、**編集 > キーチェーン「login」のパスワードを変更** を選択してアカウントパスワードと再同期してください。その後、`claude doctor` を再度実行してください。レポートが Keychain 警告をリストしなくなったら、次のステップに進んでください。
  </Step>

  <Step title="ログアウトしてから再度ログインする">
    Keychain が再び書き込み可能になったら、Claude Code は次回認証情報を書き込むときに認証情報を Keychain に戻します。今すぐ強制するには、`/logout` を実行してから `/login` を実行してください。ログアウトすると、プレーンテキストファイルの内容、保存された MCP サーバーログイン、プラグイン機密値を含むすべての保存された認証情報が削除されるため、その後 MCP サーバーを再度認可し、プラグインシークレットを再度入力することを期待してください。再度ログインすると、ログインが Keychain に保存されます。
  </Step>
</Steps>

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Bedrock、Agent Platform、または Foundry 認証情報が読み込まれない
</h3>

Claude Code をクラウドプロバイダーを使用するように設定し、Amazon Bedrock で `Could not load credentials from any providers`、Google Cloud の Agent Platform で `Could not load the default credentials`、または Microsoft Foundry で `ChainedTokenCredential authentication failed` が表示される場合、クラウドプロバイダー CLI は現在のシェルで認証されていない可能性があります。

Amazon Bedrock の場合、AWS 認証情報が有効であることを確認してください：

```bash theme={null}
aws sts get-caller-identity
```

Google Cloud の Agent Platform の場合、`ANTHROPIC_VERTEX_PROJECT_ID` と `CLOUD_ML_REGION` がシェルに設定されていることを確認してから、アプリケーションのデフォルト認証情報を設定してください：

```bash theme={null}
gcloud auth application-default login
```

Microsoft Foundry の場合、`ANTHROPIC_FOUNDRY_API_KEY` が設定されていることを確認するか、Azure CLI でサインインして、デフォルト認証情報チェーンがアカウントを見つけられるようにしてください：

```bash theme={null}
az login
```

認証情報がターミナルで機能するが VS Code または JetBrains 拡張機能では機能しない場合、IDE プロセスはおそらくシェル環境を継承していません。IDE 独自の設定でプロバイダー環境変数を設定するか、既にエクスポートされているターミナルから IDE を起動してください。

完全なプロバイダーセットアップについては、[Amazon Bedrock](/docs/ja/amazon-bedrock)、[Google Cloud の Agent Platform](/docs/ja/google-vertex-ai)、または [Microsoft Foundry](/docs/ja/microsoft-foundry) を参照してください。

<h2 id="still-stuck">
  まだ立ち往生している
</h2>

上記のいずれも問題を解決しない場合：

1. [GitHub リポジトリ](https://github.com/anthropics/claude-code/issues)で既知の問題を確認するか、オペレーティングシステム、実行したインストールコマンド、および完全なエラー出力を含めて新しい問題を開いてください
2. `claude --version` が機能するが他に何か問題がある場合は、`claude doctor` を実行して自動診断レポートを取得してください
3. セッションを開始できる場合は、Claude Code 内で `/feedback` を使用して問題を報告してください
4. 問題がインストールではなくアカウントに関するものである場合（ログインループ、認識されないサブスクリプション、無効な組織など）は、Anthropic サポートにお問い合わせください。[claude.ai](https://claude.ai)（Console ユーザーの場合：[platform.claude.com](https://platform.claude.com)）にサインインし、左下のイニシャルをクリックして、**Get help** を選択してください。完全なフローについては、[How to get support](https://support.claude.com/en/articles/9015913-how-to-get-support)を参照してください。
