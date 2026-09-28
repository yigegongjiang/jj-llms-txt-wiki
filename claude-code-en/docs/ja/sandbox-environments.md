> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# サンドボックス環境を選択する

> Claude Code のサンドボックスオプションを比較します。組み込みのサンドボックス化された Bash ツール、サンドボックスランタイム、dev コンテナ、Docker、VM があります。脅威モデルに適した分離を選択してください。

Claude Code を分離することで、セッションがホスト上で読み取り、書き込み、ネットワークにアクセスできる内容を制限します。これは、Claude に権限プロンプトを少なくして作業させたり、無人で実行したり、完全に信頼していないコードを指定したりする場合に最も重要です。

Claude Code は、軽量なコマンド単位のサンドボックスから完全に独立した仮想マシンまで、複数の種類の分離環境で実行できます。このページでは、それらを分離内容と必要な要件で比較し、脅威モデルに合ったものを選択するのに役立ち、組織全体でその選択を強制する方法を示します。

<Info>
  より広いセキュリティモデルについては、[セキュリティ](/docs/ja/security)を参照してください。Agent SDK デプロイメントについては、[セキュアなデプロイメント](/docs/ja/agent-sdk/secure-deployment)を参照してください。
</Info>

<h2 id="compare-sandboxing-approaches">
  サンドボックス化アプローチの比較
</h2>

以下の表の最初の 2 つのアプローチはコンテナなしでホストオペレーティングシステム上で実行されます。残りは Claude Code をコンテナまたは仮想マシン内に配置します。

