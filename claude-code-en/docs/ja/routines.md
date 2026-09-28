> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# ルーティンで作業を自動化する

> Claude Code を自動操縦に設定します。スケジュールで実行するルーティンを定義したり、API 呼び出しでトリガーしたり、Anthropic が管理するクラウドインフラストラクチャから GitHub イベントに反応させたりできます。

<Note>
  ルーティンはリサーチプレビュー段階です。動作、制限、API サーフェスは変更される可能性があります。
</Note>

ルーティンは保存された Claude Code 構成です。プロンプト、1 つ以上のリポジトリ、および一連の [コネクタ](/docs/ja/mcp) をパッケージ化して、1 回定義し、自動的に実行します。ルーティンは Anthropic が管理するクラウドインフラストラクチャで実行されるか、ルーティングされた場合は組織の [自己ホスト環境](/docs/ja/self-hosted-environments) で実行されるため、ラップトップを閉じても動作し続けます。

各ルーティンには、1 つ以上のトリガーを接続できます。

* **スケジュール**: 時間ごと、毎晩、毎週など、定期的なペースで実行、または特定の将来の時刻に 1 回実行
* **API**: ベアラートークン付きで HTTP POST をルーティン固有のエンドポイントに送信してオンデマンドでトリガー
* **GitHub**: プルリクエストやリリースなどのリポジトリイベントに自動的に反応して実行

1 つのルーティンは複数のトリガーを組み合わせることができます。たとえば、PR レビュールーティンは毎晩実行でき、デプロイスクリプトからトリガーでき、新しい PR すべてに反応することもできます。

