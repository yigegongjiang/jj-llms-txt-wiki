> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 자체 호스팅 환경을 엔드투엔드로 테스트하기

> CI에서 자체 호스팅 러너 이미지 검증: CLI로 세션을 디스패치하고, Stop 훅을 통해 Claude의 응답을 읽으며, 전체 루프를 스크립트로 작성합니다.

<Note>
  자체 호스팅 환경은 Team 및 Enterprise 플랜에서 공개 베타 상태입니다. [가용성 및 제한사항](/docs/ko/self-hosted-environments#availability-and-limitations)에서 활성화 경로를 확인할 수 있습니다. 이 페이지는 CI 테스트 레시피입니다. 설정은 [빠른 시작](/docs/ko/self-hosted-environments-quickstart)을, 프로덕션 배포 레시피는 [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy)를 참조하세요.
</Note>

[자체 호스팅 환경](/docs/ko/self-hosted-environments)에서 Claude Code [클라우드 세션](/docs/ko/claude-code-on-the-web)은 사용자가 빌드하고 유지 관리하는 러너 이미지에서 실행됩니다. 새 이미지를 프로덕션 환경에 배포하기 전에 스크립트에서 테스트 환경에 대해 전체 세션을 실행합니다. 세션을 생성하고, Claude의 응답을 읽고, 후속 메시지를 보내고, 그 응답도 읽습니다. 이것이 러너 이미지, git 액세스 및 모든 사용자 정의 도구를 검증한 후 변경 사항을 승격하기 전에 수행하는 CI 스모크 테스트의 형태입니다.

이 레시피는 사용자가 이미 [환경과 러너를 설정](/docs/ko/self-hosted-environments-quickstart#set-up-an-environment-and-runner)했으며, CI 작업이 테스트 스크립트와 동일한 호스트에서 러너 프로세스를 시작한다고 가정합니다. 이는 새 러너 이미지를 테스트하기 위한 자연스러운 설정입니다. 러너에 설치한 Stop 훅은 각 턴의 최종 응답을 로컬 파일에 기록하고, 스크립트는 그곳에서 읽으므로 Anthropic API에 대한 유일한 호출은 두 디스패치 자체입니다. 테스트 러너가 별도의 인프라에 있는 경우 [원격 테스트 러너](#remote-test-runners)를 참조하세요.

<h2 id="install-the-capture-hook-on-your-test-runner">
  테스트 러너에 캡처 훅 설치
</h2>

읽기 작업은 Claude Code [Stop 훅](/docs/ko/hooks#stop)을 통해 작동합니다. Claude가 턴을 완료하면 훅은 최종 어시스턴트 메시지를 stdin JSON의 `last_assistant_message`로 받고 `$E2E_REPLY_DIR/<session_id>.txt`에 추가합니다. [commit-nudge Stop 훅](/docs/ko/self-hosted-environments-configuration#prompt-sessions-to-push-their-work)과 동일한 방식으로 러너 호스트의 `~/.claude/`에 설치합니다. 러너는 모든 세션에 이를 시드합니다.

<h3 id="save-the-hook-files">
  훅 파일 저장
</h3>

러너 호스트에 아래 두 파일을 저장합니다:

* 설정 블록: 러너 호스트의 `~/.claude/settings.json`에 병합
* 스크립트: 러너 호스트의 `~/.claude/hooks/e2e-stop-hook-capture.sh`로 저장하고 실행 가능하게 설정

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
  러너를 시작하기 전에
</h3>

훅이 의존하는 두 가지:

* 러너를 시작하기 전에 설치합니다. 러너는 시작 시 `~/.claude/`를 한 번 스냅샷하므로, 실행 중인 러너에 추가된 훅은 재시작 후에만 적용됩니다.
* 러너 프로세스에 `E2E_REPLY_DIR`을 내보냅니다. 훅은 변수가 설정되지 않았거나 디렉토리가 없을 때 작동하지 않으므로, systemd 단위, pod 사양 또는 CI 단계와 같이 러너를 시작하는 곳에서 설정합니다. 아래 테스트 스크립트도 이를 필요로 합니다.

이 훅은 테스트 환경을 제공하는 러너에만 설치합니다. `E2E_REPLY_DIR`이 존재할 때마다 모든 세션의 최종 응답을 디스크에 기록합니다. 이는 일회용 CI 러너에서는 무해하지만 변수가 실수로 설정될 수 있는 프로덕션 환경 러너 이미지로 옮기는 것은 좋지 않습니다.

<h2 id="run-the-test-loop">
  테스트 루프 실행
</h2>

`--environment` 및 `--ref` 디스패치 플래그는 스크립트를 실행하는 머신(러너 자체와 동일한 수준)에서 Claude Code v2.1.224 이상이 필요합니다. 훅이 설치되고 이 호스트에서 러너가 시작된 상태에서 테스트 스크립트는:

1. `claude -p "<prompt>" --environment <environment-id> --output-format json`으로 테스트 환경에서 세션을 생성합니다. `origin` 원격에서 저장소를 자동 감지할 수 있도록 git 체크아웃에서 실행합니다. 선택적 `--ref <branch>`는 로컬 HEAD 대신 명명된 ref를 기반으로 세션의 체크아웃을 설정합니다. 명령은 세션을 생성하고, `session_id`를 포함하는 한 줄의 JSON을 인쇄하고, Claude의 응답을 기다리지 않고 종료합니다.
2. 러너의 Stop 훅이 턴을 완료한 후 `$E2E_REPLY_DIR/<session_id>.txt`에 기록된 응답이 나타날 때까지 기다립니다.
3. `claude -p "<message>" --cloud <session_id> --output-format json`으로 후속 메시지를 보냅니다([실행 중인 세션에 후속 메시지 보내기](/docs/ko/claude-code-on-the-web#send-follow-ups-from-the-cli) 참조). 이는 기존 세션에 사용자 이벤트를 게시하고 종료합니다.
4. 2단계와 동일한 방식으로 후속 응답을 기다립니다.

<h3 id="environment-dispatch-behavior">
  `--environment` 디스패치 동작
</h3>

Claude Code는 세션을 생성하고, 세션 ID와 링크를 인쇄하고, 종료합니다.

플래그는 [`remote.defaultEnvironmentId`](/docs/ko/settings-reference#remote-defaultenvironmentid) 설정보다 우선합니다. `--output-format stream-json`을 지원하지 않으며, `--resume`, `--continue`, `--teleport`, `--session-id` 또는 `--init-only`와 같이 세션을 재개, 연결 또는 사전 구성하는 플래그와 결합할 수 없습니다. `--cloud`는 세션 ID 또는 URL과 함께 거부되며, 설명을 포함하는 비대화형 실행에서도 거부됩니다. 단순 `--cloud`는 없는 것으로 처리됩니다. 터미널에서 위치 프롬프트 대신 `--cloud` 설명으로 작업을 전달할 수 있습니다.

<h2 id="example-script">
  예제 스크립트
</h2>

아래 스크립트는 `$CLAUDE_TEST_ENVIRONMENT_ID`에 대해 전체 루프를 실행합니다. 이는 테스트 환경의 `ccpool_...` ID이며, 관리 페이지의 환경 상세 대화상자에 표시되거나 [환경 생성 호출](#create-a-dedicated-test-environment)에서 반환됩니다. 각 응답의 센티널 구문을 어설션합니다. 캡처 훅이 설치되고 `E2E_REPLY_DIR`이 내보내진 이 호스트에서 러너를 시작한 후, 작업하려는 저장소의 git 체크아웃에서 실행합니다.

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

`TURN1`/`TURN2` 프롬프트와 `EXPECT1`/`EXPECT2` 센티널을 사용자 정의 MCP 도구 중 하나를 실행하도록 Claude에 요청하고 출력을 어설션하는 등 설정을 실행하는 것으로 바꿉니다.

<h2 id="remote-test-runners">
  원격 테스트 러너
</h2>

테스트 러너가 CI 작업이 파일 시스템을 공유할 수 없는 지속적인 Kubernetes 플릿과 같은 별도의 인프라에 있는 경우, Stop 훅의 파일 쓰기를 드라이버가 수신 대기하는 엔드포인트로의 POST로 바꿉니다:

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

드라이버 측에서는 POST를 수락하고 테스트가 요청할 때까지 응답을 보유하는 모든 것을 실행합니다. 예를 들어 CI 작업 내부의 작은 HTTP 리스너 또는 이미 실행 중인 웹훅 수신기입니다. 훅은 인프라에서 실행되므로 엔드포인트는 러너에서만 도달 가능하면 됩니다.

<h2 id="authenticate-from-ci">
  CI에서 인증
</h2>

`claude -p ... --environment`와 `claude -p ... --cloud` 모두 claude.ai OAuth 토큰으로 인증합니다. `sk-ant-xxxxx`와 같은 API 키는 두 호출 모두에서 허용되지 않습니다. 두 가지 방법으로 CI에서 토큰을 사용할 수 있습니다.

<h3 id="long-lived-ci-host">
  장기 CI 호스트
</h3>

스크립트를 실행하는 머신에서 자동화용 전용 사용자 계정을 사용하여 `claude auth login`을 한 번 대화형으로 실행합니다. Claude Code는 macOS의 OS 키체인에 토큰을 저장하거나, Linux 및 Windows의 `~/.claude/.credentials.json`에 저장합니다. 키체인을 쓸 수 없는 macOS 호스트(예: SSH 세션에서 로그인 키체인이 잠긴 경우)에서 Claude Code는 토큰을 `~/.claude/.credentials.json`에도 저장합니다. [자격증명 관리](/docs/ko/authentication#credential-management)를 참조하세요.

CLI는 각 호출 시 단기 액세스 토큰을 자동으로 새로 고치지만, 기본 새로 고침 토큰 부여는 초기 로그인으로부터 30일로 제한되므로, 해당 호스트에서 30일마다 `claude auth login`을 대화형으로 다시 실행합니다.

<h3 id="ephemeral-ci-runners">
  임시 CI 러너
</h3>

현재 이에 대한 장기 CI 토큰이 없습니다. 클라우드 세션 제어를 부여하는 범위인 `user:sessions:claude_code`는 서버 측에서 30일로 제한되므로, 1년 추론 전용 토큰을 발급하는 `claude setup-token`은 이를 포함하지 않습니다. [환경 시크릿](/docs/ko/self-hosted-environments-quickstart#set-up-an-environment-and-runner)도 허용되지 않습니다. 러너가 환경에 등록하도록만 승인하고 세션을 생성하지는 않기 때문입니다.

임시 러너에 저장된 로그인을 프로비저닝하려면 [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` 및 `CLAUDE_CODE_OAUTH_SCOPES`](/docs/ko/env-vars#variables)를 설정하여 `claude auth login`이 브라우저 없이 토큰을 교환하도록 합니다. 새로 고침 부여에 동일한 30일 상한이 적용됩니다. 인간 계정에 바인딩되지 않은 머신 ID 경로가 필요한 경우 Anthropic 계정 팀에 문의하세요.

<h2 id="create-a-dedicated-test-environment">
  전용 테스트 환경 생성
</h2>

환경을 프로그래밍 방식으로 생성 및 삭제하여 각 CI 실행이 깨끗한 환경을 얻도록 합니다. CI 작업이 시작하는 러너가 새 환경에 등록됩니다. 아래의 생성 및 삭제 호출은 claude.ai의 **클라우드 환경** 관리 페이지에서 사용하는 동일한 엔드포인트이며, `anthropic-beta: ccr-byoc-2025-07-29` 헤더가 필요합니다.

<h3 id="mint-the-admin-token">
  관리자 토큰 발급
</h3>

`$ADMIN_TOKEN`은 Owner 역할을 보유하는 계정의 claude.ai OAuth 액세스 토큰이며, [CI에서 인증](#authenticate-from-ci)과 동일한 방식으로 발급됩니다:

* **발급**: Owner 역할을 보유하는 계정으로 `claude auth login`을 실행한 후, [장기 CI 호스트](#long-lived-ci-host)에서 Claude Code가 저장한 위치에서 현재 액세스 토큰을 읽습니다.
* **각 실행마다 새로 읽기**: CLI가 액세스 토큰을 회전하고, 동일한 30일 새로 고침 부여 상한이 적용되므로, 복사본을 저장하지 마세요.
* **stdin을 통해 전달**: 예제처럼 토큰이 curl의 인수 목록이나 빌드 로그에 남지 않도록 합니다.

<h3 id="create-the-environment">
  환경 생성
</h3>

응답을 에코하지 않고 캡처합니다. `pool_secret`은 환경에 러너를 등록할 수 있는 장기 자격증명이므로, 마스킹된 CI 시크릿으로 저장하고 환경 ID만 인쇄합니다. 토큰을 프로세스 목록에서 제외하는 `-H @-` 형식은 curl 7.55 이상이 필요합니다. 이전 curl은 `@-`를 리터럴 헤더로 취급하고 인증 없이 요청을 보냅니다.

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

[Owner가 조직에 대해 **자체 호스팅 환경 허용**](/docs/ko/self-hosted-environments#availability-and-limitations)을 켤 때까지, 호출은 `403` `permission_error`로 실패하며 `self-hosted runners are disabled by your organization's policy`를 읽습니다.

이 호스트에서 `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`으로 러너를 시작합니다. [캡처 훅 설치](#install-the-capture-hook-on-your-test-runner)에 따라 캡처 훅과 `E2E_REPLY_DIR`을 추가한 후 테스트 스크립트를 실행합니다.

<h3 id="delete-the-environment">
  환경 삭제
</h3>

실행이 완료되면 환경을 삭제하여 각 CI 실행이 깨끗하게 시작되도록 합니다:

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
