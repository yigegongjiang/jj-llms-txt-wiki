> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# オートモードの設定

> オートモード分類器に、組織が信頼するリポジトリ、バケット、ドメインを指定します。環境コンテキストを設定し、デフォルトのブロックおよび許可ルールをオーバーライドし、オートモード CLI サブコマンドで有効な設定を検査します。

[オートモード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)を使用すると、Claude Code は、ツール呼び出しを分類器にルーティングして、不可逆的、破壊的、または環境外を対象とした操作をブロックすることで、定期的な権限プロンプトなしで実行できます。拒否および明示的な質問ルールは分類器の前に評価され、引き続きブロックまたはプロンプトを表示します。`autoMode` 設定ブロックを使用して、その分類器に組織が信頼するリポジトリ、バケット、ドメインを指定し、定期的な内部操作のブロックを停止させます。

<Note>
  オートモードは、Anthropic API、[AWS 上の Claude Platform](/docs/ja/claude-platform-on-aws)、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、およびサインイン済みの[Claude アプリゲートウェイ](/docs/ja/claude-apps-gateway)セッションを含む、すべてのプロバイダーのすべてのユーザーが利用できます。Claude Code がアカウントでオートモードが利用できないと報告する場合は、[完全な要件](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)を確認してください。これには、サポートされているモデルと Team および Enterprise プランでの組織レベルの制御も含まれます。v2.1.158 から v2.1.206 では、Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry、および Claude アプリゲートウェイセッション上のオートモードは `CLAUDE_CODE_ENABLE_AUTO_MODE=1` の設定が必要でしたが、v2.1.207 ではその要件が削除されました。
</Note>

デフォルトでは、分類器は作業ディレクトリと現在のリポジトリの設定されたリモートのみを信頼します。会社のソース管理組織へのプッシュやチームクラウドバケットへの書き込みなどのアクションは、`autoMode.environment` に追加するまでブロックされます。

セッションがオートモードになる方法とデフォルトでブロックされるものについては、[権限モード ページのオートモード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)を参照してください。このページは設定リファレンスです。

このページでは、以下の方法について説明します。