| アプローチ                                        | 何が分離されるか                                       | Docker が必要 | セットアップの手間                                                                 |
| :------------------------------------------- | :--------------------------------------------- | :--------- | :------------------------------------------------------------------------ |
| [サンドボックス化された Bash ツール](#sandboxed-bash-tool) | Bash、PowerShell、Monitor コマンドとその子プロセス           | いいえ        | macOS では最小限。Linux と WSL2 では低い                                             |
| [サンドボックスランタイム](#sandbox-runtime)             | Claude Code プロセス全体（ファイルツール、MCP サーバー、hooks を含む） | いいえ        | 低い                                                                        |
| [Dev コンテナ](#dev-containers)                  | 完全な開発環境                                        | はい         | 中程度                                                                       |
| [カスタムコンテナ](#custom-container)                | 完全な開発環境                                        | はい         | 中程度から高い                                                                   |
| [仮想マシン](#virtual-machine)                    | 完全なオペレーティングシステム                                | いいえ        | 高い                                                                        |
| [クラウドセッション](#cloud-sessions)                 | 完全なオペレーティングシステム（Anthropic がホスト）                | いいえ        | なし。Claude サブスクリプションが必要で、`claude --cloud` で起動する場合を除き、接続された GitHub アカウントが必要 |

[サンドボックス化された Bash ツール](/docs/ja/sandboxing)は Claude Code に組み込まれており、Bash コマンドのみを制限します。組み込みファイルツール、MCP サーバー、hooks はホスト上で直接実行されます。表内の他のすべてのアプローチは、Claude Code プロセス全体を分離境界内に配置するため、ファイルツール、MCP サーバー、hooks も制限されます。

<Warning>
  サンドボックス分離はセキュリティ侵害の影響を軽減しますが、リスクを完全に排除することはできません。ネットワーク出力を許可するアプローチは、エージェントが読み取ることができるデータをリークする可能性があり、プロジェクトディレクトリを書き込み可能にマウントするアプローチはそのコードを変更する可能性があります。ハード制御としてサンドボックスに依存する前に、[セキュリティの制限事項](/docs/ja/sandboxing#security-limitations)を確認してください。

  分離はモデルに送信される内容を変更しません。プロンプトと Claude が読み取るファイルは、サンドボックスの有無にかかわらず、Anthropic API または設定されたプロバイダーに送信されます。Claude Code が送信する内容と削減方法については、[データ使用](/docs/ja/data-usage)を参照してください。
</Warning>

<h2 id="choose-an-approach">
  アプローチを選択する
</h2>

目標を以下の行と照合してから、その後に続く詳細セクションを読んでください。

| 実現したいこと                                                     | 開始するもの                                                                                                                     |
| :---------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| 自分のマシンでの日常的な作業中に権限プロンプトを減らす                                 | [サンドボックス化された Bash ツール](/docs/ja/sandboxing)（`/sandbox` で有効化）                                                                    |
| Claude に `--dangerously-skip-permissions` または自動モードで無人で作業させる | 事前設定された [dev コンテナ](/docs/ja/devcontainer)、任意のコンテナまたは VM、または [サンドボックスランタイム](#sandbox-runtime)                                    |
| Bash だけでなく MCP サーバーと hooks も分離し、Docker なしで実行する              | サンドボックスランタイム                                                                                                               |
| 信頼できないリポジトリで作業する                                            | 専用の仮想マシン、または Claude サブスクリプションがある場合は [Web 上の Claude Code](/docs/ja/claude-code-on-the-web)。`claude --cloud` で起動する場合、GitHub は不要です |
| チーム全体でサンドボックス化された環境を標準化する                                   | 事前設定された [dev コンテナ](/docs/ja/devcontainer)（リポジトリにコピー）                                                                            |
| ローカルセットアップなしのデバイスから Claude Code を使用する                       | [Web 上の Claude Code](/docs/ja/claude-code-on-the-web)（Claude サブスクリプションと接続された GitHub アカウントが必要）                                   |
| 組織内のすべての開発者に対して分離を要求する                                      | [組織全体で分離を強制](#enforce-isolation-across-an-organization)                                                                    |
| ネイティブ Windows ホストで作業する                                      | コンテナまたは VM、または WSL2 内で Bash サンドボックスを実行                                                                                     |

<h3 id="how-isolation-relates-to-permission-modes">
  分離が権限モードとどのように関連するか
</h3>

[権限モード](/docs/ja/permission-modes)は、ツール呼び出しが実行されるかどうか、および最初にプロンプトが表示されるかどうかを決定します。分離は、コマンドが実行されたら何にアクセスできるかを制限します。この 2 つは連携して機能します。権限モードがアクションを確認なしで実行させる場合、分離境界はそれらのアクションが到達できる内容を制限します。

`--dangerously-skip-permissions` を渡すと、Claude は最初にあなたに尋ねることなく行動します。[どのモードも自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves)は依然として適用されます。

間違いをキャッチするプロンプトがないため、選択した分離境界があなたのシステムを保護するものです。常にコンテナ、VM、または [サンドボックスランタイム](#sandbox-runtime)内で `--dangerously-skip-permissions` セッションを実行してください。ファイルツール、MCP サーバー、hooks も境界内にあります。Linux と macOS では、Claude Code はこのフラグを使用して root として実行されている場合は起動を拒否するため、コンテナ、VM、またはサンドボックスランタイムを非 root ユーザーとして実行してください。

[自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)はプロンプトを、アクションをレビューする分類器に置き換えます。分類器はアクション単位の制御であり、分離境界ではないため、分離境界は無人実行の防御層を追加し、`--dangerously-skip-permissions` の場合のように必須ではありません。

[サンドボックス化された Bash ツール](#sandboxed-bash-tool)単独では Bash のみを制限するため、どちらのモードでも完全に無人で実行するには不十分です。アプローチを重ねることができます。サンドボックス化された Bash ツールをコンテナまたは VM 内で実行すると、外側の環境境界の上に OS レベルのコマンド制限が得られます。Bash サンドボックス自体が権限ルールおよび権限モードとどのように相互作用するかについては、[サンドボックス化が権限および権限モードとどのように関連するか](/docs/ja/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes)を参照してください。

<h2 id="sandboxed-bash-tool">
  サンドボックス化された Bash ツール
</h2>

<Note>
  このオプションはネイティブ Windows をサポートしていません。Windows ホストでは、WSL2 または以下のコンテナまたは VM アプローチのいずれかを使用してください。
</Note>

サンドボックス化された Bash ツールは Claude Code に組み込まれています。オペレーティングシステムプリミティブを使用して、Claude が実行するすべての Bash、PowerShell、または Monitor コマンドのファイルシステムとネットワークアクセスを制限します。

`/sandbox` コマンドを実行してサンドボックスパネルを開き、モードを選択してください。[サンドボックス化](/docs/ja/sandboxing)ガイドでは、承認モード、デフォルト境界、および拡大または縮小する方法について説明しています。

コマンド単位のサンドボックスはセッションで実行されるすべてをカバーしていません。

* Read、Edit、WebFetch などの他の [組み込みツール](/docs/ja/tools-reference)は Claude Code プロセス内で実行され、任意のコードを生成しません。[権限ルール](/docs/ja/permissions)がパスまたはドメインでそれらをゲートします。
* [MCP](/docs/ja/mcp)サーバーと [command hooks](/docs/ja/hooks#command-hook-fields)は、ホスト上で制約なく実行される別のプロセスです。

組み込みツール、MCP サーバー、hooks をすべて 1 つの OS 境界の背後に配置するには、Claude Code プロセス全体を [サンドボックスランタイム](#sandbox-runtime)、[dev コンテナ](#dev-containers)、または [カスタムコンテナ](#custom-container)内で実行してください。

<h2 id="sandbox-runtime">
  サンドボックスランタイム
</h2>

[`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) パッケージは、組み込みの Bash サンドボックスが使用するのと同じ Seatbelt または bubblewrap 分離でプロセス全体をラップします。Claude Code をそれを通して実行すると、シェルコマンドだけでなく、セッション内のすべてのツール、hook、MCP サーバーが制限されます。ランタイムはベータ研究プレビューであり、パッケージが進化するにつれて設定形式が変わる可能性があります。

このセクションでは、設定する内容とランタイムが独自に実施する内容について説明します。Agent SDK アプリケーションでランタイムをデプロイする場合は、[セキュアデプロイメントガイド](/docs/ja/agent-sdk/secure-deployment#sandbox-runtime)を参照してください。

<h3 id="set-up-and-launch-the-runtime">
  ランタイムのセットアップと起動
</h3>

Linux と WSL2 では、ランタイムは組み込みサンドボックスと同じ `bubblewrap` および `socat` パッケージに加えて、Claude Code がバンドルしているが、スタンドアロンランタイムは PATH から解決する `ripgrep` に依存しています。[Linux と WSL2 のセットアップ](/docs/ja/sandboxing#set-up-linux-and-wsl2)で説明されているように `bubblewrap` と `socat` をインストールし、ディストリビューションのパッケージマネージャーから `ripgrep` をインストールしてください。macOS では追加のパッケージは必要ありません。ランタイムはそこで組み込みの Seatbelt サンドボックスを使用します。

デフォルトでは、ランタイムはネットワークアクセスを拒否し、書き込みを小さな組み込みランタイムパスセットに限定するため、Claude Code を起動する前に設定してください。設定を `~/.srt-settings.json` に、または `--settings` で渡すファイルに配置します。パッケージ [README](https://github.com/anthropic-experimental/sandbox-runtime) は完全な設定スキーマを文書化しています。

少なくとも以下への書き込みアクセスを許可してください。

* プロジェクトディレクトリ。
* Claude Code の設定パス `~/.claude` および `~/.claude.json`。
* `/tmp`。Claude Code はランタイムファイルをここに書き込みます。

セッションが必要とするネットワークドメインを許可してください。

* `api.anthropic.com`、またはプロバイダーのエンドポイント。サードパーティプロバイダーでは、`api.anthropic.com` も保持してください。WebFetch ドメインセーフティチェックは、`skipWebFetchPreflight: true` を設定しない限り、デフォルトでそれを呼び出します。
* `claude.ai` および `platform.claude.com`。[OAuth サインインとトークンリフレッシュ](/docs/ja/network-config#network-access-requirements)に必要です。API キーで認証されたランは、これら 2 つを削除できます。

Linux と WSL2 では、ランタイムは既に存在するパスにのみ書き込み許可を適用します。新しい環境では、最初の起動前に Claude Code の設定パスを作成してください。

```bash theme={null}
mkdir -p ~/.claude && echo '{}' > ~/.claude.json
```

設定ファイルが配置されたら、`npx` で Claude Code を起動し、ラップするコマンドとして `claude` を渡します。

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code はサンドボックス内で起動し、設定したファイルシステムとネットワーク境界があります。同じコマンドは、スタンドアロン MCP サーバーまたは他のヘルパープロセスのサンドボックス化に機能します。

<h3 id="what-the-runtime-blocks-on-its-own">
  ランタイムが独自にブロックするもの
</h3>

ランタイムは、設定なしで最高リスクの書き込みをブロックします。

* `denyWrite` は `allowWrite` より優先されます。
* プロジェクトルートでは、ランタイムは `.git/hooks` を拒否し、`filesystem.allowGitConfig: true` を設定しない限り `.git/config` を拒否し、`.mcp.json`、`.claude/commands`、`.claude/agents`、およびシェルスタートアップファイルを拒否します。
* macOS では、これらの拒否は書き込みが発生したときにチェックされるため、ネストされたファイルとセッション中に作成されたリポジトリもカバーします。
* Linux と WSL2 では、ランタイムは起動時に拒否リストを構築します。プロジェクトルートを確実にカバーし、その時点で存在するネストされたコピーの最善の努力による浅いスキャンを行い、`git init`、`git clone`、またはスキャフォルディングなど、セッションが後で作成するものはカバーしません。README の `mandatoryDenySearchDepth` セクションはスキャンの正確なセマンティクスを説明しています。
* 有効な `~/.srt-settings.json` がない場合、ランタイムは起動しますが、ネットワークアクセスをブロックし、書き込みを `/tmp/claude`、`~/.npm/_logs`、`~/.claude/debug` などの組み込みランタイムパスに限定します。クリーンスタートを設定が読み込まれた証拠として受け取らないでください。
* `--settings` を渡すと、ファイルの読み込みに失敗した場合、ランタイムは起動を拒否します。

書き込み許可には、Claude Code が設定を読み込む他のパスも含まれるため、`denyWrite` でそれらを拒否してください。それらに書き込みできるサンドボックス化されたセッションは、次に Claude Code を起動するときに、サンドボックス化されていない hook、権限ルール、または MCP サーバーを永続化できます。

<h3 id="after-unattended-runs">
  無人実行後
</h3>

保持した書き込み可能なパスを確認してください。Linux と WSL2 では、セッションが作成したものも確認してください。

<h2 id="dev-containers">
  Dev コンテナ
</h2>

Dev コンテナは Claude Code を Docker コンテナ内で実行します。VS Code または互換性のあるエディターが管理し、プロジェクトがマウントされます。リポジトリの `.devcontainer/` ディレクトリで独自に定義できます。

claude-code リポジトリは、デフォルト拒否 iptables ファイアウォールを備えた [example dev container](/docs/ja/devcontainer) を出発点として公開しています。リポジトリにコピーし、ファイアウォール許可リスト、ベースイメージ、ピン留めされた Claude Code バージョンを環境に合わせて調整します。ファイアウォールが未承認の出力をブロックするため、このような設定は無人作業のために `--dangerously-skip-permissions` で Claude Code を実行することをサポートしています。

<h2 id="custom-container">
  カスタムコンテナ
</h2>

Claude Code は、独自のネットワークポリシー、マウントされたボリューム、seccomp プロファイルを備えた任意の Docker または OCI コンテナイメージで実行できます。これは、既存のコンテナインフラストラクチャまたは CI ランナーを持つ組織にとって最も一般的なパスです。

複数のマネージドサンドボックスおよびリモート実行サービスがコンテナをホストできます。操作するコンテナと同じチェックリストが適用されます。書き込み可能にマウントされているもの、その内部で到達可能な認証情報とトークン、ネットワーク出力ポリシーが許可するものを確認します。

コマンド単位の制限のためにコンテナ内に組み込みの Bash サンドボックスを重ねることができます。特権のないコンテナには、[サンドボックス化のトラブルシューティング](/docs/ja/sandboxing#troubleshooting)で説明されている nested-sandbox 設定が必要です。

<h2 id="virtual-machine">
  仮想マシン
</h2>

専用の仮想マシンは、独自のカーネルと、クラウドまたは microVM デプロイメントでは独自の仮想化ハードウェアを備えた最強の分離を提供します。オプションには、クラウドインスタンス、ローカルハイパーバイザー、Firecracker などの microVM が含まれます。信頼できないコードを評価する場合、セキュリティポリシーがエージェントとホスト間のカーネルレベルの分離を要求する場合、またはホストレベルのアプローチがコンプライアンス要件を満たさない場合に、このアプローチを使用します。

[Docker Sandboxes](https://docs.docker.com/ai/sandboxes/) は、独自の Docker デーモンとワークスペース同期を備えた microVM を提供し、Docker Sandboxes がインストールされているホストで Claude Code を実行できます。これは Docker の無料のスタンドアロン製品であり、Docker Desktop は必要ありません。

<h2 id="cloud-sessions">
  クラウドセッション
</h2>

[クラウドセッション](/docs/ja/claude-code-on-the-web)は、分離された Anthropic 管理の仮想マシンで実行されます。ネットワークプロキシはデフォルト許可リストを強制し、別のプロキシはサンドボックス内のリポジトリアクセスのためにスコープ付き認証情報を発行しながら、GitHub トークンをサンドボックスの外に保持します。組織が[セルフホスト環境](/docs/ja/self-hosted-environments)にルーティングするセッションは、代わりにユーザーがプロビジョニングするインフラストラクチャ上で実行され、分離、エグレス制御、および git 認証情報はデプロイメントの責任です。

インフラストラクチャを自分でプロビジョニングせずに完全な VM 分離が必要な場合、またはローカル開発環境がないデバイスからタスクを委任する場合に、このアプローチを使用します。Claude サブスクリプションが必要です。CLI から起動しない限り、サンドボックスがリポジトリをクローンできるように、接続された GitHub アカウントも必要です。`--cloud`を使用して CLI から起動する場合、Claude Code は代わりに[ローカルリポジトリをバンドルしてアップロード](/docs/ja/claude-code-on-the-web#send-local-repositories-without-github)できます。プラン可用性と GitHub 認証オプションについては、[クラウドで Claude Code を使用する](/docs/ja/claude-code-on-the-web)を参照してください。

<h2 id="enforce-isolation-across-an-organization">
  組織全体で分離を強制する
</h2>

個々の開発者は、このページのいずれかのサンドボックス化アプローチにオプトインできます。組織が強制できるもの、およびどのツールで強制できるかは、アプローチによって異なります。

* **組み込み Bash サンドボックス**：Claude Code が自体で強制する唯一のアプローチです。[管理設定](/docs/ja/managed-settings#delivery-mechanisms)を通じて `sandbox` 設定キーを配信します。MDM で管理されるファイルとして、または Claude.ai の[サーバー管理設定](/docs/ja/server-managed-settings)を通じて。デプロイするキーと開発者がポリシーを拡大するのを防ぐ方法については、[管理設定でサンドボックス化を強制](/docs/ja/sandboxing#enforce-sandboxing-with-managed-settings)を参照してください。
* **Dev コンテナ**：[example dev container](/docs/ja/devcontainer)をリポジトリにコミットして、チーム全体で環境を標準化します。Claude Code がコンテナを要求しないため、これは強制境界ではなく慣例です。開発者が Claude Code をその外で実行できないようにする場合は、組織のデバイス管理またはソフトウェア許可リストツールでそれを強制します。
* **カスタムコンテナと VM**：承認されたイメージを通じて Claude Code を配布し、組織のデバイス管理またはソフトウェア許可リストツールを使用して、その外でのインストールを防止します。

<h2 id="see-also">
  関連項目
</h2>

これらのページでは、このページで説明しているサンドボックス化アプローチの設定とポリシーの詳細について説明しています。

* [サンドボックス化](/docs/ja/sandboxing)：組み込みのサンドボックス化された Bash ツールを設定します
* [Dev コンテナ](/docs/ja/devcontainer)：事前設定された Docker 開発コンテナ
* [セキュリティ](/docs/ja/security)：完全な Claude Code セキュリティモデル
* [セキュアなデプロイメント](/docs/ja/agent-sdk/secure-deployment)：Agent SDK アプリケーションの分離ガイダンス
* [設定](/docs/ja/settings-reference#sandbox-settings)：管理設定配信を含むすべてのサンドボックス設定キー
