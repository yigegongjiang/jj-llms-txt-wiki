> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 자체 호스팅 환경을 프로덕션에 배포

> 프로덕션에서 자체 호스팅 러너 실행: 보안 강화, 네트워크 이그레스 제어, git 자격증명, Kubernetes 및 Compose 레시피, 그리고 문제 해결.

<Note>
  자체 호스팅 환경은 Team 및 Enterprise 플랜에서 공개 베타 상태입니다. [가용성 및 제한사항](/docs/ko/self-hosted-environments#availability-and-limitations)에서 활성화 경로를 다룹니다. 이 페이지는 프로덕션에서 플릿을 실행하는 것을 다룹니다. 첫 번째 러너 및 세션은 [빠른 시작](/docs/ko/self-hosted-environments-quickstart)을 참조하세요.
</Note>

[자체 호스팅 환경](/docs/ko/self-hosted-environments)은 네트워크 내에 배포한 러너에서 Claude Code [클라우드 세션](/docs/ko/claude-code-on-the-web)을 실행하며, 프로덕션에서 이러한 세션은 환경에 세션을 디스패치할 수 있는 모든 사용자를 대신하여 모델 지향 코드를 실행합니다. 이 페이지는 작동하는 환경을 프로덕션으로 가져가는 운영자를 위한 것입니다. 배포 순서대로 진행합니다: 실제 시스템을 연결하기 전에 잠금해야 할 사항, 플릿이 필요로 하는 이그레스, 세션이 git 호스트에 인증하는 방법, 배포 레시피 자체, 그리고 세션이 오작동할 때 확인할 사항입니다.

<h2 id="harden-your-deployment">
  배포 강화
</h2>

자체 호스팅 러너는 환경에 세션을 전달할 수 있는 모든 사람을 대신하여 인프라에서 임의의 모델 지향 코드를 실행합니다. 이는 Anthropic 조직의 모든 멤버이며, Owner가 환경으로 라우팅한 범위에서 [Claude Tag](https://claude.com/docs/claude-tag/overview) 채널 세션을 시작할 수 있는 모든 사람입니다. 환경을 프로덕션 시스템에 연결하기 전에 각 항목을 검토하세요:

* **임시 세션별 컨테이너**: `--capacity 1`과 기본값 `--drain-grace-sec 0`을 사용하여 각 러너 프로세스를 프로세스가 종료될 때 삭제되는 새로운 컨테이너 또는 VM에서 실행하므로 각 컨테이너는 정확히 하나의 세션을 제공합니다. 더 높은 용량이거나 양수 드레인 유예가 있으면 하나의 컨테이너가 동일한 [잠긴 소유자](/docs/ko/self-hosted-environments#key-concepts)의 여러 세션을 제공합니다. [러너 수명 주기](/docs/ko/self-hosted-environments#runner-lifecycle)를 참조하세요. 러너 재시작 간에 파일 시스템을 재사용하지 마세요. 의도적인 [사전 준비된 체크아웃](#reuse-a-pre-warmed-checkout) 설정을 제외하고, 소유자 간에는 절대 재사용하지 마세요.
* **이미지에 광범위한 자격증명 없음**: 장기 SSH 키, 클라우드 공급자 자격증명, 또는 세션이 필요한 것보다 더 많은 권한을 부여하는 개인 액세스 토큰을 포함하지 마세요. 세션 중에 사용되는 자격증명(예: 푸시 또는 API 토큰)을 [래퍼 스크립트](/docs/ko/self-hosted-environments-configuration#wrapper-scripts)에서 세션별로 발급하세요. 래퍼가 실행되기 전에 발생하는 초기 클론의 경우 [`checkout` 수명 주기 훅](/docs/ko/self-hosted-environments-configuration#checkout) 또는 [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy)를 사용하세요. [git 구성](#configure-git)을 참조하세요.
* **세션 실행 호스트에서 환경 시크릿 유지**: 환경 시크릿은 러너를 등록하고 환경에 대기 중인 모든 세션을 선택할 수 있습니다. 고정 플릿에서는 모든 러너 호스트에 있으며, 모든 세션의 코드가 시크릿 파일을 읽을 수 있습니다. [온디맨드 러너](/docs/ko/self-hosted-environments-configuration#on-demand-runners)를 선호하세요. 여기서 시크릿은 사용자 코드를 실행하지 않는 오케스트레이터 호스트에 유지되며, 각 러너는 정확히 하나의 러너를 등록하는 단일 사용 작업 주문을 받습니다. 고정 플릿에서는 환경 시크릿 파일을 모든 세션이 읽을 수 있는 것으로 취급하고 의심되는 세션 손상 후 시크릿을 회전하세요.
* **기본 거부 네트워크 이그레스**: 모든 환경에서 네트워크 경계에서 러너 및 세션 컨테이너 아웃바운드 트래픽을 제한하세요. [기본 거부 이그레스](#default-deny-egress)에서 허용할 항목과 이유를 다룹니다.
* **최소 권한 호스트 IAM**: 러너 호스트에 연결된 컴퓨팅 ID(예: 인스턴스 프로필 또는 노드 서비스 계정)는 러너 자체가 필요한 것만 부여해야 합니다. 세션은 호스트의 ID를 상속하는 대신 래퍼 스크립트를 통해 자신의 자격증명을 얻어야 합니다.
* **세션에서 클라우드 메타데이터 엔드포인트 차단**: 세션을 호스트 ID에서 벗어나게 유지하려면 클라우드 메타데이터 엔드포인트에 대한 액세스를 차단해야 하며, 서브넷 수준 이그레스 정책은 링크 로컬 메타데이터 트래픽을 가로채지 않으므로 컨테이너 자체에서 차단하세요:

  * 홉 제한이 1인 IMDSv2
  * 메타데이터 은폐가 있는 GKE Workload Identity
  * 세션 컨테이너의 네트워크 네임스페이스에서 `169.254.169.254`에 대한 명시적 거부

  블록은 래퍼 스크립트 및 수명 주기 훅에도 적용됩니다. 컨테이너를 공유하기 때문입니다. 모든 토큰 교환을 [세션 JWT](/docs/ko/self-hosted-environments-identity)로 자신의 토큰 서비스에 대해 인증하세요. 허용 목록 이그레스를 통해 또는 Amazon EKS의 IAM Roles for Service Accounts(IRSA)와 같은 파일 기반 웹 ID를 사용하세요.
* **러너별 파일 시스템 격리**: 각 러너 프로세스는 호스트의 다른 프로세스가 읽거나 쓸 수 없는 자신의 작업 디렉토리를 가집니다. `--hooks-dir`, 래퍼 스크립트, 호스트의 `~/.claude/`를 이미지에 내장되거나 읽기 전용으로 마운트된 세션에 읽기 전용으로 만드세요.
* **전달에는 환경별 액세스 제어가 없음**: Anthropic 조직의 모든 멤버는 모든 환경에 세션을 전달할 수 있습니다. Owner가 [Claude Tag 채널을 환경으로 라우팅](/docs/ko/cloud-environments#set-the-environment-a-claude-tag-channel-uses)하면, [Claude Tag 액세스 설정](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude)이 허용하는 모든 사람이 거기서 실행되는 채널 세션을 시작할 수 있습니다. 기본적으로 Claude 계정이 있는지 여부와 관계없이 연결된 Slack 워크스페이스의 모든 사람입니다. 모든 러너 호스트를 코드 실행에 도달할 수 있는 것으로 취급하세요. 환경에 전달할 수 있는 모든 사람이 읽을 수 있는 데이터와 자격증명만 러너 호스트에 배치하세요. [`--lock-to-account`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)는 주어진 호스트가 실행하는 계정의 세션을 제한하지만, 환경에 전달할 수 있는 사람을 좁히지는 않습니다. 자체 호스팅 환경을 유일한 선택 옵션으로 만들려면 [Owner](/docs/ko/cloud-environments#organization-shared-environments)가 전체 조직에 대해 [**클라우드 환경** 페이지](https://claude.ai/admin-settings/cloud-environments)에서 Anthropic 호스팅 환경을 숨길 수 있습니다.
* **리포지토리 설정 가드 적용**: [`--confine-repo-settings`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)로 가드 모드를 선택하세요. 기본값 `warn`은 위반을 기록하고 여전히 세션을 생성하고, `enforce`는 세션을 거부하며, `off`는 스캔을 비활성화합니다. 러너는 각 리포지토리의 커밋된 설정을 스캔합니다:

  * 해당 세션의 자신의 워크스페이스 외부에서 해결되는 권한: `additionalDirectories` 항목, `permissions.allow`의 `Edit`, `Write`, 또는 `NotebookEdit` 규칙, 또는 `sandbox.filesystem.allowWrite` 또는 `allowRead` 항목
  * 비어있지 않은 `env` 블록
  * `sandbox.enabled: false`와 같은 운영자 태세 재정의

  가드는 [`--trust-workspace`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)와 관계없이 실행되며, 리포지토리 훅, `.mcp.json`, 또는 Bash 규칙을 다루지 않습니다. [권한 및 도구 승인](/docs/ko/self-hosted-environments-configuration#permissions-and-tool-approval)에서 이러한 권한이 어디에 속하는지 참조하세요.

<Note>
  조직의 IP 허용 목록은 기본적으로 자체 호스팅 러너 트래픽을 다루지 않습니다. 러너 또는 세션 트래픽에 대한 네트워크 제어로 의존하지 마세요. 대신 자신의 네트워크 경계에서 기본 거부 이그레스를 적용하고, 조직에 대한 IP 허용 목록 적용을 원하면 Anthropic 계정 팀에 문의하세요.
</Note>

<h2 id="network-requirements">
  네트워크 요구사항
</h2>

러너 및 생성하는 세션 자식은 아래 호스트에 아웃바운드 연결을 만듭니다. 세션 컨테이너 이그레스를 이러한 호스트 및 세션이 도달해야 하는 특정 내부 서비스로 제한하세요. [기본 거부 이그레스](#default-deny-egress)에서 방법과 이유를 다룹니다.

이러한 호스트는 항상 필요합니다:

| 호스트                                               | 포트                       | 사용 목적                                                                                                                                                                                                                                                                  |
| :------------------------------------------------ | :----------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                               | 443, HTTPS; SCM 커넥터만 WSS | 러너 제어 평면 및 세션 스트리밍, 모델 추론, 기능 플래그, 제품 분석, [JWKS](/docs/ko/self-hosted-environments-identity) 키 가져오기, 커밋 서명, `--use-anthropic-git-proxy`가 설정되었을 때 git 프록시, `--scm-connector-host`가 설정되었을 때 오케스트레이터의 [SCM 커넥터](/docs/ko/self-hosted-environments-reference#scm-connector-flags) 터널 |
| `github.com` 또는 GitHub Enterprise 호스트와 같은 git 호스트 | 443 또는 22                | 리포지토리 클론 및 푸시. 러너가 `--use-anthropic-git-proxy`를 사용하는 경우 필요하지 않습니다. 이는 git 트래픽을 `api.anthropic.com`을 통해 라우팅합니다.                                                                                                                                                         |

이러한 호스트가 필요한지 여부는 구성에 따라 다릅니다:

| 호스트                                  | 포트  | 필요한 경우                                                                                                                                                                        |
| :----------------------------------- | :-- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `downloads.claude.ai`                | 443 | 설치 시간에 네이티브 설치 프로그램으로 호스트에 Claude Code를 설치하거나 업데이트할 때; `install.sh` 스크립트 자체는 `claude.ai`에서 제공됩니다. 세션 런타임에는 세션이 공식 Anthropic 마켓플레이스에서 플러그인을 설치할 때만 필요합니다.                      |
| `storage.googleapis.com`             | 443 | 세션 런타임에 `/plugin`에 표시되는 플러그인 설치 수 및 메타데이터의 경우.                                                                                                                                |
| `code.claude.com` 및 `claude.com`     | 443 | 내장 claude-code-guide 에이전트의 문서 조회 및 세션 중 사전 승인된 WebFetch 요청. 이러한 호스트를 차단하면 문서 조회에만 영향을 미칩니다.                                                                                   |
| `*.frame.claudeusercontent.com`      | 443 | 조직의 세션에 [Artifact 도구](/docs/ko/artifacts#availability)를 사용할 수 있을 때만; 기본값은 계획에 따라 다르며, 가용성 테이블을 참조하세요. 러너에서 `CLAUDE_CODE_DISABLE_ARTIFACT=1`을 설정하여 조직 설정과 관계없이 도구를 비활성화된 상태로 유지하세요. |
| `registry.npmjs.org`                 | 443 | 세션이 플러그인을 설치할 때, npm 소스 플러그인 패키지를 가져오기 위해 및 플러그인의 Node.js 종속성을 설치할 때, 또는 `npx` 시작 MCP 서버가 실행될 때                                                                               |
| `http-intake.logs.us5.datadoghq.com` | 443 | Anthropic 운영 메트릭. `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1`이 설정되었을 때만; 자체 호스팅 환경에서는 기본적으로 꺼져 있습니다.                                                                                  |
| `browser-intake-us5-datadoghq.com`   | 443 | Anthropic 오류 보고서 업로드, [오류 보고](/docs/ko/data-usage#telemetry-services)가 세션의 계정에 대해 활성화되었을 때만 전송됩니다. `DISABLE_ERROR_REPORTING=1` 또는 `DISABLE_TELEMETRY=1`로 억제됩니다.                    |

러너는 `statsig.anthropic.com`, `*.sentry.io`, `claude.ai`, 또는 `platform.claude.com`에 도달하지 않습니다. 이러한 호스트는 일부 이전 엔터프라이즈 네트워크 체크리스트에 나타나지만, 러너 또는 세션 트래픽에 대해 허용 목록에 추가할 필요가 없습니다: 기능 플래그 가져오기는 `api.anthropic.com`으로 이동하고, 러너는 대화형 OAuth가 아닌 환경 시크릿으로 인증합니다. 두 호스트 측 흐름은 `claude.ai`에 도달하므로 세션 컨테이너 이그레스를 넓히는 대신 이그레스를 허용하는 호스트에서 실행하세요: 한 줄 설치 프로그램은 설치 시간에 `claude.ai`에서 `install.sh`를 가져오고, 대화형 `claude auth login`은 [안내 설정](/docs/ko/self-hosted-environments-quickstart#set-up-an-environment-and-runner), `doctor`의 서명된 모드, 및 [CI 전달](/docs/ko/self-hosted-environments-testing#authenticate-from-ci)이 사용하며, `claude.ai`, `claude.com`, 및 `platform.claude.com`을 통해 서명합니다. `mcp-proxy.anthropic.com`도 필요하지 않습니다: 자체 호스팅 세션은 이를 사용하지 않으며, 조직의 claude.ai 커넥터를 세션에 전달하는 것(조직에 대해 활성화된 경우)은 `api.anthropic.com`을 통해 라우팅됩니다. [MCP 서버](/docs/ko/self-hosted-environments-configuration#mcp-servers)를 참조하세요.

<h3 id="default-deny-egress">
  기본 거부 이그레스
</h3>

러너 및 세션 컨테이너를 [네트워크 요구사항 테이블](#network-requirements)의 호스트, git 호스트, 및 세션이 도달해야 하는 특정 내부 서비스로 제한되는 네트워크 세그먼트 또는 네임스페이스에 배포하세요. 제품은 이를 확인하거나 적용할 수 없으므로 모든 환경에서 자신의 네트워크 경계에 적용하세요. 세션 코드는 모델 지향이며 임의의 호스트에 대한 연결을 시도할 수 있습니다. 네트워크 계층에서 기본 거부 이그레스는 이러한 시도가 도달할 수 있는 위치를 제한합니다. 이는 권한 모드와 관계없이 적용됩니다: 기본 사전 승인 도구 세트에는 이미 `Bash`가 포함되어 있으므로 [자동 모드](/docs/ko/self-hosted-environments-configuration#permissions-and-tool-approval) 없이도 셸 이그레스가 프롬프트 없이 실행됩니다.

각 세션이 내보내는 원격 분석 및 이를 끄는 방법에 대한 자세한 내용은 [원격 분석](/docs/ko/self-hosted-environments-reference#telemetry)을 참조하세요.

<h3 id="authenticate-to-an-egress-proxy">
  이그레스 프록시에 인증
</h3>

일부 기업 이그레스 프록시는 모든 연결에 `Proxy-Authorization` 헤더를 요구합니다. 해당 헤더의 토큰은 종종 `HTTPS_PROXY`에 설정한 프록시 URL에 쓸 수 있을 정도로 빠르게 회전합니다. `HTTPS_PROXY` 또는 `HTTP_PROXY`를 평소대로 프록시의 URL로 설정한 다음 `--proxy-authorization-command` 또는 `--proxy-authorization-file`을 설정하여 러너에게 헤더 값을 읽을 위치를 알려주세요. 두 플래그 모두 Claude Code v2.1.238 이상이 필요합니다.

<h4 id="choose-where-the-proxy-authorization-value-comes-from">
  `Proxy-Authorization` 값이 어디에서 오는지 선택
</h4>

`Proxy-Authorization` 토큰을 생성하는 방법과 일치하는 플래그를 선택하세요:

* **[`--proxy-authorization-command <command>`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)**: 온디맨드로 생성하는 토큰의 경우 이를 선택하세요. 러너는 셸 명령을 실행하고 트리밍된 stdout을 헤더 값으로 사용합니다(예: `Bearer <token>`).
* **[`--proxy-authorization-file <path>`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)**: 다른 프로세스가 제자리에서 회전하는 토큰의 경우 이를 선택하세요. 러너는 파일을 읽고 트리밍된 내용을 헤더 값으로 사용합니다.

<h4 id="configurations-the-runner-refuses-to-start-with">
  러너가 시작을 거부하는 구성
</h4>

각 플래그에는 [러너 CLI 플래그 참조](/docs/ko/self-hosted-environments-reference#runner-cli-flags)에 나열된 환경 변수 형식도 있습니다. 러너가 프록시 또는 제어 평면에 연결하기 전에 플래그 및 변수를 확인하고 세 가지 경우에 시작을 거부합니다:

* **두 플래그 모두 설정**: 한 플래그와 다른 플래그의 환경 변수를 설정하는 것은 둘 다 설정하는 것으로 계산됩니다.
* **프록시 URL 없음**: `HTTPS_PROXY` 또는 `HTTP_PROXY` 중 어느 것도 `http://` 또는 `https://` URL을 보유하지 않습니다. 러너는 대문자 또는 소문자로 두 변수를 읽으며 `ALL_PROXY`를 참조하지 않습니다.
* **오케스트레이터 서브명령에 전달된 플래그**: `self-hosted-runner orchestrator`는 플래그 또는 환경 변수를 허용하지 않습니다. 대신 오케스트레이터가 시작하는 각 러너에 플래그를 전달하세요.

<h4 id="what-the-runner-changes-while-a-proxy-authorization-flag-is-set">
  프록시 인증 플래그가 설정되어 있는 동안 러너가 변경하는 것
</h4>

플래그 중 하나가 설정되면 러너는 자신의 리스너를 시작하고 자신, 수명 주기 훅, 세션의 프록시 트래픽을 해당 리스너를 통해 보냅니다. 리스너는 프록시로 가는 길에 `Proxy-Authorization` 헤더를 추가합니다.

* **리스너**: 리스너는 `127.0.0.1`의 포워드 프록시입니다. 러너는 제어 평면에 등록하기 전에 리스너를 시작하고 리스너를 시작할 수 없으면 시작 시 종료합니다.
* **프록시 변수**: 러너는 설정한 `HTTPS_PROXY` 및 `HTTP_PROXY` 중 어느 것이든 리스너를 가리키도록 다시 작성합니다. 그 다시 작성된 값은 러너 자체, 수명 주기 훅, 실행하는 모든 세션에 도달합니다.
* **토큰 회전**: 회전된 토큰은 재시작 없이 적용됩니다. 리스너가 프록시에 열 때마다 러너는 명령을 실행하거나 파일을 다시 읽고 결과를 헤더로 추가합니다.
* **세션 환경**: 세션은 리스너를 통해서만 프록시에 도달합니다. 각 세션의 환경에서 러너는 `ALL_PROXY`를 제거하고, 설정하지 않은 `HTTPS_PROXY` 또는 `HTTP_PROXY`의 모든 철자를 제거하고, `NO_PROXY`를 러너의 자신의 값으로 고정합니다.
* **로그**: 러너는 헤더 값을 기록하지 않습니다.

<h2 id="configure-git">
  git 구성
</h2>

러너는 리포지토리 체크아웃을 관리하지만 기본적으로 git ID 또는 자격증명을 구성하지 않습니다. 러너의 이미지와 프로세스 환경을 제어하므로 git 구성을 제어합니다. 두 가지 접근 방식 중 하나를 선택하세요:

* **러너가 git을 구성하도록 허용**: `--configure-git`으로 러너를 시작하여 Anthropic 호스팅 세션이 사용하는 동일한 ID 및 커밋 서명 구성을 작성하도록 합니다.
* **이미지에 git 구성 제공**: ID 및 푸시 자격증명을 직접 설정하세요(예: 자신의 봇 ID로 커밋하기 위해).

러너 호스트의 Git 버전 하한: [`--configure-git`](#let-the-runner-configure-git) SSH 커밋 서명에는 Git 2.34 이상이 필요하고, [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy)에는 2.32 이상이 필요하며, [`--push-outcome-on-release`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)로 푸시된 분기에서 세션을 재개하려면 2.29 이상이 필요합니다. 세 가지를 모두 생략하고 git ID를 직접 관리하면 Git 2.24로 충분합니다.

<h3 id="let-the-runner-configure-git">
  러너가 git을 구성하도록 허용
</h3>

`--configure-git`으로 러너를 시작하거나 `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`을 설정하여 시작 시 전역 git 구성을 작성하도록 합니다:

* `user.name = Claude` 및 `user.email = noreply@anthropic.com`, Anthropic 호스팅 세션과 일치
* SSH 형식 커밋 및 태그 서명, 러너 관리 shim을 통해 라우팅되어 세션의 자신의 자격증명을 사용하여 Anthropic의 서명 서비스를 통해 각 커밋에 서명합니다. 서명은 Anthropic의 게시된 SSH 서명 키에 대해 GitHub에서 확인할 수 있습니다.
* `push.negotiate = true`, git이 푸시를 패킹하기 전에 git 호스트에 이미 가지고 있는 커밋을 요청하도록 합니다. Claude Code v2.1.257 이상이 필요합니다.
* `core.hooksPath`는 러너 관리 훅 디렉토리를 가리킵니다. 해당 `commit-msg` 및 `prepare-commit-msg` 훅은 각 커밋에 세션 작성자를 위한 `Co-authored-by:` 트레일러를 추가하며, [`CCR_SESSION_ACCOUNT_EMAIL`](/docs/ko/self-hosted-environments-configuration#wrapper-scripts)의 이메일에서 빌드되고 해당 변수가 설정되지 않으면 생략됩니다. 이미지가 이미 `core.hooksPath`를 설정한 경우 러너는 설정을 그대로 두고 이러한 훅 설치를 건너뛰며 `[runner:git]` 경고를 출력합니다.

커밋 서명에는 Git 2.34 이상이 필요합니다. 러너는 시작 시 확인하고 git이 더 오래되면 오류로 종료합니다. 이 플래그는 푸시 자격증명을 구성하지 않으므로 여전히 이미지에 제공합니다.

<h3 id="ship-git-config-in-your-image">
  이미지에 git 구성 제공
</h3>

git ID는 모든 커밋에 필요합니다. Dockerfile에서 시스템 전체로 설정하여 러너 프로세스가 실행되는 사용자와 관계없이 구성이 적용되도록 합니다:

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

ID가 없으면 `git commit`이 `Please tell me who you are`로 실패하고 세션이 진행할 수 없습니다. 대신 자신의 봇 ID를 사용할 수 있습니다. 러너는 이러한 값을 재정의하지 않습니다.

공유 러너 이미지에 장기 또는 광범위 푸시 자격증명을 굽지 마세요: 이미지의 자격증명은 이미지가 실행하는 모든 세션에서 사용 가능하며, 이를 시작한 사람과 관계없습니다. 대신 [래퍼 스크립트](/docs/ko/self-hosted-environments-configuration#wrapper-scripts)에서 세션 JWT에서 디코딩된 세션 작성자의 ID를 사용하여 세션별로 단기, 최소 범위 토큰을 발급하세요. 이를 임시 세션별 컨테이너와 쌍으로 만드세요. `--capacity 1`이 필요하므로 어떤 자격증명도 이를 발급한 세션보다 오래 살지 않습니다. [강화 섹션](#harden-your-deployment)을 참조하세요.

이미지 수준에서 푸시 자격증명을 구성해야 하는 경우(예: 읽기 전용 배포 키의 경우) git 호스트가 허용하는 만큼 좁게 범위를 지정하세요:

* 한 리포지토리로 제한되고 `url.<base>.insteadOf` 다시 쓰기가 있는 SSH 배포 키
* 최소 범위 토큰을 반환하는 `credential.helper`
* 좁게 범위가 지정된 키를 가리키는 `GIT_SSH_COMMAND`

구성하는 메커니즘은 프롬프트 없이 작동해야 합니다. 러너의 내장 클론 및 페치는 git, SSH, Git Credential Manager가 표시할 프롬프트를 비활성화하기 때문입니다:

* 러너는 `GIT_TERMINAL_PROMPT=0`을 설정하므로 git은 사용자 이름이나 암호를 요청하지 않습니다.
* 러너는 `BatchMode=yes`로 SSH를 실행하며, 설정한 경우 `GIT_SSH_COMMAND`에 추가되므로 SSH는 암호 또는 호스트 확인을 요청하지 않습니다.
* 러너는 `GCM_INTERACTIVE=never`을 설정하므로 Git Credential Manager는 서명 대화를 열지 않습니다.
* 러너는 `core.askPass`를 지우므로 askpass 헬퍼를 사용하는 경우 `GIT_ASKPASS` 환경 변수를 통해 설정하세요.

git 호스트가 자격증명을 거부하거나 구성하지 않은 경우 러너는 몇 번 재시도한 다음 리포지토리 준비에 실패합니다. 세션이 결과를 푸시하는 리포지토리인 경우입니다. 세션이 읽기만 하는 리포지토리의 경우 [문제 해결](#troubleshooting)에서 러너가 대신 건너뛸 때를 다룹니다. 러너는 이러한 설정을 세션의 환경에 전달하지 않습니다.

체크아웃 디렉토리가 러너 프로세스와 다른 uid로 소유된 경우 git은 이에 대해 작동하기를 거부합니다. `safe.directory`를 추가하세요:

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

<h3 id="use-the-anthropic-git-proxy">
  Anthropic git 프록시 사용
</h3>

`--use-anthropic-git-proxy`로 러너를 시작하거나 `CLAUDE_RUNNER_USE_GIT_PROXY=1`을 설정하여 Anthropic의 git 프록시를 통해 클론하도록 합니다. 세션의 자신의 단기 토큰으로 인증됩니다. 일반 사용자 세션의 경우 프록시는 세션 작성자를 위해 저장된 GitHub 또는 GitHub Enterprise OAuth 토큰을 사용합니다. 봇 및 에이전트 세션의 경우 조직의 GitHub App 설치 토큰을 사용합니다. 어느 쪽이든 러너 이미지는 git 자격증명이 필요하지 않습니다: SSH 키, 자격증명 헬퍼, `.netrc` 없음. 이는 Anthropic 호스팅 환경이 사용하는 동일한 인증 경로입니다.

프록시는 `--capacity 1`이 필요합니다. 프록시 URL은 세션별이고 Git 2.32 이상이 필요합니다. 더 오래된 git은 프록시가 세션을 서로 격리하는 데 사용하는 구성 메커니즘을 무시합니다. 러너는 요구사항이 충족되지 않으면 시작을 거부합니다. 프록시가 Anthropic 측에서 가져오기 때문에 git 호스트는 Anthropic 인프라에서 도달할 수 있어야 합니다. 이는 Anthropic 호스팅 세션이 가진 동일한 요구사항입니다. 네트워크 내부에서만 라우팅 가능한 git 호스트의 경우 대신 [`checkout` 수명 주기 훅](/docs/ko/self-hosted-environments-configuration#checkout)을 사용하세요. 각 러너 프로세스는 한 번에 하나의 세션을 처리하므로 병렬 처리를 위해 더 많은 복제본을 실행하세요. 프록시가 활성화되면 `--git-host-rewrite` 및 `--git-ssh-rewrite`는 효과가 없습니다: 프록시 URL은 git 호스트가 아닌 `api.anthropic.com`을 가리킵니다.

러너는 또한 등록할 때 Anthropic에 옵트인을 보고하며, 시작 시 `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)`를 출력합니다. 옵트인 보고에는 Claude Code v2.1.267 이상이 필요하며, 이전 버전은 플래그를 수락하지만 이를 보고하거나 해당 라인을 출력하지 않습니다. 옵트인된 러너의 각 세션은 Anthropic 관리 git 또는 세션별 프록시 URL을 사용합니다. 세션이 세션별 프록시 URL을 사용할 때 러너는 그렇게 하는 것을 나타내는 `[runner:warn]` 라인 하나를 기록합니다.

<h3 id="rewrite-git-urls-for-private-networks">
  개인 네트워크에 대한 git URL 다시 쓰기
</h3>

리포지토리 URL은 제어 평면에서 HTTPS로 도착하며 git 호스트의 호스트 이름이 있습니다. GitHub Enterprise의 경우 claude.ai의 Claude Code 관리 설정에서 [GitHub Enterprise 통합](/docs/ko/github-enterprise-server)에 대해 구성한 호스트 이름입니다. 두 개의 반복 가능한 플래그는 클론 전에 이러한 URL을 다시 작성합니다:

* `--git-host-rewrite <from>=<to>`: 분할 수평 DNS의 경우, Anthropic이 외부 호스트 이름을 통해 git 호스트에 도달하지만 러너는 내부 호스트 이름을 사용해야 합니다.
* `--git-ssh-rewrite <host>`: SSH만 허용하는 git 호스트의 경우, `https://<host>/owner/repo`를 `git@<host>:owner/repo`로 다시 작성합니다.

호스트 다시 쓰기가 먼저 실행되므로 둘 다 필요한 경우 `--git-ssh-rewrite`에 내부 호스트 이름을 나열하세요. 체크아웃을 완전히 제어하려면 [`checkout` 수명 주기 훅](/docs/ko/self-hosted-environments-configuration#checkout)을 사용하세요.

<h2 id="build-the-runner-image">
  러너 이미지 빌드
</h2>

Anthropic은 사전 빌드된 러너 이미지를 게시하지 않습니다. `claude` 바이너리 주위에 자신의 이미지를 빌드하고, 리포지토리가 필요로 하는 모든 도구 체인을 계층화하세요: 언어 런타임, 컴파일러, 패키지 관리자, [MCP](/docs/ko/mcp) 사이드카.

아래 레시피는 `--capacity 4`를 사용하므로 하나의 컨테이너는 동일한 잠긴 소유자의 최대 4개의 동시 세션을 제공합니다. 이는 [강화 섹션](#harden-your-deployment)의 세션별 컨테이너 격리를 제공하지 않습니다: 환경을 프로덕션 시스템에 연결하기 전에 레시피를 `--capacity 1`로 실행하여 세션당 하나의 컨테이너를 사용하거나 [온디맨드 러너](/docs/ko/self-hosted-environments-configuration#on-demand-runners)를 사용하세요. 이는 또한 환경 시크릿을 세션 실행 호스트에서 벗어나게 유지합니다.

이 Dockerfile은 최소한의 시작점입니다:

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

노드가 ARM인 경우 `linux-x64`를 `linux-arm64`로 바꾸거나, Alpine과 같은 musl 기반 이미지에서 `linux-x64-musl` 또는 `linux-arm64-musl`로 바꾸세요. musl 이미지가 필요로 하는 추가 패키지는 [Alpine Linux 설정](/docs/ko/setup#alpine-linux-and-musl-based-distributions)을 참조하세요. URL은 표준 Claude Code 릴리스 위치이므로 [바이너리 무결성 및 코드 서명](/docs/ko/setup#binary-integrity-and-code-signing)에 설명된 대로 릴리스의 서명된 매니페스트에 대해 다운로드된 바이너리를 확인할 수 있습니다. Claude Code 버전 2.1.224 이상으로 이미지를 빌드한 다음 레지스트리로 푸시하고 아래 레시피에서 참조하세요:

```bash theme={null}
docker build --build-arg CLAUDE_CODE_VERSION=2.1.267 -t <your-registry>/claude-runner:latest .
```

<h2 id="size-cpu-and-memory-for-sessions">
  세션에 대한 CPU 및 메모리 크기 조정
</h2>

러너 프로세스가 아닌 실행하는 세션에 대해 러너의 컨테이너 또는 호스트 크기를 조정하세요. 러너 자체는 작업을 폴링하고, 각 세션의 체크아웃을 준비하고, [수명 주기 훅](/docs/ko/self-hosted-environments-configuration#lifecycle-hooks)을 실행하고, 세션 프로세스를 시작하고 감독합니다. 로드는 세션에서 나옵니다: 각 세션은 Claude Code 프로세스와 빌드, 테스트 스위트, 패키지 설치, [MCP 서버](/docs/ko/mcp)와 같이 시작하는 모든 것입니다.

한 세션의 경우 다음 값으로 시작하세요. Kubernetes 요청 및 제한 또는 플랫폼의 동등한 것으로 표시되며, 요구사항이 아닌 시작점으로 취급하세요:

* **메모리**: 각각 4 GiB의 요청 및 제한. Claude Code의 [시스템 요구사항](/docs/ko/setup#system-requirements)의 4 GB 최소값을 충족합니다. 두 개를 같게 유지하여 스케줄러가 컨테이너의 전체 메모리를 고려하도록 합니다. 컨테이너가 메모리 제한에 도달하면 커널이 내부 프로세스를 종료하여 세션을 중간에 종료할 수 있습니다.
* **CPU**: 2 CPUs의 요청 및 4 CPUs의 제한. 세션이 빌드 중에 요청 위로 버스트할 수 있습니다. 커널은 CPU 제한에서 프로세스를 종료하는 대신 스로틀링하므로 제한에서 세션이 더 느리게 실행되지만 계속 실행됩니다.

Kubernetes 컨테이너 사양에서 다음 `resources` 블록으로 이러한 시작 값을 설정하세요:

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

빌드 및 테스트는 일반적으로 세션 로드의 가장 크고 가장 변수적인 부분이므로 리포지토리의 대표적인 빌드를 실행하고 피크 CPU 및 메모리를 측정하고 Claude Code 프로세스를 위한 공간을 남기지 않는 시작 값을 올리세요.

러너는 `--capacity`를 사용하여 한 번에 실행하는 세션 수를 제한합니다. CPU 또는 메모리를 나누지 않으므로 러너의 세션은 컨테이너의 CPU 및 메모리를 공유합니다. 한 세션의 공유를 제한하려면 [래퍼 스크립트](/docs/ko/self-hosted-environments-configuration#wrapper-scripts)에서 제한을 적용하세요. 따라서 한 컨테이너에 제공할 항목은 한 번에 제공하는 세션 수에 따라 다릅니다:

* **러너당 한 세션**: 각 컨테이너에 한 세션의 값을 제공하세요. `--capacity 1`에서 이 크기 조정을 사용하세요. [강화 섹션](#harden-your-deployment)이 권장하고, [온디맨드 러너](/docs/ko/self-hosted-environments-configuration#on-demand-runners)의 경우, 값을 [`spawn-runner` 훅](/docs/ko/self-hosted-environments-configuration#the-spawn-runner-hook)이 제출하는 워크로드(예: Kubernetes Job의 pod 템플릿)에 설정합니다.
* **러너당 여러 세션**: `--capacity` 1 이상에서 한 세션의 값에 용량을 곱하세요. 컨테이너에서 한 번에 최대 그 많은 세션이 실행될 수 있기 때문입니다. [Kubernetes](#kubernetes) 및 [Docker Compose](#docker-compose) 레시피는 CPU 또는 메모리 제한 없이 `--capacity 4`를 실행하므로 실행하는 용량에 맞게 크기가 조정된 제한을 추가하세요.

<h2 id="kubernetes">
  Kubernetes
</h2>

러너는 기본적으로 포트 8080에서 `GET /healthz`를 제공하며, `--health-port`로 구성할 수 있으므로 Kubernetes 프로브는 추가 설정 없이 작동합니다. 엔드포인트는 프로세스가 살아있을 때마다 `200`을 반환하므로 아래 프로브는 막힌 프로세스가 아닌 죽은 프로세스를 감지합니다. 폴링을 중지한 러너를 잡으려면 [`/metrics`](/docs/ko/self-hosted-environments-reference#prometheus-metrics)의 `last_poll_age_seconds` 시리즈에 대해 경고하세요. 아래 Deployment는 Kubernetes Secret에서 환경 시크릿을 마운트하고, 활성 및 준비 프로브를 `/healthz`로 가리키며, 90초 종료 유예 기간을 설정합니다. [종료 타이밍](#shutdown-timing)에서 유예 기간이 중요한 이유를 참조하세요.

매니페스트는 러너 컨테이너에 CPU 또는 메모리 `resources`를 설정하지 않습니다. 실행하는 용량에 맞게 크기가 조정된 블록을 추가하세요. [세션에 대한 CPU 및 메모리 크기 조정](#size-cpu-and-memory-for-sessions)에서 설명합니다.

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

위의 Deployment는 `claude-runners` 네임스페이스에 있습니다. 먼저 네임스페이스를 생성하세요:

```bash theme={null}
kubectl create namespace claude-runners
```

관리 UI의 [**환경 키 복사** 단계](/docs/ko/self-hosted-environments-quickstart#set-up-an-environment-and-runner)에서 복사한 값을 보유하는 로컬 파일에서 지원 Secret을 생성하세요. 시크릿이 셸 기록에 나타나지 않도록 합니다. `(umask 077 && cat > ./environment-secret)`을 실행하고, 시크릿을 붙여넣고, Enter를 누른 다음 Ctrl-D를 누르세요. 그런 다음 Secret을 생성하고 파일을 삭제하세요:

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

<h2 id="docker-compose">
  Docker Compose
</h2>

아래 Compose 서비스는 종료될 때마다 러너를 재시작합니다. 이는 충돌과 드레인 후 정상 종료를 모두 다룹니다. Docker 재시작 정책은 쓰기 가능한 계층이 그대로 있는 동일한 컨테이너를 재시작하므로 러너는 [강화 태세](#harden-your-deployment)가 권장하는 새로운 파일 시스템이 아닌 재사용된 파일 시스템에서 돌아옵니다. 평가를 위해 이 레시피를 사용하고, 프로덕션의 경우 실행당 컨테이너를 재생성하거나 이를 수행하는 오케스트레이터를 사용하세요.

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

<h2 id="shutdown-timing">
  종료 타이밍
</h2>

`SIGTERM`에서 러너는 새 작업을 받지 않고, [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)을 설정하지 않으면 `--drain-wait-sec`(기본값 0)까지 대기하여 진행 중인 턴이 완료되도록 하고, 각 세션의 프로세스 트리를 종료하고, [`post-session` 수명 주기 훅](/docs/ko/self-hosted-environments-configuration#post-session)을 실행합니다. 해당 프로세스 트리에는 Claude가 여전히 세션에서 실행 중인 명령이 포함됩니다.

전체 드레인 경로는 `--session-stop-grace-sec` + `--drain-wait-sec` + `--post-session-hook-timeout-sec`까지 필요하며, 프로세스 정리를 위해 15초의 고정 오버헤드를 더하고, [`--push-outcome-on-release`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)가 설정되었을 때 30초를 더합니다. 기본값에서 80초이며, 러너는 시작 시 합계를 기록합니다. 세션은 이 하나의 예산 아래에서 병렬로 드레인되므로 합계는 `--capacity`에 따라 증가하지 않습니다.

기본값 `--drain-wait-sec 0`에서 롤링 재시작은 진행 중인 턴을 중단합니다. 각 세션은 다른 러너에서 재개되어 [알려진 문제](#additional-limitations)에 설명된 대로 푸시되지 않은 작업을 잃습니다. `--drain-wait-sec`을 설정하고 유예 기간을 일치하도록 올려서 턴이 먼저 완료되도록 합니다.

전체 경로 전체에서 러너는 0 용량으로 제어 평면에 계속 하트비트를 보내므로 세션 임대가 만료되지 않고 `post-session` 훅이 여전히 커밋되지 않은 작업을 작성하는 동안 다른 러너로 재큐되지 않습니다. 하트비트는 러너가 등록 해제되기 직전에 중지됩니다.

호스트가 이를 중지하기 전에 러너에 시작 시 기록하는 합계 이상을 제공하세요. 설정하는 위치는 호스트가 중지되는 방식에 따라 다릅니다:

* **`SIGTERM` 유예 기간 포함**: Kubernetes에서 `terminationGracePeriodSeconds`, Docker Compose에서 `stop_grace_period`, 또는 오케스트레이터의 동등한 것을 최소한 그 합계로 설정하세요. Kubernetes 기본값 30초는 러너의 드레인 경로보다 짧으므로 Kubernetes는 러너가 드레인을 완료하기 전에 pod를 중지합니다.
* **[`--retire-at`](/docs/ko/self-hosted-environments-reference#runner-cli-flags) 포함**: 은퇴 시간과 호스트의 중지 시간 사이의 여유를 일반적인 턴, 배경 작업 보유([러너 수명 주기](/docs/ko/self-hosted-environments#runner-lifecycle)에서 설명), 그리고 동일한 합계를 포함하도록 크기를 조정하세요. 각 시작 시 은퇴 시간을 계산하세요(예: `date +%s` + 러너의 의도된 수명).
* **[`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal) 포함**: 드레인 경로 합계에 두 부분을 더 추가하세요. 첫 번째는 구성하는 분입니다. 두 번째는 [첫 신호 이후 드레인 연기](#defer-the-drain-past-the-first-signal)에서 설명하는 사후 릴리스 유예입니다. 기본값에서 75초입니다. 플래그가 설정되면 러너는 드레인 경로 합계 후 시작 시 결합된 수치도 인쇄합니다.

<h3 id="defer-the-drain-past-the-first-signal">
  첫 신호 이후 드레인 연기
</h3>

재시작하는 러너가 드레인하는 대신 보유한 세션을 최대 `n`분 동안 계속 제공하도록 하려면 [`--defer-shutdown-max-min <n>`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)을 설정하세요. 첫 `SIGTERM` 또는 `SIGINT`에서 러너는 새 작업을 받지 않고 보유한 세션을 계속 제공합니다. 폴링을 계속하여 제어 평면이 이러한 세션을 재큐하지 않도록 합니다. Claude Code v2.1.238 이상이 필요합니다.

<h4 id="what-happens-to-the-sessions-the-runner-holds-after-the-first-signal">
  첫 신호 후 러너가 보유한 세션에 어떤 일이 발생하는지
</h4>

첫 신호를 따르는 처음 두 단계에서 러너는 세션을 릴리스하고, 릴리스된 세션은 사용자가 다음 메시지를 보낼 때 새로운 러너에서 재개됩니다. 첫 신호부터 세어서 러너는 세 단계를 거칩니다:

* **처음 `n`분 동안**: 러너는 세션을 정상적으로 제공하고 `--startup-timeout-min` 및 `--kill-session-after-min`을 계속 적용합니다. [`--release-idle-session-min`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)도 설정하면 러너는 사용자가 그 오래 유휴 상태인 모든 세션을 릴리스합니다. 없으면 유휴 세션은 러너에 남아 있습니다.
* **`n`분이 끝나면**: 러너는 여전히 보유한 모든 세션을 릴리스합니다. 유휴 상태이거나 아닙니다. 러너는 중간 턴 세션의 턴이 끝날 때까지 대기하고 턴의 배경 작업을 위해 최대 60초를 더 대기한 후 해당 세션을 릴리스합니다.
* **사후 릴리스 유예가 끝나면**: 러너는 여전히 보유한 모든 세션을 드레인하고 제어 평면은 각 드레인된 세션을 즉시 다른 러너로 재큐합니다. 사후 릴리스 유예는 `n`분이 끝날 때 시작되며 기본값에서 75초입니다. `--drain-wait-sec`을 60초 이상으로 설정하면 사후 릴리스 유예는 `--drain-wait-sec` + 15초입니다.

어느 단계에서든 러너는 세션을 보유하지 않으면 즉시 0으로 종료됩니다. 두 번째 신호는 단계를 단축합니다: 러너는 `--defer-shutdown-max-min` 없이 첫 신호에서처럼 즉시 드레인합니다. 드레인이 진행 중이면 다음 신호는 러너를 강제 종료합니다. 이는 두 번째 신호 또는 사후 릴리스 유예가 드레인을 시작했는지 여부와 관계없이 유지됩니다.

<h4 id="size-the-stop-timeout">
  중지 타임아웃 크기 조정
</h4>

호스트의 중지 타임아웃에 세 부분의 합계를 최소한 제공하세요: 구성하는 `n`분, 사후 릴리스 유예, [종료 타이밍](#shutdown-timing)에서 설명하는 전체 드레인 경로. 기본 설정에서 사후 릴리스 유예는 75초이고 드레인 경로는 80초이므로 `n`분 + 155초를 허용하세요. 러너는 `--defer-shutdown-max-min`이 설정되었을 때마다 시작 시 이 합계를 인쇄합니다.

중지 타임아웃이 러너가 완료되기 전에 끝나면 호스트는 러너를 종료합니다. 여전히 보유한 세션은 `post-session` 훅을 받지 않습니다. 러너는 등록 해제되지 않으며 제어 평면은 약 1분 후에 세션을 재큐합니다. 중지 타임아웃을 그 합계로 제공할 수 없으면 `--defer-shutdown-max-min`을 설정하지 않은 상태로 두어 러너가 첫 신호에서 드레인하도록 합니다.

<h3 id="what-reaches-a-running-post-session-hook">
  실행 중인 post-session 훅에 도달하는 것
</h3>

`post-session` 훅과 Claude 세션 자식은 각각 자신의 POSIX 프로세스 그룹에서 실행되며, 러너와 분리되어 있으므로 중지 메커니즘이 다르게 도달합니다:

* **러너가 이미 드레인 중일 때 `SIGTERM`**: 러너를 즉시 강제 종료하여 드레인 경로의 나머지를 건너뜁니다. [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal) 없이 이는 러너가 받는 두 번째 `SIGTERM`입니다. 아무것도 실행 중인 `post-session` 훅에 신호를 보내지 않으므로 고아를 채택하는 init 프로세스가 있는 베어 호스트에서 자신의 감독 없이 완료되지만: 타임아웃 예산이 더 이상 적용되지 않으며, 닫힌 로그 파이프에 쓰면 `SIGPIPE`로 종료될 수 있으므로 강제 종료에서 생존해야 하는 훅은 자신의 출력을 파일로 리디렉션해야 합니다. 이 페이지의 컨테이너 레시피에서 러너는 컨테이너의 PID 1이고 그 종료는 컨테이너를 종료하며, systemd의 기본 `KillMode=control-group` 아래에서 cgroup 전체 킬은 **Cgroup 전체 킬** 항목에서 설명하는 대로 훅에도 도달합니다. 둘 다에서 강제 종료를 훅에 대해 치명적인 것으로 취급하고 유예 기간 대신 의존하세요.
* **프로세스 그룹 전체 신호**(예: 래퍼 스크립트의 `kill -- -<pid>`, 셸 작업 제어, 또는 그룹 전체 감시자): 러너 및 중간 `checkout` 훅 서브프로세스에 도달합니다. 의도적으로 그룹 연결 상태를 유지하지만 실행 중인 `post-session` 훅 또는 세션 자식에는 도달하지 않습니다.
* **Cgroup 전체 킬**(예: systemd의 기본 `KillMode=control-group` 또는 `terminationGracePeriodSeconds`가 만료될 때 Kubernetes가 전체 컨테이너에 전달하는 `SIGKILL`): 훅을 포함한 모든 것에 도달합니다. 프로세스 그룹 격리는 이에 대해 보호하지 않으므로 유예 기간은 전체 드레인 경로를 포함해야 합니다.
* **훅의 자신의 타임아웃**: 훅이 `--post-session-hook-timeout-sec`을 초과하면 러너는 훅의 전체 프로세스 그룹에 `SIGTERM`을 보낸 다음 2초 후 `SIGKILL`을 보내므로 훅이 포크한 워커(예: tar, rsync, git)는 래퍼 셸과 함께 종료되고 고아로 생존하지 않습니다. 러너의 감독은 훅의 stdio가 닫히면 끝납니다: 자신의 출력을 파일로 리디렉션하고 `SIGTERM` 단계를 능가하는 워커는 러너의 범위를 벗어납니다.

드레인이 시작될 때와 강제 종료 시 러너는 여전히 실행 중인 `post-session` 훅의 수를 기록하므로 조용한 드레인과 스냅샷 중간 드레인을 구분할 수 있습니다.

<h2 id="keep-the-base-directory-and-capacity-identical-across-runners">
  기본 디렉토리 및 용량을 러너 간에 동일하게 유지
</h2>

러너가 세션 중간에 죽으면 서버는 세션을 재큐하고 환경의 다른 러너가 이를 선택합니다. 해당 러너는 자신의 `--base-dir` 및 `--capacity`에서 체크아웃 경로를 파생합니다: `--capacity 1`은 `--base-dir` 아래에 직접 체크아웃하고, `--capacity` 1 이상은 대신 세션별 worktree를 사용합니다. 동일한 환경의 러너가 두 플래그에 대해 다른 값을 사용하면 재개된 세션의 작업 디렉토리가 변경되고, 에이전트가 이전에 기록한 절대 경로(편집, 도구 호출, 또는 자신의 노트)는 더 이상 존재하지 않는 위치를 가리킵니다.

환경의 모든 러너에서 동일한 `--base-dir` 및 `--capacity`를 사용하고, 인스턴스 ID 또는 호스트 이름과 같은 호스트별 값을 사용하지 마세요.

기본 디렉토리는 [`--base-dir` 참조 행](/docs/ko/self-hosted-environments-reference#runner-cli-flags)이 기록하는 예외를 제외하고 `/workspace`로 기본값입니다. 러너는 쓰기 액세스가 필요합니다. 시작 시 등록하기 전에 러너는 디렉토리를 생성하고 쓸 수 있는지 확인하고, 할 수 없으면 `cannot create or write to base directory`로 종료합니다. root로 시작된 러너는 기본값 `/workspace`를 자체 생성합니다. root가 아닌 러너의 경우 디렉토리를 생성하고 러너를 시작하기 전에 러너의 사용자에게 소유권을 제공하거나 `--base-dir`을 해당 사용자가 이미 소유한 디렉토리로 가리키세요.

<h2 id="reuse-a-pre-warmed-checkout">
  사전 준비된 체크아웃 재사용
</h2>

대규모 리포지토리의 경우 클론이 세션 시작을 지배할 수 있습니다. `--capacity 1`에서 [`checkout` 훅](/docs/ko/self-hosted-environments-configuration#checkout) 없이 러너는 `<base-dir>/<repo-owner>/<repo>`에서 리포지토리당 하나의 정규 클론을 유지하고 세션 간에 재사용합니다: 요청된 ref를 가져오고, `HEAD`를 분리하고, 이에 대해 하드 리셋합니다. 거의 변경되지 않았을 때 거의 즉시입니다. 콜드 클론을 건너뛰려면 다음 두 가지 방법 중 하나로 클론을 제공하세요:

* **이미지에 클론**: 러너 이미지를 해당 경로에 빌드합니다. 모든 새로운 컨테이너는 디스크를 재사용하지 않고 준비된 클론으로 시작합니다.
* **지속적인 볼륨에 클론**: [`--lock-to-account`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)로 한 사용자의 계정에 사전 잠긴 러너에서 `--base-dir`을 지속적인 볼륨으로 가리키므로 디스크는 해당 계정만 제공합니다. 사전 잠긴 러너는 Claude Tag 채널 세션을 절대 선택하지 않으므로 이 옵션은 이들을 제공하는 러너에 적용되지 않습니다.

재사용 경로가 보장하고 보장하지 않는 것:

* **모든 클론 형태가 작동합니다**: 경로의 전체, 얕은, 또는 단일 분기 클론은 그대로 사용됩니다. 러너는 기존 클론으로 가져올 때 `--depth`를 절대 전달하지 않으므로 전체 사전 준비는 전체 기록을 유지하고 얕은 것은 얕게 유지됩니다. `CLAUDE_RUNNER_FETCH_DEPTH`(`full`, `0`, 또는 숫자; 기본값 50)는 클론이 아직 없을 때 러너가 만드는 콜드 클론만 제어합니다.
* **추적된 변경 사항 리셋, 추적되지 않은 파일 유지**: 각 세션은 이전 세션의 추적된 수정을 지우는 하드 리셋에서 시작하지만 러너는 절대 `git clean`을 실행하지 않으므로 잠긴 소유자의 이전 세션의 추적되지 않은 파일은 트리에 유지됩니다.
* **세션별 디렉토리도 유지됩니다**: 체크아웃 옆에 러너는 실행하는 모든 세션에 대해 `<base-dir>/_sessions/` 아래에 세션별 항목을 생성합니다. 세션의 Claude 설정 디렉토리는 대화 기록의 로컬 복사본을 보유합니다. 그 옆에는 세션이 있을 때 세션의 업로드된 파일이 있습니다. 세션 디렉토리도 거기에 있습니다: 세션이 실행되는 동안 세션별 worktrees 및 `checkout` 훅 체크아웃을 보유하며, Claude가 그 안에 작성한 다른 모든 것을 유지합니다.

  기본적으로 러너는 세션이 끝날 때 이들을 제자리에 두므로 러너 프로세스보다 오래 지속되는 디스크에서 누적됩니다. 모든 세션은 러너 자신의 사용자로 실행되므로 해당 디스크가 제공하는 나중의 모든 세션이 이들을 읽을 수 있습니다. 지속적인 `--base-dir`을 유지하면 해당 성장에 대해 볼륨의 크기를 조정하세요. 동일한 파일 시스템에서 러너를 다시 시작하는 모든 설정(예: [Docker Compose 레시피](#docker-compose))에도 동일하게 적용됩니다.
* **`--remove-session-state`를 사용하면 세션별 디렉토리가 유지되지 않습니다**: [`--remove-session-state`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)로 러너를 시작하여 세션이 끝날 때 각 세션의 세션별 디렉토리를 삭제하도록 합니다. 삭제는 최선의 노력입니다: 러너가 정리 실행 전에 종료되면 디렉토리가 유지됩니다. 정규 클론 및 세션이 호스트의 다른 곳(예: 임시 디렉토리)에 작성한 파일은 관계없이 유지됩니다.
* **git 프록시 포함, 리셋은 체크아웃이 됩니다**: [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy)로 러너는 각 세션 전에 클론의 `.git/`을 정제하여 객체 저장소, refs, 얕은 상태를 유지하지만 인덱스를 삭제하므로 각 세션은 거의 즉시 리셋 대신 전체 작업 트리 체크아웃을 지불합니다. 여전히 절대 재클론하지 않습니다. Submodule 사전 준비는 프록시 아래에서 지원되지 않습니다.
* **긴 클론은 해결 방법이 필요하지 않습니다**: 러너는 각 git 작업을 120초 진행 없음 감시자 및 30분 하드 캡으로 제한하며, 플랫 타임아웃이 아니므로 진행을 계속 보고하는 느린 콜드 클론이 완료됩니다.

<h2 id="pin-the-version">
  버전 고정
</h2>

각 세션의 자식 Claude Code 프로세스는 러너의 자신의 바이너리를 실행하고, 러너는 생성하는 세션 내에서 자동 업데이트를 끕니다. 따라서 모든 세션은 호스트에 설치하거나 이미지에 빌드한 버전을 실행합니다. 호스트 수준 업데이트는 러너가 다음 번에 시작할 때 적용됩니다.

* **플릿을 한 버전으로 유지하려면**: 이미지를 고정된 버전으로 빌드하거나 베어 호스트에 특정 버전을 설치하고 [자동 업데이트를 비활성화](/docs/ko/setup#disable-auto-updates)하세요.
* **업그레이드하려면**: 새로운 버전을 설치하거나 이미지를 다시 빌드한 다음 러너를 재시작하세요.
* **플러그인**: 플러그인 마켓플레이스도 자동 업데이트되지 않습니다. 바이너리가 고정된 상태로 유지되는 동안 플러그인이 자동 업데이트되도록 하려면 러너의 환경에서 `FORCE_AUTOUPDATE_PLUGINS=1`을 설정하세요.

<h2 id="scale-the-fleet">
  플릿 확장
</h2>

오케스트레이터는 러너를 추가하거나 제거할 시기를 결정합니다. [소유자별 러너 잠금](/docs/ko/self-hosted-environments#runner-lifecycle) 때문에 최소 복제본 수는 동시에 활성화될 것으로 예상되는 사용자 및 Claude Tag 에이전트의 수입니다. `--capacity`는 소유자 간이 아닌 한 소유자의 세션 내에서 병렬 처리를 제어합니다.

두 가지 확장 접근 방식을 사용할 수 있습니다:

* **고정 플릿**: 정적 러너 복제본 세트를 실행하고 각 러너가 제공하는 [Prometheus 메트릭](/docs/ko/self-hosted-environments-reference#prometheus-metrics)에서 확장하세요.
* **온디맨드 러너**: `claude self-hosted-runner orchestrator` 서브명령을 실행하세요. 이는 Anthropic에서 사용 가능한 러너 없이 대기 중인 세션을 폴링하고 `spawn-runner` 훅을 호출하여 세션당 하나를 부팅합니다. [온디맨드 러너](/docs/ko/self-hosted-environments-configuration#on-demand-runners)를 참조하세요.

<h2 id="known-issues-and-limitations">
  알려진 문제 및 제한 사항
</h2>

다음은 이 릴리스의 제한 사항이며, 해결 방법이 있는 경우 제시합니다.

<h3 id="connector-traffic-leaves-your-network">
  커넥터 트래픽이 네트워크를 벗어남
</h3>

Anthropic은 러너에서가 아니라 자체 인프라에서 커넥터 도구를 호출합니다. 커넥터 도구는 GitHub, Slack, Linear와 같은 claude.ai 커넥터입니다. Claude가 자체 호스팅 세션에서 커넥터를 사용할 때, 해당 트래픽은 네트워크 경계 내에서 발생하지 않고 `api.anthropic.com`을 통해 이동합니다.

자체 호스팅 세션에서 커넥터를 제외하려면 [`allowedMcpServers` 및 `deniedMcpServers` 정책 설정](/docs/ko/managed-mcp#policy-based-control-with-allowlists-and-denylists)으로 필터링합니다. Claude Code는 이러한 설정을 Anthropic이 제공하는 커넥터뿐만 아니라 러너 호스트에서 시드한 서버 및 사용자가 추가한 서버에도 적용하므로, 다른 서버에 대한 허용 목록을 배포하면 Claude Code는 제공된 커넥터도 차단합니다. 제공된 커넥터를 URL 기반 허용 목록과 함께 사용 가능하게 유지하려면 제공된 커넥터의 Anthropic 프록시 경로와 일치하는 항목을 추가합니다:

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

도구 트래픽이 네트워크 내부에 머물러야 하는 경우, 대신 러너 이미지에서 로컬 MCP 서버로 동등한 도구를 실행합니다. [MCP 서버](/docs/ko/self-hosted-environments-configuration#mcp-servers)를 참조합니다.

<h3 id="some-sessions-don’t-count-as-idle">
  일부 세션이 유휴 상태로 계산되지 않음
</h3>

백그라운드 작업을 보유한 세션이 완료되지 않으면 유휴 상태로 계산되지 않으므로 `--release-idle-session-min`은 해당 세션의 슬롯을 해제하지 않습니다. 실행 중인 도구 호출 내에서 요청된 승인을 기다리는 세션도 유휴 상태로 계산되지 않습니다. 항상 `--kill-session-after-min`을 함께 설정하여 어떤 세션도 슬롯을 무한정 보유할 수 없도록 하는 하드 백스톱으로 사용합니다.

`--kill-session-after-min`은 폭주 세션에 대한 백스톱입니다. v2.1.260 이상의 러너에서 제한에 도달한 세션은 즉시 종료되지 않습니다. 러너는 기본값 15분의 유예 기간을 제공하며, [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](/docs/ko/self-hosted-environments-reference#environment-variable-only-settings)로 변경할 수 있습니다:

* 세션이 사용자를 기다리고 있으면, 러너는 이를 해제합니다. 턴이 끝났으며 백그라운드 작업만 보유하고 있으면, 러너는 해당 작업이 완료될 때까지 최대 60초 동안 기다린 후 이를 해제합니다. 세션은 사용자가 다음 메시지를 보낼 때 재개됩니다.
* 턴이 여전히 실행 중이면, 러너는 턴이 완료될 때까지 또는 세션이 다음으로 사용자를 기다릴 때까지 기다린 후 해제합니다.
* 유예 기간이 끝났을 때 세션이 여전히 러너에 있으면, 러너는 이를 종료하고 실행 중인 턴의 작업은 손실됩니다. 실행 중인 도구 호출 내에서 요청된 승인을 기다리는 턴은 세션이 기간을 초과하는 한 가지 방법입니다.

해제된 세션은 새로운 클론에서 재개되므로 푸시하지 않은 작업은 어느 쪽이든 손실됩니다. [재개된 세션이 푸시되지 않은 작업을 손실함](#additional-limitations)을 참조합니다. v2.1.260 이전에는 러너가 제한에 도달한 모든 세션을 종료했으며, 실행 중인 턴이 완료될 때까지 최대 유예 기간 동안 기다렸습니다.

예를 들어 `--kill-session-after-min 480`으로 8시간 동안 가장 긴 예상 세션 위에 플래그를 설정합니다. 유휴 상태가 된 대화에서 슬롯을 해제하려면 대신 `--release-idle-session-min`을 사용합니다.

<h3 id="additional-limitations">
  추가 제한 사항
</h3>

* **재개된 세션이 푸시되지 않은 작업을 손실함**: 세션이 해제되거나 러너가 재시작되고 사용자가 다른 메시지를 보내면, 세션은 저장소를 시작 분기에서 다시 복제하는 새로운 러너에서 재개되므로 세션이 푸시하지 않은 작업은 손실됩니다. [`--push-outcome-on-release`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)를 설정하여 러너가 해제하기 전에 세션의 결과 분기를 최선의 노력으로 푸시하도록 하면, 재개된 세션이 해당 커밋에서 시작되므로 커밋된 작업이 보존됩니다. 더티 워킹 트리는 아닙니다. 활성화하기 전에 소스 원격의 `claude/*` refs에 푸시할 수 있는 사람을 제한합니다. 예를 들어 분기 규칙 집합을 사용합니다: 재개 시 러너는 이전에 푸시된 분기를 누가 푸시했는지 확인하지 않고 가져오므로, 해당 refs에 푸시 액세스 권한이 있는 모든 사람이 재개된 워크스페이스에 콘텐츠를 배치할 수 있습니다. 러너는 또한 재개 시 세션별 구성을 삭제하므로, 세션의 Claude 구성 디렉터리 및 세션이 작성한 모든 셸 상태가 삭제됩니다. `--push-outcome-on-release`는 이를 포함하지 않습니다.
* **비공개 저장소는 세션 중간에 추가할 수 없음**: 세션이 시작된 후 세션에 추가된 저장소는 자체 호스팅 러너에서 자격 증명으로 복제되지 않으므로 추가가 실패합니다. 세션을 만들 때 세션이 필요한 모든 저장소를 선택합니다.
* **일부 커넥터는 자체 호스팅 세션에 나타나지 않음**: claude.ai 설정에서 아직 연결하지 않은 커넥터는 자체 호스팅 세션에 나열되지 않으며, 세션은 연결하라는 메시지를 표시하지 않습니다. 먼저 설정에서 연결한 후 새로운 세션을 시작합니다. 이미 실행 중인 세션에 커넥터를 추가해도 Claude에서 해당 도구를 사용할 수 없습니다. 새로 추가된 커넥터를 선택하려면 새로운 세션을 시작합니다.

<h3 id="report-an-issue">
  문제 보고
</h3>

자체 호스팅 환경의 문제는 Anthropic 계정 팀에 문의합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

안내 진단을 위해 러너 호스트에서 doctor 서브명령을 실행하세요. doctor 서브명령은 러너의 로그 및 상태가 첨부된 대화형 Claude Code 세션을 시작합니다. 해당 호스트에서 먼저 `claude auth login`으로 서명하여 세션이 환경, 러너, 대기 중인 세션을 쿼리할 수 있도록 합니다. 해당 서명 없이(예: 호스트가 API 키로 인증할 때) 로컬 상태 엔드포인트, 메트릭, 러너의 로그로 제한되며, `--log-file`로 러너를 시작한 경우에만 로그를 읽습니다.

```bash theme={null}
claude self-hosted-runner doctor
```

일반적인 문제:

* **러너가 환경에 나타나지 않음**: 호스트가 HTTPS를 통해 `api.anthropic.com`에 도달할 수 있는지, 환경 시크릿이 현재인지, 호스트 시계가 실제 시간의 5분 이내인지 확인하세요. 더 큰 스큐는 인증 실패를 유발합니다. 러너는 인증 실패 시 거부 이유와 함께 `[runner:fatal]`을 기록합니다.
* **러너가 `cannot create or write to base directory`로 시작 시 종료됨**: 러너가 `--base-dir`을 생성하거나 쓸 수 없습니다. 기본값은 `/workspace`입니다. 디렉토리의 소유권을 수정하거나 `--base-dir`을 쓰기 가능한 경로로 가리키세요. [기본 디렉토리 및 용량을 러너 간에 동일하게 유지](#keep-the-base-directory-and-capacity-identical-across-runners)에서 설명합니다. 러너가 대신 기본 디렉토리 확인이 시간 초과되었다고 `[runner:fatal]`을 기록하면 디렉토리는 행(hung) NFS 또는 CSI 마운트에 있습니다. 권한이 아닌 마운트 상태를 확인하세요. 러너는 `--log-file`을 열기 전에 이러한 시작 실패를 stderr에 인쇄하므로 로그 파일이 아닌 터미널 또는 플랫폼의 컨테이너 로그에서 찾으세요. v2.1.225 이전에 러너는 시작 시 기본 디렉토리를 확인하지 않았으며, 이 잘못된 구성은 대신 선택 후 세션에 실패했습니다.
* **세션이 대기 중 상태로 유지됨**: 모든 온라인 러너는 다른 소유자로 잠길 수 있습니다. 각 러너의 `claude_code_self_hosted_runner_locked_account` [메트릭](/docs/ko/self-hosted-environments-reference#prometheus-metrics) 또는 `[runner:health]` 로그 라인의 `locked_account` 필드를 확인하여 누가 보유하는지 확인하세요. 둘 다 러너가 `act.email` 클레임을 전달하는 세션 토큰을 발급받은 후에만 소유자의 이메일을 표시합니다. Claude Tag 에이전트의 세션은 절대 이를 수행하지 않습니다. 클레임 없이 러너는 `locked_account` 시리즈를 내보내지 않으며 `locked_account=yes`를 기록합니다. 이는 러너가 잠겨 있지만 어느 소유자에게 잠겨 있는지 알려줍니다. 복제본을 추가하거나 기존 러너가 드레인되고 재시작될 때까지 기다리세요. 환경이 온디맨드 러너를 사용하면 대신 오케스트레이터를 확인하세요. [온디맨드 러너](/docs/ko/self-hosted-environments-configuration#on-demand-runners)를 참조하세요.
* **세션이 선택 직후 실패함**: claude.ai/code에서 세션을 열어 오류를 확인하세요. 가장 일반적인 원인은 러너 이미지의 누락된 [git 자격증명](#configure-git) 및 설치되지 않은 빌드 도구입니다. 쓰기 불가능한 기본 디렉토리는 세션 실패 대신 시작 시 러너를 중지합니다. 이 목록의 **러너가 `cannot create or write to base directory`로 시작 시 종료됨** 항목을 참조하세요.
* **세션이 인증하는 이그레스 프록시를 통해 네트워크에 도달할 수 없음**: [`--proxy-authorization-command` 또는 `--proxy-authorization-file`](#authenticate-to-an-egress-proxy)로 설정한 소스가 실패하거나 30초 후 시간 초과되거나 빈 값을 생성하면 러너는 해당 연결에 `502 Bad Gateway`로 응답하고 이유를 기록합니다. 러너는 해당 로그에서 명령의 stderr를 수정하고 헤더 값을 절대 기록하지 않습니다. `--proxy-authorization-command`로 호스트에서 명령을 직접 실행하여 전체 헤더 값을 stdout에 인쇄하는지 확인하세요. 러너가 대신 `could not start the proxy-authorization listener`로 시작 시 종료되면 루프백 리스너를 열 수 없습니다.
* **러너 로그에 `rejecting the malformed poll response`를 포함하는 `Poll failed` 라인**: 러너가 큐의 예상 JSON이 아닌 본문을 가진 작업 폴 응답을 받았습니다. 가장 자주 러너와 `api.anthropic.com` 사이의 무언가(예: 가로채는 프록시 또는 캡티브 포털)가 자신의 페이지로 응답했기 때문입니다. 러너는 응답을 거부하고 [`claude_code_self_hosted_runner_poll_errors_total` 메트릭](/docs/ko/self-hosted-environments-reference#prometheus-metrics)의 `transport` 종류 아래에서 계산하고 [세션 수명 주기](/docs/ko/self-hosted-environments#session-lifecycle)에서 설명하는 실패한 폴 일정에서 재시도합니다. 러너는 라이브 세션을 계속 제공합니다. 프록시를 구성하여 `api.anthropic.com`의 응답을 변경되지 않은 상태로 전달하세요. v2.1.246 이전에 러너는 그러한 응답을 빈 작업 큐로 읽었으며, 이는 라이브 세션을 종료하거나 종료하게 할 수 있습니다.
* **세션의 분기가 원격에 더 이상 존재하지 않음**: 세션이 읽기만 하는 git 소스의 경우 러너는 해당 소스를 건너뛰고 나머지에서 계속합니다. 세션이 결과를 푸시하는 소스의 경우 삭제된 분기(일반적으로 병합되고 자동 삭제되었기 때문)는 리포지토리 및 분기를 이름 지정하고 분기를 복원하고 재시도하도록 요청하는 오류로 세션을 실패합니다. 러너는 건너뛰기가 리포지토리 없이 남겨질 때 동일한 오류로 세션을 실패합니다. v2.1.228 이전에 그러한 세션은 빈 디렉토리에서 시작했습니다.
* **세션이 해당 리포지토리 중 하나 없이 시작됨**: [`checkout` hook](/docs/ko/self-hosted-environments-configuration#checkout)이 없는 러너에서 git 호스트는 세션이 읽기만 하는 리포지토리에 대한 러너의 액세스 확인을 거부할 수 있습니다. 그러면 러너는 해당 리포지토리를 건너뛰고 거부를 이름 지정하는 `[runner:warn] could not access context source` 라인을 기록하고 나머지에서 세션을 시작합니다.

  러너는 명확한 거부만 건너뜁니다. 호스트가 리포지토리를 찾을 수 없다고 응답하거나, git이 호스트에 대한 자격증명을 찾지 못하거나, 인증이 실패합니다. 네트워크 실패, 시간 초과, 또는 HTTP `403`은 여전히 세션 시작을 실패하게 하며, 세션이 결과를 푸시하는 리포지토리에 대한 거부도 마찬가지입니다. 러너는 여전히 건너뛰기가 리포지토리 없이 남겨질 세션을 실패합니다. [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy)를 사용하면 러너는 git 프록시 자체가 거부하는 리포지토리만 건너뜁니다.

  액세스 확인은 세션이 러너에서 시작될 때마다 다시 실행되므로, 러너의 git 아이덴티티가 읽기 액세스를 가지면 다음 시작은 리포지토리를 복제합니다. v2.1.274 이전에 이러한 각 거부는 세션 시작을 실패했습니다.
* **세션이 시작하는 데 분이 걸림**: 초기 클론이 일반적으로 지배합니다. `claude_code_self_hosted_runner_session_init_duration_seconds` [메트릭](/docs/ko/self-hosted-environments-reference#prometheus-metrics)을 확인하여 확인하고 [사전 준비된 체크아웃](#reuse-a-pre-warmed-checkout) 또는 더 작은 `CLAUDE_RUNNER_FETCH_DEPTH`로 클론을 자르세요.
* **턴이 401로 실패함**: 각 세션은 러너가 Anthropic에서 가져오고 세션의 stdin을 통해 회전하는 단기 [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ko/self-hosted-environments-configuration#wrapper-scripts)으로 모델 호출을 인증합니다. 턴이 모델 API에서 401 또는 403으로 끝나면 러너는 새로운 토큰을 가져오고 세션에 전달합니다. 실패한 턴은 재시도되지 않습니다.

  가져오기가 실패하면 러너는 언제 재시도할지 말하는 `inference_token refresh failed` 라인을 기록하고 세션이 실행되는 동안 계속 재시도합니다.

  모든 호출이 세션 약 30분 후에 실패하기 시작하면 래퍼 스크립트가 세션의 stdin을 끊었을 가능성이 높으므로 토큰 회전이 도달할 수 없습니다. [stdin 및 파일 디스크립터 3 유지](/docs/ko/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached)를 참조하세요.

  v2.1.274 이전에 러너는 실패한 가져오기 후 몇 번의 시도 후 재시도를 중지하고 다음 예약된 것을 기다렸습니다. 실패한 턴은 가져오기를 트리거하지 않았으므로 모든 턴은 다음 예약된 가져오기까지 401로 실패했습니다.
* **Pod이 드레인 중간에 종료됨**: `terminationGracePeriodSeconds`를 최소한 러너가 시작 시 기록하는 값으로 올리세요. [종료 타이밍](#shutdown-timing)을 참조하세요.

로깅이 초기화되면 러너는 수명 주기 로그(JSON이 아닌 일반 텍스트 라인으로 `[runner:fatal]` 라인 포함)를 stdout에 쓰고 디버그 출력을 stderr에 씁니다. 위의 문제 해결 항목에서 설명하는 시작 실패는 그 지점 전에 stderr에 인쇄됩니다. `--log-file`로 두 스트림을 캡처하세요. 이는 또한 `self-hosted-runner doctor`가 이들을 추적하도록 합니다. 또는 플랫폼의 로그 수집으로 캡처하세요.

각 세션의 자식 프로세스는 별도의 디버그 로그를 작성합니다. 실패 시 러너는 로그의 꼬리를 claude.ai/code의 세션과 함께 표시합니다. [`--remove-session-state`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)로 러너를 시작하지 않은 경우 실패한 세션의 로그를 디스크에 유지하고 러너 로그에서 해당 경로를 인쇄합니다.

<h2 id="what’s-next">
  다음 단계
</h2>

* [세션 사용자 정의](/docs/ko/self-hosted-environments-configuration): 래퍼 스크립트, 수명 주기 훅, 온디맨드 러너, MCP 서버, 권한
* [엔드 투 엔드 테스트](/docs/ko/self-hosted-environments-testing): CI에서 새로운 러너 이미지를 확인한 후 프로모션
* [참조](/docs/ko/self-hosted-environments-reference): 모든 CLI 플래그, 환경 변수, 메트릭
