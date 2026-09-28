> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# モバイルの Claude Code

> Claude アプリ for iOS と Android を使用して、携帯電話から Claude Code タスクを開始、監視、操作します。

Claude アプリ for [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) と [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) は、コードが実行される場所ではなく、Claude Code セッションのクライアントです。携帯電話からクラウド上の [クラウドセッション](#start-and-monitor-cloud-sessions) と [プロジェクト](/docs/ja/claude-projects)、[リモートコントロール](#continue-a-local-session-with-remote-control) を通じて自分のマシンで実行されているセッション、または [Dispatch](/docs/ja/desktop#sessions-from-dispatch) を通じて Desktop アプリにアクセスできます。

<Note>
  Claude Code には別のモバイルアプリはありません。クラウドセッションとリモートコントロールは両方とも Claude アプリの **Code** タブに存在し、Dispatch はアプリでメッセージを送って依頼するタスクです。
</Note>

<h2 id="get-the-app">
  アプリを入手する
</h2>

<Steps>
  <Step title="Claude アプリをダウンロード">
    Claude アプリを [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) または [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) にインストールします。iPad では同じ iOS アプリをインストールしてください。

    <Tip>
      Claude Code セッションで `/mobile` を実行すると、[claude.ai/mobile](https://claude.ai/mobile) のスキャン可能な QR コードが表示されます。これはお客様の電話に適切なアプリストアを開きます。`/ios` と `/android` も同じ機能です。
    </Tip>
  </Step>

  <Step title="サインイン">
    Claude Code に使用するのと同じ claude.ai アカウントと組織でサインインします。クラウドセッションとリモートコントロールには claude.ai アカウントが必要なため、Anthropic Console API キーまたは Amazon Bedrock などのサードパーティプロバイダーからはアクセスできません。
  </Step>

  <Step title="Code タブを開く">
    アプリのナビゲーションで **Code** をタップしてセッションにアクセスするか、スマートフォンで [claude.ai/code/new](https://claude.ai/code/new) を開いてアプリで新しい Code セッションを開始します。Code タブが表示されない場合、お客様のプランまたは組織にこれらの機能が含まれていない可能性があります。[サブスクリプションプランごとの利用可能性](/docs/ja/feature-availability#availability-by-subscription-plan)を参照してください。
  </Step>
</Steps>

<h2 id="work-from-your-phone">
  スマートフォンから作業する
</h2>

アプリからクラウドセッションを開始したり、プロジェクトを開いたり、コンピュータで実行されている Claude Code セッションを操作したり、Dispatch にタスクをメッセージで送ったりできます。アプリはすべてで同じですが、作業が行われる場所が異なります。

| 機能                                             | 接続先                                  | 使用時期                                                                                                        |
| :--------------------------------------------- | :----------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| [クラウドセッション](/docs/ja/claude-code-on-the-web)        | Anthropic が管理するクラウドインフラストラクチャ上のセッション | リポジトリが GitHub 上にあり、スマートフォンを置いた後もタスクが実行され続ける必要がある場合。セットアップについては [クラウドクイックスタート](/docs/ja/web-quickstart)を参照してください。 |
| [プロジェクト](/docs/ja/claude-projects)                  | Claude が並列クラウドセッションをスレッドとして調整する会話    | 1 つのタスクではなく関連する作業のストリームがあり、どのスレッドが完了したか、または自分の対応が必要かを確認したい場合。                                               |
| [リモートコントロール](/docs/ja/remote-control)               | コンピュータで実行されている Claude Code セッション     | 作業にローカルファイルシステム、ツール、または MCP サーバーが必要な場合。                                                                     |
| [Dispatch](/docs/ja/desktop#sessions-from-dispatch) | コンピュータの Desktop アプリ                  | タスクをメッセージで送信し、Dispatch に実行方法を決定させたい場合。Pro または Max プランが必要です。                                                 |

コンピュータがオフになる場合は、クラウドセッションまたはプロジェクトを使用してください。これらはクラウドで実行され、ラップトップを閉じた後も続行されます。リモートコントロールと Dispatch は自分のマシンを操作するため、Claude Code または Desktop アプリが実行されている状態を保つ必要があります。リモートコントロールセッション中にマシンがスリープ状態になった場合、Claude Code はマシンがオンラインに戻ったときに再接続されます。

より詳細な比較については、[ターミナルから離れているときに作業する](/docs/ja/platforms#work-when-you-are-away-from-your-terminal)を参照してください。

クラウドセッションとリモートコントロールは **Code** タブから実行されます。アプリでタスクとしてメッセージを送る Dispatch については、[Dispatch からのセッション](/docs/ja/desktop#sessions-from-dispatch)を参照してください。

<h3 id="start-and-monitor-cloud-sessions">
  クラウドセッションを開始および監視する
</h3>

クラウドセッションはクラウドインフラストラクチャでタスクを実行し、Anthropic が管理するため、スマートフォンを置いた後もセッションが続行されます。Code タブからリポジトリとブランチを選択し、タスクを説明して送信します。セッションはデバイス間で永続化されます。ラップトップで開始したタスクはスマートフォンから確認できる状態で待機し、スマートフォンから開始したタスクはデスクに戻ったときに待機しています。

アプリでセッションを開いて進捗を確認したり、Claude の質問に答えたり、新しい方向に操作したりできます。Claude に [プルリクエストを監視](/docs/ja/claude-code-on-the-web#auto-fix-pull-requests)させて、CI の失敗やレビューコメントが到着したときに修正することもできます。GitHub を接続して環境をセットアップするには、[クラウドクイックスタート](/docs/ja/web-quickstart)に従い、クラウドセッションで実行できるすべてのことについては [クラウドで Claude Code を使用する](/docs/ja/claude-code-on-the-web)を参照してください。

<h3 id="continue-a-local-session-with-remote-control">
  リモートコントロールでローカルセッションを続行する
</h3>

リモートコントロールは Claude アプリをマシンで実行されている Claude Code セッションに接続するため、コード実行とファイルシステムアクセスはローカルのままで、スマートフォンからセッションを操作できます。コンピュータで `claude remote-control` を使用してセッションを開始するか、既に開いているセッションで `/remote-control` を実行します。その後、ターミナルが表示できる QR コードをスキャンするか、Claude アプリを開いて **Code** をタップし、リストからセッションを選択します。各オプションについては、[別のデバイスから接続](/docs/ja/remote-control#connect-from-another-device)を参照してください。

Claude アプリで添付ファイルを追加すると、ローカルセッションにも到達します。

* **写真**: Claude は添付された写真をメッセージの一部として直接見ることができます。Claude Code は各写真を `~/.claude/uploads/` の下に保存し、Claude に保存されたファイルパスを伝えるため、Claude はその画像をそれが作成するファイルにコピーできます。
* **その他のファイル**: Claude Code はそれらをマシンにダウンロードし、`@` ファイル参照として Claude に渡します。

要件、呼び出しモード、トラブルシューティングについては、[リモートコントロール概要](/docs/ja/remote-control)を参照してください。

<h3 id="get-push-notifications">
  プッシュ通知を取得する
</h3>

リモートコントロールがアクティブな場合、Claude はスマートフォンにプッシュ通知を送信できます。通常は、長時間実行されるタスクが完了したときまたは決定が必要なときです。プロンプトで 1 つをリクエストすることもできます。例えば、`notify me when the tests finish` のようにです。2 つの `/config` トグルと配信トラブルシューティングについては、[モバイルプッシュ通知](/docs/ja/remote-control#mobile-push-notifications)を参照してください。

Dispatch は、生成した Code セッションが完了したときまたは承認が必要なときに独自の通知を送信します。これについては [Dispatch からのセッション](/docs/ja/desktop#sessions-from-dispatch)で説明されています。

<h2 id="limitations">
  制限事項
</h2>

モバイルクライアントはセッションが必要とするほとんどのことをカバーしていますが、いくつかの制限があります。

* **ローカルのみのコマンド**: `/plugin` や `/resume` など、ターミナルインターフェイスでのみ実行されるコマンドはアプリから機能しません。[リモートコントロール制限](/docs/ja/remote-control#limitations)には、モバイルから機能するコマンドと動作の違いが記載されています。
* **権限モード**: クラウドセッションはモードドロップダウンで Accept edits、Plan、Auto を提供し、リモートコントロールセッションは Manual、Accept edits、Plan を提供します。どちらの場合でもアプリから Bypass permissions を選択することはできず、リモートコントロールセッションの Auto を選択することもできません。[権限モードを切り替える](/docs/ja/permission-modes#switch-permission-modes)を参照してください。
* **Dispatch プラン**: Dispatch には Pro または Max プランが必要であり、Team または Enterprise では利用できません。

<h2 id="related-resources">
  関連リソース
</h2>

* [プラットフォームと統合](/docs/ja/platforms): Claude Code が実行されるすべてのサーフェスを比較します
* [Web の Claude Code](/docs/ja/claude-code-on-the-web): クラウドセッションの実行方法とターミナルとの間で作業を移動する方法
* [クラウド環境を構成する](/docs/ja/cloud-environments): クラウドセッションのネットワークアクセスレベル、環境変数、セットアップスクリプト
* [リモートコントロール](/docs/ja/remote-control): 任意のデバイスからローカルセッションを続行します
* [Dispatch からのセッション](/docs/ja/desktop#sessions-from-dispatch): Dispatch タスクが Desktop アプリで Code セッションになる方法
* [Channels](/docs/ja/channels): 作業がマシンで実行されている間に、Telegram、Discord、または iMessage を通じてスマートフォンから Claude に何かを尋ねます
* [Slack の Claude Code](/docs/ja/slack): `@Claude` をメンションして Slack ワークスペースからコーディングタスクを委任します
