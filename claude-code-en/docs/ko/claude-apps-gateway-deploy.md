> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 앱 게이트웨이 배포 및 운영

> IdP에 게이트웨이를 등록하고, 컨테이너를 빌드하며, Kubernetes 또는 Cloud Run에 배포하고 운영합니다: 상태 확인, 시크릿 로테이션, 업그레이드 및 보안.

이 페이지는 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 실행의 운영 측면을 다룹니다: 게이트웨이를 ID 공급자(IdP)에 등록하고, 게이트웨이를 컨테이너로 배포하며, 일상적으로 운영합니다. 게이트웨이가 부팅 시 읽는 `gateway.yaml` 파일의 모든 옵션에 대해서는 [구성 참조](/docs/ko/claude-apps-gateway-config)를 참조하세요.

프로덕션 배포는 순서대로 4단계를 따르며, 아래 섹션이 이를 일치시킵니다. 처음 두 단계는 선택을 하는 곳이고, 나머지 두 단계는 실행 중일 때 참조할 참고 자료입니다.

1. [ID 공급자 설정](#identity-provider-setup): OAuth 클라이언트를 등록하고 Okta, Entra, Google에 대한 IdP별 참고사항 확인
2. [게이트웨이 배포](#deployment): 고정된 컨테이너 이미지를 빌드하고 Kubernetes, Cloud Run 또는 자신의 플랫폼에서 실행합니다. 이 섹션은 비용, 우회, 다중 게이트웨이 및 서버리스 결정도 다룹니다
3. [운영 설정](#operations): 로그, 상태 프로브, 중단 동작, 시크릿 로테이션 및 업그레이드. 모니터링 및 런북을 연결할 때 참조할 참고 자료
4. [보안 태세 검토](#security): 데이터가 어디로 흐르는지, 위협 모델 및 규정 준수 답변. 보안 검토를 위한 참고 자료

로그인 또는 부팅이 실패하면 [문제 해결](#troubleshooting)로 바로 이동하세요. 이는 표시되는 오류를 기준으로 구성되어 있습니다.

<Note>
  **프라이빗 네트워크에 배포하세요.** Claude Code는 주소가 프라이빗인 게이트웨이에만 연결합니다. 이는 보안 가드입니다. 신뢰할 수 있는 게이트웨이는 개발자 머신에서 명령을 실행하는 설정을 푸시할 수 있기 때문입니다. 게이트웨이를 내부 로드 밸런서 또는 VPN 뒤에 배치하고 프라이빗 IP로만 확인되는 호스트명을 지정하세요.

  Anthropic이 운영하는 공개 게이트웨이 엔드포인트는 예외입니다: `/login`은 `https://`를 통해 이들을 수락합니다. 이는 Anthropic 자체가 운영하는 작은 고정 게이트웨이 세트입니다. 이들은 선택하거나 구성할 수 있는 배포 옵션이 아닙니다. 목록은 Claude Code에 컴파일되므로 구성이 호스트명을 추가할 수 없으며 호스팅하는 게이트웨이는 면제 대상이 될 수 없습니다. v2.1.206 이전에는 `/login`이 다른 공개 주소처럼 이러한 엔드포인트를 거부했습니다.
</Note>

<h2 id="identity-provider-setup">
  ID 공급자 설정
</h2>

단일 리디렉션 URI `https://<gateway>/oauth/callback`을 사용하여 기밀 OAuth/OpenID Connect(OIDC) 웹 애플리케이션을 ID 공급자에 등록하고, 게이트웨이 액세스 권한이 있어야 하는 사용자 또는 그룹에 할당하세요.

모든 OIDC 호환 IdP가 작동합니다: Okta, Microsoft Entra ID, Google Workspace, Keycloak, Dex, PingFederate 등. IdP는 세 가지 요구사항을 충족해야 합니다:

* `/.well-known/openid-configuration`을 프로덕션에서 HTTPS를 통해 제공합니다. 게이트웨이는 [`http://` 발급자](/docs/ko/claude-apps-gateway-config#oidc)를 허용하며, 루프백 발급자는 추가로 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`이 필요합니다
* 인증 코드 흐름을 지원합니다. PKCE(Proof Key for Code Exchange)는 기본적으로 활성화되어 있습니다. 이를 지원하지 않는 IdP의 경우 `oidc.use_pkce: false`로 비활성화하세요
* id\_token에서 `email`을 반환하고 선택적으로 `groups`를 반환하거나, `oidc.userinfo_fallback: true`를 사용하여 userinfo 엔드포인트에서 제공합니다

프라이빗 PKI의 경우 `oidc.ca_cert_pem`을 설정하세요.

일부 공급자는 이메일 및 그룹 클레임을 다르게 처리합니다:

* **Okta**: `https://example.okta.com`의 조직 인증 서버는 `email` 및 `groups`를 생략하는 얇은 id\_token을 반환하므로, 이를 `issuer`로 사용할 때마다 `oidc.userinfo_fallback: true`를 설정하세요. `https://example.okta.com/oauth2/default`와 같은 id\_token에 `email` 및 선택적으로 `groups`를 포함하는 사용자 정의 인증 서버는 이를 직접 내보내며 폴백이 필요하지 않습니다. Okta는 `oidc.scopes`에서 `groups` 범위가 요청되고 앱의 그룹 클레임 필터가 이를 허용할 때만 `groups`를 내보냅니다. `userinfo_fallback`은 IdP가 요청하지 않은 클레임을 채울 수 없습니다.
* **Microsoft Entra ID**: `issuer` = `https://login.microsoftonline.com/<tenant-id>/v2.0`. Entra는 이름이 아닌 그룹 개체 ID를 내보내므로, `managed.policies.match.groups`에서 GUID를 사용하거나 인간이 읽을 수 있는 이름을 위해 앱 역할을 사용하세요. 테넌트가 `groups` 대신 `roles` 아래에 역할을 내보내는 경우 `oidc.groups_claim: roles`을 설정하세요.
* **Google Workspace**: `issuer` = `https://accounts.google.com`. Google의 id\_token은 그룹을 전달하지 않습니다. Google을 IdP로 사용하여 그룹 기반 `allowed_groups` 또는 `managed.policies`를 사용하려면 [`oidc.google_groups`](/docs/ko/claude-apps-gateway-config#oidc)를 구성하세요. 이는 도메인 전체 위임이 있는 서비스 계정을 사용하여 Admin SDK Directory API를 통해 각 사용자의 그룹을 조회합니다. 이 없이는 멤버십 게이팅을 위해 `oidc.allowed_email_domains`를 사용하고 정책 할당을 위해 `managed.policies.match.email_domain`을 사용하세요. Google은 또한 표준 `offline_access` 범위를 무시합니다. 새로고침 토큰의 경우 `oidc.scopes: [openid, profile, email]`과 `oidc.extra_auth_params: { access_type: offline, prompt: consent }`를 설정하세요.

위에서 다루지 않은 ID 공급자에 대한 지원은 [문제 해결](#troubleshooting)을 참조하세요.

<Warning>
  새로고침 토큰을 사용하면 게이트웨이가 개발자를 브라우저로 다시 보내지 않고 개발자의 세션을 자동으로 갱신할 수 있습니다. 또한 IdP가 사용자를 비활성화할 때 다음 새로고침이 실패하고 세션이 `ttl_hours` 내에 종료되므로 프로비저닝 해제를 주도합니다. 게이트웨이는 기본적으로 새로고침 토큰을 얻기 위해 `offline_access`를 요청합니다. IdP가 오프라인 액세스에 대한 명시적 동의를 요구하는 경우 OAuth 클라이언트를 구성하여 이를 허용하세요.

  IdP가 전혀 새로고침 토큰을 발급할 수 없는 경우, 게이트웨이는 여전히 작동하지만 자동 갱신이 없으므로 개발자는 세션이 만료될 때 브라우저 로그인을 다시 실행합니다. 이것이 매시간 발생하지 않도록 하려면 [`session.ttl_hours`](/docs/ko/claude-apps-gateway-config#session)를 `8` 또는 `12`로 올리세요. 트레이드오프는 프로비저닝 해제 지연입니다. 새로고침 토큰이 없으면 비활성화된 사용자는 더 긴 TTL이 경과할 때까지 액세스를 유지합니다.
</Warning>

<h2 id="deployment">
  배포
</h2>

게이트웨이는 단일 Linux 바이너리입니다. 복제본이 상태 비저장이고 Postgres가 공유 조정 계층이므로 수평으로 확장됩니다. 환경에서 상태 비저장 서비스를 실행하는 방식대로 실행하세요. 이 섹션의 나머지 부분은 이미지가 필요한 것을 설명하며, Kubernetes 및 Cloud Run에 대한 짧은 참고사항이 있습니다.

게이트웨이는 업스트림 자격 증명을 보유하고 추론을 위한 단일 송신 지점으로 작동하므로 네트워크 내부에서 실행되도록 설계되었습니다. 개발자와 IdP가 HTTPS를 통해 도달할 수 있는 곳이면 어디든 실행할 수 있습니다. 프로덕션 자격 증명을 보유하는 다른 서비스처럼 취급하세요.

몇 가지 결정이 실행 위치 이상으로 배포를 형성합니다:

* **비용**: 게이트웨이에 대한 별도의 라이선스 또는 사용자당 요금이 없습니다. 이는 `claude` 바이너리의 일부입니다. 기존 클라우드 또는 Anthropic 약정을 통해 추론에 대해 비용을 지불하고, 컨테이너 및 텔레메트리 수집기의 컴퓨팅을 지불합니다.
* **우회**: 게이트웨이는 모델로의 유일한 경로가 이를 통과하도록 강제하지 않습니다. 자신의 자격 증명이 있는 개발자는 여전히 공급자를 직접 호출할 수 있으므로, 해당 경로를 닫는 것은 네트워크 정책 결정입니다. 예를 들어 `api.anthropic.com`으로의 송신을 게이트웨이를 제외하고 차단합니다. 해당 송신을 차단하면 각 개발자의 머신에서 `api.anthropic.com`을 호출하는 [WebFetch 도메인 안전 확인](/docs/ko/data-usage#webfetch-domain-safety-check)도 중단됩니다. 관리형 정책에서 `skipWebFetchPreflight: true`를 설정하여 비활성화하세요.
* **다중 게이트웨이**: 각 게이트웨이는 자신의 구성을 가진 별도의 배포입니다. CLI는 게이트웨이 호스트명별로 신뢰 지문 및 자격 증명을 저장하므로, 다른 팀이 충돌 없이 다른 게이트웨이에 연결할 수 있습니다. 여러 OIDC 발급자를 제공하려면 별도의 인스턴스를 실행하세요.
* **서버리스**: Cloud Run이 작동합니다. 콜드 OIDC 검색을 피하려면 `min-instances: 1`을 설정하세요. Lambda 및 Cloud Functions는 작동하지 않습니다. 게이트웨이는 장기 실행 HTTP 서버이기 때문입니다.

여기의 모든 프로덕션 토폴로지는 일반 HTTP 복제본 앞에 Ingress, Cloud Run의 프론트 엔드 또는 ALB와 같은 L7 프록시를 배치합니다. [`listen.trusted_proxies`](/docs/ko/claude-apps-gateway-config#listen)를 프록시의 소스 범위로 설정하여 게이트웨이가 `X-Forwarded-For`에서 클라이언트 IP를 읽도록 하세요. 게이트웨이는 TCP 피어가 신뢰할 수 있을 때만 헤더를 인정합니다. [Google Cloud 작동 예제](/docs/ko/claude-apps-gateway-on-gcp)는 토폴로지별 구체적인 값을 가지고 있습니다. 신뢰할 수 있는 프록시가 없으면, 모든 요청이 프록시의 IP에서 오는 것으로 나타나므로 IP당 속도 제한이 하나의 공유 버킷으로 축소되고 감사 이벤트에 프록시의 IP가 기록됩니다.

<h3 id="container-image">
  컨테이너 이미지
</h3>

표준 Claude Code 릴리스의 네이티브 `claude` 바이너리 주위에 자신의 이미지를 빌드하세요:

1. 고정된 릴리스에서 이미지 아키텍처용 Linux 빌드를 다운로드하세요. 다운로드 URL은 [특정 버전 설치](/docs/ko/setup#install-a-specific-version)를 참조하세요.
2. [바이너리 무결성 및 코드 서명](/docs/ko/setup#binary-integrity-and-code-signing)에 설명된 대로 릴리스의 GPG 서명된 `manifest.json`에 대해 확인하세요.
3. 빌드 컨텍스트에 복사하세요.

빌드가 릴리스 호스트에 도달할 수 없는 경우 릴리스를 내부 레지스트리로 미러링하고 플릿이 실행하는 버전을 고정하세요.

바이너리 외에도 이미지는 다음이 필요합니다:

* **glibc 기반 이미지**: glibc 빌드의 유일한 동적 종속성은 glibc 라이브러리입니다. Musl 기반 이미지는 `linux-x64-musl` 또는 `linux-arm64-musl` 빌드와 추가 패키지가 필요합니다. [Alpine Linux 설정](/docs/ko/setup#alpine-linux-and-musl-based-distributions)을 참조하세요.
* **쓰기 가능한 상태 디렉토리**: 게이트웨이는 모든 사용자로 실행되지만, 최소 이미지에는 쓰기 가능한 홈이 없습니다. `CLAUDE_CONFIG_DIR`을 `/tmp/.claude`와 같은 쓰기 가능한 경로로 설정하세요.
* **컨테이너 명령**: `claude gateway --config /etc/claude/gateway.yaml`. 구성 파일은 읽기 전용으로 마운트되고 시크릿은 환경 변수로 제공됩니다. 게이트웨이는 `listen.port`에서 수신하며, 기본값은 `8080`입니다.

<h3 id="kubernetes">
  Kubernetes
</h3>

게이트웨이를 모든 상태 비저장 서비스처럼 배포로 실행하세요:

* ConfigMap에서 구성을 마운트하고 Secret에서 시크릿을 마운트하세요. YAML에서 `${file:/path/to/secret}` 또는 환경 변수를 통해 시크릿을 참조하세요
* Ingress에서 TLS를 종료하고 `listen.public_url`을 Ingress 호스트명으로 설정하세요
* 준비 프로브를 `GET /readyz`로 지정하고 생존 프로브를 `GET /healthz`로 지정하세요

<Note>
  **워크로드 ID**

  정적 키보다 플랫폼의 워크로드 ID를 선호하세요: EKS의 Bedrock용 IRSA, GKE의 Agent Platform용 Workload Identity, AKS의 Foundry용 워크로드 ID. 업스트림 블록에서 `auth: {}`를 설정하거나 Foundry의 경우 `use_azure_ad: true`를 설정하면, 게이트웨이는 해당 공급자의 기본 자격 증명 체인을 통해 포드의 ID를 선택합니다. Bedrock 업스트림을 GKE에서 사용하는 경우와 같은 클라우드 간 페어링의 경우, 업스트림의 `auth` 블록에서 명시적 자격 증명을 설정하세요. [`upstreams` 참조](/docs/ko/claude-apps-gateway-config#upstreams)는 플랫폼별 설정 세부사항을 가지고 있습니다.
</Note>

<h3 id="cloud-run">
  Cloud Run
</h3>

서비스를 다음과 같이 구성하세요:

* `listen.port`를 기본값 `8080`으로 유지하세요. 이는 Cloud Run의 기본 `PORT`와 일치하거나 `port: ${PORT}`를 설정하세요
* `public_url`을 외부에서 도달 가능한 원본으로 설정하세요. 프로덕션의 경우 이는 일반적으로 내부 로드 밸런서의 호스트명입니다. `/login`이 [공개 주소를 거부](/docs/ko/claude-apps-gateway#prerequisites)하고 `*.run.app` URL이 하나로 확인되므로, Cloud Run URL 단독은 `curl` 또는 브라우저 스모크 테스트에만 작동합니다. 예외는 `*.run.app`이 Private Service Connect를 통해 프라이빗으로 확인되고 Cloud DNS 프라이빗 영역이 있는 네트워크입니다. 해당 토폴로지에서 Cloud Run URL은 유효한 `public_url`입니다. [Google Cloud 작동 예제](/docs/ko/claude-apps-gateway-on-gcp#deploy-the-gateway)는 둘 다 다룹니다.
* 구성을 시크릿 볼륨으로 마운트하세요
* 첫 번째 요청에서 콜드 OIDC 검색을 피하려면 `min-instances: 1`을 설정하세요

<Note>
  Cloud Run 또는 GKE, Cloud SQL 및 Secret Manager를 다루는 Google Cloud의 완전한 작동 예제는 [Google Cloud에 배포](/docs/ko/claude-apps-gateway-on-gcp)를 참조하세요.
</Note>

<h3 id="push-the-gateway-url-to-developer-machines">
  게이트웨이 URL을 개발자 머신으로 푸시
</h3>

게이트웨이가 제공되면, MDM을 통해 또는 OS별 `managed-settings.json`을 직접 작성하여 관리형 설정을 통해 각 개발자의 머신으로 `forceLoginMethod` 및 `forceLoginGatewayUrl`을 푸시하세요. 이 없이는 `/login`이 게이트웨이 옵션이 없는 표준 계정 선택기를 표시합니다. 파일 경로는 [클라이언트 측 관리형 설정](/docs/ko/claude-apps-gateway-config#client-side-managed-settings)을 참조하세요.

<h2 id="operations">
  운영
</h2>

게이트웨이가 트래픽을 제공하면, 일상적인 운영은 로그를 읽고, 상태를 프로브하며, 일정에 따라 시크릿을 로테이션하는 것입니다. 아래 섹션은 각각을 다루고, Postgres가 보유한 것과 업그레이드 및 롤백이 어떻게 동작하는지를 다룹니다.

<h3 id="logs">
  로그
</h3>

게이트웨이는 stderr에 두 개의 스트림을 작성하며, 둘 다 JSON 친화적입니다:

* **감사 이벤트**: 보안 관련 이벤트당 한 줄의 JSON. stderr를 로그 수집기로 파이프하세요. 내보낸 이벤트는 `config.load`, `session.mint`, `session.refresh`, `device.authorize`, `device.verify`, `auth.denied`, `access.denied`, `inference`, `managed.serve`, `spend.blocked` 및 `admin.denied`를 포함합니다. 필드는 이벤트에 따라 다릅니다:
  * 성공적인 mint 및 refresh 이벤트는 `sub`, `email`, `client_ip` 및 결과를 전달합니다
  * 거부 이벤트는 이유, 경로 및 클라이언트 IP를 전달합니다. 거부 시 ID가 없기 때문입니다
  * `inference`는 어느 업스트림이 요청을 제공했는지 및 응답 상태를 기록합니다
  * `admin.denied`는 거부된 관리자 API 인증 시도를 이유(`invalid_key` 또는 `no_credentials`), 클라이언트 IP, 메서드 및 경로와 함께 기록하며, 제시된 키 자료는 없습니다
* **운영 로그**: 부팅, 경고 및 업스트림 오류에 대한 인간이 읽을 수 있는 `[gateway]` 접두사 줄. `CLAUDE_GATEWAY_LOG_LEVEL` 환경 변수는 상세도를 제어하고 `info`, `warn` 또는 `error`를 허용하며, 기본값은 `info`입니다. 감사 이벤트는 항상 내보내지므로 이는 감사 이벤트에 영향을 주지 않습니다.

<h3 id="health">
  상태
</h3>

게이트웨이는 `GET /healthz`를 생존 프로브로 제공하고 `GET /readyz`를 준비 프로브로 제공합니다. `/readyz`는 저장소에 도달 가능한지 확인합니다. 둘 다 `access_control.allow_cidrs`에서 제외되므로 프로브는 잠긴 리스너에서 작동합니다.

`/.well-known/oauth-authorization-server`의 OAuth 검색 문서는 구성 로드, OIDC 검색, 업스트림 클라이언트 구성 및 Postgres 마이그레이션이 모두 성공한 후에만 `200`을 반환하므로, 엔드 투 엔드 부팅 확인으로도 작동합니다.

실행 중인 게이트웨이는 또한 `<public_url>/protocol`에서 실행 중인 버전과 일치하는 수락하는 경로 및 요청 형태의 설명을 제공합니다. 내용은 릴리스 간에 안정적이지 않습니다.

<h3 id="outage-behavior">
  중단 동작
</h3>

Postgres가 다운되면, 게이트웨이 자체는 로그인한 개발자를 계속 제공하고 새로운 로그인은 실패합니다. 개발자가 실제로 계속 작동하는지는 오케스트레이터가 준비를 처리하는 방식에 따라 다릅니다:

* **기존 세션**: 베어러 토큰은 JWT 시크릿으로 로컬에서 검증되고, 세션 새로고침은 저장소를 건드리지 않으며, 게이트웨이 프로세스는 여전히 추론을 제공할 수 있습니다
* **새로운 로그인**: Postgres가 복구될 때까지 실패합니다. 장치 흐름 및 속도 제한 카운터가 Postgres에 있기 때문입니다
* **[지출 제한 적용](/docs/ko/claude-apps-gateway-spend-limits#postgres-availability)**: 중단 중에 기본적으로 열린 상태로 실패하므로 추론이 계속 흐릅니다. 차단하는 것을 선호하면 닫힌 상태로 뒤집으세요
* **준비**: `/readyz`는 중단 중에 준비되지 않음을 보고하므로, 준비에 대한 트래픽을 게이트하는 오케스트레이터는 Postgres가 복구될 때까지 모든 복제본을 한 번에 로테이션에서 제거합니다. 해당 토폴로지에서 게이트웨이가 여전히 제공할 수 있는 추론을 포함한 모든 트래픽은 로드 밸런서에서 실패합니다. `/healthz`의 생존 프로브는 계속 통과하므로 복제본은 다시 시작되지 않습니다. 로그인한 개발자가 저장소 중단을 통해 계속 작동하도록 하려면 준비 프로브를 `/healthz`로 지정하세요. 비용은 새로운 로그인이 여전히 준비됨을 보고하는 복제본에 대해 실패한다는 것입니다.

IdP가 다운되면, 기존 세션은 `ttl_hours`까지 작동하고, 새로운 로그인 및 새로고침은 실패합니다. IdP가 자주 유지보수 창을 가지면 더 긴 `ttl_hours`를 설정하세요.

<h3 id="jwt-secret-rotation">
  JWT 시크릿 로테이션
</h3>

기존 세션이 유효하게 유지되도록 3단계로 서명 시크릿을 로테이션하세요:

1. 새 시크릿을 생성하세요. `session.jwt_secret` 배열에 앞에 추가하세요.
2. 배포를 롤링하세요. 새 토큰은 새 시크릿으로 서명합니다. 이전 토큰은 여전히 검증합니다.
3. `ttl_hours`와 여유 후에 이전 시크릿을 제거하고 다시 롤링하세요.

로테이션은 또한 만료 전에 세션을 강제로 제거하는 유일한 방법입니다: 베어러 토큰은 JWT 시크릿에 대해 로컬에서 검증되므로 세션별 해제가 없습니다. 배열에 이전 시크릿을 유지하지 않고 시크릿을 완전히 교체하면 한 번에 모든 미해결 세션이 무효화됩니다. 개별 오프보딩의 경우 IdP에서 사용자를 프로비저닝 해제하세요. 세션은 `ttl_hours` 내에 종료됩니다.

<h3 id="postgres">
  Postgres
</h3>

게이트웨이는 부팅 시 마이그레이션으로 생성되는 5개의 테이블을 보유합니다:

| 테이블                | 내용                                            | 보존                                                |
| ------------------ | --------------------------------------------- | ------------------------------------------------- |
| `kv`               | 장치 부여(10분 TTL) 및 속도 제한 카운터                    | 행별 TTL                                            |
| `spend`            | 주요 기간별 누적 지출 카운터(센트)                          | `admin.spend_retention_months`, 기본값 13            |
| `spend_limits`     | 구성된 지출 상한                                     | API를 통해 삭제될 때까지                                   |
| `admin_audit`      | 관리자 API 변경 추적                                 | `admin.audit_retention_days`, 기본값 365             |
| `principal_emails` | 각 주요의 마지막 확인 이메일, 표시 이름 및 IdP 그룹. PII를 포함합니다. | `admin.identity_retention_days` 마지막 활동 이후, 기본값 90 |

30초 루프는 TTL을 지난 `kv` 행을 만료하고, 시간별 스윕은 지출 테이블의 보존 창을 적용하므로 아무것도 무한정 증가하지 않습니다. [지출 제한](/docs/ko/claude-apps-gateway-spend-limits)이 구성되지 않으면 `kv`만 작성됩니다. 보안 정책이 애플리케이션 역할의 DDL을 금지하면 이러한 테이블과 `_migrations`을 관리자 역할로 미리 생성하고 앱 역할에 각각에 대해 `SELECT, INSERT, UPDATE, DELETE`를 부여하세요.

지출 제한이 사용 중이면, 손실된 데이터베이스는 개발자 재로그인뿐만 아니라 손실된 지출 추적 및 상한을 의미하므로 정기적인 백업을 실행하세요. 보존을 기다리지 않고 떠난 개발자 하나를 즉시 지우려면 `DELETE FROM principal_emails WHERE principal = '<sub>'`을 직접 실행하세요. 이는 이메일, 이름 및 그룹을 보유하는 유일한 테이블을 제거합니다. `spend` 및 `admin_audit` 행은 의사명 OIDC `sub`만 참조합니다.

<h3 id="upgrades">
  업그레이드
</h3>

복제본은 상태 비저장이므로 롤링 재시작은 언제든지 안전합니다. 게이트웨이는 부팅 시 스키마 마이그레이션을 실행하므로, 새 바이너리를 배포하면 데이터베이스가 자동으로 마이그레이션됩니다. 데이터베이스 역할이 DDL을 실행할 수 없으면 스키마를 미리 생성하세요. 현재 버전으로 시드된 `_migrations` 테이블을 포함합니다. 그렇지 않으면 부팅이 `CREATE TABLE`을 시도하면서 실패합니다.

마이그레이션은 추가 전용이므로 더 적은 마이그레이션을 아는 이전 바이너리로 롤백하는 것은 안전합니다. 추가 행을 무시합니다. 롤백은 또한 YAML을 이전 바이너리의 스키마에 대해 재검증하므로, 새 릴리스에서 도입한 키를 채택한 구성은 이전 바이너리에서 부팅이 실패합니다. 롤백 전에 새 키를 제거하세요.

게이트웨이의 버전을 자신의 이미지에 고정하므로, 새 Claude Code 릴리스의 수정사항(보안 수정사항 포함)은 핀을 업데이트하고 재배포할 때만 배포에 도달합니다. 게이트웨이를 프로덕션 자격 증명을 보유하는 다른 서비스에 사용하는 것과 동일한 패칭 주기에 포함하세요.

<h2 id="security">
  보안
</h2>

이 섹션은 보안 검토가 묻는 질문에 답합니다: 어떤 데이터가 게이트웨이를 통해 흐르고 어디로 가는지, 설계가 방어하는 공격, 규정 준수 질문지에 속하는 답변.

<h3 id="data-flow">
  데이터 흐름
</h3>

| 데이터                                                                          | 경로                                              | 게이트웨이에서 Anthropic으로 전송됨       |
| ---------------------------------------------------------------------------- | ----------------------------------------------- | ----------------------------- |
| 추론(프롬프트, 완료)                                                                 | CLI → 게이트웨이 → 업스트림                              | Anthropic API가 구성된 업스트림인 경우에만 |
| 텔레메트리(OTLP 메트릭, 플러스 [옵트인 로그 및 추적](/docs/ko/claude-apps-gateway-config#telemetry)) | CLI → 게이트웨이 → 수집기                               | 절대 아님                         |
| ID(이메일, 그룹, sub)                                                             | IdP → 게이트웨이 → JWT → CLI; CLI는 OTLP 내보내기에 스탬프합니다 | 절대 아님                         |
| 관리형 설정                                                                       | 게이트웨이 YAML → CLI                                | 절대 아님                         |
| 감사 로그                                                                        | 게이트웨이 stderr → 수집기                              | 절대 아님                         |

<h3 id="threat-model-summary">
  위협 모델 요약
</h3>

게이트웨이는 네트워크 경계 내부에 있지만, 개별 개발자 노트북은 신뢰할 수 있는 것으로 취급되지 않습니다. 설계는 3가지 방식으로 이를 설명합니다:

* 개발자는 원시 업스트림 키 대신 단기 JWT를 보유합니다. CLI-게이트웨이 레그는 RFC 8628 장치 부여를 사용하고, 게이트웨이의 IdP와의 인증 코드 교환은 기본 구성에서 PKCE를 실행하므로, 가로챈 IdP 인증 코드는 쓸모가 없습니다.
* 장치 검증 페이지는 동일 출처 POST 및 RFC 8628 §5.1당 IP당 속도 제한을 적용합니다. [사용자 코드 무차별 대입 저항](#user-code-brute-force-resistance)을 참조하세요.
* 아웃바운드 요청은 DNS를 확인하고, 링크 로컬 및 클라우드 메타데이터 주소와 기본적으로 루프백을 차단하며, 연결을 확인된 IP에 고정하는 서버 측 요청 위조(SSRF) 가드를 통과합니다. 따라서 IdP 및 OTLP 대상과 같은 운영자 영향 URL은 클라우드 메타데이터 엔드포인트로 리디렉션될 수 없습니다. RFC 1918 프라이빗 범위는 의도적으로 허용됩니다. IdP 및 OTLP 수집기가 일반적으로 프라이빗 IP에 있기 때문입니다. 루프백 IdP 또는 수집기에 대한 로컬 개발의 경우 게이트웨이의 환경에서 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`을 설정하세요. 프로덕션에서는 설정하지 마세요.

자신의 송신 제어를 추가하면, 게이트웨이는 워크로드 ID와 같은 인스턴스 메타데이터 자격 증명을 사용할 때마다 메타데이터 서버에 도달해야 합니다.

두 가지 위협은 범위를 벗어났습니다. 이는 보안할 인프라입니다:

* **손상된 게이트웨이 호스트**: 호스트는 업스트림 자격 증명을 보유하고 [관리형 설정](/docs/ko/claude-apps-gateway-config#managed)을 모든 연결된 개발자에게 배포하므로, 게이트웨이 구성에 대한 제어는 MDM에 대한 제어와 비교할 수 있습니다. CLI의 일회성 승인 대화는 셸 가능 설정의 자동 변경을 제한하지만 호스트 보안을 대체하지 않습니다.
* **악의적인 OIDC 공급자**: 공급자는 게이트웨이가 신뢰하는 id\_token에 서명하므로 모든 ID를 주장할 수 있습니다. IdP 검증 및 보안은 귀사의 책임입니다.

<h3 id="user-code-brute-force-resistance">
  사용자 코드 무차별 대입 저항
</h3>

개발자가 `/device` 검증 페이지에 입력하는 `user_code`는 20자 알파벳에서 그려진 8자이며, 20⁸ 또는 약 2.56×10¹⁰ 조합을 산출하고 10분 후 만료됩니다.

게이트웨이는 [`rate_limits`](/docs/ko/claude-apps-gateway-config#http-tuning)를 통해 구성 가능한 장치 부여 엔드포인트에 IP당 속도 제한을 적용합니다. 많은 개발자가 단일 공유 회사 NAT 주소에서 로그인하면 제한을 올리세요. 제한은 로그인 흐름에만 적용되며 추론에는 적용되지 않습니다.

<h3 id="compliance-posture">
  규정 준수 태세
</h3>

* **데이터 거주지**: 게이트웨이의 자체 데이터 평면은 Anthropic API가 구성된 업스트림인 경우를 제외하고 Anthropic에 아무것도 보내지 않습니다. 그 경우 기존 데이터 처리 계약이 추론 경로에 적용됩니다. 텔레메트리, 감사, ID 및 설정은 구성한 대상으로만 이동합니다.
* **호스트 프로세스 트래픽**: 호스트 프로세스는 Claude Code CLI이며, 시작 분석 및 업데이트 확인을 Anthropic으로 보낼 수 있습니다. 엄격한 송신 배포의 경우 게이트웨이의 컨테이너 환경에서 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`을 설정하세요.
* **클라이언트 분석**: CLI는 게이트웨이에 로그인하는 동안 자신의 사용 분석을 비활성화하고, 오류 보고는 타사 API 표면에서 기본적으로 꺼져 있습니다.
* **클라이언트 머신**: 개발자의 CLI는 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` 및 `skipWebFetchPreflight: true`가 설정되지 않으면 여전히 WebFetch 호스트명 확인 및 버전 확인을 Anthropic으로 보냅니다. [데이터 사용](/docs/ko/data-usage)을 참조하세요.
* **설문 조사 평가**: 게이트웨이 자격 증명은 Anthropic 바운드 평가 싱크를 비활성화하므로 평가는 Anthropic으로 전송되지 않습니다.
* **트랜스크립트 공유**: 설문 조사의 트랜스크립트 공유 프롬프트에서 예를 선택하면 Anthropic으로 업로드하는 대신 `~/.claude/feedback-bundles/` 아래의 로컬 파일을 작성합니다.
* **클라이언트 업데이트**: 업데이트 확인은 게이트웨이 트래픽과 별개입니다. 자신의 배포를 통해 버전을 고정하고 노트북이 릴리스를 가져오면 안 되면 `DISABLE_UPDATES`를 설정하세요. `DISABLE_AUTOUPDATER`는 `claude update`가 여전히 작동하는 동안 백그라운드 업데이트만 중지합니다.
* **TLS**: 프로덕션에서 `public_url`을 HTTPS를 통해 제공하세요. 게이트웨이의 자체 리스너를 통해 `listen.tls`를 사용하거나 `listen.public_url`이 설정된 일반 HTTP 복제본 앞의 TLS 종료 ingress에서. 게이트웨이는 일반 HTTP를 거부하지 않습니다. IdP는 프로덕션에서 HTTPS를 제공해야 하고, Postgres는 `?sslmode=require`를 지원합니다. ingress에서 `Strict-Transport-Security`를 설정하세요.
* **취약점 공개**: [보안 문제 보고](/docs/ko/security#reporting-security-issues)를 따르세요

<h2 id="troubleshooting">
  문제 해결
</h2>

질문 및 피드백은 [Claude Code 지원](https://support.claude.com/en/collections/14445694-claude-code)을 사용하거나 [Claude Code GitHub 저장소](https://github.com/anthropics/claude-code/issues)에서 이슈를 열어주세요. 문제를 보고할 때 다음을 포함하세요:

* **게이트웨이 문제**: 관련 창의 게이트웨이 stderr, 시크릿이 수정된 `gateway.yaml`, 게이트웨이 버전(랜딩 페이지 `/`에 표시되고 `/managed/settings`의 `x-cc-gateway-version` 응답 헤더에 표시됨), 최근에 변경된 것
* **로그인 문제**: 개발자는 `claude --debug-file ./claude-debug.txt`를 실행하고, 재현하며, 해당 파일과 동일한 창의 게이트웨이 감사 로그를 보냅니다
* **추론 문제**: 요청된 모델, 구성된 업스트림, 요청의 게이트웨이 감사 로그(어느 업스트림이 제공했는지 및 응답 상태를 기록함)

게이트웨이의 stderr에는 감사 이벤트 스트림이 포함되고, 감사 로그는 개발자 신원을 기록하며, 디버그 파일은 개발자 머신의 hook 및 MCP 서버 출력을 기록합니다. 공개 이슈에 게시하기 전에 이를 검토하고 제거해주시기 바랍니다.

| 증상                                                                                                                                                | 원인                                                                                                                                                                                                                                                                                                         | 수정                                                                                                                                                                                                                                                                                                     |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 개발자의 `/login`이 **Cloud 게이트웨이** 화면 대신 표준 계정 선택기를 표시합니다                                                                                             | `forceLoginMethod` 또는 `forceLoginGatewayUrl`이 해당 머신의 관리형 설정에 설정되지 않음                                                                                                                                                                                                                                       | [관리형 설정 파일](/docs/ko/claude-apps-gateway#set-the-gateway-url)을 장치에 배포하세요. `/login`은 거기서 게이트웨이 URL을 읽습니다                                                                                                                                                                                                     |
| 시작 시 `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.` 표시                     | 설치된 Claude Code 빌드가 게이트웨이 지원보다 앞서갑니다                                                                                                                                                                                                                                                                       | 개발자가 Claude Code를 Cloud 게이트웨이 지원을 포함하는 릴리스로 업데이트하도록 하세요                                                                                                                                                                                                                                                |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>`        | 게이트웨이 호스트명이 적어도 하나의 공개 IP 주소로 확인됩니다. Claude Code는 각 확인된 주소를 확인하고 모든 주소가 프라이빗이어야 합니다. 일반적인 원인은 한 패밀리가 공개 주소로 확인되는 이중 스택 이름입니다. AWS 내부 이중 스택 로드 밸런서를 포함하여 공개 범위 AAAA 주소를 반환합니다. Anthropic이 운영하는 공개 게이트웨이 엔드포인트는 확인에서 제외되며, `/login`은 `https://`를 통해 이들을 허용합니다. v2.1.206 이전에는 `/login`이 다른 공개 주소처럼 이들을 거부했습니다 | 게이트웨이 이름이 개발자 머신에서 프라이빗 주소로만 확인되도록 하세요. 이중 스택 이름의 경우 공개 범위 레코드를 삭제하거나 별도의 내부 전용 DNS 이름을 제공하세요. [프라이빗 네트워크 전제조건](/docs/ko/claude-apps-gateway#prerequisites)을 참조하세요.                                                                                                                                         |
| CLI `/login`: `Gateway login requires a direct connection and does not support connecting through an HTTP proxy`                                  | `HTTPS_PROXY` 또는 `HTTP_PROXY`가 게이트웨이 호스트에 적용되고 프록시의 호스트명이 공개 주소로 확인됩니다. 호스트가 프라이빗 주소로만 확인되는 프록시는 허용되며 이 오류를 트리거하지 않습니다                                                                                                                                                                                     | 개발자의 머신에서 `NO_PROXY`에 게이트웨이 호스트를 추가하여 연결이 직접이거나 호스트명이 프라이빗 주소로 확인되는 프록시를 사용하세요                                                                                                                                                                                                                         |
| CLI `/login`: `Could not resolve gateway host <host>`                                                                                             | 머신이 게이트웨이의 내부 DNS 이름을 확인할 수 없습니다. 일반적으로 회사 네트워크에 없기 때문입니다                                                                                                                                                                                                                                                  | 개발자가 네트워크 또는 VPN에 연결한 후 `/login`을 다시 시도하도록 하세요                                                                                                                                                                                                                                                         |
| 부팅이 `store.postgres_url`을 명명하는 구성 검증 오류로 종료됩니다                                                                                                    | Postgres가 구성되지 않음. 게이트웨이는 Postgres가 필요합니다                                                                                                                                                                                                                                                                  | `store.postgres_url`을 설정하세요. 로컬 개발의 경우 일회용 컨테이너를 사용하세요: `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.                                                                                                                                                                    |
| 부팅 종료: `requires the native binary`                                                                                                               | Node 대신 네이티브 바이너리에서 실행 중입니다                                                                                                                                                                                                                                                                                | [독립 실행형 설치 방법](/docs/ko/setup) 중 하나로 Claude Code를 설치하세요                                                                                                                                                                                                                                                     |
| 부팅이 `config.load` 후 OIDC 검색 오류로 종료됩니다                                                                                                             | `oidc.issuer`에 도달할 수 없거나 TLS 체인을 신뢰하지 않습니다                                                                                                                                                                                                                                                                 | 발급자가 포드에서 도달 가능하고 `/.well-known/openid-configuration`을 제공하는지 확인하세요. 프라이빗 PKI의 경우 `ca_cert_pem`을 설정하세요.                                                                                                                                                                                                 |
| 부팅이 Postgres 권한 오류로 종료됩니다                                                                                                                         | 앱 역할에 `CREATE TABLE` 권한이 없습니다                                                                                                                                                                                                                                                                              | 관리자 역할로 스키마를 미리 생성하고 앱 역할에 DML을 부여하거나, 새 마이그레이션을 적용하는 부팅을 위해 DDL을 임시로 부여하세요                                                                                                                                                                                                                            |
| `/oauth/callback`이 "Sign-in could not be completed" 표시                                                                                            | 이메일 도메인이 거부되었거나, id\_token 검증이 실패했거나, `email_verified`가 명시적으로 `false`입니다. 게이트웨이는 항상 이를 거부하며 재정의가 없습니다                                                                                                                                                                                                      | `allowed_email_domains`를 확인하고 IdP가 검증된 `email` 클레임을 반환하는지 확인하세요. `email_verified: false`의 경우 IdP 측 검증을 수정하세요. IdP가 다른 클레임 이름 아래에 이메일을 내보내면 `oidc.email_claim`을 설정하세요.                                                                                                                                  |
| 로그: `token exchange failed: id_token missing email claim`                                                                                         | IdP가 기본적으로 id\_token에 `email`을 포함하지 않습니다. 이 거부는 `allowed_email_domains`가 설정된 경우에만 발생합니다. 없으면 누락된 이메일이 이메일 없이 세션을 민트합니다                                                                                                                                                                                     | IdP를 구성하여 id\_token에 `email`을 내보내세요. Okta: 사용자 정의 인증 서버의 ID 토큰 클레임에 `email`을 추가하세요. Entra: 앱 등록에서 `email`을 선택적 클레임으로 추가하세요. PingFederate: `email`을 내보내는 OpenID Connect 정책을 활성화하세요. IdP가 userinfo 엔드포인트에서 `email`을 제공하지만 id\_token에 포함하지 않으려면(예: Okta 조직 인증 서버), `oidc.userinfo_fallback: true`를 설정하세요. |
| 모든 Amazon Bedrock 요청이 502를 반환합니다. 로그는 `Could not load credentials from any providers` 표시                                                          | EC2에서 IMDSv2의 기본 홉 제한 1은 컨테이너 내부에서 인스턴스 메타데이터 요청을 차단합니다. 부팅 및 `/readyz`는 AWS SDK가 클라이언트 구성이 아닌 첫 번째 요청에서 인스턴스 자격 증명을 확인하므로 어쨌든 통과합니다                                                                                                                                                                       | `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`로 홉 제한을 올리거나 시작 템플릿에서 설정하세요. 변경은 인스턴스의 모든 컨테이너에 적용됩니다. 가능한 경우 ECS 작업 역할을 선호하세요. 이는 ECS 컨테이너 자격 증명 엔드포인트에서 자격 증명을 읽고 변경을 완전히 피하거나, 노출을 제한하기 위해 전용 게이트웨이 인스턴스에 변경을 적용하세요.                                   |
| IdP 오류: unknown or unsupported scope                                                                                                              | IdP가 인식하지 못하는 범위를 거부합니다                                                                                                                                                                                                                                                                                    | `oidc.scopes`를 IdP가 허용하는 정확한 목록으로 설정하세요. `openid`를 포함해야 합니다. 기본값은 `openid profile email offline_access`입니다.                                                                                                                                                                                            |
| `oidc.scopes` 설정 후 세션이 자동으로 갱신되지 않습니다                                                                                                             | `offline_access`가 재정의에서 삭제되었습니다                                                                                                                                                                                                                                                                            | IdP가 지원하면 `offline_access`를 다시 추가하세요. 새로고침 토큰이 없으면 개발자는 매 `session.ttl_hours`마다 브라우저 로그인을 다시 실행합니다.                                                                                                                                                                                                    |
| 브라우저가 "This request came from another site and was blocked" 표시                                                                                    | CSRF 보호로 차단된 교차 사이트 양식 POST. 포함되거나 프록시된 페이지의 경우 예상됩니다                                                                                                                                                                                                                                                      | 검증 링크를 직접 열어주세요                                                                                                                                                                                                                                                                                        |
| Chrome이 "Refused to send form data … violates … Content Security Policy directive: form-action"으로 승인 버튼을 차단하지만 동일한 페이지가 Safari 또는 Firefox에서 작동합니다 | Chrome은 전체 리디렉션 체인에 대해 `form-action`을 적용합니다. IdP가 허용 목록에 없는 두 번째 호스트로 계속 리디렉션합니다.                                                                                                                                                                                                                          | 리디렉션 체인의 각 추가 원본을 `oidc.form_action_origins`에 추가하세요. 승인 페이지에서 Chrome DevTools → 콘솔을 열어 어느 원본이 차단되었는지 확인하세요.                                                                                                                                                                                            |
| 로그인이 IdP에서 완료되지만 콜백이 실패합니다. Chrome에서 CSP 오류 또는 Safari에서 "this sign-in link has expired"                                                           | IdP가 `response_mode=form_post`를 통해 코드를 반환했으며, 이는 POST를 통해 `/oauth/callback`으로 교차 원본을 자동 제출합니다. Chrome은 엄격한 CSP에서 이를 차단합니다. Safari는 제출을 허용하지만 콜백은 쿼리 문자열만 읽습니다.                                                                                                                                             | IdP가 `response_mode=query`를 준수하는지 확인하세요. 게이트웨이가 명시적으로 요청하므로 콜백은 일반 리디렉션입니다                                                                                                                                                                                                                             |
| 로그인이 로컬에서 작동하지만 ALB 뒤에서 실패합니다                                                                                                                     | `public_url`이 설정되지 않아 IdP가 내부 `http://` 원본을 `redirect_uri`로 받습니다                                                                                                                                                                                                                                           | `listen.public_url`을 외부 `https://` 원본으로 설정하세요                                                                                                                                                                                                                                                          |
| 개발자가 신뢰 프롬프트를 반복적으로 봅니다                                                                                                                           | TLS 인증서가 복제본별 또는 요청별로 회전합니다                                                                                                                                                                                                                                                                                | ingress에서 안정적인 인증서를 사용하거나 TLS를 한 번 종료하고 내부적으로 일반 HTTP를 통해 복제본을 실행하세요                                                                                                                                                                                                                                   |
| CLI `/login`: "Could not verify the gateway's TLS certificate" 또는 `SELF_SIGNED_CERT_IN_CHAIN`                                                     | 게이트웨이의 TLS 체인이 CLI 호스트의 신뢰 저장소에 없는 프라이빗 CA로 서명됩니다                                                                                                                                                                                                                                                          | Claude Code는 기본적으로 네이티브 바이너리 및 Node 22.15 이상에서 OS 신뢰 저장소를 읽습니다. [`CLAUDE_CODE_CERT_STORE`](/docs/ko/network-config#ca-certificate-store)는 이 동작을 제어합니다. CA가 OS 신뢰 저장소에 설치되면 개발자가 현재 런타임에 있는지 확인하세요. 그렇지 않으면 시작 전에 `NODE_EXTRA_CA_CERTS`를 CA 인증서 PEM으로 설정하세요. 첫 연결 지문 프롬프트는 여전히 적용됩니다.                        |

<h2 id="related">
  관련
</h2>

* [Claude 앱 게이트웨이 개요](/docs/ko/claude-apps-gateway): 빠른 시작 및 개발자 연결
* [구성 참조](/docs/ko/claude-apps-gateway-config): 모든 `gateway.yaml` 옵션
