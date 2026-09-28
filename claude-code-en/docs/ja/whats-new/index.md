> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 新機能

> Claude Code の注目すべき機能を毎週紹介するダイジェスト。コードスニペット、デモ、およびそれらが重要である理由についての説明が含まれています。

週間開発ダイジェストは、あなたの仕事のやり方を変える可能性が最も高い機能をハイライトします。各エントリには実行可能なコード、短いデモ、および完全なドキュメントへのリンクが含まれています。すべてのバグ修正と軽微な改善については、[changelog](/docs/en/changelog) を参照してください。

<Update label="Week 37" description="September 7–11, 2026" tags={["v2.1.263–v2.1.269"]}>
  **`claude plugin eval`**：プラグインをテストケースのスイートに対して実行し、結果をスコアリングし、プラグインなしのベースラインと比較します。`claude plugin eval init` はケースとグレーダーをあなたのために作成します。

  今週のその他の機能：**Claude Code Desktop ペイン** をポップアウトして独自のウィンドウに表示し、後で戻してドックできます。**`maxEffortLevel`** 設定はすべてのプロバイダーのエフォートレベルを制限します。**WebFetch** が 5 分以内にダウンロードを完了しなかったページは、ハングする代わりに失敗します。

  [Week 37 ダイジェストを読む →](/docs/ja/whats-new/2026-w37)
</Update>

<Update label="Week 36" description="August 31 – September 4, 2026" tags={["v2.1.251–v2.1.261"]}>
  **Claude Fable 5.1**：1M トークンコンテキストウィンドウで Claude Code で利用可能です。

  今週のその他の機能：Pro および Max プランでは、**Desktop アプリでのコンピュータ使用** は macOS でバックグラウンドで動作し、あなたは作業を続けます。フルスクリーンレンダリングでは、**`/diff`** は会話の横にライブパネルを開き、Claude が編集するにつれて更新されます。**`/skill-doctor`** は各スキルがコンテキストでどのくらいのコストがかかるか、およびどのくらいの頻度で使用されるかを表示します。

  [Week 36 ダイジェストを読む →](/docs/ja/whats-new/2026-w36)
</Update>

<Update label="Week 35" description="August 24–28, 2026" tags={["v2.1.240–v2.1.250"]}>
  **Desktop アプリでターミナルセッションを再開**：Claude Code Desktop プロンプトボックスで `/resume` を入力して、CLI から開始したセッションを選択し、完全な会話とコンテキストをそのまま再開します。

  今週のその他の機能：**Claude が作成したフィードバック** は、セッションで何か問題が発生したときに Claude がフィードバックレポートを作成し、あなたが確認して `/feedback` から送信します。**`--restricted`** はコマンド実行ツールやあなたのユーザーおよびプロジェクト設定なしでセッションを開始し、共有マシン上の評価ハーネス向けです。**`modelPicker`** 設定は `/model` ピッカーがリストするモデルを制御します。

  [Week 35 ダイジェストを読む →](/docs/ja/whats-new/2026-w35)
</Update>

<Update label="Week 34" description="August 17–21, 2026" tags={["v2.1.234–v2.1.239"]}>
  **`/design`**：Claude Design のアートボードワークフローを CLI と Claude Code Desktop に導入するリサーチプレビュー。アーティファクト上に構築されているため、Claude はあなたの UI 用に編集可能なアートボードを作成し、選択したものを実装します。

  今週のその他の機能：ビルトイン **Concise output style** により Claude は結果を最初に示し、前置きをスキップします。`claude remote-control` を実行しているマシンは、電話の Code タブから **device card** として表示されるため、そこからセッションを開始できます。**`ANTHROPIC_DEFAULT_MODEL`** は新しいセッションが開始するモデルを設定します。

  [Week 34 ダイジェストを読む →](/docs/ja/whats-new/2026-w34)
</Update>

<Update label="Week 33" description="August 10–14, 2026" tags={["v2.1.225–v2.1.233"]}>
  **Desktop でのセッション制限後の自動継続**：Claude Code Desktop でセッション制限に達した場合、制限カードで **Auto-continue when limits reset** をチェックすると、制限がリセットされたら、アプリは中断されたターンを再試行します。

  今週のその他の機能：**fork mode** はインタラクティブセッションでデフォルトで有効になっているため、Claude は完全な会話を継承するサブエージェントに副タスクを渡すことができます。**GitLab** マージリクエスト URL は `--worktree` と `claude agents` ビューで動作し、マーケットプレイスは bare `gitlab.com` URL をクローンします。プロンプトで **`@`** を入力すると、別の Claude セッションを名前で言及できます。

  [Week 33 ダイジェストを読む →](/docs/ja/whats-new/2026-w33)
