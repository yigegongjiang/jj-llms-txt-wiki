> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 자체 호스팅 환경 참조

> 자체 호스팅 러너 및 오케스트레이터에 대한 완전한 참조: CLI 플래그, 환경 변수 및 Prometheus 메트릭.

<Note>
  자체 호스팅 환경은 Team 및 Enterprise 플랜에서 공개 베타 상태이며, [Owner](/docs/ko/cloud-environments#organization-shared-environments)가 [**Cloud environments** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)에서 **Allow self-hosted environments**를 켜서 활성화합니다. 이 페이지는 플래그 및 메트릭 참조이며, 설정은 [빠른 시작](/docs/ko/self-hosted-environments-quickstart)을 참조하고 프로덕션 배포는 [Deploy to production](/docs/ko/self-hosted-environments-deploy)에서 플릿 레시피를 참조하세요.
</Note>

이 페이지는 [자체 호스팅 환경](/docs/ko/self-hosted-environments)에서 실행하는 두 가지 프로세스에 대한 참조입니다: 호스트에서 Claude Code [클라우드 세션](/docs/ko/claude-code-on-the-web)을 실행하는 러너와 세션이 대기열에 있을 때 러너를 시작하는 선택적 자동 스케일링 오케스트레이터입니다. 각각 자체 플래그 테이블이 있습니다. 둘 다 Linux 또는 macOS 호스트에서 실행되며, `/workspace` 및 `~/.claude`와 같은 기본값을 가정합니다. 설치된 버전에서 권한 있는 목록을 보려면 `claude self-hosted-runner --help`를 실행하세요.

메트릭 시리즈 및 일부 API 필드는 여전히 이 페이지에서 환경이라고 부르는 것을 `pool`이라고 사용합니다. 두 용어 모두 동일한 것을 나타냅니다. 환경 ID는 `pool_id` 필드이며, `ccpool_...` 형식입니다: 이 페이지에서 `pool` 식별자를 표시하는 곳마다 환경을 나타냅니다. CLI 플래그 및 환경 변수는 `--environment-secret-file`과 같이 `environment`로 표기합니다. 더 이상 사용되지 않는 `pool` 표기법은 여전히 작동하며, [`--environment-secret-file` 행](#runner-cli-flags)에서 설명합니다.

<h2 id="runner-cli-flags">
  Runner CLI 플래그
</h2>

대부분의 플래그에는 해당하는 환경 변수가 있습니다. 둘 다 설정된 경우 플래그가 우선합니다. 기간 플래그는 CLI에서 분 또는 초를 사용하지만, 쌍을 이루는 환경 변수는 항상 밀리초 단위이며 `_MS` 접미사로 표시되고, 기본값 열은 플래그의 단위를 표시합니다: `--exit-if-unused-min 10`은 `SELF_HOSTED_RUNNER_IDLE_SHUTDOWN_MS=600000`과 동일하며, `SELF_HOSTED_RUNNER_STARTUP_TIMEOUT_MS: "15"`와 같은 Helm 값은 15분 기본값이 아닌 15밀리초를 의미합니다.

| 플래그                                       | 환경 변수                                             | 기본값                         | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :---------------------------------------- | :------------------------------------------------ | :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--api-url <url>`                         | 없음                                                | `https://api.anthropic.com` | API 기본 URL입니다. 테스트 목적으로만 재정의하십시오.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `--base-dir <path>`                       | `SELF_HOSTED_RUNNER_BASE_DIR`                     | `/workspace`; Windows에서는 없음 | 저장소 체크아웃 및 세션별 작업 디렉터리용 디렉터리입니다. 러너는 이 경로 또는 상위 경로에 대한 쓰기 액세스 권한이 필요합니다. 러너는 시작 시 디렉터리를 생성하고 생성하거나 쓸 수 없을 때 `cannot create or write to base directory`로 종료됩니다. v2.1.225 이전에는 러너가 첫 번째 세션이 시작될 때 디렉터리를 생성했으므로 사용할 수 없는 경로로 인해 시작이 아닌 세션이 실패했습니다. Windows는 지원되지 않는 러너 호스트이므로 기본값이 없습니다: 플래그를 전달하거나 변수를 설정하지 않으면 러너가 시작 시 종료됩니다. 환경의 모든 러너에서 동일한 값을 사용하십시오. [러너 간 기본 디렉터리 및 용량 동일하게 유지](/docs/ko/self-hosted-environments-deploy#keep-the-base-directory-and-capacity-identical-across-runners)를 참조하십시오.                                                                                                                                                |
| `--capacity <n>`                          | 없음                                                | `1`                         | 이 러너가 처리하는 최대 동시 세션 수입니다. 모든 세션은 동일한 잠금된 [소유자](/docs/ko/self-hosted-environments#key-concepts)에 속합니다. 환경의 모든 러너에서 동일한 값을 사용하십시오. [러너 간 기본 디렉터리 및 용량 동일하게 유지](/docs/ko/self-hosted-environments-deploy#keep-the-base-directory-and-capacity-identical-across-runners)를 참조하십시오.                                                                                                                                                                                                                                                                                                                                                                             |
| `--client-label <label>`                  | `SELF_HOSTED_RUNNER_CLIENT_LABEL`                 | 호스트의 호스트명                   | 러너가 등록할 때 전송하는 레이블입니다. 러너는 또한 이를 [`claude_code_self_hosted_runner_info`](#prometheus-metrics)의 `client_label` 레이블로 보고합니다. Claude Code v2.1.248 이상이 필요합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `--configure-git`                         | `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`              | 꺼짐                          | 시작 시 전역 git 신원을 작성하고, Anthropic 커밋 서명을 활성화하고, git push 협상을 켜고, `Co-authored-by:` 트레일러를 추가하는 커밋 훅을 설치합니다. Push 협상에는 Claude Code v2.1.257 이상이 필요합니다. [git 구성](/docs/ko/self-hosted-environments-deploy#configure-git)을 참조하십시오.                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `--confine-repo-settings <mode>`          | `SELF_HOSTED_RUNNER_CONFINE_REPO_SETTINGS`        | `warn`                      | 저장소의 커밋된 설정이 해당 세션의 자체 작업 공간 외부에 대한 쓰기 또는 읽기 액세스 권한을 부여하거나, 환경 변수를 설정하거나, 운영자의 샌드박스 또는 훅 상태를 재정의하려고 할 때 세션에 플래그를 지정하는 가드의 모드를 설정합니다(예: `sandbox.enabled: false` 또는 `disableAllHooks`). 기본값 `warn`은 위반을 기록하고 여전히 세션을 시작하고, `enforce`는 세션을 거부하며, `off`는 스캔을 비활성화합니다. [배포 강화](/docs/ko/self-hosted-environments-deploy#harden-your-deployment)를 참조하십시오.                                                                                                                                                                                                                                                                                                 |
| `--debug-token-dir <path>`                | `SELF_HOSTED_RUNNER_DEBUG_TOKEN_DIR`              | 설정되지 않음                     | 검사를 위해 라이브 토큰을 디스크에 씁니다. 디버그 전용이며 프로덕션에서 사용하지 마십시오.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `--defer-shutdown-max-min <n>`            | `SELF_HOSTED_RUNNER_DEFER_SHUTDOWN_MAX_MS`        | `0`                         | 첫 번째 `SIGTERM` 또는 `SIGINT`에서 드레인하는 대신 이미 연결된 세션을 계속 제공한 다음 N분 후에 여전히 연결된 모든 것을 해제하고 종료합니다. 이를 설정하기 전에 호스트의 중지 타임아웃을 높이십시오. [첫 번째 신호 이후 드레인 연기](/docs/ko/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal)를 참조하십시오. `0`은 비활성화합니다. Claude Code v2.1.238 이상이 필요합니다.                                                                                                                                                                                                                                                                                                                                                                |
| `--drain-grace-sec <n>`                   | `SELF_HOSTED_RUNNER_DRAIN_GRACE_MS`               | `0`                         | 러너가 종료 신호를 받거나 은퇴 시간에 도달할 때까지, 활성 세션이 완료된 후 러너가 종료되는 시기를 제어합니다: `0`은 더 이상 폴링하지 않고 즉시 종료되고, 양수 값은 러너를 활성 상태로 유지하고 잠금된 소유자의 큐를 그 많은 초 동안 다시 폴링하며, 이는 [강화 섹션](/docs/ko/self-hosted-environments-deploy#harden-your-deployment)에 설명된 세션별 컨테이너 격리의 비용이 발생합니다. [`--defer-shutdown-max-min`](/docs/ko/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal)으로 연기한 첫 번째 신호 이후, 러너는 세션을 보유하지 않는 즉시 종료되며, 여기서 설정한 내용과 관계없이 종료됩니다.                                                                                                                                                                                                                  |
| `--drain-marker-file <path>`              | `SELF_HOSTED_RUNNER_DRAIN_MARKER_FILE`            | 설정되지 않음                     | 호스트가 `SIGTERM`을 보내기 전에 우아한 드레인을 알리기 위해 작성하는 마커 파일입니다. 드레인이 시작될 때 파일이 존재하면 러너는 종료를 일반 종료 신호가 아닌 호스트 드레인으로 Anthropic에 보고합니다. 드레인 자체(플래그 없이 `--drain-wait-sec` 보유 포함)는 동일하게 실행됩니다. 세션이 쓸 수 없는 로컬 파일 시스템의 경로를 지정하십시오. Claude Code v2.1.271 이상이 필요합니다.                                                                                                                                                                                                                                                                                                                                                                                               |
| `--drain-wait-sec <n>`                    | `SELF_HOSTED_RUNNER_DRAIN_WAIT_MS`                | `0`                         | 드레인이 시작되면([`--defer-shutdown-max-min`](/docs/ko/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal)을 설정하지 않은 경우 `SIGTERM` 수신 시), 각 세션의 진행 중인 턴과 백그라운드 작업이 완료될 때까지 최대 N초를 기다린 후 자식을 종료합니다. 이 대기 중에 러너는 방금 완료된 백그라운드 작업을 여전히 실행 중인 것으로 계산하며, 최대 [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](#environment-variable-only-settings) 윈도우 동안 결과를 읽는 후속 턴이 시작될 때까지입니다.                                                                                                                                                                                                                                                                      |
| `--environment-secret-file <path>`        | `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET`           | 필수                          | 환경 비밀을 포함하는 파일의 경로이거나, [오케스트레이터](/docs/ko/self-hosted-environments-configuration#on-demand-runners)에 의해 생성된 러너의 경우 일회용 작업 주문 JWT입니다. `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET`은 파일 경로가 아닌 비밀 값을 직접 전달합니다. 더 이상 사용되지 않는 `--pool-secret-file` 플래그 및 `SELF_HOSTED_RUNNER_POOL_SECRET` 변수는 여전히 작동하며 stderr에 지원 중단 알림을 인쇄합니다. 2.1.216보다 오래된 미리보기 프로그램 러너 빌드는 이러한 더 이상 사용되지 않는 이름만 인식합니다.                                                                                                                                                                                                                                                                    |
| `--exec-path <path>`                      | `SELF_HOSTED_RUNNER_EXEC_PATH`                    | 자체 바이너리                     | 각 세션에 대해 생성할 바이너리 또는 래퍼 스크립트입니다. [래퍼 스크립트](/docs/ko/self-hosted-environments-configuration#wrapper-scripts)를 참조하십시오.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `--exit-if-unused-min <n>`                | `SELF_HOSTED_RUNNER_IDLE_SHUTDOWN_MS`             | `0`                         | 작업이 할당되지 않은 N분 동안 폴링한 후 종료합니다(자동 스케일러 축소용). `0`은 비활성화합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `--git-host-rewrite <from>=<to>`          | 없음                                                | 설정되지 않음                     | 분할 수평 DNS를 위해 복제하기 전에 `https://<from>/...` 소스 URL을 `https://<to>/...`로 다시 작성합니다. 반복 가능; 플래그만 해당.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `--git-ssh-rewrite <host>`                | 없음                                                | 설정되지 않음                     | SSH 전용 git 호스트를 위해 복제하기 전에 `https://<host>/...` 소스 URL을 `git@<host>:...`로 다시 작성합니다. 반복 가능; 플래그만 해당.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `--health-port <port>`                    | `SELF_HOSTED_RUNNER_HEALTH_PORT`                  | `8080`                      | `/healthz` 및 `/metrics` 리스너용 포트입니다. 비활성화하려면 `0`으로 설정하십시오.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `--hooks-dir <path>`                      | `SELF_HOSTED_RUNNER_HOOKS_DIR`                    | 설정되지 않음                     | 라이프사이클 훅 스크립트의 디렉터리입니다. [라이프사이클 훅](/docs/ko/self-hosted-environments-configuration#lifecycle-hooks)을 참조하십시오.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `--host-config-snapshot <mode>`           | `SELF_HOSTED_RUNNER_HOST_CONFIG_SNAPSHOT`         | `disk`                      | 러너가 각 세션을 시드하는 [호스트 구성 디렉터리](#environment-variable-only-settings)의 시작 스냅샷을 유지하는 위치입니다. `disk`는 스냅샷을 `--base-dir` 아래의 러너 소유 디렉터리로 복사하고, 각 세션 시작 시 모든 파일을 메모리 내 다이제스트에 대해 확인합니다. 복사본의 파일이 수정되었으면 세션이 실패하고 러너는 재시작할 때까지 세션을 거부합니다. `memory`는 전체 스냅샷을 힙에 보유하며, 64 MiB로 제한됩니다. 상한을 초과하면 세션이 호스트 구성 없이 시작되고 그렇게 함을 나타내는 알림을 표시합니다. 러너가 디스크 스냅샷을 쓸 수 없으면 실패를 기록하고 해당 실행에 `memory`를 사용합니다. Claude Code v2.1.271 이상이 필요합니다.                                                                                                                                                                                                                              |
| `--kill-session-after-min <n>`            | `SELF_HOSTED_RUNNER_MAX_LIFETIME_MS`              | `0`                         | 세션을 N분 벽시계 시간으로 제한합니다(고착된 세션의 안전 제한). v2.1.260 이상에서 러너는 제한에 도달한 세션을 해제하여 사용자의 다음 메시지에서 재개할 수 있으며, [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](#environment-variable-only-settings) 유예 윈도우가 끝날 때 여전히 러너에 있는 경우에만 종료합니다. v2.1.260 이전에는 러너가 제한에서 세션을 종료했습니다. 세부 정보 및 값을 선택하는 방법은 [일부 세션은 유휴로 계산되지 않음](/docs/ko/self-hosted-environments-deploy#some-sessions-don%E2%80%99t-count-as-idle)을 참조하십시오. `0`은 비활성화합니다.                                                                                                                                                                                                                                       |
| `--lock-to-account <id>`                  | `SELF_HOSTED_RUNNER_LOCK_TO_ACCOUNT`              | 설정되지 않음                     | 첫 번째 세션에서 잠금하는 대신 시작 시 러너를 특정 계정으로 사전 잠금합니다. 환경의 조직에서 이메일 주소 또는 `user_...` ID를 허용합니다. 사전 잠금된 러너는 계정이 없는 Claude Tag 채널 세션을 절대 선택하지 않습니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `--log-file <path>`                       | `SELF_HOSTED_RUNNER_LOG_FILE`                     | 설정되지 않음                     | 러너 로그를 stdout 및 stderr 외에도 파일로 미러링하며, `0600` 권한으로 생성됩니다. `self-hosted-runner doctor`가 로그를 로컬로 추적하는 데 필요합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `--log-level <level>`                     | 없음                                                | `info`                      | `info` 또는 `debug`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `--post-session-hook-timeout-sec <n>`     | `SELF_HOSTED_RUNNER_POST_SESSION_HOOK_TIMEOUT_MS` | `60`                        | 러너 종료를 포함한 모든 세션 종료 시 [`post-session` 훅](/docs/ko/self-hosted-environments-configuration#post-session)의 예산입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `--proxy-authorization-command <command>` | `SELF_HOSTED_RUNNER_PROXY_AUTHORIZATION_COMMAND`  | 설정되지 않음                     | 러너가 이그레스 프록시에 대한 모든 연결에 대해 실행하는 셸 명령이며, 트리밍된 stdout을 `Proxy-Authorization` 헤더 값으로 사용합니다. `HTTPS_PROXY` 또는 `HTTP_PROXY`가 필요하며, `--proxy-authorization-file`과 결합할 수 없습니다. [이그레스 프록시에 인증](/docs/ko/self-hosted-environments-deploy#authenticate-to-an-egress-proxy)을 참조하십시오. Claude Code v2.1.238 이상이 필요합니다.                                                                                                                                                                                                                                                                                                                                            |
| `--proxy-authorization-file <path>`       | `SELF_HOSTED_RUNNER_PROXY_AUTHORIZATION_FILE`     | 설정되지 않음                     | 러너가 이그레스 프록시에 대한 모든 연결에 대해 읽는 파일이며, 트리밍된 내용을 `Proxy-Authorization` 헤더 값으로 사용합니다. 다른 프로세스가 제자리에서 회전하는 토큰에 이 플래그를 사용하십시오. `--proxy-authorization-command`와 동일한 요구 사항을 전달하며, 이와 결합할 수 없습니다. [이그레스 프록시에 인증](/docs/ko/self-hosted-environments-deploy#authenticate-to-an-egress-proxy)을 참조하십시오. Claude Code v2.1.238 이상이 필요합니다.                                                                                                                                                                                                                                                                                                                           |
| `--push-outcome-on-release`               | `SELF_HOSTED_RUNNER_PUSH_OUTCOME_ON_RELEASE`      | 꺼짐                          | 드레인 또는 유휴 해제와 같은 러너 시작 세션 종료 시 추적된 결과 분기를 `origin`으로 푸시한 후 작업 공간을 삭제하여 진행 중인 커밋이 재시작을 생존하도록 합니다. 최선의 노력; 종료 예산에 30초를 추가하며, 푸시된 분기에서 재개하려면 git 2.29 이상이 필요합니다. 활성화하기 전에 `claude/*` refs에 대한 푸시 액세스를 제한하십시오. [재개된 세션이 푸시되지 않은 작업을 잃음](/docs/ko/self-hosted-environments-deploy#additional-limitations)을 참조하십시오. `checkout` 라이프사이클 훅을 통해 체크아웃된 저장소는 푸시되지 않습니다. [`post-session` 훅](/docs/ko/self-hosted-environments-configuration#post-session)에서 이를 스냅샷하십시오.                                                                                                                                                                                                |
| `--release-idle-session-min <n>`          | `SELF_HOSTED_RUNNER_SESSION_IDLE_MS`              | `0`                         | 턴이 완료되거나 세션이 사용자의 작업을 기다린 후 N분의 비활성 상태 후 세션 슬롯을 해제합니다. 여전히 턴 중간에 있는 세션(절대 완료되지 않는 백그라운드 작업을 보유하거나 실행 중인 도구 호출 내에서 요청된 승인을 포함)은 유휴로 계산되지 않습니다. `--kill-session-after-min`과 쌍을 이루어 하드 백스톱으로 사용하십시오. 세션의 백그라운드 작업이 완료된 후, 러너는 최대 [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](#environment-variable-only-settings) 윈도우 동안 결과를 읽는 후속 턴이 시작될 때까지 세션을 바쁜 것으로 간주합니다. 러너가 종료 신호를 받거나 은퇴 시간에 도달할 때까지, 러너를 활성 세션 없이 남겨두는 해제는 `--drain-grace-sec`에 의해 관리되는 정상 드레인과 동일한 종료 경로를 시작합니다. [`--defer-shutdown-max-min`](/docs/ko/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal)으로 연기한 첫 번째 신호 이후, 러너는 해제로 인해 세션을 보유하지 않는 즉시 종료됩니다. `0`은 비활성화합니다. |
| `--remove-session-state [bool]`           | `SELF_HOSTED_RUNNER_REMOVE_SESSION_STATE`         | 꺼짐                          | 세션이 이 러너에서 종료될 때 결과와 관계없이 `<base-dir>/_sessions/` 아래의 세션별 디렉터리를 제거합니다. [사전 준비된 체크아웃 재사용](/docs/ko/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout)은 이들이 보유한 내용과 유지될 때 이를 읽을 수 있는 사람을 설명합니다. 제거는 최선의 노력입니다: 러너가 종료되거나 정리가 실행되기 전에 드레인 마감에 도달하면 세션별 디렉터리가 제자리에 남아 있습니다. 플래그가 켜져 있으면 실패하거나 중단된 세션의 디버그 로그가 디스크에 유지되지 않습니다. Claude Code v2.1.268 이상이 필요합니다.                                                                                                                                                                                                                                                                                  |
| `--retire-at <epoch-seconds>`             | `SELF_HOSTED_RUNNER_RETIRE_AT`                    | 설정되지 않음                     | 러너가 알려진 시간에 종료되는 인프라를 위해 절대 Unix 타임스탬프(초)에서 러너를 은퇴시킵니다. [러너 라이프사이클](/docs/ko/self-hosted-environments#runner-lifecycle)은 해제 시퀀스 및 여유를 크기 조정하는 방법을 설명합니다. 2001 이전 또는 5138년 이후의 값은 플래그에 의해 거부되고 환경 변수에 의해 무시됩니다.                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `--session-stop-grace-sec <n>`            | `SELF_HOSTED_RUNNER_SESSION_STOP_GRACE_MS`        | `5`                         | 세션이 종료된 후 Claude 프로세스가 깔끔하게 종료될 때까지 기다린 후 강제 종료하기 전까지 기다리는 시간입니다. 자식의 자체 `SessionEnd` 훅에 더 많은 시간이 필요한 경우 값을 높이십시오.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `--startup-timeout-min <n>`               | `SELF_HOSTED_RUNNER_STARTUP_TIMEOUT_MS`           | `15`                        | 자식이 생성 후 N분 이내에 [활동 채널](/docs/ko/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached)에서 초기화되었음을 신호하지 않은 경우 세션 슬롯을 해제합니다. 일반 출력이 아닌 자식의 초기화 신호로 지워지며, 그 후 `--release-idle-session-min`이 인수합니다. `0`은 비활성화합니다.                                                                                                                                                                                                                                                                                                                                                                                                             |
| `--trust-workspace [bool]`                | `SELF_HOSTED_RUNNER_TRUST_WORKSPACE`              | 켜짐                          | 각 세션의 저장소 경로에 대해 지속된 신뢰를 시드하여 저장소 커밋된 `permissions.allow` 및 `additionalDirectories`가 준수되도록 합니다. 저장소 커밋된 권한 부여를 삭제하고 대신 호스트 구성의 `settings.json`에서 허용 규칙을 구성하려면 `false`로 설정하십시오. 저장소 커밋된 `sandbox.*` 설정은 여전히 어느 쪽이든 적용되며, 이것이 [저장소 설정 가드](/docs/ko/self-hosted-environments-deploy#harden-your-deployment)가 이 플래그와 관계없이 이를 스캔하는 이유입니다.                                                                                                                                                                                                                                                                                                                 |
| `--use-anthropic-git-proxy`               | `CLAUDE_RUNNER_USE_GIT_PROXY=1`                   | 꺼짐                          | 고객 관리 git 인증 대신 [Anthropic git 프록시](/docs/ko/self-hosted-environments-deploy#use-the-anthropic-git-proxy)를 통해 복제합니다. `--capacity 1` 및 git 2.32 이상이 필요합니다. 러너는 그렇지 않으면 시작을 거부합니다. 다시 작성 플래그를 대체합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

대부분의 기간 플래그에는 최대값이 있으며, 각 타임아웃을 런타임의 32비트 타이머 상한인 약 24.85일 이내로 유지하도록 선택됩니다. `--*-min` 플래그는 10080분(7일)에서 상한선을 설정합니다. `--drain-grace-sec`는 604800초(역시 7일)에서 상한선을 설정합니다. `--drain-wait-sec`는 86400초(24시간)에서 상한선을 설정합니다. `--session-stop-grace-sec` 및 `--post-session-hook-timeout-sec`는 상한선이 없습니다. 상한선을 초과하는 동작은 표면별로 다릅니다:

* **플래그**: 시작이 오류로 실패합니다.
* **환경 변수**: 러너는 값을 거부하는 대신 타이머 상한선으로 고정합니다.

<h2 id="orchestrator-cli-flags">
  오케스트레이터 CLI 플래그
</h2>

`self-hosted-runner orchestrator` 서브명령은 [on-demand runners](/docs/ko/self-hosted-environments-configuration#on-demand-runners)를 생성하며, `--api-url`, `--environment-secret-file`, `--hooks-dir`, `--health-port` 및 `--log-level`을 러너와 동일한 기본값으로 허용하며, 러너의 플래그에 하나가 있는 경우 동일한 환경 변수를 허용합니다. 단, `--hooks-dir`은 필수이며 `spawn-runner` 훅을 포함해야 합니다. 또한 자체 플래그를 사용합니다:

| 플래그                              | 기본값     | 설명                                                                                                                                  |
| :------------------------------- | :------ | :---------------------------------------------------------------------------------------------------------------------------------- |
| `--hook-concurrency <n>`         | `4`     | 병렬로 실행되는 최대 `spawn-runner` 훅입니다. 또한 폴링당 청구되는 스폰 요청 수를 제한합니다.                                                                        |
| `--hook-timeout <sec>`           | `60`    | 이 많은 초 후 훅의 프로세스 트리를 종료합니다. 타임아웃과 5초 킬 유예는 `--expected-spawn-seconds` 아래에 있어야 합니다. 오케스트레이터는 시작 시 이를 적용합니다.                          |
| `--expected-spawn-seconds <sec>` | `120`   | 생성된 러너의 예상 p99 부팅 시간(서버 적용 범위 10\~3600). 모든 폴링에서 서버 측 임차로 전송됩니다. 경과하기 전에 러너가 등록되지 않으면 세션이 새 주문 ID로 다시 제공됩니다. 모든 복제본은 이 값을 공유해야 합니다. |
| `--min-idle <n>`                 | `0`     | 대기 중인 러너를 사전에 생성하여 최소 N개의 유휴 세션 슬롯을 무료로 유지합니다. `0`은 사전 워밍을 비활성화합니다. 러너의 `--exit-if-unused-min`과 쌍을 이루어 잉여 대기 러너가 자신을 회수하도록 합니다.     |
| `--debug-dir <path>`             | 설정되지 않음 | 각 스폰 요청의 작업 주문 및 훅 stderr을 디스크에 씁니다. 디버그 전용이며 프로덕션에서 설정하지 마세요.                                                                      |

<h3 id="scm-connector-flags">
  SCM 커넥터 플래그
</h3>

오케스트레이터는 Anthropic의 제어 평면에 대한 상시 WebSocket 연결을 유지할 수 있으므로 저장소 선택기 및 분기 또는 ref 리졸버와 같은 호스팅된 사전 세션 흐름이 네트워크 내부에서만 라우팅 가능한 GitHub Enterprise Server 호스트에 도달할 수 있습니다. 커넥터는 `--scm-connector-host`를 설정하지 않으면 꺼져 있습니다.

| 플래그                                                     | 기본값                        | 설명                                                                                    |
| :------------------------------------------------------ | :------------------------- | :------------------------------------------------------------------------------------ |
| `--scm-connector-host <host[:port]>`                    | 설정되지 않음                    | 요청을 전달할 GitHub Enterprise Server 호스트명입니다. 포트는 기본값이 `443`입니다. 이 플래그를 설정하면 커넥터가 활성화됩니다. |
| `--scm-connector-id <n>`                                | `--scm-connector-host`와 필수 | 조직의 GitHub Enterprise Server 연결의 숫자 ID입니다. 커넥터를 활성화할 때 Anthropic 계정 팀에 문의하여 값을 얻으세요.  |
| `--scm-connector-provider <slug>`                       | `ghe`                      | 공급자를 식별하는 경로 세그먼트이며, `^[a-z0-9-]{1,32}$`와 일치합니다.                                      |
| `--scm-connector-ca-file <path>`                        | 설정되지 않음                    | GitHub Enterprise Server 호스트에 대한 TLS 연결을 위한 추가 CA 번들(PEM 형식)입니다.                      |
| `--scm-connector-host-rewrite <from>=<to_host:to_port>` | 설정되지 않음                    | 엔드투엔드 테스트 전용: Host 헤더 및 TLS SNI를 `--scm-connector-host`로 유지하면서 TCP 연결을 리디렉션합니다.       |

커넥터는 오케스트레이터의 기존 환경 비밀로 인증하고 자동으로 다시 연결합니다: 끊어진 연결에서 지수 백오프를 사용하거나, 다른 오케스트레이터 복제본이 이미 이를 보유하고 있기 때문에 제어 평면이 연결을 닫을 때 고정 30초 지연입니다.

<h2 id="environment-variable-only-settings">
  환경 변수 전용 설정
</h2>

이러한 러너 설정은 환경에서만 읽으며 대부분의 배포가 기본값으로 남겨두는 동작을 다룹니다:

| 환경 변수                                      | 기본값         | 설명                                                                                                                                                                                                                                                                                                                                                               |
| :----------------------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`    | `30000`     | 러너가 백그라운드 작업이 완료된 후 결과를 읽는 후속 턴이 시작되지 않은 동안 세션을 바쁜 것으로 간주하는 시간입니다. [`--drain-wait-sec` 및 `--release-idle-session-min` 행](#runner-cli-flags)은 드레인 및 유휴 해제에서 보유가 적용되는 위치를 설명하며, [Runner lifecycle](/docs/ko/self-hosted-environments#runner-lifecycle)은 `--retire-at` 은퇴에서 적용되는 위치를 설명합니다. `0` 또는 사용할 수 없는 값은 기본값으로 폴백되므로 보유를 끌 수 없습니다. Claude Code v2.1.228 이상이 필요합니다. |
| `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR`       | `~/.claude` | 러너의 시작 스냅샷으로 캡처되고 각 세션의 `CLAUDE_CONFIG_DIR`로 시드되는 디렉토리입니다. 디스크의 변경 사항은 러너 재시작 후 적용됩니다. 변수를 설정하면 러너가 [MCP seeding](/docs/ko/self-hosted-environments-configuration#mcp-servers)을 위해 `.claude.json`을 읽는 위치도 이동하므로 설정하면 자체 기본값을 포함하여 해당 조회를 재배치합니다. 빈 디렉토리를 가리켜 시딩을 완전히 비활성화하세요.                                                                                         |
| `SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS` | `900000`    | 세션이 `--kill-session-after-min` 제한에 도달한 후, 러너가 세션을 종료하기 전에 실행 중인 턴이 완료되거나 해제가 완료될 때까지 대기하는 시간입니다.                                                                                                                                                                                                                                                                 |
| `SELF_HOSTED_RUNNER_POST_TURN_SETTLE_MS`   | `7000`      | 턴이 완료된 후 세션의 프로세스가 턴의 끝을 Anthropic에 보고하는 동안 러너가 `--drain-wait-sec` 드레인에 대해 세션을 바쁜 것으로 계산하는 시간의 상한입니다. `0` 또는 사용할 수 없는 값은 기본값으로 폴백되므로 보유를 끌 수 없습니다. Claude Code v2.1.275 이상이 필요합니다.                                                                                                                                                                               |
| `SELF_HOSTED_RUNNER_SIGKILL_GRACE_MS`      | `30000`     | 러너가 중단 불가능한 I/O에 갇혀 있는 자식에게 OS가 `SIGKILL`을 전달할 때까지 기다린 후 자신을 종료하기 전까지 기다리는 시간입니다. `--post-session-hook-timeout-sec` 더하기 15초로 바닥이 정해지며, `--push-outcome-on-release`가 설정되면 30초 더 추가되므로 효과적인 최소값은 기본값에서 75초입니다.                                                                                                                                                     |
| `CLAUDE_RUNNER_FETCH_DEPTH`                | `50`        | 신선한 복제를 위한 Git 페치 깊이입니다. 양의 정수 또는 완전한 페치를 위해 `full` 또는 `0`을 설정하세요. 작업 공간에 이미 있는 저장소는 기존 깊이를 유지합니다.                                                                                                                                                                                                                                                               |
| `CLAUDE_RUNNER_SKIP_GIT_VERIFY`            | 설정되지 않음     | `1`일 때, `checkout` 훅이 실행된 후 `.git` 존재 확인을 건너뜁니다. 훅이 비git 소스를 구체화할 때 이를 설정하세요.                                                                                                                                                                                                                                                                                    |
| `FORCE_AUTOUPDATE_PLUGINS`                 | 설정되지 않음     | `1`일 때, 바이너리가 고정되어 있어도 플러그인 마켓플레이스가 자동 업데이트되도록 합니다.                                                                                                                                                                                                                                                                                                              |
| `CLAUDE_CODE_DISABLE_ARTIFACT`             | 설정되지 않음     | `1`일 때, 조직의 관리자 설정과 관계없이 세션에서 Artifact 도구를 비활성화하고 `*.frame.claudeusercontent.com` 이그레스 요구 사항을 삭제합니다.                                                                                                                                                                                                                                                             |

<h2 id="telemetry">
  텔레메트리
</h2>

세션 자식은 끄지 않으면 Anthropic에 운영 텔레메트리를 전송합니다. 코드 또는 저장소 내용은 전송되지 않습니다. 러너 프로세스에서 텔레메트리 변수를 설정하세요. 러너는 서버 제공 환경 변수를 적용한 후 다시 주장하므로 운영자의 설정이 항상 우선합니다.

한 가지 제어는 자체 호스팅 환경에만 해당됩니다: `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1`은 Datadog 운영 메트릭을 옵트인하며, 이는 자체 호스팅 환경에서 기본적으로 꺼져 있습니다. 일반 Claude Code 텔레메트리 제어인 `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_ERROR_REPORTING` 및 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`은 [environment variable reference](/docs/ko/env-vars)에서 문서화된 대로 세션 자식에 적용됩니다. `DISABLE_GROWTHBOOK`은 관련이 있지만 다릅니다: `DISABLE_GROWTHBOOK=1`을 설정하면 기능 플래그 페칭이 비활성화되고, `DISABLE_TELEMETRY`도 설정되지 않으면 텔레메트리가 켜져 있습니다.

`CLAUDE_CODE_ENABLE_TELEMETRY`는 관련이 없습니다: [Monitoring](/docs/ko/monitoring-usage)에서 설명한 대로 자신의 수집기로 OpenTelemetry 내보내기를 활성화하며, Anthropic의 분석을 제어하지 않습니다.

<h2 id="health-endpoint">
  상태 엔드포인트
</h2>

러너는 구성된 상태 포트에서 `GET /healthz`를 제공합니다. 응답은 프로세스가 살아 있을 때마다 `200 OK`이며, 폴 루프가 어떤 상태에 있든 관계없이, 이 엔드포인트의 HTTP 프로브는 죽은 프로세스만 감지합니다. JSON 본문은 현재 상태를 설명합니다:

```json theme={null}
{
  "status": "ok",
  "runner_id": "ccrunner_...",
  "active_sessions": 2,
  "last_poll_at": "2026-03-31T18:04:11.220Z",
  "last_poll_age_ms": 842
}
```

사용자 정의 프로브에서 `last_poll_age_ms`를 생존 신호로 사용하세요. 무한정 증가하는 값은 폴 루프가 고착되었음을 나타냅니다. `last_poll_at` 및 `last_poll_age_ms` 모두 첫 번째 폴이 완료될 때까지 `null`입니다.

오케스트레이터는 상태 포트에서 자체 `/healthz`를 제공합니다. 엔드포인트는 항상 `200`을 반환하며, 본문은 가장 최근 폴이 성공했는지 여부를 보고하는 `connected` 필드와 `queue_counts`의 상태별 스폰 큐 수를 전달합니다. 상태 코드가 아닌 `connected`에 준비 및 경고를 게이트하세요.

[SCM connector](#scm-connector-flags)가 구성되면, 오케스트레이터의 `/healthz` 본문도 `scm_connector_connected` 및 `connected`, `last_connected_at`, `last_error`, `reconnects` 및 `requests_forwarded`를 포함하는 `scm_connector` 객체를 전달합니다. `--scm-connector-host`가 설정되지 않으면 두 필드 모두 `null`입니다.

<h2 id="prometheus-metrics">
  Prometheus 메트릭
</h2>

각 러너는 `/healthz`와 동일한 포트에서 `GET /metrics`에서 Prometheus 메트릭을 제공합니다. 주요 시리즈:

| 시리즈                                                                               | 참고                                                                                                                                                                                                                                                                                                                               |
| :-------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude_code_self_hosted_runner_info{runner_id,version,client_label}`             | 항상 `1`입니다. 플릿 인벤토리 및 버전 드리프트 감지에 유용합니다.                                                                                                                                                                                                                                                                                          |
| `claude_code_self_hosted_runner_capacity`                                         | 구성된 `--capacity`                                                                                                                                                                                                                                                                                                                 |
| `claude_code_self_hosted_runner_active_sessions`                                  | 현재 실행 중인 세션                                                                                                                                                                                                                                                                                                                      |
| `claude_code_self_hosted_runner_locked_account{email}`                            | 러너가 사용자에게 잠금되고 `act.email` 클레임을 전달하는 세션 토큰이 발급된 후 존재합니다. 시리즈는 세션 토큰이 `act.email`을 전달하지 않는 Claude Tag 에이전트에 잠긴 러너에 없습니다. 레이블 값은 계정 이메일입니다. 메트릭 저장소를 광범위하게 읽을 수 있으면 스크래핑 시간에 레이블을 삭제하거나 해시하세요(예: Prometheus `metric_relabel_configs`).                                                                                             |
| `claude_code_self_hosted_runner_last_poll_age_seconds`                            | 마지막 성공적인 폴 이후 초입니다. 60 이상이면 경고하세요.                                                                                                                                                                                                                                                                                               |
| `claude_code_self_hosted_runner_poll_errors_total{error_kind}`                    | 종류별 누적 PollWork 실패: `transport`, `timeout`, `5xx`, `429` 또는 `4xx`. 모든 5개 시리즈는 프로세스 시작부터 존재합니다. `rate(...[5m]) > 0`에서 경고하세요.                                                                                                                                                                                                      |
| `claude_code_self_hosted_runner_sessions_started_total{client_platform}`          | 러너의 수명 동안 생성된 세션 자식 프로세스이며, `web_claude_ai`, `ios`, `android`, `desktop_app` 또는 `claude_code_cli`와 같은 세션 원점당 하나의 시리즈 또는 서버가 하나를 전송하지 않을 때 `unknown`입니다. Slack 세션은 Slack 통합을 생성한 것에 따라 `claude_in_slack` 또는 `claude-in-slack`을 전달하므로 `{client_platform=~"claude[-_]in[-_]slack"}`과 같은 정규식 선택기로 둘 다 일치시키세요. 플릿 총계에 `sum()`을 사용하세요. |
| `claude_code_self_hosted_runner_sessions_completed_total{client_platform}`        | 깔끔하게 종료된 세션이며, 동일한 방식으로 레이블이 지정됩니다. 일반 깔끔한 종료보다 광범위합니다: [session lifecycle counter semantics](#session-lifecycle-counter-semantics)에서 계산되는 것을 참조하세요.                                                                                                                                                                             |
| `claude_code_self_hosted_runner_sessions_failed_total{client_platform}`           | 실패로 종료된 세션이며, 동일한 방식으로 레이블이 지정됩니다. 동일한 주의: [session lifecycle counter semantics](#session-lifecycle-counter-semantics)를 참조하세요.                                                                                                                                                                                                   |
| `claude_code_self_hosted_runner_sessions_interrupted_total{client_platform}`      | 러너가 세션 결과가 아닌 운영 이유로 종료한 세션이며, 동일한 방식으로 레이블이 지정됩니다. [session lifecycle counter semantics](#session-lifecycle-counter-semantics)를 참조하세요.                                                                                                                                                                                          |
| `claude_code_self_hosted_runner_initializing_sessions`                            | 초기화 단계의 세션이며, 할당부터 자식의 초기화 이벤트까지입니다.                                                                                                                                                                                                                                                                                             |
| `claude_code_self_hosted_runner_session_init_duration_seconds`                    | 세션 초기화 기간의 히스토그램                                                                                                                                                                                                                                                                                                                 |
| `claude_code_self_hosted_runner_session_init_errors_total`                        | 초기화에 도달하기 전에 실패한 세션: 체크아웃 훅 실패, git 준비, 토큰 문제 또는 사전 초기화 자식 충돌                                                                                                                                                                                                                                                                    |
| `claude_code_self_hosted_runner_session_start_hook_errors_total`                  | 오류 결과를 보고한 `SessionStart` 훅이며, 실패한 훅 실행당 하나입니다.                                                                                                                                                                                                                                                                                  |
| `claude_code_self_hosted_runner_session_idle_seconds{session_id,client_platform}` | 세션이 유휴 상태가 된 이후 초의 세션별 게이지입니다. 응답 없는 권한 프롬프트에 고착된 세션을 종료하는 데 유용합니다.                                                                                                                                                                                                                                                              |

오케스트레이터는 `/healthz`와 동일한 포트에서 `GET /metrics`에서 자체 시리즈를 제공합니다:

| 시리즈                                                                                     | 참고                                                                                                                                                                                          |
| :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `claude_code_self_hosted_orchestrator_info{version,pool_id,orchestrator_uuid,hostname}` | 항상 `1`                                                                                                                                                                                      |
| `claude_code_self_hosted_orchestrator_connected`                                        | 가장 최근 폴이 성공했을 때 `1`입니다. 실패 종류와 관계없이 실패한 폴 후 `0`으로 떨어집니다.                                                                                                                                    |
| `claude_code_self_hosted_orchestrator_last_poll_age_seconds`                            | 마지막 폴 시도 이후 초이며, 성공 또는 실패이며, 마지막 성공 이후를 측정하는 러너의 동일한 이름의 메트릭과 달리 `connected`와 쌍을 이루어 실패한 폴을 포착합니다. 오케스트레이터의 폴 루프는 훅 실행을 기다리므로 기본값에서 약 90초인 `--hook-timeout` 더하기 여백 위에 경고하세요. 평면 60이 아닙니다.   |
| `claude_code_self_hosted_orchestrator_poll_errors_total{error_kind}`                    | 종류별 누적 PollSpawnHints 실패: `transport`, `timeout`, `5xx`, `429` 또는 `4xx`. 모든 5개 시리즈는 프로세스 시작부터 존재합니다. `rate(...[5m]) > 0`에서 경고하세요.                                                           |
| `claude_code_self_hosted_orchestrator_queue_pending_sessions`                           | 지금 청구 가능한 스폰 요청                                                                                                                                                                             |
| `claude_code_self_hosted_orchestrator_queue_backing_off_sessions`                       | 재시도 가능한 훅 실패 후 재시도 백오프의 스폰 요청                                                                                                                                                               |
| `claude_code_self_hosted_orchestrator_queue_circuit_broken_sessions`                    | Owner가 환경의 **Activity** 탭에서 재시도할 때까지 차단된 스폰 요청입니다. 0 이상이면 경고하세요.                                                                                                                            |
| `claude_code_self_hosted_orchestrator_pool_pending_sessions`                            | 이 환경의 러너를 기다리는 총 세션입니다. 환경 전체 집계이며, 모든 오케스트레이터 인스턴스에서 동일합니다: 인스턴스 전체에서 `SUM` 대신 `MAX`를 사용하세요.                                                                                               |
| `claude_code_self_hosted_orchestrator_pool_active_sessions`                             | 현재 이 환경의 살아있는 러너에 할당된 세션입니다. 환경 전체 집계이며, 모든 오케스트레이터 인스턴스에서 동일합니다: 인스턴스 전체에서 `SUM` 대신 `MAX`를 사용하세요.                                                                                          |
| `claude_code_self_hosted_orchestrator_spawn_hooks_total{result}`                        | 누적 `spawn-runner` 훅 결과: `ok`, `retryable`, `non_retryable`. 오케스트레이터 훅 호출을 계산하며, 러너가 생성하는 세션 자식이 아닙니다: 용량이 1 이상, 웜 풀 및 동일한 세션에 대해 다시 생성된 러너가 둘을 분산시키므로 `sessions_started_total`과 비교할 수 없습니다. |
| `claude_code_self_hosted_orchestrator_spawn_hook_duration_seconds`                      | 훅 기간의 히스토그램                                                                                                                                                                                 |
| `claude_code_self_hosted_orchestrator_warm_hints_dispatched_total`                      | 프로세스 시작 이후 발송된 대기 중인 스폰 요청                                                                                                                                                                  |
| `claude_code_self_hosted_orchestrator_session_queue_wait_seconds`                       | 오케스트레이터가 스폰을 위해 청구하기 전에 각 세션이 대기열에서 기다린 초의 히스토그램이며, 제어 평면이 각 세션의 스폰 요청과 함께 전송하는 큐 대기 타임스탬프에서 기록됩니다. p50/p99 큐 시간 경고에 사용하세요. 사전 워밍 스폰은 샘플링되지 않습니다.                                           |
| `claude_code_self_hosted_orchestrator_clock_skew_seconds`                               | 로컬 빼기 서버 클록 스큐입니다. 진단이며, 측정되면 존재합니다.                                                                                                                                                        |
| `claude_code_self_hosted_orchestrator_scm_connector_connected`                          | [SCM connector](#scm-connector-flags)의 WebSocket이 열려 있을 때 `1`입니다. 다이얼링 또는 백오프 중일 때 `0`입니다. `--scm-connector-host`가 설정되지 않으면 없습니다.                                                           |
| `claude_code_self_hosted_orchestrator_scm_connector_requests_forwarded_total`           | 프로세스 시작 이후 구성된 SCM 호스트로 프록시된 누적 HTTP 요청입니다. `--scm-connector-host`가 설정되지 않으면 없습니다.                                                                                                          |

자동 스케일링의 경우 스케일링 스타일과 일치하는 시리즈를 선택하고 스케일러에 공급하기 전에 게이트하세요:

* **큐 깊이 스케일링**: `queue_pending_sessions`이 아닌 `claude_code_self_hosted_orchestrator_pool_pending_sessions`을 HPA 또는 KEDA 스케일러에 공급하세요.
* **용량 스케일링**: 러너의 `active_sessions`과 `capacity`의 비율에 따라 스케일하세요.
* **`connected`에 게이트**: 쿼리를 인스턴스당 `claude_code_self_hosted_orchestrator_connected == 1`로 필터링하여 연결이 끊긴 복제본의 오래된 값이 스케일러에 공급되지 않도록 하세요.

전체 폴 중단 중에 모든 복제본이 연결이 끊어지면 게이트된 쿼리는 데이터를 반환하지 않습니다. HPA는 누락된 메트릭에서 현재 복제본 수를 유지하지만, KEDA의 Prometheus 스케일러는 기본 `ignoreNullValues: "true"`에서 빈 결과를 0으로 읽고 축소합니다. ScaledObject에서 `ignoreNullValues: "false"`를 설정하고, 선택적으로 `fallback` 복제본 바닥으로 설정하세요.

다음 Prometheus Operator `PodMonitor`는 두 프로세스를 모두 다룹니다. `app.kubernetes.io/part-of: claude-code-self-hosted-runner` 레이블 및 [Kubernetes recipe](/docs/ko/self-hosted-environments-deploy#kubernetes)가 설정하는 명명된 `health` 포트로 포드를 선택합니다. 배포와 일치하도록 네임스페이스를 조정하세요:

```yaml theme={null}
# Claude Code 자체 호스팅 러너 + 오케스트레이터용 Prometheus Operator PodMonitor 예제입니다.
# 배포와 일치하도록 네임스페이스 및 레이블 선택기를 조정하세요. 러너와 오케스트레이터 모두
# --health-port(기본값 8080)에서 /metrics를 제공합니다.
apiVersion: monitoring.coreos.com/v1
kind: PodMonitor
metadata:
  name: claude-code-self-hosted-runner
  namespace: monitoring
spec:
  namespaceSelector:
    matchNames:
      - claude-runners
  selector:
    matchExpressions:
      # Kubernetes 레시피의 러너 배포와 동일한 방식으로 레이블이 지정되고
      # 명명된 'health' containerPort를 제공하는 모든 온디맨드 러너 작업 및 오케스트레이터 포드와 일치합니다.
      - key: app.kubernetes.io/part-of
        operator: In
        values: [claude-code-self-hosted-runner]
  podMetricsEndpoints:
    - port: health
      path: /metrics
      interval: 30s
```

이러한 샘플 경고 규칙은 시작점입니다. 플릿 크기에 맞게 임계값을 조정하세요:

```yaml theme={null}
# Claude Code 자체 호스팅 러너 + 오케스트레이터용 Prometheus 경고 규칙 예제입니다.
# 플릿 크기 및 SLO에 맞게 임계값을 조정하세요.
groups:
  - name: claude-code-self-hosted-runner
    rules:
      - alert: ClaudeRunnerPollStale
        expr: claude_code_self_hosted_runner_last_poll_age_seconds > 60
        for: 2m
        labels: {severity: warning}
        annotations:
          summary: "러너 {{ $labels.pod }}이(가) 60초 이상 폴링하지 않았습니다."
      - alert: ClaudeRunnerVersionDrift
        expr: count(count by (version) (claude_code_self_hosted_runner_info)) > 1
        for: 30m
        labels: {severity: info}
        annotations:
          summary: "러너가 혼합 버전을 실행 중입니다."
      - alert: ClaudeRunnerInitErrorsHigh
        expr: increase(claude_code_self_hosted_runner_session_init_errors_total[10m]) > 3
        for: 5m
        labels: {severity: warning}
        annotations:
          summary: "러너 {{ $labels.pod }}: 10분 내 >3개 세션 초기화 실패(체크아웃 훅 / git / 토큰 / 사전 초기화 충돌)"
      - alert: ClaudeRunnerPollErrors
        expr: sum by (pod) (rate(claude_code_self_hosted_runner_poll_errors_total[5m])) > 0
        for: 2m
        labels: {severity: warning}
        annotations:
          summary: "러너 {{ $labels.pod }}: PollWork 실패(5분 이상 {{ $value | humanize }}/s)"
      - alert: ClaudeRunnerSessionStartHookErrors
        expr: increase(claude_code_self_hosted_runner_session_start_hook_errors_total[10m]) > 3
        for: 5m
        labels: {severity: warning}
        annotations:
          summary: "러너 {{ $labels.pod }}: 10분 내 >3개 SessionStart 훅 실패"

  - name: claude-code-self-hosted-orchestrator
    rules:
      - alert: ClaudeOrchestratorDisconnected
        expr: claude_code_self_hosted_orchestrator_connected == 0
        for: 2m
        labels: {severity: critical}
        annotations:
          summary: "오케스트레이터 {{ $labels.pod }}이(가) Anthropic 제어 평면에 도달할 수 없습니다."
      - alert: ClaudeOrchestratorPollStale
        expr: claude_code_self_hosted_orchestrator_last_poll_age_seconds > 90
        for: 2m
        labels: {severity: warning}
        annotations:
          summary: "오케스트레이터 {{ $labels.pod }}이(가) 90초 이상 폴링하지 않았습니다(폴 루프는 훅 실행을 기다립니다)."
      - alert: ClaudeOrchestratorCircuitBroken
        expr: claude_code_self_hosted_orchestrator_queue_circuit_broken_sessions > 0
        for: 1m
        labels: {severity: critical}
        annotations:
          summary: "{{ $value }}개 세션이 회로 차단됨 — spawn-runner 훅이 반복적으로 재시도 불가능합니다. 인프라를 수정한 후 Activity 탭에서 재시도하세요."
      - alert: ClaudeOrchestratorPollErrors
        expr: sum by (pod) (rate(claude_code_self_hosted_orchestrator_poll_errors_total[5m])) > 0
        for: 2m
        labels: {severity: warning}
        annotations:
          summary: "오케스트레이터 {{ $labels.pod }}: PollSpawnHints 실패(5분 이상 {{ $value | humanize }}/s)"
      - alert: ClaudeOrchestratorSpawnHookFailing
        expr: sum by (pod) (increase(claude_code_self_hosted_orchestrator_spawn_hooks_total{result!="ok"}[5m])) > 3
        for: 5m
        labels: {severity: warning}
        annotations:
          summary: "오케스트레이터 {{ $labels.pod }}: 5분 내 >3개 spawn-runner 훅 실패"
```

<h3 id="pass-through-session-child-metrics">
  세션 자식 메트릭 통과
</h3>

각 세션은 자체 자식 프로세스에서 자체 OpenTelemetry 메트릭으로 실행됩니다. `--capacity`가 1 이상일 때, 러너는 이러한 자식 메트릭이 노출되는 방식을 다시 작성합니다. 러너 호스트에서 `OTEL_METRICS_EXPORTER=prometheus`를 설정하고 세션의 환경에서 `CLAUDE_CODE_ENABLE_TELEMETRY=1`을 설정하면(예: [wrapper script](/docs/ko/self-hosted-environments-configuration#wrapper-scripts) 또는 러너의 자체 환경에서, 세션이 상속함), 각 자식의 카운터 및 게이지 도구를 러너의 자체 `/metrics` 엔드포인트에 다시 노출하며, 러너의 시리즈와 함께 노출합니다. 러너는 자식의 내보내기를 루프백 전용 수신기로 OTLP를 통해 상태 포트로 푸시하도록 다시 작성하고, 각 시리즈에 `session_id` 및 `client_platform` 레이블을 태그하며, 해당 세션이 종료될 때 세션의 시리즈를 제거합니다. 히스토그램은 통과하지 않으며, 러너의 자체 접두사와 충돌하는 자식 메트릭은 삭제됩니다.

기본 `--capacity 1`에서 다시 작성이 적용되지 않습니다: 세션의 자식은 일반적으로 포트 9464에서 자체 Prometheus 엔드포인트를 바인딩합니다.

<h3 id="session-lifecycle-counter-semantics">
  세션 라이프사이클 카운터 의미론
</h3>

`sessions_started_total`, `sessions_completed_total`, `sessions_failed_total` 및 `sessions_interrupted_total` 카운터는 각 세션을 종료 방식으로 분류합니다. 생성된 모든 세션 자식은 생성 시 `sessions_started_total`을 증가시키고, 정확히 하나의 다른 세 개가 종료 시 증가하므로, `sessions_started_total` 빼기 다른 세 개의 합은 현재 실행 중인 세션 자식의 수와 같습니다.

* `completed`: 세션이 깔끔하게 종료되었습니다. 이는 자식이 코드 `0`으로 자체 종료, 자식이 여전히 연결된 동안 세션이 보관되거나 삭제됨, 그리고 러너가 슬롯을 깔끔하게 반환하는 경우(유휴 타임아웃, 은퇴 시간 또는 `--kill-session-after-min` 제한에서 세션을 해제하는 경우, 시작 타임아웃, 또는 자식이 종료되기 전에 폴 루프가 알아챈 서버 측 할당 해제)를 다룹니다. `sessions_completed_total`을 증가시킵니다.
* `failed`: 자식이 자체적으로 0이 아닌 코드로 종료되었으며, 충돌 또는 생성 후 설정 실패입니다. `sessions_failed_total`을 증가시킵니다.
* `interrupted`: 러너가 세션 성공도 러너 결함도 아닌 운영 이유로 자식을 종료했습니다(예: 드레인 또는 `--kill-session-after-min` 한계 후 [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](#environment-variable-only-settings) 유예 기간이 끝날 때 여전히 러너에 남아 있던 세션을 종료함). Kubernetes 롤링 재시작이 `SIGTERM`을 전송하는 것은 드레인의 한 예입니다. `sessions_interrupted_total`을 증가시킵니다.

v2.1.260 이전에는 러너가 `--kill-session-after-min` 한계에 도달한 모든 세션을 종료하고 `sessions_interrupted_total`에서 계산했습니다.

[`post-session` 훅](/docs/ko/self-hosted-environments-configuration#post-session)의 `CLAUDE_RUNNER_EXIT_REASON`은 이러한 깔끔한 슬롯 반환을 다르게 분류합니다. 훅은 해제, 시작 타임아웃 및 서버 할당 해제를 `interrupted`로 보고합니다. 러너가 자식을 중지했기 때문입니다. 이러한 카운터는 동일한 이벤트를 `completed`로 기록합니다. 슬롯이 깔끔하게 반환되었기 때문입니다.

훅 수신을 `sessions_completed_total`에 직접 조정하면 완료를 과소 계산합니다. 세션별 보장을 위해 훅을 사용하고 집계 비율을 위해 카운터를 사용하세요.

원샷 환경에서 `--capacity 1`과 기본 `--drain-grace-sec 0`을 사용하면, 각 러너 프로세스는 하나의 세션이 종료된 후 잠시 후 종료됩니다. `sessions_completed_total`, `sessions_failed_total` 및 `sessions_interrupted_total`은 세션 종료 시에만 증가하며, 그 종료 직전이므로, 15\~60초마다 Prometheus 스크래핑은 러너의 시리즈가 사라지기 전에 증가를 거의 포착하지 못합니다. 이 세 개의 세션 종료 카운터는 이 섹션의 나머지 부분이 참조하는 터미널 카운터입니다. `sessions_started_total`은 생성 시 증가하고 세션의 수명 동안 표시되므로 안정적으로 표시되지만, 원샷 환경에서는 누적 수보다 "현재 실행 중인 세션"에 더 가깝게 읽힙니다.

대신 해당 목표에 대해 이 표의 시리즈를 사용하세요:

| 목표  | 사용                                                                                                                                                                                                                                                                                                         |
| :-- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 처리량 | `claude_code_self_hosted_orchestrator_spawn_hooks_total{result="ok"}`, 성공한 `spawn-runner` 훅당 한 번 증가하고 `rate()` 아래에서 의미 있는 장기 오케스트레이터의 카운터입니다. 훅 호출을 계산하며, 세션이 아니므로 사전 워밍 및 동일한 세션에 대한 반복 스폰이 세션 수에서 분산됩니다.                                                                                                 |
| 사용률 | `sum(claude_code_self_hosted_runner_active_sessions)` 대 `sum(claude_code_self_hosted_runner_capacity)`, 러너 수명과 관계없이 모든 스크래핑에서 유효한 게이지                                                                                                                                                                      |
| 백로그 | 큐 깊이를 위한 `claude_code_self_hosted_orchestrator_pool_pending_sessions` 및 `claude_code_self_hosted_orchestrator_queue_circuit_broken_sessions`, 0 이상이면 경고                                                                                                                                                    |
| 실패  | `claude_code_self_hosted_runner_sessions_failed_total`, 최선의 노력: 생성 후 실제 충돌은 증가시키며, `rate()`은 `--drain-grace-sec` 이상 `0`으로 세션을 능가하는 러너에서 의미 있습니다. 원샷 환경은 다른 터미널 카운터와 동일한 스크래핑 윈도우 문제를 가지므로 표시되는 0이 아닌 값을 조사할 가치가 있는 것으로 취급하세요. 체크아웃 훅 실패, git 준비 또는 토큰 문제와 같은 생성 전 실패는 `session_init_errors_total`에만 나타납니다. |

`orchestrator_*` 행은 [on-demand orchestrator](/docs/ko/self-hosted-environments-configuration#on-demand-runners)를 실행하는 환경에만 존재합니다. 세션을 능가하는 러너가 있는 고정 플릿에서 `--drain-grace-sec` 이상 `0`을 사용하면 처리량을 위해 `sum(rate(claude_code_self_hosted_runner_sessions_started_total[5m]))`을 사용하세요. 원샷 플릿에서 해당 시리즈는 터미널 카운터와 동일한 스크래핑 윈도우 문제를 가지므로 대신 대기 중인 세션 수에 의존하세요. 환경의 **Activity** 탭, [**Cloud environments** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)에서 백로그를 확인하세요: 러너는 큐 깊이 시리즈를 내보내지 않습니다.

세션별 결과 보고의 경우 대신 [`post-session` 훅](/docs/ko/self-hosted-environments-configuration#post-session)을 사용하세요: VM 선점과 같은 갑작스러운 러너 종료를 제외하고 자식 프로세스가 생성된 모든 세션 종료에서 실행됩니다([훅의 자체 계약](/docs/ko/self-hosted-environments-configuration#post-session)에 따름).

<h2 id="what’s-next">
  다음 단계
</h2>

* [Self-hosted environments](/docs/ko/self-hosted-environments): 환경, 러너 및 세션 모델입니다. [quickstart](/docs/ko/self-hosted-environments-quickstart) 및 [Deploy to production](/docs/ko/self-hosted-environments-deploy)은 설정 및 운영을 보유합니다.
* [Customize sessions](/docs/ko/self-hosted-environments-configuration): 래퍼 스크립트, 라이프사이클 훅 및 온디맨드 러너
* [Verify session identity](/docs/ko/self-hosted-environments-identity): 세션 토큰, 해당 클레임 및 확인 방법
