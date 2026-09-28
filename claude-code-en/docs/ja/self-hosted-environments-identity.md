> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 自己ホスト環境でセッション ID を検証する

> CLAUDE_CODE_SESSION_ACCESS_TOKEN JWT を検証して、自己ホスト環境内のセッションからのリクエストをネットワーク上のサービスが信頼できるようにします。

<Note>
  自己ホスト環境は Team および Enterprise プランでパブリックベータ版です。[オーナー](/docs/ja/cloud-environments#organization-shared-environments)が [**Cloud environments** 管理ページ](https://claude.ai/admin-settings/cloud-environments)で **Allow self-hosted environments** をオンにすることで有効になります。このページではセッション ID 検証について説明します。セットアップについては[クイックスタート](/docs/ja/self-hosted-environments-quickstart)を、フリート構成については[本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy)を参照してください。
</Note>

[自己ホスト環境](/docs/ja/self-hosted-environments)では、[ウェブ上の Claude Code](/docs/ja/claude-code-on-the-web) セッションが Anthropic のインフラストラクチャではなく、お客様が運用するインフラストラクチャ上で実行されます。セッションはお客様のネットワーク内で実行されるため、Claude はお客様の内部サービスを直接呼び出すことができます。これらのサービスは、リクエストが環境内の Claude Code セッションから来たことを確認し、そのセッションを作成したユーザーまたはサービス ID を識別する方法が必要です。

自己ホスト環境内のすべてのセッションは、`CLAUDE_CODE_SESSION_ACCESS_TOKEN` 環境変数に署名付き JSON Web Token（JWT）を受け取ります。セッションはトークンをベアラー認証情報として提示します。たとえば、Claude が実行するスクリプトは `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"` を使用してお客様のサービスを呼び出すことができます。Anthropic はトークンに署名し、検証キーを公開 JWKS エンドポイントで公開します。お客様のサービスはこれらのキーを取得し、署名を検証し、クレームを読んでアクセス権を決定します。

<h2 id="the-session-token">
  セッショントークン
</h2>

検証コードを作成する前に、トークンが何を確立するか、および JWT ライブラリが見る形式を理解してください。

<h3 id="what-the-token-proves">
  トークンが証明すること
</h3>

有効なトークンは一部の事実を確立し、意図的に他の事実は確立しません。

* **証明すること**: Anthropic が特定の環境内の特定のセッション用にトークンを発行したこと、およびセッションがどのように作成されたか。組織内のユーザーによって、または組織のサービス ID によって（[Claude Tag チャネルセッション](https://claude.com/docs/claude-tag/concepts/agent-identity)の開始方法）
* **証明しないこと**: ランナーホスト上のどのプロセスがそれを提示するか。トークンはセッション内の環境変数に存在するため、Claude が実行するコード、およびセッションが開始するツールまたは MCP サーバーは、それを読んで提示できます。

お客様のサービスに対する 2 つの結果：

* `aud` クレームを環境 ID（[**Cloud environments** 管理ページ](https://claude.ai/admin-settings/cloud-environments)で環境と共に表示される `ccpool_...` 値）に対して検証し、他の組織の環境に発行されたトークンを拒否します。
* トークンから派生させた認証情報を、単一のコーディングセッションが実行できることにスコープします。セッションの作成者ができるすべてのことではなく。[派生認証情報のスコープ](#scope-derived-credentials)を参照してください。

<h3 id="token-format">
  トークン形式
</h3>

`CLAUDE_CODE_SESSION_ACCESS_TOKEN` の値は `sk-ant-cc-` プレフィックスの後に標準的な 3 部構成の JWT が続きます。

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

JWT ライブラリに値を渡す前にプレフィックスを削除します。Anthropic ホスト型クラウドセッションに発行されたトークンは代わりに `sk-ant-si-` プレフィックスを持ち、異なるキーセットで署名されているため、`sk-ant-cc-` で始まらない値は拒否します。

署名アルゴリズムは `ES256` で、これは P-256 曲線上の ECDSA と SHA-256 です。トークンヘッダーは、それに署名した JWKS 内のキーを識別する `kid` を持ちます。

<h2 id="verify-the-token">
  トークンを検証する
</h2>

検証は 2 つの場所のいずれかで実行されます。ネットワーク上のサービスは Anthropic の公開キーに対してトークンを暗号的に検証し、セッション内のラッパースクリプトはランナーバイナリの組み込みデコーダーを代わりに使用できます。

<h3 id="verify-the-token-from-your-service">
  サービスからトークンを検証する
</h3>

Anthropic は検証キーを公開の認証なしエンドポイントで公開します。

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

レスポンスは標準的な [JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517) です。Anthropic は署名キーを定期的にローテーションし、ローテーション前のキーはセットに十分な期間残り、それらが署名したトークンが検証を続けるため、単一のキーをピンしないでください。エンドポイントは `Cache-Control: public, max-age=300` を設定するため、キーセットをキャッシュして 5 分ごとに再取得することは安全です。

これらのチェックに対して各受信トークンを検証します。

<Steps>
  <Step title="プレフィックスを確認する">
    値が `sk-ant-cc-` で始まらない場合は拒否し、そのプレフィックスを削除します。残りは標準的なコンパクト JWT です。
  </Step>

  <Step title="署名を検証する">
    JWKS を取得し、トークンヘッダーの `kid` と一致するキーを選択し、`ES256` 署名を検証します。`alg` ヘッダーが `ES256` でないトークンを拒否します。キャッシュされたキーセットにない `kid` を持つトークンが到着した場合、拒否する前に JWKS を 1 回再取得します。ローテーション後、新しいトークンはキャッシュされたセットにまだないキーで署名されます。
  </Step>

  <Step title="発行者を検証する">
    `iss` が正確に `ccr` でない場合、トークンを拒否します。
  </Step>

  <Step title="環境に対してオーディエンスを検証する">
    `aud` クレームは配列です。環境 ID（`ccpool_...` の形式）を含まない限り、トークンを拒否します。環境 ID は [**Cloud environments** 管理ページ](https://claude.ai/admin-settings/cloud-environments)の環境の詳細ダイアログに表示され、環境のセッショントークンのいずれかに `ccr:pool_id` クレームとして表示されます。このチェックはトークンを環境にスコープし、他の組織に発行されたトークンを拒否するものです。
  </Step>

  <Step title="ロールを検証する">
    `ccr:role` が正確に `session_worker` でない場合、トークンを拒否します。環境シークレット、ランナートークン、ワークオーダーなど、自己ホスト環境用に発行された他のトークンは同じキーセットで署名されていますが、異なるロールを持ちます。
  </Step>

  <Step title="有効期限を検証する">
    `exp` が過去の場合、トークンを拒否します。Anthropic はセッショントークンをデフォルトで 4 時間の有効期限、最大 8 時間で発行します。ランナーは有効期限前にトークンを更新し、新しい値をセッションにプッシュするため、Claude が更新後に開始するサブプロセスはそれを継承します。したがって、1 つのセッションは有効期間中にお客様のサービスに複数の異なる有効なトークンを提示できます。
  </Step>

  <Step title="ID を読む">
    作成ユーザーの ID は `act` クレームにあります。`act.sub` はプレフィックス形式 `user:<id>` の Anthropic ユーザー ID で、`act.email` は作成サーフェスが記録した場合、メールアドレスです。組織のサービス ID が作成するセッション（Claude Tag チャネルセッションを含む）は代わりに `agent:` サブジェクトを持つため、`act.sub` が `user:` プレフィックスを持つ場合にのみセッションをユーザー作成として扱い、ID クレームが存在しないかどうかをテストするのではなく。完全な構造とフラット重複クレームについては、[クレームリファレンス](#claims-reference)を参照してください。
  </Step>
</Steps>

チェックは標準 JWT ライブラリに直接マップされます。以下の例は、JWKS フェッチ、キャッシング、および `kid` 選択を処理する [`jose`](https://www.npmjs.com/package/jose) を使用した Node.js での完全なシーケンスと、[`PyJWT`](https://pyjwt.readthedocs.io/) とその組み込み JWKS クライアントを使用した Python で実装しています。

<Tabs>
  <Tab title="Node.js (jose)">
    ```typescript theme={null}
    import { createRemoteJWKSet, jwtVerify } from "jose";

    const JWKS = createRemoteJWKSet(
      new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
    );

    const PREFIX = "sk-ant-cc-";
    const EXPECTED_POOL_ID = "ccpool_...";

    export async function verifySessionToken(raw: string) {
      if (!raw.startsWith(PREFIX)) {
        throw new Error("not a self-hosted runner session token");
      }
      const jwt = raw.slice(PREFIX.length);

      const { payload } = await jwtVerify(jwt, JWKS, {
        issuer: "ccr",
        audience: EXPECTED_POOL_ID,
        algorithms: ["ES256"],
      });

      if (payload["ccr:role"] !== "session_worker") {
        throw new Error("token is not a session_worker token");
      }

      const act = payload.act as { email?: string; sub?: string };
      return {
        sessionId: payload["ccr:session_id"] as string,
        poolId: payload["ccr:pool_id"] as string,
        orgId: payload["ccr:org_id"] as string,
        creatorEmail: act?.email,
        creatorSub: act?.sub,
      };
    }
    ```
  </Tab>

  <Tab title="Python (PyJWT)">
    ```python theme={null}
    import jwt
    from jwt import PyJWKClient

    JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
    PREFIX = "sk-ant-cc-"
    EXPECTED_POOL_ID = "ccpool_..."

    jwks = PyJWKClient(JWKS_URL)


    def verify_session_token(raw: str) -> dict:
        if not raw.startswith(PREFIX):
            raise ValueError("not a self-hosted runner session token")
        token = raw.removeprefix(PREFIX)

        signing_key = jwks.get_signing_key_from_jwt(token)
        payload = jwt.decode(
            token,
            signing_key.key,
            algorithms=["ES256"],
            issuer="ccr",
            audience=EXPECTED_POOL_ID,
        )

        if payload.get("ccr:role") != "session_worker":
            raise ValueError("token is not a session_worker token")

        act = payload.get("act") or {}
        return {
            "session_id": payload["ccr:session_id"],
            "pool_id": payload["ccr:pool_id"],
            "org_id": payload["ccr:org_id"],
            "creator_email": act.get("email"),
            "creator_sub": act.get("sub"),
        }
    ```
  </Tab>
</Tabs>

<h3 id="verify-the-token-inside-the-session">
  セッション内でトークンを検証する
</h3>

[ラッパースクリプト](/docs/ja/self-hosted-environments-configuration#wrapper-scripts)はセッション内で Claude が開始する前に実行されます。JWT ライブラリを呼び出す代わりに、ランナーバイナリの `self-hosted-runner decode-token` サブコマンドを実行できます。サブコマンドは位置引数、`CLAUDE_CODE_SESSION_ACCESS_TOKEN`、またはパイプされた stdin からトークンを読み取ります（この順序で）。その後、プレフィックスを削除し、JWKS エンドポイントに対して署名を検証し、有効期限をチェックし、クレームを JSON として出力します。サブコマンドは署名と有効期限チェックのみを実行します。`iss`、`aud`、または `ccr:role` はチェックしません。ラッパーの認証決定がこれらのクレームに依存する場合、出力された JSON からそれらを読み取り、明示的に比較します。

このコマンドは作成者 ID を抽出し、SSO プロバイダーのサブジェクト、メールアドレス、作成者の `act.sub` サブジェクト（`user:<id>` または `agent:<id>`）の順で優先します。

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.attested_by.sub // .act.email // .act.sub'
```

ラッパーはランナー自身のバイナリへの絶対パスを `CLAUDE_RUNNER_CLAUDE_BIN` で受け取ります。PATH で解決された `claude` ではなく、そのパスを使用して、デコードがランナー自身が使用するのと同じバイナリで実行されるようにします。

`jq -r` ではなく `jq -re` を使用して、クレームが見つからない場合は 0 以外の終了コードが発生するようにします。`-r` だけでは、クレームが見つからない場合、リテラル文字列 `null` を出力して 0 で終了し、不正な値を静かに下流に渡します。JWKS エンドポイントに到達できないオフライン検査の場合のみ、`decode-token` に `--no-verify` を渡します。

<h2 id="claims-reference">
  クレームリファレンス
</h2>

以下の表は、検証に関連するセッショントークンクレームをリストしています。`ccr:*` 名前空間と `act` チェーンから ID を読み取ります。フラット `account_email`、`organization_uuid`、および `account_uuid` クレームは、削除される可能性のある後方互換性の重複です。組織のサービス ID が作成するセッション（Claude Tag チャネルセッションを含む）は、`act.sub` に `agent:` サブジェクトを持ち、`act.email`、`ccr:account_id`、`account_email`、および `account_uuid` を省略します。2 つのメールクレームはユーザー作成セッションでもオプションです。Anthropic はセッション作成時にそれらを記録するのは、作成リクエストの認証情報がメールを持つ場合のみで、CLI からディスパッチされたセッションは両方を欠く可能性があるため、メールではなく `act.sub` または `ccr:account_id` で ID をキーにします。トークンはこのテーブルを超えて追加のクレームを持つこともできます。認識しないクレームは無視します。

| クレーム                | 型      | 説明                                                                                                                                                                                                                                                                                                                         |
| :------------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `iss`               | 文字列    | 常に `ccr`。                                                                                                                                                                                                                                                                                                                  |
| `sub`               | 文字列    | `ccr:session:<session_id>`。                                                                                                                                                                                                                                                                                                |
| `aud`               | 文字列の配列 | 常に `anthropic-api` を含みます。自己ホスト環境内のセッションの場合、配列は `ccpool_...` などの環境 ID も含みます。`anthropic-api` ではなく環境 ID を検証します。                                                                                                                                                                                                               |
| `exp`               | 数値     | Unix タイムスタンプとしての有効期限。4 時間のデフォルト有効期限、8 時間の最大値。                                                                                                                                                                                                                                                                              |
| `iat`               | 数値     | Unix タイムスタンプとして発行された時刻。                                                                                                                                                                                                                                                                                                    |
| `jti`               | 文字列    | 一意のトークン識別子。                                                                                                                                                                                                                                                                                                                |
| `ccr:role`          | 文字列    | セッショントークンの場合、常に `session_worker`。                                                                                                                                                                                                                                                                                          |
| `ccr:session_id`    | 文字列    | セッション ID。`sub` のサフィックスと同じ値。                                                                                                                                                                                                                                                                                                |
| `ccr:pool_id`       | 文字列    | 環境 ID。`aud` に表示される同じ値。                                                                                                                                                                                                                                                                                                     |
| `ccr:org_id`        | 文字列    | Anthropic 組織 ID。                                                                                                                                                                                                                                                                                                           |
| `ccr:account_id`    | 文字列    | 作成ユーザーの Anthropic アカウント ID。`act.sub` の値から `user:` プレフィックスを除いたもので、タグ付き `user_...` ID。[spawn-runner フック](/docs/ja/self-hosted-environments-configuration#the-spawn-runner-hook)の `CLAUDE_RUNNER_ACCOUNT_ID` が持つ値と同じで、[`--lock-to-account`](/docs/ja/self-hosted-environments-reference#runner-cli-flags) が受け入れるため、3 つは等しい文字列として比較されます。 |
| `account_email`     | 文字列    | `act.email` の重複。`act.email` がない場合は常に存在しません。                                                                                                                                                                                                                                                                                |
| `organization_uuid` | 文字列    | Anthropic 組織 UUID。                                                                                                                                                                                                                                                                                                         |
| `account_uuid`      | 文字列    | 作成ユーザーの Anthropic アカウント UUID。                                                                                                                                                                                                                                                                                              |
| `act`               | オブジェクト | [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693) 委任チェーン。[`act` チェーン](#the-act-chain)を参照してください。                                                                                                                                                                                                                           |

<h3 id="the-act-chain">
  `act` チェーン
</h3>

`act` クレームは、セッションを作成したユーザーまたはサービス ID から、ランナーを認めた[環境](/docs/ja/self-hosted-environments#key-concepts)のシークレット、およびそのシークレットを作成した ID までの完全な委任パスを記録します。作成者は最も外側のアクターであるため、`act.sub` は直接それらを識別します。

| パス                | 説明                                                                                                                                                   |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `act.sub`         | 作成ユーザーの Anthropic ユーザー ID（`user:<id>` の形式）、または組織のサービス ID がセッションを作成した場合は `agent:<id>`（Claude Tag チャネルセッションの場合）。                                       |
| `act.email`       | 作成ユーザーのメールアドレス（セッション作成時に記録された場合）。それを要求しないでください。`act.sub` でキーにします。                                                                                    |
| `act.attested_by` | 作成ユーザーのアップストリーム ID プロバイダーの証明（利用可能な場合）。`act.attested_by.sub` は Google や Okta などの SSO プロバイダーが発行したサブジェクトです。独自のシステムの ID にマップする場合、`act.email` よりこれを優先します。 |
| `act.act`         | セッションを生成したランナー。`act.act.sub` は `ccr:runner:<runner_id>`。                                                                                             |
| `act.act.act`     | 環境。`act.act.act.sub` は `ccr:pool:<pool_id>`。                                                                                                         |
| `act.act.act.act` | ランナーが登録した環境シークレットを作成した ID。チェーンはここで終わります。                                                                                                             |

<h2 id="scope-derived-credentials">
  派生認証情報のスコープ
</h2>

セッショントークンはセッションを作成したユーザーまたはサービス ID を識別しますが、それを作成者が直接ログインするのと同等として扱わないでください。トークンはセッション内の環境変数に存在するため、Claude が実行するコード、およびセッションが開始するツールまたは MCP サーバーは、それを読んで提示できます。

検証もオフラインです。JWKS に対して検証するトークンは、その `exp` まで有効なままで、セッションに何が起こったかに関係なく、Anthropic はセッショントークンの失効フィードを公開しません。トークンから派生させるものはそれに応じてバインドします。

サービスがトークンを内部認証情報と交換する場合、1 つのコーディングセッションが到達すべきことにスコープされた認証情報を発行します。

* **機能を制限する**: セッションがコーディングタスクに必要なリソースへの読み取りおよび書き込みアクセスを付与し、作成者が他の場所で保持する管理機能は付与しません。
* **有効期限を制限する**: 派生認証情報をトークンの `exp` またはそれより短い期間にバインドします。
* **セッションとして監査する**: `ccr:session_id` と `jti` を作成者 ID と共に記録して、アクションを特定のセッションにトレースバックできるようにします。

<h2 id="related-environment-variables">
  関連環境変数
</h2>

作成者 ID は、トークンを検証しない 2 つのサーフェスのプレーンテキスト環境変数にも表示されます。

* **[`spawn-runner` フック](/docs/ja/self-hosted-environments-configuration#the-spawn-runner-hook)（オーケストレーター上）**: フックはキューに入れられたセッションのランナーが存在する前に実行され、`CLAUDE_RUNNER_ACCOUNT_EMAIL` や `CLAUDE_RUNNER_ACCOUNT_ID` などの変数で作成者 ID を受け取ります。オーケストレーターはワークオーダー（1 つのランナーを生成することを認可する署名付き 1 回限りのトークン）からそれらを読み取り、ワークオーダーの署名自体を検証せずに。クレームは環境シークレットが認証するオーケストレーターの Anthropic への接続を介してワークオーダーが到着するため、信頼されます。
* **[ラッパースクリプト](/docs/ja/self-hosted-environments-configuration#wrapper-scripts)（セッション内）**: ラッパーは `CCR_SESSION_ACCOUNT_EMAIL` を受け取ります。これは作成者のメールで、署名検証なしでトークンから事前に抽出されたものです。変数は認証決定ではなく、コミットトレーラーなどのラベル付けに適しています。

オーケストレーター側の決定（マシンイメージの選択など）にはプレーンテキスト変数を使用します。ランナーの環境を信頼するのではなく、ダウンストリームサービスが独立した暗号化証明を必要とする場合は `CLAUDE_CODE_SESSION_ACCESS_TOKEN` を使用します。

<h2 id="what’s-next">
  次のステップ
</h2>

* [自己ホスト環境](/docs/ja/self-hosted-environments): 環境、ランナー、およびセッションモデル。[クイックスタート](/docs/ja/self-hosted-environments-quickstart)と[本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy)はセットアップと運用を保持しています。
* [セッションをカスタマイズする](/docs/ja/self-hosted-environments-configuration): トークンを使用するラッパースクリプト、および `spawn-runner` フック
* [リファレンス](/docs/ja/self-hosted-environments-reference): CLI フラグ、環境変数、およびメトリクス