</Update>

<Update label="Week 32" description="August 3–7, 2026" tags={["v2.1.220–v2.1.224"]}>
  **クロスセッションメッセージング**：macOS と Linux では、Claude Code セッションが相互にメッセージを送信できるようになったため、Claude は 1 つのセッションから別のセッションに検出結果または決定を渡すことができ、再度説明する必要がありません。

  今週のその他の機能：**self-hosted environments** は Claude Code クラウドセッションをあなたの組織が運用するインフラストラクチャで実行し、Team および Enterprise プランでパブリックベータ版です。**auto mode** は 8 月 14 日から Pro、Max、および Team プランの新しいセッションのデフォルト権限モードになります。**VS Code extension** は Focus view を取得します。

  [Week 32 ダイジェストを読む →](/docs/ja/whats-new/2026-w32)
</Update>

<Update label="Week 30" description="July 20–24, 2026" tags={["v2.1.214–v2.1.219"]}>
  **Claude Opus 5**：Claude Code の新しいデフォルト Opus モデル。1M トークンコンテキストウィンドウと fast mode で 1 MTok あたり $10/$50 です。

  今週のその他の機能：**Claude Code Desktop** は iOS Simulator ペインをパブリックベータで開くため、Claude はあなたのアプリを実行してタップして操作でき、あなたはそれを見ることができます。**Claude Security plugin** はあなたのコードベースのマルチエージェント脆弱性スキャンを実行し、選択した検出結果をあなたが自分で適用するパッチに変換します。**`/code-review`** はバックグラウンドサブエージェントとして実行されます。

  [Week 30 ダイジェストを読む →](/docs/ja/whats-new/2026-w30)
</Update>

<Update label="Week 29" description="July 13–17, 2026" tags={["v2.1.207–v2.1.212"]}>
  **Artifacts は MCP コネクタを呼び出します**：公開されたアーティファクトは、ページを開くときに各ビューアー自身の MCP コネクタを通じてライブデータを取得し、アクションを実行できます。今週はパブリック共有リンク、Team および Enterprise のエディタロール、および Claude Tag セッションから作成されたアーティファクトも追加されます。

  今週のその他の機能：**screen reader mode** は視覚的なターミナルインターフェイスを VoiceOver や NVDA などのスクリーンリーダー用のプレーンな線形テキストに置き換えます。**`/fork`** は会話を新しいバックグラウンドセッションにコピーしながら、あなたは作業を続けます。**auto mode** は Amazon Bedrock、Google Cloud の Agent Platform、および Microsoft Foundry でオプトイン変数が不要になりました。

  [Week 29 ダイジェストを読む →](/docs/ja/whats-new/2026-w29)
</Update>

