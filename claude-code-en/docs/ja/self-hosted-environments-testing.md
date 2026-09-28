> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 自己ホスト環境をエンドツーエンドでテストする

> CI から自己ホスト実行イメージを検証します。CLI でセッションをディスパッチし、Stop フックを通じて Claude の返信を読み取り、完全なループをスクリプト化します。

<Note>
  自己ホスト環境は Team および Enterprise プランでパブリックベータ版です。[利用可能性と制限事項](/docs/ja/self-hosted-environments#availability-and-limitations)に有効化パスが記載されています。このページは CI テストレシピです。セットアップについては[クイックスタート](/docs/ja/self-hosted-environments-quickstart)を、フリートレシピについては[本番環境へのデプロイ](/docs/ja/self-hosted-environments-deploy)をご覧ください。
</Note>

[自己ホスト環境](/docs/ja/self-hosted-environments)では、Claude Code の[クラウドセッション](/docs/ja/claude-code-on-the-web)は、ユーザーが構築・管理する実行イメージ上で実行されます。新しいイメージを本番環境にロールアウトする前に、スクリプトからテスト環境に対して完全なセッションを実行します。セッションを作成し、Claude の返信を読み取り、フォローアップを送信し、その返信も読み取ります。これは実行イメージ、git アクセス、カスタムツールを検証する CI スモークテストの形です。

このレシピは、[環境と実行イメージのセットアップ](/docs/ja/self-hosted-environments-quickstart#set-up-an-environment-and-runner)が完了していることを前提としています。また、CI ジョブがテストスクリプトと同じホスト上で実行イメージプロセスを開始することを想定しています。これは新しい実行イメージをテストするための自然なセットアップです。実行イメージにインストールする Stop フックは、各ターンの最終返信をローカルファイルに書き込み、スクリプトがそこから読み取ります。そのため、Anthropic API への呼び出しは 2 つのディスパッチだけです。テスト実行イメージが別のインフラストラクチャ上にある場合は、[リモートテスト実行イメージ](#remote-test-runners)をご覧ください。

<h2 id="install-the-capture-hook-on-your-test-runner">
  テスト実行イメージにキャプチャフックをインストールする
</h2>

読み取りは Claude Code の[Stop フック](/docs/ja/hooks#stop)を通じて機能します。Claude がターンを完了すると、フックは最終アシスタントメッセージを stdin JSON の `last_assistant_message` として受け取り、`$E2E_REPLY_DIR/<session_id>.txt` に追加します。[commit-nudge Stop フック](/docs/ja/self-hosted-environments-configuration#prompt-sessions-to-push-their-work)と同じ方法でインストールします。実行イメージホストの `~/.claude/` にインストールします。実行イメージはこれをすべてのセッションにシードします。

<h3 id="save-the-hook-files">
  フックファイルを保存する
</h3>

実行イメージホストに以下の 2 つのファイルを保存します。

* 設定ブロック：実行イメージホストの `~/.claude/settings.json` にマージします
* スクリプト：実行イメージホストに `~/.claude/hooks/e2e-stop-hook-capture.sh` として保存し、実行可能にします

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/e2e-stop-hook-capture.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop hook for testing a self-hosted environment end to end: writes each
# turn's final assistant reply to $E2E_REPLY_DIR/<session_id>.txt so a
# co-located test driver can read it without calling the Anthropic API.
# Install on the TEST runner only. Requires jq.

# No-op unless the driver is listening. Never fail the turn.
[ -n "${E2E_REPLY_DIR:-}" ] && [ -d "$E2E_REPLY_DIR" ] || exit 0

# CLAUDE_CODE_REMOTE_SESSION_ID is exported in cse_... form; the session
# id the dispatch CLI prints is in session_... form. Same id, different
# prefix.
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0

# last_assistant_message is absent when the final assistant turn had no
# text, such as a tool-use-only turn. The `// empty` filter makes that a
# zero-byte write rather than the literal string "null".
jq -r '.last_assistant_message // empty' >> "$E2E_REPLY_DIR/$sid.txt" 2>/dev/null
exit 0
```

<h3 id="before-you-start-the-runner">
  実行イメージを開始する前に
</h3>

フックには以下の要件があります。

* 実行イメージを開始する前にインストールします。実行イメージは起動時に `~/.claude/` をスナップショットするため、実行中の実行イメージに追加されたフックは再起動後にのみ有効になります。
* 実行イメージプロセスに `E2E_REPLY_DIR` をエクスポートします。フックは変数が未設定または ディレクトリが存在しない場合は no-op です。実行イメージを開始する場所（systemd ユニット、pod spec、CI ステップなど）で設定します。以下のテストスクリプトもこれが必要です。

このフックはテスト環境にサービスを提供する実行イメージにのみインストールします。`E2E_REPLY_DIR` が存在するたびにすべてのセッションの最終返信をディスクに書き込みます。これは使い捨ての CI 実行イメージでは無害ですが、変数が誤って設定される可能性がある本番環境実行イメージには含めるべきではありません。

<h2 id="run-the-test-loop">
  テストループを実行する
</h2>

`--environment` および `--ref` ディスパッチフラグには、スクリプトを実行するマシン上の Claude Code v2.1.224 以降が必要です。これは実行イメージ自体と同じ下限です。フックが配置され、このホストで実行イメージが開始されている場合、テストスクリプトは以下を実行します。

1. `claude -p "<prompt>" --environment <environment-id> --output-format json` でテスト環境にセッションを作成します。git チェックアウトから実行して、CLI が `origin` リモートからリポジトリを自動検出できるようにします。オプションの `--ref <branch>` は、ローカル HEAD の代わりに名前付き ref に基づいてセッションのチェックアウトを行います。コマンドはセッションを作成し、`session_id` を含む 1 行の JSON を出力し、Claude の返信を待たずに終了します。
2. Stop フックが実行イメージ上でターンが完了したら `$E2E_REPLY_DIR/<session_id>.txt` に返信が表示されるまで待機します。
3. `claude -p "<message>" --cloud <session_id> --output-format json` でフォローアップを送信します（[実行中のセッションにフォローアップメッセージを送信する](/docs/ja/claude-code-on-the-web#send-follow-ups-from-the-cli)を参照）。これは既存のセッションにユーザーイベントをポストし、終了します。
4. ステップ 2 と同じ方法でフォローアップの返信を待機します。

<h3 id="environment-dispatch-behavior">
  `--environment` ディスパッチ動作
</h3>

Claude Code はセッションを作成し、セッション ID とそのリンクを出力して終了します。

フラグは [`remote.defaultEnvironmentId`](/docs/ja/settings-reference#remote-defaultenvironmentid) 設定よりも優先されます。`--output-format stream-json` をサポートしておらず、`--resume`、`--continue`、`--teleport`、`--session-id`、`--init-only` など、セッションを再開、アタッチ、または事前設定するフラグと組み合わせることはできません。`--cloud` はセッション ID または URL で拒否され、非対話型実行では説明を含む場合に拒否されます。ベアの `--cloud` は存在しないものとして扱われます。ターミナルから、位置指定プロンプトの代わりに `--cloud` 説明としてタスクを渡すことができます。

<h2 id="example-script">
  スクリプト例
</h2>

以下のスクリプトは `$CLAUDE_TEST_ENVIRONMENT_ID`（テスト環境の `ccpool_...` ID）に対して完全なループを実行します。これは管理ページの環境詳細ダイアログに表示されるか、[環境作成呼び出し](#create-a-dedicated-test-environment)によって返されます。各返信のセンチネルフレーズをアサートします。キャプチャフックがインストールされ、`E2E_REPLY_DIR` がエクスポートされている実行イメージを使用して、このホストで実行イメージを開始した後、セッションを実行したいリポジトリの git チェックアウトから実行します。

```bash theme={null}
#!/usr/bin/env bash
# End-to-end test against a self-hosted environment, using Stop-hook read-back.
# Prereqs: `claude auth login` has been run on this machine (see "Authenticate
# from CI" below); jq is installed; CLAUDE_TEST_ENVIRONMENT_ID names an
# environment whose runner is the one on this host, with the capture hook
# installed and E2E_REPLY_DIR in its environment.

set -euo pipefail

: "${CLAUDE_TEST_ENVIRONMENT_ID:=${CLAUDE_TEST_POOL_ID:-}}"  # CLAUDE_TEST_POOL_ID is the legacy spelling
: "${CLAUDE_TEST_ENVIRONMENT_ID:?set CLAUDE_TEST_ENVIRONMENT_ID to a ccpool_... id served by a runner on this host}"
: "${E2E_REPLY_DIR:?set E2E_REPLY_DIR to the directory the Stop hook on your test runner writes to, and export it to the runner process}"
: "${TEST_REPO_REF:=main}"

[ -d "$E2E_REPLY_DIR" ] || {
  echo "FAIL: E2E_REPLY_DIR ($E2E_REPLY_DIR) does not exist. The Stop hook on the runner needs it." >&2
  exit 1
}

# Waits until $E2E_REPLY_DIR/<session_id>.txt contains $2, or fails after
# 90 seconds. Tune the timeout to your environment's cold-start time. The
# file is written by the Stop hook on the runner.
await_reply() {
  local expect="$2" f="$E2E_REPLY_DIR/$1.txt"
  local deadline=$(($(date +%s) + 90))
  while :; do
    if [ -f "$f" ] && grep -qF -- "$expect" "$f"; then
      return
    fi
    [ "$(date +%s)" -lt "$deadline" ] || {
      echo "FAIL: '$expect' not in $f within 90s. The Stop hook on the runner did not write it." >&2
      echo "-- $E2E_REPLY_DIR contents --" >&2; ls -la "$E2E_REPLY_DIR" >&2
      [ -f "$f" ] && { echo "-- $f --" >&2; cat "$f" >&2; }
      exit 1
    }
    sleep 1
  done
}

# 1. Create the session on the test environment. Run from a git checkout
# so the CLI can auto-detect the repo. --ref pins the checkout to a named
# ref regardless of local HEAD.
TURN1="e2e-probe-$(date +%s)-$$: say exactly 'ok: custom tools are reachable' and nothing else"
EXPECT1="ok: custom tools are reachable"
create_json=$(claude -p "$TURN1" --environment "$CLAUDE_TEST_ENVIRONMENT_ID" \
  --ref "$TEST_REPO_REF" --output-format json)
echo "create: $create_json"
SESSION_ID=$(jq -er '.session_id' <<<"$create_json")

# 2. Wait for the turn-1 reply.
await_reply "$SESSION_ID" "$EXPECT1"
echo "turn-1 reply ok"

# 3. Post a follow-up via the CLI.
TURN2="e2e-probe-followup-$(date +%s): say exactly 'ok: follow-up delivered' and nothing else"
EXPECT2="ok: follow-up delivered"
followup_json=$(claude -p "$TURN2" --cloud "$SESSION_ID" --output-format json)
echo "followup: $followup_json"
jq -e '.ok == true' <<<"$followup_json" >/dev/null

# 4. Wait for the turn-2 reply.
await_reply "$SESSION_ID" "$EXPECT2"
echo "turn-2 reply ok"

echo "PASS: test-environment round-trip (session $SESSION_ID)"
```

`TURN1`/`TURN2` プロンプトと `EXPECT1`/`EXPECT2` センチネルを、カスタム MCP ツールの 1 つを実行するよう Claude に依頼し、その出力をアサートするなど、セットアップを実行するものに置き換えます。

<h2 id="remote-test-runners">
  リモートテスト実行イメージ
</h2>

テスト実行イメージが別のインフラストラクチャ上にある場合（CI ジョブがファイルシステムを共有できない永続的な Kubernetes フリートなど）、Stop フックのファイル書き込みをドライバーがリッスンするエンドポイントへの POST に置き換えます。

```sh theme={null}
#!/bin/sh
# Variant of the capture hook for runners on separate infrastructure.
# Set E2E_REPLY_URL on the runner to an endpoint the driver controls.
[ -n "${E2E_REPLY_URL:-}" ] || exit 0
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0
jq -r '.last_assistant_message // empty' | \
  curl -fsS -X POST --data-binary @- "$E2E_REPLY_URL/$sid" >/dev/null 2>&1
exit 0
```

ドライバー側では、POST を受け入れ、テストが要求するまで返信を保持するものを実行します。CI ジョブ内の小さな HTTP リスナーや、既に実行している webhook レシーバーなどです。フックはインフラストラクチャ上で実行されるため、エンドポイントは実行イメージからのみ到達可能である必要があります。

<h2 id="authenticate-from-ci">
  CI から認証する
</h2>

`claude -p ... --environment` と `claude -p ... --cloud` の両方は claude.ai OAuth トークンで認証します。`sk-ant-xxxxx` などの API キーはどちらの呼び出しでも受け入れられません。2 つのアプローチにより、CI でトークンを利用できるようになります。

<h3 id="long-lived-ci-host">
  長期間存続する CI ホスト
</h3>

スクリプトを実行するマシン上で、自動化用の専用ユーザーアカウントを使用して、`claude auth login` を 1 回対話的に実行します。Claude Code はトークンを macOS では OS キーチェーンに、Linux と Windows では `~/.claude/.credentials.json` に保存します。キーチェーンに書き込みできない macOS ホスト（SSH セッションでログインキーチェーンがロックされたままの場合など）では、Claude Code はトークンを `~/.claude/.credentials.json` にも保存します。[認証情報管理](/docs/ja/authentication#credential-management)を参照してください。

CLI は各呼び出しで短期アクセストークンを自動的に更新しますが、基盤となるリフレッシュトークングラントは初期ログインから 30 日間に制限されているため、そのホストで 30 日ごとに `claude auth login` を対話的に再実行してください。

<h3 id="ephemeral-ci-runners">
  エフェメラル CI 実行イメージ
</h3>

現在、これに対する長期間存続する CI トークンはありません。クラウドセッション制御を付与するスコープ `user:sessions:claude_code` はサーバー側で 30 日間に制限されているため、1 年間の推論のみのトークンを発行する `claude setup-token` はこれをカバーしていません。[環境シークレット](/docs/ja/self-hosted-environments-quickstart#set-up-an-environment-and-runner)も受け入れられません。これは実行イメージが環境に登録することのみを認可し、セッションを作成することは認可しないためです。

エフェメラル実行イメージに保存されたログインをプロビジョニングするには、[`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` と `CLAUDE_CODE_OAUTH_SCOPES`](/docs/ja/env-vars#variables) を設定して、`claude auth login` がブラウザなしでトークンを交換できるようにします。リフレッシュグラントに対して同じ 30 日間の上限が適用されます。人間のアカウントにバインドされていないマシンアイデンティティパスが必要な場合は、Anthropic アカウントチームにお問い合わせください。

<h2 id="create-a-dedicated-test-environment">
  専用テスト環境を作成する
</h2>

CI の各実行ごとにクリーンな環境を取得するために、環境をプログラムで作成および削除します。CI ジョブが開始する runner がこのフレッシュな環境に登録されます。以下の作成および削除呼び出しは、claude.ai の **Cloud environments** 管理ページが使用するのと同じエンドポイントであり、`anthropic-beta: ccr-byoc-2025-07-29` ヘッダーが必要です。

<h3 id="mint-the-admin-token">
  管理トークンを生成する
</h3>

`$ADMIN_TOKEN` は、Owner ロールを保持するアカウントの claude.ai OAuth アクセストークンであり、[CI から認証する](#authenticate-from-ci)と同じ方法で生成されます。

* **生成する**: Owner ロールを保持するアカウントで `claude auth login` を実行してから、[長期間実行される CI ホスト](#long-lived-ci-host)が Claude Code がそれを保存した場所を示す場所から現在のアクセストークンを読み取ります。
* **各実行ごとにフレッシュに読み取る**: CLI はアクセストークンをローテーションし、同じ 30 日間のリフレッシュ許可上限が適用されるため、コピーを保存しないでください。
* **stdin 経由で渡す**: 例が行うように、トークンが curl の引数リストまたはビルドログに記録されないようにします。

<h3 id="create-the-environment">
  環境を作成する
</h3>

レスポンスをキャプチャして出力しないようにします。`pool_secret` は、runner を環境に登録できる長期間有効な認証情報であるため、マスクされた CI シークレットとして保存し、環境 ID のみを出力します。トークンをプロセスリストから除外する `-H @-` 形式には curl 7.55 以降が必要です。古い curl は `@-` をリテラルヘッダーとして扱い、認可なしでリクエストを送信します。

```bash theme={null}
create=$(curl -fsS -X POST -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"name":"ci-test-environment"}' \
  https://api.anthropic.com/v1/code/runners/self-hosted/pools \
  <<<"Authorization: Bearer $ADMIN_TOKEN")
ENVIRONMENT_ID=$(jq -er .pool.pool_id <<<"$create")
ENVIRONMENT_SECRET=$(jq -er .pool_secret <<<"$create")
```

[Owner が組織に対して **Allow self-hosted environments** をオンにする](/docs/ja/self-hosted-environments#availability-and-limitations)までは、呼び出しは `403` `permission_error` で失敗し、`self-hosted runners are disabled by your organization's policy` と表示されます。

このホストで runner を開始します。`SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET` と、[キャプチャフックをテスト runner にインストールする](#install-the-capture-hook-on-your-test-runner)に従う capture hook および `E2E_REPLY_DIR` を使用してから、テストスクリプトを実行します。

<h3 id="delete-the-environment">
  環境を削除する
</h3>

各 CI 実行がクリーンな状態で開始されるように、実行が終了したときに環境を削除します。

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
