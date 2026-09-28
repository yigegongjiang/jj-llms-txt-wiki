> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# iOS シミュレータでアプリをテストする

> Claude Code Desktop は、Claude がアプリをビルド、実行、またはチェックするときに、iOS シミュレータペインでアプリを開きます。各セッションに対して個別のシミュレータが用意されます。

<Note>
  iOS シミュレータペインは、macOS 上の Claude Code Desktop でパブリックベータ版です。Pro、Max、Team、Enterprise プランで利用可能です。ただし、HIPAA 設定が有効になっている Enterprise 組織では利用できません。
</Note>

iOS シミュレータペインは、Claude Code Desktop の会話の横に Apple の iOS シミュレータで実行されているアプリを表示します。Claude がシミュレータでアプリをビルド、インストール、起動、またはチェックするときに、ペインが自動的に開き、デバイス画面がライブでストリーミングされます。Claude がアプリを実行してテストするのを見守ったり、Claude が作業を続けている間に自分でアプリをタップして操作したりできます。

シミュレータペインはシミュレータを直接操作するため、[コンピュータの使用](/docs/ja/desktop#let-claude-use-your-computer)を必要とせず、画面を乗っ取ったり他のウィンドウを隠したりすることはありません。CLI からは、Claude は [コンピュータの使用](/docs/ja/computer-use#test-a-simulator-flow)を通じて iOS シミュレータに到達し、マウスで操作するのと同じ方法で画面上のシミュレータを制御します。

<h2 id="requirements">
  要件
</h2>

シミュレータペインは Apple のシミュレータツールを使用しており、デスクトップアプリには含まれていません。セッションを開始する前に、以下を確認してください。

* Claude Desktop v1.24012.0 以降
* Mac（Apple の iOS シミュレータは macOS でのみ実行されるため）
* [Xcode](https://developer.apple.com/xcode/)（iOS プラットフォームがインストールされている状態）。これはシミュレータデバイスを提供します。Xcode にまだシミュレータが表示されていない場合は、[シミュレータペインにシミュレータが見つからないと表示される](#the-simulator-pane-says-no-simulators-were-found)を参照してください。
  * Xcode 26.x を使用してください。ペインはまだ Xcode 27 では動作しません。Xcode 27 は Simulator アプリを Device Hub に置き換えます。Mac 上の `xcode-select` が Xcode 27 を指している場合は、[シミュレータペインが Xcode 27 で失敗する](#the-simulator-pane-fails-with-xcode-27)を参照してください。

<Note>
  このページでは、「デバイス」はシミュレートされた iPhone または iPad を指し、Xcode の **Window → Devices and Simulators** で管理するのと同じシミュレータデバイスの 1 つであり、物理ハードウェアではありません。
</Note>

シミュレータペインはローカルセッションでのみ利用可能です。[クラウド](/docs/ja/desktop#run-long-running-tasks-in-the-cloud)および [SSH](/docs/ja/desktop#ssh-sessions) セッションでは、Claude は Mac 上のシミュレータに到達できないマシン上で実行されます。

<h2 id="run-your-app-in-the-simulator">
  シミュレータでアプリを実行する
</h2>

シミュレータペインを開くためにコマンドや設定は必要ありません。Claude がシミュレータでアプリを実行するときに、ペインが開きます。

<Steps>
  <Step title="iOS プロジェクトを開く">
    Claude Code Desktop で **Code** タブを開き、アプリのプロジェクトを [プロジェクトフォルダ](/docs/ja/desktop#start-a-session)としてセッションを開始します。iOS シミュレータ用にアプリをビルドするプロジェクトであれば、どのプロジェクトでも動作します。
  </Step>

  <Step title="Claude にアプリを実行またはテストするよう依頼する">
    タスクをアプリの実行または検証の周辺で表現します。例えば：

    ```text theme={null}
    Build the app and run it in the simulator to check the onboarding flow.
    ```
  </Step>

  <Step title="シミュレータペインでアプリを見る">
    アプリがシミュレータで起動すると、iOS シミュレータペインが会話の横に開きます。Claude がデバイスを初めて使用するときは、デスクトップアプリがアクセスを許可するよう求めます。[Claude にデバイスへのアクセスを許可する](#grant-claude-access-to-a-device)を参照してください。Claude はアプリをインストールし、タップして操作し、画面を読み取って独自の変更を検証しながら、あなたが見守ります。
  </Step>
</Steps>

シミュレータペインは、Claude がセッション内のいずれかの時点でシミュレータでアプリを起動するたびに開きます。リクエストがアプリを見ることについてである場合、例えば「新しい画面は正しく見えますか？」という場合、Claude は作業を開始する前にシミュレータを起動します。Claude がバグを修正または画面を変更した後、変更を検証するよう依頼します。アプリを再起動すると、ペインが開いていない場合は再度開きます。

シミュレータペインは、アプリが実際に起動したデバイスを表示します。特定のデバイスでテストするには、リクエストでそれを指定します。例えば「iPhone SE シミュレータで実行してください」と言えば、Claude はビルドと起動時にそのデバイスをターゲットにします。

Claude が起動したデバイスは Apple の Simulator アプリにも表示され、Claude は既に起動しているデバイスにアプリをインストールできます。

シミュレータペインを自分で開くこともできます。セッションがシミュレータを接続したか Swift ファイルを編集した後、セッションツールバーの **Views** メニューに **iOS Simulator** エントリが表示されます。ペインがまだデバイスを表示していない場合は、**Attach simulator** をクリックするか、その横のデバイスメニューから特定のデバイスを選択します。シャットダウンしたデバイスを選択すると、それが起動します。Xcode またはそのシミュレータが見つからない場合、ペインはセットアップステップを表示し、完了するたびにそれらをチェックします。

<h2 id="control-the-simulator-yourself">
  シミュレータを自分で制御する
</h2>

シミュレータペインはビューアだけではなく、インタラクティブです。Claude が作業している間、またはタスク間に、以下を実行できます。

* デバイス画面をクリックしてドラッグすることでタップとスワイプを実行する
* Apple の Simulator アプリと同じショートカットでハードウェアボタンを押す。**Cmd+Shift+H** でホーム、**Cmd+L** でロック、**Cmd+Up Arrow** と **Cmd+Down Arrow** でボリューム
* 回転ボタンまたは **Cmd+Right Arrow** でデバイスを時計回りに 4 分の 1 回転させる
* デバイスメニューからペインが表示するデバイスを切り替える。このメニューには各シミュレータの OS バージョンと起動状態が表示されます
* **Cmd+S** でスクリーンショットを保存するか、**Cmd+R** でスクリーン録画を保存します。ペインのキャプチャボタンまたはショートカットを使用します。ファイルはデスクトップに保存されます
* **Detach simulator** をクリックしてデバイスをシャットダウンせずにストリーミングを停止します。ペインは **Attach simulator** 状態に戻ります

デバイス名の下の行は、シミュレータからのビデオストリームを調整します。Mac に負荷がかかっている場合は **Frame rate** または **Resolution** を下げるか、**Encoding** を H.264 と JPEG の間で切り替えるか、**FPS** をチェックしてペインが受け取っているフレームレートを表示します。これらの設定は、ペインがデバイスを表示する方法を変更し、アプリの実行方法は変更しません。

あなたと Claude は同じデバイスを操作するため、あなたのタップは Claude が見るアプリの状態を変更します。Claude に特定の画面をチェックさせるには、タップして移動してから依頼します。Claude がデバイスを操作している間、ペインは画面の上に **Claude is using this device** バッジを表示します。バッジが消えるまでタップを控えて、結果があなたの入力ではなくアプリを反映するようにします。

<h2 id="how-sessions-manage-devices">
  セッションがデバイスを管理する方法
</h2>

各デバイスはそれを起動したセッションに属するため、[並列セッション](/docs/ja/desktop#work-in-parallel-with-sessions)はデバイスを共有しません。1 つのセッションのペインに表示されるのは、そのセッションの作業を反映し、別のセッションの作業ではありません。サイドバーでセッションを切り替えると、シミュレータビューが会話と一緒に切り替わり、戻すと同じデバイスが中断したところから再開されます。Claude が複数のデバイスで作業する場合、各デバイスは独自のペインを開き、セッションあたり最大 4 つまでです。

Claude Code Desktop は、起動したシミュレータが使用されなくなると、それらをシャットダウンします。アプリを終了するとき、セッションをアーカイブするとき、またはペインからデバイスをデタッチしてから 10 分後です。ペインまたは Apple の Simulator アプリから自分で起動したデバイスは、自動的にシャットダウンされることはありません。接続されたデバイスをすぐにシャットダウンするには、ペインのシャットダウンボタンを使用します。

<h2 id="grant-claude-access-to-a-device">
  Claude にデバイスへのアクセスを許可する
</h2>

Claude はデバイスを制御する前に同意を求めますが、アプリのビルドまたはデバイス上の URL を開くことはセッションの権限モードに従います。あなたまたはあなたの組織は、Claude のアクセスを完全にオフにすることもできます。

<h3 id="allow-a-device-the-first-time">
  デバイスを初めて許可する
</h3>

Claude がシミュレータを初めて使用するときは、デスクトップアプリがそれを許可するよう求めます。同意はそのデバイスの制御とスクリーンショットの撮影をカバーし、セッションごとではなくデバイスごとに 1 回与えます。Claude のデバイスのスクリーンショットは Anthropic に送信され、通常の会話保持設定の下で保持されるため、Claude が使用するデバイスで実際のアカウントにサインインしないでください。

デバイスを許可した後、タップ、入力、アプリの起動、スクリーンショットの撮影など、Claude のそのデバイス上のアクションは、さらなるプロンプトなしに実行されます。これらはペインをクリックするのと同じ信頼を持ち、シミュレートされたデバイスのみに触れるため、ペインはコンピュータの使用が必要とする macOS アクセシビリティおよびスクリーン録画権限を必要としません。

拒否した場合、デバイスは起動し、ペインは独自のタップで動作します。Claude のアクセスのみがオフのままです。後で考え直した場合は、ペインの **Let Claude use it** をクリックします。

<h3 id="actions-that-follow-your-permission-mode">
  権限モードに従うアクション
</h3>

2 つのアクションは、1 回限りの同意ではなく、セッションの [権限モード](/docs/ja/permissions#permission-modes)に従います。

* デバイス上で URL を開く。例えば、ディープリンクをテストするか、デバイスの Safari でページを読み込むため。URL はデータをデバイスから持ち出す可能性があります。
* アプリをビルドする。`xcodebuild` は Mac 上でプロジェクトのビルドスクリプトを実行するため。既に進行中のビルドをチェックしてもプロンプトは表示されません。

<h3 id="turn-off-simulator-access">
  シミュレータアクセスをオフにする
</h3>

デスクトップアプリの設定でシミュレータアクセスをオフにできます。組織がすべてのユーザーに対してオフにする方法は 2 つあります。

* `disableMobileSimulatorTools` [管理設定](/docs/ja/desktop#managed-settings)は Claude のシミュレータツールをブロックします。シミュレータペインは独自のタップで使用可能なままで、設定はアプリ内からオーバーライドできません。
* `requireCoworkFullVmSandbox` ポリシーキー。これは Claude のツールを Mac 上ではなく分離された仮想マシン内で実行し、シミュレータペインと Claude のシミュレータツールを完全に無効にするため、設定されている間はペインがデバイスを接続できません。

Claude はどちらが適用されるかを通知します。

<h2 id="limitations">
  制限事項
</h2>

Claude はシミュレートされたデバイスのみを操作でき、物理的な iPhone または iPad を制御できません。物理デバイスでテストするには、Xcode から自分でアプリを実行し、表示内容を説明するか、スクリーンショットを会話に添付して Claude が作業できるようにします。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="the-simulator-pane-doesn’t-open-when-claude-runs-the-app">
  Claude がアプリを実行するときにシミュレータペインが開かない
</h3>

Claude がアプリを実行またはテストしたいことを認識していないか、シミュレータツールが見つからない可能性があります。以下を確認してください。

* 目標を明確に述べます。例えば「iOS シミュレータでアプリを実行し、サインアップフローをタップして操作してください」。
* Xcode と iOS シミュレータがインストールされており、Xcode バージョンが [要件](#requirements)を満たしていることを確認します。
* 組織が Claude Code を管理している場合、[シミュレータツールはポリシーによって無効にされている可能性があります](#turn-off-simulator-access)。
* Enterprise 組織で HIPAA 設定が有効になっている場合、シミュレータペインは利用できません。
* シミュレータペインには Claude Desktop v1.24012.0 以降が必要です。**Claude → Check for Updates** を開き、アプリを再起動します。

<h3 id="the-simulator-pane-says-no-simulators-were-found">
  シミュレータペインにシミュレータが見つからないと表示される
</h3>

`xcode-select` が Xcode 27 を指している場合、デバイスが存在していても、ペインはシミュレータが見つからないと報告できます。[シミュレータペインが Xcode 27 で失敗する](#the-simulator-pane-fails-with-xcode-27)を参照してください。それ以外の場合、Xcode はインストールされていますが、iOS シミュレータがリストされていません。シミュレータペインはセットアップステップを表示し、各ステップが完了するたびにそれらをチェックします。見つからないピースを手動でインストールするには、Xcode の設定から iOS シミュレータランタイムをダウンロードするか、`xcodebuild -downloadPlatform iOS` を実行します。

<h3 id="the-simulator-pane-fails-with-xcode-27">
  シミュレータペインが Xcode 27 で失敗する
</h3>

ペインはまだ Xcode 27 では動作しません。Xcode 27 は Simulator アプリを Device Hub に置き換えます。Xcode 27 が選択されている場合、デバイスの接続に失敗するか、デバイスが存在していてもペインはシミュレータが見つからないと報告します。

ペインは `xcode-select` が指す Xcode を使用します。Xcode 27 が唯一のインストールである場合、まず Xcode 26.x をそれと並行してインストールします。次に、26.x インストールをそのパスで選択します。例えば、`/Applications/Xcode-26.4.app` としてインストールされている場合：

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

`xcode-select -p` を実行して、どのインストールが選択されているかを確認します。

<h2 id="see-also">
  関連項目
</h2>

* [Desktop でのコンピュータの使用](/docs/ja/desktop#let-claude-use-your-computer)：専用ペインのないアプリの画面制御
* [CLI からのコンピュータの使用](/docs/ja/computer-use)：CLI が iOS シミュレータに到達する方法
* [セッションで並列に作業する](/docs/ja/desktop#work-in-parallel-with-sessions)：セッションが変更を分離する方法
* [Claude Code Desktop を始める](/docs/ja/desktop-quickstart)
