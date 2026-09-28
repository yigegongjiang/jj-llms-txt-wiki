> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 音声ディクテーション

> Claude Code CLI で音声ディクテーション機能を使用して、プロンプトを話して入力できます。長押しまたはタップで録音できます。

Claude Code CLI でプロンプトを入力する代わりに、話して入力できます。音声はプロンプト入力にリアルタイムで文字起こしされるため、同じメッセージ内で音声と入力を混在させることができます。`/voice` で音声ディクテーションを有効にしてから、キーを押しながら話すか、1 回タップして開始し、もう 1 回タップして送信します。

ディクテーション機能は [エージェントビュー](/docs/ja/agent-view#peek-and-reply) でも機能します。ディスパッチ入力またはピークパネルの返信がフォーカスされている間、プッシュトークキーを長押しまたはタップして、バックグラウンドセッションにディクテーションを行うことができます。

<h2 id="requirements">
  要件
</h2>

音声ディクテーションは、記録された音声を Anthropic のサーバーにストリーミングして文字起こしします。音声はローカルで処理されません。以下のすべてが必要です。

* **Claude.ai アカウント**：音声テキスト変換サービスは Claude.ai アカウントで認証した場合にのみ利用可能であり、Claude Code が Anthropic API キー、Amazon Bedrock、Google Cloud の Agent Platform、または Microsoft Foundry を直接使用するように設定されている場合は利用できません。
* **ローカルマイク**：音声ディクテーションは [Web 上の Claude Code](/docs/ja/claude-code-on-the-web)や SSH セッションなどのリモート環境では機能しません。
* **WSL で Claude Code を実行する場合は WSLg**：WSLg は Windows 10 または 11 の Microsoft Store からインストールされた WSL2 に含まれています。WSLg が利用できない場合（例えば WSL1 など）は、代わりにネイティブ Windows で Claude Code を実行してください。

文字起こしは Claude メッセージやトークンを消費せず、`/usage` に表示される制限にはカウントされません。Anthropic がデータをどのように処理するかについては、[データ使用](/docs/ja/data-usage)を参照してください。

音声録音は macOS、Linux、Windows のビルトイン ネイティブ モジュールを使用します。Linux では、ネイティブ モジュールが読み込めない場合、Claude Code は ALSA utils の `arecord` または SoX の `rec` にフォールバックします。どちらも利用できない場合、`/voice` はパッケージ マネージャーのインストール コマンドを出力します。

Claude Code [VS Code 拡張機能](/docs/ja/vs-code)も、同じ Claude.ai アカウント要件で音声ディクテーションをサポートしています。マイクはローカル マシンにあり、拡張機能はリモート ホストで実行されるため、SSH、Dev Containers、Codespaces などの VS Code Remote セッションでは利用できません。

<h2 id="enable-voice-dictation">
  音声ディクテーションを有効にする
</h2>

`/voice` を実行して音声ディクテーションを有効にします。初めて有効にするときは、Claude Code はマイク チェックを実行します。macOS では、ターミナルにマイク権限がまだ付与されていない場合、システム マイク権限プロンプトがトリガーされます。

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice` はオプションのモード引数を受け入れます。

| コマンド          | 効果                                      |
| :------------ | :-------------------------------------- |
| `/voice`      | オン/オフを切り替え、現在のモードを保持                    |
| `/voice hold` | [長押しモード](#hold-to-record)で有効にする         |
| `/voice tap`  | [タップモード](#tap-to-record-and-send)で有効にする |
| `/voice off`  | 無効にする                                   |

音声ディクテーションはセッション間で保持されます。`/voice` を実行する代わりに、[ユーザー設定ファイル](/docs/ja/settings)で直接設定します。

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

音声ディクテーションが有効な場合、最初の 3 つのセッションでは、プロンプトが空のときに入力フッターに `hold space to speak` ヒントが表示されます。ヒントは現在の `voice:pushToTalk` バインディングを反映し、[ディクテーション キーを再バインド](#rebind-the-dictation-key)する場合は更新されます。ヒント テキストは両方のモードで同じであり、[カスタム ステータス ラインを](/docs/ja/statusline)設定している場合は表示されません。

文字起こしは両方のモードでコーディング語彙用に調整されています。`regex`、`OAuth`、`JSON`、`localhost` などの一般的な開発用語は正しく認識され、現在のプロジェクト名と git ブランチ名は認識ヒントとして自動的に追加されます。

<h2 id="hold-to-record">
  長押しして録音
</h2>

長押しモードはプッシュツートーク機能です。キーを押している間は録音が実行され、キーを離すと停止します。これはデフォルト モードです。

`Space` を長押しして録音を開始します。Claude Code はターミナルからの高速キー リピート イベントを監視することでキーの長押しを検出するため、録音が開始される前に短いウォームアップがあります。フッターはウォームアップ中に `keep holding…` を表示し、録音がアクティブになると `listening…` に切り替わります。録音中、プロンプト カーソルはマイク レベルに応じて上下する棒になります。ただし、[`prefersReducedMotion`](/docs/ja/settings-reference#prefersreducedmotion) がオンになっている場合を除きます。

最初の数個のキー リピート文字はウォームアップ中に入力に入力され、録音がアクティブになると自動的に削除されます。単一の `Space` タップはスペースを入力します。長押し検出は高速リピートでのみトリガーされるためです。

`Space` を長押しまたはタップして音声入力を開始するのは、キープレスがプロンプトに入力される場所のみです。[トランスクリプト ビューアー](/docs/ja/interactive-mode#transcript-viewer)では、`Space` は会話をページングし、[vim モード](/docs/ja/interactive-mode#vim-editor-mode)の INSERT 外では コマンドです。[修飾子の組み合わせにリバインド](#rebind-the-dictation-key)（`meta+k` など）は テキストを入力しないため、これらの場所からも音声入力を開始します。

<Tip>
  ウォームアップをスキップするには、`/voice tap` で[タップモード](#tap-to-record-and-send)に切り替えるか、`meta+k` などの[修飾子の組み合わせにリバインド](#rebind-the-dictation-key)してください。修飾子の組み合わせは最初のキープレスで録音を開始します。
</Tip>

音声はプロンプトに話すときに表示され、文字起こしが確定されるまで薄く表示されます。`Space` を離して録音を停止し、テキストを確定します。文字起こしはカーソル位置に挿入され、カーソルは挿入されたテキストの末尾に留まるため、任意の順序で入力と音声ディクテーションを混在させることができます。`Space` を再度長押しして別の録音を追加するか、カーソルを最初に移動して、プロンプト内の別の場所に音声を挿入します。

```
> refactor the auth middleware to ▮
  # hold space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

デフォルトでは、キーを離すと文字起こしが挿入され、`Enter` を押すのを待ちます。`voice` 設定オブジェクトで `"autoSubmit": true` を設定して、文字起こしが少なくとも 3 語以上の場合、キーを離すときにプロンプトを自動的に送信します。

<h2 id="tap-to-record-and-send">
  タップして録音して送信
</h2>

タップモードは単一のキープレスで録音を切り替えます。1 回タップして開始し、話してから、もう 1 回タップしてプロンプトを送信します。ウォームアップはなく、キーを押し続ける必要はありません。

`/voice tap` でタップモードを有効にします。プロンプト入力が空の場合、`Space` をタップして録音を開始します。フッターは録音中に `● REC · tap to send` と表示されます。`Space` をもう 1 回タップして停止します。

Claude Code は文字起こしを挿入し、文字起こしが少なくとも 3 語以上の場合、プロンプトを自動的に送信します。短い文字起こしは挿入されますが送信されないため、誤ったタップは単語を送信しません。

3 語のしきい値は、スペースなしで書かれた言語の単語をカウントします。日本語、中国語、タイ語の文字起こしは個々の単語をカウントするため、タップモードおよび `autoSubmit` を使用したホールドモードで自動送信されます。

最初のタップはプロンプト入力が空の場合にのみ録音を開始するため、メッセージを作成しながら通常どおりスペースを入力できます。2 番目のタップは入力内容に関係なく録音を停止します。15 秒以上の無音または 2 分間の合計の後、録音も自動的に停止します。

<h2 id="cancel-a-recording">
  録音をキャンセルする
</h2>

`Esc` キーまたは `Ctrl+C` キーを押して、ディクテーションを確定する代わりにキャンセルできます。Claude Code はマイクを停止し、トランスクリプトを破棄し、プロンプトを録音開始前の状態に復元します。

両方のキーは、完了した録音のトランスクリプトがまだ処理中の場合もキャンセルできます。処理中に編集または送信したプロンプトは、そのままの状態で保持されます。

どちらのキーもキャンセルを実行する以外の処理は行いません。`Esc` は Claude の応答を中断しませんし、`Ctrl+C` はプロンプトをクリアしたり、[Claude Code を終了する 2 回のプレスの最初のプレス](/docs/ja/interactive-mode#general-controls)としてカウントされません。

<h2 id="change-the-dictation-language">
  音声ディクテーション言語を変更する
</h2>

音声ディクテーションは、Claude の応答言語を制御する同じ [`language` 設定](/docs/ja/settings-reference#language)を使用します。その設定が空の場合、音声ディクテーションはデフォルトで英語になります。VS Code 拡張機能では、`language` が空の場合、音声ディクテーションは VS Code の `accessibility.voice.speechLanguage` 設定を使用してから、デフォルトで英語になります。

<Accordion title="サポートされている音声ディクテーション言語">
  | 言語      | コード  |
  | :------ | :--- |
  | チェコ語    | `cs` |
  | デンマーク語  | `da` |
  | オランダ語   | `nl` |
  | 英語      | `en` |
  | フランス語   | `fr` |
  | ドイツ語    | `de` |
  | ギリシャ語   | `el` |
  | ヒンディー語  | `hi` |
  | インドネシア語 | `id` |
  | イタリア語   | `it` |
  | 日本語     | `ja` |
  | 韓国語     | `ko` |
  | ノルウェー語  | `no` |
  | ポーランド語  | `pl` |
  | ポルトガル語  | `pt` |
  | ロシア語    | `ru` |
  | スペイン語   | `es` |
  | スウェーデン語 | `sv` |
  | トルコ語    | `tr` |
  | ウクライナ語  | `uk` |
</Accordion>

`/config` で言語を設定するか、設定で直接設定します。[BCP 47 言語コード](https://en.wikipedia.org/wiki/IETF_language_tag)または言語名のいずれかを使用できます。

```json theme={null}
{
  "language": "japanese"
}
```

`language` 設定がサポートされているリストにない場合、`/voice` は有効化時に警告を表示し、音声ディクテーションの場合は英語にフォールバックします。Claude のテキスト応答はこのフォールバックの影響を受けません。

<h2 id="rebind-the-dictation-key">
  音声ディクテーション キーをリバインドする
</h2>

音声ディクテーション キーは `Chat` コンテキストの `voice:pushToTalk` にバインドされ、デフォルトは `Space` です。同じバインディングは長押しモードとタップモードの両方を制御します。[`~/.claude/keybindings.json`](/docs/ja/keybindings)でリバインドします。

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "meta+k": "voice:pushToTalk",
        "space": null
      }
    }
  ]
}
```

`voice:pushToTalk` アクションは一度に 1 つのキーを使用します。カスタム キーをバインドすると、デフォルトの `Space` バインディングが置き換わり、2 番目のトリガーが追加されるのではなく、この例の `"space": null` 行は明確にするためのものであり、動作を変更することなく省略できます。

長押しモードでは、`v` などのベア文字キーへのバインディングを避けてください。長押し検出はキー リピートに依存し、文字はウォームアップ中にプロンプトに入力されるためです。`Space` を使用するか、`meta+k` などの修飾子の組み合わせを使用して、ウォームアップなしで最初のキープレスで録音を開始します。タップモードにはウォームアップがないため、ほとんどのキーが機能します。

一部のキーはターミナル アプリケーションに配信されず、まったくバインドできません。たとえば、`Caps Lock` をバインドしようとするとエラーが表示されます。完全なキーバインディング構文と予約済みショートカットのリストについては、[キーボード ショートカットをカスタマイズする](/docs/ja/keybindings)を参照してください。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

音声ディクテーションが起動または記録されない場合の一般的な問題：

* **`Voice mode requires a Claude.ai account`**：API キーまたはサードパーティプロバイダーで認証されています。`/login` を実行して claude.ai アカウントでサインインしてください。
* **`Voice mode is disabled by your organization's policy`**：組織の管理者ポリシーが音声ディクテーションをオフにしています。組織の管理者に連絡して、組織で音声ディクテーションが利用可能かどうかを確認してください。
* **`Microphone access is denied`**：システム設定でターミナルにマイク権限を付与してください。macOS では、システム設定 → プライバシーとセキュリティ → マイク に移動してターミナルアプリを有効にしてから、`/voice` を再度実行してください。Windows では、設定 → プライバシーとセキュリティ → マイク に移動してデスクトップアプリのマイクアクセスをオンにしてから、`/voice` を再度実行してください。ターミナルが macOS 設定に表示されていない場合は、[Terminal not listed in macOS Microphone settings](#terminal-not-listed-in-macos-microphone-settings) を参照してください。
* **`Voice mode requires SoX for audio recording` on Linux**：ネイティブオーディオモジュールを読み込めず、フォールバックがインストールされていません。エラーメッセージに表示されているコマンドで SoX をインストールしてください。例えば `sudo apt-get install sox` です。
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**：SoX はインストールされていますが、ホストにオーディオキャプチャデバイスがありません。例えば、ヘッドレスサーバーまたはコンテナです。マイクを備えたマシンで Claude Code を実行してください。v2.1.195 以降、Linux 上の Claude Code はこの状況でこのメッセージを報告します。以前のバージョンは、SoX がすでにインストールされている場合でも SoX をインストールするよう求めていました。
* **`Voice mode could not find a working audio recorder in WSL`**：WSLg は ALSA デバイスではなく PulseAudio を通じてオーディオをルーティングするため、SoX は PulseAudio バックエンドを明示的にインストールする必要があります。`sudo apt install sox libsox-fmt-pulse` を実行してください。`sox` だけをインストールすると ALSA バックエンドが引き込まれますが、WSL では `/dev/snd` デバイスがないため記録できません。
* **`Voice input is failing repeatedly and has been paused`**：音声ディクテーションが 10 秒以内に 3 回のキャプチャ失敗に達しました。Claude Code は最初の失敗から 10 秒が経過するまでディクテーションを一時停止します。マイクの起動に失敗した場合、またはレコーダーが起動してからオーディオを生成せずに停止した場合、失敗としてカウントされます。これは通常、このホストのマイクまたはオーディオスタックがオーディオをキャプチャできないことを意味します。例えば、ヘッドレスサーバー、オーディオパススルーのないリモートシェル、またはマイク権限が拒否されている場合です。動作する入力デバイスを確認し、上記のエントリから根本原因を修正してから、音声を再度トリガーしてください。v2.1.202 より前は、起動時の失敗のみが一時停止にカウントされていました。
* **ホールドモードで `Space` を押し続けても何も起こらない**：押し続けている間、プロンプト入力を監視してください。スペースが蓄積し続ける場合、音声ディクテーションがオフになっている可能性があります。`/voice hold` を実行して有効にしてください。1 つまたは 2 つのスペースだけが表示されてから何も起こらない場合、音声ディクテーションはオンですがホールド検出がトリガーされていません。ホールド検出はターミナルがキーリピートイベントを送信する必要があるため、OS レベルでキーリピートが無効になっている場合、押し続けているキーを検出できません。キーリピート要件を回避するために `/voice tap` でタップモードに切り替えてください。
* **タップモードで `Space` をタップするとスペースが入力され、記録されない**：最初のタップは、プロンプト入力が空の場合にのみ記録を開始します。入力を最初にクリアするか、`/voice tap` を実行してタップモードであることを確認してください。
* **`No audio detected from microphone`**：記録が開始されましたが、無音がキャプチャされました。正しい入力デバイスがシステムデフォルトとして設定されており、その入力レベルがミュートされていないか、ゼロに近くないことを確認してください。Windows では、設定 → システム → サウンド → 入力 を開いてマイクを選択してください。macOS では、システム設定 → サウンド → 入力 を開いてください。
* **`Voice connection failed`**：接続に失敗したため、記録が文字起こしサービスに到達しませんでした。ネットワークを確認して、もう一度試してください。オーディオをキャプチャしない記録は、このメッセージの代わりに `No audio detected from microphone` を報告します。v2.1.200 より前は、無音マイクが接続失敗を報告する可能性があり、実際の問題が入力デバイスであるのに、ネットワーク問題を示唆していました。
* **`Voice stream error: WebSocket upgrade rejected with HTTP <status>`**：サーバーが表示された HTTP ステータスで接続を拒否したため、これはネットワーク障害ではありません。400 範囲のステータスは通常、古いサインイン、またはプロキシもしくはボット保護サービスが文字起こしサービスの代わりに応答していることを意味します。`/login` を実行してサインインを更新し、ステータスが続く場合はネットワークパス上の VPN またはプロキシを確認してください。拒否が到着したときにまだ記録中の場合、Claude Code は 400 範囲外のステータスを 1 回再試行してからこのメッセージを表示します。400 範囲のステータスは再試行しません。v2.1.229 から v2.1.231 では、ネイティブビルドはこのメッセージを表示しませんでした：Claude Code は記録を続け、ホールドモードフッターは `listening…` を表示し続け、記録を停止した後に `Voice connection failed` を報告しました。
* **`No speech detected`**：オーディオは文字起こしサービスに到達しましたが、単語は認識されませんでした。マイクに近づいて話し、背景ノイズを減らし、[ディクテーション言語](#change-the-dictation-language) が話している言語と一致することを確認してください。
* **文字起こしが乱れているか、言語が間違っている**：ディクテーションはデフォルトで英語です。別の言語でディクテーションしている場合は、最初に `/config` で設定してください。[ディクテーション言語を変更する](#change-the-dictation-language) を参照してください。

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  Terminal not listed in macOS Microphone settings
</h3>

ターミナルアプリがシステム設定 → プライバシーとセキュリティ → マイク に表示されない場合、有効にできるトグルはありません。ターミナルの権限状態をリセットして、次の `/voice` 実行が新しい macOS 権限プロンプトをトリガーするようにしてください。

<Steps>
  <Step title="Reset the microphone permission for your terminal">
    `tccutil reset Microphone <bundle-id>` を実行します。`<bundle-id>` をターミナルの識別子に置き換えてください：組み込みターミナルの場合は `com.apple.Terminal`、iTerm2 の場合は `com.googlecode.iterm2`。その他のターミナルの場合は、`osascript -e 'id of app "AppName"'` で識別子を調べてください。

    <Warning>
      バンドル ID なしで `tccutil reset Microphone` を実行できますが、Mac 上のすべてのアプリ（Zoom や Slack などのアプリを含む）からマイクアクセスを取り消します。各アプリは次の使用時にアクセスを再度リクエストする必要があるため、アクティブな通話中に実行しないでください。
    </Warning>
  </Step>

  <Step title="Quit and relaunch your terminal">
    macOS は既に実行中のプロセスに再度プロンプトを表示しません。ウィンドウを閉じるだけでなく、Cmd+Q でターミナルアプリを終了してから、再度開いてください。
  </Step>

  <Step title="Trigger a fresh prompt">
    Claude Code を起動して `/voice` を実行してください。macOS がマイクアクセスを求めます。許可してください。
  </Step>
</Steps>

<h2 id="see-also">
  関連項目
</h2>

* [キーボード ショートカットをカスタマイズする](/docs/ja/keybindings): `voice:pushToTalk` および他の CLI キーボード アクションをリバインドする
* [すべての設定](/docs/ja/settings-reference#voice): `voice`、`language`、およびその他の設定キー
* [インタラクティブ モード](/docs/ja/interactive-mode): キーボード ショートカット、入力モード、およびセッション コントロール
* [コマンド](/docs/ja/commands): `/voice`、`/config`、およびその他すべてのコマンドのリファレンス