ルーティンは Pro、Max、Team、Enterprise プランで利用可能です。[claude.ai/code/routines](https://claude.ai/code/routines) で作成・管理するか、CLI で `/schedule` を使用して管理できます。

Team および Enterprise オーナーは、[claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) の Routines トグルを使用して、すべてのメンバーのルーティンを無効にできます。無効にすると、既存のルーティンは実行を停止し、メンバーは新しいルーティンを作成できません。

このページでは、ルーティンの作成、各トリガータイプの構成、実行の管理、および使用制限の適用方法について説明します。

<h2 id="example-use-cases">
  ユースケースの例
</h2>

各例は、トリガータイプと、ルーティンが適している作業の種類をペアにしています。無人で実行でき、繰り返し可能で、明確な成果に結びついています。

**バックログメンテナンス。** スケジュールトリガーは毎週夜間にコネクタ経由で問題追跡ツールに対して実行されます。ルーティンは最後の実行以降にオープンされた問題を読み取り、ラベルを適用し、参照されているコード領域に基づいて所有者を割り当て、Slack に概要を投稿して、チームが 1 日を整理されたキューで開始できるようにします。

**アラートトリアージ。** 監視ツールがエラー閾値を超えたときにルーティンの API エンドポイントを呼び出し、アラート本文を `text` として渡します。ルーティンのプロンプトは Claude にアラートを調査するよう指示し、スタックトレースを取得し、リポジトリの最近のコミットと相関させ、提案された修正とアラートへのリンク付きのドラフトプルリクエストを開きます。オンコール担当者は空のターミナルから始めるのではなく PR をレビューします。

**カスタムコードレビュー。** GitHub トリガーは `pull_request.opened` で実行されます。ルーティンはチームの独自のレビューチェックリストを適用し、セキュリティ、パフォーマンス、スタイルの問題についてインラインコメントを残し、概要コメントを追加して、人間のレビュアーが機械的なチェックではなく設計に焦点を当てられるようにします。

**デプロイ検証。** CD パイプラインは各本番デプロイ後にルーティンの API エンドポイントを呼び出します。ルーティンは新しいビルドに対してスモークテストを実行し、エラーログをスキャンして回帰を検出し、デプロイウィンドウが閉じる前にリリースチャネルに go または no-go を投稿します。

**ドキュメントドリフト。** スケジュールトリガーは毎週実行されます。ルーティンは最後の実行以降にマージされた PR をスキャンし、変更された API を参照するドキュメントにフラグを立て、エディターがレビューするためにドキュメントリポジトリに対して更新 PR を開きます。

**ライブラリポート。** GitHub トリガーは `pull_request.closed` で実行され、1 つの SDK リポジトリのマージされた PR にフィルタリングされます。ルーティンは別の言語の並列 SDK に変更をポートし、マッチング PR を開き、人間が各変更を再実装することなく 2 つのライブラリを同期させます。

<h2 id="create-a-routine">
  ルーチンを作成する
</h2>

[claude.ai/code/routines](https://claude.ai/code/routines) のウェブから、Desktop アプリから、または CLI から、ルーチンを作成できます。3 つのサーフェスすべてが同じクラウドアカウントに書き込まれるため、1 つで作成したルーチンは他のサーフェスにすぐに表示されます。Desktop アプリの **Code** タブで、サイドバーまたはサイドバーの **More** メニューの **Routines** をクリックしてから **New routine** を選択し、**Cloud** を選択します。代わりに **Local** を選択すると、[Desktop scheduled task](/docs/ja/desktop-scheduled-tasks) が作成されます。これはクラウドではなくマシン上で実行されます。

作成フォームは、ルーチンのプロンプト、リポジトリ、環境、コネクタ、およびトリガーを設定します。

ルーチンは完全な Claude Code クラウドセッションとして自律的に実行されます。権限モードピッカーはなく、セッションはシェルコマンドを実行し、クローンされたリポジトリにコミットされた [skills](/docs/ja/skills) を使用し、含めたコネクタを呼び出します。すべて承認を求めずに実行されます。ただし、一部の [artifact](/docs/ja/artifacts) アクションは除きます。

ルーチンが到達できる範囲は、選択したリポジトリ、[環境](/docs/ja/cloud-environments) のネットワークアクセスと変数、および含めたコネクタによって決定されます。これらのそれぞれをルーチンが実際に必要とするものにスコープします。

ルーチンのスケジュールまたは **Run now** が実行を開始するとき、Claude は以下のすべてが当てはまる場合にのみ、既存の artifact を再公開します。承認を求めません。

* artifact を編集でき、自分の組織に属している
* artifact は公開共有されておらず、特定の人または組織と共有されていない。または、最新バージョンが閲覧者が見るバージョンとして選択されていない
* 公開は、ページのみを含み、サポートファイルやその他のものは追加されず、新しいバージョンを強制的に上書きしない
* ページは、[connector calls](/docs/ja/artifacts#pull-live-data-with-mcp-connectors) などのページを超えて到達する権限を保持していない

その他のすべての場合（新しい artifact を公開する場合を含む）、Claude は最初に確認します。ルーチンの仕事がページを最新に保つことである場合、既に公開した artifact を与えます。

ルーチンは個別の claude.ai アカウントに属します。チームメイトと共有されず、アカウントの日次実行許容量に対してカウントされます。ルーチンが接続された GitHub ID またはコネクタを通じて実行するすべてのことは、あなたとして表示されます。コミットとプルリクエストは GitHub ユーザーを含み、Slack メッセージ、Linear チケット、またはその他のコネクタアクションはこれらのサービスのリンクされたアカウントを使用します。

<h3 id="create-from-the-web">
  ウェブから作成する
</h3>

<Steps>
  <Step title="作成フォームを開く">
    [claude.ai/code/routines](https://claude.ai/code/routines) にアクセスして、**New routine** をクリックします。
  </Step>

  <Step title="ルーチンに名前を付けてプロンプトを書く">
    ルーチンに説明的な名前を付け、Claude が毎回実行するプロンプトを書きます。プロンプトが最も重要な部分です。ルーチンは自律的に実行されるため、プロンプトは自己完結型で、何をするか、成功がどのように見えるかについて明確である必要があります。

    トリガーが発火すると、セッションはルーチンの保存されたプロンプトを割り当てられたタスクとして受け取り、それを実行します。会話の途中に到着した信頼できないコンテンツとして扱うのではなく。トリガーは、プロンプトがアカウント上の認可されたセッションによって事前に保存されたことのみを証明するため、発火したプロンプトはライブユーザー入力ではなく、実行中のアクションの承認または同意として機能することはできません。セッションが実行中に取得するコンテンツは、通常の処理を保持します。v2.1.213 より前では、セッションは同じプロンプトを信頼できないバックグラウンド通知としてフレーム化して受け取り、それに対して行動することを拒否する可能性がありました。

    プロンプト入力にはモデルセレクタが含まれます。Claude は毎回実行時に選択されたモデルを使用します。
  </Step>

  <Step title="リポジトリを選択する">
    Claude が作業するための 1 つ以上の GitHub リポジトリを追加します。各リポジトリは実行の開始時にクローンされ、デフォルトブランチから開始されます。Claude は変更用に `claude/` プレフィックス付きブランチを作成します。
  </Step>

  <Step title="環境を選択する">
    ルーチン用の [cloud environment](/docs/ja/cloud-environments) を選択します。環境は、クラウドセッションがアクセスできるものを制御します。

    * **Network access**: 各実行中に利用可能なインターネットアクセスのレベルを設定します
    * **Environment variables**: Claude が各実行中に使用できる値を提供します。これらは [環境を使用する誰もが見ることができます](/docs/ja/cloud-environments#what-carries-over-from-your-setup)。Pro および Max プランでは、Claude が実行中に呼び出す API のキーを [API credentials](/docs/ja/cloud-environments#add-api-credentials) として保存してください。そのセクションには、認証情報を取得しないリクエストもリストされています
    * **Setup script**: ルーチンが必要とする依存関係とツールをインストールします。結果は [cached](/docs/ja/cloud-environments#environment-caching) されるため、スクリプトはセッションごとに再実行されません

    **Default** 環境は **Trusted** ネットワークアクセスで提供されます。これにより、[default allowlist](/docs/ja/cloud-environments#default-allowed-domains) のパッケージレジストリ、クラウドプロバイダー API、コンテナレジストリ、および一般的な開発ドメインのみがセッションのネットワークを通じて許可されます。ルーチンに追加するコネクタは Anthropic のサーバーを通じてサービスに到達するため、許可リストの変更は必要ありません。ルーチンが独自のサービスに直接到達する必要がある場合、またはそのリスト外のドメインに到達する必要がある場合は、実行前に環境の [network access](/docs/ja/cloud-environments#network-access) を編集してください。別の環境を使用するには、最初に [create one](/docs/ja/cloud-environments#configure-your-environment) してください。
  </Step>

  <Step title="トリガーを選択する">
    **Select a trigger** で、ルーチンの開始方法を選択します。1 つのトリガータイプを選択するか、複数を組み合わせることができます。

    <Tabs>
      <Tab title="Schedule">
        定期実行のプリセット頻度を選択するか、特定のタイムスタンプで 1 回限りの実行をスケジュールします。タイムゾーン処理、スタッガー、カスタム cron 間隔、および 1 回限りの実行については、[Add a schedule trigger](#add-a-schedule-trigger) を参照してください。
      </Tab>

      <Tab title="GitHub event">
        リポジトリ、反応するイベント、およびオプションのフィルタを選択します。サポートされているイベントとフィルタフィールドの完全なリストについては、[Add a GitHub trigger](#add-a-github-trigger) を参照してください。
      </Tab>

      <Tab title="API">
        ここで **API** を選択してから、ルーチンを保存します。URL とトークンはルーチンが保存された後に生成されます。ルーチン ID に依存するためです。URL をコピーしてトークンを生成するには、[Add an API trigger](#add-an-api-trigger) を参照してください。
      </Tab>
    </Tabs>
  </Step>

  <Step title="コネクタを確認する">
    フォームの下部の **Connectors** で、接続されたすべての [MCP connectors](/docs/ja/mcp) がデフォルトで含まれます。ルーチンが必要としないものを削除します。Claude は、実行中に権限を求めずに、含まれたコネクタからすべてのツール（書き込みを含む）を使用できます。
  </Step>

  <Step title="ルーチンを作成する">
    **Create** をクリックします。ルーチンはリストに表示され、トリガーの 1 つが一致する次の時間に実行されます。すぐに実行を開始するには、ルーチンの詳細ページで **Run now** をクリックします。

    各実行は、他のセッションの横に新しいセッションを作成します。Claude が何をしたかを確認し、変更を確認し、プルリクエストを作成できます。
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  CLI から作成する
</h3>

任意のセッションで `/schedule` を実行して、スケジュール済みルーチンを会話形式で作成します。説明を直接渡すこともできます。`/schedule daily PR review at 9am` のような定期ルーチンまたは `/schedule clean up feature flag in one week` のような 1 回限りのルーチンの場合です。Claude はウェブフォームが収集するのと同じ情報（スケジュール、リポジトリ、プロンプト）について質問を進め、ルーチンをアカウントに保存します。コマンドは `/routines` というエイリアスでも利用可能です。

成功した開始は会話のように見えます。Claude はスケジュール、リポジトリ、プロンプトについてのフォローアップ質問をしてから保存します。代わりに Claude が認証が必要であるか、リモート claude.ai アカウントに接続できないと返信する場合、ルーチンは作成されていません。[Troubleshooting](#troubleshooting) を参照してください。

CLI の `/schedule` はスケジュール済みルーチンを作成します。API トリガーを追加するには、[claude.ai/code/routines](https://claude.ai/code/routines) のウェブでルーチンを編集します。ウェブまたは CLI から [GitHub trigger](#add-a-github-trigger) を追加できます。CLI パスには Claude Code v2.1.225 以降が必要です。

スケジュールトリガーがないルーチン（API 呼び出しまたは GitHub イベントによってのみ開始されるなど）には次の実行時間がなく、Claude がそれを保存または更新するときに CLI は何も表示しません。v2.1.211 より前では、CLI はこれらのルーチンの年 1 の次の実行時間を報告していました。

<h2 id="configure-triggers">
  トリガーを構成する
</h2>

ルーティンはトリガーの 1 つが一致したときに開始されます。同じルーティンにスケジュール、API、GitHub トリガーの任意の組み合わせを接続でき、ルーティンの編集フォームの **Select a trigger** セクションからいつでも追加または削除できます。

<h3 id="add-a-schedule-trigger">
  スケジュールトリガーを追加する
</h3>

スケジュールトリガーは定期的なペースでルーティンを実行するか、特定の将来の時刻に 1 回実行します。**Select a trigger** セクションでプリセット周波数を選択します。時間ごと、毎日、平日、または毎週。時間はローカルゾーンで入力され、自動的に変換されるため、ルーティンはクラウドインフラストラクチャがどこにあるかに関係なく、その壁時計時間で実行されます。

スタガーのため、実行はスケジュール時刻の数分後に開始される可能性があります。オフセットは各ルーティンで一貫しています。

2 時間ごと、または毎月の最初など、カスタム間隔の場合は、フォームで最も近いプリセットを選択してから、CLI で `/schedule update` を実行して特定の cron 式を設定します。最小間隔は 1 時間です。より頻繁に実行される式は拒否されます。

<h4 id="schedule-a-one-off-run">
  1 回限りの実行をスケジュールする
</h4>

1 回限りのスケジュールは、特定のタイムスタンプでルーティンを 1 回だけ実行します。週の後半に自分自身に通知したり、ロールアウトが完了した後にクリーンアップ PR を開いたり、アップストリームの変更がランディングしたときにフォローアップタスクをキックオフしたりするために使用します。ルーティンが実行された後、自動的に無効になり、Web UI は **Ran** としてマークします。再度実行するには、ルーティンを編集して新しい 1 回限りの時刻を設定します。

CLI から 1 回限りの実行を作成するには、自然言語で時刻を説明します。Claude は現在の時刻に対してフレーズを解決し、保存する前に絶対タイムスタンプを確認します。

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

定期的なスケジュールと同じローカル UTC 変換が 1 回限りのタイムスタンプに適用されます。

1 回限りの実行は日次ルーティン実行上限にカウントされません。詳細については、[使用量と制限](#usage-and-limits) を参照してください。

<h3 id="add-an-api-trigger">
  API トリガーを追加する
</h3>

API トリガーはルーティンに専用 HTTP エンドポイントを提供します。ルーティンのベアラートークンでエンドポイントに POST すると、新しいセッションが開始され、セッション URL が返されます。これを使用して Claude Code をアラートシステム、デプロイパイプライン、内部ツール、または認証済み HTTP リクエストを実行できる任意の場所に接続します。

API トリガーは Web から既存のルーティンに追加されます。CLI は現在、トークンを作成または取り消すことができません。

<Steps>
  <Step title="ルーティンを編集用に開く">
    [claude.ai/code/routines](https://claude.ai/code/routines) に移動し、API 経由でトリガーするルーティンをクリックしてから、ルーティン名の横のメニューを開き、**Edit** を選択します。
  </Step>

  <Step title="API トリガーを追加する">
    **Instructions** ボックスの下の **Select a trigger** セクションまでスクロールし、**Add another trigger** をクリックして、**API** を選択します。
  </Step>

  <Step title="URL をコピーしてトークンを生成する">
    モーダルはこのルーティンの URL とサンプル curl コマンドを表示します。URL をコピーしてから、**Generate token** をクリックしてトークンをすぐにコピーします。トークンは 1 回表示され、後で取得できないため、アラートツールのシークレットストアなどの安全な場所に保存してください。
  </Step>

  <Step title="エンドポイントを呼び出す">
    URL に POST するときに `Authorization: Bearer` ヘッダーでトークンを送信します。以下の [ルーティンをトリガーする](#trigger-a-routine) セクションに完全な例が示されています。
  </Step>
</Steps>

各ルーティンは独自のトークンを持ち、そのルーティンのトリガーのみにスコープされています。ローテーションまたは取り消すには、同じモーダルに戻り、**Regenerate** または **Revoke** をクリックします。

<h4 id="trigger-a-routine">
  ルーティンをトリガーする
</h4>

`Authorization` ヘッダーのベアラートークンで `/fire` エンドポイントに POST リクエストを送信します。リクエスト本文は、アラート本文またはログの失敗など、実行固有のコンテキスト用のオプションの `text` フィールドを受け入れます。保存されたプロンプトと共にルーティンに渡されます。値はフリーフォームテキストで、解析されません。JSON または別の構造化ペイロードを送信する場合、ルーティンはリテラル文字列として受け取ります。

`text` 値は、ルーティンに裸のメッセージとして到達しません。これは `<routine-fire-payload>` ブロックでラップされて到達し、信頼できないデータとしてラベル付けされ、ルーティン独自のプロンプトが言わない限り、Claude にその中の指示に従わないよう指示します。同じラップが Web UI の **Run now** で提供されるテキストに適用されます。

これは、ルーティンの保存されたプロンプトが火災テキストに対して行動するためにオプトインする必要があることを意味します。プロンプトを書いて、ペイロードを明示的に参照するか、例えば「routine-fire-payload ブロックで説明されているアラートを調査する」、またはルーティンはテキストを不活性コンテキストとして扱います。ベアラートークンを保持している人は誰でも `text` を送信できるため、ラッパーは漏洩したトークンからの火災テキストが、ルーティンへの直接指示ではなく、信頼できないデータとしてラベル付けされて到達するようにします。

以下の例は、シェルからルーティンをトリガーします。表示されているルーティン ID とトークンはプレースホルダーです。[API トリガーを追加する](#add-an-api-trigger) 時にコピーした URL とトークンで置き換えてください。そうしないと、リクエストは `401` 認証エラーで失敗します。

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

成功したリクエストは、新しいセッション ID と URL を含む JSON 本文を返します。

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

ブラウザでセッション URL を開いて、実行をリアルタイムで監視し、変更をレビューするか、会話を手動で続行します。

<Warning>
  `/fire` エンドポイントは `experimental-cc-routine-2026-04-01` ベータヘッダーの下で出荷されます。リクエストと応答の形状、レート制限、トークンセマンティクスは、機能がリサーチプレビュー段階にある間に変更される可能性があります。破壊的な変更は新しい日付付きベータヘッダーバージョンの背後で出荷され、最新の 2 つの前のヘッダーバージョンは引き続き機能するため、呼び出し元は移行する時間があります。
</Warning>

<h4 id="api-reference">
  API リファレンス
</h4>

すべてのエラー応答、検証ルール、フィールド制限を含む完全な API リファレンスについては、Claude Platform ドキュメントの [API 経由でルーティンをトリガーする](https://platform.claude.com/docs/en/api/claude-code/routines-fire) を参照してください。

`/fire` エンドポイントは claude.ai ユーザーのみが利用でき、Claude Platform API サーフェスの一部ではありません。

<h3 id="add-a-github-trigger">
  GitHub トリガーを追加する
</h3>

GitHub トリガーは、接続されたリポジトリで一致するイベントが発生したときに、新しいセッションを自動的に開始します。Claude Code はイベント間でセッションを再利用しないため、2 つの PR 更新は 2 つの独立したセッションを生成します。

<Note>
  リサーチプレビュー中、GitHub webhook イベントはルーティンごとおよびアカウントごとの時間単位の上限の対象です。制限を超えるイベントはウィンドウがリセットされるまでドロップされます。現在の制限は [claude.ai/code/routines](https://claude.ai/code/routines) で確認してください。
</Note>

Claude GitHub App は、サブスクライブするリポジトリにインストールする必要があります。

* Web UI から GitHub トリガーを構成します。これは、アプリがない場合はインストールするよう促します。以下の手順に従って、Web から 1 つを構成してください。
* CLI から、最初に [GitHub App ページ](https://github.com/apps/claude) からアプリをインストールしてから、Claude に既存のルーティンに GitHub トリガーを接続するよう依頼します。例えば `/schedule add a GitHub trigger to my nightly review for pull requests opened in acme/webapp`。CLI パスには Claude Code v2.1.225 以降が必要です。Claude がトリガーを追加すると、トリガーが発火するルーティンへのリンクで返信します。

<Steps>
  <Step title="ルーティンを編集用に開く">
    [claude.ai/code/routines](https://claude.ai/code/routines) に移動し、ルーティンをクリックしてから、ルーティン名の横のメニューを開き、**Edit** を選択します。
  </Step>

  <Step title="GitHub イベントトリガーを追加する">
    **Select a trigger** セクションまでスクロールし、**Add another trigger** をクリックして、**GitHub event** を選択します。

    <Note>
      CLI で `/web-setup` を実行するとリポジトリアクセスがクローン用に付与されますが、Claude GitHub App はインストールされず、webhook 配信は有効になりません。
    </Note>
  </Step>

  <Step title="トリガーを構成する">
    リポジトリを選択し、[サポートされているイベント](#supported-events) リストからイベントを選択し、オプションでフィルタを追加します。トリガーを保存します。
  </Step>
</Steps>

<h4 id="supported-events">
  サポートされているイベント
</h4>

GitHub トリガーは、次のいずれかのイベントカテゴリにサブスクライブできます。各カテゴリ内で、`pull_request.opened` などの特定のアクションを選択するか、カテゴリ内のすべてのアクションに反応することができます。

| イベント    | トリガーのタイミング                                    |
| :------ | :-------------------------------------------- |
| プルリクエスト | PR がオープン、クローズ、割り当て、ラベル付け、同期、またはその他の方法で更新されたとき |
| リリース    | リリースが作成、公開、編集、または削除されたとき                      |

<h4 id="filter-pull-requests">
  プルリクエストをフィルタリングする
</h4>

フィルタを使用して、新しいセッションを開始するプルリクエストを絞り込みます。すべてのフィルタ条件がルーティンをトリガーするために一致する必要があります。利用可能なフィルタフィールドは次のとおりです。

| フィルタ    | マッチ                  |
| :------ | :------------------- |
| 作成者     | PR 作成者の GitHub ユーザー名 |
| タイトル    | PR タイトルテキスト          |
| 本文      | PR 説明テキスト            |
| ベースブランチ | PR がターゲットするブランチ      |
| ヘッドブランチ | PR が由来するブランチ         |
| ラベル     | PR に適用されたラベル         |
| ドラフト    | PR がドラフト状態かどうか       |
| マージ済み   | PR がマージされたかどうか       |

各フィルタはフィールドを演算子とペアにします。等しい、含む、で始まる、の 1 つ、の 1 つではない、または正規表現に一致します。

`matches regex` 演算子はフィールド値全体をテストし、その中の部分文字列ではありません。`hotfix` を含むタイトルに一致させるには、`.*hotfix.*` を記述します。周囲の `.*` がない場合、フィルタは前後に何もない正確に `hotfix` であるタイトルのみに一致します。正規表現構文なしのリテラル部分文字列マッチングの場合は、代わりに `contains` 演算子を使用してください。

いくつかのフィルタ組み合わせの例。

* **認証モジュールレビュー**: ベースブランチ `main`、ヘッドブランチに `auth-provider` を含む。認証に触れる PR を焦点を絞ったレビュアーに送信します。
* **レビュー準備完了のみ**: ドラフト `false`。ドラフトをスキップして、ルーティンが PR がレビュー準備完了のときのみ実行されるようにします。
* **ラベルゲート付きバックポート**: ラベルに `needs-backport` を含む。メンテナーが PR にタグを付けたときのみ、別のブランチへのポートルーティンをトリガーします。

<h2 id="manage-routines">
  ルーチンを管理する
</h2>

リストのルーチンをクリックして、その詳細ページを開きます。詳細ページには、ルーチンのリポジトリ、コネクタ、プロンプト、スケジュール、API トークン、GitHub トリガー、および過去の実行のリストが表示されます。

<h3 id="view-and-interact-with-runs">
  実行を表示して操作する
</h3>

任意の実行をクリックして、フルセッションとして開きます。そこから Claude が何をしたかを確認し、変更をレビューし、プルリクエストを作成するか、会話を続けることができます。各実行セッションは他のセッションと同じように機能します。セッションタイトルの横のドロップダウンメニューを使用して、名前変更、アーカイブ、または削除を行います。

<Note>
  実行リストの緑色のステータスは、セッションが開始され、インフラストラクチャエラーなしで終了したことを意味します。これはプロンプト内のタスクが成功したことを意味しません。実行を開いてトランスクリプトを読み、Claude が実際に何をしたかを確認してください。ブロックされたネットワークリクエスト、不足しているコネクタツール、およびタスクレベルの失敗はすべて、ステータスインジケータではなくそこに表示されます。
</Note>

<h3 id="edit-and-control-routines">
  ルーチンを編集して制御する
</h3>

ルーチン詳細ページから、以下を実行できます。

* **Run now** をクリックして、次のスケジュール時刻を待たずに実行を直ちに開始します。オプションで実行固有のテキストを指定できます。これは API トリガーの `text` フィールドと同じ方法でルーチンに到達します。
* ページの上部にあるオン/オフスイッチを使用して、スケジュールを一時停止または再開します。一時停止されたルーチンは設定を保持しますが、再度有効にするまで実行されません。
* ルーチンの名前の横にあるメニューを開き、**Edit** を選択して、名前、プロンプト、リポジトリ、環境、コネクタ、またはルーチンのトリガーのいずれかを変更します。**Select a trigger** セクションは、スケジュール、API トークン、および GitHub イベントトリガーを追加または削除する場所です。
* 同じメニューを開き、**Delete** を選択してルーチンを削除します。

<h3 id="manage-routines-from-the-cli">
  CLI からルーチンを管理する
</h3>

CLI は既存のルーチンの管理をサポートしています。`/schedule list` を実行してすべてのルーチンを表示し、`/schedule update` で 1 つを変更するか、`/schedule run` で直ちにトリガーします。

また、ルーチンの実行履歴について質問することもできます。例えば `/schedule why did my nightly review do nothing this morning?` のようにです。Claude はルーチンの最近の実行をステータスと共にリストし、[各実行をウェブで開く](#view-and-interact-with-runs)ためのリンクを提供し、実行のログを読んでツールエラー、権限拒否、最終結果を含めて何が起こったかを説明します。Claude Code v2.1.227 以降が必要です。

<h3 id="repositories-and-branch-permissions">
  リポジトリとブランチの権限
</h3>

ルーチンはリポジトリをクローンするために GitHub アクセスが必要です。CLI で `/schedule` を使用してルーチンを作成する場合、Claude はアカウントが実行元のリポジトリに対して GitHub アクセスを持っているかどうかを確認し、持っていない場合はアクセスを許可する方法を名前付きで示すセットアップノートを追加します。アクセスを許可する 2 つの方法については、[GitHub 認証オプション](/docs/ja/claude-code-on-the-web#github-authentication-options)を参照してください。

GitHub 接続が実行予定時に不足しているか期限切れの場合、ルーチンは最大 72 時間まで実行をスキップします。その期間内に GitHub を再接続すると、ルーチンは自動的に再開されます。接続なしで 72 時間経過すると、ルーチンはオフになり、GitHub を再接続した後に再度オンにします。

追加する各リポジトリは、すべての実行でクローンされます。Claude はリポジトリのデフォルトブランチから開始します。ただし、プロンプトで別の方法を指定する場合を除きます。

Claude はその作業を `claude/` で始まるブランチにプッシュします。これらは常に受け入れられます。プロンプトが Claude に別のブランチにプッシュするよう指示する場合、Claude Code は最初にプッシュをチェックし、以下のいずれかが当てはまる場合は拒否します。

* ブランチは GitHub で保護されている
* 他の誰かがそのブランチからのオープンなプルリクエストを持っている
* ブランチは自分以外の誰かによって作成されたコミットを含んでいる

<h3 id="connectors">
  コネクタ
</h3>

ルーチンは接続された MCP コネクタを使用して、各実行中に外部サービスから読み取り、外部サービスに書き込むことができます。例えば、サポートリクエストをトリアージするルーチンは Slack チャネルから読み取り、Linear で問題を作成する可能性があります。

コネクタは、アカウント上の [claude.ai インテグレーション](/docs/ja/mcp#use-mcp-servers-from-claude-ai)です。CLI で `claude mcp add` を使用してローカルに追加した MCP サーバーはマシンに保存されており、claude.ai アカウントには保存されないため、コネクタリストに表示されません。ルーチンでそれらのサーバーの 1 つを使用するには、[claude.ai/customize/connectors](https://claude.ai/customize/connectors) でコネクタとして追加します。1 つのリポジトリを持つルーチンの場合、代わりにコミットされた [`.mcp.json`](/docs/ja/mcp#project-scope) で宣言できます。これはクローンされたリポジトリの一部です。

ルーチンを作成する場合、現在接続されているすべてのコネクタがデフォルトで含まれます。実行中に Claude がアクセスできるツールを制限するために、不要なものを削除します。ルーチンフォームから直接コネクタを追加することもできます。

ルーチンフォーム外でコネクタを管理または追加するには、[claude.ai/customize/connectors](https://claude.ai/customize/connectors) にアクセスするか、CLI で `/schedule update` を使用します。

<h3 id="environments-and-network-access">
  環境とネットワークアクセス
</h3>

各ルーチンは、ネットワークアクセス、環境変数、およびセットアップスクリプトを制御する [クラウド環境](/docs/ja/cloud-environments)を使用します。ルーチンはすべての実行でその環境のネットワークポリシーを継承します。

**Default** 環境は **Trusted** ネットワークアクセスを使用します。これにより、セッションのネットワークを通じて [デフォルト許可リスト](/docs/ja/cloud-environments#default-allowed-domains)のみが許可されます。そのパスの外のホストへのリクエストは `403` と `x-deny-reason: host_not_allowed` で失敗します。MCP コネクタトラフィックは Anthropic のサーバーを通じてルーティングされるため、ルーチンに追加するコネクタは **Allowed domains** にホストを追加しなくても機能します。[コネクタ](#connectors)の下で不要なコネクタを削除します。

1 つ以上の独自の環境で追加のドメインを許可するには、以下の手順に従います。[組織共有環境](/docs/ja/cloud-environments#organization-shared-environments)はここで読み取り専用で開くため、所有者は [管理設定](https://claude.ai/admin-settings)の **Cloud environments** ページからそのネットワークアクセスを変更します。

<Steps>
  <Step title="ルーチンを編集用に開く">
    ルーチンの詳細ページで、ルーチンの名前の横にあるメニューを開き、**Edit** を選択します。
  </Step>

  <Step title="環境セレクタを開く">
    **Instructions** ボックスの下で、**Default** などの環境の名前を示すクラウドアイコンを選択します。
  </Step>

  <Step title="環境設定を開く">
    リスト内の環境にマウスを置き、右側に表示される設定アイコンをクリックします。
  </Step>

  <Step title="ネットワークアクセスレベルを変更する">
    **Update cloud environment** ダイアログで、**Network access** を **Custom** に変更し、**Allowed domains** にドメインを入力します。**Also include default list of common package managers** をチェックして、カスタムドメインと共に [デフォルト許可リスト](/docs/ja/cloud-environments#default-allowed-domains)を保持します。制限のないアクセスの場合は、代わりに **Full** を選択します。
  </Step>

  <Step title="保存">
    **Save changes** をクリックします。新しいポリシーは次の実行から適用されます。
  </Step>
</Steps>

アクセスレベルとデフォルト許可リストの詳細については、[ネットワークアクセス](/docs/ja/cloud-environments#network-access)を参照してください。

<h2 id="usage-and-limits">
  使用と制限
</h2>

ルーティンは対話型セッションと同じ方法でサブスクリプション使用量を削減します。標準的なサブスクリプション制限に加えて、ルーティンはアカウントごとに 1 日に開始できる実行数の上限があります。現在の消費と残りの日次ルーティン実行数は [claude.ai/code/routines](https://claude.ai/code/routines) または [claude.ai/settings/usage](https://claude.ai/settings/usage) で確認してください。

ルーティンが日次上限またはサブスクリプション使用制限に達したとき、使用クレジットが有効な組織は、メーター付きオーバーエッジでルーティンを実行し続けることができます。使用クレジットがない場合、ウィンドウがリセットされるまで追加実行は拒否されます。[claude.ai/settings/usage](https://claude.ai/settings/usage) で使用クレジットを有効にしてください。Team プランと Enterprise プランでは、管理者が [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) で組織の使用クレジットを有効にします。

1 回限りの実行は日次ルーティン実行上限にはカウントされません。他のセッションと同じように通常のサブスクリプション使用量を削減します。

サブスクリプションが一時停止されている間、ルーティンは保留中になり、実行されません。サブスクリプションが再度アクティブになったら、ルーティンを再度有効にしてください。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="schedule-returns-unknown-command">
  `/schedule` が「Unknown command」を返す
</h3>

CLI は `/schedule` の要件の 1 つが満たされていない場合、このコマンドを非表示にします。コマンドメニューは入力中に「No commands match "/schedule"」と表示され、送信すると `Unknown command: /schedule` が返されます。ただし、以下のケースのうち異なる回答を示すものは除きます。

原因は通常、以下のいずれかです。

* Console API キー、[Anthropic プロフィールまたはフェデレーション認証情報](/docs/ja/authentication#anthropic-profiles-and-federation-credentials)、または Amazon Bedrock、Google Cloud の Agent Platform、Microsoft Foundry などのクラウドプロバイダーで認証されている。`/schedule` には claude.ai サブスクリプションログインが必要です。Console API キーまたはプロフィールを使用していて、フィーチャーフラグ取得が有効な場合、`/schedule` を送信すると代わりに `/schedule is available with Claude for Enterprise — ask your admin about migrating from API-key access` と表示されます。クラウドプロバイダーログインの場合、`Unknown command: /schedule` が表示されます。シェルで `ANTHROPIC_API_KEY` または `ANTHROPIC_AUTH_TOKEN` が設定されている場合、または `settings.json` で `apiKeyHelper` が設定されている場合は、これらが claude.ai ログインより優先されるため、まず削除してください。プロフィールまたはフェデレーション認証情報も優先されるため、それも無効にしてください
* 完全にサインアウトしており、API キーまたは他の認証情報がない。フィーチャーフラグ取得が有効な場合、`/schedule` を送信すると `/schedule requires a claude.ai subscription. Run /login to sign in with your claude.ai account.` と表示されます。v2.1.268 より前では、サインアウトしたセッションは Console API キーと同じ Claude for Enterprise メッセージを表示していました
* Claude Code on the web セッション内にいる。代わりに[ウェブ UI](https://claude.ai/code/routines) からルーチンを管理してください
* 組織のポリシーが[Claude Code on the web](/docs/ja/claude-code-on-the-web) を無効にしており、ルーチンはこれで実行されます。この場合、`/schedule` を送信すると [`Cloud sessions are disabled by your organization's policy`](/docs/ja/errors#cloud-sessions-are-disabled-by-your-organizations-policy) と回答されます。v2.1.268 より前は、`Unknown command: /schedule` が返されていました
* Owner がチームまたはエンタープライズ組織の[ルーチンを無効にしました](#routines-are-disabled-by-your-organizations-policy)。v2.1.227 より前は、このケースでもコマンドがまだ表示されており、Claude がルーチンを作成または実行しようとするときに claude.ai がそれを拒否していました

組織のポリシーがルーチンまたは Claude Code on the web を無効にしていない限り、CLI がどのように設定されているかに関わらず、[claude.ai/code/routines](https://claude.ai/code/routines) でルーチンを作成および管理できます。

<h3 id="routines-are-disabled-by-your-organizations-policy">
  「Routines are disabled by your organization's policy」
</h3>

チームまたはエンタープライズ組織の Owner が [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) で **Routines** トグルをオフにしている可能性があります。Claude Code v2.1.227 以降では、同じトグルが CLI の `/schedule` も非表示にします。これはサーバー側の組織設定であるため、ローカル設定から上書きすることはできません。Owner に組織のルーチンを有効にするよう依頼してください。

<h2 id="related-resources">
  関連リソース
</h2>

* [`/loop` とセッション内スケジューリング](/docs/ja/scheduled-tasks): オープン CLI セッション内でローカルタスクをスケジュール
* [Desktop スケジュール済みタスク](/docs/ja/desktop-scheduled-tasks): マシンで実行され、ローカルファイルへのアクセスを持つローカルスケジュール済みタスク
* [クラウド環境](/docs/ja/cloud-environments): クラウドセッションのネットワークアクセス、環境変数、セットアップスクリプトを構成
* [Projects](/docs/ja/claude-projects): Claude が複数のクラウドセッション全体で調整する継続的な作業。プロジェクトから作成されたルーチンはその **Routines** タブに表示されます
* [MCP コネクタ](/docs/ja/mcp): Slack、Linear、Google Drive などの外部サービスを接続
* [GitHub Actions](/docs/ja/github-actions): リポジトリイベントで CI パイプラインで Claude を実行
