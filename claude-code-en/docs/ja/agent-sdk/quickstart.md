> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# クイックスタート

> Python または TypeScript Agent SDK を使用して、自律的に動作する AI エージェントを構築する方法を学びます

Agent SDK を使用して、コードを読み、バグを見つけ、すべて手動操作なしで修正する AI エージェントを構築します。

**実行内容：**

1. Agent SDK でプロジェクトをセットアップする
2. バグのあるコードを含むファイルを作成する
3. バグを自動的に見つけて修正するエージェントを実行する

<h2 id="prerequisites">
  前提条件
</h2>

* **Node.js 18+** または **Python 3.10+**
* **Anthropic アカウント**。アカウントをお持ちでない場合は、[こちらでサインアップ](https://platform.claude.com/)してください。

<h2 id="setup">
  セットアップ
</h2>

<Steps>
  <Step title="プロジェクトフォルダを作成する">
    このクイックスタート用に新しいディレクトリを作成します：

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    独自のプロジェクトの場合、任意のフォルダから SDK を実行できます。デフォルトでは、そのディレクトリとそのサブディレクトリ内のファイルにアクセスできます。
  </Step>

  <Step title="SDK をインストールする">
    お使いの言語用の Agent SDK パッケージをインストールします：

    <Tabs>
      <Tab title="TypeScript（新規プロジェクト）">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        `package.json` で `"type": "module"` を設定すると、エージェントスクリプトでトップレベルの `await` を使用でき、[tsx](https://tsx.hirok.io) は TypeScript ファイルを直接実行します。npm はインストールが成功すると `added N packages` と出力します。
      </Tab>

      <Tab title="TypeScript（既存プロジェクト）">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.hirok.io) は TypeScript ファイルを直接実行します。プロジェクトが CommonJS を使用している場合は、エージェントスクリプトを `agent.ts` の代わりに `agent.mts` という名前にしてください。`.mts` 拡張子により、tsx はファイルを ES モジュールとして扱うため、プロジェクト全体を ES モジュールに変換することなく、トップレベルの `await` が機能します。このクイックスタートの後の作成と実行のステップで、`agent.ts` の代わりに `agent.mts` を使用してください。
      </Tab>

      <Tab title="Python（uv）">
        [uv](https://docs.astral.sh/uv/) をインストールしてください。これは仮想環境を自動的に処理する高速な Python パッケージマネージャーです。その後、プロジェクトを初期化して SDK を追加します：

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python（pip）">
        仮想環境を作成してアクティベートしてから、パッケージをインストールします。

        macOS または Linux の場合：

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Windows の場合：

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        PowerShell が実行ポリシーエラーで `Activate.ps1` をブロックする場合は、まず `Set-ExecutionPolicy -Scope Process RemoteSigned` を実行してください。
      </Tab>
    </Tabs>

    <Note>
      TypeScript SDK と Python SDK の両方は、ネイティブ Claude Code バイナリをバンドルしているため、ほとんどのインストールでは Claude Code を別途インストールする必要がありません。一部のインストールではバンドルされたバイナリがありません：

      * pip が Python SDK のソース配布をプラットフォームホイールの代わりにインストールする場合（たとえば ARM64 Windows）、バイナリはバンドルされません。[Claude Code をネイティブにインストール](/docs/ja/setup#install-claude-code)してください。Python SDK は `PATH` 上でそれを見つけます。
      * TypeScript SDK は npm オプション依存関係を通じてバイナリをインストールするため、それらをスキップするインストール（たとえば `npm ci --omit=optional`）は、サポートされているプラットフォームでもバイナリを取得しません。オプション依存関係をスキップせずに再インストールするか、[Claude Code をネイティブにインストール](/docs/ja/setup#install-claude-code)して `pathToClaudeCodeExecutable` をそのパスに設定してください。
    </Note>
  </Step>

  <Step title="API キーを設定する">
    [Claude Console](https://platform.claude.com/) から API キーを取得し、エージェントを実行するシェルで環境変数として設定します：

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows（PowerShell）">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    SDK はエージェントを実行するプロセスの環境からキーを読み取ります。`.env` ファイルを自動的に読み込みません。キーを `.env` ファイルに保持している場合は、SDK を呼び出す前に、たとえば `dotenv` パッケージを使用して自分で読み込んでください。

    SDK はサードパーティ API プロバイダーを介した認証もサポートしています：

    * **Amazon Bedrock**：`CLAUDE_CODE_USE_BEDROCK=1` 環境変数を設定し、AWS 認証情報を構成します
    * **Claude Platform on AWS**：`CLAUDE_CODE_USE_ANTHROPIC_AWS=1` と `ANTHROPIC_AWS_WORKSPACE_ID` を設定し、AWS 認証情報を構成します
    * **Google Cloud の Agent Platform**：`CLAUDE_CODE_USE_VERTEX=1` 環境変数を設定し、Google Cloud 認証情報を構成します
    * **Microsoft Foundry**：`CLAUDE_CODE_USE_FOUNDRY=1` 環境変数を設定し、Azure 認証情報を構成します

    詳細については、[Amazon Bedrock](/docs/ja/amazon-bedrock)、[Claude Platform on AWS](/docs/ja/claude-platform-on-aws)、[Google Cloud の Agent Platform](/docs/ja/google-vertex-ai)、または [Microsoft Foundry](/docs/ja/microsoft-foundry) のセットアップガイドを参照してください。

    <Note>
      事前に承認されていない限り、Anthropic は、Claude Agent SDK で構築されたエージェントを含む、サードパーティ開発者が claude.ai ログインまたはレート制限を提供することを許可していません。代わりに、このドキュメントで説明されている API キー認証方法を使用してください。
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  バグのあるファイルを作成する
</h2>

このクイックスタートでは、コード内のバグを見つけて修正できるエージェントを構築する手順を説明します。まず、エージェントが修正するための意図的なバグを含むファイルが必要です。`my-agent` ディレクトリに `utils.py` を作成し、次のコードを貼り付けます：

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

このコードには 2 つのバグがあります：

1. `calculate_average([])` はゼロで除算してクラッシュします
2. `get_user_name(None)` は TypeError でクラッシュします

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  バグを見つけて修正するエージェントを構築する
</h2>

Python SDK を使用している場合は `agent.py` を作成し、TypeScript の場合は `agent.ts` を作成します。既存のプロジェクトが CommonJS を使用している場合は、代わりに `agent.mts` を使用してください：

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic ループ：Claude が動作するときにメッセージをストリーミングします
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # これらのツールを自動承認します
              permission_mode="acceptEdits",  # ファイル編集を自動承認します
          ),
      ):
          # 人間が読める出力を印刷します
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude の推論
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # 呼び出されているツール
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # 最終結果


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic ループ：Claude が動作するときにメッセージをストリーミングします
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // これらのツールを自動承認します
      permissionMode: "acceptEdits" // ファイル編集を自動承認します
    }
  })) {
    // 人間が読める出力を印刷します
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude の推論
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // 呼び出されているツール
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // 最終結果
    }
  }
  ```
</CodeGroup>

このコードには 3 つの主要な部分があります：

1. **`query`**：agentic ループを作成するメインエントリーポイント。非同期イテレーターを返すため、`async for` を使用して Claude が動作するときにメッセージをストリーミングします。完全な API については、[Python](/docs/ja/agent-sdk/python#query) または [TypeScript](/docs/ja/agent-sdk/typescript#query) SDK リファレンスを参照してください。

2. **`prompt`**：Claude に実行させたいこと。Claude はタスクに基づいて使用するツールを判断します。

3. **`options`**：エージェントの構成。この例では、`allowedTools` を使用して `Read`、`Edit`、`Glob` を事前承認し、`permissionMode: "acceptEdits"` を使用してファイル変更を自動承認します。その他のオプションには、`systemPrompt`、`mcpServers` などがあります。[Python](/docs/ja/agent-sdk/python#claudeagentoptions) または [TypeScript](/docs/ja/agent-sdk/typescript#options) のすべてのオプションを参照してください。

`async for` ループは、Claude が考え、ツールを呼び出し、結果を観察し、次に何をするかを決定する間、実行し続けます。各反復はメッセージを生成します：Claude の推論、ツール呼び出し、ツール結果、または最終的な結果。SDK はオーケストレーション（ツール実行、コンテキスト管理、再試行）を処理するため、ストリームを消費するだけです。Claude がタスクを完了するか、エラーに達するとループが終了します。

ループ内のメッセージ処理は、人間が読める出力をフィルタリングします。フィルタリングなしでは、システム初期化と内部状態を含む生のメッセージオブジェクトが表示されます。これはデバッグに役立ちますが、そうでない場合はノイズが多くなります。

<Note>
  この例はストリーミングを使用してリアルタイムで進行状況を表示します。ライブ出力が不要な場合（バックグラウンドジョブや CI パイプラインなど）、すべてのメッセージを一度に収集できます。詳細については、[ストリーミング対単一ターンモード](/docs/ja/agent-sdk/streaming-vs-single-mode) を参照してください。
</Note>

<h3 id="run-your-agent">
  エージェントを実行する
</h3>

エージェントの準備ができました。次のコマンドで実行します：

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    スクリプトを `agent.mts` という名前にした場合は、代わりに `npx tsx agent.mts` を実行してください。
  </Tab>

  <Tab title="Python（uv）">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python（pip）">
    仮想環境がまだアクティブな状態で：

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

実行すると、エージェントは推論と呼び出す各ツールを印刷し、`Done: success` で終了します。実行後、`utils.py` を確認します。空のリストと null ユーザーを処理する防御的なコードが表示されます。エージェントは自律的に：

1. **読み取り** `utils.py` でコードを理解する
2. **分析** ロジックを分析し、クラッシュを引き起こすエッジケースを特定する
3. **編集** ファイルを編集して適切なエラーハンドリングを追加する

これが Agent SDK を異なるものにする理由です：Claude は、実装するよう求める代わりに、ツールを直接実行します。

<Note>
  `Not logged in` や `Invalid API key` などの認証エラーが表示される場合は、エージェントを実行するシェルで `ANTHROPIC_API_KEY` 環境変数を設定していることを確認してください。SDK は `.env` ファイルを自動的に読み込みません。

  これらおよび他の認証エラーの原因と修正については、エラーリファレンスの [認証エラー](/docs/ja/errors#authentication-errors) を参照してください。
</Note>

<h3 id="try-other-prompts">
  他のプロンプトを試す
</h3>

エージェントがセットアップされたので、いくつかの異なるプロンプトを試してください：

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  エージェントをカスタマイズする
</h3>

オプションを変更することで、エージェントの動作を変更できます。いくつかの例を次に示します：

**Web 検索機能を追加する：**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Claude にカスタムシステムプロンプトを提供する：**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**ターミナルでコマンドを実行する：**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

`Bash` を有効にして、次を試してください：`"Write unit tests for utils.py, run them, and fix any failures"`

各スニペットは同じオプションオブジェクトのフィールドを設定します。詳細については、[エージェントを構成する](/docs/ja/agent-sdk/configuration) を参照してください。

<h2 id="key-concepts">
  主要な概念
</h2>

**ツール** はエージェントが何ができるかを制御します：

| ツール                                | エージェントが実行できること |
| ---------------------------------- | -------------- |
| `Read`、`Glob`、`Grep`               | 読み取り専用分析       |
| `Read`、`Edit`、`Glob`               | コードの分析と変更      |
| `Read`、`Edit`、`Bash`、`Glob`、`Grep` | 完全な自動化         |

**権限モード** は、必要な人間の監視の量を制御します。SDK は、アクティブなモードをあなたの許可ルールと拒否ルールと共に、[権限がどのように評価されるか](/docs/ja/agent-sdk/permissions#how-permissions-are-evaluated) で説明されている固定の順序で評価します。モードの完全なリスト、その動作、および各モードをいつ使用するかについては、[エージェントループの仕組みの権限モード](/docs/ja/agent-sdk/agent-loop#permission-mode) を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

最初のエージェントを作成したので、その機能を拡張し、ユースケースに合わせてカスタマイズする方法を学びます：

* **[エージェントを設定する](/docs/ja/agent-sdk/configuration)**：オプションオブジェクトを構成し、各設定をカバーするページを見つける
* **[権限](/docs/ja/agent-sdk/permissions)**：エージェントが何ができるか、いつ承認が必要かを制御する
* **[Hooks](/docs/ja/agent-sdk/hooks)**：ツール呼び出しの前後にカスタムコードを実行する
* **[セッション](/docs/ja/agent-sdk/sessions)**：コンテキストを維持するマルチターンエージェントを構築する
* **[MCP サーバー](/docs/ja/agent-sdk/mcp)**：データベース、ブラウザー、API、その他の外部システムに接続する
* **[ホスティング](/docs/ja/agent-sdk/hosting)**：Docker、クラウド、CI/CD にエージェントをデプロイする
* **[サンプルエージェント](https://github.com/anthropics/claude-agent-sdk-demos)**：完全な例を参照：メールアシスタント、リサーチエージェント、その他
* **[トラブルシューティング](/docs/ja/agent-sdk/troubleshooting)**：CLI の起動に失敗したり終了したりするエラーを修正するか、構造化された出力なしで結果が到着する場合
