> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# セッションを外部ストレージに永続化する

> Agent SDK セッションのトランスクリプトを独自のオブジェクトストア、キーバリューストア、またはデータベースにミラーリングして、他のホストがセッションを再開できるようにします。

デフォルトでは、SDK はセッションのトランスクリプトを `~/.claude/projects/` 下のローカルファイルシステム上の JSONL ファイルに書き込みます。`SessionStore` アダプターを使用すると、これらのトランスクリプトをオブジェクトストア、キーバリューストア、またはデータベースなどの独自のバックエンドにミラーリングできます。これにより、あるホストで作成されたセッションを、同じ作業ディレクトリから実行している別のホストで再開できます。

セッションストアを使用する一般的な理由：

* **マルチホスト展開。** サーバーレス関数、オートスケーリングワーカー、CI ランナーはファイルシステムを共有しません。共有ストアにより、レプリカは互いのセッションを再開できます。
* **耐久性。** ローカルコンテナーは一時的です。外部ストアは再起動と再デプロイを乗り越えて存続します。
* **コンプライアンスと監査。** トランスクリプトを既に管理しているストレージに保持し、独自の保持ルール、暗号化、アクセス制御を適用できます。

<h2 id="the-sessionstore-interface">
  `SessionStore` インターフェース
</h2>

