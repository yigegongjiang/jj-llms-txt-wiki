> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# セッション出力をアーティファクトとして共有する

> Artifacts は Claude Code の成果物を claude.ai 上のライブでインタラクティブなページに変え、プライベートに保つ、組織と共有する、または公開リンクで公開することができます。

<Note>
  Artifacts は Pro、Max、Team、Enterprise プランで利用可能で、[`/login`](/docs/ja/setup#authenticate) でサインインしたセッションが必要です。要件の完全なセットについては、[利用可能性](#availability)を参照してください。
</Note>

[アーティファクト](https://claude.com/features/artifacts)は、Claude Code がセッションから claude.ai 上のプライベート URL に公開するライブでインタラクティブな Web ページです。ブラウザで開くと、セッションが続く間、その場で更新されます。他の人にも見てもらいたい場合は、ページヘッダーから共有します。

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="claude.ai/code/artifact で開かれたアーティファクト。ビューアヘッダーには、アーティファクトタイトル acme-funnel-fix、Share ボタン、著者アバターが表示されています。Share メニューが開いており、Always share latest version トグル、Sharing version 2 と表示されたバージョンピッカー、Everyone at Acme オーディエンスセレクタ、Copy link ボタンが表示されています。ヘッダーの下には、2 つのモバイルモックアップが並んで表示され、ファネルチャート、メトリクスカードの行が表示されています。" width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  アーティファクトを使用する場合
</h2>

ターミナルテキストが Claude が生成したものを表示するのに適さない場合にアーティファクトを使用してください。つまり、1 行ずつ読むよりも見たり操作したりする方が簡単な出力です。Claude はセッションが到達できるもの（コードベースや [接続されたツール](/docs/ja/mcp) を通じて取得するデータを含む）からページを構築するため、説明に段落が必要になるようなものを表示できます。たとえば、Claude に以下のことを依頼してください。

* 注釈付きの差分を使用してレビュアーをプルリクエストの説明をする
* セッションが既に取得したデータからダッシュボードをレンダリングする
* 複数の設計または実装オプションを並べて配置する
* 長いタスクが実行されている間に入力される調査タイムラインを保持する
* Slack に出力を貼り付ける代わりに、チームメイトにリンクを送信する
* [MCP コネクタを通じて新しいデータを取得](#pull-live-data-with-mcp-connectors)する状態ボードを公開する

これらに対応するプロンプトについては [構築できるもの](#what-you-can-build) を参照し、コネクタバックアップボードのプロンプトについては [MCP コネクタを使用してライブデータを取得](#pull-live-data-with-mcp-connectors) を参照してください。

<h3 id="what-an-artifact-is-not">
  アーティファクトではないもの
</h3>

アーティファクトは作業のキャプチャです。バックエンドのない 1 つの自己完結型ページなので、複数のルートを提供することはできません。バックエンドを備えたホストされた内部ツールの場合は、代わりに独自のインフラストラクチャにデプロイしてください。制限の完全なセットについては [ページの制約](#page-constraints) を参照してください。

<h2 id="create-an-artifact">
  アーティファクトを作成する
</h2>

Claude は出力がページに適している場合、自動的にアーティファクトを公開することがあります。また、直接リクエストすることもできます。リクエストするには、機能の名前を指定するか、希望する視覚的な出力をプレーンテキストで説明してください。テキストとして読むより見る方が簡単なもの（注釈付きの diff、チャート、比較するオプションのセットなど）が良い候補です。以下のプロンプトは 2 つの例です。詳細は[構築できるもの](#what-you-can-build)を参照してください。

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

場所を指定しない限り、Claude はページを HTML または Markdown ファイルとしてプロジェクト外の一時ディレクトリに書き込み、公開します。新しいアーティファクトを公開する場合、セッションの[権限モード](/docs/ja/permission-modes)を通じて処理されます。

* **Auto モード**：分類器がプロンプトの代わりに公開をレビューするため、Claude はプロンプトを表示せずにページを公開できます。セッションが開始される権限モードはプランによって異なります。詳細は[開始時の権限モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)を参照してください。
* **Manual および Accept edits モード**：Claude Code は権限を要求します。「Claude wants to publish deploy-failures.html, uploading it to claude.ai (Anthropic's servers) to host as the page "Deploy failures by service", private to you until you share it」のようなメッセージが表示される場合があります。**Yes** を選択して公開します。

アーティファクトを一度承認すると、Claude Code は再度質問することなく再公開し、以下の場合を含むいくつかのケースで再度質問します。

* Claude がページの[コネクタ呼び出し](#pull-live-data-with-mcp-connectors)や[ファイルダウンロード](#offer-a-file-download)などのランタイム機能を宣言する場合
* その後、[公開で共有](#share-an-artifact)した場合
* その後、特定の人またはあなたの組織と共有し、最新バージョンが視聴者が見るバージョンとして選択された場合

最初の公開後、Claude は URL を出力し、ブラウザが新しいページに開きます。[Remote Control](/docs/ja/remote-control)から claude.ai、Claude Desktop、または Claude モバイルアプリを通じてプロンプトを送信した場合、セッションを実行しているマシンではタブが開きません。ブラウザは、Claude がターミナルで入力したプロンプトからアーティファクトを再度公開する次回に開きます。任意の時点で `Ctrl+]` を押して、セッションの最新アーティファクトを再度開きます。

Claude はアーティファクトのタイトルと絵文字を選択し、両方が claude.ai の[アーティファクトギャラリー](#share-an-artifact)と共有リンクに表示されます。Claude はまた、チャートやカレンダーなど、ページが何であるかに一致するブラウザタブアイコンを選択することもできます。特定のタイトル、絵文字、またはタブアイコンが必要な場合は、Claude に要求してください。

新しいアーティファクトが公開されたときにブラウザが自動的に開くのを停止するには、環境で `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` を設定します。

Claude が公開できないと応答した場合、またはリンクなしでローカル HTML ファイルを書き込んだ場合、ツールはセッションに対して有効になっていません。[利用可能性](#availability)の要件を確認してください。

<h2 id="update-an-artifact">
  アーティファクトを更新する
</h2>

Claude にページを修正するよう依頼するか、長時間実行されるタスクが進行状況に応じて再度公開するようにします。Claude は基になるファイルを編集し、同じ URL に再度公開します。

```text wrap theme={null}
概要チャートの下に地域別の内訳を追加して再度公開します。
```

ページを開いているすべてのユーザーは、その場で更新を確認できます。公開するたびにバージョンになり、ページヘッダーの **Share** コントロールから、ビューアーに表示するバージョンを選択できます。

別のセッションからアーティファクトを更新するには、Claude にその URL を指定するか、[`/artifacts`](#find-an-artifact-again) で添付します。どちらもない場合、新しいセッションは既存のアーティファクトを更新する代わりに、新しいアーティファクトを作成します。

```text wrap theme={null}
https://claude.ai/code/artifact/5fbea6f3-... を本日の数値で更新します。
```

<h2 id="find-an-artifact-again">
  アーティファクトを再度見つける
</h2>

Claude Code で `/artifacts` を実行して、所有しているすべてのアーティファクトと共有されているすべてのアーティファクトをリストアップします。1 つを選択して、`o` を押してブラウザで開くか、`c` を押してそのリンクをコピーします。`Enter` キーを押して現在のセッションに添付します。v2.1.216 より前のバージョンでは、`Enter` キーでブラウザで開きました。Claude Code は claude.ai アカウントからリストを読み込むため、新しいセッションで機能し、リンクがターミナルからスクロールアウトした後の `/clear` の後でも機能します。Claude Code v2.1.208 以降が必要です。

<h2 id="share-an-artifact">
  アーティファクトを共有する
</h2>

新しいアーティファクトは最初、あなただけに表示されます。共有するには、ブラウザでアーティファクトを開き、ページヘッダーの **Share** コントロールを使用してください。ヘッダーには [claude.ai/code/artifacts](https://claude.ai/code/artifacts) のギャラリーへのリンクもあり、作成したすべてのアーティファクトが一覧表示されます。

組織内のビューアーは、ページを公開した人を確認できます。組織内で共有されたアーティファクトでは、あなたの名前がタイトルメニューに表示され、公開アーティファクトでは、組織内のサインイン済みビューアーのページヘッダーに表示されます。公開リンクをサインインせずに開いたビューアー、または組織外からアクセスしたビューアーには、あなたの名前の代わりに `Content is user-generated and unverified.` というラベルが表示されます。

共有できる相手はプランによって異なります。

* **組織内**: Team プランと Enterprise プランでは、組織内の特定のユーザーまたは全員にアクセス権を付与できます。ビューアーは、ページを表示するために claude.ai に組織のメンバーとしてサインインします。
* **公開**: インターネット上の誰でも開くことができるリンクを共有でき、claude.ai へのサインインは不要です。Pro プランと Max プランでは、公開リンクがアーティファクトを共有する唯一の方法です。Team プランと Enterprise プランでは、Owner が [組織に対して公開共有を有効にする](#control-public-sharing) まで、公開共有はオフになっています。

<h3 id="let-someone-edit-with-you">
  他のユーザーと一緒に編集する
</h3>

共有相手はデフォルトではビューアーです。公開した各バージョンを表示できますが、ページを変更することはできません。Team プランと Enterprise プランでは、ユーザーをエディターにすることもできます。共有ダイアログで、ユーザーを追加し、その役割を **viewer** から **editor** に切り替えます。

エディターは、[別のセッションからアーティファクトを更新する](#update-an-artifact) のと同じ方法で新しいバージョンを公開します。アーティファクトの URL を Claude に提供するか、[`/artifacts`](#find-an-artifact-again) から添付し、Claude が現在のコンテンツを取得して変更を反映して再公開します。ページを開いているすべてのユーザーが各更新をリアルタイムで確認できます。

<h2 id="read-an-artifact-shared-with-you">
  共有されたアーティファクトを読む
</h2>

誰かがアーティファクトを共有した場合、Claude にそれを読ませることができます。Claude にその URL を提供するか、[`/artifacts`](#find-an-artifact-again) から添付してください。

Claude は他の人が書いたページを、[WebFetch](/docs/ja/tools-reference#webfetch-tool-behavior) でウェブページを読む方法と同じように読みます。つまり、生のページではなく、質問した内容の要約を取得し、その要約はページに書き込まれた指示を報告しますが、それらを実行する代わりに報告するのです。Claude Code はまた、ページの完全なソースをローカルファイルに保存します。Claude は、アーティファクトを [エディター](#let-someone-edit-with-you) として再発行する場合など、正確なコンテンツが必要な場合にそのファイルを開くことができます。

<h2 id="collect-comments-on-an-artifact">
  アーティファクトのコメントを収集する
</h2>

組織内でアーティファクトを共有すると、共有相手はページにコメントを残すことができ、Claude がそのコメントを読んで返信することができます。Claude Code v2.1.221 以降と Team または Enterprise プランが必要です。これは、[組織内で共有](#share-an-artifact)したアーティファクトのみがコメントを受け付けるためです。Claude がコメントを読む場合は 2 つあります。

* **Claude に読むよう依頼する場合**：Claude にアーティファクトの URL を提供し、コメントを求めます。Claude は各スレッドをリストアップし、アーティファクトを編集できるユーザーが送信したコメントをマークします。
* **アーティファクトを編集できるユーザーが Claude にコメントを送信する場合**：ページのスレッドで、**Send to Claude** でコメントを送信するか、その中で `@claude` にメンションします。どちらの方法でも、スレッドが有効になります。

Claude は有効になったスレッドにのみ返信または解決できます。その他のスレッドは、ユーザーがページで解決するまで開いたままになります。ビューアーは、各返信が Claude から送信されたものとして表示されます（あなた経由で）。

アーティファクトを公開共有する場合、ビューアーはコメントできません。ページに「`Comments aren't available while this Artifact is shared publicly.`」と表示されます。既にコメントスレッドがあるアーティファクトを公開リンクに切り替えるには、まずスレッドを削除してください。

コメントを自分で読むよう Claude に依頼するには、URL を提供します。

```text wrap theme={null}
Read the comments on https://claude.ai/code/artifact/5fbea6f3-... and make the changes the commenters ask for.
```

Claude がコメントを読めないと言う場合は、バージョン、セッション、フィーチャーフラグ設定を確認してください。

* Claude Code v2.1.221 以降を実行しています。
* Claude Code をインストールした後、または v2.1.221 より前のバージョンからアップグレードした後の最初のセッションではありません。[インストールまたはアップグレード後の最初のセッション](/docs/ja/env-vars#first-session-after-an-install-or-upgrade)では、Claude はまだコメントを読めない可能性があります。新しいセッションを開始して、もう一度試してください。
* フィーチャーフラグ取得をオフにしていません。

<h3 id="let-claude-reply-to-comments-on-its-own">
  Claude にコメントに自動で返信させる
</h3>

セッションがアーティファクトを公開した後、Claude Code はセッションが実行されている限り、そのアーティファクトのコメントを監視します。アーティファクトを編集できるユーザーが Claude にコメントを送信すると、すぐにセッションに到達し、Claude はスレッドを読んで、あなたに尋ねることなく返信できます。

Claude Code v2.1.228 以降が必要です。[フィーチャーフラグ取得](/docs/ja/env-vars#features-that-need-feature-flag-fetching)をオフにした場合、Claude Code はコメントを監視しません。

[権限モード](/docs/ja/permission-modes)によって、送信されたコメントが到着したときに Claude が何をするかが決まります。

* **Claude が自動で返信する**：権限モードで Claude が確認を求めずに返信を投稿できる場合、Claude はスレッドを読んで返信し、コメントが変更を求めている場合はアーティファクトを編集します。`Auto-replied to comment thread on Artifact: <name>` または `Auto-edited Artifact: <name> in response to a comment thread` が表示されます。
* **Claude があなたを待つ**：プランモード外で、返信の投稿に承認が必要な場合、`Comments are waiting on Artifact: <name>` が表示されます。その後、Claude はスレッドを読む承認を求め、返信を投稿する承認を再度求めます。
* **Claude がプランモードで一時停止する**：`Comments are waiting on Artifact: <name>` が表示され、Claude はプランモードを終了して読んで返信するよう求めるまで返信しません。

Claude は、1 時間以内にそのアーティファクトで 60 件の送信されたコメントまたはスレッド有効化を処理した後、アーティファクトへの自動返信を停止します。`Comments are waiting on Artifact: <name>` が 1 回表示され、Claude はその時間のコメントが古くなると再開します。

`/tasks` を実行して、セッションが監視している各アーティファクトをライブアップデートタスクとしてリストアップされたものを確認します。以下のいずれかの方法で、Claude がアーティファクトに自動で返信するのを停止できます。

* **アイドルプロンプトで Ctrl+C を 1 回押す**：Claude はセッションが監視しているすべてのアーティファクトへの返信を一時停止します。次のメッセージを送信した後、返信が再開されます。
* **`/tasks` でタスクを停止する**：Claude はそのアーティファクトへの返信を停止し、そこで返信を再開するよう求めるまで停止したままになります。アーティファクトを再度公開しても返信は再開されず、セッションを再開しても停止は適用されたままになります。
* **3 秒以内に `Ctrl+X Ctrl+K` を 2 回押す**：[すべての実行中のバックグラウンドサブエージェントを停止](/docs/ja/interactive-mode#general-controls)するコードは、セッションの残りの間、Claude がすべてのアーティファクトに返信するのも停止します。Claude に返信を再開するよう求めても、この停止は元に戻りません。

コメントを配信するサービスが利用できなくなるか、応答を停止した場合、Claude Code はしばらく再接続を試み、その後、セッションが監視していた各アーティファクトの監視を停止します。

<h2 id="pull-live-data-with-mcp-connectors">
  MCP コネクタで ライブデータを取得する
</h2>

アーティファクトは、誰かがそれを表示するたびに [MCP コネクタ](/docs/ja/mcp#use-mcp-servers-from-claude-ai) を呼び出すことができるため、ページはセッションから取得したスナップショットではなく、現在のデータを表示します。アーティファクトからのコネクタ呼び出しは Pro、Max、Team、Enterprise プランで利用可能であり、Claude Code v2.1.209 以降が必要です。以前のバージョンでは、Claude はセッション中に収集したデータでページを公開します。

コネクタバックアップページを作成するには、プロンプトでコネクタと必要なデータを指定します。

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude はページの公開の一部として、ページが呼び出す可能性のあるコネクタを宣言し、ページはその宣言の外側のコネクタを呼び出すことはできません。claude.ai アカウントからのコネクタのみが対象となります。Claude はそれらを宣言で指定し、ページを表示するときに、各呼び出しは [表示アカウント独自の接続を通じて実行されます](#how-connector-calls-work-for-viewers)。`.mcp.json` など Claude Code で設定するローカル MCP サーバーは、Claude がページを構築している間にデータを提供できますが、公開されたページはそれらを呼び出すことはできません。

ページはロード時にデータを取得し、間隔で更新するか、表示者がページの更新コントロールを使用するときに更新できます。応答はビューアのブラウザにキャッシュされるため、再度開いたページはキャッシュされた応答からすぐにレンダリングされ、その後新しい結果で更新されます。

<h3 id="how-connector-calls-work-for-viewers">
  ビューアのコネクタ呼び出しの仕組み
</h3>

公開されたページがコネクタを呼び出すとき、呼び出しはそれを公開した人のアカウントではなく、ページを表示している人のアカウントを使用します。

* **各ビューアは独自のコネクタを使用します**。呼び出しは表示アカウントの接続されたツールを通じて行われるため、同じダッシュボードを開く 2 人は、アカウントがアクセスできる内容に応じて異なるデータを表示できます。ページは誰の認証情報も表示しません。claude.ai はページに代わって呼び出しを行います。
* **ビューアは最初にアクセスを承認します**。claude.ai は、ページの最初のコネクタ呼び出しの前に、各ビューアに権限を求めます。ビューアが拒否した場合、またはページが使用するコネクタを接続していない場合でも、ページはライブセクションなしで表示されます。
* **アクションもビューアのアカウントを使用します**。ページは、メッセージの投稿や問題の更新など、副作用を伴うコネクタツールを呼び出すコントロールを提供できます。アクションは、コントロールを選択した人のアカウントを通じて実行されます。

コネクタバックアップページを共有する予定がある場合は、Claude に各ライブセクションに必要なコネクタを指定するフォールバックメッセージを含めるよう依頼してください。接続が不足しているビューアには、空のセクションの代わりに接続する内容が表示されます。

コネクタを呼び出すアーティファクトは、どのプランでも公開リンクで共有することはできません。Team および Enterprise プランでは、プライベートに保つか、[組織内で共有](#share-an-artifact) することができます。公開リンクが唯一の共有方法である Pro および Max プランでは、コネクタバックアップアーティファクトはあなたのみにプライベートのままです。

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  ページがビューアのライブデータを表示しない
</h3>

コネクタバックアップページがレンダリングされても、共有した人のライブセクションが空のままの場合は、これらの原因を確認してください。

* **ビューアがコネクタを接続していない**。コネクタはアカウントごとであるため、各ビューアはページが呼び出すすべてのコネクタへの独自の接続が必要です。claude.ai の **Settings > Connectors** で接続を追加してから、ページを再度読み込むことができます。
* **ビューアが権限要求を拒否した**。拒否はそのページロードの残りの間続きます。ページを再度読み込むと、権限要求が戻ります。
* **組織のコネクタ呼び出しがオフになっている**。所有者は管理設定で [**Enable artifact connectors** トグル](#control-connector-calls-from-artifacts) を制御します。
* **ページがコネクタが公開していないツール名を呼び出している**。影響を受けたセクションはあなたを含むすべての人に対して空のままです。これは、ページが、独自のツールのみを公開するゲートウェイスタイルコネクタの背後にある個別のツールに名前を付けるときに発生する可能性があります。Claude にページが呼び出すツール名を修正して再度公開するよう依頼してください。

  Claude がページを公開し、そのコネクタのツールがセッションで利用可能な場合、Claude Code はページが宣言するツール名をそれらと照合し、一致しない名前について Claude に警告し、一致しない場合は公開を拒否します。v2.1.265 より前では、それらをチェックせずにページを公開していました。

<h2 id="offer-a-file-download">
  ファイルダウンロードを提供する
</h2>

アーティファクトは、ページが生成するファイル（テーブルの CSV エクスポートやチャートの PNG など）をビューアーに提供できます。ビューアーはページ上のダウンロードコントロール（ボタンなど）を通じてそれを保存します。ファイルダウンロードは claude.ai が提供するランタイム機能で、アカウント単位で有効になるため、Claude はコントロールを構築する前にアカウントがこの機能を持っているかどうかを確認します。

ビューアーは通常のダウンロードリンクやページ上のスクリプトからファイルを保存することはできません。これは、claude.ai のアーティファクトビューアーがページ自体が開始するダウンロード（`data:` または `blob:` URL へのリンクを含む）をブロックするためです。ページにこのような方法で構築されたダウンロードボタンがある場合は、Claude にダウンロード機能を使用して再構築するよう依頼してください。

ファイルを提供するには、プロンプトでコントロールとファイル形式をリクエストしてください。

```text wrap theme={null}
Add a button that downloads this table as a CSV file.
```

Claude はダウンロード機能を公開の一部として宣言します。これは[コネクターを宣言](#pull-live-data-with-mcp-connectors)する方法と同じです。

<h2 id="what-you-can-build">
  構築できるもの
</h2>

アーティファクトは単一の HTML ページであるため、HTML、CSS、およびインラインの JavaScript で表現できるものはすべてスコープ内です。以下のパターンが最も頻繁に発生します。

<h3 id="walk-through-a-change">
  変更を通じて説明する
</h3>

差分またはデザイン変更をレンダリングするページを、関連する行の横に注釈を付けてリクエストします。これにより、レビュアーは説明から再構築するのではなく、コードの横にあなたの推論を読むことができます。

```text wrap theme={null}
このプルリクエストを通じて説明するアーティファクトを作成してください。差分をマージン注釈でレンダリングし、重大度別に検出結果をカラーコーディングしてください。
```

<h3 id="compare-alternatives">
  代替案を比較する
</h3>

1 つのページに複数のバリアントをリクエストして、互いに評価できるようにします。これはレイアウト、コピー、API シェイプ、または実装計画に機能します。

```text wrap theme={null}
設定パネルの 4 つの明らかに異なるレイアウトを含むアーティファクトを作成してください。密度とグループ化を変更し、それぞれの下に 1 行のトレードオフを含むグリッドとしてレイアウトしてください。
```

<h3 id="tune-with-interactive-controls">
  インタラクティブコントロールで調整する
</h3>

スライダー、トグル、または入力フィールドを調整しているものにバインドするようリクエストします。これにより、説明する代わりに値を直接探索できます。

```text wrap theme={null}
イージングカーブ、期間、および遅延のスライダーを含むアーティファクトを構築してください。これにより、このトランジションの値を試すことができます。移動するとアニメーションがライブで表示されます。
```

<h3 id="bring-the-result-back-to-your-session">
  結果をセッションに戻す
</h3>

アーティファクトは、Claude に返す決定の軽量エディタとして機能できます。エクスポートコントロールをリクエストして、ターミナルに貼り付けることができるテキストを生成します。これにより、ページとの相互作用の結果がページに留まるのではなく、セッションに流れ込みます。

```text wrap theme={null}
各オープンイシューをドラッグ可能なカードとして Now、Next、Later、Cut 列全体に配置するトリアージボードアーティファクトを作成してください。「Copy as prompt」ボタンを追加して、ここに貼り付けるための最終的な順序を提供してください。
```

<h3 id="track-work-in-progress">
  進行中の作業を追跡する
</h3>

長いタスクが実行される間、Claude にアーティファクトを最新の状態に保つよう依頼します。これにより、リンクを持つ人は誰でもターミナルを読まずにフォローできます。

```text wrap theme={null}
この移行計画をチェックリストアーティファクトに変換してください。完了したアイテムをチェックし、スキップしたものについてはメモを追加してください。
```

<h2 id="improve-the-visual-design">
  ビジュアルデザインを改善する
</h2>

Claude がアーティファクトを構築する際、組み込みのデザインスキルを適用するため、ページは追加のプロンプトなしで意図的なパレット、タイポグラフィ、レイアウトを取得します。そのスキルはまた、独自のものを選択する前に、プロジェクト内の既存のデザインシステムを探します。デザイントークンは、デザインシステムが再利用する名前付きの色、タイポグラフィ、スペーシング値です。アーティファクトを製品のブランディングと一致させるために、Claude が見つけられる場所（プロジェクトの [CLAUDE.md](/docs/ja/memory) またはリポジトリのテーマファイルなど）に記録します。

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude はデザインシステムを独自の選択肢より高い優先度として扱い、プロンプトを両方より高い優先度として扱います。上記の見出しと形式は例です。色、フォント、スペーシングの明確なリストであれば、どのようなものでも機能します。

タイポグラフィについて、Claude は Google Fonts からタイプフェイスを読み込むことができます。これはアーティファクトページが読み込むことができる唯一の外部フォントソースです。Claude は他のタイプフェイスを `@font-face` データ URI としてインライン化し、すべてのタイプフェイスにフォールバックスタックを提供するため、フォントが読み込まれない場合でもページは引き続きレンダリングされます。特定のタイプフェイスを使用するには、プロンプトまたはデザインシステムで名前を付けます。

<h2 id="draft-a-design-canvas">
  デザインキャンバスを作成する
</h2>

UI、画面フロー、ランディングページ、またはポスターをモックアップするために、ページを構築するのではなく、`/design` をブリーフと共に実行します。Claude はデザインを 1 つのキャンバス上のアートボードとして作成し、キャンバスを Design アーティファクトとして公開します。ブリーフは描画する内容を指定します。

```text wrap theme={null}
/design a settings screen for a mobile banking app
```

公開されたアーティファクトをデスクトップブラウザーで開いてアートボードを確認します。アートボード上の要素を選択して変更すると、編集は自動的に保存されます。各アートボードを PNG または PDF としてエクスポートできます。

`/design` は [アーティファクトが利用可能](#availability) なセッションと Claude Code v2.1.265 以降が必要です。

<h2 id="page-constraints">
  ページの制約
</h2>

各アーティファクトは 1 つの自己完結したページです。Claude Code は公開するファイルを HTML ドキュメントシェルでラップし、厳密なコンテンツセキュリティポリシー（CSP）の下で提供します。これはページが実行できることを形作ります。

| 制約         | 効果                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :--------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 外部リクエスト    | ページは Google Fonts からタイプフェイスを読み込むことができ、[5 つのパブリック CDN ホスト](#allowlist-the-viewer-domain)からスクリプトを読み込むことができます：cdnjs、unpkg、Tailwind と jQuery CDN、および jsDelivr 上の `/npm/` などの選択されたパス。CSP はすべての外部画像とその他すべての外部スクリプト、スタイルシート、フォントをブロックし、`fetch`、XHR、WebSocket 呼び出しがページ自身のオリジンと Google Fonts ホストにのみ到達できるようにします。Claude はページが必要とするライブラリをこれらの CDN の 1 つから読み込み、その他すべての CSS と JavaScript をインライン化し、画像をデータ URI として埋め込みます。[コネクタ呼び出し](#pull-live-data-with-mcp-connectors)は claude.ai を通じて行われ、ネットワーク呼び出しを自身で実行します。 |
| バックエンドなし   | アーティファクトは静的ページです。ビューアを自身で認証することはできません。                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ダウンロード     | ページはダウンロードを自身で開始することはできません。ビューアがページが生成するファイルを保存できるようにするには、Claude はダウンロード機能を宣言します。[ファイルダウンロードを提供する](#offer-a-file-download)を参照してください。                                                                                                                                                                                                                                                                                                                                                             |
| シングルページ    | 相対リンクは解決されません。ページと一緒に何もデプロイされていないためです。マルチセクションコンテンツの場合、Claude は個別ファイルではなくページ内アンカーを使用します。                                                                                                                                                                                                                                                                                                                                                                                                         |
| ソースファイルタイプ | 公開されるファイルは `.html`、`.htm`、または `.md` である必要があり、UTF-8 として、またはバイトオーダーマークによってリトルエンディアン UTF-16 としてデコードできる必要があります。Markdown ファイルはスタイル付きドキュメントページとしてレンダリングされ、構文強調表示されたコードが含まれます。デコードできないファイル、または置換文字 `U+FFFD` を含むファイルは、[修正する行と列とともに拒否されます](/docs/ja/errors#the-source-file-is-not-valid-utf-8-text)。                                                                                                                                                                                                           |
| レンダリングサイズ  | レンダリングされたページは 16 MiB 以下である必要があります。大きな埋め込み画像は、公開が失敗する場合の通常の原因です。                                                                                                                                                                                                                                                                                                                                                                                                                                  |

アーティファクトを生成することは、他のレスポンスと同様に出力トークンを使用し、スタイル付きページはターミナルテキストと同じコンテンツよりもトークン集約的です。インライン CSS、インタラクティブコントロール用の JavaScript、特にデータ URI として埋め込まれた画像が主な要因です。アーティファクトのトークンコストを削減するには：

* 埋め込みラスター画像よりも図表に SVG または HTML と CSS を優先する
* 必要のないインタラクティビティを省略する
* ページが大規模なデータセットを完全にインライン化するのではなく要約するようにする

<h2 id="availability">
  利用可能性
</h2>

Artifacts には以下のすべての条件が必要です。いずれかが満たされていない場合、Claude はローカル HTML ファイルを作成するか、公開できないと述べます。

| 要件        | 利用可能な場合                                                                                                                                                                                                                                                                                                                                           |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| プラン       | Pro、Max、Team、または Enterprise。Pro および Max プランでは、Artifacts は共有するまであなたのみがアクセスでき、管理者管理は適用されません。Team プランでは、Artifacts はデフォルトで有効です。Enterprise プランでは、Owner が claude.ai 管理設定で[それらを有効にします](#manage-artifacts-for-your-organization)。                                                                                                                         |
| 認証        | セッションが claude.ai アカウントでサポートされています。CLI またはデスクトップアプリで `/login` でサインインします。Claude Tag セッションはエージェントの ID を通じてサインインするため、追加の手順は不要です。API キー、[ゲートウェイトークン](/docs/ja/llm-gateway)、またはクラウドプロバイダー認証情報を使用するセッションは公開できません。                                                                                                                                             |
| モデルプロバイダー | Anthropic API。[Amazon Bedrock](/docs/ja/amazon-bedrock)、[Google Cloud の Agent Platform](/docs/ja/google-vertex-ai)、または[Microsoft Foundry](/docs/ja/microsoft-foundry)では利用できません。                                                                                                                                                                                  |
| 組織ポリシー    | カスタマー管理暗号化キー（CMEK）、HIPAA、および[Zero Data Retention](/docs/ja/zero-data-retention)は組織に対して有効になっていません。                                                                                                                                                                                                                                                      |
| サーフェス     | Claude Code CLI、または Claude デスクトップアプリバージョン 1.13576.0 以降。[Claude Tag](https://claude.com/docs/claude-tag/overview) セッションは、Claude Tag と Artifacts の両方が組織に対して有効な場合にも Artifacts を公開できます。[Agent SDK](/docs/ja/agent-sdk/overview)、GitHub Action、および MCP サーバーコンテキストではデフォルトで無効です。また、[`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ja/env-vars)が設定されている場合も無効です。 |

Artifacts が組織に対して許可されているかどうかは、Claude Code が `api.anthropic.com` から読み込む組織のポリシーから決まります。Claude Code がポリシーを読み込めない場合、Artifacts は利用できません。リクエストすると、Claude がその理由を説明します。

プロキシ、VPN、またはウェブフィルターが関係している場合は、IT 管理者に `api.anthropic.com` を許可するよう依頼してください。Claude Code はバックグラウンドで再試行を続け、ポリシーが読み込まれて許可されると、Artifacts が利用可能になります。

<h2 id="disable-artifacts">
  アーティファクトを無効にする
</h2>

組織の設定に関わらず、自分のセッションのアーティファクトをオフにするには、以下のいずれかを使用します。

| 場所                        | 実行内容                                                                                                             |
| :------------------------ | :--------------------------------------------------------------------------------------------------------------- |
| [`/config`](/docs/ja/commands) | **Artifacts** 行をオフにします。これにより、ユーザー設定に [`"enableArtifact": false`](/docs/ja/settings-reference#enableartifact) が書き込まれます |
| [設定ファイル](/docs/ja/settings)    | `"enableArtifact": false` を設定します。非推奨の `"disableArtifact": true` もアーティファクトをオフにします                                 |
| [環境変数](/docs/ja/env-vars)      | `CLAUDE_CODE_DISABLE_ARTIFACT=1` を設定します                                                                          |
| [権限ルール](/docs/ja/permissions)  | `permissions.deny` に `Artifact` を追加します                                                                           |

[`--settings`](/docs/ja/cli-reference#cli-flags) ファイルで、または `CLAUDE_CODE_DISABLE_ARTIFACT` でアーティファクトをオフにした場合、あるいは管理者が [管理設定](/docs/ja/server-managed-settings) でアーティファクトをオフにした場合、どの設定ファイルもアーティファクトを再度オンにすることはできません。v2.1.242 より前では、[優先度スタック](/docs/ja/settings#settings-precedence) の上位にあるファイルが、下位のファイルで `"enableArtifact": false` が設定されていても、アーティファクトを再度オンにすることができました。

プロジェクトの `.claude/settings.json` または `.claude/settings.local.json` で `"enableArtifact": false` を設定して、そのプロジェクト内のセッションのアーティファクトをオフにすることもできます。どちらのファイルでも `"enableArtifact": true` はアーティファクトを再度オンにしません。プロジェクトおよびローカル設定でこのキーを尊重するには、Claude Code v2.1.242 以降が必要です。

`domain:` 部分のない `WebFetch` 拒否またはアスクルールを追加した場合、アーティファクトをオフにしたり、アーティファクト読み取りをブロックしたりしません。[`deny` または `ask` の `WebFetch(domain:claude.ai)` ルールはアーティファクト読み取りに適用されます](/docs/ja/permissions#allow-or-deny-every-fetch)。

<h2 id="manage-artifacts-for-your-organization">
  組織のアーティファクトを管理する
</h2>

Team プランと Enterprise プランのオーナーは、[claude.ai 管理設定](https://claude.ai/admin-settings/claude-code)からアーティファクトを管理できます。アーティファクトのコンテンツは Anthropic が運用するインフラストラクチャに保存され、アーティファクトが[公開で共有](#control-public-sharing)されていない限り、発行元の組織の認証済みメンバーのみに表示されます。

<h3 id="enable-or-disable-artifacts">
  アーティファクトを有効または無効にする
</h3>

組織全体のアーティファクトを有効または無効にするには、[**Settings > Claude Code > Capabilities**](https://claude.ai/admin-settings/claude-code)に移動して、**Artifacts** トグルを使用します。ロールベースのアクセス制御を備えた Enterprise プランでは、アーティファクトを特定のロールにスコープすることもできます。[**Settings > Roles**](https://claude.ai/admin-settings/roles)に移動してロールを編集し、**Claude Code** グループの下の **Artifacts** 権限を設定します。

<h3 id="control-connector-calls-from-artifacts">
  アーティファクトからのコネクタ呼び出しを制御する
</h3>

[アーティファクトからのコネクタ呼び出し](#pull-live-data-with-mcp-connectors)には、アーティファクトのオン/オフを切り替える **Artifacts** トグルとは別の専用トグルがあります。[**Settings > Capabilities**](https://claude.ai/admin-settings/capabilities)に移動して、**Enable artifact connectors** トグルを使用します。同じトグルは claude.ai の会話で作成されたアーティファクトからのコネクタ呼び出しも管理します。そのため、**Settings > Claude Code** ではなく **Settings > Capabilities** の下に配置されています。

<h3 id="control-public-sharing">
  公開共有を制御する
</h3>

公開共有は Team プランと Enterprise プランではデフォルトでオフになっているため、メンバーはオーナーがオンにするまで、組織内でのみアーティファクトを共有できます。メンバーがサインインなしで誰でも表示できるパブリックリンクにアーティファクトを公開できるようにするには、**Settings > Claude Code > Capabilities** に移動して、**Artifacts** トグルの下の **External sharing** をオンにします。オフに戻すと、各アーティファクトのオーディエンスを変更することなく、既存のパブリックリンク経由のアクセスがブロックされます。再度有効にすると、アクセスが再開されます。

<h3 id="set-a-retention-policy">
  保持ポリシーを設定する
</h3>

アーティファクトが自動削除される前に保持される期間を設定するには、[**Settings > Data & privacy controls**](https://claude.ai/admin-settings/data-privacy-controls)に移動します。作成者にのみプライベートなアーティファクトと共有されたアーティファクトに対して、別々の保持期間を設定できます。

<h3 id="review-the-audit-log">
  監査ログを確認する
</h3>

アーティファクトの公開、共有、削除は、それぞれ組織の監査ログに `claude_artifact_*` イベントタイプの下に表示されます。これは claude.ai の会話で作成されたアーティファクトに使用されるのと同じファミリーです。

<h3 id="allowlist-the-viewer-domain">
  ビューアドメインをホワイトリストに登録する
</h3>

claude.ai のビューアは、サンドボックス化された `*.claudeusercontent.com` オリジンから各アーティファクトを読み込みます。組織が送信ネットワークアクセスを制限している場合は、`claude.ai` と共にそのドメインをホワイトリストに追加します。完全なリストについては、[ネットワークアクセス要件](/docs/ja/network-config#network-access-requirements)を参照してください。

[Google Fonts](#improve-the-visual-design)からタイプフェイスを読み込むアーティファクトは、`fonts.googleapis.com` と `fonts.gstatic.com` もリクエストします。どちらのホストもオプションです。ブロックすると、アーティファクトはフォールバックタイプフェイスでレンダリングされます。フォントリクエストが即座に失敗するように、サイレントドロップではなく高速拒否でブロックして、ページの最初のレンダリングが遅延しないようにします。

アーティファクトは、React やチャートパッケージなどの JavaScript ライブラリを `cdnjs.cloudflare.com`、`cdn.jsdelivr.net`、`cdn.tailwindcss.com`、`code.jquery.com`、および `unpkg.com` から読み込むことができ、他の外部ホストからは読み込めません。これらのホストをブロックすると、ライブラリに依存するアーティファクトの部分が機能しません。ブロックされたフォントとは異なり、ブロックされたライブラリにはフォールバックがありません。ブロックされたライブラリリクエストが即座に失敗するように、ここでも高速拒否でブロックして、タイムアウトするまでハングしないようにします。

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  Compliance API でアーティファクトをリストおよび削除する
</h3>

[Compliance API](https://docs.claude.com/en/api/compliance)は、組織のアーティファクトをリストアップし、特定のバージョンのコンテンツを取得し、アーティファクトを削除するエンドポイントを提供します。

| Method   | Endpoint                                                            |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

リクエストとレスポンススキーマについては、[Compliance API リファレンス](https://docs.claude.com/en/api/compliance/code/artifacts)を参照してください。

<h2 id="related-resources">
  関連リソース
</h2>

* アーティファクトと組み合わせる[プロンプティングパターンとワークフロー](/docs/ja/prompt-library)を参照する
* 再利用するアーティファクトプロンプトを[スキル](/docs/ja/skills)に変換して、コマンドとして呼び出せるようにする
* [MCP サーバーを接続](/docs/ja/mcp)して、Claude がアーティファクトにライブデータを取得できるようにする