<Update label="Week 28" description="July 6–10, 2026" tags={["v2.1.202–v2.1.206"]}>
  **デスクトップ上のアプリ内ブラウザ**：Claude Code デスクトップはビルトインブラウザを取得し、Claude はドキュメント、デザイン、またはその他のサイトを表示して、ローカル開発サーバープレビューと同じ方法でページと対話できます。

  今週のその他の機能：**`/doctor`** は完全なセットアップチェックアップで、問題を診断して修正でき、`/checkup` がそのエイリアスです。**auto mode** はトランスクリプト改ざんをブロックし、未解決の変数で `rm -rf` の前に確認を求めます。**agent view rows** は色付きの状態単語と分類器が作成したヘッドラインを表示します。

  [Week 28 ダイジェストを読む →](/docs/ja/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="June 29 – July 3, 2026" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**：Pro、Team Standard、および Enterprise サブスクリプションシートの新しいデフォルトモデル。Sonnet 価格での最高レベルのコーディングとツール使用、ネイティブ 1M トークンコンテキストウィンドウ、およびデフォルトで有効な適応的思考を備えています。

  今週のその他の機能：**Chrome の Claude** はすべての直接 Anthropic プランで一般提供されています。**subagents はデフォルトでバックグラウンドで実行** されるため、Claude は実行中も動作し続けます。**Claude Desktop on Linux** は Ubuntu および Debian でベータ版として登場しました。**`/radio`** は Claude FM lo-fi ラジオにチューニングします。

  [Week 27 ダイジェストを読む →](/docs/ja/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="June 22–26, 2026" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**：インタラクティブな `/mcp` メニューの代わりに、シェルから設定済み MCP サーバーを認証し、後で `claude mcp logout` でその保存された認証情報をクリアします。

  今週のその他の機能：**シェルモードがコマンド出力に応答します**（`! npm test` は 2 番目のプロンプトなしで説明を取得します）。**`/rewind`** は `/clear` が実行される前の会話を再開できます。**バックグラウンドサブエージェント** は許可プロンプトをメインセッションに表示するようになり、自動的に拒否されなくなります。

  [Week 26 ダイジェストを読む →](/docs/ja/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="June 15–19, 2026" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts**：セッションの出力を claude.ai 上のライブで共有可能なページに変換し、セッションが動作するにつれてその場で更新されます。現在 Team および Enterprise プランでベータ版です。

  今週のその他の機能：**deny および ask ルール** は `Tool(param:value)` でツールパラメータにマッチします。例えば `Agent(model:opus)`。**`/config key=value`** は `-p` モードおよび Remote Control からプロンプトから任意の設定を設定します。**auto mode** は、ローカル作業を破棄するよう求めなかった場合、破壊的な git コマンドをブロックします。

  [Week 25 ダイジェストを読む →](/docs/ja/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="June 8–12, 2026" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**：プロンプトキャッシュを再構築することなく、会話の途中で現在のセッションを新しい作業ディレクトリに移動します。

  今週のその他の機能：**サブエージェントは独自のサブエージェントを生成できます**（バックグラウンドチェーンは 5 レベルの深さでキャップされています）。**`--safe-mode`** はすべてのカスタマイズを無効にした状態で Claude Code を起動してトラブルシューティングを行います。**`fallbackModel`** は順番に試される最大 3 つのフォールバックモデルを設定します。

  [Week 24 ダイジェストを読む →](/docs/ja/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="June 1–5, 2026" tags={["v2.1.158–v2.1.165"]}>
  **Amazon Bedrock、Google Cloud の Agent Platform、および Microsoft Foundry での Auto mode**：auto mode は Opus 4.7 および Opus 4.8 のサードパーティプロバイダーで利用可能になり、許可プロンプトをバックグラウンド安全チェックに置き換えます。

  今週のその他の機能：**より安全な自動編集** は `acceptEdits` モードでコードを実行できるファイルを書き込む前にプロンプトを表示します。**`/plugin list`** はインストール済みプラグインをインラインで出力します。**バージョン要件** により、マネージドデプロイメントは承認された Claude Code バージョン範囲を要求できます。

  [Week 23 ダイジェストを読む →](/docs/ja/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="May 25–29, 2026" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**：Max、Team Premium、Enterprise 従量課金制、および Anthropic API アカウント向けの新しいデフォルトモデル。デフォルトで高いエフォートレベルを備え、最も難しいタスク向けに `/effort xhigh` をサポートしています。

  今週のその他の機能：**dynamic workflows** は Claude が作成するスクリプトから数十から数百のサブエージェントを調整します。**security-guidance プラグイン** は Claude の変更を脆弱性についてレビューします。**fast mode** は Opus 4.8 で 1 MTok あたり $10/$50 で実行されます。

  [Week 22 ダイジェストを読む →](/docs/ja/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="May 18–22, 2026" tags={["v2.1.143–v2.1.149"]}>
  **Pro プランの Auto mode**：auto mode は Pro アカウントで実行され、Opus と並行して Sonnet 4.6 をサポートし、許可プロンプトをバックグラウンド安全チェックに置き換えます。

  今週のその他の機能：**`/usage`** はスキル、サブエージェント、プラグイン、および MCP サーバーごとにプラン制限を駆動するものを分解します。新しい **`/code-review`** コマンドは正確性バグを報告します。**background sessions** は `/resume` に表示され、ピン留めされると生き続けます。

  [Week 21 ダイジェストを読む →](/docs/ja/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="May 11–15, 2026" tags={["v2.1.139–v2.1.142"]}>
  **エージェントビュー**：`claude agents` は Claude Code セッションごとに 1 つの画面を開き、何が実行中か、何があなたをブロックしているか、何が完了したかを表示します。

  今週のその他の機能：**`/goal`** は完了条件が成立するまで Claude を複数ターンにわたって動作させ続けます。**fast mode** はデフォルトで Opus 4.7 で実行されるようになりました。**Rewind メニュー** は「ここまで要約」で以前のコンテキストを圧縮できます。

  [Week 20 ダイジェストを読む →](/docs/ja/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="May 4–8, 2026" tags={["v2.1.128–v2.1.136"]}>
  **プラグインが `.zip` アーカイブと URL から読み込まれます**：`--plugin-dir` は `.zip` ファイルを受け入れるようになり、`--plugin-url` は現在のセッション用にプラグインアーカイブをフェッチします。

  今週のその他の機能：**`worktree.baseRef`** は新しい worktree がリモートデフォルトまたはローカル `HEAD` からブランチするかどうかを選択します。**auto mode hard deny ルール** は許可例外に関係なく無条件にアクションをブロックします。**hooks は有効なエフォートレベルを確認** できます（`effort.level` および `$CLAUDE_EFFORT` 経由）。

  [Week 19 ダイジェストを読む →](/docs/ja/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="April 27 – May 1, 2026" tags={["v2.1.120–v2.1.126"]}>
  **Git Bash なしの Windows**：Git for Windows は不要になり、Bash がない場合、Claude Code は PowerShell をシェルツールとして使用します。

  今週のその他の機能：**`claude ultrareview`** はクラウドコードレビューを CI とスクリプトにもたらします。**`claude project purge`** はプロジェクトのローカル状態をクリーンアップします。**PR URL を `/resume` に貼り付ける** とそれを作成したセッションを見つけます。

  [Week 18 ダイジェストを読む →](/docs/ja/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="April 20–24, 2026" tags={["v2.1.114–v2.1.119"]}>
  **`/ultrareview`** がパブリックリサーチプレビューとしてオープンしました。バグ検出エージェントのフリートがクラウドで実行され、検出結果が自動的に CLI またはデスクトップに戻ります。

  今週のその他の機能：**セッションの概要** はターミナルがフォーカスされていない間に何が起こったかを表示します。**カスタムテーマ** では `/theme` またはプラグインから色パレットを構築して配布できます。**ウェブ上の Claude Code** は新しいセッションサイドバーとドラッグアンドドロップレイアウトでリデザインされました。

  [Week 17 ダイジェストを読む →](/docs/ja/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="April 13–17, 2026" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7** が Max および Team Premium の新しいデフォルトとしてリリースされました。ほとんどのコーディング作業に推奨される新しい `xhigh` エフォートレベルと、インタラクティブな `/effort` スライダーで調整できます。

  今週のその他の機能：ウェブ上の Claude Code の **Routines** はスケジュール、GitHub イベント、または API 呼び出しからテンプレート化されたクラウドエージェントを実行します。**モバイルプッシュ通知** は長いタスクが終了したときまたは Claude があなたが必要なときに電話に通知を送ります。`/usage` はあなたの制限を何が駆動しているかを表示します。CLI はネイティブバイナリに移行しました。

  [Week 16 ダイジェストを読む →](/docs/ja/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="April 6–10, 2026" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan** が早期プレビューに入りました。CLI からクラウドでプランを作成し、ウェブエディタで確認およびコメントしてから、リモートで実行するか、ローカルに戻します。最初の実行は自動的にクラウド環境を作成します。

  今週のその他の機能：**Monitor** ツールはバックグラウンドイベントをストリーミングして会話に流し込むため、Claude はログをテールして実時間で反応できます。`/loop` は間隔を省略すると自動ペースします。`/team-onboarding` はセットアップを再生可能なガイドにパッケージします。`/autofix-pr` はターミナルから PR 自動修正をオンにします。

  [Week 15 ダイジェストを読む →](/docs/ja/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="March 30 – April 3, 2026" tags={["v2.1.86–v2.1.91"]}>
  **Computer use** がリサーチプレビューで CLI に登場しました。Claude はネイティブアプリを開き、UI をクリックして、ターミナルから変更を確認できます。GUI でのみ確認できるものを完了するのに最適です。

  今週のその他の機能：`/powerup` インタラクティブレッスン、ちらつきのない alt スクリーンレンダリング、ツールごとの MCP 結果サイズオーバーライド（最大 500K）、および Bash ツールの `PATH` 上のプラグイン実行ファイル。

  [Week 14 ダイジェストを読む →](/docs/ja/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="March 23–27, 2026" tags={["v2.1.83–v2.1.85"]}>
  **Auto mode** がリサーチプレビューでリリースされました。分類器があなたの許可プロンプトを処理するため、安全なアクションは中断なく実行され、危険なアクションはブロックされます。すべてを承認することと `--dangerously-skip-permissions` の中間地点です。

  今週のその他の機能：デスクトップアプリでのコンピュータ使用、ウェブでの PR 自動修正、`/` でのトランスクリプト検索、Windows 用のネイティブ PowerShell ツール、および条件付き `if` フック。

  [Week 13 ダイジェストを読む →](/docs/ja/whats-new/2026-w13)
</Update>
