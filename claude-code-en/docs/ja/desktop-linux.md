> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop on Linux (beta)

> Ubuntu と Debian に Claude デスクトップアプリをインストールおよび更新する

<Note>
  Claude デスクトップアプリの Linux サポートはベータ版です。
</Note>

Linux 上のデスクトップアプリは、macOS と Windows と同じ Chat、Cowork、Claude Code エクスペリエンスを提供します。並列セッション、ビジュアル diff レビュー、統合ターミナルとエディタ、ライブアプリプレビューが含まれます。機能リファレンスについては、[Claude Code Desktop を使用する](/docs/ja/desktop)を参照してください。

<h2 id="requirements">
  要件
</h2>

* Debian ベースのディストリビューション：Ubuntu 22.04 以降、または Debian 12 以降
* x86\_64 または arm64

これらの要件を満たす他の Debian ベースのディストリビューションは動作する可能性がありますが、公式にはテストされていません。Fedora や Arch などの Debian ベース以外のディストリビューションでは、代わりに [CLI](/docs/ja/setup#system-requirements) を実行してください。Windows で WSL 2 を使用している場合は、Windows デスクトップアプリをインストールし、ディストリビューション内でセッションを実行してください。[Claude Code Desktop in WSL](/docs/ja/desktop-wsl) を参照してください。

<h3 id="cowork-requirements">
  Cowork の要件
</h3>

Cowork は [Dispatch とより長い agentic work](https://claude.com/docs/cowork/overview) 用のデスクトップタブです。Linux では、Cowork はこれらのタスクを、デスクトップアプリが QEMU と KVM でホストする仮想マシン内で実行します。Cowork を使用するには、マシンが以下を必要とします。

* **ハードウェア仮想化**：ファームウェア設定で有効にされていること。これがない場合、Cowork タブは「Cowork requires hardware virtualization (KVM)」と報告します。
* **QEMU と UEFI ファームウェア**：x86\_64 では `qemu-system-x86`、`ovmf`、`virtiofsd`、または arm64 では `qemu-system-arm`、`qemu-efi-aarch64`、`virtiofsd`。`apt install claude-desktop` はデフォルトで推奨パッケージとしてこれらをインストールします。`--no-install-recommends` でインストールした場合、またはシステムが推奨パッケージをスキップする最小イメージの場合、Cowork タブは「Cowork requires QEMU」と報告し、実行する `apt install` コマンドを表示します。Ubuntu 22.04 には `virtiofsd` パッケージがありません。アプリはそこでバンドルされたコピーを使用します。
* **`/dev/kvm` へのアクセス**：`sudo usermod -aG kvm $USER` でユーザーを `kvm` グループに追加し、ログアウトしてからログインし直してください。一部のデスクトップ環境は、グループなしでログインユーザーに `/dev/kvm` へのアクセスを許可しますが、Cowork は `/dev/vhost-vsock` も必要とし、これは `kvm` グループメンバーのみが開くことができます。`/dev/kvm` が既に機能している場合でも、グループに参加してください。

アプリはこれらの要件を起動時に 1 回チェックします。パッケージをインストール後にアプリを再起動し、グループに参加後にログアウトしてからログインし直してください。`/dev/vhost-vsock` が見つからず、実行中のカーネルが `/lib/modules` の下にモジュールディレクトリを持たない場合、Cowork タブは、カーネルが Cowork が必要とする仮想化サポートを含まず、手動で追加できないことを報告します。この組み合わせは ChromeOS とコンテナベースの Linux 環境で一般的です。

<h2 id="install">
  インストール
</h2>

Anthropic の apt リポジトリからインストールして、更新がシステムの通常のパッケージ更新を通じて提供されるようにします。ターミナルを開き、各ステップのコマンドを実行してください。

<Steps>
  <Step title="Anthropic の apt リポジトリを追加する">
    このステップでは `curl` を使用して署名キーをダウンロードし、`gpg` で検証します。新しい Debian および Ubuntu インストールには、これらのコマンドが含まれていない場合があります。どちらかのコマンドが `command not found` を報告する場合は、まず両方をインストールしてください。

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Anthropic の署名キーをダウンロードします。

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    コマンドは成功時には何も出力せず、失敗時には `curl:` エラーを出力します。署名キーが不足しているか間違っていると、後で `apt update` が `NO_PUBKEY BAA929FF1A7ECACE` で失敗するため、キーがダウンロードされ、Anthropic に属していることを確認してから続行してください。

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    gpg が出力するフィンガープリントは `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE` である必要があります。gpg がファイルを開けない、または有効な OpenPGP データが含まれていないと報告する場合は、ダウンロードが失敗したか、間違ったコンテンツが返されました。ネットワークが `downloads.claude.ai` に到達できることを確認してから、ダウンロードコマンドを再度実行してください。

    リポジトリを登録します。

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="パッケージをインストールする">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="起動してサインインする">
    アプリケーションランチャーから **Claude** を起動するか、ターミナルから `claude-desktop` を実行して、Anthropic アカウントでサインインします。

    Linux アプリは macOS と Windows と同じ方法でサインインします。claude.ai サブスクリプション、または組織の SSO を通じてサインインします。Desktop は Claude Console API キーを直接受け入れません。API キー認証には [CLI](/docs/ja/quickstart) を使用してください。Google Cloud の Agent Platform または LLM ゲートウェイに Desktop をルーティングするエンタープライズデプロイメントについては、[Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) および [ネットワーク設定](/docs/ja/network-config) を参照してください。
  </Step>
</Steps>

<h3 id="install-from-a-downloaded-file">
  ダウンロードしたファイルからインストールする
</h3>

apt リポジトリを使用できない場合は、リポジトリのパッケージプールから `.deb` パッケージを直接ダウンロードしてください。このコマンドはリポジトリインデックスでアーキテクチャに対応した最新パッケージを検索し、現在のディレクトリにダウンロードします。

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

コマンドが `Remote file name has no length` で失敗する場合、検索がパッケージパスを返しませんでした。これはリポジトリインデックスを取得できなかった場合（例えば、ネットワークが `downloads.claude.ai` をブロックしている場合）、またはアーキテクチャに対応したパッケージが存在しない場合を意味します。ネットワークが `downloads.claude.ai` に到達できることを確認し、`dpkg --print-architecture` が `amd64` または `arm64` を出力することを確認してください。リポジトリは他のアーキテクチャのパッケージを公開していません。

Anthropic の apt リポジトリを登録せずにインストールするには、まず `/etc/default/claude-desktop` を `CLAUDE_DESKTOP_ADD_REPO="false"` という行で作成してください。リポジトリがない場合、apt は新しいバージョンを提供しません。更新するには、ダウンロードコマンドを再度実行して再インストールするか、後で [リポジトリを登録](#install) してください。

次に、ダウンロードしたファイルをソフトウェアインストーラー（GNOME Software など）で開くか、ダウンロードしたファイルが含まれているディレクトリから apt でインストールします。

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

apt が `E: Unsupported file ./claude-desktop_*.deb given on commandline` を報告する場合、パターンが現在のディレクトリ内の `.deb` ファイルと一致しませんでした。ダウンロードが完了したことを確認してから、ファイルが含まれているディレクトリからコマンドを再度実行してください。

`.deb` をインストールすると、Anthropic の apt リポジトリも `/etc/apt/sources.list.d/claude-desktop.list` に登録されるため、今後の更新はシステムの [通常のパッケージ更新](#update) で提供されます。

<h2 id="update">
  更新
</h2>

デスクトップアプリは Linux では自動的に更新されません。更新はシステムの通常のパッケージ更新で提供されます。

```bash theme={null}
sudo apt update && sudo apt upgrade
```

ディストリビューションのグラフィカルソフトウェアアップデーターも新しいバージョンを検出します。

<h2 id="uninstall">
  アンインストール
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

パッケージをアンインストールすると、登録されたリポジトリエントリと署名キーも削除されます。[Add Anthropic's apt repository](#install) ステップでリポジトリエントリを自分で追加した場合は、それも削除してください。

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  トラブルシューティング
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  claude-desktop パッケージが見つからない
</h3>

`sudo apt install claude-desktop` が `E: Unable to locate package claude-desktop` で失敗する場合、apt が追加したリポジトリを見つけられていません。以下を確認してください。

* リポジトリを追加した後に `sudo apt update` を実行してください。`apt install` だけでは、最後に `apt update` を実行した後に追加したリポジトリは認識されません。
* リポジトリエントリが書き込まれたことを確認します。`cat /etc/apt/sources.list.d/claude-desktop.list` は [Anthropic の apt リポジトリを追加](#install) ステップの `deb` 行を表示する必要があります。ファイルが空または見つからない場合は、そのステップを再度実行してください。
* アーキテクチャがサポートされていることを確認します。`dpkg --print-architecture` は `amd64` または `arm64` を出力する必要があります。リポジトリは他のアーキテクチャのパッケージを公開していません。
* `sudo apt update` を再度実行し、`downloads.claude.ai` に関連するエラーの出力を確認します。そこでネットワークまたはキーエラーが発生している場合は、リポジトリが追加されましたが、到達またはベリファイできなかったことを意味します。

リポジトリが配置されており到達可能で、パッケージがまだ見つからない場合は、代わりに [ダウンロードしたファイルからインストール](#install-from-a-downloaded-file) してください。

<h3 id="unmet-dependencies">
  未解決の依存関係
</h3>

`apt` が `The following packages have unmet dependencies` または `Unsatisfied dependencies` で停止する場合は、名前が付けられている依存関係を読んでください。

* `libc6 (>= 2.34)`：ディストリビューションがパッケージがサポートするより古いです。Ubuntu 20.04 は `libc6` 2.31 を搭載しています。Ubuntu 22.04 以降または Debian 12 以降にアップグレードしてください。
* すべての欠落している依存関係が `:amd64` または `:arm64` サフィックス付きで `not installable` を表示する場合：マシンのアーキテクチャと異なるアーキテクチャの `.deb` をダウンロードしました。`dpkg --print-architecture` を実行して一致する `.deb` をダウンロードするか、[apt リポジトリからインストール](#install) してください。これはアーキテクチャのパッケージを選択します。

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  root として --no-sandbox なしで実行することはサポートされていません
</h3>

`claude-desktop` がこのメッセージで終了する場合、root として起動しました。通常のユーザーとしてログインして、そこから起動してください。

<h3 id="cowork-isn’t-available">
  Cowork が利用できない
</h3>

Cowork タブがこれらのメッセージのいずれかを表示する場合は、名前が付けられている要件を修正してから、アプリを再起動してください。

* **Cowork には QEMU が必要です**：メッセージが一覧表示する [QEMU と UEFI ファームウェアパッケージ](#cowork-requirements) をインストールしてください。
* **Cowork にはハードウェア仮想化（KVM）が必要です**：ファームウェア設定で [ハードウェア仮想化](#cowork-requirements) をオンにしてください。
* **Claude には仮想化を使用する権限がありません（/dev/kvm）**：ユーザーを [`kvm` グループ](#cowork-requirements) に追加してから、ログアウトしてログインし直してください。
* **Cowork には `vhost_vsock` カーネルモジュールが必要です**：`sudo modprobe vhost_vsock` を実行してから、アプリを再起動してください。これは現在のブートのみモジュールをロードします。すべてのブートでロードするには、`echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf` を実行してください。

<h2 id="what’s-not-in-the-linux-beta-yet">
  Linux ベータ版にまだ含まれていない機能
</h2>

* **Computer Use**: [アプリとスクリーン制御](/docs/ja/desktop#let-claude-use-your-computer)は Linux では利用できません。
* **Dictation**: 音声入力は Linux デスクトップアプリでは利用できません。代わりに CLI で[音声ディクテーション](/docs/ja/voice-dictation)を使用してください。
* **Quick Entry グローバルホットキー**: X11 で動作します。ネイティブ Wayland では、デスクトップ環境の GlobalShortcuts ポータルが必要です。
* **Fedora と RHEL**: 現在、Debian ベースのディストリビューションのみがサポートされています。追加のディストリビューションのサポートは今後提供される予定です。

デスクトップアプリでまだ利用できない機能については、[CLI](/docs/ja/quickstart)は同じ Claude Code エンジンを実行し、より広い範囲の Linux ディストリビューションをサポートしています。[システム要件](/docs/ja/setup#system-requirements)を参照してください。