* [`permissions.ask`](#add-a-human-checkpoint)を使用してプッシュおよびプルリクエストの人間チェックポイントを追加する
* [CLAUDE.md、ユーザー設定、管理設定全体でルールを設定する場所を選択する](#where-the-classifier-reads-configuration)
* [`autoMode.environment`](#define-trusted-infrastructure)を使用して信頼できるインフラストラクチャを定義する
* [`/auto-mode-setup`](#generate-environment-entries)を使用して環境エントリを生成する
* [デフォルトがパイプラインに適さない場合、ブロックおよび許可ルールをオーバーライドする](#override-the-block-and-allow-rules)
* [`/permissions`](#edit-rules-from-permissions)からルールを編集する（設定ファイルを開かずに）
* [`autoMode.classifyAllShell`](#route-all-shell-commands-through-the-classifier)を使用してすべてのシェルコマンドを分類器にルーティングする
* [`claude auto-mode`](#inspect-the-defaults-and-your-effective-config)サブコマンドで有効な設定を検査する
* [拒否を確認](#review-denials)して、次に何を追加するかを把握する

<h2 id="common-boundaries">
  一般的な境界
</h2>

オートモードでは、作業中のリポジトリのあらゆるブランチ（デフォルトブランチを含む）へのプッシュが許可され、デフォルトではプルリクエストの作成も可能です。`production`、`release`、`gh-pages` など、デプロイまたは公開ターゲットとしてマークされた非デフォルトブランチは、このデフォルトの対象外です。分類器はそこへのプッシュを独自の条件で判定し、本番環境へのデプロイとして扱います。プッシュの内容も引き続きチェックされるため、強制プッシュ、コミットへのシークレット入力、または CI やデプロイパイプラインの実行時にシークレットをリポジトリ外に送信する変更は、ブロックされたままです。

<Info>v2.1.211 より前では、分類器は作業ブランチ、Claude が作成したブランチ、およびデフォルトブランチへの定期的なプッシュのみを許可していました。</Info>

Claude のプッシュおよびプルリクエストコマンドの前に人間によるチェックポイントが必要な場合は、権限ルールを追加してください。[以下のレシピ](#add-a-human-checkpoint)では、他のすべてのアクションに対してオートモードを有効に保ちます。

<h3 id="add-a-human-checkpoint">
  人間によるチェックポイントを追加する
</h3>

最も直接的なメカニズムは [`permissions.ask`](/docs/ja/permissions#permission-rule-syntax) です。以下のようなコンテンツスコープの ask ルールは、分類器の前に評価され、オートモードでも常に権限プロンプトを強制します。これは、明示的な ask ルールがそのアクションについてプロンプトを表示する意図を示しているためです。[設定](/docs/ja/settings#where-settings-live)にルールを追加してください。

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

これらのルールは `git push` または `gh pr create` で始まるコマンドに一致します。Claude が別の方法で記述したプッシュ（例：`git -C <dir> push` または `git -c <key>=<value> push`）は、[ルールに一致しない](/docs/ja/permissions#bash-rule-limits)ため、チェックポイントされません。完全なコマンドテキストを検査するチェックポイントの場合は、[PreToolUse フック](/docs/ja/hooks#pretooluse)を追加してください。

境界がどの程度厳密である必要があるかに応じて、メカニズムを選択してください。

| 境界             | メカニズム                       | オートモードでの動作                                                                                                                                            |
| :------------- | :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| アクション前にプロンプト表示 | `permissions.ask`           | 上記のレシピのようなコンテンツスコープのルールに一致するコマンドに対して常にプロンプトを表示します。分類器は一致するアクションを自動承認できません。                                                                            |
| アクションを実行しない    | `permissions.deny`          | 分類器が参照される前にブロックします。分類器もユーザーの意図も、これをオーバーライドできません。                                                                                                      |
| このセッション限定の境界   | 「レビューするまでプッシュしない」のように会話で述べる | 分類器は一致するアクションをブロックしますが、[コンテキストコンパクション](/docs/ja/costs#reduce-token-usage)がそのステートメントを含むメッセージを削除すると、境界が失われる可能性があります。永続的な保証のために ask ルールまたは deny ルールを使用してください。 |

<h2 id="where-the-classifier-reads-configuration">
  分類器が設定を読み込む場所
</h2>

分類器は Claude 自体が読み込む同じ [CLAUDE.md](/docs/ja/memory) コンテンツを読み込むため、プロジェクトの CLAUDE.md に「force push を絶対にしない」のような指示があれば、Claude と分類器の両方を同時に制御します。プロジェクト規約と動作ルールについては、ここから始めてください。

複数のプロジェクトに適用されるルール（信頼できるインフラストラクチャや組織全体の拒否ルールなど）については、`autoMode` 設定ブロックを使用してください。分類器は以下のスコープから `autoMode` を読み込みます。

| スコープ                          | ファイル                                   | 用途                           |
| :---------------------------- | :------------------------------------- | :--------------------------- |
| 1 人の開発者                       | `~/.claude/settings.json`              | 個人の信頼できるインフラストラクチャ           |
| 組織全体                          | [マネージド設定](/docs/ja/server-managed-settings) | すべての開発者に配布される信頼できるインフラストラクチャ |
| `--settings` フラグまたは Agent SDK | インライン JSON                             | 自動化のための呼び出しごとのオーバーライド        |

分類器は `.claude/settings.json` または `.claude/settings.local.json` のプロジェクト設定から `autoMode` を読み込みません。両方のファイルはリポジトリディレクトリに存在するため、チェックインされたリポジトリまたはビルドステップが独自の許可ルールを注入する可能性があります。v2.1.207 より前は、分類器は `.claude/settings.local.json` も読み込んでいました。そのファイルの `autoMode` ブロックを `~/.claude/settings.json` に移動してください。`.claude/settings.local.json` を除外することで、リポジトリがファイルをコミットしたり、ローカルツールまたはビルドステップが書き込んだりする場合も対応できます。

各スコープからのエントリは結合されます。開発者は個人的なエントリで `environment`、`allow`、`soft_deny`、`hard_deny` を拡張できますが、マネージド設定が提供するエントリを削除することはできません。許可ルールは分類器内のソフトブロックルールの例外として機能するため、開発者が追加した `allow` エントリは組織の `soft_deny` エントリをオーバーライドできます。組み合わせは加算的であり、ハードポリシー境界ではありません。

<Note>
  分類器は [権限システム](/docs/ja/permissions) の後に実行される 2 番目のゲートです。ユーザーの意図または分類器の設定に関係なく、絶対に実行してはいけないアクションについては、マネージド設定で `permissions.deny` を使用してください。これは分類器が参照される前にアクションをブロックし、オーバーライドできません。
</Note>

<h2 id="define-trusted-infrastructure">
  信頼できるインフラストラクチャを定義する
</h2>

ほとんどの組織では、`autoMode.environment` が設定する必要がある唯一のフィールドです。これは分類器にどのリポジトリ、バケット、ドメインが信頼できるかを伝えます。分類器はこれを使用して「外部」が何を意味するかを決定するため、リストに記載されていない宛先は潜在的なデータ流出ターゲットです。

Claude Code v2.1.198 以降、`claude auto-mode defaults` は 3 種類の環境エントリを出力します。v2.1.195 より前のバージョンは最初の 5 つの信頼スロットのみを出力します。

* **コンテキストスロット**: 組織、スタック、セキュリティ体制を説明し、分類器がコンテキスト内の他のルールを読み取れるようにします。各スロットはデフォルトで `None configured` または次に示す保守的な仮定に設定されます。
  * **Organization（組織）**
  * **Claude Code の主な用途**: ソフトウェア開発がデフォルト
  * **クラウドプロバイダー**
  * **リポジトリの可視性**: リポジトリはリモートホストと名前が別途示さない限りプライベートと見なされます。または分類器が会話の前の段階で読み取った可視性チェックでそれがパブリックであることが示されている場合。

    Claude Code 自体が送信する分類器リクエストでは、分類器はメッセージと Claude が実行するコマンドを読み取り、その出力は読み取りません。証拠は分類器が読み取ることができるもの（リポジトリをパブリックとして名前を付けた独自のメッセージなど）である必要があります。`gh repo view` の出力だけではそこに到達しません。トランスクリプト証拠チェックには Claude Code v2.1.200 以降が必要です
  * **内部共有 / スニペットホスティング**: パブリックペーストおよび gist サービスは、名前を付けるまで信頼境界の外側として扱われます
  * **組織固有の CLI**
  * **シークレット管理**
  * **CI/CD デプロイターゲット**
  * **ネットワーク体制**
  * **ホスト隔離**: デフォルトは、オープンインターネットを備えた通常の開発者マシンまたは CI ランナーです。Claude Code がコンテナ、VM、またはポッド内で実行され、エグレス許可リストまたは接触してはいけないネイバーがある場合は、許可されたホスト、クラウドメタデータエンドポイントに到達可能かどうか、およびタスクが使用するクラウドプロジェクト、クラスタ、またはレジストリと使用する ID を名前付けします。このエントリがその ID を名前付けするまで、分類器は[ホストの独自の認証情報に対するリクエストをブロック](/docs/ja/permission-modes#what-the-classifier-blocks-by-default)します。Claude Code v2.1.257 以降が必要です
  * **保護されたデプロイメント名前空間 / 環境**: 名前を付けるまで「機密リモートターゲット」ヒューリスティックにフォールバックします
  * **データ保持 / 機密解除**
* **信頼スロット**: 分類器が境界内として扱うものを名前付けします。スロットは「信頼できるリポジトリ」、「ソース管理」、「信頼できる内部ドメイン」、「信頼できるクラウドバケット」、「主要な内部サービス」、および「内部パッケージレジストリ」です。リポジトリおよびソース管理エントリはデフォルトで作業リポジトリとその設定されたリモートに設定されます。他のすべての信頼スロットはデフォルトで `None configured` に設定されるため、追加するまで他に信頼できるものはありません。リポジトリの可視性は機密資料のみをスコープします。プライベートリポジトリは機密資料の許容可能な宛先ですが、リポジトリをプライベートにしてもシークレット、個人データ、または信頼されたデータがそこにクリアされることはなく、分類器は作業リポジトリの外側からポート、リポイント、または最初に読み取られたコンテンツをそのリポジトリ独自の作業として扱いません。このスコーピングには Claude Code v2.1.203 以降が必要です。
* **感度スロット**: 保護ルールが高リスクとして扱うものを名前付けします。スロットは「機密データの場所とオーディエンス」、「機密リモートターゲット」、および「保護された IaC スコープ」です。各スロットはデフォルトで広範なヒューリスティック（名前に `prod` または `production` を含むホストまたは名前空間を機密リモートターゲットとして扱うなど）に設定されるため、保護ルールは何も設定する前からアクティブです。感度スロットに具体的なターゲットを名前付けすると、これらのルールはヒューリスティックではなく名前付けされたターゲットに適用されます。

<Info>v2.1.211 より前では、コンテキストスロットには、`main` と `master` を他の名前を付けるまで保護されたものとして扱う「デフォルト / 保護されたブランチ」エントリも含まれていました。v2.1.211 はこれを削除しました。[作業しているリポジトリのどのブランチへのプッシュ](#common-boundaries)もデフォルトで許可されるため、設定する保護ブランチのデフォルトはありません。</Info>

デフォルトと並行して独自のエントリを追加するには、配列にリテラル文字列 `"$defaults"` を含めます。デフォルトエントリはその位置に挿入されるため、カスタムエントリはその前後に配置できます。

次の例は、デフォルトエントリを保持し、組織のリポジトリ、バケット、ドメイン、サービスを追加します。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

設定を保存した後、`claude auto-mode config` を実行して、[有効なルールがエントリを含む](#inspect-the-defaults-and-your-effective-config)ことを確認します。

エントリは散文であり、正規表現やツールパターンではありません。分類器はそれらを自然言語ルールとして読み取ります。新しいエンジニアにインフラストラクチャを説明する方法で記述してください。十分な環境セクションは以下をカバーします。

* **Organization（組織）**: 会社名と Claude Code が主に使用される用途（ソフトウェア開発、インフラストラクチャオートメーション、データエンジニアリングなど）
* **ソース管理**: 開発者がプッシュするすべての GitHub、GitLab、または Bitbucket 組織
* **クラウドプロバイダーと信頼できるバケット**: Claude が読み取りおよび書き込みできるバケット名またはプレフィックス
* **信頼できる内部ドメイン**: `*.internal.example.com` のようなネットワーク内の API、ダッシュボード、サービスのホスト名
* **主要な内部サービス**: CI、アーティファクトレジストリ、内部パッケージインデックス、インシデント対応ツール
* **内部パッケージレジストリ**: インストールがルーティングされるプライベート npm、PyPI、またはその他のレジストリ。パブリックレジストリをバイパスするインストールがブロックされるようにします
* **機密データの場所とオーディエンス**: 個人データ、機密ビジネスデータ、認証情報、規制データ、または同様に機密性の高い資料を保持するバケット、データベース、またはパス、および各場所のデータが共有される可能性があるオーディエンス。分類器がコンテンツから推測する代わりにこれらの場所を保護します。Claude Code v2.1.195 から v2.1.197 はこのエントリを「PII / 規制データの場所」と名前付けし、オーディエンスディメンションなしで個人データまたは規制データを保持する場所のみをカバーします
* **機密リモートターゲット**: 本番環境としてカウントされる名前空間、ホスト、またはコンテナ。リモートシェルおよびポートフォワードにはユーザーの明示的な承認が必要です
* **保護された IaC スコープ**: 適用または破棄が常にユーザーが変更を名前付けする必要があるインフラストラクチャリソース
* **追加コンテキスト**: 規制業界の制約、マルチテナントインフラストラクチャ、または分類器がリスクとして扱うべきことに影響するコンプライアンス要件

内部パッケージレジストリ、機密データの場所とオーディエンス、機密リモートターゲット、および保護された IaC スコープエントリには Claude Code v2.1.195 以降が必要です。以前のバージョンはそれらをプレーンコンテキストとして読み取りますが、それらをターゲットにする組み込みルールはありません。

有用な開始テンプレート: 括弧で囲まれたフィールドを入力し、適用されない行を削除します。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

より具体的なコンテキストを提供するほど、分類器は日常的な内部操作とデータ流出の試みをより適切に区別できます。

すべてを一度に入力する必要はありません。合理的なロールアウト: デフォルトから始めて、ソース管理組織と主要な内部サービスを追加します。これにより、独自のリポジトリへのプッシュなど、最も一般的な誤検知が解決されます。次に信頼できるドメインとクラウドバケットを追加します。ブロックが発生したら残りを入力します。

<h2 id="generate-environment-entries">
  `/auto-mode-setup` で環境エントリを生成する
</h2>

`/auto-mode-setup` を実行すると、Claude Code がプロジェクトと最近のセッションから `autoMode.environment` エントリを作成し、時には[ルールエントリ](#override-the-block-and-allow-rules)も作成します。ドラフトを承認すると、Claude Code は `~/.claude/settings.json` に書き込みます。

<Note>
  `/auto-mode-setup` には Pro、Max、または Team プランと Claude Code v2.1.228 以降が必要です。ネイティブ Windows では v2.1.233 以降が必要です。[Web 上の Claude Code](/docs/ja/claude-code-on-the-web) では実行できません。また[フィーチャーフラグ取得](/docs/ja/env-vars#features-that-need-feature-flag-fetching)が必要なため、フラグ取得をオフにしたセッションでは実行できません。
</Note>

<h3 id="what-auto-mode-setup-reads">
  `/auto-mode-setup` が読み込むもの
</h3>

`~/.claude/settings.json` に既に `autoMode` エントリが含まれている場合、Claude Code はまず環境リストに追加するか置き換えるかを尋ね、どちらの場合でも作成したルールを保持します。その後、Claude Code はこのプロジェクトの使用方法を尋ね、スキャンの前に 2 つのオプションスキャンを提供します。スキャンでは、Claude Code は常にこれらのソースを読み込みます：

* このプロジェクトの `CLAUDE.md`、`README.md`、設定ファイル、および git リモート
* `autoMode` および `permissions.allow` 設定
* Claude がこのプロジェクトの最近のセッションで実行したコマンドのホスト、バケット、コマンド名（メッセージは含まない）

2 つのオプションスキャンは各々1 つのソースを追加します：

* シェル履歴内の各コマンドの最初の単語
* ホームディレクトリ下のリポジトリのリモートホストと名前

<h3 id="review-and-save-the-draft">
  ドラフトを確認して保存する
</h3>

Claude Code はバックグラウンドでスキャンを実行し、ドラフトを表示します。ドラフト全体を承認または破棄し、その後 `~/.claude/settings.json` を編集して個別のエントリを調整します。承認すると、Claude Code はドラフトを書き込み、既に持っている設定と調整します：

* Claude Code は `"$defaults"` なしで `environment` リストを書き込みます。ドラフトが変更されていない組み込みエントリを明示しているため
* Claude Code は、ドラフトがエントリを追加する `allow`、`soft_deny`、および `hard_deny` リストのそれぞれに `"$defaults"` を含めます。ただし、既に `allow` リストを `"$defaults"` なしで作成している場合は除きます。そのため、置き換えていない[組み込みルール](#override-the-block-and-allow-rules)は有効なままです
* 保存後、Claude Code は `~/.claude/settings.json` 内の `permissions.allow` ルールを削除することを提案します。オートモードが無視するもの（`Bash(*)` など）、または破壊的なコマンドを自動承認するもの

その後、`claude auto-mode config` を実行して[有効な結果を確認](#inspect-the-defaults-and-your-effective-config)します。

<h3 id="turn-off-auto-mode-setup">
  `/auto-mode-setup` をオフにする
</h3>

オートモードが複数のアクションをブロックし、まだ `autoMode.environment` エントリがない場合、Claude Code はターンの終わりに「オートモードに環境について教えますか？」というタイトルのダイアログを表示し、`/auto-mode-setup` を実行することを提案します。提案を停止してコマンドを保持するには、そのダイアログで **\[今後表示しない]** を選択します。

コマンドと提案の両方をオフにするには、この[`skillOverrides`](/docs/ja/skills#override-skill-visibility-from-settings) エントリを `~/.claude/settings.json` に追加します：

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup` は[バンドルされたスキル](/docs/ja/skills#bundled-skills)ではなく組み込みコマンドなため、この `skillOverrides` エントリは引き続き適用されますが、[`disableBundledSkills`](/docs/ja/settings-reference#disablebundledskills) はオフにしません。

<h2 id="override-the-block-and-allow-rules">
  ブロックルールと許可ルールをオーバーライドする
</h2>

3 つの追加フィールドを使用すると、分類器の組み込みルールリストを置き換えることができます。

* `autoMode.hard_deny`：無条件のセキュリティ境界
* `autoMode.soft_deny`：ユーザーの意図でクリアできる破壊的なアクション
* `autoMode.allow`：ソフトブロックルールの例外

各フィールドは散文説明の配列であり、自然言語ルールとして読み込まれます。分類器の前に実行されるツールパターンベースのハードブロックについては、[`permissions.deny`](/docs/ja/permissions) を使用します。

分類器内では、優先順位は 4 つのレベルで機能します。

* `hard_deny` ルールは無条件にブロックします。ユーザーの意図と `allow` 例外は適用されません。
* `soft_deny` ルールが次にブロックします。ユーザーの意図と `allow` 例外はこれらをオーバーライドできます。
* `allow` ルールは一致する `soft_deny` ルールを例外としてオーバーライドします。
* 明示的なユーザーの意図が残りのソフトブロックをオーバーライドします。ユーザーのメッセージが Claude が実行しようとしている正確なアクションを直接かつ具体的に説明する場合、`soft_deny` ルールが一致しても分類器はそれを許可します。

一般的なリクエストは明示的な意図としてカウントされません。Claude に「リポジトリをクリーンアップする」ように依頼することは force push を認可しませんが、「このブランチを force push する」ように依頼することは認可します。

緩和するには、分類器がデフォルトの例外がカバーしていないルーチンパターンを繰り返しフラグする場合、`allow` に追加します。厳しくするには、環境に固有で、デフォルトが見落としているリスクについて `soft_deny` に追加するか、絶対に越えてはいけないセキュリティ境界について `hard_deny` に追加します。

組み込みルールを保持しながら独自のルールを追加するには、配列にリテラル文字列 `"$defaults"` を含めます。デフォルトルールはその位置に挿入されるため、カスタムルールはそれらの前後に配置でき、リリース全体でビルトインリストが変更されるにつれて更新を継続して継承します。

次の例は、すべての 4 つのリストでデフォルトを保持し、各リストに組織固有のルールを追加しています。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  `environment`、`allow`、`soft_deny`、または `hard_deny` のいずれかを `"$defaults"` なしで設定すると、そのセクション全体のデフォルトリストが置き換わります。`"$defaults"` なしで配列を設定する場合、そのセクションの組み込みルールを破棄します。

  * `soft_deny`：force push、`curl | bash`、本番環境へのデプロイ、およびオートモードバイパスを含むすべての組み込みソフトブロックルール
  * `hard_deny`：組み込みのデータ流出ルール
</Danger>

各セクションは独立して評価されるため、`environment` のみを設定すると、デフォルトの `allow`、`soft_deny`、および `hard_deny` リストはそのままになります。

`"$defaults"` は、リストの完全な所有権を取得する意図がある場合のみ省略します。その場合、`claude auto-mode defaults` を実行して組み込みルールを出力し、それらを設定ファイルにコピーしてから、各ルールを独自のパイプラインとリスク許容度に対して確認します。

<h2 id="edit-rules-from-permissions">
  `/permissions` から編集ルール
</h2>

設定ファイルを開かずに分類器ルールを表示および編集するには、[`/permissions`](/docs/ja/permissions#manage-permissions) を実行して **Auto mode** タブを選択します。このタブは Claude Code v2.1.246 以降が必要であり、[auto mode がセッションで利用可能](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)な場合にのみ表示されます。

このタブには、[分類器が設定を読み込むスコープ](#where-the-classifier-reads-configuration)のそれぞれから `allow`、`soft_deny`、`hard_deny`、および `environment` エントリが一覧表示され、各セクションに対して組み込みルールが有効かどうかが表示されます。Claude Code は [管理設定](/docs/ja/server-managed-settings)または `--settings` フラグからのエントリを読み取り専用として表示し、タブで行ったすべての変更を `~/.claude/settings.json` に保存します。タブから以下の操作ができます。

* `allow`、`soft_deny`、および `hard_deny` セクションのルールを追加、編集、または削除します。セクションに最初のルールを追加すると、Claude Code は `"$defaults"` も挿入して、[組み込みルール](#override-the-block-and-allow-rules)が有効なままになるようにします。
* `allow`、`soft_deny`、または `hard_deny` の組み込みルールをオフにするか、再度オンにします。Claude Code は、そのセクションのリストに `"$defaults"` を追加または削除することで選択を記録するため、セクションの組み込みルールをオフにする前に、少なくとも 1 つの独自ルールが必要です。
* `environment` エントリをエディタで 1 つのドキュメントとして編集します。まだ `environment` エントリを設定していない場合、Claude Code は最初に組み込み環境を置き換えるかどうかを尋ね、その後、組み込みテキスト全体でエディタを開きます。保存すると、Claude Code はドキュメントで `autoMode.environment` 配列を置き換えます。`"$defaults"` 行を含めて、[組み込みエントリを保持](#define-trusted-infrastructure)します。

<h2 id="route-all-shell-commands-through-the-classifier">
  すべてのシェルコマンドを分類器を通してルーティングする
</h2>

デフォルトでは、狭い Bash および PowerShell の許可ルール（`Bash(npm test)` など）はオートモードで有効なままであり、Claude Code は分類器が実行される前にそれらを解決します。ただし、コマンドが [コマンドごとの許可ドメイン](/docs/ja/sandboxing#per-command-allowed-domains-in-auto-mode) を含む場合は除きます。Claude Code は、`Bash(*)` やワイルドカード化されたインタープリターなどの任意のコード実行を許可する広いルール、および [`Monitor`](/docs/ja/tools-reference#monitor-tool) という名前のすべてのルールを一時停止します。Monitor コマンドはシェルを通して実行されるためです。これは、狭いルールが、分類器が見ることなく、破壊的な引数（スクリプトパスやルールのプレフィックスが予想しなかったフラグなど）を通す可能性があることを意味します。

`autoMode.classifyAllShell` を `true` に設定して、オートモードがアクティブな間、すべての Bash および PowerShell の許可ルールを一時停止し、分類器が許可リストに関係なくすべてのシェルコマンドを評価するようにします。

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

これは、レイテンシーをカバレッジと引き換えにします。許可ルールが即座に承認したコマンドは、分類器の決定を待つようになり、各シェルコマンドは分類器呼び出しとしてカウントされます。

この設定はオートモードがアクティブな間のみ適用され、他の権限モードではあなたの許可ルールは通常通り動作します。

<Note>
  `autoMode.classifyAllShell` には Claude Code v2.1.193 以降が必要です。それより前のバージョンはこのキーを無視し、狭いシェル許可ルールをオートモードに引き継ぎます。
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  デフォルトと有効な設定を確認する
</h2>

`claude auto-mode` サブコマンドは、設定を確認、検証、リセットするのに役立ちます。

組み込みの `environment`、`allow`、`soft_deny`、`hard_deny` ルールを JSON として出力します：

```bash theme={null}
claude auto-mode defaults
```

`jq` にパイプしなくても 1 つのルールの完全な文言を読むには、`--label` にルールのラベルの開始部分を渡します。例えば `claude auto-mode defaults --label 'Git Destructive'` のようにします。マッチングは各ルールのラベルに対する大文字小文字を区別しないプレフィックスであり、マッチしないセクションは空のリストとして出力されます。Claude Code v2.1.208 以降が必要です。

分類器が実際に使用する設定を JSON として出力します。設定が指定されている場合はその設定を、そうでない場合はデフォルトを適用します：

```bash theme={null}
claude auto-mode config
```

`defaults` と `config` の両方は、4 つのルールリストを単一の JSON オブジェクトとして出力し、各ルールは散文文字列です。これは切り詰められた例です：

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, or hardcoding test cases. Placeholder means authored as a placeholder — a file or value copied from a real secret or sensitive path is never a test artifact (see Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), deleting remote branches, tags, or releases, or rewriting remote history. Also `git commit --amend` when the commit being rewritten is not the agent's own unpushed work: either no prior `git commit` is visible (HEAD pre-dates the session), or a `git push` of the current branch is visible after the most recent commit (it has been pushed). Clears when the user asked to amend/reword/fixup, or when it is a message-only reword (`--amend -m …`, nothing newly staged) of a commit the agent visibly created this session.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: The git repository the agent started in (its working directory) and its configured remote(s). When the repo's public/private visibility is given — by the Repository visibility entry or the user's own message — use it to scope what is OK to commit or push there: confidential material is fine in a private repo; in a public one, only that repo's own work is — and content ported, repointed, or first read from outside this session's repo is not its own work, whoever directed the port. Visibility scopes confidential material only: secrets and sensitive data (personal & entrusted) are never cleared into any repo by its visibility (see Definitions).",
    ...
  ]
}
```

カスタム `allow`、`soft_deny`、`hard_deny` ルールについて AI からのフィードバックを取得します：

```bash theme={null}
claude auto-mode critique
```

設定を保存した後に `claude auto-mode config` を実行して、有効なルールが期待通りであることを確認します。`"$defaults"` は展開されて表示されます。カスタムルールを作成した場合、`claude auto-mode critique` はそれらをレビューし、曖昧、冗長、または偽陽性を引き起こす可能性のあるエントリにフラグを立てます。

カスタマイズを破棄して組み込みデフォルトに戻すには、reset サブコマンドを実行します。Claude Code v2.1.212 以降が必要であり、ユーザー設定ファイルから `autoMode` セクションを削除します：

```bash theme={null}
claude auto-mode reset
```

このコマンドは削除する内容を要約し、書き込む前に `Reset auto mode configuration to defaults?` と確認を求めます。`--yes` を渡すと確認をスキップできます。Reset は `~/.claude/settings.json` のみを変更します。[managed settings](/docs/ja/server-managed-settings) または `--settings` フラグからの `autoMode` ルールは引き続き適用されます。

<h2 id="review-denials">
  拒否の確認
</h2>

オートモード分類器が拒否したアクションを確認して再試行するには、`/permissions` を開いて **Recently denied** タブを選択します。ここで Claude Code は各拒否を記録しています。拒否されたアクションで `r` を押してリトライ対象としてマークします。ダイアログを終了すると、Claude Code はモデルにそのツール呼び出しを再試行できることを伝えるメッセージを送信し、会話を再開します。

分類器が[アクションの安全性について判定できない](/docs/ja/errors#auto-mode-cannot-determine-the-safety-of-an-action)場合、オートモードとは別の安全チェックが分類器自身のリクエストを拒否したか、またはそのレスポンスが解析できなかったため、Claude Code はアクションを拒否しますが、**Recently denied** に記録しません。リンク先のエラーエントリでは、Claude に伝えられる内容と、必要な場合にアクションを実行する方法について説明しています。

<h3 id="fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry">
  許可ルール、環境エントリ、またはリトライで拒否を修正する
</h3>

分類器がブロックした内容を確認するには、会話でツール呼び出しを見つけます。呼び出しが短縮されているか、`Ran 3 shell commands` のような概要行に折りたたまれている場合は、`Ctrl+O` を押して[トランスクリプトビューア](/docs/ja/interactive-mode#transcript-viewer)を開き、展開します。

画面上の拒否を報告する他の 2 つの場所では、コマンドまたは URL が省略されています。入力ボックスの近くの通知（`bash denied by auto mode · [Data Exfiltration] · /permissions` など）はツールと理由を示し、**Recently denied** タブはシェルコマンドを Claude が記述した説明で一覧表示します。これらの拒否の正確な入力をプログラムで取得するには、[`PermissionDenied` フック](/docs/ja/hooks#permissiondenied)を追加します。これは `tool_input` として受け取ります。

呼び出しの下のテキストは、修正すべきことがあるかどうかを示します。分類器自体の問題を報告するテキスト（`is temporarily unavailable` のようなモデルや分類器エラーなど）は、Claude Code が分類器からの最終判定なしに呼び出しをブロックしたことを意味します。詳細は[オートモードがアクションの安全性を判定できない](/docs/ja/errors#auto-mode-cannot-determine-the-safety-of-an-action)を参照してください。それ以外の場合、`Denied by auto mode classifier` と `[Production Deploy]` または `Blocked by classifier` などの理由が記載された行は、分類器が呼び出しを安全でないと判定したことを意味するため、呼び出しが何に到達しようとしていたか、または何をしようとしていたかから修正を選択します。

* タスク全体を通じて Claude が必要とする宛先（パッケージレジストリ、内部ドメイン、リポジトリホストなど）：`autoMode.environment` に追加します。
* これからレビューなしで実行したいコマンド：`allow` ルールを追加します。
* 実際に意図した 1 回限りのアクション：次のメッセージでその意図を述べて、Claude に再試行させます。

環境エントリまたは `allow` ルールは、`/permissions` ダイアログの[**Auto mode** タブ](#edit-rules-from-permissions)から追加できます。

ほとんどのセッションでは、理由は分類器が一致したルールを角括弧で示します。例えば `[Data Exfiltration]` または `[Production Deploy]` のようにです。一部のセッションでは、短い説明を追加する分類器モデルを実行します。Claude Code が分類器モデルを選択するため、表示される形式はユーザーが設定できるものではありません。

<h3 id="fix-repeated-denials">
  繰り返される拒否を修正する
</h3>

同じ宛先に対する繰り返される拒否は、通常、分類器がコンテキストを欠落していることを意味します。その宛先を `autoMode.environment` に追加するか、[`/auto-mode-setup` を実行](#generate-environment-entries)して Claude Code にエントリを作成させ、その後 `claude auto-mode config` を実行して変更が有効になったことを確認します。

拒否にプログラムで対応するには、[`PermissionDenied` フック](/docs/ja/hooks#permissiondenied)を使用します。

<h2 id="see-also">
  関連項目
</h2>

* [権限モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)：auto モードとは何か、デフォルトでブロックされるもの、およびどのセッションがそれで開始されるか
* [マネージド設定](/docs/ja/server-managed-settings)：組織全体に `autoMode` 設定をデプロイする
* [権限](/docs/ja/permissions)：分類器が実行される前に適用される許可、確認、および拒否ルール
* [すべての設定](/docs/ja/settings-reference#automode)：`autoMode` を含むすべての設定キー
