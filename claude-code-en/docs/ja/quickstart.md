> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# クイックスタート

> Claude Code へようこそ！

このクイックスタートガイドを使用すれば、数分で AI を活用したコーディング支援を利用できます。このガイドを終了する頃には、一般的な開発タスクに Claude Code を使用する方法を理解できるようになります。

<h2 id="before-you-begin">
  始める前に
</h2>

以下を確認してください：

* ターミナルまたはコマンドプロンプトが開いている
  * ターミナルを使用したことがない場合は、[ターミナルガイド](/docs/ja/terminal-guide)をご覧ください
* 作業するコードプロジェクトがある
* [Claude サブスクリプション](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq)（Pro、Max、Team、または Enterprise）、[Claude Console](https://platform.claude.com/) アカウント、または[サポートされているクラウドプロバイダー](/docs/ja/third-party-integrations)経由のアクセスがある

<Note>
  このガイドはターミナル CLI について説明しています。Claude Code は[ウェブ](https://claude.ai/code)、[デスクトップアプリ](/docs/ja/desktop)、[VS Code](/docs/ja/vs-code) および [JetBrains IDE](/docs/ja/jetbrains)、[Slack](/docs/ja/slack)、および [GitHub Actions](/docs/ja/github-actions) と [GitLab](/docs/ja/gitlab-ci-cd) を使用した CI/CD でも利用できます。[すべてのインターフェース](/docs/ja/overview#use-claude-code-everywhere)を参照してください。
</Note>

<h2 id="step-1-install-claude-code">
  ステップ 1：Claude Code をインストールする
</h2>

Claude Code をインストールするには、以下のいずれかの方法を使用してください。

<Tabs>
  <Tab title="ネイティブインストール（推奨）">
    **macOS、Linux、WSL：**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell：**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD：**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    `The token '&&' is not a valid statement separator` というエラーが表示される場合は、CMD ではなく PowerShell を使用しています。`'irm' is not recognized as an internal or external command` というエラーが表示される場合は、PowerShell ではなく CMD を使用しています。PowerShell を使用している場合、プロンプトに `PS C:\` と表示され、CMD を使用している場合は `PS` なしで `C:\` と表示されます。

    インストールコマンドが `syntax error near unexpected token '<'`、`403`、またはその他の curl エラーで失敗する場合は、[インストールのトラブルシューティング](/docs/ja/troubleshoot-install#find-your-error)を参照して、エラーを修正方法に照合し、代替インストール方法を確認してください。

    [Git for Windows](https://git-scm.com/downloads/win) は、Claude Code が Bash ツールを使用できるようにネイティブ Windows で推奨されます。Git for Windows がインストールされていない場合、Claude Code はシェルツールとして PowerShell を代わりに使用します。WSL セットアップは Git for Windows を必要としません。

    <Info>
      ネイティブインストールは、最新バージョンに保つために自動的にバックグラウンドで更新されます。
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew は 2 つの cask を提供しています。`claude-code` は安定リリースチャネルを追跡しており、通常は約 1 週間遅れており、大きな回帰を伴うリリースをスキップします。`claude-code@latest` は最新チャネルを追跡し、新しいバージョンが出荷されるとすぐに受け取ります。

    <Info>
      Homebrew インストールは自動更新されません。インストールした cask に応じて、`brew upgrade claude-code` または `brew upgrade claude-code@latest` を実行して、最新の機能とセキュリティ修正を取得してください。
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet インストールは自動更新されません。最新の機能とセキュリティ修正を取得するために、定期的に `winget upgrade Anthropic.ClaudeCode` を実行してください。
    </Info>
  </Tab>
</Tabs>

また、Debian、Fedora、RHEL、Alpine で [apt、dnf、または apk](/docs/ja/setup#install-with-linux-package-managers) を使用してインストールすることもできます。

インストールが正常に機能したことを確認するには、以下を実行してください：

```bash theme={null}
claude --version
```

このコマンドは、バージョン番号の後に `(Claude Code)` を出力します。

<h2 id="step-2-log-in-to-your-account">
  ステップ 2：アカウントにログインする
</h2>

Claude Code を使用するにはアカウントが必要です。`claude` コマンドでインタラクティブセッションを開始すると、初回使用時にログインするよう求められます：

```bash theme={null}
claude
```

Claude サブスクリプションまたは Console アカウントの場合は、プロンプトに従ってブラウザで認証を完了してください。`ANTHROPIC_API_KEY` 環境変数を設定している場合、Claude Code はログインプロンプトをスキップし、代わりにキーを承認するよう求めます。後でアカウントを切り替えるか再認証するには、実行中のセッション内で `/login` と入力します：

```text wrap theme={null}
/login
```

以下のいずれかのアカウントタイプを使用してログインできます：

* [Claude Pro、Max、Team、または Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login)（推奨）
* [Claude Console](https://platform.claude.com/)（プリペイドクレジット付き API アクセス）。初回ログイン時に、コスト追跡を一元化するために「Claude Code」ワークスペースが Console に自動的に作成されます。
* [Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry](/docs/ja/third-party-integrations)（エンタープライズクラウドプロバイダー）
* 組織が実行している自己ホスト型の [Claude apps gateway](/docs/ja/claude-apps-gateway)：管理者がゲートウェイ URL を事前に設定し、`/login` で **Cloud gateway** 画面が直接開き、企業 SSO でサインインできます

ログイン後、認証情報が保存され、再度ログインする必要はありません。詳細は [認証情報管理](/docs/ja/authentication#credential-management) をご覧ください。

<h2 id="step-3-start-your-first-session">
  ステップ 3：最初のセッションを開始する
</h2>

任意のプロジェクトディレクトリでターミナルを開き、Claude Code を開始します：

```bash theme={null}
cd /path/to/your/project
claude
```

`/path/to/your/project` を、作業したいプロジェクトのパスに置き換えてください。

Claude Code プロンプトが表示され、バージョン、現在のモデル、および作業ディレクトリが上に表示されます。利用可能なコマンドについては `/help` を入力するか、前の会話を続行するには `/resume` を入力します。

<h2 id="step-4-ask-your-first-question">
  ステップ 4：最初の質問をする
</h2>

コードベースを理解することから始めましょう。以下のコマンドのいずれかを試してください：

```text wrap theme={null}
what does this project do?
```

Claude はファイルを分析して概要を提供します。より具体的な質問をすることもできます：

```text wrap theme={null}
what technologies does this project use?
```

```text wrap theme={null}
where is the main entry point?
```

```text wrap theme={null}
explain the folder structure
```

Claude 自体の機能について質問することもできます：

```text wrap theme={null}
what can Claude Code do?
```

```text wrap theme={null}
how do I create custom skills in Claude Code?
```

```text wrap theme={null}
can Claude Code work with Docker?
```

<Note>
  Claude Code は必要に応じてプロジェクトファイルを読み込みます。コンテキストを手動で追加する必要はありません。
</Note>

<h2 id="step-5-make-your-first-code-change">
  ステップ 5：最初のコード変更を行う
</h2>

次に、Claude Code に実際のコーディングを行わせましょう。簡単なタスクを試してください：

```text wrap theme={null}
メインファイルに hello world 関数を追加してください
```

Claude Code は適切なファイルを見つけて、変更内容を表示します。変更を行う前に確認を求める場合は、**Yes** を選択して承認してください。

Auto モードは、Pro、Max、Team プランのインタラクティブターミナルセッションの[組み込みの開始権限モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)です。分類器があなたの代わりにアクションをレビューし、Claude はほとんどのファイルを編集し、ほとんどのコマンドをあなたに尋ねることなく実行します。その他のプランでは、Manual モードが組み込みの開始権限モードです。インストール直後に開始するセッションについては、[インストールまたはアップグレード後の最初のセッション](/docs/ja/env-vars#first-session-after-an-install-or-upgrade)を参照してください。

<Note>
  設定またはお客様の組織が異なる開始権限モードを設定できます。[セッションが開始する権限モード](/docs/ja/permission-modes#which-mode-a-session-starts-in)に、その内容が記載されています。いつでも `Shift+Tab` を押して、現在のセッションの権限モードを切り替えることができます。
</Note>

<h2 id="step-6-use-git-with-claude-code">
  ステップ 6：Claude Code で Git を使用する
</h2>

Claude Code は Git 操作を会話形式にします：

```text wrap theme={null}
どのファイルを変更しましたか？
```

```text wrap theme={null}
説明的なメッセージで変更をコミットしてください
```

より複雑な Git 操作を求めることもできます：

```text wrap theme={null}
feature/quickstart という名前の新しいブランチを作成してください
```

```text wrap theme={null}
最後の 5 つのコミットを表示してください
```

```text wrap theme={null}
マージコンフリクトの解決を手伝ってください
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  ステップ 7：バグを修正するか機能を追加する
</h2>

Claude はデバッグと機能実装に長けています。

自然言語で実現したいことを説明します：

```text wrap theme={null}
ユーザー登録フォームに入力検証を追加してください
```

または既存の問題を修正します：

```text wrap theme={null}
ユーザーが空のフォームを送信できるバグがあります。修正してください
```

Claude Code は以下を実行します：

* 関連するコードを見つける
* コンテキストを理解する
* ソリューションを実装する
* 利用可能な場合はテストを実行する

<h2 id="step-8-test-out-other-common-workflows">
  ステップ 8：他の一般的なワークフローを試す
</h2>

Claude と連携する方法は多数あります：

**コードをリファクタリングする**

```text wrap theme={null}
認証モジュールをリファクタリングして、コールバックの代わりに async/await を使用するようにしてください
```

**テストを書く**

```text wrap theme={null}
計算機関数のユニットテストを書いてください
```

**ドキュメントを更新する**

```text wrap theme={null}
インストール手順で README を更新してください
```

**コードレビュー**

```text wrap theme={null}
変更をレビューして改善を提案してください
```

<Tip>
  有能な同僚と話すように Claude と話してください。実現したいことを説明すれば、それを実現するのに役立ちます。
</Tip>

<h2 id="essential-commands">
  必須コマンド
</h2>

日常的に使用する最も重要なコマンドは以下の通りです。シェルコマンドはターミナルから実行して Claude Code を開始または再開します。セッションコマンドは Claude Code 起動後に実行します。

**シェルコマンド**

| コマンド                | 機能                         | 例                                   |
| ------------------- | -------------------------- | ----------------------------------- |
| `claude`            | インタラクティブモードを開始する           | `claude`                            |
| `claude "task"`     | 初期プロンプト付きでインタラクティブモードを開始する | `claude "fix the build error"`      |
| `claude -p "query"` | 1 回限りのクエリを実行してから終了する       | `claude -p "explain this function"` |
| `claude -c`         | 現在のディレクトリで最新の会話を続行する       | `claude -c`                         |
| `claude -r`         | 前の会話を再開する                  | `claude -r`                         |

**セッションコマンド**

| コマンド                   | 機能                | 例        |
| ---------------------- | ----------------- | -------- |
| `/clear`               | 会話履歴をクリアする        | `/clear` |
| `/help`                | 利用可能なコマンドを表示する    | `/help`  |
| `/exit` または Ctrl+D 2 回 | Claude Code を終了する | `/exit`  |

シェルコマンドの完全なリストについては [CLI リファレンス](/docs/ja/cli-reference)を、セッションコマンドの完全なリストについては [コマンドリファレンス](/docs/ja/commands)を参照してください。

<h2 id="pro-tips-for-beginners">
  初心者向けのプロのヒント
</h2>

詳細については、[ベストプラクティス](/docs/ja/best-practices)と[一般的なワークフロー](/docs/ja/common-workflows)を参照してください。

<AccordionGroup>
  <Accordion title="リクエストを具体的にする">
    代わりに：'バグを修正してください'

    試してください：'ユーザーが間違った認証情報を入力した後に空白の画面が表示されるログインバグを修正してください'
  </Accordion>

  <Accordion title="段階的な指示を使用する">
    複雑なタスクをステップに分割します：

    ```text wrap theme={null}
    1. ユーザープロファイル用の新しいデータベーステーブルを作成する
    2. ユーザープロファイルを取得および更新するための API エンドポイントを作成する
    3. ユーザーが自分の情報を表示および編集できるウェブページを構築する
    ```
  </Accordion>

  <Accordion title="Claude に最初に探索させる">
    変更を加える前に、Claude にコードを理解させます：

    ```text wrap theme={null}
    データベーススキーマを分析する
    ```

    ```text wrap theme={null}
    英国の顧客によって最も頻繁に返品される製品を表示するダッシュボードを構築する
    ```
  </Accordion>

  <Accordion title="ショートカットで時間を節約する">
    * `/` を入力してすべてのコマンドとスキルを表示する
    * Tab キーでコマンド補完を使用する
    * ↑ キーでコマンド履歴を表示する
    * `Shift+Tab` を押して権限モードをサイクルさせる
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  次のステップ
</h2>

基本を学習したので、より高度な機能を探索してください：

<CardGroup cols={2}>
  <Card title="Claude Code の仕組み" icon="microchip" href="/docs/ja/how-claude-code-works">
    agentic ループ、組み込みツール、および Claude Code がプロジェクトと相互作用する方法を理解する
  </Card>

  <Card title="ベストプラクティス" icon="star" href="/docs/ja/best-practices">
    効果的なプロンプティングとプロジェクト設定でより良い結果を得る
  </Card>

  <Card title="一般的なワークフロー" icon="graduation-cap" href="/docs/ja/common-workflows">
    一般的なタスクのステップバイステップガイド
  </Card>

  <Card title="Claude Code を拡張する" icon="puzzle-piece" href="/docs/ja/features-overview">
    CLAUDE.md、スキル、フック、MCP などでカスタマイズする
  </Card>
</CardGroup>

<h2 id="getting-help">
  ヘルプを取得する
</h2>

* **Claude Code 内**：`/help` を入力するか、「how do I」という質問をする
* **ドキュメント**：ここにいます！他のガイドを参照してください
* **コース**：[Claude Code 101](https://academy.claude.com/courses/claude-code-101) と [Claude Academy](https://academy.claude.com/) の他の無料のセルフペースコースを受講する
* **コミュニティ**：[Discord](https://www.anthropic.com/discord) に参加してヒントとサポートを得る
