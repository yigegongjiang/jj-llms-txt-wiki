> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 자체 호스팅 환경에서 세션 사용자 정의

> 세션별 자격 증명, 라이프사이클 훅, 온디맨드 러너 생성을 위한 래퍼 스크립트로 자체 호스팅 환경 세션을 사용자 정의합니다.

<Note>
  자체 호스팅 환경은 Team 및 Enterprise 플랜에서 공개 베타 상태이며, [Owner](/docs/ko/cloud-environments#organization-shared-environments)가 [**Cloud environments** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)에서 **Allow self-hosted environments**를 활성화합니다. 이 페이지는 작동하는 러너를 가정합니다. 설정은 [빠른 시작](/docs/ko/self-hosted-environments-quickstart)을 참조하고 [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy)는 플릿 레시피를 참조하십시오.
</Note>

[자체 호스팅 환경](/docs/ko/self-hosted-environments)은 배포하는 러너 프로세스에 의해 실행되는 자신의 인프라에서 Claude Code [클라우드 세션](/docs/ko/claude-code-on-the-web)을 실행합니다. 구성이 없으면 해당 러너는 세션의 저장소를 복제하고, Claude Code를 생성하며, 정리합니다. 이 페이지는 러너를 운영하는 플랫폼 엔지니어를 위한 것입니다. 세션별 자격 증명 프로비저닝에서 체크아웃 완전 교체까지 기본값이 맞지 않을 때의 확장 지점을 다룹니다. 래퍼와 훅은 Linux 또는 macOS인 러너 호스트에서 실행 가능한 파일로 실행되며, 이 페이지의 예제는 POSIX 셸을 가정합니다.

이 페이지의 일부 훅 환경 변수는 여전히 `pool`을 사용합니다(예: `CLAUDE_RUNNER_POOL_ID`). CLI 플래그 및 환경 변수 이름은 `environment`을 사용합니다(예: `--environment-secret-file`).

<h2 id="wrapper-scripts">
  래퍼 스크립트
</h2>

각 세션이 러너가 자체적으로 수행할 수 없는 설정이 필요할 때 래퍼 스크립트를 사용합니다. 세션 작성자로 범위가 지정된 단기 자격증명 프로비저닝, 환경별 비밀 내보내기, 언어 도구 체인 준비, 또는 자식 프로세스 주변의 리소스 제한 적용입니다. 러너는 세션당 한 번 Claude Code 바이너리 대신 래퍼를 시작합니다. `$CLAUDE_RUNNER_CLAUDE_BIN`(러너 자체의 바이너리)으로 `exec`하여 래퍼를 종료하면 신호와 종료 코드가 올바르게 전파됩니다.

러너를 시작할 때 `--exec-path` 또는 `SELF_HOSTED_RUNNER_EXEC_PATH`를 래퍼로 지정합니다:

```bash theme={null}
claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --exec-path /etc/claude/session-wrapper.sh
```

러너는 래퍼의 환경에 다음을 설정합니다:

| 변수                                  | 설명                                                                                                                                                                                                                                                                                                                                                                                               |
| :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN`  | 세션 JWT이며, `sk-ant-cc-` 접두사가 붙습니다. `act` 클레임은 세션 작성자를 식별하며, 작성 표면이 기록한 경우 작성자의 이메일과 업스트림 ID 공급자 주체를 포함합니다. 값은 생성 시점의 토큰입니다. 새로고침은 자식의 stdin을 통해 도착하므로 래퍼는 초기 값만 봅니다. [세션 ID 확인](/docs/ko/self-hosted-environments-identity)을 참조하세요.                                                                                                                                                                    |
| `CCR_SESSION_ACCOUNT_EMAIL`         | 세션 작성자의 이메일이며, 서명 검증 없이 러너에 의해 토큰의 `act.email` 클레임에서 미리 추출됩니다. 커밋 트레일러와 같은 레이블 지정에 적합합니다. 이메일이 자격증명 발급을 제어할 때는 토큰을 확인하고 [세션 작성자로 범위가 지정된 자격증명 프로비저닝](#provision-credentials-scoped-to-the-session-creator)에 설명된 대로 클레임을 읽으세요. 토큰이 작성자 이메일을 전달하지 않으면 설정되지 않습니다. 개인 식별 정보로 취급하세요.                                                                                                                  |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`     | 세션을 생성한 클라이언트 표면(예: `web_claude_ai`, `desktop_app`, `ios`, `claude_code_cli`, `scheduled_trigger`)입니다. Anthropic은 세션 생성 시 값을 한 번 기록하므로 래퍼와 모든 라이프사이클 훅이 동일한 값을 봅니다. 채택 분석 및 레이블 지정에만 사용하고 인증 신호로는 사용하지 마세요. 세션에 기록되거나 인식된 표면이 없으면 설정되지 않으므로 `set -u` 아래에서 `${CLAUDE_RUNNER_CLIENT_PLATFORM:-}`로 참조하세요. Claude Code v2.1.229 이상이 필요합니다.                                                           |
| `CLAUDE_RUNNER_CLAUDE_BIN`          | 러너 자체의 Claude Code 바이너리에 대한 절대 경로입니다. 설치 경로를 하드코딩하지 않고 고정된 바이너리로 제어를 전달하려면 `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"`로 래퍼를 종료하세요.                                                                                                                                                                                                                                                                   |
| `CLAUDE_CODE_REMOTE_SESSION_ID`     | 태그가 지정된 `cse_...` 형식의 세션 ID입니다. 이는 [라이프사이클 훅](#lifecycle-hooks)이 `session_...` 형식의 `CLAUDE_RUNNER_SESSION_ID`로 보는 동일한 세션입니다. UUID 변수는 두 형식 모두에서 일치하며, `cse_` 접두사를 `session_`으로 대체하면 세션 URL에 표시되는 ID가 됩니다.                                                                                                                                                                                        |
| `CLAUDE_CODE_REMOTE_SESSION_UUID`   | 정규 UUID 형식의 동일한 세션 ID이며, UUID를 키로 사용하는 시스템용입니다.                                                                                                                                                                                                                                                                                                                                                  |
| `CLAUDE_SESSION_INGRESS_TOKEN_FILE` | 현재 세션 JWT를 보유하는 세션별 파일의 절대 경로이며, 토큰 새로고침 전체에서 최신 상태로 유지됩니다. 셸 서브프로세스는 사용자가 세션에 추가한 첨부 파일을 다운로드할 때 `Authorization` 헤더에 대해 이를 읽습니다. `exec`는 변수를 자동으로 보존합니다. 자식의 환경을 다시 빌드하는 래퍼는 변수를 전달해야 하거나 첨부 파일 다운로드가 자동으로 중지됩니다.                                                                                                                                                                               |
| `CLAUDE_CONFIG_DIR`                 | 세션별 Claude 구성 디렉토리이며, 러너가 시작 시 캡처한 러너 호스트 구성의 스냅샷에서 세션 시작 시 작성됩니다. [권한 및 도구 승인](#permissions-and-tool-approval)을 참조하세요. 여기에 대한 쓰기는 이 세션으로 격리됩니다. 디렉토리는 [`--remove-session-state`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)로 러너를 시작하지 않는 한 세션이 끝난 후 `<base-dir>/_sessions/` 아래에 유지됩니다. [사전 준비된 체크아웃 재사용](/docs/ko/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout)을 참조하세요. |
| `ANTHROPIC_BASE_URL`                | 자식이 사용할 API 기본 URL이며, 제어 평면에서 세션별로 전달되고 일반적으로 `https://api.anthropic.com`입니다. 이를 재정의하지 마세요. 세션의 추론 자격증명은 Anthropic에서 발급한 OAuth 토큰이며 다른 공급자는 이를 수락하지 않으므로 자체 호스팅 환경의 추론은 다른 곳으로 라우팅할 수 없습니다.                                                                                                                                                                                                      |
| `CLAUDE_CODE_OAUTH_TOKEN`           | 자식이 모델 추론에 사용하는 단기 OAuth 액세스 토큰이며, 모델 추론 및 파일 업로드로만 범위가 지정되고 약 30분의 수명을 가집니다. 러너는 만료 전에 이를 다시 발급하고 자식의 stdin을 통해 회전을 전달하므로 [stdin을 연결된 상태로 유지](#keep-stdin-and-file-descriptor-3-attached)하지 않는 래퍼는 초기 값만 봅니다. 조직의 IP 허용 목록에 의존하여 이 토큰의 사용을 제한하지 마세요. 이를 베어러 자격증명으로 취급하며 누출되면 약 30분 동안 사용 가능하므로 로깅하거나 디스크에 쓰거나 세션 컨테이너 외부로 전달하지 마세요.                                                             |

래퍼는 또한 서버 제공 환경 변수를 포함한 자식의 관리되는 환경의 나머지를 상속합니다. `exec`는 모두 자동으로 전파합니다. 래퍼가 자식을 다른 방식으로 생성하면 전체 환경을 전달하세요.

<h3 id="keep-stdin-and-file-descriptor-3-attached">
  stdin 및 파일 디스크립터 3을 연결된 상태로 유지
</h3>

자식의 stdin은 러너의 제어 채널입니다. 토큰 회전 및 세션 종료 신호가 이를 통해 도착합니다. 러너는 또한 파일 디스크립터 3에서 파이프를 열고 자식의 활동 신호를 읽어 유휴 및 시작 타임아웃을 구동합니다. 일반 `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"`는 둘 다 자동으로 보존합니다.

래퍼가 맨 `&`로 자식을 백그라운드에 두면 자식의 stdin이 끊깁니다. 세션은 초기 OAuth 토큰의 약 30분 수명이 만료될 때까지 정상으로 보이다가 모든 API 호출이 `401 authentication_error`로 실패합니다. 래퍼가 자식을 백그라운드에 두어야 하는 경우(예: 정리 트랩을 살리기 위해) stdin을 파일 디스크립터 4 이상에 저장하고 명시적으로 다시 연결하세요:

```bash theme={null}
exec 4<&0
"$CLAUDE_RUNNER_CLAUDE_BIN" "$@" <&4 4<&- &
CHILD=$!
trap 'teardown' EXIT
wait "$CHILD"
```

래퍼에서 파일 디스크립터 3을 닫거나 재사용하지 마세요. 자식의 stdout 및 stderr 리디렉션은 괜찮습니다.

<h3 id="provision-credentials-scoped-to-the-session-creator">
  세션 작성자로 범위가 지정된 자격증명 프로비저닝
</h3>

`decode-token` 서브명령을 사용하여 세션 JWT에서 클레임을 읽으세요. 인수, `CLAUDE_CODE_SESSION_ACCESS_TOKEN` 또는 stdin에서 토큰을 읽습니다(이 순서대로). [세션 내에서 토큰 확인](/docs/ko/self-hosted-environments-identity#verify-the-token-inside-the-session)을 참조하여 확인 내용을 확인하세요. 아래 예제는 작성자 ID를 디코딩하고, 단기 AWS 자격증명으로 교환하고, Claude Code로 실행합니다:

```bash theme={null}
#!/bin/bash
# 안정적인 Anthropic 사용자 ID를 기반으로 하고 인간 작성자를 요구합니다.
CREATOR_SUB=$("$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token \
  | jq -re '.act.sub // "" | select(startswith("user:"))') \
  || { echo "decode-token: verification failed or no human creator" >&2; exit 1; }

creds=$(your-sts-helper assume-role --subject "$CREATOR_SUB") \
  || { echo "credential exchange failed" >&2; exit 1; }
eval "$creds"

exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"
```

추출된 클레임이 인증 결정을 제어할 때 `jq -r` 대신 `jq -re`를 사용하면 누락된 클레임이 리터럴 문자열 `null`을 다운스트림으로 전달하는 대신 0이 아닌 값으로 종료됩니다. 조직 서비스 ID(예: 봇 및 에이전트 세션)에 의해 생성된 세션은 `user:` 주체 대신 `agent:` 주체를 전달하므로 이 예제는 이를 거부합니다. 환경이 이러한 세션을 제공하는 경우 래퍼가 종료하는 대신 기본 자격증명으로 폴백할지 명시적으로 결정하세요. 자격증명 교환이 SSO 주체 또는 이메일이 필요한 경우 `.act.attested_by.sub` 또는 `.act.email`을 읽고 부재를 처리하세요. 토큰은 작성 표면이 기록한 경우에만 이를 전달하며 [CLI 디스패치 세션](/docs/ko/self-hosted-environments-testing#run-the-test-loop)은 둘 다 부족할 수 있습니다. 전체 클레임 참조 및 러너 외부 서비스의 검증은 [세션 ID 확인](/docs/ko/self-hosted-environments-identity)을 참조하세요.

<h2 id="lifecycle-hooks">
  라이프사이클 훅
</h2>

라이프사이클 훅은 러너의 세션별 파이프라인 단계를 자신의 스크립트로 교체합니다. `--hooks-dir <path>` 또는 `SELF_HOSTED_RUNNER_HOOKS_DIR`로 러너를 훅 디렉토리로 지정합니다. 러너는 잘 알려진 이름의 실행 가능한 파일을 찾습니다. 존재하지 않는 훅은 기본 동작으로 폴백하므로 필요한 것만 작성하면 됩니다. 훅은 러너 자체의 권한으로 실행되며 세션 자식은 해당 UID를 공유하므로 훅 디렉토리를 읽기 전용으로 마운트하거나 이미지에 구워서 세션 코드가 수정할 수 없도록 하세요. [강화 섹션](/docs/ko/self-hosted-environments-deploy#harden-your-deployment)을 참조하세요.

이러한 훅은 [Claude Code 훅](/docs/ko/hooks)(세션 내에서 실행)과 다르며, 라이프사이클 훅은 러너에서 세션 주변에서 실행됩니다.

<h3 id="checkout">
  checkout
</h3>

저장소당 한 번 실행되며, 러너의 기본 제공 복제 및 페치 대신 실행됩니다. 훅을 사용하여 읽기 전용 미러에서 복제하거나, 아카이브에서 작업 트리를 시드하거나, 세션별 git 인증을 적용하세요. 러너는 다음을 설정합니다:

| 변수                                 | 설명                                                                                              |
| :--------------------------------- | :---------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_REPO_URL`           | 복제할 저장소 URL이며, `--git-host-rewrite` 및 `--git-ssh-rewrite`가 적용된 후입니다.                            |
| `CLAUDE_RUNNER_REPO_REF`           | 체크아웃할 리비전: 세션이 요청한 대로 분기, 태그 또는 커밋 SHA입니다. 비어 있으면 저장소의 기본 분기를 의미합니다.                            |
| `CLAUDE_RUNNER_CHECKOUT_PATH`      | 작업 트리를 남겨야 할 절대 경로                                                                              |
| `CLAUDE_RUNNER_SESSION_ID`         | 태그가 지정된 `session_...` 형식의 세션 ID이며, 로깅 및 상관관계 지정용입니다.                                            |
| `CLAUDE_RUNNER_SESSION_UUID`       | 정규 UUID 형식의 동일한 세션 ID                                                                           |
| `CLAUDE_RUNNER_API_BASE_URL`       | 세션 범위 호출용 Anthropic API 기본 URL                                                                  |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | 세션을 생성한 클라이언트 표면(예: `web_claude_ai`, `desktop_app`, `ios`)입니다. 세션에 기록되거나 인식된 표면이 없으면 설정되지 않습니다. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | 세션 범위 API 호출용 세션 액세스 토큰                                                                         |

스크립트는 요청된 리비전에서 체크아웃된 `CLAUDE_RUNNER_CHECKOUT_PATH`에 작업 트리를 남겨야 합니다. 분리된 HEAD는 괜찮습니다. 러너는 그 위에 세션의 작업 분기를 생성합니다. 러너는 이후 경로에 `.git`이 포함되어 있는지 확인합니다. 훅이 Perforce 또는 압축 해제된 tarball과 같은 비git 소스를 구체화하면 러너의 환경에서 `CLAUDE_RUNNER_SKIP_GIT_VERIFY=1`을 설정하여 해당 확인을 건너뛰세요. 작업 분기 생성 및 결과 푸시와 같은 git 기반 흐름에는 git 체크아웃이 필요하므로 [`post-session` 훅](#post-session)으로 비git 트리에서 결과를 내보내세요.

러너는 훅에 git 자격증명을 전달하지 않습니다. 대신 세션의 ID에서 세션별 복제 자격증명을 발급합니다. [세션 서비스에서 토큰 확인](/docs/ko/self-hosted-environments-identity#verify-the-token-from-your-service)에 설명된 대로 `CLAUDE_RUNNER_API_BASE_URL` 아래의 JWKS 엔드포인트에 대해 표준 JWT 라이브러리로 `CLAUDE_CODE_SESSION_ACCESS_TOKEN`을 확인한 다음 자격증명 서비스가 토큰의 `act` 클레임의 ID에 대한 단기 복제 자격증명을 발급하도록 합니다. `CLAUDE_RUNNER_CLAUDE_BIN`은 체크아웃 훅 환경에서 설정되지 않으므로 `decode-token` 서브명령을 사용할 수 없습니다. SSH 에이전트, 자격증명 도우미 또는 `.netrc`와 같이 호스트가 이미 가지고 있는 git 인증으로 폴백하는 것도 옵션입니다.

훅이 0이 아닌 값으로 종료되거나 0으로 종료되지만 사용 가능한 체크아웃을 남기지 않으면 러너가 수행하는 작업은 저장소에 따라 다릅니다:

* **세션이 결과를 푸시하는 저장소**: 러너가 세션을 실패하고 0이 아닌 종료 시 스크립트의 stderr 끝을 사용자에게 표시합니다.
* **세션이 읽기만 하는 저장소**(예: 실행 중인 세션에 추가된 저장소): 러너는 `[runner:warn]` 줄을 실패 세부 정보와 함께 로깅하고, `Skipped` 단계를 세션에 게시하고, 훅이 체크아웃 경로에 남긴 것을 제거하고, 나머지 저장소로 계속합니다. 러너가 경로를 즉시 제거할 수 없으면 세션 종료 시 제거를 다시 시도합니다. 건너뛰기로 인해 세션에 저장소가 전혀 없으면 러너는 어쨌든 세션을 실패합니다.

v2.1.228 이전에는 러너가 모든 저장소에 대해 훅 실패 시 세션을 실패했으므로 훅이 제공할 수 없는 읽기 전용 저장소는 세션이 새 러너에서 다시 시작될 때마다 다시 세션을 실패했습니다.

러너는 세션이 끝난 후 체크아웃 경로를 제거합니다.

<h3 id="post-session">
  post-session
</h3>

세션당 한 번 실행되며, Claude Code 자식이 종료된 후 러너가 작업 공간을 정리하기 전입니다. 이 훅은 커밋되지 않은 작업을 저장할 수 있는 유일한 기회입니다. `--capacity`가 1보다 크면 러너는 훅이 반환된 직후 세션별 작업 트리를 삭제하고, `--capacity 1`이면 재사용된 [정규 복제](/docs/ko/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout)는 다음 세션이 시작될 때 하드 리셋되므로 커밋되지 않은 추적 변경 사항은 어느 경로에서도 유지되지 않습니다. 일반적인 용도는 커밋되지 않은 변경 사항의 스냅샷 분기 푸시, 로그 아카이빙 또는 자신의 시스템에 세션 종료 이벤트 내보내기입니다.

훅은 자식 프로세스가 생성된 모든 세션 종료에서 실행되며, 원인이 무엇이든 상관없습니다. 아래의 `CLAUDE_RUNNER_EXIT_REASON` 값은 경우를 열거합니다. VM 선점 또는 정전과 같이 러너가 갑자기 종료될 때는 실행될 수 없습니다. 갑작스러운 종료에 대한 보장이 필요하면 Claude Code `PostToolUse` 훅으로 세션 내에서 주기적으로 스냅샷하세요. 러너는 다음을 설정합니다:

| 변수                                 | 설명                                                                                                                              |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `CLAUDE_RUNNER_SESSION_ID`         | 태그가 지정된 `session_...` 형식의 세션 ID                                                                                                 |
| `CLAUDE_RUNNER_SESSION_UUID`       | 정규 UUID 형식의 동일한 세션 ID                                                                                                           |
| `CLAUDE_RUNNER_EXIT_REASON`        | 세션이 종료된 방식이며, 아래 표 아래의 값을 참조하세요.                                                                                                |
| `CLAUDE_RUNNER_WORKSPACE_PATHS`    | 세션의 작업 트리의 콜론 구분 절대 경로입니다. 0개 저장소 세션의 경우 비어 있습니다.                                                                               |
| `CLAUDE_RUNNER_DEBUG_LOG_PATH`     | 세션의 디버그 로그 경로이며, 훅이 실행되는 동안 여전히 디스크에 있습니다.                                                                                      |
| `CLAUDE_RUNNER_API_BASE_URL`       | 세션 범위 호출용 Anthropic API 기본 URL                                                                                                  |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | 세션을 생성한 클라이언트 표면(예: `web_claude_ai`, `desktop_app`, `ios`)입니다. 세션에 기록되거나 인식된 표면이 없으면 설정되지 않습니다. Claude Code v2.1.229 이상이 필요합니다. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | 세션 범위 API 호출용 세션 액세스 토큰                                                                                                         |

`CLAUDE_RUNNER_EXIT_REASON`은 네 가지 값 중 하나를 취합니다:

* `completed`: 세션이 깔끔하게 종료되었습니다. Claude Code 프로세스가 정상적으로 종료되었거나, 세션이 여전히 실행 중인 동안 아카이브되거나 삭제되었습니다.
* `failed`: Claude Code 프로세스가 충돌했거나, 시작 후 설정이 실패했습니다.
* `interrupted`: 러너가 세션을 중지했습니다. 슬롯을 해제하기 위해 세션을 해제했거나, 시작 시 타임아웃되었거나, 서버가 이 러너에서 세션을 이동했거나, 러너가 드레인 중이었거나, 세션이 [`--kill-session-after-min`](/docs/ko/self-hosted-environments-reference#runner-cli-flags) 제한을 초과했습니다.
* `abandoned`: 다른 러너가 요청한 세션용으로 예약되어 있습니다. 훅은 현재 이 경우에 실행되지 않습니다.

[세션 라이프사이클 카운터 의미론](/docs/ko/self-hosted-environments-reference#session-lifecycle-counter-semantics)은 해제, 시작 타임아웃 및 서버 이동을 `interrupted` 대신 `completed`로 계산합니다. 러너가 슬롯을 깔끔하게 반환했기 때문입니다. 훅 수신과 카운터를 비교할 때 이 차이를 예상하세요.

훅의 종료 상태는 세션 결과에 영향을 주지 않습니다. 실패는 로깅되고 무시됩니다. 러너는 `--post-session-hook-timeout-sec`(기본값 60초)까지 모든 세션 종료(러너 종료 포함)에서 대기합니다. 이 예제는 커밋되지 않은 작업을 구조 분기에 저장합니다:

```bash theme={null}
#!/usr/bin/env bash
set -u
IFS=':'
# 체크아웃의 .git/config에 세션이 심을 수 있는 구성을 고정합니다:
# -c 재정의는 저장소 로컬 설정을 이기므로 세션 작성 fsmonitor,
# 훅 경로 및 gpg-program 구성이 훅의 권한으로 코드를 실행하는 것을 차단합니다.
# 저장소 로컬 credential.helper, core.sshCommand 및 pushurl
# 여전히 적용됩니다. 훅이 세션이 가지지 않은 자격증명을 보유하면 푸시 URL 및 도우미도 고정하세요
# (아래 스크립트의 참고 사항 참조).
g() { git -c core.fsmonitor=false -c core.hooksPath=/dev/null \
        -c commit.gpgsign=false "$@"; }
for ws in $CLAUDE_RUNNER_WORKSPACE_PATHS; do
  cd "$ws" 2>/dev/null || continue
  [ -z "$(g status --porcelain 2>/dev/null)" ] && continue
  g add -A
  g commit -q -m "runner snapshot: $CLAUDE_RUNNER_SESSION_ID ($CLAUDE_RUNNER_EXIT_REASON)" || continue
  g push -q origin "HEAD:refs/heads/rescue/$CLAUDE_RUNNER_SESSION_ID" || true
done
```

훅은 러너 호스트의 자체 환경에서 사용 가능한 git 자격증명으로 푸시합니다. [이미지에 자격증명 없음 자세](/docs/ko/self-hosted-environments-deploy#configure-git) 아래에서(기본 제공 복제가 Anthropic git 프록시를 통과할 때 포함) 없으므로 푸시하기 전에 훅 내에서 단기 푸시 자격증명을 발급합니다. 훅이 `CLAUDE_CODE_SESSION_ACCESS_TOKEN`에서 받는 세션 토큰을 자신의 토큰 서비스와 교환하고 [세션 ID 확인](/docs/ko/self-hosted-environments-identity)에서 설명하는 대로 확인합니다. 훅이 세션이 가지지 않은 자격증명을 보유하면 푸시 위치도 고정합니다. `origin`을 운영자 제공 URL로 바꾸고 `-c credential.helper=` 및 자신의 도우미를 전달하면 세션이 작성한 저장소 로컬 구성이 자격증명 푸시를 리디렉션할 수 없습니다.

<h4 id="hook-timing-when-the-runner-releases-a-session">
  러너가 세션을 해제할 때 훅 타이밍
</h4>

해제된 세션은 다른 러너에서 다시 시작할 수 있습니다. v2.1.236 이상의 러너에서 세션이 해제 시 수행 중이던 작업이 이 훅이 완료되기 전에 다른 러너에서 다시 시작할 수 있는지 결정합니다:

* **턴 후 유휴 상태이거나 시작 시 타임아웃**: 러너가 자식을 중지하고 이 훅을 완료할 때까지 실행합니다. 그 후에만 세션을 해제합니다. 훅이 실행되는 동안 전송된 사용자 메시지는 훅이 완료되기 전에 다른 러너에서 세션을 다시 시작할 수 없습니다.
* **사용자가 권한 프롬프트와 같은 프롬프트에 답하기를 기다리는 중**: 러너가 먼저 세션을 해제한 다음 이 훅을 실행합니다. 훅이 실행되는 동안 전송된 사용자 메시지는 훅이 완료되기 전에 다른 러너에서 세션을 다시 시작할 수 있습니다.

이는 러너가 세션을 해제할 때마다 적용됩니다: 유휴 타임아웃 시, [`--retire-at`](/docs/ko/self-hosted-environments-reference#runner-cli-flags) 시간에, 그리고 v2.1.260 이상의 러너에서 세션의 [`--kill-session-after-min`](/docs/ko/self-hosted-environments-reference#runner-cli-flags) 제한 시. 턴이 끝났고 백그라운드 작업만 보유한 세션은 여기서 유휴로 계산됩니다. v2.1.236 이전에는 러너가 두 경우 모두에서 먼저 세션을 해제한 다음 이 훅을 실행했습니다.

`SIGTERM` 드레인 중에 러너는 훅이 완료될 때까지 세션 임차를 유지합니다. [종료 타이밍](/docs/ko/self-hosted-environments-deploy#shutdown-timing)을 참조하세요.

<h3 id="command">
  command
</h3>

세션당 한 번 실행되며, 체크아웃 후 기본 제공 자식 생성 대신 실행됩니다. 훅은 [래퍼 스크립트](#wrapper-scripts)와 동일한 환경을 받으며 동일한 방식으로 `"$CLAUDE_RUNNER_CLAUDE_BIN"`으로 `exec`해야 합니다. 모든 사용자 정의를 하나의 훅 디렉토리에 유지하려면 `command` 훅을 사용하세요. 래퍼가 다른 곳에 있을 때 `--exec-path`를 사용하세요. `--exec-path`도 설정되면 플래그가 우선하고 `command` 훅은 무시됩니다.

PATH 해석 `claude` 대신 항상 러너 자체의 바이너리로 `exec`하세요. 그렇지 않으면 [버전 고정](/docs/ko/self-hosted-environments-deploy#pin-the-version)을 무효화합니다.

<h2 id="on-demand-runners">
  온디맨드 러너
</h2>

고정 플릿을 실행하는 대신 세션당 하나의 러너를 부팅할 수 있습니다. 오케스트레이터는 별도의 상태 비저장 서브명령이며 Anthropic에 생성 요청을 폴링합니다(사용 가능한 러너가 없는 큐에 있는 각 세션당 하나). 각각에 대해 `spawn-runner` 훅을 실행합니다. 훅은 Kubernetes Job, EC2 인스턴스, Nomad 디스패치와 같은 플랫폼에 워크로드를 제출합니다.

온디맨드 러너는 자격증명 위생을 개선합니다. 고정 플릿에서 환경 비밀은 모든 러너 호스트에 있으며, 이는 사용자 세션을 실행하는 동일한 호스트입니다. 오케스트레이터를 사용하면 환경 비밀은 오케스트레이터 호스트에만 유지되며, 이는 사용자 코드를 실행하지 않습니다. 각 생성된 러너는 정확히 하나의 러너를 등록한 다음 만료되는 단일 사용 작업 주문을 받습니다.

오케스트레이터를 시작하려면 환경 비밀과 실행 가능한 `spawn-runner` 스크립트를 포함하는 훅 디렉토리를 전달합니다:

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks
```

오케스트레이터는 폴 간에 상태를 유지하지 않으므로 가용성을 위해 동일한 환경에 대해 두 개 이상의 복제본을 실행할 수 있습니다. 각 생성 요청은 서버 측에서 정확히 하나의 복제본으로 요청됩니다. 모든 복제본은 동일한 `--expected-spawn-seconds` 값을 사용해야 합니다. [훅 계약](#the-spawn-runner-hook)을 참조하세요.

<h3 id="the-spawn-runner-hook">
  spawn-runner 훅
</h3>

오케스트레이터는 생성 요청당 한 번 `${hooks-dir}/spawn-runner`를 실행합니다. 훅은 러너가 부팅될 때까지 기다리지 않고 비동기적으로 작업을 제출해야 하며 `--hook-timeout`(기본값 60초) 내에 반환해야 합니다. 훅은 다음을 받습니다:

| 변수                                    | 설명                                                                                                                                                                                                                              |
| :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `CLAUDE_RUNNER_WORK_ORDER_FILE`       | 새 러너가 등록하는 서명된 작업 주문 JWT를 포함하는 임시 파일의 경로입니다. 훅이 종료된 후 삭제됩니다. 파일의 내용을 로깅하지 마세요.                                                                                                                                                  |
| `CLAUDE_RUNNER_ORDER_ID`              | 생성 요청당 고유하고 Kubernetes 리소스 이름에 안전한 불투명 멱등성 키입니다. 프로비저너의 중복 제거 키로 사용하세요.                                                                                                                                                         |
| `CLAUDE_RUNNER_SESSION_ID`            | 이 요청이 대상인 세션입니다. [`--min-idle`](/docs/ko/self-hosted-environments-reference#orchestrator-cli-flags)이 설정되어 있을 때 특정 세션 전에 대기 러너를 부팅하는 사전 워밍 요청의 경우 비어 있으므로 변수가 설정되어 있다고 가정하지 마세요.                                                      |
| `CLAUDE_RUNNER_SESSION_UUID`          | 정규 UUID 형식의 동일한 세션 ID입니다. 사전 워밍 요청의 경우 비어 있습니다.                                                                                                                                                                                 |
| `CLAUDE_RUNNER_ATTEMPT`               | 이 세션이 가진 생성 요청의 수입니다. 사전 워밍 요청의 경우 `0`입니다.                                                                                                                                                                                      |
| `CLAUDE_RUNNER_ORDER_SERVER_TIME`     | 폴 응답의 HTTP `Date` 헤더에서 서버 시간입니다. 훅이 작업 주문 JWT의 `exp`를 확인할 때 로컬 클록 대신 이 값과 비교하여 스큐를 허용합니다. 게이트웨이가 헤더를 생략한 경우 비어 있습니다.                                                                                                            |
| `CLAUDE_RUNNER_POOL_ID`               | 새 러너가 조인해야 할 환경의 ID이며, `ccpool_...` 형식입니다.                                                                                                                                                                                      |
| `CLAUDE_RUNNER_ACCOUNT_ID`            | 세션을 큐에 넣은 계정의 태그가 지정된 ID이며, 계정별 라우팅, 할당량 또는 차지백용입니다. 사용할 수 없을 때 비어 있으며, Claude Tag 채널 세션의 경우 항상 비어 있습니다(계정이 큐에 넣지 않음).                                                                                                          |
| `CLAUDE_RUNNER_ACCOUNT_EMAIL`         | 세션을 큐에 넣은 계정의 이메일입니다. 사용할 수 없을 때 비어 있습니다. 이메일을 개인 식별 정보로 취급하고 로깅하지 마세요.                                                                                                                                                         |
| `CLAUDE_RUNNER_PRIMARY_REPO_URL`      | 세션의 첫 번째 git 소스의 URL이며, 해당 저장소가 사전 워밍된 러너로 라우팅하기 위한 것입니다. 세션에 git 소스가 없으면 비어 있습니다.                                                                                                                                              |
| `CLAUDE_RUNNER_PRIMARY_REPO_REVISION` | 세션의 첫 번째 git 소스의 리비전: 분기, SHA 또는 태그입니다. 지정되지 않으면 비어 있습니다.                                                                                                                                                                       |
| `CLAUDE_RUNNER_REPO_SOURCES`          | 모든 세션의 git 소스에 대한 `{url, revision}` JSON 배열이며, 보조 저장소에서 라우팅하는 훅용입니다. 소스가 없으면 비어 있습니다.                                                                                                                                           |
| `CLAUDE_RUNNER_CORRELATION_ID`        | 세션 생성 시 제공된 상관관계 ID이며, 훅이 이 작업 주문을 세션을 생성한 요청에 매핑할 수 있도록 다시 에코됩니다. 세션에 없으면 비어 있습니다.                                                                                                                                             |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`       | 채택 분석용 세션을 생성한 클라이언트 표면(예: `web_claude_ai`, `desktop_app`, `ios`, `scheduled_trigger`)입니다. 세션에 기록되거나 인식된 표면이 없으면 설정되지 않으며, 사전 워밍 요청의 경우도 마찬가지입니다. `[ -n "${CLAUDE_RUNNER_CLIENT_PLATFORM:-}" ]`로 확인하면 `set -u` 아래에서 안전하게 유지됩니다. |

생성된 러너는 환경 비밀 대신 작업 주문으로 등록합니다:

* **작업 주문으로 시작**: [`--environment-secret-file`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)을 작업 주문 JWT를 포함하는 파일로 지정하거나 `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET`을 JWT 값으로 설정합니다.
* **훅이 종료되기 전에 JWT 복사**: 오케스트레이터는 훅이 종료된 후 작업 주문 파일을 삭제하므로 JWT를 제출하는 워크로드(예: 생성된 Job의 Kubernetes Secret)에 복사하고 파일 경로를 통과하지 마세요.
* **생성된 러너에서 `--capacity 1` 사용**: 세션 바운드 작업 주문은 정확히 하나의 세션에 바운드된 러너를 등록하므로 더 높은 용량은 작업을 받지 않는 슬롯을 추가하며 러너는 시작 시 경고를 로깅합니다.
* **사전 워밍 작업 주문은 바운드되지 않음**: 대기 러너는 세션에 바운드되지 않으며 고정 플릿 러너처럼 큐에 있는 작업을 요청합니다.

계약에는 네 가지 프로비저너 불가지론적 규칙이 있습니다:

1. **`CLAUDE_RUNNER_ORDER_ID`에서 멱등성**: 동일한 요청의 재전달은 최대 하나의 러너를 생성해야 합니다. ID에서 결정론적 리소스 이름을 파생하고 플랫폼이 중복을 거부하도록 하세요.
2. **워크로드를 다시 시도하지 마세요**: 하나의 주문 ID는 최대 하나의 생성된 워크로드를 의미합니다. 러너가 등록되지 않으면 Anthropic은 `--expected-spawn-seconds` 후 새 주문 ID로 다시 요청합니다.
3. **종료 코드 계약 사용**: 0으로 종료하면 제출됨을 의미합니다. 1로 종료하면 재시도 가능한 실패를 의미합니다. 세션이 백오프되고 다시 제공됩니다. 2 이상으로 종료하면 재시도 불가능을 의미합니다. 세션은 [Owner](/docs/ko/cloud-environments#organization-shared-environments)가 환경의 **Activity** 탭에서 **Retry**를 선택할 때까지 다시 생성되지 않습니다. 0이 아닌 종료 시 훅의 stderr 끝이 실패 이유로 표시되므로 실행 가능한 오류를 stderr에 작성하고 비밀을 작성하지 마세요. 사전 워밍 요청의 경우 세션이 없습니다. 오케스트레이터는 0이 아닌 종료를 로컬로만 로깅하고 서버는 임차 후 생성을 다시 요청합니다.
4. **`--expected-spawn-seconds`를 최소한 p99 부팅 시간으로 설정**: 이는 서버 측 임차입니다. 모든 오케스트레이터 복제본은 동일한 값을 사용해야 합니다.

훅이 stdout 또는 stderr에 작성하는 모든 것은 자격증명이 자동으로 수정된 오케스트레이터의 로그에 나타납니다. 세션이 큐에 있으면 오케스트레이터의 `/healthz` 본문에서 큐 수를 확인한 다음 [**Cloud environments** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)에서 환경의 **Activity** 탭을 엽니다. 실패한 세션을 확장하여 생성 오류를 확인하고 **Retry**를 선택하여 다시 요청하세요.

<h2 id="mcp-servers">
  MCP 서버
</h2>

모든 세션에서 [MCP 서버](/docs/ko/mcp)를 사용 가능하게 하려면 데스크톱 설치에서 사용되는 동일한 `claude mcp add` 명령으로 이미지 빌드 시간에 추가합니다. 러너가 컨테이너가 아닌 베어 프로세스인 경우 호스트의 러너 사용자로 동일한 명령을 실행한 다음 러너를 다시 시작합니다. 시작 시 호스트 구성을 한 번 읽습니다. `--scope user` 플래그가 필요합니다. 기본 로컬 범위는 러너가 시드하지 않는 디렉토리별 키 아래에 작성합니다. 예를 들어 Dockerfile에서:

```dockerfile theme={null}
RUN claude mcp add --scope user sidecar -- /usr/local/bin/mcp-sidecar
RUN claude mcp add --scope user --transport http internal http://mcp-gateway.svc.cluster.local:8080
```

러너는 시작 시 호스트의 구성을 스냅샷합니다. 스냅샷은 호스트의 `.claude.json`(\~/.claude/ 내부가 아닌 옆에 있음)에서 `mcpServers` 키를 캡처하며, 러너는 각 세션의 격리된 구성에 해당 키만 시드합니다. 계정 상태 및 프로젝트 기록은 삭제됩니다. 서버가 세션에 도달했는지 확인하려면 환경에서 세션을 시작하고 Claude에 MCP 도구를 나열하도록 요청하세요. 러너는 또한 `type`을 인식하지 못하는 캡처된 항목에 대해 시작 경고를 로깅하고 항목을 삭제하므로 해당 서버가 세션에서 누락된 이유를 볼 수 있습니다. `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR`이 설정되면 러너는 대신 해당 디렉토리에서 `.claude.json`을 읽으므로 변수를 빈 디렉토리로 지정하면 MCP 시드도 비활성화됩니다.

Claude Code는 또한 다른 소스에서 MCP 서버를 로드합니다:

* 엔터프라이즈 범위 [관리 MCP 파일](/docs/ko/managed-mcp)의 표준 시스템 경로: Linux 러너 호스트의 `/etc/claude-code/managed-mcp.json`, macOS 호스트의 `/Library/Application Support/ClaudeCode/managed-mcp.json`. 관리자 목록 서버만 로드할 수 있는 잠금 플릿에 사용합니다. [managed-mcp.json으로 배타적 제어](/docs/ko/managed-mcp#exclusive-control-with-managed-mcp-json)의 우선순위 규칙을 참조하세요. 이 파일이 러너 호스트에 있으면 Claude Code는 Anthropic의 제어 평면이 세션에 전달하는 MCP 서버(claude.ai 커넥터 포함)를 건너뛰고 세션 자식의 stderr에 경고로 이름을 지정합니다(러너는 `debug` 로그 수준에서 기록). v2.1.229 이전에는 이러한 세션이 `You cannot dynamically configure MCP servers when an enterprise MCP config is present`로 시작 시 종료되었습니다.
* 러너 호스트의 [관리 설정](/docs/ko/managed-settings)의 [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers) 키: 배타적 제어를 취하지 않고 HTTP 및 SSE 서버를 제공하므로 다른 소스의 서버가 여전히 로드됩니다. Claude Code v2.1.259 이상이 필요합니다.
* `<repo>/.mcp.json`: 프로젝트 범위입니다. 파일을 저장소에 커밋합니다. 해당 서버는 클라우드 세션에서 자동 승인됩니다.

조직에 대해 커넥터 전달이 활성화되면 Anthropic의 제어 평면은 claude.ai에서 구성한 커넥터를 대화형으로 생성된 세션에 서버 제공 MCP 구성을 통해 `api.anthropic.com`을 통해 라우팅하여 전달합니다. [CLI 디스패치](/docs/ko/self-hosted-environments-testing#run-the-test-loop)와 같이 프로그래밍 방식으로 생성된 세션은 커넥터 전달을 받지 않습니다. 이 섹션에 나열된 다른 소스를 통해 MCP 서버를 제공하세요. 자식의 OAuth 토큰은 커넥터를 직접 가져오기 위한 범위를 전달하지 않으므로 자식은 해당 가져오기를 시도하지 않습니다. 전달은 서버 구동입니다.

`settings.json`은 MCP 서버 정의를 전달하지 않으며 설정 스키마에 최상위 `mcpServers` 필드가 없습니다. 관리 설정에서 [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers) 키로 서버를 제공하세요.

세션은 러너의 환경을 상속하므로 [`ENABLE_TOOL_SEARCH`](/docs/ko/mcp#scale-with-mcp-tool-search)를 설정하여 러너가 생성하는 모든 세션에 대해 MCP 도구 검색을 제어합니다. MCP 페이지에서 값을 다룹니다.

<h2 id="prompt-sessions-to-push-their-work">
  세션에 작업 푸시 프롬프트
</h2>

Anthropic 호스팅 세션은 Claude가 응답을 마칠 때 실행되는 Claude Code 훅인 [`Stop` 훅](/docs/ko/hooks#stop)을 실행하며, 작업을 커밋하고 푸시하도록 Claude에 프롬프트합니다. 러너는 하나를 설치하지 않습니다. 이 없이 커밋되지 않은 변경 사항으로 끝나는 세션은 해당 작업을 러너의 디스크에만 남기며 분기가 원격에 존재할 때까지 claude.ai/code의 **Create PR** 버튼은 비활성 상태로 유지됩니다.

아래의 참조 구현에는 두 부분이 있습니다. 설정 블록을 러너 호스트의 `~/.claude/settings.json`에 병합합니다(러너가 모든 세션에 시드). 스크립트를 러너 호스트의 `~/.claude/hooks/stop-hook-nudge.sh`로 저장하고 실행 가능하게 만듭니다:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/stop-hook-nudge.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# 자체 호스팅 러너용 Stop 훅 참조 구현입니다.
#
# 프로젝트 디렉토리에 커밋되지 않은 변경 사항 또는 푸시되지 않은 커밋이 있으면
# 턴당 한 번 Claude에 프롬프트하므로 유휴 세션이 해제될 때 작업이 손실되지 않고
# claude.ai/code의 "Create PR" 버튼이 켜집니다.
#
# 러너 수준(저장소 변경 없음): 이 파일을 러너 호스트의 ~/.claude/hooks/에 놓고
# 함께 제공되는 Stop 훅 설정 블록을 ~/.claude/settings.json에 병합합니다.
# 러너가 둘 다 모든 세션에 시드합니다.
# 저장소 수준 대안: <repo>/.claude/hooks/에 커밋하고 settings.json 명령 경로를
# $CLAUDE_PROJECT_DIR/.claude/hooks/로 변경합니다.
#
# stdin: 훅 JSON 페이로드 (https://code.claude.com/docs/en/hooks 참조)
# stdout: {"decision":"block","reason":"..."} 프롬프트하거나 중지를 허용하려면 아무것도 없음.

# 재진입 가드: 하네스는 Stop 훅을 다시 호출할 때 stop_hook_active=true를 설정합니다.
# 블록 후. 종료하므로 턴당 한 번만 프롬프트합니다. 하네스는 컴팩트 JSON을 내보냅니다
# (콜론 뒤에 공백 없음). 이 패턴이 의존하는 것입니다. 공백 허용이 필요하면 jq를 사용하세요.
in=$(cat)
case "$in" in *'"stop_hook_active":true'*) exit 0 ;; esac

d="$CLAUDE_PROJECT_DIR"

# git 저장소가 아님 → 프롬프트할 것이 없습니다.
git -C "$d" rev-parse --git-dir >/dev/null 2>&1 || exit 0

# 원격이 없음 → "원격으로 푸시"는 불가능합니다. 종료합니다.
[ -z "$(git -C "$d" remote 2>/dev/null)" ] && exit 0

# 커밋되지 않은 변경 사항 (스테이징됨, 스테이징되지 않음 또는 추적되지 않음).
# .claude/ 전체 제외 — 운영자 시드 설정 및 CLI 작성 런타임 상태
# (스케줄러 잠금, 작업 트리, 루틴 상태)가 있으며 둘 다
# "커밋되지 않은 작업"이 아니며 모델이 푸시해야 합니다.
s=$(git -C "$d" status --porcelain -- . ':(exclude).claude/' 2>/dev/null)
if [ -n "$s" ]; then
  printf '{"decision":"block","reason":"There are uncommitted changes in the repository. Please commit and push these changes to the remote branch."}'
  exit 0
fi

# 푸시되지 않은 커밋. 모든 원격 추적 ref 또는 FETCH_HEAD에서 도달할 수 없는
# HEAD의 커밋 수를 계산합니다. 이는 다음에 대해 균일하게 작동합니다:
#   - init+fetch 체크아웃 (러너 기본값: FETCH_HEAD만 존재)
#   - 복제 기반 체크아웃 (origin/* 존재)
#   - 러너 기본값: 자식이 세션의 결과 분기에서 시작합니다.
#     러너가 체크아웃 후 생성합니다.
#   - 분리된 HEAD, 사용자 정의 설정이 해당 분기 생성을 건너뛸 때
# 모든 참조 지점이 없으면 (절대 페치되지 않음) 읽기 전용 턴에서
# 거짓 양성을 피하기 위해 침묵합니다.
base=""
git -C "$d" rev-parse --verify -q FETCH_HEAD >/dev/null && base="FETCH_HEAD"
if [ -z "$base" ] && [ -z "$(git -C "$d" for-each-ref --count=1 refs/remotes/origin 2>/dev/null)" ]; then
  exit 0
fi
# shellcheck disable=SC2086  # $base는 "" 또는 "FETCH_HEAD"이며 의도적 단어 분할입니다.
unpushed=$(git -C "$d" rev-list HEAD --not $base --remotes=origin --count 2>/dev/null) || unpushed=0
if [ "$unpushed" -gt 0 ]; then
  branch=$(git -C "$d" symbolic-ref --short -q HEAD)
  if [ -n "$branch" ]; then
    # $branch는 공격자 영향을 받습니다 — git-check-ref-format(1)은 `"`를 허용합니다.
    # ref 이름에서. `\`는 금지됩니다 (규칙 10) 하지만 저렴한 방어 심화로 어쨌든 이스케이프됩니다.
    # 손으로 빌드된 페이로드에 보간하기 전에 JSON 메타문자를 이스케이프하므로
    # x","continue":false와 같은 분기는 하네스가 구문 분석하는 훅 출력 JSON에
    # 키를 주입할 수 없습니다. $unpushed는 안전합니다 — -gt 가드 위는
    # 일반 정수가 아닌 모든 것을 거부합니다.
    branch_esc=$(printf '%s' "$branch" | sed 's/\\/\\\\/g; s/"/\\"/g')
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on branch '\''%s'\''. Please push these changes to the remote repository."}' "$unpushed" "$branch_esc"
  else
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on a detached HEAD. Please create a branch and push it to the remote repository."}' "$unpushed"
  fi
  exit 0
fi

exit 0
```

훅은 세션이 끝나기 전에 Claude에 커밋하고 푸시하도록 프롬프트하며 디렉토리가 git 저장소가 아니거나 원격이 없을 때 침묵합니다.

<h2 id="permissions-and-tool-approval">
  권한 및 도구 승인
</h2>

자체 호스팅 세션에는 연결된 터미널이 없으므로 답변되지 않은 권한 프롬프트는 사용자가 UI에서 응답할 때까지 턴을 정지합니다. Anthropic의 제어 평면은 각 세션의 도구 목록과 권한 규칙을 작업 페이로드와 함께 전송합니다. 기본 구성은 `Bash`를 포함한 일상적인 도구 호출을 사전 승인하며, 클라우드 세션은 [모드에 관계없이 파일 편집을 사전 승인합니다](/docs/ko/permission-modes#switch-permission-modes). 아무것도 사전 승인하지 않은 호출은 세션 UI를 통해 프롬프트됩니다.

<Note>
  자신의 환경에서만 자동 모드를 고정하십시오. 해당 환경의 세션 컨테이너는 [기본 거부 네트워크 이그레스](/docs/ko/self-hosted-environments-deploy#default-deny-egress)로 실행되고 [강화 섹션](/docs/ko/self-hosted-environments-deploy#harden-your-deployment)의 나머지 부분이 적용되어 있어야 합니다. `Bash` 네트워크 요청을 포함한 일상적인 도구 호출은 기본 사전 승인 도구 세트와 자동 모드 모두에서 인간의 개입 없이 실행되므로 네트워크 경계가 이러한 호출이 도달할 수 있는 위치를 제한합니다.
</Note>

제어 평면이 무엇을 전송하든 프롬프트를 최소화하려면 래퍼 스크립트 또는 [`command` 훅](#command)에서 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 고정하십시오. 자동 모드를 사용하면 세션이 일상적인 권한 프롬프트 없이 실행될 수 있습니다. 별도의 분류기 모델이 실행 전에 작업을 검토하고 거부하는 작업을 차단하며, 명시적 요청 규칙은 여전히 프롬프트를 강제합니다. 권한 모드 페이지에서 분류기가 확인하는 내용을 다룹니다. 러너는 래퍼를 호출하기 전에 서버 계산 플래그를 추가하며, `--permission-mode`와 같은 단일 값 플래그의 경우 파서는 마지막 발생을 인정하므로 `"$@"` 뒤에 추가하는 플래그는 서버 전송 값을 재정의합니다:

```bash theme={null}
#!/bin/bash
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --permission-mode auto
```

대신 특정 도구를 사전 승인하려면 `--allowed-tools`를 규칙과 함께 추가하십시오. 예를 들어 `--allowed-tools "Bash(bazel *) Bash(yarn *) mcp__internal__*"`입니다. `--allowed-tools` 및 `--disallowed-tools`와 같은 목록 플래그는 재정의하지 않고 발생 전체에 누적되므로 규칙은 제어 평면이 전송하는 모든 규칙 위에 적용됩니다. 좁히려면 `--disallowed-tools`를 추가하십시오. 이는 다른 규칙이 도구를 허용하더라도 도구를 거부합니다.

<h3 id="how-each-session’s-config-is-assembled">
  각 세션의 구성이 조합되는 방식
</h3>

러너는 각 세션에 자신의 구성 디렉토리를 제공하며, 런타임 시작 시 러너가 캡처하는 호스트의 `~/.claude/`의 스냅샷에서 시드됩니다: `settings.json`, `CLAUDE.md`, 훅, 에이전트, 명령 및 러너 이미지의 스킬은 사용자 수준 기준선으로 모든 세션에 적용됩니다. 실행 중인 호스트의 구성을 변경하면 러너를 재시작한 후에만 변경 사항이 적용됩니다.

`SELF_HOSTED_RUNNER_HOST_CONFIG_DIR`을 설정하여 다른 경로에서 시드하거나 빈 디렉토리를 가리켜 시딩을 비활성화하십시오.

저장소 커밋된 `.claude/settings.json`은 프로젝트 설정으로 위에 계층화됩니다. 세션은 또한 러너 이미지의 표준 시스템 경로에서 [`managed-settings.json`](/docs/ko/settings#where-settings-live)을 읽습니다. 해당 키가 [서버 관리 설정](/docs/ko/server-managed-settings)과 함께 적용되는지 여부는 [Claude Code가 관리되는 소스를 결합하는 방식](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)을 따릅니다. 기본적으로 조직이 서버 관리 키를 제공할 때 세션은 [Claude Code가 모든 관리자 소스에서 읽는 키](/docs/ko/managed-settings#keys-read-from-every-admin-source)(예: `env` 블록, 샌드박스 잠금, 샌드박스 바이너리 경로 및 `forceRemoteSettingsRefresh`)를 제외하고 러너 이미지의 파일을 무시합니다. [설정 우선순위](/docs/ko/settings#settings-precedence)를 참조하십시오.

Anthropic의 제어 평면이 세션에 [Claude Code 훅](/docs/ko/hooks)을 제공할 때 러너는 자신의 구성 위에 설치하지 않고 자신의 구성과 함께 설치합니다. Claude Code v2.1.229 이상이 필요합니다.

* **설치 위치**: 러너는 제공된 각 훅 스크립트를 세션의 구성 디렉토리의 예약된 `hooks/.ccr-launcher/` 하위 디렉토리에 작성하고 스크립트를 `--settings`로 세션에 전달하는 별도의 설정 파일에 등록하여 시드된 `settings.json`과 `hooks/<name>`의 자신의 스크립트를 그대로 둡니다. 러너는 각 세션에 대해 예약된 하위 디렉토리를 다시 생성하며 `~/.claude/hooks/.ccr-launcher/`의 호스트 콘텐츠를 세션으로 시드하지 않습니다.
* **작성자**: 제어 평면은 세션별 또는 타사 입력이 아닌 자신의 배포의 고정 상수에서 스크립트를 채웁니다.
* **여전히 이를 관리하는 것**: `--settings`를 통해 제공된 훅은 관리되는 계층이 아닌 일반 병합 훅 구성에 들어가므로 관리되는 설정이 여전히 적용됩니다. `disableAllHooks`는 이를 비활성화하며, [`allowManagedHooksOnly`](/docs/ko/settings-reference#allowmanagedhooksonly)가 로드된 상태로 유지하는 범주에 포함되지 않습니다.

<h3 id="repository-committed-permission-rules">
  저장소 커밋된 권한 규칙
</h3>

저장소 커밋된 `permissions.allow`에 베어 `"Edit"`, `"Write"` 또는 `"NotebookEdit"` 항목을 넣지 마십시오. 베어 파일 도구 규칙은 경로에 관계없이 도구와 일치하여 작업 공간만이 아닌 호스트의 어디서나 쓰기를 허용하므로 러너의 쓰기 범위 제한 가드는 세션에 플래그를 지정합니다. [`--confine-repo-settings enforce`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)를 사용하면 로깅하고 계속하는 대신 세션 생성을 거부합니다. [강화 섹션](/docs/ko/self-hosted-environments-deploy#harden-your-deployment)을 참조하십시오.

저장소는 파일 도구 규칙이 전혀 필요하지 않습니다. 클라우드 세션은 [모드에 관계없이 파일 편집을 사전 승인합니다](/docs/ko/permission-modes#switch-permission-modes). 규칙을 커밋하는 경우 작업 공간으로 범위를 지정하십시오. 예를 들어 `"Edit(/**)"`입니다. 단일 선행 슬래시는 프로젝트 루트(세션의 작업 공간)에 상대적입니다. 베어 파일 도구 규칙은 해당 파일이 저장소 커밋되지 않으므로 운영자의 호스트 수준 `settings.json`에서 문제가 없습니다.

`defaultMode`의 `auto`는 이미지 전체 또는 사용자 수준 설정 파일에서만 인정되므로 체크아웃된 저장소는 자신에게 자동 모드를 부여할 수 없습니다. 클라우드 세션이 허용하는 모드와 전체 규칙 구문은 [권한 모드](/docs/ko/permission-modes)를 참조하십시오.

<h2 id="what’s-next">
  다음 단계
</h2>

* [참조](/docs/ko/self-hosted-environments-reference): 모든 CLI 플래그, 환경 변수 및 메트릭
* [세션 ID 확인](/docs/ko/self-hosted-environments-identity): 러너 외부 서비스에서 세션 토큰 검증