`SessionStore` は、2 つの必須メソッド `append` と `load`、および 4 つのオプションメソッドを持つオブジェクトです。SDK は `append` を呼び出してクエリ中にトランスクリプトエントリを書き込み、`load` を呼び出して再開用にそれらを読み戻します。

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

  type SessionKey = {
    projectKey: string;
    sessionId: string;
    subpath?: string;
  };

  type SessionStore = {
    // Required
    append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
    load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

    // Optional
    listSessions?(
      projectKey: string,
    ): Promise<Array<{ sessionId: string; mtime: number }>>;
    listSessionSummaries?(projectKey: string): Promise<SessionSummaryEntry[]>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };

  type SessionSummaryEntry = {
    sessionId: string;
    mtime: number;
    data: Record<string, unknown>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

  class SessionKey(TypedDict):
      project_key: str
      session_id: str
      subpath: NotRequired[str]

  class SessionStore(Protocol):
      # Required
      async def append(
          self, key: SessionKey, entries: list[SessionStoreEntry]
      ) -> None: ...
      async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

      # Optional — omit or raise NotImplementedError
      async def list_sessions(
          self, project_key: str
      ) -> list[SessionStoreListEntry]: ...
      async def list_session_summaries(
          self, project_key: str
      ) -> list[SessionSummaryEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...

  class SessionSummaryEntry(TypedDict):
      session_id: str
      mtime: int
      data: dict[str, Any]
  ```
</CodeGroup>

`SessionKey` は 1 つのトランスクリプトをアドレス指定します。`projectKey` は作業ディレクトリの安定したファイルシステム安全なエンコーディング、`sessionId` はセッション UUID、`subpath` はエントリがサブエージェントトランスクリプトまたはサイドカーファイルではなくメイン会話に属する場合に設定されます。

`projectKey` は作業ディレクトリをエンコードするため、ストアから再開または続行する場合は、元の実行の作業ディレクトリと一致する作業ディレクトリから行ってください。TypeScript では、クエリの [`env` オプション](/docs/ja/agent-sdk/typescript#options)で `CLAUDE_CONFIG_DIR` の横に [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ja/sessions#name-the-project-directory-yourself) を設定した場合、SDK はそのクエリのエントリ、および `resume` と `continue` ルックアップをその名前でキーイングします。`listSessions` や `deleteSession` などのスタンドアロンヘルパーは `env` を取らず、プロセス環境を読むため、ホストプロセス環境でも `CLAUDE_CONFIG_DIR` と同じ名前を設定してください。Agent SDK v0.3.234 以降が必要です。

`subpath` を不透明なキーサフィックスとして扱います。これはオンディスクレイアウトに従います。例えば `subagents/agent-<id>` です。`subpath` が未定義の場合、キーはメイントランスクリプトを参照します。

| メソッド                   | 必須  | 呼び出されるタイミング                                                                                                                                                                                                            |
| :--------------------- | :-- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | はい  | トランスクリプトエントリの各バッチがローカルに書き込まれた後。エントリは JSON セーフなオブジェクトで、ローカル JSONL では 1 行に 1 つです。                                                                                                                                        |
| `load`                 | はい  | `resume` が設定されている場合、またはサブプロセスが生成される前に `continue: true` が最新のストアセッションを解決する場合、およびリスティングが `listSessionSummaries` からフォールバックする場合は 1 セッションあたり 1 回。セッションが不明な場合は `null` を返します。                                                  |
| `listSessions`         | いいえ | `listSessions({ sessionStore })` および `continue: true` を指定した `query()`/`startup()` によって。未定義の場合、`continue: true` はスロー、`listSessions({ sessionStore })` は `listSessionSummaries` が実装されていない限りスロー。                          |
| `listSessionSummaries` | いいえ | `listSessions({ sessionStore })` によって、1 回の呼び出しですべてのセッションのメタデータを読み込むため。`append` 内でサマリーを保持します。未定義の場合、リスティングは `listSessions` プラスセッションごとの `load` にフォールバック。                                                                 |
| `delete`               | いいえ | `deleteSession({ sessionStore })` によって。メインキー（`subpath` なし）を削除する場合、そのセッションのすべてのサブキーにカスケードし、セッションのサマリーエントリも削除する必要があります。これにより、削除されたセッションは `listSessionSummaries` に表示されなくなります。未定義の場合、削除は no-op です。これはアペンドのみのバックエンドに適しています。 |
| `listSubkeys`          | いいえ | 再開中に、サブエージェントトランスクリプトを検出するため。未定義の場合、メイントランスクリプトのみが復元されます。                                                                                                                                                              |

`SessionSummaryEntry` では、`mtime` はサイドカーのストレージ書き込み時刻であり、`listSessions` が返す `mtime` 値と同じクロックソースを共有する必要があります。`data` は不透明な SDK 所有の状態です。それを解釈せずに逐語的に永続化します。

エントリを構築するには、`append` 内の各バッチで、エクスポートされた `foldSessionSummary` ヘルパー（Python では `fold_session_summary`）を呼び出します。`subpath` を持つキーのバッチはスキップします。サブエージェントトランスクリプトはメインセッションのサマリーに貢献してはいけません。フォールドは `mtime` を設定しません。TypeScript では `options.mtime` 引数を通じて、または Python では返されたエントリのフィールドを上書きして、永続化時にスタンプします。同じセッションに対する同時 `append` 呼び出しはサイドカーで競合する可能性があるため、トランザクション、compare-and-swap、またはセッションごとのロックで読み取り-フォールド-書き込みをシリアル化します。フォール自体は純粋です。

SDK がトランスクリプト `load` が返すもので何をするかについては、[ストアから再開](#resume-from-the-store)を参照してください。

<h2 id="quick-start">
  クイックスタート
</h2>

SDK は開発とテスト用に `InMemorySessionStore` を提供しています。以下の例は、ストアを接続してクエリを実行し、結果メッセージからセッション ID をキャプチャしてから、2 番目の `query()` 呼び出しでストアから再開します。2 番目の呼び出しは同じストアインスタンスと `resume` を渡すため、SDK はローカルファイルシステムではなくストアからトランスクリプトを読み込みます：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  try {
    for await (const message of query({
      prompt: "List the TypeScript files under src/",
      options: { sessionStore: store },
    })) {
      if (message.type === "result") {
        sessionId = message.session_id;
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, sessionId was already captured by the loop
    // above; connection or process failures yield no result message.
    console.error(`Session ended with an error: ${error}`);
  }

  // Resume from the store. The agent has full context from the first call.
  for await (const message of query({
    prompt: "Summarize what those files do",
    options: { sessionStore: store, resume: sessionId },
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeAgentOptions,
      InMemorySessionStore,
      ResultMessage,
      query,
  )

  store = InMemorySessionStore()


  async def main():
      session_id = None
      try:
          async for message in query(
              prompt="List the Python files under src/",
              options=ClaudeAgentOptions(session_store=store),
          ):
              if isinstance(message, ResultMessage):
                  session_id = message.session_id
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, session_id was already captured by the
          # loop above; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")

      # Resume from the store. The agent has full context from the first call.
      async for message in query(
          prompt="Summarize what those files do",
          options=ClaudeAgentOptions(session_store=store, resume=session_id),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

2 番目のクエリは最初のクエリのファイルの概要を出力し、エージェントがストアから完全なコンテキストで再開されたことを示しています。

<h2 id="write-your-own-adapter">
  独自のアダプターを作成する
</h2>

バックエンドに対して `append` と `load` を実装します。ストアに対して `listSessions()`、ワンコール メタデータ読み取り、`deleteSession()`、およびサブエージェント再開を機能させたい場合は、`listSessions`、`listSessionSummaries`、`delete`、および `listSubkeys` を追加します。

`append` に渡されるエントリは `SessionStoreEntry` として型付けされます（`{ type: string; ... }` オブジェクト）。これらを不透明な JSON セーフ値として扱います。順序を保持して永続化し、`load` から同じ順序で返します。`load` は追加されたものと深く等しいエントリを返す必要があります。バイト等しいシリアル化は必須ではないため、バイナリ JSON カラム型のようなオブジェクトキーを並べ替えるバックエンドは問題ありません。

<h2 id="reference-implementations">
  リファレンス実装
</h2>

両方の SDK リポジトリには、TypeScript の [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) と Python の [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) の下に実行可能なリファレンスアダプターが含まれています。ストレージタイプごとに 1 つのアダプターがあり、各アダプターは `append` と `load` がそのバックエンドの種類にどのようにマップされるかを示しています。これらはパッケージとして公開されていません。バックエンドに最も近いタイプのアダプターをプロジェクトにコピーし、バックエンドのクライアントをインストールして、それを適応させてください。

| ストレージタイプ                  | ストレージモデル                                                                           | サンプルアダプター                                                                                                                                                                                                                                                 |
| :------------------------ | :--------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| オブジェクトストア                 | `append()` ごとに 1 つのパートファイル。`load()` はパートをリストアップ、ソート、連結します。                         | S3 （[TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)、[Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py)）                   |
| キーバリューストア                 | トランスクリプトごとに 1 つのリスト。`append()` がプッシュし、`load()` が範囲で読み込み、さらにセッションのソート済みインデックスがあります。 | Redis （[TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)、[Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py)）          |
| リレーショナルデータベースまたはドキュメントストア | エントリごとに 1 行またはドキュメント。JSON として保存され、挿入時に割り当てられたキーで順序付けされます。                          | Postgres （[TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres)、[Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)） |

各アダプターは事前に設定されたクライアントインスタンスを取得するため、認証情報、TLS、リージョン、プーリングを制御できます。以下の例は、オブジェクトストアアダプターを `query()` に接続し、別のホストからそれを再開する方法を示しています。

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

<h3 id="validate-your-adapter">
  アダプターを検証する
</h3>

両方の SDK は、`append`、`load`、およびオプションメソッドが満たす必要がある動作契約をアサートする適合スイートを提供しています。オプションメソッドのテストは、これらのメソッドが実装されていない場合、自動的にスキップされます。

TypeScript では、例ディレクトリから [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) をテストスイートにコピーしてください。Python では、スイートはパッケージに含まれています。pytest で実行するには、まず pytest をインストールしてください。pytest は SDK の依存関係ではありません：

```bash theme={null}
pip install pytest
```

その後、テストファイルでアダプターをスイートに渡します。引数なしのファクトリとして渡してください。`run_session_store_conformance` は契約ごとに 1 回呼び出して、新しいストアを構築します：

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

この例のように `MyRedisStore` クラス自体を渡すことは、コンストラクターが引数を取らない場合に機能します。事前に設定されたクライアントを取得するアダプターの場合は、代わりにストアを構築するラムダを渡してください。契約は同じセッションキーを再利用するため、ファクトリが返す各ストアは空のストレージで開始する必要があります。ラムダが呼び出しごとに分離されたバッキングストレージをプロビジョニングするようにしてください。例えば、新しいメモリ内フェイク、一意のキープレフィックス、または新しいテストデータベースなどです。

<h2 id="behavior-notes">
  動作に関する注記
</h2>

<h3 id="dual-write-architecture">
  デュアルライトアーキテクチャ
</h3>

Claude Code サブプロセスは常にトランスクリプトエントリの各バッチをローカルディスクに最初に書き込み、その後 SDK がその同じバッチをストアの `append()` に転送するため、ストアはローカルトランスクリプトのミラーであり、置き換えではありません。どちらのコピーが実行を超えて存続するかは、実行がどのように開始されたかによって異なります。

* **新規セッション、またはストアがセッションに何も持っていない場合の再開**：設定ディレクトリの下のローカルトランスクリプトが実行を超えて存続し、ストアはコピーを受け取ります。
* **[ストアから再開](#resume-from-the-store)**：ローカルコピーは実行終了時に削除されるため、ストアが唯一の耐久性のあるコピーを保持します。

新規セッションがローカルディスクにトランスクリプトを残さないようにしたい場合は、`options.env` の一時ディレクトリに `CLAUDE_CONFIG_DIR` を設定します。ストアから再開した実行は既にローカルコピーを削除するため、そのような設定は必要ありません。TypeScript では、[`env` オプション](/docs/ja/agent-sdk/typescript#options)がサブプロセス環境を置き換えるため、`process.env` を `env` に展開します。

アプリが設定ディレクトリ内のファイル（OAuth 認証情報や user `settings.json` の `apiKeyHelper` など）を通じてサインインする場合は、これらのファイルを一時ディレクトリに最初にコピーするか、代わりに `env` で `ANTHROPIC_API_KEY` を設定します。そうしないと、実行は `Not logged in` で失敗します。

2 つのオプションはミラーと競合し、SDK はいずれかをストアと組み合わせた場合、スタートアップ時にスローします。

* **TypeScript の `persistSession: false`**：ミラーが構築されるローカル書き込みをオフにします。Python SDK には同等のオプションがありません。
* **ファイルチェックポイント**、TypeScript の `enableFileCheckpointing` または Python の `enable_file_checkpointing`：ファイルバックアップをローカルディスクに直接書き込み、SDK はそれらをストアにミラーリングしません。

<h3 id="resume-from-the-store">
  ストアから再開
</h3>

`resume` を渡すか、TypeScript で `continue: true` または Python で `continue_conversation=True` をストアと一緒に渡すと、SDK はサブプロセスを生成する前にストアにトランスクリプトを要求します。

* **`resume`**：SDK は渡したセッション ID のセッションを要求します。
* **`continue: true`** または **`continue_conversation=True`**：SDK はストアの最新セッションを要求します。

ストアがトランスクリプトを返すと、SDK はそれを一時的な設定ディレクトリに書き込み、`CLAUDE_CONFIG_DIR` がそこを指すようにサブプロセスを実行し、実行終了時にディレクトリを削除します。その実行が書き込むローカルトランスクリプトはそれと一緒に削除されます。これが、このパスでストアが唯一の耐久性のあるコピーを保持する理由です。

SDK は実際の設定ディレクトリからのファイルで一時ディレクトリもシードします。コピーされるものは言語によって異なります。

* **TypeScript**：認証情報、`.claude.json`、および user `settings.json`。`settings.json` から、一時的な設定ディレクトリの下で動作が悪いキーを削除します：`enabledPlugins`、`extraKnownMarketplaces`、その [`additionalMarketplaces`](/docs/ja/settings-reference#extraknownmarketplaces) エイリアス、およびファイルの `env` ブロック内の任意の `CLAUDE_CONFIG_DIR`。Agent SDK v0.3.232 より前は、SDK はエイリアスを削除しませんでした。設定で設定された認証（[`apiKeyHelper`](/docs/ja/settings-reference#apikeyhelper) など）は、ストアから再開するときに機能します。Agent SDK v0.3.222 より前は、TypeScript SDK は認証情報と `.claude.json` のみをコピーしました。
* **Python**：認証情報と `.claude.json` のみなので、user `settings.json` の `apiKeyHelper` を通じて認証するアプリは、ストアから再開するときに `Not logged in` で失敗します。管理またはプロジェクト設定の `apiKeyHelper` は、Claude Code が `CLAUDE_CONFIG_DIR` の影響を受けない場所からこれらのファイルを読み込むため、引き続き機能します。

ストアがセッションに何も持っていない場合、SDK は代わりに実際の設定ディレクトリの下で実行され、結果は渡したオプションによって異なります。

* **`resume`**：両方の SDK は ID をサブプロセスに渡します。サブプロセスはストアなしで `resume` が行うのと同じようにローカルトランスクリプトを再開します。
* **TypeScript の `continue: true`**：SDK は新規セッションを開始します。
* **Python の `continue_conversation=True`**：SDK は最新のローカルセッションから続行します。

<h3 id="mirror-writes-are-best-effort">
  ミラー書き込みはベストエフォート
</h3>

`append()` が拒否した場合、SDK は短いバックオフで最大 2 回まで再試行し、合計で最大 3 回の試行を行います。タイムアウトした呼び出しは再試行されません。元の呼び出しがまだ到達する可能性があるためです。バッチがまだ失敗した場合、SDK はエラーをログに記録し、`{ type: "system", subtype: "mirror_error" }` メッセージをイテレータに発行し、バッチを削除して、クエリを続行します。再試行されたバッチは既に到達したエントリを再配信できるため、`append()` 実装で `entry.uuid` によって重複排除してください。

ストア障害はエージェントを中断しません。サブプロセスはローカルに最初に書き込むためです。ストアデータ損失を検出する必要がある場合は `mirror_error` を監視してください。[ストアから再開](#resume-from-the-store)した実行では、削除されたバッチは実行終了後に存続するコピーがありません。

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` は圧縮後のチェーンを返す
</h3>

`getSessionMessages({ sessionStore })` は、エージェントが再開時に見るリンクされたメッセージチェーンを返します。自動圧縮後、以前のターンは概要に置き換えられるため、ストアが 503 個の生エントリを保持するセッションは `getSessionMessages` から 18 個のメッセージを返す場合があります。圧縮前のターンとメタデータエントリを含む完全な生履歴については、`store.load(key)` を直接呼び出します。

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` はバイトコピーではない
</h3>

`forkSession({ sessionStore })` はソースエントリを読み込み、すべての `sessionId` フィールドを書き直し、メッセージ UUID を再マップしてから、変換されたエントリを新しいキーの下に追加します。アダプターレベルのコピーまたは `CopyObject` ショートカットは、古いセッション ID を参照するトランスクリプトを生成するため、SDK はそれを使用しません。

<h3 id="subagent-transcripts">
  サブエージェントトランスクリプト
</h3>

サブエージェントトランスクリプトは `subpath: "subagents/agent-<id>"` の下にミラーリングされます。`listSubagents({ sessionStore })` はアダプターが `listSubkeys` を実装する必要があります。`getSubagentMessages({ sessionStore })` は利用可能な場合はそれを使用しますが、未定義の場合は直接サブパスにフォールバックします。再開も `listSubkeys` を呼び出してサブエージェントファイルを復元します。これがない場合、メイントランスクリプトのみが具体化されます。

<h3 id="retention">
  保持
</h3>

SDK は独自の判断でストアから削除することはありません。保持はアダプターの責任です。コンプライアンス要件に従って、バックエンドの有効期限またはライフサイクルメカニズムを使用するか、スケジュール済みクリーンアップを実行します。

`CLAUDE_CONFIG_DIR` の下のローカルトランスクリプトは `cleanupPeriodDays` 設定によって独立して掃除されます。[保持スイープルール](/docs/ja/claude-directory#cleaned-up-automatically)に従います。[ストアから再開](#resume-from-the-store)した実行はローカルトランスクリプトを残さないため、これらの実行ではストアの保持が唯一の保持です。

<h2 id="supported-on">
  サポート対象
</h2>

以下の TypeScript SDK 関数は `sessionStore` オプションを受け入れ、提供されている場合、ローカルファイルシステムではなくストアに対して動作します：

* [`query()`](/docs/ja/agent-sdk/typescript#query)
* [`startup()`](/docs/ja/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/ja/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/ja/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/ja/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/ja/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/ja/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/ja/agent-sdk/typescript)
* [`forkSession()`](/docs/ja/agent-sdk/typescript)
* [`listSubagents()`](/docs/ja/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/ja/agent-sdk/typescript)

Python SDK では、[`ClaudeAgentOptions`](/docs/ja/agent-sdk/python#claudeagentoptions) で `session_store` を設定して、ストアに対して `query()` を実行します。残りの操作には、ストアを引数として受け取るストア対応の Python 関数があります：`list_sessions_from_store()`、`get_session_info_from_store()`、`get_session_messages_from_store()`、`list_subagents_from_store()`、`get_subagent_messages_from_store()`、`rename_session_via_store()`、`tag_session_via_store()`、`delete_session_via_store()`、および `fork_session_via_store()`。`startup()` には Python の同等物がありません。[Python SDK リファレンス](/docs/ja/agent-sdk/python#functions) に記載されている `list_sessions()` などのスタンドアロン関数は、ローカルセッションファイルを読み取ります。

<h2 id="related-resources">
  関連リソース
</h2>

* [セッションを操作する](/docs/ja/agent-sdk/sessions)：カスタムストアなしで続行、再開、フォーク
* [SDK をホストする](/docs/ja/agent-sdk/hosting)：マルチホスト環境のデプロイメントパターン
* [TypeScript `Options`](/docs/ja/agent-sdk/typescript#options)：完全なオプションリファレンス
* [リファレンス実装](#reference-implementations)：オブジェクトストア、キーバリューストア、データベース用の実行可能なサンプルアダプター（両方の SDK リポジトリ内）
