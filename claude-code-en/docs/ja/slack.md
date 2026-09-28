> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Slack での Claude Code

> Slack ワークスペースから直接コーディングタスクを委任する。Anthropic は Team および Enterprise ワークスペース向けにこの以前のバージョンを Claude Tag に置き換えています。Pro および Max プランではセットアップパスのままです。

<Warning>
  このページは、各セッションが個別ユーザーのアカウントで実行される以前の Claude Code in Slack について説明しています。

  * **Team および Enterprise プラン：** Anthropic は、組織の共有 ID として @Claude を実行し、管理者が設定したアクセス権限を持つ [Claude Tag](https://claude.com/product/tag) に置き換えています。既存の Slack アプリと @Claude ハンドルは変わらず、Anthropic アカウントチームが切り替え日を通知できます。新しいワークスペースの場合は [Claude Tag をセットアップ](https://claude.com/docs/claude-tag/overview)してください。このバージョンを既に使用しているワークスペースを移行する場合は、[以前の Claude in Slack から移行する](https://claude.com/docs/claude-tag/admins/migrate-from-earlier)を参照してください。
  * **Pro および Max プラン：** Claude Tag は個別プランでは利用できないため、このページがセットアップパスのままです。
</Warning>

Slack での Claude Code は、Claude Code の機能を Slack ワークスペースに直接もたらします。`@Claude` にコーディングタスクをメンションすると、Claude は自動的に意図を検出し、Claude Code クラウドセッションを作成します。これにより、チームの会話を離れることなく開発作業を委任できます。

この統合は既存の Claude for Slack アプリに基づいていますが、コーディング関連のリクエストに対して Claude Code クラウドセッションへのインテリジェントなルーティングを追加しています。各セッションは自分の Claude アカウントで実行され、接続されたリポジトリと自分のプラン制限を使用します。

<h2 id="use-cases">
  ユースケース
</h2>

* **バグ調査と修正**: Slack チャネルで報告されたバグを Claude に調査・修正させます。
* **迅速なコードレビューと修正**: Claude にチームのフィードバックに基づいて小さな機能を実装したりコードをリファクタリングさせます。
* **協調的なデバッグ**: チームの議論が重要なコンテキスト（エラーの再現やユーザーレポートなど）を提供する場合、Claude はその情報を使用してデバッグアプローチを知らせることができます。
* **並列タスク実行**: Slack でコーディングタスクを開始しながら他の作業を続け、完了時に通知を受け取ります。

<h2 id="prerequisites">
  前提条件
</h2>

Claude Code in Slack を使用する前に、以下を確認してください：

| 要件           | 詳細                                                                                  |
| :----------- | :---------------------------------------------------------------------------------- |
| Claude プラン   | Pro、Max、Team、または Claude Code アクセス付き Enterprise（プレミアムシート、または Chat + Claude Code シート） |
| クラウドセッション    | [クラウドセッション](/docs/ja/claude-code-on-the-web) がアカウントで有効になっている                             |
| GitHub アカウント | [claude.ai/code](https://claude.ai/code) で接続され、少なくとも 1 つのリポジトリが認証されている              |
| Slack 認証     | Slack アカウントが Claude アプリを通じて Claude アカウントにリンクされている                                   |

<h2 id="setting-up-claude-code-in-slack">
  Slack での Claude Code のセットアップ
</h2>

<Steps>
  <Step title="Slack に Claude アプリをインストールする">
    ワークスペース管理者は Slack App Marketplace から Claude アプリをインストールする必要があります。[Slack App Marketplace](https://slack.com/marketplace/A08SF47R6P4) にアクセスして「Add to Slack」をクリックしてインストールプロセスを開始します。
  </Step>

  <Step title="Claude アカウントを接続する">
    アプリがインストールされた後、個別の Claude アカウントを認証します：

    1. Apps セクションで「Claude」をクリックして Slack で Claude アプリを開きます
    2. App Home タブを開きます
    3. 「Connect」をクリックして Slack アカウントを Claude アカウントにリンクします
    4. ブラウザで認証フローを完了します
  </Step>

  <Step title="クラウドセッションを設定する">
    クラウドセッションがアカウント用に適切に設定されていることを確認します：

    * [claude.ai/code](https://claude.ai/code) にアクセスして、Slack に接続したのと同じアカウントでサインインします
    * GitHub アカウントがまだ接続されていない場合は接続します
    * Claude が作業するリポジトリを少なくとも 1 つ認証します
  </Step>

  <Step title="ルーティングモードを選択する">
    アカウントを接続した後、Claude が Slack のメッセージをどのように処理するかを設定します。Slack の Claude App Home を開いて、**ルーティングモード**設定を見つけます。

    | モード             | 動作                                                                                                                                          |
    | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
    | **Code のみ**     | Claude はすべての @mentions を Claude Code セッションにルーティングします。Claude を Slack で開発タスク専用に使用するチームに最適です。                                                  |
    | **Code + Chat** | Claude は各メッセージを分析し、Claude Code（コーディングタスク用）と Claude Chat（執筆、分析、一般的な質問用）の間でインテリジェントにルーティングします。すべてのタイプの作業に対して単一の @Claude エントリポイントが必要なチームに最適です。 |

    <Note>
      Code + Chat モードでは、Claude がメッセージを Chat にルーティングしたがコーディングセッションが必要な場合は、「Retry as Code」をクリックして Claude Code セッションを作成できます。同様に、Code にルーティングされたが Chat セッションが必要な場合は、そのスレッドでそのオプションを選択できます。
    </Note>
  </Step>

  <Step title="Claude をチャネルに追加する">
    Claude はインストール後、自動的にどのチャネルにも追加されません。チャネルで Claude を使用するには、そのチャネルで `/invite @Claude` と入力して招待します。Claude は追加されたチャネルの @mentions にのみ応答できます。
  </Step>
</Steps>

<h2 id="how-it-works">
  仕組み
</h2>

<h3 id="automatic-detection">
  自動検出
</h3>

Code + Chat ルーティングモードでは、Slack チャネルまたはスレッドで @Claude をメンションすると、Claude は自動的にメッセージがコーディングタスクかどうかを検出します。コーディングタスクは Claude Code クラウドセッションに送信されます。その他のものは通常のチャット返信を受け取ります。Code のみモードでは、すべての @mention が Claude Code に送信されます。

また、Claude が自動的に検出しない場合でも、リクエストをコーディングタスクとして処理するよう Claude に明示的に指示することもできます。

<Note>
  Slack での Claude Code はチャネル（公開または非公開）でのみ機能します。ダイレクトメッセージ（DM）では機能しません。
</Note>

<h3 id="context-gathering">
  コンテキスト収集
</h3>

**スレッドから**: スレッドで @Claude をメンションすると、そのスレッド内のすべてのメッセージからコンテキストを収集して、完全な会話を理解します。

**チャネルから**: チャネルで直接メンションされた場合、Claude は関連するコンテキストについて最近のチャネルメッセージを確認します。

このコンテキストは Claude が問題を理解し、適切なリポジトリを選択し、タスクへのアプローチを知らせるのに役立ちます。

<Warning>
  @Claude が Slack で呼び出されると、Claude はリクエストをより良く理解するために会話コンテキストへのアクセスが与えられます。Claude は他のメッセージからの指示に従う可能性があるため、ユーザーは信頼できる Slack 会話でのみ Claude を使用するようにしてください。
</Warning>

<h3 id="session-flow">
  セッションフロー
</h3>

1. **開始**: @Claude にコーディングリクエストをメンションします
2. **検出**: Claude がメッセージを分析してコーディング意図を検出します
3. **セッション作成**: claude.ai/code で新しい Claude Code セッションが作成されます
4. **進捗更新**: Claude は作業が進むにつれて Slack スレッドにステータス更新を投稿します
5. **完了**: 完了時に、Claude は概要とアクションボタンを含めてあなたをメンションします
6. **レビュー**: 「View Session」をクリックして完全なトランスクリプトを表示するか、「Create PR」をクリックしてプルリクエストを開きます

<h2 id="user-interface-elements">
  ユーザーインターフェース要素
</h2>

<h3 id="message-actions">
  メッセージアクション
</h3>

* **View Session**: ブラウザで完全な Claude Code セッションを開き、実行されたすべての作業、セッションの継続、または追加のリクエストを確認できます。
* **Create PR**: セッションの変更から直接プルリクエストを作成します。
* **Retry as Code**: Claude が最初はチャットアシスタントとして応答したがコーディングセッションが必要な場合は、このボタンをクリックしてリクエストを Claude Code タスクとして再試行します。
* **Change Repo**: Claude が誤って選択した場合、別のリポジトリを選択できます。

<h3 id="repository-selection">
  リポジトリ選択
</h3>

Claude は Slack 会話のコンテキストに基づいてリポジトリを自動的に選択します。複数のリポジトリが適用される可能性がある場合、Claude は正しいものを選択できるドロップダウンを表示する場合があります。

<h2 id="access-and-permissions">
  アクセスと権限
</h2>

<h3 id="user-level-access">
  ユーザーレベルのアクセス
</h3>

| アクセスタイプ           | 要件                                            |
| :---------------- | :-------------------------------------------- |
| Claude Code セッション | 各ユーザーは自分の Claude アカウントでセッションを実行します            |
| 使用状況とレート制限        | セッションは個別ユーザーのプラン制限に対してカウントされます                |
| リポジトリアクセス         | ユーザーは個人的に接続したリポジトリにのみアクセスできます                 |
| セッション履歴           | セッションは claude.ai/code の Claude Code 履歴に表示されます |

<h3 id="workspace-level-access">
  ワークスペースレベルのアクセス
</h3>

Slack ワークスペース管理者は、Claude アプリをワークスペースで利用可能にするかどうかを制御します：

| コントロール              | 説明                                                                 |
| :------------------ | :----------------------------------------------------------------- |
| アプリのインストール          | ワークスペース管理者は Slack App Marketplace から Claude アプリをインストールするかどうかを決定します |
| Enterprise Grid の配布 | Enterprise Grid 組織の場合、組織管理者は Claude アプリへのアクセスを持つワークスペースを制御できます     |
| アプリの削除              | ワークスペースからアプリを削除すると、そのワークスペース内のすべてのユーザーのアクセスが直ちに取り消されます             |

<h3 id="channel-based-access-control">
  チャネルベースのアクセス制御
</h3>

アプリをインストールしても、Claude は自動的にどのチャネルにも追加されません。Claude は @mentions に応答するのは、追加されたチャネルのみです。`/invite @Claude` で招待してください。公開チャネルと非公開チャネルの両方で機能します。管理者は Claude が招待されるチャネルと、それらのチャネルへのアクセス権を持つユーザーを管理することで、Claude Code の使用を制御できます。これにより、ワークスペースレベルの権限を超えた追加のアクセス制御層が提供されます。

<h2 id="what’s-accessible-where">
  どこでアクセスできるか
</h2>

**Slack で**: ステータス更新、完了概要、アクションボタンが表示されます。完全なトランスクリプトは保存され、常にアクセス可能です。

**claude.ai/code で**: 完全な Claude Code セッション、完全な会話履歴、すべてのコード変更、ファイル操作があります。セッションは [claude.ai/code](https://claude.ai/code) の Claude Code 履歴に保存され、過去のセッションを続行したり、参照したり、プルリクエストを作成したりできます。

Enterprise および Team アカウントの場合、Slack の Claude から作成されたセッションは自動的に組織に表示されます。詳細については、[クラウドセッション共有](/docs/ja/claude-code-on-the-web#share-sessions) を参照してください。

<h2 id="best-practices">
  ベストプラクティス
</h2>

<h3 id="writing-effective-requests">
  効果的なリクエストの作成
</h3>

* **具体的に**: ファイル名、関数名、またはエラーメッセージが関連する場合は含めます。
* **コンテキストを提供**: 会話から明確でない場合はリポジトリまたはプロジェクトをメンションします。
* **成功を定義**: 「完了」とはどういう意味か説明します。Claude はテストを書くべきですか？ドキュメントを更新しますか？PR を作成しますか？
* **スレッドを使用**: バグや機能について議論する場合はスレッドで返信して、Claude が完全なコンテキストを収集できるようにします。

<h3 id="when-to-use-slack-vs-web">
  Slack とウェブの使い分け
</h3>

**Slack を使用する場合**: コンテキストが既に Slack の議論に存在する場合、タスクを非同期で開始したい場合、またはチームメイトが可視性を必要とする場合に協力しています。

**ウェブを直接使用する場合**: ファイルをアップロードする必要がある場合、開発中のリアルタイムインタラクションが必要な場合、またはより長く複雑なタスクに取り組んでいる場合。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  「Claude Code がアカウントで有効になっていません」
</h3>

このエラーは、Claude アカウントにまだクラウド環境がないことを意味します。Slack に接続したのと同じアカウントで [claude.ai/code](https://claude.ai/code) に 1 回サインインして、[ウェブオンボーディング](/docs/ja/web-quickstart#connect-github)を完了してください。これにより、デフォルトのクラウド環境が作成されるか、作成するよう求められます。エラーは次回のメンション時に解消されます。各ユーザーが個別に実行する必要があります。

<h3 id="sessions-not-starting">
  セッションが開始しない
</h3>

1. Claude アカウントが Claude App Home で接続されていることを確認します
2. クラウドセッションがアカウントで有効になっていることを確認します
3. Claude Code に接続された GitHub リポジトリが少なくとも 1 つあることを確認します

<h3 id="sessions-from-a-claude-tag-channel-fail-to-start">
  Claude Tag チャネルからのセッションが開始に失敗する
</h3>

このエントリは [Claude Tag](https://claude.com/docs/claude-tag/overview) を使用しているワークスペースに適用されます。Claude Tag では、Claude はメンバーのアカウントではなく、組織の共有 ID としてチャネルで機能します。[claude.ai/code](https://claude.ai/code) でチャネルのクラウド環境を作成した場合、それはあなたの個人アカウントに属しており、Claude は個人環境でチャネルセッションを開始できません。Claude Code はセッションを直ちに失敗させ、再試行しても役に立ちません。

Owner の場合で環境があなたのものである場合は、環境セレクターから [組織と共有](/docs/ja/cloud-environments#organization-shared-environments)してください。それ以外の場合は、Owner が [admin settings](https://claude.ai/admin-settings) の **Cloud environments** ページから組織共有環境として再作成します。

次の 2 つの方法で適用できます。

* [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) で組織のデフォルトとして設定します。
* Claude Tag admin settings で [チャネルに設定](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one)します。

Owner でない場合は、このエントリを Owner に送信してください。

<h3 id="repository-not-showing">
  リポジトリが表示されない
</h3>

1. [claude.ai/code](https://claude.ai/code) でリポジトリを接続します
2. そのリポジトリの GitHub 権限を確認します
3. GitHub アカウントを切断して再接続してみます

<h3 id="wrong-repository-selected">
  誤ったリポジトリが選択された
</h3>

1. 「Change Repo」ボタンをクリックして別のリポジトリを選択します
2. より正確な選択のためにリクエストにリポジトリ名を含めます

<h3 id="authentication-errors">
  認証エラー
</h3>

1. App Home で Claude アカウントを切断して再接続します
2. ブラウザで正しい Claude アカウントにサインインしていることを確認します
3. Claude プランに Claude Code アクセスが含まれていることを確認します

<h2 id="current-limitations">
  現在の制限事項
</h2>

* **GitHub のみ**: リポジトリは GitHub 上にある必要があります。
* **一度に 1 つの PR**: 各セッションは 1 つのプルリクエストを作成できます。
* **クラウドセッションアクセスが必要**: ユーザーは [クラウドセッション](/docs/ja/claude-code-on-the-web) へのアクセスが必要です。アクセスがない場合、Claude は標準的なチャット応答で返信します。

<h2 id="related-resources">
  関連リソース
</h2>

<CardGroup>
  <Card title="クラウド上の Claude Code" icon="cloud" href="/docs/ja/claude-code-on-the-web">
    クラウドセッションについて詳しく学ぶ
  </Card>

  <Card title="Claude for Slack" icon="slack" href="https://claude.com/claude-and-slack">
    Claude for Slack の一般的なドキュメント
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    Slack での組織管理の @Claude（管理者が設定したアクセス権限付き）
  </Card>

  <Card title="Slack App Marketplace" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    Slack Marketplace から Claude アプリをインストール
  </Card>

  <Card title="Claude ヘルプセンター" icon="circle-question" href="https://support.claude.com">
    追加サポートを取得
  </Card>
</CardGroup>
